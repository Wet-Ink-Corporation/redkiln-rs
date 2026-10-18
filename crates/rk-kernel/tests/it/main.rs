//! `rk-kernel`'s **one** integration binary — a `mod` per case, never one file per case
//! (ADR-0083:74). This is also what makes `--test it` addressable in Checkpoint 7b.
//!
//! What is here is **Slices 1, 2 and 3** — the ADR-0103 grammar, its differential against
//! S-GRAMMAR's probe, the closed `on_false` set, the three-valued evaluator, the two fail-closed
//! facet lints with their two fixture sets, rigor monotonicity, and the seven-step selection
//! function with its replay — plus **Slices 4a, 4b, 4c and 4d**: [`budget`], the three budget
//! counters and the exhaustion gate; [`envelope`], the pack-declared `N`/`K`/token appetite;
//! [`retry`], the policy over ADR-0112's eleven variants, the backoff envelope and the inner-attempt
//! accounting; [`breaker`], the K-consecutive-`Platform` rule, the per-family isolation and the two
//! source lints AC-034 and AC-035 name; [`degradation`], the substitution refusal, the `Degraded`
//! record in both sinks, and the five-row per-rigor gate table; and — **Slice 4e** — [`fsm`] and
//! [`divergence`], the FSM the runbook twice recorded as **NOT BUILDABLE** and AC-007's typed halt.
//!
//! **Extended 2026-09-20, Step 7d Slice 4 cycle 1** — and the earlier list is left standing rather
//! than rewritten, because it is accurate about the slices it names. Step 7d edits this closed
//! crate three times, each edit re-opening Checkpoints 7b and 7b-ii, and each landing its tests
//! here: Slice 2's in [`config`], Slice 3's in [`keywords`], [`question`] and [`resume`], and
//! **Slice 4's in [`catalog_diagnostic`]** — the loader's diagnostic mode (RT-54) and
//! `Catalog::from_sources`' AC-409 refusal of a `Gate` step carrying `outputs`, the two edits
//! `s7d` §7d.1 enumerates as (4) and (7). `doctor` itself is `rk-cli`'s and is asserted there; what
//! is here is only the loader, which is the half that lives in this crate.
//!
//! **Corrected 2026-09-14, four times.** The sentence that stood here said *"The selection engine
//! (Slice 3) and the runtime (Slice 4) have no modules here yet"*; it was already false of Slice 3
//! when it was written and is now false of nine of Slice 4's modules. The rule it carried still
//! holds and is restated: **no empty module stands in for an unbuilt one** — a `mod` that exists and
//! asserts nothing reads as coverage — so there is no `edges` module here, because the five `ac4xx`
//! hand-forwards are not built. (`breaker` and `degradation` left that list with Slice 4d, `fsm`
//! with Slice 4e, `memo` with Slice 4g, and [`runtime`], [`ledger`] and [`advisory`] with Slice 4h;
//! the sentence is corrected rather than extended.) **The clause about the five is now false of
//! one**: AC-487's is built, in [`kernel_denylist`], and **four** remain — AC-484, AC-494, AC-495
//! and AC-498. **Corrected again 2026-09-14: two of those four are now built** — AC-494's in
//! [`resolver`] and AC-495's in [`pack_validation`] — and **two** remain, AC-484 and AC-498. It is
//! still true that no module here stands in for either of them. **Corrected a third time
//! 2026-09-14: AC-484's is now built**, in [`adapter_edge`], and **one** remains — AC-498.
//! **Corrected a fourth and last time 2026-09-14: AC-498's is now built**, in [`no_egress`], and
//! **none** of the five remains. The rule the sentence carries outlives the list it was written
//! about and is restated rather than retired: no empty module stands in for an unbuilt one, and
//! the clause that says so is now about what the five modules **do** assert rather than about
//! which of them is missing.
//!
//! **[`adapter_edge`], 2026-09-14: AC-484, the last of the five `ac4xx` hand-forwards but one.**
//! The mechanism is **D-1**: `tests/fixtures/rk-adapter-edge-fixture/` (renamed from `rk-adapter-sqlite`
//! at Step 7c, 2026-09-17) is a zero-dependency workspace
//! member on the `rk-banned-dep`/`rk-clock-read` precedent, and `rk-kernel` carries an **optional**
//! edge to it behind the non-default `adapter-edge-reintroduce` feature — so the *"fixture commit …
//! that must fail"* the register asks for is committed and permanent rather than made and reverted,
//! and the must-fail half is reached by a feature flip inside the one test.
//!
//! 🛑 **The whole test is one distinction: `resolve`, never `packages`.** `cargo metadata` publishes
//! two readings of the dependency relation, and an **optional** edge appears in the manifest view
//! (`packages[].dependencies[]`) whether or not its feature is on — so an assertion written there
//! would be red on a green tree. The criterion's question is answered only by the feature-resolved
//! `resolve.nodes[]`. The test takes **both** readings off the same bytes and asserts they
//! **disagree**, so the trap is a committed assertion rather than a warning. A second trap sits
//! under it: `resolve.nodes[].deps[].name` is the **lib** name (`rk_adapter_sqlite`), and AC-484's
//! THEN is *"naming the adapter crate"*, so every `pkg` id is resolved back through `packages[]`
//! and the hyphenated package name is what the violation renders.
//!
//! 🛑 **One of AC-484's three named instruments is ARMED and an erratum is OWED against it.** Its
//! third is *"the same `deny.toml` `[bans].deny` list AC-005 names"*; measured 2026-09-14, that list
//! names five workflow engines and **no adapter**, and it cannot become AC-484's — `cargo deny`
//! bans are **workspace-wide** while AC-484 is a **per-crate edge** property, so banning
//! `rk-adapter-sqlite` workspace-wide would forbid it in `rk-cli`, which legitimately depends on
//! adapters at 7d. The register is not edited here; [`adapter_edge`]'s header carries the
//! measurement and the reasoning, and nothing in the file claims the clause is covered.
//!
//! **[`no_egress`], 2026-09-14: AC-498, the last of the five `ac4xx` hand-forwards.** The mechanism
//! is **D-2**, and it is three instruments and a fourth that is not in this binary at all. In
//! tree: **one caller-owned `AtomicU32`**, bound as the only `Harness` a sweep of both suites
//! AC-498 names can reach and read at **0** on the test's own thread — with a control on a second
//! fake that reads **1**, because a counter nothing can move reports zero forever; **no
//! `rk-adapter-*` in the resolved graph**, which is [`adapter_edge`]'s own walk re-used rather than
//! a second walker; and **no networking crate** in it, against the seven tokens D-2 measured at
//! zero, with the rule shown to fire over a synthetic graph reaching `tokio-util` transitively
//! while leaving `hyperlocal` alone.
//!
//! 🛑 **The larger half of AC-498 is NOT in this binary and nothing here reads as covering it.** A
//! resolved-graph assertion answers *"is a networking crate linked?"*, never *"can this process
//! open a socket?"* — `std::net::TcpStream` needs no dependency and reaches no `Cargo.lock` — and
//! **no lint closes that**: measured 2026-09-14, `clippy.toml` and `deny.toml` carry **no network
//! entry at all**, so **AC-003's network half is enforced by nothing and stays ARMED**. The one
//! instrument that reaches the class is the **`no-egress` CI job**'s denied container, and this
//! file asserts only that the job is **present** — never that it has run. Two further bounds are in
//! the module's table: the token list is a **denylist** that cannot see `native-tls` or `socket2`,
//! and the walk is over `kind: null` edges, so a networking crate arriving through the **dev**
//! closure is outside it (the direct dev rows are closed by name; the closure is not).
//!
//! **Slice 4h, 2026-09-14: [`runtime`], [`ledger`] and [`advisory`].** This is where `rk-ports` is
//! CALLED for the first time in the workspace. [`runtime`] holds AC-009's verbatim journaling and
//! its zero-dispatch re-read, AC-010's three refusals with a **dispatch count of 0** behind each,
//! AC-011's strictly increasing attempt and its whole-log triple, AC-012's two-thread late-duplicate
//! discard, AC-013's re-entry, AC-030's `retry_at` mark and AC-031's two dispatches — plus the
//! prose-extraction grep-lint AC-031's sub-clause names and its committed must-fail fixture.
//! [`ledger`] holds ADR-0114 decision 2. [`advisory`] holds AC-525's five instruments.
//!
//! 🛑 **Five halves in those three files are ARMED and none is claimed.** AC-013 is **PROVISIONAL**
//! *and* its process boundary is a dropped `Runtime` — ADR-0087 clause 2 says *"a reload that reads
//! state back in-process is not a valid implementation of this decision and is not evidence for
//! it"*, which is a **second** reason §7b.15's ARMED row does not carry; as of 2026-09-14 it is
//! also **ROUTED TO 7d** (D-8), because AC-013's WHEN names `redkiln run <item>`. AC-030's
//! *"exits 0"* is likewise 7d's, on its own *Verified by*'s words. *(This sentence read **"AC-024's
//! and AC-030's *\"exits 0\"* is a **process** claim a `cargo test` binary cannot make"** until
//! 2026-09-14. **AC-024's half left it**: a `cargo test` binary cannot assert its OWN exit, which is
//! all that clause supports, and [`cross_process`] asserts a **CHILD's**. The count of five is kept
//! because AC-030's half is still one of them and only its REASON moved.)* AC-030's *"the next run
//! resumes from the mark"* rides on AC-013. AC-031 is **compound** by the register's own admission.
//! And `ac110_executed_ledger_first_and_last` is named for a criterion whose own THEN is a
//! **load-time configuration error** and says nothing about a ledger: the test measures **ADR-0114
//! decision 2**, and AC-110's ledger coverage is **POWERLESS**. Each file's header carries its
//! table.
//!
//! **Slice 4g, 2026-09-14: [`memo`].** AC-014's nine cases over the assembly of
//! `rk_store::memo::StepKeyComponents` — nine, because the register says *"eight cases, one per
//! component"* plus *"a companion test asserting a hit"* — AC-015's edited atom, AC-016's three
//! instruments with the discriminating one first, AC-017's two iterations and their control,
//! AC-018's three fixtures and the shipped catalog's **nineteen** live violations *(dated
//! 2026-09-25, s8b, D21-43 N26: the shipped catalog is repaired and asserts zero; the nineteen live
//! on only in the frozen p9 control `ac018_frozen_p9_catalog_is_red`)*, and AC-019's two
//! halves. Its header carries the same WHAT-IS-MEASURED table [`breaker`] and [`fsm`] do, and four
//! of its rows are ARMED: memo-key component 5's **tool branch** has no source in this tree,
//! AC-019's *"event-schema test"* is over JSON keys because `Event.payload` is untyped and the
//! record is OWED, and every *"re-execution"* in the file is measured against a **projection**
//! of a decision sequence rather than against a dispatched run, because nothing dispatches
//! before Slice 4h. **Component 8's source was the fourth and is no longer ARMED**: the operator
//! decision of 2026-09-14 sources it from `ActivitySpec.schema`'s whole id, and
//! `component_eight_is_the_whole_schema_id_and_never_a_parsed_version` pins it — named without
//! an AC id, because a dated note saying what `schema_version` formally is remains **OWED**.
//!
//! 🛑 **[`breaker`], [`degradation`] and [`fsm`] each carry a WHAT-IS-MEASURED table, and it is not
//! decoration.** **Two** halves inside the first two are ARMED and not discharged — the breaker's
//! **durable mark** and the resolved card's **model tier** — and every case that needs those values
//! supplies them itself. A green there is coverage of the mechanism and of nothing above it; the
//! tables say so by id.
//!
//! **[`cross_process`], 2026-09-14: A REAL PROCESS BOUNDARY — AC-035 and AC-024.** Operator
//! decision **D-8**, on ADR-0087 clause 2: *"Resume must cross a real process boundary. A reload
//! that reads state back in-process is not a valid implementation of this decision and **is not
//! evidence for it**."* The mechanism is a re-exec harness — `std::env::current_exe()`, an env
//! marker, an `#[ignore]`d child entrypoint per criterion, and a parent that waits on the child and
//! then asserts **what the child left on disk**. The precedents for spawning are
//! `crates/rk-store/tests/it/contention.rs`:77-85 (AC-181's two-process test, which re-execs
//! `current_exe()` exactly this way) and [`adapter_edge`]'s two `cargo` children; AC-003's process
//! ban scopes to the **workflow module**, not to tests, and `clippy.toml`'s `Command` entry is
//! Checkpoint 6's **git** ban.
//!
//! The two criteria are here because **neither's *Verified by* names a binary**: AC-035's is *"a
//! kill-and-re-enter test asserting the refusal survives the restart"* and AC-024's is *"a test
//! asserting process exit and a written mark"*. **AC-013 and AC-030 are ROUTED TO 7d** by the same
//! decision and the harness is deliberately not pointed at them — AC-013's WHEN is *"WHEN `redkiln
//! run <item>` re-enters"* and AC-030's *Verified by* is *"the next **`redkiln run`** resumes from
//! the mark (AC-013's re-entry test)"*. Both are reported ARMED with that reason, in [`runtime`]'s
//! table and in `src/runtime.rs`'s, and AC-013 stays **PROVISIONAL on P8** either way.
//!
//! 🛑 **Three things there are NOT claimed, and each is in the file's own table.** The process that
//! exits 0 is a **libtest binary**, not `redkiln run` — the CLI half stays 7d's. The durable medium
//! is a **test-owned file-backed `EventStore`**, because measured 2026-09-14 the only
//! `impl EventStore` in `crates/*/src` is `rk_ports::fakes::VecStore`, an in-memory `Vec`; **no
//! durable store exists in this tree at 7b** and the SQLite adapter is 7c's. And the breaker
//! expiry's **value** is still sized by no corpus figure. A fourth is recorded as a finding and not
//! repaired: `rk_ports::event_store::Snapshot`'s only field is `pub(crate)`, so **no `EventStore`
//! implementation outside `rk-ports` can construct one** — the fixture refuses `snapshot`/`rebuild`
//! rather than faking them, and `rk-ports` is not edited, because Checkpoint 7a has passed.
//!
//! *(This said **four halves** until 2026-09-14 and listed the breaker's **model-family key** and
//! its **expiry** as the other two. Operator decision **D-7** built both — the key is the pack's
//! `[role.<name>] family` resolved from the step's `role`, the expiry is pack-declared at
//! `[breaker] expiry_secs` — and
//! `breaker_key_is_the_packs_role_family_never_the_activity_family` pins them, named without an
//! AC id because the five pack key **spellings** remain a reading with a record **OWED**. The count
//! moved 4 → 2 rather than the sentence being deleted, so a reader can see which two left. The
//! breaker's two AC-033/AC-034/AC-035 cases still supply arbitrary tokens on purpose: AC-034's
//! property is over two *arbitrary* distinct keys, and [`breaker`]'s header says so.)*
//!
//! **[`fsm`] is the sharpest instance of that rule in this binary**, and its own name is the reason
//! it needs saying: `ac008_terminal_state_and_the_next_action_grains` is named for a criterion whose
//! comparison has **two sides**, and Slice 4e built only the side that is recomputed. **Slice 4f,
//! 2026-09-14, built the other**: the recorded side is now `run.json`'s `terminal_state` and
//! `next_action`, written at record time, committed, and read back by [`replay`] — so AC-006
//! assertion 3 and AC-008's terminal-state half are measured against a committed byte string
//! rather than against a second fold of the same journal. **Slice 4i, 2026-09-14, moved the last
//! of it**: this sentence read *"**What did not move**: AC-008's GIVEN names \"a run that
//! dispatched N activities\" and **N is 0**"*, and N is now **2** — the recorded journal carries
//! two `dispatch` and two `activity_committed` rows, and the replay serves both from the journal
//! with the `RefusingHarness` bound. **What still did not move**: AC-161's **route** *"from git
//! alone"* is 7d's; and AC-164's
//! `doctor`-classification half is in neither §7b.15's claimed list nor its ARMED table (residual
//! 10l). None of the three is reported as covered, and [`replay`]'s header carries each reason.
//! AC-039 is a **HUMAN GATE** §7b.15 excludes from the claim outright, so
//! `ac039_gate_record_and_the_void_answer` asserts the mechanism and never the answer. Four values
//! the FSM needs are **caller-supplied arguments** on [`breaker`]'s precedent — the gate's
//! `tree_hash` and `deadline`, the successor stage (this system models no stage ordering), and the
//! `@version` half of `activity_id@version`, which has no carrier in this tree at all.
//!
//! [`divergence`] holds AC-007 alone and is separate from [`fsm`] on design §4's own filing: the
//! divergence is the **replay's** subject, and the two tests fail for different reasons.
//!
//! 🛑 **[`recorded_log`], 2026-09-14: the committed golden log's UNFEATURED controls, and the
//! reason it is a module of its own.** [`replay`] is `#![cfg(feature = "corpus")]` **whole**, so
//! every control in it is invisible to Checkpoint 7b's third loop and to :1345-1346's unfeatured
//! `--list`. Two guards did not need that gating and had no unfeatured control at all, and an
//! adversarial pass reproduced both: `src/replay.rs`'s divergence comparison (patched to
//! `if false && …`: **83 passed, 0 failed**) and `read_run`'s refusal of a log missing
//! `inputs/specs.json` (removed: the whole replay suite **green**). `read_run` and the extracted
//! `compare_recorded_residue` bind no port, spawn no `git` and read no corpus, so their controls
//! belong in the run that is actually executed. **Neither name here contains `golden_log_replay`
//! or `ac161_ac164_delete_db_replay`**, so the zero-selection line is unaffected.
//!
//! *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's checkpoint minor.)*
//! ~~[`replay`] is `#![cfg(feature = "corpus")]` **whole**~~ — package S8E-P3 replaced that
//! module-wide gate with one `#[cfg(feature = "corpus")]` on each item, and ONE item is ungated:
//! `replay::golden_log_canonical_digests_are_unchanged_by_the_interim_item_key`, which runs in
//! the unfeatured suite (Checkpoint 8e (2)). Its name contains neither `golden_log_replay` nor
//! `ac161_ac164_delete_db_replay`, so Checkpoint 7b's zero-selection line still selects nothing
//! unfeatured; every other control in [`replay`] is still invisible to it, which is the reason
//! above for this module.
//!
//! That rule is also why `ac029_n_is_declared_never_defaulted` is in [`envelope`] and **stays
//! there** now that [`retry`] exists. Design §4 files it beside `outer_retry_ac029`; it asserts the
//! pack-validation half — that `N` is declared and never defaulted — which is `src/workflow/
//! envelope.rs`'s subject and not the backoff policy's, and moving it would separate the test from
//! the module it reads. The checkpoint grep is over the test NAME and not over its file, so the
//! placement costs no assertion either way. [`envelope`]'s own header states it.
//!
//! **[`facet_registry`], 2026-09-14: AC-072, and the sentence this paragraph used to carry.** It
//! read *"**`facet_registry_entry_shape` is absent and that is deliberate.** AC-072 is a **third**
//! rule with its own eleven-case fixture set and the register forbids collapsing it into either
//! facet lint; it is not built, so there is no module for it and no test claiming it."* The rule is
//! built, the eleven fixtures are committed under `tests/fixtures/registries/entry-shape/`, and the
//! module and the test both exist — so the paragraph is corrected rather than deleted, on the rule
//! the header above states: **no empty module stands in for an unbuilt one**, and its converse is
//! that a module arriving is the absence-note's retirement.
//!
//! **[`quality_block`], 2026-09-14: AC-527, and the absence note in [`evaluator`] it retires.**
//! `evaluator.rs`:236 carried *"the lint that acts on it **IS NOT BUILT** —
//! `ac527_empty_set_terminal_condition` selects zero tests on this crate"*; the filter now selects
//! one. *(Rewritten 2026-09-25, s8b refuting round 1: the empty-set rule is retired.)* AC-527 was
//! amended in the register on 2026-09-25 to ADR-0080 decision 2 (D21-41 (h); D21-55; `s8b` §8b.4),
//! so no erratum is owed. The rule refuses two things on every `check` step: the named form
//! `findings.none(evidence in [...])`, on the parse tree, alone or as a required conjunct; and a
//! required conjunct that a one-finding, `evidence`-only witness of one blocking class falsifies,
//! each conjunct evaluated alone. Nothing is refused for its truth over an empty set. The fixtures
//! are one per branch under `tests/fixtures/activities/` (seven, plus the fail-closed
//! `does_not_parse`), and the recast fixture 2, `findings.all(not (evidence in [...]))`, is refused
//! by the witness **alone**. 🛑 **Not covered here**: the `rk-catalog` CLI surface, which stays
//! ARMED.
//!
//! Its **three positives** are the discrimination and not decoration (§7b.13 says so by name), and
//! its **fourth** is `spikes/p9/facets.toml` itself: two of the eleven cases are predicates that
//! must fail *"against the current registry"*, so that file is an input and a rule that rejected it
//! would be caught there. 🛑 **Three clauses of AC-072 are recorded and not built**, each with its
//! measurement in `src/facet_registry.rs`'s header: the **`anchor`** clause has no on-disk form in
//! any record; the domain-shape set is **four on disk and three in the register**; and the general
//! *"may abstain"* case is decidable from no entry. **Three register amendments are OWED**, and the
//! CLI surface is Step 7d's over Step 7e's file — none of the four is reported as covered.
//!
//! **[`kernel_denylist`], 2026-09-14: AC-487, the first of the five `ac4xx` hand-forwards to land.**
//! The lint is a **source rule** over `crates/rk-kernel/src/` and not a `clippy` entry (`clippy.toml`
//! has no module scoping and `downcast_ref` has no nameable path) and not a `trybuild` case (an
//! identity check *compiles*). Its positive control is `tests/fixtures/lint-controls/`'s, the
//! directory `s7b-ii` §7b-ii.10 opens for exactly that, and the compile-fail fixture the register
//! names **beside** the lint is `tests/ui/ac487_port_identity_is_not_askable.rs`, which pins the
//! half a source rule cannot: no port trait carries an `Any` supertrait, so the kernel cannot ask a
//! port object what it is. 🛑 **Two things here are not covered.** AC-487's *"and the difference is
//! expressed as a capability lookup"* is **STRUCK by the criterion's own note** as unverifiable by
//! this instrument and is carried by AC-494 and AC-495, **both of which are now built** — in
//! [`resolver`] and [`pack_validation`], and deliberately not in this file, because an absence rule
//! and a presence rule sharing one test file is how the two get read as one; and the
//! *"no adapter-discriminant enum"* clause is reached **by prefix only** — a substring rule sees
//! `enum AdapterKind` and not `enum Backend`, which is **ARMED, NOT DISCHARGED**.

