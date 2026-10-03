//! **The termination paths, through the real `dispatch`** — AC-360, AC-227, AC-225, AC-224,
//! AC-215, AC-509, AC-210's runtime half, RT-21's worktree key, AC-368 (s7c §7c.10, §7c.11,
//! §7c.12, §7c.18; ADR-0126; ADR-0099 decisions 2 and 3; ADR-0127).
//!
//! Every dispatch here is the REAL one: `ClaudeHarness::dispatch` over the fixture child
//! (`rk-fixture-child-claude`, §7c.17) spawned through `rk-containment`, with the `VecStore` fake
//! as the assertion surface and a fresh marker directory per test (Halt 6). Nothing mocks the port.

use std::sync::Arc;
use std::thread;

use rk_ports::cancel::CancelToken;
use rk_ports::fakes::VecStore;
use rk_ports::harness::Harness;
use rk_types::activity::ActivitySpec;
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::event_kind::EventKind;
use rk_types::retention::Retained;
use rk_types::usage::CliVersion;

use rk_adapter_claude::binder::WORKTREE_KEY;
use rk_adapter_claude::config::{CACHE_TTL, Config, TESTED_CLI_VERSION};
use rk_adapter_claude::env::{PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL, child_env};
use rk_adapter_claude::env_denylist::is_denylisted;

use rk_adapter_claude::runner::EXIT_GRACE_MS;

use crate::support::{
    FailingStore, Markers, Part, bind_probe, completed_rows, dispatch_bounded, dispatch_mode,
    dispatch_replay, harness, harness_with_token, marker_count_in, megabyte_prompt, ok_transcript,
    orphan_holder_pid_in, spawn_config, spawn_values, transcript, values, wait_until,
};

/// The payload of the ONE `dispatch_completed` row in `store`, selected by KIND — never by
/// position, since a dispatch through the child may journal a `rate_limit_reading` before it
/// (ADR-0163 clause 2).
fn completed_payload(store: &VecStore) -> serde_json::Value {
    let mut rows = store
        .events()
        .iter()
        .filter(|e| e.kind == EventKind::DispatchCompleted);
    let row = rows.next().expect("a dispatch_completed row");
    assert!(rows.next().is_none(), "exactly one dispatch_completed row");
    row.payload.clone()
}

/// The variant name of an outcome, for the per-path table.
fn variant(result: &Result<rk_types::typed::Typed, StepError>) -> String {
    match result {
        Ok(_) => "Ok".to_owned(),
        Err(StepError::Transport { kind, .. }) => format!("Transport{{{kind:?}}}"),
        Err(StepError::Schema { .. }) => "Schema".to_owned(),
        Err(StepError::Timeout { .. }) => "Timeout".to_owned(),
        Err(StepError::Cancelled { by }) => format!("Cancelled{{{by:?}}}"),
        Err(StepError::NonZero { .. }) => "NonZero".to_owned(),
        Err(StepError::RateLimited { .. }) => "RateLimited".to_owned(),
        Err(StepError::NotAdopted { .. }) => "NotAdopted".to_owned(),
        Err(StepError::NotIsolated { .. }) => "NotIsolated".to_owned(),
        Err(StepError::Platform { .. }) => "Platform".to_owned(),
        Err(StepError::Unsupported { .. }) => "Unsupported".to_owned(),
        Err(StepError::Overage { .. }) => "Overage".to_owned(),
    }
}

