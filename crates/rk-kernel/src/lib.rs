//! `rk-kernel` — the workflow side of the split: the selection engine, the predicate grammar and
//! its three-valued evaluator, the registry lints, the memo, the budgets and the replay.
//!
//! **This crate is Step 7b's, and Step 7b is six to eight sessions.** What is here at any moment is
//! a slice, and the slices are `runbook/phase-1/s7b-kernel.md` §7b.3's. **Slices 1 and 2 are on
//! disk:** the crate skeleton (§7b.4), the determinism gate (§7b.5) and the ADR-0103 grammar with
//! its evaluator (§7b.6); and the **two fail-closed facet lints** with their two fixture sets
//! (§7b.7) plus **rigor monotonicity** (§7b.8); and [`select`] carries **steps 1-4 of the seven**
//! (§7b.9) — rigor, the required set, the short-circuit and the producer. **Steps 5 (validate),
//! 6 (execute) and 7 (deviation) are not built and are not stubbed**, and neither is the kernel
//! runtime (Slice 4): a module that exists and returns nothing reads as built to the next reader,
//! and `00-preamble.md`'s golden constraint 10 is that powerless is reported as powerless. The
//! entry point is [`select::select`], which is steps 1-5; step 6 is `Selection::plan` and step 7 is
//! `select::step7`, and neither runs anything.
//!
//! **Two things Slice 2 does not contain, stated so the absence is not read as an oversight.**
//! **AC-072 — the registry-entry loader rule — is a THIRD rule with its own eleven-case fixture
//! set**, the register forbids collapsing it into either lint, and it is **not written**: nothing
//! in `src/` is named for it, so no checkpoint assertion keyed on its name can read [`registry`]'s
//! loader as that rule. And **both lints' CLI surfaces are absent**: `redkiln lint
//! --facet-stability` and `--facet-provenance` are `rk-cli`'s (**Step 7d**) over `.redkiln/`
//! (**Step 7e**), so those halves are **ARMED, NOT DISCHARGED**. What is discharged here is the
//! fail-closed behaviour, which §7b.14 says is what Step 7b owes in their place.
//!
//! # The two halves, and why the split is a compile-time fact here
//!
//! [`workflow`] is the deterministic half: stage selection, gate evaluation, retry decisions, DAG
//! order. It may not read the wall clock, draw randomness, perform I/O or iterate an unordered
//! map — **AC-001–AC-004** — and the instrument is not a convention. The root `clippy.toml`'s
//! `disallowed-methods` / `disallowed-types` tables name the calls and the types; that file has
//! **no path or module scoping**, so [`workflow`] carries its own module-level
//! `#[deny(clippy::disallowed_methods)]`, and CI runs `cargo clippy --workspace --all-targets`
//! **gated on its exit code** — never `-D warnings`, which promotes `clippy::nursery` to a group
//! deny ADR-0083 forbids and ADR-0102 restates. Time, randomness and I/O reach the workflow only
//! as journaled activity results.
//!
//! Every in-source lint exception in this crate is written `#[expect(clippy::<lint>, reason = "…")]`
//! and never `#[allow]`, so an override that stops being needed fails the build (ADR-0102
//! decision 3).
//!
//! # The predicate grammar
//!
//! [`predicate`] is **ADR-0103's** grammar and nothing else's: precedence `not` > comparison/`in` >
//! `and` > `or`, left-associative; an unquoted identifier in value position is a literal;
//! three-valued `null`; `any` over `null` or an empty list is `false` while `all` and `none` are
//! `true`; and a closed `on_false` form set. It is checked against `spikes/p9/grammar-probe.py`,
//! the S-GRAMMAR probe F42 ran over every live predicate, by a differential test rather than by
//! assertion.
//!
//! # What this crate's dependency table says by omission
//!
//! No async runtime, directly or transitively (**AC-528**, ADR-0076), and no `rk-adapter-*` edge
//! (**AC-484**). `rk-git` is a dependency **as a concrete crate** and not as a port (**AC-483**;
//! ADR-0003:57-58). The assertions are over the resolved graph — `cargo tree -p rk-kernel -e
//! normal` — because a manifest sees direct dependencies only.

//! # The two fail-closed facet lints
//!
//! [`facet_stability`] is **AC-071**, the unblocking gate, and its subject is `source = "llm"`.
//! [`facet_provenance`] is **AC-520**, the sibling gate F39 forced, and its subject is
//! `source = "derived"`. They are **two rules with two fixture sets**, never one gate with two
//! subjects: AC-520's own record says why — *"a single gate carrying two different subjects, two
//! different records and two different failure meanings is the shape that invites that collapse"*.
//!
//! # Rigor monotonicity
//!
//! [`rigor`] is **AC-105 / SM-035** over **ADR-0081 clause 1's** superset relation, and it is the
//! first measurement of monotonicity by any tool: `grep -ci monoton spikes/p9/lattice.py` is **0**.

pub mod catalog;
pub mod facet_provenance;
pub mod facet_stability;
pub mod predicate;
pub mod registry;
pub mod replay;
pub mod rigor;
pub mod select;
pub mod toml_subset;
pub mod workflow;
