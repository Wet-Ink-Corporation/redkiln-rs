//! `predicate_null_arithmetic` and `quantifier_empty_and_null` — the first implementation of
//! ADR-0103 decision 4.
//!
//! F42 says so in terms: the probe parses and does not evaluate, and the corpus carries **0**
//! quantifiers, so every fixture here is **authored**. That is why each arm is asserted separately:
//! there is no corpus to regress against and a single happy-path case would pin nothing.

use std::collections::BTreeSet;

use rk_kernel::predicate::{Context, EvalError, LintError, Tri, Value, evaluate, lint_predicate};

use crate::support::{BANDS, abstaining_context, eval, ok_parse, truth};

fn nullable(axes: &[&str]) -> BTreeSet<String> {
    axes.iter().map(|s| (*s).to_owned()).collect()
}

/// One must-fail case per arm of ADR-0103 decision 4.
#[test]
fn predicate_null_arithmetic() {
    let absent = abstaining_context();
    let present = Context::new()
        .with("novelty", Value::ordinal(&BANDS, "high").unwrap())
        .with("has_repro", Value::Bool(true))
        .with("market_facing", Value::Bool(false));

    // --- equality ------------------------------------------------------------------------
    // `null == null` is TRUE. It is only reachable through a facet, because an unquoted `null` in
    // OPERAND position is not a field and does not parse — which is itself the literal rule.
    assert_eq!(truth("novelty == null", &absent), Tri::True, "null == null");
    // `null == v` and `v == null` for non-null v are NULL, not false. This is the clause ADR-0089
    // decision 3 was written to protect: an absent fact must not behave like a wrong fact.
    assert_eq!(truth("novelty == high", &absent), Tri::Null, "null == v");
    assert_eq!(truth("novelty == null", &present), Tri::Null, "v == null");
    // `!=` is the negation of `==` (see rk_kernel::predicate::eval's reading 1).
    assert_eq!(
        truth("novelty != null", &absent),
        Tri::False,
        "null != null"
    );
    assert_eq!(truth("novelty != high", &absent), Tri::Null, "null != v");
    assert_eq!(truth("novelty != null", &present), Tri::Null, "v != null");
    // `in` against `null` is `null`.
    assert_eq!(
        truth("novelty in [low, high]", &absent),
        Tri::Null,
        "null in […]"
    );
    // ADR-0050's set-membership form against an absent set is `null` too.
    let no_set = Context::new().with("change", Value::Null);
    assert_eq!(
        truth("'text' in change", &no_set),
        Tri::Null,
        "lit in null-set"
    );
    let with_set = Context::new().with("change", Value::set(["text", "code"]));
    assert_eq!(truth("'text' in change", &with_set), Tri::True);
    assert_eq!(truth("'schema' in change", &with_set), Tri::False);

    // --- `not` ---------------------------------------------------------------------------
    assert_eq!(truth("not has_repro", &absent), Tri::Null, "not null");
    assert_eq!(truth("not has_repro", &present), Tri::False);
    assert_eq!(truth("not market_facing", &present), Tri::True);

    // --- three-valued `and` / `or`, both absorbing positions -----------------------------
    // The record names one example per connective; both positions absorb (reading 2, and the
    // erratum it records as OWED). A non-commutative `and` would make `a and b` differ from
    // `b and a`, which nothing in the records contemplates.
    let f_n = Context::new()
        .with("has_repro", Value::Bool(false))
        .with("market_facing", Value::Null);
    let n_f = Context::new()
        .with("has_repro", Value::Null)
        .with("market_facing", Value::Bool(false));
    let t_n = Context::new()
        .with("has_repro", Value::Bool(true))
        .with("market_facing", Value::Null);
    let n_t = Context::new()
        .with("has_repro", Value::Null)
        .with("market_facing", Value::Bool(true));
    let n_n = Context::new()
        .with("has_repro", Value::Null)
        .with("market_facing", Value::Null);

    assert_eq!(
        truth("has_repro and market_facing", &f_n),
        Tri::False,
        "false and null"
    );
    assert_eq!(
        truth("has_repro and market_facing", &n_f),
        Tri::False,
        "null and false"
    );
    assert_eq!(
        truth("has_repro and market_facing", &t_n),
        Tri::Null,
        "true and null"
    );
    assert_eq!(
        truth("has_repro and market_facing", &n_t),
        Tri::Null,
        "null and true"
    );
    assert_eq!(
        truth("has_repro and market_facing", &n_n),
        Tri::Null,
        "null and null"
    );
    assert_eq!(
        truth("has_repro or market_facing", &t_n),
        Tri::True,
        "true or null"
    );
    assert_eq!(
        truth("has_repro or market_facing", &n_t),
        Tri::True,
        "null or true"
    );
    assert_eq!(
        truth("has_repro or market_facing", &f_n),
        Tri::Null,
        "false or null"
    );
    assert_eq!(
        truth("has_repro or market_facing", &n_f),
        Tri::Null,
        "null or false"
    );
    assert_eq!(
        truth("has_repro or market_facing", &n_n),
        Tri::Null,
        "null or null"
    );

    // --- ordering: a LOAD-TIME lint error unless guarded ---------------------------------
    let axes = nullable(&["novelty"]);
    let unguarded = ok_parse("novelty >= high");
    assert!(
        matches!(
            lint_predicate(&unguarded.expr, &axes),
            Err(LintError::UnguardedOrderingAgainstNullable { .. })
        ),
        "an unguarded ordering against a nullable axis must fail LOAD, not evaluate"
    );
    // The ONE narrowing pattern, in both spellings the two records give.
    for good in [
        "novelty != null and novelty >= high",
        "novelty != unknown and novelty >= high",
    ] {
        assert!(
            lint_predicate(&ok_parse(good).expr, &axes).is_ok(),
            "{good:?} is ADR-0103 decision 4's pattern and must LOAD"
        );
    }
    // THE PATTERN IS SYNTACTIC, NOT SEMANTIC (AC-069's Verified by; PIA-052). Both of the first
    // two read as equivalent to a human and both must fail: one carries a `not`, the other an `or`.
    //
    // THE FIVE THAT FOLLOW PIN THE GUARD'S OWN SHAPE, and nothing did until 2026-09-13. The two
    // above both fail with the top-level guarded set already EMPTY, so they discriminate nothing
    // about how a guard is recognised or how far it reaches; each of the five below is admitted by
    // a one-line widening of `guarded_axis` or of the `and`/`or` walk, and each such widening left
    // this test green. In order: the guard's operator is `!=` and not `==` (the inverse guard);
    // the RHS is `null` or `unknown` and no third spelling (the module header's *"both spellings
    // and no third"*); the guard must come BEFORE the ordering at its own level, because the
    // pattern is written `x != null and <ordering on x>` and running order is what makes the
    // short-circuit safe; a guard does not survive into an `or`, in either branch.
    for bad in [
        "not (novelty == unknown) and novelty >= high",
        "novelty == unknown or novelty >= high",
        "novelty == unknown and novelty >= high",
        "novelty != high and novelty >= high",
        "novelty >= high and novelty != null",
        "novelty != null and (novelty >= high or market_facing)",
        "(novelty != null or market_facing) and novelty >= high",
    ] {
        assert!(
            matches!(
                lint_predicate(&ok_parse(bad).expr, &axes),
                Err(LintError::UnguardedOrderingAgainstNullable { .. })
            ),
            "{bad:?} is not the pattern and must FAIL load"
        );
    }
    // A non-nullable axis orders freely — without this the rule is satisfied by a lint that
    // rejects every ordering.
    let ordered = Context::new().with("novelty", Value::ordinal(&BANDS, "critical").unwrap());
    assert!(lint_predicate(&unguarded.expr, &nullable(&[])).is_ok());
    assert_eq!(truth("novelty >= high", &ordered), Tri::True);
    assert_eq!(truth("novelty < high", &ordered), Tri::False);

    // --- and the third defence: an ordering that still REACHES an absent operand ----------
    // ADR-0069 forbids evaluating it "with some default ordering", so it is an ERROR and never a
    // value. The guard's short-circuit is what keeps a LOADED predicate off this path.
    let err = evaluate(&unguarded.expr, &absent)
        .expect_err("ordering against an absent operand is not a truth value");
    assert!(matches!(err, EvalError::OrderingAgainstNull(_)), "{err}");
    assert_eq!(
        truth("novelty != null and novelty >= high", &absent),
        Tri::False,
        "the guard short-circuits: `false and X` never evaluates X, which is what makes \
         ADR-0103 decision 4's own pattern safe against ADR-0069's error"
    );

    // RAISED, NOT SETTLED, and asserted here so it is visible rather than latent: under the
    // record's literal clause 4 (`v != null` is `null` for non-null v) the same blessed pattern
    // abstains when the axis IS present, so a row guarded exactly as ADR-0103 prescribes can
    // never fire. An erratum against ADR-0103 is OWED and is not written by this crate.
    assert_eq!(
        truth("novelty != null and novelty >= high", &present),
        Tri::Null,
        "the guarded pattern abstains on a PRESENT axis — the residual recorded above"
    );
}

