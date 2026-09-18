//! **AC-238 (7c half) / AC-489 / AC-492 (the Claude row)** — the dated capability registry: the
//! named `dispatch` row and nothing else (D17-9; RT-24; s7c §7c.7).
//!
//! AC-238: *"a dated registry entry per adapter … `{capability, verified_on, probe}`"*. AC-489:
//! *"a row missing `capability`, `verified_on` or `probe` fails the build naming the row"*.
//! AC-492: *"an adapter registry naming a capability token … the canonical vocabulary does not
//! define … fails naming the unknown token / AND no code path maps an unknown token to a default"*.
//!
//! **The Claude registry's rows, named** (§7c.7): `dispatch` — `verified_on: "2026-09-17"`,
//! `Probe::Recorded { source }` naming the corpus pin's recorded dispatches (`results/e3.jsonl` at
//! `61f590e5…`; the F20 transcript on the tree) at Slice 4, **re-dated in the close commit to the
//! probe's corpus commit**; every capability the Harness port does not implement (`snapshot`,
//! `rebuild`, `change_summary`, `affected-selection`) **absent**, not `Probe::None` — a Harness
//! claims no store or toolchain capability. The name of `ac238_…` is honest about its scope: one
//! crate's run cannot check "per adapter"; AC-238's per-adapter half is the four `ac489_…` lines.

use std::path::Path;

use rk_adapter_claude::registry::{self, Capability, Probe, Row};

const BARE_ROW: &str = include_str!("../fixtures/registry/bare_row.json");
const UNDATED_ROW: &str = include_str!("../fixtures/registry/undated_row.json");
const PROBELESS_ROW: &str = include_str!("../fixtures/registry/probeless_row.json");
const UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/unknown_member.json");
const PROBE_UNKNOWN_MEMBER: &str = include_str!("../fixtures/registry/probe_unknown_member.json");
const READS_MINDS: &str = include_str!("../fixtures/registry/reads_minds.json");

/// `YYYY-MM-DD`, digits in the right places — a dated entry, not a free string.
fn is_date(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 10
        && b[4] == b'-'
        && b[7] == b'-'
        && b.iter()
            .enumerate()
            .all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit())
}

/// AC-238 (7c half): the `dispatch` row's three members, dated, with a recorded probe naming the
/// corpus pin; the registry names no token outside the crate's `enum Capability` (the
/// `reads_minds` red); and the capabilities the Harness does not implement are ABSENT.
///
/// RED (observed 2026-09-18) against the skeleton registry (an empty row list):
/// `the Claude registry carries exactly the dispatch row (§7c.7), left: [] right: ["dispatch"]`.
#[test]
fn ac238_claude_registry_entry_dated() {
    let rows = registry::rows().expect("the shipped registry loads");
    let tokens: Vec<&str> = rows.iter().map(Row::token).collect();
    assert_eq!(
        tokens,
        vec!["dispatch"],
        "the Claude registry carries exactly the dispatch row (§7c.7)"
    );
    let row = &rows[0];
    assert_eq!(row.capability, Capability::Dispatch);
    assert!(
        is_date(&row.verified_on),
        "dated YYYY-MM-DD: {:?}",
        row.verified_on
    );
    assert_eq!(
        row.verified_on, "2026-09-17",
        "Slice 4's date; re-dated in the close commit to the probe's corpus commit (§7c.7)"
    );
    match &row.probe {
        Probe::Recorded { source } => {
            assert!(
                source.contains("61f590e51cbb00301c310e46aab615563a7787b8")
                    && source.contains("results/e3.jsonl"),
                "the probe names the corpus pin's recorded dispatches: {source}"
            );
            assert!(
                source.contains("ac282_f20_golden_transcript.jsonl"),
                "and the F20 transcript on the tree: {source}"
            );
        }
        Probe::None { reason } => {
            panic!("dispatch is a recorded capability, not Probe::None ({reason})")
        }
    }
    assert_eq!(row.probe.as_str(), "recorded", "7d's bridge tag");
    // Absent, not Probe::None: a Harness claims no store or toolchain capability.
    for absent in [
        Capability::Snapshot,
        Capability::Rebuild,
        Capability::ChangeSummary,
        Capability::AffectedSelection,
    ] {
        assert!(
            !rows.iter().any(|r| r.capability == absent),
            "{} is absent from the Claude registry, not Probe::None (§7c.7)",
            absent.as_str()
        );
    }
    assert!(
        registry::claims(&rows, Capability::Dispatch),
        "the row claims dispatch"
    );
    assert!(
        !registry::claims(&rows, Capability::Snapshot),
        "and nothing else"
    );
    // The vocabulary is closed: `reads_minds` does not load (AC-492's red, asserted here too).
    assert!(registry::load(READS_MINDS).is_err());
}

