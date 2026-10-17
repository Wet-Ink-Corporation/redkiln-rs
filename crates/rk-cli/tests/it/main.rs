//! `rk-cli`'s **one** integration binary — a `mod` per case, never one file per case (ADR-0083).
//! This is also what makes `--test it` addressable in Checkpoint 7d.
//!
//! What is here is **Slices 1 to 3**. Slice 1: [`surface`], the `clap` router — its committed
//! `--help` snapshot, the resume-shaped denylist walked over the built parser tree, the two grammars
//! ADR-0128 and ADR-0129 fix, and the one source ban AC-403's lint half names; and [`exit`], the
//! closed exit table. Slice 2: [`config`], the configuration root — the required key, the key
//! surface, the one denylist and the refusal before spawn or store write; [`init`], the eager
//! idempotent scaffold; and [`root`], the poisoned-lock halt and the signal handler. Slice 3:
//! [`catalog`] and [`lint`], the read verbs; [`reconcile`], AC-402's negative; [`run`], the loop;
//! [`answer`], the gate's other end; and [`explain`], which prints the record and recomputes
//! nothing.
//!
//! **No empty module stands in for an unbuilt one** (`rk-kernel`'s `tests/it/main.rs` states the rule
//! and this file inherits it). *(Corrected 2026-09-19 at Slice 3, cycle 2; corrected again
//! 2026-09-20 at cycle 7, and again at cycle 9. The cycle-2 read was **"there is no `run`,
//! `answer`, `doctor` or `migrate` module yet, and the first two arrive with their own cases later
//! in this slice"** —
//! true when written and now false three times over: [`run`] landed at cycles 4 to 6,
//! [`answer`] at cycle 7 and [`explain`] at cycle 9. What survives is the rule and the two that
//! still have no module: there is no `doctor` module (Slice 4 builds what it would assert) and no
//! `migrate` module (Slice 5's).
//! **Corrected a fourth time 2026-09-21, Step 7d Slice 4 cycle 3: [`doctor`] exists**, and one of
//! the two the sentence above still excepted is now built. It holds
//! `doctor_rule_table_covers_every_register_named_row` — RA-M-01's anti-vacuity floor — and that
//! one test is the whole module, which is what the rule the sentence carries requires: a module
//! that asserts something is not an empty one. `migrate` remains Slice 5's.
//! **Corrected a fifth time 2026-09-21, Step 7d Slice 5: `migrate` is built, and it is NOT a module
//! of this binary — nor will it ever be.** The migrator's tests live in
//! `crates/redkiln-migrate/tests/it/`, because an integration binary cannot link another crate's
//! integration binary and AC-441 makes `redkiln-migrate` a separate crate nothing in the product
//! reaches. So the clause *"there is no `migrate` module"* stays TRUE permanently, for a different
//! reason than the one it was written with: not *not yet*, but *never here*. The four earlier
//! corrections each recorded a module arriving; this one records that the last name on the list
//! belongs to another crate, and the sentence is closed rather than waiting.)*
//! [`support`] remains the one module in this binary the rule does not reach — it **holds no test
//! of the product at all**, by design, carrying only the two self-controls of its own spawn bound,
//! so it cannot read as coverage of anything else.)* A `mod` that exists and asserts nothing reads
//! as coverage.
//!
//! **[`answer`] opens with a scan and no product code, and that is the sequencing, not an
//! oversight.** `ac405_gate_answer_has_exactly_one_call_site` is *red at zero* before
//! `src/verbs/answer.rs` exists, and it is what fixes that file's import spelling — an import of
//! the symbol counts as a call site, so the verb must reach it through a module import. A module
//! holding one honest red is not an empty module.
//!
//! **The two source-scan primitives live HERE, not in `surface.rs`.** [`rs_files`] and [`code_lines`]
//! are used by `exit::ac438_step_error_mapping_is_exhaustive` and by
//! `surface::ac403_no_blocking_stdin_read_in_rk_cli`, which sit in different modules; a helper in
//! either one is unreachable from the other. *(Corrected 2026-09-19 at Slice 3, cycle 2. This read:
//! **"`tests/it/support.rs` — the scripted-harness helpers — is Slice 3's (§7d.16 item 3) and is not
//! created early to hold two functions."** It is created NOW, and it holds nine seams rather than
//! two functions. **Nothing moved out of this file to reach it**: `support.rs` is a sibling `mod` of
//! the same binary, so `crate::temp_repo()`, `crate::git(..)` and `crate::code_lines(..)` are
//! reachable from it, and the reason the primitives live here is about CASE modules reaching each
//! other, which is unchanged.)*
//!
//! **The division, stated so it is not re-litigated.** This file holds the *process-and-tree*
//! primitives ([`redkiln`], [`temp_repo`], [`git`], [`run_init`], [`refs`], [`porcelain`],
//! [`common_dir`]) and the *source-scan* primitives ([`src_dir`], [`lint_controls_dir`],
//! [`rs_files`], [`code_lines`], [`assert_no_block_comment`]). [`support`] holds the *in-process
//! seam* values only.

