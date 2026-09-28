//! The migration runner and the one opening transaction — ADR-0181 decision 6 (`s8e` §8e.2;
//! D21-46 R2; AC-180).
//!
//! *(dated 2026-09-28, s8e build; ADR-0181, proposed.)* Before `0002_gate_transitions` every
//! shipped migration was `CREATE TABLE IF NOT EXISTS …` and the opener re-ran all of them on every
//! open. `0002` adds a column (`ALTER TABLE t2_event ADD COLUMN item`), which refuses a second
//! run, so a store now counts the shipped migrations applied to it in its own `PRAGMA
//! user_version` and [`apply`] runs only the ones past that count. The count is the runner's
//! bookkeeping, not an identity member: the identity is still `rk_identity` (AC-369), and the
//! refusal of an old store is still [`crate::identity::check`]'s, which runs **first** and inside
//! the same transaction, so a refused open writes nothing (AC-180: *"no implicit in-place
//! migration occurs"*). Old stores are rebuilt from git (`doctor --rebuild`), never migrated in
//! place.

use std::path::Path;

use rusqlite::Connection;

use crate::conn::{StoreError, Writer};
use crate::identity::{self, Identity};

/// Apply every shipped migration this store has not applied yet, in order, and record the count.
///
/// # Errors
/// [`StoreError::Sqlite`] when a statement refuses.
pub fn apply(conn: &Connection) -> Result<(), StoreError> {
    let applied: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    let all = crate::schema_lint::all();
    for (i, (_name, sql, _keys)) in all.iter().enumerate() {
        if i64::try_from(i).is_ok_and(|i| i >= applied) {
            conn.execute_batch(sql)?;
        }
    }
    let total = i64::try_from(all.len()).map_err(|e| StoreError::Sqlite(e.to_string()))?;
    if total > applied {
        conn.pragma_update(None, "user_version", total)?;
    }
    Ok(())
}

/// Establish a store on `conn`, inside the caller's one transaction; `true` when it was fresh.
///
/// Reads the stamp, refuses a foreign or other-version store before any other statement, applies
/// the migrations, and stamps a fresh store.
///
/// # Errors
/// [`StoreError::Identity`] naming both identities or both schema versions (AC-179, AC-180);
/// [`StoreError::Sqlite`] otherwise.
pub fn establish(conn: &Connection, opening: &Identity) -> Result<bool, StoreError> {
    let stamped = identity::read(conn)?;
    if let Some(stamped) = &stamped {
        identity::check(stamped, opening)?;
    }
    apply(conn)?;
    let fresh = stamped.is_none();
    if fresh {
        identity::initialise(conn, opening)?;
    }
    Ok(fresh)
}

/// Open — or create — the store under `common_dir` and establish it in ONE write transaction:
/// the refusal is an `Err` out of the closure, the transaction rolls back, and the file is left
/// as it was.
///
/// # Errors
/// As [`establish`], plus [`Writer::open`]'s.
pub fn open(common_dir: &Path, opening: &Identity) -> Result<Writer, StoreError> {
    let writer = Writer::open(common_dir)?;
    writer.transaction(|tx| establish(tx, opening))?;
    Ok(writer)
}
