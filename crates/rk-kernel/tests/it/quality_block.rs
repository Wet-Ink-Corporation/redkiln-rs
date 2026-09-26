//! **AC-527**, as amended 2026-09-25 to ADR-0080 decision 2 (D21-41 (h); `s8b` §8b.4; D21-55), and the
//! ADR-0072 record of the catalog edits that brought `.redkiln/catalog` under it.
//!
//! # The fixture set, one per branch
//!
//! | fixture | must | discriminates |
//! | --- | --- | --- |
//! | `terminal_condition_is_findings_none` (1) | FAIL | both refusals; linted as its own activity, refused once by the named form, and its `expr` pinned to ADR-0080's Context spelling (the quoted form §8b.4 settles, per ADR-0103 d3) *(dated 2026-09-25, s8b refuting round 4; wording corrected in refuting round 6: was "ADR-0080 d2's predicate verbatim", but decision 2 spells it unquoted)* |
//! | `terminal_condition_is_findings_all` (2, recast) | FAIL | the witness **alone**: no `findings.none(` in it (asserted over the file's bytes); the predicate uses no `none` quantifier *(dated 2026-09-25, s8b refuting round 1: was "no `none(` in it", which the file's own comment contains)* |
//! | `terminal_condition_names_any_member_list` | FAIL | the named form **alone**: no blocking witness falsifies it |
//! | `terminal_condition_is_one_member_conjunct` | FAIL | the conjunct case, the pre-s8b `user-story-ac` row verbatim |
//! | `terminal_condition_witness_is_per_class` | FAIL | the per-class witness alone: a joint witness does not falsify it (D21-55) |
//! | `terminal_condition_is_severity_only` | LOAD | that the rule does not over-reach: a severity-only predicate reads no `evidence`, so refusal 1 does not match it and refusal 2 finds it indeterminate *(dated 2026-09-25, s8b refuting round 5: was "neither refusal reaches it"; the test asserts `reads_findings == 1` and `indeterminate == 1`, so refusal 2 reaches it and cannot decide)* *(dated 2026-09-25, s8b refuting round 2: was "bullet 1's form"; whether a severity-only row without re-execution conforms to d2 bullet 1 is OPEN, D21-57)* |
//! | `terminal_condition_names_a_verify_result` (3) | LOAD | unchanged. *(dated 2026-09-25, s8b refuting round 4)* Its `verify.green_at != null` conjunct is null for any present `green_at`, so the predicate can never be true (the `x != null` class, `s8b` §8b.10): its LOAD proves only that the rule does not refuse it |
//! | `terminal_condition_does_not_parse` | REFUSED | fail closed |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 3)* `findings.any(evidence in ['Reproduced', 'MutationSurvived'])`, swapped into fixture 3 inside `d2_severity_only_predicates_load` | LOAD, `decided == 1` | the rule's decided arm: a findings-reading predicate that yields a value and that no single-class witness falsifies. No fixture above and no walked catalog reached it |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 4)* `findings.any(evidence in ['Reproduced']) or check.failed == 0`, swapped into fixture 3 inside `d2_severity_only_predicates_load` | LOAD, `indeterminate == 1` | a conjunct valued in some cells and erroring in another is indeterminate, not decided (D21-55) |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 5)* `not (findings.any(evidence in ['MutationSurvived']) or check.failed == 0)`, swapped into fixture 3 inside `d2_per_class_witness_refuses_one_member_rows` | FAIL, `MutationSurvived` at `optional` | a `false` cell outranks an erroring cell of the same conjunct (D21-55; M-W1a) |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 5)* `findings.none(severity in ['blocking']) and findings.len() == 0`, swapped likewise | FAIL, `Reproduced` at `optional` | a falsified conjunct outranks an indeterminate one (D21-55; M-W1b) |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 5)* `rigor < mandatory or not findings.any(evidence in ['Reproduced'])` and `rigor > optional or …` (the same right side), swapped likewise | FAIL, naming `mandatory` / `optional` | the rigor axis: each is false at one live level only (M-W2) |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 5)* `rigor != discouraged or findings.any(evidence in ['NarrowedPin'])`, swapped likewise | LOAD, `decided == 1` | false only at the reserved `discouraged` level, which the loop does not visit |
//! | *(no fixture; dated 2026-09-25, s8b refuting round 6)* `rigor < mandatory and findings.none(severity in ['blocking'])`, swapped likewise | LOAD, `indeterminate == 1` | only a conjunct that reads `findings` is evaluated (D21-55): the first conjunct is false at `mandatory` and is skipped (M-W3) |
//!
//! 🛑 **This file is the one place outside the evaluator's own tests that spells the forbidden form
//! as a Rust literal**, and the checkpoint's d2 guard excludes it by path for that reason. Every other
//! test that needs the forbidden form reads it from a fixture file. *(dated 2026-09-25, s8b refuting
//! round 6: was "Every other test reads predicates from fixture files", which is false — this file's
//! round 3–6 swaps and `tests/it/lattice.rs`'s `applicable_if` probes spell other predicates as
//! literals.)*
//!
//! 🛑 **The `rk-catalog` CLI surface is ARMED.** A kernel test over `.redkiln/catalog` is not
//! `rk-catalog lint`, and the register amendment says so.
//!
//! **Open, carried rather than decided (D21-55):** `findings == null` is not refused by either
//! refusal — a one-finding witness makes it `null`, not `false` — and it is carried to its own item,
//! recorded open in `s8b` §8b.10 *(dated 2026-09-25, s8b refuting round 3)*.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Catalog, StepKind};
use rk_kernel::predicate::{Context, Expr, Tri, Value, evaluate};
use rk_kernel::quality_block::{
    BlockingClass, LIVE_RIGOR_LEVELS, TerminalConditionViolation, blocking_witness, conjuncts,
    names_forbidden_form, quality_block_terminal_condition_rule, walk,
};
use rk_kernel::toml_subset;

use crate::support::ok_parse;

const NONE_TWO_MEMBER: &str =
    include_str!("../fixtures/activities/terminal_condition_is_findings_none.toml");
const ALL_NOT_BLOCKING: &str =
    include_str!("../fixtures/activities/terminal_condition_is_findings_all.toml");
const NAMED_ANY_LIST: &str =
    include_str!("../fixtures/activities/terminal_condition_names_any_member_list.toml");
const ONE_MEMBER_CONJUNCT: &str =
    include_str!("../fixtures/activities/terminal_condition_is_one_member_conjunct.toml");
const PER_CLASS: &str =
    include_str!("../fixtures/activities/terminal_condition_witness_is_per_class.toml");
