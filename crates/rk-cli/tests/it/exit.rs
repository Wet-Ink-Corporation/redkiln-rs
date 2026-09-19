//! **The closed exit table** — AC-438 as amended 2026-09-18 (D-01, D-12), `s7d` §7d.4.
//!
//! Three of AC-438's five instruments land here; the other two are later slices'. The
//! table-driven CLI test over fixtures × expected code is §7d.5's (Slice 3, and it grows its
//! doctor-fixture arm at Slice 4), and `ac438_doctor_error_rule_exits_config_error` is §7d.8's.
//!
//! The THEN this module pins: *"its exit code comes from a closed, snapshot-pinned table in which
//! success, configuration error …, findings-block and each runtime error class are distinct values;
//! a new `StepError` variant with no mapping fails to compile."*

use std::collections::BTreeSet;

use rk_types::digest::sha256;
use rk_types::error::{CancelledBy, IsolationSurface, StepError, TransportKind};

use rk_cli::exit::{ALL, Exit};

use crate::{assert_no_block_comment, code_lines, rs_files, src_dir};

/// The seven rows of §7d.4's table that no `StepError` variant maps to.
///
/// Written out rather than derived, because this is the half of `ac438_exit_codes_distinct` that
/// would be circular if both sides were computed from the same source. The other half — the eleven
/// `StepError` codes — IS computed, by mapping one representative value per variant through the real
/// `From` impl.
const NON_STEP_ERROR_CODES: [i32; 7] = [0, 2, 3, 4, 5, 70, 130];

/// The eleven `StepError` variant heads, in `rk-types`' own declaration order.
const STEP_ERROR_HEADS: [&str; 11] = [
    "StepError::Transport",
    "StepError::Schema",
    "StepError::Timeout",
    "StepError::Cancelled",
    "StepError::NonZero",
    "StepError::RateLimited",
    "StepError::NotAdopted",
    "StepError::NotIsolated",
    "StepError::Platform",
    "StepError::Unsupported",
    "StepError::Overage",
];

/// One representative value per `StepError` variant.
///
/// **Constructed, not listed.** `ac438_exit_codes_distinct`'s second conjunct is *"the
/// non-`StepError` set being anything but `{0, 2, 3, 4, 5, 70, 130}`"* — and a test that wrote out
/// both sides would assert nothing about `impl From<&StepError> for Exit`. These eleven values go
/// through the real impl, so the eleven codes are its IMAGE and the complement is measured.
fn one_of_every_step_error() -> Vec<StepError> {
    vec![
        StepError::Transport {
            kind: TransportKind::Spawn,
            detail: String::new(),
        },
        StepError::Schema {
            attempts: 1,
            last_error: String::new(),
        },
        StepError::Timeout { after_secs: 1 },
        StepError::Cancelled {
            by: CancelledBy::Signal,
        },
        StepError::NonZero {
            code: None,
            stderr_digest: sha256(b""),
        },
        StepError::RateLimited { resets_at: None },
        StepError::NotAdopted {
            expected_agent: String::new(),
            observed: String::new(),
        },
        StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest: sha256(b""),
        },
        StepError::Platform {
            status: 529,
            retry_after: None,
        },
        StepError::Unsupported {
            capability: String::new(),
        },
        StepError::Overage { reason: None },
    ]
}

/// AC-438's *"a committed snapshot of the exit-code table"*.
///
/// Red by any change to `ALL`'s rendered `code variant` lines against the committed `insta`
/// snapshot; a weaker red snapshots the count (§7d.14).
///
/// Rendered into a NAMED LOCAL and asserted with no custom snapshot name. Both are load-bearing and
/// neither is style. `insta` records the caller expression verbatim in the snapshot's own
/// `expression:` header, and Checkpoint 7d greps the WHOLE file — header included — for
/// `gate.?open`, refusing a table that grew a gate row (D-01: a run that opens a gate exits `0`).
/// An inlined `format!` carrying the word would put it in the header. The default name is what makes
/// the file match the block's `*ac438_exit_table_snapshot*.snap` glob.
#[test]
fn ac438_exit_table_snapshot() {
    let table = ALL
        .iter()
        .map(|e| format!("{} {}", e.code(), e.name()))
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(table);
}

