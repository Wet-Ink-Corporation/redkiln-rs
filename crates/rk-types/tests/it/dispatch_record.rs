//! `DispatchRecord`, completed — AC-360's type half, AC-227's type half, AC-210's non-optional
//! half (`s7c` §7c.11; ADR-0126 decision 1's payload table).
//!
//! The record was five fields at Step 7a with its own module doc saying the set was *"incomplete
//! by the record"* and *"owed at 7c"*. ADR-0126 decision 1 names the members and decision 2 homes
//! the types here. What this file pins is the **shape**: every member the table names is a member
//! (a structural destructure), the readings are `u16` ticks and never a float (a type-level
//! binding), `cli_version` is non-optional on both arms, absence is `null` and never `{}` or `0`,
//! and the whole row round-trips through `serde_json` — which is what `Event.payload` is.

use rk_types::activity::{DispatchRecord, RecordTail};
use rk_types::error::CancelledBy;
use rk_types::retention::{HEAD_TAIL, Retained, STDIO_CAP};
use rk_types::usage::{
    CacheCreation, CliVersion, CostUsd, DetectedBy, Timings, Usage, WindowReading,
};

fn argv() -> Vec<String> {
    vec!["claude".to_owned(), "-p".to_owned(), "--secret".to_owned()]
}

fn env() -> Vec<(String, String)> {
    vec![(
        "ANTHROPIC_API_KEY".to_owned(),
        "sk-verysecretvalue".to_owned(),
    )]
}

/// A tail with every optional member `Some`, so the round trip below exercises every arm.
fn full_tail() -> RecordTail {
    RecordTail {
        cli_version: CliVersion::Observed("2.1.275".to_owned()),
        api_retry_events: 2,
        prompt_cache_ttl: "1h".to_owned(),
        subagent_prompt_cache_ttl: "1h".to_owned(),
        timings: Timings {
            wall_ms: 4_200,
            api_ms: Some(3_900),
        },
        usage: Some(Usage {
            input_tokens: Some(2048),
            output_tokens: Some(512),
            cache_read_input_tokens: Some(1800),
            cache_creation_input_tokens: Some(248),
            ephemeral_cache_read_input_tokens: None,
            ephemeral_cache_creation_input_tokens: None,
            cache_creation: Some(CacheCreation {
                ephemeral_1h_input_tokens: Some(100),
                ephemeral_5m_input_tokens: Some(148),
            }),
        }),
        total_cost_usd: Some(CostUsd("2.337147".to_owned())),
        headroom_before: Some(WindowReading {
            five_hour: 4,
            seven_day: 12,
        }),
        headroom_after: Some(WindowReading {
            five_hour: 5,
            seven_day: 12,
        }),
        cancelled_by: Some(CancelledBy::Timeout),
        detected_by: Some(DetectedBy::TextMarker),
    }
}

/// A pre-spawn tail: nothing measured, the pin as `Pinned`, every optional member `None`.
fn pre_spawn_tail() -> RecordTail {
    RecordTail {
        cli_version: CliVersion::Pinned("2.1.275".to_owned()),
        api_retry_events: 0,
        prompt_cache_ttl: "1h".to_owned(),
        subagent_prompt_cache_ttl: "1h".to_owned(),
        timings: Timings {
            wall_ms: 3,
            api_ms: None,
        },
        usage: None,
        total_cost_usd: None,
        headroom_before: None,
        headroom_after: None,
        cancelled_by: None,
        detected_by: None,
    }
}

