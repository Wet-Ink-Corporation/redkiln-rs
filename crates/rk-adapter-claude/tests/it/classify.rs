//! **The classifier over the closed eleven** — AC-229 (the adapter's exhaustive match, as a
//! source parse in 7b's AC-028 shape) and AC-218 / AC-428's four-way matrix over the recorded
//! `e3.jsonl` frames (s7c §7c.7, §7c.17, §7c.18).

use rk_types::digest::sha256;
use rk_types::error::{CancelledBy, StepError};
use rk_types::typed::SchemaId;

use rk_adapter_claude::classify::{Evidence, cancelled_by_of, classify};

use crate::support::{fixture, schema};

/// AC-229 (the adapter's obligation): the file that constructs the row's outcome —
/// `src/classify.rs` — names each of the eleven `StepError::<Variant>`s (ADMISSION first), THEN
/// carries no `_ =>`, no `_ if` and no `matches!(` (the absences), in 7b's AC-028 shape
/// (`crates/rk-kernel/tests/it/retry.rs`). Red by any name absent or any wildcard; the
/// exhaustiveness itself is observable only by deleting an arm (E0004). This test observes
/// nothing about what the two undriven arms DO (§7c.11).
///
/// RED (observed 2026-09-18) against the skeleton (the runner's shape, the eleven-arm match
/// absent): `classify.rs does not name StepError::Timeout — the adapter's match over the closed
/// eleven is not exhaustive by name (AC-229)`.
#[test]
fn ac229_adapter_maps_every_failure_to_closed_variant() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/classify.rs"),
    )
    .expect("src/classify.rs");
    let code: Vec<&str> = src
        .lines()
        .filter(|l| {
            let t = l.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*'))
        })
        .collect();
    let code = code.join("\n");
    // admission: every one of the eleven, by name
    for v in [
        "Transport",
        "Schema",
        "Timeout",
        "Cancelled",
        "NonZero",
        "RateLimited",
        "NotAdopted",
        "NotIsolated",
        "Platform",
        "Unsupported",
        "Overage",
    ] {
        assert!(
            code.contains(&format!("StepError::{v}")),
            "classify.rs does not name StepError::{v} — the adapter's match over the closed eleven \
             is not exhaustive by name (AC-229)"
        );
    }
    // then the absences
    for banned in ["_ =>", "_ if", "matches!("] {
        assert!(
            !code.contains(banned),
            "classify.rs carries `{banned}` — a catch-all lets a twelfth variant compile silently \
             (AC-229; ADR-0112)"
        );
    }
    // and the match is live: the one function that matches all eleven answers per variant
    assert_eq!(
        cancelled_by_of(&StepError::Cancelled {
            by: CancelledBy::Timeout
        }),
        Some(CancelledBy::Timeout)
    );
    assert_eq!(
        cancelled_by_of(&StepError::Unsupported {
            capability: "x".to_owned()
        }),
        None
    );
}

/// An `Evidence` for the matrix: an init was read (the isolation decisions are `isolation.rs`'s),
/// the declared list is the probe card's, nothing on stderr.
fn evidence<'a>(
    exit_code: Option<i32>,
    result_raw: Option<&'a str>,
    schema_id: &'a SchemaId,
    schema_json: &'a serde_json::Value,
) -> Evidence<'a> {
    Evidence {
        cancelled: None,
        decided: None,
        init_seen: true,
        declared_tools: &[],
        reading_refused: None,
        stdin_error: None,
        exit_code,
        result_raw,
        stderr: b"",
        stderr_digest: sha256(b""),
        last_non_allowed: None,
        schema_id,
        schema: schema_json,
        // *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; s8h §8h.5.)* No
        // denial event and no worktree: the matrix observes neither channel.
        denial_events: &[],
        worktree: None,
    }
}

