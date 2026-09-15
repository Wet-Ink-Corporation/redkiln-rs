//! `ac527_empty_set_terminal_condition` — **AC-527**, the quality block's terminal condition.
//!
//! **Slice 2 debt, and the absence note it retires.** `evaluator.rs`'s `quantifier_empty_and_null`
//! carried the forward reference — *"the lint that acts on it **IS NOT BUILT** —
//! `ac527_empty_set_terminal_condition` selects zero tests on this crate"* — and this file is the
//! rule arriving. The premise it rests on is pinned there and is not re-derived here: `none` and
//! `all` over `null` or an empty list are **true** (ADR-0103 decision 4), which is exactly why a
//! block can terminate on a reviewer that found nothing.
//!
//! 🛑 **THE RULE IS ON THE FORM, NOT THE STRING**, and the fixture set is built to prove it.
//! §7b.13:1047-1051: *"The `all`-over-empty case is the discrimination — a lint matching the
//! `findings.none(…)` string alone passes it, and the rule is on the **form**, not the string."*
//! So `terminal_condition_is_findings_all.toml` shares **no text** with the forbidden predicate —
//! this file asserts that, by grep, over the fixture's own bytes — and still fails for the same
//! reason. What the rule does is evaluate the candidate predicate over a context in which
//! `findings` is empty and refuse it if the answer is `true`.
//!
//! # WHAT IS MEASURED, AND WHAT IS ARMED
//!
//! | AC-527 clause | pinned here | by |
//! | --- | --- | --- |
//! | the verbatim `findings.none(evidence in [Reproduced, MutationSurvived])` fails | yes | fixture 1 |
//! | *"or any predicate whose truth over an empty findings set is `true`"* | yes | fixture 2, and the six-cell witness table |
//! | *"lint fails **naming the activity and the predicate**"* | yes | both are asserted out of the violation and out of its `Display` |
//! | a positive terminating on a verify result loads | yes | fixture 3, plus the two patched controls |
//! | *"a quality block terminates on a positive evidence record … **a human gate answer**"* | **NO — ARMED** | see below |
//!
//! 🛑 **THE `rk-catalog` SURFACE IS NOT HERE.** AC-527's area line is `rk-catalog` lint, and there
//! is no `rk-catalog` crate in this workspace — the CLI is **Step 7d**'s and the shipped-catalog
//! repairs are **Step 7e**'s. What is discharged here is the **rule**; that it is reachable from a
//! `rk-catalog lint` invocation is not, and is not claimed. This is the same split
//! `facet_registry.rs` and `memo.rs` record for AC-072 and AC-018.
//!
//! 🛑 **THE HUMAN-GATE TERMINAL CONDITION IS NOT EXERCISED, AND THAT IS STATED RATHER THAN
//! PAPERED OVER.** AC-527's THEN lists three lawful terminations and fixture 3 is the first of them
//! (a verify result). The third — *"a human gate answer"* — would be `answer == 'approve'`, which
//! `spikes/p9/catalog/blameless-postmortem.toml`:66 carries live; it is `null` over an empty
//! findings set exactly as fixture 3 is, so it discriminates nothing this file does not already
//! pin, and a fourth fixture asserting the same `null` would be decoration. §7b.12 names **three**
//! fixtures and there are three.
//!
//! 🛑 **AC-527's THEN AND ADR-0080's FORBIDS CLAUSE ARE NOT THE SAME SENTENCE, AND THIS STAGE
//! IMPLEMENTS THE CRITERION'S.** The criterion says *"any predicate whose **truth over an empty
//! findings set is `true`**"*; ADR-0080 decision 2 says *"Lint rejects a policy predicate whose
//! **satisfaction requires** an empty finding set"*, which §7b.13 restates as *"satisfiable only by
//! an empty finding set"*. The first is an evaluation; the second is a **satisfiability** question
//! over every other operand in the predicate. They part company on a conjunction such as
//! `check.failed == 0 and findings.none(severity in ['blocking'])`, which has **no truth value**
//! over an empty findings set (nothing binds `check.failed` at lint time, and this rule invents no
//! binding for it) and which is nonetheless *satisfied* by an empty finding set whenever the other
//! conjunct holds. [`rk_kernel::quality_block`] implements the criterion's sentence and **counts**
//! the predicates the difference hides — **83** of the shipped catalog's 94, asserted below so it is
//! a number and not a worry. **An erratum reconciling AC-527's THEN with ADR-0080 decision 2 is
//! OWED and is not written here** (`00-preamble.md`:55).
//!
//! 🛑 **AND THE REACH, MEASURED: not one shipped terminal condition is decided LAWFUL by this
//! rule.** 11 of 94 are refused and 83 have no truth value; `decided` is **0**. That is asserted
//! below, because a future green over a repaired catalog would otherwise read as *"these ninety-four
//! predicates were checked and passed"*, and it would not mean that.

