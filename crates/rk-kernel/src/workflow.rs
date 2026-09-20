//! The workflow side of the split — the deterministic half (**AC-001–AC-004**).
//!
//! Stage selection, gate evaluation, retry decisions and DAG order live here, and none of them may
//! read the wall clock, draw randomness, perform I/O or iterate an unordered map. Time, randomness
//! and I/O reach this module only as **journaled activity results**.
//!
//! # The attribute below is the criterion's own named instrument
//!
//! The root `clippy.toml` carries the ban list (`disallowed-methods`, `disallowed-types`), and it
//! has **no path or module scoping** — `clippy.toml:1-3` says so in terms, and `s5` §5.1 says why.
//! The workspace tables are therefore green with or without this module's attribute, and **no grep
//! anywhere in the runbook can detect its absence** except the one Checkpoint 7b anchors on the
//! declaration. AC-001's *Verified by* names it directly: the ban list *"applied to
//! `rk-kernel::workflow` under a module-level `#[deny(clippy::disallowed_methods)]`"*.
//!
//! **The level, not the attribute, is what makes the gate bite**, and the distinction is worth
//! stating because it is where a new member silently leaves the deny set:
//! `clippy::disallowed_methods` is **warn-by-default**, and it is `[workspace.lints.clippy] all =
//! { level = "deny", priority = -1 }` reaching this crate through `Cargo.toml`'s
//! `[lints] workspace = true` that turns a reintroduced clock read into a non-zero exit. Drop that
//! table from the manifest and every lint assertion in Checkpoint 7b passes over an unlinted crate.
//! This attribute is what scopes the criterion to the module the register names; the manifest table
//! is what gives it teeth. Both are asserted.
//!
//! # 🛑 WHAT THIS GATE MEASURES IS AC-001, AND AC-002 AND AC-003 ARE ARMED, NOT MEASURED
//!
//! The reintroduce control below reintroduces **a wall-clock read**, which is AC-001's WHEN. The
//! three sibling criteria share AC-001's *instrument* and not its coverage, and two of them have no
//! entry behind them:
//!
//! - **AC-002** (randomness, non-deterministic ids) — `clippy.toml` carries `rand::random`,
//!   `rand::thread_rng` and `uuid::Uuid::new_v4`, and **no fixture reintroduces one**, so the ban is
//!   declared and unexercised. ARMED.
//! - **AC-003** (I/O — *"filesystem read/write, network, process spawn, or a blocking sleep"*) —
//!   its *Verified by* names **`std::fs`, `std::process`, `std::net`, `tokio::time::sleep`,
//!   `thread::sleep`**. `clippy.toml` carries the two sleeps and `std::process::Command`, and
//!   **no filesystem entry and no network entry at all** [measured 2026-09-13]. The filesystem
//!   half of AC-003 is therefore enforced by **nothing**. ARMED.
//! - **AC-004** (unordered iteration) — `HashMap`/`HashSet` are in `disallowed-types` and this
//!   module's `#![deny(clippy::disallowed_types)]` scopes them here; unexercised by a fixture.
//!   ARMED.
//!
//! **And this crate now performs real filesystem I/O in a public API**: `Catalog::from_dir` calls
//! `std::fs::read_dir` and `std::fs::read_to_string` (`catalog.rs`:223, :241), reached from
//! [`crate::catalog`]'s own surface. Whether that sits on the workflow side of the split is a
//! question **no record answers** — AC-003's GIVEN is *"any function on the workflow side"* and
//! nothing assigns `rk-kernel::catalog` to a side. **Two things are owed and neither is taken
//! here** (`00-preamble.md`:55): `clippy.toml` gaining the filesystem and network entry points the
//! criterion names — a **workspace-wide** table, so `rk-store` and `rk-git` would each need an
//! `#[expect(clippy::disallowed_methods, reason = …)]` per ADR-0102 decision 3 — and a record
//! settling which side the catalog loader is on, because if it is the workflow side the load path
//! moves behind the boundary. `clippy.toml` is outside this session's editable set.
//!
//! # Its positive control, which is committed rather than observed and reverted
//!
//! [`reads_the_wall_clock`] sits behind the non-default `workflow-clock-reintroduce` feature **of
//! this crate**, so a workspace-scoped clippy selects it:
//!
//! ```text
//! green:  cargo clippy --workspace --all-targets
//! RED:    cargo clippy --workspace --all-targets --features rk-kernel/workflow-clock-reintroduce
//! ```
//!
//! A grep shows the attribute is *declared*; only a run that goes red shows it *bites*. The
//! fixture is committed on `tests/fixtures/rk-clock-read/`'s precedent (`s5` §5.1a,
//! DECISIONS-2026-09-12 E4) because a reverted observation is not an assertion. Checkpoint 7b
//! asserts **both** halves of that run — a non-zero exit **and** `disallowed_method` in the
//! output — so an unrelated failure cannot read as the pass (`s5`:168-174 is the shape).
//!
//! # What is here now, and what is still not
//!
//! **The sentence that stood here until 2026-09-14 said *"the memo, the budgets, retry, the breaker,
//! degradation and replay are Slice 4's … Neither is stubbed here"*, and half of it is now false.**
//! [`budget`] is on disk: the three counters AC-020 separates, lint rule 8's declared minimum
//! (AC-021), the runtime fan-out charge (AC-022), the appetite-raise refusal AC-023 and AC-389
//! share, and — **added 2026-09-14, Slice 4b** — the exhaustion gate AC-025 names, its kernel-local
//! `BudgetExhausted` and the durable mark AC-024's wall clock owes. [`envelope`] is on disk beside
//! it: `N`, `K` and the per-item token appetite, the numbers §7b.10 says are *"declared in the pack
//! and refused as code defaults"*. Both are **child modules of this one**, so the two `#![deny]`
//! attributes below cover them — lint levels propagate down the module tree — and that inheritance
//! is the reason the budget counters live here rather than in a sibling module named for the
//! register's area.
//!
//! **Corrected again 2026-09-14, Slice 4c.** The paragraph that stood here said *"Still **not built
//! and not stubbed**: retry (AC-026–AC-032) …"* and *"The two kernel retry constants (base 2 s, cap
//! 5 min) are NOT in [`envelope`] and are not anywhere yet"*. The second sentence is still true of
//! [`envelope`] and is now false of this module: [`retry`] is on disk, and
//! `BACKOFF_BASE_SECS` / `BACKOFF_CAP_SECS` are where PIA-055 puts them. With them come the policy
//! over ADR-0112's eleven variants (AC-027, AC-028), the full-jitter envelope and the attempt cap
//! against the pack's `N` (AC-029), ADR-0122's inner-attempt accounting (AC-026) and the one door an
//! activity outcome reaches a retry decision through (AC-032). A pack key for the base or the cap
//! would violate the register exactly as a code default for `N` does, and the two mistakes are one
//! edit apart.
//!
//! **Corrected once more 2026-09-14, Slice 4d.** The sentence that stood here said *"Still **not
//! built and not stubbed**: the breaker (AC-033–AC-035), degradation (AC-036–AC-038), the memo
//! (AC-014–AC-019) and the FSM"*, and half of that list is now false. [`breaker`] and [`degrade`]
//! are on disk: the K-consecutive-`Platform` counting rule with its reset rule, the per-key
//! isolation AC-034 calls *"never trips globally"*, the store-derived state AC-035's own lint names,
//! the two source lints those two criteria specify, the substitution refusal AC-036 owes, the
//! `Degraded { from, to, why }` record in both of AC-037's sinks, and the five-row per-rigor policy
//! table AC-038 asks for.
//!
//! **What those two modules do NOT contain is as load-bearing as what they do**, and each absence is
//! a criterion half reported ARMED rather than an omission: the breaker's **durable mark**
//! (`EventKind` is closed at 29 and ADR-0116 says *"the kind is minted by record or not at all"*),
//! the **model-tier binding** (the binder is built by no stage — `PROGRESS.md`:279; D-11 routes it
//! to 7c), and any **ordering** over model tiers. Each is taken as a caller-supplied argument with
//! exactly one constructor, so a later record narrows the shape rather than rewriting it. Each
//! module's header states which record is owed.
//!
//! **Corrected 2026-09-14 (operator decision D-7), and the two that left are named rather than
//! dropped.** This list also carried the breaker's **key** — *"no model family exists in `rk-types`,
//! `rk-ports` or this crate, and the catalog's `family` is ADR-0058's **activity** family"* — and the
//! breaker's **expiry value and its home** — *"§7b.10's two lists contain neither"*. Both are built:
//! the key is the **pack's** `[role.<name>] family`, resolved from [`crate::catalog::Step::role`]
//! through [`crate::select::Pack::require_family_for_role`], and the expiry is pack-declared at
//! `[breaker] expiry_secs` beside `K`. **Both measurements the old clauses rested on are still true**
//! and were re-taken today — there is still no model family *type* in `rk-types` or `rk-ports`, and
//! §7b.10 still lists no expiry — because what D-7 added is a **pack declaration**, which is a TOML
//! table and not a type, and a **home**, which §7b.10 never fixed. The catalog's `family` is still a
//! different family and is now guarded by a third source lint,
//! [`breaker::family_key_never_from_the_activity_catalog_rule`], plus a behavioural pin over a
//! fixture activity that carries both. What is still OWED is narrower: the five pack key
//! **spellings** are a reading, and the expiry's **value** is sized by no corpus figure.
//!
//! **Corrected a fourth time 2026-09-14, Slice 4e.** The sentence that stood here said *"Still
//! **not built and not stubbed**: the memo (AC-014–AC-019) and **the FSM**"*, and half of that is
//! now false too. [`fsm`] is on disk, and with it the half `s7b-kernel.md`:1103-1109 and :1153-1159
//! each recorded **NOT BUILDABLE** — *"both are the FSM's, and **there is no FSM**"*. What is there
//! now: `RunState` as a **pure fold** over the recorded journal (never a mutable machine — which is
//! what makes AC-035's durable-state rule structural rather than conventional), the step-grain
//! `NextAction` AC-006 assertion 3 compares, the **stage-grain** `StageAction` AC-161 and AC-164
//! compare with the tier filter **inside** the function, the five-state `TerminalState` a kernel
//! that does nothing at all cannot fake, AC-039's gate record bound to
//! `(token, tree_hash, deadline)` with its **void, not reconciled** rule, and AC-007's `Divergence`
//! and its three-arm no-wildcard halt.
//!
//! **And [`fsm`] carries the proof that these five child modules are inside the gate at all.**
//! Every module added since Slice 4a relies on the two attributes below propagating **down** the
//! module tree, and until 2026-09-14 that was asserted in six headers and demonstrated nowhere:
//! `workflow-clock-reintroduce` reintroduces the banned call in **this file**, which carries the
//! attribute, so it cannot tell inheritance from declaration. [`fsm::fsm_reads_the_wall_clock`],
//! behind the new non-default `workflow-fsm-clock-reintroduce`, reintroduces it in a **child**:
//!
//! ```text
//! RED:  cargo clippy --workspace --all-targets --features rk-kernel/workflow-fsm-clock-reintroduce
//! ```
//!
//! [measured 2026-09-14: exit **101**, *"use of a disallowed method `std::time::SystemTime::now`"*
//! at `src/workflow/fsm.rs`, with clippy's own *"note: the lint level is defined here"* pointing at
//! the `#![deny]` **in this file** — the parent's attribute, reached from a child. The same run over
//! `workflow-clock-reintroduce` names the same level source, which is what makes the pair a
//! comparison rather than two copies of one observation.] **No Checkpoint 7b line runs the new
//! feature**; :1396-1398 names only `workflow-clock-reintroduce`, so an owed §7b.14 line is recorded
//! in the slice report and this control is never reported as a checkpoint pass.
//!
//! **What [`fsm`] does NOT contain is four values it needs and cannot compute**, each a
//! caller-supplied argument with one constructor on [`breaker`]'s precedent: the gate's
//! `tree_hash` (ADR-0105 clause 4's, `rk-git`'s), its `deadline` (a journaled instant), **the
//! successor stage** (this system models no stage ordering — `spikes/p9/packs/incident.toml`:1-2
//! says so in its own first comment — so `next_action` answers `Done` rather than inventing one),
//! and the `@version` half of `activity_id@version`, which has no carrier in this tree at all.
//!
//! **Corrected a fifth time 2026-09-14, Slice 4g.** The sentence that stood here said *"Still **not
//! built and not stubbed**: the memo (AC-014–AC-019), and the runtime boundary that would act on
//! what [`fsm`] decides"*, and its first clause is now false. [`memo`] is on disk: the eight runtime
//! facts `rk_store::memo::StepKeyComponents` is **assembled** from — assembled, because ADR-0119
//! clause 1 says that struct does not move and a second derivation in this crate is the accidental
//! equality AC-014/AC-016/AC-019 exist to prevent — the activity-level key's separate formula with
//! AC-016's single-card lint, the nine-member `memo_hit` payload AC-019 enumerates, the journal
//! identity *"executed steps plus `memo_hit` events equals step **activations**"*, and AC-018's two
//! pack-validation clauses.
//!
//! **What [`memo`] does NOT contain, each absence a reported half rather than an omission**: memo
//! key component 5's **tool branch source** (`Toolchain::detect` returns a `bool`, not a version
//! string, and there is no Toolchain adapter at 7b), AC-043's **`Evicted` decision** (its third
//! state is
//! a dated amendment ADR-0119 clause 7 owes the register, and a variant no call site can produce
//! reads as a refusal the system performs), and AC-045's **selection memo key**, which is
//! `rk-kernel::select`'s and stays ARMED under ADR-0119 clause 8. Nothing in it appends: AC-015's
//! *"re-runs"* and AC-019's identity are measured over a **projection** of a decision sequence into
//! the events Slice 4h will write.
//!
//! *(Corrected 2026-09-14: component 8's **`schema_version` source** was the second absence in
//! that list until the operator decision of that date sourced it from
//! `rk_types::activity::ActivitySpec::schema`'s **whole id**. What was retired is the missing
//! source, NOT the missing record: a dated note saying what `schema_version` formally is stays
//! **OWED**, and [`memo`]'s own header carries the decision, the `vN`-parse it rejected and the
//! test that pins it.)*
//!
//! **Corrected a sixth time 2026-09-14, Slice 4h.** The paragraph that stood here said *"Still **not
//! built and not stubbed**: the runtime boundary that would act on what [`fsm`] decides"*, and named
//! three things inside [`retry`] that needed it — *"AC-030's durable mark, AC-031's
//! scripted-responder instrument and the dispatch loop itself"*. **All four are now built, and none
//! of them is in this module.** The boundary is [`crate::runtime`], a **top-level** module beside
//! [`crate::replay`]: it dispatches, it appends and it takes a `Mutex`, and the two attributes below
//! reach every child of this one — so a `runtime` module here would either fail to compile or need
//! an `#[expect]` per I/O call, which in the module whose *purpose* is I/O is a gate that has
//! stopped meaning anything.
//!
//! **What stayed here is the half that has no port in it**, and [`idem`] is the new one: the
//! `(item, activity_id@version, attempt)` key is a **pure function** of three values, and its three
//! refusals (AC-010) are decided before any process is spawned. [`idem::PlannedDispatch`] is a
//! **value** carrying the key by field. 🛑 **Nothing in this crate consumes a `PlannedDispatch`**:
//! `rk_ports::harness::Harness::dispatch` takes `(spec, store)` and **no key**
//! (`crates/rk-ports/src/harness.rs`:37), and `crate::runtime`'s one live call site passes an
//! `ActivitySpec` that carries none either. The key is computed, journaled and returned; it never
//! crosses the port. *(Corrected 2026-09-14: this sentence asserted a port call that does not
//! exist.)*
//!
//! So the rule the retracted sentence carried still holds, one level out: **nothing under this
//! module dispatches, appends or writes a mark.** `NextAction` is a decision value, a backoff delay
//! is a number, a `BudgetExtensionMark` is a value, and `OpenMark` is a fold's output. The module
//! that acts on any of them is one `pub mod` line away in `src/lib.rs` and is outside this gate by
//! construction.

