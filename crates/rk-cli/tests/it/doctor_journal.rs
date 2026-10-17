//! **`doctor` over the store** (`s8f` §8f.9 and §8f.5; D21-42 12c; D21-43 N12; the telemetry-key
//! record ADR-0190 decisions 7 and 8, proposed). *(Added 2026-09-29, s8f build, package
//! S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL.)*
//!
//! Three things this module pins, each in its own test so no arm hides behind another:
//! - **the journal source** — `Ctx::build` takes it explicitly; the verb passes the store's rows
//!   (the whole journal) and never names the `events.jsonl` fixture source, which a fixture test
//!   still may;
//! - **`RK-D-HEADROOM-MARGIN`'s five-hour reading** — the latest reading that answers for
//!   `five_hour`, read exactly as the guard's `RateLimitReading::ticks(Window::FiveHour)` reads it;
//! - **`RK-D-UNREAD` over real rows** — every `artifact.write` / `artifact.read` a `FakeHarness`
//!   lane appends carries the `stage` and `run` the rule keys on, and its widened `run_of` reads a
//!   string or integer `run` by canonical text and a `null` one as an unknown run (D21-48 X11).
//!
//! These are NOT in `tests/it/doctor.rs`: D21-42 18 keeps `rk-fix-doctor-test-count` as *"the
//! untouched fixture item, prose defect included"*, and a test added there would move the count
//! that item is about (§8f.9).

use std::path::{Path, PathBuf};

use rk_cli::doctor::rules::RULES;
use rk_cli::doctor::{Ctx, Finding, JournalSource};
use rk_cli::exit::Exit;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::lane_e2e::{self, GATE_ITEM, ScriptedFactory};
use crate::support;

/// The fixed `now` every in-process `Ctx` here carries (no rule reads a clock).
const NOW: i64 = 1_800_000_000;

/// A stub probe: every in-process `Ctx` here is about the journal, never the installed CLI.
struct Pinned;

impl rk_cli::doctor::probe::VersionProbe for Pinned {
    fn version(&self, _program: &str) -> Result<String, rk_cli::doctor::probe::ProbeError> {
        Ok(rk_adapter_claude::config::TESTED_CLI_VERSION.to_owned())
    }
}

/// `Ctx::build` over the repository at `root`, through the one builder the verb uses, with the
/// store the root would hand in and the journal `source` named explicitly.
fn ctx_over(root: &Path, source: JournalSource, window: Option<u32>) -> Ctx {
    let config_src = std::fs::read_to_string(root.join(".redkiln").join("config.toml"))
        .expect("the repository carries a .redkiln/config.toml");
    let mut config = rk_cli::config::parse(&config_src).expect("the ConfigFile parses");
    if let Some(window) = window {
        config.doctor.unread_window_runs = window;
    }
    let repo = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let store = Some(rk_cli::adapters::open_store(&repo.exec, &repo.worktree));
    Ctx::build(
        root.to_path_buf(),
        repo.common_dir,
        config,
        support::clean_env(),
        store,
        Box::new(Pinned),
        NOW,
        source,
    )
}

/// One rule's findings over `ctx`.
fn fire(rule_id: &str, ctx: &Ctx) -> Vec<Finding> {
    let rule = RULES
        .iter()
        .find(|r| r.id == rule_id)
        .unwrap_or_else(|| panic!("{rule_id} is in RULES"));
    (rule.check)(ctx)
}

/// A `rate_limit_reading` row.
const fn reading(payload: serde_json::Value) -> Event {
    Event::new(EventKind::RateLimitReading, payload)
}

/// Spawn `redkiln -C <repo> doctor` with the fixture child's two names on its `Command`.
fn spawn_doctor(repo: &Path, markers: &Path) -> std::process::Output {
    let mut cmd = crate::redkiln_command(crate::Launch::Withheld);
    cmd.arg("-C")
        .arg(repo)
        .arg("doctor")
        .env("RK_FIXTURE_MODE", "exit:0")
        .env("RK_FIXTURE_MARKER_DIR", markers)
        .stdin(std::process::Stdio::null());
    support::output_bounded(&mut cmd)
}

