//! **`run`'s refusal paths** — §7d.5 steps (1)–(7), every one of which refuses *before* a worktree
//! exists.
//!
//! None of these needs a harness, a worktree or a resume, which is why the contract sequences them
//! together: they are the largest block of `run` behaviour reachable without step (8).

use std::path::Path;

use crate::support::{PackVariant, pack_min_repo, set_enabled};
use crate::{common_dir, refs, run_init, temp_repo};

/// Spawn `redkiln -C <repo> run <item> [--lane <lane>]` with stdin closed (D-15).
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS — golden constraint 4's 'a stub never replaced' is \
              caught only by driving the real binary (s7d §7d.13's temp-repository rule)"
)]
fn spawn_run(repo: &Path, item: &str, lane: Option<&str>) -> std::process::Output {
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(repo).arg("run").arg(item);
    if let Some(lane) = lane {
        cmd.arg("--lane").arg(lane);
    }
    crate::support::output_bounded(&mut cmd)
}

/// **D18-8 — an empty `[packs] enabled` is a CONFIGURATION outcome, before any lane derivation.**
///
/// §7d.5 step (7), verbatim: *"an empty `[packs] enabled` is exit `3` with \"no process pack
/// enabled\" (**D18-8**) **before any lane derivation** — so the refusal is the same whatever item
/// was named"*.
///
/// # What is an instrument here, and what is a FLOOR
///
/// §7d.9 and §7d.17 name the discriminating assertions: *"the store holds **exactly one row**,
/// `reconcile_started`"*, and *"no `rk-run/*` ref and no `<common-dir>/redkiln/worktrees/`
/// directory"* — because those are step (8)'s acts and their absence is what says step (7)'s refusal
/// came first.
///
/// 🛑 **The `reconcile_started` ROW-COUNT half is OWED to the next cycle, not silently dropped.**
/// §7d.5 step (6) writes that row, and step (6) is the cycle that builds steps (8)-(10); until it
/// does, the store holds nothing and a "exactly one row" assertion here would pass over an EMPTY
/// store — true of a `run` that never opened one. It lands with the row.
///
/// 🛑 **The harness marker count is NOT an instrument and is reported as a floor.** An empty pack set
/// refuses at step (7) before any DAG exists, so **no implementation whatever could have written a
/// marker** — the assertion cannot fail, and an assertion that cannot fail is not evidence. It is
/// left out rather than written down as a passing check.
#[test]
fn run_on_init_only_repo_is_a_config_outcome() {
    let repo = temp_repo();
    let root = repo.path();
    assert_eq!(
        run_init(root).status.code(),
        Some(0),
        "the repository initialises"
    );

    // PREMISE: the scaffold `init` writes carries `enabled = []`, which is the state under test.
    let config = std::fs::read_to_string(root.join(".redkiln").join("config.toml"))
        .expect("the scaffold is readable");
    assert!(
        config.contains("enabled = []"),
        "PREMISE: the scaffold does not carry an empty pack set, so this test is not over D18-8's \
         case. config.toml:\n{config}"
    );

    let out = spawn_run(root, "some-item", None);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert_eq!(
        out.status.code(),
        Some(3),
        "an init-only repository must be a CONFIGURATION outcome; stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("no process pack enabled"),
        "the refusal does not say \"no process pack enabled\". Exit 3 alone is what the Slice-1 \
         stub also produced, so the wording is what makes this the absent behaviour. stderr:\n{stderr}"
    );

    // The refusal is the SAME whatever item was named — which is what "before any lane derivation"
    // means operationally. `some-item` above has no record at all; this one is a real record whose
    // kind derives a lane, and it must still refuse for the pack set.
    let with_record = spawn_run(root, "bug-1", Some("fix"));
    assert_eq!(with_record.status.code(), Some(3));
    assert!(
        String::from_utf8_lossy(&with_record.stderr).contains("no process pack enabled"),
        "a named lane and a derivable item changed the refusal, so the pack-set check is NOT \
         before lane derivation"
    );

    // ---- the two discriminating absences: step (8) did not happen. -----------------------------
    assert!(
        refs(root).iter().all(|r| !r.contains("rk-run/")),
        "a refused run created an `rk-run/*` ref, so step (8) ran before step (7) refused: {:?}",
        refs(root)
    );
    assert!(
        !common_dir(root).join("redkiln").join("worktrees").is_dir(),
        "a refused run created the run-owned worktrees directory, so step (8) ran before step (7) \
         refused"
    );
}

