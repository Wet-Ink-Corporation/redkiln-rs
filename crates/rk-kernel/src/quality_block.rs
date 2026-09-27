//! **AC-527**, amended 2026-09-25 to **ADR-0080 decision 2** — a quality block's terminal condition,
//! refused by its **form** and never by its string.
//!
//! # The rule, as the register now states it
//!
//! ADR-0080 decision 2: *"The predicate `findings.none(evidence in [Reproduced, MutationSurvived])` is
//! forbidden as a quality block's terminal condition. A policy step's terminal condition must be
//! decidable over something the reviewer varies."* Its bullet 1 is *"no finding at or above threshold T
//! survives re-execution"*, and that form is **true over an empty finding set**. So AC-527's earlier
//! clause refused what the ADR permits. ADR-0092 decision 2 item 1 sets the direction:
//! *"a criterion or a runbook step that contradicts an accepted ADR is the defect; the ADR is not"*.
//! The register was brought to d2 by a dated amendment on 2026-09-25 (D21-41 (h); `s8b` §8b.4), and
//! this module implements the amended THEN. It refuses two things, on **every** `check` step:
//!
//! 1. **The named form** — `findings.none(evidence in [...])`, in any quoting and with any member
//!    list, alone or as a conjunct required for truth ([`names_forbidden_form`]). It is matched on the
//!    **parse tree**, so a respelling that parses the same is caught and a comment is never read.
//! 2. **A conjunct that reads `findings` and that a single-class blocking witness falsifies**
//!    ([`blocking_witness`]; *dated 2026-09-25, s8b refuting round 5: D21-55's "that reads
//!    `findings`" scope restored — a findings-free conjunct is skipped, never refused*; *dated
//!    2026-09-25, s8b refuting round 6: pinned by a swap in
//!    `quality_block::d2_per_class_witness_refuses_one_member_rows`, mutant M-W3*). Each witness is
//!    a findings list of ONE finding carrying ONE field, `evidence`, in one blocking class —
//!    `Reproduced` or `MutationSurvived` — evaluated across the live rigor levels. The refusal is
//!    D21-55's: a predicate is refused if some such conjunct is `false` in some cell.
//!    *(dated 2026-09-26, `s8d` build, package P2; D21-67 ruling 3)* WIDENED to **"not `true` in some
//!    cell"**, matching the runtime `check` arm (`== Tri::True`): a `null` cell refuses as a `false`
//!    one does. An error is still not a value and stays indeterminate (D21-55 unchanged there), and
//!    "a `false` cell outranks an error" below reads "a not-`true` cell" from here on. Pinned by
//!    `quality_block::a_findings_conjunct_that_is_not_true_in_some_cell_is_refused` (mutant D67-5).
//!    *(dated
//!    2026-09-25, s8b refuting round 1)* That is all this measures. It does NOT establish that only
//!    a reviewer who found nothing can satisfy the predicate: `findings.any(evidence in
//!    ['Reproduced'])` is refused under the `MutationSurvived` witness, yet it is false over an
//!    empty set.
//!
//! # Per conjunct, and why nothing else is bound (D21-55)
//!
//! Refusal 2 is evaluated **per conjunct required for truth**. The walker flattens the predicate's
//! top-level `and` nodes, looking through grouping and not through `or` or `not`, and each conjunct
//! that reads `findings` is evaluated **alone**, over two bindings: `findings` as the witness and
//! `rigor` across [`LIVE_RIGOR_LEVELS`]. A conjunct that is `false` in some cell refuses the
//! predicate. This is ADR-0121 decision 1's absorption read as a rule about values: a false conjunct
//! makes the conjunction false whatever the operands left unevaluated turn out to be. Evaluating the
//! whole predicate instead fails on the first operand nothing binds — `and` computes its left side
//! first and propagates an `UnknownField` before absorption can act — so
//! `check.failed == 0 and findings.none(…)` would never reach its second conjunct.
//!
//! 🛑 **NO OTHER OPERAND IS BOUND, AND NULL-BINDING STAYS REJECTED.** Binding every read path to
//! `null` was tried under the earlier rule and measured wrong on 2026-09-14. `null` means *abstained*,
//! ADR-0103 decision 4 makes `all` and `none` over `null` **true**, and so every quantifier over an
//! unknown collection became vacuously true and the rule refused `mutation-testing`'s threshold and
//! `flag-progressive-rollout`'s re-execution outcome, which are d2's own remedies. A conjunct that
//! errors here — it reads something other than `findings` and `rigor`, or a `${…}` reference, which
//! the resolver owns — has **no truth value** and is counted indeterminate, never refused.
//! *(dated 2026-09-25, s8b refuting round 4)* "Errors" means errors in ANY witness cell, D21-55's
//! wording taken literally: a conjunct valued in some cells and erroring in others (an `or` whose
//! right side reads an unbound operand where its left side is false) is indeterminate, not decided,
//! unless some cell is `false`, which refuses it. Pinned by a swap in
//! `quality_block::d2_severity_only_predicates_load`. *(dated 2026-09-25, s8b refuting round 5)*
//! That swap has no `false` cell, so it pins only the error-plus-true half. The reading the code
//! takes where D21-55's two sentences meet — "a conjunct that errors is not refused" and "refused if
//! some such conjunct is `false` in some cell" — is that **a `false` cell outranks an error**, in
//! another cell of the same conjunct and in another conjunct. It is pinned by two swaps in
//! `quality_block::d2_per_class_witness_refuses_one_member_rows`, each REFUSED:
//! `not (findings.any(evidence in ['MutationSurvived']) or check.failed == 0)` errors in every
//! `Reproduced` cell (the `or` reads the unbound `check.failed`) and is `false` in every
//! `MutationSurvived` cell (mutant M-W1a: `cells` returns on its first error); and
//! `findings.none(severity in ['blocking']) and findings.len() == 0` has a first conjunct that errors
//! in every cell and a second that is `false` in every cell (mutant M-W1b: `blocking_witness`
//! returns on its first indeterminate conjunct). The rigor axis is pinned in the same test:
//! `rigor < mandatory or not findings.any(evidence in ['Reproduced'])` is `false` only at
//! `mandatory`, and is REFUSED naming it (mutant M-W2: the rigor loop visits one live level), and
//! `rigor != discouraged or findings.any(evidence in ['NarrowedPin'])` is `false` only at the
//! reserved `discouraged`, which the loop does not visit, and LOADS as decided.
//!
//! **The witness is per class, and the reason is measured.** In this evaluator `none` is `any.not()`,
//! so a joint `{Reproduced, MutationSurvived}` list also falsifies `findings.none(evidence in
//! ['Reproduced'])`. What a joint list does NOT falsify is a predicate that a single class alone
//! breaks, such as `findings.any(evidence in ['MutationSurvived']) or findings.none(evidence in
//! ['Reproduced'])`; the per-class fixture carries that shape (mutants M2 and M3). *(dated
//! 2026-09-25, s8b refuting round 1)* The fixture spells it `findings.any(evidence in
//! ['MutationSurvived']) or not findings.any(evidence in ['Reproduced'])`, the same predicate
//! without the named form's text, so the d2 guard's exclusions do not grow (D21-55's dated note).
//!
//! 🛑 **THE WITNESS CARRIES `evidence` ONLY.** Severity-only predicates read no `evidence` and stay
//! lawful: `findings.none(severity in ['blocking'])` over a witness looks `severity` up, falls through
//! to the context and errors, so it is indeterminate and loads. The witness is an untyped record for
//! that reason. It must never be built from a typed `Finding`, because once `s8h` adds `severity` to
//! `rk_types` `Finding` a typed witness would carry it and refuse every bullet-1 form (mutant M4).
//! *(dated 2026-09-25, s8b refuting round 2)* "Bullet-1 form" there means a severity-only
//! predicate. This rule only does not refuse one; whether a severity-only row without
//! re-execution conforms to d2 bullet 1 (*"no finding at or above threshold T survives
//! re-execution"*) is OPEN (D21-57).
//!
//! # WHAT IS MEASURED, AND WHAT IS ARMED
//!
//! | AC-527 clause (amended) | pinned here | by |
//! | --- | --- | --- |
//! | the named form, any quoting, any member list, alone or as a required conjunct | yes | [`names_forbidden_form`], fixtures 1, `names_any_member_list`, `is_one_member_conjunct` |
//! | a conjunct that reads `findings` and that a single-class blocking witness falsifies | yes | [`blocking_witness`], fixture 2 (recast) and the per-class fixture; the rigor axis and a `false` cell outranking an error, by swaps *(dated 2026-09-25, s8b refuting round 5)* |
//! | *"lint fails naming the activity and the predicate"* | yes | both are fields **and** both are in the `Display` |
//! | severity-only and verify-result predicates load | yes | `is_severity_only`, fixture 3 |
//! | the `rk-catalog` **CLI surface** | **NO — ARMED** | there is no `rk-catalog` lint; a kernel test over `.redkiln/catalog` is not that command |
//!
//! 🛑 **THE SUBJECT IS EVERY `check` STEP'S `expr`.** The `quality` shorthand ships with no default
//! (ADR-0080 decision 4) and no shipped activity carries it, so a rule keyed on it would enumerate
//! nothing. Disposition (i) licenses the path-free subject, and D21-42 11a took (i).
//!
//! 🛑 **A TERMINAL CONDITION THAT DOES NOT PARSE IS REFUSED, NEVER SKIPPED**
//! ([`TerminalConditionViolation::PredicateDoesNotParse`]): a rule that passed over what it could not
//! read would return `Ok` on a catalog of garbage.
//!
//! # Evidence
//!
//! ADR-0080 decision 2 [documented] forbids the predicate. F37 supplies the mechanism: **33 of 33**
//! adversary dispatches produced at least one finding in a blocking evidence class, so the forbidden
//! condition is satisfied only by a reviewer that does not review. The one-member rows refused here
//! **extend** F37's measurement, which was of the two-member predicate on one graph.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::catalog::{Catalog, StepKind};
use crate::predicate::{Context, EvalError, Expr, Quantifier, Tri, Value, evaluate, parse};

