//! **`s8f` §8f.5 — `rk-store`'s render, write and read path** (the render record ADR-0189,
//! proposed; D21-42 5a; D21-43 N11, N12; D21-47). *(Added 2026-09-28, s8f build, package
//! S8F-P4-RENDER-WRITE-READ.)*
//!
//! Every test here runs over a temporary run-owned tree and an in-memory journal (a `Vec<Event>`
//! behind the two functions `rk_store::render::Journal` is built from). The run ref is the
//! composition root's to read; here it is a function over the tree's own files, with a fixed
//! tree hash, so a test can assert what the reader appends without a repository.
//!
//! What is read: the rows the journal holds, the files in the managed directory, and the byte
//! ranges `rk_store::locator` returns. No assertion reads a value the product computed for the
//! test, except where the assertion IS that value (a digest recomputed here from the bytes).

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use rk_store::locator::{self, Anchor};
use rk_store::render::{
    self, ArtifactReader, ArtifactWriter, Journal, ManagedDir, Reconciled, RenderError, RunRefBlob,
    StageRun, Template,
};
use rk_types::digest::sha256;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use serde_json::{Value as Json, json};

const ITEM: &str = "item-1";
const STAGE: &str = "implement";
const RUN: &str = "implement-0123456789abcdef";
const TREE: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
const COMMIT: &str = "c0ffee0000000000000000000000000000000001";

// ------------------------------------------------------------------------------- fixtures

type Rows = Arc<Mutex<Vec<Event>>>;

fn journal() -> (Journal, Rows) {
    let rows: Rows = Arc::new(Mutex::new(Vec::new()));
    let (a, r) = (Arc::clone(&rows), Arc::clone(&rows));
    (
        Journal::new(
            move |e| {
                a.lock().expect("not poisoned").push(e);
                Ok(())
            },
            move || Ok(r.lock().expect("not poisoned").clone()),
        ),
        rows,
    )
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

fn at() -> StageRun {
    StageRun {
        item: ITEM.to_owned(),
        stage: STAGE.to_owned(),
        run: RUN.to_owned(),
    }
}

fn schemas() -> BTreeMap<String, String> {
    [("Spec", "Spec"), ("ImplReport", "ImplReport")]
        .into_iter()
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .collect()
}

fn spec() -> &'static Template {
    render::template_of("Spec").expect("PREMISE: `Spec` is a human-facing kind")
}

fn spec_value() -> Json {
    json!({
        "id": "SPEC-1",
        "status": "draft",
        "title": "The spec",
        "acceptance": [{ "criterion": "the store renders", "id": "AC-1" }],
        "boundary": { "crates": ["rk-store"] },
        "context": "Why this exists.\n",
        "notes": "",
    })
}

fn report_value() -> Json {
    json!({ "summary": "implemented", "tests": 3 })
}

/// A `reconcile_started` row naming `rk-run/<ITEM>`: an entry's reconcile record.
fn reconcile_started() -> Event {
    Event::new(
        EventKind::ReconcileStarted,
        json!({
            "adapter_set": ["claude", "sqlite"],
            "branch": format!("rk-run/{ITEM}"),
            "config_digest": "0".repeat(64),
            "worktree": "/run/tree",
        }),
    )
}

fn writer(tree: &Path, rows: &Rows) -> ArtifactWriter {
    ArtifactWriter::new(tree, at(), schemas(), journal_over(rows))
}

/// The run ref, as a function over the tree's own files at a fixed tree hash.
fn reader(tree: &Path, rows: &Rows, reconciled: &Reconciled) -> ArtifactReader {
    let root = tree.to_path_buf();
    ArtifactReader::new(
        reconciled,
        at(),
        schemas(),
        journal_over(rows),
        move |path| {
            Ok(std::fs::read(root.join(path)).ok().map(|bytes| RunRefBlob {
                bytes,
                tree: TREE.to_owned(),
            }))
        },
    )
}

fn no_commit(_: &str) -> Result<Option<String>, String> {
    Ok(None)
}

fn the_commit(_: &str) -> Result<Option<String>, String> {
    Ok(Some(COMMIT.to_owned()))
}

fn rows_of(rows: &Rows, kind: EventKind) -> Vec<Event> {
    rows.lock()
        .expect("not poisoned")
        .iter()
        .filter(|e| e.kind == kind)
        .cloned()
        .collect()
}

