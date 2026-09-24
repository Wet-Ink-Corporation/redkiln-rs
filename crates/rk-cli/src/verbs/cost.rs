//! `cost <item>` — **the headroom an item's dispatches drew, as ADR-0110's batch delta or the reason
//! none is quoted.** ADR-0163 decisions 1, 4 and 6; ADR-0110; AC-373, AC-395.
//!
//! The ninth verb (ADR-0163 decision 1). It reads the store's `rate_limit_reading` rows — one per
//! dispatch that went through the child, each carrying the `item` the composition root bound
//! (ADR-0163 decisions 2–3) — and renders them through `rk_adapter_claude::throttle::attribute`,
//! which until this module had no caller in any `src/`. It is **read-only**: it does not reconcile,
//! appends nothing, and never calls the harness it is handed (AC-395: *"from the event store alone
//! … it dispatches no model call"*).
//!
//! # What it renders, and the closed set of lines
//!
//! - **The empty case** — no reading in the store names this item. One line that says there is
//!   nothing to attribute and names none of ADR-0110's renderings: a `low_sample` over `n = 0` is
//!   the vacuous green `s7i` §7i.2 forbids in terms.
//! - **`not attributable interleaving=<n>`** — any foreign or `null`-item traffic between the
//!   item's first owned reading and its last. Counted over **traffic**, not samples:
//!   `throttle::attribute` sees only the samples it is given, and a foreign dispatch that produced
//!   no reading drew on the account all the same ([`interleaving`]).
//! - **Per window, a draw line** with `n`, the `resets_at` it is grouped by, the integer delta only
//!   when `throttle` quotes one, and the `low_sample` / `non_monotonic` / `resolution_limited`
//!   flags; a window whose readings carry ticks and a `null` `resets_at` says `unkeyed=<N>`; a window
//!   with neither says `no sample`.
//!
//! `first` and `last` are never printed — their difference is a delta, and printing both beside a
//! withheld one would quote it at `n < 10` by subtraction. **Nothing is divided by anything**, so
//! there is no per-run or per-dispatch figure anywhere in this file (AC-373; F6).
//!
//! # 🛑 WHAT IS NOT RENDERED, AND WHY
//!
//! AC-395's per-activity breakdown — token counters, `total_cost_usd`, wall clock, cache counters —
//! lives on `dispatch_completed`, and **`dispatch_completed` carries no item**. Attributing it would
//! need the same ownership key on a row `rk-types` owns, and `s7i` §7i.2 says *"nothing else"*; so
//! it is not rendered here and nothing here reads item ownership off that row or off journal
//! position (ADR-0163 decision 5).
//!
//! # 🛑 THE NAMED LIMIT: THIS STORE'S TRAFFIC ONLY
//!
//! ADR-0110 decision 1's named set is *"the account's complete dispatch sequence between the two
//! readings"*, and that is the one property this rendering cannot check: another repository, or
//! interactive `claude` use on the same account, draws on the same windows and never reaches this
//! store. The header line says *"this store's traffic only"* so no reader assumes otherwise
//! (ADR-0163 decision 4).

use std::io::Write;
use std::sync::{Arc, Mutex};

use rk_adapter_claude::config::Window;
use rk_adapter_claude::throttle::{self, Attribution, BatchDraw, Sample};
use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

use crate::cli::{Cli, CostArgs};
use crate::exit::{Exit, Halt};
use crate::{repo, store};

/// The outer verb: discover the repository, open the real store, delegate.
///
/// # Errors
/// [`Halt::PoisonedLock`] from [`crate::store::acquire`]. Every repository and store refusal is
/// exit `3` and is returned as a value, not a halt.
pub fn run(cli: &Cli, args: &CostArgs) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    // AC-413's identity check lives inside `open`: a store belonging to another repository is
    // refused here, before a single row is read.
    let opened = match rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
    let mut out = std::io::stdout();
    run_with(&NeverDispatched, &store, &args.item, &mut out)
}