use std::collections::BTreeSet;

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::predicate::{Context, EvalError, Tri, Value, evaluate};
use rk_kernel::quality_block::{
    FindingsWitness, LIVE_RIGOR_LEVELS, TerminalConditionViolation, no_findings_witness,
    quality_block_terminal_condition_rule, walk,
};

use crate::support::ok_parse;

/// Fixture 1 — the verbatim ADR-0080 predicate. Must FAIL.
const NONE_OVER_EMPTY: &str =
    include_str!("../fixtures/activities/terminal_condition_is_findings_none.toml");
/// Fixture 2 — THE DISCRIMINATOR. `findings.all(…)` over an empty set. Must also FAIL.
const ALL_OVER_EMPTY: &str =
    include_str!("../fixtures/activities/terminal_condition_is_findings_all.toml");
/// The FAIL-CLOSED case, added 2026-09-14 after a mutation survived. Must be REFUSED, not skipped.
const DOES_NOT_PARSE: &str =
    include_str!("../fixtures/activities/terminal_condition_does_not_parse.toml");
/// Fixture 3 — the positive. Must LOAD.
const VERIFY_RESULT: &str =
    include_str!("../fixtures/activities/terminal_condition_names_a_verify_result.toml");

/// The shipped catalog, read at the workspace root as `memo.rs` and `rigor.rs` read it.
const SHIPPED_CATALOG: &str = "spikes/p9/catalog";

/// The shipped catalog's activity count [measured 2026-09-14], asserted as a premise so that a
/// subset cannot masquerade as the artefact.
const SHIPPED_ACTIVITIES: usize = 102;

/// 🛑 **The shipped catalog is RED under this rule by design, and the debt is pinned by number.**
/// §7b.12:972-976 forbids editing `spikes/p9/` to make a green, so the repairs are **Step 7e**'s
/// and this figure is the notification when they land. Measured 2026-09-14 by this rule itself.
const SHIPPED_VIOLATIONS: usize = 11;

/// Predicates the rule PARSED but could not decide — `null` or an evaluation error over the
/// empty-findings context. This is the size of the gap between AC-527's THEN and ADR-0080's
/// forbids clause on the live artefacts, and it is asserted rather than described.
const SHIPPED_INDETERMINATE: usize = 83;

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// One anchored edit, asserted before it writes (golden constraint 7).
fn patch(src: &str, anchor: &str, replacement: &str) -> String {
    let hits = src.matches(anchor).count();
    assert_eq!(
        hits, 1,
        "the repair anchor {anchor:?} must occur exactly once in the fixture, and occurs {hits}"
    );
    src.replace(anchor, replacement)
}

fn one(name: &str, src: &str) -> Catalog {
    Catalog::from_sources(&[(name, src)]).unwrap_or_else(|e| panic!("{name} loads: {e}"))
}

/// The `expr` of the single `check` step, which every fixture here carries exactly one of.
fn terminal_condition(cat: &Catalog) -> String {
    let checks: Vec<&str> = cat
        .activities()
        .iter()
        .flat_map(|a| &a.steps)
        .filter(|s| s.kind == StepKind::Check)
        .filter_map(|s| s.expr.as_deref())
        .collect();
    assert_eq!(
        checks.len(),
        1,
        "premise: each AC-527 fixture carries exactly one policy predicate, and this one carries \
         {checks:?} — a rule over a catalog with no predicate in it passes"
    );
    checks[0].to_owned()
}

