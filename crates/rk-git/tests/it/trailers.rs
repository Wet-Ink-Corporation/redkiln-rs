//! `trailer_roundtrip_matches_interpret_trailers` — the second red test s6 §6.3 names.
//!
//! Over ADR-0105 clause 3's **named fixture matrix**: an existing trailer block, no trailing
//! newline, a `---` divider, CRLF, and a duplicate token. The stage file is explicit about which
//! case carries the weight: *"Red on the duplicate-token case specifically: git's default
//! `addIfDifferentNeighbor` silently drops a repeated identical `Gate:` trailer, so a wrapper that
//! omits `--if-exists=add` must fail this test. That case is the discriminating one; the other four
//! pass either way."*
//!
//! The wrapper applies the flag itself, so the mutant is to strip it from `GitExec::config_prelude`
//! and from the `interpret-trailers` argv — and the recorded result of running that mutant is in
//! PROGRESS §6.

use std::path::Path;

use rk_git::git_exec::{GitExec, GitOp};
use rk_git::path::WorktreePath;
use rk_git::trailer::grammar::{Trailer, TrailerKey};
use rk_git::trailer::write;
use rk_git::version;

/// ADR-0105 clause 3's five named cases, as exact bytes.
fn matrix() -> Vec<(&'static str, &'static [u8])> {
    vec![
        ("existing_block", b"subject\n\nbody\n\nStory: demo/s-1\n"),
        ("no_trailing_newline", b"subject\n\nbody\n\nStory: demo/s-1"),
        ("divider", b"subject\n\nbody\n---\nnot a trailer\n"),
        ("crlf", b"subject\r\n\r\nbody\r\n\r\nStory: demo/s-1\r\n"),
        ("duplicate_token", b"subject\n\nbody\n\nGate: pass\n"),
    ]
}

fn exec_for(fx: &crate::support::Fixture) -> GitExec {
    let tc = version::assert_at_startup(fx.path()).expect("git at or above the floor");
    GitExec::new(fx.sub("hooks-empty"), tc).expect("wrapper")
}

/// The wrapper's `interpret-trailers` must produce exactly what the oracle produces, over all five
/// cases — and the wrapper must be the thing supplying `--if-exists=add`, not the caller.
#[test]
fn trailer_roundtrip_matches_interpret_trailers() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let cwd = WorktreePath::new(fx.path()).expect("fixture path normalises");
    let trailer = Trailer {
        key: TrailerKey::Gate,
        value: "pass".into(),
    };

    for (case, input) in matrix() {
        // The oracle: git itself, with the flag, exactly as ADR-0105 clause 3 requires.
        let oracle_file = fx.sub(&format!("oracle_{case}"));
        std::fs::write(&oracle_file, input).expect("write oracle fixture");
        fx.git(
            fx.path(),
            &[
                "interpret-trailers",
                "--in-place",
                "--if-exists=add",
                "--trailer",
                "Gate: pass",
                oracle_file.to_str().expect("utf-8"),
            ],
        );
        let expected = std::fs::read(&oracle_file).expect("read oracle back");

        // The candidate: the same operation through the wrapper.
        let cand_file = fx.sub(&format!("cand_{case}"));
        std::fs::write(&cand_file, input).expect("write candidate fixture");
        write::interpret_trailers_in_place(&exec, &cwd, &cand_file, &trailer)
            .unwrap_or_else(|e| panic!("{case}: wrapper refused: {e:?}"));
        let got = std::fs::read(&cand_file).expect("read candidate back");

        assert_eq!(
            got,
            expected,
            "case `{case}` diverged.\n  wrapper: {:?}\n  oracle : {:?}",
            String::from_utf8_lossy(&got),
            String::from_utf8_lossy(&expected)
        );
    }
}

