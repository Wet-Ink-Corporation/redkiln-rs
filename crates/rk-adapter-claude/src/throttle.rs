//! Headroom-based dispatch gating in integer ticks — the guard that refuses before spawn, and the
//! batch-delta calculator (`s7c` §7c.8; ADR-0110; AC-372, AC-375, AC-376, AC-377, AC-220, AC-381).
//!
//! **Lifted, with edits, from `runner/src/throttle.rs` in `redkiln-corpus` at
//! `61f590e51cbb00301c310e46aab615563a7787b8`** (the SHA in `CORPUS_PIN`; blob
//! `9b7e1126111fc6bd56941a7ba130906fc81d4a3f`, 233 lines). That file was the spike runner's
//! ceiling check over the undocumented `rate_limit_event`, **built and run over E0's batches and
//! exercised, not validated against an oracle** (PIA-036, PIA-062). Two properties of the
//! instrument shaped it and shape this module, both measured (F1, F6): **`utilization` is
//! quantised to 0.01** — a single dispatch does not move it, so the guard is a *ceiling* check,
//! never a per-step cost prediction, and per-run cost is only ever a batch delta divided by `n` —
//! and **there is no Opus or Sonnet family key**: `unifiedWindows` carries `five_hour` and
//! `seven_day` only, so this guard can only keep a run out of the shared windows (AC-375).
//!
//! **What is lifted** (the table's `throttle.rs` row): the [`Guard`] / [`Verdict`] shape —
//! [`Guard::observe`] keeps the LAST reading, [`Guard::check`] answers `Proceed` on the opening
//! dispatch (*"headroom is only learned from a dispatch"*), `HaltOverage` on
//! `isUsingOverage: true` (charter §6.2: crossing into usage credits silently drops the cache TTL
//! from 1h to 5m, so records either side are not comparable — AC-221's refusal), and `Halt` on a
//! window at or past its margin.
//!
//! **Edits, each forced by a rule the corpus version predates:**
//!
//! 1. **`Margins` loses `impl Default`** and becomes integer `margin_ticks` per window from the
//!    pack (`config::Margins`, a `Config` value — AC-377: *"the margin comes from the pack, never
//!    from a code default"*; the runner's `max_five_hour: 0.90` was a code margin). `Margins` is
//!    declared in `config.rs` because it is configuration; this module consumes it.
//! 2. **`Guard::check` compares `ticks >= 100 - margin_ticks`** (AC-376; ADR-0110 decision 2) —
//!    the margin is consumed HERE, by the guard, and **not** by [`batch_draw`]. The runner compared
//!    `f64` utilisation against an `f64` ceiling; here both sides are `u16`. Halt 12: a firing
//!    guard is a reading, never a reason to move `margin_ticks`.
//! 3. **The input is `headroom::RateLimitReading`**, whose ticks are the digit scanner's
//!    (AC-374) and whose windows are exactly two (AC-375; `rk_types::usage::WindowReading`); the
//!    runner's `event::Headroom` with its `Option<f64>` members is not lifted.
//!
//! **Rewritten, not lifted: [`batch_draw`]** (the table's *rewritten* column; O-7: the calculator
//! is the adapter's, the reader over the log that assembles a dispatch set is 7d's `redkiln cost`).
//! The runner's `delta < 0.02` on `f64` is the defect AC-372 names: under `f64`, `0.12 − 0.10` is
//! `0.01999999999999999`, which `< 0.02` flags, while ticks read `2` and do not [measured
//! 2026-09-17]. In ticks: `first`/`last` are `u16`, `delta_ticks = last − first`,
//! `resolution_limited = delta_ticks <= 1` (ADR-0110 decision 2), `dispatches.max(1)` is dropped
//! (a batch of zero dispatches is `low_sample`, not a batch of one), `n < 10` is `low_sample` **and
//! no delta is quoted** (SM-111's floor; AC-372), a batch that crosses a window reset is **split
//! per `resetsAt`** and each partial window reported with its own `n`
//! ([`batch_draws_by_reset`]), and a set whose dispatches were not the only traffic is
//! **`not attributable` with the interleaving count** ([`attribute`]). No `f64` arithmetic in
//! any form (the block's AC-374 ban reaches this module).
//!
//! **AC-381's type-level separation**: the calculator's input is [`Sample`] — a `resets_at` and a
//! tick count — and neither it nor [`BatchDraw`] can name a cache field; the trybuild case
//! `tests/ui/ac381_cache_field_on_headroom_input.rs` is the compile-fail instrument and
//! `ac381_headroom_calculator_still_delivers_a_delta` the positive one.

