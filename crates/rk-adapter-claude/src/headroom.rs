//! Headroom in `u16` ticks — the structural, escape-aware digit scanner over `StreamLine.raw`.
//!
//! The `rate_limit_event` reading in both recorded shapes, and the cost scanner beside it
//! (`s7c` §7c.8; ADR-0110; RT-2, RT-19; AC-374, AC-375, AC-380, AC-219, AC-221).
//!
//! **Not lifted.** The runner's `headroom_from_event` (`runner/src/event.rs` at the pin) read
//! `utilization` through `serde_json::Value::as_f64` — `0.10` became `0.1`, `0.29` became
//! `28` under a truncating cast, and the digits the CLI printed were gone. ADR-0110 decision 2 and
//! AC-374 say *"verbatim"* means **the decimal digits**, parsed into an integer tick count
//! (hundredths, `u16`), *"at the resolution reported … no interpolation, smoothing, rescaling or
//! extrapolation"*. `serde_json` without `arbitrary_precision` hands a visitor an `f64` (RT-2's
//! ground), so the reading is taken **from the retained line bytes** by this module's own scanner
//! and the block's AC-374 ban forbids every float spelling over this module, `throttle` and
//! `event`. The named alternative — `serde_json`'s `raw_value` feature — is recorded in RT-2 and
//! not taken (a review may not re-route the operator's routing).
//!
//! **The scanner's contract** (§7c.8, stated so it is implementable and not a substring search):
//!
//! - It is a **structural** scan over the line's bytes: [`walk`] tokenises JSON objects member by
//!   member with an escape-aware string skipper ([`skip_string`]), so a `"utilization"` **inside a
//!   string value** is never matched — a `// SYNTHETIC` fixture carrying one beside the two real
//!   keys is the scanner's own red (`ac374_…`).
//! - It locates `utilization` **by key path**, separately under
//!   `rate_limit_info.unifiedWindows.five_hour` and `….seven_day` (two identical keys under two
//!   parents — [`UNIFIED_PATH`]), and tolerates the SDK `rate_limit_info.utilization` placement
//!   ([`FLAT_PATH`]; AC-380's flat shape, **`// SYNTHETIC` — unrecorded at the pin, its key names
//!   an assumption, [surface unverified]**).
//! - At each site it reads the decimal digits after the point into ticks ([`ticks_from_decimal`]:
//!   `0.04` → 4, `0.1` → 10, `1` → 100); **more than two decimals is a refusal**
//!   ([`HeadroomError::TooManyDecimals`]), never a rounding — AC-374's *"at the resolution
//!   reported"* is 0.01 at every measurement (F6), and a third digit is a resolution this
//!   instrument does not represent. The runner's drain turns that refusal into
//!   `Transport { kind: Decode }` (fail closed: a reading the throttle cannot represent is not a
//!   reading it may proceed blind on).
//! - **`seven_day_opus` / `seven_day_sonnet` are tolerated and ignored, never named** (AC-375):
//!   the walker skips any member it is not looking for, and no identifier in this crate spells a
//!   family key (the block's ban).
//! - The **cost scanner** ([`cost_usd_from_result`]) reads the result frame's `total_cost_usd`
//!   digits **verbatim into `CostUsd(String)`** (RT-19 as amended) — no scale, no truncation,
//!   whatever digit count the CLI prints; it lives here beside the digit scanner it shares so the
//!   AC-374 ban's module filter reaches it.
//!
//! **What a reading is** ([`RateLimitReading`]): the event's `rate_limit_info` as the two
//! shapes carry it — [`Shape::Unified`] with a [`WindowReading`] of **exactly two windows**
//! (AC-375; `rk-types`' type) and each window's `resetsAt`, or [`Shape::Flat`] with the one
//! utilisation the SDK shape carries — plus the top-level `status`, `resetsAt`, `rateLimitType`,
//! `isUsingOverage` and `overageStatus` (e1-smoke's recorded keys, §7c.17). **A reading with
//! neither shape is [`HeadroomError::NoUtilization`]** — a named error, so an event that changed
//! shape is a finding the drain can name rather than a silent zero.
//!
//! **What this module never reads**: `result.rate_limit` (AC-219/AC-380 — the runner's
//! `rate_limit_from_result` is not lifted and the block bans the token), a cache metric (AC-381 —
//! the calculator's input struct in `throttle.rs` cannot name one; the trybuild case
//! `tests/ui/ac381_cache_field_on_headroom_input.rs` is the type-level instrument), or a family
//! key (AC-375).

