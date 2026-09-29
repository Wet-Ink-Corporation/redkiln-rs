//! The typed outputs of the toolchain verbs (s8c §8c.4 "Typed outputs"; record V = ADR-0170,
//! proposed; `BuildReport` is record T's, ADR-0169 decision 8, proposed).
//!
//! `BuildReport` (the `build` port method's outcome), `SmokeResult`, `ScanReport`, `BuildArtifact`
//! and `SetupResult` (the Rust adapter's `smoke`, `sca_scan`, `package` and `setup` handlers),
//! `ItemRefs` (`item_create`, a `store` row, `s8f`'s) and `ReleaseTag` (`release_tag`, a `git` row,
//! `s8g`'s). The kernel types a `run_verb` output at the boundary through a closed decode table
//! that maps each registry row's declared `outputs` kind to one of these (record V decision 2).
//!
//! Every struct is `#[serde(deny_unknown_fields)]`, has **no `Default`**, and requires **every
//! member**: an `Option` member is required PRESENT (written `null` when there is no value), never
//! inferred from absence, which is why each one carries `deserialize_with = "Option::deserialize"`
//! (serde otherwise reads a missing `Option` as `None`). They are **not in `EXPORTED`**: which
//! kinds an agent emits is `s8h`'s (D21-41 g). They are kept `oneOf`-free anyway, because
//! Checkpoint 5 greps the snapshot directory for it: no enum here carries per-variant docs or a
//! tag, and each status enum is a plain string set.
//!
//! **What no record fixes, named rather than decided:** the member sets of `ItemRefs` and
//! `ReleaseTag`. Record V lands their rows as refused-naming-the-route until `s8f` and `s8g`, and
//! gives them no members. The sets here are the smallest the owning stages' texts read (`s8f`
//! §8f.7: *"a journalled `ItemRefs` output of an `item_create` step lists it"*; `s8g` §8g.6: *"a
//! check that the record names a tag and a rollback target"*, and *"A first release names its
//! target explicitly as `none`"*); each owner widens its own by record.
//!
//! *(Added 2026-09-26, s8c.)*

use serde::{Deserialize, Serialize};

/// `build`'s verdict (record T decision 8): the build compiled, or it did not. A build that could
/// not run is `Err`, never a status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum BuildStatus {
    Passing,
    Failing,
}

/// `build`'s outcome report (record T decision 8).
///
/// D21-43 N25: *"`build` is widened to an outcome report as 1a did for `test`"*. A compile error is `Ok(BuildReport { status: failing, errors ≥
/// 1 })`; `Err` means only that the build could not run, was killed, or its stream was unreadable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BuildReport {
    /// The build vector as handed to the OS (AC-325's rule, carried over).
    pub argv: Vec<String>,
    /// The toolchain's detected version string.
    pub toolchain_version: String,
    /// The build's wall time.
    pub wall_secs: u64,
    /// The count of error-level diagnostics.
    pub errors: u32,
    /// `passing` or `failing`.
    pub status: BuildStatus,
}

/// A smoke run's verdict over its rows: every row passed, at least one failed, or there were none.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SmokeStatus {
    Passing,
    Failing,
    Empty,
}

/// One checklist row as the smoke handler ran it (record V decision 4; the item form is `s8g`
/// §8g.6's: `argv`, `cwd`, `expect_exit`, one stdout predicate).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SmokeRow {
    /// The checklist item's id.
    pub id: String,
    /// The vector run, against the binaries the call's build produced.
    pub argv: Vec<String>,
    /// The working directory it ran in (the resolved scratch directory for `cwd = "scratch"`).
    pub cwd: String,
    /// The exit status the item expects.
    pub expect_exit: i32,
    /// The exit status observed.
    pub exit: i32,
    /// The item's one stdout predicate, as declared.
    pub predicate: String,
    /// Whether the predicate held over the row's stdout.
    pub predicate_held: bool,
    /// The row's verdict: the exit matched and the predicate held.
    pub passed: bool,
}

/// `smoke`'s output (record V decision 4). A failing row is `Ok` with `failed ≥ 1`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SmokeResult {
    /// One row per checklist entry, in checklist order.
    pub rows: Vec<SmokeRow>,
    /// Rows that passed.
    pub passed: u32,
    /// Rows that failed.
    pub failed: u32,
    /// Derived from the counts.
    pub status: SmokeStatus,
}

/// A scored advisory's CVSS v3 qualitative band (record V decision 5): `critical` ≥ 9.0, `high`
/// 7.0–8.9, `medium` 4.0–6.9, `low` 0.0–3.9. An unscored advisory has no band.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

/// One advisory a scan tool reported against the lockfile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Advisory {
    /// The advisory id (`RUSTSEC-…`).
    pub id: String,
    /// The affected package.
    pub package: String,
    /// The locked version.
    pub version: String,
    /// The band, or `null` when the advisory carries no CVSS: counted in `unrated`, never `low`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub severity: Option<Severity>,
}