fn keys(e: &Event) -> Vec<String> {
    e.payload
        .as_object()
        .map(|m| m.keys().cloned().collect::<std::collections::BTreeSet<_>>())
        .unwrap_or_default()
        .into_iter()
        .collect()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).expect("the rendering is UTF-8")
}

fn md_of(tree: &Path) -> String {
    text(
        &ManagedDir::of(tree, ITEM)
            .read("Spec.md")
            .expect("the rendering is in the managed directory"),
    )
}

fn put_md(tree: &Path, md: &str) {
    std::fs::write(ManagedDir::of(tree, ITEM).abs("Spec.md"), md).expect("the edit is written");
}

fn slice_of<'d>(doc: &'d str, key: &str) -> &'d str {
    let r = locator::resolve(
        doc,
        &Anchor {
            text: key.to_owned(),
            level: 2,
        },
    )
    .unwrap_or_else(|e| panic!("`{key}` does not locate: {e}"));
    &doc[r]
}

/// The first line of `s` and the rest.
fn split_first_line(s: &str) -> (&str, &str) {
    let at = s.find('\n').map_or(s.len(), |i| i + 1);
    (&s[..at], &s[at..])
}

// ------------------------------------------------------------------------------- the tests

/// **Layout C** (ADR-0006; ADR-0189 decisions 1 and 2): typed frontmatter whose last line is the
/// YAML stamp over its other bytes, one `## <field>` section per structured field whose first line
/// is the HTML-comment stamp over the rest of the slice, and prose sections carrying their text
/// with no stamp. Neither stamp is a line the locator reads as a heading.
#[test]
fn layout_c_renders_frontmatter_stamped_sections_and_prose() {
    let md = text(&render::render(spec(), &spec_value()).expect("the value renders"));
    assert!(md.starts_with("---\n"), "no frontmatter opener: {md}");

    // The frontmatter, found by the locator, stamped over its other bytes.
    let fm = locator::frontmatter(&md).expect("the rendering opens with a frontmatter block");
    let inner = &md[fm];
    let (body, stamp) = inner
        .rsplit_once("# rk-stamp ")
        .unwrap_or_else(|| panic!("the frontmatter carries no `# rk-stamp` line:\n{inner}"));
    assert_eq!(
        stamp.trim_end(),
        format!(
            "template=Spec@1 sha256={}",
            sha256(body.as_bytes()).as_str()
        ),
        "the frontmatter stamp is not over the frontmatter's other bytes"
    );
    for line in [
        "kind: \"Spec\"\n",
        "id: \"SPEC-1\"\n",
        "status: \"draft\"\n",
    ] {
        assert!(
            body.contains(line),
            "the frontmatter lacks {line:?}:\n{body}"
        );
    }

    // Each structured section: its first line is the stamp over the rest of its slice.
    for key in ["acceptance", "boundary"] {
        let slice = slice_of(&md, key);
        let (first, rest) = split_first_line(slice);
        assert_eq!(
            first,
            format!(
                "<!-- rk-stamp template=Spec@1 sha256={} -->\n",
                sha256(rest.as_bytes()).as_str()
            ),
            "`{key}`'s first line is not the stamp over the rest of its slice"
        );
        assert!(rest.contains("```json\n"), "`{key}` carries no typed block");
    }
    assert!(
        slice_of(&md, "acceptance").contains("\"criterion\":\"the store renders\""),
        "the structured section does not render the typed value"
    );

    // Prose: the text, and no stamp.
    assert_eq!(slice_of(&md, "context"), "Why this exists.\n\n");
    assert!(!slice_of(&md, "context").contains("rk-stamp"));

    // Neither stamp is a heading: the locator finds no anchor at either stamp's text.
    let yaml_heading = format!("rk-stamp {}", stamp.trim_end());
    assert_eq!(
        locator::resolve(
            &md,
            &Anchor {
                text: yaml_heading,
                level: 1
            }
        ),
        Err(locator::LocatorError::Occurrences {
            anchor: format!("rk-stamp {}", stamp.trim_end()),
            level: 1,
            count: 0
        }),
        "the frontmatter's `# rk-stamp` line is read as a heading"
    );
}