/// AC-360 (adapter half): *"exactly one dispatch event is appended … AND no dispatch path exists
/// that can terminate without appending that event"* — every path §7c.11 lists, each induced
/// through the real `dispatch` by the fixture child's mode, a replayed frame, a flipped payload
/// or the token, asserting exactly one `dispatch_completed` row per return — at most one
/// `rate_limit_reading` beside it and before it, none on a pre-spawn path (ADR-0163 clause 2) —
/// and `cli_version: Pinned` on the pre-spawn rows (the throttle's two, a spawn failure, and a token that fired
/// before `dispatch` was entered — R5-5). **Nine variants (`Transport` by three kinds) plus `Ok` are
/// driven; `Unsupported` and `Timeout { after_secs }` are not** — the adapter produces neither
/// (§7c.11) and their arms exist in `classify.rs` by exhaustiveness alone.
///
/// RED (observed 2026-09-18) against the Slice 5 skeleton (`dispatch` spawns nothing and appends
/// nothing): `path Ok: expected exactly one dispatch_completed row per return, found 0 (AC-360;
/// ADR-0126's forbids)`.
#[test]
fn ac360_one_event_per_termination_path() {
    let mut driven: Vec<String> = Vec::new();
    let mut check = |name: &str, want: &str, r: &crate::support::Replayed, pinned: bool| {
        let rows = completed_rows(&r.store);
        assert_eq!(
            rows.len(),
            1,
            "path {name}: expected exactly one dispatch_completed row per return, found {} \
             (AC-360; ADR-0126's forbids)",
            rows.len()
        );
        let got = variant(&r.outcome.result);
        assert_eq!(got, want, "path {name}: {:?}", r.outcome.result);
        // *(Amended 2026-09-29, s8f build, package S8F-P7; ADR-0190 decision 1, proposed.)* Every
        // path's row carries `activity_id_at_version` as a PRESENT member — `null` here, because
        // the probe binding names no activity — and never an omitted key.
        let payload = completed_payload(&r.store);
        assert_eq!(
            payload.get("activity_id_at_version"),
            Some(&serde_json::Value::Null),
            "path {name}: the row carries `activity_id_at_version`, present and null: {payload}"
        );
        // The adapter writes TWO kinds and no other (the kernel's step_*/rate_limited rows are
        // the kernel's): exactly one dispatch_completed, at most one rate_limit_reading, and a
        // reading — when there is one — BEFORE the completion (ADR-0163 clause 2).
        let kinds = crate::support::kinds(&r.store);
        let readings = kinds
            .iter()
            .filter(|k| **k == EventKind::RateLimitReading)
            .count();
        assert!(
            kinds.iter().all(|k| matches!(
                k,
                EventKind::DispatchCompleted | EventKind::RateLimitReading
            )),
            "path {name}: the adapter writes dispatch_completed and rate_limit_reading and no \
             other kind: {kinds:?}"
        );
        assert!(
            readings <= 1,
            "path {name}: at most ONE rate_limit_reading per dispatch (ADR-0163 clause 2): \
             {kinds:?}"
        );
        assert_eq!(
            kinds.last(),
            Some(&EventKind::DispatchCompleted),
            "path {name}: a reading, when there is one, precedes the dispatch_completed row \
             (ADR-0163 clause 2): {kinds:?}"
        );
        if pinned {
            assert_eq!(
                rows[0].cli_version,
                CliVersion::Pinned(TESTED_CLI_VERSION.to_owned()),
                "path {name}: a pre-spawn row carries the pin as Pinned, never Observed (Halt 11)"
            );
            assert_eq!(r.markers.count(), 0, "path {name}: nothing spawned");
            assert_eq!(
                readings, 0,
                "path {name}: a pre-spawn path journals no reading — the guard's is the PREVIOUS \
                 dispatch's, and re-journalling it double-counts a sample (ADR-0163 clause 2): \
                 {kinds:?}"
            );
        } else {
            assert_eq!(
                r.outcome.record, rows[0],
                "path {name}: the row IS the record dispatch returned"
            );
        }
        driven.push(got);
    };

    // Ok — a replayed init + result frame with structured_output
    check("Ok", "Ok", &dispatch_replay(&ok_transcript(), 30), false);
    // Schema — the F20 frame: exit 0, subtype success, no structured_output
    let f20 = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("recorded/f20.result_raw.jsonl"),
    ]);
    check("Schema", "Schema", &dispatch_replay(&f20, 30), false);
    // Transport { Io } — the child closes stdin before the parent's ≥ 1 MiB write completes
    check(
        "Transport{Io}",
        "Transport{Io}",
        &dispatch_mode("close-stdin-early", 30, &megabyte_prompt()),
        false,
    );
    // Transport { Spawn } — a non-existent harness_binary
    {
        let markers = Markers::new();
        let mut v = spawn_values(&markers, "exit:0");
        v.harness_binary = markers.path().join("no-such-claude.exe");
        let worktree = tempfile::tempdir().expect("wt");
        let (config, spec) = bind_probe(
            Config::new(v).expect("constructs"),
            30,
            worktree.path(),
            "x",
        );
        let mut store = VecStore::default();
        let outcome = harness(config).dispatch(&spec, &mut store);
        let r = crate::support::Replayed {
            outcome,
            store,
            markers,
            worktree,
        };
        check("Transport{Spawn}", "Transport{Spawn}", &r, true);
    }
    // Transport { Decode } — a non-JSON line where the result frame is required, exit 0.
    // [amended 2026-09-25, s8a] The child READS stdin to EOF first (`wait-stdin-eof`): exit 0 with
    // no frame is `Decode` only when the prompt write completed (classify step 8), and a `replay:`
    // child that exits unread raced the write into `Transport { Io }` under the whole suite's load
    // (os error 232, measured in Checkpoint 5's whole run at s8a; 30/30 green in isolation).
    let decode = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("streams/malformed-where-result-required.scripted.ndjson"),
    ]);
    check(
        "Transport{Decode}",
        "Transport{Decode}",
        &dispatch_mode(
            &format!("wait-stdin-eof:{}", decode.path.display()),
            30,
            "hello",
        ),
        false,
    );
    // NonZero — exit 7, no result frame
    check(
        "NonZero",
        "NonZero",
        &dispatch_mode("exit:7", 30, "x"),
        false,
    );
    // Platform — a replayed is_error: true frame from results/e3.jsonl
    let platform = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("recorded/e3.L14.result_raw.jsonl"),
    ]);
    check(
        "Platform",
        "Platform",
        &dispatch_replay(&platform, 30),
        false,
    );
    // NotAdopted / NotIsolated — a replayed init naming the wrong agent / a superset tool set
    let wrong = transcript(&[
        Part::File("streams/init-wrong-agent.scripted.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    check(
        "NotAdopted",
        "NotAdopted",
        &dispatch_replay(&wrong, 30),
        false,
    );
    let superset = transcript(&[
        Part::File("streams/init-superset.scripted.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    check(
        "NotIsolated",
        "NotIsolated",
        &dispatch_replay(&superset, 30),
        false,
    );
    // Cancelled { by: Signal } — the child in hold mode, the token fired from a second thread
    {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("wt");
        let (config, spec) = bind_probe(spawn_config(&markers, "hold"), 60, worktree.path(), "x");
        let token = CancelToken::new();
        let h = harness_with_token(config, token.clone());
        let marker_dir = markers.path().to_path_buf();
        let firer = thread::spawn(move || {
            let up = wait_until(1500, || {
                std::fs::read_dir(&marker_dir)
                    .map(|d| {
                        d.flatten()
                            .any(|e| e.path().extension().is_some_and(|x| x == "marker"))
                    })
                    .unwrap_or(false)
            });
            assert!(up, "the hold child wrote its marker within the bound");
            token.cancel();
        });
        let mut store = VecStore::default();
        let outcome = h.dispatch(&spec, &mut store);
        firer.join().expect("the firing thread");
        let r = crate::support::Replayed {
            outcome,
            store,
            markers,
            worktree,
        };
        check("Cancelled{Signal}", "Cancelled{Signal}", &r, false);
        assert_eq!(r.outcome.record.cancelled_by, Some(CancelledBy::Signal));
    }
    // Cancelled { by: Timeout } — the card's timeout_secs expiring on the adapter's own timer
    let r = dispatch_mode("hold", 1, "x");
    check("Cancelled{Timeout}", "Cancelled{Timeout}", &r, false);
    assert_eq!(r.outcome.record.cancelled_by, Some(CancelledBy::Timeout));
    // pre-spawn RateLimited — the throttle over a reading at the boundary (AC-376): the FIRST
    // dispatch replays it, the SECOND is refused before spawn
    {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("wt");
        let at = transcript(&[
            Part::File("streams/init.scripted.ndjson"),
            Part::File("streams/rate-limit-at-boundary-five-hour-93.scripted.ndjson"),
            Part::File("streams/result-ok.scripted.ndjson"),
        ]);
        let (config, spec) = bind_probe(
            spawn_config(&markers, &format!("replay:{}", at.path.display())),
            30,
            worktree.path(),
            "x",
        );
        let h = harness(config);
        let mut first = VecStore::default();
        let o1 = h.dispatch(&spec, &mut first);
        assert!(
            o1.result.is_ok(),
            "the first dispatch proceeds: {:?}",
            o1.result
        );
        assert_eq!(markers.count(), 1);
        let mut store = VecStore::default();
        let outcome = h.dispatch(&spec, &mut store);
        let r = crate::support::Replayed {
            outcome,
            store,
            markers: Markers::new(),
            worktree,
        };
        check("RateLimited(pre-spawn)", "RateLimited", &r, true);
        assert_eq!(markers.count(), 1, "no second spawn (AC-376)");
    }
    // pre-spawn Cancelled { by: Signal } — a token that fired before dispatch was entered (R5-5)
    {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("wt");
        let (config, spec) = bind_probe(spawn_config(&markers, "hold"), 60, worktree.path(), "x");
        let token = CancelToken::new();
        token.cancel();
        let mut store = VecStore::default();
        let outcome = harness_with_token(config, token).dispatch(&spec, &mut store);
        let r = crate::support::Replayed {
            outcome,
            store,
            markers,
            worktree,
        };
        check(
            "Cancelled{Signal}(pre-spawn)",
            "Cancelled{Signal}",
            &r,
            true,
        );
    }
    // pre-spawn Overage — the flipped e1-smoke payload (AC-221)
    {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("wt");
        let recorded = crate::support::e1_smoke_rate_limit_event(0);
        let flipped = String::from_utf8(recorded)
            .expect("utf-8")
            .replace("\"isUsingOverage\":false", "\"isUsingOverage\":true");
        let mut line = flipped.into_bytes();
        line.push(b'\n');
        let t = transcript(&[
            Part::File("streams/init.scripted.ndjson"),
            Part::Bytes(line),
            Part::File("streams/result-ok.scripted.ndjson"),
        ]);
        let (config, spec) = bind_probe(
            spawn_config(&markers, &format!("replay:{}", t.path.display())),
            30,
            worktree.path(),
            "x",
        );
        let h = harness(config);
        let mut first = VecStore::default();
        let o1 = h.dispatch(&spec, &mut first);
        assert!(
            o1.result.is_ok(),
            "the first dispatch proceeds: {:?}",
            o1.result
        );
        let mut store = VecStore::default();
        let outcome = h.dispatch(&spec, &mut store);
        let r = crate::support::Replayed {
            outcome,
            store,
            markers: Markers::new(),
            worktree,
        };
        check("Overage(pre-spawn)", "Overage", &r, true);
        assert_eq!(markers.count(), 1, "no second spawn (AC-221)");
    }

    // Nine variants (Transport by three kinds) plus Ok are driven; Unsupported and Timeout are
    // not (§7c.11) — the count against ADR-0112's eleven reproduces from this list.
    let want = [
        "Ok",
        "Schema",
        "Transport{Io}",
        "Transport{Spawn}",
        "Transport{Decode}",
        "NonZero",
        "Platform",
        "NotAdopted",
        "NotIsolated",
        "Cancelled{Signal}",
        "Cancelled{Timeout}",
        "RateLimited",
        "Cancelled{Signal}",
        "Overage",
    ];
    assert_eq!(driven, want, "every driven path, in order");
}

/// AC-227: *"any harness event written to the event store … carries the CLI version the dispatch
/// actually ran against"* — EVERY row the ADAPTER appends, of both kinds it writes
/// (`dispatch_completed` and `rate_limit_reading`, ADR-0163), carries the non-optional
/// `CliVersion`, `Observed(v)` where an init reported one and `Pinned` on the pre-spawn rows (a
/// pre-spawn row written `Observed` is the red — the pin masquerading as a measurement, Halt 11).
/// The Observed arm dispatches a stream that carries a reading, so both kinds are on the store and
/// every row is iterated, whatever its kind.
///
/// Kills: `cli_version` dropped from the reading payload; the reading carrying a version other
/// than its dispatch's (e.g. the pin where the init observed one).
///
/// RED (observed 2026-09-18) against the skeleton: `the Ok path appended no dispatch_completed
/// row to iterate (AC-227)`.
#[test]
fn ac227_every_harness_event_carries_cli_version() {
    // Observed rows, of BOTH adapter-written kinds: a stream carrying a reading
    let r = dispatch_replay(
        &transcript(&[
            Part::File("streams/init.scripted.ndjson"),
            Part::File("streams/rate-limit-seven-day-81.scripted.ndjson"),
            Part::File("streams/result-ok.scripted.ndjson"),
        ]),
        30,
    );
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let rows = completed_rows(&r.store);
    assert!(
        !rows.is_empty(),
        "the Ok path appended no dispatch_completed row to iterate (AC-227)"
    );
    for row in &rows {
        assert_eq!(
            row.cli_version,
            CliVersion::Observed("2.1.278".to_owned()),
            "an init that reported a version lands as Observed(v)"
        );
    }
    let observed = serde_json::to_value(CliVersion::Observed("2.1.278".to_owned()))
        .expect("a CliVersion serialises");
    let events = r.store.events();
    let written: Vec<EventKind> = events.iter().map(|e| e.kind).collect();
    assert_eq!(
        written,
        vec![EventKind::RateLimitReading, EventKind::DispatchCompleted],
        "premise: both adapter-written kinds are on the store to iterate"
    );
    for event in events {
        assert_eq!(
            event.payload.get("cli_version"),
            Some(&observed),
            "every adapter-written row carries the dispatch's cli_version ({:?}): {}",
            event.kind,
            event.payload
        );
    }
    // a pre-spawn row (a spawn failure): Pinned, never Observed
    let markers = Markers::new();
    let mut v = spawn_values(&markers, "exit:0");
    v.harness_binary = markers.path().join("absent.exe");
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(Config::new(v).expect("constructs"), 30, wt.path(), "x");
    let mut store = VecStore::default();
    let _ = harness(config).dispatch(&spec, &mut store);
    let rows = completed_rows(&store);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0].cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned()),
        "a row on a path that read no system/init carries Pinned (ADR-0126 d1; AC-227)"
    );
    assert_eq!(rows[0].instrument_version, TESTED_CLI_VERSION);
    // a killed-before-init row (hold mode, the timer): Pinned too
    let r = dispatch_mode("hold", 1, "x");
    assert_eq!(
        r.outcome.record.cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned()),
        "a kill before init read no version: Pinned"
    );
}

