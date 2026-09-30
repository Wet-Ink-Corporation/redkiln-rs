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

/// A recorded tree, as the composition root's `cat-file blob <tree>:<path>` reads it: `bytes` at
/// every path of `tree`, and no other tree. *(Added 2026-09-29, s8f build, package R5FIX-8f;
/// D21-113 step 4.)*
fn recorded_tree(
    tree: &'static str,
    bytes: Vec<u8>,
) -> impl Fn(&str, &str) -> Result<Option<Vec<u8>>, String> + Send + Sync + 'static {
    move |t, _path| Ok((t == tree).then(|| bytes.clone()))
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

    // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 tests-mutants major.)* THE
    // DIVERGENT-RECORD ARM. The journal records an `artifact.write` of `Spec` at the same path
    // under a DIFFERENT digest from the bytes this write derives: the replay is refused by name
    // (`ReplayDiverged`, naming the kind and both digests), appends nothing and writes nothing.
    // Above, the recorded digest equals the derived one, so "serves the recorded digest" could not
    // be told from "serves the digest it just derived"; here it can.
    let other = json!({ "id": "SPEC-1", "status": "shipped", "title": "Another spec",
        "acceptance": [], "boundary": { "crates": [] }, "context": "", "notes": "" });
    let derived = sha256(&rk_store::canon::canonical(&other).expect("canonical"));
    assert_ne!(derived, digest, "PREMISE: the two values' digests differ");
    let before = rows.lock().expect("ok").len();
    let diverged = w.write("Spec", &other);
    assert!(
        matches!(
            &diverged,
            Err(RenderError::ReplayDiverged { kind, recorded, derived: d })
                if kind == "Spec" && recorded == digest.as_str() && d == derived.as_str()
        ),
        "a replay whose recorded digest differs from the derived one was not refused by name: \
         {diverged:?}"
    );
    assert_eq!(
        rows.lock().expect("ok").len(),
        before,
        "a diverged replay appended a row"
    );
    assert!(
        !dir.path().join(".redkiln").exists(),
        "a diverged replay wrote into the tree"
    );
}

/// **The read seam on replay** (§8f.5: *"On replay the seam serves the recorded digest and appends
/// nothing new"*; ADR-0189 decision 9). *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE;
/// round-1 tests-mutants major: `ArtifactReader::replaying` had no caller and no test.)* The
/// journal records one `artifact.read` of `ImplReport` whose digest and tree are NOT the bytes now
/// in the tree, so the served values can only be the record's. A kind with no recorded read is
/// refused by name, and AC-126's gate still refuses a replaying reader bound to an earlier entry.
#[test]
fn a_replaying_reader_serves_the_recorded_read_and_appends_nothing() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = report_value();
    w.write("ImplReport", &v).expect("the write");
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    // *(dated 2026-09-29, s8f build, package R5FIX-8f; D21-113 step 4.)* ~~`sha256(b"the bytes
    // the recorded run read")`~~: a replayed read now serves the blob at the RECORDED tree,
    // checked against the recorded digest, so the recorded bytes are a sidecar the recorded tree
    // holds (`recorded_tree`), and they are still not the bytes now in the tree.
    let recorded_bytes = rk_store::canon::canonical(
        &json!({ "summary": "as the recorded run read it", "tests": 1 }),
    )
    .expect("canonical");
    let recorded = sha256(&recorded_bytes);
    let live = sha256(
        &ManagedDir::of(dir.path(), ITEM)
            .read("ImplReport.json")
            .expect("the sidecar"),
    );
    assert_ne!(
        recorded, live,
        "PREMISE: the record is not the tree's bytes"
    );
    rows.lock().expect("ok").push(
        Event::new(
            EventKind::ArtifactRead,
            json!({
                "digest": recorded.as_str(),
                "item": ITEM,
                "kind": "ImplReport",
                "run": RUN,
                "schema_version": "ImplReport",
                "stage": STAGE,
                "tree": "recorded-tree",
            }),
        )
        .with_item(ITEM),
    );
    let before = rows.lock().expect("ok").len();
    let r = ArtifactReader::replaying(
        &done,
        at(),
        schemas(),
        journal_over(&rows),
        recorded_tree("recorded-tree", recorded_bytes),
    );

    let served = r
        .serve("ImplReport", Some(&v))
        .expect("replay serves the recorded read");
    assert!(served.replayed, "the read was not served from the journal");
    assert_eq!(served.digest, recorded, "not the recorded digest");
    assert_eq!(served.tree, "recorded-tree", "not the recorded tree");
    assert_eq!(
        rows.lock().expect("ok").len(),
        before,
        "a replayed read appended a row"
    );

    let unrecorded = r.serve("Spec", None);
    assert!(
        matches!(&unrecorded, Err(RenderError::ReplayUnrecorded { kind, .. }) if kind == "Spec"),
        "a read replay has no record of was not refused by name: {unrecorded:?}"
    );

    rows.lock().expect("ok").push(reconcile_started());
    let stale = r.serve("ImplReport", Some(&v));
    assert!(
        matches!(
            stale,
            Err(RenderError::NotReconciled {
                bound_entry: 1,
                journal_entries: 2,
                ..
            })
        ),
        "a replaying read bound to an earlier entry was served (AC-126): {stale:?}"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactRead).len(),
        1,
        "a replaying reader appended an `artifact.read`"
    );
}

/// **T3 (D21-113): the read still FAILS CLOSED on bytes no recorded write accounts for** (ADR-0189
/// decision 9's dated note, proposed; Halt 4). *(Added 2026-09-29, s8f build, package R5FIX-8f;
/// the control the ruling names.)* `Spec` is written through the writer (the emit's own
/// `artifact.write`), the entry's pass runs, and then the sidecar on the run ref is rewritten
/// OUTSIDE the writer, with no row. The read is a `DigestMismatch` naming both digests, and appends
/// nothing. Red under M3 (the digest check dropped).
#[test]
fn an_out_of_band_sidecar_edit_with_no_writer_row_still_halts() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = spec_value();
    let written = w.write("Spec", &v).expect("the write");
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let other = rk_store::canon::canonical(&json!({ "id": "SPEC-1", "status": "shipped",
        "title": "edited outside the writer", "acceptance": [], "boundary": { "crates": [] },
        "context": "", "notes": "" }))
    .expect("canonical");
    std::fs::write(ManagedDir::of(dir.path(), ITEM).abs("Spec.json"), &other)
        .expect("the out-of-band edit");
    let writes = rows_of(&rows, EventKind::ArtifactWrite).len();
    let halted = reader(dir.path(), &rows, &done).serve("Spec", Some(&v));
    assert!(
        matches!(
            &halted,
            Err(RenderError::DigestMismatch { kind, served, bound })
                if kind == "Spec"
                    && served == sha256(&other).as_str()
                    && bound == written.digest.as_str()
        ),
        "an out-of-band sidecar edit with no writer row was served: {halted:?}"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactRead).len(),
        0,
        "a halted read appended a row"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactWrite).len(),
        writes,
        "PREMISE: the edit appended no `artifact.write`"
    );
}