/// AC-218 / AC-428: *"`is_error: true` or an `api_error_status` / WHEN the process exits with
/// any code / THEN `StepError::Platform { status, retry_after }` — the exit code is not consulted
/// to overturn it"* — the four-way matrix, each cell with its source named (§7c.17):
/// **non-zero × set** over the SEVEN recorded `e3.jsonl` frames (every one exited 1 at the pin);
/// **exit-0 × unset** over the F20 `result_raw` (exit 0, `is_error: false` → `Schema`, never
/// `Ok`, never `Platform`); **non-zero × unset** — `// SYNTHETIC — no recorded exit≠0 /
/// is_error-unset frame identified at the pin` (→ `NonZero`, never `Platform`); **exit-0 × set** —
/// the discriminating cell, `// SYNTHETIC exit code — every recorded is_error frame exited 1 at
/// the pin` (a recorded frame with the exit substituted 0 → `Platform`, never `Ok`).
///
/// RED (observed 2026-09-18) against the skeleton (is_error never consulted): `exit-0 × set
/// (e3.L14 with a SYNTHETIC exit code 0) must be Platform, never Ok: Ok(Typed { … })`.
#[test]
fn ac218_is_error_is_platform_regardless_of_exit() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    // non-zero × set: the seven recorded frames, exit 1 (the runner's recorded exit_code)
    for n in [14, 16, 17, 18, 19, 21, 26] {
        let raw = fixture(&format!("recorded/e3.L{n}.result_raw.jsonl"));
        let c = classify(&evidence(Some(1), Some(raw.trim_end()), &id, &json));
        match c.result {
            Err(StepError::Platform { status, .. }) => assert!(
                status == 500 || status == 529,
                "e3.L{n}: the recorded api_error_status: {status}"
            ),
            other => panic!("non-zero × set (e3.L{n}) must be Platform: {other:?}"),
        }
    }
    // exit-0 × set: SYNTHETIC exit code — every recorded is_error frame exited 1 at the pin
    let raw = fixture("recorded/e3.L14.result_raw.jsonl");
    let c = classify(&evidence(Some(0), Some(raw.trim_end()), &id, &json));
    assert!(
        matches!(c.result, Err(StepError::Platform { status: 500, .. })),
        "exit-0 × set (e3.L14 with a SYNTHETIC exit code 0) must be Platform, never Ok: {:?}",
        c.result
    );
    // exit-0 × unset: the F20 result_raw — Schema (no structured_output), never Platform, never Ok
    let raw = fixture("recorded/f20.result_raw.jsonl");
    let c = classify(&evidence(Some(0), Some(raw.trim_end()), &id, &json));
    assert!(
        matches!(c.result, Err(StepError::Schema { .. })),
        "exit-0 × unset (F20) is Schema: {:?}",
        c.result
    );
    // non-zero × unset: SYNTHETIC — no recorded exit≠0 / is_error-unset frame identified at the pin
    let raw = fixture("streams/result-ok.scripted.ndjson");
    let c = classify(&evidence(Some(1), Some(raw.trim_end()), &id, &json));
    match c.result {
        Err(StepError::NonZero {
            code,
            stderr_digest,
        }) => {
            assert_eq!(code, Some(1));
            assert_eq!(stderr_digest, sha256(b""));
        }
        other => panic!("non-zero × unset (SYNTHETIC) must not be Platform: {other:?}"),
    }
    // and the exit-0 × unset cell with a validated value is the one Ok
    let c = classify(&evidence(Some(0), Some(raw.trim_end()), &id, &json));
    assert!(c.result.is_ok(), "{:?}", c.result);
    assert_eq!(c.detected_by, None);
}

// ------------------------------------------------------------------------------------------------
// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* `s8h` §8h.5's guard 3 —
// `permission_denials` and the `permission_denied` stream event, each a typed failure — and §8h.4's
// N24 ingest check on a `Spec`'s authored `verification`. N17's Q1 (iv): a denial is
// `NotIsolated { surface: Env }`, the digest over the sorted denied `(tool_name, path)` entries
// against the empty set; it outranks a valid `structured_output` and is taken whatever the exit
// code, after the platform arms (`classify.rs`'s order).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_types::error::IsolationSurface;

use rk_adapter_claude::classify::{
    Denial, DenialChannel, ProtectedSegmentDenial, denial_digest, denial_path, protected_segment,
};

/// An `Evidence` over a frame, with the denial events and the worktree the test names.
fn evidence_with<'a>(
    exit_code: Option<i32>,
    result_raw: Option<&'a str>,
    schema_id: &'a SchemaId,
    schema_json: &'a serde_json::Value,
    denial_events: &'a [String],
    worktree: Option<&'a Path>,
) -> Evidence<'a> {
    Evidence {
        denial_events,
        worktree,
        ..evidence(exit_code, result_raw, schema_id, schema_json)
    }
}

