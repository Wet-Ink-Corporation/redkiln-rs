//! **`s7g` §7g.4: a `tool` step reaches `rk-adapter-toolchain-rust` through the `Toolchain` port —
//! asserted by the ADAPTER BEING OBSERVED rather than by the step being selected.**
//!
//! §7g.4 states that bullet and its checkpoint block asserted nothing for it. The two lints the
//! stage discharges are **source** lints: they pass on the existence of a `ports.toolchain` spelling
//! in the kernel, and a `ports.toolchain` mention satisfies them without any adapter ever running.
//! That gap is the whole shape this programme keeps paying for, so the bullet gets its own test.
//!
//! # How "the adapter was observed" is discriminated, and why a fixture cannot fake it
//!
//! The real adapter and the fake differ in a way no mention can imitate: `FixtureToolchain::build`
//! returns **`Ok(())`** unconditionally, while `RustToolchain::build` builds a `Command` and hands
//! it to `rk_containment::run_bounded`, which for an **absolute, nonexistent** program fails to
//! spawn and comes back as a `StepError`. So the test points the adapter's `program` at an absolute
//! path that cannot exist and asserts the step **failed**.
//!
//! 🛑 **That is a negative assertion, so it is paired with a POSITIVE control in the same test.**
//! A negative over an empty or refusing world is satisfied by every failure mode — a bad `Ports`, a
//! bad `Step`, a typo in the verb. The control drives the **identical** `Step` through the
//! **identical** `Runtime` shape with `FixtureToolchain` in the port instead, and asserts it
//! **succeeds**. One `Step`, one walk, two toolchains, opposite outcomes: the only thing that moved
//! is which adapter the port held, so the outcome is attributable to the adapter and to nothing else.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rk_kernel::catalog::{Step, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::step::StepVerdict;
use rk_ports::event_store::EventStore;
use rk_ports::root::Ports;
use rk_ports::toolchain::Toolchain;

/// The one `tool` step both halves drive. `toolchain:build` is in ADR-0135's four-verb map and is
/// the cheapest of the four — it takes no selection and parses no report.
fn build_step() -> Step {
    Step {
        id: "verify".to_owned(),
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

/// An empty evaluation context, which is CORRECT here and would not be on a run path.
///
/// ADR-0142 gives the executor the selection's own context, because a guard evaluated against an
/// empty one fails on `UnknownField`. [`build_step`] declares **no `when` and no `expr`**, so
/// nothing in this test ever reads a binding: the context is threaded to satisfy the signature and
/// is deliberately empty so the test stays about the `tool` arm reaching the port, not about facets.
fn empty_context() -> rk_kernel::predicate::Context {
    rk_kernel::predicate::Context::new()
}

fn runtime_with(toolchain: Arc<dyn Toolchain>) -> Runtime {
    let store: Arc<Mutex<dyn EventStore>> =
        Arc::new(Mutex::new(rk_ports::fakes::VecStore::default()));
    Runtime::new(
        Ports {
            store,
            // The harness is never reached: this walk drives ONE `tool` step.
            harness: Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new())),
            tracker: Arc::new(rk_ports::fakes::BoardTracker::default()),
            docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
            toolchain,
        },
        "run-toolchain-observed",
        "implement",
    )
}

/// **AC-473's mechanism, exercised: the `tool` arm reaches the REAL adapter.**
#[test]
fn a_tool_step_reaches_the_rust_adapter_and_not_the_fixture() {
    let root = std::env::temp_dir();

    // THE POSITIVE CONTROL FIRST, so a failure of the negative half below cannot be read as a pass.
    // Identical step, identical runtime shape, fixture in the port: the walk SUCCEEDS.
    let control = runtime_with(Arc::new(rk_ports::fakes::FixtureToolchain::default()));
    let control_verdict = control
        .execute_step(&build_step(), &empty_context())
        .expect("the control walk completes");
    assert_eq!(
        control_verdict,
        StepVerdict::Continue,
        "CONTROL: `FixtureToolchain::build` returns `Ok(())`, so this step must succeed. It did \
         not, which means the negative half below would pass for a reason that has nothing to do \
         with which adapter the port holds: {control_verdict:?}"
    );

    // THE REAL ADAPTER, with an absolute program that cannot exist.
    let program = root.join("rk-s7g-no-such-cargo-9f3a1c").join("cargo.exe");
    assert!(
        program.is_absolute() && !program.exists(),
        "the discriminator needs an ABSOLUTE, NONEXISTENT program: {}",
        program.display()
    );
    let mut child_env: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in std::env::vars() {
        child_env.insert(k, v);
    }
    let config = rk_adapter_toolchain_rust::Config::new(
        program,
        Duration::from_secs(60),
        &root,
        child_env,
        PathBuf::from(&root),
    )
    .expect("the adapter configuration is accepted");
    let adapter = rk_adapter_toolchain_rust::RustToolchain::new(config)
        .expect("the shipped capability registry loads");

    let real = runtime_with(Arc::new(adapter));
    let verdict = real
        .execute_step(&build_step(), &empty_context())
        .expect("the walk completes; a step's own failure is a VALUE, not an error (AC-032)");

    // 🛑 **THE ADAPTER WAS OBSERVED.** The identical step through the identical walk succeeded with
    // the fixture and failed here, and the only difference is the adapter behind the port.
    match verdict {
        StepVerdict::Failed(e) => {
            // And the failure is the ADAPTER's, not a routing refusal. ADR-0135 refuses an UNMAPPED
            // verb with `Unsupported { capability: "<verb>" }` — a bare verb name. If that is what
            // came back, the step never reached the port at all and this test would be asserting
            // the map rather than the adapter.
            let rendered = format!("{e:?}");
            assert!(
                !rendered.contains("\"build\""),
                "the step was refused by ADR-0135's verb map rather than by the adapter — it never \
                 reached the port: {rendered}"
            );
        }
        other => panic!(
            "the real `rk-adapter-toolchain-rust` was handed an absolute nonexistent program and \
             the step did not fail: {other:?}. Either the adapter was not reached, or a spawn \
             failure is being swallowed."
        ),
    }
}