/// The five rigor levels, in order, as `crate::select::pack` closes them.
///
/// Carried whole because an ordinal compares **positions in its domain**: `rigor < mandatory` is
/// only meaningful against the full ladder, even where only three of its rungs are bound.
pub const RIGOR_DOMAIN: [&str; 5] = [
    "forbidden",
    "discouraged",
    "optional",
    "recommended",
    "mandatory",
];

/// The three rungs a Phase-1 pack may actually set.
///
/// `forbidden` admits nothing — nothing in that stage runs — and `discouraged` is **RESERVED**
/// (AC-050's third AND; `crate::select::pack::PackError::DiscouragedIsReserved`). A violation
/// witnessed only at one of those two would be witnessed at a level no pack may set, so the witness
/// loop does not visit them.
pub const LIVE_RIGOR_LEVELS: [&str; 3] = ["optional", "recommended", "mandatory"];

/// The path a policy step's findings arrive on.
///
/// Every live policy step in `spikes/p9/catalog/` at `P9_PIN` binds `findings = "${…outputs?}"`
/// and reads `findings` [measured 2026-09-14], and ADR-0080 decision 2 names the *"finding set"*.
/// A rule that guessed at a second spelling would be inventing one.
///
/// *(dated 2026-09-25, s8b refuting round 3)* That premise no longer holds over `.redkiln/catalog`:
/// after `s8b`'s d2 rewrites, `root-cause`, `differential-debugging` and
/// `instrumented-trace-analysis` read a re-execution outcome (`report.failed`), `user-story-ac` and
/// `vertical-slicing` read `check.failed`, and `adversarial-review`'s `verdict` reads
/// `record.blocking` and does not bind `findings`. Those steps are outside this rule's subject by
/// D21-55's `reads findings` gate ([`walk`] skips them). The subject stays keyed on this name by
/// ruling, not because every policy step reads it.
///
/// *(dated 2026-09-25, s8b refuting round 4)* The p9 sentence above was false at `P9_PIN` too, for
/// one row, and round 3 misfiled that row: `docs-update`'s policy binds `lint` and `check` and reads
/// `lint.errors.len() == 0 and check.failed == 0` in `spikes/p9/catalog` as in `.redkiln/catalog`.
/// It was never a findings reader, and `s8b`'s d2 rewrites did not touch it. Read the sentence as
/// "every live policy step at `P9_PIN` except `docs-update`'s" [measured 2026-09-25: of the policy
/// steps in `git ls-tree ca8f387 spikes/p9/catalog/`, only `docs-update`'s `expr` reads no
/// `findings`].
///
/// *(dated 2026-09-25, s8b refuting round 6)* The binding clause of that sentence was never
/// measured and is wrong as written: the p9 policy steps bind `findings` to `${<reviewer>.outputs}`
/// or `${<reviewer>.outputs?}`, optional or not — twelve of them (among them `adversarial-review`,
/// `code-review`, `security-review` and `threat-model`) bind `${review.outputs}` or
/// `${adversary.outputs}` with no `?` — and `docs-update` binds none. The measured claim is the
/// reads claim: every live policy step at `P9_PIN` except `docs-update`'s reads `findings`.
pub const FINDINGS: &str = "findings";

