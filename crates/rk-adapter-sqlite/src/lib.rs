//! `rk-adapter-sqlite` — the real `EventStore` row over `rk-store` (design §9.1; ADR-0094
//! decision 1's second adapter; `s7c` §7c.13).
//!
//! **What this crate is.** `impl EventStore for SqliteStore` over `rk_store::conn::{Writer,
//! Reader}` — the connection discipline ADR-0105 clause 7 gives `rk-store` (*"one writer
//! connection per process, behind a mutex, no queue; reader connections are one per thread …
//! WAL, `busy_timeout` 5000 ms, bundled"*), consumed and not re-implemented. Every SQL statement
//! here runs through those two types' inherent methods; `rusqlite` is not a dependency of this
//! manifest (§7c.19's row), and the crate opens **no second writer connection** (clause 7). The
//! store's on-disk layout is `rk-store`'s too: `0001_initial.sql` applied **unchanged** from
//! `rk_store::schema_lint::all()`, so every key column — and every `-- repo-key` marker AC-166's
//! lint decides over — is Step 6's, and no mutation of this crate can add a branch or path key
//! (AC-371, cross-listed to `rk-store`, §7c.13).
//!
//! **The adapter holds the `Writer` only and opens a `Reader` per `query` call.**
//! `rk_store::conn::Reader` wraps a bare `rusqlite::Connection`, which is `Send` but `!Sync`
//! (`conn.rs`'s own doc), and `EventStore: Send + Sync` — so a struct holding a `Reader` field
//! cannot implement the port (E0277); per-call readers are clause 7's *"readers one per thread"*
//! read literally. The `Writer` is `Mutex<Connection>` and is `Sync` by that mutex, which is
//! `rk-store`'s. **This crate inherits `rk-store`'s poison recovery on that mutex and adds none
//! of its own** — the recovery in `conn.rs` is ADR-0118's violation, RAISED and not repaired
//! here (§7c.1, §7c.22); nothing in this crate names a lock, a poison or an unwind.
//!
//! **On open the adapter applies the schema itself**: `rk-store` exposes no migration runner —
//! `Writer::open` creates the file and sets WAL and the timeout and nothing else, and the
//! migration SQL is reachable only through `rk_store::schema_lint::all()` — so [`SqliteStore::open`]
//! runs that SQL through `Writer::transaction(|tx| tx.execute_batch(sql))`, and stamps or checks
//! the identity ([`identity`], AC-369) **in the same transaction**. What makes *"nothing is
//! written"* true is that one transaction: a refusal is an `Err` out of the closure, `Writer::
//! transaction` drops the `rusqlite::Transaction` unfinished and it rolls back, so a refused open
//! leaves the file and its WAL byte-identical — `ac369_…` copies a store with a table dropped and
//! watches that a schema committed in a transaction of its own *before* the check re-creates it
//! [measured 2026-09-18: red under that mutant; green again with one transaction]. Inside the
//! transaction the stamp is read and checked before the schema statements, which is the readable
//! order and not the defence: with one transaction either order leaves the bytes alone [measured
//! 2026-09-18: the swapped order is green], and the schema has to run before the stamp is written
//! on a fresh store in any case.
//!
//! **Where the store is** ([`path`], AC-370): `<git rev-parse --git-common-dir>/redkiln/
//! store.sqlite3`, the common dir taken through `rk_git::GitExec` (RT-17; ADR-0077's one
//! wrapper) and joined onto the worktree when git answers relatively (`s7c` §7c.13's join rule).
//! The constructor takes the `GitExec` the composition root minted — `GitExec::new` needs a
//! `GitToolchain` only `rk_git::version::assert_at_startup` mints, which spawns `git --version`,
//! so this crate never constructs one.
//!
//! **What the four port methods do.** `append` inserts one `t2_event(kind, payload,
//! anchor_sha)` row: the kind's wire name, the payload as JSON text, and `anchor_sha` **NULL** —
//! `rk_types::event::Event` is `{ kind, payload }` and carries no anchor (the typed-payload
//! record is OWED, `s7a` §7a.5 / ADR-0126 decision 6), the column is nullable by Step 6's schema,
//! and reading an anchor out of an untyped payload would be a decision this stage may not take.
//! `query` returns every row **ordered by id**, and every selector — `ByItem`, `BySha`, `ByRun`
//! — returns everything, **matching `rk_ports::fakes::VecStore`** (RT-4, routed): the event
//! carries no item id, commit SHA or run id to filter on until that same record lands, and the
//! kernel's item-less gate fold reads the whole journal; the filter arrives with the record and
//! the shared conformance body does not change. `snapshot` and `rebuild` answer
//! `Err(StepError::Unsupported { capability })` **from the dated registry row** ([`registry`];
//! D17-9): [`SqliteStore::open`] loads the shipped registry once (a registry that does not load
//! fails the open typed, [`StoreOpenError::Registry`]) and the store holds its rows; the two
//! methods derive their answer from those rows through [`registry::refusal`], so the token in the
//! error is the row's, and a registry that carries no such row is a typed decode failure, never
//! a default — `ac489_…` opens a store over an empty registry and watches exactly that.
//!
//! **What a failed write is.** A tier-2 append that does not land returns
//! `StepError::Transport { kind: Io, detail }` and never panics (AC-368's sqlite negative; the
//! warning-with-a-counter is the *caller*'s, the Harness adapter's `dispatch_completed` append,
//! `s7c` §7c.13): every `StoreError` out of `Writer::transaction` — the `INSERT` refused, the
//! transaction longer than `busy_timeout`, `SQLITE_BUSY` past it — crosses the port at the one
//! `map_err` in `append`. **What is measured** is a corrupt store (`ac368_…` drops `t2_event`
//! from a second connection); lock contention past the timeout is `rk-store`'s documented
//! behaviour (`conn.rs`), not exercised here (the test's doc says why). No modifying statement
//! exists in this crate: a correction is a second appended row that references the first in its
//! payload (AC-364), and the port has no `update`/`delete` to reach (`rk-ports`' source-parse
//! half).
//!
//! **Lint posture.** Every override in this crate is `#[expect(clippy::…, reason = "…")]` per
//! site (ADR-0102 decision 3) — and there are none: no clock, no sleep, no `HashMap`, no
//! `Command`, no environment read. `thiserror` for the error enum (decision 6); `anyhow` nowhere.

