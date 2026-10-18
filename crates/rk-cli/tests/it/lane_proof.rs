//! **`s8j` §8j.4 — both shipped lanes proven end to end under a scripted harness** (D21-40
//! decision 2's fixture proofs), and §8j.3's three boundary-hold proofs over the same fixture.
//!
//! - Every proof enters through `rk_cli::verbs::run::run` (the binary arm through the built
//!   `redkiln`). None calls `replay::resume` or a kernel seam.
//! - The toolchain is the REAL `rk-adapter-toolchain-rust`, chosen by `detect()` over the run-owned
//!   worktree (ADR-0137 decision 2), `program = cargo` (`toolchain_programs` unset). Never
//!   `FixtureToolchain`, never `rk-fixture-toolchain`; Checkpoint 8j's floor greps this file.
//! - The fixture repository is [`rig::fixture`]: the `lane` fixture workspace, a byte copy of the
//!   tracked `.redkiln/` minus `worktrees/` (asserting `registry.toml` arrived), the two lane items,
//!   and the product's own `init`.
//! - The harness is [`rig::ScriptedLane`]: one response per `(activity@version, step id, visit)`
//!   and **no default response**; an unscripted dispatch fails the proof by name.
//!
//! Each proof builds the fixture crate in the run's own target directory, so a proof costs real
//! `cargo` time (ADR-0137 decision 3).

#[cfg(test)]
mod rig;

use std::collections::BTreeMap;

use rk_types::acceptance::AcceptanceCriterion;
use rk_types::artifact::lane;
use rk_types::text::Text;

use rig::{FIX_ITEM, Key, Response};

/// The fixture crate's planted defect, fixed: what the scripted implementer writes at `green`.
const FIXED_LIB: &str = "//! The lane proofs' fixture crate, fixed by the lane.\n\n\
/// The number of words in `text`, a word being a maximal run of non-whitespace characters.\n\
#[must_use]\npub fn word_count(text: &str) -> usize {\n    text.split_whitespace().count()\n}\n\n\
#[cfg(test)]\nmod tests {\n    use super::word_count;\n\n    #[test]\n    fn counts_single_spaced_words() {\n        \
assert_eq!(word_count(\"one two three\"), 3);\n    }\n}\n";

/// The reproduction the investigator writes: red over the defect, green after the fix.
const REPRO_TEST: &str = "#[test]\nfn repro_empty_text_has_no_words() {\n    \
assert_eq!(rk_lane_fixture::word_count(\"\"), 0);\n}\n";
/// The repro's command: the kernel's one selection parser reads the tokens after ` -- `.
const REPRO_NAME: &str = "cargo test -- repro_empty_text_has_no_words";

/// The investigator's discriminating test: red over the defect, green after the fix.
const HYPOTHESIS_TEST: &str = "#[test]\nfn hypothesis_doubled_space_is_one_separator() {\n    \
assert_eq!(rk_lane_fixture::word_count(\"one  two\"), 2);\n}\n";
const HYPOTHESIS_NAME: &str = "cargo test -- hypothesis_doubled_space_is_one_separator";

/// The implementer's failing test at `red`, for the Spec's one criterion.
const RED_TEST: &str = "#[test]\nfn red_ac_1_leading_space_is_not_a_word() {\n    \
assert_eq!(rk_lane_fixture::word_count(\" one\"), 1);\n}\n";
const RED_NAME: &str = "red_ac_1_leading_space_is_not_a_word";

fn key(activity: &str, step: &str, visit: usize) -> Key {
    (activity.to_owned(), step.to_owned(), visit)
}

fn t(s: &str) -> Text {
    Text::new(s)
}

fn fix_spec() -> lane::Spec {
    lane::Spec {
        story: t("word_count counts words separated by any run of whitespace"),
        acceptance_criteria: vec![AcceptanceCriterion {
            id: "AC-1".to_owned(),
            criterion: t("a leading space does not count as a word"),
            gwt: None,
            verification: Some(t(RED_NAME)),
            mutation: None,
        }],
        open_questions: Vec::new(),
        boundary: t("crates/rk-lane-fixture only"),
    }
}

