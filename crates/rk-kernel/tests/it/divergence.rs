//! **AC-007 — the typed replay divergence.** `src/workflow/fsm.rs`'s `Divergence`. Slice 4e.
//!
//! > **AC-007** — `rk-kernel::replay` · evidence: F19 [measured], principle 4 *"absence of evidence
//! > is a block"* [documented] · decisions: ADR-0022
//! > GIVEN a replay whose recomputed decision differs from the recorded one
//! > WHEN the divergence is detected
//! > THEN the run halts with a typed `ReplayDivergence { activity_id, recorded, recomputed }`, no
//! > new action is taken, and no continuation on the recomputed value is reachable.
//! > *Verified by:* a unit test that mutates one recorded decision and asserts the halt; an
//! > exhaustive `match` on the divergence outcome with no wildcard arm, and a `trybuild` test
//! > proving the wildcard cannot be reintroduced.
//!
//! **Three obligations, and all three are here**, two of them at compile time:
//!
//! 1. the unit test below — a journal with one decision changed, folded, and the halt asserted with
//!    all three fields populated;
//! 2. the **no-wildcard `match`** inside `Divergence::halt` — a type fact, asserted by the fourth
//!    variant being an `E0004` there and by the sweep below over `Divergence::ALL_TAGS`;
//! 3. `tests/ui/ac007_divergence_match_has_no_wildcard.rs`, asserted **by path** at the end of this
//!    test, because §7b.4's six-row case table and §7b.14's by-path loop (:1580-1589) each enumerate
//!    the same six files and **this one is not among them** — so a deletion would be caught by
//!    nothing in Checkpoint 7b. The by-path assertion catches a deletion; it does **not** catch
//!    `.stderr` drift, which only `trybuild` itself catches.
//!
//! # 🛑 WHAT IS MEASURED IS THE HALT, NOT THE DETECTION
//!
//! AC-007's GIVEN is *"a replay whose recomputed decision differs from the recorded one"*. This
//! slice builds the **FSM** that recomputes and the **typed halt** that the difference produces;
//! `src/replay.rs` gains the two comparison steps that DETECT it at Slice **4f**
//! (`ReplayError::NextActionDiverged` / `TerminalStateDiverged`). The divergence values below are
//! therefore built from **two real folds of two real journals** — never hand-written strings — so
//! that what is asserted is a difference the FSM actually produces. That is as far as this slice
//! reaches, and the header of `src/workflow/fsm.rs` says so in its ARMED table.
//!
//! 🛑 **AND THE DETECTION IS PINNED ELSEWHERE, BY NAME — added 2026-09-14 on an adversarial
//! verification pass that reproduced the gap.** `ac007_replay_divergence_halts` **constructs** the
//! `Divergence` itself and calls `.halt()`, so **both production guards can be dead while this test
//! is green**: patched to `if false && …`, the unfeatured suite reported **83 passed, 0 failed**
//! [measured 2026-09-14]. A reader must not take this file's green as evidence that a divergence is
//! ever *noticed*. The controls that do notice are named here so the claim is checkable:
//!
//! | what pins the DETECTION | where | run by |
//! | --- | --- | --- |
//! | `ac007_divergence_detection_over_the_committed_log` | `tests/it/recorded_log.rs` | the **unfeatured** suite — Checkpoint 7b's third loop runs it |
//! | `the_replay_still_calls_the_divergence_detection` | `tests/it/recorded_log.rs` | the unfeatured suite; a source lint, because a guard nothing invokes is the same defect |
//! | `golden_log_replay_next_action_mismatch_is_red` | `tests/it/replay.rs` | `--features corpus` only |
//! | `golden_log_replay_terminal_state_mismatch_is_red` | `tests/it/replay.rs` | `--features corpus` only |
//!
//! The first two exist **because** the last two were the only ones: `src/replay.rs`'s
//! `compare_recorded_residue` was extracted out of `replay()` on that pass so the comparison could
//! be reached without the corpus feature, without `git` and without a `ReplayPorts`.

use rk_kernel::workflow::fsm::{Divergence, TerminalState, fold};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

const ITEM: &str = "INC-1";
const STAGE: &str = "implement";
const ACTIVITY: &str = "impl-report";

fn activation(kind: EventKind, activity: &str, attempt: i64) -> Event {
    Event::new(
        kind,
        serde_json::json!({
            "item": ITEM,
            "activity_id_at_version": activity,
            "attempt": attempt,
        }),
    )
}

