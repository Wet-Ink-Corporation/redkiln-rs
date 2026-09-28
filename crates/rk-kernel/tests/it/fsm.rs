//! **The FSM** — `src/workflow/fsm.rs`. Slice 4e.
//!
//! Three tests, and what each is for — the third replaced a **pin on a known defect** with the
//! repair, on the operator decision of 2026-09-14:
//!
//! | test | criteria | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac039_gate_record_and_the_void_answer`] | **AC-039** (HUMAN GATE) | the *mechanism* residue only: the gate record bound to `(token, tree_hash, deadline)`, the derived token, and *"an answer whose tree hash no longer matches is **void, not reconciled**"* | **never the answer.** §7b.15 excludes AC-039 from the claim and this file may not restore it |
//! | [`ac008_terminal_state_and_the_next_action_grains`] | AC-008, AC-006 (3), AC-161, AC-164 — all **ARMED** | the *recomputable* side: the five terminal states, the six next actions in their fixed order, and the tier-1 filter | **the recorded side**, which is `run.json`'s and Slice **4f**'s; and the **route** from git trailers, which is 7d's |
//! | [`gate_answered_after_expiry_is_void_and_the_fold_continues`] | **none** — no criterion covers it | that `gate_answered` is **three** cases: open → answered, **expired → void and the fold CONTINUES**, never seen → `GateAnsweredWithoutOpen`. The warrant is exactly this wide: **the arm conflated two situations and only one is forgery** | **whether a voided gate RE-OPENS** — gate-record semantics, assigned to **7b-ii** by AC-176's Boundary note. It supersedes `gate_answered_after_expiry_is_unfoldable_known_defect`, the pin that held the `permanently unfoldable` journal as a DEFECT until the decision landed; **a durable record naming that decision is still OWED**, so the name carries no AC id |
//!
//! 🛑 **A green here is not AC-008 discharged, and the name says so deliberately.** AC-008's
//! comparison has two sides — *"the replay reaches **the recorded** terminal state"* — and this
//! slice builds only the side that is recomputed. The recorded side is a value written into
//! `run.json` at record time, which `src/replay.rs` gains at Slice 4f. The name here carries `ac008`
//! because the criterion is what the test is *about*; the table above is what says how much of it is
//! measured. The same is true of AC-006 assertion 3, AC-161 and AC-164.
//!
//! The extra name is authorised: residual 10a is FORCED — *"the ten [runbook] names are a floor, not
//! a ceiling"* — and `00-preamble.md`:188-189 requires a discriminating test for every criterion in
//! the acceptance set. Neither name contains the substring `golden_log_replay` or
//! `ac161_ac164_delete_db_replay`, which the `--list` differential at :1345-1346 counts without a
//! word anchor.

use rk_kernel::select::{ArtifactRegistry, Catalog, GateReason, Item, Pack, select};
use rk_kernel::workflow::fsm::{
    FsmContext, FsmError, GateRecord, HaltReason, NextAction, StageAction, TerminalState, fold,
    next_action, stage_action,
};
use rk_types::event::Event;
use rk_types::event_kind::{EventKind, Tier};
use rk_types::time::Timestamp;

const ITEM: &str = "INC-1";
const STAGE: &str = "implement";
const ACTIVITY: &str = "impl-report";

/// The tree hash AC-039's answer is bound to. **An argument**: ADR-0105 clause 4 owns the one
/// `tree_hash` algorithm and it is `rk-git`'s, so a value computed in this crate would be a second
/// derivation that clause's own forbids bullet bans.
const TREE_A: &str = "aaaa111122223333444455556666777788889999";
/// A *different* tree hash — the item moved under the operator while the gate was open.
const TREE_B: &str = "bbbb111122223333444455556666777788889999";

const INCIDENT: &str = include_str!("../fixtures/packs/incident-forbidden.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../fixtures/catalog-select/spec-authoring.toml"),
    ),
];

fn facets() -> rk_kernel::registry::Registry {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry must load")
}

fn parts() -> (
    Pack,
    Catalog,
    ArtifactRegistry,
    rk_kernel::registry::Registry,
) {
    (
        Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("the fixture pack loads"),
        Catalog::from_sources(CATALOG).expect("the fixture catalog loads"),
        ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the registry loads"),
        facets(),
    )
}

/// A selection whose plan is **not empty** — `implement` over a mixed item. `execute.rs` already
/// asserts that this plan has more than one layer, so the dispatch order below is a real order.
pub fn planned() -> rk_kernel::select::Selection {
    let (pack, catalog, artifacts, facets) = parts();
    let item = Item::new(ITEM, "incident").with_toolchain("node", ["text"]);
    select(&pack, &catalog, &artifacts, &facets, STAGE, &item).expect("selection resolves")
}

/// A selection carrying **exactly one** `gate_opened { reason: unknown_facet, axis }` — the
/// abstaining-facet case `selection.rs`'s `unknown_facet_abstains_and_is_not_unsatisfiable` pins.
pub fn gated() -> rk_kernel::select::Selection {
    let (pack, catalog, artifacts, facets) = parts();
    let item = Item::new(ITEM, "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_null("external_dep");
    select(&pack, &catalog, &artifacts, &facets, "analysis", &item).expect("selection resolves")
}

fn gate_opened(kind: &str, tree_hash: &str, deadline: i64) -> Event {
    Event::new(
        EventKind::GateOpened,
        serde_json::json!({ "kind": kind, "tree_hash": tree_hash, "deadline": deadline }),
    )
}

fn gate_answered(token: &str, tree_hash: &str, verdict: &str) -> Event {
    Event::new(
        EventKind::GateAnswered,
        serde_json::json!({ "token": token, "tree_hash": tree_hash, "verdict": verdict }),
    )
}

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

fn transition(from: &str, to: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": from, "to": to }),
    )
}