/// The discriminating case, stated on its own so a failure names it.
///
/// Without `--if-exists=add` git leaves ONE `Gate: pass` where the input already had one. Tier-1
/// state would be silently dropped and the commit would report success.
#[test]
fn a_repeated_identical_trailer_is_appended_not_dropped() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let cwd = WorktreePath::new(fx.path()).expect("normalises");
    let file = fx.sub("dup");
    std::fs::write(&file, b"subject\n\nbody\n\nGate: pass\n").expect("write");

    write::interpret_trailers_in_place(
        &exec,
        &cwd,
        &file,
        &Trailer {
            key: TrailerKey::Gate,
            value: "pass".into(),
        },
    )
    .expect("wrapper");

    let text = String::from_utf8(std::fs::read(&file).expect("read")).expect("utf-8");
    let n = text
        .lines()
        .filter(|l| l.trim_end() == "Gate: pass")
        .count();
    assert_eq!(
        n, 2,
        "a repeated identical Gate: trailer was DROPPED — the wrapper is not supplying \
         --if-exists=add (ADR-0105 clause 3).\n{text}"
    );
}

/// The prelude is the wrapper's job, asserted directly rather than only through its effect.
#[test]
fn the_wrapper_supplies_the_hooks_and_trailer_prelude() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);

    let commit: Vec<String> = exec
        .config_prelude(GitOp::Commit)
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    assert!(
        commit.iter().any(|a| a.starts_with("core.hooksPath=")),
        "commit carries no core.hooksPath (ADR-0105 clause 2): {commit:?}"
    );
    assert!(
        commit.iter().any(|a| a == "trailer.ifexists=add"),
        "commit carries no trailer.ifexists=add (clause 3): {commit:?}"
    );

    let it: Vec<String> = exec
        .config_prelude(GitOp::InterpretTrailers)
        .iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect();
    assert!(it.iter().any(|a| a == "trailer.ifexists=add"));

    // A read carries neither: the prelude is scoped to what it is for, not sprayed everywhere.
    assert!(
        exec.config_prelude(GitOp::RevParse).is_empty(),
        "a read operation is carrying a write-side prelude"
    );
}

/// AC-162's fault-injection fixture: a hook that rewrites the commit message **must have no
/// effect**, because every commit runs with `core.hooksPath` pointed at an empty directory.
#[test]
fn a_message_rewriting_hook_has_no_effect() {
    let fx = crate::support::Fixture::new();
    let exec = exec_for(&fx);
    let repo = fx.repo_with_one_commit("hooked");
    let cwd = WorktreePath::new(&repo).expect("normalises");

    // Install a commit-msg hook that would rewrite every message.
    let hooks = repo.join(".git/hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    let hook = hooks.join("commit-msg");
    std::fs::write(&hook, "#!/bin/sh\necho 'REWRITTEN BY HOOK' > \"$1\"\n").expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }

    let sha = write::commit_with_trailers(
        &exec,
        &cwd,
        "the real subject",
        &[Trailer {
            key: TrailerKey::Story,
            value: "demo/s-1".into(),
        }],
        true,
    )
    .expect("commit");

    let msg = exec
        .run(GitOp::Log, &cwd, ["log", "-1", "--format=%B", &sha])
        .expect("read the message back")
        .stdout_trimmed();
    assert!(
        msg.contains("the real subject"),
        "the hook rewrote a Redkiln commit — core.hooksPath isolation is not in force.\n{msg}"
    );
    assert!(
        !msg.contains("REWRITTEN BY HOOK"),
        "the hook ran on a Redkiln commit (ADR-0105 clause 2).\n{msg}"
    );
    assert!(
        msg.contains("Story: demo/s-1"),
        "the trailer did not survive into the commit message.\n{msg}"
    );
}

