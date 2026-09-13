//! The `Toolchain` port — *"detect; `build`; `test(selection)`; `lint`; `classify(path) ->
//! Text/Binary/Generated`; `change_summary(range) -> ChangeSummary`"* (design:269).
//!
//! The spike has build/test/classify/change_summary, **no `detect` and no `lint`**, and spells
//! `classify(&self, paths: &[String]) -> Vec<String>` where design:269 and AC-497's header both
//! fix `Text`/`Binary`/`Generated`. The design's column is the authority, not the spike.
//!
//! **What no record fixes and this crate does not pretend it does:** `test`'s selection type and
//! `change_summary`'s range type — taken here as `&[String]` and `&str` rather than as two further
//! opaque newtypes, because `s7a` §7a.3 fixes the cost at **four** opaque newtypes and a fifth and
//! sixth would be two more undecided things to reverse — and whether `detect` reports a bool or a
//! version. Each is a routing and Checkpoint 7a asserts none of them.

use rk_types::error::StepError;

/// What `classify` says about one path (design:269; AC-497's port shape turns on `Binary`,
/// register :7109).
///
/// **Routed to `rk-ports`, not decided — no record assigns it.** It is named and homed here
/// because `toolchain_fake_classifies_binary` has nothing to assert otherwise, and because its
/// only consumer before 7c is this port's own fake while `rk-types`' *"typed artifacts"* row
/// (design:1699) does not name it. **A record naming the home is OWED**, and Checkpoint 7a's
/// re-declaration ban deliberately does not cover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Text,
    Binary,
    Generated,
}

/// `change_summary` returns an opaque, non-finalised newtype until S-BIN reports (**ADR-0099
/// clause 5**, :90-91).
///
/// **This is the one site in `crates/` the name may appear** (ADR-0099's forbids bullet at :131),
/// and Checkpoint 5's grep for its absence covers `crates/` minus exactly this declaration. The
/// rule is about where the **type** lives, not about one textual occurrence: the shape necessarily
/// carries the name on `change_summary`'s return type and on the fake's impl too.
///
/// **Its body is not written here.** That is 7c's, behind S-BIN (ADR-0093 decision 4): the
/// Phase-1 fake returns `ChangeSummary(())` and no real adapter implements it at this stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChangeSummary(pub(crate) ());

/// The build toolchain. The kernel knows no language.
pub trait Toolchain: Send + Sync {
    /// Is this toolchain present for the project under test?
    fn detect(&self) -> Result<bool, StepError>;

    /// Build.
    fn build(&self) -> Result<(), StepError>;

    /// Run the selected tests; the count that ran comes back.
    fn test(&self, selection: &[String]) -> Result<u32, StepError>;

    /// Lint.
    fn lint(&self) -> Result<(), StepError>;

    /// Text, binary or generated — **per path**, as design:269 spells it.
    fn classify(&self, path: &str) -> Result<Classification, StepError>;

    /// What changed over a range, as the typed value a reviewer receives instead of a diff when a
    /// path classifies as `Binary` (AC-497).
    fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError>;
}
