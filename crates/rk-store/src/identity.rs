//! The store's identity row — AC-179 and AC-180.
//!
//! Two refusals, and what makes them criteria rather than good manners is that **neither may read
//! or write a single row on the way to refusing**:
//!
//! * **AC-179** — a store opened against a repository with a different **root commit** is refused
//!   with a typed error naming *both* identities, and *"no row is read or written"*. Verified by a
//!   file digest unchanged after the refused open.
//! * **AC-180** — a store at an earlier **schema version** opened by a newer binary is refused,
//!   naming the migration required, and *"no implicit in-place migration occurs"*. Verified by the
//!   file's **bytes** being unchanged.
//!
//! The identity is the root commit plus the common dir plus the schema version (AC-369 bundles
//! those with the harness versions; `boot-id` was **cut** for having no source, and is not invented
//! here).

use std::path::Path;

use rusqlite::Connection;

use crate::conn::StoreError;

/// The schema version this binary writes and understands.
///
/// *(dated 2026-09-28, s8e build; ADR-0181 decision 5, proposed.)* **2**, from `1`: D21-46 R2's
/// `0002_gate_transitions`. A version-1 store is refused by [`check`] and rebuilt from git by
/// `doctor --rebuild`, never migrated in place (AC-180).
pub const SCHEMA_VERSION: u32 = 2;

/// What a store says it is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub root_commit: String,
    pub common_dir: String,
    pub schema_version: u32,
}

/// Create the identity table and stamp it, if it is not already stamped.
///
/// # Errors
/// [`StoreError::Sqlite`] on any failure.
pub fn initialise(conn: &Connection, identity: &Identity) -> Result<(), StoreError> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS rk_identity (
             id            INTEGER PRIMARY KEY CHECK (id = 1),
             root_commit   TEXT    NOT NULL,
             common_dir    TEXT    NOT NULL,
             schema_version INTEGER NOT NULL
         );",
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO rk_identity (id, root_commit, common_dir, schema_version)
         VALUES (1, ?1, ?2, ?3)",
        (
            &identity.root_commit,
            &identity.common_dir,
            identity.schema_version,
        ),
    )?;
    Ok(())
}

/// Read the stamped identity, if there is one.
///
/// # Errors
/// [`StoreError::Sqlite`] on any failure other than an absent table or row.
pub fn read(conn: &Connection) -> Result<Option<Identity>, StoreError> {
    let exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='rk_identity'",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);
    if !exists {
        return Ok(None);
    }
    let row = conn.query_row(
        "SELECT root_commit, common_dir, schema_version FROM rk_identity WHERE id = 1",
        [],
        |r| {
            Ok(Identity {
                root_commit: r.get(0)?,
                common_dir: r.get(1)?,
                schema_version: r.get(2)?,
            })
        },
    );
    match row {
        Ok(i) => Ok(Some(i)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Check a store's stamped identity against the repository being opened.
///
/// **This reads the identity row and nothing else**, and it writes nothing at all — the criteria's
/// "no row is read or written" is about the store's *data*, and the refusal has to read the stamp
/// to know what it is refusing. A caller runs this before touching any other table.
///
/// # Errors
/// [`StoreError::Identity`] naming both identities (AC-179) or both schema versions and the
/// migration required (AC-180).
pub fn check(stamped: &Identity, opening: &Identity) -> Result<(), StoreError> {
    if stamped.root_commit != opening.root_commit {
        return Err(StoreError::Identity(format!(
            "this store belongs to a different repository. stored root commit `{}` at common dir \
             `{}`; opening against root commit `{}` at common dir `{}`. Refusing; no row was read \
             or written (AC-179).",
            stamped.root_commit, stamped.common_dir, opening.root_commit, opening.common_dir
        )));
    }
    if stamped.schema_version < opening.schema_version {
        return Err(StoreError::Identity(format!(
            "this store is at schema version {} and this binary writes {}. A migration from {} to \
             {} is required; there is no implicit in-place migration and no `adopt this store` \
             fallback (AC-180).",
            stamped.schema_version,
            opening.schema_version,
            stamped.schema_version,
            opening.schema_version
        )));
    }
    if stamped.schema_version > opening.schema_version {
        return Err(StoreError::Identity(format!(
            "this store is at schema version {}, which is NEWER than the {} this binary \
             understands. Refusing rather than reading it as though the extra columns were absent.",
            stamped.schema_version, opening.schema_version
        )));
    }
    Ok(())
}

/// A content fingerprint of the store file, for the "no row read or written" assertions.
///
/// Content, never mtime: AC-159 makes the same point about artifacts — a clone, a checkout or a
/// format-on-save advances mtime with identical content, and an mtime-preserving write hides a real
/// mutation.
///
/// # Errors
/// [`StoreError::Sqlite`] if the file cannot be read.
pub fn file_digest(path: &Path) -> Result<rk_types::digest::Digest, StoreError> {
    let bytes = std::fs::read(path).map_err(|e| StoreError::Sqlite(e.to_string()))?;
    Ok(rk_types::digest::sha256(&bytes))
}
