//! **The FSM** — the state the split is about, and the next action AC-006, AC-008, AC-161 and
//! AC-164 all name.
//!
//! Slice 3 recorded this half **NOT BUILDABLE**, twice, in the runbook's own words
//! (`s7b-kernel.md`:1103-1109 and :1153-1159):
//!
//! > **(b) Only three of the five assertions are measured.** *Byte-identical **next action*** and
//! > the **terminal-state** half of the fifth are **NOT BUILDABLE**: both are the FSM's, and
//! > **there is no FSM** — the workflow/activity split, the memo, retry and the runtime are
//! > §7b.10's **Slice 4**.
//!
//! > *"the next action"* is the FSM's and there is no FSM … **AC-161's and AC-164's next-action
//! > halves stay ARMED, NOT DISCHARGED**, and their instrument is **Slice 4's runtime plus this
//! > test**.
//!
//! That sentence stops being true of this crate on 2026-09-14. What replaces it is stated
//! precisely below, because *"the FSM exists"* and *"AC-006 and AC-008 are discharged"* are two
//! different propositions and only the first is this module's.
//!
//! > **AC-008** — `rk-kernel::replay` · decisions: ADR-0022
//! > GIVEN a replay of a run that dispatched N activities
//! > WHEN the log is replayed
//! > THEN the replay reaches the recorded terminal state with all N results served from the
//! > journal, **zero** harness dispatches, and zero `rk-git` writes.
//! > *Verified by:* … a test that separately asserts the terminal state matches the record — so …
//! > **a kernel that does nothing at all fails the terminal-state half.**
//! >
//! > **AC-161** — `rk-store` + `rk-git` · decisions: **ADR-0109**, ADR-0097, ADR-0013
//! > THEN the **stage-level next action** — the FSM's next stage transition and the gate records
//! > the trailers carry — is equal, under the canonical serialiser, to the one selected before the
//! > deletion … *The comparison is at stage grain; a step-grain comparison would assert what tier 2
//! > cannot promise.*
//! >
//! > **AC-007** — `rk-kernel::replay` · decisions: ADR-0022
//! > THEN the run halts with a typed `ReplayDivergence { activity_id, recorded, recomputed }`, no
//! > new action is taken, and no continuation on the recomputed value is reachable.
//! >
//! > **AC-039** — `rk-kernel::budget` / **human gate** · decisions: ADR-0017, ADR-0009
//! > THEN the decision arrives as a gate answer bound to `(token, tree_hash, deadline)` and the
//! > kernel takes no action until it does. *Verified by:* routed to a **human gate**; **not
//! > machine-decidable** … The *mechanism* is machine-checked (a test asserting the halt writes a
//! > gate record, and that an answer whose tree hash no longer matches is **void, not
//! > reconciled**), but the answer is not.
//!
//! # Why this is a FOLD and not a mutable machine
//!
//! [`RunState`] is `fold(initial(item, stage), events)` and there is no `advance(&mut self)` on it.
//! `apply` — the fold's step — is **private to this module**, so the only route to a non-initial
//! state is a recorded journal. Three consequences, and they are the reason the shape was chosen:
//!
//! 1. **A machine that advanced by mutation would hold the answer in memory, and AC-035's THEN is
//!    that it does not.** The breaker, the budget ledger and the gate set are all folds over
//!    `&[Event]` for one reason: a second process lifetime is a second fold over the same store,
//!    and it decides the same way because the decision is a function of the store's contents.
//! 2. **The replay can re-run it.** AC-008's *"all N results served from the journal, zero harness
//!    dispatches"* is an architecture here rather than an assertion: replaying is folding a
//!    journal, and the only thing that differs between a recording and a replay is where the next
//!    event came from.
//! 3. **Time enters only as a journaled value.** Nothing in this file reads a clock.
//!    `rk_types::time::Timestamp` *"has no `now()`, deliberately"*, and the parent module's
//!    `#![deny(clippy::disallowed_methods)]` reaches this child — which
//!    [`fsm_reads_the_wall_clock`] is the committed positive control for.
//!
//! # The two grains, and why there are exactly two
//!
//! | | [`NextAction`] | [`StageAction`] |
//! | --- | --- | --- |
//! | grain | step — what the runtime does next | **stage** — the next stage transition and the gate records |
//! | computed from | the whole journal plus the [`Selection`] | the **tier-1 projection alone** |
//! | who compares it | AC-006 assertion 3, byte-identical across a replay | AC-161 across a deleted database, AC-164 across a truncated tier 2 |
//! | why two types | *"The comparison is at stage grain; a **step-grain comparison would assert what tier 2 cannot promise**"* (AC-161's own *Verified by*) |
//!
//! # The tier filter is INSIDE [`stage_action`], not a caller's discipline
//!
//! [`stage_action`] filters `EventKind::tier() == Tier::One` **itself**, so a caller holding tier-2
//! facts cannot leak one into a stage-grain answer. ADR-0109 decision 4 is the warrant:
//! *"`doctor --rebuild` reproduces the stage-level next action **from git alone**"* — and git
//! carries the five tier-1 kinds and nothing else.
//!
//! **[`stage_action`] therefore takes no [`Selection`]**, and that is a decision rather than an
//! omission: the stage-grain projection is a function of the tier-1 journal alone, which is exactly
//! what *"from git alone"* means, and a `&Selection` parameter the body did not read would suggest
//! to the next reader that it did.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | id | what this module gives it | state |
//! | --- | --- | --- |
//! | **AC-007** | [`Divergence`], its three-arm no-wildcard [`Divergence::halt`], and the `trybuild` case that pins the arms | the **halt** is measured; the divergence is not yet *detected* anywhere — `src/replay.rs` gains steps 4b/4c at Slice **4f** |
//! | **AC-008** | [`TerminalState`] and [`RunState::canonical_bytes`] — the recomputable side of the comparison | the recorded side is `run.json`'s (Slice 4f) and the **N dispatched activities its GIVEN names** arrive at Slice 4i, when `crate::replay::record` drives [`NextAction::Dispatch`] through a bound `rk_ports::harness::Harness`. **No claim is made here**: what this module owns is the fold and the two grains, and `crate::replay`'s header carries the criterion's state |
//! | **AC-006** assertion 3 | [`NextAction::canonical_bytes`] | **ARMED**, same reason |
//! | **AC-161 / AC-164** | [`stage_action`] and its tier filter | **ARMED**: the *equality* is computable now; the **route** (from git trailers rather than from the log) is `rk-git`'s and `rk-cli`'s, Step 7d. AC-164's `doctor`-classification half is claimed by **no stage at all**, which is a homelessness finding and not a gap this module can close |
//! | **AC-039** | the gate record bound to `(token, tree_hash, deadline)` and the **void, not reconciled** rule | the *mechanism* half only. **HUMAN GATE** — §7b.15 excludes the id, and the answer is never asserted |
//!
//! # The gate token is derived, never minted
//!
//! [`GateRecord::derive`] is the **only** constructor, and the token is
//! `rk_store::canon::canonical_digest` over the record's other three members. AC-002 bans a minted
//! id — the escape its own ban names is *"a content hash"*, and this is one. `tree_hash` and
//! `deadline` are **arguments**: ADR-0105 clause 4 owns the one `tree_hash` algorithm and it is
//! `rk-git`'s, and a second derivation is forbidden by that clause's own forbids bullet.
//!
//! A consequence, stated because it is load-bearing: a `token` member **recorded in the journal is
//! not read**. The fold derives the token from the payload it can verify, so a forged token cannot
//! steer the state — and an answer naming a token **no derived gate this journal ever opened**
//! carries is [`FsmError::GateAnsweredWithoutOpen`], a typed refusal, never a silently dropped
//! answer. The emphasis is the subject of the next section.
//!
//! # `gate_answered` IS THREE CASES, AND ONLY ONE OF THEM IS FORGERY
//!
//! **The repair of a defect found 2026-09-14 on an adversarial verification pass, landed on the
//! operator decision of the same day.** [`apply`]'s [`EventKind::GateAnswered`] arm asked one
//! question — *is the token in `gates_open`?* — and gave **one** answer to **two different
//! situations**. That is the whole warrant, and it is deliberately no wider than this sentence.
//!
//! The three cases now:
//!
//! | the token is… | the fold | why |
//! | --- | --- | --- |
//! | in `gates_open` | answered (or **void** on a `tree_hash` mismatch) — **unchanged** | AC-039 |
//! | in `gates_expired` | **void**, and the fold **CONTINUES** | this section |
//! | never seen | [`FsmError::GateAnsweredWithoutOpen`] | forgery — the message is correct *here* |
//!
//! **What the middle row was before, and why it could not stay.** The `GateExpired` arm **removes**
//! the token from `gates_open`, so a journal carrying `gate_opened → gate_expired → gate_answered`
//! for one derived token folded to [`FsmError::GateAnsweredWithoutOpen`] — **and never stopped
//! doing so**. The journal is append-only: the run was not merely halted, it was **unfoldable for
//! ever**, and `next_action`, `stage_action` and every replay over that item were unreachable with
//! it. The token was also not forged — the operator answered a gate this system itself derived,
//! opened and expired, an ordinary race (F18 measured **45% of gate→verdict intervals above one
//! hour**, so it is the common case, not the pathological one) — and the diagnostic accused them of
//! forgery for it.
//!
//! **Nothing is minted.** [`EventKind::GateAnswerVoid`] already exists and [`apply`] already has an
//! arm for it writing exactly this disposition; `rk_types::drift` already models a void answer; and
//! `gates_void` is already terminal — nothing in this module moves a token out of it — so a voided
//! answer can never later count as one.
//!
//! # 🛑 WHETHER A VOIDED GATE RE-OPENS IS **NOT** SETTLED HERE — IT IS 7b-ii's
//!
//! Stated separately so the narrow warrant above cannot be read wider. AC-176 classifies a
//! `tree_hash`-drifted answer void **and** re-opens the gate, but its **Boundary note assigns
//! gate-record semantics to the gates area (7b-ii)**, and AC-039 — the nearest rule in this module —
//! is about a `tree_hash` mismatch, says nothing about expiry, and is marked **HUMAN GATE**.
//! ADR-0105 clause 10 constrains only what an expiry *action* may be.
//!
//! So [`next_action`]'s step 2 is **untouched**: it still declines to re-open a token in
//! `gates_expired`, which is the behaviour that was already here and not a reading taken today.
//! [`RunState::awaiting_gate`] reads `gates_open` alone, so an expired token does not hold the run
//! at [`TerminalState::AwaitingGate`] either — also unchanged.
//!
//! Pinned by `tests/it/fsm.rs::gate_answered_after_expiry_is_void_and_the_fold_continues`, which
//! supersedes `gate_answered_after_expiry_is_unfoldable_known_defect`, the pin that held the
//! unfoldable journal as a DEFECT until the decision landed. That name carries **no AC id** and
//! **a durable record naming the decision is OWED**, because no criterion states the disposition of
//! an answer to an expired gate.
//!
//! # 🛑 FOUR VALUES THIS MODULE NEEDS AND CANNOT COMPUTE, EACH AN ARGUMENT WITH ONE CONSTRUCTOR
//!
//! The same discipline `breaker.rs` applies to its family key and its expiry, for the same reason:
//! a value invented here would be a decision taken at a stage that may not take one
//! (`00-preamble.md`:55).
//!
//! 1. **`tree_hash`** — ADR-0105 clause 4's, `rk-git`'s. An argument on [`FsmContext`].
//! 2. **`deadline`** — a journaled instant. `Timestamp` has no clock constructor, so it can only
//!    have come from data. An argument.
//! 3. **the successor stage** — **there is no stage ordering in this system.**
//!    `spikes/p9/packs/incident.toml`:1-2 says so in its own first comment: *"Stage ORDER
//!    (triage → operate → analysis → implement → release → closeout) is **not modelled** by the
//!    selection function; only per-stage requirement is."* So [`next_action`] never invents a
//!    successor: with one supplied it can answer [`NextAction::AdvanceStage`], and with none it
//!    answers [`NextAction::Done`]. **Both are asserted**, so the absence is visible rather than
//!    defaulted.
//! 4. **`activity_id@version`** — ADR-0072's version has **no carrier on a `DagNode`**
//!    [measured 2026-09-14: `DagNode` carries `id` and `activity_id` and no version]. The
//!    idempotency key's second part is therefore the DAG node's `activity_id` **verbatim** unless
//!    the caller supplies an activation for that node, and the `@version` suffix is **ARMED**.
//!    AC-010's *"an `activity_id` without `@version` is refused"* is a **validator**,
//!    `src/workflow/idem.rs`'s — this module mints no key that pretends to carry a version it
//!    could not read.
//!
//!    🛑 **[`FsmContext::with_activations`] is the fifth supplied value, added 2026-09-14, and it
//!    is SUPPLIED rather than DERIVED for the same reason the other four are.** The join between a
//!    DAG node and `crate::catalog::Activity::id_at_version` has three candidate homes — step 5's
//!    `DagNode` construction, this function's node lookup, or the composition root — and choosing
//!    one is a decision `00-preamble.md`:55 reserves. **This slice takes none of the three.** What
//!    it adds is an argument: a `(node id, activity_id@version)` list the caller transcribes, on
//!    the same model as `tree_hash`, `deadline` and the successor stage. `crate::replay` supplies
//!    it out of `inputs/specs.json` — a **logged input**, so AC-042's *"from the logged inputs
//!    alone"* still holds — and **a record naming where the join lives is still OWED**. With no
//!    activation supplied the key carries the bare `activity_id` exactly as before, which is what
//!    keeps `the_fsm_dispatch_key_has_no_version_and_the_boundary_refuses_it`'s pinned finding
//!    true rather than quietly repairing it.
//!
//! # The payload shape is a READING, against an already-OWED record
//!
//! `Event.payload` is `serde_json::Value` *"provisionally and visibly"*, and
//! `crates/rk-types/src/event.rs`:46-58 records that **the typed-payload record, per kind, is
//! OWED**. This module does not invent it. Eleven kinds move the state and eighteen do not, in one
//! grouped arm; a member that is absent or of the wrong shape is [`FsmError::MalformedPayload`],
//! never a silent zero:
//!
//! | kind | tier | members read | effect |
//! | --- | --- | --- | --- |
//! | `stage_transition` | 1 | `to`, optional `from` | push `to`; set the stage; record the transition |
//! | `gate_opened` | 1 | `kind`, `tree_hash`, `deadline` — **or** the selection's `reason` (see reading 3) | derive the token and insert the record; an announcement moves nothing |
//! | `gate_answered` | 1 | `token`, `tree_hash`, `verdict` | the answer's `tree_hash` differs from the gate's → **void** (AC-039); else answered |
//! | `gate_expired` | 1 | `token` | remove from the open set, record the expiry |
//! | `slice_sealed` | 1 | `verdict` | the run is sealed |
//! | `activity_committed` | 2 | `item`, `activity_id_at_version`, `attempt`; *(2026-09-27, `s8d` build, R2FIX-8d; R1 = ADR-0174 decision 6, proposed)* and, when present, `step_id` with `iteration_path` (a row carrying `step_id` without a readable `iteration_path` is malformed) and `completion`; *(dated 2026-09-27, `s8d` build, D2177-8d; D21-76, ADR-0174 decision 7)* and `decision`, whose presence alone is read | ~~the activation is committed~~ a row with `step_id` commits that step activation (`is_step_committed`); the activation is COMPLETE on a completion row or a pre-R1 row (`is_committed`); a row carrying `decision` folds its attempt only — not a commit and not a completion |
//! | `dispatch` | 2 | the same three | the activation was dispatched; the attempt high-water mark moves |
//! | `gate_answer_void` | ~~2~~ 1 *(2026-09-28, `s8e` build, S8E-P4; ADR-0184, proposed: a git-first `Gate: … voided` mark, D21-43 N6)* | `token` | the mirror of the void decision, for a run whose voiding was already recorded |
//! | `step_refused` | 2 | `class` | a recorded halt (ADR-0095 decision 2's refusal arm) |
//! | `memo_hit`, `dag_amended` | 2 | none | counted only (AC-019's identity, ADR-0120's delta) |
//! | every other kind | — | none | **no state change**, one grouped arm |
//!
//! Three of those rows are readings this file takes because **no record states them**, and all
//! three are asserted in `tests/it/fsm.rs` or `tests/it/replay.rs`:
//!
//! 1. **A `gate_opened`'s `kind` for a selection gate is `"<reason>:<axis>"`.** Design §5 gives
//!    `gate_opened` two members — `reason` and `axis` — and [`GateRecord`] carries one `kind`, so
//!    two gates differing only in axis would otherwise derive one token and collapse into one gate.
//!    A record naming the trailer's `kind` spelling is **OWED**.
//! 2. **`step_refused` is the ONLY halt this crate can read out of a journal.** AC-024's budget
//!    exhaustion and AC-033's breaker refusal are halts too, and neither has an `EventKind` —
//!    `EventKind` is closed at 29 and ADR-0116 fixes the route (*"the kind is minted by record or
//!    not at all"*) — which `budget.rs` and `breaker.rs` each already report as ARMED.
//!    [`HaltReason`] therefore has **one** variant and not three: a variant no call site can
//!    produce reads as a refusal the system performs.
//! 3. 🛑 **`gate_opened` HAS TWO PRODUCERS AND ONE KIND, and this was found by building the replay
//!    on 2026-09-14 rather than by reading a record.** `crates/rk-kernel/src/select.rs`:367-386
//!    emits `gate_opened` with the payload `{ reason, axis, site }` (and, for the unsatisfiable
//!    arm, `{ reason: "unsatisfiable", unsatisfiable: {…} }`) — and it emits **no** `tree_hash` and
//!    **no** `deadline`, because `select()` is forbidden to read either: AC-041 bans a clock and a
//!    filesystem read inside the pure function, and ADR-0105 clause 4's tree hash is a git fact.
//!    The runtime's own `gate_opened` — the durable `Gate:` mark ADR-0022 names — carries all
//!    three. **Reading the selection's announcement as a durable gate is impossible and defaulting
//!    its two missing members is forbidden**, so the arm below distinguishes the two by shape:
//!
//!    - a payload carrying `tree_hash` **or** `deadline` is the RUNTIME's, and **all three**
//!      members are then required — a half-shaped runtime gate is [`FsmError::MalformedPayload`]
//!      and never a silently dropped gate;
//!    - a payload carrying `reason` and neither of those two is the SELECTION's announcement. It
//!      is counted in [`FoldSurvey::gates_announced`] and **moves no state**: the durable mark has
//!      not been written, so `terminal()` must not report [`TerminalState::AwaitingGate`] on it.
//!      [`next_action`]'s step 2 is what turns such an announcement into
//!      [`NextAction::OpenGate`], which is the act that mints the three members;
//!    - a payload that is neither is [`FsmError::MalformedPayload`], as before.
//!
//!    **This is a reading and the record that would settle it is the already-OWED typed payload
//!    per kind** (`crates/rk-types/src/event.rs`:46-58). What it does NOT do is default a value:
//!    the survey counter makes an announcement visible, so *"the fold ignored a gate"* and *"the
//!    fold never saw one"* are distinguishable.
//!
//! [`FsmError`] carries no `UnknownKind` variant for the same rule. `EventKind` is closed, so no
//! call site could produce one.
//!
//! # What is deliberately NOT here
//!
//! **No stage ordering** (above). **No dispatch and no append** — [`NextAction`] is a decision
//! *value*, and the boundary that acts on one is Slice 4h's. **No clock, no randomness, no I/O, no
//! `HashMap`** — the parent module's two `#![deny]` attributes reach this child. **No
//! `Deserialize` and no `Default` on [`RunState`]**:
//! `tests/ui/stay_class_state_is_unconstructible.rs` is what makes that a type fact rather than a
//! convention, on `RequiredSet`'s exact doctrine (`src/select.rs`:97-105) — *"a `pub(crate)`
//! constructor, or a `#[cfg(test)]` public one, would silently convert it into a convention"*. **No
//! `pub trait` of any shape** (`replay.rs`:200-203: AC-483's denylist parses `rk-ports`' source
//! only and would not see one declared here).
//!
//! # The positive control
//!
//! [`fsm_reads_the_wall_clock`] sits behind the non-default `workflow-fsm-clock-reintroduce`
//! feature **of this crate**:
//!
//! ```text
//! green:  cargo clippy --workspace --all-targets
//! RED:    cargo clippy --workspace --all-targets --features rk-kernel/workflow-fsm-clock-reintroduce
//! ```
//!
//! It is a **different proposition** from `workflow-clock-reintroduce`, which reintroduces the call
//! in `src/workflow.rs` itself — the file that carries the attribute. This one is in a **child
//! module**, so it is the only thing in the tree that shows the deny is **inherited** rather than
//! merely declared one file away. Without it, the inheritance every module since Slice 4a has
//! relied on is an assumption. Committed rather than observed and reverted, on
//! `tests/fixtures/rk-clock-read/`'s precedent (DECISIONS-2026-09-12 E4): a reverted observation is
//! not an assertion. **No Checkpoint 7b line runs it** — :1396-1398 names only
//! `workflow-clock-reintroduce` — so its result is reported in this slice's record and is never
//! claimed as a checkpoint pass.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use rk_store::canon::{CanonError, canonical, canonical_digest};
use rk_store::memo::IdempotencyKey;
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, ReplayDivergence, Tier};
use rk_types::time::Timestamp;