/// Control for the test above: the hook is genuinely capable of firing. Without this, a hook that
/// silently never ran (wrong name, not executable) would make the isolation test pass vacuously.
#[test]
fn control_the_fault_injection_hook_really_fires_without_isolation() {
    let fx = crate::support::Fixture::new();
    let repo = fx.repo_with_one_commit("unhooked");

    let hooks = repo.join(".git/hooks");
    std::fs::create_dir_all(&hooks).expect("mkdir hooks");
    let hook = hooks.join("commit-msg");
    std::fs::write(&hook, "#!/bin/sh\necho 'REWRITTEN BY HOOK' > \"$1\"\n").expect("write hook");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&hook, std::fs::Permissions::from_mode(0o755)).expect("chmod");
    }

    // Deliberately NOT through the wrapper, and without the hooksPath prelude: a bare commit.
    let out = rk_git::git_exec::run_with_env(
        Path::new(&repo.display().to_string()),
        ["commit", "--quiet", "--allow-empty", "-m", "plain subject"],
        [
            ("GIT_CONFIG_NOSYSTEM", "1"),
            ("GIT_AUTHOR_NAME", "rk"),
            ("GIT_COMMITTER_NAME", "rk"),
            ("GIT_AUTHOR_EMAIL", "rk@example.invalid"),
            ("GIT_COMMITTER_EMAIL", "rk@example.invalid"),
        ],
    );
    assert!(out.is_ok(), "the control commit did not run: {out:?}");

    let msg = String::from_utf8_lossy(
        &rk_git::git_exec::run(
            Path::new(&repo.display().to_string()),
            ["log", "-1", "--format=%B"],
        )
        .expect("read back")
        .stdout,
    )
    .to_string();
    assert!(
        msg.contains("REWRITTEN BY HOOK"),
        "the fault-injection hook did NOT fire even without isolation, so the isolation test above \
         asserts nothing. Fix the fixture before trusting it.\n{msg}"
    );
}

// ---------------------------------------------------------------------------------------------
// *(Added 2026-09-28, `s8e` build, package S8E-P4.)* **The widened `Story:` and `Gate:` grammar,
// and the fold over it** — `s8e` §8e.5 and §8e.7; ADR-0183 and ADR-0184, both `proposed`.
//
// Pure: every case is a message body handed to the grammar and a commit list handed to
// `reconcile`, which is the whole of what the two records change in this crate. The eleven names
// are `s8e` §8e.12's `rk-git` rows, each selected by Checkpoint 8e (2) as `trailers::<name>`.
// ---------------------------------------------------------------------------------------------

use rk_git::reconcile::{CommitTrailers, reconcile};
use rk_git::trailer::grammar::{
    GateExpiry, GateState, MalformedReason, ParsedTrailer, StoryStage, parse_body,
};
use rk_types::drift::Drift;

const TREE: &str = "0123456789abcdef0123456789abcdef01234567";

/// One commit carrying `body`'s trailers, parsed the way `read_commits` parses a real one.
fn mark(sha: &str, body: &str) -> CommitTrailers {
    CommitTrailers {
        sha: sha.to_owned(),
        unix_time: 0,
        subject: "s".to_owned(),
        trailers: parse_body(body),
    }
}

/// The latest stage `reconcile` folds for `project/slug`, if any.
fn latest(commits: &[CommitTrailers], project: &str, slug: &str) -> Option<StoryStage> {
    let (state, _) = reconcile(commits);
    state
        .projects
        .get(project)
        .and_then(|p| p.stages.get(slug))
        .map(|m| m.stage.clone())
}

fn stage(s: &str) -> Option<StoryStage> {
    Some(StoryStage::Stage(s.to_owned()))
}

/// **§8e.5's grammar: two tokens are a stage-bearing mark.** Red at entry because `classify`
/// called `split_qualified` on the whole value, so the slug absorbed the stage — ADR-0155 option
/// 2's failure mode, *"That it type-checks is what makes it dangerous"*.
#[test]
fn story_value_with_a_stage_parses_as_a_stage_bearing_mark() {
    let parsed = parse_body("Story: redkiln-rs/rk-x analysis\n");
    let [
        ParsedTrailer::Story {
            project,
            slug,
            stage: got,
        },
    ] = parsed.as_slice()
    else {
        panic!("not one Story mark: {parsed:?}");
    };
    assert_eq!(project, "redkiln-rs");
    assert_eq!(
        slug, "rk-x",
        "the slug absorbed the stage: `split_qualified` ran over the whole value"
    );
    assert_eq!(*got, stage("analysis"), "the stage token was not read");

    // The renderer round-trips it, so a producer and this parser agree by construction.
    let rendered = parsed[0]
        .to_trailer()
        .expect("a stage-bearing mark renders");
    assert_eq!(rendered.to_line(), "Story: redkiln-rs/rk-x analysis");
}

