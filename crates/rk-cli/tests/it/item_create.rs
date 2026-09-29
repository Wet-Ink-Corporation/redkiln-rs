//! **A minted debt item, run** (`s8f` §8f.7; the render record ADR-0189 decision 11, proposed; D21-42
//! 6a: *"human-sourced facets are null and go to `unknown_facet`"*). *(Added 2026-09-28, s8f build,
//! package S8F-P6-ITEMS-BACKLOG-FEEDS.)*
//!
//! The item is minted by `rk-store`'s item writer — the function `item_create`'s `store`-route
//! handler calls — into the fixture repository's `.redkiln/items/`, from an extracted entry that
//! CARRIES a value for the human-sourced `has_repro`. The writer withholds it, so `run`'s
//! `build_item` nulls the declared facet and the selection abstains: the binary opens an
//! `unknown_facet:has_repro` gate and exits 0 at it.
//!
//! **Through the binary**, under `pack-min`'s `Lane` variant, whose `[adapters] harness_binary` the
//! builder rewrites to the fixture child, `rk-fixture-child-claude`, located as the sibling of
//! `redkiln` in the target directory (`support::fixture_child`, the `pack-min` fixtures' route;
//! D21-47, review m15). *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS: §8f.7
//! names `env!("CARGO_BIN_EXE_rk-fixture-child-claude")`, which cargo sets only for
//! `rk-adapter-claude`'s own tests and does not compile here; the route `pack-min` takes is
//! `fixture_child`'s, whose presence is asserted.)* No real `claude` can be spawned.

use std::collections::BTreeMap;
use std::path::Path;

use rk_types::event_kind::EventKind;

use crate::support::{self, PackVariant};

/// Replace `from` with `to` in `path`, asserting it was there exactly once.
fn patch(path: &Path, from: &str, to: &str) {
    let text = std::fs::read_to_string(path).expect("the file reads");
    assert_eq!(
        text.matches(from).count(),
        1,
        "PREMISE: `{from}` occurs once in {}",
        path.display()
    );
    std::fs::write(path, text.replace(from, to)).expect("the file writes");
}

/// **D21-42 6a through the binary**: an item `item_create` minted, when run, abstains on the
/// human-sourced facet the write withheld and opens an `unknown_facet` gate. M14 writes the facet.
#[test]
fn a_minted_debt_item_opens_an_unknown_facet_gate_when_run() {
    let repo = support::pack_min_repo(PackVariant::Lane);
    let root = repo.path();
    let rk = root.join(".redkiln");
    // The lane admits the minted kind, and stops at `implement` (no `review` stage, so no `slice` is
    // owed at the lane's entry, D21-94): the gate is the lane's only stop.
    for pack in ["bug-standard.toml", "story.toml"] {
        let path = rk.join("packs").join(pack);
        patch(
            &path,
            r#"applies = "kind == bug or kind == story""#,
            r#"applies = "kind == bug or kind == story or kind == tech-debt""#,
        );
        patch(
            &path,
            r#"stages = { triage = "optional", discovery = "forbidden", implement = "recommended", review = "mandatory" }"#,
            r#"stages = { triage = "optional", discovery = "forbidden", implement = "recommended" }"#,
        );
        patch(
            &path,
            r#"stage_order = ["triage", "discovery", "implement", "review"]"#,
            r#"stage_order = ["triage", "discovery", "implement"]"#,
        );
        patch(&path, "[stage.review]\nbaseline = [\"Findings\"]\n", "");
    }
    patch(
        &rk.join("facets.toml"),
        r#"values = ["bug", "story"]"#,
        r#"values = ["bug", "story", "tech-debt"]"#,
    );

    // ---- the mint: the facet registry's `source` column, as `run`'s store route reads it -------
    let registry = rk_kernel::registry::Registry::from_toml(
        &std::fs::read_to_string(rk.join("facets.toml")).expect("facets.toml reads"),
    )
    .expect("the fixture facet registry loads");
    let sources: BTreeMap<String, String> = registry
        .facets()
        .map(|f| (f.name.clone(), f.source.as_str().to_owned()))
        .collect();
    assert_eq!(
        sources.get("has_repro").map(String::as_str),
        Some("human"),
        "PREMISE: `has_repro` is human-sourced"
    );
    let out = rk_store::item_write::write_items(
        root,
        &serde_json::json!([{ "title": "a shortcut", "facets": { "has_repro": true } }]),
        &sources,
        &rk_store::item_write::Provenance {
            activity: "debt-logging@1".to_owned(),
            run: "closeout-fixture".to_owned(),
            source_impl_report: "a".repeat(64),
            source_item: "lane-2".to_owned(),
        },
    )
    .expect("the item is minted");
    assert_eq!(out.ids, vec!["lane-2-debt-1".to_owned()]);
    let id = out.ids[0].as_str();

    // ---- run it, through the binary -----------------------------------------------------------
    let run = crate::run::spawn_run(root, id, Some("fix"));
    let stderr = String::from_utf8_lossy(&run.stderr);
    assert_eq!(
        run.status.code(),
        Some(0),
        "`run {id}` exits 0 at its gate; stderr:\n{stderr}"
    );
    let gates: Vec<String> = support::store_rows(root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| e.item() == Some(id) || e.payload["item"].as_str() == Some(id))
        .filter_map(|e| e.payload["kind"].as_str().map(str::to_owned))
        .collect();
    assert_eq!(
        gates,
        vec!["unknown_facet:has_repro".to_owned()],
        "the minted item's selection did not abstain on the withheld facet; stderr:\n{stderr}"
    );
    // ... because the write withheld the extracted value, and named it.
    let record = std::fs::read_to_string(rk.join("items").join(format!("{id}.toml")))
        .expect("the minted record is on disk");
    assert!(
        record.contains("withheld = [{ facet = \"has_repro\", source = \"human\" }]"),
        "the extracted `has_repro` is not withheld and named:\n{record}"
    );
}
