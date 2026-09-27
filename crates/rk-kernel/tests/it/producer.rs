//! **Step 4 — producer** (design §4.4 row 4), and the two negative criteria that hold it in place.
//!
//! Candidates are the activities with the kind in `provides`, `applicable_if` true, tags matching,
//! `min_rigor ≤ r ≤ max_rigor`, and `stage` equal to the requiring stage. **Lowest `method_rank`
//! wins.** An equal-rank pair simultaneously admissible in any enumerated context is a **pack
//! validation error at LOAD** — `RankTie { kind, context, activities }` — and there is **no runtime
//! tie-break, of any kind, for any artifact kind** (ADR-0056:71,:81; ADR-0073:24 withdraws
//! specificity for producer-of-a-kind selection; §7b.9(1)).

use rk_kernel::rigor::Rigor;
use rk_kernel::select::{
    ArtifactRegistry, Catalog, Instance, Item, LoadError, Unsatisfiable, rank_lint, step4,
};

const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

const CHEAP: (&str, &str) = (
    "verdict-cheap.toml",
    include_str!("../fixtures/catalog-rank/verdict-cheap.toml"),
);
const DEAR: (&str, &str) = (
    "verdict-dear.toml",
    include_str!("../fixtures/catalog-rank/verdict-dear.toml"),
);
const TIE: (&str, &str) = (
    "verdict-tie.toml",
    include_str!("../fixtures/catalog-rank/verdict-tie.toml"),
);
const NEIGHBOUR: (&str, &str) = (
    "verdict-neighbour.toml",
    include_str!("../fixtures/catalog-rank/verdict-neighbour.toml"),
);
const CEILING: (&str, &str) = (
    "verdict-ceiling.toml",
    include_str!("../fixtures/catalog-rank/verdict-ceiling.toml"),
);
const INAPPLICABLE: (&str, &str) = (
    "verdict-inapplicable.toml",
    include_str!("../fixtures/catalog-rank/verdict-inapplicable.toml"),
);

fn registry() -> ArtifactRegistry {
    ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS).expect("the fixture registry loads")
}

fn facets() -> rk_kernel::registry::Registry {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("spikes/p9/facets.toml");
    let src = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry at {} is readable: {e}",
            path.display()
        )
    });
    rk_kernel::registry::Registry::from_toml(&src).expect("the shipped facet registry loads")
}

fn catalog(sources: &[(&str, &str)]) -> Catalog {
    Catalog::from_sources(sources).expect("the fixture catalog loads")
}

fn story() -> Item {
    Item::new("STORY-1", "story").with_toolchain("node", ["text"])
}