/// The harness the shipped `cost` path holds: it panics on `dispatch` and names the verb.
///
/// Not [`crate::no_dispatch::NoDispatchHarness`], whose message says `doctor` dispatched — a
/// diagnostic that misnamed the verb would send a reader to the wrong rule table. A panic, not a
/// returned value, for the reason that module states: `dispatch` has no error arm, and a fabricated
/// `DispatchOutcome` would be a dispatch that succeeded as far as any caller could tell.
struct NeverDispatched;

impl Harness for NeverDispatched {
    fn dispatch(&self, spec: &ActivitySpec, _store: &mut dyn EventStore) -> DispatchOutcome {
        panic!(
            "AC-395: `cost` issued a harness dispatch — it renders from the event store alone. \
             Card digest: {:?}.",
            spec.card_digest
        )
    }
}

/// The inner verb — the in-process twin the tests drive.
///
/// # 🛑 IT TAKES A HARNESS IT NEVER CALLS, AND THAT IS THE POINT
///
/// AC-395's *"it dispatches no model call"* is an absence, and an absence over a function that
/// holds no harness is true of every possible body — evidence of nothing, the shape
/// `status::run_with`'s header describes for reconciliation. Handing one in makes the property
/// falsifiable: `ac395_cost_dispatches_nothing_under_a_panicking_harness` passes a fake whose
/// `dispatch` panics.
///
/// # Errors
/// [`Halt::PoisonedLock`] naming this verb.
pub fn run_with(
    harness: &dyn Harness,
    store: &Arc<Mutex<dyn EventStore>>,
    item: &str,
    out: &mut dyn Write,
) -> Result<Exit, Halt> {
    // Held and never called — see the doc comment above.
    let _ = harness;

    let guard = store::acquire(store, "cost")?;
    // The WHOLE journal. The selector is inert today (`SqliteStore::query` has no `WHERE`), and it
    // is passed empty rather than as the item on purpose: a future store that honoured `ByItem`
    // must not make foreign traffic vanish from the span, which is exactly what `not attributable`
    // is computed from (ADR-0163 decision 4).
    let read = guard.query(rk_kernel::runtime::journal_query(""));
    drop(guard);
    let rows = match read {
        Ok(rows) => rows,
        Err(e) => {
            // `StepError` has no `Display`; the `Debug` rendering keeps every field, on
            // `status::run_with`'s precedent.
            eprintln!("redkiln: the store could not be read: {e:?}");
            return Ok(Exit::ConfigError);
        }
    };

    let rendered = match render(&rows, item) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("redkiln: a reading owned by `{item}` is malformed: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    if let Err(e) = out.write_all(rendered.as_bytes()) {
        eprintln!("redkiln: the rendering could not be written: {e}");
        return Ok(Exit::ConfigError);
    }
    // Every rendering is `0`, the empty one included. `Exit::NoRecord` is `explain`'s alone
    // (ADR-0163 decision 1).
    Ok(Exit::Success)
}

/// A reading in the span whose payload is not the ADR-0163 decision 3 shape.
///
/// Exit `3`, as `explain` treats a record it cannot read — never a skipped row, which would shrink
/// `n` silently, and never a coerced value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Malformed {
    /// The row's index in the whole journal, in journal order.
    pub row: usize,
    /// Which member, and what it held.
    pub what: String,
}

impl std::fmt::Display for Malformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "journal row {}: {}", self.row, self.what)
    }
}

impl std::error::Error for Malformed {}

/// The `item` member of a row, when it is a string.
fn item_of(row: &Event) -> Option<&str> {
    row.payload.get("item").and_then(serde_json::Value::as_str)
}

/// Is `row` a reading this item owns? `null`, absent and non-string `item` are **not** owned — a
/// reading with no owner counts against attribution, never for it (ADR-0163's drivers).
fn owned(row: &Event, item: &str) -> bool {
    row.kind == EventKind::RateLimitReading && item_of(row) == Some(item)
}