const SEVERITY_ONLY: &str =
    include_str!("../fixtures/activities/terminal_condition_is_severity_only.toml");
const VERIFY_RESULT: &str =
    include_str!("../fixtures/activities/terminal_condition_names_a_verify_result.toml");
const DOES_NOT_PARSE: &str =
    include_str!("../fixtures/activities/terminal_condition_does_not_parse.toml");

/// The shipped catalog. **Re-pointed 2026-09-25 (s8b, D21-42 13a)** from the spike copy; the p9
/// reading is kept by [`ac527_frozen_p9_catalog_is_red`], at its pinned commit.
const SHIPPED_CATALOG: &str = ".redkiln/catalog";

/// The frozen p9 catalog's refusals under the amended rule, **one per step** (D21-55), measured
/// 2026-09-25 from inside this test binary. It replaces the earlier rule's `11`, which counted a
/// different rule.
const P9_REFUSED: usize = 7;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
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
        "premise: each AC-527 fixture carries exactly one policy predicate: {checks:?}"
    );
    checks[0].to_owned()
}

/// The one violation a must-FAIL fixture produces, asserted to name the activity and the predicate.
fn refused_once(name: &str, src: &str) -> TerminalConditionViolation {
    let cat = one(name, src);
    let expr = terminal_condition(&cat);
    let v =
        quality_block_terminal_condition_rule(&cat).expect_err(&format!("{name} must FAIL lint"));
    assert_eq!(v.len(), 1, "{name}: one violation, not a cascade: {v:?}");
    let activity = cat.activities()[0].id.clone();
    assert_eq!(v[0].activity(), activity);
    assert_eq!(v[0].predicate(), expr);
    let rendered = v[0].to_string();
    assert!(
        rendered.contains(&activity) && rendered.contains(&expr),
        "{name}: the message names the activity and the predicate (AC-527's THEN): {rendered}"
    );
    v.into_iter().next().expect("one")
}

// ---- refusal 2: the per-class witness ---------------------------------------------------------------

