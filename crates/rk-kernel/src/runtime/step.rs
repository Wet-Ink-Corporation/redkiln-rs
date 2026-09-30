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

use std::collections::BTreeMap;

use rk_ports::toolchain::{Selection, StepBound, VerbCall, VerbOutput};
use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::{
    BacklogSnapshot, BuildArtifact, BuildReport, BuildStatus, ItemRefs, ReleaseTag, ScanReport,
    SetupResult, SmokeResult, SmokeStatus,
};

use super::activity::{ActivityReport, Decision, DecisionKind, StepAt};
use crate::catalog::{InputValue, Step, StepKind, StepTimeout};
use crate::predicate::{self, Tri};
use crate::workflow::breaker::JournaledNow;
use crate::workflow::envelope::AttemptCap;
use crate::workflow::idem::IterationPath;
use crate::workflow::memo::{PortMethod, Route, ToolchainValues, VerbRegistry};

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
    /// The activation's own spec, when the binding carries one.
    ///
    /// *(Amended 2026-09-26, `s8d` build, package P3; record R1, proposed.)* An `Option` since R1:
    /// an activation with NO agent step is bound as an explicit absence of agent specs and never a
    /// synthesised card (ADR-0143's forbids; §8d.8 defect 4), so it has no spec to carry.
    pub spec: Option<&'a rk_types::activity::ActivitySpec>,
    /// **R1: the per-step specs**, keyed by `[[step]]` id — each agent step's own card, schema and
    /// bound (§8d.2). `None` is a binding that carries none (the kernel's machinery suites and a
    /// pre-R1 transcription): its first runnable agent step dispatches under [`Self::spec`] and a
    /// second refuses by name (ADR-0143 decision 2, as R1 narrows it).
    pub step_specs: Option<&'a BTreeMap<String, rk_types::activity::ActivitySpec>>,
    /// The selection's effective rigor — component 2 of each agent step's memo key (§8d.2).
    pub rigor: crate::rigor::Rigor,
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

/// **What the `tool` arm routes through** — the verb registry and the value table.
///
/// s8c §8c.4-§8c.6; records V and B = ADR-0170/0171, proposed. *(Replaced ADR-0135's `MAPPED_VERBS` on
/// 2026-09-26: the four-verb map is now the registry's four `port` rows, and the other routes are
/// the registry's closed set.)*
///
/// `Default` binds **no registry**, so every verb has no row and refuses by its bare name
/// (ADR-0135 clause 2, unchanged). It is never a built-in table: a second verb table in code would
/// give `registry.toml` two meanings (§8c.4, *"One parser, not two"*).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ToolRouting {
    registry: Option<VerbRegistry>,
    values: ToolchainValues,
    /// *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* What the composition root
    /// performs for the `review_clone` and `review_confirm` routes. Unbound: both refuse by name.
    review: ReviewSlot,
    /// *(2026-09-26, `s8d` build, package P4; R2 = ADR-0173, proposed.)* The feed table a
    /// `toolchain:<feed>` input reads — the item's `toolchain` value, data the caller holds (s8c's
    /// `resolve_source` contract). `None`: every feed input refuses by name.
    feed_table: Option<String>,
    /// The directory a declared `kb:` / `checklist:` path is relative to (`.redkiln/`). `None`: a
    /// declared path refuses by name, never resolves to empty content.
    sources: Option<std::path::PathBuf>,
    /// *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 4, proposed; D21-73
    /// item 5.)* **Row inputs**: verb → input → value, bound by the composition root — a
    /// declaration row of the selected toolchain's table (`package_binaries` → `package.binaries`)
    /// or a tool's default location (`sca_scan`'s `audit_db`/`deny_db`, ADR-0170's *"the
    /// resolver's to supply"*). A `toolchain` row's input the step's own `inputs` leaves undeclared
    /// is taken from here; a declared one is never overridden. Empty: nothing is supplied.
    row_inputs: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
    /// *(2026-09-27, `s8d` build, package F3-D2174; D21-74 ruling 2; R2 = ADR-0173 decision 9 as
    /// amended, proposed.)* The item's run-branch changeset reader, which `"affected"` reads inside
    /// `Changeset`'s own producer. Unbound: such a selection refuses by name.
    run_branch: RunBranchSlot,
    /// *(dated 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ; the render record ADR-0189
    /// decisions 7 and 9, proposed.)* `rk-store`'s reader and writer for this stage run, bound by
    /// the composition root and applied by `replay::resume_routed` to the runtime it builds.
    /// Unbound: the runtime keeps its non-emitting stubs.
    artifact_seams: ArtifactSeamSlot,
    /// *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106; the render
    /// record ADR-0189 decisions 11 and 12, proposed.)* **The `store` route's ONE seam**, shared by
    /// the step-grain `Route::Store` arm (`item_create`) and `call_feed`'s `store` arm (`backlog`).
    /// Unbound: every `store`-route call refuses by name ([`UnboundStoreRoute`]), never `Ok`-empty.
    store_route: StoreRouteSlot,
    /// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R1 = ADR-0191 decision 8,
    /// proposed.)* The adapter `rk-cli`'s `detect()` loop chose and the range the run's calls are
    /// made over, bound by the composition root and carried onto every `verify_completed` row
    /// the `toolchain:test` arm journals, opaque (AC-487). Unbound: the row reads
    /// [`UNBOUND_IDENTITY`] for each, which names the absence rather than guessing an adapter.
    verify_identity: Option<crate::verify::runner::VerifyIdentity>,
}

/// What a `verify_completed` row's `adapter`, `range.base` and `range.head` read when no
/// composition root bound an identity (a lone kernel walk): the absence, named.
pub const UNBOUND_IDENTITY: &str = "unbound";

impl ToolRouting {
    /// Bind the identity carried onto every `verify_completed` row (R1 = ADR-0191 decision 8).
    #[must_use]
    pub fn with_verify_identity(mut self, identity: crate::verify::runner::VerifyIdentity) -> Self {
        self.verify_identity = Some(identity);
        self
    }

    /// The bound identity, or the named absence.
    #[must_use]
    pub fn verify_identity(&self) -> crate::verify::runner::VerifyIdentity {
        self.verify_identity
            .clone()
            .unwrap_or_else(|| crate::verify::runner::VerifyIdentity {
                adapter: UNBOUND_IDENTITY.to_owned(),
                range: rk_types::verify::VerifyRange {
                    base: UNBOUND_IDENTITY.to_owned(),
                    head: UNBOUND_IDENTITY.to_owned(),
                },
            })
    }

    /// Bind a loaded registry and the value table `rk-cli` selected for the chosen adapter. The
    /// table carries no adapter name (AC-487).
    #[must_use]
    pub const fn new(registry: VerbRegistry, values: ToolchainValues) -> Self {
        Self {
            registry: Some(registry),
            values,
            review: ReviewSlot(None),
            feed_table: None,
            sources: None,
            row_inputs: BTreeMap::new(),
            run_branch: RunBranchSlot(None),
            artifact_seams: ArtifactSeamSlot(None),
            store_route: StoreRouteSlot(None),
            verify_identity: None,
        }
    }

    /// Bind the `store` route's handler (D21-106; ADR-0189 decisions 11 and 12, proposed): the
    /// composition root's seam over `rk-store`'s item writer and backlog reader. *(dated
    /// 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
    #[must_use]
    pub fn with_store_route(mut self, route: std::sync::Arc<dyn StoreRoute>) -> Self {
        self.store_route = StoreRouteSlot(Some(route));
        self
    }

    /// The `store` route's handler: the bound one, or [`UnboundStoreRoute`], which refuses by name.
    #[must_use]
    pub fn store_route(&self) -> &dyn StoreRoute {
        match self.store_route.0.as_deref() {
            Some(route) => route,
            None => &UnboundStoreRoute,
        }
    }

    /// Bind `rk-store`'s reader and writer for this stage run (the render record ADR-0189,
    /// proposed). *(dated 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ.)*
    #[must_use]
    pub fn with_artifact_seams(
        mut self,
        read: std::sync::Arc<dyn super::reference::ArtifactReadSeam>,
        write: std::sync::Arc<dyn super::reference::ArtifactWriteSeam>,
    ) -> Self {
        self.artifact_seams = ArtifactSeamSlot(Some((read, write)));
        self
    }

    /// The bound reader and writer, when the composition root bound them.
    #[must_use]
    pub fn artifact_seams(
        &self,
    ) -> Option<(
        std::sync::Arc<dyn super::reference::ArtifactReadSeam>,
        std::sync::Arc<dyn super::reference::ArtifactWriteSeam>,
    )> {
        self.artifact_seams.0.clone()
    }

    /// Bind the item's run-branch changeset reader (D21-74 ruling 2; R2 = ADR-0173 decision 9 as
    /// amended, proposed). The composition root binds R6's reader over its checkout.
    #[must_use]
    pub fn with_run_branch(
        mut self,
        reader: std::sync::Arc<dyn super::reference::RunBranch>,
    ) -> Self {
        self.run_branch = RunBranchSlot(Some(reader));
        self
    }

    /// The run-branch reader, when the composition root bound one.
    #[must_use]
    pub fn run_branch(&self) -> Option<&dyn super::reference::RunBranch> {
        self.run_branch.0.as_deref()
    }

    /// Bind one row input (R8 = ADR-0179 decision 4, proposed): `verb`'s `input`, supplied when
    /// the step leaves it undeclared. A second binding of the same pair replaces the first.
    #[must_use]
    pub fn with_row_input(mut self, verb: &str, input: &str, value: serde_json::Value) -> Self {
        self.row_inputs
            .entry(verb.to_owned())
            .or_default()
            .insert(input.to_owned(), value);
        self
    }

    /// Bind every row input in `table` (verb → input → value), as
    /// [`crate::workflow::memo::VerbRegistry::row_inputs_for`] returns it.
    #[must_use]
    pub fn with_row_inputs(
        mut self,
        table: BTreeMap<String, BTreeMap<String, serde_json::Value>>,
    ) -> Self {
        for (verb, inputs) in table {
            for (input, value) in inputs {
                self = self.with_row_input(&verb, &input, value);
            }
        }
        self
    }

    /// The row input bound for `verb`'s `input`, if any.
    #[must_use]
    pub fn row_input(&self, verb: &str, input: &str) -> Option<&serde_json::Value> {
        self.row_inputs.get(verb).and_then(|m| m.get(input))
    }

    /// Bind the review routes (R5 = ADR-0177 decision 2): the composition root's clone and the
    /// re-executor `review::confirm` runs through. `rk-kernel` opens no files, so the side that
    /// writes the clone is the root's, and the kernel is handed the `Workspace` value it returns.
    #[must_use]
    pub fn with_review_routes(mut self, routes: std::sync::Arc<dyn ReviewRoutes>) -> Self {
        self.review = ReviewSlot(Some(routes));
        self
    }

    /// The review routes, when the composition root bound them.
    #[must_use]
    pub fn review_routes(&self) -> Option<&dyn ReviewRoutes> {
        self.review.0.as_deref()
    }

    /// Bind what the resolver's `kb:` / `checklist:` / `toolchain:<feed>` arm reads (D21-43 N14):
    /// the feed table's name and the pack directory the declared paths are relative to.
    #[must_use]
    pub fn with_sources(
        mut self,
        feed_table: Option<String>,
        sources: Option<std::path::PathBuf>,
    ) -> Self {
        self.feed_table = feed_table;
        self.sources = sources;
        self
    }

    /// The feed table's name, when one was bound.
    #[must_use]
    pub fn feed_table(&self) -> Option<&str> {
        self.feed_table.as_deref()
    }

    /// The pack directory, when one was bound.
    #[must_use]
    pub fn sources(&self) -> Option<&std::path::Path> {
        self.sources.as_deref()
    }

    /// The registry, when one was bound.
    #[must_use]
    pub const fn registry(&self) -> Option<&VerbRegistry> {
        self.registry.as_ref()
    }

    /// The value table.
    #[must_use]
    pub const fn values(&self) -> &ToolchainValues {
        &self.values
    }
}

/// **R5 (ADR-0177, proposed): what the composition root performs for the two review routes.**
///
/// `rk-git` is a concrete crate and not a port, and `rk-kernel` opens no files (its manifest), so
/// the clone is written by the root, through this seam, and the kernel is handed its `Workspace`
/// value: the destination, the sha and the working-tree choice. The value is what a
/// `side_effecting` step commits (R1), so on re-entry or replay it is SERVED and no clone is made.
pub trait ReviewRoutes: Send + Sync {
    /// `shallow_clone`: ADR-0098 decision 2's retained gix write at `sha`, with a working tree
    /// checked out (`WorkingTree::Declared`), because `review::confirm` re-executes in it. Returns
    /// the `Workspace` value `{ path, sha, working_tree }`.
    ///
    /// # Errors
    /// A [`StepError`] naming what the clone could not do.
    fn clone_at(&self, sha: &str) -> Result<serde_json::Value, StepError>;

    /// `verdict_record`: the re-executor `review::confirm` re-runs each finding through, rooted in
    /// the clone the activation's `review_clone` step committed (`workspace`, its `Workspace`).
    ///
    /// # Errors
    /// A [`StepError`] naming what could not be built over that workspace.
    fn reexecutor(
        &self,
        workspace: &serde_json::Value,
    ) -> Result<Box<dyn crate::review::reexec::ReExecutor + '_>, StepError>;
}

/// **One `store`-route call** (D21-106): the verb, the step's resolved `inputs` (empty for a feed),
/// and where it runs. *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
#[derive(Debug, Clone, Copy)]
pub struct StoreCall<'a> {
    /// The registry verb (`item_create`, `backlog`).
    pub verb: &'a str,
    /// The step's resolved `inputs` table; a feed call's is empty.
    pub inputs: &'a BTreeMap<String, serde_json::Value>,
    /// The item the walk runs for.
    pub item: &'a str,
    /// The stage the runtime runs.
    pub stage: &'a str,
    /// `activity_id@version` of the activation that reached the route.
    pub activity_id_at_version: &'a str,
    /// The stage run's id.
    pub run_id: &'a str,
}

