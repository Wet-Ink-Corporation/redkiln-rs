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

// ---------------------------------------------------------------- s8e §8e.2 / §8e.10: ADR-0181, ADR-0182

/// Every row of `sql` as strings (a `NULL` reads as `"NULL"`), over a fresh reader on the store the
/// adapter opened. `rusqlite` is not this crate's dependency, so nothing here names its types.
fn rows(store: &rk_adapter_sqlite::SqliteStore, sql: &str) -> Vec<Vec<String>> {
    let reader = rk_store::conn::Reader::open(store.common_dir()).expect("a reader opens");
    let mut stmt = reader.conn().prepare(sql).expect("the probe prepares");
    let width = stmt.column_count();
    stmt.query_map([], |r| {
        (0..width)
            .map(|i| {
                r.get::<_, Option<String>>(i)
                    .map(|v| v.unwrap_or_else(|| "NULL".to_owned()))
            })
            .collect::<Result<Vec<String>, _>>()
    })
    .expect("the probe runs")
    .map(|row| row.expect("a row reads"))
    .collect()
}

/// **`SqliteStore::append_tier1`** (ADR-0181 decisions 1–3): the `t2_event` row carries the
/// anchor and the interim item, and the kind's `t1_*` row is written beside it — a boundary's
/// `t1_stage_transition` with the anchor commit's tree, a gate opening's `t1_gate_transition` with
/// the opening run, a seal's `t1_slice_seal`. **In one transaction**: a boundary that cannot name
/// its item refuses, and the `t2_event` row it would have anchored does not exist either.
///
/// RED at entry: no `append_tier1` (E0599); against an adapter that appends the row the ordinary
/// way, `anchor_sha` is NULL and no `t1_*` row exists.
#[test]
fn append_tier1_writes_the_t1_row_and_the_anchored_t2_row_in_one_transaction() {
    use rk_ports::event_store::TierOneAnchor;

    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);
    let (sha, tree) = ("a".repeat(40), "b".repeat(40));

    let boundary = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "triage", "to": "implement" }),
    )
    .with_item("item-a");
    store
        .append_tier1(boundary, &TierOneAnchor::new(&sha, &tree))
        .expect("the boundary appends");
    assert_eq!(
        rows(&store, "SELECT kind, anchor_sha, item FROM t2_event"),
        vec![vec![
            "stage_transition".to_owned(),
            sha.clone(),
            "item-a".to_owned()
        ]],
        "the t2_event row carries its anchor and its item"
    );
    assert_eq!(
        rows(
            &store,
            "SELECT item, from_stage, to_stage, anchor_sha, tree_hash FROM t1_stage_transition"
        ),
        vec![vec![
            "item-a".to_owned(),
            "triage".to_owned(),
            "implement".to_owned(),
            sha,
            tree.clone()
        ]],
        "the t1_stage_transition row is written beside it, with the anchor commit's tree"
    );

    let token = "c".repeat(64);
    let opened = Event::new(
        EventKind::GateOpened,
        serde_json::json!({ "deadline": 1_760_000_000, "kind": "human", "reach": 1, "token": token, "tree_hash": tree }),
    )
    .with_item("item-a");
    let open_sha = "d".repeat(40);
    store
        .append_tier1(
            opened,
            &TierOneAnchor::new(&open_sha, &tree)
                .with_opening_run("implement", "implement-0123456789abcdef"),
        )
        .expect("the opening appends");
    assert_eq!(
        rows(
            &store,
            "SELECT token, state, tree_hash, deadline, kind, reach, stage, run_id, anchor_sha FROM t1_gate_transition"
        ),
        vec![vec![
            token,
            "opened".to_owned(),
            tree.clone(),
            "1760000000".to_owned(),
            "human".to_owned(),
            "1".to_owned(),
            "implement".to_owned(),
            "implement-0123456789abcdef".to_owned(),
            open_sha,
        ]],
        "a gate opening writes its t1_gate_transition row, the opening run from the anchor"
    );

    let seal = Event::new(
        EventKind::SliceSealed,
        serde_json::json!({ "project": "redkiln-rs", "slice": "s-1", "verdict": "approved" }),
    )
    .with_item("item-a");
    let seal_sha = "e".repeat(40);
    store
        .append_tier1(seal, &TierOneAnchor::new(&seal_sha, &tree))
        .expect("the seal appends");
    assert_eq!(
        rows(
            &store,
            "SELECT project, slice, verdict, anchor_sha FROM t1_slice_seal"
        ),
        vec![vec![
            "redkiln-rs".to_owned(),
            "s-1".to_owned(),
            "approved".to_owned(),
            seal_sha
        ]],
        "a seal writes its t1_slice_seal row"
    );

    // ONE transaction: a boundary with no item cannot write its t1 row, so the whole act refuses
    // and the anchored t2_event row it began with is rolled back too.
    let before = rows(&store, "SELECT CAST(count(*) AS TEXT) FROM t2_event");
    let orphan = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "implement", "to": "review" }),
    );
    let refused = store.append_tier1(orphan, &TierOneAnchor::new("f".repeat(40), &tree));
    assert!(
        matches!(&refused, Err(StepError::Transport { kind: TransportKind::Io, detail }) if detail.contains("item")),
        "a boundary with no item refuses, naming the member (ADR-0181 decision 3): {refused:?}"
    );
    assert_eq!(
        rows(&store, "SELECT CAST(count(*) AS TEXT) FROM t2_event"),
        before,
        "the refused act left no anchored t2_event row: both rows are one transaction"
    );
    assert_eq!(
        rows(
            &store,
            "SELECT CAST(count(*) AS TEXT) FROM t1_stage_transition"
        ),
        vec![vec!["1".to_owned()]]
    );
}

