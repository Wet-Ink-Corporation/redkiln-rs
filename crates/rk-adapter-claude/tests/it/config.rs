//! **AC-488, AC-217, AC-422, AC-526, AC-377 (the `Config::new` lints)** — s7c §7c.4, §7c.6, §7c.8.
//!
//! Every refusal here is `Err(ConfigError::…)` from `Config::new` over a captured map the test
//! writes as a value: on a strip-and-pass implementation the call returns `Ok`, so the `is_err()`
//! assertion is red by itself (§7c.18). **The marker-count belt — dispatch once through the fixture
//! child on the `Ok` branch and assert a marker count of 0 — is Slice 5's** (§7c.20 item 4: the
//! refusal tests land at Slice 4 with their `is_err()` half only, because a stubbed `dispatch` would
//! give the belt an `Ok` branch that reaches no spawn, a red for the wrong reason).

use std::collections::BTreeMap;
use std::path::Path;

use rk_adapter_claude::config::{
    CACHE_TTL, Config, ConfigError, MIN_MARGIN_TICKS, Margins, MismatchPolicy, TESTED_CLI_VERSION,
    Window, margin_ticks_from_decimal,
};
use rk_adapter_claude::env::{PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL};
use rk_adapter_claude::env_denylist::{DENYLIST, DENYLIST_PREFIXES, is_denylisted};

use crate::support::{
    Markers, belt_dispatch_count, config_with_env, fixture, margins_ok, spawn_values, values,
};

/// AC-488 (construction half): *"GIVEN `ANTHROPIC_API_KEY` is present in the environment the
/// Claude adapter would dispatch from / WHEN the adapter is constructed at startup / THEN
/// construction fails with a typed configuration error naming the variable, and no dispatch is
/// attempted."* The variable is a value in the captured map, never `set_var` (ADR-0083).
///
/// RED (observed 2026-09-18) against the skeleton whose denylist was empty:
/// `a Config map carrying ANTHROPIC_API_KEY must not construct (AC-488)`.
#[test]
fn api_key_refuses_construction() {
    let markers = Markers::new();
    let mut v = spawn_values(&markers, "exit:0");
    v.captured_env.insert(
        "ANTHROPIC_API_KEY".to_owned(),
        "sk-ant-not-a-real-key".to_owned(),
    );
    v.captured_env.insert("PATH".to_owned(), "x".to_owned());
    let result = Config::new(v.clone());
    // The marker-count belt (§7c.18; Slice 5): on an `Ok` the test dispatches once through the
    // fixture child and asserts zero marker files — the belt that turns a warning-not-refusal
    // implementation red on the second assertion.
    // RED (belt half, observed 2026-09-18) against `Config::new` mutated to STRIP the denylisted
    // name and proceed: `belt: a marker file appeared after a Config carrying ANTHROPIC_API_KEY
    // constructed (AC-488; dispatch count must be 0)`: left `1`, right `0`.
    if result.is_ok() {
        assert_eq!(
            belt_dispatch_count(v),
            0,
            "belt: a marker file appeared after a Config carrying ANTHROPIC_API_KEY constructed \
             (AC-488; dispatch count must be 0)"
        );
    }
    assert_eq!(markers.count(), 0, "construction itself spawned nothing");
    let err =
        result.expect_err("a Config map carrying ANTHROPIC_API_KEY must not construct (AC-488)");
    match &err {
        ConfigError::DenylistedEnv { name } => assert_eq!(name, "ANTHROPIC_API_KEY"),
        other => panic!("the typed error names the variable; got {other:?}"),
    }
    assert!(
        err.to_string().contains("ANTHROPIC_API_KEY"),
        "the rendered error names the variable: {err}"
    );
}

