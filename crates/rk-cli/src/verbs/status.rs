//! **`status` — the roll-up read.** §7d.7, design §8.1: *"roll-up read; read-only, so it does not
//! reconcile"*.
//!
//! It opens the store, reads the journal, and prints
//! `rk_kernel::workflow::fsm::stage_action(item, stage, &events)` per item. **No
//! `reconcile_at_entry`** (ADR-0105 clause 8: *"AC-172: `status` does not reconcile"*), **no clock**
//! (the fold is over recorded events), no git write, no store append.
//!
//! # 🛑 IT TAKES EVERYTHING RECONCILING WOULD NEED, AND CALLS NOTHING
//!
//! `run_with` is handed the `&dyn EntryReconciler` **and** the `&GitExec` and `&WorktreePath` that
//! seam's one method requires, and uses none of them. That is deliberate and is what makes
//! `ac402_status_and_explain_do_not_reconcile` mean anything: **a verb that *cannot* call the seam
//! proves nothing about a verb that chose not to.** Handing in the trait object alone would have
//! been exactly that — `reconcile_at_entry` needs a wrapper and a worktree, so a function holding
//! neither could not reconcile under any body, and the test's zero would have been true of every
//! possible implementation. The type is the second guard, and it is `rk_git`'s rather than this
//! crate's: `Entry` is exactly `{ Run, Answer, Doctor }`, so there is no `Entry::Status` to pass
//! even if this function wanted one (§7d.7: *"the type forbids it"*).
//!
//! # Why an empty store is exit `0` and not `5`
//!
//! Three independent grounds. The kind of verb — a read over a store with nothing in it has nothing
//! to report; it does not fail to report. The store's own answer — `SqliteStore::query` over an
//! empty `t2_event` returns `Ok(vec![])`, and there is no empty-store refusal anywhere on this
//! path. And §7d.17's block asserts it in terms, over the **init-only** repository: *"a roll-up read
//! over an empty store is not an error; ADR-0105 c8"*. `Exit::NoRecord` = `5` is documented on the
//! tree as AC-432's row and is **`explain`'s alone**.
//!
//! # 🛑 RAISED — THE ITEM PARTITION OF THE ITEM-LESS ROWS IS DECIDED BY NO RECORD
//!
//! The store's `query` **ignores its selector**: `SqliteStore::query`'s body is
//! `SELECT kind, payload FROM t2_event ORDER BY id` with no `WHERE`, and the crate's own doc says
//! *"every selector — `ByItem`, `BySha`, `ByRun` — returns everything … the event carries no item
//! id, commit SHA or run id to filter on until that same record lands"*. `rk_ports::fakes::VecStore`
//! matches it. So **the per-item partition is `rk-cli`'s, here.** Three tier-1 kinds carry no item
//! at all — `stage_transition` is `{from, to}`, `gate_opened` is
//! `{deadline, kind, reach, token, tree_hash}`, and `dag_assembled` carries neither `run_id` nor
//! `item` — and **no record says which item they belong to.** This module hands them to every item,
//! because the store cannot tell and inventing an owner would be worse than over-reporting; a
//! multi-item store therefore over-reports its transitions and gates. Slice 3's stores hold one item,
//! so nothing here is measured against that case. RAISED, not settled.

use std::collections::BTreeMap;
use std::io::Write;
use std::sync::{Arc, Mutex};

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_kernel::workflow::fsm::{self, StageAction};
use rk_ports::event_store::EventStore;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::cli::Cli;
use crate::exit::{Exit, Halt};
use crate::verbs::{EntryReconciler, Startup};
use crate::{repo, store};

/// The outer verb: discover the repository, open the real store, delegate.
///
/// # Errors
/// [`Halt::PoisonedLock`] from [`crate::store::acquire`]. Every repository and store refusal is
/// exit `3` and is returned as a value, not a halt.
pub fn run(cli: &Cli, startup: Startup<'_>) -> Result<Exit, Halt> {
    run_to(cli, startup, &mut std::io::stdout())
}

/// *(dated 2026-09-26, s8c build; D21-64 ruling 3.)* [`run`] with the writer handed in; an
/// in-process caller passes its own, so nothing it renders reaches libtest's stdout.
///
/// # Errors
/// As [`run`].
pub fn run_to(cli: &Cli, _startup: Startup<'_>, out: &mut dyn Write) -> Result<Exit, Halt> {
    let repo = match repo::discover(cli.cwd.as_deref()) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    // AC-413's identity check lives inside `open`: a store belonging to another repository is
    // refused here, before a single row is read.
    let opened = match rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
    run_with(
        &crate::verbs::RealReconciler,
        &repo.exec,
        &repo.worktree,
        &store,
        out,
    )
}