/// **Refusal 2, per class, per conjunct** (s8b §8b.4; D21-55). The recast fixture 2 carries no
/// `findings.none(` and is refused by the witness alone; the per-class fixture is refused by a single-class witness and
/// NOT by a joint one; and the conjunct fixture's second conjunct is reached although its first reads
/// an operand nothing binds.
///
/// *(dated 2026-09-25, s8b refuting round 5)* It also pins the two branches of the witness loop no
/// fixture reached: a `false` cell outranks an error, in another cell of the same conjunct (mutant
/// M-W1a) and in another conjunct (M-W1b), and the rigor axis — a conjunct false at one live level
/// only is refused naming that level (M-W2), and one false only at the reserved `discouraged` level
/// loads. *(dated 2026-09-25, s8b refuting round 6)* And D21-55's scope: a findings-free conjunct
/// that is `false` in some cell is skipped, never refused (M-W3).
#[test]
fn d2_per_class_witness_refuses_one_member_rows() {
    // Fixture 2, recast: the witness alone.
    let forbidden_spelling = format!("findings.{}(", "none");
    assert!(
        !ALL_NOT_BLOCKING.contains(&forbidden_spelling),
        "fixture 2 must share no text with the named form, or it cannot show the witness alone"
    );
    // *(dated 2026-09-25, s8b refuting round 4)* The control on that absence check, restored: fixture 1
    // DOES carry the assembled needle, so the assertion above is a discrimination and not a string that
    // never occurs anywhere.
    assert!(
        NONE_TWO_MEMBER.contains(&forbidden_spelling),
        "control: fixture 1 carries `{forbidden_spelling}` — a needle it lacks could never turn the \
         absence check red"
    );
    let v = refused_once("terminal_condition_is_findings_all.toml", ALL_NOT_BLOCKING);
    assert_eq!(v.clause(), "falsified_by_blocking_witness", "{v}");

    // The per-class fixture: refused, by the witness, under the single `Reproduced` class.
    let v = refused_once("terminal_condition_witness_is_per_class.toml", PER_CLASS);
    let TerminalConditionViolation::FalsifiedByBlockingWitness { witness, .. } = &v else {
        panic!("the per-class fixture is not the named form; got {v}");
    };
    assert_eq!(witness.class, BlockingClass::Reproduced);
    // …and a JOINT witness does not falsify it, which is why the witness is per class.
    let per_class = ok_parse(&terminal_condition(&one("p.toml", PER_CLASS))).expr;
    let joint = Value::List(
        BlockingClass::ALL
            .iter()
            .map(|c| match c.witness() {
                Value::List(mut items) => items.remove(0),
                other => panic!("a witness is a one-finding list, got {other:?}"),
            })
            .collect(),
    );
    for rigor in LIVE_RIGOR_LEVELS {
        let ctx = Context::new().with("findings", joint.clone()).with(
            "rigor",
            Value::ordinal(&rk_kernel::quality_block::RIGOR_DOMAIN, rigor).expect("a live level"),
        );
        assert_eq!(
            evaluate(&per_class, &ctx).expect("evaluates").value,
            Tri::True,
            "a joint {{Reproduced, MutationSurvived}} witness satisfies the per-class fixture at \
             {rigor} — only a witness per class refuses it"
        );
    }

    // The conjunct fixture: the named form refuses it, and the witness reaches its second conjunct.
    let v = refused_once(
        "terminal_condition_is_one_member_conjunct.toml",
        ONE_MEMBER_CONJUNCT,
    );
    assert_eq!(v.clause(), "names_forbidden_form");
    let conj = ok_parse(&terminal_condition(&one("c.toml", ONE_MEMBER_CONJUNCT))).expr;
    assert_eq!(
        conjuncts(&conj).len(),
        2,
        "premise: two conjuncts required for truth"
    );
    let w = blocking_witness(&conj)
        .expect("the findings conjunct yields a value")
        .expect(
            "and a Reproduced witness falsifies it, although `check.failed` is bound by nothing",
        );
    assert_eq!(w.class, BlockingClass::Reproduced);

    // *(dated 2026-09-25, s8b refuting round 5)* D21-55's "refused if some such conjunct is `false`
    // in some cell", where other cells or other conjuncts ERROR, and the rigor axis. Each predicate
    // is swapped into fixture 3 (the verify-result positive), so only the terminal condition moves.
    let base = terminal_condition(&one("v.toml", VERIFY_RESULT));
    let swapped = |expr: &str| -> String {
        let src = VERIFY_RESULT.replace(&base, expr);
        assert_eq!(
            terminal_condition(&one("w.toml", &src)),
            expr,
            "premise: the swap reached the check step"
        );
        src
    };
    let cell = |expr: &Expr, class: BlockingClass, rigor: &str| {
        let ctx = Context::new().with("findings", class.witness()).with(
            "rigor",
            Value::ordinal(&rk_kernel::quality_block::RIGOR_DOMAIN, rigor).expect("in the domain"),
        );
        evaluate(expr, &ctx).map(|o| o.value)
    };
    let witness_of = |v: &TerminalConditionViolation| match v {
        TerminalConditionViolation::FalsifiedByBlockingWitness { witness, .. } => {
            (witness.class, witness.rigor)
        }
        other => panic!("refused by the witness, not {}: {other}", other.clause()),
    };
    assert_eq!(
        BlockingClass::ALL[0],
        BlockingClass::Reproduced,
        "premise: the witness loop visits the Reproduced cells first"
    );

    // (a) THE RIGOR AXIS. Each conjunct is false at exactly one live level and true at the other
    // two, under the Reproduced witness, and is REFUSED naming that level. The two ends are both
    // pinned, so a loop that visited any single live level misses one of them (M-W2).
    for (expr, level) in [
        (
            "rigor < mandatory or not findings.any(evidence in ['Reproduced'])",
            "mandatory",
        ),
        (
            "rigor > optional or not findings.any(evidence in ['Reproduced'])",
            "optional",
        ),
    ] {
        let e = ok_parse(expr).expr;
        for rigor in LIVE_RIGOR_LEVELS {
            let want = if rigor == level {
                Tri::False
            } else {
                Tri::True
            };
            assert_eq!(
                cell(&e, BlockingClass::Reproduced, rigor).expect("valued"),
                want,
                "premise: `{expr}` under Reproduced at {rigor}"
            );
        }
        let v = refused_once("rigor_axis.toml", &swapped(expr));
        assert_eq!(
            witness_of(&v),
            (BlockingClass::Reproduced, level),
            "the witness names the one live rigor that falsifies `{expr}`: {v}"
        );
    }
    // …and a conjunct false ONLY at the reserved `discouraged` level, which no pack may set, is not
    // refused: the loop visits the live levels, never the whole domain. It yields a value in every
    // live cell, so it is decided.
    let only_reserved = "rigor != discouraged or findings.any(evidence in ['NarrowedPin'])";
    let e = ok_parse(only_reserved).expr;
    assert_eq!(
        cell(&e, BlockingClass::Reproduced, "discouraged").expect("valued"),
        Tri::False,
        "premise: `{only_reserved}` is false at discouraged — a whole-domain loop would refuse it"
    );
    let r = quality_block_terminal_condition_rule(&one("r.toml", &swapped(only_reserved)))
        .unwrap_or_else(|e| panic!("false only at a non-live rigor must LOAD: {e:?}"));
    assert_eq!(
        (r.reads_findings, r.decided, r.refused, r.indeterminate),
        (1, 1, 0, 0),
        "`{only_reserved}` is decided and not refused over the live levels: {r:?}"
    );

    // (b) ONE conjunct, erroring in some cells and `false` in others: REFUSED. Under `Reproduced`
    // the `or`'s left side is false, so its right side reads `check.failed`, which nothing binds,
    // and every Reproduced cell errors; under `MutationSurvived` the left side is true, the `or` is
    // true, and the `not` makes the conjunct false in every cell. The erroring cells are visited
    // first, so a loop that stopped at its first error would never reach the false one (M-W1a).
    let across_cells = "not (findings.any(evidence in ['MutationSurvived']) or check.failed == 0)";
    let e = ok_parse(across_cells).expr;
    for rigor in LIVE_RIGOR_LEVELS {
        assert!(
            cell(&e, BlockingClass::Reproduced, rigor).is_err(),
            "premise: `{across_cells}` errors under Reproduced at {rigor}"
        );
        assert_eq!(
            cell(&e, BlockingClass::MutationSurvived, rigor).expect("valued"),
            Tri::False,
            "premise: `{across_cells}` is false under MutationSurvived at {rigor}"
        );
    }
    let v = refused_once("across_cells.toml", &swapped(across_cells));
    assert_eq!(
        witness_of(&v),
        (BlockingClass::MutationSurvived, "optional"),
        "a false cell outranks an erroring cell of the same conjunct (D21-55): {v}"
    );

    // (c) TWO findings-reading conjuncts, one erroring in every cell and one false in every cell:
    // REFUSED. `severity` is no witness field, so the first conjunct's lookup falls through to the
    // context and errors; the witness is a one-finding list, so `findings.len() == 0` is false. The
    // erroring conjunct comes first, so a walk that stopped at it would never reach the false one
    // (M-W1b).
    let across_conjuncts = "findings.none(severity in ['blocking']) and findings.len() == 0";
    let e = ok_parse(across_conjuncts).expr;
    let parts = conjuncts(&e);
    assert_eq!(parts.len(), 2, "premise: two conjuncts required for truth");
    for class in BlockingClass::ALL {
        for rigor in LIVE_RIGOR_LEVELS {
            assert!(
                cell(parts[0], class, rigor).is_err(),
                "premise: the severity conjunct errors under {class} at {rigor}"
            );
            assert_eq!(
                cell(parts[1], class, rigor).expect("valued"),
                Tri::False,
                "premise: `findings.len() == 0` is false over a one-finding witness"
            );
        }
    }
    let v = refused_once("across_conjuncts.toml", &swapped(across_conjuncts));
    assert_eq!(
        witness_of(&v),
        (BlockingClass::Reproduced, "optional"),
        "a falsified conjunct outranks an indeterminate one (D21-55): {v}"
    );

    // (d) *(dated 2026-09-25, s8b refuting round 6)* ONLY A CONJUNCT THAT READS `findings` IS
    // EVALUATED (D21-55). The first conjunct reads only `rigor` and is `false` at `mandatory`, so a
    // witness loop that evaluated it would refuse the predicate; the second reads `severity`, no
    // witness field, and errors in every cell. Skipped and indeterminate, the predicate LOADS,
    // counted indeterminate (mutant M-W3: the `reads_findings` skip removed).
    let rigor_only = "rigor < mandatory and findings.none(severity in ['blocking'])";
    let e = ok_parse(rigor_only).expr;
    let parts = conjuncts(&e);
    assert_eq!(parts.len(), 2, "premise: two conjuncts required for truth");
    for class in BlockingClass::ALL {
        for rigor in LIVE_RIGOR_LEVELS {
            let want = if rigor == "mandatory" {
                Tri::False
            } else {
                Tri::True
            };
            assert_eq!(
                cell(parts[0], class, rigor).expect("valued"),
                want,
                "premise: `rigor < mandatory` alone under {class} at {rigor} — false at mandatory, \
                 so evaluating it would refuse"
            );
            assert!(
                cell(parts[1], class, rigor).is_err(),
                "premise: the severity conjunct errors under {class} at {rigor}"
            );
        }
    }
    let s = quality_block_terminal_condition_rule(&one("s.toml", &swapped(rigor_only)))
        .unwrap_or_else(|e| panic!("a findings-free conjunct is skipped, never refused: {e:?}"));
    assert_eq!(
        (s.reads_findings, s.decided, s.refused, s.indeterminate),
        (1, 0, 0, 1),
        "`{rigor_only}` loads, indeterminate: its false conjunct reads no `findings` — {s:?}"
    );

    // Fixture 1 — both refusals reach it.
    // *(dated 2026-09-25, s8b refuting round 4)* It carries ADR-0080's Context spelling, the quoted
    // form §8b.4 settles per ADR-0103 d3 (the premise the pre-s8b test pinned), and the lint over it
    // as its own activity refuses it once, naming that activity and that predicate, by the named
    // form (reported first when present). *(dated 2026-09-25, s8b refuting round 6: was "ADR-0080
    // d2's predicate VERBATIM"; decision 2 spells the members unquoted, `[Reproduced,
    // MutationSurvived]`, and the quoted string is its Context line's.)*
    assert_eq!(
        terminal_condition(&one("n.toml", NONE_TWO_MEMBER)),
        "findings.none(evidence in ['Reproduced', 'MutationSurvived'])",
        "premise: fixture 1 carries ADR-0080's Context spelling (the quoted form §8b.4 settles, per \
         ADR-0103 d3)"
    );
    let v = refused_once("terminal_condition_is_findings_none.toml", NONE_TWO_MEMBER);
    assert_eq!(v.clause(), "names_forbidden_form", "{v}");
    let one_expr = ok_parse(&terminal_condition(&one("n.toml", NONE_TWO_MEMBER))).expr;
    assert!(names_forbidden_form(&one_expr));
    assert!(
        blocking_witness(&one_expr).expect("evaluates").is_some(),
        "fixture 1 is refused by both branches"
    );
}