/// **The legacy stage-less mark (D21-42 8d's default)** parses as membership with no stage, and
/// folds to no latest stage, so the seed falls back to the item record. Red at entry because no
/// stage field existed; the stage-bearing control below is what shows the field is read at all.
#[test]
fn the_stage_less_legacy_mark_parses_and_carries_no_stage() {
    let legacy = "Story: redkiln-rs/rk-fix-doctor-test-count\n";
    assert_eq!(
        parse_body(legacy),
        vec![ParsedTrailer::Story {
            project: "redkiln-rs".to_owned(),
            slug: "rk-fix-doctor-test-count".to_owned(),
            stage: None,
        }]
    );
    let only_legacy = [mark("a", legacy)];
    let (state, drift) = reconcile(&only_legacy);
    assert!(drift.is_empty(), "a legacy mark is not drift: {drift:?}");
    assert!(
        state.projects["redkiln-rs"]
            .stories_committed
            .contains("rk-fix-doctor-test-count"),
        "the legacy mark is still membership"
    );
    assert_eq!(
        latest(&only_legacy, "redkiln-rs", "rk-fix-doctor-test-count"),
        None,
        "a stage-less mark must fold to NO stage, so the seed falls back to the item record"
    );
    // CONTROL: the same item with a stage-bearing mark does fold to a stage, so the `None` above
    // is the legacy rule and not a fold that never records one.
    assert_eq!(
        latest(
            &[mark(
                "b",
                "Story: redkiln-rs/rk-fix-doctor-test-count triage\n"
            )],
            "redkiln-rs",
            "rk-fix-doctor-test-count"
        ),
        stage("triage"),
        "no stage field exists: a stage-bearing mark folded to no stage"
    );
}

/// **Three or more tokens are drift (AC-183)**, surfaced with a new `MalformedReason` and never
/// coerced. Red at entry because the value parsed as a story whose slug was `x analysis extra`.
#[test]
fn a_story_value_with_three_tokens_is_malformed_drift() {
    for body in [
        "Story: redkiln-rs/rk-x analysis extra\n",
        "Story: redkiln-rs/rk-x a b c\n",
        // a stage token carrying `/`, and a stage-bearing value with no `project/` form
        "Story: redkiln-rs/rk-x ana/lysis\n",
        "Story: rk-x analysis\n",
    ] {
        let parsed = parse_body(body);
        assert!(
            matches!(
                parsed.as_slice(),
                [ParsedTrailer::Malformed {
                    why: MalformedReason::MalformedStoryValue,
                    ..
                }]
            ),
            "{body:?} parses as a story, not as malformed drift: {parsed:?}"
        );
        let (state, drift) = reconcile(&[mark("a", body)]);
        assert!(state.projects.is_empty(), "{body:?} created state");
        assert!(state.bare_story_ids.is_empty(), "{body:?} became a bare id");
        assert!(
            matches!(drift.as_slice(), [Drift::UnparsableTrailer { key, .. }] if key == "Story"),
            "{body:?} must be exactly one unparsable-trailer drift on the Story key: {drift:?}"
        );
    }
}