/// AC-217: *"a denylisted name found there fails the dispatch as a typed configuration error
/// rather than being stripped and passed through"* — REFUSAL, never subtraction (§7c.6: the
/// 211-line file said "minus a denylist" as a strip; the register's THEN says refusal). The fixture
/// map carries `ANTHROPIC_FOUNDRY_API_KEY` as a positive case beside `ANTHROPIC_API_KEY` (§7c.6 —
/// it is persisted at user scope on this box), plus one name per denylist clause.
///
/// RED (observed 2026-09-18) against §7c.18's named implementation — a `Config::new` that STRIPS
/// every denylisted name and proceeds (`captured_env.retain(|k, _| !is_denylisted(k))`, `Ok`):
/// `ANTHROPIC_API_KEY in the captured map must be REFUSED, not stripped (AC-217; the register's
/// first exact name); the constructed Config carries 2 names` — TERM and LANG kept, the key
/// stripped. (Against the empty-denylist skeleton, earlier, the same assertion read `3 names`.)
#[test]
fn env_denylist_config_map() {
    let cases: &[(&str, &str)] = &[
        ("ANTHROPIC_API_KEY", "the register's first exact name"),
        ("ANTHROPIC_AUTH_TOKEN", "the register's second exact name"),
        ("ANTHROPIC_BASE_URL", "the register's third exact name"),
        (
            "ANTHROPIC_FOUNDRY_API_KEY",
            "§7c.6's added exact name — persisted on this box",
        ),
        (
            "CLAUDE_CODE_USE_BEDROCK",
            "the register's vendor-routing prefix CLAUDE_CODE_USE_*",
        ),
        ("CLAUDE_CODE_USE_VERTEX", "the prefix, a second instance"),
        ("GITHUB_TOKEN", "RT-25: the tracker credential (AC-509)"),
        ("GH_TOKEN", "RT-25"),
        ("GH_ENTERPRISE_TOKEN", "RT-25"),
    ];
    for (name, why) in cases {
        let markers = Markers::new();
        let mut v = spawn_values(&markers, "exit:0");
        for (k, val) in [(*name, "value"), ("TERM", "xterm"), ("LANG", "C.UTF-8")] {
            v.captured_env.insert(k.to_owned(), val.to_owned());
        }
        let result = Config::new(v.clone());
        match result {
            Err(ConfigError::DenylistedEnv { name: named }) => {
                assert_eq!(
                    &named, name,
                    "the refusal names the variable it found ({why})"
                );
            }
            Err(other) => panic!("{name}: the refusal is DenylistedEnv, got {other:?} ({why})"),
            Ok(cfg) => {
                // The marker-count belt (§7c.18; Slice 5): a STRIPPED name passes through to a
                // spawn, and the marker file is the red.
                // RED (belt half, observed 2026-09-18) against `Config::new` mutated to strip:
                // `belt: ANTHROPIC_API_KEY was STRIPPED and a marker file appeared (AC-217: refusal,
                // dispatch count 0)`: left `1`, right `0`.
                assert_eq!(
                    belt_dispatch_count(v),
                    0,
                    "belt: {name} was STRIPPED and a marker file appeared (AC-217: refusal, \
                     dispatch count 0)"
                );
                panic!(
                    "{name} in the captured map must be REFUSED, not stripped (AC-217; {why}); the \
                     constructed Config carries {} names",
                    cfg.captured_env().len()
                );
            }
        }
        assert_eq!(markers.count(), 0, "{name}: the refusal spawned nothing");
    }
    // A denylisted name is refused on its NAME alone — an empty value is still the variable being
    // present, which is what ADR-0010's "assert absent" means.
    assert!(
        config_with_env(&[("ANTHROPIC_API_KEY", "")]).is_err(),
        "an empty-valued ANTHROPIC_API_KEY is still present (AC-488)"
    );
    // Windows environment names are case-insensitive: a case variant IS the variable to the child,
    // so it is refused too (fail closed on every platform).
    assert!(
        config_with_env(&[("anthropic_api_key", "x")]).is_err(),
        "a case variant of a denylisted name is the same variable on Windows and is refused"
    );
    // Names the list does NOT carry pass — the list is exact names plus one prefix, never an
    // `ANTHROPIC_*` prefix rule (§7c.6: a prefix rule would be a stage decision no record makes).
    let cfg = config_with_env(&[
        ("ANTHROPIC_MODEL", "opus"),
        ("CLAUDE_CODE_MAX_OUTPUT_TOKENS", "8192"),
        ("GITHUB_ACTIONS", "true"),
        ("TERM", "xterm"),
    ])
    .expect("names outside the committed list construct (inherit minus denylist, decision #16)");
    assert_eq!(
        cfg.captured_env().len(),
        4,
        "nothing is stripped on the accepted path either"
    );
}

