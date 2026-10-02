//! **The review gate's `rk-types` half** — Step 7b-ii, `s7b-ii` §7b-ii.3 and §7b-ii.6. Four ids
//! whose register subject tag is `rk-types`, filtered here by Checkpoint 7b-ii's third loop.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac281_findings_ingest_rejects_an_incomplete_repro`] | **AC-281** | the five negative fixtures, each rejected at ingest AND each refused by the exported JSON schema; the well-formed control round-trips through `serde` and validates | the corpus replay (*100% of accepted items begin with an executable token*) — Step 8's |
//! | [`ac293_review_outcome_carries_the_reviewed_tree_digest`] | **AC-293** | the digest member is a non-optional `TreeId`; a round-trip captures `basis`/`sha`; the digest-removed, empty and `null` fixtures all fail to construct | — |
//! | [`ac319_quarantine_without_until_is_unconstructible`] | **AC-319** | `until` is a non-optional `Timestamp`; the fixture with no `until` fails to construct | the expiry behaviour, which is AC-306's in `rk-kernel` |
//! | [`ac280_decision_fields_are_refused_at_schema_load`] | **AC-280** (`rk-types` half) | the exported `Findings` schema declares no denylisted field; a schema that does is named; fixture cards carrying `pass` and `approve` fail ingest | the `doctor` rule-table row (7d) |
//!
//! Red first, each on its own claim: AC-281's prose case via a temporary mutant of the ingest
//! guard (the behaviour was built at Step 5); AC-293 and AC-319 against skeleton types whose
//! members were `Option` — the exact shape the two criteria forbid; AC-280 against a denylist
//! check that never fired.

use jsonschema::Validator;
use rk_types::artifact::findings::{
    DECISION_FIELD_DENYLIST, EvidenceLabel, Findings, IngestError, decision_fields_declared,
};
use rk_types::registry;
use rk_types::review::{Quarantine, ReviewBasis, ReviewOutcome, TreeId};
use rk_types::text::Text;
use rk_types::time::Timestamp;
use serde_json::json;

const TREE: &str = "aaaa111122223333444455556666777788889999";

fn exported_findings_schema() -> serde_json::Value {
    registry::export_all()
        .into_iter()
        .find(|(name, _)| *name == "Findings")
        .map(|(_, schema)| schema)
        .expect("the registry exports Findings")
}

fn validator() -> Validator {
    jsonschema::validator_for(&exported_findings_schema()).expect("the exported schema compiles")
}

fn valid_run() -> serde_json::Value {
    json!({
        "items": [{
            "ac": "AC-281",
            "claim": "the ingest guard rejects a prose command",
            "evidence": "reproduced",
            // *(dated 2026-10-02, s8h build, package P2-rk-types: `Finding.severity` is required, ADR-0197 d6.)*
            "severity": "blocking",
            "rationale": "ADR-0096 decision 1",
            "files": ["crates/rk-types/src/artifact/findings.rs"],
            "repro": {
                "kind": "run",
                "command": "cargo test -p rk-types --test it",
                "mutation": null,
                "expected": { "kind": "exit", "exit": 0 },
                "observed": "exit 101",
                "expected_when_met": "exit 0"
            }
        }]
    })
}

fn spoil(f: impl FnOnce(&mut serde_json::Value)) -> serde_json::Value {
    let mut v = valid_run();
    f(&mut v["items"][0]["repro"]);
    v
}

