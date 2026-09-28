//! **AC-489 / AC-492 (the SQLite row)** — the dated capability registry behind `snapshot` /
//! `rebuild` (D17-9; RT-24; s7c §7c.7).
//!
//! AC-489: *"a row missing `capability`, `verified_on` or `probe` fails the build naming the row /
//! AND no capability is readable by the kernel except through a row carrying all three."*
//! *Verified by:* *"a registry schema test at build time (`serde` deny-unknown + required
//! fields); a fixture registry with a bare `{capability}` row that must fail."*
//!
//! AC-492: *"an adapter registry naming a capability token … the canonical vocabulary does not
//! define … the build fails naming the unknown token / AND no code path maps an unknown token to
//! a default, an empty set or a no-op."* *Verified by:* *"a fixture registry with `capability =
//! "reads_minds"` that must fail; a lint forbidding a catch-all match arm in the token mapper."*
//!
//! **What the SQLite entries are**: exactly the `snapshot` and `rebuild` rows D17-9 names, each
//! dated and each **explicitly probe-less** (`Probe::None { reason }`) — a row is present, dated
//! and says why it has no probe, never absent-member (§7c.7). *(dated 2026-09-28, `s8e` build, package S8E-P7:
//! both rows now carry a recorded probe — `s8e` §8e.9 implemented snapshot and rebuild —
//! `ac489_…` is re-pointed and `snapshot_and_rebuild_rows_carry_a_recorded_probe` is new.)* A `test -f registry.rs` plus a
//! `verified_on` grep is the block's floor, never the whole assertion; this is the rest.

use std::path::Path;

use rk_ports::event_store::{EventStore, Snapshot};
use rk_types::error::{StepError, TransportKind};

use rk_adapter_sqlite::SqliteStore;
use rk_adapter_sqlite::registry::{self, Capability, Probe, Row};

use crate::support::{Fixture, open, wt};

const BARE_ROW: &str = include_str!("../fixtures/registry/bare_row.json");
const UNDATED_ROW: &str = include_str!("../fixtures/registry/undated_row.json");
const PROBELESS_ROW: &str = include_str!("../fixtures/registry/probeless_row.json");
const UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/unknown_member.json");
const PROBE_UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/probe_unknown_member.json");
const READS_MINDS: &str = include_str!("../fixtures/registry/reads_minds.json");

/// `YYYY-MM-DD`, digits in the right places — a dated entry, not a free string.
fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

/// The snapshot must come from somewhere the body can hand the row: `Snapshot` is `rk-ports`'
/// opaque newtype, and the shared body is what carries one. Here the arm is exercised directly
/// through `rebuild`'s parameter type via the fake, which is the only constructor that exists.
fn a_snapshot() -> Snapshot {
    use rk_ports::fakes::VecStore;
    VecStore::default().snapshot().expect("the fake mints one")
}

/// A `Transport { kind: Decode, detail }` whose detail names `token` — the shape a registry
/// defect surfaces as, never `Unsupported` and never a panic.
fn decode_naming(err: &StepError, token: &str, what: &str) {
    match err {
        StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        } => assert!(
            detail.contains(token),
            "{what}: the decode failure names `{token}`: {detail}"
        ),
        other => panic!("{what}: a registry defect is Transport {{ Decode }}, got {other:?}"),
    }
}

