//! **Step 5 — validate.** Design §4.4 row 5:
//!
//! > Acyclic; every `requires` bound; every element of `R` reachable; `Σ cost_class` within the
//! > item's appetite; `excludes` respected. Emits
//! > `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped }`.
//! >
//! > *Invariant:* the engine **never trims, defers or drops** a member of `R` to fit the appetite —
//! > it gates, carrying the DAG, the appetite and the overage.
//!
//! AC-058, AC-078 (runtime acyclicity) and AC-059/AC-060 (the load-time cycle and `excludes`
//! lints), plus AC-053's dangling `requires` and AC-057's dead-content warning.
//!
//! **AC-078's own case is NOT here.** Its *Verified by* says *"a hand-assembled cyclic DAG
//! (constructed inside the crate, since the catalog lint of AC-059 prevents one arising from a
//! pack)"*, and [`rk_kernel::select::Dag`]'s constructor is `pub(in crate::select)` for the same
//! reason [`rk_kernel::select::RequiredSet`]'s is. The case lives at
//! `crates/rk-kernel/src/select/step5.rs` under `#[cfg(test)]`; a public constructor written to
//! move it here would convert the visibility fact into a convention.

use rk_kernel::select::{
    ArtifactRegistry, Catalog, Item, LoadError, Pack, SelectError, SkipReason, Step5Error,
    Step5Outcome, dangling_requires_lint, dead_content_report, excludes_lint, requires_cycle_lint,
    select,
};

const INCIDENT: &str = include_str!("../fixtures/packs/incident-forbidden.toml");
const APPETITE: &str = include_str!("../fixtures/packs/incident-appetite.toml");
const NO_APPETITE: &str = include_str!("../fixtures/packs/incident-no-appetite.toml");
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

const CYCLE: &[(&str, &str)] = &[
    (
        "cycle-a.toml",
        include_str!("../fixtures/catalog-cycle/cycle-a.toml"),
    ),
    (
        "cycle-b.toml",
        include_str!("../fixtures/catalog-cycle/cycle-b.toml"),
    ),
    (
        "cycle-c.toml",
        include_str!("../fixtures/catalog-cycle/cycle-c.toml"),
    ),
];

/// The optional-only and mixed mutual pairs — `catalog-cycle/`'s must-LOAD twin. See
/// `../fixtures/catalog-cycle-optional/opt-a.toml` for why it exists.
const CYCLE_OPTIONAL: &[(&str, &str)] = &[
    (
        "opt-a.toml",
        include_str!("../fixtures/catalog-cycle-optional/opt-a.toml"),
    ),
    (
        "opt-b.toml",
        include_str!("../fixtures/catalog-cycle-optional/opt-b.toml"),
    ),
    (
        "mixed-a.toml",
        include_str!("../fixtures/catalog-cycle-optional/mixed-a.toml"),
    ),
    (
        "mixed-b.toml",
        include_str!("../fixtures/catalog-cycle-optional/mixed-b.toml"),
    ),
];

const EXCLUDES_ASYMMETRIC: &[(&str, &str)] = &[
    (
        "asymmetric-a.toml",
        include_str!("../fixtures/catalog-excludes/asymmetric-a.toml"),
    ),
    (
        "asymmetric-b.toml",
        include_str!("../fixtures/catalog-excludes/asymmetric-b.toml"),
    ),
];

const EXCLUDES_SYMMETRIC: &[(&str, &str)] = &[
    (
        "asymmetric-a.toml",
        include_str!("../fixtures/catalog-excludes/asymmetric-a.toml"),
    ),
    (
        "symmetric-b.toml",
        include_str!("../fixtures/catalog-excludes/symmetric-b.toml"),
    ),
];

const EXCLUDES_PACK: &str = include_str!("../fixtures/packs/incident-excludes.toml");

const EXCLUDES_PAIR: &[(&str, &str)] = &[
    (
        "pair-a.toml",
        include_str!("../fixtures/catalog-excludes/pair-a.toml"),
    ),
    (
        "pair-b.toml",
        include_str!("../fixtures/catalog-excludes/pair-b.toml"),
    ),
];

