//! The `--output-format stream-json` event stream.
//!
//! RFC §10 lists the exact `stream-json` schema as something it could not
//! establish, and the doc sweep confirmed it: the result message's usage block
//! is documented under two different names for the same cache fields
//! (`ephemeral_cache_*` in the SDK reference, `ephemeral_1h/5m_*` in the costs
//! page). So this module types what is known and **retains the raw JSON of
//! every line regardless**. An unrecognised event is a finding about the
//! platform, not a parse error to swallow.

use serde::{Deserialize, Serialize};

/// One line of the NDJSON stream: a typed view plus the bytes it came from.
#[derive(Debug, Clone, Serialize)]
pub struct StreamLine {
    pub kind: EventKind,
    /// Retained verbatim. Charter §7.1: "unknown events logged, never dropped".
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum EventKind {
    SystemInit,
    /// `system` with a subtype we do not model — carries the subtype so a new
    /// platform event shows up in the results as itself, not as "unknown".
    System(String),
    ApiRetry,
    /// Undocumented, and the best headroom instrument we have. Observed on
    /// v2.1.258; see `RateLimitEvent`.
    RateLimitEvent,
    Assistant,
    User,
    Result,
    /// A `type` we have never seen. Distinct from `System(_)`: this is a new
    /// top-level shape, which is the one that breaks a parser.
    Unknown(String),
    /// A line that was not JSON at all.
    Malformed,
}

/// Token accounting from the final `result` message.
///
/// Every field is optional because the docs mark most of them optional and the
/// two cache namings are mutually exclusive in practice. Readers pick whichever
/// arrived rather than assuming a naming.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    #[serde(default)]
    pub input_tokens: Option<u64>,
    #[serde(default)]
    pub output_tokens: Option<u64>,
    #[serde(default)]
    pub cache_read_input_tokens: Option<u64>,
    #[serde(default)]
    pub cache_creation_input_tokens: Option<u64>,

    // The SDK-reference naming.
    #[serde(default)]
    pub ephemeral_cache_read_input_tokens: Option<u64>,
    #[serde(default)]
    pub ephemeral_cache_creation_input_tokens: Option<u64>,

    /// The TTL split, which the platform docs place in a **nested**
    /// `usage.cache_creation` object — not as flat `ephemeral_*` siblings.
    ///
    /// Getting this wrong is how an instrument reports "no TTL data" forever
    /// while the data is right there. It is also only documented to appear when
    /// a 1h cache is actually in play, so its *absence* is itself evidence that
    /// the dispatch ran entirely on the 5-minute tier.
    #[serde(default)]
    pub cache_creation: Option<CacheCreation>,
}

/// `usage.cache_creation` — cache writes split by TTL tier.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheCreation {
    #[serde(default)]
    pub ephemeral_5m_input_tokens: Option<u64>,
    #[serde(default)]
    pub ephemeral_1h_input_tokens: Option<u64>,
}

impl Usage {
    /// Cache-read share — E1's primary metric.
    ///
    /// Returns `None` rather than 0.0 when neither field arrived: "no cache
    /// data" and "a cold write every time" are different findings, and E1's
    /// kill criterion turns on telling them apart.
    pub fn cache_read_share(&self) -> Option<f64> {
        let read = self
            .cache_read_input_tokens
            .or(self.ephemeral_cache_read_input_tokens)?;
        let created = self
            .cache_creation_input_tokens
            .or(self.ephemeral_cache_creation_input_tokens)
            .unwrap_or(0);
        let denom = read + created;
        if denom == 0 {
            return None;
        }
        Some(read as f64 / denom as f64)
    }

    /// Which TTL tier the cache writes landed in, if the split was reported.
    ///
    /// Returns `(five_min, one_hour)`. The platform default is a 5-minute TTL
    /// and a 1h tier must be requested explicitly, so this is the field that
    /// decides whether the RFC's headline asymmetry — `-p` on 1h versus
    /// in-session subagents forced to 5m — actually exists for us.
    pub fn ttl_split(&self) -> Option<(u64, u64)> {
        let c = self.cache_creation.as_ref()?;
        Some((
            c.ephemeral_5m_input_tokens.unwrap_or(0),
            c.ephemeral_1h_input_tokens.unwrap_or(0),
        ))
    }

