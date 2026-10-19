//! The card — its tier vocabulary, the authored card record, and the bound card as `--agents`
//! accepts it (`s7c` §7c.2's lift table, §7c.12).
//!
//! **Lifted, with edits, from `runner/src/step.rs` in `redkiln-corpus` at
//! `61f590e51cbb00301c310e46aab615563a7787b8`** (the SHA in `CORPUS_PIN`; blob
//! `71c32547c080a317103db43f75769a42c0718a0d`, 348 lines). That file was the spike runner's *"what a
//! dispatch is, and how it can fail"* — **built and run over the programme's recorded dispatches
//! (E3's two arms, F7, F15, F20) and exercised, not validated against an oracle** — so what travels
//! here is the shape the CLI accepted at the pin, not a warrant (PIA-036, PIA-062).
//!
//! **What is lifted** (the table's `step.rs` row): `Tier`, [`Effort`] and
//! `Tier::accepts_effort` (*"haiku rejects the parameter, so an effort on haiku is a spec error
//! rather than a runtime one"* — the lifted rule the binder enforces; *(dated 2026-10-02, s8h
//! build, package P3-adapter-argv-models)* `Tier` has since left the binding path, and the rule is
//! the `[models]` row's [`ModelRow::accepts_effort`]); [`AgentCard`] and its
//! [`AgentCard::to_agents_json`] (*"a single-key object keyed by name"*, confirmed against v2.1.258
//! — the shape that retired RFC R2, *"effort has no `--agents` equivalent"*); and the `StepSpec`
//! fields the command builder needs, which are now [`AgentCard`]'s own members plus
//! `command::CommandSpec`'s (`adopt_agent`, the schema, the user message).
//!
//! **Edits, each forced by a rule the corpus version predates:**
//!
//! 1. **`tools` is required and non-empty** (AC-493). The runner's `tools: Option<Vec<String>>`
//!    with `skip_serializing_if` let a card with no tools reach the CLI, which then *"inherits every
//!    tool when `tools:` is absent"* (shipmates L4.4 — fail open). Here an absent or empty list is
//!    refused at [`AuthoredCard::from_file`] naming the card, and [`AgentCard::tools`] is a
//!    `Vec<String>` the serialiser always writes.
//! 2. **`cache_profile: static | default` is required** (AC-383): a card that declares neither is
//!    refused before any dispatch, and the profile selects the delivery route (`command.rs`;
//!    decision #19 of 2026-09-11 — `default` is F28's measured arm). It is the adapter's routing
//!    key, `#[serde(skip)]` on the CLI-facing card: the CLI never sees it.
//! 3. **`model` is required and `permissionMode` is required** on the bound card. The runner's
//!    `model: Option<String>` rode a `StepSpec.model: Tier` that every experiment pinned; here the
//!    tier is the ROLE's, injected by the binder (§7c.12; AC-212), and an [`AuthoredCard`] carrying
//!    `model` or `effort` is a tier leak refused naming the member (AC-118, AC-418). `permissionMode`
//!    was `Option` and the CLI's default filled its absence; AC-214 says *"there is no implicit
//!    default"*, so a card silent on it is refused at load.
//! 4. **The card record is closed** (`#[serde(deny_unknown_fields)]` on [`CardFile`]): a card is
//!    configuration, and a misspelt key is refused, not ignored. (AC-201's ban on that attribute is
//!    module-scoped to the stream decoder and `usage.rs` — a card file is not a stream frame.)
//! 5. **AC-269's validator lives here**: no card exposes the answer, mint or resolve path —
//!    [`lint_card`] refuses a tool whose name spells one of [`ANSWER_API_TOKENS`] as a segment
//!    (`answer`, `mcp__redkiln__answer`, `rk/mint`, `rk-answer`, `answerQuestion` — the rule is on
//!    the constant), naming the tool. It runs at load ([`AuthoredCard::from_file`]) and again at
//!    bind (through `command::assemble`), so a card built by literal is linted too. The catalog it
//!    runs over at 7c is the fixture card set (O-6; `ac269_…` iterates the directory).
//!
//! **NOT lifted** (the table's last column): `StepError` (six variants, including `UsageLimit` and a
//! unit `Cancelled`) — the adapter's failures are `rk_types::StepError`'s closed eleven (ADR-0095,
//! ADR-0112), mapped in `classify.rs` at Slice 5; `retryable()` (the kernel's `retry_class()`,
//! Step 5); `experiment` / `arm` / `occurrence` / `resume_key` / `identity` (spike keys with no
//! consumer here); `cwd` (the worktree travels on the spec — RT-21); `isolate_mcp` (always on —
//! `--strict-mcp-config` is unconditional, AC-202); `extra_flags` (E1's flag arms; the argv has ONE
//! order, §7c.5); `timeout` (`ActivitySpec.timeout_secs`). The lifted tests `haiku_rejects_effort`
//! and `agent_card_renders_a_name_keyed_object` are in `tests/it/card.rs`, edited to the required
//! members.
//!
//! **Two shapes, on purpose.** [`AuthoredCard`] is the card as a pack author writes it (our own
//! record: `description`, `prompt`, `tools`, `cache_profile`, `permission_mode`; the name is the
//! file's), validated once at load. [`AgentCard`] is the bound card as `--agents` accepts it (the
//! lifted shape: `permissionMode` in the CLI's camelCase, `model` and `effort` the role's), whose
//! `--agents` bytes are what the spec's `card_digest` digests ([`AgentCard::digest`]). The binder
//! is the only path from the first to the second.