const DANGLING: &[(&str, &str)] = &[(
    "dangling-requires.toml",
    include_str!("../fixtures/catalog-dangling/dangling-requires.toml"),
)];

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

fn facets() -> rk_kernel::registry::Registry {
    let path = workspace_root().join("spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry at {} is readable: {e}",
            path.display()
        )
    });
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry must load")
}

fn registry() -> ArtifactRegistry {
    ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the fixture registry loads")
}

fn catalog() -> Catalog {
    Catalog::from_sources(CATALOG).expect("the fixture catalog loads")
}

fn node_item() -> Item {
    Item::new("INC-1", "incident").with_toolchain("node", ["text"])
}

/// **AC-058, the negative criterion.** *"THEN a gate opens carrying the DAG, the appetite and the
/// overage AND the engine never trims, defers, or drops a member of `R` to fit the appetite."*
///
/// The comparison is *"`R` is byte-identical before and after the appetite check"* and it is made
/// under the canonical serialiser, which is the same comparison AC-040 is measured under rather
/// than a second one.
#[test]
fn appetite_gate_never_trims_r() {
    let facets = facets();
    let registry = registry();
    let catalog = catalog();
    let item = node_item();

    let roomy = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");
    let tight = Pack::from_toml("incident-appetite.toml", APPETITE).expect("loads");

    let under = select(&roomy, &catalog, &registry, &facets, "implement", &item)
        .expect("selection resolves under a roomy appetite");
    let over = select(&tight, &catalog, &registry, &facets, "implement", &item)
        .expect("an over-budget DAG is a GATE, never an error and never a trim");

    // The premise: the tight pack really does overrun, or the rest of this test asserts nothing.
    let gate = over
        .appetite_gate()
        .expect("the fixture must actually exceed its appetite");
    assert_eq!(gate.appetite, 4, "the pack's declared appetite.points");
    assert_eq!(
        gate.cost, 5,
        "S=1 + M=3 + S=1 over change-summary, implement-change and impl-report \
         (spikes/p9/lattice.py:17's I6 mapping)"
    );
    assert_eq!(
        gate.overage, 1,
        "the gate carries the OVERAGE, not just a flag"
    );
    assert!(
        under.appetite_gate().is_none(),
        "the roomy pack must not gate, or the gate is firing on something else"
    );

    // …and the gate carries the DAG.
    assert!(
        !gate.dag.nodes().is_empty(),
        "AC-058's THEN is `a gate opens CARRYING THE DAG`"
    );

    // The invariant: `R` is byte-identical to the un-gated run's.
    assert_eq!(
        under.required_canonical_bytes().expect("canonical"),
        over.required_canonical_bytes().expect("canonical"),
        "R is byte-identical across the appetite check: nothing was trimmed, deferred or dropped"
    );
    assert_eq!(
        over.dag().nodes().len(),
        under.dag().nodes().len(),
        "and the DAG the gate carries is the whole DAG, not a trimmed one"
    );
}

/// **The non-default.** A pack that declares no `appetite` gets **no appetite check**, and the
/// record says so with a `null` rather than with an unbounded sentinel. `00-preamble.md`:55 forbids
/// a code default for a value a record owes, and `spikes/p9/lattice.py`:146's own `10**9` fallback
/// is exactly the silent-pass shape.
#[test]
fn an_undeclared_appetite_is_not_an_unbounded_one() {
    let pack = Pack::from_toml("incident-no-appetite.toml", NO_APPETITE).expect("loads");
    let out = select(
        &pack,
        &catalog(),
        &registry(),
        &facets(),
        "implement",
        &node_item(),
    )
    .expect("selection resolves");

    assert!(out.appetite_gate().is_none());
    assert_eq!(
        out.dag().appetite(),
        None,
        "an undeclared appetite is `None`, never `i64::MAX` and never a large number"
    );
    let event = out.dag().assembled_event();
    assert_eq!(
        event.payload.get("appetite"),
        Some(&serde_json::Value::Null),
        "`dag_assembled` says the appetite was not declared, rather than omitting the question"
    );
}