/// **AC-527.** *"GIVEN an activity whose quality block's terminal condition is
/// `findings.none(evidence in [Reproduced, MutationSurvived])`, or any predicate whose truth over an
/// **empty** findings set is `true` / WHEN lint runs / THEN lint fails naming the activity and the
/// predicate."*
///
/// **Evidence, cited because the rule is not self-evident from the grammar.** ADR-0080 decision 2
/// forbids the predicate outright. F29 is why the empty set is not evidence in the other direction
/// either: over 60 draws a **reproduced** regression was approved **3 of 10 times** (class mean
/// 0.70 rejected), so "no finding survived" and "the artefact is good" are different propositions
/// and neither implies the other.
#[test]
fn ac527_empty_set_terminal_condition() {
    // ---- premise 0: the ADR-0103 clause the whole rule rests on, restated where it bites --------
    // `quantifier_empty_and_null` pins this over `findings`; here it is asserted over the two
    // predicates the fixtures actually carry, because a rule built on a mis-remembered clause is
    // the one failure the fixtures cannot show.
    let empty = Context::new().with("findings", Value::List(Vec::new()));
    for src in [
        "findings.none(evidence in ['Reproduced', 'MutationSurvived'])",
        "findings.all(severity != 'blocking')",
    ] {
        let parsed = ok_parse(src);
        assert_eq!(
            evaluate(&parsed.expr, &empty).expect("evaluates").value,
            Tri::True,
            "premise: {src:?} is TRUE over an empty findings set — that is the defect, and if this \
             ever stops holding the rule below has nothing to refuse"
        );
    }

    // ---- fixture 1: the verbatim predicate must FAIL --------------------------------------------
    let none_cat = one("terminal_condition_is_findings_none.toml", NONE_OVER_EMPTY);
    let none_expr = terminal_condition(&none_cat);
    assert_eq!(
        none_expr, "findings.none(evidence in ['Reproduced', 'MutationSurvived'])",
        "premise: fixture 1 carries ADR-0080 decision 2's predicate VERBATIM. Paraphrasing it \
         would make the criterion's own named case untested"
    );
    let v1 = quality_block_terminal_condition_rule(&none_cat)
        .expect_err("the verbatim forbidden predicate must FAIL lint");
    assert_eq!(v1.len(), 1, "one violation, not a cascade: {v1:?}");
    // *"THEN lint fails **naming the activity and the predicate**"* — both halves, out of the
    // violation AND out of its rendered message, because a criterion that says "naming" is not
    // satisfied by a struct field nothing prints.
    let TerminalConditionViolation::SatisfiedByNoFindings {
        activity,
        step,
        predicate,
        witness,
    } = &v1[0]
    else {
        panic!("expected SatisfiedByNoFindings, got {:?}", v1[0]);
    };
    assert_eq!(activity, "ac527-none-over-empty");
    assert_eq!(step, "policy");
    assert_eq!(predicate, &none_expr);
    assert_eq!(witness.findings, FindingsWitness::Empty);
    let rendered = v1[0].to_string();
    assert!(
        rendered.contains("ac527-none-over-empty") && rendered.contains(&none_expr),
        "the message names the activity and the predicate, and reads: {rendered}"
    );

    // ---- fixture 2: THE DISCRIMINATOR ------------------------------------------------------------
    // Asserted over the fixture's own bytes first, because a lint that matched the forbidden string
    // would pass fixtures 1 and 3 and fail the rule. The spelling is ASSEMBLED rather than written,
    // so that the check cannot be defeated by the very literal it is looking for occurring in this
    // file — which is not hypothetical: it occurs four lines above, in the criterion's own text.
    let forbidden_spelling = format!("findings.{}(", "none");
    assert!(
        !ALL_OVER_EMPTY.contains(&forbidden_spelling),
        "THE DISCRIMINATION: fixture 2 must share no text with the forbidden predicate — not in \
         its predicate and not in its comments — or it cannot tell a form rule from a denylist of \
         spellings"
    );
    assert!(
        NONE_OVER_EMPTY.contains(&forbidden_spelling),
        "and the control on that check: fixture 1 DOES carry it, so the assertion above is a \
         discrimination and not a string that never occurs anywhere"
    );
    let all_cat = one("terminal_condition_is_findings_all.toml", ALL_OVER_EMPTY);
    let all_expr = terminal_condition(&all_cat);
    assert_eq!(all_expr, "findings.all(severity != 'blocking')");
    let v2 = quality_block_terminal_condition_rule(&all_cat)
        .expect_err("`findings.all(…)` over an empty set must ALSO fail — §7b.13:1047-1049");
    assert_eq!(v2.len(), 1, "{v2:?}");
    assert_eq!(v2[0].activity(), "ac527-all-over-empty");
    assert_eq!(v2[0].predicate(), all_expr);
    assert_eq!(v2[0].clause(), "terminates_on_no_findings");

    // ---- fixture 3: the positive must LOAD -------------------------------------------------------
    let ok_cat = one(
        "terminal_condition_names_a_verify_result.toml",
        VERIFY_RESULT,
    );
    let ok_expr = terminal_condition(&ok_cat);
    assert_eq!(
        ok_expr, "verify.status == 'passing' and verify.green_at != null",
        "premise: the positive terminates on a verify result, which is the first of AC-527's three \
         lawful terminations"
    );
    let report = quality_block_terminal_condition_rule(&ok_cat).unwrap_or_else(|e| {
        panic!(
            "the positive fixture MUST LOAD — without it every assertion above is satisfied by a \
             lint that refuses every activity: {e:?}"
        )
    });
    assert_eq!(report.activities_walked, 1);
    assert_eq!(
        report.check_steps_walked, 1,
        "premise: the rule really reached the policy step. A walk that reached zero check steps \
         returns Ok for the wrong reason, which is Halt 8 one level down"
    );
    assert_eq!(report.predicates_parsed, 1);

    // The positive's structure is fixture 1's. Swapping ONLY the terminal condition must flip the
    // verdict — if the rule were keyed on the adversary step, the `on_false` loop-back, the block's
    // shape or the activity's stage, this control fails and the three fixtures above would not
    // have shown it.
    let structural = patch(VERIFY_RESULT, &ok_expr, &none_expr);
    let flipped = quality_block_terminal_condition_rule(&one("structural.toml", &structural))
        .expect_err("the SAME activity with ONLY its terminal condition swapped must now fail");
    assert_eq!(flipped.len(), 1);
    assert_eq!(flipped[0].activity(), "ac527-verify-result");

    // And the converse control: the rule is not a denylist of `findings`. `any` over an empty list
    // is FALSE (ADR-0103 decision 4's third arm), so a terminal condition that READS the findings
    // set and is not satisfied by its emptiness must still LOAD.
    let reads_findings = patch(
        VERIFY_RESULT,
        &ok_expr,
        "findings.any(evidence in ['Reproduced', 'MutationSurvived'])",
    );
    let still_ok = quality_block_terminal_condition_rule(&one("reads.toml", &reads_findings))
        .unwrap_or_else(|e| {
            panic!(
                "a findings-reading terminal condition that is FALSE over the empty set must LOAD \
                 — the rule is a form test, not a denylist of the word `findings`: {e:?}"
            )
        });
    assert_eq!(
        still_ok.reads_findings, 1,
        "and the rule really did read the findings set here, so the LOAD above is a decision and \
         not an oversight"
    );

    // ---- the six-cell witness table --------------------------------------------------------------
    // *"any predicate whose truth over an empty findings set is true"* is one sentence with two
    // readings of "no findings" — the empty list and the absent set — and F37 measured both: in the
    // `optional` arm no adversary runs at all, so `${adversary.outputs?}` is ABSENT and the check
    // was *"vacuously true"* 30 times; in `recommended` it is a list. Both are refused, the witness
    // says which, and `rigor` is bound across the three LIVE levels because a block forbidden at
    // one rigor is forbidden.
    assert_eq!(
        LIVE_RIGOR_LEVELS,
        ["optional", "recommended", "mandatory"],
        "`forbidden` admits nothing and `discouraged` is RESERVED in Phase 1 (AC-050, decision #20 \
         of 2026-09-11), so a violation witnessed only at one of those two would be witnessed at a \
         level no pack may set"
    );
    // Absent-only: `findings == null` is true when the set is absent and `null` when it is empty.
    let absent_only = no_findings_witness(&ok_parse("findings == null").expr)
        .expect("evaluates")
        .expect("`findings == null` terminates on the ABSENCE of findings and is refused");
    assert_eq!(absent_only.findings, FindingsWitness::Absent);
    // Empty-only: `findings.len() == 0` is true over the empty list and `null` over the absent set.
    let empty_only = no_findings_witness(&ok_parse("findings.len() == 0").expr)
        .expect("evaluates")
        .expect("`findings.len() == 0` terminates on an empty finding set and is refused");
    assert_eq!(empty_only.findings, FindingsWitness::Empty);
    // Rigor-witnessed: true only where `rigor < mandatory`, which is the shape
    // `spikes/p9/catalog/adversarial-review.toml`:41 carries live. A rule that bound `rigor` to
    // `null` would error on the ordering and let this one through.
    let by_rigor = no_findings_witness(
        &ok_parse("rigor < mandatory and findings.none(evidence in ['Reproduced'])").expr,
    )
    .expect("evaluates")
    .expect("a block forbidden at SOME live rigor is forbidden");
    assert_eq!(by_rigor.rigor, "optional");
    // And the negative that keeps the rigor loop from being a rubber stamp: no live level makes
    // this true, so it is not a violation.
    assert_eq!(
        no_findings_witness(
            &ok_parse("rigor > mandatory and findings.none(evidence in ['X'])").expr
        )
        .expect("evaluates"),
        None,
        "no live rigor level satisfies `rigor > mandatory`, so the witness loop must find nothing"
    );
    // A predicate that never mentions findings is untouched by this rule whatever else it says —
    // and it is untouched because it has **no truth value** here, not because it was decided
    // lawful. The rule binds `findings` and `rigor` and nothing else.
    assert_eq!(
        no_findings_witness(&ok_parse("report.status == 'passing'").expr),
        Err(EvalError::UnknownField("report.status".to_owned())),
        "the rule binds two paths and invents no third"
    );
    // 🛑 AND THE CASE THAT MADE THAT CHOICE, which no fixture in AC-527's set reaches. Binding every
    // read path to `null` was tried first: `null` means *abstained*, and ADR-0103 decision 4 makes
    // `X.all(…)` over an abstaining facet TRUE — so every quantifier over an unknown collection
    // became vacuously true and the rule refused `mutation-testing`'s threshold and
    // `flag-progressive-rollout`'s re-execution outcome, which are the first two shapes ADR-0080
    // decision 2 *prescribes as the remedy*. Only the shipped catalog showed it.
    for prescribed in ["reports.all(score >= 0.8)", "logs.all(status == 'ok')"] {
        assert!(
            no_findings_witness(&ok_parse(prescribed).expr).is_err(),
            "{prescribed} is ADR-0080 decision 2's own remedy and must NOT be refused: it reads no \
             findings set, and a rule that treats an unknown collection as an empty one refuses the \
             record's repair"
        );
    }

    // ---- the three together, which is the only way the counts are checkable ----------------------
    let all_three = Catalog::from_sources(&[
        ("terminal_condition_is_findings_none.toml", NONE_OVER_EMPTY),
        ("terminal_condition_is_findings_all.toml", ALL_OVER_EMPTY),
        (
            "terminal_condition_names_a_verify_result.toml",
            VERIFY_RESULT,
        ),
    ])
    .expect("the three fixtures load as one catalog");
    let both =
        quality_block_terminal_condition_rule(&all_three).expect_err("two of the three must fail");
    assert_eq!(
        both.len(),
        2,
        "EVERY violation, never the first — a rule that returns on the first defect hides the \
         second, and the second is the discriminator: {both:?}"
    );
    let named: BTreeSet<&str> = both
        .iter()
        .map(TerminalConditionViolation::activity)
        .collect();
    assert_eq!(
        named,
        BTreeSet::from(["ac527-all-over-empty", "ac527-none-over-empty"]),
        "and the third activity is NOT among them"
    );

    // ---- 🛑 THE SHIPPED CATALOG: RED BY DESIGN, and the repairs are Step 7e's ---------------------
    let shipped = Catalog::from_dir(&workspace_root().join(SHIPPED_CATALOG))
        .expect("the shipped catalog loads through the new per-step `expr` parse path");
    assert_eq!(
        shipped.len(),
        SHIPPED_ACTIVITIES,
        "premise: this is the shipped catalog and not a subset of it"
    );
    let (shipped_report, shipped_violations) = walk(&shipped);
    assert!(
        quality_block_terminal_condition_rule(&shipped).is_err(),
        "AC-527 is red against the artefacts, and §7b.12 forbids editing them green"
    );
    assert_eq!(
        shipped_violations.len(),
        SHIPPED_VIOLATIONS,
        "eleven shipped policy predicates terminate on the absence of findings. §7b.12:972-976 \
         forbids editing `spikes/p9/` to make a green, so the debt is pinned by this number; if \
         Step 7e has repaired them, this assertion is the notification"
    );
    // The record's own instance is among them, which is what makes this a measurement of AC-527
    // rather than of twelve unrelated predicates.
    assert!(
        shipped_violations.iter().any(|v| {
            v.predicate() == "findings.none(evidence in ['Reproduced', 'MutationSurvived'])"
        }),
        "ADR-0080 decision 2's own predicate is live in the shipped catalog \
         (`differential-debugging.toml`:65 and `root-cause.toml`:62) and the rule must catch it \
         there, not only in the fixture that copies it"
    );
    // The rule walked the whole file set and not a corner of it. A count that COLLAPSES reads
    // exactly like a clean catalog, which is the failure these two numbers exist to catch.
    assert_eq!(
        shipped_report.check_steps_walked, 94,
        "ninety-four shipped steps declare an `expr` [measured 2026-09-14: \
         `grep -rhc '^expr' spikes/p9/catalog/*.toml` sums to 94]"
    );
    assert_eq!(shipped_report.predicates_parsed, 94, "and all 94 parse");
    // 🛑 And the gap between AC-527's THEN and ADR-0080 decision 2, as a NUMBER rather than a worry.
    // These are the predicates that read the findings set — or do not — and resolve to `null` over
    // an empty one, so the criterion's own sentence cannot reach them while the ADR's broader
    // "satisfiable only by an empty finding set" would. The erratum is OWED; this is its size.
    assert_eq!(
        shipped_report.indeterminate(),
        SHIPPED_INDETERMINATE,
        "the size of the reading gap between AC-527's THEN and ADR-0080 decision 2. This stage \
         implements the criterion's sentence and does not decide the ADR's; if that erratum lands \
         and the broader reading is taken, this number is the work it creates"
    );
    assert_eq!(
        shipped_report.refused + shipped_report.indeterminate() + shipped_report.decided,
        shipped_report.predicates_parsed,
        "the four buckets partition the predicates — no verdict is counted twice and none is lost"
    );
    assert_eq!(
        shipped_report.reads_findings, 46,
        "forty-six of the ninety-four read the findings set at all; the rule reaches no other kind, \
         and this is the number that bounds it"
    );
    // 🛑 AND THE HONEST MEASURE OF THIS RULE'S REACH ON THE PRODUCT ARTEFACT, asserted rather than
    // left to be inferred from a green: **not one** shipped terminal condition is decided LAWFUL.
    // Every one of the ninety-four is either refused (11) or has no truth value here (83). A reader
    // who takes the rule's `Ok` on some future repaired catalog as "these predicates were checked
    // and passed" would be wrong, and this assertion is where that is written down.
    assert_eq!(
        shipped_report.decided, 0,
        "no shipped terminal condition is DECIDED lawful by this rule — a green here would be \
         the absence of a refusal, not a verdict"
    );
    // ---- THE FAIL-CLOSED CASE: a terminal condition that does not PARSE is REFUSED ------------
    //
    // Added 2026-09-14 because a mutation survived: replacing the whole
    // `TerminalConditionViolation::PredicateDoesNotParse` arm in `walk` with a silent `continue`
    // left the ENTIRE suite green (106 passed, 0 failed). This module's header and that variant's
    // own `Display` both claim a rule skipping what it could not read would "return Ok on a
    // catalog of garbage" -- and nothing asserted it.
    let garbage = one("terminal_condition_does_not_parse.toml", DOES_NOT_PARSE);
    let refused = quality_block_terminal_condition_rule(&garbage)
        .expect_err("a terminal condition that does not parse must be REFUSED, never skipped");
    assert_eq!(refused.len(), 1, "exactly one violation: {refused:?}");
    assert_eq!(
        refused[0].clause(),
        "predicate_does_not_parse",
        "and it is the FAIL-CLOSED clause, not an empty-set one: an unparseable predicate has no
         truth value over ANY findings set, so it cannot be refused for being true over an empty one"
    );

    println!(
        "ac527_empty_set_terminal_condition: shipped_activities={} check_steps={} refused={} \
         reads_findings={} undecided={} errored={} decided={}",
        shipped.len(),
        shipped_report.check_steps_walked,
        shipped_report.refused,
        shipped_report.reads_findings,
        shipped_report.undecided,
        shipped_report.errored,
        shipped_report.decided
    );
}