/// **The `store` route's handler** (D21-106; the render record ADR-0189 decisions 11 and 12,
/// proposed). `rk-kernel` opens no managed tree, so the composition root binds this over
/// `rk-store`'s item writer (`item_create`) and its backlog reader (`backlog`), as it binds the
/// review routes. ONE seam serves both the step-grain `Route::Store` arm and `call_feed`'s `store`
/// arm. Its output is still typed at the boundary through the closed decode table against the
/// row's `outputs` (D21-106 ruling 4). *(dated 2026-09-28, s8f build, package
/// S8F-P6-ITEMS-BACKLOG-FEEDS.)*
pub trait StoreRoute: Send + Sync {
    /// Serve one call.
    ///
    /// # Errors
    /// A [`StepError`] naming what the route could not do.
    fn serve(&self, call: &StoreCall<'_>) -> Result<VerbOutput, StepError>;
}

/// **The kernel's default: REFUSES BY NAME** (D21-106 ruling 3). Unlike `UnwiredReadSeam`, whose
/// `read` returns `Ok(())`, it never answers with an empty value: an unbound kernel that served an
/// empty `BacklogSnapshot` would be N14's silent `null`.
#[derive(Debug, Clone, Copy, Default)]
pub struct UnboundStoreRoute;

impl StoreRoute for UnboundStoreRoute {
    fn serve(&self, call: &StoreCall<'_>) -> Result<VerbOutput, StepError> {
        Err(StepError::Unsupported {
            capability: format!(
                "`{}` takes the `store` route, and no store-route handler is bound for this run: \
                 the composition root binds `rk-store`'s (D21-106; ADR-0189 decisions 11 and 12), \
                 and an unbound route is refused by name, never served empty",
                call.verb
            ),
        })
    }
}

/// The bound [`StoreRoute`], compared by identity as [`ReviewSlot`] is.
#[derive(Clone, Default)]
struct StoreRouteSlot(Option<std::sync::Arc<dyn StoreRoute>>);

impl std::fmt::Debug for StoreRouteSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "StoreRouteSlot(bound)"
        } else {
            "StoreRouteSlot(unbound)"
        })
    }
}

impl PartialEq for StoreRouteSlot {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(a), std::sync::Arc::as_ptr(b))
            }
            _ => false,
        }
    }
}

/// The bound [`ReviewRoutes`], compared by identity: two routings are equal when both bind none or
/// both bind the same object.
#[derive(Clone, Default)]
struct ReviewSlot(Option<std::sync::Arc<dyn ReviewRoutes>>);

impl std::fmt::Debug for ReviewSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "ReviewSlot(bound)"
        } else {
            "ReviewSlot(unbound)"
        })
    }
}

impl PartialEq for ReviewSlot {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(a), std::sync::Arc::as_ptr(b))
            }
            _ => false,
        }
    }
}

/// The bound [`super::reference::RunBranch`], compared by identity as [`ReviewSlot`] is.
#[derive(Clone, Default)]
struct RunBranchSlot(Option<std::sync::Arc<dyn super::reference::RunBranch>>);

impl std::fmt::Debug for RunBranchSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "RunBranchSlot(bound)"
        } else {
            "RunBranchSlot(unbound)"
        })
    }
}

impl PartialEq for RunBranchSlot {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(a), std::sync::Arc::as_ptr(b))
            }
            _ => false,
        }
    }
}

/// The bound reader and writer, compared by identity as [`RunBranchSlot`] is. *(dated 2026-09-28,
/// s8f build, package S8F-P4-RENDER-WRITE-READ.)*
#[derive(Clone, Default)]
struct ArtifactSeamSlot(
    Option<(
        std::sync::Arc<dyn super::reference::ArtifactReadSeam>,
        std::sync::Arc<dyn super::reference::ArtifactWriteSeam>,
    )>,
);

impl std::fmt::Debug for ArtifactSeamSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "ArtifactSeamSlot(bound)"
        } else {
            "ArtifactSeamSlot(unbound)"
        })
    }
}

impl PartialEq for ArtifactSeamSlot {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some((ar, aw)), Some((br, bw))) => {
                std::ptr::addr_eq(std::sync::Arc::as_ptr(ar), std::sync::Arc::as_ptr(br))
                    && std::ptr::addr_eq(std::sync::Arc::as_ptr(aw), std::sync::Arc::as_ptr(bw))
            }
            _ => false,
        }
    }
}

/// A `tool` step's typed output, `(kind, value)`, when its route produces one (R1).
type ToolOutput = Option<(String, serde_json::Value)>;

/// What a `tool` step's route check leaves to call, once its bound is resolved.
#[derive(Debug, Clone, Copy)]
enum ToolCall {
    Port(PortMethod),
    Verb,
    /// R5: `shallow_clone`, through [`ReviewRoutes::clone_at`].
    ReviewClone,
    /// R5: `verdict_record`, through `review::confirm` over [`ReviewRoutes::reexecutor`].
    ReviewConfirm,
    /// *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106.)* The `store`
    /// route, through [`StoreRoute::serve`].
    Store,
}

/// *(2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decisions 9 and 11, proposed.)* ADR-0148's
/// refusal is REVERSED by the resolver, not by a revert (its decision 4): a `toolchain:test` step
/// reaches the port with the selection its `inputs.selection` resolves to. What stands of ADR-0148
/// is its forbids — no declared selection passed through as a test name, and no empty selection —
/// so a selection that does not resolve refuses HERE, by name, before the bound and the port.
fn selection_refusal(step: &Step, e: &super::reference::RefError) -> StepError {
    StepError::Unsupported {
        capability: format!(
            "the `tool` step `{}` names `toolchain:test`, and its `inputs.selection` does not \
             resolve to a test selection: {e}. An unresolvable selection is refused by name and \
             never reaches the port as an empty one (ADR-0148's forbids stand; R2 = ADR-0173 \
             decision 9)",
            step.id
        ),
    }
}

/// *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177 decision 5, proposed.)* What the
/// `review_confirm` route reads beyond its inputs: the selection's context, for the authored stage
/// rigor the review tier derives from (AC-302), and the walk's scope, for the `Workspace` the
/// activation's `review_clone` step committed.
#[derive(Clone, Copy)]
struct ReviewSite<'a> {
    ctx: &'a predicate::Context,
    scope: &'a super::reference::Scope,
    /// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)* The item and the
    /// `activity_id@version` the step runs in: the `verify_completed` row's item key and the
    /// first half of its `origin` (R1 = ADR-0191 decision 8, proposed). Empty for a lone step.
    item: &'a str,
    activity: &'a str,
}

/// The refusal a review route gives when no routes are bound. Checked at (3) before the call, so at
/// (5) it is unreachable; it is a value rather than a panic all the same.
fn unreachable_unbound(verb: &str) -> StepError {
    StepError::Unsupported {
        capability: format!(
            "`{verb}` takes a review route and no review routes are bound for this run (R5 = \
             ADR-0177 decision 2)"
        ),
    }
}

/// **`verdict_record` (R5 = ADR-0177 decision 5): the kernel's `review::confirm` over the resolved
/// finding set.** The tier is the authored stage rigor's (AC-302: no parameter a model-produced
/// value could arrive through); the re-execution tree is the clone the activation's `review_clone`
/// step committed, found in the scope by its verb's route. The committed value is the `Verdict`:
/// the reviewed sha, the weight, the block set and the advisories, each finding by its AC-305
/// identity. The rows `confirm` computes (`review_block`, `advisory_created`, ...) are NOT
/// appended here: ADR-0162 decision 6's review pass stays their one writer (R5 decision 6).
fn review_confirm(
    step: &Step,
    verb: &str,
    inputs: &BTreeMap<String, serde_json::Value>,
    review: ReviewSite<'_>,
    routing: &ToolRouting,
    routes: &dyn ReviewRoutes,
    reached: &mut bool,
) -> Result<serde_json::Value, StepError> {
    let refuse = |why: String| StepError::Unsupported {
        capability: format!(
            "the `tool` step `{}` names `toolchain:{verb}`: {why} (R5 = ADR-0177 decision 5)",
            step.id
        ),
    };
    let raw = inputs
        .get("findings")
        .ok_or_else(|| refuse("its `inputs.findings` did not resolve".to_owned()))?;
    let findings = rk_types::artifact::findings::Findings::ingest(raw.clone()).map_err(|e| {
        refuse(format!(
            "its `inputs.findings` is not a finding set ({e:?}); an unreadable payload is a \
             refusal, never an empty set (AC-282)"
        ))
    })?;
    let rigor = match review.ctx.get("rigor") {
        Some(predicate::Value::Ordinal { domain, index }) => domain
            .get(*index)
            .and_then(|token| crate::rigor::Rigor::from_token(token)),
        _ => None,
    }
    .ok_or_else(|| {
        refuse(
            "the selection's context binds no stage `rigor`, and the review tier derives from \
             the authored stage rigor and nothing else (AC-302)"
                .to_owned(),
        )
    })?;
    let weight = crate::review::policy::resolve_review_tier(Some(rigor));
    let clones_here = |candidate: &str| {
        routing
            .registry()
            .and_then(|r| r.entry(candidate))
            .is_some_and(|e| e.route == Route::ReviewClone)
    };
    let (clone_step, workspace) = review.scope.output_of_verb(clones_here).ok_or_else(|| {
        refuse(
            "no step of this activation taking the `review_clone` route has committed a \
             `Workspace`, so there is no checked-out tree at the reviewed sha to re-execute in"
                .to_owned(),
        )
    })?;
    let sha = workspace
        .get("sha")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            refuse(format!(
                "the `Workspace` committed by `{clone_step}` carries no `sha`"
            ))
        })?
        .to_owned();
    // m14 (dated 2026-09-27, `s8d` build, package R4FIX-8d): the route is TAKEN here, past every
    // input refusal above; from this call on the step made its one dispatch (§8d.3).
    *reached = true;
    let reexecutor = routes.reexecutor(workspace)?;
    let quarantines = BTreeMap::new();
    let confirmed = crate::review::confirm::confirm_with(
        &crate::review::confirm::ConfirmInput {
            findings: &findings,
            weight,
            sha: &sha,
            review_id: &step.id,
            now: rk_types::time::Timestamp::from_unix_seconds(0),
            quarantines: &quarantines,
        },
        reexecutor.as_ref(),
    );
    let finding = |r: &crate::review::record::PersistedFinding| {
        serde_json::json!({
            "finding_id": r.finding_id.as_str(),
            "ac": r.ac,
            "class": r.computed_class.as_str(),
        })
    };
    let blocking: Vec<serde_json::Value> = confirmed.block_set().into_iter().map(finding).collect();
    let advisories: Vec<serde_json::Value> = confirmed
        .records
        .iter()
        .filter(|r| !matches!(r.decision, crate::review::policy::Decision::Block(_)))
        .map(finding)
        .collect();
    Ok(serde_json::json!({
        "sha": sha,
        "weight": weight.as_str(),
        "blocking": blocking,
        "advisories": advisories,
        "override": inputs.get("override").cloned().unwrap_or(serde_json::Value::Null),
    }))
}

/// The port's `Selection` for a resolved plan. `recorded` is `None`: the resolver holds no
/// recorded whole-suite count, and the adapter's rules decide what an absent one means.
fn port_selection(plan: super::reference::SelectionPlan) -> Selection {
    match plan {
        super::reference::SelectionPlan::Filter(tests) => Selection::Filter(tests),
        super::reference::SelectionPlan::Affected(changed) => Selection::Affected {
            changed,
            recorded: None,
        },
        super::reference::SelectionPlan::Named(name) => Selection::Named(name),
    }
}

/// Whether a later `check` step of the list reads `step`'s output (`${<step>…}` in its `inputs`).
///
/// *(2026-09-26, `s8d` build, package P4.)* The decode table's and `build`'s interim verdicts hold
/// *"until `s8d` binds the typed output to `${step.outputs}` and the catalog's own `status ==
/// 'passing'` check reads it"* (record T's `build_verdict` note; ADR-0170's *"Until `s8d` binds step
/// outputs"*). This is that condition, read literally: in a walk where a later `check` reads the
/// step, the output is BOUND and the check decides (N25: a red build is a report whose status fails
/// `built`); anywhere else — a lone `execute_step`, or a step no check reads — the interim verdict
/// stands, so a red build is never walked past unread.
fn read_by_a_later_check(steps: &[Step], index: usize) -> bool {
    let id = steps[index].id.as_str();
    steps[index + 1..].iter().any(|s| {
        s.kind == StepKind::Check
            && s.inputs.values().filter_map(InputValue::as_text).any(|t| {
                super::reference::parse_source(t)
                    .is_ok_and(|src| super::reference::step_names(&src).contains(&id))
            })
    })
}

/// **Record B decision 2.** The step's bound.
///
/// Undeclared, a literal, or a `${toolchain.<key>}` resolved through the value table. An absent or `unmeasured` key refuses by name and **never
/// falls back to the construction bound**; a zero refuses (RT-5).
///
/// # Errors
/// [`StepError::Unsupported`] naming `${toolchain.<key>}` and why, or naming RT-5 for a zero.
pub fn resolve_bound(step: &Step, values: &ToolchainValues) -> Result<StepBound, StepError> {
    match &step.timeout_secs {
        None => Ok(StepBound::Undeclared),
        Some(StepTimeout::Literal(secs)) => StepBound::secs(*secs),
        Some(StepTimeout::Toolchain(key)) => {
            values
                .resolve(key)
                .map(StepBound::Secs)
                .map_err(|refusal| StepError::Unsupported {
                    capability: format!("the `tool` step `{}`'s timeout_secs: {refusal}", step.id),
                })
        }
    }
}

/// **The closed decode table** (record V decision 2).
///
/// A `run_verb` output is typed at the boundary, as `verify::report_at_boundary` types a `VerifyReport`. An output whose `kind` is not
/// the row's `outputs` is refused naming BOTH kinds; a value that does not decode is refused naming
/// the kind; a kind with no entry here is refused naming it.
///
/// The typed value binds nothing yet (`s8d` binds step outputs). Until then this function's only
/// channel is the step's `Result`, so a report that carries a verdict is read for it, as
/// [`build_verdict`] reads `build`'s: a `failing` `BuildReport` and a `SmokeResult` that is not
/// `passing` are not a walked step.
///
/// # Errors
/// [`StepError::Transport`] with [`TransportKind::Decode`], or the verdict's `NonZero`.
pub fn decode_verb_output(expected: &str, output: &VerbOutput) -> Result<(), StepError> {
    decode_verb_value(expected, output, true)
}