/// AC-438's *"distinct values"*.
///
/// Red by two variants sharing a code, **and** by the non-`StepError` set being anything but
/// `{0, 2, 3, 4, 5, 70, 130}` (§7d.14); a weaker red checks distinctness only.
#[test]
fn ac438_exit_codes_distinct() {
    assert_eq!(ALL.len(), 18, "§7d.4's table has eighteen rows");

    let all_codes: BTreeSet<i32> = ALL.iter().map(|e| e.code()).collect();
    assert_eq!(
        all_codes.len(),
        ALL.len(),
        "two rows of Exit::ALL share a code — AC-438 requires distinct values. Rendered table:\n{}",
        ALL.iter()
            .map(|e| format!("{} {}", e.code(), e.name()))
            .collect::<Vec<_>>()
            .join("\n")
    );

    let image: BTreeSet<i32> = one_of_every_step_error()
        .iter()
        .map(|e| Exit::from(e).code())
        .collect();
    assert_eq!(
        image.len(),
        11,
        "the eleven StepError variants map onto {} distinct codes, not eleven — two runtime error \
         classes share a row",
        image.len()
    );

    let expected_non_step: BTreeSet<i32> = NON_STEP_ERROR_CODES.into_iter().collect();
    let measured_non_step: BTreeSet<i32> = all_codes.difference(&image).copied().collect();
    assert_eq!(
        measured_non_step, expected_non_step,
        "the rows of Exit::ALL that no StepError maps to are not {{0, 2, 3, 4, 5, 70, 130}}"
    );
    assert!(
        image.is_subset(&all_codes),
        "a StepError maps to a code that is not a row of Exit::ALL"
    );
}

/// AC-438's *"a new `StepError` variant with no mapping fails to compile"*.
///
/// **Not a `trybuild` case** (§7d.4): the exhaustiveness is the compiler's, on the real `match`.
/// What the compiler cannot tell us is whether that `match` still HAS the property — a `_ =>` arm,
/// or a bare-identifier catch-all like `other =>`, would make a twelfth variant compile silently,
/// which is exactly what the THEN forbids. So this is a source assertion over the impl's own region,
/// and its declared mutants are `_ => Exit::Transport` and `other => Exit::Transport` in place of an
/// arm (§7d.4 names both).
///
/// The region is located by walking `src/` for the impl header rather than by naming a file: §7d.1
/// leaves module file names inside `rk-cli` to the implementer, and a scan hard-coded to `exit.rs`
/// would go quietly vacuous the day the impl moved. Zero or more than one carrier is a loud failure.
#[test]
fn ac438_step_error_mapping_is_exhaustive() {
    const HEADER: &str = "impl From<&StepError> for Exit";

    let src = src_dir();
    let carriers: Vec<_> = rs_files(&src)
        .into_iter()
        .filter(|p| {
            let text = std::fs::read_to_string(p).expect("a source file");
            code_lines(&text).iter().any(|(_, l)| l.contains(HEADER))
        })
        .collect();
    assert_eq!(
        carriers.len(),
        1,
        "expected exactly one file under {} carrying `{HEADER}`, found {}: {:?} — a scan that \
         cannot find its region reads as a pass",
        src.display(),
        carriers.len(),
        carriers
    );

    let text = std::fs::read_to_string(&carriers[0]).expect("the carrier");
    let region = impl_region(&text, HEADER);
    assert_no_block_comment(&region, "the From<&StepError> for Exit impl");

    let lines: Vec<String> = code_lines(&region).into_iter().map(|(_, l)| l).collect();

    for head in STEP_ERROR_HEADS {
        let hits = lines.iter().filter(|l| l.contains(head)).count();
        assert_eq!(
            hits, 1,
            "`{head}` appears {hits} times as code in the impl region, not exactly once — eleven \
             arms, one per variant, and no arm named twice"
        );
    }

    for line in &lines {
        let t = line.trim_start();
        if let Some((head, _)) = t.split_once("=>") {
            let head = head.trim();
            let is_catch_all = head == "_"
                || (!head.is_empty()
                    && head
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'));
            assert!(
                !is_catch_all,
                "the impl carries a catch-all arm `{head} =>` — a twelfth StepError variant would \
                 compile silently against it, which is what AC-438's THEN forbids"
            );
        }
    }
}

/// The `{ … }` body of the `impl` whose header line contains `header`, braces balanced.
///
/// The header is located on a **code** line, never by a raw `str::find`. The carrier file's own
/// module documentation quotes the impl header — deliberately, and it is the standing control for
/// this function: a locator that read the raw text would anchor on the prose, extract whatever
/// braces followed it, and report zero arms over a correct impl. That is the guard-satisfied-by-its-
/// own-prose shape, and it is what this locator was written wrong as and caught by, once.
fn impl_region(text: &str, header: &str) -> String {
    let start = text
        .match_indices(header)
        .map(|(at, _)| at)
        .find(|at| {
            let line_start = text[..*at].rfind('\n').map_or(0, |nl| nl + 1);
            let trimmed = text[line_start..*at].trim_start();
            !(trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*'))
        })
        .unwrap_or_else(|| panic!("`{header}` is not on any code line of the carrier file"));
    let open = start
        + text[start..]
            .find('{')
            .unwrap_or_else(|| panic!("`{header}` has no opening brace"));
    let mut depth = 0_i32;
    for (i, c) in text[open..].char_indices() {
        if c == '{' {
            depth += 1;
        } else if c == '}' {
            depth -= 1;
            if depth == 0 {
                return text[open..=open + i].to_owned();
            }
        }
    }
    panic!("`{header}`'s braces never balance — the region cannot be read");
}