/// `all` and `none` over `null` or an empty list are **true**; `any` is **false**.
#[test]
fn quantifier_empty_and_null() {
    let absent = Context::new().with("findings", Value::Null);
    let empty = Context::new().with("findings", Value::List(Vec::new()));

    for (label, ctx) in [("null", &absent), ("empty", &empty)] {
        assert_eq!(
            truth("findings.any(evidence == Reproduced)", ctx),
            Tri::False,
            "`any` over {label} is FALSE — ADR-0032's \"vacuously true\" is withdrawn for `any`"
        );
        assert_eq!(
            truth("findings.all(evidence == Reproduced)", ctx),
            Tri::True,
            "`all` over {label} is true"
        );
        // THE LOAD-BEARING ONE. `none` over an empty set is TRUE, which is exactly why
        // `findings.none(…)` is satisfiable by an empty finding set and why ADR-0080 forbids it as
        // a quality block's terminal condition. AC-527's rule rests on this value, and **the lint
        // that acts on it IS NOT BUILT** — `ac527_empty_set_terminal_condition` selects zero tests
        // on this crate. (This comment said "is Slice 2's" until 2026-09-13; Slice 2 did not build
        // it and reported it out of scope, so a later reader could take the forward reference for
        // a landed one. The slice that owns it is not named here, because no session has been
        // scoped to it.) What the rule will be on, when it exists, is the FORM and not the string:
        // ADR-0080's forbids clause reads "or any policy predicate satisfiable only by an empty
        // finding set", so `findings.all(…)` over an empty set fails it too.
        assert_eq!(
            truth("findings.none(evidence == Reproduced)", ctx),
            Tri::True,
            "`none` over {label} is TRUE — AC-527's whole premise"
        );
    }

    // The ADR's own named must-fail fixture: *"`any` over absent evidence must not pass an
    // activity."* The prefix alias ADR-0103 decision 3 accepts is exercised here, because it is
    // the form the record writes the fixture in.
    let no_surfaces = Context::new().with("surfaces", Value::Null);
    assert_eq!(
        truth("any(s in surfaces, s == \"auth\")", &no_surfaces),
        Tri::False,
        "`any` over an absent set must NOT fire the activity"
    );
    let surfaces = Context::new().with("surfaces", Value::set(["auth", "ui"]));
    assert_eq!(
        truth("any(s in surfaces, s == \"auth\")", &surfaces),
        Tri::True
    );
    assert_eq!(
        truth("all(s in surfaces, s == \"auth\")", &surfaces),
        Tri::False
    );

    // Non-empty method form, with the element's fields in scope — the positive that keeps the
    // empty-set rules above from being satisfied by a quantifier that always answers the same way.
    let reproduced = Context::new().with(
        "findings",
        Value::List(vec![record(
            "evidence",
            Value::Str("Reproduced".to_owned()),
        )]),
    );
    let absent_evidence = Context::new().with(
        "findings",
        Value::List(vec![record("evidence", Value::Str("Absent".to_owned()))]),
    );
    assert_eq!(
        truth(
            "findings.none(evidence in [Reproduced, MutationSurvived])",
            &reproduced
        ),
        Tri::False
    );
    assert_eq!(
        truth(
            "findings.none(evidence in [Reproduced, MutationSurvived])",
            &absent_evidence
        ),
        Tri::True
    );
    assert_eq!(truth("findings.len() == 1", &reproduced), Tri::True);

    // --- THE NON-EMPTY DISCRIMINATION, which every case above is blind to -------------------
    // Every list so far has ONE element, and over a singleton `any` and `all` are necessarily
    // equal — so `none` cannot be told apart from `not all`, and a `none` implemented as
    // `all.not()` (which answers TRUE for a list that DOES contain a match, as long as not every
    // element matches) passed this test until 2026-09-13. AC-527's whole premise leans on `none`,
    // so the mixed two-element list is written out: one match, one miss.
    let mixed = Context::new().with(
        "findings",
        Value::List(vec![
            record("evidence", Value::Str("Reproduced".to_owned())),
            record("evidence", Value::Str("Absent".to_owned())),
        ]),
    );
    assert_eq!(
        truth("findings.any(evidence == Reproduced)", &mixed),
        Tri::True,
        "`any` over a list CONTAINING a match is true"
    );
    assert_eq!(
        truth("findings.all(evidence == Reproduced)", &mixed),
        Tri::False,
        "`all` over a list containing a MISS is false"
    );
    assert_eq!(
        truth("findings.none(evidence == Reproduced)", &mixed),
        Tri::False,
        "`none` over a list CONTAINING a match is FALSE — it is `any` negated, never `all` \
         negated, and over a singleton the two are indistinguishable"
    );
    // The prefix alias, over the same shape, so the two spellings cannot drift apart.
    let two = Context::new().with("surfaces", Value::set(["auth", "ui"]));
    assert_eq!(truth("any(s in surfaces, s == \"auth\")", &two), Tri::True);
    assert_eq!(truth("all(s in surfaces, s == \"auth\")", &two), Tri::False);
    assert_eq!(
        truth("none(s in surfaces, s == \"auth\")", &two),
        Tri::False
    );

    // --- AC-079's accounting: once per READ facet, never per abstaining facet -------------
    // The context binds four abstaining axes; the predicate reads ONE of them, twice.
    let out = eval("novelty == high or novelty == low", &abstaining_context());
    assert_eq!(out.value, Tri::Null, "the row abstains");
    assert_eq!(
        out.abstention_axes(),
        vec!["novelty"],
        "ONE axis, counted once though read twice, and nothing for the three abstaining axes no \
         row read"
    );
    // A short-circuited conjunct reads nothing, so it opens nothing.
    let mut ctx = abstaining_context();
    ctx.bind("has_repro", Value::Bool(false));
    let out = eval("has_repro and novelty == high", &ctx);
    assert_eq!(out.value, Tri::False);
    assert!(
        out.abstention_axes().is_empty(),
        "a row that resolved to false opens no gate"
    );
    assert_eq!(
        out.read_facets
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["has_repro"],
        "the short-circuited conjunct's axis was never read"
    );
}