use std::path::{Path, PathBuf};

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_ports::event_store::{EventStore, Query, Snapshot};
use rk_store::conn::{Reader, StoreError, Writer};
use rk_store::identity::Identity;
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

pub mod identity;
pub mod path;
pub mod registry;

/// Why an open was refused or could not complete.
///
/// Closed; matched exhaustively by callers that distinguish a refusal (AC-369 — the store is
/// another repository's) from a failure to reach the repository or the file.
///
/// `rk_git::error::GitError` derives `Debug` only and `rk_store::conn::StoreError` implements
/// `Display`; neither implements `std::error::Error`, so both are carried as members and
/// rendered, never chained as a `source` — `thiserror`'s `#[from]`/`#[source]` would demand the
/// trait they do not implement. A `GitError` is rendered through [`render_git`], which writes a
/// `NonZeroExit`'s retained stderr as text (`from_utf8_lossy`) rather than as `Debug`'s byte
/// vector — retained and never parsed (ADR-0077:70) is not the same as unreadable in the one
/// place it is shown. The `GitError` is boxed: unboxed, the `Git` variant is 152 bytes and every
/// `Result` carrying this enum trips `clippy::result_large_err` (denied through `clippy::all`,
/// ADR-0102).
#[derive(Debug, thiserror::Error)]
pub enum StoreOpenError {
    /// `git` could not run or exited non-zero for the read named in `op`. Its stderr is inside
    /// the carried error, retained and never parsed (ADR-0077:70).
    #[error("git {op} in {worktree}: {}", render_git(error))]
    Git {
        op: &'static str,
        worktree: String,
        error: Box<rk_git::error::GitError>,
    },
    /// `git log --max-parents=0` exited 0 and printed nothing. **No observed producer**: a
    /// repository with no commit — the case `s7c` §7c.13 describes as "`Log` prints nothing" —
    /// exits non-zero under git 2.55 (`fatal: your current branch 'main' does not have any commits
    /// yet`) and surfaces as [`Self::Git`] (`open_refuses_a_repository_with_no_commit` measures it
    /// and would name a producer if one appeared). The variant is the guard that keeps an empty
    /// answer from ever being stamped as a root commit `""`; it is not a path any fixture reaches.
    #[error(
        "{worktree} has no commit: `git log --max-parents=0` printed nothing, so there is no root \
         commit to stamp the store's identity with (AC-369)"
    )]
    NoRootCommit { worktree: String },
    /// The joined common dir is a path `rk-git` refuses (the `\\?\UNC\` device form).
    #[error(
        "the common dir derived from {worktree} is a path rk-git refuses: {}",
        render_git(error)
    )]
    Path {
        worktree: String,
        error: Box<rk_git::error::GitError>,
    },
    /// The store belongs to a different repository, or is at a schema version this binary does
    /// not write. The text names both identities (AC-179 / AC-180's wording, `rk-store`'s).
    /// **Nothing was written.**
    #[error("refused: {0}")]
    Refused(String),
    /// The file could not be opened or configured, the schema could not be applied, or a
    /// transaction failed — `rk-store`'s own error, rendered.
    #[error("store: {0}")]
    Store(StoreError),
    /// The shipped capability registry did not load — a build defect `ac489_…` reports; an
    /// adapter whose registry does not load answers no port method from it (AC-238, D17-9).
    #[error("the shipped capability registry does not load: {detail}")]
    Registry { detail: String },
}

