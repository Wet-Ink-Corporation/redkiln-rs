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
mod catalog;
mod config;
mod doctor;
mod exit;
mod explain;
mod init;
mod lint;
mod reconcile;
mod review_pass;
mod root;
mod run;
mod story_axis;
mod support;
mod surface;
mod toolchain;
mod transcription;

use std::path::{Path, PathBuf};
use std::process::Stdio;

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
pub fn temp_repo() -> tempfile::TempDir {
    static NONCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let nonce = NONCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = tempfile::tempdir().expect("a temp dir");
    git(dir.path(), &["init", "--quiet"]);
    git(
        dir.path(),
        &["config", "user.email", "test@example.invalid"],
    );
    git(dir.path(), &["config", "user.name", "Test"]);
    std::fs::write(
        dir.path().join("README"),
        format!("root {}-{nonce}\n", std::process::id()).as_bytes(),
    )
    .expect("the root file writes");
    git(dir.path(), &["add", "README"]);
    git(dir.path(), &["commit", "--quiet", "-m", "root"]);
    dir
}

/// Run git in `cwd`. The tests build repositories; the PRODUCT never spawns git outside `rk-git`'s
/// one wrapper, which is what Checkpoint 7d's `Command::new("git")` ban over `src/` enforces.
#[expect(
    clippy::disallowed_types,
    reason = "a test building its own fixture repository; the product's git goes through rk-git's \
              one GitExec wrapper (ADR-0077), which the block's src/ ban enforces"
)]
pub fn git(cwd: &Path, args: &[&str]) {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .output()
        .expect("git runs");
    assert!(
        out.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Spawn `redkiln -C <repo> init` with stdin closed (D-15: no verb reads stdin).
#[expect(
    clippy::disallowed_types,
    reason = "the verb is exercised AS A PROCESS, which is the only way golden constraint 4's \
              'a stub never replaced' is caught (s7d §7d.13's temp-repository rule)"
)]
pub fn run_init(repo: &Path) -> std::process::Output {
    let mut cmd = std::process::Command::new(redkiln());
    cmd.args(["-C", &repo.display().to_string(), "init"]);
    crate::support::output_bounded(&mut cmd)
}

/// Every ref in the repository, sorted — the `refs/` surface AC-531's enumerator covers beside the
/// working tree and the hooks directory.
#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository's own refs; see `git` above"
)]
pub fn refs(repo: &Path) -> Vec<String> {
    let out = std::process::Command::new("git")
        .args(["for-each-ref", "--format=%(refname)"])
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()
        .expect("git for-each-ref runs");
    let mut names: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    names.sort();
    names
}

/// `git status --porcelain` over the whole tree, as a sorted listing.
#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository's own porcelain listing; see `git` above"
)]
pub fn porcelain(repo: &Path) -> Vec<String> {
    let out = std::process::Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()
        .expect("git status runs");
    let mut lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    lines.sort();
    lines
}

/// The repository's common dir, read from git rather than assumed — a linked worktree's is not
/// `<root>/.git`.
#[expect(
    clippy::disallowed_types,
    reason = "reading the fixture repository's own common dir; see `git` above"
)]
pub fn common_dir(repo: &Path) -> PathBuf {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .current_dir(repo)
        .stdin(Stdio::null())
        .output()
        .expect("git rev-parse runs");
    PathBuf::from(String::from_utf8_lossy(&out.stdout).trim().to_owned())
}
