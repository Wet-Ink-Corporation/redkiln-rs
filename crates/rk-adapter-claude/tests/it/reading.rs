//! **The `rate_limit_reading` producer** — ADR-0163 decision clauses 2 and 3 (superseding `0133`
//! clauses 2 and 3 in part; AC-368, AC-374).
//!
//! Every dispatch here is the REAL one: `ClaudeHarness::dispatch` over the fixture child replaying
//! a transcript composed from `tests/fixtures/streams/`, with the `VecStore` fake (or the AC-368
//! `FailingStore`) as the assertion surface. What is pinned: ONE row per dispatch through the child,
//! from the stream's LAST representable reading, BEFORE its `dispatch_completed` row, whether or not
//! the dispatch succeeded; none when the stream carried no reading or a refused one; and the fixed
//! six-member payload — `windows`, `utilization`, `resets_at` as integer seconds, `rate_limit_type`,
//! `item`, `cli_version` (AC-227) — with `null` for what the frame did not carry.

use rk_ports::harness::Harness;
use rk_types::error::StepError;
use rk_types::event_kind::EventKind;

use crate::support::{
    FailingStore, Part, bind_probe_with_inputs, completed_rows, dispatch_replay,
    dispatch_replay_with_inputs, fixture, harness, kinds, reading_rows, spawn_config, transcript,
};

/// The unified reading every single-reading test here replays: five-hour 0.10, seven-day 0.81,
/// the five-hour window's `resetsAt` equal to the top-level one and the seven-day window's NOT.
const SEVEN_DAY_81: &str = "streams/rate-limit-seven-day-81.scripted.ndjson";

/// A transcript: the scripted init, `parts`, then the scripted `Ok` result.
fn init_parts_ok(parts: &[&str]) -> crate::support::Transcript {
    let mut all = vec![Part::File("streams/init.scripted.ndjson")];
    all.extend(parts.iter().map(|p| Part::File(p)));
    all.push(Part::File("streams/result-ok.scripted.ndjson"));
    transcript(&all)
}

