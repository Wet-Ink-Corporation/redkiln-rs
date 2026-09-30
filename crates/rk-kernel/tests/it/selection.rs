//! Steps 1 and 2 of design §4.4's seven-step pure selection function, and the purity contract the
//! whole function is written under.
//!
//! **All seven steps now exist** — steps 5, 6 and 7 landed with `src/select/step5.rs`, `step6.rs`
//! and `step7.rs` — so [`selection_seven_steps_are_pure`] is written under that name and exercises
//! all seven inside its loop. It was `selection_steps_one_to_four_are_pure` while four of the seven
//! were built, because claiming the seven over four is the shape `00-preamble.md`'s golden
//! constraint 10 forbids; the rename is the behaviour arriving, not a relabelling.
//!
//! Steps 5, 6 and 7's own cases live in `dag.rs`, `execute.rs` and `deviation.rs`.

use std::collections::BTreeSet;

use rk_kernel::rigor::Rigor;
use rk_kernel::select::{
    ArtifactRegistry, Catalog, GateReason, Item, Pack, PackError, RigorSource, SkipReason, step1,
    step2,
};

pub(crate) const INCIDENT: &str = include_str!("../fixtures/packs/incident-forbidden.toml");
const DISCOURAGED: &str = include_str!("../fixtures/packs/discouraged.toml");

fn pack() -> Pack {
    match Pack::from_toml("incident-forbidden.toml", INCIDENT) {
        Ok(p) => p,
        Err(e) => panic!("the fixture pack must load: {e}"),
    }
}

/// **Step 1 (AC-050, AC-051's stage half).** Rigor is read per stage from `policy.<class>` and an
/// explicit item-frontmatter override replaces it **and is logged**. Design §4.4 row 1 puts
/// "(logged)" inside the rule, so an override that silently wins is not this step.
#[test]
fn rigor_is_per_stage_and_an_item_override_is_logged() {
    let pack = pack();
    let plain = step1(&pack, &Item::new("INC-1", "incident"));
    assert_eq!(plain.rigor("analysis"), Some(Rigor::Recommended));
    assert_eq!(plain.rigor("review"), Some(Rigor::Mandatory));
    assert_eq!(plain.source("analysis"), Some(RigorSource::PackPolicy));
    assert!(
        plain.overrides().is_empty(),
        "an item with no frontmatter override logs no override row"
    );

    let overridden = step1(
        &pack,
        &Item::new("INC-1", "incident").with_rigor_override("analysis", Rigor::Mandatory),
    );
    assert_eq!(overridden.rigor("analysis"), Some(Rigor::Mandatory));
    assert_eq!(
        overridden.source("analysis"),
        Some(RigorSource::ItemOverride)
    );
    let rows = overridden.overrides();
    assert_eq!(rows.len(), 1, "exactly one override row: {rows:?}");
    assert_eq!(rows[0].stage, "analysis");
    assert_eq!(rows[0].from, Rigor::Recommended);
    assert_eq!(rows[0].to, Rigor::Mandatory);
    // Every other stage is untouched by the override.
    assert_eq!(overridden.rigor("review"), Some(Rigor::Mandatory));
    assert_eq!(overridden.source("review"), Some(RigorSource::PackPolicy));
}

/// **Step 1 (AC-051, AC-077's first half).** `forbidden` is a **logged skip, not an absence**
/// (design §4.4 row 1). A forbidden stage that produced no row would be indistinguishable from a
/// stage the pack never mentioned, and AC-077's THEN is *"every such exclusion appears as a logged
/// skip row naming the forbidden stage"*.
#[test]
fn forbidden_is_a_logged_skip_not_an_absence() {
    let out = step1(&pack(), &Item::new("INC-1", "incident"));

    assert_eq!(out.rigor("discovery"), Some(Rigor::Forbidden));
    assert!(
        out.is_forbidden("discovery"),
        "discovery is `forbidden` in this pack's policy"
    );
    assert!(!out.is_forbidden("analysis"));

    let stages: Vec<&str> = out
        .skipped()
        .iter()
        .filter(|r| r.reason == SkipReason::ForbiddenStage)
        .map(|r| r.stage.as_str())
        .collect();
    assert_eq!(
        stages,
        vec!["design", "discovery", "planning", "specification"],
        "one skip row per forbidden stage, in byte order, naming the stage"
    );

    // The absence half, stated as its own assertion: a stage the pack does not mention at all is
    // NOT a skip row, because it is an absence and not a refusal.
    assert!(
        out.rigor("nonexistent-stage").is_none(),
        "an unmentioned stage has no rigor; it is not silently `optional`"
    );
    assert!(
        !out.skipped().iter().any(|r| r.stage == "nonexistent-stage"),
        "an absence must not manufacture a skip row"
    );
}

/// **AC-050's third AND.** `discouraged` is RESERVED: the variant exists in the ordinal, and a pack
/// that sets any stage to it **fails validation naming the site**. A fourth live rigor level was
/// rejected for having no measured need (§7b:768-771); nothing here implements one.
#[test]
fn rigor_discouraged_refused() {
    let err = Pack::from_toml("discouraged.toml", DISCOURAGED)
        .expect_err("a pack declaring `discouraged` must not load");
    match &err {
        PackError::DiscouragedIsReserved { site, .. } => {
            assert_eq!(site, "discouraged.toml: policy.stages.discovery");
        }
        other => panic!("expected DiscouragedIsReserved, got {other:?}"),
    }
    // The message names the site, not just the token: AC-050 says "naming the site".
    assert!(
        err.to_string().contains("policy.stages.discovery"),
        "the message must name the site: {err}"
    );

    // The admissibility table across all five levels, which is the other half of AC-050's
    // *Verified by*. `forbidden` and `discouraged` are not live, so neither admits an activity.
    for (stage_rigor, min, admissible) in [
        (Rigor::Optional, Rigor::Optional, true),
        (Rigor::Optional, Rigor::Recommended, false),
        (Rigor::Recommended, Rigor::Optional, true),
        (Rigor::Recommended, Rigor::Mandatory, false),
        (Rigor::Mandatory, Rigor::Mandatory, true),
        (Rigor::Forbidden, Rigor::Optional, false),
        (Rigor::Discouraged, Rigor::Optional, false),
    ] {
        assert_eq!(
            rk_kernel::select::admits(stage_rigor, min, None),
            admissible,
            "stage at {stage_rigor}, min_rigor {min}"
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Step 2 — the required set
// ---------------------------------------------------------------------------------------------

const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

/// The shipped facet registry, read at the workspace root exactly as `rigor.rs` reads it. The path
/// is on a non-comment line deliberately: the closed vocabulary AC-065 lints against is the SHIPPED
/// one, and a fixture registry of the implementer's choosing is not that claim.
const SHIPPED_FACETS: &str = "spikes/p9/facets.toml";

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

pub(crate) fn registry() -> ArtifactRegistry {
    match ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS) {
        Ok(r) => r,
        Err(e) => panic!("the fixture artifact registry must load: {e}"),
    }
}

pub(crate) fn facets() -> rk_kernel::registry::Registry {
    let path = workspace_root().join(SHIPPED_FACETS);
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry at {} is readable: {e}",
            path.display()
        )
    });
    match rk_kernel::registry::Registry::from_toml(&src) {
        Ok(r) => r,
        Err(e) => panic!("the shipped facet registry must load: {e}"),
    }
}

/// **Step 2's own rule** (design §4.4 row 2): `R(stage, ctx)` = `baseline` ∪ the artifacts of every
/// `require` row whose `when` holds. A row whose `when` is **false** contributes nothing and is not
/// an error; that distinction is the whole of the step-2/step-4 boundary.
#[test]
fn baseline_union_fired_require_rows_is_the_required_set() {
    let pack = pack();
    let registry = registry();
    let facets = facets();

    // criticality = high fires row 1; external_dep = false does not fire row 2.
    let item = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "high")
        .with_facet_bool("external_dep", false);
    let out = match step2(&pack, &registry, &facets, "analysis", &item) {
        Ok(o) => o,
        Err(e) => panic!("step 2 must resolve this context: {e}"),
    };
    let names: Vec<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert_eq!(names, vec!["CausalChain", "IncidentTimeline", "RootCause"]);
    assert_eq!(out.rows_fired(), 1, "one of the two require rows fired");
    assert!(
        out.gates().is_empty(),
        "a row that evaluated false opens no gate: {:?}",
        out.gates()
    );

    // criticality = low fires neither row: R is the baseline alone.
    let quiet = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_bool("external_dep", false);
    let out = step2(&pack, &registry, &facets, "analysis", &quiet).expect("resolves");
    let names: Vec<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert_eq!(names, vec!["IncidentTimeline", "RootCause"]);
    assert_eq!(out.rows_fired(), 0);
}

