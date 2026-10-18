//! **The boundary hold** — `src/workflow/hold.rs` and `fsm::next_action`'s step 5 (`s8j` §8j.3;
//! ADR-0201 decisions 2 and 3, proposed). *(Added 2026-10-18, `s8j` build, package S8J-C3-HOLD.)*
//!
//! A minimal pack whose `analysis` stage selects nothing, so its plan is exhausted on an empty
//! journal and step 5 is reached at once with the successor `specification` supplied. Each test
//! folds a journal and asks the FSM for its next action with the hold supplied (or not) beside the
//! successor, the route the composition root takes (ADR-0153).

use rk_kernel::runtime::gate_opened_event;
use rk_kernel::select::{ArtifactRegistry, Catalog, Item, Pack, Selection, select};
use rk_kernel::workflow::fsm::{FsmContext, HaltReason, NextAction, fold, next_action};
use rk_kernel::workflow::gate::{ExpiryAction, gate_answered_event, gate_expired_event};
use rk_types::event::Event;
use rk_types::time::Timestamp;

const ITEM: &str = "BUG-1";
const STAGE: &str = "analysis";
const HELD: &str = "specification";
const TREE_A: &str = "aaaa111122223333444455556666777788889999";
const TREE_B: &str = "bbbb111122223333444455556666777788889999";
const DEADLINE: i64 = 1_800_000_000;

const PACK: &str = r#"[pack]
id = "hold-fixture"
applies = "kind == bug"
appetite = { points = 50 }

[policy]
stages = { triage = "mandatory", discovery = "forbidden", analysis = "recommended", specification = "recommended", design = "forbidden", planning = "forbidden", implement = "recommended", verify = "recommended", review = "mandatory", release = "mandatory", operate = "mandatory", closeout = "mandatory" }
"#;

const ARTIFACTS: &str = r#"[artifact.ReproCase]
source = "activity"
schema = "ReproCase"
"#;

