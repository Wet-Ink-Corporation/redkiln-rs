//! **The step-4 gate on a human-sourced kind** — `src/workflow/human_root.rs` and `fsm::next_action`'s
//! step 2 (`s8j` §8j.2; ADR-0200 decision 2, proposed). *(Added 2026-10-18, `s8j` build, package
//! S8J-C2-ROOT-QUESTIONS, review finding F2/F3.)*
//!
//! A minimal pack, catalog and registry, inline: one `analysis` activity (`repro`) providing
//! `ReproCase` and requiring the human kind `BugReport`, and one `discovery` activity (`intake`)
//! providing `BugReport` that only the ForbiddenStage control installs. Three behaviours, one test
//! each: the gate's lifecycle (reach 1 on open, nothing after an answer, reach 2 after a void); the
//! `unknown_facet` abstentions open before it; and a `ForbiddenStage` `Unsatisfiable` on a human kind
//! opens no `human_root` gate (ADR-0200 decision 2, *"Not widened"*).

use rk_kernel::runtime::gate_opened_event;
use rk_kernel::select::{ArtifactRegistry, Catalog, Item, Pack, Selection, UnsatReason, select};
use rk_kernel::workflow::fsm::{FsmContext, NextAction, fold, next_action};
use rk_kernel::workflow::gate::gate_answered_event;
use rk_types::event::Event;
use rk_types::time::Timestamp;

const ITEM: &str = "BUG-1";
const STAGE: &str = "analysis";
const TREE_A: &str = "aaaa111122223333444455556666777788889999";
const TREE_B: &str = "bbbb111122223333444455556666777788889999";
const DEADLINE: i64 = 1_800_000_000;

const PACK: &str = r#"[pack]
id = "root-fixture"
applies = "kind == bug"
appetite = { points = 50 }

[policy]
stages = { triage = "mandatory", discovery = "forbidden", analysis = "recommended", specification = "forbidden", design = "forbidden", planning = "forbidden", implement = "recommended", verify = "recommended", review = "mandatory", release = "mandatory", operate = "mandatory", closeout = "mandatory" }

[stage.analysis]
baseline = ["ReproCase"]
[[stage.analysis.require]]
when = "external_dep"
artifacts = ["APIContract"]
"#;

const ARTIFACTS: &str = r#"[artifact.APIContract]
source = "activity"
schema = "APIContract"

[artifact.BugReport]
source = "human"
schema = "BugReport"

[artifact.ReproCase]
source = "activity"
schema = "ReproCase"
"#;

const REPRO: &str = r#"[activity]
id           = "repro"
version      = 1
stage        = "analysis"
provides     = ["ReproCase"]
requires     = ["BugReport"]
derived_from = ["BugReport"]
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

