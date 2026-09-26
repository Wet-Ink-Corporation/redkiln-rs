//! **`"affected"` is the package-level affected set** (s8c §8c.3; record T = ADR-0169 decision 7
//! rules 1-7, proposed; AC-326, AC-328, AC-332, AC-347, AC-352).
//!
//! Over the workspace fixture, `a` <- `b` <- `c` (`b` depends on `a`, `c` DEV-depends on `b`):
//! the affected set is the touched packages plus their reverse-dependency closure over
//! `resolve.nodes`, dev-dependencies included, computed by the PURE `affected_packages` over
//! `cargo metadata --format-version 1` read before it is called. The rules, in order: the
//! `affected-selection` row must be `Recorded` (1); an empty change set is refused by name (2); a
//! path in no package forces the whole suite with `PolicyForced` (3); the closure becomes `-p`
//! flags (4); no recorded whole-suite count, or one at another toolchain version, runs the whole
//! suite with `ProbeUnverified` (5); a `-p` set that lists no test escalates to the whole suite with
//! `SelectionEmpty` (6); otherwise `basis: Affected`, no reason, and the total from the recorded
//! run (7).
//!
//! Every test pins `Force(cargo test)`: the nextest path is proved elsewhere, and these assert the
//! selection, not the runner.
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::path::Path;

use rk_adapter_toolchain_rust::affected::{
    AFFECTED_POLICY, AffectedSet, Metadata, affected_packages,
};
use rk_adapter_toolchain_rust::{ForcedRunner, RunnerPolicy, RustToolchain};
use rk_ports::toolchain::{RecordedWholeSuite, Selection, StepBound, TestOutcome, Toolchain};
use rk_types::error::StepError;
use rk_types::verify::{Basis, BasisReason, TotalTestCountSource};

use crate::support::{adapter_over, copy_tree, fixture_workspace, whole};

fn over(root: &Path, scratch: &Path) -> RustToolchain {
    adapter_over(root, RunnerPolicy::Force(ForcedRunner::CargoTest), scratch)
}

fn affected(changed: &[&str], recorded: Option<RecordedWholeSuite>) -> Selection {
    Selection::Affected {
        changed: changed.iter().map(|s| (*s).to_owned()).collect(),
        recorded,
    }
}

/// The whole-suite run the store would have recorded: its count and its toolchain version.
fn recorded_from(tc: &RustToolchain) -> RecordedWholeSuite {
    let whole = tc
        .test(&whole(), StepBound::Undeclared)
        .expect("PREMISE: the workspace's whole suite runs");
    assert_eq!(
        whole.failed, 0,
        "PREMISE: the workspace is green unselected"
    );
    RecordedWholeSuite {
        run_id: "run-recorded-whole".to_owned(),
        toolchain_version: whole.report.toolchain_version.clone(),
        count: whole.report.selected_test_count,
    }
}

/// The `-p` packages a run vector names, in order.
fn packages_in(argv: &[String]) -> Vec<String> {
    argv.windows(2)
        .filter(|w| w[0] == "-p")
        .map(|w| w[1].clone())
        .collect()
}

fn assert_whole_suite(outcome: &TestOutcome, reason: BasisReason, why: &str) {
    assert_eq!(
        outcome.report.basis,
        Basis::WholeSuite,
        "{why}: {outcome:?}"
    );
    assert_eq!(
        outcome.report.basis_reason,
        Some(reason),
        "{why}: {outcome:?}"
    );
    assert!(
        outcome.report.argv.iter().any(|a| a == "--workspace"),
        "{why}: the run is the whole workspace: {:?}",
        outcome.report.argv
    );
    assert!(
        packages_in(&outcome.report.argv).is_empty(),
        "{why}: no -p set on a whole-suite run: {:?}",
        outcome.report.argv
    );
    assert_eq!(
        outcome.report.total_test_count_source,
        TotalTestCountSource::OwnExecutedCount,
        "{why}: a whole-suite report's total is its own executed count (the store's rule)"
    );
}

