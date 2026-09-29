//! **The committed golden log's UNFEATURED controls** — the two guards `--features corpus` was the
//! only thing pinning. Authored 2026-09-14 on an adversarial verification pass.
//!
//! [`crate::replay`] — `tests/it/replay.rs` — is `#![cfg(feature = "corpus")]` whole, for two
//! stated reasons: its read route re-points to `$CORPUS_REPO/golden-logs/` at §7b.11's pin move
//! (AC-464), and it spawns `git` through `rk_git::version::assert_at_startup`, the only minter of
//! the capability token `GitExec::new` demands. **Neither reason reaches the two guards below.**
//! `rk_kernel::replay::read_run` and `rk_kernel::replay::compare_recorded_residue` read a directory
//! and compare two strings; they bind no port, spawn no process and touch no corpus. So the
//! controls for them belong in the **unfeatured** run, which is the one Checkpoint 7b's third loop
//! (`s7b`:1520-1532) and its zero-selection line (:1345-1346) actually execute.
//!
//! **Neither test name here contains `golden_log_replay` or `ac161_ac164_delete_db_replay`**, so
//! :1345-1346's bare-substring count over the unfeatured `--list` stays at **0**. That line counts
//! substrings with **no** word anchor, which is why the names below avoid them outright rather than
//! relying on a boundary.
//!
//! # What each control closes, and the measurement that found it open
//!
//! | control | the defect it closes | how it was measured |
//! | --- | --- | --- |
//! | [`ac007_divergence_detection_over_the_committed_log`] | `ac007_replay_divergence_halts` constructs a `Divergence` itself and calls `.halt()`, so it pins the **halt** and not the **detection**. Both production comparisons could be dead | both patched to `if false && …`: the unfeatured suite reported **83 passed, 0 failed** |
//! | [`read_run_refuses_a_log_whose_specs_json_is_missing`] | `read_run`'s refusal of a log without `inputs/specs.json` had no positive control | the refusal removed: the whole replay suite stayed **green**. Same class as the 2026-09-13 G23 finding |
//!
//! # 🛑 A PINNED GUARD THAT IS NO LONGER CALLED IS THE SAME DEFECT WEARING A GREEN
//!
//! Extracting the two comparisons into [`rk_kernel::replay::compare_recorded_residue`] makes them
//! reachable from here — and it also moves the mutation target. Deleting the **call** from
//! `replay()` would leave every assertion below passing. [`the_replay_still_calls_the_divergence_
//! detection`] is the source lint that closes that, over `include_str!` of the real file, on this
//! crate's own established pattern (`tests/it/retry.rs`'s `ac028_policy_match_has_no_wildcard`
//! reads `src/workflow/retry.rs` the same way).

use std::path::{Path, PathBuf};

use rk_kernel::replay::{ReplayError, compare_recorded_residue, read_run};

/// The committed in-crate copies (§7b.12's `logs/` row) — the same path `tests/it/replay.rs` uses.
fn committed_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/logs")
}

/// The one committed run, whichever `<run_id>` directory it is under.
///
/// **Found rather than spelled**, because `run_id` is a content hash of the inputs and moves with
/// them; a hard-coded directory name would turn a re-blessed fixture into a compile-time-looking
/// failure in the wrong place.
pub fn the_committed_run() -> PathBuf {
    let mut runs: Vec<PathBuf> = std::fs::read_dir(committed_root())
        .expect("tests/fixtures/logs is readable")
        .map(|e| e.expect("a readable entry").path())
        .filter(|p| p.is_dir())
        .collect();
    runs.sort();
    assert_eq!(
        runs.len(),
        1,
        "exactly one committed golden log is expected under {}; found {:?}",
        committed_root().display(),
        runs
    );
    runs.remove(0)
}