/// **AC-079, and the step-2/step-4 boundary the stage file fixes at §7b:507-509.** A top-level
/// `null` is the **abstention** case: it emits `gate_opened { reason: unknown_facet, axis }`,
/// **counted once per read facet, never per abstaining facet**, and it is **never**
/// `Unsatisfiable` — which is reserved for step 4 over predicates that evaluated to **false**.
#[test]
fn unknown_facet_abstains_and_is_not_unsatisfiable() {
    let pack = pack();
    let registry = registry();
    let facets = facets();

    // `external_dep` is nullable (`facets.toml`: source = "human", nullable = true) and holds no
    // evidence. The row `when = "external_dep"` is therefore `null` at the top level.
    let item = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_null("external_dep");
    let out =
        step2(&pack, &registry, &facets, "analysis", &item).expect("abstention is not an error");

    let gates = out.gates();
    assert_eq!(gates.len(), 1, "one gate, not one per row: {gates:?}");
    assert_eq!(gates[0].reason, GateReason::UnknownFacet);
    assert_eq!(gates[0].axis, "external_dep");
    // "...and NEVER `Unsatisfiable`, which is step 4's." `Step2Outcome` has no `unsatisfiable`
    // accessor at all - step 2 cannot construct one - so the claim is asserted where it can
    // actually be falsified: over the whole steps-1-to-4 path, where a wrong implementation that
    // routed the `null` on to step 4 WOULD gate `APIContract`.
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");
    let whole = rk_kernel::select::select(&pack, &catalog, &registry, &facets, "analysis", &item)
        .expect("selection resolves");
    let gated: Vec<&str> = whole
        .unsatisfiable()
        .iter()
        .map(|u| u.kind.as_str())
        .collect();
    assert!(
        !gated.contains(&"APIContract"),
        "the abstaining row's kind must not be gated Unsatisfiable - the order is lint at load \
         (AC-069), gate at step 2 (AC-079), Unsatisfiable at step 4 (AC-054): {gated:?}"
    );
    assert_eq!(
        whole.gates().len(),
        1,
        "and the gate survives to the end of step 4: {:?}",
        whole.gates()
    );

    // The abstaining row contributed NOTHING to R, and no value was assigned for the facet: the
    // absence of evidence never resolves to `false`.
    let names: BTreeSet<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert!(
        !names.contains("APIContract"),
        "an abstaining row must not fire: {names:?}"
    );
    assert_eq!(out.rows_fired(), 0);

    // The negative companion AC-079 names in terms: a boolean whose evidence IS computable and
    // resolves to `false` must NOT open a gate, "or the rule is satisfied by a system that gates
    // everything".
    let evidenced = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_bool("external_dep", false);
    let out = step2(&pack, &registry, &facets, "analysis", &evidenced).expect("resolves");
    assert!(out.gates().is_empty(), "a computed `false` opens nothing");

    // "Counted once per READ facet, never per ABSTAINING facet": a second nullable facet that no
    // firing row reads opens nothing.
    let extra = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_null("external_dep")
        .with_facet_null("ui");
    let out = step2(&pack, &registry, &facets, "analysis", &extra).expect("resolves");
    let axes: Vec<&str> = out.gates().iter().map(|g| g.axis.as_str()).collect();
    assert_eq!(
        axes,
        vec!["external_dep"],
        "`ui` abstains but no row reads it, so it opens nothing"
    );
}

/// **AC-065.** A `require` row `when`, an `applicable_if` or a `claimed_predicate` that does not
/// resolve to an artifact presence, a digest comparison, a **registered facet value** or a typed
/// verdict fails load with `JudgementPredicate { location, text }`.
#[test]
fn judgement_predicate_fails_load() {
    let facets = facets();
    let err = rk_kernel::select::lint_selection_predicate(
        "packs/judgement.toml: stage.review.require[0].when",
        "stakeholders in agreement",
        &facets,
    )
    .expect_err("a judgement predicate must not load");
    assert_eq!(
        err.location,
        "packs/judgement.toml: stage.review.require[0].when"
    );
    assert_eq!(err.text, "stakeholders in agreement");
    assert!(
        err.to_string().contains("gate"),
        "the authoring guidance points at a gate node instead: {err}"
    );

    // The must-LOAD companion, without which the criterion is satisfied by a lint that rejects
    // everything. All three of these are the vocabulary the rule admits.
    for ok in [
        "criticality >= high",
        "external_dep or kind == project",
        "'text' in change",
    ] {
        rk_kernel::select::lint_selection_predicate("fixture", ok, &facets)
            .unwrap_or_else(|e| panic!("{ok:?} must load: {e}"));
    }
}

/// **AC-049 [measured, F35 §1.3].** A required kind declared `keyed_by = "toolchain"` expands to
/// one instance **per element of the item's `toolchain` set**, each with its own producer. This is
/// what replaces the withdrawn specificity tie-break (ADR-0073), and it is why step 4 never has to
/// break a tie between the Node and the UE `ChangeSummary` producer: they never contend for one
/// slot.
#[test]
fn keyed_kind_expands_per_toolchain() {
    let pack = pack();
    let registry = registry();
    let facets = facets();

    // `Toolchain::classify` is per affected path (ADR-0022 (i)), so the item carries the change
    // kinds ATTRIBUTED TO EACH TOOLCHAIN, and the item-level `change` facet is their union. That is
    // what makes AC-049's "evaluated per instance rather than once against the item" mean
    // something: the per-instance context narrows `toolchain` to the singleton and `change` to that
    // toolchain's own kinds.
    let mixed = Item::new("STORY-1", "story")
        .with_toolchain("node", ["text"])
        .with_toolchain("unreal", ["binary"]);
    let out = step2(&pack, &registry, &facets, "implement", &mixed).expect("resolves");

    let summaries: Vec<Option<&str>> = out
        .required()
        .kinds()
        .filter(|k| k.kind == "ChangeSummary")
        .map(|k| k.key.as_deref())
        .collect();
    assert_eq!(
        summaries,
        vec![Some("node"), Some("unreal")],
        "two ChangeSummary instances, one per toolchain — the F35 §1.3 defect was receiving one"
    );

    // A NON-keyed kind in the same baseline stays single, so the expansion is keyed-kind-scoped
    // and not a blanket per-toolchain fan-out.
    let reports: Vec<Option<&str>> = out
        .required()
        .kinds()
        .filter(|k| k.kind == "ImplReport")
        .map(|k| k.key.as_deref())
        .collect();
    assert_eq!(reports, vec![None]);

    // ---- and each instance gets ITS OWN producer, which is the half that repairs F35 §1.3 -------
    //
    // Expanding to two instances is not enough on its own: the measured defect was two producers
    // both admissible for one slot at rank 10, with specificity silently picking the UE one. What
    // makes them not contend is that the per-instance context is NARROWED — `toolchain` to the
    // singleton, `change` to that toolchain's own kinds — so `change-summary`'s `'text' in change`
    // is false for the `unreal` instance and `change-summary-ue`'s `toolchain:unreal` tag is false
    // for the `node` one. Without the narrowing both are admissible at rank 10 for the `unreal`
    // instance and selection fails with `RankTie`, which is the correct refusal and not the
    // required behaviour.
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");
    let whole = rk_kernel::select::select(&pack, &catalog, &registry, &facets, "implement", &mixed)
        .expect("selection resolves: the two producers never contend for one slot");
    let chosen: Vec<(Option<&str>, &str)> = whole
        .producers()
        .filter(|p| p.instance.kind == "ChangeSummary")
        .map(|p| (p.instance.key.as_deref(), p.activity_id.as_str()))
        .collect();
    assert_eq!(
        chosen,
        vec![
            (Some("node"), "change-summary"),
            (Some("unreal"), "change-summary-ue"),
        ],
        "one producer per instance, chosen on the NARROWED context — both are rank 10 and \
         specificity is withdrawn (ADR-0073), so nothing else could have told them apart"
    );

    // ---- and the LOAD-time lint accepts the pair, which is the other half of the same claim -----
    //
    // `change-summary` and `change-summary-ue` both provide `ChangeSummary` in `implement` at
    // `method_rank = 10`. If AC-047's lint reported that as a `RankTie`, the keyed mechanism would
    // not be a replacement for the withdrawn tie-break at all — it would be a runtime dodge around
    // a catalog the lint refuses. It does not, because `ChangeSummary` is registered
    // `keyed_by = "toolchain"` and the lint therefore enumerates only the SINGLETON toolchain
    // instances, which is exactly the set of contexts step 2 can present to a keyed kind's
    // producer filter.
    rk_kernel::select::rank_lint(&catalog, &registry, &facets).expect(
        "the shipped-shape keyed pair must LOAD: two rank-10 producers of one keyed kind are not \
         a tie, because the keyed expansion means they are never simultaneously admissible",
    );

    // Single-toolchain: one instance, still keyed, so the keyed kind's identity is stable across
    // the two cases rather than switching shape on set size.
    let single = Item::new("STORY-2", "story").with_toolchain("node", ["text"]);
    let out = step2(&pack, &registry, &facets, "implement", &single).expect("resolves");
    let summaries: Vec<Option<&str>> = out
        .required()
        .kinds()
        .filter(|k| k.kind == "ChangeSummary")
        .map(|k| k.key.as_deref())
        .collect();
    assert_eq!(summaries, vec![Some("node")]);
}

