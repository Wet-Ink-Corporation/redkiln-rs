//! `Usage` and `CacheCreation` — AC-362, and the two tests lifted with them (`s7c` §7c.2).
//!
//! AC-362's THEN: *"the field is recorded as absent, and the type that carries it is an `Option`
//! with no `Default` impl and no `unwrap_or(0)` at any call site, so a consumer cannot read absent
//! as zero without a visible match."* Its *Verified by* names three instruments and this file
//! carries the two that are tests: *"a fixture replay of a recorded `usage` block with the TTL
//! fields missing, asserting `None` not `Some(0)`; a serialiser test that absent and zero round-trip
//! distinguishably"*. The third — the CI grep over `unwrap_or(0)` / `unwrap_or_default()` and the
//! derive-Default walk — is Checkpoint 7c's (`s7c` §7c.21).
//!
//! **The absence fixture is SYNTHETIC and says so** (`s7c` §7c.9): the register's evidence gloss
//! (*"F3 measured … absent"*) is stale against ADR-0113 — the nested `cache_creation` object is
//! present on 950 of 955 recorded runs — so no recorded row at the pin has the split missing, and
//! the shape below is the recorded shape with the object removed by hand.

use rk_types::usage::{CacheCreation, Usage};

/// The recorded shape of a `result` frame's `usage` block (ADR-0113 decision 2's surface), with
/// the counters a `-p` run reports. This is the shape both fixtures below start from.
fn recorded_shape() -> serde_json::Value {
    serde_json::json!({
        "input_tokens": 2048,
        "output_tokens": 512,
        "cache_read_input_tokens": 1800,
        "cache_creation_input_tokens": 248,
        "cache_creation": {
            "ephemeral_5m_input_tokens": 148,
            "ephemeral_1h_input_tokens": 100
        }
    })
}

/// **AC-362, first instrument.** A `usage` block with the TTL split missing deserialises to
/// `None`, never to `Some(0)` — on the object, on the helper that reads it, and on a partial object
/// that names one tier and not the other.
#[test]
fn ac362_absent_usage_is_none_not_zero() {
    // SYNTHETIC: the recorded shape with the nested object removed by hand (§7c.9).
    let mut absent = recorded_shape();
    absent
        .as_object_mut()
        .expect("an object")
        .remove("cache_creation");
    let usage: Usage = serde_json::from_value(absent).expect("a usage block with no TTL split");

    assert_eq!(
        usage.cache_creation, None,
        "an absent `cache_creation` object is None, not an object of zeros"
    );
    assert_eq!(
        usage.ttl_split(),
        None,
        "ttl_split() over an absent object is None: a Some((0, 0)) would make a TTL claim look \
         verified (Halt 11; ADR-0078)"
    );
    // The counters that did arrive are still read; absence is per field, not per block.
    assert_eq!(usage.cache_creation_input_tokens, Some(248));

    // SYNTHETIC: one tier named, the other missing. A partial split is not a split.
    let partial: Usage = serde_json::from_value(serde_json::json!({
        "cache_creation_input_tokens": 248,
        "cache_creation": { "ephemeral_1h_input_tokens": 100 }
    }))
    .expect("a partial split");
    assert_eq!(
        partial.cache_creation,
        Some(CacheCreation {
            ephemeral_1h_input_tokens: Some(100),
            ephemeral_5m_input_tokens: None,
        })
    );
    assert_eq!(
        partial.ttl_split(),
        None,
        "a split with one tier absent propagates the absence rather than reading it as 0"
    );

    // An entirely empty block: every counter None, every helper None.
    let empty: Usage = serde_json::from_value(serde_json::json!({})).expect("an empty block");
    assert_eq!(empty.total_input(), None);
    assert_eq!(empty.cache_read_share(), None);
    assert_eq!(empty.ttl_split(), None);
}