/// ADR-0163 clause 3's `resets_at` and `windows`, over the recorded unified shape: exactly one
/// row, journalled BEFORE `dispatch_completed`; `resets_at.five_hour` / `.seven_day` are JSON
/// integers equal to each WINDOW's own `resetsAt`; `windows` are the frame's ticks and equal the
/// record's `headroom_after`; `rate_limit_type` verbatim.
///
/// Kills: the append deleted from `dispatch` (zero rows); the append moved after `finish` (kinds
/// reversed); `resets_at` read from the top-level `RateLimitReading.resets_at` for both windows
/// (seven-day 1788370800, not 1788408000 — the PREMISE assertion keeps that discriminating); a
/// formatted-date spelling (`is_i64` false); `windows` swapped or read through `ticks()`'s flat
/// fallback.
#[test]
fn rate_limit_reading_row_carries_integer_resets_at_per_window() {
    let frame: serde_json::Value =
        serde_json::from_str(fixture(SEVEN_DAY_81).trim_end()).expect("the fixture is one line");
    let info = &frame["rate_limit_info"];
    let top = info["resetsAt"].as_i64().expect("top-level resetsAt");
    let five_at = info["unifiedWindows"]["five_hour"]["resetsAt"]
        .as_i64()
        .expect("five_hour resetsAt");
    let seven_at = info["unifiedWindows"]["seven_day"]["resetsAt"]
        .as_i64()
        .expect("seven_day resetsAt");
    // PREMISE: the fixture discriminates a per-window read from a top-level one
    assert_ne!(
        seven_at, top,
        "premise: the seven-day window's resetsAt differs from the top-level one, or a top-level \
         read would pass this test"
    );
    // PREMISE: the ticks below are the frame's digits, not constants this test chose
    let text = fixture(SEVEN_DAY_81);
    assert!(
        text.contains(r#""five_hour":{"resetsAt":1788370800,"utilization":0.10}"#)
            && text.contains(r#""seven_day":{"resetsAt":1788408000,"utilization":0.81}"#),
        "premise: the fixture carries 0.10 / 0.81"
    );

    let r = dispatch_replay(&init_parts_ok(&[SEVEN_DAY_81]), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let rows = reading_rows(&r.store);
    assert_eq!(
        rows.len(),
        1,
        "exactly one rate_limit_reading row: {rows:?}"
    );
    assert_eq!(
        kinds(&r.store),
        vec![EventKind::RateLimitReading, EventKind::DispatchCompleted],
        "the reading precedes its dispatch_completed row (ADR-0163 clause 2)"
    );
    let row = &rows[0];
    assert!(
        row["resets_at"]["five_hour"].is_i64() && row["resets_at"]["seven_day"].is_i64(),
        "resets_at is integer unix seconds, never a formatted date: {row}"
    );
    assert_eq!(row["resets_at"]["five_hour"], five_at, "{row}");
    assert_eq!(
        row["resets_at"]["seven_day"], seven_at,
        "the seven-day window's OWN resetsAt: {row}"
    );
    assert_eq!(
        row["windows"],
        serde_json::json!({ "five_hour": 10, "seven_day": 81 }),
        "{row}"
    );
    assert_eq!(row["utilization"], serde_json::Value::Null, "{row}");
    assert_eq!(
        row["rate_limit_type"], info["rateLimitType"],
        "verbatim: {row}"
    );
    let completed = completed_rows(&r.store);
    let after = completed[0]
        .headroom_after
        .as_ref()
        .expect("the record's headroom_after");
    assert_eq!(
        row["windows"],
        serde_json::json!({ "five_hour": after.five_hour, "seven_day": after.seven_day }),
        "the row's windows are the record's headroom_after — one reading, two readers"
    );
    let completed_row = r
        .store
        .events()
        .iter()
        .find(|e| e.kind == EventKind::DispatchCompleted)
        .map(|e| e.payload.clone())
        .expect("the dispatch_completed row");
    assert_eq!(
        row.get("cli_version"),
        completed_row.get("cli_version"),
        "the reading carries its dispatch_completed row's cli_version, serialised alike (AC-227)"
    );
    assert!(
        row["cli_version"].is_object(),
        "premise: a serialised CliVersion, not null: {row}"
    );
    let mut keys: Vec<&str> = row
        .as_object()
        .expect("an object")
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(
        keys,
        [
            "cli_version",
            "item",
            "rate_limit_type",
            "resets_at",
            "utilization",
            "windows"
        ],
        "the fixed key set (ADR-0163 clause 3)"
    );
}

/// ADR-0163 clause 3's Flat arm: a flat frame journals `windows: null` and its one utilisation
/// under `utilization`, and `resets_at` through the accessor — the top-level value for both.
///
/// Kills: a payload built through `RateLimitReading::ticks()` (its flat arm writes the one
/// utilisation into a window — the extrapolation AC-374 forbids); `utilization` dropped or
/// written `null` on the flat shape.
#[test]
fn rate_limit_reading_flat_shape_carries_no_windows() {
    let r = dispatch_replay(
        &init_parts_ok(&["streams/rate-limit-flat-sdk.synthetic.ndjson"]),
        30,
    );
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let rows = reading_rows(&r.store);
    assert_eq!(rows.len(), 1, "{rows:?}");
    let row = &rows[0];
    assert_eq!(
        row["windows"],
        serde_json::Value::Null,
        "one utilisation is never written into a window pair (AC-374): {row}"
    );
    assert_eq!(row["utilization"], 37, "0.37 as ticks, verbatim: {row}");
    assert_eq!(
        row["resets_at"],
        serde_json::json!({ "five_hour": 1_788_370_800_i64, "seven_day": 1_788_370_800_i64 }),
        "a flat reading's resets_at falls back to the top-level value per window: {row}"
    );
    assert_eq!(row["rate_limit_type"], "five_hour", "{row}");
}

/// ADR-0163 clause 3's *"`rate_limit_type` — verbatim, or `null`"*, the `null` half: a unified
/// frame whose `rate_limit_info` carries no `rateLimitType` key journals `rate_limit_type` as JSON
/// `null` — the key present, the value `null`, never a defaulted window name — beside the frame's
/// own windows.
///
/// Kills: the member defaulted to a window (`"five_hour"`) when the frame names none; the key
/// omitted from the payload when the value is absent (the fixed key set, and `cost`'s
/// missing-member refusal, both need it present).
#[test]
fn rate_limit_reading_type_absent_is_null() {
    const NO_TYPE: &str = "streams/rate-limit-no-type.scripted.ndjson";
    // PREMISE: the frame carries no `rateLimitType`, and does carry both windows
    let frame: serde_json::Value =
        serde_json::from_str(fixture(NO_TYPE).trim_end()).expect("the fixture is one line");
    let info = frame["rate_limit_info"]
        .as_object()
        .expect("premise: a rate_limit_info object");
    assert!(
        !info.contains_key("rateLimitType"),
        "premise: the frame carries no rateLimitType: {frame}"
    );
    assert!(
        info.contains_key("unifiedWindows"),
        "premise: a unified reading: {frame}"
    );

    let r = dispatch_replay(&init_parts_ok(&[NO_TYPE]), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let rows = reading_rows(&r.store);
    assert_eq!(
        rows.len(),
        1,
        "exactly one rate_limit_reading row: {rows:?}"
    );
    let row = &rows[0];
    let payload = row.as_object().expect("the payload is an object");
    assert!(
        payload.contains_key("rate_limit_type"),
        "the key is present though the frame named no type: {row}"
    );
    assert_eq!(
        row["rate_limit_type"],
        serde_json::Value::Null,
        "an absent rateLimitType is JSON null, never a defaulted window: {row}"
    );
    assert_eq!(
        row["windows"],
        serde_json::json!({ "five_hour": 12, "seven_day": 34 }),
        "{row}"
    );
}

/// No `rate_limit_event` in the stream → no reading row; the dispatch still journals its
/// `dispatch_completed` row alone.
///
/// Kills: a row written unconditionally (an all-`null` reading, or the guard's previous one).
#[test]
fn rate_limit_reading_absent_writes_no_row() {
    let r = dispatch_replay(&init_parts_ok(&[]), 30);
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    assert!(reading_rows(&r.store).is_empty(), "{:?}", kinds(&r.store));
    assert_eq!(kinds(&r.store), vec![EventKind::DispatchCompleted]);
}

/// A reading the scanner refuses (a third decimal) after a representable one → no reading row:
/// `reading_refused` leaves `Run::last_reading` `None` (ADR-0163 clause 2), and the dispatch fails
/// closed as `Transport { Decode }`.
///
/// Kills: the producer reading the drain's last GOOD reading past a refusal (the runner's
/// `if reading_refused.is_some() { None }` removed — one row carrying 10 / 81).
#[test]
fn rate_limit_reading_refused_reading_writes_no_row() {
    let r = dispatch_replay(
        &init_parts_ok(&[
            SEVEN_DAY_81,
            "streams/rate-limit-three-decimals.synthetic.ndjson",
        ]),
        30,
    );
    assert!(
        matches!(
            r.outcome.result,
            Err(StepError::Transport {
                kind: rk_types::error::TransportKind::Decode,
                ..
            })
        ),
        "a refused reading fails closed: {:?}",
        r.outcome.result
    );
    assert!(
        reading_rows(&r.store).is_empty(),
        "a refused reading journals nothing: {:?}",
        kinds(&r.store)
    );
    assert_eq!(kinds(&r.store), vec![EventKind::DispatchCompleted]);
}

/// ADR-0163 clause 2's *"whether or not the dispatch succeeds"*: a reading followed by a result
/// whose structured output fails the schema → `Err`, and the reading is journalled all the same,
/// before the row.
///
/// Kills: the append guarded on `run.result.is_ok()`.
#[test]
fn rate_limit_reading_is_journalled_on_a_failed_dispatch() {
    let t = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File(SEVEN_DAY_81),
        Part::File("streams/result-invalid-structured-output.scripted.ndjson"),
    ]);
    let r = dispatch_replay(&t, 30);
    assert!(
        matches!(r.outcome.result, Err(StepError::Schema { .. })),
        "the structured output fails the schema: {:?}",
        r.outcome.result
    );
    let rows = reading_rows(&r.store);
    assert_eq!(
        rows.len(),
        1,
        "a failed dispatch through the child still journals its reading: {:?}",
        kinds(&r.store)
    );
    assert_eq!(
        rows[0]["windows"],
        serde_json::json!({ "five_hour": 10, "seven_day": 81 })
    );
    assert_eq!(
        kinds(&r.store),
        vec![EventKind::RateLimitReading, EventKind::DispatchCompleted]
    );
}

/// ADR-0163 clause 3's `item`: copied from the spec's `"item"` input when it is a string, `null`
/// otherwise — a spec with no item and a spec whose item is not a string both journal `null`.
///
/// Kills: `item` hard-wired `null` (the first arm); `item` read from a key other than `"item"`,
/// or a non-string serialised into it (the other two).
#[test]
fn rate_limit_reading_item_is_copied_from_spec_inputs_or_null() {
    let t = init_parts_ok(&[SEVEN_DAY_81]);
    let with = dispatch_replay_with_inputs(&t, 30, &[("item", serde_json::json!("bug-1"))]);
    assert!(with.outcome.result.is_ok(), "{:?}", with.outcome.result);
    let rows = reading_rows(&with.store);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["item"], "bug-1", "{}", rows[0]);

    let without = dispatch_replay_with_inputs(&t, 30, &[]);
    let rows = reading_rows(&without.store);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0]["item"],
        serde_json::Value::Null,
        "no item input: null, owned by nothing: {}",
        rows[0]
    );

    let non_string = dispatch_replay_with_inputs(&t, 30, &[("item", serde_json::json!(7))]);
    let rows = reading_rows(&non_string.store);
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0]["item"],
        serde_json::Value::Null,
        "a non-string item input is not an item: {}",
        rows[0]
    );
}

