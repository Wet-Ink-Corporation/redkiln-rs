//! **AC-489 / AC-492 / AC-328 (the Rust row)** — the dated capability registry behind
//! `change_summary` (D17-9) and `affected-selection` (AC-328); RT-24; s7c §7c.7, §7c.14.
//!
//! AC-489: *"a row missing `capability`, `verified_on` or `probe` fails the build naming the row /
//! AND no capability is readable by the kernel except through a row carrying all three."*
//! AC-492: *"an adapter registry naming a capability token … the canonical vocabulary does not
//! define … the build fails naming the unknown token / AND no code path maps an unknown token to
//! a default, an empty set or a no-op."*
//! AC-328: *"GIVEN a toolchain adapter whose `affected-selection` capability has no probe result
//! recorded at the current version pin / WHEN the kernel resolves a verify activity for that
//! toolchain / THEN selection is refused with `StepError::Unsupported("affected-selection")` …
//! AND no `basis: affected` record can be produced for that toolchain while the probe entry is
//! absent or stale."* *Verified by:* *"capability-registry unit test; a fixture adapter with an
//! expired `verified_on` that must produce `Unsupported`; an assertion that no run under that
//! fixture emits a record with `basis: affected`."*
//!
//! **What the Rust entries are**: exactly `change_summary` (S-BIN unrun) and `affected-selection`
//! (no probe exists at any pin — §7c.14 [measured 2026-09-17]), each dated and each **explicitly
//! probe-less** (`Probe::None { reason }`).
//!
//! *(Amended 2026-09-26, s8c build, P5; record T = ADR-0169 decision 7 rule 1, proposed: the
//! `affected-selection` row now carries a dated `Probe::Recorded { source }` naming the fixture
//! workspace and the `affected::` test that exercises it; `change_summary` alone stays probe-less.
//! `ac328_…` keeps its name and runs over a controlled probe-less registry.)*

use std::path::Path;

use rk_ports::toolchain::{Selection, StepBound, Toolchain};
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason};

use rk_adapter_toolchain_rust::RustToolchain;
use rk_adapter_toolchain_rust::registry::{self, Capability, Probe, Row};

use crate::support::{
    GENEROUS, binary_range_repo, config_with, fixture_project, is_date, minimal_env,
};

const BARE_ROW: &str = include_str!("../fixtures/registry/bare_row.json");
const UNDATED_ROW: &str = include_str!("../fixtures/registry/undated_row.json");
const PROBELESS_ROW: &str = include_str!("../fixtures/registry/probeless_row.json");
const UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/unknown_member.json");
const PROBE_UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/probe_unknown_member.json");
const READS_MINDS: &str = include_str!("../fixtures/registry/reads_minds.json");
const INVENTED_PROBE: &str = include_str!("../fixtures/registry/invented_probe.json");

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

