//! **AC-527** — the quality block's terminal condition, and a rule on the **FORM** rather than on
//! the string.
//!
//! # What the criterion says, and why the obvious implementation is the wrong one
//!
//! AC-527: *"GIVEN an activity whose quality block's terminal condition is
//! `findings.none(evidence in [Reproduced, MutationSurvived])`, **or any predicate whose truth over
//! an empty findings set is `true`** / WHEN lint runs / THEN lint fails naming the activity and the
//! predicate — a quality block terminates on a positive evidence record (a `GreenAt(sha)` verify
//! result, a reproduced-and-flipped finding set, a human gate answer), never on the absence of
//! findings."*
//!
//! 🛑 **A lint that matched the forbidden STRING would satisfy two of the criterion's three
//! fixtures and still be wrong.** Its *Verified by* names three: the verbatim predicate, which must
//! fail; **`findings.all(…)` over an empty set, which must also fail**; and a positive terminating
//! on a verify result, which must load. The middle one shares no text with the first, so a spelling
//! denylist passes the first and the third and admits the defect. §7b.13:1047-1051 says so in
//! terms: *"the rule is on the **form**, not the string"*.
//!
//! So [`no_findings_witness`] **evaluates** the candidate. ADR-0103 decision 4 — pinned by
//! `tests/it/evaluator.rs::quantifier_empty_and_null`, not restated here — makes `all` and `none`
//! over `null` or an empty list **true** while `any` is **false**. One evaluation therefore reaches
//! `none`, `all`, `findings == null`, `findings.len() == 0` and every spelling nobody has thought of
//! yet, and reaches none of `any`, a threshold, or a verify result.
//!
//! # The context the candidate is evaluated over, and why each binding is what it is
//!
//! `findings` is the only thing the criterion fixes, and it fixes it **twice**: F37 measured the
//! policy check *"30 pass / 0 false in the `optional` arm (where no adversary runs, so it is
//! vacuously true)"* and `0 pass / 30 false` in `recommended`. In the first arm the step's input is
//! `${adversary.outputs?}` and is **absent**; in the second it is a list. [`FindingsWitness`] carries
//! both shapes and both are refused, because AC-527's THEN ends on *"never on the **absence** of
//! findings"* and `none` over `null` and over `[]` are the same `true`.
//!
//! **`rigor` is bound across the three LIVE levels and is not left null**, and this is the one
//! binding that is a reading rather than a transcription. `spikes/p9/catalog/adversarial-review.toml`
//! :41 carries `findings.none(…) and (rigor < mandatory or findings.none(…))` — the record's own
//! forbidden predicate, conjoined with an ordering. An ordering against a `null` operand is
//! [`crate::predicate::EvalError::OrderingAgainstNull`] (ADR-0069 forbids evaluating it with a
//! default ordering), so a rule that bound `rigor` to `null` would **error** on that predicate and
//! let the single most on-the-nose live instance through. `rigor` is never absent at evaluation
//! time — it is selection's input — so binding it is not inventing a fact. The three levels are
//! [`LIVE_RIGOR_LEVELS`]: `forbidden` admits nothing and `discouraged` is **RESERVED** in Phase 1
//! (AC-050; decision #20 of 2026-09-11), so a violation witnessed only at one of those two would be
//! witnessed at a level no pack may set. A predicate true at **any** live level is refused: a block
//! that terminates on nothing at `optional` is forbidden whatever it does at `mandatory`.
//!
//! 🛑 **EVERY OTHER PATH IS LEFT UNBOUND, AND BINDING THEM TO `null` WAS TRIED FIRST AND MEASURED
//! WRONG.** An unbound path is [`crate::predicate::EvalError::UnknownField`] — not an abstention —
//! so a predicate that reads one has **no truth value** here and is counted as indeterminate rather
//! than refused. The tempting alternative is to bind every read path to `null`, and it is a
//! **different claim**: `null` means *"this facet abstained"*, and ADR-0103 decision 4 makes
//! `X.all(…)` and `X.none(…)` over `null` **true**. So binding an unknown collection to `null`
//! silently asserts that the collection is empty, and every quantifier over it becomes vacuously
//! true.
//!
//! **What that cost, measured 2026-09-14 over `spikes/p9/catalog/` from inside
//! `ac527_empty_set_terminal_condition` itself**: the null-binding version refused **13** predicates,
//! and **two of them read no findings set at all** — `flag-progressive-rollout`'s
//! `logs.all(status == 'ok')` and `mutation-testing`'s `reports.all(score >= 0.8)`. Those two are a
//! **re-execution outcome** and a **threshold**, which are the first and second of the shapes
//! ADR-0080 decision 2 *prescribes as the remedy*. A rule that refuses the record's own repair is
//! worse than no rule, and no fixture in AC-527's set would have shown it — only the shipped
//! catalog did. With the binding corrected the count is **11**, and every one of the eleven reads
//! `findings`.
//!
//! # WHAT IS MEASURED, AND WHAT IS ARMED
//!
//! | AC-527 clause | pinned here | by |
//! | --- | --- | --- |
//! | the verbatim predicate fails | yes | [`TerminalConditionViolation::SatisfiedByNoFindings`], over the fixture **and** over `spikes/p9/catalog/` |
//! | *"any predicate whose truth over an empty findings set is `true`"* | yes | [`no_findings_witness`], six cells per predicate |
//! | *"naming the activity and the predicate"* | yes | both are fields **and** both are in the `Display` |
//! | the positive loads | yes | the rule is `Ok` on a terminal condition that is not `true` over no findings |
//! | the `rk-catalog` **CLI surface** | **NO — ARMED** | there is no `rk-catalog` in this workspace; the CLI is Step 7d's and the catalog repairs are Step 7e's |
//!
//! 🛑 **AC-527's THEN AND ADR-0080 DECISION 2 ARE NOT THE SAME SENTENCE, AND THIS MODULE
//! IMPLEMENTS THE CRITERION'S.** ADR-0080 decision 2 closes with *"Lint rejects a policy predicate
//! whose **satisfaction requires** an empty finding set"*, which §7b.13 restates as *"satisfiable
//! only by an empty finding set"*. AC-527's THEN asks a different question: whether the predicate's
//! **truth over an empty findings set is `true`**. The first is a satisfiability search over every
//! operand; the second is one evaluation. They part company on a conjunction — the shipped
//! `check.failed == 0 and findings.none(severity in ['blocking'])` has no truth value under AC-527
//! (nothing binds `check.failed`) and *is* satisfied by an empty finding set whenever the other
//! conjunct holds. **This module implements AC-527's sentence and COUNTS the gap**:
//! [`TerminalConditionReport::indeterminate`] is the number of predicates the difference hides —
//! **83** on the shipped catalog, **35** of which read the findings set and would be candidates
//! under the broader reading — and `ac527_empty_set_terminal_condition` asserts it so it is a
//! figure and not a worry. **An erratum reconciling the two is OWED and is not written here**
//! (`00-preamble.md`:55) — deciding for the broader reading would change which live activities are
//! refused, and that is a decision, not an execution order.
//!
//! 🛑 **THE SUBJECT IS EVERY `check` STEP'S `expr`, AND THE `quality` SHORTHAND HAS NO SUBJECT.**
//! The activity-graph schema §9 lists `quality = { adversary = <role>, policy = <policy-id>, gate =
//! <rigor> }` among the shorthands *"to add **after P7**"*, and **ADR-0080 decision 4 ships it with
//! NO default** — *"every activity that wants the block writes it"*. Measured 2026-09-14: **no
//! activity in `spikes/p9/catalog/` carries a `quality` key** (`grep -rlE '^ *quality *=' ` matches
//! nothing), so a rule keyed on the shorthand would enumerate zero and pass, which is Halt 8's
//! *"a filter that matches nothing exits 0"* one level up. The schema's policy step **is** the
//! `check` step, so that is what is walked. The over-reach a reader would expect from this is not
//! there, and the reason is the form rule itself. Measured 2026-09-14 by this rule over
//! `spikes/p9/catalog/`: **94** live terminal conditions on **102** activities, of which **46**
//! read `findings` at all and **11** are refused. The other **83** have no truth value here — they
//! read something this rule does not bind — and not one of the 94 is **decided lawful**. That last
//! figure is the honest measure of this rule's reach on the product artefact, and it is asserted by
//! number in `ac527_empty_set_terminal_condition` rather than left for a reader to infer from a
//! green.
//!
//! 🛑 **AN EVALUATION THAT ERRORS IS NOT REPORTED AS A VIOLATION, AND THE RULE THAT OWNS IT IS
//! NAMED.** A predicate that errors in every cell — an ordering against an operand no binding here
//! supplies, or a `${…}` reference, which is the selection function's to resolve — is counted in
//! [`TerminalConditionReport::errored`] and not refused. It cannot be *"`true` over an empty
//! findings set"* if it has no truth value at all, and the ordering case is already a **load-time
//! lint error** under ADR-0069 / AC-069 (`crate::predicate::lint`), which is a different rule with
//! its own fixtures. What is **not** silently admitted is a predicate that does not **parse**: that
//! is [`TerminalConditionViolation::PredicateDoesNotParse`], because a rule that skipped what it
//! could not read would return `Ok` on a catalog of garbage.
//!
//! # Evidence
//!
//! ADR-0080 decision 2 [documented] forbids the predicate. **F29** is why the empty set is not
//! evidence in the other direction either: over 60 draws a **reproduced** regression was approved
//! **3 of 10 times** (class mean: `Reproduced` 0.70 rejected, against `MutationSurvived` 0.37,
//! `NarrowedPin` 0.10 and a control of 0.00), with **19/19 rejections on target** — so "no finding
//! survived" and "the artefact is sound" are different propositions and neither implies the other.
//! F37 supplies the mechanism: **33 of 33** adversary dispatches produced at least one finding in a
//! blocking evidence class and **none** produced zero, so the forbidden condition is satisfied only
//! by a reviewer that does not review.

