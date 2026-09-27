//! **The forward-ref licence** (`s8d` §8d.3; D21-48 X8; D21-60; R2 = ADR-0173, written `proposed`).
//!
//! ADR-0042's Decision: *"A ref to a later-declared step is legal iff that step or one after it has
//! `on_false`/`repeat` targeting the referencing step; it is `null` on the first pass."* D21-48 X8
//! widened it: *"an escalate gate whose re-entry resumes into the referencing step also licenses the
//! ref. The lint widens to match. Any ref still unlicensed is a lint error, fixed in the catalog in
//! s8d."* So a ref from step R to a later-declared step is admitted at load under one of three
//! licences — the letter, the repeat body (D21-48 X6) and the escalate re-entry — and only with `?`;
//! every other ref is refused by name, giving the activity, the step and the reference.
//!
//! | test | reads | red first because |
//! | --- | --- | --- |
//! | [`a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name`] | `tests/fixtures/forward_refs/`, through `select::load_lints` | the load had no licence check, so the five must-refuse fixtures loaded |
//! | [`every_shipped_forward_ref_is_licensed_and_the_census_is_printed`] | `.redkiln/catalog` | before the catalog edit, `unlicensed` was 4 |
//! | [`the_four_unlicensed_refs_are_removed_versioned_and_changelogged`] | `.redkiln/catalog`, `.redkiln/CHANGELOG.md` | the four inputs were present, the versions had not moved, and no row existed |
//!
//! The census is read from INSIDE this binary, because a Python census is a proxy (Checkpoint 8d (6b)).

use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Licence, forward_ref_census, forward_refs};
use rk_kernel::registry::Registry;
use rk_kernel::select::{ArtifactRegistry, Catalog, load_lints};
use rk_kernel::toml_subset;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

fn shipped_registries() -> (ArtifactRegistry, Registry) {
    let pack = workspace_root().join(".redkiln");
    let artifacts_src =
        std::fs::read_to_string(pack.join("artifacts.toml")).expect("artifacts.toml is readable");
    let facets_src =
        std::fs::read_to_string(pack.join("facets.toml")).expect("facets.toml is readable");
    (
        ArtifactRegistry::from_toml("artifacts.toml", &artifacts_src)
            .expect("the shipped artifact registry loads"),
        Registry::from_toml(&facets_src).expect("the shipped facet registry loads"),
    )
}

const LETTER: &str = include_str!("../fixtures/forward_refs/licensed_by_the_letter.toml");
const REPEAT_BODY: &str = include_str!("../fixtures/forward_refs/licensed_by_the_repeat_body.toml");
const ESCALATE: &str =
    include_str!("../fixtures/forward_refs/licensed_by_the_escalate_re_entry.toml");
const APPROVAL_AFTER_LOOP: &str =
    include_str!("../fixtures/forward_refs/refused_approval_answer_after_the_only_loop.toml");
const OTHER_QUESTION: &str =
    include_str!("../fixtures/forward_refs/refused_escalate_to_another_question.toml");
const PAST_BODY: &str =
    include_str!("../fixtures/forward_refs/refused_past_the_end_of_the_repeat_body.toml");
const D59B: &str = include_str!("../fixtures/forward_refs/refused_back_ref_no_loop_targets.toml");
const MANDATORY: &str = include_str!("../fixtures/forward_refs/refused_mandatory_forward_ref.toml");

