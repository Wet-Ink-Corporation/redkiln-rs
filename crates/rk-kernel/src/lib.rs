//! `rk-kernel` — the workflow side of the split: the selection engine, the predicate grammar and
//! its three-valued evaluator, the registry lints, the memo, the budgets and the replay.
//!
//! **This crate is Step 7b's, and Step 7b is six to eight sessions.** What is here at any moment is
//! a slice, and the slices are `runbook/phase-1/s7b-kernel.md` §7b.3's. **Slices 1 and 2 are on
//! disk:** the crate skeleton (§7b.4), the determinism gate (§7b.5) and the ADR-0103 grammar with
//! its evaluator (§7b.6); and the **two fail-closed facet lints** with their two fixture sets
//! (§7b.7) plus **rigor monotonicity** (§7b.8); and [`select`] carries **steps 1-4 of the seven**
//! (§7b.9) — rigor, the required set, the short-circuit and the producer. **Steps 5 (validate),
//! 6 (execute) and 7 (deviation) are not built and are not stubbed**, and neither is the kernel
//! runtime (Slice 4): a module that exists and returns nothing reads as built to the next reader,
//! and `00-preamble.md`'s golden constraint 10 is that powerless is reported as powerless. The
//! entry point is [`select::select`], which is steps 1-5; step 6 is `Selection::plan` and step 7 is
//! `select::step7`, and neither runs anything.
//!
//! **Corrected 2026-09-14.** The sentence above still reads *"neither is the kernel runtime (Slice
//! 4)"*, and that is now false in four parts: **Slices 4a, 4b, 4c and 4d are on disk** —
//! [`workflow::budget`], the three counters AC-020 separates, lint rule 8's declared minimum
//! (AC-021), the runtime fan-out charge (AC-022), the appetite-raise refusal AC-023 and AC-389
//! share, the exhaustion gate AC-025 names with its kernel-local `BudgetExhausted`, the durable mark
//! AC-024's wall clock owes, and ADR-0122's single inner-attempt charge site;
//! [`workflow::envelope`], the pack's `N`, `K` and per-item token appetite, which this crate reads
//! and refuses to default; and [`workflow::retry`], the policy over ADR-0112's eleven variants
//! (AC-027, AC-028), the full-jitter backoff envelope and its two **kernel** constants against the
//! pack's `N` (AC-029), the inner-attempt accounting (AC-026) and the one door an activity outcome
//! reaches a retry decision through (AC-032); and — **Slice 4d** — [`workflow::breaker`], the
//! K-consecutive-`Platform` counting rule, the per-key isolation AC-034 calls *"never trips
//! globally"*, the store-derived state AC-035's lint names and the two source lints those criteria
//! specify, plus [`workflow::degrade`], the substitution refusal (AC-036), the
//! `Degraded { from, to, why }` record in both of AC-037's sinks, and AC-038's five-row per-rigor
//! policy table. **Neither gets a `pub use` here**, and that is deliberate: the register files
//! AC-033–AC-035 under `rk-kernel::retry` and AC-036–AC-038 under *"`rk-kernel` policy"*, so
//! neither area name is `breaker` or `degrade`, and a re-export at a name no record uses would
//! invent one — [`workflow::envelope`] is already on disk under the same rule.
//!
//! **And — Slice 4e — [`workflow::fsm`].** The sentence that stood here said *"the rest of the
//! runtime — the memo, **the FSM** and the port boundary — is still absent and still not stubbed"*,
//! and the middle term is now false. `s7b-kernel.md` recorded that half **NOT BUILDABLE** twice,
//! both times for one reason — *"there is no FSM"* — and there is one now: `RunState` as a **pure
//! fold** over the recorded journal, the step-grain `NextAction`, the **stage-grain** `StageAction`
//! with AC-161's tier filter inside the function, the five-state `TerminalState` AC-008's
//! *"a kernel that does nothing at all"* cannot fake, AC-039's gate record with its **void, not
//! reconciled** rule, and AC-007's `Divergence` and typed halt. **It gets no `pub use` either**, for
//! [`workflow::breaker`]'s reason: AC-006–AC-008 are filed under `rk-kernel::replay`, AC-039 under
//! `rk-kernel::budget`, and AC-161/AC-164 under `rk-store` + `rk-git` — no register area is named
//! `fsm`. **Every one of those ids is ARMED and none is claimed**: the recorded side of AC-008's and
//! AC-006's comparison is `run.json`'s and is Slice 4f's, AC-161/AC-164's route from git trailers is
//! Step 7d's, and AC-039 is a **human gate** §7b.15 excludes outright. [`workflow::fsm`]'s header
//! carries the table. The memo and the port boundary are still absent and still not stubbed.
//!
//! **And — Slice 4f — [`replay`] consumes it.** The sentence directly above said *"the recorded
//! side of AC-008's and AC-006's comparison is `run.json`'s and is Slice 4f's"*, and that is now
//! done: `run.json` carries `terminal_state` and `next_action`, [`replay::ReplayReport`] carries
//! the recomputed pair, [`replay::ReplayError`] gains
//! [`replay::ReplayError::NextActionDiverged`] and
//! [`replay::ReplayError::TerminalStateDiverged`] as typed halts over
//! `rk_types::event_kind::ReplayDivergence`, and a log written before this layout is **refused**
//! rather than replayed with nothing to compare. **AC-006's five assertions are all measured.**
//!
//! **And — Slice 4i — AC-008's dispatch half stopped being vacuous.** *(Corrected 2026-09-14. This
//! paragraph read "**AC-008 is still ARMED** — its GIVEN names *\"a run that dispatched N
//! activities\"* and N is 0 on the only recorded run, because a journal with dispatches needs the
//! dispatcher Slice 4h builds." Slice 4h built it, Slice 4i drove it, and **N is 2**.)* [`record`]
//! now takes ports and runs the FSM loop through a bound `Harness`, so the recorded journal carries
//! real `dispatch` and `activity_committed` rows; [`replay`] serves those rows **from the journal**
//! and dispatches nothing, which is AC-008's *"all N results served from the journal, zero harness
//! dispatches"* as an architecture rather than an assertion. The five `ActivitySpec` fields are a
//! **recorded input** (`inputs/specs.json`) and not a binder output — the binder is unbuilt — and a
//! record routing spec assembly is **OWED**. [`replay`]'s header carries the table and the reading. **AC-161's equality is measured at stage grain and its route *"from git alone"* is not**;
//! **AC-164's `doctor`-classification half has no home in this stage at all** (residual 10l) and
//! neither id is reported as covered.
//!
//! **And — Slice 4g — [`memo`].** Both paragraphs above end on the same clause — *"The memo and the
//! port boundary are still absent and still not stubbed"* — and its first half is now false. What
//! is on disk: the eight runtime facts `rk_store::memo::StepKeyComponents` is **assembled** from and
//! nothing that re-derives a key (ADR-0119 clause 1: that struct *"does not move"*, and a second
//! derivation is the accidental equality AC-014/AC-016/AC-019 exist to prevent); the activity-level
//! key's **separate formula** with AC-016's single-card-reference lint and its discriminating third
//! assertion — the activity key is *unchanged* when a step's resolved inputs change; the loop
//! iteration counter that stops loops learning (AC-017); the nine-member `memo_hit` payload AC-019
//! enumerates and the journal identity beneath it; and AC-018's two pack-validation clauses, which
//! are **red against the shipped catalog by design** — nineteen shipped steps across fourteen
//! activities declare the `memo` key ADR-0097 decision 3 withdrew [measured 2026-09-14] and the
//! repairs are Step 7e's, not this stage's. **Component 5's tool branch and component 8's
//! `schema_version` have no source in this tree and neither is invented**; AC-043's `Evicted`
//! decision is not built, because its third state is an amendment ADR-0119 clause 7 owes the
//! register; AC-045 stays ARMED under clause 8. **The port boundary is still absent and still not
//! stubbed.**
//!
//! **And — Slice 4h — [`runtime`], where `rk-ports` is CALLED.** Every paragraph above ends on
//! some form of the same clause, and the last of them said *"**The port boundary is still absent
//! and still not stubbed.**"* It is neither now. `crates/rk-kernel/Cargo.toml` carried a pointer to
//! this commit from Step 7a onward — *"Where `rk-ports` will first be USED … **Delete that line
//! when its call site lands**"* — and the line is deleted. What is on disk: `Runtime`, holding
//! [`rk_ports::root::Ports`] as a **value** the composition root constructed (ADR-0083's
//! Configuration clause); `Runtime::step`, the one method that turns a
//! [`workflow::fsm::NextAction`] into an effect, three of whose six arms append **nothing at all**
//! because ADR-0022, the recorded halt and the empty plan each say so; the retry loop with AC-011's
//! strictly increasing attempt and AC-031's single re-prompt; AC-010's key validator and the
//! [`workflow::idem::PlannedDispatch`] that cannot be constructed without one; AC-012's
//! late-duplicate discard, whose journal re-read and append are **one** lock acquisition;
//! ADR-0114 decision 2's executed-once ledger; AC-525's run-scoped, **log-free** advisory sink; and
//! AC-030's `retry_at` mark. **[`activity`] gets a `pub use`** — AC-010, AC-011 and AC-012 are
//! filed under the area `rk-kernel::activity` — and [`runtime`] itself is a **top-level** module
//! beside [`replay`] rather than a child of [`workflow`], which is the split and not a filing
//! choice: this module dispatches, appends and takes a `Mutex`, and `workflow`'s two `#![deny]`
//! attributes reach every child.
//!
//! **Five halves inside it are ARMED and none is claimed**: AC-013's *"process was killed"* (the
//! boundary here is a dropped `Runtime`, and ADR-0087 clause 2 says *"a reload that reads state
//! back in-process is not a valid implementation of this decision and is not evidence for it"* —
//! on top of AC-013 being **PROVISIONAL**), AC-024's and AC-030's *"exits 0"* (a **process** claim;
//! `rk-cli` is Step 7d), AC-030's *"the next run resumes from the mark"* (it rides on AC-013),
//! AC-031's prose-extraction sub-clause (built, and the register itself calls the criterion
//! compound), and AC-110's ledger coverage, which is **POWERLESS**: AC-110's own THEN is a
//! load-time configuration error and says nothing about `executed: { first, last }`.
//!
//! **And four halves inside Slice 4d are ARMED, not discharged**, each because the value it needs
//! is named by no type and no record: the breaker's model-family key, its durable mark, its expiry
//! (value and home both), and the resolved card's model tier. Each arrives as a caller-supplied
//! argument with one constructor; [`workflow::breaker`]'s and [`workflow::degrade`]'s headers carry
//! the measurements and name the records that are owed.
//!
//! **Two things Slice 2 does not contain, stated so the absence is not read as an oversight.**
//! **AC-072 — the registry-entry loader rule — is a THIRD rule with its own eleven-case fixture
//! set**, the register forbids collapsing it into either lint, and it is **not written**: nothing
//! in `src/` is named for it, so no checkpoint assertion keyed on its name can read [`registry`]'s
//! loader as that rule. And **both lints' CLI surfaces are absent**: `redkiln lint
//! --facet-stability` and `--facet-provenance` are `rk-cli`'s (**Step 7d**) over `.redkiln/`
//! (**Step 7e**), so those halves are **ARMED, NOT DISCHARGED**. What is discharged here is the
//! fail-closed behaviour, which §7b.14 says is what Step 7b owes in their place.
//!
//! # The two halves, and why the split is a compile-time fact here
//!
//! [`workflow`] is the deterministic half: stage selection, gate evaluation, retry decisions, DAG
//! order. It may not read the wall clock, draw randomness, perform I/O or iterate an unordered
//! map — **AC-001–AC-004** — and the instrument is not a convention. The root `clippy.toml`'s
//! `disallowed-methods` / `disallowed-types` tables name the calls and the types; that file has
//! **no path or module scoping**, so [`workflow`] carries its own module-level
//! `#[deny(clippy::disallowed_methods)]`, and CI runs `cargo clippy --workspace --all-targets`
//! **gated on its exit code** — never `-D warnings`, which promotes `clippy::nursery` to a group
//! deny ADR-0083 forbids and ADR-0102 restates. Time, randomness and I/O reach the workflow only
//! as journaled activity results.
//!
//! Every in-source lint exception in this crate is written `#[expect(clippy::<lint>, reason = "…")]`
//! and never `#[allow]`, so an override that stops being needed fails the build (ADR-0102
//! decision 3).
//!
//! # The predicate grammar
//!
//! [`predicate`] is **ADR-0103's** grammar and nothing else's: precedence `not` > comparison/`in` >
//! `and` > `or`, left-associative; an unquoted identifier in value position is a literal;
//! three-valued `null`; `any` over `null` or an empty list is `false` while `all` and `none` are
//! `true`; and a closed `on_false` form set. It is checked against `spikes/p9/grammar-probe.py`,
//! the S-GRAMMAR probe F42 ran over every live predicate, by a differential test rather than by
//! assertion.
//!
//! # What this crate's dependency table says by omission
//!
//! No async runtime, directly or transitively (**AC-528**, ADR-0076), and no `rk-adapter-*` edge
//! (**AC-484**). `rk-git` is a dependency **as a concrete crate** and not as a port (**AC-483**;
//! ADR-0003:57-58). The assertions are over the resolved graph — `cargo tree -p rk-kernel -e
//! normal` — because a manifest sees direct dependencies only.