use std::collections::BTreeMap;

use rk_types::digest::{Digest, sha256};
use serde::{Deserialize, Serialize};

use crate::config::ConfigError;

/// One `[models.<key>]` row of an installed pack (`s8h` §8h.3; ADR-0197 decision 4, proposed):
/// the full model id and whether that model takes `--effort`.
///
/// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models.)* **The closed `Tier` enum left
/// the binding path here.** A tier is now the open set of `[models]` row keys: a role names a key
/// (`RoleExtras.model`), the binder resolves it in the pack's table, and the row's full `id` is
/// what reaches `--model`, `AgentCard.model`, the `--agents` JSON and the card digest. The lifted
/// rule `Tier::accepts_effort` (`tiers.mjs`: `EFFORTLESS_TIERS = ['haiku']` — *"haiku rejects the
/// parameter, so an effort on haiku is a spec error rather than a runtime one"*) is the row's
/// [`ModelRow::accepts_effort`], and its warrant is carried beside the haiku row in each pack.
/// Exactly the two keys (`deny_unknown_fields`), both required.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelRow {
    /// The full model id (`claude-sonnet-5`), never a tier word.
    pub id: String,
    /// Whether the model takes `--effort` (the lifted rule, per row).
    pub accepts_effort: bool,
}

/// A pack's `[models]` table: row key → row. The binder's one source for a role's model.
pub type Models = BTreeMap<String, ModelRow>;

/// The CLI's own permission-mode set, plus `default` (`s8h` §8h.2; ADR-0197 decision 2, proposed).
///
/// Cited to `claude --help`, the CLI's argv parser, and not to an SDK enum: *"(choices:
/// "acceptEdits", "auto", "bypassPermissions", "manual", "dontAsk", "plan")"* [measured
/// 2026-09-24 on 2.1.281; re-read 2026-10-02 on 2.1.287, the same six, captured to
/// `tests/fixtures/cli-help/2.1.287.permission-mode.txt`; re-read 2026-10-19 on 2.1.303 at s8j's pin
/// move, byte-identical, and that capture is now `2.1.303.permission-mode.txt`]. The binary maps `manual` to `default`
/// [measured by string search], so `default` is the name the init reports and the name a card
/// carries: seven in all. Only the shipped pair `default` / `acceptEdits` is admitted in a SHIPPED
/// card, and that is the doctor row's job, not this set's.
pub const PERMISSION_MODES: &[&str] = &[
    "acceptEdits",
    "auto",
    "bypassPermissions",
    "manual",
    "dontAsk",
    "plan",
    "default",
];

/// Reasoning effort (lifted). F15: applied per dispatch — thinking tokens low 0.0 / high 4.8 /
/// xhigh 221.0; low vs high did NOT separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl Effort {
    /// The `--effort` element.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Xhigh => "xhigh",
            Self::Max => "max",
        }
    }
}

/// Which axis a card is optimised against (AC-383).
///
/// `static` — commit and worktree churn; the body travels as `--system-prompt` (ADR-0016's
/// delivery clause, kept by ADR-0078) — or `default` — time and human gates; the adapter omits
/// `--system-prompt` and prepends the body to the user message (F28's measured arm; decision #19
/// of 2026-09-11). A card declares one; there is no third and no default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheProfile {
    Static,
    Default,
}

impl CacheProfile {
    /// The register's spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::Default => "default",
        }
    }
}

/// The tokens no card's tool list may spell as a segment (AC-269: *"the answer, mint or resolve
/// path"*), compared ASCII-case-insensitively against each segment of the tool name.
///
/// **The matching rule** (the register names the PATH, not a spelling; recorded here and in O-6's
/// RAISED note): a segment ends at any character outside `[A-Za-z0-9]` — so `_`, `/`, `.`, `:` AND
/// `-`, which MCP tool names (`mcp__<server>__<tool>`) routinely carry in the tool part — and at a
/// camelCase boundary (a lower-case letter or digit followed by an upper-case one, or an upper-case
/// run followed by a lower-case letter, as in `XMLAnswer`). So `answer`, `mcp__redkiln__answer`,
/// `rk/mint`, `rk-answer`, `answer-v2`, `mcp__tracker__resolve-issue`, `AnswerApi` and
/// `answerQuestion` are refused; `Resolver`, `mintmark`, `Answers` and `unanswered` are not — a
/// token inside a longer word is a word, not the path.
pub const ANSWER_API_TOKENS: &[&str] = &["answer", "mint", "resolve"];

