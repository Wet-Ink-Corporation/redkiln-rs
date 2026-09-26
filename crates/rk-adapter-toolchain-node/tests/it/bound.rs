//! **A step's bound is the tighter one** (s8c §8c.6; record B = ADR-0171 decision 4, proposed): the
//! Node adapter enforces `min(step, construction)` on every child it spawns, and
//! `Timeout { after_secs }` names the bound that fired. The victim is the fixture's skip-marked
//! slow test, reachable by `-t slow_by_selection` only (AC-331's).
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::time::Duration;

use rk_ports::toolchain::{Selection, StepBound, Toolchain};
use rk_types::error::StepError;

use crate::support::{SUITE_FILE, adapter};

/// **A step bound tighter than the construction bound fires first** (an adapter ignoring
/// `StepBound` runs to its 60 s construction bound and reports 60).
#[test]
fn a_step_bound_tighter_than_construction_fires_first() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = adapter(Duration::from_secs(60), scratch.path());
    let selection = Selection::Filter(vec![
        SUITE_FILE.to_owned(),
        "-t".to_owned(),
        "slow_by_selection".to_owned(),
    ]);
    match tc.test(&selection, StepBound::secs(8).expect("a non-zero bound")) {
        Err(StepError::Timeout { after_secs }) => assert_eq!(
            after_secs, 8,
            "the step's own 8 s bound fired, and Timeout names it, not the 60 s construction bound"
        ),
        other => {
            panic!("a run killed at the step's bound is Timeout {{ after_secs: 8 }}, got {other:?}")
        }
    }
}
