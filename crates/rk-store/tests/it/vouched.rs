//! **`s8f` §8f.5 — AC-127's vouched-root half** (`rk_store::vouched`; the render record ADR-0189
//! decision 10, proposed; D21-46 R7). *(Added 2026-09-28, s8f build, package
//! S8F-P5-DRIFT-GATE-VOUCHED.)*
//!
//! Every test runs over a temporary checkout (where the attached file lives), a temporary
//! run-owned tree (where the store puts the root) and an in-memory journal. The run's advise
//! function is this file's [`Run`]: the composition root binds the kernel's `AdvisorySink` there,
//! and `rk-store` depends on no kernel crate, so the test binds a sink of the same shape — one
//! memory per run, keyed on `(root, null, new_digest)` — and a new [`Run`] is a new run.
//!
//! What is read: the rows the journal holds and the bytes on disk. No assertion reads a value the
//! product computed for the test, except a digest recomputed here from the bytes.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_store::render::{self, ArtifactWriter, Journal, RenderError, StageRun};
use rk_store::vouched::{self, Admitted};
use rk_types::digest::{Digest, sha256};
use rk_types::drift::Drift;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use serde_json::{Value as Json, json};

const ITEM: &str = "item-1";
const STAGE: &str = "implement";
const RUN: &str = "implement-0123456789abcdef";
const KIND: &str = "Findings";
const ATTACHED: &str = "notes/brief.json";
const VOUCHED: &[u8] = b"{\"brief\":\"what the human supplied\"}\n";
const EDITED: &[u8] = b"{\"brief\":\"edited on the run ref after vouching\"}\n";
const EDITED_AGAIN: &[u8] = b"{\"brief\":\"edited a second time\"}\n";

type Rows = Arc<Mutex<Vec<Event>>>;

fn rows() -> Rows {
    Arc::new(Mutex::new(Vec::new()))
}

fn journal_over(rows: &Rows) -> Journal {
    let (a, r) = (Arc::clone(rows), Arc::clone(rows));
    Journal::new(
        move |e| {
            a.lock().expect("not poisoned").push(e);
            Ok(())
        },
        move || Ok(r.lock().expect("not poisoned").clone()),
    )
}

fn writer(tree: &Path, rows: &Rows) -> ArtifactWriter {
    ArtifactWriter::new(
        tree,
        StageRun {
            item: ITEM.to_owned(),
            stage: STAGE.to_owned(),
            run: RUN.to_owned(),
        },
        [(KIND, KIND), ("Spec", "Spec")]
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect::<BTreeMap<_, _>>(),
        journal_over(rows),
    )
}

/// `answer`'s attach row, in ADR-0189 decision 8's shape.
fn attach_row(digest: &Digest) -> Event {
    Event::new(
        EventKind::ArtifactWrite,
        json!({
            "attached_path": ATTACHED,
            "digest": digest.as_str(),
            "item": ITEM,
            "kind": KIND,
            "run": "implement-fedcba9876543210",
            "schema_version": KIND,
            "stage": STAGE,
        }),
    )
    .with_item(ITEM)
}

/// A checkout holding the attached file, a run-owned tree, and a journal holding the attach row.
struct Fixture {
    checkout: tempfile::TempDir,
    tree: tempfile::TempDir,
    rows: Rows,
}

fn fixture_with(file: Option<&[u8]>, vouched: &[u8]) -> Fixture {
    let checkout = tempfile::tempdir().expect("a checkout");
    if let Some(bytes) = file {
        let at = checkout.path().join(ATTACHED);
        std::fs::create_dir_all(at.parent().expect("a parent")).expect("the directory");
        std::fs::write(&at, bytes).expect("the attached file is written");
    }
    let tree = tempfile::tempdir().expect("a run-owned tree");
    let rows = rows();
    rows.lock()
        .expect("not poisoned")
        .push(attach_row(&sha256(vouched)));
    Fixture {
        checkout,
        tree,
        rows,
    }
}

fn fixture() -> Fixture {
    fixture_with(Some(VOUCHED), VOUCHED)
}

fn root_rel() -> String {
    render::managed_path(ITEM, "Findings.root.json")
}

fn root_bytes(f: &Fixture) -> Option<Vec<u8>> {
    std::fs::read(f.tree.path().join(root_rel())).ok()
}

