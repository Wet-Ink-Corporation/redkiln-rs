//! **`ConfigFile` — `.redkiln/config.toml` as a closed struct** (`s7d` §7d.1's `ConfigFile` row,
//! **D18-3**; the field set is **RT-34**'s).
//!
//! Closed for two reasons that are really one. `#[serde(deny_unknown_fields)]` makes an unrecognised
//! key a refusal rather than a silently ignored line — which is what turns a typo like
//! `unread_window_runss` into exit `3` instead of a default nobody chose. And a closed struct is a
//! struct whose **key list is enumerable**, which is what AC-474's *"config-surface snapshot test
//! over the full key list"* needs: the snapshot is a function of the type, so a dual-write key
//! cannot be added without the snapshot moving.
//!
//! **No field here has a default, and that is load-bearing for exactly one of them.** AC-426 makes
//! `[doctor] unread_window_runs` *"a required key, no default"*. `serde` gives that for free on a
//! non-`Option` field — but only as long as nobody writes an `impl Default`, a `#[serde(default)]`
//! or an `unwrap_or(` somewhere upstream, and no fixture can see any of the three. That is why
//! `ac426_unread_window_runs_required_no_default` scans the source as well as the fixtures.
//!
//! **The two optional `[adapters]` keys are RT-49** — routed, not decided. `harness_binary` and
//! `toolchain_programs` are the one way a process-level test reaches the fixture child without the
//! environment toggle AC-474 forbids. Both are on the key surface below; neither is in the scaffold
//! `init` writes; neither is a write-path toggle.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// `.redkiln/config.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigFile {
    /// Which adapter fills each port.
    pub adapters: Adapters,
    /// Which process packs are enabled. Step 7e fills this; `init` writes it empty.
    pub packs: Packs,
    /// `doctor`'s own configuration.
    pub doctor: Doctor,
    /// **`[project]` — ADR-0159.** The `<project>` component of every trailer value this system
    /// mints.
    ///
    /// `Option` because the table is new and `redkiln init` leaves an existing file untouched
    /// (ADR-0107 decision 2), and because `deny_unknown_fields` above means a config that does NOT
    /// declare it must still load: a run that mints no trailer has no business failing on it. The
    /// refusal is at the mint, not here — see `verbs::run`'s `story_trailer`.
    pub project: Option<Project>,
}

/// `[project]` — ADR-0159.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    /// The project name. **Never derived**: not from the pack, the lane, the item id, the worktree
    /// path or the remote. An empty or whitespace-only value is the ABSENT case and not a value,
    /// because `split_qualified` cannot tell `Story: /slug` from a missing project either.
    pub name: String,
}

/// `[adapters]` — the four reals, and the two RT-49 test seams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Adapters {
    /// The harness adapter's name.
    pub harness: String,
    /// The store adapter's name.
    pub store: String,
    /// The toolchain adapters' names.
    pub toolchains: Vec<String>,
    /// RT-49: the harness program to spawn. Absent, the adapter's own default is used.
    pub harness_binary: Option<String>,
    /// RT-49: per-toolchain programs. Absent, each is resolved on the child's `PATH`.
    pub toolchain_programs: Option<BTreeMap<String, String>>,
    /// **ADR-0140, superseding ADR-0137 clause 5 in part**: the construction bound on every child
    /// a toolchain adapter spawns, in seconds.
    ///
    /// *(Corrected 2026-09-26, s8c §8c.6, under record B = ADR-0171 decisions 1-4 and 7, proposed,
    /// which supersedes for `tool` steps ADR-0140 decision 2's sentence, decision 4 and its first
    /// two forbids bullets.)* This read *"the **only** bound"*, and the two paragraphs below say
    /// `catalog::Step` carries no timeout field. Both are no longer so for a `tool` step:
    /// `catalog::Step` carries `timeout_secs`, the kernel resolves it through the value table (an
    /// absent or `unmeasured` key refuses by name, never falling back to this value), and the
    /// adapter enforces `min(step, construction)` on every child, its `Timeout` naming the bound
    /// that fired. This value is the cap behind a step's declared bound, and the whole bound of a
    /// step that declares none. For an `agent` step the paragraphs below stand until `s8d` (D21-43
    /// N16). They are left as written.
    ///
    /// The port carries no timeout parameter (ADR-0099 decision 2, RT-5), and **nothing finer
    /// applies underneath**: `catalog::Step` carries no timeout field and this crate's loader reads
    /// none, so the shipped pack's 298 `timeout_secs` declarations reach no executor. Absent,
    /// [`crate::verbs::run::DEFAULT_TOOLCHAIN_TIMEOUT_SECS`].
    ///
    /// 🛑 **This read "a backstop … while a step's own `timeout_secs` stays the kernel's" until
    /// 2026-09-22, and that was false.** There is no step timeout for it to sit behind. The
    /// `timeout_secs` that does exist is `ActivitySpec`'s — per ACTIVITY, handed to the `Harness`,
    /// and never in scope for a toolchain child. Lowering this value shortens the only bound a
    /// `tool` step has; it is not an outer fence.
    pub toolchain_timeout_secs: Option<u64>,
}

