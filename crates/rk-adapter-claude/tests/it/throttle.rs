//! **The throttle in ticks** — AC-376, AC-220, AC-221, AC-378 (through the real `dispatch`) and
//! the batch-delta calculator — `tick_arithmetic_vectors`, AC-372's three, AC-381's positive
//! half (s7c §7c.8, §7c.18; ADR-0110; O-3, O-7).

use rk_ports::fakes::VecStore;
use rk_ports::harness::Harness;
use rk_types::error::StepError;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::usage::{CliVersion, WindowReading};

use rk_adapter_claude::config::{Margins, TESTED_CLI_VERSION, Window};
use rk_adapter_claude::headroom::{read_rate_limit_event, ticks_from_decimal};
use rk_adapter_claude::throttle::{
    Attribution, Guard, LOW_SAMPLE_BELOW, Sample, Verdict, attribute, batch_draw,
    batch_draws_by_reset,
};

use crate::support::{
    Markers, Part, bind_probe, completed_rows, e1_smoke_rate_limit_event, harness, margins_ok,
    spawn_config, transcript,
};

fn reading(five: &str, seven: &str) -> rk_adapter_claude::headroom::RateLimitReading {
    let line = format!(
        "{{\"type\":\"rate_limit_event\",\"rate_limit_info\":{{\"status\":\"allowed\",\
         \"resetsAt\":1788370800,\"isUsingOverage\":false,\"unifiedWindows\":{{\"five_hour\":\
         {{\"resetsAt\":1788370800,\"utilization\":{five}}},\"seven_day\":{{\"resetsAt\":1788408000,\
         \"utilization\":{seven}}}}}}}}}"
    );
    read_rate_limit_event(line.as_bytes()).expect("a scripted reading parses")
}

/// AC-220: *"the margin is at least two instrument ticks"* — over a margin table including the
/// 0.02 boundary: at margin 2 the guard refuses at 98 ticks and proceeds at 97; at the register's
/// 0.07 / 0.18 margins against the bases 0.09 / 0.10 / 0.20 nothing fires (those bases are far
/// from any ceiling), and at 93 / 92 (five_hour, margin 7) and 82 / 81 (seven_day, margin 18) the
/// boundary and one below. A weaker red accepts a margin of 1 tick — `Config::new` refuses it
/// (`ac377_…`), and the guard's arithmetic is what this table pins.
///
/// RED (observed 2026-09-18) against the skeleton (the guard decides nothing): `at margin 2 the
/// guard refuses at 98 ticks (100 - 2): Proceed`.
#[test]
fn ac220_refuse_within_margin_two_ticks() {
    let m2 = Margins {
        five_hour: 2,
        seven_day: 2,
    };
    let mut g = Guard::empty();
    g.observe(&reading("0.98", "0.10"));
    assert!(
        matches!(
            g.check(m2),
            Verdict::Halt {
                window: Window::FiveHour,
                ticks: 98,
                margin_ticks: 2,
                ..
            }
        ),
        "at margin 2 the guard refuses at 98 ticks (100 - 2): {:?}",
        g.check(m2)
    );
    g.observe(&reading("0.97", "0.10"));
    assert_eq!(
        g.check(m2),
        Verdict::Proceed,
        "one tick below the boundary proceeds"
    );
    // the register's decimal margins 0.07 / 0.18 (7 / 18 ticks) against the bases
    let m = margins_ok();
    for (five, seven) in [("0.09", "0.09"), ("0.10", "0.10"), ("0.20", "0.20")] {
        g.observe(&reading(five, seven));
        assert_eq!(
            g.check(m),
            Verdict::Proceed,
            "base {five}/{seven} is far from the ceiling"
        );
    }
    g.observe(&reading("0.93", "0.10"));
    assert!(
        matches!(
            g.check(m),
            Verdict::Halt {
                window: Window::FiveHour,
                ticks: 93,
                margin_ticks: 7,
                ..
            }
        ),
        "{:?}",
        g.check(m)
    );
    g.observe(&reading("0.92", "0.10"));
    assert_eq!(g.check(m), Verdict::Proceed);
    g.observe(&reading("0.10", "0.82"));
    assert!(
        matches!(
            g.check(m),
            Verdict::Halt {
                window: Window::SevenDay,
                ticks: 82,
                margin_ticks: 18,
                ..
            }
        ),
        "{:?}",
        g.check(m)
    );
    g.observe(&reading("0.10", "0.81"));
    assert_eq!(g.check(m), Verdict::Proceed);
    // the opening dispatch is unguarded (lifted)
    assert_eq!(Guard::empty().check(m), Verdict::Proceed);
}