// ---- refusal 1: the named form ---------------------------------------------------------------------

/// **Refusal 1** (s8b §8b.4): the named form with ANY member list, in any quoting, alone or as a
/// conjunct required for truth — matched on the parse tree.
#[test]
fn d2_named_form_refuses_any_member_list() {
    let v = refused_once(
        "terminal_condition_names_any_member_list.toml",
        NAMED_ANY_LIST,
    );
    assert_eq!(v.clause(), "names_forbidden_form", "{v}");
    let expr = ok_parse(&terminal_condition(&one("a.toml", NAMED_ANY_LIST))).expr;
    assert_eq!(
        blocking_witness(&expr).expect("evaluates"),
        None,
        "no blocking-class witness falsifies a `NarrowedPin`-only list — this fixture is caught by \
         the named-form match and by nothing else"
    );
    // Any quoting, any member list, alone or as a required conjunct.
    for src in [
        "findings.none(evidence in [NarrowedPin])",
        "findings.none(evidence in [\"Reproduced\", \"MutationSurvived\"])",
        "findings.none(evidence in [])",
        "(findings.none(evidence in ['Advisory']))",
        "report.failed > 0 and (findings.none(evidence in ['Reproduced']))",
    ] {
        assert!(
            names_forbidden_form(&ok_parse(src).expr),
            "{src} is the named form"
        );
    }
    // …and not through `or` or `not`, which are not required for truth, nor on another field.
    for src in [
        "report.failed > 0 or findings.none(evidence in ['Reproduced'])",
        "not findings.none(evidence in ['Reproduced'])",
        "findings.none(severity in ['blocking'])",
        "findings.any(evidence in ['Reproduced'])",
    ] {
        assert!(
            !names_forbidden_form(&ok_parse(src).expr),
            "{src} is not the named form"
        );
    }
    // *(dated 2026-09-25, s8b refuting round 2)* One negative per remaining branch of the match. A
    // source other than `findings` is not the named form (the source check)…
    let other_source = "report.none(evidence in ['Reproduced'])";
    assert!(
        !names_forbidden_form(&ok_parse(other_source).expr),
        "{other_source} quantifies over `report`, not `findings`: not the named form"
    );
    // …and neither is an `in` whose right operand is not a list literal (the list check). The
    // grammar cannot spell one — `in` always parses a `[...]` — so the tree is the named form's own,
    // with its list replaced by a path.
    let mut non_list = ok_parse("findings.none(evidence in ['Reproduced'])").expr;
    assert!(
        names_forbidden_form(&non_list),
        "premise: before the edit this tree is the named form"
    );
    let Expr::Quant { body, .. } = &mut non_list else {
        panic!("the method form parses to a quantifier: {non_list:?}");
    };
    let Expr::In { list, .. } = body.as_mut() else {
        panic!("its body parses to `in`: {body:?}");
    };
    **list = Expr::Path("tags".to_owned());
    assert!(
        !names_forbidden_form(&non_list),
        "`findings.none(evidence in tags)` has no list literal: not the named form"
    );
}

// ---- no over-reach ---------------------------------------------------------------------------------