/// **ADR-0128 decision 2 — the lane is derived from the item, or refused. Never defaulted.**
///
/// §7d.14's reds: *"by a kindless item running under a default lane, by a disagreeing `--lane`
/// accepted, or by a derivable kind not deriving."*
#[test]
fn adr0128_lane_derived_from_item_or_refused() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();

    // A derivable kind DERIVES — it gets past the lane and reaches the step this cycle has not
    // built, which is a DIFFERENT outcome from the lane's refusal.
    //
    // 🛑 ASSERTED POSITIVELY, and it was not at first. The original form here asserted only that
    // stderr did NOT say "no lane for" — and that absence was satisfied by a completely unrelated
    // failure: a `PackExtras` parse error (a root key written below a table header) refused every
    // one of these runs, and the test passed. An absence that any other failure can satisfy is not
    // evidence the thing under test happened.
    for item in ["bug-1", "story-1"] {
        let out = spawn_run(root, item, None);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            stderr.contains("reaches step (8)"),
            "`{item}` carries a derivable kind, so the run must get PAST the lane and the pack and \
             reach step (8). It did not; stderr:\n{stderr}"
        );
    }

    // A kindless item with no `--lane` is REFUSED, naming the item and the two admissible values.
    let kindless = spawn_run(root, "kindless-1", None);
    let stderr = String::from_utf8_lossy(&kindless.stderr);
    assert_eq!(kindless.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        stderr.contains("kindless-1") && stderr.contains("fix") && stderr.contains("story"),
        "the refusal does not name the item and the two admissible values. ADR-0128 forbids a \
         default in terms: \"an item whose lane is neither given nor derivable is refused, never \
         run under `fix`\". stderr:\n{stderr}"
    );

    // A DISAGREEING `--lane` is refused — "never resolved in favour of either".
    let disagree = spawn_run(root, "bug-1", Some("story"));
    let stderr = String::from_utf8_lossy(&disagree.stderr);
    assert_eq!(disagree.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        stderr.contains("disagrees"),
        "a `--lane` disagreeing with the derived kind was not refused as a disagreement; \
         stderr:\n{stderr}"
    );

    // And RT-50's neighbouring refusal, which the fixtures deliberately separate from the lane's:
    // an item with a kind but NO stage.
    let stageless = spawn_run(root, "stageless-1", None);
    let stderr = String::from_utf8_lossy(&stageless.stderr);
    assert_eq!(stageless.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        stderr.contains("no stage on the item record") && stderr.contains("stageless-1"),
        "RT-50's refusal does not name the item and its reason; stderr:\n{stderr}"
    );
}

/// **ADR-0128 decision 3 — the lane names the pack, and a pack not enabled is refused by name.**
#[test]
fn adr0128_lane_maps_to_pack_fix_bug_standard_story_story() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();

    // The two-row map, read off the filesystem rather than asserted about the code: the fixture
    // installs the SAME pack bytes under both lane names, so a run that reached a pack at all
    // reached the one its lane names.
    for name in ["bug-standard.toml", "story.toml"] {
        assert!(
            root.join(".redkiln").join("packs").join(name).is_file(),
            "PREMISE: the fixture does not install `{name}`"
        );
    }

    // The not-enabled arm: remove `story` and run the STORY item. The bug item must still get past
    // its pack check, or the refusal below would be true of a configuration that enabled nothing.
    set_enabled(root, &["bug-standard"]);

    let refused = spawn_run(root, "story-1", None);
    let stderr = String::from_utf8_lossy(&refused.stderr);
    assert_eq!(refused.status.code(), Some(3), "stderr:\n{stderr}");
    assert!(
        stderr.contains("story") && stderr.contains("enabled"),
        "the refusal does not NAME the pack that is not enabled (ADR-0128 decision 3); \
         stderr:\n{stderr}"
    );

    // The control: with `story` removed, the BUG item's pack is still enabled and its run does not
    // refuse for the pack set.
    let bug = spawn_run(root, "bug-1", None);
    let bug_stderr = String::from_utf8_lossy(&bug.stderr);
    assert!(
        bug_stderr.contains("reaches step (8)"),
        "removing `story` from the enabled set also stopped the BUG item, so the refusal above is \
         not about the pack THE LANE NAMES. Asserted positively rather than as an absence: an \
         absence here is satisfied by any unrelated failure; stderr:\n{bug_stderr}"
    );

    // Nothing above created a run-owned worktree: every one of these refusals precedes step (8).
    assert!(
        !common_dir(root).join("redkiln").join("worktrees").is_dir(),
        "a lane refusal created the run-owned worktrees directory"
    );
}