/// The whole rendering, from the journal rows and the item alone. **Pure**: no store, no clock, no
/// environment.
///
/// # Errors
/// [`Malformed`] when a reading in the span does not carry the ADR-0163 decision 3 shape.
pub fn render(rows: &[Event], item: &str) -> Result<String, Malformed> {
    let first = rows.iter().position(|r| owned(r, item));
    let last = rows.iter().rposition(|r| owned(r, item));
    let (Some(first), Some(last)) = (first, last) else {
        let dispatches = rows
            .iter()
            .filter(|r| r.kind == EventKind::Dispatch && item_of(r) == Some(item))
            .count();
        return Ok(format!(
            "cost {item}: nothing to attribute \u{2014} no rate_limit_reading row in this store \
             names this item ({dispatches} dispatch rows)\n"
        ));
    };

    let span = &rows[first..=last];
    // Every OWNED reading in the span is validated BEFORE interleaving is counted, so a malformed
    // owned row is exit `3` whether or not the span interleaves. Foreign rows are never validated:
    // they are traffic here, not samples.
    for (i, row) in span.iter().enumerate() {
        if owned(row, item) {
            validate(row, first + i)?;
        }
    }
    let readings = span
        .iter()
        .filter(|r| r.kind == EventKind::RateLimitReading)
        .count();
    let mut out = format!(
        "cost {item}: rate_limit_reading rows in span: {readings}, this store's traffic only\n"
    );

    let foreign = interleaving(span, item);
    if foreign > 0 {
        out.push_str(&format!("not attributable interleaving={foreign}\n"));
        return Ok(out);
    }

    for window in [Window::FiveHour, Window::SevenDay] {
        render_window(&mut out, window, span, first)?;
    }
    Ok(out)
}

/// How much foreign traffic lies inside the span — ADR-0163 decision 4's count.
///
/// Walked in journal order with one flag, `foreign_open`: a foreign (or `null`-item) kernel
/// `dispatch` row whose reading has not been seen yet. Each such dispatch counts **once**, whether
/// or not it produced a reading — a dispatch refused before spawn, or killed before its first
/// `rate_limit_event`, never becomes a sample and drew all the same. A foreign reading that follows
/// its own foreign dispatch is that dispatch's and is not counted again; a foreign or `null`-item
/// reading that no counted dispatch accounts for counts once on its own. Every other kind —
/// `dispatch_completed` included, which carries no item — is not traffic here.
///
/// Errs toward `not attributable`: a foreign dispatch refused before spawn is counted though it
/// drew nothing (ADR-0163 *"What is NOT asserted"*). **The count can over-count across
/// interleaved processes**: when a foreign dispatch and its own reading straddle an owned reading,
/// the owned reading closes the foreign dispatch's flag and the foreign reading is then counted
/// again on its own — one foreign dispatch counted twice. The algorithm is left as it is because
/// every error it makes is in that direction: it errs toward `not attributable`, and it never
/// under-counts foreign traffic in the span to zero.
fn interleaving(span: &[Event], item: &str) -> u32 {
    let mut count = 0_u32;
    let mut foreign_open = false;
    for row in span {
        let mine = item_of(row) == Some(item);
        match row.kind {
            EventKind::Dispatch => {
                if foreign_open {
                    count = count.saturating_add(1);
                }
                foreign_open = !mine;
            }
            EventKind::RateLimitReading => {
                if !mine || foreign_open {
                    count = count.saturating_add(1);
                }
                foreign_open = false;
            }
            _ => {}
        }
    }
    if foreign_open {
        count = count.saturating_add(1);
    }
    count
}

