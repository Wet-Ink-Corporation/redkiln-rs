//! **ADR-0142: a step guard evaluates against the selection's own context, and DECIDES.**
//!
//! The executor shipped evaluating every guard against `predicate::Context::new()`. An empty
//! context binds nothing and `eval.rs`'s unbound-path arm returns `UnknownField`, so
//! `execute_step` mapped it to `StepVerdict::Failed` and the walk ended. Measured over the shipped
//! pack: **121 of 121 `when` guards failed**, and **67 of the 67 ref-free ones read `rigor`** — they
//! never reached a `${…}` reference at all, so ADR-0136's account of the gap named the wrong cause
//! (corrected by **E-0136-1**).
//!
//! # 🛑 "It stopped erroring" is not the property. DECIDING is.
//!
//! A guard that always returned `true` would also stop erroring, and would run every guarded step
//! unconditionally. So the case below drives **one** step against **three** contexts and asserts
//! three different outcomes:
//!
//! | context | `rigor >= recommended` | verdict |
//! | --- | --- | --- |
//! | empty — what the executor used before | `UnknownField("rigor")` | `Failed` |
//! | the selection's, at `mandatory` | true | `Continue` |
//! | the selection's, at `optional` | false | **`Skipped`** |
//!
//! The first row is the control: it is the pre-ADR-0142 behaviour, asserted rather than remembered,
//! so this test fails if the context stops being threaded. The third is the one that shows the
//! guard is a *decision* and not a formality.
//!
//! # And it makes [`StepVerdict::Skipped`] reachable, which it was not
//!
//! An adversarial pass found `Skipped` **unreachable by construction**: `guard_holds` could never
//! return `Ok(false)`, because `parser::operand` admits only a `${ref}`, a prefix call, or an
//! identifier path — there are no bare literals — so every leaf of every parseable guard errored on
//! an empty context. The variant, its `match` arm and its docstring were all dead. Binding the
//! context is what gives it a first reachable case, and the `optional` row is that case.

use std::sync::Arc;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::predicate::Context;
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::StepVerdict;
use rk_ports::harness::Harness;

use crate::runtime::{counters, ports, store};

/// The facet registry the product ships, so the `rigor` domain is the real one rather than the
/// `Rigor::ALL` fallback `with_rigor` uses when a registry carries no `rigor` facet.
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

/// A step guarded exactly as **67 of the shipped pack's 121** guards are.
fn rigor_guarded_step() -> Step {
    Step {
        id: "guarded".to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: None,
        role: None,
        when: Some("rigor >= recommended".to_owned()),
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

#[test]
fn a_rigor_guard_refuses_on_an_empty_context_and_decides_on_the_selections_one() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(Vec::new()));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-guard-context",
        "implement",
    );
    let step = rigor_guarded_step();
    let registry = shipped_registry();

    // ---- CONTROL: the empty context, which is what shipped. ------------------------------------
    let refused = rt
        .execute_step(&step, &Context::new())
        .expect("a step's own failure is a VALUE, not an error of the walk (AC-032)");
    match &refused {
        StepVerdict::Failed(e) => {
            let rendered = format!("{e:?}");
            assert!(
                rendered.contains("rigor"),
                "the empty-context refusal does not name `rigor`, so it is failing for some other \
                 reason and this control does not establish what ADR-0142 changed: {rendered}"
            );
        }
        other => panic!(
            "PREMISE: an empty context binds nothing, so `rigor >= recommended` must refuse. It \
             did not ({other:?}), which means either the evaluator stopped requiring a binding or \
             this guard no longer reads one — and the two assertions below would then be passing \
             for a reason that has nothing to do with the context being threaded."
        ),
    }

    // ---- THE FIX: the selection's context, at a rigor that SATISFIES the guard. -----------------
    let mandatory = rk_kernel::rigor::with_rigor(Context::new(), Rigor::Mandatory, &registry);
    assert_eq!(
        rt.execute_step(&step, &mandatory)
            .expect("the walk completes"),
        StepVerdict::Continue,
        "`rigor >= recommended` at `mandatory` must RUN the step. If this is `Failed`, the context \
         is not reaching the guard; if it is `Skipped`, the ordinal comparison is inverted."
    );

    // ---- AND AT A RIGOR THAT DOES NOT. ---------------------------------------------------------
    //
    // 🛑 This is the assertion that makes the pair a decision rather than a repair. A guard that
    // always answered `true` would satisfy the row above and run every guarded step in the pack.
    // It is also `StepVerdict::Skipped`'s FIRST reachable case: before the context was bound, no
    // input could produce it.
    assert_eq!(
        rt.execute_step(
            &step,
            &rk_kernel::rigor::with_rigor(Context::new(), Rigor::Optional, &registry)
        )
        .expect("the walk completes"),
        StepVerdict::Skipped,
        "`rigor >= recommended` at `optional` must SKIP the step. A guard that cannot answer `false` \
         is not a guard, and `StepVerdict::Skipped` would remain the dead variant an adversarial \
         pass found it to be."
    );

    assert_eq!(
        harness.dispatches(),
        0,
        "no dispatch: `execute_step`'s `agent` arm names the kind and defers to `run_activity`, so \
         a guard decision must not reach the `Harness` port"
    );
}

/// **ADR-0142 clause 4: the context does NOT repair `check`, and that is asserted, not assumed.**
///
/// All **94** `check` exprs in the shipped pack read a prior step's OUTPUT — `report.status` (22),
/// `check.failed` (21), `findings` (11), `lint.errors` (4) and a tail of singletons. None is a
/// facet, so no context assembled from the item can supply one. This pins that limit so a later
/// reader cannot take "guards now evaluate" as covering `check` steps.
#[test]
fn a_check_expr_reading_a_step_output_still_refuses_with_the_full_context() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let harness = Arc::new(crate::runtime::CountingHarness::scripted(Vec::new()));
    let rt = Runtime::new(
        ports(Arc::clone(&harness) as Arc<dyn Harness>, &db),
        "run-check-limit",
        "implement",
    );
    let check = Step {
        id: "verify".to_owned(),
        kind: StepKind::Check,
        run: None,
        output_type: None,
        role: None,
        when: None,
        // The most frequent `expr` shape in the pack, verbatim in form.
        expr: Some("report.status == 'passing'".to_owned()),
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    };

    let full = rk_kernel::rigor::with_rigor(Context::new(), Rigor::Mandatory, &shipped_registry());
    match rt.execute_step(&check, &full).expect("the walk completes") {
        StepVerdict::Failed(e) => {
            let rendered = format!("{e:?}");
            assert!(
                rendered.contains("report.status"),
                "the refusal does not name the unresolved output, so it does not tell an operator \
                 what is missing: {rendered}"
            );
        }
        other => panic!(
            "a `check` expr reading a step OUTPUT resolved against a facet context ({other:?}). \
             Either step-output resolution landed — in which case ADR-0142 clause 4 and ADR-0136 \
             clause 4 are discharged and this test should be replaced rather than deleted — or the \
             arm stopped evaluating its `expr` at all."
        ),
    }
}