use crate::select::Selection;

/// One gate as the trailers carry it — **AC-161's *"the gate records the trailers carry"*** and
/// AC-039's *"a gate answer bound to `(token, tree_hash, deadline)`"*.
///
/// Fields are private and [`derive`](GateRecord::derive) is the only constructor, because the token
/// is **derived, never minted**: a public `token` field would admit a record whose token is not the
/// digest of its own contents, and AC-039's binding would then be a convention.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateRecord {
    token: Digest,
    kind: String,
    tree_hash: String,
    deadline: Timestamp,
    reach: u32,
}

impl GateRecord {
    /// Derive a gate record from the three things that identify a gate.
    ///
    /// The token is `canonical_digest({deadline, kind, tree_hash})` — a **content hash**, which is
    /// the escape AC-002's ban names. `tree_hash` is ADR-0105 clause 4's and arrives as a value;
    /// nothing here computes one.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the canonical serialiser, which `rk-store` refuses by
    /// decision. No member of this record is a float, so this cannot fire on a value this module
    /// built — it can on a serialiser that later grows one, which is why it is not swallowed.
    pub fn derive(kind: &str, tree_hash: &str, deadline: Timestamp) -> Result<Self, CanonError> {
        Self::derive_reach(kind, tree_hash, deadline, 1)
    }

