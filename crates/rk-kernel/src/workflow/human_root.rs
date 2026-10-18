//! **The step-4 gate on a human-sourced kind** (`s8j` §8j.2; ADR-0200 decision 2, proposed).
//! *(Added 2026-10-18, `s8j` build, package S8J-C2-ROOT-QUESTIONS.)*
//!
//! Selection's step 4 reports a required kind no activity can produce as `Unsatisfiable { reason:
//! NoAdmissibleProducer }`. When the artifact registry declares that kind `source = "human"`, an
//! answer can supply it, so [`super::fsm::next_action`]'s step 2 opens a durable gate of kind
//! `human_root:<Kind>`, `<Kind>` the registry name verbatim. The pack's question for it is bound
//! through the loader's `binds` (ADR-0177 decision 7), never by name here.
//!
//! **The two Phase-1 spellings are constants on code lines** (ADR-0200 decision 3; `s8h` §8h.0
//! item 7), so the gate-expiry lint's minted table and Checkpoint 8h read them where they are spelt.
//! [`gate_kind`] is the one mint, and a later human kind brings its own constant, question and rows.

/// The gate-kind prefix: the one case of step 4's `NoAdmissibleProducer` an answer resolves.
const PREFIX: &str = "human_root";

/// The fix lane's root: the gate the analysis stage opens when no `BugReport` is attached.
pub const BUG_REPORT_KIND: &str = "human_root:BugReport";

/// The story lane's root: the gate the specification stage opens when no `IntakeRecord` is attached.
pub const INTAKE_RECORD_KIND: &str = "human_root:IntakeRecord";

/// The gate kind of the step-4 gate on the human-sourced `kind`: `human_root:<kind>`.
#[must_use]
pub fn gate_kind(kind: &str) -> String {
    format!("{PREFIX}:{kind}")
}