/// ADR-0163 clause 2's *"once per dispatch"*: two readings with different ticks in one stream →
/// exactly ONE row, carrying the LAST (five-hour 0.05, seven-day 0.40, the later `resetsAt`).
///
/// Kills: a row per parsed frame (`0133` clause 2's *"once per reading"* — two rows, `n` counting
/// frames); the FIRST reading kept instead of the last.
#[test]
fn rate_limit_reading_once_per_dispatch_carries_the_last() {
    let r = dispatch_replay(
        &init_parts_ok(&[
            SEVEN_DAY_81,
            "streams/rate-limit-allowed-later.scripted.ndjson",
        ]),
        30,
    );
    assert!(r.outcome.result.is_ok(), "{:?}", r.outcome.result);
    let rows = reading_rows(&r.store);
    assert_eq!(
        rows.len(),
        1,
        "one row per dispatch, not per frame: {rows:?}"
    );
    assert_eq!(
        rows[0]["windows"],
        serde_json::json!({ "five_hour": 5, "seven_day": 40 }),
        "the LAST reading: {}",
        rows[0]
    );
    assert_eq!(
        rows[0]["resets_at"],
        serde_json::json!({ "five_hour": 1_788_388_800_i64, "seven_day": 1_788_408_000_i64 }),
        "{}",
        rows[0]
    );
}