    /// Derive the record a given **reach** of the gate step mints (AC-524, ADR-0105 clause 10).
    ///
    /// `reach` is a member of the digest, so a loop-back that re-reaches the step over an
    /// unchanged tree inside one deadline window still mints a **fresh** token — freshness is
    /// structural, not a property of the clock. The first reach is `1`, and [`derive`] is that.
    ///
    /// # Errors
    ///
    /// As [`derive`](Self::derive).
    pub fn derive_reach(
        kind: &str,
        tree_hash: &str,
        deadline: Timestamp,
        reach: u32,
    ) -> Result<Self, CanonError> {
        let token = canonical_digest(&serde_json::json!({
            "deadline": deadline.unix_seconds(),
            "kind": kind,
            "reach": reach,
            "tree_hash": tree_hash,
        }))?;
        Ok(Self {
            token,
            kind: kind.to_owned(),
            tree_hash: tree_hash.to_owned(),
            deadline,
            reach,
        })
    }

    /// Which reach of this gate step minted the record (1 on first reach).
    #[must_use]
    pub const fn reach(&self) -> u32 {
        self.reach
    }

    /// The derived token — the value a `Gate:` trailer carries.
    #[must_use]
    pub const fn token(&self) -> &Digest {
        &self.token
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// The tree hash the answer is bound to (AC-039). An answer carrying a different one is
    /// **void, not reconciled**.
    #[must_use]
    pub fn tree_hash(&self) -> &str {
        &self.tree_hash
    }

    #[must_use]
    pub const fn deadline(&self) -> Timestamp {
        self.deadline
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "deadline": self.deadline.unix_seconds(),
            "kind": self.kind,
            "reach": self.reach,
            "token": self.token.as_str(),
            "tree_hash": self.tree_hash,
        })
    }
}

/// A gate answer the journal recorded and the fold accepted — i.e. one whose `tree_hash` still
/// matched the gate's. An answer that did **not** match never becomes one of these; it lands in
/// [`RunState::void_gates`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GateAnswer {
    token: Digest,
    tree_hash: String,
    verdict: String,
}

impl GateAnswer {
    #[must_use]
    pub const fn token(&self) -> &Digest {
        &self.token
    }

    #[must_use]
    pub fn tree_hash(&self) -> &str {
        &self.tree_hash
    }

    #[must_use]
    pub fn verdict(&self) -> &str {
        &self.verdict
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "token": self.token.as_str(),
            "tree_hash": self.tree_hash,
            "verdict": self.verdict,
        })
    }
}

/// Where a run has come to rest — **AC-008's *"the recorded terminal state"***.
///
/// [`TerminalState::NotStarted`] exists so that *"a kernel that does nothing at all"* has somewhere
/// to land, and AC-008's own warning is that such a kernel must **fail** the terminal-state half.
/// It is the value a fold over an empty journal returns.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalState {
    /// Nothing in the journal moved the state.
    NotStarted,
    /// A gate is open and neither answered nor void. ADR-0022: the kernel takes no action until it
    /// is answered.
    AwaitingGate { token: Digest },
    /// In a stage, with dispatched activations still uncommitted — or with none dispatched yet.
    AwaitingDispatch { stage: String },
    /// In a stage, and every activation the journal dispatched has committed.
    StageComplete { stage: String },
    /// A `slice_sealed` verdict.
    Sealed { verdict: String },
}

impl TerminalState {
    /// Every tag, so a sixth state is a compile error in a sweep rather than a forgotten row.
    pub const ALL_TAGS: [&'static str; 5] = [
        "not_started",
        "awaiting_gate",
        "awaiting_dispatch",
        "stage_complete",
        "sealed",
    ];

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::AwaitingGate { .. } => "awaiting_gate",
            Self::AwaitingDispatch { .. } => "awaiting_dispatch",
            Self::StageComplete { .. } => "stage_complete",
            Self::Sealed { .. } => "sealed",
        }
    }

    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::NotStarted => serde_json::json!({ "terminal": self.as_str() }),
            Self::AwaitingGate { token } => {
                serde_json::json!({ "terminal": self.as_str(), "token": token.as_str() })
            }
            Self::AwaitingDispatch { stage } | Self::StageComplete { stage } => {
                serde_json::json!({ "terminal": self.as_str(), "stage": stage })
            }
            Self::Sealed { verdict } => {
                serde_json::json!({ "terminal": self.as_str(), "verdict": verdict })
            }
        }
    }

    /// The canonical byte form the replay compares.
    ///
    /// `rk_store::canon::canonical`, never `serde_json::to_vec`: `serde_json` carries
    /// `preserve_order` workspace-wide, so a plain serialisation leaks construction order into the
    /// comparison (`src/select/canonical.rs`:1-19).
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical(&self.to_json())
    }
}

/// The one halt this crate can read out of a recorded journal.
///
/// **One variant, deliberately** — until 2026-09-17, when Step 7b-ii's slice C added the second
/// for ADR-0017's first terminal path, recorded by a row that DOES exist: `gate_expired { action:
/// "fail" }`. The reasoning below stands for every other candidate.
///
/// **Originally:** AC-024's budget exhaustion and AC-033's breaker refusal are halts
/// too, and neither has an `EventKind` to be recorded under — see the module header's second
/// reading. A variant no call site can produce reads as a refusal the system performs.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HaltReason {
    /// `step_refused { class }` — ADR-0095 decision 2's refusal arm. *"A refusal is a configuration
    /// or billing state, never a judgement"*, so it halts the run and never reaches a gate as a
    /// verdict.
    Refused { class: String },
    /// A gate passed its deadline under an expiry policy of `fail` (ADR-0017's first terminal
    /// path; AC-256, AC-259). Recorded by the `gate_expired { action: "fail" }` row the expiry
    /// resolution writes (Step 7b-ii, slice C).
    GateExpired { token: Digest },
}

impl HaltReason {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Refused { .. } => "refused",
            Self::GateExpired { .. } => "gate_expired",
        }
    }

    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Refused { class } => {
                serde_json::json!({ "reason": self.as_str(), "class": class })
            }
            Self::GateExpired { token } => {
                serde_json::json!({ "reason": self.as_str(), "token": token.as_str() })
            }
        }
    }
}

/// What the runtime does next — **AC-006 assertion 3's *"next action"*, at full grain.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextAction {
    /// A gate the selection requires that the journal has not opened.
    OpenGate(GateRecord),
    /// An open gate, neither answered nor void. ADR-0022: the kernel takes no action until it is.
    AwaitGate { token: Digest },
    /// The next uncommitted activation in step 6's topological order.
    Dispatch {
        key: IdempotencyKey,
        step_id: String,
    },
    /// The stage's plan is exhausted **and a successor was supplied**.
    AdvanceStage { from: String, to: String },
    /// A recorded halt.
    Halt { reason: HaltReason },
    /// Nothing to do — including the case where the stage is finished and **no successor was
    /// supplied**, because this system models no stage ordering.
    Done,
}

impl NextAction {
    /// Every tag, so a seventh action is a compile error in a sweep rather than a forgotten row.
    pub const ALL_TAGS: [&'static str; 6] = [
        "open_gate",
        "await_gate",
        "dispatch",
        "advance_stage",
        "halt",
        "done",
    ];

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::OpenGate(_) => "open_gate",
            Self::AwaitGate { .. } => "await_gate",
            Self::Dispatch { .. } => "dispatch",
            Self::AdvanceStage { .. } => "advance_stage",
            Self::Halt { .. } => "halt",
            Self::Done => "done",
        }
    }

    fn to_json(&self) -> serde_json::Value {
        match self {
            Self::OpenGate(record) => {
                serde_json::json!({ "action": self.as_str(), "gate": record.to_json() })
            }
            Self::AwaitGate { token } => {
                serde_json::json!({ "action": self.as_str(), "token": token.as_str() })
            }
            Self::Dispatch { key, step_id } => serde_json::json!({
                "action": self.as_str(),
                "key": {
                    "activity_id_at_version": key.activity_id_at_version,
                    "attempt": key.attempt,
                    "item": key.item,
                },
                "step_id": step_id,
            }),
            Self::AdvanceStage { from, to } => {
                serde_json::json!({ "action": self.as_str(), "from": from, "to": to })
            }
            Self::Halt { reason } => {
                serde_json::json!({ "action": self.as_str(), "halt": reason.to_json() })
            }
            Self::Done => serde_json::json!({ "action": self.as_str() }),
        }
    }

    /// The canonical byte form AC-006 assertion 3 compares.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical(&self.to_json())
    }
}

/// **AC-161/AC-164's *"stage-level next action"*** — a strictly coarser grain, and a separate type
/// so the two cannot be confused at a call site.
///
/// *"The comparison is at stage grain; a step-grain comparison would assert what tier 2 cannot
/// promise."*
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StageAction {
    /// The stage transition the tier-1 journal determines, as `(from, to)`.
    pub transition: Option<(String, String)>,
    /// The gate records the trailers carry, in token order.
    pub gates: Vec<GateRecord>,
}

impl StageAction {
    fn to_json(&self) -> serde_json::Value {
        let transition = match &self.transition {
            Some((from, to)) => serde_json::json!({ "from": from, "to": to }),
            None => serde_json::Value::Null,
        };
        serde_json::json!({
            "gates": self.gates.iter().map(GateRecord::to_json).collect::<Vec<_>>(),
            "transition": transition,
        })
    }

    /// The canonical byte form AC-161 and AC-164 compare across a deleted database and a truncated
    /// tier 2.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical(&self.to_json())
    }
}

/// **AC-007's divergence outcome, matched exhaustively with no wildcard arm.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Divergence {
    /// A recorded decision at `index` differs from the recomputed one.
    Decision {
        index: usize,
        recorded: String,
        recomputed: String,
    },
    /// The recomputed [`NextAction`]'s canonical bytes differ from the recorded ones.
    NextAction {
        recorded: String,
        recomputed: String,
    },
    /// The recomputed [`TerminalState`]'s canonical bytes differ from the recorded ones.
    TerminalState {
        recorded: String,
        recomputed: String,
    },
}

