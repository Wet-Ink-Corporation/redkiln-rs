//! **AC-210 (the builder half), AC-217 (the constructed-map assertion), AC-526's AND** — the
//! environment builder over a validated `Config` (s7c §7c.6).
//!
//! The runtime half of AC-210 — both values recorded on the `dispatch_completed` row — and AC-509's
//! echo through the real spawn path are Slice 5's (`ac210_both_ttl_vars_set_and_recorded`,
//! `ac509_child_echoes_env_without_tracker_credential`); what a builder over a value can be
//! asserted on is here.

use rk_adapter_claude::config::Config;
use rk_adapter_claude::env::{PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL, child_env};
use rk_adapter_claude::env_denylist::is_denylisted;

use crate::support::{config, config_with_env, values};

/// AC-210: *"`CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` are
/// set explicitly"* — the builder sets BOTH, to one value, on every map it constructs; AC-526's
/// AND: *"a constructed child environment never carries exactly one of the two"*.
///
/// RED (observed 2026-09-18) against the skeleton that set the first variable only:
/// `the builder sets CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL beside the first (AC-210; ADR-0078 (ii))`.
#[test]
fn child_env_sets_both_ttls_explicitly() {
    let cfg = config();
    let env = child_env(&cfg);
    let map = env.as_map();
    assert_eq!(
        map.get(PROMPT_CACHE_TTL).map(String::as_str),
        Some("1h"),
        "the builder sets CLAUDE_CODE_PROMPT_CACHE_TTL (AC-210)"
    );
    assert_eq!(
        map.get(SUBAGENT_PROMPT_CACHE_TTL).map(String::as_str),
        Some("1h"),
        "the builder sets CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL beside the first (AC-210; ADR-0078 (ii))"
    );
    assert_eq!(env.ttl_pair(), (Some("1h"), Some("1h")));
    assert_eq!(PROMPT_CACHE_TTL, "CLAUDE_CODE_PROMPT_CACHE_TTL");
    assert_eq!(
        SUBAGENT_PROMPT_CACHE_TTL,
        "CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL"
    );

    // A configured pair travels as one value onto both.
    let mut v = values();
    v.prompt_cache_ttl = Some("5m".to_owned());
    v.subagent_prompt_cache_ttl = Some("5m".to_owned());
    let env = child_env(&Config::new(v).expect("both equal constructs"));
    assert_eq!(env.ttl_pair(), (Some("5m"), Some("5m")));
}

/// AC-385: *"`CLAUDE_CODE_PROMPT_CACHE_TTL` is passed explicitly, never inherited"* — a captured
/// map that carries either TTL name is overridden by the configured pair, so the child's pair is
/// the adapter's on every path (and never exactly one — AC-526's AND).
#[test]
fn child_env_overrides_inherited_ttls_with_the_configured_pair() {
    let cfg = config_with_env(&[(PROMPT_CACHE_TTL, "5m"), ("TERM", "xterm")])
        .expect("the TTL names are not denylisted; the map constructs");
    let env = child_env(&cfg);
    assert_eq!(
        env.ttl_pair(),
        (Some("1h"), Some("1h")),
        "the inherited value is overridden, never passed on"
    );
    assert_eq!(env.as_map().get("TERM").map(String::as_str), Some("xterm"));

    // Windows environment names are case-insensitive (the denylist reasons the same way): a
    // captured `claude_code_prompt_cache_ttl=5m` IS the variable to the child, and `std`'s
    // `Command::envs` keys compare case-insensitively there — a map carrying both spellings would
    // hand the child whichever `envs()` inserted LAST (the lowercase one, in BTreeMap order), so
    // the inherited value would win over the configured one (AC-385) and the two names would
    // carry two values (AC-526's AND). The builder removes every case variant of either name.
    //
    // RED (observed 2026-09-18) against the exact-key insert: `the constructed map carries exactly
    // the two canonical TTL names — no case variant survives beside them (AC-385; AC-526's AND):
    // ["CLAUDE_CODE_PROMPT_CACHE_TTL", "CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL",
    // "Claude_Code_Prompt_Cache_Ttl", "TERM", "claude_code_prompt_cache_ttl",
    // "claude_code_subagent_prompt_cache_ttl"]`.
    let cfg = config_with_env(&[
        ("claude_code_prompt_cache_ttl", "5m"),
        ("claude_code_subagent_prompt_cache_ttl", "5m"),
        ("Claude_Code_Prompt_Cache_Ttl", "7m"),
        ("TERM", "xterm"),
    ])
    .expect("constructs");
    let env = child_env(&cfg);
    let keys: Vec<&str> = env.as_map().keys().map(String::as_str).collect();
    assert_eq!(
        keys,
        vec![PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL, "TERM"],
        "the constructed map carries exactly the two canonical TTL names — no case variant survives beside them (AC-385; AC-526's AND): {keys:?}"
    );
    assert_eq!(env.ttl_pair(), (Some("1h"), Some("1h")));
    assert!(
        env.as_map()
            .iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(PROMPT_CACHE_TTL)
                || k.eq_ignore_ascii_case(SUBAGENT_PROMPT_CACHE_TTL))
            .all(|(_, v)| v == "1h"),
        "every spelling of either name carries the configured value"
    );
}

/// AC-217's constructed-map assertion: *"an assertion on the constructed child env map that no
/// denylisted name is present"* — and, since `Config::new` refused every such map already, that
/// the builder is the captured map plus the two TTL names and nothing else (what `ac509_…`'s echo
/// asserts through the real spawn at Slice 5).
#[test]
fn child_env_is_the_captured_map_plus_the_two_ttls_and_nothing_else() {
    let cfg = config_with_env(&[
        ("PATH", "/usr/bin"),
        ("TERM", "xterm"),
        ("LANG", "C.UTF-8"),
        ("HTTPS_PROXY", "http://proxy:3128"),
        ("ANTHROPIC_MODEL", "opus"),
    ])
    .expect("constructs");
    let env = child_env(&cfg);
    let map = env.as_map();
    assert!(
        map.keys().all(|k| !is_denylisted(k)),
        "no denylisted name on the constructed map (AC-217)"
    );
    let mut expected: Vec<&str> = cfg.captured_env().keys().map(String::as_str).collect();
    expected.push(PROMPT_CACHE_TTL);
    expected.push(SUBAGENT_PROMPT_CACHE_TTL);
    expected.sort_unstable();
    let got: Vec<&str> = map.keys().map(String::as_str).collect();
    assert_eq!(
        got, expected,
        "the captured map plus exactly the two TTL names"
    );
    for (k, v) in cfg.captured_env() {
        assert_eq!(
            map.get(k),
            Some(v),
            "{k} travels as captured (inherit minus denylist, #16)"
        );
    }
}