/// AC-422 (adapter instrument): *"an assertion that the `doctor` row and AC-217's construction test
/// read one list"* — the test ITERATES the exported `DENYLIST` slice and asserts `Config::new`
/// refuses a map carrying each name in it, and one instance of each exported prefix; red by any
/// name in the export that `Config::new` accepts (a private copy missing a name). "A second list"
/// is not something a test observes; the iteration is (§7c.18).
///
/// RED (observed 2026-09-18) against the empty-denylist skeleton:
/// `the exported DENYLIST is empty — AC-217/AC-422/AC-488 have nothing to read`.
#[test]
fn ac422_denylist_is_one_exported_list() {
    assert!(
        !DENYLIST.is_empty(),
        "the exported DENYLIST is empty — AC-217/AC-422/AC-488 have nothing to read"
    );
    for name in DENYLIST {
        assert!(
            is_denylisted(name),
            "{name} is exported but is_denylisted says no"
        );
        match config_with_env(&[(name, "x")]) {
            Err(ConfigError::DenylistedEnv { name: named }) => assert_eq!(&named, name),
            other => panic!("{name} is on the exported list but Config::new answered {other:?}"),
        }
    }
    assert!(
        !DENYLIST_PREFIXES.is_empty(),
        "the register's CLAUDE_CODE_USE_* prefix is exported as a prefix"
    );
    for prefix in DENYLIST_PREFIXES {
        let name = format!("{prefix}X");
        match config_with_env(&[(&name, "x")]) {
            Err(ConfigError::DenylistedEnv { name: named }) => assert_eq!(named, name),
            other => {
                panic!("{name} matches exported prefix {prefix} but Config::new answered {other:?}")
            }
        }
    }
    // The committed list holds exactly what §7c.6 says it holds: the register's three exact names,
    // the added ANTHROPIC_FOUNDRY_API_KEY, RT-25's three tracker names, and the one prefix.
    let mut exact: Vec<&str> = DENYLIST.to_vec();
    exact.sort_unstable();
    assert_eq!(
        exact,
        vec![
            "ANTHROPIC_API_KEY",
            "ANTHROPIC_AUTH_TOKEN",
            "ANTHROPIC_BASE_URL",
            "ANTHROPIC_FOUNDRY_API_KEY",
            "GH_ENTERPRISE_TOKEN",
            "GH_TOKEN",
            "GITHUB_TOKEN",
        ],
        "the exact names are §7c.6's (the register's three + ANTHROPIC_FOUNDRY_API_KEY) and RT-25's three"
    );
    assert_eq!(DENYLIST_PREFIXES, ["CLAUDE_CODE_USE_"]);
}

