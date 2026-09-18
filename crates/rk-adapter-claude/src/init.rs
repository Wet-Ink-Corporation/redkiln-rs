//! `system/init` — what the child says it actually ran with, and the three decisions taken from it.
//!
//! Adoption, isolation, and the observed CLI version against the pin
//! (`s7c` §7c.4, §7c.9; AC-202, AC-204, AC-386, AC-226, AC-385, AC-227).
//!
//! **The runner's `EffectiveConfig`** (`runner/src/fingerprint.rs` at `61f590e5…`, cited by
//! symbol — no bytes are lifted from that file and no lift header is carried here) read eight keys
//! from the init line — `session_id`, `model`, `agent`, `cwd`, `permissionMode`/`permission_mode`,
//! `tools`, `mcp_servers`/`mcpServers`, `slash_commands` — and its isolation reading was a count
//! (`mcp_servers.is_empty()`). This module keeps the key set (it is the shape the CLI emitted at the
//! pin) and replaces the count with **set digests** (SM-061; PIA-016): AC-202's instrument is
//! *"a set digest of the reported list against the declared one, not a count"*, so a scripted
//! init with the card's tool COUNT and one substituted name is red here (the same-count fixture,
//! §7c.17).
//!
//! **Every `system/init` a 7c fixture carries is SCRIPTED** (§7c.4, §7c.17): the corpus at the pin
//! retains no raw `claude -p` stream, only the runner's parsed view; the first raw init is the live
//! probe's (§7c.15). No `deny_unknown_fields` here (AC-201): a field the init gains is ignored.
//!
//! **The decisions, in the order the drain takes them on the init line** ([`decide`]):
//!
//! 1. **Version** ([`EffectiveConfig::version`]; AC-226, AC-385, AC-227): the observed CLI
//!    version is read from the init under [`VERSION_KEYS`] — **[surface unverified]**: the runner
//!    read no version and the corpus names no key (`apiKeySource` and the version key both appear
//!    in zero corpus files at the pin — §7c.11, §7c.15); the probe's raw init names the key, and
//!    until then the first key that matches wins. A version that is present goes on the row as
//!    `CliVersion::Observed(v)`; against the pin under [`MismatchPolicy::Refuse`] a mismatch is
//!    `Err(Transport { kind: Decode, detail })` after the child is killed through the group
//!    (RT-20 as amended, §7c.1), and under [`MismatchPolicy::Degraded`] the dispatch proceeds
//!    and `record::mismatch_warning` reads `Some` off the row. **An init that carries no version
//!    under any key is not a mismatch**: nothing was measured, the row carries `Pinned(pin)` (the
//!    provenance the record reserves for *"every path that terminated before one was read"* — the
//!    only honest arm when nothing was read; an `Observed("")` would be Halt 11's shape), the
//!    policy does not fire, and the probe's `OBSERVED_CLI_VERSION` line is what names the key.
//! 2. **Adoption** (AC-204's fourth case; ADR-0095 decision 2): the init's `agent` must be the
//!    card's name — a different agent (or none) is `NotAdopted { expected_agent, observed }`.
//!    `NotAdopted` is reserved for the wrong agent answering, never for tool-set inequality.
//! 3. **Isolation** (AC-202, AC-204, AC-386): the reported `mcp_servers` must be EMPTY under
//!    `--strict-mcp-config` — a non-empty list is `NotIsolated { surface: McpServers,
//!    diff_digest }` — and the reported `tools` set must be **equal** to the card's declared list:
//!    *"any difference — extra or missing — fails the dispatch … a superset is not adoption"*, so
//!    a subset and a superset are both `NotIsolated { surface: Tools, diff_digest }`. The
//!    `diff_digest` is [`set_digest`] over the symmetric difference, sorted — a digest, never the
//!    names (ADR-0105 clause 7's posture for what a row carries). **An absent list is the EMPTY
//!    set on both surfaces, and the two surfaces read differently from that one rule** (S5-08):
//!    an init with no `tools` key reports the empty set, which is UNEQUAL to any card's list (a
//!    card's list is non-empty, AC-493) — refused; an init with no `mcp_servers` / `mcpServers`
//!    key reports the empty set, which is EQUAL to the empty set `--strict-mcp-config` declares —
//!    proceeds (`ac202_absent_mcp_key_is_the_empty_set`). A refusal on the MCP surface for an
//!    absent key would carry a digest over two empty sets — a row saying *"not isolated,
//!    differing in nothing"*; a key the CLI renamed while servers were configured is a surface
//!    move the live probe's raw init names, not a case this rule can tell from an honest empty.
//!
//! **What this module never reads**: the adapter's own command line (AC-202's lint half — the
//! block's ban over the `init|isolation` module filter names the tokens, and none is spelt here):
//! isolation is decided from the child's own report, because *"an ignored flag looks identical to
//! an honoured one"* (brief §C.11).

use std::collections::BTreeSet;

use rk_types::digest::{Digest, sha256};
use rk_types::error::{IsolationSurface, StepError, TransportKind};

use crate::config::MismatchPolicy;

/// The keys the observed CLI version is read from, first match wins.
///
/// **[surface unverified]** until the live probe's raw `system/init` names the field (§7c.11,
/// §7c.15). The first spelling is the one the current CLI is believed to print; the second is the
/// plain fallback.
pub const VERSION_KEYS: &[&str] = &["claude_code_version", "version"];

