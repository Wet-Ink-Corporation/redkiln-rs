//! The budget counters and the exhaustion gate — **AC-020, AC-021, AC-022, AC-024, AC-025** and the
//! half of **AC-023** that is common ground between it and AC-389.
//!
//! Four of the five names here are §7b.13's Slice-4 names and are asserted verbatim by §7b.14's
//! **third name loop** — the one whose `for t in` list opens `budget_three_counters_never_share` —
//! which runs **without** `--features corpus`, so nothing in this file is feature-gated, and no name
//! here contains `golden_log_replay` or `ac161_ac164_delete_db_replay` as a substring (the
//! `cargo test -p rk-kernel -- --list` differential in the same block counts bare substrings and
//! requires the unfeatured count to be zero). **Both were line cites — `:1520-1532` and
//! `:1345-1346` — and both were stale: `s7b-kernel.md` moves under this file on every editing pass
//! [re-derived 2026-09-15: `grep -n "for t in" runbook/phase-1/s7b-kernel.md`].**
//!
//! # The one name here that is in no checkpoint loop, and why it is not `ac023_…`
//!
//! `appetite_raise_from_an_activity_result_is_refused` is **not** in any checkpoint loop. It exists
//! because §7b.13 pairs AC-023 with `for_each_charges_once_independent_of_n`, and an N-cardinality
//! charge test asserts nothing whatever about an appetite raise or an author field — so AC-023 would
//! otherwise be reached by nothing but the unfiltered `cargo test -p rk-kernel` floor, which
//! `00-preamble.md`'s **golden constraint 12** says *"is satisfied by an implementation nothing
//! pins"*, in its *"A checkpoint grep is not this"* paragraph. **Cited by constraint number and no
//! longer by line, 2026-09-15**: the `:186-189` that stood here lands inside constraint **11**'s F40
//! scope finding [re-derived 2026-09-15: `grep -n 'nothing pins' runbook/00-preamble.md`]. It
//! **was** deliberately not named `ac023_…`: **contradiction C3 was open**, only the refusal both
//! criteria shared was built, and naming the test after the id would have claimed the escape hatch
//! AC-023 admitted and AC-389 denied. See `src/workflow/budget.rs`'s header.
//!
//! # 🛑 THAT TABOO IS LIFTED, AND THE RENAME IS DELIBERATELY NOT DONE HERE
//!
//! **C3 is RESOLVED in favour of AC-389** (`DECISIONS-2026-09-14 D-13`): an in-run raise is refused
//! **outright**, AC-023's *"unless it arrives as a typed budget-change event with an author"* clause
//! is **CUT** by dated register amendment, and that amendment withdraws AC-023's second instrument —
//! *"an event-schema test that every appetite change carries an author field"* — **with the clause**
//! rather than leaving it ARMED against no stage. **AC-023's only surviving instrument is therefore
//! *"a unit test asserting refusal of an in-run raise from an activity result"*, which is exactly
//! the test below**, so there is no hatch left for the id to claim.
//!
//! It is **not** renamed here. Checkpoint 7b asserts **literal** test names (`s7b-kernel.md`
//! §7b.14), so a rename is a separate, checked act against the checkpoint rather than a consequence
//! of this correction. Recorded, not done.
//!
//! # 🛑 AND AC-389's OTHER HALF IS STILL UNBUILT AND STILL OWED A RECORD
//!
//! *"The attempt is appended as an event"* is **not built and not buildable here**: there is no
//! `EventKind` for a refused appetite raise, the enum is closed at 29, ADR-0112's eleven `StepError`
//! variants carry no member for one, and design §5's table has no row. It is residual **R-1** in
//! `spikes/DECISIONS-2026-09-15.md`, and C3's resolution does not discharge it.
//!
//! # 🛑 WHAT IS MEASURED HERE AND WHAT IS NOT
//!
//! | criterion | measured here | not here |
//! | --- | --- | --- |
//! | **AC-020** | the runtime half — three pair-cases over [`fold`] | the separation half is a compile failure and lives at `tests/ui/ac020_counters_never_share.rs` |
//! | **AC-021** | the **declared** field, ADR-0037's lint rule 8 (ADR-0122 decision 3) | the runtime draw-down is AC-026's and is Slice 4c's |
//! | **AC-022** | the **runtime** charge at N = 1, 2, 50 | — |
//! | **AC-023** | the refusal — and as of D-13 that is AC-023's **only** surviving instrument | *"an event-schema test that every appetite change carries an author field"* — **WITHDRAWN WITH THE CLAUSE**, not ARMED. This cell read *"ARMED, NOT DISCHARGED"* until 2026-09-15: C3 is **RESOLVED** in favour of AC-389 (DECISIONS-2026-09-14 D-13), AC-023's *"unless"* clause is **CUT**, and D-13 withdraws its second instrument **with** the clause. What is still unbuilt is **AC-389's** *"the attempt is appended as an event"* — no `EventKind` can carry a budget change and minting one takes a record — residual **R-1** in `spikes/DECISIONS-2026-09-15.md` |
//! | **AC-024** | the durable mark's **value** and its carrier; the *"no thread sleeps"* half, over the module's own source | the mark's **write on a live path** and *"no further dispatch"*. *(**"exits 0" left this cell on 2026-09-14.** It read *"*\"exits 0\"* (a **process** exit — 7d's)"*, and operator decision **D-8** built the register's own instrument for it — `tests/it/cross_process.rs`'s `ac024_wall_clock_mark_outlives_the_process_that_wrote_it` re-execs this binary and reads a **child's** exit code beside the mark the child left. What is still 7d's is narrower and is stated there: that the process exiting 0 is `redkiln run` rather than a libtest binary.)* AC-024 is a **twin**, owner AC-240 + AC-278 |
//! | **AC-025** | the unit test per budget (three, swept over `Counter::ALL`), the nine-row negative table, and the must-close positive | — the compile-fail companion is `tests/ui/ac025_exhaustion_is_not_a_pass.rs`, taken **in addition to** the register's named instrument |
//!
//! **Corrected 2026-09-14 (Slice 4b).** The row that stood in the table above read *"AC-024, AC-025
//! | nothing | `exhaustion_ac024_ac025` is Slice 4b's and no test here may be read as covering it"*.
//! It is Slice 4b's, and this is Slice 4b.

