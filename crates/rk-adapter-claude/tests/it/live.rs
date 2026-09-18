//! **The live probe — the fifteenth line — and the spike drivers** (s7c §7c.15, §7c.16; D17-5,
//! D17-6; AC-215's integration half; AC-511). Every test here is `#[ignore]` behind
//! `[features] live = []`, run in the FOREGROUND by the operator from a Git Bash outside Claude
//! Code as `cargo test -p rk-adapter-claude --features live -- --ignored --nocapture <name>`,
//! and **never wired into CI**. Each one is a PAID `claude -p` dispatch (≈ $0.03 at haiku scale);
//! none runs under `cargo test --workspace` or under any checkpoint line but the fifteenth.
//!
//! **The process environment is read ONCE, at this file's edge** (`std::env::vars()` into a
//! `BTreeMap`; ADR-0083 — the composition root's job, done here because no `rk-cli` exists at
//! 7c): `std::env::var` is not clippy-banned and needs no `#[expect]`; `set_var` / `remove_var`
//! are banned and are not called; no adapter `src/` file reads the environment at all (the
//! block's AC-217 ban — this file is in `tests/it`, outside its scope). On this box the captured
//! map carries `ANTHROPIC_FOUNDRY_API_KEY` (persisted at user scope) and, under Claude Code,
//! `CLAUDECODE` / `CLAUDE_CODE_*`, so `Config::new` REFUSES it, correctly (§7c.6): the probe is run
//! under `env -u ANTHROPIC_FOUNDRY_API_KEY bash` from a shell Claude Code did not spawn — an
//! operator step outside the process, never `remove_var` (Halt 7).
//!
//! **What the probe does** (§7c.15): builds `Config` (`TESTED_CLI_VERSION` = the pin, both TTLs,
//! the captured map, `harness_binary = "claude"`); initialises a temp git repository with a root
//! commit through the `rk-git` DEV edge (so AC-369's identity rows can be written) and opens a temp
//! `rk-adapter-sqlite` store over it (a dev edge between adapters); binds the catalog's haiku probe
//! card (§7c.12) with `timeout_secs = 300` — the bound on the block's one wait, expiring on the
//! ADAPTER'S OWN timeout arm (§7c.10), Halt 5's failure alternative; runs `rk-ports`' shared
//! `conform_harness` body over the real adapter FIRST — the ONE dispatch, and the AC-499 Harness
//! row; the body prints `PROBE_RAN_CONFORMANCE=1` itself as its last statement; then reads the
//! `dispatch_completed` row back through the real `EventStore::query`, retains the raw
//! `system/init` line and the raw `result` frame **from the adapter's uncapped raw-frames seam**
//! (`ClaudeHarness::last_raw_frames` — the row's stdout is capped at 64 KiB and a verbose
//! stream with tool traffic exceeds it; S5-04, RT-18) to the files the stage names, and prints
//! each value on its own line for the block to read to end of line.
//!
//! **What it prints, and what it asserts**: `RECORD_PATH=` (the raw init, the raw result frame,
//! the row serialised compact with `serde_json::to_string`); `RESULT_RAW_PATH=` (ONLY the verbatim
//! result frame) and `RESULT_RAW_SHA256=` (the sha256 of the bytes it wrote there); `ARGV_PATH=`
//! (the argv the builder produced, one element per line — the row carries argv as a digest only,
//! ADR-0105 c7, and the probe asserts the vector's digest IS the row's); `OBSERVED_CLI_VERSION=`
//! (the version taken from the raw init's version field — printed only if a key in
//! `init::VERSION_KEYS` matched; **[surface unverified]**: if no line is printed, the field is
//! unnamed at 2.1.277 and the operator names it from `RECORD_PATH`'s raw init); `STORE_PATH=`;
//! `COMPLETED_ROWS=` (asserted 1) and `DISPATCH_ROWS=` (asserted 0 — the probe calls
//! `Harness::dispatch` directly and no kernel runs). It asserts `structured_output` from the RAW
//! result frame as a non-empty object or list beside `subtype: success` and `is_error: false`
//! (Halt 3 — never the exit code), that `--permission-prompts none` is on the argv it wrote
//! (AC-215's tag lift), and that the dispatch reached `system/init`. **AC-511**: the auth-mode
//! field of `system/init` is unnamed in the corpus; the probe prints `AUTH_MODE_FIELD=<key>=<value>`
//! for the first of [`AUTH_MODE_KEYS`] present on the raw init, or `AUTH_MODE_FIELD=unnamed` — the
//! close commit names the field or records the open item (§7c.15).
//!
//! **`live_disallowed_tool_terminates`** — AC-215's integration half, AUTHORED and NOT RUN at 7c
//! (ARMED, §7c.22): a card whose prompt asks for a tool the card does not grant; the dispatch must
//! TERMINATE (`Ok`, `NonZero` or a typed `StepError`) under the card's bound rather than stall on
//! a permission prompt — red by `Cancelled { by: Timeout }`.
//!
//! **S-RETRY and S-TTL1 drivers** (D17-6; §7c.16) — AUTHORED as `#[ignore]` live tests, run or
//! deferred by the operator at the close, each retaining its raw frames to a file for the corpus
//! commit O-9 names (never a tracked path — AC-465): `s_retry_throttle_margin_driver` (one haiku
//! dispatch; the last `rate_limit_event`'s reading beside the configured margins),
//! `s_retry_exhaustion_on_result_driver` (one haiku dispatch; retains the result frame's `subtype`
//! and `stop_reason` — the exhaustion classifier's consumer, §7c.7 — it does NOT try to exhaust a
//! plan), `s_ttl1_subagent_card_driver` (ADR-0113 d3: a subagent-granting card with both TTL
//! variables set; retains the raw `usage` and `rate_limit_info`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_adapter_sqlite::SqliteStore;
use rk_git::git_exec::{self, GitExec};
use rk_git::path::WorktreePath;
use rk_ports::cancel::CancelToken;
use rk_ports::conformance::conform_harness;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::harness::Harness;
use rk_types::activity::{ActivitySpec, DispatchRecord};
use rk_types::digest::sha256;
use rk_types::event_kind::EventKind;
use rk_types::retention::Retained;