/// A human's edit of the stored root, as the run ref carries it after a commit.
fn edit_root(f: &Fixture, bytes: &[u8]) {
    std::fs::write(f.tree.path().join(root_rel()), bytes).expect("the root is edited");
}

fn count(rows: &Rows, kind: EventKind) -> usize {
    rows.lock()
        .expect("not poisoned")
        .iter()
        .filter(|e| e.kind == kind)
        .count()
}

fn of_kind(rows: &Rows, kind: EventKind) -> Vec<Event> {
    rows.lock()
        .expect("not poisoned")
        .iter()
        .filter(|e| e.kind == kind)
        .cloned()
        .collect()
}

/// **One run's advise function**: the `AdvisorySink`'s shape — one memory per run, keyed on the
/// triple `(root, upstream = null, new_digest)` — and the kernel's four-field row.
#[derive(Default)]
struct Run {
    seen: BTreeSet<(String, Option<String>, String)>,
}

impl Run {
    fn advise(&mut self, drift: &Drift) -> Result<Admitted, String> {
        let Drift::Advisory {
            root,
            vouched,
            observed,
        } = drift
        else {
            return Err(format!("not an own-change advisory: {drift:?}"));
        };
        let first_in_run = self
            .seen
            .insert((root.clone(), None, observed.as_str().to_owned()));
        Ok(Admitted {
            row: Event::new(
                EventKind::DriftAdvisory,
                json!({
                    "new_digest": observed.as_str(),
                    "old_digest": vouched.as_str(),
                    "root": root,
                    "upstream": null,
                }),
            )
            .with_item(ITEM),
            first_in_run,
        })
    }

    fn pass(&mut self, f: &Fixture) -> Result<vouched::VouchedPass, RenderError> {
        let w = writer(f.tree.path(), &f.rows);
        vouched::vouched_pass(&w, f.checkout.path(), &mut |d: &Drift| self.advise(d))
    }
}

/// The first entry after the attach: the root is stored. A PREMISE of every later arm.
fn stored(f: &Fixture) {
    let pass = Run::default()
        .pass(f)
        .expect("PREMISE: the attached root is stored");
    assert_eq!(
        pass.stored().len(),
        1,
        "PREMISE: one root stored at the entry after the attach"
    );
    assert_eq!(
        root_bytes(f).as_deref(),
        Some(VOUCHED),
        "PREMISE: the stored root holds the vouched bytes"
    );
}

// ------------------------------------------------------------------------------- the tests