/// **AC-046 and AC-047.** Lowest rank wins, the selection record names the ranks it beat, and a tie
/// is a **load** failure and never a runtime choice.
#[test]
fn rank_total_order_and_tie_is_an_error() {
    let reg = registry();
    let facets = facets();

    // ---- lowest rank wins, and the record names the losers -------------------------------------
    let cat = catalog(&[CHEAP, DEAR]);
    let picked = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("a producer resolves");
    let chosen = picked.chosen().expect("one producer was chosen");
    assert_eq!(chosen.activity_id, "verdict-cheap");
    assert_eq!(chosen.method_rank, 5);
    let beaten: Vec<(&str, i64)> = picked
        .beaten()
        .iter()
        .map(|c| (c.activity_id.as_str(), c.method_rank))
        .collect();
    assert_eq!(
        beaten,
        vec![("verdict-dear", 20)],
        "AC-046: the selection record names the rank of every candidate it beat"
    );

    // ---- a tie is a LOAD error, before any activity is dispatched -------------------------------
    let tied = catalog(&[CHEAP, DEAR, TIE]);
    let err = rank_lint(&tied, &reg, &facets).expect_err("an equal-rank pair must fail load");
    match &err {
        LoadError::RankTie {
            kind,
            context,
            activities,
        } => {
            assert_eq!(kind, "ReviewVerdict");
            assert_eq!(
                activities,
                &vec!["verdict-cheap".to_owned(), "verdict-tie".to_owned()],
                "both tied activities are named, in byte order"
            );
            assert!(
                !context.is_empty(),
                "the context the tie was found in is part of the error (AC-047's payload)"
            );
        }
        other => panic!("expected RankTie, got {other:?}"),
    }

    // ---- and the shipped-shape catalog does NOT tie ---------------------------------------------
    // Without this the lint is satisfied by one that fails on everything.
    rank_lint(&catalog(&[CHEAP, DEAR]), &reg, &facets)
        .expect("a catalog with no equal-rank pair must load");

    // ---- there is no runtime tie-break to reach --------------------------------------------------
    // The tie never arrives at `step4` because `rank_lint` refuses the catalog at load. If one were
    // somehow constructed, step 4 refuses it as an internal fault rather than ordering the pair by
    // file order, load order or name.
    let err = step4(
        &tied,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect_err("step 4 must not silently pick one of a tied pair");
    assert!(
        matches!(err, LoadError::RankTie { .. }),
        "the runtime path re-raises the load error rather than tie-breaking: {err:?}"
    );
}

/// **AC-052, and AC-048's ceiling half (PIA-054).** Only activities whose `stage` equals the
/// requiring stage and whose `min_rigor ≤ r ≤ max_rigor` are candidates — "whatever its rank".
#[test]
fn rigor_stage_scope_and_ceiling() {
    let reg = registry();
    let facets = facets();

    // A rank-1 producer in a NEIGHBOURING stage must not be chosen. Candidacy does not cross stage
    // boundaries (ADR-0057's Decision).
    let cat = catalog(&[CHEAP, NEIGHBOUR]);
    let picked = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("resolves");
    assert_eq!(
        picked.chosen().expect("chosen").activity_id,
        "verdict-cheap"
    );
    assert!(
        !picked
            .beaten()
            .iter()
            .any(|c| c.activity_id == "verdict-neighbour"),
        "an out-of-stage activity is not a CANDIDATE, so it is not even among the beaten"
    );

    // A rank-1 producer in the right stage but ABOVE ITS CEILING is not a candidate either.
    let cat = catalog(&[CHEAP, CEILING]);
    let above = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("resolves");
    assert_eq!(above.chosen().expect("chosen").activity_id, "verdict-cheap");

    // …and AT its ceiling it is, which is what stops the ceiling reading as "never admissible".
    let at = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Optional,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("resolves");
    assert_eq!(
        at.chosen().expect("chosen").activity_id,
        "verdict-ceiling",
        "at rigor `optional` the ceilinged rank-1 producer IS admissible and wins"
    );

    // An ABSENT ceiling is no upper bound (ADR-0056), which `verdict-cheap` demonstrates by being
    // admissible at `mandatory`.
    let cat = catalog(&[CHEAP]);
    let top = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Mandatory,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("resolves");
    assert_eq!(top.chosen().expect("chosen").activity_id, "verdict-cheap");
}

/// **AC-054 and AC-055's type half.** No admissible producer ⇒ `Unsatisfiable { kind,
/// failed_predicates }` and a gate — and **no fallback**: no next-best rank, no relaxation of
/// `min_rigor`, no substitute kind, no dropping the artifact from `R`.
///
/// §7b.9(4): the operands are **typed into the variant**, not re-derived at gate-write time. AC-055
/// requires *"the evaluated value of each facet that predicate read"*, and re-evaluating a
/// three-valued grammar at write time over a context that may have been re-expanded turns a
/// disagreement into a runtime write failure rather than a test failure.
#[test]
fn unsatisfiable_is_a_typed_gate() {
    let reg = registry();
    let facets = facets();
    let cat = catalog(&[CHEAP, INAPPLICABLE]);

    // `market_facing` is computed and FALSE, so the only SmokeResult producer's `applicable_if` is
    // false and there is no candidate.
    let item = story().with_facet_bool("market_facing", false);
    let picked = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("SmokeResult"),
        &item,
    )
    .expect("step 4 returns Ok with the kind GATED -- unsatisfiability is DATA, not an abort");

    assert!(picked.chosen().is_none());
    let un: &Unsatisfiable = picked
        .unsatisfiable()
        .expect("the kind is gated as Unsatisfiable");
    assert_eq!(un.kind, "SmokeResult");
    assert_eq!(un.failed_predicates.len(), 1);
    let failed = &un.failed_predicates[0];
    assert_eq!(failed.activity_id, "smoke-on-market-facing");
    assert_eq!(failed.text, "market_facing");
    assert_eq!(
        failed.value,
        rk_kernel::predicate::Tri::False,
        "Unsatisfiable is reserved for predicates that evaluated to FALSE (AC-079's third AND)"
    );
    assert_eq!(
        failed.operands,
        vec![(
            "market_facing".to_owned(),
            rk_kernel::select::FacetValue::Bool(false)
        )],
        "the evaluated value of each facet the predicate read is TYPED INTO the variant"
    );
    assert_eq!(un.rigor, Rigor::Recommended, "and the rigor in force");

    // NO FALLBACK. `verdict-cheap` is admissible, rank 5, in the same stage — and it provides a
    // DIFFERENT kind, so it must not be substituted.
    assert!(
        picked.beaten().is_empty(),
        "there were no candidates to beat: {:?}",
        picked.beaten()
    );

    // A `null` read is NOT Unsatisfiable: the order is lint at load (AC-069), gate at step 2
    // (AC-079), `Unsatisfiable` at step 4 (AC-054).
    let abstaining = story().with_facet_null("market_facing");
    let picked = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("SmokeResult"),
        &abstaining,
    )
    .expect("resolves");
    assert!(
        picked.unsatisfiable().is_none(),
        "a producer whose applicable_if is NULL abstains; it does not make the kind Unsatisfiable"
    );
    assert_eq!(
        picked.abstained().len(),
        1,
        "it is recorded as an abstention, which is AC-079's route and not AC-054's"
    );
}

