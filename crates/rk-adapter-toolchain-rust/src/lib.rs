//! `rk-adapter-toolchain-rust` — the real Rust `Toolchain` row (ADR-0094 decision 1: *"Phase 1
//! ships four real adapters: … Node (`Toolchain`) and Rust (`Toolchain`) … The crate is
//! `rk-adapter-toolchain-rust`; it implements the same `Toolchain` port as
//! `rk-adapter-toolchain-node`, with `build` = `cargo build --workspace`, `test` = `cargo test
//! --workspace` (or `cargo nextest run` where present, recorded on the report), `classify` and
//! `change_summary` over `cargo metadata` and the changeset — `change_summary` remaining an
//! opaque non-finalised newtype until S-BIN"*; design §9.1; `s7c` §7c.14).
//!
//! **What this crate is.** `impl rk_ports::toolchain::Toolchain for RustToolchain` — the six
//! methods Checkpoint 7a counts and no seventh — over a **fixture project, never the workspace
//! running the checkpoint** (§7c.14: a Rust adapter over `D:/repos/redkiln-rs` would run `cargo
//! test --workspace` from inside Checkpoint 7c's own, re-entering the real row without bound on
//! the same target dir; the self-hosting run over `redkiln-rs` itself is Step 8's). Every child
//! it spawns goes through `rk_containment::run_bounded` under a **construction-time timeout**
//! ([`Config::timeout`], RT-5: ADR-0099 decision 2's *"the two Toolchain adapters run bounded
//! commands under the same timeout without a token"* — the port carries no timeout parameter and
//! no `CancelToken`); a `Command` is BUILT here and HANDED over, never spawned here (ADR-0127
//! decision 3 and its forbids; the block's `.spawn()|.output()|.status()` ban over `src/`).
//!
//! **The construction values** ([`Config::new`]): `program` — the cargo to run, default
//! [`DEFAULT_PROGRAM`] (`cargo`, resolved through the parent's `PATH`), a `PathBuf` so a test can
//! point it at an ABSOLUTE nonexistent path (AC-344's adapter share: a PATH-less child map cannot
//! induce the condition — `std` on Windows resolves a bare name against the application
//! directory, cwd, the system directories and the *parent's* `PATH` regardless of the child map,
//! measured 2026-09-17) or at AC-325's wrapper fixture; `timeout` — the bound (RT-5); the
//! **project root** as an `rk_git::path::WorktreePath` (the drive-letter form git and cargo both
//! accept, never `\\?\`; refused relative, refused absent); the **captured child-environment
//! map** (`BTreeMap` — `HashMap` is clippy-banned), refused at construction when it carries a
//! denylisted name ([`env`]: the Harness rule — AC-217, AC-488, AC-509's tracker names) or lacks
//! a name `cargo` needs on this box ([`env::REQUIRED_CHILD_ENV`] / [`env::REQUIRED_ANY_OF`],
//! MEASURED 2026-09-18 — the module's table); and a **scratch directory** the child's stdout /
//! stderr files and `CARGO_TARGET_DIR` (= `<scratch>/target`, so concurrent tests share no build
//! lock and no `target/` is written into the fixture) live under — one construction value more
//! than §7c.20's sketch names, because *"stdout to a FILE while polling"* (DECISIONS-2026-09-09
//! #31) needs a place to put the file and this crate reads no `TEMP` from the process
//! environment (the AC-217 ban).
//!
//! **What the six port methods do.** `detect`: the project root carries a `Cargo.toml` and
//! `<program> --version` starts and exits 0 — an absent program is `Ok(false)`, not an error
//! (presence is the question). `build`: `cargo build --workspace` (ADR-0094 d1). `test(selection)`:
//! [`RustToolchain::verify_report`]'s count — one spawn path, `Ok(selected_test_count)` on a green
//! suite and `Err(NonZero { code, stderr_digest })` on a red one (RT-22), `Err(Timeout {
//! after_secs })` / `Err(Transport { Io })` on a run the bound or the OS killed (AC-331: never a
//! pass, never a failing test), `Err(Transport { Spawn })` on a program that cannot start
//! (AC-344). The **selection's elements are libtest arguments** — what follows `--` (a name
//! filter, `--ignored <name>`, `--exact`, …), the way DECISIONS-2026-09-09 #20 parsed a
//! selection into real argv elements and never a string a shell re-parses. `lint`: `cargo clippy
//! --workspace --all-targets` (ADR-0102's gate, on the fixture). `classify(path)`: [`classify`],
//! per path, honest for non-Rust paths (ADR-0029 i; AC-497's classify half, PROVISIONAL while
//! S-BIN is unrun). `change_summary(range)`: `Err(Unsupported { capability: "change_summary" })`
//! **from the dated registry row** ([`registry`]; D17-9: *"the opaque newtypes stay opaque; real
//! adapters return `Unsupported` from a dated registry entry"*) — the range is never read.
//!
//! **A deviation from ADR-0094 decision 1, named (a routing, §7c.1; the Slice 6 commit body
//! carries it as a dated note OWED against that record):** the decision puts `classify` *and*
//! `change_summary` *"over `cargo metadata` and the changeset"*, and **no `cargo metadata` is read
//! anywhere in this crate** — `classify` is a rule over the path and its bytes (the same rule the
//! Node crate applies, because ADR-0029 (i) runs one `classify` per adapter over the SAME affected
//! paths of a mixed item), and `change_summary`, the method that would read the workspace and a
//! range, is S-BIN's and refuses from the registry. The `cargo metadata --format-version 1` read
//! (through `serde_json`, §7c.14: no `cargo_metadata` crate) is deferred with `change_summary` to
//! S-BIN; the pack fixture's contributed `CargoMetadata` kind names the artifact that read will
//! produce and no producer emits today.
//!
//! **The report** ([`report`]): `rk_types::verify::VerifyReport` by the INHERENT
//! [`RustToolchain::verify_report`] (RT-6), counts from libtest's stable `--list` (RT-26), argv
//! the OS vector (AC-325), the nextest fallback recorded (nextest ABSENT on this box, measured),
//! `basis: WholeSuite` always (AC-328; [`RustToolchain::select_affected`] refuses from the
//! registry row). No member is obtained by parsing prose (AC-329; the block's ban over this
//! `src/`).
//!
//! **Lint posture.** Every override is `#[expect(clippy::…, reason = "…")]` per site (ADR-0102
//! decision 3): the one `std::process::Command` site (`run.rs`'s `build_command`) and the two
//! `Instant::now` sites bounding `wall_secs` (`run.rs`). No `HashMap`, no sleep, no environment
//! read, no `unsafe`, no `anyhow`; `thiserror` for [`ConfigError`] (decision 6).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use rk_containment::Outcome;
use rk_git::path::WorktreePath;
use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};
use rk_types::digest::sha256;
use rk_types::error::{StepError, TransportKind};
use rk_types::verify::VerifyReport;