//! # The two fail-closed facet lints
//!
//! [`facet_stability`] is **AC-071**, the unblocking gate, and its subject is `source = "llm"`.
//! [`facet_provenance`] is **AC-520**, the sibling gate F39 forced, and its subject is
//! `source = "derived"`. They are **two rules with two fixture sets**, never one gate with two
//! subjects: AC-520's own record says why — *"a single gate carrying two different subjects, two
//! different records and two different failure meanings is the shape that invites that collapse"*.
//!
//! # Rigor monotonicity
//!
//! [`rigor`] is **AC-105 / SM-035** over **ADR-0081 clause 1's** superset relation, and it is the
//! first measurement of monotonicity by any tool: `grep -ci monoton spikes/p9/lattice.py` is **0**.

pub mod catalog;
pub mod facet_provenance;
pub mod facet_stability;
pub mod predicate;
pub mod registry;
pub mod replay;
pub mod rigor;

/// **The impure shell — AC-009–AC-013, AC-030, AC-031, AC-110, AC-525.**
///
/// A **top-level** module beside [`replay`] and deliberately not a child of [`workflow`]: it
/// dispatches, it appends and it takes a `Mutex`, and `workflow`'s two `#![deny]` attributes reach
/// every child of it. `src/replay.rs`:10-15 is the precedent and its sentence is the whole argument
/// — *"putting them inside the pure function would make the criterion false by construction."*
///
/// It is also not under [`select`], and that placement is what lets Slice 4 build the
/// workflow/activity split **without assigning the catalog loader a side**: nothing reachable from
/// here reaches `Catalog::from_dir`, so the open question [`workflow`]'s header records stays open
/// rather than being answered by a module layout (`00-preamble.md`:55).
pub mod runtime;
pub mod select;
pub mod toml_subset;
pub mod workflow;