/// An activity in another stage, so the catalog is not empty and `analysis` still selects nothing.
const REPRO: &str = r#"[activity]
id           = "repro"
version      = 1
stage        = "implement"
provides     = ["ReproCase"]
requires     = []
tags         = ["kind:bug"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"

[[step]]
id      = "write"
kind    = "agent"
outputs = "value:ReproCase"

[[step]]
id      = "emit"
kind    = "emit"
"#;

/// The fixture's selection and the FSM's answer over it (TST-14: a test helper module).
#[cfg(test)]
mod fixture {
    use super::{
        ARTIFACTS, ArtifactRegistry, Catalog, DEADLINE, Event, FsmContext, HELD, ITEM, Item,
        NextAction, PACK, Pack, REPRO, STAGE, Selection, TREE_A, Timestamp, fold, next_action,
        select,
    };

    fn facets() -> rk_kernel::registry::Registry {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        rk_kernel::registry::Registry::from_toml(&src)
            .expect("the shipped facet registry must load")
    }

    /// The `analysis` selection: nothing selected, no gate required.
    pub(super) fn selection() -> Selection {
        let pack = Pack::from_toml("hold-fixture.toml", PACK).expect("the fixture pack loads");
        let catalog =
            Catalog::from_sources(&[("repro.toml", REPRO)]).expect("the fixture catalog loads");
        let artifacts = ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("it loads");
        let item = Item::new(ITEM, "bug");
        select(&pack, &catalog, &artifacts, &facets(), STAGE, &item)
            .expect("the selection resolves")
    }

    /// The next action over `journal`, the successor `specification` supplied and `hold` beside it.
    pub(super) fn act(journal: &[Event], hold: Option<&str>) -> NextAction {
        let selection = selection();
        let state = fold(ITEM, STAGE, journal).expect("the journal folds");
        let ctx = FsmContext::supplied(
            ITEM,
            TREE_A,
            Timestamp::from_unix_seconds(DEADLINE),
            Some(HELD),
        );
        let ctx = hold.map_or(ctx, |stage| ctx.with_hold_before(stage));
        next_action(&selection, &state, &ctx).expect("the next action computes")
    }

    /// The advance the FSM takes when nothing holds it.
    pub(super) fn advance() -> NextAction {
        NextAction::AdvanceStage {
            from: STAGE.to_owned(),
            to: HELD.to_owned(),
        }
    }

    /// The hold's gate, opened on a fresh journal (asserted, so a caller never builds a journal
    /// over some other action).
    pub(super) fn opened_hold() -> rk_kernel::workflow::fsm::GateRecord {
        match act(&[], Some(HELD)) {
            NextAction::OpenGate(record) => record,
            other => {
                panic!("a fresh journal with the hold supplied must open its gate: {other:?}")
            }
        }
    }
}

use fixture::{act, advance, opened_hold, selection};

/// **ADR-0201 decision 2: the advance into the held stage opens `hold:<stage>` instead.** The
/// controls: with no hold, and with a hold on a stage the successor is not, the same journal
/// advances, so the gate is the hold's and only the held boundary's. Kills `M-hold-ignored`.
#[test]
fn a_held_advance_opens_the_hold_gate_instead_of_advancing() {
    assert!(
        selection().gates().is_empty() && selection().unsatisfiable().is_empty(),
        "PREMISE: the stage requires no gate, so step 5 is reached"
    );
    assert_eq!(
        act(&[], None),
        advance(),
        "PREMISE: unheld, the stage advances"
    );
    assert_eq!(
        act(&[], Some("design")),
        advance(),
        "a hold on another stage does not hold this boundary"
    );

    let record = opened_hold();
    assert_eq!(record.kind(), "hold:specification");
    assert_eq!(record.kind(), rk_kernel::workflow::hold::SPECIFICATION_KIND);
    assert_eq!(record.reach(), 1);
    assert_eq!(record.tree_hash(), TREE_A);

    // Once opened and unanswered, step 1 awaits it: the run stops at the gate.
    let opened = gate_opened_event(&record);
    assert_eq!(
        act(std::slice::from_ref(&opened), Some(HELD)),
        NextAction::AwaitGate {
            token: record.token().clone()
        }
    );
}

/// **ADR-0201 decision 3: an answered hold is spent and never re-fires; `approve` releases the
/// advance.** The answer is folded from the journal, so every later process sees it; a second
/// answer to the spent token moves nothing. Kills `M-hold-spent-refires`.
#[test]
fn an_approved_hold_is_spent_releases_the_advance_and_never_refires() {
    let record = opened_hold();
    let opened = gate_opened_event(&record);
    let approved = gate_answered_event(record.token(), TREE_A, "approve");
    let journal = [opened.clone(), approved.clone()];
    assert_eq!(
        act(&journal, Some(HELD)),
        advance(),
        "an approved hold releases the advance and opens no second gate"
    );

    // A second answer (spent, ADR-0123) changes nothing: still the advance, still no re-open.
    let again = gate_answered_event(record.token(), TREE_A, "changes-requested");
    assert_eq!(act(&[opened, approved, again], Some(HELD)), advance());
}

/// **ADR-0201 decision 3: `changes-requested` stands and holds the lane at the boundary.** No
/// advance and no second gate, on this call and any later one over the same journal.
#[test]
fn a_changes_requested_hold_stands_and_does_not_advance() {
    let record = opened_hold();
    let journal = [
        gate_opened_event(&record),
        gate_answered_event(record.token(), TREE_A, "changes-requested"),
    ];
    assert_eq!(
        act(&journal, Some(HELD)),
        NextAction::Done,
        "a standing `changes-requested` neither advances nor re-opens the hold"
    );
    let standing = {
        let state = fold(ITEM, STAGE, &journal).expect("folds");
        rk_kernel::workflow::hold::standing(&state, rk_kernel::workflow::hold::SPECIFICATION_KIND)
            .map(|a| a.verdict().to_owned())
    };
    assert_eq!(standing.as_deref(), Some("changes-requested"));
}

/// **The hold's gate is a gate like any other** (ADR-0201 decision 3's last three bullets): a void
/// answer (AC-039) is not one that stood and is followed by a fresh re-open at reach 2 (AC-252); an
/// expired hold under `fail` halts the item at the boundary (ADR-0198 decision 6).
#[test]
fn a_void_hold_answer_reopens_fresh_and_an_expired_hold_fails() {
    let record = opened_hold();
    let opened = gate_opened_event(&record);
    let stale = gate_answered_event(record.token(), TREE_B, "approve");
    let NextAction::OpenGate(again) = act(&[opened.clone(), stale], Some(HELD)) else {
        panic!("a voided hold must re-open, not advance");
    };
    assert_eq!(again.kind(), "hold:specification");
    assert_eq!(again.reach(), 2);
    assert_ne!(again.token(), record.token());

    let expired = gate_expired_event(record.token(), &ExpiryAction::Fail, None);
    assert_eq!(
        act(&[opened, expired], Some(HELD)),
        NextAction::Halt {
            reason: HaltReason::GateExpired {
                token: record.token().clone()
            }
        },
        "an expired hold fails and does not advance"
    );
}