/// **AC-360 (type half), AC-227 (type half), AC-210 (non-optional half).** Every member ADR-0126
/// decision 1's table names is a member, the readings are ticks, the version is on both arms, and
/// the row survives the round trip `Event.payload` puts it through.
///
/// **What its red is, and is not** (`s7c` §7c.18 says *"red by a `Default` or an `f64` reading"*):
/// the `f64` half is a compile red and was taken [2026-09-17: `seven_day: f64` → E0277 *"the
/// trait bound `f64: Eq` is not satisfied"* on `WindowReading`'s derive — the library fails before
/// this binary is built; the `(u16, u16)` binding below is the second line, E0308 if the `Eq`
/// derive were ever dropped] — and its first red was E0026/E0599 on the absent members and
/// constructor. The `Default` half is **not** observable by a runtime test on a value and is not
/// this test's:
/// `usage::ac362_no_default_on_any_usage_adjacent_type` (a source scan) and Checkpoint 7c's
/// derive-Default walk carry it. The stage bullet over-states what this test can see, and that is
/// reported as a stage-file inaccuracy rather than smoothed here.
#[test]
fn ac360_dispatch_record_carries_the_usage_fields() {
    let rec = DispatchRecord::new(&argv(), &env(), "2.1.275", b"out", b"err", full_tail());

    // Structural: destructuring names EVERY member, so a member added or removed stops this
    // compiling. There is no `argv`, `env` or `result` member to name (ADR-0105 clause 7; §7c.11).
    let DispatchRecord {
        argv_digest,
        env_digest,
        instrument_version,
        stdout,
        stderr,
        usage,
        timings,
        total_cost_usd,
        headroom_before,
        headroom_after,
        cli_version,
        api_retry_events,
        prompt_cache_ttl,
        subagent_prompt_cache_ttl,
        cancelled_by,
        detected_by,
    } = rec.clone();

    // The digests are digests: 64 hex characters, and the secret is in neither.
    assert_eq!(argv_digest.as_str().len(), 64);
    assert!(!argv_digest.as_str().contains("secret"));
    assert_eq!(env_digest.as_str().len(), 64);
    assert!(!env_digest.as_str().contains("secret"));
    assert_eq!(instrument_version, "2.1.275");
    assert_eq!(stdout, Retained::Verbatim(b"out".to_vec()));
    assert_eq!(stderr, Retained::Verbatim(b"err".to_vec()));

    // The readings are u16 TICKS (AC-374) of EXACTLY two windows (AC-375); binding them to a
    // `(u16, u16)` is the type-level assertion — an `f64` member here does not compile.
    let WindowReading {
        five_hour,
        seven_day,
    } = headroom_before.expect("a before reading");
    let _ticks: (u16, u16) = (five_hour, seven_day);
    assert_eq!((five_hour, seven_day), (4, 12));
    assert_eq!(
        headroom_after,
        Some(WindowReading {
            five_hour: 5,
            seven_day: 12
        })
    );

    // The pinned version the THEN names and the observed one are TWO members (§7c.11).
    assert_eq!(cli_version, CliVersion::Observed("2.1.275".to_owned()));
    assert_eq!(api_retry_events, 2);
    // The two TTL fields are TWO members, by the variables' names minus the `CLAUDE_CODE_` prefix
    // (AC-210: *"no dispatch event lacks either TTL field"*; §7c.11: *"the two TTL fields"*).
    assert_eq!(prompt_cache_ttl, "1h");
    assert_eq!(subagent_prompt_cache_ttl, "1h");
    assert_eq!(
        timings,
        Timings {
            wall_ms: 4_200,
            api_ms: Some(3_900)
        }
    );
    assert_eq!(total_cost_usd, Some(CostUsd("2.337147".to_owned())));
    assert_eq!(cancelled_by, Some(CancelledBy::Timeout));
    assert_eq!(detected_by, Some(DetectedBy::TextMarker));
    assert_eq!(
        usage.as_ref().and_then(Usage::ttl_split),
        Some((148, 100)),
        "the usage block travels verbatim, nested object included (ADR-0113)"
    );

    // The row IS `Event.payload`: a `serde_json::Value`, and back.
    let payload = serde_json::to_value(&rec).expect("the record serialises");
    let back: DispatchRecord = serde_json::from_value(payload.clone()).expect("and deserialises");
    assert_eq!(back, rec, "the round trip is identity");

    // What the wire carries, by name: the cost as the decimal string the CLI printed (never a
    // number the serialiser could re-render), the version's provenance as the arm's name, the two
    // TTL values under their variables' names, and NO argv, env or result member.
    assert_eq!(payload["total_cost_usd"], serde_json::json!("2.337147"));
    assert_eq!(
        payload["cli_version"],
        serde_json::json!({ "observed": "2.1.275" })
    );
    assert_eq!(payload["prompt_cache_ttl"], serde_json::json!("1h"));
    assert_eq!(
        payload["subagent_prompt_cache_ttl"],
        serde_json::json!("1h")
    );
    assert_eq!(payload["cancelled_by"], serde_json::json!("timeout"));
    assert_eq!(payload["detected_by"], serde_json::json!("text_marker"));
    assert_eq!(
        payload["headroom_before"]["five_hour"],
        serde_json::json!(4)
    );
    for absent in ["argv", "env", "environment", "result"] {
        assert!(
            payload.get(absent).is_none(),
            "the row carries a `{absent}` member, which no record admits (ADR-0105 clause 7)"
        );
    }
}

