//! **`RK-D-DRIFT-KIND` — AC-127's doctor row: the two drift kinds** (`s8f` §8f.9; D21-46 R7; the
//! render record ADR-0189 decision 13, proposed). *(Added 2026-09-29, s8f build, package
//! S8F-P8-DOCTOR-RULES-AND-NOTES.)*
//!
//! AC-127's *Verified by*: *"a `doctor` rule-table row distinguishing the two drift kinds"*. The
//! rule reads the store's journal in append order and takes the blocking property from
//! `rk_types::drift::Drift::blocks()` — never from a second table. Two crossings are reported:
//! - **(a)** a `drift_advisory` followed, in the same entry, by a `render.structured-drift` gate
//!   naming its root: an advisory promoted to a blocking gate (ADR-0025's first forbids bullet;
//!   AC-067);
//! - **(b)** a `structured_drift` with no `render.structured-drift` gate naming its artifact opened
//!   after it in the same entry: a blocking drift that did not block (N11).
//!
//! One named test per arm, because a fixture per function is not a fixture per branch, and a
//! control over a REAL lane journal that carries both kinds, each handled correctly. The in-process
//! rows are built by the producers' own constructors (`Advisory::own_change(..).to_event()`,
//! `drift_gate_act`, `gate_opened_event`); only `reconcile_started` and `structured_drift`, whose
//! constructors are private to `rk-cli` and `rk-store`, are spelled here in their producers' shape.

use std::path::Path;

use rk_cli::doctor::rules::RULES;
use rk_cli::doctor::{Ctx, Finding, JournalSource};
use rk_cli::exit::Exit;
use rk_kernel::runtime::tier_one::{DRIFT_KIND, drift_gate_act, drift_gate_record};
use rk_types::digest::{Digest, sha256};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

use crate::lane_e2e::{self, GATE_ITEM, ScriptedFactory};
use crate::{landing, support};

const RULE: &str = "RK-D-DRIFT-KIND";
const ITEM: &str = "RK-S0042";
const NOW: i64 = 1_800_000_000;
const TREE: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

struct Pinned;

impl rk_cli::doctor::probe::VersionProbe for Pinned {
    fn version(&self, _program: &str) -> Result<String, rk_cli::doctor::probe::ProbeError> {
        Ok(rk_adapter_claude::config::TESTED_CLI_VERSION.to_owned())
    }
}

/// `Ctx::build` over `root`, the verb's one builder, with the store the root would hand in.
fn ctx(root: &Path, source: JournalSource) -> Ctx {
    let config_src = std::fs::read_to_string(root.join(".redkiln").join("config.toml"))
        .expect("the repository carries a .redkiln/config.toml");
    let config = rk_cli::config::parse(&config_src).expect("the ConfigFile parses");
    let repo = rk_cli::repo::discover(Some(root)).expect("the repository discovers");
    let store = Some(rk_cli::adapters::open_store(&repo.exec, &repo.worktree));
    Ctx::build(
        root.to_path_buf(),
        repo.common_dir,
        config,
        support::clean_env(),
        store,
        Box::new(Pinned),
        NOW,
        source,
    )
}

fn fire(ctx: &Ctx) -> Vec<Finding> {
    let rule = RULES
        .iter()
        .find(|r| r.id == RULE)
        .unwrap_or_else(|| panic!("{RULE} is in RULES"));
    (rule.check)(ctx)
}

/// The rule over `rows`, dumped verbatim to `events.jsonl` in a fresh `clean` checkout.
fn fire_over(rows: &[Event]) -> Vec<Finding> {
    let dir = crate::doctor::materialise("clean");
    let mut jsonl = String::new();
    for e in rows {
        jsonl.push_str(
            &serde_json::json!({ "kind": e.kind.wire_name(), "payload": e.payload }).to_string(),
        );
        jsonl.push('\n');
    }
    std::fs::write(dir.path().join("events.jsonl"), jsonl).expect("events.jsonl");
    fire(&ctx(dir.path(), JournalSource::FixtureEventsJsonl))
}