/// The ADR-0163 decision 3 shape of one OWNED reading, checked whole before anything is rendered
/// from it.
///
/// All six members must be present as keys — a missing key is [`Malformed`], never "no sample",
/// which would shrink `n` silently. `windows` is an object carrying both windows as integer `u16`
/// ticks, or `null`; `utilization` is integer `u16` ticks or `null`, checked whenever present and
/// not only when `rate_limit_type` names a window; exactly one of the two is non-`null` (both or
/// neither is malformed — a reading is one shape or the other); `resets_at` is an object whose two
/// members are each integer seconds or `null`; `rate_limit_type` is a string or `null`; and
/// `cli_version` is present (its value is not interpreted here). No member is ever read as a
/// float (AC-374).
fn validate(row: &Event, at: usize) -> Result<(), Malformed> {
    let malformed = |what: String| Malformed { row: at, what };
    let Some(payload) = row.payload.as_object() else {
        return Err(malformed(format!(
            "the payload is {:?}, not an object",
            row.payload
        )));
    };
    for key in [
        "windows",
        "utilization",
        "resets_at",
        "rate_limit_type",
        "item",
        "cli_version",
    ] {
        if !payload.contains_key(key) {
            return Err(malformed(format!("`{key}` is missing")));
        }
    }
    let is_ticks = |v: &serde_json::Value| v.as_u64().and_then(|t| u16::try_from(t).ok()).is_some();
    let windows = &row.payload["windows"];
    match windows {
        serde_json::Value::Null => {}
        serde_json::Value::Object(w) => {
            for name in [Window::FiveHour.as_str(), Window::SevenDay.as_str()] {
                if !w.get(name).is_some_and(is_ticks) {
                    return Err(malformed(format!(
                        "`windows.{name}` is {:?}, not integer ticks in u16",
                        w.get(name)
                    )));
                }
            }
        }
        other => {
            return Err(malformed(format!(
                "`windows` is {other:?}, neither an object nor null"
            )));
        }
    }
    let utilization = &row.payload["utilization"];
    if !utilization.is_null() && !is_ticks(utilization) {
        return Err(malformed(format!(
            "`utilization` is {utilization:?}, not integer ticks in u16"
        )));
    }
    if windows.is_null() == utilization.is_null() {
        return Err(malformed(format!(
            "`windows` is {windows:?} and `utilization` is {utilization:?}: exactly one of the \
             two must be non-null"
        )));
    }
    let Some(resets) = row.payload["resets_at"].as_object() else {
        return Err(malformed(format!(
            "`resets_at` is {:?}, not an object",
            row.payload["resets_at"]
        )));
    };
    for name in [Window::FiveHour.as_str(), Window::SevenDay.as_str()] {
        match resets.get(name) {
            Some(serde_json::Value::Null) => {}
            Some(v) if v.as_i64().is_some() => {}
            other => {
                return Err(malformed(format!(
                    "`resets_at.{name}` is {other:?}, not integer unix seconds or null"
                )));
            }
        }
    }
    let kind = &row.payload["rate_limit_type"];
    if !kind.is_null() && !kind.is_string() {
        return Err(malformed(format!(
            "`rate_limit_type` is {kind:?}, neither a string nor null"
        )));
    }
    Ok(())
}

/// One window's lines, appended to `out`.
///
/// A sample is formed from each reading that carries this window's ticks **and** a non-`null`
/// `resets_at`. A reading with ticks and a `null` `resets_at` closes the current segment and is
/// counted `unkeyed` — never grouped under `0`, an epoch or "now", and no delta is ever formed
/// across it (ADR-0163's forbids). Within a segment `throttle` splits per `resets_at` itself.
fn render_window(
    out: &mut String,
    window: Window,
    span: &[Event],
    offset: usize,
) -> Result<(), Malformed> {
    let name = window.as_str();
    let mut segments: Vec<Vec<Sample>> = vec![Vec::new()];
    let mut unkeyed = 0_u32;
    for (i, row) in span.iter().enumerate() {
        if row.kind != EventKind::RateLimitReading {
            continue;
        }
        let at = offset + i;
        let Some(ticks) = ticks(row, window, at)? else {
            continue;
        };
        match resets_at(row, window, at)? {
            Some(secs) => {
                if let Some(segment) = segments.last_mut() {
                    segment.push(Sample {
                        resets_at: Timestamp::from_unix_seconds(secs),
                        ticks,
                    });
                }
            }
            None => {
                unkeyed = unkeyed.saturating_add(1);
                segments.push(Vec::new());
            }
        }
    }

    let mut drew = false;
    for samples in segments.iter().filter(|s| !s.is_empty()) {
        // Every reading in the span is owned — any other was interleaving and stopped the
        // rendering above — so the flags are all `true`. They are still passed, because
        // `attribute` is the calculator's one entry point and its `NotAttributable` arm is
        // rendered rather than assumed away.
        let owned = vec![true; samples.len()];
        match throttle::attribute(window, samples, &owned) {
            Attribution::Attributable(draws) => {
                for draw in &draws {
                    out.push_str(&draw_line(name, draw));
                    drew = true;
                }
            }
            Attribution::NotAttributable { interleaving } => {
                out.push_str(&format!("not attributable interleaving={interleaving}\n"));
            }
        }
    }
    if unkeyed > 0 {
        out.push_str(&format!("{name}: unkeyed={unkeyed}\n"));
    }
    if !drew && unkeyed == 0 {
        out.push_str(&format!("{name}: no sample\n"));
    }
    Ok(())
}

