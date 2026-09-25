//! **The step executor — design §4.5's five closed step kinds, each executed as such.**
//!
//! `s7g` §7g.1. Until this module, the runtime dispatched an **activation** and implemented exactly
//! one of the five kinds by handing the whole activity to the `Harness`. design §4.5 fixes the set:
//! *"`agent` (the only place a model runs), `tool` (a toolchain verb through the port), `check` (a
//! deterministic predicate in Rust; cannot dispatch), `gate` (the kernel writes the mark and **exits
//! 0**, resuming into the same step), `emit` (binds outputs to `provides`; exactly one per kind
//! provided; terminal)"*.
//!
//! Measured over the shipped pack at `s7g`'s open: **574 steps — 185 `agent`, 113 `tool`, 94
//! `check`, 80 `gate`, 102 `emit`** — of which one kind executed.
//!
//! # What each arm reaches, and the one that reaches nothing
//!
//! | kind | arm | port |
//! | --- | --- | --- |
//! | `agent` | [`Runtime::run_activity`]'s retry loop, **unchanged** | `Harness` |
//! | `tool` | the four-verb map (**ADR-0135**) | **`Toolchain`** |
//! | `check` | `predicate::evaluate` | none |
//! | `gate` | the mark, and the run exits 0 | none |
//! | `emit` | terminal; the provided kind is the activation's | none |
//!
//! 🛑 **`check` cannot dispatch, and the compiler carries it rather than a comment.** design §4.5's
//! parenthesis is a prohibition. [`check_verdict`] is a **free function taking no `&self`**, so it
//! has no `Ports`, no store and no journal in scope: the `check` arm cannot reach a port because
//! there is no path from it to one. A reviewer does not have to trust a docstring, and a later edit
//! that wanted to dispatch from `check` would have to change the signature to do it.
//!
//! # A SECOND runnable `agent` step refuses (ADR-0143)
//!
//! `ActivitySpec` is **one spec per node** (ADR-0099:61), so a second `agent` step has no card,
//! schema or inputs of its own, and AC-013's per-activation grain admits no second dispatch. This
//! arm used to call `run_activity` anyway, get its **empty re-entry report** back, and store that
//! **over the committed one** — so `dispatch_through` answered `NotCommitted { class: None }` for an
//! activation that had committed, on a report whose `skipped()` was also `true`.
//!
//! Treating the later steps as *already served* was the other candidate and is measurably false:
//! **63 of 102** activities declare more than one `agent` step and **43 of those use DIFFERENT
//! roles** — `[spec-author, reviewer]`, `[architect, reviewer, architect]` — so a dispatch bound to
//! the first role's card has not performed the reviewer's step. The refusal is keyed on a second
//! **runnable** step, so a guarded-off one still costs nothing; per-step specs are OWED.
//!
//! # AC-013's grain is per ACTIVATION and stays so
//!
//! **ADR-0138.** `RunState::is_committed(item, activity_id_at_version)` has no step component, and
//! the activation commits atomically. A run that dies **before the dispatch** re-runs that
//! activation's steps. Anything else breaks `kill_and_resume_n_step` and criterion (3).
//!
//! 🛑 **CORRECTED 2026-09-22 (ADR-0150), AND THE CORRECTION IS THE POINT.** This paragraph read
//! *"the walk happens **inside** an activation that is not yet committed"*. That is true of the
//! steps **before** the `agent` step and FALSE of every step after it: `run_activity` appends
//! `activity_committed` when it dispatches, so the rest of the walk runs inside an
//! already-committed activation. ADR-0144 exists precisely because a step can fail *after* a
//! committed dispatch, so the tree already contradicted the sentence when it was written — it
//! described the intent rather than the behaviour, which is ADR-0146's shape.
//!
//! The consequence, now named rather than implied: because `is_committed` has no step component, a
//! process death **after** the commit and before the walk ends skips the whole activation on
//! re-entry, and its remaining steps never run and never retry. Measured 2026-09-22: **39** mapped
//! `tool` steps across **23** activities sit after the first `agent` step (`test` 26,
//! `change_summary` 9, `lint` 2, `build` 2) — of which **4** could succeed today, since ADR-0148
//! refuses `test` and ADR-0135 clause 4 routes `change_summary` to the adapters' dated refusal.
//! ADR-0150 declines to move the commit: doing so would re-dispatch a committed `agent` step after
//! a crash, which is the criterion-(3) regression ADR-0143 and ADR-0144 were each written to
//! prevent. What is owed is an idempotency key whose identity includes the **step** reached — which
//! is *larger* than ADR-0143 clause 5's per-agent-step specs and is **not** closed by them: clause 5
//! is scoped to `agent` steps, and this residual is about the `tool` steps behind one. **No record
//! routes it today.**
//!
//! # The two refusals this module exists to make visible
//!
//! **ADR-0135** — 113 `tool` steps name **33** distinct verbs and only four name a port method
//! (`test` 28, `change_summary` 11, `build` 4, `lint` 4). The other 29 are refused **by name**. A
//! silent fallthrough would turn 66 of 113 steps into invisible non-events, and a run that executed
//! nothing would be indistinguishable from one that worked.
//!
//! ⚠️ **AMENDED BY ADR-0148, 2026-09-22 — the paragraph above is left standing because it is
//! true of ADR-0135 as taken, and it is no longer true of the tree.** `test`, the single largest
//! verb in the pack, now refuses too: `toolchain.test(&[])` was `cargo test --workspace` to the
//! real adapter, and the `inputs.selection` each of the 28 declares is not a test name in any of
//! its four shapes. So the executing side is **three** verbs, not four, and **30 of 33** verbs and
//! **94 of 113** steps refuse rather than 29 and 66. The refusals stay distinguishable: the 29 are
//! refused with the bare verb string, `test` with a sentence naming its selection.
//!
//! **ADR-0136** — a step guard carrying a `${…}` reference nothing resolves **refuses by name**.
//! `rigor.rs`'s `null_out_refs` is a *static lint's* treatment and is correct there; calling it here
//! would not express uncertainty but manufacture the value the guard turns on, and under ADR-0103's
//! three-valued connectives those guards would be silently skipped while the run exited 0. That
//! function is `pub(crate)` and one import away, which is why the prohibition is a record and not a
//! review note.
//!
//! ⚠️ **ADR-0136's COUNT was wrong and is corrected by E-0136-1 — it is not 54, and the cause was
//! not references.** The executor shipped evaluating every guard against `Context::new()`, which
//! binds nothing, so `eval.rs`'s unbound-path arm returned `UnknownField` and **all 121** guards
//! failed — **67 of the 67 ref-free ones on `rigor`**, before any reference was reached. The refusal
//! a reader saw therefore named the wrong cause.
//!
//! **ADR-0142** binds the selection's own context here — the item's facets with `rigor` as an
//! ordinal, built in `replay` from values it already holds and **not** transcribed onto
//! `RecordedSpec`. Measured after: **75 of 121** guards evaluate, the other **46** keep ADR-0136's
//! refusal, and `StepVerdict::Skipped` becomes reachable for the first time (an adversarial pass
//! found it unreachable by construction, because no parseable guard could answer `false` against an
//! empty context).
//!
//! 🛑 **It repairs NONE of the 94 `check` exprs.** Every one reads a prior step's OUTPUT, which no
//! context assembled from the item can bind — step-output resolution stays OWED, and the `check` arm
//! stays the dominant reason a walk ends.