/// **Step 5's event.** `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped }` — the
/// five fields design §4.4 row 5 names, and the kind `rk-types` carries.
#[test]
fn dag_assembled_carries_the_five_named_fields() {
    let pack = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");
    let out = select(
        &pack,
        &catalog(),
        &registry(),
        &facets(),
        "implement",
        &node_item(),
    )
    .expect("selection resolves");

    let event = out.dag().assembled_event();
    assert_eq!(event.kind, rk_types::event_kind::EventKind::DagAssembled);
    for field in ["inputs_digest", "nodes", "edges", "rows_fired", "skipped"] {
        assert!(
            event.payload.get(field).is_some(),
            "`dag_assembled` must carry `{field}`: {}",
            event.payload
        );
    }
    assert_eq!(
        event.payload["inputs_digest"].as_str(),
        Some(out.inputs_digest().as_str()),
        "the event's `inputs_digest` is the four-digest tuple's, not a second hash"
    );

    // The skip row AC-077 owes is IN THE EVENT, not only in the selection struct. §7b.9(7):
    // "the skip row in `dag_assembled.skipped` naming the stage".
    let skipped = event.payload["skipped"]
        .as_array()
        .expect("`skipped` is an array");
    assert!(
        skipped.iter().any(|r| {
            r["stage"] == "specification"
                && r["kind"] == "Spec"
                && r["reason"] == SkipReason::ForbiddenStageViaRequires.as_str()
        }),
        "the forbidden stage reached through a conditional `requires` names itself here: {skipped:?}"
    );

    // Every edge names two nodes that exist. An edge to a node the DAG does not carry is the
    // "every `requires` bound" half failing silently.
    let ids: Vec<&str> = out.dag().nodes().iter().map(|n| n.id.as_str()).collect();
    for edge in out.dag().edges() {
        assert!(
            ids.contains(&edge.from.as_str()),
            "dangling edge from {edge:?}"
        );
        assert!(ids.contains(&edge.to.as_str()), "dangling edge to {edge:?}");
    }
    assert!(
        !out.dag().edges().is_empty(),
        "a DAG with no edges would satisfy the loop above vacuously"
    );
}

/// **AC-059's optional-edge reading, and the LIVE instance that forced it.**
///
/// A catalog is cyclic iff its **non-optional** subgraph is cyclic. AC-059's evidence line and
/// [`requires_cycle_lint`]'s own docstring both cite F35's *hard cycles **0** across the enumerated
/// lattice*, and `Requirement` has carried `optional: bool` since it was authored — the lint simply
/// did not read it, so every `Kind?` was a hard edge.
///
/// This is not a hypothetical. Measured 2026-09-21 over all **102** shipped activities in
/// `.redkiln/catalog/`: counting only non-optional edges there is **no cycle anywhere**, which
/// reproduces F35's 0 exactly; counting optional edges too, the walk finds
/// `adr-record -> design-doc -> adr-record` and load fails at exit 3 — so no `redkiln run` could
/// load the shipped catalog at all, and `doctor` exits 0 across all 39 rules without seeing it.
/// Three mutual pairs have that shape and every one is both-optional.
///
/// **Both directions are asserted here**, because an exemption that also exempted hard edges would
/// be a lint that enumerates nothing (Halt 8).
#[test]
fn ac059_optional_only_requires_cycle_loads() {
    let optional =
        Catalog::from_sources(CYCLE_OPTIONAL).expect("the fixture catalog itself parses");
    assert!(
        requires_cycle_lint(&optional).is_ok(),
        "a cycle every edge of which is `Kind?` is not a HARD cycle and must LOAD — this is the \
         shape `adr-record <-> design-doc` carries in the shipped catalog"
    );

    let hard = Catalog::from_sources(CYCLE).expect("the fixture catalog itself parses");
    assert!(
        matches!(
            requires_cycle_lint(&hard),
            Err(LoadError::RequiresCycle { .. })
        ),
        "the 3-node HARD cycle must STILL fail load — the optional reading may not empty the lint"
    );
}

