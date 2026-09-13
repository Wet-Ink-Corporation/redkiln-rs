//! `repro_rejects_prose_only` — AC-281, on ADR-0096 decision 1's typed repro.
//!
//! The claim under test is a **rejection**, not a classification. A finding whose `command` does
//! not begin with an executable token, or whose `kind = mutate` carries no `mutation`, is
//! **rejected at ingest** as malformed — it is not `Unverifiable`, it is not routed to advisory,
//! it is not a finding. The classifier never sees it.
//!
//! AC-281 names five negative fixtures. Two of them — the prose command and the mutate-without-
//! mutation — are this test's discriminators, because they are the two that serde alone cannot
//! catch. The other three (missing `command`, `observed`, `expected_when_met`) go green off
//! `#[derive(Deserialize)]` and are asserted here for completeness, **not** counted as evidence
//! that the ingest guard discriminates.

use rk_types::artifact::findings::{Findings, IngestError};
use serde_json::json;

/// A well-formed `kind = run` item. Every negative below is this, with one field spoiled.
fn valid_run() -> serde_json::Value {
    json!({
        "items": [{
            "ac": "AC-281",
            "claim": "the ingest guard rejects a prose command",
            "rationale": "ADR-0096 decision 1",
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

#[test]
fn repro_accepts_a_well_formed_run() {
    // The positive control. Without it, a guard that rejects EVERYTHING would pass every
    // negative below and look like a working ingest check.
    let got = Findings::ingest(valid_run());
    assert!(
        got.is_ok(),
        "a well-formed run item must be accepted, got {got:?}"
    );
}

#[test]
fn repro_rejects_prose_only() {
    // The discriminator. F31 measured `repro.command` filled 147/147 — but "filled" was measured
    // against the pre-0096 shape, where a prose sentence counted as a command.
    let got = Findings::ingest(spoil(|r| r["command"] = json!("run the tests and see")));
    assert_eq!(
        got,
        Err(IngestError::CommandNotExecutable {
            item: "AC-281".to_owned(),
            command: "run the tests and see".to_owned(),
        }),
        "a prose command is rejected at ingest — never routed to advisory, never Unverifiable"
    );
}

#[test]
fn repro_rejects_mutate_without_mutation() {
    // The second half of ADR-0096 decision 1's ingest rule.
    let got = Findings::ingest(spoil(|r| r["kind"] = json!("mutate")));
    assert_eq!(
        got,
        Err(IngestError::MutateWithoutMutation {
            item: "AC-281".to_owned()
        }),
        "kind = mutate with no mutation is malformed, not a finding"
    );
}

#[test]
fn repro_rejects_missing_required_fields() {
    // Serde's job, not the guard's. Asserted so the negative set is complete; NOT evidence that
    // the ingest guard discriminates.
    for field in ["command", "observed", "expected_when_met"] {
        let v = spoil(|r| {
            r.as_object_mut().expect("repro is an object").remove(field);
        });
        assert!(
            Findings::ingest(v).is_err(),
            "a repro with no `{field}` must not deserialise into a finding"
        );
    }
}
