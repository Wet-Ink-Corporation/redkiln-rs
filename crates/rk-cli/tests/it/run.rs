//! **`run`'s refusal paths** — §7d.5 steps (1)–(7), every one of which refuses *before* a worktree
//! exists.
//!
//! None of these needs a harness, a worktree or a resume, which is why the contract sequences them
//! together: they are the largest block of `run` behaviour reachable without step (8).

use std::path::Path;

use crate::support::{PackVariant, pack_min_repo, set_enabled};
use crate::{common_dir, refs, run_init, temp_repo};

/// Spawn `redkiln -C <repo> run <item> [--lane <lane>]` with stdin closed (D-15).
pub(crate) fn spawn_run(repo: &Path, item: &str, lane: Option<&str>) -> std::process::Output {
    spawn_run_in_mode(repo, item, lane, "exit:0")
}

/// `spawn_run`, with the fixture child's MODE named.
///
/// # 🛑 THE MODE TRAVELS IN THE CHILD ENVIRONMENT, AND AN ABSENT ONE IS NOT NEUTRAL
///
/// §7d.13: the child's mode *"travels in the CHILD environment the test spawns `redkiln` with
/// (`RK_FIXTURE_MODE`, `RK_FIXTURE_MARKER_DIR`, via `Command::env`, never `set_var`), which the
/// adapter's `child_env` passes through to the child."* `set_var` is banned workspace-wide
/// **including** `crates/rk-cli/tests/it`.
///
/// Leaving it unset is not "no mode": `rk-fixture-child.rs` reads
/// `env::var("RK_FIXTURE_MODE").unwrap_or_else(|_| "hold".to_owned())`, so an absent variable
/// selects a 600 s park with stdout held open. That is what wedged this box on 2026-09-20, and it
/// became reachable from these tests the moment cycle 6 wired step (8): a lane that DERIVES gets
/// past the pack and dispatches.
///
/// `exit:0` is the default here because the lane cases are about **lane derivation**, not about
/// what the child said: the child exits at once, the dispatch fails for want of a stream, and the
/// worktree step (8) created — which is what those cases actually read — is already on disk.
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS — golden constraint 4's 'a stub never replaced' is \
              caught only by driving the real binary (s7d §7d.13's temp-repository rule)"
)]
pub(crate) fn spawn_run_in_mode(
    repo: &Path,
    item: &str,
    lane: Option<&str>,
    mode: &str,
) -> std::process::Output {
    let markers = repo.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C").arg(repo).arg("run").arg(item);
    if let Some(lane) = lane {
        cmd.arg("--lane").arg(lane);
    }
    cmd.env("RK_FIXTURE_MODE", mode)
        .env("RK_FIXTURE_MARKER_DIR", &markers);
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
/// ✅ **The `reconcile_started` ROW-COUNT half LANDED at cycle 7, as this note promised.** It was
/// owed while nothing in either binary appended the row: until then a "exactly one row" assertion
/// would have passed over an EMPTY store — true of a `run` that never opened one, so not evidence.
/// Cycle 7 writes the row (§7d.5 step (6)) because `answer` cannot resolve the run-owned worktree
/// without it, and moved the reconcile to its numbered position **before** step (7) — without that
/// move this refusal returned before reconciling and could not have written any row at all.
///
/// The assertion is therefore made TWICE and the second is the stronger one: one row after the
/// first invocation, **two after the second**. AC-172's text is *"exactly one `reconcile_started`
/// event per invocation"* — a single count of `1` is also true of a binary that writes the row once
/// and then never again, and only the per-invocation count discriminates that.
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

    // ---- AC-172, RT-46, D18-8 as corrected: the refusal that REACHED the reconcile owes one row.
    let after_first = crate::support::store_rows(root);
    assert_eq!(
        after_first.len(),
        1,
        "a refused `run` that reached step (6) writes EXACTLY ONE row. §7d.5: *\"a refused `run` \
         that reached the reconcile (no pack; …) writes exactly one `reconcile_started` row\"*; \
         D18-8 read \"zero store writes\" until its dated correction, and AC-172 is the register's. \
         Rows: {:?}",
        after_first.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert_eq!(
        after_first[0].kind,
        rk_types::event_kind::EventKind::ReconcileStarted,
        "the one row a refused run leaves is `reconcile_started`; found {:?}",
        after_first[0].kind
    );
    // It carries RT-41's two members and NOT RT-47's pair: no worktree was created, and §7d.5 is
    // explicit that the pair goes on only "once (8) has re-created the worktree".
    assert!(
        after_first[0].payload.get("adapter_set").is_some()
            && after_first[0].payload.get("config_digest").is_some(),
        "the row carries neither RT-41 member: {:?}",
        after_first[0].payload
    );
    assert!(
        after_first[0].payload.get("worktree").is_none(),
        "a step-(7) refusal created no worktree, so the row must NOT carry RT-47's pair — a row \
         that does is claiming an ownership nothing established: {:?}",
        after_first[0].payload
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

    // AC-172's actual text is PER INVOCATION. A binary that wrote the row once and never again
    // would satisfy the count of `1` above; only this second reading discriminates it.
    let after_second = crate::support::store_rows(root);
    assert_eq!(
        after_second.len(),
        2,
        "two invocations must leave two `reconcile_started` rows — AC-172 is *\"exactly one \
         `reconcile_started` event per invocation\"*, not one per repository. Rows: {:?}",
        after_second.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert!(
        after_second
            .iter()
            .all(|e| e.kind == rk_types::event_kind::EventKind::ReconcileStarted),
        "a refused run wrote something besides its one `reconcile_started` row: {:?}",
        after_second.iter().map(|e| e.kind).collect::<Vec<_>>()
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
    // 🛑 AND IT IS NOW A STRUCTURAL FACT RATHER THAN A MESSAGE. Until cycle 6 this read
    // `stderr.contains("reaches step (8)")` — the stub's own words, which cycle 6 deleted when it
    // wired step (8). The replacement keys on what step (8) DOES: it re-creates
    // `<common-dir>/redkiln/worktrees/<item>` on `rk-run/<item>` (H-5 / RT-47). No message change
    // can invalidate that, and no lane refusal can produce it.
    for item in ["bug-1", "story-1"] {
        let out = spawn_run(root, item, None);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            common_dir(root)
                .join("redkiln")
                .join("worktrees")
                .join(item)
                .is_dir(),
            "`{item}` carries a derivable kind, so the run must get PAST the lane and the pack and \
             reach step (8)'s run-owned worktree. It did not; stderr:\n{stderr}"
        );
        assert!(
            refs(root)
                .iter()
                .any(|r| r.contains(&format!("rk-run/{item}"))),
            "`{item}` reached step (8)'s worktree but no `rk-run/{item}` ref exists, so the \
             worktree is not the run-owned one H-5 names; refs: {:?}",
            refs(root)
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
        common_dir(root)
            .join("redkiln")
            .join("worktrees")
            .join("bug-1")
            .is_dir(),
        "removing `story` from the enabled set also stopped the BUG item, so the refusal above is \
         not about the pack THE LANE NAMES. Asserted positively rather than as an absence: an \
         absence here is satisfied by any unrelated failure; stderr:\n{bug_stderr}"
    );

    // 🛑 THE REFUSALS CREATED NO WORKTREE, AND THIS IS ASSERTED AS AN EXACT SET.
    //
    // Until cycle 6 this read `!worktrees.is_dir()` — a blanket absence, which was true only while
    // step (8) did not exist. The `bug-1` control above now legitimately creates
    // `worktrees/bug-1`, so the blanket form became false of a CORRECT binary.
    //
    // The narrowing chosen is the exact CONTENTS, not merely "`story-1` is absent": a refused run
    // that created `worktrees/<anything-else>` — a wrong-item worktree, a stray under a mis-derived
    // name — would pass the weaker form and is exactly the defect this assertion exists to catch.
    let mut owned: Vec<String> =
        std::fs::read_dir(common_dir(root).join("redkiln").join("worktrees"))
            .expect("step (8) created the worktrees directory for `bug-1` above")
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
    owned.sort();
    assert_eq!(
        owned,
        ["bug-1"],
        "exactly one run-owned worktree may exist here — `bug-1`'s, from the control above. Every \
         other run in this test is a refusal that precedes step (8), so any other entry is a \
         worktree a refusal created"
    );
}

/// **AC-401 — a second `run` issues NO dispatch for a step the first one committed.**
///
/// §7d.5: *"Two runs against the two-activity `pack-min/` with `FakeHarness::scripted`; after the
/// second: **exactly one `dispatch` row**"* — per activation, the two-activity variant assembling
/// two nodes.
///
/// # 🛑 THE ITEM IS `story-1`, AND THAT IS NOT INTERCHANGEABLE WITH `bug-1`
///
/// `story-item.toml` carries `has_repro = false`, a DECIDED false, so step 2's `Tri::False` arm
/// adds nothing and the required set is the baseline alone. `bug-item.toml` carries
/// `has_repro = true`, which fires the pack's one `require` row for `TestReport` — and the
/// registry says of that kind, in terms, *"It is produced by nothing in this catalog ON
/// PURPOSE"*, so step 4 classifies it `Unsatisfiable` and the selection carries a `gate_opened`
/// row. This test is about memoisation, not gates, and cycle 5 is *"steps 8–10 minus the gate"*.
///
/// # 🛑 RUN 2'S SCRIPT IS EMPTY, AND THAT IS THE DISCRIMINATING CHOICE
///
/// A non-empty script would let a re-dispatch SUCCEED and the test would pass over the very bug it
/// names. With an empty one, any dispatch at all runs the fake dry into
/// `Transport { detail: "script exhausted" }` — so a re-dispatching run cannot return `Success`,
/// and the assertion has somewhere to fail.
#[test]
fn ac401_second_run_issues_no_dispatch_for_memoised_step() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::TwoActivities);
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "story-1");
    let (inputs, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
    let inputs = inputs
        .with_spec(
            "emit-report",
            rk_kernel::replay::RecordedSpec::supplied(
                "emit-report@1",
                crate::support::any_activity_spec(),
            ),
        )
        .with_spec(
            "second-activity",
            rk_kernel::replay::RecordedSpec::supplied(
                "second-activity@1",
                crate::support::any_activity_spec(),
            ),
        );

    // ONE store across both runs — the whole mechanism is that run 2 reads run 1's journal.
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let token = rk_ports::cancel::CancelToken::new();

    // ---- RUN 1: both nodes dispatch and commit. ------------------------------------------------
    let h1 = std::sync::Arc::new(crate::support::ScriptedHarness::scripted(vec![
        Ok(crate::support::impl_report()),
        Ok(crate::support::impl_report()),
    ]));
    let p1 = crate::support::ports(
        std::sync::Arc::clone(&h1) as std::sync::Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let mut s1 = crate::support::no_spec();
    let exit1 = rk_cli::verbs::run::run_with(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        p1,
        &mut s1,
        &token,
    )
    .expect("a sound lock is not a halt");
    assert_eq!(
        exit1,
        rk_cli::exit::Exit::Success,
        "run 1 did not complete ({exit1:?}), so there is no memoised state for run 2 to skip"
    );
    assert_eq!(
        h1.dispatches(),
        2,
        "run 1 reached the port {} times; the two-activity pack assembles two nodes and both must \
         dispatch, or run 2 has nothing to memoise",
        h1.dispatches()
    );

    let rows1 = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("story-1"))
        .expect("the journal reads");
    let dispatches1 = rows1
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::Dispatch)
        .count();
    let committed1 = rows1
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::ActivityCommitted)
        .count();
    assert_eq!(
        (dispatches1, committed1),
        (2, 2),
        "run 1 wrote {dispatches1} dispatch and {committed1} activity_committed rows; both nodes \
         must have committed or the memo below is about nothing"
    );
    let kinds1: Vec<rk_types::event_kind::EventKind> = rows1.iter().map(|e| e.kind).collect();
    let appends1 = trace.steps().len();

    // ---- RUN 2: the SAME inputs, the SAME store, and an EMPTY script. --------------------------
    let h2 = std::sync::Arc::new(crate::support::ScriptedHarness::scripted(Vec::new()));
    let p2 = crate::support::ports(
        std::sync::Arc::clone(&h2) as std::sync::Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let mut s2 = crate::support::no_spec();
    let exit2 = rk_cli::verbs::run::run_with(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        p2,
        &mut s2,
        &token,
    )
    .expect("a sound lock is not a halt");

    // THE PRIMARY INSTRUMENT. A run that re-dispatched would exhaust the empty script into
    // `Transport` and return 10, not 0.
    assert_eq!(
        exit2,
        rk_cli::exit::Exit::Success,
        "run 2 returned {exit2:?}. With an EMPTY script that means it dispatched again: the fold \
         was not seeded from the store, so a committed activation was selected a second time"
    );
    assert_eq!(
        h2.dispatches(),
        0,
        "run 2 reached the port {} times. Every activation the first run committed is memoised; a \
         second dispatch of one is AC-401's whole negative",
        h2.dispatches()
    );

    // THE JOURNAL IS UNCHANGED — not merely "no new dispatch", but no new row of any kind.
    let rows2 = store
        .lock()
        .expect("the fixture store's lock is sound")
        .query(rk_kernel::runtime::journal_query("story-1"))
        .expect("the journal reads");
    let kinds2: Vec<rk_types::event_kind::EventKind> = rows2.iter().map(|e| e.kind).collect();
    assert_eq!(
        kinds2,
        kinds1,
        "run 2 changed the journal. It appended: {:?}",
        &kinds2[kinds1.len().min(kinds2.len())..]
    );

    // AND NOTHING WAS WRITTEN AT ALL, read off the ORDERED trace rather than inferred from a
    // count: a run that appended and then somehow matched the earlier kinds would still be caught.
    let after: Vec<crate::support::Step> = trace.steps().into_iter().skip(appends1).collect();
    assert!(
        after
            .iter()
            .all(|s| matches!(s, crate::support::Step::Query)),
        "run 2 wrote to the store. It is a READ of a memoised journal and nothing else; it did: \
         {after:?}"
    );

    // AND IT DID NOT RE-OPEN THE PLAN. `resume` skips the opening rows only when the journal
    // already carries a `dag_assembled`; a second pair here would mean it re-assembled.
    for kind in [
        rk_types::event_kind::EventKind::DagAssembled,
        rk_types::event_kind::EventKind::StageTransition,
    ] {
        assert_eq!(
            rows2.iter().filter(|e| e.kind == kind).count(),
            1,
            "the journal carries more than one {kind:?} row, so run 2 re-opened a plan the store \
             already held"
        );
    }
}

/// **AC-403 — an open gate is a TRAILER, an EVENT, and exit 0. `run` never waits.**
///
/// §7d.5 step (10): a `gate_opened` among the resumed run's new events makes the `Gate:` trailer
/// commit a **TIER-1 WRITE** (H-5), taken through `rk_git::worktree::authorise_tier1_write`
/// (ADR-0105 clause 8; AC-529; AC-518 — **never `cwd`**), then `commit_with_trailers(..,
/// &[Trailer { key: TrailerKey::Gate, value: format!("{token} {tree_hash} {deadline_secs}") }],
/// allow_empty = true)`.
///
/// # 🛑 THE ITEM IS `gate-1`, AND ITS `[facets]` TABLE IS EMPTY ON PURPOSE
///
/// `pack.toml`'s one `[[stage.implement.require]]` row reads `when = "has_repro"`.
/// `gate-item.toml` OMITS that facet, so the item carries `with_facet_null` for it, the evaluator
/// returns `Tri::Null`, and step 2 mints exactly one `GateOpened { reason: UnknownFacet, axis }` —
/// the abstention gate, and the only gate this pack opens (AC-079 / ADR-0089). `bug-1`
/// (`has_repro = true`) and `story-1` (`false`) both DECIDE it and open nothing.
///
/// # 🛑 NO CHILD IS SPAWNED ON THIS PATH, SO THE MARKER COUNT IS A FLOOR
///
/// The gate opens BEFORE any dispatch — `resume`'s loop breaks on `NextAction::OpenGate` — so no
/// implementation reachable from this fixture could spawn the fixture child. A "zero markers"
/// assertion here could not fail and is reported rather than written. `RK_FIXTURE_MODE` is still
/// set on the spawn, because leaving it unset would make an UNEXPECTED dispatch hang for 600 s
/// instead of failing fast (§7d.13).
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side bound, not a decision (AC-001) — s7d §7d.15's form for §7d.14's bounds"
)]
fn ac403_gate_open_writes_trailer_and_event_and_exits_zero_without_waiting() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();

    let started = std::time::Instant::now();
    let out = spawn_run_in_mode(root, "gate-1", None, "exit:0");
    let elapsed = started.elapsed();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);

    // (1) THE EXIT. A gate is a successful outcome, not a failure (D-01; AC-240; AC-024).
    assert_eq!(
        out.status.code(),
        Some(0),
        "an open gate is exit 0. stderr:\n{stderr}\nstdout:\n{stdout}"
    );

    // (2) IT DID NOT WAIT. Golden constraint 2: no channel, prompt, socket or sleep on any arm.
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "the run took {elapsed:?}; an open gate is an EXIT, not a wait"
    );

    // (3) THE EVENT. Read from the store the run wrote, not from what it printed.
    let rows = crate::support::journal_rows(root, "gate-1");
    // 🛑 TWO `gate_opened` ROWS ARE CORRECT, AND ONLY ONE OF THEM IS THIS TEST'S.
    //
    // §7d.5: `select()`'s own ANNOUNCEMENT rows go into `Selection::journal()` and *"the fold
    // counts as `gates_announced` and moves no state on"*; then `resume` steps
    // `NextAction::OpenGate(record)` through `Runtime::step`, *"which appends
    // `runtime::gate_opened_event(record)` — the `(kind, token, tree_hash, deadline)` row
    // `ac403_…` reads"*. The announcement carries no token; the runtime's row does. So the count
    // that discriminates is of rows carrying a TOKEN, and asserting `gate_opened == 1` would have
    // been red against a correct binary (measured: 2).
    let gates: Vec<&rk_types::event::Event> = rows
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::GateOpened)
        .collect();
    assert!(
        !gates.is_empty(),
        "no `gate_opened` row at all — `gate-1` omits `has_repro`, so the require row must abstain \
         and the abstention gate must open. Rows: {:?}",
        rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    let tokened: Vec<&&rk_types::event::Event> = gates
        .iter()
        .filter(|e| e.payload.get("token").is_some_and(|t| !t.is_null()))
        .collect();
    assert_eq!(
        tokened.len(),
        1,
        "exactly one gate row carries a token — the runtime's, the one a `Gate:` trailer is \
         derived from. Gate payloads: {:?}",
        gates.iter().map(|e| &e.payload).collect::<Vec<_>>()
    );
    let token = tokened[0]
        .payload
        .get("token")
        .and_then(serde_json::Value::as_str)
        .expect("the gate row carries its token");

    // (4) THE TRAILER, on the RUN-OWNED worktree's HEAD — and it carries the row's own token, so
    //     the commit and the event are about the same gate rather than two coincidences.
    let owned = common_dir(root)
        .join("redkiln")
        .join("worktrees")
        .join("gate-1");
    assert!(
        owned.is_dir(),
        "step (8) created no run-owned worktree, so there was nothing to take a tier-1 write \
         against. stderr:\n{stderr}"
    );
    let body = crate::support::head_message(&owned);
    assert!(
        body.contains("Gate:"),
        "the run-owned worktree's HEAD carries no `Gate:` trailer. §7d.5 step (10) makes it a \
         TIER-1 WRITE, which is the one thing a gate-opening run must leave behind. HEAD:\n{body}"
    );
    assert!(
        body.contains(token),
        "the `Gate:` trailer does not carry the token the `gate_opened` row recorded ({token}), so \
         the commit and the event are not about the same gate. HEAD:\n{body}"
    );

    // (5) THE WRITE WENT TO THE OWNED WORKTREE AND NOT TO THE PRODUCT REPOSITORY. AC-518/AC-529:
    //     never `cwd`. The outer repository's HEAD is still its single root commit.
    assert_eq!(
        crate::support::commit_count(root),
        1,
        "the `Gate:` trailer was committed into the PRODUCT repository. It is a tier-1 write \
         against the run-owned worktree, authorised through `authorise_tier1_write`, and AC-518 \
         forbids `cwd`"
    );

    // ---- (6) THE INVOCATION'S ONE ROW, IN ITS FOUR-MEMBER FORM (RT-41 + RT-47). ---------------
    //
    // 🛑 THIS ASSERTION LIVES HERE BECAUSE THIS IS THE ONLY TEST THAT CAN MAKE IT.
    //
    // §7d.5 step (6): *"the row is written once (8) has re-created the worktree, with all four
    // members"*. Step (8) is the OUTER path's. `ac485_…` and `adr0083_…` both drive `run_with`,
    // the INNER path, which never performs (8) and therefore never writes this row — their notes
    // owed the `adapter_set` and `config_digest` halves to "the cycle that writes the row", and
    // this is where that cycle can pay them against a real run rather than a constructed value.
    //
    // RT-41 and RT-47 are both ROUTED, not decided: AC-485's *"the adapter set recorded on the
    // run-start event"* names an event `EventKind::ALL` does not carry, and ADR-0105 clause 8's
    // *"resolves the run-owned worktree from the store"* names no carrier. §7d.18 holds the record
    // OWED. Asserted here as built; the ids are not claimed on the strength of it.
    let all_rows = crate::support::store_rows(root);
    let first = all_rows
        .first()
        .expect("a run that reached step (8) wrote at least its own first row");
    assert_eq!(
        first.kind,
        rk_types::event_kind::EventKind::ReconcileStarted,
        "the invocation's FIRST row is not `reconcile_started`. §7d.5 step (6) puts it before the \
         kernel is reached, so anything ahead of it is a row written before the reconcile. Rows: \
         {:?}",
        all_rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert_eq!(
        all_rows
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::ReconcileStarted)
            .count(),
        1,
        "AC-172: exactly one `reconcile_started` event per invocation. Rows: {:?}",
        all_rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // RT-41's two. The adapter set is the RESOLVED four, in `Config`'s order — harness, store,
    // then the toolchains `Config::new` sorted.
    assert_eq!(
        first.payload.get("adapter_set").and_then(|v| v.as_array()),
        Some(&vec![
            serde_json::Value::String("rk-adapter-claude".to_owned()),
            serde_json::Value::String("rk-adapter-sqlite".to_owned()),
            serde_json::Value::String("rk-adapter-toolchain-node".to_owned()),
            serde_json::Value::String("rk-adapter-toolchain-rust".to_owned()),
        ]),
        "AC-485's *\"the adapter set recorded on the run-start event\"* — the row must carry the \
         four the fixture's `[adapters]` table resolves to. Payload: {:?}",
        first.payload
    );
    let digest = first
        .payload
        .get("config_digest")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    assert_eq!(
        digest.len(),
        64,
        "ADR-0083's `config_digest` is a sha256 of the threaded `Config`'s canonical JSON, so it \
         is 64 hex characters; found {digest:?} in {:?}",
        first.payload
    );
    assert!(
        digest.chars().all(|c| c.is_ascii_hexdigit()),
        "the `config_digest` is not hex: {digest:?}"
    );

    // RT-47's pair, and it must name the tree the trailer was actually committed into — otherwise
    // `answer` would resolve a DIFFERENT worktree from the store than the one `run` wrote to.
    assert_eq!(
        first.payload.get("branch").and_then(|v| v.as_str()),
        Some("rk-run/gate-1"),
        "the row does not record the run-owned BRANCH. ADR-0105 clause 8 resolves a tier-1 write's \
         target *from the store*, and `answer` reads this member. Payload: {:?}",
        first.payload
    );
    let recorded_worktree = first
        .payload
        .get("worktree")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    assert_eq!(
        std::path::Path::new(recorded_worktree),
        owned.as_path(),
        "the row records a worktree that is not the one this run created and committed into. \
         `answer`'s tree hash and its `Gate:` trailer are both taken over the tree THIS member \
         names, so a mismatch here is a mismatch there. Payload: {:?}",
        first.payload
    );
}

/// One `exit/<name>/` case: what to run, against what, and what the child should do.
#[derive(Debug, serde::Deserialize)]
struct ExitCase {
    args: Vec<String>,
    pack: String,
    mode: String,
    /// A transcript beside `case.toml`, replayed to the child's stdout.
    ///
    /// `RK_FIXTURE_MODE=replay:<file>` makes `rk-fixture-child-claude` `fs::read` the path and
    /// `write_all` the bytes verbatim, so the path must be ABSOLUTE — which a committed `case.toml`
    /// cannot carry. Named here and resolved against the case directory instead.
    #[serde(default)]
    transcript: Option<String>,
    /// Overrides `[adapters] harness_binary` in the built repository.
    ///
    /// §7d.4 reaches `Transport` *"via a missing `harness_binary`"*, which is a property of the
    /// REPOSITORY and not of the child, so it cannot travel in the child environment like `mode`.
    #[serde(default)]
    harness_binary: Option<String>,
}

/// **AC-438's BINARY arm** — §7d.4's *"through the binary"* half of the table-driven test,
/// over `tests/fixtures/exit/<name>/`.
///
/// §7d.13 fixes the layout: *"one directory per expected code, with an `expected` file carrying
/// the integer"*. Each directory here also carries a `case.toml` saying how the code is reached,
/// so the fixture is readable without the test.
///
/// # 🛑 NINE DIRECTORIES — GAP 21's SET, COMPLETE AS OF SLICE 3 CYCLE 10
///
/// `0, 2, 3, 5, 10, 11 (×2), 13, 18`. **Never a `12`**: §7d.4 fixes that the adapter classifies its
/// timer arm `Cancelled { by: Timeout }`, *"so this fixture is the binary-level instance of row
/// `13`, not `12`, and a fixture whose `expected` reads `12` is red on a correct binary"*. `70` is
/// in-process only. There is **no `14`** — GAP 21 does not derive one and §7d.4's enumeration of
/// what the fixture child can produce names none.
///
/// Four of the nine were RAISED and unreachable until cycle 10, and the reason was measured: until
/// then `replay::dispatch_through` discarded the failure class into `NotCommitted { step_id,
/// activity_id_at_version }`, two `String`s, and **nothing a real run wrote named it** —
/// `ClaudeHarness` appends one `dispatch_completed` carrying `cancelled_by` and no class member,
/// and the only callers of `StepError::event()` in any `src/` were `FakeHarness`'s own bridge and a
/// discarded call in `conformance.rs`. So every failed dispatch exited `3`. **Mechanism B** — the
/// operator's choice — carries the `StepError` on the refusal, and step (10) maps
/// `impl From<&StepError> for Exit` with no journal round trip. `fixtures/exit/README.md` is the
/// record of that decision.
///
/// Row **13 was the exception** and is why it was here from cycle 6: `cancelled_by` IS on the
/// record, so step (10) recovered that one class and nothing else. Row **`5` landed at cycle 9**
/// with `explain`, and could not have been authored earlier — the verb was a stub returning `3`, so
/// the fixture would have been green against something that did nothing.
///
/// # 🛑 §7d.4's *"`Schema` via a malformed frame"* IS NOT ONE OF THEM
///
/// Measured through the binary: a system/init frame followed by `this is not json {"type":"result"`
/// exits **`10`**, not `11`. `classify.rs`'s rule 8 takes a stream that yielded no parseable result
/// frame to `Transport { kind: Decode }`, and `extract_digest` — where `Schema` is minted — is rule
/// 10 and is never reached. A `schema-malformed-frame/` reading `11` would be red on a correct
/// binary in exactly the way a `12` would. GAP 21's second `11` is authored as the other arm of
/// `extract_digest` instead: `structured_output` present and failing the schema, beside its sibling
/// carrying none at all. RAISED.
///
/// # 🛑 THE DIRECTORY SET IS ASSERTED, SO A DELETED CASE IS RED
///
/// A table-driven test that walks a directory and asserts over whatever it finds passes over an
/// EMPTY directory. The names are listed here and compared against what is on disk, in both
/// directions.
#[test]
#[expect(
    clippy::disallowed_types,
    reason = "every row of this table is driven AS A PROCESS; that is what makes it the BINARY \
              arm (s7d §7d.4)"
)]
fn ac438_table_driven_exit_codes_over_fixtures_binary_arm() {
    const CASES: [&str; 9] = [
        "cancelled-timeout",
        "gate-open",
        "no-pack",
        "no-record",
        "platform-is-error",
        "resume-flag",
        "schema-invalid-structured-output",
        "schema-no-structured-output",
        "transport-missing-binary",
    ];

    let dir = crate::fixtures_dir().join("exit");
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .expect("tests/fixtures/exit/ exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    on_disk.sort();
    assert_eq!(
        on_disk, CASES,
        "the `exit/` directory set and this test's list disagree. A table that walked the \
         directory alone would pass over an empty one"
    );

    for name in CASES {
        let case_dir = dir.join(name);
        let expected: i32 = std::fs::read_to_string(case_dir.join("expected"))
            .unwrap_or_else(|e| panic!("{name}/expected is readable: {e}"))
            .trim()
            .parse()
            .unwrap_or_else(|e| panic!("{name}/expected carries an integer: {e}"));
        let case: ExitCase = toml::from_str(
            &std::fs::read_to_string(case_dir.join("case.toml"))
                .unwrap_or_else(|e| panic!("{name}/case.toml is readable: {e}")),
        )
        .unwrap_or_else(|e| panic!("{name}/case.toml parses: {e}"));

        // The repository the case names. `none` is a bare `init`-only one — D18-8's own state.
        let repo = match case.pack.as_str() {
            "none" => {
                let repo = temp_repo();
                assert_eq!(
                    run_init(repo.path()).status.code(),
                    Some(0),
                    "{name}: the repository initialises"
                );
                repo
            }
            "base" => pack_min_repo(PackVariant::Base),
            "dispatch-1s" => pack_min_repo(PackVariant::DispatchOneSecond),
            other => panic!("{name}/case.toml names an unknown pack {other:?}"),
        };
        let root = repo.path();

        // The repository-side override, before anything is spawned.
        if let Some(binary) = &case.harness_binary {
            let path = root.join(".redkiln").join("config.toml");
            let src = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("{name}: {} is readable: {e}", path.display()));
            let rewritten: String = src
                .lines()
                .map(|l| {
                    if l.trim_start().starts_with("harness_binary") {
                        format!("harness_binary = \"{binary}\"")
                    } else {
                        l.to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            assert_ne!(
                rewritten, src,
                "{name}: no `harness_binary` line was rewritten, so the case is not over the \
                 configuration it names"
            );
            std::fs::write(&path, rewritten).expect("the config rewrites");
        }

        // The child's mode, with a named transcript resolved to an absolute path.
        let mode = case.transcript.as_ref().map_or_else(
            || case.mode.clone(),
            |file| {
                let path = case_dir.join(file);
                assert!(
                    path.is_file(),
                    "{name}/case.toml names the transcript {file:?} and {} does not exist",
                    path.display()
                );
                format!("replay:{}", path.display())
            },
        );

        let markers = root.join(".rk-markers");
        std::fs::create_dir_all(&markers).expect("the marker directory is created");
        let mut cmd = std::process::Command::new(crate::redkiln());
        cmd.arg("-C").arg(root);
        for arg in &case.args {
            cmd.arg(arg);
        }
        cmd.env("RK_FIXTURE_MODE", &mode)
            .env("RK_FIXTURE_MARKER_DIR", &markers);
        let out = crate::support::output_bounded(&mut cmd);

        assert_eq!(
            out.status.code(),
            Some(expected),
            "{name}: `redkiln {}` under mode {:?} exited {:?}, not the {expected} its `expected` \
             file carries.\nstderr:\n{}\nstdout:\n{}",
            case.args.join(" "),
            mode,
            out.status.code(),
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
    }
}

/// **AC-264 / AC-186 — a `run` after an APPLIED answer resumes PAST the gate, with no flag.**
///
/// §7d.3, verbatim: *"open a gate (`ac403_…`'s fixture), `answer <item> approve --explain-sufficed
/// yes --toml-opened no` (`Applied`), then `redkiln run <item>` with no flag — exit `0`, the fold
/// shows the gate step committed and the next step reached (a second `dispatch` row, or `run`
/// completing the pack), and no `--resume` was needed; red by a `run` that re-opens the gate or
/// refuses"*.
///
/// # 🛑 THIS IS THE CLI-SIDE PROOF OF `replay::resume`, AND THE PREMISE IS WHAT MAKES IT ONE
///
/// `record()` folds a **fresh local vector** each call and breaks on `NextAction::OpenGate` without
/// appending, so a `run` built on it re-derives `OpenGate` at reach 1 for ever and the gate can
/// never be got past. `resume` seeds the fold from the store's journal instead, which is why the
/// answered gate moves state. Run 1 is asserted to stop AT the gate — one answerable gate row and
/// **zero** `dispatch` rows — so the `dispatch` row after run 3 is new, and "the next step reached"
/// is a change this test observed rather than a state it found lying around.
///
/// # 🛑 §7d.3's `exit 0` IS NOT ASSERTED, AND THE REASON IS MEASURED RATHER THAN CONCEDED
///
/// §7d.3 says the resumed run is exit `0`. It is exit **`3`** here, and not because anything
/// refused to resume. Measured, run 3's journal and stderr:
///
/// ```text
/// …, GateAnswered, ReconcileStarted, GateResumed, Dispatch, DispatchCompleted
/// redkiln: the dispatch of `emit-report` (emit-report@1) did not commit.
/// redkiln: the dispatch did not commit and the journal carries no error row for `gate-1`, so no
/// row of the exit table describes it.
/// ```
///
/// The gate WAS resumed and the next step WAS entered — that is what the assertions below read.
/// Exit `0` additionally requires the dispatch to **commit**, and a committing dispatch needs a
/// child that emits a result frame: `RK_FIXTURE_MODE=replay:<transcript>`, which no `rk-cli` test
/// has yet (`exit:0` makes the child exit before any frame). So exit `0` is a claim about the
/// harness, not about resumption, and this test does not make it — AC-264's own register text is
/// about the **flag** (*"`redkiln run <item>` is sufficient and `--resume` is not an accepted
/// flag"*), discharged here and by `ac400_resume_refused_as_unknown_argument`.
///
/// **RAISED**: §7d.3's exit `0` is unreachable for every `rk-cli` test until a committing
/// transcript fixture exists, and this exact case is also the one mechanism B turns from `3` into
/// its true row — the class here is `Transport`, `10`.
#[test]
fn ac264_run_after_applied_answer_resumes_past_gate() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();

    // ---- (1) THE GATE OPENS, and the run stops there. ------------------------------------------
    let first = spawn_run(root, "gate-1", None);
    assert_eq!(
        first.status.code(),
        Some(0),
        "the gate-opening run is exit 0 (AC-403). stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    let after_one = crate::support::journal_rows(root, "gate-1");
    assert_eq!(
        after_one
            .iter()
            .filter(|e| crate::answer::is_answerable_gate(e))
            .count(),
        1,
        "run 1 opened no answerable gate, so there is nothing to resume past. A gate writes TWO \
         `gate_opened` rows and only the runtime's carries a token. Rows: {:?}",
        after_one.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert_eq!(
        after_one
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::Dispatch)
            .count(),
        0,
        "run 1 dispatched before the gate was answered, so a `dispatch` row after run 3 would not \
         be evidence that anything resumed. Rows: {:?}",
        after_one.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // ---- (2) THE ANSWER, which must APPLY. -----------------------------------------------------
    let answered =
        crate::answer::spawn_answer(root, "gate-1", "approve", &crate::answer::BOTH_FLAGS);
    assert_eq!(
        answered.status.code(),
        Some(0),
        "PREMISE: the answer must APPLY, or run 3 would be resuming past nothing. stderr:\n{}",
        String::from_utf8_lossy(&answered.stderr)
    );
    let after_answer = crate::support::journal_rows(root, "gate-1");
    assert_eq!(
        after_answer
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::GateAnswered)
            .count(),
        1,
        "PREMISE: exactly one `gate_answered` row is owed after an applied answer. Rows: {:?}",
        after_answer.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // ---- (3) THE RESUMED RUN — `redkiln run <item>`, AND NO FLAG. ------------------------------
    let second = spawn_run(root, "gate-1", None);
    let after_two = crate::support::journal_rows(root, "gate-1");

    // The gate was not re-opened: that is the named red.
    assert_eq!(
        after_two
            .iter()
            .filter(|e| crate::answer::is_answerable_gate(e))
            .count(),
        1,
        "the resumed run RE-OPENED the gate — a second answerable `gate_opened` row. That is \
         `record()`'s behaviour, which folds a fresh vector and re-derives `OpenGate` at reach 1; \
         §7d.5 step (9) binds `run` to `replay::resume`, which seeds the fold from the \
         journal.\nrun 3 exited {:?}\nstderr:\n{}\nrows: {:?}",
        second.status.code(),
        String::from_utf8_lossy(&second.stderr),
        after_two.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // The KERNEL's own record that it got past it. `gate_resumed` is written by the resume path and
    // by nothing else, so this is the row that separates "resumed" from "never gated in the first
    // place" — a strictly stronger reading than the dispatch count below.
    assert_eq!(
        after_two
            .iter()
            .filter(|e| e.kind == rk_types::event_kind::EventKind::GateResumed)
            .count(),
        1,
        "the resumed run wrote no `gate_resumed` row, so it did not get past the gate it was asked \
         to resume.\nrun 3 exited {:?}\nstderr:\n{}\nrows: {:?}",
        second.status.code(),
        String::from_utf8_lossy(&second.stderr),
        after_two.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // And the next step was REACHED — §7d.3's *"a second `dispatch` row"*.
    assert!(
        after_two
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::Dispatch),
        "the resumed run reached no `dispatch`, so the step after the gate was never \
         entered.\nrun 3 exited {:?}\nstderr:\n{}\nrows: {:?}",
        second.status.code(),
        String::from_utf8_lossy(&second.stderr),
        after_two.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
}

/// **AC-257 — a gate whose deadline has passed is resolved at the next `run`, in a FRESH PROCESS.**
///
/// §7d.14, verbatim: *"Red by a gate whose deadline passed not resolving at the next `run` in a
/// fresh process (no `gate_expired` row; or `gate::answer` applying it — the expiry pass of §7d.5
/// step (9) is what makes the deadline observable, since the kernel refuses `Expired` only on a row
/// already in the journal); a weaker red resolves it in the same process that opened it"*.
///
/// # 🛑 BOTH HALVES ARE ASSERTED, AND THE FIRST IS WHAT MAKES THIS A CROSS-PROCESS TEST
///
/// Process 1 opens the gate under `pack.gate-1s.toml`'s `gate_deadline_secs = 1` and must leave
/// **no** `gate_expired` row behind. Resolving a gate in the process that opened it is the weaker
/// red §7d.14 names, and an assertion on process 2 alone is satisfied by a binary that expires the
/// gate the instant it opens it. Process 2 is a fresh `redkiln` and the row must exist after it.
///
/// # 🛑 THE WAIT IS TEST-SIDE, AND THE RECORDED DEADLINE IS READ BACK SO IT CANNOT BE VACUOUS
///
/// AC-474 forbids a clock seam — no env var, no flag and no config key makes a deadline pass — so
/// the only admitted mechanism is the 1-second pack variant plus a wait here (*"a test may sleep;
/// the product never does"*). The deadline process 1 recorded is read off its own gate row and
/// asserted to be in the past **before** process 2 is spawned. Without that, a variant carrying the
/// base 900 s deadline would redden every assertion below for the wait rather than for the expiry
/// pass — which is the wrong reason, and is invisible from the failure text otherwise.
///
/// # 🛑 PROCESS 2's EXIT IS REPORTED AND NOT CLAIMED
///
/// AC-257's THEN is the resolution, and the resolution is the row. What `run` then decides about a
/// stage whose only gate has failed is §7d.5 step (10)'s and is asserted by its own tests; this one
/// carries process 2's exit in its failure text and asserts nothing about it.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side wait past a 1 s gate deadline, the clock reads that prove it cleared it, \
              and the Instant bound that stops the poll — not a decision (AC-003); s7d §7d.15's \
              form for §7d.14's bounds"
)]
fn ac257_expired_gate_resolved_by_fresh_process() {
    let repo = pack_min_repo(PackVariant::GateOneSecond);
    let root = repo.path();

    // ---- PROCESS 1 — open the gate. ------------------------------------------------------------
    let first = spawn_run_in_mode(root, "gate-1", None, "exit:0");
    assert_eq!(
        first.status.code(),
        Some(0),
        "the gate-opening run is exit 0 (AC-403), so anything below would be measuring a refusal. \
         stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );

    let after_one = crate::support::journal_rows(root, "gate-1");
    let gate = after_one
        .iter()
        .find(|e| {
            e.kind == rk_types::event_kind::EventKind::GateOpened
                && e.payload.get("token").is_some_and(|t| !t.is_null())
        })
        .unwrap_or_else(|| {
            panic!(
                "process 1 opened no answerable gate, so there is nothing for a later run to \
                 expire. A gate writes TWO `gate_opened` rows and only the runtime's carries a \
                 token. Rows: {:?}",
                after_one.iter().map(|e| e.kind).collect::<Vec<_>>()
            )
        });
    let deadline = gate
        .payload
        .get("deadline")
        .and_then(serde_json::Value::as_i64)
        .expect("the runtime's gate row carries its deadline in unix seconds");

    // The weaker red, asserted directly rather than trusted.
    assert!(
        !after_one
            .iter()
            .any(|e| e.kind == rk_types::event_kind::EventKind::GateExpired),
        "process 1 wrote a `gate_expired` row for a gate it had just opened. AC-257 is about the \
         NEXT run observing the deadline; a gate resolved in the process that opened it is the \
         weaker red §7d.14 names. Rows: {:?}",
        after_one.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // ---- THE WAIT, past the deadline process 1 itself recorded. --------------------------------
    //
    // 🛑 POLLED, NOT A FIXED SLEEP — a repair, not a preference. This read
    // `sleep(1_500)` followed by a single `now > deadline` assertion, which is a COIN FLIP: the
    // deadline is recorded in WHOLE unix seconds as `floor(t0) + 1`, so clearing it after a 1.5 s
    // sleep requires `frac(t0) + d + 1.5 >= 2` — i.e. `frac(t0) >= 0.5 - d`, where `d` is the
    // elapsed spawn and journal read. MEASURED 2026-09-21 at Step 7d's close: **2 of 10 filtered
    // runs failed** on the premise guard, while `--test it` whole stayed green because it burns
    // more wall clock between the two points. That is why eleven Slice-3 cycles never saw it, and
    // why Checkpoint 7d failed on this line at its first whole run. A checkpoint line that fails
    // one run in five is not a gate.
    //
    // The premise is now the loop's own EXIT CONDITION, so it cannot be observed unsatisfied.
    // Bounded at 30 s (Halt 5), and the bound's failure text names the VARIANT rather than the box:
    // a deadline that never passes means the gate row does not carry the 1-second one, which is the
    // defect worth reporting.
    let wait_started = std::time::Instant::now();
    loop {
        let now = i64::try_from(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("the clock is after the unix epoch")
                .as_secs(),
        )
        .expect("a unix second fits an i64");
        if now > deadline {
            break;
        }
        assert!(
            wait_started.elapsed() < std::time::Duration::from_secs(30),
            "30 s elapsed and the clock never passed the deadline process 1 recorded ({deadline}); \
             now is {now}. Every assertion below would be red for the wait rather than for the \
             absent expiry pass, so this stops here: the gate row does not carry \
             `pack.gate-1s.toml`'s `gate_deadline_secs = 1`, which is the variant's defect and not \
             a slow box — check that `PackVariant::GateOneSecond` installed it (Halt 5)"
        );
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // ---- PROCESS 2 — a FRESH `redkiln`, which must resolve it at its step (9). ------------------
    let second = spawn_run_in_mode(root, "gate-1", None, "exit:0");
    let after_two = crate::support::journal_rows(root, "gate-1");
    let expired: Vec<&rk_types::event::Event> = after_two
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::GateExpired)
        .collect();
    assert_eq!(
        expired.len(),
        1,
        "the next `run` in a fresh process wrote no `gate_expired` row for a gate whose deadline \
         had passed. §7d.5 step (9)'s expiry pass is what makes the deadline observable: the kernel \
         refuses `Expired` only on a row already in the journal, so nothing else resolves \
         it.\nprocess 2 exited {:?}\nstderr:\n{}\nrows: {:?}",
        second.status.code(),
        String::from_utf8_lossy(&second.stderr),
        after_two.iter().map(|e| e.kind).collect::<Vec<_>>()
    );

    // And the row is about THE gate process 1 opened, not some other one.
    let token = gate
        .payload
        .get("token")
        .and_then(serde_json::Value::as_str)
        .expect("the gate row carries its token");
    assert_eq!(
        expired[0]
            .payload
            .get("token")
            .and_then(serde_json::Value::as_str),
        Some(token),
        "the `gate_expired` row names a token other than the one process 1's gate carried, so the \
         two rows are not about the same gate. Payload: {:?}",
        expired[0].payload
    );
}