/// AC-368 over the second kind — PROVISIONAL as `ac368_tier2_append_failure_is_warning` is (R-16's
/// reading under test, not a claim): a store that refuses every append, a stream carrying a
/// reading → BOTH appends are attempted (the reading's, then the completion's), BOTH refusals are
/// counted on the adapter's one warning counter, and the dispatch's own outcome stands.
///
/// Kills: the reading append deleted (attempts 1); the reading append's failure not recorded on
/// `append_warnings` (count 1); a failed reading append short-circuiting the completion append
/// (attempts 1) or overturning the outcome.
#[test]
fn ac368_reading_append_failure_is_warning() {
    let markers = crate::support::Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let t = init_parts_ok(&[SEVEN_DAY_81]);
    let (config, spec) = bind_probe_with_inputs(
        spawn_config(&markers, &format!("replay:{}", t.path.display())),
        30,
        wt.path(),
        "x",
        &[("item", serde_json::json!("bug-1"))],
    );
    let h = harness(config);
    let mut store = FailingStore::default();
    let o = h.dispatch(&spec, &mut store);
    assert_eq!(
        store.attempts, 2,
        "the reading append AND the dispatch_completed append were both attempted (AC-368)"
    );
    assert_eq!(
        h.append_warnings().count(),
        2,
        "both refusals are counted on the adapter's one warning counter"
    );
    assert!(
        o.result.is_ok(),
        "R-16 under test: a refused reading append does not overturn the outcome: {:?}",
        o.result
    );
}
