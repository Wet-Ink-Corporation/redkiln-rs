//! **`s8f` §8f.6 — the run-path `artifact.write` producer, and the read seam re-pointed** (the render
//! record ADR-0189 decisions 7 to 9, proposed; D21-42 5b; D21-43 N12; D21-47). *(Added 2026-09-28,
//! s8f build, package S8F-P4-RENDER-WRITE-READ.)*
//!
//! The `emit` arm stops being inert: it hands each provided kind's bound value to the write seam,
//! and `rk-store`'s writer — bound here as the composition root binds it — writes the sidecar into
//! the item's managed directory and appends that output's ONE `artifact.write` (ADR-0116's forbids:
//! *"an activity output with no `artifact.write` event"*). Every resolved `artifact:` input reaches
//! `rk-store`'s reader through the resolver's one seam call, and the reader appends its ONE
//! `artifact.read`; the kernel appends neither row itself.

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_kernel::catalog::{EmitArtifact, EmitBinding, InputValue, StepKind};
use rk_kernel::runtime::Runtime;
use rk_kernel::runtime::activity::is_completion_row;
use rk_ports::event_store::{EventStore, Query};
use rk_ports::harness::Harness;
use rk_store::render::{
    ArtifactReader, ArtifactWriter, Journal, ManagedDir, RunRefBlob, StageRun, reconcile_entry,
};
use rk_types::digest::sha256;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use serde_json::{Value as Json, json};

use crate::reference_resolver::{activation, emitted_row, refusal, step, with_inputs};
use crate::runtime::{
    CAP, CountingHarness, ITEM, STAGE, at, counters, empty_state, gate_site, journal, ok_typed,
    ports, spec, store,
};

const RUN: &str = "implement-00112233445566aa";
const TREE: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";

type Db = Arc<Mutex<dyn EventStore>>;

/// The store, as `rk-store` is handed it: append one row, read the item's rows.
fn store_journal(db: &Db) -> Journal {
    let (a, r) = (Arc::clone(db), Arc::clone(db));
    Journal::new(
        move |e| {
            a.lock()
                .expect("not poisoned")
                .append(e)
                .map_err(|e| format!("{e:?}"))
        },
        move || {
            r.lock()
                .expect("not poisoned")
                .query(Query::ByItem(ITEM.to_owned()))
                .map_err(|e| format!("{e:?}"))
        },
    )
}

fn stage_run(run: &str) -> StageRun {
    StageRun {
        item: ITEM.to_owned(),
        stage: STAGE.to_owned(),
        run: run.to_owned(),
    }
}

fn schemas() -> BTreeMap<String, String> {
    [("ImplReport", "ImplReport"), ("Spec", "Spec")]
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}

/// An `emit` step providing `ImplReport` (a reference) and `Summary` (a composing table).
fn emit_two_kinds() -> rk_kernel::catalog::Step {
    let mut s = step("emit", StepKind::Emit);
    let mut artifacts = BTreeMap::new();
    artifacts.insert(
        "ImplReport".to_owned(),
        EmitArtifact::Ref("${draft.outputs}".to_owned()),
    );
    let mut members = BTreeMap::new();
    members.insert(
        "doc".to_owned(),
        InputValue::Text("${draft.outputs.doc}".to_owned()),
    );
    artifacts.insert("Summary".to_owned(), EmitArtifact::Compose(members));
    s.emit = Some(EmitBinding {
        artifacts,
        evidence: Vec::new(),
    });
    s
}

/// `[draft (agent), emit]`, walked once over `rt`.
fn walk_emit(
    rt: &Runtime,
    emit: rk_kernel::catalog::Step,
) -> rk_kernel::runtime::activity::ActivityReport {
    let sp = spec();
    let steps = vec![step("draft", StepKind::Agent), emit];
    rt.run_activity_steps(
        activation(&sp),
        Some(&steps),
        &rk_kernel::predicate::Context::new(),
        gate_site(&empty_state()),
        rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
        at(100),
    )
    .expect("a value")
}

fn draft_output() -> Json {
    json!({ "doc": "D", "passed": 3 })
}

fn harness() -> Arc<dyn Harness> {
    Arc::new(CountingHarness::scripted(vec![ok_typed(draft_output())])) as Arc<dyn Harness>
}

fn of_kind(rows: &[Event], kind: EventKind) -> Vec<Event> {
    rows.iter().filter(|e| e.kind == kind).cloned().collect()
}

/// A `reconcile_started` row naming `rk-run/<ITEM>`: the entry's reconcile record.
fn reconcile_started() -> Event {
    Event::new(
        EventKind::ReconcileStarted,
        json!({ "branch": format!("rk-run/{ITEM}"), "worktree": "/run/tree" }),
    )
}

