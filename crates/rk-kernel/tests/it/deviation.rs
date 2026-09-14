//! **Step 7 — deviation.** Design §4.4 row 7:
//!
//! > An activity may return `RequestDeviation { target, kind, claimed_predicate, why }`. The engine
//! > re-evaluates the predicate against evidence: true ⇒ **re-run 2–5 and log the delta**; false ⇒
//! > gate with the request and its evaluated operands attached.
//! >
//! > *Invariant:* **the agent never edits `R`.**
//!
//! AC-061, AC-062, AC-063 and AC-064, plus **ADR-0120** (the `dag_amended` kind, its tier, its
//! producing step and the two branches that write nothing) and the register's **2026-09-13
//! amendment to AC-064** (`deviation_id`'s derivation, and `why`'s exclusion from it).
//!
//! # The three branches, and why each needs its own negative
//!
//! A test that exercises only the accepted branch pins nothing: ADR-0120 decision 5's whole content
//! is that the *other two* branches write no event. So the false-predicate branch and the
//! refused-`kind` branch are each asserted to produce **no** `dag_amended`, by the same route the
//! accepted one is asserted to produce exactly one.
//!
//! # F37's empty denominator
//!
//! Deviation was **offered 71 times and used 0** across 61 runs, so P7 §4.4(3) is recorded as *not
//! answerable* rather than as 0%. AC-063's evidence line says what follows: *"the branch must be
//! exercised by fixtures, never by field data"*. Every case here is a fixture.

use rk_kernel::select::{
    ArtifactRegistry, Catalog, DeviationKind, DeviationOutcome, Item, Pack, RequestDeviation,
    select, step7,
};
use rk_types::event_kind::EventKind;

const DEVIATION: &str = include_str!("../fixtures/packs/incident-deviation.toml");
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

const BANDS: [&str; 4] = ["low", "med", "high", "critical"];

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
        Pack::from_toml("incident-deviation.toml", DEVIATION).expect("loads"),
        Catalog::from_sources(CATALOG).expect("loads"),
        ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("loads"),
        facets(),
    )
}

/// The item the whole file runs over: `criticality = critical`, so the conditional `require` row
/// fires and `ChangeSummary` is in `R` for a `skip` to remove; `external_dep = true` so an `add`
/// has a true predicate; `ui = false` so a `false` predicate is available without a `null`.
fn item() -> Item {
    Item::new("INC-1", "incident")
        .with_toolchain("node", ["text"])
        .with_facet_ordinal("criticality", &BANDS, "critical")
        .with_facet_bool("external_dep", true)
        .with_facet_bool("ui", false)
}

fn kinds(selection: &rk_kernel::select::Selection) -> Vec<String> {
    selection
        .required()
        .kinds()
        .map(ToString::to_string)
        .collect()
}

/// **AC-061 and AC-063.** *"`R` is unchanged at the moment the gate opens"*, and the gate carries
/// *"the request, the predicate and its evaluated operands attached"*.
///
/// The name is §7b.13's, verbatim.
#[test]
fn deviation_never_edits_the_required_set() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = item();
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");
    let before = base.required_canonical_bytes().expect("canonical");

    // A `claimed_predicate` that evaluates FALSE: `ui` is bound `false` on this item.
    let request = RequestDeviation::new(
        "ChangeSummary",
        DeviationKind::Skip,
        "ui",
        "the change is server-side only",
    );
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        "impl-report",
        &request,
        &base,
    )
    .expect("a false predicate is a GATE, not an error");

    let gate = match &outcome {
        DeviationOutcome::Gated(g) => g,
        other => panic!("expected a gate on a false claimed_predicate, got {other:?}"),
    };
    // (a) the request.
    assert_eq!(gate.request.target, "ChangeSummary");
    assert_eq!(gate.request.kind, DeviationKind::Skip);
    assert_eq!(gate.request.why, "the change is server-side only");
    // (b) the predicate, and its evaluated value — FALSE, never `null` coerced to false.
    assert_eq!(gate.request.claimed_predicate, "ui");
    assert_eq!(gate.value, rk_kernel::predicate::Tri::False);
    // (c) its evaluated OPERANDS, typed in rather than re-derived.
    assert_eq!(
        gate.operands,
        vec![("ui".to_owned(), rk_kernel::select::FacetValue::Bool(false))],
        "the gate carries the evaluated value of each facet the predicate read"
    );

    // (d) `R` is unchanged AT THE MOMENT THE GATE OPENS, byte for byte.
    assert_eq!(
        before,
        base.required_canonical_bytes().expect("canonical"),
        "R is byte-identical across the call"
    );
    // (e) …and the request was neither silently dropped nor auto-accepted.
    assert!(
        outcome.amended().is_none(),
        "a false predicate must not amend R"
    );
    assert!(
        outcome.event().is_none(),
        "ADR-0120 decision 5: the false branch writes NO `dag_amended`"
    );
}

