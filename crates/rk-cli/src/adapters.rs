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

/// The resolver. A namespace rather than a value, because §7d.1 and §7d.16 item 2 both name
/// **`AdapterSet`** as the thing that resolves, and a free function under a module of the same name
/// is a different spelling of the record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdapterSet;

/// What the configuration resolved to: names and constructor values, nothing constructed.
///
/// The two RT-49 keys are carried rather than dropped. §7d.5 step (4) says `resolve` returns *"names
/// and constructor values"*, and `harness_binary` / `toolchain_programs` are the constructor values:
/// they are the one way a process-level test reaches the fixture child without the environment
/// toggle AC-474 forbids. A resolver that accepted them on the `ConfigFile` and then let them fall
/// on the floor would make that route silently inoperative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
    harness: String,
    store: String,
    toolchains: Vec<String>,
    harness_binary: Option<String>,
    toolchain_programs: Option<std::collections::BTreeMap<String, String>>,
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

    /// RT-49: the harness program to spawn, when the configuration names one.
    #[must_use]
    pub fn harness_binary(&self) -> Option<&str> {
        self.harness_binary.as_deref()
    }

    /// RT-49: per-toolchain programs, when the configuration names them.
    #[must_use]
    pub const fn toolchain_programs(&self) -> Option<&std::collections::BTreeMap<String, String>> {
        self.toolchain_programs.as_ref()
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

impl AdapterSet {
    /// Resolve the configuration's adapter set against what this build ships and what the
    /// environment carries.
    ///
    /// The environment is checked FIRST, and the order is asserted by
    /// `ac422_config_reads_the_one_denylist`'s precedence arm rather than merely documented here. A
    /// repository whose configuration is fine but whose shell carries a credential must not be told
    /// its configuration is wrong, and a refusal that named the adapter would be exactly that.
    ///
    /// # Errors
    /// [`Refusal::DenylistedEnv`] naming the variable, or [`Refusal::UnknownAdapter`] naming the key
    /// and the adapter. Never both: the first refusal wins and nothing after it runs.
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
            harness_binary: file.adapters.harness_binary.clone(),
            toolchain_programs: file.adapters.toolchain_programs.clone(),
        })
    }
}

// ------------------------------------------------------------------ the root's store open path
//
// **Why this lives here and not in `doctor`** (`s7d` §7d.8, Step 7d Slice 4 cycle 2). Checkpoint
// 7d's D-10 / D18-7 ban forbids `SqliteStore::open` on any non-comment line under a module path
// matching `doctor` — which admits both `src/doctor/*.rs` and `src/verbs/doctor.rs` — while §7d.8
// requires `Ctx.store` to carry *"the store handle **the root opened**"* and, on the `--rebuild`
// path, names *"`adapters.rs`'s open path, the module the block's `doctor` filter does not admit"*
// as where the rebuilt store is re-opened for its identity check.
//
// So the two sentences together put the open HERE. Measured 2026-09-20 before this was added: the
// six `SqliteStore::open` call sites in this crate were all under `src/verbs/`, and none was in a
// module the doctor filter excludes — so `doctor` had no open path it was allowed to use.
//
// **It is not `AdapterSet::resolve`, and `doctor` calls neither as a startup refusal.** §7d.8:
// *"`doctor` does NOT call `AdapterSet::resolve` as a startup refusal (that is `run`'s step (4)):
// it builds its `Ctx` from the `ConfigFile` and reports adapter defects only through
// `RK-D-ADAPTER`"*. This function is the same shape — it RETURNS what happened and refuses
// nothing, so a foreign store reaches the table as `RK-D-STORE-ID`'s row rather than as an exit.

/// Open the event store for a repository, **returning the outcome rather than refusing on it**.
///
/// The `Result` is the value `doctor` puts in `Ctx.store`, not an error its caller acts on:
/// AC-413's identity check lives inside `open`, and a `doctor` that exited on a foreign store would
/// leave `RK-D-STORE-ID`'s id unobservable on the real binary. Every other verb wants the opposite
/// and calls `SqliteStore::open` directly, because for them a foreign store IS the refusal.
///
/// # Errors
/// [`rk_adapter_sqlite::StoreOpenError`] — **as the value `Ctx.store` carries**, not as something
/// the caller acts on. No `#[must_use]`: `Result` already carries it, and a second is a lint error
/// under this workspace's levels.
pub fn open_store(
    exec: &rk_git::git_exec::GitExec,
    worktree: &rk_git::path::WorktreePath,
) -> Result<rk_adapter_sqlite::SqliteStore, rk_adapter_sqlite::StoreOpenError> {
    rk_adapter_sqlite::SqliteStore::open(exec, worktree)
}

/// Re-open a just-rebuilt store so its stamped identity is checked, and report the outcome.
///
/// # 🛑 IT RUNS AFTER `verbs::doctor` HAS RETURNED, AND THAT IS THE WHOLE POINT
///
/// §7d.8: the rebuilt store is re-opened for its identity check *"**by the ROOT** —
/// `adapters.rs`'s open path, the module the block's `doctor` filter does not admit — after
/// `verbs::doctor` has returned; **neither `doctor/rebuild.rs` nor `verbs/doctor.rs` may name
/// `SqliteStore::open`**"*. `verbs::dispatch` is the caller because it is the one place that both
/// knows `--rebuild` was asked for and sits outside the ban's module filter.
///
/// **It cannot run earlier.** `SqliteStore` holds a `Writer` with a `Mutex<Connection>` in WAL
/// mode, and on Windows a rename over a file that handle holds open without `FILE_SHARE_DELETE` is
/// a sharing violation — so the check has to follow the rename, not precede it.
///
/// The return is the exit the root should take: `Success` when the rebuilt store opens under its
/// own identity, `ConfigError` when it does not. A rebuild that produced a store this binary
/// cannot open is a configuration error the operator must see, not a silent zero.
#[must_use]
pub fn rebuilt_store_identity(
    exec: &rk_git::git_exec::GitExec,
    worktree: &rk_git::path::WorktreePath,
) -> crate::exit::Exit {
    match rk_adapter_sqlite::SqliteStore::open(exec, worktree) {
        Ok(store) => {
            let identity = store.identity();
            println!(
                "rebuilt-store-identity ok root_commit={} schema_version={}",
                identity.root_commit, identity.schema_version
            );
            crate::exit::Exit::Success
        }
        Err(e) => {
            eprintln!("redkiln: the rebuilt store does not open under its own identity: {e:?}");
            crate::exit::Exit::ConfigError
        }
    }
}
