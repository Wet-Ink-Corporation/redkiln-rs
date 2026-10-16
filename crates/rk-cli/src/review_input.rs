//! **The review input, and where the `Binary` route lives** (`s8i` §8i.8; ADR-0199, proposed).
//!
//! [`assemble`] reads the item's typed [`ChangeSummary`] through the `Toolchain` port, for the
//! item's range, and builds what the review receives:
//!
//! - a **`Text`** summary goes to the reviewer ([`ReviewInput::Reviewer`]): the typed summary, and
//!   the diff the [`DiffAssembler`] assembles from it;
//! - a **`Binary`** summary assembles **no diff** (AC-497's AND: *"no textual diff of the binary
//!   path is assembled or dispatched"*) and takes ADR-0199's route, the human gate
//!   ([`ReviewInput::HumanGate`]): an automated verdict is refused, and the refusal opens a gate
//!   through D21-48 X16's type-neutral request ([`rk_kernel::workflow::gate::GateRequest`]) with
//!   the closed reason `automated-verdict-refused` and, as the opaque attachment, this module's
//!   serialisation of the typed summary. The kernel carries those bytes and never types them; only
//!   this crate and the render side ([`rk_store::render::render_binary_summary`]) know what they are.
//!
//! **The hook is [`route`]**, the `Toolchain` decorator `redkiln run`'s `resume_and_read` wraps the
//! stage's toolchain in before the kernel resumes. Its `change_summary` reads the summary from the
//! toolchain it wraps: a `Text` summary is handed to the kernel unchanged; a `Binary` one assembles
//! no diff, is refused to the kernel by name (so the walk stops and no automated reviewer is
//! dispatched), and its [`AutomatedVerdictRefused`] is kept in the [`Refusals`] slot. When the
//! kernel's outcome is that refusal and nothing else, `resume_and_read` opens the gate from it
//! ([`AutomatedVerdictRefused::open_gate`]) and appends its `gate_opened` row through the run's gate
//! path. **Binary producers in Phase 1: none** (ADR-0199 decision 4), so in Phase 1 the route is
//! unreachable from a real adapter.
//!
//! It lives here and not in `rk-kernel`, because Checkpoint 8a's `rk-kernel` production ban
//! (ADR-0135's surviving forbids bullet) forbids `crates/rk-kernel/src` to name the type.

use std::sync::{Arc, OnceLock};

use rk_kernel::workflow::fsm::{FsmContext, RunState};
use rk_kernel::workflow::gate::{GateAttachment, GateReason, GateRequest, OpenedGate};
use rk_ports::toolchain::{
    BinaryChangeSummary, ChangeSummary, Classification, Selection, StepBound, TestOutcome,
    Toolchain, VerbCall, VerbOutput,
};
use rk_store::canon::CanonError;
use rk_types::error::{StepError, TransportKind};
use rk_types::event::Event;
use rk_types::verb_output::BuildReport;

/// What assembles the textual diff the reviewer reads from a summary.
///
/// [`assemble`] calls it for a `Text` summary only: AC-497's *"Verified by"* asserts it is never
/// called for a `Binary` classification, which is why it is a seam.
pub trait DiffAssembler {
    /// The textual diff of `summary` the reviewer reads.
    ///
    /// # Errors
    ///
    /// A [`StepError`] when no textual diff can be assembled for `summary`.
    fn assemble(&self, summary: &ChangeSummary) -> Result<String, StepError>;
}

/// A [`DiffAssembler`] that copies the `Text` arm's own diff, as the `Toolchain` read it.
///
/// Callers that want the diff itself use it; [`route`] hands [`assemble`] a private assembler that
/// copies nothing, because the kernel's walk carries the summary, diff included.
///
/// It refuses a `Binary` summary by name, so a caller that reached it for one fails loudly rather
/// than inventing a diff.
#[derive(Debug, Clone, Copy, Default)]
pub struct SummaryDiff;

impl DiffAssembler for SummaryDiff {
    fn assemble(&self, summary: &ChangeSummary) -> Result<String, StepError> {
        match summary {
            // Genuine duplication: the reviewer's input stores the typed summary AND its diff.
            ChangeSummary::Text(text) => Ok(text.diff.clone()),
            ChangeSummary::Binary(_) => Err(StepError::Unsupported {
                capability: "diff: a Binary summary has no textual diff (AC-497)".to_owned(),
            }),
        }
    }
}

/// What the review receives for one item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewInput {
    /// A `Text` summary: the reviewer receives it, with its assembled diff.
    Reviewer(ReviewerInput),
    /// A `Binary` summary: no reviewer; an automated verdict is refused (ADR-0199's route).
    HumanGate(AutomatedVerdictRefused),
}