mod adapter_selection;
mod answer;
mod cards;
mod catalog;
mod change_deriver;
mod checklist_bridge;
mod config;
mod cost;
mod demands;
mod doctor;
mod doctor_artifact;
mod doctor_drift;
mod doctor_journal;
mod exit;
mod explain;
mod init;
mod item_create;
mod landing;
mod lane_e2e;
mod lint;
mod processes;
mod reconcile;
mod review_input;
mod review_pass;
mod root;
mod run;
mod run_worktree;
mod schemas;
mod slice_verdict;
mod status;
mod story_axis;
mod support;
mod surface;
mod tag_release;
mod toolchain;
mod transcription;
mod verb_rows;
mod verify_pass;
mod worktree_assigned;

use std::path::{Path, PathBuf};

/// The crate's own `src/` directory, resolved from the manifest rather than from the process cwd.
///
/// `env::current_dir` is banned in `crates/rk-cli/src` and the ban's scope is the source tree, not
/// the test tree — but a test that read the cwd would still be pinning the harness's choice of
/// working directory rather than the crate, and `cargo test` does not promise one.
pub fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// The `tests/fixtures/` root.
pub fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// The lint-control directory every in-process source-scan test controls itself against (§7d.13).
pub fn lint_controls_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("lint-controls")
}

/// Every `.rs` file under `root`, recursively, sorted — the scan surface of a source assertion.
///
/// **Sorted, and that is not cosmetic**: a scanner whose file order came from the filesystem would
/// report a different first hit on a different box, and the failure message is part of what makes a
/// red readable.
///
/// Panics rather than returning an empty vector if `root` does not exist. An absence ban over a
/// directory that is not there is the Halt 8 shape this whole file exists to refuse: it reports a
/// pass having read nothing.
pub fn rs_files(root: &Path) -> Vec<PathBuf> {
    assert!(
        root.is_dir(),
        "the scan root {} does not exist — an absence assertion over a missing directory reads as a \
         pass having read nothing (Halt 8 shape (a))",
        root.display()
    );
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries =
            std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("read_dir {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("a directory entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    assert!(
        !out.is_empty(),
        "no .rs file under {} — the scan would read nothing and report a pass",
        root.display()
    );
    out
}

/// A source line that carries CODE, as `(1-indexed line number, content)`.
///
/// Two filters, and the second is the one this project has been burned by. **The first** is the
/// block's own, applied verbatim so a Rust scanner and the checkpoint's shell scanner agree on what
/// a comment is: drop every line whose `trim_start()` begins `//`, `/*` or `*`. **The second** strips
/// a trailing `// …` from the lines that survive, because the block's filter is line-leading only and
/// a ban that fires on the prose explaining it is a guard satisfied by its own text — a defect this
/// tree has shipped before and now tests for.
///
/// The trailing strip tracks double-quoted string state so a `//` inside a literal is not a comment
/// opener. It does not track raw strings or char literals: [`assert_no_block_comment`] refuses the
/// one construct that would make that matter, so an unhandled shape stops the scan loudly instead of
/// silently under-reading.
pub fn code_lines(src: &str) -> Vec<(usize, String)> {
    src.lines()
        .enumerate()
        .map(|(i, line)| (i + 1, line))
        .filter(|(_, line)| {
            let t = line.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .map(|(n, line)| (n, strip_trailing_comment(line)))
        .filter(|(_, line)| !line.trim().is_empty())
        .collect()
}

/// Cut `line` at the first `//` that lies outside a double-quoted string.
fn strip_trailing_comment(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut in_string = false;
    let mut escaped = false;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'"' {
                in_string = false;
            }
        } else if c == b'"' {
            in_string = true;
        } else if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            return line[..i].to_owned();
        }
        i += 1;
    }
    line.to_owned()
}

