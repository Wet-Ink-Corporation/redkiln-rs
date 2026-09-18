//! **The binder (D-11)** — AC-212 (model/effort from the bound card only), the spec it materialises
//! (`card_digest`, `tools`, `schema`, `timeout_secs`, `inputs` with RT-21's `"worktree"` key), and
//! the digest-keyed card store — `Config::cards`, which only the binder fills — `dispatch` reads
//! the body from (s7c §7c.12, §7c.4).
//!
//! **The marker-count belt is Slice 5's** (§7c.18, §7c.20 item 4); these are the `is_err()` halves.
//! `dispatch_cwd_is_the_spec_worktree_never_process_cwd` (the key read INSIDE `dispatch`) is Slice
//! 5's too; what the binder puts on the spec is asserted here.

use rk_types::digest::sha256;
use rk_types::typed::SchemaId;

use rk_adapter_claude::binder::{BindRequest, Binder, Role, Schema, WORKTREE_KEY};
use rk_adapter_claude::card::{AgentCard, AuthoredCard, Tier};
use rk_adapter_claude::command::{CommandSpec, assemble};
use rk_adapter_claude::config::ConfigError;

use crate::support::{catalog_card, config, config_with_env, role, schema, worktree};

fn request<'a>(
    role_name: &'a str,
    role: &'a Role,
    card: &'a AuthoredCard,
    schema: &'a Schema,
    worktree: &'a std::path::Path,
) -> BindRequest<'a> {
    BindRequest {
        role_name,
        role,
        card,
        schema,
        timeout_secs: 300,
        worktree,
        inputs: vec![("item".to_owned(), serde_json::json!("issue-81"))],
    }
}

/// AC-212: *"unit tests for (a) a bound card whose role declares no `effort`, (b) a bound card
/// whose role declares no `model`, (c) an env var set while the bound card is complete (the env
/// value must not appear in argv)"* — (a) and (b) refused at bind, (c) a `Config` map carrying the
/// value (ADR-0083: never `set_var`) whose value reaches no element; plus a haiku role carrying an
/// effort, which the tier rejects (the lifted `accepts_effort`).
///
/// RED (observed 2026-09-18) against the skeleton that passed an absent effort through and fell
/// back to sonnet for an absent model: `(a) a sonnet role declaring no effort must be refused
/// (AC-212); got Ok(ActivitySpec { card_digest: Digest("74df61…"), tools: ["Read", "Grep", "Glob",
/// "StructuredOutput"], … })`.
#[test]
fn ac212_model_effort_from_bound_card_only() {
    let schema = schema();
    let card = catalog_card("reviewer");
    let wt = worktree();
    let mut binder = Binder::new(config());

    // (a) a role that declares no effort, on a tier that takes one.
    let r = role("sonnet-no-effort");
    match binder.bind(request("sonnet-no-effort", &r, &card, &schema, &wt)) {
        Err(ConfigError::EffortUntraceable {
            card: c,
            role: rn,
            tier,
        }) => {
            assert_eq!(
                (c.as_str(), rn.as_str(), tier.as_str()),
                ("reviewer", "sonnet-no-effort", "sonnet")
            );
        }
        other => {
            panic!("(a) a sonnet role declaring no effort must be refused (AC-212); got {other:?}")
        }
    }
    // (b) a role that declares no model.
    let r = role("no-model");
    match binder.bind(request("no-model", &r, &card, &schema, &wt)) {
        Err(ConfigError::ModelUntraceable { card: c, role: rn }) => {
            assert_eq!((c.as_str(), rn.as_str()), ("reviewer", "no-model"));
        }
        other => panic!("(b) a role declaring no model must be refused (AC-212); got {other:?}"),
    }
    // An effort on a tier that rejects it is a spec error, not a runtime one (the lifted rule).
    let r = role("haiku-with-effort");
    match binder.bind(request("haiku-with-effort", &r, &card, &schema, &wt)) {
        Err(ConfigError::EffortRefusedByTier { tier, .. }) => assert_eq!(tier, "haiku"),
        other => {
            panic!("an effort on haiku is refused (haiku rejects the parameter); got {other:?}")
        }
    }
    assert!(
        binder.config().cards().is_empty(),
        "three refusals stored nothing"
    );

    // (c) the complete role binds; an env value in the captured map reaches no argv element.
    let cfg_env = config_with_env(&[
        ("ANTHROPIC_MODEL", "opus"),
        ("CLAUDE_CODE_EFFORT_LEVEL", "max"),
        ("ANTHROPIC_SMALL_FAST_MODEL", "haiku"),
    ])
    .expect("none of those names is denylisted");
    let r = role("sonnet-high");
    let mut binder = Binder::new(cfg_env);
    let spec = binder
        .bind(request("sonnet-high", &r, &card, &schema, &wt))
        .expect("(c) the complete role binds");
    let cfg_env = binder.into_config();
    let bound = cfg_env
        .bound(&spec.card_digest)
        .expect("the store holds the bound card");
    assert_eq!(
        bound.card.model, "sonnet",
        "the model is the role's, never the environment's"
    );
    assert_eq!(bound.card.effort.as_deref(), Some("high"));
    let built = assemble(
        &cfg_env,
        &CommandSpec {
            card: &bound.card,
            schema: &bound.schemas[&spec.schema],
            adopt_agent: true,
            strict_tools: &[],
            user_message: "",
        },
    )
    .expect("builds");
    let i = built
        .argv
        .iter()
        .position(|a| a == "--model")
        .expect("--model");
    assert_eq!(built.argv[i + 1], "sonnet");
    let i = built
        .argv
        .iter()
        .position(|a| a == "--effort")
        .expect("--effort");
    assert_eq!(built.argv[i + 1], "high");
    assert!(
        !built.argv.iter().any(|a| a == "opus" || a == "max"),
        "the env values appear in no element"
    );
    // Slice 5 adds the marker-count belt here (§7c.18).
}