/// Copy the committed run into a scratch directory so a control can perturb it.
///
/// **Not under `target/golden-logs-staging/`**, which is what the operator's single `CORPUS_PIN`
/// move copies: a deliberately-corrupted run sitting there would be copied into the corpus with the
/// good ones (`s7b`:922-926).
fn copy_into_scratch(name: &str) -> PathBuf {
    let dest = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/golden-log-scratch")
        .join(name);
    if dest.exists() {
        std::fs::remove_dir_all(&dest).expect("the scratch copy is removable");
    }
    copy_tree(&the_committed_run(), &dest);
    dest
}

fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the scratch tree is creatable");
    for entry in std::fs::read_dir(from).expect("readable") {
        let path = entry.expect("a readable entry").path();
        let target = to.join(path.file_name().expect("a file name"));
        if path.is_dir() {
            copy_tree(&path, &target);
        } else {
            std::fs::copy(&path, &target).expect("the file copies");
        }
    }
}

/// **AC-007's DETECTION, pinned without `--features corpus`.**
///
/// > **AC-007** — GIVEN a replay whose recomputed decision differs from the recorded one / WHEN the
/// > divergence is **detected** / THEN the run halts with a typed
/// > `ReplayDivergence { activity_id, recorded, recomputed }`.
///
/// `tests/it/divergence.rs::ac007_replay_divergence_halts` builds the `Divergence` itself, so what
/// it pins is `.halt()`. **This pins the comparison that produces one**, over the recorded side of
/// the committed log — `run.json`'s `terminal_state` and `next_action`, values written at record
/// time — so nothing here is compared against a second fold of the same journal.
///
/// **What makes it discriminating, clause by clause:**
///
/// - The *agreement* case comes first, and it uses the log's **own** recorded strings. A comparison
///   that refused everything would fail here.
/// - Each divergence case perturbs **one** side, and the perturbed value is asserted **non-empty
///   and different** before the comparison runs — otherwise the halt is built from two equal
///   strings and asserts nothing.
/// - The **order** is asserted: with both sides wrong, the error is `NextActionDiverged`. The next
///   action is the earlier decision, so it is the one that caused the other, and a comparison that
///   reported the terminal state first would name the consequence as the cause.
/// - All three fields of the typed halt are asserted populated, which is AC-007's THEN verbatim.
#[test]
fn ac007_divergence_detection_over_the_committed_log() {
    let run = read_run(&the_committed_run()).expect("the committed log reads back");

    let recorded_terminal = run.terminal_state().to_owned();
    let recorded_next = run.next_action().to_owned();
    // PREMISE: the recorded side carries content. An empty pair would make every comparison below
    // a comparison of two empty strings.
    assert!(
        !recorded_terminal.is_empty() && !recorded_next.is_empty(),
        "the committed run.json must carry both residue strings, or AC-008's terminal-state half \
         is being compared against nothing"
    );

    // ---- 1. AGREEMENT. The recorded values compare equal to themselves --------------------------
    compare_recorded_residue(&run, &recorded_terminal, &recorded_next)
        .expect("the recorded residue agrees with itself, or the comparison refuses everything");

    // ---- 2. A DIVERGENT TERMINAL STATE IS DETECTED, not absorbed --------------------------------
    let tampered_terminal = format!("{recorded_terminal}-tampered");
    assert_ne!(tampered_terminal, recorded_terminal, "PREMISE");
    let err = compare_recorded_residue(&run, &tampered_terminal, &recorded_next)
        .expect_err("a rewritten terminal state must be DETECTED, never absorbed");
    match err {
        ReplayError::TerminalStateDiverged(halt) => {
            assert_eq!(halt.recorded, recorded_terminal);
            assert_eq!(halt.recomputed, tampered_terminal);
            assert!(
                !halt.activity_id.is_empty(),
                "AC-007 names three fields and all three must be populated"
            );
            assert_ne!(
                halt.recorded, halt.recomputed,
                "a halt whose two sides are equal is not a divergence"
            );
        }
        other => panic!("expected TerminalStateDiverged, got {other:?}"),
    }

    // ---- 3. A DIVERGENT NEXT ACTION IS DETECTED -------------------------------------------------
    let tampered_next = format!("{recorded_next}-tampered");
    assert_ne!(tampered_next, recorded_next, "PREMISE");
    let err = compare_recorded_residue(&run, &recorded_terminal, &tampered_next)
        .expect_err("a rewritten next action must be DETECTED, never absorbed");
    match err {
        ReplayError::NextActionDiverged(halt) => {
            assert_eq!(halt.recorded, recorded_next);
            assert_eq!(halt.recomputed, tampered_next);
            assert!(!halt.activity_id.is_empty());
        }
        other => panic!("expected NextActionDiverged, got {other:?}"),
    }

    // ---- 4. THE ORDER, because naming the consequence as the cause is a real failure ------------
    let err = compare_recorded_residue(&run, &tampered_terminal, &tampered_next)
        .expect_err("both sides wrong is still a divergence");
    assert!(
        matches!(err, ReplayError::NextActionDiverged(_)),
        "with both wrong the NEXT ACTION is reported: it is the earlier decision and therefore the \
         cause. Got {err:?}"
    );
}