/// The `(tool_name, path)` entry digest guard 3 carries, computed by the TEST from its own literal
/// entries (never through the implementation's extractor).
fn entries_digest(entries: &[(&str, &str)]) -> rk_types::digest::Digest {
    let set: BTreeSet<String> = entries
        .iter()
        .map(|(t, p)| format!("{t}\u{1e}{p}"))
        .collect();
    rk_adapter_claude::init::set_digest(&set, &BTreeSet::new())
}

/// The scripted `Ok` result frame with `permission_denials` set to `denials` (and, when `so` is
/// `false`, its `structured_output` removed).
fn frame_with_denials(denials: &serde_json::Value, so: bool) -> String {
    let mut v: serde_json::Value =
        serde_json::from_str(fixture("streams/result-ok.scripted.ndjson").trim_end())
            .expect("the scripted result parses");
    let o = v.as_object_mut().expect("an object");
    o.insert("permission_denials".to_owned(), denials.clone());
    if !so {
        o.remove("structured_output");
    }
    v.to_string()
}

fn outside_write() -> serde_json::Value {
    serde_json::json!([{
        "tool_name": "Write",
        "tool_use_id": "toolu_w1",
        "tool_input": { "file_path": "/outside/outside-write.txt", "content": "token" }
    }])
}

const fn is_env(
    r: &Result<rk_types::typed::Typed, StepError>,
) -> Option<&rk_types::digest::Digest> {
    match r {
        Err(StepError::NotIsolated {
            surface: IsolationSurface::Env,
            diff_digest,
        }) => Some(diff_digest),
        _ => None,
    }
}

/// §8h.5 guard 3 (a): a non-empty result `permission_denials` fails the dispatch as
/// `NotIsolated { surface: Env }` (N17 Q1 (iv)) WHATEVER THE EXIT CODE — exit 0 and exit 1 alike,
/// and over a frame with no `structured_output` (which is `Schema` without the guard). Through the
/// real `dispatch` too, so the drain's frame reaches the arm.
#[test]
fn permission_denials_non_empty_is_a_typed_failure() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let want = entries_digest(&[("Write", "/outside/outside-write.txt")]);
    let raw = frame_with_denials(&outside_write(), false);
    for code in [Some(0), Some(1)] {
        let c = classify(&evidence(code, Some(&raw), &id, &json));
        assert_eq!(
            is_env(&c.result),
            Some(&want),
            "exit {code:?}: a non-empty permission_denials must be NotIsolated {{ surface: env }} \
             over the denied (tool_name, path) entries (s8h §8h.5; N17 Q1 (iv)); got {:?}",
            c.result
        );
    }
    let r = crate::support::dispatch_replay(
        &crate::support::transcript(&[
            crate::support::Part::File("streams/init.scripted.ndjson"),
            crate::support::Part::File("streams/result-ok-permission-denials.scripted.ndjson"),
        ]),
        30,
    );
    assert_eq!(
        is_env(&r.outcome.result),
        Some(&want),
        "through dispatch: the result frame's permission_denials is guard 3's channel (a); got {:?}",
        r.outcome.result
    );
}

/// The control: an EMPTY `permission_denials` beside a valid `structured_output` is `Ok` — unit
/// and through the real `dispatch`.
#[test]
fn permission_denials_empty_is_not_a_failure() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let raw = frame_with_denials(&serde_json::json!([]), true);
    let c = classify(&evidence(Some(0), Some(&raw), &id, &json));
    assert!(
        c.result.is_ok(),
        "an empty permission_denials is not a failure (s8h §8h.5): {:?}",
        c.result
    );
    let r = crate::support::dispatch_replay(
        &crate::support::transcript(&[
            crate::support::Part::File("streams/init.scripted.ndjson"),
            crate::support::Part::File(
                "streams/result-ok-empty-permission-denials.scripted.ndjson",
            ),
        ]),
        30,
    );
    assert!(
        r.outcome.result.is_ok(),
        "through dispatch: an empty list is Ok: {:?}",
        r.outcome.result
    );
}

