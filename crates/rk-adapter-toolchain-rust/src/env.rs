//! The child environment: a captured `Config` map, the denylist refused at construction, the measured required names.
//!
//! The child environment — a `Config` value, never the inherited environment (`s7c` §7c.14: *"The
//! toolchain children's environment is the Harness rule, not the inherited environment … each
//! toolchain adapter spawns with `env_clear()` plus the same captured `Config` map the Harness
//! adapter uses — the AC-217/AC-488 denylist refused at construction, AC-509's tracker names
//! included — plus `CARGO_TARGET_DIR` (Rust)"*).
//!
//! **A name on the committed denylist found in the captured map is a REFUSAL of construction,
//! never a subtraction** (AC-217's THEN; AC-488): [`crate::Config::new`] answers
//! `ConfigError::DenylistedEnv { name }` and no map carrying a listed name ever becomes a
//! `Config`. **No file in this crate's `src/` reads the process environment** (the block's AC-217
//! ban): the map is captured once at the composition root and handed in as a value (ADR-0083).
//!
//! **The list is the Harness adapter's, name for name** (`rk-adapter-claude/src/env_denylist.rs`,
//! §7c.6, RT-25) — the three register names, `ANTHROPIC_FOUNDRY_API_KEY`, the three tracker
//! credentials and the `CLAUDE_CODE_USE_` prefix — restated here rather than imported because
//! §7c.19's manifest row for this crate names no edge to the Harness crate (an edge would drag
//! tokio into a crate whose job is `cargo test`), and *"one exported list"* (AC-422) is that
//! crate's own criterion over its own `Config`. A name added there is owed here; nothing checks
//! the two lists against each other today, and that is stated rather than hidden.
//!
//! **What the map must carry for `cargo` on this box — MEASURED 2026-09-18** (Slice 6's first
//! act, §7c.14; the preflight had measured `cargo --version` only, which needs nothing). A
//! `cargo test --workspace -- --list` over the fixture project under `env_clear()` — a build that
//! LINKS — was run additively with a fresh `CARGO_TARGET_DIR` each time:
//!
//! | map | result |
//! | --- | --- |
//! | `{SystemRoot, PATH}`, plus any ONE of `PATHEXT`, `COMSPEC`, `windir`, `LOCALAPPDATA`, `APPDATA`, `TEMP`+`TMP`, `USERPROFILE`, `ProgramFiles(x86)`+`ProgramFiles` | exit 101: `error: linker 'link.exe' not found` / `= note: program not found` / `note: the msvc targets depend on the msvc linker but 'link.exe' was not found` -- rustc's MSVC tool discovery needs **`SystemDrive`** |
//! | `{SystemDrive}` or `{SystemRoot, PATH, SystemDrive}` | exit 101: `error: linking with 'link.exe' failed: exit code: 1104` (the linker FOUND, at `C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.44.35207/bin/HostX64/x64/link.exe`; its temp file unopenable) -- `link.exe` needs a temp directory: **one of `TMP`, `TEMP`, `USERPROFILE`** |
//! | `{SystemDrive, TMP}` -- and `{SystemDrive, TEMP}`, `{SystemDrive, USERPROFILE}`, each with or without `SystemRoot` / `PATH` | exit 0, the four `: test` lines |
//!
//! So [`REQUIRED_CHILD_ENV`] on Windows is `SystemDrive`, and [`REQUIRED_ANY_OF`] is the temp
//! trio, of which the map must carry at least one; `Config::new` refuses a map lacking either
//! (`ConfigError::MissingChildEnv`), so the failure is a typed configuration error at
//! construction and never a `link.exe` diagnostic on stderr at the first spawn. The program NAME
//! (`cargo`) resolves through the *parent's* `PATH` regardless of the child map [measured in the
//! preflight], so `PATH` is not required — carrying it, and `SystemRoot`, is harmless. On POSIX
//! the two lists are empty and ARMED to a CI run that has never executed. Names compare
//! ASCII-case-insensitively (Windows names are).

use std::collections::BTreeMap;

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

/// The names the captured map MUST carry, every one of them, for `cargo` to build and link on
/// this box [measured 2026-09-18, the module doc]: `SystemDrive` on Windows; nothing on POSIX
/// (ARMED).
#[cfg(windows)]
pub const REQUIRED_CHILD_ENV: &[&str] = &["SystemDrive"];
/// POSIX arm: nothing measured; ARMED to a CI run.
#[cfg(not(windows))]
pub const REQUIRED_CHILD_ENV: &[&str] = &[];

/// The names of which the map must carry AT LEAST ONE — `link.exe`'s temp directory [measured
/// 2026-09-18]: `TMP`, `TEMP` or `USERPROFILE` on Windows; nothing on POSIX (ARMED).
#[cfg(windows)]
pub const REQUIRED_ANY_OF: &[&str] = &["TMP", "TEMP", "USERPROFILE"];
/// POSIX arm: nothing measured; ARMED to a CI run.
#[cfg(not(windows))]
pub const REQUIRED_ANY_OF: &[&str] = &[];

/// The name the adapter ADDS to the map on every spawn: the per-adapter target directory, so
/// concurrent tests share no build lock and no `target/` is ever written into the fixture project
/// (`s7c` §7c.14).
pub const CARGO_TARGET_DIR: &str = "CARGO_TARGET_DIR";

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

/// The first required name the map does NOT carry (ASCII-case-insensitively), or — when none of
/// [`REQUIRED_ANY_OF`] is carried — the first of that list; `None` when the map suffices.
#[must_use]
pub fn first_missing_required(env: &BTreeMap<String, String>) -> Option<&'static str> {
    let carries = |want: &str| env.keys().any(|k| k.eq_ignore_ascii_case(want));
    if let Some(missing) = REQUIRED_CHILD_ENV.iter().copied().find(|w| !carries(w)) {
        return Some(missing);
    }
    if !REQUIRED_ANY_OF.is_empty() && !REQUIRED_ANY_OF.iter().any(|w| carries(w)) {
        return REQUIRED_ANY_OF.first().copied();
    }
    None
}
