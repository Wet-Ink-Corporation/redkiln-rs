//! The child environment: a captured `Config` map, the denylist refused at construction, the measured required names.
//!
//! The child environment — a `Config` value, never the inherited environment (`s7c` §7c.14: *"The
//! toolchain children's environment is the Harness rule, not the inherited environment … each
//! toolchain adapter spawns with `env_clear()` plus the same captured `Config` map the Harness
//! adapter uses — the AC-217/AC-488 denylist refused at construction, AC-509's tracker names
//! included"*).
//!
//! **A name on the committed denylist found in the captured map is a REFUSAL of construction,
//! never a subtraction** (AC-217's THEN; AC-488): [`crate::Config::new`] answers
//! `ConfigError::DenylistedEnv { name }` and no map carrying a listed name ever becomes a
//! `Config`. **No file in this crate's `src/` reads the process environment** (the block's AC-217
//! ban): the map is captured once at the composition root and handed in as a value (ADR-0083).
//!
//! **The list is the Harness adapter's, name for name** (`rk-adapter-claude/src/env_denylist.rs`,
//! §7c.6, RT-25) — restated here rather than imported because §7c.19's manifest row for this
//! crate names no edge to the Harness crate (an edge would drag tokio into a crate whose job is
//! `vitest run`); *"one exported list"* (AC-422) is that crate's criterion over its own `Config`.
//! A name added there is owed here; nothing checks the two lists against each other today, and
//! that is stated rather than hidden.
//!
//! **What the map must carry for `npx.cmd` on this box — MEASURED 2026-09-18** (Slice 6's first
//! act, §7c.14; the preflight's scratch program, `std::process::Command` + `env_clear()` +
//! stdin null + stdout/stderr to files, candidates tried additively and then eliminated to a
//! minimal set, each minimal name removed alone for its failure text):
//!
//! | command | minimal map | absent name → failure |
//! | --- | --- | --- |
//! | `npx.cmd --version` | `{SystemRoot}` | `SystemRoot` absent: exit 134, stderr `Assertion failed: ncrypto::CSPRNG(nullptr, 0)` at `src\node.cc:1224` (OpenSSL's CSPRNG seeding needs it; printed twice under npx.cmd, two node launches) |
//! | `npx.cmd vitest --version` (cwd = the predecessor) | `{SystemRoot, PATH}` | `PATH` absent: exit 1, stderr `'vitest' is not recognized as an internal or external command,` / `operable program or batch file.`; `PATH` present without the node install dir: `'node' is not recognized …` |
//! | `npx.cmd vitest run --reporter=json --outputFile=<f> <file> -t AC-015` (cwd = the predecessor) | `{SystemRoot, PATH}` | as above; exit 0 in 2.1 s, the JSON's `numPassedTests` 2 |
//! | `node --version` | `{}` | but `node -e 1` under `{}` aborts as `SystemRoot` absent does — node's real need is `SystemRoot` |
//!
//! `PATH` must contain the node install dir (`C:\Program Files\nodejs` here — that alone
//! suffices; the parent's full `PATH` gives the identical result). `PATHEXT`, `COMSPEC`,
//! `APPDATA`, `LOCALAPPDATA`, `USERPROFILE`, `TEMP`, `TMP`, `HOMEDRIVE`, `HOMEPATH`,
//! `ProgramFiles` are NOT needed: `npx.cmd` injects `COMSPEC`, `PATHEXT`, `HOME`, `PROMPT` and
//! the `npm_*` names into its own child. The program NAME (`npx.cmd`) resolves through the
//! *parent's* `PATH` regardless of the child map; only what the `.cmd` shim re-launches (`node`,
//! `vitest`) needs `PATH` in the child map. So [`REQUIRED_CHILD_ENV`] is **`SystemRoot` and
//! `PATH`** on Windows; on POSIX (`npx`, a shell script) nothing is measured and the list is
//! empty and ARMED to a CI run that has never executed. Names compare ASCII-case-insensitively
//! (Windows names are).
//!
//! **A network touch the measurement found, and the rule it made:** one extra probe,
//! `npx.cmd node -e …`, made npx fetch `node@26.9.0` from the registry because `node` is not a
//! local bin (`npm warn exec The following package was not found and will be installed`). So this
//! adapter **never spawns `npx.cmd <name>` unless `<name>` is installed locally** — [`local_bin`]
//! walks up from the project root for `node_modules/.bin/<name>` and the spawn is refused typed
//! (`Transport { Spawn }`) when it is absent; `npx` without the local bin would otherwise fetch,
//! or prompt on a closed stdin until the bound.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The exact names (§7c.6; RT-25) — the Harness list, restated (the module doc).
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

/// The names the captured map MUST carry, every one of them, for `npx.cmd vitest run` to start
/// on this box [measured 2026-09-18, the module doc]: `SystemRoot` and `PATH` on Windows.
#[cfg(windows)]
pub const REQUIRED_CHILD_ENV: &[&str] = &["SystemRoot", "PATH"];
/// POSIX arm: nothing measured; ARMED to a CI run.
#[cfg(not(windows))]
pub const REQUIRED_CHILD_ENV: &[&str] = &[];

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

/// The first required name the map does NOT carry (ASCII-case-insensitively), or `None`.
#[must_use]
pub fn first_missing_required(env: &BTreeMap<String, String>) -> Option<&'static str> {
    REQUIRED_CHILD_ENV
        .iter()
        .copied()
        .find(|want| !env.keys().any(|k| k.eq_ignore_ascii_case(want)))
}

/// The locally installed bin `name`, or `None`.
///
/// The locally installed bin `name` — `node_modules/.bin/<name>` (the POSIX shim; `<name>.CMD`
/// beside it on Windows) — found by walking up from `root` the way `npx` resolves a local bin,
/// or `None`. The guard that keeps `npx` from fetching (the module doc).
#[must_use]
pub fn local_bin(root: &Path, name: &str) -> Option<PathBuf> {
    let mut dir = Some(root);
    while let Some(d) = dir {
        let bin = d.join("node_modules").join(".bin").join(name);
        if bin.is_file() {
            return Some(bin);
        }
        dir = d.parent();
    }
    None
}
