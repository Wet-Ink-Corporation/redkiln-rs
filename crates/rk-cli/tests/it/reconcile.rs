//! **AC-402's negative** — `status` and `explain` are read-only and do not reconcile (ADR-0105
//! clause 8; AC-172).
//!
//! # 🛑 WHAT THIS MODULE COVERS, AND WHAT ITS NAME PROMISES
//!
//! The test below is named `ac402_status_and_explain_do_not_reconcile` — the spelling §7d.14 and
//! §7d.17's name loop fix, which may not be changed.
//!
//! *(Corrected 2026-09-20 at Slice 3, cycle 9. This read: **"At this cycle only the `status` arm
//! exists, because `crates/rk-cli/src/verbs/explain.rs` does not … until it does this test does not
//! cover the half of its own name."** It was true when written and is now false: `explain.rs`
//! landed at cycle 9 and **both arms exist**, each with its own recorder and its own trace so
//! neither can pass on the other's zero. The source scan's `else { continue }` escape — correct
//! while the file was absent, a silent pass the moment it was not — went with it.)*
//!
//! **What this test still does NOT cover is AC-402's positive half.**
//! `ac402_reconcile_called_on_run_answer_doctor` drives `doctor` in process and is **Slice 4's**,
//! where `verbs/doctor.rs` exists; in this slice it would be red for the stub's `3`. So Slice 3
//! discharges AC-402's NEGATIVE plus the `run` path, and the prescribed commit subject's
//! *"AC-402 three paths"* is a drift this slice RAISES rather than satisfies.

use rk_cli::verbs::{explain, status};

use crate::support::{RecordingReconciler, Trace, counting_store};
use crate::{code_lines, src_dir};

