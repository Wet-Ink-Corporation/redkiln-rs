//! The command builder — ONE builder assembles every argv, and the lints that read it share it.
//!
//! (`s7c` §7c.5; ADR-0108 decision 4: *"the two lints share the command builder"*; AC-523's fourth
//! instrument, the argv cap, sits on the same path.)
//!
//! **The golden order, and where it comes from.** The vector has ONE element order and its source
//! is the lifted `build_args` in `runner/src/runner.rs` at `61f590e5…` (§7c.2; TD3-2: the lifted
//! order IS the golden order), which pushed `--strict-mcp-config` **after** `--permission-mode` and
//! `extra_flags` last [measured 2026-09-17]:
//!
//! ```text
//! -p --output-format stream-json --verbose --model <tier> [--effort <e>]
//! --agents <json> --agent <name> --json-schema <schema> --permission-mode <m>
//! --strict-mcp-config --permission-prompts none [--system-prompt <card body>]
//! ```
//!
//! The first six elements and the element set are CORPUS-MEASURED: `results/h1-gap.jsonl`'s six
//! dispatch rows (F28's `default` arm) all carry `["-p", "--output-format", "stream-json",
//! "--verbose", "--model", "haiku", "--strict-mcp-config"]`, and `results/h1-sysprompt.jsonl`'s
//! two carry the same followed by `"--system-prompt", <45 bytes>` — LAST, where its `extra_flags`
//! put it (§7c.5; the extracted rows are under `tests/fixtures/recorded/` with PROVENANCE). The
//! authored elements — `--agents`/`--agent` (AC-203), `--json-schema` (AC-205), `--permission-mode`
//! (AC-214), `--permission-prompts none` (AC-215) — are each warranted by a criterion and were NOT
//! measured in that combination at the pin; the one recorded card-dispatch argv is F20's
//! (`--agents … --agent … --json-schema … --permission-mode bypassPermissions --strict-mcp-config`,
//! no `--system-prompt`). **The live probe (§7c.15) is the first measurement of the full vector.**
//! `--allowed-tools` (the runner's optional element) is not emitted: the pre-spawn tool set is the
//! card's declared `tools` list inside the `--agents` object and nothing else (ADR-0108 d2).
//!
//! **Per-profile delivery** (AC-383; AC-211 PROVISIONAL on S-H1c; decision #19 of 2026-09-11):
//! under `static` the card body is `--system-prompt <body>`, appended last, AND the card travels
//! as `--agents`/`--agent` for adoption — **both are passed** (O-5, OPEN: the combination is
//! unmeasured at the pin; the probe measures it and a refusal by the CLI is a Halt, not a
//! re-tune). Under `default` `--system-prompt` is omitted and the body is prepended to the user
//! message on stdin (F28's measured arm: 18,603 cache-creation flat at 65.2 min and 49.7 h), the
//! body and the message separated by one blank line — the separator is authored, not measured.
//! Neither append flag is ever emitted (AC-211: the append route is S-H1c, unrun; the block bans
//! both tokens over `src/`). **The `--system-prompt` bytes are the card body and nothing else**
//! (AC-209): no volatile input reaches any argv element — the user message is stdin — so two
//! dispatches of one card version hash the same system prompt whatever their inputs.
//!
//! **The lints, on this path, each a [`ConfigError`] before spawn** (§7c.4):
//!
//! - **AC-203** — `--agents` without `--agent` is refused ([`ConfigError::AgentsWithoutAgent`]):
//!   F7 measured that `--agents` alone leaves the card inert for the main turn. The binder always
//!   adopts; [`CommandSpec::adopt_agent`] exists so the register's *"fixture spec that must fail"*
//!   is a spec, not a comment.
//! - **AC-205 / AC-417** — a schema-bearing dispatch whose card's `tools` omit `StructuredOutput`
//!   is refused ([`ConfigError::StructuredOutputAbsent`]); every dispatch here carries a schema
//!   (`ActivitySpec.schema` is not optional). F20 is why this is configuration-time: the allowlist
//!   silently voids `--json-schema` and the process exits 0 reporting success (Halt 3).
//! - **AC-132** — the per-request budget precondition: ONE output schema (the builder takes exactly
//!   one, by type) and NO strict user-defined tool on the command line; a spec that adds one is
//!   refused ([`ConfigError::StrictToolOnArgv`]) — re-opening the budget is a record's decision,
//!   not a builder flag.
//! - **The card-level lints, again, over the bound card** (`card::lint_card`; §7c.12: the binder
//!   runs *"every card-level and argv-level lint of §7c.4 at bind time"*): an empty name, an empty
//!   tool list or a blank tool (AC-493), a tool on the answer path (AC-269), a blank
//!   `permission_mode` (AC-214) — refused here whatever built the card, since an `AgentCard` is a
//!   `pub` struct a caller may assemble by literal.
//! - **AC-214** — `--permission-mode` is unconditional, from the card's explicit value; a silent
//!   card is refused at load (`AuthoredCard::from_file`) and a blank one here. The lint half — that
//!   the push sits outside every guard — is ARMED (§7c.5): read this function.
//! - **AC-215** (argv half) — `--permission-prompts none`, unconditional. The stdin write-then-
//!   shutdown is Slice 5's (`runner.rs`).
//! - **AC-216 / AC-511** — the bare flag on no input: there is no code path that pushes it; the
//!   matrix property test is the instrument, the block's `rk_absent` the cheap second line (the
//!   token itself is spelt only in the lint control, so the `src/` tree carries it nowhere).
//! - **AC-523's cap (RT-1)** — the command line measures at or over [`ARGV_CAP_BYTES`] →
//!   [`ConfigError::ArgvAtOrOverCap`]. **The measure** ([`command_line_units`]) is the command
//!   line `std::process::Command` builds for `CreateProcessW` on Windows, in UTF-16 units: the
//!   program in quotes, then for each element a space and the element quoted and escaped by
//!   MSVCRT's rules (quoted when empty or carrying a space or tab; each `"` escaped with a
//!   backslash, backslashes before a `"` doubled, trailing backslashes doubled when quoted), then
//!   the terminating NUL — `std`'s `make_command_line` / `append_arg`, re-stated. The constant is
//!   named as RT-1 spells it, in "bytes", and the documented limit is 32,767 *characters* including
//!   the null: for ASCII the two coincide, and the measure counts what the limit counts — a
//!   non-ASCII prompt measures FEWER units than bytes, which is the unit the limit is in (RAISED
//!   beside RT-1: the owed record should name the unit). A vector that measures exactly the cap is
//!   refused — one unit stricter than the documented maximum, the safe direction; one under is not.
//!   **Precondition: the program is not a batch file.** For a `.bat`/`.cmd` `std` builds
//!   `cmd.exe /d /c "<resolved path> …"` (`make_bat_command_line`) — longer, quoted differently —
//!   and this measure would under-count; `Config::new` refuses such a `harness_binary`
//!   (`ConfigError::HarnessBinaryIsBatch`), so every program this measure sees is the executable.
//!   The same measure applies on every platform (a POSIX `ARG_MAX` is megabytes; the cap is the
//!   Windows floor, routed — a record naming the value is OWED). `--system-prompt-file` is S-SPF,
//!   registered and unused (decision #18).
//!
//! **What this module does not do**: spawn (Slice 5's `runner.rs`, through `rk-containment`),
//! read the environment (the child map is `env.rs`'s, over the `Config`), or read `system/init`
//! (AC-202: no isolation assertion reads the adapter's own argv — this file is not in the
//! `init|isolation` module filter and nothing in those modules names `argv`).

