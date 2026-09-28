//! **`init` — non-interactive, eager, idempotent** (AC-437 as amended by **D-14**, AC-531, AC-412;
//! ADR-0107 decision 2; `s7d` §7d.9).
//!
//! It does exactly ~~two~~ three things: writes `.redkiln/config.toml` **if and only if the file is
//! absent**, ~~and~~ opens the store, which creates it under the common directory and stamps its
//! identity rows on first open, *and appends the `# redkiln-runtime` block to
//! `<commondir>/info/exclude` **if and only if its marker line is absent*** *(dated 2026-09-27,
//! `s8d-ii` build, package P5: D21-45 W4, ADR-0180, proposed)*. It prompts nothing, reads no stdin
//! (D-15), installs no hook (AC-412) and creates no git ref.
//!
//! **The exclude block is `init`'s first write to a file git itself reads** (`s8d-ii` §8d-ii.5).
//! It is not its first write under the common dir — the store is there already — but `info/exclude`
//! changes what `git status` reports, so its idempotence is new, not inherited: a second `init`
//! finds the marker and writes nothing. Lines the operator wrote are never rewritten, and
//! `.gitignore` is never written (D21-45: *"never to `.gitignore`"*). `info/exclude` sits in the
//! common dir, outside `hooks`, so AC-531's lint surfaces and [`CreatedNames`] do not change.
//!
//! **Eager, and that was a decision** (D-14; the rejected alternative was lazy creation with an
//! empty `[adapters]`). The four reals are compiled in, so there is nothing to discover and nothing
//! to ask; a scaffold that has to be filled in later is a scaffold that is wrong in the meantime.
//!
//! **Idempotent by ABSENCE, not by comparison.** A second `init` does not read the file, diff it and
//! rewrite it — it does not write at all when the file is there, *"whatever it says"* (ADR-0107
//! decision 2). That is why the test compares bytes and the porcelain listing rather than mtimes: a
//! rewrite that happened to produce identical bytes would still be a write, and a repository whose
//! operator has edited the file must not have that edit reverted by a command that claims to change
//! nothing.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::Path;

use crate::exit::{Exit, Halt};
use crate::repo;

/// `doctor`'s unread window, as `init` writes it. **One literal** (H-4; **D18-5**), and the scaffold
/// below is rendered from it — so the declaration and the written key cannot disagree.
pub const INIT_UNREAD_WINDOW_RUNS: u32 = 50;

/// AC-506's regex, quoted once. The matcher below implements it; this string is what the test pins
/// and what any future reader compares against, so the two cannot drift.
///
/// AC-506 itself is **Phase 2** (`rk-plugin-gen`) and is not claimed here.
pub const NAME_REGEX: &str = "^rk-[a-z0-9][a-z0-9-]*$";

