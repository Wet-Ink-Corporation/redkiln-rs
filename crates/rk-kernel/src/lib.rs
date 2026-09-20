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
//! repairs are Step 7e's, not this stage's. **Component 5's tool branch has no source in this
//! tree and is not invented**; **component 8 is `ActivitySpec.schema`'s whole id** as of
//! 2026-09-14, and the note saying what `schema_version` formally is stays OWED; AC-043's
//! `Evicted`
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
//! **FOUR halves inside it are ARMED and none is claimed** *(this read **five** until 2026-09-14;
//! the count is moved rather than the sentence deleted, so a reader can see which one left)*:
//! AC-013's *"process was killed"* (the boundary here is a dropped `Runtime`, and ADR-0087 clause 2
//! says *"a reload that reads state back in-process is not a valid implementation of this decision
//! and is not evidence for it"* — on top of AC-013 being **PROVISIONAL**; **ROUTED TO 7d** under
//! operator decision **D-8**, because its WHEN names `redkiln run <item>`), AC-030's *"exits 0"*
//! (**ROUTED TO 7d** the same day — this clause read *"AC-024's and AC-030's *\"exits 0\"* (a
//! **process** claim; `rk-cli` is Step 7d)"*, and **AC-024's half left the list**: D-8 built the
//! re-exec harness in `tests/it/cross_process.rs`, which reads a **child's** exit code beside the
//! mark the child wrote — the register's own *"test asserting process exit and a written mark"*.
//! AC-030 stays because its *Verified by* names `redkiln run` itself), AC-030's *"the next run
//! resumes from the mark"* (it rides on AC-013),
//! AC-031's prose-extraction sub-clause (built, and the register itself calls the criterion
//! compound), and AC-110's ledger coverage, which is **POWERLESS**: AC-110's own THEN is a
//! load-time configuration error and says nothing about `executed: { first, last }`.
//!
//! **And TWO halves inside Slice 4d are ARMED, not discharged**: the breaker's **durable mark**
//! (`EventKind` is closed at 29 and ADR-0116 says *"the kind is minted by record or not at all"*)
//! and the resolved card's **model tier**. Each arrives as a caller-supplied argument with one
//! constructor; [`workflow::breaker`]'s and [`workflow::degrade`]'s headers carry the measurements
//! and name the records that are owed.
//!
//! *(This paragraph said "**four halves**" until 2026-09-14 and listed the breaker's **model-family
//! key** and its **expiry (value and home both)** as the other two. Operator decision **D-7** built
//! both: the key is the pack's `[role.<name>] family` resolved from
//! [`catalog::Step::role`] through [`select::Pack::require_family_for_role`], and the expiry is
//! pack-declared at `[breaker] expiry_secs` beside `K`. The count moved 4 → 2 rather than the
//! sentence being deleted, because a reader needs to see which two left and why. What is **still
//! owed** is narrower and is recorded at each site: the five pack key **spellings** are a reading,
//! and the expiry's **value** is sized by no corpus figure — only its home is settled.)*
//!
//! **Slice 2's third rule, 2026-09-14: [`facet_registry`].** *(This paragraph opened "**Two things
//! Slice 2 does not contain**" and its first was "**AC-072 — the registry-entry loader rule — is a
//! THIRD rule with its own eleven-case fixture set**, the register forbids collapsing it into
//! either lint, and it is **not written**". It is written now, and the correction is the behaviour
//! arriving.)* Its subject is the **entry** — every entry, whatever its `source` — and it reads no
//! record at all: the unregistered-facet clause over all three of AC-072's predicate carriers, the
//! declared domain and its shape, the five closed `source` tokens, `derived`'s three fields
//! **together** and `kernel`'s none of them, the `declared` require-row bar, nullability declared
//! **by name**, and ADR-0103 decision 2's lexis. **Three clauses of it are recorded and not built**
//! and the module header carries each measurement: AC-072's **`anchor`** has no on-disk form in any
//! record [measured: one occurrence in `spikes/p9/facets.toml`, inside a comment]; the domain-shape
//! set is **four on disk and three in the register** [measured: `kind`, `cos`, `stage`, `role`]; and
//! the general *"may abstain"* case is decidable from no entry, so the `nullable` clause is pinned
//! at ADR-0089 decision 3's boolean grain. **Three register amendments are OWED** and this stage
//! takes none of the three decisions (`00-preamble.md`:55).
//!
//! **What Slice 2 still does not contain**: all three lints' CLI surfaces. `redkiln lint
//! --facet-stability`, `--facet-provenance` and the registry lint are `rk-cli`'s (**Step 7d**) over
//! `.redkiln/` (**Step 7e**), so those halves are **ARMED, NOT DISCHARGED**. What is discharged here
//! is the fail-closed behaviour, which §7b.14 says is what Step 7b owes in their place.
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
//!
//! # The kernel holds no adapter identity, which is a different claim from the one above
//!
//! [`kernel_denylist`] is **AC-487**, and it is **not** AC-484 one level down. AC-484 is about the
//! dependency **graph**: no `rk-adapter-*` edge, direct or transitive. AC-487 is about this crate's
//! **source**: no comparison against an adapter identity — no adapter name string, no
//! `Any::downcast`, no adapter-discriminant enum — which a graph assertion cannot see, because a
//! `match id { "rk-adapter-sqlite" => … }` needs no edge at all. The rule is a source scan over
//! `crates/rk-kernel/src/` reporting **file and line**, over **code lines only**: this very
//! paragraph, `:172` above and `workflow/retry.rs`:493 name the forbidden crates in prose, and a
//! whole-file `contains` would fire on the sentence describing the ban.
//!
//! 🛑 **AC-487's own note strikes a clause and this crate does not claim it**: *"the lint proves the
//! absence of identity checks; it cannot prove the presence of a capability lookup, so the earlier
//! clause 'and the difference is expressed as a capability lookup' is struck as unverifiable by the
//! named instrument."* That the lookup exists is AC-494's and AC-495's, and **both are built as of
//! 2026-09-14** — [`resolver`] and [`pack_validation`]. The sentence that stood here said *"neither
//! is built"* and is corrected in place rather than deleted, because the reason it was written still
//! holds: [`kernel_denylist`] proves an absence and cannot prove a presence, and the presence is a
//! different instrument in a different module. The two claims stay separate.
//!
//! # The capability lookup is a lookup on DATA, at two moments
//!
//! [`resolver`] is **AC-494** and [`pack_validation`] is **AC-495**, and they are one reading at two
//! moments rather than one rule twice: validation refuses a **pack** at startup whose only route
//! through a stage needs a token no configured adapter claims, naming the requiring row with a
//! **span**; the resolver refuses an **activity** at resolution whose need the claimed set does not
//! hold, with `StepError::Unsupported { capability }` carrying the token. `tests/it/resolver.rs`'s
//! `the_pack_that_cannot_route_and_the_activity_that_cannot_resolve_name_one_token` asserts the two
//! agree on the token, so the pair cannot drift into describing two systems.
//!
//! **Neither module holds an adapter.** The claimed set arrives as a value — `resolver`'s header
//! carries the measurement that **no capability registry exists in this workspace** — so the
//! question both rules ask is *"does this set contain this token?"* and never *"which adapter is
//! this?"*. That is what keeps them consistent with [`kernel_denylist`] rather than in tension with
//! it, and it is design §3.3 P5's *"capability differences are **data**"* read literally.
//!
//! 🛑 **Three halves are ARMED and none is claimed.** Where an activity's capability **need** comes
//! from is a decision no record has taken — `ActivitySpec` carries *"five fields and no sixth"* and
//! the catalog schema declares no `capability` key — so the need is handed in and **a record routing
//! it is OWED** (`00-preamble.md`:55). AC-495's *"before any activity runs"* is discharged as a type
//! fact and not as an ordering inside a live run, which needs `rk-cli`'s composition root at **Step
//! 7d**. And **AC-500 is ARMED, not claimed**: the route-around is a *dispatch-time* behaviour that
//! AC-495's own *Note* trims out of this criterion, and its pack half is Step 7e's.

