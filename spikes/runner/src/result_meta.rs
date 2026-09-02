//! The rich fields the `result` message actually carries.
//!
//! Discovered by dumping a retained raw payload, 2026-09-02. Nearly everything
//! this runner had hand-rolled has a better platform-native equivalent here,
//! and three of these fields are direct instruments for experiments the charter
//! had planned to measure indirectly:
//!
//! - `output_tokens_details.thinking_tokens` — E2's effort check becomes a
//!   direct reading rather than an inference from output length.
//! - `subagent_stats` — E3 arm (a) can *prove* an in-session subagent ran, and
//!   detect refusals, instead of assuming dispatch succeeded.
//! - `modelUsage` — per-model cost attribution, which RFC §4.3 predicted would
//!   become free under inversion. It is, and it carries `costBasis` so the
//!   list-price caveat is explicit rather than remembered.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultMeta {
    // --- Completion classification. Structured, unlike the text we were
    // matching on before. `subtype`/`terminal_reason` are how a usage limit
    // should be recognised; string matching was always a guess.
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub stop_reason: Option<String>,
    #[serde(default)]
    pub terminal_reason: Option<String>,
    #[serde(default)]
    pub is_error: Option<bool>,
    #[serde(default)]
    pub api_error_status: Option<serde_json::Value>,
    #[serde(default)]
    pub num_turns: Option<u32>,

    // --- Latency, split by the platform itself. `duration_ms` minus
    // `duration_api_ms` is harness overhead, which is exactly the quantity
    // RFC §3.1 calls "per-run process overhead ... mostly latency".
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub duration_api_ms: Option<u64>,
    #[serde(default)]
    pub ttft_ms: Option<u64>,
    #[serde(default)]
    pub time_to_request_ms: Option<u64>,

    // --- Confounds. Each of these can move between dispatches and change what
    // a comparison means; none is visible unless recorded.
    #[serde(default)]
    pub service_tier: Option<String>,
    #[serde(default)]
    pub speed: Option<String>,
    #[serde(default)]
    pub fast_mode_state: Option<String>,
    #[serde(default)]
    pub inference_geo: Option<String>,

    // --- Instruments.
    /// Per-model attribution: tokens, `costUSD`, `costBasis`, `thinkingTokens`.
    #[serde(rename = "modelUsage", default)]
    pub model_usage: Option<serde_json::Value>,
    /// Subagent taxonomy: spawned / completed / failed / killed / refused.
    #[serde(default)]
    pub subagent_stats: Option<serde_json::Value>,
    #[serde(default)]
    pub permission_denials: Option<serde_json::Value>,
    /// `{ "thinking_tokens": N }` — E2's direct effort instrument.
    ///
    /// Observed at `usage.output_tokens_details`, NOT at the result root. A
    /// root-only read returns null on every dispatch, which would have read as
    /// "effort changes nothing" — the exact conclusion E2 exists to test.
    #[serde(default)]
    pub output_tokens_details: Option<serde_json::Value>,
}

impl ResultMeta {
    /// Harness overhead: total wall time minus time spent in the API.
    pub fn overhead_ms(&self) -> Option<u64> {
        Some(self.duration_ms?.saturating_sub(self.duration_api_ms?))
    }

    /// Reasoning tokens, if reported. E2 asks whether `--effort` actually
    /// applies per dispatch; if this does not separate across effort levels,
    /// it is not being applied.
    pub fn thinking_tokens(&self) -> Option<u64> {
        self.output_tokens_details
            .as_ref()?
            .get("thinking_tokens")?
            .as_u64()
    }

    /// How many subagents the child actually spawned.
    ///
    /// E3 arm (a) dispatches in-session. Without this, "the subagent ran and
    /// disagreed" and "the subagent never spawned" look identical in the
    /// results — the same conflation as RK-B0013, one level up.
    pub fn subagents_spawned(&self) -> Option<u64> {
        self.subagent_stats.as_ref()?.get("spawned")?.as_u64()
    }

    /// Did the platform itself say this run hit a usage limit?
    ///
    /// Preferred over matching on stderr text: there is no documented exit code
    /// or error subtype for exhaustion (charter §6.3), but these structured
    /// fields are far better evidence than a substring, and a false positive
    /// here would mislabel a real failure as exhaustion and void an arm.
    pub fn looks_like_usage_limit(&self) -> bool {
        let hay = [
            self.subtype.as_deref(),
            self.terminal_reason.as_deref(),
            self.stop_reason.as_deref(),
        ];
        hay.iter().flatten().any(|s| {
            let s = s.to_ascii_lowercase();
            s.contains("usage_limit") || s.contains("rate_limit") || s.contains("quota")
        })
    }
}