/// **AC-123**: a render over a prior section holding a sentinel absent from the value removes it,
/// and one value written over two different prior contents is byte-identical. Red under M9 (the
/// renderer reads the prior section text).
#[test]
fn render_is_a_function_of_value_and_template_only() {
    const SENTINEL: &str = "SENTINEL-7f3a-not-in-the-value";
    let v1 = spec_value();
    let mut v2 = spec_value();
    v2["status"] = json!("in-review");
    v2["acceptance"] = json!([{ "criterion": "the store re-renders", "id": "AC-1" }]);

    let outcomes: Vec<Vec<u8>> = [0, 1]
        .into_iter()
        .map(|prior| {
            let dir = tempfile::tempdir().expect("a temp dir");
            let (_, rows) = journal();
            let w = writer(dir.path(), &rows);
            w.write("Spec", &v1).expect("the first write");
            let planted = if prior == 0 {
                // Prior A: the first rendering, with the sentinel planted INSIDE a structured
                // section.
                md_of(dir.path()).replacen(
                    "\"criterion\":\"the store renders\"",
                    &format!("\"criterion\":\"{SENTINEL}\""),
                    1,
                )
            } else {
                // Prior B: unrelated bytes altogether.
                format!("# not a rendering\n\n{SENTINEL}\n")
            };
            put_md(dir.path(), &planted);
            w.write("Spec", &v2).expect("the second write");
            let md = md_of(dir.path());
            assert!(
                !md.contains(SENTINEL),
                "prior {prior}: the sentinel survived the re-render — a byte of the prior section \
                 text was read:\n{md}"
            );
            md.into_bytes()
        })
        .collect();
    assert_eq!(
        outcomes[0], outcomes[1],
        "one value over two prior contents rendered two different byte strings"
    );
    assert_eq!(
        outcomes[0],
        render::render(spec(), &v2).expect("renders"),
        "the re-render is not the pure rendering of the value"
    );
}

/// **AC-121's same-OS half**: rendering one value twice is byte-identical, LF only, even when a prose
/// value arrives with CRLF line endings.
#[test]
fn render_is_byte_identical_twice_and_lf_only() {
    let mut v = spec_value();
    v["context"] = json!("line one\r\nline two\r\n");
    let a = render::render(spec(), &v).expect("renders");
    let b = render::render(spec(), &v).expect("renders");
    assert_eq!(a, b, "two renderings of one value differ");
    assert!(!a.contains(&b'\r'), "the rendering carries a CR");
    let md = text(&a);
    assert!(
        md.contains("line one\nline two\n"),
        "the prose is not rendered, LF-normalised:\n{md}"
    );
    assert!(
        md.contains("kind: \"Spec\"\n"),
        "no typed frontmatter:\n{md}"
    );
    let on_demand = render::render_on_demand("ImplReport", &report_value()).expect("renders");
    assert_eq!(
        on_demand,
        render::render_on_demand("ImplReport", &report_value()).expect("renders")
    );
    assert!(!on_demand.contains(&b'\r'));
    assert!(
        text(&on_demand).contains("\"summary\":\"implemented\""),
        "the on-demand rendering does not carry the value"
    );
}

/// **C2's surface-and-restore** (AC-124, AC-411; ADR-0189 decision 4), one arm per location: an
/// edit inside a structured section, and one inside the frontmatter. Each appends ONE
/// `structured_drift` naming the kind, the item, the anchor, both digests and the causing commit;
/// THEN the file is restored from the type by a write whose `artifact.write` follows the drift
/// row; the typed value is unchanged. Red under M10 (restore without the event) and M11 (the event
/// without the restore).
#[test]
fn structured_edit_emits_structured_drift_then_restores() {
    for (anchor, from, to) in [
        (
            "acceptance",
            "\"criterion\":\"the store renders\"",
            "\"criterion\":\"EDITED BY HAND\"",
        ),
        ("frontmatter", "status: \"draft\"", "status: \"shipped\""),
    ] {
        let dir = tempfile::tempdir().expect("a temp dir");
        let (_, rows) = journal();
        rows.lock().expect("ok").push(reconcile_started());
        let w = writer(dir.path(), &rows);
        let v = spec_value();
        w.write("Spec", &v).expect("the write");
        let rendered = md_of(dir.path());
        let edited = rendered.replacen(from, to, 1);
        assert_ne!(
            edited, rendered,
            "PREMISE ({anchor}): the edit changed the file"
        );
        put_md(dir.path(), &edited);
        let (expected, observed) = if anchor == "frontmatter" {
            let fm = |d: &str| d[locator::frontmatter(d).expect("frontmatter")].to_owned();
            (fm(&rendered), fm(&edited))
        } else {
            (
                slice_of(&rendered, anchor).to_owned(),
                slice_of(&edited, anchor).to_owned(),
            )
        };

        let done = render::reconcile_entry(&w, &the_commit).expect("the pass runs");

        let drifts = rows_of(&rows, EventKind::StructuredDrift);
        assert_eq!(
            drifts.len(),
            1,
            "({anchor}) exactly one `structured_drift`: {drifts:?}"
        );
        assert_eq!(
            drifts[0].payload,
            json!({
                "anchor": anchor,
                "causing_commit": COMMIT,
                "item": ITEM,
                "kind": "Spec",
                "new_digest": sha256(observed.as_bytes()).as_str(),
                "old_digest": sha256(expected.as_bytes()).as_str(),
            }),
            "({anchor}) the drift row's members"
        );
        assert_eq!(done.drifts().len(), 1, "({anchor}) the pass reports it");
        assert_eq!(
            md_of(dir.path()),
            rendered,
            "({anchor}) the file is not restored from the type"
        );
        assert_eq!(
            w.value("Spec").expect("the sidecar reads"),
            v,
            "({anchor}) the edit reached the typed value"
        );
        let all = rows.lock().expect("ok").clone();
        let drift_at = all
            .iter()
            .position(|e| e.kind == EventKind::StructuredDrift)
            .expect("the drift row");
        assert!(
            all[drift_at..]
                .iter()
                .any(|e| e.kind == EventKind::ArtifactWrite
                    && e.payload["path"] == json!(format!(".redkiln/artifacts/{ITEM}/Spec.md"))),
            "({anchor}) no restore `artifact.write` after the drift row: {all:?}"
        );
    }
}