pub mod classify;
pub mod env;
pub mod registry;
pub mod report;
pub mod run;

/// The program element's default: `cargo`, resolved through the parent's `PATH` (§7c.14).
pub const DEFAULT_PROGRAM: &str = "cargo";

/// Why a [`Config`] was refused — a typed configuration error at construction, never a
/// diagnostic on a child's stderr at the first spawn.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConfigError {
    /// A name on the committed denylist is in the captured map (AC-217, AC-488, AC-509): refused,
    /// never stripped and passed through.
    #[error(
        "the captured environment carries `{name}`, which is on the committed denylist (AC-217, \
         AC-488, AC-509): construction is refused; nothing is stripped and passed through"
    )]
    DenylistedEnv { name: String },
    /// A name `cargo` needs on this box is absent from the map (`env`'s measured table).
    #[error(
        "the captured environment lacks `{name}`, which cargo/rustc/link.exe need to build on this \
         box [measured 2026-09-18, env.rs]"
    )]
    MissingChildEnv { name: &'static str },
    /// A zero bound is no bound (RT-5).
    #[error("the construction-time timeout is zero: a bounded run needs a bound (RT-5)")]
    ZeroTimeout,
    /// A relative root would resolve against the process cwd (RT-21's sibling).
    #[error(
        "the project root {path} is not an absolute path: a relative root would resolve against \
         the process cwd"
    )]
    ProjectRootNotAbsolute { path: String },
    /// No directory there.
    #[error("the project root {path} is not a directory")]
    ProjectRootNotDir { path: String },
    /// The `\\?\UNC\` device form `rk-git` refuses.
    #[error("the project root is a path rk-git refuses: {detail}")]
    ProjectRootPath { detail: String },
    /// A relative scratch directory would resolve against the process cwd too.
    #[error("the scratch directory {path} is not an absolute path")]
    ScratchNotAbsolute { path: String },
    /// The shipped registry did not load — a build defect `ac489_…` reports; an adapter whose
    /// registry does not load answers no method from it (AC-238, D17-9).
    #[error("the shipped capability registry does not load: {detail}")]
    Registry { detail: String },
}