use std::path::{Path, PathBuf};

use crate::card::{AgentCard, CacheProfile, lint_card};
use crate::config::{Config, ConfigError};

/// The argv cap (RT-1; decision #18 of 2026-09-11): Windows `CreateProcessW`'s documented
/// command-line limit, 32,767 characters including the terminating null.
///
/// A command line that measures AT or over it is refused before spawn; the fixture at the cap is
/// `tests/fixtures/argv/at-the-cap.card.json`. Routed, not decided — a record naming the value is
/// OWED (§7c.1).
pub const ARGV_CAP_BYTES: usize = 32_767;

/// The period, in milliseconds, at which `dispatch` polls `CancelToken::is_cancelled()` while the
/// child runs.
///
/// (RT-3 as amended; `s7c` §7c.10.) The token is an `Arc<AtomicBool>` with no wake-up, so the
/// cancel arm of the three-way race is a `tokio::time::interval` poll, and this is its
/// period. **Chosen at Slice 5: 50 ms** — a cancel is honoured within one period plus the kill,
/// two orders of magnitude under any test's bound and invisible against a dispatch measured in
/// seconds; **UNCONFIRMED** — no record names the value, and RT-3's OWED record carries it.
/// Declared beside [`ARGV_CAP_BYTES`], where §7c.1's routing table places it; consumed by
/// `runner.rs`.
pub const CANCEL_POLL_MS: u64 = 50;

