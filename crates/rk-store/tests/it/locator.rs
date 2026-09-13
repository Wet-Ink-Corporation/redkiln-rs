//! The section locator — AC-149, AC-150, AC-151.

use rk_store::locator::{Anchor, LocatorError, resolve};

fn a(text: &str, level: usize) -> Anchor {
    Anchor {
        text: text.into(),
        level,
    }
}

const DOC: &str = "\
# Title

## Notes

prose one
prose two

### Deeper inside Notes

still inside Notes

## Other

other content
";

/// AC-149: the slice runs from the line after the anchor to the next heading **at that level or
/// shallower** — and a **deeper** heading inside it is prose, not a boundary.
#[test]
fn the_slice_ends_at_the_next_heading_of_that_level_or_shallower() {
    let r = resolve(DOC, &a("Notes", 2)).expect("resolve");
    let slice = &DOC[r];
    assert!(slice.contains("prose one"));
    assert!(
        slice.contains("### Deeper inside Notes"),
        "a DEEPER heading ended the slice; it is prose bytes, not an anchor (PIA-033)"
    );
    assert!(slice.contains("still inside Notes"));
    assert!(
        !slice.contains("other content"),
        "the slice ran past the next same-level heading"
    );
}

/// The other end rule: a slice that reaches EOF.
#[test]
fn a_slice_may_run_to_end_of_file() {
    let r = resolve(DOC, &a("Other", 2)).expect("resolve");
    assert!(DOC[r].contains("other content"));
}

/// A shallower heading also ends a slice.
#[test]
fn a_shallower_heading_ends_the_slice() {
    let doc = "## A\n\ninside a\n\n# B\n\ninside b\n";
    let r = resolve(doc, &a("A", 2)).expect("resolve");
    let slice = &doc[r];
    assert!(slice.contains("inside a"));
    assert!(
        !slice.contains("inside b"),
        "a shallower heading did not end the slice"
    );
}

/// AC-149's other half: what comes back is a **byte range**, and nothing derived from the content.
/// Asserted structurally — the return type is `Range<usize>`, so there is no content-derived field
/// for a caller to read.
#[test]
fn the_return_is_a_byte_range_and_carries_no_content_derived_field() {
    let r: std::ops::Range<usize> = resolve(DOC, &a("Notes", 2)).expect("resolve");
    assert!(r.start < r.end);
    assert!(r.end <= DOC.len());
}

// ---------------------------------------------------------------- AC-150: decoys

/// One fixture per decoy form. Each puts the target heading text somewhere that looks like a
/// heading and is not, plus one real heading — and the real one must be selected.
#[test]
fn decoys_are_not_selected() {
    let cases: [(&str, &str); 5] = [
        (
            "fenced code block",
            "# Doc\n\n```\n## Target\n```\n\n## Target\n\nreal content\n",
        ),
        (
            "indented block",
            "# Doc\n\n    ## Target\n\n## Target\n\nreal content\n",
        ),
        (
            "html comment",
            "# Doc\n\n<!--\n## Target\n-->\n\n## Target\n\nreal content\n",
        ),
        (
            "blockquote",
            "# Doc\n\n> ## Target\n\n## Target\n\nreal content\n",
        ),
        (
            "deeper heading inside another slice",
            "# Doc\n\n## Elsewhere\n\n### Target\n\n## Target\n\nreal content\n",
        ),
    ];

    for (name, doc) in cases {
        let r = resolve(doc, &a("Target", 2))
            .unwrap_or_else(|e| panic!("{name}: the real heading was not resolved: {e}"));
        assert!(
            doc[r].contains("real content"),
            "{name}: a decoy was selected instead of the real heading"
        );
    }
}

/// The property AC-150 states: inserting a decoy into a document does not move the resolved range.
#[test]
fn inserting_a_decoy_does_not_change_the_resolved_range() {
    let plain = "# Doc\n\n## Target\n\nreal content\n";
    let before = resolve(plain, &a("Target", 2)).expect("resolve");
    let before_text = plain[before].to_string();

    for decoy in ["```\n## Target\n```\n", "    ## Target\n", "> ## Target\n"] {
        let with = format!("# Doc\n\n{decoy}\n## Target\n\nreal content\n");
        let r = resolve(&with, &a("Target", 2)).expect("resolve");
        assert_eq!(
            with[r], before_text,
            "inserting `{decoy}` changed the resolved content"
        );
    }
}

/// A `#hashtag` is not a heading: ATX requires a space after the hashes.
#[test]
fn a_hash_without_a_space_is_not_a_heading() {
    let doc = "# Doc\n\n##Target\n\n## Target\n\nreal content\n";
    let r = resolve(doc, &a("Target", 2)).expect("resolve");
    assert!(doc[r].contains("real content"));
}

// ---------------------------------------------------------------- AC-151: occurrence counting

/// Zero occurrences is a named error carrying the count — **not** an empty range.
#[test]
fn zero_occurrences_is_an_error_naming_the_count() {
    match resolve(DOC, &a("Absent", 2)) {
        Err(LocatorError::Occurrences {
            anchor,
            level,
            count,
        }) => {
            assert_eq!(anchor, "Absent");
            assert_eq!(level, 2);
            assert_eq!(count, 0);
        }
        other => panic!("expected an Occurrences error: {other:?}"),
    }
}

/// **The discriminating case.** Two genuine headings at the declared level must be an error — never
/// a first-match. The mutation AC-151 names is changing this path to first-match, and it must fail
/// the suite.
#[test]
fn a_duplicate_genuine_heading_is_an_error_and_never_a_first_match() {
    let doc = "# Doc\n\n## Target\n\nfirst\n\n## Target\n\nsecond\n";
    match resolve(doc, &a("Target", 2)) {
        Err(LocatorError::Occurrences { count, .. }) => assert_eq!(count, 2),
        Ok(r) => panic!(
            "a first-match range was returned for a duplicated anchor: {:?}. AC-151 forbids \
             first-match, nearest-match and fuzzy-match.",
            &doc[r]
        ),
        other => panic!("wrong error: {other:?}"),
    }
}

/// A same-level duplicate **inside another section's prose** is prose, and must still resolve.
/// This is the fixture AC-151 names beside the duplicate case, and it is the one that makes the
/// rule about *genuine* headings rather than about text.
#[test]
fn a_same_level_duplicate_inside_a_code_block_is_prose_and_still_resolves() {
    let doc = "# Doc\n\n## Elsewhere\n\n```\n## Target\n```\n\n## Target\n\nreal content\n";
    let r = resolve(doc, &a("Target", 2)).expect("the decoy must not count as an occurrence");
    assert!(doc[r].contains("real content"));
}

/// The count is right when there are three, so the error reports what it found rather than just
/// "more than one".
#[test]
fn the_occurrence_count_is_exact() {
    let doc = "## T\n\na\n\n## T\n\nb\n\n## T\n\nc\n";
    match resolve(doc, &a("T", 2)) {
        Err(LocatorError::Occurrences { count, .. }) => assert_eq!(count, 3),
        other => panic!("expected three: {other:?}"),
    }
}