    /// Total input actually processed.
    ///
    /// The docs are explicit that `input_tokens` counts only tokens AFTER the
    /// last cache breakpoint, so it is not the denominator anyone expects.
    /// E1's headline metric stays the charter's `read / (read + creation)`;
    /// this is recorded alongside so the other denominator can be computed
    /// later without re-running anything.
    pub fn total_input(&self) -> Option<u64> {
        let read = self.cache_read_input_tokens.or(self.ephemeral_cache_read_input_tokens);
        let created = self
            .cache_creation_input_tokens
            .or(self.ephemeral_cache_creation_input_tokens);
        match (self.input_tokens, read, created) {
            (None, None, None) => None,
            (i, r, c) => Some(i.unwrap_or(0) + r.unwrap_or(0) + c.unwrap_or(0)),
        }
    }
}

/// Plan headroom, as reported in the result message.
///
/// Whether these request/token counters correspond to the plan's five-hour and
/// seven-day windows is unverified — E0 checks it. Until then they are recorded
/// but never used to gate a dispatch.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(default)]
    pub requests_limit: Option<u64>,
    #[serde(default)]
    pub requests_remaining: Option<u64>,
    #[serde(default)]
    pub requests_reset_unix_seconds: Option<i64>,
    #[serde(default)]
    pub tokens_limit: Option<u64>,
    #[serde(default)]
    pub tokens_remaining: Option<u64>,
    #[serde(default)]
    pub tokens_reset_unix_seconds: Option<i64>,
}

/// A `system`/`api_retry` event. The harness consumes these (charter §6.3) so a
/// throttled step is distinguishable from a slow one.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiRetry {
    #[serde(default)]
    pub attempt: Option<u32>,
    #[serde(default)]
    pub max_retries: Option<u32>,
    #[serde(default)]
    pub retry_delay_ms: Option<u64>,
    #[serde(default)]
    pub error_status: Option<i64>,
    #[serde(default)]
    pub error: Option<String>,
}

/// Classify one NDJSON line. This never fails: a line we cannot parse is a
/// recorded observation, not an error.
pub fn classify(line: &str) -> StreamLine {
    let value: serde_json::Value = match serde_json::from_str(line) {
        Ok(v) => v,
        Err(_) => {
            return StreamLine {
                kind: EventKind::Malformed,
                raw: serde_json::Value::String(line.to_string()),
            };
        }
    };

    let ty = value.get("type").and_then(|v| v.as_str()).unwrap_or("");
    let subtype = value.get("subtype").and_then(|v| v.as_str()).unwrap_or("");

    let kind = match (ty, subtype) {
        ("system", "init") => EventKind::SystemInit,
        ("system", "api_retry") => EventKind::ApiRetry,
        ("system", other) => EventKind::System(other.to_string()),
        ("rate_limit_event", _) => EventKind::RateLimitEvent,
        ("assistant", _) => EventKind::Assistant,
        ("user", _) => EventKind::User,
        ("result", _) => EventKind::Result,
        (other, _) => EventKind::Unknown(other.to_string()),
    };

    StreamLine { kind, raw: value }
}

/// The result message nests its payload one level deep in the documented
/// shape (`result.result.usage`) but flat in observed older streams. Resolve
/// once here so every reader agrees.
fn payload(raw: &serde_json::Value) -> &serde_json::Value {
    raw.get("result")
        .filter(|inner| inner.is_object())
        .unwrap_or(raw)
}

pub fn usage_from_result(raw: &serde_json::Value) -> Option<Usage> {
    serde_json::from_value(payload(raw).get("usage")?.clone()).ok()
}

pub fn rate_limit_from_result(raw: &serde_json::Value) -> Option<RateLimit> {
    let holder = payload(raw);
    let rl = holder
        .get("rate_limit")
        .or_else(|| holder.get("rate_limits"))?;
    serde_json::from_value(rl.clone()).ok()
}

pub fn total_cost_usd(raw: &serde_json::Value) -> Option<f64> {
    payload(raw).get("total_cost_usd").and_then(|v| v.as_f64())
}

pub fn api_retry_from(raw: &serde_json::Value) -> ApiRetry {
    serde_json::from_value(raw.clone()).unwrap_or_default()
}