// ---------------------------------------------------------------------------------------------
// Purity — AC-040, AC-041
// ---------------------------------------------------------------------------------------------

/// `pub(crate)` since 2026-09-14: `facet_registry.rs` drives AC-072's positive `nullable = true`
/// fixture through the WHOLE steps-1-to-4 path to assert *"and never an `Unsatisfiable`"*, which
/// needs a catalog. Sharing this one is deliberate rather than convenient — a second copy of the
/// same eight files would let the two tests' notions of "the fixture catalog" drift apart.
pub(crate) const CATALOG: &[(&str, &str)] = &[
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

/// **AC-040 and AC-041.** `select()` through step 4 is a function of the four digests and of
/// nothing else, and *"byte-identical"* is identity **under the canonical serialiser** —
/// `rk_store::canon`, not `serde_json::to_vec`.
///
/// **The shuffle is of MAP CONSTRUCTION, not of call order** (§7b.9(2)). `serde_json` carries
/// `preserve_order` workspace-wide, so a plain `to_vec` over an insertion-ordered map leaks
/// authoring order into this assertion *and still passes 100 repetitions of the same call*. Every
/// repetition below rebuilds the item's facet bindings, the pack and the catalog in a different
/// insertion order and asserts the canonical bytes and the `inputs_digest` are unchanged.
#[test]
fn selection_seven_steps_are_pure() {
    let registry = registry();
    let facets = facets();
    let item_toolchains: Vec<(&str, &[&str])> =
        vec![("node", &["text"][..]), ("unreal", &["binary"][..])];

    let mut first_bytes: Option<Vec<u8>> = None;
    let mut first_digest: Option<String> = None;

    for rep in 0..100_u32 {
        // A deterministic permutation of construction order, seeded by the repetition index: no
        // clock and no random source reaches this crate, so the shuffle is arithmetic.
        let rotate = (rep as usize) % item_toolchains.len();
        let mut item = Item::new("STORY-1", "story");
        for i in 0..item_toolchains.len() {
            let (name, kinds) = item_toolchains[(i + rotate) % item_toolchains.len()];
            item = item.with_toolchain(name, kinds.iter().copied());
        }
        let mut sources: Vec<(&str, &str)> = CATALOG.to_vec();
        let n = sources.len();
        sources.rotate_left((rep as usize) % n);
        let catalog = Catalog::from_sources(&sources).expect("the fixture catalog loads");
        let pack = Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads");

        let selection =
            rk_kernel::select::select(&pack, &catalog, &registry, &facets, "implement", &item)
                .expect("selection resolves");

        // ---- steps 6 and 7, inside the same repetition ---------------------------------------
        //
        // **A purity loop over steps 1-5 alone would not reach the two steps most likely to leak
        // an input**: step 6 orders a graph (a place iteration order shows up) and step 7 re-runs
        // 2-5 and derives an identity (a place a counter, a clock or a random source would show
        // up). Both are folded into the canonical bytes below, so a non-deterministic plan or a
        // stored `deviation_id` fails this test rather than a later one.
        let plan = selection
            .plan()
            .expect("the assembled DAG is acyclic, so it orders");
        let request = rk_kernel::select::RequestDeviation::new(
            "ChangeSummary",
            rk_kernel::select::DeviationKind::Skip,
            "'text' in change",
            "prose that differs between repetitions is deliberately NOT varied here; its exclusion \
             from `deviation_id` is asserted in deviation.rs",
        );
        let deviation = rk_kernel::select::step7(
            &pack,
            &catalog,
            &registry,
            &facets,
            "implement",
            &item,
            "impl-report",
            &request,
            &selection,
        )
        .expect("step 7 resolves");

        let seven = serde_json::json!({
            "steps_1_to_5": String::from_utf8(
                selection.canonical_bytes().expect("canonical"),
            )
            .expect("utf-8"),
            "step_6_order": plan.order(),
            "step_7_deviation_id": request.deviation_id().expect("derivable").as_str(),
            "step_7_event": deviation.event().map(|e| e.payload),
        });
        let bytes = rk_store::canon::canonical(&seven).expect("canonical");
        let digest = selection.inputs_digest().as_str().to_owned();
        match (&first_bytes, &first_digest) {
            (None, None) => {
                first_bytes = Some(bytes);
                first_digest = Some(digest);
            }
            (Some(b), Some(d)) => {
                assert_eq!(
                    &bytes, b,
                    "repetition {rep} produced different canonical bytes; construction order \
                     leaked into the serialisation"
                );
                assert_eq!(
                    &digest, d,
                    "repetition {rep} produced a different inputs_digest"
                );
            }
            _ => unreachable!("the two are set together"),
        }
    }

    let bytes = first_bytes.expect("100 repetitions ran");
    assert!(
        !bytes.is_empty(),
        "100 repetitions over an EMPTY serialisation would pass this test trivially"
    );

    // ---- and the serialisation is the CANONICAL one, not `serde_json::to_vec` ------------------
    //
    // **This block is here because the 100 repetitions above do not discriminate the two, and that
    // was measured rather than assumed.** Replacing `rk_store::canon::canonical` with
    // `serde_json::to_vec` and re-running the loop above passes: the shuffle re-orders the catalog
    // sources and the toolchain insertions, but every one of those goes through a `BTreeMap` or a
    // sorted `Vec` before it reaches the JSON map, so the map's own insertion order never varies
    // between repetitions. That is precisely §7b.9(2)'s warning — *"a plain `to_vec` over an
    // insertion-ordered map leaks construction order into AC-040's assertion AND CAN STILL PASS ITS
    // 100 SHUFFLED REPETITIONS if the shuffle is of call order rather than of map construction"* —
    // and without what follows this test would be satisfied by the implementation the criterion
    // exists to refuse.
    //
    // The discriminating property is the canonical form's first rule: **object keys are sorted
    // byte-wise**. `serde_json` carries `preserve_order` workspace-wide, so a `to_vec` emits them
    // in the order the builder inserted them.
    // The key-order discrimination is over `Selection::to_json`'s OWN canonical bytes, not the
    // seven-step wrapper above: the wrapper's keys are this test's and would assert its own
    // construction order rather than the serialiser's rule.
    let selection = rk_kernel::select::select(
        &Pack::from_toml("incident-forbidden.toml", INCIDENT).expect("loads"),
        &Catalog::from_sources(CATALOG).expect("loads"),
        &registry,
        &facets,
        "implement",
        &Item::new("STORY-1", "story").with_toolchain("node", ["text"]),
    )
    .expect("selection resolves");
    let text = String::from_utf8(selection.canonical_bytes().expect("canonical")).expect("UTF-8");
    let keys = top_level_keys(&text);
    let mut sorted = keys.clone();
    sorted.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
    assert_eq!(
        keys, sorted,
        "the canonical form sorts object keys byte-wise; these are in construction order, which \
         means the serialisation is not `rk_store::canon`"
    );
    // …and the two orders genuinely differ, or the assertion above is vacuous.
    let insertion_order = vec![
        "stage",
        "rigor",
        "required",
        "classifications",
        "producers",
        "gates",
        "skipped",
        "unsatisfiable",
        "drift",
        "dag",
        "appetite_gate",
        "rows_fired",
        "inputs_digest",
    ];
    assert_ne!(
        keys, insertion_order,
        "`Selection::to_json` must insert its keys in an order that is NOT byte order, or the \
         sortedness assertion above cannot tell `canon` from `to_vec`"
    );
    assert!(
        !text.ends_with('\n') && !text.contains('\r'),
        "LF only, no trailing newline (rk_store::canon's second rule)"
    );
}

/// The top-level object keys of a canonical JSON document, in the order they appear.
///
/// Hand-written rather than taken from a parser, because a parser under `preserve_order` would
/// report *its own* map order and the question here is what the BYTES say.
fn top_level_keys(text: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let bytes = text.as_bytes();
    let mut depth = 0_i32;
    let mut i = 0;
    let mut in_string = false;
    let mut escaped = false;
    let mut start = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
            } else if c == b'\\' {
                escaped = true;
            } else if c == b'"' {
                in_string = false;
                // A string that closes at depth 1 and is followed by `:` is a top-level key.
                if depth == 1 && bytes.get(i + 1) == Some(&b':') {
                    keys.push(text[start + 1..i].to_owned());
                }
            }
        } else {
            match c {
                b'"' => {
                    in_string = true;
                    start = i;
                }
                b'{' | b'[' => depth += 1,
                b'}' | b']' => depth -= 1,
                _ => {}
            }
        }
        i += 1;
    }
    keys
}

