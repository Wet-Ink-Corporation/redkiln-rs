//! **The lane order and the stage boundary** — Step 7h, `s7h` §7h.1.
//!
//! | what | record | asserted here |
//! | --- | --- | --- |
//! | `[policy] stage_order` is the only ordered stage carrier | ADR-0153 | the shipped packs' lanes, walked end to end |
//! | the two policy keys may not disagree | ADR-0153 | both directions of the difference, plus a duplicate |
//! | a pack declaring no order cannot advance | ADR-0153 | `successor() == None` against a pack that omits the key |
//! | forbidden stages are SKIPPED, not stopped at | ADR-0153 | `bug-standard` marks two, mid-order |
//!
//! 🛑 **Why an array and not the table's key order.** `policy.stages` is an inline table;
//! `crate::toml_subset::Table` is a `BTreeMap`, so its twelve keys come back ALPHABETISED, and
//! `parse_spans` cannot recover the textual order either because the whole inline table sits on one
//! line and spans are at line grain. A TOML table has no order by specification. The lane order is
//! therefore declared, and the alphabetisation below is asserted so that a future reader who
//! "fixes" `stage_order` away by reading `stages` instead finds out here.

use std::sync::{Arc, Mutex};

use rk_kernel::replay::{self, RunInputs, RuntimeContext};
use rk_kernel::rigor::Rigor;
use rk_kernel::select::{Item, Pack};
use rk_ports::event_store::EventStore;
use rk_ports::fakes::{BoardTracker, FakeHarness, FixtureToolchain, RecordingDocs, VecStore};
use rk_ports::root::Ports;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The shipped packs, read off disk rather than from a fixture: the lane these assert is the lane
/// `redkiln run` will walk, and a fixture copy could drift from it silently.
fn shipped(name: &str) -> Pack {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("the workspace root is two levels above this crate")
        .to_path_buf();
    let path = root.join(".redkiln").join("packs").join(name);
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the shipped pack {} is readable: {e}", path.display()));
    Pack::from_toml(name, &src).unwrap_or_else(|e| panic!("the shipped pack {name} loads: {e}"))
}

/// A minimal pack body, with `[policy]` supplied by the caller.
///
/// `[pack]` is mandatory — the loader refuses *"no [pack] table"* without it — so it is here rather
/// than in each caller.
fn pack_src(policy: &str) -> String {
    format!("[pack]\nid = \"fixture\"\n\n[policy]\n{policy}\n")
}

#[test]
fn adr0153_the_shipped_lane_order_survives_loading_and_the_table_does_not() {
    for name in ["bug-standard.toml", "story.toml"] {
        let pack = shipped(name);
        let order = pack.stage_order();

        assert_eq!(
            order.len(),
            12,
            "{name} declares {} stages in its lane order, not twelve",
            order.len()
        );
        assert_eq!(
            order.first().map(String::as_str),
            Some("triage"),
            "{name}'s lane must open at triage"
        );
        assert_eq!(
            order.last().map(String::as_str),
            Some("closeout"),
            "{name}'s lane must end at closeout"
        );

        // 🛑 THE ANCHOR THAT MAKES THIS TEST MEAN ANYTHING. If the declared order were merely the
        // table's keys, `stage_order` would be redundant and this module would be asserting a
        // tautology. It is not: the loaded order is NOT sorted, and the sorted form differs.
        let mut sorted = order.to_vec();
        sorted.sort();
        assert_ne!(
            order.to_vec(),
            sorted,
            "{name}'s declared lane order is in alphabetical order, so this test cannot tell a \
             declared order from a `BTreeMap`'s key order — the fixture has stopped discriminating"
        );
    }
}

