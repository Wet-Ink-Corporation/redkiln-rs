//! AC-499's SQLite real row — **the same body the fake row runs** (RT-16 as re-amended, s7c
//! §7c.13): `rk_ports::conformance::conform_event_store`, from `rk-ports`' `conformance` feature
//! module over this crate's dev edge. No body of this crate's own.
//!
//! The body is a required core plus a capability arm. The core (append, then query, is identity
//! on what was appended) runs unchanged over the real store; the arm takes the tokens this row's
//! registry declares unsupported — `snapshot` and `rebuild`, D17-9 — and asserts
//! `Err(StepError::Unsupported { capability })` naming each, where 7a's unchanged
//! `store.snapshot().expect("snapshot")` would panic on this row. What the split is and is not
//! is stated once in `rk_ports::conformance`'s module doc; the register amendment it owes is
//! s7c §7c.22's.
//!
//! *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's tests-mutants minor.)*
//! ~~the arm takes … `snapshot` and `rebuild`, D17-9 — and asserts `Err(StepError::Unsupported …)`~~
//! is no longer what runs: since `s8e` §8e.9 (package S8E-P7) both registry rows carry
//! `Probe::Recorded`, so `unsupported_tokens()` returns neither, the unsupported arm is empty, and
//! the body's snapshot-then-rebuild ROUND TRIP runs over the real store (after the tier-1 arm).
//!
//! The real row adds what the fake cannot carry: ADR-0105 clause 7's `journal_mode = wal` and
//! `busy_timeout = 5000` on the writer connection (`wal_and_busy_timeout_are_enforced`).

use rk_adapter_sqlite::registry;
use rk_ports::conformance::conform_event_store;
use rk_store::conn::BUSY_TIMEOUT_MS;

use crate::support::{Fixture, open};

/// **AC-499 (SQLite row).** The shared body over the real store, with the capability arm fed from
/// this crate's registry rather than from a literal — a row that declared `snapshot` verified
/// without an implementation would reach the body's `Ok` path and panic on the opaque newtype.
///
/// RED (observed 2026-09-18) against the skeleton that applied no schema: the core's first
/// `append` failed with `no such table: t2_event`. Also observed red with the arm fed `&[]` —
/// the unchanged 7a shape — where the body panicked at its own `expect("snapshot")`: the row's
/// declaration is what keeps the body honest, not the body itself.
#[test]
fn port_conformance_real_row_sqlite() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);

    // The tokens come from the registry the adapter answers from (D17-9), never a literal here:
    // what the row declares is what the arm asserts, and `ac489_capability_registry_dated` is
    // where the declaration itself is pinned to exactly `snapshot` and `rebuild`.
    // *(dated 2026-09-28, `s8e` build, package R1FIX-8e.)* ~~pinned to exactly `snapshot` and
    // `rebuild`~~: `ac489_…` now pins both rows to a recorded probe (S8E-P7), so `arm` is empty
    // and the body runs the snapshot/rebuild round trip.
    let unsupported = registry::unsupported_tokens().expect("the shipped registry loads");
    let arm: Vec<&str> = unsupported.iter().map(String::as_str).collect();

    conform_event_store(&mut store, &arm);
}

/// **ADR-0105 clause 7, on the writer the adapter holds**: WAL journal, `busy_timeout` 5000 ms.
/// Read through the adapter's own writer — a fresh reader's pragmas would report the reader's
/// configuration, not the writer's.
///
/// RED (observed 2026-09-18) via a temporary mutant of `SqliteStore::open` that switched the
/// journal back to `DELETE` and the timeout to `0` after `Writer::open` — WAL and the timeout are
/// `rk-store`'s built behaviour (Step 6), so the absent-behaviour red is not available here and
/// the mutant is what shows the assertion bites; run and reverted, output quoted in the slice.
#[test]
fn wal_and_busy_timeout_are_enforced() {
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let store = open(&exec, &repo);

    let pragmas = store.pragmas().expect("pragmas readable");
    assert!(
        pragmas.journal_mode.eq_ignore_ascii_case("wal"),
        "journal_mode is `{}`, not WAL (ADR-0105 clause 7)",
        pragmas.journal_mode
    );
    assert_eq!(
        pragmas.busy_timeout_ms, BUSY_TIMEOUT_MS,
        "busy_timeout is not clause 7's {BUSY_TIMEOUT_MS} ms on the writer"
    );
    assert_eq!(
        BUSY_TIMEOUT_MS, 5000,
        "rk-store's constant is clause 7's value"
    );
}

/// **AC-499 (SQLite row), with the tier-1 arm** (ADR-0181 decision 1; `s8e` §8e.2): the shared
/// body now runs an anchored append through `append_tier1` and asserts, through the port, that
/// the event is queryable and that the row reports its anchor (`BySha`). Over the real store the
/// arm must also have written the `t1_stage_transition` row beside the anchored `t2_event` row,
/// which is what this row adds: the body cannot see a table, and a real row that answered the
/// port's queries without writing tier 1 would pass the body alone.
///
/// RED at entry: no tier-1 arm in the body and no `TIER_ONE_*` constants (E0425).
#[test]
fn sqlite_store_conforms_including_the_tier_one_arm() {
    use rk_ports::conformance::{TIER_ONE_ITEM, TIER_ONE_SHA, TIER_ONE_TREE};

    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    let mut store = open(&exec, &repo);
    let unsupported = registry::unsupported_tokens().expect("the shipped registry loads");
    let arm: Vec<&str> = unsupported.iter().map(String::as_str).collect();

    conform_event_store(&mut store, &arm);

    let reader = rk_store::conn::Reader::open(store.common_dir()).expect("a reader opens");
    let t1: (String, String, String) = reader
        .conn()
        .query_row(
            "SELECT item, anchor_sha, tree_hash FROM t1_stage_transition",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .expect("the arm's boundary wrote exactly one t1_stage_transition row");
    assert_eq!(
        t1,
        (
            TIER_ONE_ITEM.to_owned(),
            TIER_ONE_SHA.to_owned(),
            TIER_ONE_TREE.to_owned()
        ),
        "the tier-1 arm's row carries its item, its anchor and the anchor's tree"
    );
    let anchored: i64 = reader
        .conn()
        .query_row(
            "SELECT count(*) FROM t2_event WHERE anchor_sha = ?1",
            [TIER_ONE_SHA],
            |r| r.get(0),
        )
        .expect("count");
    assert_eq!(
        anchored, 1,
        "exactly one t2_event row carries the arm's anchor"
    );
}