/// **AC-152**: a prose section rewritten entirely — markdown headings (deeper than the section's),
/// an HTML comment, a list — is accepted whole as the field's new value, with no drift record.
/// Red under M12 (prose treated as structured).
#[test]
fn prose_edit_is_accepted_with_no_drift_record() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    w.write("Spec", &spec_value()).expect("the write");
    let rewritten =
        "Rewritten entirely.\n### A deeper heading\n* with *markdown*\n<!-- and a comment -->\n\n";
    let md = md_of(dir.path()).replacen("Why this exists.\n\n", rewritten, 1);
    put_md(dir.path(), &md);

    let done = render::reconcile_entry(&w, &the_commit).expect("the pass runs");

    assert_eq!(
        rows_of(&rows, EventKind::StructuredDrift).len(),
        0,
        "a prose edit appended a drift record"
    );
    assert!(
        done.drifts().is_empty(),
        "a prose edit was surfaced as drift"
    );
    assert_eq!(
        done.accepted(),
        &[("Spec".to_owned(), "context".to_owned())],
        "the prose edit was not accepted"
    );
    let value = w.value("Spec").expect("the sidecar reads");
    assert_eq!(
        value["context"].as_str().map(|s| s.trim_end_matches('\n')),
        Some(rewritten.trim_end_matches('\n')),
        "the field's new value is not the section's bytes"
    );
    assert_eq!(
        value["acceptance"],
        spec_value()["acceptance"],
        "a structured field moved"
    );
    assert!(
        md_of(dir.path()).contains("### A deeper heading"),
        "the accepted prose is not in the file"
    );
}