/// **Every activity output appends ONE `artifact.write`** — a referenced kind and a composing
/// table alike — each carrying the stage and run it was written in, and the `rk-types` digest of
/// the sidecar bytes now in the managed directory, which is the committed binding's canonical
/// digest. The writes precede the completion row, whose landing commits them (N10).
#[test]
fn every_activity_output_appends_one_artifact_write() {
    let (q, a) = counters();
    let db = store(&q, &a);
    let dir = tempfile::tempdir().expect("a temp dir");
    let writer = ArtifactWriter::new(dir.path(), stage_run(RUN), schemas(), store_journal(&db));
    let rt = Runtime::new(ports(harness(), &db), RUN, STAGE).with_write_seam(Arc::new(writer));

    let report = walk_emit(&rt, emit_two_kinds());
    assert!(
        report.committed(),
        "the walk completed: {}",
        refusal(&report)
    );

    let rows = journal(&db);
    let writes = of_kind(&rows, EventKind::ArtifactWrite);
    assert_eq!(
        writes
            .iter()
            .map(|w| w.payload["kind"].as_str().unwrap_or_default().to_owned())
            .collect::<Vec<_>>(),
        vec!["ImplReport".to_owned(), "Summary".to_owned()],
        "one `artifact.write` per activity output: {writes:?}"
    );
    let completion = rows
        .iter()
        .position(is_completion_row)
        .expect("the completion row");
    for w in &writes {
        let kind = w.payload["kind"].as_str().expect("a kind");
        assert_eq!(
            (&w.payload["item"], &w.payload["stage"], &w.payload["run"]),
            (&json!(ITEM), &json!(STAGE), &json!(RUN)),
            "`{kind}`'s row names its stage run"
        );
        let bytes = ManagedDir::of(dir.path(), ITEM)
            .read(&format!("{kind}.json"))
            .unwrap_or_else(|| panic!("`{kind}`'s sidecar is not in the managed directory"));
        assert_eq!(w.payload["digest"], json!(sha256(&bytes).as_str()));
        let bound = &rows[completion].payload["artifacts"][kind];
        assert_eq!(
            w.payload["digest"],
            json!(
                rk_store::canon::canonical_digest(bound)
                    .expect("canonical")
                    .as_str()
            ),
            "`{kind}`'s written digest is not the committed binding's"
        );
        let at = rows.iter().position(|e| e == w).expect("the row");
        assert!(
            at < completion,
            "`{kind}` was written after the completion row"
        );
    }
}

/// **An `emit` replayed writes nothing and serves its recorded digest** (ADR-0189 decision 7):
/// over a replaying writer and a journal holding the output's recorded `artifact.write`, the walk
/// completes, no file is written and no row is appended; with no recorded row, the `emit` is
/// refused by name and the activation does not complete.
#[test]
fn an_emit_replayed_writes_nothing_and_serves_its_recorded_digest() {
    let emit_one = || {
        let mut s = step("emit", StepKind::Emit);
        let mut artifacts = BTreeMap::new();
        artifacts.insert(
            "ImplReport".to_owned(),
            EmitArtifact::Ref("${draft.outputs}".to_owned()),
        );
        s.emit = Some(EmitBinding {
            artifacts,
            evidence: Vec::new(),
        });
        s
    };
    let recorded = rk_store::canon::canonical_digest(&draft_output()).expect("canonical");

    for seeded in [true, false] {
        let (q, a) = counters();
        let db = store(&q, &a);
        if seeded {
            db.lock()
                .expect("ok")
                .append(
                    Event::new(
                        EventKind::ArtifactWrite,
                        json!({
                            "digest": recorded.as_str(),
                            "item": ITEM,
                            "kind": "ImplReport",
                            "path": format!(".redkiln/artifacts/{ITEM}/ImplReport.json"),
                            "run": RUN,
                            "schema_version": "ImplReport",
                            "stage": STAGE,
                        }),
                    )
                    .with_item(ITEM),
                )
                .expect("seeded");
        }
        let before = of_kind(&journal(&db), EventKind::ArtifactWrite).len();
        let dir = tempfile::tempdir().expect("a temp dir");
        let writer =
            ArtifactWriter::replaying(dir.path(), stage_run(RUN), schemas(), store_journal(&db));
        let rt = Runtime::new(ports(harness(), &db), RUN, STAGE).with_write_seam(Arc::new(writer));

        let report = walk_emit(&rt, emit_one());
        let rows = journal(&db);
        assert_eq!(
            of_kind(&rows, EventKind::ArtifactWrite).len(),
            before,
            "(seeded: {seeded}) replay appended an `artifact.write`"
        );
        assert!(
            !dir.path().join(".redkiln").exists(),
            "(seeded: {seeded}) replay wrote into the tree"
        );
        if seeded {
            assert!(
                report.committed(),
                "the replayed emit completed: {}",
                refusal(&report)
            );
            let completion = rows
                .iter()
                .find(|e| is_completion_row(e))
                .expect("completion");
            assert_eq!(
                rk_store::canon::canonical_digest(&completion.payload["artifacts"]["ImplReport"])
                    .expect("canonical"),
                recorded,
                "the replayed binding is not the recorded digest's value"
            );
        } else {
            assert!(
                !rows.iter().any(is_completion_row),
                "an emit replay has no record of was completed anyway"
            );
            assert!(
                refusal(&report).contains("replay found no recorded row"),
                "the unrecorded emit was not refused by name: {}",
                refusal(&report)
            );
        }
    }
}

