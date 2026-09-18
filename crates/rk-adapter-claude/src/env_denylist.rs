//! The committed environment denylist — the ONE list AC-217, AC-422, AC-488 and AC-509 read
//! (decision #16 of 2026-09-11; `s7c` §7c.6; RT-25).
//!
//! **Inherit minus a committed denylist, never an allowlist** (#16): Claude Code reads many
//! undocumented variables — proxy, locale, `TERM`, editor — so the child environment is the
//! captured parent environment, and this file names what may NOT be in it. **A name on this list
//! found in the captured map is a REFUSAL of construction, never a subtraction** (AC-217's THEN:
//! *"fails the dispatch as a typed configuration error rather than being stripped and passed
//! through"*; AC-488: *"construction fails with a typed configuration error naming the variable"*):
//! [`crate::config::Config::new`] answers `ConfigError::DenylistedEnv { name }` and no map with a
//! listed name ever becomes a `Config`. The check is [`first_denylisted`], over a map the
//! composition root captured once at the edge and handed in as a value (ADR-0083); **no file in
//! this crate's `src/` reads the process environment** (the block's AC-217 ban).
//!
//! **What the list holds, exactly, and why each name** (§7c.6 — stated so the probe's outcome on
//! this box is decided by this file and not by luck):
//!
//! | name | why |
//! | --- | --- |
//! | `ANTHROPIC_API_KEY` | the register's first exact name (AC-217, AC-488); ADR-0010: subscription only, *"an API-key-mode fallback is not an allowed degradation path"* |
//! | `ANTHROPIC_AUTH_TOKEN` | the register's second exact name |
//! | `ANTHROPIC_BASE_URL` | the register's third exact name — a routed endpoint is not the subscription session |
//! | `ANTHROPIC_FOUNDRY_API_KEY` | added by `s7c` §7c.6 as an exact name: a vendor credential of the same kind as `ANTHROPIC_API_KEY`, **persisted at user scope on this box** [measured 2026-09-17], so it is in every fresh Git Bash — `Config::new` refuses it, correctly, and the probe runs under `env -u ANTHROPIC_FOUNDRY_API_KEY bash` |
//! | `GITHUB_TOKEN`, `GH_TOKEN`, `GH_ENTERPRISE_TOKEN` | the tracker credentials (AC-509: *"the child's environment contains no tracker credential"*) — the GitHub tracker's ambient names, **RT-25's routing** (the register names no variable; a record naming the set is OWED) |
//! | prefix `CLAUDE_CODE_USE_` | the register's vendor-routing variables (`CLAUDE_CODE_USE_BEDROCK`, `_VERTEX`, `_FOUNDRY`, …), named as a prefix because the register names them as one |
//!
//! **The list is exact names plus that one register-named prefix; it is NOT `ANTHROPIC_*`-prefix
//! keyed** — a prefix rule over the vendor namespace would be a stage decision no record makes
//! (§7c.6), and it would refuse `ANTHROPIC_MODEL`, which is not a credential.
//!
//! **Names compare ASCII-case-insensitively.** Windows environment names are case-insensitive: a
//! captured `Anthropic_Api_Key` IS `ANTHROPIC_API_KEY` to the child, so it is refused too. On POSIX
//! the two are distinct variables and the refusal is one case-variant wider than it need be — the
//! safe direction (fail closed), the same on every platform.
//!
//! `ac422_denylist_is_one_exported_list` iterates [`DENYLIST`] and [`DENYLIST_PREFIXES`] and
//! asserts `Config::new` refuses each — the export is what a second list could never satisfy.

use std::collections::BTreeMap;

/// The exact names (§7c.6; RT-25). Order is the register's, then the stage's additions.
pub const DENYLIST: &[&str] = &[
    "ANTHROPIC_API_KEY",
    "ANTHROPIC_AUTH_TOKEN",
    "ANTHROPIC_BASE_URL",
    "ANTHROPIC_FOUNDRY_API_KEY",
    "GITHUB_TOKEN",
    "GH_TOKEN",
    "GH_ENTERPRISE_TOKEN",
];

/// The one prefix the register names: the `CLAUDE_CODE_USE_*` vendor-routing variables.
pub const DENYLIST_PREFIXES: &[&str] = &["CLAUDE_CODE_USE_"];

/// Is `name` on the committed list — an exact name or under the prefix — comparing ASCII
/// case-insensitively (Windows names are)?
#[must_use]
pub fn is_denylisted(name: &str) -> bool {
    DENYLIST.iter().any(|d| d.eq_ignore_ascii_case(name))
        || DENYLIST_PREFIXES.iter().any(|p| {
            name.len() >= p.len() && name.as_bytes()[..p.len()].eq_ignore_ascii_case(p.as_bytes())
        })
}

/// The first denylisted name in a captured map, in the map's (sorted) order — the name
/// `ConfigError::DenylistedEnv` carries. `None` means the map may become a `Config`.
#[must_use]
pub fn first_denylisted(env: &BTreeMap<String, String>) -> Option<&str> {
    env.keys().map(String::as_str).find(|k| is_denylisted(k))
}