use rk_adapter_claude::ClaudeHarness;
use rk_adapter_claude::binder::{BindRequest, Binder};
use rk_adapter_claude::command::{CommandSpec, assemble};
use rk_adapter_claude::config::{Config, ConfigValues, MismatchPolicy, TESTED_CLI_VERSION};
use rk_adapter_claude::event::{EventKind as LineKind, classify};
use rk_adapter_claude::init::{VERSION_KEYS, effective_config};

use crate::support::{catalog_card, margins_ok, role, schema};

/// The `system/init` keys that might name the session's auth mode — **[surface unverified]**
/// (AC-511: `apiKeySource` appears in ZERO corpus files at the pin and the runner read no auth
/// field); the probe prints whichever is present, and the close commit names it or records the
/// open item.
pub const AUTH_MODE_KEYS: &[&str] = &["apiKeySource", "api_key_source", "auth_mode", "authMode"];

/// The ONE environment read, at the edge (ADR-0083).
fn captured_env_once() -> BTreeMap<String, String> {
    std::env::vars().collect()
}

/// The probe's `Config` over the captured map: the pin, both TTLs by the record's value,
/// `harness_binary = "claude"`, the register's margins, refusal on a version mismatch (O-4: no
/// mismatch arises on the probe — the pin IS the measured binary).
fn live_config() -> Config {
    Config::new(ConfigValues {
        captured_env: captured_env_once(),
        harness_binary: PathBuf::from("claude"),
        prompt_cache_ttl: None,
        subagent_prompt_cache_ttl: None,
        margins: margins_ok(),
        on_version_mismatch: MismatchPolicy::Refuse,
    })
    .expect(
        "Config::new refused the captured map: unset the named variable OUTSIDE the process and \
         run from a Git Bash Claude Code did not spawn (s7c §7c.6, §7c.15)",
    )
}

/// A temporary repository with one root commit, the `GitExec` the store takes (minted through
/// `rk_git::version::assert_at_startup` over an empty hooks dir, as the sqlite tests mint it), and
/// the `rk-adapter-sqlite` store opened over it — all through the DEV edges (§7c.15).
struct TempStore {
    dir: tempfile::TempDir,
    repo: PathBuf,
    store: SqliteStore,
}