/// One draw, in the closed grammar: `n`, the batch key, the delta only when `throttle` quoted one,
/// then the flags. `first` and `last` are never printed (the module doc).
fn draw_line(name: &str, draw: &BatchDraw) -> String {
    let mut line = format!(
        "{name}: n={} resets_at={}",
        draw.n,
        draw.resets_at.unix_seconds()
    );
    if let Some(delta) = draw.delta_ticks {
        line.push_str(&format!(" delta_ticks={delta}"));
    }
    if draw.low_sample {
        line.push_str(" low_sample");
    }
    if draw.non_monotonic {
        line.push_str(" non_monotonic");
    }
    if draw.resolution_limited {
        line.push_str(" resolution_limited");
    }
    line.push('\n');
    line
}

/// This window's ticks off one reading, or `None` when the reading carries none for it.
///
/// A unified reading's `windows` object answers for both windows. A flat reading (`windows: null`)
/// answers **only** for the window its `rate_limit_type` names — writing one utilisation into both,
/// or into a window it does not name, is the extrapolation AC-374 forbids. Read as an integer and
/// narrowed to `u16`; a float is a malformed row, never read (AC-374).
fn ticks(row: &Event, window: Window, at: usize) -> Result<Option<u16>, Malformed> {
    let name = window.as_str();
    let malformed = |what: String| Malformed { row: at, what };
    match row.payload.get("windows") {
        Some(serde_json::Value::Object(windows)) => {
            let value = windows.get(name);
            value
                .and_then(serde_json::Value::as_u64)
                .and_then(|t| u16::try_from(t).ok())
                .map(Some)
                .ok_or_else(|| {
                    malformed(format!(
                        "`windows.{name}` is {value:?}, not integer ticks in u16"
                    ))
                })
        }
        Some(serde_json::Value::Null) | None => {
            let named = row
                .payload
                .get("rate_limit_type")
                .and_then(serde_json::Value::as_str)
                == Some(name);
            match row.payload.get("utilization") {
                Some(serde_json::Value::Null) | None => Ok(None),
                Some(value) if named => value
                    .as_u64()
                    .and_then(|t| u16::try_from(t).ok())
                    .map(Some)
                    .ok_or_else(|| {
                        malformed(format!(
                            "`utilization` is {value:?}, not integer ticks in u16"
                        ))
                    }),
                Some(_) => Ok(None),
            }
        }
        Some(other) => Err(malformed(format!(
            "`windows` is {other:?}, neither an object nor null"
        ))),
    }
}

/// This window's `resets_at` off one reading, as the integer unix seconds the platform reported.
/// `null` (or absent) is `None` and stays `None` — never coerced (ADR-0163's forbids).
fn resets_at(row: &Event, window: Window, at: usize) -> Result<Option<i64>, Malformed> {
    let name = window.as_str();
    let malformed = |what: String| Malformed { row: at, what };
    let Some(serde_json::Value::Object(resets)) = row.payload.get("resets_at") else {
        return Err(malformed(format!(
            "`resets_at` is {:?}, not an object",
            row.payload.get("resets_at")
        )));
    };
    match resets.get(name) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(value) => value.as_i64().map(Some).ok_or_else(|| {
            malformed(format!(
                "`resets_at.{name}` is {value:?}, not integer unix seconds"
            ))
        }),
    }
}