/// **One `artifact.write` per write** (AC-122's journalled path; ADR-0189 decision 8): each carries
/// exactly `kind`, `item`, `stage`, `run`, `schema_version`, `digest` and `path`, and the digest is
/// the `rk-types` digest of the bytes at `path` — for the value's write, for the restore's write,
/// and for a machine-only kind's. Red under M13 (no `artifact.write` on the restore write).
#[test]
fn one_artifact_write_per_write_carrying_the_rk_types_digest() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let check = |row: &Event, kind: &str, schema: &Json| {
        assert_eq!(
            keys(row),
            [
                "digest",
                "item",
                "kind",
                "path",
                "run",
                "schema_version",
                "stage"
            ],
            "the row's members"
        );
        let path = row.payload["path"].as_str().expect("a path");
        let bytes = std::fs::read(dir.path().join(path)).expect("the path is written");
        assert_eq!(
            row.payload["digest"],
            json!(sha256(&bytes).as_str()),
            "the digest is not the rk-types digest of the bytes at {path}"
        );
        assert_eq!(
            (
                &row.payload["kind"],
                &row.payload["item"],
                &row.payload["stage"],
                &row.payload["run"],
                &row.payload["schema_version"]
            ),
            (
                &json!(kind),
                &json!(ITEM),
                &json!(STAGE),
                &json!(RUN),
                schema
            )
        );
        assert_eq!(
            row.item(),
            Some(ITEM),
            "the row carries the interim item key"
        );
    };

    w.write("Spec", &spec_value()).expect("the write");
    let writes = rows_of(&rows, EventKind::ArtifactWrite);
    assert_eq!(writes.len(), 1, "one write, one row: {writes:?}");
    check(&writes[0], "Spec", &json!("Spec"));
    assert_eq!(
        writes[0].payload["path"],
        json!(format!(".redkiln/artifacts/{ITEM}/Spec.json"))
    );

    // The restore is a write, so it appends one row naming the rendering.
    let md = md_of(dir.path()).replacen("\"crates\":[\"rk-store\"]", "\"crates\":[]", 1);
    put_md(dir.path(), &md);
    render::reconcile_entry(&w, &the_commit).expect("the pass runs");
    let writes = rows_of(&rows, EventKind::ArtifactWrite);
    assert_eq!(
        writes.len(),
        2,
        "the restore appended no row, or more than one: {writes:?}"
    );
    check(&writes[1], "Spec", &json!("Spec"));
    assert_eq!(
        writes[1].payload["path"],
        json!(format!(".redkiln/artifacts/{ITEM}/Spec.md"))
    );

    // A machine-only kind, and one no schema names.
    w.write("ImplReport", &report_value()).expect("the write");
    w.write("Unregistered", &json!({"a": 1}))
        .expect("the write");
    let writes = rows_of(&rows, EventKind::ArtifactWrite);
    assert_eq!(writes.len(), 4);
    check(&writes[2], "ImplReport", &json!("ImplReport"));
    check(&writes[3], "Unregistered", &Json::Null);
}

/// **AC-478's *Verified by***: no artifact type carries a drift field or a signature-comparison
/// field. A code-lines-only scan of `rk-types`' artifact types (with a planted control it must
/// catch), and the behavioural half: the sidecar the writer stores is the value's canonical bytes
/// exactly — the store adds no drift, stamp or signature member to a value.
#[test]
fn no_artifact_type_carries_a_drift_or_signature_field() {
    let scan = |src: &str| -> Vec<String> {
        src.lines()
            .map(str::trim_start)
            .filter(|l| !l.starts_with("//"))
            .filter(|l| {
                let Some(field) = l.strip_prefix("pub ").and_then(|r| r.split_once(':')) else {
                    return false;
                };
                let name = field.0.trim().to_ascii_lowercase();
                !name.contains(' ')
                    && ["drift", "signature", "_sig", "stamp"]
                        .iter()
                        .any(|n| name.contains(n))
            })
            .map(str::to_owned)
            .collect()
    };
    // The control: a planted field is caught, and the same words in a comment are not.
    assert_eq!(
        scan("/// a drift field\n    pub rendered_sig: String,\n    // pub drift: u8,\n"),
        vec!["pub rendered_sig: String,".to_owned()],
        "CONTROL: the scan cannot see a planted signature field"
    );
    let types = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("rk-types")
        .join("src");
    let mut read = 0;
    for rel in [
        "artifact/mod.rs",
        "artifact/findings.rs",
        "artifact/ledger.rs",
        "typed.rs",
    ] {
        let src = std::fs::read_to_string(types.join(rel)).expect("an artifact type source reads");
        read += 1;
        assert!(
            scan(&src).is_empty(),
            "{rel} declares a drift or signature field: {:?}",
            scan(&src)
        );
    }
    assert_eq!(read, 4, "the scan read {read} files");

    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    let w = writer(dir.path(), &rows);
    w.write("Spec", &spec_value()).expect("the write");
    assert_eq!(
        ManagedDir::of(dir.path(), ITEM).read("Spec.json"),
        Some(rk_store::canon::canonical(&spec_value()).expect("canonical")),
        "the stored sidecar is not the value's canonical bytes exactly"
    );
}

/// **AC-477**: the typed value is read back from the store's sidecar, never from the markdown — a
/// rendering overwritten with a DIFFERENT value's rendering, or deleted, changes nothing the store
/// or the reader serves.
#[test]
fn typed_value_reads_back_from_the_store_never_from_the_markdown() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = spec_value();
    w.write("Spec", &v).expect("the write");
    let mut other = spec_value();
    other["title"] = json!("A different spec");
    other["acceptance"] = json!([]);
    put_md(
        dir.path(),
        &text(&render::render(spec(), &other).expect("renders")),
    );
    assert_eq!(
        w.value("Spec").expect("reads"),
        v,
        "the value followed the markdown"
    );

    // The reader serves the sidecar too — after the entry's pass restored the rendering.
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let served = reader(dir.path(), &rows, &done)
        .serve("Spec", Some(&v))
        .expect("the read is served");
    assert_eq!(served.value, v, "the reader served the markdown's value");

    std::fs::remove_file(ManagedDir::of(dir.path(), ITEM).abs("Spec.md")).expect("deleted");
    assert_eq!(
        w.value("Spec").expect("reads"),
        v,
        "deleting the rendering moved the value"
    );
}