use rk_types::time::Timestamp;
use rk_types::usage::{CostUsd, WindowReading};

use crate::config::Window;

/// The key path of the recorded shape's `utilization`, per window (e1-smoke's rows at the pin,
/// §7c.17): `rate_limit_info.unifiedWindows.<window>.utilization`.
pub const UNIFIED_PATH: [&str; 2] = ["rate_limit_info", "unifiedWindows"];

/// The key path of the SDK flat shape's `utilization`: `rate_limit_info.utilization`.
///
/// **SYNTHETIC** — the SDK `rate_limit_info { status, resetsAt, utilization }` shape is unrecorded
/// at the pin; its key names are an assumption, [surface unverified] (AC-380, §7c.8).
pub const FLAT_PATH: [&str; 2] = ["rate_limit_info", "utilization"];

/// Why a `rate_limit_event` line yielded no reading.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum HeadroomError {
    /// Neither `unifiedWindows.{five_hour,seven_day}.utilization` nor the flat
    /// `rate_limit_info.utilization` was found — the event carries no utilisation this adapter
    /// knows how to read.
    #[error(
        "the rate_limit_event carries no utilization under unifiedWindows.five_hour / \
         unifiedWindows.seven_day (the recorded shape) or rate_limit_info.utilization (the SDK \
         shape) — a shape this adapter cannot read is named, never read as zero (AC-380)"
    )]
    NoUtilization,
    /// The recorded shape carries one window and not the other — a reading of exactly two windows
    /// cannot be formed (AC-375) and a half-reading is not extrapolated (AC-374).
    #[error(
        "unifiedWindows carries {present} but not {absent}: a reading is exactly two windows \
         (AC-375) and the missing one is never extrapolated (AC-374)"
    )]
    OneWindow {
        present: &'static str,
        absent: &'static str,
    },
    /// The value under a `utilization` key is not a JSON decimal of the accepted shape.
    #[error(
        "{key_path}: {text:?} is not a decimal utilization (digits, optionally a point and digits; \
         no sign, exponent or fraction-only form) — refused, never read as a number"
    )]
    NotDecimal { key_path: String, text: String },
    /// More than two fractional digits — a resolution the tick instrument does not represent
    /// (AC-374: never a rounding).
    #[error(
        "{key_path}: {text:?} carries more than two decimals — the instrument's resolution is 0.01 \
         (F6) and a finer reading is refused, never rounded (AC-374)"
    )]
    TooManyDecimals { key_path: String, text: String },
    /// The ticks do not fit a `u16` (a utilisation over 655.35 is not a utilisation).
    #[error("{key_path}: {text:?} does not fit u16 ticks")]
    Overflow { key_path: String, text: String },
}

/// One `rate_limit_event`, read from the retained line bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RateLimitReading {
    /// Which shape carried the utilisation, and the ticks.
    pub shape: Shape,
    /// `rate_limit_info.status` — `allowed` on every recorded reading; a non-`allowed` value is
    /// what `RateLimited { resets_at }` reads its `resets_at` from (AC-523; the per-window
    /// `status` the register names is not in the recorded shape, §7c.7).
    pub status: Option<String>,
    /// `rate_limit_info.resetsAt`, the top-level one.
    pub resets_at: Option<Timestamp>,
    /// `rate_limit_info.rateLimitType` — the platform's own view of the binding window.
    pub rate_limit_type: Option<String>,
    /// `rate_limit_info.isUsingOverage` (AC-221: `true` refuses the next dispatch before spawn).
    pub is_using_overage: Option<bool>,
    /// `rate_limit_info.overageStatus`.
    pub overage_status: Option<String>,
}

/// The two shapes a reading arrives in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    /// The recorded shape: `unifiedWindows.{five_hour,seven_day}` — exactly two windows.
    Unified {
        windows: WindowReading,
        five_hour_resets_at: Option<Timestamp>,
        seven_day_resets_at: Option<Timestamp>,
    },
    /// The SDK flat shape (**SYNTHETIC**, [surface unverified]): one utilisation, attributed to
    /// the window `rateLimitType` names when it names one of the two.
    Flat { utilization: u16 },
}

impl RateLimitReading {
    /// The two-window reading, when the event carried the recorded shape. The flat shape yields
    /// `None`: one utilisation is not extrapolated to two windows (AC-374's *"no extrapolation"*).
    #[must_use]
    pub const fn windows(&self) -> Option<WindowReading> {
        match &self.shape {
            Shape::Unified { windows, .. } => Some(WindowReading {
                five_hour: windows.five_hour,
                seven_day: windows.seven_day,
            }),
            Shape::Flat { .. } => None,
        }
    }

