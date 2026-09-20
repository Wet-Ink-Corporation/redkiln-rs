//! **AC-402's negative** — `status` and `explain` are read-only and do not reconcile (ADR-0105
//! clause 8; AC-172).
//!
//! # 🛑 WHAT THIS MODULE COVERS AT CYCLE 3, AND WHAT ITS NAME PROMISES
//!
//! The test below is named `ac402_status_and_explain_do_not_reconcile` — the spelling §7d.14 and
//! §7d.17's name loop fix, which may not be changed. At this cycle **only the `status` arm exists**,
//! because `crates/rk-cli/src/verbs/explain.rs` does not. The `explain` arm lands with that file
//! later in this slice, and until it does **this test does not cover the half of its own name**.
//! That sentence is here rather than implied, because a test whose name claims more than it asserts
//! is the shape this stage has already shipped three times.

use rk_cli::verbs::status;

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
    for verb in ["status.rs", "explain.rs"] {
        let path = src_dir().join("verbs").join(verb);
        let Ok(src) = std::fs::read_to_string(&path) else {
            // `explain.rs` does not exist at this cycle; see the module header.
            continue;
        };
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