/// A synthetic `cargo metadata --format-version 1` over `root` holding `a`, `b` and `c`, with `b`
/// depending on `a` and `c` on `b` under `c_edge`'s kind (`null` normal, `"dev"`), and an extra
/// package `ab` whose directory shares a string prefix with `a`.
fn synthetic(root: &str, c_edge: &str) -> serde_json::Value {
    let id = |n: &str| format!("path+file:///{root}/{n}#{n}@0.0.0");
    let pkg = |n: &str| {
        serde_json::json!({
            "name": n, "version": "0.0.0", "id": id(n),
            "manifest_path": format!("{root}/{n}/Cargo.toml"), "targets": []
        })
    };
    let dep = |n: &str, kind: serde_json::Value| serde_json::json!({ "name": n, "pkg": id(n), "dep_kinds": [ { "kind": kind, "target": null } ] });
    let c_kind: serde_json::Value = serde_json::from_str(c_edge).expect("a JSON kind");
    serde_json::json!({
        "packages": [pkg("a"), pkg("ab"), pkg("b"), pkg("c")],
        "workspace_members": [id("a"), id("ab"), id("b"), id("c")],
        "workspace_root": root,
        "resolve": { "root": null, "nodes": [
            { "id": id("a"), "deps": [] },
            { "id": id("ab"), "deps": [] },
            { "id": id("b"), "deps": [dep("a", serde_json::Value::Null)] },
            { "id": id("c"), "deps": [dep("b", c_kind)] }
        ] }
    })
}

/// **The affected set is the touched crates plus the reverse-dependency closure, dev-dependencies
/// included** (rule 4; M6's kill), from a PURE function (AC-347), and a run over it is
/// `basis: Affected` with the recorded total (rule 7). An empty change set is refused by name
/// (rule 2).
#[test]
fn affected_set_is_touched_crates_plus_reverse_dependency_closure() {
    // ---- the pure function, over synthetic metadata.
    let root = "D:/w";
    let dev = Metadata::parse(&synthetic(root, "\"dev\""), Path::new(root)).expect("parses");
    let a_change = vec!["a/src/lib.rs".to_owned()];
    let first = affected_packages(&a_change, &dev, AFFECTED_POLICY);
    assert_eq!(
        first,
        AffectedSet::Packages {
            touched: vec!["a".to_owned()],
            packages: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            policy_version: AFFECTED_POLICY.to_owned(),
        },
        "a change to `a` reaches `b` (normal edge) and `c` (a DEV edge on `b`)"
    );
    assert_eq!(
        affected_packages(&a_change, &dev, AFFECTED_POLICY),
        first,
        "a pure function: the same inputs give the same set (AC-347)"
    );
    let normal = Metadata::parse(&synthetic(root, "null"), Path::new(root)).expect("parses");
    assert_eq!(
        affected_packages(&a_change, &normal, AFFECTED_POLICY),
        AffectedSet::Packages {
            touched: vec!["a".to_owned()],
            packages: vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            policy_version: AFFECTED_POLICY.to_owned(),
        },
        "a normal edge is followed too"
    );
    assert_eq!(
        affected_packages(&["c/src/main.rs".to_owned()], &dev, AFFECTED_POLICY),
        AffectedSet::Packages {
            touched: vec!["c".to_owned()],
            packages: vec!["c".to_owned()],
            policy_version: AFFECTED_POLICY.to_owned(),
        },
        "nothing depends on `c`, so its set is itself"
    );
    assert_eq!(
        affected_packages(&["ab\\src\\lib.rs".to_owned()], &dev, AFFECTED_POLICY),
        AffectedSet::Packages {
            touched: vec!["ab".to_owned()],
            packages: vec!["ab".to_owned()],
            policy_version: AFFECTED_POLICY.to_owned(),
        },
        "the longest prefix is taken by PATH COMPONENT: `ab/…` is `ab`'s, never `a`'s"
    );

    // ---- over the real workspace, through the port.
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = over(&fixture_workspace(), scratch.path());
    let recorded = recorded_from(&tc);
    let outcome = tc
        .test(
            &affected(&["a/src/lib.rs"], Some(recorded.clone())),
            StepBound::Undeclared,
        )
        .expect("an affected run over the workspace");
    assert_eq!(
        packages_in(&outcome.report.argv),
        ["a", "b", "c"],
        "the run vector's -p set is the closure: {:?}",
        outcome.report.argv
    );
    assert!(
        !outcome.report.argv.iter().any(|a| a == "--workspace"),
        "-p flags in place of --workspace (the record T deviation): {:?}",
        outcome.report.argv
    );
    assert_eq!(outcome.report.basis, Basis::Affected);
    assert_eq!(
        outcome.report.basis_reason, None,
        "an affected report carries no reason"
    );
    assert_eq!(outcome.report.total_test_count, recorded.count);
    assert_eq!(
        outcome.report.total_test_count_source,
        TotalTestCountSource::RecordedRun {
            run_id: recorded.run_id.clone()
        },
        "the total comes from the recorded run, named (the store's affected rule)"
    );
    assert_eq!(outcome.failed, 0);
    assert!(outcome.passed > 0, "the -p set executed tests: {outcome:?}");
    outcome
        .report
        .validate_shape()
        .expect("the report's own invariants hold");

    let c_only = tc
        .test(
            &affected(&["c/src/main.rs"], Some(recorded.clone())),
            StepBound::Undeclared,
        )
        .expect("an affected run over `c`");
    assert_eq!(
        packages_in(&c_only.report.argv),
        ["c"],
        "{:?}",
        c_only.report.argv
    );
    assert_eq!(c_only.report.basis, Basis::Affected);

    // ---- rule 2: an empty change set is a caller error, refused by name.
    match tc.test(&affected(&[], Some(recorded)), StepBound::Undeclared) {
        Err(StepError::Unsupported { capability }) => assert!(
            capability.contains("affected-selection") && capability.contains("empty"),
            "the refusal names the capability and the empty change set: {capability}"
        ),
        other => panic!(
            "an empty change set is refused by name, never a green or a whole suite; got {other:?}"
        ),
    }
}