/// **Severity-only predicates read no `evidence` and stay lawful** (s8b §8b.4). A severity-only
/// predicate reads no `evidence`, so a witness finding (evidence only) makes its `severity` lookup
/// error: indeterminate, never refused (mutant M4). *(dated 2026-09-25, s8b refuting round 5: this
/// sentence replaces "d2's bullet 1 is a threshold over severity, so its form must load", which
/// truncated d2 and settled what D21-57 leaves OPEN.)*
///
/// *(dated 2026-09-25, s8b refuting round 2)* What this pins is that the rule does not refuse a
/// severity-only predicate, because it reads no `evidence`. It does not settle that such a
/// predicate is d2 bullet 1's form: bullet 1 reads *"no finding at or above threshold T survives
/// re-execution"*, and whether a severity-only row without re-execution conforms to it is OPEN
/// (D21-57).
#[test]
fn d2_severity_only_predicates_load() {
    let cat = one("terminal_condition_is_severity_only.toml", SEVERITY_ONLY);
    let report = quality_block_terminal_condition_rule(&cat).unwrap_or_else(|e| {
        panic!("a severity-only predicate reads no `evidence` and must LOAD: {e:?}")
    });
    assert_eq!(
        report.check_steps_walked, 1,
        "premise: the rule reached the policy step"
    );
    assert_eq!(
        report.reads_findings, 1,
        "and the step reads the findings set"
    );
    assert_eq!(
        report.indeterminate, 1,
        "the severity lookup has no value over an evidence-only witness: indeterminate, not refused"
    );
    let verify = one(
        "terminal_condition_names_a_verify_result.toml",
        VERIFY_RESULT,
    );
    let ok = quality_block_terminal_condition_rule(&verify)
        .unwrap_or_else(|e| panic!("the verify-result positive must LOAD: {e:?}"));
    assert_eq!(ok.check_steps_walked, 1);
    assert_eq!(
        ok.reads_findings, 0,
        "it reads no findings, so the rule has no subject in it"
    );
    // The positive's structure is fixture 1's: swapping ONLY the terminal condition flips the verdict.
    let ok_expr = terminal_condition(&verify);
    let none_expr = terminal_condition(&one("n.toml", NONE_TWO_MEMBER));
    assert_eq!(
        VERIFY_RESULT.matches(&ok_expr).count(),
        1,
        "golden constraint 7"
    );
    let swapped = VERIFY_RESULT.replace(&ok_expr, &none_expr);
    assert!(
        quality_block_terminal_condition_rule(&one("s.toml", &swapped)).is_err(),
        "the SAME activity with only its terminal condition swapped must now fail"
    );
    // *(dated 2026-09-25, s8b refuting round 3)* The DECIDED arm: a findings-reading predicate that
    // yields a value under both single-class witnesses and is falsified by neither must LOAD through
    // the rule, counted as decided. No fixture and no walked catalog reached `walk`'s `Ok(None)` arm,
    // so a rule that refused there survived every test. The same activity, only its terminal
    // condition swapped.
    let decidable = "findings.any(evidence in ['Reproduced', 'MutationSurvived'])";
    let decided_src = VERIFY_RESULT.replace(&ok_expr, decidable);
    let decided_cat = one("d.toml", &decided_src);
    assert_eq!(
        terminal_condition(&decided_cat),
        decidable,
        "premise: the swap reached the check step"
    );
    let d = quality_block_terminal_condition_rule(&decided_cat).unwrap_or_else(|e| {
        panic!("a findings-reading predicate no single-class witness falsifies must LOAD: {e:?}")
    });
    assert_eq!(
        d.check_steps_walked, 1,
        "premise: the rule reached the step"
    );
    assert_eq!(d.reads_findings, 1, "the step reads the findings set");
    assert_eq!(
        (d.decided, d.refused, d.indeterminate),
        (1, 0, 0),
        "it yields a value in every cell and no witness falsifies it: decided, not refused — {d:?}"
    );
    // *(dated 2026-09-25, s8b refuting round 4)* A conjunct that errors in SOME cells and is valued
    // in the others is INDETERMINATE (D21-55, literally: "a conjunct that errors is not refused; it
    // is counted as indeterminate"). Under the `Reproduced` witness the `or` is true on its left;
    // under `MutationSurvived` the left is false and the right reads `check.failed`, which nothing
    // binds, so that cell errors. No cell is false, so it is not refused either.
    let partial = "findings.any(evidence in ['Reproduced']) or check.failed == 0";
    let partial_cat = one("p.toml", &VERIFY_RESULT.replace(&ok_expr, partial));
    assert_eq!(
        terminal_condition(&partial_cat),
        partial,
        "premise: the swap reached the check step"
    );
    let p = quality_block_terminal_condition_rule(&partial_cat).unwrap_or_else(|e| {
        panic!("a conjunct that errors in some cells and is false in none must LOAD: {e:?}")
    });
    assert_eq!(p.reads_findings, 1, "the step reads the findings set");
    assert_eq!(
        (p.decided, p.refused, p.indeterminate),
        (0, 0, 1),
        "a conjunct that errors in some cell is indeterminate, not decided (D21-55) — {p:?}"
    );
    // Fail closed: a terminal condition that does not parse is REFUSED, never skipped.
    let v = refused_once("terminal_condition_does_not_parse.toml", DOES_NOT_PARSE);
    assert_eq!(v.clause(), "predicate_does_not_parse");
}

// ---- the shipped catalog, and the frozen one -------------------------------------------------------

/// **AC-527 over `.redkiln/catalog`**, after s8b's ADR-0072 edits: every check step walked, and none
/// refused. The name is kept because Checkpoint 7b selects it. *(dated 2026-09-25, s8b refuting round
/// 4)* "Every check step" is pinned, not only a non-zero count: the catalog carries **94** `check`
/// steps with an `expr` [measured 2026-09-25, printed by this test as `checks=94`], the figure the
/// pre-s8b test pinned; a walker that skipped most of them would otherwise reach `refused == 0` more
/// easily.
#[test]
fn ac527_empty_set_terminal_condition() {
    let shipped = Catalog::from_dir(&workspace_root().join(SHIPPED_CATALOG))
        .expect("the shipped catalog loads");
    assert_eq!(shipped.len(), 102, "premise: the whole shipped catalog");
    let (report, violations) = walk(&shipped);
    assert!(
        report.check_steps_walked > 0 && report.predicates_parsed == report.check_steps_walked,
        "the count walked comes first: a rule over zero check steps refuses nothing — {report:?}"
    );
    assert_eq!(
        report.check_steps_walked, 94,
        "every check step carrying an `expr` in `.redkiln/catalog` is walked: {report:?}"
    );
    assert_eq!(
        report.refused + report.decided + report.indeterminate,
        report.reads_findings,
        "the three buckets partition the findings-reading steps: {report:?}"
    );
    println!(
        "ac527: catalog={SHIPPED_CATALOG} checks={} refused={} indeterminate={}",
        report.check_steps_walked, report.refused, report.indeterminate
    );
    assert!(
        violations.is_empty() && report.refused == 0,
        "AC-527 over `.redkiln/catalog`: {} terminal condition(s) refused (ADR-0080 decision 2): \
         {violations:#?}",
        violations.len()
    );
}