/// An entry's first row, in the shape `run`'s `reconcile_started_event` writes it.
fn entry() -> Event {
    Event::new(
        EventKind::ReconcileStarted,
        serde_json::json!({ "branch": format!("rk-run/{ITEM}") }),
    )
}

fn digest(text: &str) -> Digest {
    sha256(text.as_bytes())
}

/// A vouched root's own-change advisory, through the kernel's own constructor.
fn advisory(root: &str) -> Event {
    rk_kernel::runtime::advisory::Advisory::own_change(root, digest("vouched"), digest("observed"))
        .to_event()
}

/// A `structured_drift` row, in `rk-store`'s `drift_row` shape.
fn structured(kind: &str, anchor: &str) -> Event {
    Event::new(
        EventKind::StructuredDrift,
        serde_json::json!({
            "anchor": anchor,
            "causing_commit": null,
            "item": ITEM,
            "kind": kind,
            "new_digest": digest("on disk").as_str(),
            "old_digest": digest("rendered").as_str(),
        }),
    )
    .with_item(ITEM)
}

/// A `render.structured-drift` gate's `gate_opened` row naming `(kind, anchor)`, through the
/// kernel's own `drift_gate_act`.
fn drift_gate(kind: &str, anchor: &str, reach: u32) -> Event {
    let record = drift_gate_record(TREE, Timestamp::from_unix_seconds(NOW + 86_400), reach)
        .expect("the drift gate's record");
    let act = drift_gate_act(&record, "implement", vec![structured(kind, anchor).payload])
        .expect("the drift gate's act");
    let rows: Vec<Event> = act
        .rows
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .collect();
    assert_eq!(
        rows.len(),
        1,
        "PREMISE: the act carries one gate_opened row"
    );
    rows.into_iter().next().expect("one row")
}

/// A gate of ANOTHER class (the refused landing's), which may carry an advisory (ADR-0025).
fn landing_gate() -> Event {
    let record = rk_kernel::workflow::fsm::GateRecord::derive_reach(
        "landing.refused-path",
        TREE,
        Timestamp::from_unix_seconds(NOW + 86_400),
        1,
    )
    .expect("the landing gate's record");
    rk_kernel::runtime::gate_opened_event(&record)
}

fn of_arm<'a>(found: &'a [Finding], kind: &str) -> Vec<&'a Finding> {
    found
        .iter()
        .filter(|f| f.detail.contains(&format!("`{kind}`")))
        .collect()
}

// ------------------------------------------------------------------------------- the tests

/// **Arm (a)**: a `drift_advisory` followed in the SAME entry by a `render.structured-drift` gate
/// naming its root is reported — one finding, of this rule, at `Error`, naming the drift kind, the
/// root and both digests. Controls: the gate in the NEXT entry, and a gate of another class after
/// the advisory in the same entry (the next gate CARRYING it, which ADR-0025 requires), are clean.
/// The committed fixture `doctor/RK-D-DRIFT-KIND/` trips this arm too.
///
/// Red by `M41` (arm (a) removed).
#[test]
fn an_advisory_that_opened_a_blocking_gate_is_reported() {
    let found = fire_over(&[
        entry(),
        advisory("Findings"),
        drift_gate("Findings", "document", 1),
    ]);
    assert_eq!(found.len(), 1, "arm (a): {found:#?}");
    let f = &found[0];
    assert_eq!(f.id, RULE);
    assert_eq!(f.severity, rk_cli::doctor::Severity::Error);
    for needle in [
        "`drift_advisory`",
        "`Findings`",
        digest("vouched").as_str(),
        digest("observed").as_str(),
    ] {
        assert!(
            f.detail.contains(needle),
            "the finding does not name {needle}: {}",
            f.detail
        );
    }

    assert!(
        fire_over(&[
            entry(),
            advisory("Findings"),
            entry(),
            drift_gate("Findings", "document", 1)
        ])
        .is_empty(),
        "CONTROL: a drift gate in a LATER entry is not this advisory's promotion"
    );
    assert!(
        fire_over(&[entry(), advisory("Findings"), landing_gate()]).is_empty(),
        "CONTROL: the next gate of another class carrying the advisory is ADR-0025's rule, not a \
         promotion"
    );

    let dir = crate::doctor::materialise(RULE);
    let fixture = fire(&ctx(dir.path(), JournalSource::FixtureEventsJsonl));
    assert_eq!(
        of_arm(&fixture, "drift_advisory").len(),
        1,
        "the committed fixture trips arm (a): {fixture:#?}"
    );
}