/// [`decode_verb_output`], with the interim verdict read only when `verdict` is set: a walk whose
/// later `check` reads the output binds it and lets the check decide (see
/// `read_by_a_later_check`). The kind check and the decode always run.
fn decode_verb_value(expected: &str, output: &VerbOutput, verdict: bool) -> Result<(), StepError> {
    let decode = |detail: String| StepError::Transport {
        kind: TransportKind::Decode,
        detail,
    };
    if output.kind != expected {
        return Err(decode(format!(
            "the verb's registry row declares `outputs = {expected:?}` and the adapter returned \
             kind {:?}; an output of the wrong kind never reaches the kernel as a typed value \
             (ADR-0170 decision 2)",
            output.kind
        )));
    }
    let value = output.value.clone();
    let undecodable =
        |e: serde_json::Error| decode(format!("the `{expected}` value does not decode: {e}"));
    match expected {
        "BuildReport" => {
            let report = serde_json::from_value::<BuildReport>(value).map_err(undecodable)?;
            if verdict {
                build_verdict(&report)
            } else {
                Ok(())
            }
        }
        "SmokeResult" => {
            let smoke: SmokeResult = serde_json::from_value(value).map_err(undecodable)?;
            match smoke.status {
                _ if !verdict => Ok(()),
                SmokeStatus::Passing => Ok(()),
                SmokeStatus::Failing | SmokeStatus::Empty => Err(StepError::NonZero {
                    code: None,
                    stderr_digest: report_digest(&smoke),
                }),
            }
        }
        "ScanReport" => serde_json::from_value::<ScanReport>(value)
            .map(|_| ())
            .map_err(undecodable),
        "BuildArtifact" => serde_json::from_value::<BuildArtifact>(value)
            .map(|_| ())
            .map_err(undecodable),
        "SetupResult" => serde_json::from_value::<SetupResult>(value)
            .map(|_| ())
            .map_err(undecodable),
        "ItemRefs" => serde_json::from_value::<ItemRefs>(value)
            .map(|_| ())
            .map_err(undecodable),
        // *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; ADR-0189 decision 12,
        // proposed.)* `backlog`'s output.
        "BacklogSnapshot" => serde_json::from_value::<BacklogSnapshot>(value)
            .map(|_| ())
            .map_err(undecodable),
        "ReleaseTag" => serde_json::from_value::<ReleaseTag>(value)
            .map(|_| ())
            .map_err(undecodable),
        other => Err(decode(format!(
            "the kind `{other}` has no entry in the kernel's closed decode table (ADR-0170 \
             decision 2); an entry is added with its `rk-types` struct"
        ))),
    }
}

/// The digest of a report's canonical JSON, so a journal names what was read.
fn report_digest<T: serde::Serialize>(report: &T) -> rk_types::digest::Digest {
    rk_types::digest::sha256(
        serde_json::to_string(report)
            .unwrap_or_else(|e| format!("unserialisable report: {e}"))
            .as_bytes(),
    )
}

/// **ADR-0136.** Is this guard decidable on this tree, and does it hold?
///
/// Returns `Ok(true)` to run the step, `Ok(false)` to skip it, and `Err` when the guard carries a
/// reference the evaluator cannot resolve. A step with no `when` always runs. *(dated 2026-09-27,
/// `s8d` build, package D2177-8d; ADR-0174 decision 7.)* The value is the guard's truth value:
/// `Tri::True` runs the step, and `Tri::False` and `Tri::Null` skip it. The walk tells the two
/// apart because ~~a `null` gate `when` is not a decision and is not journaled~~ a gate's `when`
/// is journaled with its three-valued outcome: *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* a `null` gate `when` skips the
/// gate, which continues the walk, so it is journaled as `null` and replayed as `null`.
///
/// # Errors
///
/// [`StepError::Unsupported`] naming **the step id and the guard text**. `Unsupported` is the closed
/// set's variant for *"this is not something I can do"*, which is exactly what an unresolved
/// reference is — a failure to ask the question, not an answer to it. It is deliberately **not**
/// [`Tri::Null`]: that is a decided policy for a *known absent* value (ADR-0032, AC-079), and
/// routing a missing mechanism through it would fire the abstention gate 54 times a run for a reason
/// no operator can act on.
fn guard_holds(
    step: &Step,
    ctx: &predicate::Context,
    resolve_ref: &mut dyn FnMut(&str) -> Result<serde_json::Value, super::reference::RefError>,
) -> Result<Tri, StepError> {
    let Some(src) = step.when.as_deref() else {
        return Ok(Tri::True);
    };
    let parsed = predicate::parse(src).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `when` guard of step `{}` does not parse ({e:?}): {src}",
            step.id
        ),
    })?;
    // **No `null_out_refs` here, deliberately** — see the module header. *(2026-09-26, `s8d` build,
    // package P4; R2 = ADR-0173.)* Each `${…}` is resolved through `runtime::reference` and bound as
    // a PATH to its value; one that does not resolve refuses by name (step id and reference), and
    // is never read as null, false or a skip.
    let (expr, bound) = super::reference::bind_refs(&parsed.expr, ctx, resolve_ref)
        .map_err(|e| e.to_step_error())?;
    let outcome = predicate::evaluate(&expr, &bound).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `when` guard of step `{}` cannot be evaluated on this tree ({e:?}): {src}",
            step.id
        ),
    })?;
    Ok(outcome.value)
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
fn check_verdict(step: &Step, ctx: &predicate::Context) -> Result<Tri, StepError> {
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
    //
    // *(2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decision 6, proposed.)* ADR-0142 clause
    // 4 is DISCHARGED: `ctx` is now the selection's context with the step's own `inputs` bound over
    // it (`runtime::reference::check_context`), through the one `predicate::evaluate`. An
    // `EvalError` is still this arm's `Err` — never read as `false`.
    let outcome = predicate::evaluate(&parsed.expr, ctx).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `expr` of check step `{}` cannot be evaluated on this tree ({e:?}): {src} — an \
             `EvalError` refuses by name and is never read as false (R2 = ADR-0173 decision 6)",
            step.id
        ),
    })?;
    Ok(outcome.value)
}

/// A `check` whose `expr` is `false`. *(2026-09-26, `s8d` build, package P4.)* `Tri::False` goes to
/// the step's `on_false` (R2 decision 6). *(Amended 2026-09-26, `s8d` build, package P5; R3,
/// proposed.)* Inside a walk `run_activity_steps` routes it (`Runtime::on_false`); this refusal is
/// what a LONE step (`execute_step`, outside any walk) returns, since no loop exists to route it
/// into. A free function, as [`check_verdict`] is.
fn check_did_not_hold(step: &Step) -> StepError {
    StepError::Unsupported {
        capability: format!(
            "the `check` step `{}` did not hold; its `on_false` ({}) is routed only inside an \
             activation's walk (R3), so a lone step refuses here by name (R2 = ADR-0173 decision 6)",
            step.id,
            step.on_false_to.as_deref().map_or_else(
                || "a keyword form, or none".to_owned(),
                |to| format!("to `{to}`")
            ),
        ),
    }
}

/// A `check` whose `expr` abstained (`null`): ADR-0103's null, which AC-079 treats as an
/// abstention and never as `false` (R2 decision 6).
fn check_abstained(step: &Step) -> StepError {
    StepError::Unsupported {
        capability: format!(
            "the `check` step `{}` abstained: its `expr` evaluated `null` over its bound inputs, \
             which is neither true nor false (ADR-0103; AC-079)",
            step.id
        ),
    }
}

