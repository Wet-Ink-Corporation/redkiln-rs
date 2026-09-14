//! **Step 6 — execute.** Design §4.4 row 6:
//!
//! > Topological order; antichains parallel where §7 allows.
//! >
//! > *Invariant:* the engine's own invariant is that execution **cannot change `R`** — step 7 is
//! > the only channel, and it re-enters at step 2.
//!
//! # 🛑 THIS STEP SHIPS UNCOVERED, AND THAT IS THE DESIGN'S OWN WORDING
//!
//! Design §4.4 row 6's criterion cell reads, in full: *"**No criterion in the thirteen area files
//! covers step 6.** §7.7 states four parallelism rules and names exactly one criterion (AC-388, the
//! stagger); the sequential-within-a-tree, one-worktree-per-antichain-member and merge-queue rules
//! are pinned by AC-189 and AC-196–AC-198 in the AC register's git-reconcile section, and **the
//! execute step of the pure selection function itself is uncovered.** Recorded as a gap, not
//! papered over."*
//!
//! `s7b`:759-766 gives this stage its instruction: *"Build step 6 to its stated invariant —
//! execution cannot change `R`; step 7 is the only channel and it re-enters at step 2 — and record
//! in `PROGRESS.md` that it ships uncovered."*
//!
//! **So no test in this file is named after a criterion**, and none may be: naming one
//! `ac0xx_…` would claim coverage the design records as absent. The two tests below assert the
//! stated invariant and the stated ordering, and nothing else.

use rk_kernel::select::{
    ArtifactRegistry, Catalog, DeviationKind, DeviationOutcome, Item, Pack, RequestDeviation,
    select, step6, step7,
};

