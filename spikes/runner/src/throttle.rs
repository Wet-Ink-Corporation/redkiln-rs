//! Headroom-based dispatch gating.
//!
//! Charter §12: "The runner reads `rate_limits` before and after every step and
//! **refuses to dispatch** below a configured margin." That was written on the
//! assumption headroom was unreadable outside a statusline script. It is
//! readable (FINDINGS F1), so the refusal is real rather than aspirational.
//!
//! Two properties of the instrument shape this module, and both are measured
//! rather than assumed:
//!
//! - **`utilization` is quantised to 0.01.** A single dispatch does not move
//!   it. So the guard is a *ceiling* check, never a "how much is one more step
//!   going to cost" prediction, and per-run cost is only ever reported as a
//!   batch delta divided by n.
//! - **There is no Opus or Sonnet family key.** `unifiedWindows` carries
//!   `five_hour` and `seven_day` only. The sub-limit that actually binds our
//!   Opus-heavy arms is invisible, so this guard cannot prevent an Opus
//!   interruption — it can only keep us from walking into the shared windows.
//!   Detection of that case is `StepError::UsageLimit`, not this.

use crate::event::Headroom;

/// The margins an experiment declares before it runs.
#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub max_five_hour: f64,
    pub max_seven_day: f64,
}

impl Default for Margins {
    fn default() -> Self {
        // Two ticks of the instrument's 0.01 resolution below full, so the
        // guard trips while there is still measurable room to stop cleanly.
        Self {
            max_five_hour: 0.90,
            max_seven_day: 0.90,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    /// Dispatch. Carries no promise about what the step will cost.
    Proceed,
    /// A declared margin is exceeded. The experiment pauses; it does not fail.
    Halt { window: &'static str, at: f64, margin: f64 },
    /// Charter §6.2: crossing into usage credits silently drops the cache TTL
    /// from 1h to 5m, so records either side of the crossing are not
    /// comparable. Halt rather than quietly averaging them together.
    HaltOverage,
}

impl Verdict {
    pub fn is_halt(&self) -> bool {
        !matches!(self, Verdict::Proceed)
    }

    pub fn reason(&self) -> String {
        match self {
            Verdict::Proceed => "proceed".into(),
            Verdict::Halt { window, at, margin } => {
                format!("{window} utilisation {at:.2} is at or past the declared margin {margin:.2}")
            }
            Verdict::HaltOverage => {
                "run crossed into usage credits — cache TTL drops 1h to 5m, so records \
                 either side are not comparable (charter §6.2)"
                    .into()
            }
        }
    }
}

/// Tracks the last headroom reading and decides whether to dispatch again.
///
/// The first dispatch of a batch is necessarily unguarded: headroom is only
/// learned *from* a dispatch. That is acceptable because one step cannot move a
/// quantised figure, but it is the reason a batch must never be started close
/// to a margin in the first place.
#[derive(Debug, Default)]
pub struct Guard {
    last: Option<Headroom>,
}

impl Guard {
    pub fn observe(&mut self, headroom: Option<&Headroom>) {
        if let Some(h) = headroom {
            self.last = Some(h.clone());
        }
    }

    pub fn check(&self, margins: Margins) -> Verdict {
        let Some(h) = &self.last else {
            // Nothing observed yet — the opening dispatch.
            return Verdict::Proceed;
        };

        if h.is_using_overage == Some(true) {
            return Verdict::HaltOverage;
        }

        if let Some(at) = h.five_hour_utilization
            && at >= margins.max_five_hour
        {
            return Verdict::Halt {
                window: "five_hour",
                at,
                margin: margins.max_five_hour,
            };
        }

        if let Some(at) = h.seven_day_utilization
            && at >= margins.max_seven_day
        {
            return Verdict::Halt {
                window: "seven_day",
                at,
                margin: margins.max_seven_day,
            };
        }

        Verdict::Proceed
    }

    pub fn last(&self) -> Option<&Headroom> {
        self.last.as_ref()
    }
}

/// Per-dispatch draw, reported honestly.
///
/// `utilization` is quantised to 0.01, so a per-run figure is meaningless and
/// this deliberately reports the batch. E0's output is this, not a per-step
/// number the instrument cannot produce.
#[derive(Debug, Clone, serde::Serialize)]
pub struct BatchDraw {
    pub window: &'static str,
    pub first: f64,
    pub last: f64,
    pub delta: f64,
    pub dispatches: u32,
    /// `delta / dispatches`. Meaningful only when `delta` spans several ticks;
    /// `resolution_limited` says whether it does.
    pub mean_per_dispatch: f64,
    /// True when the whole batch moved the figure by less than two ticks — in
    /// which case the mean is an artefact of the instrument, not a measurement.
    pub resolution_limited: bool,
}

pub fn batch_draw(window: &'static str, first: f64, last: f64, dispatches: u32) -> BatchDraw {
    let delta = last - first;
    let n = dispatches.max(1);
    BatchDraw {
        window,
        first,
        last,
        delta,
        dispatches: n,
        mean_per_dispatch: delta / f64::from(n),
        resolution_limited: delta < 0.02,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headroom(five: f64, seven: f64, overage: bool) -> Headroom {
        Headroom {
            five_hour_utilization: Some(five),
            seven_day_utilization: Some(seven),
            five_hour_resets_at: None,
            seven_day_resets_at: None,
            is_using_overage: Some(overage),
            overage_status: None,
            status: None,
            rate_limit_type: None,
        }
    }

    #[test]
    fn the_opening_dispatch_is_unguarded() {
        assert_eq!(Guard::default().check(Margins::default()), Verdict::Proceed);
    }

    #[test]
    fn halts_on_either_window() {
        let mut g = Guard::default();
        g.observe(Some(&headroom(0.95, 0.10, false)));
        assert!(matches!(
            g.check(Margins::default()),
            Verdict::Halt { window: "five_hour", .. }
        ));

        let mut g = Guard::default();
        g.observe(Some(&headroom(0.10, 0.95, false)));
        assert!(matches!(
            g.check(Margins::default()),
            Verdict::Halt { window: "seven_day", .. }
        ));
    }

    #[test]
    fn overage_halts_even_with_ample_headroom() {
        // Crossing into credits changes the cache TTL, which changes what the
        // measurement means. Plenty of room left is not a reason to continue.
        let mut g = Guard::default();
        g.observe(Some(&headroom(0.01, 0.01, true)));
        assert_eq!(g.check(Margins::default()), Verdict::HaltOverage);
    }

    #[test]
    fn proceeds_with_room() {
        let mut g = Guard::default();
        g.observe(Some(&headroom(0.04, 0.04, false)));
        assert_eq!(g.check(Margins::default()), Verdict::Proceed);
    }

    #[test]
    fn a_batch_that_moved_less_than_two_ticks_is_flagged_resolution_limited() {
        // This is the observed smoke case: 0.04 -> 0.04 over 2 dispatches. The
        // mean is 0.0, which is not "free" — it is "below the instrument".
        let d = batch_draw("seven_day", 0.04, 0.04, 2);
        assert!(d.resolution_limited);
        assert_eq!(d.mean_per_dispatch, 0.0);
    }

    #[test]
    fn a_batch_spanning_several_ticks_reports_a_usable_mean() {
        let d = batch_draw("seven_day", 0.10, 0.20, 20);
        assert!(!d.resolution_limited);
        assert!((d.mean_per_dispatch - 0.005).abs() < 1e-9);
    }
}