/// AC-225: *"the classifier is keyed on the cancel token having fired, not on the exit code"* —
/// two positive arms (the token fired by the test → `Cancelled { by: Signal }`; the card's
/// `timeout_secs` expiring on the adapter's own timer → `Cancelled { by: Timeout }`; RT-3 as
/// amended), both terminating the child with `CANCEL_EXIT_CODE` through the group, and one
/// negative: an EXTERNAL kill with the same code (`rk_containment::terminate_pid` on the pid the
/// marker carries) with neither arm fired is NOT `Cancelled`.
///
/// RED (observed 2026-09-18) against the skeleton: `the token arm: expected Cancelled { by: Signal
/// }, got Err(Transport { kind: Spawn, detail: "SKELETON: no dispatch path exists yet" })`.
#[test]
fn ac225_cancelled_keyed_on_token_not_exit_code() {
    // positive arm 1: the token, fired from a second thread while dispatch blocks
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "hold"), 60, wt.path(), "x");
    let token = CancelToken::new();
    let h = harness_with_token(config, token.clone());
    let dir = markers.path().to_path_buf();
    let firer = thread::spawn(move || {
        assert!(
            wait_until(1500, || marker_count_in(&dir) >= 1),
            "the hold child came up"
        );
        token.cancel();
    });
    let mut store = VecStore::default();
    let o = h.dispatch(&spec, &mut store);
    firer.join().expect("firer");
    assert!(
        matches!(
            o.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Signal
            })
        ),
        "the token arm: expected Cancelled {{ by: Signal }}, got {:?}",
        o.result
    );
    assert_eq!(o.record.cancelled_by, Some(CancelledBy::Signal));
    // R5-7: the kill happened ON the token arm, not on a later grace — a mutant that drops the
    // arm's kill and lets the post-exit grace reap the child is red on the wall time.
    assert!(
        o.record.timings.wall_ms < EXIT_GRACE_MS,
        "the token arm's kill is immediate, never a grace: {} ms",
        o.record.timings.wall_ms
    );

    // positive arm 2: the adapter's own timer (timeout_secs = 1 on a child that holds)
    let r = dispatch_mode("hold", 1, "x");
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Timeout
            })
        ),
        "the timer arm: expected Cancelled {{ by: Timeout }}, got {:?}",
        r.outcome.result
    );
    assert_eq!(r.outcome.record.cancelled_by, Some(CancelledBy::Timeout));
    assert!(
        r.outcome.record.timings.wall_ms < EXIT_GRACE_MS,
        "the timer arm's kill is immediate, never a grace: {} ms",
        r.outcome.record.timings.wall_ms
    );

    // negative control: an external kill with the SAME code, neither arm fired
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "hold"), 60, wt.path(), "x");
    let h = harness(config);
    let dir = markers.path().to_path_buf();
    let killer = thread::spawn(move || {
        assert!(
            wait_until(1500, || marker_count_in(&dir) >= 1),
            "the hold child came up"
        );
        let pid = crate::support::first_marker_pid_in(&dir);
        rk_containment::terminate_pid(pid, rk_containment::CANCEL_EXIT_CODE)
            .expect("the external kill");
    });
    let mut store = VecStore::default();
    let o = h.dispatch(&spec, &mut store);
    killer.join().expect("killer");
    match &o.result {
        // The exit code is recorded per platform: on Windows `TerminateProcess` carries the
        // adapter's own code; on POSIX `terminate_pid` is a SIGKILL and ignores it, so the status
        // has no code at all (`ExitStatus::code()` is `None` for a signal death) — rk-containment's
        // `CANCEL_EXIT_CODE` doc: "On POSIX the group is killed with `SIGKILL` and there is no
        // code to choose".
        Err(StepError::NonZero { code, .. }) => {
            #[cfg(windows)]
            let expected = Some(i32::try_from(rk_containment::CANCEL_EXIT_CODE).expect("fits"));
            #[cfg(unix)]
            let expected = None;
            assert_eq!(
                *code, expected,
                "the exit code is recorded per platform (the adapter's own code on Windows, no \
                 code for a POSIX signal death)"
            );
        }
        other => panic!(
            "an EXTERNAL kill with CANCEL_EXIT_CODE and neither arm fired must not classify as \
             Cancelled (AC-225: keyed on the token, not the exit code); got {other:?}"
        ),
    }
    assert_eq!(o.record.cancelled_by, None);
}

/// AC-224 (register-fixed name; the Windows arm is the live arm on this box): the fixture child
/// runs in the three-level `orphan` mode — it spawns an intermediate that spawns the HOLDER and
/// exits, and the child itself HOLDS — so `dispatch` is in flight, blocked on the child, when the
/// test fires the token from a second thread; the holder's parent is dead, and a PID walk from the
/// live child finds nothing. Red by the holder alive after `dispatch` returns (read by the
/// job-independent `is_alive`), by the job handle dropped before the kill completed (the
/// `KillDropOrder` probe), and separately by the `breakaway` attempt SUCCEEDING (the negative
/// control: a spawn error or in-job).
///
/// RED (observed 2026-09-18) against the skeleton: `the orphan child never came up (no orphan
/// marker within the bound) — nothing was spawned`.
#[test]
fn cancel_reaps_process_tree() {
    // positive control: the orphaned holder is reaped through the job
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "orphan"), 120, wt.path(), "x");
    let token = CancelToken::new();
    let h = Arc::new(harness_with_token(config, token.clone()));
    let dir = markers.path().to_path_buf();
    let firer = thread::spawn(move || {
        // F9: the holder is observed ALIVE before the token fires — the WHEN of AC-224 ("WHEN the
        // adapter terminates the child") is an event with a live tree, not a marker's existence.
        let holder = wait_until(2000, || orphan_holder_pid_in(&dir));
        let holder_pid = holder.expect(
            "the orphan child never came up (no orphan marker within the bound) — nothing was \
             spawned",
        );
        assert!(
            rk_containment::is_alive(holder_pid),
            "the holder (pid {holder_pid}) was not alive when the token fired — the WHEN never \
             occurred"
        );
        token.cancel();
        holder_pid
    });
    // F2: the dispatch is joined under a bound, so a containment that leaves a pipe holder alive
    // fails on the holder's liveness instead of hanging the suite on the stderr reader.
    let (o, _store) = dispatch_bounded(Arc::clone(&h), spec, 90).unwrap_or_else(|| {
        panic!(
            "dispatch did not return within the bound: a descendant survived the kill and holds \
             the pipes (AC-224; F2) — holders alive: {:?}",
            markers
                .with_mode("orphan-holder")
                .iter()
                .map(|m| (m.pid, rk_containment::is_alive(m.pid)))
                .collect::<Vec<_>>()
        )
    });
    let holder_pid_before = firer.join().expect("firer");
    assert!(
        matches!(
            o.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Signal
            })
        ),
        "the token fired while the child held: {:?}",
        o.result
    );
    assert!(
        o.record.timings.wall_ms < EXIT_GRACE_MS,
        "killed on the arm, not on a grace: {} ms",
        o.record.timings.wall_ms
    );
    let orphan = markers.with_mode("orphan");
    assert_eq!(orphan.len(), 1, "one orphan marker: {:?}", markers.read());
    let holder_pid: u32 = orphan[0]
        .extra
        .iter()
        .find_map(|l| l.strip_prefix("holder_pid=")?.parse().ok())
        .expect("holder_pid= on the orphan marker");
    assert_eq!(holder_pid, holder_pid_before);
    assert!(
        orphan[0].extra.iter().any(|l| l == "intermediate=exited"),
        "the holder's parent was dead before the token fired: {:?}",
        orphan[0].extra
    );
    let holders = markers.with_mode("orphan-holder");
    assert_eq!(holders.len(), 1, "the holder wrote its own marker");
    assert_eq!(holders[0].pid, holder_pid);
    assert!(
        crate::support::wait_dead(holder_pid),
        "the orphaned holder (pid {holder_pid}) survived the cancel: only the job holds it, and a \
         PID walk from the live child misses it (AC-224; F22)"
    );
    assert!(
        !rk_containment::is_alive(orphan[0].pid),
        "the direct child (pid {}) survived the cancel",
        orphan[0].pid
    );
    let order = h
        .last_kill_drop_order()
        .expect("the adapter kept the group's kill-then-drop probe");
    assert!(
        order.killed(),
        "the kill COMPLETED (Group::kill records only an Ok — a swallowed Err reads false here; R5-4)"
    );
    assert!(
        order.killed_before_drop(),
        "the kill must COMPLETE through the job before the handle drops — a mutant relying on \
         KILL_ON_JOB_CLOSE at drop is red here (killed={}, dropped={})",
        order.killed(),
        order.dropped()
    );
    assert!(
        h.kill_warnings().is_empty(),
        "no kill through the group failed on this dispatch (S5-11): {:?}",
        h.kill_warnings()
    );

    // negative control: the breakaway attempt must FAIL (a spawn error, or in-job and reaped)
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "breakaway"), 120, wt.path(), "x");
    let token = CancelToken::new();
    let h = harness_with_token(config, token.clone());
    let dir = markers.path().to_path_buf();
    let firer = thread::spawn(move || {
        assert!(
            wait_until(2000, || {
                std::fs::read_dir(&dir).is_ok_and(|d| {
                    d.flatten().any(|e| {
                        std::fs::read_to_string(e.path())
                            .is_ok_and(|t| t.lines().any(|l| l == "mode=breakaway"))
                    })
                })
            }),
            "the breakaway child came up"
        );
        token.cancel();
    });
    let mut store = VecStore::default();
    let _ = h.dispatch(&spec, &mut store);
    firer.join().expect("firer");
    let attempt = markers.with_mode("breakaway");
    assert_eq!(
        attempt.len(),
        1,
        "one breakaway marker: {:?}",
        markers.read()
    );
    let line = attempt[0]
        .extra
        .iter()
        .find(|l| l.starts_with("breakaway="))
        .expect("the breakaway line on the marker");
    if let Some(rest) = line.strip_prefix("breakaway=ok pid=") {
        // O-2: the attempt spawned — then it landed INSIDE the job (no BREAKAWAY_OK is granted)
        // and the kill through the job reaped it; a survivor here is a breakaway that succeeded.
        let pid: u32 = rest.parse().expect("pid");
        #[cfg(windows)]
        assert!(
            !rk_containment::is_alive(pid),
            "the breakaway attempt SUCCEEDED: its holder (pid {pid}) is alive after the kill \
             through the job (ADR-0127 d7: the job never grants BREAKAWAY_OK; O-2)"
        );
        // POSIX: a MEASUREMENT, not a control (rk-containment's posix.rs module doc; the fixture's
        // mode table). The holder was spawned with `process_group(0)`, the POSIX analogue of
        // `CREATE_BREAKAWAY_FROM_JOB` that no flag on the group can refuse, so `killpg` on the
        // child's group cannot reach it and ADR-0127 (:194) forbids the PID walk that would. The
        // hole is asserted present so that closing it reads red here and is recorded, and the
        // survivor is killed by pid so it does not hold the fixture's 600 s cap.
        #[cfg(unix)]
        {
            let survived = rk_containment::is_alive(pid);
            let _ = rk_containment::terminate_pid(pid, rk_containment::CANCEL_EXIT_CODE);
            assert!(
                survived,
                "the POSIX breakaway holder (pid {pid}) did NOT survive the kill through the \
                 group: the hole posix.rs records is closed — update this measurement and \
                 ADR-0127 d7's POSIX reading"
            );
        }
    } else {
        assert!(
            line.starts_with("breakaway=err os="),
            "the breakaway line is `breakaway=err os=<code>` or `breakaway=ok pid=<n>`: {line}"
        );
    }
}