impl super::Runtime {
    /// **The `tool` arm — a toolchain verb through the port (ADR-0135).**
    ///
    /// The verb is parsed by `workflow::memo::verb_of`, the tree's existing parse, and routed
    /// through the verb registry's row for it. **A verb with no row is refused by its bare name.**
    ///
    /// *(Amended 2026-09-26, s8c §8c.4; record V = ADR-0170 decision 3, proposed.)* This read
    /// *"routed through `MAPPED_VERBS`"*. The four-verb map is replaced by the registry lookup, in
    /// a fixed order: the lookup, then ADR-0148's `test` refusal, then the route and input checks,
    /// and only then the step's bound (record B = ADR-0171 decision 2), then the port. Each
    /// refusal is tellable from the others in the journal: the bare verb name (no row), ADR-0148's
    /// sentence (`test`), the verb and its first input (a `toolchain` row the resolver must
    /// supply), the route and its stage (`store`, `git`), and `${toolchain.<key>}` (the bound).
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
    /// The port's own [`StepError`]; [`StepError::Unsupported`] naming a verb with no registry
    /// row (ADR-0135 clause 2, the bare verb string); [`StepError::Unsupported`] naming
    /// `inputs.selection` for `test`, which refuses rather than calling the port with an empty
    /// selection (ADR-0148); [`StepError::Unsupported`] naming a `toolchain` row's first input,
    /// or a `store` / `git` route and its stage; [`StepError::Unsupported`] naming an unresolved
    /// `${toolchain.<key>}` (record B); or [`StepError::Transport`] `Decode` for a `run_verb`
    /// output of the wrong kind or one that does not decode (record V decision 2).
    ///
    /// *(Corrected 2026-09-22. This read "or [`StepError::Unsupported`] naming an unmapped
    /// verb", a closed set of two outcomes, and ADR-0148 produced a third from INSIDE the
    /// map. A `# Errors` section that does not list an error the function returns is worse
    /// than none, because it reads as exhaustive.)*
    ///
    /// *(Amended 2026-09-26, `s8d` build, package P4; R2 = ADR-0173, proposed.)* `inputs` is the
    /// step's `inputs` table as `runtime::reference` resolved it. ADR-0148's `test` refusal is
    /// reversed (R2 decision 9): at (2) the step's selection is resolved, and one that does not
    /// resolve refuses by name before the bound and the port. A `toolchain` row's declared inputs
    /// are supplied from the resolved table, and one the step does not declare refuses naming the
    /// verb and the input. `change_summary` is handed the resolved `inputs.range` (ADR-0165
    /// decision 4's two-dot range), and its output, `test`'s outcome and `build`'s report are bound
    /// as the step's output. `verdict` is `false` only in a walk where a later `check` reads the
    /// step's output (`read_by_a_later_check`): the interim verdicts then give way to that check.
    fn run_tool_step(
        &self,
        step: &Step,
        resolved: &Result<BTreeMap<String, serde_json::Value>, super::reference::RefError>,
        env: &super::reference::Env<'_>,
        verdict: bool,
        review: ReviewSite<'_>,
        reached: &mut bool,
    ) -> Result<ToolOutput, StepError> {
        let run = step.run.as_deref().unwrap_or_default();
        let Some(verb) = crate::workflow::memo::verb_of(run) else {
            return Err(StepError::Unsupported {
                capability: format!(
                    "the `tool` step `{}` names no toolchain verb (`run` = {run:?})",
                    step.id
                ),
            });
        };
        let routing = self.tool_routing();
        // ---- (1) THE REGISTRY LOOKUP, FIRST (ADR-0170 decision 3). -------------------------------
        // **ADR-0135 clause 2, unchanged.** No row: refused by the BARE verb name, so the journal
        // says which verb was reached, and before anything else about the step is read -- its
        // bound included (`rolling-deploy`'s `deploy` refuses by its name, never by its key).
        let Some(entry) = routing.registry().and_then(|r| r.entry(verb)) else {
            return Err(StepError::Unsupported {
                capability: verb.to_owned(),
            });
        };
        // ---- (2) THE SELECTION (R2 decision 9): ADR-0148's refusal, reversed by the resolver. An
        // unresolvable selection refuses by name HERE, before the bound and the port. ---------------
        let selection = if entry.route == Route::Port(PortMethod::Test) {
            match resolved {
                Ok(inputs) => Some(port_selection(
                    super::reference::selection_plan(env, review.scope, step, inputs)
                        .map_err(|e| selection_refusal(step, &e))?,
                )),
                Err(e) if e.binding == "selection" => return Err(selection_refusal(step, e)),
                // Another input of the step did not resolve: refused at (3), as every tool step's.
                Err(_) => None,
            }
        } else {
            None
        };
        // An input that did not resolve refuses by name here, after the registry lookup and the
        // selection (ADR-0170 decision 3's order), and before any route is taken.
        let inputs = resolved
            .as_ref()
            .map_err(super::reference::RefError::to_step_error)?;
        // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 4, proposed; D21-73
        // item 5.)* **Row inputs.** A `toolchain` row's declared input that the step's own `inputs`
        // leaves undeclared is supplied from the routing's row inputs, which the composition root
        // binds; a declared one is never overridden. Only `toolchain` rows take them: a `port` row's
        // inputs are the step's own (the selection, the range), and the review routes' are R5's.
        let supplied: BTreeMap<String, serde_json::Value>;
        let inputs = if entry.route == Route::Toolchain {
            let mut filled = inputs.clone();
            for name in &entry.inputs {
                if !filled.contains_key(name)
                    && let Some(value) = routing.row_input(verb, name)
                {
                    filled.insert(name.clone(), value.clone());
                }
            }
            supplied = filled;
            &supplied
        } else {
            inputs
        };
        // ---- (3) THE ROUTE AND INPUT CHECKS. The match is exhaustive with NO wildcard: a route
        // is added by record, and a fifth is `E0004` here (ADR-0170 decision 1). --------------------
        let call = match entry.route {
            Route::Port(method) => ToolCall::Port(method),
            Route::Toolchain => {
                // *(2026-09-26, `s8d` build, package P4.)* The row's declared inputs are supplied
                // from the step's RESOLVED `inputs`; one the step does not declare refuses BEFORE
                // the port, naming the verb and the input (a declared one that did not resolve was
                // refused by name before this arm).
                if let Some(input) = entry.inputs.iter().find(|i| !inputs.contains_key(*i)) {
                    return Err(StepError::Unsupported {
                        capability: format!(
                            "the `tool` step `{}` names `toolchain:{verb}`, whose registry row \
                             declares the input `{input}`, and the step declares no \
                             `inputs.{input}` for the resolver to bind, so the port is not called",
                            step.id
                        ),
                    });
                }
                ToolCall::Verb
            }
            // *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106; ADR-0189
            // decision 11, proposed.)* ~~Refused naming the route until `s8f`~~: the route is served
            // through the composition root's store-route seam, whose unbound default refuses by name.
            Route::Store => ToolCall::Store,
            Route::Git => {
                return Err(StepError::Unsupported {
                    capability: format!(
                        "`{verb}` takes the `git` route, which is not executed until `s8g` builds \
                         it (ADR-0170 decision 1)"
                    ),
                });
            }
            // *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177 decisions 1-5, proposed.)* The
            // two routes R5 adds by record. Each needs the composition root's review routes, and
            // each row's declared inputs, resolved; either missing refuses by name before the call.
            route @ (Route::ReviewClone | Route::ReviewConfirm) => {
                if routing.review_routes().is_none() {
                    return Err(StepError::Unsupported {
                        capability: format!(
                            "`{verb}` takes the `{}` route, and no review routes are bound for \
                             this run: the composition root binds them (R5 = ADR-0177 decision 2), \
                             and the kernel opens no files to clone with",
                            route.as_str()
                        ),
                    });
                }
                if let Some(input) = entry.inputs.iter().find(|i| !inputs.contains_key(*i)) {
                    return Err(StepError::Unsupported {
                        capability: format!(
                            "the `tool` step `{}` names `toolchain:{verb}`, whose registry row \
                             declares the input `{input}`, and the step declares no \
                             `inputs.{input}` for the resolver to bind, so the route is not taken",
                            step.id
                        ),
                    });
                }
                if route == Route::ReviewClone {
                    ToolCall::ReviewClone
                } else {
                    ToolCall::ReviewConfirm
                }
            }
        };
        // ---- (4) ONLY THEN THE STEP'S BOUND (record B decision 2): absent or unmeasured refuses
        // by name, and nothing falls back to the construction bound. -------------------------------
        let bound = resolve_bound(step, routing.values())?;
        // ---- (5) THE CALL BOUNDARY. `ports.toolchain` -- the port's first non-test call site. ----
        let toolchain = &self.ports().toolchain;
        let typed = |kind: &str, value: Result<serde_json::Value, serde_json::Error>| {
            value
                .map(|v| Some((kind.to_owned(), v)))
                .map_err(|e| StepError::Transport {
                    kind: TransportKind::Decode,
                    detail: format!("the `{kind}` value does not serialise: {e}"),
                })
        };
        match call {
            // A red build is `Ok` with `status: failing`. The report is BOUND as the step's output;
            // [`build_verdict`] reads it unless a later `check` does (N25: `build-package`'s
            // `built` reads `report.status == 'passing'`).
            ToolCall::Port(PortMethod::Build) => {
                *reached = true;
                let report = toolchain.build(bound)?;
                if verdict {
                    build_verdict(&report)?;
                }
                typed(
                    PortMethod::Build.output_kind(),
                    serde_json::to_value(&report),
                )
            }
            ToolCall::Port(PortMethod::Lint) => {
                *reached = true;
                toolchain.lint(bound).map(|()| None)
            }
            // R2 decision 9: the resolved selection, bound as the step's output (record T's
            // `TestOutcome`, whose serde form the catalog's `report.failed` / `status` read).
            ToolCall::Port(PortMethod::Test) => {
                let Some(selection) = selection else {
                    return Err(selection_refusal(
                        step,
                        &super::reference::RefError {
                            step: step.id.clone(),
                            binding: "selection".to_owned(),
                            reference: "inputs.selection".to_owned(),
                            why: "no selection was resolved".to_owned(),
                        },
                    ));
                };
                *reached = true;
                // *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT; R1 = ADR-0191
                // decision 8, proposed; `s8g` section 8g.4.)* ~~`toolchain.test(&selection,
                // bound)?`~~ -- **the one site**: the port call goes through
                // `verify::runner::run_test`, which journals `verify_completed` for a COMPLETED
                // call only, with this step's `origin` and stage, the item key beside it, and the
                // identity the composition root bound (opaque, AC-487).
                let identity = routing.verify_identity();
                let paths: Vec<String> = match &selection {
                    Selection::Affected { changed, .. } => changed.clone(),
                    Selection::WholeSuite { .. } | Selection::Filter(_) | Selection::Named(_) => {
                        Vec::new()
                    }
                };
                let outcome = crate::verify::runner::run_test(
                    toolchain.as_ref(),
                    &crate::verify::runner::TestCall {
                        item: review.item,
                        selection: &selection,
                        bound,
                        origin: crate::verify::runner::TestOrigin::Step {
                            activity_at_version: review.activity,
                            step: &step.id,
                        },
                        stage: self.stage(),
                        identity: &identity,
                        paths: &paths,
                    },
                    &mut |row| self.append(row).map_err(|e| format!("{e:?}")),
                )?;
                typed(
                    PortMethod::Test.output_kind(),
                    serde_json::to_value(&outcome),
                )
            }
            // *(2026-09-26, `s8d` build, package P4.)* The resolved `inputs.range`, never `""`:
            // ADR-0165 decision 4's range is exactly `<base>..<head>`, and the adapter refuses a
            // single revision by name (decision 6). The value is serialised without the type being
            // named in this crate (ADR-0135's surviving forbids).
            ToolCall::Port(PortMethod::SummariseChanges) => {
                let Some(range) = inputs.get("range").and_then(serde_json::Value::as_str) else {
                    return Err(StepError::Unsupported {
                        capability: format!(
                            "the `tool` step `{}` names `toolchain:change_summary` and declares no \
                             `inputs.range` that resolves to a string, so the port is not called \
                             (ADR-0165 decision 4)",
                            step.id
                        ),
                    });
                };
                *reached = true;
                let summary = toolchain.change_summary(range, bound)?;
                typed(
                    PortMethod::SummariseChanges.output_kind(),
                    serde_json::to_value(&summary),
                )
            }
            // **The seventh method** (record V decision 2): the row's declared inputs, resolved, and
            // its output typed at the boundary through the closed decode table.
            ToolCall::Verb => {
                let call_inputs: BTreeMap<String, serde_json::Value> = entry
                    .inputs
                    .iter()
                    .filter_map(|name| inputs.get(name).map(|v| (name.clone(), v.clone())))
                    .collect();
                *reached = true;
                let output = toolchain.run_verb(&VerbCall {
                    verb,
                    inputs: &call_inputs,
                    bound,
                })?;
                decode_verb_value(&entry.outputs, &output, verdict)?;
                // R1 (M8): the typed output is what a `side_effecting` step commits.
                Ok(Some((entry.outputs.clone(), output.value)))
            }
            // R5 decisions 2-4: the root writes the clone and returns its `Workspace`; the value is
            // what the step commits, so a re-entry or a replay SERVES it and clones nothing.
            ToolCall::ReviewClone => {
                let _ = bound;
                let Some(sha) = inputs.get("sha").and_then(serde_json::Value::as_str) else {
                    return Err(StepError::Unsupported {
                        capability: format!(
                            "the `tool` step `{}` names `toolchain:{verb}`, and its `inputs.sha` \
                             did not resolve to a string, so no clone is made (R5 = ADR-0177)",
                            step.id
                        ),
                    });
                };
                let routes = routing
                    .review_routes()
                    .ok_or_else(|| unreachable_unbound(verb))?;
                *reached = true;
                Ok(Some((entry.outputs.clone(), routes.clone_at(sha)?)))
            }
            // R5 decision 5: `review::confirm`, the kernel's, over the resolved finding set, at the
            // weight the authored stage rigor resolves (AC-302), re-executing in the clone.
            ToolCall::ReviewConfirm => {
                let _ = bound;
                let routes = routing
                    .review_routes()
                    .ok_or_else(|| unreachable_unbound(verb))?;
                review_confirm(step, verb, inputs, review, routing, routes, reached)
                    .map(|value| Some((entry.outputs.clone(), value)))
            }
            // *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106.)* The
            // `store` route: the step's resolved inputs to the ONE store-route seam, and its output
            // typed at the boundary through the closed decode table, as a `run_verb` output is.
            ToolCall::Store => {
                let _ = bound;
                *reached = true;
                let output = routing.store_route().serve(&StoreCall {
                    verb,
                    inputs,
                    item: env.item,
                    stage: env.stage,
                    activity_id_at_version: env.activity,
                    run_id: self.run_id(),
                })?;
                decode_verb_value(&entry.outputs, &output, verdict)?;
                Ok(Some((entry.outputs.clone(), output.value)))
            }
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
            step_specs,
            rigor,
        } = act;
        let Some(steps) = steps else {
            let Some(spec) = spec else {
                return Ok(ActivityReport::refused(
                    now,
                    StepError::Unsupported {
                        capability: format!(
                            "the activation `{activity_id_at_version}` transcribed no step list \
                             and carries no spec, so there is nothing to dispatch"
                        ),
                    },
                ));
            };
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

        // ---- R2 (`s8d` §8d.8 defect 1; ADR-0173 decision 7, proposed): A STEP AFTER THE `emit`. ----
        //
        // The loader refuses such an activity by name; a list transcribed some other way is refused
        // here too, before anything runs, rather than walked to its `emit` and its later steps
        // dropped in silence (the arm that read `StepVerdict::Emitted => break`).
        if let Some(e) = steps.iter().position(|s| s.kind == StepKind::Emit)
            && let Some(after) = steps.get(e + 1)
        {
            return Ok(ActivityReport::refused(
                now,
                StepError::Unsupported {
                    capability: format!(
                        "the activation `{activity_id_at_version}` transcribed the step `{}` after \
                         its `emit` step `{}`. An `emit` is terminal (design §4.5), so the later \
                         step would never run; it is refused by name, never dropped (R2 = \
                         ADR-0173 decision 7; `s8d` §8d.8 defect 1)",
                        after.id, steps[e].id
                    ),
                },
            ));
        }

        // ---- R1 (`s8d` §8d.2; record R1, proposed): THE WALK AT STEP GRAIN. -----------------------
        //
        // D21-42 3a: *"each agent step commits under its own key, and re-entry resumes at the first
        // uncommitted step."* A COMPLETE activation (its completion row journalled, or a pre-R1 row)
        // is not walked again; otherwise the walk runs in declaration order, SERVES every step
        // activation whose commit is in the journal, and executes the first that is not.
        if self.state(item)?.is_committed(item, activity_id_at_version) {
            return Ok(ActivityReport::walk(now));
        }
        // ---- R3 and R2 decision 5 (`s8d` §8d.3, §8d.4; package P5): THE LOOP EXECUTOR. -----------
        //
        // The walk runs in declaration order from a program counter, so a loop can move it BACK: a
        // `check`'s `{ to, max }` loop-back (ADR-0031, ADR-0044), a `repeat` pass (whose body runs
        // through the last step `until` reads, D21-48 X6) and an escalate gate's re-entry into the
        // step that escalated (schema §3). Each opens a NEW frame (D21-42 3b's iteration path), so
        // every step activation it re-runs commits under a key no earlier frame used, and the
        // committed step activations of the earlier frames STAND (D21-42 4d). A `for_each` runs one
        // copy per element, each under its own `for_each` frame. On re-entry after a crash or a
        // gate, the walk re-runs from the top: committed step activations are SERVED, ~~`check`s
        // and~~ idempotent tool steps re-execute, and the same loops are taken again.
        //
        // *(dated 2026-09-27, `s8d` build, package D2176-8d; D21-76, option (a).)* The same loops
        // are taken again BECAUSE THEY ARE REPLAYED, not because the re-executed tools answer as
        // they did. Each control decision — a `check`'s verdict, a `repeat`'s `until` and an
        // `gate` step's `when` — is journaled under the commit key of the frame that took it
        // (`Runtime::commit_decision`), and a re-entry replays every one it finds: the deciding
        // `check` is not re-executed and no loop is re-decided. Where the prior walk journaled no
        // decision, the walk decides fresh, so a crash re-entry still re-verifies whatever it had
        // not yet decided. Idempotent tool steps still re-execute, and a later `${…}` binds their
        // re-executed values (ADR-0174 decision 7); only the decisions over them are replayed.
        //
        // *(dated 2026-09-27, `s8d` build, package D2177-8d; D21-77 ruling 1.)* Only a decision
        // whose route CONTINUES the walk is journaled, so only such a decision is replayed: a
        // `true`, a loop-back, a `repeat` pass, an escalation, a `continue`, a gate walked past. A
        // `false` routed to a refusal (`"fail"`, no `on_false`, an exhausted loop-back, an `until`
        // exhausted into a refusal) leaves no row, and the next run re-verifies it. ~~A `null` is not
        // a decision and leaves no row either.~~ *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* A `null`
        // `until` or gate `when` is journaled on the same rule: when its route continues the walk
        // (another pass, a `continue` or escalation at `max`, a gate walked past), and not when it is
        // routed to a refusal.
        let mut w = Walk {
            report: ActivityReport::walk(now),
            scope: super::reference::Scope::new(steps),
            journal: self.journal(item)?,
            // With no per-step specs (a pre-R1 binding, or the kernel's machinery suites), the FIRST
            // runnable agent step dispatches under the activation's own spec and a second refuses
            // by name — ADR-0143 decision 2, as R1 narrows it. The same step re-run by a loop is
            // not a second agent step.
            first_agent: None,
            loops: Loops::default(),
            dispatched: 0,
        };
        let env = WalkEnv {
            item,
            activity: activity_id_at_version,
            node: step_id,
            spec,
            step_specs,
            rigor,
            ctx,
            gate,
            cap,
            now,
        };
        let mut pc = 0_usize;
        while let Some(step) = steps.get(pc) {
            let index = pc;
            // (1) A `repeat` starts when its step is reached outside a pass of its own.
            if let Some(max) = step.repeat_max
                && w.loops.repeat.as_ref().is_none_or(|r| r.start != index)
            {
                if step.repeat_until.is_none() {
                    let cause = StepError::Unsupported {
                        capability: format!(
                            "the step `{}` declares `repeat` with no `until`, so no pass could ever \
                             end it (R2 = ADR-0173 decision 5)",
                            step.id
                        ),
                    };
                    return Ok(w.halt(now, Halt::Failed(cause)));
                }
                w.loops.repeat = Some(Repeat {
                    start: index,
                    end: crate::catalog::repeat_body_end(steps, index),
                    pass: 1,
                    max,
                });
            }
            let path = w.loops.path_at(index);
            w.scope.enter(&step.id);
            // (2) The step: one copy, or one per `for_each` element.
            let flow = if let Some(source) = step.for_each.as_deref() {
                self.fan_out(&mut w, &env, steps, index, &path, source)?
            } else {
                let flow = self.one_step(&mut w, &env, steps, index, &path)?;
                // m14, settled by accepted ADR-0122 decision 1 (dated 2026-09-27, `s8d` build,
                // package M14-8d): the pass is charged by the dispatches the step MADE — a
                // kernel re-dispatch charges again — never a flat one per executed step.
                let dispatches = std::mem::take(&mut w.dispatched);
                if dispatches > 0 {
                    w.report.note_charge(
                        path.clone(),
                        crate::workflow::budget::charge_dispatched(step, dispatches).get(),
                    );
                }
                flow
            };
            // (3) What it did to the walk.
            pc = match flow {
                Flow::Next => match self.after_step(&mut w, &env, steps, index)? {
                    Ok(next) => next,
                    Err(halt) => return Ok(w.halt(now, halt)),
                },
                Flow::Jump(target) => target,
                Flow::Halt(halt) => return Ok(w.halt(now, halt)),
                Flow::Complete(outcome) => return Ok(w.report.with_completion(outcome)),
            };
        }

        // **R1.** The list ran off its end with no `emit` step. Every shipped activity ends in
        // exactly one (§8d.3's `emit_binding` test is R2's); a transcribed list without one — a
        // fixture catalog — completes where its walk ends, on its LAST step's id, so the plan walk
        // moves on and a run over it cannot loop re-walking a node that has nothing left to do.
        // *(Before R1 a walk with no agent step was refused here, because nothing could commit it;
        // the completion row is that commit.)*
        let last = steps.last().map_or("", |s| s.id.as_str());
        let base = w.loops.base.clone();
        let outcome = self.commit_walk_row(
            item,
            activity_id_at_version,
            StepAt {
                step_id: last,
                iteration_path: &base,
                family: None,
            },
            None,
            now,
        )?;
        Ok(w.report.with_completion(outcome))
    }

    /// **One step at one iteration path** — the guard, the inputs, and the kind's arm (R1, R2, R3).
    /// Sets [`Walk::dispatched`] to the dispatches the step MADE when it EXECUTED (neither served
    /// nor skipped), which is what the pass is charged for (m14; ADR-0122 decision 1).
    fn one_step(
        &self,
        w: &mut Walk,
        e: &WalkEnv<'_>,
        steps: &[Step],
        index: usize,
        path: &IterationPath,
    ) -> Result<Flow, super::RuntimeError> {
        let step = &steps[index];
        let (item, activity_id_at_version) = (e.item, e.activity);
        let at = StepAt {
            step_id: &step.id,
            iteration_path: path,
            family: None,
        };
        let key = at.commit_key(item, activity_id_at_version);
        // 🛑 **R1 (M8): A `side_effecting` TOOL STEP ALREADY COMMITTED IS SERVED, NEVER
        // RE-EXECUTED.** Re-running it would repeat a write — a second review clone, verdict
        // record, item or tag. An idempotent tool step re-executes (ADR-0150 decision 2's
        // residual, closed by re-execution) and commits nothing.
        let side_effecting = step.kind == StepKind::Tool && self.side_effecting(step);
        if side_effecting && self.state(item)?.is_step_committed(&key) {
            bind_committed(&mut w.scope, &w.journal, step, &key);
            w.report.note_served();
            return Ok(Flow::Next);
        }
        // 🛑 **D21-76: A CONTROL DECISION THIS ACTIVATION JOURNALED IS REPLAYED, NEVER RE-DECIDED.**
        // *(dated 2026-09-27, `s8d` build, package D2176-8d; ADR-0174 decision 7 and ADR-0175, both
        // `proposed`, amended.)* A `check` whose verdict is journaled under this commit key is not
        // re-executed: `true` runs on, `false` takes its `on_false` again — over the loop counts
        // the replay itself rebuilds, so the same route. A `gate` step's `when` is replayed the
        // same way. Where nothing is journaled the walk decides FRESH and journals what it decided,
        // so a crash re-entry still re-verifies whatever it had not yet decided. *(dated
        // 2026-09-27, `s8d` build, package D2177-8d; D21-77 ruling 1.)* A journaled `false` is one
        // whose route continued the walk (a loop-back or `continue`), so its replay re-takes a
        // route that continues; a `false` routed to a refusal was never journaled.
        // A `check`'s `null` abstains and refuses by name, so no `check` row is ever `null`; one
        // read as `null` is not replayed and the `check` decides fresh. *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)*
        if step.kind == StepKind::Check {
            match super::activity::journaled_decision(&w.journal, &key, DecisionKind::Check) {
                Some(Tri::True) => {
                    w.scope.mark_ran(&step.id);
                    return Ok(Flow::Next);
                }
                Some(Tri::False) => return Ok(self.on_false(w, steps, index)),
                Some(Tri::Null) | None => {}
            }
        }
        // A `gate` step's `when` decides whether the walk asks the human — an escalate gate's
        // decides the escalation — so it is journaled and replayed as a `check` is. ~~17 shipped
        // gates~~ 17 `when` references on 15 shipped gates read an idempotent tool step's output
        // (ADR-0174 decision 7's census; `sca-scan`'s `exception` holds three; corrected by dated
        // note 2026-09-27, `s8d` build, package D2177-8d, refuting round 8's records minor).
        let gate_when = step.kind == StepKind::Gate && step.when.is_some();
        let replayed_when = if gate_when {
            super::activity::journaled_decision(&w.journal, &key, DecisionKind::When)
        } else {
            None
        };
        // A replayed `false` walks past the gate, and so does a replayed `null` (D21-92): each
        // skips the gate, as the first walk did. *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)*
        if matches!(replayed_when, Some(Tri::False | Tri::Null)) {
            return Ok(Flow::Next);
        }
        let run = {
            let site = StepSite {
                item,
                activity: activity_id_at_version,
                events: &w.journal,
                verdict: !read_by_a_later_check(steps, index),
                guard: replayed_when,
            };
            self.run_step(step, e.ctx, &w.scope, &site)?
        };
        // D21-76: a decision taken FRESH is journaled under this step's commit key. *(dated
        // 2026-09-27, `s8d` build, package D2177-8d; D21-77 ruling 1.)* A `check`'s `false` is
        // journaled below, and only when its `on_false` route continues the walk. A `null` gate
        // ~~`when` is not a decision (ADR-0174 decision 7) and is not journaled.~~ *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)*
        // `when` skips the gate, which continues the walk, so it is journaled as `null`.
        let fresh = match (step.kind, &run.verdict) {
            (StepKind::Check, StepVerdict::Continue) => Some((DecisionKind::Check, Tri::True)),
            (StepKind::Gate, StepVerdict::Skipped) if gate_when && replayed_when.is_none() => {
                let holds = if run.guard_null {
                    Tri::Null
                } else {
                    Tri::False
                };
                Some((DecisionKind::When, holds))
            }
            (StepKind::Gate, StepVerdict::Gate) if gate_when && replayed_when.is_none() => {
                Some((DecisionKind::When, Tri::True))
            }
            _ => None,
        };
        if let Some((kind, holds)) = fresh {
            self.commit_decision(
                item,
                activity_id_at_version,
                at,
                Decision { kind, holds },
                e.now,
            )?;
        }
        // **R3: a `check` that does not hold goes to its `on_false`** (D21-42 4d), inside the walk
        // only; a lone `execute_step` keeps its by-name refusal.
        // m14 (ADR-0122 decision 1; dated 2026-09-27, `s8d` build, package R4FIX-8d): a tool step's
        // port call is one dispatch, charged whether the call answered or erred — set BEFORE any
        // halt, so the walk loop notes it on every flow. The kernel has no retry loop over it.
        if run.dispatched {
            w.dispatched = 1;
        }
        if run.held_false {
            // 🛑 **D21-77 ruling 1: A `false` ROUTED TO A REFUSAL IS NOT JOURNALED.** *(dated
            // 2026-09-27, `s8d` build, package D2177-8d; ADR-0174 decision 7 and ADR-0175, both
            // `proposed`, amended.)* Only a `false` whose route continues the walk — a loop-back
            // within `max`, or `"continue"` — is journaled and replayed. A `"fail"`, an absent
            // `on_false`, an exhausted loop-back or a refused form leaves no row, so the next run
            // re-executes the `check` over re-executed tool output and a fixed tree completes.
            let flow = self.on_false(w, steps, index);
            if !matches!(flow, Flow::Halt(_)) {
                self.commit_decision(
                    item,
                    activity_id_at_version,
                    at,
                    Decision {
                        kind: DecisionKind::Check,
                        holds: Tri::False,
                    },
                    e.now,
                )?;
            }
            return Ok(flow);
        }
        match run.verdict {
            StepVerdict::Skipped => Ok(Flow::Next),
            StepVerdict::Continue => {
                if step.kind == StepKind::Agent {
                    return self.agent_arm(w, e, step, &run, at, &key);
                }
                // R2: a tool step's typed output is bound as `${<step>.outputs}`; a `check` (or a
                // tool whose port method returns none) is marked run.
                match &run.output {
                    Some((_, value)) => w.scope.bind_output(&step.id, value.clone()),
                    None => w.scope.mark_ran(&step.id),
                }
                if side_effecting {
                    // R1 (M8): the tool step's output commits under the commit key, so a re-entry
                    // serves it.
                    let (kind, value) = run
                        .output
                        .unwrap_or_else(|| (String::new(), serde_json::Value::Null));
                    let outcome = self.commit_walk_row(
                        item,
                        activity_id_at_version,
                        at,
                        Some((kind.as_str(), &value)),
                        e.now,
                    )?;
                    w.report.note_step_commit(outcome);
                    w.journal = self.journal(item)?;
                }
                Ok(Flow::Next)
            }
            StepVerdict::Gate => self.gate_arm(w, e, steps, index),
            // **R1: AN ACTIVATION'S COMPLETION IS ITS `emit`.** design §4.5 makes it terminal;
            // the completion row — one more `activity_committed`, whose `step_id` is this
            // step's and which carries `completion: true` — is appended here, and
            // `fsm::next_action` moves past the node only on it (R1; M5). No agent-step row
            // moves: they were appended when their steps dispatched.
            //
            // **R2 decision 7: the `emit` BINDING is what commits the activation.** Each
            // provided kind (a reference or a composing table) and the evidence refs are
            // resolved against the scope and carried on the completion row, where a later
            // activation's `artifact:Kind[.field]` reads them (`read_artifact_input`). A
            // binding that does not resolve refuses by name, and the activation stays
            // incomplete.
            StepVerdict::Emitted => {
                let site = StepSite {
                    item,
                    activity: activity_id_at_version,
                    events: &w.journal,
                    verdict: true,
                    guard: None,
                };
                let emitted = match self.with_env(&site, |env| {
                    super::reference::bind_emit(env, &w.scope, step)
                }) {
                    Ok(emitted) => emitted,
                    Err(err) => return Ok(Flow::Halt(Halt::Stop(err.to_step_error()))),
                };
                // *(dated 2026-09-28, s8f build, package S8F-P4-RENDER-WRITE-READ; `s8f` §8f.6;
                // the render record ADR-0189 decisions 7 and 8, proposed.)* **The `emit` arm stops
                // being inert.** Each provided kind's bound value goes to the write seam, whose
                // `rk-store` writer writes it into the managed directory and appends that output's
                // ONE `artifact.write` (ADR-0116's forbids). The writes precede the completion row,
                // so its step landing commits them (N10). A refused write stops the activation by
                // name, and it does not complete.
                if let Some(emitted) = emitted.as_ref() {
                    for (kind, value) in &emitted.artifacts {
                        let written = self.write_seam().write(&super::reference::ArtifactWrite {
                            item,
                            stage: self.stage(),
                            kind,
                            value,
                        });
                        if let Err(e) = written {
                            return Ok(Flow::Halt(Halt::Stop(StepError::Unsupported {
                                capability: format!(
                                    "the `emit` step `{}` could not write `{kind}` through                                      rk-store's writer: {e} (ADR-0189 decision 7)",
                                    step.id
                                ),
                            })));
                        }
                    }
                }
                let outcome = self.commit_completion(
                    item,
                    activity_id_at_version,
                    at,
                    emitted.as_ref(),
                    e.now,
                )?;
                Ok(Flow::Complete(outcome))
            }
            // 🛑 **ADR-0144: A STEP THAT FAILS AFTER A COMMITTED DISPATCH IS NOT DROPPED.**
            //
            // The commit is kept and the failure travels on `step_error`. At step grain a
            // commit may be this walk's or an earlier run's (a served step): either way it is
            // in the journal, and pretending otherwise would be the lie ADR-0143 refused.
            StepVerdict::Failed(err) => Ok(Flow::Halt(Halt::Failed(err))),
        }
    }

    /// **The `agent` arm** at the step grain (R1): the step's own spec, its resolved inputs, and the
    /// dispatch at its commit key.
    fn agent_arm(
        &self,
        w: &mut Walk,
        e: &WalkEnv<'_>,
        step: &Step,
        run: &StepRun,
        at: StepAt<'_>,
        key: &crate::workflow::idem::CommitKey,
    ) -> Result<Flow, super::RuntimeError> {
        let activity_id_at_version = e.activity;
        // 🛑 **R1: EACH AGENT STEP DISPATCHES UNDER ITS OWN SPEC.** A runnable agent
        // step with no per-step spec refuses by name (ADR-0143 decision 2, as R1
        // narrows it), and the report STAYS COMMITTED when an earlier step
        // committed: `with_step_failure`, ADR-0144's shape (§8d.8 defect 2), never
        // `ActivityReport::refused`, whose report answers `committed() == false`
        // for an activation whose row is in the journal.
        let own = match e.step_specs {
            Some(map) => map.get(&step.id),
            None if w
                .first_agent
                .as_deref()
                .is_none_or(|first| first == step.id) =>
            {
                e.spec
            }
            None => None,
        };
        if w.first_agent.is_none() {
            w.first_agent = Some(step.id.clone());
        }
        let Some(own) = own else {
            let why = if e.step_specs.is_some() {
                "the binding's per-step specs carry none for it".to_owned()
            } else {
                "the binding carries no per-step specs, and the activation's own \
                 `ActivitySpec` is one spec per node (ADR-0099:61) and already \
                 served its first agent step, so this step has no card, schema or \
                 inputs of its own"
                    .to_owned()
            };
            let cause = StepError::Unsupported {
                capability: format!(
                    "the activation `{activity_id_at_version}` reached the \
                     runnable `agent` step `{}`, which has no per-step spec: {why} \
                     (ADR-0143 decision 2, as R1 narrows it). {}",
                    step.id,
                    if w.report.holds_a_commit() {
                        "An earlier step COMMITTED — its result is not discarded."
                    } else {
                        "No step of this activation had committed."
                    }
                ),
            };
            return Ok(Flow::Halt(Halt::Stop(cause)));
        };
        // **R2 (§8d.2): the step's RESOLVED inputs join ITS spec at dispatch.** The
        // binder's keys are never overwritten: a catalog binding of the same name is
        // refused by name.
        let own = match with_resolved_inputs(own, &step.id, &run.inputs) {
            Ok(own) => own,
            Err(cause) => return Ok(Flow::Halt(Halt::Stop(cause))),
        };
        // **R4 (ADR-0176 decision 4).** The breaker's key for THIS step: the pack's family for
        // its role (D-7), when a breaker is bound. A role the pack binds to no family, or an agent
        // step with no role, refuses by name before any dispatch — never keyed on a default.
        let family = match self.step_family(step) {
            Ok(family) => family,
            Err(cause) => return Ok(Flow::Halt(Halt::Stop(cause))),
        };
        let at = StepAt {
            family: family.as_ref(),
            ..at
        };
        // **The `agent` arm.** The dispatch, its retry loop and AC-012's commit race
        // are `run_activity`'s, entered at THIS step's commit key.
        let dispatched = self.run_activity_at(
            (e.item, activity_id_at_version, e.node),
            &own,
            Some(at),
            e.cap,
            e.now,
        )?;
        if dispatched.skipped() {
            // Committed by an earlier run (AC-013 at step grain): SERVED, and its
            // output bound from the journalled row. A served step charges nothing (m14).
            bind_committed(&mut w.scope, &w.journal, step, key);
            w.report.note_served();
            return Ok(Flow::Next);
        }
        // m14 (ADR-0122 decision 1): every dispatch `run_activity`'s retry loop made, its kernel
        // re-dispatches included — not a flat one per step.
        w.dispatched = dispatched.dispatches();
        let held = w.report.holds_a_commit();
        let ok = dispatched.committed() || dispatched.discarded_late();
        // The memo facts of THIS step, from THIS step's own spec (§8d.2).
        w.report.note_memo(
            crate::workflow::memo::StepContext::for_agent_step(
                activity_id_at_version,
                e.rigor,
                &step.id,
                &own,
                at.iteration_path.loop_iteration(),
            )
            .with_kb_atoms(step, &own),
        );
        w.report.absorb(dispatched);
        if !ok {
            // The dispatch failed, or ended on AC-030's mark. Before any commit the
            // report is the dispatch's own (Mechanism B's class travels on it);
            // after one, the commit stands and this is the step's failure.
            if held {
                let error = w
                    .report
                    .outcome()
                    .and_then(|o| o.as_ref().err())
                    .cloned()
                    .unwrap_or_else(|| StepError::Unsupported {
                        capability: format!("the agent step `{}` did not commit", step.id),
                    });
                return Ok(Flow::Halt(Halt::Stop(error)));
            }
            return Ok(Flow::Halt(Halt::AsIs));
        }
        w.journal = self.journal(e.item)?;
        bind_committed(&mut w.scope, &w.journal, step, key);
        Ok(Flow::Next)
    }

    /// **R2 decision 5: `for_each`** — one copy of the step per element of its source, each under
    /// its own `for_each` frame with `${item}` bound to the element, and the step's output bound as
    /// the list of the copies' outputs in element order (the value ADR-0035's `[*]` projects). The
    /// fan-out charges its pass ONCE, whatever its element count (AC-022; `charge_for_each`).
    fn fan_out(
        &self,
        w: &mut Walk,
        e: &WalkEnv<'_>,
        steps: &[Step],
        index: usize,
        path: &IterationPath,
        source: &str,
    ) -> Result<Flow, super::RuntimeError> {
        let step = &steps[index];
        let refuse = |why: String| {
            Ok(Flow::Halt(Halt::Failed(StepError::Unsupported {
                capability: format!(
                    "the step `{}` declares `for_each = {source:?}`, which {why} (R2 = ADR-0173 \
                     decision 5)",
                    step.id
                ),
            })))
        };
        if !matches!(step.kind, StepKind::Agent | StepKind::Tool) {
            return refuse(format!(
                "is a `{}` step's; a fan-out runs a dispatching (agent or tool) step",
                super::reference::kind_token(step.kind)
            ));
        }
        let site = StepSite {
            item: e.item,
            activity: e.activity,
            events: &w.journal,
            verdict: true,
            guard: None,
        };
        let resolved = self.with_env(&site, |env| {
            super::reference::resolve_text(env, &w.scope, &step.id, "for_each", source)
        });
        let items = match resolved {
            Ok(serde_json::Value::Array(items)) if !items.is_empty() => items,
            Ok(serde_json::Value::Array(_)) => {
                return refuse(
                    "resolved to an EMPTY list; a fan-out over nothing runs no copy, and a step \
                     that ran no copy is not a success"
                        .to_owned(),
                );
            }
            Ok(other) => return refuse(format!("resolved to {other}, which is not a list")),
            Err(err) => return Ok(Flow::Halt(Halt::Failed(err.to_step_error()))),
        };
        let mut outputs = Vec::with_capacity(items.len());
        let mut executed = false;
        for (i, element) in items.into_iter().enumerate() {
            let frame = crate::workflow::idem::Frame::ForEach(u32::try_from(i).unwrap_or(u32::MAX));
            w.scope.unbind(&step.id);
            w.scope.set_element(Some(element));
            let flow = self.one_step(w, e, steps, index, &path.pushed(frame))?;
            executed |= std::mem::take(&mut w.dispatched) > 0;
            match flow {
                Flow::Next => outputs.push(
                    w.scope
                        .output(&step.id)
                        .cloned()
                        .unwrap_or(serde_json::Value::Null),
                ),
                other => {
                    w.scope.set_element(None);
                    // m14 (dated 2026-09-27, `s8d` build, package R4FIX-8d): the copies already
                    // dispatched charge the pass once (AC-022) before the halt, never zero.
                    if executed {
                        w.report.note_charge(
                            path.clone(),
                            crate::workflow::budget::charge_for_each(step, outputs.len() + 1).get(),
                        );
                    }
                    return Ok(other);
                }
            }
        }
        w.scope.set_element(None);
        if executed {
            w.report.note_charge(
                path.clone(),
                crate::workflow::budget::charge_for_each(step, outputs.len()).get(),
            );
        }
        w.scope
            .bind_output(&step.id, serde_json::Value::Array(outputs));
        Ok(Flow::Next)
    }

    /// **R3: a `check` that did not hold, routed through its `on_false`** — ADR-0103 clause 6's
    /// closed set, classified by the one classifier (`predicate::classify_on_false`).
    ///
    /// | form | the walk |
    /// | --- | --- |
    /// | `{ to, max }` | re-enters `to` (an earlier step, ADR-0031) under a NEW loop-back frame; the earlier frames' commits stand |
    /// | after `max` loop-backs | refuses by name: `StepFailedAfterCommit` when a step committed, a refusal otherwise (ADR-0144) |
    /// | `"continue"` | proceeds |
    /// | `"fail"`, or no `on_false` | refuses by name, as the exhausted case |
    /// | `{ to = "deviate", … }` | refuses by name, naming ADR-0046's route OWED (D21-48 X7) |
    fn on_false(&self, w: &mut Walk, steps: &[Step], index: usize) -> Flow {
        let step = &steps[index];
        let did_not = |why: String| {
            Flow::Halt(Halt::Failed(StepError::Unsupported {
                capability: format!("the `check` step `{}` did not hold{why}", step.id),
            }))
        };
        let Some(declared) = step.on_false.as_ref() else {
            return did_not(
                ", and it declares no `on_false`: a false check with no `on_false` refuses by name \
                 as the exhausted case does (D21-42 4d; R3)"
                    .to_owned(),
            );
        };
        match predicate::classify_on_false(declared) {
            Err(err) => did_not(format!(
                ", and its `on_false` is outside ADR-0103 clause 6's closed set ({err}), so it is \
                 refused by name and never read as another form"
            )),
            Ok(predicate::OnFalse::Continue) => Flow::Next,
            Ok(predicate::OnFalse::Fail) => did_not(
                ", and its `on_false` is the keyword `fail`, which refuses by name (ADR-0103 clause \
                 6; R3)"
                    .to_owned(),
            ),
            Ok(predicate::OnFalse::Deviate) => did_not(
                ", and its `on_false` is `{ to = \"deviate\" }`, ADR-0046's route to a \
                 `RequestDeviation`, which is OWED: no mid-walk deviation is built, so the form \
                 refuses by name (D21-48 X7; R3 states this narrowing of D21-42 4d's \"keyword \
                 forms follow the schema\")"
                    .to_owned(),
            ),
            Ok(predicate::OnFalse::LoopBack { to, max }) => {
                let Some(target) = steps.iter().position(|s| s.id == to).filter(|t| *t < index)
                else {
                    return did_not(format!(
                        ", and its `on_false` names `{to}`, which is not a step declared before it \
                         in this activity (ADR-0031: loop-back only)"
                    ));
                };
                let taken = w.loops.loop_backs.get(&step.id).copied().unwrap_or(0);
                let max = u32::try_from(max).unwrap_or(0);
                if taken >= max {
                    return did_not(format!(
                        ", and its `on_false = {{ to = \"{to}\", max = {max} }}` is exhausted after \
                         {taken} loop-back(s): the walk refuses by name, and every step activation \
                         committed before it stands (ADR-0144; R3)"
                    ));
                }
                w.loops.loop_backs.insert(step.id.clone(), taken + 1);
                w.loops.reenter();
                w.scope.open_frame(target, steps.len().saturating_sub(1));
                Flow::Jump(target)
            }
        }
    }

    /// **The `gate` arm** (ADR-0145, ADR-0146 clause 2) — and, when the gate is the escalate gate
    /// of a step whose `repeat` exhausted in this frame, **the escalate re-entry** (R2 decision 5;
    /// schema §3: answering *"resumes **into the same step** with `${gate.answer}` bound"*).
    fn gate_arm(
        &self,
        w: &mut Walk,
        e: &WalkEnv<'_>,
        steps: &[Step],
        index: usize,
    ) -> Result<Flow, super::RuntimeError> {
        let step = &steps[index];
        let (item, activity_id_at_version, gate) = (e.item, e.activity, e.gate);
        let Some(kind) = step.question.as_deref() else {
            // A `gate` step with no `question` has no KIND, and a token derived over
            // an invented one would bind an answer to a gate nobody declared. Refused
            // by name — ADR-0135's discipline, one kind over. All 80 shipped gate
            // steps declare one, so this is a guard against a catalog that changes,
            // not a state the pack is in.
            return Ok(Flow::Halt(Halt::Fresh(StepError::Unsupported {
                capability: format!(
                    "the `gate` step `{}` of activation \
                     `{activity_id_at_version}` declares no `question`, so it has \
                     no gate KIND to derive a token over (ADR-0145). An answer \
                     bound to an invented kind would be bound to a gate the \
                     catalog never declared",
                    step.id
                ),
            })));
        };
        let ctx =
            crate::workflow::fsm::FsmContext::supplied(item, gate.tree_hash, gate.deadline, None);
        if let Some(&escalated) = w.loops.exhausted.get(kind) {
            // **THE ESCALATE RE-ENTRY.** The walk's n-th escalation through this gate reads the
            // n-th reach that was not voided (a void reach re-opens at the next, AC-252): answered,
            // the answer is bound and the walk re-enters the step that escalated under a NEW frame,
            // so that step reads this frame's later values, and the answer, as its previous
            // frame's (D21-48 X8); expired, the expiry policy's terminal path proceeds past it
            // (ADR-0017); open, the run is at this gate; absent, the gate is opened.
            let n = w.loops.escalations.get(kind).copied().unwrap_or(0);
            let live: Vec<&rk_types::digest::Digest> = gate
                .state
                .gates_of(kind)
                .iter()
                .filter(|t| !gate.state.is_void(t))
                .collect();
            if let Some(token) = live.get(usize::try_from(n).unwrap_or(usize::MAX)) {
                if gate.state.is_answered(token) {
                    if let Some(answer) = gate_answer_at(gate.state, &w.journal, item, token) {
                        w.scope.bind_answer(&step.id, answer);
                    }
                    w.loops.escalations.insert(kind.to_owned(), n + 1);
                    w.loops.exhausted.remove(kind);
                    w.loops.reenter();
                    w.scope.open_frame(escalated, steps.len().saturating_sub(1));
                    return Ok(Flow::Jump(escalated));
                }
                if gate.state.is_expired(token) {
                    return Ok(Flow::Next);
                }
                return Ok(Flow::Halt(Halt::Gated((*token).clone())));
            }
        } else {
            // 🛑 **ADR-0146 clause 2: DOES THIS GATE NEED OPENING, OR HAS IT BEEN
            // ANSWERED? ASKED BEFORE MINTING, AND THE FIRST VERSION OF THIS ARM DID NOT
            // ASK.** Three dispositions: never reached -> open at reach 1; latest is VOID
            // -> re-open at reach n+1 (AC-252); otherwise -> **PROCEED PAST IT** (an
            // answered gate is done, and an EXPIRED one is the expiry policy's terminal
            // path, ADR-0017, AC-257, AC-259). This mirrors `fsm::next_action`'s clause 2.
            let reaches = gate.state.reaches(kind);
            let void = gate
                .state
                .latest_gate(kind)
                .is_some_and(|t| gate.state.is_void(t));
            if reaches > 0 && !void {
                // R2 (ADR-0045): an ANSWERED gate binds its answer and its admitted
                // attachments as `${<gate>.answer}` / `${<gate>.attachments.<Kind>}`; an
                // expired one binds nothing, so a mandatory ref to it refuses by name.
                if let Some(answer) = gate
                    .state
                    .latest_gate(kind)
                    .and_then(|t| gate_answer_at(gate.state, &w.journal, item, t))
                {
                    w.scope.bind_answer(&step.id, answer);
                }
                return Ok(Flow::Next);
            }
        }
        // A `CanonError` here is a step's own failure and therefore a VALUE (AC-032),
        // not an error of this function.
        let record = match crate::workflow::gate::reach(gate.state, kind, &ctx) {
            Ok(record) => record,
            Err(err) => {
                return Ok(Flow::Halt(Halt::Fresh(StepError::Unsupported {
                    capability: format!(
                        "the `gate` step `{}` of activation \
                         `{activity_id_at_version}` could not derive a token for \
                         gate kind `{kind}` ({err:?})",
                        step.id
                    ),
                })));
            }
        };
        // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3, N6.)*
        // ~~`self.append(super::gate_opened_event(&record))?;`~~ — a gate step's opening is a
        // tier-1 act like `OpenGate`'s: its `Gate: … opened` mark first, the row anchored to it.
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE.)* The opening lands, and the
        // token is the landed record's (defect (b); ADR-0188 decision 12).
        let token = self.open_gate_act(&record)?;
        Ok(Flow::Halt(Halt::Gated(token)))
    }