/// AC-526: five cases — the register's four (neither / first only / second only / both equal) plus
/// the mismatched pair the 211-line file omitted (§7c.6) — *"refused as a typed configuration error
/// naming both variables, before any dispatch"* on the two single cases and the mismatch. The
/// "neither" case constructs on the record's value (`1h`/`1h`, ADR-0078 (ii); AC-210's THEN), the
/// "both equal" case on the pair's value.
///
/// RED (observed 2026-09-18) against the skeleton that took whichever value was present:
/// `first only: CLAUDE_CODE_PROMPT_CACHE_TTL without CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL must be
/// refused (AC-526)`.
#[test]
fn ac526_ttl_pair_refuses_single_or_mismatch() {
    // Every case carries the spawn seam (the fixture child, a marker dir) so the belt below can
    // dispatch on an `Ok` and the refusing cases can assert nothing spawned (§7c.18; Slice 5).
    let markers = Markers::new();
    let with = |p: Option<&str>, s: Option<&str>| {
        let mut v = spawn_values(&markers, "exit:0");
        v.prompt_cache_ttl = p.map(str::to_owned);
        v.subagent_prompt_cache_ttl = s.map(str::to_owned);
        Config::new(v)
    };
    // The belt: dispatch once through the fixture child on an `Ok` from a refusing case.
    // RED (belt half, observed 2026-09-18) against `Config::new` mutated to accept a single TTL:
    // `belt (first only): a marker file appeared after a single-TTL Config constructed (AC-526)`:
    // left `1`, right `0`.
    let belt = |case: &str, p: Option<&str>, s: Option<&str>| {
        let mut v = spawn_values(&markers, "exit:0");
        v.prompt_cache_ttl = p.map(str::to_owned);
        v.subagent_prompt_cache_ttl = s.map(str::to_owned);
        if Config::new(v.clone()).is_ok() {
            assert_eq!(
                belt_dispatch_count(v),
                0,
                "belt ({case}): a marker file appeared after a single-TTL Config constructed (AC-526)"
            );
        }
    };
    // neither: the record's value, explicitly.
    let cfg = with(None, None).expect("neither: the pair is the record's 1h/1h (ADR-0078 (ii))");
    assert_eq!(cfg.cache_ttl(), CACHE_TTL);
    assert_eq!(CACHE_TTL, "1h", "AC-210 names the value");
    // first only.
    belt("first only", Some("1h"), None);
    let err = with(Some("1h"), None).expect_err(
        "first only: CLAUDE_CODE_PROMPT_CACHE_TTL without CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL must be refused (AC-526)",
    );
    names_both(&err, "first only");
    // second only.
    belt("second only", None, Some("1h"));
    let err = with(None, Some("1h")).expect_err(
        "second only: CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL without CLAUDE_CODE_PROMPT_CACHE_TTL must be refused (AC-526)",
    );
    names_both(&err, "second only");
    // both equal.
    let cfg = with(Some("5m"), Some("5m")).expect("both equal: one value, accepted");
    assert_eq!(cfg.cache_ttl(), "5m");
    // the mismatched pair.
    belt("mismatched pair", Some("1h"), Some("5m"));
    let err = with(Some("1h"), Some("5m")).expect_err("a mismatched pair must be refused (AC-526)");
    names_both(&err, "mismatched pair");
    match err {
        ConfigError::TtlPairMismatch { prompt, subagent } => {
            assert_eq!((prompt.as_str(), subagent.as_str()), ("1h", "5m"));
        }
        other => panic!("the mismatch is its own variant carrying both values; got {other:?}"),
    }
    assert_eq!(markers.count(), 0, "no refusing case spawned");
}

/// The refusal names BOTH variables (AC-526's THEN), whichever was missing.
fn names_both(err: &ConfigError, case: &str) {
    let text = err.to_string();
    assert!(
        text.contains(PROMPT_CACHE_TTL) && text.contains(SUBAGENT_PROMPT_CACHE_TTL),
        "{case}: the refusal names both variables (AC-526): {text}"
    );
}