/// RED (observed 2026-09-18) against the skeleton registry — an empty row list with defaulted
/// members: the bare row LOADED, and `snapshot()` answered `Unsupported` with no row behind it.
///
/// RED again (observed 2026-09-18) on the derivation: with `registry::refusal` hard-coded to
/// `Unsupported { capability }` and never reading its rows — the mutant three reviews ran against
/// the first form of this test and watched survive — the store-level assertions below fail at
/// "snapshot over an EMPTY registry … a registry defect is Transport { Decode }, got
/// Unsupported { capability: "snapshot" }". Quoted in the slice.
#[test]
fn ac489_capability_registry_dated() {
    // ---- the must-fail fixtures: every member required, no unknown member, at every level.
    let err = registry::load(BARE_ROW).expect_err("a bare capability-only row must not load");
    let text = err.to_string();
    assert!(
        text.contains("verified_on") || text.contains("probe"),
        "the refusal names the missing member: {text}"
    );
    // The bare row is missing BOTH members and serde names the first it meets, so each member
    // is pinned on its own: a `#[serde(default)]` on `verified_on` would let an undated row load
    // (the `snapshot` row without `verified_on` s7c §7c.18 names), and a defaulted `probe` would
    // make an absent member read as `Probe::None`. Each fixture must fail naming ITS member.
    let err = registry::load(UNDATED_ROW).expect_err("an undated row must not load (AC-489)");
    let text = err.to_string();
    assert!(
        text.contains("verified_on") && !text.contains("probe"),
        "the undated row's refusal names `verified_on` and nothing else missing: {text}"
    );
    let err = registry::load(PROBELESS_ROW).expect_err("a probe-less row must not load (AC-489)");
    let text = err.to_string();
    assert!(
        text.contains("probe") && !text.contains("verified_on"),
        "the probe-less row's refusal names `probe` and nothing else missing: {text}"
    );
    let err = registry::load(UNKNOWN_MEMBER).expect_err("an undeclared member must not load");
    assert!(
        err.to_string().contains("adopted"),
        "the refusal names the unknown member: {err}"
    );
    let err = registry::load(PROBE_UNKNOWN_MEMBER)
        .expect_err("an undeclared member on the probe must not load");
    assert!(
        err.to_string().contains("extra"),
        "the refusal names the unknown probe member: {err}"
    );

    // ---- the shipped registry: exactly the two rows D17-9 names, dated. *(Re-pointed 2026-09-28,
    // `s8e` build, package S8E-P7; `s8e` §8e.9; ADR-0181 decision 11, proposed: ~~explicitly
    // probe-less~~ — both rows flipped together to a recorded probe naming the test that exercises
    // the method, and the store now answers both.)*
    let rows = registry::rows().expect("the shipped registry loads");
    let tokens: Vec<&str> = rows.iter().map(registry::Row::token).collect();
    assert_eq!(
        tokens,
        vec!["snapshot", "rebuild"],
        "the SQLite registry carries exactly the snapshot and rebuild rows (D17-9)"
    );
    for row in &rows {
        assert!(
            is_date(&row.verified_on),
            "row {} is not dated YYYY-MM-DD: {:?}",
            row.token(),
            row.verified_on
        );
        match &row.probe {
            Probe::Recorded { source } => assert!(
                !source.trim().is_empty(),
                "row {} claims a recorded probe with no source",
                row.token()
            ),
            Probe::None { reason } => panic!(
                "row {} is probe-less ({reason:?}) — `s8e` §8e.9 implemented snapshot and rebuild, \
                 and the two rows flip together",
                row.token()
            ),
        }
    }
    assert!(
        registry::unsupported_tokens().expect("loads").is_empty(),
        "no shipped row is probe-less, so the real conformance row's capability arm is empty"
    );

    // ---- the derivation, as a pure function over rows (D17-9: "from a dated registry entry").
    // No row: a typed decode failure naming the missing row, never Unsupported by default.
    decode_naming(
        &registry::refusal(&[], Capability::Snapshot),
        "snapshot",
        "no row for snapshot",
    );
    decode_naming(
        &registry::answer(&[], Capability::Snapshot).expect_err("no row answers nothing"),
        "snapshot",
        "answer with no row for snapshot",
    );
    // A recorded row: the method answers.
    let recorded = Row {
        capability: Capability::Snapshot,
        verified_on: "2026-09-28".to_owned(),
        probe: Probe::Recorded {
            source: "a recorded probe".to_owned(),
        },
    };
    assert_eq!(
        registry::answer(std::slice::from_ref(&recorded), Capability::Snapshot),
        Ok(()),
        "a row carrying a recorded probe lets the method answer"
    );
    // A probe-less row: Unsupported, the token the row's.
    let probeless = |capability: Capability| Row {
        capability,
        verified_on: "2026-09-18".to_owned(),
        probe: Probe::None {
            reason: "a controlled probe-less row".to_owned(),
        },
    };
    match registry::answer(&[probeless(Capability::Snapshot)], Capability::Snapshot) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, "snapshot"),
        other => panic!("a probe-less snapshot row answers Unsupported; got {other:?}"),
    }
    // Only a probe-less `rebuild` present: rebuild is Unsupported and snapshot is the missing-row
    // failure — one row does not answer for another.
    let rebuild_only = vec![probeless(Capability::Rebuild)];
    match registry::answer(&rebuild_only, Capability::Rebuild) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, "rebuild"),
        other => panic!("rebuild over its own probe-less row is Unsupported; got {other:?}"),
    }
    decode_naming(
        &registry::answer(&rebuild_only, Capability::Snapshot).expect_err("no snapshot row"),
        "snapshot",
        "snapshot with only a rebuild row",
    );

    // ---- the adapter answers FROM the rows it holds: the shipped rows let both methods run, and a
    // store opened over a registry with no row answers the decode failure, not Unsupported.
    let fx = Fixture::new();
    let exec = fx.exec();
    let repo = fx.repo_with_one_commit("repo");
    {
        let mut store = open(&exec, &repo);
        let snapshot = store
            .snapshot()
            .expect("the shipped snapshot row lets the method answer");
        store
            .rebuild(&snapshot)
            .expect("the shipped rebuild row lets the method answer; a snapshot of the current state rebuilds as identity");
        store
            .rebuild(&a_snapshot())
            .expect("an empty snapshot is a prefix-compatible rebuild over an empty store");
    }
    {
        let mut store = SqliteStore::open_over_registry(&exec, &wt(&repo), Vec::new())
            .expect("the store opens; the registry is consulted by the port methods, not by open");
        decode_naming(
            &store.snapshot().expect_err("no row, no answer"),
            "snapshot",
            "snapshot over an EMPTY registry",
        );
        decode_naming(
            &store.rebuild(&a_snapshot()).expect_err("no row, no answer"),
            "rebuild",
            "rebuild over an EMPTY registry",
        );
    }
    {
        let mut store =
            SqliteStore::open_over_registry(&exec, &wt(&repo), rebuild_only).expect("opens");
        decode_naming(
            &store.snapshot().expect_err("no snapshot row"),
            "snapshot",
            "snapshot over a registry carrying only rebuild",
        );
        match store.rebuild(&a_snapshot()) {
            Err(StepError::Unsupported { capability }) => assert_eq!(capability, "rebuild"),
            other => panic!("rebuild over its own probe-less row is Unsupported; got {other:?}"),
        }
    }
}

