//! **The card validator** — AC-383 (no `cache_profile`), AC-493 (empty or absent `tools`), AC-269
//! (the answer API absent from every card in the catalog — O-6: the fixture cards ARE the catalog),
//! AC-118/AC-418's leak at the card level, and the lifted `step.rs` tests (s7c §7c.2, §7c.12).
//!
//! **The marker-count belt is Slice 5's** (§7c.18, §7c.20 item 4); these are the `is_err()` halves.

use std::collections::BTreeMap;

use rk_adapter_claude::card::{AgentCard, AuthoredCard, CacheProfile, Effort, Tier};
use rk_adapter_claude::config::ConfigError;

use crate::support::{catalog_card, fixture, fixtures, red_card};

/// AC-383 (card-validator half): *"a card that declares neither [profile] is refused as a
/// configuration error before any dispatch"* — *"a fixture card with no profile that must fail"*.
///
/// RED (observed 2026-09-18) against the skeleton that defaulted a profile-less card to `static`:
/// `a card with no cache_profile must be refused (AC-383)`.
#[test]
fn ac383_card_without_profile_refused() {
    match red_card("no-profile") {
        Err(ConfigError::CacheProfileAbsent { card }) => assert_eq!(card, "no-profile"),
        Err(other) => panic!("the refusal is CacheProfileAbsent naming the card; got {other:?}"),
        Ok(card) => panic!(
            "a card with no cache_profile must be refused (AC-383); loaded as {:?}",
            card.cache_profile
        ),
    }
    // The two profiles the register names load; nothing else does.
    assert_eq!(
        catalog_card("golden-static").cache_profile,
        CacheProfile::Static
    );
    assert_eq!(
        catalog_card("golden-default").cache_profile,
        CacheProfile::Default
    );
    let text = fixture("cards/catalog/probe.json").replace("\"static\"", "\"append\"");
    assert!(
        AuthoredCard::from_file("probe", &text).is_err(),
        "a profile outside {{static, default}} is refused (AC-383 names two axes)"
    );
    // Slice 5 adds the marker-count belt here (§7c.18).
}

/// AC-493 (adapter half): *"a fixture mapping with `tools = []` and a second with `tools` absent,
/// both of which must fail"* — rejected naming the card, never passed on as an empty or absent
/// allowlist (ADR-0108 d2; L4.4: an absent list inherits everything).
///
/// RED (observed 2026-09-18) against the skeleton that read an absent `tools` as an empty list and
/// an empty list as a card: `tools-empty.json: a card with tools = [] must be refused (AC-493)`.
#[test]
fn ac493_empty_or_absent_tools_rejected() {
    match red_card("tools-empty") {
        Err(ConfigError::ToolsEmpty { card }) => assert_eq!(card, "tools-empty"),
        Err(other) => panic!("tools-empty.json: the refusal is ToolsEmpty; got {other:?}"),
        Ok(_) => panic!("tools-empty.json: a card with tools = [] must be refused (AC-493)"),
    }
    match red_card("tools-absent") {
        Err(ConfigError::ToolsAbsent { card }) => assert_eq!(card, "tools-absent"),
        Err(other) => panic!("tools-absent.json: the refusal is ToolsAbsent; got {other:?}"),
        Ok(_) => panic!("tools-absent.json: a card with no tools key must be refused (AC-493)"),
    }
    // A blank tool name is not a tool either.
    let text = fixture("cards/catalog/probe.json").replace("\"StructuredOutput\"", "\"\"");
    assert!(
        AuthoredCard::from_file("probe", &text).is_err(),
        "an empty tool name is refused"
    );
    // Slice 5 adds the marker-count belt here (§7c.18).
}

