//! **ADR-0141 clause 4's OWED guard: the composition root always transcribes a step list.**
//!
//! ADR-0141 decision 2 gives `RecordedSpec` two constructors — `supplied` (yields `steps: None`,
//! *this caller transcribed none*) and `with_steps` (yields `Some`) — and decision 4 records the
//! hole this module closes, in terms:
//!
//! > **The production path must always transcribe, and that is currently a convention rather than a
//! > guard.** Measured: **one** `with_steps` call site (`rk-cli`'s `bind_nodes`) against **16**
//! > `supplied`, all in suites. Nothing fails if a future production site reaches for `supplied`.
//! > **A guard asserting that the composition root transcribes is OWED.**
//!
//! # What goes wrong without it, and why it is silent
//!
//! `steps: None` is a legitimate value — it is what the kernel's own machinery suites supply, and
//! `run_activity_steps` falls through to the pre-`s7g` activation dispatch on it. That is exactly
//! what makes a production `supplied` dangerous: it does not crash, it does not warn, and it does
//! not fail a test. The run simply stops walking steps and goes back to dispatching whole
//! activations, which is the defect `s7g` exists to end — reintroduced by a one-word edit, on a
//! path no suite exercises. **A wrong `None` here is indistinguishable from a correct one.**
//!
//! # 🛑 The comment filter is LOAD-BEARING, and the control for it is LIVE rather than synthetic
//!
//! `crates/rk-cli/src/verbs/run.rs` carries the forbidden spelling **today**, in `run_with`'s
//! docstring, quoting §7d.5: *"the fake-harness tests hand a supplier over
//! `RecordedSpec::supplied(..)` values"*. That sentence is correct and must stay. So a naive
//! `grep -rn 'RecordedSpec::supplied' crates/rk-cli/src` is **RED on a correct tree right now**, and
//! any version of this guard that did not strip comments would either fire falsely or be "fixed" by
//! deleting true prose — which is the *"do not widen the guard, and do not write the hit"* rule
//! (Halt 7) in its most tempting form.
//!
//! [`the_comment_filter_is_doing_real_work_on_run_rs`] asserts that situation directly: the raw
//! bytes contain the spelling and the code lines do not. It is a **positive control over the live
//! tree**, not a fixture — so if the docstring is ever reworded away, the control fails loudly and
//! says the guard below has stopped being exercised, rather than quietly becoming decorative.
//!
//! # Two halves, because the source lint alone is a SPELLING check
//!
//! This stage has already paid for believing a source lint is a capability check —
//! `conformance.rs`'s check-arm lint was defeated by a `&Ports` parameter and by an aliased import.
//! So the lint has a second half beside it, and **the division of labour is measured, not assumed**:
//!
//! | attack | source lint | [`the_spec_a_real_run_binds_carries_its_steps`] |
//! | --- | --- | --- |
//! | `bind_nodes` reaches for `supplied` by name | **KILLED** | killed |
//! | the transcription is deleted outright | **KILLED** (required half) | killed |
//! | the binder is renamed at both call sites | **KILLED** (after the anchor fix) | — |
//! | the spelling in a leading or trailing comment | correctly **green** | — |
//! | **an ALIAS, with a dead `with_steps` keeping the lint satisfied** | **GREEN — blind** | **KILLED** |
//! | **`to_json` stops writing the key** | **GREEN — blind** | **KILLED** |
//!
//! The last two rows are the point. A source lint sees a spelling in one crate's `src/`; the
//! behavioural half sees **what the composition root actually produced**, so an alias, a helper one
//! call deep, a third constructor or a refactor nobody anticipated are all red there without the
//! lint needing to know their shape. The lint earns its place by being *cheap and precise* — it
//! names the offending file and line, and it runs with no process spawn.
//!
//! # What neither half closes, stated rather than left to be discovered
//!
//! The behavioural half observes the binding **for the node `pack-min` selects** — one activity, one
//! step. A `bind_nodes` that transcribed for some activities and not others would only be caught if
//! this fixture exercised the branch that fails, and it binds a single node. Widening that means
//! more fixtures, not a different instrument.
//!
//! It also depends on a **gate** run continuing to exit 0 and write a complete record. If that ever
//! stops being true the PREMISE assertion fires rather than the test silently reading nothing — but
//! the coverage would be gone until another committing path exists, which is criterion (1)'s
//! business rather than this file's.