/// **`query(ByItem(x))` in `rk-store`'s SQL** (ADR-0182 decision 4): the rows keyed `x`, and in
/// the interim the unkeyed rows, in append order, never a row keyed to another item; `BySha`
/// selects by anchor (ADR-0181 decision 12); `ByRun` stays inert (ADR-0182 decision 5).
///
/// RED at entry: no `Event::with_item` (E0599); against the adapter whose `query` is
/// `fn query(&self, _by: Query)` over `SELECT kind, payload FROM t2_event ORDER BY id`, `_by` is
/// ignored and item-b's row comes back under `ByItem("item-a")`.
#[test]
fn query_by_item_returns_only_that_items_rows() {
    use rk_ports::event_store::TierOneAnchor;

    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);

    let a = event(1).with_item("item-a");
    let b = event(2).with_item("item-b");
    let unkeyed = event(3);
    for e in [&a, &b, &unkeyed] {
        store.append(e.clone()).expect("append");
    }
    let sha = "a".repeat(40);
    let anchored = Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "triage", "to": "implement" }),
    )
    .with_item("item-a");
    store
        .append_tier1(anchored.clone(), &TierOneAnchor::new(&sha, "b".repeat(40)))
        .expect("append_tier1");

    assert_eq!(
        store.query(Query::ByItem("item-a".to_owned())).expect("a"),
        vec![a.clone(), unkeyed.clone(), anchored.clone()],
        "ByItem(item-a): item-a's rows and the unkeyed row, with the key read back, never item-b's"
    );
    assert_eq!(
        store.query(Query::ByItem("item-b".to_owned())).expect("b"),
        vec![b.clone(), unkeyed.clone()],
        "ByItem(item-b): item-b's row and the unkeyed row"
    );
    assert_eq!(
        store.query(Query::BySha(sha)).expect("sha"),
        vec![anchored.clone()],
        "BySha selects the row anchored to that commit"
    );
    assert_eq!(
        store.query(Query::ByRun("any".to_owned())).expect("run"),
        vec![a, b, unkeyed, anchored],
        "ByRun stays inert"
    );
}