/// D-11 / §7c.12: the binder materialises ADR-0099:61's five fields — `card_digest` (the bound
/// card's `--agents` bytes, digested), `tools` (the card's declared list), `schema` (the id),
/// `timeout_secs`, `inputs` (the activity's plus RT-21's `"worktree"`) — and the `Config`'s
/// digest-keyed store holds the bound card and the schemas it was bound with, so `dispatch` reads
/// the body by digest and never from the spec.
#[test]
fn bind_materialises_the_spec_and_the_digest_keyed_store() {
    let cfg = config();
    let schema = schema();
    let card = catalog_card("probe");
    let r = role("haiku");
    let wt = worktree();
    assert!(
        cfg.cards().is_empty(),
        "PREMISE: a fresh Config holds no card"
    );
    let mut binder = Binder::new(cfg);
    let spec = binder
        .bind(request("haiku", &r, &card, &schema, &wt))
        .expect("the probe card binds");

    assert_eq!(spec.tools, card.tools);
    assert_eq!(spec.schema, schema.id);
    assert_eq!(spec.timeout_secs, 300);
    let bound = binder
        .config()
        .bound(&spec.card_digest)
        .expect("keyed by digest");
    assert_eq!(bound.card.name, "probe");
    assert_eq!(bound.card.model, "haiku");
    assert_eq!(bound.card.effort, None, "haiku takes no effort");
    assert_eq!(bound.schemas.len(), 1);
    assert_eq!(
        bound.schemas[&schema.id], schema.json,
        "the schema JSON is stored by the id the spec carries"
    );
    assert_eq!(
        spec.card_digest,
        sha256(bound.card.to_agents_json().as_bytes()),
        "the digest is over the --agents bytes"
    );
    assert_eq!(spec.card_digest, bound.card.digest());

    // RT-21: the worktree travels on the spec under the fixed key, as a string, beside the
    // activity's own inputs — and the binder never overwrites a key the activity carries.
    let wt_value = spec
        .inputs
        .iter()
        .find(|(k, _)| k == WORKTREE_KEY)
        .map(|(_, v)| v.clone())
        .expect("the binder puts \"worktree\" on every spec it materialises");
    assert_eq!(wt_value, serde_json::json!(wt.to_str().expect("utf-8")));
    assert_eq!(
        spec.inputs[0],
        ("item".to_owned(), serde_json::json!("issue-81"))
    );
    assert_eq!(WORKTREE_KEY, "worktree");
    let mut req = request("haiku", &r, &card, &schema, &wt);
    req.inputs
        .push((WORKTREE_KEY.to_owned(), serde_json::json!("elsewhere")));
    assert!(
        matches!(binder.bind(req), Err(ConfigError::Worktree { .. })),
        "an activity input that already carries \"worktree\" is a collision the binder refuses, never a silent overwrite"
    );

    // Two binds of one card under one role yield one digest and one store entry — a second schema
    // is added to it (one card, two activities); a different tier is a different bound card.
    let spec2 = binder
        .bind(request("haiku", &r, &card, &schema, &wt))
        .expect("binds again");
    assert_eq!(spec2.card_digest, spec.card_digest);
    let other = Schema {
        id: SchemaId::new("probe-two"),
        json: serde_json::json!({ "type": "object", "properties": { "n": { "type": "integer" } } }),
    };
    let spec2b = binder
        .bind(request("haiku", &r, &card, &other, &wt))
        .expect("binds with a second schema");
    assert_eq!(
        spec2b.card_digest, spec.card_digest,
        "the digest is the card's, not the schema's"
    );
    let sonnet = role("sonnet-high");
    let spec3 = binder
        .bind(request("sonnet-high", &sonnet, &card, &schema, &wt))
        .expect("binds under sonnet");
    assert_ne!(spec3.card_digest, spec.card_digest);
    let cfg = binder.into_config();
    assert_eq!(cfg.cards().len(), 2, "two bound cards: haiku and sonnet");
    let haiku = cfg.bound(&spec.card_digest).expect("stored");
    assert_eq!(haiku.schemas.len(), 2, "one entry, both schemas");
    assert_eq!(haiku.schemas[&other.id], other.json);
    assert_eq!(
        cfg.bound(&spec3.card_digest).expect("stored").card.model,
        Tier::Sonnet.as_str()
    );
}