use crate::{rs_files, src_dir};

/// One file's CODE lines as `(1-indexed line, content)` — **comments stripped with the tree's own
/// block-aware stripper**, `rk_kernel::kernel_denylist::strip_comments`.
///
/// 🛑 **Not `crate::code_lines`, and the reason is measured rather than stylistic.** That helper is
/// line-based and its companion `assert_no_block_comment` exists to stop it reading a `/*` region it
/// cannot parse — a rule `rk-cli`'s own lints keep by scanning NARROW REGIONS. This guard scans two
/// whole crates, and both carry block comments today: `rk-cli/src/adapters.rs` and
/// `rk-kernel/src/replay.rs` each fired that assertion when this module first used it. Refusing the
/// scan would make the guard unwritable; widening `assert_no_block_comment` across crates would be
/// asserting one crate's house style over another's. `conformance.rs` reached the same conclusion
/// for the same reason: *"Three lints read whole directories of other people's code, so the stripper
/// has to be the real one."*
fn code_lines(src: &str) -> Vec<(usize, String)> {
    let mut in_block = false;
    src.lines()
        .enumerate()
        .map(|(i, l)| {
            (
                i + 1,
                rk_kernel::kernel_denylist::strip_comments(l, &mut in_block),
            )
        })
        .filter(|(_, l)| !l.trim().is_empty())
        .collect()
}

/// The constructor that yields `steps: None`. **Forbidden on any production code line.**
const FORBIDDEN: &str = "RecordedSpec::supplied";

/// The constructor that transcribes. **At least one production code line must carry it**, or the
/// guard above is satisfied by a tree that binds no steps at all.
const REQUIRED: &str = "RecordedSpec::with_steps";

/// `bind_nodes` is the composition root's binding pass and the site the property is about. Anchoring
/// on it means a rename or a deletion fails here rather than silently emptying the scan.
///
/// ⚠️ **The trailing `(` is load-bearing and was added after a mutant survived without it.** As
/// `"fn bind_nodes"` this anchor matched `fn bind_nodes_renamed(` — a substring anchor binds to any
/// longer name that starts with it, so renaming the binder at both call sites compiled AND kept the
/// guard green. That is the same family as the decoy-`PortsDraft` hole an adversarial pass found in
/// `conformance.rs`'s two parsed-set anchors, reproduced here by writing the anchor the same way.
const BINDER: &str = "fn bind_nodes(";

