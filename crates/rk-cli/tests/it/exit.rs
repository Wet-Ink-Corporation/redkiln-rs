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

/// **AC-438's IN-PROCESS ELEVEN-ROW ARM** — §7d.4's *"the eleven `StepError` rows are driven
/// in-process through the verb function with a `FakeHarness::scripted` returning each variant"*.
///
/// The three arms above pin the TABLE (its snapshot, its distinctness, the exhaustive `From`). None
/// of them drives a run. This one pins that the verb **uses** the table: each variant goes through
/// `run_with` once and the returned `Exit` is that variant's row.
///
/// # 🛑 WHY THE CLASS IS READ OFF THE JOURNAL AND NOT OFF `resume`'s ERROR
///
/// `Runtime::run_activity` carries a step's failure as a **value** in its report — *"A step's own
/// failure is a value in the report (AC-032), never an error of this function"* — and
/// `replay::dispatch_through` then turns an uncommitted report into
/// `ReplayError::NotCommitted { step_id, activity_id_at_version }`, which has **two `String`
/// members and no `StepError`**. So all eleven arrive at the CLI as the *same* error value and are
/// indistinguishable from it alone.
///
/// The class survives in exactly one place: the row the harness appended before returning, through
/// `StepError::event()` → `fakes.rs::event_for`. That map is **bijective over the eleven** —
/// `StepFailed { class }` covers five, `StepRefused { class }` four, plus `StepCancelled` and
/// `RateLimited` — and both class enums are CLOSED and `Deserialize`, so step (10) matches them
/// exhaustively and a twelfth class is a compile error rather than a silent `ConfigError`.
///
/// # 🛑 ONE SCRIPT ELEMENT PER ROW, AND THAT IS ENOUGH FOR ALL ELEVEN
///
/// `pack-min` declares `[retry] attempts_cap = 1` (AC-029: declared, never defaulted). `retry::decide`
/// checks the cap **before** the class, and `fsm::next_attempt` returns 1 on a fresh journal, so
/// `1 >= 1` is `CapReached` for every redispatching class and `authorises_redispatch` is false. No
/// variant re-dispatches, so no script runs dry into `Transport { detail: "script exhausted" }` —
/// which §7d.5 names as the way *"a retried class would otherwise land on row 10 for a reason
/// unrelated to the mapping"*.
///
/// # What this test does NOT claim
///
/// **Row 10 is not discriminated from an exhausted script.** `event_for` journals only
/// `{ class, status }`; the `TransportKind` and the `detail` are not written and are not on the
/// returned `Exit`. A scripted `Transport` and a dry script are observationally identical here. The
/// `dispatches() == 1` assertion bounds it (a dry script needs a SECOND dispatch, which cap = 1
/// forbids) and nothing stronger is claimed.
///
/// **Row 70 is not one of the eleven.** `Exit::PoisonedLockHalt` is never an `Ok` of a verb: it
/// arrives as `Err(Halt::PoisonedLock { .. })` and becomes 70 only through `code_for`, which is the
/// arm asserted at the foot. Its real instrument is §7d.10's
/// `adr0118_poisoned_store_lock_halts_run_exit_70` over a genuinely poisoned lock.
///
/// Red by any variant returning a code other than its row — and, before step (10) read the class at
/// all, red on all eleven at once with `Ok(Exit::ConfigError)`.
#[test]
fn ac438_table_driven_exit_codes_over_fixtures() {
    // The eleven expected rows, in `rk-types`' own declaration order — the order
    // `one_of_every_step_error` builds. Written out, because the whole content of this test is that
    // the verb agrees with the table; deriving them through the same `From` impl the verb uses
    // would assert nothing.
    let expected = [
        Exit::Transport,
        Exit::Schema,
        Exit::Timeout,
        Exit::Cancelled,
        Exit::NonZero,
        Exit::RateLimited,
        Exit::NotAdopted,
        Exit::NotIsolated,
        Exit::Platform,
        Exit::Unsupported,
        Exit::Overage,
    ];
    let variants = one_of_every_step_error();
    assert_eq!(
        variants.len(),
        expected.len(),
        "the eleven variants and the eleven expected rows are not the same length, so some row \
         below is unasserted"
    );

    for (variant, want) in variants.into_iter().zip(expected) {
        let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
        let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
        // Step (8)'s BINDING, which cycle 6 will perform through the real `Binder` and which the
        // in-process cases supply themselves. It is not optional decoration: `RunInputs::specs()`
        // is what `FsmContext::with_activations` is fed, so WITHOUT a binding the FSM has no
        // `activity_id@version` for the node and the runtime boundary refuses before any dispatch
        // with "`emit-report` is not an `activity_id@version`" (ADR-0097 decision 2) — exit 3 for a
        // reason that has nothing to do with the exit table. Measured, before the binding was added.
        let (inputs, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
        let inputs = inputs.with_spec(
            "emit-report",
            rk_kernel::replay::RecordedSpec::supplied("emit-report@1", emit_report_spec()),
        );

        let trace = crate::support::Trace::new();
        let store = crate::support::counting_store(&trace);
        let harness = std::sync::Arc::new(crate::support::ScriptedHarness::scripted(vec![Err(
            variant.clone(),
        )]));
        let ports = crate::support::ports(
            std::sync::Arc::clone(&harness) as std::sync::Arc<dyn rk_ports::harness::Harness>,
            &store,
        );

        // The token is NEVER fired. `FakeHarness::dispatch` checks its own token BEFORE the script
        // and would substitute `Cancelled { by: Operator }` for whatever was scripted; `run_with`
        // would return 130 before the dispatch. Either would make this row measure the token.
        let token = rk_ports::cancel::CancelToken::new();
        let mut supply = crate::support::any_spec("emit-report@1", emit_report_spec());

        let got = rk_cli::verbs::run::run_with(
            &resolved,
            &inputs,
            crate::support::FIXED_NOW,
            &tree_hash,
            ports,
            &crate::support::clean_env(),
            &mut supply,
            &token,
        );

        assert_eq!(
            got,
            Ok(want),
            "{variant:?} must return {want:?} ({}); step (10) either collapsed the class or read \
             the wrong row",
            want.code()
        );

        // The port was REACHED exactly once. Without this, a row could land on the right code
        // because the run refused BEFORE the dispatch — a missing `results_at`, an unbound spec —
        // and every one of those refusals is exit 3 today, so the wrong reason is reachable.
        assert_eq!(
            harness.dispatches(),
            1,
            "{variant:?} did not reach the harness exactly once ({} dispatches), so the exit above \
             is not a reading of a dispatch",
            harness.dispatches()
        );

        // And the harness JOURNALLED the class beside returning it.
        //
        // 🛑 THIS IS NO LONGER WHERE THE EXIT COMES FROM, AND THE DIFFERENCE IS THE POINT.
        //
        // Until Slice 3 cycle 10 the exit above WAS read off this row: `NotCommitted` carried two
        // `String`s and `exit_for_uncommitted` recovered the class by re-reading the journal.
        // Mechanism B carries the `StepError` on the refusal itself, so the two are now
        // independent — the exit comes from `impl From<&StepError> for Exit` and this assertion is
        // about the FAKE's own bridge, `FakeHarness::dispatch`'s
        // `if let Err(error) = &result { store.append(event_for(error)) }`.
        //
        // It is kept because it is exactly what separates the fake from the real adapter:
        // `ClaudeHarness` appends one `dispatch_completed` carrying `cancelled_by` and no class
        // member, which is why the journal round trip recovered only row `13` and why mechanism B
        // was needed at all. A reader comparing this arm with the binary arm needs that visible.
        let appended: Vec<rk_types::event_kind::EventKind> = trace
            .steps()
            .into_iter()
            .filter_map(|s| match s {
                crate::support::Step::Append(kind) => Some(kind),
                crate::support::Step::Reconcile(_) | crate::support::Step::Query => None,
            })
            .collect();
        let want_kind = error_row_kind(&variant);
        assert!(
            appended.contains(&want_kind),
            "{variant:?} appended no {want_kind:?} row. `FakeHarness` journals every `Err` it \
             returns, and that bridge is what this arm rides; the EXIT itself now comes from the \
             class `NotCommitted` carries. Rows appended, in order: {appended:?}"
        );
    }
}

/// The spec the fixture's one activity binds to. `emit-report` provides `ImplReport`.
fn emit_report_spec() -> rk_types::activity::ActivitySpec {
    rk_types::activity::ActivitySpec {
        card_digest: rk_types::digest::sha256(b"pack-min/cards/implementer.json"),
        tools: vec!["StructuredOutput".to_owned()],
        schema: rk_types::typed::SchemaId::new("ImplReport"),
        timeout_secs: 900,
        inputs: Vec::new(),
    }
}

/// The event kind `StepError::event()` mints for a variant — the row whose class step (10) reads.
fn error_row_kind(error: &StepError) -> rk_types::event_kind::EventKind {
    use rk_types::event::ErrorEvent;
    use rk_types::event_kind::EventKind;
    match error.event() {
        ErrorEvent::StepFailed { .. } => EventKind::StepFailed,
        ErrorEvent::StepCancelled => EventKind::StepCancelled,
        ErrorEvent::RateLimited => EventKind::RateLimited,
        ErrorEvent::StepRefused { .. } => EventKind::StepRefused,
    }
}

/// **Row 70, and the only way it is reached.** Not an `Ok` of any verb: a poisoned lock is a
/// [`Halt`], and `code_for` is the one place the table's `70` is produced.
#[test]
fn ac438_poisoned_lock_halt_is_seventy_through_code_for() {
    let halt = rk_cli::exit::Halt::PoisonedLock {
        lock: "store",
        verb: "the run",
    };
    assert_eq!(
        rk_cli::code_for(&Err(halt)),
        70,
        "a poisoned-lock halt is exit 70 (ADR-0118 clause 6; §7d.4's table)"
    );
    assert!(
        ALL.contains(&Exit::PoisonedLockHalt),
        "row 70 is absent from the closed table it belongs to"
    );
}
/// One `[[step]]`, for a walk this test transcribes itself.
fn adr0144_step(
    id: &str,
    kind: rk_kernel::catalog::StepKind,
    run: Option<&str>,
    role: Option<&str>,
) -> rk_kernel::catalog::Step {
    rk_kernel::catalog::Step {
        id: id.to_owned(),
        kind,
        run: run.map(ToOwned::to_owned),
        output_type: None,
        role: role.map(ToOwned::to_owned),
        when: None,
        expr: None,
        for_each: None,
        retry_max: None,
        repeat_max: None,
        memo_declared: None,
        question: None,
    }
}

/// **ADR-0144 END TO END: a step failing after a committed dispatch exits the CLASS's row, not 3.**
///
/// 🛑 **This exists because an adversarial pass measured that ADR-0144's kernel guard covered ONE of
/// the three edits the record makes.** `post_commit_failure.rs` asserts over `ActivityReport` and
/// nothing else — the report may carry `step_error` and every caller may ignore it. Measured on the
/// tree as ADR-0144 first shipped it:
///
/// | mutation | workspace | exit |
/// | --- | --- | --- |
/// | delete `dispatch_through`'s six-line `step_error()` check | **GREEN** | **0** |
/// | the `run.rs` arm returns `Exit::ConfigError` (clause 5's forbidden shape) | **GREEN** | **3** |
/// | the pre-ADR-0144 `Failed` one-liner in `step.rs` | RED (the kernel guard) | 0 |
///
/// Two of the three edits were protected by nothing but the comment above them. `StepFailedAfterCommit`
/// appeared on four source lines and **zero test lines**.
///
/// ⚠️ **And `post_commit_failure.rs`'s own header claimed this test already existed** — *"The mapping
/// … lives in `rk-cli` and is asserted there"*. It did not. That sentence is corrected in the same
/// change that makes it true, which is the only honest order.
///
/// # What makes it non-vacuous
///
/// The dispatch must really COMMIT — `harness.dispatches() == 1` is asserted, so an exit produced by
/// a run that never dispatched cannot read as a pass. `toolchain:publish` is one of ADR-0135's 29
/// refused verbs, so the failing step reaches no port and the class is `Unsupported` → row **19**,
/// a pre-existing row. **No row is added, moved or renumbered** — clause 6.
#[test]
fn adr0144_post_commit_step_failure_exits_the_class_row_not_three() {
    use std::sync::Arc;

    use rk_kernel::catalog::StepKind;

    let steps = vec![
        adr0144_step("draft", StepKind::Agent, None, Some("implementer")),
        adr0144_step(
            "publish-it",
            StepKind::Tool,
            Some("toolchain:publish"),
            None,
        ),
    ];
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let resolved = crate::support::resolved_over_pack_min(repo.path(), "bug-1");
    let (inputs, tree_hash) = crate::support::inputs_over_pack_min(&resolved);
    let inputs = inputs.with_spec(
        "emit-report",
        rk_kernel::replay::RecordedSpec::with_steps(
            "emit-report@1",
            crate::support::any_activity_spec(),
            steps.clone(),
        ),
    );
    let trace = crate::support::Trace::new();
    let store = crate::support::counting_store(&trace);
    let harness = Arc::new(crate::support::ScriptedHarness::scripted(vec![Ok(
        crate::support::impl_report(),
    )]));
    let ports = crate::support::ports(
        Arc::clone(&harness) as Arc<dyn rk_ports::harness::Harness>,
        &store,
    );
    let token = rk_ports::cancel::CancelToken::new();
    let spec2 = crate::support::any_activity_spec();
    let steps2 = steps.clone();
    let mut supply = move |_node: &str| {
        Ok(rk_kernel::replay::RecordedSpec::with_steps(
            "emit-report@1",
            spec2.clone(),
            steps2.clone(),
        ))
    };

    let got = rk_cli::verbs::run::run_with(
        &resolved,
        &inputs,
        crate::support::FIXED_NOW,
        &tree_hash,
        ports,
        &crate::support::clean_env(),
        &mut supply,
        &token,
    );

    assert_eq!(
        got,
        Ok(Exit::Unsupported),
        "ADR-0144 clauses 3, 5 and 6. `Ok(Success)` here is the defect the record closed — a run \
         that executed NOTHING after its dispatch reporting success, which is ADR-0135's by-name \
         refusal voided at the last step. `Ok(ConfigError)` is the catch-all clause 5 forbids, and \
         is what `run.rs` returns for every variant it is not explicitly told about. Got {got:?}"
    );
    assert_eq!(
        harness.dispatches(),
        1,
        "the exit above is not a reading of a COMMITTED dispatch — with zero dispatches the walk \
         never reached the failing step and this assertion is about nothing"
    );
}