/// The adapter's construction values (the module doc).
#[derive(Debug, Clone)]
pub struct Config {
    program: PathBuf,
    timeout: Duration,
    project_root: WorktreePath,
    child_env: BTreeMap<String, String>,
    scratch: PathBuf,
}

impl Config {
    /// Build the configuration, refusing a denylisted name, a map lacking a measured-required
    /// name, a zero bound, a relative or absent project root, a relative scratch directory.
    ///
    /// # Errors
    /// [`ConfigError`] naming what was refused.
    pub fn new(
        program: PathBuf,
        timeout: Duration,
        project_root: &Path,
        child_env: BTreeMap<String, String>,
        scratch: PathBuf,
    ) -> Result<Self, ConfigError> {
        if let Some(name) = env::first_denylisted(&child_env) {
            return Err(ConfigError::DenylistedEnv {
                name: name.to_owned(),
            });
        }
        if let Some(name) = env::first_missing_required(&child_env) {
            return Err(ConfigError::MissingChildEnv { name });
        }
        if timeout.is_zero() {
            return Err(ConfigError::ZeroTimeout);
        }
        if !project_root.is_absolute() {
            return Err(ConfigError::ProjectRootNotAbsolute {
                path: project_root.display().to_string(),
            });
        }
        if !project_root.is_dir() {
            return Err(ConfigError::ProjectRootNotDir {
                path: project_root.display().to_string(),
            });
        }
        if !scratch.is_absolute() {
            return Err(ConfigError::ScratchNotAbsolute {
                path: scratch.display().to_string(),
            });
        }
        let project_root =
            WorktreePath::new(project_root).map_err(|e| ConfigError::ProjectRootPath {
                detail: format!("{e:?}"),
            })?;
        Ok(Self {
            program,
            timeout,
            project_root,
            child_env,
            scratch,
        })
    }

    /// The program the adapter runs — `cargo`, or what the caller pointed it at.
    #[must_use]
    pub fn program(&self) -> &Path {
        &self.program
    }

    /// The construction-time bound (RT-5).
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    /// The project root, in the form git and cargo accept.
    #[must_use]
    pub const fn project_root(&self) -> &WorktreePath {
        &self.project_root
    }

    /// The captured map every child is spawned with (plus `CARGO_TARGET_DIR`).
    #[must_use]
    pub const fn child_env(&self) -> &BTreeMap<String, String> {
        &self.child_env
    }

    /// Where the output files and the target directory live.
    #[must_use]
    pub fn scratch(&self) -> &Path {
        &self.scratch
    }

    /// `CARGO_TARGET_DIR` for every spawn: `<scratch>/target`.
    #[must_use]
    pub fn target_dir(&self) -> PathBuf {
        self.scratch.join("target")
    }
}

/// The real Rust toolchain: a [`Config`] and the registry rows it answers `Unsupported` from.
#[derive(Debug)]
pub struct RustToolchain {
    config: Config,
    registry: Vec<registry::Row>,
    /// The next run directory's number: every port-method call spawns into its own
    /// `<scratch>/run-<n>` ([`RustToolchain::run_dir`]), so two concurrent calls on this `Send + Sync`
    /// adapter share no output file.
    runs: AtomicU64,
}

impl RustToolchain {
    /// Construct over the shipped registry.
    ///
    /// # Errors
    /// [`ConfigError::Registry`] if the shipped registry does not load.
    pub fn new(config: Config) -> Result<Self, ConfigError> {
        let registry = registry::rows().map_err(|e| ConfigError::Registry {
            detail: e.to_string(),
        })?;
        Ok(Self {
            config,
            registry,
            runs: AtomicU64::new(0),
        })
    }

    /// Test support: construct over controlled registry rows, so `ac489_…` / `ac328_…` can watch
    /// the adapter answer from an empty registry or from a row with an invented probe.
    #[doc(hidden)]
    #[must_use]
    pub const fn new_over_registry(config: Config, registry: Vec<registry::Row>) -> Self {
        Self {
            config,
            registry,
            runs: AtomicU64::new(0),
        }
    }