/// **AC-007.** A recorded decision is mutated; the run halts with the typed
/// `ReplayDivergence { activity_id, recorded, recomputed }`, and the recomputed value is **moved
/// into the halt** so no continuation on it is reachable.
#[test]
fn ac007_replay_divergence_halts() {
    // ---- the two journals differ in EXACTLY ONE decision --------------------------------------
    //
    // Both dispatch the same activation. In the recorded one it committed; in the recomputed one
    // it did not. That is one decision's worth of difference and nothing else.
    let recorded_journal = vec![
        activation(EventKind::Dispatch, ACTIVITY, 1),
        activation(EventKind::ActivityCommitted, ACTIVITY, 1),
    ];
    let recomputed_journal = vec![activation(EventKind::Dispatch, ACTIVITY, 1)];
    assert_eq!(
        recorded_journal.len() - recomputed_journal.len(),
        1,
        "PREMISE: exactly one event distinguishes the two journals"
    );

    let recorded_state = fold(ITEM, STAGE, &recorded_journal).expect("the journal folds");
    let recomputed_state = fold(ITEM, STAGE, &recomputed_journal).expect("the journal folds");
    assert_eq!(
        recorded_state.terminal(),
        TerminalState::StageComplete {
            stage: STAGE.to_owned()
        }
    );
    assert_eq!(
        recomputed_state.terminal(),
        TerminalState::AwaitingDispatch {
            stage: STAGE.to_owned()
        }
    );

    let recorded = String::from_utf8(
        recorded_state
            .terminal()
            .canonical_bytes()
            .expect("canonical"),
    )
    .expect("the canonical serialiser emits UTF-8");
    let recomputed = String::from_utf8(
        recomputed_state
            .terminal()
            .canonical_bytes()
            .expect("canonical"),
    )
    .expect("utf-8");
    // PREMISE: the mutation really is visible under the canonical serialiser. Without this the
    // halt below is constructed from two equal strings and asserts nothing about the FSM.
    assert_ne!(
        recorded, recomputed,
        "one changed decision must change the canonical terminal bytes, or there is nothing for a \
         replay to diverge on"
    );

    // ---- THE HALT: all three fields populated, and the recomputed value MOVED IN ---------------
    let divergence = Divergence::TerminalState {
        recorded: recorded.clone(),
        recomputed: recomputed.clone(),
    };
    // `halt` takes `self` BY VALUE. After this line `divergence` is gone: there is no path from
    // the returned value back to a `RunState`, a `NextAction` or the divergence itself, which is
    // AC-007's *"no continuation on the recomputed value is reachable"* expressed in the type
    // rather than in a comment. A `&self` signature would leave the caller holding the divergence
    // and free to carry on.
    let halt = divergence.halt(ACTIVITY);
    assert_eq!(halt.activity_id, ACTIVITY);
    assert_eq!(halt.recorded, recorded);
    assert_eq!(halt.recomputed, recomputed);
    for (field, value) in [
        ("activity_id", &halt.activity_id),
        ("recorded", &halt.recorded),
        ("recomputed", &halt.recomputed),
    ] {
        assert!(
            !value.is_empty(),
            "AC-007 names three fields and all three must be populated; `{field}` is empty"
        );
    }
    assert_ne!(
        halt.recorded, halt.recomputed,
        "a halt whose two sides are equal is not a divergence"
    );

    // CONTROL, and it is the discriminating one: two byte-identical journals produce equal
    // canonical bytes, so there is nothing to halt on. Without it every assertion above is
    // satisfied by a fold that returns a different value every time it is called — which would
    // make every replay diverge and AC-006's byte-identity impossible.
    let again = fold(ITEM, STAGE, &recorded_journal).expect("folds");
    assert_eq!(
        recorded,
        String::from_utf8(again.terminal().canonical_bytes().expect("canonical")).expect("utf-8"),
        "the same journal folds to the same terminal bytes — the comparison discriminates because \
         the fold is deterministic, not because it is unstable"
    );

    // ---- the three variants, swept, with the fourth one an `E0004` in `halt` ------------------
    assert_eq!(Divergence::ALL_TAGS.len(), 3);
    let all = vec![
        Divergence::Decision {
            index: 7,
            recorded: recorded.clone(),
            recomputed: recomputed.clone(),
        },
        Divergence::NextAction {
            recorded: recorded.clone(),
            recomputed: recomputed.clone(),
        },
        Divergence::TerminalState {
            recorded,
            recomputed,
        },
    ];
    assert_eq!(all.len(), Divergence::ALL_TAGS.len());
    for (i, d) in all.iter().enumerate() {
        assert_eq!(d.as_str(), Divergence::ALL_TAGS[i]);
    }
    // Every variant halts, and every halt carries all three fields.
    for d in all {
        let tag = d.as_str();
        let indexed = matches!(d, Divergence::Decision { .. });
        let halt = d.halt(ACTIVITY);
        assert!(!halt.recorded.is_empty(), "{tag}: recorded");
        assert!(!halt.recomputed.is_empty(), "{tag}: recomputed");
        if indexed {
            // Which decision diverged is part of the ID, not a fourth field: ADR-0097's forbids
            // clause on a fourth part of a key is the house instinct, and `ReplayDivergence` has
            // exactly three fields.
            assert_eq!(halt.activity_id, format!("{ACTIVITY}#7"));
        } else {
            assert_eq!(halt.activity_id, ACTIVITY);
        }
    }

    // ---- obligation 3, asserted by path because no checkpoint line asserts it ------------------
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    for f in [
        "ac007_divergence_match_has_no_wildcard.rs",
        "ac007_divergence_match_has_no_wildcard.stderr",
    ] {
        let p = ui.join(f);
        assert!(
            p.is_file(),
            "AC-007's third instrument is a `trybuild` case, and §7b.14's by-path loop does not \
             enumerate it — so this assertion is the only thing that notices {} going missing",
            p.display()
        );
        assert!(
            std::fs::metadata(&p).expect("readable").len() > 0,
            "{} is empty, so the case would be green whenever compilation fails for ANY reason",
            p.display()
        );
    }
}
