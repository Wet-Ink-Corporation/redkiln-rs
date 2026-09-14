//! The one compile-fail harness, named after its rule and not after the tool (`s7b` §7b.4).
//!
//! `tests/ui/` is this glob's root and therefore holds **compile-FAIL cases only** — a file that
//! must compile placed there makes `trybuild` report an unexpected success and turns the harness
//! red. Each case commits a `.stderr` snapshot, so a case that starts failing for a **different**
//! reason goes red rather than silently continuing to pass.
//!
//! # What is here, and what §7b.4's table still owes
//!
//! **Corrected 2026-09-14, Slice 4e.** The paragraph that stood here said *"`ac480_generics_leak.rs`
//! and **`stay_class_state_is_unconstructible.rs` are NOT written**"*, and half of it is now false.
//! **FIVE of §7b.4:426-433's six cases are here**: `ac041_selection_reads_no_clock.rs`,
//! `ac050_rigor_is_not_arithmetic.rs`, `ac061_required_set_is_not_constructible_outside_select.rs`,
//! `ac061_required_set_has_no_deserialize.rs` and — new — `stay_class_state_is_unconstructible.rs`.
//! **`ac480_generics_leak.rs` is still absent** and is Slice 4i's; Checkpoint 7b asserts all six
//! **by path** with their `.stderr`, so that one absence is live and is reported here rather than
//! left to be discovered by that assertion.
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
//! # 🛑 SIX OF THE ELEVEN CASES ARE ASSERTED BY NO CHECKPOINT LINE, AND THAT IS WHY IT IS SAID HERE
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
//! Because no checkpoint line would notice any of the six going missing, each of the four newest
//! is asserted **by path, from the test that carries its criterion** —
//! `ac028_policy_match_has_no_wildcard` and `disjoint_channels_ac032` in `tests/it/retry.rs`,
//! `ac007_replay_divergence_halts` in `tests/it/divergence.rs`, and
//! `ac010_malformed_key_is_refused_before_spawn` in `tests/it/runtime.rs` — each asserting that its
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
    t.compile_fail("tests/ui/ac020_counters_never_share.rs");
    t.compile_fail("tests/ui/ac025_exhaustion_is_not_a_pass.rs");
    t.compile_fail("tests/ui/ac028_steperror_wildcard.rs");
    t.compile_fail("tests/ui/ac032_finding_is_not_a_steperror.rs");
    t.compile_fail("tests/ui/ac050_rigor_is_not_arithmetic.rs");
    t.compile_fail("tests/ui/ac061_required_set_is_not_constructible_outside_select.rs");
    t.compile_fail("tests/ui/ac061_required_set_has_no_deserialize.rs");
    t.compile_fail("tests/ui/stay_class_state_is_unconstructible.rs");
}