/// The pre-spawn row: `cli_version` is present as `Pinned` (AC-227's non-optional field holds on
/// both arms), every usage-adjacent member is `null` — never `0`, never `{}` — and the row still
/// round-trips.
#[test]
fn a_pre_spawn_record_carries_the_pin_and_absence_as_null() {
    let rec = DispatchRecord::new(&argv(), &env(), "2.1.275", b"", b"", pre_spawn_tail());
    let payload = serde_json::to_value(&rec).expect("serialises");

    assert_eq!(
        payload["cli_version"],
        serde_json::json!({ "pinned": "2.1.275" })
    );
    assert_ne!(
        rec.cli_version,
        CliVersion::Observed("2.1.275".to_owned()),
        "the same version string under the two arms is two different values: the provenance is \
         the point (ADR-0126 decision 1; Halt 11's shape)"
    );
    for member in [
        "usage",
        "total_cost_usd",
        "headroom_before",
        "headroom_after",
        "cancelled_by",
        "detected_by",
    ] {
        // `.get()`, not `[]`: `Value`'s `Index<&str>` answers `Null` for a MISSING key too, so the
        // indexed form could not tell "present as null" from "omitted" and a
        // `skip_serializing_if` on the member would have passed it.
        assert_eq!(
            payload.get(member),
            Some(&serde_json::Value::Null),
            "`{member}` is absent on a pre-spawn row and absence travels as a PRESENT null member, \
             not 0, not {{}} and not an omitted key"
        );
    }
    assert_eq!(
        payload["timings"].get("api_ms"),
        Some(&serde_json::Value::Null)
    );
    assert_eq!(payload["api_retry_events"], serde_json::json!(0));

    let back: DispatchRecord = serde_json::from_value(payload).expect("deserialises");
    assert_eq!(back, rec);
    assert_eq!(back.usage, None);
}

/// **`Retained`'s bytes travel as the derived `Vec<u8>` form — a JSON array of integers** (§7c.11):
/// byte-lossless, which a UTF-8-lossy text is not and *"verbatim"* (AC-200) has to survive the
/// round trip; and `rk-store`'s canonical serialiser accepts integer arrays (it refuses only
/// floats). The cap is `rk-types`' too, so the capped arm round-trips from here as well.
#[test]
fn retained_bytes_travel_as_integer_arrays_and_survive_the_round_trip() {
    // Not valid UTF-8: a lossy text form would replace 0xFF and the round trip would not be identity.
    let raw = [b'o', b'k', 0xFF, 0xFE, b'\n'];
    let verbatim = Retained::of(&raw);
    let wire = serde_json::to_value(&verbatim).expect("serialises");
    assert_eq!(
        wire,
        serde_json::json!({ "verbatim": [111, 107, 255, 254, 10] })
    );
    let back: Retained = serde_json::from_value(wire).expect("deserialises");
    assert_eq!(back, verbatim);

    let big = vec![0xFFu8; STDIO_CAP + 1];
    let capped = Retained::of(&big);
    let wire = serde_json::to_value(&capped).expect("serialises");
    assert_eq!(
        wire["capped"]["head"].as_array().map(std::vec::Vec::len),
        Some(HEAD_TAIL)
    );
    assert_eq!(
        wire["capped"]["total_len"],
        serde_json::json!(STDIO_CAP + 1)
    );
    let back: Retained = serde_json::from_value(wire).expect("deserialises");
    assert_eq!(back, capped);
}

/// The constructor digests what must be digested and drops the environment: `env` is taken,
/// digested and never stored, and two dispatches under different environments are told apart by
/// the digest alone.
#[test]
fn the_constructor_digests_argv_and_env_and_stores_neither() {
    let a = DispatchRecord::new(&argv(), &env(), "2.1.275", b"", b"", pre_spawn_tail());
    let other_env = vec![("PATH".to_owned(), "/usr/bin".to_owned())];
    let b = DispatchRecord::new(&argv(), &other_env, "2.1.275", b"", b"", pre_spawn_tail());
    assert_eq!(a.argv_digest, b.argv_digest, "same argv, same digest");
    assert_ne!(
        a.env_digest, b.env_digest,
        "different environments, different digests"
    );

    // Order-independent: the environment is sorted before digesting, so a map's iteration order
    // is not part of the identity.
    let two = vec![
        ("B".to_owned(), "2".to_owned()),
        ("A".to_owned(), "1".to_owned()),
    ];
    let two_rev = vec![
        ("A".to_owned(), "1".to_owned()),
        ("B".to_owned(), "2".to_owned()),
    ];
    let c = DispatchRecord::new(&argv(), &two, "2.1.275", b"", b"", pre_spawn_tail());
    let d = DispatchRecord::new(&argv(), &two_rev, "2.1.275", b"", b"", pre_spawn_tail());
    assert_eq!(c.env_digest, d.env_digest);
}