    /// The ticks on `window` — the unified reading's own, or the flat reading's one utilisation
    /// when `rateLimitType` names that window (or names none — a flat reading with no window
    /// named counts against BOTH windows, the conservative reading; never a synthesised value
    /// for the other window when one IS named).
    #[must_use]
    pub fn ticks(&self, window: Window) -> Option<u16> {
        match &self.shape {
            Shape::Unified { windows, .. } => Some(match window {
                Window::FiveHour => windows.five_hour,
                Window::SevenDay => windows.seven_day,
            }),
            Shape::Flat { utilization } => match self.rate_limit_type.as_deref() {
                None => Some(*utilization),
                Some(named) if named == window.as_str() => Some(*utilization),
                Some(_) => None,
            },
        }
    }

    /// The `resetsAt` for `window`: the window's own under the unified shape, else the top-level.
    #[must_use]
    pub const fn resets_at(&self, window: Window) -> Option<Timestamp> {
        match &self.shape {
            Shape::Unified {
                five_hour_resets_at,
                seven_day_resets_at,
                ..
            } => match window {
                Window::FiveHour => *five_hour_resets_at,
                Window::SevenDay => *seven_day_resets_at,
            },
            Shape::Flat { .. } => self.resets_at,
        }
    }

    /// Is the top-level `status` `allowed`? `false` for any other value AND for an absent status —
    /// an absent status is not evidence of allowance (Halt 11's shape, read the safe way).
    #[must_use]
    pub fn is_allowed(&self) -> bool {
        self.status.as_deref() == Some("allowed")
    }
}

// ---------------------------------------------------------------- the structural walker

/// Index just past the whitespace at `i`.
const fn skip_ws(b: &[u8], mut i: usize) -> usize {
    while i < b.len() && matches!(b[i], b' ' | b'\t' | b'\n' | b'\r') {
        i += 1;
    }
    i
}

/// `i` at an opening quote: the index just past the closing quote, escape-aware (`\"` and `\\`
/// never end the string). `None` if the string is unterminated.
fn skip_string(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i) != Some(&b'"') {
        return None;
    }
    let mut j = i + 1;
    while j < b.len() {
        match b[j] {
            b'\\' => j += 2,
            b'"' => return Some(j + 1),
            _ => j += 1,
        }
    }
    None
}

/// `i` at the first byte of a value: the index just past it. Strings by [`skip_string`], objects
/// and arrays by depth counting that skips strings, scalars up to the next delimiter.
fn skip_value(b: &[u8], i: usize) -> Option<usize> {
    match *b.get(i)? {
        b'"' => skip_string(b, i),
        open @ (b'{' | b'[') => {
            let close = if open == b'{' { b'}' } else { b']' };
            let mut depth = 0usize;
            let mut j = i;
            while j < b.len() {
                match b[j] {
                    b'"' => j = skip_string(b, j)?,
                    b'{' | b'[' => {
                        depth += 1;
                        j += 1;
                    }
                    b'}' | b']' => {
                        depth -= 1;
                        if depth == 0 {
                            return (b[j] == close).then_some(j + 1);
                        }
                        j += 1;
                    }
                    _ => j += 1,
                }
            }
            None
        }
        _ => {
            let mut j = i;
            while j < b.len() && !matches!(b[j], b',' | b'}' | b']' | b' ' | b'\t' | b'\n' | b'\r')
            {
                j += 1;
            }
            (j > i).then_some(j)
        }
    }
}

/// `i` at an object's `{`: the byte span of the value under member `key` (the raw key bytes
/// compared, so a key with an escape never matches), or `None` if the object has no such member.
fn member(b: &[u8], i: usize, key: &str) -> Option<(usize, usize)> {
    if b.get(i) != Some(&b'{') {
        return None;
    }
    let mut j = skip_ws(b, i + 1);
    loop {
        if b.get(j) == Some(&b'}') {
            return None;
        }
        let key_end = skip_string(b, j)?;
        let this_key = &b[j + 1..key_end - 1];
        j = skip_ws(b, key_end);
        if b.get(j) != Some(&b':') {
            return None;
        }
        j = skip_ws(b, j + 1);
        let value_end = skip_value(b, j)?;
        if this_key == key.as_bytes() {
            return Some((j, value_end));
        }
        j = skip_ws(b, value_end);
        // a comma continues; a `}` ends the object without the member; anything else is malformed
        if b.get(j) != Some(&b',') {
            return None;
        }
        j = skip_ws(b, j + 1);
    }
}