#[test]
fn adr0153_successor_skips_forbidden_and_ends_the_lane() {
    let pack = shipped("bug-standard.toml");

    // The two forbidden stages are mid-order, which is the whole reason a successor SKIPS rather
    // than stops: `discovery` sits between `triage` and `analysis`, `operate` between `release` and
    // `closeout`. A successor that stopped at the first forbidden entry would end every lane at
    // `triage`.
    assert_eq!(pack.rigor("discovery"), Some(Rigor::Forbidden));
    assert_eq!(pack.rigor("operate"), Some(Rigor::Forbidden));

    assert_eq!(
        pack.successor("triage"),
        Some("analysis"),
        "triage's successor must skip the forbidden discovery"
    );
    assert_eq!(
        pack.successor("release"),
        Some("closeout"),
        "release's successor must skip the forbidden operate"
    );
    assert_eq!(
        pack.successor("implement"),
        Some("verify"),
        "an ordinary adjacent pair"
    );

    // The end of the lane is an absence, not an error (`NextAction::Done`'s own reading).
    assert_eq!(
        pack.successor("closeout"),
        None,
        "the last stage of the order has no successor"
    );
    assert_eq!(
        pack.successor("no-such-stage"),
        None,
        "a stage the order does not name has no successor"
    );

    // The whole lane, walked. This is the sequence a fix lane visits, and it is asserted as a
    // sequence rather than pairwise so that a reordering shows up as one failure.
    let mut walk = vec!["triage"];
    while let Some(next) = pack.successor(walk[walk.len() - 1]) {
        walk.push(next);
        assert!(
            walk.len() <= 12,
            "the lane walk did not terminate: {walk:?}"
        );
    }
    assert_eq!(
        walk,
        vec![
            "triage",
            "analysis",
            "specification",
            "design",
            "planning",
            "implement",
            "verify",
            "review",
            "release",
            "closeout",
        ],
        "bug-standard's lane: twelve declared stages minus the two it forbids"
    );

    // `story` forbids only `discovery`, so its lane is eleven stages and includes `operate`. Two
    // packs with the same twelve names and different lanes is the case a canonical kernel-side
    // sequence could not represent.
    let story = shipped("story.toml");
    assert_eq!(story.rigor("operate"), Some(Rigor::Optional));
    assert_eq!(
        story.successor("release"),
        Some("operate"),
        "story does not forbid operate, so its lane keeps it — the two packs' lanes differ"
    );
}

#[test]
fn adr0153_the_two_policy_keys_may_not_disagree() {
    // A stage in `stages` that the order omits.
    let missing = Pack::from_toml(
        "missing.toml",
        &pack_src(
            "stages = { triage = \"mandatory\", review = \"mandatory\" }\n\
             stage_order = [\"triage\"]",
        ),
    )
    .expect_err("a stage with a rigor but no place in the lane must refuse");
    let text = missing.to_string();
    assert!(
        text.contains("review"),
        "the refusal must name the stage that has no place in the lane: {text}"
    );

    // A stage in the order that `stages` does not mention.
    let extra = Pack::from_toml(
        "extra.toml",
        &pack_src(
            "stages = { triage = \"mandatory\" }\n\
             stage_order = [\"triage\", \"review\"]",
        ),
    )
    .expect_err("a stage in the lane with no declared rigor must refuse");
    let text = extra.to_string();
    assert!(
        text.contains("review"),
        "the refusal must name the stage with no rigor: {text}"
    );

    // A repeated entry: the successor of the repeated stage would otherwise be ambiguous.
    let dup = Pack::from_toml(
        "dup.toml",
        &pack_src(
            "stages = { triage = \"mandatory\", review = \"mandatory\" }\n\
             stage_order = [\"triage\", \"review\", \"triage\"]",
        ),
    )
    .expect_err("a duplicate entry must refuse");
    assert!(
        dup.to_string().contains("triage"),
        "the refusal must name the repeated stage: {dup}"
    );

    // CONTROL: the agreeing pair loads, so the three refusals above are about the disagreement and
    // not about the fixture being malformed some other way.
    let ok = Pack::from_toml(
        "ok.toml",
        &pack_src(
            "stages = { triage = \"mandatory\", review = \"mandatory\" }\n\
             stage_order = [\"triage\", \"review\"]",
        ),
    )
    .expect("the agreeing pair loads");
    assert_eq!(ok.successor("triage"), Some("review"));
}

