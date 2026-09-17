//! The one compile-fail harness, named after its rule and not after the tool (`s7b` §7b.4).
//!
//! `tests/ui/` is this glob's root and therefore holds **compile-FAIL cases only** — a file that
//! must compile placed there makes `trybuild` report an unexpected success and turns the harness
//! red. Each case commits a `.stderr` snapshot, so a case that starts failing for a **different**
//! reason goes red rather than silently continuing to pass.
//!
//! # What is here, and what §7b.4's table still owes
//!
//! **Corrected 2026-09-14, Slice 4e, and again on this pass.** The paragraph that stood here said
//! *"`ac480_generics_leak.rs` and **`stay_class_state_is_unconstructible.rs` are NOT written**"*;
//! then it said *"FIVE of §7b.4:426-433's six cases are here … **`ac480_generics_leak.rs` is still
//! absent** and is Slice 4i's"*. **Slice 4i shipped without it** — the absence survived three
//! slices of being reported — and **all SIX are now here**: `ac041_selection_reads_no_clock.rs`,
//! `ac050_rigor_is_not_arithmetic.rs`, `ac061_required_set_is_not_constructible_outside_select.rs`,
//! `ac061_required_set_has_no_deserialize.rs`, `stay_class_state_is_unconstructible.rs` and
//! `ac480_generics_leak.rs`. Checkpoint 7b's by-path loop (:1584-1589) enumerates exactly those six
//! and would have halted on the last one.
//!
//! **`ac480_generics_leak.rs` is a SUBSTITUTE and its own header says so**, on
//! `ac028_steperror_wildcard.rs`'s precedent: a `<P: Harness>` bound compiles, so `s7b`:1020's
//! *"red by adding one `<P: Harness>` bound"* has no literal construction. What the file pins is
//! the property that makes AC-480's THEN true of this crate — `Runtime::new` takes
//! `rk_ports::root::Ports` and nothing generic, a bare port-bound parameter is not a trait object,
//! `Ports`' fields are the trait objects, and no `rk-kernel` type is a port so an `impl Harness`
//! return has nothing to name. **AC-480's other named instrument, the API-surface lint over
//! `rk-kernel` (design:279; `s7a`:584 *"added at 7b"*), is NOT built and is ARMED, NOT DISCHARGED**
//! — no grep in Checkpoint 7b asserts it, so it is recorded here rather than left to be discovered.
//!
//! **Why the `--stay` case could not be written before now, and why it can be.** §7b.13 assigns it
//! to **Slice 1**. Residual 9 is FORCED on why it did not land there: *"it cannot be authored until
//! the FSM's resumed/half-advanced state type exists (§7b.4:437-444)"* — and until 2026-09-14 there
//! was no such type, `src/workflow.rs`'s whole body being one `#[cfg(feature)]` control.
//! `fsm::RunState` is that type. The case asserts three closed routes, not one: the constructor is
//! `pub(in crate::workflow)` (`E0624`), there is no `Default` (`E0599`), and every field is private
//! (the struct literal is refused). **It decides nothing about which type carries the ban** — §7b.15
//! records that the `--stay` fixture has **no criterion** and that *"a criterion is allocated … or
//! exit check (5) is withdrawn"*, which is the operator's — and the file's own header says so.
//!
//! # 🛑 SEVEN OF THE THIRTEEN CASES ARE ASSERTED BY NO CHECKPOINT LINE, AND THAT IS WHY IT IS SAID HERE
//!
//! *(This heading read "SIX OF THE ELEVEN" until the AC-487 pass of 2026-09-14 and was already
//! stale at twelve. The running correction is below, one dated paragraph per case, and the count
//! in the set-equality test's own floor is re-measured with it.)*
//!
//! `ac020_counters_never_share.rs` (**AC-020**, Slice 4a) is the third of §7b.9:824-830's *"four
//! `trybuild` compile-fail obligations"* — *"**AC-020**'s separation test over the three budget
//! counters"* — and it is in **neither** §7b.4's six-row case table **nor** §7b.14's by-path loop at
//! :1580-1589, which enumerates six files and none of them this one. Its absence, and any drift in
//! its `.stderr`, would therefore be caught by nothing in Checkpoint 7b. Authoring it as a runtime
//! assertion instead was not open: the register's instrument for that clause is a compile failure,
//! and *"assigning `trybuild` to a criterion whose register instrument is a unit test changes an
//! instrument, which is the one thing a stage file may not do"* (:829-830) — in reverse.
//!
//! `ac025_exhaustion_is_not_a_pass.rs` (**AC-025**, Slice 4b) is in the same position and is there
//! for the opposite reason: AC-025's *Verified by* says *"a **negative fixture** proving no code
//! path maps an exhausted budget to `pass`"* and does **not** say `trybuild`. So the register's own
//! instrument is built as a runtime table in `tests/it/budget.rs` — nine ledgers × two evidence
//! sets, asserting that no combination produces a closed gate while any counter has run out — and
//! the compile-fail case is taken **in addition to** it and never in substitution for it. It pins
//! what the table cannot reach: that there is no `From<BudgetExhausted> for GateVerdict` and that
//! `GateVerdict`'s variants cannot be minted outside `crate::workflow` at all.
//!
//! **The gap was measured, not assumed** [2026-09-14]: adding an `impl From<BudgetExhausted> for
//! GateVerdict` to `src/workflow/budget.rs` turns this case's `.stderr` comparison red
//! (`tests/ui/ac025_exhaustion_is_not_a_pass.rs ... mismatch`), and the temporary impl was removed
//! immediately after the observation — so the case is known to bite and not merely to be present.
//!
//! **Extended 2026-09-14, Slice 4c: there are now FOUR unasserted cases, not two.** The sentence
//! that stood here said *"the same is true of the cases AC-007, AC-010 and AC-028 will need in
//! Slices 4c–4e"*; AC-028's and AC-032's are written, so the future tense is now false of two of
//! them. `ac028_steperror_wildcard.rs` and `ac032_finding_is_not_a_steperror.rs` are committed with
//! their `.stderr` snapshots and are in **neither** §7b.4's six-row case table nor §7b.14's by-path
//! loop at :1580-1589. AC-007's and AC-010's cases are still unwritten and are Slices 4e's and 4h's.
//!
//! **Extended again 2026-09-14, Slice 4e: FIVE unasserted cases, not four.** The sentence above
//! said *"AC-007's and AC-010's cases are still unwritten and are Slices 4e's and 4h's"*; AC-007's
//! is written. `ac007_divergence_match_has_no_wildcard.rs` is committed with its `.stderr` and is in
//! **neither** §7b.4's six-row case table nor §7b.14's by-path loop at :1580-1589.
//! `stay_class_state_is_unconstructible.rs` is the one new case in the **opposite** position: the
//! by-path loop **does** enumerate it, which is why its absence was a live finding for three slices
//! and is now discharged.
//!
//! **And once more 2026-09-14, Slice 4h: SIX unasserted cases, not five.** The sentence above ended
//! *"AC-010's is still unwritten and is Slice 4h's"*, and that is now false.
//! `ac010_dispatch_without_key.rs` is committed with its `.stderr` and is in neither list either.
//! **It stands where `ac032_finding_is_not_a_steperror.rs` stands, not where
//! `ac028_steperror_wildcard.rs` does**: AC-010's *Verified by* names `trybuild` **outright** — *"a
//! `trybuild` test that a `dispatch` call without the key does not compile (the port takes it by
//! value, not `Option`)"* — so the *tool* is the criterion's own and §7b.9:829-830 is not engaged.
//! Its three statements are three distinct errors: the struct literal is refused for private
//! fields, `new(spec, None)` is `E0308`, and `new(spec)` is `E0061`.
//!
//! 🛑 **But the SUBJECT is substituted, and the sentence here claimed it was not.** *(Corrected
//! 2026-09-14 on an adversarial verification pass.)* The clause parenthesises **the port** — *"the
//! port takes it by value, not `Option`"* — and `rk_ports::harness::Harness::dispatch` takes
//! `(spec, store)` and **no key at all** (`crates/rk-ports/src/harness.rs`:37). What this case
//! compiles against is `rk_kernel::workflow::idem::PlannedDispatch`, a **kernel-local** value with
//! **zero construction sites in `src/`**. The case is real and pins what it pins; it is **not** the
//! port-side clause, AC-010 is **ARMED** for that half, and a record carrying the key across the
//! port boundary is **OWED**. Editing `Harness::dispatch` is refused here — ADR-0099 clause 1 fixes
//! that signature and moving it disturbs AC-481/AC-483/AC-499 after Checkpoint 7a passed.
//!
//! **And once more on the AC-487 pass, 2026-09-14: SEVEN unasserted cases of THIRTEEN, not six of
//! twelve.** `ac487_port_identity_is_not_askable.rs` is committed with its `.stderr` and is in
//! neither §7b.4's six-row case table nor §7b.14's by-path loop at :1580-1589. It stands where
//! `ac010_dispatch_without_key.rs` stands and **not** where `ac028_steperror_wildcard.rs` does:
//! AC-487's *Verified by* reads *"a kernel lint (denylist: adapter crate names, `downcast_ref`,
//! `downcast_mut` on port objects); **a compile-fail fixture**"* — two instruments, the second
//! named outright and **beside** the first rather than in substitution for it — so §7b.9:829-830
//! is not engaged and nothing is substituted. The lint (`src/kernel_denylist.rs`) says nobody
//! **has** written an identity check; this case says the **language refuses the question**, because
//! none of the five port traits carries an `Any` supertrait. Neither half is the other and both are
//! built.
//!
//! Because no checkpoint line would notice any of the seven going missing, each of the five newest
//! is asserted **by path, from the test that carries its criterion** —
//! `ac028_policy_match_has_no_wildcard` and `disjoint_channels_ac032` in `tests/it/retry.rs`,
//! `ac007_replay_divergence_halts` in `tests/it/divergence.rs`,
//! `ac010_malformed_key_is_refused_before_spawn` in `tests/it/runtime.rs`, and
//! `ac487_kernel_denylist` in `tests/it/kernel_denylist.rs` — each asserting that its
//! `.rs` and its `.stderr` exist and are non-empty. That catches a deletion; it does **not** catch
//! `.stderr` drift, which only `trybuild` itself catches and which only this harness runs.
//!
//! # `ac007_divergence_match_has_no_wildcard.rs` stands where AC-028's case stands, and says so
//!
//! AC-007's third instrument is *"a `trybuild` test proving the wildcard cannot be reintroduced"* —
//! word for word AC-028's, and with word for word the same problem: **that has no literal
//! construction**, because adding `_ => …` to a `match` compiles. So the file takes the same two
//! readings `ac028_steperror_wildcard.rs` takes, and labels them the same way: a `match` over
//! `Divergence` missing a variant is `E0004` (the property that makes AC-007's THEN true), and a
//! wildcard after an exhaustive match is dead under the **fixture's own**
//! `deny(unreachable_patterns)` — which proves something about Rust and nothing about `rk-kernel`.
//! What `rk-kernel`'s own `Divergence::halt` carries is asserted in `src/workflow/fsm.rs` by its
//! three-arm exhaustiveness and in `tests/it/divergence.rs` by the `ALL_TAGS` sweep.
//!
//! # The two new cases stand in different relations to their criteria, and it matters
//!
//! `ac032_finding_is_not_a_steperror.rs` **is** AC-032's own first instrument. The register says *"a
//! **type** test proving the success and failure channels are disjoint"*, a type test's subject is
//! what compiles, and a compile failure is the only thing that can observe it — so §7b.9:829-830 is
//! not engaged and nothing is substituted.
//!
//! `ac028_steperror_wildcard.rs` is **a substitute, and its own header says so.** AC-028's
//! instrument is *"a compile-fail test proving the wildcard cannot be reintroduced"*, and that has
//! **no literal construction**: adding `_ => …` to a `match` compiles, here as anywhere, and
//! `clippy::wildcard_enum_match_arm` is a `restriction` lint that is not in
//! `[workspace.lints.clippy]`. What the file pins instead is the property that makes AC-028's THEN
//! true — a policy `match` missing a variant does not compile (`E0004`) — plus the deadness of a
//! wildcard placed after an exhaustive one, under the fixture's **own** `deny(unreachable_patterns)`,
//! which proves nothing about `rk-kernel` and is labelled as such. The assertion that `rk-kernel`'s
//! shipped policy carries no `_ =>` arm and no `matches!` is the source lint in
//! `tests/it/retry.rs`, over `include_str!` of the real file. **Neither half is the other and both
//! are reported.**