/// **The three licences load, and every other forward ref is refused at load by name.** Each fixture
/// is its own one-activity catalog, loaded and then run through `select::load_lints` — the refusing
/// load rules every shipped path into selection runs — beside the shipped registries. Each
/// must-refuse fixture must be refused naming its activity, its step and the reference; each
/// must-load fixture must load with its one forward ref classed under the licence it exercises.
#[test]
fn a_forward_ref_is_licensed_by_a_back_edge_a_repeat_body_or_an_escalate_re_entry_and_otherwise_refused_by_name()
 {
    let (artifacts, facets) = shipped_registries();
    let load = |name: &str, src: &str| {
        let cat = Catalog::from_sources(&[(name, src)])
            .unwrap_or_else(|e| panic!("{name} parses as a catalog: {e}"));
        let verdict = load_lints(&cat, &artifacts, &facets);
        (cat, verdict)
    };

    // ---- the three licences LOAD, each classed as the licence it exercises ----------------------
    let must_load: [(&str, &str, Licence, usize); 3] = [
        ("licensed_by_the_letter.toml", LETTER, Licence::Letter, 1),
        (
            "licensed_by_the_repeat_body.toml",
            REPEAT_BODY,
            Licence::RepeatBody,
            1,
        ),
        (
            "licensed_by_the_escalate_re_entry.toml",
            ESCALATE,
            Licence::EscalateReEntry,
            2,
        ),
    ];
    for (name, src, licence, n) in must_load {
        let (cat, verdict) = load(name, src);
        assert!(
            verdict.is_ok(),
            "{name}: a forward ref under {licence:?} must LOAD: {verdict:?}"
        );
        let refs = forward_refs(&cat.activities()[0]);
        assert_eq!(
            refs.len(),
            n,
            "premise: {name} carries {n} forward ref(s): {refs:?}"
        );
        for r in &refs {
            assert_eq!(
                (r.optional, r.licence),
                (true, Some(licence)),
                "{name}: {r:?} is classed under the licence the fixture exercises"
            );
        }
    }

    // ---- every other forward ref is REFUSED, by activity, step and reference -------------------
    let must_refuse: [(&str, &str, &str, &str, &str); 5] = [
        (
            "refused_approval_answer_after_the_only_loop.toml",
            APPROVAL_AFTER_LOOP,
            "fr-approval-after-loop",
            "draft",
            "${accept.answer?}",
        ),
        (
            "refused_escalate_to_another_question.toml",
            OTHER_QUESTION,
            "fr-other-question",
            "attempt",
            "${approve.answer?}",
        ),
        (
            "refused_past_the_end_of_the_repeat_body.toml",
            PAST_BODY,
            "fr-past-body",
            "green",
            "${refactor.outputs?}",
        ),
        (
            "refused_back_ref_no_loop_targets.toml",
            D59B,
            "fr-d59b",
            "model",
            "${review.outputs?}",
        ),
        (
            "refused_mandatory_forward_ref.toml",
            MANDATORY,
            "fr-mandatory",
            "draft",
            "${review.outputs}",
        ),
    ];
    let mut loaded = Vec::new();
    for (name, src, activity, step, reference) in must_refuse {
        let (_, verdict) = load(name, src);
        match verdict {
            Ok(()) => loaded.push(name),
            Err(e) => {
                let rendered = e.to_string();
                for part in [activity, step, reference] {
                    assert!(
                        rendered.contains(part),
                        "{name}: the refusal names the activity, the step and the reference — \
                         `{part}` is missing from: {rendered}"
                    );
                }
            }
        }
    }
    assert!(
        loaded.is_empty(),
        "{} of 5 unlicensed forward-ref fixtures LOADED — the load has no licence check (D21-48 X8): \
         {loaded:?}",
        loaded.len()
    );
    // The licensed-but-mandatory ref is refused for its missing `?`, not for want of a licence.
    let (cat, _) = load("refused_mandatory_forward_ref.toml", MANDATORY);
    let refs = forward_refs(&cat.activities()[0]);
    assert_eq!(
        refs.iter()
            .map(|r| (r.optional, r.licence))
            .collect::<Vec<_>>(),
        vec![(false, Some(Licence::Letter))],
        "premise: the mandatory fixture's one forward ref is licensed and carries no `?`"
    );
}