use std::collections::BTreeSet;
use std::fmt;

use crate::catalog::{Catalog, StepKind};
use crate::predicate::{Context, EvalError, Expr, Tri, Value, evaluate, parse};

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
/// Every live policy step in `spikes/p9/catalog/` binds `inputs = { findings = "${adversary.
/// outputs?}" }` and reads `findings` [measured 2026-09-14], and AC-527's own sentence names the
/// *"findings set"*. A rule that guessed at a second spelling would be inventing one.
pub const FINDINGS: &str = "findings";

/// The rigor axis, which is never absent at evaluation time — it is selection's own input.
pub const RIGOR: &str = "rigor";

/// Which shape of *"no findings"* witnessed the violation.
///
/// **Both, because F37 measured both.** The `recommended` arm's adversary produced a list; the
/// `optional` arm ran no adversary at all, so `${adversary.outputs?}` was **absent** and the check
/// was *"vacuously true"* 30 times. ADR-0103 decision 4 gives `none` and `all` the same `true` over
/// either, and AC-527's THEN closes on *"never on the **absence** of findings"*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingsWitness {
    /// `findings` bound to the empty list.
    Empty,
    /// `findings` absent — the `optional`-arm shape.
    Absent,
}

impl fmt::Display for FindingsWitness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("an empty findings set"),
            Self::Absent => f.write_str("an absent findings set"),
        }
    }
}

