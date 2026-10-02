//! The agent-step output kinds the two shipped lanes reach.
//!
//! `s8h` §8h.4's universe of 24, less `Findings`, which `artifact::findings` already declares and is registered, never redeclared.
//! *(Added 2026-10-02, `s8h` build, package P2-rk-types; Record A = ADR-0197 decisions 6 and 7,
//! proposed.)*
//!
//! **Transcription.** Field names and list flags are `spikes/p9/schemas.toml`'s `[value.<Kind>]`
//! rows (*"Stand-in for schemars types"*); the Rust types are the build's, under ADR-0095 decision
//! 1's rule: no union nested in a union, `additionalProperties: false` (`deny_unknown_fields`),
//! every required member before any optional one, and the vendor caps. **Every member the stand-in
//! lists is required** — an `Option` is never a default (Record A decision 6) — so no type here
//! declares an optional member.
//!
//! **Typing rule** (stated so it reproduces). A stand-in list is a `Vec`. A scalar is:
//! - a closed enum when a shipped `expr`/`when`/`until` compares it to a literal (`Confidence`,
//!   `ReproStatus`, `HypothesisStatus`), or when the catalog, a checklist or a record names its
//!   complete vocabulary (`DocsQuadrant`, `ArchitectureLevel`, `CausalMethod`, `EstimateMethod`,
//!   `RolloutStrategy`, `ChangeType`, `AdrStatus`);
//! - `bool` or an unsigned count when a shipped expression reads it as one (`fits_appetite`,
//!   `SelfCheck.passed`/`failed`);
//! - a plain `String` when it is a machine token a later step consumes (a command, a test
//!   selection, a path, an AC id);
//! - free [`Text`] otherwise.
//!
//! **AC-133.** Every enum here is entered in `schema::judgement::REGISTRY` (the one registry,
//! `s8g`'s). Where an enum is drafted judgement-bearing and the stand-in gives its object no
//! `rationale`, the type gains a REQUIRED free-text `rationale` (Record A decision 7), declared last.

use crate::acceptance::AcceptanceCriterion;
use crate::text::Text;
use serde::{Deserialize, Serialize};

// ---- enums --------------------------------------------------------------------------------------

/// The author's confidence in a conclusion. `high` is the spelling the shipped loops compare
/// (`differential-debugging`, `instrumented-trace-analysis`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

/// Whether an attempt reproduced the defect. `reproduced` is the spelling `minimal-repro` compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ReproStatus {
    Reproduced,
    NotReproduced,
}

/// Where a hypothesis stands. `confirmed` is the spelling `root-cause` compares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum HypothesisStatus {
    Open,
    Confirmed,
    Refuted,
}

/// The Diataxis quadrant a page belongs to (the `docs-diataxis` checklist: tutorial, how-to,
/// reference or explanation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum DocsQuadrant {
    Tutorial,
    HowTo,
    Reference,
    Explanation,
}

/// The C4 level an architecture description is drawn at (`c4-context`, `c4-container`,
/// `c4-component`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ArchitectureLevel {
    Context,
    Container,
    Component,
}

/// The causal-analysis method a chain was built by (`five-whys`, `fishbone-analysis`,
/// `fault-tree-analysis`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CausalMethod {
    FiveWhys,
    Fishbone,
    FaultTree,
}

/// The estimation method (`appetite-check`, `t-shirt-sizing`, `relative-estimation`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum EstimateMethod {
    Appetite,
    TShirt,
    Relative,
}

/// The deployment pattern a rollout plan chooses (the release stage's `blue-green-deploy`,
/// `canary-release`, `flag-progressive-rollout` and `rolling-deploy`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RolloutStrategy {
    BlueGreen,
    Canary,
    FlagProgressive,
    Rolling,
}

/// The ITIL 4 change type (`change-authorization`: standard, normal, emergency).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ChangeType {
    Standard,
    Normal,
    Emergency,
}

/// An architecture decision record's lifecycle state (`references/adr/README.md`: proposed,
/// accepted, then rejected, deprecated or superseded).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum AdrStatus {
    Proposed,
    Accepted,
    Rejected,
    Deprecated,
    Superseded,
}

// ---- the kinds, in s8h section 8h.4's table order --------------------------------------------

/// An architecture decision record (`adr-record`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "ADR")]
pub struct Adr {
    pub title: Text,
    pub context: Text,
    pub decision: Text,
    pub options: Vec<Text>,
    pub consequences: Vec<Text>,
    pub status: AdrStatus,
}

/// An API contract (`api-contract-spec`). `document` is the contract text the lint step reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(rename = "APIContract")]
pub struct ApiContract {
    pub format: Text,
    pub document: Text,
    pub endpoints: Vec<Text>,
    pub uncovered_ac: Vec<String>,
    pub breaking_changes: Vec<Text>,
    pub summary: Text,
}

/// A C4 architecture description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArchitectureDescription {
    pub level: ArchitectureLevel,
    pub elements: Vec<Text>,
    pub relationships: Vec<Text>,
    pub diagram: Text,
    pub assumptions: Vec<Text>,
    pub summary: Text,
}

/// A causal chain from the technical cause to the systemic one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CausalChain {
    pub method: CausalMethod,
    pub whys: Vec<Text>,
    pub categories: Vec<Text>,
    pub systemic_cause: Text,
    pub confidence: Confidence,
    pub summary: Text,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the confidence is what it is.
    pub rationale: Text,
}