/// **A present store IS the journal — through the binary, with no `events.jsonl` in the tree**
/// (§8f.9; ADR-0190 decision 7). The `RK-D-HEADROOM-MARGIN` fixture, its `events.jsonl` removed,
/// its STORE seeded with the same three readings: the shipped `doctor` reports the margin rule
/// from the store's rows. The fixture's pack declares `five_hour = 10`, and the last reading is 99.
///
/// Red by `M20` (the verb passes the fixture source: with no `events.jsonl` the journal is empty
/// and the rule reads `ok`).
#[test]
fn a_present_store_is_the_journal() {
    let dir = crate::doctor::materialise("RK-D-HEADROOM-MARGIN");
    let root = dir.path();
    let events = root.join("events.jsonl");
    let rows = support::cost_rows(&events);
    assert_eq!(rows.len(), 3, "PREMISE: the fixture's three readings");
    std::fs::remove_file(&events).expect("events.jsonl is removed");
    assert!(!events.exists(), "PREMISE: no events.jsonl in the tree");
    support::seed_store(root, &rows);

    let markers = tempfile::tempdir().expect("a marker directory");
    let out = spawn_doctor(root, markers.path());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let margin: Vec<&str> = stdout
        .lines()
        .filter(|l| l.starts_with("RK-D-HEADROOM-MARGIN "))
        .collect();
    assert_eq!(
        margin.len(),
        1,
        "one RK-D-HEADROOM-MARGIN line:\n{stdout}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        margin[0].starts_with("RK-D-HEADROOM-MARGIN warning ") && margin[0].contains(" 99 of 100"),
        "the shipped doctor did not read the store's readings: {:?}",
        margin[0]
    );
}

/// **`events.jsonl` is read only in fixture mode** (§8f.9): over ONE repository holding both an
/// `events.jsonl` and a store with DIFFERENT rows, `JournalSource::Store` reads the store's rows
/// and none of the file's, and `JournalSource::FixtureEventsJsonl` reads the file's and none of the
/// store's.
///
/// Red by: the store source still reading the file (the pre-`s8f` builder).
#[test]
fn events_jsonl_is_read_only_in_fixture_mode() {
    let dir = crate::doctor::materialise("RK-D-HEADROOM-MARGIN");
    let root = dir.path();
    let store_rows = vec![reading(
        serde_json::json!({ "windows": { "five_hour": 12, "seven_day": 34 } }),
    )];
    support::seed_store(root, &store_rows);

    let fixture = ctx_over(root, JournalSource::FixtureEventsJsonl, None);
    let fives: Vec<i64> = fixture
        .journal
        .iter()
        .filter_map(|r| r.payload.get("windows")?.get("five_hour")?.as_i64())
        .collect();
    assert_eq!(fives, vec![40, 70, 99], "fixture mode reads events.jsonl");

    let store = ctx_over(root, JournalSource::Store, None);
    let fives: Vec<i64> = store
        .journal
        .iter()
        .filter_map(|r| r.payload.get("windows")?.get("five_hour")?.as_i64())
        .collect();
    assert_eq!(
        fives,
        vec![12],
        "the store source reads the store's rows and never the file's"
    );
    assert!(
        store.journal.iter().all(|r| r.kind == "rate_limit_reading"),
        "the row's kind is its wire name: {:?}",
        store.journal
    );
}

/// A fixture-mode `Ctx` over the `RK-D-HEADROOM-MARGIN` fixture whose `events.jsonl` holds `rows`.
fn margin_over(rows: &[serde_json::Value]) -> Vec<Finding> {
    let dir = crate::doctor::materialise("RK-D-HEADROOM-MARGIN");
    let text: String = rows
        .iter()
        .map(|p| {
            format!(
                "{}\n",
                serde_json::json!({ "kind": "rate_limit_reading", "payload": p })
            )
        })
        .collect();
    std::fs::write(dir.path().join("events.jsonl"), text).expect("events.jsonl is written");
    fire(
        "RK-D-HEADROOM-MARGIN",
        &ctx_over(dir.path(), JournalSource::FixtureEventsJsonl, None),
    )
}

