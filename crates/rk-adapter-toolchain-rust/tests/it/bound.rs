//! **A step's bound is the tighter one** (s8c §8c.6; record B = ADR-0171 decision 4, proposed):
//! the adapter enforces `min(step, construction)` on every child it spawns for the call, and
//! `Timeout { after_secs }` carries **the bound that fired**, so a journal can tell a step's own
//! bound from the construction bound.
//!
//! The victim is the fixture project's `slow_by_selection` (AC-331's), which holds for 600 s on a
//! Condvar nobody signals, so it outlives every bound here. Each test warms the build first under a
//! generous bound, over the same scratch, so the bound under test times the test child and not a
//! cold compile.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::time::Duration;

use rk_ports::toolchain::StepBound;
use rk_types::error::StepError;

use crate::support::{GENEROUS, adapter, filter, run_test, whole};

fn slow() -> rk_ports::toolchain::Selection {
    filter(&["--ignored", "slow_by_selection"])
}

fn warm(scratch: &std::path::Path) {
    let warm = adapter(GENEROUS, scratch);
    assert_eq!(
        run_test(&warm, &whole()).expect("green").passed,
        2,
        "PREMISE: the fixture is built and green"
    );
}

/// **A step bound tighter than the construction bound fires first** (M4's kill: an adapter that
/// ignores `StepBound` runs to its 60 s construction bound and reports 60).
#[test]
fn a_step_bound_tighter_than_construction_fires_first() {
    let scratch = tempfile::tempdir().expect("scratch");
    warm(scratch.path());
    let tc = adapter(Duration::from_secs(60), scratch.path());
    let bound = StepBound::secs(5).expect("a non-zero bound");
    match rk_ports::toolchain::Toolchain::test(&tc, &slow(), bound) {
        Err(StepError::Timeout { after_secs }) => assert_eq!(
            after_secs, 5,
            "the step's own 5 s bound fired, and Timeout names it, not the 60 s construction bound"
        ),
        other => {
            panic!("a run killed at the step's bound is Timeout {{ after_secs: 5 }}, got {other:?}")
        }
    }
}

/// **The construction bound caps a larger step bound** (M5's kill: `max` in place of `min` runs to
/// the step's 30 s and reports 30).
#[test]
fn the_construction_bound_caps_a_larger_step_bound() {
    let scratch = tempfile::tempdir().expect("scratch");
    warm(scratch.path());
    let tc = adapter(Duration::from_secs(5), scratch.path());
    let bound = StepBound::secs(30).expect("a non-zero bound");
    match rk_ports::toolchain::Toolchain::test(&tc, &slow(), bound) {
        Err(StepError::Timeout { after_secs }) => assert_eq!(
            after_secs, 5,
            "the construction bound (5 s) is the tighter one and fired; Timeout names it, not the step's 30 s"
        ),
        other => panic!(
            "a run killed at the construction bound is Timeout {{ after_secs: 5 }}, got {other:?}"
        ),
    }
}
