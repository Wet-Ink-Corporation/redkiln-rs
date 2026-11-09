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
mod improvising;
#[cfg(test)]
mod rig;

use std::collections::BTreeMap;

use rk_cli::exit::Exit;
use rk_types::acceptance::AcceptanceCriterion;
use rk_types::artifact::lane;
use rk_types::text::Text;

use rig::{FIX_ITEM, Key, Response, STORY_ITEM};

/// The fixture crate's planted defect, fixed: what the scripted implementer writes at `green`.
const FIXED_LIB: &str = "//! The lane proofs' fixture crate, fixed by the lane.\n\n\
/// The number of words in `text`, a word being a maximal run of non-whitespace characters.\n\
#[must_use]\npub fn word_count(text: &str) -> usize {\n    text.split_whitespace().count()\n}\n\n\
#[cfg(test)]\nmod tests {\n    use super::word_count;\n\n    #[test]\n    fn counts_single_spaced_words() {\n        \
assert_eq!(word_count(\"one two three\"), 3);\n    }\n}\n";

/// *(Rewritten 2026-11-08, s8 repro-pair route; ADR-0204, proposed; plan Decision D9.)* The
/// investigator's `command` is prose of the 2026-11-08 relaunch's shape, and it writes no file: the
/// repro the lane runs is the fix item's committed pair ([`rig::REPRO_SOURCE`]), placed by
/// `minimal-repro@2`'s `place`. ~~The reproduction the investigator writes, and its
/// discriminating test, each red over the defect and green after the fix.~~
const AGENT_PROSE: &str = "Append the repro (crates/rk-lane-fixture/tests/it/words.rs) and run \
`cargo test -p rk-lane-fixture --test it -- words::repro_empty_text_has_no_words --exact`";

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

/// The story implementer's failing test at `red`, for the story Spec's one criterion.
const STORY_RED_TEST: &str = "#[test]\nfn red_story_ac_1_a_tab_separates_words() {\n    \
assert_eq!(rk_lane_fixture::word_count(\"one\\ttwo\"), 2);\n}\n";
const STORY_RED_NAME: &str = "red_story_ac_1_a_tab_separates_words";
const STORY_RED_PATH: &str = "crates/rk-lane-fixture/tests/story.rs";
/// The page the story's `docs-update` writes.
const STORY_DOC: &str = "crates/rk-lane-fixture/README.md";
const STORY_DOC_TEXT: &str = "# rk-lane-fixture\n\n`word_count` counts words separated by any run \
of whitespace.\n";

fn story_criteria() -> Vec<AcceptanceCriterion> {
    vec![AcceptanceCriterion {
        id: "AC-1".to_owned(),
        criterion: t("a tab separates two words"),
        gwt: None,
        verification: Some(t(STORY_RED_NAME)),
        mutation: None,
    }]
}

fn story_spec() -> lane::Spec {
    lane::Spec {
        story: t("as a writer I count words separated by any run of whitespace"),
        acceptance_criteria: story_criteria(),
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
        discriminating_test: AGENT_PROSE.to_owned(),
        rationale: t("the placed repro fails before the fix"),
    };
    BTreeMap::from([
        (
            key("minimal-repro@2", "attempt", 0),
            Response::of(&lane::ReproAttempt {
                status: lane::ReproStatus::Reproduced,
                command: AGENT_PROSE.to_owned(),
                observed: t("word_count(\"\") is 1"),
                notes: t("deterministic"),
                rationale: t("the placed repro fails"),
            }),
        ),
        (
            key("minimal-repro@2", "reduce", 0),
            Response::of(&lane::ReproCase {
                status: lane::ReproStatus::Reproduced,
                steps: vec![t("call word_count(\"\")")],
                command: AGENT_PROSE.to_owned(),
                expected: t("0"),
                observed: t("1"),
                confidence: lane::Confidence::High,
                rationale: t("one call reproduces it"),
            }),
        ),
        (
            key("root-cause@6", "hypothesise", 0),
            Response::of(&lane::HypothesisLog {
                status: lane::HypothesisStatus::Confirmed,
                entries: vec![t("split(' ') yields an empty piece per extra space")],
                discriminating_test: AGENT_PROSE.to_owned(),
                root_cause,
                summary: t("split(' ') counts separators"),
                rationale: t("the placed repro fails"),
            }),
        ),
        (key("root-cause@6", "adversary", 0), no_findings()),
        (key("fix-spec@2", "draft", 0), Response::of(&fix_spec())),
        (
            key("fix-spec@2", "self-check", 0),
            Response::of(&self_check("fix-scope")),
        ),
        // *(dated 2026-11-08; ADR-0205, proposed; plan test 20, Decision D9.)* A bug with a placed
        // repro implements through `repro-green-refactor@1`: `green` lands the fix and the AC's
        // named test together. Were `tdd-red-green-refactor@3`'s `red` (the test), `green` (the
        // fix) and `refactor`.
        (
            key("repro-green-refactor@1", "green", 0),
            repro_green_landing_the_fix(),
        ),
        (
            key("repro-green-refactor@1", "refactor", 0),
            Response::of(&impl_report(&[], &[])),
        ),
        (key("adversarial-review@4", "review", 0), no_findings()),
    ])
    .into_iter()
    .chain(closeout_script())
    .collect()
}