/// The byte span of the value at `path` from the top-level object of `line`.
#[must_use]
pub fn walk<'a>(line: &'a [u8], path: &[&str]) -> Option<&'a [u8]> {
    let mut start = skip_ws(line, 0);
    let mut end = skip_value(line, start)?;
    for key in path {
        let (s, e) = member(line, start, key)?;
        start = s;
        end = e;
    }
    Some(&line[start..end])
}

/// The text of a JSON string value's span, unescaped only for the plain `\"` / `\\` forms (the
/// values this module reads — `status`, `rateLimitType`, `overageStatus` — carry no escapes).
fn string_of(span: &[u8]) -> Option<String> {
    if span.len() < 2 || span[0] != b'"' || span[span.len() - 1] != b'"' {
        return None;
    }
    let inner = std::str::from_utf8(&span[1..span.len() - 1]).ok()?;
    Some(inner.replace("\\\"", "\"").replace("\\\\", "\\"))
}

/// A JSON integer span as `i64`.
fn integer_of(span: &[u8]) -> Option<i64> {
    std::str::from_utf8(span).ok()?.parse().ok()
}

const fn bool_of(span: &[u8]) -> Option<bool> {
    match span {
        b"true" => Some(true),
        b"false" => Some(false),
        _ => None,
    }
}

// ---------------------------------------------------------------- the digit scanner

/// The decimal digits of a JSON number into ticks (hundredths): `0.04` → 4, `0.1` → 10, `0.10` →
/// 10, `1` → 100. Digits, optionally a point and one or two digits, nothing else.
///
/// # Errors
/// [`HeadroomError::NotDecimal`] for a sign, an exponent, a fraction with no integer part or a
/// trailing point; [`HeadroomError::TooManyDecimals`] for a third fractional digit (never a
/// rounding — AC-374); [`HeadroomError::Overflow`] when the ticks do not fit a `u16`.
pub fn ticks_from_decimal(key_path: &str, text: &[u8]) -> Result<u16, HeadroomError> {
    let shown = || String::from_utf8_lossy(text).into_owned();
    let not_decimal = || HeadroomError::NotDecimal {
        key_path: key_path.to_owned(),
        text: shown(),
    };
    let (int, frac) = text
        .iter()
        .position(|&c| c == b'.')
        .map_or((text, None), |p| (&text[..p], Some(&text[p + 1..])));
    let all_digits = |s: &[u8]| !s.is_empty() && s.iter().all(u8::is_ascii_digit);
    if !all_digits(int) || frac.is_some_and(|f| !all_digits(f)) {
        return Err(not_decimal());
    }
    let frac = frac.unwrap_or(b"");
    if frac.len() > 2 {
        return Err(HeadroomError::TooManyDecimals {
            key_path: key_path.to_owned(),
            text: shown(),
        });
    }
    let overflow = || HeadroomError::Overflow {
        key_path: key_path.to_owned(),
        text: shown(),
    };
    let whole: u32 = std::str::from_utf8(int)
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or_else(overflow)?;
    let digit = |i: usize| frac.get(i).map_or(0, |d| u32::from(d - b'0'));
    let ticks = whole
        .checked_mul(100)
        .and_then(|t| t.checked_add(digit(0) * 10 + digit(1)))
        .ok_or_else(overflow)?;
    u16::try_from(ticks).map_err(|_| overflow())
}

/// Ticks back to the two-decimal text the instrument reports: 4 → `0.04`, 10 → `0.10`, 100 →
/// `1.00`.
///
/// The AC-374 round-trip: a reading stored as ticks re-renders to its source digits
/// byte-for-byte for every two-decimal source, which is every recorded one.
#[must_use]
pub fn render_ticks(ticks: u16) -> String {
    format!("{}.{:02}", ticks / 100, ticks % 100)
}

