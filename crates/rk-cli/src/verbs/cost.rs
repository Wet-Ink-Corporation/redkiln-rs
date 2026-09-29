//! `cost <item>` — **the headroom an item's dispatches drew, as ADR-0110's batch delta or the reason
//! none is quoted.** ADR-0163 decisions 1, 4 and 6; ADR-0110; AC-373, AC-395.
//!
//! The ninth verb (ADR-0163 decision 1). It reads the store's `rate_limit_reading` rows — one per
//! dispatch that went through the child, each carrying the `item` the composition root bound
//! (ADR-0163 decisions 2–3) — and renders them through `rk_adapter_claude::throttle::attribute`,
//! which until this module had no caller in any `src/`. It **appends no event and writes no journal
//! row** (ADR-0163 decision 1): it does not reconcile, and never calls the harness it is handed
//! (AC-395: *"from the event store alone … it dispatches no model call"*). It is not strictly
//! read-only: opening the store through `SqliteStore::open` applies the schema and, on a repository
//! with no store, creates and identity-stamps one — as `status` and `explain` already do.
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
//!   with neither says `no sample`. A draw at `n < 10` is always `low_sample` and never carries a
//!   delta; a draw at `n >= 10` is never `low_sample` and carries a delta unless it is
//!   `non_monotonic` (`throttle::batch_draw`).
//! - **`unwindowed=<N>`** — after both windows, when `N >= 1` owned flat readings (`windows: null`)
//!   carry a `rate_limit_type` naming neither window (`null`, or another string). Such a reading
//!   answers for no window, so it forms no sample in either; it is counted here rather than dropped
//!   silently, and never written into both windows (the extrapolation AC-374 forbids).
//!
//! The closed set, as regular expressions over each line:
//!
//! ```text
//! ^cost \S+: nothing to attribute — no rate_limit_reading row in this store names this item \([0-9]+ dispatch rows\)$
//! ^cost \S+: rate_limit_reading rows in span: [0-9]+, this store's traffic only$
//! ^not attributable interleaving=[1-9][0-9]*$
//! ^(five_hour|seven_day): n=[1-9] resets_at=-?[0-9]+ low_sample( non_monotonic| resolution_limited)?$
//! ^(five_hour|seven_day): n=[1-9][0-9]+ resets_at=-?[0-9]+( non_monotonic| delta_ticks=[01] resolution_limited| delta_ticks=([2-9]|[1-9][0-9]+))$
//! ^(five_hour|seven_day): unkeyed=[1-9][0-9]*$
//! ^(five_hour|seven_day): no sample$
//! ^unwindowed=[1-9][0-9]*$
//! ^breakdown \S+: dispatch_completed rows owned: [1-9][0-9]*$
//! ^activity (\S+@\S+|\(none\)): dispatches=[1-9][0-9]* input_tokens=<S> output_tokens=<S> cache_read_input_tokens=<S> cache_creation_input_tokens=<S> wall_ms=[0-9]+ total_cost_usd=<D>$
//! ```
//!
//! *(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; ADR-0190 decision
//! 4, proposed.)* The last two forms are AC-395's breakdown ([`breakdown`]), printed after the
//! lines above when the item OWNS at least one `dispatch_completed` row. `<S>` is `<sum>`,
//! `<sum>(absent=<k>)` or `absent`; `<D>` is the same with the sum a decimal. ADR-0163's eight forms
//! stand as written.
//!
//! `first` and `last` are never printed — their difference is a delta, and printing both beside a
//! withheld one would quote it at `n < 10` by subtraction. **Nothing is divided by anything**, so
//! there is no per-run or per-dispatch figure anywhere in this file (AC-373; F6).
//!
//! # 🛑 WHAT IS NOT RENDERED, AND WHY
//!
//! *(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; ADR-0190 decisions 3
//! and 4, proposed.)* **The paragraph below no longer holds, and is kept as the history.** The
//! adapter keys its `dispatch_completed` row with the spec's `item` on ADR-0182's carrier beside the
//! event, and the row carries `activity_id_at_version`; [`breakdown`] renders AC-395's per-activity
//! lines over the rows an item OWNS by that key. Ownership is never read off the payload and never
//! off journal position (ADR-0163's journal-position half stands whole).
//!
//! ~~AC-395's per-activity breakdown~~ — token counters, `total_cost_usd`, wall clock, cache counters —
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
//!
//! The same boundary holds **inside** this store: interleaving is counted from the rows that lie in
//! the span. A foreign dispatch whose `dispatch` row precedes the item's first owned reading and
//! whose reading follows its last — one in flight across the whole span, from a second process —
//! leaves no row inside the span and is **not counted**. "This store's traffic only" and "in the
//! span" are the boundary; nothing outside the span is read as traffic ([`interleaving`]).

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
    run_to(cli, args, &mut std::io::stdout())
}

