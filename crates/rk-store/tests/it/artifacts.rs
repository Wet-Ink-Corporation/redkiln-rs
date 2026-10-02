//! **`rk-store-artifacts`** — the review gate's two `rk-store` criteria (Step 7b-ii, `s7b-ii`
//! §7b-ii.6–.7), filtered here by Checkpoint 7b-ii's third loop.
//!
//! | test | criterion | what it measures | what it does NOT |
//! | --- | --- | --- | --- |
//! | [`ac305_finding_identity_is_stable`] | **AC-305** | identity holds under a rationale rewording AND a file reorder; breaks under a changed command AND a changed patch; a run and a mutant with one command differ; the tuple has four components | the two-review integration (`rk-kernel`'s AC-530 test) |
//! | [`ac310_sink_accounting`] | **AC-310** (the accounting) | `findings_in == blocks + sink_records`, failing with BOTH sides printed; `live == created − retired − dismissed` over a fixture run carrying all three; a leave with no arrival is refused; the F31 seven land in the sink | the `doctor` row (7d); the durable typed record (OWED, `artifacts.rs`'s header) |
//!
//! Red first: AC-305 against an identity that hashed the rationale (stable-under-rewording failed)
//! and, separately, one that skipped the patch (unstable-under-patch-change failed); AC-310 against
//! an accounting that dropped one sink record — the identity failed naming both sides.

use rk_store::artifacts::finding_identity::{FindingIdentityComponents, finding_identity};
use rk_store::artifacts::sink::{
    IntakeMismatch, SinkAccountingError, intake_identity, sink_accounting,
};
use rk_types::artifact::findings::{EvidenceLabel, Finding};
use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::repro::{Expected, Locator, Mutation, Repro, ReproKind};
use rk_types::text::Text;

fn run_finding(ac: &str, command: &str) -> Finding {
    Finding {
        ac: ac.to_owned(),
        claim: Text::new("the claim"),
        evidence: EvidenceLabel::Reproduced,
        // *(dated 2026-10-02, s8h build, package P2-rk-types: `Finding.severity` is required, ADR-0197 d6.)*
        severity: rk_types::artifact::findings::FindingSeverity::Blocking,
        rationale: Text::new("the rationale, first wording"),
        repro: Repro {
            kind: ReproKind::Run,
            command: command.to_owned(),
            expected: Expected::Exit { exit: 0 },
            observed: "exit 101".to_owned(),
            expected_when_met: "exit 0".to_owned(),
            mutation: None,
        },
        files: vec!["src/a.rs".to_owned(), "src/b.rs".to_owned()],
    }
}

fn mutate_finding(ac: &str, command: &str, patch: &str, suite: &str) -> Finding {
    let mut f = run_finding(ac, command);
    f.evidence = EvidenceLabel::MutationSurvived;
    f.repro.kind = ReproKind::Mutate;
    f.repro.expected = Expected::Assertion {
        assertion: Locator {
            file: "tests/it/x.rs".to_owned(),
            test: "pins_it".to_owned(),
        },
    };
    f.repro.mutation = Some(Mutation {
        patch: patch.to_owned(),
        suite: suite.to_owned(),
    });
    f
}

