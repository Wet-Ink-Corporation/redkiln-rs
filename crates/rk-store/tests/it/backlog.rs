//! **The `toolchain:backlog` producer** (`s8f` §8f.7; D21-46 R7; the render record ADR-0189
//! decision 12, proposed). *(Added 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! The ids come from JOURNAL rows (a tier-1 `stage_transition` and an `item_create` step's
//! `ItemRefs` output), and the records from files each test writes at chosen paths, including
//! files no row names — which must never appear.

use std::collections::BTreeSet;
use std::path::Path;

use rk_store::backlog::{ParsedRecord, canonical_bytes, names, snapshot, to_value};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::verb_output::{BacklogAbsent, NamedBy};
use serde_json::Value;

/// A tier-1 boundary naming `item` (it carries `from`).
fn boundary(item: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": "triage", "to": "implement", "item": item }),
    )
    .with_item(item)
}

/// A lane ENTRY (no `from`): not a tier-1 boundary, so it names nothing.
fn entry(item: &str) -> Event {
    Event::new(
        EventKind::StageTransition,
        serde_json::json!({ "from": null, "to": "triage", "item": item }),
    )
    .with_item(item)
}

/// An `item_create` step's committed `ItemRefs` output listing `ids`.
fn item_refs(ids: &[&str]) -> Event {
    Event::new(
        EventKind::ActivityCommitted,
        serde_json::json!({
            "activity_id_at_version": "debt-logging@1",
            "attempt": 2,
            "item": "src-1",
            "iteration_path": [],
            "result": { "ids": ids },
            "run_id": "closeout-1",
            "schema": "ItemRefs",
            "step_id": "file",
        }),
    )
}

fn write(root: &Path, rel: &str, text: &str) {
    let path = root.join(rel);
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("the directory");
    std::fs::write(path, text).expect("the record writes");
}

fn record(id: &str, stage: &str) -> String {
    format!(
        "id = \"{id}\"\nkind = \"tech-debt\"\nstage = \"{stage}\"\n\n[facets]\nsize = \"low\"\n"
    )
}

/// The test's record parser: the line-per-key subset [`record`] writes. The product's parser is
/// the composition root's; this one reads only what these tests write.
fn parse(bytes: &[u8]) -> Result<ParsedRecord, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    let mut out = ParsedRecord::default();
    let mut table = "";
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(t) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            table = if t == "facets" { "facets" } else { "other" };
            continue;
        }
        let (k, v) = line
            .split_once('=')
            .ok_or_else(|| format!("not a key: {line}"))?;
        let v = v.trim().trim_matches('"').to_owned();
        match (table, k.trim()) {
            ("", "id") => out.id = Some(v),
            ("", "kind") => out.kind = Some(v),
            ("", "stage") => out.stage = Some(v),
            ("facets", key) => {
                out.facets.insert(key.to_owned(), Value::String(v));
            }
            _ => {}
        }
    }
    Ok(out)
}

/// **Only the named ids, each read by its exact path.** The tree holds an unnamed
/// `.redkiln/items/stray.toml` and a named id's copy under `.redkiln/worktrees/x/.redkiln/items/`
/// (and none at the top), and neither appears as an item. M32 walks `.redkiln/items/`.
#[test]
fn reads_only_the_items_the_store_names_each_by_exact_path() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    write(
        root,
        ".redkiln/items/named-a.toml",
        &record("named-a", "implement"),
    );
    write(
        root,
        ".redkiln/items/stray.toml",
        &record("stray", "triage"),
    );
    write(
        root,
        ".redkiln/items/entered.toml",
        &record("entered", "triage"),
    );
    write(
        root,
        // *(dated 2026-09-29, s8f build, package R1FIX-8f-A-CODE.)* The one planted-copy literal
        // Checkpoint 8d-ii (3)'s root floor admits by a dated exclusion: `rk-store` depends on
        // neither `rk-ports` nor `rk-git` (§8f.6 item 4), so it cannot take the path from rk-git's
        // one derivation, and this is data planted for the reader NOT to find.
        ".redkiln/worktrees/x/.redkiln/items/in-worktree.toml",
        &record("in-worktree", "triage"),
    );
    let rows = vec![
        boundary("named-a"),
        entry("entered"),
        item_refs(&["in-worktree"]),
    ];
    let named = names(&rows);
    assert_eq!(
        named.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["in-worktree", "named-a"],
        "the store names a tier-1 boundary's item and an ItemRefs id, and a lane entry names none"
    );
    let snap = snapshot(root, &named, &parse).expect("the snapshot reads");
    let ids: Vec<&str> = snap.items.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(
        ids,
        vec!["named-a"],
        "an item no row names (`stray`, `entered`) or a copy in a run tree is in the feed"
    );
    assert_eq!(
        snap.absent,
        vec![BacklogAbsent {
            id: "in-worktree".to_owned(),
            named_by: vec![NamedBy::ItemWrite],
        }],
        "the named id whose only copy is in a run tree has no record HERE"
    );
    let a = &snap.items[0];
    assert_eq!(
        (a.kind.as_deref(), a.stage.as_deref(), a.named_by.clone()),
        (
            Some("tech-debt"),
            Some("implement"),
            vec![NamedBy::StageTransition]
        )
    );
    assert_eq!(a.facets.get("size"), Some(&Value::String("low".to_owned())));
    let bytes = std::fs::read(root.join(".redkiln/items/named-a.toml")).expect("the record");
    assert_eq!(
        a.record_digest,
        rk_types::digest::sha256(&bytes).as_str(),
        "record_digest is the rk-types digest of the record's bytes"
    );
}