/// *(dated 2026-09-26, s8c build; D21-64 ruling 3.)* [`run`] with the writer handed in; an
/// in-process caller passes its own, so nothing it renders reaches libtest's stdout.
///
/// # Errors
/// As [`run`].
pub fn run_to(cli: &Cli, args: &CostArgs, out: &mut dyn Write) -> Result<Exit, Halt> {
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
    run_with(&NeverDispatched, &store, &args.item, out)
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
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; ADR-0182 decision 4's owed whole-store
    // selector.)* ~~`journal_query("")`~~: `query` honours `ByItem` now, and `ByItem("")` returns only
    // the unkeyed rows once producers key theirs (`s8e` §8e.3). `ByRun` is inert and returns every
    // row, which is the whole journal this read needs.
    let read = guard.query(rk_ports::event_store::Query::ByRun(String::new()));
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
/// *(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; ADR-0190 decision 4,
/// proposed.)* The readings' lines, then AC-395's [`breakdown`] when the item owns at least one
/// `dispatch_completed` row. An item owning neither prints exactly the one empty line.
///
/// # Errors
/// [`Malformed`] when a reading in the span does not carry the ADR-0163 decision 3 shape, or an
/// owned `dispatch_completed` row does not carry the members the breakdown reads.
pub fn render(rows: &[Event], item: &str) -> Result<String, Malformed> {
    let mut out = readings(rows, item)?;
    out.push_str(&breakdown(rows, item)?);
    Ok(out)
}

/// **The draw lines alone** (ADR-0190 decision 5).
///
/// Every line [`render`] prints after its header for an item owning at
/// least one reading, and the EMPTY string for an item owning none (no headroom line is ever
/// printed without a reading: AC-372's `status` half, ADR-0190 decision 5). Each draw is rendered
/// through [`draw_line`], the one draw renderer; `status` prints these lines and has no renderer of
/// its own.
///
/// # Errors
/// As [`render`], for the readings.
pub fn draw_lines(rows: &[Event], item: &str) -> Result<String, Malformed> {
    if !rows.iter().any(|r| owned(r, item)) {
        return Ok(String::new());
    }
    let text = readings(rows, item)?;
    Ok(text
        .split_once('\n')
        .map_or_else(String::new, |(_, rest)| rest.to_owned()))
}

/// ADR-0163's rendering: the readings' lines only.
fn readings(rows: &[Event], item: &str) -> Result<String, Malformed> {
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
    // Every reading in the span is owned here (any other was interleaving) and validated, so a
    // `null` `windows` means a non-`null` integer `utilization`.
    let unwindowed = span
        .iter()
        .filter(|r| r.kind == EventKind::RateLimitReading && unwindowed(r))
        .count();
    if unwindowed > 0 {
        out.push_str(&format!("unwindowed={unwindowed}\n"));
    }
    Ok(out)
}

/// The four token counters the breakdown sums, in the order its line prints them (ADR-0190 decision
/// 4): the documented names on `usage`, read verbatim — the `ephemeral_*` SDK namings are not
/// substituted for an absent one, because a substituted counter would be a second naming passed off
/// as the first.
const COUNTERS: [&str; 4] = [
    "input_tokens",
    "output_tokens",
    "cache_read_input_tokens",
    "cache_creation_input_tokens",
];

/// One activity's group of owned rows.
#[derive(Default)]
struct Group {
    dispatches: u64,
    /// Per counter: the sum over the rows that carry it, and how many rows did not.
    counters: [(u128, u64); 4],
    wall_ms: u128,
    /// The decimal sum of the rows that carry a cost, and how many rows did not.
    cost: (Option<Decimal>, u64),
}

/// **AC-395's breakdown** (ADR-0190 decision 4).
///
/// The header and one line per activity over the
/// `dispatch_completed` rows `item` OWNS — the rows whose ADR-0182 key beside the event is `item`
/// (`Event::item`), never a payload member and never journal position — or the EMPTY string when it
/// owns none. Activities are ordered bytewise by `<id>@<version>`; owned rows whose
/// `activity_id_at_version` is `null` or absent group under `(none)`, printed last.
///
/// An absent counter is never zero (AC-362): a group names how many of its rows lacked it, and a
/// group none of whose rows carries it reads `absent`. `total_cost_usd` is summed as decimal digits
/// ([`Decimal`]), with no binary float anywhere (ADR-0133 clause 5). No line carries a headroom
/// figure (AC-373).
///
/// # Errors
/// [`Malformed`] for an owned row whose counters are not integers, whose `timings.wall_ms` is not an
/// integer, whose `total_cost_usd` is not decimal digits, or whose `activity_id_at_version` is
/// neither a string carrying its `@version` nor `null`.
pub fn breakdown(rows: &[Event], item: &str) -> Result<String, Malformed> {
    let mut named: std::collections::BTreeMap<String, Group> = std::collections::BTreeMap::new();
    let mut none: Option<Group> = None;
    let mut owned_rows = 0_u64;
    for (at, row) in rows.iter().enumerate() {
        if row.kind != EventKind::DispatchCompleted || row.item() != Some(item) {
            continue;
        }
        owned_rows += 1;
        let malformed = |what: String| Malformed { row: at, what };
        let group = match row.payload.get("activity_id_at_version") {
            None | Some(serde_json::Value::Null) => none.get_or_insert_with(Group::default),
            Some(serde_json::Value::String(a))
                if a.split_once('@')
                    .is_some_and(|(id, v)| !id.is_empty() && !v.is_empty()) =>
            {
                named.entry(a.clone()).or_default()
            }
            Some(other) => {
                return Err(malformed(format!(
                    "`activity_id_at_version` is {other:?}, neither `<id>@<version>` nor null"
                )));
            }
        };
        group.dispatches += 1;
        let usage = row.payload.get("usage").filter(|u| !u.is_null());
        for (i, name) in COUNTERS.iter().enumerate() {
            match usage.and_then(|u| u.get(*name)) {
                None | Some(serde_json::Value::Null) => group.counters[i].1 += 1,
                Some(v) => {
                    let n = v.as_u64().ok_or_else(|| {
                        malformed(format!("`usage.{name}` is {v:?}, not an integer count"))
                    })?;
                    group.counters[i].0 += u128::from(n);
                }
            }
        }
        let wall = row
            .payload
            .get("timings")
            .and_then(|t| t.get("wall_ms"))
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| malformed("`timings.wall_ms` is not an integer".to_owned()))?;
        group.wall_ms += u128::from(wall);
        match row.payload.get("total_cost_usd") {
            None | Some(serde_json::Value::Null) => group.cost.1 += 1,
            Some(serde_json::Value::String(text)) => {
                let d = Decimal::parse(text).ok_or_else(|| {
                    malformed(format!("`total_cost_usd` is {text:?}, not decimal digits"))
                })?;
                group.cost.0 = Some(match group.cost.0.take() {
                    Some(sum) => sum.add(&d),
                    None => d,
                });
            }
            Some(other) => {
                return Err(malformed(format!(
                    "`total_cost_usd` is {other:?}, not the decimal string the CLI printed"
                )));
            }
        }
    }
    if owned_rows == 0 {
        return Ok(String::new());
    }
    let mut out = format!("breakdown {item}: dispatch_completed rows owned: {owned_rows}\n");
    let groups = named
        .iter()
        .map(|(a, g)| (a.as_str(), g))
        .chain(none.iter().map(|g| ("(none)", g)));
    for (activity, g) in groups {
        let sum = |value: String, absent: u64| {
            if absent == g.dispatches {
                "absent".to_owned()
            } else if absent > 0 {
                format!("{value}(absent={absent})")
            } else {
                value
            }
        };
        let mut line = format!("activity {activity}: dispatches={}", g.dispatches);
        for (i, name) in COUNTERS.iter().enumerate() {
            let (total, absent) = g.counters[i];
            line.push_str(&format!(" {name}={}", sum(total.to_string(), absent)));
        }
        line.push_str(&format!(" wall_ms={}", g.wall_ms));
        let cost = g.cost.0.as_ref().map_or_else(String::new, Decimal::render);
        line.push_str(&format!(" total_cost_usd={}", sum(cost, g.cost.1)));
        line.push('\n');
        out.push_str(&line);
    }
    Ok(out)
}