/// `GitError` as text: a `NonZeroExit`'s stderr decoded lossily and trimmed, every other
/// variant as its `Debug` form (each carries strings and paths, which `Debug` renders readably).
fn render_git(error: &rk_git::error::GitError) -> String {
    match error {
        rk_git::error::GitError::NonZeroExit { op, code, stderr } => format!(
            "`git {op}` exited {code:?}: {}",
            String::from_utf8_lossy(stderr).trim()
        ),
        other => format!("{other:?}"),
    }
}

impl From<StoreError> for StoreOpenError {
    /// The identity refusal is `rk-store`'s `StoreError::Identity`; it becomes [`Self::Refused`]
    /// so a caller can tell *"another repository's store"* from *"the file is broken"* without
    /// reading a message. Every other `StoreError` is carried whole.
    fn from(e: StoreError) -> Self {
        match e {
            StoreError::Identity(detail) => Self::Refused(detail),
            other => Self::Store(other),
        }
    }
}

/// The two connection settings ADR-0105 clause 7 fixes and the real conformance row asserts
/// (`s7c` §7c.13): the journal mode on the file, and the busy timeout on **this** writer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pragmas {
    pub journal_mode: String,
    pub busy_timeout_ms: u32,
}

/// The real `EventStore` row: one writer, the common dir it was derived for, and the identity it
/// was opened under.
#[derive(Debug)]
pub struct SqliteStore {
    writer: Writer,
    common_dir: PathBuf,
    identity: Identity,
    registry: Vec<registry::Row>,
}

impl SqliteStore {
    /// Open — or create — the one store for the repository `worktree` belongs to.
    ///
    /// In order: the shipped registry loaded (the port methods answer from it — a registry that
    /// does not load is a typed refusal here, never a default answer later); the common dir and
    /// the root commit through `exec` (two `git` reads); the file
    /// under `<common dir>/redkiln/` through `Writer::open` (created if absent — AC-365's 7c
    /// half); then, in **one** write transaction — the rollback of which is what makes a refusal
    /// touch nothing (the crate doc) — the identity read and checked, `0001_initial.sql` applied
    /// unchanged from `rk_store::schema_lint::all()`, and the identity stamped if the store was
    /// fresh.
    ///
    /// # Errors
    /// [`StoreOpenError::Refused`] when the store is another repository's or at another schema
    /// version — nothing written (AC-369); the other variants when git, the path or the file
    /// refuses.
    pub fn open(exec: &GitExec, worktree: &WorktreePath) -> Result<Self, StoreOpenError> {
        let rows = registry::rows().map_err(|e| StoreOpenError::Registry {
            detail: e.to_string(),
        })?;
        Self::open_over_registry(exec, worktree, rows)
    }