/// The tool a schema-bearing dispatch's card must declare (AC-205; ADR-0108).
pub const STRUCTURED_OUTPUT: &str = "StructuredOutput";

// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.2; D21-48 X12; ADR-0197
// decision 3, proposed.)* THE ARGV SPELLING OF A CARD'S `default`, one row per CLI version, declared
// in exactly the form Checkpoint 8h (9) and s8j's Checkpoint 8j (6) read. The card, the `--agents`
// JSON, the digest and the init comparison all keep `default`; only the `--permission-mode` element
// reads this table, keyed by the `Config`'s pin. `2.1.278` is the pin, warranted by the 2026-09-21
// live acceptance (`LIVE_WARRANTED`); `2.1.287` is warranted by its help capture
// (`tests/fixtures/cli-help/2.1.287.permission-mode.txt`, read at s8h stage open) and is dormant
// until s8j's pin move deletes the `2.1.278` row.
// rustfmt would fold the rows onto one line; the declared form is one row per line.
#[rustfmt::skip]
pub(crate) const PERMISSION_MODE_ARGV: &[(&str, &str)] = &[
    ("2.1.278", "default"),
    ("2.1.287", "manual"),
];

/// The rows of [`PERMISSION_MODE_ARGV`] warranted by a live acceptance rather than a help capture
/// (`s8h` §8h.2). `&[]` after s8j's pin move.
pub(crate) const LIVE_WARRANTED: &[&str] = &["2.1.278"];

/// [`PERMISSION_MODE_ARGV`], read by the help-capture test (an integration test cannot name a
/// `pub(crate)` item).
#[must_use]
pub const fn permission_mode_argv_rows() -> &'static [(&'static str, &'static str)] {
    PERMISSION_MODE_ARGV
}

/// [`LIVE_WARRANTED`], read by the help-capture test.
#[must_use]
pub const fn live_warranted_rows() -> &'static [&'static str] {
    LIVE_WARRANTED
}

/// The `--permission-mode` element for a card whose mode is `card_mode`, under `config`'s pin.
///
/// # Errors
/// [`ConfigError::NoPermissionModeSpelling`] naming the pin, when [`PERMISSION_MODE_ARGV`] has no
/// row for it.
pub fn argv_permission_mode(config: &Config, card_mode: &str) -> Result<String, ConfigError> {
    // Keyed by the Config's pin, never by TESTED_CLI_VERSION (X12: a test builds a Config at
    // another pin). A pin with no row is refused whatever the card's mode: no fallback.
    let pin = config.tested_cli_version();
    let Some((_, spelling)) = PERMISSION_MODE_ARGV.iter().find(|(v, _)| *v == pin) else {
        return Err(ConfigError::NoPermissionModeSpelling {
            version: pin.to_owned(),
        });
    };
    Ok(if card_mode == "default" {
        (*spelling).to_owned()
    } else {
        card_mode.to_owned()
    })
}

/// One entry of a directory on the worktree's ancestor chain (`s8h` §8h.6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sibling {
    /// The entry's file name.
    pub name: String,
    /// A directory (denied as `<path>/**`), or anything else (denied as `<path>`).
    pub is_dir: bool,
}

/// The worktree's ancestor chain and what each level lists — the input to [`ChildSettings`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfinementChain {
    /// The worktree's drive letter, lower-case, when the path carries one (Windows).
    pub drive: Option<char>,
    /// The worktree's path segments below the root, in order; the last is the worktree itself.
    pub segments: Vec<String>,
    /// `listings[i]` is the entry list of the directory `root + segments[..i]` — one per
    /// ancestor level, so exactly `segments.len()` lists.
    pub listings: Vec<Vec<Sibling>>,
}