fn replay_config(
    markers: &Markers,
    t: &crate::support::Transcript,
) -> rk_adapter_claude::config::Config {
    spawn_config(markers, &format!("replay:{}", t.path.display()))
}

/// AC-376 — PROVISIONAL, built and tested, NOT claimed (§7c.8): *"`ticks >= 100 - margin_ticks`
/// / THEN the adapter refuses to dispatch and returns `RateLimited { resets_at }` without spawning
/// a process"* — a scripted `rate_limit_event` at the boundary (five_hour 0.93 at margin 0.07) on
/// a FIRST dispatch, then the SECOND dispatch refused: `Err(RateLimited { resets_at })` from the
/// return value, exactly one `dispatch_completed` row in the fake with `cli_version: Pinned` and
/// `headroom_before` the reading that fired, the marker count unchanged (no spawn), and **no
/// `rate_limited` row written by the adapter** (the kernel's kind). One tick below (0.92): the
/// second dispatch proceeds.
///
/// RED (observed 2026-09-18) against the skeleton: `the first dispatch must proceed (it learns
/// the reading): Err(Transport { kind: Spawn, … })`.
#[test]
fn ac376_throttle_refuses_without_spawn_and_appends_row() {
    let at = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("streams/rate-limit-at-boundary-five-hour-93.scripted.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(replay_config(&markers, &at), 30, wt.path(), "x");
    let h = harness(config);
    let mut first = VecStore::default();
    let o1 = h.dispatch(&spec, &mut first);
    assert!(
        o1.result.is_ok(),
        "the first dispatch must proceed (it learns the reading): {:?}",
        o1.result
    );
    assert_eq!(markers.count(), 1);
    assert_eq!(
        o1.record.headroom_after,
        Some(WindowReading {
            five_hour: 93,
            seven_day: 10
        })
    );
    let mut store = VecStore::default();
    let o2 = h.dispatch(&spec, &mut store);
    match &o2.result {
        Err(StepError::RateLimited { resets_at }) => assert_eq!(
            *resets_at,
            Some(Timestamp::from_unix_seconds(1_788_370_800)),
            "the firing window's own resetsAt"
        ),
        other => panic!("at the boundary the dispatch is refused RateLimited: {other:?}"),
    }
    assert_eq!(
        markers.count(),
        1,
        "no child process was spawned in the refusing case (AC-376)"
    );
    let rows = completed_rows(&store);
    assert_eq!(
        rows.len(),
        1,
        "exactly one dispatch_completed refusal row in the fake"
    );
    assert_eq!(
        rows[0].cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
    );
    assert_eq!(
        rows[0].headroom_before,
        Some(WindowReading {
            five_hour: 93,
            seven_day: 10
        }),
        "headroom_before is the reading that fired"
    );
    assert_eq!(rows[0].headroom_after, None);
    assert!(
        store
            .events()
            .iter()
            .all(|e| e.kind == EventKind::DispatchCompleted),
        "no rate_limited row was written by the ADAPTER — that kind is the kernel's one producer \
         (§7c.8) — and no rate_limit_reading either: a pre-spawn refusal holds only the PREVIOUS \
         dispatch's reading, and re-journalling it double-counts a sample (ADR-0163 clause 2): \
         {:?}",
        store.events().iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    assert!(
        !store
            .events()
            .iter()
            .any(|e| e.kind == EventKind::RateLimited),
        "the kernel's kind"
    );
    // one tick below: proceeds
    let below = transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::File("streams/rate-limit-one-below-five-hour-92.scripted.ndjson"),
        Part::File("streams/result-ok.scripted.ndjson"),
    ]);
    let markers = Markers::new();
    let (config, spec) = bind_probe(replay_config(&markers, &below), 30, wt.path(), "x");
    let h = harness(config);
    let mut s1 = VecStore::default();
    assert!(h.dispatch(&spec, &mut s1).result.is_ok());
    let mut s2 = VecStore::default();
    let o = h.dispatch(&spec, &mut s2);
    assert!(
        o.result.is_ok(),
        "one tick below the boundary proceeds: {:?}",
        o.result
    );
    assert_eq!(markers.count(), 2, "the second dispatch spawned");
    assert_eq!(
        o.record.headroom_before,
        Some(WindowReading {
            five_hour: 92,
            seven_day: 10
        })
    );
}