fn self_check(checklist: &str) -> lane::SelfCheck {
    lane::SelfCheck {
        checklist: checklist.to_owned(),
        passed: 1,
        failed: 0,
        notes: t("every item holds"),
    }
}

fn impl_report(tests_added: &[&str], files: &[&str]) -> lane::ImplReport {
    lane::ImplReport {
        status: t("done"),
        files: files.iter().map(|f| (*f).to_owned()).collect(),
        summary: t("word_count splits on any whitespace"),
        tests_added: tests_added.iter().map(|n| (*n).to_owned()).collect(),
    }
}

/// A reviewer's empty finding set: nothing blocks.
fn no_findings() -> Response {
    Response::of(&rk_types::artifact::findings::Findings { items: Vec::new() })
}

/// The fix lane's script: every agent step `bug-standard` dispatches for the fix item, once.
fn fix_script() -> BTreeMap<Key, Response> {
    let root_cause = lane::RootCause {
        cause: t("word_count splits on a single space"),
        contributing_factors: Vec::new(),
        confidence: lane::Confidence::High,
        summary: t("split(' ') counts separators, not words"),
        discriminating_test: HYPOTHESIS_NAME.to_owned(),
        rationale: t("the doubled-space test fails before the fix"),
    };
    BTreeMap::from([
        (
            key("minimal-repro@1", "attempt", 0),
            Response::of(&lane::ReproAttempt {
                status: lane::ReproStatus::Reproduced,
                command: REPRO_NAME.to_owned(),
                observed: t("word_count(\"\") is 1"),
                notes: t("deterministic"),
                rationale: t("the repro test fails"),
            })
            .writing("crates/rk-lane-fixture/tests/repro.rs", REPRO_TEST),
        ),
        (
            key("minimal-repro@1", "reduce", 0),
            Response::of(&lane::ReproCase {
                status: lane::ReproStatus::Reproduced,
                steps: vec![t("call word_count(\"\")")],
                command: REPRO_NAME.to_owned(),
                expected: t("0"),
                observed: t("1"),
                confidence: lane::Confidence::High,
                rationale: t("one call reproduces it"),
            }),
        ),
        (
            key("root-cause@5", "hypothesise", 0),
            Response::of(&lane::HypothesisLog {
                status: lane::HypothesisStatus::Confirmed,
                entries: vec![t("split(' ') yields an empty piece per extra space")],
                discriminating_test: HYPOTHESIS_NAME.to_owned(),
                root_cause,
                summary: t("split(' ') counts separators"),
                rationale: t("the discriminating test fails"),
            })
            .writing(
                "crates/rk-lane-fixture/tests/hypothesis.rs",
                HYPOTHESIS_TEST,
            ),
        ),
        (key("root-cause@5", "adversary", 0), no_findings()),
        (key("fix-spec@2", "draft", 0), Response::of(&fix_spec())),
        (
            key("fix-spec@2", "self-check", 0),
            Response::of(&self_check("fix-scope")),
        ),
        (
            key("tdd-red-green-refactor@3", "red", 0),
            Response::of(&impl_report(
                &[RED_NAME],
                &["crates/rk-lane-fixture/tests/red.rs"],
            ))
            .writing("crates/rk-lane-fixture/tests/red.rs", RED_TEST),
        ),
        (
            key("tdd-red-green-refactor@3", "green", 0),
            Response::of(&impl_report(&[], &["crates/rk-lane-fixture/src/lib.rs"]))
                .writing("crates/rk-lane-fixture/src/lib.rs", FIXED_LIB),
        ),
        (
            key("tdd-red-green-refactor@3", "refactor", 0),
            Response::of(&impl_report(&[], &[])),
        ),
        (key("adversarial-review@4", "review", 0), no_findings()),
        (
            key("debt-logging@1", "extract", 0),
            Response::of(&lane::DebtRegisterEntry {
                items: Vec::new(),
                interest: t("none"),
                remediation: t("none"),
                summary: t("no debt taken on"),
            }),
        ),
        (
            key("lightweight-retro@2", "note", 0),
            Response::of(&lane::Retrospective {
                went_well: vec![t("the repro was one call")],
                to_change: Vec::new(),
                action_items: Vec::new(),
                unassigned_items: Vec::new(),
                summary: t("a small fix"),
            }),
        ),
        (
            key("lightweight-retro@2", "self-check", 0),
            Response::of(&self_check("retro")),
        ),
        (
            key("closure-record@1", "audit", 0),
            Response::of(&self_check("closure")),
        ),
    ])
}