/// **AC-039 — the machine residue, and nothing above it.**
///
/// > GIVEN a run halted by an exhausted wall-clock or token budget / WHEN the operator decides
/// > whether to extend the budget or abandon the item / THEN the decision arrives as a gate answer
/// > bound to `(token, tree_hash, deadline)` and the kernel takes no action until it does.
/// > *Verified by:* routed to a **human gate**; **not machine-decidable** … The *mechanism* is
/// > machine-checked (a test asserting the halt writes a gate record, and that **an answer whose
/// > tree hash no longer matches is void, not reconciled**), but the answer is not.
///
/// **HUMAN GATE.** §7b.15: *"minus AC-039, human-gated and not machine-decidable; only the mechanism
/// half is asserted here, never the answer."* Nothing below asserts, or names, an extend/abandon
/// decision.
#[test]
fn ac039_gate_record_and_the_void_answer() {
    // ---- (a) the record is BOUND to all three, and the token is DERIVED from them ------------
    //
    // PREMISE: the three inputs are three genuinely different values, so an implementation that
    // derived the token from one of them and ignored the other two is distinguishable from one
    // that reads all three.
    assert_ne!(TREE_A, TREE_B, "the two tree hashes must differ");
    let base = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(1_700),
    )
    .expect("no member of a gate record is a float");

    assert_eq!(base.kind(), "budget_extension");
    assert_eq!(base.tree_hash(), TREE_A);
    assert_eq!(base.deadline(), Timestamp::from_unix_seconds(1_700));
    assert_eq!(
        base.token().as_str().len(),
        64,
        "the token is a SHA-256 digest, not a hand-written string: {}",
        base.token().as_str()
    );
    assert_ne!(
        base.token().as_str(),
        TREE_A,
        "the token is DERIVED FROM the tree hash and is not a copy of it — an implementation that \
         aliased the two would satisfy every equality below and lose the binding AC-039 names"
    );

    // Move each of the three in turn, holding the other two fixed. All three must move the token,
    // or the binding is to fewer than three things.
    let moved_kind = GateRecord::derive("abandon", TREE_A, Timestamp::from_unix_seconds(1_700))
        .expect("derives");
    let moved_tree = GateRecord::derive(
        "budget_extension",
        TREE_B,
        Timestamp::from_unix_seconds(1_700),
    )
    .expect("derives");
    let moved_deadline = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(1_701),
    )
    .expect("derives");
    for (label, other) in [
        ("kind", &moved_kind),
        ("tree_hash", &moved_tree),
        ("deadline", &moved_deadline),
    ] {
        assert_ne!(
            base.token(),
            other.token(),
            "moving `{label}` alone must move the derived token, or the gate is not bound to it"
        );
    }
    // CONTROL: all three identical reproduces the token exactly — without this the three
    // inequalities above are satisfied by a token drawn at random, which AC-002 bans outright.
    let again = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(1_700),
    )
    .expect("derives");
    assert_eq!(
        base, again,
        "the same three inputs derive the same record, byte for byte — the token is a CONTENT \
         HASH, which is the escape AC-002's ban on minted ids names"
    );

    // ---- (b) "the halt WRITES a gate record", and the kernel takes no action until answered ---
    let opened = gate_opened("budget_extension", TREE_A, 1_700);
    let state = fold(ITEM, STAGE, std::slice::from_ref(&opened)).expect("the journal folds");
    let open: Vec<&GateRecord> = state.open_gates().collect();
    assert_eq!(open.len(), 1, "one recorded gate, one record: {open:?}");
    assert_eq!(
        open[0], &base,
        "the record the fold derives from the journal is the record `derive` builds from the same \
         three values"
    );
    assert_eq!(
        state.terminal(),
        TerminalState::AwaitingGate {
            token: base.token().clone()
        }
    );
    assert_eq!(
        next_action(
            &planned(),
            &state,
            &FsmContext::supplied(
                ITEM,
                TREE_A,
                Timestamp::from_unix_seconds(1_700),
                Some("verify")
            )
        )
        .expect("the next action computes"),
        NextAction::AwaitGate {
            token: base.token().clone()
        },
        "ADR-0022: the kernel takes NO action until the gate is answered — not a dispatch, and not \
         a stage advance, even with a plan to run and a successor supplied"
    );

    // A forged `token` member in the journal is NOT read: the fold derives the token from the
    // payload it can verify, so a recorded token cannot steer the state.
    let forged = Event::new(
        EventKind::GateOpened,
        serde_json::json!({
            "kind": "budget_extension",
            "tree_hash": TREE_A,
            "deadline": 1_700,
            "token": "0".repeat(64),
        }),
    );
    let forged_state = fold(ITEM, STAGE, &[forged]).expect("folds");
    let forged_open: Vec<&GateRecord> = forged_state.open_gates().collect();
    assert_eq!(
        forged_open[0].token(),
        base.token(),
        "a `token` member in the payload is ignored; the derived one wins"
    );

    // ---- (c) THE VOID RULE, and its control ---------------------------------------------------
    //
    // An answer whose tree hash no longer matches is **void, NOT reconciled**.
    let stale = fold(
        ITEM,
        STAGE,
        &[
            gate_opened("budget_extension", TREE_A, 1_700),
            gate_answered(base.token().as_str(), TREE_B, "extend"),
        ],
    )
    .expect("a stale answer is not a fold error — it is a void answer");
    let void: Vec<&rk_types::digest::Digest> = stale.void_gates().collect();
    assert_eq!(
        void,
        vec![base.token()],
        "the stale answer's token is VOID: {void:?}"
    );
    assert_eq!(
        stale.answered_gates().count(),
        0,
        "and it was NOT reconciled into the answered set — there is no path in this module that \
         moves a token out of the void set, which is what `not reconciled` means"
    );
    // CORRECTED 2026-09-17 at Step 7b-ii (AC-252). This asserted `TerminalState::AwaitingGate {
    // token: base.token() }` — that the run holds at the VOID token. The register says otherwise:
    // AC-252, *"GIVEN an answer that resolved void / WHEN the kernel next acts on the item / THEN a
    // new gate is opened whose record differs from the voided gate's in all three fields"*, and
    // its replay assertion *"a void is always followed by either a re-open or a terminal state,
    // never by nothing"*. A void token is TERMINAL for that token: `awaiting_gate` no longer
    // returns it, and `next_action` re-opens (pinned by
    // `tests/it/gates.rs::ac252_void_reopens_fresh_in_all_three_fields`). What AC-039's `void,
    // not reconciled` requires is preserved and asserted here: the void set holds the token, the
    // answered set does not, and NO decision was applied to the item. The register outranks the
    // reading this test took on 2026-09-14 (ADR-0092).
    assert_eq!(
        stale.awaiting_gate(),
        None,
        "a void token is terminal: the run does not hold at AwaitingGate on a dead token (AC-252)"
    );
    assert_ne!(
        stale.terminal(),
        TerminalState::AwaitingGate {
            token: base.token().clone()
        },
        "…and the terminal projection does not report the voided gate as still awaited"
    );
    assert!(
        stale.committed().next().is_none() && stale.stages_entered().is_empty(),
        "a void answer applied no decision: nothing committed, no stage entered — a runtime that \
         RECONCILED the stale answer would have moved one of these, which is the exact failure \
         AC-039's `void, not reconciled` names"
    );

    // THE CONTROL, and it is the discriminating half: the same answer against the SAME tree hash
    // is accepted. Without it every assertion above is satisfied by a fold that voids every
    // answer it ever sees.
    let matched = fold(
        ITEM,
        STAGE,
        &[
            gate_opened("budget_extension", TREE_A, 1_700),
            gate_answered(base.token().as_str(), TREE_A, "extend"),
        ],
    )
    .expect("folds");
    assert_eq!(matched.void_gates().count(), 0, "nothing is void here");
    let answered: Vec<&str> = matched
        .answered_gates()
        .map(rk_kernel::workflow::fsm::GateAnswer::verdict)
        .collect();
    assert_eq!(answered, vec!["extend"]);
    assert_ne!(
        matched.terminal(),
        TerminalState::AwaitingGate {
            token: base.token().clone()
        },
        "an answer at the matching tree hash releases the gate"
    );

    // ---- (d) an answer naming a token no derived gate carries is a TYPED refusal --------------
    let orphan = fold(
        ITEM,
        STAGE,
        &[gate_answered(&"c".repeat(64), TREE_A, "extend")],
    );
    match orphan {
        Err(FsmError::GateAnsweredWithoutOpen { token }) => {
            assert_eq!(token.as_str(), "c".repeat(64));
        }
        other => panic!(
            "an answer against no open gate must be a typed refusal, never a silently dropped \
             answer: {other:?}"
        ),
    }
    assert_eq!(
        FsmError::GateAnsweredWithoutOpen {
            token: base.token().clone()
        }
        .as_str(),
        "gate_answered_without_open"
    );

    // ---- (e) the ARMED pin, so the gap cannot close quietly -----------------------------------
    //
    // AC-039's gate is `gate_opened`-carried, and `EventKind` is closed at 29. If a thirtieth kind
    // is ever minted this goes RED here and whoever adds it must decide whether it is the
    // budget-extension gate's carrier — rather than the question never being asked.
    assert_eq!(
        EventKind::ALL.len(),
        41,
        // DECIDED HERE 2026-09-17 (ADR-0123, 29 -> 31; ADR-0124, 31 -> 36 — the review and verify
        // gates' five rows; ADR-0125, 36 -> 37 — `stage_exit_refused`, which opens no gate either:
        // AC-357's gate is what FOLLOWS a refusal): `gate_answer_spent` records a refused
        // second answer and `gate_resumed` records a resume. Neither opens a gate, so the
        // budget-extension gate's carrier is still `gate_opened` with kind `budget_extension`.
        // DECIDED AGAIN 2026-09-17 (Step 7c Slice 1; ADR-0126, 37 -> 38): `dispatch_completed` is
        // the adapter's usage-bearing row, tier-2 telemetry about a dispatch that already
        // terminated (decision 4). It opens no gate — the FSM folds it in its no-state-change
        // arm (decision 5), which `tests/it/dispatch_completed.rs::
        // adr0126_dispatch_completed_moves_nothing_in_the_fsm` asserts over a fold rather than
        // states here — so the budget-extension gate's carrier is still `gate_opened`.
        // DECIDED AGAIN 2026-09-28 (`s8e` build, package S8E-P5; the three-kinds record, `s8e` §8e.4, proposed; 38 -> 41): `worktree_assigned`, `stage_empty` and `git_op` are tier-2 rows, and none
        // opens a gate: the FSM folds all three in its no-state-change arm, so the budget-extension
        // gate's carrier is still `gate_opened`.
        "EventKind is closed at 41 (ADR-0116: `the kind is minted by record or not at all`; \n         ADR-0123 minted the two beyond 29, ADR-0124 the five beyond 31, ADR-0125 the one beyond 36, ADR-0126 the one beyond 37, s8e's three-kinds record the three beyond 38)"
    );
}

