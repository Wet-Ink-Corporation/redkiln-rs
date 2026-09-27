//! The **load-time** rules of catalog §2.6, split out from the runtime graph they protect.
//!
//! > At pack load: every stage-exit artifact has ≥1 admissible producer for every reachable
//! > context; every activity's `provides` is required by some row in some pack (**dead content is a
//! > warning**); **no `requires` cycle**; **`excludes` are symmetric**; no two producers tie at
//! > equal rank under any context; every `require` row's predicate uses only the closed vocabulary.
//! > — `references/2026-09-07-process-catalog-proposal.md`:127
//!
//! | rule | criterion | refuses? | who calls it |
//! | --- | --- | --- | --- |
//! | `requires` cycle | **AC-059** | yes, naming the path | [`load_lints`], from `select_amended` |
//! | `excludes` symmetry | **AC-060** | yes, `AsymmetricExcludes { a, b }` | [`load_lints`] |
//! | dangling `requires` | **AC-053** (load half) | yes, `DanglingRequires { activity_id, kind }` | [`load_lints`] |
//! | rank tie | **AC-047** | yes — the rule is [`super::step4::rank_lint`], not here | [`load_lints`] |
//! | dead content | **AC-057** | **no — a warning with a count and a list** | nothing yet: a report, not a refusal |
//! | forward-ref licence *(dated 2026-09-26, `s8d` build; D21-48 X8)* | **ADR-0042** as R2 (ADR-0173, `proposed`) widens it | yes, `UnlicensedForwardRef { refs }`, each naming activity, step and reference — the rule is [`crate::catalog::forward_ref_lint`], not here | [`load_lints`], first |
//!
//! # 🛑 THE "who calls it" COLUMN EXISTS BECAUSE NOTHING DID, AND THE TABLE SAID "yes" ANYWAY
//!
//! Until 2026-09-13 the four refusing rules had **no caller outside `tests/it/`**. The catalog
//! loader ran none of them, so what was discharged was *"a function that returns `Err` when a test
//! chooses to call it"* while the table's `fails the build? — yes` column was read as the shipped
//! behaviour. **Measured**: `Catalog::from_sources(DANGLING)` returned `Ok` with one activity, and
//! `select()` over it returned `Ok` with `unsat = ["Telemetry"]` — a catalog defect reported as an
//! ordinary unsatisfiable artifact, which `tests/fixtures/catalog-dangling/`'s own header forbids in
//! terms (*"this is a pack/catalog defect at LOAD and not a missing artifact at run time"*). AC-059
//! and AC-060 at least had runtime backstops in step 5 (`validate_acyclic` and the `ExcludedPair`
//! check); AC-053 had none. A lint nothing calls is Halt 8's shape.
//!
//! [`load_lints`] is the repair: `select_amended` runs all four before step 1, so every
//! shipped path into the selection function — including the replay's re-derivation — refuses a
//! defective catalog rather than selecting over it.
//!
//! **Why `select()` and not `Catalog::from_sources`.** Two of the four need the artifact registry
//! or the facet registry, which the catalog loader does not have and must not acquire; splitting
//! them across two call sites would make *"the catalog was checked"* depend on which constructor a
//! caller reached for. `select()` is the one place all four inputs are in hand, the check is pure
//! (no clock, no I/O, no allocation the four digests do not already imply), and it is the boundary
//! past which a defective catalog would otherwise become data in a `Dag`.
//!
//! **The `redkiln lint` CLI surface is still Step 7d's and is ARMED, NOT DISCHARGED**, on the same
//! shape `lib.rs`:17-20 records for the two facet lints. What is discharged here is that the rules
//! now refuse on the shipped path, not merely when a test calls them.
//!
//! # Why AC-057 does not fail the build, stated once
//!
//! *"THEN it is reported as a **warning** with a count and a list, and does not fail the build AND
//! the count, **together with the enabled pack count**, is written to the lint report artifact."*
//! The pack count travels with the count because *"the reading moves 20.8% → 14.9% on pack set
//! alone"*: a dead-content figure without its pack set is not a reading. **The golden 6/102 at 12
//! packs is against the SHIPPED catalog and pack set and is Step 7e's**, not this crate's.

use std::collections::{BTreeMap, BTreeSet};