/// AC-269 (card-validator half; the build-configuration half is ARMED to 7d — no `rk-cli` exists):
/// *"a card-validator test over every card in the catalog asserting the answer path is absent"* —
/// the catalog is the fixture card set 7c ships (O-6), iterated from the directory so a card added
/// later is validated without being named here; and a red card naming the answer path must fail.
///
/// RED (observed 2026-09-18) against the lint-free skeleton: `names-answer.json: a card exposing
/// the answer API must be refused (AC-269)`.
#[test]
fn ac269_answer_api_absent_from_cards() {
    let dir = fixtures().join("cards/catalog");
    let mut count = 0;
    for entry in std::fs::read_dir(&dir).expect("cards/catalog/") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("a stem")
            .to_owned();
        let text = std::fs::read_to_string(&path).expect("read");
        let card = AuthoredCard::from_file(&stem, &text)
            .unwrap_or_else(|e| panic!("catalog card {stem} must validate: {e}"));
        for tool in &card.tools {
            assert!(
                !rk_adapter_claude::card::exposes_answer_api(tool),
                "catalog card {stem} exposes the answer/mint/resolve path through tool {tool} (AC-269)"
            );
        }
        count += 1;
    }
    assert!(
        count >= 4,
        "PREMISE: the catalog holds the probe card and the golden cards (got {count})"
    );
    assert!(
        catalog_card("probe")
            .tools
            .contains(&"StructuredOutput".to_owned()),
        "the probe card satisfies AC-205"
    );

    match red_card("names-answer") {
        Err(ConfigError::AnswerApiExposed { card, tool }) => {
            assert_eq!(card, "names-answer");
            assert_eq!(tool, "mcp__redkiln__answer");
        }
        Err(other) => panic!("the refusal is AnswerApiExposed naming the tool; got {other:?}"),
        Ok(_) => {
            panic!("names-answer.json: a card exposing the answer API must be refused (AC-269)")
        }
    }
    // The register names the PATH, not a spelling (O-6's RAISED note carries the matching rule):
    // a token is matched as a segment at any `_ / . : -` boundary and at a camelCase boundary, so
    // the hyphenated and camelCase spellings MCP tool names routinely carry are the path too.
    //
    // RED (observed 2026-09-18) against the `_ / . :`-only split: `rk-answer is the
    // answer/mint/resolve path in another spelling` — the hyphen and camel spellings all loaded.
    for tool in [
        "answer",
        "Answer",
        "ANSWER",
        "mint",
        "resolve",
        "mcp__tracker__resolve",
        "rk/mint",
        "rk-answer",
        "answer-v2",
        "mint-token",
        "mcp__tracker__resolve-issue",
        "AnswerApi",
        "answerQuestion",
        "XMLAnswer",
        "resolveIssue",
    ] {
        let text = fixture("cards/catalog/probe.json").replace(
            "\"StructuredOutput\"",
            &format!("\"StructuredOutput\", \"{tool}\""),
        );
        assert!(
            matches!(
                AuthoredCard::from_file("probe", &text),
                Err(ConfigError::AnswerApiExposed { .. })
            ),
            "{tool} is the answer/mint/resolve path in another spelling"
        );
    }
    // A tool whose name merely CONTAINS a token as part of a word is not the path.
    let text = fixture("cards/catalog/probe.json").replace(
        "\"StructuredOutput\"",
        "\"StructuredOutput\", \"Resolver\", \"mintmark\", \"Answers\", \"unanswered\", \"Minted\", \"ResolverSet\", \"Mint2\"",
    );
    assert!(
        AuthoredCard::from_file("probe", &text).is_ok(),
        "Resolver / mintmark / Answers / unanswered / Minted / ResolverSet / Mint2 are not the answer path (a digit continues a segment)"
    );
}