#![deny(clippy::disallowed_methods)]
#![deny(clippy::disallowed_types)]

/// **AC-020–AC-025.** The three budget counters, the activity's declared envelope, the two
/// quantities `attempts_total` names, and the exhaustion gate. A child of this module, so the deny
/// attributes above reach it.
pub mod budget;
/// **The human gate's answer path and re-reach** (Step 7b-ii, `s7b-ii` §7b-ii.2): the spent-token
/// refusal, the void, the fresh record on a loop-back re-reach. A child of this module, for
/// [`budget`]'s reason — and AC-242's: no sleep, no blocking receive, no timer on the gate path.
pub mod gate;

/// **AC-029's `N`, AC-033's `K`, the per-item token appetite and — as of 2026-09-14 (D-7) — the
/// breaker's expiry and its role → family key** — the values a pack declares and this crate refuses
/// to default. A child of this module, for [`budget`]'s reason.
pub mod envelope;

/// **AC-033–AC-035.** The circuit breaker: the K-consecutive-`Platform` counting rule, the reset
/// rule, the per-key isolation, the store-derived state and the refusal — keyed on the **model
/// family the pack declares for a step's role**, resolved at plan time (D-7, 2026-09-14; this said
/// *"an opaque family token this crate never constructs"* until that decision, and the token is
/// still never constructed *here* — it is transcribed from a pack declaration).
/// A child of this module, for [`budget`]'s reason, and
/// pointedly so: AC-035's expiry comparison is against a journaled value, and a `SystemTime::now()`
/// in that comparison is a non-zero clippy exit under the attributes above rather than a review
/// comment.
pub mod breaker;