/// **AC-281 — an incomplete typed repro is rejected at ingest.**
///
/// > THEN any item lacking a complete typed `repro { kind, command, mutation, expected, observed,
/// > expected_when_met }` is rejected at ingest with the item identified, and never reaches the
/// > classifier — including an item whose `command` does not begin with an executable token, and
/// > an item with `kind = mutate` and no `mutation` (ADR-0096 decision 1: malformed, not
/// > `Unverifiable`, not a finding). *Verified by:* `serde` round-trip + `jsonschema` validation
/// > tests; five negative fixtures …
///
/// The prose-only case is *"the one a presence-only check gets wrong"* (`s7b-ii` §7b-ii.11), and
/// it is the one the JSON schema CANNOT refuse — a string is a string — so the two instruments are
/// asserted separately below and the schema's blindness to it is stated rather than hidden.
#[test]
fn ac281_findings_ingest_rejects_an_incomplete_repro() {
    // ---- the positive control: well-formed, round-trips, validates ---------------------------
    let ok = Findings::ingest(valid_run()).expect("a well-formed run item ingests");
    let back = serde_json::to_value(&ok).expect("serialises");
    assert_eq!(
        Findings::ingest(back.clone()).expect("round-trips"),
        ok,
        "serde round-trip"
    );
    assert!(
        validator().is_valid(&back),
        "the well-formed item validates against the exported schema"
    );
    assert_eq!(
        ok.items[0].evidence,
        EvidenceLabel::Reproduced,
        "the model label is carried"
    );

    // ---- the five negatives ---------------------------------------------------------------------
    let negatives: Vec<(&str, serde_json::Value, bool)> = vec![
        // (name, fixture, does the JSON SCHEMA refuse it?)
        (
            "missing command",
            spoil(|r| {
                r.as_object_mut().unwrap().remove("command");
            }),
            true,
        ),
        (
            "missing observed",
            spoil(|r| {
                r.as_object_mut().unwrap().remove("observed");
            }),
            true,
        ),
        (
            "missing expected_when_met",
            spoil(|r| {
                r.as_object_mut().unwrap().remove("expected_when_met");
            }),
            true,
        ),
        (
            "prose-only command",
            spoil(|r| r["command"] = json!("run the tests and see")),
            false,
        ),
        (
            "kind = mutate without mutation",
            spoil(|r| r["kind"] = json!("mutate")),
            false,
        ),
    ];
    for (name, fixture, schema_refuses) in negatives {
        // RED (observed 2026-09-17) for the prose case, via a temporary mutant of the ingest
        // guard that accepted every command: "prose-only command: was ACCEPTED at ingest".
        let got = Findings::ingest(fixture.clone());
        assert!(got.is_err(), "{name}: was ACCEPTED at ingest — {got:?}");
        // the item is IDENTIFIED where the guard, not serde, did the refusing
        match (&got, name) {
            (Err(IngestError::CommandNotExecutable { item, .. }), "prose-only command")
            | (
                Err(IngestError::MutateWithoutMutation { item }),
                "kind = mutate without mutation",
            ) => {
                assert_eq!(
                    item, "AC-281",
                    "{name}: the rejected item is identified by its AC id"
                );
            }
            (Err(IngestError::Malformed { .. }), _) => {}
            other => panic!("{name}: unexpected ingest result {other:?}"),
        }
        assert_eq!(
            validator().is_valid(&fixture),
            !schema_refuses,
            "{name}: the exported JSON schema {} refuse it — the two semantic cases are the guard's alone, \
             which is why AC-281 has both instruments",
            if schema_refuses { "must" } else { "cannot" }
        );
    }
}

/// **AC-293 — the review outcome carries the reviewed tree's digest, non-optionally.**
///
/// > THEN the outcome record carries the tree digest of the SHA that was reviewed, and a record
/// > without that digest fails to construct — no default, no empty string, no `None`.
/// > *Verified by:* a type-level test that the digest field is non-optional; a serialisation
/// > round-trip test that captures `basis`/`sha`; a negative fixture with the digest removed that
/// > must fail to construct.
///
/// RED (observed 2026-09-17) against a skeleton whose `reviewed_tree` was `Option<TreeId>`:
/// the digest-removed fixture constructed with `None`.
#[test]
fn ac293_review_outcome_carries_the_reviewed_tree_digest() {
    let outcome = ReviewOutcome {
        reviewed_tree: TreeId::from_hex(TREE).expect("40 hex"),
        sha: "0123456789abcdef0123456789abcdef01234567".to_owned(),
        basis: ReviewBasis::WorkingState,
        blocks: 1,
        sink_records: 6,
    };
    // the TYPE-LEVEL half: the member is a TreeId, not an Option — this line does not compile
    // against an optional member without an `unwrap`, and there is none here
    let digest: &TreeId = &outcome.reviewed_tree;
    assert_eq!(digest.as_str(), TREE);

    // round-trip captures basis and sha
    let v = serde_json::to_value(&outcome).expect("serialises");
    assert_eq!(v["reviewed_tree"], TREE);
    assert_eq!(v["basis"], "working_state");
    assert_eq!(v["sha"], "0123456789abcdef0123456789abcdef01234567");
    let back: ReviewOutcome = serde_json::from_value(v.clone()).expect("round-trips");
    assert_eq!(back, outcome);

    // the negative fixtures: removed, empty, null, and not-a-tree-id — every one fails to construct
    let mut removed = v.clone();
    removed.as_object_mut().unwrap().remove("reviewed_tree");
    assert!(
        serde_json::from_value::<ReviewOutcome>(removed).is_err(),
        "a record without the digest CONSTRUCTED"
    );
    for (name, bad) in [
        ("empty string", json!("")),
        ("null", json!(null)),
        ("not hex", json!("zz")),
    ] {
        let mut spoiled = v.clone();
        spoiled["reviewed_tree"] = bad;
        assert!(
            serde_json::from_value::<ReviewOutcome>(spoiled).is_err(),
            "a record whose digest is {name} CONSTRUCTED"
        );
    }
    assert!(TreeId::from_hex("").is_err() && TreeId::from_hex(&"A".repeat(40)).is_err());
}