/// One scan tool and the database it read (D21-42 6a: *"the advisory-DB commit is recorded"*;
/// record V decision 5: the two databases are two pins).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScanTool {
    /// `cargo-audit` or `cargo-deny`.
    pub name: String,
    /// The tool's version.
    pub version: String,
    /// The advisory database's local path the adapter gave the tool.
    pub db_path: String,
    /// That database's `HEAD`, read by the adapter before and after the tool ran (D21-62 C).
    pub db_commit: String,
}

/// `sca_scan`'s output (record V decision 5). **`unrated` is required**: an advisory with no
/// severity is counted there, never as low or none, and never dropped (D21-43 N25).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ScanReport {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    /// Advisories with no CVSS.
    pub unrated: u32,
    /// One entry per advisory.
    pub advisories: Vec<Advisory>,
    /// Per tool: its name, version, database path and database commit.
    pub tools: Vec<ScanTool>,
}

/// One declared shipped binary of a package build.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ArtifactBinary {
    /// The declared binary's name.
    pub name: String,
    /// Its path relative to the adapter's cargo target directory (e.g. `release/<name>`), as the
    /// Rust handler records it (record V = ADR-0170 decision 6, proposed).
    pub path: String,
    /// `rk_types::digest::sha256` over its bytes, the plain hash an independent `sha256sum` agrees
    /// with.
    pub sha256: String,
}

/// `package`'s output (record V decision 6). Only the DECLARED binaries are digested, never a glob.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BuildArtifact {
    /// The build profile (`release`).
    pub profile: String,
    /// One row per declared binary.
    pub binaries: Vec<ArtifactBinary>,
    /// The build vector as handed to the OS.
    pub argv: Vec<String>,
    /// The toolchain's detected version string.
    pub toolchain_version: String,
    /// The input `sha` the call named.
    pub sha: String,
    /// The project root's `HEAD`, read through `rk-git`; it must equal `sha`.
    pub head: String,
}

/// One declared project `setup` installed (record V decision 7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SetupProject {
    /// The project's path, relative to the worktree root.
    pub path: String,
    /// The install vector as handed to the OS.
    pub argv: Vec<String>,
    /// sha256 of the lockfile the install ran from.
    pub lockfile_sha256: String,
}

/// `setup`'s output (record V decision 7). It has no `status`: a failed install is never `Ok`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SetupResult {
    /// The toolchain it ran for.
    pub toolchain: String,
    /// One row per declared project, in declaration order.
    pub projects: Vec<SetupProject>,
}

/// `item_create`'s output: the ids of the items the step minted (`s8f`'s; the module doc).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ItemRefs {
    /// The minted item ids.
    pub ids: Vec<String>,
}

/// What named an id in the backlog: a tier-1 `stage_transition` row carrying it as `item`, or an
/// `item_create` step's journalled `ItemRefs` output (`s8f` §8f.7). The declaration order is the
/// sort order. *(Added 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; the render record
/// ADR-0189 decision 12, proposed.)*
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum NamedBy {
    StageTransition,
    ItemWrite,
}

/// One item record the store names and this worktree holds (`s8f` §8f.7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BacklogItem {
    /// The item id.
    pub id: String,
    /// The record's `kind`, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub kind: Option<String>,
    /// The record's `stage`, or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub stage: Option<String>,
    /// The record's `[facets]` table verbatim: an absent facet stays absent, never defaulted.
    pub facets: serde_json::Map<String, serde_json::Value>,
    /// The record's `[provenance]` table (the item write's), or `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub provenance: Option<serde_json::Value>,
    /// The `rk-types` digest of the record's bytes.
    pub record_digest: String,
    /// What named it, sorted.
    pub named_by: Vec<NamedBy>,
}

/// A named id with no record in this worktree: listed, never dropped (`s8f` §8f.7).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BacklogAbsent {
    /// The item id.
    pub id: String,
    /// What named it, sorted.
    pub named_by: Vec<NamedBy>,
}

/// `backlog`'s output: the item records the store names, read by exact path (`s8f` §8f.7; D21-46
/// R7). Serialised through `rk_store::canon::canonical`. *(Added 2026-09-28, s8f build, package
/// S8F-P6-ITEMS-BACKLOG-FEEDS.)*
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct BacklogSnapshot {
    /// The named ids with a record, ordered by `id`.
    pub items: Vec<BacklogItem>,
    /// The named ids with no record, ordered by `id`.
    pub absent: Vec<BacklogAbsent>,
}

/// `release_tag`'s output: the local tag written, its target, and the rollback target (`s8g`'s;
/// the module doc). A first release names its rollback target explicitly as `none`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReleaseTag {
    /// The tag name.
    pub tag: String,
    /// The commit it points at.
    pub target: String,
    /// The previous release's tag, or `none`.
    pub rollback_target: String,
}