use rk_kernel::budget::{
    AppetiteRaiseRefused, Attempts, BudgetEnvelope, BudgetExhausted, BudgetExtensionMark,
    BudgetLedger, Charge, Counter, FanOut, GateEvidence, Tokens, WallClockSecs,
    appetite_raise_from_activity_result, charge_pass, evaluate_gate, exhausted_counter, fold,
    required_attempts_total, wall_clock_expiry_mark,
};
use rk_kernel::catalog::{Activity, Catalog, Evidence, StepKind};
use rk_kernel::select::Pack;
use rk_types::event_kind::EventKind;

/// AC-021's two-pass graph. **Compiled in, never opened** — a fixture loaded from a path is a
/// `Result` an implementer can `unwrap_or_default()` into nothing, and a rule over nothing passes
/// (`tests/it/support.rs`:6-12).
const FIXTURE_TWO_PASS: &str = include_str!("../fixtures/catalog/two-pass.toml");
/// AC-022's fan-out graph, same rule.
const FIXTURE_FANOUT: &str = include_str!("../fixtures/catalog/fanout.toml");

/// Load one fixture activity.
///
/// # Panics
///
/// With the loader's own error, so a fixture that stops parsing names its reason.
fn fixture(name: &str, src: &str) -> Activity {
    let catalog = Catalog::from_sources(&[(name, src)])
        .unwrap_or_else(|e| panic!("the {name} fixture loads: {e}"));
    catalog
        .activities()
        .first()
        .cloned()
        .expect("the fixture holds one activity")
}

/// The activity's declared envelope. **Declared, never defaulted** — §7b.10's 🛑 and
/// `select/pack.rs`:249-256.
///
/// # Panics
///
/// If the fixture declares no `budget` table, because a defaulted envelope here would be exactly the
/// code default the record forbids, arriving through a test helper instead of through the library.
const fn declared_envelope(activity: &Activity) -> BudgetEnvelope {
    activity
        .budget
        .expect("the fixture declares budget = { attempts_total, tokens, wall_clock_secs }")
}

/// The formula ADR-0037 **replaced**: `Σ (step retry.max × 1)` over the dispatchable steps.
///
/// Computed here, in the test, so `attempts_total_counts_a_pass_not_retries` asserts against the
/// losing formula's actual value rather than against a number a reader has to trust.
fn summed_retry_allowances(activity: &Activity) -> u32 {
    activity
        .steps
        .iter()
        .filter(|s| matches!(s.kind, StepKind::Agent | StepKind::Tool))
        .map(|s| s.retry_max.unwrap_or(0))
        .sum()
}

// ---- AC-020 — three counters, three pair-cases, and a report that proves each charge landed ------

/// **AC-020.** *"GIVEN the three budgets — attempts per activity, tokens per item, wall clock per
/// run / WHEN one of them is consumed or exhausted / THEN neither of the other two changes."*
///
/// Three pair-cases, one per counter. Each folds a **baseline** that has already moved all three,
/// then folds the same sequence plus one further charge on the counter under test, and asserts (a)
/// that counter moved, (b) the other two are `==` their baseline value.
///
/// The premise — (a) — is what stops a vacuous pass: without it a [`fold`] that dropped every charge
/// on the floor would satisfy "the other two are unchanged" three times over. The `BudgetReport`'s
/// per-counter charge counts are asserted for the same reason one level down: a ledger reading zero
/// because nothing was charged is otherwise indistinguishable from one reading zero because the
/// charge was lost.
///
/// Each case charges past the declared limit as well, so *"or exhausted"* is exercised and not only
/// *"consumed"*. `BudgetReport::exhausted` is an **observation**; AC-024/AC-025's gate is Slice 4b's
/// and nothing here asserts a verdict.
#[test]
fn budget_three_counters_never_share() {
    let envelope = declared_envelope(&fixture("two-pass.toml", FIXTURE_TWO_PASS));

    // A baseline with all three already moved, so "unchanged" is a claim about a non-zero value.
    let baseline_charges = [
        Charge::Attempts(Attempts::new(1)),
        Charge::Tokens(Tokens::new(10)),
        Charge::WallClock(WallClockSecs::new(5)),
    ];
    let (base, base_report) = fold(&baseline_charges, &envelope);
    assert_eq!(base.attempts(), Attempts::new(1));
    assert_eq!(base.tokens(), Tokens::new(10));
    assert_eq!(base.wall_clock(), WallClockSecs::new(5));
    assert_eq!(
        (
            base_report.attempts_charges,
            base_report.token_charges,
            base_report.wall_clock_charges
        ),
        (1, 1, 1),
        "the baseline must actually have charged each counter once, or every pair below compares \
         two zeroes"
    );

    // ---- pair case 1: attempts consumed to exhaustion; tokens and wall clock unmoved -----------
    let mut charges = baseline_charges.to_vec();
    charges.push(Charge::Attempts(Attempts::new(
        envelope.attempts_total.get() + 1,
    )));
    let (ledger, report) = fold(&charges, &envelope);
    assert_ne!(
        ledger.attempts(),
        base.attempts(),
        "the charged counter must move, or 'the other two are unchanged' is vacuous"
    );
    assert_eq!(report.exhausted, Some(Counter::Attempts));
    assert_eq!(ledger.tokens(), base.tokens(), "tokens must not move");
    assert_eq!(
        ledger.wall_clock(),
        base.wall_clock(),
        "the wall clock must not move"
    );
    assert_eq!(
        (report.token_charges, report.wall_clock_charges),
        (base_report.token_charges, base_report.wall_clock_charges),
        "no further charge may land on the two counters this case does not touch"
    );

    // ---- pair case 2: tokens consumed to exhaustion; attempts and wall clock unmoved -----------
    let mut charges = baseline_charges.to_vec();
    charges.push(Charge::Tokens(Tokens::new(envelope.tokens.get() + 1)));
    let (ledger, report) = fold(&charges, &envelope);
    assert_ne!(ledger.tokens(), base.tokens(), "the charged counter moved");
    assert_eq!(report.exhausted, Some(Counter::Tokens));
    assert_eq!(ledger.attempts(), base.attempts(), "attempts must not move");
    assert_eq!(
        ledger.wall_clock(),
        base.wall_clock(),
        "the wall clock must not move"
    );
    assert_eq!(
        (report.attempts_charges, report.wall_clock_charges),
        (base_report.attempts_charges, base_report.wall_clock_charges),
        "no further charge may land on the two counters this case does not touch"
    );

    // ---- pair case 3: wall clock consumed to exhaustion; attempts and tokens unmoved -----------
    let mut charges = baseline_charges.to_vec();
    charges.push(Charge::WallClock(WallClockSecs::new(
        envelope.wall_clock_secs.get() + 1,
    )));
    let (ledger, report) = fold(&charges, &envelope);
    assert_ne!(
        ledger.wall_clock(),
        base.wall_clock(),
        "the charged counter moved"
    );
    assert_eq!(report.exhausted, Some(Counter::WallClock));
    assert_eq!(ledger.attempts(), base.attempts(), "attempts must not move");
    assert_eq!(ledger.tokens(), base.tokens(), "tokens must not move");
    assert_eq!(
        (report.attempts_charges, report.token_charges),
        (base_report.attempts_charges, base_report.token_charges),
        "no further charge may land on the two counters this case does not touch"
    );

    // THE CONTROL. Without it the three cases above are satisfied by a fold that exhausts every
    // counter on every input: `exhausted` would read `Some(_)` whatever was charged, and the
    // `assert_eq!` on which counter is exhausted would still pass in case 1.
    let (clean, clean_report) = fold(&baseline_charges, &envelope);
    assert_eq!(
        clean_report.exhausted, None,
        "a baseline well inside every limit must exhaust nothing: {clean:?}"
    );

    // AND THE SWEEP IS OVER THE CLOSED SET, so a fourth counter is a compile error here and not a
    // row this test forgot.
    assert_eq!(
        Counter::ALL.map(Counter::as_str),
        ["attempts", "tokens", "wall_clock"]
    );
}