/// **AC-127's *Verified by*, whole**: *"a table-driven test over {vouched root, rendered section} ×
/// {changed, unchanged} asserting the four outcomes"*. One row per arm, each measured by the rows
/// the journal gained and the bytes on disk:
///
/// | arm | `structured_drift` | `drift_advisory` | blocks | the file |
/// | --- | --- | --- | --- | --- |
/// | root changed | 0 | 1, `upstream: null` | no | kept as edited |
/// | root unchanged | 0 | 0 | no | untouched |
/// | section changed | 1 | 0 | **yes** | restored from the type |
/// | section unchanged | 0 | 0 | no | untouched |
///
/// Red under M36 (the root's change classed blocking) and M37 (the root's change appends nothing),
/// each on the root-changed row.
#[test]
fn ac127_four_outcomes_over_root_and_section_by_changed_and_unchanged() {
    #[derive(Debug, PartialEq, Eq)]
    struct Outcome {
        structured_drift: usize,
        drift_advisory: usize,
        blocks: bool,
        file_kept: bool,
    }

    // ---- root, changed and unchanged ----------------------------------------------------------
    let root = |changed: bool| -> Outcome {
        let f = fixture();
        stored(&f);
        if changed {
            edit_root(&f, EDITED);
        }
        let expected_file = if changed { EDITED } else { VOUCHED };
        let (sd, da) = (
            count(&f.rows, EventKind::StructuredDrift),
            count(&f.rows, EventKind::DriftAdvisory),
        );
        let pass = Run::default().pass(&f).expect("the pass runs");
        if changed {
            let rows = of_kind(&f.rows, EventKind::DriftAdvisory);
            if let Some(row) = rows.last() {
                let mut keys: Vec<&str> = row
                    .payload
                    .as_object()
                    .map(|o| o.keys().map(String::as_str).collect())
                    .unwrap_or_default();
                keys.sort_unstable();
                assert_eq!(keys, ["new_digest", "old_digest", "root", "upstream"]);
                assert_eq!(row.payload["upstream"], Json::Null, "the own-change mark");
                assert_eq!(row.payload["root"], json!(KIND));
                assert_eq!(row.payload["old_digest"], json!(sha256(VOUCHED).as_str()));
                assert_eq!(row.payload["new_digest"], json!(sha256(EDITED).as_str()));
            }
        }
        Outcome {
            structured_drift: count(&f.rows, EventKind::StructuredDrift) - sd,
            drift_advisory: count(&f.rows, EventKind::DriftAdvisory) - da,
            blocks: !pass.blocking().is_empty(),
            file_kept: root_bytes(&f).as_deref() == Some(expected_file),
        }
    };

    // ---- a rendered section, changed and unchanged --------------------------------------------
    let section = |changed: bool| -> Outcome {
        let tree = tempfile::tempdir().expect("a run-owned tree");
        let rows = rows();
        let w = writer(tree.path(), &rows);
        let value = json!({
            "id": "SPEC-1",
            "status": "draft",
            "title": "The spec",
            "acceptance": [{ "criterion": "the store renders", "id": "AC-1" }],
            "context": "",
            "notes": "",
        });
        w.write("Spec", &value)
            .expect("PREMISE: the spec is written");
        let md = tree.path().join(render::managed_path(ITEM, "Spec.md"));
        let rendered = std::fs::read(&md).expect("PREMISE: the rendering exists");
        if changed {
            let text = String::from_utf8(rendered.clone()).expect("utf-8");
            let edited = text.replace("the store renders", "a human edited the criterion");
            assert_ne!(edited, text, "PREMISE: the structured section is edited");
            std::fs::write(&md, edited).expect("the rendering is edited");
        }
        let (sd, da) = (
            count(&rows, EventKind::StructuredDrift),
            count(&rows, EventKind::DriftAdvisory),
        );
        let done = render::reconcile_entry(&w, &|_: &str| Ok(Some("c0ffee".to_owned())))
            .expect("the entry pass runs");
        let blocks = done.drifts().iter().any(|d| {
            Drift::StructuredSection {
                artifact: d.kind.clone(),
                anchor: d.anchor.clone(),
                stored: d.old_digest.clone(),
                observed: d.new_digest.clone(),
            }
            .blocks()
        });
        Outcome {
            structured_drift: count(&rows, EventKind::StructuredDrift) - sd,
            drift_advisory: count(&rows, EventKind::DriftAdvisory) - da,
            blocks,
            // A structured edit is RESTORED from the type: the file is the rendering again.
            file_kept: std::fs::read(&md).expect("the rendering exists") == rendered,
        }
    };

    let table = [
        (
            "root changed",
            root(true),
            Outcome {
                structured_drift: 0,
                drift_advisory: 1,
                blocks: false,
                file_kept: true,
            },
        ),
        (
            "root unchanged",
            root(false),
            Outcome {
                structured_drift: 0,
                drift_advisory: 0,
                blocks: false,
                file_kept: true,
            },
        ),
        (
            "section changed",
            section(true),
            Outcome {
                structured_drift: 1,
                drift_advisory: 0,
                blocks: true,
                file_kept: true,
            },
        ),
        (
            "section unchanged",
            section(false),
            Outcome {
                structured_drift: 0,
                drift_advisory: 0,
                blocks: false,
                file_kept: true,
            },
        ),
    ];
    for (arm, got, want) in table {
        assert_eq!(got, want, "AC-127's `{arm}` arm");
    }
}

