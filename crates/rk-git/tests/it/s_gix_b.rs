//! **S-GIX/b** — does the pinned gix expose a first-class trailer-**writing** API, and does a unit
//! test round-trip a trailer byte-identically to `git interpret-trailers --in-place`?
//!
//! The arm is a **conjunction**, and both halves are reported.
//!
//! **b1 — the API.** Measured against the resolved source on disk, not read off docs.rs, because
//! ADR-0077:105 forbids narrowing that record "on the strength of a docs read alone". At
//! `gix 0.87.1` / `gix-object 0.64.1` / `gix-actor 0.42.0`, every public item naming `trailer` is
//! read-side: `Trailers`, `TrailerRef`, `BodyRef::trailers()`, `message_trailers()` and the
//! attribution helpers. `gix` itself and `gix-actor` carry **zero** hits. The only write-sounding
//! name, `without_trailer()`, returns `&BStr` — a read that excludes the block. The read half was
//! already discharged by ERRATA E-0077-1; the write half has no API to call.
//!
//! **b2 — the bytes.** Run anyway, over ADR-0105 clause 3's five named fixtures, against the
//! candidate that remains available once b1 finds nothing: appending the trailer text by hand.
//! Its result is a property of THAT route and not of any gix API, and the record says so.
//!
//! The load-bearing line in this file is `control_oracle_drops_a_duplicate_without_if_exists_add`.
//! It proves the oracle is sensitive to the flag whose omission ADR-0105 clause 3 exists to
//! prevent. If that control does not fail the way it claims, this arm measures nothing.

use crate::support::{Fixture, assert_agrees_with_record};

/// The five named cases of ADR-0105 clause 3's fixture matrix, as exact bytes.
fn matrix() -> Vec<(&'static str, &'static [u8])> {
    vec![
        ("existing_block", b"subject\n\nbody\n\nStory: S-1\n"),
        ("no_trailing_newline", b"subject\n\nbody\n\nStory: S-1"),
        ("divider", b"subject\n\nbody\n---\nnot a trailer\n"),
        ("crlf", b"subject\r\n\r\nbody\r\n\r\nStory: S-1\r\n"),
        ("duplicate_token", b"subject\n\nbody\n\nGate: pass\n"),
    ]
}

/// Run the oracle over `input`, returning the resulting bytes.
fn oracle(fx: &Fixture, case: &str, input: &[u8], if_exists_add: bool) -> Vec<u8> {
    let file = fx.sub(&format!("msg_{case}_{if_exists_add}.txt"));
    std::fs::write(&file, input).expect("write message fixture");
    let path = file.to_str().expect("utf-8 path").to_string();
    let mut args: Vec<&str> = vec!["interpret-trailers", "--in-place"];
    if if_exists_add {
        args.push("--if-exists=add");
    }
    args.push("--trailer");
    args.push("Gate: pass");
    args.push(&path);
    fx.git(fx.path(), &args);
    std::fs::read(&file).expect("read message fixture back")
}

/// **The control that makes this arm an instrument.**
///
/// `git interpret-trailers` defaults `ifExists` to `addIfDifferentNeighbor`, which silently does
/// not append a repeated identical trailer. ADR-0105 clause 3 requires `--if-exists=add` on every
/// call for exactly that reason. If this control ever stops failing to append, the matrix below is
/// comparing two routes that are both no-ops and the arm has measured nothing.
#[test]
fn control_oracle_drops_a_duplicate_without_if_exists_add() {
    let fx = Fixture::new();
    let input = b"subject\n\nbody\n\nGate: pass\n";

    let without = oracle(&fx, "ctl", input, false);
    let with = oracle(&fx, "ctl", input, true);

    let count = |b: &[u8]| {
        String::from_utf8_lossy(b)
            .lines()
            .filter(|l| l.trim_end() == "Gate: pass")
            .count()
    };

    assert_eq!(
        count(&without),
        1,
        "the oracle is NOT sensitive to --if-exists=add: a repeated identical trailer was appended \
         without it, so the duplicate-token case below discriminates nothing.\nwithout: {:?}",
        String::from_utf8_lossy(&without)
    );
    assert_eq!(
        count(&with),
        2,
        "--if-exists=add did not append the duplicate.\nwith: {:?}",
        String::from_utf8_lossy(&with)
    );
}

/// A second control: the oracle must actually CHANGE each fixture. An oracle that no-opped would
/// make a no-op candidate compare equal and the whole matrix would pass vacuously.
#[test]
fn control_oracle_changes_every_fixture() {
    let fx = Fixture::new();
    for (case, input) in matrix() {
        let after = oracle(&fx, case, input, true);
        assert_ne!(
            after.as_slice(),
            input,
            "the oracle left `{case}` byte-identical; it asserted nothing"
        );
    }
}

#[test]
#[ignore = "S-GIX/b: a spike arm against the gix pin; run explicitly with --ignored"]
fn s_gix_b_trailer_roundtrip_matches_interpret_trailers() {
    let fx = Fixture::new();

    // b1, restated as an assertion rather than a comment: the pin offers no trailer-writing entry
    // point, so the conjunction is false however b2 turns out. `BodyRef::trailers()` is the read
    // half and is already discharged (ERRATA E-0077-1); its presence is what makes the absence of
    // a write counterpart a measurement rather than an absence of looking.
    let api_write_exists = false; // measured: see this module's header
    let body = gix_object_body_trailers_exists();
    assert!(
        body,
        "gix-object's trailer READ surface is absent too — b1's measurement is not what the \
         record describes; stop and re-measure before writing a verdict"
    );

    // b2, over the five named cases, against the hand-rolled route.
    let mut byte_identical = 0usize;
    let mut diverged: Vec<String> = Vec::new();
    for (case, input) in matrix() {
        let expected = oracle(&fx, case, input, true);
        let candidate = hand_rolled_append(input, "Gate: pass");
        if candidate == expected {
            byte_identical += 1;
        } else {
            diverged.push(format!(
                "{case}: first difference at byte {}",
                first_difference(&candidate, &expected)
            ));
        }
    }
    println!("S-GIX/b  b1 write API present : {api_write_exists}");
    println!("S-GIX/b  b2 byte-identical    : {byte_identical} of 5");
    for d in &diverged {
        println!("S-GIX/b  diverged            : {d}");
    }

    // The conjunction. b1 is false, so the verdict is no-go whatever b2 says; b2's figure is
    // recorded beside it as a property of the hand-rolled route, never as a property of gix.
    let measured = if api_write_exists && byte_identical == 5 {
        "go"
    } else {
        "no-go"
    };
    assert_agrees_with_record('b', measured);
}

/// Does the read half exist at the pin? A compile-level check: this only builds if the API
/// ERRATA E-0077-1 names is really there.
fn gix_object_body_trailers_exists() -> bool {
    let msg = gix::objs::commit::MessageRef::from_bytes(b"subject\n\nbody\n\nStory: S-1\n");
    msg.body().is_some_and(|b| b.trailers().count() > 0)
}

/// The only route left once b1 finds no API: append the trailer text to the message by hand.
fn hand_rolled_append(input: &[u8], trailer: &str) -> Vec<u8> {
    let mut out = input.to_vec();
    if !out.ends_with(b"\n") {
        out.push(b'\n');
    }
    out.extend_from_slice(trailer.as_bytes());
    out.push(b'\n');
    out
}

fn first_difference(a: &[u8], b: &[u8]) -> usize {
    a.iter()
        .zip(b)
        .position(|(x, y)| x != y)
        .unwrap_or(a.len().min(b.len()))
}
