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
//! **Corrected 2026-09-14, four times.** The sentence that stood here said *"The selection engine
//! (Slice 3) and the runtime (Slice 4) have no modules here yet"*; it was already false of Slice 3
//! when it was written and is now false of nine of Slice 4's modules. The rule it carried still
//! holds and is restated: **no empty module stands in for an unbuilt one** — a `mod` that exists and
//! asserts nothing reads as coverage — so there is no `edges` module here, because the five `ac4xx`
//! hand-forwards are not built. (`breaker` and `degradation` left that list with Slice 4d, `fsm`
//! with Slice 4e, `memo` with Slice 4g, and [`runtime`], [`ledger`] and [`advisory`] with Slice 4h;
//! the sentence is corrected rather than extended.)
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
//! it"*, which is a **second** reason §7b.15's ARMED row does not carry. AC-024's and AC-030's
//! *"exits 0"* is a **process** claim a `cargo test` binary cannot make. AC-030's *"the next run
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
//! AC-018's three fixtures and the shipped catalog's **nineteen** live violations, and AC-019's two
//! halves. Its header carries the same WHAT-IS-MEASURED table [`breaker`] and [`fsm`] do, and four
//! of its rows are ARMED: memo-key component 5's **tool branch** and component 8's
//! **`schema_version`** have no source in this tree, AC-019's *"event-schema test"* is over JSON
//! keys because `Event.payload` is untyped and the record is OWED, and every *"re-execution"* in
//! the file is measured against a **projection** of a decision sequence rather than against a
//! dispatched run, because nothing dispatches before Slice 4h.
//!
//! 🛑 **[`breaker`], [`degradation`] and [`fsm`] each carry a WHAT-IS-MEASURED table, and it is not
//! decoration.** Four halves inside the first two are ARMED and not discharged — the breaker's
//! model-family key, its durable mark, its expiry, and the resolved card's model tier — and every
//! case in both files supplies those values itself. A green there is coverage of the mechanism and
//! of nothing above it; the tables say so by id.
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
//! That rule is also why `ac029_n_is_declared_never_defaulted` is in [`envelope`] and **stays
//! there** now that [`retry`] exists. Design §4 files it beside `outer_retry_ac029`; it asserts the
//! pack-validation half — that `N` is declared and never defaulted — which is `src/workflow/
//! envelope.rs`'s subject and not the backoff policy's, and moving it would separate the test from
//! the module it reads. The checkpoint grep is over the test NAME and not over its file, so the
//! placement costs no assertion either way. [`envelope`]'s own header states it.
//!
//! **`facet_registry_entry_shape` is absent and that is deliberate.** AC-072 is a **third** rule
//! with its own eleven-case fixture set and the register forbids collapsing it into either facet
//! lint; it is not built, so there is no module for it and no test claiming it.

mod advisory;
mod breaker;
mod budget;
mod dag;
mod degradation;
mod deviation;
mod divergence;
mod envelope;
mod evaluator;
mod execute;
mod facet_provenance;
mod facet_stability;
mod fsm;
mod grammar;
mod ledger;
mod memo;
mod on_false;
mod probe_differential;
mod producer;
mod replay;
mod retry;
mod rigor;
mod runtime;
mod selection;
mod short_circuit;
mod support;
mod ui;