/// **A flat last reading no longer silences the margin rule** (D21-42 12c; ADR-0190 decision 8): a
/// flat reading (`windows: null`) whose `rate_limit_type` is `five_hour` — or `null`, which the
/// guard counts against both windows — answers for `five_hour`, as `ticks(Window::FiveHour)` reads
/// it. At `99` it is inside the pack's margin of 10, and the rule fires on it.
///
/// Red by `M21` (the rule reverts to `.last()`'s `windows.five_hour`, which a flat reading has not).
#[test]
fn headroom_margin_fires_on_a_flat_last_reading() {
    for kind in [serde_json::json!("five_hour"), serde_json::Value::Null] {
        let found = margin_over(&[
            serde_json::json!({ "windows": { "five_hour": 40, "seven_day": 20 } }),
            serde_json::json!({ "windows": null, "utilization": 99, "rate_limit_type": kind }),
        ]);
        assert_eq!(
            found.len(),
            1,
            "a flat last reading typed {kind} is silenced: {found:?}"
        );
        assert!(
            found[0].detail.contains(" 99 of 100"),
            "the rule did not read the flat reading's utilisation: {:?}",
            found[0].detail
        );
    }
}

/// **The LATEST reading that answers for `five_hour`, not the last row** (ADR-0190 decision 8): a
/// flat reading that names `seven_day` answers for no five-hour window and is skipped, so a unified
/// `99` before it still fires; and a later five-hour-bearing reading outside the margin silences an
/// earlier one inside it, so the rule is the LATEST such reading and not the maximum.
///
/// Red by `M21`; by a rule that took the maximum.
#[test]
fn headroom_margin_reads_the_latest_five_hour_bearing_reading() {
    let found = margin_over(&[
        serde_json::json!({ "windows": { "five_hour": 99, "seven_day": 20 } }),
        serde_json::json!({ "windows": null, "utilization": 5, "rate_limit_type": "seven_day" }),
    ]);
    assert_eq!(
        found.len(),
        1,
        "a flat seven_day reading silenced the five-hour margin: {found:?}"
    );
    assert!(
        found[0].detail.contains(" 99 of 100"),
        "{:?}",
        found[0].detail
    );

    let later = margin_over(&[
        serde_json::json!({ "windows": { "five_hour": 99, "seven_day": 20 } }),
        serde_json::json!({ "windows": null, "utilization": 10, "rate_limit_type": "five_hour" }),
        serde_json::json!({ "windows": null, "utilization": 5, "rate_limit_type": "seven_day" }),
    ]);
    assert!(
        later.is_empty(),
        "the latest five-hour-bearing reading is 10, outside the margin: {later:?}"
    );
}

