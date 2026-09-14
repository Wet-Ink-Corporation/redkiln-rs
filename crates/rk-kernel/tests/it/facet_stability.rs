//! `facet_stability_five_clause` — **AC-071**, the unblocking gate.
//!
//! **Six must-fail fixtures: one declaring `source = "llm"` with no record at all, plus five
//! carrying a record that fails exactly one clause.** The five clause negatives plus the no-record
//! case are the discrimination; **one positive test is not**. A lint that reads identity and skips
//! the panel clause passes a single happy-path test and unblocks the whole provisional apparatus —
//! which is AC-071's own residual-risk sentence, and it is the reason each clause has its own file
//! and its own assertion here.
//!
//! # How each fixture is shown to fail for ITS OWN clause
//!
//! Asserting that a fixture fails proves nothing about *why*. Every case therefore runs twice:
//!
//! 1. the committed fixture must produce **exactly one** violation, and it must be the clause the
//!    file is named for — not "some violation", and not a set containing it;
//! 2. the fixture **repaired in that one respect** — one anchored text edit, asserted to match
//!    exactly once before it is applied (golden constraint 7) — must then **load clean**.
//!
//! Step 2 is what rules out an incidental failure. A fixture that failed because the reader choked
//! on its TOML, or because the rule rejects every `llm` facet unconditionally, fails step 2.

use std::collections::BTreeMap;

use rk_kernel::facet_stability::{StabilityViolation, facet_stability_rule};
use rk_kernel::registry::{Registry, StabilityRecord};

/// The catalog corpus pin AC-071's fifth clause compares against.
///
/// This is `CORPUS_PIN`'s `REDKILN_SHA` — the frozen predecessor corpus F36 measured identity over
/// and F39 scored arm D over — and **it is a constant here rather than a read of `CORPUS_PIN`**.
/// Two reasons, both recorded rather than assumed. The file's `CORPUS_SHA` moves: the single pin
/// move is a named act at the END of Step 7b (§7b.11), and a test that read it would silently
/// re-pin its own expectation. And **which of the two shas AC-071's clause means is part of the
/// amendment that criterion is OWED** (§7b.2, §7b.15): its record has no producer and no lawful
/// path, and `spikes/results/p1/stability.json` names a directory that has never existed on either
/// side. Raised, not settled, and the fixtures are authored against the pin the measurement names.
const CATALOG_CORPUS_PIN: &str = "47faec9f46b8d983c2c75f0794164e68415ae706";

const NO_RECORD: &str = include_str!("../fixtures/registries/facet-stability/no_record.toml");
const IDENTITY_BELOW_BAR: &str =
    include_str!("../fixtures/registries/facet-stability/identity_below_bar.toml");
const N_BELOW_TWENTY: &str =
    include_str!("../fixtures/registries/facet-stability/n_below_twenty.toml");
const DRAWS_BELOW_TEN: &str =
    include_str!("../fixtures/registries/facet-stability/draws_below_ten.toml");
const PANEL_NOT_SCORED: &str =
    include_str!("../fixtures/registries/facet-stability/panel_not_scored.toml");
const CORPUS_NOT_PINNED: &str =
    include_str!("../fixtures/registries/facet-stability/corpus_not_pinned.toml");

/// The `llm` entry's own two lines. **Anchors are taken on the ENTRY, never on the token**: every
/// one of these fixtures also names `source = "llm"` in its header prose, which is what a fixture
/// header is for, and an anchor that matched the prose would rewrite a comment and report it as a
/// repair. `patch` refuses a non-unique anchor for exactly that reason.
const LLM_ENTRY: &str = "values = [\"low\", \"med\", \"high\", \"unknown\"]\nsource = \"llm\"";

/// A record satisfying all five clauses, appended after [`LLM_ENTRY`].
///
/// **Every numeric field sits exactly ON its bar**, and that is the discrimination, not a
/// convenience: a bar is told apart from its neighbour by the value beside it, so `n_items = 20`
/// and `draws_per_item = 10` are what make `< N` distinguishable from `<= N` (their must-fail
/// fixtures sit one short, at 19 and 9). `modal_required_set_identity` was **0.85** until
/// 2026-09-13 — neither it nor the fixture's 0.25 is 0.80, so the identity bar's comparator was the
/// one of the three that nothing pinned, in the clause this module calls the one that still holds
/// the block.
const WHOLE_RECORD: &str = "\n\n[stability.criticality]\nmodal_required_set_identity = 0.80\n\
                            n_items = 20\ndraws_per_item = 10\npanel_verdict = \"scored\"\n\
                            corpus_sha = \"47faec9f46b8d983c2c75f0794164e68415ae706\"";