/// Refuse a region carrying a `/* … */` block comment.
///
/// [`code_lines`] drops a line that OPENS a block comment and every continuation line that begins
/// `*`, but a block comment closed mid-line, or one whose continuation lines are unprefixed, would
/// leave commented text in the scan. Rather than grow the scanner until it is a parser, the one
/// construct it cannot read is refused loudly: a region that grows a block comment fails the test
/// that reads it, and the failure names the fix.
pub fn assert_no_block_comment(region: &str, what: &str) {
    assert!(
        !region.contains("/*"),
        "{what} carries a /* block comment — the line-leading comment filter cannot read one \
         reliably, and a scan that silently under-reads its own region is worth nothing. Write the \
         explanation as // lines."
    );
}

/// The `redkiln` binary, located the one way §7d.13 fixes: as the sibling of the path cargo hands
/// the test at compile time. Never `assert_cmd`'s `cargo_bin`, which guesses.
pub fn redkiln() -> &'static str {
    env!("CARGO_BIN_EXE_redkiln")
}

/// Whether a spawned `redkiln` is handed a launch record (`s8h` §8h.7; D21-47). *(dated
/// 2026-10-02, s8h build, package P6-cli-launch-gate.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Launch {
    /// The variable is removed and not set: the child sees no launch record, whatever this test
    /// process inherited (under `s8j`'s `wrap` it would carry a real one).
    Withheld,
    /// The variable names the committed fixture record, [`fixture_launch_record`]: every
    /// binary-level `run` (the real harness at the fixture child) and every binary `answer`.
    Fixture,
}

/// The committed fixture launch record (`tests/fixtures/launch/fixture-launch-record.txt`), by
/// absolute path. The launch check reads only that it names an existing file.
///
/// # Panics
/// If the fixture is absent: a run handed a path to nothing would refuse for the wrong reason.
#[must_use]
pub fn fixture_launch_record() -> PathBuf {
    let path = fixtures_dir()
        .join("launch")
        .join("fixture-launch-record.txt");
    assert!(
        path.is_file(),
        "the fixture launch record {} is absent",
        path.display()
    );
    path
}

/// **The ONE `Command` over the `redkiln` binary in this test tree** (`s8h` §8h.7; D21-47:
/// *"Launch-gate tests build their `Env` explicitly, enforced by a lint"*). *(dated 2026-10-02,
/// s8h build, package P6-cli-launch-gate.)*
///
/// It removes `REDKILN_LAUNCH_RECORD` first, so no spawn inherits the test process's value, and
/// sets it only to the fixture record when the caller says so. Every spawn site folds into it, and
/// `run::every_binary_spawn_withholds_or_supplies_the_launch_record` asserts that exactly one code
/// line constructs such a `Command`, here, with the removal (mutant M26).
///
/// It also removes every inherited [`provider_vars`] name (issue #11), so a shell exporting a
/// provider credential or a Claude Code session cannot turn a test red. A test that wants the
/// child to see one sets it on the returned `Command`, which wins over the removal.
#[expect(
    clippy::disallowed_types,
    reason = "every verb exercised AS A PROCESS goes through this one constructor (golden \
              constraint 4's 'a stub never replaced'; s7d §7d.13's temp-repository rule), so the \
              launch record is withheld or supplied at one site (s8h §8h.7)"
)]
pub fn redkiln_command(launch: Launch) -> std::process::Command {
    let mut cmd = std::process::Command::new(redkiln());
    cmd.env_remove(rk_cli::env::LAUNCH_RECORD);
    for name in provider_vars(std::env::vars_os().map(|(name, _)| name)) {
        cmd.env_remove(name);
    }
    if launch == Launch::Fixture {
        cmd.env(rk_cli::env::LAUNCH_RECORD, fixture_launch_record());
    }
    cmd
}

/// The name prefixes a Claude Code session or a provider exports (issue #11). Wider than the
/// product's denylist on purpose: the harness withholds them all, the product refuses its list.
const PROVIDER_PREFIXES: [&str; 2] = ["CLAUDE_", "ANTHROPIC_"];

/// The session names outside those prefixes that `s8h` §8h.7 step 2's launcher refuses.
const SESSION_NAMES: [&str; 2] = ["CLAUDECODE", "AI_AGENT"];

/// **What a test's `redkiln` must not inherit: the one source of truth** (issue #11).
///
/// A name the product refuses (`rk_adapter_claude::env_denylist`, ASCII case-insensitive as it
/// is), or a Claude Code session or provider name ([`PROVIDER_PREFIXES`], [`SESSION_NAMES`]).
#[must_use]
pub fn is_provider_var(name: &str) -> bool {
    rk_adapter_claude::env_denylist::is_denylisted(name)
        || PROVIDER_PREFIXES.iter().any(|p| {
            name.get(..p.len())
                .is_some_and(|head| head.eq_ignore_ascii_case(p))
        })
        || SESSION_NAMES.iter().any(|n| n.eq_ignore_ascii_case(name))
}

