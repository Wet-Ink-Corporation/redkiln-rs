//! **The nine verbs** — one per subcommand the router declares: §7d.3's eight, and [`cost`].
//!
//! [`cost`] is the ninth, which ADR-0163 decision 1 added at Step 7i (2026-09-23). As of Step 7d
//! Slice 4 cycle 2 (2026-09-20) **none of the eight is a stub**: Slice 2 built `init`, Slice 3 built
//! `run`, `answer`, `status`, `catalog`, `lint` and `explain`, and [`doctor`] is this slice's.
//!
//! **Corrected 2026-09-20.** The two sentences that stood here said *"`doctor` is the last stub"*
//! and, at the foot, *"`doctor` still exits `3` and says so; it is Slice 4's"*. Slice 4 is this
//! slice. The stub they described was an **inline `fn doctor()` in this file** calling a shared
//! `not_built(verb)` helper — not a `verbs/doctor.rs`, which did not exist until this cycle created
//! it [measured 2026-09-20]. Both it and `not_built` are gone: `not_built` had exactly one caller
//! left, so a helper for *"the one refusal every stub prints"* outlived the stubs it was written
//! for.
//!
//! **The paragraph below is kept, and it is no longer about a stub.** Its rule — a refusal is a
//! value through the exit table and never an `unimplemented!()` — is what `verbs/doctor.rs`'s
//! `--rebuild` arm obeys today: that path renames a new store over the old, and until cycle 9 fills
//! `doctor::rebuild` it would rename an EMPTY one, so it refuses with a reason and a code rather
//! than running or panicking.
//!
//! **Why a refusal rather than an unimplemented subcommand.** The surface is what Slice 1 delivers:
//! a committed `--help` snapshot and a denylist lint that bites from the first commit. Both need
//! the whole tree to exist. What they do not need is behaviour, and behaviour arrives one lane at a
//! time — the loop at Slice 3, `doctor` at Slice 4.
//!
//! **Why each refusal is a configuration error and not a panic.** ADR-0118 clause 6 makes every verb
//! return `Result<Exit, Halt>` so an in-process caller can observe a value instead of a dead test
//! process, and §7d.4 allows `std::process::exit` exactly two sites, both in `main.rs`. A stub that
//! reached for `unimplemented!()` would exit `101` through a path the exit table does not describe.
//!
//! **This file is where golden constraint 4 bites**: *a stub never replaced is caught only here* —
//! by the block's verb-level lines at the stage close, which drive the real binary and expect a
//! real answer. Every arm below now reaches one.

pub mod answer;
pub mod catalog;
pub mod cost;
pub mod doctor;
pub mod explain;
pub mod init;
pub mod lint;
pub mod run;
pub mod status;

use std::path::Path;

use rk_git::error::GitError;
use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::reconciled::{Entry, Outcome};

use crate::adapters::{AdapterSet, Refusal, Resolved};
use crate::cli::{Cli, Command, DoctorArgs};
use crate::config::{self, ConfigError, ConfigFile};
use crate::env::Env;
use crate::exit::{Exit, Halt};

// ---------------------------------------------------------------- the three startup seams
//
// **Why these are traits and not free calls.** §7d.5 names three things a test must be able to
// observe that no free function can expose: AC-402's *"exactly one reconcile call with the right
// `Entry` **before** the first store read on each verb"*, and AC-485's *"the `ConfigFile` reader is
// called exactly once and `AdapterSet::resolve` exactly once (**counting seams the verb function
// takes**)"*. `config::parse` is a free function over `&str` and `AdapterSet::resolve` an inherent
// associated fn, so neither is injectable as it stands.
//
// **Why they live in `verbs/mod.rs` and not in a module of their own.** §7d.16 item 3's `rk-cli`
// `src/` list is exactly the six verb files, and item 4 shows that new non-verb modules ARE
// enumerated when a slice adds one (`src/no_dispatch.rs`, `src/doctor/roles.rs`). So a new module
// here would be one item 3 does not name; this file already exists and is the one file every verb
// that takes these shares.
//
// **Why `reconcile` passes `exec` and `repo` THROUGH rather than capturing them.** ADR-0109
// decision 1 gives `Reconciled<T>` a private field with no public constructor, no `From`, no
// `Default` and no `DerefMut` — AC-172's compile-failure instrument. A test-side implementor
// therefore **cannot fabricate an `Outcome`**; the only way to obtain one is to call
// `reconcile_at_entry`. Any trait shape that made the recorder *produce* a `Reconciled<State>` from
// nothing would be unimplementable, so the recorder records and then delegates with the verb's own
// wrapper and worktree.