/// AC-224's Windows arm, asserted BY NAME on this box (§7c.10, §7c.18): with the child ALIVE and
/// `dispatch` in flight, the token fires and the Job Object (`KILL_ON_JOB_CLOSE`, created before
/// spawn) reaps the orphaned holder a PID walk misses, before the handle drops. Red by the
/// holder surviving the kill; assign order is not this test's red (that is
/// `ac224_job_object_created_before_spawn` in `rk-containment`).
///
/// RED (observed 2026-09-18) against the skeleton: `the orphan child never came up`.
#[cfg(windows)]
#[test]
fn ac224_windows_job_object_reaps_orphaned_grandchild() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "orphan"), 120, wt.path(), "x");
    let token = CancelToken::new();
    let h = Arc::new(harness_with_token(config, token.clone()));
    let dir = markers.path().to_path_buf();
    let firer = thread::spawn(move || {
        let holder_pid = wait_until(2000, || orphan_holder_pid_in(&dir))
            .expect("the orphan child never came up");
        assert!(
            rk_containment::is_alive(holder_pid),
            "the holder (pid {holder_pid}) was not alive when the token fired (F9)"
        );
        token.cancel();
    });
    let (o, _store) = dispatch_bounded(Arc::clone(&h), spec, 90).unwrap_or_else(|| {
        panic!(
            "dispatch did not return within the bound (F2) — holders alive: {:?}",
            markers
                .with_mode("orphan-holder")
                .iter()
                .map(|m| (m.pid, rk_containment::is_alive(m.pid)))
                .collect::<Vec<_>>()
        )
    });
    firer.join().expect("firer");
    assert!(
        matches!(o.result, Err(StepError::Cancelled { .. })),
        "{:?}",
        o.result
    );
    let holder = markers.with_mode("orphan-holder");
    assert_eq!(holder.len(), 1);
    assert!(
        !rk_containment::is_alive(holder[0].pid),
        "the orphaned holder (pid {}) survived the Job Object kill (AC-224, Windows arm)",
        holder[0].pid
    );
    let order = h.last_kill_drop_order().expect("probe");
    assert!(order.killed(), "the kill completed (R5-4)");
    assert!(order.killed_before_drop(), "kill before drop");
}

/// AC-215 (unit half): *"`--permission-prompts none` is passed and the child's stdin is closed"* —
/// write-then-EOF, OBSERVABLY: the fixture child in `wait-stdin-eof:<transcript>` mode blocks
/// until stdin reaches EOF, then replays the scripted transcript and exits 0, so the correct
/// implementation's outcome is `Ok` and a never-shut stdin hangs to the card's bound with
/// `Cancelled { by: Timeout }`. The flag is read back from the argv the marker carries.
///
/// RED (observed 2026-09-18) against the skeleton: `stdin was not shut after the write (or the
/// prompt was not written): expected Ok, got Err(Transport { kind: Spawn, … })`.
#[test]
fn ac215_permission_prompts_none_stdin_closed() {
    let t = ok_transcript();
    let r = dispatch_mode(&format!("wait-stdin-eof:{}", t.path.display()), 30, "hello");
    assert!(
        r.outcome.result.is_ok(),
        "stdin was not shut after the write (or the prompt was not written): expected Ok, got {:?}",
        r.outcome.result
    );
    assert!(
        r.outcome.record.timings.wall_ms < 15_000,
        "well under the card's bound: {} ms",
        r.outcome.record.timings.wall_ms
    );
    let m = r.markers.read();
    assert_eq!(m.len(), 1);
    let pos = m[0]
        .argv
        .iter()
        .position(|a| a == "--permission-prompts")
        .expect("--permission-prompts on the argv the child received");
    assert_eq!(m[0].argv.get(pos + 1).map(String::as_str), Some("none"));
}

/// AC-509: *"the child's environment contains no tracker credential AND the assertion is made
/// from the child's own reported environment"* — the fixture child in `echo-env` mode through the
/// real spawn path echoes an environment EQUAL to the constructed map (the captured map plus the
/// two TTL variables) and nothing else: red by `env_clear()` removed (the echo then carries the
/// outer cargo's `CARGO_MANIFEST_DIR` / `PATH`), and by any denylisted name in the echo. The
/// tracker-credential half is satisfied by construction through AC-217's refusal (§7c.6).
///
/// RED (observed 2026-09-18) against the skeleton: `the child echoed no environment (no stdout
/// retained): nothing was spawned`.
#[test]
fn ac509_child_echoes_env_without_tracker_credential() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let mut v = spawn_values(&markers, "echo-env");
    v.captured_env
        .insert("RK_TEST_PLAIN".to_owned(), "kept".to_owned());
    let config = Config::new(v).expect("constructs");
    let expected = child_env(&config).as_map().clone();
    let (config, spec) = bind_probe(config, 30, wt.path(), "x");
    let mut store = VecStore::default();
    let o = harness(config).dispatch(&spec, &mut store);
    let Retained::Verbatim(bytes) = &o.record.stdout else {
        panic!("the echo is small: {:?}", o.record.stdout);
    };
    let text = String::from_utf8_lossy(bytes);
    assert!(
        !text.is_empty(),
        "the child echoed no environment (no stdout retained): nothing was spawned"
    );
    let echoed: std::collections::BTreeMap<String, String> = text
        .lines()
        .filter(|l| !l.starts_with("CWD="))
        .filter_map(|l| l.split_once('='))
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect();
    assert_eq!(
        echoed, expected,
        "the child's OWN environment equals the constructed map plus the two TTLs and nothing else \
         (AC-509; AC-217): env_clear() then the map"
    );
    assert!(
        echoed.keys().all(|k| !is_denylisted(k)),
        "no denylisted name in the echo: {:?}",
        echoed.keys().collect::<Vec<_>>()
    );
    assert!(
        !echoed.contains_key("CARGO_MANIFEST_DIR") && !echoed.contains_key("PATH"),
        "the outer cargo's environment leaked around the map"
    );
    assert_eq!(
        echoed.get(PROMPT_CACHE_TTL).map(String::as_str),
        Some(CACHE_TTL)
    );
    assert_eq!(
        echoed.get(SUBAGENT_PROMPT_CACHE_TTL).map(String::as_str),
        Some(CACHE_TTL)
    );
}