/// **The guard above is pinned; this asserts it is still CALLED.**
///
/// Extracting the two comparisons out of `replay()` made them testable here and moved the mutation
/// target at the same time: deleting the call would leave every assertion above green. The source
/// lint is this crate's own pattern — `tests/it/retry.rs`'s `ac028_policy_match_has_no_wildcard`
/// reads `src/workflow/retry.rs` through `include_str!` for the same reason.
///
/// It is a **grep over source**, and its limit is stated rather than left to be discovered: it
/// cannot tell a call on a live path from a call inside a branch that never runs. What it does
/// catch is the deletion, the comment-out and the `if false &&` — the three shapes the mutation
/// that found this defect actually used.
#[test]
fn the_replay_still_calls_the_divergence_detection() {
    let src = include_str!("../../src/replay.rs");
    assert_eq!(
        src.matches("compare_recorded_residue(&recorded,").count(),
        1,
        "`replay()` must call `compare_recorded_residue` exactly once; a guard nothing invokes is \
         the same defect as a guard that is not there"
    );
    // **Over CODE lines only.** This file's own header quotes the mutation shape, and a naive
    // `contains` over the whole file matches the prose describing the defect as readily as the
    // defect — which is a lint that fires on its own documentation and therefore says nothing.
    let disabled: Vec<&str> = src
        .lines()
        .map(str::trim)
        .filter(|l| !l.starts_with("//"))
        .filter(|l| l.contains("if false") || l.contains("if true"))
        .collect();
    assert!(
        disabled.is_empty(),
        "src/replay.rs carries a constant-folded guard — the exact shape the mutation that found \
         this gap used to disable both comparisons: {disabled:?}"
    );
}

/// **`read_run` refuses a log without `inputs/specs.json` — the positive control it had none of.**
///
/// The refusal is at `src/replay.rs`'s `read_run`, and its warrant is `terminal_state`'s: a run
/// recorded before Slice 4i carries no dispatch bindings, so replaying one re-derives a journal
/// with **zero** dispatches and compares it against a recorded journal with zero dispatches —
/// AC-008's GIVEN satisfied over an empty N, which is the vacuous green Slice 4i exists to remove.
/// **Removing the refusal left the whole replay suite green** [measured 2026-09-14], which is the
/// same class as the 2026-09-13 G23 finding on the dispatch counter.
///
/// The control is a **copy** of the committed log with one file deleted: nothing under
/// `tests/fixtures/` is mutated, and the copy goes to `target/golden-log-scratch/`, never to the
/// staging tree the operator's `CORPUS_PIN` move copies.
#[test]
fn read_run_refuses_a_log_whose_specs_json_is_missing() {
    let dir = copy_into_scratch("specs-json-removed");

    // PREMISE, and it is the discriminating half: the copy reads back BEFORE the deletion. Without
    // it, a refusal below could be about the copy being broken rather than about the missing file.
    read_run(&dir).expect("the unperturbed copy reads back");

    let specs = dir.join("inputs/specs.json");
    assert!(specs.is_file(), "PREMISE: the committed log carries one");
    std::fs::remove_file(&specs).expect("the scratch copy is writable");

    let err = read_run(&dir).expect_err("a log without inputs/specs.json must be REFUSED");
    match err {
        ReplayError::Malformed { path, detail } => {
            assert!(
                path.ends_with("specs.json"),
                "the refusal must name the file it is about; got {path}"
            );
            assert!(
                detail.contains("specs.json"),
                "the refusal must say what is wrong; got {detail}"
            );
        }
        other => panic!(
            "the refusal must be the typed Malformed refusal, not an Io error or a silent empty \
             map; got {other:?}"
        ),
    }

    // AND IT IS NOT REFUSING EVERYTHING: restoring the file restores the read. Without this the
    // assertion above is satisfied by a `read_run` that refuses every log it is handed.
    copy_tree(&the_committed_run(), &dir);
    read_run(&dir).expect("restoring specs.json restores the read");
}

