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
//! version. Each is a routing and Checkpoint 7a asserts none of them. *(Noted 2026-09-25, s8a fix
//! round 2, ADR-0165: "four" was `s7a`'s count. `ChangeSummary` is now an enum with a finalised
//! `Text` arm and a sealed `Binary` arm, so the crate holds three opaque newtypes, as
//! `event_store.rs` says.)* *(Corrected 2026-09-25, s8a fix round 3: four, not three. The sealed
//! `Binary` arm's payload, [`BinaryChangeSummary`], has the same crate-private unit field and no
//! public constructor, so the crate holds four opaque newtypes: `Snapshot`, `ExternalRef` and
//! `TypedInput` with no record behind them, and `BinaryChangeSummary`, the sealed slot ADR-0165
//! stands behind and `s8i` opens.)*

use rk_types::error::StepError;

/// What `classify` says about one path (design:269; AC-497's port shape turns on `Binary`,
/// register :7109).
///
/// **Routed to `rk-ports`, not decided — no record assigns it.** It is named and homed here
/// because `toolchain_fake_classifies_binary` has nothing to assert otherwise, and because its
/// only consumer before 7c is this port's own fake while `rk-types`' *"typed artifacts"* row
/// (design:1699) does not name it. **A record naming the home is OWED**, and Checkpoint 7a's
/// re-declaration ban deliberately does not cover it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    Text,
    Binary,
    Generated,
}

/// What `change_summary` returns: a **`Text`** arm, finalised, and a **`Binary`** arm whose payload
/// is a sealed slot until S-BIN reports (D21-42 decision 2; ADR-0165).
///
/// *(amended 2026-09-24, s8a, ADR-0165: this was `pub struct ChangeSummary(pub(crate) ());`, "an
/// opaque, non-finalised newtype until S-BIN reports" under ADR-0099 clause 5, and its doc said
/// "This is the one site in `crates/` the name may appear" and "no real adapter implements it at
/// this stage". Both sentences are false from this commit: ADR-0165 supersedes ADR-0099 clause 5,
/// ADR-0093 decision 4, ADR-0094 decision 1, ADR-0135 decisions 4 and 5 and ADR-0008 in part.)*
///
/// **The rule now.** The type is DEFINED only here. No crate outside `rk-ports` defines, aliases,
/// redefines or implements a trait for any `...ChangeSummary` type (Checkpoint 5's ban, as the
/// record restates it). An implementor of [`Toolchain::change_summary`] may import the type and
/// name it in that method's signature, and may construct the `Text` arm. `rk-kernel`'s production
/// code still names it nowhere (the surviving half of ADR-0135's fourth forbids bullet).
///
/// **Serialised form.** Untagged: a `Text` value serialises as its payload's four members at the
/// top level, because the shipped catalog reads them off the value itself
/// (`artifact:ChangeSummary.sha`, `${summarise.outputs.diff}`, `artifact:Changeset.files`).
/// *(Amended 2026-09-25, s8a fix round 10: this read "three members", before `files`.)* A `Binary` value cannot be
/// serialised: its payload refuses by name until S-BIN decides what it carries.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum ChangeSummary {
    /// Every path in the range classifies `Text` or `Generated`: the typed diff plus a manifest.
    Text(TextChangeSummary),
    /// The range carries a `Binary` path. The payload is sealed: `s8i` (S-BIN) fills it.
    Binary(BinaryChangeSummary),
}

/// The finalised `Text` arm (ADR-0165): the members the shipped catalog already reads.
///
/// - `sha` -- the range's head commit, 40 hex (`adversarial-review.toml`'s `clone` step reads
///   `artifact:ChangeSummary.sha`). That is the real adapter's value; `FixtureToolchain` echoes the
///   range's head token as written, unresolved (amended 2026-09-25, s8a fix round 1);
/// - `diff` -- the textual diff over the range (`change-summary.toml`'s `lint` step reads
///   `${summarise.outputs.diff}`);
/// - `manifest` -- one row per changed path: the path, its `classify` result, its change kind;
/// - `files` -- serialised only, derived from `manifest`: its paths, in manifest order
///   (`mutation-testing.toml` reads `artifact:Changeset.files`, and `Changeset` is declared
///   `schema = "ChangeSummary"`). *(Amended 2026-09-25, s8a fix round 10: the member list read
///   `sha`, `diff` and `manifest` only, and missed the `Changeset` alias.)* It is not a stored
///   field, so no implementor can construct a value whose `files` disagrees with its manifest.
///
/// Public members, because a `Toolchain` implementor constructs this arm. A value that misses a
/// member the catalog reads is a value that resolves nothing, so none is optional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextChangeSummary {
    /// The range's head commit.
    pub sha: String,
    /// The textual diff over the range.
    pub diff: String,
    /// One row per changed path.
    pub manifest: Vec<ManifestEntry>,
}

impl serde::Serialize for TextChangeSummary {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let files: Vec<&str> = self.manifest.iter().map(|e| e.path.as_str()).collect();
        let mut s = serializer.serialize_struct("TextChangeSummary", 4)?;
        s.serialize_field("sha", &self.sha)?;
        s.serialize_field("diff", &self.diff)?;
        s.serialize_field("files", &files)?;
        s.serialize_field("manifest", &self.manifest)?;
        s.end()
    }
}

/// One manifest row: a changed path, what `classify` says about it, and how it changed.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ManifestEntry {
    /// The path, relative to the repository root, `/`-separated.
    pub path: String,
    /// `Toolchain::classify`'s answer for the path. *(Amended 2026-09-25, s8a fix round 15: for
    /// the Rust adapter it is `classify`'s path rules with the bytes decided on the range's blobs
    /// (`RangeEntry::binary`), never on the working tree (s8a fix round 14, ADR-0165 decisions 3
    /// and 6), so it can differ from the `classify` verb's working-tree answer for the same path.)*
    pub classification: Classification,
    /// How the path changed over the range.
    pub change: ChangeKind,
}