/// `repro-green-refactor@1`'s `green` as its `task` asks: the fix, and the test the Spec's one
/// criterion names by its verification, listed in `tests_added`.
fn repro_green_landing_the_fix() -> Response {
    Response::of(&impl_report(
        &[RED_NAME],
        &[
            "crates/rk-lane-fixture/src/lib.rs",
            "crates/rk-lane-fixture/tests/red.rs",
        ],
    ))
    .writing("crates/rk-lane-fixture/src/lib.rs", FIXED_LIB)
    .writing("crates/rk-lane-fixture/tests/red.rs", RED_TEST)
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
    let expected = crate::run_tree(root, FIX_ITEM).path;
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

/// The lane's `Story:` stage tokens on `<fork>..rk-run/<item>` equal `pack`'s non-forbidden
/// `stage_order` then `closed`, as a whole list (set and order), and the list includes at least
/// one stage the item journalled `stage_empty` for (D21-42 7).
fn assert_trailers_every_non_forbidden_stage(root: &std::path::Path, item: &str, pack: &str) {
    let trailed = rig::stages_since_fork(root, item);
    let mut expected = rig::declared_lane(root, pack);
    expected.push("closed".to_owned());
    assert_eq!(
        trailed, expected,
        "the `Story:` stage tokens on rk-run/{item} are not `{pack}`'s lane then `closed`"
    );
    let empty: Vec<String> = rig::item_rows(root, item, "stage_empty")
        .iter()
        .filter_map(|p| p.get("stage").and_then(serde_json::Value::as_str))
        .map(str::to_owned)
        .collect();
    assert!(
        empty.iter().any(|s| trailed.contains(s)),
        "PREMISE: the {item} lane crossed no `stage_empty` stage (journalled {empty:?}), so it \
         cannot judge a skipped trailer"
    );
}

/// **Every non-forbidden `bug-standard` stage leaves its `Story:` mark, in order, then `closed`**
/// (D21-43 N9), including the `stage_empty` ones.
#[test]
fn fix_lane_trailers_every_non_forbidden_stage_in_order() {
    let repo = rig::fixture();
    let root = repo.path();
    walk_fix_lane(root);
    assert_trailers_every_non_forbidden_stage(root, FIX_ITEM, "bug-standard.toml");
}

/// **The verify stage's own pass names the real Rust adapter** (s8g §8g.4): at least one
/// item-scoped `verify_completed` row is `origin = "pass"`, and every such row's `adapter` is
/// `rk-adapter-toolchain-rust`.
#[test]
fn fix_lane_verify_completed_names_the_rust_adapter() {
    let repo = rig::fixture();
    let root = repo.path();
    walk_fix_lane(root);
    let passes: Vec<rk_types::verify::VerifyCompleted> =
        rig::item_rows(root, FIX_ITEM, "verify_completed")
            .iter()
            .map(|p| {
                rk_types::verify::VerifyCompleted::from_payload(p)
                    .unwrap_or_else(|e| panic!("a verify_completed row reads back: {e}; {p}"))
            })
            .filter(|v| v.origin == "pass")
            .collect();
    assert!(
        !passes.is_empty(),
        "no item-scoped `verify_completed` row with origin = \"pass\""
    );
    let adapters: Vec<&str> = passes.iter().map(|v| v.adapter.as_str()).collect();
    assert!(
        adapters.iter().all(|a| *a == "rk-adapter-toolchain-rust"),
        "a verify pass ran on another adapter: {adapters:?}"
    );
}

/// The last `kind` artifact `item` journalled, as landed on `rk-run/<item>`.
#[cfg(test)]
fn landed_artifact(root: &std::path::Path, item: &str, kind: &str) -> serde_json::Value {
    let write = rig::artifact_writes(root, item, kind)
        .pop()
        .unwrap_or_else(|| panic!("{item} journalled no {kind} write"));
    let path = write
        .get("path")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("the {kind} write names its path"));
    let run_ref = crate::lane_e2e::run_ref(item);
    let blob = rig::blob_at(root, &run_ref, path)
        .unwrap_or_else(|| panic!("the {kind} `{path}` is not landed on {run_ref}"));
    serde_json::from_slice(&blob).unwrap_or_else(|e| panic!("the landed {kind} is JSON: {e}"))
}