/// **Every resolved `artifact:` input appends ONE `artifact.read`** — through the resolver's one
/// seam call, to `rk-store`'s reader, which appends it naming the kind, the item, the READING stage
/// run, the tree it served and the digest of the bytes served; an optional input that resolves
/// `null` appends none. A served digest that is not the committed binding's halts the input by
/// name and appends nothing. Red under M25 (the read seam appends no `artifact.read`).
///
/// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-113 step 3 as amended by D21-114, both
/// TAKEN PROVISIONALLY under D21-105.)* ~~A served digest that is not the committed binding's
/// halts~~: the reader checks the LATEST recorded `artifact.write` of the sidecar's path, and the
/// binding only when there is none. So the halt arm no longer writes its other bytes THROUGH the
/// writer (a recorded write, which is now served); it writes the emitted value through the writer
/// and then puts other bytes at the sidecar's path OUT OF BAND, which no recorded write accounts
/// for: still Halt 4, by name, with nothing appended.
#[test]
fn every_artifact_input_resolution_appends_one_artifact_read() {
    for matching in [true, false] {
        let (q, a) = counters();
        let db = store(&q, &a);
        let v = json!({ "acs": ["AC-1"], "title": "the spec" });
        {
            let mut g = db.lock().expect("ok");
            g.append(emitted_row("spec-author@1", "Spec", v.clone()))
                .expect("seeded");
            g.append(reconcile_started()).expect("seeded");
        }
        let dir = tempfile::tempdir().expect("a temp dir");
        // The producer's sidecar on the run ref: what it emitted, or (the halt arm) other bytes.
        let producer = ArtifactWriter::new(
            dir.path(),
            StageRun {
                item: ITEM.to_owned(),
                stage: "design".to_owned(),
                run: "design-0000000000000000".to_owned(),
            },
            schemas(),
            store_journal(&db),
        );
        producer.write("Spec", &v).expect("the sidecar is written");
        let sidecar = ManagedDir::of(dir.path(), ITEM)
            .read("Spec.json")
            .expect("the sidecar");
        let pass = reconcile_entry(&producer, &|_| Ok(None)).expect("the entry's pass");
        if !matching {
            // The halt arm: other bytes on the run ref, OUT OF BAND (no recorded write).
            let other = json!({ "acs": [], "title": "not what was emitted" });
            std::fs::write(
                dir.path()
                    .join(rk_store::render::managed_path(ITEM, "Spec.json")),
                serde_json::to_vec(&other).expect("serialises"),
            )
            .expect("the out-of-band edit is written");
        }
        let root = dir.path().to_path_buf();
        let reader = ArtifactReader::new(
            &pass,
            stage_run(RUN),
            schemas(),
            store_journal(&db),
            move |path: &str| {
                Ok(std::fs::read(Path::new(&root).join(path))
                    .ok()
                    .map(|bytes| RunRefBlob {
                        bytes,
                        tree: TREE.to_owned(),
                    }))
            },
        );
        let rt = Runtime::new(ports(harness(), &db), RUN, STAGE).with_read_seam(Arc::new(reader));
        let sp = spec();
        let steps = vec![with_inputs(
            step("draft", StepKind::Agent),
            &[
                ("spec", "artifact:Spec"),
                ("acs", "artifact:Spec.acs"),
                ("plan", "artifact:Plan?"),
            ],
        )];
        let report = rt
            .run_activity_steps(
                activation(&sp),
                Some(&steps),
                &rk_kernel::predicate::Context::new(),
                gate_site(&empty_state()),
                rk_kernel::workflow::envelope::AttemptCap::declared(CAP),
                at(100),
            )
            .expect("a value");
        let reads = of_kind(&journal(&db), EventKind::ArtifactRead);
        if matching {
            assert!(
                report.committed(),
                "the walk dispatched: {}",
                refusal(&report)
            );
            assert_eq!(
                reads.len(),
                2,
                "two resolved inputs, two `artifact.read` rows; the null optional none: {reads:?}"
            );
            for r in &reads {
                assert_eq!(
                    r.payload,
                    json!({
                        "digest": sha256(&sidecar).as_str(),
                        "item": ITEM,
                        "kind": "Spec",
                        "run": RUN,
                        "schema_version": "Spec",
                        "stage": STAGE,
                        "tree": TREE,
                    })
                );
            }
        } else {
            assert!(
                refusal(&report).contains("Halt 4"),
                "a served digest that is not the binding's was not halted by name: {}",
                refusal(&report)
            );
            assert!(reads.is_empty(), "a halted read appended a row: {reads:?}");
        }
    }
}
