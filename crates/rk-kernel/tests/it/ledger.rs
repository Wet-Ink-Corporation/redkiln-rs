//! **The executed-once ledger** — `src/runtime/ledger.rs`. Slice 4h.
//!
//! One test, and its name is the runbook's: `ac110_executed_ledger_first_and_last` is asserted by
//! Checkpoint 7b's **first** name loop (:1498-1507) and the block halts without it.
//!
//! 🛑 **A GREEN HERE IS NOT AC-110 DISCHARGED, AND THE NAME IS THE TRAP.** AC-110's own
//! GIVEN/WHEN/THEN is about *"an activity that lives only in the unshipped extended catalog file"*
//! failing as *"a configuration error at load time"*, and its *Verified by* names a loader unit
//! test, a `doctor` rule-table row and an integration test over the resolved catalog. **None of the
//! three is a ledger.** `s7b-kernel.md` §7b.10's *"the register routes this through AC-110"* is the
//! runbook adding a THEN the register does not carry, which ADR-0092 clause 2.2 forbids.
//!
//! Residual 6 is FORCED and its Read line is followed exactly: **build it, do not stop** — an
//! accepted ADR is authority enough to execute, and `00-preamble.md`:64 forbids a stage from
//! *deciding*. So what this file measures is **ADR-0114 decision 2**, and **AC-110's ledger coverage
//! is reported POWERLESS** (golden constraint 10). A register amendment is **OWED**. AC-110's
//! `doctor` half is 7d's per §7b.15's ARMED table and is not claimed here either.
//!
//! **The test is not named after AC-114**, and that is also the runbook's instruction: *"the register
//! marks AC-114 **HUMAN GATE**, and naming a red test after it would claim a human gate as
//! buildable."*

use rk_kernel::runtime::activity::activity_committed_event;
use rk_kernel::runtime::ledger::{Executed, LedgerError, committed_rows, fold_executed};
use rk_kernel::workflow::idem::checked_key;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;
use rk_types::typed::{SchemaId, Typed};

use crate::runtime::{ITEM, SCHEMA, at};

const A: &str = "impl-report@1";
const B: &str = "changeset@2";

/// One `activity_committed` as the **runtime's own writer** produces it.
///
/// Built through `rk_kernel::runtime::activity::activity_committed_event` rather than hand-written,
/// so the ledger folds the shape the kernel actually appends. A journal hand-written in a test
/// asserts only what the test author typed.
///
/// # Panics
///
/// If the key is malformed, which none below is.
fn commit(activity: &str, run_id: &str, at_secs: i64, attempt: u32) -> Event {
    let key = checked_key(ITEM, activity, attempt).expect("the fixture keys are well-formed");
    activity_committed_event(
        &key,
        &Typed::new(SchemaId::new(SCHEMA), serde_json::json!({"run": run_id})),
        run_id,
        at(at_secs),
    )
}

