//! **The review gate** — Step 7b-ii, `s7b-ii` §7b-ii.3–.8. *Reviewer proposes, machine confirms*
//! (ADR-0074 as ADR-0096 supersedes it in part).
//!
//! | module | what it owns | criteria |
//! | --- | --- | --- |
//! | [`classify`] | the pure classifier over the repro record and the gate's observation | AC-284 (PROVISIONAL), AC-286 |
//! | [`policy`] | the policy table — the only place a class becomes a block — and the tier resolution | AC-283, AC-294, AC-296, AC-298, AC-299, AC-302 |
//! | [`reexec`] | the re-execution outcome and the port the confirm half re-runs through | AC-287–AC-290 (slice R3) |
//! | [`confirm`] | findings in, decisions and rows out | AC-285, AC-295, AC-299; AC-307/AC-310/AC-313 (R3) |
//! | [`ingest`] | the dispatch result → `Findings` or a typed error, never an empty set | AC-282, AC-318 |
//! | [`record`] | the persisted finding, carrying the model label for drift only | AC-285 |
//! | [`pack_rules`] | the fix lane is `Strict`; a cascade is refused | AC-300, AC-301 |
//!
//! Module layout is load-bearing for Checkpoint 7b-ii's three absence-based bans, which filter on
//! the MODULE PATH: `policy(\.rs|/)` for AC-285's *no reader of the model label inside the policy
//! module* and AC-294's *no block-value construction outside it*; `classify(\.rs|/)` for AC-286's
//! *no clock, env or filesystem inside the classifier*. So the two functions the register names as
//! **two functions** — the classifier over the repro record and `policy::classify`, the policy-table
//! lookup (AC-283) — are two files, and the lints that police them live in
//! `crate::module_boundary`, outside both (`s7b-ii` §7b-ii.12).
//!
//! **ADR-0101 clause 5 bounds what any of this may be read as.** S-SUBTLE-CB is registered and
//! unrun; the gate exists and fails closed, and nothing in this module or its tests claims it is
//! guarded against false approval.

pub mod classify;
pub mod confirm;
pub mod ingest;
pub mod pack_rules;
pub mod policy;
pub mod record;
pub mod reexec;