/// The bind-time lints are the builder's (§7c.4: "so a `Config` cannot hold a card that fails
/// them"): a card the builder refuses never enters the store.
#[test]
fn a_card_the_builder_refuses_never_enters_the_store() {
    let schema = schema();
    let red = crate::support::red_card("no-structured-output").expect("loads at the card level");
    let r = role("haiku");
    let wt = worktree();
    let mut binder = Binder::new(config());
    let err = binder
        .bind(request("haiku", &r, &red, &schema, &wt))
        .expect_err("StructuredOutput absent is refused at bind (AC-205)");
    assert!(matches!(err, ConfigError::StructuredOutputAbsent { .. }));
    let would_be = AgentCard {
        name: red.name.clone(),
        description: red.description.clone(),
        prompt: red.prompt.clone(),
        tools: red.tools.clone(),
        model: "haiku".to_owned(),
        effort: None,
        permission_mode: red.permission_mode.clone(),
        cache_profile: red.cache_profile,
    };
    let cfg = binder.into_config();
    assert!(
        cfg.bound(&would_be.digest()).is_none(),
        "nothing was stored for the refused card"
    );
    assert!(
        cfg.cards().is_empty(),
        "a Config cannot hold a card that fails the lints (§7c.4)"
    );
}

/// The card store is keyed by the digest of the `--agents` bytes, and `cache_profile` is NOT in
/// those bytes (it is the adapter's routing key, `#[serde(skip)]`) — so two cards equal in every
/// CLI-facing member and different in profile are ONE digest. `dispatch` reads the delivery route
/// from `cards[digest].card.cache_profile`, so a second bind that silently kept the first entry
/// would deliver the second activity under the wrong AC-383 route. The store REFUSES the
/// conflict, naming the digest, and keeps the first entry as it was.
///
/// RED (observed 2026-09-18) against `insert_card`'s `or_insert_with`: `a second bind of the same
/// --agents bytes under the OTHER profile is a store conflict, refused (AC-383); got Ok(…)` — the
/// bind returned the first digest and the store still carried `Static`.
#[test]
fn a_card_store_conflict_on_profile_is_refused() {
    let schema = schema();
    let r = role("haiku");
    let wt = worktree();
    let first = catalog_card("golden-static");
    let mut second = first.clone();
    second.cache_profile = rk_adapter_claude::card::CacheProfile::Default;
    assert_ne!(first, second, "PREMISE: the two differ in profile only");
    let mut binder = Binder::new(config());
    let spec = binder
        .bind(request("haiku", &r, &first, &schema, &wt))
        .expect("the first binds");
    match binder.bind(request("haiku", &r, &second, &schema, &wt)) {
        Err(ConfigError::CardStoreConflict { card, digest }) => {
            assert_eq!(card, "golden-static");
            assert_eq!(digest, spec.card_digest);
        }
        other => panic!(
            "a second bind of the same --agents bytes under the OTHER profile is a store conflict, refused (AC-383); got {other:?}"
        ),
    }
    let cfg = binder.into_config();
    assert_eq!(cfg.cards().len(), 1, "the refused bind stored nothing");
    assert_eq!(
        cfg.bound(&spec.card_digest)
            .expect("the first entry stands")
            .card
            .cache_profile,
        rk_adapter_claude::card::CacheProfile::Static,
        "the first entry's route is untouched"
    );
}