impl ConfinementChain {
    /// The chain of a real worktree, each ancestor level listed from the filesystem.
    ///
    /// # Errors
    /// [`ConfigError::Confinement`].
    ///
    /// The worktree must be absolute: a drive path on Windows (`D:/…`, or its verbatim form), a
    /// rooted path elsewhere. Each ancestor directory is listed once; an ancestor that does not
    /// exist lists nothing (there is nothing there to deny), and any other listing failure, a
    /// relative or UNC path, or an entry name that is not UTF-8 is refused — never guessed.
    ///
    /// # Errors
    /// [`ConfigError::Confinement`] naming the path and the failure.
    pub fn read(worktree: &Path) -> Result<Self, ConfigError> {
        use std::path::{Component, Prefix};
        let refuse = |why: &str| ConfigError::Confinement {
            detail: format!("{}: {why}", worktree.display()),
        };
        let mut drive = None;
        let mut rooted = false;
        let mut segments = Vec::new();
        for component in worktree.components() {
            match component {
                Component::Prefix(p) => match p.kind() {
                    Prefix::Disk(d) | Prefix::VerbatimDisk(d) => {
                        drive = Some(char::from(d).to_ascii_lowercase());
                    }
                    _ => {
                        return Err(refuse(
                            "not a drive path (UNC and device paths are refused)",
                        ));
                    }
                },
                Component::RootDir => rooted = true,
                Component::Normal(n) => segments.push(
                    n.to_str()
                        .ok_or_else(|| refuse("a path segment is not UTF-8"))?
                        .to_owned(),
                ),
                Component::CurDir | Component::ParentDir => {
                    return Err(refuse("the path carries `.` or `..`"));
                }
            }
        }
        if !rooted || (cfg!(windows) && drive.is_none()) {
            return Err(refuse("the worktree path is not absolute"));
        }
        if segments.is_empty() {
            return Err(refuse("the worktree is a filesystem root"));
        }
        let mut dir = worktree
            .ancestors()
            .last()
            .unwrap_or(worktree)
            .to_path_buf();
        let mut listings = Vec::with_capacity(segments.len());
        for segment in &segments {
            listings.push(list_dir(&dir).map_err(|e| refuse(&e))?);
            dir.push(segment);
        }
        Ok(Self {
            drive,
            segments,
            listings,
        })
    }
}

/// The entries of one ancestor directory, sorted by name; a directory that does not exist lists
/// nothing. A symlink is typed by what it points at.
fn list_dir(dir: &Path) -> Result<Vec<Sibling>, String> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{} cannot be listed: {e}", dir.display())),
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("{} cannot be listed: {e}", dir.display()))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|n| format!("an entry of {} is not UTF-8: {n:?}", dir.display()))?;
        let is_dir = std::fs::metadata(entry.path()).map_or_else(
            |_| entry.file_type().is_ok_and(|t| t.is_dir()),
            |m| m.is_dir(),
        );
        out.push(Sibling { name, is_dir });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

/// The child's `--settings` element (`s8h` §8h.6; ADR-0197 decision 9, proposed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildSettings {
    json: String,
}

impl ChildSettings {
    /// The settings over a chain.
    ///
    /// `permissions.blockReadsOutsideWorkingDirectories: true` (the primary read fence);
    /// `permissions.deny` for `Read`, `Edit` and `Write` over the COMPLEMENT ALONG THE ANCESTOR
    /// CHAIN — at each level every listed entry not on the path to the worktree (a directory as
    /// `<path>/**`, anything else as `<path>`), then `//<letter>/**` for every drive letter other
    /// than the worktree's, then `~/**`; and `autoMemoryEnabled: false`. Paths take the documented
    /// form `//<drive>/<segments>` (`//d/repos/...`), the drive lower-case. No `!` carve-out and no
    /// allow rule: a `!` pattern cannot reach an anchored rule and an allow rule cannot carve an
    /// exception out of a deny rule [documented, `permissions.md`]. On Windows the path-segment
    /// comparison is ASCII-case-insensitive, as the filesystem is.
    ///
    /// **Residuals, stated rather than claimed away** (§8h.6): a sibling created after assembly is
    /// outside the set; a worktree under the home directory would be denied by `~/**` (run
    /// worktrees live under the repository's `.redkiln/worktrees/`, D21-44); an entry name carrying
    /// a glob metacharacter is written as listed.
    #[must_use]
    pub fn from_chain(chain: &ConfinementChain) -> Self {
        let root = chain
            .drive
            .map_or_else(|| "//".to_owned(), |d| format!("//{d}/"));
        let same = |a: &str, b: &str| {
            if chain.drive.is_some() {
                a.eq_ignore_ascii_case(b)
            } else {
                a == b
            }
        };
        let mut paths: Vec<String> = Vec::new();
        for (level, listing) in chain.listings.iter().enumerate() {
            let Some(on_path) = chain.segments.get(level) else {
                break;
            };
            let base = chain.segments[..level]
                .iter()
                .fold(root.clone(), |acc, s| format!("{acc}{s}/"));
            for entry in listing {
                if same(&entry.name, on_path) {
                    continue;
                }
                let p = format!("{base}{}", entry.name);
                paths.push(if entry.is_dir { format!("{p}/**") } else { p });
            }
        }
        if let Some(d) = chain.drive {
            for letter in 'a'..='z' {
                if letter != d {
                    paths.push(format!("//{letter}/**"));
                }
            }
        }
        paths.push("~/**".to_owned());
        let deny: Vec<String> = paths
            .iter()
            .flat_map(|p| ["Read", "Edit", "Write"].map(|tool| format!("{tool}({p})")))
            .collect();
        let value = serde_json::json!({
            "autoMemoryEnabled": false,
            "permissions": {
                "blockReadsOutsideWorkingDirectories": true,
                "deny": deny,
            },
        });
        Self {
            json: value.to_string(),
        }
    }

