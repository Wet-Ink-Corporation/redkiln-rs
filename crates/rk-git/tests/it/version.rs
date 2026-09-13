//! The startup precondition (ADR-0077; ADR-0091 decision 4).
//!
//! The load-bearing test here is `the_floor_in_the_code_equals_the_floor_in_the_record`. Checkpoint
//! 6's floor grep (`git (>=|version) [0-9]+\.[0-9]+`) matches ADR-0091's *measurement prose* as
//! readily as its decision — `git version 2.55.0.windows.3 on this box` satisfies it — so the grep
//! alone can read as discharged by a sentence that is not the decision. This test closes that: the
//! constant the wrapper asserts on is compared against the number the record actually decides.

use rk_git::error::GitError;
use rk_git::version::{self, GIT_FLOOR, GIT_FLOOR_STR};

/// ADR-0091 decision 4 and `version::GIT_FLOOR` must be the same number. If the record is amended
/// and the constant is not, this goes red — which is the only thing that stops the two drifting.
#[test]
fn the_floor_in_the_code_equals_the_floor_in_the_record() {
    let adr =
        crate::support::read_adr_0091().expect("ADR-0091 must exist — it is Step 6.1's record");

    // The decision sentence, not the measurement prose: the record states the floor as
    // "at least **git >= <N>**" inside its Decision section.
    let needle = format!("git >= {GIT_FLOOR_STR}");
    assert!(
        adr.contains(&needle),
        "ADR-0091 does not state the floor as `{needle}`; version::GIT_FLOOR is {GIT_FLOOR:?} and \
         the record and the code have drifted"
    );
}

#[test]
fn the_precondition_passes_on_this_box_and_reports_what_it_found() {
    let fx = crate::support::Fixture::new();
    let tc = version::assert_at_startup(fx.path()).expect("git on PATH at or above the floor");
    println!("git floor {GIT_FLOOR_STR}; found {}", tc.found);
    assert!(
        (tc.found.major, tc.found.minor) >= GIT_FLOOR,
        "assert_at_startup minted a token below the floor"
    );
}

/// Control: "at or above the floor" is a FLOOR check, not a match (Halt 2's distinction). A newer
/// git must pass, or the guard is scoped wrong in the other direction.
#[test]
fn control_a_newer_git_is_above_the_floor() {
    let v = version::parse_version("git version 2.55.0.windows.3").expect("parses");
    assert!((v.major, v.minor) >= GIT_FLOOR);
}

/// Control: the discriminating direction — something genuinely below the floor is refused.
#[test]
fn control_an_older_git_is_below_the_floor() {
    let v = version::parse_version("git version 2.19.2").expect("parses");
    assert!(
        (v.major, v.minor) < GIT_FLOOR,
        "2.19 must be below the 2.20 floor — that is the version `--if-exists=add` needs"
    );
}

/// An unparsable answer is its own error, and must NOT read as `0.0.0`, which would compare below
/// any floor and refuse a healthy toolchain (golden constraint 9).
#[test]
fn an_unparsable_version_is_not_read_as_zero() {
    assert_eq!(version::parse_version("git version"), None);
    assert_eq!(version::parse_version(""), None);
    assert_eq!(version::parse_version("no digits here"), None);
}

#[test]
fn a_version_without_a_patch_component_still_parses() {
    let v = version::parse_version("git version 2.20").expect("2.20 parses");
    assert_eq!((v.major, v.minor, v.patch), (2, 20, 0));
}

/// The error carries both halves, so a failure names the floor rather than only the finding.
#[test]
fn below_floor_error_names_both_versions() {
    let e = GitError::VersionBelowFloor {
        found: "git version 2.19.2".into(),
        floor: GIT_FLOOR_STR,
    };
    match e {
        GitError::VersionBelowFloor { found, floor } => {
            assert!(found.contains("2.19.2"));
            assert_eq!(floor, "2.20");
        }
        other => panic!("wrong variant: {other:?}"),
    }
}