/// **Every member of `RunInputs::to_json`'s output survives `write_run` → `read_run`.**
/// ADR-0130 decision 9.
///
/// `run_id` is `sha256_lp([SCHEMA, stage, canonical(to_json())])`, and `replay()` re-derives it
/// before it re-derives anything else — the kernel's own words at that refusal: *"one comparison
/// policies all of them"*. That property is worth exactly its coverage, and it is coverage of the
/// **preimage**, not of the layout. An input inside `to_json` and outside the layout does not weaken
/// the guard; it makes the guard fire on **every** record, which is worse than a gap, because a halt
/// that every record triggers is a halt nobody can act on.
///
/// # 🛑 ONE ASSERTION OVER THE WHOLE PREIMAGE, AND THAT IS THE POINT
///
/// This test compares two `run_id`s. It does **not** enumerate the members, because a field list is
/// a thing somebody has to remember to extend — and R-54 is precisely what happens when they do not:
/// Step 7d Slice 2 added `config` to `to_json` and not to the layout, and nothing on the tree
/// noticed for two slices. A digest over the whole input set catches the next one without being
/// told about it.
///
/// # 🛑 THE COMMITTED LOG CARRIES EVERY OPTIONAL INPUT EXCEPT THE CONFIG
///
/// Measured: its `inputs/` holds `runtime.json`, `specs.json` and `deviation.json` beside the five
/// required entries, and no config — it pre-dates the config edit, which is exactly why the replay
/// suite stayed green over it. So adding a config is what makes the input set complete, and the
/// premise below asserts that adding one MOVES the `run_id`: an input that is not in the preimage
/// could not be the subject of this test, and a premise that did not check would let the whole
/// thing pass over a member nothing hashes.
#[test]
fn write_run_and_read_run_round_trip_every_recorded_input() {
    let dir = copy_into_scratch("every-recorded-input");
    let mut run = read_run(&dir).expect("the committed log reads back");

    // PREMISE (1): the committed log carries every OPTIONAL input but the config, so the round trip
    // below is over a complete set once one is added.
    for optional in ["runtime.json", "specs.json", "deviation.json"] {
        assert!(
            dir.join("inputs").join(optional).is_file(),
            "PREMISE: the committed log must carry inputs/{optional}, or this test is not over a \
             complete input set"
        );
    }
    assert!(
        !dir.join("inputs/config.json").is_file(),
        "PREMISE: the committed log must carry NO config — it pre-dates the config edit, and that \
         is why premise (2) below can measure the digest moving"
    );

    let without_config = run.inputs.run_id().expect("the committed input set hashes");

    run.inputs = run
        .inputs
        .clone()
        .with_config(rk_kernel::config::Config::new(
            "rk-adapter-claude",
            "rk-adapter-sqlite",
            &[
                "rk-adapter-toolchain-node".to_owned(),
                "rk-adapter-toolchain-rust".to_owned(),
            ],
            50,
        ));
    let expected = run
        .inputs
        .run_id()
        .expect("the input set with a config hashes");

    // PREMISE (2): the config IS in the preimage. Without this the equality below is satisfied by a
    // member `to_json` never hashed, and the test would be green over the wrong thing.
    assert_ne!(
        expected, without_config,
        "PREMISE: threading a `Config` in must move the `run_id`, or `config` is not part of \
         `to_json`'s output and this test is not about a recorded input at all"
    );

    // ---- THE ROUND TRIP. -----------------------------------------------------------------------
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/golden-log-scratch/rt");
    let _ = std::fs::remove_dir_all(&root);
    let written = rk_kernel::replay::write_run(&root, &run).expect("the record writes");
    let back = read_run(&written).expect("the written record reads back");

    assert_eq!(
        back.inputs
            .run_id()
            .expect("the read-back input set hashes"),
        expected,
        "the input set that came back does not hash to the one that went in, so at least one \
         member of `to_json`'s output is in the digest and not in the layout. `replay()` \
         re-derives this value and refuses on a mismatch, so every record carrying that member is \
         unreplayable — AC-042's *\"from the logged inputs alone\"* cannot hold for it. This is \
         R-54's shape; ADR-0130 is the record."
    );
}

