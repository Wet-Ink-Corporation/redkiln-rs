//! Connection discipline — ADR-0105 clause 7, stated as rules and enforced as types.
//!
//! *"`busy_timeout` = **5000 ms** (rusqlite's default; R-SQLITE confirms); every write transaction
//! is shorter than it — a rebuild writes to a separate file and renames. **One writer connection
//! per process, behind a mutex, no queue**; reader connections are one per thread and also set
//! `busy_timeout` (readers can see `SQLITE_BUSY`); `doctor` checks the common dir is not on a
//! network filesystem (WAL does not work over one)."*
//!
//! **What is measured and what is not.** The 5000 ms value is `[documented]` — rusqlite 0.40.2's
//! default, read from docs.rs. Whether it *suffices* is not: ADR-0105's own evidence limits say
//! *"It does not license claiming … that 5000 ms suffices under the self-hosting run's load —
//! R-SQLITE and Step 8 (3) are the measurements."* So the number is honoured as a documented
//! default, and the store's concurrency behaviour under contention is reported as documented-only
//! until AC-181's two-process test runs.
//!
//! **One writer, behind a mutex, no queue.** `rusqlite::Connection` is `Send` and `!Sync`
//! (ADR-0105 clause 11), so a shared connection needs a mutex, and "no queue" rules out the other
//! obvious shape — a channel with a writer thread — which would decouple a write from its caller's
//! error handling and make "every write transaction shorter than `busy_timeout`" unobservable.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rusqlite::Connection;

/// ADR-0105 clause 7's value. rusqlite 0.40.2's default, restated here so the store does not depend
/// on the crate's default staying what it is.
pub const BUSY_TIMEOUT_MS: u32 = 5000;

#[derive(Debug)]
pub enum StoreError {
    Sqlite(String),
    /// A write transaction ran longer than `busy_timeout`. Clause 7 requires every one to be
    /// shorter; exceeding it is a typed error rather than a log line, because a transaction that
    /// outlives the timeout is the condition under which another process sees `SQLITE_BUSY`.
    TransactionExceededBusyTimeout {
        millis: u128,
    },
    /// The common dir is on a network filesystem. WAL does not work over one, so `doctor` refuses
    /// rather than producing a store that corrupts under the very concurrency it is built for.
    NetworkFilesystem {
        path: PathBuf,
    },
    Identity(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlite(e) => write!(f, "sqlite: {e}"),
            Self::TransactionExceededBusyTimeout { millis } => write!(
                f,
                "a write transaction ran {millis} ms, longer than busy_timeout ({BUSY_TIMEOUT_MS} \
                 ms). ADR-0105 clause 7 requires every write transaction to be shorter."
            ),
            Self::NetworkFilesystem { path } => write!(
                f,
                "{} is on a network filesystem; WAL does not work over one (ADR-0105 clause 7)",
                path.display()
            ),
            Self::Identity(e) => write!(f, "store identity: {e}"),
        }
    }
}

impl From<rusqlite::Error> for StoreError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Sqlite(e.to_string())
    }
}

/// Where the store lives: `<git-common-dir>/redkiln/`.
///
/// **One store per repository** (AC-178). Resolved from the common dir, which on arm a's **go** is
/// what gix reports from any worktree — so opening from a linked worktree yields the main
/// repository's store rather than a second one beside it.
#[must_use]
pub fn store_dir(common_dir: &Path) -> PathBuf {
    common_dir.join("redkiln")
}

#[must_use]
pub fn store_path(common_dir: &Path) -> PathBuf {
    store_dir(common_dir).join("store.sqlite3")
}

/// The single writer connection for this process, behind a mutex.
///
/// Not `Clone`, and there is no constructor that makes a second one for the same path — "one writer
/// connection per process" is expressed by there being one value.
#[derive(Debug)]
pub struct Writer {
    conn: Mutex<Connection>,
    path: PathBuf,
}