/// Fixtures are compiled in with `include_str!` and **normalised to LF before any anchor is
/// matched**. `core.autocrlf` is `true` on this repository, so a `.toml` checked out on Windows
/// carries CRLF while one written by a tool may not; a multi-line anchor would then match on one
/// machine and not the next, and `patch` would fail for a line-ending reason that has nothing to do
/// with the clause under test.
fn lf(src: &str) -> String {
    src.replace("\r\n", "\n")
}

/// One anchored edit, asserted before it writes (golden constraint 7: *"two silent `str.replace`
/// no-ops were read as behaviour changes"*).
///
/// A repair whose anchor has drifted must fail loudly here, not silently return the fixture
/// unchanged — an unchanged fixture still fails the rule, and the discrimination would then be
/// asserting the negative twice and calling the second one a positive.
fn patch(src: &str, anchor: &str, replacement: &str) -> String {
    let hits = src.matches(anchor).count();
    assert_eq!(
        hits, 1,
        "the repair anchor {anchor:?} must occur exactly once in the fixture, and occurs {hits}"
    );
    src.replace(anchor, replacement)
}

/// Load a fixture and run the rule over it.
fn verdict(src: &str) -> Result<(), Vec<StabilityViolation>> {
    let registry = Registry::from_toml(src).expect("the fixture registry loads");
    let records: BTreeMap<String, StabilityRecord> =
        Registry::stability_records_from_str(src).expect("the fixture's stability records load");
    facet_stability_rule(&registry, &records, CATALOG_CORPUS_PIN)
}

/// The one violation a must-fail fixture produces, with its count asserted.
fn sole_violation(name: &str, src: &str) -> StabilityViolation {
    let errs = verdict(src).expect_err("the fixture must FAIL load — this is a must-fail fixture");
    assert_eq!(
        errs.len(),
        1,
        "{name}: expected exactly one clause to fail, got {errs:?} — a fixture that trips several \
         clauses cannot discriminate between them"
    );
    errs.into_iter().next().expect("one violation")
}

