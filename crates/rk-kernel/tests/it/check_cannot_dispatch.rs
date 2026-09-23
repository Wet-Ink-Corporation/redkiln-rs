//! **design §4.5's `check` prohibition, asserted BEHAVIOURALLY** — `PROGRESS.md` row 37's finding
//! (20).
//!
//! `conformance::the_check_arm_has_no_receiver_and_so_cannot_reach_a_port` asserts the same
//! prohibition **structurally**, over source text, and its own docstring concedes that it is a
//! *spelling* check. Three defeats are measured, and a wider ban list closes none of them:
//!
//! | # | the defeat | why the source lint cannot see it |
//! | --- | --- | --- |
//! | (a) | a `&super::Runtime` parameter named `rt`, plus a benignly-named helper | zero banned tokens anywhere the lint scans |
//! | (b) | `rt.run_tool_step(step)` through that parameter | same — the ban list is a list of NAMES |
//! | (c) | a `\`-continued string literal inside the `Check` arm whose continuation line contains `StepKind::` | `blank_string_literals` keeps no cross-line state, so the continuation is handed back as CODE and `arm_of("Check")`'s region ENDS there — the dispatch sits past the scan |
//!
//! ⚠️ **(c) IS CONDITIONAL, AND THE CONDITION IS THE INTERESTING PART.** The truncation is silent
//! only when **≥ 40 non-whitespace characters of scanned code precede the continuation line**. Put
//! the literal FIRST in the arm and the lint goes RED — but on `joined.len() > 40`, *"the
//! `StepKind::Check` arm joined to 27 chars — the scan is vacuous"*, which is the **vacuity floor
//! doing its job and not the ban**. Measured twice, independently: once here and once by an
//! adversarial pass that reconstructed the mutant from scratch. So the floor genuinely bites the
//! naive shape, and an author who wants the defeat has to pad past it. Saying "the floors do not
//! fire" without that condition — as this table did until 2026-09-22 — credits the lint with less
//! than it has and the attacker with more.
//!
//! This module is the behavioural half. A `Toolchain` fake that COUNTS is bound into the port and
//! the counter is read after each `check` step. That is the instrument the source lint cannot be:
//! it does not care what the dispatch is *called*.
//!
//! # 🛑 TWO INSTRUMENTS, BECAUSE ONE OF THEM DOES NOT BITE ON THE SHIPPED FIXTURE SHAPE
//!
//! **Measured: 0 of the pack's 94 `check` steps carry a `run` key.** So a dispatch written into the
//! `check` arm as `run_tool_step(step)` reaches `memo::verb_of("")` → `None` and is refused
//! *"names no toolchain verb"* **before any port is touched**. On those fixtures the counter stays
//! `0` and instrument (i) is GREEN ON THE MUTANT.
//!
//! So the module carries two readings, and they kill disjoint sets:
//!
//! - **(i) the counter** — `CountingToolchain::calls()` must not move across the step.
//! - **(ii) the verdict and its own words** — the arm's refusal must be the `check` arm's, not the
//!   `tool` arm's. A dispatch that refuses early still *changes the message*, and that is visible
//!   even when the port was never reached.
//!
//! So **four fixtures carry a SYNTHETIC `run = "toolchain:build"`** — a shape the shipped pack never
//! uses — because that key is the only thing that lets instrument (i) see a dispatch at all. A
//! fixture's job is to make a capability observable, not to mirror the corpus.
//!
//! 🛑 **AND WHICH FOUR IS THE WHOLE LESSON OF THIS MODULE'S SECOND DRAFT.** The first draft gave the
//! `run` key to **C7 alone**, whose `expr` takes the `Ok(true)` path — and an adversarial pass then
//! built a mutant that dispatches **only on the arm's other two paths**:
//!
//! ```text
//! let verdict = match check_verdict(step, ctx) { … };
//! if verdict != StepVerdict::Continue { let _ = self.run_tool_step(step); }
//! verdict
//! ```
//!
//! It **SURVIVED all 219 tests and the source lint**. Three blindnesses composed: C7 never reaches
//! the dispatch because it returns `Continue`; C1–C4 reach it but carry no `run`, so
//! `memo::verb_of("")` refuses before the port and the counter cannot move; and `let _ =` discards
//! the refusal, so every message needle still matches. A second mutant gated on
//! `step.when.is_some()` survived for the same reason — **no fixture had a guard that was present
//! AND held**, because both guarded fixtures use `rigor < recommended`, which makes the step
//! `Skipped` before the arm ever runs.
//!
//! The fixture set answers both: **C1** (the `Err` path) and **C4** (the `Ok(false)` path) now carry
//! the key as well as C7 (`Ok(true)`), so a dispatch on ANY of the arm's three outcomes moves the
//! counter; and **C8** carries a guard that is present and **holds**, so `step.when` is no longer an
//! unobserved discriminator. One fixture per path is not thoroughness — it is the minimum at which
//! the instrument can see the arm at all.
//!
//! ⚠️ **What this module does NOT do, stated rather than left to be discovered.** It does not
//! repair finding (8) — `blank_string_literals`' missing cross-line state is still live, and defeat
//! (c) still blinds the source lint. This is a **backstop**, which is a different claim from a fix:
//! the source lint keeps catching the cheap spellings at zero runtime cost, and these tests catch
//! the ones it cannot name.
//!
//! # Why the fixtures sit where they do
//!
//! *Where a fixture puts the interesting step is part of what it asserts.* Four gate tests all at
//! reach 1 missed an infinite re-open (finding (17)); every walk fixture with its non-agent step
//! LAST missed a `break` (finding (19)). So the walk below puts a `check` step at **six** positions
//! — first, after a skipped step, after a committed dispatch, after the port has ALREADY been
//! reached, and carrying a mapped `run` — and the per-step test covers the arms a walk cannot reach
//! more than one of, because `Err` and `Ok(false)` each END a walk.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::Activation;
use rk_kernel::runtime::step::StepVerdict;
use rk_ports::harness::Harness;
use rk_ports::toolchain::Toolchain;