/// **`read_run` refuses a present-but-invalid `inputs/config.json`** — ADR-0130 decision 6's
/// positive control.
///
/// The refusal's warrant is R-54's own shape. An **absent** config is `None`, legitimately: every
/// log written before Slice 2's config edit carries none, the committed golden log included. A
/// **present and unreadable** one is different in kind — reading it as `None`, or as a default, or
/// as a partially-filled value would put the record straight back into the state ADR-0130 repaired,
/// with `run_id`'s preimage covering a value the layout did carry and the two disagreeing.
///
/// # 🛑 WHY THIS CONTROL EXISTS AT ALL
///
/// This module's header is the argument: `read_run`'s refusal of a missing `inputs/specs.json` had
/// no positive control, and when an adversarial pass removed the refusal *"the whole replay suite
/// stayed green"*. A refusal nothing drives is indistinguishable from a refusal nobody wrote.
///
/// The perturbation is `unread_window_runs` as a **string** rather than a number — the member whose
/// refusal text says *"never a silent 0"*, so the case is the one a `unwrap_or_default()` would have
/// swallowed.
#[test]
fn read_run_refuses_a_log_whose_config_json_is_invalid() {
    let dir = copy_into_scratch("config-json-invalid");
    let config = dir.join("inputs/config.json");

    // PREMISE (1): the committed log carries NO config, and reads back. Both halves matter — the
    // second is what makes a refusal below about the file rather than about the copy.
    assert!(
        !config.is_file(),
        "PREMISE: the committed log carries no config.json"
    );
    read_run(&dir).expect("the unperturbed copy reads back");

    // PREMISE (2): a VALID config.json reads back too, so the refusal below is about the CONTENT
    // and not about the file's presence. Without this, a `read_run` that refused any log carrying a
    // config at all would satisfy the assertion that follows.
    std::fs::write(
        &config,
        br#"{"harness":"rk-adapter-claude","store":"rk-adapter-sqlite","toolchains":["a","b"],"unread_window_runs":50}
"#,
    )
    .expect("the scratch copy is writable");
    read_run(&dir).expect("a VALID config.json reads back");

    // THE PERTURBATION: present, well-formed JSON, and `unread_window_runs` is a string.
    std::fs::write(
        &config,
        br#"{"harness":"rk-adapter-claude","store":"rk-adapter-sqlite","toolchains":["a","b"],"unread_window_runs":"50"}
"#,
    )
    .expect("the scratch copy is writable");

    let err = read_run(&dir).expect_err("an invalid config.json must be REFUSED, never defaulted");
    match err {
        ReplayError::Malformed { path, detail } => {
            assert!(
                path.ends_with("config.json"),
                "the refusal must name the file it is about; got {path}"
            );
            assert!(
                detail.contains("unread_window_runs"),
                "the refusal must name the member that is wrong, not merely that something is; \
                 got {detail}"
            );
        }
        other => panic!(
            "the refusal must be the typed Malformed refusal, not an Io error and not a silent \
             `None` that puts the record back in R-54's state; got {other:?}"
        ),
    }

    // AND IT IS NOT REFUSING EVERYTHING: removing the file restores the read, because an ABSENT
    // config is `None` and that is legitimate. This is the half that stops the assertion above
    // being satisfied by a `read_run` that refuses every log it is handed.
    std::fs::remove_file(&config).expect("the scratch copy is writable");
    read_run(&dir).expect("an ABSENT config.json is None, and the log reads back");
}