/// **A non-negative decimal as its digits** — `total_cost_usd`'s arithmetic (ADR-0133 clause 5:
/// *"sum them **as decimals** with no intermediate binary floating-point step"*).
///
/// The value is `digits × 10^-scale`, held as the digit string itself (most significant first).
/// Addition aligns the two scales by appending zeros and adds digit by digit with a carry; the sum
/// keeps the larger scale, so `0.0031 + 0.0009` is `0.0040`, never a trimmed or rounded figure. No
/// `f64`, and no fixed-width integer that a long digit string could overflow.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Decimal {
    digits: Vec<u8>,
    scale: usize,
}

impl Decimal {
    /// `[0-9]+(\.[0-9]+)?`, or `None`. No sign, no exponent, no bare point.
    fn parse(text: &str) -> Option<Self> {
        let (int, frac) = text.split_once('.').unwrap_or((text, ""));
        let all = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
        if !all(int) || (text.contains('.') && !all(frac)) {
            return None;
        }
        let digits = int.bytes().chain(frac.bytes()).map(|b| b - b'0').collect();
        Some(Self {
            digits,
            scale: frac.len(),
        })
    }

    /// The digit-wise sum, at the larger of the two scales.
    fn add(&self, other: &Self) -> Self {
        let scale = self.scale.max(other.scale);
        let widen = |d: &Self| {
            let mut v = d.digits.clone();
            v.extend(std::iter::repeat_n(0, scale - d.scale));
            v
        };
        let (a, b) = (widen(self), widen(other));
        let mut out: Vec<u8> = Vec::with_capacity(a.len().max(b.len()) + 1);
        let mut carry = 0_u8;
        let (mut i, mut j) = (a.len(), b.len());
        while i > 0 || j > 0 || carry > 0 {
            let x = if i > 0 {
                i -= 1;
                a[i]
            } else {
                0
            };
            let y = if j > 0 {
                j -= 1;
                b[j]
            } else {
                0
            };
            let t = x + y + carry;
            out.push(t % 10);
            carry = t / 10;
        }
        out.reverse();
        Self { digits: out, scale }
    }