/// **AC-062, the negative criterion.** *"GIVEN an activity returning `RequestDeviation` with a
/// `kind` not listed in its own `deviation_ok` … THEN the request is refused as a configuration
/// error and logged; **it does not reach predicate evaluation** and does not open a deviation
/// gate."*
///
/// One case per `kind` (`skip`/`downgrade`/`add`) against `impl-report`, which declares
/// `deviation_ok = ["skip"]`.
///
/// **The "does not reach predicate evaluation" half is discriminating and is measured, not
/// assumed:** each refused case carries a `claimed_predicate` that **cannot be evaluated** —
/// `"stakeholders in agreement"`, AC-065's own named judgement predicate, which this crate's parser
/// rejects. An implementation that evaluated first would return a `Judgement` error rather than a
/// refusal, and this test would go red.
#[test]
fn deviation_kind_outside_deviation_ok_is_refused() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = item();
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");
    let before = base.required_canonical_bytes().expect("canonical");

    let unevaluable = "stakeholders in agreement";

    for (kind, allowed) in [
        (DeviationKind::Skip, true),
        (DeviationKind::Downgrade, false),
        (DeviationKind::Add, false),
    ] {
        let request = RequestDeviation::new("ChangeSummary", kind, unevaluable, "because");
        let outcome = step7(
            &pack,
            &catalog,
            &artifacts,
            &facets,
            "implement",
            &item,
            "impl-report",
            &request,
            &base,
        );

        if allowed {
            // The control: the SAME unevaluable predicate on the ALLOWED kind does reach
            // evaluation and fails there. Without this branch the test is satisfied by a step 7
            // that refuses everything.
            match outcome {
                Err(e) => {
                    let text = e.to_string();
                    assert!(
                        text.contains("stakeholders in agreement"),
                        "the allowed kind must reach predicate evaluation and fail there: {text}"
                    );
                }
                Ok(other) => panic!(
                    "an unevaluable predicate on an ALLOWED kind must fail at evaluation, got \
                     {other:?}"
                ),
            }
            continue;
        }

        let outcome = outcome.expect("a refused kind is a configuration error, not an evaluation");
        let refused = match &outcome {
            DeviationOutcome::Refused(r) => r,
            other => panic!("expected a refusal for {kind:?}, got {other:?}"),
        };
        assert_eq!(refused.activity_id, "impl-report");
        assert_eq!(refused.requested, kind);
        assert_eq!(
            refused.deviation_ok,
            vec![DeviationKind::Skip],
            "the refusal names what the activity DOES permit, so the repair is readable"
        );
        // "does not open a deviation gate" and writes nothing.
        assert!(matches!(outcome, DeviationOutcome::Refused(_)));
        assert!(
            outcome.event().is_none(),
            "ADR-0120 decision 5: the refused branch writes NO `dag_amended`"
        );
    }

    assert_eq!(
        before,
        base.required_canonical_bytes().expect("canonical"),
        "R is untouched by three refusals"
    );
}