/// The card-level lints that need no role and no schema, over the members every card shape carries.
///
/// Run by [`AuthoredCard::from_file`] at load AND by `command::assemble` (so by `Binder::bind`)
/// over the bound card: an `AuthoredCard` built by literal (its members are `pub` for 7e's pack
/// reader) never passed `from_file`, and §7c.12 has the binder run *"every card-level … lint of
/// §7c.4 at bind time"* so a `Config` cannot hold a card that fails them.
///
/// # Errors
/// [`ConfigError::CardNameEmpty`] (the `--agent` element); [`ConfigError::ToolsEmpty`] (AC-493);
/// [`ConfigError::AnswerApiExposed`] naming the tool (AC-269); [`ConfigError::PermissionModeAbsent`]
/// on a blank mode (AC-214: an explicit value, not an empty one).
pub fn lint_card(name: &str, tools: &[String], permission_mode: &str) -> Result<(), ConfigError> {
    if name.trim().is_empty() {
        return Err(ConfigError::CardNameEmpty);
    }
    let card = || name.to_owned();
    if tools.is_empty() || tools.iter().any(|t| t.trim().is_empty()) {
        return Err(ConfigError::ToolsEmpty { card: card() });
    }
    if let Some(tool) = tools.iter().find(|t| exposes_answer_api(t)) {
        return Err(ConfigError::AnswerApiExposed {
            card: card(),
            tool: tool.clone(),
        });
    }
    if permission_mode.trim().is_empty() {
        return Err(ConfigError::PermissionModeAbsent { card: card() });
    }
    // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.2.)* The CLI's closed
    // set: a misspelling is refused at load and again at bind, naming the mode.
    if !permission_mode_admitted(permission_mode) {
        return Err(ConfigError::PermissionModeOutsideSet {
            card: card(),
            mode: permission_mode.to_owned(),
        });
    }
    Ok(())
}

/// Is `mode` one of [`PERMISSION_MODES`]? Exact spelling — `acceptEdit` is not `acceptEdits`.
#[must_use]
pub fn permission_mode_admitted(mode: &str) -> bool {
    PERMISSION_MODES.contains(&mode)
}

/// The card FILE as authored — the closed record a pack author writes.
///
/// Every member the record declares is here, the required ones as `Option` so the refusal can name
/// the card and the member (a serde "missing field" names neither); `model` and `effort` are
/// declared ONLY so a leak is refused by name rather than as an unknown key.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardFile {
    pub description: String,
    pub prompt: String,
    pub tools: Option<Vec<String>>,
    pub cache_profile: Option<CacheProfile>,
    pub permission_mode: Option<String>,
    pub model: Option<String>,
    pub effort: Option<String>,
}

/// An authored card as [`AuthoredCard::from_file`] validated it, or as 7e's pack reader built it.
///
/// The card-level lints of §7c.4 that need no role or schema (AC-493, AC-383, AC-214,
/// AC-118/AC-418, AC-269) ran at load AND run again at bind ([`lint_card`], through the builder),
/// so a card built by literal is linted before it can enter a `Config`. Carries no tier — the
/// binder injects the role's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredCard {
    /// The card's name — the file stem; the `--agent` element and the `--agents` key.
    pub name: String,
    pub description: String,
    /// The card body: the system prompt under `static`, the head of the user message under
    /// `default`.
    pub prompt: String,
    /// The declared tool list — the pre-spawn effective tool set, and nothing else (ADR-0108 d2).
    pub tools: Vec<String>,
    pub cache_profile: CacheProfile,
    /// The `--permission-mode` value, explicit (AC-214).
    pub permission_mode: String,
}