/// The reconcile-at-entry seam (AC-402). One real implementor; the tests' records the call.
pub trait EntryReconciler {
    /// Reconcile at this entry point.
    ///
    /// # Errors
    /// Propagates [`GitError`] from `rk_git::reconciled::reconcile_at_entry`.
    fn reconcile(
        &self,
        exec: &GitExec,
        repo: &WorktreePath,
        entry: Entry,
    ) -> Result<Outcome, GitError>;
}

/// The one implementor on the product path — it delegates and does nothing else.
#[derive(Debug, Clone, Copy)]
pub struct RealReconciler;

impl EntryReconciler for RealReconciler {
    fn reconcile(
        &self,
        exec: &GitExec,
        repo: &WorktreePath,
        entry: Entry,
    ) -> Result<Outcome, GitError> {
        rk_git::reconciled::reconcile_at_entry(exec, repo, entry)
    }
}

/// The `ConfigFile` reader seam (AC-485, first half).
///
/// **This is the seam that touches the filesystem**, which is the whole reason it is the countable
/// one: [`crate::config::parse`] is pure over `&str`, so a count over *it* could not tell a re-read
/// from a re-parse of bytes already in hand. §7d.5's AC-485 test rewrites `config.toml` mid-run
/// through `MutatingHarness` and asserts the verb does **not** read it again — an assertion about a
/// file read, not about a parse.
pub trait ConfigReader {
    /// Read and parse one configuration file.
    ///
    /// # Errors
    /// [`ConfigError::Unreadable`] naming the path, or [`ConfigError::Malformed`] naming the key.
    fn read(&self, path: &Path) -> Result<ConfigFile, ConfigError>;
}

/// The one implementor on the product path.
#[derive(Debug, Clone, Copy)]
pub struct FileConfigReader;

impl ConfigReader for FileConfigReader {
    fn read(&self, path: &Path) -> Result<ConfigFile, ConfigError> {
        let src = std::fs::read_to_string(path).map_err(|e| ConfigError::Unreadable {
            path: path.display().to_string(),
            detail: e.to_string(),
        })?;
        config::parse(&src)
    }
}

/// The adapter-resolution seam (AC-485, second half).
pub trait AdapterResolver {
    /// Resolve the adapter set for this configuration and environment.
    ///
    /// # Errors
    /// [`Refusal`], which every arm of maps to exit `3`.
    fn resolve(&self, file: &ConfigFile, env: &Env) -> Result<Resolved, Refusal>;
}

/// The one implementor on the product path.
#[derive(Debug, Clone, Copy)]
pub struct RealAdapterResolver;

impl AdapterResolver for RealAdapterResolver {
    fn resolve(&self, file: &ConfigFile, env: &Env) -> Result<Resolved, Refusal> {
        AdapterSet::resolve(file, env)
    }
}