/// §8h.5: the denial arm OUTRANKS a valid `structured_output` — a step whose child was denied has
/// not done what it reported, so a schema-valid value beside a non-empty list is never `Ok`.
#[test]
fn permission_denials_outrank_a_valid_structured_output() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let raw = frame_with_denials(&outside_write(), true);
    let c = classify(&evidence(Some(0), Some(&raw), &id, &json));
    assert!(
        is_env(&c.result).is_some(),
        "a valid structured_output beside a non-empty permission_denials must be NotIsolated \
         {{ surface: env }}, never Ok — the denial arm sits before the structured_output \
         validation (s8h §8h.5); got {:?}",
        c.result
    );
}

/// §8h.5 / D21-45's standing limit, named: a denial of `<wt>/.cargo/config.toml` has the SAME class
/// and digest as any denial, plus a `ProtectedSegmentDenial` note naming `.cargo`. Controls: a
/// denial outside the worktree carries no note, and neither does a path whose only `.cargo` segment
/// is ABOVE the worktree root. A relative path resolves against the worktree, and the segment match
/// is case-folded.
#[test]
fn a_denial_under_a_protected_worktree_segment_is_named_the_known_limit() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let wt = crate::support::worktree();
    let inside = format!("{}/.cargo/config.toml", wt.display());
    let denials = serde_json::json!([{
        "tool_name": "Edit", "tool_use_id": "toolu_e1",
        "tool_input": { "file_path": inside, "old_string": "a", "new_string": "b" }
    }]);
    let raw = frame_with_denials(&denials, true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &[],
        Some(&wt),
    ));
    assert_eq!(
        is_env(&c.result),
        Some(&entries_digest(&[("Edit", inside.as_str())])),
        "the class and the digest are guard 3's own — the note moves neither: {:?}",
        c.result
    );
    assert_eq!(
        c.notes,
        vec![ProtectedSegmentDenial {
            segment: ".cargo".to_owned(),
            tool_name: "Edit".to_owned(),
        }],
        "a denial under <wt>/.cargo carries the note naming D21-45's known limit (s8h §8h.5)"
    );
    assert!(
        c.notes[0].warning().contains("D21-45"),
        "the warning names D21-45: {}",
        c.notes[0].warning()
    );
    // a relative path resolves against the worktree; the segment match is case-folded
    let rel = serde_json::json!([{
        "tool_name": "Write", "tool_use_id": "toolu_w2",
        "tool_input": { "file_path": ".CARGO/rk-probe.txt", "content": "x" }
    }]);
    let raw = frame_with_denials(&rel, true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &[],
        Some(&wt),
    ));
    assert_eq!(
        c.notes
            .iter()
            .map(|n| n.segment.as_str())
            .collect::<Vec<_>>(),
        vec![".cargo"],
        "a relative .CARGO path resolves inside the worktree and is case-folded"
    );
    // control: outside the worktree, no note
    let raw = frame_with_denials(&outside_write(), true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &[],
        Some(&wt),
    ));
    assert!(is_env(&c.result).is_some(), "{:?}", c.result);
    assert!(
        c.notes.is_empty(),
        "CONTROL: a denial outside the worktree carries no note: {:?}",
        c.notes
    );
    // control: the only .cargo segment is ABOVE the worktree root
    let high = PathBuf::from(format!("{}/.cargo/wt", wt.display()));
    let below = format!("{}/src/lib.rs", high.display());
    let d = serde_json::json!([{
        "tool_name": "Edit", "tool_use_id": "toolu_e3",
        "tool_input": { "file_path": below, "old_string": "a", "new_string": "b" }
    }]);
    let raw = frame_with_denials(&d, true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &[],
        Some(&high),
    ));
    assert!(is_env(&c.result).is_some(), "{:?}", c.result);
    assert!(
        c.notes.is_empty(),
        "CONTROL: a .cargo segment above the worktree root is not a protected segment below it: \
         {:?}",
        c.notes
    );
}