/// The commutativity of Kleene absorption is a READING of ADR-0103 decision 4, not its words.
///
/// Decision 4 (:71) gives each absorbing case in ONE order — *"`false and null` = false,
/// `true or null` = true, otherwise `null`"* — while spelling out BOTH orders two sentences earlier
/// for `==`. So `null and false` and `null or true` fall under "otherwise" on a literal reading and
/// under absorption on the standard three-valued one, and the two answers are different.
///
/// This is **not academic**: `spikes/p9/` carries
/// `"${rollback.outputs?} == null and (${proceed.answer?} == 'proceed' or rigor < mandatory)"`, whose
/// left operand is `null` whenever `rollback.outputs` is PRESENT. Under Kleene the conjunction can be
/// `false`, which step 4 may report as `Unsatisfiable`; under the literal reading it is `null`, which
/// decision 4 makes the **abstention** case. Same pack, two different outcome classes.
///
/// This test pins the reading the kernel implements so the choice is visible and reversible. **It is
/// evidence of a decision taken, not of a decision the record made** — a clarification against
/// ADR-0103 decision 4 is OWED (PROGRESS §2), and if it resolves the other way this test is the thing
/// that goes red and names the site.
#[test]
fn kleene_absorption_is_commutative_which_adr_0103_does_not_state() {
    let absent = abstaining_context();
    // `novelty` is DECLARED and null — the abstaining case — while the two booleans carry real
    // values. An undeclared facet is an `unknown field` error, which is a different rule entirely.
    let present = Context::new()
        .with("novelty", Value::Null)
        .with("has_repro", Value::Bool(false))
        .with("market_facing", Value::Bool(true));

    // The orders decision 4 STATES. These are not in doubt under either reading.
    assert_eq!(
        truth("has_repro and novelty == high", &present),
        Tri::False,
        "false and null = false (stated)"
    );
    assert_eq!(
        truth("market_facing or novelty == high", &present),
        Tri::True,
        "true or null = true (stated)"
    );

    // The orders decision 4 does NOT state. A literal reading makes both `Tri::Null`.
    assert_eq!(
        truth("novelty == high and has_repro", &present),
        Tri::False,
        "null and false: Kleene says false; a literal reading of \"otherwise null\" says null"
    );
    assert_eq!(
        truth("novelty == high or market_facing", &present),
        Tri::True,
        "null or true: Kleene says true; a literal reading of \"otherwise null\" says null"
    );

    // And the genuinely-otherwise case, which both readings agree is null.
    assert_eq!(
        truth("novelty == high and novelty == high", &absent),
        Tri::Null,
        "null and null = null"
    );
}

fn record(key: &str, value: Value) -> Value {
    let mut map = std::collections::BTreeMap::new();
    map.insert(key.to_owned(), value);
    Value::Record(map)
}