/// **`s8e` §8e.9 — both SQLite rows carry a recorded probe, and the probe names a real test**
/// (D21-42 8c: *"`open_at` / `Capability::Rebuild`"*; ADR-0181 decision 11, proposed). *(Added
/// 2026-09-28, `s8e` build, package S8E-P7.)*
///
/// `conform_event_store` treats a registry that declares one of snapshot/rebuild unsupported and
/// not the other as a defect, so the rows flip together. Each row's `source` is a `;`-separated
/// list of `<path> <module>::<test>`; every path must exist and define that test, so a row cannot
/// cite a probe nobody wrote. RED at entry: both rows were `Probe::None`. Red under S8E-M39 (the
/// `rebuild` row alone restored to `Probe::None`).
#[test]
fn snapshot_and_rebuild_rows_carry_a_recorded_probe() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let rows = registry::rows().expect("the shipped registry loads");
    for capability in [Capability::Snapshot, Capability::Rebuild] {
        let row = rows
            .iter()
            .find(|r| r.capability == capability)
            .unwrap_or_else(|| panic!("no `{}` row", capability.as_str()));
        let Probe::Recorded { source } = &row.probe else {
            panic!(
                "the `{}` row is {:?}, not a recorded probe — `s8e` §8e.9 implements it",
                capability.as_str(),
                row.probe
            );
        };
        let mut cited = 0_usize;
        for entry in source.split(';').map(str::trim).filter(|e| !e.is_empty()) {
            let (path, test) = entry
                .split_once(' ')
                .unwrap_or_else(|| panic!("`{entry}` is not `<path> <module>::<test>`"));
            let name = test.rsplit("::").next().expect("a test name");
            let text = std::fs::read_to_string(workspace.join(path))
                .unwrap_or_else(|e| panic!("the probe source {path} is unreadable: {e}"));
            assert!(
                text.contains(&format!("fn {name}(")),
                "the `{}` row cites {test} in {path}, which defines no such test",
                capability.as_str()
            );
            cited += 1;
        }
        assert!(
            cited >= 1,
            "the `{}` row cites no probe",
            capability.as_str()
        );
        assert_eq!(
            registry::answer(&rows, capability),
            Ok(()),
            "the shipped `{}` row lets the method answer",
            capability.as_str()
        );
    }
}

#[test]
fn ac492_unknown_capability_token_fails() {
    let err = registry::load(READS_MINDS).expect_err("`reads_minds` is not a capability");
    assert!(
        err.to_string().contains("reads_minds"),
        "the refusal names the unknown token: {err}"
    );

    // Every shipped token is one the closed vocabulary spells — and the spelling is the
    // register's, hyphen included, for the one token that carries one.
    let known = [
        "dispatch",
        "snapshot",
        "rebuild",
        "change_summary",
        "affected-selection",
    ];
    for row in registry::rows().expect("loads") {
        assert!(
            known.contains(&row.token()),
            "the shipped registry names {} outside the canonical vocabulary",
            row.token()
        );
    }

    // The lint half: no catch-all arm in the token mapper. The mapper is `Capability::as_str`
    // in `src/registry.rs`; a `_ =>` arm there would let a variant added without a spelling
    // compile, which is the default AC-492's AND forbids. Comment lines are filtered as the
    // block filters them.
    let src =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/registry.rs"))
            .expect("read src/registry.rs");
    let catch_alls: Vec<String> = src
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            let t = l.trim_start();
            !(t.starts_with("//") || t.starts_with('*') || t.starts_with("/*"))
        })
        .filter(|(_, l)| l.trim_start().starts_with("_ =>") || l.contains(" _ =>"))
        .map(|(i, l)| format!("registry.rs:{}: {l}", i + 1))
        .collect();
    assert!(
        catch_alls.is_empty(),
        "a catch-all arm in the token mapper maps an unknown token to a default (AC-492): {catch_alls:?}"
    );
    assert!(
        src.contains("pub enum Capability"),
        "the vocabulary is a closed enum named Capability (RT-24; the block digests its body)"
    );
}
