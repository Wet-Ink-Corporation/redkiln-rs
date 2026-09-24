//! **The stream, consumed inside** — AC-200, AC-201, AC-206/AC-439/AC-282, AC-208, AC-219,
//! AC-232, AC-267, AC-523's two detection arms and `rate_limited_from_stream` (s7c §7c.3, §7c.7,
//! §7c.8, §7c.9, §7c.11, §7c.18). Every stream is composed from labelled parts
//! (`fixtures/streams/README.md`) and the byte-exact rows under `fixtures/recorded/`, replayed by
//! the fixture child through the real `dispatch`.

use proptest::prelude::*;
use rk_types::error::{RetryClass, StepError};
use rk_types::retention::Retained;
use rk_types::time::Timestamp;
use rk_types::usage::{CostUsd, DetectedBy, WindowReading};

use rk_adapter_claude::event::{EventKind, classify};

use crate::support::{
    Part, completed_rows, dispatch_replay, e1_smoke_rate_limit_event, transcript,
};

fn init_then(parts: &[Part<'_>]) -> crate::support::Transcript {
    let mut all: Vec<Part<'_>> = vec![Part::File("streams/init.scripted.ndjson")];
    for p in parts {
        all.push(match p {
            Part::File(f) => Part::File(f),
            Part::Bytes(b) => Part::Bytes(b.clone()),
        });
    }
    transcript(&all)
}

/// AC-200 — PROVISIONAL under the 64 KiB cap (§7c.3): *"the unknown event is present verbatim in
/// the event log for that run, with its raw JSON preserved"*. The carrier is
/// `DispatchRecord.stdout` (`Retained::Verbatim` under the cap). The recorded unknown lines are
/// the runner's re-serialised bytes (F20's five `tool_progress` frames; e1-smoke's
/// `rate_limit_event`), and THREE synthetic unknown types are injected beside them — every one
/// must be in the log byte-for-byte. The fixture is under the cap and the test asserts so.
///
/// RED (observed 2026-09-18) against the skeleton: `the retained stdout is empty: nothing was
/// spawned or nothing was read`.
#[test]
fn ac200_unknown_events_retained_verbatim() {
    let mut e1 = e1_smoke_rate_limit_event(0);
    e1.push(b'\n');
    let t = init_then(&[
        Part::File("recorded/f20.unknown_events.jsonl"),
        Part::Bytes(e1.clone()),
        Part::File("streams/unknown-events.synthetic.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    assert!(
        t.bytes.len() < rk_types::retention::STDIO_CAP,
        "under the cap"
    );
    let r = dispatch_replay(&t, 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let Retained::Verbatim(stdout) = &r.outcome.record.stdout else {
        panic!(
            "under the cap the stdout is Verbatim: {:?}",
            r.outcome.record.stdout
        );
    };
    assert!(
        !stdout.is_empty(),
        "the retained stdout is empty: nothing was spawned or nothing was read"
    );
    let contains = |needle: &[u8]| stdout.windows(needle.len()).any(|w| w == needle);
    // the three synthetic unknown types, byte-for-byte
    let synthetic =
        std::fs::read(crate::support::fixtures().join("streams/unknown-events.synthetic.ndjson"))
            .expect("synthetic");
    let lines: Vec<&[u8]> = synthetic
        .split(|b| *b == b'\n')
        .filter(|l| !l.is_empty())
        .collect();
    assert_eq!(lines.len(), 3);
    for l in &lines {
        assert!(
            contains(l),
            "an injected unknown line is not in the log byte-for-byte: {}",
            String::from_utf8_lossy(l)
        );
        assert!(matches!(
            classify(&String::from_utf8_lossy(l)).kind,
            EventKind::Unknown(_)
        ));
    }
    // the five recorded tool_progress frames and the recorded rate_limit_event line
    let f20 = std::fs::read(crate::support::fixtures().join("recorded/f20.unknown_events.jsonl"))
        .expect("f20");
    for l in f20.split(|b| *b == b'\n').filter(|l| !l.is_empty()) {
        assert!(
            contains(l),
            "a recorded unknown line is not in the log byte-for-byte"
        );
    }
    assert!(
        contains(&e1[..e1.len() - 1]),
        "the recorded rate_limit_event line, byte-for-byte"
    );
    // the whole transcript, in fact: the stdout IS the bytes the child wrote
    assert_eq!(stdout, &t.bytes, "the log is the stream's bytes, whole");
}

proptest! {
    /// AC-200's property half: a line with unrecognised fields classifies with `raw` equal to its
    /// input, and the retained bytes survive the record's serde round trip (the `Vec<u8>` form of
    /// `Retained` is byte-lossless — §7c.11).
    #[test]
    fn ac200_property_unrecognised_fields_round_trip(
        key in "[a-z_]{1,12}",
        value in any::<i64>(),
        ty in "[a-z_]{1,10}",
        tail in proptest::collection::vec(any::<u8>(), 0..64),
    ) {
        let line = format!("{{\"type\":\"{ty}\",\"{key}\":{value}}}");
        let parsed = classify(&line);
        prop_assert_eq!(&parsed.raw, &line);
        let retained = Retained::of(&tail);
        let json = serde_json::to_value(&retained).unwrap();
        let back: Retained = serde_json::from_value(json).unwrap();
        prop_assert_eq!(back, retained);
    }
}

/// AC-201: *"the dispatch still resolves to its normal outcome and no `StepError` is raised by
/// the unknown shape alone"* — a fixture stream carrying a future-shaped event (a `system`
/// subtype with a future field and a new top-level type) AND a result frame whose `usage` object
/// carries an unknown key (the second red: a `deny_unknown_fields` on `rk_types::Usage` fails
/// every dispatch whose usage block gains a field). Must yield `Ok`.
///
/// RED (observed 2026-09-18) against the skeleton: `a future-shaped event and an unknown usage
/// key must still yield Ok: got Err(Transport { kind: Spawn, … })`.
#[test]
fn ac201_future_shaped_event_still_ok() {
    let t = init_then(&[
        Part::File("streams/future-shaped-event.scripted.ndjson"),
        Part::File("streams/result-unknown-usage-key.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(
        r.outcome.result.is_ok(),
        "a future-shaped event and an unknown usage key must still yield Ok: got {:?}",
        r.outcome.result
    );
    let usage = r
        .outcome
        .record
        .usage
        .expect("the usage block parsed despite the unknown key");
    assert_eq!(usage.input_tokens, Some(10));
    // the decoder's own view: the two future lines are System(_) and Unknown(_), never refused
    let fut = std::fs::read_to_string(
        crate::support::fixtures().join("streams/future-shaped-event.scripted.ndjson"),
    )
    .expect("fut");
    let kinds: Vec<EventKind> = fut.lines().map(|l| classify(l).kind).collect();
    assert_eq!(
        kinds,
        vec![
            EventKind::System("thinking_tokens".to_owned()),
            EventKind::Unknown("telemetry_v9".to_owned())
        ]
    );
}

/// AC-206 / AC-439 / AC-282 (adapter half): *"exit 0 with `subtype: success` but no
/// `structured_output` … is `Err(StepError::Schema)` and never `Ok`"* — over the RECORDED F20
/// result frame (`recorded/f20.result_raw.jsonl`), replayed by a child that exits 0. The
/// fail-open path F20 records is unrepresentable: there is no `Ok` without a validated value.
///
/// RED (observed 2026-09-18) against the skeleton: `the F20 frame must classify Schema: got
/// Err(Transport { kind: Spawn, … })`.
#[test]
fn ac206_exit0_success_without_structured_output_is_schema() {
    let t = init_then(&[Part::File("recorded/f20.result_raw.jsonl")]);
    let r = dispatch_replay(&t, 30);
    match &r.outcome.result {
        Err(StepError::Schema { last_error, .. }) => assert!(
            last_error.contains("structured_output"),
            "the reason names the absent value: {last_error}"
        ),
        other => panic!("the F20 frame must classify Schema: got {other:?}"),
    }
    // the exit code was 0 and the frame said success — neither is evidence (Halt 3)
    assert_eq!(
        r.outcome.record.usage.as_ref().and_then(|u| u.input_tokens),
        Some(74)
    );
    assert_eq!(
        r.outcome.record.total_cost_usd,
        Some(CostUsd("2.337147".to_owned())),
        "the recorded cost digits, verbatim (RT-19)"
    );
    assert_eq!(r.outcome.record.timings.api_ms, Some(301_389));
}

/// AC-208: *"no structured value is recovered by reading the assistant's prose, and the failure
/// surfaces as `StepError::Schema`"* — a fixture whose prose IS valid JSON (`{"ok": true}`) and
/// which carries no `structured_output` must still fail. The lint half is the block's ban.
///
/// RED (observed 2026-09-18) against the skeleton (the runner's text-as-value fallback): `prose
/// carrying valid JSON must never be recovered as Ok (AC-208): got Ok(Typed { … value: String("{"ok":
/// true}") })`.
#[test]
fn ac208_prose_json_never_recovered() {
    let t = init_then(&[Part::File(
        "streams/result-prose-json-no-structured-output.scripted.ndjson",
    )]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::Schema { .. })),
        "prose carrying valid JSON must never be recovered as Ok (AC-208): got {:?}",
        r.outcome.result
    );
    // and a structured value the schema refuses is Schema too, never Ok
    let t = init_then(&[Part::File(
        "streams/result-invalid-structured-output.scripted.ndjson",
    )]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::Schema { .. })),
        "a structured_output the schema refuses is Schema: {:?}",
        r.outcome.result
    );
}

/// AC-219: *"the values come from the top-level `rate_limit_event` … and `result.rate_limit` is
/// never treated as a source"* — a stream where `result.rate_limit` is null and the recorded
/// e1-smoke `rate_limit_event` carries values: the event's values are recorded as
/// `headroom_after`. The lint half is the block's ban on the token.
///
/// RED (observed 2026-09-18) against the skeleton: `headroom_after must be the event's reading
/// (0.04/0.04 → 4/4): None`.
#[test]
fn ac219_headroom_from_rate_limit_event_only() {
    let mut e1 = e1_smoke_rate_limit_event(1);
    e1.push(b'\n');
    let t = init_then(&[
        Part::Bytes(e1),
        Part::File("streams/result-ok-null-rate-limit.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.headroom_after,
        Some(WindowReading {
            five_hour: 4,
            seven_day: 4
        }),
        "headroom_after must be the event's reading (0.04/0.04 → 4/4): {:?}",
        r.outcome.record.headroom_after
    );
    assert_eq!(
        r.outcome.record.headroom_before, None,
        "the opening dispatch read nothing before"
    );
}

/// AC-232 (adapter half): *"each [`system/api_retry` event] is recorded and counted … and the
/// adapter issues no retry of its own"* — over the two lines RECONSTRUCTED from
/// `results/e3.jsonl` line 6 (`recorded/e3.L6.api_retries.jsonl`; `streams/api-retry.reconstructed.ndjson`):
/// `api_retry_events == 2` (one charge per EVENT, ADR-0122 d1 — both entries are `attempt: 1`)
/// and exactly one child spawn (the marker count).
///
/// RED (observed 2026-09-18) against the skeleton: `two api_retry events must count 2: 0`.
#[test]
fn ac232_api_retry_counted_per_event_no_own_retry() {
    // the reconstruction is honest about its source: the two lines' fields are line 6's
    let parsed: Vec<serde_json::Value> = std::fs::read_to_string(
        crate::support::fixtures().join("recorded/e3.L6.api_retries.jsonl"),
    )
    .expect("e3.L6")
    .lines()
    .next()
    .map(|l| serde_json::from_str(l).expect("an array"))
    .expect("one line");
    assert_eq!(parsed.len(), 2);
    let lines = std::fs::read_to_string(
        crate::support::fixtures().join("streams/api-retry.reconstructed.ndjson"),
    )
    .expect("reconstructed");
    for (line, src) in lines.lines().zip(&parsed) {
        let v: serde_json::Value = serde_json::from_str(line).expect("json");
        for k in [
            "attempt",
            "max_retries",
            "retry_delay_ms",
            "error",
            "error_status",
        ] {
            assert_eq!(v[k], src[k], "the reconstructed line carries line 6's {k}");
        }
        assert_eq!(classify(line).kind, EventKind::ApiRetry);
    }
    let t = init_then(&[
        Part::File("streams/api-retry.reconstructed.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.api_retry_events, 2,
        "two api_retry events must count 2: {}",
        r.outcome.record.api_retry_events
    );
    assert_eq!(
        r.markers.count(),
        1,
        "exactly one child spawn — no retry of the adapter's own"
    );
}

/// AC-267 — PROVISIONAL, built and NOT claimed (§7c.9; Halt 9): the record's usage is the
/// dispatch's total as the harness reports it on the result frame — the SUM across the turns,
/// never the last assistant turn's — and `total_cost_usd` / the counters are `Option`, never
/// `Some(0)`. The `gate_resumed` fill path is R-17's and is not reachable from here.
///
/// RED (observed 2026-09-18) against the skeleton: `the record's usage is the result frame's sum
/// across turns: None`.
#[test]
fn ac267_resume_cost_is_summed_across_turns_and_option() {
    let t = init_then(&[
        Part::File("streams/assistant-turn-1.scripted.ndjson"),
        Part::File("streams/assistant-turn-2.scripted.ndjson"),
        Part::File("streams/result-two-turns-summed.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    let usage = r
        .outcome
        .record
        .usage
        .clone()
        .expect("the record's usage is the result frame's sum across turns: None");
    assert_eq!(usage.input_tokens, Some(10), "4 + 6, not the last turn's 6");
    assert_eq!(
        usage.cache_creation_input_tokens,
        Some(50),
        "20 + 30, not 30"
    );
    assert_eq!(usage.cache_read_input_tokens, Some(100));
    assert_eq!(
        r.outcome.record.total_cost_usd,
        Some(CostUsd("0.0031".to_owned()))
    );
    // absent stays None, never Some(0)
    let t = init_then(&[Part::File("streams/result-no-usage.scripted.ndjson")]);
    let r = dispatch_replay(&t, 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert_eq!(
        r.outcome.record.usage, None,
        "no usage reported: None, never Some(0)"
    );
    assert_eq!(r.outcome.record.total_cost_usd, None);
}

/// AC-523's `text_marker` arm = the stage's `rate_limited_from_stream`: a usage-limit RESULT
/// (the text marker `Usage limit reached`, no structured subtype) after a scripted non-`allowed`
/// `rate_limit_event` classifies `RateLimited { resets_at }` with `resets_at` taken from that
/// event — red by `resets_at: None` hard-coded (the runner's shape) or a backoff attempt (a
/// second marker); a weaker red accepts a `resets_at` read from the result frame.
///
/// RED (observed 2026-09-18) against the skeleton: `a usage-limit result must classify RateLimited
/// { resets_at: Some(1788370800) }: got Err(Transport { kind: Spawn, … })`.
#[test]
fn rate_limited_from_stream() {
    let t = init_then(&[
        Part::File("streams/rate-limit-non-allowed.scripted.ndjson"),
        Part::File("streams/result-usage-limit-text-marker.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    match &r.outcome.result {
        Err(StepError::RateLimited { resets_at }) => assert_eq!(
            *resets_at,
            Some(Timestamp::from_unix_seconds(1_788_370_800)),
            "resets_at is the last non-allowed rate_limit_event's, never hard-coded None"
        ),
        other => panic!(
            "a usage-limit result must classify RateLimited {{ resets_at: Some(1788370800) }}: got \
             {other:?}"
        ),
    }
    let Err(err) = &r.outcome.result else {
        unreachable!()
    };
    assert_eq!(
        err.retry_class(),
        RetryClass::WaitUntilReset,
        "never backoff (rk-types' table)"
    );
    assert_eq!(r.markers.count(), 1, "no attempt of the adapter's own");
    // with no non-allowed event in the stream: None, so the kernel falls back to the pack delay
    let t = init_then(&[Part::File(
        "streams/result-usage-limit-text-marker.scripted.ndjson",
    )]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::RateLimited { resets_at: None })
        ),
        "{:?}",
        r.outcome.result
    );
}

/// AC-523's AND: *"the detection surface is recorded on the `rate_limited` event (`detected_by:
/// subtype | text_marker`), and a text-marker detection is never silent"* — the disclosed
/// fallback writes `detected_by: TextMarker` on the row; red by `subtype` or nothing.
///
/// RED (observed 2026-09-18) against the skeleton: `a text-marker detection is disclosed as
/// TextMarker on the row: None`.
#[test]
fn ac523_text_marker_detection_disclosed() {
    let t = init_then(&[
        Part::File("streams/rate-limit-non-allowed.scripted.ndjson"),
        Part::File("streams/result-usage-limit-text-marker.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::RateLimited { .. })),
        "{:?}",
        r.outcome.result
    );
    assert_eq!(
        r.outcome.record.detected_by,
        Some(DetectedBy::TextMarker),
        "a text-marker detection is disclosed as TextMarker on the row: {:?}",
        r.outcome.record.detected_by
    );
    // selected by KIND: the non-allowed reading in this transcript is representable, so the
    // adapter journals a rate_limit_reading BEFORE the dispatch_completed row (ADR-0163 clause 2)
    // and position 0 is the reading, not the record
    let completed: Vec<_> = r
        .store
        .events()
        .iter()
        .filter(|e| e.kind == rk_types::event_kind::EventKind::DispatchCompleted)
        .collect();
    assert_eq!(completed.len(), 1, "exactly one dispatch_completed row");
    let payload = &completed[0].payload;
    assert_eq!(payload["detected_by"], "text_marker");
    // and an Ok row carries no detection
    let r = dispatch_replay(&crate::support::ok_transcript(), 30);
    assert_eq!(r.outcome.record.detected_by, None);
}

/// AC-523's `subtype` arm — PROVISIONAL, built, NOT claimed, [surface unverified]
/// (`// SCRIPTED — the CLI's usage-limit subtype string is an assumption`; the corpus carries
/// `success` on every result frame at the pin): a SYNTHETIC frame with `subtype:
/// error_usage_limit` after a non-`allowed` event classifies `RateLimited` with `detected_by:
/// Subtype` and `resets_at` from the event; red by `detected_by` written as `text_marker` or by
/// `resets_at` not taken from the last non-allowed event.
///
/// RED (observed 2026-09-18) against the skeleton: `the structured subtype arm is disclosed as
/// Subtype: None`.
#[test]
fn ac523_subtype_detection_synthetic_frame() {
    let t = init_then(&[
        Part::File("streams/rate-limit-non-allowed.scripted.ndjson"),
        Part::File("streams/result-usage-limit-subtype.synthetic.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    match &r.outcome.result {
        Err(StepError::RateLimited { resets_at }) => {
            assert_eq!(
                *resets_at,
                Some(Timestamp::from_unix_seconds(1_788_370_800))
            );
        }
        other => panic!("the synthetic subtype frame must classify RateLimited: {other:?}"),
    }
    assert_eq!(
        r.outcome.record.detected_by,
        Some(DetectedBy::Subtype),
        "the structured subtype arm is disclosed as Subtype: {:?}",
        r.outcome.record.detected_by
    );
}

/// §7c.7's *"`resets_at` from the LAST NON-ALLOWED `rate_limit_event`"*, discriminated (the
/// test-discrimination review of Slice 5, F3): a non-allowed reading (`resetsAt` 1788370800) THEN
/// an `allowed` reading with a DIFFERENT `resetsAt` (1788388800 — the ordinary shape once the
/// window turns over) THEN the text-marker result — `resets_at` is the non-allowed reading's, not
/// the last reading's; and the converse, only `allowed` readings then the marker → `None`.
///
/// RED (observed 2026-09-18) against the mutant `last_non_allowed: state.last_reading.as_ref()`:
/// `resets_at is the LAST NON-ALLOWED reading's (1788370800), never the last reading's: Some(Timestamp(1788388800))`.
#[test]
fn resets_at_from_last_non_allowed_not_last_reading() {
    let t = init_then(&[
        Part::File("streams/rate-limit-non-allowed.scripted.ndjson"),
        Part::File("streams/rate-limit-allowed-later.scripted.ndjson"),
        Part::File("streams/result-usage-limit-text-marker.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    match &r.outcome.result {
        Err(StepError::RateLimited { resets_at }) => assert_eq!(
            *resets_at,
            Some(Timestamp::from_unix_seconds(1_788_370_800)),
            "resets_at is the LAST NON-ALLOWED reading's (1788370800), never the last reading's: \
             {resets_at:?}"
        ),
        other => panic!("RateLimited: {other:?}"),
    }
    // headroom_after is still the LAST reading (the closest to the next dispatch's cost)
    assert_eq!(
        r.outcome.record.headroom_after,
        Some(WindowReading {
            five_hour: 5,
            seven_day: 40
        })
    );
    // the converse: allowed readings only → None (the kernel falls back to the pack delay)
    let t = init_then(&[
        Part::File("streams/rate-limit-allowed-later.scripted.ndjson"),
        Part::File("streams/rate-limit-allowed-later.scripted.ndjson"),
        Part::File("streams/result-usage-limit-text-marker.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::RateLimited { resets_at: None })
        ),
        "no non-allowed reading in the stream: None, never an allowed reading's resetsAt: {:?}",
        r.outcome.result
    );
}

/// A reading the instrument cannot represent fails the dispatch CLOSED — `Transport { kind: Decode
/// }` naming the key path (AC-374, AC-375; §7c.8) — WITHOUT a mid-run kill (the reviews of Slice
/// 5, F4 / S5-09): the drain runs to EOF, so the row keeps the result frame's usage and cost, the
/// child spawned exactly once, exactly one row, `headroom_after: None` (no representable reading
/// was taken) and the guard's reading unchanged. Two shapes: a third decimal (`0.045`) and one
/// window of two.
///
/// RED (observed 2026-09-18) against the log-and-continue mutant (`if false && e != NoUtilization`):
/// `streams/rate-limit-three-decimals.synthetic.ndjson: a reading the instrument cannot represent
/// fails closed as Transport { Decode } (AC-374/AC-375): Ok(Typed { … })`; and against the
/// mid-run kill this replaced: `the drain ran to EOF — the result frame's usage is on the row (no
/// kill): None`.
#[test]
fn refused_reading_is_decode_without_a_kill() {
    for (part, names) in [
        (
            "streams/rate-limit-three-decimals.synthetic.ndjson",
            "more than two decimals",
        ),
        (
            "streams/rate-limit-one-window.synthetic.ndjson",
            "seven_day",
        ),
    ] {
        let t = init_then(&[
            Part::File(part),
            Part::File("streams/result-ok.scripted.ndjson"),
        ]);
        let r = dispatch_replay(&t, 30);
        match &r.outcome.result {
            Err(StepError::Transport {
                kind: rk_types::error::TransportKind::Decode,
                detail,
            }) => assert!(
                detail.contains("rate_limit_event") && detail.contains(names),
                "{part}: the detail names the refusal: {detail}"
            ),
            other => panic!(
                "{part}: a reading the instrument cannot represent fails closed as Transport {{ \
                 Decode }} (AC-374/AC-375): {other:?}"
            ),
        }
        assert!(
            r.outcome.record.usage.is_some(),
            "{part}: the drain ran to EOF — the result frame's usage is on the row (no kill): {:?}",
            r.outcome.record.usage
        );
        assert_eq!(r.outcome.record.headroom_after, None, "{part}");
        assert_eq!(r.markers.count(), 1, "{part}: one spawn");
        assert_eq!(completed_rows(&r.store).len(), 1, "{part}: one row");
    }
    // the guard learned nothing from a refused reading: the next dispatch on the same harness is
    // not refused and the harness's last reading is None
    let markers = crate::support::Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let t = init_then(&[
        Part::File("streams/rate-limit-three-decimals.synthetic.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    let (config, spec) = crate::support::bind_probe(
        crate::support::spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "x",
    );
    let h = crate::support::harness(config);
    let mut store = rk_ports::fakes::VecStore::default();
    let _ = rk_ports::harness::Harness::dispatch(&h, &spec, &mut store);
    assert_eq!(h.last_reading(), None, "the guard's reading is unchanged");
}

/// AC-523's marker haystack, pinned (S5-12): the runner's `detect_usage_limit` scanned the WHOLE
/// result frame lower-cased; the lift scans the result TEXT member (`text` / `result`) and stderr
/// only — an edit named in `classify.rs`'s header — so a valid `structured_output` whose own text
/// quotes a marker (`plan limit`) is `Ok`, not a usage limit. Red by the whole-frame haystack.
#[test]
fn structured_output_quoting_a_marker_is_not_a_usage_limit() {
    let t = init_then(&[Part::File(
        "streams/result-ok-structured-output-quotes-marker.scripted.ndjson",
    )]);
    let r = dispatch_replay(&t, 30);
    assert!(
        r.outcome.result.is_ok(),
        "a marker quoted inside a valid structured_output is not a usage limit: {:?}",
        r.outcome.result
    );
    assert_eq!(r.outcome.record.detected_by, None);
    // the same words in the result TEXT with no structured_output ARE the disclosed fallback
    let t = init_then(&[Part::File(
        "streams/result-usage-limit-text-marker.scripted.ndjson",
    )]);
    let r = dispatch_replay(&t, 30);
    assert_eq!(r.outcome.record.detected_by, Some(DetectedBy::TextMarker));
}