impl Divergence {
    /// Every tag, so a fourth divergence is a compile error in a sweep rather than a forgotten row.
    pub const ALL_TAGS: [&'static str; 3] = ["decision", "next_action", "terminal_state"];

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Decision { .. } => "decision",
            Self::NextAction { .. } => "next_action",
            Self::TerminalState { .. } => "terminal_state",
        }
    }

    /// **The halt.** *"No new action is taken, and no continuation on the recomputed value is
    /// reachable"* — `self` is consumed, the recomputed value is **moved into** the returned
    /// `ReplayDivergence`, and this function returns no path back to a [`RunState`] or a
    /// [`NextAction`].
    ///
    /// The `match` has three arms and **no `_` arm**: a fourth variant is an `E0004` here, which is
    /// AC-007's second instrument. `tests/ui/ac007_divergence_match_has_no_wildcard.rs` is the
    /// third.
    ///
    /// `ReplayDivergence` is `rk-types`' own struct, *"deliberately not"* an `EventKind` — design
    /// §5's tier column reads `—` and calls it *"a halt, not a durable record"*, so recording it
    /// would make the halt durable and let a run continue past it.
    #[must_use]
    pub fn halt(self, activity_id: &str) -> ReplayDivergence {
        match self {
            // The decision index is folded into the id, because `ReplayDivergence` has three
            // fields and ADR-0097's forbids clause on a fourth part of a key is the house
            // instinct: WHICH decision diverged is part of the identity, not a fourth axis.
            Self::Decision {
                index,
                recorded,
                recomputed,
            } => ReplayDivergence {
                activity_id: format!("{activity_id}#{index}"),
                recorded,
                recomputed,
            },
            // GROUPED, not two arms with one body: `clippy::match_same_arms` is denied
            // workspace-wide. Exhaustiveness is unaffected, so a fourth variant is still `E0004`
            // here — which is AC-007's second instrument, and no `_` arm exists to absorb it.
            Self::NextAction {
                recorded,
                recomputed,
            }
            | Self::TerminalState {
                recorded,
                recomputed,
            } => ReplayDivergence {
                activity_id: activity_id.to_owned(),
                recorded,
                recomputed,
            },
        }
    }
}

/// Why a fold or a next-action computation refused.
///
/// There is **no `UnknownKind` variant**: `EventKind` is closed at 29 and no call site could
/// produce one. Every variant below is produced by a path in this file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsmError {
    /// A member the fold needs is absent, or is present with the wrong JSON shape. **Never a silent
    /// zero** — `catalog.rs`'s and `pack.rs`'s parse discipline, one level up.
    MalformedPayload { kind: &'static str, detail: String },
    /// A float reached the canonical serialiser.
    Canon(CanonError),
    /// A `gate_answered` naming a token **this journal never opened**. A typed refusal, never a
    /// silently dropped answer.
    ///
    /// **This is the forgery case and only the forgery case**, as of 2026-09-14: an answer to a
    /// gate the journal opened and then EXPIRED is void and the fold continues, and does not reach
    /// here. See the module header's *"`gate_answered` is three cases"*.
    GateAnsweredWithoutOpen { token: Digest },
    /// Step 6 could not order the DAG (a cycle). An assembled `Selection` has already been
    /// validated acyclic, so this cannot fire on one this crate produced.
    PlanNotOrderable { detail: String },
}

impl FsmError {
    /// A stable tag per variant, for a message or a test that asserts which refusal fired.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::MalformedPayload { .. } => "malformed_payload",
            Self::Canon(_) => "canon",
            Self::GateAnsweredWithoutOpen { .. } => "gate_answered_without_open",
            Self::PlanNotOrderable { .. } => "plan_not_orderable",
        }
    }
}

impl fmt::Display for FsmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedPayload { kind, detail } => write!(
                f,
                "the `{kind}` payload is not the shape the fold reads: {detail}. The typed-payload \
                 record per kind is OWED (rk-types/src/event.rs:46-58); until it lands this module \
                 refuses rather than defaulting the member"
            ),
            Self::Canon(e) => write!(f, "{e}"),
            Self::GateAnsweredWithoutOpen { token } => write!(
                f,
                "a `gate_answered` names the token {} and no gate this journal opened carries it — \
                 not as open, and not as expired. The token is DERIVED from (kind, tree_hash, \
                 deadline) and a recorded `token` member is not read, so a forged token cannot \
                 steer the state",
                token.as_str()
            ),
            Self::PlanNotOrderable { detail } => {
                write!(f, "step 6 could not order the assembled DAG: {detail}")
            }
        }
    }
}

impl std::error::Error for FsmError {}

impl From<CanonError> for FsmError {
    fn from(e: CanonError) -> Self {
        Self::Canon(e)
    }
}

/// The values [`next_action`] needs and this crate **cannot compute** — see the module header's
/// 🛑 section. One constructor, named `supplied` rather than `new` because every call site is
/// transcribing something this crate did not derive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsmContext<'a> {
    item: &'a str,
    tree_hash: &'a str,
    deadline: Timestamp,
    successor: Option<&'a str>,
    /// `(node id, activity_id@version)`, supplied by the caller — the header's fifth value.
    /// **Empty is the ARMED state and not a default**: with no activation for a node the key
    /// carries the bare `activity_id` and AC-010's validator refuses it at the boundary.
    activations: &'a [(String, String)],
}

impl<'a> FsmContext<'a> {
    /// The only constructor.
    #[must_use]
    pub const fn supplied(
        item: &'a str,
        tree_hash: &'a str,
        deadline: Timestamp,
        successor: Option<&'a str>,
    ) -> Self {
        Self {
            item,
            tree_hash,
            deadline,
            successor,
            activations: &[],
        }
    }

    /// Supply the `(node id, activity_id@version)` activations — the header's fifth value.
    ///
    /// **Transcribed, never derived here.** The list is a caller's; `crate::replay` reads it out of
    /// the run's own `inputs/specs.json`, so a replay is told nothing out of band.
    #[must_use]
    pub const fn with_activations(mut self, activations: &'a [(String, String)]) -> Self {
        self.activations = activations;
        self
    }

    /// The supplied activations, in the caller's order.
    #[must_use]
    pub const fn activations(&self) -> &'a [(String, String)] {
        self.activations
    }

    #[must_use]
    pub const fn item(&self) -> &'a str {
        self.item
    }

    #[must_use]
    pub const fn tree_hash(&self) -> &'a str {
        self.tree_hash
    }

    #[must_use]
    pub const fn deadline(&self) -> Timestamp {
        self.deadline
    }

    /// The next stage, when one was supplied. **`None` is not *"the run is over"*** — it is *"this
    /// system models no stage ordering and nobody named a successor"*, which [`next_action`]
    /// answers with [`NextAction::Done`].
    #[must_use]
    pub const fn successor(&self) -> Option<&'a str> {
        self.successor
    }
}

/// House Shape C: what the fold **looked at**, so *"the state did not move"* is distinguishable
/// from *"the fold never saw anything"*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoldSurvey {
    events_folded: usize,
    state_moving: usize,
    tier_one: usize,
    gates_announced: usize,
    spent_refusals: usize,
    spent_recorded: usize,
}

impl FoldSurvey {
    #[must_use]
    pub const fn events_folded(self) -> usize {
        self.events_folded
    }

    /// How many events were one of the eleven kinds that move the state.
    #[must_use]
    pub const fn state_moving(self) -> usize {
        self.state_moving
    }

    #[must_use]
    pub const fn tier_one(self) -> usize {
        self.tier_one
    }

    /// How many `gate_opened` events were the SELECTION's announcement rather than the runtime's
    /// durable mark — the header's reading 3.
    ///
    /// It exists so that *"the fold moved no gate into the open set"* and *"the fold never saw a
    /// gate at all"* are distinguishable. Without it, a run whose only gate is an announcement is
    /// indistinguishable from a run with no gate, and the reading would be a silent drop.
    #[must_use]
    pub const fn gates_announced(self) -> usize {
        self.gates_announced
    }

    /// Second answers on a spent or void token the fold refused (AC-246).
    #[must_use]
    pub const fn spent_refusals(self) -> usize {
        self.spent_refusals
    }

    /// `gate_answer_spent` rows folded (ADR-0123).
    #[must_use]
    pub const fn spent_recorded(self) -> usize {
        self.spent_recorded
    }
}

/// The FSM's state — **the fold of a recorded event journal**, and the type
/// `tests/ui/stay_class_state_is_unconstructible.rs` names.
///
/// `initial` is `pub(in crate::workflow)`, there is no `Default` and there is no `Deserialize`.
/// [`fold`] is the only public route to a value, so a resumed or half-advanced state cannot be
/// minted from nothing — which is what makes AC-035's *"never held only in memory"* structural
/// rather than conventional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunState {
    item: String,
    stage: String,
    stages_entered: Vec<String>,
    last_transition: Option<(String, String)>,
    gates_open: BTreeMap<Digest, GateRecord>,
    /// Every runtime-shaped `gate_opened` for a gate step, in journal order — the per-(item, gate
    /// step) reach counter AC-524 names as the discovery instrument.
    gates_by_kind: BTreeMap<String, Vec<Digest>>,
    /// Tokens a `gate_resumed` row (ADR-0123) has been written for — so the runtime writes one
    /// per resumed gate and never a second (AC-279's count is per resume).
    gates_resumed: BTreeSet<Digest>,
    gates_answered: BTreeMap<Digest, GateAnswer>,
    gates_void: BTreeSet<Digest>,
    gates_expired: BTreeSet<Digest>,
    dispatched: BTreeSet<(String, String)>,
    committed: BTreeSet<IdempotencyKey>,
    /// *(R1, `s8d` P3.)* Every step activation that committed, under its commit key.
    step_commits: BTreeSet<super::idem::CommitKey>,
    /// *(R1, `s8d` P3.)* Every activation whose COMPLETION is journalled: a pre-R1 row that carries
    /// no `step_id` (the activation grain, which completed on its one commit), or R1's completion
    /// row. `next_action` moves past a node only on this.
    completed: BTreeSet<(String, String)>,
    attempts: BTreeMap<(String, String), u32>,
    memo_hits: usize,
    amendments: usize,
    halt: Option<HaltReason>,
    sealed: Option<String>,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P7; D21-93; the slice record, proposed.)* The
    /// stage the latest seal was made in: the row's `stage` member, or — for a stage-less row, which
    /// a rebuilt or healed `Slice-Verdict:` mark is — the fold's stage when the row folds.
    /// [`Self::terminal`] reads `Sealed` only while the fold is still in it. Not serialised: it
    /// moves no canonical byte (no committed golden log carries `slice_sealed`, S8E-P0).
    sealed_stage: Option<String>,
    survey: FoldSurvey,
}

impl RunState {
    /// The empty state. **`pub(in crate::workflow)`** on `RequiredSet`'s doctrine
    /// (`src/select.rs`:97-105).
    pub(in crate::workflow) fn initial(item: &str, stage: &str) -> Self {
        Self {
            item: item.to_owned(),
            stage: stage.to_owned(),
            stages_entered: Vec::new(),
            last_transition: None,
            gates_open: BTreeMap::new(),
            gates_by_kind: BTreeMap::new(),
            gates_resumed: BTreeSet::new(),
            gates_answered: BTreeMap::new(),
            gates_void: BTreeSet::new(),
            gates_expired: BTreeSet::new(),
            dispatched: BTreeSet::new(),
            committed: BTreeSet::new(),
            step_commits: BTreeSet::new(),
            completed: BTreeSet::new(),
            attempts: BTreeMap::new(),
            memo_hits: 0,
            amendments: 0,
            halt: None,
            sealed: None,
            sealed_stage: None,
            survey: FoldSurvey::default(),
        }
    }