    /// The settings of a real worktree: [`ConfinementChain::read`], then [`Self::from_chain`].
    ///
    /// # Errors
    /// [`ConfigError::Confinement`].
    pub fn for_worktree(worktree: &Path) -> Result<Self, ConfigError> {
        Ok(Self::from_chain(&ConfinementChain::read(worktree)?))
    }

    /// The `--settings` element's bytes.
    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }
}

/// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY.)* The
/// CLI's settings-file limit, `Kp=2097152` [found by string search of the 2.1.287 binary, not
/// executed]. Settings over it are refused before any spawn ([`ConfigError::SettingsTooLarge`]).
pub const SETTINGS_FILE_CAP_BYTES: usize = 2_097_152;

/// The settings file's name prefix (D21-137): `child-settings-<pid>-<seq>-<sha256 prefix>.json`.
const SETTINGS_FILE_PREFIX: &str = "child-settings-";

/// The process-wide dispatch sequence a settings file's name carries, so two dispatches in one
/// process never share a name (D21-137: `create_new` with a per-dispatch unique name).
static SETTINGS_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// The sequence the next per-dispatch settings file in this process will carry — read by the live
/// probe 1, which predicts the file it tells the child to Read (D21-137 item 8). Spawns and writes
/// nothing.
#[doc(hidden)]
#[must_use]
pub fn next_settings_seq() -> u64 {
    SETTINGS_SEQ.load(std::sync::atomic::Ordering::Relaxed)
}

/// The settings file's name for `pid`, `seq` and `settings` — every field fixed-width, so every
/// name is as long as [`settings_stand_in`]'s.
#[must_use]
pub fn settings_file_name(pid: u32, seq: u64, settings: &ChildSettings) -> String {
    let sha = rk_types::digest::sha256(settings.json().as_bytes());
    format!(
        "{SETTINGS_FILE_PREFIX}{pid:010}-{seq:020}-{}.json",
        &sha.as_str()[..16]
    )
}

/// The stand-in path bind measures the argv with (D21-137 item 4): `dir` joined with a name of the
/// dispatch form at its fixed maximum length. Nothing is written there.
#[must_use]
pub fn settings_stand_in(dir: &Path) -> PathBuf {
    dir.join(format!(
        "{SETTINGS_FILE_PREFIX}{:010}-{:020}-{}.json",
        u32::MAX,
        u64::MAX,
        "f".repeat(16)
    ))
}

/// The settings directory, checked (D21-137 item 2): present as an input, absolute and not
/// verbatim, carrying no `.` or `..`, outside the worktree, and an existing directory — in that
/// order, so a directory under the worktree is refused as such whether or not it exists.
///
/// # Errors
/// [`ConfigError::SettingsDirUnset`]; [`ConfigError::SettingsPathRefused`] (not UTF-8, verbatim,
/// relative, or carrying `.`/`..`); [`ConfigError::SettingsDirUnderWorktree`];
/// [`ConfigError::SettingsDirMissing`].
pub fn check_settings_dir<'d>(
    dir: Option<&'d Path>,
    worktree: &Path,
) -> Result<&'d Path, ConfigError> {
    use std::path::Component;
    let dir = dir.ok_or(ConfigError::SettingsDirUnset)?;
    let refuse = |why: &'static str| ConfigError::SettingsPathRefused {
        path: dir.to_path_buf(),
        why,
    };
    let text = dir
        .to_str()
        .ok_or_else(|| refuse("the path is not valid UTF-8"))?;
    if text.starts_with(r"\\?\") || text.starts_with("//?/") {
        return Err(refuse(
            "the path is verbatim (`\\\\?\\`); the plain absolute form is required",
        ));
    }
    if !dir.is_absolute() {
        return Err(refuse("the path is not absolute"));
    }
    if dir
        .components()
        .any(|c| matches!(c, Component::CurDir | Component::ParentDir))
    {
        return Err(refuse("the path carries `.` or `..`"));
    }
    if is_at_or_under(dir, worktree) {
        return Err(ConfigError::SettingsDirUnderWorktree {
            dir: dir.to_path_buf(),
            worktree: worktree.to_path_buf(),
        });
    }
    if !dir.is_dir() {
        return Err(ConfigError::SettingsDirMissing {
            dir: dir.to_path_buf(),
        });
    }
    Ok(dir)
}