/// **`closed` is reserved for the lane's end, so a pack that declares a stage of that name is
/// refused at load** (`s8e` §8e.5; D21-43 N9; ADR-0183, proposed). *(Added 2026-09-28, `s8e`
/// build, package S8E-P4.)*
///
/// Red at entry because `closed` was an ordinary name: a pack could declare it, and the lane's
/// terminal `Story: <project>/<item> closed` mark would then be indistinguishable from entering
/// that stage. Every place a stage is declared is tried — `policy.stages`, `policy.stage_order`
/// and a `[stage.closed]` table — and a control shows the same body with another name loads.
#[test]
fn a_pack_stage_named_closed_is_refused_at_load() {
    let reserved = rk_git::trailer::grammar::CLOSED_STAGE_TOKEN;
    assert_eq!(reserved, "closed");
    for (site, src) in [
        (
            "policy.stages and policy.stage_order",
            pack_src(
                "stages = { triage = \"mandatory\", closed = \"mandatory\" }\n\
                 stage_order = [\"triage\", \"closed\"]",
            ),
        ),
        (
            "policy.stages alone (no order)",
            pack_src("stages = { triage = \"mandatory\", closed = \"optional\" }"),
        ),
        (
            "a [stage.closed] table",
            format!(
                "{}\n[stage.closed]\nbaseline = []\n",
                pack_src(
                    "stages = { triage = \"mandatory\" }\n\
                     stage_order = [\"triage\"]"
                )
            ),
        ),
    ] {
        let refused = Pack::from_toml("closed.toml", &src);
        let Err(err) = refused else {
            panic!("`closed` is an ordinary name: a pack declaring it in {site} loaded");
        };
        let text = err.to_string();
        assert!(
            text.contains("closed") && text.contains("reserved"),
            "the refusal must name the reserved stage `closed` ({site}): {text}"
        );
    }

    // CONTROL: the same bodies with another name load, so the refusals are about the name.
    for src in [
        pack_src(
            "stages = { triage = \"mandatory\", closeout = \"mandatory\" }\n\
             stage_order = [\"triage\", \"closeout\"]",
        ),
        format!(
            "{}\n[stage.closeout]\nbaseline = []\n",
            pack_src(
                "stages = { triage = \"mandatory\", closeout = \"mandatory\" }\n\
                 stage_order = [\"triage\", \"closeout\"]"
            )
        ),
    ] {
        Pack::from_toml("ok.toml", &src).expect("`closeout` is an ordinary stage name and loads");
    }
    // And the shipped packs, whose last stage is `closeout`, still load.
    for name in ["bug-standard.toml", "story.toml"] {
        let _ = shipped(name);
    }
}

#[test]
fn adr0153_a_pack_declaring_no_lane_order_cannot_advance() {
    // An absence is not an identity map and not an alphabetical fallback: it is a refusal to
    // advance, on `Pack::rigor`'s doctrine that an absence is not a value.
    let pack = Pack::from_toml(
        "no-order.toml",
        &pack_src("stages = { triage = \"mandatory\", review = \"mandatory\" }"),
    )
    .expect("a pack with no stage_order still loads — the key is not mandatory");

    assert!(pack.stage_order().is_empty());
    assert_eq!(
        pack.rigor("triage"),
        Some(Rigor::Mandatory),
        "the stage is declared; it is only its PLACE in a lane that is not"
    );
    assert_eq!(
        pack.successor("triage"),
        None,
        "a pack that declares no lane order cannot advance, rather than guessing one"
    );
    assert_eq!(pack.successor("review"), None);
}

// ---------------------------------------------------------------------------------------------
// §7h.1's behavioural core: `replay::resume` takes the boundary, ONCE, and breaks.
// ---------------------------------------------------------------------------------------------

const LANE_PACK: &str = include_str!("../fixtures/packs/lane-advance.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

/// One activity, at `implement` — **deliberately NOT at `triage`**, which is the stage the run
/// enters. An EMPTY catalog is refused by the loader (*"a rule that enumerates nothing passes"*),
/// so the plan is emptied by the STAGE FILTER rather than by an empty corpus: `select` finds this
/// activity and excludes it because `activity.stage != stage`. That is the shipped tree's shape at
/// `triage` too, and it keeps the anchor honest — the catalog is non-empty and still selects nothing.
const ADV_CATALOG: &[(&str, &str)] = &[(
    "impl-report.toml",
    include_str!("../fixtures/catalog-select/impl-report.toml"),
)];
const ADV_ITEM: &str = "INC-7H";
const ADV_TREE: &str = "9f2c1ab4d6e80357bb41c0d9e2f7a85643c1908df5b2e7461ac30d98f5b1e2c7";
const ADV_DEADLINE: i64 = 1_760_000_000;
const ADV_RESULTS_AT: i64 = 1_759_900_000;

/// The facet registry the shipped tree uses, read off disk exactly as `resume.rs` does.
fn facets_src() -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("the workspace root is two levels above this crate");
    std::fs::read_to_string(root.join("spikes/p9/facets.toml"))
        .expect("the shipped facet registry is readable")
}