    /// **After a step at `index` ran on**: the next step, or — at the last step of a `repeat`'s
    /// body — the repeat's `until`, evaluated after the body (D21-48 X6): `true` ends the repeat;
    /// otherwise another pass under a new `repeat` frame, and at `max` the step's `on_exhausted`
    /// (schema §1: *"`escalate | fail | continue`"*; R2 decision 5).
    fn after_step(
        &self,
        w: &mut Walk,
        e: &WalkEnv<'_>,
        steps: &[Step],
        index: usize,
    ) -> Result<Result<usize, Halt>, super::RuntimeError> {
        let Some(r) = w.loops.repeat else {
            return Ok(Ok(index + 1));
        };
        if r.end != index {
            return Ok(Ok(index + 1));
        }
        let repeating = &steps[r.start];
        let until = repeating.repeat_until.as_deref().unwrap_or_default();
        w.scope.leave();
        let site = StepSite {
            item: e.item,
            activity: e.activity,
            events: &w.journal,
            verdict: true,
            guard: None,
        };
        // D21-76 *(dated 2026-09-27, `s8d` build, package D2176-8d)*: the `until` of THIS pass is a
        // loop decision — another pass, or exhaustion and its escalation — journaled under the
        // repeating step's commit key at the pass's path, and REPLAYED on re-entry.
        let pass_path = w.loops.path_at(r.start);
        let at = StepAt {
            step_id: &repeating.id,
            iteration_path: &pass_path,
            family: None,
        };
        let key = at.commit_key(e.item, e.activity);
        let replayed = super::activity::journaled_decision(&w.journal, &key, DecisionKind::Until);
        let holds = replayed.map_or_else(
            || {
                self.with_env(&site, |env| {
                    until_holds(repeating, until, e.ctx, &mut |text: &str| {
                        super::reference::resolve_text(
                            env,
                            &w.scope,
                            &repeating.id,
                            "repeat.until",
                            text,
                        )
                    })
                })
            },
            Ok,
        );
        // *(dated 2026-09-27, `s8d` build, package D2177-8d; D21-77 ruling 1; ADR-0174 decision 7.)*
        // The `until` decided FRESH, journaled below only when ~~it is not `null` and~~ its route
        // continues the walk. *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* A `null` is
        // journaled on that rule too, as `null`, and a replayed `null` takes another pass (or at
        // `max` its `on_exhausted`) exactly as the fresh `null` did.
        let fresh = if replayed.is_none() {
            holds.as_ref().ok().copied()
        } else {
            None
        };
        let exhausted = |why: String| {
            Err(Halt::Failed(StepError::Unsupported {
                capability: format!(
                    "the `repeat` of step `{}` exhausted its `max` = {} without `until` holding \
                     ({until}), and {why} (R2 = ADR-0173 decision 5)",
                    repeating.id, r.max
                ),
            }))
        };
        let route = match holds {
            Err(cause) => Err(Halt::Failed(cause)),
            Ok(Tri::True) => {
                w.loops.repeat = None;
                Ok(index + 1)
            }
            Ok(_) if r.pass < r.max => {
                w.scope.open_frame(r.start, r.end);
                w.loops.repeat = Some(Repeat {
                    pass: r.pass + 1,
                    ..r
                });
                Ok(r.start)
            }
            Ok(_) => {
                w.loops.repeat = None;
                match (
                    repeating.on_exhausted.as_deref(),
                    repeating.escalate_question.as_deref(),
                ) {
                    (Some("continue"), _) => Ok(index + 1),
                    (Some("escalate"), None) => exhausted(
                        "its `on_exhausted` is `escalate` and it declares no `escalate = \
                         { question }`"
                            .to_owned(),
                    ),
                    // Schema item 15's documented pattern: *"an explicit `gate` step with
                    // `when` over the looped step's status"*. Exhaustion proceeds to it, in
                    // declaration order; with none to proceed to, it refuses by name.
                    (Some("escalate"), Some(question))
                        if !steps[index + 1..].iter().any(|s| {
                            s.kind == StepKind::Gate && s.question.as_deref() == Some(question)
                        }) =>
                    {
                        exhausted(format!(
                            "its `on_exhausted` is `escalate` to the question `{question}`, and \
                             no `gate` step after its body asks that question"
                        ))
                    }
                    (Some("escalate"), Some(question)) => {
                        w.loops.exhausted.insert(question.to_owned(), r.start);
                        Ok(index + 1)
                    }
                    (Some("fail"), _) => {
                        exhausted("its `on_exhausted` is `fail`, which refuses by name".to_owned())
                    }
                    (Some(other), _) => exhausted(format!(
                        "its `on_exhausted` is `{other}`, which is none of `escalate`, `fail` and \
                         `continue`"
                    )),
                    (None, _) => exhausted(
                        "it declares no `on_exhausted`, which refuses by name as `fail` does"
                            .to_owned(),
                    ),
                }
            }
        };
        // 🛑 **D21-77 ruling 1: only an `until` whose route CONTINUES the walk is journaled** — the
        // end of the repeat, another pass, or at `max` a `continue` or an escalation. An `until`
        // exhausted into a refusal leaves no row, so the next run re-evaluates it; ~~a `null` is not
        // a decision (ADR-0174 decision 7) and is evaluated again on a later walk.~~
        // *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)* A `null` whose route continues is
        // journaled as `null`; a `null` exhausted into a refusal leaves no row, as a `false` does.
        if let Some(tri) = fresh
            && route.is_ok()
        {
            self.commit_decision(
                e.item,
                e.activity,
                at,
                Decision {
                    kind: DecisionKind::Until,
                    holds: tri,
                },
                e.now,
            )?;
        }
        Ok(route)
    }

