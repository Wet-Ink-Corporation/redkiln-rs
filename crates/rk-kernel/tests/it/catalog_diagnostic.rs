//! **The catalog loader's two Step 7d Slice 4 edits** — `catalog::load_diagnostic` (RT-54,
//! `s7d` §7d.1 edit (4), §7d.8) and `Catalog::from_sources`' AC-409 refusal (§7d.8's *"the seventh
//! kernel edit"*).
//!
//! The two are one cycle because they are one mechanism read from two sides. The **strict** loader
//! refuses a `Gate` step carrying `outputs` so that `run` never loads the graph AC-409 says is
//! rejected; the **diagnostic** loader reports that same refusal typed, beside every other defect
//! in the pack, so that `doctor` can report a pack with two defects as two findings.
//!
//! # Why the diagnostic mode is not a loop over the strict loader
//!
//! `s7d` §7d.1 edit (4) names the cheap alternative and rejects it in the same breath — *"the
//! alternative — `doctor` calling the strict loader per file and collecting first errors — is named
//! and rejected because a pack with two defects would surface one"*. Per-file first errors would
//! surface **both** defects were they in two files, so that sentence is only true of two defects in
//! **one activity file**, which is what §7d.14's red describes: *"a `Gate` step with `outputs`
//! **and** a second malformed step"*. The accumulation is therefore INSIDE the activity walk, and
//! [`TWO_DEFECTS`] below puts both defects in one file for exactly that reason.

use rk_kernel::catalog::{
    Catalog, CatalogError, DiagnosticKind, load_diagnostic, load_diagnostic_dir,
};

/// One activity file carrying **two `Shape` defects**, the gate-with-`outputs` step FIRST.
///
/// The order is load-bearing rather than cosmetic: the strict loader stops at the first defect in
/// walk order, so putting the gate first makes the strict error the AC-409 refusal — which is what
/// lets `catalog_load_diagnostic_returns_every_load_error_typed` compare the two loaders' first
/// results and have the comparison be about the NEW refusal rather than about an old one.
const TWO_DEFECTS: &str = concat!(
    "[activity]\n",
    "id = \"review-gate\"\n",
    "stage = \"review\"\n",
    "method_rank = 10\n",
    "\n",
    "[[step]]\n",
    "id = \"ask\"\n",
    "kind = \"gate\"\n",
    "outputs = \"value:Verdict\"\n",
    "\n",
    "[[step]]\n",
    "id = \"tally\"\n",
    "kind = \"review\"\n",
);

/// [`TWO_DEFECTS`] with both defects repaired — the gate declares no `outputs` and the second step
/// takes a kind inside the schema's five. **The positive control for the whole module**: without
/// it, a `load_diagnostic` that reported two defects over every pack would satisfy every assertion
/// below.
const CLEAN: &str = concat!(
    "[activity]\n",
    "id = \"review-gate\"\n",
    "stage = \"review\"\n",
    "method_rank = 10\n",
    "\n",
    "[[step]]\n",
    "id = \"ask\"\n",
    "kind = \"gate\"\n",
    "\n",
    "[[step]]\n",
    "id = \"tally\"\n",
    "kind = \"check\"\n",
);

/// A `Gate` step carrying `outputs`, and **nothing else wrong** — AC-409's own fixture shape.
const GATE_WITH_OUTPUTS: &str = concat!(
    "[activity]\n",
    "id = \"approve-change\"\n",
    "stage = \"review\"\n",
    "method_rank = 20\n",
    "\n",
    "[[step]]\n",
    "id = \"approve\"\n",
    "kind = \"gate\"\n",
    "outputs = \"value:Verdict\"\n",
);

/// [`GATE_WITH_OUTPUTS`] with the gate's `outputs` line removed, and nothing else changed.
const GATE_WITHOUT_OUTPUTS: &str = concat!(
    "[activity]\n",
    "id = \"approve-change\"\n",
    "stage = \"review\"\n",
    "method_rank = 20\n",
    "\n",
    "[[step]]\n",
    "id = \"approve\"\n",
    "kind = \"gate\"\n",
);