/// **The recording's file inputs ARE the golden log's recorded inputs, byte for byte** (2026-09-24,
/// s8a, D21-50). `tests/it/replay.rs`' recording — corpus-gated, so no CI leg runs it — compiles
/// six sources in with `include_str!` and reads `spikes/p9/facets.toml` at run time, and every one
/// of those bytes enters `run_id`'s preimage. The committed log `implement-bf1d8cec3b22d288` was
/// blessed over their CRLF-smudged working bytes; ADR-0164's `eol=lf` made the six LF, the
/// recording moved to `implement-87544dc047ecc61c`, and nine corpus `replay::` tests went red
/// with nothing unfeatured to see it. `.gitattributes`' header rule: *"a file whose bytes a test
/// pins is protected from the slice that lands it"*. The six are now `-text` rows holding the
/// SAME git objects as their golden copies; the other two are LF on both sides under row 1.
///
/// A terminator-only difference is reported as such, because that is the failure this exists for.
#[test]
fn the_recording_sources_are_the_committed_logs_inputs_byte_for_byte() {
    let run = the_committed_run();
    let inputs = run.join("inputs");
    let pairs: [(&str, &[u8], &str); 8] = [
        (
            "catalog-select/impl-report.toml",
            include_bytes!("../fixtures/catalog-select/impl-report.toml"),
            "catalog/impl-report.toml",
        ),
        (
            "catalog-select/change-summary.toml",
            include_bytes!("../fixtures/catalog-select/change-summary.toml"),
            "catalog/change-summary.toml",
        ),
        (
            "catalog-select/change-summary-ue.toml",
            include_bytes!("../fixtures/catalog-select/change-summary-ue.toml"),
            "catalog/change-summary-ue.toml",
        ),
        (
            "catalog-select/changeset.toml",
            include_bytes!("../fixtures/catalog-select/changeset.toml"),
            "catalog/changeset.toml",
        ),
        (
            "catalog-select/spec-authoring.toml",
            include_bytes!("../fixtures/catalog-select/spec-authoring.toml"),
            "catalog/spec-authoring.toml",
        ),
        (
            "packs/incident-deviation.toml",
            include_bytes!("../fixtures/packs/incident-deviation.toml"),
            "pack/incident-deviation.toml",
        ),
        (
            "registries/artifacts/artifacts.toml",
            include_bytes!("../fixtures/registries/artifacts/artifacts.toml"),
            "artifacts/artifacts.toml",
        ),
        (
            "spikes/p9/facets.toml (read at run time)",
            &[],
            "facets.toml",
        ),
    ];
    let facets =
        std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml"))
            .expect("spikes/p9/facets.toml is readable");
    let mut bad = Vec::new();
    for (name, source, golden) in pairs {
        let source: &[u8] = if source.is_empty() { &facets } else { source };
        let recorded = std::fs::read(inputs.join(golden))
            .unwrap_or_else(|e| panic!("the golden input {golden} is unreadable: {e}"));
        if source != recorded.as_slice() {
            let lf = |b: &[u8]| String::from_utf8_lossy(b).replace("\r\n", "\n");
            let why = if lf(source) == lf(&recorded) {
                "line terminators only"
            } else {
                "content"
            };
            bad.push(format!("{name} differs from {} ({why})", golden));
        }
    }
    assert!(
        bad.is_empty(),
        "the recording would not reproduce {}: {bad:#?}",
        run.display()
    );
}

