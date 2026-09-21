//! `redkiln-migrate` — the binary (`s7d` §7d.11).
//!
//! `redkiln-migrate --target <path> [--dry-run]`, and `--version`. **No other verb and no other
//! flag**: AC-474 forbids any configuration key, flag or environment variable that could enable a
//! second write path, and `ac474_no_dual_write_key_flag_or_env` snapshots exactly this set.
//!
//! # RT-53's exit values — routed, not decided
//!
//! Unpinned by any register criterion: AC-438 governs `redkiln` only, and DECISIONS RT-35 says the
//! migrator *"has its own smaller table"*. They are **not snapshot-pinned** until a record asks for
//! it, which is why this table lives here and not in an `insta` snapshot beside `rk-cli`'s.
//!
//! | code | meaning |
//! | --- | --- |
//! | `0` | success, `--dry-run`, and the AC-450 no-op |
//! | `1` | any other error |
//! | `2` | `clap` usage (clap's own default, not re-implemented here) |
//! | `3` | the AC-522 dirty-target refusal and the AC-447 / AC-448 aborts before writing |
//!
//! `refuses_dirty_target` asserts the `3` and the block's dry run the `0`.

use std::path::PathBuf;

use clap::Parser;
use redkiln_migrate::run_contract::{self, Completed, MigrateError, Refusal};

/// The old-corpus converter — a separate binary, never a `redkiln` verb (AC-441).
#[derive(Debug, Parser)]
#[command(name = "redkiln-migrate", version, about, long_about = None)]
struct Cli {
    /// The repository to migrate.
    #[arg(long, value_name = "PATH")]
    target: PathBuf,

    /// Read and report, writing nothing in the working tree, refs or objects.
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    // clap exits 2 on a usage error of its own accord; that is RT-53's `2`.
    let cli = Cli::parse();

    // `Hooks::default()` and nothing else. The AC-447 and AC-448 instruments are PARAMETERS of the
    // library entry, never a flag or an environment variable reachable from here — that is the
    // whole of AC-474's second conjunct.
    match run_contract::run(&cli.target, cli.dry_run, run_contract::Hooks::default()) {
        Ok(Completed::DryRun | Completed::NoOp | Completed::Committed { .. }) => {
            std::process::exit(0)
        }
        Err(MigrateError::Refused(refusal)) => {
            report_refusal(&refusal);
            std::process::exit(3)
        }
        Err(MigrateError::Other(err)) => {
            eprintln!("redkiln-migrate: {err:#}");
            std::process::exit(1)
        }
    }
}

/// Print a refusal, naming what it refused over — AC-522 requires the dirty paths by name, and a
/// refusal that named nothing would leave the operator with a `3` and no way to act on it.
fn report_refusal(refusal: &Refusal) {
    match refusal {
        Refusal::DirtyTarget { paths } => {
            eprintln!(
                "redkiln-migrate: the target is dirty and nothing was written (AC-522). \
                 Commit or stash these {} path(s) and run again:",
                paths.len()
            );
            for path in paths {
                eprintln!("  {path}");
            }
        }
        Refusal::SecondWritePhase => eprintln!(
            "redkiln-migrate: a second write phase was requested; the run aborted before writing \
             (AC-447)"
        ),
        Refusal::NoReport => eprintln!(
            "redkiln-migrate: the layout would have been written without a differential report; \
             the run was refused before the commit (AC-448)"
        ),
        Refusal::WorktreePathExists { path } => eprintln!(
            "redkiln-migrate: {} already exists; the migration worktree is re-created, never \
             reused (ADR-0105 clause 6)",
            path.display()
        ),
    }
}