const API_CONTRACT: &str = r#"[activity]
id           = "api-contract"
version      = 1
stage        = "analysis"
provides     = ["APIContract"]
requires     = []
tags         = ["kind:bug"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"

[[step]]
id      = "write"
kind    = "agent"
outputs = "value:APIContract"

[[step]]
id      = "emit"
kind    = "emit"
"#;

/// The ForbiddenStage control's producer: `BugReport` provided only in `discovery`, which the pack
/// forbids, so step 4 reports the human kind `Unsatisfiable { ForbiddenStage }`.
const INTAKE: &str = r#"[activity]
id           = "intake"
version      = 1
stage        = "discovery"
provides     = ["BugReport"]
requires     = []
tags         = ["kind:bug"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"

[[step]]
id      = "write"
kind    = "agent"
outputs = "value:BugReport"

[[step]]
id      = "emit"
kind    = "emit"
"#;

/// The fixture's selection and the FSM's answer over it (TST-14: a test helper module).
#[cfg(test)]
mod fixture {
    use super::{
        API_CONTRACT, ARTIFACTS, ArtifactRegistry, Catalog, DEADLINE, Event, FsmContext, INTAKE,
        ITEM, Item, NextAction, PACK, Pack, REPRO, STAGE, Selection, TREE_A, Timestamp, fold,
        next_action, select,
    };

    fn facets() -> rk_kernel::registry::Registry {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        rk_kernel::registry::Registry::from_toml(&src)
            .expect("the shipped facet registry must load")
    }

    /// The analysis selection over the fixture; `external_dep` is `Some` to decide it, `None` to
    /// abstain (an `unknown_facet:external_dep` gate); `forbidden_producer` installs `intake`.
    pub(super) fn selection(external_dep: Option<bool>, forbidden_producer: bool) -> Selection {
        let mut sources = vec![("repro.toml", REPRO), ("api-contract.toml", API_CONTRACT)];
        if forbidden_producer {
            sources.push(("intake.toml", INTAKE));
        }
        let pack = Pack::from_toml("root-fixture.toml", PACK).expect("the fixture pack loads");
        let catalog = Catalog::from_sources(&sources).expect("the fixture catalog loads");
        let artifacts = ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("it loads");
        let item = Item::new(ITEM, "bug");
        let item = match external_dep {
            Some(v) => item.with_facet_bool("external_dep", v),
            None => item.with_facet_null("external_dep"),
        };
        select(&pack, &catalog, &artifacts, &facets(), STAGE, &item)
            .expect("the selection resolves")
    }

    pub(super) fn act(selection: &Selection, journal: &[Event]) -> NextAction {
        let state = fold(ITEM, STAGE, journal).expect("the journal folds");
        let ctx = FsmContext::supplied(ITEM, TREE_A, Timestamp::from_unix_seconds(DEADLINE), None);
        next_action(selection, &state, &ctx).expect("the next action computes")
    }
}

use fixture::{act, selection};

/// **Reach 1 on open; no re-open once answered; reach 2, fresh, after a void** — step 2's one rule
/// (ADR-0200 decision 2: *"Re-opening follows the one rule step 2 applies to every gate"*).
#[test]
fn a_human_root_gate_opens_at_reach_one_stays_shut_once_answered_and_reopens_after_a_void() {
    let selection = selection(Some(false), false);
    assert!(
        selection
            .unsatisfiable()
            .iter()
            .any(|u| u.kind == "BugReport" && u.reason == UnsatReason::NoAdmissibleProducer),
        "PREMISE: `repro`'s `BugReport` is Unsatisfiable {{ NoAdmissibleProducer }}"
    );
    assert!(selection.gates().is_empty(), "PREMISE: no facet abstains");

    let NextAction::OpenGate(first) = act(&selection, &[]) else {
        panic!("a fresh journal must open the human root's gate before dispatching `repro`");
    };
    assert_eq!(first.kind(), "human_root:BugReport");
    assert_eq!(first.reach(), 1);

    let opened = gate_opened_event(&first);
    let answered = gate_answered_event(first.token(), TREE_A, "attach");
    let after_answer = act(&selection, &[opened.clone(), answered]);
    assert!(
        matches!(&after_answer, NextAction::Dispatch { key, .. } if key.activity_id_at_version == "repro"),
        "an answered gate is not re-opened, and the stage dispatches `repro`; got {after_answer:?}"
    );

    // A void answer (AC-039: the tree moved) re-opens fresh, one reach past (AC-252).
    let stale = gate_answered_event(first.token(), TREE_B, "attach");
    let NextAction::OpenGate(again) = act(&selection, &[opened, stale]) else {
        panic!("a voided human-root gate must re-open");
    };
    assert_eq!(again.kind(), "human_root:BugReport");
    assert_eq!(
        again.reach(),
        2,
        "the re-open is one reach past the voided one"
    );
    assert_ne!(again.token(), first.token(), "the re-opened token is fresh");
}

/// **The step-2 `unknown_facet` abstentions open before the step-4 `human_root` gate** (ADR-0200
/// decision 2: *"beside the `unknown_facet` gates"*; the FSM's step 2 order).
#[test]
fn unknown_facet_gates_open_before_the_human_root_gate() {
    let selection = selection(None, false);
    assert_eq!(
        selection.gates().len(),
        1,
        "PREMISE: `external_dep` abstains"
    );

    let NextAction::OpenGate(facet) = act(&selection, &[]) else {
        panic!("a fresh journal must open a gate first");
    };
    assert_eq!(facet.kind(), "unknown_facet:external_dep");

    let opened = gate_opened_event(&facet);
    let answered = gate_answered_event(facet.token(), TREE_A, "false");
    let NextAction::OpenGate(root) = act(&selection, &[opened, answered]) else {
        panic!("with the facet gate answered, the human root's gate opens next");
    };
    assert_eq!(root.kind(), "human_root:BugReport");
}

/// **A `ForbiddenStage` `Unsatisfiable` on a human kind opens no `human_root` gate** (ADR-0200
/// decision 2, *"Not widened"*; its *What this forbids*). No lint refuses a catalog producer of a
/// `source = "human"` kind, so the case can arise: here its only producer sits in a forbidden stage.
#[test]
fn a_forbidden_stage_unsatisfiable_on_a_human_kind_opens_no_human_root_gate() {
    let selection = selection(Some(false), true);
    assert!(
        selection
            .unsatisfiable()
            .iter()
            .any(|u| u.kind == "BugReport" && u.reason == UnsatReason::ForbiddenStage),
        "PREMISE: `BugReport` is Unsatisfiable {{ ForbiddenStage }}: {:?}",
        selection.unsatisfiable()
    );
    let action = act(&selection, &[]);
    assert!(
        !matches!(&action, NextAction::OpenGate(r) if r.kind().starts_with("human_root:")),
        "a ForbiddenStage Unsatisfiable must open no human_root gate; got {action:?}"
    );
}
