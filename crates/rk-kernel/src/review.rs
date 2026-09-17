//! **The review gate** — Step 7b-ii, `s7b-ii` §7b-ii.3–.8. *Reviewer proposes, machine confirms*
//! (ADR-0074 as ADR-0096 supersedes it in part).
//!
//! | module | what it owns | criteria |
//! | --- | --- | --- |
//! | [`classify`] | the pure classifier over the repro record and the gate's observation | AC-284 (PROVISIONAL), AC-286 |
//! | [`policy`] | the policy table — the only place a class becomes a block — and the tier resolution | AC-283, AC-294, AC-296, AC-298, AC-299, AC-302 |
//! | [`reexec`] | the re-execution outcome, the four-cell and mutation rules, and the port the confirm half re-runs through | AC-287–AC-290 |
//! | [`confirm`] | findings in, decisions and rows out: `review_block` from ONE site, `advisory_created`, `quarantine_expired`, `advisory_retired` | AC-285, AC-295, AC-299, AC-304, AC-306, AC-307, AC-310, AC-313, AC-316, AC-530 |
//! | [`ingest`] | the dispatch result → `Findings` or a typed error, never an empty set | AC-282, AC-318 |
//! | [`record`] | the persisted finding, carrying the model label for drift only | AC-285, AC-307 |
//! | [`quarantine`] | the two overrides' rows; advisory and `Quarantined` are disjoint | AC-304, AC-307 |
//! | [`render`] | the outcome as a markdown table — a rendering, never a source | AC-304 |
//! | [`regression`] | the regression run's record: the corpus repo's sha and the item's, two fields | AC-314 |
//! | [`pack_rules`] | the fix lane is `Strict`; a cascade is refused | AC-300, AC-301 |
//!
//! The identity every row carries (`finding_id`, AC-305) and the sink's two accounting identities
//! (AC-310, AC-530) are `rk_store::artifacts`, because the store owns what persists; the confirm
//! half computes the identity through it and writes the rows the store folds.
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
//! unrun; the gate exists and fails closed, and nothing in this module or its tests claims more
//! than that. (The forbidden sentence itself is not written here even to deny it: Checkpoint
//! 7b-ii's guard is a substring rule over the crate, and it was right to fire on the negation.)

pub mod classify;
pub mod confirm;
pub mod ingest;
pub mod pack_rules;
pub mod policy;
pub mod quarantine;
pub mod record;
pub mod reexec;
pub mod regression;
pub mod render;
