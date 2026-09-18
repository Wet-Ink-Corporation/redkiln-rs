//! The environment builder — the child's environment as a VALUE over a validated [`Config`]
//! (`s7c` §7c.6; AC-210, AC-217, AC-385, AC-509, AC-526).
//!
//! **A refusal, never a subtraction.** The child environment is *"the captured parent environment
//! minus a committed denylist"* (AC-217) — and the "minus" is a refusal at [`Config::new`], not a
//! strip here: this builder takes a [`Config`], and no `Config` exists whose captured map carries a
//! denylisted name (`config.rs`). So [`child_env`] is infallible: the captured map, as captured,
//! **plus the two TTL variables set explicitly to one value** — and nothing else. That "nothing
//! else" is what `ac509_child_echoes_env_without_tracker_credential` reads back from the fixture
//! child at Slice 5 (the echo equals this map; red by `env_clear()` removed, when the echo carries
//! the outer cargo's `CARGO_MANIFEST_DIR` and `PATH`).
//!
//! **Both TTLs, one value, always** (AC-210: *"`CLAUDE_CODE_PROMPT_CACHE_TTL=1h` and
//! `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` are set explicitly"*; AC-526's AND: *"a constructed
//! child environment never carries exactly one of the two"*): the value is [`Config::cache_ttl`],
//! resolved and validated at construction (ADR-0078 (ii); AC-526's five cases), written onto both
//! names by this one function. **Passed explicitly, never inherited** (AC-385): a captured map that
//! carries either name — in ANY letter case, since Windows names and `std`'s Windows `envs` keys
//! are case-insensitive — is overridden by the configured pair under the canonical names — the
//! operator's interactive setting is not the adapter's, and the adapter's is what the row records. The runner at the pin set
//! neither variable and inherited everything (§7c.6); the two values recorded on the
//! `dispatch_completed` row (AC-210's second half, Slice 5) are read from this map.
//!
//! **How the map reaches the child** (Slice 5, `runner.rs`): `Command::env_clear()` then
//! `Command::envs(map)` on the `std::process::Command` handed to `rk_containment::spawn_into_job` —
//! `env_clear` first, so nothing of the adapter's own process environment leaks around the map.
//! That `Command` site is Slice 5's, with its `#[expect(clippy::disallowed_types)]`; this module
//! holds no `Command` and reads no environment.

use std::collections::BTreeMap;

use crate::config::Config;

/// The main-bucket cache TTL variable (code.claude.com/docs/en/prompt-caching; ADR-0078).
pub const PROMPT_CACHE_TTL: &str = "CLAUDE_CODE_PROMPT_CACHE_TTL";
/// The subagent/workflow-bucket cache TTL variable — set beside the first, always (ADR-0078 (ii)).
pub const SUBAGENT_PROMPT_CACHE_TTL: &str = "CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL";

/// The constructed child environment: the captured map plus the two TTL names, one value.
///
/// Constructible only through [`child_env`], over a validated [`Config`]. `Debug` renders the KEYS
/// and a count, never a value (`config.rs`: *"what is never printed"*).
#[derive(Clone, PartialEq, Eq)]
pub struct ChildEnv(BTreeMap<String, String>);

impl std::fmt::Debug for ChildEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ChildEnv")
            .field(&crate::config::EnvKeys(&self.0))
            .finish()
    }
}

impl ChildEnv {
    /// The map, ordered (a `BTreeMap`: `HashMap` is clippy-banned and the order is what the echo
    /// test compares).
    #[must_use]
    pub const fn as_map(&self) -> &BTreeMap<String, String> {
        &self.0
    }

    /// The two TTL values as set — `(prompt, subagent)`, each `None` when the name is absent.
    /// Always both `Some` and equal, by construction; the accessor exists so a test asserts it
    /// rather than believes it, and absence is observable as `None`, never read as an empty
    /// default (AC-362's shape, Halt 11).
    #[must_use]
    pub fn ttl_pair(&self) -> (Option<&str>, Option<&str>) {
        (
            self.0.get(PROMPT_CACHE_TTL).map(String::as_str),
            self.0.get(SUBAGENT_PROMPT_CACHE_TTL).map(String::as_str),
        )
    }
}

/// Build the child environment.
///
/// The captured map with every case variant of either TTL name removed, then both TTL variables
/// set to [`Config::cache_ttl`] under their canonical names (overriding a captured value — AC-385:
/// explicit, never inherited).
///
/// **Why the case variants go**: Windows environment names are case-insensitive (the denylist
/// compares the same way), and `std`'s `Command::envs` keys on Windows compare case-insensitively
/// too — a map carrying `claude_code_prompt_cache_ttl=5m` beside the canonical name would hand
/// the child whichever spelling was inserted LAST (the lowercase one, in `BTreeMap` order), so the
/// inherited value would win and the two names would carry two values (AC-526's AND).
#[must_use]
pub fn child_env(config: &Config) -> ChildEnv {
    let mut map: BTreeMap<String, String> = config
        .captured_env()
        .iter()
        .filter(|(k, _)| {
            !(k.eq_ignore_ascii_case(PROMPT_CACHE_TTL)
                || k.eq_ignore_ascii_case(SUBAGENT_PROMPT_CACHE_TTL))
        })
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let ttl = config.cache_ttl().to_owned();
    map.insert(PROMPT_CACHE_TTL.to_owned(), ttl.clone());
    map.insert(SUBAGENT_PROMPT_CACHE_TTL.to_owned(), ttl);
    ChildEnv(map)
}