// ---------------------------------------------------------------------------------------------
// AC-077 — the case §7b.9(7) says neither section has a fixture for
// ---------------------------------------------------------------------------------------------

/// **AC-077 + §7b.9(7): BOTH, NOT EITHER.**
///
/// AC-051's load-time failure covers only the **unconditional** `requires`, so a **conditional**
/// one (`"Spec?"`) reaching a forbidden stage **loads**, and the runtime case is live. ADR-0057's
/// Decision — *"`forbidden` means … nothing in that stage runs, even via `requires`"* — gives
/// AC-077's skip row, in `dag_assembled.skipped`; AC-054's *"no dropping the artifact from `R`"*
/// gives the gate.
///
/// The failure mode this fixture exists to catch: `forbidden` is a member of the `Rigor` ordinal
/// (AC-050) while behaving as a stage class, so the arithmetic route through AC-052 *looks* right —
/// it finds "no candidates" and **silently drops the requirement**.
#[test]
fn forbidden_stage_via_conditional_requires() {
    let pack = pack();
    let registry = registry();
    let facets = facets();
    let catalog = Catalog::from_sources(CATALOG).expect("the fixture catalog loads");

    // The pack LOADS. That is the premise: AC-051 does not reach a conditional `requires`.
    assert_eq!(
        pack.rigor("specification"),
        Some(Rigor::Forbidden),
        "specification is forbidden for this item class"
    );

    let item = Item::new("INC-1", "incident").with_toolchain("node", ["text"]);
    let out = rk_kernel::select::select(&pack, &catalog, &registry, &facets, "implement", &item)
        .expect("selection resolves: unsatisfiability is data in the graph, not an abort");

    // (a) ZERO DISPATCHES of the forbidden-stage activity. Nothing in `R`'s producer set names it.
    assert!(
        !out.producers().any(|p| p.activity_id == "spec-authoring"),
        "a forbidden stage's activity must never be selected, including transitively through \
         another activity's `requires`"
    );

    // (a2) …and exactly ONE producer per required instance, with the `requires` recursion obeying
    // the keyed expansion. `impl-report` requires `Changeset`, which is registered
    // `keyed_by = "toolchain"` and is ALSO in the stage's baseline. Two defects live here and both
    // were found by looking at the output rather than by an assertion, so both are pinned now: a
    // `requires` edge that expands un-keyed gives a mixed item ONE `Changeset` instead of two (the
    // F35 §1.3 defect arriving by a different road), and a seen-set keyed on the whole `Instance`
    // — `required_by` included — resolves the same artifact once per requiring activity and
    // double-counts it in `dag_assembled.nodes`.
    let rows: Vec<(String, &str)> = out
        .producers()
        .map(|p| (p.instance.to_string(), p.activity_id.as_str()))
        .collect();
    assert_eq!(
        rows,
        vec![
            ("ChangeSummary[node]".to_owned(), "change-summary"),
            ("Changeset[node]".to_owned(), "implement-change"),
            ("ImplReport".to_owned(), "impl-report"),
        ],
        "one producer per instance, each keyed instance keyed, no duplicates"
    );

    // (a3) …and ZERO DISPATCHES is **counted**, not read off the producer list. Step 6's plan is
    // the only source of an execution order in this crate, so walking it and counting is what
    // "zero dispatches" can mean before the runtime exists.
    let plan = out.plan().expect("the assembled DAG is acyclic");
    let dispatched: Vec<&str> = plan.order();
    assert!(
        !dispatched.contains(&"spec-authoring"),
        "the forbidden stage's activity is not in the execution order: {dispatched:?}"
    );
    assert_eq!(
        dispatched.len(),
        3,
        "three nodes ordered, none of them the forbidden stage's: {dispatched:?}"
    );

    // (b) THE SKIP ROW, naming the stage, **in `dag_assembled.skipped`** — §7b.9(7) says the row
    // lives in the EVENT, so it is read out of the event here rather than only off the struct.
    let row = out
        .skipped()
        .iter()
        .find(|r| r.reason == SkipReason::ForbiddenStageViaRequires)
        .expect("a skip row naming the forbidden stage reached through `requires`");
    assert_eq!(row.stage, "specification");
    assert_eq!(row.kind.as_deref(), Some("Spec"));
    assert_eq!(row.requiring_activity.as_deref(), Some("impl-report"));

    let event = out.dag().assembled_event();
    assert_eq!(event.kind, rk_types::event_kind::EventKind::DagAssembled);
    let skipped = event.payload["skipped"]
        .as_array()
        .expect("`dag_assembled.skipped` is an array");
    assert!(
        skipped.iter().any(|r| {
            r["stage"] == "specification"
                && r["kind"] == "Spec"
                && r["reason"] == SkipReason::ForbiddenStageViaRequires.as_str()
        }),
        "the skip row is IN `dag_assembled.skipped`, naming the stage: {skipped:?}"
    );

    // (c) AND an `Unsatisfiable` gate for the kind. A skip row alone is not enough.
    let gated: Vec<&str> = out
        .unsatisfiable()
        .iter()
        .map(|u| u.kind.as_str())
        .collect();
    assert!(
        gated.contains(&"Spec"),
        "the requirement is GATED, not dropped: {gated:?}"
    );

    // (d) …with `R` unchanged. Nothing was trimmed to make the graph resolvable.
    //
    // **WHAT STOOD HERE UNTIL 2026-09-13 WAS A TAUTOLOGY**, and it carried AC-054's name:
    //
    // ```ignore
    // let before = out.required().clone();
    // assert_eq!(out.required(), &before, "R is byte-identical across the gate");
    // ```
    //
    // — a value compared to a clone of itself taken on the previous line, which no implementation
    // could fail. `R` is built by step 2 and the claim is that steps 3-5 do not edit it, so the
    // comparison has to be against a SEPARATELY COMPUTED step-2 `R` over the same inputs. That is
    // what this is. (The byte-level form of the same claim — both sides under `rk_store::canon` —
    // needs `RequiredSet::to_json`, which is `pub(in crate::select)` by AC-061, so it lives at
    // `src/select.rs`'s `r_is_byte_identical_across_an_unsatisfiable_gate` under `#[cfg(test)]`.
    // This is the reachable half, and it discriminates: a step 4 or 5 that trimmed `R` fails it.)
    let two = step2(&pack, &registry, &facets, "implement", &item).expect("step 2 resolves");
    assert_eq!(
        out.required(),
        two.required(),
        "R after the gate is step 2's R unchanged (AC-054's 'no dropping the artifact from R')"
    );
    assert!(
        !two.required().is_empty(),
        "an empty R would make the comparison above vacuous"
    );
    let names: Vec<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert!(
        names.contains(&"ImplReport"),
        "the requiring activity's own kind stays in R: {names:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// AC-041 and AC-525 — the halves that are NOT enforced by a lint, enforced structurally
// ---------------------------------------------------------------------------------------------

/// The spellings a hidden input would arrive under, each with the criterion it violates.
///
/// **This list is the instrument, and `tests/fixtures/purity/banned-spellings.rs.txt` is its
/// positive control**: every entry here must be FOUND in that fixture, so a pattern that silently
/// stopped matching cannot read as a clean module.
const BANNED: &[(&str, &str)] = &[
    // AC-041 — the four bans `src/select.rs`'s header states. `clippy.toml`'s
    // `disallowed-methods` table covers the clock, randomness and sleeps WORKSPACE-WIDE and the
    // module-level `#![deny]` makes them errors here; the fs and env halves it does NOT cover, and
    // could not without putting an `#[expect]` on every legitimate read in `rk-git`, `rk-store` and
    // this crate's own loaders — an edit to two crates whose checkpoints have passed. So the clock
    // entries below are belt-and-braces over a lint that already bites (`select-clock-reintroduce`
    // is what shows it does) and the fs/env entries are the only thing standing behind their half.
    ("std::time::SystemTime", "AC-041 — the wall clock"),
    ("std::time::Instant", "AC-041 — a monotonic instant"),
    ("rand::", "AC-041 — randomness"),
    ("thread::sleep", "AC-041 — a blocking sleep"),
    (
        "std::fs",
        "AC-041 — nothing reachable from select() opens a file",
    ),
    (
        "fs::read",
        "AC-041 — a file read through a `use std::fs` import",
    ),
    (
        "fs::write",
        "AC-041 — a file write through a `use std::fs` import",
    ),
    ("File::open", "AC-041 — a file handle"),
    (
        "std::env",
        "AC-041 — nothing reachable from select() reads an environment variable",
    ),
    (
        "env::var",
        "AC-041 — an env read through a `use std::env` import",
    ),
    (
        "std::process",
        "AC-001..AC-004 — a subprocess is an input select() does not declare",
    ),
    (
        "Command",
        "AC-001..AC-004; git shells out through exactly one wrapper, in rk-git",
    ),
    // AC-525's negative half, as its 2026-09-13 amendment clause 1 words it: **`select()` NEVER
    // reads the event log.** D3 took the `rk-store` dependency for `rk_store::canon`, which puts
    // every OTHER module of that crate one path away from inside the pure function — so the
    // absence needs an instrument and not a sentence. `rk_store::canon` is the one admitted path
    // and is deliberately absent from this list.
    (
        "rk_store::conn",
        "AC-525 — the store is not reachable from the pure function",
    ),
    ("rk_store::memo", "AC-525"),
    ("rk_store::identity", "AC-525"),
    ("rk_store::locator", "AC-525"),
    ("rk_store::retention", "AC-525"),
    ("rk_store::tier", "AC-525"),
    ("rk_store::schema_lint", "AC-525"),
    (
        "rk_ports",
        "AC-525 — no port is reachable from the pure function",
    ),
    ("EventStore", "AC-525 — select() never reads the event log"),
    ("event_store", "AC-525 — nor its module"),
];

/// Every shipped line of the selection module, comments stripped.
///
/// **Comments are stripped, and that is correct HERE while it would be wrong for an attribute
/// grep.** A comment cannot introduce a hidden input; the hazard that makes `s6`:249-250 anchor
/// attribute greps on a declaration runs the other way. `src/select.rs`'s own header says the words
/// *"`EventStore`"* and *"opens a file"* in prose, and a scanner that read those as violations would
/// refuse the very module it is written to protect (golden constraint 9 — a guard scoped wrong
/// refuses valid work). The limitation is stated rather than hidden: this reads SPELLINGS, so a
/// determined alias (`use std::fs as f;`) would evade it. What it makes impossible is the
/// accidental reintroduction, and — with the fixture control below — a pattern list that has rotted.
///
/// The `#[cfg(test)]` module at the foot of `src/select.rs` and `src/select/step5.rs` is excluded:
/// those fixture loaders legitimately read `spikes/p9/facets.toml`, and they are not shipped code.
/// The cut is on a `#[cfg(test)]` at **column 0** only, so the INDENTED `#[cfg(test)]` on
/// `src/select/step5.rs`'s cyclic-`Dag` constructor does not truncate the scan of the 290 shipped
/// lines that follow it.
fn shipped_selection_lines() -> Vec<(String, usize, String)> {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = vec![src_dir.join("select.rs")];
    let mut nested: Vec<std::path::PathBuf> = std::fs::read_dir(src_dir.join("select"))
        .expect("crates/rk-kernel/src/select/ is readable")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    nested.sort();
    assert!(
        nested.len() >= 12,
        "the selection module has {} files — a scan over a directory that moved would report \
         nothing and pass",
        nested.len()
    );
    files.extend(nested);

    let mut out = Vec::new();
    let mut controls_skipped = 0usize;
    for path in files {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut in_control = false;
        for (i, line) in text.lines().enumerate() {
            if line == "#[cfg(test)]" {
                break;
            }
            // THE COMMITTED MUST-FAIL CONTROL IS EXCLUDED, AND IT IS THE ONLY EXCLUSION.
            // `reads_the_wall_clock` is one `SystemTime::now()` behind
            // `#[cfg(feature = "select-clock-reintroduce")]`, committed rather than reverted
            // (DECISIONS-2026-09-12 E4) precisely so the module-level `#![deny]` can be shown to
            // BITE. Scanning it would make this test refuse the instrument that proves the other
            // half. The cut is exact: from the `#[cfg(feature = ...)]` attribute to the next `}` at
            // column 0, and the count is asserted below so a scanner that skipped a whole file
            // could not pass.
            if line.starts_with("#[cfg(feature = \"select-clock-reintroduce\")]") {
                in_control = true;
                controls_skipped += 1;
                continue;
            }
            if in_control {
                if line == "}" {
                    in_control = false;
                }
                continue;
            }
            out.push((name.clone(), i + 1, strip_comment(line)));
        }
    }
    assert_eq!(
        controls_skipped, 1,
        "exactly one committed determinism control is expected in the selection module; skipping          more would be this scanner excluding real code"
    );
    out
}

/// Drop a `//` line comment, including the `///` and `//!` doc forms.
fn strip_comment(line: &str) -> String {
    line.find("//")
        .map_or_else(|| line.to_owned(), |i| line[..i].to_owned())
}

/// **AC-041's fs and env halves, and AC-525's negative half, made structural.**
///
/// 🛑 **Read why this exists before reading its green.** `src/select.rs`'s header asserts four bans
/// in one sentence — *"Nothing reachable from `select` opens a file, reads an environment variable,
/// draws a random number or reads a clock"* — and until 2026-09-13 **two of the four were prose**.
/// Measured on that tree:
///
/// ```text
/// SystemTime::now()  inserted into select_amended  → clippy rc=101, disallowed method   (caught)
/// std::env::var("RK_STAGE_OVERRIDE")  …CHANGING the selected stage → clippy clean, 59 tests green
/// std::fs::read_to_string("/nonexistent")           → clippy clean, 59 tests green
/// rk_store::conn::Reader::open(".git")              → clippy clean, 59 tests green
/// ```
///
/// `clippy.toml`'s `disallowed-methods` names `std::env::set_var`/`remove_var` (there for ADR-0083's
/// edition-2024 reason, not for AC-041) and no `std::fs` entry at all — and it CANNOT gain one
/// cheaply, because the file has no path or module scoping (clippy.toml:1-3) and every legitimate
/// read in `rk-git`, `rk-store` and this crate's own `catalog.rs` would then need an `#[expect]`,
/// editing two crates whose checkpoints have already passed. AC-525's negative half was in the same
/// position and worse: D3 put `rk-store` on the dependency edge, and the only instrument was a
/// 100-repetition identity test that a log-reading implementation returning the same value each
/// time would pass.
///
/// So the instrument is structural and in-crate. Its positive control is the fixture, which makes
/// the assertion *"this scanner finds these spellings"* rather than *"this scanner reported none"*.
#[test]
fn selection_source_reads_no_hidden_input() {
    // ---- the control FIRST: the scanner must find every banned spelling where one exists -------
    let control = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/purity/banned-spellings.rs.txt"),
    )
    .expect("the positive-control fixture is committed and readable");
    let control_body: String = control
        .lines()
        .map(strip_comment)
        .collect::<Vec<_>>()
        .join("\n");
    let missed: Vec<&str> = BANNED
        .iter()
        .map(|(t, _)| *t)
        .filter(|t| !control_body.contains(t))
        .collect();
    assert!(
        missed.is_empty(),
        "the positive control does not exercise {missed:?} — a ban with no control line is a \
         pattern nothing shows to match, and a scanner whose list has rotted reports the same \
         clean answer as a clean module"
    );

    // ---- then the real scan --------------------------------------------------------------------
    let lines = shipped_selection_lines();
    assert!(
        lines.len() > 2_000,
        "only {} shipped lines were scanned — an under-scan passes vacuously",
        lines.len()
    );

    let mut hits: Vec<String> = Vec::new();
    for (file, no, body) in &lines {
        for (token, why) in BANNED {
            if body.contains(token) {
                hits.push(format!(
                    "{file}:{no}  `{token}`  — {why}\n    {}",
                    body.trim()
                ));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "the pure selection function names {} hidden input(s). AC-041 makes the four digests its \
         ONLY inputs and AC-525's amendment clause 1 says select() NEVER reads the event log:\n{}",
        hits.len(),
        hits.join("\n")
    );
    println!(
        "AC-041/AC-525: {} shipped lines of src/select.rs and src/select/ scanned for {} banned \
         spellings, 0 hits; the control fixture carries all {}",
        lines.len(),
        BANNED.len(),
        BANNED.len()
    );
}

/// **D21-73 item 3 (R8 = ADR-0179 decision 3, proposed; `s8d` build, package F2-D2173).** A `require`
/// row's `when` may read `rigor` — `facets.toml` makes it a `kernel` facet (*"a lookup over
/// policy.<class>.stages"*, ADR-0093 decision 1) — and step 2 binds it as the run's rigor at this
/// stage: step 1's, with the item's logged override folded in. Before, step 2 evaluated `when` over
/// `item.context()`, which binds no `rigor`, and every such row refused as an unknown field (the
/// shipped story lane's `implement` and both lanes' `closeout`).
#[test]
fn a_require_row_keyed_on_rigor_reads_the_stages_rigor() {
    let registry = registry();
    let facets = facets();
    let pack = Pack::from_toml(
        "rigor-rows.toml",
        "[pack]\nid = \"rigor-rows\"\napplies = \"kind == story\"\n[policy]\n\
         stages = { design = \"recommended\" }\n[stage.design]\nbaseline = []\n\
         [[stage.design.require]]\nwhen = \"rigor >= recommended\"\nartifacts = [\"DesignDoc\"]\n\
         [[stage.design.require]]\nwhen = \"rigor >= mandatory\"\nartifacts = [\"ProblemStatement\"]\n",
    )
    .expect("the inline pack loads");

    // At the pack's own `recommended`, the first row fires and the second does not.
    let item = Item::new("STORY-1", "story");
    let out = step2(&pack, &registry, &facets, "design", &item)
        .unwrap_or_else(|e| panic!("a `when` over `rigor` evaluates at step 2: {e}"));
    let names: Vec<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert_eq!(
        names,
        vec!["DesignDoc"],
        "rigor = recommended fires only the first row"
    );
    assert_eq!(out.rows_fired(), 1);

    // The item's logged override is the run's rigor too: at `mandatory` both rows fire.
    let raised = Item::new("STORY-1", "story").with_rigor_override("design", Rigor::Mandatory);
    let out = step2(&pack, &registry, &facets, "design", &raised)
        .unwrap_or_else(|e| panic!("resolves under the override: {e}"));
    let names: Vec<&str> = out.required().kinds().map(|k| k.kind.as_str()).collect();
    assert_eq!(
        names,
        vec!["DesignDoc", "ProblemStatement"],
        "rigor = mandatory (the item's override) fires both rows"
    );
    assert_eq!(out.rows_fired(), 2);
}

// ---- `s8g` section 8g.2: `KindSource::Pass` (R1 = ADR-0191 decision 1, proposed). -------------
// *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT.)*

/// A process pack whose one stage is named `check`, not `verify`, so nothing here can be read off a
/// stage literal (ADR-0162 decision 5), requiring `TestReport` as its baseline.
const PASS_PACK: &str = r#"[pack]
id = "pass-fixture"
applies = "kind == incident"
appetite = { points = 50 }

[policy]
stages = { check = "mandatory" }

[stage.check]
baseline = ["TestReport"]
"#;

/// One catalog activity at `check` providing `TestReport`, agent-bearing, as the shipped
/// `unit-tests` was: the producer a pass-sourced kind must NOT select.
const PASS_CATALOG: &[(&str, &str)] = &[(
    "unit-tests.toml",
    r#"[activity]
id           = "unit-tests"
version      = 1
stage        = "check"
provides     = ["TestReport"]
requires     = []
derived_from = []
tags         = ["kind:incident"]
method_rank  = 10
min_rigor    = "optional"
cost_class   = "S"

[[step]]
id      = "gaps"
kind    = "agent"
outputs = "value:TestReport"

[[step]]
id   = "emit"
kind = "emit"
"#,
)];

fn test_report_registry(source: &str) -> ArtifactRegistry {
    let src = format!("[artifact.TestReport]\nsource = \"{source}\"\nschema = \"TestReport\"\n");
    ArtifactRegistry::from_toml("artifacts.toml", &src)
        .unwrap_or_else(|e| panic!("the one-row registry (source = {source}) loads: {e}"))
}

/// **R1 = ADR-0191 decision 1** (`s8g` section 8g.2): *"The selector selects no catalog producer
/// for a `pass` kind; stage completeness still requires its `artifact.write`."* Over one fixture
/// whose catalog DOES provide the kind, so the control can select it:
///
/// - `source = "activity"` (the control): `unit-tests` is selected and planned.
/// - `source = "pass"`: no producer, no plan node, no `Unsatisfiable` gate; `TestReport` stays in
///   `R`, `Missing`, and is named by `pass_sourced()` as what the stage still owes.
/// - The same, with the artifact materialised (the pass's `artifact.write` read back): `Present`,
///   and nothing is owed.
#[test]
fn a_pass_sourced_kind_selects_no_catalog_producer() {
    let pack = Pack::from_toml("pass-fixture.toml", PASS_PACK).expect("the fixture pack loads");
    let catalog = Catalog::from_sources(PASS_CATALOG).expect("the fixture catalog loads");
    let facets = facets();
    let item = Item::new("INC-9", "incident");
    let report = rk_kernel::select::Instance::of("TestReport");

    // CONTROL: the same catalog under `source = "activity"` selects its producer, so the absence
    // below is the source's doing and not a fixture that selects nothing.
    let control = rk_kernel::select::select(
        &pack,
        &catalog,
        &test_report_registry("activity"),
        &facets,
        "check",
        &item,
    )
    .expect("the control selects");
    let control_rows: Vec<&str> = control
        .producers()
        .map(|p| p.activity_id.as_str())
        .collect();
    assert_eq!(
        control_rows,
        vec!["unit-tests"],
        "CONTROL: an activity-sourced TestReport selects the catalog producer"
    );
    assert!(
        control.pass_sourced().is_empty(),
        "CONTROL: an activity-sourced kind is owed by no pass"
    );

    let out = rk_kernel::select::select(
        &pack,
        &catalog,
        &test_report_registry("pass"),
        &facets,
        "check",
        &item,
    )
    .expect("a pass-sourced kind selects");
    let rows: Vec<String> = out
        .producers()
        .map(|p| format!("{} <- {}", p.instance, p.activity_id))
        .collect();
    assert!(
        rows.is_empty(),
        "a pass-sourced kind selected a catalog producer (M16's shape): {rows:?}"
    );
    let plan = out.plan().expect("the DAG orders");
    assert!(
        plan.order().is_empty(),
        "a catalog activity is planned for a pass-sourced kind: {:?}",
        plan.order()
    );
    assert!(
        out.unsatisfiable().is_empty(),
        "a pass-sourced kind is not unsatisfiable: its producer is the pass: {:?}",
        out.unsatisfiable()
    );
    // Stage completeness still needs the write: the instance stays in `R`, `Missing`, and owed.
    assert!(
        out.required().kinds().any(|i| *i == report),
        "TestReport left R: nothing would then require its artifact.write"
    );
    assert_eq!(
        out.classifications().get(&report),
        Some(&rk_kernel::select::Classification::Missing)
    );
    assert_eq!(
        out.pass_sourced(),
        std::slice::from_ref(&report),
        "the stage does not name the pass-sourced instance it still owes"
    );

    // Once the pass's write has materialised it, nothing is owed and still nothing is selected.
    let written = item.with_artifact(
        rk_kernel::select::ArtifactState::new(&report).exists(rk_types::digest::sha256(b"report")),
    );
    let done = rk_kernel::select::select(
        &pack,
        &catalog,
        &test_report_registry("pass"),
        &facets,
        "check",
        &written,
    )
    .expect("selects");
    assert_eq!(
        done.classifications().get(&report),
        Some(&rk_kernel::select::Classification::Present)
    );
    assert!(
        done.pass_sourced().is_empty(),
        "a written pass kind is still owed"
    );
    assert_eq!(done.producers().count(), 0);
}

// ---- `s8g` section 8g.2 / 8g.5: N23's re-homing and AC-347's residual over the SHIPPED files. ----
// *(dated 2026-09-29, s8g build, package S8G-P4-PACK-REHOME; R1 = ADR-0191 decisions 3 to 7,
// proposed.)*

/// The four activities D21-43 N23 re-homes.
const REHOMED: [&str; 4] = [
    "unit-tests",
    "unit-tests-ue",
    "equivalence-boundary",
    "property-based-testing",
];

struct ShippedSel {
    pack: Pack,
    catalog: Catalog,
    artifacts: ArtifactRegistry,
    facets: rk_kernel::registry::Registry,
}

fn shipped_sel(pack_file: &str) -> ShippedSel {
    let red = workspace_root().join(".redkiln");
    let read = |p: std::path::PathBuf| {
        std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("{} reads: {e}", p.display()))
    };
    ShippedSel {
        pack: Pack::from_toml(pack_file, &read(red.join("packs").join(pack_file)))
            .unwrap_or_else(|e| panic!("the shipped pack {pack_file} loads: {e}")),
        catalog: Catalog::from_dir(&red.join("catalog"))
            .unwrap_or_else(|e| panic!("the shipped catalog loads: {e:?}")),
        artifacts: ArtifactRegistry::from_toml("artifacts.toml", &read(red.join("artifacts.toml")))
            .unwrap_or_else(|e| panic!("the shipped artifact registry loads: {e:?}")),
        facets: rk_kernel::registry::Registry::from_toml(&read(red.join("facets.toml")))
            .unwrap_or_else(|e| panic!("the shipped facet registry loads: {e:?}")),
    }
}

/// An item as `rk-cli`'s `build_item` binds one, every facet null, then the lane's values:
/// `criticality` as given and the other three ordinals `low`, `cos = standard`, `external_dep` as
/// given, `has_repro = true`, and one toolchain with its change kind.
fn shipped_item(
    s: &ShippedSel,
    kind: &str,
    criticality: &str,
    external_dep: bool,
    toolchain: (&str, &str),
) -> Item {
    let mut item = Item::new("S8G-P4-1", kind);
    for facet in s.facets.facets() {
        if facet.name == "kind" || facet.source == rk_kernel::registry::Source::Kernel {
            continue;
        }
        item = item.with_facet_null(&facet.name);
    }
    for (name, member) in [
        ("criticality", criticality),
        ("blast_radius", "low"),
        ("size", "low"),
        ("novelty", "low"),
    ] {
        let domain: Vec<String> = s
            .facets
            .facet(name)
            .unwrap_or_else(|| panic!("facets.toml declares `{name}`"))
            .values
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect();
        let refs: Vec<&str> = domain.iter().map(String::as_str).collect();
        item = item.with_facet_ordinal(name, &refs, member);
    }
    item.with_facet_str("cos", "standard")
        .with_facet_bool("external_dep", external_dep)
        .with_facet_bool("has_repro", true)
        .with_facet_set("toolchain", [toolchain.0])
        .with_toolchain(toolchain.0, [toolchain.1])
}

fn producers_for<'s>(sel: &'s rk_kernel::select::Selection, kind: &str) -> Vec<&'s str> {
    sel.producers()
        .filter(|p| p.instance.kind == kind)
        .map(|p| p.activity_id.as_str())
        .collect()
}