//! **2026-09-14: [`resolver`] and [`pack_validation`] — AC-494 and AC-495, the third and fourth of
//! the five `ac4xx` hand-forwards.** [`resolver`] holds the register's **three** named instruments
//! and not one: the unit test over a fake adapter whose registry omits the capability (with the
//! adapter's own dispatch counter reading **0**, which is *"never a `pass`"*), the exhaustiveness
//! test whose mechanism is `E0004` on a twelfth `StepError` variant, and the negative test over the
//! **shipped** retry policy at every attempt from 1 to `N + 2`. [`pack_validation`] holds the two
//! fixture packs — one unroutable, one routed, differing by one `[[stage.verify.require]]` row and
//! an `id` — and asserts the **span location** on the failing one as a literal *and* by reading the
//! fixture's own text back at that line.
//!
//! 🛑 **Three halves there are ARMED and none is claimed**: where an activity's capability need comes
//! from (no `ActivitySpec` field, no catalog key — a record is **OWED**), AC-495's *"before any
//! activity runs"* as a live ordering (`rk-cli`, Step 7d), and **AC-500**, which AC-495's own *Note*
//! trims out of this criterion. Each file's header carries its table.

mod adapter_edge;
mod advisory;
mod baseline_range;
mod breaker;
mod budget;
mod catalog_diagnostic;
mod check_cannot_dispatch;
mod check_eval;
mod config;
mod conformance;
mod cross_process;
mod dag;
mod degradation;
mod deviation;
mod dispatch_completed;
mod divergence;
mod emit;
mod emit_binding;
mod envelope;
mod evaluator;
mod execute;
mod facet_provenance;
mod facet_registry;
mod facet_stability;
mod feed;
mod forward_ref_lint;
mod fsm;
mod gate_request;
mod gate_step;
mod gates;
mod grammar;
mod guard_context;
mod hold;
mod human_root;
mod kernel_denylist;
mod keywords;
mod lane_order;
mod lane_walk;
mod lattice;
mod ledger;
mod memo;
mod module_boundary;
mod no_egress;
mod on_false;
mod on_false_exec;
mod p9_pin;
mod pack_validation;
mod post_commit_failure;
mod probe_differential;
mod producer;
mod projection;
mod quality_block;
mod question;
mod recorded_log;
mod reference_resolver;
mod replay;
mod resolver;
mod resume;
mod retry;
mod review;
mod review_confirm;
mod rigor;
mod root_input;
mod runtime;
mod second_agent;
mod selection;
mod short_circuit;
mod source_registry;
mod step_bound;
mod step_error_row;
mod step_grain;
mod step_landing;
mod support;
mod test_selection;
mod tier_one;
mod tool_verb_routing;
mod ui;
mod verb_registry;
mod verify;
mod walk_semantics;