/// This test process's environment minus every [`is_provider_var`] name (issue #11).
///
/// For the in-process tests that hand a real environment to an `Env` or an adapter `Config`: the
/// spawn path's scrub, applied to a map. Names or values that are not UTF-8 are dropped, as
/// `rk_cli::env::Env::capture` drops them.
#[must_use]
pub fn inherited_env() -> std::collections::BTreeMap<String, String> {
    std::env::vars_os()
        .filter_map(|(k, v)| match (k.into_string(), v.into_string()) {
            (Ok(k), Ok(v)) if !is_provider_var(&k) => Some((k, v)),
            _ => None,
        })
        .collect()
}

/// The names in `inherited` that [`is_provider_var`] matches, in order (issue #11).
///
/// [`redkiln_command`] removes each from its child. A name that is not UTF-8 is never yielded: the
/// product's capture drops it (`rk_cli::env::Env::capture`), so it cannot trip a refusal.
pub fn provider_vars(
    inherited: impl IntoIterator<Item = std::ffi::OsString>,
) -> impl Iterator<Item = std::ffi::OsString> {
    inherited
        .into_iter()
        .filter(|name| name.to_str().is_some_and(is_provider_var))
}

/// A temp repository with one commit, and nothing else.
///
/// # 🛑 EVERY REPOSITORY GETS A **DISTINCT ROOT COMMIT**, and that is not decoration
///
/// *(Added at Slice 3, cycle 3, after a test failed for this reason.)* This helper used to write the
/// same `README` with the same author and message every time, and git's commit SHA is a hash of
/// exactly those — so **two repositories built in the same second were byte-identical and shared a
/// root commit SHA** (measured: two bare `temp_repo()` calls both produced
/// `60bfaf79069a19ce210a4ed2ebcd3f7c6a50273d`). `rk_store::identity::check` compares `root_commit`
/// **first**, so a store copied from one into the other was accepted as its own, and
/// `ac413_store_opened_from_other_repo_refused_zero_rows` exited `0` where the criterion requires
/// `3`. Any test that distinguishes two repositories by identity was vacuous against the old helper.
///
/// The nonce is a process-wide counter, not a clock: `Instant::now` and `SystemTime::now` are on
/// `clippy.toml`'s `disallowed-methods`, and a counter is both sufficient (the collision is between
/// repositories in ONE test binary) and deterministic.
///
/// # The root is in git's own (long) spelling
///
/// *(Added 2026-10-03, the first `windows-latest` run.)* A hosted Windows runner's `%TEMP%` is an
/// 8.3 short path (`C:\Users\RUNNER~1\…`), while git — and so redkiln, which derives every run tree
/// from `main_worktree()` — reports the long one (`C:/Users/runneradmin/…`). A root built from the
/// short spelling made every expected path from `run_tree(root, ..)` miss git's listing, and seven
/// `rk-cli` tests failed on a correct product. The temp base is canonicalised (which expands 8.3
/// names on Windows and is a no-op past symlinks elsewhere) and its `\\?\` prefix removed through
/// [`rk_git::path::WorktreePath`], so `root` is spelled the way git spells it.
pub fn temp_repo() -> tempfile::TempDir {
    static NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nonce = NONCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = tempfile::Builder::new()
        .tempdir_in(long_temp_base())
        .expect("a temp dir");
    git(dir.path(), &["init", "--quiet"]);
    git(
        dir.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(dir.path(), &["config", "user.name", "Test"]);
    // *(Added 2026-10-03, an ubuntu-latest red.)* No auto-maintenance: the root commit below could
    // start git's DETACHED maintenance, whose `worktree prune` deletes a stale `worktrees/<name>/`
    // entry — exactly what `doctor/RK-D-WORKTREE-ORPHAN/` plants into a repo built here, so
    // `ac414_…` saw no finding on the run where the prune won the race. rk-git's fixture repos
    // carry the same two lines for the same reason.
    git(dir.path(), &["config", "maintenance.auto", "false"]);
    git(dir.path(), &["config", "gc.auto", "0"]);
    std::fs::write(
        dir.path().join("README"),
        format!("root {}-{nonce}\n", std::process::id()).as_bytes(),
    )
    .expect("the root file writes");
    git(dir.path(), &["add", "README"]);
    git(dir.path(), &["commit", "--quiet", "-m", "root"]);
    dir
}

/// The system temp directory in its long, canonical spelling with no `\\?\` prefix (see
/// [`temp_repo`]'s 8.3 note). Falls back to the canonical path if `WorktreePath` refuses it (a
/// `\\?\UNC\` temp directory), and to `temp_dir()` itself if it does not canonicalise.
fn long_temp_base() -> std::path::PathBuf {
    let raw = std::env::temp_dir();
    let Ok(canonical) = std::fs::canonicalize(&raw) else {
        return raw;
    };
    rk_git::path::WorktreePath::new(&canonical)
        .map_or(canonical, |w| std::path::PathBuf::from(w.as_str()))
}

/// Run git in `cwd`, asserting it succeeded.
///
/// The tests build repositories, and their git goes
/// through `rk-git`'s one process-exec site like the product's: ADR-0077's git rule is
/// workspace-wide, tests included (`support::git_run`; D21-128 A1).
pub fn git(cwd: &Path, args: &[&str]) {
    crate::support::git_ok(cwd, args);
}

/// Spawn `redkiln -C <repo> init` with stdin closed (D-15: no verb reads stdin).
pub fn run_init(repo: &Path) -> std::process::Output {
    let mut cmd = redkiln_command(Launch::Withheld);
    cmd.args(["-C", &repo.display().to_string(), "init"]);
    crate::support::output_bounded(&mut cmd)
}

/// Every ref in the repository, sorted — the `refs/` surface AC-531's enumerator covers beside the
/// working tree and the hooks directory. Panics if git fails (D21-128 A2).
pub fn refs(repo: &Path) -> Vec<String> {
    let out = crate::support::git_ok(repo, &["for-each-ref", "--format=%(refname)"]);
    let mut names: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    names.sort();
    names
}

/// `git status --porcelain` over the whole tree, as a sorted listing. Panics if git fails
/// (D21-128 A2).
pub fn porcelain(repo: &Path) -> Vec<String> {
    let out = crate::support::git_ok(repo, &["status", "--porcelain"]);
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    lines.sort();
    lines
}

/// The repository's common dir, read from git rather than assumed — a linked worktree's is not
/// `<root>/.git`. Panics if git fails (D21-128 A2).
pub fn common_dir(repo: &Path) -> PathBuf {
    let out = crate::support::git_ok(
        repo,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    );
    PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}

/// **The run tree `run` derives for `item` in the repository at `root`** — through `rk-git`'s one
/// derivation, never a joined literal (`s8d-ii` §8d-ii.1; ADR-0180 decision 1, proposed). `root`
/// is a fixture repository's main worktree, which is what every builder here returns.
///
/// *(dated 2026-09-27, `s8d-ii` build, P6.)*
///
/// # Panics
/// If `item` is not a valid item id: every fixture item is one.
#[must_use]
pub fn run_tree(root: &Path, item: &str) -> rk_git::worktree::RunTree {
    let id = rk_git::worktree::ItemId::parse(item)
        .unwrap_or_else(|e| panic!("`{item}` is a valid item id: {e:?}"));
    rk_git::worktree::derive(root, &id)
}

/// **The fixture repository's ignore row for the run-tree root** (`s8d-ii` §8d-ii.0 item 6,
/// §8d-ii.1; D21-48 X1; ADR-0180 decision 2, proposed).
///
/// Creation refuses by name when `.redkiln/worktrees/<item>` is not git-ignored, so every builder
/// whose repository a `run` creates a tree in appends the row `init` writes, taken from
/// `rk_git::worktree::RUN_ROOT_IGNORE_ROW` rather than spelled again, to `<common-dir>/info/exclude`.
/// `temp_repo` does not call it: `init`'s own tests start from git's template exclude, and a
/// repository `run` reaches through `init` gets the row from `init`'s block.
///
/// *(dated 2026-09-27, `s8d-ii` build, P6.)*
///
/// # Panics
/// If the exclude file cannot be written.
pub fn ignore_run_root(root: &Path) {
    let info = common_dir(root).join("info");
    std::fs::create_dir_all(&info).expect("the info directory is created");
    let exclude = info.join("exclude");
    let mut bytes = std::fs::read(&exclude).unwrap_or_default();
    if !bytes.is_empty() && !bytes.ends_with(b"\n") {
        bytes.push(b'\n');
    }
    bytes.extend_from_slice(rk_git::worktree::RUN_ROOT_IGNORE_ROW.as_bytes());
    bytes.push(b'\n');
    std::fs::write(&exclude, bytes).expect("the run-root ignore row writes");
}
