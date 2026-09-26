//! **AC-334's real row — the F25 ordering inversion against the real Node adapter** (s7c §7c.14;
//! ARMED from 7b-ii, whose `ac015_inversion_regression` replayed the RECORD; this runs the SUITE).
//! Behind `[features] corpus = []` — the feature gates this TEST MODULE (`#[cfg(feature =
//! "corpus")] mod ac334;` in `main.rs`), never the `rk-kernel` dev edge, and the block runs it as
//! `cargo test -p rk-adapter-toolchain-node --features corpus -- ac334_flip_predicate_real_toolchain`.
//!
//! **Over the predecessor checkout the preflight materialised ONCE** — `target/predecessor-f21f55fc`
//! (git-ignored), cloned from the pinned bundle and checked out at
//! `f21f55fc355eb49b42faf2eb07cad266b647ec1e`, the F25 record's own SHA (its inversion patch's hunk
//! is `@@ -1797,7 +1797,7 @@` there), `pnpm install --frozen-lockfile` run there once (network;
//! local-only; no junction to the live clone's `node_modules` — CORPUS_PIN). **This test never
//! clones and never installs**: it refuses, naming the preflight, when the checkout is absent, at
//! another SHA, uninstalled, or DIRTY (a previous run left the inversion applied — the operator
//! runs `git -C $PRED checkout -- .` and re-runs; Halt 6's shape).
//!
//! **The inversion is applied through `git apply` and reverted with `git apply -R` in a `Drop`
//! guard** ([`Reverted`]), so the persistent path is byte-identical before and after every run, a
//! killed or panicking attempt included — a checkout left carrying the inversion would make the
//! next run's `apply` refuse, or make the *mutation survived* baseline the mutant itself and
//! invert the verdict silently (the third review) — and `git status --porcelain` is asserted
//! empty after the revert **inside the test**. Both git calls go through `rk_git::git_exec::run`,
//! the wrapper module's fixture form (ADR-0077: no `Command::new("git")` outside `rk-git`).
//!
//! **What is measured, and what the record said.** `record.json`: suite `npx vitest run
//! test/implement-contract.test.ts`, filter `-t "AC-015"`, *"invert the ordering → 36/36 GREEN"*,
//! `suite_green_under_mutant: true`, `assertion_flipped: false`, e3 line 54: *"leaves both AC-015
//! tests GREEN (… → 2 passed)"*. Here: the baseline run through the real adapter reports 2; the
//! inversion applied, the same run reports 2 (green: the suite did not flip); the inverted text is
//! still present at completion (AC-338); the revert leaves the tree clean; and the kernel's
//! `flip_predicate` over that measured `MutationRun` says **`MutationSurvived`**, never `Pinned`.
//! **Every member of that `MutationRun` is measured here, `assertion_flipped` included**: the
//! adapter's report carries no per-assertion result (RT-26's ARMED per-test result), but the
//! vitest JSON reporter file the adapter's own vector names (`--outputFile=<f>`) carries
//! `testResults[].assertionResults[].{title, fullName, status}`, so the record's named assertion
//! (`expected.assertion.test`) is looked up in the baseline's file and in the mutant's, both must
//! be present, and `assertion_flipped = Some(baseline == "passed" && mutant == "failed")` — the
//! record's `assertion_flipped: false` is what is measured, never a literal copied from it, and a
//! filter that selected two other AC-015 tests would be red on the lookup. The suite runs **under
//! `run_bounded` with the adapter's construction-time timeout** (Halt 5).

use std::path::{Component, Path, PathBuf};

use rk_kernel::verify::flip::{FlipOutcome, MutationRun, flip_predicate};
use rk_types::digest::sha256;

use rk_adapter_toolchain_node::NodeToolchain;

use crate::support::{GENEROUS, config_with, minimal_env, run_test};

/// The `corpus`-feature constant: the predecessor checkout, relative to the repository root.
const PREDECESSOR: &str = "target/predecessor-f21f55fc";
/// The F25 record's own SHA.
const F21: &str = "f21f55fc355eb49b42faf2eb07cad266b647ec1e";
/// The kernel's F25 fixture (7b-ii's copy; 7c re-uses it rather than copying it).
const RECORD: &str =
    include_str!("../../../rk-kernel/tests/fixtures/findings/ac015_inversion/record.json");
const PATCH: &str =
    include_str!("../../../rk-kernel/tests/fixtures/findings/ac015_inversion/inversion.patch");