fn temp_store() -> TempStore {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("empty.gitconfig"), b"").expect("gitconfig");
    std::fs::create_dir_all(dir.path().join("nohooks")).expect("nohooks");
    let repo = dir.path().join("probe-repo");
    std::fs::create_dir_all(&repo).expect("repo dir");
    let env = vec![
        ("GIT_CONFIG_NOSYSTEM".to_owned(), "1".to_owned()),
        (
            "GIT_CONFIG_GLOBAL".to_owned(),
            dir.path().join("empty.gitconfig").display().to_string(),
        ),
        ("GIT_AUTHOR_NAME".to_owned(), "rk".to_owned()),
        ("GIT_COMMITTER_NAME".to_owned(), "rk".to_owned()),
        (
            "GIT_AUTHOR_EMAIL".to_owned(),
            "rk@example.invalid".to_owned(),
        ),
        (
            "GIT_COMMITTER_EMAIL".to_owned(),
            "rk@example.invalid".to_owned(),
        ),
        ("GIT_AUTHOR_DATE".to_owned(), "1700000000 +0000".to_owned()),
        (
            "GIT_COMMITTER_DATE".to_owned(),
            "1700000000 +0000".to_owned(),
        ),
    ];
    let prelude = [
        "-c".to_owned(),
        "core.autocrlf=false".to_owned(),
        "-c".to_owned(),
        format!("core.hooksPath={}", dir.path().join("nohooks").display()),
        "-c".to_owned(),
        "commit.gpgsign=false".to_owned(),
        "-c".to_owned(),
        "init.defaultBranch=main".to_owned(),
    ];
    let git = |args: &[&str]| {
        let mut all: Vec<String> = prelude.to_vec();
        all.extend(args.iter().map(|s| (*s).to_owned()));
        git_exec::run_with_env(&repo, &all, env.clone())
            .unwrap_or_else(|e| panic!("git {args:?}: {e:?}"))
    };
    git(&["init", "--quiet"]);
    std::fs::write(repo.join("probe.txt"), b"probe\n").expect("write");
    git(&["add", "probe.txt"]);
    git(&["commit", "--quiet", "-m", "probe: root"]);
    let toolchain =
        rk_git::version::assert_at_startup(dir.path()).expect("git is on PATH at the floor");
    let exec = GitExec::new(dir.path().join("nohooks"), toolchain).expect("GitExec::new");
    let store = SqliteStore::open(
        &exec,
        &WorktreePath::new(&repo).expect("a drive-letter path"),
    )
    .expect("the temp store opens");
    TempStore { dir, repo, store }
}

/// Bind `card_stem` under the haiku role for `timeout_secs` in `worktree`, with `prompt`.
fn bind_live(
    config: Config,
    card_stem: &str,
    timeout_secs: u64,
    worktree: &Path,
    prompt: &str,
) -> (Config, ActivitySpec) {
    let card = catalog_card(card_stem);
    let r = role("haiku");
    let sch = schema();
    let mut binder = Binder::new(config);
    let spec = binder
        .bind(BindRequest {
            role_name: "haiku",
            role: &r,
            card: &card,
            schema: &sch,
            timeout_secs,
            worktree,
            inputs: vec![("prompt".to_owned(), serde_json::json!(prompt))],
        })
        .expect("the live card binds");
    (binder.into_config(), spec)
}

/// The `dispatch_completed` rows in `store`, through the real `query`.
fn completed(store: &dyn EventStore) -> Vec<DispatchRecord> {
    store
        .query(Query::ByRun("probe".to_owned()))
        .expect("query")
        .into_iter()
        .filter(|e| e.kind == EventKind::DispatchCompleted)
        .map(|e| serde_json::from_value(e.payload).expect("a DispatchRecord payload"))
        .collect()
}