    #[must_use]
    pub fn item(&self) -> &str {
        &self.item
    }

    #[must_use]
    pub fn stage(&self) -> &str {
        &self.stage
    }

    #[must_use]
    pub fn stages_entered(&self) -> &[String] {
        &self.stages_entered
    }

    /// The `(from, to)` pair the most recent recorded `stage_transition` carries.
    #[must_use]
    pub const fn last_transition(&self) -> Option<&(String, String)> {
        self.last_transition.as_ref()
    }

    /// Every gate the journal opened and did not expire, in token order.
    pub fn open_gates(&self) -> impl Iterator<Item = &GateRecord> {
        self.gates_open.values()
    }

    /// How many times this gate step has been reached — the count of runtime-shaped
    /// `gate_opened` rows carrying `kind` (AC-524's per-(item, gate step) counter).
    #[must_use]
    pub fn reaches(&self, kind: &str) -> u32 {
        self.gates_by_kind
            .get(kind)
            .map_or(0, |v| u32::try_from(v.len()).unwrap_or(u32::MAX))
    }

    /// Every token minted for this gate step, in reach order (reach 1 first).
    ///
    /// *(2026-09-26, `s8d` build, package P5; R2 decision 5.)* An escalate gate is answered once per
    /// re-entry: the walk's `n`-th escalation through the gate reads the `n`-th reach that was not
    /// voided (a void reach is re-opened at the next, AC-252), so it needs the reaches in order.
    #[must_use]
    pub fn gates_of(&self, kind: &str) -> &[Digest] {
        self.gates_by_kind.get(kind).map_or(&[], Vec::as_slice)
    }

    /// The most recently minted token for this gate step, if any reach has happened.
    #[must_use]
    pub fn latest_gate(&self, kind: &str) -> Option<&Digest> {
        self.gates_by_kind.get(kind).and_then(|v| v.last())
    }

    /// The record a token was minted with, if this journal opened it (open, answered or void).
    #[must_use]
    pub fn gate_record(&self, token: &Digest) -> Option<&GateRecord> {
        self.gates_open.get(token)
    }

    #[must_use]
    pub fn is_answered(&self, token: &Digest) -> bool {
        self.gates_answered.contains_key(token)
    }

    #[must_use]
    pub fn is_void(&self, token: &Digest) -> bool {
        self.gates_void.contains(token)
    }

    #[must_use]
    pub fn is_expired(&self, token: &Digest) -> bool {
        self.gates_expired.contains(token)
    }

    #[must_use]
    pub fn is_resumed(&self, token: &Digest) -> bool {
        self.gates_resumed.contains(token)
    }

    /// Every gate whose answer matched the gate's tree hash.
    pub fn answered_gates(&self) -> impl Iterator<Item = &GateAnswer> {
        self.gates_answered.values()
    }

    /// Every gate whose answer does not count, from **two** sources as of 2026-09-14:
    ///
    /// 1. **AC-039's *"void, not reconciled"***: the answer arrived against a different tree hash;
    /// 2. the answer arrived after the journal **expired** the gate — the module header's
    ///    *"`gate_answered` is three cases"*, which carries the whole of that warrant.
    ///
    /// Nothing in this module ever moves a token out of this set, which is what makes both
    /// dispositions terminal.
    pub fn void_gates(&self) -> impl Iterator<Item = &Digest> {
        self.gates_void.iter()
    }

    /// Every gate the journal expired.
    pub fn expired_gates(&self) -> impl Iterator<Item = &Digest> {
        self.gates_expired.iter()
    }

    /// Every committed activation, as the three-part idempotency key the journal recorded.
    pub fn committed(&self) -> impl Iterator<Item = &IdempotencyKey> {
        self.committed.iter()
    }

    /// `true` when this activation is **complete**. The **attempt** is not part of the question: a
    /// committed activation is not re-dispatched under a higher attempt number.
    ///
    /// *(Amended 2026-09-26, `s8d` build, package P3; record R1, proposed.)* This read *"`true` when
    /// some attempt of this activation committed"*, and at the activation grain the two were one
    /// fact. At step grain they are two: an activation whose first agent step committed and whose
    /// walk then died is committed in PART and not complete. The answer is now the completion — a
    /// pre-R1 row with no `step_id` (the activation grain's one commit, which completed it), or
    /// R1's completion row (`completion: true`, the `emit` step's). `next_action` moves past a node
    /// only on this, and the step-grain question is [`Self::is_step_committed`].
    #[must_use]
    pub fn is_committed(&self, item: &str, activity_id_at_version: &str) -> bool {
        self.completed
            .iter()
            .any(|(i, a)| i == item && a == activity_id_at_version)
    }

    /// **R1: has this step activation committed?** — the commit key folded, step and iteration path
    /// included. Re-entry serves a step for which this is `true` and never re-dispatches it.
    #[must_use]
    pub fn is_step_committed(&self, key: &super::idem::CommitKey) -> bool {
        self.step_commits.contains(key)
    }

    /// Every committed step activation, under its commit key.
    pub fn step_commits(&self) -> impl Iterator<Item = &super::idem::CommitKey> {
        self.step_commits.iter()
    }

    /// The attempt number the next dispatch of this activation would carry — one above the highest
    /// the journal records, and `1` for an activation it has never seen. **AC-011's
    /// strictly-increasing attempt expressed as a fold**, rather than as a counter a loop holds.
    #[must_use]
    pub fn next_attempt(&self, item: &str, activity_id_at_version: &str) -> u32 {
        self.attempts
            .get(&(item.to_owned(), activity_id_at_version.to_owned()))
            .map_or(1, |a| a.saturating_add(1))
    }

    /// `memo_hit` count — AC-019's identity reads this beside the executed ledger, at Slice 4h.
    #[must_use]
    pub const fn memo_hits(&self) -> usize {
        self.memo_hits
    }

    /// `dag_amended` count — ADR-0120's delta.
    #[must_use]
    pub const fn amendments(&self) -> usize {
        self.amendments
    }

    #[must_use]
    pub const fn halt(&self) -> Option<&HaltReason> {
        self.halt.as_ref()
    }

    #[must_use]
    pub fn sealed(&self) -> Option<&str> {
        self.sealed.as_deref()
    }

    #[must_use]
    pub const fn survey(&self) -> FoldSurvey {
        self.survey
    }

    /// The first open gate that has not been **answered**, in token order.
    ///
    /// 🛑 **A VOID ANSWER LEAVES THE GATE AWAITING, and this is a reading no record states.**
    /// AC-039 says a stale answer is *"void, not reconciled"* and ADR-0022 says the kernel *"takes
    /// no action until it is answered"*; neither says whether a voided gate counts as answered.
    /// It does not, here, and the reading is fail-closed: a void answer **is not an answer**, so
    /// the gate stays open and the run stays halted until a real one arrives. The alternative —
    /// treating the void token as settled — would let a stale answer release a gate by being
    /// rejected, which is the failure AC-039's clause exists to prevent, one step removed. Asserted
    /// in both directions by `ac039_gate_record_and_the_void_answer`. A record settling it would
    /// make the assertion authoritative rather than merely stated.
    #[must_use]
    pub fn awaiting_gate(&self) -> Option<&Digest> {
        // A void token is TERMINAL (ADR-0017: void, not reconciled; AC-252: the kernel's next act
        // after a void is a fresh gate). It stays in `gates_open` as the record the trailers
        // carry, but it is not what the run awaits — otherwise a voided gate would hold the run
        // at `AwaitingGate` on a dead token for ever, which is "nothing" in AC-252's *"never by
        // nothing"*.
        // An EXPIRED token is not awaited either: the record stays (it is what the trailers carry
        // and what an escalation names), the expiry policy's terminal path decides what follows.
        self.gates_open.keys().find(|t| {
            !self.gates_answered.contains_key(*t)
                && !self.gates_void.contains(*t)
                && !self.gates_expired.contains(*t)
        })
    }

    /// **AC-008's *"the recorded terminal state"***, read off the journal alone.
    ///
    /// The order below is the definition and is written out because it is the answer:
    ///
    /// 1. a `slice_sealed` → [`TerminalState::Sealed`] — *(dated 2026-09-28, `s8e` build, package
    ///    S8E-P7; D21-93)* **only while the fold is in the stage the seal was made in**: a later
    ///    `stage_transition` ends the seal's scope, and the order falls through to 2–5. The verdict
    ///    stays readable as history through [`Self::sealed`] (ADR-0109 decision 3);
    /// 2. an open, unanswered, un-void gate → [`TerminalState::AwaitingGate`] (ADR-0022);
    /// 3. nothing moved the state at all → [`TerminalState::NotStarted`] — **the value a kernel
    ///    that does nothing at all returns**;
    /// 4. every activation the journal dispatched has committed → [`TerminalState::StageComplete`];
    /// 5. otherwise [`TerminalState::AwaitingDispatch`].
    #[must_use]
    pub fn terminal(&self) -> TerminalState {
        if let Some(verdict) = &self.sealed
            && self.sealed_stage.as_deref() == Some(self.stage.as_str())
        {
            return TerminalState::Sealed {
                verdict: verdict.clone(),
            };
        }
        if let Some(token) = self.awaiting_gate() {
            return TerminalState::AwaitingGate {
                token: token.clone(),
            };
        }
        // **AC-008's own named failure mode has a home.** A fold that saw nothing that moves the
        // state lands here, and the replay's comparison against the RECORDED terminal state is
        // what turns that into a red test rather than a pass.
        if self.survey.state_moving == 0 {
            return TerminalState::NotStarted;
        }
        if !self.dispatched.is_empty()
            && self
                .dispatched
                .iter()
                .all(|(item, activity)| self.is_committed(item, activity))
        {
            return TerminalState::StageComplete {
                stage: self.stage.clone(),
            };
        }
        TerminalState::AwaitingDispatch {
            stage: self.stage.clone(),
        }
    }

    fn to_json(&self) -> serde_json::Value {
        let transition = match &self.last_transition {
            Some((from, to)) => serde_json::json!({ "from": from, "to": to }),
            None => serde_json::Value::Null,
        };
        serde_json::json!({
            "amendments": self.amendments,
            "committed": self
                .committed
                .iter()
                .map(|k| serde_json::json!({
                    "activity_id_at_version": k.activity_id_at_version,
                    "attempt": k.attempt,
                    "item": k.item,
                }))
                .collect::<Vec<_>>(),
            "gates_answered": self
                .gates_answered
                .values()
                .map(GateAnswer::to_json)
                .collect::<Vec<_>>(),
            "gates_expired": self
                .gates_expired
                .iter()
                .map(|t| serde_json::Value::from(t.as_str()))
                .collect::<Vec<_>>(),
            "gates_open": self
                .gates_open
                .values()
                .map(GateRecord::to_json)
                .collect::<Vec<_>>(),
            "gates_void": self
                .gates_void
                .iter()
                .map(|t| serde_json::Value::from(t.as_str()))
                .collect::<Vec<_>>(),
            "halt": self
                .halt
                .as_ref()
                .map_or(serde_json::Value::Null, HaltReason::to_json),
            "item": self.item,
            "last_transition": transition,
            "memo_hits": self.memo_hits,
            "sealed": self
                .sealed
                .clone()
                .map_or(serde_json::Value::Null, serde_json::Value::from),
            "stage": self.stage,
            "stages_entered": self.stages_entered,
            "terminal": self.terminal().to_json(),
        })
    }