use rk_types::error::StepError;

use super::activity::ActivityReport;
use crate::catalog::{Step, StepKind};
use crate::predicate::{self, Tri};
use crate::workflow::breaker::JournaledNow;
use crate::workflow::envelope::AttemptCap;

/// **ADR-0145. What a `gate` step needs in order to open a gate, and nothing else.**
///
/// `GateRecord::derive_reach` digests `(kind, tree_hash, deadline, reach)`. None of the four is
/// derivable inside this crate: `rk-kernel` may not read a clock (AC-001) and may not compute a
/// tree hash (`workflow/gate.rs`'s header), and the reach is a fold over the journal. All three
/// external values are already in hand at `replay`'s loop — `fsm_context` builds them one line
/// above the dispatch — so they are **passed**, not re-derived, and this crate stays unable to
/// invent any of them.
///
/// Bundled for the same reason [`Activation`] is: `run_activity_steps` is already at the
/// `clippy::too_many_arguments` bound, and these three always travel together.
#[derive(Debug, Clone, Copy)]
pub struct GateSite<'a> {
    /// The fold of this item's journal — `reaches(kind)` is the reach counter (AC-524, AC-246).
    pub state: &'a crate::workflow::fsm::RunState,
    /// The tree hash at THIS reach. `rk-git`'s, threaded through `RuntimeContext`.
    pub tree_hash: &'a str,
    /// The deadline at THIS reach. The caller's clock, never this crate's.
    pub deadline: rk_types::time::Timestamp,
}