// ---- AC-021 — the DECLARED field, which is lint rule 8's and not the runtime counter's -----------

/// **AC-021.** *"GIVEN an activity whose graph runs A agent/tool steps in one pass and then retries
/// the pass / WHEN `attempts_total` is evaluated / THEN it counts the agent/tool steps of a pass,
/// not the sum of retried steps across passes."*
///
/// **The quantity under test is the DECLARED budget field**, per **ADR-0122 decision 3**: *"AC-021's
/// quantity is the declared budget field … What AC-021 forbids is summing the same steps across
/// passes to inflate the declared minimum: a two-pass graph does not require `2A`."* The runtime
/// draw-down is AC-026's, it is a different number, and no assertion here touches it.
///
/// The fixture discriminates three formulas that give three different answers on it: rule 8's `A`
/// = 3, the formula ADR-0037 replaced (`Σ step retry.max`) = 6 = 2A, and "count every step" = 5.
///
/// The premise is AC-021's own — `assert_ne!(A, 2A)` — which fails for `A = 0`, so an
/// implementation returning zero cannot pass by making the two sides equal.
#[test]
fn attempts_total_counts_a_pass_not_retries() {
    let activity = fixture("two-pass.toml", FIXTURE_TWO_PASS);

    let a = required_attempts_total(&activity);

    // THE PREMISE, spelled as the register spells it. At A = 0 the two sides coincide and every
    // assertion below is satisfied by a function that counts nothing.
    assert_ne!(
        a.get(),
        2 * a.get(),
        "A must be non-zero or this test asserts nothing: A = {a}"
    );

    assert_eq!(
        a,
        Attempts::new(3),
        "rule 8 counts the agent/tool steps of ONE pass"
    );

    // ---- and it is NOT the losing formula ADR-0037 replaced -----------------------------------
    let summed = summed_retry_allowances(&activity);
    assert_eq!(
        summed,
        2 * a.get(),
        "the fixture must actually declare retries summing to 2A, or the next assertion compares \
         the right answer with itself"
    );
    assert_ne!(
        a.get(),
        summed,
        "`attempts_total` is not `Σ (step retry.max × 1)` — ADR-0037's Context calls that \
         \"a formula stated in terms of summed per-step retry allowances\" and its Decision \
         replaces it"
    );

    // ---- nor is it "every step with an id" ----------------------------------------------------
    assert_eq!(
        activity.steps.len(),
        5,
        "the fixture must carry steps that mint no dispatch, or the next assertion cannot \
         discriminate"
    );
    assert_ne!(
        a.get() as usize,
        activity.steps.len(),
        "`check`, `gate` and `emit` mint no dispatch (ADR-0036) and rule 8 counts \"agent/tool \
         steps\""
    );

    // ---- and the DECLARED field satisfies the rule the function states -------------------------
    let envelope = declared_envelope(&activity);
    assert!(
        envelope.attempts_total >= a,
        "ADR-0037: `budget.attempts_total >= number of agent/tool steps on one pass` — declared \
         {}, required {a}",
        envelope.attempts_total
    );

    // THE CONTROL: a graph with no dispatchable step requires nothing, so the function is not one
    // that returns 3 for every input. Without this the assertions above are satisfied by a
    // constant.
    let bare = fixture(
        "bare.toml",
        "[activity]\nid = \"bare\"\nstage = \"implement\"\nmethod_rank = 10\n\n\
         [[step]]\nid = \"only\"\nkind = \"emit\"\n",
    );
    assert_eq!(required_attempts_total(&bare), Attempts::new(0));
}

// ---- AC-022 — the RUNTIME charge, at the three cardinalities the register names ------------------