/// **AC-008's recomputable half, AC-006 assertion 3's, and AC-161/AC-164's stage grain.**
///
/// > **AC-008** … THEN the replay reaches **the recorded terminal state** … *so … a kernel that does
/// > nothing at all fails the terminal-state half.*
/// >
/// > **AC-161** … THEN the **stage-level next action** — the FSM's next stage transition and the
/// > gate records the trailers carry — is equal, under the canonical serialiser, to the one
/// > selected before the deletion … *The comparison is at stage grain; a step-grain comparison
/// > would assert what tier 2 cannot promise.*
///
/// **All four ids are ARMED, not discharged, and this test does not claim otherwise** — see the
/// module header's table. What is measured is that the FSM the runbook twice recorded as
/// **NOT BUILDABLE** now exists and discriminates.
#[test]
fn ac008_terminal_state_and_the_next_action_grains() {
    // ---- (a) THE DO-NOTHING CONTROL — AC-008's own named failure mode -------------------------
    let nothing = fold(ITEM, STAGE, &[]).expect("an empty journal folds");
    assert_eq!(nothing.terminal(), TerminalState::NotStarted);
    assert_eq!(nothing.survey().events_folded(), 0);
    assert_eq!(nothing.survey().state_moving(), 0);

    let real = fold(
        ITEM,
        STAGE,
        &[
            transition("analysis", STAGE),
            activation(EventKind::Dispatch, ACTIVITY, 1),
        ],
    )
    .expect("folds");
    assert_ne!(
        nothing.terminal().canonical_bytes().expect("canonical"),
        real.terminal().canonical_bytes().expect("canonical"),
        "a kernel that does nothing at all must not produce the same terminal bytes as one that \
         dispatched — this inequality is the whole of AC-008's terminal-state half"
    );

    // ---- (b) the five states, with an EXHAUSTIVE match inside the sweep -----------------------
    let token = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(1_700),
    )
    .expect("derives")
    .token()
    .clone();
    let cases: Vec<(&str, Vec<Event>)> = vec![
        ("not_started", vec![]),
        (
            "awaiting_gate",
            vec![gate_opened("budget_extension", TREE_A, 1_700)],
        ),
        (
            "awaiting_dispatch",
            vec![
                transition("analysis", STAGE),
                activation(EventKind::Dispatch, ACTIVITY, 1),
            ],
        ),
        (
            "stage_complete",
            vec![
                transition("analysis", STAGE),
                activation(EventKind::Dispatch, ACTIVITY, 1),
                activation(EventKind::ActivityCommitted, ACTIVITY, 1),
            ],
        ),
        (
            "sealed",
            vec![Event::new(
                EventKind::SliceSealed,
                serde_json::json!({ "verdict": "pass" }),
            )],
        ),
    ];
    assert_eq!(
        cases.len(),
        TerminalState::ALL_TAGS.len(),
        "one case per terminal state — ALL_TAGS is what makes a sixth state a red row here rather \
         than a forgotten one"
    );

    let mut bytes = Vec::new();
    for (tag, journal) in &cases {
        let state = fold(ITEM, STAGE, journal).expect("the case folds");
        let terminal = state.terminal();
        assert_eq!(terminal.as_str(), *tag, "case {tag}: {terminal:?}");
        // EXHAUSTIVE, NO WILDCARD: a sixth `TerminalState` is an `E0004` here.
        let described = match &terminal {
            TerminalState::NotStarted => "nothing in the journal moved the state".to_owned(),
            TerminalState::AwaitingGate { token } => format!("awaiting {}", token.as_str()),
            TerminalState::AwaitingDispatch { stage } => format!("dispatching in {stage}"),
            TerminalState::StageComplete { stage } => format!("done in {stage}"),
            TerminalState::Sealed { verdict } => format!("sealed {verdict}"),
        };
        assert!(!described.is_empty());
        bytes.push(terminal.canonical_bytes().expect("canonical"));
    }
    for (i, a) in bytes.iter().enumerate() {
        for (j, b) in bytes.iter().enumerate() {
            if i != j {
                assert_ne!(
                    a, b,
                    "two terminal states serialise identically at {i}/{j} — a replay could not \
                     tell them apart"
                );
            }
        }
    }
    assert_eq!(
        String::from_utf8(bytes[0].clone()).expect("utf-8"),
        r#"{"terminal":"not_started"}"#
    );
    // And the gate token really is the one the record derives: the awaiting state names it.
    assert_eq!(
        fold(
            ITEM,
            STAGE,
            &[gate_opened("budget_extension", TREE_A, 1_700)]
        )
        .expect("folds")
        .terminal(),
        TerminalState::AwaitingGate { token }
    );

    // ---- (c) THE TIER FILTER, which is inside `stage_action` ----------------------------------
    let full = vec![
        transition("analysis", STAGE),
        gate_opened("budget_extension", TREE_A, 1_700),
        Event::new(EventKind::DagAssembled, serde_json::json!({})),
        Event::new(EventKind::MemoHit, serde_json::json!({})),
        activation(EventKind::Dispatch, ACTIVITY, 1),
        activation(EventKind::ActivityCommitted, ACTIVITY, 1),
    ];
    let tier_one: Vec<Event> = full
        .iter()
        .filter(|e| e.kind.tier() == Tier::One)
        .cloned()
        .collect();
    let tier_two = full.len() - tier_one.len();
    // PREMISE, on AC-161's own shape: an all-tier-1 journal would make the equality below
    // trivially true.
    assert!(
        tier_two >= 2 && !tier_one.is_empty(),
        "the journal must carry at least two tier-2 events and one tier-1: {tier_one:?}"
    );
    // PREMISE 2, and it is the one that makes the filter load-bearing: the tier-2 events really do
    // move the FULL state. Without this, "the tier-1 projection gives the same answer" is a claim
    // about a journal in which nothing was dropped.
    assert_ne!(
        fold(ITEM, STAGE, &full)
            .expect("folds")
            .canonical_bytes()
            .expect("canonical"),
        fold(ITEM, STAGE, &tier_one)
            .expect("folds")
            .canonical_bytes()
            .expect("canonical"),
        "the tier-2 half of this journal must change the FULL state, or the stage-grain equality \
         below asserts nothing"
    );

    let whole = stage_action(ITEM, STAGE, &full).expect("the stage action computes");
    let projected = stage_action(ITEM, STAGE, &tier_one).expect("computes");
    assert_eq!(
        whole.canonical_bytes().expect("canonical"),
        projected.canonical_bytes().expect("canonical"),
        "AC-164: truncating tier 2 leaves the stage-level next action unchanged under the \
         canonical serialiser"
    );
    // PREMISE 3: an empty stage action is equal to another empty one, so the equality must be over
    // something.
    assert_ne!(
        whole,
        StageAction::default(),
        "an empty stage action is equal to another empty one"
    );
    assert_eq!(
        whole.transition,
        Some(("analysis".to_owned(), STAGE.to_owned()))
    );
    assert_eq!(whole.gates.len(), 1);

    // CONTROL: a DIFFERENT journal must give a DIFFERENT stage action, or every equality above is
    // satisfied by a `stage_action` that returns a constant.
    let elsewhere = vec![
        transition(STAGE, "verify"),
        gate_opened("budget_extension", TREE_B, 1_700),
    ];
    assert_ne!(
        whole.canonical_bytes().expect("canonical"),
        stage_action(ITEM, "verify", &elsewhere)
            .expect("computes")
            .canonical_bytes()
            .expect("canonical"),
        "a stage action that returns a constant would pass every equality above"
    );
    // And a tier-2-only journal projects to nothing at all — the stage grain cannot see it.
    assert_eq!(
        stage_action(
            ITEM,
            STAGE,
            &[
                Event::new(EventKind::DagAssembled, serde_json::json!({})),
                activation(EventKind::ActivityCommitted, ACTIVITY, 1),
            ]
        )
        .expect("computes"),
        StageAction::default()
    );

    // ---- (d) the six next actions, in the order design §3.2 fixes -----------------------------
    let gated = gated();
    assert_eq!(
        gated.gates().len(),
        1,
        "PREMISE: the abstaining-facet selection carries exactly one gate"
    );
    let axis = gated.gates()[0].axis.clone();
    assert_eq!(gated.gates()[0].reason, GateReason::UnknownFacet);
    let deadline = Timestamp::from_unix_seconds(9_000);
    let ctx_none = FsmContext::supplied(ITEM, TREE_A, deadline, None);
    let ctx_next = FsmContext::supplied(ITEM, TREE_A, deadline, Some("verify"));

    // 2. a gate the selection requires that the journal has not opened.
    let empty = fold(ITEM, "analysis", &[]).expect("folds");
    let expected_kind = format!("{}:{axis}", GateReason::UnknownFacet.as_str());
    let opened = match next_action(&gated, &empty, &ctx_none).expect("computes") {
        NextAction::OpenGate(record) => record,
        other => panic!("a required, unopened gate must be opened first: {other:?}"),
    };
    assert_eq!(opened.kind(), expected_kind);
    assert_eq!(opened.tree_hash(), TREE_A);
    assert_eq!(opened.deadline(), deadline);

    // 1. once it is in the journal, the same selection awaits it — and that outranks everything.
    let awaiting = fold(
        ITEM,
        "analysis",
        &[gate_opened(&expected_kind, TREE_A, 9_000)],
    )
    .expect("folds");
    assert_eq!(
        next_action(&gated, &awaiting, &ctx_next).expect("computes"),
        NextAction::AwaitGate {
            token: opened.token().clone()
        }
    );

    // 3. a node in step 6's order whose activation is not committed.
    let planned = planned();
    assert!(
        planned.gates().is_empty(),
        "PREMISE: this selection opens no gate, so step 3 is reached rather than step 2"
    );
    let order: Vec<String> = planned
        .plan()
        .expect("the assembled DAG orders")
        .order()
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    assert!(
        order.len() >= 2,
        "PREMISE: a one-node plan cannot show that the SECOND node is reached: {order:?}"
    );
    let first = planned
        .dag()
        .nodes()
        .iter()
        .find(|n| n.id == order[0])
        .expect("the ordered node is in the DAG");
    let fresh = fold(ITEM, STAGE, &[]).expect("folds");
    assert_eq!(
        next_action(&planned, &fresh, &ctx_none).expect("computes"),
        NextAction::Dispatch {
            key: rk_store::memo::IdempotencyKey {
                item: ITEM.to_owned(),
                activity_id_at_version: first.activity_id.clone(),
                attempt: 1,
            },
            step_id: order[0].clone(),
        },
        "the first uncommitted node in step 6's topological order, at attempt 1"
    );

    // AC-011's attempt is a FOLD, not a counter a loop holds: a recorded dispatch moves it.
    let retried = fold(
        ITEM,
        STAGE,
        &[activation(EventKind::Dispatch, &first.activity_id, 1)],
    )
    .expect("folds");
    match next_action(&planned, &retried, &ctx_none).expect("computes") {
        NextAction::Dispatch { key, .. } => assert_eq!(
            key.attempt, 2,
            "a recorded attempt-1 dispatch that did not commit is re-dispatched at attempt 2"
        ),
        other => panic!("still the same node: {other:?}"),
    }

    // Commit every node the plan orders, and the plan is exhausted.
    let mut done: Vec<Event> = Vec::new();
    for id in &order {
        let node = planned
            .dag()
            .nodes()
            .iter()
            .find(|n| &n.id == id)
            .expect("in the DAG");
        done.push(activation(
            EventKind::ActivityCommitted,
            &node.activity_id,
            1,
        ));
    }
    let exhausted = fold(ITEM, STAGE, &done).expect("folds");

    // 4. with a successor supplied → AdvanceStage. 6. with none → Done. **Both are asserted**,
    // because this system models NO stage ordering (`spikes/p9/packs/incident.toml`:1-2) and a
    // successor invented here would be a decision this stage may not take.
    assert_eq!(
        next_action(&planned, &exhausted, &ctx_next).expect("computes"),
        NextAction::AdvanceStage {
            from: STAGE.to_owned(),
            to: "verify".to_owned(),
        }
    );
    assert_eq!(
        next_action(&planned, &exhausted, &ctx_none).expect("computes"),
        NextAction::Done,
        "with no successor supplied the FSM does not invent one"
    );

    // 5. a recorded halt. `step_refused` is the ONLY halt this crate can read out of a journal:
    // AC-024's budget exhaustion and AC-033's breaker refusal have no `EventKind` at all, which
    // `budget.rs` and `breaker.rs` each already report ARMED.
    let mut refused = done.clone();
    refused.push(Event::new(
        EventKind::StepRefused,
        serde_json::json!({ "class": "overage" }),
    ));
    let halted = fold(ITEM, STAGE, &refused).expect("folds");
    assert_eq!(
        next_action(&planned, &halted, &ctx_none).expect("computes"),
        NextAction::Halt {
            reason: HaltReason::Refused {
                class: "overage".to_owned()
            }
        }
    );
    // CONTROL: the same journal WITHOUT the refusal is `Done`, so `Halt` is a property of the
    // recorded refusal and not of an exhausted plan.
    assert_eq!(
        next_action(&planned, &exhausted, &ctx_none).expect("computes"),
        NextAction::Done
    );

    // 🛑 **ADR-0154: A RECORDED HALT OUTRANKS A SUPPLIED SUCCESSOR — asserted on the COMBINATION.**
    //
    // Every arm above has a fixture and the PAIR had none, which is why the defect survived: the
    // halt assertion ten lines up passes `ctx_none`, so it cannot tell the old order (`Halt` below
    // `AdvanceStage`) from the new one. Only a halted journal folded against a context that DOES
    // supply a successor can, and before 7h nothing in the workspace built that combination —
    // `advances_to` appears in exactly two test builders, both for gated runs that break before
    // the plan is exhausted.
    //
    // Under the pre-ADR-0154 order this assertion reads `AdvanceStage { from: "implement", to:
    // "verify" }`: a refused run writing a tier-1 `stage_transition` and handing the next stage a
    // journal whose refusal nothing acted on. That is the outcome arm 5's own comment — *"BEFORE
    // `Done`, so a refused run is not reported finished"* — exists to prevent, and it was silently
    // false for every run with a successor.
    assert_eq!(
        next_action(&planned, &halted, &ctx_next).expect("computes"),
        NextAction::Halt {
            reason: HaltReason::Refused {
                class: "overage".to_owned()
            }
        },
        "a recorded halt must outrank a supplied successor (ADR-0154): with both present the FSM \
         reported an advance, so a refused run advanced a stage"
    );
    // CONTROL, in the other direction: the successor still wins over `Done`. Without this, moving
    // the halt check above the successor check could be satisfied by breaking `AdvanceStage`
    // outright, and the assertion above would not notice.
    assert_eq!(
        next_action(&planned, &exhausted, &ctx_next).expect("computes"),
        NextAction::AdvanceStage {
            from: STAGE.to_owned(),
            to: "verify".to_owned(),
        },
        "an exhausted plan with a successor and NO halt must still advance"
    );

    // The six tags are six distinct strings, and six distinct canonical forms.
    let actions = [
        NextAction::OpenGate(opened.clone()),
        NextAction::AwaitGate {
            token: opened.token().clone(),
        },
        NextAction::Dispatch {
            key: rk_store::memo::IdempotencyKey {
                item: ITEM.to_owned(),
                activity_id_at_version: first.activity_id.clone(),
                attempt: 1,
            },
            step_id: order[0].clone(),
        },
        NextAction::AdvanceStage {
            from: STAGE.to_owned(),
            to: "verify".to_owned(),
        },
        NextAction::Halt {
            reason: HaltReason::Refused {
                class: "overage".to_owned(),
            },
        },
        NextAction::Done,
    ];
    assert_eq!(actions.len(), NextAction::ALL_TAGS.len());
    let mut forms = Vec::new();
    for (i, action) in actions.iter().enumerate() {
        assert_eq!(action.as_str(), NextAction::ALL_TAGS[i]);
        forms.push(action.canonical_bytes().expect("canonical"));
    }
    for (i, a) in forms.iter().enumerate() {
        for (j, b) in forms.iter().enumerate() {
            if i != j {
                assert_ne!(a, b, "two next actions serialise identically at {i}/{j}");
            }
        }
    }

    // ---- (e) the payload readings refuse rather than defaulting -------------------------------
    //
    // `catalog.rs`'s and `pack.rs`'s parse discipline, one level up: a member that is absent or of
    // the wrong shape is a typed refusal, NEVER a silent zero.
    for (kind, payload) in [
        (EventKind::StageTransition, serde_json::json!({})),
        (
            EventKind::GateOpened,
            serde_json::json!({ "kind": "x", "tree_hash": TREE_A }),
        ),
        (
            EventKind::ActivityCommitted,
            serde_json::json!({ "item": ITEM, "activity_id_at_version": ACTIVITY, "attempt": -1 }),
        ),
    ] {
        let out = fold(ITEM, STAGE, &[Event::new(kind, payload.clone())]);
        match out {
            Err(FsmError::MalformedPayload { .. }) => {}
            other => panic!(
                "a `{}` with the payload {payload} must refuse, never default: {other:?}",
                kind.wire_name()
            ),
        }
    }
}