    /// [`Self::open`] over a registry handed in rather than the shipped one — test support, so
    /// `ac489_…` can watch the port methods answer from the rows they were given (an absent row
    /// is a typed decode failure, never `Unsupported` by default). The composition root calls
    /// [`Self::open`].
    ///
    /// # Errors
    /// As [`Self::open`], less [`StoreOpenError::Registry`].
    #[doc(hidden)]
    pub fn open_over_registry(
        exec: &GitExec,
        worktree: &WorktreePath,
        registry: Vec<registry::Row>,
    ) -> Result<Self, StoreOpenError> {
        let common_dir = path::common_dir(exec, worktree)?;
        let opening = identity::opening(exec, worktree, &common_dir)?;
        let writer = Writer::open(&common_dir)?;
        writer.transaction(|tx| {
            // The stamp first: an existing store says whose it is before a single statement runs
            // against it. `read` is a SELECT; `establish` checks and returns the refusal as
            // `StoreError::Identity`, which `From` turns into `Refused` — and the `Err` rolls this
            // whole transaction back, which is the "nothing is written" (the crate doc).
            let stamped = rk_store::identity::read(tx)?;
            let fresh = identity::establish(stamped.as_ref(), &opening)?;
            for (_name, sql, _keys) in rk_store::schema_lint::all() {
                tx.execute_batch(sql)?;
            }
            if fresh {
                rk_store::identity::initialise(tx, &opening)?;
            }
            Ok(())
        })?;
        Ok(Self {
            writer,
            common_dir,
            identity: opening,
            registry,
        })
    }

    /// The store file — `<common dir>/redkiln/store.sqlite3`.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.writer.path()
    }

    /// The common dir the store was derived for, absolute and normalised.
    #[must_use]
    pub fn common_dir(&self) -> &Path {
        &self.common_dir
    }

    /// The identity this store was opened under: the stamp on a fresh store, the checked stamp on
    /// an existing one.
    #[must_use]
    pub const fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Clause 7's two settings, read on the writer connection this store holds.
    ///
    /// # Errors
    /// [`StoreOpenError::Store`] if either pragma cannot be read.
    pub fn pragmas(&self) -> Result<Pragmas, StoreOpenError> {
        Ok(self.writer.transaction(|tx| {
            let journal_mode: String = tx.query_row("PRAGMA journal_mode", [], |r| r.get(0))?;
            let busy_timeout_ms: u32 = tx.query_row("PRAGMA busy_timeout", [], |r| r.get(0))?;
            Ok(Pragmas {
                journal_mode,
                busy_timeout_ms,
            })
        })?)
    }
}

/// A store failure as the closed variant that crosses the port: a tier-2 write or read that did
/// not land is a transport fault on the I/O path, never a verdict (AC-229; the RK-B0013 shape).
const fn io(detail: String) -> StepError {
    StepError::Transport {
        kind: TransportKind::Io,
        detail,
    }
}

impl EventStore for SqliteStore {
    fn append(&mut self, event: Event) -> Result<(), StepError> {
        let payload = serde_json::to_string(&event.payload).map_err(|e| io(e.to_string()))?;
        let kind = event.kind.wire_name();
        self.writer
            .transaction(|tx| {
                tx.execute(
                    "INSERT INTO t2_event (kind, payload, anchor_sha) VALUES (?1, ?2, NULL)",
                    (kind, &payload),
                )?;
                Ok(())
            })
            .map_err(|e| io(e.to_string()))
    }

    fn query(&self, _by: Query) -> Result<Vec<Event>, StepError> {
        // One reader per call (clause 7's "one per thread", read literally — see the crate doc);
        // it also sets `busy_timeout`, because readers can see `SQLITE_BUSY` too.
        let reader = Reader::open(&self.common_dir).map_err(|e| io(e.to_string()))?;
        let mut stmt = reader
            .conn()
            .prepare("SELECT kind, payload FROM t2_event ORDER BY id")
            .map_err(|e| io(e.to_string()))?;
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .map_err(|e| io(e.to_string()))?;
        let mut out = Vec::new();
        for row in rows {
            let (kind, payload) = row.map_err(|e| io(e.to_string()))?;
            // A kind this binary does not know is a row a later binary wrote, or a corrupt one;
            // either is a typed decode failure, never a row silently dropped from the journal.
            let kind = EventKind::from_wire(&kind).ok_or_else(|| {
                io(format!(
                    "t2_event carries kind `{kind}`, which this binary's EventKind does not name"
                ))
            })?;
            let payload = serde_json::from_str(&payload).map_err(|e| io(e.to_string()))?;
            out.push(Event::new(kind, payload));
        }
        Ok(out)
    }

    fn snapshot(&self) -> Result<Snapshot, StepError> {
        Err(registry::refusal(
            &self.registry,
            registry::Capability::Snapshot,
        ))
    }

    fn rebuild(&mut self, _snapshot: &Snapshot) -> Result<(), StepError> {
        Err(registry::refusal(
            &self.registry,
            registry::Capability::Rebuild,
        ))
    }
}