/// **AC-059 and AC-060, the two load-time lints.** A 3-node `requires` cycle fails load naming the
/// path; an asymmetric `excludes` fails with `AsymmetricExcludes { a, b }`.
///
/// **Both carry their must-load companion.** Without one, each lint is satisfied by a rule that
/// rejects every catalog.
#[test]
fn cycle_and_excludes_fail_load() {
    // ---- AC-059 -------------------------------------------------------------------------------
    let cyclic = Catalog::from_sources(CYCLE).expect("the fixture catalog itself parses");
    match requires_cycle_lint(&cyclic) {
        Err(LoadError::RequiresCycle { path }) => {
            assert_eq!(
                path,
                vec![
                    "alpha".to_owned(),
                    "beta".to_owned(),
                    "gamma".to_owned(),
                    "alpha".to_owned()
                ],
                "the cycle PATH is named, not merely its existence"
            );
        }
        other => panic!("expected a RequiresCycle naming the path, got {other:?}"),
    }
    assert!(
        requires_cycle_lint(&catalog()).is_ok(),
        "the acyclic fixture catalog must LOAD, or the lint is a reject-everything rule"
    );

    // ---- AC-060 -------------------------------------------------------------------------------
    let asymmetric =
        Catalog::from_sources(EXCLUDES_ASYMMETRIC).expect("the fixture catalog itself parses");
    match excludes_lint(&asymmetric) {
        Err(LoadError::AsymmetricExcludes { a, b }) => {
            assert_eq!(a, "deploy-blue-green");
            assert_eq!(b, "canary-release");
        }
        other => panic!("expected AsymmetricExcludes {{ a, b }}, got {other:?}"),
    }
    let symmetric =
        Catalog::from_sources(EXCLUDES_SYMMETRIC).expect("the fixture catalog itself parses");
    assert!(
        excludes_lint(&symmetric).is_ok(),
        "a symmetric pair must LOAD — the lint is on the asymmetry, not on the key's presence"
    );
}

/// **AC-053's load half.** A `requires` naming a kind `artifacts.toml` does not declare and no
/// activity provides is a **load** failure, because ADR-0057 makes that registry the kind
/// vocabulary.
#[test]
fn dangling_requires_fails_load() {
    let dangling = Catalog::from_sources(DANGLING).expect("the fixture catalog itself parses");
    match dangling_requires_lint(&dangling, &registry()) {
        Err(LoadError::DanglingRequires { activity_id, kind }) => {
            assert_eq!(activity_id, "smoke-check");
            assert_eq!(kind, "Telemetry");
        }
        other => panic!("expected DanglingRequires {{ activity_id, kind }}, got {other:?}"),
    }
    assert!(
        dangling_requires_lint(&catalog(), &registry()).is_ok(),
        "the fixture catalog's own `requires` all bind, or the lint rejects everything"
    );
}

