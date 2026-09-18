//! The `--output-format stream-json` line classifier — one NDJSON line in, a typed view plus the
//! bytes it came from out; nothing refused (`s7c` §7c.3; AC-200, AC-201).
//!
//! **Lifted, with edits, from `runner/src/event.rs` in `redkiln-corpus` at
//! `61f590e51cbb00301c310e46aab615563a7787b8`** (the SHA in `CORPUS_PIN`; blob
//! `5ff3e4ae91a2f0ab99b08cfb3adcfab47880c27b`, 505 lines) — the first of that file's two lift
//! destinations; the second is `crates/rk-types/src/usage.rs` (`Usage`, `CacheCreation` —
//! §7c.2's lift table). That file was the spike runner's parse of the CLI stream, **built and run
//! over the programme's recorded dispatches and exercised, not validated against an oracle** —
//! RFC §10 listed the exact `stream-json` schema as something it could not establish, and the doc
//! sweep confirmed it — so what travels here is the shape the CLI emitted at the pin, not a warrant
//! (PIA-036, PIA-062). The runner's own module note is the rule this module keeps: *"types what is
//! known and retains the raw JSON of every line regardless. An unrecognised event is a finding about
//! the platform, not a parse error to swallow."*
//!
//! **What is lifted** (the table's `event.rs` row): [`StreamLine`]'s shape (raw retained on every
//! line); [`EventKind`] and [`classify`] (never fails; non-JSON → [`EventKind::Malformed`]) with
//! **no `deny_unknown_fields`** anywhere in this module (AC-201 — the block bans the token over the
//! stream decoder's files); [`ToolCall`] and [`tool_calls_from_assistant`]; [`ApiRetry`] and
//! [`api_retry_from`]; [`usage_from_result`] over the two nestings; the tests
//! `classifies_the_documented_types`, `a_new_system_subtype_names_itself`,
//! `a_new_top_level_type_is_unknown_but_retained` and `malformed_json_is_retained_verbatim_not_dropped`
//! (in `tests/it/event.rs`, edited to the `String` raw).
//!
//! **Edits, each forced by a rule the corpus version predates:**
//!
//! 1. **`StreamLine.raw: serde_json::Value` → `raw: String`, the line's bytes** (§7c.2). At the pin
//!    `raw` was a parsed `Value` (`event.rs`:18; `runner/Cargo.toml` enabled no `preserve_order`),
//!    so what the runner retained was a re-serialisation with sorted keys and `f64` numbers; lifting
//!    it as-is would leave AC-200's *byte-for-byte* and AC-374's digit scanner *"over
//!    `StreamLine.raw`"* with no bytes to read. The typed `Value` is built once inside [`classify`]
//!    to read `type` and `subtype`, and discarded; every later reader — the headroom scanner
//!    (`headroom.rs`), the cost scanner, the `system/init` parse (`init.rs`), the result frame —
//!    reads `raw` itself.
//! 2. **`Usage` and `CacheCreation` moved to `rk-types`** (ADR-0126 decision 2) and lost `Default`;
//!    [`usage_from_result`] returns `rk_types::usage::Usage`. `cache_read_share`, `ttl_split` and
//!    `total_input` propagate absence there (no unwrap-or-zero — AC-362).
//! 3. **[`ApiRetry`] loses `Default`** (RT-8) and [`api_retry_from`] loses its
//!    unwrap-or-default call: a line that classified as `system/api_retry` but does not parse as one
//!    is `None`, and the caller counts the EVENT (AC-232 counts events, not parsed fields — the
//!    count is taken from the classification, so a `None` here never loses a charge).
//! 4. **The headroom reading is split out** into `headroom.rs` (§7c.8): the runner's
//!    `Headroom { five_hour_utilization: Option<f64>, … }` and `headroom_from_event` (an `as_f64`
//!    read) are **not lifted** — utilisation is `u16` ticks from the decimal digits (AC-374), read
//!    by a structural scanner over `raw`, and the block's AC-374 ban reaches this module too, so no
//!    float type or float accessor is named here.
//!
//! **NOT lifted** (the table's last column): **`rate_limit_from_result`** and the `RateLimit`
//! struct — a headroom source reading `result.rate_limit`, which AC-219 and AC-380 forbid (the
//! block bans the token); **`total_cost_usd` as an `f64`** — the cost is `CostUsd(String)`, the
//! digits verbatim, read by the scanner in `headroom.rs` (RT-19 as amended). The runner's
//! `payload()` resolver (the result message *"nests its payload one level deep in the documented
//! shape (`result.result.usage`) but flat in observed older streams"*) is kept as [`payload`], so
//! every reader agrees.

use serde::Deserialize;

use rk_types::usage::Usage;

/// One line of the NDJSON stream: a typed view plus the bytes it came from.
///
/// `raw` is the line **as read**, minus its terminating newline (and a `\r` before it) — the bytes
/// every later reader scans (AC-200; AC-374's *"over `StreamLine.raw`"*). The retained `stdout` on
/// the record keeps the terminators too (`runner.rs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamLine {
    pub kind: EventKind,
    /// Retained verbatim. Charter §7.1: *"unknown events logged, never dropped"* (lifted).
    pub raw: String,
}