    /// **R1: one agent step activation, at its own commit key** — the walk's `agent` arm, public so
    /// a loop executor can drive a step under an iteration path of its own.
    ///
    /// # Errors
    ///
    /// [`super::RuntimeError`] from the store, the key validator or the fold.
    pub fn run_agent_step(
        &self,
        act: Activation<'_>,
        step: &Step,
        iteration: &crate::workflow::idem::IterationPath,
        cap: AttemptCap,
        now: JournaledNow,
    ) -> Result<ActivityReport, super::RuntimeError> {
        let own = act.step_specs.map_or(act.spec, |map| map.get(&step.id));
        let Some(own) = own else {
            return Ok(ActivityReport::refused(
                now,
                StepError::Unsupported {
                    capability: format!(
                        "the activation `{}` has no per-step spec for the `agent` step `{}` \
                         (ADR-0143 decision 2, as R1 narrows it)",
                        act.activity_id_at_version, step.id
                    ),
                },
            ));
        };
        // R4 (ADR-0176 decision 4): the step's family, as the walk's agent arm resolves it.
        let family = match self.step_family(step) {
            Ok(family) => family,
            Err(cause) => return Ok(ActivityReport::refused(now, cause)),
        };
        let mut report = self.run_activity_at(
            (act.item, act.activity_id_at_version, act.step_id),
            own,
            Some(StepAt {
                step_id: &step.id,
                iteration_path: iteration,
                family: family.as_ref(),
            }),
            cap,
            now,
        )?;
        if !report.skipped() {
            report.note_memo(
                crate::workflow::memo::StepContext::for_agent_step(
                    act.activity_id_at_version,
                    act.rigor,
                    &step.id,
                    own,
                    iteration.loop_iteration(),
                )
                .with_kb_atoms(step, own),
            );
        }
        Ok(report)
    }

