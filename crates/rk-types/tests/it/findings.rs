//! `Finding.severity` — `s8h` §8h.4 (D21-47 cross #4: *"ADD `severity` to rk-types `Finding`, typed
//! by s8b's 11c vocabulary, with tests"*). *(Added 2026-10-02, `s8h` build, package P2-rk-types;
//! Record A = ADR-0197 decision 6, proposed.)*
//!
//! The vocabulary is read from the 11c record, located by its RECORDED id (ADR-0168, `s8h` §8h.0
//! item 12), never from this file: its decision 1 reads *"The severity vocabulary is `blocking |
//! advisory`."* A record that moves the vocabulary moves these tests with it.

use rk_types::artifact::findings::{Findings, IngestError};
use rk_types::registry;
use serde_json::{Value, json};
use std::path::PathBuf;

/// The 11c record's recorded id (`s8h` §8h.0 item 12; `spikes/DECISIONS-2026-09-21.md` D21-65).
const ELEVEN_C_RECORD: &str = "0168";

/// The severity spellings, read from the record's decision-1 sentence.
fn vocabulary() -> Vec<String> {
    let adr = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../references/adr");
    let mut hits = std::fs::read_dir(&adr)
        .expect("references/adr is readable")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(&format!("{ELEVEN_C_RECORD}-")))
        })
        .collect::<Vec<_>>();
    assert_eq!(
        hits.len(),
        1,
        "exactly one record carries id {ELEVEN_C_RECORD}: {hits:?}"
    );
    let text = std::fs::read_to_string(hits.remove(0)).expect("the 11c record reads");
    let marker = "The severity vocabulary is `";
    let at = text
        .find(marker)
        .expect("the 11c record states its vocabulary in decision 1's sentence");
    let rest = &text[at + marker.len()..];
    let end = rest.find('`').expect("the vocabulary closes its backtick");
    let words: Vec<String> = rest[..end]
        .split('|')
        .map(|w| w.trim().to_owned())
        .collect();
    assert!(
        words.len() >= 2 && words.iter().all(|w| !w.is_empty()),
        "premise: the record names a vocabulary of at least two spellings: {words:?}"
    );
    words
}

/// A well-formed finding carrying `severity`, or none when `severity` is `None`.
fn findings_with(severity: Option<&str>) -> Value {
    let mut item = json!({
        "ac": "AC-281",
        "claim": "the review gate refuses a prose command",
        "evidence": "reproduced",
        "rationale": "ADR-0096 decision 1",
        "repro": {
            "kind": "run",
            "command": "cargo test -p rk-types --test it",
            "mutation": null,
            "expected": { "kind": "exit", "exit": 0 },
            "observed": "exit 101",
            "expected_when_met": "exit 0"
        }
    });
    if let Some(s) = severity {
        item["severity"] = json!(s);
    }
    json!({ "items": [item] })
}

/// The `Finding` object inside the exported `Findings` schema.
fn exported_finding() -> Value {
    let (_, schema) = registry::export_all()
        .into_iter()
        .find(|(n, _)| *n == "Findings")
        .expect("Findings is exported");
    schema
        .pointer("/properties/items/items")
        .cloned()
        .expect("the exported Findings schema inlines its item object")
}

/// **`severity` is a REQUIRED member typed by the 11c vocabulary.** Every spelling the record
/// names is accepted; a finding without the member is refused; and the exported schema lists it as
/// required with exactly the record's spellings. Red before the member exists: a finding with no
/// `severity` is accepted, and one carrying it is refused as an unknown field.
#[test]
fn severity_is_a_required_member_typed_by_the_11c_vocabulary() {
    let vocab = vocabulary();

    for s in &vocab {
        let got = Findings::ingest(findings_with(Some(s)));
        assert!(
            got.is_ok(),
            "the record's spelling `{s}` must be accepted on a finding, got {got:?}"
        );
    }

    let missing = Findings::ingest(findings_with(None));
    match &missing {
        Err(IngestError::Malformed { detail }) => assert!(
            detail.contains("severity"),
            "the refusal names the missing member: {detail}"
        ),
        other => panic!("a finding without `severity` must be refused at ingest, got {other:?}"),
    }

    let finding = exported_finding();
    let required: Vec<&str> = finding["required"]
        .as_array()
        .expect("the finding object lists its required members")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        required.contains(&"severity"),
        "`severity` is required in the exported schema, never optional: {required:?}"
    );
    let spelled: Vec<&str> = finding
        .pointer("/properties/severity/enum")
        .and_then(Value::as_array)
        .expect("`severity` is a closed enum in the exported schema")
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let want: Vec<&str> = vocab.iter().map(String::as_str).collect();
    assert_eq!(
        spelled, want,
        "the exported spellings are the 11c record's, in its order"
    );
}

/// **A spelling outside the record's set fails to deserialise.** The control is the record's own
/// first spelling on the same finding, accepted — so the refusal below is the vocabulary's, and
/// not `deny_unknown_fields` refusing the member itself (which is how this test would pass, for the
/// wrong reason, before the member existed).
#[test]
fn a_severity_outside_the_11c_vocabulary_is_refused() {
    let vocab = vocabulary();
    let control = Findings::ingest(findings_with(Some(&vocab[0])));
    assert!(
        control.is_ok(),
        "control: the record's spelling `{}` is accepted, got {control:?}",
        vocab[0]
    );

    for bad in ["critical", "high", "Blocking", "BLOCKING", "minor", ""] {
        assert!(
            !vocab.iter().any(|v| v == bad),
            "premise: `{bad}` is outside the record's vocabulary"
        );
        match Findings::ingest(findings_with(Some(bad))) {
            Err(IngestError::Malformed { detail }) => assert!(
                detail.contains("unknown variant"),
                "`{bad}` is refused by the closed vocabulary, not by another rule: {detail}"
            ),
            other => panic!("severity `{bad}` must be refused at ingest, got {other:?}"),
        }
    }
}