#[test]
fn facet_stability_five_clause() {
    // ---- the six must-fail fixtures, one clause each, each repaired and re-run -----------------
    let with_record = format!("{LLM_ENTRY}{WHOLE_RECORD}");
    let cases: [(&str, &str, &str, &str, &str); 6] = [
        // name, source, the clause tag it must produce, repair anchor, repair replacement
        ("no_record", NO_RECORD, "no-record", LLM_ENTRY, &with_record),
        (
            "identity_below_bar",
            IDENTITY_BELOW_BAR,
            "identity",
            "modal_required_set_identity = 0.25",
            // EXACTLY the bar, for the reason WHOLE_RECORD states: 0.85 repaired the clause and
            // discriminated nothing about `<` versus `<=`.
            "modal_required_set_identity = 0.80",
        ),
        (
            "n_below_twenty",
            N_BELOW_TWENTY,
            "n",
            "n_items = 19",
            "n_items = 20",
        ),
        (
            "draws_below_ten",
            DRAWS_BELOW_TEN,
            "draws",
            "draws_per_item = 9",
            "draws_per_item = 10",
        ),
        (
            "panel_not_scored",
            PANEL_NOT_SCORED,
            "panel_verdict",
            "panel_verdict = \"unrun\"",
            "panel_verdict = \"scored\"",
        ),
        (
            "corpus_not_pinned",
            CORPUS_NOT_PINNED,
            "corpus_sha",
            "panel_verdict = \"scored\"",
            "panel_verdict = \"scored\"\n\
             corpus_sha = \"47faec9f46b8d983c2c75f0794164e68415ae706\"",
        ),
    ];

    for (name, src, clause, anchor, replacement) in cases {
        let src = &lf(src);
        let violation = sole_violation(name, src);
        assert_eq!(
            violation.clause(),
            clause,
            "{name}: the fixture must fail on ITS OWN clause, and failed on {violation}"
        );
        assert_eq!(violation.facet(), "criticality", "{name}");
        // The message names the facet, so the lint's output is actionable rather than a bare
        // "load failed".
        let text = violation.to_string();
        assert!(
            text.contains("criticality"),
            "{name}: the message names the facet: {text}"
        );

        // DISCRIMINATION: valid in that ONE respect, and the registry then loads.
        let repaired = patch(src, anchor, replacement);
        assert_eq!(
            verdict(&repaired),
            Ok(()),
            "{name}: repairing only the {clause} clause must make the registry load — otherwise \
             the must-fail case was failing for some other reason and pins nothing"
        );
    }

    // ---- the fifth clause's OTHER half: a pin that is present and wrong ------------------------
    // The committed fixture omits `corpus_sha` entirely. A record that carries one which is not the
    // catalog corpus pin is the same clause and a different registry, and it is patched rather than
    // committed so the six files stay the six the register enumerates.
    let mismatched = patch(
        &lf(CORPUS_NOT_PINNED),
        "panel_verdict = \"scored\"",
        "panel_verdict = \"scored\"\ncorpus_sha = \"0000000000000000000000000000000000000000\"",
    );
    let violation = sole_violation("corpus_not_pinned (mismatched sha)", &mismatched);
    assert_eq!(violation.clause(), "corpus_sha");
    assert!(
        matches!(
            &violation,
            StabilityViolation::CorpusNotPinned { found, expected, .. }
                if found.as_deref() == Some("0000000000000000000000000000000000000000")
                    && expected == CATALOG_CORPUS_PIN
        ),
        "the violation carries both shas so a reader can see WHICH pin disagreed: {violation}"
    );

    // ---- A PRESENT RECORD WITH AN ABSENT FIELD, one case per clause ----------------------------
    // THE FAIL-OPEN AC-071'S RESIDUAL-RISK SENTENCE NAMES, and nothing pinned it until 2026-09-13.
    // `no_record.toml` carries no record at all — a different clause — and each of the five
    // committed clause fixtures carries its field with a FAILING value, so every one of them is
    // green under a rule that treats an ABSENT field as satisfied. The source says the opposite in
    // terms ("An absent field and a failing field are the same verdict … both must be reported as
    // that clause"), and turning any one of the three `is_none_or` calls into `is_some_and` left
    // the whole case above passing. The record here is otherwise complete, so the clause named is
    // the clause the deletion caused and no other.
    for (field, clause, line) in [
        (
            "modal_required_set_identity",
            "identity",
            "modal_required_set_identity = 0.80\n",
        ),
        ("n_items", "n", "n_items = 20\n"),
        ("draws_per_item", "draws", "draws_per_item = 10\n"),
        (
            "panel_verdict",
            "panel_verdict",
            "panel_verdict = \"scored\"\n",
        ),
        (
            "corpus_sha",
            "corpus_sha",
            "corpus_sha = \"47faec9f46b8d983c2c75f0794164e68415ae706\"",
        ),
    ] {
        let complete = format!("{LLM_ENTRY}{WHOLE_RECORD}");
        let gutted = patch(&complete, line, "");
        let src = patch(&lf(NO_RECORD), LLM_ENTRY, &gutted);
        let violation = sole_violation(&format!("{field} omitted"), &src);
        assert_eq!(
            violation.clause(),
            clause,
            "a record that OMITS {field} must fail the same clause a record that fails it does — \
             an absent field is not a satisfied one, and this is the fail-open shape AC-071's \
             residual-risk sentence names. Got {violation}"
        );
        // And the same record with the field present and passing loads, so the case above failed
        // for the omission and not for anything the surgery did to the rest of the record.
        assert_eq!(
            verdict(&patch(&lf(NO_RECORD), LLM_ENTRY, &complete)),
            Ok(()),
            "{field}: the unmutilated record loads"
        );
    }

    // ---- the gate's SUBJECT is `source = "llm"` and nothing else -------------------------------
    // Two negative controls on the scope. After Step 4a the shipped registry has ZERO llm facets —
    // ADR-0090 re-sourced all seven to `human` — so a rule that gated on anything wider would fail
    // closed on the live registry and on every fixture in the sibling suite.
    let human_no_record = patch(
        &lf(NO_RECORD),
        LLM_ENTRY,
        &LLM_ENTRY.replace("llm", "human"),
    );
    assert_eq!(
        verdict(&human_no_record),
        Ok(()),
        "a `human` facet with no stability record is not this gate's business (AC-071's GIVEN)"
    );
    let derived_no_record = patch(
        &lf(NO_RECORD),
        LLM_ENTRY,
        &LLM_ENTRY.replace("llm", "derived"),
    );
    assert_eq!(
        verdict(&derived_no_record),
        Ok(()),
        "a `derived` facet is AC-520's subject, and AC-520 is a SECOND RULE — not a sixth clause \
         here. A stability rule that also failed on `derived` would be the collapse AC-071's own \
         residual-risk sentence names"
    );

    // ---- and a record satisfying all five loads ------------------------------------------------
    let all_five = patch(
        &lf(NO_RECORD),
        LLM_ENTRY,
        &format!("{LLM_ENTRY}{WHOLE_RECORD}"),
    );
    assert_eq!(
        verdict(&all_five),
        Ok(()),
        "five clauses satisfied is the only way past this gate"
    );
}