use super::Pack;
use super::artifacts::ArtifactRegistry;
use super::step4::LoadError;
use crate::catalog::Catalog;

/// **AC-059.** A `requires` cycle in the catalog fails load with the cycle **path** named.
///
/// The relation is over activities, not kinds: an activity requires a kind, some activity provides
/// that kind, and the edge is between the two. F35 measured **hard cycles 0** across the enumerated
/// lattice, so the red fixture is authored (`tests/fixtures/catalog-cycle/`) — a lint with no red
/// fixture enumerates nothing, which is Halt 8's shape.
///
/// **An OPTIONAL requirement is not an edge, and a catalog is cyclic iff its NON-optional subgraph
/// is** *(2026-09-21, Step 8, operator decision)*. A `Kind?` may go unsatisfied, so a loop
/// traversable only through one can always be broken; and F35's *hard cycles 0* — this criterion's
/// own cited evidence — is a count under exactly that reading. `Requirement` has carried
/// `optional: bool` since it was authored and this walk did not read it, which made every `Kind?` a
/// hard edge. **It was a live defect, not a tidy-up:** measured over all 102 activities in the
/// shipped `.redkiln/catalog/`, the non-optional subgraph has no cycle at all (reproducing F35's 0),
/// while counting optional edges the walk finds `adr-record -> design-doc -> adr-record` — so
/// `redkiln run` refused the shipped catalog at load with exit 3 and could never have loaded it,
/// while `doctor` passed all 39 rules without seeing it. Three mutual pairs carry that shape
/// (`adr-record`/`design-doc`, `data-migration-plan`/`design-doc`, `design-doc`/`threat-model`) and
/// every one is both-optional. The must-LOAD twin is
/// `tests/fixtures/catalog-cycle-optional/`, whose test also re-asserts that the hard 3-node cycle
/// still fails.
///
/// # Errors
///
/// [`LoadError::RequiresCycle`] naming the path, in the order the walk found it.
pub fn requires_cycle_lint(catalog: &Catalog) -> Result<(), LoadError> {
    let mut providers: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for a in catalog.activities() {
        for kind in &a.provides {
            providers.entry(kind.as_str()).or_default().push(&a.id);
        }
    }
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for a in catalog.activities() {
        let out = edges.entry(a.id.as_str()).or_default();
        for requirement in &a.requires {
            // An OPTIONAL requirement is not a cycle edge. A `Kind?` can go unsatisfied, so a loop
            // that is only traversable through one can always be broken and is not the HARD cycle
            // this lint's own evidence is about. See the docstring; the fixture is
            // `tests/fixtures/catalog-cycle-optional/`.
            if requirement.optional {
                continue;
            }
            for producer in providers
                .get(requirement.kind.as_str())
                .into_iter()
                .flatten()
            {
                if *producer != a.id {
                    out.insert(producer);
                }
            }
        }
    }

    // Depth-first, from every activity in byte order, so the path a catalog reports is a function
    // of the catalog rather than of the walk's starting point.
    let mut done: BTreeSet<&str> = BTreeSet::new();
    for a in catalog.activities() {
        let mut stack: Vec<&str> = Vec::new();
        if let Some(cycle) = visit(a.id.as_str(), &edges, &mut done, &mut stack) {
            return Err(LoadError::RequiresCycle { path: cycle });
        }
    }
    Ok(())
}

fn visit<'a>(
    node: &'a str,
    edges: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    done: &mut BTreeSet<&'a str>,
    stack: &mut Vec<&'a str>,
) -> Option<Vec<String>> {
    if done.contains(node) {
        return None;
    }
    if let Some(at) = stack.iter().position(|n| *n == node) {
        let mut path: Vec<String> = stack[at..].iter().map(|s| (*s).to_owned()).collect();
        path.push(node.to_owned());
        return Some(path);
    }
    stack.push(node);
    for next in edges.get(node).into_iter().flatten() {
        if let Some(cycle) = visit(next, edges, done, stack) {
            return Some(cycle);
        }
    }
    stack.pop();
    done.insert(node);
    None
}