/// **The root is stored at the next entry under its vouched digest** (ADR-0129 decision 4's
/// *"accepted and stored as a human-supplied root"*; ADR-0189 decision 10). The matching arm
/// stores the attached bytes as `<Kind>.root<ext>` with ONE `artifact.write` whose digest is the
/// vouched one, and a second entry stores nothing again. The mismatching arm (the file's bytes are
/// not the ones vouched) is refused by name — the kind, the path and both digests — and nothing
/// is stored; so is the missing-file arm. Red under M40 (the digest check dropped).
#[test]
fn the_attached_root_is_stored_at_the_next_entry_under_its_vouched_digest() {
    // ---- the matching arm -----------------------------------------------------------------------
    let f = fixture();
    let pass = Run::default().pass(&f).expect("the root is stored");
    assert_eq!(pass.stored().len(), 1, "one root stored");
    assert_eq!(root_bytes(&f).as_deref(), Some(VOUCHED));
    let writes: Vec<Event> = of_kind(&f.rows, EventKind::ArtifactWrite)
        .into_iter()
        .filter(|e| e.payload.get("attached_path").is_none())
        .collect();
    assert_eq!(writes.len(), 1, "ONE `artifact.write` for the store");
    assert_eq!(writes[0].payload["path"], json!(root_rel()));
    assert_eq!(writes[0].payload["kind"], json!(KIND));
    assert_eq!(
        writes[0].payload["digest"],
        json!(sha256(VOUCHED).as_str()),
        "the stored root's digest is the VOUCHED digest"
    );
    let again = Run::default().pass(&f).expect("a second entry");
    assert!(
        again.stored().is_empty(),
        "a stored root is not stored again"
    );
    assert_eq!(
        of_kind(&f.rows, EventKind::ArtifactWrite).len(),
        2,
        "the attach row and the one store"
    );

    // ---- the mismatching arm: the file is not what was vouched ---------------------------------
    let f = fixture_with(Some(EDITED), VOUCHED);
    let refused = Run::default().pass(&f);
    match refused {
        Err(RenderError::VouchedMismatch {
            kind,
            path,
            vouched,
            observed,
        }) => {
            assert_eq!(kind, KIND);
            assert_eq!(path, ATTACHED);
            assert_eq!(vouched, sha256(VOUCHED).as_str());
            assert_eq!(observed, sha256(EDITED).as_str());
        }
        other => panic!("a mismatching root must be refused by name, and was: {other:?}"),
    }
    assert_eq!(root_bytes(&f), None, "nothing is stored");
    assert_eq!(
        of_kind(&f.rows, EventKind::ArtifactWrite).len(),
        1,
        "no `artifact.write` beyond the attach row"
    );

    // ---- the missing-file arm -------------------------------------------------------------------
    let f = fixture_with(None, VOUCHED);
    assert!(
        matches!(
            Run::default().pass(&f),
            Err(RenderError::VouchedUnreadable { ref kind, ref path, .. })
                if kind == KIND && path == ATTACHED
        ),
        "a missing attached file is refused by name"
    );
    assert_eq!(root_bytes(&f), None, "nothing is stored");
}

/// **A changed root is never restored, rewritten or gated** (ADR-0025's forbids: *"Blocking or
/// re-running work on a human-supplied artifact"*). Over three runs of the pass with the root
/// edited after its store: the file keeps the edited bytes every time, no `artifact.write` follows
/// the store, no gate row and no `structured_drift` is appended, and the pass classes nothing
/// blocking — while each run still records its advisory (never silent). Red under M38 (the root
/// restored to its vouched bytes).
#[test]
fn a_changed_root_is_never_restored_rewritten_or_gated() {
    let f = fixture();
    stored(&f);
    edit_root(&f, EDITED);
    let writes = of_kind(&f.rows, EventKind::ArtifactWrite).len();
    for run in 1..=3 {
        let pass = Run::default().pass(&f).expect("the pass runs");
        assert_eq!(
            root_bytes(&f).as_deref(),
            Some(EDITED),
            "run {run}: the changed root was restored or rewritten"
        );
        assert!(pass.blocking().is_empty(), "run {run}: classed blocking");
        assert_eq!(pass.advised(), 1, "run {run}: the advisory is never silent");
    }
    assert_eq!(
        of_kind(&f.rows, EventKind::ArtifactWrite).len(),
        writes,
        "no write of any kind followed the store"
    );
    for kind in [
        EventKind::GateOpened,
        EventKind::StructuredDrift,
        EventKind::ActivityCommitted,
    ] {
        assert_eq!(count(&f.rows, kind), 0, "a `{kind:?}` row was appended");
    }
    assert_eq!(count(&f.rows, EventKind::DriftAdvisory), 3, "one per run");
}

