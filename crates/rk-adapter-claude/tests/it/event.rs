//! **The line classifier** — the four tests lifted from `runner/src/event.rs` at the pin (§7c.2's
//! lift table), edited to the `String` raw: `classifies_the_documented_types`,
//! `a_new_system_subtype_names_itself`, `a_new_top_level_type_is_unknown_but_retained`,
//! `malformed_json_is_retained_verbatim_not_dropped`; and the two the lift table names for
//! `usage_from_result` over the two nestings.

use rk_adapter_claude::event::{
    EventKind, api_retry_from, classify, tool_calls_from_assistant, usage_from_result,
};

#[test]
fn classifies_the_documented_types() {
    let cases = [
        (
            r#"{"type":"system","subtype":"init"}"#,
            EventKind::SystemInit,
        ),
        (
            r#"{"type":"system","subtype":"api_retry","attempt":1}"#,
            EventKind::ApiRetry,
        ),
        (r#"{"type":"result"}"#, EventKind::Result),
        (r#"{"type":"assistant"}"#, EventKind::Assistant),
        (r#"{"type":"user"}"#, EventKind::User),
        (r#"{"type":"rate_limit_event"}"#, EventKind::RateLimitEvent),
    ];
    for (line, want) in cases {
        assert_eq!(classify(line).kind, want, "line: {line}");
        assert_eq!(
            classify(line).raw,
            line,
            "the bytes are retained on every branch"
        );
    }
}

#[test]
fn a_new_system_subtype_names_itself() {
    assert_eq!(
        classify(r#"{"type":"system","subtype":"brand_new"}"#).kind,
        EventKind::System("brand_new".to_owned())
    );
}

#[test]
fn a_new_top_level_type_is_unknown_but_retained() {
    let line = classify(r#"{"type":"telemetry","payload":{"a":1}}"#);
    assert_eq!(line.kind, EventKind::Unknown("telemetry".to_owned()));
    assert_eq!(line.raw, r#"{"type":"telemetry","payload":{"a":1}}"#);
}

#[test]
fn malformed_json_is_retained_verbatim_not_dropped() {
    let line = classify("this is not json");
    assert_eq!(line.kind, EventKind::Malformed);
    assert_eq!(line.raw, "this is not json");
}

#[test]
fn usage_parses_from_both_nestings() {
    let nested = r#"{"type":"result","result":{"usage":{"input_tokens":10,"output_tokens":5}}}"#;
    let flat = r#"{"type":"result","usage":{"input_tokens":10,"output_tokens":5}}"#;
    for v in [nested, flat] {
        let u = usage_from_result(v).expect("usage should parse");
        assert_eq!(u.input_tokens, Some(10));
        assert_eq!(u.output_tokens, Some(5));
        assert_eq!(u.cache_creation, None, "absent stays None");
    }
}

#[test]
fn api_retry_parses_its_fields_or_is_none_never_default() {
    let r = api_retry_from(
        r#"{"type":"system","subtype":"api_retry","attempt":1,"max_retries":10,"retry_delay_ms":618,"error":"unknown","error_status":null}"#,
    )
    .expect("parses");
    assert_eq!(
        (r.attempt, r.max_retries, r.retry_delay_ms),
        (Some(1), Some(10), Some(618))
    );
    assert_eq!(r.error_status, None);
    assert_eq!(
        api_retry_from("not json"),
        None,
        "no unwrap_or_default: None, not a default"
    );
    let calls = tool_calls_from_assistant(
        r#"{"type":"assistant","parent_tool_use_id":null,"message":{"content":[{"type":"tool_use","name":"Agent","input":{"subagent_type":"probe"}}]}}"#,
    );
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].subagent_type.as_deref(), Some("probe"));
}