/// **ADR-0114 decision 2**: *"The catalog record gains `executed: { first: Option<{ run_id, item, at
/// }>, last: Option<{ run_id, item, at }> }`, written by the kernel on the **first and every
/// subsequent** `activity_committed` for that activity."*
///
/// Six obligations:
///
/// * (a) three commits for one activity: `first` is the earliest stamp, `last` is the latest, and
///   both carry all three of `{run_id, item, at}`;
/// * (b) a fourth commit moves `last` and **not** `first` — the clause's *"and every subsequent"*;
/// * (c) **the control**: a second activity is folded separately, so the ledger is keyed and not a
///   single global pair; and an activity with no commits at all reads `never`, which is §7b.10's
///   *"at v0 every activity but `root-cause`'s P7 members reads `executed: never`"*;
/// * (d) **the discriminating case**: a journal in which append order and `at` order **disagree**,
///   which is where the two readings of *"first"* come apart. This build takes **journal order**,
///   and the module header states it as a reading with the record owed;
/// * (e) the four refusals — a row missing `activity_id_at_version`, `item`, `run_id` or `at` is
///   `Err(MalformedPayload)` and **never a zero, never an epoch**;
/// * (f) `executions()`, so *"`first` and `last` are the same stamp"* is distinguishable from
///   *"the fold saw one row"*.
#[test]
fn ac110_executed_ledger_first_and_last() {
    // ---- (a) ---------------------------------------------------------------------------------
    let journal = vec![
        commit(A, "run-1", 100, 1),
        commit(B, "run-1", 150, 1),
        commit(A, "run-2", 200, 1),
        commit(A, "run-3", 300, 1),
    ];
    assert_eq!(
        committed_rows(&journal).len(),
        4,
        "PREMISE: four `activity_committed` rows, or the fold below has nothing to be wrong about"
    );

    let led = fold_executed(&journal).expect("the rows are well-formed");
    let a_entry = led.get(A).expect("activity A is in the ledger");
    let first = a_entry.first.as_ref().expect("A executed");
    let last = a_entry.last.as_ref().expect("A executed");
    assert_eq!(first.run_id, "run-1");
    assert_eq!(first.item, ITEM);
    assert_eq!(first.at, Timestamp::from_unix_seconds(100));
    assert_eq!(last.run_id, "run-3");
    assert_eq!(last.item, ITEM);
    assert_eq!(last.at, Timestamp::from_unix_seconds(300));
    assert_eq!(a_entry.executions(), 3, "(f) three rows, not one");
    assert!(!a_entry.never());

    // ---- (b) "and every subsequent" -----------------------------------------------------------
    let mut extended = journal;
    extended.push(commit(A, "run-4", 400, 1));
    let led2 = fold_executed(&extended).expect("still well-formed");
    let a2 = led2.get(A).expect("A");
    assert_eq!(
        a2.last.as_ref().expect("A").run_id,
        "run-4",
        "a fourth commit MOVES `last`"
    );
    assert_eq!(
        a2.first, a_entry.first,
        "and leaves `first` exactly where it was — a ledger that overwrote `first` would pass \
         every other assertion in this test"
    );
    assert_eq!(a2.executions(), 4);

    // ---- (c) the control ----------------------------------------------------------------------
    let b_entry = led.get(B).expect("activity B is in the ledger");
    assert_eq!(b_entry.executions(), 1);
    assert_eq!(b_entry.first, b_entry.last, "one row, so first IS last");
    assert_eq!(
        b_entry.first.as_ref().expect("B").at,
        Timestamp::from_unix_seconds(150),
        "CONTROL: B is folded SEPARATELY — a ledger holding one global pair would report 100/300 \
         here, and every A assertion above would still pass"
    );
    assert_eq!(led.len(), 2, "two activities, two entries");
    assert!(
        !led.contains_key("review@1"),
        "an activity with no commits is absent from the map"
    );
    assert!(
        Executed::default().never(),
        "§7b.10's `executed: never` — the empty entry is a fact about the store, not a defaulted \
         policy value"
    );

    // ---- (d) THE DISCRIMINATING CASE: journal order vs `at` order -----------------------------
    // No record says which `"first"` means. This build takes JOURNAL ORDER, because ADR-0114's
    // "on the first and every subsequent `activity_committed`" is a statement about the sequence
    // of EVENTS, and `at` is a value the activity returned that this crate does not mint. The case
    // is here rather than left to be discovered: on a well-behaved journal the two rules agree.
    let backwards = vec![
        commit(A, "run-late", 900, 1),
        commit(A, "run-early", 100, 1),
    ];
    let led3 = fold_executed(&backwards).expect("well-formed");
    let a3 = led3.get(A).expect("A");
    assert_eq!(
        a3.first.as_ref().expect("A").run_id,
        "run-late",
        "`first` is the first row in APPEND order, even though its `at` is the later one — the \
         reading is stated in `src/runtime/ledger.rs`'s header and a record settling it is OWED"
    );
    assert_eq!(a3.last.as_ref().expect("A").run_id, "run-early");

    // ---- (e) the four refusals ----------------------------------------------------------------
    for member in ["activity_id_at_version", "item", "run_id", "at"] {
        let mut row = commit(A, "run-1", 100, 1);
        let obj = row.payload.as_object_mut().expect("an object payload");
        assert!(
            obj.remove(member).is_some(),
            "PREMISE: the runtime's own writer emits `{member}`, so removing it is a real \
             mutilation and not a no-op"
        );
        let got = fold_executed(&[row]).expect_err("a row missing a required member is refused");
        assert_eq!(got.as_str(), "malformed_payload");
        match got {
            LedgerError::MalformedPayload { member: m, .. } => assert_eq!(
                m, member,
                "the refusal NAMES the missing member — a bare `malformed` would not say which of \
                 the four had no lawful default"
            ),
        }
    }

    // An `at` of the wrong TYPE is refused too, not coerced: a string timestamp silently read as 0
    // is the epoch, and the epoch is a value the kernel would have minted.
    let mut wrong = commit(A, "run-1", 100, 1);
    wrong
        .payload
        .as_object_mut()
        .expect("object")
        .insert("at".to_owned(), serde_json::json!("2026-09-14"));
    assert_eq!(
        fold_executed(&[wrong])
            .expect_err("a non-integer `at` is refused")
            .as_str(),
        "malformed_payload"
    );

    // ---- a row of another kind is not a commit and is not folded ------------------------------
    let noise = vec![
        Event::new(EventKind::Dispatch, serde_json::json!({"item": ITEM})),
        commit(A, "run-1", 100, 1),
    ];
    let led4 = fold_executed(&noise).expect("the dispatch row is skipped, not refused");
    assert_eq!(
        led4.get(A).expect("A").executions(),
        1,
        "only `activity_committed` rows are executions — a fold that counted dispatches would \
         report a retried step as two"
    );
}
