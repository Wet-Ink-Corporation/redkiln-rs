//! The one hash. ADR-0097 decision 2 and ADR-0116 clause 6.
//!
//! `digest.rs` deferred computation to Step 6 on purpose: *"a type that can only be constructed
//! from data it was given cannot silently mint an id"*. Step 6 is where the memo key needs it, and
//! ADR-0116 clause 6 fixes the constraint that comes with it — *"The digest is the artifact's
//! content digest, computed by the same function the typed-artifact digests use (`rk-types`) … No
//! second digest algorithm enters the system."*
//!
//! So there is exactly one, it lives here, and the length-prefixed framing ADR-0097 mandates is
//! part of it rather than something each caller reimplements.

use rk_types::digest::{sha256, sha256_lp};

/// Against the published SHA-256 of the empty string and of "abc" — an oracle outside this
/// codebase, so the test cannot agree with a wrong implementation by construction.
#[test]
fn sha256_matches_the_published_vectors() {
    assert_eq!(
        sha256(b"").as_str(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256(b"abc").as_str(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

/// **The property length-prefixing exists for.** Without it, `("ab", "c")` and `("a", "bc")` hash
/// identically, and ADR-0097's eight-component tuple would collide across component boundaries: a
/// different activity with a compensating step id would resolve to the same memo entry and a step
/// that should have re-run would be served a stale result.
#[test]
fn length_prefixing_separates_the_components() {
    let a = sha256_lp(&[b"ab", b"c"]);
    let b = sha256_lp(&[b"a", b"bc"]);
    assert_ne!(
        a, b,
        "concatenation without a length prefix collides across component boundaries — this is the \
         defect ADR-0097's `u32-length || bytes` framing exists to prevent"
    );
}

/// The mutant this is written against: plain concatenation. Stated explicitly so the property above
/// is not mistaken for an arbitrary inequality.
#[test]
fn plain_concatenation_would_collide_where_the_framing_does_not() {
    // What a naive implementation computes for both shapes.
    let naive_a = sha256(b"abc");
    let naive_b = sha256(b"abc");
    assert_eq!(
        naive_a, naive_b,
        "sanity: the naive form really does collide"
    );

    // What the framed form computes.
    assert_ne!(sha256_lp(&[b"ab", b"c"]), naive_a);
}

/// Empty components are not free: `()`, `("",)` and `("", "")` are three different tuples.
#[test]
fn empty_components_are_distinguished() {
    let none = sha256_lp(&[]);
    let one_empty = sha256_lp(&[b""]);
    let two_empty = sha256_lp(&[b"", b""]);
    assert_ne!(none, one_empty);
    assert_ne!(one_empty, two_empty);
}

/// Deterministic across calls — the whole point, and cheap to assert.
#[test]
fn the_digest_is_stable() {
    assert_eq!(sha256(b"redkiln"), sha256(b"redkiln"));
    assert_eq!(sha256_lp(&[b"x", b"y"]), sha256_lp(&[b"x", b"y"]));
}

/// Output is always a valid `Digest`: 64 lowercase hex characters.
#[test]
fn the_output_is_always_a_valid_digest() {
    for input in [
        b"".as_slice(),
        b"a",
        b"a longer string with spaces and \xff\x00 bytes",
    ] {
        let d = sha256(input);
        assert_eq!(d.as_str().len(), 64);
        assert!(
            d.as_str()
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
    }
}

/// **There is exactly one hash construction site in `rk-types`.** ADR-0116 clause 6's "no second
/// digest algorithm" is a universal claim, and a grep is the only instrument that can see it.
#[test]
fn there_is_exactly_one_hasher_construction_site() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut hits = Vec::new();
    let mut stack = vec![src];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)
            .expect("read src")
            .filter_map(Result::ok)
        {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                let text = std::fs::read_to_string(&p).expect("read");
                for (i, line) in text.lines().enumerate() {
                    let is_doc = line.trim_start().starts_with("//");
                    if line.contains("Sha256::new()") && !is_doc {
                        hits.push(format!("{}:{}", p.display(), i + 1));
                    }
                }
            }
        }
    }
    assert_eq!(
        hits.len(),
        1,
        "expected exactly one Sha256 construction site (ADR-0116 clause 6: no second digest \
         algorithm enters the system); found: {hits:?}"
    );
}
