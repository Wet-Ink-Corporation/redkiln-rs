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
//! stands behind and `s8i` opens.)* *(Noted 2026-10-14, s8i, ADR-0199, proposed: `s8i` has
//! opened it. `BinaryChangeSummary` now carries S-BIN's four recorded fields behind one checked
//! constructor, so it is no longer one of the crate's opaque newtypes.)*
//! *(Dated 2026-09-26, s8c: `test`'s selection type is decided now. Record T = ADR-0169 decision
//! 7, proposed, replaces `&[String]` with the closed [`Selection`], and `test` returns a
//! [`TestOutcome`]; `build` returns a `BuildReport`; every step-reached method takes a
//! [`StepBound`] (record B = ADR-0171); and record V = ADR-0170 adds the seventh method,
//! [`Toolchain::run_verb`]. `change_summary`'s range type is still the `&str` routing.)*

use std::collections::BTreeMap;
use std::num::NonZeroU64;

use rk_types::error::{StepError, TransportKind};
use rk_types::verb_output::BuildReport;
use rk_types::verify::{BasisReason, VerifyReport};

mod binary;

pub use binary::{
    BinaryAsset, BinaryCapture, BinaryChangeSummary, BinaryChangeSummaryError, PointerDefect,
};

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
/// *(Amended 2026-10-14, s8i, ADR-0199, proposed: S-BIN has decided. A `Binary` value serialises
/// as its payload's four members at the top level, `assets`, `captures`, `tests` and `build`
/// ([`BinaryChangeSummary`]).)*
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(untagged)]
pub enum ChangeSummary {
    /// Every path in the range classifies `Text` or `Generated`: the typed diff plus a manifest.
    Text(TextChangeSummary),
    /// The range carries a `Binary` path: the four fields S-BIN fixed (ADR-0199), built only
    /// through [`BinaryChangeSummary::new`].
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

/// A step's declared bound (record B = ADR-0171 decision 3, proposed).
///
/// It is handed to every `Toolchain` method a `tool` step reaches: `build`, `test`, `lint`, `change_summary` and `run_verb`.
/// `detect` takes none (adapter selection at construction, ADR-0137 clause 2) and neither does
/// `classify` (it spawns nothing).
///
/// `Undeclared` leaves the adapter's construction bound alone; `Secs(n)` is the step's own
/// `timeout_secs`, and the adapter enforces `min(step, construction)` on every child it spawns for
/// the call (record B decision 4). **A zero bound is refused** (RT-5: *"a zero bound is no
/// bound"*): the payload is a `NonZeroU64`, so `Secs(0)` is unrepresentable and [`Self::secs`]
/// refuses `0` by name.
///
/// *(Added 2026-09-26, s8c §8c.6.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepBound {
    /// The step declares no `timeout_secs`.
    Undeclared,
    /// The step's own bound, in seconds.
    Secs(NonZeroU64),
}

impl StepBound {
    /// The bound for a declared `timeout_secs`.
    ///
    /// # Errors
    ///
    /// `Unsupported` naming `timeout_secs = 0` and RT-5 for a zero bound.
    pub fn secs(n: u64) -> Result<Self, StepError> {
        NonZeroU64::new(n)
            .map(Self::Secs)
            .ok_or_else(|| StepError::Unsupported {
                capability:
                    "timeout_secs = 0: a zero bound is no bound (RT-5; ADR-0171 decision 3)"
                        .to_owned(),
            })
    }

    /// The declared seconds, or `None` when undeclared.
    #[must_use]
    pub const fn as_secs(self) -> Option<u64> {
        match self {
            Self::Undeclared => None,
            Self::Secs(n) => Some(n.get()),
        }
    }
}

/// The most recent recorded whole-suite count the caller read from the store, for an `Affected`
/// selection (record T decision 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordedWholeSuite {
    /// The run that recorded it.
    pub run_id: String,
    /// The toolchain version it was recorded at.
    pub toolchain_version: String,
    /// The whole-suite count.
    pub count: u32,
}

