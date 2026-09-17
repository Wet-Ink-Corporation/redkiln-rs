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
    /// The gate ran and separated nothing in the candidate's disfavour: the command passed on the
    /// candidate (AC-288's two "candidate passes" cells), or the mutant was killed or did not
    /// discriminate. Never a block. NOT one of AC-290's `Unverifiable` reasons — the run was
    /// verifiable and it said "no" — and not in the register's `Flipped | Unverifiable {..}`
    /// spelling either; the tracker raises the enum's wording rather than filing these two cells
    /// under a reason the register does not name.
    DidNotFlip,
    Unverifiable(Unverifiable),
}

/// **AC-288's four-cell truth table** over a `kind = run` re-execution:
///
/// | candidate | reference | outcome |
/// | --- | --- | --- |
/// | fails | passes | `Flipped` — the block |
/// | fails | fails | `Unverifiable { both_fail }` — cannot separate (AC-290) |
/// | passes | passes | `DidNotFlip` — the claim did not reproduce |
/// | passes | fails | `DidNotFlip` — not the candidate's fault |
///
/// The "fails on both" cell is the one a flip-only implementation gets wrong (`s7b-ii`
/// §7b-ii.11): a command that fails everywhere separates nothing and is advisory.
#[must_use]
pub const fn four_cell(candidate_fails: bool, reference_fails: bool) -> ReExecution {
    match (candidate_fails, reference_fails) {
        (true, false) => ReExecution::Flipped,
        (true, true) => ReExecution::Unverifiable(Unverifiable::BothFail),
        (false, _) => ReExecution::DidNotFlip,
    }
}

/// **AC-289's mutation rule** over a `kind = mutate` re-execution: the mutant "flips" — survives
/// when re-applied (ADR-0074 clause 3) — only if the patch applied, the named suite stayed green
/// AND the mutant discriminates (a reverted fix, or an observed assertion flip — ADR-0096 decision
/// 2). A patch that did not apply is `Unverifiable { cannot_run }`; a killed mutant, or a green one
/// that does not discriminate (the equivalent mutant), `DidNotFlip`.
#[must_use]
pub const fn mutation_outcome(
    applied: bool,
    suite_green: bool,
    discriminates: bool,
) -> ReExecution {
    if !applied {
        return ReExecution::Unverifiable(Unverifiable::CannotRun);
    }
    if suite_green && discriminates {
        ReExecution::Flipped
    } else {
        ReExecution::DidNotFlip
    }
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
    /// Re-run `finding.repro` at `sha` and record what was observed — or `None` when the gate
    /// produced no record for it (AC-287: `Unverifiable { cannot_run }`, never a block).
    fn run(&self, finding: &Finding, sha: &str) -> Option<ReExecutionRecord>;
}