/// A path's comparison form (D21-137 item 2): the verbatim prefix stripped, `\` read as `/`, no
/// trailing separator, and on Windows ASCII-lower-cased, as the filesystem folds case.
fn comparison_form(path: &Path) -> String {
    let text = path.to_string_lossy();
    let text = text
        .strip_prefix(r"\\?\UNC\")
        .map(|rest| format!(r"\\{rest}"))
        .unwrap_or_else(|| {
            text.strip_prefix(r"\\?\")
                .or_else(|| text.strip_prefix("//?/"))
                .unwrap_or(&text)
                .to_owned()
        });
    let text = text.replace('\\', "/");
    let text = text.trim_end_matches('/').to_owned();
    if cfg!(windows) {
        text.to_ascii_lowercase()
    } else {
        text
    }
}

/// Is `dir` the worktree itself or anything under it, compared in [`comparison_form`]?
fn is_at_or_under(dir: &Path, worktree: &Path) -> bool {
    let dir = comparison_form(dir);
    let worktree = comparison_form(worktree);
    dir == worktree || dir.starts_with(&format!("{worktree}/"))
}

/// The settings' size against [`SETTINGS_FILE_CAP_BYTES`]: over it is refused.
///
/// # Errors
/// [`ConfigError::SettingsTooLarge`] naming the measure and the limit.
pub fn check_settings_size(settings: &ChildSettings) -> Result<(), ConfigError> {
    let bytes = settings.json().len();
    if bytes > SETTINGS_FILE_CAP_BYTES {
        return Err(ConfigError::SettingsTooLarge {
            bytes,
            cap: SETTINGS_FILE_CAP_BYTES,
        });
    }
    Ok(())
}

/// The `--settings` element for `path`: its UTF-8 text, refused when it has none.
fn settings_path_element(path: &Path) -> Result<String, ConfigError> {
    path.to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| ConfigError::SettingsPathRefused {
            path: path.to_path_buf(),
            why: "the path is not valid UTF-8",
        })
}

/// The path of the next per-dispatch settings file in `dir`, after the directory checks.
///
/// # Errors
/// [`check_settings_dir`]'s.
pub fn next_settings_path(
    dir: Option<&Path>,
    worktree: &Path,
    settings: &ChildSettings,
) -> Result<PathBuf, ConfigError> {
    let dir = check_settings_dir(dir, worktree)?;
    let seq = SETTINGS_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(dir.join(settings_file_name(std::process::id(), seq, settings)))
}

/// Write `settings` to `path` with `create_new`, mark it read-only, and return what was delivered:
/// the path as the `--settings` element names it and the SHA-256 of the bytes written, which are
/// exactly [`ChildSettings::json`] (D21-137 item 3). The file is never deleted here: it is kept
/// with the run's evidence.
///
/// # Errors
/// [`ConfigError::SettingsTooLarge`]; [`ConfigError::SettingsPathRefused`] (not UTF-8);
/// [`ConfigError::SettingsWrite`] for a name that already exists (`create_new`: nothing is ever
/// overwritten) or any other I/O failure. No caller spawns without the file.
pub fn write_settings_file(
    path: &Path,
    settings: &ChildSettings,
) -> Result<rk_types::activity::SettingsFile, ConfigError> {
    use std::io::Write as _;
    check_settings_size(settings)?;
    let element = settings_path_element(path)?;
    let failed = |e: &std::io::Error| ConfigError::SettingsWrite {
        path: path.to_path_buf(),
        detail: e.to_string(),
    };
    let bytes = settings.json().as_bytes();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| failed(&e))?;
    file.write_all(bytes).map_err(|e| failed(&e))?;
    file.sync_all().map_err(|e| failed(&e))?;
    drop(file);
    let mut perms = std::fs::metadata(path)
        .map_err(|e| failed(&e))?
        .permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(path, perms).map_err(|e| failed(&e))?;
    Ok(rk_types::activity::SettingsFile {
        path: element,
        sha256: rk_types::digest::sha256(bytes),
    })
}