/// `[packs]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Packs {
    /// The enabled process packs. Empty until Step 7e.
    pub enabled: Vec<String>,
}

/// `[doctor]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Doctor {
    /// How many runs back the unread-artifact rule looks. **Required, no default** (AC-426).
    pub unread_window_runs: u32,
}

/// Why a configuration file was refused. Every arm is exit `3` (§7d.4's `ConfigError` row):
/// a refusal of the repository's recorded state, read as configuration of the item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigError {
    /// The bytes are not a `ConfigFile` — an unknown key, a missing required key, a wrong type.
    /// `serde`'s own message is carried through, because it names the key and the line.
    Malformed(String),
    /// The file could not be read at all — absent, a directory, unreadable.
    ///
    /// **A separate arm rather than a `Malformed` carrying an io message**, added at Slice 3 with
    /// [`crate::verbs::ConfigReader`], which is the first thing in this crate to open the file
    /// rather than be handed its bytes. "There is no configuration here" and "the configuration
    /// says something this type does not admit" are different facts about a repository, and a run
    /// on a tree that was never `init`ed should not be reported as a malformed key.
    Unreadable {
        /// The path that could not be read.
        path: String,
        /// The io error's own message.
        detail: String,
    },
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(detail) => {
                write!(
                    f,
                    "`.redkiln/config.toml` is not a valid configuration: {detail}"
                )
            }
            Self::Unreadable { path, detail } => {
                write!(f, "`{path}` could not be read: {detail}")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

/// Read a configuration from BYTES.
///
/// Pure, and deliberately: every fixture test reads a string rather than a repository, which is what
/// lets the configuration layer be tested at Slice 2 — before any verb that would open one.
///
/// # Errors
/// [`ConfigError::Malformed`] naming the key `serde` refused.
pub fn parse(src: &str) -> Result<ConfigFile, ConfigError> {
    toml::from_str(src).map_err(|e| ConfigError::Malformed(e.to_string()))
}

/// **Every key the type admits**, as dotted paths, sorted — AC-474's surface.
///
/// Enumerated from a fully-populated value rather than from a hand-kept list, so the snapshot is a
/// function of the struct: a key added to `ConfigFile` appears here without anyone remembering to
/// add it, which is the only version of this test worth having.
///
/// # Panics
/// It cannot, and the `expect` says which fact makes that true: `serde_json::to_value` fails only on
/// a float that is not a number or a map key that is not a string, and `ConfigFile` has neither. If
/// a future field introduces one, this panics loudly rather than returning a surface with a hole in
/// it — a silently short key list is the one outcome AC-474's snapshot must never have.
#[must_use]
pub fn key_surface() -> Vec<String> {
    let every = ConfigFile {
        adapters: Adapters {
            harness: String::new(),
            store: String::new(),
            toolchains: Vec::new(),
            // The optionals are populated, or they would not appear in the serialised form and the
            // surface would silently under-report exactly the two keys RT-49 routed.
            harness_binary: Some(String::new()),
            toolchain_programs: Some(BTreeMap::from([(String::new(), String::new())])),
            toolchain_timeout_secs: Some(0),
        },
        packs: Packs {
            enabled: Vec::new(),
        },
        doctor: Doctor {
            unread_window_runs: 0,
        },
        // Populated for the reason the comment above gives: an absent optional does not appear in
        // the serialised form, and the surface would under-report the key ADR-0159 added.
        project: Some(Project {
            name: String::new(),
        }),
    };
    let value = serde_json::to_value(&every).expect("ConfigFile serialises");
    let mut keys = Vec::new();
    walk(&value, "", &mut keys);
    keys.sort();
    keys
}

/// Collect dotted key paths from a serialised value. A map's own keys are data, not schema, so the
/// one map-valued field (`toolchain_programs`) contributes its field name and stops.
fn walk(value: &serde_json::Value, prefix: &str, out: &mut Vec<String>) {
    let Some(object) = value.as_object() else {
        return;
    };
    for (key, child) in object {
        let path = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };
        let recurse = child.is_object() && path != "adapters.toolchain_programs";
        out.push(path.clone());
        if recurse {
            walk(child, &path, out);
        }
    }
}
