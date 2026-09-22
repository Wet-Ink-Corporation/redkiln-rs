//! **ADR-0141 clause 4's OWED guard: the composition root always transcribes a step list.**
//!
//! ADR-0141 decision 2 gives `RecordedSpec` two constructors — `supplied` (yields `steps: None`,
//! *this caller transcribed none*) and `with_steps` (yields `Some`) — and decision 4 records the
//! hole this module closes, in terms:
//!
//! > **The production path must always transcribe, and that is currently a convention rather than a
//! > guard.** Measured: **one** `with_steps` call site (`rk-cli`'s `bind_nodes`) against **16**
//! > `supplied`, all in suites. Nothing fails if a future production site reaches for `supplied`.
//! > **A guard asserting that the composition root transcribes is OWED.**
//!
//! # What goes wrong without it, and why it is silent
//!
//! `steps: None` is a legitimate value — it is what the kernel's own machinery suites supply, and
//! `run_activity_steps` falls through to the pre-`s7g` activation dispatch on it. That is exactly
//! what makes a production `supplied` dangerous: it does not crash, it does not warn, and it does
//! not fail a test. The run simply stops walking steps and goes back to dispatching whole
//! activations, which is the defect `s7g` exists to end — reintroduced by a one-word edit, on a
//! path no suite exercises. **A wrong `None` here is indistinguishable from a correct one.**
//!
//! # 🛑 The comment filter is LOAD-BEARING, and the control for it is LIVE rather than synthetic
//!
//! `crates/rk-cli/src/verbs/run.rs` carries the forbidden spelling **today**, in `run_with`'s
//! docstring, quoting §7d.5: *"the fake-harness tests hand a supplier over
//! `RecordedSpec::supplied(..)` values"*. That sentence is correct and must stay. So a naive
//! `grep -rn 'RecordedSpec::supplied' crates/rk-cli/src` is **RED on a correct tree right now**, and
//! any version of this guard that did not strip comments would either fire falsely or be "fixed" by
//! deleting true prose — which is the *"do not widen the guard, and do not write the hit"* rule
//! (Halt 7) in its most tempting form.
//!
//! [`the_comment_filter_is_doing_real_work_on_run_rs`] asserts that situation directly: the raw
//! bytes contain the spelling and the code lines do not. It is a **positive control over the live
//! tree**, not a fixture — so if the docstring is ever reworded away, the control fails loudly and
//! says the guard below has stopped being exercised, rather than quietly becoming decorative.
//!
//! # What this guard does NOT close, stated rather than left to be discovered
//!
//! It is a **source lint over one spelling**, and this stage has already paid for believing such a
//! thing is a capability check (`conformance.rs`'s check-arm lint was defeated by a `&Ports`
//! parameter and by an aliased import). **The limits below are MEASURED, not assumed** — an
//! unverified limitation is as much a claim as an unverified guarantee, and the first draft of this
//! paragraph got one of them wrong in the guard's own favour.
//!
//! **An alias is caught in one form and not the other**, and the boundary is exactly where the
//! two-sidedness reaches:
//!
//! - `use …RecordedSpec as RS;` **replacing** the sole `with_steps` call → **CAUGHT**
//!   [measured], but by the REQUIRED half noticing the transcription vanished, *not* by detecting
//!   `RS::supplied`. The guard is right for the wrong reason, which is worth knowing.
//! - an aliased `supplied` **added alongside** a surviving `with_steps` → **NOT CAUGHT**
//!   [measured: a second production binding compiled and the guard stayed green]. **This is the
//!   shape a real regression takes**, and it is the open hole.
//!
//! Also not caught: a **helper one call deep** — a private `fn bind_one(..) -> RecordedSpec` that
//! calls `supplied` and is itself called from `bind_nodes` — and a future constructor yielding
//! `None` under a third name (the `replay.rs` anchor fires if `RecordedSpec`'s two-constructor shape
//! changes, but not if a third is added beside them).
//!
//! The honest closure for all of these is a **behavioural** assertion that a `RecordedSpec` built by
//! the production binder carries `Some`, and it is not written here because `bind_nodes` is private
//! and its inputs (`Resolved`, a live `Binder`, a run-owned worktree) are not constructible from an
//! integration test. That remains OWED, and it is recorded in `runbook/PROGRESS.md` §1's Step 7g
//! row rather than implied to be done by this file.

