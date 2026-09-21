//! **`doctor` — the rule table run over one `Ctx`.** `s7d` §7d.8, design §8.1.
//!
//! This file replaces the inline Slice-1 stub that lived in `verbs/mod.rs` and exited `3` saying
//! *"not built at slice 1"*. It is the **real body**: discover the repository, read the
//! configuration, reconcile at entry, build the `Ctx`, evaluate every rule, print one line per
//! evaluated rule, exit `0` or `3`.
//!
//! # 🛑 It reconciles, and the entry is `Entry::Doctor`
//!
//! AC-402's *"three paths"*, and C-01 D-06 dropped the finder's doubt about it: `rk_git`'s `Entry`
//! is exactly `{ Run, Answer, Doctor }`, and `status` / `explain` cannot reach the seam because no
//! variant names them. `ac402_reconcile_called_on_run_answer_doctor` is the test, and it is cycle
//! 10's — this cycle claims none.
//!
//! # 🛑 It does NOT call `AdapterSet::resolve` as a startup refusal
//!
//! §7d.8, verbatim: *"`doctor` does **NOT** call `AdapterSet::resolve` as a startup refusal (that
//! is `run`'s step (4)): it builds its `Ctx` from the `ConfigFile` and reports adapter defects only
//! through `RK-D-ADAPTER`, **which is why the row and its id are observable on the real binary**."*
//! So `Startup`'s `adapters` seam is taken and not called — the same shape, and for the same
//! reason, as `status` taking the reconciler it must not use: a verb that *could not* resolve would
//! make `ac486_unknown_adapter_is_a_doctor_error`'s row true of every possible body.
//!
//! The store is the second half of the same rule. `Ctx.store` is `Option<Result<…>>` and a foreign
//! store is carried **in** as `Some(Err(..))`, never refused before the table, so `RK-D-STORE-ID`
//! renders it. It is opened by [`crate::adapters::open_store`] — the root's path, outside the
//! module Checkpoint 7d's D-10 ban filters — because `SqliteStore::open` may not appear in this
//! file or under `src/doctor/`.
//!
//! # 🛑 It never constructs a real `Harness` (D18-7; AC-414 as amended)
//!
//! The `Ports` it builds take [`crate::no_dispatch::NoDispatchHarness`], which panics on dispatch
//! and names the rule that reached it; `tracker` and `docs` are the two Phase-1 stand-ins and
//! `toolchain` is `FixtureToolchain::default().absent()` — a builder over `self`, not an associated
//! constructor, and `doctor` calls no toolchain method.
//!
//! # 🚧 CYCLE 2: `RULES` is empty, and `--rebuild` is refused with a reason
//!
//! The table this drives is empty until cycle 3, so a `doctor` run here prints nothing and exits
//! `0`. That is honest and it is **not** claimed as a pass: §7d.14 records that over an empty rule
//! table AC-414, AC-415 and SM-121 are all vacuous, and this cycle claims none of the three.
//!
//! **`--rebuild` returns a refusal rather than running a half-built rebuild**, and that is a
//! decision taken at this site rather than an omission. The rebuild path *renames a new store over
//! the old* (ADR-0105 clause 7); `doctor::rebuild::events_from_trailers` returns no events until
//! cycle 9, so running it would rename an **empty** store over a real one — it would destroy the
//! journal it exists to reconstruct. The refusal is a value through the exit table, on `run.rs`'s
//! own cycle-6 precedent, and cycle 9 replaces it.

use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_git::reconciled::Entry;
use rk_ports::event_store::EventStore;

use crate::cli::{Cli, DoctorArgs};
use crate::doctor::{Ctx, Report, probe, rebuild, rules};
use crate::env::Env;
use crate::exit::{Exit, Halt};
use crate::no_dispatch::NoDispatchHarness;
use crate::verbs::Startup;
use crate::{adapters, repo};