/// The rigor axis, which is never absent at evaluation time — it is selection's own input.
pub const RIGOR: &str = "rigor";

/// The field a finding's evidence class is read from, in the catalog's own spelling.
pub const EVIDENCE: &str = "evidence";

/// The two blocking evidence classes ADR-0080 decision 2 names, one witness each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockingClass {
    Reproduced,
    MutationSurvived,
}

impl BlockingClass {
    /// Both classes, in the order the witness loop visits them.
    pub const ALL: [Self; 2] = [Self::Reproduced, Self::MutationSurvived];

    /// The class as the catalog spells it in `evidence in [...]`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reproduced => "Reproduced",
            Self::MutationSurvived => "MutationSurvived",
        }
    }

    /// The witness: a findings list of one finding carrying ONE field, `evidence`, in this class.
    #[must_use]
    pub fn witness(self) -> Value {
        Value::List(vec![Value::Record(BTreeMap::from([(
            EVIDENCE.to_owned(),
            Value::Str(self.as_str().to_owned()),
        )]))])
    }
}

impl fmt::Display for BlockingClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The cell under which a conjunct came out `false`: which blocking class, at which rigor.
/// *(dated 2026-09-26, `s8d` build; D21-67 ruling 3: or `null` — any value that is not `true`.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub class: BlockingClass,
    pub rigor: &'static str,
    /// The value the conjunct took in that cell: `False`, or since D21-67 ruling 3 `Null` (*"not
    /// `true` in some cell"*). *(dated 2026-09-26, `s8d` build, package P2.)*
    pub value: Tri,
    /// The falsified conjunct, rendered with `Debug` over its parse tree (see `render`): a
    /// structural rendering, not the author's spelling. *(dated 2026-09-25, s8b refuting round 1:
    /// it is not rendered from a source span.)*
    pub conjunct: String,
}