/// **AC-489.** Every member required at every level; the shipped rows dated and explicitly
/// probe-less; the adapter answers FROM its rows, and a missing row is a decode failure.
#[test]
fn ac489_capability_registry_dated() {
    let err = registry::load(BARE_ROW).expect_err("a bare capability-only row must not load");
    let text = err.to_string();
    assert!(
        text.contains("verified_on") || text.contains("probe"),
        "the refusal names the missing member: {text}"
    );
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

    // ---- the shipped registry: exactly the two rows, dated, explicitly probe-less.
    // *(Amended 2026-09-26, s8c build, P5; record T = ADR-0169 decision 7 rule 1, proposed: "explicitly
    // probe-less" holds for `change_summary` only. The `affected-selection` row carries a dated
    // `Recorded { source }` naming the fixture workspace and the test that exercises it; that
    // row's behaviour is proved by the `affected::` tests.)*
    let rows = registry::rows().expect("the shipped registry loads");
    let tokens: Vec<&str> = rows.iter().map(Row::token).collect();
    assert_eq!(
        tokens,
        vec!["change_summary", "affected-selection"],
        "the Rust registry carries exactly the change_summary (D17-9) and affected-selection (AC-328) rows"
    );
    for row in &rows {
        assert!(
            is_date(&row.verified_on),
            "row {} is not dated YYYY-MM-DD: {:?}",
            row.token(),
            row.verified_on
        );
        match (&row.probe, row.capability) {
            (Probe::None { reason }, Capability::ChangeSummary) => assert!(
                !reason.trim().is_empty(),
                "row {} says Probe::None with no reason",
                row.token()
            ),
            (Probe::Recorded { source }, Capability::AffectedSelection) => assert!(
                source.contains("tests/fixtures/workspace")
                    && source.contains(
                        "affected::affected_set_is_touched_crates_plus_reverse_dependency_closure"
                    ),
                "the affected-selection row's recorded probe names the fixture workspace and the \
                 test that exercises it (record T decision 7 rule 1): {source:?}"
            ),
            (probe, _) => panic!(
                "row {} carries {probe:?}: change_summary stays probe-less (S-BIN is unrun) and \
                 affected-selection is Recorded from s8c on",
                row.token()
            ),
        }
    }
    assert_eq!(
        registry::unsupported_tokens().expect("loads"),
        vec!["change_summary".to_owned()],
        "only change_summary is answered Unsupported from its row (amended 2026-09-26, s8c: \
         affected-selection is Recorded)"
    );

    // ---- the derivation, as a pure function over rows (D17-9: "from a dated registry entry").
    decode_naming(
        &registry::refusal(&[], Capability::ChangeSummary),
        "change_summary",
        "no row for change_summary",
    );
    let recorded = Row {
        capability: Capability::ChangeSummary,
        verified_on: "2026-09-18".to_owned(),
        probe: Probe::Recorded {
            source: "a probe nobody ran".to_owned(),
        },
    };
    decode_naming(
        &registry::refusal(std::slice::from_ref(&recorded), Capability::ChangeSummary),
        "change_summary",
        "a recorded-probe row for change_summary",
    );
    match registry::refusal(&rows, Capability::ChangeSummary) {
        StepError::Unsupported { capability } => assert_eq!(capability, "change_summary"),
        other => panic!("the shipped change_summary row answers Unsupported; got {other:?}"),
    }

    // ---- the change_summary row is re-dated and its reason narrowed to the sealed arm.
    // *(amended 2026-09-24, s8a, ADR-0165: the row said "no Ok is constructible" of an opaque
    // newtype. The Text arm is now produced; the row answers only what S-BIN still owns.)*
    match &rows[0].probe {
        Probe::None { reason } => assert!(
            reason.contains("Binary arm") && reason.contains("ADR-0165"),
            "the change_summary row's reason is narrowed to the sealed Binary arm and cites the \
             record: {reason:?}"
        ),
        other => panic!("the change_summary row stays explicitly probe-less: {other:?}"),
    }

    // ---- the adapter answers FROM the rows it holds.
    // *(amended 2026-09-24, s8a, ADR-0165: this asserted that `change_summary("HEAD~1..HEAD")`
    // over the fixture project answered Unsupported from the row -- true of every range while no
    // `Ok` was constructible. The row now answers the BINARY case, so the assertion moves there:
    // over a range carrying a `.uasset`, the refusal is the row's token naming the path, and over
    // an EMPTY registry the same range is a decode failure naming the capability.)*
    let scratch = tempfile::tempdir().expect("scratch");
    let repo = binary_range_repo();
    let tc = repo.adapter(scratch.path());
    match tc.change_summary("HEAD~1..HEAD", StepBound::Undeclared) {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.starts_with(rows[0].token()) && capability.contains("Content/Hero.uasset"),
            "a binary range answers Unsupported from the change_summary row, naming the path: \
             {capability:?}"
        ),
        other => panic!(
            "change_summary over a binary range answers Unsupported from its registry row; got \
             {other:?}"
        ),
    }
    let empty = RustToolchain::new_over_registry(repo.config(scratch.path()), Vec::new());
    decode_naming(
        &empty
            .change_summary("HEAD~1..HEAD", StepBound::Undeclared)
            .expect_err("a binary range over an EMPTY registry is never Ok"),
        "change_summary",
        "change_summary over an EMPTY registry",
    );
}

