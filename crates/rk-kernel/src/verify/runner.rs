//! **The one `Toolchain::test` port-call site, and the `verify_completed` row it journals** —
//! `s8g`'s verify record (R1 = ADR-0191 decisions 2 and 8, proposed; D21-42 10b; `s8g` §8g.4).
//! *(Added 2026-09-29, `s8g` build, package S8G-P3-KERNEL-GIT.)*
//!
//! - **One site.** [`run_test`] makes the port call and, when the call COMPLETED, appends one
//!   `verify_completed` row in the same function. The executor's `toolchain:test` arm
//!   (`runtime::step`) and the verify pass (`rk-cli`'s `verify_pass`, through this function) both
//!   reach the port here, so a `confirmation-test` walk journals the same kind as the pass.
//! - **Completed only.** `Ok` is a completed run, green or red (record T: a red suite is `Ok` with
//!   `failed >= 1`). A timed-out, killed, unspawnable or unreadable call is `Err`, writes NO row,
//!   and its `StepError` travels back unchanged for the caller's typed refusal row (AC-321: *"a
//!   **completed** verify run"*).
//! - **`origin` and `stage` are data.** `origin` is `"pass"` for the pass's call and
//!   `"<activity>@<version>/<step>"` for a catalog step's; `stage` is the node's stage name. Neither
//!   is compared here: this is not selection (ADR-0162 decision 5 bars a stage literal there).
//! - **`adapter` is opaque** (AC-487): the name `rk-cli`'s `detect()` loop chose, carried on the
//!   runtime context as [`VerifyIdentity`] and never compared by this crate.
//! - **The item key rides beside the payload** (ADR-0182's carrier; D21-46 R3), never in it: the
//!   payload type refuses unknown members.
//!
//! [`replay`] is AC-321's audit read: the range endpoints, the path list and the toolchain version
//! come back from the rows verbatim, and nothing here reads a working tree.

use rk_ports::toolchain::{Selection, StepBound, TestOutcome, Toolchain};
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::verify::{VerifyCompleted, VerifyRange, VerifySelection};

/// Which call made the port call: the verify pass, or one catalog step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestOrigin<'a> {
    /// The verify pass's one whole-suite call.
    Pass,
    /// A catalog `toolchain:test` step: `activity_id@version` and the `[[step]]` id.
    Step {
        activity_at_version: &'a str,
        step: &'a str,
    },
    /// The verify pass's mutation harness, running one criterion's declared `suite` over its
    /// declared patch in the ephemeral `<item>.mut` tree (`s8g` §8g.5; R1 = ADR-0191 decision 11,
    /// proposed). *(dated 2026-09-29, s8g build, package S8G-P6-MUTATION-RESUME-DRIFT.)* Its row
    /// is not the pass's whole-suite row, so AC-473's `origin = "pass"` reading never takes it.
    Mutation { ac: &'a str },
}

impl TestOrigin<'_> {
    /// The row's `origin` member: `"pass"`, `"<activity>@<version>/<step>"`, or
    /// `"mutation/<ac>"`.
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Pass => "pass".to_owned(),
            Self::Step {
                activity_at_version,
                step,
            } => format!("{activity_at_version}/{step}"),
            Self::Mutation { ac } => format!("mutation/{ac}"),
        }
    }
}

/// The runtime-context values the composition root binds for the row: the adapter `detect()`
/// chose, and the range the run's calls are made over. Opaque to this crate (AC-487).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyIdentity {
    /// The chosen adapter's name, carried and never compared.
    pub adapter: String,
    /// The range endpoints (AC-321).
    pub range: VerifyRange,
}

/// One port call's context.
#[derive(Debug, Clone, Copy)]
pub struct TestCall<'a> {
    /// The item the row is scoped to (the interim key, beside the payload). Empty for a lone
    /// step outside any item, which writes the row unscoped.
    pub item: &'a str,
    pub selection: &'a Selection,
    pub bound: StepBound,
    pub origin: TestOrigin<'a>,
    /// The node's stage name, as data.
    pub stage: &'a str,
    pub identity: &'a VerifyIdentity,
    /// The resolved changed-path list the call was made over (AC-321).
    pub paths: &'a [String],
}