/// How a path changed over a range. Rename tracking is not requested, so a rename reads as a
/// deletion and an addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    /// The path exists at the head end only.
    Added,
    /// The path exists at the base end only.
    Deleted,
    /// The path exists at both ends with different content or mode.
    Modified,
}

/// The `Binary` arm's payload: a **sealed slot** until S-BIN reports at `s8i` (ADR-0165).
///
/// The type is public so the enum can name it; its one field is `pub(crate)` and it has no
/// public constructor, no `Default` and no `From`, so no crate outside `rk-ports` can build one
/// (`tests/ui/change_summary_binary_not_constructible.rs` pins the privacy reason). Nothing inside
/// `rk-ports` constructs it either: [`crate::fakes::FixtureToolchain`] refuses a range carrying a
/// `Binary` path by name rather than answering this arm. **`s8i` supersedes this seal by name**
/// when S-BIN finalises what the arm carries; a later record opens the slot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryChangeSummary(pub(crate) ());

impl serde::Serialize for BinaryChangeSummary {
    fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom(
            "the Binary arm of ChangeSummary is sealed until S-BIN reports (ADR-0165); it carries \
             nothing to serialise",
        ))
    }
}

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
    ///
    /// *(amended 2026-09-25, s8a fix round 11, ADR-0165: the sentence above describes only the
    /// sealed `Binary` arm. Today the method answers [`ChangeSummary::Text`], which carries the
    /// range's diff itself plus a per-path manifest, over a range whose paths classify `Text` or
    /// `Generated`; a range carrying a `Binary` or LFS pointer path is refused naming the path
    /// until S-BIN opens the `Binary` arm at `s8i`. AC-497's instead-of-a-diff value is that
    /// sealed arm.)*
    fn change_summary(&self, range: &str) -> Result<ChangeSummary, StepError>;
}

/// **AC-337 — a toolchain pack declares mutation feasibility explicitly.** *"Per-AC mutation
/// evidence is required only where the toolchain declares mutation feasible"* (design §4.2;
/// ADR-0008's Unreal fixture is the canonical infeasible toolchain). The declaration is a type,
/// not a trait method: no record fixes a sixth port method, Checkpoint 7a counts exactly six on
/// this port, and a declaration is a fact about the PACK, read when the pack is loaded.
///
/// An absent declaration is a **configuration error** ([`FeasibilityError::Undeclared`]), never a
/// default in either direction — a pack that says nothing has said nothing. Where mutation is
/// infeasible, every AC's mutation-path status is `not-feasible(reason)`
/// ([`Self::ledger_status`]), which is **not `pinned`** and does not let a stage exit (AC-346);
/// AC-357 routes such an AC to a human gate.
///
/// The `[mutation] feasible = …, reason = …` table this is read from is the pack loader's
/// (`rk-kernel`); the key name is the runbook's fixture wording, not a record's, and is raised as
/// such.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutationFeasibility {
    Feasible,
    NotFeasible { reason: String },
}

/// How a feasibility declaration was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeasibilityError {
    /// The pack declares nothing — a configuration error, not a default (AC-337).
    Undeclared,
    /// `feasible = false` with no reason: the register's shape is `not-feasible(reason)`.
    InfeasibleWithoutReason,
    /// `feasible = true` with a reason, which means nothing and is refused rather than dropped.
    FeasibleWithReason,
}

impl std::fmt::Display for FeasibilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undeclared => write!(
                f,
                "the toolchain pack declares no mutation feasibility — `[mutation] feasible` is \
                 required, and its absence is a configuration error, not a default (AC-337)"
            ),
            Self::InfeasibleWithoutReason => write!(
                f,
                "`feasible = false` needs a `reason`: an AC records `not-feasible(reason)` (AC-337)"
            ),
            Self::FeasibleWithReason => {
                write!(
                    f,
                    "`feasible = true` carries a `reason`, which means nothing here"
                )
            }
        }
    }
}

impl std::error::Error for FeasibilityError {}

impl MutationFeasibility {
    /// Build the declaration from the pack's two members as the loader read them.
    ///
    /// # Errors
    ///
    /// [`FeasibilityError`] — absent, infeasible without a reason, or feasible with one.
    pub fn declared(
        feasible: Option<bool>,
        reason: Option<&str>,
    ) -> Result<Self, FeasibilityError> {
        let reason = reason.map(str::trim).filter(|r| !r.is_empty());
        match (feasible, reason) {
            (None, _) => Err(FeasibilityError::Undeclared),
            (Some(false), None) => Err(FeasibilityError::InfeasibleWithoutReason),
            (Some(false), Some(reason)) => Ok(Self::NotFeasible {
                reason: reason.to_owned(),
            }),
            (Some(true), None) => Ok(Self::Feasible),
            (Some(true), Some(_)) => Err(FeasibilityError::FeasibleWithReason),
        }
    }

    /// The status an infeasible toolchain fixes for every AC's mutation path — `not-feasible`,
    /// which is not `pinned`. A feasible toolchain fixes nothing: the flip predicate decides per
    /// AC (AC-334).
    #[must_use]
    pub fn ledger_status(&self) -> Option<rk_types::artifact::ledger::LedgerStatus> {
        match self {
            Self::Feasible => None,
            Self::NotFeasible { reason } => {
                Some(rk_types::artifact::ledger::LedgerStatus::NotFeasible {
                    reason: rk_types::text::Text::new(reason.clone()),
                })
            }
        }
    }
}
