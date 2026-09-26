//! A stub `cargo` for the review fixture — the CANDIDATE arm fails, the REFERENCE arm is green.
//!
//! **Why a stub and not real cargo.** ADR-0162 decision 4's re-executor runs the repro's selection
//! at two trees and reads AC-288's four cells off `Toolchain::test`, whose contract is *"`Ok(count)`
//! only on a green suite"* (RT-22). Making a real suite red at one tree and green at the other
//! means compiling a real crate twice inside a temp repository, which is minutes of wall clock in a
//! test whose subject is the WIRING. `toolchain_programs` is the tree's own sanctioned seam for
//! exactly this — `crates/rk-cli/src/config.rs` calls it *"the one way a process-level test reaches
//! the fixture child"*.
//!
//! 🛑 **THIS FIXTURE IS RIGGED, AND SAYING SO IS THE POINT.** It decides by its WORKING DIRECTORY:
//! `rk_adapter_toolchain_rust::run::build_command` spawns every child with
//! `.current_dir(config.project_root())`, so the candidate arm runs in the run-owned worktree
//! (`…/worktrees/<item>`) and the reference arm in the ephemeral one (`…/worktrees/<item>.review-ref`,
//! ADR-0162 decision 10). Failing unless the directory is the reference tree is a stand-in for
//! *"the change under review breaks the suite"* — the ONE data condition that makes
//! `four_cell(true, false)` reachable. It is not evidence that any real change breaks any real
//! suite, and no test may read it as such.
//!
//! **What it must satisfy, because `verify_report` spawns it four times:**
//!
//! | vector | what this answers |
//! | --- | --- |
//! | `--version` | exit 0, so `RustToolchain::detect` chooses this adapter |
//! | `test --workspace -- <sel> --list` | one `: test` line, which `count_listed` counts |
//! | `test --workspace -- <sel> --list --ignored` | no such line, so the subtraction is 0 |
//! | `test --workspace -- <sel>` | the verdict: non-zero in the candidate tree, 0 in the reference |
//!
//! `rustc -V` is spawned separately by `toolchain_version` and is the REAL `rustc` — this program
//! is not asked for it, because the adapter looks for a `rustc` sibling of the configured program
//! and falls back to the one on `PATH`.
//!
//! *(Amended 2026-09-26, s8c §8c.2; record T = ADR-0169, proposed.)* `Toolchain::test` no longer
//! reads the exit status alone: RT-22 is reversed, a red suite is `Ok` with its counts, and the
//! counts come from the pinned libtest-line parser (§8c.1), which refuses anything outside its
//! grammar as `Transport { Decode }` -- and the re-executor reads either arm's `Err` as
//! `Unverifiable { cannot_run }`. So the run vector must print ONE WHOLE §8c.1 BLOCK for its one
//! test: the `running 1 test` header, the record, the empty line that closes the record region,
//! and the full-grammar summary with its `finished in` tail. `FAILED` in the candidate tree and
//! `ok` in `…review-ref`; its exit code agrees (101 red, 0 green). If it did not, the candidate
//! would read as cannot-run and AC-438's `exit/findings-block/` row (code 4) would silently
//! become exit 0, because this fixture is the ONLY data condition that reaches
//! `four_cell(true, false)`. The version is the REAL `rustc -V`, so the parser's `PINNED` holds
//! it. Two vectors change: the run vector now carries `--no-fail-fast` (record T decision 6),
//! which this program ignores; and the runner probe `nextest --version`, which the Rust adapter
//! spawns at construction under `Detect` (record T decision 4), is answered as cargo answers an
//! unknown subcommand -- exit 101, *"error: no such command: `nextest`"* on stderr, as measured
//! at stage open -- so the adapter runs the `cargo test` vectors this table describes.

use std::path::Path;

/// The suffix [`rk_cli`]'s review pass gives its ephemeral reference worktree.
///
/// Kept as a literal rather than imported: a fixture that shared the constant with the code under
/// test would agree with it by construction, and this one is meant to be able to disagree.
const REFERENCE_SUFFIX: &str = ".review-ref";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // The runner probe (record T decision 4): answered as cargo answers an unknown subcommand.
    if args.first().is_some_and(|a| a == "nextest") {
        eprintln!("error: no such command: `nextest`");
        std::process::exit(101);
    }

    // `detect()`: a version probe. Answered by every tree, so both arms choose this adapter.
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("cargo 0.0.0-rk-fixture (review wiring stub)");
        return;
    }

    // The two listing vectors. `count_listed` counts lines ending in `": test"`; the `--ignored`
    // listing is SUBTRACTED, so it must list nothing or the selected count goes negative-by-
    // saturation to zero and the report says the selection was empty.
    if args.iter().any(|a| a == "--list") {
        if !args.iter().any(|a| a == "--ignored") {
            println!("rk_fixture::the_reviewed_case: test");
        }
        return;
    }

    // The run itself, and the whole point of the fixture.
    let cwd = std::env::current_dir().unwrap_or_else(|e| {
        eprintln!("rk-fixture-toolchain: no working directory: {e}");
        std::process::exit(2);
    });
    // ONE WHOLE §8c.1 BLOCK per arm (the module doc): header, record, the empty line that closes
    // the record region, and the full-grammar summary.
    if is_reference(&cwd) {
        println!("\nrunning 1 test\ntest rk_fixture::the_reviewed_case ... ok\n");
        println!(
            "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
        );
        return;
    }
    println!("\nrunning 1 test\ntest rk_fixture::the_reviewed_case ... FAILED\n");
    println!("failures:\n\nfailures:\n    rk_fixture::the_reviewed_case\n");
    println!(
        "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
    );
    eprintln!(
        "rk-fixture-toolchain: the reviewed change breaks the suite in {} (the fixture's one data \
         condition; the reference tree is green)",
        cwd.display()
    );
    std::process::exit(101);
}

/// Is this the reference tree?
///
/// Compared on the final path component, not with `contains`: the candidate worktree's path is a
/// PREFIX of nothing, but a substring rule would also match a repository whose own directory
/// happened to carry the suffix, and a fixture that can be satisfied by an accident of where the
/// temp directory landed reports a green it did not earn.
fn is_reference(cwd: &Path) -> bool {
    cwd.file_name()
        .and_then(|n| n.to_str())
        .is_some_and(|n| n.ends_with(REFERENCE_SUFFIX))
}
