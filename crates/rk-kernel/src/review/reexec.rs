//! Re-execution — the deterministic gate's re-run of a finding's repro at the reviewed SHA
//! (ADR-0074 clauses 3 and 4; AC-287–AC-290). The four-cell rule and the S0.1 harness are slice R3's;
//! the types are here because the policy table reads the outcome.

use rk_types::artifact::findings::Finding;

use super::classify::Observation;

/// Why a re-execution could not separate candidate from reference (AC-290).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Unverifiable {
    /// Failed on the candidate AND on the reference.
    BothFail,
    /// The command could not be run at all.
    CannotRun,
    /// The run exceeded its budget.
    Timeout,
}

impl Unverifiable {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BothFail => "both_fail",
            Self::CannotRun => "cannot_run",
            Self::Timeout => "timeout",
        }
    }
}

/// The outcome of a re-execution — `Flipped | Unverifiable {..}` and nothing else (AC-290: the
/// class enum has four variants and no `Unverifiable`; this enum is where that word lives).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReExecution {
    /// Fails on the candidate and passes on the reference (AC-288), or the mutant survived and
    /// discriminates (AC-289).
    Flipped,
    Unverifiable(Unverifiable),
}

/// What one re-execution recorded: the observation the classifier reads, and the outcome the
/// policy table reads. Two members, because ADR-0096 made them two questions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReExecutionRecord {
    pub id: String,
    pub observation: Observation,
    pub outcome: ReExecution,
}

/// The port the confirm half re-executes through. The Toolchain fake stands behind it at this
/// stage (AC-334 is ARMED for that reason, `s7b-ii` §7b-ii.9).
pub trait ReExecutor {
    /// Re-run `finding.repro` at `sha` and record what was observed.
    fn run(&self, finding: &Finding, sha: &str) -> ReExecutionRecord;
}