/// **ADR-0141 clause 4.** No production path constructs a `RecordedSpec` with no steps, and at least
/// one constructs one with them.
///
/// Two-sided on purpose. The forbidden half alone is satisfied by deleting the transcription
/// entirely; the required half alone is satisfied by transcribing at one site and supplying `None` at
/// another. Neither half is evidence without the other.
#[test]
fn the_production_path_transcribes_a_step_list_and_never_supplies_none() {
    let src = src_dir();

    // ---- ANCHORS FIRST. Every one of these fails loudly rather than emptying the scan. ----------
    let files = rs_files(&src);
    assert!(
        !files.is_empty(),
        "no .rs files under {} — an absence assertion over an empty file set reads as a pass",
        src.display()
    );

    // The API this lint is about must still exist, and with BOTH constructors. If `RecordedSpec`
    // grew a third `None`-yielding constructor, or lost one of these two, the spellings below stop
    // describing the property and this guard must be rewritten rather than left passing.
    let replay = src
        .join("..")
        .join("..")
        .join("rk-kernel")
        .join("src")
        .join("replay.rs");
    let replay_src = std::fs::read_to_string(&replay)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", replay.display()));
    // An EXISTENCE anchor, so it is taken over CODE lines: an unstripped doc comment naming a
    // constructor would satisfy it falsely, which is the whole failure mode this module guards.
    let replay_code = code_lines(&replay_src);
    for decl in ["pub fn supplied", "pub fn with_steps", "steps: Option<"] {
        assert!(
            replay_code.iter().any(|(_, l)| l.contains(decl)),
            "`{decl}` is not in {} — `RecordedSpec`'s two-constructor shape (ADR-0141 decision 2) \
             has changed, and this guard's spellings no longer describe the property it asserts",
            replay.display()
        );
    }

    // ---- THE SCAN --------------------------------------------------------------------------
    let mut offenders: Vec<String> = Vec::new();
    let mut transcriptions: Vec<String> = Vec::new();
    let mut binder_seen = false;
    let mut scanned = 0usize;

    for file in &files {
        let text = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", file.display()));
        let rel = file
            .strip_prefix(&src)
            .unwrap_or(file.as_path())
            .display()
            .to_string();
        for (n, line) in code_lines(&text) {
            scanned += 1;
            if line.contains(BINDER) {
                binder_seen = true;
            }
            if line.contains(FORBIDDEN) {
                offenders.push(format!("{rel}:{n}: {}", line.trim()));
            }
            if line.contains(REQUIRED) {
                transcriptions.push(format!("{rel}:{n}"));
            }
        }
    }

    assert!(
        scanned > 500,
        "only {scanned} code lines under {} — the scan is too small to be reading this crate",
        src.display()
    );
    assert!(
        binder_seen,
        "`{BINDER}` was not found on any code line under {} — the composition root's binding pass \
         was renamed or removed, so this guard is asserting a property about a site that no longer \
         exists",
        src.display()
    );

    // ---- (1) NO PRODUCTION `None`. ----------------------------------------------------------
    assert!(
        offenders.is_empty(),
        "ADR-0141 decision 4: a PRODUCTION path constructs a `RecordedSpec` with NO step list.\n  \
         `{FORBIDDEN}` yields `steps: None`, on which `run_activity_steps` falls through to the \
         pre-`s7g` activation dispatch — so this does not crash, does not warn, and fails no suite; \
         it silently stops the runtime walking steps at all.\n  Use `{REQUIRED}`: the composition \
         root holds the `catalog::Activity` and already reads its steps.\n  Offending line(s):\n    \
         {}",
        offenders.join("\n    ")
    );

    // ---- (2) AND SOMETHING ACTUALLY TRANSCRIBES. --------------------------------------------
    assert!(
        !transcriptions.is_empty(),
        "no production code line carries `{REQUIRED}`. The half above is then satisfied by a tree \
         that transcribes NOTHING, which is the same defect wearing the other sign: every \
         activation would take the `steps: None` fall-through and the step executor would be dead \
         code. ADR-0141 decision 2 puts this at `rk-cli`'s `bind_nodes`."
    );
}

/// **The positive control, over the LIVE tree rather than a fixture.**
///
/// `run.rs` quotes `RecordedSpec::supplied(..)` in `run_with`'s docstring today. That makes the
/// comment filter in the guard above load-bearing *right now*: without it the guard is red on a
/// correct tree. This test asserts both halves of that situation, so the day the docstring is
/// reworded the control fails and says the guard has stopped being exercised — rather than the
/// guard quietly becoming a scan with nothing left to strip.
#[test]
fn the_comment_filter_is_doing_real_work_on_run_rs() {
    let run_rs = src_dir().join("verbs").join("run.rs");
    let text = std::fs::read_to_string(&run_rs)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", run_rs.display()));

    assert!(
        text.contains(FORBIDDEN),
        "{} no longer mentions `{FORBIDDEN}` anywhere. That spelling lived in `run_with`'s \
         docstring, quoting §7d.5, and it is what made the comment filter in \
         `the_production_path_transcribes_a_step_list_and_never_supplies_none` load-bearing on a \
         correct tree. If it was deliberately reworded, this control is dead and a synthetic case \
         is owed in its place; if it was deleted to make a lint pass, that is the defect Halt 7 \
         names — do not widen the guard, and do not write the hit.",
        run_rs.display()
    );

    let code = code_lines(&text);
    let survivors: Vec<String> = code
        .iter()
        .filter(|(_, l)| l.contains(FORBIDDEN))
        .map(|(n, l)| format!("{n}: {}", l.trim()))
        .collect();
    assert!(
        survivors.is_empty(),
        "the comment filter kept `{FORBIDDEN}` on a line it reported as CODE in {}. Either the \
         mention moved out of a comment and into real code — in which case the guard above should \
         be failing and this is the more precise report of why — or `code_lines` has stopped \
         stripping the shape it carries:\n    {}",
        run_rs.display(),
        survivors.join("\n    ")
    );
}

