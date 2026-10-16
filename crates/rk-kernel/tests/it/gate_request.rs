//! **D21-48 X16's type-neutral gate request** — `s8i` §8i.8 *"The gate path. Taken, D21-48 X16"*,
//! ADR-0199 decision 3 (proposed). Built on the human-gate route only (G = 3).
//!
//! | test | what it measures | its mutant |
//! | --- | --- | --- |
//! | [`automated_verdict_refused_opens_a_gate_through_the_existing_reach_path`] | the request opens its gate through `gate::reach`: the token is `GateRecord::derive_reach`'s over the context's tree and deadline, the reason is the gate's kind, a re-reach mints reach 2, and the fold opens the row as any gate's | a fixed token instead of one derived through `GateRecord::derive_reach` |
//! | [`the_reason_is_closed_and_an_unknown_reason_is_refused_by_name`] | the reason has exactly one value, spelled `automated-verdict-refused`; any other string is refused, and the refusal names it | an `Other(String)` arm carrying an unknown reason as free text |
//! | [`the_attachment_is_carried_byte_for_byte_and_never_parsed`] | an attachment that is neither UTF-8 nor JSON is carried on the opened gate's row byte for byte, beside its digest, and read back unchanged | decode the attachment as UTF-8 JSON before carrying it |
//! | [`the_request_row_is_the_producer_row_plus_the_two_attachment_members`] | at reach 1 and 2, the request's `gate_opened` payload less `attachment` and `attachment_digest` equals the producer's `gate_opened_event` payload, and the kinds match | an extra member in either row |
//!
//! The kernel never types the attachment: what the bytes are is the caller's (`rk-cli`'s
//! `review_input`) and the render side's knowledge, never this crate's.

use rk_kernel::workflow::fsm::{FsmContext, GateRecord, fold};
use rk_kernel::workflow::gate::{
    self, GateAttachment, GateAttachmentError, GateReason, GateRequest,
};
use rk_types::digest::sha256;
use rk_types::event_kind::EventKind;
use rk_types::time::Timestamp;

const ITEM: &str = "INC-1";
const STAGE: &str = "review";
const TREE_A: &str = "4b825dc642cb6eb9a060e54bf8d69288fbee4904";
const TREE_B: &str = "9daeafb9864cf43055ae93beb0afd6c7d144bfa4";
const DEADLINE: i64 = 1_700_003_600;
/// The reason's one spelling, as the record writes it.
const REASON: &str = "automated-verdict-refused";

/// Bytes that are neither UTF-8 (a lone `0xff`, an overlong lead, a truncated sequence) nor JSON.
const NOT_UTF8_NOT_JSON: &[u8] = &[
    0xff, 0xfe, 0x00, 0x7b, 0x22, 0x80, 0xc3, 0x28, 0x0a, 0x0d, 0x00, 0xf8, 0x88, 0x80, 0x80, 0xe2,
];

const fn ctx(tree_hash: &str) -> FsmContext<'_> {
    FsmContext::supplied(
        ITEM,
        tree_hash,
        Timestamp::from_unix_seconds(DEADLINE),
        None,
    )
}

fn lower_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn request(bytes: &[u8]) -> GateRequest {
    GateRequest::new(
        GateReason::AutomatedVerdictRefused,
        GateAttachment::new(bytes.to_vec()),
    )
}