/// AC-489: every member required, no unknown member, at every level — the must-fail fixtures each
/// name THEIR member; the shipped registry loads and is dated.
///
/// RED (observed 2026-09-18) against the skeleton whose members were `#[serde(default)]`:
/// `a bare capability-only row must not load: [Row { capability: Dispatch, verified_on: "",
/// probe: None { reason: "" } }]` — the bare row LOADED, with an empty date and an empty reason.
#[test]
fn ac489_capability_registry_dated() {
    let err = registry::load(BARE_ROW).expect_err("a bare capability-only row must not load");
    let text = err.to_string();
    assert!(
        text.contains("verified_on") || text.contains("probe"),
        "the refusal names the missing member: {text}"
    );
    let err = registry::load(UNDATED_ROW).expect_err("an undated row must not load (AC-489)");
    let text = err.to_string();
    assert!(
        text.contains("verified_on") && !text.contains("probe"),
        "the undated row's refusal names `verified_on` and nothing else missing: {text}"
    );
    let err = registry::load(PROBELESS_ROW).expect_err("a probe-less row must not load (AC-489)");
    let text = err.to_string();
    assert!(
        text.contains("probe") && !text.contains("verified_on"),
        "the probe-less row's refusal names `probe` and nothing else missing: {text}"
    );
    let err = registry::load(UNKNOWN_MEMBER).expect_err("an undeclared member must not load");
    assert!(
        err.to_string().contains("adopted"),
        "the refusal names the unknown member: {err}"
    );
    let err = registry::load(PROBE_UNKNOWN_MEMBER)
        .expect_err("an undeclared member on the probe must not load");
    assert!(
        err.to_string().contains("extra"),
        "the refusal names the unknown probe member: {err}"
    );

    let rows = registry::rows().expect("the shipped registry loads");
    assert_eq!(rows.len(), 1);
    for row in &rows {
        assert!(
            is_date(&row.verified_on),
            "row {} is not dated YYYY-MM-DD",
            row.token()
        );
        match &row.probe {
            Probe::Recorded { source } => assert!(!source.trim().is_empty()),
            Probe::None { reason } => assert!(!reason.trim().is_empty()),
        }
    }
    // A row is readable only through all three members: the type has no Option member and no
    // default, so the shape below is the only one that constructs.
    let _ = Row {
        capability: Capability::Dispatch,
        verified_on: "2026-09-17".to_owned(),
        probe: Probe::Recorded {
            source: "x".to_owned(),
        },
    };
}