/// AC-377 (config half): the register's vectors *"margins 0.005 / 0.01 / 0.02 (1 / 1 / 2 ticks — the
/// decimal parse must not round 0.01 to 2 or 0.02 to 1) and at 0.07 / 0.18"*, under O-3's
/// recommended default (a decimal pack margin is parsed to hundredths by CEILING, never rounding —
/// pending the operator's answer, RAISED); *"a fixture config that must fail; a fixture config with
/// no margin that must fail (no default)"*; and *"a grep rule that `Margins` implements no
/// `Default`"* — the block's `dd_hits` walk, re-run here over `src/config.rs` in the same shape.
///
/// RED (observed 2026-09-18) against the skeleton's two-digit truncating parse and lint-free
/// `Config::new`: `assertion left == right failed: 0.005 -> 1 tick(s) (ceiling, O-3)` — `left: 0
/// right: 1` (the first vector; the one-tick fixture's refusal is the next assertion the same
/// skeleton fails).
#[test]
fn ac377_margin_parse_vectors_and_no_default() {
    // ---- the parse vectors (O-3: ceiling to hundredths, for margins only).
    let vectors: &[(&str, u16)] = &[
        ("0.005", 1),
        ("0.01", 1),
        ("0.02", 2),
        ("0.07", 7),
        ("0.18", 18),
        ("0.1", 10),
        ("0.10", 10),
        ("0.0", 0),
        ("0", 0),
        ("1", 100),
        ("0.001", 1),
        ("0.011", 2),
        ("0.019", 2),
        ("0.020", 2),
        ("0.0200", 2),
    ];
    for (text, ticks) in vectors {
        assert_eq!(
            margin_ticks_from_decimal(text).unwrap_or_else(|e| panic!("{text}: {e}")),
            *ticks,
            "{text} -> {ticks} tick(s) (ceiling, O-3)"
        );
    }
    // The two overflow shapes: `655.36` is one tick past `u16::MAX` (65,536 ticks); `42949672.99`
    // is a `whole * 100` that fits a `u32` (4,294,967,200) whose fractional ticks do NOT — a
    // parser that checked only the multiplication panicked here in a debug build instead of
    // refusing (a margin that cannot be read is refused, never read as a number).
    for bad in [
        "",
        ".",
        "-0.02",
        "0.a",
        "1e-2",
        "0,02",
        " 0.02",
        "0.02 ",
        "2.",
        "abc",
        "655.36",
        "42949672.99",
        "42949673",
        "99999999999999999999.5",
    ] {
        assert!(
            margin_ticks_from_decimal(bad).is_err(),
            "{bad:?} is not a decimal margin and must be refused, never read as a number"
        );
    }
    assert_eq!(
        MIN_MARGIN_TICKS, 2,
        "AC-220/AC-377: at least two instrument ticks"
    );

    // ---- the fixture configs.
    let text = fixture("config/no-margin.json");
    assert!(
        serde_json::from_str::<Margins>(&text).is_err(),
        "no-margin.json: a config with no margin must fail — there is no default (AC-377)"
    );
    let text = fixture("config/one-window.json");
    assert!(
        serde_json::from_str::<Margins>(&text).is_err(),
        "one-window.json: a margin for one window is not a margin (AC-375: exactly two windows)"
    );
    let text = fixture("config/margin-one-tick.json");
    let one_tick: Margins = serde_json::from_str(&text).expect("margin-one-tick.json parses");
    let markers = Markers::new();
    let mut v = spawn_values(&markers, "exit:0");
    v.margins = one_tick;
    // The marker-count belt (§7c.18; Slice 5): a sub-two-tick Config that constructs dispatches
    // once through the fixture child, and the marker file is the red.
    // RED (belt half, observed 2026-09-18) against `Config::new` mutated to skip the margin lint:
    // `belt: a marker file appeared after a sub-two-tick margin Config constructed (AC-377)`:
    // left `1`, right `0`.
    if Config::new(v.clone()).is_ok() {
        assert_eq!(
            belt_dispatch_count(v.clone()),
            0,
            "belt: a marker file appeared after a sub-two-tick margin Config constructed (AC-377)"
        );
    }
    let err = Config::new(v).expect_err(
        "margin-one-tick.json: a margin under two ticks must be refused (AC-377/AC-220)",
    );
    assert_eq!(markers.count(), 0, "the refusal spawned nothing");
    match err {
        ConfigError::MarginBelowTwoTicks { window, ticks } => {
            assert_eq!((window, ticks), (Window::FiveHour, 1));
        }
        other => panic!("the refusal names the window and the ticks; got {other:?}"),
    }
    // The other window is lint-ed too, and exactly two ticks is the boundary that passes.
    let mut v = values();
    v.margins = Margins {
        five_hour: 2,
        seven_day: 1,
    };
    match Config::new(v) {
        Err(ConfigError::MarginBelowTwoTicks { window, ticks }) => {
            assert_eq!((window, ticks), (Window::SevenDay, 1));
        }
        other => panic!("seven_day at 1 tick is refused; got {other:?}"),
    }
    let mut v = values();
    v.margins = Margins {
        five_hour: 2,
        seven_day: 2,
    };
    assert!(
        Config::new(v).is_ok(),
        "two ticks on both windows is the boundary and passes"
    );
    let text = fixture("config/margins-ok.json");
    let ok: Margins = serde_json::from_str(&text).expect("margins-ok.json parses");
    assert_eq!(ok, margins_ok());
    let mut v = values();
    v.margins = ok;
    let cfg = Config::new(v).expect("margins-ok.json constructs");
    assert_eq!(
        cfg.margins(),
        margins_ok(),
        "the margins reach the Config as handed in (ADR-0083)"
    );

    // ---- no Default on Margins: the block's derive-Default walk, in its own shape (a contiguous
    // attribute block above `pub struct Margins` naming Default — including the block's
    // CONTINUATION rule, a line of `[A-Za-z_,\s]+` ending in `,`, `)` or `]`, so a rustfmt-wrapped
    // `#[derive(\n    Debug, …, Default,\n)]` is read too; an `impl Default for Margins`).
    //
    // RED (observed 2026-09-18) against a mutant config.rs whose derive was rewritten to the
    // wrapped three-line form with `Default,` appended: `Margins derives Default (AC-377: never a
    // code default): derive( Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, )`
    // — the walk WITHOUT the continuation rule stopped at `Debug, Clone, …` and was green on it.
    // `^[A-Za-z_,\s]+[,)\]]\s*$`, as the block's `dd_hits` spells it.
    let is_continuation = |t: &str| {
        let t = t.trim_end();
        t.len() >= 2
            && t.ends_with([',', ')', ']'])
            && t[..t.len() - 1].bytes().all(|b| {
                b.is_ascii_alphabetic() || b == b'_' || b == b',' || b == b' ' || b == b'\t'
            })
    };
    let src = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/config.rs"))
        .expect("read src/config.rs");
    let lines: Vec<&str> = src.lines().collect();
    let at = lines
        .iter()
        .position(|l| l.starts_with("pub struct Margins"))
        .expect("`pub struct Margins` is declared at column 0 in src/config.rs (§7c.17)");
    let mut j = at;
    let mut attrs = String::new();
    while j > 0 {
        j -= 1;
        let t = lines[j].trim_start();
        if lines[j].trim().is_empty()
            || t.starts_with("#[")
            || t.starts_with("///")
            || t.starts_with("//")
            || t.starts_with("/*")
            || t.starts_with('*')
            || t.starts_with(')')
            || t.starts_with(']')
            || is_continuation(t)
        {
            attrs.insert(0, ' ');
            attrs.insert_str(0, t);
        } else {
            break;
        }
    }
    for derive in attrs.split("derive(").skip(1) {
        let inner = derive.split(')').next().unwrap_or("");
        assert!(
            !inner.split(',').any(|t| t.trim() == "Default"),
            "Margins derives Default (AC-377: never a code default): derive({inner})"
        );
    }
    assert!(
        !src.lines()
            .filter(|l| !l.trim_start().starts_with("//"))
            .any(|l| l.contains("impl Default for Margins")),
        "Margins has a hand-written Default (AC-377)"
    );
}

