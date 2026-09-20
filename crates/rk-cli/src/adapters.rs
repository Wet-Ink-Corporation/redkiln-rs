//! **`AdapterSet` — the four reals, resolved once at startup** (AC-485, AC-486; ADR-0094 decision 1;
//! `s7d` §7d.1's `AdapterSet` row, §7d.5 step (4)).
//!
//! **Resolution returns NAMES, not handles.** §7d.5 step (4): *"`resolve` returns names and
//! constructor values only and **constructs no port implementor**"*. The distinction is the whole
//! design: a resolver that built adapters would be the composition root, and the composition root is
//! `main`. What this module decides is *which* adapters the configuration asks for and whether that
//! request is admissible; building them is somebody else's line of code.
//!
//! **That is also why AC-486's refusal can be "before spawn or store write" at all.** Nothing here
//! touches the filesystem, spawns a process or opens a store, so a refusal is provably before all
//! three rather than merely early.
//!
//! **There is ONE credential denylist in this workspace** and it is not here: AC-422/AC-488 put it in
//! `rk_adapter_claude::env_denylist`, and this module reads it. A second list — even a correct one —
//! is a list that can drift out of agreement with the first, silently, in the direction of admitting
//! a credential.

use std::fmt;

use rk_adapter_claude::env_denylist;

use crate::config::ConfigFile;
use crate::env::Env;

/// The harness adapter this build ships.
const HARNESS: &str = "rk-adapter-claude";
/// The store adapter this build ships.
const STORE: &str = "rk-adapter-sqlite";
/// The toolchain adapters this build ships.
const TOOLCHAINS: [&str; 2] = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"];

/// What the configuration resolved to: names, and nothing constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    harness: String,
    store: String,
    toolchains: Vec<String>,
}

impl Resolved {
    /// Every resolved adapter name, harness then store then the toolchains in their declared order
    /// — the four AC-486's positive control counts.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let mut names = vec![self.harness.clone(), self.store.clone()];
        names.extend(self.toolchains.iter().cloned());
        names
    }

    /// The resolved harness adapter's name.
    #[must_use]
    pub fn harness(&self) -> &str {
        &self.harness
    }

    /// The resolved store adapter's name.
    #[must_use]
    pub fn store(&self) -> &str {
        &self.store
    }

    /// The resolved toolchain adapters' names.
    #[must_use]
    pub fn toolchains(&self) -> &[String] {
        &self.toolchains
    }
}

/// Why a configuration's adapter set was refused. Exit `3` (§7d.4's `ConfigError` row).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// A named adapter is not one this build ships.
    UnknownAdapter {
        /// Which `[adapters]` key named it.
        key: &'static str,
        /// The name the file asked for.
        name: String,
    },
    /// The captured environment carries a credential the one denylist refuses.
    DenylistedEnv {
        /// The variable's name. Its VALUE is never carried, never printed and never logged.
        name: String,
    },
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownAdapter { key, name } => write!(
                f,
                "`[adapters] {key}` names `{name}`, which this build does not ship. The adapters it \
                 ships are: {HARNESS}, {STORE}, {}",
                TOOLCHAINS.join(", ")
            ),
            Self::DenylistedEnv { name } => write!(
                f,
                "the environment carries `{name}`, which is refused: a dispatch runs on the \
                 subscription session, and an API-key-mode fallback is not an allowed degradation \
                 path (ADR-0010; AC-422, AC-488)"
            ),
        }
    }
}

impl std::error::Error for Refusal {}

/// Resolve the configuration's adapter set against what this build ships and what the environment
/// carries.
///
/// The environment is checked FIRST. A repository whose configuration is fine but whose shell
/// carries a credential must not be told its configuration is wrong, and a refusal that named the
/// adapter would be exactly that.
///
/// # Errors
/// [`Refusal::DenylistedEnv`] naming the variable, or [`Refusal::UnknownAdapter`] naming the key and
/// the adapter. Never both: the first refusal wins and nothing after it runs.
pub fn resolve(file: &ConfigFile, env: &Env) -> Result<Resolved, Refusal> {
    if let Some(name) = env_denylist::first_denylisted(env.as_map()) {
        return Err(Refusal::DenylistedEnv {
            name: name.to_owned(),
        });
    }

    if file.adapters.harness != HARNESS {
        return Err(Refusal::UnknownAdapter {
            key: "harness",
            name: file.adapters.harness.clone(),
        });
    }
    if file.adapters.store != STORE {
        return Err(Refusal::UnknownAdapter {
            key: "store",
            name: file.adapters.store.clone(),
        });
    }
    for name in &file.adapters.toolchains {
        if !TOOLCHAINS.contains(&name.as_str()) {
            return Err(Refusal::UnknownAdapter {
                key: "toolchains",
                name: name.clone(),
            });
        }
    }

    Ok(Resolved {
        harness: file.adapters.harness.clone(),
        store: file.adapters.store.clone(),
        toolchains: file.adapters.toolchains.clone(),
    })
}