/// The four values that identify one activation to the walk.
///
/// Bundled rather than passed loose because `run_activity_steps` would otherwise take eight
/// arguments and trip `clippy::too_many_arguments` (denied here). The bundle is also the honest
/// shape: these four always travel together and `run_activity` already threads all four.
#[derive(Debug, Clone, Copy)]
pub struct Activation<'a> {
    /// The item the activation belongs to.
    pub item: &'a str,
    /// `activity_id@version` — the idempotency key's second part (ADR-0097 decision 2).
    pub activity_id_at_version: &'a str,
    /// The DAG node id, which is NOT a `[[step]]` id.
    pub step_id: &'a str,
    /// The spec the `Harness` port is handed.
    pub spec: &'a rk_types::activity::ActivitySpec,
}

/// What one step's execution did to the walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepVerdict {
    /// The step ran (or was correctly skipped by its guard) and the walk continues.
    Continue,
    /// The step's guard was not [`Tri::True`], so the step did not run. Distinct from
    /// [`StepVerdict::Continue`] so a caller can count what a run actually executed rather than
    /// what it walked past — an executor that cannot tell the two apart reports a skipped activity
    /// as a performed one.
    Skipped,
    /// **A `gate`.** The kernel writes the mark and the run **exits 0**, resuming into this same
    /// step (design §4.5). The walk ends here and the activation does not commit.
    Gate,
    /// **An `emit`.** design §4.5 makes it terminal. The walk ends and the activation's provided
    /// kind stands.
    Emitted,
    /// The step failed. The walk ends and this is the activation's error.
    Failed(StepError),
}

/// The four catalog verbs that name a `Toolchain` port method — **ADR-0135**'s closed map.
///
/// The spelling is identical on both sides, which is why this is a map and not an invention. A verb
/// outside this set is refused by name; `classify` and `detect` are port methods that **no** `tool`
/// step names (measured: 0 run lines each), so they are deliberately absent — adding them would be
/// inventing a demand to fit a supply.
///
/// 🛑 **`test` IS STILL IN THIS SET AND NO LONGER REACHES ITS PORT METHOD (ADR-0148), AND THAT IS
/// DELIBERATE.** Membership is what keeps the two refusals distinguishable: a verb *outside* the set
/// is refused with the bare verb string (ADR-0135 clause 2, so the journal names which of the 29 was
/// reached), while `test` is refused with a sentence naming its unparsed `inputs.selection`. Moving
/// it out would collapse the map's own boundary and make a journal unable to say which edge a run
/// hit. This sentence read *"Every **other** verb is refused by name"* until 2026-09-22, which
/// stopped being true the moment `test` began refusing; it is corrected rather than left, because a
/// constant's docstring asserting the invariant its own change broke is how the tree learns to lie.
const MAPPED_VERBS: [&str; 4] = ["build", "change_summary", "lint", "test"];

/// **ADR-0136.** Is this guard decidable on this tree, and does it hold?
///
/// Returns `Ok(true)` to run the step, `Ok(false)` to skip it, and `Err` when the guard carries a
/// reference the evaluator cannot resolve. A step with no `when` always runs.
///
/// # Errors
///
/// [`StepError::Unsupported`] naming **the step id and the guard text**. `Unsupported` is the closed
/// set's variant for *"this is not something I can do"*, which is exactly what an unresolved
/// reference is — a failure to ask the question, not an answer to it. It is deliberately **not**
/// [`Tri::Null`]: that is a decided policy for a *known absent* value (ADR-0032, AC-079), and
/// routing a missing mechanism through it would fire the abstention gate 54 times a run for a reason
/// no operator can act on.
fn guard_holds(step: &Step, ctx: &predicate::Context) -> Result<bool, StepError> {
    let Some(src) = step.when.as_deref() else {
        return Ok(true);
    };
    let parsed = predicate::parse(src).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `when` guard of step `{}` does not parse ({e:?}): {src}",
            step.id
        ),
    })?;
    // **No `null_out_refs` here, deliberately** — see the module header. An unresolved reference
    // reaches `evaluate` and comes back as an error, which is the honest answer.
    let outcome = predicate::evaluate(&parsed.expr, ctx).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `when` guard of step `{}` cannot be evaluated on this tree ({e:?}): {src} — \
                 reference resolution is OWED (ADR-0136) and no stage in this runbook claims it",
            step.id
        ),
    })?;
    Ok(outcome.value == Tri::True)
}