// ---- the three-way split, and the defect it replaced --------------------------------------------

/// **`gate_answered` is THREE cases, not two** — the operator decision of **2026-09-14**, landed as
/// code. An answer to a gate this system opened and then EXPIRED is **void**, and **the fold
/// continues**; an answer naming a token the journal never opened is still
/// `FsmError::GateAnsweredWithoutOpen`.
///
/// **What this replaces.** Until today this file carried
/// `gate_answered_after_expiry_is_unfoldable_known_defect`, a pin asserting the refusal *as a
/// defect*: the `GateExpired` arm removes the token from `gates_open`, the `GateAnswered` arm
/// refused any token absent from it, and the journal is append-only — so
/// `gate_opened → gate_expired → gate_answered` was **permanently unfoldable**, and the diagnostic
/// accused the operator of forging a token this system itself derived, opened and expired. That
/// pin required whoever changed the behaviour to name what licensed it; the line above is that
/// naming. F18 measured **45% of gate→verdict intervals above one hour**, so the answer that races
/// its own deadline is the common case, not the pathological one.
///
/// **The warrant is narrow and is stated here so it cannot widen: the arm conflated two different
/// situations, and only one of them is forgery.** Nothing here decides whether a voided gate
/// RE-OPENS — that is gate-record semantics, and AC-176's Boundary note assigns it to **7b-ii**.
/// This test asserts nothing about `next_action`, whose step 2 is untouched.
///
/// **Nothing is minted**: `EventKind::GateAnswerVoid` already exists with its own `apply` arm
/// writing exactly this disposition, `rk_types::drift` already models a void answer, and
/// `gates_void` was already terminal.
///
/// No AC id in the name, and that is deliberate: no criterion states the disposition of an answer
/// to an expired gate — AC-039 is about a `tree_hash` mismatch and is marked HUMAN GATE — so a
/// green here claims no settlement, and **a durable record naming the decision is OWED**.
/// (`budget.rs`'s `appetite_raise_from_an_activity_result_is_refused` is the precedent in this
/// crate for a name that withholds an id.)
#[test]
fn gate_answered_after_expiry_is_void_and_the_fold_continues() {
    let record = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(1_700),
    )
    .expect("derives");
    let token = record.token().as_str().to_owned();

    let opened = gate_opened("budget_extension", TREE_A, 1_700);
    let expired = Event::new(
        EventKind::GateExpired,
        serde_json::json!({ "token": token }),
    );
    let answered = gate_answered(&token, TREE_A, "extend");
    // The row that proves the fold CONTINUES rather than merely not erroring: a state-moving event
    // AFTER the answer, whose effect is visible in the folded state.
    let moved_on = transition(STAGE, "review");

    // ---- (a) the expired case: VOID, and the fold carries on past it -------------------------
    let state = fold(
        ITEM,
        STAGE,
        &[opened.clone(), expired, answered.clone(), moved_on.clone()],
    )
    .expect("an answer to an EXPIRED gate must not make the journal unfoldable");
    let void: Vec<&str> = state
        .void_gates()
        .map(rk_types::digest::Digest::as_str)
        .collect();
    assert_eq!(
        void,
        vec![token.as_str()],
        "the answer to the expired gate is VOID — `EventKind::GateAnswerVoid`'s own disposition, \
         and nothing is minted for it"
    );
    assert_eq!(
        state.answered_gates().count(),
        0,
        "and it is NOT an answer: a gate the system expired is not settled by an answer that \
         arrived after the deadline"
    );
    let expired_tokens: Vec<&str> = state
        .expired_gates()
        .map(rk_types::digest::Digest::as_str)
        .collect();
    assert_eq!(
        expired_tokens,
        vec![token.as_str()],
        "the expiry itself still stands — voiding the answer does not un-expire the gate"
    );
    assert_eq!(
        state.stage(),
        "review",
        "THE FOLD CONTINUES: the state-moving row recorded AFTER the void answer was applied. \
         This is the whole point — an append-only journal must never become unfoldable"
    );

    // ---- (b) THE CONTROL that keeps the forgery refusal alive --------------------------------
    //
    // Without this, (a) is satisfied by an arm that voids every answer it cannot place, and
    // `GateAnsweredWithoutOpen` would be dead code.
    let orphan = fold(
        ITEM,
        STAGE,
        &[gate_answered(&"d".repeat(64), TREE_A, "extend")],
    );
    match orphan {
        Err(FsmError::GateAnsweredWithoutOpen { token: refused }) => {
            assert_eq!(
                refused.as_str(),
                "d".repeat(64),
                "a token the journal NEVER opened is the forgery case, and the message is correct \
                 there"
            );
        }
        other => panic!(
            "an answer naming a token no derived gate ever carried must STILL halt the fold — \
             that is the one case `GateAnsweredWithoutOpen` is about: {other:?}"
        ),
    }

    // ---- (c) THE SECOND CONTROL: without the expiry row the answer is ANSWERED, not void ------
    let still_open = fold(ITEM, STAGE, &[opened, answered, moved_on]).expect("folds");
    assert_eq!(
        still_open.void_gates().count(),
        0,
        "an answer to a gate that is still OPEN, at the matching tree hash, is unchanged by D-5"
    );
    assert_eq!(
        still_open.answered_gates().count(),
        1,
        "…and it is recorded as the answer it is. Without this row (a)'s `void` is satisfied by \
         an arm that voids everything"
    );

    // ---- (c) THE FORGERY BRANCH IS NOT REACHABLE-AROUND VIA A FORGED EXPIRY ---------------
    //
    // Added 2026-09-15 after an adversarial pass found the hole this very split opened.
    // `GateOpened` DERIVES its token from `(kind, tree_hash, deadline)`, so a token in
    // `gates_open` is unforgeable; the `GateExpired` arm reads its token VERBATIM off the
    // payload. Until the repair it did an unconditional `gates_expired.insert(token)` after a
    // `remove` that silently no-ops — so appending ONE `gate_expired` naming any digest made a
    // later `gate_answered` for that digest fold as VOID AND CONTINUE instead of halting as
    // `GateAnsweredWithoutOpen`. D-5 deliberately kept that halt ("token never seen -> forgery,
    // and the message is correct HERE"), and one extra row went around it.
    let forged = GateRecord::derive(
        "budget_extension",
        TREE_A,
        Timestamp::from_unix_seconds(9_999),
    )
    .expect("derives");
    let forged_token = forged.token().as_str().to_owned();
    assert_ne!(
        forged_token, token,
        "PREMISE: the forged token must differ from the one a gate actually opened, or this case \
         is re-testing (a)"
    );

    let forged_expiry = Event::new(
        EventKind::GateExpired,
        serde_json::json!({ "token": forged_token }),
    );
    let forged_answer = gate_answered(&forged_token, TREE_A, "extend");

    let err = fold(ITEM, STAGE, &[forged_expiry, forged_answer])
        .expect_err("a gate this fold never saw OPEN cannot be expired into existence");
    assert!(
        matches!(err, FsmError::GateAnsweredWithoutOpen { .. }),
        "the answer must still meet the FORGERY halt. If this folds as a void instead, a \
         forged `gate_expired` row is a way around the one branch D-5 deliberately kept: \
         {err:?}"
    );

    // And the expiry itself moved no state: it is counted, never silently dropped, but a gate
    // that was never open is not recorded as expired either.
    let lone = fold(
        ITEM,
        STAGE,
        &[Event::new(
            EventKind::GateExpired,
            serde_json::json!({ "token": forged_token }),
        )],
    )
    .expect("a lone expiry folds; it simply records nothing");
    assert_eq!(
        lone.expired_gates().count(),
        0,
        "a gate that was never open must not appear in `expired_gates` — `gates_expired` has one \
         writer and this is the guard on it"
    );
}