/// The e1-smoke recorded payload with `isUsingOverage` FLIPPED to `true` — *"the only
/// construction the record licenses"* (AC-221; [surface unverified]: no true-valued instance is
/// recorded).
fn flipped_overage_transcript() -> crate::support::Transcript {
    let recorded = e1_smoke_rate_limit_event(0);
    let text = String::from_utf8(recorded).expect("utf-8");
    assert!(
        text.contains("\"isUsingOverage\":false"),
        "the recorded value is false"
    );
    let mut line = text
        .replace("\"isUsingOverage\":false", "\"isUsingOverage\":true")
        .into_bytes();
    line.push(b'\n');
    transcript(&[
        Part::File("streams/init.scripted.ndjson"),
        Part::Bytes(line),
        Part::File("streams/result-ok.scripted.ndjson"),
    ])
}

/// AC-221 — PROVISIONAL, built and NOT claimed ([surface unverified]): *"a `rate_limit_event`
/// whose `rate_limit_info.isUsingOverage` is `true` / WHEN a dispatch is requested / THEN it is
/// refused as `StepError::Overage` before spawn"* — the recorded false-valued payload flipped;
/// red by a marker file appearing after the flipped reading.
///
/// RED (observed 2026-09-18) against the skeleton: `the first dispatch must proceed (it learns the
/// flipped reading): Err(Transport { kind: Spawn, … })`.
#[test]
fn ac221_overage_refuses_before_spawn() {
    let t = flipped_overage_transcript();
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(replay_config(&markers, &t), 30, wt.path(), "x");
    let h = harness(config);
    let mut first = VecStore::default();
    let o1 = h.dispatch(&spec, &mut first);
    assert!(
        o1.result.is_ok(),
        "the first dispatch must proceed (it learns the flipped reading): {:?}",
        o1.result
    );
    assert_eq!(markers.count(), 1);
    let mut store = VecStore::default();
    let o2 = h.dispatch(&spec, &mut store);
    assert!(
        matches!(o2.result, Err(StepError::Overage { .. })),
        "after the flipped reading the dispatch is refused Overage before spawn: {:?}",
        o2.result
    );
    assert_eq!(
        markers.count(),
        1,
        "no marker file appeared after the flipped reading (AC-221)"
    );
}

/// AC-378 — PROVISIONAL, built and NOT claimed: in exactly AC-376's shape — `dispatch` returns
/// `Err(Overage)`, the fake holds exactly one `dispatch_completed` row carrying `cli_version:
/// Pinned` and no reading beyond the flipped one, the spawn-marker count is 0 for that dispatch,
/// and **no `step_refused` row was written by the adapter** (the kernel's kind; the retry table
/// maps `Overage` to zero attempts in rk-types).
///
/// RED (observed 2026-09-18) against the skeleton: `the first dispatch must proceed: Err(Transport
/// { kind: Spawn, … })`.
#[test]
fn ac378_overage_zero_attempts_no_spawn() {
    let t = flipped_overage_transcript();
    let markers = Markers::new();
    let wt = tempfile::tempdir().expect("wt");
    let (config, spec) = bind_probe(replay_config(&markers, &t), 30, wt.path(), "x");
    let h = harness(config);
    let mut first = VecStore::default();
    assert!(
        h.dispatch(&spec, &mut first).result.is_ok(),
        "the first dispatch must proceed"
    );
    let before = markers.count();
    let mut store = VecStore::default();
    let o = h.dispatch(&spec, &mut store);
    let Err(err) = &o.result else {
        panic!("Overage expected, got Ok");
    };
    assert!(matches!(err, StepError::Overage { .. }), "{err:?}");
    assert_eq!(
        err.retry_class(),
        rk_types::error::RetryClass::Never,
        "zero attempts (rk-types)"
    );
    assert_eq!(
        markers.count(),
        before,
        "marker count unchanged: nothing spawned"
    );
    let rows = completed_rows(&store);
    assert_eq!(rows.len(), 1, "exactly one dispatch_completed row");
    assert_eq!(
        rows[0].cli_version,
        CliVersion::Pinned(TESTED_CLI_VERSION.to_owned())
    );
    assert_eq!(
        rows[0].headroom_before,
        Some(WindowReading {
            five_hour: 4,
            seven_day: 4
        }),
        "no reading beyond the flipped one"
    );
    assert_eq!(rows[0].headroom_after, None);
    assert!(
        !store
            .events()
            .iter()
            .any(|e| e.kind == EventKind::StepRefused),
        "no step_refused row written by the adapter — `StepError::Overage.event()` is the kernel's \
         arm (§7c.7)"
    );
    assert!(
        store
            .events()
            .iter()
            .all(|e| e.kind == EventKind::DispatchCompleted),
        "the refused dispatch journals only its dispatch_completed row — no rate_limit_reading on \
         a pre-spawn path (ADR-0163 clause 2): {:?}",
        store.events().iter().map(|e| e.kind).collect::<Vec<_>>()
    );
}