fn shipped_select(
    s: &ShippedSel,
    pack: &Pack,
    stage: &str,
    item: &Item,
) -> rk_kernel::select::Selection {
    rk_kernel::select::select(pack, &s.catalog, &s.artifacts, &s.facets, stage, item)
        .unwrap_or_else(|e| panic!("`{stage}` selects: {e:?}"))
}

/// **D21-43 N23** (`s8g` section 8g.2; R1 = ADR-0191 decisions 3 to 5): *"Re-homed to the
/// implement stage as test-authoring activities under ADR-0072. Verify only measures."* Over the
/// SHIPPED catalog, packs and registries:
///
/// - both packs' `criticality >= high` escalation demands `AuthoredTests` at `implement`, and it
///   selects a re-homed activity there (`equivalence-boundary`, rank 30: `unit-tests` stops at
///   `max_rigor = "recommended"` and both packs' implement stage is mandatory);
/// - at `verify`, no re-homed activity is selected, `TestReport` has no catalog producer and no
///   `Unsatisfiable` gate, and the stage names it as pass-owed (R1 decision 1);
/// - at `criticality = low` (the two `s8` lanes) no re-homed activity is selected at implement;
/// - over a probe pack requiring `AuthoredTests` at implement at `recommended`, the rank-20 members
///   are implement candidates: `unit-tests` for a Rust text change, `unit-tests-ue` for an Unreal
///   binary one; the same probe at `verify` has no candidate. M18's shape turns the probe red.
#[test]
fn the_rehomed_test_authoring_activities_select_at_implement() {
    for (pack_file, kind) in [("bug-standard.toml", "bug"), ("story.toml", "story")] {
        let s = shipped_sel(pack_file);
        let high = shipped_item(&s, kind, "high", false, ("rust", "text"));
        let implement = shipped_select(&s, &s.pack, "implement", &high);
        assert!(
            implement
                .required()
                .kinds()
                .any(|i| i.kind == "AuthoredTests"),
            "{pack_file}: the escalation did not move to implement over AuthoredTests (N23): R = {:?}",
            implement.required().kinds().collect::<Vec<_>>()
        );
        assert_eq!(
            producers_for(&implement, "AuthoredTests"),
            vec!["equivalence-boundary"],
            "{pack_file}: the re-homed test-design escalation does not select at implement"
        );

        let verify = shipped_select(&s, &s.pack, "verify", &high);
        let at_verify: Vec<&str> = verify
            .producers()
            .map(|p| p.activity_id.as_str())
            .filter(|a| REHOMED.contains(a))
            .collect();
        assert!(
            at_verify.is_empty(),
            "{pack_file}: a re-homed activity is still selected at verify: {at_verify:?}"
        );
        assert!(
            producers_for(&verify, "TestReport").is_empty(),
            "{pack_file}: TestReport has a catalog producer beside the pass (two routes)"
        );
        assert!(
            verify
                .unsatisfiable()
                .iter()
                .all(|u| u.kind != "TestReport"),
            "{pack_file}: a pass-sourced TestReport was gated Unsatisfiable"
        );
        assert!(
            verify.pass_sourced().iter().any(|i| i.kind == "TestReport"),
            "{pack_file}: the verify stage does not name TestReport as pass-owed: {:?}",
            verify.pass_sourced()
        );

        let low = shipped_item(&s, kind, "low", false, ("rust", "text"));
        let quiet = shipped_select(&s, &s.pack, "implement", &low);
        let lane: Vec<&str> = quiet
            .producers()
            .map(|p| p.activity_id.as_str())
            .filter(|a| REHOMED.contains(a))
            .collect();
        assert!(
            lane.is_empty(),
            "{pack_file}: at criticality low a re-homed activity is selected at implement: {lane:?}"
        );
    }

    // The probe: the shipped catalog and registries under a pack that asks for AuthoredTests at
    // implement at `recommended`, and the same ask at verify.
    let probe = Pack::from_toml(
        "rehome-probe.toml",
        concat!(
            "[pack]\nid = \"rehome-probe\"\napplies = \"kind == story\"\nappetite = { points = 60 }\n\n",
            "[policy]\nstages = { implement = \"recommended\", verify = \"recommended\" }\n\n",
            "[stage.implement]\nbaseline = [\"AuthoredTests\"]\n\n",
            "[stage.verify]\nbaseline = [\"AuthoredTests\"]\n",
        ),
    )
    .expect("the probe pack loads");
    let s = shipped_sel("story.toml");
    for ((tc, change), want) in [
        (("rust", "text"), "unit-tests"),
        (("unreal", "binary"), "unit-tests-ue"),
    ] {
        let item = shipped_item(&s, "story", "low", false, (tc, change));
        let at = shipped_select(&s, &probe, "implement", &item);
        assert_eq!(
            producers_for(&at, "AuthoredTests"),
            vec![want],
            "for a {tc} {change} change the rank-20 test-authoring member is not an implement \
             candidate (N23: re-homed to implement)"
        );
        let v = shipped_select(&s, &probe, "verify", &item);
        assert!(
            producers_for(&v, "AuthoredTests").is_empty()
                && v.unsatisfiable().iter().any(|u| u.kind == "AuthoredTests"),
            "for {tc}: an AuthoredTests producer still sits at verify: {:?}",
            v.producers().collect::<Vec<_>>()
        );
    }
}