/// **The harness seam** (`s8e` §8e.11; D21-42 24c; D21-40 decision 2). *(Added 2026-09-28, `s8e`
/// build, package S8E-P2.)*
///
/// `run`'s lane loop builds one harness per stage, over the configuration that stage's bindings
/// filled, and until this seam it always built `rk_adapter_claude::ClaudeHarness` there — so no test
/// could hand the lane a `FakeHarness`, and `grep -rn "verbs::run::run(" crates/*/tests` returned
/// nothing (ADR-0161's *"Describing the lane loop as covered by tests"*). The product path builds
/// exactly what it built before ([`ClaudeHarnessFactory`]); a test hands a factory over
/// `rk_ports::fakes::FakeHarness::scripted(..)`.
pub trait HarnessFactory {
    /// Build the harness one stage of the lane dispatches through.
    ///
    /// `config` is the harness configuration the stage's binding pass filled (`Binder::into_config`),
    /// and `token` is the run's cancel token, which a real adapter receives at construction.
    ///
    /// # Errors
    /// A sentence naming why the harness could not be built; `run` prints it and exits `3`.
    fn build(
        &self,
        config: rk_adapter_claude::config::Config,
        token: rk_ports::cancel::CancelToken,
    ) -> Result<std::sync::Arc<dyn rk_ports::harness::Harness>, String>;

    /// Does this factory build the REAL harness (`rk_adapter_claude::ClaudeHarness`)?
    ///
    /// *(dated 2026-10-02, s8h build, package P6-cli-launch-gate; `s8h` §8h.7.)* `run::run` refuses
    /// a real-harness run without a launch record ([`launch_check`]); a fake-harness path is not
    /// subject. No default: every factory states which it is.
    fn builds_the_real_harness(&self) -> bool;
}

/// The one implementor on the product path: `ClaudeHarness::new(config, token)`, exactly as `run`
/// built it before the seam existed.
#[derive(Debug, Clone, Copy)]
pub struct ClaudeHarnessFactory;

impl HarnessFactory for ClaudeHarnessFactory {
    fn build(
        &self,
        config: rk_adapter_claude::config::Config,
        token: rk_ports::cancel::CancelToken,
    ) -> Result<std::sync::Arc<dyn rk_ports::harness::Harness>, String> {
        rk_adapter_claude::ClaudeHarness::new(config, token)
            .map(|harness| {
                std::sync::Arc::new(harness) as std::sync::Arc<dyn rk_ports::harness::Harness>
            })
            .map_err(|e| e.to_string())
    }

    fn builds_the_real_harness(&self) -> bool {
        true
    }
}

/// **The launch check** (`s8h` §8h.7; D21-42 17a; D21-46; ADR-0197 decision 10, proposed): `Ok`
/// only when the captured `Env` carries `REDKILN_LAUNCH_RECORD` naming an existing file, read
/// through `env.rs` (ADR-0083's one read). *(dated 2026-10-02, s8h build, package
/// P6-cli-launch-gate.)*
///
/// Called first by the two binary composition roots it binds: `run::run` when its factory builds
/// the real harness, and `answer::run` (the binary's `answer`; its in-process twin
/// `answer::run_with` is not subject). So the refusal comes before `reconcile_started` is appended
/// and before any worktree operation. It is a guard against accident, not a security boundary:
/// the variable can be forged, and that is stated.
///
/// # Errors
/// [`Exit::ConfigError`], printed naming the variable and the launcher.
pub fn launch_check(env: &Env, verb: &str) -> Result<(), Exit> {
    let refused = |why: String| {
        eprintln!(
            "redkiln: `{verb}` refused before any row or worktree: {why}. A real run and a gate \
             answer go through `tools/live-run.sh {verb}`, from a plain terminal, which writes the \
             launch record and exports {} (s8h §8h.7)",
            crate::env::LAUNCH_RECORD
        );
        Err(Exit::ConfigError)
    };
    match env.launch_record() {
        None => refused(format!("{} is not set", crate::env::LAUNCH_RECORD)),
        Some(path) if !Path::new(path).is_file() => refused(format!(
            "{} names `{path}`, which is not an existing file",
            crate::env::LAUNCH_RECORD
        )),
        Some(_) => Ok(()),
    }
}

