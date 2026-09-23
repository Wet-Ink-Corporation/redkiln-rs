#![cfg(windows)]
//! 🛑 **WINDOWS ONLY, AND THE GATE IS LOAD-BEARING RATHER THAN INCIDENTAL.** This module's
//! premise is that the two adapters' required-env tables DIFFER, and both tables are
//! `cfg`-split: on `not(windows)` each is `&[]`, so the fixture environment refuses NEITHER
//! adapter and the first CONTROL below (`node_cfg.is_err()`) fires — red for an environmental
//! reason rather than a defect. `.github/workflows/ci.yml` runs `cargo test --workspace` on
//! `ubuntu-latest` as well as `windows-latest`, so without this gate ADR-0149's guard would
//! break the very CI that ADR-0151 just made able to run this suite at all. `#[cfg(windows)]`
//! precedent is in `support.rs` in this same directory.
//! **`detect()` chooses; a candidate that cannot be CONFIGURED does not end the run** — ADR-0149,
//! `PROGRESS.md` row 37's finding (7).
//!
//! `real_toolchain` walks `[adapters] toolchains` in the file's declared order and asks each
//! candidate's `detect()`. It used to CONSTRUCT and VALIDATE every candidate with `?` **before**
//! reaching that question, so the first candidate whose `Config::new` refused ended the whole run
//! with exit **3** — even when a later candidate would have detected the project.
//!
//! 🛑 **THIS IS THE DEFAULT ORDERING, NOT AN EDGE CASE.** Every shipped and fixture
//! `.redkiln/config.toml` in this repository declares
//! `toolchains = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"]` — **node first** — and
//! the two adapters' required-env tables DIFFER. Measured 2026-09-22 on Windows:
//!
//! | adapter | `REQUIRED_CHILD_ENV` | `REQUIRED_ANY_OF` |
//! | --- | --- | --- |
//! | node | `SystemRoot`, `PATH` | — |
//! | rust | `SystemDrive` | `TMP`, `TEMP`, `USERPROFILE` |
//!
//! So a child environment carrying `SystemDrive` and `TMP` but no `SystemRoot` refuses **node's**
//! configuration and satisfies **rust's**. That is the environment this test builds, and before
//! ADR-0149 it made `redkiln run` exit 3 on a Rust repository with a refusal naming Node.
//!
//! ⚠️ **The assertion is paired with a control that the fixture ENVIRONMENT really does refuse
//! node**, because "the loop returned `Ok`" is also what a loop that never refused anything returns.
//! Without that control this test would pass on an environment that satisfies both adapters, which
//! is most of them.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use rk_cli::env::Env;

use crate::support::resolved_over_pack_min;

/// `SystemDrive` + `TMP`, and deliberately NO `SystemRoot` and no `PATH`.
fn env_that_refuses_node_and_satisfies_rust() -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    map.insert("SystemDrive".to_owned(), "C:".to_owned());
    map.insert("TMP".to_owned(), std::env::temp_dir().display().to_string());
    map
}

/// **ADR-0149: the first candidate's refusal is recorded and the loop goes on.**
#[test]
fn a_candidate_whose_config_is_refused_does_not_end_the_run_before_detect_chooses() {
    let child_env = env_that_refuses_node_and_satisfies_rust();

    // ---- THE CONTROL FIRST: this environment REALLY DOES refuse node's configuration. ----------
    //
    // Asserted against the adapter's own `Config::new`, not against a copy of its rules, so the
    // control cannot drift from what `real_toolchain` will call. If node ever stops requiring
    // `SystemRoot`, this fires and says so rather than letting the assertion below go vacuous.
    let node_cfg = rk_adapter_toolchain_node::Config::new(
        PathBuf::from(rk_adapter_toolchain_node::DEFAULT_PROGRAM),
        Duration::from_secs(60),
        &std::env::temp_dir(),
        child_env.clone(),
        std::env::temp_dir(),
    );
    assert!(
        node_cfg.is_err(),
        "CONTROL: this child environment does NOT refuse `rk-adapter-toolchain-node`'s \
         configuration, so the loop below is never asked the question this test is about. The \
         adapter's required-env table has moved: rebuild the fixture environment from it."
    );

    // ---- AND rust's configuration is satisfied by the same environment. ------------------------
    let rust_cfg = rk_adapter_toolchain_rust::Config::new(
        PathBuf::from(rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
        Duration::from_secs(60),
        &std::env::temp_dir(),
        child_env.clone(),
        std::env::temp_dir(),
    );
    assert!(
        rust_cfg.is_ok(),
        "CONTROL: the same environment also refuses `rk-adapter-toolchain-rust`, so nothing is \
         left for the loop to fall through TO and a pass below would mean only that everything \
         failed: {:?}",
        rust_cfg.err()
    );

    // ---- THE ASSERTION. ------------------------------------------------------------------------
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);

    // 🛑 **THE FIXTURE MUST BE A TREE RUST ACTUALLY DETECTS, AND THE FIRST DRAFT WAS NOT.** An
    // adversarial pass measured it: `temp_repo()` writes only a `README`, and
    // `RustToolchain::detect` opens with `if !root.join("Cargo.toml").is_file() { return Ok(false) }`
    // — so rust did not detect either, the loop fell off the end, and the value the test asserted
    // `is_ok()` on was `Ok(UndetectedToolchain)`. That is the FALLBACK, not the selection, and after
    // ADR-0149 `Ok` is what this function returns on nearly every path, so `is_ok()` alone was close
    // to a tautology. A `Cargo.toml` makes the second candidate genuinely selectable, and the
    // `detect()` assertion below is what distinguishes "rust was chosen" from "nothing was".
    std::fs::write(
        repo.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("the fixture manifest is writable");

    let resolved = resolved_over_pack_min(repo.path(), "bug-1");
    assert_eq!(
        resolved.toolchains.first().map(String::as_str),
        Some("rk-adapter-toolchain-node"),
        "the fixture no longer declares node FIRST, so the refusing candidate is not the one the \
         loop reaches first and this test asserts nothing: {:?}",
        resolved.toolchains
    );

    let selected =
        rk_cli::verbs::run::real_toolchain(&resolved, &Env::from_map(child_env), repo.path());
    let toolchain = selected.unwrap_or_else(|e| {
        panic!(
            "the first candidate's refused configuration ended the whole run. ADR-0149: `detect()` \
             chooses, and a candidate that cannot be built is recorded in `tried` and walked past — \
             otherwise node's required-env table gates every run on a tree where rust would be \
             selected. Got: {e:?}"
        )
    });

    // 🛑 **AND THE SECOND CANDIDATE WAS ACTUALLY SELECTED, not merely not-refused.** `Ok` is what
    // this function returns on nearly every path since ADR-0149 — including the
    // `UndetectedToolchain` fallback, whose own `detect()` does NOT report a project. Without this,
    // the assertion above passes for a tree on which nothing was chosen, which is what an
    // adversarial pass measured the first draft doing.
    assert_eq!(
        toolchain.detect().ok(),
        Some(true),
        "`real_toolchain` returned `Ok`, but the toolchain it returned does not detect this \
         project — so the loop fell through to ADR-0139's `UndetectedToolchain` rather than \
         selecting rust, and this test would pass on a tree where node's refusal HAD ended the run \
         for every candidate"
    );
}
