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
//! # 🚧 CYCLE 3 FILLED THE TABLE. IT DID NOT END THE VACUITY, AND THE DIFFERENCE IS THE POINT
//!
//! §7d.14: *"over an empty rule table AC-414's per-fixture join, AC-415's per-id join and SM-121's
//! dispatch count"* are all **vacuous**. Cycle 2 landed [`RULES`] empty and claimed none of the
//! three. Cycle 3 grew it to all thirty-eight ids and landed
//! `doctor_rule_table_covers_every_register_named_row` — the twenty-nine register-named ids as a
//! floor (RA-M-01) — **watched red on the empty table first**, on *"`RULES` is empty"*, which is
//! the vacuity stated as a failure rather than as a comment.
//!
//! **What that test buys, and what it does not.** It buys the floor: no criterion in the register's
//! §9, §12 or §14 names a rule id the table lacks. It does **not** buy AC-414, AC-415 or SM-121,
//! and none of the three is claimed yet — every `check` is [`unbuilt`] until cycle 5, and a table
//! of thirty-eight rules that all find nothing satisfies a per-**id** join exactly as an empty one
//! does. A floor over ids cannot see that, because ids are all it reads. What sees it is cycle 4's
//! `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through **its own rule
//! only** and demands a finding — the first assertion in this slice that a rule which does nothing
//! cannot pass.
//!
//! **Thirty-eight, not thirty-two.** §7d.8 and §7d.13 both say thirty-eight and an independent
//! count off the table's first column agrees (39 `RK-D-*` tokens less the prose prefix
//! `RK-D-DANGLING-` of the compound cell `RK-D-DANGLING-{ROLE,SLOT,ATTACH}`). §7d.16 item 4 said
//! *thirty-two* and was corrected in the stage file on 2026-09-20: D18-19 moved the table 32 → 38
//! on 2026-09-19 and that item was not updated with it.

use super::{Ctx, Finding, Severity};

/// The placeholder every `check` points at until cycle 5 writes its predicate.
///
/// 🚧 **One shared function and not thirty-eight identical stubs**, because thirty-eight bodies
/// that all return nothing are thirty-eight places to forget the same thing, and because the thing
/// that catches a row still pointing here is not a reader: it is cycle 4's
/// `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through its own rule
/// alone and demands a finding. A row left on this function fails it **by id**.
#[expect(
    clippy::missing_const_for_fn,
    reason = "const only because the cycle-3 placeholder returns nothing — cycle 5 replaces every               reference to it with a predicate that reads the `Ctx` and allocates. The expectation               goes UNFULFILLED when the last row leaves it, which is what says this function can               be deleted."
)]
fn unbuilt(_: &Ctx) -> Vec<Finding> {
    Vec::new()
}

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

/// **The table** — the thirty-eight ids of `s7d` §7d.8, in its own row order, which is the order
/// `verbs/doctor.rs` evaluates them in.
///
/// **Not transcribed by hand.** The rows were walked out of §7d.8's table's first column, with its
/// compound cells split, by a script that refused to write unless the walk yielded exactly 38
/// distinct ids — so the table here and the table in the runbook cannot drift through a
/// transcription step nobody re-ran. Halt 7: count it off the table, and re-count after any edit.
///
/// **Severities, counted off the same walk** [measured 2026-09-21]: **32** `Error`, **5**
/// `Warning` (`RK-D-DEAD-DEF`, `RK-D-UNFILLED-SLOT`, `RK-D-UNREAD`, `RK-D-HEADROOM-MARGIN`,
/// `RK-D-QUARANTINE-LIVE`) and **1** `Ok` (`RK-D-REBUILT-TIER2`). The three ids of the compound
/// `WORKTREE-ORPHAN / NETWORK-FS / CREDENTIAL-RETAINED` cell are `Error` because design §8.4 fixes
/// them so and the cell says the design outranks the runbook (ADR-0092 clause 2.5).
///
/// 🚧 **Cycle 3: every `check` is [`unbuilt`]** — the ids and their severities are real, the
/// predicates are cycle 5's. `doctor_rule_table_covers_every_register_named_row` is green on this
/// table; **AC-414, AC-415 and SM-121 are still not claimed**, because a table of rules that find
/// nothing satisfies a per-id join exactly as an empty one does. What ends that is cycle 4's
/// `ac415_every_rule_id_has_a_rejecting_fixture`, which runs each fixture through **its own rule
/// only** and requires at least one finding — so a row still pointing at [`unbuilt`] when cycle 5
/// is done is red there, by id.
pub static RULES: &[Rule] = &[
    Rule {
        id: "RK-D-GATE-OUTPUTS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-HOOKS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-STORE-ID",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-STRAY-STORE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ROLE-UNRESOLVED",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ROLE-AMBIGUOUS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CARD-TOOLS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CARD-TOOLS-ABSENT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-TIER-LEAK",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-TIMEOUT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-ROLE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-SLOT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DANGLING-ATTACH",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-DEAD-DEF",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-UNFILLED-SLOT",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ENV-DENYLIST",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CLI-VERSION",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-TTL",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-UNKNOWN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-VALUE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-NEW",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-FACET-FACT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ADAPTER",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-UNREAD",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-AGENTS-WITHOUT-AGENT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CACHE-PROFILE",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-MARGIN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-HEADROOM-MARGIN",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-REBUILT-TIER2",
        severity: Severity::Ok,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-WORKTREE-ORPHAN",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-NETWORK-FS",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CREDENTIAL-RETAINED",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-REVIEW-REEXEC",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-QUARANTINE-LIVE",
        severity: Severity::Warning,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-SINK-COUNT",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-ADVISORY-LEDGER",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-CORPUS-SHA",
        severity: Severity::Error,
        check: unbuilt,
    },
    Rule {
        id: "RK-D-VERIFICATION-POLICY",
        severity: Severity::Error,
        check: unbuilt,
    },
];

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