/// **D21-46 R5 and D21-48 X9** (`s8g` section 8g.2; R1 = ADR-0191 decision 6): `integration-tests`
/// is re-homed whole, and `story.toml`'s `external_dep` -> `IntegrationReport` row moved with it.
/// Over `story.toml`, with `external_dep = true`, `integration-tests` is selected at implement and
/// at no other stage, and verify gates no `IntegrationReport`; with `external_dep = false` (the
/// control, and the story lane) it is selected at none. M21's shape (the row moved back to verify)
/// selects it nowhere and gates `IntegrationReport`.
#[test]
fn integration_tests_selects_at_implement_under_external_dep() {
    let s = shipped_sel("story.toml");
    let stages: Vec<String> = s.pack.stage_order().to_vec();
    assert!(
        stages.len() >= 10,
        "story.toml's stage order was not read: {stages:?}"
    );
    // A stage whose selection REFUSES selects nothing, so it cannot select `integration-tests`;
    // measured 2026-09-29, story's `design` refuses under `external_dep = true` with
    // `Step5(CyclicDag { path: ["adr-record", "design-doc", "adr-record"] })`, which is not this
    // stage's (no row this package moves reaches `design`). Such a refusal is printed, must not
    // name `integration-tests`, and `implement` and `verify` must select.
    let walk = |external_dep: bool| -> Vec<String> {
        let item = shipped_item(&s, "story", "low", external_dep, ("rust", "text"));
        let mut at = Vec::new();
        for stage in &stages {
            let sel = match rk_kernel::select::select(
                &s.pack,
                &s.catalog,
                &s.artifacts,
                &s.facets,
                stage,
                &item,
            ) {
                Ok(sel) => sel,
                Err(e) => {
                    let d = format!("{e:?}");
                    println!("s8g-p4 external_dep={external_dep} `{stage}` refuses: {d}");
                    assert!(
                        stage != "implement" && stage != "verify",
                        "story `{stage}` must select: {d}"
                    );
                    assert!(
                        !d.contains("integration-tests"),
                        "story `{stage}` refuses naming integration-tests: {d}"
                    );
                    continue;
                }
            };
            if sel
                .producers()
                .any(|p| p.activity_id == "integration-tests")
            {
                at.push(stage.clone());
            }
            if external_dep && stage == "verify" {
                assert!(
                    sel.unsatisfiable()
                        .iter()
                        .all(|u| u.kind != "IntegrationReport"),
                    "story's verify stage gates IntegrationReport, whose only producer sits at \
                     implement: the require row did not move (D21-46 R5)"
                );
            }
        }
        at
    };
    assert_eq!(
        walk(true),
        vec!["implement".to_owned()],
        "under external_dep, integration-tests is not selected at implement alone"
    );
    assert!(
        walk(false).is_empty(),
        "CONTROL: without external_dep integration-tests is selected somewhere"
    );
}