/// **A path in no package runs the whole suite, with `PolicyForced`** (rule 3; record T's reading of
/// the reason, flagged for the operator).
#[test]
fn a_path_in_no_package_runs_the_whole_suite_with_a_reason() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = over(&fixture_workspace(), scratch.path());
    let recorded = recorded_from(&tc);
    for changed in [
        vec!["Cargo.lock"],
        vec!["Cargo.toml"],
        vec!["a/src/lib.rs", "README.md"],
        vec![".cargo/config.toml"],
    ] {
        let outcome = tc
            .test(
                &affected(&changed, Some(recorded.clone())),
                StepBound::Undeclared,
            )
            .expect("a whole-suite run");
        assert_whole_suite(
            &outcome,
            BasisReason::PolicyForced,
            &format!("{changed:?} holds a path in no package"),
        );
    }
}

/// **No recorded whole-suite count runs the whole suite, `ProbeUnverified`** (rule 5: AC-326's
/// bootstrap), and so does a recorded count taken at another toolchain version.
#[test]
fn no_recorded_whole_suite_runs_the_whole_suite_probe_unverified() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = over(&fixture_workspace(), scratch.path());
    let outcome = tc
        .test(&affected(&["a/src/lib.rs"], None), StepBound::Undeclared)
        .expect("a whole-suite run");
    assert_whole_suite(
        &outcome,
        BasisReason::ProbeUnverified,
        "no recorded whole suite",
    );

    let mut stale = recorded_from(&tc);
    stale.toolchain_version = "rustc 0.0.0 (000000000 1970-01-01)".to_owned();
    let outcome = tc
        .test(
            &affected(&["a/src/lib.rs"], Some(stale)),
            StepBound::Undeclared,
        )
        .expect("a whole-suite run");
    assert_whole_suite(
        &outcome,
        BasisReason::ProbeUnverified,
        "a recorded count at another toolchain version",
    );
}

/// **A `-p` set that lists no test escalates to the whole suite, `SelectionEmpty`** (rule 6;
/// AC-332's declared policy is escalate). Over a COPY of the workspace whose `c` has no test at
/// all, a change to `c` alone selects a package set that lists nothing.
#[test]
fn a_package_set_that_lists_no_test_escalates_selection_empty() {
    let tmp = crate::support::repo_tempdir();
    let copy = tmp.path().join("ws");
    copy_tree(&fixture_workspace(), &copy);
    std::fs::remove_dir_all(copy.join("c/tests")).expect("drop c's only test file");
    let scratch = tmp.path().join("scratch");
    let tc = over(&copy, &scratch);
    let recorded = recorded_from(&tc);
    let outcome = tc
        .test(
            &affected(&["c/src/main.rs"], Some(recorded)),
            StepBound::Undeclared,
        )
        .expect("an escalated whole-suite run");
    assert_whole_suite(
        &outcome,
        BasisReason::SelectionEmpty,
        "the -p set {c} lists no test",
    );
    assert!(
        outcome.passed > 0,
        "the escalated whole suite executed tests: {outcome:?}"
    );
}

/// **A named selection the adapter does not declare is refused by name** (record T decision 7;
/// `"e2e"` included: no record says what it means for a Cargo workspace).
#[test]
fn a_named_selection_the_adapter_does_not_declare_is_refused_by_name() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc = over(&fixture_workspace(), scratch.path());
    for name in ["e2e", "smoke-lane", ""] {
        match tc.test(&Selection::Named(name.to_owned()), StepBound::Undeclared) {
            Err(StepError::Unsupported { capability }) => assert!(
                capability.contains(&format!("`{name}`")),
                "the refusal names the selection `{name}`: {capability}"
            ),
            other => {
                panic!("an undeclared named selection `{name}` is refused by name; got {other:?}")
            }
        }
    }
}