/// A `Tool` step carrying `outputs` — lawful, and the schema's ordinary case.
const TOOL_WITH_OUTPUTS: &str = concat!(
    "[activity]\n",
    "id = \"run-tests\"\n",
    "stage = \"implement\"\n",
    "method_rank = 30\n",
    "\n",
    "[[step]]\n",
    "id = \"test\"\n",
    "kind = \"tool\"\n",
    "run = \"toolchain:test\"\n",
    "outputs = \"ToolReport\"\n",
);

/// **RT-54 — the loader's diagnostic mode returns every load error typed, not the first.**
///
/// §7d.14's red, verbatim: *"Red by `load_diagnostic` stopping at the first error over a pack
/// carrying **two `Shape` defects** — a `Gate` step with `outputs` **and** a second malformed step
/// (the loader's own vocabulary is `Io` / `Toml` / `Shape`; it reads no `timeout_secs` and
/// validates no facet, §7d.8), or by the strict loader's first-error result differing from
/// `load_diagnostic`'s first typed error."*
///
/// **The instrument is the COUNT and the AGREEMENT, in that order.** The count is what a
/// first-error loader fails; the agreement is what a second, divergent walk of the same file would
/// fail. The clean-pack control below is what stops a loader that invents defects from passing
/// both.
#[test]
fn catalog_load_diagnostic_returns_every_load_error_typed() {
    let pack = [("review-gate.toml", TWO_DEFECTS)];

    // ---- what the STRICT loader says. One error, and it is the first in walk order.
    let strict = Catalog::from_sources(&pack)
        .expect_err("the pack carries two shape defects — the strict loader refuses it");

    // ---- what the DIAGNOSTIC loader says.
    let load = load_diagnostic(&pack);

    assert!(
        load.diagnostics.len() >= 2,
        "`load_diagnostic` returned {} diagnostic(s) over a pack carrying TWO shape defects in ONE \
         activity file — a gate step declaring `outputs` and a step whose kind is outside the \
         schema's five. This is the first-error behaviour `s7d` §7d.1 edit (4) names and rejects: \
         *\"a pack with two defects would surface one\"*. What came back:\n{}",
        load.diagnostics.len(),
        render(&load.diagnostics)
    );

    // A loader that pushed the SAME error twice would satisfy the count and report one defect.
    assert!(
        load.diagnostics[0] != load.diagnostics[1],
        "the first two diagnostics are the same value, so the pack's two distinct defects were not \
         both observed:\n{}",
        render(&load.diagnostics)
    );

    assert_eq!(
        load.diagnostics[0].error, strict,
        "`load_diagnostic`'s FIRST typed error differs from the strict loader's only error. They \
         walk the same file and must agree on what comes first, or `doctor` and `run` disagree \
         about which defect a pack has.\n  strict:     {strict}\n  diagnostic: {}",
        load.diagnostics[0].error
    );

    // ---- the two defects are typed apart, which is what `rk-cli`'s RK-D-GATE-OUTPUTS row maps.
    let kinds: Vec<DiagnosticKind> = load.diagnostics.iter().map(|d| d.kind).collect();
    assert!(
        kinds.contains(&DiagnosticKind::GateOutputs),
        "no diagnostic is typed `GateOutputs`, so the gate-with-`outputs` refusal is \
         indistinguishable from every other `Shape` and RK-D-GATE-OUTPUTS would have to recognise \
         it by matching text inside the message. Kinds: {kinds:?}"
    );
    assert!(
        kinds.contains(&DiagnosticKind::Shape),
        "no diagnostic is typed plain `Shape`, so the second malformed step was either not \
         observed or was mistyped. Kinds: {kinds:?}"
    );
    for d in &load.diagnostics {
        assert!(
            matches!(d.error, CatalogError::Shape { .. }),
            "both defects are SHAPE defects — `s7d` §7d.8 puts the gate refusal *\"among the \
             `Shape`s\"* and the loader's variant set stays three. Got: {:?}",
            d.error
        );
    }

    // ---- THE POSITIVE CONTROL. Over a pack with no defect the loader reports none and loads the
    // ---- activity. Without this every assertion above is satisfied by a loader that always
    // ---- returns two errors and never loads anything.
    let clean = load_diagnostic(&[("review-gate.toml", CLEAN)]);
    assert!(
        clean.diagnostics.is_empty(),
        "the repaired pack yielded diagnostics, so the ones above are not evidence that the loader \
         READ anything:\n{}",
        render(&clean.diagnostics)
    );
    assert_eq!(
        clean.catalog.len(),
        1,
        "the repaired pack loaded no activity — the diagnostic loader must still LOAD what is \
         lawful, because `doctor` has rules that read the loaded activities"
    );
    assert!(
        Catalog::from_sources(&[("review-gate.toml", CLEAN)]).is_ok(),
        "the strict loader refuses the repaired pack, so the defects asserted above are not the \
         two this test names"
    );

    // ---- THE DIRECTORY FORM, over the same bytes. It is the form the composition root uses, and
    // ---- it ships asserted rather than merely compiled.
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("review-gate.toml"), TWO_DEFECTS).expect("the pack is written");
    let from_dir = load_diagnostic_dir(dir.path());
    assert_eq!(
        from_dir.diagnostics, load.diagnostics,
        "`load_diagnostic_dir` and `load_diagnostic` disagree over the same bytes"
    );
}