/// **§8e.7: the value carries kind, reach and state**, and the state's outcome tokens. Red at entry
/// because six tokens were legacy drift.
#[test]
fn the_widened_gate_value_carries_kind_reach_and_state() {
    let cases = [
        ("opened", GateState::Opened),
        (
            "answered approve",
            GateState::Answered {
                option: "approve".to_owned(),
            },
        ),
        (
            "expired fail",
            GateState::Expired {
                action: GateExpiry::Fail,
            },
        ),
        (
            "expired escalate",
            GateState::Expired {
                action: GateExpiry::Escalate,
            },
        ),
        (
            "expired auto-decide approve",
            GateState::Expired {
                action: GateExpiry::AutoDecide {
                    verdict: "approve".to_owned(),
                },
            },
        ),
        ("voided", GateState::Voided),
    ];
    for (tail, want) in cases {
        let body = format!("Gate: tok-1 {TREE} 1789257600 unknown_facet:criticality 2 {tail}\n");
        let parsed = parse_body(&body);
        let [
            ParsedTrailer::Gate {
                token,
                tree_hash,
                deadline,
                kind,
                reach,
                state,
            },
        ] = parsed.as_slice()
        else {
            panic!("six tokens are legacy drift: {body:?} parsed as {parsed:?}");
        };
        assert_eq!(token, "tok-1");
        assert_eq!(tree_hash, TREE);
        assert_eq!(*deadline, 1_789_257_600);
        assert_eq!(
            kind, "unknown_facet:criticality",
            "the kind was not carried: {body:?}"
        );
        assert_eq!(*reach, 2, "the reach was not carried: {body:?}");
        assert_eq!(*state, want, "the state was not carried: {body:?}");
        let rendered = parsed[0]
            .to_trailer()
            .expect("a widened gate value renders");
        assert_eq!(
            format!("{}\n", rendered.to_line()),
            body,
            "the renderer round-trips"
        );
    }

    // The outcome tokens are exactly the state's: an `answered` with no option, an `opened` with
    // one, and an unknown state are each malformed, never read as some other state.
    for bad in [
        "answered",
        "opened approve",
        "voided x",
        "expired",
        "expired auto-decide",
        "expired later",
        "closed",
    ] {
        let body = format!("Gate: tok-1 {TREE} 1789257600 human:review 1 {bad}\n");
        let parsed = parse_body(&body);
        assert!(
            matches!(parsed.as_slice(), [ParsedTrailer::Malformed { .. }]),
            "{body:?} was accepted: {parsed:?}"
        );
    }
}

/// **The triple is legacy drift under the widened grammar** — counted, surfaced with its offending
/// text, never a gate. Red at entry because the triple was the valid value.
#[test]
fn a_gate_value_without_kind_reach_and_state_is_legacy_drift() {
    let body = format!("Gate: tok-1 {TREE} 1789257600\n");
    assert!(
        matches!(
            parse_body(&body).as_slice(),
            [ParsedTrailer::Malformed {
                why: MalformedReason::LegacyGateValue,
                ..
            }]
        ),
        "the triple is valid today: {:?}",
        parse_body(&body)
    );
    let (state, drift) = reconcile(&[mark("a", &body)]);
    assert!(state.gates.is_empty(), "the triple became a gate record");
    match drift.as_slice() {
        [Drift::LegacyGateTrailer { offending, .. }] => {
            assert_eq!(offending.as_str(), format!("tok-1 {TREE} 1789257600"));
        }
        other => panic!("expected one legacy-gate drift: {other:?}"),
    }
}

/// **§8e.5's fold: the latest stage mark per `(project, slug)`, oldest first.** Red at entry
/// because the fold kept no per-item stage.
#[test]
fn reconcile_folds_the_latest_stage_mark_per_item() {
    let commits = [
        mark("a", "Story: redkiln-rs/rk-x triage\n"),
        mark("b", "Story: redkiln-rs/rk-y implement\n"),
        mark("c", "Story: redkiln-rs/rk-x analysis\n"),
        mark("d", "Story: other/rk-x review\n"),
    ];
    let (state, drift) = reconcile(&commits);
    assert!(
        drift.is_empty(),
        "distinct stage marks are not drift: {drift:?}"
    );
    let p = &state.projects["redkiln-rs"];
    assert_eq!(
        p.stages
            .get("rk-x")
            .map(|m| (m.stage.clone(), m.commit.as_str())),
        Some((StoryStage::Stage("analysis".to_owned()), "c")),
        "no per-item stage, or not the LATEST one: {:?}",
        p.stages
    );
    assert_eq!(
        p.stages.get("rk-y").map(|m| m.stage.clone()),
        Some(StoryStage::Stage("implement".to_owned()))
    );
    // The key is (project, slug): another project's mark for the same slug does not move it.
    assert_eq!(
        latest(&commits, "other", "rk-x"),
        stage("review"),
        "the project half of the key"
    );
    // Membership is kept alongside it.
    assert_eq!(p.stories_committed.len(), 2);
}