use rk_types::time::Timestamp;

use crate::config::{Margins, Window};
use crate::headroom::RateLimitReading;

/// The fewest dispatches a batch needs before a delta is quoted (AC-372; SM-111's floor,
/// mirroring AC-382).
pub const LOW_SAMPLE_BELOW: u32 = 10;

/// The guard's answer for the next dispatch (lifted shape).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Dispatch. Carries no promise about what the step will cost (lifted).
    Proceed,
    /// A window is at or past its margin: `ticks >= 100 - margin_ticks` (AC-376). The dispatch is
    /// refused before spawn as `RateLimited { resets_at }` with the window's own `resetsAt`.
    Halt {
        window: Window,
        ticks: u16,
        margin_ticks: u16,
        resets_at: Option<Timestamp>,
    },
    /// The last reading reported `isUsingOverage: true` (AC-221; charter §6.2): the dispatch is
    /// refused before spawn as `Overage`, whatever the windows read.
    HaltOverage { overage_status: Option<String> },
}

impl Verdict {
    /// Anything but `Proceed`.
    #[must_use]
    pub const fn is_halt(&self) -> bool {
        !matches!(self, Self::Proceed)
    }
}

/// Tracks the last headroom reading and decides whether to dispatch again (lifted).
///
/// The first dispatch of a batch is necessarily unguarded: headroom is only learned *from* a
/// dispatch. That is acceptable because one step cannot move a quantised figure, but it is the
/// reason a batch must never be started close to a margin in the first place (lifted). **No
/// `Default`**: the guard starts empty through [`Guard::empty`] (named so, not `new`, because a
/// `new()` with no arguments is a `Default` by another spelling), and a reading enters only
/// through [`Guard::observe`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guard {
    last: Option<RateLimitReading>,
}

impl Guard {
    /// An empty guard — nothing observed yet.
    #[must_use]
    pub const fn empty() -> Self {
        Self { last: None }
    }

    /// Keep the LAST reading: it is the closest to the next dispatch's true cost (lifted).
    pub fn observe(&mut self, reading: &RateLimitReading) {
        self.last = Some(reading.clone());
    }

    /// The last reading, if any.
    #[must_use]
    pub const fn last(&self) -> Option<&RateLimitReading> {
        self.last.as_ref()
    }

    /// Decide the next dispatch against `margins`: overage first, then each window in the
    /// register's order (`five_hour`, then `seven_day`) at `ticks >= 100 - margin_ticks`.
    ///
    /// A flat (SDK-shape) reading is checked on the window its `rateLimitType` names, or on both
    /// when it names none (`RateLimitReading::ticks`); a window the reading carries nothing for
    /// is not checked and never synthesised (AC-374).
    #[must_use]
    pub fn check(&self, margins: Margins) -> Verdict {
        let Some(reading) = &self.last else {
            return Verdict::Proceed;
        };
        if reading.is_using_overage == Some(true) {
            return Verdict::HaltOverage {
                overage_status: reading.overage_status.clone(),
            };
        }
        for window in [Window::FiveHour, Window::SevenDay] {
            let margin_ticks = margins.ticks(window);
            if let Some(ticks) = reading.ticks(window)
                && ticks >= 100u16.saturating_sub(margin_ticks)
            {
                return Verdict::Halt {
                    window,
                    ticks,
                    margin_ticks,
                    resets_at: reading.resets_at(window),
                };
            }
        }
        Verdict::Proceed
    }
}

/// One reading of one window, as the calculator takes it: the window's `resetsAt` and its ticks.
/// **Cannot name a cache field** (AC-381) — this is the calculator's whole input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sample {
    pub resets_at: Timestamp,
    pub ticks: u16,
}