/// The reviewer's input: the typed summary and the diff assembled from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewerInput {
    summary: ChangeSummary,
    diff: String,
}

impl ReviewerInput {
    /// The typed summary the reviewer receives.
    #[must_use]
    pub const fn summary(&self) -> &ChangeSummary {
        &self.summary
    }

    /// The assembled diff.
    #[must_use]
    pub fn diff(&self) -> &str {
        &self.diff
    }

    /// The typed summary, by value: what [`route`] hands the kernel for a `Text` item.
    #[must_use]
    pub fn into_summary(self) -> ChangeSummary {
        self.summary
    }
}

/// A `Binary` item's refusal of an automated verdict, and the gate request it opens.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomatedVerdictRefused {
    /// Always the `Binary` arm: [`refuse`], the only constructor, wraps a `Binary` payload.
    summary: ChangeSummary,
    request: GateRequest,
}

impl AutomatedVerdictRefused {
    /// The typed summary the human gate receives: the `Binary` arm.
    #[must_use]
    pub const fn summary(&self) -> &ChangeSummary {
        &self.summary
    }

    /// The type-neutral request: the closed reason and, as its attachment, the typed summary's
    /// serialisation.
    #[must_use]
    pub const fn request(&self) -> &GateRequest {
        &self.request
    }

    /// Open the human gate through the request: the `gate_opened` row the caller appends.
    ///
    /// The token is `GateRecord::derive_reach`'s through `gate::reach`, the gate's kind is the
    /// reason, and the row carries the attachment's bytes and digest.
    ///
    /// # Errors
    ///
    /// [`CanonError`] as [`GateRequest::open`].
    pub fn open_gate(&self, state: &RunState, ctx: &FsmContext<'_>) -> Result<Event, CanonError> {
        self.request.open(state, ctx).map(OpenedGate::into_event)
    }
}

/// Why the review input could not be assembled.
#[derive(Debug)]
pub enum ReviewInputError {
    /// The `Toolchain` port could not read the range's summary.
    Toolchain(StepError),
    /// The diff assembler refused the `Text` summary.
    Diff(StepError),
    /// The typed summary did not serialise.
    Serialise(serde_json::Error),
}

impl std::fmt::Display for ReviewInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toolchain(e) => write!(f, "the range's change summary could not be read: {e:?}"),
            Self::Diff(e) => write!(f, "the reviewer's diff could not be assembled: {e:?}"),
            Self::Serialise(e) => write!(f, "the typed summary did not serialise: {e}"),
        }
    }
}

impl std::error::Error for ReviewInputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Toolchain(_) | Self::Diff(_) => None,
            Self::Serialise(e) => Some(e),
        }
    }
}

/// Assemble the review input for the item whose change is `range`.
///
/// # Errors
///
/// [`ReviewInputError::Toolchain`] when the port refuses the range;
/// [`ReviewInputError::Diff`] when the assembler refuses a `Text` summary;
/// [`ReviewInputError::Serialise`] when a `Binary` summary does not serialise.
pub fn assemble(
    toolchain: &dyn Toolchain,
    range: &str,
    bound: StepBound,
    diff: &dyn DiffAssembler,
) -> Result<ReviewInput, ReviewInputError> {
    let summary = toolchain
        .change_summary(range, bound)
        .map_err(ReviewInputError::Toolchain)?;
    match summary {
        ChangeSummary::Text(_) => {
            let assembled = diff.assemble(&summary).map_err(ReviewInputError::Diff)?;
            Ok(ReviewInput::Reviewer(ReviewerInput {
                summary,
                diff: assembled,
            }))
        }
        ChangeSummary::Binary(binary) => refuse(binary)
            .map(ReviewInput::HumanGate)
            .map_err(ReviewInputError::Serialise),
    }
}

/// The refusal for a `Binary` summary: no diff, and the gate request carrying its serialisation.
fn refuse(binary: BinaryChangeSummary) -> Result<AutomatedVerdictRefused, serde_json::Error> {
    let summary = ChangeSummary::Binary(binary);
    let bytes = serialise(&summary)?;
    Ok(AutomatedVerdictRefused {
        request: GateRequest::new(
            GateReason::AutomatedVerdictRefused,
            GateAttachment::new(bytes),
        ),
        summary,
    })
}

/// This module's serialisation of the typed summary: the `ChangeSummary` value's JSON, which for
/// the untagged `Binary` arm is its four members (`assets`, `captures`, `tests`, `build`).
fn serialise(summary: &ChangeSummary) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(summary)
}