/// AC-210 (runtime half): *"`CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and
/// `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` are set explicitly and both values are recorded on
/// the dispatch event"* — the echo carries both, and the `dispatch_completed` row carries both
/// (the non-optional-field half is `-p rk-types`).
///
/// RED (observed 2026-09-18) against the skeleton: `no dispatch_completed row carries the TTL
/// pair (AC-210)`.
#[test]
fn ac210_both_ttl_vars_set_and_recorded() {
    let r = dispatch_mode("echo-env", 30, "x");
    let Retained::Verbatim(bytes) = &r.outcome.record.stdout else {
        panic!("small echo");
    };
    let text = String::from_utf8_lossy(bytes);
    assert!(
        text.lines()
            .any(|l| l == format!("{PROMPT_CACHE_TTL}={CACHE_TTL}"))
    );
    assert!(
        text.lines()
            .any(|l| l == format!("{SUBAGENT_PROMPT_CACHE_TTL}={CACHE_TTL}"))
    );
    let rows = completed_rows(&r.store);
    assert!(
        !rows.is_empty(),
        "no dispatch_completed row carries the TTL pair (AC-210)"
    );
    assert_eq!(rows[0].prompt_cache_ttl, CACHE_TTL);
    assert_eq!(rows[0].subagent_prompt_cache_ttl, CACHE_TTL);
    let payload = &completed_payload(&r.store);
    assert_eq!(payload["prompt_cache_ttl"], CACHE_TTL);
    assert_eq!(payload["subagent_prompt_cache_ttl"], CACHE_TTL);
}

/// RT-21 (§7c.12; AC-529's clause cited, not claimed): the fixture child in `echo-env` mode
/// echoes its cwd — the spec's `"worktree"` input, never the process's cwd — and a key-less spec
/// driven straight into `dispatch` is `Err(Transport { kind: Spawn })` with marker count 0 and
/// exactly one `dispatch_completed` row (RT-20's sibling).
///
/// RED (observed 2026-09-18) against the skeleton: `the child echoed no CWD= line: nothing was
/// spawned`.
#[test]
fn dispatch_cwd_is_the_spec_worktree_never_process_cwd() {
    let r = dispatch_mode("echo-env", 30, "x");
    let Retained::Verbatim(bytes) = &r.outcome.record.stdout else {
        panic!("small echo");
    };
    let text = String::from_utf8_lossy(bytes);
    let cwd = text
        .lines()
        .find_map(|l| l.strip_prefix("CWD="))
        .expect("the child echoed no CWD= line: nothing was spawned");
    let observed = std::fs::canonicalize(cwd).expect("the echoed cwd exists");
    let want = std::fs::canonicalize(r.worktree.path()).expect("the worktree exists");
    assert_eq!(
        observed, want,
        "the child ran in the spec's worktree (RT-21), not the process cwd"
    );
    // The process cwd is NOT read here (env::current_dir is banned in src/ and unwanted in a test
    // that would compare against it); the worktree is a fresh temp dir, which is not the crate
    // dir the test binary runs in.
    assert!(
        !observed.ends_with("rk-adapter-claude"),
        "the child ran in the crate directory, the process cwd: {}",
        observed.display()
    );
    // a key-less spec straight into dispatch
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "echo-env"), 30, wt.path(), "x");
    let keyless = ActivitySpec {
        inputs: spec
            .inputs
            .iter()
            .filter(|(k, _)| k != WORKTREE_KEY)
            .cloned()
            .collect(),
        ..spec
    };
    let mut store = VecStore::default();
    let o = harness(config).dispatch(&keyless, &mut store);
    assert!(
        matches!(
            o.result,
            Err(StepError::Transport {
                kind: TransportKind::Spawn,
                ..
            })
        ),
        "a spec without the worktree key is Transport {{ kind: Spawn }} before spawn: {:?}",
        o.result
    );
    assert_eq!(markers.count(), 0, "nothing spawned");
    assert_eq!(completed_rows(&store).len(), 1, "exactly one row");
    assert_eq!(
        completed_rows(&store)[0].cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
    );
    // S5-10: a RELATIVE worktree value would make `current_dir` resolve against the process cwd —
    // the very source RT-21 forbids — so it is refused before spawn, as is a path that is not a
    // directory; marker count 0, one Pinned row each.
    for (label, value) in [
        ("relative", serde_json::json!("worktrees/run-1")),
        (
            "not a directory",
            serde_json::json!(wt.path().join("no-such-dir").to_string_lossy()),
        ),
    ] {
        let markers = Markers::new();
        let (config, spec) = bind_probe(spawn_config(&markers, "echo-env"), 30, wt.path(), "x");
        let bad = ActivitySpec {
            inputs: spec
                .inputs
                .iter()
                .map(|(k, v)| {
                    if k == WORKTREE_KEY {
                        (k.clone(), value.clone())
                    } else {
                        (k.clone(), v.clone())
                    }
                })
                .collect(),
            ..spec
        };
        let mut store = VecStore::default();
        let o = harness(config).dispatch(&bad, &mut store);
        assert!(
            matches!(
                o.result,
                Err(StepError::Transport {
                    kind: TransportKind::Spawn,
                    ..
                })
            ),
            "a {label} worktree is refused before spawn as Transport {{ kind: Spawn }} (RT-21; \
             AC-529's clause): {:?}",
            o.result
        );
        assert_eq!(markers.count(), 0, "{label}: nothing spawned");
        assert_eq!(completed_rows(&store).len(), 1, "{label}: exactly one row");
        assert_eq!(
            completed_rows(&store)[0].cli_version,
            CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
        );
    }
}

/// AC-368 — PROVISIONAL, built and NOT claimed (§7c.13): a fault-injecting fake handed to
/// `dispatch`; the append is ATTEMPTED on the failing store (the fake's call count 1), the failure
/// is counted and typed on the adapter's warning counter, and there is no panic. **The
/// `Ok(Typed)`-after-failed-append assertion below is R-16's reading under test, not a claim**:
/// ADR-0126's forbids reads against a path that returns without appending, decision 6 leaves the
/// reconciliation to a register amendment.
///
/// RED (observed 2026-09-18) against the skeleton: `the append was not attempted on the failing
/// store (call count 0) — AC-368`.
#[test]
fn ac368_tier2_append_failure_is_warning() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let t = ok_transcript();
    let (config, spec) = bind_probe(
        spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "x",
    );
    let h = harness(config);
    let mut store = FailingStore::default();
    let o = h.dispatch(&spec, &mut store);
    assert_eq!(
        store.attempts, 1,
        "the append was not attempted on the failing store (call count 0) — AC-368"
    );
    assert_eq!(
        h.append_warnings().count(),
        1,
        "the failure is counted on the adapter's warning counter"
    );
    let w = h.append_warnings().warnings();
    assert!(
        matches!(
            w[0].error,
            StepError::Transport {
                kind: TransportKind::Io,
                ..
            }
        ),
        "the failure is TYPED: {:?}",
        w[0]
    );
    // R-16's reading under test (not a claim): the activity's own outcome stands.
    assert!(
        o.result.is_ok(),
        "R-16 under test: the outcome after a failed tier-2 append is the dispatch's own: {:?}",
        o.result
    );
}

/// The Send + Sync static assertion is in `lib.rs`; this is the runtime shape it guards — the
/// adapter is one object the composition root holds as `Arc<dyn Harness>` across threads
/// (ADR-0099 d4), constructed once, never per call.
#[test]
fn harness_is_held_as_arc_dyn_and_dispatches_from_another_thread() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let t = ok_transcript();
    let (config, spec) = bind_probe(
        spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "x",
    );
    let h: Arc<dyn Harness> = Arc::new(harness(config));
    let h2 = Arc::clone(&h);
    let o = thread::spawn(move || {
        let mut store = VecStore::default();
        h2.dispatch(&spec, &mut store)
    })
    .join()
    .expect("the dispatching thread");
    assert!(o.result.is_ok(), "{:?}", o.result);
    let _ = values();
}

/// The child's own verdict outranks a failed stdin write (the runtime-safety and
/// test-discrimination reviews of Slice 5, R5-2 / F1 / S5-03): the write runs on its own task,
/// concurrently with the child, and a child that never reads stdin (every fixture mode but
/// `wait-stdin-eof`; a real `claude` that dies at startup) fails the parent's ≥ 1 MiB write
/// whatever else it did — so the write failure may not decide the variant when the stream or the
/// exit code did. Three cells, each with `megabyte_prompt()`: `exit:7` → `NonZero { code: Some(7) }`
/// (never `Transport { Io }`); a replayed recorded `is_error` frame (e3.L14) → `Platform { status:
/// 500 }`; a usage-limit marker on stderr (the fixture child names an unknown mode on stderr and
/// exits 2 — the mode string IS the marker) → `RateLimited` with `detected_by: TextMarker`, the
/// class AC-523 forbids to backoff. The write failure is retained on the row's stderr as a marker
/// line, never lost. The fourth cell is `ac360_…`'s: `close-stdin-early` (exit 0, no frame) stays
/// `Transport { kind: Io }` — the write failure is the only thing that happened.
///
/// RED (observed 2026-09-18) against the classify order that consulted the write first:
/// `exit:7 with a 1 MiB prompt must be the child's own NonZero { code: Some(7) }, never the
/// write's Transport { Io }: Err(Transport { kind: Io, detail: "the prompt could not be written to
/// the child's stdin: The pipe has been ended. (os error 109)" })`.
#[test]
fn stdin_write_failure_never_overturns_the_childs_verdict() {
    let big = megabyte_prompt();
    // exit:7 — the child's verdict is its exit code
    let r = dispatch_mode("exit:7", 30, &big);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::NonZero { code: Some(7), .. })
        ),
        "exit:7 with a 1 MiB prompt must be the child's own NonZero {{ code: Some(7) }}, never the \
         write's Transport {{ Io }}: {:?}",
        r.outcome.result
    );
    let Retained::Verbatim(stderr) = &r.outcome.record.stderr else {
        panic!("small stderr");
    };
    assert!(
        String::from_utf8_lossy(stderr).contains("could not be written to the child's stdin"),
        "the write failure is retained on the row's stderr as a marker line, not lost: {:?}",
        String::from_utf8_lossy(stderr)
    );
    // a recorded is_error frame — Platform, whatever the write did (AC-218's twin)
    let platform = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("recorded/e3.L14.result_raw.jsonl"),
    ]);
    let r = dispatch_mode(&format!("replay:{}", platform.path.display()), 30, &big);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Platform { status: 500, .. })
        ),
        "a recorded is_error frame with a 1 MiB prompt is Platform {{ status: 500 }}: {:?}",
        r.outcome.result
    );
    // a usage-limit marker on stderr — RateLimited, disclosed, never a Backoff-class Transport
    let r = dispatch_mode("usage limit reached", 30, &big);
    assert!(
        matches!(r.outcome.result, Err(StepError::RateLimited { .. })),
        "a stderr text marker with a 1 MiB prompt is RateLimited (AC-523: never backoff): {:?}",
        r.outcome.result
    );
    assert_eq!(
        r.outcome.record.detected_by,
        Some(rk_types::usage::DetectedBy::TextMarker)
    );
    // and the Io path stands where the write failure is the only thing that happened
    let r = dispatch_mode("close-stdin-early", 30, &big);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Transport {
                kind: TransportKind::Io,
                ..
            })
        ),
        "exit 0, no frame, the write refused: Transport {{ Io }} (§7c.11): {:?}",
        r.outcome.result
    );
}