/// **Arm (b)**: a `structured_drift` with no `render.structured-drift` gate naming its artifact
/// opened after it in the same entry is reported — one finding naming the drift kind, the artifact
/// and anchor, and both digests. Each non-answer is its own row: no gate; a drift gate in the NEXT
/// entry; a drift gate in the same entry naming another SECTION of it, or another ARTIFACT KIND at
/// the same anchor; a drift gate BEFORE the drift.
/// The control is the gate naming it after it in the same entry.
///
/// Red by `M42` (arm (b) removed).
#[test]
fn a_structured_drift_with_no_drift_gate_is_reported() {
    let rows: [(&str, Vec<Event>); 5] = [
        ("no gate", vec![entry(), structured("Spec", "acceptance")]),
        (
            "the gate in the next entry",
            vec![
                entry(),
                structured("Spec", "acceptance"),
                entry(),
                drift_gate("Spec", "acceptance", 1),
            ],
        ),
        (
            "a gate naming another section of it",
            vec![
                entry(),
                structured("Spec", "acceptance"),
                drift_gate("Spec", "scope", 1),
            ],
        ),
        (
            "a gate naming another artifact at the same anchor",
            vec![
                entry(),
                structured("Spec", "acceptance"),
                drift_gate("Plan", "acceptance", 1),
            ],
        ),
        (
            "a gate before the drift",
            vec![
                entry(),
                drift_gate("Spec", "acceptance", 1),
                structured("Spec", "acceptance"),
            ],
        ),
    ];
    for (arm, journal) in rows {
        let found = fire_over(&journal);
        assert_eq!(found.len(), 1, "arm (b), {arm}: {found:#?}");
        let f = &found[0];
        assert_eq!(f.id, RULE);
        assert_eq!(f.severity, rk_cli::doctor::Severity::Error);
        for needle in [
            "`structured_drift`",
            "`Spec`",
            "`acceptance`",
            digest("rendered").as_str(),
            digest("on disk").as_str(),
        ] {
            assert!(
                f.detail.contains(needle),
                "arm (b), {arm}: the finding does not name {needle}: {}",
                f.detail
            );
        }
    }
    assert!(
        fire_over(&[
            entry(),
            structured("Spec", "acceptance"),
            drift_gate("Spec", "acceptance", 1)
        ])
        .is_empty(),
        "CONTROL: the drift gate naming it, after it, in the same entry"
    );
}