/// **One advisory per run for one observed digest** (AC-525's amendment (3), run-scoped): two
/// entry passes inside ONE run append one `drift_advisory` and count one suppression; a second
/// run over the same change appends one more (the stated cost of run scope); and inside that run a
/// further edit to a NEW digest is a new triple and appends again — the control that the memory is
/// on the triple, not a refuse-after-one rule. Red under M39 (the run's admission ignored).
#[test]
fn one_advisory_per_run_for_one_observed_digest() {
    let f = fixture();
    stored(&f);
    edit_root(&f, EDITED);

    let mut run = Run::default();
    let first = run.pass(&f).expect("the first entry of the run");
    let second = run.pass(&f).expect("a second entry of the SAME run");
    assert_eq!((first.advised(), first.suppressed()), (1, 0));
    assert_eq!(
        (second.advised(), second.suppressed()),
        (0, 1),
        "the second entry of the run is SUPPRESSED, and counted"
    );
    assert_eq!(count(&f.rows, EventKind::DriftAdvisory), 1, "one per run");

    let mut next = Run::default();
    next.pass(&f).expect("a new run");
    assert_eq!(
        count(&f.rows, EventKind::DriftAdvisory),
        2,
        "a new run over the same change appends one more (AC-525's amendment (3))"
    );
    edit_root(&f, EDITED_AGAIN);
    let moved = next.pass(&f).expect("the same run, a new digest");
    assert_eq!(
        moved.advised(),
        1,
        "CONTROL: a NEW observed digest is a new triple within the run"
    );
    let rows = of_kind(&f.rows, EventKind::DriftAdvisory);
    let digests: Vec<&str> = rows
        .iter()
        .filter_map(|e| e.payload["new_digest"].as_str())
        .collect();
    assert_eq!(
        digests,
        [
            sha256(EDITED).as_str(),
            sha256(EDITED).as_str(),
            sha256(EDITED_AGAIN).as_str()
        ]
    );
}

/// **The vouched pass over a REPLAYING writer** (ADR-0189 decisions 7 and 10; ADR-0105 clause 1).
/// *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 tests-mutants major: the
/// `!writer.is_replaying()` guard and `write_root`'s replay arm were reached by no test.)*
///
/// (a) A root the journal records as stored, then edited: the replayed pass reports the change,
/// calls no advise function, appends no advisory and restores nothing. (b) An attached root the
/// journal does NOT record as stored: the replayed store is refused by name (`ReplayUnrecorded`,
/// the kind), writes no file and appends nothing.
#[test]
fn a_replayed_pass_stores_nothing_and_appends_no_advisory() {
    let replaying = |f: &Fixture| {
        ArtifactWriter::replaying(
            f.tree.path(),
            StageRun {
                item: ITEM.to_owned(),
                stage: STAGE.to_owned(),
                run: RUN.to_owned(),
            },
            [(KIND, KIND), ("Spec", "Spec")]
                .into_iter()
                .map(|(k, v)| (k.to_owned(), v.to_owned()))
                .collect::<BTreeMap<_, _>>(),
            journal_over(&f.rows),
        )
    };

    // (a) recorded as stored, then edited.
    let f = fixture();
    stored(&f);
    edit_root(&f, EDITED);
    let before = f.rows.lock().expect("not poisoned").len();
    let mut called = 0_usize;
    let pass = vouched::vouched_pass(&replaying(&f), f.checkout.path(), &mut |d: &Drift| {
        called += 1;
        Run::default().advise(d)
    })
    .expect("the replayed pass runs");
    assert_eq!(
        pass.changed().len(),
        1,
        "the replayed pass reports the change"
    );
    assert_eq!(
        called, 0,
        "a replayed pass called the run's advise function"
    );
    assert_eq!(pass.advised(), 0, "a replayed pass advised");
    assert_eq!(
        f.rows.lock().expect("not poisoned").len(),
        before,
        "a replayed pass appended a row"
    );
    assert_eq!(
        root_bytes(&f).as_deref(),
        Some(EDITED),
        "a replayed pass rewrote the root"
    );

    // (b) attached, never recorded as stored.
    let g = fixture();
    let before = g.rows.lock().expect("not poisoned").len();
    let refused = vouched::vouched_pass(&replaying(&g), g.checkout.path(), &mut |d: &Drift| {
        Run::default().advise(d)
    });
    assert!(
        matches!(&refused, Err(RenderError::ReplayUnrecorded { kind, .. }) if kind == KIND),
        "a store replay has no record of was not refused by name: {refused:?}"
    );
    assert_eq!(
        g.rows.lock().expect("not poisoned").len(),
        before,
        "a refused replayed store appended a row"
    );
    assert_eq!(
        root_bytes(&g),
        None,
        "a refused replayed store wrote the root"
    );
}