/// **AC-144**: a machine-only kind is its sidecar. The store stores no `.md` for it; a `.md`
/// rendered on demand and then deleted — or replaced with junk — changes nothing a read serves.
#[test]
fn machine_only_kind_reads_its_sidecar_and_survives_md_deletion() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = report_value();
    w.write("ImplReport", &v).expect("the write");
    let managed = ManagedDir::of(dir.path(), ITEM);
    assert!(
        managed.read("ImplReport.md").is_none(),
        "the store stored a machine-only kind's rendering"
    );
    let sidecar = managed
        .read("ImplReport.json")
        .expect("the sidecar is stored");
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let r = reader(dir.path(), &rows, &done);

    let first = r.serve("ImplReport", Some(&v)).expect("served");
    assert_eq!((&first.value, &first.digest), (&v, &sha256(&sidecar)));

    let md = managed.abs("ImplReport.md");
    std::fs::write(
        &md,
        render::render_on_demand("ImplReport", &v).expect("renders"),
    )
    .expect("rendered on demand");
    std::fs::remove_file(&md).expect("deleted");
    let after_delete = r.serve("ImplReport", Some(&v)).expect("served");
    std::fs::write(&md, "# junk\n\nnot the value\n").expect("junk planted");
    let after_junk = r.serve("ImplReport", Some(&v)).expect("served");
    assert_eq!(
        (after_delete.value, after_delete.digest),
        (v.clone(), sha256(&sidecar))
    );
    assert_eq!((after_junk.value, after_junk.digest), (v, sha256(&sidecar)));
}

/// **AC-122**: no filesystem-write handle into a managed directory is exported from `rk-store`: a
/// sibling crate's write through the managed directory does not compile.
#[test]
fn ac122_a_sibling_crate_cannot_write_a_managed_path() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/ac122_sibling_write.rs");
}

/// **ADR-0189 decision 7**: on replay the writer performs ZERO writes and serves the recorded
/// digest; a write replay has no record of is refused by name, and still writes nothing.
#[test]
fn replay_performs_zero_writes() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    let v = spec_value();
    let digest = sha256(&rk_store::canon::canonical(&v).expect("canonical"));
    rows.lock().expect("ok").push(
        Event::new(
            EventKind::ArtifactWrite,
            json!({
                "digest": digest.as_str(),
                "item": ITEM,
                "kind": "Spec",
                "path": format!(".redkiln/artifacts/{ITEM}/Spec.json"),
                "run": RUN,
                "schema_version": "Spec",
                "stage": STAGE,
            }),
        )
        .with_item(ITEM),
    );
    let before = rows.lock().expect("ok").len();
    let w = ArtifactWriter::replaying(dir.path(), at(), schemas(), journal_over(&rows));

    let served = w
        .write("Spec", &v)
        .expect("replay serves the recorded write");
    assert!(served.replayed, "the write was not served from the journal");
    assert_eq!(served.digest, digest, "not the recorded digest");
    assert_eq!(
        rows.lock().expect("ok").len(),
        before,
        "replay appended a row"
    );
    assert!(
        !dir.path().join(".redkiln").exists(),
        "replay wrote into the tree"
    );

    let refused = w.write("ImplReport", &report_value());
    assert!(
        matches!(refused, Err(RenderError::ReplayUnrecorded { ref kind, .. }) if kind == "ImplReport"),
        "an unrecorded write on replay was not refused by name: {refused:?}"
    );
    assert!(!dir.path().join(".redkiln").exists());
}

