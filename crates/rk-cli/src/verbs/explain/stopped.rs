//! **A run that stopped at a failed step, as `explain` names it** (ADR-0204 decision 7, proposed;
//! plan Decision D7; added 2026-11-08, s8 repro-pair route).
//!
//! `run` writes the run record (`replay::write_run`) only on success, so a run that stopped at a
//! failed step leaves `activity_committed` rows naming its run id and no `runs/<id>/run.json`. A
//! partial record would be a format migration, and the `RecordedRun` it needs does not exist on the
//! error path, so `explain` reads the journal instead:
//!
//! ```text
//! run: <id> (no run record: the run stopped at a failed step before its record was written)
//! stopped: <kind> <class, capability, status and step members, compact JSON in that order>
//! last recorded run:
//! ```
//!
//! The `stopped:` line is the item's last `step_failed` or `step_refused` row after `<id>`'s last
//! `activity_committed` row, when there is one. `last recorded run:` precedes the newest earlier
//! run whose record exists, which the unchanged `render` then prints. Every read here is of the
//! journal rows the caller already holds, or a metadata probe of a record path: no selection
//! (AC-432).

use std::path::Path;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The members a `stopped:` line carries, in this order, when the row has them.
const MEMBERS: [&str; 5] = ["class", "capability", "status", "step_id", "step"];

/// The stopped block (empty when the newest run has a record) and the run to render.
///
/// A record path whose probe fails for any reason other than "not found" counts as present, so
/// `read_run` reports the failure by name rather than this probe hiding it.
pub(super) fn resolve(rows: &[Event], item: &str, runs: &Path) -> (String, Option<String>) {
    let mut ids = run_ids(rows, item).into_iter();
    let Some(newest) = ids.next() else {
        return (String::new(), None);
    };
    let has_record = |id: &str| !matches!(runs.join(id).join("run.json").try_exists(), Ok(false));
    if has_record(&newest) {
        return (String::new(), Some(newest));
    }
    let mut text = block(rows, item, &newest);
    let found = ids.find(|id| has_record(id));
    if found.is_some() {
        text.push_str("last recorded run:\n");
    }
    (text, found)
}

/// The item's distinct `activity_committed` run ids, newest first.
pub(super) fn run_ids(rows: &[Event], item: &str) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for row in rows.iter().rev().filter(|e| committed_for(e, item)) {
        if let Some(id) = row
            .payload
            .get("run_id")
            .and_then(serde_json::Value::as_str)
            && !ids.iter().any(|seen| seen == id)
        {
            ids.push(id.to_owned());
        }
    }
    ids
}

/// The `run:` line for the stopped run `id`, and its `stopped:` line when a failure row follows.
pub(super) fn block(rows: &[Event], item: &str, id: &str) -> String {
    let mut text = format!(
        "run: {id} (no run record: the run stopped at a failed step before its record was \
         written)\n"
    );
    let after = rows
        .iter()
        .rposition(|e| {
            committed_for(e, item)
                && e.payload.get("run_id").and_then(serde_json::Value::as_str) == Some(id)
        })
        .map_or(0, |at| at.saturating_add(1));
    let failure = rows.iter().skip(after).rev().find(|e| {
        matches!(e.kind, EventKind::StepFailed | EventKind::StepRefused) && keyed_to(e, item)
    });
    if let Some(row) = failure {
        let mut members = serde_json::Map::new();
        for name in MEMBERS {
            if let Some(value) = row.payload.get(name) {
                members.insert(name.to_owned(), value.clone());
            }
        }
        text.push_str("stopped: ");
        text.push_str(row.kind.wire_name());
        text.push(' ');
        text.push_str(&serde_json::Value::Object(members).to_string());
        text.push('\n');
    }
    text
}

/// An `activity_committed` row naming `item` in its payload.
fn committed_for(row: &Event, item: &str) -> bool {
    row.kind == EventKind::ActivityCommitted
        && row.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
}

/// A row that is the item's: keyed to it (ADR-0182), or naming it in its payload.
fn keyed_to(row: &Event, item: &str) -> bool {
    row.item() == Some(item)
        || row.payload.get("item").and_then(serde_json::Value::as_str) == Some(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn committed(item: &str, run: &str) -> Event {
        Event::new(
            EventKind::ActivityCommitted,
            serde_json::json!({ "item": item, "run_id": run }),
        )
    }

    fn refused(item: &str) -> Event {
        Event::new(
            EventKind::StepRefused,
            serde_json::json!({ "at": 1, "capability": "c", "class": "unsupported" }),
        )
        .with_item(item)
    }

    #[test]
    fn run_ids_are_distinct_and_newest_first() {
        let rows = [
            committed("i", "a"),
            committed("i", "b"),
            committed("j", "c"),
            committed("i", "b"),
        ];
        assert_eq!(run_ids(&rows, "i"), vec!["b".to_owned(), "a".to_owned()]);
    }

    #[test]
    fn the_stopped_line_is_the_last_failure_after_the_runs_last_commit() {
        let rows = [
            refused("i"),
            committed("i", "x"),
            refused("j"),
            refused("i"),
        ];
        assert_eq!(
            block(&rows, "i", "x"),
            "run: x (no run record: the run stopped at a failed step before its record was \
             written)\nstopped: step_refused {\"class\":\"unsupported\",\"capability\":\"c\"}\n"
        );
    }

    #[test]
    fn a_failure_before_the_runs_commit_is_not_its_stop() {
        let rows = [refused("i"), committed("i", "x")];
        assert!(!block(&rows, "i", "x").contains("stopped:"));
    }
}