use crate::{rs_files, src_dir};

/// One file's CODE lines as `(1-indexed line, content)` — **comments stripped with the tree's own
/// block-aware stripper**, `rk_kernel::kernel_denylist::strip_comments`.
///
/// 🛑 **Not `crate::code_lines`, and the reason is measured rather than stylistic.** That helper is
/// line-based and its companion `assert_no_block_comment` exists to stop it reading a `/*` region it
/// cannot parse — a rule `rk-cli`'s own lints keep by scanning NARROW REGIONS. This guard scans two
/// whole crates, and both carry block comments today: `rk-cli/src/adapters.rs` and
/// `rk-kernel/src/replay.rs` each fired that assertion when this module first used it. Refusing the
/// scan would make the guard unwritable; widening `assert_no_block_comment` across crates would be
/// asserting one crate's house style over another's. `conformance.rs` reached the same conclusion
/// for the same reason: *"Three lints read whole directories of other people's code, so the stripper
/// has to be the real one."*
fn code_lines(src: &str) -> Vec<(usize, String)> {
    let mut in_block = false;
    src.lines()
        .enumerate()
        .map(|(i, l)| {
            (
                i + 1,
                rk_kernel::kernel_denylist::strip_comments(l, &mut in_block),
            )
        })
        .filter(|(_, l)| !l.trim().is_empty())
        .collect()
}

/// The constructor that yields `steps: None`. **Forbidden on any production code line.**
const FORBIDDEN: &str = "RecordedSpec::supplied";

/// The constructor that transcribes. **At least one production code line must carry it**, or the
/// guard above is satisfied by a tree that binds no steps at all.
const REQUIRED: &str = "RecordedSpec::with_steps";

/// `bind_nodes` is the composition root's binding pass and the site the property is about. Anchoring
/// on it means a rename or a deletion fails here rather than silently emptying the scan.
///
/// ⚠️ **The trailing `(` is load-bearing and was added after a mutant survived without it.** As
/// `"fn bind_nodes"` this anchor matched `fn bind_nodes_renamed(` — a substring anchor binds to any
/// longer name that starts with it, so renaming the binder at both call sites compiled AND kept the
/// guard green. That is the same family as the decoy-`PortsDraft` hole an adversarial pass found in
/// `conformance.rs`'s two parsed-set anchors, reproduced here by writing the anchor the same way.
const BINDER: &str = "fn bind_nodes(";