/// **D21-113 step 3's fallback: with NO recorded write of the sidecar's path, the read is checked
/// against the binding's canonical digest** (*"With no such row it falls back to the binding's
/// canonical digest, which is byte-identical to today"*; `s8f` §8f.5's R5FIX2-8f note; ADR-0189
/// decision 9's dated note, proposed). *(Added 2026-09-29, s8f build, package R6FIX-8f; refuting
/// round 6's tests-mutants minor.)* The journal holds the entry's reconcile and NO `artifact.write`
/// of `Spec.json` (a pre-`s8f` journal's shape), and the sidecar's bytes are put at the path
/// outside the writer. Bytes whose digest is the binding's canonical digest are served; other bytes
/// are a `DigestMismatch` naming the binding's canonical digest, and append nothing. Red under
/// M-READ-FALLBACK (the fallback arm dropped: a read with no recorded write is never checked).
#[test]
fn a_read_with_no_recorded_write_of_its_path_is_checked_against_the_binding() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let v = spec_value();
    let bound = rk_store::canon::canonical(&v).expect("canonical");
    let bound_digest = rk_store::canon::canonical_digest(&v).expect("canonical digest");
    assert_eq!(
        sha256(&bound).as_str(),
        bound_digest.as_str(),
        "PREMISE: the canonical bytes hash to the canonical digest"
    );
    let sidecar = ManagedDir::of(dir.path(), ITEM).abs("Spec.json");
    std::fs::create_dir_all(sidecar.parent().expect("a parent")).expect("the managed dir");
    let path = render::managed_path(ITEM, "Spec.json");
    let written_of_path = || {
        rows_of(&rows, EventKind::ArtifactWrite)
            .iter()
            .filter(|e| e.payload.get("path").and_then(Json::as_str) == Some(path.as_str()))
            .count()
    };

    // Other bytes, no recorded write: a halt naming the binding's canonical digest.
    let other = rk_store::canon::canonical(&json!({ "id": "SPEC-1", "status": "shipped",
        "title": "not the binding", "acceptance": [], "boundary": { "crates": [] },
        "context": "", "notes": "" }))
    .expect("canonical");
    std::fs::write(&sidecar, &other).expect("the bytes are put at the path");
    assert_eq!(
        written_of_path(),
        0,
        "PREMISE: no `artifact.write` names the sidecar's path"
    );
    let halted = reader(dir.path(), &rows, &done).serve("Spec", Some(&v));
    assert!(
        matches!(
            &halted,
            Err(RenderError::DigestMismatch { kind, served, bound })
                if kind == "Spec"
                    && served == sha256(&other).as_str()
                    && bound == bound_digest.as_str()
        ),
        "bytes that are not the binding's, with no recorded write, were not a halt naming the \
         binding's canonical digest: {halted:?}"
    );
    assert_eq!(
        rows_of(&rows, EventKind::ArtifactRead).len(),
        0,
        "a halted read appended a row"
    );

    // The binding's own bytes, no recorded write: served.
    std::fs::write(&sidecar, &bound).expect("the bytes are put at the path");
    let served = reader(dir.path(), &rows, &done)
        .serve("Spec", Some(&v))
        .expect("the binding's bytes with no recorded write are served");
    assert_eq!(served.digest.as_str(), bound_digest.as_str());
    assert_eq!(served.value, v, "the served value is not the sidecar's");
    assert_eq!(
        written_of_path(),
        0,
        "PREMISE: still no `artifact.write` names the sidecar's path"
    );
    assert_eq!(rows_of(&rows, EventKind::ArtifactRead).len(), 1);
}

/// **T4 (D21-113 step 4): a replayed read serves the blob at the RECORDED tree, checked against the
/// recorded digest, never the journalled binding, and appends nothing** (ADR-0189 decision 9's
/// dated note, proposed). *(Added 2026-09-29, s8f build, package R5FIX-8f.)* The journal records
/// one `artifact.read` of `Spec` whose tree holds the ACCEPTED value (a prose edit accepted after
/// the emit), while the binding the resolver hands the reader is the EMITTED value. Replay serves
/// the accepted value. A recorded tree whose blob is not the recorded digest is refused by name
/// (`DigestMismatch`), and a recorded tree without the blob is `ReplayUnrecorded`. Red under M4
/// (replay serves `bound.cloned()`).
#[test]
fn a_replayed_read_serves_the_recorded_trees_blob_not_the_binding() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let emitted = spec_value();
    w.write("Spec", &emitted).expect("the write");
    let done = render::reconcile_entry(&w, &no_commit).expect("the pass runs");
    let mut accepted = emitted.clone();
    accepted["context"] = json!("A human's accepted context.\n");
    let accepted_bytes = rk_store::canon::canonical(&accepted).expect("canonical");
    let digest = sha256(&accepted_bytes);
    rows.lock().expect("ok").push(
        Event::new(
            EventKind::ArtifactRead,
            json!({
                "digest": digest.as_str(),
                "item": ITEM,
                "kind": "Spec",
                "run": RUN,
                "schema_version": "Spec",
                "stage": STAGE,
                "tree": "accepted-tree",
            }),
        )
        .with_item(ITEM),
    );
    let before = rows.lock().expect("ok").len();
    let replay = |bytes: Vec<u8>| {
        ArtifactReader::replaying(
            &done,
            at(),
            schemas(),
            journal_over(&rows),
            recorded_tree("accepted-tree", bytes),
        )
    };

    let served = replay(accepted_bytes)
        .serve("Spec", Some(&emitted))
        .expect("replay serves the recorded read");
    assert!(served.replayed, "the read was not served from the journal");
    assert_eq!(
        served.value, accepted,
        "replay did not serve the recorded tree's blob: it served {}",
        served.value
    );
    assert_eq!(served.digest, digest, "not the recorded digest");
    assert_eq!(served.tree, "accepted-tree", "not the recorded tree");

    let diverged = replay(rk_store::canon::canonical(&emitted).expect("canonical"))
        .serve("Spec", Some(&emitted));
    assert!(
        matches!(&diverged, Err(RenderError::DigestMismatch { kind, bound, .. })
            if kind == "Spec" && bound == digest.as_str()),
        "a recorded tree whose blob is not the recorded digest was served: {diverged:?}"
    );
    let absent =
        ArtifactReader::replaying(&done, at(), schemas(), journal_over(&rows), |_, _| Ok(None))
            .serve("Spec", Some(&emitted));
    assert!(
        matches!(&absent, Err(RenderError::ReplayUnrecorded { kind, .. }) if kind == "Spec"),
        "a recorded tree without the blob was served: {absent:?}"
    );
    assert_eq!(
        rows.lock().expect("ok").len(),
        before,
        "a replayed read appended a row"
    );
}