/// A token that has ALREADY fired when `dispatch` is entered is a pre-spawn refusal (R5-5 / F6 /
/// S5-06): after Ctrl-C at the root, no further `claude -p` is launched to be killed on the first
/// poll tick — `Cancelled { by: Signal }` with its row (`cli_version: Pinned`, `cancelled_by:
/// Some(Signal)`) and marker count 0. ADR-0099 decision 2's *"when the token fires, the adapter
/// kills the child tree"* is honoured by never starting one.
///
/// RED (observed 2026-09-18) against the tree that read the token only inside the race: `a fired
/// token must create no group: a child was spawned and killed on the first poll tick`. The marker
/// count is the weaker instrument — it read 0 on one run (the child killed on the first tick,
/// before it wrote its marker) and 1 on the next — so the group probe is asserted first.
#[test]
fn cancelled_token_refuses_before_spawn() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(spawn_config(&markers, "hold"), 60, wt.path(), "x");
    let token = CancelToken::new();
    token.cancel();
    let h = harness_with_token(config, token);
    let mut store = VecStore::default();
    let o = h.dispatch(&spec, &mut store);
    // The marker count alone races a child killed before it wrote its marker; the group probe
    // does not: a spawn creates a group, and a pre-spawn refusal creates none.
    assert!(
        h.last_kill_drop_order().is_none(),
        "a fired token must create no group: a child was spawned and killed on the first poll tick"
    );
    assert_eq!(
        markers.count(),
        0,
        "a fired token must spawn nothing (marker count 0): {}",
        markers.count()
    );
    assert!(
        matches!(
            o.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Signal
            })
        ),
        "{:?}",
        o.result
    );
    let rows = completed_rows(&store);
    assert_eq!(rows.len(), 1, "exactly one row (AC-360)");
    assert_eq!(
        rows[0].cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
    );
    assert_eq!(rows[0].cancelled_by, Some(CancelledBy::Signal));
}

/// The race is over the CHILD'S EXIT, as §7c.10 states it, not over the stdout drain (R5-3 / F8 /
/// S5-02): the fixture child in `replay-then-orphan` mode replays the good transcript, spawns an
/// `orphan-holder` that INHERITS its stdout/stderr pipes, and exits 0 — so the child has exited
/// while a descendant withholds EOF. The outcome must be decided by the exit and the frame (`Ok`),
/// well inside the card's bound, with the leftover reaped through the group (AC-278's shape) so
/// the pipes close — never `Cancelled { by: Timeout }` after `timeout_secs` waiting for an EOF.
///
/// RED (observed 2026-09-18) against the drain-as-exit race: `a child that exited leaving a stdout
/// holder is decided by its frame, never by the timer: Err(Cancelled { by: Timeout })` (after the
/// full 20 s bound).
#[test]
fn exit_with_stdout_holder_is_decided_by_the_frame() {
    let t = ok_transcript();
    let r = dispatch_mode(&format!("replay-then-orphan:{}", t.path.display()), 20, "x");
    assert!(
        r.outcome.result.is_ok(),
        "a child that exited leaving a stdout holder is decided by its frame, never by the timer: \
         {:?}",
        r.outcome.result
    );
    assert!(
        r.outcome.record.timings.wall_ms < rk_adapter_claude::runner::EXIT_GRACE_MS,
        "decided at the exit, not at a grace or the timer: {} ms",
        r.outcome.record.timings.wall_ms
    );
    let holders = r.markers.with_mode("orphan-holder");
    assert_eq!(
        holders.len(),
        1,
        "the holder came up: {:?}",
        r.markers.read()
    );
    assert!(
        crate::support::wait_dead(holders[0].pid),
        "the leftover holder (pid {}) survived the child's exit: it is reaped through the group \
         (AC-278's shape)",
        holders[0].pid
    );
    assert_eq!(completed_rows(&r.store).len(), 1);
}

/// RT-3 as amended, through the real race: a frame READ before the adapter's own timer fired does
/// not overturn the arm — the fixture child in `replay-then-hold` mode replays the good transcript
/// and then holds with stdout open, and the card's 1 s bound expires: `Cancelled { by: Timeout }`,
/// what was read before the kill retained on the row (`usage` from the frame — Halt 4: nothing is
/// compared across the kill; the reading is what was read before it), and the kill on the arm,
/// not on a grace (`wall_ms` under `EXIT_GRACE_MS`).
#[test]
fn frame_read_before_the_timer_fired_does_not_overturn_the_arm() {
    let t = ok_transcript();
    let r = dispatch_mode(&format!("replay-then-hold:{}", t.path.display()), 1, "x");
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Cancelled {
                by: CancelledBy::Timeout
            })
        ),
        "the timer arm classifies, frame or no frame (RT-3): {:?}",
        r.outcome.result
    );
    assert!(
        r.outcome.record.usage.is_some(),
        "what was read before the kill is on the row: {:?}",
        r.outcome.record.usage
    );
    assert!(
        r.outcome.record.timings.wall_ms < rk_adapter_claude::runner::EXIT_GRACE_MS,
        "killed on the arm, not after a grace: {} ms",
        r.outcome.record.timings.wall_ms
    );
    assert_eq!(
        r.outcome.record.cli_version,
        CliVersion::Observed("2.1.278".to_owned()),
        "the init was read before the kill"
    );
}

/// A stream that never carries a `system/init` line resolves to no `Ok` (S5-01): adoption
/// (AC-204), isolation (AC-202/AC-386) and the version decision (AC-226/AC-385) were never taken,
/// so a result frame without a preceding init is `NotIsolated { surface: Tools, diff_digest }`
/// over the empty reported set against the declared list (the rule `init.rs` applies to an
/// absent `tools` key), the row carries `cli_version: Pinned` (nothing was read — §7c.11), and
/// exactly one row is appended.
///
/// RED (observed 2026-09-18) against the tree that took the init decisions only on an init line:
/// `a result frame with no system/init before it must never be Ok: Ok(Typed { schema:
/// SchemaId("probe-ok"), value: Object {"ok": Bool(true)} })`.
#[test]
fn result_without_system_init_never_ok() {
    let t = transcript(&[Part::File("streams/result-ok.scripted.ndjson")]);
    let r = dispatch_replay(&t, 30);
    match &r.outcome.result {
        Err(StepError::NotIsolated {
            surface: rk_types::error::IsolationSurface::Tools,
            diff_digest,
        }) => {
            let want = rk_adapter_claude::init::set_digest(
                &std::collections::BTreeSet::new(),
                &std::iter::once("StructuredOutput".to_owned()).collect(),
            );
            assert_eq!(
                diff_digest, &want,
                "the digest is over the empty reported set against the declared list"
            );
        }
        other => panic!("a result frame with no system/init before it must never be Ok: {other:?}"),
    }
    assert_eq!(
        r.outcome.record.cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned()),
        "no init read: Pinned"
    );
    assert_eq!(completed_rows(&r.store).len(), 1);
}