// ---- D21-73 item 1 (R8 = ADR-0179, proposed; `s8d` build, package F2-D2173) -----------------------
//
// `rank_total_order_and_tie_is_an_error` could not tell the rank rule from the byte order of the ids:
// its winner, `verdict-cheap`, sorts before its loser, `verdict-dear`, so a step 4 that took the
// first id ALPHABETICALLY passed it. That is what shipped: `Candidate` derived `Ord` over
// `(activity_id, method_rank)` in field order and step 4 called `candidates.sort()`, so the fix lane
// chose `bisect` (rank 30) for `RootCause` over `root-cause` (rank 10). The two fixtures below put
// the LOWER rank on the id that sorts LAST, so only the rank rule passes them.

/// One `review` activity providing `ReviewVerdict` at `rank`, for an inline fixture catalog.
fn ranked(id: &str, rank: i64) -> String {
    format!(
        "[activity]\nid = \"{id}\"\nversion = 1\nstage = \"review\"\nprovides = [\"ReviewVerdict\"]\n\
         requires = []\nderived_from = []\ntags = [\"kind:story\"]\nmethod_rank = {rank}\n\
         min_rigor = \"optional\"\ncost_class = \"S\"\n[[step]]\nid = \"emit\"\nkind = \"emit\"\n"
    )
}

/// **AC-046, over ids whose byte order is the reverse of their ranks.** `a-dear` (rank 30) sorts
/// first and `z-cheap` (rank 5) last; `m-mid` (rank 20) between. The rank rule picks `z-cheap`, and
/// the beaten list is in rank order, lowest first.
#[test]
fn step4_picks_the_lowest_rank_not_the_first_id() {
    let reg = registry();
    let facets = facets();
    let (a, m, z) = (
        ranked("a-dear", 30),
        ranked("m-mid", 20),
        ranked("z-cheap", 5),
    );
    let cat = catalog(&[("a.toml", &a), ("m.toml", &m), ("z.toml", &z)]);
    let picked = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect("a producer resolves");
    let chosen = picked.chosen().expect("one producer was chosen");
    assert_eq!(
        (chosen.activity_id.as_str(), chosen.method_rank),
        ("z-cheap", 5),
        "AC-046: the LOWEST method_rank wins, whatever its id sorts to"
    );
    let beaten: Vec<(&str, i64)> = picked
        .beaten()
        .iter()
        .map(|c| (c.activity_id.as_str(), c.method_rank))
        .collect();
    assert_eq!(
        beaten,
        vec![("m-mid", 20), ("a-dear", 30)],
        "the beaten candidates are named with their ranks, in rank order"
    );
}

/// **AC-047 at runtime, over the same reversed order.** Two producers tie at the LOWEST rank (5)
/// while a higher-ranked one (`a-dear`, 30) sorts before both. Step 4 refuses with `RankTie` naming
/// the tied pair, and does not pick `a-dear` for sorting first: there is no tie-break (ADR-0056).
#[test]
fn a_tie_at_the_lowest_rank_is_refused_whatever_the_ids_sort_to() {
    let reg = registry();
    let facets = facets();
    let (a, y, z) = (
        ranked("a-dear", 30),
        ranked("y-cheap", 5),
        ranked("z-cheap", 5),
    );
    let cat = catalog(&[("a.toml", &a), ("y.toml", &y), ("z.toml", &z)]);
    let err = step4(
        &cat,
        &reg,
        &facets,
        "review",
        Rigor::Recommended,
        &Instance::of("ReviewVerdict"),
        &story(),
    )
    .expect_err("a tie at the lowest rank must refuse, never resolve");
    match err {
        LoadError::RankTie { activities, .. } => assert_eq!(
            activities,
            vec!["y-cheap".to_owned(), "z-cheap".to_owned()],
            "the tied pair at the LOWEST rank is named, in byte order"
        ),
        other => panic!("expected RankTie over the rank-5 pair, got {other:?}"),
    }
}