/// **AC-409 as amended (C-01 RA-M-05) — a `Gate` step carrying `outputs` is rejected AT LOAD.**
///
/// The register's own words: *"a loader test asserting that a `Gate` step carrying `outputs` is
/// rejected at load"*, and the amendment note says why this is the instrument — Step 7b's loader
/// has one `Step` struct for every kind, so `Step { kind: Gate, output_type: Some(..) }` is
/// representable and the compile-time half the criterion first named could not be written.
///
/// **The subject is `Catalog::from_sources`, the STRICT loader, and that is the point.** §7d.14
/// names the weaker test that would pass without it: *"a weaker red asserts `load_diagnostic`'s
/// refusal alone and leaves `run` loading the graph"*. `run` loads through the strict path.
///
/// **Two positive controls in the same test**, both from §7d.14: the same pack with the gate's
/// `outputs` removed loads, and a `Tool` step carrying `outputs` loads — so a loader that refuses
/// every `outputs`, or every `Gate` step, is red here rather than green.
#[test]
fn ac409_gate_step_with_outputs_rejected_at_load() {
    // ---- the refusal.
    let refused = Catalog::from_sources(&[("approve-change.toml", GATE_WITH_OUTPUTS)]).expect_err(
        "AC-409: a gate answer populates the gate step's `answer` and can bind nothing downstream, \
         so a `Gate` step carrying `outputs` is a configuration error at LOAD — the strict loader \
         `run` uses must refuse it, or `run` loads the graph AC-409 says is rejected",
    );
    let CatalogError::Shape { source, detail } = &refused else {
        panic!(
            "the refusal is a `Shape` defect (`s7d` §7d.8: *\"among the `Shape`s\"*): {refused:?}"
        )
    };
    assert_eq!(source, "approve-change.toml");
    assert!(
        detail.contains("approve"),
        "the refusal names neither the activity nor the step, so an operator reading it cannot find \
         the gate: {detail}"
    );

    // ---- POSITIVE CONTROL 1: the same pack, `outputs` removed, LOADS.
    // A loader that refused every `Gate` step would be red here.
    let without = Catalog::from_sources(&[("approve-change.toml", GATE_WITHOUT_OUTPUTS)]).expect(
        "a gate step declaring no `outputs` is lawful — AC-409 refuses the binding, not the gate",
    );
    assert_eq!(without.len(), 1);

    // ---- POSITIVE CONTROL 2: a `Tool` step carrying `outputs` LOADS, and the value is still READ.
    // A loader that refused every `outputs`, or that gutted the parse to make this test pass,
    // would be red on one of these two assertions.
    let tool = Catalog::from_sources(&[("run-tests.toml", TOOL_WITH_OUTPUTS)])
        .expect("`outputs` on a non-gate step is the schema's ordinary case");
    let step = &tool.activities()[0].steps[0];
    assert_eq!(
        step.output_type.as_deref(),
        Some("ToolReport"),
        "the loader stopped READING `outputs` instead of refusing it on gate steps only — every \
         `Step::mints` reading of `output_type` goes with it"
    );

    // ---- and the diagnostic mode reports the same refusal, typed.
    let load = load_diagnostic(&[("approve-change.toml", GATE_WITH_OUTPUTS)]);
    assert_eq!(
        load.diagnostics.len(),
        1,
        "the diagnostic loader found {} defects in a pack whose ONE defect is the gate's \
         `outputs`:\n{}",
        load.diagnostics.len(),
        render(&load.diagnostics)
    );
    assert_eq!(load.diagnostics[0].kind, DiagnosticKind::GateOutputs);
    assert_eq!(
        load.diagnostics[0].error, refused,
        "the two loaders refuse the same pack with different errors"
    );
}