/// **AC-305 — identity stable under rewording and reorder, unstable under a changed command or patch.**
///
/// One direction alone pins nothing (`s7b-ii` §7b-ii.11): an identity that hashed everything would
/// pass the "unstable" half and an identity that hashed nothing would pass the "stable" half.
#[test]
fn ac305_finding_identity_is_stable() {
    assert_eq!(
        FindingIdentityComponents::COMPONENT_COUNT,
        4,
        "the register's tuple: ac, kind, command digest, mutation digest"
    );
    let base = run_finding("AC-042", "cargo test -p rk-kernel -- ac042");
    let id = finding_identity(&base);

    // ---- STABLE under rationale rewording and file-order change ------------------------------
    let mut reworded = base.clone();
    reworded.rationale = Text::new("the rationale, restated at length in different words");
    reworded.claim = Text::new("a different claim sentence too");
    assert_eq!(
        finding_identity(&reworded),
        id,
        "identity moved under a rationale/claim rewording"
    );
    let mut reordered = base.clone();
    reordered.files.reverse();
    assert_eq!(
        finding_identity(&reordered),
        id,
        "identity moved under a file-order change"
    );
    let mut relabelled = base.clone();
    relabelled.evidence = EvidenceLabel::Advisory;
    assert_eq!(
        finding_identity(&relabelled),
        id,
        "the model's label is not part of the identity"
    );

    // ---- UNSTABLE under a changed repro command --------------------------------------------------
    let mut recommanded = base.clone();
    recommanded.repro.command = "cargo test -p rk-kernel -- ac042 --exact".to_owned();
    assert_ne!(
        finding_identity(&recommanded),
        id,
        "a changed command must change the identity"
    );
    let mut re_ac = base.clone();
    re_ac.ac = "AC-043".to_owned();
    assert_ne!(
        finding_identity(&re_ac),
        id,
        "a different AC is a different finding"
    );

    // ---- UNSTABLE under a changed mutation patch, and the mutate/run distinction -----------------
    let m = mutate_finding(
        "AC-042",
        "cargo test -p rk-kernel",
        "--- a\n+++ b\n-fix\n",
        "cargo test -p rk-kernel",
    );
    let mid = finding_identity(&m);
    assert_ne!(mid, id, "a mutant and a run with one AC are two findings");
    let mut repatched = m.clone();
    repatched.repro.mutation = Some(Mutation {
        patch: "--- a\n+++ b\n-other fix\n".to_owned(),
        suite: "cargo test -p rk-kernel".to_owned(),
    });
    assert_ne!(
        finding_identity(&repatched),
        mid,
        "a changed patch must change the identity"
    );
    let mut resuited = m.clone();
    resuited.repro.mutation = Some(Mutation {
        patch: "--- a\n+++ b\n-fix\n".to_owned(),
        suite: "cargo test -p rk-kernel --test it".to_owned(),
    });
    assert_ne!(
        finding_identity(&resuited),
        mid,
        "a changed suite must change the identity"
    );
    let mut m_reworded = m.clone();
    m_reworded.rationale = Text::new("reworded");
    assert_eq!(
        finding_identity(&m_reworded),
        mid,
        "…while a rewording leaves a mutant's identity alone"
    );

    // ---- the run's fourth slot is EMPTY, so the tuple never changes shape ---------------------
    let comps = FindingIdentityComponents::of(&base);
    assert!(comps.mutation_digest.is_none());
    assert!(FindingIdentityComponents::of(&m).mutation_digest.is_some());
    assert_eq!(comps.digest(), id, "the components digest to the identity");
}

fn row(kind: EventKind, id: &Digest, extra: serde_json::Value) -> Event {
    let mut payload = serde_json::json!({ "finding_id": id.as_str() });
    if let (Some(obj), Some(e)) = (payload.as_object_mut(), extra.as_object()) {
        for (k, v) in e {
            obj.insert(k.clone(), v.clone());
        }
    }
    Event::new(kind, payload)
}

