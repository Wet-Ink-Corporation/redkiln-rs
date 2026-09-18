//! **The TTL classifier — FIVE states** (s7c §7c.9; ADR-0113 decision 1; AC-362).

use rk_types::usage::{CacheCreation, Usage};

use rk_adapter_claude::ttl::{TtlState, classify_ttl};

fn usage(total: Option<u64>, split: Option<(Option<u64>, Option<u64>)>) -> Usage {
    Usage {
        input_tokens: Some(10),
        output_tokens: Some(20),
        cache_read_input_tokens: Some(100),
        cache_creation_input_tokens: total,
        ephemeral_cache_read_input_tokens: None,
        ephemeral_cache_creation_input_tokens: None,
        cache_creation: split.map(|(one_hour, five_minute)| CacheCreation {
            ephemeral_1h_input_tokens: one_hour,
            ephemeral_5m_input_tokens: five_minute,
        }),
    }
}

/// ADR-0113 decision 1's five states, one outcome each: **verified 1 h** (`1h > 0 && 5m == 0`),
/// **verified 5 m** (the converse), **mixed** (both positive, both counts), **TTL-unverified**
/// (the object absent, a tier absent, no usage at all, or both zero with
/// `cache_creation_input_tokens > 0`), **no-write** (all three zero). Red by the 211-line file's
/// three states: `verified 5m` and `mixed` must be DISTINCT outcomes.
///
/// RED (observed 2026-09-18) against the skeleton (three states): `verified 5m is its own state
/// (ADR-0113 d1), not TTL-unverified: Unverified`.
#[test]
fn adr0113_ttl_classifier_five_states() {
    assert_eq!(
        classify_ttl(Some(&usage(Some(50), Some((Some(50), Some(0)))))),
        TtlState::Verified1h { one_hour: 50 }
    );
    assert_eq!(
        classify_ttl(Some(&usage(Some(70), Some((Some(0), Some(70)))))),
        TtlState::Verified5m { five_minute: 70 },
        "verified 5m is its own state (ADR-0113 d1), not TTL-unverified: {:?}",
        classify_ttl(Some(&usage(Some(70), Some((Some(0), Some(70))))))
    );
    assert_eq!(
        classify_ttl(Some(&usage(Some(90), Some((Some(60), Some(30)))))),
        TtlState::Mixed {
            one_hour: 60,
            five_minute: 30
        },
        "mixed is its own state, reported with both counts"
    );
    assert_eq!(
        classify_ttl(Some(&usage(Some(0), Some((Some(0), Some(0)))))),
        TtlState::NoWrite,
        "all three zero: a warm re-read carries no TTL claim"
    );
    // TTL-unverified, each way it arises
    assert_eq!(
        classify_ttl(Some(&usage(Some(30), Some((Some(0), Some(0)))))),
        TtlState::Unverified,
        "both tiers zero while cache_creation_input_tokens > 0: a write of unknown tier"
    );
    assert_eq!(
        classify_ttl(Some(&usage(Some(30), None))),
        TtlState::Unverified,
        "the object absent"
    );
    assert_eq!(
        classify_ttl(Some(&usage(Some(30), Some((Some(30), None))))),
        TtlState::Unverified,
        "a tier absent from the split: not a split (AC-362, never Some(0))"
    );
    assert_eq!(classify_ttl(None), TtlState::Unverified, "no usage at all");
    assert_eq!(
        classify_ttl(Some(&usage(None, Some((Some(0), Some(0)))))),
        TtlState::Unverified,
        "both tiers zero and the total ABSENT: nothing measured, not a no-write"
    );
    // the five are five distinct outcomes
    let states = [
        TtlState::Verified1h { one_hour: 1 },
        TtlState::Verified5m { five_minute: 1 },
        TtlState::Mixed {
            one_hour: 1,
            five_minute: 1,
        },
        TtlState::Unverified,
        TtlState::NoWrite,
    ];
    let names: std::collections::BTreeSet<&str> = states.iter().map(|s| s.as_str()).collect();
    assert_eq!(names.len(), 5, "{names:?}");
    // the recorded F20 usage: 81159 on 1h, 0 on 5m — verified 1h (the -p bucket's evidence)
    let f20 = rk_adapter_claude::event::usage_from_result(
        crate::support::fixture("recorded/f20.result_raw.jsonl").trim_end(),
    )
    .expect("F20's usage parses");
    assert_eq!(
        classify_ttl(Some(&f20)),
        TtlState::Verified1h { one_hour: 81_159 }
    );
}
