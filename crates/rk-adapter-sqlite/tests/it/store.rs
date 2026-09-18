//! The append-only store halves — AC-364 (sqlite half), AC-365 (7c half), AC-368 (the sqlite
//! negative). s7c §7c.13.

use std::path::Path;

use rk_ports::event_store::{EventStore, Query};
use rk_store::conn::Writer;
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::support::{Fixture, event, open};

fn all(store: &dyn EventStore) -> Vec<Event> {
    store.query(Query::ByItem("any".to_owned())).expect("query")
}

/// **AC-364 (sqlite half).** *"a correction is expressible only as a new appended event that
/// references the corrected one"* — *"an integration test that a correction produces two rows."*
///
/// The port-absence half (no `update`/`delete` method on `EventStore`) is `rk-ports`'
/// `ac364_event_store_port_has_no_update_or_delete`. What this half can add over the real store:
/// a correction that names the row it corrects lands as a **second** row, the first row's bytes
/// come back unchanged, and the adapter's source carries no `UPDATE` / `DELETE` statement at all
/// — the store *refuses* a modification by having no path that issues one.
///
/// RED (observed 2026-09-18) via a temporary mutant of `append` that, on a payload carrying
/// `corrects`, `UPDATE`d the referenced row in place: the count read 1, not 2, and the first row
/// carried the corrected value. Run and reverted; the output is quoted in the slice.
#[test]
fn ac364_correction_appends_never_mutates() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);

    let original = Event::new(
        EventKind::ReconcileStarted,
        serde_json::json!({ "n": 1, "note": "first reading" }),
    );
    store.append(original.clone()).expect("append original");
    // The correction references the corrected event by content — `Event` carries no id (the
    // typed-payload record is OWED, s7a §7a.5), so the reference is in the payload.
    let correction = Event::new(
        EventKind::ReconcileStarted,
        serde_json::json!({ "n": 1, "note": "corrected reading", "corrects": { "n": 1, "note": "first reading" } }),
    );
    store.append(correction.clone()).expect("append correction");

    let rows = all(&store);
    assert_eq!(
        rows.len(),
        2,
        "a correction produces TWO rows (AC-364); the store holds {rows:?}"
    );
    assert_eq!(rows[0], original, "the corrected row is unchanged");
    assert_eq!(rows[1], correction, "the correction is the second row");

    // The refusal half, on the source: no statement that modifies or deletes a row exists in the
    // adapter. A `-- comment` naming the token does not count; the filter is the block's.
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    for entry in std::fs::read_dir(&src_dir).expect("read src/") {
        let path = entry.expect("entry").path();
        if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).expect("read source");
            for (i, line) in text.lines().enumerate() {
                let t = line.trim_start();
                if t.starts_with("//") || t.starts_with('*') || t.starts_with("/*") {
                    continue;
                }
                let upper = t.to_ascii_uppercase();
                if upper.contains("UPDATE ") || upper.contains("DELETE FROM") {
                    hits.push(format!("{}:{}: {line}", path.display(), i + 1));
                }
            }
        }
    }
    assert!(
        hits.is_empty(),
        "the adapter's source carries a modifying statement (AC-364): {hits:?}"
    );
}

/// **AC-365 (7c half).** The deleted store file is **recreated on open** — with its schema, so
/// the first append after the recreation succeeds. Next-action equality and the repopulation from
/// git are AC-161's / 7d / Step 8's, not this half's.
///
/// RED (observed 2026-09-18) against the skeleton that applied no schema on open: the very first
/// `INSERT INTO t2_event` on the fresh file failed with `no such table: t2_event` — the second red
/// s7c §7c.18 names for this test.
#[test]
fn ac365_missing_store_file_recreated() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");

    let store_path = {
        let mut store = open(&exec, &repo);
        store.append(event(1)).expect("append on a fresh store");
        assert_eq!(all(&store).len(), 1);
        store.path().to_path_buf()
    };
    assert!(
        store_path.is_file(),
        "the store file exists after the first open"
    );

    // Delete the store file and its WAL/SHM sidecars (a deleted database is all three gone).
    for suffix in ["", "-wal", "-shm"] {
        let p = format!("{}{suffix}", store_path.display());
        if Path::new(&p).exists() {
            std::fs::remove_file(&p).expect("remove store file");
        }
    }
    assert!(!store_path.exists(), "PREMISE: the store file is gone");

    let mut store = open(&exec, &repo);
    assert_eq!(
        store.path(),
        store_path,
        "the recreated store is at the same path"
    );
    assert!(store_path.is_file(), "the store file was recreated on open");
    store
        .append(event(2))
        .expect("append on the recreated store — the schema was applied on open");
    let rows = all(&store);
    assert_eq!(
        rows,
        vec![event(2)],
        "the recreated store holds the new row and nothing else"
    );
}

/// **AC-368 (the sqlite negative).** What the SQLite crate can observe of AC-368 is only that a
/// failing append returns a **typed `Err`** and never panics; the warning-with-a-counter half is
/// the *caller*'s — `rk-adapter-claude`'s `ac368_tier2_append_failure_is_warning`, PROVISIONAL
/// (s7c §7c.13). The failure induced here is the criterion's *"corrupt store"*: a second
/// connection the test opens drops `t2_event` from under the adapter, so the next `INSERT` fails
/// on the store, not in the adapter's own code path. **Not induced: lock contention** — §7c.18's
/// *"busy store"*. `SQLITE_BUSY` past `busy_timeout` is `rk-store`'s documented behaviour
/// (`conn.rs`, Step 6), and inducing it here means a second writer holding `BEGIN IMMEDIATE` on
/// another thread for the full 5000 ms, a sleep-shaped wait the block bans (`thread::sleep`,
/// `Instant`) for one assertion the corrupt-store arm already makes at the same `map_err`;
/// stated as a deviation, not hidden.
///
/// RED (observed 2026-09-18) via a temporary mutant of `append` that `.expect("append")`ed the
/// transaction over the schema'd tree: the PREMISE append landed and the corrupt-store append
/// panicked — `panicked at crates/rk-adapter-sqlite/src/lib.rs:300:14: append: Sqlite("no such
/// table: t2_event")` — where an `Err` was due. Run and reverted. (The slice's first quote of this
/// red was the no-schema skeleton's, which panicked on the premise append; that was AC-365's red,
/// not this test's.)
#[test]
fn ac368_sqlite_append_error_is_typed_not_panic() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);
    store
        .append(event(1))
        .expect("PREMISE: a healthy store accepts an append");

    // Corrupt the store from a second connection — the test's own, standing in for another
    // process; the adapter itself holds one writer and opens no second (ADR-0105 clause 7).
    let other = Writer::open(store.common_dir()).expect("a second connection for the fault");
    other
        .transaction(|tx| {
            tx.execute_batch("DROP TABLE t2_event;")?;
            Ok(())
        })
        .expect("the fault is injected");
    drop(other);

    let err = store
        .append(event(2))
        .expect_err("an append on a corrupt store is an Err, not a panic and not Ok");
    match err {
        StepError::Transport {
            kind: TransportKind::Io,
            detail,
        } => assert!(
            detail.contains("t2_event"),
            "the typed error names the store failure: {detail}"
        ),
        other => panic!(
            "a store failure is Transport {{ Io }}, the closed variant for a tier-2 write that did not land (AC-229); got {other:?}"
        ),
    }
    // ...and the adapter is still usable as a value: no poisoned state, no second panic.
    let err2 = store
        .append(event(3))
        .expect_err("still typed on the second attempt");
    assert!(matches!(err2, StepError::Transport { .. }));
}