/// **AC-060.** *"GIVEN activity A declaring `excludes = [B]` while B does not declare
/// `excludes = [A]` / WHEN lint runs / THEN load fails with `AsymmetricExcludes { a, b }`."*
///
/// # Errors
///
/// [`LoadError::AsymmetricExcludes`] on the first asymmetric pair, in byte order of `a` then `b`,
/// so the failure is a function of the catalog.
pub fn excludes_lint(catalog: &Catalog) -> Result<(), LoadError> {
    let by_id: BTreeMap<&str, &Vec<String>> = catalog
        .activities()
        .iter()
        .map(|a| (a.id.as_str(), &a.excludes))
        .collect();
    for a in catalog.activities() {
        for b in &a.excludes {
            // A name the catalog does not carry is a DANGLING exclusion, and it is reported as the
            // asymmetry it is rather than skipped: `None` here would silently pass a typo.
            let reciprocal = by_id.get(b.as_str()).is_some_and(|e| e.contains(&a.id));
            if !reciprocal {
                return Err(LoadError::AsymmetricExcludes {
                    a: a.id.clone(),
                    b: b.clone(),
                });
            }
        }
    }
    Ok(())
}

/// **AC-053's load half.** A `requires` naming a kind that is neither declared in `artifacts.toml`
/// nor provided by any activity binds to nothing.
///
/// ADR-0057's Decision makes `artifacts.toml` the kind vocabulary, so this is a catalog defect at
/// load and not a missing artifact at run time. **The `redkiln lint` surface is 7d's.**
///
/// # Errors
///
/// [`LoadError::DanglingRequires`] naming the activity and the kind.
pub fn dangling_requires_lint(
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
) -> Result<(), LoadError> {
    let provided: BTreeSet<&str> = catalog
        .activities()
        .iter()
        .flat_map(|a| a.provides.iter().map(String::as_str))
        .collect();
    for a in catalog.activities() {
        for requirement in a.requires.iter().chain(a.derived_from.iter()) {
            if artifacts.kind(&requirement.kind).is_none()
                && !provided.contains(requirement.kind.as_str())
            {
                return Err(LoadError::DanglingRequires {
                    activity_id: a.id.clone(),
                    kind: requirement.kind.clone(),
                });
            }
        }
    }
    Ok(())
}

/// **AC-057's report.** A warning with a count, a list, and the enabled pack count.
///
/// # Three definitions of "dead" exist and this carries two of them, named
///
/// 1. **The register's, which is what [`Self::dead`] holds.** AC-057's GIVEN: *"an activity whose
///    `provides` is required by no `require` row or `baseline` in any enabled pack"*. Literal, and
///    **not transitive**: a producer reached only through another activity's `requires` edge
///    satisfies it and is reported dead.
/// 2. **[`Self::reachable_via_requires`]**, the discriminating companion: which of those are in
///    fact reachable through some live activity's `requires` closure. Without it a reader cannot
///    tell a genuinely unreferenced activity from a transitive producer, and the count alone would
///    invite the "fix" of making rule 1 transitive.
/// 3. **`spikes/p9/lattice.py`:446's**, which is neither — *"never selected in any pack × context"*,
///    i.e. never appearing in an assembled DAG anywhere on the enumerated lattice. That is the
///    definition behind F35's **6/102 at 12 packs**, it requires running selection over the whole
///    lattice, and **it is not computed here.** The golden count is against the shipped catalog and
///    pack set, which §7b.15 routes to **Step 7e**; reporting rule 1's number as though it were
///    that figure is the confusion this paragraph exists to stop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadContentReport {
    /// Rule 1: the activity ids whose `provides` no enabled pack's `baseline` or `require` row
    /// names, in byte order.
    pub dead: Vec<String>,
    pub count: usize,
    /// Rule 2: those of [`Self::dead`] that some live activity's `requires` closure reaches.
    pub reachable_via_requires: Vec<String>,
    /// *"The count, **together with the enabled pack count**"* — the reading moves 20.8% → 14.9% on
    /// pack set alone, so a count without its pack set is not a reading.
    pub pack_count: usize,
    /// The denominator. `6/102` is the shipped figure's shape and this is its `102`.
    pub activity_count: usize,
}