/// **AC-022.** *"GIVEN a `for_each` step projecting over N items / WHEN the attempts budget is
/// charged / THEN the step counts once, independent of N."*
///
/// *Verified by:* *"a parameterised unit test at N = 1, 2, 50 asserting a constant charge."* All
/// three values, because a single-N test does not discharge the criterion — it cannot tell a
/// constant charge from one that happens to agree at that N.
///
/// 🛑 **This is built against ADR-0053's `Negative`, deliberately.** That paragraph says *"a
/// `for_each` over many elements can still exhaust `budget.attempts_total` at runtime even though it
/// passed the static lint check"* — i.e. that the runtime charge scales. It is a `Consequences`
/// paragraph, not a `## Decision` clause; ADR-0053's Decision governs **lint rule 8** only, and
/// under ADR-0092 the register owns AC-022's THEN. **Do not "fix" this test to match the ADR**: a
/// charge that scales with N is precisely the red §7b.13 names for this name, and the criterion
/// would then be pinned by nothing.
#[test]
fn for_each_charges_once_independent_of_n() {
    let activity = fixture("fanout.toml", FIXTURE_FANOUT);

    // THE PREMISE, part 1: the fixture really carries a fan-out step.
    let fan_step = activity
        .steps
        .iter()
        .find(|s| s.for_each.is_some())
        .expect("the fixture declares a `for_each` step, or this test is about nothing");
    assert_eq!(
        fan_step.for_each.as_deref(),
        Some("artifact:GherkinScenarios.scenarios")
    );

    let mut charges = Vec::new();
    for n in [1_usize, 2, 50] {
        let fan_out = FanOut::new().with_observed(&fan_step.id, n);

        // THE PREMISE, part 2: the fan-out this pass is handed really did produce N items. Without
        // it the three cases could be three runs of the same empty observation.
        assert_eq!(
            fan_out.observed(&fan_step.id),
            n,
            "the fan-out must actually carry N = {n}"
        );

        let charge = charge_pass(&activity, &fan_out);
        assert_eq!(
            charge,
            Attempts::new(2),
            "one `for_each` agent step + one `tool` step = 2 charges at N = {n}; `emit` mints no \
             dispatch"
        );

        // A charge that scaled would read 1 + N here. At N = 1 that coincides with the right
        // answer and the assertion is vacuous, which is exactly why the register names three
        // values; it discriminates at 2 and at 50.
        if n > 1 {
            assert_ne!(
                charge.get() as usize,
                1 + n,
                "a charge that scaled with the fan-out cardinality would read 1 + N = {} at N = \
                 {n} — ADR-0053's `Negative` is contradicted by AC-022's THEN and AC-022 governs",
                1 + n
            );
        }
        charges.push(charge);
    }
    assert_eq!(
        charges,
        vec![Attempts::new(2); 3],
        "constant across N = 1, 2, 50"
    );

    // THE CONTROL: the charge is not a constant 2 for every graph. A pass over the two-pass fixture
    // — three dispatchable steps, no fan-out — charges 3, so the assertions above are not satisfied
    // by a function that returns 2 whatever it is handed.
    let two_pass = fixture("two-pass.toml", FIXTURE_TWO_PASS);
    assert_eq!(charge_pass(&two_pass, &FanOut::new()), Attempts::new(3));
}

// ---- AC-023 ∩ AC-389 — the refusal, and nothing either criterion does not share ------------------

/// **AC-023**, refusal half only. *"GIVEN an item whose token appetite was fixed at triage / WHEN
/// any activity attempts to raise it mid-run / THEN the raise is refused unless it arrives as a
/// typed budget-change event with an author."*
///
/// **AC-389**, the other side of C3: *"THEN the raise is refused and the attempt is appended as an
/// event AND raising the appetite is possible only by returning the item to triage."*
///
/// What both criteria assert is that a raise **arriving on an activity result** is refused, and that
/// is all this test asserts. The escape hatch AC-023 admits is not built (no `EventKind` can carry a
/// typed budget change and the enum is closed at 29), and AC-389's append is not built either — it
/// is the half where the two differ. **C3 is RESOLVED in favour of AC-389 — refused outright — by
/// DECISIONS-2026-09-14 D-13**, which CUTS AC-023's *"unless"* clause and withdraws its second
/// instrument, *"an event-schema test that every appetite change carries an author field"*, **with**
/// the clause rather than leaving it ARMED. These three lines read *"so appending would take C3.
/// **C3 is open and a decision is owed.** … is **ARMED, NOT DISCHARGED**"* until 2026-09-15.
/// **AC-389's append half is still unbuilt and still owed a record — residual R-1 in
/// `spikes/DECISIONS-2026-09-15.md`** — so the sentence above, that it is not built, stands.
///
/// The envelope is read off the fixture's **declared** `budget` table; nothing in this test or in
/// the library supplies a token appetite of its own.
#[test]
fn appetite_raise_from_an_activity_result_is_refused() {
    let activity = fixture("two-pass.toml", FIXTURE_TWO_PASS);
    let envelope = declared_envelope(&activity);

    // THE PREMISE: the appetite is non-zero, or "raised to twice it" is a raise to zero and the
    // refusal below is about nothing.
    assert!(
        envelope.tokens.get() > 0,
        "the fixture declares a non-zero token appetite"
    );

    let before = format!("{envelope:?}");
    let proposed = Tokens::new(envelope.tokens.get() * 2);

    let err = appetite_raise_from_activity_result(&envelope, proposed)
        .expect_err("an in-run raise arriving on an activity result is refused");
    assert_eq!(
        err,
        AppetiteRaiseRefused {
            fixed_at_triage: envelope.tokens,
            proposed,
        },
        "the refusal names the triage value and what was asked for"
    );

    // The envelope is byte-identical before and after: the function takes it by shared reference,
    // so this restates a type fact rather than discovering one — and a type fact a test can read is
    // worth reading, because the signature is what a later change would quietly widen.
    assert_eq!(
        before,
        format!("{envelope:?}"),
        "the envelope is unchanged across the refused raise"
    );

    // THE CONTROL, and it is the discriminating one. A guard written as `proposed > current` would
    // refuse the raise above and let an EQUAL or LOWER appetite write through on the same path —
    // and neither criterion admits ANY appetite write from an activity result. Both are refused.
    for other in [envelope.tokens, Tokens::new(envelope.tokens.get() / 2)] {
        assert!(
            appetite_raise_from_activity_result(&envelope, other).is_err(),
            "no proposal arriving on an activity result may write the appetite, including \
             {other}: C3 is resolved in favour of AC-389 (DECISIONS-2026-09-14 D-13), so the only \
             route to a higher appetite is a return to triage — an activity result is not one"
        );
    }
}

