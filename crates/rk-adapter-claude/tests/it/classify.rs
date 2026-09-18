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