/// **Every item-scoped kind carries the interim `item` key** (D21-43 N5; ADR-0182 decision 3;
/// `s8e` §8e.10). The record's list, verbatim, against `EventKind::ALL`: every kind is named in
/// exactly one of the three lists and `EventKind::item_key()` agrees with it, so a kind added later
/// without a decision fails here (and at `item_key`'s exhaustive match). Then each carried kind,
/// keyed to one item, is read by that item's journal and never by another's, through the kernel's
/// own selector (`runtime::journal_query`, `ByItem`) over the fake store.
///
/// RED at entry: no `item` member — `Event::with_item` and `EventKind::item_key` do not exist.
#[test]
fn every_item_scoped_kind_carries_the_interim_item_key() {
    use rk_ports::event_store::EventStore;
    use rk_ports::fakes::VecStore;
    use rk_types::event_kind::ItemKey;

    // ADR-0182 decision 3, verbatim.
    const CARRIED: [&str; 33] = [
        "stage_transition",
        "gate_opened",
        "gate_answered",
        "gate_expired",
        "slice_sealed",
        "reconcile_started",
        "gate_answer_void",
        "gate_answer_spent",
        "gate_resumed",
        "review_block",
        "quarantine_granted",
        "quarantine_removed",
        "quarantine_expired",
        "selection_ineffective",
        "stage_exit_refused",
        "dag_assembled",
        "dag_amended",
        "activity_committed",
        "dispatch",
        "dispatch_completed",
        "step_failed",
        "step_cancelled",
        "step_refused",
        "rate_limited",
        "degraded",
        "structured_drift",
        "drift_advisory",
        "review_class_drift",
        "artifact.read",
        "artifact.write",
        // *(dated 2026-09-28, `s8e` build, package S8E-P5; the three-kinds record decision 5,
        // proposed.)* The three kinds that record mints are item-scoped and carried, beside ADR-0182's
        // thirty.
        "worktree_assigned",
        "stage_empty",
        "git_op",
    ];
    // ADR-0163's payload member stays where it put it; no second source beside it.
    const IN_PAYLOAD: [&str; 1] = ["rate_limit_reading"];
    // Not item-scoped, each with the record's reason.
    const NOT_ITEM_SCOPED: [(&str, &str); 7] = [
        (
            "memo_hit",
            "the memo is keyed by content and shared across items (ADR-0097 d1)",
        ),
        (
            "memo_miss",
            "the memo is keyed by content and shared across items (ADR-0097 d1)",
        ),
        (
            "memo_evicted",
            "the memo is keyed by content and shared across items (ADR-0097 d1)",
        ),
        (
            "rewrite",
            "old_sha -> new_sha is a fact about the repository",
        ),
        (
            "advisory_created",
            "the advisory sink is store-wide (AC-310's invariant)",
        ),
        (
            "advisory_retired",
            "the advisory sink is store-wide (AC-310's invariant)",
        ),
        (
            "advisory_dismissed",
            "the advisory sink is store-wide (AC-310's invariant)",
        ),
    ];

    assert_eq!(
        CARRIED.len() + IN_PAYLOAD.len() + NOT_ITEM_SCOPED.len(),
        EventKind::ALL.len(),
        "the record's three lists name every kind"
    );
    for kind in EventKind::ALL {
        let wire = kind.wire_name();
        let named: Vec<ItemKey> = [
            CARRIED.contains(&wire).then_some(ItemKey::Carried),
            IN_PAYLOAD.contains(&wire).then_some(ItemKey::InPayload),
            NOT_ITEM_SCOPED
                .iter()
                .any(|(w, _)| *w == wire)
                .then_some(ItemKey::NotItemScoped),
        ]
        .into_iter()
        .flatten()
        .collect();
        assert_eq!(
            named.len(),
            1,
            "`{wire}` must be named in exactly one of ADR-0182's lists: {named:?}"
        );
        assert_eq!(
            kind.item_key(),
            named[0],
            "`{wire}`: EventKind::item_key() disagrees with ADR-0182 decision 3"
        );
    }

    let mut carried = 0;
    for kind in EventKind::ALL {
        if kind.item_key() != ItemKey::Carried {
            continue;
        }
        let mut store = VecStore::default();
        store
            .append(Event::new(kind, serde_json::json!({ "n": 1 })).with_item("item-a"))
            .expect("append a");
        store
            .append(Event::new(kind, serde_json::json!({ "n": 2 })).with_item("item-b"))
            .expect("append b");
        let mine = store
            .query(rk_kernel::runtime::journal_query("item-a"))
            .expect("the kernel's read");
        assert_eq!(
            mine.iter().map(Event::item).collect::<Vec<_>>(),
            vec![Some("item-a")],
            "`{}`: the kernel's journal for item-a must carry item-a's row alone",
            kind.wire_name()
        );
        carried += 1;
    }
    assert_eq!(carried, CARRIED.len(), "every carried kind was exercised");
}