/// **The `check` arm — a deterministic predicate in Rust, and it CANNOT DISPATCH.**
///
/// 🛑 **This is a free function and takes no `&self` on purpose.** design §4.5's *"cannot dispatch"*
/// is a prohibition, and the structural form of a prohibition is an absent capability rather than a
/// comment asking for restraint. With no receiver there is no `Ports`, no store and no journal in
/// scope, so the arm has no path to a port at all. `conformance.rs`'s sibling lints exist because
/// this programme has repeatedly been bitten by guards satisfied by the prose beside them; this one
/// is satisfied by the type checker.
///
/// # Errors
///
/// [`StepError::Unsupported`] when the `expr` does not parse or cannot be evaluated — the same
/// treatment [`guard_holds`] gives a `when`, for the same reason.
fn check_verdict(step: &Step, ctx: &predicate::Context) -> Result<bool, StepError> {
    let Some(src) = step.expr.as_deref() else {
        // A `check` with no `expr` asserts nothing. It is not a silent pass: an executor that
        // treated "no predicate" as "predicate held" would make every malformed check green.
        return Err(StepError::Unsupported {
            capability: format!(
                "the `check` step `{}` carries no `expr` to evaluate",
                step.id
            ),
        });
    };
    let parsed = predicate::parse(src).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `expr` of check step `{}` does not parse ({e:?}): {src}",
            step.id
        ),
    })?;
    // 🛑 **ADR-0142 clause 4: THE CONTEXT DOES NOT REPAIR THIS ARM, AND IT IS THREADED ANYWAY.**
    // Measured over the shipped pack: **all 94** `check` exprs read a prior step's OUTPUT —
    // `report.status` (22), `check.failed` (21), `findings` (11), `lint.errors` (4) and a tail of
    // singletons. None is a facet, so no context assembled from the item can bind one. It is passed
    // here so this arm evaluates against the SAME context as `when` rather than a second one, and
    // so that an `expr` which does read a facet is not failed for a reason that is not its own.
    let outcome = predicate::evaluate(&parsed.expr, ctx).map_err(|e| {
        StepError::Unsupported {
            capability: format!(
                "the `expr` of check step `{}` cannot be evaluated on this tree ({e:?}): {src} — \
                 a `check` reads a prior step's OUTPUT, and step-output resolution is OWED \
                 (ADR-0136 clause 4; ADR-0142 clause 4 measures 94 of 94 shipped exprs reading one)",
                step.id
            ),
        }
    })?;
    Ok(outcome.value == Tri::True)
}

impl super::Runtime {
    /// **The `tool` arm — a toolchain verb through the port (ADR-0135).**
    ///
    /// The verb is parsed by `workflow::memo::verb_of`, the tree's existing parse, and routed
    /// through [`MAPPED_VERBS`]. **Every other verb is refused by name.**
    ///
    /// 🛑 **`change_summary` reaches the port and receives the adapters' dated refusal, and that is
    /// correct.** Both real adapters return `Unsupported` from a row dated `2026-09-18` whose probe
    /// is `none`, citing S-BIN and ADR-0099 clause 5. The 11 such steps in the shipped pack are
    /// **expected** to refuse and must not be "fixed" into a synthesised value. Note also that the
    /// value is discarded without ever being bound: naming the type in this crate would trip
    /// Checkpoint 5's ban, so the call is made and the `Ok` arm carries nothing.
    ///
    /// *(Noted 2026-09-25, s8a fix round 5, ADR-0165: "both real adapters return `Unsupported`
    /// from a row dated `2026-09-18`" is no longer so. The Rust adapter's row is re-dated
    /// `2026-09-24` and its method answers `Ok` with the finalised `Text` arm over a non-empty text
    /// range; the Node adapter's row is unchanged. The call below passes `""`, and an empty range
    /// still refuses by name in both, because ADR-0135 decision 4 stands for the empty range, so
    /// this arm's behaviour does not move. The `Ok` is constructible by any implementor now, and
    /// this crate still binds and names nothing from it.)*
    ///
    /// # Errors
    ///
    /// The port's own [`StepError`]; [`StepError::Unsupported`] naming an unmapped verb
    /// (ADR-0135 clause 2, the bare verb string); or [`StepError::Unsupported`] naming
    /// `inputs.selection` for the MAPPED verb `test`, which refuses rather than calling the
    /// port with an empty selection (ADR-0148).
    ///
    /// *(Corrected 2026-09-22. This read "or [`StepError::Unsupported`] naming an unmapped
    /// verb", a closed set of two outcomes, and ADR-0148 produced a third from INSIDE the
    /// map. A `# Errors` section that does not list an error the function returns is worse
    /// than none, because it reads as exhaustive.)*
    fn run_tool_step(&self, step: &Step) -> Result<(), StepError> {
        let run = step.run.as_deref().unwrap_or_default();
        let Some(verb) = crate::workflow::memo::verb_of(run) else {
            return Err(StepError::Unsupported {
                capability: format!(
                    "the `tool` step `{}` names no toolchain verb (`run` = {run:?})",
                    step.id
                ),
            });
        };
        if !MAPPED_VERBS.contains(&verb) {
            // **ADR-0135 clause 2.** By name, so the journal says which of the 29 was reached.
            return Err(StepError::Unsupported {
                capability: verb.to_owned(),
            });
        }
        // THE CALL BOUNDARY. `ports.toolchain` — the port's first non-test call site.
        let toolchain = &self.ports().toolchain;
        match verb {
            "build" => toolchain.build(),
            "lint" => toolchain.lint(),
            // **ADR-0148, superseding ADR-0135's `test` row in part.** This read
            // `toolchain.test(&[])` — an EMPTY selection, which `rk-adapter-toolchain-rust` reads
            // as `cargo test --workspace`. All **28** shipped `toolchain:test` steps therefore
            // became whole-workspace runs under the 3600s bound on a shared target dir.
            //
            // 🛑 **AND THE SELECTION THEY DECLARE CANNOT BE MADE INTO A TEST LIST HERE.** Measured
            // over `.redkiln/catalog/` 2026-09-22: all 28 declare `inputs.selection`, and it is
            // `"affected"` (17), a `${…}` reference the resolver ADR-0136 clause 4 records as OWED
            // (6), `artifact:…` (4) and `"e2e"` (1). None is a test name. Parsing the key and
            // passing the string through would run `cargo test affected`, which matches **no test
            // and exits 0** — a green from a toolchain that ran nothing, which is strictly worse
            // than the whole-workspace run it would replace. So the verb refuses BY NAME, on
            // ADR-0135 clause 2's own precedent, until a selection resolver exists.
            "test" => Err(StepError::Unsupported {
                capability: format!(
                    "the `tool` step `{}` names `toolchain:test`, whose declared \
                     `inputs.selection` `catalog::Step` does not parse and this crate cannot \
                     resolve. An empty selection is `cargo test --workspace` and the literal \
                     matches no test at all, so neither is run",
                    step.id
                ),
            }),
            // The one whose `Ok` is never constructible outside `rk-ports`. Mapped, called, and the
            // value dropped — see the 🛑 above.
            // (Noted 2026-09-25, s8a fix round 5, ADR-0165: an implementor may now construct the
            // `Ok`, the `Text` arm; the empty range passed here still refuses by name in both real
            // adapters, and this crate still binds nothing from it.)
            "change_summary" => toolchain.change_summary("").map(|_| ()),
            other => Err(StepError::Unsupported {
                capability: other.to_owned(),
            }),
        }
    }

