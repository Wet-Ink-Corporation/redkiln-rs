//! Shared helpers for the configuration-time tests (s7c §7c.18): a valid `Config`, the fixture
//! cards and roles under `tests/fixtures/`, and the one tiny schema every bind in this binary uses.
//!
//! Every `Config` here is built from a captured map the TEST writes as a value (ADR-0083; AC-217):
//! nothing reads or sets the process environment — `set_var` is clippy-banned workspace-wide and
//! the block's `rk_absent` over `tests/it` would name it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rk_types::typed::SchemaId;

use rk_adapter_claude::binder::{Role, Schema};
use rk_adapter_claude::card::AuthoredCard;
use rk_adapter_claude::config::{Config, ConfigValues, Margins, MismatchPolicy};

/// `tests/fixtures/`, absolute — through the compile-time manifest dir, never the process cwd.
#[must_use]
pub fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// The text of a fixture file.
#[must_use]
pub fn fixture(rel: &str) -> String {
    let path = fixtures().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The register's 0.07 / 0.18 vectors as ticks (AC-377) — above two on both windows.
#[must_use]
pub const fn margins_ok() -> Margins {
    Margins {
        five_hour: 7,
        seven_day: 18,
    }
}

/// The values a valid `Config` is built from: an EMPTY captured map, `claude` as the program (the
/// probe's value — every spawn-observing test replaces it with the fixture child at Slice 5), the
/// TTL pair left to the record's value, the margins above and refusal on a version mismatch (O-4's
/// default). Tests mutate what they need.
#[must_use]
pub fn values() -> ConfigValues {
    ConfigValues {
        captured_env: BTreeMap::new(),
        harness_binary: PathBuf::from("claude"),
        prompt_cache_ttl: None,
        subagent_prompt_cache_ttl: None,
        margins: margins_ok(),
        on_version_mismatch: MismatchPolicy::Refuse,
    }
}

/// A `Config` over `values()` — the construction every lint-free path starts from.
#[must_use]
pub fn config() -> Config {
    Config::new(values()).expect("the baseline values construct")
}

/// A `Config` whose captured map is `pairs`, everything else `values()`.
pub fn config_with_env(
    pairs: &[(&str, &str)],
) -> Result<Config, rk_adapter_claude::config::ConfigError> {
    let mut v = values();
    v.captured_env = pairs
        .iter()
        .map(|(k, val)| ((*k).to_owned(), (*val).to_owned()))
        .collect();
    Config::new(v)
}

/// A catalog card by stem (`tests/fixtures/cards/catalog/<stem>.json`), validated.
#[must_use]
pub fn catalog_card(stem: &str) -> AuthoredCard {
    let text = fixture(&format!("cards/catalog/{stem}.json"));
    AuthoredCard::from_file(stem, &text)
        .unwrap_or_else(|e| panic!("catalog card {stem} must load: {e}"))
}

/// A red card's load result by stem (`tests/fixtures/cards/red/<stem>.json`).
pub fn red_card(stem: &str) -> Result<AuthoredCard, rk_adapter_claude::config::ConfigError> {
    let text = fixture(&format!("cards/red/{stem}.json"));
    AuthoredCard::from_file(stem, &text)
}

/// A role fixture by stem (`tests/fixtures/cards/roles/<stem>.json`).
#[must_use]
pub fn role(stem: &str) -> Role {
    let text = fixture(&format!("cards/roles/{stem}.json"));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("role {stem} must parse: {e}"))
}

/// The one-field schema every bind in this binary carries: `{ ok: boolean }`, required. Its JSON
/// is fixed here and in the golden argv fixtures, so the `--json-schema` element is deterministic.
#[must_use]
pub fn schema() -> Schema {
    Schema {
        id: SchemaId::new("probe-ok"),
        json: serde_json::json!({
            "type": "object",
            "properties": { "ok": { "type": "boolean" } },
            "required": ["ok"]
        }),
    }
}

/// A worktree path for a bind — a fixed, absolute-looking value; nothing is spawned into it at
/// Slice 4.
#[must_use]
pub fn worktree() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"D:\rk\worktrees\run-1")
    } else {
        PathBuf::from("/rk/worktrees/run-1")
    }
}