/// **AC-362, second instrument.** Absent and zero are two values on the wire and two values after
/// the round trip, on the counters and on the helpers that read them.
#[test]
fn ac362_absent_and_zero_roundtrip_distinct() {
    let absent = Usage {
        input_tokens: Some(1),
        output_tokens: None,
        cache_read_input_tokens: None,
        cache_creation_input_tokens: None,
        ephemeral_cache_read_input_tokens: None,
        ephemeral_cache_creation_input_tokens: None,
        cache_creation: None,
    };
    let zero = Usage {
        input_tokens: Some(1),
        output_tokens: Some(0),
        cache_read_input_tokens: Some(0),
        cache_creation_input_tokens: Some(0),
        ephemeral_cache_read_input_tokens: None,
        ephemeral_cache_creation_input_tokens: None,
        cache_creation: Some(CacheCreation {
            ephemeral_1h_input_tokens: Some(0),
            ephemeral_5m_input_tokens: Some(0),
        }),
    };
    assert_ne!(absent, zero, "the two values are not equal as values");

    let absent_wire = serde_json::to_string(&absent).expect("serialise");
    let zero_wire = serde_json::to_string(&zero).expect("serialise");
    assert_ne!(
        absent_wire, zero_wire,
        "absent and zero serialise identically, so a reader cannot tell them apart"
    );

    let absent_back: Usage = serde_json::from_str(&absent_wire).expect("round trip");
    let zero_back: Usage = serde_json::from_str(&zero_wire).expect("round trip");
    assert_eq!(absent_back, absent);
    assert_eq!(zero_back, zero);
    assert_ne!(absent_back, zero_back);

    // The helpers keep the distinction: a measured zero is a measurement, an absence is not.
    assert_eq!(absent_back.ttl_split(), None);
    assert_eq!(zero_back.ttl_split(), Some((0, 0)));
    assert_eq!(
        absent_back.total_input(),
        None,
        "a term did not arrive, so there is no total: a sum over present terms would read the \
         absent counters as 0 by another spelling (AC-362's THEN; ADR-0126 forbids)"
    );
    assert_eq!(
        zero_back.total_input(),
        Some(1),
        "three measured counters, one of them non-zero, and the total is their sum"
    );
    assert_eq!(
        absent_back.cache_read_share(),
        None,
        "no read counter arrived, so there is no share"
    );
    assert_eq!(
        zero_back.cache_read_share(),
        None,
        "a measured zero over a measured zero is no share either — the denominator is 0"
    );
}

/// **AC-362's no-`Default` clause, as an assertion this crate's own suite can fail on** (ADR-0126
/// forbids: *"`Default` on `Usage`, `CacheCreation` or any usage-adjacent reading type"*; PIA-062).
/// Rust has no negative trait bound to write, so this is asserted over the source with comments
/// stripped — the house form of `verify_types::verify_report_has_no_default` — over the whole of
/// `usage.rs` and over `activity.rs` from the record's declaration down (`DispatchRecord`,
/// `RecordTail` and the constructor). Checkpoint 7c's derive-Default walk (`dd_hits`, `s7c` §7c.21)
/// is the CI instrument; that block is NOT run at Slice 1, and without this test a
/// `#[derive(Default)]` on any of the four types the walk names left every test in this file green
/// [measured 2026-09-17 on a scratch copy].
#[test]
fn ac362_no_default_on_any_usage_adjacent_type() {
    let usage_src = include_str!("../../src/usage.rs");
    let activity_src = include_str!("../../src/activity.rs");
    let record_from = activity_src
        .find("pub struct DispatchRecord")
        .expect("activity.rs declares DispatchRecord");
    let mut scanned = 0usize;
    for (name, src) in [
        ("usage.rs", usage_src),
        (
            "activity.rs (from `pub struct DispatchRecord`)",
            &activity_src[record_from..],
        ),
    ] {
        for (i, line) in src.lines().enumerate() {
            let code = line.split("//").next().unwrap_or("");
            scanned += 1;
            assert!(
                !code.contains("Default"),
                "{name}:{}: a `Default` on a usage-adjacent type is the absent-as-zero constructor \
                 AC-362 forbids (ADR-0126 forbids; PIA-062): {line}",
                i + 1
            );
        }
    }
    // The scan looked at something: a silent walk over an empty include would be a fake pass.
    assert!(
        scanned > 100,
        "the scan read {scanned} lines, which is not the two sources it names"
    );
}