/// **The entry pass over a REPLAYING writer** (ADR-0189 decisions 4 and 7; ADR-0105 clause 1).
/// *(Added 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 tests-mutants major: the
/// restore's replay arm was reached by no test.)* A live entry pass records a structured edit's
/// `structured_drift` and its restore. Replayed over the same drifted file and that journal, the
/// pass reports the drift, appends NOTHING (the drift row and the restore's `artifact.write` are
/// both recorded) and writes nothing: the drifted file stays as it is.
#[test]
fn a_replayed_entry_pass_restores_nothing_and_appends_nothing() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    w.write("Spec", &spec_value()).expect("the write");
    let rendered = md_of(dir.path());
    let edited = rendered.replacen(
        "\"criterion\":\"the store renders\"",
        "\"criterion\":\"EDITED BY HAND\"",
        1,
    );
    assert_ne!(edited, rendered, "PREMISE: the edit changed the file");
    put_md(dir.path(), &edited);
    render::reconcile_entry(&w, &the_commit).expect("the live pass runs");
    assert_eq!(
        rows_of(&rows, EventKind::StructuredDrift).len(),
        1,
        "PREMISE: the live pass recorded one drift"
    );
    assert_eq!(
        md_of(dir.path()),
        rendered,
        "PREMISE: the live pass restored"
    );

    put_md(dir.path(), &edited);
    let before = rows.lock().expect("ok").len();
    let replaying = ArtifactWriter::replaying(dir.path(), at(), schemas(), journal_over(&rows));
    let done = render::reconcile_entry(&replaying, &the_commit).expect("the replayed pass runs");
    assert_eq!(
        done.drifts().len(),
        1,
        "the replayed pass reports the drift"
    );
    let after = rows.lock().expect("ok").clone();
    assert_eq!(
        after.len(),
        before,
        "a replayed entry pass appended a row: {:?}",
        after.get(before..)
    );
    assert_eq!(
        md_of(dir.path()),
        edited,
        "a replayed entry pass restored the file (a write on replay)"
    );
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

/// **The entry pass's replay refusal** (ADR-0189 decision 7; ADR-0105 clause 1): on replay the
/// `structured_drift` row is SERVED, so a drift the journal does not record is refused by name
/// (`ReplayUnrecorded`, naming the kind and the rendering's path), nothing is appended and the file
/// is left as it is. The journal here holds the live pass's restore row but NOT its drift row, so
/// the restore that follows would be served: only the entry pass's own refusal can fail the
/// replay. *(Added 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2's
/// tests-mutants major.)* Red under M-EPU (the refusal dropped).
#[test]
fn a_replayed_entry_pass_refuses_a_drift_the_journal_does_not_record() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    w.write("Spec", &spec_value()).expect("the write");
    let rendered = md_of(dir.path());
    let edited = rendered.replacen(
        "\"criterion\":\"the store renders\"",
        "\"criterion\":\"EDITED BY HAND\"",
        1,
    );
    assert_ne!(edited, rendered, "PREMISE: the edit changed the file");
    put_md(dir.path(), &edited);
    render::reconcile_entry(&w, &the_commit).expect("the live pass runs");
    assert_eq!(
        rows_of(&rows, EventKind::StructuredDrift).len(),
        1,
        "PREMISE: the live pass recorded one drift"
    );
    // The journal loses the drift row and keeps the restore's `artifact.write`.
    rows.lock()
        .expect("ok")
        .retain(|e| e.kind != EventKind::StructuredDrift);
    assert!(
        rows_of(&rows, EventKind::ArtifactWrite)
            .iter()
            .any(|e| e.payload["path"] == json!(format!(".redkiln/artifacts/{ITEM}/Spec.md"))),
        "PREMISE: the restore's row is recorded, so a replayed restore is served"
    );

    put_md(dir.path(), &edited);
    let before = rows.lock().expect("ok").clone();
    let replaying = ArtifactWriter::replaying(dir.path(), at(), schemas(), journal_over(&rows));
    let out = render::reconcile_entry(&replaying, &the_commit);
    assert!(
        matches!(
            &out,
            Err(RenderError::ReplayUnrecorded { kind, path })
                if kind == "Spec" && *path == format!(".redkiln/artifacts/{ITEM}/Spec.md")
        ),
        "a replayed entry pass served a drift the journal does not record: {out:?}"
    );
    assert_eq!(
        *rows.lock().expect("ok"),
        before,
        "a refused replay appended a row"
    );
    assert_eq!(md_of(dir.path()), edited, "a refused replay wrote the file");
}

/// **`move-to-prose` of a `document` drift** (ADR-0189 decision 5; `s8f` §8f.6 reading 6: *"A
/// missing rendering of a human-facing kind is one `structured_drift` at anchor `document`, then a
/// restore"*). The drifted state is the ABSENT file, whose bytes are none (the row's `new_digest` is
/// the empty digest), so the move reads no commit (the causing commit is the one that deleted the
/// file, where it cannot be read), appends nothing to the prose section, and applies as the
/// answer's one typed write: one `artifact.write` naming the sidecar. The next entry is clean. A
/// `document` row whose `new_digest` is NOT the empty digest names bytes the move cannot find and
/// is refused by name. *(Added 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2's
/// spec-conformance major.)* Red under M-DOC (the `document` arm dropped).
#[test]
fn move_to_prose_of_a_document_drift_moves_nothing_and_applies() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    let v = spec_value();
    w.write("Spec", &v).expect("the write");
    std::fs::remove_file(ManagedDir::of(dir.path(), ITEM).abs("Spec.md"))
        .expect("the rendering is deleted");
    render::reconcile_entry(&w, &the_commit).expect("the pass surfaces and restores");
    let drift = rows_of(&rows, EventKind::StructuredDrift)
        .pop()
        .expect("PREMISE: a drift row");
    assert_eq!(
        drift.payload["anchor"],
        json!(render::DOCUMENT_ANCHOR),
        "PREMISE: a missing rendering is a `document` drift"
    );
    assert_eq!(
        drift.payload["new_digest"],
        json!(sha256(b"").as_str()),
        "PREMISE: the drifted bytes are none"
    );

    let asked: Arc<Mutex<Vec<(String, String)>>> = Arc::default();
    let log = Arc::clone(&asked);
    let read_at = move |commit: &str, path: &str| -> Result<Vec<u8>, String> {
        log.lock()
            .expect("ok")
            .push((commit.to_owned(), path.to_owned()));
        Err(format!(
            "`{path}` is not in {commit}: the commit deleted it"
        ))
    };
    let writes_before = rows_of(&rows, EventKind::ArtifactWrite).len();
    let moved = render::move_to_prose(&w, &drift.payload, "notes", &read_at);
    assert!(
        moved.is_ok(),
        "a `move-to-prose` answer to a `document` drift cannot be applied: {moved:?}"
    );
    assert!(
        asked.lock().expect("ok").is_empty(),
        "a move of no bytes read a commit"
    );
    assert_eq!(w.value("Spec").expect("reads"), v, "the typed value moved");
    let writes = rows_of(&rows, EventKind::ArtifactWrite);
    assert_eq!(
        writes.len(),
        writes_before + 1,
        "the move is one write, one row"
    );
    assert_eq!(
        writes.last().map(|e| e.payload["path"].clone()),
        Some(json!(format!(".redkiln/artifacts/{ITEM}/Spec.json"))),
        "the move's row names the sidecar (the answer's applied mark)"
    );
    let again = render::reconcile_entry(&w, &the_commit).expect("the next pass runs");
    assert!(
        again.drifts().is_empty() && again.accepted().is_empty(),
        "the next entry is not clean: {again:?}"
    );

    let mut named_bytes = drift.payload;
    named_bytes["new_digest"] = json!(sha256(b"some bytes").as_str());
    assert!(
        matches!(
            render::move_to_prose(&w, &named_bytes, "notes", &read_at),
            Err(RenderError::MoveToProse { .. })
        ),
        "a `document` drift naming bytes was moved as none"
    );
}