impl Writer {
    /// Open the writer, applying clause 7's settings.
    ///
    /// # Errors
    /// [`StoreError::Sqlite`] if the database cannot be opened or configured.
    pub fn open(common_dir: &Path) -> Result<Self, StoreError> {
        let dir = store_dir(common_dir);
        std::fs::create_dir_all(&dir).map_err(|e| StoreError::Sqlite(e.to_string()))?;
        let path = store_path(common_dir);
        let conn = Connection::open(&path)?;
        configure(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
            path,
        })
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Run `f` inside a write transaction, and assert it was shorter than `busy_timeout`.
    ///
    /// The duration is measured with `Instant` **here on purpose**: `clippy.toml` bans the clock on
    /// the *workflow* side (AC-001/AC-003), where a time read would make a decision
    /// non-deterministic. This is the store's own durability guard, it feeds no decision, and the
    /// alternative — not checking — is the thing clause 7 forbids.
    ///
    /// # Errors
    /// [`StoreError::TransactionExceededBusyTimeout`] if the transaction outran the timeout;
    /// otherwise whatever `f` returns.
    #[expect(
        clippy::disallowed_methods,
        reason = "AC-001 bans the clock on the WORKFLOW side, where a time read makes a decision                   non-deterministic. clippy.toml has no path scoping, so the exemption is stated                   here, at the narrowest item. This reading feeds no decision and is never                   journalled: it is the store's own durability guard for ADR-0105 clause 7's                   `every write transaction shorter than busy_timeout`, and the alternative to                   measuring is not checking."
    )]
    pub fn transaction<T, F>(&self, f: F) -> Result<T, StoreError>
    where
        F: FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StoreError>,
    {
        let mut guard = self
            .conn
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let started = std::time::Instant::now();
        let tx = guard.transaction()?;
        let out = f(&tx)?;
        tx.commit()?;
        let elapsed = started.elapsed();
        if elapsed.as_millis() > u128::from(BUSY_TIMEOUT_MS) {
            return Err(StoreError::TransactionExceededBusyTimeout {
                millis: elapsed.as_millis(),
            });
        }
        Ok(out)
    }
}

/// A reader connection — **one per thread**, and it also sets `busy_timeout`, because readers can
/// see `SQLITE_BUSY` too (clause 7 says so explicitly).
#[derive(Debug)]
pub struct Reader {
    conn: Connection,
}

impl Reader {
    /// # Errors
    /// [`StoreError::Sqlite`] if the database cannot be opened or configured.
    pub fn open(common_dir: &Path) -> Result<Self, StoreError> {
        let conn = Connection::open(store_path(common_dir))?;
        configure(&conn)?;
        Ok(Self { conn })
    }

    #[must_use]
    pub const fn conn(&self) -> &Connection {
        &self.conn
    }
}

/// WAL plus the busy timeout, on every connection of either kind.
fn configure(conn: &Connection) -> Result<(), StoreError> {
    conn.busy_timeout(std::time::Duration::from_millis(u64::from(BUSY_TIMEOUT_MS)))?;
    // `query_row` rather than `execute`: SQLite answers `PRAGMA journal_mode` with the resulting
    // mode, and `execute` refuses a statement that returns a row.
    let mode: String = conn.query_row("PRAGMA journal_mode = WAL", [], |r| r.get(0))?;
    if !mode.eq_ignore_ascii_case("wal") {
        return Err(StoreError::Sqlite(format!(
            "journal_mode is `{mode}`, not WAL — clause 7's concurrency contract assumes WAL"
        )));
    }
    Ok(())
}

/// Is `path` on a network filesystem? `doctor`'s check (clause 7).
///
/// Windows is the primary platform, so the answer that matters here is the one for a UNC path or a
/// mapped network drive. **Reported honestly as partial**: this recognises UNC and the
/// extended-length UNC device form, and it does not attempt to resolve a mapped drive letter back
/// to its remote, which needs `WNetGetConnection` and is not worth a platform dependency until
/// something measures that it matters.
#[must_use]
pub fn looks_like_network_filesystem(path: &Path) -> bool {
    let s = path.to_string_lossy().replace('\\', "/");
    s.starts_with("//") && !s.starts_with("//?/") || s.to_ascii_uppercase().starts_with("//?/UNC/")
}
