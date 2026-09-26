//! **A `tool` step's declared `timeout_secs` is the tighter bound** — s8c §8c.6, record B =
//! ADR-0171 decisions 1-3, proposed.
//!
//! `catalog::Step` carries `timeout_secs` (a literal or a `${toolchain.<key>}`), and the kernel
//! resolves it through the `ToolchainValues` `rk-cli` supplies, AFTER the registry lookup and the
//! route and input checks. An absent or `unmeasured` key refuses by name and **never falls back to
//! the construction bound** (M7). Each case reads the bound the port was HANDED, off the counting
//! fake, so a resolution that never reached the port cannot pass for one that did.
//!
//! The registries and value tables are test-only; the shipped `[toolchain.rust]` is
//! `verb_registry::the_shipped_rust_value_table_is_exactly_the_declared_key_map`'s.

use rk_kernel::catalog::StepTimeout;
use rk_kernel::predicate::Context;
use rk_kernel::runtime::step::{StepVerdict, ToolRouting};
use rk_kernel::workflow::memo::ToolchainValues;
use rk_ports::toolchain::StepBound;
use rk_types::error::StepError;

use crate::runtime::CountingToolchain;
use crate::tool_verb_routing::{bounded_step, registry, rt_routed, tool_step};

const PIN: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

/// A value table whose two measured keys follow ADR-0170 decision 10's rule
/// (`60 × ⌈2 × max / 60⌉`): 300 s → 600 s, and 50 s → 120 s.
fn tables() -> String {
    format!(
        "[toolchain.t]\n\
         build_timeout = {{ secs = 600, measured = [250, 300], rule = \"twice the slowest, up to the minute\", pin = \"{PIN}\", command = \"cargo build\" }}\n\
         scan_timeout = {{ secs = 120, measured = [50], rule = \"twice the slowest, up to the minute\", pin = \"{PIN}\", command = \"cargo audit\" }}\n\
         deploy_timeout = {{ unmeasured = \"no Phase-1 Rust verb deploys\" }}\n"
    )
}

fn routing() -> ToolRouting {
    let reg = registry("", &tables());
    let values = reg.values_for("t").expect("the test table loads");
    ToolRouting::new(reg, values)
}

fn bounds_for(
    step: &rk_kernel::catalog::Step,
    routing: ToolRouting,
) -> (StepVerdict, Vec<StepBound>, u32) {
    let (rt, tc) = rt_routed("step-bound", CountingToolchain::default(), routing);
    let v = rt
        .execute_step(step, &Context::new())
        .expect("a step's own failure is a VALUE (AC-032)");
    (v, tc.bounds(), tc.calls())
}

fn secs(n: u64) -> StepBound {
    StepBound::secs(n).expect("non-zero")
}

#[test]
fn a_literal_timeout_secs_reaches_the_port_call() {
    let (v, bounds, _) = bounds_for(
        &bounded_step("b", "build", StepTimeout::Literal(120)),
        routing(),
    );
    assert_eq!(v, StepVerdict::Continue);
    assert_eq!(
        bounds,
        vec![secs(120)],
        "the literal did not reach the port as the call's bound"
    );

    // The same literal on `lint`, a second port method, so the bound is the step's and not `build`'s.
    let (_, bounds, _) = bounds_for(
        &bounded_step("l", "lint", StepTimeout::Literal(45)),
        routing(),
    );
    assert_eq!(bounds, vec![secs(45)]);
}

#[test]
fn a_toolchain_substitution_resolves_through_the_value_table() {
    let (v, bounds, _) = bounds_for(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("build_timeout".to_owned()),
        ),
        routing(),
    );
    assert_eq!(v, StepVerdict::Continue);
    assert_eq!(
        bounds,
        vec![secs(600)],
        "`${{toolchain.build_timeout}}` is 600 s in the table"
    );
    // A second key on the same verb resolves to ITS value, so the resolution reads the key.
    let (_, bounds, _) = bounds_for(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("scan_timeout".to_owned()),
        ),
        routing(),
    );
    assert_eq!(bounds, vec![secs(120)]);
}

/// **M7: an unresolved key falling back to the construction bound turns this red** — the port
/// would be reached with an undeclared bound.
#[test]
fn an_unresolved_toolchain_key_is_refused_by_name() {
    let refused = |step: &rk_kernel::catalog::Step, routing: ToolRouting, needles: &[&str]| {
        let (v, bounds, calls) = bounds_for(step, routing);
        match v {
            StepVerdict::Failed(StepError::Unsupported { capability }) => {
                for n in needles {
                    assert!(
                        capability.contains(n),
                        "the refusal does not name {n:?}: {capability}"
                    );
                }
            }
            other => panic!(
                "an unresolved bound did not refuse by name ({other:?}); bounds handed to the port: \
                 {bounds:?}. A fallback to the construction bound is what ADR-0171 forbids"
            ),
        }
        assert_eq!(calls, 0, "an unresolved bound reached the port");
    };

    // ABSENT from the table.
    refused(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("gone_timeout".to_owned()),
        ),
        routing(),
        &["${toolchain.gone_timeout}", "absent"],
    );
    // Declared `unmeasured`, naming its reason.
    refused(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("deploy_timeout".to_owned()),
        ),
        routing(),
        &[
            "${toolchain.deploy_timeout}",
            "unmeasured",
            "no Phase-1 Rust verb deploys",
        ],
    );
    // No table selected (no adapter detected): the reason, and no key guessed.
    refused(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("build_timeout".to_owned()),
        ),
        ToolRouting::new(
            registry("", &tables()),
            ToolchainValues::unselected("no toolchain adapter detected a project"),
        ),
        &[
            "${toolchain.build_timeout}",
            "no toolchain adapter detected a project",
        ],
    );
    // An EMPTY table is not a fallback either: every key is absent.
    refused(
        &bounded_step(
            "b",
            "build",
            StepTimeout::Toolchain("build_timeout".to_owned()),
        ),
        ToolRouting::new(registry("", ""), ToolchainValues::default()),
        &["${toolchain.build_timeout}", "absent"],
    );
    // A zero literal is no bound (RT-5).
    refused(
        &bounded_step("b", "build", StepTimeout::Literal(0)),
        routing(),
        &["timeout_secs = 0", "RT-5"],
    );
}

#[test]
fn a_step_without_timeout_secs_passes_an_undeclared_bound() {
    // CONTROL: the same verb with a literal hands the port that literal, so `Undeclared` below is
    // the step's absence and not a fake that records nothing else.
    let (_, bounds, _) = bounds_for(
        &bounded_step("b", "build", StepTimeout::Literal(30)),
        routing(),
    );
    assert_eq!(bounds, vec![secs(30)]);

    let (v, bounds, calls) = bounds_for(&tool_step("b", "build"), routing());
    assert_eq!(v, StepVerdict::Continue);
    assert_eq!(calls, 1);
    assert_eq!(
        bounds,
        vec![StepBound::Undeclared],
        "a step with no timeout_secs hands the port an UNDECLARED bound (the construction bound \
         stands), never a value borrowed from the row's `timeout` key"
    );
}