/// **Lifted** from `runner/src/event.rs` at the corpus pin (`s7c` §7c.2), with the runner's
/// `usage_from_result` replaced by a direct deserialise of the `usage` member: the platform docs
/// place the split in a **nested** `usage.cache_creation` object, not as flat `ephemeral_*`
/// siblings, and parsing it flat reports "no TTL data" forever while the data sits one level down.
#[test]
fn ttl_split_reads_the_nested_cache_creation_object() {
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
    let u: Usage = serde_json::from_value(raw["result"]["usage"].clone()).expect("usage");
    assert_eq!(u.ttl_split(), Some((148, 100)));
    assert_eq!(u.total_input(), Some(2048 + 1800 + 248));
    assert_eq!(u.cache_read_share(), Some((1800, 248)));
}

/// **Lifted** from `runner/src/event.rs` at the corpus pin (`s7c` §7c.2); the runner's
/// `..Default::default()` became a struct literal because the type has no `Default` (AC-362).
/// Absence is evidence the dispatch ran wholly on the 5m tier. It must not be reported as
/// "0 tokens on 1h", which reads as a measurement.
#[test]
fn an_absent_split_is_none_not_zero() {
    let u = Usage {
        input_tokens: None,
        output_tokens: None,
        cache_read_input_tokens: None,
        cache_creation_input_tokens: Some(3611),
        ephemeral_cache_read_input_tokens: None,
        ephemeral_cache_creation_input_tokens: None,
        cache_creation: None,
    };
    assert_eq!(u.ttl_split(), None);
}

/// The SDK-reference naming is read when the documented naming is absent, and the documented
/// naming wins when both arrive — the runner's `.or()` order, kept.
#[test]
fn the_sdk_naming_is_a_fallback_never_a_sum() {
    let sdk: Usage = serde_json::from_value(serde_json::json!({
        "input_tokens": 10,
        "ephemeral_cache_read_input_tokens": 10,
        "ephemeral_cache_creation_input_tokens": 5
    }))
    .expect("the SDK naming");
    assert_eq!(sdk.cache_read_share(), Some((10, 5)));
    assert_eq!(sdk.total_input(), Some(25));

    let both: Usage = serde_json::from_value(serde_json::json!({
        "input_tokens": 1,
        "cache_read_input_tokens": 100,
        "ephemeral_cache_read_input_tokens": 10,
        "cache_creation_input_tokens": 7,
        "ephemeral_cache_creation_input_tokens": 5
    }))
    .expect("both namings");
    assert_eq!(both.cache_read_share(), Some((100, 7)));
    assert_eq!(both.total_input(), Some(108));

    // The fallback is per counter and the total is strict: with `input_tokens` absent there is
    // no total, whichever naming the cache counters arrived under.
    let no_input: Usage = serde_json::from_value(serde_json::json!({
        "ephemeral_cache_read_input_tokens": 10,
        "ephemeral_cache_creation_input_tokens": 5
    }))
    .expect("the SDK naming without input_tokens");
    assert_eq!(no_input.cache_read_share(), Some((10, 5)));
    assert_eq!(no_input.total_input(), None);
}

/// Unknown members of a `usage` block are tolerated (AC-201's rule reaches every stream struct):
/// `server_tool_use`, `service_tier` and whatever the next CLI adds are not a decode error.
#[test]
fn a_usage_block_with_unknown_members_still_decodes() {
    let mut raw = recorded_shape();
    raw["server_tool_use"] = serde_json::json!({ "web_search_requests": 0 });
    raw["service_tier"] = serde_json::json!("standard");
    let u: Usage = serde_json::from_value(raw).expect("unknown members are ignored, not refused");
    assert_eq!(u.ttl_split(), Some((148, 100)));
}