    /// **R4 (ADR-0176 decision 4).** The breaker's key for an `agent` step: `Ok(None)` when no
    /// breaker is bound (the kernel's machinery suites); otherwise the selected pack's family for
    /// the step's role, through the one resolution D-7 names
    /// ([`crate::select::Pack::require_family_for_role`]). An agent step with no role, or a role
    /// the pack binds to no family, is refused by name.
    fn step_family(
        &self,
        step: &Step,
    ) -> Result<Option<crate::workflow::breaker::FamilyKey>, StepError> {
        let Some(binding) = self.breaker() else {
            return Ok(None);
        };
        let Some(role) = step.role.as_deref() else {
            return Err(StepError::Unsupported {
                capability: format!(
                    "the agent step `{}` declares no `role`, so the breaker has no model family to \
                     key its dispatch on (D-7; R4 = ADR-0176 decision 4)",
                    step.id
                ),
            });
        };
        binding
            .pack()
            .require_family_for_role(role)
            .map(Some)
            .map_err(|e| StepError::Unsupported {
                capability: format!(
                    "the agent step `{}`: {e} (R4 = ADR-0176 decision 4)",
                    step.id
                ),
            })
    }

    /// **R1 (M8).** Whether a `tool` step's verb row declares `side_effecting = true`. A verb with no
    /// row, or no registry, is not: it refuses by name before it could write anything.
    fn side_effecting(&self, step: &Step) -> bool {
        let run = step.run.as_deref().unwrap_or_default();
        crate::workflow::memo::verb_of(run)
            .and_then(|verb| self.tool_routing().registry().and_then(|r| r.entry(verb)))
            .is_some_and(|entry| entry.side_effecting)
    }

    /// **Execute one step, and let its kind choose the arm.**
    ///
    /// The match is **total with no wildcard**, on this tree's established idiom
    /// (`workflow/budget.rs`, `rigor.rs`; `retry.rs`'s `ac028_policy_match_has_no_wildcard` is the
    /// precedent lint). A sixth kind would be `E0004` here rather than a silent default.
    ///
    /// *(2026-09-26, `s8d` build, package P4.)* Over an EMPTY scope: every `${<step>…}` a guard or an
    /// input reads is a step that has not run, so a mandatory one refuses by name. A caller with
    /// prior outputs uses [`Self::execute_step_in`]; the walk binds its own scope.
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
        self.execute_step_in(
            step,
            ctx,
            &super::reference::Scope::new(std::slice::from_ref(step)),
        )
    }

    /// [`Self::execute_step`] against a `scope` the caller has bound (R2): the step's `when`, its
    /// `inputs` and, for a `check`, its `expr` resolve through `runtime::reference` against it. No
    /// item journal is read, so an `artifact:` input finds no `emit` binding.
    ///
    /// # Errors
    ///
    /// As [`Self::execute_step`].
    pub fn execute_step_in(
        &self,
        step: &Step,
        ctx: &predicate::Context,
        scope: &super::reference::Scope,
    ) -> Result<StepVerdict, super::RuntimeError> {
        let site = StepSite {
            item: "",
            activity: "",
            events: &[],
            verdict: true,
            guard: None,
        };
        self.run_step(step, ctx, scope, &site)
            .map(|run| run.verdict)
    }

    /// [`Self::execute_step_in`], returning the step's typed output beside its verdict: a `tool`
    /// step's `(kind, value)` — what a `side_effecting` step commits (R1) — or `None`. *(Added
    /// 2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed: a lone review route's
    /// `Workspace` and `Verdict` are observable without a walk.)*
    ///
    /// # Errors
    ///
    /// As [`Self::execute_step`].
    pub fn execute_step_output(
        &self,
        step: &Step,
        ctx: &predicate::Context,
        scope: &super::reference::Scope,
    ) -> Result<(StepVerdict, Option<(String, serde_json::Value)>), super::RuntimeError> {
        let site = StepSite {
            item: "",
            activity: "",
            events: &[],
            verdict: true,
            guard: None,
        };
        self.run_step(step, ctx, scope, &site)
            .map(|run| (run.verdict, run.output))
    }

    /// The resolver's environment for one step, over this runtime's routing and read seam.
    fn with_env<R>(
        &self,
        site: &StepSite<'_>,
        f: impl FnOnce(&super::reference::Env<'_>) -> R,
    ) -> R {
        let feed = |verb: &str| self.call_feed(verb, site);
        let routing = self.tool_routing();
        let env = super::reference::Env {
            item: site.item,
            stage: self.stage(),
            activity: site.activity,
            events: site.events,
            values: routing.values(),
            registry: routing.registry(),
            feed_table: routing.feed_table(),
            sources: routing.sources(),
            seam: self.read_seam(),
            feed: &feed,
            run_branch: routing.run_branch(),
        };
        f(&env)
    }

    /// **A feed's producing verb** (D21-43 N14; `s8c`'s `resolve_source` names it): a `toolchain`
    /// row with no inputs, called through `run_verb` under the row's own `${toolchain.*}` bound and
    /// typed through the closed decode table. Any other route refuses by name.
    ///
    /// *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-106 ruling 1; the
    /// render record ADR-0189 decision 12, proposed.)* **Widened by exactly one shape**: a `store`
    /// row with no inputs (`backlog`) is served by the ONE store-route seam the step-grain arm uses,
    /// and its output still passes the closed decode table against the row's `outputs`. Every other
    /// combination keeps the refusal below, byte-unchanged.
    fn call_feed(&self, verb: &str, site: &StepSite<'_>) -> Result<serde_json::Value, String> {
        let routing = self.tool_routing();
        let entry = routing
            .registry()
            .and_then(|r| r.entry(verb))
            .ok_or_else(|| format!("the feed verb `{verb}` has no registry row"))?;
        if entry.route == Route::Store && entry.inputs.is_empty() {
            let output = routing
                .store_route()
                .serve(&StoreCall {
                    verb,
                    inputs: &BTreeMap::new(),
                    item: site.item,
                    stage: self.stage(),
                    activity_id_at_version: site.activity,
                    run_id: self.run_id(),
                })
                .map_err(|e| format!("{e:?}"))?;
            decode_verb_output(&entry.outputs, &output).map_err(|e| format!("{e:?}"))?;
            return Ok(output.value);
        }
        if entry.route != Route::Toolchain || !entry.inputs.is_empty() {
            return Err(format!(
                "the feed verb `{verb}` takes the `{}` route with {} declared input(s); a feed is \
                 produced by a `toolchain` row with no inputs",
                entry.route.as_str(),
                entry.inputs.len()
            ));
        }
        let bound = routing
            .values()
            .resolve(&entry.timeout)
            .map(StepBound::Secs)
            .map_err(|r| r.to_string())?;
        let output = self
            .ports()
            .toolchain
            .run_verb(&VerbCall {
                verb,
                inputs: &BTreeMap::new(),
                bound,
            })
            .map_err(|e| format!("{e:?}"))?;
        decode_verb_output(&entry.outputs, &output).map_err(|e| format!("{e:?}"))?;
        Ok(output.value)
    }

    /// **One step, resolved and executed** — the guard, the inputs, and the kind's arm.
    fn run_step(
        &self,
        step: &Step,
        ctx: &predicate::Context,
        scope: &super::reference::Scope,
        site: &StepSite<'_>,
    ) -> Result<StepRun, super::RuntimeError> {
        self.with_env(site, |env| {
            let refuse = |e: StepError| StepRun {
                verdict: StepVerdict::Failed(e),
                output: None,
                inputs: BTreeMap::new(),
                held_false: false,
                dispatched: false,
                guard_null: false,
            };
            // (1) THE GUARD, its `${…}` resolved through the scope (ADR-0136; R2).
            let mut resolve_ref =
                |text: &str| super::reference::resolve_text(env, scope, &step.id, "when", text);
            let held = site
                .guard
                .map_or_else(|| guard_holds(step, ctx, &mut resolve_ref), Ok);
            match held {
                Ok(tri @ (Tri::False | Tri::Null)) => {
                    return Ok(StepRun {
                        verdict: StepVerdict::Skipped,
                        output: None,
                        inputs: BTreeMap::new(),
                        held_false: false,
                        dispatched: false,
                        guard_null: tri == Tri::Null,
                    });
                }
                Err(e) => return Ok(refuse(e)),
                Ok(Tri::True) => {}
            }
            // (2) THE INPUTS (R2 decision 1). A `check`'s refusal travels INTO its arm, so the arm
            // is the one place a `check` is decided or refused; every other kind refuses here.
            let resolved = super::reference::resolve_inputs(env, scope, step);
            // A `check`'s or a `tool` step's refusal travels INTO its arm (the `tool` arm's registry
            // lookup comes first, ADR-0170 decision 3); every other kind refuses here.
            let inputs = match (&resolved, step.kind) {
                (Ok(inputs), _) => inputs.clone(),
                (Err(_), StepKind::Check | StepKind::Tool) => BTreeMap::new(),
                (Err(e), _) => return Ok(refuse(e.to_step_error())),
            };
            // (3) A `check`'s own context (R2 decision 6): the selection's, with its inputs bound.
            let check_ctx = match (&resolved, step.kind) {
                (Ok(inputs), StepKind::Check) => {
                    let parsed = step
                        .expr
                        .as_deref()
                        .and_then(|src| predicate::parse(src).ok());
                    Ok(super::reference::check_context(
                        ctx,
                        inputs,
                        parsed.as_ref().map(|p| &p.expr),
                    ))
                }
                (Err(e), StepKind::Check) => Err(e.to_step_error()),
                _ => Ok(predicate::Context::new()),
            };
            let mut output: ToolOutput = None;
            let mut held_false = false;
            // m14 (R4FIX-8d): set by the tool arm once its port, verb or route call is MADE, on
            // success or error; a refusal before the call leaves it `false`.
            let mut dispatched = false;
            let verdict = match step.kind {
                // The only place a model runs. The dispatch itself stays in `run_activity`'s retry
                // loop, unchanged, so AC-013's re-entry and `kill_and_resume_n_step` are
                // untouched; this arm names the kind and defers to it.
                StepKind::Agent => StepVerdict::Continue,
                StepKind::Tool => match self.run_tool_step(
                    step,
                    &resolved,
                    env,
                    site.verdict,
                    ReviewSite {
                        ctx,
                        scope,
                        item: site.item,
                        activity: site.activity,
                    },
                    &mut dispatched,
                ) {
                    Ok(value) => {
                        output = value;
                        StepVerdict::Continue
                    }
                    Err(e) => StepVerdict::Failed(e),
                },
                // R2 decision 6: `true` continues; `false` goes to `on_false` (§8d.4, refused by
                // name until R3 routes it); `null` abstains; an error or an unresolvable input
                // refuses by name and is NEVER read as false.
                StepKind::Check => match check_ctx.and_then(|c| check_verdict(step, &c)) {
                    Ok(Tri::True) => StepVerdict::Continue,
                    Ok(Tri::False) => {
                        held_false = true;
                        StepVerdict::Failed(check_did_not_hold(step))
                    }
                    Ok(Tri::Null) => StepVerdict::Failed(check_abstained(step)),
                    Err(e) => StepVerdict::Failed(e),
                },
                // The kernel writes the mark and exits 0, resuming into the same step (design §4.5).
                //
                // ⚠️ **This comment used to say the mark was `Runtime::step`'s `OpenGate` arm's,
                // and that was FALSE for a catalog gate step** — `OpenGate` is only ever reached
                // from `selection.gates()`. ADR-0145 makes it true by having `run_activity_steps`
                // call `workflow::gate::reach` and append `runtime::gate_opened_event` itself. The
                // MINTING is deliberately not here: this arm names the kind and has no `GateSite`,
                // exactly as the `agent` arm names the kind and defers the dispatch to
                // `run_activity`.
                StepKind::Gate => StepVerdict::Gate,
                // Terminal (design §4.5). The walk binds the `emit` (R2 decision 7) and appends
                // the completion row that carries the binding.
                StepKind::Emit => StepVerdict::Emitted,
            };
            Ok(StepRun {
                verdict,
                output,
                inputs,
                held_false,
                dispatched,
                guard_null: false,
            })
        })
    }
}