/// A recorded change (`change-authorization`). `status` is the authorisation outcome, composed
/// from the gate's answer at emit, so it is free text here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ChangeRecord {
    #[serde(rename = "type")]
    pub change_type: ChangeType,
    pub authorized_by: Text,
    pub window: Text,
    pub rollback: Text,
    pub status: Text,
    pub summary: Text,
}

/// A technical-debt register entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DebtRegisterEntry {
    pub items: Vec<Text>,
    pub interest: Text,
    pub remediation: Text,
    pub summary: Text,
}

/// A design document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DesignDoc {
    pub summary: Text,
    pub components: Vec<Text>,
    pub interfaces: Vec<Text>,
}

/// The docs a change lands, in their quadrant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Docs {
    pub quadrant: DocsQuadrant,
    pub files: Vec<String>,
    pub summary: Text,
    pub stale: Vec<String>,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the pages sit in that quadrant.
    pub rationale: Text,
}

/// An estimate. `fits_appetite` is the boolean `appetite-check` reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Estimate {
    pub method: EstimateMethod,
    pub sizes: Vec<Text>,
    pub total: Text,
    pub fits_appetite: bool,
    pub confidence: Confidence,
    pub summary: Text,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the confidence is what it is.
    pub rationale: Text,
}

/// A fix design: where the fix goes and what guards it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct FixDesign {
    pub location: Text,
    pub guard: Text,
    pub workaround: Text,
    pub regression_risk: Text,
    pub files: Vec<String>,
    pub summary: Text,
}

/// Gherkin scenarios. Its criteria are `AcceptanceCriterion`, embedded unchanged, as `Spec`'s are.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct GherkinScenarios {
    pub features: Vec<Text>,
    pub scenarios: Vec<Text>,
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    pub story: Text,
    pub open_questions: Vec<Text>,
}

/// The implementer's report. `status` is the implementer's own word; the verify pass measures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ImplReport {
    pub status: Text,
    pub files: Vec<String>,
    pub summary: Text,
    pub tests_added: Vec<String>,
}

/// A blameless postmortem.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Postmortem {
    pub summary: Text,
    pub impact: Text,
    pub timeline: Text,
    pub root_causes: Vec<Text>,
    pub contributing_factors: Vec<Text>,
    pub went_well: Vec<Text>,
    pub went_badly: Vec<Text>,
    pub lucky: Vec<Text>,
    pub action_items: Vec<Text>,
    pub unassigned_items: Vec<Text>,
    pub status: Text,
}

/// One attempt to reproduce a defect. `command` is the selection a later step re-runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReproAttempt {
    pub status: ReproStatus,
    pub command: String,
    pub observed: Text,
    pub notes: Text,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the observation is read as reproduced or not.
    pub rationale: Text,
}

/// A minimal reproduction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReproCase {
    pub status: ReproStatus,
    pub steps: Vec<Text>,
    pub command: String,
    pub expected: Text,
    pub observed: Text,
    pub confidence: Confidence,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the status and confidence are what they are.
    pub rationale: Text,
}

/// A retrospective.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Retrospective {
    pub went_well: Vec<Text>,
    pub to_change: Vec<Text>,
    pub action_items: Vec<Text>,
    pub unassigned_items: Vec<Text>,
    pub summary: Text,
}

/// A rollout plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RolloutPlan {
    pub strategy: RolloutStrategy,
    pub steps: Vec<Text>,
    pub thresholds: Text,
}

/// A root cause. `discriminating_test` is the test selection `confirm` re-runs (D21-47 cross #5),
/// a required string.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RootCause {
    pub cause: Text,
    pub contributing_factors: Vec<Text>,
    pub confidence: Confidence,
    pub summary: Text,
    pub discriminating_test: String,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the confidence is what it is.
    pub rationale: Text,
}

/// A self-check against a checklist. `passed` and `failed` are the counts the shipped
/// `check.failed == 0` rows read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SelfCheck {
    pub checklist: String,
    pub passed: u32,
    pub failed: u32,
    pub notes: Text,
}

/// A specification. Its criteria are `rk_types::AcceptanceCriterion`, embedded unchanged under
/// the member name the shipped catalog reads (`s8g` section 8g.3; ADR-0192 decision 7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Spec {
    pub story: Text,
    pub acceptance_criteria: Vec<AcceptanceCriterion>,
    pub open_questions: Vec<Text>,
    pub boundary: Text,
}

/// A test design: conditions and cases derived per criterion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TestDesign {
    pub technique: Text,
    pub conditions: Vec<Text>,
    pub cases: Vec<Text>,
    pub uncovered_ac: Vec<String>,
    pub summary: Text,
}

/// A threat model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ThreatModel {
    pub assets: Vec<Text>,
    pub threats: Vec<Text>,
    pub mitigations: Vec<Text>,
    pub residual: Vec<Text>,
}

/// A hypothesis log (`root-cause`). Its `root_cause` is the `RootCause` the activity emits, and
/// `discriminating_test` the selection its `confirm` step re-runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HypothesisLog {
    pub status: HypothesisStatus,
    pub entries: Vec<Text>,
    pub discriminating_test: String,
    pub root_cause: RootCause,
    pub summary: Text,
    // AC-133's rationale sibling (Record A = ADR-0197 decision 7, proposed).
    /// Why the hypothesis stands where it does.
    pub rationale: Text,
}