/// Inputs at `triage` — a stage with no catalog activity, so the plan is empty and exhausted on the
/// first iteration. `successor` is supplied by the caller, which is the whole variable under test.
fn advance_inputs(successor: Option<&str>) -> RunInputs {
    advance_inputs_at("triage", successor)
}

/// The same inputs at an arbitrary stage — what the lane loop builds per stage (ADR-0152).
fn advance_inputs_at(stage: &str, successor: Option<&str>) -> RunInputs {
    let mut runtime = RuntimeContext::supplied(ADV_TREE, ADV_DEADLINE).results_at(ADV_RESULTS_AT);
    if let Some(to) = successor {
        runtime = runtime.advances_to(to);
    }
    RunInputs::new(stage)
        .with_runtime(runtime)
        .with_pack("lane-advance.toml", LANE_PACK)
        .with_catalog(ADV_CATALOG)
        .with_artifacts("artifacts.toml", ARTIFACTS)
        .with_facets(&facets_src())
        .with_item(&Item::new(ADV_ITEM, "incident").with_toolchain("node", ["text"]))
}

/// The store's journal for the fixture item — what `rk-cli` hands `resume` as `prior` at step (9).
fn journal_of(store: &Arc<Mutex<dyn EventStore>>) -> Vec<Event> {
    store
        .lock()
        .expect("the fake store is not poisoned")
        .query(rk_kernel::runtime::journal_query(ADV_ITEM))
        .expect("the fake store queries")
}

fn advance_ports(store: &Arc<Mutex<dyn EventStore>>) -> Ports {
    Ports {
        store: Arc::clone(store),
        // An EMPTY script: this run must dispatch nothing. A scripted response would let a
        // dispatch pass unnoticed, and the plan being empty is half of what the test asserts.
        harness: Arc::new(FakeHarness::scripted(Vec::new())),
        tracker: Arc::new(BoardTracker::default()),
        docs: Arc::new(RecordingDocs::default()),
        toolchain: Arc::new(FixtureToolchain::default()),
    }
}

/// Every `stage_transition` row in `events`, as `(from, to)`.
///
/// Both the stage-ENTRY row and the ADVANCE row are `EventKind::StageTransition`, so counting the
/// kind alone cannot tell them apart — the entry row carries only `to`, the advance row carries
/// both. Judging the payload is what makes "exactly one boundary was taken" assertable.
fn transitions(events: &[Event]) -> Vec<(Option<String>, Option<String>)> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::StageTransition)
        .map(|e| {
            let get = |k: &str| {
                e.payload
                    .get(k)
                    .and_then(serde_json::Value::as_str)
                    .map(ToOwned::to_owned)
            };
            (get("from"), get("to"))
        })
        .collect()
}