/// What `test` runs: a CLOSED selection (record T = ADR-0169 decision 7, proposed).
///
/// It replaces the
/// `&[String]` `s7a` §7a.3 called a routing, because no declared selection string can be passed
/// through (ADR-0148: *"`cargo test affected` matches nothing and **exits 0**"*).
///
/// *(Added 2026-09-26, s8c §8c.3.)*
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selection {
    /// The whole suite; the reason travels with the call (AC-352's closed six).
    WholeSuite { reason: BasisReason },
    /// Runner arguments: what `review_pass::selection_of` produces from a repro's `--` tail
    /// (ADR-0162).
    Filter(Vec<String>),
    /// The change set, and the most recent recorded whole-suite count.
    Affected {
        changed: Vec<String>,
        recorded: Option<RecordedWholeSuite>,
    },
    /// A selection the adapter DECLARES; an undeclared name is refused by name.
    Named(String),
}

/// A test outcome's verdict, DERIVED from its counts and never stored independently.
///
/// Record T decision 1: `passing` iff `failed == 0 && passed > 0`, `failing` iff `failed > 0`, `empty` iff
/// nothing executed. The serde form is the vocabulary the catalog's 25 `status == 'passing'`
/// comparisons use, so an empty run never satisfies a check (AC-332).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    Passing,
    Failing,
    Empty,
}

impl TestStatus {
    /// The status the counts fix.
    #[must_use]
    pub const fn of(passed: u32, failed: u32) -> Self {
        if failed > 0 {
            Self::Failing
        } else if passed > 0 {
            Self::Passing
        } else {
            Self::Empty
        }
    }
}

/// Which runner ran (D21-48 X15: *"records on the report which runner ran"*; record T decision 1).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TestRunner {
    /// `cargo-test`, `cargo-nextest`, `vitest`, or `fixture` for the fake. Never empty.
    pub name: String,
    /// The runner's version.
    pub version: String,
    /// The doctest companion child's vector, when the runner runs no doctests itself (record T
    /// decision 5); `None` when no companion ran.
    pub companion_argv: Option<Vec<String>>,
    /// The libtest parser's count of foreign lines it skipped (D21-62 A, record T decision 3):
    /// a count from the parser, never a parsed prose value.
    pub foreign_lines: u32,
}

impl TestRunner {
    /// A runner with no companion and no foreign line.
    #[must_use]
    pub fn named(name: &str, version: &str) -> Self {
        Self {
            name: name.to_owned(),
            version: version.to_owned(),
            companion_argv: None,
            foreign_lines: 0,
        }
    }
}

/// `test`'s outcome (record T = ADR-0169 decision 1, proposed): the unchanged
/// [`VerifyReport`] plus the per-test counts, the derived status and the runner.
///
/// **`Ok` means the suite ran and every executed test reported, green or red.** A red suite is
/// `Ok(TestOutcome { failed ≥ 1, status: Failing, … })`. `Err` means exactly three things: cannot
/// run (`Transport { Spawn }`, or `NonZero` with no per-test record), killed (`Timeout`, or the
/// `os-killed:` `Transport { Io }`), or unreadable (`Transport { Io | Decode }`). `VerifyReport`
/// gains no member (AC-329's list is closed).
///
/// Construct through [`Self::new`], which derives `status` and checks the invariant
/// `passed + failed == report.selected_test_count`. The members are public so a reader can match
/// on them; `rk_ports::conformance::validate_report` re-checks both over a value however it was
/// built.
///
/// *(Added 2026-09-26, s8c §8c.2.)*
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct TestOutcome {
    /// The report, unchanged (AC-329).
    pub report: VerifyReport,
    /// Executed tests that passed.
    pub passed: u32,
    /// Executed tests that failed.
    pub failed: u32,
    /// Derived from `passed` and `failed` ([`TestStatus::of`]).
    pub status: TestStatus,
    /// Which runner ran.
    pub runner: TestRunner,
}

impl TestOutcome {
    /// The outcome, with `status` derived and the count invariant checked.
    ///
    /// # Errors
    ///
    /// `Transport { Decode }` naming both numbers when `passed + failed` is not the report's
    /// `selected_test_count`; never a corrected count.
    pub fn new(
        report: VerifyReport,
        passed: u32,
        failed: u32,
        runner: TestRunner,
    ) -> Result<Self, StepError> {
        let executed = passed.checked_add(failed);
        if executed != Some(report.selected_test_count) {
            return Err(StepError::Transport {
                kind: TransportKind::Decode,
                detail: format!(
                    "passed + failed ({passed} + {failed}) is not the report's selected_test_count \
                     ({}): the per-test records and the --list count disagree, so the run is \
                     unreadable (record T decision 1)",
                    report.selected_test_count
                ),
            });
        }
        Ok(Self {
            report,
            passed,
            failed,
            status: TestStatus::of(passed, failed),
            runner,
        })
    }
}