/// **`move-to-prose` of a `frontmatter` drift is stable** (ADR-0189 decisions 2 and 5; AC-152).
/// The frontmatter's drifted bytes end with its stamp, `# rk-stamp …`, a line the locator reads as
/// a level-1 heading once it sits in a prose section — so moved bare it would end the section, and
/// the SAME entry's pass would accept the store's own rendering as a human's prose edit and drop
/// the line. The moved bytes are therefore fenced: the next entry records no drift and accepts no
/// prose, and the moved text is whole in the section. *(Added 2026-09-29, s8f build, package
/// R2FIX-8f-A-CODE; refuting round 2's spec-conformance minor.)* Red under M-FMF (the fence
/// dropped).
#[test]
fn move_to_prose_of_a_frontmatter_drift_is_stable_at_the_next_entry() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    rows.lock().expect("ok").push(reconcile_started());
    let w = writer(dir.path(), &rows);
    w.write("Spec", &spec_value()).expect("the write");
    let at_commit =
        md_of(dir.path()).replacen("title: \"The spec\"", "title: \"EDITED OUT OF BAND\"", 1);
    assert_ne!(
        at_commit,
        md_of(dir.path()),
        "PREMISE: the frontmatter is edited"
    );
    put_md(dir.path(), &at_commit);
    render::reconcile_entry(&w, &the_commit).expect("the pass surfaces and restores");
    let drift = rows_of(&rows, EventKind::StructuredDrift)
        .pop()
        .expect("PREMISE: a drift row");
    assert_eq!(
        drift.payload["anchor"],
        json!(render::FRONTMATTER_ANCHOR),
        "PREMISE: the drift is the frontmatter's"
    );
    let bytes = at_commit.into_bytes();
    let read_at = move |commit: &str, _: &str| -> Result<Vec<u8>, String> {
        if commit == COMMIT {
            Ok(bytes.clone())
        } else {
            Err(format!("no such commit {commit}"))
        }
    };
    render::move_to_prose(&w, &drift.payload, "notes", &read_at).expect("the move applies");
    let moved = w.value("Spec").expect("reads")["notes"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        moved.contains("title: \"EDITED OUT OF BAND\"") && moved.contains("# rk-stamp "),
        "the frontmatter's drifted bytes are not whole in the prose section: {moved:?}"
    );

    let before = rows.lock().expect("ok").len();
    let again = render::reconcile_entry(&w, &the_commit).expect("the next pass runs");
    assert_eq!(
        (again.drifts().len(), again.accepted().to_vec()),
        (0, Vec::new()),
        "the next entry read the moved text as a drift or a human's prose edit: {again:?}"
    );
    assert_eq!(
        rows.lock().expect("ok").len(),
        before,
        "the next entry appended a row"
    );
    assert_eq!(
        w.value("Spec").expect("reads")["notes"].as_str(),
        Some(moved.as_str()),
        "the next entry rewrote the moved text"
    );
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
///
/// *(dated 2026-09-29, s8f build, package R5FIX2-8f; D21-113 step 3 as amended by D21-114, both
/// TAKEN PROVISIONALLY under D21-105; ADR-0189 decision 9's dated note, proposed.)* ~~a served
/// digest that is not the committed binding's is a halt~~: a served digest that is not the LATEST
/// recorded write of the sidecar's path (the binding's only when there is none) is a halt. The halt
/// arm no longer hands the reader a different binding over the recorded bytes (that read is now
/// served, as D21-113 rules); it puts other bytes at the sidecar's path out of band, and the read
/// halts and appends nothing.
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
    std::fs::write(
        dir.path()
            .join(render::managed_path(ITEM, "ImplReport.json")),
        serde_json::to_vec(&other).expect("serialises"),
    )
    .expect("the out-of-band edit is written");
    let halted = r.serve("ImplReport", Some(&v));
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

// ------------------------------------------------------------------------------- D21-109

/// The code lines of `src` (1-based numbers kept): a line whose trimmed start is `//`, `/*` or `*`
/// is a comment and is dropped. It cannot see a trailing comment after code or a string literal,
/// and says so rather than claiming them away.
fn code_lines(src: &str) -> Vec<(usize, &str)> {
    src.lines()
        .enumerate()
        .filter(|(_, l)| {
            let t = l.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .map(|(i, l)| (i + 1, l))
        .collect()
}

/// The `fn` a line sits in: the last line at or before it that declares one.
fn owner_fn(lines: &[(usize, &str)], at: usize) -> String {
    lines
        .iter()
        .rev()
        .filter(|(n, _)| *n <= at)
        .find(|(_, l)| {
            let t = l.trim_start();
            t.starts_with("fn ") || t.contains(" fn ")
        })
        .map(|(_, l)| l.trim().to_owned())
        .unwrap_or_default()
}

/// Every word-boundary `Reconciled` in `line`, as the text before it and the text after it.
fn reconciled_at(line: &str) -> Vec<(&str, &str)> {
    line.match_indices("Reconciled")
        .filter(|(at, _)| {
            line[..*at]
                .chars()
                .next_back()
                .is_none_or(|p| !(p.is_ascii_alphanumeric() || p == '_'))
                && line[at + "Reconciled".len()..]
                    .chars()
                    .next()
                    .is_none_or(|n| !(n.is_ascii_alphanumeric() || n == '_'))
        })
        .map(|(at, w)| (&line[..at], &line[at + w.len()..]))
        .collect()
}

/// A scanned file set: `(repo-relative path, source)` pairs.
type Files = Vec<(String, String)>;

/// Does `text` hold `word` at word boundaries? *(Added 2026-09-29, s8f build, package
/// R1FIX-8f-A-CODE.)*
fn has_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(at, _)| {
        text[..at]
            .chars()
            .next_back()
            .is_none_or(|p| !(p.is_ascii_alphanumeric() || p == '_'))
            && text[at + word.len()..]
                .chars()
                .next()
                .is_none_or(|n| !(n.is_ascii_alphanumeric() || n == '_'))
    })
}

/// The traits a `Reconciled` may derive or implement: none of them builds one from nothing (a
/// `Clone` needs one to exist). Every other derived or implemented trait is (3). *(Added
/// 2026-09-29, s8f build, package R3FIX-8f; refuting round 3's tests-mutants major.)*
const NON_CONSTRUCTING: [&str; 8] = [
    "Debug",
    "Display",
    "Clone",
    "PartialEq",
    "Eq",
    "Hash",
    "PartialOrd",
    "Ord",
];