/// What AC-527, as amended, refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalConditionViolation {
    /// Refusal 1: `findings.none(evidence in [...])`, alone or as a conjunct required for truth.
    NamesForbiddenForm {
        activity: String,
        step: String,
        predicate: String,
    },
    /// Refusal 2: a conjunct required for truth is `false` under a one-finding witness of one
    /// blocking class, at some live rigor. *(dated 2026-09-26, `s8d` build; D21-67 ruling 3)* Is
    /// NOT `true`: `false` or `null`.
    FalsifiedByBlockingWitness {
        activity: String,
        step: String,
        predicate: String,
        witness: Witness,
    },
    /// A terminal condition outside ADR-0103's grammar. **Fail closed.**
    PredicateDoesNotParse {
        activity: String,
        step: String,
        predicate: String,
        detail: String,
    },
}

impl TerminalConditionViolation {
    /// The activity id AC-527's THEN requires the failure to name.
    #[must_use]
    pub fn activity(&self) -> &str {
        match self {
            Self::NamesForbiddenForm { activity, .. }
            | Self::FalsifiedByBlockingWitness { activity, .. }
            | Self::PredicateDoesNotParse { activity, .. } => activity,
        }
    }

    /// The `[[step]]` the terminal condition sits on.
    #[must_use]
    pub fn step(&self) -> &str {
        match self {
            Self::NamesForbiddenForm { step, .. }
            | Self::FalsifiedByBlockingWitness { step, .. }
            | Self::PredicateDoesNotParse { step, .. } => step,
        }
    }

    /// The predicate AC-527's THEN requires the failure to name, verbatim.
    #[must_use]
    pub fn predicate(&self) -> &str {
        match self {
            Self::NamesForbiddenForm { predicate, .. }
            | Self::FalsifiedByBlockingWitness { predicate, .. }
            | Self::PredicateDoesNotParse { predicate, .. } => predicate,
        }
    }