/// The outer verb: discover, read, reconcile, build the `Ctx`, delegate.
///
/// # Errors
/// [`Halt::PoisonedLock`] never arises here today — `doctor` takes no store lock, because its rules
/// read the handle rather than the journal through it. The signature matches the other verbs so the
/// router's arms are one shape.
pub fn run(cli: &Cli, args: &DoctorArgs, env: &Env, startup: Startup<'_>) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let root = Path::new(repo.worktree.as_str()).to_path_buf();

    // The configuration, through AC-485's counting seam. A repository with no `.redkiln/` is a
    // refusal and not a row: every rule below reads this value, so there is no table to run.
    let config_path = root.join(".redkiln").join("config.toml");
    let config = match startup.config.read(&config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    // RECONCILE AT ENTRY (AC-402; ADR-0105 clause 8), before the store is touched.
    if let Err(e) = startup
        .reconciler
        .reconcile(&repo.exec, &repo.worktree, Entry::Doctor)
    {
        // `{e:?}`: `rk_git::error::GitError` implements no `Display` [measured 2026-09-20], and
        // `run.rs`'s own reconcile arm renders it the same way.
        eprintln!("redkiln: reconcile at entry refused: {e:?}");
        return Ok(Exit::ConfigError);
    }

    if args.rebuild {
        // 🛑 THE REBUILD PATH RETURNS HERE, BEFORE `Ctx::build`, AND THAT ORDERING IS THE RULE.
        //
        // §7d.8: *"For `--rebuild` the root does NOT hand the open handle in"* — `Ctx.store` is
        // `None` on this path. Returning above `Ctx::build` is how that is expressed as code
        // rather than as a comment: no `SqliteStore` exists to hold the file open when
        // `rebuild::run` renames over it, which on Windows is a sharing violation and not a
        // style point. The rebuilt store is re-opened for its identity check BY THE ROOT
        // (`adapters::rebuilt_store_identity`, called from `verbs::dispatch`) after this
        // function has returned — the module the block's `doctor` filter does not admit.
        return Ok(rebuild_and_report(
            &repo.exec,
            &repo.worktree,
            &repo.common_dir,
            &root,
        ));
    }

    let ctx = Ctx::build(
        root,
        repo.common_dir.clone(),
        config,
        env.clone(),
        // Carried in, never refused here — the type's whole reason. See the header.
        Some(adapters::open_store(&repo.exec, &repo.worktree)),
        Box::new(probe::RealProbe),
        // THE ONE WALL-CLOCK READ (§7d.5). No rule reads a clock; the quarantine row's age
        // is `now - until` off this value.
        crate::clock::now().unix_seconds(),
    );

    let mut out = std::io::stdout();
    Ok(evaluate_and_render(&ctx, &mut out))
}

/// Run the rebuild and print what it could and could not resolve.
///
/// # 🛑 `unresolvable` IS PRINTED, NOT SWALLOWED
///
/// AC-165: a mark that does not become a transition is *"surfaced with the offending text, never
/// skipped and never coerced"*. Every `Gate:` mark is one of these by construction (§7d.8), and so
/// is any `Story:` mark whose item record carries no stage. A rebuild that printed only its
/// successes would report a store reconstructed from *some* of git as though it were all of it.
///
/// The exit is `Success` — an incomplete rebuild is a reported fact about the repository, not a
/// configuration error in it. Only a refusal is `ConfigError`, and a refusal renames nothing.
fn rebuild_and_report(
    exec: &rk_git::git_exec::GitExec,
    worktree: &rk_git::path::WorktreePath,
    common_dir: &Path,
    root: &Path,
) -> Exit {
    match rebuild::run(exec, worktree, common_dir, root) {
        Ok(outcome) => {
            println!(
                "rebuilt {} tier-1 row(s) from git at {}",
                outcome.events_written, outcome.head
            );
            for mark in &outcome.unresolvable {
                println!("unresolvable-from-git-alone {mark}");
            }
            Exit::Success
        }
        Err(e) => {
            eprintln!("redkiln: `doctor --rebuild` refused, and nothing was renamed: {e}");
            Exit::ConfigError
        }
    }
}

/// The inner verb — the in-process twin every Slice-4 test drives.
///
/// # 🛑 It builds the `Ports` and hands them nowhere yet, and that is not dead code
///
/// §7d.8: *"the root builds the `doctor` path's `Ports` … and hands it in"*, and AC-414's
/// Verified-by is *"the `doctor` integration test injects a `HarnessPort` fake that fails the test
/// on any dispatch"*. The harness is bound to the rule loop through
/// [`rules::evaluate`], which names each rule to it before running the check — so a dispatch from
/// inside any rule panics with that rule's id. The other three ports are constructed because
/// `Ports` is a closed struct and because the **absence** of a real harness among them is what
/// AC-414 asserts; a `doctor` that built no `Ports` at all would make that assertion true of a verb
/// that could not dispatch under any body.
#[must_use]
pub fn evaluate_and_render(ctx: &Ctx, out: &mut dyn Write) -> Exit {
    let harness = Arc::new(NoDispatchHarness::default());
    let store: Arc<Mutex<dyn EventStore>> =
        Arc::new(Mutex::new(rk_ports::fakes::VecStore::default()));
    let _ports = rk_ports::root::Ports {
        store,
        harness: Arc::clone(&harness) as Arc<_>,
        tracker: Arc::new(rk_ports::fakes::BoardTracker::default()),
        docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
        toolchain: Arc::new(rk_ports::fakes::FixtureToolchain::default().absent()),
    };

    let report = rules::evaluate(ctx, &harness);
    render(&report, out);
    if report.any_error() {
        Exit::ConfigError
    } else {
        Exit::Success
    }
}

/// Print one line per **evaluated** rule — fired or not (RT-44; §7d.8).
///
/// `<id> <status> <detail>`, with `ok` a status word. **Nothing else about the line is pinned**
/// (RT-44, E5, D18-6 as corrected 2026-09-18), so the separator and the detail's wording are this
/// implementation's and are not asserted anywhere.
///
/// A rule that found nothing prints `<id> ok` and no detail. That is what makes the checkpoint
/// block's FLOOR loop — which greps the register-named ids off a **clean** repository — find them:
/// a renderer that printed only findings would make that loop find nothing on a green tree, which
/// is Halt 8's shape one level up.
fn render(report: &Report, out: &mut dyn Write) {
    for id in &report.evaluated {
        let mut fired = report.findings.iter().filter(|f| f.id == *id).peekable();
        if fired.peek().is_none() {
            let _ = writeln!(out, "{id} ok");
            continue;
        }
        for finding in fired {
            let _ = writeln!(out, "{id} {} {}", finding.severity.word(), finding.detail);
        }
    }
}