/// **ADR-0141 clause 4's BEHAVIOURAL half: the spec a REAL RUN binds carries its steps.**
///
/// The source lint above asserts a *spelling* in production code, and its own docstring records two
/// measured holes it cannot close — an aliased `supplied` added beside a surviving `with_steps`, and
/// a helper one call deep. **This test closes both, because it does not care how the value was
/// built.** It spawns the real binary, lets the composition root bind whatever it binds, and reads
/// what came out the other end. An alias, a helper, a third constructor, a refactor nobody
/// anticipated — all of them produce a `RecordedSpec` with no `steps` key, and all of them are red
/// here.
///
/// # What makes the observation possible at all, and why it is a GATE run
///
/// `RecordedSpec::to_json` writes `steps` **only when `Some`** — the absent-not-null shape that
/// keeps a step-less binding byte-identical to its pre-`s7g` serialisation, because the `run_id` is
/// a digest over those bytes. So the key's presence in `inputs/specs.json` *is* the transcription,
/// observed rather than inferred.
///
/// The run record is only written when the run completes, and **no fixture on this tree drives a
/// committing dispatch through the real binary** — the three transcript cases under
/// `tests/fixtures/exit/` are all failure cases, which is criterion (1)'s state showing through.
/// A **gate** run is the way through: design §4.5 has the kernel write the mark and **exit 0**, so
/// `gate-1` binds its node, opens its gate, exits 0 and writes a complete record. Measured at
/// `s7g`'s repair pass: `bug-1` exits **10** (*"the dispatch of `emit-report` … did not commit"*)
/// and writes **no** record at all, so the item choice here is load-bearing rather than incidental.
///
/// # 🛑 The anchors, and the one that matters most
///
/// **`specs.json` for a run that bound nothing is `{}`** — measured on this repository, where
/// selection assembles `"nodes": []`. Over `{}`, *"every entry carries steps"* is **vacuously
/// true**. The non-empty anchor below is therefore not a formality; without it this test would pass
/// on a tree where the binder had stopped binding entirely.
///
/// The negative control is the committed **pre-`s7g`** record at
/// `tests/fixtures/explain/two-activities/inputs/specs.json`: two nodes, same file format, and
/// **no `steps` key on either**. It proves the key is something this format can lack, so its
/// presence below discriminates rather than being an artefact of how the file is written.
#[test]
fn the_spec_a_real_run_binds_carries_its_steps() {
    // ---- NEGATIVE CONTROL FIRST, over a record written before the transcription existed. --------
    let pre_s7g: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(
            crate::fixtures_dir()
                .join("explain")
                .join("two-activities")
                .join("inputs")
                .join("specs.json"),
        )
        .expect("the committed pre-s7g specs.json is readable"),
    )
    .expect("the committed pre-s7g specs.json parses");
    let pre_nodes = pre_s7g
        .as_object()
        .expect("the committed record is a JSON object");
    assert!(
        !pre_nodes.is_empty(),
        "the committed pre-s7g record binds no node, so it cannot control anything"
    );
    for (node, spec) in pre_nodes {
        assert!(
            spec.get("steps").is_none(),
            "the committed PRE-`s7g` record already carries a `steps` key on `{node}`. That record \
             predates the transcription, so either it was regenerated — in which case this control \
             is dead and the assertion below no longer discriminates — or `steps` is written \
             unconditionally, which would make the live check below pass on a binder that \
             transcribes nothing."
        );
    }

    // ---- THE LIVE RUN. --------------------------------------------------------------------------
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();

    // The fixture's one activity declares one step, and its id is read from the fixture rather than
    // written here: if the fixture is re-authored, this anchor fires instead of the assertion below
    // quietly weakening to a shape that matches anything.
    let activity = std::fs::read_to_string(
        root.join(".redkiln")
            .join("catalog")
            .join("emit-report.toml"),
    )
    .expect("the installed fixture activity is readable");
    let declared: Vec<String> = activity
        .lines()
        .filter_map(|l| l.trim().strip_prefix("id"))
        .filter_map(|r| r.split('"').nth(1).map(str::to_owned))
        .collect();
    assert!(
        !declared.is_empty(),
        "no step id parsed out of the fixture activity — the anchor below would compare against \
         nothing:\n{activity}"
    );

    let out = crate::run::spawn_run(root, "gate-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(0),
        "PREMISE: a gate run exits 0 (design §4.5, AC-403) and only a completed run writes its \
         record. Without a record there is nothing to read and this test would assert nothing.\n\
         stderr:\n{stderr}"
    );

    // ---- THE RECORD. ------------------------------------------------------------------------
    let runs = crate::common_dir(root).join("redkiln").join("runs");
    assert!(
        runs.is_dir(),
        "the run exited 0 and wrote no run-record directory at {}",
        runs.display()
    );
    let mut specs_files: Vec<std::path::PathBuf> = std::fs::read_dir(&runs)
        .expect("the runs directory is readable")
        .flatten()
        .map(|e| e.path().join("inputs").join("specs.json"))
        .filter(|p| p.is_file())
        .collect();
    specs_files.sort();
    assert_eq!(
        specs_files.len(),
        1,
        "expected exactly one run record with an `inputs/specs.json`, found {}: {specs_files:?}",
        specs_files.len()
    );

    let specs: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&specs_files[0]).expect("specs.json reads"))
            .expect("specs.json parses");
    let nodes = specs.as_object().expect("specs.json is a JSON object");

    // 🛑 THE ANCHOR THAT CARRIES THIS TEST. `{}` is a real outcome — it is what this repository's
    // own `redkiln run` produces, because selection assembles zero nodes — and over `{}` the loop
    // below asserts nothing at all.
    assert!(
        !nodes.is_empty(),
        "the run bound NO node, so `specs.json` is `{{}}` and the assertion below is vacuous. \
         Either the fixture stopped selecting an activity, or the binder stopped binding."
    );

    // ---- EVERY BOUND NODE CARRIES ITS STEPS. -------------------------------------------------
    for (node, spec) in nodes {
        let steps = spec.get("steps").unwrap_or_else(|| {
            panic!(
                "ADR-0141 clause 4: the production binder bound node `{node}` with NO step list.\n  \
                 `RecordedSpec::to_json` writes `steps` only when `Some`, so an absent key means \
                 the composition root reached for `supplied` — by name, through an alias, or via a \
                 helper. On that binding `run_activity_steps` falls through to the pre-`s7g` \
                 activation dispatch and the runtime stops walking steps, silently.\n  \
                 The recorded spec was: {spec}"
            )
        });
        let steps = steps
            .as_array()
            .unwrap_or_else(|| panic!("`steps` on `{node}` is not an array: {steps}"));
        assert!(
            !steps.is_empty(),
            "`steps` on `{node}` is an EMPTY array. `run_activity_steps` refuses an empty \
             transcribed list by name, so this is not a working binding — and an empty array would \
             satisfy a bare `is_some()` check, which is why the contents are asserted here."
        );
        for step in steps {
            let id = step.get("id").and_then(serde_json::Value::as_str);
            let kind = step.get("kind").and_then(serde_json::Value::as_str);
            assert!(
                id.is_some() && kind.is_some(),
                "a recorded step on `{node}` carries no `id`/`kind` pair, so `steps` is present \
                 without being a step list: {step}"
            );
            let id = id.unwrap_or_default();
            assert!(
                declared.iter().any(|d| d == id),
                "node `{node}` recorded a step id {id:?} that the installed fixture activity does \
                 not declare (it declares {declared:?}). The list was synthesised somewhere rather \
                 than transcribed from the catalog."
            );
        }
    }
}