/// §7c.12: the binder *"runs every card-level and argv-level lint of §7c.4 at bind time"*, and
/// §7c.4: *"so a `Config` cannot hold a card that fails them"*. `AuthoredCard`'s members are `pub`
/// (the pack reader of 7e constructs one), so a card built by literal never passed
/// `AuthoredCard::from_file`'s lints — the binder must run them itself, or the invariant is a
/// doc comment. Three literal-built cards that fail a card-level lint: an empty
/// `permission_mode` (AC-214), a tool on the answer path (AC-269), an empty tool list (AC-493).
///
/// RED (observed 2026-09-18) against the binder that ran only the builder's lints: `a literal-built
/// card with permission_mode "" must be refused at bind (AC-214); got Ok(ActivitySpec { … })` —
/// the argv carried `--permission-mode ""` and the store held the card.
#[test]
fn a_literal_built_card_that_fails_the_card_lints_never_enters_the_store() {
    type Case = (&'static str, AuthoredCard, fn(&ConfigError) -> bool);
    let schema = schema();
    let r = role("haiku");
    let wt = worktree();
    let base = catalog_card("golden-static");
    let cases: Vec<Case> = vec![
        (
            "permission_mode \"\"",
            AuthoredCard {
                permission_mode: String::new(),
                ..base.clone()
            },
            |e| matches!(e, ConfigError::PermissionModeAbsent { .. }),
        ),
        (
            "a tool on the answer path",
            AuthoredCard {
                tools: vec!["StructuredOutput".to_owned(), "answer".to_owned()],
                ..base.clone()
            },
            |e| matches!(e, ConfigError::AnswerApiExposed { .. }),
        ),
        (
            "an empty tool list",
            AuthoredCard {
                tools: Vec::new(),
                ..base.clone()
            },
            |e| matches!(e, ConfigError::ToolsEmpty { .. }),
        ),
        (
            "an empty name",
            AuthoredCard {
                name: String::new(),
                ..base
            },
            |e| matches!(e, ConfigError::CardNameEmpty),
        ),
    ];
    for (what, card, is_the_variant) in cases {
        let mut binder = Binder::new(config());
        match binder.bind(request("haiku", &r, &card, &schema, &wt)) {
            Err(e) => assert!(
                is_the_variant(&e),
                "{what}: refused by the card-level lint that names it; got {e:?}"
            ),
            Ok(spec) => panic!(
                "a literal-built card with {what} must be refused at bind (§7c.12); got Ok({spec:?})"
            ),
        }
        assert!(
            binder.config().cards().is_empty(),
            "{what}: a Config cannot hold a card that fails the lints (§7c.4)"
        );
    }
}
