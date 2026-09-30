//! **AC-348 and AC-355 over `s8c`'s affected-set function** (`s8g` §8g.5's rows; §8g.1: tests only,
//! no source line of this crate changes). *(Added 2026-09-29, `s8g` build, package
//! S8G-P5-VERIFY-PASS.)*
//!
//! - **AC-348** — *"GIVEN identical inputs … WHEN the selection is computed twice THEN the two
//!   selections are byte-identical"*, by a property test over 200 synthetic change sets. §8g.5's
//!   precondition is that `affected_packages` is *"a pure function of (change set, metadata, policy
//!   version)"*: it takes those three and nothing else (its signature, and `affected.rs`'s header).
//!   The generator PERMUTES each set's order and re-parses the pinned metadata, so a result that
//!   depended on input order or on a hash map's iteration order (M14) differs in some case.
//! - **AC-355** — *"a change whose correct basis is `whole-suite` (a lockfile …) and a change whose
//!   correct basis is `affected` over a single file … both classify correctly on basis and on
//!   selected-test count"*: two fixtures through the port over the real workspace fixture.

use std::path::Path;

use rk_adapter_toolchain_rust::affected::{
    AFFECTED_POLICY, AffectedSet, Metadata, affected_packages,
};
use rk_adapter_toolchain_rust::{ForcedRunner, RunnerPolicy, RustToolchain};
use rk_ports::toolchain::{RecordedWholeSuite, Selection, StepBound, Toolchain};
use rk_types::verify::{Basis, BasisReason};

use crate::support::{adapter_over, fixture_workspace, whole};

/// The pinned `cargo metadata --format-version 1` fixture: six members under `root`, `core` <- `io`
/// <- `net` <- `app`, `core` <- `util` <- `app`, `cli` DEV-depends on `app`, and `solo` alone. The
/// member list is declared out of name order on purpose.
fn pinned_metadata(root: &str) -> serde_json::Value {
    let id = |n: &str| format!("path+file:///{root}/crates/{n}#{n}@0.1.0");
    let pkg = |n: &str| {
        serde_json::json!({
            "name": n, "version": "0.1.0", "id": id(n),
            "manifest_path": format!("{root}/crates/{n}/Cargo.toml"), "targets": []
        })
    };
    let dep = |n: &str, kind: serde_json::Value| serde_json::json!({ "name": n, "pkg": id(n), "dep_kinds": [ { "kind": kind, "target": null } ] });
    let names = ["util", "app", "core", "solo", "net", "cli", "io"];
    serde_json::json!({
        "packages": names.iter().map(|n| pkg(n)).collect::<Vec<_>>(),
        "workspace_members": names.iter().map(|n| id(n)).collect::<Vec<_>>(),
        "workspace_root": root,
        "resolve": { "root": null, "nodes": [
            { "id": id("core"), "deps": [] },
            { "id": id("solo"), "deps": [] },
            { "id": id("io"), "deps": [dep("core", serde_json::Value::Null)] },
            { "id": id("util"), "deps": [dep("core", serde_json::Value::Null)] },
            { "id": id("net"), "deps": [dep("io", serde_json::Value::Null)] },
            { "id": id("app"), "deps": [dep("net", serde_json::Value::Null), dep("util", serde_json::Value::Null)] },
            { "id": id("cli"), "deps": [dep("app", serde_json::json!("dev"))] }
        ] }
    })
}

/// A deterministic generator (a 64-bit LCG), so a failing case is reproducible by its index.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next()).expect("a 31-bit value fits") % n
    }
}