/// **A named id with no record is listed absent, with what named it — never dropped.** M33 drops it.
#[test]
fn a_named_item_with_no_record_is_listed_absent_never_dropped() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    write(
        root,
        ".redkiln/items/present.toml",
        &record("present", "triage"),
    );
    let named = names(&[
        boundary("present"),
        item_refs(&["minted-elsewhere"]),
        boundary("gone"),
        item_refs(&["gone"]),
    ]);
    let snap = snapshot(root, &named, &parse).expect("the snapshot reads");
    assert_eq!(
        snap.items.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
        vec!["present"]
    );
    assert_eq!(
        snap.absent,
        vec![
            BacklogAbsent {
                id: "gone".to_owned(),
                named_by: vec![NamedBy::StageTransition, NamedBy::ItemWrite],
            },
            BacklogAbsent {
                id: "minted-elsewhere".to_owned(),
                named_by: vec![NamedBy::ItemWrite],
            },
        ],
        "each named id with no record is listed absent, ordered by id, with its sorted named_by"
    );
}

/// **The snapshot is canonical**: items ordered by id whatever the journal's order, `named_by`
/// sorted, `provenance` present as `null` when the record has none, and the canonical bytes
/// identical twice and equal to the verb's output value re-canonicalised.
#[test]
fn the_snapshot_is_canonical_ordered_by_id_and_byte_identical_twice() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let root = dir.path();
    for id in ["zed", "alpha", "mid"] {
        write(
            root,
            &format!(".redkiln/items/{id}.toml"),
            &record(id, "triage"),
        );
    }
    let rows = vec![
        item_refs(&["zed"]),
        boundary("mid"),
        boundary("zed"),
        item_refs(&["alpha"]),
    ];
    let one = snapshot(root, &names(&rows), &parse).expect("the snapshot reads");
    let mut reversed = rows.clone();
    reversed.reverse();
    let two = snapshot(root, &names(&reversed), &parse).expect("the snapshot reads");
    assert_eq!(
        one.items.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
        vec!["alpha", "mid", "zed"],
        "items are ordered by id"
    );
    assert_eq!(
        one.items[2].named_by,
        vec![NamedBy::StageTransition, NamedBy::ItemWrite],
        "named_by is sorted: stage_transition before item_write"
    );
    let bytes = canonical_bytes(&one).expect("canonical");
    assert_eq!(
        bytes,
        canonical_bytes(&two).expect("canonical"),
        "journal order moved the bytes"
    );
    assert_eq!(
        bytes,
        canonical_bytes(&one).expect("canonical"),
        "not byte-identical twice"
    );
    let value = to_value(&one).expect("the output value");
    assert_eq!(
        rk_store::canon::canonical(&value).expect("canonical"),
        bytes,
        "the verb's output value is not the canonical snapshot"
    );
    assert_eq!(
        value["items"][0]["provenance"],
        Value::Null,
        "an absent provenance is `null`"
    );
    let keys: BTreeSet<&str> = value["items"][0]
        .as_object()
        .expect("an object")
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        keys,
        [
            "facets",
            "id",
            "kind",
            "named_by",
            "provenance",
            "record_digest",
            "stage"
        ]
        .into_iter()
        .collect(),
        "an entry carries exactly id, kind, stage, facets, provenance, record_digest and named_by"
    );
}