/// **`move-to-prose`** (ADR-0189 decision 5): the drifted bytes are read from the causing commit the
/// drift row names — through the reader of git the composition root hands in — and appended to
/// the named prose section through the writer, as one `artifact.write`. A structured section is not
/// a destination, and a drift that names no commit has nothing to move. Red under M31 (the bytes
/// read from anywhere but the causing commit).
#[test]
fn move_to_prose_appends_the_drifted_bytes_from_the_causing_commit() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = spec_value();
    w.write("Spec", &v).expect("the write");
    let at_commit = md_of(dir.path()).replacen(
        "\"criterion\":\"the store renders\"",
        "\"criterion\":\"DRIFTED AT THE COMMIT\"",
        1,
    );
    put_md(dir.path(), &at_commit);
    render::reconcile_entry(&w, &the_commit).expect("the pass surfaces and restores");
    let drift = rows_of(&rows, EventKind::StructuredDrift)
        .pop()
        .expect("PREMISE: a drift row");
    assert!(
        !md_of(dir.path()).contains("DRIFTED AT THE COMMIT"),
        "PREMISE: the working file was restored, so only the commit holds the drifted bytes"
    );

    let asked: Arc<Mutex<Vec<(String, String)>>> = Arc::default();
    let log = Arc::clone(&asked);
    let bytes = at_commit.clone().into_bytes();
    let read_at = move |commit: &str, path: &str| {
        log.lock()
            .expect("ok")
            .push((commit.to_owned(), path.to_owned()));
        if commit == COMMIT {
            Ok(bytes.clone())
        } else {
            Err(format!("no such commit {commit}"))
        }
    };
    let writes_before = rows_of(&rows, EventKind::ArtifactWrite).len();

    render::move_to_prose(&w, &drift.payload, "notes", &read_at).expect("the move applies");

    assert_eq!(
        asked.lock().expect("ok").clone(),
        vec![(
            COMMIT.to_owned(),
            format!(".redkiln/artifacts/{ITEM}/Spec.md")
        )],
        "the bytes were not read from the causing commit the drift row names"
    );
    let value = w.value("Spec").expect("reads");
    assert!(
        value["notes"]
            .as_str()
            .is_some_and(|n| n.contains("DRIFTED AT THE COMMIT")),
        "the drifted bytes are not in the prose section: {value}"
    );
    assert_eq!(
        value["acceptance"], v["acceptance"],
        "a structured field moved"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactWrite).len(),
        writes_before + 1,
        "the move is one write, one row"
    );
    assert!(slice_of(&md_of(dir.path()), "notes").contains("DRIFTED AT THE COMMIT"));

    let not_prose = render::move_to_prose(&w, &drift.payload, "acceptance", &read_at);
    assert!(
        matches!(not_prose, Err(RenderError::MoveToProse { .. })),
        "a structured section accepted drift text: {not_prose:?}"
    );
    let mut no_commit_row = drift.payload.clone();
    no_commit_row["causing_commit"] = Json::Null;
    assert!(matches!(
        render::move_to_prose(&w, &no_commit_row, "notes", &read_at),
        Err(RenderError::MoveToProse { .. })
    ));
}

/// **AC-125's architecture test**: behavioural, over a drifted section whose text PARSES as the
/// field's type — the typed value is unchanged and the re-render is the unchanged value's —
/// paired with a code-lines-only scan: `render.rs` has exactly one deserializer call, and it is
/// the sidecar's (`value_of_sidecar`).
#[test]
fn drift_text_never_reaches_a_deserializer() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = spec_value();
    w.write("Spec", &v).expect("the write");
    let rendered = md_of(dir.path());
    // A hand edit that is itself a VALID value of the field's type.
    let injected = r#"[{"criterion":"injected by hand","id":"AC-9"}]"#;
    let edited = rendered.replacen(
        r#"[{"criterion":"the store renders","id":"AC-1"}]"#,
        injected,
        1,
    );
    assert_ne!(edited, rendered, "PREMISE: the edit applies");
    put_md(dir.path(), &edited);
    render::reconcile_entry(&w, &the_commit).expect("the pass runs");
    assert_eq!(
        w.value("Spec").expect("reads"),
        v,
        "drift text reached the value"
    );
    assert_eq!(
        md_of(dir.path()),
        rendered,
        "the re-render is not the unchanged value's"
    );

    // The scan, code lines only, with its control.
    let calls = |src: &str| -> Vec<(usize, String)> {
        src.lines()
            .enumerate()
            .filter(|(_, l)| !l.trim_start().starts_with("//"))
            .filter(|(_, l)| {
                // A call, not a longer identifier ending in one (`extend_from_slice(`).
                ["from_slice(", "from_str(", "from_reader(", "from_value("]
                    .iter()
                    .any(|c| {
                        l.match_indices(c).any(|(at, _)| {
                            l[..at]
                                .chars()
                                .next_back()
                                .is_none_or(|p| !(p.is_ascii_alphanumeric() || p == '_'))
                        })
                    })
            })
            .map(|(i, l)| (i, l.to_owned()))
            .collect()
    };
    assert_eq!(
        calls(
            "// serde_json::from_str(x)\nlet v = serde_json::from_str(x);\nb.extend_from_slice(x);\n"
        )
        .len(),
        1,
        "CONTROL: the scan keeps the code call and drops the comment and a longer identifier"
    );
    let src = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("render.rs"),
    )
    .expect("render.rs reads");
    let found = calls(&src);
    assert_eq!(
        found.len(),
        1,
        "render.rs must have exactly one deserializer call: {found:?}"
    );
    let owner = src
        .lines()
        .take(found[0].0)
        .filter(|l| l.trim_start().starts_with("fn ") || l.contains(" fn "))
        .last()
        .unwrap_or_default();
    assert!(
        owner.contains("fn value_of_sidecar("),
        "the one deserializer call is not the sidecar's: it is in `{owner}`"
    );
}