/// §8h.5 guard 3 (b), D21-47 / s8h review #2: an EMPTY result list beside one `permission_denied`
/// stream event (the DRAFTED frame, `decision_reason_code` `outside_reads_blocked`) is
/// `NotIsolated { surface: Env }` — unit, and through the real `dispatch`, where the drain collects
/// the event. The digest is over the event's `(tool_name, path)`.
#[test]
fn a_permission_denied_stream_event_is_a_typed_failure() {
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let want = entries_digest(&[("Read", "/outside/outside-read.txt")]);
    let event = fixture("streams/permission-denied-outside-read.scripted.ndjson")
        .trim_end()
        .to_owned();
    let raw = frame_with_denials(&serde_json::json!([]), true);
    let events = [event];
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &events,
        None,
    ));
    assert_eq!(
        is_env(&c.result),
        Some(&want),
        "a permission_denied event beside an empty result list must be NotIsolated {{ surface: \
         env }} (s8h §8h.5 channel (b)); got {:?}",
        c.result
    );
    let r = crate::support::dispatch_replay(
        &crate::support::transcript(&[
            crate::support::Part::File("streams/init.scripted.ndjson"),
            crate::support::Part::File("streams/permission-denied-outside-read.scripted.ndjson"),
            crate::support::Part::File(
                "streams/result-ok-empty-permission-denials.scripted.ndjson",
            ),
        ]),
        30,
    );
    assert_eq!(
        is_env(&r.outcome.result),
        Some(&want),
        "through dispatch: the drain carries the event to the arm; got {:?}",
        r.outcome.result
    );
    // a denial on BOTH channels with one tool_use_id is one entry (the union, de-duplicated)
    let both = serde_json::json!([{
        "tool_name": "Read", "tool_use_id": "toolu_scripted_read_1",
        "tool_input": { "file_path": "/outside/outside-read.txt" }
    }]);
    let raw = frame_with_denials(&both, true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &events,
        None,
    ));
    assert_eq!(
        is_env(&c.result),
        Some(&want),
        "one denial on both channels is one entry in the digest: {:?}",
        c.result
    );
}

/// §8h.5 / s8h review #10: a denial's path is extracted PER TOOL from its `tool_input` —
/// `file_path` for `Read`/`Edit`/`Write`, `path` for `Glob`/`Grep` (the working directory when the
/// key is absent), an absolute `pattern` for `Glob`, and `""` otherwise. `.config/git` is a PAIR of
/// consecutive segments: `<wt>/.config/git/x` carries the note, `<wt>/.config/x` and `<wt>/git/x`
/// do not. A denial with no extractable path enters the digest as `(tool_name, "")` and carries no
/// note.
#[test]
fn denial_paths_are_extracted_per_tool() {
    let wt = crate::support::worktree();
    let cwd = Some(wt.as_path());
    let cases: [(&str, serde_json::Value, String); 12] = [
        (
            "Read",
            serde_json::json!({ "file_path": "/a/r.txt" }),
            "/a/r.txt".to_owned(),
        ),
        (
            "Edit",
            serde_json::json!({ "file_path": "/a/e.txt", "old_string": "x" }),
            "/a/e.txt".to_owned(),
        ),
        (
            "Write",
            serde_json::json!({ "file_path": "/a/w.txt", "content": "x" }),
            "/a/w.txt".to_owned(),
        ),
        (
            "Glob",
            serde_json::json!({ "pattern": "**/*.rs", "path": "/a/src" }),
            "/a/src".to_owned(),
        ),
        (
            "Grep",
            serde_json::json!({ "pattern": "fn", "path": "/a/src" }),
            "/a/src".to_owned(),
        ),
        (
            "Grep",
            serde_json::json!({ "pattern": "fn" }),
            wt.display().to_string(),
        ),
        (
            "Glob",
            serde_json::json!({ "pattern": "**/*.rs" }),
            wt.display().to_string(),
        ),
        (
            "Glob",
            serde_json::json!({ "pattern": "/a/**/*.rs" }),
            "/a/**/*.rs".to_owned(),
        ),
        (
            "Glob",
            serde_json::json!({ "pattern": "D:/a/**" }),
            "D:/a/**".to_owned(),
        ),
        (
            "Read",
            serde_json::json!({ "path": "/a/r.txt" }),
            String::new(),
        ),
        (
            "WebFetch",
            serde_json::json!({ "url": "https://example.invalid" }),
            String::new(),
        ),
        (
            "Bash",
            serde_json::json!({ "command": "cat /a/r.txt" }),
            String::new(),
        ),
    ];
    for (tool, input, want) in &cases {
        assert_eq!(
            &denial_path(tool, input, cwd),
            want,
            "{tool} {input}: the path is extracted per tool (s8h §8h.5)"
        );
    }
    // the pair, never a segment name
    let pair = |rel: &str| protected_segment(&format!("{}/{rel}", wt.display()), &wt);
    assert_eq!(pair(".config/git/x"), Some(".config/git".to_owned()));
    assert_eq!(pair(".Config/GIT/x"), Some(".config/git".to_owned()));
    assert_eq!(pair(".config/x"), None, ".config alone is not protected");
    assert_eq!(pair("git/x"), None, "git alone is not protected");
    assert_eq!(pair("src/.git/x"), Some(".git".to_owned()));
    // no extractable path: (tool, "") in the digest, and no note
    let sch = schema();
    let id = sch.id.clone();
    let json = sch.json;
    let d = serde_json::json!([{ "tool_name": "WebFetch", "tool_use_id": "toolu_f1",
        "tool_input": { "url": "https://example.invalid" } }]);
    let raw = frame_with_denials(&d, true);
    let c = classify(&evidence_with(
        Some(0),
        Some(&raw),
        &id,
        &json,
        &[],
        Some(&wt),
    ));
    assert_eq!(
        is_env(&c.result),
        Some(&entries_digest(&[("WebFetch", "")])),
        "a denial with no extractable path is (tool_name, \"\"): {:?}",
        c.result
    );
    assert!(c.notes.is_empty(), "and carries no note: {:?}", c.notes);
    // the digest helper agrees with the test's own over a de-duplicated pair
    let one = Denial {
        channel: DenialChannel::Result,
        tool_name: "Read".to_owned(),
        tool_use_id: Some("t1".to_owned()),
        path: "/a".to_owned(),
        decision_reason_code: None,
    };
    let twin = Denial {
        channel: DenialChannel::Event,
        decision_reason_code: Some("outside_reads_blocked".to_owned()),
        ..one.clone()
    };
    assert_eq!(
        denial_digest(&[one, twin]),
        entries_digest(&[("Read", "/a")])
    );
}