/// An absolute path with its `..` components resolved lexically — git and CreateProcess both
/// accept `..`, but a clean path reads better in every message.
fn normalise(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for c in p.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

fn predecessor() -> PathBuf {
    normalise(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(PREDECESSOR),
    )
}

fn patch_file() -> PathBuf {
    normalise(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../rk-kernel/tests/fixtures/findings/ac015_inversion/inversion.patch"),
    )
}

/// The named assertion's `status` in the vitest JSON reporter file a recorded vector names
/// (`--outputFile=<f>`), matched on the test's own `title` (vitest's `fullName` prefixes the
/// `describe` titles). Absent is a panic naming the file: the filter did not run that test.
fn assertion_status(argv: &[String], named: &str) -> String {
    let path = argv
        .iter()
        .find_map(|a| a.strip_prefix("--outputFile="))
        .expect("the recorded vector names the reporter file");
    let text = std::fs::read_to_string(path).expect("the reporter file is readable");
    let v: serde_json::Value = serde_json::from_str(&text).expect("the reporter file is JSON");
    let results = v["testResults"].as_array().expect("testResults");
    results
        .iter()
        .flat_map(|file| {
            file["assertionResults"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(&[])
                .iter()
        })
        .find(|a| {
            a["title"].as_str() == Some(named)
                || a["fullName"].as_str().is_some_and(|f| f.ends_with(named))
        })
        .and_then(|a| a["status"].as_str())
        .unwrap_or_else(|| {
            panic!("the reporter file {path} carries no result for the named assertion {named:?}")
        })
        .to_owned()
}

fn git(cwd: &Path, args: &[&str]) -> String {
    rk_git::git_exec::run(cwd, args)
        .unwrap_or_else(|e| panic!("git {args:?} in {}: {e:?}", cwd.display()))
        .stdout_trimmed()
}

/// The inversion, applied on construction and reverted on drop — whatever happens in between.
struct Reverted {
    pred: PathBuf,
    patch: PathBuf,
}

impl Reverted {
    fn apply(pred: &Path, patch: &Path) -> Self {
        git(pred, &["apply", &patch.to_string_lossy()]);
        Self {
            pred: pred.to_path_buf(),
            patch: patch.to_path_buf(),
        }
    }
}

impl Drop for Reverted {
    fn drop(&mut self) {
        // A revert that fails must be loud, not silent: the next run's preflight would refuse
        // on a dirty checkout, and that message names the operator's step.
        if let Err(e) =
            rk_git::git_exec::run(&self.pred, ["apply", "-R", &self.patch.to_string_lossy()])
        {
            eprintln!(
                "AC-334: `git apply -R` FAILED in {}: {e:?} — run `git -C {} checkout -- .` before the next run",
                self.pred.display(),
                self.pred.display()
            );
        }
    }
}

/// The predecessor's preflight (the module doc): present, at F21, installed, clean.
fn preflight(pred: &Path) {
    assert!(
        pred.join(".git").is_dir(),
        "the AC-334 predecessor checkout is absent at {} — run the preflight ONCE (§7c.14): git clone \
         $CORPUS_REPO/predecessor-47faec9f.bundle {PREDECESSOR} && git -C {PREDECESSOR} checkout {F21}",
        pred.display()
    );
    let head = git(pred, &["rev-parse", "HEAD"]);
    assert_eq!(
        head,
        F21,
        "{} is at {head}, not {F21} — the F25 record's provenance SHA",
        pred.display()
    );
    assert!(
        pred.join("node_modules/.bin/vitest").is_file(),
        "run `pnpm install --frozen-lockfile --dir {}` ONCE before this test (network; local-only)",
        pred.display()
    );
    let dirty = git(pred, &["status", "--porcelain"]);
    assert!(
        dirty.is_empty(),
        "{} is dirty — a previous run left the inversion applied; run `git -C {} checkout -- .` and \
         re-run (Halt 6):\n{dirty}",
        pred.display(),
        pred.display()
    );
}

/// **AC-334 (Node real row).** The F25 inversion is mutation-survived against the real adapter.
#[test]
fn ac334_flip_predicate_real_toolchain() {
    let pred = predecessor();
    let patch = patch_file();
    preflight(&pred);

    // The record: the suite, the filter, the digest of the patch on disk.
    let record: serde_json::Value = serde_json::from_str(RECORD).expect("record.json parses");
    assert_eq!(
        sha256(PATCH.replace("\r\n", "\n").as_bytes()).as_str(),
        record["patch_sha256"].as_str().expect("patch_sha256"),
        "the patch on disk is the one the record digests"
    );
    let suite = record["mutation"]["suite"].as_str().expect("suite");
    let file = suite.rsplit(' ').next().expect("the suite names its file");
    assert_eq!(file, "test/implement-contract.test.ts");
    let filter = record["mutation"]["suite_filter"]
        .as_str()
        .expect("suite_filter");
    let (flag, pattern) = filter.split_once(' ').expect("`-t \"<pattern>\"`");
    let pattern = pattern.trim_matches('"');
    assert_eq!((flag, pattern), ("-t", "AC-015"));
    let selection = vec![file.to_owned(), flag.to_owned(), pattern.to_owned()];
    let named = record["expected"]["assertion"]["test"]
        .as_str()
        .expect("the named assertion");
    assert!(
        named.starts_with("AC-015"),
        "the record names the AC-015 assertion: {named}"
    );
    let target = pred.join("workflows/forge-implement.js");
    let inverted_text = "as the FIRST thing you do, before you make the checkpoint at all";

    let scratch = tempfile::tempdir().expect("scratch");
    let tc = NodeToolchain::new(config_with(
        rk_adapter_toolchain_node::DEFAULT_PROGRAM.into(),
        GENEROUS,
        &pred,
        minimal_env(),
        scratch.path(),
    ))
    .expect("loads");

    // The baseline: the recorded "2 passed" under the filter, through the real adapter.
    // *(Amended 2026-09-26, s8c §8c.2: the retired inherent `verify_report` is the port's `test`;
    // the report is the outcome's.)*
    let baseline = run_test(&tc, &selection)
        .expect("the baseline suite is green at F21")
        .report;
    assert_eq!(
        baseline.selected_test_count, 2,
        "the record's `-> 2 passed`"
    );
    let baseline_status = assertion_status(&baseline.argv, named);
    assert_eq!(
        baseline_status, "passed",
        "the named assertion passes at F21 before the inversion"
    );
    println!(
        "AC334_BASELINE_SELECTED={} named_assertion={baseline_status}",
        baseline.selected_test_count
    );

    // The mutant: applied through git, reverted on drop.
    let applied = true;
    let (suite_green, still_present, mutant_count, mutant_status) = {
        let guard = Reverted::apply(&pred, &patch);
        let text = std::fs::read_to_string(&target).expect("forge-implement.js readable");
        assert!(
            text.contains(inverted_text),
            "PREMISE: the inversion is applied"
        );
        let mutant = run_test(&tc, &selection);
        // *(Amended 2026-09-26, s8c §8c.2, record T = ADR-0169 decision 1, proposed: this arm read
        // `Err(StepError::NonZero { .. }) => (false, None)`, RT-22's "a red suite is Err" -- exactly
        // the reading D21-42 1a reverses. A red suite is now `Ok` with `failed > 0`, so the arm is a
        // `failed > 0` read, and its reporter file is readable like a green one's. Every `Err` is
        // cannot-run, killed or unreadable, and is no pass and no failing suite.)*
        let (suite_green, mutant_status) = match &mutant {
            Ok(outcome) => {
                let status = assertion_status(&outcome.report.argv, named);
                println!(
                    "AC334_MUTANT_SELECTED={} failed={} named_assertion={status}",
                    outcome.report.selected_test_count, outcome.failed
                );
                (outcome.failed == 0, Some(status))
            }
            Err(other) => panic!("the mutant run must be a pass or a failing suite, got {other:?}"),
        };
        let still_present = std::fs::read_to_string(&target)
            .expect("readable")
            .contains(inverted_text);
        let count = mutant
            .as_ref()
            .map(|o| o.report.selected_test_count)
            .unwrap_or(0);
        drop(guard);
        (suite_green, still_present, count, mutant_status)
    };

    // The revert left the persistent path byte-identical (Halt 6; the third review).
    let dirty = git(&pred, &["status", "--porcelain"]);
    assert!(
        dirty.is_empty(),
        "the checkout is dirty after the Drop guard's revert:\n{dirty}"
    );
    assert!(
        !std::fs::read_to_string(&target)
            .expect("readable")
            .contains(inverted_text),
        "the inversion was reverted"
    );

    // What was measured, through the kernel's predicate: a survivor, never pinned.
    assert!(
        suite_green,
        "F25: the suite stays GREEN under the inversion (measured here)"
    );
    assert_eq!(
        mutant_count, 2,
        "both AC-015 tests passed under the mutant (e3 line 54)"
    );
    assert!(
        still_present,
        "the mutation was present at completion (AC-338)"
    );
    // `assertion_flipped` is MEASURED: the named assertion's status in the two reporter files.
    let mutant_status = mutant_status.expect("the mutant run was green, so its report exists");
    let assertion_flipped = Some(baseline_status == "passed" && mutant_status == "failed");
    println!("AC334_ASSERTION_FLIPPED={assertion_flipped:?}");
    let run = MutationRun {
        applied,
        suite_green,
        assertion_flipped,
        still_present,
    };
    let outcome = flip_predicate(&run);
    assert_eq!(
        outcome,
        FlipOutcome::MutationSurvived,
        "the F25 inversion is mutation-survived against the real Node adapter"
    );
    assert_ne!(outcome, FlipOutcome::Pinned);
    assert_eq!(
        assertion_flipped,
        Some(false),
        "what was measured agrees with the record's `assertion_flipped: false`"
    );
    println!("AC334_FLIP_OUTCOME={outcome:?}");
}