    /// The digits back as text, with the point at `scale` and the integer part's leading zeros
    /// dropped (one kept).
    fn render(&self) -> String {
        let split = self.digits.len().saturating_sub(self.scale);
        let text: String = self.digits.iter().map(|d| char::from(b'0' + d)).collect();
        let (int, frac) = text.split_at(split);
        let int = int.trim_start_matches('0');
        let int = if int.is_empty() { "0" } else { int };
        if self.scale == 0 {
            int.to_owned()
        } else if frac.len() < self.scale {
            format!("{int}.{}{frac}", "0".repeat(self.scale - frac.len()))
        } else {
            format!("{int}.{frac}")
        }
    }
}

/// Is `row` a flat reading (`windows: null`) whose `rate_limit_type` names neither window — `null`
/// or any other string? It answers for no window, so [`ticks`] forms no sample from it in either;
/// [`render`] counts it on the `unwindowed=<N>` line instead of dropping it (the module doc).
fn unwindowed(row: &Event) -> bool {
    if !row
        .payload
        .get("windows")
        .is_some_and(serde_json::Value::is_null)
    {
        return false;
    }
    let kind = row
        .payload
        .get("rate_limit_type")
        .and_then(serde_json::Value::as_str);
    kind != Some(Window::FiveHour.as_str()) && kind != Some(Window::SevenDay.as_str())
}

