//! **AC-455 — the policy, not the type, decides whether a missing verification blocks a stage**
//! (S-MIG decision 1). *"GIVEN a spec containing an AC with `verification: None` and a process pack
//! whose policy requires a verification at this stage WHEN the stage's admissibility is evaluated
//! THEN the stage is refused with the AC id named, at zero model cost AND the same spec passes under
//! a pack whose policy does not require it."*
//!
//! The type (`rk_types::acceptance::AcceptanceCriterion`; *dated 2026-09-29, s8g build: R2 =
//! ADR-0192 decision 4 retires the kernel-local shape*) carries `verification: Option<Text>` and
//! refuses nothing;
//! the pack's `[policy] verification_required = [<stages>]` is the row that decides, and this is
//! a pure function of the two — zero model cost by construction. The key's spelling is this
//! stage's reading and no record's (raised in the tracker); the `doctor` rule reporting the same at
//! configuration time is `rk-cli`'s and is RAISED as orphaned (`s7b-ii` §7b-ii.14).

use rk_types::acceptance::AcceptanceCriterion;

use crate::select::pack::Pack;

/// The stage was refused: which stage, which ACs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    pub stage: String,
    pub acs: Vec<String>,
}

impl std::fmt::Display for Refused {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "stage `{}` refused: the pack's policy requires a verification here and {} declare none \
             (AC-455)",
            self.stage,
            self.acs.join(", ")
        )
    }
}

impl std::error::Error for Refused {}

/// **The two rows over one spec.** If the pack's policy requires a verification at `stage`, every
/// AC at `verification: None` is named and the stage refused; otherwise the same spec passes.
///
/// # Errors
///
/// [`Refused`] naming the ACs at `verification: None` when the pack requires one at `stage`.
pub fn admissible(stage: &str, spec: &[AcceptanceCriterion], pack: &Pack) -> Result<(), Refused> {
    if !pack.requires_verification_at(stage) {
        return Ok(());
    }
    let acs: Vec<String> = spec
        .iter()
        .filter(|ac| ac.verification.is_none())
        .map(|ac| ac.id.clone())
        .collect();
    if acs.is_empty() {
        Ok(())
    } else {
        Err(Refused {
            stage: stage.to_owned(),
            acs,
        })
    }
}