/// The last path segment of a name, less its generic arguments: `serde::Deserialize<'de>` is
/// `Deserialize`.
fn last_segment(path: &str) -> String {
    let path = path.split('<').next().unwrap_or_default().trim();
    path.rsplit("::")
        .next()
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// Every name in every `derive(…)` of `attrs`. *(Added 2026-09-29, s8f build, package R3FIX-8f.)*
fn derived_names(attrs: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = attrs;
    while let Some(at) = rest.find("derive(") {
        let after = &rest[at + "derive(".len()..];
        let list = after.split(')').next().unwrap_or_default();
        out.extend(list.split(',').map(last_segment).filter(|n| !n.is_empty()));
        rest = after;
    }
    out
}

/// The trait an `impl … for` head names (`head` ends at `for`): `impl<'de> serde::Deserialize<'de>
/// for` is `Deserialize`; `None` when `head` holds no `impl`. *(Added 2026-09-29, s8f build,
/// package R3FIX-8f.)*
fn impl_trait_name(head: &str) -> Option<String> {
    let at = head.rfind("impl")?;
    let mut rest = head[at + "impl".len()..].trim_start();
    if rest.starts_with('<') {
        let mut depth = 0_usize;
        let mut end = rest.len();
        for (i, c) in rest.char_indices() {
            match c {
                '<' => depth += 1,
                '>' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        rest = rest[end..].trim_start();
    }
    let name = last_segment(rest.trim_end().strip_suffix("for").unwrap_or(rest));
    (!name.is_empty()).then_some(name)
}

/// **D21-109's construction lint, as one function over `(repo-relative path, source)` pairs** so
/// each check can be driven red over a planted file first. Returns every violation, each naming
/// its check. `crates/rk-git/src/` is not scanned: it declares an unrelated generic
/// `rk_git::reconciled::Reconciled<T>` and cannot name `rk-store`'s type (no dependency either way).
fn reconciled_violations(files: &[(String, String)]) -> Vec<String> {
    let mut bad = Vec::new();
    let mut literals = Vec::new();
    let mut returns = Vec::new();
    let mut bindings = Vec::new();
    for (path, src) in files {
        if path.starts_with("crates/rk-git/src/") {
            continue;
        }
        let lines = code_lines(src);
        for (k, (n, line)) in lines.iter().enumerate() {
            // *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2.)* The
            // previous code line, for an alias rustfmt wrapped after its `=`.
            let prev = if k > 0 { lines[k - 1].1.trim() } else { "" };
            for (before, after) in reconciled_at(line) {
                // (7) an alias of it: `type X = …Reconciled…;` (on one line, or wrapped after its
                // `=`) or `use …Reconciled as X`. Every check here keys on the word, so a second
                // name would build, return or default one unseen. *(dated 2026-09-29, s8f build,
                // package R2FIX-8f-A-CODE.)*
                let head = if prev.ends_with('=') {
                    format!("{prev} {before}")
                } else {
                    before.to_owned()
                };
                if (has_word(&head, "type") && head.contains('='))
                    || after.trim_start().starts_with("as ")
                {
                    bad.push(format!(
                        "(7) {path}:{n}: an alias names a Reconciled: {}",
                        line.trim()
                    ));
                }
                let decl = before.trim_end();
                let is_decl =
                    decl.ends_with("struct") || decl.ends_with("impl") || decl.ends_with("for");
                // (1) a struct literal: `Reconciled {` that declares nothing.
                if after.trim_start().starts_with('{') && !is_decl {
                    literals.push((path.clone(), *n, owner_fn(&lines, *n)));
                }
                // (2) a return type naming it.
                if before.contains("->") {
                    returns.push((path.clone(), *n, owner_fn(&lines, *n)));
                }
                // (3) a Default or From impl for it. *(dated 2026-09-29, s8f build, package
                // R3FIX-8f; refuting round 3.)* ~~`Default` or `From` in the head~~ — ANY trait
                // impl for it whose trait is not in [`NON_CONSTRUCTING`] (a `Deserialize`, a
                // `FromStr`, a `TryFrom` each build one), read from a head rustfmt wrapped before
                // `for Reconciled` too (the previous code line joined).
                if decl.ends_with("for") {
                    let head = if before.contains("impl") {
                        before.to_owned()
                    } else {
                        format!("{prev} {before}")
                    };
                    if let Some(name) = impl_trait_name(&head)
                        && !NON_CONSTRUCTING.contains(&name.as_str())
                    {
                        bad.push(format!(
                            "(3) {path}:{n}: a `{name}` impl builds a Reconciled: {}",
                            line.trim()
                        ));
                    }
                }
            }
            // (6) no holder that outlives an invocation.
            if !reconciled_at(line).is_empty()
                && [
                    "static ",
                    "OnceLock",
                    "LazyLock",
                    "OnceCell",
                    "thread_local!",
                    "lazy_static",
                ]
                .iter()
                .any(|h| line.contains(h))
            {
                bad.push(format!(
                    "(6) {path}:{n}: a holder that outlives an invocation: {}",
                    line.trim()
                ));
            }
            // (5) rk-cli's bindings of `Option<…Reconciled>`.
            if path.starts_with("crates/rk-cli/src/")
                && line.contains("Option<")
                && reconciled_at(line).iter().any(|(_, a)| a.starts_with('>'))
                && line.trim_start().starts_with("let ")
            {
                bindings.push((
                    path.clone(),
                    *n,
                    owner_fn(&lines, *n),
                    line.trim().to_owned(),
                ));
            }
        }
        // (2b) and (1b): inside EVERY `impl … Reconciled … {` block, in any scanned file. *(dated
        // 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 tests-mutants major: the check
        // read only the FIRST `impl Reconciled {` block of render.rs and only the spelling
        // `-> Self`, so a second block, `-> Option<Self>` or `-> Result<Self, _>` built one
        // unseen.)* A return type naming `Self` or `Reconciled` in any form is (2); a `Self {` or
        // `Self(` construction is (1).
        for (k, (_, head)) in lines.iter().enumerate() {
            let t = head.trim();
            if !(t.starts_with("impl") && t.ends_with('{') && !reconciled_at(head).is_empty()) {
                continue;
            }
            for (n, l) in lines[k + 1..].iter().take_while(|(_, l)| *l != "}") {
                if let Some((_, ret)) = l.split_once("->")
                    && (has_word(ret, "Self") || !reconciled_at(ret).is_empty())
                {
                    bad.push(format!(
                        "(2) {path}:{n}: a fn in `{t}` returns a Reconciled: {}",
                        l.trim()
                    ));
                }
                if l.contains("Self {") || l.contains("Self(") {
                    bad.push(format!(
                        "(1) {path}:{n}: a `Self` construction in `{t}`: {}",
                        l.trim()
                    ));
                }
            }
        }
        // (4): inside the type's own declaration.
        if path == "crates/rk-store/src/render.rs" {
            let block = |head: &str| -> Vec<(usize, &str)> {
                let Some(i) = lines
                    .iter()
                    .position(|(_, l)| l.trim_start().starts_with(head))
                else {
                    return Vec::new();
                };
                lines[i + 1..]
                    .iter()
                    .take_while(|(_, l)| *l != "}")
                    .copied()
                    .collect()
            };
            let fields = block("pub struct Reconciled {");
            if fields.is_empty() {
                bad.push(format!("(4) {path}: `pub struct Reconciled {{` not found"));
            }
            for (n, l) in fields {
                if l.trim_start().starts_with("pub") {
                    bad.push(format!(
                        "(4) {path}:{n}: a pub field on Reconciled: {}",
                        l.trim()
                    ));
                }
            }
            // (3)'s derived half reads EVERY attribute line above the struct, back to the previous
            // item's last code line (one ending `}`, `;` or `{`), joined, so another attribute
            // between the derive and the struct, or a derive list rustfmt wrapped one name per
            // line, hides nothing. *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE; it
            // read only the line directly above the struct.)*
            if let Some(i) = lines
                .iter()
                .position(|(_, l)| l.trim_start().starts_with("pub struct Reconciled {"))
            {
                let mut attrs: Vec<&str> = lines[..i]
                    .iter()
                    .rev()
                    .map(|(_, l)| l.trim())
                    .take_while(|t| !(t.ends_with('}') || t.ends_with(';') || t.ends_with('{')))
                    .collect();
                attrs.reverse();
                let joined = attrs.join(" ");
                // *(dated 2026-09-29, s8f build, package R3FIX-8f; refuting round 3.)*
                // ~~`has_word(&joined, "Default")`~~ — every name in every `derive(…)` of the
                // joined attributes (a `cfg_attr` one included) must be in [`NON_CONSTRUCTING`]:
                // an allowlist, so `serde::Deserialize`, a bare `Deserialize` or a derive this lint
                // has never heard of is refused, not only `Default`.
                for name in derived_names(&joined) {
                    if !NON_CONSTRUCTING.contains(&name.as_str()) {
                        bad.push(format!(
                            "(3) {path}:{}: Reconciled derives `{name}`, which can build one: \
                             {joined}",
                            lines[i].0
                        ));
                    }
                }
            }
        }
    }
    let in_entry_pass = |(p, _, f): &(String, usize, String)| {
        p == "crates/rk-store/src/render.rs" && f.contains("fn reconcile_entry(")
    };
    if literals.len() != 1 || !literals.iter().all(in_entry_pass) {
        bad.push(format!("(1) the struct literal `Reconciled {{` is not exactly once, inside reconcile_entry: {literals:?}"));
    }
    if returns.len() != 1 || !returns.iter().all(in_entry_pass) {
        bad.push(format!(
            "(2) a fn other than reconcile_entry returns a Reconciled: {returns:?}"
        ));
    }
    if bindings.len() != 1
        || !bindings.iter().all(|(p, _, f, l)| {
            p == "crates/rk-cli/src/verbs/run.rs"
                && f.contains("fn run_lane(")
                && l.ends_with("= None;")
        })
    {
        bad.push(format!("(5) rk-cli's Option<Reconciled> is not one function-local `= None` in run_lane: {bindings:?}"));
    }
    bad
}

/// Every `.rs` file under `crates/*/src`, as `(repo-relative path, source)`.
fn workspace_sources() -> Vec<(String, String)> {
    fn walk(dir: &Path, rel: &str, out: &mut Vec<(String, String)>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir)
            .expect("a source directory reads")
            .filter_map(Result::ok)
            .collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for e in entries {
            let name = e.file_name().to_string_lossy().into_owned();
            let path = e.path();
            if path.is_dir() {
                walk(&path, &format!("{rel}/{name}"), out);
            } else if name.ends_with(".rs") {
                let src = std::fs::read_to_string(&path).expect("a source file reads");
                out.push((format!("{rel}/{name}"), src));
            }
        }
    }
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .to_path_buf();
    let mut out = Vec::new();
    let mut members: Vec<_> = std::fs::read_dir(&crates)
        .expect("crates/ reads")
        .filter_map(Result::ok)
        .collect();
    members.sort_by_key(std::fs::DirEntry::file_name);
    for m in members {
        let src = m.path().join("src");
        if src.is_dir() {
            walk(
                &src,
                &format!("crates/{}/src", m.file_name().to_string_lossy()),
                &mut out,
            );
        }
    }
    out
}

/// **D21-109's construction lint (mandatory; floor +1)**: a [`Reconciled`] — the proof a reader
/// needs that its entry's reconcile ran (AC-126) — is built ONLY by the entry pass, so no lane can
/// serve a read before its entry's reconcile. M24's lane arm is unobservable by construction
/// (D21-109); this lint pins the construction instead. CODE LINES ONLY under `crates/*/src`, with
/// EXACT counts:
/// 1. the struct literal `Reconciled {` exactly once, inside `reconcile_entry`;
/// 2. no fn but `reconcile_entry` returns a `Reconciled`, and no fn in `impl Reconciled` returns
///    `Self`; *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE)* in EVERY `impl …
///    Reconciled … {` block of any scanned file, no return type names `Self` or `Reconciled` in
///    any form (`-> Option<Self>`, `-> Result<Self, _>`), and no `Self {` / `Self(` construction
///    appears (reported as (1));
/// 3. no `Default` or `From` impl (and no derived `Default`); *(dated 2026-09-29, s8f build,
///    package R2FIX-8f-A-CODE)* the derive is read from every attribute line above the struct,
///    joined, so an attribute between or a rustfmt-wrapped derive list hides nothing;
/// 4. no `pub` field;
/// 5. `rk-cli`'s `Option<Reconciled>` is ONE function-local `let … = None;` in the invocation
///    (`run_lane`);
/// 6. no `static` / `OnceLock` / `LazyLock` / `OnceCell` / `thread_local!` holder of one;
/// 7. *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE)* no second name for it: no `type …
///    = …Reconciled…;` (one line, or wrapped after its `=`) and no `use …Reconciled as …`, since
///    every check above keys on the word `Reconciled`.
///
/// Every check is driven red first over a planted file set, and the unplanted synthetic set is
/// clean, so the silence over the real tree is the lint's and not a scan that matches nothing.
/// Mutant M24-G plants each of these in the real source in turn and flips this test.
/// *(Added 2026-09-29, s8f build, package S8F-P8-DOCTOR-RULES-AND-NOTES; D21-109 ruling 2, taken
/// provisionally under D21-105.)*
#[test]
fn a_reconciled_is_built_only_by_the_entry_pass() {
    let store = "crates/rk-store/src/render.rs".to_owned();
    let cli = "crates/rk-cli/src/verbs/run.rs".to_owned();
    let good_store = "#[derive(Debug, Clone, PartialEq, Eq)]\npub struct Reconciled {\n    entry: usize,\n}\n\nimpl Reconciled {\n    pub const fn entry(&self) -> usize {\n        self.entry\n    }\n}\n\npub fn reconcile_entry(\n    w: &W,\n) -> Result<Reconciled, E> {\n    let done = Reconciled {\n        entry: 0,\n    };\n    Ok(done)\n}\n";
    let good_cli = "fn run_lane(\n    x: u8,\n) {\n    // let x: Option<Reconciled> = static;\n    let mut reconciled: Option<rk_store::render::Reconciled> = None;\n}\n\nfn stage_seams(\n    reconciled: &mut Option<rk_store::render::Reconciled>,\n) {\n}\n";
    let set = |s: &str, c: &str| vec![(store.clone(), s.to_owned()), (cli.clone(), c.to_owned())];
    assert_eq!(
        reconciled_violations(&set(good_store, good_cli)),
        Vec::<String>::new(),
        "CONTROL: the unplanted synthetic set is clean"
    );
    let plants: [(&str, String, String); 7] = [
        (
            "(1)",
            format!(
                "{good_store}\npub fn second() -> usize {{\n    Reconciled {{ entry: 1 }}.entry\n}}\n"
            ),
            good_cli.to_owned(),
        ),
        (
            "(2)",
            good_store.replace(
                "    pub const fn entry(&self) -> usize {\n        self.entry\n    }\n",
                "    pub const fn empty() -> Self {\n        Self { entry: 0 }\n    }\n",
            ),
            good_cli.to_owned(),
        ),
        (
            "(3)",
            format!(
                "{good_store}\nimpl Default for Reconciled {{\n    fn default() -> Self {{\n        Self {{ entry: 0 }}\n    }}\n}}\n"
            ),
            good_cli.to_owned(),
        ),
        (
            "(3)",
            good_store.replace(
                "#[derive(Debug, Clone, PartialEq, Eq)]",
                "#[derive(Debug, Clone, Default, PartialEq, Eq)]",
            ),
            good_cli.to_owned(),
        ),
        (
            "(4)",
            good_store.replace(
                "pub struct Reconciled {\n    entry",
                "pub struct Reconciled {\n    pub entry",
            ),
            good_cli.to_owned(),
        ),
        (
            "(5)",
            good_store.to_owned(),
            good_cli.replace("= None;", "= Option::default();"),
        ),
        (
            "(6)",
            good_store.to_owned(),
            format!(
                "{good_cli}\npub static HELD: std::sync::OnceLock<Option<rk_store::render::Reconciled>> = std::sync::OnceLock::new();\n"
            ),
        ),
    ];
    for (check, s, c) in &plants {
        let found = reconciled_violations(&set(s, c));
        assert!(
            found.iter().any(|v| v.starts_with(check)),
            "CONTROL: the planted {check} violation was not found: {found:?}"
        );
    }
    // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE; round-1 tests-mutants major.)* Three
    // more plants, each of which built a Reconciled outside the entry pass and left the lint green:
    // (A) a SECOND inherent `impl Reconciled` block with a `-> Self` fn building `Self { … }`; (B)
    // `-> Option<Self>` in the first block; (C) a `Self { … }` construction in a fn that returns
    // no Self; and (D) `-> Result<Self, E>` in a block whose head is spelled
    // `impl rk_store::render::Reconciled {`, in another file of the crate.
    let second_block =
        "\nimpl Reconciled {\n    pub fn forged() -> Self {\n        Self { entry: 0 }\n    }\n}\n";
    let widened: [(&str, &str, Files); 4] = [
        (
            "(A)",
            "(2)",
            set(&format!("{good_store}{second_block}"), good_cli),
        ),
        (
            "(B)",
            "(2)",
            set(
                &good_store.replace(
                    "    pub const fn entry(&self) -> usize {\n        self.entry\n    }\n",
                    "    pub fn forged() -> Option<Self> {\n        Some(Self { entry: 0 })\n    }\n",
                ),
                good_cli,
            ),
        ),
        (
            "(C)",
            "(1)",
            set(
                &good_store.replace(
                    "    pub const fn entry(&self) -> usize {\n        self.entry\n    }\n",
                    "    pub fn leak(n: usize) -> usize {\n        let r = Self { entry: n };\n        r.entry\n    }\n",
                ),
                good_cli,
            ),
        ),
        ("(D)", "(2)", {
            let mut s = set(good_store, good_cli);
            s.push((
                "crates/rk-store/src/other.rs".to_owned(),
                "impl crate::render::Reconciled {\n    pub fn forged() -> Result<Self, ()> {\n        Ok(Self { entry: 0 })\n    }\n}\n".to_owned(),
            ));
            s
        }),
    ];
    for (plant, check, files) in &widened {
        let found = reconciled_violations(files);
        assert!(
            found.iter().any(|v| v.starts_with(check)),
            "CONTROL: plant {plant}'s {check} violation was not found: {found:?}"
        );
    }
    // *(dated 2026-09-29, s8f build, package R2FIX-8f-A-CODE; refuting round 2's tests-mutants
    // major.)* Four more plants, each of which built a Reconciled outside the entry pass, or
    // derived its Default, and left the lint green: (E) a `pub type` alias and a `pub fn`
    // returning and building the alias; (E2) `use … Reconciled as …` in another file, the same
    // way; (E3) a rustfmt-wrapped alias, `type … =` on one line and `Reconciled;` on the next; (F)
    // `Default` in the derive with another attribute between it and the struct; (G) a
    // rustfmt-wrapped derive list, one name per line.
    let aliased: [(&str, &str, Files); 5] = [
        (
            "(E)",
            "(7)",
            set(
                &format!(
                    "{good_store}\npub type ForgedAlias = Reconciled;\n\n#[must_use]\npub fn forged_alias() -> ForgedAlias {{\n    ForgedAlias {{ entry: 0 }}\n}}\n"
                ),
                good_cli,
            ),
        ),
        ("(E2)", "(7)", {
            let mut s = set(good_store, good_cli);
            s.push((
                "crates/rk-store/src/other.rs".to_owned(),
                "use crate::render::Reconciled as Forged;\n\npub fn forged() -> Forged {\n    Forged { entry: 0 }\n}\n".to_owned(),
            ));
            s
        }),
        (
            "(E3)",
            "(7)",
            set(
                &format!(
                    "{good_store}\npub type ForgedAliasWithAVeryLongNameThatRustfmtWraps =\n    Reconciled;\n"
                ),
                good_cli,
            ),
        ),
        (
            "(F)",
            "(3)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(Debug, Clone, PartialEq, Eq, Default)]\n#[non_exhaustive]",
                ),
                good_cli,
            ),
        ),
        (
            "(G)",
            "(3)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(\n    Debug,\n    Clone,\n    Default,\n    PartialEq,\n    Eq,\n)]",
                ),
                good_cli,
            ),
        ),
    ];
    let missed: Vec<String> = aliased
        .iter()
        .filter_map(|(plant, check, files)| {
            let found = reconciled_violations(files);
            (!found.iter().any(|v| v.starts_with(check)))
                .then(|| format!("plant {plant}'s {check}: {found:?}"))
        })
        .collect();
    assert_eq!(
        missed,
        Vec::<String>::new(),
        "CONTROL: a planted violation was not found"
    );
    // *(dated 2026-09-29, s8f build, package R3FIX-8f; refuting round 3's tests-mutants major.)*
    // (3) keyed on `Default` alone, and a derived `serde::Deserialize` let any crate build a
    // Reconciled from JSON with the lint green (measured by the finder at `bab968e`). Six more
    // plants, each (3): (H) `serde::Deserialize` in the derive; (H2) `Deserialize` by its bare name
    // in a rustfmt-wrapped list; (H3) a `cfg_attr` derive on its own attribute line; (H4) a derive
    // of a trait this lint has never heard of (the derive list is an allowlist, not a denylist);
    // (I) a manual `impl<'de> serde::Deserialize<'de>` whose head rustfmt wrapped before `for
    // Reconciled`, so the (2b) block scan never opens it; (J) `impl std::str::FromStr for
    // Reconciled`.
    let deserialize_impl = "\nimpl<'de> serde::Deserialize<'de>\n    for Reconciled\n{\n    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {\n        todo!()\n    }\n}\n";
    let constructing: [(&str, Files); 6] = [
        (
            "(H)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]",
                ),
                good_cli,
            ),
        ),
        (
            "(H2)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(\n    Debug,\n    Clone,\n    PartialEq,\n    Eq,\n    Deserialize,\n)]",
                ),
                good_cli,
            ),
        ),
        (
            "(H3)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(Debug, Clone, PartialEq, Eq)]\n#[cfg_attr(feature = \"wire\", derive(Deserialize))]",
                ),
                good_cli,
            ),
        ),
        (
            "(H4)",
            set(
                &good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(Debug, Clone, PartialEq, Eq, arbitrary::Arbitrary)]",
                ),
                good_cli,
            ),
        ),
        (
            "(I)",
            set(&format!("{good_store}{deserialize_impl}"), good_cli),
        ),
        (
            "(J)",
            set(
                &format!(
                    "{good_store}\nimpl std::str::FromStr for Reconciled {{\n    type Err = ();\n    fn from_str(_: &str) -> Result<Self, ()> {{\n        Err(())\n    }}\n}}\n"
                ),
                good_cli,
            ),
        ),
    ];
    let missed: Vec<String> = constructing
        .iter()
        .filter_map(|(plant, files)| {
            let found = reconciled_violations(files);
            (!found.iter().any(|v| v.starts_with("(3)")))
                .then(|| format!("plant {plant}'s (3): {found:?}"))
        })
        .collect();
    assert_eq!(
        missed,
        Vec::<String>::new(),
        "CONTROL: a planted constructing derive or impl was not found"
    );
    // CONTROL: the widening does not over-reach. A `Display` impl constructs nothing, and the
    // derive's non-constructing names (`Hash`, `PartialOrd`, `Ord`) are admitted.
    let display = "\nimpl std::fmt::Display for Reconciled {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{}\", self.entry)\n    }\n}\n";
    assert_eq!(
        reconciled_violations(&set(
            &format!(
                "{}{display}",
                good_store.replace(
                    "#[derive(Debug, Clone, PartialEq, Eq)]",
                    "#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]",
                )
            ),
            good_cli
        )),
        Vec::<String>::new(),
        "CONTROL: a non-constructing impl and derive are clean"
    );

    let sources = workspace_sources();
    assert!(
        sources
            .iter()
            .any(|(p, _)| p == "crates/rk-store/src/render.rs")
            && sources
                .iter()
                .any(|(p, _)| p == "crates/rk-cli/src/verbs/run.rs"),
        "PREMISE: the scan reads rk-store's render.rs and rk-cli's run.rs ({} files)",
        sources.len()
    );
    assert_eq!(
        reconciled_violations(&sources),
        Vec::<String>::new(),
        "a Reconciled can be built, returned, defaulted, exposed or held outside the entry pass"
    );
}

