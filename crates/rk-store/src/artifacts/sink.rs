//! **AC-310 / AC-530 — the advisory sink's accounting**, as a pure fold over the sink's rows.
//!
//! Two invariants, two functions, both printing BOTH sides on failure (`s7b-ii` §7b-ii.11: *"the
//! identity must fail with both sides printed, so the failure names which side moved"*):
//!
//! - **intake** (AC-310): `findings_in == blocks + sink_records` — every non-blocking finding of a
//!   review is persisted to the sink and none is dropped; [`intake_identity`].
//! - **lifetime** (AC-530): `live_sink == created − retired − dismissed` at every point, where
//!   `dismissed` counts AC-311's human dispositions and a record leaves *"never silently, never by
//!   deletion"*; [`sink_accounting`] folds the three advisory kinds and refuses a retirement or
//!   dismissal of a record the fold never saw created — a leave with no arrival is the silent
//!   deletion the invariant forbids.
//!
//! `finding_id` on every row is AC-305's identity (`super::finding_identity`). This module reads
//! `rk_types::event::Event` slices and never a port: `rk-store` does not depend on `rk-ports`.

use std::collections::BTreeSet;

use rk_types::digest::Digest;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;

/// The sink's counts after a fold, with `live` derived and asserted equal to the identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SinkCount {
    pub created: usize,
    pub retired: usize,
    pub dismissed: usize,
    pub live: usize,
}

/// How the fold refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkAccountingError {
    /// A row with no readable `finding_id`.
    MalformedRow { kind: &'static str, index: usize },
    /// A retirement or dismissal names a record the fold never saw created — a leave with no
    /// arrival, which is the silent deletion the invariant forbids.
    LeftWithoutArriving {
        kind: &'static str,
        finding_id: Digest,
        index: usize,
    },
    /// The same record retired or dismissed twice.
    LeftTwice {
        kind: &'static str,
        finding_id: Digest,
        index: usize,
    },
}

impl std::fmt::Display for SinkAccountingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MalformedRow { kind, index } => {
                write!(f, "row {index} ({kind}) carries no readable finding_id")
            }
            Self::LeftWithoutArriving {
                kind,
                finding_id,
                index,
            } => write!(
                f,
                "row {index} ({kind}) leaves the sink for {} which was never created — a record \
                 leaves only by a recorded disposition, never by deletion (AC-530)",
                finding_id.as_str()
            ),
            Self::LeftTwice {
                kind,
                finding_id,
                index,
            } => write!(
                f,
                "row {index} ({kind}) leaves the sink for {} a second time",
                finding_id.as_str()
            ),
        }
    }
}

impl std::error::Error for SinkAccountingError {}

fn finding_id(event: &Event) -> Option<Digest> {
    event
        .payload
        .get("finding_id")
        .and_then(serde_json::Value::as_str)
        .and_then(|s| Digest::from_hex(s).ok())
}

/// Fold the sink's rows into its counts, holding `live == created − retired − dismissed` at every
/// row. Non-advisory rows are ignored (this is a sink fold, not a run fold).
///
/// # Errors
///
/// [`SinkAccountingError`] on a malformed row, a leave with no arrival, or a second leave.
pub fn sink_accounting(events: &[Event]) -> Result<SinkCount, SinkAccountingError> {
    let mut count = SinkCount::default();
    let mut live: BTreeSet<Digest> = BTreeSet::new();
    let mut left: BTreeSet<Digest> = BTreeSet::new();
    for (index, event) in events.iter().enumerate() {
        let kind = match event.kind {
            EventKind::AdvisoryCreated => "advisory_created",
            EventKind::AdvisoryRetired => "advisory_retired",
            EventKind::AdvisoryDismissed => "advisory_dismissed",
            _ => continue,
        };
        let id = finding_id(event).ok_or(SinkAccountingError::MalformedRow { kind, index })?;
        match event.kind {
            EventKind::AdvisoryCreated => {
                count.created += 1;
                live.insert(id);
            }
            EventKind::AdvisoryRetired | EventKind::AdvisoryDismissed => {
                if !live.remove(&id) {
                    return Err(if left.contains(&id) {
                        SinkAccountingError::LeftTwice {
                            kind,
                            finding_id: id,
                            index,
                        }
                    } else {
                        SinkAccountingError::LeftWithoutArriving {
                            kind,
                            finding_id: id,
                            index,
                        }
                    });
                }
                left.insert(id);
                if event.kind == EventKind::AdvisoryRetired {
                    count.retired += 1;
                } else {
                    count.dismissed += 1;
                }
            }
            _ => {}
        }
        // the invariant, at EVERY point
        debug_assert_eq!(live.len(), count.created - count.retired - count.dismissed);
    }
    count.live = live.len();
    Ok(count)
}

/// AC-310's intake identity failed: both sides, named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntakeMismatch {
    pub findings_in: usize,
    pub blocks: usize,
    pub sink_records: usize,
}

impl std::fmt::Display for IntakeMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "findings_in = {} but blocks + sink_records = {} + {} = {} — {} finding(s) were {} (AC-310)",
            self.findings_in,
            self.blocks,
            self.sink_records,
            self.blocks + self.sink_records,
            self.findings_in.abs_diff(self.blocks + self.sink_records),
            if self.findings_in > self.blocks + self.sink_records {
                "DROPPED"
            } else {
                "counted twice"
            }
        )
    }
}

impl std::error::Error for IntakeMismatch {}

/// **AC-310's intake identity**: `findings_in == blocks + sink_records`.
///
/// # Errors
///
/// [`IntakeMismatch`] carrying all three counts, so the failure names which side moved.
pub fn intake_identity(
    findings_in: usize,
    blocks: usize,
    sink_records: usize,
) -> Result<(), IntakeMismatch> {
    if findings_in == blocks + sink_records {
        Ok(())
    } else {
        Err(IntakeMismatch {
            findings_in,
            blocks,
            sink_records,
        })
    }
}