/// §8h.4 N24 (final consistency pass 2026-09-24): a `Spec` structured output with one criterion
/// whose `verification` is ABSENT, and one whose `verification` is `""` (or blank), each fails as
/// `StepError::Schema` — after the schema validation, which the type admits (AC-137). Control: the
/// same `Spec` with a non-empty `verification` on every criterion is `Ok`.
#[test]
fn a_spec_criterion_without_an_authored_verification_is_a_schema_failure() {
    let id = SchemaId::new("Spec");
    let json = rk_types::registry::EXPORTED
        .iter()
        .find(|(n, _)| *n == "Spec")
        .map(|(_, f)| f())
        .expect("PREMISE: Spec is registered in EXPORTED (s8h §8h.4)");
    let spec = |second: serde_json::Value| {
        let mut ac2 = serde_json::json!({ "id": "AC-2", "criterion": "the second holds" });
        if !second.is_null() {
            ac2["verification"] = second;
        }
        serde_json::json!({
            "type": "result", "subtype": "success", "is_error": false,
            "structured_output": {
                "story": "a story",
                "acceptance_criteria": [
                    { "id": "AC-1", "criterion": "the first holds", "verification": "cargo test -p x first" },
                    ac2
                ],
                "open_questions": [],
                "boundary": "the boundary"
            }
        })
        .to_string()
    };
    let ok = spec(serde_json::json!("cargo test -p x second"));
    let c = classify(&evidence(Some(0), Some(&ok), &id, &json));
    assert!(
        c.result.is_ok(),
        "CONTROL: a Spec whose every criterion carries an authored verification is Ok: {:?}",
        c.result
    );
    for (label, v) in [
        ("absent", serde_json::Value::Null),
        ("empty", serde_json::json!("")),
        ("blank", serde_json::json!("   ")),
    ] {
        let raw = spec(v);
        let c = classify(&evidence(Some(0), Some(&raw), &id, &json));
        match &c.result {
            Err(StepError::Schema { last_error, .. }) => assert!(
                last_error.contains("AC-2"),
                "{label}: the refusal names the criterion: {last_error}"
            ),
            other => panic!(
                "{label}: a Spec criterion without an authored verification must be \
                 StepError::Schema at ingest (s8h §8h.4; N24); got {other:?}"
            ),
        }
    }
}
