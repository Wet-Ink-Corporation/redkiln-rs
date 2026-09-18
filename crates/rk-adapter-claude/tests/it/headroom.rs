//! **The digit scanner** — AC-374, AC-375, AC-380 (s7c §7c.8, §7c.18; ADR-0110; RT-2, RT-19).
//! The scanner is a structural, escape-aware walk over the line's bytes; the vectors below are the
//! register's, labelled for what they are (§7c.17).

use rk_types::usage::{CostUsd, WindowReading};

use rk_adapter_claude::config::Window;
use rk_adapter_claude::headroom::{
    HeadroomError, Shape, cost_usd_from_result, read_rate_limit_event, render_ticks,
    ticks_from_decimal, walk,
};

use crate::support::{e1_smoke_rate_limit_event, fixture};

/// AC-374: *"parsed from the JSON decimal digits into an integer tick count (hundredths, u16) …
/// at the resolution reported … no interpolation, smoothing, rescaling or extrapolation"* — the
/// round-trip test is two-part (§7c.8): (1) the RECORDED e1-smoke line (`utilization: 0.04` on
/// both windows) for provenance — which cannot discriminate an `f64` path; (2) the SYNTHETIC line
/// (`0.10`, `0.20`, `0.09`, `0.29`, `0.57`, `0.58` — trailing-zero digits are unrecorded at the
/// pin) whose stored ticks (10, 20, 9, 29, 57, 58) must re-render to the source bytes — red by a
/// TRUNCATING `f64` path (0.29 → 28, 0.57 → 56, 0.58 → 57); the rounding path is refused by the
/// block's lint, not by this vector. Plus the scanner's own reds: `"utilization"` inside a string
/// value is never matched, and a third decimal is refused, never rounded.
///
/// RED (observed 2026-09-18) against the skeleton (a first-fractional-digit read): `the recorded
/// e1-smoke reading is 4/4 ticks: Err(NoUtilization)`.
#[test]
fn ac374_ticks_roundtrip_decimal_digits() {
    // (1) provenance: the recorded line
    let recorded = e1_smoke_rate_limit_event(0);
    let r = read_rate_limit_event(&recorded);
    let r = r.unwrap_or_else(|e| panic!("the recorded e1-smoke reading is 4/4 ticks: Err({e:?})"));
    assert_eq!(
        r.windows(),
        Some(WindowReading {
            five_hour: 4,
            seven_day: 4
        })
    );
    assert_eq!(render_ticks(4), "0.04");
    let src = walk(
        &recorded,
        &[
            "rate_limit_info",
            "unifiedWindows",
            "five_hour",
            "utilization",
        ],
    )
    .expect("the span");
    assert_eq!(src, b"0.04", "the recorded digits");
    // (2) the synthetic line: ticks re-render to the SOURCE bytes
    let synthetic = fixture("streams/rate-limit-synthetic-digits.synthetic.ndjson");
    let want = [(10u16, 20u16), (9, 29), (57, 58)];
    for (line, (five, seven)) in synthetic.lines().zip(want) {
        let r = read_rate_limit_event(line.as_bytes()).expect("parses");
        let w = r.windows().expect("unified");
        assert_eq!((w.five_hour, w.seven_day), (five, seven), "{line}");
        for (window, ticks) in [(Window::FiveHour, five), (Window::SevenDay, seven)] {
            let src = walk(
                line.as_bytes(),
                &[
                    "rate_limit_info",
                    "unifiedWindows",
                    window.as_str(),
                    "utilization",
                ],
            )
            .expect("span");
            assert_eq!(
                render_ticks(ticks).as_bytes(),
                src,
                "tick {ticks} re-renders to the source digits byte-for-byte ({})",
                String::from_utf8_lossy(src)
            );
        }
    }
    // the scanner's own red: a "utilization" inside a STRING value is never matched
    let in_string = fixture("streams/rate-limit-utilization-in-string.synthetic.ndjson");
    let r = read_rate_limit_event(in_string.trim_end().as_bytes()).expect("parses");
    assert_eq!(
        r.windows(),
        Some(WindowReading {
            five_hour: 4,
            seven_day: 4
        }),
        "a substring scan would have read 0.99 out of the string value"
    );
    // a third decimal is refused, never rounded
    let three = fixture("streams/rate-limit-three-decimals.synthetic.ndjson");
    assert!(
        matches!(
            read_rate_limit_event(three.trim_end().as_bytes()),
            Err(HeadroomError::TooManyDecimals { .. })
        ),
        "0.045 is a resolution the instrument does not represent (AC-374)"
    );
    // the digit reader's own vectors
    assert_eq!(ticks_from_decimal("t", b"0.04"), Ok(4));
    assert_eq!(ticks_from_decimal("t", b"0.1"), Ok(10));
    assert_eq!(ticks_from_decimal("t", b"0.10"), Ok(10));
    assert_eq!(ticks_from_decimal("t", b"1"), Ok(100));
    assert_eq!(ticks_from_decimal("t", b"0.29"), Ok(29));
    assert_eq!(ticks_from_decimal("t", b"0.57"), Ok(57));
    assert_eq!(ticks_from_decimal("t", b"0.58"), Ok(58));
    assert!(matches!(
        ticks_from_decimal("t", b"1e-2"),
        Err(HeadroomError::NotDecimal { .. })
    ));
    assert!(matches!(
        ticks_from_decimal("t", b".5"),
        Err(HeadroomError::NotDecimal { .. })
    ));
    assert!(matches!(
        ticks_from_decimal("t", b"-0.5"),
        Err(HeadroomError::NotDecimal { .. })
    ));
    assert!(matches!(
        ticks_from_decimal("t", b"0.045"),
        Err(HeadroomError::TooManyDecimals { .. })
    ));
    assert!(matches!(
        ticks_from_decimal("t", b"700"),
        Err(HeadroomError::Overflow { .. })
    ));
    // the cost scanner: the digits verbatim (RT-19), never a scale
    let f20 = fixture("recorded/f20.result_raw.jsonl");
    assert_eq!(
        cost_usd_from_result(f20.trim_end().as_bytes()),
        Some(CostUsd("2.337147".to_owned()))
    );
    assert_eq!(
        cost_usd_from_result(br#"{"type":"result","total_cost_usd":0.0000001234567}"#),
        Some(CostUsd("0.0000001234567".to_owned()))
    );
    assert_eq!(cost_usd_from_result(br#"{"type":"result"}"#), None);
}

/// AC-375: *"consults only `unifiedWindows.five_hour` and `unifiedWindows.seven_day` AND no code
/// path reads, derives, estimates or synthesises a per-model-family headroom value"* — the
/// throttle input struct has EXACTLY two window fields (an exhaustive destructure that fails to
/// compile on a third), and a reading carrying `seven_day_opus` / `seven_day_sonnet` beside them
/// is tolerated and ignored, never named: the two windows read 4/4 and the family values (0.55,
/// 0.66) reach nothing.
///
/// RED (observed 2026-09-18) against the skeleton: `the family-keyed line parses to the two
/// windows only: Err(NoUtilization)`.
#[test]
fn ac375_throttle_input_has_two_windows_only() {
    let line = fixture("streams/rate-limit-family-keys.synthetic.ndjson");
    let r = read_rate_limit_event(line.trim_end().as_bytes()).unwrap_or_else(|e| {
        panic!("the family-keyed line parses to the two windows only: Err({e:?})")
    });
    let w = r.windows().expect("unified");
    // exactly two fields: a third member here is a compile error
    let WindowReading {
        five_hour,
        seven_day,
    } = w;
    assert_eq!((five_hour, seven_day), (4, 4));
    // the family values reach no reading
    assert_ne!(five_hour, 55);
    assert_ne!(seven_day, 66);
    assert_eq!(r.ticks(Window::FiveHour), Some(4));
    assert_eq!(r.ticks(Window::SevenDay), Some(4));
    // and the record's type is rk-types' two-field struct, sizes agree
    assert_eq!(
        std::mem::size_of::<WindowReading>(),
        2 * std::mem::size_of::<u16>()
    );
}

/// AC-380: *"headroom is read from the top-level `rate_limit_event` … and `result.rate_limit` is
/// never read"* — both shapes parse with a named error on neither: the RECORDED `unifiedWindows`
/// arm (e1-smoke) and the flat SDK `rate_limit_info { status, resetsAt, utilization }` arm —
/// SYNTHETIC, unrecorded at the pin, its key names an assumption, [surface unverified],
/// PROVISIONAL (§7c.8). A reading with neither shape is the named `NoUtilization`.
///
/// RED (observed 2026-09-18) against the skeleton: `the recorded unifiedWindows shape parses:
/// Err(NoUtilization)`.
#[test]
fn ac380_rate_limit_event_sole_source_both_shapes() {
    let recorded = e1_smoke_rate_limit_event(1);
    let r = read_rate_limit_event(&recorded)
        .unwrap_or_else(|e| panic!("the recorded unifiedWindows shape parses: Err({e:?})"));
    assert!(matches!(r.shape, Shape::Unified { .. }));
    assert_eq!(r.status.as_deref(), Some("allowed"));
    assert!(r.is_allowed());
    assert_eq!(r.is_using_overage, Some(false));
    assert_eq!(r.rate_limit_type.as_deref(), Some("five_hour"));
    assert_eq!(
        r.resets_at.map(rk_types::time::Timestamp::unix_seconds),
        Some(1_788_370_800)
    );
    assert_eq!(
        r.resets_at(Window::SevenDay)
            .map(rk_types::time::Timestamp::unix_seconds),
        Some(1_788_408_000)
    );
    // the flat SDK shape — SYNTHETIC, [surface unverified]
    let flat = fixture("streams/rate-limit-flat-sdk.synthetic.ndjson");
    let r = read_rate_limit_event(flat.trim_end().as_bytes()).unwrap_or_else(|e| {
        panic!("the flat SDK shape parses with a named error on neither: Err({e:?})")
    });
    assert_eq!(r.shape, Shape::Flat { utilization: 37 });
    assert_eq!(
        r.windows(),
        None,
        "one utilisation is not extrapolated to two windows (AC-374)"
    );
    assert_eq!(
        r.ticks(Window::FiveHour),
        Some(37),
        "attributed to the window rateLimitType names"
    );
    assert_eq!(r.ticks(Window::SevenDay), None);
    // neither shape: a NAMED error, never a silent zero
    assert_eq!(
        read_rate_limit_event(
            br#"{"type":"rate_limit_event","rate_limit_info":{"status":"allowed"}}"#
        ),
        Err(HeadroomError::NoUtilization)
    );
    // one window of two: named too
    assert!(matches!(
        read_rate_limit_event(
            br#"{"rate_limit_info":{"unifiedWindows":{"five_hour":{"utilization":0.04}}}}"#
        ),
        Err(HeadroomError::OneWindow { .. })
    ));
}