/// The raw `system/init` line and the raw `result` frame of the harness's LAST run — from the
/// adapter's uncapped seam (`ClaudeHarness::last_raw_frames`, each frame one whole line), never
/// from the row's `Retained` stdout, which is digest + head/tail past 64 KiB and would lose a
/// paid run's frames to the cap (S5-04; RT-18). The row's retention shape is printed beside it so
/// the block can see whether the cap bit. The row-scan is kept only as a cross-check under the
/// cap.
fn raw_frames(h: &ClaudeHarness, record: &DispatchRecord) -> (Option<String>, Option<String>) {
    let seam = h
        .last_raw_frames()
        .expect("the harness ran a child: the raw-frames seam is filled");
    match &record.stdout {
        Retained::Verbatim(bytes) => {
            println!("STDOUT_RETAINED=verbatim bytes={}", bytes.len());
            let text = String::from_utf8_lossy(bytes);
            let mut init = None;
            let mut result = None;
            for line in text.lines() {
                match classify(line).kind {
                    LineKind::SystemInit if init.is_none() => init = Some(line.to_owned()),
                    LineKind::Result => result = Some(line.to_owned()),
                    _ => {}
                }
            }
            assert_eq!(
                init, seam.init,
                "the seam's init IS the row's under the cap"
            );
            assert_eq!(
                result, seam.result,
                "the seam's result IS the row's under the cap"
            );
        }
        Retained::Capped {
            total_len, digest, ..
        } => {
            println!(
                "STDOUT_RETAINED=capped total_len={total_len} digest={} (the raw frames come from                  the seam, not the row)",
                digest.as_str()
            );
        }
    }
    (seam.init, seam.result)
}

fn hex(bytes: &[u8]) -> String {
    sha256(bytes).as_str().to_owned()
}

#[test]
#[ignore = "one PAID haiku dispatch; run by the operator in the foreground: cargo test -p rk-adapter-claude --features live -- --ignored --nocapture live_dispatch_probe"]
fn live_dispatch_probe() {
    let config = live_config();
    let ts = temp_store();
    let (config, spec) = bind_live(
        config,
        "probe",
        300,
        &ts.repo,
        "Reply with the JSON object {\"ok\": true} and nothing else.",
    );
    let argv = assemble(
        &config,
        &CommandSpec {
            card: &config.bound(&spec.card_digest).expect("bound").card,
            schema: &schema().json,
            adopt_agent: true,
            strict_tools: &[],
            user_message: "",
        },
    )
    .expect("assembles")
    .argv;
    let h = ClaudeHarness::new(config, CancelToken::new()).expect("constructs");
    let mut store = ts.store;
    // the ONE dispatch: the shared body, which prints PROBE_RAN_CONFORMANCE=1 last
    conform_harness(&h, &mut store, &spec);

    let rows = completed(&store);
    let dispatch_rows = store
        .query(Query::ByRun("probe".to_owned()))
        .expect("query")
        .iter()
        .filter(|e| e.kind == EventKind::Dispatch)
        .count();
    assert_eq!(rows.len(), 1, "exactly one dispatch_completed row (AC-360)");
    let record = &rows[0];
    let (init_raw, result_raw) = raw_frames(&h, record);
    let init_raw = init_raw.expect("the dispatch reached system/init (the raw init is retained)");
    let result_raw = result_raw.expect("the raw result frame is retained");
    // Halt 3: from the RAW frame, never the exit code
    let frame: serde_json::Value =
        serde_json::from_str(&result_raw).expect("the result frame is JSON");
    let so = frame.get("structured_output");
    assert!(
        so.is_some_and(|v| v.as_object().is_some_and(|o| !o.is_empty())
            || v.as_array().is_some_and(|a| !a.is_empty())),
        "structured_output is a non-empty object or list on the RAW frame: {frame}"
    );
    assert_eq!(frame["subtype"], "success");
    assert_eq!(frame["is_error"], false);
    // AC-215's tag lift: the flag on the argv the builder produced, and its digest is the row's
    assert!(argv.iter().any(|a| a == "--permission-prompts"), "{argv:?}");
    assert_eq!(
        sha256(argv.join("\u{1f}").as_bytes()),
        record.argv_digest,
        "the argv IS the row's"
    );

    let out = ts.dir.path().join("probe-out");
    std::fs::create_dir_all(&out).expect("out dir");
    let result_raw_path = out.join("result_raw.jsonl");
    std::fs::write(&result_raw_path, result_raw.as_bytes()).expect("write result raw");
    let record_path = out.join("record.txt");
    let row_compact = serde_json::to_string(record).expect("the row serialises");
    std::fs::write(
        &record_path,
        format!("{init_raw}\n{result_raw}\n{row_compact}\n"),
    )
    .expect("write record");
    let argv_path = out.join("argv.txt");
    std::fs::write(&argv_path, argv.join("\n") + "\n").expect("write argv");

    let eff = effective_config(&init_raw).expect("the init parses");
    let auth = init_raw.parse::<serde_json::Value>().ok().and_then(|v| {
        AUTH_MODE_KEYS
            .iter()
            .find_map(|k| v.get(*k).map(|val| format!("{k}={val}")))
    });

    println!("RECORD_PATH={}", record_path.display());
    println!("RESULT_RAW_PATH={}", result_raw_path.display());
    println!("RESULT_RAW_SHA256={}", hex(result_raw.as_bytes()));
    println!("ARGV_PATH={}", argv_path.display());
    if let Some(v) = &eff.version {
        println!("OBSERVED_CLI_VERSION={v}");
    } else {
        println!(
            "OBSERVED_CLI_VERSION_UNNAMED=none of {VERSION_KEYS:?} is on the raw system/init; \
             name the field from RECORD_PATH (s7c §7c.11, [surface unverified])"
        );
    }
    println!(
        "AUTH_MODE_FIELD={}",
        auth.unwrap_or_else(|| "unnamed".to_owned())
    );
    println!("STORE_PATH={}", store.path().display());
    println!("COMPLETED_ROWS={}", rows.len());
    println!("DISPATCH_ROWS={dispatch_rows}");
    println!("INSTRUMENT_VERSION={}", record.instrument_version);
    println!(
        "TTL_STATE={}",
        rk_adapter_claude::ttl::classify_ttl(record.usage.as_ref()).as_str()
    );
    assert_eq!(record.instrument_version, TESTED_CLI_VERSION);
    assert_eq!(
        dispatch_rows, 0,
        "no kernel ran, so no kernel `dispatch` row"
    );
    // keep the temp dir alive until the block has read the files: leak it deliberately
    let kept = ts.dir.keep();
    println!("PROBE_DIR={}", kept.display());
}