// ---- AC-024 / AC-025 — the exhaustion gate, its refusal, and the mark a wall clock owes ---------

/// The pack that declares the per-item token appetite. **Compiled in, never opened**, as above.
const FIXTURE_PACK_ENVELOPE: &str = include_str!("../fixtures/packs/runtime-envelope.toml");
/// The pack that declares none, so the refusal has something to refuse.
const FIXTURE_PACK_NO_APPETITE: &str = include_str!("../fixtures/packs/runtime-no-appetite.toml");
/// `src/workflow/budget.rs`'s own source, for AC-024's *"no thread sleeps"* half.
const BUDGET_SOURCE: &str = include_str!("../../src/workflow/budget.rs");

/// A ledger at an exact spend, built the only way a ledger can be built: by folding charges.
///
/// There is no constructor — [`BudgetLedger`]'s fields are private and the fold is the only writer,
/// which is the *"never a mutable accumulator held across a process"* rule as a type fact. So a
/// test that wants a ledger at a chosen spend charges it.
fn ledger_at(
    envelope: &BudgetEnvelope,
    attempts: u32,
    tokens: u64,
    wall_clock: u64,
) -> BudgetLedger {
    let (ledger, _) = fold(
        &[
            Charge::Attempts(Attempts::new(attempts)),
            Charge::Tokens(Tokens::new(tokens)),
            Charge::WallClock(WallClockSecs::new(wall_clock)),
        ],
        envelope,
    );
    ledger
}