/// **AC-348: the selection is byte-identical over 200 synthetic change sets** — computed once over
/// the set as generated, once over a permutation of it against a freshly parsed copy of the same
/// metadata, and compared as bytes. M14 (the affected set built by iterating a hash map) turns it
/// red.
#[test]
fn ac348_selection_is_byte_identical_over_200_change_sets() {
    let root = "D:/w";
    let metadata = pinned_metadata(root);
    let first = Metadata::parse(&metadata, Path::new(root)).expect("the pinned metadata parses");
    let pool = [
        "crates/core/src/lib.rs",
        "crates/io/src/lib.rs",
        "crates/util/src/lib.rs",
        "crates/net/src/lib.rs",
        "crates/app/src/main.rs",
        "crates/cli/tests/it.rs",
        "crates/solo/src/lib.rs",
        "crates/net/src/tcp.rs",
        "crates/core/Cargo.toml",
    ];
    let mut rng = Lcg(0x5eed_8a48);
    let mut cases = 0_usize;
    let mut multi = 0_usize;
    let mut permuted_differently = 0_usize;
    for case in 0..200 {
        let size = 1 + rng.below(4);
        let mut set: Vec<String> = (0..size)
            .map(|_| pool[rng.below(pool.len())].to_owned())
            .collect();
        // One case in ten carries a workspace-level file (rule 3's whole-suite arm).
        if rng.below(10) == 0 {
            set.push("Cargo.lock".to_owned());
        }
        let mut permuted = set.clone();
        for i in (1..permuted.len()).rev() {
            permuted.swap(i, rng.below(i + 1));
        }
        if permuted != set {
            permuted_differently += 1;
        }
        let second = Metadata::parse(&metadata, Path::new(root)).expect("parses again");
        let a = affected_packages(&set, &first, AFFECTED_POLICY);
        let b = affected_packages(&permuted, &second, AFFECTED_POLICY);
        assert_eq!(
            format!("{a:?}").into_bytes(),
            format!("{b:?}").into_bytes(),
            "case {case}: the selection moved with input order or between two computations: \
             {set:?} vs {permuted:?}"
        );
        if let AffectedSet::Packages { packages, .. } = &a
            && packages.len() >= 3
        {
            multi += 1;
        }
        cases += 1;
    }
    assert_eq!(cases, 200, "exactly 200 change sets were computed");
    assert!(
        multi >= 50,
        "only {multi} cases selected three or more packages: too few for an order defect to show"
    );
    assert!(
        permuted_differently >= 50,
        "CONTROL: the generator permuted only {permuted_differently} sets"
    );
}

fn affected(changed: &[&str], recorded: RecordedWholeSuite) -> Selection {
    Selection::Affected {
        changed: changed.iter().map(|s| (*s).to_owned()).collect(),
        recorded: Some(recorded),
    }
}

/// **AC-355: a lockfile change is whole-suite, and a one-file change is affected** — each asserted
/// on basis AND selected-test count, so a selector that always answers `whole-suite` fails the
/// second fixture and one that always answers `affected` fails the first.
#[test]
fn ac355_lockfile_change_is_whole_suite_and_one_file_is_affected() {
    let scratch = tempfile::tempdir().expect("scratch");
    let tc: RustToolchain = adapter_over(
        &fixture_workspace(),
        RunnerPolicy::Force(ForcedRunner::CargoTest),
        scratch.path(),
    );
    let whole_run = tc
        .test(&whole(), StepBound::Undeclared)
        .expect("PREMISE: the workspace's whole suite runs");
    let whole_count = whole_run.report.selected_test_count;
    assert!(
        whole_count >= 2,
        "PREMISE: the workspace has tests in more than one package"
    );
    let recorded = RecordedWholeSuite {
        run_id: "run-recorded-whole".to_owned(),
        toolchain_version: whole_run.report.toolchain_version.clone(),
        count: whole_count,
    };

    let lockfile = tc
        .test(
            &affected(&["Cargo.lock"], recorded.clone()),
            StepBound::Undeclared,
        )
        .expect("the lockfile fixture runs");
    assert_eq!(
        (lockfile.report.basis, lockfile.report.basis_reason),
        (Basis::WholeSuite, Some(BasisReason::PolicyForced)),
        "a lockfile change is a whole-suite basis: {:?}",
        lockfile.report
    );
    assert_eq!(
        lockfile.report.selected_test_count, whole_count,
        "a whole-suite basis selects the whole suite's count"
    );

    let one_file = tc
        .test(
            &affected(&["c/src/main.rs"], recorded),
            StepBound::Undeclared,
        )
        .expect("the one-file fixture runs");
    assert_eq!(
        (one_file.report.basis, one_file.report.basis_reason),
        (Basis::Affected, None),
        "a one-file change is an affected basis: {:?}",
        one_file.report
    );
    assert!(
        one_file.report.selected_test_count > 0
            && one_file.report.selected_test_count < whole_count,
        "the affected selection runs fewer tests than the whole suite ({} of {whole_count})",
        one_file.report.selected_test_count
    );
}