/// **Every shipped forward ref is licensed, over a census read from inside this binary.** Prints
/// exactly one `FORWARD_REFS total=<n> letter=<a> repeat=<b> escalate=<c> unlicensed=<d>` line, which
/// Checkpoint 8d (6b) reads. `n > 0` over a non-zero count of activities walked (D21-42 13a),
/// `n = a + b + c + d`, `d = 0`, and `c > 0` because the lanes' own `minimal-repro` and
/// `tdd-red-green-refactor` refs ride the escalate licence. D21-57: the `${adversary.outputs?}`
/// back-edges the localisation and spec-form chains depend on (`root-cause`,
/// `differential-debugging`, `instrumented-trace-analysis`, `user-story-ac`) stay licensed.
#[test]
fn every_shipped_forward_ref_is_licensed_and_the_census_is_printed() {
    let dir = workspace_root().join(".redkiln").join("catalog");
    let catalog = Catalog::from_dir(&dir).expect("the shipped catalog parses");
    let (census, refused) = forward_ref_census(&catalog);
    println!("{census}");
    assert_eq!(census.activities, 102, "premise: the whole shipped catalog");
    assert!(
        census.total > 0,
        "a census over nothing is Halt 8's shape: {census:?}"
    );
    assert_eq!(
        census.total,
        census.letter + census.repeat + census.escalate + census.unlicensed,
        "the four classes partition the forward refs: {census:?}"
    );
    assert!(
        census.escalate > 0,
        "the lanes' minimal-repro and tdd refs ride the escalate licence: {census:?}"
    );
    let named: Vec<String> = refused.iter().map(ToString::to_string).collect();
    assert_eq!(
        census.unlicensed,
        0,
        "{} shipped forward ref(s) are unlicensed (D21-48 X8):\n  {}",
        census.unlicensed,
        named.join("\n  ")
    );
    // D21-57: the adversary back-edges stay licensed.
    for id in [
        "root-cause",
        "differential-debugging",
        "instrumented-trace-analysis",
        "user-story-ac",
    ] {
        let activity = catalog
            .activities()
            .iter()
            .find(|a| a.id == id)
            .unwrap_or_else(|| panic!("{id} is in the shipped catalog"));
        let back_edges: Vec<_> = forward_refs(activity)
            .into_iter()
            .filter(|r| r.reference == "${adversary.outputs?}")
            .collect();
        assert!(
            !back_edges.is_empty()
                && back_edges
                    .iter()
                    .all(rk_kernel::catalog::ForwardRef::is_admitted),
            "{id}: its `${{adversary.outputs?}}` back-edge must be a licensed forward ref (D21-57): \
             {back_edges:?}"
        );
    }
}

/// The four refs `s8d` §8d.3's table removes, with the version each moves to.
const REMOVED: [(&str, &str, &str, i64, i64); 4] = [
    ("adr-record", "draft", "answer", 1, 2),
    ("spike-question-spec", "draft", "answer", 1, 2),
    ("structured-nl-requirements", "draft", "resolution", 2, 3),
    ("vertical-slicing", "slice", "answer", 2, 3),
];

/// **The four unlicensed refs are removed under ADR-0072** (*"Edits to shipped activities made for
/// catalog coherence bump `version` and are listed in the pack changelog."*): each `(id, old, new)`
/// triple of §8d.3's table, each named input key gone from its step, and one changelog row per
/// triple naming `D21-48 X8` and the version it replaced.
#[test]
fn the_four_unlicensed_refs_are_removed_versioned_and_changelogged() {
    let root = workspace_root();
    let log = std::fs::read_to_string(root.join(".redkiln").join("CHANGELOG.md"))
        .expect(".redkiln/CHANGELOG.md exists (ADR-0072's pack changelog)");
    for (id, step, key, old, new) in REMOVED {
        let path = root
            .join(".redkiln")
            .join("catalog")
            .join(format!("{id}.toml"));
        let src = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("{} is readable: {e}", path.display()));
        let doc = toml_subset::parse(&src).unwrap_or_else(|e| panic!("{id}.toml parses: {e:?}"));
        let version = doc
            .get("activity")
            .and_then(toml_subset::Value::as_table)
            .and_then(|a| a.get("version"))
            .and_then(toml_subset::Value::as_int)
            .unwrap_or_else(|| panic!("{id} declares an integer version"));
        assert_eq!(
            version, new,
            "{id}: the removal bumps version {old} → {new} (ADR-0072)"
        );
        let the_step = doc
            .get("step")
            .and_then(toml_subset::Value::as_array)
            .expect("[[step]] tables")
            .iter()
            .filter_map(toml_subset::Value::as_table)
            .find(|s| s.get("id").and_then(toml_subset::Value::as_str) == Some(step))
            .unwrap_or_else(|| panic!("{id} has a `{step}` step"));
        let inputs = the_step
            .get("inputs")
            .and_then(toml_subset::Value::as_table)
            .unwrap_or_else(|| panic!("{id}/{step} binds inputs"));
        assert!(
            !inputs.contains_key(key),
            "{id}/{step} still binds `{key}` = {:?} — an unlicensed forward ref (D21-48 X8)",
            inputs.get(key)
        );
        let row = log
            .lines()
            .find(|l| l.starts_with(&format!("| {id}@{new} |")))
            .unwrap_or_else(|| panic!("no changelog row names {id}@{new} (ADR-0072)"));
        assert!(
            row.contains(&format!("from v{old}")) && row.contains("D21-48 X8"),
            "{id}@{new}'s changelog row names neither the version it replaced nor D21-48 X8: {row}"
        );
    }
}