/// The `<sha> <subject>` of each commit on `<fork>..rk-run/<item>`, oldest first.
#[cfg(test)]
fn run_subjects(root: &std::path::Path, item: &str) -> Vec<String> {
    let range = format!(
        "{}..{}",
        rig::fork_point(root, item),
        crate::lane_e2e::run_ref(item)
    );
    crate::lane_e2e::git_read(root, &["log", "--reverse", "--format=%H %s", &range])
        .expect("git log over the run range")
        .lines()
        .map(str::to_owned)
        .collect()
}

/// **The fix lane reproduces through the committed pair, never through agent prose** (ADR-0204,
/// proposed; added 2026-11-08, s8 repro-pair route; plan test 28). Every agent `command` and
/// `discriminating_test` in the script is the 2026-11-08 relaunch's kind of prose, and no agent
/// writes a file, so the lane can only reproduce through `place`: (a) `place` commits on
/// `rk-run/<item>` at the step grain, with the pair appended to `tests/it/words.rs`; (b) the
/// placed test ran red, one failed and none passed; (c) the committed `ReproCase.command` is the
/// placement's; (d) verify confirmed it passing; (e) the landed tree keeps the repro.
#[test]
fn fix_lane_reproduces_through_the_committed_pair_not_agent_prose() {
    let repo = rig::fixture();
    let root = repo.path();
    let harness = rig::ScriptedLane::new(root, FIX_ITEM, fix_script());
    rig::walk_to_close(root, FIX_ITEM, &harness, 16);

    // (a) the step-grain commit, and the bytes it placed.
    let prefix = format!("rk step {FIX_ITEM} minimal-repro@2 place ");
    let subjects = run_subjects(root, FIX_ITEM);
    let place = subjects
        .iter()
        .find_map(|l| l.split_once(' ').filter(|(_, s)| s.starts_with(&prefix)))
        .map(|(sha, _)| sha.to_owned())
        .unwrap_or_else(|| panic!("no `{prefix}...` commit on rk-run/{FIX_ITEM}: {subjects:#?}"));
    let placed =
        rig::blob_at(root, &place, rig::REPRO_FILE).expect("words.rs is at place's commit");
    let placed = String::from_utf8(placed).expect("words.rs is UTF-8");
    assert!(
        placed.ends_with(rig::REPRO_SOURCE),
        "place's commit does not end {} with the committed repro:\n{placed}",
        rig::REPRO_FILE
    );

    // (b) the placed test, and only it, ran red: what `confirm` produced, as `attempt` read it.
    let runs = harness.input_of("minimal-repro@2", "attempt", "run");
    let [report] = runs.as_slice() else {
        panic!("`attempt` was dispatched with one confirm report, not {runs:?}");
    };
    let count = |name: &str| report.get(name).and_then(serde_json::Value::as_u64);
    assert_eq!(
        (count("failed"), count("passed")),
        (Some(1), Some(0)),
        "minimal-repro's confirm did not run the placed test red alone: {report}"
    );

    // (c) the committed ReproCase carries the placement's command, not the agent's prose.
    let placement = crate::support::journal_rows(root, FIX_ITEM)
        .into_iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
        .find(|e| e.payload.get("step_id").and_then(serde_json::Value::as_str) == Some("place"))
        .and_then(|e| {
            e.payload
                .pointer("/result/command")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .expect("place committed a ReproPlacement with a command");
    assert_eq!(placement, rig::REPRO_COMMAND, "the placement's command");
    let case = landed_artifact(root, FIX_ITEM, "ReproCase");
    assert_eq!(
        case.get("command").and_then(serde_json::Value::as_str),
        Some(placement.as_str()),
        "the committed ReproCase's command is not the placement's: {case}"
    );

    // (d) verify's confirmation ran the placed repro green.
    let confirmation = landed_artifact(root, FIX_ITEM, "ConfirmationResult");
    assert_eq!(
        confirmation
            .get("status")
            .and_then(serde_json::Value::as_str),
        Some("passing"),
        "verify's ConfirmationResult: {confirmation}"
    );

    // (e) the landed tree keeps the repro.
    let run_ref = crate::lane_e2e::run_ref(FIX_ITEM);
    let landed = rig::blob_at(root, &run_ref, rig::REPRO_FILE).expect("words.rs is landed");
    assert!(
        String::from_utf8_lossy(&landed).ends_with(rig::REPRO_SOURCE),
        "the landed {} lost the repro",
        rig::REPRO_FILE
    );
}

/// **With no repro pair, the fix lane refuses at `place`, by name** (ADR-0204, proposed; added
/// 2026-11-08; plan test 29), through the binary arm so stderr is the operator's: the `run`
/// process exits on the refusal's row (`Unsupported`), its stderr names
/// `.redkiln/intake/rk-lane-fix.repro.toml`, no fixture child is spawned (so no `minimal-repro`
/// agent step read a confirm report), no `place` commit exists, and `words.rs` on `rk-run/<item>`
/// is the fixture's.
#[test]
fn fix_lane_without_a_repro_pair_refuses_at_place_by_name() {
    let repo = rig::binary_fixture_without_repro_pair();
    let root = repo.path();
    rig::write_child_script(root, Vec::new());
    let mut out = rig::spawn_run(root, FIX_ITEM);
    for _ in 0..8 {
        if out.status.code() != Some(0) {
            break;
        }
        let (_, kind) = rig::awaited_gate(root, FIX_ITEM)
            .unwrap_or_else(|| panic!("the run exited 0 at no gate and without refusing"));
        let (option, attach) = rig::first_option(root, FIX_ITEM, &kind);
        let answered = rig::spawn_answer(root, FIX_ITEM, &option, &attach);
        assert_eq!(
            answered.status.code(),
            Some(0),
            "the `{kind}` gate's answer"
        );
        out = rig::spawn_run(root, FIX_ITEM);
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(Exit::Unsupported.code()),
        "the run did not stop on place's refusal; stderr:\n{stderr}"
    );
    let looked_for = format!(".redkiln/intake/{FIX_ITEM}.repro.toml");
    assert!(
        stderr.contains(&looked_for),
        "the run's stderr does not name `{looked_for}`:\n{stderr}"
    );
    assert_eq!(
        rig::children_spawned(root),
        0,
        "an agent step was dispatched past the refusal"
    );
    let prefix = format!("rk step {FIX_ITEM} minimal-repro@2 place ");
    let subjects = run_subjects(root, FIX_ITEM);
    assert!(
        subjects.iter().all(|l| !l.contains(&prefix)),
        "a refused place committed: {subjects:#?}"
    );

    let fork = rig::fork_point(root, FIX_ITEM);
    let run_ref = crate::lane_e2e::run_ref(FIX_ITEM);
    assert_eq!(
        rig::blob_at(root, &run_ref, rig::REPRO_FILE),
        rig::blob_at(root, &fork, rig::REPRO_FILE),
        "{} moved on {run_ref} with no pair to place",
        rig::REPRO_FILE
    );
}

/// The story lane's script: every agent step `story` dispatches for the story item, once.
fn story_script() -> BTreeMap<Key, Response> {
    BTreeMap::from([
        (
            key("bdd-scenarios@5", "draft", 0),
            Response::of(&lane::GherkinScenarios {
                features: vec![t("Feature: word count")],
                scenarios: vec![t("Scenario: a tab separates words")],
                acceptance_criteria: story_criteria(),
                story: t("as a writer I count words separated by any run of whitespace"),
                open_questions: Vec::new(),
            }),
        ),
        (
            key("bdd-scenarios@5", "self-check", 0),
            Response::of(&self_check("gherkin-quality")),
        ),
        (key("bdd-scenarios@5", "adversary", 0), no_findings()),
        (
            key("bdd-scenarios@5", "to-spec", 0),
            Response::of(&story_spec()),
        ),
        (
            key("t-shirt-sizing@2", "size", 0),
            Response::of(&lane::Estimate {
                method: lane::EstimateMethod::TShirt,
                sizes: vec![t("S")],
                total: t("S"),
                fits_appetite: true,
                confidence: lane::Confidence::High,
                summary: t("one function"),
                rationale: t("a one-line change"),
            }),
        ),
        (
            key("docs-update@5", "write", 0),
            Response::of(&lane::Docs {
                quadrant: lane::DocsQuadrant::Reference,
                files: vec![STORY_DOC.to_owned()],
                summary: t("word_count's contract, stated"),
                stale: Vec::new(),
                rationale: t("the function's contract is reference material"),
            })
            .writing(STORY_DOC, STORY_DOC_TEXT),
        ),
        (
            key("docs-update@5", "self-check", 0),
            Response::of(&self_check("docs-diataxis")),
        ),
        // *(dated 2026-11-08; ADR-0205, proposed; plan test 20, Decision D9.)* Re-keyed
        // `tdd-red-green-refactor@3` -> `@4`; the responses are unchanged.
        (
            key("tdd-red-green-refactor@4", "red", 0),
            Response::of(&impl_report(&[STORY_RED_NAME], &[STORY_RED_PATH]))
                .writing(STORY_RED_PATH, STORY_RED_TEST),
        ),
        (
            key("tdd-red-green-refactor@4", "green", 0),
            Response::of(&impl_report(&[], &["crates/rk-lane-fixture/src/lib.rs"]))
                .writing("crates/rk-lane-fixture/src/lib.rs", FIXED_LIB),
        ),
        (
            key("tdd-red-green-refactor@4", "refactor", 0),
            Response::of(&impl_report(&[], &[])),
        ),
        (key("adversarial-review@4", "review", 0), no_findings()),
    ])
    .into_iter()
    .chain(closeout_script())
    .collect()
}

/// The closeout stage's agent steps, which both lanes select alike.
fn closeout_script() -> BTreeMap<Key, Response> {
    BTreeMap::from([
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
                went_well: vec![t("the change was one call")],
                to_change: Vec::new(),
                action_items: Vec::new(),
                unassigned_items: Vec::new(),
                summary: t("a small change"),
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

/// **Walk `item`'s lane to `closed` under `script`**, asserting the dispatched set IS the scripted
/// set (no unscripted dispatch, no unconsumed response), and that a step visited `n` times
/// committed under at least `n` distinct iteration paths (D21-42 4d), read back from the journal.
fn walk(root: &std::path::Path, item: &str, script: BTreeMap<Key, Response>) -> rig::Walk {
    let harness = rig::ScriptedLane::new(root, item, script);
    let walk = rig::walk_to_close(root, item, &harness, 16);
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
    let committed = rig::committed_paths(root, item);
    for (activity, step, visit) in harness.visited() {
        let paths: Vec<String> = committed
            .iter()
            .filter(|(a, s, _)| *a == activity && *s == step)
            .map(|(_, _, p)| p.to_string())
            .collect();
        let distinct = paths
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        assert!(
            distinct > visit,
            "the dispatch ({activity}, {step}, {visit}) has no committed row under its own \
             iteration path; committed under {paths:?}"
        );
    }
    walk
}

fn walk_story_lane(root: &std::path::Path) -> rig::Walk {
    walk(root, STORY_ITEM, story_script())
}

/// **The story lane's `Spec` is landed and journalled** (D21-42 5b; ADR-0116 decision 3): the
/// item's last `Spec` write names a blob on `rk-run/<item>` whose sha256 is the row's `digest`;
/// the producer is the one the specification stage's own selection record names; every AC carries
/// a verification at the mark entering verify.
#[test]
fn story_lane_spec_is_landed_and_journalled() {
    let repo = rig::fixture();
    let root = repo.path();
    walk_story_lane(root);

    let write = rig::artifact_writes(root, STORY_ITEM, "Spec")
        .pop()
        .expect("the story item journalled a Spec write (scoped by t2_event.item)");
    let path = write
        .get("path")
        .and_then(serde_json::Value::as_str)
        .expect("the Spec write names its path");
    assert!(
        path.starts_with(&format!(".redkiln/artifacts/{STORY_ITEM}/")),
        "the Spec `{path}` is not under the item's managed artifact directory"
    );
    let run_ref = crate::lane_e2e::run_ref(STORY_ITEM);
    let object = format!("{run_ref}:{path}");
    assert_eq!(
        crate::lane_e2e::git_rc(root, &["cat-file", "-e", &object]),
        Some(0),
        "`git cat-file -e {object}` finds no landed Spec"
    );
    let blob = rig::blob_at(root, &run_ref, path).expect("the landed Spec reads");
    assert_eq!(
        write.get("digest").and_then(serde_json::Value::as_str),
        Some(rk_types::digest::sha256(&blob).as_str()),
        "the last Spec write's digest is not the sha256 of the landed blob"
    );

    // The producer, never typed: the activity the Spec's committing row names is one the
    // specification stage's own recorded run bound.
    let selected = rig::selected_at(root, "specification");
    let committed = rig::committed_producers(root, STORY_ITEM, "Spec");
    let last = committed.last().expect("an activity committed the Spec");
    assert!(
        selected.contains(last),
        "the Spec's committing activity `{last}` is not one the specification stage's recorded \
         run bound ({selected:?})"
    );

    let entering_verify = rig::first_mark_of(root, STORY_ITEM, "verify")
        .expect("the lane marked its entry to verify");
    let acs = rig::spec_verifications(root, STORY_ITEM, &entering_verify);
    assert!(!acs.is_empty(), "the story Spec has no acceptance criteria");
    assert!(
        acs.iter().all(|(_, v)| !v.trim().is_empty()),
        "an AC of the story Spec lacks a verification: {acs:?}"
    );
}

/// **Every non-forbidden `story` stage leaves its `Story:` mark, in order, then `closed`**: the
/// whole story lane, not only up to the Spec.
#[test]
fn story_lane_trailers_every_non_forbidden_stage_in_order() {
    let repo = rig::fixture();
    let root = repo.path();
    walk_story_lane(root);
    assert_trailers_every_non_forbidden_stage(root, STORY_ITEM, "story.toml");
}

fn walk_fix_lane(root: &std::path::Path) -> rig::Walk {
    walk(root, FIX_ITEM, fix_script())
}

/// **The human root is attached through the pack question on both lanes** (s8j §8j.2; ADR-0200):
/// the invocation that reaches the root's step-4 gate exits 0 awaiting it; `answer --attach`
/// with that question's first option, which attaches exactly the root kind, applies; and the
/// re-invocation journals the attached root as the kind's write and never re-opens the gate.
#[test]
fn human_root_is_attached_through_the_pack_question_on_both_lanes() {
    let repo = rig::fixture();
    let root = repo.path();
    for (item, kind, script) in [
        (FIX_ITEM, "BugReport", fix_script()),
        (STORY_ITEM, "IntakeRecord", story_script()),
    ] {
        let harness = rig::ScriptedLane::new(root, item, script);
        let gate = format!("human_root:{kind}");
        // triage's `unknown_facet` gate for `change` comes first (D21-43 N3); answer it.
        assert_eq!(rig::run_once(root, item, &harness), Exit::Success);
        let facet = rig::awaited_kind(root, item);
        assert_eq!(facet, "unknown_facet:change", "{item}'s first gate");
        let (option, attach) = rig::first_option(root, item, &facet);
        assert_eq!(rig::answer(root, item, &option, attach), Exit::Success);

        // The invocation that reaches the root stops at its gate and exits 0.
        assert_eq!(
            rig::run_once(root, item, &harness),
            Exit::Success,
            "the invocation stopping at the `{gate}` gate exits 0"
        );
        assert_eq!(
            rig::awaited_kind(root, item),
            gate,
            "{item} awaits its root"
        );
        let (option, attach) = rig::first_option(root, item, &gate);
        assert_eq!(
            attach.iter().map(|a| a.kind.as_str()).collect::<Vec<_>>(),
            vec![kind],
            "the `{gate}` question's first option `{option}` attaches exactly `{kind}`"
        );
        assert_eq!(
            rig::answer(root, item, &option, attach),
            Exit::Success,
            "`answer {item} {option} --attach {kind}=..` applies"
        );

        // The re-invocation reads the attached root and does not ask again.
        assert_eq!(
            rig::run_once(root, item, &harness),
            Exit::Success,
            "the re-invocation after the attach exits 0; unscripted {:?}; journal tail:\n{}",
            harness.unscripted(),
            rig::journal_tail(root, item, 8)
        );
        let opened = crate::support::journal_rows(root, item)
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::GateOpened)
            .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(&gate))
            .count();
        assert_eq!(opened, 1, "the `{gate}` gate re-opened after its attach");
        let expected = rig::root_file(item, kind);
        let writes = rig::artifact_writes(root, item, kind);
        let attached = writes.iter().find(|w| {
            w.get("attached_path")
                .and_then(serde_json::Value::as_str)
                .is_some_and(|p| std::path::Path::new(p) == expected)
        });
        let Some(digest) = attached.and_then(|w| w.get("digest")?.as_str()) else {
            panic!(
                "no `{kind}` write names the attached root {}: {writes:?}",
                expected.display()
            );
        };
        // Present: the re-invocation dispatched a step that declares `artifact:<kind>` with the
        // root BOUND (a missing root never reaches the port; a null binding is not a read).
        let reads = harness.reads_of(kind);
        assert!(
            !reads.is_empty() && reads.iter().all(|(_, v)| !v.is_null()),
            "the re-invocation did not read the attached `{kind}` (digest {digest}) as present; \
             its reads of `artifact:{kind}`: {reads:?}; visited {:?}",
            harness.visited()
        );
    }
}

/// **The multi-process arm** (D21-41 (a); `s8h` §8h.7): the built `redkiln`, with
/// `[adapters] harness_binary` naming the fixture child (`support::fixture_child()`) in its
/// per-dispatch `script` mode and the fixture launch record passed through `s8h`'s one spawn
/// helper. Each `run` stops at a gate, the gate is answered by a `redkiln answer` process, and the
/// next `run` is a new process that continues from the stage-bearing trailer. The gates come in
/// the lane's order and include triage's `unknown_facet` gate, the human-root gate and the §8j.3
/// hold; at least five `run` processes; the last reaches the landing; and the final trailer
/// sequence equals the in-process arm's.
#[test]
fn gate_then_answer_then_second_process_continues_binary_arm() {
    // The in-process arm over its own fixture: the dispatch order the child's script follows, and
    // the gates and trailers the binary arm must reproduce.
    let reference = rig::fixture();
    let harness = rig::ScriptedLane::new(reference.path(), FIX_ITEM, fix_script());
    let in_process = rig::walk_to_close(reference.path(), FIX_ITEM, &harness, 16);
    assert!(
        harness.unscripted().is_empty() && harness.unconsumed().is_empty(),
        "PREMISE: the in-process arm dispatched exactly the script"
    );
    let in_process_stages = rig::stages_since_fork(reference.path(), FIX_ITEM);

    let repo = rig::binary_fixture();
    let root = repo.path();
    let mut script = fix_script();
    let visited = harness.visited();
    let ordered = visited
        .iter()
        .map(|k| {
            script
                .remove(k)
                .unwrap_or_else(|| panic!("PREMISE: {k:?} is scripted once"))
        })
        .collect();
    rig::write_child_script(root, ordered);
    let walk = rig::walk_binary_to_close(root, FIX_ITEM, 16);

    assert!(
        walk.runs >= 5,
        "the lane crossed only {} `run` processes",
        walk.runs
    );
    assert_eq!(
        rig::children_spawned(root),
        visited.len(),
        "one fixture child per scripted dispatch"
    );
    let kinds: Vec<&str> = walk.gates.iter().map(|(k, _)| k.as_str()).collect();
    let reference_kinds: Vec<&str> = in_process.gates.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        kinds, reference_kinds,
        "the binary arm's gates are not the in-process arm's, in order"
    );
    let position = |kind: &str| kinds.iter().position(|k| *k == kind);
    let (facet, human, hold) = (
        position("unknown_facet:change"),
        position("human_root:BugReport"),
        position("hold:specification"),
    );
    assert!(
        matches!((facet, human, hold), (Some(f), Some(h), Some(s)) if f < h && h < s),
        "the gates do not include triage's facet gate, the human root and the hold, in that \
         order: {kinds:?}"
    );
    assert_eq!(
        rig::stages_since_fork(root, FIX_ITEM),
        in_process_stages,
        "the binary arm's trailer sequence is not the in-process arm's"
    );
    let src = format!("{}/src", rig::CRATE_DIR);
    assert!(
        !rig::commits_touching(root, FIX_ITEM, &src).is_empty(),
        "the last `run` process did not reach the landing: no commit touches `{src}`"
    );
}

/// The §8j.3 hold's gate kind on the fix item (`hold_before = "specification"`).
const HOLD: &str = "hold:specification";

/// Walk the fix lane in process to the run that stops at the hold, asserting the stop: the run
/// exits 0 awaiting [`HOLD`], the last `Story:` mark is `analysis`, nothing of `specification`
/// dispatched, and exactly one hold gate is open.
fn stop_at_the_hold(root: &std::path::Path) -> std::sync::Arc<rig::ScriptedLane> {
    let harness = rig::ScriptedLane::new(root, FIX_ITEM, fix_script());
    rig::walk_to_gate(root, FIX_ITEM, &harness, HOLD);
    let stages = rig::story_stages(root, FIX_ITEM);
    assert_eq!(
        stages.last().map(String::as_str),
        Some("analysis"),
        "the lane did not stop at analysis -> specification; marks {stages:?}"
    );
    assert!(
        harness.visited().iter().all(|(a, _, _)| a != "fix-spec@2"),
        "a specification dispatch ran before the hold was answered: {:?}",
        harness.visited()
    );
    assert_eq!(
        rig::gate_tokens(root, FIX_ITEM, HOLD).len(),
        1,
        "one hold gate opens"
    );
    harness
}

/// **§8j.3 (2)-(3): the hold stops the lane at analysis -> specification**: the run that would
/// enter `specification` opens the default question's gate instead and exits 0 (C-01 D-01).
#[test]
fn boundary_hold_stops_the_lane_at_analysis_to_specification() {
    let repo = rig::fixture();
    let root = repo.path();
    stop_at_the_hold(root);
    let registry = rk_cli::processes::load(root).expect("the fixture's questions load");
    assert_eq!(
        registry.question_for(HOLD),
        rk_kernel::workflow::question::DEFAULT_QUESTION,
        "the hold is answered with the default question (ADR-0129 decision 3)"
    );
    // A second run without an answer stays put, and does not open a second gate.
    let again = rig::ScriptedLane::new(root, FIX_ITEM, BTreeMap::new());
    assert_eq!(rig::run_once(root, FIX_ITEM, &again), Exit::Success);
    assert!(again.visited().is_empty(), "an unanswered hold dispatched");
    assert_eq!(rig::gate_tokens(root, FIX_ITEM, HOLD).len(), 1);
    assert_eq!(
        rig::story_stages(root, FIX_ITEM).last().map(String::as_str),
        Some("analysis")
    );
}

/// **§8j.3 (5)-(6): `approve` lets the next `redkiln run` PROCESS enter `specification`**, across
/// the process boundary from the stage-bearing trailer (D21-41 (a)), and the answered hold never
/// re-fires.
#[test]
fn boundary_hold_answered_lets_the_next_process_enter_specification() {
    // The in-process reference walk: the dispatch order after the hold.
    let reference = rig::fixture();
    let full = rig::ScriptedLane::new(reference.path(), FIX_ITEM, fix_script());
    rig::walk_to_close(reference.path(), FIX_ITEM, &full, 16);

    let repo = rig::binary_fixture();
    let root = repo.path();
    let held = stop_at_the_hold(root);
    let token = rig::gate_tokens(root, FIX_ITEM, HOLD)
        .pop()
        .expect("the hold's token");
    let answered = rig::spawn_answer(root, FIX_ITEM, "approve", &[]);
    assert_eq!(
        answered.status.code(),
        Some(0),
        "`redkiln answer {FIX_ITEM} approve` applies; stderr:\n{}",
        String::from_utf8_lossy(&answered.stderr)
    );
    let before = held.visited();
    let mut script = fix_script();
    let ordered = full
        .visited()
        .iter()
        .filter(|k| !before.contains(k))
        .map(|k| script.remove(k).expect("a reference key is scripted"))
        .collect();
    rig::write_child_script(root, ordered);

    let out = rig::spawn_run(root, FIX_ITEM);
    assert_eq!(
        out.status.code(),
        Some(0),
        "the next `redkiln run` process exits 0; stderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stages = rig::story_stages(root, FIX_ITEM);
    let at = |s: &str| stages.iter().position(|x| x == s);
    assert!(
        matches!((at("analysis"), at("specification")), (Some(a), Some(s)) if a < s),
        "the next process did not enter `specification` after `analysis`; marks {stages:?}"
    );
    assert!(
        rig::children_spawned(root) > 0,
        "the next process dispatched nothing in `specification`"
    );
    assert_eq!(
        rig::gate_tokens(root, FIX_ITEM, HOLD),
        vec![token],
        "the answered hold re-fired"
    );
}

/// **§8j.3 (7): `changes-requested` does not advance the lane** (D21-40 decision 2): the next runs
/// exit 0, dispatch nothing, mint no `specification` mark and open no second hold gate.
#[test]
fn boundary_hold_changes_requested_does_not_advance() {
    let repo = rig::fixture();
    let root = repo.path();
    stop_at_the_hold(root);
    assert_eq!(
        rig::answer(root, FIX_ITEM, "changes-requested", Vec::new()),
        Exit::Success,
        "`changes-requested` answers the hold"
    );
    for n in 0..2 {
        let rerun = rig::ScriptedLane::new(root, FIX_ITEM, BTreeMap::new());
        assert_eq!(
            rig::run_once(root, FIX_ITEM, &rerun),
            Exit::Success,
            "rerun {n} exits 0"
        );
        assert!(
            rerun.visited().is_empty(),
            "rerun {n} dispatched {:?}",
            rerun.visited()
        );
    }
    let stages = rig::story_stages(root, FIX_ITEM);
    assert!(
        !stages.iter().any(|s| s == "specification"),
        "`changes-requested` advanced the lane; marks {stages:?}"
    );
    assert_eq!(
        rig::gate_tokens(root, FIX_ITEM, HOLD).len(),
        1,
        "the answered hold re-fired"
    );
}