/// A `rate_limit_event` line.
///
/// **Undocumented.** Charter §6.1 states that the programmatic headroom read is
/// the statusline `rate_limits` object and that it is unavailable outside a
/// statusline script; the doc sweep further suggested `result.rate_limit`. On
/// v2.1.258 neither is true for `-p`: `result.rate_limit` is absent, and this
/// event carries the numbers instead — including the `seven_day` utilisation
/// that E0 exists to measure, and an explicit overage flag.
///
/// Because it is undocumented it may change without notice (RFC R8). The raw
/// line is retained on every record so a schema change is recoverable after the
/// fact rather than silently losing the series.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Headroom {
    pub five_hour_utilization: Option<f64>,
    pub seven_day_utilization: Option<f64>,
    pub five_hour_resets_at: Option<i64>,
    pub seven_day_resets_at: Option<i64>,
    /// Charter §6.2: crossing into usage credits silently drops the prompt-cache
    /// TTL from 1h to 5m, so a run that crossed over is not comparable with one
    /// that did not. Now detectable per dispatch rather than assumed.
    pub is_using_overage: Option<bool>,
    pub overage_status: Option<String>,
    pub status: Option<String>,
    /// Which window the platform itself considers closest to binding. Useful
    /// because it is the platform's own view, not our inference from two
    /// quantised numbers that currently read identically.
    pub rate_limit_type: Option<String>,
}