// ------------------------------------------------------------------------------------------------
// R1: PER-STEP SPECS AT THE COMPOSITION ROOT (`s8d` §8d.2, §8d.8 defect 4; record R1, proposed).
// *(Added 2026-09-26, `s8d` build, package P3.)* Each reads what `bind_nodes` produced for a REAL
// run of the binary over `pack-min`'s step-grain variant, through its run record's `specs.json`.
// `gate-1` is the item because a gate run exits 0 and writes the record (the reason
// `the_spec_a_real_run_binds_carries_its_steps` gives), and binding happens before the gate.
// ------------------------------------------------------------------------------------------------

/// Run `gate-1` over `pack-min`'s step-grain variant and read back its `specs.json`.
fn step_grain_specs() -> (
    tempfile::TempDir,
    serde_json::Map<String, serde_json::Value>,
) {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::StepGrain);
    let out = crate::run::spawn_run(repo.path(), "gate-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(
        out.status.code(),
        Some(0),
        "PREMISE: the gate run over the step-grain variant exits 0 and writes its record. A \
         binder that cannot bind one of its nodes refuses the run here, by name.\nstderr:\n{stderr}"
    );
    let runs = crate::common_dir(repo.path()).join("redkiln").join("runs");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&runs)
        .expect("the runs directory is readable")
        .flatten()
        .map(|e| e.path().join("inputs").join("specs.json"))
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    assert_eq!(files.len(), 1, "exactly one run record: {files:?}");
    let specs: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&files[0]).expect("specs.json reads"))
            .expect("specs.json parses");
    let map = specs.as_object().expect("an object").clone();
    (repo, map)
}