/// The cell under which the predicate came out `true` with no findings.
///
/// Carried rather than discarded so the violation can say *which* emptiness and *which* rigor: a
/// message that says only "this is forbidden" leaves an author guessing at the repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub findings: FindingsWitness,
    pub rigor: &'static str,
}

/// What AC-527 refuses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerminalConditionViolation {
    /// *"any predicate whose truth over an empty findings set is `true`"* — the rule.
    SatisfiedByNoFindings {
        activity: String,
        step: String,
        predicate: String,
        witness: Witness,
    },
    /// A terminal condition outside ADR-0103's grammar.
    ///
    /// **Fail closed.** This is not AC-527's clause, and it is here because the alternative is:
    /// skip what cannot be parsed, and return `Ok` on a catalog whose every policy predicate is
    /// unreadable. The grammar itself is `crate::predicate`'s and its own rules are AC-069's.
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
            Self::SatisfiedByNoFindings { activity, .. }
            | Self::PredicateDoesNotParse { activity, .. } => activity,
        }
    }

    /// The `[[step]]` the terminal condition sits on.
    #[must_use]
    pub fn step(&self) -> &str {
        match self {
            Self::SatisfiedByNoFindings { step, .. } | Self::PredicateDoesNotParse { step, .. } => {
                step
            }
        }
    }

    /// The predicate AC-527's THEN requires the failure to name, verbatim.
    #[must_use]
    pub fn predicate(&self) -> &str {
        match self {
            Self::SatisfiedByNoFindings { predicate, .. }
            | Self::PredicateDoesNotParse { predicate, .. } => predicate,
        }
    }

    /// A stable token per clause, for a test that asserts *which* rule fired without matching prose.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::SatisfiedByNoFindings { .. } => "terminates_on_no_findings",
            Self::PredicateDoesNotParse { .. } => "predicate_does_not_parse",
        }
    }
}

