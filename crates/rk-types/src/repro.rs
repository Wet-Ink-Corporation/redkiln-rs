//! The typed reproduction record (ADR-0096 decision 1).
//!
//! ADR-0074's repro had no field for the mutant or the suite that AC-289/AC-305 re-execute, and a
//! prose sentence in `command` satisfied it. Both are closed here.

use serde::{Deserialize, Serialize};

/// `run` reproduces; `mutate` applies a patch, runs a suite and reverts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReproKind {
    Run,
    Mutate,
}

/// REQUIRED when `kind = mutate`: the unified diff and the suite id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Mutation {
    pub patch: String,
    pub suite: String,
}

/// Where an assertion lives, so a gate can watch it flip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Locator {
    pub file: String,
    pub test: String,
}

/// Machine-readable, beside the prose.
///
/// A `kind`-discriminated enum. On export it passes through `rk-types`' post-derive transform and
/// emerges as an `anyOf` (ADR-0095 decision 1); `schema::transform` carries the full reason, and
/// this text deliberately does not repeat the forbidden token — doc comments are serialised into
/// the schema as `description`, Checkpoint 5 greps the snapshot directory for that token as a
/// SUBSTRING, and it cannot tell a schema node from prose.
///
/// REQUIRED on `Repro`: wrapped in an `Option` it would become a union nested inside a union,
/// which AC-131 forbids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Expected {
    Exit { exit: i32 },
    Assertion { assertion: Locator },
}

/// The repro record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Repro {
    pub kind: ReproKind,
    /// A shell string. MUST begin with an executable token.
    pub command: String,
    pub expected: Expected,
    pub observed: String,
    pub expected_when_met: String,
    /// REQUIRED when `kind = mutate` (enforced at ingest, not by the schema).
    ///
    /// Declared LAST, and that is not cosmetic: AC-158 requires every required property to precede
    /// every optional one in the emitted order, and `mutation` is this struct's only optional
    /// field. ADR-0096 decision 1 lists the fields; it does not fix their order, and the register
    /// does. Declared in the record's own order this struct fails its own lint — which is how the
    /// defect was found.
    pub mutation: Option<Mutation>,
}