    /// A fresh run directory for one port-method call: `<scratch>/run-<n>`, `n` from a counter
    /// this adapter owns, created here. Every child the call spawns writes its stdout / stderr
    /// (and, on the Node side, the reporter file) under it, so concurrent calls share no file.
    ///
    /// # Errors
    /// `Transport { Io }` if the directory cannot be created.
    pub fn run_dir(&self) -> Result<PathBuf, StepError> {
        let n = self.runs.fetch_add(1, Ordering::Relaxed);
        let dir = self.config.scratch().join(format!("run-{n}"));
        std::fs::create_dir_all(&dir).map_err(|e| StepError::Transport {
            kind: TransportKind::Io,
            detail: format!("could not create the run directory {}: {e}", dir.display()),
        })?;
        Ok(dir)
    }

    /// The construction values.
    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// The rows this adapter answers from.
    #[must_use]
    pub fn registry(&self) -> &[registry::Row] {
        &self.registry
    }

    /// The report (RT-6: an inherent method; the port stays six methods) — [`report`].
    ///
    /// # Errors
    /// As [`report::verify_report`].
    pub fn verify_report(&self, selection: &[String]) -> Result<VerifyReport, StepError> {
        report::verify_report(self, selection)
    }

    /// Affected selection — what the kernel would resolve a verify activity through (AC-328):
    /// refused with `Unsupported { capability: "affected-selection" }` **from the registry row**
    /// while no probe is recorded; a row with an invented probe, or no row, is a decode failure
    /// naming it (never a selection basis). The changed paths are never read.
    ///
    /// # Errors
    /// Always — [`registry::refusal`] over this adapter's rows.
    pub fn select_affected(&self, _changed_paths: &[&str]) -> Result<Vec<String>, StepError> {
        Err(registry::refusal(
            &self.registry,
            registry::Capability::AffectedSelection,
        ))
    }

    /// Run `cargo <args>` bounded and map its outcome: `Ok(())` on exit 0, `NonZero` otherwise,
    /// `Timeout` / `Transport { Io }` on a killed run, `Transport { Spawn }` on no program.
    fn cargo(&self, tag: &str, args: &[&str]) -> Result<(), StepError> {
        let args: Vec<String> = args.iter().map(|s| (*s).to_owned()).collect();
        let dir = self.run_dir()?;
        let run = run::run(&self.config, &dir, tag, &args)?;
        match run.outcome {
            Outcome::Exited(status) if status.success() => Ok(()),
            Outcome::Exited(status) => Err(StepError::NonZero {
                code: status.code(),
                stderr_digest: sha256(&run.stderr_bytes()?),
            }),
            Outcome::TimedOut => Err(StepError::Timeout {
                after_secs: self.config.timeout().as_secs(),
            }),
            Outcome::OsKilled => Err(report::os_killed(&run.argv)),
        }
    }
}

impl Toolchain for RustToolchain {
    /// Present when the root carries a `Cargo.toml` and `<program> --version` starts and exits 0;
    /// an absent program is `Ok(false)`.
    fn detect(&self) -> Result<bool, StepError> {
        let manifest = Path::new(self.config.project_root().as_str()).join("Cargo.toml");
        if !manifest.is_file() {
            return Ok(false);
        }
        match self.cargo("detect", &["--version"]) {
            Ok(()) => Ok(true),
            Err(
                StepError::Transport {
                    kind: TransportKind::Spawn,
                    ..
                }
                | StepError::NonZero { .. },
            ) => Ok(false),
            Err(other) => Err(other),
        }
    }

    /// `cargo build --workspace` (ADR-0094 decision 1).
    fn build(&self) -> Result<(), StepError> {
        self.cargo("build", &["build", "--workspace"])
    }

    /// The report's count (RT-22: `Ok(count)` only on a green suite).
    fn test(&self, selection: &[String]) -> Result<u32, StepError> {
        self.verify_report(selection).map(|r| r.selected_test_count)
    }

    /// `cargo clippy --workspace --all-targets`.
    fn lint(&self) -> Result<(), StepError> {
        self.cargo("lint", &["clippy", "--workspace", "--all-targets"])
    }

    /// Per path (ADR-0029 i; AC-497's classify half).
    fn classify(&self, path: &str) -> Result<Classification, StepError> {
        Ok(classify::classify(
            Path::new(self.config.project_root().as_str()),
            path,
        ))
    }

    /// `Unsupported` from the dated registry row (D17-9); the range is never read.
    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {
        Err(registry::refusal(
            &self.registry,
            registry::Capability::ChangeSummary,
        ))
    }
}