#[test]
fn trybuild_pins_the_reason_each_kernel_fixture_fails() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/ac007_divergence_match_has_no_wildcard.rs");
    t.compile_fail("tests/ui/ac010_dispatch_without_key.rs");
    t.compile_fail("tests/ui/ac041_selection_reads_no_clock.rs");
    t.compile_fail("tests/ui/ac480_generics_leak.rs");
    t.compile_fail("tests/ui/ac487_port_identity_is_not_askable.rs");
    t.compile_fail("tests/ui/ac020_counters_never_share.rs");
    t.compile_fail("tests/ui/ac025_exhaustion_is_not_a_pass.rs");
    t.compile_fail("tests/ui/ac028_steperror_wildcard.rs");
    t.compile_fail("tests/ui/ac032_finding_is_not_a_steperror.rs");
    t.compile_fail("tests/ui/ac050_rigor_is_not_arithmetic.rs");
    t.compile_fail("tests/ui/ac061_required_set_is_not_constructible_outside_select.rs");
    t.compile_fail("tests/ui/ac061_required_set_has_no_deserialize.rs");
    t.compile_fail("tests/ui/stay_class_state_is_unconstructible.rs");
    // Step 7b-ii (s7b-ii §7b-ii.11, §7b-ii.12): AC-244's type-level half.
    t.compile_fail("tests/ui/ac244_gate_record_missing_a_field.rs");
    // Step 7b-ii (s7b-ii §7b-ii.4, §7b-ii.12): AC-294's block constructor.
    t.compile_fail("tests/ui/ac294_block_constructor.rs");
}