    /// A stable token per clause, for a test that asserts *which* rule fired without matching prose.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::NamesForbiddenForm { .. } => "names_forbidden_form",
            Self::FalsifiedByBlockingWitness { .. } => "falsified_by_blocking_witness",
            Self::PredicateDoesNotParse { .. } => "predicate_does_not_parse",
        }
    }
}

impl fmt::Display for TerminalConditionViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NamesForbiddenForm {
                activity,
                step,
                predicate,
            } => write!(
                f,
                "{activity}/{step}: the terminal condition `{predicate}` carries ADR-0080 decision \
                 2's forbidden form — `none` over the findings set, tested on its evidence class — as \
                 a conjunct required for truth. ADR-0080 decision 2: a terminal condition must be \
                 decidable over something the reviewer varies — a threshold over severity or \
                 confidence (\"no finding at or above threshold T survives re-execution\", not \"no \
                 finding exists\"), or a different quantity entirely: a bounded loop-back count, a \
                 re-execution outcome, or a delta against the previous iteration's findings"
            ),
            // *(dated 2026-09-25, s8b refuting round 4)* The message quotes d2's own words. It read
            // "a threshold over severity", dropping "or confidence" and the re-execution clause, which
            // recommended a bare severity threshold, the row shape D21-57 left OPEN.
            Self::FalsifiedByBlockingWitness {
                activity,
                step,
                predicate,
                witness,
            } => write!(
                f,
                "{activity}/{step}: the terminal condition `{predicate}` has a conjunct, `{}`, that \
                 is not TRUE ({:?}) under a one-finding `{}` witness (rigor `{}`). ADR-0080 \
                 decision 2, as D21-55 applies it and D21-67 ruling 3 widens it, refuses a terminal \
                 condition with such a conjunct",
                witness.conjunct, witness.value, witness.class, witness.rigor
            ),
            Self::PredicateDoesNotParse {
                activity,
                step,
                predicate,
                detail,
            } => write!(
                f,
                "{activity}/{step}: the terminal condition `{predicate}` is outside ADR-0103's \
                 grammar and cannot be evaluated: {detail}. It is refused rather than skipped — a \
                 rule that passed over what it could not read would return Ok on a catalog of \
                 garbage"
            ),
        }
    }
}

impl std::error::Error for TerminalConditionViolation {}

/// What the rule walked, so that a collapse cannot read like a clean catalog.
///
/// `refused + decided + indeterminate == reads_findings`: every step whose predicate reads the
/// findings set is in exactly one of the three. A step that reads no findings is walked and parsed
/// and evaluated by nothing here — the rule has no subject in it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerminalConditionReport {
    /// Activities enumerated.
    pub activities_walked: usize,
    /// `check` steps carrying an `expr`.
    pub check_steps_walked: usize,
    /// Terminal conditions that parsed.
    pub predicates_parsed: usize,
    /// Of those, the ones with at least one conjunct that reads `findings`.
    pub reads_findings: usize,
    /// Refused, one per step whichever refusal fired.
    pub refused: usize,
    /// Not refused, and every findings-reading conjunct yielded a truth value in every cell.
    /// *(dated 2026-09-25, s8b refuting round 4: was "in some cell"; see `indeterminate`.)*
    pub decided: usize,
    /// Not refused, and some findings-reading conjunct errored in at least one cell — it reads an
    /// operand nothing here binds (a `severity` field no witness carries, a `${…}` reference), in
    /// every cell or only in those an `or` does not short-circuit. D21-55: *"A conjunct that errors
    /// is not refused; it is counted as indeterminate."* *(dated 2026-09-25, s8b refuting round 4:
    /// was "yielded no truth value in any cell", which the old code matched and D21-55 does not.)*
    pub indeterminate: usize,
}

/// The conjuncts required for the predicate's truth: its top-level `and` operands, through grouping,
/// and never through `or` or `not`.
#[must_use]
pub fn conjuncts(expr: &Expr) -> Vec<&Expr> {
    match expr {
        Expr::And(l, r) => {
            let mut out = conjuncts(l);
            out.extend(conjuncts(r));
            out
        }
        Expr::Group(inner) => conjuncts(inner),
        other => vec![other],
    }
}

fn ungroup(mut e: &Expr) -> &Expr {
    while let Expr::Group(inner) = e {
        e = inner;
    }
    e
}

