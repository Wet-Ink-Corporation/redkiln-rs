//! **ADR-0202 (proposed): an `artifact:` input reads a human-attached root** when the item has no
//! committed `emit` binding of the kind. *(Added 2026-10-18, `s8j` build, package S8J-C4-FIX D1;
//! the operator's ruling A.)*
//!
//! The root is the item's latest attach row of the kind: an `artifact.write` carrying
//! `attached_path`, which only `answer` writes (ADR-0189 decision 8), read through `rk-store`'s typed
//! reader `vouched::attached`. The value served is that row's reference, `{kind, attached_path,
//! digest}`, and no read-seam call is made for it (the store's seam serves `<Kind>.json` sidecars,
//! and a root has none). An `emit` binding, when there is one, still wins and still goes through
//! the seam.

use rk_kernel::catalog::StepKind;
use rk_kernel::runtime::reference::{Env, Scope, resolve_text};
use rk_kernel::workflow::memo::ToolchainValues;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::reference_resolver::{CountingSeam, emitted_row, step};
use crate::runtime::{AID, ITEM, STAGE};

type Json = serde_json::Value;

const ROOT_PATH: &str = ".redkiln/intake/INC-1.BugReport.md";

/// The digest an attach row records for `bytes`.
fn digest_of(bytes: &[u8]) -> String {
    rk_types::digest::sha256(bytes).as_str().to_owned()
}

/// An attach row as `answer`'s `store_attachments` writes it: one `artifact.write` of `kind` for
/// `item`, carrying `attached_path`.
fn attach_row(item: &str, kind: &str, path: &str, bytes: &[u8]) -> Event {
    Event::new(
        EventKind::ArtifactWrite,
        serde_json::json!({
            "kind": kind,
            "item": item,
            "schema_version": 1,
            "digest": digest_of(bytes),
            "stage": "analysis",
            "run": "analysis-run",
            "attached_path": path,
        }),
    )
    .with_item(item)
}

fn no_feed(verb: &str) -> Result<Json, String> {
    Err(format!("no feed is bound in this test (`{verb}`)"))
}

/// Resolve `text` as the `report` input of an agent step `attempt`, over `events`, through `seam`.
fn resolve_over(
    events: &[Event],
    seam: &CountingSeam,
    text: &str,
) -> Result<Json, rk_kernel::runtime::reference::RefError> {
    let steps = vec![step("attempt", StepKind::Agent)];
    let mut scope = Scope::new(&steps);
    scope.enter("attempt");
    let values = ToolchainValues::default();
    let feed = no_feed;
    let env = Env {
        item: ITEM,
        stage: STAGE,
        activity: AID,
        events,
        values: &values,
        registry: None,
        feed_table: None,
        sources: None,
        seam,
        feed: &feed,
        run_branch: None,
    };
    resolve_text(&env, &scope, "attempt", "report", text)
}

#[test]
fn a_human_attached_root_resolves_an_artifact_input_with_no_emit_binding() {
    let events = vec![attach_row(ITEM, "BugReport", ROOT_PATH, b"the report")];
    let seam = CountingSeam::default();

    let whole = resolve_over(&events, &seam, "artifact:BugReport");
    let field = resolve_over(&events, &seam, "artifact:BugReport.attached_path");

    assert_eq!(
        whole,
        Ok(serde_json::json!({
            "kind": "BugReport",
            "attached_path": ROOT_PATH,
            "digest": digest_of(b"the report"),
        })),
        "the attach row's reference is the input's value"
    );
    assert_eq!(field, Ok(serde_json::json!(ROOT_PATH)));
    assert_eq!(
        seam.calls(),
        0,
        "a root is no emit binding: the sidecar seam is not called for it"
    );
}

#[test]
fn an_emit_binding_of_the_kind_wins_over_an_attached_root() {
    let bound = serde_json::json!({"title": "the emitted report"});
    let events = vec![
        attach_row(ITEM, "BugReport", ROOT_PATH, b"the report"),
        emitted_row("bug-intake@1", "BugReport", bound.clone()),
    ];
    let seam = CountingSeam::default();

    let value = resolve_over(&events, &seam, "artifact:BugReport");

    assert_eq!(value, Ok(bound), "the committed emit binding is served");
    assert_eq!(seam.calls(), 1, "and through the read seam, once");
}

#[test]
fn another_items_attached_root_is_not_read() {
    let events = vec![
        attach_row("INC-2", "BugReport", ROOT_PATH, b"another item's report"),
        // A writer row of the kind for this item (a managed write, no `attached_path`) is no root.
        Event::new(
            EventKind::ArtifactWrite,
            serde_json::json!({
                "kind": "BugReport",
                "item": ITEM,
                "digest": digest_of(b"stored"),
                "path": ".redkiln/artifacts/INC-1/BugReport.root.md",
            }),
        )
        .with_item(ITEM),
    ];
    let seam = CountingSeam::default();

    let mandatory = resolve_over(&events, &seam, "artifact:BugReport");
    let optional = resolve_over(&events, &seam, "artifact:BugReport?");

    let refusal = mandatory.expect_err("no root of this item: the mandatory input refuses");
    assert!(
        refusal.to_string().contains("BugReport") && refusal.to_string().contains(ITEM),
        "the refusal names the kind and the item: {refusal}"
    );
    assert_eq!(optional, Ok(Json::Null));
    assert_eq!(seam.calls(), 0);
}

#[test]
fn the_latest_attach_row_of_the_kind_is_the_root_served() {
    let later = ".redkiln/intake/INC-1.BugReport.v2.md";
    let events = vec![
        attach_row(ITEM, "BugReport", ROOT_PATH, b"first"),
        attach_row(ITEM, "BugReport", later, b"second"),
    ];
    let seam = CountingSeam::default();

    let value = resolve_over(&events, &seam, "artifact:BugReport");

    assert_eq!(
        value,
        Ok(serde_json::json!({
            "kind": "BugReport",
            "attached_path": later,
            "digest": digest_of(b"second"),
        })),
        "a re-attach replaces the root: the latest attach row in journal order is served"
    );
}