/// **ADR-0152: the arm steps the boundary and BREAKS, and the successor is what makes it fire.**
///
/// The control is the point. The same inputs with no successor supplied must produce NO advance —
/// otherwise the assertion would pass against a `resume` that advanced unconditionally, which is a
/// thing `next_action` cannot do but a future arm could.
#[test]
fn adr0152_resume_takes_one_stage_boundary_and_breaks() {
    let pack = Pack::from_toml("lane-advance.toml", LANE_PACK).expect("the fixture pack loads");
    // The successor comes from the pack's declared order, skipping the forbidden `discovery` — the
    // integration ADR-0153 describes, not a literal the test chose.
    let successor = pack.successor("triage").expect("triage has a successor");
    assert_eq!(successor, "implement", "the forbidden discovery is skipped");

    // ---- WITH a successor: one boundary, taken and recorded.
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let inputs = advance_inputs(Some(successor));
    let recorded = replay::resume(&inputs, advance_ports(&store), &[], &mut |node| {
        panic!("no node may be supplied: the plan at `triage` is empty, yet {node:?} was requested")
    })
    .expect("resume over an empty plan with a successor supplied");

    assert_eq!(
        recorded.advanced_to(),
        Some("implement"),
        "the boundary the loop took must be reported by the boundary itself"
    );

    let rows = transitions(&recorded.events);
    assert_eq!(
        rows,
        vec![
            (None, Some("triage".to_owned())),
            (Some("triage".to_owned()), Some("implement".to_owned())),
        ],
        "exactly two `stage_transition` rows: the stage ENTRY (`to` only) and ONE advance \
         (`from` and `to`). More than one advance row means the arm continued the loop instead of \
         breaking, which is the unbounded append ADR-0152 exists to prevent"
    );

    // The advance row reached the STORE, not only the returned journal: `rt.step` writes it through
    // the port, and the pre-loop append covers only rows minted before the loop.
    let stored = store
        .lock()
        .expect("the fake store is not poisoned")
        .query(rk_kernel::runtime::journal_query(ADV_ITEM))
        .expect("the fake store queries");
    assert_eq!(
        transitions(&stored),
        rows,
        "the rows in the store must be the rows the run reports"
    );

    // ---- CONTROL: no successor, no advance. Same pack, same item, same empty plan.
    let store2: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let bare = advance_inputs(None);
    let plain = replay::resume(&bare, advance_ports(&store2), &[], &mut |node| {
        panic!("no node may be supplied: {node:?}")
    })
    .expect("resume over an empty plan with no successor");

    assert_eq!(
        plain.advanced_to(),
        None,
        "with no successor supplied the run must take no boundary"
    );
    assert_eq!(
        transitions(&plain.events),
        vec![(None, Some("triage".to_owned()))],
        "the stage-entry row alone: a supplied successor is what makes the boundary happen, and \
         without one the FSM reports `Done` (it never invents a successor)"
    );
}

/// **ADR-0157: a stage does not advance into itself, and a second run appends nothing.**
///
/// The defect this pins was MEASURED against the real binary, not anticipated: `from` is the fold's
/// stage and `to` is the supplied successor, and across runs they diverge because the composition
/// root computes the successor from the item record (which never moves) while the fold follows the
/// `stage_transition` rows the previous run wrote. Run two appended
/// `{"from":"analysis","to":"analysis"}`, and run three would have appended another.
///
/// The fixture reproduces that by REPLAYING the first run's journal as `prior`, which is exactly what
/// `rk-cli` does at step (9) — it hands `resume` the store's own rows.
#[test]
fn adr0157_a_second_run_advances_nowhere_and_appends_nothing() {
    let pack = Pack::from_toml("lane-advance.toml", LANE_PACK).expect("the fixture pack loads");
    let successor = pack.successor("triage").expect("triage has a successor");

    // ---- Run one: the boundary is taken.
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let inputs = advance_inputs(Some(successor));
    let first = replay::resume(&inputs, advance_ports(&store), &[], &mut |n| {
        panic!("no node may be supplied: {n:?}")
    })
    .expect("the first run advances");
    assert_eq!(first.advanced_to(), Some("implement"));

    // The journal the store now holds — what the next invocation is handed.
    let prior = store
        .lock()
        .expect("not poisoned")
        .query(rk_kernel::runtime::journal_query(ADV_ITEM))
        .expect("queries");
    let before = transitions(&prior);
    assert_eq!(
        before,
        vec![
            (None, Some("triage".to_owned())),
            (Some("triage".to_owned()), Some("implement".to_owned())),
        ],
        "the first run's two rows"
    );

    // ---- Run two: SAME inputs, same supplied successor, the first run's journal as `prior`. This
    // is the invocation that used to append `{"from":"implement","to":"implement"}`.
    let second = replay::resume(&inputs, advance_ports(&store), &prior, &mut |n| {
        panic!("no node may be supplied: {n:?}")
    })
    .expect("the second run resumes");

    assert_eq!(
        second.advanced_to(),
        None,
        "the second run must take NO boundary: the fold is already in the successor's stage"
    );

    let after = store
        .lock()
        .expect("not poisoned")
        .query(rk_kernel::runtime::journal_query(ADV_ITEM))
        .expect("queries");
    assert_eq!(
        transitions(&after),
        before,
        "the second run must append NO stage_transition row. A self-transition (`from == to`) is \
         not a transition, and one per run grows without bound"
    );
    assert!(
        !transitions(&after)
            .iter()
            .any(|(f, t)| f.is_some() && f == t),
        "no row may carry `from == to`"
    );
}