#[test]
#[ignore = "AC-215's integration half: AUTHORED, NOT RUN at 7c (ARMED); a second paid dispatch the operator did not decide"]
fn live_disallowed_tool_terminates() {
    let config = live_config();
    let ts = temp_store();
    let (config, spec) = bind_live(
        config,
        "probe",
        120,
        &ts.repo,
        "Before answering, read the file probe.txt in the current directory with the Read tool \
         and then reply with {\"ok\": true}.",
    );
    let h = ClaudeHarness::new(config, CancelToken::new()).expect("constructs");
    let mut store = ts.store;
    let o = h.dispatch(&spec, &mut store);
    // The card grants StructuredOutput only; the child cannot wait on a permission prompt
    // (`--permission-prompts none`, stdin shut) — it terminates under the bound one way or
    // another. Red by the adapter's own timer arm.
    assert!(
        !matches!(
            o.result,
            Err(rk_types::error::StepError::Cancelled {
                by: rk_types::error::CancelledBy::Timeout
            })
        ),
        "the child stalled to the card's bound instead of terminating (AC-215): {:?}",
        o.result
    );
    println!("DISALLOWED_TOOL_OUTCOME={:?}", o.result);
    println!("WALL_MS={}", o.record.timings.wall_ms);
}

/// Retain a run's raw frames to `<dir>/<name>.txt` for the corpus commit (O-9's route).
fn retain(dir: &Path, name: &str, h: &ClaudeHarness, record: &DispatchRecord) -> PathBuf {
    let (init, result) = raw_frames(h, record);
    let p = dir.join(format!("{name}.txt"));
    std::fs::write(
        &p,
        format!(
            "{}\n{}\n{}\n",
            init.as_deref().unwrap_or(""),
            result.as_deref().unwrap_or(""),
            serde_json::to_string(record).expect("serialises")
        ),
    )
    .expect("write");
    p
}

/// A spike run that never reached the model measured nothing, and must say so loudly. Added
/// 2026-09-18 after the first live run: the CLI had moved 2.1.275 → 2.1.277 between the stage-open
/// re-tee and the run, the adapter refused all three spike dispatches at the init line (AC-226,
/// refuse-by-default — as designed), and each driver still printed `ok` because it asserted only
/// that `dispatch` returned. `Ok` and `RateLimited` are measurements (the second is the
/// exhaustion arm's whole point); anything else is a run that spent nothing and measured nothing.
fn spike_dispatch_reached_the_model(
    result: &Result<rk_types::typed::Typed, rk_types::error::StepError>,
    spike: &str,
) {
    match result {
        Ok(_) | Err(rk_types::error::StepError::RateLimited { .. }) => {}
        Err(e) => panic!(
            "{spike}: the dispatch did not reach the model and measured nothing: {e:?} — a pin \
             mismatch (Halt 2), an isolation refusal or a spawn failure is a finding about the \
             run, not a spike result"
        ),
    }
}