impl fmt::Display for TerminalConditionViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SatisfiedByNoFindings {
                activity,
                step,
                predicate,
                witness,
            } => write!(
                f,
                "{activity}/{step}: the terminal condition `{predicate}` is TRUE over {} at rigor \
                 `{}`, so the quality block terminates on the absence of findings. ADR-0080 \
                 decision 2 forbids that: a terminal condition must be decidable over something the \
                 reviewer varies — a threshold over severity or confidence, a bounded loop-back \
                 count, a re-execution outcome, or a delta against the previous iteration",
                witness.findings, witness.rigor
            ),
            Self::PredicateDoesNotParse {
                activity,
                step,
                predicate,
                detail,
            } => write!(
                f,
                "{activity}/{step}: the terminal condition `{predicate}` is outside ADR-0103's \
                 grammar and cannot be evaluated over an empty findings set: {detail}. It is \
                 refused rather than skipped — a rule that passed over what it could not read would \
                 return Ok on a catalog of garbage"
            ),
        }
    }
}

impl std::error::Error for TerminalConditionViolation {}

/// What the rule walked, so that a collapse cannot read like a clean catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TerminalConditionReport {
    /// Activities enumerated.
    pub activities_walked: usize,
    /// `check` steps carrying an `expr`.
    pub check_steps_walked: usize,
    /// Terminal conditions that parsed.
    pub predicates_parsed: usize,
    /// Of those, the ones that read `findings` at all. The rule reaches no other kind, and this is
    /// the number that says so.
    pub reads_findings: usize,
    /// Refused — equal to the violation count for [`TerminalConditionViolation::SatisfiedByNoFindings`].
    pub refused: usize,
    /// `false` in every cell: decided, and lawful.
    pub decided: usize,
    /// `null` in some cell and `true` in none — the gap between AC-527's THEN and ADR-0080
    /// decision 2's *"satisfiable only by an empty finding set"*.
    pub undecided: usize,
    /// Errored in every cell it was evaluated in: an ordering AC-069 owns, or a `${…}` reference
    /// the selection function resolves. **Never refused here**, and never silently lawful either.
    pub errored: usize,
}

impl TerminalConditionReport {
    /// Predicates the rule parsed and could not decide, either way.
    #[must_use]
    pub const fn indeterminate(&self) -> usize {
        self.undecided + self.errored
    }
}

/// Every context path the expression reads, so each can be bound before evaluation.
///
/// An unbound path is [`EvalError::UnknownField`] and **not** an abstention
/// (`crate::predicate::eval`), so this walk is what makes the evaluation possible at all. Paths
/// that are a quantifier's bound variable or an element field are collected too and bound to
/// `null`; the evaluator's own scope frames shadow them whenever a body actually runs.
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