    /// **Walk an activation's steps in declaration order — `s7g` §7g.1's unit of execution.**
    ///
    /// `steps` is `None` when the caller transcribed none. **That is not a default and it is not an
    /// empty walk** (ADR-0138's forbids): it means *this caller has no catalog activity behind it*,
    /// and the activation runs exactly as it did before `s7g` — the `agent` dispatch and its retry
    /// loop. The kernel never derives, guesses or invents a list.
    ///
    /// 🛑 **The production path must always transcribe, and that is asserted rather than trusted.**
    /// `None` is for the kernel's own machinery suites, which test dispatch, retry, commit races and
    /// AC-013 re-entry and have no activity behind them. `rk-cli`'s composition root holds the
    /// `catalog::Activity` and uses [`super::super::replay::RecordedSpec::with_steps`].
    ///
    /// # Errors
    ///
    /// [`super::RuntimeError`] from the store or the fold. A step's own failure is a **value** in
    /// the report, never an error of this function (AC-032).
    pub fn run_activity_steps(
        &self,
        act: Activation<'_>,
        steps: Option<&[Step]>,
        ctx: &predicate::Context,
        gate: GateSite<'_>,
        cap: AttemptCap,
        now: JournaledNow,
    ) -> Result<ActivityReport, super::RuntimeError> {
        let Activation {
            item,
            activity_id_at_version,
            step_id,
            spec,
        } = act;
        let Some(steps) = steps else {
            return self.run_activity(item, activity_id_at_version, step_id, spec, cap, now);
        };
        // An activation whose transcribed list is EMPTY is refused, not walked past. A zero-step
        // walk reporting success is the vacuity this stage exists to end (ADR-0138's forbids).
        if steps.is_empty() {
            return Ok(ActivityReport::refused(
                now,
                StepError::Unsupported {
                    capability: format!(
                        "the activation `{activity_id_at_version}` transcribed an EMPTY step list; \
                         an activity has at least one step and a zero-step walk is not a success"
                    ),
                },
            ));
        }

        let mut dispatched: Option<ActivityReport> = None;
        for step in steps {
            let verdict = self.execute_step(step, ctx)?;
            match verdict {
                StepVerdict::Skipped => {}
                StepVerdict::Continue => {
                    if step.kind == StepKind::Agent {
                        // 🛑 **ADR-0143: A SECOND RUNNABLE `agent` STEP REFUSES, AND THE COMMITTED
                        // REPORT IS NEVER OVERWRITTEN.**
                        //
                        // `ActivitySpec` is ONE SPEC PER NODE (ADR-0099:61; `bind_nodes`' own
                        // header), so this step has no card digest, no schema and no inputs of its
                        // own — the binding took its card from whichever step first nominated a
                        // role. And AC-013's grain is per ACTIVATION, so `run_activity` would
                        // answer with its re-entry report: `dispatches: 0`, `commit: None`,
                        // `outcome: None`.
                        //
                        // Storing THAT over the committed one is what this arm used to do, and it
                        // produced `NotCommitted { class: None }` for an activation that HAD
                        // committed — the very hole Mechanism B exists to close, and a report for
                        // which `skipped()` is `true`, so a failed walk read as a legitimate
                        // AC-013 skip.
                        //
                        // Treating it as "already served" was the other candidate and is measurably
                        // false: **43 of the 63** multi-agent activities use DIFFERENT roles
                        // (`[spec-author, reviewer]`, `[architect, reviewer, architect]`), so a
                        // dispatch bound to the first role's card has not performed the reviewer's
                        // step. The refusal names the activation rather than dropping a review.
                        if let Some(done) = dispatched.as_ref() {
                            let cause = StepError::Unsupported {
                                capability: format!(
                                    "the activation `{activity_id_at_version}` reached a SECOND \
                                     runnable `agent` step (`{}`), and `ActivitySpec` is one spec \
                                     per node (ADR-0099:61): this step has no card, schema or \
                                     inputs of its own, and AC-013's per-activation grain admits \
                                     no second dispatch. Per-step specs are OWED (ADR-0143 \
                                     clause 5). The first dispatch {} — its result is not \
                                     discarded.",
                                    step.id,
                                    if done.committed() {
                                        "COMMITTED"
                                    } else {
                                        "did not commit"
                                    }
                                ),
                            };
                            return Ok(ActivityReport::refused(now, cause));
                        }
                        // **The `agent` arm.** The dispatch, its retry loop, AC-013's re-entry and
                        // AC-012's commit race are `run_activity`'s, entered unchanged.
                        let report = self.run_activity(
                            item,
                            activity_id_at_version,
                            step_id,
                            spec,
                            cap,
                            now,
                        )?;
                        let ended = !report.committed()
                            && (report.outcome().is_some_and(Result::is_err)
                                || report.mark().is_some());
                        dispatched = Some(report);
                        if ended {
                            break;
                        }
                    }
                }
                // 🛑 **ADR-0145: THE GATE ARM WRITES THE MARK, AND IT IS THE SAME MARK.**
                //
                // This arm used to `break` beside `Emitted`, writing nothing — and the comment
                // that stood here said the mark was *"written by `Runtime::step`'s `OpenGate`
                // arm"*, which was FALSE for a catalog gate step. `OpenGate` is only ever reached
                // from `selection.gates()` — the abstention, unsatisfiable and appetite gates —
                // and nothing in the tree connected the pack's **80** `[[step]] kind = "gate"` to
                // it. Measured: **55** activities had a gate silently skipped after their first
                // agent step, **2** got the `emit` refusal below for a gate they reached BEFORE
                // one, and **13** carry gates and no agent step at all.
                //
                // The record it mints is `GateRecord::derive_reach`'s, through
                // `workflow::gate::reach` — the entry that module's own header calls *"the entry a
                // loop executor calls"* and that had no `src/` caller until now. The row it
                // appends is `runtime::gate_opened_event`, the SAME token-bearing row the
                // selection-level path writes, so the fold cannot tell the two producers apart and
                // does not have to: `awaiting_gate` answers, `next_action` returns `AwaitGate`, the
                // resume loop breaks, and `terminal_read` trailers the token and exits 0.
                //
                // **No second path is authored.** That was true as an intention before this record
                // and false as a fact; it is now true because this arm calls the first one.
                StepVerdict::Gate => {
                    let Some(kind) = step.question.as_deref() else {
                        // A `gate` step with no `question` has no KIND, and a token derived over
                        // an invented one would bind an answer to a gate nobody declared. Refused
                        // by name — ADR-0135's discipline, one kind over. All 80 shipped gate
                        // steps declare one, so this is a guard against a catalog that changes,
                        // not a state the pack is in.
                        return Ok(ActivityReport::refused(
                            now,
                            StepError::Unsupported {
                                capability: format!(
                                    "the `gate` step `{}` of activation \
                                     `{activity_id_at_version}` declares no `question`, so it has \
                                     no gate KIND to derive a token over (ADR-0145). An answer \
                                     bound to an invented kind would be bound to a gate the \
                                     catalog never declared",
                                    step.id
                                ),
                            },
                        ));
                    };
                    let ctx = crate::workflow::fsm::FsmContext::supplied(
                        item,
                        gate.tree_hash,
                        gate.deadline,
                        None,
                    );
                    // 🛑 **ADR-0146 clause 2: DOES THIS GATE NEED OPENING, OR HAS IT BEEN
                    // ANSWERED? ASKED BEFORE MINTING, AND THE FIRST VERSION OF THIS ARM DID NOT
                    // ASK.**
                    //
                    // As first written the arm called `gate::reach` unconditionally, and
                    // `gate::reach` mints at `reaches(kind) + 1` — so an ANSWERED gate re-opened
                    // at reach 2, then 3, for ever. Measured through the real binary over three
                    // run→answer rounds: three `gate_opened` rows, three distinct tokens, and
                    // **never a `Dispatch`**. The **15** activities that gate BEFORE any dispatch
                    // never commit, so `is_committed` never skips their node, so every resume
                    // walked back to the same gate step. They were permanently unexecutable and
                    // the `answer` verb was a treadmill. (The 55 that gate AFTER a commit were
                    // safe only because `is_committed` skips the node next time — the defect was
                    // masked, not absent.)
                    //
                    // This mirrors `fsm::next_action`'s clause 2, which has had exactly this
                    // discipline since 2026-09-17 and whose own comment records the same bug one
                    // level up: *"a tree that moved after an answer would have re-opened an
                    // answered gate on every call, and a void token could never re-open at all."*
                    // Three dispositions, and the third is the one that was missing:
                    //
                    //   * never reached      -> open at reach 1;
                    //   * latest is VOID     -> re-open at reach n+1 (AC-252);
                    //   * otherwise          -> **PROCEED PAST IT**. An answered gate is done, and
                    //     an EXPIRED one is the expiry policy's terminal path (ADR-0017, AC-257,
                    //     AC-259) and is deliberately not re-opened here — `fsm` falls through to
                    //     `Dispatch` on both, and so does this walk.
                    let reaches = gate.state.reaches(kind);
                    let void = gate
                        .state
                        .latest_gate(kind)
                        .is_some_and(|t| gate.state.is_void(t));
                    if reaches > 0 && !void {
                        continue;
                    }
                    // A `CanonError` here is a step's own failure and therefore a VALUE (AC-032),
                    // not an error of this function — the same treatment `guard_holds` and
                    // `check_verdict` give theirs. It is also unreachable on today's payload,
                    // which is four strings and two integers; it is refused by name rather than
                    // `expect`ed so that a later edit widening that payload fails loudly instead
                    // of panicking inside a walk.
                    let record = match crate::workflow::gate::reach(gate.state, kind, &ctx) {
                        Ok(record) => record,
                        Err(e) => {
                            return Ok(ActivityReport::refused(
                                now,
                                StepError::Unsupported {
                                    capability: format!(
                                        "the `gate` step `{}` of activation \
                                         `{activity_id_at_version}` could not derive a token for \
                                         gate kind `{kind}` ({e:?})",
                                        step.id
                                    ),
                                },
                            ));
                        }
                    };
                    let token = record.token().clone();
                    self.append(super::gate_opened_event(&record))?;
                    return Ok(match dispatched {
                        Some(done) => done.with_gate(token),
                        None => ActivityReport::gated(now, token),
                    });
                }
                // An `emit` is terminal (design §4.5) and writes nothing.
                StepVerdict::Emitted => break,
                // 🛑 **ADR-0144: A STEP THAT FAILS AFTER A COMMITTED DISPATCH IS NOT DROPPED.**
                //
                // This arm read `dispatched.unwrap_or_else(|| refused(now, e))`, which discards
                // `e` outright whenever a dispatch had already happened. The committed report
                // answers `committed() == true`, `replay::dispatch_through`'s only question is
                // exactly that, and the run therefore continued — or exited 0 — having executed
                // NOTHING after the dispatch and said so nowhere.
                //
                // That is ADR-0135's refusal voided at the last step: the 29 unmapped `tool` verbs
                // are refused BY NAME precisely so a run that executed nothing is distinguishable
                // from one that worked, and this arm made them indistinguishable again. Measured
                // over the shipped pack at this repair: **33 of 102** activities reach a `tool` or
                // `check` step after their first agent step and before the walk ends, and **29**
                // of those reach a step that CANNOT succeed on this tree today.
                //
                // The commit is kept. AC-013's grain is per ACTIVATION (ADR-0138) and the
                // `activity_committed` row is already in the journal, so re-running it is not an
                // option and pretending it did not happen would be the lie ADR-0143 refused.
                StepVerdict::Failed(e) => {
                    return Ok(match dispatched {
                        Some(done) => done.with_step_failure(e),
                        None => ActivityReport::refused(now, e),
                    });
                }
            }
        }

        // The walk ended. If no `agent` step produced a value, the activation has nothing to
        // commit, and it is REFUSED BY NAME rather than returning a report that reads as a success
        // with no commit.
        //
        // Measured at `s7g`: 20 of the shipped pack's 102 activities carry no `role =` on any step.
        // Binding `emit`'s declared outputs would need the reference resolver ADR-0136 records as
        // OWED — and `catalog::Step` parses neither `emit`'s `artifacts` nor its `evidence` key, so
        // there is no binding data on this tree to bind with.
        Ok(dispatched.unwrap_or_else(|| {
            ActivityReport::refused(
                now,
                StepError::Unsupported {
                    capability: format!(
                        "the activation `{activity_id_at_version}` walked its steps with no \
                         `agent` step producing a value: an `emit` binds outputs to `provides` and \
                         `catalog::Step` parses neither `artifacts` nor `evidence`, so the binding \
                         needs the reference resolver ADR-0136 records as OWED"
                    ),
                },
            )
        }))
    }