impl AuthoredCard {
    /// Parse and validate a card file (JSON) named `name`.
    ///
    /// # Errors
    /// [`ConfigError::CardFile`] when the text is not the closed card record;
    /// [`ConfigError::TierLeak`] when the authored card carries `model` or `effort` (AC-118,
    /// AC-418); [`ConfigError::ToolsAbsent`] / [`ConfigError::ToolsEmpty`] (AC-493);
    /// [`ConfigError::CacheProfileAbsent`] (AC-383); [`ConfigError::PermissionModeAbsent`]
    /// (AC-214); [`ConfigError::AnswerApiExposed`] naming the tool (AC-269).
    pub fn from_file(name: &str, text: &str) -> Result<Self, ConfigError> {
        let card = || name.to_owned();
        let file: CardFile = serde_json::from_str(text).map_err(|e| ConfigError::CardFile {
            card: card(),
            detail: e.to_string(),
        })?;
        for (member, present) in [
            ("model", file.model.is_some()),
            ("effort", file.effort.is_some()),
        ] {
            if present {
                return Err(ConfigError::TierLeak {
                    card: card(),
                    member: member.to_owned(),
                });
            }
        }
        let tools = file
            .tools
            .ok_or_else(|| ConfigError::ToolsAbsent { card: card() })?;
        let cache_profile = file
            .cache_profile
            .ok_or_else(|| ConfigError::CacheProfileAbsent { card: card() })?;
        let permission_mode = file
            .permission_mode
            .ok_or_else(|| ConfigError::PermissionModeAbsent { card: card() })?;
        lint_card(name, &tools, &permission_mode)?;
        Ok(Self {
            name: card(),
            description: file.description,
            prompt: file.prompt,
            tools,
            cache_profile,
            permission_mode,
        })
    }
}

/// Does a tool name spell one of [`ANSWER_API_TOKENS`] as a segment (the matching rule stated on
/// the constant: any non-alphanumeric separator, and camelCase boundaries)?
#[must_use]
pub fn exposes_answer_api(tool: &str) -> bool {
    segments(tool).iter().any(|seg| {
        ANSWER_API_TOKENS
            .iter()
            .any(|t| seg.eq_ignore_ascii_case(t))
    })
}

/// The segments of a tool name: split at every non-alphanumeric character and at every camelCase
/// boundary (`answerQuestion` → `answer`, `Question`; `XMLAnswer` → `XML`, `Answer`; `Mint2` →
/// `Mint2` — a digit continues a segment).
fn segments(name: &str) -> Vec<&str> {
    let chars: Vec<(usize, char)> = name.char_indices().collect();
    let mut out = Vec::new();
    let mut start = 0usize;
    for (k, &(i, c)) in chars.iter().enumerate() {
        if !c.is_ascii_alphanumeric() {
            out.push(&name[start..i]);
            start = i + c.len_utf8();
            continue;
        }
        if k > 0 && i > start && c.is_ascii_uppercase() {
            let prev = chars[k - 1].1;
            let next_lower = chars
                .get(k + 1)
                .is_some_and(|&(_, n)| n.is_ascii_lowercase());
            if prev.is_ascii_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_ascii_uppercase() && next_lower)
            {
                out.push(&name[start..i]);
                start = i;
            }
        }
    }
    out.push(&name[start..]);
    out.retain(|s| !s.is_empty());
    out
}

/// An agent card as `--agents` accepts it (lifted): the bound card — the authored card with the
/// role's tier injected — serialised as the CLI reads it.
///
/// Confirmed against v2.1.258 at the pin: the object takes `description`, `prompt`, `tools`,
/// `model`, `effort`, `permissionMode` and more. `name` is the key, never an inner field;
/// `cache_profile` is the adapter's routing key and never reaches the CLI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentCard {
    #[serde(skip)]
    pub name: String,
    pub description: String,
    pub prompt: String,
    /// Required and non-empty (AC-493) — always written.
    pub tools: Vec<String>,
    /// The role's model (AC-212) — always written. *(dated 2026-10-02, s8h build, package
    /// P3-adapter-argv-models.)* The FULL id from the pack's `[models]` row the role names
    /// (ADR-0197 decision 5, proposed), never the row key.
    pub model: String,
    /// The role's effort, present exactly when the tier accepts one (the lifted `accepts_effort`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    /// Explicit (AC-214) — always written, in the CLI's camelCase.
    #[serde(rename = "permissionMode")]
    pub permission_mode: String,
    /// The delivery route (AC-383) — the adapter's routing key, `#[serde(skip)]`: the CLI never
    /// sees it.
    #[serde(skip)]
    pub cache_profile: CacheProfile,
}

impl AgentCard {
    /// Render the `--agents` argument: a single-key object keyed by name (lifted).
    ///
    /// # Panics
    /// Never in practice: the card is a struct of strings and vectors, and `serde_json` cannot
    /// fail to serialise it; the `expect` names the invariant.
    #[must_use]
    pub fn to_agents_json(&self) -> String {
        let mut map = BTreeMap::new();
        map.insert(self.name.clone(), self);
        serde_json::to_string(&map).expect("agent card is serialisable")
    }

    /// The card's content digest — SHA-256 over the `--agents` bytes ([`AgentCard::to_agents_json`]),
    /// which is `ActivitySpec.card_digest` (ADR-0099:61: *"a digest, never the card body"*) and the
    /// key of the binder's card store.
    #[must_use]
    pub fn digest(&self) -> Digest {
        sha256(self.to_agents_json().as_bytes())
    }
}
