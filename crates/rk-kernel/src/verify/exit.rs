//! **AC-346's stage-exit check** — *"GIVEN an AC that is neither pinned by a flip-verified
//! assertion nor answered at a human gate explicitly for that AC WHEN the stage attempts to exit
//! THEN the exit is refused and the unpinned AC ids are named in the refusal AND the refusal is an
//! event, not only a console line."*
//!
//! The check reads the computed [`AcLedger`] and nothing else: a quarantine of the survivor's
//! finding is not an input and is not an answer (ADR-0104 decision 4; AC-345). The refusal is the
//! `stage_exit_refused` row (**ADR-0125**), written from this one site (its decision 3), carrying
//! the ids and the ledger they were read from. A permitted exit writes nothing.

use rk_types::artifact::ledger::AcLedger;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The exit was refused: the ids, and the row that says so.
#[derive(Debug, Clone, PartialEq)]
pub struct ExitRefused {
    /// Every AC whose status is neither `pinned` nor `human_answered`, in ledger order.
    pub unpinned: Vec<String>,
    /// The `stage_exit_refused` row (ADR-0125).
    pub event: Event,
}

impl std::fmt::Display for ExitRefused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "stage exit refused: {} unpinned (AC-346; a quarantine is not an answer, ADR-0104 \
             decision 4)",
            self.unpinned.join(", ")
        )
    }
}

impl std::error::Error for ExitRefused {}

/// **The check.** Every row whose status does not permit exit is named; the row is the ONE
/// construction site of `stage_exit_refused` in `src/` (ADR-0125 decision 3;
/// `crate::module_boundary::stage_exit_refused_single_site_rule`).
///
/// # Errors
///
/// [`ExitRefused`] when any AC is neither `pinned` nor `human_answered`.
pub fn stage_exit(item: &str, stage: &str, ledger: &AcLedger) -> Result<(), ExitRefused> {
    let unpinned: Vec<String> = ledger
        .rows
        .iter()
        .filter(|r| !r.status.permits_exit())
        .map(|r| r.ac.clone())
        .collect();
    if unpinned.is_empty() {
        return Ok(());
    }
    let statuses: serde_json::Map<String, serde_json::Value> = ledger
        .rows
        .iter()
        .map(|r| (r.ac.clone(), serde_json::Value::from(r.status.as_str())))
        .collect();
    let event = Event::new(
        EventKind::StageExitRefused,
        serde_json::json!({
            "item": item,
            "stage": stage,
            "unpinned": unpinned,
            "statuses": statuses,
        }),
    );
    Err(ExitRefused { unpinned, event })
}