// ======================================= `ActivitySpec.activity_id_at_version` (ADR-0190; D21-107)
//
// *(Added 2026-09-29, s8f build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; ADR-0190 decision 2,
// proposed; D21-107 item 3, TAKEN PROVISIONALLY.)* The sixth spec member is carried to the adapter
// for its own `dispatch_completed` row and for nothing else. The two tests below pin its two
// halves over the record: it moves no byte `run_id` or the memo key reads, and a record's reader
// repopulates it from the binding's own recorded `activity_id_at_version`.

/// A spec for the two tests below, its sixth member given.
fn member_spec(member: Option<&str>) -> rk_types::activity::ActivitySpec {
    rk_types::activity::ActivitySpec {
        card_digest: rk_types::digest::sha256(b"card/member-probe"),
        tools: vec!["Read".to_owned()],
        schema: rk_types::typed::SchemaId::new("ImplReport"),
        timeout_secs: 900,
        inputs: vec![("item".to_owned(), serde_json::json!("bug-1"))],
        activity_id_at_version: member.map(str::to_owned),
    }
}

/// Write `run` under a fresh scratch root named `name` and return the written directory.
fn written(name: &str, run: &rk_kernel::replay::RecordedRun) -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/golden-log-scratch")
        .join(name);
    let _ = std::fs::remove_dir_all(&root);
    rk_kernel::replay::write_run(&root, run).expect("the record writes")
}

/// **The byte-identity half (D21-107 item 3).** The committed log's input set, with one binding
/// added whose spec carries the member `Some("member-probe@2")` and, in the twin, `None`: the two
/// `run_id`s are equal, the two written `inputs/specs.json` files are byte-identical, and neither
/// holds the member inside the spec's object; and the memo key `StepContext::for_agent_step`
/// assembles from the two specs is one digest. (The card digest is the binder's, over the bound
/// card alone; `cost::a_real_run_writes_its_activitys_id_at_version_on_dispatch_completed` pins
/// that half.)
///
/// PREMISE: a binding added under a DIFFERENT activation moves the `run_id`, so the equality below
/// is over a preimage that holds the bindings at all.
///
/// Red by: `spec_json` writing the member (the `run_id`s and the bytes differ); the memo key
/// reading the spec whole.
#[test]
fn the_activity_member_moves_no_specs_json_byte_no_run_id_and_no_memo_key() {
    let dir = copy_into_scratch("member-bytes");
    let base = read_run(&dir).expect("the committed log reads back");
    let with = |member: Option<&str>, activation: &str| {
        let mut run = base.clone();
        run.inputs = run.inputs.clone().with_spec(
            "member-probe[node]",
            // A per-step spec too (R1): the activation spec's object also carries the binding's
            // own `activity_id_at_version` key, so a spec member written there would be masked by
            // it; a per-step spec's object carries no such key and would show it.
            rk_kernel::replay::RecordedSpec::supplied(activation, member_spec(member))
                .and_step_specs(
                    std::iter::once(("check".to_owned(), member_spec(member))).collect(),
                ),
        );
        run
    };
    let some = with(Some("member-probe@2"), "member-probe@2");
    let none = with(None, "member-probe@2");
    let other = with(Some("member-probe@2"), "member-probe@3");

    let id = |run: &rk_kernel::replay::RecordedRun| run.inputs.run_id().expect("the set hashes");
    assert_ne!(
        id(&some),
        id(&other),
        "PREMISE: the binding is in `run_id`'s preimage at all"
    );
    assert_eq!(
        id(&some),
        id(&none),
        "the spec's `activity_id_at_version` moved `run_id` (ADR-0190 forbids it in the preimage)"
    );

    let bytes = |name: &str, run: &rk_kernel::replay::RecordedRun| {
        std::fs::read(written(name, run).join("inputs").join("specs.json"))
            .expect("specs.json is written")
    };
    let (a, b) = (bytes("member-some", &some), bytes("member-none", &none));
    assert_eq!(a, b, "specs.json differs with the member Some or None");
    let specs: serde_json::Value = serde_json::from_slice(&a).expect("specs.json is JSON");
    let probe = &specs["member-probe[node]"];
    assert_eq!(
        probe["activity_id_at_version"],
        serde_json::json!("member-probe@2"),
        "PREMISE: the binding's OWN member is recorded"
    );
    assert_eq!(
        probe.as_object().map(serde_json::Map::len),
        Some(7),
        "the binding writes the spec's five members, its own activation and its per-step specs: \
         {probe}"
    );
    assert_eq!(
        probe["step_specs"]["check"]
            .as_object()
            .map(serde_json::Map::len),
        Some(5),
        "a per-step spec's object is the spec's five members and no sixth: {probe}"
    );

    let key = |member: Option<&str>| {
        rk_kernel::memo::StepContext::for_agent_step(
            "member-probe@2",
            rk_kernel::rigor::Rigor::Mandatory,
            "check",
            &member_spec(member),
            0,
        )
        .key()
        .expect("the key assembles")
    };
    assert_eq!(
        key(Some("member-probe@2")),
        key(None),
        "the memo key moved with the spec's `activity_id_at_version`"
    );
}