/// **ADR-0141 clause 4.** No production path constructs a `RecordedSpec` with no steps, and at least
/// one constructs one with them.
///
/// Two-sided on purpose. The forbidden half alone is satisfied by deleting the transcription
/// entirely; the required half alone is satisfied by transcribing at one site and supplying `None` at
/// another. Neither half is evidence without the other.
#[test]
fn the_production_path_transcribes_a_step_list_and_never_supplies_none() {
    let src = src_dir();

    // ---- ANCHORS FIRST. Every one of these fails loudly rather than emptying the scan. ----------
    let files = rs_files(&src);
    assert!(
        !files.is_empty(),
        "no .rs files under {} — an absence assertion over an empty file set reads as a pass",
        src.display()
    );

    // The API this lint is about must still exist, and with BOTH constructors. If `RecordedSpec`
    // grew a third `None`-yielding constructor, or lost one of these two, the spellings below stop
    // describing the property and this guard must be rewritten rather than left passing.
    let replay = src
        .join("..")
        .join("..")
        .join("rk-kernel")
        .join("src")
        .join("replay.rs");
    let replay_src = std::fs::read_to_string(&replay)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", replay.display()));
    // An EXISTENCE anchor, so it is taken over CODE lines: an unstripped doc comment naming a
    // constructor would satisfy it falsely, which is the whole failure mode this module guards.
    let replay_code = code_lines(&replay_src);
    for decl in ["pub fn supplied", "pub fn with_steps", "steps: Option<"] {
        assert!(
            replay_code.iter().any(|(_, l)| l.contains(decl)),
            "`{decl}` is not in {} — `RecordedSpec`'s two-constructor shape (ADR-0141 decision 2) \
             has changed, and this guard's spellings no longer describe the property it asserts",
            replay.display()
        );
    }

    // ---- THE SCAN --------------------------------------------------------------------------
    let mut offenders: Vec<String> = Vec::new();
    let mut transcriptions: Vec<String> = Vec::new();
    let mut binder_seen = false;
    let mut scanned = 0usize;

    for file in &files {
        let text = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", file.display()));
        let rel = file
            .strip_prefix(&src)
            .unwrap_or(file.as_path())
            .display()
            .to_string();
        for (n, line) in code_lines(&text) {
            scanned += 1;
            if line.contains(BINDER) {
                binder_seen = true;
            }
            if line.contains(FORBIDDEN) {
                offenders.push(format!("{rel}:{n}: {}", line.trim()));
            }
            if line.contains(REQUIRED) {
                transcriptions.push(format!("{rel}:{n}"));
            }
        }
    }

    assert!(
        scanned > 500,
        "only {scanned} code lines under {} — the scan is too small to be reading this crate",
        src.display()
    );
    assert!(
        binder_seen,
        "`{BINDER}` was not found on any code line under {} — the composition root's binding pass \
         was renamed or removed, so this guard is asserting a property about a site that no longer \
         exists",
        src.display()
    );

    // ---- (1) NO PRODUCTION `None`. ----------------------------------------------------------
    assert!(
        offenders.is_empty(),
        "ADR-0141 decision 4: a PRODUCTION path constructs a `RecordedSpec` with NO step list.\n  \
         `{FORBIDDEN}` yields `steps: None`, on which `run_activity_steps` falls through to the \
         pre-`s7g` activation dispatch — so this does not crash, does not warn, and fails no suite; \
         it silently stops the runtime walking steps at all.\n  Use `{REQUIRED}`: the composition \
         root holds the `catalog::Activity` and already reads its steps.\n  Offending line(s):\n    \
         {}",
        offenders.join("\n    ")
    );

    // ---- (2) AND SOMETHING ACTUALLY TRANSCRIBES. --------------------------------------------
    assert!(
        !transcriptions.is_empty(),
        "no production code line carries `{REQUIRED}`. The half above is then satisfied by a tree \
         that transcribes NOTHING, which is the same defect wearing the other sign: every \
         activation would take the `steps: None` fall-through and the step executor would be dead \
         code. ADR-0141 decision 2 puts this at `rk-cli`'s `bind_nodes`."
    );
}

/// **The positive control, over the LIVE tree rather than a fixture.**
///
/// `run.rs` quotes `RecordedSpec::supplied(..)` in `run_with`'s docstring today. That makes the
/// comment filter in the guard above load-bearing *right now*: without it the guard is red on a
/// correct tree. This test asserts both halves of that situation, so the day the docstring is
/// reworded the control fails and says the guard has stopped being exercised — rather than the
/// guard quietly becoming a scan with nothing left to strip.
#[test]
fn the_comment_filter_is_doing_real_work_on_run_rs() {
    let run_rs = src_dir().join("verbs").join("run.rs");
    let text = std::fs::read_to_string(&run_rs)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", run_rs.display()));

    assert!(
        text.contains(FORBIDDEN),
        "{} no longer mentions `{FORBIDDEN}` anywhere. That spelling lived in `run_with`'s \
         docstring, quoting §7d.5, and it is what made the comment filter in \
         `the_production_path_transcribes_a_step_list_and_never_supplies_none` load-bearing on a \
         correct tree. If it was deliberately reworded, this control is dead and a synthetic case \
         is owed in its place; if it was deleted to make a lint pass, that is the defect Halt 7 \
         names — do not widen the guard, and do not write the hit.",
        run_rs.display()
    );

    let code = code_lines(&text);
    let survivors: Vec<String> = code
        .iter()
        .filter(|(_, l)| l.contains(FORBIDDEN))
        .map(|(n, l)| format!("{n}: {}", l.trim()))
        .collect();
    assert!(
        survivors.is_empty(),
        "the comment filter kept `{FORBIDDEN}` on a line it reported as CODE in {}. Either the \
         mention moved out of a comment and into real code — in which case the guard above should \
         be failing and this is the more precise report of why — or `code_lines` has stopped \
         stripping the shape it carries:\n    {}",
        run_rs.display(),
        survivors.join("\n    ")
    );
}