use crate::runtime::{
    AID, CAP, CountingHarness, CountingToolchain, ITEM, STAGE, STEP, at, counters, empty_state,
    gate_site, ok_typed, ports_with_toolchain, spec, store,
};

/// The facet registry the product ships, so the `rigor` domain is the real one rather than the
/// `Rigor::ALL` fallback. Copied from `guard_context.rs`, which is the module that established it.
fn shipped_registry() -> Registry {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves");
    let src = std::fs::read_to_string(root.join(".redkiln").join("facets.toml"))
        .expect("the shipped facet registry is readable");
    Registry::from_toml(&src).expect("the shipped facet registry loads")
}

/// **One** context for every fixture: the selection's own, with `rigor` at the TOP of the shipped
/// domain.
///
/// One and not two, deliberately — a second context is a second thing to keep in sync, and
/// `Rigor::Mandatory` already makes `rigor >= recommended` decide TRUE and `rigor < recommended`
/// decide FALSE, which is both of the `check` arm's deciding paths off a single value.
fn full_context() -> Context {
    rk_kernel::rigor::with_rigor(Context::new(), Rigor::Mandatory, &shipped_registry())
}

/// A `check` step. `when` guards it; `expr` is what the arm evaluates; `run` is normally absent.
fn check_step(id: &str, expr: Option<&str>, when: Option<&str>, run: Option<&str>) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Check,
        run: run.map(ToOwned::to_owned),
        output_type: None,
        role: None,
        when: when.map(ToOwned::to_owned),
        expr: expr.map(ToOwned::to_owned),
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