/// **A repeated IDENTICAL `(project, slug, stage)` stays `DuplicateStoryTrailer` drift**, and moves
/// nothing; distinct stages for one slug are not duplicates. Red at entry because duplicates were
/// read per slug alone.
#[test]
fn a_repeated_identical_stage_mark_is_duplicate_story_drift() {
    let commits = [
        mark("a", "Story: redkiln-rs/rk-x triage\n"),
        mark("b", "Story: redkiln-rs/rk-x analysis\n"),
        mark("c", "Story: redkiln-rs/rk-x triage\n"),
    ];
    let (state, drift) = reconcile(&commits);
    let p = &state.projects["redkiln-rs"];
    assert_eq!(
        p.stories_committed.iter().collect::<Vec<_>>(),
        vec!["rk-x"],
        "the slug is `rk-x`, once — a slug-only duplicate rule cannot see the stage"
    );
    match drift.as_slice() {
        [
            Drift::DuplicateStoryTrailer {
                project,
                slug,
                commit,
            },
        ] => {
            assert_eq!(
                (project.as_str(), slug.as_str(), commit.as_str()),
                ("redkiln-rs", "rk-x", "c")
            );
        }
        other => panic!("expected exactly one duplicate, on the third mark: {other:?}"),
    }
    assert_eq!(
        p.stages
            .get("rk-x")
            .map(|m| (m.stage.clone(), m.commit.as_str())),
        Some((StoryStage::Stage("analysis".to_owned()), "b")),
        "a duplicate is counted, not a transition: it must not move the latest stage"
    );
    // CONTROL: two identical legacy marks are still the duplicate they always were.
    let (_, legacy_drift) = reconcile(&[
        mark("a", "Story: redkiln-rs/rk-x\n"),
        mark("b", "Story: redkiln-rs/rk-x\n"),
    ]);
    assert_eq!(legacy_drift.len(), 1, "{legacy_drift:?}");
}

/// **A stage-less mark never resets a stage a later-or-earlier stage-bearing mark set** (§8e.5).
/// Red at entry because there was no stage to keep.
#[test]
fn a_legacy_mark_after_a_stage_mark_does_not_reset_the_stage() {
    let after = [
        mark("a", "Story: redkiln-rs/rk-x analysis\n"),
        mark("b", "Story: redkiln-rs/rk-x\n"),
    ];
    let (_, drift) = reconcile(&after);
    assert!(
        drift.is_empty(),
        "a legacy mark beside a stage mark is not a duplicate: {drift:?}"
    );
    assert_eq!(
        latest(&after, "redkiln-rs", "rk-x"),
        stage("analysis"),
        "no stage to keep, or the legacy mark reset it"
    );
    let before = [
        mark("a", "Story: redkiln-rs/rk-y\n"),
        mark("b", "Story: redkiln-rs/rk-y triage\n"),
    ];
    assert_eq!(latest(&before, "redkiln-rs", "rk-y"), stage("triage"));
}

/// **N9: `closed` is the lane's end, not a stage** — reserved, so a pack cannot declare it. Red at
/// entry because `closed` parsed as part of the slug, and with a naive two-token split it would
/// parse as a stage named `closed`.
#[test]
fn the_closed_mark_parses_as_the_lanes_end() {
    let parsed = parse_body("Story: redkiln-rs/rk-x closed\n");
    assert_eq!(
        parsed,
        vec![ParsedTrailer::Story {
            project: "redkiln-rs".to_owned(),
            slug: "rk-x".to_owned(),
            stage: Some(StoryStage::Closed),
        }],
        "`closed` parsed as a stage (or into the slug), not as the lane's end"
    );
    let commits = [
        mark("a", "Story: redkiln-rs/rk-x review\n"),
        mark("b", "Story: redkiln-rs/rk-x closed\n"),
    ];
    assert_eq!(
        latest(&commits, "redkiln-rs", "rk-x"),
        Some(StoryStage::Closed),
        "the fold must read `closed` as the item's latest state"
    );
    // The renderer never emits a stage NAMED `closed`: it does not round-trip, so a producer
    // refuses it at the mint.
    let as_stage = ParsedTrailer::Story {
        project: "redkiln-rs".to_owned(),
        slug: "rk-x".to_owned(),
        stage: stage("closed"),
    };
    assert!(
        as_stage.to_trailer().is_none(),
        "a stage named `closed` rendered, so it would read back as the lane's end"
    );
}