//! # The three registry rules
//!
//! [`facet_stability`] is **AC-071**, the unblocking gate, and its subject is `source = "llm"`
//! against a scored stability record. [`facet_provenance`] is **AC-520**, the sibling gate F39
//! forced, and its subject is `source = "derived"` against a scored precision record.
//! [`facet_registry`] is **AC-072**, and its subject is the **entry** — every entry, no record at
//! all. They are **three rules with three fixture sets**, never one gate with three subjects:
//! AC-520's own record says why — *"a single gate carrying two different subjects, two different
//! records and two different failure meanings is the shape that invites that collapse"* — and
//! AC-072's 2026-09-13 amendment says it again for the pair it is half of: *"AC-520 owns what the
//! class and the record MEAN, this criterion owns that the ENTRY carries them … and the two must not
//! be collapsed into one lint rule."*
//!
//! # Rigor monotonicity
//!
//! [`rigor`] is **AC-105 / SM-035** over **ADR-0081 clause 1's** superset relation, and it is the
//! first measurement of monotonicity by any tool: `grep -ci monoton spikes/p9/lattice.py` is **0**.
//!
//! # The quality block's terminal condition
//!
//! **And — the last of Slice 2's debt — [`quality_block`], which is AC-527.** `tests/it/
//! evaluator.rs`:236 carried the absence note for it (*"the lint that acts on it **IS NOT BUILT**"*)
//! and that note is retired in place rather than deleted. The rule is on the **FORM**: it evaluates
//! a `check` step's `expr` over a context in which the findings set is empty — and again over one in
//! which it is absent, which is the shape F37's `optional` arm measured — across the three live
//! rigor levels, and refuses it if any cell comes out `true`. That reaches `findings.none(…)`,
//! `findings.all(…)`, `findings == null` and `findings.len() == 0` with one rule, where a denylist
//! of ADR-0080's spelling reaches only the first. **It binds `findings` and `rigor` and nothing
//! else**, and that restraint is measured rather than assumed: binding every read path to `null`
//! makes `X.all(…)` over an unknown collection vacuously true and refuses `mutation-testing`'s
//! threshold and `flag-progressive-rollout`'s re-execution outcome — the two shapes ADR-0080
//! decision 2 *prescribes as the remedy*. **It is red against the shipped catalog by design**: 11 of
//! the 94 live terminal conditions are refused, 83 have no truth value here, and **none is decided
//! lawful**. The repairs are Step 7e's and the CLI surface is Step 7d's; neither is claimed, and an
//! erratum reconciling AC-527's THEN with ADR-0080 decision 2's broader *"satisfiable only by an
//! empty finding set"* is **OWED**. The module header carries the table.