/// Does `name` match [`NAME_REGEX`]?
///
/// Hand-written, because no regex crate is a dependency of this workspace and §7d.16 item 2's
/// dependency set is not widened for one matcher. The shape is small and total: `rk-`, then one
/// `[a-z0-9]`, then any number of `[a-z0-9-]`.
#[must_use]
pub fn name_is_admissible(name: &str) -> bool {
    let Some(rest) = name.strip_prefix("rk-") else {
        return false;
    };
    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The manifest of names `init` created, as AC-531 scopes it (D-14: the working tree, refs and
/// hooks).
///
/// **Empty at 7d, and the emptiness is the point.** The scaffold is inside `.redkiln/`, which the
/// criterion exempts by name — *"every name it created **outside** the repository's own `.redkiln/`
/// directory"* — and the store is in the common directory, listed separately and unlinted. So the
/// lint runs over nothing, which is exactly why the test also enumerates the real tree diff: a lint
/// over an empty manifest passes whatever the regex says.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CreatedNames {
    lintable: Vec<String>,
}

impl CreatedNames {
    /// The manifest as Step 7d's `init` produces it.
    #[must_use]
    pub fn at_seven_d() -> Self {
        Self::default()
    }

    /// The names the naming lint applies to.
    #[must_use]
    pub fn lintable(&self) -> &[String] {
        &self.lintable
    }

    /// Every name in the manifest matches [`NAME_REGEX`].
    #[must_use]
    pub fn all_admissible(&self) -> bool {
        self.lintable.iter().all(|n| name_is_admissible(n))
    }
}

/// The marker line that opens `init`'s block in `<commondir>/info/exclude` (D21-45 W4).
///
/// The block is written when no line of the file equals it, a trailing CR dropped; a line that
/// merely starts with it is the operator's.
pub const EXCLUDE_BEGIN: &str = "# redkiln-runtime";

/// The line that closes the block.
pub const EXCLUDE_END: &str = "# end redkiln-runtime";

/// The block's bytes: the marker, the run-tree root's ignore row from `rk_git::worktree` (the
/// root's one home, D21-44), and the closing line. LF only, one trailing newline.
#[must_use]
pub fn exclude_block() -> String {
    format!(
        "{EXCLUDE_BEGIN}\n{}\n{EXCLUDE_END}\n",
        rk_git::worktree::RUN_ROOT_IGNORE_ROW
    )
}

/// Does any line of `bytes` equal the marker?
fn has_exclude_marker(bytes: &[u8]) -> bool {
    bytes
        .split(|b| *b == b'\n')
        .any(|line| line.strip_suffix(b"\r").unwrap_or(line) == EXCLUDE_BEGIN.as_bytes())
}

/// Append the block to `<common>/info/exclude` unless the marker is there, creating `info/` and the
/// file when git's template did not. The existing bytes are never rewritten: the write is an
/// append, preceded by one `\n` only when the operator's last line is unterminated, so the marker
/// is never glued onto it.
fn append_exclude_block(common: &Path) -> std::io::Result<()> {
    let info = common.join("info");
    let path = info.join("exclude");
    let existing = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    if has_exclude_marker(&existing) {
        return Ok(());
    }
    std::fs::create_dir_all(&info)?;
    let mut add = Vec::new();
    if !existing.is_empty() && !existing.ends_with(b"\n") {
        add.push(b'\n');
    }
    add.extend_from_slice(exclude_block().as_bytes());
    std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?
        .write_all(&add)
}

/// The scaffold's bytes, rendered from the one literal above.
///
/// LF only, no trailing whitespace, one trailing newline. The comment header is part of the pinned
/// bytes — `ac437_init_scaffold_bytes_pinned` compares the whole file, so a dropped comment is a
/// failure, not a tidy-up.
fn scaffold() -> String {
    let mut out = String::new();
    out.push_str(
        "# Written by `redkiln init` (Step 7d). Step 7e fills [packs].enabled; every other key is read at startup\n\
         # (AC-485/AC-486) and by `redkiln doctor` (AC-426). A second `init` leaves this file untouched (ADR-0107 d2).\n\
         \n\
         [adapters]\n\
         harness = \"rk-adapter-claude\"\n\
         store = \"rk-adapter-sqlite\"\n\
         toolchains = [\"rk-adapter-toolchain-node\", \"rk-adapter-toolchain-rust\"]\n\
         \n\
         [packs]\n\
         enabled = []\n\
         \n\
         [doctor]\n",
    );
    let _ = writeln!(out, "unread_window_runs = {INIT_UNREAD_WINDOW_RUNS}");
    out
}

/// Run `init`.
///
/// # Errors
/// Never a [`Halt`] at this slice: the store acquisition ADR-0118 guards is the loop's, not
/// `init`'s. The signature is the one every verb has (clause 6), so the root's exit path is one.
pub fn run(cwd_arg: Option<&Path>) -> Result<Exit, Halt> {
    let repo = match repo::discover(cwd_arg) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let root = Path::new(repo.worktree.as_str());

    // AC-531's AND: a pre-existing `rk-` name that `init` did not create is REPORTED, never
    // touched. Reported before anything is written, so the operator sees it even if a later step
    // refuses.
    for name in preexisting_rk_names(root) {
        eprintln!(
            "redkiln: `{name}` already exists and was not created by `init`; it is left untouched \
             (AC-531)"
        );
    }

    let manifest = CreatedNames::at_seven_d();
    if !manifest.all_admissible() {
        eprintln!(
            "redkiln: `init` would create a name outside `{NAME_REGEX}`: {:?}",
            manifest.lintable()
        );
        return Ok(Exit::ConfigError);
    }

    // The scaffold, written only when absent (ADR-0107 d2).
    let dir = root.join(".redkiln");
    let config = dir.join("config.toml");
    if !config.exists() {
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("redkiln: could not create `.redkiln/`: {e}");
            return Ok(Exit::ConfigError);
        }
        if let Err(e) = std::fs::write(&config, scaffold().as_bytes()) {
            eprintln!("redkiln: could not write `.redkiln/config.toml`: {e}");
            return Ok(Exit::ConfigError);
        }
    }

    // The `# redkiln-runtime` block, appended only when its marker is absent (D21-45 W4).
    if let Err(e) = append_exclude_block(&repo.common_dir) {
        eprintln!("redkiln: could not write the `{EXCLUDE_BEGIN}` block to `info/exclude`: {e}");
        return Ok(Exit::ConfigError);
    }

    // The store, eagerly. On a second run this re-reads the identity rows rather than re-stamping
    // them, which is what AC-437's "identical store identity rows" is about.
    if let Err(e) = rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree) {
        eprintln!("redkiln: the store could not be opened: {e}");
        return Ok(Exit::ConfigError);
    }

    Ok(Exit::Success)
}

/// Working-tree entries whose name matches the `rk-` prefix the naming lint governs.
///
/// Top level only: AC-531 is about names `init` would create, and `init` creates nothing nested.
fn preexisting_rk_names(root: &Path) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("rk-"))
        .collect();
    names.sort();
    names
}