/// **AC-036–AC-038.** Degradation as a typed value: the substitution refusal, the
/// `Degraded { from, to, why }` record in both of the sinks AC-037 names, and the per-rigor policy
/// table a tier-demanding gate closes (or does not) under. A child of this module, for [`budget`]'s
/// reason.
pub mod degrade;

/// **AC-006–AC-008, AC-039, AC-161, AC-164.** The FSM: `RunState` as a pure fold over the recorded
/// journal, the step-grain `NextAction` and the stage-grain `StageAction`, the `TerminalState` a
/// kernel that does nothing at all cannot fake, and `Divergence`'s typed halt. A child of this
/// module, for [`budget`]'s reason — and this one carries the proof of it: [`fsm`]'s
/// `fsm_reads_the_wall_clock`, behind `workflow-fsm-clock-reintroduce`, is the only thing in the
/// tree that shows the two attributes below are **inherited by a child** rather than merely
/// declared one file away.
pub mod fsm;

/// **AC-010–AC-012.** The idempotency key's validator and the dispatch that cannot be constructed
/// without one. A child of this module, for [`budget`]'s reason — and pointedly so: a key is a pure
/// function of `(item, activity_id@version, attempt)`, and a key minted from a clock or a random
/// source would be the non-deterministic id AC-002 bans. What is NOT here is the dispatch itself:
/// [`idem::PlannedDispatch`] is a value that **nothing consumes** — the port takes no key
/// (`crates/rk-ports/src/harness.rs`:37). *(Corrected 2026-09-14; see this module's header.)*
pub mod idem;

