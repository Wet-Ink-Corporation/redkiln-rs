//! **`run`'s refusal paths** — §7d.5 steps (1)–(7), every one of which refuses *before* a worktree
//! exists.
//!
//! None of these needs a harness, a worktree or a resume, which is why the contract sequences them
//! together: they are the largest block of `run` behaviour reachable without step (8).

use std::path::Path;

use crate::support::{PackVariant, pack_min_repo, set_enabled};
use crate::{common_dir, refs, run_init, temp_repo};

/// Spawn `redkiln -C <repo> run <item> [--lane <lane>]` with stdin closed (D-15).
fn spawn_run(repo: &Path, item: &str, lane: Option<&str>) -> std::process::Output {
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
fn spawn_run_in_mode(
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
    let inputs = crate::support::inputs_over_pack_min(&resolved)
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
    let exit1 = rk_cli::verbs::run::run_with(&resolved, &inputs, p1, &mut s1, &token)
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
    let exit2 = rk_cli::verbs::run::run_with(&resolved, &inputs, p2, &mut s2, &token)
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
}

/// One `exit/<name>/` case: what to run, against what, and what the child should do.
#[derive(Debug, serde::Deserialize)]
struct ExitCase {
    args: Vec<String>,
    pack: String,
    mode: String,
}

/// **AC-438's BINARY arm** — §7d.4's *"through the binary"* half of the table-driven test,
/// over `tests/fixtures/exit/<name>/`.
///
/// §7d.13 fixes the layout: *"one directory per expected code, with an `expected` file carrying
/// the integer"*. Each directory here also carries a `case.toml` saying how the code is reached,
/// so the fixture is readable without the test.
///
/// # 🛑 FOUR DIRECTORIES, NOT NINE — AND THE MISSING FIVE ARE A MEASUREMENT, NOT AN OVERSIGHT
///
/// GAP 21 derives nine rows for this arm (0, 2, 3, 5, 10, 11 ×2, 13, 18). Four are here, and the
/// other five are **RAISED** in `fixtures/exit/README.md` with what was measured:
///
///   * **10 / 11 / 14 / 18 need the failure CLASS, and the real harness journals none.** The only
///     callers of `StepError::event()` in any `src/` are `rk-ports`' `fakes.rs` (`FakeHarness`'s
///     own bridge) and a discarded call in `conformance.rs`. `ClaudeHarness` appends ONE
///     `dispatch_completed` row whose `DispatchRecord` carries `cancelled_by` and **no class
///     member**, and `Runtime::run_activity` appends only `dispatch` and `activity_committed`. So
///     the class is nowhere in what a real run writes, and `exit_for_uncommitted` has nothing to
///     read. Two mechanisms would close it; choosing one is above this stage.
///   * **5 (`Exit::NoRecord`) is `explain`'s**, and nothing in `rk-cli/src` returns it yet.
///
/// Row **13 IS here**, because it is the one class the real path does make derivable: the
/// adapter's own timer arm sets `cancelled_by: timeout` on the record, and step (10) reads it.
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
    const CASES: [&str; 4] = ["cancelled-timeout", "gate-open", "no-pack", "resume-flag"];

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

        let markers = root.join(".rk-markers");
        std::fs::create_dir_all(&markers).expect("the marker directory is created");
        let mut cmd = std::process::Command::new(crate::redkiln());
        cmd.arg("-C").arg(root);
        for arg in &case.args {
            cmd.arg(arg);
        }
        cmd.env("RK_FIXTURE_MODE", &case.mode)
            .env("RK_FIXTURE_MARKER_DIR", &markers);
        let out = crate::support::output_bounded(&mut cmd);

        assert_eq!(
            out.status.code(),
            Some(expected),
            "{name}: `redkiln {}` under mode {:?} exited {:?}, not the {expected} its `expected` \
             file carries.\nstderr:\n{}\nstdout:\n{}",
            case.args.join(" "),
            case.mode,
            out.status.code(),
            String::from_utf8_lossy(&out.stderr),
            String::from_utf8_lossy(&out.stdout)
        );
    }
}