/// Read one `rate_limit_event` line into a reading.
///
/// # Errors
/// [`HeadroomError`] — neither shape found, one window of two, or a value under a
/// `utilization` key that is not a two-decimal-or-fewer decimal.
pub fn read_rate_limit_event(raw: &[u8]) -> Result<RateLimitReading, HeadroomError> {
    let info_path = [UNIFIED_PATH[0]];
    let s = |key: &str| walk(raw, &[info_path[0], key]).and_then(string_of);
    let status = s("status");
    let rate_limit_type = s("rateLimitType");
    let overage_status = s("overageStatus");
    let resets_at = walk(raw, &[info_path[0], "resetsAt"])
        .and_then(integer_of)
        .map(Timestamp::from_unix_seconds);
    let is_using_overage = walk(raw, &[info_path[0], "isUsingOverage"]).and_then(bool_of);

    let window_ticks = |window: Window| -> Result<Option<(u16, Option<Timestamp>)>, HeadroomError> {
        let key_path = format!(
            "{}.{}.{}.utilization",
            UNIFIED_PATH[0],
            UNIFIED_PATH[1],
            window.as_str()
        );
        let Some(span) = walk(
            raw,
            &[
                UNIFIED_PATH[0],
                UNIFIED_PATH[1],
                window.as_str(),
                "utilization",
            ],
        ) else {
            return Ok(None);
        };
        let ticks = ticks_from_decimal(&key_path, span)?;
        let resets = walk(
            raw,
            &[
                UNIFIED_PATH[0],
                UNIFIED_PATH[1],
                window.as_str(),
                "resetsAt",
            ],
        )
        .and_then(integer_of)
        .map(Timestamp::from_unix_seconds);
        Ok(Some((ticks, resets)))
    };
    let five = window_ticks(Window::FiveHour)?;
    let seven = window_ticks(Window::SevenDay)?;
    let shape = match (five, seven) {
        (Some((five_hour, five_hour_resets_at)), Some((seven_day, seven_day_resets_at))) => {
            Shape::Unified {
                windows: WindowReading {
                    five_hour,
                    seven_day,
                },
                five_hour_resets_at,
                seven_day_resets_at,
            }
        }
        (Some(_), None) => {
            return Err(HeadroomError::OneWindow {
                present: Window::FiveHour.as_str(),
                absent: Window::SevenDay.as_str(),
            });
        }
        (None, Some(_)) => {
            return Err(HeadroomError::OneWindow {
                present: Window::SevenDay.as_str(),
                absent: Window::FiveHour.as_str(),
            });
        }
        (None, None) => {
            // SYNTHETIC shape (AC-380's flat arm; [surface unverified]).
            let Some(span) = walk(raw, &FLAT_PATH) else {
                return Err(HeadroomError::NoUtilization);
            };
            let key_path = format!("{}.{}", FLAT_PATH[0], FLAT_PATH[1]);
            Shape::Flat {
                utilization: ticks_from_decimal(&key_path, span)?,
            }
        }
    };
    Ok(RateLimitReading {
        shape,
        status,
        resets_at,
        rate_limit_type,
        is_using_overage,
        overage_status,
    })
}

// ---------------------------------------------------------------- the result frame's scanners

/// The byte span of a member of the result payload — the top level, or the nested `result`
/// object when the frame nests its payload (the documented shape; `event::payload`'s rule).
fn result_member<'a>(raw: &'a [u8], key: &str) -> Option<&'a [u8]> {
    if let Some(inner) = walk(raw, &["result"])
        && inner.first() == Some(&b'{')
        && let Some(span) = walk(raw, &["result", key])
    {
        return Some(span);
    }
    walk(raw, &[key])
}

/// `total_cost_usd` as the decimal digits verbatim (RT-19 as amended): `2.337147` stays
/// `"2.337147"`, `0` stays `"0"`, whatever the CLI printed. `None` when absent or not a number.
#[must_use]
pub fn cost_usd_from_result(raw: &[u8]) -> Option<CostUsd> {
    let span = result_member(raw, "total_cost_usd")?;
    let text = std::str::from_utf8(span).ok()?;
    let mut chars = text.bytes();
    let first = chars.next()?;
    (first == b'-' || first.is_ascii_digit())
        .then(|| CostUsd(text.to_owned()))
        .filter(|_| {
            text.bytes()
                .all(|c| c.is_ascii_digit() || matches!(c, b'-' | b'+' | b'.' | b'e' | b'E'))
        })
}

/// `duration_api_ms` from the result frame, as the harness reported it (`Timings.api_ms`;
/// AC-362's rule: absent is `None`).
#[must_use]
pub fn api_ms_from_result(raw: &[u8]) -> Option<u64> {
    let span = result_member(raw, "duration_api_ms")?;
    integer_of(span).and_then(|v| u64::try_from(v).ok())
}