/// The budget counters at the name the register uses for them.
///
/// Every criterion from AC-020 to AC-025 is filed under the area **`rk-kernel::budget`**, and this
/// re-export is what makes that path resolve. The code itself stays inside [`workflow`], because
/// that module's `#![deny(clippy::disallowed_methods)]` / `#![deny(clippy::disallowed_types)]` reach
/// a child module and would not reach a sibling — so moving the file to satisfy the name would move
/// it out of the determinism gate AC-001-AC-004 names.
pub use crate::workflow::budget;

/// The retry policy at the name the register uses for it.
///
/// AC-027 to AC-032 are filed under the area **`rk-kernel::retry`**, and AC-029's *Home of the
/// numbers* note names it in terms — *"base and cap are **kernel retry-policy constants**"* — so
/// this re-export is what makes the path the register writes resolve. The code stays inside
/// [`workflow`] for [`budget`]'s reason: a sibling module would leave the determinism gate, and a
/// retry policy outside it is precisely the module AC-001's *"retry decision"* clause names.
pub use crate::workflow::retry;

/// The memo at the name the register uses for it.
///
/// AC-014, AC-015, AC-016, AC-017 and AC-019 are every one filed under the area
/// **`rk-kernel::memo`**, so this re-export is what makes the path the register writes resolve.
/// (AC-018's subject tag is *"`rk-kernel` pack lint"* rather than an area, and its rule is here
/// because the field it refuses is a step's.) The code stays inside [`workflow`] for [`budget`]'s
/// reason: a memo key is a pure function of journaled facts, and a sibling module would leave the
/// determinism gate that makes that structural rather than conventional.
pub use crate::workflow::memo;

/// The activity execution loop at the name the register uses for it.
///
/// AC-010, AC-011 and AC-012 are every one filed under the area **`rk-kernel::activity`**, and this
/// re-export is what makes that path resolve. Unlike [`budget`], [`retry`] and [`memo`], the code is
/// **not** inside [`workflow`] and could not be: this module dispatches, appends and takes a lock,
/// and `workflow`'s two `#![deny]` attributes reach every child. The split is the criterion, not a
/// filing convention — `src/replay.rs`:10-15's *"putting them inside the pure function would make
/// the criterion false by construction"*, one module over.
pub use crate::runtime::activity;
