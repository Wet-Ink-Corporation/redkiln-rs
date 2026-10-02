//! **The binder (D-11)** — AC-212 (model/effort from the bound card only), the spec it materialises
//! (`card_digest`, `tools`, `schema`, `timeout_secs`, `inputs` with RT-21's `"worktree"` key), and
//! the digest-keyed card store — `Config::cards`, which only the binder fills — `dispatch` reads
//! the body from (s7c §7c.12, §7c.4).
//!
//! Each refusal is the `is_err()` half plus, since Slice 5, the marker-count belt (§7c.18): on an
//! `Ok` the test dispatches the spec once through the fixture child and asserts zero markers.
//! `dispatch_cwd_is_the_spec_worktree_never_process_cwd` (the key read INSIDE `dispatch`) is in
//! `dispatch.rs`; what the binder puts on the spec is asserted here.

use rk_types::digest::sha256;
use rk_types::typed::SchemaId;

use rk_adapter_claude::binder::{BindRequest, Binder, Role, Schema, WORKTREE_KEY};
use rk_adapter_claude::card::{AgentCard, AuthoredCard, Models};
use rk_adapter_claude::command::{CommandSpec, assemble};
use rk_adapter_claude::config::ConfigError;

use crate::support::{
    Markers, belt_dispatch_spec, catalog_card, config, config_with_env, models, role, schema,
    spawn_config, worktree,
};

/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)* The fixture `[models]` table every `request` binds
/// through, leaked once so a `BindRequest` can borrow it for any lifetime.
fn fixture_models() -> &'static Models {
    static MODELS: std::sync::OnceLock<Models> = std::sync::OnceLock::new();
    MODELS.get_or_init(models)
}

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
        activity_id_at_version: None,
        models: fixture_models(),
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
    // A REAL temp worktree, so the belt's dispatch has a directory to spawn into (a fixed
    // non-existent path would make every belt count 0 for the wrong reason — a spawn failure).
    let wt_dir = tempfile::tempdir().expect("a temp worktree");
    let wt = wt_dir.path().to_path_buf();
    // The binder sits over a SPAWN config (the fixture child in `exit:0` mode, a marker dir) so the
    // belt below can dispatch an `Ok(spec)` and every refusal can assert nothing spawned.
    let markers = Markers::new();
    let mut binder = Binder::new(spawn_config(&markers, "exit:0"));
    // The marker-count belt (§7c.18; Slice 5): a spec the binder should have refused, dispatched
    // once through the fixture child — the marker file is the red.
    // RED (belt half, observed 2026-09-18) against `Binder::bind` mutated to pass an absent effort
    // through and default an absent model to sonnet: `belt (a): a bind that should have refused
    // produced a spec and a marker file appeared (AC-212)`: left `1`, right `0`.
    let belt = |case: &str, binder: &Binder, spec: &rk_types::activity::ActivitySpec| {
        assert_eq!(
            belt_dispatch_spec(binder.config().clone(), spec, &markers),
            0,
            "belt ({case}): a bind that should have refused produced a spec and a marker file \
             appeared (AC-212)"
        );
    };

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
        Ok(spec) => {
            belt("a", &binder, &spec);
            panic!(
                "(a) a sonnet role declaring no effort must be refused (AC-212); got Ok({spec:?})"
            )
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
        Ok(spec) => {
            belt("b", &binder, &spec);
            panic!("(b) a role declaring no model must be refused (AC-212); got Ok({spec:?})")
        }
        other => panic!("(b) a role declaring no model must be refused (AC-212); got {other:?}"),
    }
    // An effort on a tier that rejects it is a spec error, not a runtime one (the lifted rule).
    let r = role("haiku-with-effort");
    match binder.bind(request("haiku-with-effort", &r, &card, &schema, &wt)) {
        Err(ConfigError::EffortRefusedByTier { tier, .. }) => assert_eq!(tier, "haiku"),
        Ok(spec) => {
            belt("haiku-with-effort", &binder, &spec);
            panic!("an effort on haiku is refused (haiku rejects the parameter); got Ok({spec:?})")
        }
        other => {
            panic!("an effort on haiku is refused (haiku rejects the parameter); got {other:?}")
        }
    }
    assert!(
        binder.config().cards().is_empty(),
        "three refusals stored nothing"
    );
    assert_eq!(markers.count(), 0, "three refusals spawned nothing");

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
    // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)* The role names the
    // row key; the bound card and `--model` carry the row's full id.
    assert_eq!(
        bound.card.model, "claude-sonnet-5",
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
            settings: None,
        },
    )
    .expect("builds");
    let i = built
        .argv
        .iter()
        .position(|a| a == "--model")
        .expect("--model");
    assert_eq!(built.argv[i + 1], "claude-sonnet-5");
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
    // (c) with the bound card INCOMPLETE (a haiku role with no effort is complete; a sonnet role
    // with no effort is not — case (a)'s red again, through the belt): red by a marker file.
    let markers = Markers::new();
    let mut binder = Binder::new(spawn_config(&markers, "exit:0"));
    let r = role("sonnet-no-effort");
    if let Ok(spec) = binder.bind(request("sonnet-no-effort", &r, &card, &schema, &wt)) {
        assert_eq!(
            belt_dispatch_spec(binder.config().clone(), &spec, &markers),
            0,
            "belt (c, incomplete): the incomplete bound card produced a spec and a marker file \
             appeared (AC-212)"
        );
        panic!("(c) an incomplete bound card must be refused (AC-212)");
    }
    assert_eq!(markers.count(), 0);
}