/// **§8d.8 defect 4: every selected node is bound, including one with NO agent step** — as an
/// explicit absence of agent specs (an empty `step_specs` map, no activation spec), never skipped
/// by a role-gated `continue` and never given a synthesised card.
#[test]
fn every_selected_node_is_bound_including_one_with_no_agent_step() {
    let (_repo, specs) = step_grain_specs();
    let nodes: std::collections::BTreeSet<&str> = specs.keys().map(String::as_str).collect();
    assert_eq!(
        nodes,
        ["emit-report", "no-agent-steps", "two-agents"]
            .into_iter()
            .collect(),
        "every node the selection assembled is bound — the zero-agent `no-agent-steps` included \
         (§8d.8 defect 4: a role-gated `continue` left it unbound, and the run's supplier then \
         refused it)"
    );
    let bare = &specs["no-agent-steps"];
    assert_eq!(
        bare.get("step_specs"),
        Some(&serde_json::json!({})),
        "a zero-agent node's binding says so EXPLICITLY — an empty `step_specs` map: {bare}"
    );
    assert!(
        bare.get("card_digest").is_none(),
        "and it carries no activation spec at all: a card digest here would be a synthesised card \
         (ADR-0143's forbids): {bare}"
    );
    assert_eq!(
        bare["steps"].as_array().map(Vec::len),
        Some(2),
        "its two steps are transcribed: {bare}"
    );
}

