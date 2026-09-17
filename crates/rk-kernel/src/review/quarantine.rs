//! Quarantine — ADR-0074 clause 6's first-class finding state, as an override with a row
//! (AC-304, AC-307; ADR-0082 clause 4; ADR-0124).
//!
//! A quarantine is a bounded human override: `Quarantined { reason, by, until }` with `until`
//! non-optional (`rk_types::review::Quarantine`; AC-319). [`grant`] and [`remove`] are the two
//! overrides AC-307 names — *"any block and any override (quarantine grant or removal) … an event is
//! appended"* — and each returns its row carrying AC-307's members; the confirm half reads the
//! active set by finding identity and writes `quarantine_expired` when `until` has passed (AC-306).
//!
//! **Advisory and `Quarantined` are disjoint** (ADR-0082 clause 4; AC-304's negative fixture): a
//! quarantine stops a finding BLOCKING, and an advisory finding does not block, so a quarantine on
//! one is refused — it would be a second sink wearing the other name. What quarantine is NOT an
//! answer to is the verify gate's unpinned AC (ADR-0104 decision 4; AC-345, AC-346): that stays
//! refused, quarantine or no quarantine.

use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::review::Quarantine;

use super::classify::EvidenceClass;
use super::confirm::ac307_members;
use super::record::PersistedFinding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuarantineError {
    /// The finding's computed class is `advisory`; advisory and `Quarantined` are disjoint.
    AdvisoryIsDisjoint { ac: String },
}

impl std::fmt::Display for QuarantineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdvisoryIsDisjoint { ac } => write!(
                f,
                "{ac} is advisory and does not block; advisory and Quarantined are disjoint states \
                 (ADR-0082 clause 4) — a quarantine here would be a second sink"
            ),
        }
    }
}

impl std::error::Error for QuarantineError {}

/// Grant a quarantine on a persisted finding: the `quarantine_granted` row (ADR-0124) carrying
/// AC-307's members plus `reason, by, until`.
///
/// # Errors
///
/// [`QuarantineError::AdvisoryIsDisjoint`] when the finding's computed class is `advisory`.
pub fn grant(
    record: &PersistedFinding,
    quarantine: &Quarantine,
    actor: &str,
) -> Result<Event, QuarantineError> {
    if record.computed_class == EvidenceClass::Advisory {
        return Err(QuarantineError::AdvisoryIsDisjoint {
            ac: record.ac.clone(),
        });
    }
    let mut payload = ac307_members(record, record.weight, &record.sha, actor);
    if let Some(obj) = payload.as_object_mut() {
        obj.insert(
            "reason".to_owned(),
            serde_json::Value::from(quarantine.reason.as_str()),
        );
        obj.insert(
            "by".to_owned(),
            serde_json::Value::from(quarantine.by.as_str()),
        );
        obj.insert(
            "until".to_owned(),
            serde_json::Value::from(quarantine.until.unix_seconds()),
        );
    }
    Ok(Event::new(EventKind::QuarantineGranted, payload))
}

/// Lift a quarantine before `until`: the `quarantine_removed` row (ADR-0124) with AC-307's members.
#[must_use]
pub fn remove(record: &PersistedFinding, actor: &str) -> Event {
    Event::new(
        EventKind::QuarantineRemoved,
        ac307_members(record, record.weight, &record.sha, actor),
    )
}