/// **AC-472's first clause** (N12; D21-47; ADR-0189 decision 9): one read appends ONE
/// `artifact.read` naming the kind and the item, with `stage`, `run`, `schema_version`, the tree it
/// served and the digest of the bytes served; a served digest that is not the committed binding's
/// is a halt, and appends nothing. Red under M25 (the read seam appends no `artifact.read`).
#[test]
fn read_appends_one_artifact_read_naming_kind_and_id() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = report_value();
    w.write("ImplReport", &v).expect("the write");
    let sidecar = ManagedDir::of(dir.path(), ITEM)
        .read("ImplReport.json")
        .expect("the sidecar");
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let r = reader(dir.path(), &rows, &done);

    let served = r.serve("ImplReport", Some(&v)).expect("served");
    assert_eq!(served.tree, TREE);
    let reads = rows_of(&rows, EventKind::ArtifactRead);
    assert_eq!(reads.len(), 1, "one read, one row: {reads:?}");
    assert_eq!(
        reads[0].payload,
        json!({
            "digest": sha256(&sidecar).as_str(),
            "item": ITEM,
            "kind": "ImplReport",
            "run": RUN,
            "schema_version": "ImplReport",
            "stage": STAGE,
            "tree": TREE,
        })
    );
    assert_eq!(reads[0].item(), Some(ITEM));

    let other = json!({ "summary": "not what was emitted", "tests": 3 });
    let halted = r.serve("ImplReport", Some(&other));
    assert!(
        matches!(halted, Err(RenderError::DigestMismatch { .. })),
        "a digest mismatch was served: {halted:?}"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactRead).len(),
        1,
        "a halted read appended a row"
    );
}

/// **AC-126**: the read seam refuses a read when the current entry has no reconcile record — no
/// `reconcile_started` row at all, or a reader bound to an earlier entry than the journal's
/// latest — and appends nothing; a reader bound after the current entry's pass is served. Red
/// under M24 (the read seam serves before the entry's reconcile).
#[test]
fn a_read_before_the_entrys_reconcile_is_refused() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    let w = writer(dir.path(), &rows);
    let v = report_value();
    w.write("ImplReport", &v).expect("the write");

    // (a) No reconcile record at all.
    let early = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let refused = reader(dir.path(), &rows, &early).serve("ImplReport", Some(&v));
    assert!(
        matches!(refused, Err(RenderError::NotReconciled { .. })),
        "a read with no reconcile record was served: {refused:?}"
    );

    // (b) A reader bound to entry 1 while the journal is at entry 2.
    rows.lock().expect("ok").push(reconcile_started());
    let first = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    rows.lock().expect("ok").push(reconcile_started());
    let stale = reader(dir.path(), &rows, &first).serve("ImplReport", Some(&v));
    assert!(
        matches!(
            stale,
            Err(RenderError::NotReconciled {
                bound_entry: 1,
                journal_entries: 2,
                ..
            })
        ),
        "a read bound to an earlier entry was served: {stale:?}"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactRead).len(),
        0,
        "a refused read appended a row"
    );

    // (c) The control: bound after the current entry's pass.
    let current = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    reader(dir.path(), &rows, &current)
        .serve("ImplReport", Some(&v))
        .expect("a read after the entry's reconcile is served");
    assert_eq!(rows_of(&rows, EventKind::ArtifactRead).len(), 1);
}