/// **AC-064.** *"GIVEN a `RequestDeviation` whose `claimed_predicate` evaluates **true** … THEN
/// steps 2–5 re-run and a `dag_amended { deviation_id, added, removed, rows_refired }` event is
/// written naming every change to `R`."*
///
/// **The producer is the RE-RUN, not step 5 as first fired** (ADR-0120 decision 3), and the event
/// is written **once per accepted deviation** (decision 1).
#[test]
fn ac064_dag_amended_delta_event() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = item();
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");
    let before = base.required_canonical_bytes().expect("canonical");

    assert_eq!(
        kinds(&base),
        vec![
            "ChangeSummary[node]".to_owned(),
            "Changeset[node]".to_owned(),
            "ImplReport".to_owned()
        ],
        "the premise: the conditional row fired, so there is something for a `skip` to remove"
    );

    // `criticality >= high` is TRUE on this item.
    let request = RequestDeviation::new(
        "ChangeSummary",
        DeviationKind::Skip,
        "criticality >= high",
        "the incident is already summarised in the timeline",
    );
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        "impl-report",
        &request,
        &base,
    )
    .expect("an accepted deviation re-runs 2-5");

    let (amended, amended_selection) = match &outcome {
        DeviationOutcome::Accepted { amended, selection } => (amended, selection),
        other => panic!("expected an accepted deviation, got {other:?}"),
    };

    // ---- the four fields are present, and `added`/`removed` match the fixture's delta ----------
    assert_eq!(
        amended
            .removed
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["ChangeSummary[node]".to_owned()]
    );
    assert!(amended.added.is_empty());
    assert_eq!(
        amended.deviation_id,
        request
            .deviation_id()
            .expect("the id is derivable from the request alone"),
        "the id on the event is the content hash of the request, not a second value"
    );
    assert!(
        amended.rows_refired > 0,
        "the re-run re-evaluated the stage's `require` rows"
    );

    // ---- exactly ONE event, of the right kind, with the four payload names ---------------------
    let event = outcome
        .event()
        .expect("the accepted branch writes one event");
    assert_eq!(event.kind, EventKind::DagAmended);
    assert_eq!(
        rk_types::event_kind::EventKind::DagAmended.tier(),
        rk_types::event_kind::Tier::Two,
        "ADR-0120 decision 2: tier 2, because the deviation record is already in the replay input set"
    );
    for field in ["deviation_id", "added", "removed", "rows_refired"] {
        assert!(
            event.payload.get(field).is_some(),
            "`dag_amended` must carry `{field}`: {}",
            event.payload
        );
    }

    // ---- the re-run's `R` is a NEW required set; the base's is untouched -----------------------
    assert_eq!(
        kinds(amended_selection),
        vec!["Changeset[node]".to_owned(), "ImplReport".to_owned()],
        "the re-run produced a different R"
    );
    assert_eq!(
        before,
        base.required_canonical_bytes().expect("canonical"),
        "DEVIATION NEVER EDITS `R` IN PLACE — the base selection is byte-identical after the call"
    );

    // ---- and the re-run went all the way through step 5 ----------------------------------------
    assert!(
        !amended_selection.dag().nodes().is_empty(),
        "steps 2-5 re-ran: the amended selection carries an assembled DAG"
    );
    assert!(
        !amended_selection
            .dag()
            .nodes()
            .iter()
            .any(|n| n.instance.kind == "ChangeSummary"),
        "the skipped kind is not in the amended DAG"
    );
}

/// **`add` and `downgrade` are not dead branches.** `implement-change` declares
/// `deviation_ok = ["add", "downgrade"]`, so the two kinds AC-062 refuses on `impl-report` are
/// exercised here on an activity that permits them — otherwise the refusals above are satisfied by
/// two unimplemented kinds.
#[test]
fn accepted_add_and_downgrade_amend_the_graph() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = item();
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");

    // ---- add ----------------------------------------------------------------------------------
    let add = RequestDeviation::new(
        "APIContract",
        DeviationKind::Add,
        "external_dep",
        "this change crosses a published contract",
    );
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        "implement-change",
        &add,
        &base,
    )
    .expect("an accepted `add` re-runs 2-5");
    let amended = outcome.amended().expect("accepted");
    assert_eq!(
        amended
            .added
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        vec!["APIContract".to_owned()]
    );
    assert!(amended.removed.is_empty());
    assert!(outcome.event().is_some());

    // ---- downgrade ----------------------------------------------------------------------------
    //
    // `downgrade` moves the rigor in force one LIVE level down for the target's producer
    // resolution; `R` itself does not change, which is why `added` and `removed` are both empty and
    // the event still fires. ADR-0120 decision 1 writes one event per **accepted** deviation, not
    // one per changed member.
    let down = RequestDeviation::new(
        "ImplReport",
        DeviationKind::Downgrade,
        "external_dep",
        "a short report is enough for a contract-only change",
    );
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        "implement-change",
        &down,
        &base,
    )
    .expect("an accepted `downgrade` re-runs 2-5");
    let amended = outcome.amended().expect("accepted");
    assert!(amended.added.is_empty() && amended.removed.is_empty());
    assert!(
        outcome.event().is_some(),
        "one event per accepted deviation, even when the delta is empty — otherwise SM-017's use \
         count misses the downgrades"
    );
}

