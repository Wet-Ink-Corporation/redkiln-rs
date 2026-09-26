//! **AC-489 / AC-492 / AC-328 (the Node row)** — the dated capability registry behind
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
//! **What the Node entries are**: exactly `change_summary` (S-BIN unrun) and `affected-selection`
//! (no probe exists at any pin — §7c.14 [measured 2026-09-17]; the Node probe, `vitest --changed`
//! / `related`, is OWED as the named spike S-SEL-NODE, §7c.16), each dated and each **explicitly
//! probe-less** (`Probe::None { reason }`).

use std::path::Path;

use rk_ports::toolchain::Toolchain;
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::{Basis, BasisReason};

use rk_adapter_toolchain_node::NodeToolchain;
use rk_adapter_toolchain_node::registry::{self, Capability, Probe, Row};

use crate::support::{
    GENEROUS, adapter, config_with, fixture_project, is_date, minimal_env, run_test,
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
    let rows = registry::rows().expect("the shipped registry loads");
    let tokens: Vec<&str> = rows.iter().map(Row::token).collect();
    assert_eq!(
        tokens,
        vec!["change_summary", "affected-selection"],
        "the Node registry carries exactly the change_summary (D17-9) and affected-selection (AC-328) rows"
    );
    for row in &rows {
        assert!(
            is_date(&row.verified_on),
            "row {} is not dated YYYY-MM-DD: {:?}",
            row.token(),
            row.verified_on
        );
        match &row.probe {
            Probe::None { reason } => assert!(
                !reason.trim().is_empty(),
                "row {} says Probe::None with no reason",
                row.token()
            ),
            Probe::Recorded { source } => panic!(
                "row {} claims a recorded probe ({source:?}) — S-BIN is unrun and no affected-selection probe exists at any pin",
                row.token()
            ),
        }
    }
    assert_eq!(
        registry::unsupported_tokens().expect("loads"),
        vec!["change_summary".to_owned(), "affected-selection".to_owned()]
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

    // ---- the adapter answers FROM the rows it holds.
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    match tc.change_summary("HEAD~1..HEAD", rk_ports::toolchain::StepBound::Undeclared) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, rows[0].token()),
        other => panic!("change_summary answers Unsupported from its registry row; got {other:?}"),
    }
    let empty = NodeToolchain::new_over_registry(
        config_with(
            rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
            GENEROUS,
            &fixture_project(),
            minimal_env(),
            scratch.path(),
        ),
        Vec::new(),
    );
    decode_naming(
        &empty
            .change_summary("HEAD~1..HEAD", rk_ports::toolchain::StepBound::Undeclared)
            .expect_err("no Ok is constructible"),
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
#[test]
fn ac328_affected_selection_unsupported_without_probe() {
    let rows = registry::rows().expect("loads");
    let row = rows
        .iter()
        .find(|r| r.capability == Capability::AffectedSelection)
        .expect("the affected-selection row is present (AC-328: an entry, never an absence)");
    assert!(is_date(&row.verified_on));
    assert!(
        matches!(&row.probe, Probe::None { reason } if reason.contains("AC-328")),
        "the affected-selection row is explicitly probe-less and says why (no probe at any pin): {row:?}"
    );

    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(GENEROUS, scratch.path());
    match tc.select_affected(&["src/lib.rs"]) {
        Err(StepError::Unsupported { capability }) => assert_eq!(capability, "affected-selection"),
        other => panic!(
            "selection is refused with Unsupported(\"affected-selection\") while no probe is recorded; got {other:?}"
        ),
    }

    // No run — unselected or selected — emits a `basis: affected` record.
    for selection in [
        vec![],
        vec![
            "test/suite.test.ts".to_owned(),
            "-t".to_owned(),
            "filterable".to_owned(),
        ],
    ] {
        let report = run_test(&tc, &selection)
            .map(|o| o.report)
            .expect("the fixture suite is green under this selection");
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

    // An invented probe on the row is refused (a decode failure naming the row), never a basis.
    let invented = registry::load(INVENTED_PROBE).expect("well-formed, so it loads");
    let over_invented = NodeToolchain::new_over_registry(
        config_with(
            rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
            GENEROUS,
            &fixture_project(),
            minimal_env(),
            scratch.path(),
        ),
        invented,
    );
    decode_naming(
        &over_invented
            .select_affected(&["src/lib.rs"])
            .expect_err("an invented probe is not a selection basis"),
        "affected-selection",
        "select_affected over a row claiming a recorded probe nobody ran",
    );
    // And no row at all is the missing-row failure, not a default.
    let over_empty = NodeToolchain::new_over_registry(
        config_with(
            rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
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