/// **A frozen control, named so** (D21-47 V2). The spike catalog at `P9_PIN` still carries the seven
/// d2 rows `.redkiln/catalog` carried until s8b, so the amended rule must stay RED over it, counted
/// one refusal per step (D21-55). *(dated 2026-09-25, s8b refuting round 2)* The refused
/// `(activity, step)` set is pinned to [`D2_ROWS`], not only its size: a rule that stopped refusing
/// one d2 row and started refusing another p9 check step would keep the count at seven.
#[test]
fn ac527_frozen_p9_catalog_is_red() {
    let p9 = Catalog::from_dir(&crate::p9_pin::root().join("catalog"))
        .expect("the pinned p9 catalog loads");
    assert_eq!(p9.len(), 102, "premise: the whole pinned catalog");
    let (report, violations) = walk(&p9);
    assert!(
        report.check_steps_walked > 0,
        "the count walked comes first: {report:?}"
    );
    let steps: BTreeSet<(&str, &str)> = violations
        .iter()
        .map(|v| (v.activity(), v.step()))
        .collect();
    assert_eq!(
        steps.len(),
        violations.len(),
        "one refusal per step, never per clause"
    );
    assert_eq!(
        report.refused, P9_REFUSED,
        "the frozen p9 catalog's refusals under the amended rule: {violations:#?}"
    );
    let d2: BTreeSet<(&str, &str)> = D2_ROWS.into_iter().collect();
    assert_eq!(
        steps, d2,
        "the frozen p9 catalog's refusals are exactly the seven d2 rows: {violations:#?}"
    );
}

// ---- conditions (b) and (c) over the rewritten rows (s8b §8b.5) -----------------------------------

/// The seven ADR-0080 d2 rows s8b rewrote, by (activity, step).
const D2_ROWS: [(&str, &str); 7] = [
    ("root-cause", "policy"),
    ("adversarial-review", "verdict"),
    ("differential-debugging", "policy"),
    ("instrumented-trace-analysis", "policy"),
    ("threat-model", "policy"),
    ("user-story-ac", "policy"),
    ("vertical-slicing", "policy"),
];

fn shipped_toml(id: &str) -> toml_subset::Table {
    let path = workspace_root()
        .join(SHIPPED_CATALOG)
        .join(format!("{id}.toml"));
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    toml_subset::parse(&src).unwrap_or_else(|e| panic!("{id}.toml parses: {e:?}"))
}

fn toml_steps(t: &toml_subset::Table) -> Vec<&toml_subset::Table> {
    t.get("step")
        .and_then(toml_subset::Value::as_array)
        .expect("[[step]] tables")
        .iter()
        .filter_map(toml_subset::Value::as_table)
        .collect()
}

fn step_id(s: &toml_subset::Table) -> &str {
    s.get("id")
        .and_then(toml_subset::Value::as_str)
        .expect("a step id")
}

fn toml_step<'a>(t: &'a toml_subset::Table, id: &str) -> &'a toml_subset::Table {
    toml_steps(t)
        .into_iter()
        .find(|s| step_id(s) == id)
        .unwrap_or_else(|| panic!("no step `{id}`"))
}

fn toml_str<'a>(s: &'a toml_subset::Table, key: &str) -> Option<&'a str> {
    s.get(key).and_then(toml_subset::Value::as_str)
}