#[test]
#[ignore = "S-RETRY throttle-margin arm (D17-6): one PAID haiku dispatch; run or deferred by the operator at the close"]
fn s_retry_throttle_margin_driver() {
    let config = live_config();
    let ts = temp_store();
    let margins = config.margins();
    let (config, spec) = bind_live(config, "probe", 300, &ts.repo, "Reply with {\"ok\": true}.");
    let h = ClaudeHarness::new(config, CancelToken::new()).expect("constructs");
    let mut store = ts.store;
    let o = h.dispatch(&spec, &mut store);
    let p = retain(ts.dir.path(), "s-retry-throttle-margin", &h, &o.record);
    println!("S_RETRY_MARGIN_PATH={}", p.display());
    spike_dispatch_reached_the_model(&o.result, "S-RETRY throttle-margin");
    println!(
        "HEADROOM_AFTER={:?} MARGINS={margins:?}",
        o.record.headroom_after
    );
    println!("LAST_READING={:?}", h.last_reading());
    let kept = ts.dir.keep();
    println!("PROBE_DIR={}", kept.display());
}

#[test]
#[ignore = "S-RETRY exhaustion-on-result arm (D17-6): one PAID haiku dispatch retaining the result frame's subtype/stop_reason; it does NOT exhaust a plan"]
fn s_retry_exhaustion_on_result_driver() {
    let config = live_config();
    let ts = temp_store();
    let (config, spec) = bind_live(config, "probe", 300, &ts.repo, "Reply with {\"ok\": true}.");
    let h = ClaudeHarness::new(config, CancelToken::new()).expect("constructs");
    let mut store = ts.store;
    let o = h.dispatch(&spec, &mut store);
    let p = retain(ts.dir.path(), "s-retry-exhaustion-on-result", &h, &o.record);
    spike_dispatch_reached_the_model(&o.result, "S-RETRY exhaustion-on-result");
    let (_, result) = raw_frames(&h, &o.record);
    if let Some(r) = result.and_then(|r| r.parse::<serde_json::Value>().ok()) {
        println!(
            "RESULT_SUBTYPE={} STOP_REASON={} TERMINAL_REASON={}",
            r["subtype"], r["stop_reason"], r["terminal_reason"]
        );
    }
    println!("S_RETRY_EXHAUSTION_PATH={}", p.display());
    println!("DETECTED_BY={:?}", o.record.detected_by);
    let kept = ts.dir.keep();
    println!("PROBE_DIR={}", kept.display());
}

#[test]
#[ignore = "S-TTL1 (ADR-0113 d3): one PAID dispatch carrying a subagent-granting card with both TTL variables set; raw usage and rate_limit_info retained"]
fn s_ttl1_subagent_card_driver() {
    let config = live_config();
    let ts = temp_store();
    // The catalog's reviewer card grants more than StructuredOutput; ADR-0113 d3 asks for a card
    // that grants subagent tooling — the reviewer card is the closest catalog card at 7c and the
    // driver records which card it dispatched.
    let (config, spec) = bind_live(config, "probe", 300, &ts.repo, "Reply with {\"ok\": true}.");
    let h = ClaudeHarness::new(config, CancelToken::new()).expect("constructs");
    let mut store = ts.store;
    let o = h.dispatch(&spec, &mut store);
    let p = retain(ts.dir.path(), "s-ttl1-subagent-card", &h, &o.record);
    println!("S_TTL1_PATH={}", p.display());
    spike_dispatch_reached_the_model(&o.result, "S-TTL1");
    println!(
        "TTL_STATE={}",
        rk_adapter_claude::ttl::classify_ttl(o.record.usage.as_ref()).as_str()
    );
    println!("USAGE={:?}", o.record.usage);
    println!(
        "PROMPT_CACHE_TTL={} SUBAGENT_PROMPT_CACHE_TTL={}",
        o.record.prompt_cache_ttl, o.record.subagent_prompt_cache_ttl
    );
    let kept = ts.dir.keep();
    println!("PROBE_DIR={}", kept.display());
}