/// **D21-46 R5** (`s8g` section 8g.5; R1 = ADR-0191 decision 7): *"`migration-dry-run` stays a
/// named residual, because no Phase-1 lane reaches it (`data_migration` is NOT MEASURABLE)."*
/// Both shipped packs, over every cell of the shipped lattice (`Axes::shipped()`), with the
/// covering array's `data_migration` column forced to `1` in every row, select `migration-dry-run`
/// in none. The AC-347 lint cannot see reachability, so this is the test a later row that reaches
/// the residual turns red (M22).
///
/// **CONTROL** (the guard can go red): a copy of the shipped tree whose `story.toml` carries M22's
/// row (`[[stage.verify.require]] when = "external_dep"`, `artifacts = ["MigrationTestReport"]`),
/// walked over one Rust cell with `external_dep = 1` and `data_migration = 1`, DOES select it.
///
/// Red first: this pins 7e's outcome (the demand was DROPPED at 7e), so it is green on entry by
/// construction; its red is the control above and M22.
#[test]
fn migration_dry_run_is_selected_on_no_shipped_lane() {
    use rk_kernel::lattice::{self, Axes, Sources};
    let red = workspace_root().join(".redkiln");
    let inputs = lattice::load(&Sources::under(&red))
        .unwrap_or_else(|e| panic!("the shipped lattice inputs load: {e}"));
    assert!(
        inputs
            .catalog()
            .activities()
            .iter()
            .any(|a| a.id == "migration-dry-run"),
        "PREMISE: the shipped catalog holds migration-dry-run"
    );
    let dm = lattice::BOOLS
        .iter()
        .position(|b| *b == "data_migration")
        .expect("data_migration is a covering column");
    let mut axes = Axes::shipped();
    for row in &mut axes.covering_rows {
        row[dm] = 1;
    }
    let report = lattice::run(&inputs, &axes, None)
        .unwrap_or_else(|e| panic!("the shipped lattice runs: {e}"));
    assert_eq!(report.packs().len(), 2, "both shipped packs were walked");
    assert!(
        report.contexts("pack-default") > 0,
        "the forced run walked no context"
    );
    for pack in report.packs() {
        let sel = report.selected_in(pack);
        assert!(
            !sel.is_empty(),
            "{pack}: the lattice selected nothing, so an absence proves nothing"
        );
        assert!(
            !sel.contains("migration-dry-run"),
            "{pack} selects migration-dry-run in some cell with data_migration forced true: the \
             residual is reachable, which is AC-347's breach (D21-46 R5)"
        );
    }

    // CONTROL: M22's row, in a copy, over one cell.
    let probe = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("s8g-p4-m22-probe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&probe);
    for sub in ["catalog", "packs"] {
        std::fs::create_dir_all(probe.join(sub)).expect("the probe tree is created");
        for e in std::fs::read_dir(red.join(sub)).expect("the shipped directory reads") {
            let p = e.expect("an entry").path();
            if p.extension().is_some_and(|x| x == "toml") {
                std::fs::copy(&p, probe.join(sub).join(p.file_name().expect("a name")))
                    .expect("a copy");
            }
        }
    }
    for f in ["facets.toml", "artifacts.toml"] {
        std::fs::copy(red.join(f), probe.join(f)).expect("a copy");
    }
    let story = probe.join("packs").join("story.toml");
    let src = std::fs::read_to_string(&story).expect("the copied story pack reads");
    let anchor = "[stage.review]\n";
    assert_eq!(
        src.matches(anchor).count(),
        1,
        "the control's anchor is unique"
    );
    std::fs::write(
        &story,
        src.replace(
            anchor,
            concat!(
                "[[stage.verify.require]]\nwhen = \"external_dep\"\n",
                "artifacts = [\"MigrationTestReport\"]\n\n[stage.review]\n",
            ),
        ),
    )
    .expect("the control row is written");
    let ext = lattice::BOOLS
        .iter()
        .position(|b| *b == "external_dep")
        .expect("external_dep is a covering column");
    let mut one = Axes::shipped();
    one.toolchains.retain(|t| t.name == "rust");
    let mut row = one.covering_rows[0];
    row[dm] = 1;
    row[ext] = 1;
    one.covering_rows = vec![row];
    one.surfaces = Some(vec![Vec::new()]);
    let control = lattice::run(
        &lattice::load(&Sources::under(&probe)).expect("the control tree loads"),
        &one,
        Some(&["story"]),
    )
    .expect("the control runs");
    assert!(
        control.selected_in("story").contains("migration-dry-run"),
        "CONTROL: M22's row did not make migration-dry-run reachable, so the assertion above could \
         not go red"
    );
    let _ = std::fs::remove_dir_all(&probe);
}