/// **AC-319 — a quarantine without `until` is unconstructible.**
///
/// > GIVEN a quarantine being constructed / WHEN it carries no `until` / THEN construction is
/// > refused. *Verified by:* a type test that `until` is non-optional; a negative fixture that
/// > must fail to construct.
///
/// RED (observed 2026-09-17) against a skeleton whose `until` was `Option<Timestamp>`: the
/// fixture with no `until` constructed.
#[test]
fn ac319_quarantine_without_until_is_unconstructible() {
    let q = Quarantine {
        reason: Text::new("disputed: the fixture's baseline was already red"),
        by: "ryan".to_owned(),
        until: Timestamp::from_unix_seconds(1_700_003_600),
    };
    // the TYPE test: `until` is a Timestamp, not an Option
    let until: Timestamp = q.until;
    assert_eq!(until.unix_seconds(), 1_700_003_600);

    let v = serde_json::to_value(&q).expect("serialises");
    assert_eq!(
        serde_json::from_value::<Quarantine>(v.clone()).expect("round-trips"),
        q
    );
    let mut without = v;
    without.as_object_mut().unwrap().remove("until");
    assert!(
        serde_json::from_value::<Quarantine>(without).is_err(),
        "a quarantine with no `until` CONSTRUCTED"
    );
    assert!(
        serde_json::from_value::<Quarantine>(json!({ "reason": "r", "by": "b", "until": null }))
            .is_err(),
        "a quarantine with `until: null` CONSTRUCTED"
    );
}

/// **AC-280 — no decision field on the review card's output schema.**
///
/// > THEN a schema whose top-level `Findings` object declares any field named in the
/// > decision-field denylist (`pass`, `verdict`, `approve`, `approved`, `approval`, `ok`, `status`,
/// > `decision`) is refused as a configuration error, before any dispatch. *Verified by:* a
/// > `rk-types` schema-shape unit test over the `Findings` derivation; a `redkiln doctor`
/// > rule-table row on card output schemas; fixture cards carrying a `pass` field and an `approve`
/// > field, each of which must fail.
///
/// RED (observed 2026-09-17) against a denylist check that returned nothing over a schema
/// declaring `pass`.
#[test]
fn ac280_decision_fields_are_refused_at_schema_load() {
    assert_eq!(
        DECISION_FIELD_DENYLIST.len(),
        8,
        "the register enumerates eight spellings"
    );
    // the shipped derivation declares none of them, at the top level or on an item
    let schema = exported_findings_schema();
    assert_eq!(
        decision_fields_declared(&schema),
        Vec::<String>::new(),
        "the derived Findings schema declares a decision field"
    );
    // a schema that does is NAMED — the positive control for the check
    let mut with_pass = schema.clone();
    with_pass["properties"]["pass"] = json!({ "type": "boolean" });
    assert_eq!(
        decision_fields_declared(&with_pass),
        vec!["pass".to_owned()]
    );
    let mut with_approve_on_item = schema;
    let item_props = with_approve_on_item
        .pointer_mut("/properties/items/items/properties")
        .expect("the closed-output transform inlines the Finding item at /properties/items/items");
    item_props["approve"] = json!({ "type": "boolean" });
    assert_eq!(
        decision_fields_declared(&with_approve_on_item),
        vec!["approve".to_owned()],
        "a decision field on the item object is refused too"
    );
    // fixture cards carrying `pass` and `approve` fail at ingest (deny_unknown_fields, both levels)
    let mut top_pass = valid_run();
    top_pass["pass"] = json!(true);
    assert!(
        Findings::ingest(top_pass).is_err(),
        "a card with a top-level `pass` INGESTED"
    );
    let mut item_approve = valid_run();
    item_approve["items"][0]["approve"] = json!(true);
    assert!(
        Findings::ingest(item_approve).is_err(),
        "a card with an item-level `approve` INGESTED"
    );
}
