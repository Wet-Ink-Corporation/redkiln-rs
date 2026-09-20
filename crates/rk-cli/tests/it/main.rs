//! `rk-cli`'s **one** integration binary — a `mod` per case, never one file per case (ADR-0083).
//! This is also what makes `--test it` addressable in Checkpoint 7d.
//!
//! What is here is **Slice 1** (`s7d` §7d.16 item 1): [`surface`], the `clap` router — its committed
//! `--help` snapshot, the resume-shaped denylist walked over the built parser tree, the two grammars
//! ADR-0128 and ADR-0129 fix, and the one source ban AC-403's lint half names; and [`exit`], the
//! closed exit table — its `insta` snapshot, the distinctness of its eighteen codes, and the
//! exhaustiveness of the eleven-arm `From<&StepError>` map.
//!
//! **No empty module stands in for an unbuilt one** (`rk-kernel`'s `tests/it/main.rs` states the rule
//! and this file inherits it): there is no `config`, `init`, `signal`, `run`, `answer`, `doctor` or
//! `migrate` module here, because Slices 2–5 build what they would assert. A `mod` that exists and
//! asserts nothing reads as coverage.
//!
//! **The two source-scan primitives live HERE, not in `surface.rs`.** [`rs_files`] and [`code_lines`]
//! are used by `exit::ac438_step_error_mapping_is_exhaustive` and by
//! `surface::ac403_no_blocking_stdin_read_in_rk_cli`, which sit in different modules; a helper in
//! either one is unreachable from the other. `tests/it/support.rs` — the scripted-harness helpers —
//! is Slice 3's (§7d.16 item 3) and is not created early to hold two functions.

mod config;
mod exit;
mod init;
mod root;
mod surface;

use std::path::{Path, PathBuf};

/// The crate's own `src/` directory, resolved from the manifest rather than from the process cwd.
///
/// `env::current_dir` is banned in `crates/rk-cli/src` and the ban's scope is the source tree, not
/// the test tree — but a test that read the cwd would still be pinning the harness's choice of
/// working directory rather than the crate, and `cargo test` does not promise one.
pub fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
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