/// The [`AutomatedVerdictRefused`] a [`route`]d toolchain met, kept for the caller that opens the
/// gate.
///
/// **Shared, set once** (OWN-11): the decorator lives inside the `Ports` the kernel takes by value
/// for the stage, and the composition root reads the slot after the kernel returns. A `OnceLock`
/// holds it, so there is no lock to poison and no second writer: the first refusal of the stage
/// run is the one the gate carries (a retry of the same step reads the same range).
#[derive(Debug, Clone, Default)]
pub struct Refusals {
    first: Arc<OnceLock<AutomatedVerdictRefused>>,
}

impl Refusals {
    /// The refusal the routed toolchain met, if it met one.
    #[must_use]
    pub fn first(&self) -> Option<&AutomatedVerdictRefused> {
        self.first.get()
    }
}

/// **The review route's hook on `redkiln run`'s path** (ADR-0199 decision 3, proposed; `s8i`
/// §8i.8): `inner`, decorated so that its `change_summary` goes through [`assemble`].
///
/// A `Text` summary reaches the kernel unchanged. A `Binary` one assembles no diff, its
/// [`AutomatedVerdictRefused`] is kept in the returned [`Refusals`], and the kernel receives
/// [`StepError::Unsupported`] naming `automated-verdict-refused`, so the stage's walk stops and no
/// automated reviewer is dispatched. Every other method delegates. It takes and returns the
/// `Arc<dyn Toolchain>` that `rk_ports::root::Ports` holds, the field it decorates, as
/// [`crate::verbs::run::bridged`] does.
#[must_use]
pub fn route(inner: Arc<dyn Toolchain>) -> (Arc<dyn Toolchain>, Refusals) {
    let refusals = Refusals::default();
    // An `Arc` handle: the decorator writes the slot the caller reads.
    let routed = Routed {
        inner,
        refusals: refusals.clone(),
    };
    (Arc::new(routed), refusals)
}

/// The [`DiffAssembler`] [`route`] hands [`assemble`]: the kernel's walk carries the typed summary
/// itself, whose `Text` arm already holds its diff, so no separate copy is assembled.
struct CarriedInSummary;

impl DiffAssembler for CarriedInSummary {
    fn assemble(&self, _summary: &ChangeSummary) -> Result<String, StepError> {
        Ok(String::new())
    }
}

/// The decorator [`route`] returns.
struct Routed {
    inner: Arc<dyn Toolchain>,
    refusals: Refusals,
}

/// The capability a `Binary` summary's refusal names: the step, and the reason it is refused.
fn refused_capability() -> String {
    format!(
        "change_summary: the range carries a Binary-classified path, and ADR-0199 routes it to the \
         human gate ({}): no automated reviewer receives it",
        GateReason::AutomatedVerdictRefused
    )
}

/// What the kernel receives for a `Binary` summary.
fn refused_to_the_kernel() -> StepError {
    StepError::Unsupported {
        capability: refused_capability(),
    }
}

/// Whether `class` is exactly the refusal a [`route`]d toolchain hands the kernel for a `Binary`
/// summary: a stage whose walk ended in it ended at the kept refusal and nowhere else.
pub(crate) fn refused_the_step(class: &StepError) -> bool {
    matches!(class, StepError::Unsupported { capability } if *capability == refused_capability())
}

impl Toolchain for Routed {
    fn detect(&self) -> Result<bool, StepError> {
        self.inner.detect()
    }
    fn build(&self, bound: StepBound) -> Result<BuildReport, StepError> {
        self.inner.build(bound)
    }
    fn test(&self, selection: &Selection, bound: StepBound) -> Result<TestOutcome, StepError> {
        self.inner.test(selection, bound)
    }
    fn lint(&self, bound: StepBound) -> Result<(), StepError> {
        self.inner.lint(bound)
    }
    fn classify(&self, path: &str) -> Result<Classification, StepError> {
        self.inner.classify(path)
    }
    fn change_summary(&self, range: &str, bound: StepBound) -> Result<ChangeSummary, StepError> {
        match assemble(self.inner.as_ref(), range, bound, &CarriedInSummary) {
            Ok(ReviewInput::Reviewer(input)) => Ok(input.into_summary()),
            Ok(ReviewInput::HumanGate(refused)) => {
                self.refusals.first.get_or_init(|| refused);
                Err(refused_to_the_kernel())
            }
            Err(ReviewInputError::Toolchain(e) | ReviewInputError::Diff(e)) => Err(e),
            Err(ReviewInputError::Serialise(e)) => Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!("change_summary: the Binary summary did not serialise: {e}"),
            }),
        }
    }
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError> {
        self.inner.run_verb(call)
    }
}