/// **N6: `reconcile` folds a gate's latest state per token**, keeping every transition. Red at
/// entry because no state token existed.
#[test]
fn reconcile_folds_a_gates_latest_state_per_token() {
    let g = |token: &str, tail: &str| {
        format!("Gate: {token} {TREE} 1789257600 human:review 1 {tail}\n")
    };
    let commits = [
        mark("a", &g("tok-1", "opened")),
        mark("b", &g("tok-2", "opened")),
        mark("c", &g("tok-1", "answered approve")),
        mark("d", &g("tok-3", "opened")),
        mark("e", &g("tok-3", "expired fail")),
        mark("f", &g("tok-4", "opened")),
        mark("g", &g("tok-4", "voided")),
    ];
    let (state, drift) = reconcile(&commits);
    assert!(drift.is_empty(), "{drift:?}");
    let got = |t: &str| {
        state
            .gates
            .get(t)
            .map(|r| (r.state.clone(), r.commit.as_str()))
    };
    assert_eq!(
        got("tok-1"),
        Some((
            GateState::Answered {
                option: "approve".to_owned()
            },
            "c"
        )),
        "no state token: an answered gate folded as {:?}",
        state.gates.get("tok-1")
    );
    assert_eq!(got("tok-2"), Some((GateState::Opened, "b")));
    assert_eq!(
        got("tok-3"),
        Some((
            GateState::Expired {
                action: GateExpiry::Fail
            },
            "e"
        ))
    );
    assert_eq!(got("tok-4"), Some((GateState::Voided, "g")));
    assert_eq!(
        state.gates["tok-1"].history,
        vec![
            ("a".to_owned(), GateState::Opened),
            (
                "c".to_owned(),
                GateState::Answered {
                    option: "approve".to_owned()
                }
            ),
        ],
        "every transition is kept, oldest first, so a rebuild can emit the opening too"
    );
    assert_eq!(state.gates["tok-1"].kind, "human:review");
    assert_eq!(state.gates["tok-1"].reach, 1);
}

/// **N8: the deadline is integer unix seconds; an RFC 3339 deadline is malformed drift.** Red at
/// entry because any deadline token parsed.
#[test]
fn an_rfc3339_gate_deadline_is_malformed_drift() {
    for body in [
        // The triple the old grammar test fed.
        format!("Gate: tok-1 {TREE} 2026-09-13T00:00:00Z\n"),
        // The widened value with an RFC 3339 deadline.
        format!("Gate: tok-1 {TREE} 2026-09-13T00:00:00Z human:review 1 opened\n"),
        // A signed or fractional deadline is not integer unix seconds either.
        format!("Gate: tok-1 {TREE} -5 human:review 1 opened\n"),
        format!("Gate: tok-1 {TREE} 1789257600.5 human:review 1 opened\n"),
    ] {
        let parsed = parse_body(&body);
        assert!(
            matches!(parsed.as_slice(), [ParsedTrailer::Malformed { .. }]),
            "any deadline token parses: {body:?} became {parsed:?}"
        );
        let (state, drift) = reconcile(&[mark("a", &body)]);
        assert!(state.gates.is_empty(), "{body:?} became a gate record");
        assert_eq!(drift.len(), 1, "{body:?}: {drift:?}");
    }
    // CONTROL: the integer spelling of the same instant parses.
    let ok = format!("Gate: tok-1 {TREE} 1789257600 human:review 1 opened\n");
    assert!(matches!(
        parse_body(&ok).as_slice(),
        [ParsedTrailer::Gate { .. }]
    ));
}