/// D-11 / §7c.12: the binder materialises ADR-0099:61's five fields (*(dated 2026-09-29, s8f build, package S8F-P7; ADR-0190 decision 2, proposed; D21-107.)*
/// plus `activity_id_at_version`, copied from the request) — `card_digest` (the bound
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
    // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.3.)* The role names the
    // row key; the bound card and `--model` carry the row's full id.
    assert_eq!(bound.card.model, "claude-haiku-4-5-20251001");
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
        models()["sonnet"].id
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

/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models.)* `s8h` §8h.3 (D21-42 15a, 15b;
/// ADR-0197 decisions 4 and 5, proposed): the role names a `[models]` row KEY, and the row's FULL
/// id travels everywhere the tier word did — `AgentCard.model`, the `--model` element, the
/// `--agents` JSON and so the card digest (`ActivitySpec.card_digest`, which enters the per-step
/// memo key). Two ids under one row key are two digests; the row key reaches no argv element.
///
/// RED (observed 2026-10-02, through D21-47's wrapper): with the binder writing the row key,
/// *"the bound card carries the row's full id"*, left `"sonnet"`, right `"claude-sonnet-5"`.
#[test]
fn the_full_model_id_reaches_argv_and_the_card_digest() {
    let schema = schema();
    let card = catalog_card("reviewer");
    let wt = worktree();
    let r = role("sonnet-high");
    assert_eq!(
        r.model.as_deref(),
        Some("sonnet"),
        "PREMISE: the role names the row key"
    );
    let mut other_models = models();
    other_models.get_mut("sonnet").expect("a sonnet row").id = "claude-sonnet-4-6".to_owned();
    let mut digests = std::collections::BTreeSet::new();
    for (table, want) in [
        (models(), "claude-sonnet-5"),
        (other_models, "claude-sonnet-4-6"),
    ] {
        let mut binder = Binder::new(config());
        let spec = binder
            .bind(BindRequest {
                models: &table,
                ..request("sonnet-high", &r, &card, &schema, &wt)
            })
            .expect("binds");
        let cfg = binder.into_config();
        let bound = cfg.bound(&spec.card_digest).expect("stored");
        assert_eq!(
            bound.card.model, want,
            "the bound card carries the row's full id"
        );
        assert_eq!(
            spec.card_digest,
            sha256(bound.card.to_agents_json().as_bytes()),
            "the digest is over the --agents bytes"
        );
        let agents: serde_json::Value =
            serde_json::from_str(&bound.card.to_agents_json()).expect("JSON");
        assert_eq!(
            agents["reviewer"]["model"], want,
            "the --agents JSON carries the id"
        );
        let built = assemble(
            &cfg,
            &CommandSpec {
                card: &bound.card,
                schema: &bound.schemas[&spec.schema],
                adopt_agent: true,
                strict_tools: &[],
                user_message: "",
                settings: None,
            },
        )
        .expect("builds");
        let i = built
            .argv
            .iter()
            .position(|a| a == "--model")
            .expect("--model");
        assert_eq!(
            built.argv[i + 1],
            want,
            "the --model element is the full id"
        );
        assert!(
            !built.argv.iter().any(|a| a == "sonnet"),
            "the row key reaches no argv element"
        );
        digests.insert(spec.card_digest);
    }
    assert_eq!(
        digests.len(),
        2,
        "two ids under one row key are two card digests"
    );
}

/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models.)* `s8h` §8h.3: a role naming a
/// key absent from the pack's `[models]` table is refused at bind, naming the role and the key —
/// the full id has no source, and nothing falls back to the key. Control: the same role binds over
/// a table that carries the key.
///
/// RED (observed 2026-10-02, through D21-47's wrapper): *"a role naming a key absent from [models]
/// must be refused at bind (s8h §8h.3), got Ok(ActivitySpec { … })"*.
#[test]
fn a_tier_absent_from_the_models_table_is_refused() {
    let schema = schema();
    let card = catalog_card("reviewer");
    let wt = worktree();
    let r = role("sonnet-high");
    let mut table = models();
    table.remove("sonnet");
    assert!(
        table.contains_key("opus"),
        "PREMISE: the table is otherwise intact"
    );
    let mut binder = Binder::new(config());
    match binder.bind(BindRequest {
        models: &table,
        ..request("sonnet-high", &r, &card, &schema, &wt)
    }) {
        Err(ConfigError::ModelKeyAbsent { card, role, key }) => {
            assert_eq!(
                (card.as_str(), role.as_str(), key.as_str()),
                ("reviewer", "sonnet-high", "sonnet")
            );
        }
        other => panic!(
            "a role naming a key absent from [models] must be refused at bind (s8h §8h.3), got \
             {other:?}"
        ),
    }
    assert!(
        binder.config().cards().is_empty(),
        "nothing entered the store"
    );
    binder
        .bind(request("sonnet-high", &r, &card, &schema, &wt))
        .expect("CONTROL: the same role binds over a table that carries the key");
}

/// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; `s8h` §8h.6; D21-137 (g), TAKEN
/// PROVISIONALLY.)* Binds over a worktree under `%TEMP%` and one under `target/tmp` both SUCCEED
/// now that the settings travel as a file (inline, P3 measured them at about 1,068,619 and 243,048
/// units against RT-1's 32,767), and bind WRITES NOTHING: the settings directory is as empty after
/// both binds as before (D21-137 item 4: bind measures a stand-in path).
///
/// RED (observed 2026-10-02, through D21-47's wrapper, against the inline scaffold): *"a bind over a
/// %TEMP% worktree succeeds (D21-137 (g)): ArgvAtOrOverCap { units: …, cap: 32767 }"*.
#[test]
fn binds_over_temp_and_target_tmp_worktrees_succeed_and_write_nothing() {
    let settings = tempfile::tempdir().expect("a settings dir");
    let temp_wt = tempfile::tempdir().expect("a worktree under %TEMP%");
    let target_tmp = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(&target_tmp).expect("target/tmp");
    let tmp_wt = tempfile::tempdir_in(&target_tmp).expect("a worktree under target/tmp");
    let schema = schema();
    let card = catalog_card("probe");
    let r = role("haiku");
    let mut binder = Binder::new(config().with_settings_dir(settings.path().to_path_buf()));
    for (what, wt) in [("%TEMP%", temp_wt.path()), ("target/tmp", tmp_wt.path())] {
        binder
            .bind(request("haiku", &r, &card, &schema, wt))
            .unwrap_or_else(|e| {
                panic!("a bind over a {what} worktree succeeds (D21-137 (g)): {e:?}")
            });
    }
    assert_eq!(binder.config().cards().len(), 1, "one card, bound twice");
    let written = std::fs::read_dir(settings.path())
        .expect("the settings dir lists")
        .count();
    assert_eq!(written, 0, "bind writes nothing (D21-137 item 4)");
}