/// AC-492: `reads_minds` fails naming the token; every shipped token is one the closed vocabulary
/// spells; and the token mapper carries no catch-all arm in any spelling (the lint half, over the
/// `impl Capability` block of `src/registry.rs`, with an admission that the five arms are named).
/// RT-24's byte-equality with the other three crates' `enum Capability` bodies is the block's
/// digest line; asserted here against `rk-adapter-sqlite`'s copy on the tree as well.
///
/// RED (observed 2026-09-18) against the skeleton whose `Capability` carried a catch-all
/// `#[serde(other)] Unknown` variant: `` `reads_minds` is not a capability: [Row { capability:
/// Unknown, verified_on: "2026-09-17", probe: None { reason: "no probe reads minds" } }] `` — it
/// loaded as a row, mapped to a default. RED again (observed 2026-09-18) against a mutant mapper
/// whose last arm was the named binding `other => "affected-selection"` — the `_ =>`-only lint was
/// green on it: `a catch-all arm in the token mapper maps an unknown token to a default (AC-492):
/// ["registry.rs:71:             other => \"affected-selection\","]`.
#[test]
fn ac492_unknown_capability_token_fails() {
    let err = registry::load(READS_MINDS).expect_err("`reads_minds` is not a capability");
    assert!(
        err.to_string().contains("reads_minds"),
        "the refusal names the unknown token: {err}"
    );
    let known = [
        "dispatch",
        "snapshot",
        "rebuild",
        "change_summary",
        "affected-selection",
    ];
    for row in registry::rows().expect("loads") {
        assert!(
            known.contains(&row.token()),
            "the shipped registry names {} outside the vocabulary",
            row.token()
        );
    }
    for (cap, token) in [
        (Capability::Dispatch, "dispatch"),
        (Capability::Snapshot, "snapshot"),
        (Capability::Rebuild, "rebuild"),
        (Capability::ChangeSummary, "change_summary"),
        (Capability::AffectedSelection, "affected-selection"),
    ] {
        assert_eq!(cap.as_str(), token);
    }

    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src = std::fs::read_to_string(here.join("src/registry.rs")).expect("read src/registry.rs");
    // The lint half, in the 7b AC-028 shape §7c.18's ac229 bullet names for this class: names
    // present, then absences. ADMISSION first — the token mapper's `match self` names all five
    // variants as `Self::<Variant> =>` arms inside `impl Capability` — so an empty or mis-scoped
    // search is never green; then no catch-all in ANY spelling inside that block: `_ =>`, a guarded
    // wildcard `_ if`, a `matches!(`, or a named binding as the whole pattern (`other =>`).
    let lines: Vec<&str> = src.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.starts_with("impl Capability"))
        .expect("column-0 `impl Capability` in src/registry.rs");
    let end = lines[start..]
        .iter()
        .position(|l| *l == "}")
        .expect("closing brace")
        + start;
    let block: Vec<(usize, &str)> = lines[start..=end]
        .iter()
        .enumerate()
        .map(|(k, l)| (start + k + 1, *l))
        .filter(|(_, l)| {
            let t = l.trim_start();
            !(t.starts_with("//") || t.starts_with('*') || t.starts_with("/*"))
        })
        .collect();
    for variant in [
        "Dispatch",
        "Snapshot",
        "Rebuild",
        "ChangeSummary",
        "AffectedSelection",
    ] {
        let arm = format!("Self::{variant} =>");
        assert!(
            block.iter().any(|(_, l)| l.contains(&arm)),
            "ADMISSION: the token mapper names `{arm}` (the exhaustive match the lint reads)"
        );
    }
    let is_ident_arm = |t: &str| {
        // `<ident> =>` where the whole pattern is one identifier not starting with an upper-case
        // letter: a named binding that matches everything (`other => …`).
        t.split_once(" =>").is_some_and(|(pat, _)| {
            let pat = pat.trim();
            !pat.is_empty()
                && pat
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b == b'_' || b.is_ascii_digit())
        })
    };
    let catch_alls: Vec<String> = block
        .iter()
        .filter(|(_, l)| {
            let t = l.trim_start();
            t.starts_with("_ =>")
                || l.contains(" _ =>")
                || t.starts_with("_ if")
                || l.contains(" _ if ")
                || l.contains("matches!(")
                || is_ident_arm(t)
        })
        .map(|(n, l)| format!("registry.rs:{n}: {l}"))
        .collect();
    assert!(
        catch_alls.is_empty(),
        "a catch-all arm in the token mapper maps an unknown token to a default (AC-492): {catch_alls:?}"
    );
    // RT-24: the `pub enum Capability` body is byte-equal with rk-adapter-sqlite's (the block digests
    // all four; two exist on the tree at Slice 4).
    // The block's range: from the column-0 `pub enum Capability {` line to the next column-0 `}`.
    let body = |text: &str| -> String {
        let lines: Vec<&str> = text.lines().collect();
        let start = lines
            .iter()
            .position(|l| l.starts_with("pub enum Capability"))
            .expect("column-0 `pub enum Capability`");
        let end = lines[start..]
            .iter()
            .position(|l| *l == "}")
            .expect("closing brace")
            + start;
        lines[start..=end].join("\n")
    };
    let sqlite = std::fs::read_to_string(here.join("../rk-adapter-sqlite/src/registry.rs"))
        .expect("sqlite's registry");
    assert_eq!(
        body(&src),
        body(&sqlite),
        "one vocabulary, byte-equal bodies (RT-24)"
    );
}