/// D17-11 / ADR-0078: the pin is a `Config` value, declared as the block's anchored grep reads it,
/// and never read from the process environment; `Config` carries it as `tested_cli_version`.
#[test]
fn tested_cli_version_is_the_pin_as_a_config_value() {
    assert_eq!(TESTED_CLI_VERSION, "2.1.303");
    let cfg = crate::support::config();
    assert_eq!(cfg.tested_cli_version(), TESTED_CLI_VERSION);
    let src = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/config.rs"))
        .expect("read src/config.rs");
    assert!(
        src.lines()
            .any(|l| l.trim_start() == "pub const TESTED_CLI_VERSION: &str = \"2.1.303\";"),
        "config.rs declares the constant in the form the checkpoint's anchored grep reads (§7c.12)"
    );
}

/// AC-226's configuration arms (O-4): the mismatch policy is a `Config` value the composition root
/// names explicitly — refusal, or an opted-in `Degraded` allowance — with no `Default` to fall into.
/// The runtime halves (a scripted `system/init` at a higher and a lower version under both policies,
/// `ac226_version_mismatch_configured_outcome`) are Slice 5's.
#[test]
fn version_mismatch_policy_is_explicit_and_carried() {
    let mut v = values();
    v.on_version_mismatch = MismatchPolicy::Refuse;
    assert_eq!(
        Config::new(v).expect("constructs").on_version_mismatch(),
        MismatchPolicy::Refuse
    );
    let mut v = values();
    v.on_version_mismatch = MismatchPolicy::Degraded;
    assert_eq!(
        Config::new(v).expect("constructs").on_version_mismatch(),
        MismatchPolicy::Degraded
    );
    // The wire form the pack reader will hand in (7d): `refuse` / `degraded`, nothing else.
    assert_eq!(
        serde_json::from_str::<MismatchPolicy>("\"refuse\"").expect("parses"),
        MismatchPolicy::Refuse
    );
    assert_eq!(
        serde_json::from_str::<MismatchPolicy>("\"degraded\"").expect("parses"),
        MismatchPolicy::Degraded
    );
    assert!(serde_json::from_str::<MismatchPolicy>("\"warn\"").is_err());
}