/// **`SqliteStore::open_at` targets a staging file, and `rebuild` writes anchored tier-1 rows**
/// (`s8e` §8e.9; D21-42 8c: *"`open_at` / `Capability::Rebuild`"*; ADR-0181 decision 11, proposed).
/// *(Added 2026-09-28, `s8e` build, package S8E-P7.)*
///
/// A store holding every shape a snapshot must carry — an unanchored tier-2 row, a boundary with
/// its anchor tree, a gate opening with its opening run, an answer, and a seal — is snapshotted,
/// and the snapshot is rebuilt into a store opened with `open_at` on a STAGING root. The staging
/// file is `<root>/redkiln/store.sqlite3`, not the repository's; the repository's store is not
/// touched; and every `t2_event` row and every `t1_*` row comes back equal, anchors and tree
/// included. A rebuild over a store whose rows are not a prefix of the snapshot refuses and writes
/// nothing (append-only, AC-364). RED at entry: no `open_at` (E0599), and `snapshot`/`rebuild`
/// answered `Unsupported` from their probe-less registry rows.
#[test]
fn open_at_targets_a_staging_file_and_rebuild_writes_anchored_tier_one_rows() {
    use rk_ports::event_store::TierOneAnchor;

    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut source = open(&exec, &repo);
    let (sha1, sha2, sha3, sha4) = (
        "1".repeat(40),
        "2".repeat(40),
        "3".repeat(40),
        "4".repeat(40),
    );
    let tree = "b".repeat(40);
    let token = "c".repeat(64);

    source.append(event(1)).expect("an ordinary row");
    source
        .append_tier1(
            Event::new(
                EventKind::StageTransition,
                serde_json::json!({ "from": "triage", "to": "implement" }),
            )
            .with_item("item-a"),
            &TierOneAnchor::new(&sha1, &tree),
        )
        .expect("a boundary");
    source
        .append_tier1(
            Event::new(
                EventKind::GateOpened,
                serde_json::json!({ "deadline": 1_760_000_000, "kind": "human", "reach": 1, "token": token, "tree_hash": tree }),
            )
            .with_item("item-a"),
            &TierOneAnchor::new(&sha2, &tree).with_opening_run("implement", "implement-0123456789abcdef"),
        )
        .expect("an opening");
    source
        .append_tier1(
            Event::new(
                EventKind::GateAnswered,
                serde_json::json!({ "token": token, "tree_hash": tree, "verdict": "approve" }),
            )
            .with_item("item-a"),
            &TierOneAnchor::new(&sha3, &tree),
        )
        .expect("an answer");
    source
        .append_tier1(
            Event::new(
                EventKind::SliceSealed,
                serde_json::json!({ "project": "demo", "slice": "sl-1", "stage": "review", "verdict": "approved" }),
            )
            .with_item("item-a"),
            &TierOneAnchor::new(&sha4, &tree),
        )
        .expect("a seal");

    let snapshot = source.snapshot().expect("the snapshot reads");
    assert_eq!(
        snapshot.rows().len(),
        5,
        "PREMISE: every row is in the snapshot"
    );
    assert_eq!(
        snapshot.rows()[1].tree.as_deref(),
        Some(tree.as_str()),
        "the boundary's row carries its anchor tree"
    );
    assert_eq!(
        snapshot.rows()[2].opening_run,
        Some((
            "implement".to_owned(),
            "implement-0123456789abcdef".to_owned()
        )),
        "the opening's row carries its opening run"
    );

    // ---- the staging store, under a root that is NOT the repository's common dir.
    let staging_root = fx.path().join("staging");
    let identity = source.identity().clone();
    let repo_store_before = std::fs::read(source.path()).expect("the repository's store reads");
    let mut staged = rk_adapter_sqlite::SqliteStore::open_at(&staging_root, &identity)
        .expect("open_at opens a fresh staging store");
    assert_eq!(
        staged.path(),
        staging_root.join("redkiln").join("store.sqlite3").as_path(),
        "open_at's file is <root>/redkiln/store.sqlite3"
    );
    assert_ne!(
        staged.path(),
        source.path(),
        "the staging file is not the repository's"
    );
    staged
        .rebuild(&snapshot)
        .expect("the rebuild writes the snapshot");

    for sql in [
        "SELECT kind, payload, anchor_sha, item FROM t2_event ORDER BY id",
        "SELECT item, from_stage, to_stage, anchor_sha, tree_hash FROM t1_stage_transition ORDER BY id",
        "SELECT token, state, tree_hash, deadline, kind, reach, option_id, stage, run_id, anchor_sha FROM t1_gate_transition ORDER BY id",
        "SELECT project, slice, verdict, anchor_sha FROM t1_slice_seal ORDER BY id",
    ] {
        let want = rows(&source, sql);
        assert!(
            !want.is_empty(),
            "PREMISE: the source holds rows for `{sql}`"
        );
        assert_eq!(
            rows(&staged, sql),
            want,
            "the rebuilt store differs on `{sql}`"
        );
    }
    assert_eq!(
        staged.snapshot().expect("the rebuilt snapshot reads"),
        snapshot,
        "the rebuilt store's snapshot is the snapshot it was rebuilt from"
    );
    assert_eq!(
        std::fs::read(source.path()).expect("the repository's store reads"),
        repo_store_before,
        "the rebuild wrote nothing to the repository's store"
    );

    // ---- append-only: a store whose rows are not a prefix of the snapshot refuses, writing nothing.
    let other_root = fx.path().join("other");
    let mut other = rk_adapter_sqlite::SqliteStore::open_at(&other_root, &identity).expect("opens");
    other.append(event(99)).expect("a divergent row");
    let err = other
        .rebuild(&snapshot)
        .expect_err("a divergent store refuses the rebuild");
    assert!(
        format!("{err:?}").contains("not a prefix"),
        "the refusal names the prefix rule: {err:?}"
    );
    assert_eq!(
        rows(&other, "SELECT CAST(count(*) AS TEXT) FROM t2_event"),
        vec![vec!["1".to_owned()]],
        "a refused rebuild writes nothing"
    );
}