/// What the child reports it actually ran with, from `system/init` (the runner's key set, with
/// the version added and the lists kept as sets for the digest comparison).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectiveConfig {
    pub session_id: Option<String>,
    pub model: Option<String>,
    pub agent: Option<String>,
    pub cwd: Option<String>,
    pub permission_mode: Option<String>,
    /// The child's effective tool set, as reported. `None` when the init carries no `tools` list.
    pub tools: Option<BTreeSet<String>>,
    /// The child's MCP servers, by name. `None` when the init carries no list.
    pub mcp_servers: Option<BTreeSet<String>>,
    pub slash_commands_count: Option<usize>,
    /// The CLI version the child reported under one of [`VERSION_KEYS`]; `None` when no key
    /// matched (nothing measured — the row then carries `Pinned`).
    pub version: Option<String>,
    /// Digest of the whole init line. A change here between two dispatches means the environment
    /// moved underneath them (the runner's `init_digest`, kept).
    pub init_digest: Digest,
}

/// Parse a `system/init` line's bytes. `None` when the line is not a JSON object.
#[must_use]
pub fn effective_config(raw: &str) -> Option<EffectiveConfig> {
    let init: serde_json::Value = serde_json::from_str(raw).ok()?;
    init.as_object()?;
    let s = |k: &str| init.get(k).and_then(|v| v.as_str()).map(str::to_owned);
    let list = |k: &str| {
        init.get(k).and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .map(|x| {
                    x.as_str().map(str::to_owned).unwrap_or_else(|| {
                        x.get("name")
                            .and_then(|n| n.as_str())
                            .map_or_else(|| x.to_string(), str::to_owned)
                    })
                })
                .collect::<BTreeSet<String>>()
        })
    };
    Some(EffectiveConfig {
        session_id: s("session_id"),
        model: s("model"),
        agent: s("agent"),
        cwd: s("cwd"),
        permission_mode: s("permissionMode").or_else(|| s("permission_mode")),
        tools: list("tools"),
        mcp_servers: list("mcp_servers").or_else(|| list("mcpServers")),
        slash_commands_count: init
            .get("slash_commands")
            .and_then(|v| v.as_array())
            .map(Vec::len),
        version: VERSION_KEYS.iter().find_map(|k| s(k)),
        init_digest: sha256(raw.as_bytes()),
    })
}

/// The set digest SM-061 names: SHA-256 over the sorted symmetric difference of the two sets,
/// joined by `\u{1f}` — the same for the two sets whatever order either was reported in.
#[must_use]
pub fn set_digest(reported: &BTreeSet<String>, declared: &BTreeSet<String>) -> Digest {
    let diff: Vec<&str> = reported
        .symmetric_difference(declared)
        .map(String::as_str)
        .collect();
    sha256(diff.join("\u{1f}").as_bytes())
}

/// What the init decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    /// Adoption and isolation hold; the version is the pin or the configuration allowed the
    /// mismatch (`mismatch` says which).
    Proceed { mismatch: Option<String> },
    /// The dispatch is refused with this error; the child is killed through the group.
    Refuse(StepError),
}

/// Take the three decisions of the module doc over a parsed init: version against `pinned`
/// under `policy`, adoption against `expected_agent`, isolation against `declared_tools`.
#[must_use]
pub fn decide(
    init: &EffectiveConfig,
    pinned: &str,
    policy: MismatchPolicy,
    expected_agent: &str,
    declared_tools: &[String],
) -> Decision {
    // 1. the version — only a MEASURED version can mismatch
    let mismatch = init
        .version
        .as_deref()
        .filter(|v| *v != pinned)
        .map(str::to_owned);
    if let Some(observed) = &mismatch
        && policy == MismatchPolicy::Refuse
    {
        return Decision::Refuse(StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "the child reported CLI version {observed} against the pinned {pinned} and the \
                 configured outcome for a mismatch is refusal (AC-226; MismatchPolicy::Refuse) — \
                 the child was killed through the group"
            ),
        });
    }
    // 2. adoption — the wrong agent answering (never tool-set inequality)
    if init.agent.as_deref() != Some(expected_agent) {
        return Decision::Refuse(StepError::NotAdopted {
            expected_agent: expected_agent.to_owned(),
            observed: init
                .agent
                .clone()
                .unwrap_or_else(|| "<no agent reported>".to_owned()),
        });
    }
    // 3. isolation — the MCP surface (an absent list is the empty set, which equals the declared
    //    empty set — the module doc), then the tool set as EQUALITY
    let empty = BTreeSet::new();
    let servers = init.mcp_servers.as_ref().map_or(&empty, |s| s);
    if !servers.is_empty() {
        return Decision::Refuse(StepError::NotIsolated {
            surface: IsolationSurface::McpServers,
            diff_digest: set_digest(servers, &empty),
        });
    }
    let declared: BTreeSet<String> = declared_tools.iter().cloned().collect();
    // An init with no `tools` list reports the empty set — which is unequal to any declared list
    // (a card's list is non-empty, AC-493), so it is refused as a missing set, never accepted.
    let reported = init.tools.as_ref().map_or(&empty, |t| t);
    if *reported != declared {
        return Decision::Refuse(StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest: set_digest(reported, &declared),
        });
    }
    Decision::Proceed { mismatch }
}