/// **AC-057.** Dead content is a **warning with a count and a list**, and *"does not fail the
/// build"*. The pack count travels with the count because *"the reading moves 20.8% → 14.9% on
/// pack set alone"*.
///
/// **The golden `6/102` at 12 packs is NOT asserted here** — that figure is against the *shipped*
/// catalog and pack set, which §7b.15 routes to **Step 7e**, and it is computed under
/// `spikes/p9/lattice.py`:446's **third** definition (*"never selected in any pack × context"*),
/// which is not this rule. What is asserted is the instrument: the rule, the count, the list, the
/// pack count, and the companion that keeps the literal rule readable.
#[test]
fn dead_activity_is_a_warning_with_counts() {
    let pack = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");
    let report = dead_content_report(&catalog(), &[&pack], &registry());

    // `spec-authoring` provides `Spec`, which no `baseline` and no `require` row of this pack
    // names: dead under AC-057's literal GIVEN.
    assert!(
        report.dead.contains(&"spec-authoring".to_owned()),
        "an activity no enabled pack's row or baseline names is dead content: {:?}",
        report.dead
    );
    // …and the companion says why the literal rule flagged it: `impl-report` reaches `Spec`
    // through a `requires` edge. Without this list the count alone invites the "repair" of making
    // the rule transitive, which would report 0 dead on any catalog whose roots are all reachable.
    assert_eq!(
        report.reachable_via_requires,
        vec!["spec-authoring".to_owned()],
        "the discriminating companion: dead by the register's rule, reached by an edge"
    );
    assert_eq!(report.count, report.dead.len());
    assert_eq!(
        report.pack_count, 1,
        "the pack count travels with the count"
    );
    assert_eq!(report.activity_count, 5);
    // …and it is a WARNING: nothing above returned an error, and the selection still resolves.
    assert!(
        select(
            &pack,
            &catalog(),
            &registry(),
            &facets(),
            "implement",
            &node_item()
        )
        .is_ok(),
        "dead content does not fail the build"
    );
    assert!(
        !report.dead.contains(&"impl-report".to_owned()),
        "a live activity must not be reported dead, or the rule flags everything"
    );
}

/// **Design §4.4 row 5's *"`excludes` respected"* — the RUNTIME half.**
///
/// **This test exists because a falsification found the check untested.** Mutation G22 of
/// 2026-09-13 made step 5's `excludes` loop enumerate nothing and **every test stayed green**: the
/// only `excludes` fixtures were AC-060's load-time pair, which never reaches an assembled graph.
/// A check that enumerates nothing passes, which is Halt 8's shape one level down.
///
/// The pair here is **symmetric**, so [`excludes_lint`] passes on it — which is the discrimination:
/// a catalog can clear the load-time lint and still assemble a graph holding both members.
#[test]
fn excluding_activities_never_share_one_dag() {
    let pack = Pack::from_toml("incident-excludes.toml", EXCLUDES_PACK).expect("loads");
    let catalog = Catalog::from_sources(EXCLUDES_PAIR).expect("loads");
    let item = Item::new("INC-1", "incident");

    // The premise: the pair is symmetric, so the LOAD-time lint is satisfied.
    assert!(
        excludes_lint(&catalog).is_ok(),
        "the pair must clear AC-060, or this test is measuring the load lint again"
    );

    match select(&pack, &catalog, &registry(), &facets(), "release", &item) {
        Err(rk_kernel::select::SelectError::Step5(Step5Error::ExcludedPair { a, b })) => {
            let mut pair = [a, b];
            pair.sort();
            assert_eq!(
                pair,
                ["canary-release".to_owned(), "deploy-blue-green".to_owned()],
                "the refusal names BOTH members"
            );
        }
        other => panic!("expected an ExcludedPair refusal, got {other:?}"),
    }

    // The control: drop one member from `R` and the same catalog assembles. Without it the test is
    // satisfied by a step 5 that refuses this pack for any reason at all.
    let one = Pack::from_toml(
        "one.toml",
        "[pack]\nid = \"one\"\n\n[policy]\nstages = { release = \"recommended\" }\n\n\
         [stage.release]\nbaseline = [\"Deployment\"]\n",
    )
    .expect("loads");
    let out = select(&one, &catalog, &registry(), &facets(), "release", &item)
        .expect("one member alone assembles");
    assert_eq!(out.dag().nodes().len(), 1);
}