/// AC-402's negative, ADR-0105 clause 8. §7d.14: *"Red by one recorded reconcile call on either."*
///
/// **TWO INSTRUMENTS, AND THE SECOND IS NOT OPTIONAL.** §7d.14 declares this test's mutant as
/// *"a `reconcile_at_entry(Entry::Doctor)` call added to `status`"* — and that mutant **escapes the
/// recorder**. The recorder's only observable is a call that went *through the seam*; a direct
/// `rk_git::reconciled::reconcile_at_entry(..)` call added to the verb bypasses it entirely, records
/// nothing, and leaves `reconciles()` empty and this test green. A recorder alone would therefore be
/// a guard that cannot fail against the very mutant the runbook declares for it.
///
/// So the second instrument is a **comment-filtered source scan** over the verb's own file for the
/// symbol, asserting zero — and, per §7d.14's standing rule (*"Every in-process source-scan test is
/// self-controlled: it runs its scan over its `lint-controls/` file and asserts ≥ 1 hit **before**
/// asserting zero hits over `src/`"*), it is controlled FIRST against a file that legitimately names
/// the symbol. The control here is `crates/rk-cli/src/verbs/mod.rs`, which holds `RealReconciler` —
/// the same control-against-a-real-file form §7d.7 uses for the kernel's AC-436 lint (*"control:
/// `catalog.rs` names it"*), and it needs no nineteenth `lint-controls/` file, so §7d.13's
/// enumerated eighteen are undisturbed.
///
/// **`status` TAKES the reconciler it must not call.** A verb that *cannot* call the seam proves
/// nothing about a verb that chose not to. The type is the second guard and it is `rk_git`'s:
/// `Entry` is exactly `{ Run, Answer, Doctor }`, so there is no `Entry::Status` to pass.
#[test]
fn ac402_status_and_explain_do_not_reconcile() {
    // ---- (1) the seam: zero recorded calls on `status`. ----------------------------------------
    //
    // The verb is handed a REAL wrapper and worktree beside the recorder, so a `status` that wanted
    // to reconcile could. Without them the zero below would be true of every possible body.
    let repo_dir = crate::temp_repo();
    let discovered =
        rk_cli::repo::discover(Some(repo_dir.path())).expect("the temp repository discovers");
    let trace = Trace::new();
    let store = counting_store(&trace);
    let reconciler = RecordingReconciler::new(&trace);
    let mut out: Vec<u8> = Vec::new();

    let exit = status::run_with(
        &reconciler,
        &discovered.exec,
        &discovered.worktree,
        &store,
        &mut out,
    )
    .expect("status does not halt");
    assert_eq!(
        exit.code(),
        0,
        "a roll-up read over an empty store is not an error (ADR-0105 c8)"
    );
    assert!(
        trace.reconciles().is_empty(),
        "`status` reconciled: {:?}. ADR-0105 clause 8 — AC-172: `status` does not reconcile",
        trace.reconciles()
    );
    // The store WAS reached, which is what stops the assertion above being true of a `status` that
    // did nothing at all.
    assert!(
        !trace.steps().is_empty(),
        "`status` never touched the store, so the zero above is not evidence that it read without \
         reconciling — it is evidence that it did nothing"
    );

    // ---- (1b) THE SAME, FOR `explain`. ---------------------------------------------------------
    //
    // A second recorder and a second trace, so this arm cannot pass on `status`'s zero.
    //
    // 🛑 IT IS EXIT `5`, NOT `0`, AND THE STORE IS STILL REACHED. `explain`'s first act is the
    // item→run_id resolver, which is a store `query` — only `activity_committed` carries both
    // `item` and `run_id`, so the journal must be read before anything can be said about the item.
    // Over an empty store that query finds nothing and the verb refuses with `NoRecord`, which is
    // AC-432's outcome and not a failure to reach the seam. That is what keeps the zero below
    // falsifiable without planting a run record here: the trace shows the store WAS touched.
    let trace_x = Trace::new();
    let store_x = counting_store(&trace_x);
    let reconciler_x = RecordingReconciler::new(&trace_x);
    let mut out_x: Vec<u8> = Vec::new();

    let exit_x = explain::run_with(
        &reconciler_x,
        &discovered.exec,
        &discovered.worktree,
        &crate::common_dir(repo_dir.path()),
        &store_x,
        "no-such-item",
        &mut out_x,
    )
    .expect("explain does not halt");
    assert_eq!(
        exit_x.code(),
        5,
        "an item with no recorded resolution is `NoRecord` (AC-432); a different code here means \
         this arm is not measuring the read path it is supposed to be measuring"
    );
    assert!(
        trace_x.reconciles().is_empty(),
        "`explain` reconciled: {:?}. ADR-0105 clause 8 — `status` and `explain` are read-only and \
         do not reconcile",
        trace_x.reconciles()
    );
    assert!(
        !trace_x.steps().is_empty(),
        "`explain` never touched the store, so the zero above is not evidence that it read \
         without reconciling — it is evidence that it did nothing"
    );

    // ---- (2) the source scan the declared mutant needs. ----------------------------------------
    let needle = "reconcile_at_entry";

    // Self-controlled FIRST. `verbs/mod.rs` holds `RealReconciler`, which legitimately delegates to
    // the symbol; if this reads zero the scan is broken and the zero below would mean nothing.
    let control = src_dir().join("verbs").join("mod.rs");
    let control_src = std::fs::read_to_string(&control).expect("the control file is readable");
    let control_hits = code_lines(&control_src)
        .into_iter()
        .filter(|(_, line)| line.contains(needle))
        .count();
    assert!(
        control_hits >= 1,
        "the scan found no `{needle}` on a code line of {}, which legitimately names it — the scan \
         itself is broken and the zero below would be vacuous",
        control.display()
    );

    // And zero over the read verbs' own files.
    // 🛑 THE ABSENCE ESCAPE IS GONE, AND ITS REMOVAL IS PART OF THIS CYCLE'S WORK.
    //
    // This loop read `let Ok(src) = std::fs::read_to_string(&path) else { continue };` while
    // `explain.rs` did not exist — correct then, and a silent pass the moment it did: a DELETED
    // `explain.rs` would have skipped the scan and reported green. Both files exist now, so a read
    // failure is a real failure.
    for verb in ["status.rs", "explain.rs"] {
        let path = src_dir().join("verbs").join(verb);
        let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "{} must exist for this scan to mean anything: {e}",
                path.display()
            )
        });
        let hits: Vec<(usize, String)> = code_lines(&src)
            .into_iter()
            .filter(|(_, line)| line.contains(needle))
            .collect();
        assert!(
            hits.is_empty(),
            "{} names `{needle}` on a code line: {hits:?}. This is the instrument the declared \
             mutant needs — a DIRECT call bypasses the recorder above and leaves it empty",
            path.display()
        );
    }

    // ---- (3) the comment filter is CONTROLLED, not assumed. ------------------------------------
    //
    // The zero above is only meaningful if `code_lines` actually drops comments — and every one of
    // the three guards this stage has already shipped that could not fail was satisfied by the prose
    // explaining it. `status.rs`'s own header names `reconcile_at_entry` several times in exactly
    // that way, so this asserts the mechanism directly: a source in which the symbol appears ONLY on
    // comment lines must scan to zero, and the same source with one code line must not.
    let only_comments = format!("//! a header naming {needle}\n// and a line comment: {needle}\n");
    assert_eq!(
        code_lines(&only_comments)
            .into_iter()
            .filter(|(_, line)| line.contains(needle))
            .count(),
        0,
        "the comment filter counted a comment — every zero above is then satisfiable by prose"
    );
    let with_code = format!("{only_comments}let _ = {needle}(exec, repo, entry);\n");
    assert_eq!(
        code_lines(&with_code)
            .into_iter()
            .filter(|(_, line)| line.contains(needle))
            .count(),
        1,
        "the filter dropped a real code line, so the zeros above would be unfalsifiable"
    );
}