const INCIDENT: &str = include_str!("../fixtures/packs/incident-forbidden.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

const CATALOG: &[(&str, &str)] = &[
    (
        "impl-report.toml",
        include_str!("../fixtures/catalog-select/impl-report.toml"),
    ),
    (
        "change-summary.toml",
        include_str!("../fixtures/catalog-select/change-summary.toml"),
    ),
    (
        "change-summary-ue.toml",
        include_str!("../fixtures/catalog-select/change-summary-ue.toml"),
    ),
    (
        "changeset.toml",
        include_str!("../fixtures/catalog-select/changeset.toml"),
    ),
    (
        "spec-authoring.toml",
        include_str!("../fixtures/catalog-select/spec-authoring.toml"),
    ),
];

fn facets() -> rk_kernel::registry::Registry {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry must load")
}

fn fixtures() -> (
    Pack,
    Catalog,
    ArtifactRegistry,
    rk_kernel::registry::Registry,
) {
    (
        Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads"),
        Catalog::from_sources(CATALOG).expect("loads"),
        ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("loads"),
        facets(),
    )
}

/// **Step 6's stated invariant, and the only thing this step owes.** *"Execution cannot change
/// `R` — step 7 is the only channel, and it re-enters at step 2."*
///
/// The assertion is byte-identity of `R` under the canonical serialiser across a full execution
/// plan, taken over an item whose plan has more than one layer — a single-layer plan cannot
/// distinguish an engine that rebuilds `R` between layers from one that does not.
#[test]
fn execution_cannot_change_the_required_set() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = Item::new("INC-1", "incident").with_toolchain("node", ["text"]);
    let selection = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");

    let before = selection.required_canonical_bytes().expect("canonical");
    let plan = step6(selection.dag()).expect("the assembled DAG is acyclic, so it orders");
    assert!(
        plan.layers().len() > 1,
        "a single-layer plan cannot discriminate: {:?}",
        plan.layers()
    );

    // Walk the whole plan. `step6` takes `&Dag` and returns a plan; there is no `&mut` anywhere in
    // its signature, so this is a shape assertion as much as a value one — but the value is taken
    // anyway, because a shape a reader has to infer is not an assertion.
    let mut executed = 0_usize;
    for layer in plan.layers() {
        for _node in layer {
            executed += 1;
        }
    }
    assert_eq!(
        executed,
        selection.dag().nodes().len(),
        "every node ordered"
    );

    // 🛑 **THE LINE BELOW CANNOT FAIL ON ITS OWN, AND SAYING SO IS THE POINT.** The walk above only
    // increments a counter; `step6` takes `&Dag` and `selection` is never borrowed mutably, so
    // `before == after` holds with nothing executed at all — measured 2026-09-13 by deleting the
    // walk entirely, after which this test still passed. What actually carries *"execution cannot
    // change `R`"* is the type signature, and a test whose name claims more than its body delivers
    // is the shape this programme keeps being bitten by. The line stays as the value half of the
    // shape assertion; **what makes the invariant checkable is the second half below**, which
    // exercises the ONE channel design §4.4 row 6 says exists and shows that it, too, leaves this
    // `R` untouched.
    assert_eq!(
        before,
        selection.required_canonical_bytes().expect("canonical"),
        "R is byte-identical after the whole plan has been walked"
    );

    // ---- the only channel: step 7, which RE-ENTERS at step 2 rather than editing ---------------
    //
    // *"step 7 is the only channel, and it re-enters at step 2."* An accepted deviation is the one
    // thing in this system that changes what a run requires. It is driven here — over the very
    // selection whose plan was just walked — so that `before` is compared across something that
    // COULD have changed `R` and did not: the amendment arrives as a SECOND `Selection`, and this
    // one's bytes are the same as they were before the plan ran and before step 7 was called.
    let request = RequestDeviation::new(
        "ChangeSummary",
        DeviationKind::Skip,
        "criticality >= high",
        "the incident is already summarised in the timeline",
    );
    let critical = Item::new("INC-1", "incident")
        .with_toolchain("node", ["text"])
        .with_facet_ordinal(
            "criticality",
            &["low", "med", "high", "critical"],
            "critical",
        );
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &critical)
        .expect("selection resolves for the deviating item");
    let base_before = base.required_canonical_bytes().expect("canonical");
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &critical,
        "impl-report",
        &request,
        &base,
    )
    .expect("an accepted deviation re-runs 2-5");
    let amended = match &outcome {
        DeviationOutcome::Accepted { selection, .. } => selection,
        other => panic!("the premise fails: this fixture must ACCEPT, got {other:?}"),
    };
    let amended_bytes = amended.required_canonical_bytes().expect("canonical");
    assert_ne!(
        base_before, amended_bytes,
        "the premise: the re-run must actually produce a DIFFERENT R, or the equality below is \
         vacuous for the same reason the walk was"
    );
    assert_eq!(
        base_before,
        base.required_canonical_bytes().expect("canonical"),
        "the one channel that changes what a run requires returns a NEW Selection; it does not \
         edit this one's R (design §4.4 row 6's invariant, and AC-063)"
    );
}

/// **Topological order, and antichains as layers.** Every edge points from an earlier layer to a
/// later one; nothing inside one layer depends on anything else inside it.
#[test]
fn topological_order_and_antichains() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = Item::new("INC-1", "incident").with_toolchain("node", ["text"]);
    let selection = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");
    let dag = selection.dag();
    let plan = step6(dag).expect("acyclic");

    let layer_of = |id: &str| -> usize {
        plan.layers()
            .iter()
            .position(|l| l.iter().any(|n| n == id))
            .unwrap_or_else(|| panic!("{id} is in no layer"))
    };
    for edge in dag.edges() {
        assert!(
            layer_of(&edge.from) < layer_of(&edge.to),
            "every edge crosses layers forward: {edge:?}"
        );
    }
    assert!(
        !dag.edges().is_empty(),
        "a DAG with no edges satisfies the loop above vacuously"
    );

    // `Changeset` is produced before the two activities that require it, and the two of those are
    // an antichain: neither requires the other.
    let order: Vec<&str> = plan.order();
    let pos = |id: &str| {
        order
            .iter()
            .position(|n| *n == id)
            .unwrap_or_else(|| panic!("{id} is not in the order: {order:?}"))
    };
    assert!(pos("implement-change[node]") < pos("change-summary[node]"));
    assert!(pos("implement-change[node]") < pos("impl-report"));
    assert_eq!(
        layer_of("change-summary[node]"),
        layer_of("impl-report"),
        "two nodes with no edge between them share an antichain"
    );
}
