//! The binder — D-11: role tier → bound card → `ActivitySpec`, and the digest-keyed card store
//! `dispatch` reads the body from (`s7c` §7c.12; DECISIONS-2026-09-14 D-11; its routing record R-5
//! is OWED).
//!
//! **What it does, in order** ([`Binder::bind`]):
//!
//! 1. **Injects the role's tier** (AC-212; AC-118/AC-418: the authored card carries no `model` or
//!    `effort` — `card.rs` refuses one that does — so the values are traceable only through the
//!    role): a [`Role`] with no `model` is [`ConfigError::ModelUntraceable`]; on a tier that takes
//!    an effort (the lifted `Tier::accepts_effort`), a role with none is
//!    [`ConfigError::EffortUntraceable`]; on haiku, a role WITH one is
//!    [`ConfigError::EffortRefusedByTier`] (*"a spec error rather than a runtime one"*). AC-212's
//!    case (c) — an environment variable that would supply the value — is never read: nothing here
//!    looks at the `Config`'s captured map, and the argv the builder emits is the bound card's.
//! 2. **Runs every card-level and argv-level lint of §7c.4** by assembling the command through
//!    the one builder (`command::assemble`) with the bound card: the card-level lints over the
//!    bound card (`card::lint_card` — AC-493, AC-269, AC-214's explicit value; an `AuthoredCard`
//!    is a `pub` struct 7e's pack reader builds, so a card that never passed `from_file` is linted
//!    HERE), then AC-205 (`StructuredOutput`), AC-203, AC-132, and the argv cap — *"so a `Config`
//!    cannot hold a card that fails them"*. The user message is not part of the argv (AC-209), so
//!    the assembly at bind time measures the same command line every dispatch of this bound card
//!    will.
//! 3. **Materialises the [`ActivitySpec`]** the port takes — ADR-0099:61's five fields and no
//!    sixth: `card_digest` (SHA-256 over the bound card's `--agents` bytes — a digest, never the
//!    body), `tools` (the card's declared list, which is also what `system/init` is compared
//!    against at Slice 5 — AC-204), `schema` (the id), `timeout_secs`, `inputs` (the activity's,
//!    plus RT-21's key).
//! 4. **Puts the worktree on the spec** (RT-21): `inputs` gains the fixed key [`WORKTREE_KEY`]
//!    (`"worktree"`) whose value is the run-owned worktree path the kernel's activity runtime
//!    resolved from the store (AC-529's clause: *"never from cwd"*) — the binder puts it on every
//!    spec it materialises; a path it cannot fill (not UTF-8) or an activity whose inputs already
//!    carry the key is a bind-time [`ConfigError::Worktree`], never a silent overwrite. Inside
//!    `dispatch` (Slice 5) a spec without the key is `Err(Transport { kind: Spawn })` with its row —
//!    the residual RT-20 routes, since `dispatch` cannot return a `ConfigError`.
//! 5. **Stores the bound card by digest in the `Config`'s card store** (`Config::cards`;
//!    `config.rs`): the binder OWNS the `Config` while it binds ([`Binder::new`]) and hands it back
//!    with the store filled ([`Binder::into_config`]) — so *"a `Config` cannot hold a card that
//!    fails them"* (§7c.4) is literal: the only writer of the store is this method, after every
//!    lint. A `Bound` entry is the [`AgentCard`] plus every schema it was bound with by id, so
//!    `dispatch` reads the body, the route and the schema JSON by `spec.card_digest` +
//!    `spec.schema` and never from the spec — **how the card BODY reaches the adapter** is routed
//!    as this store and RAISED (§7c.22). Two binds of one card under one role are one digest and
//!    one entry (a second schema is added to it); a different tier is a different bound card; the
//!    same `--agents` bytes under the OTHER `cache_profile` are the same digest and a DIFFERENT
//!    card — a store conflict the bind refuses (`ConfigError::CardStoreConflict`), since the
//!    profile is not in the digested bytes and `dispatch` reads the route from the stored card.
//!
//! **What reaches `dispatch`** is therefore a bound, validated spec whose digest names a card in
//! the `Config` the harness was constructed with (`ClaudeHarness::new(binder.into_config(), token)`
//! at the composition root — Slice 5); a `ConfigError` is the return of `bind` and never of
//! `dispatch` (§7c.4). The user message and the rest of the activity's inputs travel on
//! `spec.inputs`; how Slice 5 renders them into the stdin payload is that slice's
//! (`command::CommandSpec::user_message` is the value it hands the builder).
//!
//! **Not the binder's**: reading a card file from a pack (7e's pack reader hands in an
//! [`AuthoredCard`]), choosing a role for an activity (the kernel's, via the pack), the schema's
//! contents (the composition root takes it from `rk_types::registry::EXPORTED` by name).

use std::path::Path;

use rk_types::activity::ActivitySpec;
use rk_types::typed::SchemaId;
use serde::Deserialize;

use crate::card::{AgentCard, AuthoredCard, Effort, Tier};
use crate::command::{CommandSpec, assemble};
use crate::config::{Config, ConfigError};

pub use crate::config::Bound;