/// Every code line of every `.rs` file under `root`, with its path.
fn code_lines_under(root: &Path) -> Vec<(PathBuf, usize, String)> {
    crate::rs_files(root)
        .into_iter()
        .flat_map(|path| {
            let src = std::fs::read_to_string(&path).expect("a source file reads");
            crate::code_lines(&src)
                .into_iter()
                .map(move |(n, line)| (path.clone(), n, line))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// Code lines of `lines` naming `needle`.
fn hits<'a>(
    lines: &'a [(PathBuf, usize, String)],
    needle: &str,
) -> Vec<&'a (PathBuf, usize, String)> {
    lines
        .iter()
        .filter(|(_, _, l)| l.contains(needle))
        .collect()
}

/// **The verb never selects the fixture source** (§8f.9; ADR-0190's forbids) — a CODE-LINES-ONLY
/// scan, paired with [`a_present_store_is_the_journal`], because a source lint passes on code
/// nothing calls. Over every `.rs` file under `crates/rk-cli/src`, `JournalSource::FixtureEventsJsonl`
/// appears on no code line outside the builder's own module (`src/doctor/mod.rs`, which defines
/// and reads it), and `src/verbs/doctor.rs` names `JournalSource::Store` on exactly one.
///
/// Self-controlled first: the scan finds a planted code-line hit and drops a planted comment.
#[test]
fn the_doctor_verb_never_selects_the_fixture_source() {
    let planted = "let s = JournalSource::FixtureEventsJsonl;\n// JournalSource::FixtureEventsJsonl in prose\n";
    let lines: Vec<(PathBuf, usize, String)> = crate::code_lines(planted)
        .into_iter()
        .map(|(n, l)| (PathBuf::from("planted.rs"), n, l))
        .collect();
    assert_eq!(
        hits(&lines, "JournalSource::FixtureEventsJsonl").len(),
        1,
        "CONTROL: the scan keeps the planted code line and drops the planted comment"
    );

    let src = crate::src_dir();
    let all = code_lines_under(&src);
    assert!(
        all.len() > 1000,
        "PREMISE: the scan read rk-cli's source ({} lines)",
        all.len()
    );
    let builder = src.join("doctor").join("mod.rs");
    let fixture: Vec<_> = hits(&all, "JournalSource::FixtureEventsJsonl")
        .into_iter()
        .filter(|(p, _, _)| *p != builder)
        .collect();
    assert!(
        fixture.is_empty(),
        "the fixture source is named outside its builder: {fixture:?}"
    );
    let verb = src.join("verbs").join("doctor.rs");
    let store: Vec<_> = hits(&all, "JournalSource::Store")
        .into_iter()
        .filter(|(p, _, _)| *p == verb)
        .collect();
    assert_eq!(
        store.len(),
        1,
        "the verb names the store source exactly once: {store:?}"
    );
}

// ------------------------------------------------------ `RK-D-UNREAD` over a `FakeHarness` lane

/// `emit-report` with an `emit` of its agent step's output (`landing.rs`'s render lane).
const EMIT_REPORT: &str = r#"[activity]
id           = "emit-report"
version      = 1
stage        = "implement"
provides     = ["ImplReport"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "check"
kind    = "agent"
role    = "implementer"
outputs = "value:ImplReport"

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { ImplReport = "${check.outputs}" }
"#;

/// ~~An `implement`-stage reader of `artifact:ImplReport`, after `emit-report` by its `requires`: so
/// the emitted kind is READ at the stage that WROTE it, which is what `RK-D-UNREAD`'s kind-per-stage
/// predicate (D-08) counts as read.~~ *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE;
/// D21-111 item 5: the workaround is dropped.)* `review-min` whose agent step reads
/// `artifact:ImplReport` at `review` — the stage AFTER the one that wrote it (`landing.rs`'s render
/// lane). Under D21-111 a read at any stage consumes the write.
const REVIEW_MIN: &str = r#"[activity]
id           = "review-min"
version      = 1
stage        = "review"
provides     = ["Findings"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "check"
kind    = "agent"
role    = "implementer"
outputs = "value:Findings"
inputs  = { report = "artifact:ImplReport" }
"#;

/// **Every artifact row a `FakeHarness` lane writes carries `stage` and `run`, and `RK-D-UNREAD`
/// counts them** (D21-47, review M2; §8f.5). The fixture lane's `lane-1` walks through `run`:
/// invocation 1 opens its abstention gate; `answer … vouch` ATTACHES a `Findings` root; invocation 2
/// stores the root, emits `ImplReport` at `implement` and reads it at `review` (`review-min`'s
/// `artifact:ImplReport` input; *dated 2026-09-29, R1FIX-8f-A-CODE, D21-111: struck "reads it back
/// there (`check-report`'s input)"*), and walks to the lane's end. Every `artifact.write` and
/// `artifact.read` row then carries a string `stage` and a `run`, and `RK-D-UNREAD` over the STORE's
/// journal at `unread_window_runs = 1` is silent on the emitted-and-read kind and fires on the
/// attached one, which nothing reads.
///
/// Red by: `run_of` reading integers only (the attach row's string `run` is passed over and nothing
/// fires); `M44` (the writer's row omits `stage`); `M45` (`answer`'s attach row omits `run`).
#[test]
fn unread_counts_every_artifact_row_a_fake_harness_lane_writes() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let redkiln = root.join(".redkiln");
    std::fs::write(
        redkiln.join("catalog").join("emit-report.toml"),
        EMIT_REPORT,
    )
    .expect("the catalog is written");
    std::fs::write(redkiln.join("catalog").join("review-min.toml"), REVIEW_MIN)
        .expect("the catalog is written");
    let processes = redkiln.join("processes");
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("vouch.toml"),
        r#"[question."unknown_facet:has_repro"]
text = "Does the item reproduce? `vouch` attaches the human's findings."
options = ["approve", { id = "vouch", attaches = "Findings" }]
"#,
    )
    .expect("the question is written");

    let item = GATE_ITEM;
    let run = |script| {
        lane_e2e::ran(
            rk_cli::verbs::run::run(
                &lane_e2e::cli(root, item),
                &lane_e2e::args(item),
                &support::clean_env(),
                lane_e2e::seam(&ScriptedFactory::new(script)),
                &rk_ports::cancel::CancelToken::new(),
            ),
            item,
        )
    };
    assert_eq!(
        run(Vec::new()),
        Exit::Success,
        "PREMISE: the abstention gate opens"
    );
    let brief = root.join("notes").join("brief.json");
    std::fs::create_dir_all(brief.parent().expect("a parent")).expect("notes/");
    std::fs::write(&brief, b"{\"items\":[]}\n").expect("the root is written");
    let answered = rk_cli::verbs::answer::run(
        &rk_cli::cli::Cli {
            cwd: Some(root.to_path_buf()),
            command: rk_cli::cli::Command::Status,
        },
        &rk_cli::cli::AnswerArgs {
            item: item.to_owned(),
            option: "vouch".to_owned(),
            attach: vec![rk_cli::cli::Attachment {
                kind: "Findings".to_owned(),
                path: brief,
            }],
            explain_sufficed: rk_cli::cli::YesNo::Yes,
            toml_opened: rk_cli::cli::YesNo::No,
        },
        // *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; s8h §8h.7.)* The binary's
        // `answer` composition root, so the fixture launch record, explicitly.
        &support::launched_env(),
        rk_cli::verbs::Startup::real(),
    )
    .expect("answer does not halt");
    assert_eq!(answered, Exit::Success, "PREMISE: the root is attached");
    assert_eq!(
        run(vec![
            lane_e2e::implement(),
            lane_e2e::review(),
            lane_e2e::review()
        ]),
        Exit::Success,
        "invocation 2 walks the lane"
    );
    assert_eq!(
        lane_e2e::story_stages(root, item)
            .last()
            .map(String::as_str),
        Some("closed"),
        "PREMISE: the lane walks to its end"
    );

    // ---- every artifact row carries a string `stage` and a `run` -------------------------------
    let rows = support::store_rows(root);
    let artifact_rows: Vec<&Event> = rows
        .iter()
        .filter(|e| matches!(e.kind, EventKind::ArtifactWrite | EventKind::ArtifactRead))
        .collect();
    let of = |kind: EventKind, artifact: &str| {
        artifact_rows
            .iter()
            .filter(|e| e.kind == kind)
            .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(artifact))
            .count()
    };
    assert!(
        of(EventKind::ArtifactWrite, "ImplReport") >= 1,
        "PREMISE: the emit wrote: {artifact_rows:?}"
    );
    assert!(
        of(EventKind::ArtifactRead, "ImplReport") >= 1,
        "PREMISE: the input was read: {artifact_rows:?}"
    );
    assert!(
        artifact_rows
            .iter()
            .any(|e| e.kind == EventKind::ArtifactWrite && e.payload.get("attached_path").is_some()),
        "PREMISE: the attach row: {artifact_rows:?}"
    );
    for row in &artifact_rows {
        assert!(
            row.payload
                .get("stage")
                .is_some_and(serde_json::Value::is_string),
            "an artifact row without a string `stage`: {row:?}"
        );
        assert!(
            row.payload
                .get("run")
                .is_some_and(|r| r.is_string() || r.is_u64()),
            "an artifact row without a `run`: {row:?}"
        );
    }
    let read_at = |artifact: &str| -> Vec<String> {
        artifact_rows
            .iter()
            .filter(|e| e.kind == EventKind::ArtifactRead)
            .filter(|e| e.payload.get("kind").and_then(serde_json::Value::as_str) == Some(artifact))
            .filter_map(|e| e.payload.get("stage").and_then(serde_json::Value::as_str))
            .map(str::to_owned)
            .collect()
    };
    assert!(
        !read_at("ImplReport").is_empty() && read_at("ImplReport").iter().all(|s| s == "review"),
        "PREMISE: ImplReport is read only at `review`, the stage after the one that wrote it: {:?}",
        read_at("ImplReport")
    );

    // ---- RK-D-UNREAD over the STORE's journal, at a window of one run --------------------------
    let ctx = ctx_over(root, JournalSource::Store, Some(1));
    assert!(
        ctx.journal.len() == rows.len(),
        "PREMISE: the Ctx's journal is the store's {} rows, not {}",
        rows.len(),
        ctx.journal.len()
    );
    let found = fire("RK-D-UNREAD", &ctx);
    let named: Vec<&str> = found.iter().map(|f| f.detail.as_str()).collect();
    assert!(
        found.iter().any(|f| f.detail.contains("`Findings`")),
        "the attached kind, which nothing reads, is not flagged: {named:?}"
    );
    assert!(
        !found.iter().any(|f| f.detail.contains("`ImplReport`")),
        "the kind written at `implement` and read at `review` is flagged (D21-111): {named:?}"
    );
}