// ------------------------------------------------------------------------------- the keyed write

// D21-122 (TAKEN PROVISIONALLY under D21-105; ADR-0191, proposed, supersedes ADR-0189 decision 8's
// member list in part): the keyed write carries a `key` member, and a second DISTINCT key for one
// kind in one stage run is refused by name before any write. *(Added 2026-09-29, s8g build,
// package R-D2122-D2123-8g.)*

/// The `artifact.write` rows of `kind`, as `(key, digest)`.
fn keyed_rows(rows: &Rows, kind: &str) -> Vec<(Option<String>, String)> {
    rows_of(rows, EventKind::ArtifactWrite)
        .into_iter()
        .filter(|e| e.payload["kind"] == json!(kind))
        .map(|e| {
            (
                e.payload["key"].as_str().map(str::to_owned),
                e.payload["digest"].as_str().unwrap_or_default().to_owned(),
            )
        })
        .collect()
}

/// **A second key is refused by name** (D21-122 item 2): after `TestReport{rust}`, a write of
/// `TestReport{node}` in the same stage run is `KeyedSidecarCollision`, before any write: the
/// journal still holds exactly one row, and the sidecar's bytes are still rust's.
#[test]
fn write_keyed_refuses_a_second_distinct_key_by_name() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    let w = writer(dir.path(), &rows);
    let rust = json!({ "passed": 7, "toolchain": "rust" });
    let node = json!({ "passed": 3, "toolchain": "node" });

    let first = w
        .write_keyed("TestReport", "rust", &rust)
        .expect("the first key writes");
    let sidecar = dir.path().join(&first.path);
    let before = std::fs::read(&sidecar).expect("the first sidecar is written");

    let refused = w.write_keyed("TestReport", "node", &node);
    assert_eq!(
        refused,
        Err(RenderError::KeyedSidecarCollision {
            kind: "TestReport".to_owned(),
            prior_key: "rust".to_owned(),
            key: "node".to_owned(),
        }),
        "a second distinct key for one kind in one stage run was not refused by name"
    );
    let named = refused.expect_err("refused").to_string();
    assert!(
        named.starts_with(concat!(
            "keyed-sidecar-collision: TestReport",
            "{node} after TestReport",
            "{rust}"
        )),
        "the refusal names the case: {named}"
    );
    assert_eq!(
        keyed_rows(&rows, "TestReport"),
        vec![(Some("rust".to_owned()), first.digest.as_str().to_owned())],
        "the refused write appended a row"
    );
    assert_eq!(
        std::fs::read(&sidecar).expect("the sidecar is still there"),
        before,
        "the refused write replaced the first key's sidecar"
    );

    // CONTROL: another stage run of the same item holds no prior key, so the same write is taken.
    let other = ArtifactWriter::new(
        dir.path(),
        StageRun {
            item: ITEM.to_owned(),
            stage: STAGE.to_owned(),
            run: "implement-fedcba9876543210".to_owned(),
        },
        schemas(),
        journal_over(&rows),
    );
    other
        .write_keyed("TestReport", "node", &node)
        .expect("CONTROL: another stage run's write is not a collision");
    assert_eq!(keyed_rows(&rows, "TestReport").len(), 2);
}

/// **The same key replaces** (D21-122 item 2; ADR-0189 decision 3 stands): two writes of
/// `TestReport{rust}` append two rows, and the sidecar and the last row carry the last digest.
#[test]
fn write_keyed_same_key_replaces() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let (_, rows) = journal();
    let w = writer(dir.path(), &rows);
    let one = w
        .write_keyed("TestReport", "rust", &json!({ "passed": 1 }))
        .expect("the first write");
    let two = w
        .write_keyed("TestReport", "rust", &json!({ "passed": 2 }))
        .expect("the same key writes again");
    assert_ne!(one.digest, two.digest, "PREMISE: the two values differ");
    let rows_now = keyed_rows(&rows, "TestReport");
    assert_eq!(
        rows_now,
        vec![
            (Some("rust".to_owned()), one.digest.as_str().to_owned()),
            (Some("rust".to_owned()), two.digest.as_str().to_owned()),
        ]
    );
    let bytes = std::fs::read(dir.path().join(&two.path)).expect("the sidecar");
    assert_eq!(
        sha256(&bytes).as_str(),
        two.digest.as_str(),
        "the last write's bytes are the sidecar's"
    );
}