fn agent_step(id: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: None,
        role: Some("implementer".to_owned()),
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

/// A `tool` step naming `toolchain:build` — one of ADR-0135's four MAPPED verbs, so it DOES reach
/// the port. This is the positive control's instrument.
fn build_step(id: &str) -> Step {
    Step {
        id: id.to_owned(),
        kind: StepKind::Tool,
        run: Some("toolchain:build".to_owned()),
        output_type: None,
        role: None,
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

const fn activation<'a>(sp: &'a rk_types::activity::ActivitySpec) -> Activation<'a> {
    Activation {
        item: ITEM,
        activity_id_at_version: AID,
        step_id: STEP,
        spec: sp,
    }
}

/// A `Runtime` whose `Toolchain` port counts, returned with the counter and the harness.
fn rt_with_counters(run_id: &str) -> (Runtime, Arc<CountingHarness>, Arc<CountingToolchain>) {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(CountingHarness::scripted(vec![ok_typed(
        serde_json::json!({"ok": true}),
    )]));
    let tc = Arc::new(CountingToolchain::default());
    let rt = Runtime::new(
        ports_with_toolchain(
            Arc::clone(&harness) as Arc<dyn Harness>,
            &db,
            Arc::clone(&tc) as Arc<dyn Toolchain>,
        ),
        run_id,
        STAGE,
    );
    (rt, harness, tc)
}

/// What a fixture is expected to do, so a fixture that stopped exercising its arm fails LOUDLY
/// rather than passing the counter assertion by never reaching the code.
enum Expect {
    /// `StepVerdict::Continue` — the arm decided TRUE.
    Continues,
    /// `StepVerdict::Skipped` — the guard held the step off and the arm never ran.
    Skipped,
    /// `StepVerdict::Failed`, whose rendering contains this substring.
    FailsNaming(&'static str),
}

/// **THE PROHIBITION, per arm: no `check` step reaches the `Toolchain` port, whatever it decides.**
#[test]
fn a_check_step_never_reaches_the_toolchain_port() {
    let ctx = full_context();

    // ---- THE POSITIVE CONTROL FIRST. -----------------------------------------------------------
    //
    // A zero read off a port that nothing can reach is satisfied by a broken `Ports`, an unbound
    // toolchain, a typo in `run` and a verb outside ADR-0135's map alike. So: the SAME runtime
    // shape, a `tool` step, and the counter MOVES.
    let (control_rt, _h, control_tc) = rt_with_counters("run-check-control");
    assert_eq!(
        control_tc.calls(),
        0,
        "CONTROL: the counter did not start at zero"
    );
    let control_verdict = control_rt
        .execute_step(&build_step("verify"), &ctx)
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert_eq!(
        control_verdict,
        StepVerdict::Continue,
        "CONTROL: a `toolchain:build` step did not succeed through the counting fake, whose \
         `build` is an unconditional `Ok(())`: {control_verdict:?}"
    );
    assert_eq!(
        control_tc.calls(),
        1,
        "CONTROL: a `tool` step naming a MAPPED verb did not reach the port. Every zero below \
         would then hold for a reason that has nothing to do with the `check` arm."
    );

    // ---- THE FIXTURES. Every distinguishable path through the `check` arm. ----------------------
    //
    // C6 is the one that does NOT exercise the arm — its guard holds the step off — and it is here
    // as a POSITION rather than as arm coverage. Saying so is the difference between a fixture set
    // and a fixture set that reads as bigger than it is.
    let fixtures: [(&str, Step, Expect); 8] = [
        (
            "C1 no `expr` at all, carrying a MAPPED `run` (see the header: the Err path)",
            check_step("c1", None, None, Some("toolchain:build")),
            Expect::FailsNaming("carries no `expr`"),
        ),
        (
            "C2 an `expr` reading a prior step's OUTPUT (the pack's most frequent shape)",
            check_step("c2", Some("report.status == 'passing'"), None, None),
            // NOT the bare `report.status`: that string is the fixture's own `expr`, and the arm
            // interpolates `{src}` into BOTH its parse-failure and its evaluation-failure message,
            // so it would match either. This needle names the EVALUATION branch and nothing else.
            Expect::FailsNaming("the `expr` of check step"),
        ),
        (
            "C3 an `expr` that does not parse (ADR-0103 decision 1 bans the chain)",
            check_step("c3", Some("rigor < recommended < mandatory"), None, None),
            // Corrected before it was trusted: this needle read `"check step"`, which BOTH failure
            // messages carry, so the fixture named the parse branch while asserting something true
            // of the evaluation branch too. Measured: with this needle it still passes, so C3 does
            // take the parse path.
            Expect::FailsNaming("the `expr` of check step"),
        ),
        (
            "C4 an `expr` that decides FALSE",
            check_step(
                "c4",
                Some("rigor < recommended"),
                None,
                Some("toolchain:build"),
            ),
            Expect::FailsNaming("did not hold"),
        ),
        (
            "C5 an `expr` that decides TRUE",
            check_step("c5", Some("rigor >= recommended"), None, None),
            Expect::Continues,
        ),
        (
            "C6 a guard that holds the step off (a POSITION, not arm coverage)",
            check_step(
                "c6",
                Some("rigor >= recommended"),
                Some("rigor < recommended"),
                None,
            ),
            Expect::Skipped,
        ),
        (
            "C7 SYNTHETIC: a `check` carrying a MAPPED `run`, so defeat (a) becomes observable",
            check_step(
                "c7",
                Some("rigor >= recommended"),
                None,
                Some("toolchain:build"),
            ),
            Expect::Continues,
        ),
        (
            "C8 a guard that is PRESENT and HOLDS, also carrying a MAPPED `run`",
            check_step(
                "c8",
                Some("rigor >= recommended"),
                Some("rigor >= recommended"),
                Some("toolchain:build"),
            ),
            Expect::Continues,
        ),
    ];

    for (name, step, expect) in fixtures {
        // A FRESH runtime per fixture, so one fixture's reading cannot be another's residue.
        let (rt, _harness, tc) = rt_with_counters("run-check-arm");

        // Captured, never assumed to be 0: the assertion below then reads "THIS STEP moved it by
        // zero" and survives an edit that runs something ahead of it.
        let before = tc.calls();
        let verdict = rt
            .execute_step(&step, &ctx)
            .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

        // ---- INSTRUMENT (i): the port was not reached. -----------------------------------------
        assert_eq!(
            tc.calls(),
            before,
            "{name}: the `check` step reached the `Toolchain` port ({before} -> {}). design §4.5's \
             \"cannot dispatch\" is a prohibition, not a convention, and no ban list of NAMES can \
             assert it — this counter can. Verdict was {verdict:?}",
            tc.calls()
        );

        // ---- INSTRUMENT (ii): it was the CHECK arm that decided, in its own words. -------------
        //
        // This is the half that bites when a dispatch refuses BEFORE the port (a `check` step
        // carries no `run`, so `verb_of` refuses it early and instrument (i) stays green).
        match (&expect, &verdict) {
            // One or-pattern rather than two arms: the bodies are both empty, and `clippy`'s
            // `match_same_arms` is denied in this workspace.
            (Expect::Continues, StepVerdict::Continue)
            | (Expect::Skipped, StepVerdict::Skipped) => {}
            (Expect::FailsNaming(needle), StepVerdict::Failed(e)) => {
                let rendered = format!("{e:?}");
                assert!(
                    rendered.contains(needle),
                    "{name}: the step failed, but not in the `check` arm's own words — it does \
                     not contain {needle:?}. A dispatch written into the arm refuses with the \
                     `tool` arm's message instead, which is exactly this reading: {rendered}"
                );
            }
            (_, got) => panic!(
                "{name}: the fixture no longer exercises the path it was built for — expected \
                 {}, got {got:?}. A fixture that stopped reaching its arm would satisfy the \
                 counter assertion above by never running the code it is about.",
                match expect {
                    Expect::Continues => "Continue".to_owned(),
                    Expect::Skipped => "Skipped".to_owned(),
                    Expect::FailsNaming(n) => format!("Failed naming {n:?}"),
                }
            ),
        }
    }
}

/// **THE PROHIBITION, over a real WALK, with the `check` steps at six different positions.**
#[test]
fn a_walk_of_check_steps_moves_the_toolchain_counter_exactly_once_and_that_once_is_the_tool_step() {
    let ctx = full_context();
    let state = empty_state();
    let sp = spec();

    // ---- WALK A, THE CONTROL: the counter DOES move through a walk. ----------------------------
    let (rt_a, harness_a, tc_a) = rt_with_counters("run-walk-control");
    let control_steps = vec![agent_step("draft"), build_step("verify")];
    let control = rt_a
        .run_activity_steps(
            activation(&sp),
            Some(&control_steps),
            &ctx,
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    assert_eq!(
        tc_a.calls(),
        1,
        "CONTROL: a `tool` step did not reach the port THROUGH THE WALK. The `1` asserted below \
         would then be unreachable for reasons unrelated to `check`: {control:?}"
    );
    assert!(control.committed(), "CONTROL: the walk did not commit");
    assert_eq!(harness_a.dispatches(), 1, "CONTROL: exactly one dispatch");

    // ---- WALK B: six `check` steps, six positions, one `tool` step. ----------------------------
    let (rt_b, harness_b, tc_b) = rt_with_counters("run-walk-checks");
    let steps = vec![
        // position 1: a check as the FIRST step of the walk
        check_step("opens", Some("rigor >= recommended"), None, None),
        // position 2: a step the guard holds off
        check_step(
            "not-now",
            Some("rigor >= recommended"),
            Some("rigor < recommended"),
            None,
        ),
        // position 3: a check immediately AFTER a skipped step
        check_step("after-skip", Some("rigor >= recommended"), None, None),
        agent_step("draft"),
        // position 4: a check AFTER a committed dispatch
        check_step("after-agent", Some("rigor >= recommended"), None, None),
        // THE ONLY STEP IN THIS WALK THAT MAY MOVE THE COUNTER
        build_step("build-it"),
        // position 5: a check AFTER the port has already been reached
        check_step("after-tool", Some("rigor >= recommended"), None, None),
        // position 6: a check CARRYING a mapped `run`, after the counter has moved
        check_step(
            "with-run",
            Some("rigor >= recommended"),
            None,
            Some("toolchain:build"),
        ),
        // ENDS the walk. It must be last: an `Err` check is terminal, so "a check after a failing
        // check" is not a position this fixture can hold, and widening it means a SECOND walk
        // rather than a longer one.
        check_step("verify", Some("report.status == 'passing'"), None, None),
    ];
    let report = rt_b
        .run_activity_steps(
            activation(&sp),
            Some(&steps),
            &ctx,
            gate_site(&state),
            rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
            at(100),
        )
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");

    assert_eq!(
        tc_b.calls(),
        1,
        "the walk reached the `Toolchain` port {} times and exactly ONE of its nine steps may: the \
         `tool` step. Six `check` steps at six different positions each contributed zero, and the \
         reading against which that is a \"did not move FURTHER\" is Walk A's 1: {report:?}",
        tc_b.calls()
    );
    assert!(
        report.committed(),
        "the `check` steps broke the commit: {report:?}"
    );
    assert_eq!(harness_b.dispatches(), 1, "exactly one dispatch");

    // The walk REACHED THE END rather than stopping early at one of the passing checks — and the
    // failure it ends on is the last check's own, not a toolchain one.
    let err = report.step_error().unwrap_or_else(|| {
        panic!("the walk did not end on its final `check` step's refusal: {report:?}")
    });
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("report.status"),
        "the walk ended on something other than the final check's unresolved output, so the steps \
         after the `tool` step may never have run at all: {rendered}"
    );
}