/// One `run_verb` call (record V = ADR-0170 decision 2, proposed): the verb, its resolved inputs as
/// JSON values by name, and the step's bound.
///
/// *(Added 2026-09-26, s8c §8c.4.)*
#[derive(Debug, Clone, Copy)]
pub struct VerbCall<'a> {
    /// The registry verb.
    pub verb: &'a str,
    /// The resolved inputs, by name.
    pub inputs: &'a BTreeMap<String, serde_json::Value>,
    /// The step's bound (record B).
    pub bound: StepBound,
}

/// A `run_verb` output, ERASED at the port (record V decision 2).
///
/// It carries the kind the adapter says it produced and the value. The kernel types it at the boundary through a closed decode table; an
/// output whose `kind` is not the row's `outputs` is refused naming both kinds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbOutput {
    /// The output kind (a registry row's `outputs`).
    pub kind: String,
    /// The value, decoded by the kernel.
    pub value: serde_json::Value,
}

/// The build toolchain. The kernel knows no language.
pub trait Toolchain: Send + Sync {
    /// Is this toolchain present for the project under test?
    fn detect(&self) -> Result<bool, StepError>;

    /// Build. A red build is `Ok(BuildReport { status: failing, .. })`; `Err` means the build
    /// could not run, was killed, or its stream was unreadable (record T = ADR-0169 decision 8,
    /// proposed; D21-43 N25). *(Amended 2026-09-26, s8c: this was `fn build(&self) -> Result<(),
    /// StepError>`, so a red build was `Err` and `build-package`'s `status == 'passing'` could
    /// never see one.)*
    fn build(&self, bound: StepBound) -> Result<BuildReport, StepError>;

    /// Run the selected tests. A red suite is `Ok(TestOutcome { failed ≥ 1, .. })`; `Err` is only
    /// cannot-run, killed or unreadable (record T decision 1; D21-42 1a). *(Amended 2026-09-26,
    /// s8c: this was `fn test(&self, selection: &[String]) -> Result<u32, StepError>`, "the count
    /// that ran comes back", `Ok` only on a green suite (RT-22, now reversed by record T).)*
    fn test(&self, selection: &Selection, bound: StepBound) -> Result<TestOutcome, StepError>;

    /// Lint.
    fn lint(&self, bound: StepBound) -> Result<(), StepError>;

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
    fn change_summary(&self, range: &str, bound: StepBound) -> Result<ChangeSummary, StepError>;

    /// **The seventh method** (record V = ADR-0170 decision 2, proposed; D21-42 1b): a registry
    /// row routed `toolchain` reaches its verb here. The output is erased at the port and typed
    /// by the kernel at the boundary. An adapter implements the verbs it can and refuses every
    /// other one BY NAME (`Unsupported { capability: "<verb>" }`). It is not a patch applier:
    /// no `mutate` row lands, and ADR-0162's refusal of `ReproKind::Mutate` stands.
    fn run_verb(&self, call: &VerbCall<'_>) -> Result<VerbOutput, StepError>;
}

/// **AC-337 — a toolchain pack declares mutation feasibility explicitly.** *"Per-AC mutation
/// evidence is required only where the toolchain declares mutation feasible"* (design §4.2;
/// ADR-0008's Unreal fixture is the canonical infeasible toolchain). The declaration is a type,
/// not a trait method: no record fixes a sixth port method, Checkpoint 7a counts exactly six on
/// this port, and a declaration is a fact about the PACK, read when the pack is loaded.
///
/// *(Corrected 2026-09-26, s8c §8c.7: "Checkpoint 7a counts exactly six on this port" is false.
/// Checkpoint 7a counts no method; it floors the NAMES `fn detect` and `fn lint` (`s7a` §7a.3:
/// *"The floor pins **names**, never a signature"*; E-0162-2). Under record V = ADR-0170,
/// proposed, the port has SEVEN methods, and the exact count is
/// `source_parse::toolchain_port_declares_exactly_seven_methods`'s. The declaration is still a
/// type and not a method: it is a fact about the pack.)*
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