pub fn parse(raw: &serde_json::Value) -> ResultMeta {
    let payload = raw
        .get("result")
        .filter(|inner| inner.is_object())
        .unwrap_or(raw);
    let mut meta: ResultMeta = serde_json::from_value(payload.clone()).unwrap_or_default();

    // `output_tokens_details` sits under `usage`, not at the root. Lift it so
    // callers have one place to look.
    if meta.output_tokens_details.is_none() {
        meta.output_tokens_details = payload
            .get("usage")
            .and_then(|u| u.get("output_tokens_details"))
            .cloned();
    }
    meta
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observed() -> serde_json::Value {
        // Trimmed from a real v2.1.258 result message, 2026-09-02.
        serde_json::json!({
            "type": "result", "subtype": "success",
            "stop_reason": "end_turn", "terminal_reason": "completed",
            "is_error": false, "api_error_status": null, "num_turns": 1,
            "duration_ms": 1371, "duration_api_ms": 1063,
            "ttft_ms": 1197, "time_to_request_ms": 161,
            "service_tier": "standard", "speed": "standard",
            "fast_mode_state": "off", "inference_geo": "not_available",
            "usage": { "output_tokens_details": { "thinking_tokens": 35 } },
            "output_tokens_details": { "thinking_tokens": 35 },
            "subagent_stats": { "spawned": 0, "completed": 0, "failed": 0 },
            "modelUsage": { "claude-haiku-4-5-20251001": {
                "costUSD": 0.0027461, "costBasis": "list", "outputTokens": 42 } },
            "permission_denials": []
        })
    }

    #[test]
    fn parses_the_observed_shape() {
        let m = parse(&observed());
        assert_eq!(m.subtype.as_deref(), Some("success"));
        assert_eq!(m.terminal_reason.as_deref(), Some("completed"));
        assert_eq!(m.num_turns, Some(1));
        assert_eq!(m.service_tier.as_deref(), Some("standard"));
        assert_eq!(m.fast_mode_state.as_deref(), Some("off"));
    }

    #[test]
    fn overhead_is_wall_time_minus_api_time() {
        // The RFC predicted per-run process overhead would be "worse, mostly
        // latency". Here it is 308 ms of a 1371 ms dispatch.
        assert_eq!(parse(&observed()).overhead_ms(), Some(308));
    }

    #[test]
    fn thinking_tokens_are_readable_for_the_effort_check() {
        assert_eq!(parse(&observed()).thinking_tokens(), Some(35));
    }

    #[test]
    fn subagent_spawn_count_separates_ran_from_never_spawned() {
        assert_eq!(parse(&observed()).subagents_spawned(), Some(0));
    }

    #[test]
    fn a_successful_run_is_not_mistaken_for_a_usage_limit() {
        assert!(!parse(&observed()).looks_like_usage_limit());
    }

    #[test]
    fn a_structured_limit_signal_is_recognised() {
        let raw = serde_json::json!({
            "subtype": "error", "terminal_reason": "usage_limit_reached"
        });
        assert!(parse(&raw).looks_like_usage_limit());
    }

    #[test]
    fn missing_timings_yield_none_rather_than_zero() {
        // A zero would read as "no overhead", which is a measurement.
        assert_eq!(ResultMeta::default().overhead_ms(), None);
    }

    #[test]
    fn thinking_tokens_are_found_under_usage_not_just_the_root() {
        // The shape actually observed on v2.1.258. Reading only the root
        // returns null on every dispatch and would read as "effort changes
        // nothing" — precisely the conclusion E2 exists to test.
        let raw = serde_json::json!({
            "type": "result",
            "usage": { "output_tokens_details": { "thinking_tokens": 37 } }
        });
        assert_eq!(parse(&raw).thinking_tokens(), Some(37));
    }

    #[test]
    fn a_root_level_details_block_still_wins() {
        let raw = serde_json::json!({
            "output_tokens_details": { "thinking_tokens": 5 },
            "usage": { "output_tokens_details": { "thinking_tokens": 99 } }
        });
        assert_eq!(parse(&raw).thinking_tokens(), Some(5));
    }
}