/// **AC-310 — the sink's accounting: nothing dropped on intake, nothing deleted over its lifetime.**
///
/// > *Verified by:* a replay assertion `findings_in == blocks + sink_records` and, over the sink's
/// > lifetime, `live_sink == created − retired − dismissed` (AC-530's accounting) …; the seven F31
/// > `narrowed-pin` findings replayed as the seed fixture, which must all land in the sink.
#[test]
fn ac310_sink_accounting() {
    // ---- intake: the identity, both sides printed on failure ------------------------------------
    intake_identity(9, 2, 7).expect("9 in, 2 blocked, 7 sunk");
    let err = intake_identity(9, 2, 6).expect_err("one sink record dropped is a failure");
    assert_eq!(
        err,
        IntakeMismatch {
            findings_in: 9,
            blocks: 2,
            sink_records: 6
        },
        "the failure carries all three counts"
    );
    let text = err.to_string();
    assert!(
        text.contains("findings_in = 9") && text.contains("2 + 6 = 8") && text.contains("DROPPED"),
        "both sides are printed: {text}"
    );

    // ---- lifetime: created, retired and dismissed in one run ------------------------------------
    let ids: Vec<Digest> = (0..7u8)
        .map(|i| {
            finding_identity(&run_finding(
                &format!("AC-00{i}"),
                &format!("cargo test -- ac00{i}"),
            ))
        })
        .collect();
    let mut journal: Vec<Event> = ids
        .iter()
        .map(|id| row(EventKind::AdvisoryCreated, id, serde_json::json!({})))
        .collect();
    let count = sink_accounting(&journal).expect("folds");
    assert_eq!(
        count,
        rk_store::artifacts::sink::SinkCount {
            created: 7,
            retired: 0,
            dismissed: 0,
            live: 7
        }
    );
    journal.push(row(
        EventKind::AdvisoryRetired,
        &ids[0],
        serde_json::json!({ "reason": "not_re_raised", "review_id": "review-2" }),
    ));
    journal.push(row(
        EventKind::AdvisoryRetired,
        &ids[1],
        serde_json::json!({ "reason": "not_re_raised", "review_id": "review-2" }),
    ));
    journal.push(row(
        EventKind::AdvisoryDismissed,
        &ids[2],
        serde_json::json!({ "by": "ryan" }),
    ));
    let count = sink_accounting(&journal).expect("folds");
    assert_eq!(count.created, 7);
    assert_eq!(count.retired, 2);
    assert_eq!(count.dismissed, 1);
    assert_eq!(
        count.live,
        7 - 2 - 1,
        "live == created − retired − dismissed"
    );

    // a record may leave only by a recorded disposition: a leave with no arrival is refused
    let stranger = finding_identity(&run_finding("AC-999", "cargo test -- ac999"));
    let mut bad = journal.clone();
    bad.push(row(
        EventKind::AdvisoryRetired,
        &stranger,
        serde_json::json!({ "reason": "not_re_raised", "review_id": "review-3" }),
    ));
    assert!(
        matches!(
            sink_accounting(&bad),
            Err(SinkAccountingError::LeftWithoutArriving { .. })
        ),
        "a retirement of a record never created is the deletion the invariant forbids"
    );
    let mut twice = journal.clone();
    twice.push(row(
        EventKind::AdvisoryDismissed,
        &ids[0],
        serde_json::json!({ "by": "ryan" }),
    ));
    assert!(matches!(
        sink_accounting(&twice),
        Err(SinkAccountingError::LeftTwice { .. })
    ));

    // ---- the F31 seed: seven narrowed-pin findings under the model-label policy all land -----
    let seed: serde_json::Value = serde_json::from_str(include_str!(
        "../../../rk-kernel/tests/fixtures/findings/ac310_f31_narrowed_pin_seed.json"
    ))
    .expect("the seed parses");
    let seven = seed["seed"].as_array().expect("seed rows");
    assert_eq!(seven.len(), 7, "PREMISE: F31's seven");
    for s in seven {
        assert_eq!(
            s["finding"]["evidence"], "narrowed-pin",
            "the model label is narrowed-pin"
        );
        assert_eq!(s["adjudication"]["verdict"], "CONFIRMED");
    }
    // under the severity decision F31 recorded, narrowed-pin ADVISES: 0 blocks, 7 sink records
    assert!(
        seed["severity_decision"]["decision"]
            .as_str()
            .unwrap()
            .starts_with("narrowed-pin ADVISES")
    );
    intake_identity(7, 0, 7).expect("the seven land in the sink and none is dropped");
    let seeded: Vec<Event> = seven
        .iter()
        .map(|s| {
            // the seed's identity is over the model-label record's command (pre-ADR-0096 shape)
            let cmd = s["finding"]["repro"]["command"].as_str().unwrap();
            let id = rk_types::digest::sha256_lp(&[s["ac"].as_str().unwrap().as_bytes(), b"run", rk_types::digest::sha256(cmd.as_bytes()).as_str().as_bytes(), &[]]);
            row(EventKind::AdvisoryCreated, &id, serde_json::json!({ "class": "narrowed-pin", "reason": "narrowed_pin_advises_at_weight", "policy": "model-label (E-0074-2)" }))
        })
        .collect();
    assert_eq!(
        sink_accounting(&seeded).expect("folds").live,
        7,
        "all seven are live in the sink"
    );
}