/// The diagnostics, one per line, for an assertion message. A failure that prints what it saw is
/// the difference between a red that can be read and one that has to be reproduced.
fn render(diagnostics: &[rk_kernel::catalog::CatalogDiagnostic]) -> String {
    if diagnostics.is_empty() {
        return "    (none)".to_owned();
    }
    diagnostics
        .iter()
        .map(|d| format!("    [{:?}] {}", d.kind, d.error))
        .collect::<Vec<_>>()
        .join("\n")
}
/// A gate step whose `question` carries a `:` — the shape a selection-level gate kind has.
const GATE_QUESTION_WITH_COLON: &str = concat!(
    "[activity]\n",
    "id = \"triage-repro\"\n",
    "stage = \"triage\"\n",
    "method_rank = 10\n",
    "\n",
    "[[step]]\n",
    "id = \"confirm\"\n",
    "kind = \"gate\"\n",
    "question = \"unknown_facet:has_repro\"\n",
);

/// The same activity with an ordinary question — the positive control.
const GATE_QUESTION_PLAIN: &str = concat!(
    "[activity]\n",
    "id = \"triage-repro\"\n",
    "stage = \"triage\"\n",
    "method_rank = 10\n",
    "\n",
    "[[step]]\n",
    "id = \"confirm\"\n",
    "kind = \"gate\"\n",
    "question = \"repro.confirm\"\n",
);

/// **ADR-0146 clause 4: a `question` may not carry a colon, refused at LOAD.**
///
/// Since ADR-0145 a catalog `question` IS a gate kind, and `RunState::gates_by_kind` is keyed on the
/// bare kind string. A selection-level gate's kind is the synthesised `"{reason}:{axis}"`
/// (`fsm::next_action` clause 2), so `question = "unknown_facet:has_repro"` lands in the SAME reach
/// sequence as that abstention gate.
///
/// 🛑 **Measured on a fixture before this refusal existed**: the step gate minted **reach 2 of the
/// selection gate's sequence**, `latest_gate` then pointed at the step gate's token, and — because
/// `fsm`'s clause 2 opens only when `reaches == 0` — the abstention gate of that axis could no
/// longer be opened AT ALL. A catalog edit that silently suppresses an abstention, unsatisfiable or
/// appetite gate is a far larger hole than the one ADR-0145 set out to close, and it is a hole this
/// record's own change OPENED: before it, `question` was dropped at load and could collide with
/// nothing.
///
/// Zero of the shipped pack's 45 distinct questions contains a colon, so this refuses nothing that
/// exists — it is what keeps that true.
#[test]
fn adr0146_gate_question_may_not_carry_a_colon() {
    let refused = Catalog::from_sources(&[("triage-repro.toml", GATE_QUESTION_WITH_COLON)])
        .expect_err(
            "ADR-0146 clause 4: a `question` carrying a `:` can join a selection gate's reach \
             sequence and suppress it. The STRICT loader must refuse it — `run` loads through this \
             path, so a diagnostic-only refusal would leave `run` loading the collision",
        );
    let CatalogError::Shape { source, detail } = &refused else {
        panic!("the refusal is a `Shape` defect, as AC-409's is: {refused:?}")
    };
    assert_eq!(source, "triage-repro.toml");
    for expected in ["confirm", "unknown_facet:has_repro"] {
        assert!(
            detail.contains(expected),
            "the refusal does not name {expected:?}, so an operator cannot find the step: {detail}"
        );
    }

    // ---- POSITIVE CONTROL: an ordinary question LOADS, and is still READ.
    // A loader that refused every `question`, or that stopped parsing it to make this pass, is red
    // on one of these two assertions.
    let ok = Catalog::from_sources(&[("triage-repro.toml", GATE_QUESTION_PLAIN)])
        .expect("a colon-free `question` is the schema's ordinary case");
    assert_eq!(
        ok.activities()[0].steps[0].question.as_deref(),
        Some("repro.confirm"),
        "the loader stopped READING `question` instead of refusing only the colliding shape — the \
         gate arm derives its token from this value, so dropping it un-does ADR-0145 entirely"
    );
}