/// Per-dispatch draw over a batch, reported honestly (rewritten in ticks).
///
/// `utilization` is quantised to 0.01, so a per-run figure is meaningless and this deliberately
/// reports the batch (lifted rationale).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchDraw {
    pub window: Window,
    /// The `resetsAt` every sample in this batch shares (a batch never crosses a reset —
    /// ADR-0110 decision 1; [`batch_draws_by_reset`] splits at one).
    pub resets_at: Timestamp,
    pub first: u16,
    pub last: u16,
    /// `last − first`, quoted only when `n >= 10` (AC-372: a `low_sample` set quotes no delta) and
    /// the readings did not fall inside one window (a fall is `non_monotonic`, not a negative
    /// draw).
    pub delta_ticks: Option<u16>,
    /// The number of dispatches in the set — reported with the batch, never divided into it
    /// (AC-373: no per-dispatch accessor).
    pub n: u32,
    /// True when the whole batch moved the figure by at most one tick — the mean is then an
    /// artefact of the instrument, not a measurement (ADR-0110 decision 2: `delta_ticks <= 1`).
    pub resolution_limited: bool,
    /// True when `n < 10` (SM-111's floor): no delta is quoted.
    pub low_sample: bool,
    /// True when `last < first` inside one `resetsAt` — a reading the instrument's own model
    /// (monotonic within a window) does not admit; reported, never subtracted.
    pub non_monotonic: bool,
}

/// The draw of one batch of `n` dispatches between `first` and `last` ticks sharing `resets_at`.
#[must_use]
pub fn batch_draw(
    window: Window,
    resets_at: Timestamp,
    first: u16,
    last: u16,
    n: u32,
) -> BatchDraw {
    let low_sample = n < LOW_SAMPLE_BELOW;
    let raw_delta = last.checked_sub(first);
    let non_monotonic = raw_delta.is_none();
    let delta_ticks = if low_sample { None } else { raw_delta };
    BatchDraw {
        window,
        resets_at,
        first,
        last,
        delta_ticks,
        n,
        // A delta the instrument cannot resolve is flagged whether or not it is quoted — a
        // low-sample batch that also moved one tick is both.
        resolution_limited: raw_delta.is_some_and(|d| d <= 1),
        low_sample,
        non_monotonic,
    }
}

/// The draws of a sample sequence, **split per `resetsAt`**.
///
/// ADR-0110 decision 1: *"a batch that crosses a window reset is split per window and the partial
/// window is reported with its own `n`"* — consecutive samples sharing a `resets_at` form one
/// batch whose `n` is its sample count and whose `first`/`last` are its first and last ticks.
#[must_use]
pub fn batch_draws_by_reset(window: Window, samples: &[Sample]) -> Vec<BatchDraw> {
    let mut out: Vec<BatchDraw> = Vec::new();
    for sample in samples {
        match out.last_mut() {
            Some(batch) if batch.resets_at == sample.resets_at => {
                let n = batch.n + 1;
                *batch = batch_draw(window, batch.resets_at, batch.first, sample.ticks, n);
            }
            _ => out.push(batch_draw(
                window,
                sample.resets_at,
                sample.ticks,
                sample.ticks,
                1,
            )),
        }
    }
    out
}

/// Whether a per-item delta may be printed (ADR-0110 decision 1; AC-372).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Attribution {
    /// The item's dispatches were the only traffic between the two readings: the batch is the
    /// item's.
    Attributable(Vec<BatchDraw>),
    /// Other dispatches interleaved: `not attributable`, with the count of the interleaving ones.
    NotAttributable { interleaving: u32 },
}

/// Attribute a sample sequence to one item: `owned[i]` says whether sample `i`'s dispatch belongs
/// to the item. A set in which any sample is not the item's is `NotAttributable` with the count.
///
/// # Panics
/// If `owned` and `samples` differ in length — a caller error, not a reading.
#[must_use]
pub fn attribute(window: Window, samples: &[Sample], owned: &[bool]) -> Attribution {
    assert_eq!(
        samples.len(),
        owned.len(),
        "one ownership flag per sample (the calculator takes an explicitly named dispatch set)"
    );
    let interleaving = u32::try_from(owned.iter().filter(|o| !**o).count()).unwrap_or(u32::MAX);
    if interleaving > 0 {
        return Attribution::NotAttributable { interleaving };
    }
    Attribution::Attributable(batch_draws_by_reset(window, samples))
}