    /// The canonical byte form the replay compares.
    ///
    /// # Errors
    ///
    /// [`CanonError`] if a float reached the serialiser.
    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CanonError> {
        canonical(&self.to_json())
    }

    fn note_attempt(&mut self, key: &IdempotencyKey) {
        let at = (key.item.clone(), key.activity_id_at_version.clone());
        self.attempts
            .entry(at)
            .and_modify(|a| *a = (*a).max(key.attempt))
            .or_insert(key.attempt);
    }
}

// ---------------------------------------------------------------------------------------------
// The three free functions.
// ---------------------------------------------------------------------------------------------

/// Fold a recorded journal into a state.
///
/// **Total over the closed set (`EventKind::ALL`), with no catch-all arm**, so a new kind is an
/// `E0004` (~~the closed 38 kinds … a thirty-ninth kind~~ *(dated 2026-09-28, `s8e` build, package R3FIX-8e: ADR-0185's three kinds made `ALL` 41)*)
/// here exactly as it is in `EventKind::tier()` — *"a kind cannot enter the system without someone
/// deciding whether losing it would change the next action."*
///
/// # Errors
///
/// [`FsmError`] — a payload member the fold reads is absent or of the wrong shape, an answer names
/// a token no derived gate carries, or a float reached the canonical serialiser.
pub fn fold(item: &str, stage: &str, events: &[Event]) -> Result<RunState, FsmError> {
    let mut state = RunState::initial(item, stage);
    for event in events {
        apply(&mut state, event)?;
    }
    Ok(state)
}

/// The fold's step.
///
/// **Private.** A `pub fn apply(&mut RunState, &Event)` would be the mutable machine this module
/// exists to refuse: with [`fold`] the only public route, a state is always the fold of a journal
/// somebody recorded.
fn apply(state: &mut RunState, event: &Event) -> Result<(), FsmError> {
    state.survey.events_folded += 1;
    if event.kind.tier() == Tier::One {
        state.survey.tier_one += 1;
    }
    // TOTAL OVER THE CLOSED SET (`EventKind::ALL`), WITH NO CATCH-ALL ARM. *(Corrected 2026-09-28,
    // `s8e` build, package S8E-P5, `s8e` §8e.13 finding 9: this comment used to state the enum's
    // size, which went stale at every mint, so it names no count.)* Grouped or-patterns, because
    // `clippy::match_same_arms` is denied workspace-wide and grouping is what the lint asks for.
    match event.kind {
        EventKind::StageTransition => {
            state.survey.state_moving += 1;
            let to = required_str(event, "stage_transition", "to")?;
            let from = optional_str(event, "from").unwrap_or_else(|| state.stage.clone());
            state.stages_entered.push(to.clone());
            state.last_transition = Some((from, to.clone()));
            state.stage = to;
        }
        EventKind::GateOpened => {
            // TWO PRODUCERS, ONE KIND — the module header's reading 3, added 2026-09-14. The
            // runtime's durable mark carries `tree_hash` and `deadline`; the selection's
            // announcement cannot (AC-041 forbids `select()` both a clock and a git read).
            let runtime_shaped =
                event.payload.get("tree_hash").is_some() || event.payload.get("deadline").is_some();
            if runtime_shaped {
                state.survey.state_moving += 1;
                let kind = required_str(event, "gate_opened", "kind")?;
                let tree_hash = required_str(event, "gate_opened", "tree_hash")?;
                let deadline = required_i64(event, "gate_opened", "deadline")?;
                // `reach` (AC-524, ADR-0105 clause 10): absent on a row written before this member
                // existed, and that is reach 1 — the only reach a pre-7b-ii runtime could write.
                let reach = match event.payload.get("reach") {
                    None => 1,
                    Some(v) => v
                        .as_u64()
                        .and_then(|n| u32::try_from(n).ok())
                        .filter(|n| *n >= 1)
                        .ok_or_else(|| FsmError::MalformedPayload {
                            kind: "gate_opened",
                            detail: format!(
                                "`reach` is present but is not a positive integer: {v}"
                            ),
                        })?,
                };
                let record = GateRecord::derive_reach(
                    &kind,
                    &tree_hash,
                    Timestamp::from_unix_seconds(deadline),
                    reach,
                )?;
                state
                    .gates_by_kind
                    .entry(kind)
                    .or_default()
                    .push(record.token.clone());
                state.gates_open.insert(record.token.clone(), record);
            } else if event.payload.get("reason").is_some() {
                // The selection's announcement. Counted, never dropped silently, and it moves no
                // state: ADR-0022's mark has not been written, so the run is not awaiting a gate.
                state.survey.gates_announced += 1;
            } else {
                return Err(FsmError::MalformedPayload {
                    kind: "gate_opened",
                    detail: "carries neither the runtime's `tree_hash`/`deadline` nor the \
                             selection's `reason` — it is neither producer's shape, and this fold \
                             refuses rather than guessing which"
                        .to_owned(),
                });
            }
        }
        EventKind::GateAnswered => {
            state.survey.state_moving += 1;
            let token = required_digest(event, "gate_answered", "token")?;
            let tree_hash = required_str(event, "gate_answered", "tree_hash")?;
            let verdict = required_str(event, "gate_answered", "verdict")?;
            // **THE SPENT TOKEN, 2026-09-17 (AC-246; ADR-0017's single-use token).** A token the
            // answered set already holds, or one already voided, is CONSUMED: a second
            // `gate_answered` carrying it moves NOTHING — the first answer stands, a void stays
            // void — and the row is counted, never dropped and never an error (an error here
            // would make the journal unfoldable for ever, D-5's lesson). Until this arm existed
            // the lookup below asked only "is the token open?", and an answered token IS still
            // open, so a duplicate silently rewrote the verdict [measured 2026-09-17: `approve ->
            // changes-requested`; ADR-0123's evidence line]. The refusal's own record is
            // `gate_answer_spent`, appended by the answer path (`super::gate::answer`) and folded
            // as a no-op below; the mechanism is this set membership, not that row.
            if state.gates_answered.contains_key(&token) || state.gates_void.contains(&token) {
                state.survey.spent_refusals += 1;
                return Ok(());
            }
            // The expired case of the three-way split below, checked FIRST since 2026-09-17: the
            // `gate_expired` arm no longer removes the record from `gates_open` (an escalation
            // names it and the trailers carry it), so "is it open?" would now say yes for an
            // expired gate. Same disposition as before: VOID, and the fold CONTINUES.
            if state.gates_expired.contains(&token) {
                state.gates_void.insert(token);
                return Ok(());
            }
            // **THE THREE-WAY SPLIT, 2026-09-14.** Until today this arm asked one question — *is
            // the token in `gates_open`?* — and gave one answer to two different situations. The
            // `GateExpired` arm below REMOVES the token from `gates_open`, so
            // `gate_opened -> gate_expired -> gate_answered` fell into the `else` and refused; and
            // because the journal is append-only, it refused for ever. Only ONE of the two
            // situations is forgery, and the refusal is the wrong sentence for the other.
            //
            // 🛑 **THE WARRANT IS NARROW AND DOES NOT INCLUDE RE-OPENING.** What is settled here is
            // that an answer the system's own expiry raced is not a forged token. Whether a voided
            // gate RE-OPENS is gate-record semantics and belongs to **7b-ii** (AC-176 classifies a
            // `tree_hash`-drifted answer void *and* re-opens, but its Boundary note assigns those
            // semantics to the gates area). Step 2 of `next_action` is UNCHANGED by this arm and
            // still declines to re-open an expired token; that is the behaviour that was already
            // here, not a reading taken today.
            let Some(gate) = state.gates_open.get(&token) else {
                // The expired case: VOID, and the fold CONTINUES. Nothing is minted —
                // `EventKind::GateAnswerVoid` already exists and its arm already writes exactly
                // this, and `gates_void` is already terminal (nothing in this module moves a token
                // out of it), so a void answer can never later count as one.
                if state.gates_expired.contains(&token) {
                    state.gates_void.insert(token);
                    return Ok(());
                }
                // The forgery case, and the ONLY one: a token no derived gate this journal opened
                // ever carried. The diagnostic's sentence — *"a forged token cannot steer the
                // state"* — is correct **here**.
                return Err(FsmError::GateAnsweredWithoutOpen { token });
            };
            // **AC-039's machine residue.** An answer whose tree hash no longer matches the gate's
            // is VOID, NOT RECONCILED: nothing in this module moves a token out of `gates_void`,
            // so there is no path by which a stale answer later counts — and `awaiting_gate` does
            // not treat it as settled either.
            if gate.tree_hash == tree_hash {
                state.gates_answered.insert(
                    token.clone(),
                    GateAnswer {
                        token,
                        tree_hash,
                        verdict,
                    },
                );
            } else {
                state.gates_void.insert(token);
            }
        }
        EventKind::GateExpired => {
            state.survey.state_moving += 1;
            let token = required_digest(event, "gate_expired", "token")?;
            // 🛑 A GATE THAT WAS NEVER OPEN CANNOT EXPIRE, AND RECORDING IT AS EXPIRED WOULD
            // REOPEN THE FORGERY HOLE D-5 EXISTS TO KEEP SHUT.
            //
            // `GateOpened` above DERIVES its token from `(kind, tree_hash, deadline)`, so a token in
            // `gates_open` is unforgeable. This arm reads the token VERBATIM off the payload, and until
            // 2026-09-15 it did `gates_open.remove(&token)` — which silently no-ops for a token no gate
            // ever opened — followed by an UNCONDITIONAL `gates_expired.insert(token)`. `gates_expired`
            // has exactly one writer, which is this line.
            //
            // The consequence ran straight through D-5's three-way split: append one `gate_expired`
            // naming any digest, and a later `gate_answered` for that same digest folds as **void and
            // continues** instead of halting as `GateAnsweredWithoutOpen`. The branch D-5 kept
            // deliberately — *"token never seen -> forgery, and the message is correct HERE"* — became
            // reachable-around with one extra row, and the fold would have reported the run healthy.
            //
            // So expiry is recorded only for a gate this fold actually SAW open. A `gate_expired` for
            // anything else moves no state: it is counted (the survey still increments above, so it is
            // never a silently dropped row) and then ignored, and the answer that follows it still
            // meets the forgery halt. This decides nothing about gate semantics that D-5 reserved to
            // 7b-ii — it restores the precondition D-5 was written against.
            //
            // 2026-09-17 (slice C): the record is KEPT in `gates_open` — an escalation names it
            // and `stage_action` carries it — and the row's `action` member, when present, is
            // ADR-0017's terminal path taken: `fail` halts the run (AC-256/AC-259's first
            // terminus), `auto-decide` answers the gate with the policy's verdict at the gate's
            // own tree hash, `escalate` is recorded here and the escalation gate arrives as its
            // own `gate_opened` row. A row with no `action` (the shape written before this
            // member existed) expires the gate and decides nothing further.
            let already_settled = state.gates_answered.contains_key(&token)
                || state.gates_void.contains(&token)
                || state.gates_expired.contains(&token);
            if let Some(record) = state.gates_open.get(&token)
                && !already_settled
            {
                let bound_tree = record.tree_hash.clone();
                state.gates_expired.insert(token.clone());
                match optional_str(event, "action").as_deref() {
                    Some("fail") => {
                        state.halt = Some(HaltReason::GateExpired { token });
                    }
                    Some("auto-decide") => {
                        let verdict = required_str(event, "gate_expired", "verdict")?;
                        state.gates_answered.insert(
                            token.clone(),
                            GateAnswer {
                                token,
                                tree_hash: bound_tree,
                                verdict,
                            },
                        );
                    }
                    _ => {}
                }
            }
        }
        EventKind::SliceSealed => {
            state.survey.state_moving += 1;
            state.sealed = Some(required_str(event, "slice_sealed", "verdict")?);
            // D21-93: the seal is scoped to the stage it was made in (dated 2026-09-28, S8E-P7).
            state.sealed_stage =
                Some(optional_str(event, "stage").unwrap_or_else(|| state.stage.clone()));
        }
        EventKind::GateAnswerVoid => {
            state.survey.state_moving += 1;
            let token = required_digest(event, "gate_answer_void", "token")?;
            state.gates_answered.remove(&token);
            state.gates_void.insert(token);
        }
        EventKind::ActivityCommitted => {
            state.survey.state_moving += 1;
            let key = activation_key(event, "activity_committed")?;
            state.note_attempt(&key);
            // *(dated 2026-09-27, `s8d` build, package D2176-8d; D21-76.)* A row carrying a
            // `decision` is a control decision the walk journaled (a `check`'s verdict, a
            // `repeat`'s `until`, a `gate` step's `when`), not a commit: it takes its attempt
            // (so the triple stays unique) and moves nothing else. The walk replays it on re-entry.
            // *(dated 2026-09-27, `s8d` build, package D2177-8d; D21-77 ruling 2a.)* The attempt it
            // takes moves `next_attempt`, which numbers rows; AC-029's retry cap does not read it
            // (`run_activity_at` counts the activation's `dispatch` rows).
            if event.payload.get("decision").is_some() {
                return Ok(());
            }
            // *(R1, `s8d` P3.)* A row with no `step_id` is the activation grain's one commit, and
            // it completed its activation. A row WITH one is a step activation's commit under the
            // commit key; it completes the activation only when it is the completion row.
            match optional_str(event, "step_id") {
                None => {
                    state
                        .completed
                        .insert((key.item.clone(), key.activity_id_at_version.clone()));
                }
                Some(step_id) => {
                    let iteration_path = event
                        .payload
                        .get("iteration_path")
                        .and_then(super::idem::IterationPath::from_json)
                        .ok_or_else(|| FsmError::MalformedPayload {
                            kind: "activity_committed",
                            detail: "a row carrying `step_id` carries no readable \
                                     `iteration_path` (an array of frame tokens, `[]` outside a \
                                     loop); R1's commit key is never folded with a guessed path"
                                .to_owned(),
                        })?;
                    if event
                        .payload
                        .get("completion")
                        .and_then(serde_json::Value::as_bool)
                        == Some(true)
                    {
                        state
                            .completed
                            .insert((key.item.clone(), key.activity_id_at_version.clone()));
                    }
                    state.step_commits.insert(super::idem::CommitKey {
                        item: key.item.clone(),
                        activity_id_at_version: key.activity_id_at_version.clone(),
                        step_id,
                        iteration_path,
                    });
                }
            }
            state.committed.insert(key);
        }
        EventKind::Dispatch => {
            state.survey.state_moving += 1;
            let key = activation_key(event, "dispatch")?;
            state.note_attempt(&key);
            state
                .dispatched
                .insert((key.item, key.activity_id_at_version));
        }
        EventKind::StepRefused => {
            state.survey.state_moving += 1;
            let class = required_str(event, "step_refused", "class")?;
            state.halt = Some(HaltReason::Refused { class });
        }
        EventKind::MemoHit => {
            state.survey.state_moving += 1;
            state.memo_hits += 1;
        }
        EventKind::DagAmended => {
            state.survey.state_moving += 1;
            state.amendments += 1;
        }
        // NO STATE CHANGE, in one grouped arm, and the reason is one sentence: none of these
        // eighteen changes what the runtime does next. `step_failed`, `step_cancelled` and
        // `rate_limited` are the retry policy's input (`super::retry`), not the FSM's — a failure
        // moves the attempt number, and `dispatch` above is where that is read. The memo misses
        // and evictions, the drift rows, the advisory rows, the artifact rows, the rate readings
        // and `rewrite` are telemetry. `dag_assembled` is the selection the state is read AGAINST
        // rather than a fact about the run's progress. `reconcile_started` is `rk-git`'s.
        // `gate_answer_spent` (ADR-0123) is the RECORD of a refusal, not its mechanism: the
        // refusal is derived from `gates_answered`/`gates_void` in the `gate_answered` arm above.
        // The row moves no member of the state and is counted, so `spent_recorded` can be held
        // against `spent_refusals` by a replay assertion.
        EventKind::GateAnswerSpent => {
            state.survey.spent_recorded += 1;
        }
        // `gate_resumed` (ADR-0123) is resume telemetry; its `prefix_rewrites <= 1` is AC-279's
        // assertion over the journal, not a fold effect. The token is kept so the runtime writes
        // one row per resumed gate and never a second.
        EventKind::GateResumed => {
            let token = required_digest(event, "gate_resumed", "token")?;
            state.gates_resumed.insert(token);
        }
        // The review and verify gates' five (ADR-0124) are the gates' own rows — blocks, quarantine
        // dispositions, the bootstrap comparator's reclassification — and move no member of the
        // run's state; `crate::review` folds them for its own invariants (AC-307, AC-310, AC-530).
        // `dispatch_completed` (ADR-0126, 2026-09-17) is the adapter's usage-bearing row — the
        // completed `DispatchRecord`, tier-2 telemetry — and its decision 5 routes it here in
        // terms: *"the kernel folds it and moves nothing"*. It opens no gate and answers none.
        EventKind::ReconcileStarted
        | EventKind::DispatchCompleted
        | EventKind::ReviewBlock
        | EventKind::QuarantineGranted
        | EventKind::QuarantineRemoved
        | EventKind::QuarantineExpired
        | EventKind::SelectionIneffective
        | EventKind::StageExitRefused
        | EventKind::DagAssembled
        | EventKind::MemoMiss
        | EventKind::MemoEvicted
        | EventKind::StepFailed
        | EventKind::StepCancelled
        | EventKind::RateLimited
        | EventKind::RateLimitReading
        | EventKind::Degraded
        | EventKind::StructuredDrift
        | EventKind::DriftAdvisory
        | EventKind::ReviewClassDrift
        | EventKind::Rewrite
        | EventKind::ArtifactRead
        | EventKind::ArtifactWrite
        | EventKind::AdvisoryCreated
        | EventKind::AdvisoryRetired
        | EventKind::AdvisoryDismissed
        | EventKind::WorktreeAssigned
        | EventKind::StageEmpty
        | EventKind::GitOp => {}
    }
    Ok(())
}