/// **A `null` `run` is an unknown run, never a match** (D21-48 X11: *"RK-D-UNREAD reads null as an
/// unknown run, never as a match"*; §8f.5). A hand-built journal at `unread_window_runs = 2` writes
/// one unread `(stage, kind)` once under the string run `implement-aaaa` and twice under `run:
/// null`: ONE known run, below the window, and the rule is silent. The control adds a second string
/// run, `implement-bbbb`, and the rule fires naming two runs; an INTEGER `0` and a STRING `"0"` in a
/// third journal are two runs (the canonical text keeps them distinct).
///
/// Red by: `run_of` reading integers only (the control does not fire); `M52` (`null` keyed by its
/// text, so the first arm counts two runs and fires).
#[test]
fn unread_reads_a_null_run_as_an_unknown_run_never_a_match() {
    let write = |run: serde_json::Value| {
        serde_json::json!({
            "kind": "artifact.write",
            "payload": { "kind": "Findings", "stage": "implement", "run": run },
        })
    };
    let over = |rows: &[serde_json::Value]| {
        let dir = crate::doctor::materialise("clean");
        let text: String = rows.iter().map(|r| format!("{r}\n")).collect();
        std::fs::write(dir.path().join("events.jsonl"), text).expect("events.jsonl is written");
        fire(
            "RK-D-UNREAD",
            &ctx_over(dir.path(), JournalSource::FixtureEventsJsonl, Some(2)),
        )
    };

    let silent = over(&[
        write(serde_json::json!("implement-aaaa")),
        write(serde_json::Value::Null),
        write(serde_json::Value::Null),
    ]);
    assert!(
        silent.is_empty(),
        "two null-run rows were counted as a run: {silent:?}"
    );

    let control = over(&[
        write(serde_json::json!("implement-aaaa")),
        write(serde_json::Value::Null),
        write(serde_json::Value::Null),
        write(serde_json::json!("implement-bbbb")),
    ]);
    assert_eq!(
        control.len(),
        1,
        "two string runs reach the window: {control:?}"
    );
    assert!(
        control[0].detail.contains("in 2 runs"),
        "{:?}",
        control[0].detail
    );

    let typed = over(&[write(serde_json::json!(0)), write(serde_json::json!("0"))]);
    assert_eq!(
        typed.len(),
        1,
        "an integer 0 and a string \"0\" are two runs: {typed:?}"
    );
}

