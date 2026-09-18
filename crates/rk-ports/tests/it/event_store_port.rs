//! AC-364's port half — *"a store trait test asserting no `update`/`delete` method exists on the
//! `EventStore` port"* (`s7c` §7c.13).
//!
//! The criterion: *"GIVEN an appended event / WHEN any later operation attempts to modify or delete
//! it / THEN the store refuses the write / AND a correction is expressible only as a new appended
//! event that references the corrected one."* The port's contribution is that the **trait offers
//! no such operation**: an adapter cannot implement a method the port does not declare, and the
//! kernel cannot call one. The integration half — a correction produces two rows — is
//! `rk-adapter-sqlite`'s (`ac364_correction_appends_never_mutates`, Slice 3). No named port-absence
//! test existed on the tree before Step 7c (the trait has four methods; `rk-store`'s
//! `there_is_no_delete_against_a_tier_one_table` is tier-1 only — critic gap 7).
//!
//! **A source parse, in 7b's AC-028 shape: admission first, then the absences.** The four methods
//! design:265 names are asserted PRESENT on the trait before any absence is asserted, so a parse
//! that finds no trait at all — or the wrong file — cannot go green on an empty set (Halt 8 shape
//! (g)). The parse is over `src/event_store.rs`'s own text, not `Source::parse()`'s crate-wide
//! method set, because the claim is about ONE trait and a method named `delete` on `Tracker`
//! would be a different question.

use std::collections::BTreeSet;

/// The trait block's method names, from `pub trait EventStore` to its closing brace: every
/// non-comment line that declares a method (`fn <name>(` ending in `;`).
fn event_store_methods() -> BTreeSet<String> {
    let text = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/event_store.rs"))
        .expect("rk-ports/src/event_store.rs is readable");
    let start = text
        .find("pub trait EventStore")
        .expect("the EventStore port is declared in src/event_store.rs");
    let body = &text[start..];
    let end = body
        .find("\n}")
        .expect("the trait block closes at column 0");
    let mut methods = BTreeSet::new();
    for line in body[..end].lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("fn ")
            && let Some((name, _)) = rest.split_once('(')
        {
            methods.insert(name.trim().to_owned());
        }
    }
    methods
}

/// The mutating verbs a store might grow and may not: any of these on the port is a write path
/// that AC-364's THEN refuses, and a correction is a NEW appended event referencing the corrected
/// one, never an edit of it.
const FORBIDDEN_VERBS: [&str; 8] = [
    "update",
    "delete",
    "remove",
    "truncate",
    "overwrite",
    "edit",
    "purge",
    "clear",
];

#[test]
fn ac364_event_store_port_has_no_update_or_delete() {
    let methods = event_store_methods();

    // ADMISSION: design:265's four are the port, by name, or the absences below assert nothing.
    for required in ["append", "query", "snapshot", "rebuild"] {
        assert!(
            methods.contains(required),
            "the parse did not find `{required}` on `pub trait EventStore` — the absence checks \
             below would be vacuous over a parse that reads the wrong block: {methods:?}"
        );
    }
    assert_eq!(
        methods.len(),
        4,
        "`EventStore` declares exactly design:265's four methods, and a fifth is a write path a \
         record has to admit: {methods:?}"
    );

    // THE ABSENCES: no method whose name carries a mutating verb, whole or as a part
    // (`delete_before`, `soft_delete`, `update_payload`).
    for method in &methods {
        let lower = method.to_ascii_lowercase();
        for verb in FORBIDDEN_VERBS {
            assert!(
                !lower.contains(verb),
                "`EventStore::{method}` names a mutating operation (`{verb}`): an appended event \
                 is never modified or deleted, and a correction is a new appended event that \
                 references the corrected one (AC-364)"
            );
        }
    }
}