/// The evidence the two-pass fixture's steps actually mint, per ADR-0036.
fn minted_by(activity: &Activity) -> Vec<Evidence> {
    let mut out: Vec<Evidence> = activity
        .steps
        .iter()
        .flat_map(rk_kernel::catalog::Step::mints)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// **AC-024 + AC-025.** Two criteria, five obligations, one name — §7b.13 spells it this way and
/// the name is asserted verbatim by §7b.14's third loop.
///
/// **AC-025:** *"GIVEN a gate whose evidence-producing activity ran out of any budget / WHEN the
/// gate is evaluated / THEN the result is `Err(BudgetExhausted)` and the gate does not close.
/// Verified by: a unit test per budget asserting the gate does not close; a negative fixture
/// proving no code path maps an exhausted budget to `pass`."*
///
/// **AC-024:** *"GIVEN a run that reaches its wall-clock budget / WHEN the budget expires / THEN the
/// kernel writes a durable mark and exits 0; no thread sleeps waiting for the budget to be
/// extended."*
///
/// # What is measured, one obligation at a time
///
/// 1. **A unit test per budget** — three cases swept over `Counter::ALL`, so a fourth counter is a
///    compile error here and not a row this test forgot. Each asserts `Err(BudgetExhausted {
///    counter })` **and** that no verdict came back at all.
/// 2. **The negative fixture** — a table of nine ledgers × two evidence sets, asserting that no
///    combination of inputs produces a closed gate while any counter has run out. This is the
///    discriminating instrument and the one easiest to omit: the three cases above are all passed by
///    a gate that closes for one input in ten. Its companion is the compile-fail case
///    `tests/ui/ac025_exhaustion_is_not_a_pass.rs`, which is taken **in addition to** the register's
///    named instrument and never in substitution for it.
/// 3. **The must-close positive** — without it, a gate that never closes passes obligations 1 and 2
///    perfectly, and the whole test is satisfied by `fn evaluate_gate(..) -> Err(..)`.
/// 4. **AC-024's durable mark** — the value, its carrier (`EventKind::GateOpened`, kind
///    `budget_extension`), and the control that a run inside its wall clock owes none.
/// 5. **AC-024's *"no thread sleeps"*** — a source lint over `src/workflow/budget.rs`'s code lines
///    with an anchor assertion, so an emptied or renamed file is not a vacuous pass.
///
/// # 🛑 ARMED, NOT DISCHARGED — named rather than left for a reader to discover
///
/// * **AC-024's *"exits 0"* — NOT HERE, AND NO LONGER NOWHERE (corrected 2026-09-14).** This bullet
///   read *"`rk-kernel` is a library; there is no `rk-cli` before Step 7d, and a test that called
///   `std::process::exit` would kill this harness rather than assert anything. The exit is
///   **7d's**"*. The middle clause is still true and is why the half is not asserted **in this
///   test**: a test cannot assert its own exit. It does not follow that no test can assert a
///   **child's**, and operator decision **D-8** built the one that does —
///   `cross_process::ac024_wall_clock_mark_outlives_the_process_that_wrote_it`. What remains 7d's is
///   that the exiting process is `redkiln run`. AC-024 is also a **twin** — the owner is the
///   human-gate pair AC-240 + AC-278 — so this half may not be double-claimed against that area.
/// * **The mark's *write*.** The append is outside the pure function (ADR-0119 clause 5, ADR-0120
///   clause 4) and is Slice 4h's, with `rk-ports`' `EventStore`. What is asserted here is the value
///   the budget hands the writer.
/// * **"No further dispatch"** needs a dispatch counter and therefore the runtime. Slice 4h.
/// * **The gate record itself.** ADR-0017's `(token, tree_hash, deadline)` triple is the FSM's
///   (Slice 4e): the `tree_hash` is `rk-git`'s and ADR-0105 clause 4 forbids a second derivation.
#[test]
fn exhaustion_ac024_ac025() {
    let activity = fixture("two-pass.toml", FIXTURE_TWO_PASS);
    let declared = declared_envelope(&activity);

    // The token budget is the PACK's per-item appetite, not a number this test chose and not the
    // activity's own `budget.tokens`: §7b.10 requires `K`, `N` and the appetite to be "declared in
    // the pack and refused as code defaults".
    let pack = Pack::from_toml("runtime-envelope.toml", FIXTURE_PACK_ENVELOPE)
        .expect("the runtime-envelope pack loads");
    let appetite = pack.require_token_appetite().expect(
        "the fixture pack declares `appetite.tokens`; there is no default behind this call",
    );

    // THE PREMISE for that sentence: the pack's appetite is a DIFFERENT number from the activity's
    // declared `budget.tokens`, so an assembly that quietly used the activity's would be caught.
    assert_ne!(
        appetite, declared.tokens,
        "the pack's appetite and the activity's declared tokens must differ, or 'the appetite came \
         from the pack' is unfalsifiable here"
    );

    let envelope = BudgetEnvelope {
        attempts_total: declared.attempts_total,
        tokens: appetite,
        wall_clock_secs: declared.wall_clock_secs,
    };

    // AND THE REFUSAL: a pack that declares no appetite yields no envelope. Zero would close
    // nothing and unbounded would close everything; `None` is neither (`select/pack.rs`:249-256).
    let bare = Pack::from_toml("runtime-no-appetite.toml", FIXTURE_PACK_NO_APPETITE)
        .expect("a pack that declares no appetite still LOADS: the absence is not a parse error");
    let refused = bare
        .require_token_appetite()
        .expect_err("a gate is not evaluated against a defaulted token budget");
    assert_eq!(refused.as_str(), "token_appetite_not_declared");

    // The gate's evidence: what this activity's steps actually mint (ADR-0036), and a demand it
    // cannot satisfy.
    let minted = minted_by(&activity);
    assert!(
        minted.len() >= 2,
        "the fixture must mint real evidence, or 'satisfied' is an empty claim: {minted:?}"
    );
    let satisfied = GateEvidence::new(minted.clone(), minted.clone());
    let unsatisfied = GateEvidence::new(
        minted.iter().copied().chain([Evidence::Mutation]),
        minted.clone(),
    );
    assert!(satisfied.is_satisfied());
    assert_eq!(
        unsatisfied.missing(),
        vec![Evidence::Mutation],
        "the premise for every 'the gate opens for a reason' assertion below"
    );

    // ---- 1. AC-025's unit test per budget, swept over the closed set of counters ---------------
    for counter in Counter::ALL {
        // A ledger that has run out on exactly this counter and on no other, AT THAT COUNTER'S OWN
        // BOUNDARY: `>` for attempts and tokens (AC-025's *"ran out"*), `>=` for the wall clock
        // (AC-024's *"reaches"*). The boundary itself is pinned by
        // `minimum_declaration_is_not_exhausted_by_its_own_clean_pass`; what is asserted here is
        // AC-025's per-budget obligation, one case per counter. The match is exhaustive, so a
        // fourth counter fails to compile here rather than being skipped.
        let ledger = match counter {
            Counter::Attempts => ledger_at(&envelope, envelope.attempts_total.get() + 1, 1, 1),
            Counter::Tokens => ledger_at(&envelope, 1, envelope.tokens.get() + 1, 1),
            Counter::WallClock => ledger_at(&envelope, 1, 1, envelope.wall_clock_secs.get()),
        };

        // THE PREMISE: this ledger really has run out on this counter and on no other, or the
        // assertion below is about a different case than the one it names.
        assert_eq!(
            exhausted_counter(&ledger, &envelope),
            Some(counter),
            "the {counter} case must exhaust {counter} and nothing else: {ledger:?}"
        );

        let err = evaluate_gate(&ledger, &envelope, &satisfied).expect_err(
            "AC-025: an activity that ran out of any budget yields Err(BudgetExhausted), even when \
             every piece of evidence the gate demands is in",
        );
        assert_eq!(err, BudgetExhausted { counter });
        assert_eq!(err.counter.as_str(), counter.as_str());
    }

    // ---- 2. THE NEGATIVE FIXTURE: no code path maps an exhausted budget to a pass --------------
    //
    // Nine ledgers spanning both sides of every boundary, each crossed with both evidence sets.
    // The expectation column is written out here rather than computed, so this table is an
    // independent statement of what exhaustion means and not a restatement of the implementation.
    // AC-025's own words are "*no code path*", and one happy-path case cannot say that.
    //
    // THE BOUNDARY IS PER COUNTER, and the three "at the declared limit" rows are where that shows:
    // attempts and tokens AT their declared number have spent the budget, not run out of it
    // (AC-025's *"ran out"* → `>`), while a run that has REACHED its wall clock has (AC-024's
    // *"reaches"* → `>=`). `minimum_declaration_is_not_exhausted_by_its_own_clean_pass` pins the
    // rule itself; these rows are it crossed with the evidence sets.
    let cap = envelope.attempts_total.get();
    let tok = envelope.tokens.get();
    let wall = envelope.wall_clock_secs.get();
    let table: [(&str, BudgetLedger, Option<Counter>); 9] = [
        (
            "well inside every limit",
            ledger_at(&envelope, 1, 10, 5),
            None,
        ),
        (
            "one unit inside every limit",
            ledger_at(&envelope, cap - 1, tok - 1, wall - 1),
            None,
        ),
        (
            "attempts at the declared limit — spent, NOT run out",
            ledger_at(&envelope, cap, 10, 5),
            None,
        ),
        (
            "attempts past it",
            ledger_at(&envelope, cap + 1, 10, 5),
            Some(Counter::Attempts),
        ),
        (
            "tokens at the declared limit — spent, NOT run out",
            ledger_at(&envelope, 1, tok, 5),
            None,
        ),
        (
            "tokens past it",
            ledger_at(&envelope, 1, tok + 1, 5),
            Some(Counter::Tokens),
        ),
        (
            "wall clock at the declared limit — REACHED, and that is AC-024's word",
            ledger_at(&envelope, 1, 10, wall),
            Some(Counter::WallClock),
        ),
        (
            "wall clock past it",
            ledger_at(&envelope, 1, 10, wall + 1),
            Some(Counter::WallClock),
        ),
        (
            "all three past their limits",
            ledger_at(&envelope, cap + 1, tok + 1, wall + 1),
            Some(Counter::Attempts),
        ),
    ];

    let mut closed = 0_usize;
    for (label, ledger, expected) in &table {
        for (evidence_label, evidence) in [("satisfied", &satisfied), ("missing", &unsatisfied)] {
            let verdict = evaluate_gate(ledger, &envelope, evidence);
            match expected {
                Some(counter) => {
                    let err = verdict.as_ref().err().unwrap_or_else(|| {
                        panic!(
                            "AC-025: `{label}` with {evidence_label} evidence must not produce a \
                             verdict at all, and it produced {verdict:?}"
                        )
                    });
                    assert_eq!(err, &BudgetExhausted { counter: *counter }, "{label}");
                    assert!(
                        !verdict.is_ok_and(|v| v.is_closed()),
                        "no code path maps an exhausted budget to a pass — `{label}`, \
                         {evidence_label} evidence"
                    );
                }
                None => {
                    let v = verdict.unwrap_or_else(|e| {
                        panic!("`{label}` has run out of nothing, and the gate refused with {e}")
                    });
                    if v.is_closed() {
                        closed += 1;
                    } else {
                        let reason = v.reason().unwrap_or_default().to_owned();
                        assert!(
                            reason.contains("Mutation"),
                            "an open gate names what is missing, and this one said {reason:?}"
                        );
                    }
                }
            }
        }
    }

    // ---- 3. THE MUST-CLOSE POSITIVE, and it is the control the other two obligations need ------
    //
    // Without it every assertion above is satisfied by `fn evaluate_gate(..) -> Err(..)`, or by one
    // that never closes: "the gate does not close" is trivially true of a gate that cannot.
    assert_eq!(
        closed, 4,
        "exactly the four not-run-out rows with satisfied evidence must CLOSE — the two inside \
         every limit and the two AT the attempts and token lines — or this test is satisfied by a \
         gate that never closes"
    );

    // ---- 4. AC-024's durable mark, its carrier, and the run that owes none ----------------------
    let expired = ledger_at(&envelope, 1, 10, wall);
    let mark = wall_clock_expiry_mark(&expired, &envelope)
        .expect("a run that has reached its wall-clock budget owes a durable mark");
    assert_eq!(
        BudgetExtensionMark::event_kind(),
        EventKind::GateOpened,
        "the mark rides one of the closed 29 kinds; nothing here mints a thirtieth (ADR-0116)"
    );
    assert_eq!(mark.kind(), "budget_extension");
    assert_eq!(mark.declared(), envelope.wall_clock_secs);
    assert_eq!(mark.spent(), WallClockSecs::new(wall));

    // THE CONTROL: one second inside the declared wall clock, everything else identical. Without it
    // the assertion above is satisfied by a function that returns a mark for every input — which
    // would open a budget-extension gate on every run that ever evaluated a budget.
    let inside = ledger_at(&envelope, 1, 10, wall - 1);
    assert_eq!(
        wall_clock_expiry_mark(&inside, &envelope),
        None,
        "a run inside its wall clock owes no mark"
    );

    // ---- 5. AC-024's "no thread sleeps", over the module's own source --------------------------
    //
    // The structural half is AC-003's `#![deny(clippy::disallowed_methods)]` on `crate::workflow`,
    // which is re-run here and NOT re-claimed (§7b.15). What this adds is the file-local statement
    // that the elapsed value arrives as an argument: there is no clock read and no sleep in the
    // module at all, so "no thread sleeps waiting for the budget to be extended" is a property of
    // the code rather than of a timer that happens to be absent today. Comment lines are stripped
    // first — the criterion's own THEN is quoted in this file and in the module header, and a lint
    // that matched prose would be measuring its own documentation.
    let code: String = BUDGET_SOURCE
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    // THE ANCHOR, asserted before the absences: an emptied, renamed or moved file passes every
    // "does not contain" assertion in existence (`src/rigor.rs`:862's form).
    assert!(
        code.contains("pub fn evaluate_gate"),
        "the anchor applied: this lint is reading the gate's own module"
    );
    assert!(
        code.contains("pub fn wall_clock_expiry_mark"),
        "the anchor applied: this lint is reading AC-024's own function"
    );
    for banned in ["sleep", "SystemTime", "Instant::", "std::time"] {
        assert!(
            !code.contains(banned),
            "AC-024/AC-001: `{banned}` appears in `src/workflow/budget.rs`'s code. Time reaches \
             the workflow only as a journaled activity result"
        );
    }
}

// ---- THE PER-COUNTER BOUNDARY, and the activity that declares rule 8's bare minimum -------------

/// **The exhaustion boundary, one rule per counter** — and the live defect it repairs.
///
/// `exhausted_counter` stays **one** function, so [`fold`]'s observation and [`evaluate_gate`]'s
/// verdict still cannot disagree, but the comparison follows each criterion's own verb:
///
/// * **AC-025** — *"an activity that **ran out** of any budget"* → `spent > declared` for
///   [`Counter::Attempts`] and [`Counter::Tokens`]. Spending exactly what was declared is spending
///   the budget, not running out of it.
/// * **AC-024** — *"a run that **reaches** its wall-clock budget"* → `spent >= declared` for
///   [`Counter::WallClock`]. *"Reaches"* is the criterion's own word and it names the equality.
///
/// The declared-`0` guard is kept on all three: nothing can run out of a budget it never drew on.
///
/// # The defect was live, not theoretical [measured 2026-09-14]
///
/// **9 of the 102 shipped activities declare exactly their lawful ADR-0037 rule-8 minimum** —
/// `change-summary` 2/2, `change-summary-ue` 3/3, `checklist-review` 2/2, `debt-logging` 2/2,
/// `dependency-update` 6/6, `rolling-deploy` 4/4, `smoke-test` 2/2, `t-shirt-sizing` 2/2,
/// `unit-tests` 4/4. Under `>=` each is exhausted by its own **successful** pass, so
/// [`evaluate_gate`] could never close a gate on its evidence. `tests/fixtures/catalog/two-pass.toml`
/// is the same shape (`attempts_total = 3`, `required_attempts_total` = 3) and is what this test
/// runs on, so the case is reproduced in-tree rather than asserted about `spikes/p9/`.
///
/// # Why this name carries no AC id
///
/// **A dated note fixing the comparison per counter is OWED**: neither AC-024 nor AC-025 states the
/// comparison outright, and the verbs above are a reading of them. The name is deliberately not
/// `ac024_…`/`ac025_…` for the same reason
/// `appetite_raise_from_an_activity_result_is_refused` is not `ac023_…` — claiming the id would
/// claim a settlement no record has made. `exhaustion_ac024_ac025` keeps both ids and both
/// criteria's own halves; what is pinned **here** is the boundary.
///
/// # The controls, without which this test is satisfied by a gate that never refuses
///
/// A second clean pass **is** exhausted, tokens past the declared number **are** exhausted, and the
/// wall clock **at** its declared number is exhausted while attempts and tokens at theirs are not —
/// so a blanket `>` on all three fails here, and so does a blanket `>=`.
#[test]
fn minimum_declaration_is_not_exhausted_by_its_own_clean_pass() {
    let activity = fixture("two-pass.toml", FIXTURE_TWO_PASS);
    let envelope = declared_envelope(&activity);

    // THE PREMISE: this activity declares EXACTLY rule 8's minimum, with no slack. Without it the
    // clean pass below could be inside a roomy budget and the test would assert nothing.
    assert_eq!(
        required_attempts_total(&activity),
        envelope.attempts_total,
        "ADR-0037 rule 8 is `budget.attempts_total >= agent/tool steps on one pass`, and the \
         fixture declares it with zero slack — that is the whole case"
    );

    // ONE CLEAN PASS spends exactly the declared amount.
    let pass = charge_pass(&activity, &FanOut::new());
    assert_eq!(
        pass, envelope.attempts_total,
        "one pass must spend the whole declared budget, or 'exhausted by its own pass' is not the \
         situation under test"
    );

    let minted = minted_by(&activity);
    let satisfied = GateEvidence::new(minted.clone(), minted);
    assert!(satisfied.is_satisfied(), "the premise for a closed gate");

    // AC-025's verb: the pass SPENT its budget, it did not RUN OUT of it.
    let after_one_pass = ledger_at(&envelope, pass.get(), 10, 5);
    assert_eq!(
        exhausted_counter(&after_one_pass, &envelope),
        None,
        "an activity declaring rule 8's lawful minimum is not exhausted by its own successful pass"
    );
    let verdict = evaluate_gate(&after_one_pass, &envelope, &satisfied)
        .expect("the gate is evaluated, not refused, after one clean pass");
    assert!(
        verdict.is_closed(),
        "AC-025 closes the gate on evidence a minimally-declared activity actually minted: \
         {verdict:?}"
    );

    // ---- CONTROL 1: a SECOND pass does run out, so the rule is not "never exhausted" -----------
    let after_two_passes = ledger_at(&envelope, pass.get() * 2, 10, 5);
    assert_eq!(
        exhausted_counter(&after_two_passes, &envelope),
        Some(Counter::Attempts),
        "the draw-down accumulates across passes (ADR-0122 decision 3), so the second pass runs out"
    );
    assert_eq!(
        evaluate_gate(&after_two_passes, &envelope, &satisfied),
        Err(BudgetExhausted {
            counter: Counter::Attempts
        }),
    );

    // ---- CONTROL 2: tokens take the SAME verb as attempts, and the wall clock does NOT ---------
    //
    // Three ledgers, each at exactly one declared number. A blanket `>` fails the wall-clock row
    // and a blanket `>=` fails the other two, so this block is what discriminates the per-counter
    // rule from either uniform one.
    let at_attempts = ledger_at(&envelope, envelope.attempts_total.get(), 10, 5);
    let at_tokens = ledger_at(&envelope, 1, envelope.tokens.get(), 5);
    let at_wall = ledger_at(&envelope, 1, 10, envelope.wall_clock_secs.get());
    assert_eq!(
        (
            exhausted_counter(&at_attempts, &envelope),
            exhausted_counter(&at_tokens, &envelope),
            exhausted_counter(&at_wall, &envelope),
        ),
        (None, None, Some(Counter::WallClock)),
        "AC-025's `ran out` is `>` for attempts and tokens; AC-024's `reaches` is `>=` for the \
         wall clock"
    );

    // AND ONE UNIT PAST EACH, so "not exhausted at the line" is not "never exhausted".
    let past_attempts = ledger_at(&envelope, envelope.attempts_total.get() + 1, 10, 5);
    let past_tokens = ledger_at(&envelope, 1, envelope.tokens.get() + 1, 5);
    assert_eq!(
        (
            exhausted_counter(&past_attempts, &envelope),
            exhausted_counter(&past_tokens, &envelope),
        ),
        (Some(Counter::Attempts), Some(Counter::Tokens)),
    );

    // ---- CONTROL 3: the declared-`0` guard survives the change ---------------------------------
    //
    // Nothing runs out of a budget it never drew on. `0 > 0` is false for attempts and tokens
    // anyway; the guard is load-bearing only for the wall clock's `>=`, and that is why it stays.
    let undeclared = BudgetEnvelope {
        attempts_total: Attempts::new(0),
        tokens: Tokens::new(0),
        wall_clock_secs: WallClockSecs::new(0),
    };
    let (empty, report) = fold(&[], &undeclared);
    assert_eq!(exhausted_counter(&empty, &undeclared), None);
    assert_eq!(
        report.exhausted, None,
        "the observation calls the same function as the verdict"
    );
}