/// **AC-484's dependency-edge reading** — the pure reader that answers *"does `rk-kernel` reach an
/// `rk-adapter-*` crate, directly or transitively?"* from `cargo metadata` output.
///
/// A **top-level** module beside [`kernel_denylist`] and for the same reason: its subject is the
/// **whole crate's** graph, so it cannot sit inside [`workflow`] or [`select`], both of which deny
/// the file reads and the unordered containers such a rule reaches for.
///
/// 🛑 **It reads `resolve.nodes[]` and never `packages[].dependencies[]`.** D-1 (2026-09-14) makes
/// the must-fail fixture a permanent **optional** edge on this crate, which the manifest view lists
/// whether or not its feature is on; an assertion over that view would be red on a green tree. The
/// module header states the trap and `tests/it/adapter_edge.rs` asserts the two readings disagree,
/// so it is a committed assertion rather than a warning to a future reader.
pub mod adapter_edge;
pub mod catalog;
pub mod config;
pub mod facet_provenance;
pub mod facet_registry;
pub mod facet_stability;
/// **AC-487's kernel denylist** — the source rule that proves `rk-kernel` compares against no
/// adapter identity. A **top-level** module beside [`facet_registry`] and deliberately not inside
/// [`workflow`] or [`select`]: it reads files, which both of those deny.
///
/// It is also outside the two modules the other architecture lints police, for the reason
/// `s7b-ii`:534-539 states in general — *"a lint written inside `policy` or `classify` makes the
/// ban fire on its own implementation"*. This rule's subject is the **whole crate**, so it cannot
/// stand outside its own scope; its header records the one marked, counted exemption that follows
/// from that, and `tests/it/kernel_denylist.rs` asserts the exemption's size.
pub mod kernel_denylist;
/// **AC-294's third instrument** — the module-boundary architecture lint, for the **`--stay` class**
/// state the criterion is extended to carry.
///
/// A **top-level** module beside [`kernel_denylist`], for that module's stated reason: a lint
/// written inside the module it polices makes the ban fire on its own implementation
/// (`s7b-ii`:534-539, quoted at [`kernel_denylist`] above). Its subject is `crate::workflow`, so it
/// sits outside it.
///
/// 🛑 **It does NOT cover AC-294's block value.** The criterion's own THEN is about a block value
/// constructed outside `rk-kernel::policy`, and measured 2026-09-15 there is **no `policy` module,
/// no block-value type and no `stage_weight`** anywhere under `crates/*/src/` — so that half is
/// **ARMED, NOT DISCHARGED** and `tests/it/module_boundary.rs`'s table says so by clause.
pub mod module_boundary;
/// **AC-498's egress surface** — the same resolved graph [`adapter_edge`] reads, classified a
/// second time: is a **networking** crate linked?
///
/// A **top-level** module beside [`adapter_edge`], for that module's reason and not a new one: its
/// subject is the **whole crate's** graph. It calls [`adapter_edge::resolved_reading`] and writes
/// no second walker (D-2, 2026-09-14), so the adapter answer and the networking answer are two
/// classifications of **one** reading rather than two readings that could disagree.
///
/// 🛑 **What it cannot see is larger than what it can.** A graph reading answers *"is a networking
/// crate linked?"*; it does not answer *"can this process open a socket?"* — `std::net` needs no
/// dependency and reaches no `resolve.nodes[]` entry — and **no lint closes that**: measured
/// 2026-09-14, `clippy.toml` and `deny.toml` carry **no network entry at all**, so AC-003's network
/// half is enforced by nothing and stays **ARMED**. The instrument for that class is the
/// `no-egress` CI job's denied container, and this module is not a substitute for it. Its header
/// and `tests/it/no_egress.rs`'s carry the measurement.
pub mod no_egress;
/// **AC-495's pack validation** — the startup rule that refuses a stage whose only route needs a
/// capability nothing claims, with a span at the requiring row.
///
/// A **top-level** module and not a child of [`select`], for a reason that is the mirror of
/// [`runtime`]'s: this rule is not part of the selection function. Its verdict feeds no `DagNode`,
/// its `capability` key is deliberately absent from `Pack::to_json` — which is the `dag_assembled`
/// payload AC-040's byte-identity is measured over — and a routing concern living inside `select`
/// would put a startup validation on the pure function's input path.
pub mod pack_validation;
pub mod predicate;
pub mod quality_block;
pub mod registry;
pub mod replay;
/// **AC-494's resolver** — a missing capability is `StepError::Unsupported { capability }` carrying
/// the token, decided against a claimed set the kernel is **handed**.
///
/// A **top-level** module, for the reason its header states in full: it holds no adapter, binds no
/// port and takes no type parameter, so there is no value in its scope from which an adapter
/// identity could be read. That is what makes it consistent with [`kernel_denylist`] rather than in
/// tension with it.
pub mod resolver;
/// **The review gate** (Step 7b-ii, `s7b-ii` §7b-ii.3–.8): the classifier, the policy table, re-execution,
/// ingest, the confirm half. Two functions, two files — `review::classify` and `review::policy::classify`
/// (AC-283) — and the lints that police them live in [`module_boundary`], outside both.
pub mod review;
pub mod rigor;
/// **The verify gate** (Step 7b-ii, `s7b-ii` §7b-ii.9; the register's `rk-kernel-verify`): the flip
/// predicate, the AC ledger, the stage-exit refusal, the bootstrap comparator, the gate-eligible list,
/// the admissibility policy. Reads no suite verdict, finding, quarantine or CI status — the lints that
/// hold that live in [`module_boundary`], outside it.
pub mod verify;

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
