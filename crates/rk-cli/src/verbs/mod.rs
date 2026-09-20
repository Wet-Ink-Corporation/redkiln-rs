//! **The eight verbs** — one per subcommand §7d.3 declares, and at Slice 1 every one of them is a
//! stub that refuses.
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
//! answer. Until then these eight exit `3` and say so.

pub mod init;

use crate::cli::{Cli, Command};
use crate::exit::{Exit, Halt};

/// Route a parsed command line to its verb.
///
/// Eight arms, one per subcommand, each a distinct call — the arms are not collapsed into one
/// wildcard, so adding a ninth subcommand is `E0004` here rather than a silent fall-through.
///
/// # Errors
/// A [`Halt`], when a verb stops rather than returning a code. None of Slice 1's stubs does; the
/// first is Slice 2's poisoned-lock acquisition.
pub fn dispatch(cli: &Cli) -> Result<Exit, Halt> {
    match &cli.command {
        Command::Run(_) => run(),
        Command::Answer(_) => answer(),
        Command::Status => status(),
        Command::Explain(_) => explain(),
        Command::Doctor(_) => doctor(),
        Command::Init => init::run(cli.cwd.as_deref()),
        Command::Catalog(_) => catalog(),
        Command::Lint(_) => lint(),
    }
}

/// The one refusal every stub prints, so the eight are one sentence with one subject.
fn not_built(verb: &str) -> Result<Exit, Halt> {
    eprintln!("redkiln: `{verb}` is not built at slice 1");
    Ok(Exit::ConfigError)
}

fn run() -> Result<Exit, Halt> {
    not_built("run")
}

fn answer() -> Result<Exit, Halt> {
    not_built("answer")
}

fn status() -> Result<Exit, Halt> {
    not_built("status")
}

fn explain() -> Result<Exit, Halt> {
    not_built("explain")
}

fn doctor() -> Result<Exit, Halt> {
    not_built("doctor")
}

fn catalog() -> Result<Exit, Halt> {
    not_built("catalog")
}

fn lint() -> Result<Exit, Halt> {
    not_built("lint")
}