/// What one assembly takes.
///
/// The bound card, the ONE output schema, adoption, the strict tools a spec would add (always
/// empty from the binder; the AC-132 fixture adds one), and the user message (stdin — never argv).
#[derive(Debug, Clone, Copy)]
pub struct CommandSpec<'a> {
    /// The bound card (the role's tier injected — `binder.rs`).
    pub card: &'a AgentCard,
    /// The one output schema, as JSON; the `--json-schema` element is its compact serialisation.
    pub schema: &'a serde_json::Value,
    /// `--agent <name>` beside `--agents` — `false` is AC-203's refusal.
    pub adopt_agent: bool,
    /// Strict user-defined tools a spec would put on the command line — any is AC-132's refusal.
    pub strict_tools: &'a [String],
    /// The user message, written to the child's stdin then shut (Slice 5). Under `default` the
    /// card body is prepended to it here.
    pub user_message: &'a str,
    /// *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; `s8h` §8h.6.)* The child's
    /// confinement settings, the `--settings` element, assembled from the spec's worktree.
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY.)*
    /// The settings AND the path of the file that carries them: the element is the path, and the
    /// settings are what the file's bytes are (checked against [`SETTINGS_FILE_CAP_BYTES`] here).
    pub settings: Option<SettingsArg<'a>>,
}

/// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY;
/// ADR-0197 decision 9 as amended, proposed.)* The `--settings` element's two halves: the
/// settings the file carries and the absolute path the element names. At bind the path is
/// [`settings_stand_in`] (nothing is written); at dispatch it is the real per-dispatch file.
#[derive(Debug, Clone, Copy)]
pub struct SettingsArg<'a> {
    /// What the file's bytes are: exactly [`ChildSettings::json`].
    pub settings: &'a ChildSettings,
    /// The file the `--settings` element names.
    pub path: &'a Path,
}

/// What the builder produced: the program, the argv (the golden vector), the stdin payload, and
/// the command line's measure against the cap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assembled {
    /// `Config::harness_binary`.
    pub program: PathBuf,
    /// The argv, in the ONE stated order.
    pub argv: Vec<String>,
    /// The bytes written to stdin before it is shut: the user message, with the card body at its
    /// head under `default`.
    pub stdin: String,
    /// [`command_line_units`] of `program` + `argv` — under the cap, or this value would not exist.
    pub command_line_units: usize,
}

/// The UTF-16 length of one element as `std` quotes it for `CreateProcessW` (MSVCRT rules).
fn quoted_units(arg: &str) -> usize {
    let quote = arg.is_empty() || arg.contains(' ') || arg.contains('\t');
    let mut units = if quote { 2 } else { 0 };
    let mut backslashes = 0usize;
    for c in arg.chars() {
        if c == '\\' {
            backslashes += 1;
        } else {
            if c == '"' {
                units += backslashes + 1;
            }
            backslashes = 0;
        }
        units += c.len_utf16();
    }
    if quote {
        units += backslashes;
    }
    units
}

/// The command line `std::process::Command` would build for `program` + `argv` on Windows, in
/// UTF-16 units.
///
/// The quoted program, a space and the quoted-and-escaped element for each element, and the
/// terminating null — what `CreateProcessW`'s 32,767 limit counts (the module doc).
///
/// **For a non-batch program only**: a `.bat`/`.cmd` is spawned by `std` through `cmd.exe /d /c`
/// on a different, longer line; `Config::new` refuses such a program so this measure is never
/// asked about one.
#[must_use]
pub fn command_line_units(program: &Path, argv: &[String]) -> usize {
    let program: usize = program.to_string_lossy().chars().map(char::len_utf16).sum();
    2 + program + argv.iter().map(|a| 1 + quoted_units(a)).sum::<usize>() + 1
}