/// The full-grain next action.
///
/// The evaluation order **is** the answer, so it is written out rather than inferred from the body:
///
/// 1. an open gate that is neither answered nor void → [`NextAction::AwaitGate`] (ADR-0022:
///    escalation is a kernel-owned durable `Gate:` mark and the kernel takes no action until it is
///    answered);
/// 2. a gate the selection requires that the journal has not opened → [`NextAction::OpenGate`];
/// 3. a node in step 6's topological order whose activation is not committed →
///    [`NextAction::Dispatch`];
/// 4. a recorded halt → [`NextAction::Halt`] — **above the successor check, ADR-0154: a recorded
///    halt outranks a supplied successor**, so a refused run is never reported as advancing;
/// 5. the plan exhausted, a successor supplied, **and that successor is not the stage the fold is
///    already in** → [`NextAction::AdvanceStage`] (ADR-0157: a stage does not advance into itself);
/// 6. otherwise [`NextAction::Done`].
///
/// # Errors
///
/// [`FsmError::PlanNotOrderable`] if step 6 cannot order the DAG; [`FsmError::Canon`] if a float
/// reached the canonical serialiser while a gate token was derived.
pub fn next_action(
    selection: &Selection,
    state: &RunState,
    ctx: &FsmContext<'_>,
) -> Result<NextAction, FsmError> {
    // 1. ADR-0022: escalation is a kernel-owned durable `Gate:` mark and the kernel takes no
    //    action until it is answered — not a dispatch, and not a stage advance.
    if let Some(token) = state.awaiting_gate() {
        return Ok(NextAction::AwaitGate {
            token: token.clone(),
        });
    }
    // 2. The `kind` spelling is a READING — see the module header's first reading. Design §5 gives
    //    `gate_opened` a `reason` and an `axis`; `GateRecord` carries one `kind`, and two gates
    //    differing only in axis must not derive one token and collapse into one gate.
    for gate in selection.gates() {
        let kind = format!("{}:{}", gate.reason.as_str(), gate.axis);
        // The gate STEP decides, not a token derived from the caller's context: the reach
        // counter says whether this step has been reached, and the latest record's disposition
        // says what to do. (Until 2026-09-17 this derived `(kind, ctx.tree_hash, ctx.deadline)`
        // and looked the token up — so a tree that moved after an answer would have re-opened an
        // answered gate on every call, and a void token could never re-open at all.)
        let reaches = state.reaches(&kind);
        if reaches == 0 {
            return Ok(NextAction::OpenGate(GateRecord::derive_reach(
                &kind,
                ctx.tree_hash,
                ctx.deadline,
                1,
            )?));
        }
        // **AC-252 — a void is followed by a re-open, fresh in all three fields.** The record is
        // minted over the CURRENT tree and the deadline the caller supplies at this act; the
        // reach is one past the voided one, so the token is fresh even if neither moved.
        // (`DECISIONS-2026-09-14 D-5` left "does a voided gate re-open?" to this stage; the
        // register answers it in AC-252 and this arm executes that answer.) An EXPIRED latest
        // record is deliberately NOT re-opened here: its disposition is the expiry policy's
        // terminal path (ADR-0017: fail / escalate / auto-decide; AC-257, AC-259), not a bare
        // re-open — and an answered one proceeds.
        if let Some(latest) = state.latest_gate(&kind)
            && state.gates_void.contains(latest)
        {
            return Ok(NextAction::OpenGate(GateRecord::derive_reach(
                &kind,
                ctx.tree_hash,
                ctx.deadline,
                reaches.saturating_add(1),
            )?));
        }
    }
    // 3. Step 6's topological order is the source of *which one*, and step 6 is already on disk as
    //    `Selection::plan`. Nothing here re-derives an order.
    let plan = selection.plan().map_err(|e| FsmError::PlanNotOrderable {
        detail: e.to_string(),
    })?;
    for node_id in plan.order() {
        let Some(node) = selection.dag().nodes().iter().find(|n| n.id == node_id) else {
            continue;
        };
        // The header's fifth supplied value. **`unwrap_or` on the bare id is the ARMED state, not
        // a default**: with no activation supplied the key is exactly what it was before
        // 2026-09-14, and `crate::workflow::idem::checked_key` refuses it for want of an
        // `@version`. Nothing here derives a version from the catalog — that join is the decision
        // this module does not take.
        let activation = ctx
            .activations
            .iter()
            .find(|(id, _)| *id == node.id)
            .map_or(node.activity_id.as_str(), |(_, at)| at.as_str());
        if state.is_committed(ctx.item, activation) {
            continue;
        }
        return Ok(NextAction::Dispatch {
            key: IdempotencyKey {
                item: ctx.item.to_owned(),
                activity_id_at_version: activation.to_owned(),
                attempt: state.next_attempt(ctx.item, activation),
            },
            step_id: node.id.clone(),
        });
    }
    // 4. A recorded halt, at design §3.2's own position in the order — AFTER the plan, so a run
    //    with work still dispatchable does the work, and BEFORE `Done`, so a refused run is not
    //    reported finished.
    //
    //    🛑 **AND BEFORE `AdvanceStage` (ADR-0154): a recorded halt OUTRANKS a supplied
    //    successor.** It sat below the successor check until 7h, which was unobservable only
    //    because `ctx.successor` was `None` on every real run — `base_inputs` never chained
    //    `advances_to`. The moment 7h supplies one (ADR-0152), the old order made a refused run
    //    with an exhausted plan ADVANCE A STAGE, writing a tier-1 `stage_transition` and leaving
    //    the next stage to start from a journal whose refusal nothing acted on — the exact outcome
    //    the "BEFORE `Done`" clause above exists to prevent. The test is on the COMBINATION: the
    //    order sweep asserted `Halt` against the NO-successor context, so a fixture per arm did
    //    not instrument the pair.
    if let Some(reason) = state.halt.clone() {
        return Ok(NextAction::Halt { reason });
    }
    // 5. The successor is an ARGUMENT and is never invented here: this system models no stage
    //    ordering (`spikes/p9/packs/incident.toml`:1-2). Which stage it IS comes from the pack's
    //    declared `[policy] stage_order` (ADR-0153), resolved by the composition root.
    //
    //    🛑 **A STAGE DOES NOT ADVANCE INTO ITSELF (ADR-0157), AND THIS WAS MEASURED, NOT
    //    ANTICIPATED.** `from` is the FOLD's stage and `to` is the SUPPLIED successor, and the two
    //    diverge across runs: the composition root computes the successor from the item record's
    //    stage, which never moves, while the fold's `state.stage` follows the `stage_transition`
    //    rows the previous run wrote. A second `redkiln run` over the shipped fix item therefore
    //    folded `state.stage = "analysis"`, was handed the successor `"analysis"`, and appended
    //    `{"from":"analysis","to":"analysis"}` — a tier-1 row for a transition that is not one, and
    //    one more on every subsequent run [measured 2026-09-22 against the real binary].
    //
    //    🛑 **THE TEST IS `stages_entered`, NOT `to != state.stage` — ADR-0161, superseding ADR-0157
    //    clause 1 in part.** The equality form was a PROXY for *"has this transition already been
    //    recorded"*, and it is only sufficient for a lane that takes ONE boundary, which is the only
    //    lane this tree can currently walk. `apply`'s `StageTransition` arm sets `state.stage` to the
    //    LAST transition's target, while the composition root computes the successor from the item
    //    record's stage, which never moves. So for a lane `[triage, analysis, specification]` walked
    //    in one process: run 2 folds `state.stage == "specification"`, is handed the successor
    //    `"analysis"`, and `"analysis" != "specification"` **passes** — appending a BACKWARD
    //    `{"from":"specification","to":"analysis"}` row, then advancing forward again, two tier-1
    //    rows per run for ever. That is the unbounded append ADR-0157 claimed to have closed, one hop
    //    further out, and ADR-0157's own experiment could not have seen it because run 1 stops at
    //    `analysis` with exit 3.
    //
    //    `stages_entered` is the state that answers the real question, and it already existed: the
    //    same `StageTransition` arm pushes every target onto it. A stage the lane has entered is not
    //    entered again.
    if let Some(to) = ctx.successor
        && !state.stages_entered.iter().any(|s| s == to)
        && to != state.stage
    {
        return Ok(NextAction::AdvanceStage {
            from: state.stage.clone(),
            to: to.to_owned(),
        });
    }
    // 6.
    Ok(NextAction::Done)
}