/// **Every case in `tests/ui/` is registered above, and every one commits a non-empty `.stderr`.**
///
/// Two failures this closes, neither of which the harness above can see:
///
/// 1. **A case on disk that no `compile_fail` line names never runs.** `trybuild` is driven by the
///    explicit list, not by the directory, so an unregistered file is invisible — and
///    `trybuild_pins_the_reason_each_kernel_fixture_fails` stays green while it is. The set
///    equality below is what makes adding a file without registering it red.
/// 2. **A case with an empty or absent `.stderr` is green whenever compilation fails for ANY
///    reason** (`s7a` §7a.8), which is the same defect one level down.
///
/// Checkpoint 7b's by-path loop (`s7b`:1584-1589) asserts **six** of these files. The other seven
/// are asserted by no checkpoint line at all — this module's header lists them and says why — so
/// this test is what notices any of the thirteen going missing, not only the six. *(This sentence
/// read "eleven" and "the other six" until 2026-09-14; it was already stale at twelve, and it is
/// corrected in place rather than rewritten, because the drift is what this file exists to make
/// visible.)*
#[test]
fn every_kernel_compile_fail_case_is_registered_and_commits_a_stderr() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    let harness = include_str!("ui.rs");

    let mut on_disk: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("tests/ui is readable") {
        let path = entry.expect("a readable directory entry").path();
        if path.extension().and_then(std::ffi::OsStr::to_str) != Some("rs") {
            continue;
        }
        let name = path
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .expect("a UTF-8 file name")
            .to_owned();

        // Its `.stderr` is committed and carries bytes.
        let stderr = path.with_extension("stderr");
        assert!(
            stderr.is_file(),
            "{} commits no .stderr, so the case is green whenever compilation fails for ANY              reason (s7a §7a.8)",
            stderr.display()
        );
        assert!(
            std::fs::metadata(&stderr).expect("readable").len() > 0,
            "{} is empty, which is the same defect as its absence",
            stderr.display()
        );

        on_disk.push(name);
    }
    on_disk.sort();

    // PREMISE: a directory that had shrunk to nothing would satisfy every equality below.
    assert!(
        on_disk.len() >= 13,
        "tests/ui holds {} cases; it held 13 when this assertion was last re-measured, ac480_generics_leak.rs and ac487_port_identity_is_not_askable.rs included -- a floor one below the set it guards lets a case be deleted with its registration and still pass, and a shrinking set          is what it exists to notice",
        on_disk.len()
    );

    let mut registered: Vec<String> = harness
        .lines()
        .filter_map(|l| l.trim().strip_prefix("t.compile_fail(\"tests/ui/"))
        .filter_map(|l| l.split('"').next())
        .map(str::to_owned)
        .collect();
    registered.sort();

    assert_eq!(
        registered, on_disk,
        "every case in tests/ui must be registered in the one harness and vice versa — an          unregistered file NEVER RUNS and the harness stays green while it sits there"
    );

    // AC-480's case specifically, because Checkpoint 7b's by-path loop found it absent for three
    // slices while this module's header reported it: the path assertion is written out so the
    // report and the instrument cannot drift apart again.
    for f in ["ac480_generics_leak.rs", "ac480_generics_leak.stderr"] {
        assert!(
            dir.join(f).is_file(),
            "AC-480's compile-fail fixture is handed here by `s7a`:995 and asserted by path at              `s7b`:1584-1589; {f} is missing"
        );
    }

    // AC-487's case specifically, for the OPPOSITE reason: Checkpoint 7b's by-path loop does NOT
    // enumerate it, and neither does §7b.4's table, so nothing in the checkpoint would notice it
    // going missing. `ac487_kernel_denylist` in `tests/it/kernel_denylist.rs` asserts the same two
    // paths from the test that carries the criterion; this is the assertion from the harness that
    // actually RUNS the case, and the two are deliberately not one.
    for f in [
        "ac487_port_identity_is_not_askable.rs",
        "ac487_port_identity_is_not_askable.stderr",
    ] {
        assert!(
            dir.join(f).is_file(),
            "AC-487's *Verified by* names a compile-fail fixture beside the kernel lint; {f} is              missing, and no Checkpoint 7b line would say so"
        );
    }
}