/// Assemble the command: the golden argv in the ONE stated order, the stdin payload per profile,
/// and every argv-level lint of §7c.4 — refused before any spawn.
///
/// # Errors
/// The card-level lints over the bound card (`card::lint_card`: [`ConfigError::CardNameEmpty`],
/// [`ConfigError::ToolsEmpty`], [`ConfigError::AnswerApiExposed`],
/// [`ConfigError::PermissionModeAbsent`]); then [`ConfigError::AgentsWithoutAgent`] (AC-203);
/// [`ConfigError::StructuredOutputAbsent`] (AC-205); [`ConfigError::StrictToolOnArgv`] (AC-132);
/// [`ConfigError::ArgvAtOrOverCap`] (AC-523's cap).
pub fn assemble(config: &Config, spec: &CommandSpec<'_>) -> Result<Assembled, ConfigError> {
    let card = spec.card;
    // The card-level lints, over the BOUND card, whatever built it (§7c.12).
    lint_card(&card.name, &card.tools, &card.permission_mode)?;
    // X12: the pin's spelling row exists, refused before anything else is built.
    argv_permission_mode(config, &card.permission_mode)?;
    // AC-203: `--agents` alone defines; only `--agent` adopts (F7).
    if !spec.adopt_agent {
        return Err(ConfigError::AgentsWithoutAgent {
            card: card.name.clone(),
        });
    }
    // AC-205: every dispatch here carries a schema, so the declared list must carry the tool.
    if !card.tools.iter().any(|t| t == STRUCTURED_OUTPUT) {
        return Err(ConfigError::StructuredOutputAbsent {
            card: card.name.clone(),
        });
    }
    // AC-132: no strict user-defined tool on the command line.
    if let Some(tool) = spec.strict_tools.first() {
        return Err(ConfigError::StrictToolOnArgv { tool: tool.clone() });
    }

    let mut argv: Vec<String> = vec![
        "-p".to_owned(),
        "--output-format".to_owned(),
        "stream-json".to_owned(),
        "--verbose".to_owned(),
        "--model".to_owned(),
        card.model.clone(),
    ];
    // Present exactly when the bound card carries one (the binder puts it there only on a tier
    // that accepts it — the lifted rule).
    if let Some(effort) = &card.effort {
        argv.push("--effort".to_owned());
        argv.push(effort.clone());
    }
    argv.push("--agents".to_owned());
    argv.push(card.to_agents_json());
    argv.push("--agent".to_owned());
    argv.push(card.name.clone());
    argv.push("--json-schema".to_owned());
    argv.push(spec.schema.to_string());
    // AC-214: unconditional, the card's explicit value — *(dated 2026-10-02, s8h build, package
    // P3-adapter-argv-models; s8h §8h.2; D21-48 X12)* spelt on the argv by PERMISSION_MODE_ARGV's
    // row for the Config's pin when the card says `default`. Only this element moves: the card,
    // the --agents JSON and the digest keep `default`.
    argv.push("--permission-mode".to_owned());
    argv.push(argv_permission_mode(config, &card.permission_mode)?);
    // AC-202: unconditional; the flag in argv is never evidence of isolation — `system/init` is.
    argv.push("--strict-mcp-config".to_owned());
    // AC-215 (argv half): unconditional.
    argv.push("--permission-prompts".to_owned());
    argv.push("none".to_owned());
    // *(dated 2026-10-02, s8h build, package P3-adapter-argv-models; s8h §8h.6; ADR-0197 decision 9,
    // proposed.)* Unconditional and EMPTY: none of the user, project or local setting files loads
    // (the parser begins `if(e==="")return[];` [measured by string search, 2.1.281 and 2.1.287]).
    argv.push("--setting-sources".to_owned());
    argv.push(String::new());
    // The child's confinement, one element, assembled from the spec's worktree (§8h.6).
    // *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; D21-137, TAKEN PROVISIONALLY.)*
    // The element is the settings FILE's path, never the JSON: the CLI reads the file. The
    // settings are checked against the CLI's file limit here, on the path bind and dispatch share.
    if let Some(arg) = spec.settings {
        check_settings_size(arg.settings)?;
        argv.push("--settings".to_owned());
        argv.push(settings_path_element(arg.path)?);
    }
    // AC-383 / AC-211: the delivery route, LAST under static (h1-sysprompt's position).
    let stdin = match card.cache_profile {
        CacheProfile::Static => {
            argv.push("--system-prompt".to_owned());
            argv.push(card.prompt.clone());
            spec.user_message.to_owned()
        }
        CacheProfile::Default => format!(
            "{}

{}",
            card.prompt, spec.user_message
        ),
    };

    let program = config.harness_binary().to_path_buf();
    let units = command_line_units(&program, &argv);
    // AC-523's cap (RT-1): at or over the cap is refused, before spawn.
    if units >= ARGV_CAP_BYTES {
        return Err(ConfigError::ArgvAtOrOverCap {
            units,
            cap: ARGV_CAP_BYTES,
        });
    }
    Ok(Assembled {
        program,
        argv,
        stdin,
        command_line_units: units,
    })
}