/// The captured map reaches the `Config` as handed in, and the accessors hand back the values that
/// were constructed from — nothing is read from anywhere else (ADR-0083).
#[test]
fn config_carries_its_values_and_nothing_else() {
    let mut v = values();
    v.captured_env = BTreeMap::from([
        ("PATH".to_owned(), "/usr/bin".to_owned()),
        ("TERM".to_owned(), "xterm".to_owned()),
    ]);
    v.harness_binary = Path::new("C:/tools/claude.exe").to_path_buf();
    let cfg = Config::new(v.clone()).expect("constructs");
    assert_eq!(cfg.captured_env(), &v.captured_env);
    assert_eq!(cfg.harness_binary(), Path::new("C:/tools/claude.exe"));
    assert_eq!(cfg.margins(), margins_ok());
}

/// The captured map is the whole parent environment minus a denylist: proxy URLs with embedded
/// credentials, tokens the denylist does not name. A derived `Debug` printed every VALUE, so one
/// `{config:?}` in a log line at the composition root printed them all (ADR-0105 clause 7: the
/// child environment is where credentials live). `Config`, `ConfigValues` and `ChildEnv` render
/// the captured map's KEYS and a count, never a value; `Binder` derives over `Config` and inherits it.
///
/// RED (observed 2026-09-18) against the derived `Debug`: `ConfigValues' Debug must not print a
/// captured value (ADR-0105 clause 7): ConfigValues { captured_env: {"HTTPS_PROXY":
/// "http://user:s3cret-proxy-pass@proxy:3128", …`.
#[test]
fn debug_never_prints_a_captured_value() {
    let secret = "s3cret-proxy-pass";
    let pairs = [
        ("HTTPS_PROXY", "http://user:s3cret-proxy-pass@proxy:3128"),
        ("TERM", "xterm-256color-value"),
    ];
    let mut v = values();
    v.captured_env = pairs
        .iter()
        .map(|(k, val)| ((*k).to_owned(), (*val).to_owned()))
        .collect();
    let rendered = format!("{v:?}");
    assert!(
        !rendered.contains(secret) && !rendered.contains("xterm-256color-value"),
        "ConfigValues' Debug must not print a captured value (ADR-0105 clause 7): {rendered}"
    );
    assert!(
        rendered.contains("HTTPS_PROXY") && rendered.contains("TERM"),
        "the KEYS are printed, so a log line still says what was captured: {rendered}"
    );
    let cfg = Config::new(v).expect("constructs");
    let rendered = format!("{cfg:?}");
    assert!(
        !rendered.contains(secret) && !rendered.contains("xterm-256color-value"),
        "Config's Debug must not print a captured value (ADR-0105 clause 7): {rendered}"
    );
    assert!(rendered.contains("HTTPS_PROXY") && rendered.contains("TERM"));
    let env = rk_adapter_claude::env::child_env(&cfg);
    let rendered = format!("{env:?}");
    assert!(
        !rendered.contains(secret),
        "ChildEnv's Debug must not print a captured value: {rendered}"
    );
    let binder = rk_adapter_claude::binder::Binder::new(cfg);
    let rendered = format!("{binder:?}");
    assert!(
        !rendered.contains(secret),
        "Binder's Debug is Config's: {rendered}"
    );
}