    /// **Execute one step, and let its kind choose the arm.**
    ///
    /// The match is **total with no wildcard**, on this tree's established idiom
    /// (`workflow/budget.rs`, `rigor.rs`; `retry.rs`'s `ac028_policy_match_has_no_wildcard` is the
    /// precedent lint). A sixth kind would be `E0004` here rather than a silent default.
    ///
    /// # Errors
    ///
    /// [`super::RuntimeError`] only from the `agent` arm's store and fold. **A step's own failure is
    /// a value** — [`StepVerdict::Failed`] — never an error of this function (AC-032's shape, which
    /// [`super::Runtime::run_activity`] already follows).
    pub fn execute_step(
        &self,
        step: &Step,
        ctx: &predicate::Context,
    ) -> Result<StepVerdict, super::RuntimeError> {
        match guard_holds(step, ctx) {
            Ok(false) => return Ok(StepVerdict::Skipped),
            Err(e) => return Ok(StepVerdict::Failed(e)),
            Ok(true) => {}
        }
        let verdict = match step.kind {
            // The only place a model runs. The dispatch itself stays in `run_activity`'s retry
            // loop, unchanged, so AC-013's re-entry and `kill_and_resume_n_step` are untouched;
            // this arm names the kind and defers to it.
            StepKind::Agent => StepVerdict::Continue,
            StepKind::Tool => match self.run_tool_step(step) {
                Ok(()) => StepVerdict::Continue,
                Err(e) => StepVerdict::Failed(e),
            },
            StepKind::Check => match check_verdict(step, ctx) {
                Ok(true) => StepVerdict::Continue,
                // A `check` that does not hold ends the walk. `on_false`'s bounded re-entry table
                // (ADR-0044) is parsed and linted and has no executor; routing to it is not this
                // stage's, and inventing a destination would be worse than stopping.
                Ok(false) => StepVerdict::Failed(StepError::Unsupported {
                    capability: format!(
                        "the `check` step `{}` did not hold, and `on_false` has no executor \
                         (ADR-0044 parsed the table; nothing re-enters a step yet)",
                        step.id
                    ),
                }),
                Err(e) => StepVerdict::Failed(e),
            },
            // The kernel writes the mark and exits 0, resuming into the same step (design §4.5).
            //
            // ⚠️ **This comment used to say the mark was `Runtime::step`'s `OpenGate` arm's, and
            // that was FALSE for a catalog gate step** — `OpenGate` is only ever reached from
            // `selection.gates()`. ADR-0145 makes it true by having `run_activity_steps` call
            // `workflow::gate::reach` and append `runtime::gate_opened_event` itself. The MINTING
            // is deliberately not here: this arm names the kind and has no `GateSite`, exactly as
            // the `agent` arm names the kind and defers the dispatch to `run_activity`.
            StepKind::Gate => StepVerdict::Gate,
            // Terminal (design §4.5). What a run provides is the activity's `provides`, which the
            // composition root already binds to the spec's schema and which
            // `run_activity` already checks the returned value against.
            StepKind::Emit => StepVerdict::Emitted,
        };
        Ok(verdict)
    }
}