#[test]
fn automated_verdict_refused_opens_a_gate_through_the_existing_reach_path() {
    let empty = fold(ITEM, STAGE, &[]).expect("an empty journal folds");
    let first = request(b"summary")
        .open(&empty, &ctx(TREE_A))
        .expect("the request opens its gate");

    // The existing reach path, and the one derivation every gate's token comes from.
    let via_reach = gate::reach(&empty, REASON, &ctx(TREE_A)).expect("gate::reach mints");
    let derived =
        GateRecord::derive_reach(REASON, TREE_A, Timestamp::from_unix_seconds(DEADLINE), 1)
            .expect("derive_reach mints");
    assert_eq!(
        first.record(),
        &via_reach,
        "the request opens through gate::reach"
    );
    assert_eq!(
        first.record(),
        &derived,
        "the token is derive_reach's, at reach 1"
    );
    assert_eq!(
        first.record().kind(),
        REASON,
        "the reason is the gate's kind"
    );
    assert_eq!(first.record().tree_hash(), TREE_A);

    // The row is the runtime's producer shape, with the attachment's digest beside the kind.
    let event = first.event();
    assert_eq!(event.kind, EventKind::GateOpened);
    let p = &event.payload;
    assert_eq!(
        p["token"],
        derived.token().as_str(),
        "the row's token is the derived one"
    );
    assert_eq!(p["kind"], REASON);
    assert_eq!(p["tree_hash"], TREE_A);
    assert_eq!(p["deadline"], DEADLINE);
    assert_eq!(p["reach"], 1);
    assert_eq!(p["attachment_digest"], sha256(b"summary").as_str());

    // The fold opens it as it opens any gate: the record it re-derives is the request's.
    let state = fold(ITEM, STAGE, std::slice::from_ref(event)).expect("the row folds");
    assert_eq!(state.gate_record(derived.token()), Some(&derived));
    assert_eq!(state.open_gates().count(), 1);
    assert_eq!(state.reaches(REASON), 1);

    // A re-reach mints a fresh token at reach 2, as gate::reach does for every gate step.
    let second = request(b"summary")
        .open(&state, &ctx(TREE_A))
        .expect("the second request opens");
    let derived_2 =
        GateRecord::derive_reach(REASON, TREE_A, Timestamp::from_unix_seconds(DEADLINE), 2)
            .expect("derive_reach mints");
    assert_eq!(
        second.record(),
        &derived_2,
        "reach 2's token is derive_reach's"
    );
    assert_eq!(
        second.record(),
        &gate::reach(&state, REASON, &ctx(TREE_A)).expect("gate::reach mints"),
    );
    assert_ne!(
        second.record().token(),
        first.record().token(),
        "a re-reach is a fresh token"
    );

    // The token follows the context's tree: no token is fixed.
    let over_b = request(b"summary")
        .open(&empty, &ctx(TREE_B))
        .expect("opens over tree B");
    assert_ne!(over_b.record().token(), first.record().token());
    assert_eq!(over_b.record().tree_hash(), TREE_B);
}

#[test]
fn the_reason_is_closed_and_an_unknown_reason_is_refused_by_name() {
    assert_eq!(
        REASON.parse::<GateReason>(),
        Ok(GateReason::AutomatedVerdictRefused)
    );
    assert_eq!(GateReason::AutomatedVerdictRefused.as_str(), REASON);
    assert_eq!(GateReason::AutomatedVerdictRefused.to_string(), REASON);

    // Closed: an exhaustive match with no wildcard arm compiles only while the set has one value.
    let closed = |reason: GateReason| match reason {
        GateReason::AutomatedVerdictRefused => REASON,
    };
    assert_eq!(closed(GateReason::AutomatedVerdictRefused), REASON);

    for unknown in [
        "reviewer-unavailable",
        "Automated-Verdict-Refused",
        "automated_verdict_refused",
        " automated-verdict-refused",
        "automated-verdict-refused ",
        "automated-verdict-refused\n",
        "",
    ] {
        let refused = unknown
            .parse::<GateReason>()
            .expect_err("an unknown reason is refused, never carried");
        assert_eq!(
            refused.reason(),
            unknown,
            "the refusal carries the reason it refused"
        );
        let shown = refused.to_string();
        assert!(
            shown.contains(&format!("{unknown:?}")),
            "the refusal names the reason: {shown}"
        );
        assert!(
            shown.contains(REASON),
            "the refusal names the closed set: {shown}"
        );
    }
}