/// The ~~three~~ four seams a verb's outer `run` takes, as one value.
///
/// So a fourth seam is one field rather than a fourth parameter on six functions. *(dated
/// 2026-09-28, `s8e` build, package S8E-P2: the fourth is [`Self::harness`], `s8e` §8e.11.)*
#[derive(Clone, Copy)]
pub struct Startup<'a> {
    /// AC-402's observable.
    pub reconciler: &'a dyn EntryReconciler,
    /// AC-485's first counter.
    pub config: &'a dyn ConfigReader,
    /// AC-485's second counter.
    pub adapters: &'a dyn AdapterResolver,
    /// The harness each stage of `run`'s lane dispatches through (`s8e` §8e.11). Only `run` reads
    /// it; every other verb dispatches nothing.
    pub harness: &'a dyn HarnessFactory,
}

impl Startup<'static> {
    /// The product ~~trio~~ set. `main.rs` and every verb's outer `run` use this one.
    #[must_use]
    pub const fn real() -> Self {
        Self {
            reconciler: &RealReconciler,
            config: &FileConfigReader,
            adapters: &RealAdapterResolver,
            harness: &ClaudeHarnessFactory,
        }
    }
}

/// Route a parsed command line to its verb.
///
/// Nine arms, one per subcommand, each a distinct call — the arms are not collapsed into one
/// wildcard, so adding a tenth subcommand is `E0004` here rather than a silent fall-through (the
/// ninth, `cost`, arrived that way: ADR-0163).
///
/// # Errors
/// A [`Halt`], when a verb stops rather than returning a code. None of Slice 1's stubs does; the
/// first is Slice 2's poisoned-lock acquisition.
pub fn dispatch(cli: &Cli, env: &Env) -> Result<Exit, Halt> {
    match &cli.command {
        Command::Run(args) => run::run(cli, args, env, Startup::real()),
        Command::Answer(args) => answer::run(cli, args, env, Startup::real()),
        Command::Status => status::run(cli, Startup::real()),
        Command::Explain(args) => explain::run(cli, args, Startup::real()),
        Command::Cost(args) => cost::run(cli, args),
        Command::Doctor(args) => doctor_with_rebuild_reopen(cli, args, env),
        Command::Init => init::run(cli.cwd.as_deref()),
        Command::Catalog(args) => catalog::run(cli, &args.command),
        Command::Lint(args) => lint::run(cli, args),
    }
}

/// `doctor`, plus the one step §7d.8 puts **after** the verb has returned.
///
/// # 🛑 THIS ARM EXISTS BECAUSE OF A MODULE-PATH BAN, AND NOT FOR CONVENIENCE
///
/// Checkpoint 7d's D-10 / D18-7 ban forbids `SqliteStore::open` on any non-comment line under a
/// module path matching `doctor`, which admits both `src/doctor/*.rs` and `src/verbs/doctor.rs`.
/// §7d.8 nonetheless requires the rebuilt store to be re-opened for its identity check *"by the
/// ROOT … after `verbs::doctor` has returned"*. `dispatch` is the one place outside that filter
/// which knows `--rebuild` was asked for, so the re-open is sequenced here and performed in
/// [`crate::adapters::rebuilt_store_identity`].
///
/// **The rebuild's own exit wins over the re-open's success**, never the other way round: a
/// refused rebuild renamed nothing, so re-opening would report on the OLD store and turn a `3`
/// into a `0`. The re-open can only make a successful rebuild fail, which is what it is for.
fn doctor_with_rebuild_reopen(cli: &Cli, args: &DoctorArgs, env: &Env) -> Result<Exit, Halt> {
    let exit = doctor::run(cli, args, env, Startup::real())?;
    if !args.rebuild || exit != Exit::Success {
        return Ok(exit);
    }
    let Ok(repo) = crate::repo::discover(cli.cwd.as_deref()) else {
        // Unreachable in practice: `doctor::run` discovered the same repository a moment ago and
        // returned `Success`. Reported rather than unwrapped, because a panic here would be the
        // root's, on a path that has already succeeded.
        eprintln!("redkiln: the repository could not be re-read after the rebuild");
        return Ok(Exit::ConfigError);
    };
    Ok(crate::adapters::rebuilt_store_identity(
        &repo.exec,
        &repo.worktree,
    ))
}