/// AC-118 / AC-418 at the card level: an AUTHORED card never carries `model` or `effort` — the tier
/// is the role's and the binder injects it (§7c.12; the register's glossary). A card that carries
/// one is refused naming the leaked member, so the binder can never see a tier it did not inject.
#[test]
fn authored_card_carrying_a_tier_is_refused() {
    match red_card("authored-with-model") {
        Err(ConfigError::TierLeak { card, member }) => {
            assert_eq!(card, "authored-with-model");
            assert_eq!(member, "model");
        }
        other => panic!("an authored card carrying `model` is a tier leak (AC-118); got {other:?}"),
    }
    let text = fixture("cards/catalog/probe.json").replace(
        "\"cache_profile\"",
        "\"effort\": \"high\", \"cache_profile\"",
    );
    match AuthoredCard::from_file("probe", &text) {
        Err(ConfigError::TierLeak { member, .. }) => assert_eq!(member, "effort"),
        other => {
            panic!("an authored card carrying `effort` is a tier leak (AC-418); got {other:?}")
        }
    }
    // A member the card record does not declare is refused too — the card file is a closed record.
    let text = fixture("cards/catalog/probe.json")
        .replace("\"cache_profile\"", "\"tool\": [], \"cache_profile\"");
    assert!(matches!(
        AuthoredCard::from_file("probe", &text),
        Err(ConfigError::CardFile { .. })
    ));
}

/// Lifted from `runner/src/step.rs` at the pin: `haiku_rejects_effort`.
#[test]
fn haiku_rejects_effort() {
    assert!(!Tier::Haiku.accepts_effort());
    assert!(Tier::Sonnet.accepts_effort());
    assert!(Tier::Opus.accepts_effort());
    assert_eq!(Tier::Haiku.as_str(), "haiku");
    assert_eq!(Effort::Xhigh.as_str(), "xhigh");
}

/// Lifted from `runner/src/step.rs` at the pin: `agent_card_renders_a_name_keyed_object` — with the
/// edits §7c.2 names: `tools` is required (AC-493), `model` is the injected tier (never optional),
/// `permissionMode` is required (AC-214).
#[test]
fn agent_card_renders_a_name_keyed_object() {
    let card = AgentCard {
        name: "reviewer".into(),
        description: "Adversarial verification gate".into(),
        prompt: "You are the review subagent.".into(),
        tools: vec!["Read".into(), "Bash".into(), "StructuredOutput".into()],
        model: "opus".into(),
        effort: Some("xhigh".into()),
        permission_mode: "bypassPermissions".into(),
        cache_profile: CacheProfile::Static,
    };
    let json: serde_json::Value = serde_json::from_str(&card.to_agents_json()).expect("valid json");

    // Keyed by name, and the name is not also an inner field.
    assert!(json.get("reviewer").is_some());
    assert!(json["reviewer"].get("name").is_none());
    // Model and effort ride in the card — the fix for RFC R2.
    assert_eq!(json["reviewer"]["model"], "opus");
    assert_eq!(json["reviewer"]["effort"], "xhigh");
    assert_eq!(json["reviewer"]["tools"][1], "Bash");
    assert_eq!(json["reviewer"]["permissionMode"], "bypassPermissions");
    // The profile is the adapter's routing key, not a card member the CLI sees.
    assert!(json["reviewer"].get("cache_profile").is_none());
    assert!(json["reviewer"].get("cacheProfile").is_none());
    // An absent optional is omitted, not null.
    let no_effort = AgentCard {
        effort: None,
        ..card.clone()
    };
    let json: serde_json::Value =
        serde_json::from_str(&no_effort.to_agents_json()).expect("valid json");
    assert!(json["reviewer"].get("effort").is_none());
    // The digest is over those bytes: an equal card re-built from the same members digests the same.
    let rebuilt = AgentCard {
        name: "reviewer".into(),
        description: "Adversarial verification gate".into(),
        prompt: "You are the review subagent.".into(),
        tools: vec!["Read".into(), "Bash".into(), "StructuredOutput".into()],
        model: "opus".into(),
        effort: Some("xhigh".into()),
        permission_mode: "bypassPermissions".into(),
        cache_profile: CacheProfile::Static,
    };
    assert_eq!(card.digest(), rebuilt.digest());
    assert_ne!(card.digest(), no_effort.digest());
    // The map form is one key, the name.
    let map: BTreeMap<String, serde_json::Value> =
        serde_json::from_str(&card.to_agents_json()).expect("map");
    assert_eq!(map.len(), 1);
}