/// **The fix lane reaches a kernel-made content checkpoint commit on `rk-run/<item>`** (D21-41
/// (b)), and the user's checkout is unchanged (AC-518).
#[test]
fn fix_lane_reaches_a_landed_content_commit_under_fake_harness() {
    let repo = rig::fixture();
    let root = repo.path();
    let before = rig::checkout_state(root);
    let walk = walk_fix_lane(root);

    // D21-41 (b): a kernel-made content commit under the crate's `src/`, and the tree differs.
    let src = format!("{}/src", rig::CRATE_DIR);
    let fork = rig::fork_point(root, FIX_ITEM);
    let run_ref = crate::lane_e2e::run_ref(FIX_ITEM);
    assert!(
        !rig::commits_touching(root, FIX_ITEM, &src).is_empty(),
        "no commit on {fork}..rk-run/{FIX_ITEM} touches `{src}`"
    );
    assert_eq!(
        crate::lane_e2e::git_rc(root, &["diff", "--quiet", &fork, &run_ref, "--", &src]),
        Some(1),
        "rk-run/{FIX_ITEM}'s tree does not differ from the fork point under `{src}`"
    );
    let landed = rig::blob_at(root, &run_ref, &format!("{src}/lib.rs")).expect("lib.rs is landed");
    assert_eq!(
        landed,
        FIXED_LIB.as_bytes(),
        "the landed lib.rs is the implementer's fix"
    );

    // D21-44: the run-owned worktree is under `.redkiln/worktrees/`, never under the common dir.
    let trees = walk.trees;
    let expected = root.join(".redkiln").join("worktrees").join(FIX_ITEM);
    let canonical =
        |p: &std::path::Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    assert!(
        trees.iter().any(|t| canonical(t) == canonical(&expected)),
        "no worktree at {}; git knows {trees:?}",
        expected.display()
    );
    let common = canonical(&crate::common_dir(root));
    assert!(
        trees.iter().all(|t| !canonical(t).starts_with(&common)),
        "a worktree lives under the common dir: {trees:?}"
    );

    // AC-518: the fixture's own checkout is unchanged.
    let after = rig::checkout_state(root);
    assert_eq!(after.0, before.0, "HEAD moved");
    assert_eq!(after.1, "", "`git status --porcelain` is not empty");
    assert_eq!(after.2, Some(0), "`git diff-index --quiet HEAD` failed");

    // s8g §8g.3: before verify's first step (at the mark that enters verify), every Spec AC
    // carries a verification.
    let entering_verify =
        rig::first_mark_of(root, FIX_ITEM, "verify").expect("the lane marked its entry to verify");
    let acs = rig::spec_verifications(root, FIX_ITEM, &entering_verify);
    assert!(
        !acs.is_empty(),
        "the fix lane's Spec has no acceptance criteria"
    );
    assert!(
        acs.iter().all(|(_, v)| !v.trim().is_empty()),
        "an AC of the fix Spec lacks a verification: {acs:?}"
    );
}

/// Walk the fix lane to `closed` under [`fix_script`], asserting the dispatched set equals the
/// scripted set (no unscripted dispatch, no unconsumed response).
fn walk_fix_lane(root: &std::path::Path) -> rig::Walk {
    let harness = rig::ScriptedLane::new(root, FIX_ITEM, fix_script());
    let walk = rig::walk_to_close(root, FIX_ITEM, &harness, 16);
    assert!(
        harness.unscripted().is_empty(),
        "unscripted dispatches: {:?}",
        harness.unscripted()
    );
    assert!(
        harness.unconsumed().is_empty(),
        "scripted responses no dispatch reached: {:?}",
        harness.unconsumed()
    );
    walk
}