/// **The assembled DAG is what step 6 orders, and step 5 returns it whole.** A separate assertion
/// from AC-058's because `Step5Outcome`'s two variants must both carry a DAG — an
/// `AppetiteExceeded` that carried no graph would make AC-058's *"carrying the DAG"* unwritable.
#[test]
fn both_step5_outcomes_carry_a_dag() {
    let facets = facets();
    let registry = registry();
    let catalog = catalog();
    let item = node_item();

    let roomy = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");
    let tight = Pack::from_toml("incident-appetite.toml", APPETITE).expect("loads");

    for (pack, expect_gate) in [(&roomy, false), (&tight, true)] {
        let out = select(pack, &catalog, &registry, &facets, "implement", &item)
            .expect("selection resolves");
        assert_eq!(out.appetite_gate().is_some(), expect_gate);
        assert!(!out.dag().nodes().is_empty());
        match out.step5() {
            Step5Outcome::Assembled(d) => assert!(!expect_gate && !d.nodes().is_empty()),
            Step5Outcome::AppetiteExceeded(g) => {
                assert!(expect_gate && !g.dag.nodes().is_empty());
            }
        }
    }
}

/// **The load rules REFUSE ON THE SHIPPED PATH, not only when a test calls them.**
///
/// The three tests above call `requires_cycle_lint`, `excludes_lint` and `dangling_requires_lint`
/// directly. That is the right shape for pinning each rule's *verdict* and the wrong shape for
/// claiming the build fails: until 2026-09-13 **no shipped code called any of them**, and the
/// consequence was measurable rather than theoretical —
///
/// ```text
/// Catalog::from_sources(DANGLING)                → Ok, 1 activity
/// select(pack, dangling, registry, facets, …)    → Ok, nodes=1 edges=0 unsat=["Telemetry"]
/// ```
///
/// — a **catalog defect reported as an ordinary unsatisfiable artifact**, which
/// `tests/fixtures/catalog-dangling/`'s own header forbids in terms: *"this is a pack/catalog
/// defect at LOAD and not a missing artifact at run time."* AC-059 and AC-060 at least had runtime
/// backstops in step 5; AC-053 had none.
///
/// This test is over the **entry point**, so it goes red if `load_lints` is unwired again — which
/// is the failure the three tests above cannot see. Its must-pass companion is every other
/// `select()` in this crate: a reject-everything precondition would take the whole suite red.
#[test]
fn select_refuses_a_catalog_that_would_not_have_loaded() {
    let facets = facets();
    let registry = registry();
    let pack = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");
    let item = node_item();

    // ---- AC-053 — the one with no runtime backstop --------------------------------------------
    let dangling = Catalog::from_sources(DANGLING).expect("the fixture catalog itself parses");
    match select(&pack, &dangling, &registry, &facets, "implement", &item) {
        Err(SelectError::Load(LoadError::DanglingRequires { activity_id, kind })) => {
            assert_eq!(activity_id, "smoke-check");
            assert_eq!(
                kind, "Telemetry",
                "the kind is named as a LOAD defect, not returned as an Unsatisfiable gate"
            );
        }
        other => panic!(
            "select() over a catalog with a dangling `requires` must refuse at load, got {other:?}"
        ),
    }

    // ---- AC-059 --------------------------------------------------------------------------------
    let cyclic = Catalog::from_sources(CYCLE).expect("the fixture catalog itself parses");
    match select(&pack, &cyclic, &registry, &facets, "implement", &item) {
        Err(SelectError::Load(LoadError::RequiresCycle { path })) => {
            assert_eq!(path.first().map(String::as_str), Some("alpha"));
        }
        other => panic!("select() over a cyclic catalog must refuse at load, got {other:?}"),
    }

    // ---- AC-060 --------------------------------------------------------------------------------
    let asymmetric =
        Catalog::from_sources(EXCLUDES_ASYMMETRIC).expect("the fixture catalog itself parses");
    match select(&pack, &asymmetric, &registry, &facets, "implement", &item) {
        Err(SelectError::Load(LoadError::AsymmetricExcludes { a, b })) => {
            assert_eq!(a, "deploy-blue-green");
            assert_eq!(b, "canary-release");
        }
        other => {
            panic!("select() over an asymmetric `excludes` must refuse at load, got {other:?}")
        }
    }

    // ---- the must-load companion, so this is not a reject-everything precondition --------------
    select(&pack, &catalog(), &registry, &facets, "implement", &item)
        .expect("the good fixture catalog must still select");
}
