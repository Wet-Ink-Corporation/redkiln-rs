//! **A walk-ending step failure reaches stderr and the journal** (ADR-0205, proposed; D21-177; plan
//! `spikes/plans/s8-repro-aware-implement-2026-11-08.md` section 4, Decision D7 option J1; test 16).
//!
//! `run`'s `StepFailedAfterCommit` arm named neither the class nor its capability text, and a
//! non-agent step's failure appended no journal row, so the operator's 2026-11-08 run of
//! `rk-fix-doctor-margin-2` stopped at exit 19 with a generic sentence only.

use std::path::Path;

use rk_types::event_kind::EventKind;

use crate::support::{PackVariant, journal_rows, pack_min_repo};

/// `pack-min`'s `emit-report`, with a `tool` step naming a verb no registry row routes, placed
/// after the agent step's commit and before the `emit`.
const EMIT_REPORT_WITH_UNROUTED_VERB: &str = r#"[activity]
id           = "emit-report"
version      = 1
stage        = "implement"
provides     = ["ImplReport"]
requires     = []
derived_from = []
deviation_ok = []
tags         = ["kind:bug", "kind:story"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"
budget       = { attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }

[[step]]
id      = "check"
kind    = "agent"
role    = "implementer"
outputs = "value:ImplReport"

[[step]]
id           = "after"
kind         = "tool"
run          = "toolchain:unrouted_verb"
timeout_secs = 60

[[step]]
id        = "emit"
kind      = "emit"
artifacts = { ImplReport = "${check.outputs}" }
"#;

/// A transcript whose result frame carries a valid `ImplReport`: `schema-invalid-structured-output`'s
/// frames, with its invalid payload replaced.
fn valid_transcript(dir: &Path) -> std::path::PathBuf {
    let frames = std::fs::read_to_string(
        crate::fixtures_dir()
            .join("exit")
            .join("schema-invalid-structured-output")
            .join("frames.jsonl"),
    )
    .expect("the schema-invalid transcript reads");
    let invalid = r#""structured_output":{"item":7}"#;
    assert!(frames.contains(invalid), "PREMISE: the payload to replace");
    let valid = frames.replace(
        invalid,
        r#""structured_output":{"status":"implemented","files":[],"summary":"s","tests_added":[]}"#,
    );
    let path = dir.join("frames.jsonl");
    std::fs::write(&path, valid).expect("the transcript writes");
    path
}

/// **Test 16.** A tool step's refusal after an agent commit: stderr names the class with its
/// capability text, and the store holds the `step_refused` row carrying the step. Red before: the
/// generic sentence only, and no row.
#[test]
fn a_walk_ending_refusal_names_its_capability_on_stderr() {
    let repo = pack_min_repo(PackVariant::Base);
    let root = repo.path();
    std::fs::write(
        root.join(".redkiln")
            .join("catalog")
            .join("emit-report.toml"),
        EMIT_REPORT_WITH_UNROUTED_VERB,
    )
    .expect("the activity installs");
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let transcript = valid_transcript(scratch.path());
    let mode = format!("replay:{}", transcript.display());

    let out = crate::run::spawn_run_in_mode(root, "bug-1", None, &mode);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.code() == Some(19) && stderr.contains("committed, and the later step"),
        "PREMISE: the agent step committed and the tool step failed after it (exit {:?}); \
         stderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("the step's class: Unsupported { capability: \"")
            && stderr.contains("unrouted_verb"),
        "stderr names the class, its capability text and the verb; stderr:\n{stderr}"
    );
    let rows = journal_rows(root, "bug-1");
    let refused: Vec<_> = rows
        .iter()
        .filter(|e| e.kind == EventKind::StepRefused)
        .collect();
    assert_eq!(
        refused.len(),
        1,
        "one `step_refused` row: {:?}",
        rows.iter().map(|e| e.kind).collect::<Vec<_>>()
    );
    let row = refused.first().expect("one row");
    assert_eq!(
        row.payload.get("step_id"),
        Some(&serde_json::json!("after"))
    );
    assert_eq!(
        row.payload.get("activity_id_at_version"),
        Some(&serde_json::json!("emit-report@1"))
    );
}