/// **D21-93 — an approved seal is scoped to the stage it was made in** (`s8e` §8e.8; §8e.0 item 8;
/// the slice record, proposed). *(Added 2026-09-28, `s8e` build, package S8E-P7.)*
///
/// S8E-P0 measured the defect in the test binary: `terminal()` read `sealed` first and nothing
/// cleared it, so a journal `review -> slice_sealed{approved} -> release -> gate_opened` folded to
/// `awaiting_gate = Some(..)` and `terminal = sealed`. The ruling: the fold records the stage a seal
/// was made in, and `terminal()` returns `Sealed` only while the fold is still in that stage; after
/// a later `stage_transition` it falls through to gate, dispatch or complete. The `sealed` member
/// keeps the verdict as history (ADR-0109 decision 3).
///
/// Three arms. (1) The seal made in `review`, journaled AFTER the advance into `release` (the order
/// `run` writes it: the kernel's advance is inside `resume`, the review pass after it), then a gate
/// at `release`: `AwaitingGate`, with `sealed()` still `approved`. (2) The same seal and no later
/// transition, the fold in `review`: `Sealed`. (3) A stage-less seal row (a rebuilt or healed row,
/// whose trailer carries no stage) is scoped to the stage it folds in: `Sealed` there, and not after
/// a later transition. Red under the scope dropped (S8E-M50).
#[test]
fn an_approved_seal_reads_sealed_only_until_the_next_stage_transition() {
    let seal = |stage: Option<&str>| {
        let mut payload = serde_json::json!({
            "project": "demo",
            "slice": "sl-1",
            "verdict": "approved",
        });
        if let Some(stage) = stage {
            payload["stage"] = serde_json::Value::from(stage);
        }
        Event::new(EventKind::SliceSealed, payload)
    };

    // (1) the lane moved on: a gate at `release` is what the run awaits.
    let moved_on = vec![
        transition("implement", "review"),
        transition("review", "release"),
        seal(Some("review")),
        gate_opened("budget_extension", TREE_A, 1_700),
    ];
    let state = fold(ITEM, "implement", &moved_on).expect("the journal folds");
    assert!(
        state.awaiting_gate().is_some(),
        "PREMISE: a gate is open at `release`"
    );
    assert_eq!(
        state.sealed(),
        Some("approved"),
        "the seal is kept as history (ADR-0109 decision 3)"
    );
    assert_eq!(
        state.terminal().as_str(),
        "awaiting_gate",
        "D21-93: a seal made in `review` must not read as the terminal state of a lane that went \
         on to `release` and halted at its gate: {:?}",
        state.terminal()
    );

    // (2) the fold is still in the stage the seal was made in.
    let at_review = vec![transition("implement", "review"), seal(Some("review"))];
    assert_eq!(
        fold(ITEM, "implement", &at_review)
            .expect("folds")
            .terminal(),
        TerminalState::Sealed {
            verdict: "approved".to_owned()
        },
        "a seal made in the fold's current stage is the terminal state"
    );

    // (3) a stage-less seal row is scoped to the stage it folds in.
    let stageless = vec![transition("implement", "review"), seal(None)];
    assert_eq!(
        fold(ITEM, "implement", &stageless)
            .expect("folds")
            .terminal()
            .as_str(),
        "sealed",
        "a stage-less seal folds as made in the fold's stage at that row"
    );
    let stageless_then_on = vec![
        transition("implement", "review"),
        seal(None),
        transition("review", "release"),
        gate_opened("budget_extension", TREE_A, 1_700),
    ];
    assert_eq!(
        fold(ITEM, "implement", &stageless_then_on)
            .expect("folds")
            .terminal()
            .as_str(),
        "awaiting_gate",
        "a later stage_transition ends a stage-less seal's scope"
    );
}