/// Where a step runs, for the resolver: the item, the activation, the item's journal, and whether
/// the tool arm reads its interim verdicts (`read_by_a_later_check`).
struct StepSite<'a> {
    item: &'a str,
    activity: &'a str,
    events: &'a [rk_types::event::Event],
    verdict: bool,
    /// *(dated 2026-09-27, `s8d` build, package D2176-8d; D21-76.)* A guard verdict the walk
    /// REPLAYS from its journal (a `gate` step's `when`): `Some` stands for the guard and it is
    /// not re-evaluated. `None` everywhere else: the guard is evaluated. *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)*
    /// Three-valued: the walk passes only a replayed `true` here (a replayed `false` or `null`
    /// walks past the gate before the step runs).
    guard: Option<Tri>,
}

/// What one step's execution produced: its verdict, a tool step's typed output, and its resolved
/// `inputs` (which an `agent` step's dispatch carries on its spec).
struct StepRun {
    verdict: StepVerdict,
    output: ToolOutput,
    inputs: BTreeMap<String, serde_json::Value>,
    /// *(P5; R3.)* A `check` that evaluated `false`: the walk routes it through its `on_false`;
    /// a lone `execute_step` keeps the by-name refusal its verdict carries.
    held_false: bool,
    /// *(dated 2026-09-27, `s8d` build, package R4FIX-8d; refuting round 4's spec-conformance
    /// major.)* A `tool` step whose port, verb or route call was MADE — on success OR error — made
    /// one dispatch, which charges its pass (§8d.3 m14, ADR-0175 decision 7). A refusal before the
    /// call (the registry lookup, the selection, an unresolved input, the route) made none.
    dispatched: bool,
    /// *(dated 2026-09-27, `s8d` build, package D2177-8d; ADR-0174 decision 7.)* The step was
    /// skipped because its `when` evaluated `null`. ~~which is not a decision: a gate's `null` `when`
    /// is not journaled, and a later walk evaluates it again.~~ *(dated 2026-09-28, `s8e` build, package S8E-RULINGS; D21-92; ADR-0187, proposed.)*
    /// A gate's `null` `when` continues the walk, so it is journaled as `null` and replayed.
    guard_null: bool,
}

// ---- R3 and R2 decision 5 (`s8d` package P5): the loop executor's state. ------------------------

/// What one walk holds as it runs: the report it accumulates, the activation's scope, the item's
/// journal (re-read after each commit), the first agent step's id (R1's pre-R1 binding rule), the
/// loops' state, and how many dispatches the step just run MADE (the pass charge's input, m14;
/// `0` for a served, skipped or non-dispatching step — ADR-0122 decision 1).
struct Walk {
    report: ActivityReport,
    scope: super::reference::Scope,
    journal: Vec<rk_types::event::Event>,
    first_agent: Option<String>,
    loops: Loops,
    dispatched: u32,
}

impl Walk {
    /// The walk's report at a halt.
    fn halt(self, now: JournaledNow, halt: Halt) -> ActivityReport {
        let held = self.report.holds_a_commit();
        match halt {
            // A step's own failure (a `check` that did not hold and routes to a refusal, a tool
            // port's error, a guard it cannot decide): ADR-0144's shape after a commit, and a
            // refusal outright before one.
            Halt::Failed(err) if held => self.report.with_step_failure(err),
            // *(dated 2026-09-27, `s8d` build, package R4FIX-8d.)* The fresh refusal keeps the
            // walk's pass charges: a step dispatched before the halt stays charged (m14).
            Halt::Failed(err) | Halt::Fresh(err) => {
                ActivityReport::refused(now, err).with_charges_of(self.report)
            }
            // The walk's own refusal, keeping every count the walk read.
            Halt::Stop(err) if held => self.report.with_step_failure(err),
            Halt::Stop(err) => self.report.refusing(err),
            Halt::AsIs => self.report,
            Halt::Gated(token) => {
                if self.report.dispatches() > 0 || held {
                    self.report.with_gate(token)
                } else {
                    ActivityReport::gated(now, token).with_charges_of(self.report)
                }
            }
        }
    }
}

/// The values every step of one walk reads, bundled.
#[derive(Clone, Copy)]
struct WalkEnv<'a> {
    item: &'a str,
    activity: &'a str,
    /// The DAG node id, which is NOT a `[[step]]` id.
    node: &'a str,
    spec: Option<&'a rk_types::activity::ActivitySpec>,
    step_specs: Option<&'a BTreeMap<String, rk_types::activity::ActivitySpec>>,
    rigor: crate::rigor::Rigor,
    ctx: &'a predicate::Context,
    gate: GateSite<'a>,
    cap: AttemptCap,
    now: JournaledNow,
}

/// The loops' state: the loop-back base frame, the counts each bound is checked against, the
/// `repeat` in progress, and which escalate questions a step's exhausted `repeat` has raised.
#[derive(Debug, Default)]
struct Loops {
    /// Empty, or one `loop_back:<n>` frame: the walk's current re-entry (a loop-back or an escalate
    /// re-entry), under which every step it re-runs commits.
    base: IterationPath,
    /// Every re-entry this walk took, in order; the base frame's counter.
    reentries: u32,
    /// Loop-backs taken per `check` step, bounded by its `max`.
    loop_backs: BTreeMap<String, u32>,
    /// Escalate re-entries taken per gate kind: the n-th reads the n-th live reach.
    escalations: BTreeMap<String, u32>,
    /// The `repeat` in progress.
    repeat: Option<Repeat>,
    /// Escalate question → the index of the step whose `repeat` exhausted and escalated to it.
    exhausted: BTreeMap<String, usize>,
}

impl Loops {
    /// The iteration path a step at `index` runs under: the base frame, and the `repeat` frame when
    /// the step lies in the body of the pass in progress.
    fn path_at(&self, index: usize) -> IterationPath {
        match &self.repeat {
            Some(r) if r.start <= index && index <= r.end => self
                .base
                .pushed(crate::workflow::idem::Frame::Repeat(r.pass)),
            _ => self.base.clone(),
        }
    }

    /// A re-entry: a new loop-back frame, no `repeat` in progress, and no escalation pending.
    fn reenter(&mut self) {
        self.reentries = self.reentries.saturating_add(1);
        self.base =
            IterationPath::root().pushed(crate::workflow::idem::Frame::LoopBack(self.reentries));
        self.repeat = None;
        self.exhausted.clear();
    }
}

/// One `repeat` in progress: its body `start..=end` (D21-48 X6), its pass and its `max`.
#[derive(Debug, Clone, Copy)]
struct Repeat {
    start: usize,
    end: usize,
    pass: u32,
    max: u32,
}

/// What one step did to the walk.
enum Flow {
    /// Run on to the next step (or the `repeat`'s `until`).
    Next,
    /// Re-enter the step at this index, under the frame just opened.
    Jump(usize),
    /// End the walk.
    Halt(Halt),
    /// The `emit` committed the activation.
    Complete(super::activity::CommitOutcome),
}

/// How a walk ends when it does not complete.
enum Halt {
    /// A step's own failure: `step_error` after a commit, `ActivityReport::refused` before one.
    Failed(StepError),
    /// The walk's own refusal: `step_error` after a commit, `refusing` before one.
    Stop(StepError),
    /// The dispatch's own report, unchanged (Mechanism B's class travels on it).
    AsIs,
    /// A refusal that discards the walk's report (a gate step with no question).
    Fresh(StepError),
    /// A gate opened (or already open): the run exits 0 at it (design §4.5).
    Gated(rk_types::digest::Digest),
}

/// **A `repeat`'s `until`, evaluated after its body** (D21-48 X6): its `${…}` resolved through the
/// resolver as PATHS bound to their values (never the literal `null`, ADR-0172 clause 3), then the
/// one `predicate::evaluate`. An unresolvable reference or an evaluation error refuses by name.
fn until_holds(
    step: &Step,
    until: &str,
    ctx: &predicate::Context,
    resolve_ref: &mut dyn FnMut(&str) -> Result<serde_json::Value, super::reference::RefError>,
) -> Result<Tri, StepError> {
    let parsed = predicate::parse(until).map_err(|e| StepError::Unsupported {
        capability: format!(
            "the `repeat.until` of step `{}` does not parse ({e:?}): {until}",
            step.id
        ),
    })?;
    let (expr, bound) = super::reference::bind_refs(&parsed.expr, ctx, resolve_ref)
        .map_err(|e| e.to_step_error())?;
    predicate::evaluate(&expr, &bound)
        .map(|o| o.value)
        .map_err(|e| StepError::Unsupported {
            capability: format!(
                "the `repeat.until` of step `{}` cannot be evaluated ({e:?}): {until}",
                step.id
            ),
        })
}

/// Bind a committed step activation's output from its journalled row (a SERVED step, or one just
/// committed); a row with no `result` marks the step run.
fn bind_committed(
    scope: &mut super::reference::Scope,
    journal: &[rk_types::event::Event],
    step: &Step,
    key: &crate::workflow::idem::CommitKey,
) {
    match super::activity::committed_step_result(journal, key) {
        Some(value) => scope.bind_output(&step.id, value.clone()),
        None => scope.mark_ran(&step.id),
    }
}

/// **R2 (§8d.2): an agent step's resolved `inputs`, appended to ITS spec at dispatch.** The binder's
/// keys (`worktree`, `item`) are never overwritten: a catalog binding of the same name is refused by
/// name.
fn with_resolved_inputs(
    own: &rk_types::activity::ActivitySpec,
    step: &str,
    inputs: &BTreeMap<String, serde_json::Value>,
) -> Result<rk_types::activity::ActivitySpec, StepError> {
    let mut spec = own.clone();
    for (name, value) in inputs {
        if spec.inputs.iter().any(|(bound, _)| bound == name) {
            return Err(StepError::Unsupported {
                capability: format!(
                    "the `agent` step `{step}` declares the input `{name}`, which the binder \
                     already set on its spec; a binder's key is never overwritten (`s8d` §8d.2)"
                ),
            });
        }
        spec.inputs.push((name.clone(), value.clone()));
    }
    Ok(spec)
}

/// An answered gate's option and admitted attachments (ADR-0045; ADR-0129 decision 4): the
/// answer the fold holds for `token`, and the `artifact.write` rows `answer` appended right after
/// it. *(2026-09-26, `s8d` build, package P5: keyed on the TOKEN, not the kind's latest, so the
/// escalate re-entry reads the reach it consumes.)*
fn gate_answer_at(
    state: &crate::workflow::fsm::RunState,
    journal: &[rk_types::event::Event],
    item: &str,
    token: &rk_types::digest::Digest,
) -> Option<super::reference::GateAnswer> {
    let answer = state.answered_gates().find(|a| a.token() == token)?;
    let mut attachments = BTreeMap::new();
    if let Some(at) = journal.iter().position(|e| {
        e.kind == rk_types::event_kind::EventKind::GateAnswered
            && e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token.as_str())
    }) {
        for row in journal[at + 1..]
            .iter()
            .take_while(|e| e.kind == rk_types::event_kind::EventKind::ArtifactWrite)
            .filter(|e| e.payload.get("item").and_then(serde_json::Value::as_str) == Some(item))
        {
            if let Some(kind) = row.payload.get("kind").and_then(serde_json::Value::as_str) {
                attachments.insert(kind.to_owned(), row.payload.clone());
            }
        }
    }
    Some(super::reference::GateAnswer {
        option: answer.verdict().to_owned(),
        attachments,
    })
}

/// `build`'s verdict for a `tool` step whose only channel is its `Result` (s8c §8c.2; record T =
/// ADR-0169 decision 8, proposed). A `passing` report is `Ok(())`. A `failing` one is a red build:
/// the step did not succeed, so it is `NonZero` with no exit code (the build ran and reported;
/// no status was lost) and the digest of the report's canonical JSON, so the journal names what
/// was read. This is the interim reading until `s8d` binds the typed output to
/// `${step.outputs}` and the catalog's own `status == 'passing'` check reads it.
///
/// *(Added 2026-09-26, s8c.)*
fn build_verdict(report: &BuildReport) -> Result<(), StepError> {
    match report.status {
        BuildStatus::Passing => Ok(()),
        BuildStatus::Failing => Err(StepError::NonZero {
            code: None,
            stderr_digest: report_digest(report),
        }),
    }
}