/// **AC-492.** An unknown token fails naming itself; every shipped token is canonical; the
/// token mapper has no catch-all arm.
#[test]
fn ac492_unknown_capability_token_fails() {
    let err = registry::load(READS_MINDS).expect_err("`reads_minds` is not a capability");
    assert!(
        err.to_string().contains("reads_minds"),
        "the refusal names the unknown token: {err}"
    );
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

/// **AC-328.** The `affected-selection` row is present, dated and probe-less; selection is
/// refused with `Unsupported("affected-selection")` from it; no run produces `basis: affected`;
/// a row with an invented probe is refused, never honoured.
///
/// **Not discharged here, and said so:** the Verified-by's *"a fixture adapter with an expired
/// `verified_on` that must produce `Unsupported`"*. Staleness needs a pin the date is compared
/// against and no record fixes one at 7c (`registry.rs`'s doc); the comparison and the
/// expired-date fixture are the `CapabilityClaim` bridge's at 7d. This test covers the ABSENT
/// half of *"absent or stale"* only.
///
/// *(Amended 2026-09-26, s8c build, P5; record T = ADR-0169 decision 7 rule 1, proposed. This test
/// KEEPS ITS NAME -- Checkpoint 7c's loop names it -- and is RE-POINTED at a controlled probe-less
/// registry: the shipped row now carries `Probe::Recorded`, and its behaviour is proved by the
/// `affected::` tests. Over the probe-less row, selection is still refused from the row, and an
/// `Affected` selection through the port is refused the same way, so no `basis: affected` record
/// is produced. The invented-probe half is dropped, not weakened: rule 1 honours ANY `Recorded`
/// row, and nothing compares a row's provenance or its `verified_on` (record T states both facts;
/// the bridge is OWED). The missing-row half stands.)*
#[test]
fn ac328_affected_selection_unsupported_without_probe() {
    let rows = vec![Row {
        capability: Capability::AffectedSelection,
        verified_on: "2026-09-18".to_owned(),
        probe: Probe::None {
            reason: "AC-328: a controlled probe-less row (the shape this row shipped with to s8c)"
                .to_owned(),
        },
    }];
    assert!(is_date(&rows[0].verified_on));

    let scratch = tempfile::tempdir().expect("scratch");
    let tc = RustToolchain::new_over_registry(
        config_with(
            rk_adapter_toolchain_rust::DEFAULT_PROGRAM.into(),
            GENEROUS,
            &fixture_project(),
            minimal_env(),
            scratch.path(),
        ),
        rows,
    );
    match tc.select_affected(&["src/lib.rs"]) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, "affected-selection"),
        other => panic!(
            "selection is refused with Unsupported(\"affected-selection\") while no probe is recorded; got {other:?}"
        ),
    }

    // No run — unselected or selected — emits a `basis: affected` record.
    for selection in [vec![], vec!["passes_and_is_filterable".to_owned()]] {
        // *(Amended 2026-09-26, s8c: the retired inherent `verify_report` is the port's `test`.)*
        // `Ok` is no longer "green" (record T), so the verdict is read off `failed`.
        let outcome = tc
            .test(&Selection::Filter(selection.clone()), StepBound::Undeclared)
            .expect("the fixture suite runs under this selection");
        assert_eq!(
            outcome.failed, 0,
            "the fixture suite is green under {selection:?}"
        );
        let report = outcome.report;
        assert_eq!(
            report.basis,
            Basis::WholeSuite,
            "no `basis: affected` record while the probe entry is absent (AC-328); selection {selection:?}"
        );
        assert_eq!(
            report.basis_reason,
            Some(BasisReason::ProbeUnverified),
            "the fallback names AC-328's reason (AC-352)"
        );
        report
            .validate_shape()
            .expect("the report's own invariants hold");
    }

    // An `Affected` selection through the port, over the probe-less row, is refused from the row
    // before anything is spawned: no `basis: affected` record (AC-328).
    // *(Added 2026-09-26, s8c build, P5.)*
    match tc.test(
        &Selection::Affected {
            changed: vec!["src/lib.rs".to_owned()],
            recorded: None,
        },
        StepBound::Undeclared,
    ) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, "affected-selection"),
        other => panic!(
            "an Affected selection over a probe-less row is refused from the row (AC-328); got {other:?}"
        ),
    }
    // ~~An invented probe on the row is refused (a decode failure naming the row), never a
    // basis.~~ *(Struck 2026-09-26, s8c build, P5: record T decision 7 rule 1 honours any
    // `Recorded` row, and nothing compares its provenance; the `INVENTED_PROBE` fixture still
    // loads, which is all it now shows.)*
    registry::load(INVENTED_PROBE).expect("well-formed, so it loads");
    // And no row at all is the missing-row failure, not a default.
    let over_empty = RustToolchain::new_over_registry(
        config_with(
            rk_adapter_toolchain_rust::DEFAULT_PROGRAM.into(),
            GENEROUS,
            &fixture_project(),
            minimal_env(),
            scratch.path(),
        ),
        Vec::new(),
    );
    decode_naming(
        &over_empty
            .select_affected(&["src/lib.rs"])
            .expect_err("no row, no answer"),
        "affected-selection",
        "select_affected over an EMPTY registry",
    );
}
