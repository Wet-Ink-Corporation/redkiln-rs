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
use std::sync::Arc;
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

// ================================================================================================
// s8c §8c.5 / §8c.9: the chosen adapter's value table reaches the kernel WITHOUT its name.
// ================================================================================================

/// A `Toolchain` that records the bound each step-reached call is handed, and refuses the rest.
#[derive(Default)]
struct BoundRecorder {
    bounds: std::sync::Mutex<Vec<rk_ports::toolchain::StepBound>>,
}

impl BoundRecorder {
    fn record(&self, bound: rk_ports::toolchain::StepBound) {
        self.bounds.lock().expect("unpoisoned").push(bound);
    }
    fn refuse(what: &str) -> rk_types::error::StepError {
        rk_types::error::StepError::Unsupported {
            capability: what.to_owned(),
        }
    }
}

impl rk_ports::toolchain::Toolchain for BoundRecorder {
    fn detect(&self) -> Result<bool, rk_types::error::StepError> {
        Ok(true)
    }
    fn build(
        &self,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_types::verb_output::BuildReport, rk_types::error::StepError> {
        self.record(bound);
        Ok(rk_types::verb_output::BuildReport {
            argv: vec!["recorder".to_owned()],
            toolchain_version: "recorder".to_owned(),
            wall_secs: 0,
            errors: 0,
            status: rk_types::verb_output::BuildStatus::Passing,
        })
    }
    fn test(
        &self,
        _selection: &rk_ports::toolchain::Selection,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_ports::toolchain::TestOutcome, rk_types::error::StepError> {
        self.record(bound);
        Err(Self::refuse("test"))
    }
    fn lint(
        &self,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<(), rk_types::error::StepError> {
        self.record(bound);
        Err(Self::refuse("lint"))
    }
    fn classify(
        &self,
        _path: &str,
    ) -> Result<rk_ports::toolchain::Classification, rk_types::error::StepError> {
        Err(Self::refuse("classify"))
    }
    fn change_summary(
        &self,
        _range: &str,
        bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {
        self.record(bound);
        Err(Self::refuse("change_summary"))
    }
    fn run_verb(
        &self,
        call: &rk_ports::toolchain::VerbCall<'_>,
    ) -> Result<rk_ports::toolchain::VerbOutput, rk_types::error::StepError> {
        self.record(call.bound);
        Err(Self::refuse(call.verb))
    }
}

/// **§8c.5 (AC-487): `rk-cli`, which holds the name `real_toolchain`'s loop chose, selects that
/// adapter's value table and hands the kernel a flat `ToolchainValues`; the kernel never sees
/// `rust`.** The two tables differ in the one key the step reads, so the wrong table is a wrong
/// NUMBER at the port, and the value handed over is read for any adapter name.
#[test]
fn the_chosen_adapters_value_table_reaches_the_kernel_without_its_name() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    std::fs::write(
        repo.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
    )
    .expect("the fixture manifest is writable");
    let pin = "0123456789abcdef0123456789abcdef01234567";
    // rust: 60 s slowest → 120 s by ADR-0170 decision 10's rule; node: 300 s → 600 s.
    let registry = format!(
        "[verb]\nbuild = {{ outputs = \"BuildReport\", live = false, timeout = \"build_timeout\", \
         inputs = [], route = \"port\" }}\n\
         [toolchain.node]\nbuild_timeout = {{ secs = 600, measured = [300], rule = \"r\", pin = \
         \"{pin}\", command = \"tsc\" }}\n\
         [toolchain.rust]\nbuild_timeout = {{ secs = 120, measured = [60], rule = \"r\", pin = \
         \"{pin}\", command = \"cargo build\" }}\n"
    );
    std::fs::write(repo.path().join(".redkiln").join("registry.toml"), registry)
        .expect("the registry is writable");

    let resolved = resolved_over_pack_min(repo.path(), "bug-1");
    let mut reports = Vec::new();
    let chosen = rk_cli::verbs::run::choose_toolchain(
        &rk_cli::verbs::run::ToolchainCandidates::of(&resolved),
        &Env::from_map(env_that_refuses_node_and_satisfies_rust()),
        repo.path(),
        &mut |line| reports.push(line.to_owned()),
    )
    .expect("selection runs");
    // PREMISE: rust was CHOSEN (node, listed first, is refused by this environment).
    assert_eq!(
        chosen.adapter.as_deref(),
        Some("rk-adapter-toolchain-rust"),
        "PREMISE: the loop did not choose rust: {reports:?}"
    );

    let routing = rk_cli::verbs::run::tool_routing(repo.path(), chosen.adapter.as_deref())
        .expect("the registry loads");
    assert_eq!(
        routing
            .values()
            .resolve("build_timeout")
            .map(std::num::NonZeroU64::get),
        Ok(120),
        "the value table handed over is not the CHOSEN adapter's (rust: 120 s; node: 600 s)"
    );
    let rendered = format!("{:?}", routing.values());
    for name in ["rust", "node", "rk-adapter"] {
        assert!(
            !rendered.contains(name),
            "the value table the kernel is handed names `{name}` (AC-487): {rendered}"
        );
    }

    // AND THE KERNEL RESOLVES IT: a `${toolchain.build_timeout}` step hands the port 120 s.
    let recorder = Arc::new(BoundRecorder::default());
    let store: Arc<std::sync::Mutex<dyn rk_ports::event_store::EventStore>> =
        Arc::new(std::sync::Mutex::new(rk_ports::fakes::VecStore::default()));
    let rt = rk_kernel::runtime::Runtime::new(
        rk_ports::root::Ports {
            store,
            harness: Arc::new(rk_ports::fakes::FakeHarness::scripted(Vec::new())),
            tracker: Arc::new(rk_ports::fakes::BoardTracker::default()),
            docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
            toolchain: Arc::clone(&recorder) as Arc<dyn rk_ports::toolchain::Toolchain>,
        },
        "run-value-table",
        "implement",
    )
    .with_tool_routing(routing);
    let step = rk_kernel::catalog::Step {
        id: "build".to_owned(),
        kind: rk_kernel::catalog::StepKind::Tool,
        run: Some("toolchain:build".to_owned()),
        output_type: None,
        role: None,
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
        timeout_secs: Some(rk_kernel::catalog::StepTimeout::Toolchain(
            "build_timeout".to_owned(),
        )),
    };
    let verdict = rt
        .execute_step(&step, &rk_kernel::predicate::Context::new())
        .expect("a value");
    assert_eq!(verdict, rk_kernel::runtime::step::StepVerdict::Continue);
    assert_eq!(
        *recorder.bounds.lock().expect("unpoisoned"),
        vec![rk_ports::toolchain::StepBound::secs(120).expect("non-zero")],
        "the kernel did not hand the port the chosen table's value"
    );
}
