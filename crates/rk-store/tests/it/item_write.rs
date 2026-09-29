//! **`item_create`'s `store`-route handler: the in-repo item write** (`s8f` §8f.7; the render record
//! ADR-0189 decision 11, proposed; D21-42 6a). *(Added 2026-09-28, s8f build, package
//! S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! Every assertion reads the FILE the handler wrote (its exact bytes, or its absence), never a
//! value the handler returned about it alone.

use std::collections::BTreeMap;
use std::path::Path;

use rk_store::item_write::{ItemWriteError, Provenance, item_path, write_items};

const SRC: &str = "bug-7";

fn provenance() -> Provenance {
    Provenance {
        activity: "debt-logging@1".to_owned(),
        run: "closeout-0123abcd".to_owned(),
        source_impl_report: "a".repeat(64),
        source_item: SRC.to_owned(),
    }
}

/// The registry's `source` column for the facets these tests name: one `declared`, three `human`,
/// one `llm`, one `derived`.
fn sources() -> BTreeMap<String, String> {
    [
        ("surfaces", "declared"),
        ("kind", "human"),
        ("size", "human"),
        ("criticality", "human"),
        ("summary", "llm"),
        ("toolchain", "derived"),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v.to_owned()))
    .collect()
}

fn read(root: &Path, id: &str) -> Option<String> {
    std::fs::read(root.join(item_path(id)))
        .ok()
        .map(|b| String::from_utf8(b).expect("the record is UTF-8"))
}

/// The `[facets]` table's lines of a written record.
fn facets_section(text: &str) -> Vec<String> {
    text.split("\n[facets]\n")
        .nth(1)
        .and_then(|rest| rest.split("\n[provenance]\n").next())
        .expect("the record has a [facets] then a [provenance] table")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(str::to_owned)
        .collect()
}

/// **The record, byte for byte**: `ItemRecord`'s key set (`id`, `kind`, `stage = "triage"`,
/// `[facets]`) and a `[provenance]` table naming the activity at its version, the run, the source
/// `ImplReport` digest and the extracted entry's digest — under `.redkiln/items/<id>.toml`.
#[test]
fn debt_item_is_written_under_items_with_its_provenance() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    let items = serde_json::json!([
        { "title": "a shortcut in the parser" },
        { "id": "named-debt", "title": "a second", "facets": { "surfaces": ["api"] } },
    ]);
    let out = write_items(root, &items, &sources(), &provenance()).expect("the write succeeds");
    assert_eq!(
        out.ids,
        vec![format!("{SRC}-debt-1"), "named-debt".to_owned()],
        "the ItemRefs value names both ids in entry order"
    );
    assert_eq!(
        out.written, out.ids,
        "both files are new, so both are written"
    );

    let first = read(root, &format!("{SRC}-debt-1"))
        .expect("the first record is written under .redkiln/items/<id>.toml");
    let entry = rk_store::canon::canonical_digest(&items[0]).expect("canonical");
    let want = format!(
        "id = \"{SRC}-debt-1\"\nkind = \"tech-debt\"\nstage = \"triage\"\n\n[facets]\n\n\
         [provenance]\nactivity = \"debt-logging@1\"\nentry = \"{}\"\nrun = \
         \"closeout-0123abcd\"\nsource_impl_report = \"{}\"\nwithheld = []\n",
        entry.as_str(),
        "a".repeat(64)
    );
    assert_eq!(
        first, want,
        "the record is not the key set plus [provenance]"
    );
    let second = read(root, "named-debt").expect("the entry's own id names its file");
    assert_eq!(
        facets_section(&second),
        vec!["surfaces = [\"api\"]".to_owned()],
        "a `declared` facet the entry carries is written"
    );
    assert!(!second.contains('\r'), "LF only: the record carries a CR");
}