/// F11 / R5-8: a child that spews past `STDIO_CAP` is retained as `Retained::Capped` whose digest
/// is over EVERY byte the child wrote (the record's shape) while the adapter accumulates a bounded
/// window, never the whole stream — the `spew:<bytes>` fixture mode with a known pattern, so the
/// expected digest is computable here. `total_len` is the byte count and the head/tail are the
/// pattern's ends. (Exit 0 with no frame: the variant is `Transport { Decode }`; the retention is
/// the observation.)
#[test]
fn spew_beyond_the_cap_is_capped_with_the_full_digest() {
    let n = 200_000usize;
    // [amended 2026-09-25, s8a] `spew-after-stdin-eof`: the same race as the `Decode` path in
    // `ac360_one_event_per_termination_path` — exit 0 with no frame is `Decode` only once the write
    // completed.
    let r = dispatch_mode(&format!("spew-after-stdin-eof:{n}"), 30, "x");
    let line = b"xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n";
    let mut expected = Vec::with_capacity(n);
    while expected.len() < n {
        let take = line.len().min(n - expected.len());
        expected.extend_from_slice(&line[..take]);
    }
    match &r.outcome.record.stdout {
        Retained::Capped {
            digest,
            head,
            tail,
            total_len,
        } => {
            assert_eq!(*total_len, n);
            assert_eq!(digest, &rk_types::digest::sha256(&expected));
            assert_eq!(head.as_slice(), &expected[..rk_types::retention::HEAD_TAIL]);
            assert_eq!(
                tail.as_slice(),
                &expected[n - rk_types::retention::HEAD_TAIL..]
            );
        }
        other => panic!("{n} bytes is past the cap: {other:?}"),
    }
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Transport {
                kind: TransportKind::Decode,
                ..
            })
        ),
        "{:?}",
        r.outcome.result
    );
}

/// R5-9: the sync port makes no promise about its caller's thread, and `Runtime::block_on` from
/// a thread inside ANOTHER tokio runtime panics (*"Cannot start a runtime from within a
/// runtime"*) — so `dispatch` runs its `block_on` on a scoped thread of its own and is callable
/// from inside any composition root's runtime. Here: a `current_thread` runtime the test owns,
/// `dispatch` called from inside its `block_on`.
///
/// RED (observed 2026-09-18) against the tree that called `block_on` on the caller's thread: the
/// test's runtime panicked with `Cannot start a runtime from within a runtime. This happens
/// because a function (like `block_on`) attempted to block the current thread while the thread is
/// being used to drive asynchronous tasks.`
#[test]
fn dispatch_is_callable_from_inside_another_runtime() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let t = ok_transcript();
    let (config, spec) = bind_probe(
        spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "x",
    );
    let h = harness(config);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("the caller's own runtime");
    let o = rt.block_on(async {
        let mut store = VecStore::default();
        h.dispatch(&spec, &mut store)
    });
    assert!(o.result.is_ok(), "{:?}", o.result);
}

/// **ADR-0118's principle, applied to the adapter's own locks.** A poisoned accumulator means its
/// holder panicked mid-update; the data behind it is not trusted and is never recovered through
/// `PoisonError::into_inner` in any spelling (the block's ban, `rk_absent 'PoisonError|catch_unwind|
/// unwrap_or_else(..into_inner'`). The seam is `runner::locked`: on poison it answers an `Err`
/// naming the record, and the dispatch that meets it fails closed as `Transport { Decode }` with its
/// row. RED before the fix: `locked` recovered the guard and this assertion read `Ok`.
#[test]
fn a_poisoned_accumulator_is_never_recovered() {
    let m = std::sync::Arc::new(std::sync::Mutex::new(0_u32));
    let poisoner = std::sync::Arc::clone(&m);
    let joined = std::thread::spawn(move || {
        let _guard = poisoner.lock().expect("first lock");
        panic!("the holder panics mid-update");
    })
    .join();
    assert!(joined.is_err(), "the poisoning thread must have panicked");
    assert!(m.is_poisoned(), "the lock must be poisoned by the panic");
    match rk_adapter_claude::runner::locked(&m) {
        Ok(_) => panic!("a poisoned lock was recovered — ADR-0118 forbids it in any spelling"),
        Err(detail) => assert!(
            detail.contains("ADR-0118") && detail.contains("poisoned"),
            "the refusal names the record and the fault: {detail}"
        ),
    }
}

/// **ADR-0190 decisions 1 and 3 (proposed; D21-107)** — the adapter writes the dispatching
/// activity's `<id>@<version>` on its OWN `dispatch_completed` row, verbatim from the spec, or a
/// PRESENT `null` when the spec carries none; and it keys that row with the spec's `"item"` input
/// beside the event (ADR-0182's carrier), never in the payload. *(Added 2026-09-29, s8f build,
/// package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL.)*
///
/// Three arms, each through the REAL `dispatch`: a dispatch through the child whose spec carries
/// `Some("probe-activity@3")` and an `"item"` input; the same with `None` and no item; and a
/// pre-spawn refusal (no worktree key) carrying `Some`, because the member rides on EVERY
/// termination path, not only the ones that reached the child. The returned record equals the row.
///
/// Red by: the member never copied (the row reads `null`); `M16` — the member written without its
/// `@version` (the row reads `probe-activity`); the row left item-less (`item()` is `None`).
#[test]
fn dispatch_completed_carries_activity_id_at_version_or_null() {
    let transcript = ok_transcript();
    let arm = |activity: Option<&str>, item: Option<&str>, keyless: bool| {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("a temp worktree");
        let config = spawn_config(&markers, &format!("replay:{}", transcript.path.display()));
        let extra: Vec<(&str, serde_json::Value)> = item
            .map(|i| vec![("item", serde_json::json!(i))])
            .unwrap_or_default();
        let (config, bound) =
            crate::support::bind_probe_with_inputs(config, 30, worktree.path(), "hello", &extra);
        let spec = ActivitySpec {
            card_digest: bound.card_digest.clone(),
            tools: bound.tools.clone(),
            schema: bound.schema.clone(),
            timeout_secs: bound.timeout_secs,
            inputs: bound
                .inputs
                .iter()
                .filter(|(k, _)| !keyless || k != WORKTREE_KEY)
                .cloned()
                .collect(),
            activity_id_at_version: activity.map(str::to_owned),
        };
        let mut store = VecStore::default();
        let outcome = harness(config).dispatch(&spec, &mut store);
        (outcome, store, markers)
    };

    // (1) through the child, Some, keyed.
    let (outcome, store, markers) = arm(Some("probe-activity@3"), Some("bug-7"), false);
    assert!(
        outcome.result.is_ok(),
        "PREMISE: the ok transcript dispatches Ok: {:?}",
        outcome.result
    );
    assert_eq!(markers.count(), 1, "PREMISE: the child ran");
    let rows: Vec<&rk_types::event::Event> = store
        .events()
        .iter()
        .filter(|e| e.kind == EventKind::DispatchCompleted)
        .collect();
    assert_eq!(rows.len(), 1, "exactly one dispatch_completed row");
    assert_eq!(
        rows[0].payload.get("activity_id_at_version"),
        Some(&serde_json::json!("probe-activity@3")),
        "the row carries the spec's `<id>@<version>` verbatim: {}",
        rows[0].payload
    );
    assert_eq!(
        outcome.record.activity_id_at_version.as_deref(),
        Some("probe-activity@3"),
        "the returned record IS the row"
    );
    assert_eq!(
        rows[0].item(),
        Some("bug-7"),
        "the row is keyed with the spec's `item` input, beside the event (ADR-0182)"
    );
    assert!(
        rows[0].payload.get("item").is_none(),
        "the item key never enters the payload (ADR-0131; ADR-0182)"
    );

    // (2) through the child, None, unkeyed: the member is a PRESENT null.
    let (outcome, store, _markers) = arm(None, None, false);
    assert!(outcome.result.is_ok(), "PREMISE: {:?}", outcome.result);
    let payload = completed_payload(&store);
    assert_eq!(
        payload.get("activity_id_at_version"),
        Some(&serde_json::Value::Null),
        "a spec with no activity writes a PRESENT null, never an omitted key: {payload}"
    );
    let row = store
        .events()
        .iter()
        .find(|e| e.kind == EventKind::DispatchCompleted)
        .expect("the row");
    assert_eq!(row.item(), None, "no `item` input, no key");

    // (3) a pre-spawn refusal still carries the member.
    let (outcome, store, markers) = arm(Some("probe-activity@3"), Some("bug-7"), true);
    assert!(
        matches!(
            outcome.result,
            Err(StepError::Transport {
                kind: TransportKind::Spawn,
                ..
            })
        ),
        "PREMISE: a spec without the worktree key is refused before spawn: {:?}",
        outcome.result
    );
    assert_eq!(markers.count(), 0, "PREMISE: nothing spawned");
    let payload = completed_payload(&store);
    assert_eq!(
        payload.get("activity_id_at_version"),
        Some(&serde_json::json!("probe-activity@3")),
        "the pre-spawn row carries the member too: {payload}"
    );

    // (4) D21-107 item 3's card-digest half: the binder copies the member onto the spec and reads
    // nothing from it — the same card bound with `Some` and with `None` has one card digest.
    let bind = |member: Option<&str>| {
        let markers = Markers::new();
        let worktree = tempfile::tempdir().expect("a temp worktree");
        let mut binder = rk_adapter_claude::binder::Binder::new(spawn_config(&markers, "exit:0"));
        binder
            .bind(rk_adapter_claude::binder::BindRequest {
                role_name: "haiku",
                role: &crate::support::role("haiku"),
                card: &crate::support::catalog_card("probe"),
                schema: &crate::support::schema(),
                timeout_secs: 30,
                worktree: worktree.path(),
                inputs: vec![("prompt".to_owned(), serde_json::json!("hello"))],
                activity_id_at_version: member.map(str::to_owned),
                // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)*
                models: &crate::support::models(),
            })
            .expect("the probe card binds")
    };
    let (with, without) = (bind(Some("probe-activity@3")), bind(None));
    assert_eq!(
        with.activity_id_at_version.as_deref(),
        Some("probe-activity@3"),
        "the binder copies the member onto the spec"
    );
    assert_eq!(
        with.card_digest, without.card_digest,
        "the member moved the card digest"
    );
}

/// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* `s8h` §8h.3:
/// `DispatchRecord.observed_model` is `Some` on every path that read a `system/init` carrying
/// `model` — the reported id EXACTLY as printed, on a refused path too — and `None` on every path
/// that terminated before one was read (pre-spawn, a child that wrote no init) or whose init
/// carried no `model`. It is never the requested id copied in (Halt 11's shape, the reasoning that
/// gives `cli_version` its two arms). The stored row carries the same member.
#[test]
fn observed_model_is_recorded_on_every_path_that_read_init() {
    use rk_types::usage::ObservedModel;
    let with = |init: &str| {
        crate::support::transcript(&[
            crate::support::Part::File(&format!("streams/{init}")),
            crate::support::Part::File("streams/result-ok.scripted.ndjson"),
        ])
    };
    let haiku = "claude-haiku-4-5-20251001";
    // Ok, through the init
    let r = dispatch_replay(&with("init.scripted.ndjson"), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.observed_model,
        Some(ObservedModel::new(haiku)),
        "a path that read an init carrying `model` records it (s8h §8h.3)"
    );
    assert_eq!(
        completed_payload(&r.store)["observed_model"],
        serde_json::json!(haiku),
        "the stored row carries it"
    );
    // a refusal decided ON the init line still read it
    let r = dispatch_replay(&with("init-wrong-agent.scripted.ndjson"), 30);
    assert!(r.outcome.result.is_err(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.observed_model,
        Some(ObservedModel::new(haiku)),
        "an init-time refusal read the init: Some"
    );
    // the reported id, never the requested one
    let r = dispatch_replay(&with("init-model-haiku-undated.scripted.ndjson"), 30);
    assert_eq!(
        r.outcome.record.observed_model,
        Some(ObservedModel::new("claude-haiku-4-5")),
        "the REPORTED id, never the requested {haiku} copied in"
    );
    // an init with no `model` key: nothing measured
    let r = dispatch_replay(&with("init-no-model.scripted.ndjson"), 30);
    assert_eq!(
        r.outcome.record.observed_model, None,
        "an init without `model` measured no model"
    );
    // a child that wrote no init at all
    let r = crate::support::dispatch_mode("exit:0", 30, "hello");
    assert!(r.outcome.result.is_err(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.observed_model, None,
        "a path that terminated before any init was read is None"
    );
    assert_eq!(
        completed_payload(&r.store)["observed_model"],
        serde_json::Value::Null
    );
    // pre-spawn: nothing was spawned
    let markers = crate::support::Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let mut v = crate::support::spawn_values(&markers, "exit:0");
    v.harness_binary = markers.path().join("absent.exe");
    let (config, spec) =
        crate::support::bind_probe(Config::new(v).expect("constructs"), 30, wt.path(), "x");
    let mut store = VecStore::default();
    let o = crate::support::harness(config).dispatch(&spec, &mut store);
    assert_eq!(o.record.observed_model, None, "pre-spawn is None");
}

// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate.)* `s8h` §8h.6 as ruled by D21-137
// (TAKEN PROVISIONALLY under D21-105): the settings FILE a dispatch writes, and its evidence on the
// row. ADR-0197 decision 9 as amended, proposed.

/// The `--settings` element a marker's argv carries.
fn settings_element(argv: &[String]) -> String {
    let at: Vec<usize> = argv
        .iter()
        .enumerate()
        .filter(|(_, a)| *a == "--settings")
        .map(|(i, _)| i)
        .collect();
    assert_eq!(at.len(), 1, "--settings occurs exactly once in {argv:?}");
    argv[at[0] + 1].clone()
}

/// D21-137 (b): a dispatch writes its settings file before the spawn, and the child's
/// `--settings` element names it. The file is in the caller-supplied directory (beside, never
/// under, the worktree), read-only, its bytes are the confinement JSON (the worktree's own sibling
/// is denied, the read fence and `autoMemoryEnabled: false` are set), and the record AND the
/// `dispatch_completed` row carry its path and the sha256 of exactly those bytes.
///
/// RED (observed 2026-10-02, through D21-47's wrapper, against the scaffold's writer): *"the
/// settings file the child was handed exists (D21-137 (b)): … (os error 2)"*.
#[test]
fn the_settings_file_is_the_json_and_its_sha256_is_recorded() {
    let markers = Markers::new();
    let tmp = tempfile::tempdir().expect("a temp dir");
    let wt = tmp.path().join("wt");
    std::fs::create_dir_all(&wt).expect("the worktree");
    std::fs::create_dir_all(tmp.path().join("sib")).expect("a sibling of the worktree");
    let t = ok_transcript();
    let config = spawn_config(&markers, &format!("replay:{}", t.path.display()));
    let (config, spec) = bind_probe(config, 30, &wt, "hello");
    let mut store = VecStore::default();
    let outcome = harness(config).dispatch(&spec, &mut store);
    assert!(outcome.result.is_ok(), "{:?}", outcome.result);
    let spawned = markers.read();
    assert_eq!(spawned.len(), 1, "exactly one spawn");
    let element = settings_element(&spawned[0].argv);
    let bytes = std::fs::read(&element).unwrap_or_else(|e| {
        panic!("the settings file the child was handed exists (D21-137 (b)): {element}: {e}")
    });
    let recorded = outcome
        .record
        .settings_file
        .clone()
        .expect("the record carries the settings file it delivered (D21-137 item 3)");
    assert_eq!(
        recorded.path, element,
        "the record names the file the argv named"
    );
    assert_eq!(
        recorded.sha256,
        rk_types::digest::sha256(&bytes),
        "the recorded sha256 is the delivered bytes'"
    );
    let payload = completed_payload(&store);
    assert_eq!(payload["settings_file"]["path"], serde_json::json!(element));
    assert_eq!(
        payload["settings_file"]["sha256"],
        serde_json::json!(recorded.sha256.as_str())
    );
    let path = std::path::Path::new(&element);
    assert!(path.is_absolute(), "the element is an absolute path");
    assert_eq!(
        path.parent(),
        Some(markers.settings_dir()),
        "the file is in the caller-supplied directory"
    );
    assert!(!path.starts_with(&wt), "the file is outside the worktree");
    assert!(
        std::fs::metadata(path)
            .expect("meta")
            .permissions()
            .readonly(),
        "the file is read-only"
    );
    let json: serde_json::Value = serde_json::from_slice(&bytes).expect("the bytes are JSON");
    assert_eq!(
        json["permissions"]["blockReadsOutsideWorkingDirectories"],
        serde_json::json!(true)
    );
    assert_eq!(json["autoMemoryEnabled"], serde_json::json!(false));
    let deny: Vec<&str> = json["permissions"]["deny"]
        .as_array()
        .expect("deny")
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    let level = tmp
        .path()
        .file_name()
        .and_then(|n| n.to_str())
        .expect("the temp dir has a UTF-8 name");
    let sib = format!("/{level}/sib/**)");
    let own = format!("/{level}/wt");
    assert!(
        deny.iter()
            .any(|r| r.starts_with("Read(") && r.ends_with(&sib)),
        "the worktree's own sibling is denied: the file is THIS worktree's settings"
    );
    assert!(
        !deny.iter().any(|r| r.contains(&own)),
        "the worktree itself is never denied"
    );
}

/// D21-137 (d): two dispatches of one bound spec through one harness write two DISTINCT files,
/// and the second leaves the first as it was (`create_new` under a per-dispatch name): each
/// record's sha256 still matches its own file after both ran.
///
/// RED (observed 2026-10-02, through D21-47's wrapper, against the scaffold's writer): *"each
/// dispatch's settings file exists (D21-137 (d)): … (os error 2)"*.
#[test]
fn two_dispatches_write_two_distinct_settings_files() {
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("a temp worktree");
    let t = ok_transcript();
    let config = spawn_config(&markers, &format!("replay:{}", t.path.display()));
    let (config, spec) = bind_probe(config, 30, wt.path(), "hello");
    let h = harness(config);
    let mut store = VecStore::default();
    let first = h.dispatch(&spec, &mut store);
    let second = h.dispatch(&spec, &mut store);
    let files: Vec<_> = [&first, &second]
        .iter()
        .map(|o| {
            o.record
                .settings_file
                .clone()
                .expect("each dispatch records its settings file")
        })
        .collect();
    assert_ne!(files[0].path, files[1].path, "two dispatches, two files");
    for f in &files {
        let bytes = std::fs::read(&f.path).unwrap_or_else(|e| {
            panic!(
                "each dispatch's settings file exists (D21-137 (d)): {}: {e}",
                f.path
            )
        });
        assert_eq!(
            f.sha256,
            rk_types::digest::sha256(&bytes),
            "{}: neither file was overwritten",
            f.path
        );
    }
    let in_dir = std::fs::read_dir(markers.settings_dir())
        .expect("the settings dir lists")
        .count();
    assert_eq!(
        in_dir, 2,
        "exactly the two files, in the supplied directory"
    );
}