/// **The control: a REAL lane journal that carries both kinds, each handled correctly** (§8f.9).
/// One item over the drift lane (`Spec` emitted at `implement`, a last stage `wrapup`) whose
/// abstention gate a pack question answers with `vouch`, attaching `Findings`:
/// - invocation 1 opens the abstention gate; `answer … vouch` attaches the root;
/// - invocation 2 stores the root at its entry, and `implement`'s step edits it and emits `Spec`;
/// - a human's edit of `Spec`'s structured section is committed on the run ref;
/// - invocation 3's ONE entry records BOTH kinds: the root's own change as `drift_advisory` (no
///   gate) and the `Spec` edit as `structured_drift`, restored and gated by one
///   `render.structured-drift` gate naming `Spec`.
///
/// Read from the STORE (the verb's source), the rule reports nothing: the advisory opened no gate
/// naming its root, and the structured drift has its gate. The premises pin that both kinds sit in
/// the same entry, so the rule is discriminating by kind and by artifact, not by entry alone.
#[test]
fn a_lane_journal_carrying_both_kinds_correctly_is_clean() {
    const VOUCHED: &[u8] = b"{\"items\":[]}\n";
    const EDITED: &[u8] = b"{\"items\":[\"a finding added after vouching\"]}\n";
    let repo = landing::drift_lane();
    let root = repo.path();
    let item = GATE_ITEM;
    let processes = root.join(".redkiln").join("processes");
    std::fs::create_dir_all(&processes).expect("the processes directory");
    std::fs::write(
        processes.join("vouch.toml"),
        r#"[question."unknown_facet:has_repro"]
text = "Does the item reproduce? `vouch` attaches the human's findings."
options = ["approve", { id = "vouch", attaches = "Findings" }]
"#,
    )
    .expect("the question is written");

    assert_eq!(
        landing::run(root, item, &ScriptedFactory::new(Vec::new())),
        Exit::Success,
        "PREMISE: the abstention gate opens"
    );
    landing::write(root, "notes/brief.json", VOUCHED);
    assert_eq!(
        landing::answer(
            root,
            item,
            "vouch",
            Some(("Findings", root.join("notes/brief.json")))
        ),
        Exit::Success,
        "PREMISE: the root is attached"
    );
    let run_tree = crate::run_tree(root, item).path;
    let edit_rel = format!(".redkiln/artifacts/{item}/Findings.root.json");
    let second = landing::writing(&run_tree, vec![lane_e2e::implement()], move |n, tree| {
        if n == 0 {
            landing::write(tree, &edit_rel, EDITED);
        }
    });
    landing::run(root, item, &second);
    landing::human_edits_the_spec(root, item);
    landing::run(root, item, &ScriptedFactory::new(vec![lane_e2e::review()]));

    // ---- the premises: both kinds, in ONE entry, each handled correctly.
    let rows = support::store_rows(root);
    let entries: Vec<usize> = rows
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::ReconcileStarted)
        .map(|(i, _)| i)
        .collect();
    let entry_of = |at: usize| entries.iter().rposition(|&e| e < at);
    let at = |kind: EventKind| -> Vec<usize> {
        rows.iter()
            .enumerate()
            .filter(|(_, e)| e.kind == kind)
            .map(|(i, _)| i)
            .collect()
    };
    let drifts = at(EventKind::StructuredDrift);
    let advisories = at(EventKind::DriftAdvisory);
    let gates: Vec<usize> = at(EventKind::GateOpened)
        .into_iter()
        .filter(|&i| {
            rows[i]
                .payload
                .get("kind")
                .and_then(serde_json::Value::as_str)
                == Some(DRIFT_KIND)
        })
        .collect();
    assert_eq!(drifts.len(), 1, "PREMISE: one structured_drift: {drifts:?}");
    assert_eq!(
        advisories.len(),
        1,
        "PREMISE: one drift_advisory: {advisories:?}"
    );
    assert_eq!(gates.len(), 1, "PREMISE: one drift gate: {gates:?}");
    assert_eq!(
        entry_of(drifts[0]),
        entry_of(advisories[0]),
        "PREMISE: both kinds were recorded in ONE entry"
    );
    assert_eq!(
        entry_of(gates[0]),
        entry_of(drifts[0]),
        "PREMISE: the drift gate opened in that entry"
    );
    assert!(gates[0] > drifts[0], "PREMISE: after the drift");

    let found = fire(&ctx(root, JournalSource::Store));
    assert!(
        found.is_empty(),
        "a lane journal carrying both kinds, each handled correctly, is reported: {found:#?}"
    );
}