#[test]
fn the_attachment_is_carried_byte_for_byte_and_never_parsed() {
    // PREMISE: the fixture is neither UTF-8 nor JSON, so a parse on the way would fail on it.
    assert!(
        String::from_utf8(NOT_UTF8_NOT_JSON.to_vec()).is_err(),
        "PREMISE: not UTF-8"
    );
    assert!(
        serde_json::from_slice::<serde_json::Value>(NOT_UTF8_NOT_JSON).is_err(),
        "PREMISE: not JSON"
    );

    let attachment = GateAttachment::new(NOT_UTF8_NOT_JSON.to_vec());
    assert_eq!(attachment.bytes(), NOT_UTF8_NOT_JSON);
    assert_eq!(attachment.digest(), &sha256(NOT_UTF8_NOT_JSON));

    let empty = fold(ITEM, STAGE, &[]).expect("an empty journal folds");
    let opened = GateRequest::new(GateReason::AutomatedVerdictRefused, attachment)
        .open(&empty, &ctx(TREE_A))
        .expect("bytes that are neither UTF-8 nor JSON are carried, not parsed");

    // On the row: every byte, in order, as lower-case hex, and the digest of those bytes.
    let p = &opened.event().payload;
    assert_eq!(p["attachment"], lower_hex(NOT_UTF8_NOT_JSON));
    assert_eq!(p["attachment_digest"], sha256(NOT_UTF8_NOT_JSON).as_str());

    // Read back from the row: the same bytes and the same digest.
    let back = GateAttachment::from_gate_opened(opened.event()).expect("the row carries it");
    assert_eq!(back.bytes(), NOT_UTF8_NOT_JSON);
    assert_eq!(back.digest(), &sha256(NOT_UTF8_NOT_JSON));

    // The row still folds as a gate.
    let state = fold(ITEM, STAGE, std::slice::from_ref(opened.event())).expect("the row folds");
    assert_eq!(
        state.gate_record(opened.record().token()),
        Some(opened.record())
    );

    // A row whose bytes do not match their digest is refused, never re-digested to fit.
    let mut tampered = opened.event().clone();
    let mut flipped = NOT_UTF8_NOT_JSON.to_vec();
    if let Some(first) = flipped.first_mut() {
        *first ^= 0x01;
    }
    tampered.payload["attachment"] = serde_json::Value::from(lower_hex(&flipped));
    assert!(matches!(
        GateAttachment::from_gate_opened(&tampered),
        Err(GateAttachmentError::DigestMismatch { .. })
    ));
    // A gate row with no attachment carries none.
    let plain = rk_kernel::runtime::gate_opened_event(opened.record());
    assert_eq!(
        GateAttachment::from_gate_opened(&plain),
        Err(GateAttachmentError::Absent)
    );
}

/// The request's row is the runtime's producer row plus the attachment's two members, and nothing
/// else: `GateRequest::open` spells the producer's members itself, so this pins the two spellings
/// together at every reach.
#[test]
fn the_request_row_is_the_producer_row_plus_the_two_attachment_members() {
    let empty = fold(ITEM, STAGE, &[]).expect("an empty journal folds");
    let first = request(NOT_UTF8_NOT_JSON)
        .open(&empty, &ctx(TREE_A))
        .expect("the request opens its gate");
    let state = fold(ITEM, STAGE, std::slice::from_ref(first.event())).expect("the row folds");
    let second = request(b"summary")
        .open(&state, &ctx(TREE_B))
        .expect("the second request opens");

    for opened in [&first, &second] {
        let producer = rk_kernel::runtime::gate_opened_event(opened.record());
        assert_eq!(opened.event().kind, producer.kind);

        let mut members = opened
            .event()
            .payload
            .as_object()
            .expect("the request's row is a JSON object")
            .clone();
        assert!(
            members.remove(gate::ATTACHMENT_MEMBER).is_some(),
            "the row carries `{}`",
            gate::ATTACHMENT_MEMBER
        );
        assert!(
            members.remove(gate::ATTACHMENT_DIGEST_MEMBER).is_some(),
            "the row carries `{}`",
            gate::ATTACHMENT_DIGEST_MEMBER
        );
        assert_eq!(
            serde_json::Value::Object(members),
            producer.payload,
            "without its attachment, the request's row is gate_opened_event's (reach {})",
            opened.record().reach()
        );
    }
}