/// The argv-cap measure (`command::command_line_units`) re-states `std`'s `make_command_line` for
/// a NON-batch program. When the program resolves to a `.bat`/`.cmd` — the npm shim form of
/// `claude` on many Windows installs — `std` instead builds `cmd.exe /d /c "<full path> …"` with
/// different quoting and a longer line, so the at-the-cap refusal would under-measure. The
/// precondition is enforced where the program is configured: a batch `harness_binary` is refused
/// at `Config::new`, naming the path.
///
/// RED (observed 2026-09-18) against the `Config::new` that accepted any path: `a C:/tools/claude.cmd
/// harness binary must be refused at construction (the cap measure is the non-batch command line);
/// got Ok(Config { … })`.
#[test]
fn harness_binary_that_is_a_batch_file_is_refused() {
    for batch in [
        "C:/tools/claude.cmd",
        "claude.bat",
        "C:/x/CLAUDE.CMD",
        "shim.Bat",
    ] {
        let mut v = values();
        v.harness_binary = Path::new(batch).to_path_buf();
        match Config::new(v) {
            Err(ConfigError::HarnessBinaryIsBatch { path }) => {
                assert_eq!(path, Path::new(batch).to_path_buf());
            }
            other => panic!(
                "a {batch} harness binary must be refused at construction (the cap measure is the non-batch command line); got {other:?}"
            ),
        }
    }
    for fine in [
        "claude",
        "claude.exe",
        "C:/Users/x/.local/bin/claude.exe",
        "/usr/bin/claude",
    ] {
        let mut v = values();
        v.harness_binary = Path::new(fine).to_path_buf();
        assert!(Config::new(v).is_ok(), "{fine} is not a batch file");
    }
}

/// AC-215: *"the register's floor is 2.1.259 [documented]; the pin is 2.1.277, above it, recorded
/// beside `TESTED_CLI_VERSION` in `Config`"* (§7c.5) — the floor is declared beside the pin, and
/// the pin is at or above it by a version-tuple compare (no float), so the probe's tag lift at
/// Slice 5 has the floor to read.
#[test]
fn permission_prompts_floor_is_recorded_beside_the_pin() {
    use rk_adapter_claude::config::PERMISSION_PROMPTS_FLOOR;
    assert_eq!(PERMISSION_PROMPTS_FLOOR, "2.1.259");
    let tuple = |v: &str| -> Vec<u32> {
        v.split('.')
            .map(|p| p.parse().unwrap_or_else(|e| panic!("{v}: {e}")))
            .collect()
    };
    assert_eq!(tuple(PERMISSION_PROMPTS_FLOOR).len(), 3);
    assert!(
        tuple(TESTED_CLI_VERSION) >= tuple(PERMISSION_PROMPTS_FLOOR),
        "the pin {TESTED_CLI_VERSION} is at or above the --permission-prompts floor {PERMISSION_PROMPTS_FLOOR} (AC-215)"
    );
    let src = std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/config.rs"))
        .expect("read src/config.rs");
    assert!(
        src.lines()
            .any(|l| l.trim_start() == "pub const PERMISSION_PROMPTS_FLOOR: &str = \"2.1.259\";"),
        "the floor is declared in the pin's form, beside it"
    );
}