/// The inner verb — the in-process twin every Slice-3 test drives.
///
/// # 🛑 IT TAKES `exec` AND `worktree` IT DOES NOT USE, AND THAT IS THE POINT
///
/// `reconciler`, `exec` and `worktree` are the three arguments
/// `rk_git::reconciled::reconcile_at_entry` needs, and this function takes **all three** and calls
/// nothing. Handing in the seam without the values it needs would leave the recorder's zero
/// **unfalsifiable** — no implementation of a function holding no `GitExec` could reconcile, so the
/// assertion would be true of every possible body and would be evidence of nothing. That is the same
/// guard-that-cannot-fail shape one level down from the one §7d.14's declared mutant already
/// escapes, and it was found by running that mutant rather than by reading the code.
///
/// # Errors
/// [`Halt::PoisonedLock`] naming this verb.
pub fn run_with(
    reconciler: &dyn EntryReconciler,
    exec: &GitExec,
    worktree: &WorktreePath,
    store: &Arc<Mutex<dyn EventStore>>,
    out: &mut dyn Write,
) -> Result<Exit, Halt> {
    let (_, _, _) = (reconciler, exec, worktree);
    let guard = store::acquire(store, "status")?;
    // ONE query. Per the header the selector is inert, so this is the whole journal and calling it
    // once per item would read the same rows N times.
    let events = match guard.query(rk_kernel::runtime::journal_query("")) {
        Ok(events) => events,
        Err(e) => {
            // `StepError` has no `Display` — it is a closed enum its own crate matches
            // exhaustively — so the `Debug` rendering is carried, on `repo::RepoError`'s precedent
            // one module over and for the same reason: it keeps every field in the diagnostic.
            eprintln!("redkiln: the store could not be read: {e:?}");
            return Ok(Exit::ConfigError);
        }
    };
    drop(guard);

    for (item, action) in roll_up(&events) {
        match action {
            Ok(action) => writeln!(out, "{item}\t{}", render(&action)).unwrap_or(()),
            Err(e) => {
                eprintln!("redkiln: {item}: the recorded journal did not fold: {e}");
                return Ok(Exit::ConfigError);
            }
        }
    }
    Ok(Exit::Success)
}

/// Which item a row belongs to, or `None` for the three kinds that carry no item.
fn item_of(event: &Event) -> Option<&str> {
    event
        .payload
        .get("item")
        .and_then(serde_json::Value::as_str)
}

/// The stage every item is folded against: the `to` of the LAST recorded `stage_transition`.
///
/// Empty when the journal records no transition — which is honest rather than a default. `fold`
/// uses this only to seed `RunState::initial`, and `stage_action` filters by tier and nothing else.
fn recorded_stage(events: &[Event]) -> String {
    events
        .iter()
        .rev()
        .find(|e| e.kind == EventKind::StageTransition)
        .and_then(|e| e.payload.get("to"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

/// One `stage_action` per item, in item order. Empty for an empty journal — which is the whole of
/// why the init-only repository exits `0`.
fn roll_up(events: &[Event]) -> Vec<(String, Result<StageAction, fsm::FsmError>)> {
    let stage = recorded_stage(events);
    let mut per_item: BTreeMap<String, Vec<Event>> = BTreeMap::new();
    for event in events {
        if let Some(item) = item_of(event) {
            per_item
                .entry(item.to_owned())
                .or_default()
                .push(event.clone());
        }
    }
    // The item-less tier-1 rows, handed to every item — see the header's RAISED note.
    let shared: Vec<&Event> = events.iter().filter(|e| item_of(e).is_none()).collect();
    per_item
        .into_iter()
        .map(|(item, own)| {
            let mut slice: Vec<Event> = shared.iter().map(|e| (*e).clone()).collect();
            slice.extend(own);
            let folded = fsm::stage_action(&item, &stage, &slice);
            (item, folded)
        })
        .collect()
}

/// One line per item. **Its shape is not pinned** — §7d.7: *"Its stdout is unpinned (E5)."* No
/// snapshot and no `.trycmd` body asserts these bytes, so this rendering may change without a
/// record moving.
fn render(action: &StageAction) -> String {
    let transition = action
        .transition
        .as_ref()
        .map_or_else(|| "-".to_owned(), |(from, to)| format!("{from}->{to}"));
    format!("{transition}\tgates={}", action.gates.len())
}