/// **N16: an agent step's `timeout_secs` resolves through the toolchain table and never exceeds the
/// activity budget** — `min(step, budget.wall_clock_secs)`, both arms pinned, at the binder that
/// computes it (M10: the test lives beside `bind_nodes`).
#[test]
fn an_agent_steps_timeout_resolves_through_the_toolchain_table_and_never_exceeds_the_activity_budget()
 {
    use rk_kernel::catalog::{Step, StepKind, StepTimeout};
    use rk_kernel::workflow::memo::VerbRegistry;

    let pin = "0123456789abcdef0123456789abcdef01234567";
    let registry = VerbRegistry::from_toml(&format!(
        "[verb]\nbuild = {{ outputs = \"BuildReport\", live = false, timeout = \"build_timeout\", \
         inputs = [], route = \"port\", side_effecting = false }}\n[toolchain.t]\n\
         agent_timeout = {{ secs = 180, measured = [61.5, 90, 72], rule = \"twice the slowest, up \
         to the minute\", pin = \"{pin}\", command = \"c\" }}\n"
    ))
    .expect("the test registry loads");
    let values = registry.values_for("t").expect("the table");
    let agent = |timeout: Option<StepTimeout>| Step {
        id: "draft".to_owned(),
        kind: StepKind::Agent,
        run: None,
        output_type: Some("ImplReport".to_owned()),
        role: Some("implementer".to_owned()),
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: timeout,
        inputs: std::collections::BTreeMap::new(),
        on_false_to: None,
        repeat_until: None,
        escalate_question: None,
        on_false: None,
        on_exhausted: None,
        emit: None,
    };
    let bound = |timeout: Option<StepTimeout>, budget: Option<u64>| {
        rk_cli::verbs::run::agent_step_timeout_secs(&agent(timeout), &values, 900, budget)
    };
    let table = || Some(StepTimeout::Toolchain("agent_timeout".to_owned()));
    // ---- THE TABLE ARM: a `${toolchain.*}` bound resolves through 4a's table ... ---------------
    assert_eq!(
        bound(table(), Some(900)),
        Ok(180),
        "the table's 180 stands under a 900 budget"
    );
    // ... and the budget still bounds it.
    assert_eq!(
        bound(table(), Some(100)),
        Ok(100),
        "never above the activity budget (N16)"
    );
    // ---- THE LITERAL ARM, both sides of the budget. ---------------------------------------------
    assert_eq!(bound(Some(StepTimeout::Literal(5000)), Some(900)), Ok(900));
    assert_eq!(bound(Some(StepTimeout::Literal(60)), Some(900)), Ok(60));
    // ---- UNDECLARED: the activation's own per-activity value (ADR-0140 decision 5), bounded. -----
    assert_eq!(bound(None, Some(900)), Ok(900));
    assert_eq!(bound(None, Some(30)), Ok(30));
    // ---- REFUSALS BY NAME. ----------------------------------------------------------------------
    let absent = bound(
        Some(StepTimeout::Toolchain("no_such_key".to_owned())),
        Some(900),
    );
    assert!(
        absent.as_ref().is_err_and(|e| e.contains("no_such_key")),
        "an unresolved key is refused by name (D21-42 4a), never defaulted: {absent:?}"
    );
    assert!(
        bound(Some(StepTimeout::Literal(0)), Some(900)).is_err(),
        "a zero bound is refused (RT-5)"
    );
    let unbudgeted = bound(Some(StepTimeout::Literal(60)), None);
    assert!(
        unbudgeted
            .as_ref()
            .is_err_and(|e| e.contains("wall_clock_secs")),
        "an activity with no declared budget has no ceiling for the bound, and is refused: \
         {unbudgeted:?}"
    );

    // ---- AND AT THE BINDER, over a real run's record. --------------------------------------------
    let (_repo, specs) = step_grain_specs();
    let steps = &specs["two-agents"]["step_specs"];
    assert_eq!(
        steps["draft"]["timeout_secs"],
        serde_json::json!(900),
        "`draft` declares 5000 and its budget is 900: the per-step spec carries 900 (N16): {steps}"
    );
    assert_eq!(
        steps["review"]["timeout_secs"],
        serde_json::json!(60),
        "`review` declares 60, under the budget, and it stands: {steps}"
    );
}

/// **R1: each agent step carries its OWN card and schema in the run record** — its role's card, its
/// `outputs = "value:<Kind>"` schema — never the node's first role, and never `provides[0]` (M18).
#[test]
fn each_agent_step_carries_its_own_card_and_schema_in_the_run_record() {
    let (_repo, specs) = step_grain_specs();
    let two = &specs["two-agents"];
    let steps = two
        .get("step_specs")
        .and_then(serde_json::Value::as_object)
        .unwrap_or_else(|| {
            panic!("the two-agent node carries no per-step specs (R1; D21-42 3a): {two}")
        });
    assert_eq!(
        steps.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["draft", "review"],
        "one per-step spec per AGENT step, keyed by step id: {two}"
    );
    let card = |id: &str| {
        steps[id]["card_digest"]
            .as_str()
            .expect("a card digest")
            .to_owned()
    };
    let schema = |id: &str| steps[id]["schema"].as_str().expect("a schema").to_owned();
    assert_ne!(
        card("draft"),
        card("review"),
        "the two steps nominate different roles, so they carry different cards"
    );
    assert_eq!(
        card("draft"),
        specs["emit-report"]["card_digest"]
            .as_str()
            .expect("emit-report's card")
            .to_owned(),
        "`draft`'s role is `implementer`, emit-report's one card"
    );
    assert_eq!(
        schema("draft"),
        "ImplReport",
        "`draft` outputs value:ImplReport"
    );
    assert_eq!(
        schema("review"),
        "Findings",
        "`review` outputs value:Findings"
    );
    assert!(
        steps
            .values()
            .all(|s| s["schema"] != serde_json::json!("ChangeSummary")),
        "no step's schema is the activity's provides[0] (M18): {two}"
    );
}