impl DeadContentReport {
    /// The lint report artifact's payload — *"written to the lint report artifact keyed by catalog
    /// version"*. The **keying** by catalog version is `redkiln lint`'s and is 7d's; what is here
    /// is the content.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "dead": self.dead,
            "count": self.count,
            "reachable_via_requires": self.reachable_via_requires,
            "pack_count": self.pack_count,
            "activity_count": self.activity_count,
        })
    }
}

/// **AC-057.** *"GIVEN an activity whose `provides` is required by no `require` row or `baseline`
/// in any enabled pack … THEN it is reported as a warning with a count and a list, and **does not
/// fail the build**."*
///
/// Read [`DeadContentReport`]'s own header first: three definitions of *dead* are in play and this
/// computes the register's, plus the companion that keeps it honest.
#[must_use]
pub fn dead_content_report(
    catalog: &Catalog,
    packs: &[&Pack],
    _artifacts: &ArtifactRegistry,
) -> DeadContentReport {
    let mut wanted: BTreeSet<String> = BTreeSet::new();
    for pack in packs {
        // Every `[stage.*]` table, NOT only the stages `[policy]` mentions: a table the policy
        // omits still declares requirements, and skipping it would mark its producers dead.
        for (_, spec) in pack.stage_specs() {
            wanted.extend(spec.baseline.iter().cloned());
            for row in &spec.require {
                wanted.extend(row.artifacts.iter().cloned());
            }
        }
    }

    // Rule 1, literal.
    let dead: Vec<String> = catalog
        .activities()
        .iter()
        .filter(|a| !a.provides.iter().any(|p| wanted.contains(p)))
        .map(|a| a.id.clone())
        .collect();

    // Rule 2: the transitive closure of `requires` over the activities rule 1 calls live.
    let mut reached: BTreeSet<String> = BTreeSet::new();
    let mut frontier: Vec<&str> = catalog
        .activities()
        .iter()
        .filter(|a| !dead.contains(&a.id))
        .map(|a| a.id.as_str())
        .collect();
    let mut visited: BTreeSet<&str> = frontier.iter().copied().collect();
    while let Some(id) = frontier.pop() {
        let Some(a) = catalog.activities().iter().find(|x| x.id == id) else {
            continue;
        };
        for requirement in &a.requires {
            for producer in catalog
                .activities()
                .iter()
                .filter(|x| x.provides.contains(&requirement.kind))
            {
                reached.insert(producer.id.clone());
                if visited.insert(producer.id.as_str()) {
                    frontier.push(producer.id.as_str());
                }
            }
        }
    }
    let reachable_via_requires: Vec<String> = dead
        .iter()
        .filter(|id| reached.contains(*id))
        .cloned()
        .collect();

    DeadContentReport {
        count: dead.len(),
        reachable_via_requires,
        pack_count: packs.len(),
        activity_count: catalog.len(),
        dead,
    }
}

/// **Every refusing load rule, run at the one place all four inputs are in hand.**
///
/// Called by `select_amended` before step 1. Order is fixed and stated: the cheapest
/// catalog-only rules first, so a cyclic or asymmetric catalog is named as such rather than as
/// whatever the registry-dependent rules would have said about it.
///
/// [`dead_content_report`] is deliberately **not** here: AC-057 is *"reported as a warning … and
/// does not fail the build"*, so folding it in would convert a warning into a refusal.
///
/// # Errors
///
/// The first [`LoadError`] any of the four rules returns. *(dated 2026-09-26, `s8d` build: five
/// rules; the forward-ref licence runs first.)*
pub fn load_lints(
    catalog: &Catalog,
    artifacts: &ArtifactRegistry,
    facets: &crate::registry::Registry,
) -> Result<(), LoadError> {
    // *(dated 2026-09-26, `s8d` build, package P2; D21-48 X8; R2 = ADR-0173, written `proposed`)*
    // The forward-ref licence, catalog-only and so first. It is the catalog module's rule, run
    // here rather than in the strict parser, for the reason `catalog::forward_ref_lint` states.
    crate::catalog::forward_ref_lint(catalog)
        .map_err(|refs| LoadError::UnlicensedForwardRef { refs })?;
    requires_cycle_lint(catalog)?;
    excludes_lint(catalog)?;
    dangling_requires_lint(catalog, artifacts)?;
    super::step4::rank_lint(catalog, artifacts, facets)
}
