//! **The rule table, and it IS data** — AC-415's own words: *"the `doctor` rule table, which is
//! data with a stable id per rule"*. `s7d` §7d.8.
//!
//! The shape is the stage file's, transcribed: `Rule { id: &'static str, severity: Severity, check:
//! fn(&Ctx) -> Vec<Finding> }` and `pub static RULES: &[Rule]`, run in order by `verbs/doctor.rs`.
//!
//! # Why `fn` and not `Box<dyn Fn>`
//!
//! A plain function pointer is what makes the table a `static` at all: a `&[Rule]` in read-only
//! memory with no allocation and no initialisation order. It is also what makes *"the table is
//! data"* checkable rather than aspirational — a rule cannot capture a `Ctx`, a clock or a store
//! handle, because a `fn` item captures nothing. Everything a rule reads it reads through its one
//! parameter, which is the property `ac414_…` depends on when it builds a `Ctx` itself and runs the
//! table over it.
//!
//! # 🚧 THE TABLE IS EMPTY AT CYCLE 2, AND THE EMPTINESS IS DECLARED RATHER THAN HIDDEN
//!
//! §7d.14: *"over an empty rule table AC-414's per-fixture join, AC-415's per-id join and SM-121's
//! dispatch count"* are all **vacuous**. This cycle claims none of those three. Cycle 3 grows
//! [`RULES`] to all thirty-eight ids and lands
//! `doctor_rule_table_covers_every_register_named_row` — the twenty-nine register-named ids as a
//! floor (RA-M-01) — which is the instrument that closes the vacuity, and its red on this empty
//! table is the behavioural one.
//!
//! **Thirty-eight, not thirty-two.** §7d.8 and §7d.13 both say thirty-eight and an independent
//! count off the table's first column agrees (39 `RK-D-*` tokens less the prose prefix
//! `RK-D-DANGLING-` of the compound cell `RK-D-DANGLING-{ROLE,SLOT,ATTACH}`). §7d.16 item 4 said
//! *thirty-two* and was corrected in the stage file on 2026-09-20: D18-19 moved the table 32 → 38
//! on 2026-09-19 and that item was not updated with it.

use super::{Ctx, Finding, Severity};

/// One row of the table.
#[derive(Debug, Clone, Copy)]
pub struct Rule {
    /// The stable `RK-D-*` id. `&'static str` because AC-415 joins it against a fixture
    /// **directory name**, and a per-run `String` would make that join a comparison of two things
    /// built at run time rather than of a table against the tree.
    pub id: &'static str,
    /// What a finding from this rule weighs.
    pub severity: Severity,
    /// The check. Reads its `Ctx` and nothing else; returns every finding it has, never the first.
    pub check: fn(&Ctx) -> Vec<Finding>,
}

/// **The table.** Thirty-eight rows at cycle 3; empty here.
///
/// 🚧 Cycle 2 — the skeleton. See the module header: the emptiness is what
/// `doctor_rule_table_covers_every_register_named_row` is red on, and that test is cycle 3's.
pub static RULES: &[Rule] = &[];

/// Evaluate every rule in table order, into a [`super::Report`].
///
/// **Every rule is evaluated and every rule is recorded as evaluated**, whether or not it found
/// anything — §7d.8's stdout is *"one line per **evaluated** rule — fired or not"*, and the
/// checkpoint block's FLOOR loop greps the register-named ids off a **clean** repository. A loop
/// that recorded only the rules that fired would make that loop find nothing on a green tree.
///
/// **It takes the harness so a dispatch is attributable.** [`crate::no_dispatch::NoDispatchHarness`]
/// panics naming the rule that reached it, and the only way it can know the rule is to be told
/// before each check. AC-414's instrument is that panic; this is what makes its message useful.
#[must_use]
pub fn evaluate(ctx: &Ctx, harness: &crate::no_dispatch::NoDispatchHarness) -> super::Report {
    let mut report = super::Report::default();
    for rule in RULES {
        harness.entering(rule.id);
        report.evaluated.push(rule.id);
        report.findings.extend((rule.check)(ctx));
    }
    report
}