/// The fixed key on `ActivitySpec.inputs` carrying the run-owned worktree path (RT-21).
///
/// Put on every spec by the binder, read into a `WorktreePath` at the top of `dispatch` (Slice 5)
/// and used as `Command::current_dir` — never the process's cwd (AC-529). A record naming the
/// channel is OWED.
pub const WORKTREE_KEY: &str = "worktree";

/// A role's tier declaration (AC-118: *"the tier is a role property the binder materialises"*).
///
/// Both members optional on the wire so the binder can refuse an absent one NAMING it (AC-212 a/b)
/// rather than as a serde "missing field"; closed (no member the record does not declare).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Role {
    pub model: Option<Tier>,
    pub effort: Option<Effort>,
}

/// The one output schema a dispatch validates against: its id (the spec's `schema`) and its JSON
/// (the `--json-schema` element; Slice 5's result validation).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schema {
    pub id: SchemaId,
    pub json: serde_json::Value,
}

/// One bind: the role (and its name, for the refusal), the authored card, the schema, the
/// activity's timeout, the run-owned worktree, and the activity's inputs as assembled.
#[derive(Debug, Clone)]
pub struct BindRequest<'a> {
    pub role_name: &'a str,
    pub role: &'a Role,
    pub card: &'a AuthoredCard,
    pub schema: &'a Schema,
    pub timeout_secs: u64,
    pub worktree: &'a Path,
    pub inputs: Vec<(String, serde_json::Value)>,
}

/// The binder: owns the validated `Config` while cards are bound into its store, and hands it back
/// when binding is done.
///
/// Constructed at the composition root beside the `Config`, before the harness. `Debug` is
/// `Config`'s — the captured map's keys, never a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binder {
    config: Config,
}

impl Binder {
    /// Take the validated `Config` (its card store empty or already holding earlier binds).
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// The `Config` as it stands — its lints passed, its store holding every bind so far.
    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// Hand the `Config` back with the card store filled — what `ClaudeHarness::new` takes.
    #[must_use]
    pub fn into_config(self) -> Config {
        self.config
    }

    /// Bind: inject the role's tier, run the card and argv lints through the builder, materialise
    /// the spec, store the bound card by digest (the module doc's five steps).
    ///
    /// # Errors
    /// [`ConfigError::ModelUntraceable`], [`ConfigError::EffortUntraceable`],
    /// [`ConfigError::EffortRefusedByTier`] (AC-212); every refusal of `command::assemble` (the
    /// card-level lints, AC-203, AC-205, AC-132, the cap); [`ConfigError::Worktree`] (RT-21);
    /// [`ConfigError::CardStoreConflict`] (the same bytes under the other profile).
    pub fn bind(&mut self, req: BindRequest<'_>) -> Result<ActivitySpec, ConfigError> {
        let config = &self.config;
        let card = req.card;
        let tier = req
            .role
            .model
            .ok_or_else(|| ConfigError::ModelUntraceable {
                card: card.name.clone(),
                role: req.role_name.to_owned(),
            })?;
        let effort = match (tier.accepts_effort(), req.role.effort) {
            (true, Some(e)) => Some(e.as_str().to_owned()),
            (true, None) => {
                return Err(ConfigError::EffortUntraceable {
                    card: card.name.clone(),
                    role: req.role_name.to_owned(),
                    tier: tier.as_str().to_owned(),
                });
            }
            (false, None) => None,
            (false, Some(_)) => {
                return Err(ConfigError::EffortRefusedByTier {
                    card: card.name.clone(),
                    role: req.role_name.to_owned(),
                    tier: tier.as_str().to_owned(),
                });
            }
        };
        let agent = AgentCard {
            name: card.name.clone(),
            description: card.description.clone(),
            prompt: card.prompt.clone(),
            tools: card.tools.clone(),
            model: tier.as_str().to_owned(),
            effort,
            permission_mode: card.permission_mode.clone(),
            cache_profile: card.cache_profile,
        };
        // The argv-level lints, through the one builder (§7c.5): the user message is stdin, not
        // argv (AC-209), so an empty one measures what every dispatch of this card will.
        assemble(
            config,
            &CommandSpec {
                card: &agent,
                schema: &req.schema.json,
                adopt_agent: true,
                strict_tools: &[],
                user_message: "",
            },
        )?;
        // RT-21: the worktree onto the inputs, refusing a collision or a path with no UTF-8 form.
        let worktree = req.worktree.to_str().ok_or_else(|| ConfigError::Worktree {
            detail: format!("{} is not valid UTF-8", req.worktree.display()),
        })?;
        let mut inputs = req.inputs;
        if inputs.iter().any(|(k, _)| k == WORKTREE_KEY) {
            return Err(ConfigError::Worktree {
                detail: format!(
                    "the activity's inputs already carry \"{WORKTREE_KEY}\" — the key is the \
                     binder's, and a value the activity supplied would be the process cwd by \
                     another route"
                ),
            });
        }
        inputs.push((
            WORKTREE_KEY.to_owned(),
            serde_json::Value::String(worktree.to_owned()),
        ));
        let spec = ActivitySpec {
            card_digest: agent.digest(),
            tools: agent.tools.clone(),
            schema: req.schema.id.clone(),
            timeout_secs: req.timeout_secs,
            inputs,
        };
        self.config
            .insert_card(agent, req.schema.id.clone(), req.schema.json.clone())?;
        Ok(spec)
    }
}