/// **The replay round-trip half (D21-107 item 3).** Written and read back, a spec's member is
/// repopulated from the binding's own recorded `activity_id_at_version`, never from the spec's
/// five members: a binding recorded as `member-probe@2` whose spec carried `None` reads back
/// `Some("member-probe@2")`, and an UNVERSIONED binding reads back `None`, so a value without its
/// `@version` never reaches an adapter (ADR-0190 decision 1). Over the committed log, every
/// versioned binding's spec reads back its own activation.
///
/// Red by `M16a` (`spec_from_json` forcing the member to `None`).
#[test]
fn the_activity_member_is_repopulated_from_the_recorded_binding_on_read() {
    let dir = copy_into_scratch("member-round-trip");
    let mut run = read_run(&dir).expect("the committed log reads back");
    let committed: Vec<(String, Option<String>)> = run
        .inputs
        .specs()
        .iter()
        .filter_map(|(_, b)| {
            b.spec().map(|s| {
                (
                    b.activity_id_at_version().to_owned(),
                    s.activity_id_at_version.clone(),
                )
            })
        })
        .collect();
    assert!(
        !committed.is_empty(),
        "PREMISE: the committed log carries activation specs"
    );
    for (activation, member) in &committed {
        assert!(
            activation.contains('@'),
            "PREMISE: the committed activation `{activation}` is versioned"
        );
        assert_eq!(
            member.as_deref(),
            Some(activation.as_str()),
            "the committed log's `{activation}` spec did not read back its own activation"
        );
    }

    run.inputs = run
        .inputs
        .clone()
        .with_spec(
            "member-probe[node]",
            rk_kernel::replay::RecordedSpec::supplied("member-probe@2", member_spec(None))
                .and_step_specs(std::iter::once(("check".to_owned(), member_spec(None))).collect()),
        )
        .with_spec(
            "member-plain[node]",
            rk_kernel::replay::RecordedSpec::supplied("member-plain", member_spec(None)),
        );
    let back = read_run(&written("member-rt", &run)).expect("the written record reads back");
    let member = |node: &str| {
        back.inputs
            .specs()
            .iter()
            .find(|(n, _)| n == node)
            .and_then(|(_, b)| b.spec())
            .unwrap_or_else(|| panic!("`{node}` reads back with a spec"))
            .activity_id_at_version
            .clone()
    };
    assert_eq!(
        member("member-probe[node]").as_deref(),
        Some("member-probe@2"),
        "the member is repopulated from the binding's recorded `activity_id_at_version`"
    );
    assert_eq!(
        member("member-plain[node]"),
        None,
        "an unversioned activation reads back as `None`, never as an id without its `@version`"
    );
    let step = back
        .inputs
        .specs()
        .iter()
        .find(|(n, _)| n == "member-probe[node]")
        .and_then(|(_, b)| b.step_specs())
        .and_then(|m| m.get("check"))
        .expect("the per-step spec reads back");
    assert_eq!(
        step.activity_id_at_version.as_deref(),
        Some("member-probe@2"),
        "a per-step spec is repopulated from its binding too"
    );
}