pub fn headroom_from_event(raw: &serde_json::Value) -> Option<Headroom> {
    let info = raw.get("rate_limit_info")?;
    let windows = info.get("unifiedWindows");
    let window = |name: &str, field: &str| -> Option<f64> {
        windows?.get(name)?.get(field)?.as_f64()
    };
    Some(Headroom {
        five_hour_utilization: window("five_hour", "utilization"),
        seven_day_utilization: window("seven_day", "utilization"),
        five_hour_resets_at: window("five_hour", "resetsAt").map(|v| v as i64),
        seven_day_resets_at: window("seven_day", "resetsAt").map(|v| v as i64),
        is_using_overage: info.get("isUsingOverage").and_then(|v| v.as_bool()),
        overage_status: info
            .get("overageStatus")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        status: info.get("status").and_then(|v| v.as_str()).map(str::to_string),
        rate_limit_type: info
            .get("rateLimitType")
            .and_then(|v| v.as_str())
            .map(str::to_string),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_the_documented_types() {
        let cases = [
            (r#"{"type":"system","subtype":"init"}"#, EventKind::SystemInit),
            (
                r#"{"type":"system","subtype":"api_retry","attempt":1}"#,
                EventKind::ApiRetry,
            ),
            (r#"{"type":"result"}"#, EventKind::Result),
            (r#"{"type":"assistant"}"#, EventKind::Assistant),
            (r#"{"type":"user"}"#, EventKind::User),
        ];
        for (line, want) in cases {
            assert_eq!(classify(line).kind, want, "line: {line}");
        }
    }

    #[test]
    fn a_new_system_subtype_names_itself() {
        assert_eq!(
            classify(r#"{"type":"system","subtype":"brand_new"}"#).kind,
            EventKind::System("brand_new".into())
        );
    }

    #[test]
    fn a_new_top_level_type_is_unknown_but_retained() {
        let line = classify(r#"{"type":"telemetry","payload":{"a":1}}"#);
        assert_eq!(line.kind, EventKind::Unknown("telemetry".into()));
        assert_eq!(line.raw["payload"]["a"], 1);
    }

    #[test]
    fn malformed_json_is_retained_verbatim_not_dropped() {
        let line = classify("this is not json");
        assert_eq!(line.kind, EventKind::Malformed);
        assert_eq!(
            line.raw,
            serde_json::Value::String("this is not json".into())
        );
    }

    #[test]
    fn usage_parses_from_both_nestings() {
        let nested = serde_json::json!({
            "type": "result",
            "result": { "usage": { "input_tokens": 10, "output_tokens": 5 } }
        });
        let flat = serde_json::json!({
            "type": "result",
            "usage": { "input_tokens": 10, "output_tokens": 5 }
        });
        for v in [nested, flat] {
            let u = usage_from_result(&v).expect("usage should parse");
            assert_eq!(u.input_tokens, Some(10));
            assert_eq!(u.output_tokens, Some(5));
        }
    }

    #[test]
    fn cache_share_distinguishes_absent_from_cold() {
        // Absent: no cache fields at all.
        assert_eq!(Usage::default().cache_read_share(), None);

        // Cold: a creation write and no reads. This must be 0.0, not None — it
        // is E1's H0, and reporting it as "no data" would hide the result.
        let cold = Usage {
            cache_creation_input_tokens: Some(4000),
            cache_read_input_tokens: Some(0),
            ..Default::default()
        };
        assert_eq!(cold.cache_read_share(), Some(0.0));

        let warm = Usage {
            cache_read_input_tokens: Some(9000),
            cache_creation_input_tokens: Some(1000),
            ..Default::default()
        };
        assert_eq!(warm.cache_read_share(), Some(0.9));
    }

    #[test]
    fn cache_share_falls_back_to_the_ephemeral_naming() {
        let u = Usage {
            ephemeral_cache_read_input_tokens: Some(3),
            ephemeral_cache_creation_input_tokens: Some(1),
            ..Default::default()
        };
        assert_eq!(u.cache_read_share(), Some(0.75));
    }

    #[test]
    fn rate_limit_accepts_either_key() {
        let a = serde_json::json!({"rate_limit": {"requests_remaining": 7}});
        let b = serde_json::json!({"rate_limits": {"requests_remaining": 7}});
        for v in [a, b] {
            assert_eq!(
                rate_limit_from_result(&v).unwrap().requests_remaining,
                Some(7)
            );
        }
    }

    #[test]
    fn rate_limit_event_is_recognised_and_parsed() {
        let line = r#"{"type":"rate_limit_event","rate_limit_info":{
            "isUsingOverage":false,"overageStatus":"rejected","status":"allowed",
            "unifiedWindows":{
                "five_hour":{"resetsAt":1788370800,"utilization":0.04},
                "seven_day":{"resetsAt":1788408000,"utilization":0.11}}}}"#;
        let parsed = classify(line);
        assert_eq!(parsed.kind, EventKind::RateLimitEvent);

        let h = headroom_from_event(&parsed.raw).expect("headroom");
        assert_eq!(h.five_hour_utilization, Some(0.04));
        assert_eq!(h.seven_day_utilization, Some(0.11));
        assert_eq!(h.seven_day_resets_at, Some(1788408000));
        assert_eq!(h.is_using_overage, Some(false));
        assert_eq!(h.status.as_deref(), Some("allowed"));
        assert_eq!(h.rate_limit_type, None);
    }

    #[test]
    fn headroom_tolerates_a_missing_windows_block() {
        // The event is undocumented; a shape change must degrade to None
        // rather than panic or silently report zero utilisation.
        let raw = serde_json::json!({"rate_limit_info": {"isUsingOverage": true}});
        let h = headroom_from_event(&raw).expect("still parses");
        assert_eq!(h.seven_day_utilization, None);
        assert_eq!(h.is_using_overage, Some(true));
    }

    #[test]
    fn ttl_split_reads_the_nested_cache_creation_object() {
        // The shape the platform docs actually document. Parsing this as flat
        // `ephemeral_*` siblings — as this module originally did — reports "no
        // TTL data" forever while the data sits one level down.
        let raw = serde_json::json!({
            "type": "result",
            "result": { "usage": {
                "input_tokens": 2048,
                "cache_read_input_tokens": 1800,
                "cache_creation_input_tokens": 248,
                "cache_creation": {
                    "ephemeral_5m_input_tokens": 148,
                    "ephemeral_1h_input_tokens": 100
                }
            }}
        });
        let u = usage_from_result(&raw).expect("usage");
        assert_eq!(u.ttl_split(), Some((148, 100)));
        assert_eq!(u.total_input(), Some(2048 + 1800 + 248));
    }

    #[test]
    fn an_absent_split_is_none_not_zero() {
        // Absence is evidence the dispatch ran wholly on the 5m tier. It must
        // not be reported as "0 tokens on 1h", which reads as a measurement.
        let u = Usage {
            cache_creation_input_tokens: Some(3611),
            ..Default::default()
        };
        assert_eq!(u.ttl_split(), None);
    }
}