/// **D21-111: `RK-D-UNREAD`'s read side is keyed by KIND** — any stage and any run consumes a write
/// (*"It becomes the set of `kind` over every `artifact.read` row, whatever its stage or run"*).
/// *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE; D21-111 item 5, taken provisionally
/// under D21-105.)*
///
/// `unread/cross-stage-51.jsonl`: 51 runs, each writing `ImplReport` at `implement` and reading it
/// at `review` (a `review-…` run, so no `(stage, kind)` or `(kind, run)` key could pair them), beside
/// a control kind `never-read` written at `implement` and read nowhere. At `unread_window_runs =
/// 50` exactly ONE finding appears, and it is the control's.
///
/// Red by the per-stage read key (both kinds flagged), M-a (that key restored), M-b (keyed on
/// `(kind, run)`) and M-c (the read filter dropped: the control and `ImplReport` both fire).
#[test]
fn unread_reads_a_cross_stage_read_as_consuming_the_write() {
    let dir = crate::doctor::materialise("clean");
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("unread")
        .join("cross-stage-51.jsonl");
    let bytes = std::fs::read(&src).expect("the fixture reads");
    assert_eq!(
        bytes.iter().filter(|b| **b == b'\n').count(),
        153,
        "PREMISE: 51 runs of three rows"
    );
    std::fs::write(dir.path().join("events.jsonl"), bytes).expect("events.jsonl is written");
    let found = fire(
        "RK-D-UNREAD",
        &ctx_over(dir.path(), JournalSource::FixtureEventsJsonl, Some(50)),
    );
    let named: Vec<&str> = found.iter().map(|f| f.detail.as_str()).collect();
    assert_eq!(
        found.len(),
        1,
        "exactly one finding, the never-read control: {named:?}"
    );
    assert!(
        found[0].detail.contains("`never-read`") && found[0].detail.contains("in 51 runs"),
        "the one finding is not the control's: {named:?}"
    );
}