/// **Golden constraint 1**: a `human` or `llm` facet is NEVER written, even when the extracted value
/// carries one, and each withheld facet is NAMED in `[provenance] withheld` with its source. M14
/// writes a human-sourced facet.
#[test]
fn human_sourced_facets_are_never_written_and_are_named() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    let items = serde_json::json!([{
        "kind": "bug",
        "facets": {
            "size": "low",
            "criticality": "high",
            "summary": "the model's summary",
            "toolchain": ["rust"],
            "surfaces": ["cli"],
        },
    }]);
    write_items(root, &items, &sources(), &provenance()).expect("the write succeeds");
    let text = read(root, &format!("{SRC}-debt-1")).expect("the record is written");
    assert_eq!(
        facets_section(&text),
        vec!["surfaces = [\"cli\"]".to_owned()],
        "only the `declared` facet is written: a human, llm or derived facet reached [facets]"
    );
    assert!(
        text.contains("\nkind = \"tech-debt\"\n"),
        "the record's kind is the minted kind, never the extracted human value:\n{text}"
    );
    let withheld = text
        .lines()
        .find(|l| l.starts_with("withheld = "))
        .expect("[provenance] names the withheld facets");
    for (facet, source) in [
        ("criticality", "human"),
        ("kind", "human"),
        ("size", "human"),
        ("summary", "llm"),
        ("toolchain", "derived"),
    ] {
        assert!(
            withheld.contains(&format!("{{ facet = \"{facet}\", source = \"{source}\" }}")),
            "the withheld `{facet}` ({source}) is not named: {withheld}"
        );
    }
    assert!(
        !withheld.contains("surfaces"),
        "a written facet is not withheld: {withheld}"
    );
}

/// **An existing id whose file DIFFERS is refused by name and never overwritten**, and nothing of
/// the call is written — not even a second, new id. M15 overwrites.
#[test]
fn an_existing_item_id_is_refused_never_overwritten() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    let existing = b"id = \"taken\"\nkind = \"bug\"\nstage = \"triage\"\n";
    let path = root.join(item_path("taken"));
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("the items dir");
    std::fs::write(&path, existing).expect("the existing record");
    let items = serde_json::json!([{ "id": "fresh" }, { "id": "taken", "title": "x" }]);
    let e = write_items(root, &items, &sources(), &provenance())
        .expect_err("a differing existing record must refuse the write");
    assert_eq!(
        e,
        ItemWriteError::Exists {
            id: "taken".to_owned(),
            path: item_path("taken"),
        }
    );
    assert!(
        e.to_string().contains("taken"),
        "the refusal names the id: {e}"
    );
    assert_eq!(
        std::fs::read(&path).expect("still there"),
        existing,
        "the existing record was overwritten"
    );
    assert_eq!(
        read(root, "fresh"),
        None,
        "a refused call wrote another entry's record"
    );
}

/// **A byte-identical re-execution is SERVED and writes nothing** (the `on_false` loop-back re-runs
/// `file` under a new iteration path and meets the file it wrote). The second call must not open
/// the file for writing: it is made read-only first, so a write would fail. M29 refuses it.
#[test]
fn a_byte_identical_re_execution_is_served_and_writes_nothing() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    let items = serde_json::json!([{ "title": "t" }]);
    let first = write_items(root, &items, &sources(), &provenance()).expect("the first write");
    let id = first.ids[0].clone();
    assert_eq!(
        first.written,
        vec![id.clone()],
        "PREMISE: the first call wrote"
    );
    let path = root.join(item_path(&id));
    let before = std::fs::read(&path).expect("the record");
    let mut perms = std::fs::metadata(&path).expect("metadata").permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&path, perms.clone()).expect("read-only");

    let again = write_items(root, &items, &sources(), &provenance())
        .expect("a byte-identical re-execution is served, never refused");
    assert_eq!(
        again.ids,
        vec![id.clone()],
        "the served call names the same id"
    );
    assert_eq!(again.served, vec![id], "the id is served");
    assert!(
        again.written.is_empty(),
        "a served call wrote: {:?}",
        again.written
    );
    assert_eq!(std::fs::read(&path).expect("the record"), before);

    #[expect(
        clippy::permissions_set_readonly_false,
        reason = "the temp dir is removed after the test, and a read-only file blocks that on Windows"
    )]
    perms.set_readonly(false);
    std::fs::set_permissions(&path, perms).expect("writable again");
}