/// The **stage-grain** next action — AC-161's and AC-164's comparison.
///
/// The tier filter is **inside this function**, so a caller holding tier-2 facts cannot leak one
/// into a stage-grain answer. Takes no [`Selection`]: see the module header.
///
/// # Errors
///
/// [`FsmError`] from the fold.
pub fn stage_action(item: &str, stage: &str, events: &[Event]) -> Result<StageAction, FsmError> {
    let tier_one: Vec<Event> = events
        .iter()
        .filter(|e| e.kind.tier() == Tier::One)
        .cloned()
        .collect();
    let state = fold(item, stage, &tier_one)?;
    Ok(StageAction {
        transition: state.last_transition.clone(),
        gates: state.gates_open.values().cloned().collect(),
    })
}

// ---------------------------------------------------------------------------------------------
// The payload readers. Absent or wrongly shaped is a typed refusal, never a silent zero.
// ---------------------------------------------------------------------------------------------

fn required_str(event: &Event, kind: &'static str, member: &str) -> Result<String, FsmError> {
    event
        .payload
        .get(member)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| FsmError::MalformedPayload {
            kind,
            detail: format!("`{member}` is absent or is not a string"),
        })
}

fn optional_str(event: &Event, member: &str) -> Option<String> {
    event
        .payload
        .get(member)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}

fn required_i64(event: &Event, kind: &'static str, member: &str) -> Result<i64, FsmError> {
    event
        .payload
        .get(member)
        .and_then(serde_json::Value::as_i64)
        .ok_or_else(|| FsmError::MalformedPayload {
            kind,
            detail: format!("`{member}` is absent or is not an integer"),
        })
}

fn required_u32(event: &Event, kind: &'static str, member: &str) -> Result<u32, FsmError> {
    let raw = required_i64(event, kind, member)?;
    u32::try_from(raw).map_err(|_| FsmError::MalformedPayload {
        kind,
        detail: format!("`{member}` is {raw}, which is negative or out of u32 — never a silent 0"),
    })
}

fn required_digest(event: &Event, kind: &'static str, member: &str) -> Result<Digest, FsmError> {
    let raw = required_str(event, kind, member)?;
    Digest::from_hex(&raw).map_err(|e| FsmError::MalformedPayload {
        kind,
        detail: format!("`{member}` is not a 64-character lowercase-hex digest: {e:?}"),
    })
}

fn activation_key(event: &Event, kind: &'static str) -> Result<IdempotencyKey, FsmError> {
    Ok(IdempotencyKey {
        item: required_str(event, kind, "item")?,
        activity_id_at_version: required_str(event, kind, "activity_id_at_version")?,
        attempt: required_u32(event, kind, "attempt")?,
    })
}

/// **The AC-001 must-fail control for the CHILD module** — exactly one banned determinism call,
/// behind a non-default feature.
///
/// If this function ever clips **clean** with
/// `--features rk-kernel/workflow-fsm-clock-reintroduce`, `src/workflow.rs`'s module-level
/// `#![deny(clippy::disallowed_methods)]` is **not inherited by its children**, and every module
/// under `src/workflow/` — the budget counters, the retry policy, the breaker, degradation and this
/// FSM — is outside the determinism gate AC-001–AC-004 names while appearing to be inside it.
#[cfg(feature = "workflow-fsm-clock-reintroduce")]
#[must_use]
pub fn fsm_reads_the_wall_clock() -> std::time::SystemTime {
    // AC-001: the wall clock on the workflow side of the split, in a CHILD of the module that
    // carries the attribute.
    std::time::SystemTime::now()
}

/// The cases the **restricted constructor** forces into this file rather than into `tests/it/`.
///
/// `RunState::initial` is `pub(in crate::workflow)`, so no integration test can call it — and
/// therefore no integration test can state the one proposition that makes the fold a *fold*:
/// **folding nothing is the initial state.** `src/select.rs`:689-693 is the precedent and its
/// reason applies verbatim: *"a `pub(crate)` constructor, or a `#[cfg(test)]` public one, silently
/// converts a type fact into a convention"*, so the assertion moves to the code rather than the
/// visibility moving to the assertion.
///
/// Everything else about the FSM is asserted from outside, in `tests/it/fsm.rs` and
/// `tests/it/divergence.rs`, because everything else is reachable through [`fold`].
#[cfg(test)]
mod tests {
    use super::{RunState, TerminalState, fold};

    /// `fold` over an empty journal **is** `initial` — the identity that makes the whole module a
    /// fold rather than a machine with a start-up routine. Not statable from `tests/it/`.
    #[test]
    fn folding_nothing_is_the_initial_state() {
        let initial = RunState::initial("INC-1", "implement");
        let folded = fold("INC-1", "implement", &[]).expect("an empty journal folds");
        assert_eq!(initial, folded);
        assert_eq!(folded.terminal(), TerminalState::NotStarted);
        assert_eq!(initial.survey().events_folded(), 0);
        // The two arguments really are read: an initial state for a different item or stage is a
        // different value, so `initial` is not a constant the identity above is trivially true of.
        assert_ne!(initial, RunState::initial("INC-2", "implement"));
        assert_ne!(initial, RunState::initial("INC-1", "verify"));
    }

    /// The initial state serialises, and it serialises to the value AC-008's *"a kernel that does
    /// nothing at all"* would hand a replay. The recorded side of that comparison is `run.json`'s
    /// and is Slice 4f's; this is the side the FSM owns.
    #[test]
    fn the_initial_state_is_canonical_and_says_it_did_nothing() {
        let bytes = RunState::initial("INC-1", "implement")
            .canonical_bytes()
            .expect("no field of a run state is a float");
        let text = String::from_utf8(bytes).expect("the canonical serialiser emits UTF-8");
        assert!(
            text.contains(r#""terminal":{"terminal":"not_started"}"#),
            "the initial state must announce that nothing moved: {text}"
        );
        // Sorted keys, LF only, no trailing newline — `rk_store::canon`'s three rules.
        assert!(!text.ends_with('\n'));
        assert!(!text.contains('\r'));
        assert!(
            text.find(r#""amendments""#) < text.find(r#""item""#),
            "object keys are sorted byte-wise: {text}"
        );
    }
}