/// What a line is, by its `type` / `subtype` (lifted).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    SystemInit,
    /// `system` with a subtype we do not model — carries the subtype so a new platform event shows
    /// up as itself, not as "unknown" (lifted; F4's `system/thinking_tokens` is one).
    System(String),
    ApiRetry,
    /// Undocumented, and the best headroom instrument we have (lifted; F1). Read by `headroom.rs`.
    RateLimitEvent,
    Assistant,
    User,
    Result,
    /// A `type` we have never seen. Distinct from `System(_)`: this is a new top-level shape, which
    /// is the one that breaks a parser (lifted).
    Unknown(String),
    /// A line that was not JSON at all (lifted).
    Malformed,
}

impl EventKind {
    /// The census key the runner used (`system/init`, `unknown/<type>`, …) — kept for the record's
    /// readers and the tests; the adapter keeps no census on the row (ADR-0126 names none).
    #[must_use]
    pub fn census_key(&self) -> String {
        match self {
            Self::SystemInit => "system/init".to_owned(),
            Self::System(s) => format!("system/{s}"),
            Self::ApiRetry => "system/api_retry".to_owned(),
            Self::Assistant => "assistant".to_owned(),
            Self::User => "user".to_owned(),
            Self::Result => "result".to_owned(),
            Self::RateLimitEvent => "rate_limit_event".to_owned(),
            Self::Unknown(t) => format!("unknown/{t}"),
            Self::Malformed => "malformed".to_owned(),
        }
    }
}

/// One tool call seen on the stream (lifted).
///
/// The attribution field is `parent_tool_use_id`: a call made by the PARENT carries `null`, and a
/// call made by a subagent carries the id of the `Agent` tool_use that spawned it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCall {
    pub name: String,
    /// `None` = the parent's own call.
    pub parent_tool_use_id: Option<String>,
    /// Present on `Agent`/`Task` calls: which card was dispatched.
    pub subagent_type: Option<String>,
}

/// Extract every `tool_use` block from an `assistant` line (lifted; reads `raw`).
#[must_use]
pub fn tool_calls_from_assistant(raw: &str) -> Vec<ToolCall> {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) else {
        return Vec::new();
    };
    let parent = value
        .get("parent_tool_use_id")
        .and_then(|v| v.as_str())
        .map(str::to_owned);
    let Some(content) = value
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_array())
    else {
        return Vec::new();
    };
    content
        .iter()
        .filter(|c| c.get("type").and_then(|t| t.as_str()) == Some("tool_use"))
        .filter_map(|c| {
            Some(ToolCall {
                name: c.get("name")?.as_str()?.to_owned(),
                parent_tool_use_id: parent.clone(),
                subagent_type: c
                    .get("input")
                    .and_then(|i| i.get("subagent_type"))
                    .and_then(|v| v.as_str())
                    .map(str::to_owned),
            })
        })
        .collect()
}

/// A `system`/`api_retry` line's fields (lifted; **no `Default`** — RT-8).
///
/// The harness consumes these (charter §6.3) so a throttled step is distinguishable from a slow
/// one; the adapter counts the EVENTS on the row (AC-232) and issues no retry of its own.
///
/// No `deny_unknown_fields` (AC-201): a future field on the line is ignored, never refused.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ApiRetry {
    pub attempt: Option<u32>,
    pub max_retries: Option<u32>,
    pub retry_delay_ms: Option<u64>,
    pub error_status: Option<i64>,
    pub error: Option<String>,
}

/// Parse an `api_retry` line's fields; `None` when the line does not parse as one.
///
/// The runner's unwrap-or-default call is not lifted — an absent field is absent, and the charge
/// is counted from the classification, not from this.
#[must_use]
pub fn api_retry_from(raw: &str) -> Option<ApiRetry> {
    serde_json::from_str(raw).ok()
}

/// Classify one NDJSON line. This never fails: a line we cannot parse is a recorded observation,
/// not an error (lifted). The bytes are retained on every branch.
#[must_use]
pub fn classify(line: &str) -> StreamLine {
    let raw = line.to_owned();
    let Ok(value) = serde_json::from_str::<serde_json::Value>(line) else {
        return StreamLine {
            kind: EventKind::Malformed,
            raw,
        };
    };
    let ty = value.get("type").and_then(|v| v.as_str());
    let subtype = value.get("subtype").and_then(|v| v.as_str());
    let kind = match (ty, subtype) {
        (Some("system"), Some("init")) => EventKind::SystemInit,
        (Some("system"), Some("api_retry")) => EventKind::ApiRetry,
        (Some("system"), other) => EventKind::System(other.map_or_else(String::new, str::to_owned)),
        (Some("rate_limit_event"), _) => EventKind::RateLimitEvent,
        (Some("assistant"), _) => EventKind::Assistant,
        (Some("user"), _) => EventKind::User,
        (Some("result"), _) => EventKind::Result,
        (other, _) => EventKind::Unknown(other.map_or_else(String::new, str::to_owned)),
    };
    StreamLine { kind, raw }
}

/// The result message nests its payload one level deep in the documented shape
/// (`result.result.usage`) but flat in observed older streams. Resolve once here so every reader
/// agrees (lifted).
#[must_use]
pub fn payload(raw: &serde_json::Value) -> &serde_json::Value {
    raw.get("result")
        .filter(|inner| inner.is_object())
        .unwrap_or(raw)
}

/// The `usage` block of a result line as `rk_types::usage::Usage`.
///
/// Every counter `Option`, the nested `cache_creation` object included, unknown members ignored
/// (AC-201; AC-362). `None` when the line has no `usage` object or it does not parse.
#[must_use]
pub fn usage_from_result(raw: &str) -> Option<Usage> {
    let value: serde_json::Value = serde_json::from_str(raw).ok()?;
    serde_json::from_value(payload(&value).get("usage")?.clone()).ok()
}