/// How much foreign traffic lies inside the span — ADR-0163 decision 4's count.
///
/// Walked in journal order with one flag, `foreign_open`: a foreign (or `null`-item) kernel
/// `dispatch` row whose reading has not been seen yet. Each such dispatch counts **once**, whether
/// or not it produced a reading — a dispatch refused before spawn, or killed before its first
/// `rate_limit_event`, never becomes a sample and drew all the same. It is counted when the flag
/// is closed: by the next `dispatch` row (any owner), or by the next reading (an owned reading
/// after it counts it; a foreign reading after it is that dispatch's own and counts it once, not
/// twice). A foreign or `null`-item reading that no open dispatch accounts for counts once on its
/// own. Every other kind — `dispatch_completed` included, which carries no item — is not traffic
/// here. The span always ends at an owned reading, which closes the flag, so no dispatch is left
/// open when the walk ends.
///
/// **The named limit** (the module doc): only rows inside the span are traffic. A foreign dispatch
/// whose `dispatch` row precedes the span and whose reading follows it — in flight across the whole
/// span, from a second process — leaves no row inside the span and is not counted. "This store's
/// traffic only" and "in the span" are the boundary.
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
    // `render` slices the span from the first owned reading to the last, so its final row is an
    // owned `RateLimitReading`, whose arm above clears the flag. A trailing count here would be
    // dead code; the assertion states the invariant instead.
    debug_assert!(
        !foreign_open,
        "the span ends at an owned reading, which closes every open foreign dispatch"
    );
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
/// `cli_version` is a serialised `CliVersion` — an object with exactly one key, `observed` or
/// `pinned`, holding a string ([`is_cli_version`]); its version string is not interpreted here. No
/// member is ever read as a float (AC-374).
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
    let version = &row.payload["cli_version"];
    if !is_cli_version(version) {
        return Err(malformed(format!(
            "`cli_version` is {version:?}, not a serialised CliVersion (one key, `observed` or \
             `pinned`, holding a string)"
        )));
    }
    Ok(())
}

/// Is `v` an `rk_types::usage::CliVersion` as serde writes it — an object with exactly one key,
/// the `snake_case` arm name (`observed` or `pinned`, the enum's only two variants), whose value
/// is a string? Checked structurally rather than by deserialising, so no other shape is coerced.
fn is_cli_version(v: &serde_json::Value) -> bool {
    let Some(obj) = v.as_object() else {
        return false;
    };
    obj.len() == 1
        && ["observed", "pinned"]
            .iter()
            .any(|arm| obj.get(*arm).is_some_and(serde_json::Value::is_string))
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
                    out.push_str(&draw_line(draw));
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
///
/// *(dated 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; ADR-0190 decisions
/// 5 and 6, proposed.)* **THE one draw renderer.** `status` prints an item's draws through
/// [`draw_lines`], which renders each draw here, so a flag cannot be kept on one surface and dropped
/// on the other (AC-372). It has no second home.
#[must_use]
pub fn draw_line(draw: &BatchDraw) -> String {
    let name = draw.window.as_str();
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