/// **AC-014–AC-019.** The memo: the eight runtime facts `rk_store::memo::StepKeyComponents` is
/// **assembled** from (never a second derivation of the key), the activity-level key's separate
/// formula and its single-card-reference lint, the `memo_hit` payload AC-019 enumerates, and the
/// two lints ADR-0059 and ADR-0097 decision 3 leave behind. A child of this module, for [`budget`]'s
/// reason — and pointedly so here too: a memo key is a **pure function of journaled facts**, and a
/// clock read or an unordered set inside it would make the same key derive differently on a replay.
/// The ADR-0063 question registry — what a gate may be answered with, and what an answer must
/// carry (ADR-0129 d3; Step 7d Slice 3).
pub mod question;

pub mod memo;

/// **AC-026–AC-032.** The retry policy over ADR-0112's eleven variants, the backoff envelope and
/// its two kernel constants, the attempt cap against the pack's `N`, and the one door an activity
/// outcome reaches a retry decision through. A child of this module, for [`budget`]'s reason — and
/// pointedly so: a delay computed here is a **number**, and the two sleeps `clippy.toml` bans are
/// denied in this file by the attributes above.
pub mod retry;

/// The AC-001 must-fail control: exactly one banned determinism call, behind a non-default feature.
///
/// If this function ever clips **clean** with `--features rk-kernel/workflow-clock-reintroduce`,
/// the module-level deny above is declared and does not bite, and AC-001–AC-004's `rk-kernel` half
/// is pinned by nothing.
#[cfg(feature = "workflow-clock-reintroduce")]
#[must_use]
pub fn reads_the_wall_clock() -> std::time::SystemTime {
    // AC-001: the wall clock on the workflow side of the split. Banned by `clippy.toml`'s
    // `disallowed-methods` table and denied at module level by the attribute above.
    std::time::SystemTime::now()
}