/// Every root name the expression reads through a path (`report` in `report.failed`).
fn roots(e: &Expr, out: &mut BTreeSet<String>) {
    match e {
        Expr::Path(p) => {
            out.insert(p.split('.').next().unwrap_or(p).to_owned());
        }
        Expr::Not(x) | Expr::Group(x) | Expr::Len(x) => roots(x, out),
        Expr::Or(l, r) | Expr::And(l, r) => {
            roots(l, out);
            roots(r, out);
        }
        Expr::Cmp { left, right, .. } => {
            roots(left, out);
            roots(right, out);
        }
        Expr::In { left, list } => {
            roots(left, out);
            roots(list, out);
        }
        Expr::Member { literal, set } => {
            roots(literal, out);
            roots(set, out);
        }
        Expr::List(items) => items.iter().for_each(|i| roots(i, out)),
        // *(dated 2026-09-25, s8b refuting round 5)* Was: "A quantifier's body reads its element's
        // fields; only its SOURCE is a context read." The evaluator pushes the element's fields as a
        // frame on the OUTER scope, so a body path that is no field falls through to the context
        // (`src/predicate/eval.rs` `quantify`; the same blind spot as the lattice's M30b). The body's
        // roots are collected by `body_roots`, and condition (c) checks those that name a binding.
        Expr::Quant { source, .. } => roots(source, out),
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

/// Every root read inside a quantifier body, less the body's bound variable (and those of any
/// enclosing quantifier). Which of them are element fields and which fall through to the context is
/// not decidable here without the element's schema; condition (c) treats a body root that names one
/// of the check's own input bindings as a context read, because that is the only way such a read
/// can have a value the loop might not vary. *(dated 2026-09-25, s8b refuting round 5.)*
fn body_roots(e: &Expr, bound: &mut Vec<String>, out: &mut BTreeSet<String>) {
    match e {
        Expr::Quant {
            source, var, body, ..
        } => {
            body_roots(source, bound, out);
            let mut inner = BTreeSet::new();
            roots(body, &mut inner);
            if let Some(v) = var {
                bound.push(v.clone());
            }
            out.extend(inner.into_iter().filter(|r| !bound.contains(r)));
            body_roots(body, bound, out); // nested quantifiers' bodies too
            if var.is_some() {
                bound.pop();
            }
        }
        Expr::Not(x) | Expr::Group(x) | Expr::Len(x) => body_roots(x, bound, out),
        Expr::Or(l, r) | Expr::And(l, r) => {
            body_roots(l, bound, out);
            body_roots(r, bound, out);
        }
        Expr::Cmp { left, right, .. } => {
            body_roots(left, bound, out);
            body_roots(right, bound, out);
        }
        Expr::In { left, list } => {
            body_roots(left, bound, out);
            body_roots(list, bound, out);
        }
        Expr::Member { literal, set } => {
            body_roots(literal, bound, out);
            body_roots(set, bound, out);
        }
        Expr::List(items) => items.iter().for_each(|i| body_roots(i, bound, out)),
        Expr::Path(_) | Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

/// Every full path the expression reads (`report.failed`), quantifier bodies excluded.
fn full_paths(e: &Expr, out: &mut BTreeSet<String>) {
    match e {
        Expr::Path(p) => {
            out.insert(p.clone());
        }
        Expr::Not(x) | Expr::Group(x) | Expr::Len(x) => full_paths(x, out),
        Expr::Or(l, r) | Expr::And(l, r) => {
            full_paths(l, out);
            full_paths(r, out);
        }
        Expr::Cmp { left, right, .. } => {
            full_paths(left, out);
            full_paths(right, out);
        }
        Expr::In { left, list } => {
            full_paths(left, out);
            full_paths(list, out);
        }
        Expr::Member { literal, set } => {
            full_paths(literal, out);
            full_paths(set, out);
        }
        Expr::List(items) => items.iter().for_each(|i| full_paths(i, out)),
        Expr::Quant { source, .. } => full_paths(source, out),
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

/// **Condition (b)** (s8b §8b.5): each rewritten `expr` is not a constant function of its operands'
/// domains. Every path it reads is bound over a small pool of values of every shape — numbers,
/// strings, an empty list and a one-finding list — and the evaluations must yield BOTH truth values.
/// The design's golden constraint 15: *"A policy predicate that is a constant function of its arm
/// carries no information and may not terminate anything."*
#[test]
fn d2_rewritten_rows_are_not_constant_over_their_quantity() {
    let finding = Value::Record(BTreeMap::from([
        ("evidence".to_owned(), Value::Str("Reproduced".to_owned())),
        ("severity".to_owned(), Value::Str("blocking".to_owned())),
    ]));
    let pool = [
        Value::Int(0),
        Value::Int(1),
        Value::Str("high".to_owned()),
        Value::Str("low".to_owned()),
        Value::List(Vec::new()),
        Value::List(vec![finding]),
    ];
    // *(dated 2026-09-25, s8b refuting round 5)* Was `assert!(walked > 0, …)`, which held whenever
    // the loop was reached. Both counts are now pinned exactly: `walked` is every combination, and
    // `valued` only those whose evaluation yielded a value.
    let mut walked = 0usize;
    let mut valued = 0usize;
    for (id, step) in D2_ROWS {
        let t = shipped_toml(id);
        let expr_src = toml_str(toml_step(&t, step), "expr").expect("the row carries an expr");
        let expr = ok_parse(expr_src).expr;
        let mut paths = BTreeSet::new();
        full_paths(&expr, &mut paths);
        let paths: Vec<String> = paths.into_iter().filter(|p| p != "rigor").collect();
        assert!(
            !paths.is_empty(),
            "{id}/{step}: `{expr_src}` reads nothing — a constant"
        );
        let mut seen = BTreeSet::new();
        let combos = pool
            .len()
            .pow(u32::try_from(paths.len()).expect("few paths"));
        for n in 0..combos {
            let mut ctx = Context::new().with(
                "rigor",
                Value::ordinal(&rk_kernel::quality_block::RIGOR_DOMAIN, "mandatory")
                    .expect("a live level"),
            );
            let mut k = n;
            for p in &paths {
                ctx.bind(p, pool[k % pool.len()].clone());
                k /= pool.len();
            }
            if let Ok(o) = evaluate(&expr, &ctx) {
                seen.insert(format!("{:?}", o.value));
                valued += 1;
            }
            walked += 1;
        }
        assert!(
            seen.contains("True") && seen.contains("False"),
            "{id}/{step}: `{expr_src}` is constant over its bindings ({seen:?}) — golden constraint \
             15: it carries no information and may not terminate anything"
        );
    }
    // Six rows read one path (6 combinations each) and `instrumented-trace-analysis` reads two (36):
    // 72 walked, of which 54 yield a value [measured 2026-09-25, printed by this test as
    // `d2-constancy: rows=7 walked=72 valued=54`]. `threat-model`'s row waits on §8b.6, so its
    // rewrite moves `valued` by a dated edit.
    println!(
        "d2-constancy: rows={} walked={walked} valued={valued}",
        D2_ROWS.len()
    );
    assert_eq!(
        walked, 72,
        "every combination of every row's paths is walked"
    );
    assert_eq!(valued, 54, "the evaluations that yielded a value");
}

/// The step ids a step's `${<step>.outputs…}` inputs reference. `artifact:` and `toolchain:` inputs
/// reference no step, so they break a chain. The reference parse is `src`'s own
/// (`rigor::output_refs`, which AC-303's confirm rule walks; D21-57), not a private copy.
fn output_refs(s: &toml_subset::Table) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    if let Some(inputs) = s.get("inputs").and_then(toml_subset::Value::as_table) {
        for v in inputs.values() {
            if let Some(src) = v.as_str() {
                out.extend(rk_kernel::rigor::output_refs(src));
            }
        }
    }
    out
}

/// Condition (c) over one row: `Ok(false)` when the check does not loop back, `Ok(true)` when it does
/// and every binding it reads is reachable from the loop-back target, and `Err` naming the first
/// binding that is not. *(dated 2026-09-25, s8b refuting round 5: factored out of the test so that a
/// crafted must-fail row runs the same check.)*
fn loop_row_check(id: &str, t: &toml_subset::Table, step: &str) -> Result<bool, String> {
    let check = toml_step(t, step);
    let Some(target) = check
        .get("on_false")
        .and_then(toml_subset::Value::as_table)
        .and_then(|o| o.get("to"))
        .and_then(toml_subset::Value::as_str)
    else {
        return Ok(false); // `continue` / `fail`: no loop, so no quantity the loop varies
    };
    let by_id: BTreeMap<&str, &toml_subset::Table> =
        toml_steps(t).into_iter().map(|s| (step_id(s), s)).collect();
    // A step "carries" the loop when it is the target or references, transitively, a step that is.
    let carries = |start: &str| -> bool {
        let mut stack = vec![start.to_owned()];
        let mut seen = BTreeSet::new();
        while let Some(s) = stack.pop() {
            if s == target {
                return true;
            }
            if !seen.insert(s.clone()) {
                continue;
            }
            if let Some(st) = by_id.get(s.as_str()) {
                stack.extend(output_refs(st));
            }
        }
        false
    };
    let expr = ok_parse(toml_str(check, "expr").expect("an expr")).expr;
    let inputs = check
        .get("inputs")
        .and_then(toml_subset::Value::as_table)
        .expect("the check binds inputs");
    let mut read = BTreeSet::new();
    roots(&expr, &mut read);
    // *(dated 2026-09-25, s8b refuting round 5)* A quantifier body's root that names one of this
    // check's input bindings falls through to the context and is a read too (M-C1).
    let mut in_bodies = BTreeSet::new();
    body_roots(&expr, &mut Vec::new(), &mut in_bodies);
    read.extend(in_bodies.into_iter().filter(|r| inputs.contains_key(r)));
    read.remove("rigor");
    if read.is_empty() {
        return Err(format!("{id}/{step}: the expr reads no binding"));
    }
    for binding in &read {
        let src = inputs
            .get(binding)
            .and_then(toml_subset::Value::as_str)
            .ok_or_else(|| format!("{id}/{step}: `{binding}` is read and not bound"))?;
        let refs = output_refs(&BTreeMap::from([(
            "inputs".to_owned(),
            toml_subset::Value::Table(BTreeMap::from([(
                binding.clone(),
                toml_subset::Value::Str(src.to_owned()),
            )])),
        )]));
        if !refs.iter().any(|r| carries(r)) {
            return Err(format!(
                "{id}/{step}: `{binding} = {src}` is not reachable from the loop-back target \
                 `{target}` — a quantity the loop does not vary"
            ));
        }
    }
    Ok(true)
}

/// **Condition (c)** (s8b §8b.5; D21-47 V5): for a loop-back row, every binding the `expr` reads comes
/// from the `on_false.to` target, or from a step reachable from it through a chain of
/// `${<step>.outputs…}` input references — transitively, because `instrumented-trace-analysis`'s
/// target reaches `confirm` only through `collect` and `analyse`.
///
/// *(dated 2026-09-25, s8b refuting round 5)* A binding read inside a quantifier body counts too: the
/// evaluator lets a body path that is no element field fall through to the context. A crafted
/// `root-cause` row whose body reads an `artifact:` binding must fail (mutant M-C1 drops the body
/// walk and turns it green).
#[test]
fn d2_rewritten_rows_read_a_quantity_the_loop_varies() {
    let mut loop_rows = 0usize;
    for (id, step) in D2_ROWS {
        match loop_row_check(id, &shipped_toml(id), step) {
            Ok(true) => loop_rows += 1,
            Ok(false) => {}
            Err(e) => panic!("{e}"),
        }
    }
    assert_eq!(
        loop_rows, 6,
        "six of the seven rows loop back; adversarial-review continues"
    );
    // The crafted must-fail row: `root-cause`'s policy, reading `limit` only inside a body.
    let path = workspace_root()
        .join(SHIPPED_CATALOG)
        .join("root-cause.toml");
    let src = std::fs::read_to_string(&path).expect("root-cause.toml is readable");
    let (old_in, old_expr) = (
        r#"inputs  = { report = "${confirm.outputs}", findings = "${adversary.outputs?}" }"#,
        r#"expr    = "report.failed > 0""#,
    );
    assert_eq!(
        (src.matches(old_in).count(), src.matches(old_expr).count()),
        (1, 1),
        "premise: root-cause's policy lines are the ones this swap edits"
    );
    let crafted = src
        .replace(
            old_in,
            r#"inputs  = { report = "${confirm.outputs}", findings = "${adversary.outputs?}", limit = "artifact:Threshold" }"#,
        )
        .replace(
            old_expr,
            r#"expr    = "report.failed > 0 and report.cases.none(limit.level > 0)""#,
        );
    let t = toml_subset::parse(&crafted).expect("the crafted row parses");
    let err = loop_row_check("crafted root-cause", &t, "policy").expect_err(
        "a body that reads an `artifact:` binding reads a quantity the loop does not vary",
    );
    assert!(
        err.contains("`limit = artifact:Threshold`"),
        "the refusal names the body's binding: {err}"
    );
}

// ---- ADR-0072, executed ----------------------------------------------------------------------------

fn changelog() -> String {
    std::fs::read_to_string(workspace_root().join(".redkiln/CHANGELOG.md"))
        .expect(".redkiln/CHANGELOG.md exists (ADR-0072's pack changelog)")
}

fn version_of(id: &str) -> i64 {
    shipped_toml(id)
        .get("activity")
        .and_then(toml_subset::Value::as_table)
        .and_then(|a| a.get("version"))
        .and_then(toml_subset::Value::as_int)
        .unwrap_or_else(|| panic!("{id} declares an integer version"))
}

fn assert_versioned(triples: &[(&str, i64, i64)]) {
    let log = changelog();
    for (id, old, new) in triples {
        assert_eq!(
            version_of(id),
            *new,
            "{id}: a coherence edit bumps version {old} → {new} (ADR-0072)"
        );
        let row = log
            .lines()
            .find(|l| l.contains(&format!("| {id}@{new} |")))
            .unwrap_or_else(|| panic!("no changelog row names {id}@{new} (ADR-0072)"));
        assert!(
            row.contains(&format!("from v{old}")),
            "{id}@{new}'s changelog row does not name the version it replaced: {row}"
        );
    }
}

/// **ADR-0072 for the seven d2 rows:** *"Edits to shipped activities made for catalog coherence bump
/// `version` and are listed in the pack changelog."* Also D21-47 V4: `adversarial-review`'s `verdict`
/// runs after `record`, whose outcome it reads (mutant M29).
#[test]
fn adr0072_d2_edits_are_versioned_and_changelogged() {
    assert_versioned(&[
        ("root-cause", 4, 5),
        ("adversarial-review", 3, 4),
        ("differential-debugging", 2, 3),
        ("instrumented-trace-analysis", 2, 3),
        ("threat-model", 1, 2),
        ("user-story-ac", 2, 3),
        ("vertical-slicing", 1, 2),
    ]);
    let review = shipped_toml("adversarial-review");
    let order: Vec<&str> = toml_steps(&review).into_iter().map(step_id).collect();
    let pos = |s: &str| order.iter().position(|x| *x == s);
    assert!(
        matches!((pos("record"), pos("verdict")), (Some(r), Some(v)) if r < v),
        "D21-47 V4: `verdict` must follow `record`, whose outcome it reads — order is {order:?}"
    );
}

/// **ADR-0072 for s8b's three further edits** (D21-43 N26, N13, N25): the fourteen `memo` repairs
/// (`instrumented-trace-analysis`'s bump shared with its N18 edit), `tdd-red-green-refactor` and
/// `sca-scan`.
#[test]
fn adr0072_s8b_repairs_are_versioned_and_changelogged() {
    assert_versioned(&[
        ("blue-green-deploy", 2, 3),
        ("failover", 2, 3),
        ("feature-flag-kill", 3, 4),
        ("flag-progressive-rollout", 2, 3),
        ("incident-command", 1, 2),
        ("instrumented-trace-analysis", 2, 3),
        ("migration-dry-run", 2, 3),
        ("migration-expand-execute", 2, 3),
        ("rate-limit-shed", 2, 3),
        ("rollback-mitigation", 2, 3),
        ("rollback", 1, 2),
        ("rolling-deploy", 2, 3),
        ("rolling-restart", 2, 3),
        ("slo-monitoring", 2, 3),
        ("tdd-red-green-refactor", 2, 3),
        ("sca-scan", 1, 2),
    ]);
}