/// AC-527's form test: is this predicate `true` over **no findings**?
///
/// Six cells — the two [`FindingsWitness`] shapes across the three [`LIVE_RIGOR_LEVELS`]. The first
/// `true` wins and is returned as the witness; `Ok(None)` means no cell was `true`.
///
/// # Errors
///
/// [`EvalError`] when **no** cell yielded a truth value. The common case is
/// [`EvalError::UnknownField`] — the predicate reads something other than `findings` and `rigor`,
/// and this rule binds nothing else; the others are an ordering ADR-0069 forbids evaluating against
/// an absent operand (AC-069's rule, not this one) and a `${…}` reference, which the selection
/// function resolves and this rule never does. **A predicate that errors is not refused**: it
/// cannot be *"`true` over an empty findings set"* if it has no truth value at all.
pub fn no_findings_witness(expr: &Expr) -> Result<Option<Witness>, EvalError> {
    match verdict(expr) {
        Verdict::Refused(w) => Ok(Some(w)),
        Verdict::Decided | Verdict::Undecided => Ok(None),
        Verdict::Errored(e) => Err(e),
    }
}

/// What the six cells said, all four outcomes kept apart.
///
/// [`no_findings_witness`] collapses `Decided` and `Undecided` into `Ok(None)` because AC-527's
/// THEN only asks whether the answer is `true`. [`walk`] needs them separate, because their
/// difference **is** the gap between AC-527's THEN and ADR-0080 decision 2's *"satisfiable only by
/// an empty finding set"*, and a gap that is not counted is a gap nobody can act on.
enum Verdict {
    /// Some cell was `true`.
    Refused(Witness),
    /// Every cell was `false`.
    Decided,
    /// No cell was `true`, and at least one was `null` or errored.
    Undecided,
    /// No cell yielded a truth value at all.
    Errored(EvalError),
}

fn verdict(expr: &Expr) -> Verdict {
    let mut first_error: Option<EvalError> = None;
    let mut saw_value = false;
    let mut saw_null = false;

    for findings in [FindingsWitness::Empty, FindingsWitness::Absent] {
        for rigor in LIVE_RIGOR_LEVELS {
            // TWO BINDINGS AND NO MORE. `findings` is the one the criterion fixes; `rigor` is the
            // one that is never absent at evaluation time. A third would be a claim about a fact no
            // catalog file carries — see this module's header for what binding every path to `null`
            // cost when it was tried.
            let mut ctx = Context::new();
            ctx.bind(
                FINDINGS,
                match findings {
                    FindingsWitness::Empty => Value::List(Vec::new()),
                    FindingsWitness::Absent => Value::Null,
                },
            );
            match Value::ordinal(&RIGOR_DOMAIN, rigor) {
                Ok(v) => ctx.bind(RIGOR, v),
                Err(e) => return Verdict::Errored(e),
            }
            match evaluate(expr, &ctx) {
                Ok(outcome) => {
                    saw_value = true;
                    match outcome.value {
                        Tri::True => return Verdict::Refused(Witness { findings, rigor }),
                        Tri::Null => saw_null = true,
                        Tri::False => {}
                    }
                }
                Err(e) => {
                    saw_null = true;
                    first_error = first_error.or(Some(e));
                }
            }
        }
    }
    // A predicate that never resolved anywhere is indeterminate, not lawful — and not refused
    // either: it has no truth value, so it cannot have the one AC-527 forbids.
    match first_error {
        Some(e) if !saw_value => Verdict::Errored(e),
        _ if saw_null => Verdict::Undecided,
        _ => Verdict::Decided,
    }
}

/// Walk a catalog, returning what was seen **and** what is refused.
///
/// Separate from [`quality_block_terminal_condition_rule`] because a `Result` discards one of the
/// two on every call, and the counts matter most on the failing side: the shipped catalog is red
/// under this rule, and *how* red — and how much it does not reach — is the measurement.
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
            if reads_findings(&parsed.expr) {
                report.reads_findings += 1;
            }
            match verdict(&parsed.expr) {
                Verdict::Refused(witness) => {
                    report.refused += 1;
                    violations.push(TerminalConditionViolation::SatisfiedByNoFindings {
                        activity: activity.id.clone(),
                        step: step.id.clone(),
                        predicate: src.to_owned(),
                        witness,
                    });
                }
                Verdict::Decided => report.decided += 1,
                Verdict::Undecided => report.undecided += 1,
                Verdict::Errored(_) => report.errored += 1,
            }
        }
    }
    (report, violations)
}

/// **AC-527**, over a catalog.
///
/// # Errors
///
/// **Every** violation, never the first: a rule that returned on the first defect would hide the
/// second, and in AC-527's own fixture set the second is the discriminator.
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