/// **ADR-0161: a stage the lane has ENTERED is not entered again — the two-hop case.**
///
/// ADR-0157 guarded the advance arm on `to != state.stage`, and that is a PROXY for *"has this
/// transition already been recorded"*. It is sufficient for a one-hop lane, which is the only lane
/// the shipped tree can walk, so ADR-0157's own experiment could not tell the two guards apart. This
/// fixture can: it walks TWO boundaries and then replays, which is what a second `redkiln run` does.
///
/// Under the `to != state.stage` guard this test appends a **backward**
/// `{"from":"closeout","to":"implement"}` row and then re-advances forward, two tier-1 rows per run
/// for ever — the unbounded append ADR-0157 claimed to have closed, one hop further out.
///
/// Found by this stage's own adversarial pass, not by the fixtures that shipped with ADR-0157.
#[test]
fn adr0161_a_stage_the_lane_has_entered_is_not_entered_again() {
    let pack = Pack::from_toml("lane-advance.toml", LANE_PACK).expect("the fixture pack loads");
    assert_eq!(pack.successor("triage"), Some("implement"));
    assert_eq!(
        pack.successor("implement"),
        Some("closeout"),
        "the fixture must be able to hop TWICE, or it cannot discriminate the guards"
    );

    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(VecStore::default()));
    let refuse = &mut |n: &str| panic!("no node may be supplied: {n:?}");

    // ---- HOP 1, as the lane loop does it: enter at the record's stage, advance.
    let first = replay::resume(
        &advance_inputs_at("triage", Some("implement")),
        advance_ports(&store),
        &[],
        refuse,
    )
    .expect("hop 1");
    assert_eq!(first.advanced_to(), Some("implement"));

    // ---- HOP 2: the loop re-enters at the stage it advanced into, carrying the journal.
    let prior = journal_of(&store);
    let second = replay::resume(
        &advance_inputs_at("implement", Some("closeout")),
        advance_ports(&store),
        &prior,
        refuse,
    )
    .expect("hop 2");
    assert_eq!(
        second.advanced_to(),
        Some("closeout"),
        "the lane takes TWO boundaries"
    );

    let after_two = transitions(&journal_of(&store));
    assert_eq!(
        after_two,
        vec![
            (None, Some("triage".to_owned())),
            (Some("triage".to_owned()), Some("implement".to_owned())),
            (Some("implement".to_owned()), Some("closeout".to_owned())),
        ],
        "one entry row and two forward boundaries"
    );

    // ---- THE NEXT PROCESS. `inputs.stage` is the ITEM RECORD's stage again — it never moves — so
    // the successor handed over is `implement`, while the fold's `state.stage` is now `closeout`.
    // `to != state.stage` is TRUE here, which is exactly why that guard was not enough.
    let third = replay::resume(
        &advance_inputs_at("triage", Some("implement")),
        advance_ports(&store),
        &journal_of(&store),
        refuse,
    )
    .expect("the next process resumes");

    assert_eq!(
        third.advanced_to(),
        None,
        "a stage the lane has already ENTERED must not be entered again: `implement` is in \
         `stages_entered`, even though it differs from the fold's current stage"
    );
    assert_eq!(
        transitions(&journal_of(&store)),
        after_two,
        "no further stage_transition row — and in particular no BACKWARD one"
    );
    assert!(
        !transitions(&journal_of(&store))
            .iter()
            .any(|(f, t)| matches!(
                (f.as_deref(), t.as_deref()),
                (Some("closeout"), Some("implement"))
            )),
        "a backward transition is the specific row the old guard let through"
    );
}
