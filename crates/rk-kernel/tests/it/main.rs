//! `rk-kernel`'s **one** integration binary — a `mod` per case, never one file per case
//! (ADR-0083:74). This is also what makes `--test it` addressable in Checkpoint 7b.
//!
//! What is here is **Slices 1 and 2**: the ADR-0103 grammar, its differential against S-GRAMMAR's
//! probe, the closed `on_false` set, the three-valued evaluator — and the two fail-closed facet
//! lints with their two fixture sets, plus rigor monotonicity. The selection engine (Slice 3) and
//! the runtime (Slice 4) have no modules here yet, and **no empty module stands in for them**: a
//! `mod` that exists and asserts nothing reads as coverage.
//!
//! **`facet_registry_entry_shape` is absent and that is deliberate.** AC-072 is a **third** rule
//! with its own eleven-case fixture set and the register forbids collapsing it into either facet
//! lint; it is not built, so there is no module for it and no test claiming it.

mod dag;
mod deviation;
mod evaluator;
mod execute;
mod facet_provenance;
mod facet_stability;
mod grammar;
mod on_false;
mod probe_differential;
mod producer;
mod replay;
mod rigor;
mod selection;
mod short_circuit;
mod support;
mod ui;