/// AC-372 (calculator half) = the stage's `tick_arithmetic_vectors`: the delta calculator in
/// ticks at 10/10 (flagged), 10/11 (flagged), 10/13 (not flagged), and at the decimal bases
/// 0.09/0.10, 0.10/0.12, 0.20/0.21 — **0.10/0.12 is the discriminator**: under `f64` the delta is
/// 0.01999999999999999, which `< 0.02` flags, while ticks read 2 and do not [measured
/// 2026-09-17]; 0.09/0.10 and 0.20/0.21 agree under both rules and are provenance vectors.
///
/// RED (observed 2026-09-18) against the skeleton (the runner's shape re-stated in ticks): `10/10
/// over 10 dispatches is resolution_limited (delta 0 <= 1): false`.
#[test]
fn tick_arithmetic_vectors() {
    let r = Timestamp::from_unix_seconds(1_788_370_800);
    let n = LOW_SAMPLE_BELOW;
    let d = batch_draw(Window::SevenDay, r, 10, 10, n);
    assert!(
        d.resolution_limited,
        "10/10 over {n} dispatches is resolution_limited (delta 0 <= 1): {}",
        d.resolution_limited
    );
    assert_eq!(d.delta_ticks, Some(0));
    let d = batch_draw(Window::SevenDay, r, 10, 11, n);
    assert!(d.resolution_limited, "10/11: delta 1 <= 1 is flagged");
    assert_eq!(d.delta_ticks, Some(1));
    let d = batch_draw(Window::SevenDay, r, 10, 13, n);
    assert!(!d.resolution_limited, "10/13: delta 3 is not flagged");
    assert_eq!(d.delta_ticks, Some(3));
    // the decimal bases, through the digit scanner
    let t = |s: &str| ticks_from_decimal("test", s.as_bytes()).expect("a two-decimal base");
    let d = batch_draw(Window::SevenDay, r, t("0.09"), t("0.10"), n);
    assert!(
        d.resolution_limited,
        "0.09/0.10: one tick, flagged under both rules"
    );
    let d = batch_draw(Window::SevenDay, r, t("0.10"), t("0.12"), n);
    assert!(
        !d.resolution_limited,
        "0.10/0.12: ticks read 2 and do NOT flag — an f64 subtraction (0.01999…) would (AC-372's \
         defect)"
    );
    assert_eq!(d.delta_ticks, Some(2));
    let d = batch_draw(Window::SevenDay, r, t("0.20"), t("0.21"), n);
    assert!(
        d.resolution_limited,
        "0.20/0.21: one tick, flagged under both rules"
    );
    assert!(!d.low_sample);
    assert_eq!(d.n, n);
}

/// AC-372: *"a batch that crosses a window reset is split per window and the partial window is
/// reported with its own `n`, as F29 did"* — a sample sequence over two `resetsAt` values yields
/// two batches, each with its own `n`, `first`/`last` and delta; red by one delta across the
/// reset.
///
/// RED (observed 2026-09-18) against the skeleton (no split): `two resetsAt values yield two
/// batches, each with its own n: 1`.
#[test]
fn ac372_reset_crossing_splits_per_window() {
    let r1 = Timestamp::from_unix_seconds(1_788_370_800);
    let r2 = Timestamp::from_unix_seconds(1_788_388_800);
    let mut samples: Vec<Sample> = (0..12u16)
        .map(|i| Sample {
            resets_at: r1,
            ticks: 40 + i,
        })
        .collect();
    samples.extend((0..11u16).map(|i| Sample {
        resets_at: r2,
        ticks: 2 + i,
    }));
    let draws = batch_draws_by_reset(Window::FiveHour, &samples);
    assert_eq!(
        draws.len(),
        2,
        "two resetsAt values yield two batches, each with its own n: {}",
        draws.len()
    );
    assert_eq!(
        (
            draws[0].resets_at,
            draws[0].n,
            draws[0].first,
            draws[0].last
        ),
        (r1, 12, 40, 51)
    );
    assert_eq!(draws[0].delta_ticks, Some(11));
    assert_eq!(
        (
            draws[1].resets_at,
            draws[1].n,
            draws[1].first,
            draws[1].last
        ),
        (r2, 11, 2, 12)
    );
    assert_eq!(draws[1].delta_ticks, Some(10));
    assert!(!draws[0].non_monotonic && !draws[1].non_monotonic);
}