/// Refusal 1, on the parse tree: some conjunct required for truth is `findings.none(evidence in
/// [...])` — method form or bound-variable form, with any member list.
#[must_use]
pub fn names_forbidden_form(expr: &Expr) -> bool {
    conjuncts(expr).into_iter().any(|c| {
        let Expr::Quant {
            quantifier: Quantifier::None,
            var,
            source,
            body,
        } = ungroup(c)
        else {
            return false;
        };
        if !matches!(ungroup(source), Expr::Path(p) if p == FINDINGS) {
            return false;
        }
        let Expr::In { left, list } = ungroup(body) else {
            return false;
        };
        let field = match var {
            Some(v) => format!("{v}.{EVIDENCE}"),
            None => EVIDENCE.to_owned(),
        };
        matches!(ungroup(left), Expr::Path(p) if *p == field)
            && matches!(ungroup(list), Expr::List(_))
    })
}

/// Every context path the expression reads.
fn read_paths(expr: &Expr, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Path(p) => {
            out.insert(p.clone());
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => read_paths(inner, out),
        Expr::Or(l, r) | Expr::And(l, r) => {
            read_paths(l, out);
            read_paths(r, out);
        }
        Expr::Cmp { left, right, .. } => {
            read_paths(left, out);
            read_paths(right, out);
        }
        Expr::In { left, list } => {
            read_paths(left, out);
            read_paths(list, out);
        }
        Expr::Member { literal, set } => {
            read_paths(literal, out);
            read_paths(set, out);
        }
        Expr::List(items) => {
            for item in items {
                read_paths(item, out);
            }
        }
        Expr::Quant { source, body, .. } => {
            read_paths(source, out);
            read_paths(body, out);
        }
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

/// `true` when the predicate reads the findings set at all.
#[must_use]
pub fn reads_findings(expr: &Expr) -> bool {
    let mut paths = BTreeSet::new();
    read_paths(expr, &mut paths);
    paths
        .iter()
        .any(|p| p == FINDINGS || p.starts_with("findings."))
}

/// What one conjunct said across the witness cells.
enum Cells {
    /// Some cell was `false` — *(dated 2026-09-26, `s8d` build; D21-67 ruling 3)* or, since the
    /// widening, `null`: some cell was not `true`. The value is carried.
    Falsified(BlockingClass, &'static str, Tri),
    /// Every cell yielded a truth value, and none was `false`.
    Valued,
    /// No cell was `false`, and at least one yielded no truth value. *(dated 2026-09-25, s8b refuting
    /// round 4: was "no cell yielded a truth value". D21-55 reads "a conjunct that errors is not
    /// refused; it is counted as indeterminate", so an error in ANY cell, with no `false` cell, is
    /// indeterminate, not only an error in every cell.)*
    NoValue(EvalError),
}

// M2/M3: the witness set — one list per blocking class, each of ONE finding.
fn witnesses() -> Vec<(BlockingClass, Value)> {
    BlockingClass::ALL
        .iter()
        .map(|c| (*c, c.witness()))
        .collect()
}

fn cells(conjunct: &Expr) -> Cells {
    let mut first_error = None;
    for (class, list) in witnesses() {
        // M-W2: every live rigor level is a cell (D21-55's "`rigor` across the live levels").
        for rigor in LIVE_RIGOR_LEVELS {
            // TWO BINDINGS AND NO MORE (D21-55).
            let mut ctx = Context::new();
            ctx.bind(FINDINGS, list.clone());
            match Value::ordinal(&RIGOR_DOMAIN, rigor) {
                Ok(v) => ctx.bind(RIGOR, v),
                Err(e) => return Cells::NoValue(e),
            }
            match evaluate(conjunct, &ctx) {
                Ok(outcome) => {
                    // *(dated 2026-09-26, `s8d` build, package P2; D21-67 ruling 3)* WIDENED from
                    // `== Tri::False` to any evaluated value that is not `Tri::True`, matching the
                    // runtime `check` arm. An `Err` is not a value and stays indeterminate (D21-55).
                    // D67-5: the refusal restored to `== Tri::False`.
                    if outcome.value != Tri::True {
                        return Cells::Falsified(class, rigor, outcome.value);
                    }
                }
                // M-W1a: an erroring cell does not stop the loop; a later `false` cell refuses.
                Err(e) => first_error = first_error.or(Some(e)),
            }
        }
    }
    // A `false` cell has already returned; any error left makes the conjunct indeterminate (D21-55).
    first_error.map_or(Cells::Valued, Cells::NoValue)
}

/// Refusal 2: the first findings-reading conjunct a single-class blocking witness falsifies.
///
/// The witness's conjunct is rendered with `Debug` over the parse tree, not from the predicate's
/// source. *(dated 2026-09-25, s8b refuting round 1: this function takes no `src`.)*
///
/// # Errors
///
/// [`EvalError`] when some findings-reading conjunct errored in at least one cell and none was
/// falsified: the predicate is indeterminate here, which is never a refusal. *(dated 2026-09-25, s8b
/// refuting round 4: was "yielded no truth value in any cell"; D21-55 counts a conjunct that errors
/// as indeterminate, and a conjunct valued in some cells and erroring in others errors.)*
pub fn blocking_witness(expr: &Expr) -> Result<Option<Witness>, EvalError> {
    let mut indeterminate = None;
    for c in conjuncts(expr) {
        // M-W3: only a conjunct that reads `findings` is evaluated (D21-55); a findings-free conjunct
        // is skipped, never refused, even where it is `false` in some cell.
        if !reads_findings(c) {
            continue;
        }
        match cells(c) {
            Cells::Falsified(class, rigor, value) => {
                return Ok(Some(Witness {
                    class,
                    rigor,
                    value,
                    conjunct: render(c),
                }));
            }
            Cells::Valued => {}
            // M-W1b: an indeterminate conjunct does not stop the walk; a later falsified one refuses.
            Cells::NoValue(e) => indeterminate = indeterminate.or(Some(e)),
        }
    }
    indeterminate.map_or(Ok(None), Err)
}

/// A conjunct, rendered for a message. The grammar has no printer, so this is a structural rendering
/// and not the author's spelling; the whole predicate is always quoted verbatim beside it.
fn render(e: &Expr) -> String {
    format!("{e:?}")
}

/// Walk a catalog, returning what was seen **and** what is refused.
#[must_use]
pub fn walk(catalog: &Catalog) -> (TerminalConditionReport, Vec<TerminalConditionViolation>) {
    let mut report = TerminalConditionReport {
        activities_walked: catalog.len(),
        ..TerminalConditionReport::default()
    };
    let mut violations = Vec::new();

    for activity in catalog.activities() {
        for step in &activity.steps {
            if step.kind != StepKind::Check {
                continue;
            }
            let Some(src) = step.expr.as_deref() else {
                continue;
            };
            report.check_steps_walked += 1;
            let parsed = match parse(src) {
                Ok(p) => p,
                Err(e) => {
                    violations.push(TerminalConditionViolation::PredicateDoesNotParse {
                        activity: activity.id.clone(),
                        step: step.id.clone(),
                        predicate: src.to_owned(),
                        detail: e.to_string(),
                    });
                    continue;
                }
            };
            report.predicates_parsed += 1;
            if !conjuncts(&parsed.expr).into_iter().any(reads_findings) {
                continue;
            }
            report.reads_findings += 1;
            // One refusal per step: the named form is reported when present, the witness otherwise.
            // M5: the named-form match.
            if names_forbidden_form(&parsed.expr) {
                report.refused += 1;
                violations.push(TerminalConditionViolation::NamesForbiddenForm {
                    activity: activity.id.clone(),
                    step: step.id.clone(),
                    predicate: src.to_owned(),
                });
                continue;
            }
            match blocking_witness(&parsed.expr) {
                Ok(Some(witness)) => {
                    report.refused += 1;
                    violations.push(TerminalConditionViolation::FalsifiedByBlockingWitness {
                        activity: activity.id.clone(),
                        step: step.id.clone(),
                        predicate: src.to_owned(),
                        witness,
                    });
                }
                Ok(None) => report.decided += 1,
                Err(_) => report.indeterminate += 1,
            }
        }
    }
    (report, violations)
}

/// **AC-527**, as amended, over a catalog.
///
/// # Errors
///
/// **Every** violation, never the first: a rule that returned on the first defect would hide the
/// second.
pub fn quality_block_terminal_condition_rule(
    catalog: &Catalog,
) -> Result<TerminalConditionReport, Vec<TerminalConditionViolation>> {
    let (report, violations) = walk(catalog);
    if violations.is_empty() {
        Ok(report)
    } else {
        Err(violations)
    }
}
