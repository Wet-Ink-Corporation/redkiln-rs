//! **The eight verbs** — one per subcommand §7d.3 declares. `doctor` is the last stub: Slice 2
//! built `init`, and Slice 3 has built `run`, `answer`, `status`, `catalog`, `lint` and `explain`.
//!
//! **Why a stub rather than an unimplemented subcommand.** The surface is what Slice 1 delivers: a
//! committed `--help` snapshot and a denylist lint that bites from the first commit. Both need the
//! whole tree to exist. What they do not need is behaviour, and behaviour arrives one lane at a
//! time — the loop at Slice 3, `doctor` at Slice 4.
//!
//! **Why each refusal is a configuration error and not a panic.** ADR-0118 clause 6 makes every verb
//! return `Result<Exit, Halt>` so an in-process caller can observe a value instead of a dead test
//! process, and §7d.4 allows `std::process::exit` exactly two sites, both in `main.rs`. A stub that
//! reached for `unimplemented!()` would exit `101` through a path the exit table does not describe.
//!
//! **This file is where golden constraint 4 bites**: *a stub never replaced is caught only here* —
//! by the block's verb-level lines at the stage close, which drive the real binary and expect a real
//! answer. `doctor` still exits `3` and says so; it is Slice 4's.

pub mod answer;
pub mod catalog;
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
use crate::cli::{Cli, Command};
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

/// The three seams a verb's outer `run` takes, as one value — so a fourth seam is one field rather
/// than a fourth parameter on six functions.
#[derive(Clone, Copy)]
pub struct Startup<'a> {
    /// AC-402's observable.
    pub reconciler: &'a dyn EntryReconciler,
    /// AC-485's first counter.
    pub config: &'a dyn ConfigReader,
    /// AC-485's second counter.
    pub adapters: &'a dyn AdapterResolver,
}

impl Startup<'static> {
    /// The product trio. `main.rs` and every verb's outer `run` use this one.
    #[must_use]
    pub const fn real() -> Self {
        Self {
            reconciler: &RealReconciler,
            config: &FileConfigReader,
            adapters: &RealAdapterResolver,
        }
    }
}

/// Route a parsed command line to its verb.
///
/// Eight arms, one per subcommand, each a distinct call — the arms are not collapsed into one
/// wildcard, so adding a ninth subcommand is `E0004` here rather than a silent fall-through.
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
        Command::Doctor(_) => doctor(),
        Command::Init => init::run(cli.cwd.as_deref()),
        Command::Catalog(args) => catalog::run(cli, &args.command),
        Command::Lint(args) => lint::run(cli, args),
    }
}

/// The one refusal every stub prints, so the eight are one sentence with one subject.
fn not_built(verb: &str) -> Result<Exit, Halt> {
    eprintln!("redkiln: `{verb}` is not built at slice 1");
    Ok(Exit::ConfigError)
}

fn doctor() -> Result<Exit, Halt> {
    not_built("doctor")
}
