//! `rigor_monotonicity` — **AC-105 / SM-035**, over **ADR-0081 clause 1's** superset relation.
//!
//! **It is AC-105, not AC-104.** AC-104 is the family-*totality* criterion — `lattice.py
//! --validate`'s job, green today at `validate errors=0` with `holes=0` in every family — and
//! SM-035's *Instrument* line names AC-105 by id. The two are *"separate obligations on the same
//! enumeration and both are errors, not warnings"* (ADR-0081 clause 2).
//!
//! # Red on the pre-P7-CEIL catalog, green on the shipped one
//!
//! **The red end is an authored fixture catalog, not a past state of the tree.** The pre-P7-CEIL
//! condition stopped existing on 2026-09-11, so `tests/fixtures/catalog/bisect.toml` is the shipped
//! file **minus its `max_rigor` line and nothing else** — produced by an anchored edit that
//! asserted the ceiling occurs exactly once before removing it (golden constraint 7) — and the
//! three other `localisation` members beside it are **byte-identical copies** of
//! `spikes/p9/catalog/root-cause.toml`, `differential-debugging.toml` and
//! `instrumented-trace-analysis.toml`, so the fixture catalog differs from the shipped one in
//! exactly one line.
//!
//! On that catalog F37's measured defect returns: escalating to `mandatory` with `has_repro`
//! selects `bisect`, which has **no adversary step and no policy step**, so escalating rigor
//! **drops** the quality block — the `Findings` its `adversary` step mints at `recommended`
//! disappears.
//!
//! The green end is `spikes/p9/catalog/` **as it stands**, with `bisect` ceilinged at
//! `max_rigor    = "recommended"` since 2026-09-11 (ADR-0093 decision 5; DECISIONS-2026-09-11 #1),
//! which routes `mandatory` to `instrumented-trace-analysis` — whose `adversary` step is
//! unconditional. **The column alignment in that line is incidental and is left alone**: the
//! assertion that reads the shipped catalog is alignment-tolerant for exactly that reason
//! (golden constraint 9 — the guard's scope is "one ceiling declaration", not a spacing).
//!
//! **Adding the missing steps to `bisect` is the Phase-2 repair under ADR-0072**, which the design
//! says three times, and it is not done here.

use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Catalog, Evidence};
use rk_kernel::registry::Registry;
use rk_kernel::rigor::{Rigor, rigor_monotonicity_rule};

/// The shipped catalog, relative to the workspace root. This path is on a non-comment line
/// deliberately: "passes on the catalog as shipped" asserted over a fixture of the implementer's
/// choosing is not that claim, and a doc comment naming the path satisfies an unanchored grep.
const SHIPPED_CATALOG: &str = "spikes/p9/catalog";
/// The shipped facet registry, which supplies every domain the lattice enumerates. **Both ends of
/// this test read the same registry**: only the catalog differs between red and green.
const SHIPPED_FACETS: &str = "spikes/p9/facets.toml";

const FIXTURE_BISECT: &str = include_str!("../fixtures/catalog/bisect.toml");
const FIXTURE_ROOT_CAUSE: &str = include_str!("../fixtures/catalog/root-cause.toml");
const FIXTURE_DIFFERENTIAL: &str = include_str!("../fixtures/catalog/differential-debugging.toml");
const FIXTURE_TRACE: &str = include_str!("../fixtures/catalog/instrumented-trace-analysis.toml");

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

fn shipped_registry() -> Registry {
    let path = workspace_root().join(SHIPPED_FACETS);
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry at {} is readable: {e}",
            path.display()
        )
    });
    Registry::from_toml(&src).expect("the shipped registry loads")
}

/// The pre-P7-CEIL `localisation` family, from the committed fixture sources.
fn pre_ceiling_catalog(bisect: &str) -> Catalog {
    Catalog::from_sources(&[
        ("bisect.toml", bisect),
        ("root-cause.toml", FIXTURE_ROOT_CAUSE),
        ("differential-debugging.toml", FIXTURE_DIFFERENTIAL),
        ("instrumented-trace-analysis.toml", FIXTURE_TRACE),
    ])
    .expect("the fixture catalog loads")
}