/// **The register's 2026-09-13 amendment to AC-064, clauses 1 and 2.**
///
/// `deviation_id = sha256_lp([target, kind, claimed_predicate])` — the SHA-256 of the
/// **length-prefixed tuple** of exactly those three, in that order, under `rk_store::canon`'s
/// bytes. **`why` is excluded**, and the amendment's stated cost is that *"two deviations that
/// differ **only** in `why` are indistinguishable by id"* — which is asserted here as a property
/// rather than left as prose.
#[test]
fn deviation_id_is_the_content_hash_of_the_request() {
    let base = RequestDeviation::new("ChangeSummary", DeviationKind::Skip, "ui", "reason one");
    let same_but_why =
        RequestDeviation::new("ChangeSummary", DeviationKind::Skip, "ui", "reason two");
    assert_eq!(
        base.deviation_id().expect("derivable"),
        same_but_why.deviation_id().expect("derivable"),
        "clause 2: `why` is EXCLUDED, so two requests differing only in prose are one deviation"
    );

    for other in [
        RequestDeviation::new("ImplReport", DeviationKind::Skip, "ui", "reason one"),
        RequestDeviation::new("ChangeSummary", DeviationKind::Add, "ui", "reason one"),
        RequestDeviation::new("ChangeSummary", DeviationKind::Skip, "not ui", "reason one"),
    ] {
        assert_ne!(
            base.deviation_id().expect("derivable"),
            other.deviation_id().expect("derivable"),
            "each of the three components is inside the identity: {other:?}"
        );
    }

    // **The framing, and a measured correction to how it is asserted.**
    //
    // The amendment states the hazard as *"without it `("ab", "c")` and `("a", "bc")` hash
    // identically"*. The pair below is kept because it is the amendment's own example — but
    // **falsification G13 of 2026-09-13 measured that it does not discriminate**: dropping
    // `sha256_lp` for a plain `sha256` of the concatenation left this assertion green, because the
    // component bytes are `rk_store::canon`'s and the canonical form **quotes every string**, so
    // `"ab""skip""c"` and `"a""skip""bc"` already differ. The quoting frames the components by
    // accident; the amendment's named collision is unreachable under it.
    let ab_c = RequestDeviation::new("ab", DeviationKind::Skip, "c", "");
    let a_bc = RequestDeviation::new("a", DeviationKind::Skip, "bc", "");
    assert_ne!(
        ab_c.deviation_id().expect("derivable"),
        a_bc.deviation_id().expect("derivable"),
        "the amendment's own example — kept, but NOT the framing's discriminator (see above)"
    );

    // So the framing is asserted where it actually bites: the id **is** `sha256_lp` over the three
    // canonical components and **is not** `sha256` of their concatenation, computed here
    // independently of the implementation. This is what goes red when the framing is dropped.
    let canon = |s: &str| {
        rk_store::canon::canonical(&serde_json::Value::from(s.to_owned())).expect("canonical")
    };
    let (t, k, c) = (canon("ChangeSummary"), canon("skip"), canon("ui"));
    let expected = rk_types::digest::sha256_lp(&[&t, &k, &c]);
    assert_eq!(
        base.deviation_id().expect("derivable"),
        expected,
        "deviation_id = sha256_lp([target, kind, claimed_predicate]) under rk_store::canon"
    );
    let mut flat = Vec::new();
    flat.extend_from_slice(&t);
    flat.extend_from_slice(&k);
    flat.extend_from_slice(&c);
    assert_ne!(
        base.deviation_id().expect("derivable"),
        rk_types::digest::sha256(&flat),
        "…and NOT sha256 of the concatenation: the u32-length || bytes framing is what ADR-0097 \
         decision 2 fixes and what AC-064's amendment requires"
    );

    // **Derived, not stored** (clause 3): the id is a pure function of the request, so replaying
    // the recorded `RequestDeviation` recomputes it without the id being an input of its own.
    assert_eq!(
        base.deviation_id().expect("derivable"),
        base.deviation_id().expect("derivable"),
        "a stored id — a counter, a UUID, a timestamp — would need a clock, a random source or the \
         store, all three of which AC-041 forbids at compile time"
    );
}

/// **A `null` `claimed_predicate` is NOT the accepted branch.** AC-064's GIVEN is *"evaluates
/// **true**"* and AC-063's is *"evaluates **false**"*; neither names the abstention, and ADR-0089
/// forbids coercing it to `false`. Step 7 gates it, carrying `Tri::Null`, and writes no event —
/// the reading is stated in `step7`'s own doc comment and asserted here so it cannot drift into
/// "close enough to false".
#[test]
fn an_abstaining_claimed_predicate_gates_and_writes_nothing() {
    let (pack, catalog, artifacts, facets) = fixtures();
    let item = item().with_facet_null("has_repro");
    let base = select(&pack, &catalog, &artifacts, &facets, "implement", &item)
        .expect("selection resolves");

    let request = RequestDeviation::new(
        "ChangeSummary",
        DeviationKind::Skip,
        "has_repro",
        "no evidence either way",
    );
    let outcome = step7(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "implement",
        &item,
        "impl-report",
        &request,
        &base,
    )
    .expect("an abstention is a gate, not an error");

    let gate = match &outcome {
        DeviationOutcome::Gated(g) => g,
        other => panic!("expected a gate on an abstaining predicate, got {other:?}"),
    };
    assert_eq!(gate.value, rk_kernel::predicate::Tri::Null);
    assert!(outcome.event().is_none());
    assert!(outcome.amended().is_none());
}