/// AC-372: *"a set with `n < 10` is reported `low_sample` and no delta is quoted"* — an `n = 9`
/// fixture; red by a delta quoted at n = 9.
///
/// RED (observed 2026-09-18) against the skeleton: `n = 9 is low_sample: false`.
#[test]
fn ac372_low_sample_at_n9() {
    let r = Timestamp::from_unix_seconds(1_788_370_800);
    let d = batch_draw(Window::SevenDay, r, 10, 20, 9);
    assert!(d.low_sample, "n = 9 is low_sample: {}", d.low_sample);
    assert_eq!(d.delta_ticks, None, "no delta is quoted at n = 9");
    let d = batch_draw(Window::SevenDay, r, 10, 20, 10);
    assert!(!d.low_sample);
    assert_eq!(d.delta_ticks, Some(10), "at n = 10 the delta is quoted");
    // through the splitter: nine samples in one window is one low_sample batch
    let samples: Vec<Sample> = (0..9u16)
        .map(|i| Sample {
            resets_at: r,
            ticks: 10 + i,
        })
        .collect();
    let draws = batch_draws_by_reset(Window::SevenDay, &samples);
    assert_eq!(draws.len(), 1);
    assert!(draws[0].low_sample && draws[0].delta_ticks.is_none());
}

/// AC-372: *"prints a per-item delta only when the item's dispatches were the only traffic in
/// the set, and otherwise prints `not attributable` with the interleaving count"* — an
/// interleaved-traffic fixture; red by a per-item delta under interleaving.
///
/// RED (observed 2026-09-18) against the skeleton (interleaving not observed): `three
/// interleaving dispatches make the set not attributable: Attributable([…])`.
#[test]
fn ac372_interleaved_not_attributable() {
    let r = Timestamp::from_unix_seconds(1_788_370_800);
    let samples: Vec<Sample> = (0..12u16)
        .map(|i| Sample {
            resets_at: r,
            ticks: 10 + i,
        })
        .collect();
    let mut owned = vec![true; 12];
    owned[3] = false;
    owned[7] = false;
    owned[8] = false;
    let a = attribute(Window::SevenDay, &samples, &owned);
    assert_eq!(
        a,
        Attribution::NotAttributable { interleaving: 3 },
        "three interleaving dispatches make the set not attributable: {a:?}"
    );
    let a = attribute(Window::SevenDay, &samples, &[true; 12]);
    match a {
        Attribution::Attributable(draws) => {
            assert_eq!(draws.len(), 1);
            assert_eq!(draws[0].delta_ticks, Some(11));
        }
        other => panic!("the item's own traffic only: attributable, got {other:?}"),
    }
}

/// AC-381's positive half: the headroom calculator still produces a batch delta from the two
/// window readings alone — so the criterion is not satisfied by removing the calculator. The
/// compile-fail half is `tests/ui/ac381_cache_field_on_headroom_input.rs`.
///
/// RED (observed 2026-09-18) against the skeleton: passes vacuously against the skeleton's own
/// subtraction — the observed red is the trybuild half's (`.stderr` absent) and this half's red is
/// by removing the calculator (E0425), which cannot be observed on a compiling tree.
#[test]
fn ac381_headroom_calculator_still_delivers_a_delta() {
    let r = Timestamp::from_unix_seconds(1_788_370_800);
    let five: Vec<Sample> = (0..10u16)
        .map(|i| Sample {
            resets_at: r,
            ticks: 4 + i,
        })
        .collect();
    let seven: Vec<Sample> = (0..10u16)
        .map(|i| Sample {
            resets_at: r,
            ticks: 4 + i / 2,
        })
        .collect();
    let d5 = batch_draws_by_reset(Window::FiveHour, &five);
    let d7 = batch_draws_by_reset(Window::SevenDay, &seven);
    assert_eq!(
        d5[0].delta_ticks,
        Some(9),
        "a delta from the five_hour readings alone"
    );
    assert_eq!(
        d7[0].delta_ticks,
        Some(4),
        "a delta from the seven_day readings alone"
    );
    // the input names exactly what the calculator needs, and nothing cache-shaped
    let Sample { resets_at, ticks } = five[0];
    assert_eq!((resets_at, ticks), (r, 4));
    // the ui case and its .stderr exist (asserted by path from the test that carries the
    // criterion, the kernel's precedent)
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    for f in [
        "ac381_cache_field_on_headroom_input.rs",
        "ac381_cache_field_on_headroom_input.stderr",
    ] {
        let p = ui.join(f);
        assert!(
            p.is_file() && std::fs::metadata(&p).expect("meta").len() > 0,
            "{f} is missing or empty"
        );
    }
}