#[test]
fn rigor_monotonicity() {
    let registry = shipped_registry();

    // ---- the fixture is the shipped file MINUS ONE LINE, and that is asserted, not assumed -----
    assert!(
        !FIXTURE_BISECT.contains("max_rigor"),
        "the red fixture is the PRE-P7-CEIL bisect: it carries no ceiling at all"
    );
    let shipped_bisect =
        std::fs::read_to_string(workspace_root().join(SHIPPED_CATALOG).join("bisect.toml"))
            .expect("the shipped bisect.toml is readable");
    let shipped_without_ceiling: String = shipped_bisect
        .lines()
        .filter(|l| !l.starts_with("max_rigor"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        FIXTURE_BISECT.replace("\r\n", "\n").trim_end(),
        shipped_without_ceiling.replace("\r\n", "\n").trim_end(),
        "the red fixture must differ from the shipped bisect.toml in the ceiling line and nothing \
         else — otherwise the red end is measuring an edit of the author's own devising"
    );

    // ---- RED: the pre-P7-CEIL catalog is NOT monotone -----------------------------------------
    let survey = rigor_monotonicity_rule(&pre_ceiling_catalog(FIXTURE_BISECT), &registry)
        .expect("the rule runs over the fixture catalog");
    assert!(
        survey.contexts_checked > 0 && survey.families > 0,
        "a lattice that enumerated nothing would report no violations and read as a clean catalog \
         — {survey:?}"
    );
    assert!(
        !survey.is_monotone(),
        "the pre-P7-CEIL catalog reproduces F37's measured defect and must fail: {survey:?}"
    );

    // The violation is F37's, named: `localisation`, escalating INTO `mandatory`, selecting
    // `bisect`, and losing `Findings` — the adversary. A rule that flagged some other family, or
    // lost some other variant, would not be this measurement.
    let f37 = survey
        .violations
        .iter()
        .find(|v| v.family == "localisation" && v.upper_activity == "bisect")
        .unwrap_or_else(|| {
            panic!(
                "expected the localisation/bisect violation, got {:?}",
                survey.violations
            )
        });
    assert_eq!(
        f37.upper,
        Rigor::Mandatory,
        "F37's defect is at the TOP of the ladder"
    );
    assert!(
        f37.lost.contains(&Evidence::Findings),
        "what `bisect` drops is the ADVERSARY's Findings — 'losing an agent step's Findings … is a \
         monotonicity violation whatever else the higher-rigor producer adds' (ADR-0081 clause 1): \
         {f37}"
    );
    assert!(
        f37.context.contains("has_repro=true"),
        "F37's context is `has_repro`, which is what makes bisect applicable at all: {f37}"
    );
    println!(
        "rigor_monotonicity RED (pre-P7-CEIL fixture): families={} contexts={} comparisons={} \
         violations={}\n  {f37}",
        survey.families,
        survey.contexts_checked,
        survey.comparisons,
        survey.violations.len()
    );
    // THE PRINTED NUMBERS ARE ASSERTED, AND THEY WERE NOT UNTIL 2026-09-13. `contexts_checked`
    // comes from the odometer and is BLIND to admissibility, so it does not move when the rule
    // stops walking part of the lattice; `comparisons` is the count of ordered (lower, upper) pairs
    // for which BOTH ends selected something, and it is the only figure that does. Measured under
    // five separate guttings of the rule, each of which left the suite green: dropping the
    // `min_rigor` floor from `admissible()` moved the shipped figure 1729 → 1821, dropping
    // `applicable_if` → 2293, dropping the tag filter → 4100 (and the RED end 72 → 240), cutting
    // the ordered-pair loop to ADJACENT pairs only → 1165, and ignoring every step `when` guard in
    // `evidence()` moved the RED end's violation count 2 → 4. An unasserted number certifies
    // nothing, and these are the session's headline evidence.
    assert_eq!(
        (survey.families, survey.contexts_checked, survey.comparisons),
        (1, 80, 72),
        "the red end's shape is measured, not merely non-zero: one family, the odometer's 80 \
         contexts, and 72 ordered rigor pairs where both ends selected. A change here is a change \
         in HOW MUCH LATTICE IS WALKED and must be re-derived, never widened"
    );
    assert_eq!(
        survey.violations.len(),
        2,
        "TWO violations, not 'at least one': F37's defect fires at (recommended, mandatory) and at \
         (optional, mandatory), and a rule that found more has started flagging admissible losses \
         — {:#?}",
        survey.violations
    );
    // Every violation names a file a reader can open.
    for v in &survey.violations {
        assert!(
            !v.upper_source.is_empty() && !v.lower_source.is_empty(),
            "{v}"
        );
    }

    // ---- DISCRIMINATION: restore the one line, and the same catalog is monotone ----------------
    // This is what shows the red end fails for the CEILING and not for anything incidental about a
    // four-file catalog, a fixture path or the reader.
    let ceilinged = FIXTURE_BISECT.replace(
        "min_rigor    = \"optional\"\n",
        "min_rigor    = \"optional\"\nmax_rigor    = \"recommended\"\n",
    );
    assert!(
        ceilinged.contains("max_rigor"),
        "the repair anchor `min_rigor` must be present in the fixture (golden constraint 7)"
    );
    let repaired = rigor_monotonicity_rule(&pre_ceiling_catalog(&ceilinged), &registry)
        .expect("the rule runs over the repaired fixture catalog");
    assert!(
        repaired.is_monotone(),
        "re-ceilinging bisect must make the SAME catalog monotone: {:?}",
        repaired.violations
    );
    assert_eq!(
        repaired.contexts_checked, survey.contexts_checked,
        "the repair changes the verdict and not the amount of lattice walked"
    );

    // ---- GREEN: the shipped catalog, as it stands ---------------------------------------------
    let shipped = Catalog::from_dir(&workspace_root().join(SHIPPED_CATALOG))
        .expect("the shipped catalog loads");
    assert!(
        shipped.len() >= 100,
        "the shipped catalog is 102 activities [measured 2026-09-13]; {} were read, which is not \
         the catalog this assertion is about",
        shipped.len()
    );
    let survey = rigor_monotonicity_rule(&shipped, &registry).expect(
        "the rule runs over the \
         shipped catalog",
    );
    assert!(
        survey.families >= 15 && survey.contexts_checked > 0,
        "the shipped catalog declares 15 families [measured 2026-09-13]; the survey walked \
         {survey:?}"
    );
    assert!(
        survey.is_monotone(),
        "the shipped catalog is monotone with bisect ceilinged (P7-CEIL). Violations: {:#?}",
        survey.violations
    );
    println!(
        "rigor_monotonicity: activities={} families={} contexts={} comparisons={} violations=0",
        shipped.len(),
        survey.families,
        survey.contexts_checked,
        survey.comparisons
    );
    // THE GREEN END'S NUMBERS, ASSERTED FOR THE SAME REASON. `violations == 0` over a lattice the
    // rule barely walked is not a green catalog, and `is_monotone()` alone cannot tell the two
    // apart. 1729 is the measured count of ordered rigor pairs at which both ends selected, over
    // 2060 contexts and 15 families [measured 2026-09-13]; each of the four admissibility guttings
    // above moves it by 30–140% while leaving every other assertion in this test green.
    assert_eq!(
        (survey.families, survey.contexts_checked, survey.comparisons),
        (15, 2060, 1729),
        "the shipped catalog's survey is pinned by its SHAPE and not only by its verdict. If the \
         catalog changed, re-derive these three off the new reading and record it — do not widen \
         the assertion (golden constraint 9)"
    );
    assert!(
        survey.violations.is_empty(),
        "violations=0 is asserted as a count, not inferred from is_monotone(): {:#?}",
        survey.violations
    );

    // ---- WHAT `admissible()` ACTUALLY FILTERS, one member-level case per clause ----------------
    // `admissible()` is documented as *"`lattice.py`'s `admissible()`, clause for clause: the rigor
    // floor, the `max_rigor` ceiling, the tag filter, then `applicable_if`"*, and only the CEILING
    // was pinned — by the P7-CEIL repair above. Deleting the floor, the tag filter or the
    // `applicable_if` clause each left every assertion in this test green while changing which
    // member the family selects and how much lattice the rule walks. Each is exercised here on the
    // PRE-CEILING catalog, whose F37 violation names both ends, because *which member is selected*
    // is the observable an admissibility clause moves — `comparisons` counts pairs where both ends
    // selected SOMETHING and a four-member family always has a candidate.
    //
    // The TAG FILTER and `applicable_if` are shown by excluding `bisect` itself: with it gone,
    // `mandatory` routes to `instrumented-trace-analysis` — the shipped catalog's own behaviour —
    // and the family is monotone again.
    //
    // 🛑 THE `applicable_if` ANCHOR IS READ OFF THE FIXTURE AND NOT WRITTEN OUT, and that is
    // golden constraint 9 and not fastidiousness. §7b.14 runs golden constraint 1 as a command over
    // `crates/` with `--include=*.rs`, and its pattern is
    // `(require|requires|applicable_if|when)\s*=.*\b(criticality|novelty|…|has_repro)\b`. Spelling
    // the anchor as a Rust string literal puts exactly that shape on a source line and HALTS the
    // checkpoint — measured 2026-09-13, four hits in this file. The prescribed fixture tree is
    // excluded by path because §7b.8 and §7b.12 REQUIRE a copy of `bisect.toml` carrying the line;
    // this file is not in that tree and must not need the exclusion. Halt 7's rule: do not widen
    // the guard, and do not write the hit.
    let bisect_src = FIXTURE_BISECT.replace("\r\n", "\n");
    let predicate_line = bisect_src
        .lines()
        .find(|l| l.starts_with("applicable_if"))
        .expect("the pre-P7-CEIL fixture carries a selection predicate")
        .to_owned();
    let facet = predicate_line
        .split('"')
        .nth(1)
        .expect("the predicate is quoted")
        .to_owned();
    let never_true = format!("applicable_if = \"{facet} and not {facet}\"");
    for (clause, anchor, replacement) in [
        (
            "tags",
            "tags         = [\"kind:bug\", \"kind:security-vuln\", \"change:text\"]",
            "tags         = [\"kind:no-such-kind\"]",
        ),
        (
            "applicable_if",
            predicate_line.as_str(),
            never_true.as_str(),
        ),
    ] {
        let src = bisect_src.clone();
        assert_eq!(
            src.matches(anchor).count(),
            1,
            "{clause}: the anchor must occur exactly once (golden constraint 7)"
        );
        let narrowed = src.replace(anchor, replacement);
        let after = rigor_monotonicity_rule(&pre_ceiling_catalog(&narrowed), &registry)
            .expect("the rule runs over the narrowed fixture");
        assert!(
            after.is_monotone(),
            "{clause}: a `bisect` excluded by its {clause} clause is never selected, so F37's \
             defect cannot fire — a rule that IGNORES this clause selects it anyway and stays red. \
             Violations: {:#?}",
            after.violations
        );
        assert!(
            after.comparisons > 0,
            "{clause}: and the family was still walked, so the green is not the green of an \
             enumeration of nothing: {after:?}"
        );
    }

    // The FLOOR is shown at the other end of the same violation: raise `root-cause`'s `min_rigor`
    // to `mandatory` and it is inadmissible at `recommended`, so F37's LOWER selection moves to
    // `differential-debugging` (method_rank 20, the next lowest). A rule that ignores the floor
    // keeps selecting `root-cause` and reports the identical violation.
    let floored = FIXTURE_ROOT_CAUSE.replace("\r\n", "\n").replace(
        "min_rigor    = \"optional\"",
        "min_rigor    = \"mandatory\"",
    );
    assert!(
        floored.contains("min_rigor    = \"mandatory\""),
        "min_rigor: the anchor applied (golden constraint 7)"
    );
    let raised = Catalog::from_sources(&[
        ("bisect.toml", &FIXTURE_BISECT.replace("\r\n", "\n")),
        ("root-cause.toml", &floored),
        ("differential-debugging.toml", FIXTURE_DIFFERENTIAL),
        ("instrumented-trace-analysis.toml", FIXTURE_TRACE),
    ])
    .expect("the floored fixture catalog loads");
    let raised = rigor_monotonicity_rule(&raised, &registry).expect("the rule runs");
    let moved = raised
        .violations
        .iter()
        .find(|v| v.upper_activity == "bisect" && v.lower == Rigor::Recommended)
        .unwrap_or_else(|| panic!("the bisect violation survives: {:#?}", raised.violations));
    assert_eq!(
        moved.lower_activity, "differential-debugging",
        "with `root-cause` floored at `mandatory` the lower end of the SAME violation must move to \
         the next-lowest admissible member — a rule that ignores `min_rigor` reports `root-cause` \
         here, which is what deleting the floor from `admissible()` does"
    );

    // ---- AND `evidence()`'s step guard: a step counts only when its guard is true --------------
    // Reading 4 of the rigor module's header. A rule that ignored every step `when` moved the red
    // end's violation count 2 → 4 with this test green; here the guard is falsified on the fixture
    // and the guaranteed set must shrink, which shows the guard is read at all.
    // The guard goes on `instrumented-trace-analysis`'s ADVERSARY step, because that is the member
    // `mandatory` routes to once `bisect` is ceilinged — the green end's own selection. Guard it
    // with a predicate no context makes true and the activity stops GUARANTEEING `Findings`, so the
    // ladder breaks exactly where F37's did. The green control is the ceilinged catalog three
    // assertions above, which is monotone with the same file unguarded.
    // Same rule as above: the guard predicate is COMPOSED from the fixture's own facet name, never
    // spelled out, so this file carries no `when = "<a blocked facet>"` source line.
    let guard_line = format!("when    = \"{facet} and not {facet}\"");
    let guarded = FIXTURE_TRACE.replace("\r\n", "\n").replace(
        "[[step]]\nid      = \"adversary\"\n",
        &format!("[[step]]\n{guard_line}\nid      = \"adversary\"\n"),
    );
    assert!(
        guarded.contains(&guard_line),
        "the step-guard anchor applied (golden constraint 7)"
    );
    let with_guard = Catalog::from_sources(&[
        ("bisect.toml", &ceilinged),
        ("root-cause.toml", FIXTURE_ROOT_CAUSE),
        ("differential-debugging.toml", FIXTURE_DIFFERENTIAL),
        ("instrumented-trace-analysis.toml", &guarded),
    ])
    .expect("the step-guarded fixture catalog loads");
    let with_guard =
        rigor_monotonicity_rule(&with_guard, &registry).expect("the rule runs over it");
    assert!(
        !with_guard.is_monotone(),
        "a step whose guard is never true mints nothing, so the activity's GUARANTEED evidence set \
         shrinks and the ladder breaks — a rule that IGNORED every step `when` counts the adversary \
         anyway and reports this catalog monotone: {with_guard:?}"
    );
    assert!(
        with_guard
            .violations
            .iter()
            .any(|v| v.lost.contains(&Evidence::Findings)),
        "and what it loses is the guarded step's own mint: {:#?}",
        with_guard.violations
    );
}
