//! **ADR-0202 (proposed): an `artifact:` input reads a human-attached root** when the item has no
//! committed `emit` binding of the kind. *(Added 2026-10-18, `s8j` build, package S8J-C4-FIX D1;
//! the operator's ruling A. Amended the same day, review of PR #106 F1: the stored copy is served,
//! not the attach row's `attached_path`.)*
//!
//! The root is the item's latest attach row of the kind (an `artifact.write` carrying
//! `attached_path`, which only `answer` writes, ADR-0189 decision 8). What is served is the vouched
//! pass's STORED copy of it, `<Kind>.root<ext>` in the item's managed directory, as `rk-store`'s
//! typed reader `vouched::stored` names it: `{kind, path, digest}`, the digest being the vouched
//! digest the bytes at `path` had when the pass stored them (a later edit of the stored copy is
//! AC-127's advisory drift, not a new digest here). An attached root with no stored copy under its vouched digest is refused by name (a `?`
//! input is `null`), never read from `attached_path`. No read-seam call is made for a root. An
//! `emit` binding, when there is one, still wins and still goes through the seam.

use rk_kernel::catalog::StepKind;
use rk_kernel::runtime::reference::{Env, Scope, resolve_text};
use rk_kernel::workflow::memo::ToolchainValues;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::reference_resolver::{CountingSeam, emitted_row, step};
use crate::runtime::{AID, ITEM, STAGE};

type Json = serde_json::Value;

/// Where the operator's attachment lies in the invoking checkout: never what is served.
const ATTACHED: &str = ".redkiln/intake/INC-1.BugReport.md";
/// Where the vouched pass stores it in the item's managed directory.
const STORED: &str = ".redkiln/artifacts/INC-1/BugReport.root.md";
/// The `schema` `.redkiln/artifacts.toml` registers for `BugReport`: what `answer` records as an
/// attach row's `schema_version`, and what the writer's `write_root` records on the stored row.
const SCHEMA: &str = "BugReport";

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
            "schema_version": SCHEMA,
            "digest": digest_of(bytes),
            "stage": "analysis",
            "run": "analysis-run",
            "attached_path": path,
        }),
    )
    .with_item(item)
}

/// The writer row the vouched pass appends when it stores a root at `path` (no `attached_path`).
fn stored_row(item: &str, kind: &str, path: &str, bytes: &[u8]) -> Event {
    Event::new(
        EventKind::ArtifactWrite,
        serde_json::json!({
            "kind": kind,
            "item": item,
            "schema_version": SCHEMA,
            "digest": digest_of(bytes),
            "stage": "analysis",
            "run": "analysis-run",
            "path": path,
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
fn a_stored_human_root_resolves_an_artifact_input_with_no_emit_binding() {
    let events = vec![
        attach_row(ITEM, "BugReport", ATTACHED, b"the report"),
        stored_row(ITEM, "BugReport", STORED, b"the report"),
    ];
    let seam = CountingSeam::default();

    let whole = resolve_over(&events, &seam, "artifact:BugReport");
    let field = resolve_over(&events, &seam, "artifact:BugReport.path");

    assert_eq!(
        whole,
        Ok(serde_json::json!({
            "kind": "BugReport",
            "path": STORED,
            "digest": digest_of(b"the report"),
        })),
        "the stored, digest-checked copy in the managed directory is served"
    );
    assert_eq!(field, Ok(serde_json::json!(STORED)));
    assert_eq!(
        seam.calls(),
        0,
        "a root is no emit binding: the sidecar seam is not called for it"
    );
}

#[test]
fn an_attached_root_the_vouched_pass_has_not_stored_is_refused_by_name() {
    let events = vec![attach_row(ITEM, "BugReport", ATTACHED, b"the report")];
    let seam = CountingSeam::default();

    let mandatory = resolve_over(&events, &seam, "artifact:BugReport");
    let optional = resolve_over(&events, &seam, "artifact:BugReport?");

    let refusal = mandatory.expect_err("an unstored root is never read from `attached_path`");
    assert!(
        refusal.to_string().contains("BugReport") && refusal.to_string().contains("not stored"),
        "the refusal names the kind and that its stored copy is missing: {refusal}"
    );
    assert_eq!(optional, Ok(Json::Null));
}

#[test]
fn a_stored_copy_under_another_digest_is_not_the_root() {
    // Stored once, then re-attached with other bytes: the earlier copy is not the vouched root.
    let events = vec![
        attach_row(ITEM, "BugReport", ATTACHED, b"first"),
        stored_row(ITEM, "BugReport", STORED, b"first"),
        attach_row(ITEM, "BugReport", ATTACHED, b"second"),
    ];
    let seam = CountingSeam::default();

    let value = resolve_over(&events, &seam, "artifact:BugReport");

    assert!(
        value.is_err_and(|e| e.to_string().contains("not stored")),
        "a re-attach is not served until its own bytes are stored"
    );
}

#[test]
fn an_emit_binding_of_the_kind_wins_over_an_attached_root() {
    let bound = serde_json::json!({"title": "the emitted report"});
    let events = vec![
        attach_row(ITEM, "BugReport", ATTACHED, b"the report"),
        stored_row(ITEM, "BugReport", STORED, b"the report"),
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
        attach_row("INC-2", "BugReport", ATTACHED, b"another item's report"),
        stored_row(
            "INC-2",
            "BugReport",
            ".redkiln/artifacts/INC-2/BugReport.root.md",
            b"another item's report",
        ),
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
fn the_latest_attach_rows_stored_copy_is_the_root_served() {
    let events = vec![
        attach_row(ITEM, "BugReport", ATTACHED, b"first"),
        stored_row(ITEM, "BugReport", STORED, b"first"),
        attach_row(ITEM, "BugReport", ATTACHED, b"second"),
        stored_row(ITEM, "BugReport", STORED, b"second"),
    ];
    let seam = CountingSeam::default();

    let value = resolve_over(&events, &seam, "artifact:BugReport");

    assert_eq!(
        value,
        Ok(serde_json::json!({
            "kind": "BugReport",
            "path": STORED,
            "digest": digest_of(b"second"),
        })),
        "a re-attach replaces the root once its bytes are stored"
    );
}