/// The closed port selection, as the row records it.
fn recorded_selection(selection: &Selection) -> VerifySelection {
    match selection {
        Selection::WholeSuite { .. } => VerifySelection::WholeSuite,
        Selection::Filter(args) => VerifySelection::Filter { args: args.clone() },
        Selection::Affected { .. } => VerifySelection::Affected,
        Selection::Named(name) => VerifySelection::Named { name: name.clone() },
    }
}

/// The digest of the outcome's JSON: the report, the counts and the runner (D21-48 X15: the
/// runner reaches the row through `report_digest`, never as a member).
fn report_digest(outcome: &TestOutcome) -> rk_types::digest::Digest {
    rk_types::digest::sha256(
        serde_json::to_string(outcome)
            .unwrap_or_else(|e| format!("unserialisable outcome: {e}"))
            .as_bytes(),
    )
}

/// **The payload** one completed call journals (R1 decision 8's thirteen members).
#[must_use]
pub fn verify_completed(outcome: &TestOutcome, call: &TestCall<'_>) -> VerifyCompleted {
    VerifyCompleted {
        report_digest: report_digest(outcome),
        selection: recorded_selection(call.selection),
        basis: outcome.report.basis,
        basis_reason: outcome.report.basis_reason,
        passed: outcome.passed,
        failed: outcome.failed,
        selected_test_count: outcome.report.selected_test_count,
        adapter: call.identity.adapter.clone(),
        range: call.identity.range.clone(),
        paths: call.paths.to_vec(),
        toolchain_version: outcome.report.toolchain_version.clone(),
        origin: call.origin.render(),
        stage: call.stage.to_owned(),
    }
}

/// **The one port call.** Calls `Toolchain::test`; on a COMPLETED call appends one
/// `verify_completed` row through `append` and returns the outcome; on any `Err` appends nothing.
///
/// # Errors
///
/// The port's own [`StepError`], unchanged; or [`StepError::Transport`] `Io` naming the store's
/// refusal when the row could not be appended (the outcome is then not returned, because a
/// completed call whose row is not journalled cannot be replayed).
pub fn run_test(
    toolchain: &dyn Toolchain,
    call: &TestCall<'_>,
    append: &mut dyn FnMut(Event) -> Result<(), String>,
) -> Result<TestOutcome, StepError> {
    let outcome = toolchain.test(call.selection, call.bound)?;
    let row = Event::new(
        EventKind::VerifyCompleted,
        verify_completed(&outcome, call).to_payload(),
    );
    let row = if call.item.is_empty() {
        row
    } else {
        row.with_item(call.item)
    };
    append(row).map_err(|detail| StepError::Transport {
        kind: TransportKind::Io,
        detail: format!("the verify_completed row could not be appended: {detail}"),
    })?;
    Ok(outcome)
}

/// A row that did not read back.
#[derive(Debug)]
pub struct ReplayError {
    /// The row's position among the events handed in.
    pub index: usize,
    pub error: serde_json::Error,
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "verify_completed row {} does not read back: {}",
            self.index, self.error
        )
    }
}

impl std::error::Error for ReplayError {}

/// **AC-321's replay**: every `verify_completed` row, in journal order, read from the rows alone
/// (the range endpoints, the path list and the toolchain version verbatim). Other kinds are
/// skipped; no tree is read.
///
/// # Errors
///
/// [`ReplayError`] for a row whose payload does not read back (an absent or unknown member).
pub fn replay(events: &[Event]) -> Result<Vec<VerifyCompleted>, ReplayError> {
    events
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EventKind::VerifyCompleted)
        .map(|(index, e)| {
            VerifyCompleted::from_payload(&e.payload).map_err(|error| ReplayError { index, error })
        })
        .collect()
}
