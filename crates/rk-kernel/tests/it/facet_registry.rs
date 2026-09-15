//! `facet_registry_entry_shape` — **AC-072**, the registry-entry loader rule, and the **THIRD** of
//! the three the register forbids collapsing into one another.
//!
//! **The eleven cases, one file each**, as s7b §7b.12 prescribes them: eight that must FAIL load —
//! predicates naming `dynamism` and `binary_change`; a `derived` entry with no `deriver`; a `source`
//! outside the five tokens; a `kernel` entry carrying a `deriver`; a `require` row keyed on a
//! `declared` facet; a fourth domain shape; an entry that abstains without `nullable = true` — and
//! **three that must LOAD**: a positive `nullable = true` entry whose `null` read emits exactly one
//! `gate_opened { reason: unknown_facet, axis }` per read facet and never an `Unsatisfiable`; the
//! **verbatim** `[facet.surfaces]` block; and a positive `set = true` entry whose member predicate
//! **resolves**.
//!
//! 🛑 **THE THREE POSITIVES ARE THE DISCRIMINATION, NOT DECORATION.** The criterion labels itself
//! *"NEGATIVE criterion apart from the positive fixtures, **which are what stop a reject-everything
//! lint passing**"*, and §7b.13 repeats it for this test by name: *"The `nullable = true` abstention
//! case and the three must-load cases are the discrimination; a reject-everything loader passes
//! without them."* Two of the three go further than loading — one drives the registry through
//! selection step 2 and one evaluates a member predicate — because *"the shape is recognised rather
//! than merely tolerated"* and *"never an `Unsatisfiable`"* are claims about behaviour that a parse
//! cannot make.
//!
//! **The fourth positive is the SHIPPED registry**, and it is the strongest one available: two of
//! the eleven cases are *"predicates naming `dynamism` and `binary_change` that must fail load
//! **against the current registry**"*, so this test loads `spikes/p9/facets.toml` itself and asserts
//! both that it is clean under the rule and that those two names are absent from it. A rule that
//! rejected the product registry would be caught here and nowhere else in the fixture set.
//!
//! **What is NOT claimed here** — each stated at [`rk_kernel::facet_registry`] with its measurement:
//! AC-072's **`anchor`** clause has no on-disk form in any record and is **not built**; the domain
//! shape set is **four on disk and three in the register**; the `nullable` clause is pinned at the
//! boolean grain ADR-0089 decision 3 decides and the general *"may abstain"* case is **ARMED**; and
//! the CLI surface is **Step 7d**'s over **Step 7e**'s file. Three records are OWED and this stage
//! takes none of the three decisions (`00-preamble.md`:55).

use std::collections::BTreeSet;

use rk_kernel::facet_registry::{
    DomainShape, EntryShapeReport, EntryShapeViolation, facet_registry_rule, matches_lexis,
    normalised_name,
};
use rk_kernel::predicate::{Context, Tri, Value};
use rk_kernel::registry::{Registry, SiteKind};
use rk_kernel::select::{ArtifactRegistry, Catalog, GateReason, Item, Pack, step2};

const DYNAMISM: &str =
    include_str!("../fixtures/registries/entry-shape/predicate_names_dynamism.toml");
const BINARY_CHANGE: &str =
    include_str!("../fixtures/registries/entry-shape/predicate_names_binary_change.toml");
const DERIVED_WITHOUT_DERIVER: &str =
    include_str!("../fixtures/registries/entry-shape/derived_without_deriver.toml");
const SOURCE_OUTSIDE_THE_FIVE: &str =
    include_str!("../fixtures/registries/entry-shape/source_outside_the_five.toml");
const KERNEL_ENTRY_CARRIES_DERIVER: &str =
    include_str!("../fixtures/registries/entry-shape/kernel_entry_carries_deriver.toml");
const REQUIRE_KEYED_ON_DECLARED: &str =
    include_str!("../fixtures/registries/entry-shape/require_keyed_on_declared.toml");
const DOMAIN_SHAPE_IS_A_FOURTH: &str =
    include_str!("../fixtures/registries/entry-shape/domain_shape_is_a_fourth.toml");
const ABSTAINS_WITHOUT_NULLABLE: &str =
    include_str!("../fixtures/registries/entry-shape/abstains_without_nullable.toml");
const POSITIVE_NULLABLE: &str =
    include_str!("../fixtures/registries/entry-shape/positive_nullable.toml");
const SURFACES_BLOCK_VERBATIM: &str =
    include_str!("../fixtures/registries/entry-shape/surfaces_block_verbatim.toml");
const POSITIVE_SET_MEMBER_PREDICATE: &str =
    include_str!("../fixtures/registries/entry-shape/positive_set_member_predicate.toml");

/// The pack whose two `require` rows the positive `nullable` fixture is built to satisfy, and the
/// artifact registry step 2 resolves kinds against — both already committed for AC-079's own case.
const INCIDENT_PACK: &str = include_str!("../fixtures/packs/incident-forbidden.toml");
const ARTIFACTS: &str = include_str!("../fixtures/registries/artifacts/artifacts.toml");

/// The shipped facet registry, read at the workspace root exactly as `selection.rs` and `rigor.rs`
/// read it. The path is on a non-comment line deliberately: *"against the current registry"* is the
/// criterion's own instrument for two of its eleven cases, and a registry of the implementer's
/// choosing is not that claim.
const SHIPPED_FACETS: &str = "spikes/p9/facets.toml";

/// The twenty entries the shipped registry carries [measured 2026-09-14]. Asserted as a floor and
/// not as an equality: `s7e` §7e.2 owns the repairs to that file and may add entries, but an
/// enumeration that COLLAPSES is the failure this number exists to catch — a rule that walked two
/// entries and reported `Ok` reads exactly like a clean registry.
const SHIPPED_ENTRIES: usize = 20;

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

fn shipped_registry_source() -> String {
    let path = workspace_root().join(SHIPPED_FACETS);
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry at {} is readable: {e}",
            path.display()
        )
    })
}

/// Fixtures are normalised to LF before any anchor is matched: `core.autocrlf` is `true` on this
/// repository, so a multi-line anchor would otherwise match on one checkout and not the next.
fn lf(src: &str) -> String {
    src.replace("\r\n", "\n")
}

/// One anchored edit, asserted before it writes (golden constraint 7).
fn patch(src: &str, anchor: &str, replacement: &str) -> String {
    let hits = src.matches(anchor).count();
    assert_eq!(
        hits, 1,
        "the repair anchor {anchor:?} must occur exactly once in the fixture, and occurs {hits}"
    );
    src.replace(anchor, replacement)
}

/// Run the rule with the registry read from one source and the predicate sites from another.
///
/// The two are separable because two of the eleven cases are predicates with **no registry of their
/// own**, asserted against the shipped one; every other case passes the same source twice.
fn verdict_against(
    registry_src: &str,
    sites_src: &str,
) -> Result<EntryShapeReport, Vec<EntryShapeViolation>> {
    let registry = Registry::from_toml(registry_src).expect("the fixture registry loads");
    let sites =
        Registry::predicate_sites_from_str(sites_src).expect("the fixture's predicate sites load");
    facet_registry_rule(&registry, &sites)
}

/// The report a clean load produces, with the walked counts asserted so an enumeration of
/// **nothing** cannot read as a clean registry — the job [`rk_kernel::rigor::Survey`]'s counts do
/// for AC-105 and [`rk_kernel::facet_provenance::ProvenanceReport`]'s do for AC-520.
fn loads(name: &str, registry_src: &str, sites_src: &str, min_entries: usize) -> EntryShapeReport {
    let report = verdict_against(registry_src, sites_src)
        .unwrap_or_else(|e| panic!("{name}: must LOAD, got {e:?}"));
    assert!(
        report.entries >= min_entries,
        "{name}: the rule walked {} entries, expected at least {min_entries} — a registry this \
         rule never looked at loads for the wrong reason (golden constraint 10)",
        report.entries
    );
    report
}

fn sole_violation(name: &str, registry_src: &str, sites_src: &str) -> EntryShapeViolation {
    let errs = verdict_against(registry_src, sites_src)
        .expect_err("the fixture must FAIL load — this is a must-fail fixture");
    assert_eq!(
        errs.len(),
        1,
        "{name}: expected exactly one clause to fail, got {errs:?}"
    );
    errs.into_iter().next().expect("one violation")
}

#[test]
fn facet_registry_entry_shape() {
    let shipped = lf(&shipped_registry_source());

    // ---- THE FOURTH POSITIVE: the SHIPPED registry is clean under this rule --------------------
    // Two of the eleven cases are asserted "against the current registry", so that registry is an
    // input to this test whether or not it is named as a fixture — and a rule that rejected it
    // would make those two cases fail for the rule's own defect instead of for the absent facet.
    // This is also where the module header's two recorded residuals are made falsifiable: if the
    // `anchor` clause were implemented, every one of these twenty entries would fail it; if the
    // domain-shape set admitted only the three the register names, `kind`, `cos`, `stage` and
    // `role` would fail it.
    let clean = loads("the shipped registry", &shipped, &shipped, SHIPPED_ENTRIES);
    assert_eq!(
        clean.shapes.get("kind"),
        Some(&DomainShape::Enumerated),
        "the shape the register does not name and four live entries carry — an amendment is OWED, \
         and this assertion is what makes the discrepancy visible rather than latent: {:?}",
        clean.shapes
    );
    for (facet, shape) in [
        ("stage", DomainShape::Enumerated),
        ("cos", DomainShape::Enumerated),
        ("role", DomainShape::Enumerated),
        ("criticality", DomainShape::Ordinal),
        ("rigor", DomainShape::Ordinal),
        ("cost", DomainShape::Ordinal),
        ("toolchain", DomainShape::Set),
        ("change", DomainShape::Set),
        ("surfaces", DomainShape::Set),
        ("external_dep", DomainShape::Boolean),
        ("recent_deploy", DomainShape::Boolean),
    ] {
        assert_eq!(
            clean.shapes.get(facet),
            Some(&shape),
            "the shipped {facet:?} entry declares the {} shape BY NAME",
            shape.as_str()
        );
    }
    // And the two names the first two cases turn on really are absent from it, so those cases fail
    // for the registry's content and not for a typo in this file.
    for absent in ["dynamism", "binary_change"] {
        assert!(
            !shipped.contains(&format!("[facet.{absent}]")),
            "{absent:?} must have NO entry in the shipped registry — that absence IS case 1/2"
        );
    }

    // ---- CASES 1 AND 2: predicates against the current registry --------------------------------
    // name · the predicate source · the unregistered name · the carrier · the repair
    for (name, src, facet, kind, anchor, replacement) in [
        (
            "predicate_names_dynamism",
            DYNAMISM,
            "dynamism",
            SiteKind::RequireWhen,
            "when = \"dynamism\"",
            "when = \"toolchain\"",
        ),
        (
            "predicate_names_binary_change",
            BINARY_CHANGE,
            "binary_change",
            SiteKind::ApplicableIf,
            "applicable_if = \"binary_change\"",
            "applicable_if = \"toolchain\"",
        ),
    ] {
        let src = &lf(src);
        let repaired = patch(src, anchor, replacement);
        let violation = sole_violation(name, &shipped, src);
        assert_eq!(
            violation.clause(),
            "unregistered_facet",
            "{name}: {violation}"
        );
        assert_eq!(violation.facet(), facet, "{name}");
        let text = violation.to_string();
        assert!(
            text.contains(facet) && text.contains(kind.as_str()),
            "{name}: the message names the facet AND the carrier a reader must go and edit, \
             because the fixture carries no registry entry to point at: {text}"
        );

        // DISCRIMINATION: the same predicate naming a REGISTERED facet loads against the same
        // registry, so the failure is the name and not the site, the carrier or the file.
        let report = loads(
            &format!("{name} (repaired)"),
            &shipped,
            &repaired,
            SHIPPED_ENTRIES,
        );
        assert_eq!(report.sites, 1, "{name}: one site, walked: {report:?}");
        assert_eq!(
            report.reads_resolved, 1,
            "{name}: and its one facet read RESOLVED — a site whose roots were never collected \
             also produces no violation: {report:?}"
        );
    }

    // ---- CASES 3 TO 8: the six self-contained must-fail files ----------------------------------
    // name · source · clause tag · the facet · repair anchor · repair replacement
    let cases: [(&str, &str, &str, &str, &str, &str); 6] = [
        (
            "derived_without_deriver",
            DERIVED_WITHOUT_DERIVER,
            "derived_without_deriver",
            "toolchain",
            "source = \"derived\"\nderiver_class",
            "source = \"derived\"\nderiver = \"R-toolchain-extension-manifest-table\"\n\
             deriver_class",
        ),
        (
            "source_outside_the_five",
            SOURCE_OUTSIDE_THE_FIVE,
            "source_outside_the_five",
            "toolchain",
            "source = \"Derived\"",
            "source = \"derived\"",
        ),
        (
            "kernel_entry_carries_deriver",
            KERNEL_ENTRY_CARRIES_DERIVER,
            "kernel_entry_carries_deriver",
            "rigor",
            "\nderiver = \"R-rigor-policy-lookup\"",
            "",
        ),
        (
            "require_keyed_on_declared",
            REQUIRE_KEYED_ON_DECLARED,
            "require_keyed_on_declared",
            "surfaces",
            "when = \"'flags' in surfaces\"",
            "when = \"kind == bug\"",
        ),
        (
            "domain_shape_is_a_fourth",
            DOMAIN_SHAPE_IS_A_FOURTH,
            "domain_shape_is_a_fourth",
            "data_migration",
            "values = [true, false, \"null\"]",
            "values = [true, false]",
        ),
        (
            "abstains_without_nullable",
            ABSTAINS_WITHOUT_NULLABLE,
            "abstains_without_nullable",
            "ui",
            "values = [true, false]\n",
            "values = [true, false]\nnullable = true\n",
        ),
    ];

    for (name, src, clause, facet, anchor, replacement) in cases {
        let src = &lf(src);
        let repaired = patch(src, anchor, replacement);
        let violation = sole_violation(name, src, src);
        assert_eq!(
            violation.clause(),
            clause,
            "{name}: the fixture must fail on ITS OWN clause, and failed on {violation}"
        );
        assert_eq!(violation.facet(), facet, "{name}");
        assert!(
            violation.to_string().contains(facet),
            "{name}: the message names the facet: {violation}"
        );

        // DISCRIMINATION: valid in that ONE respect, and the registry then loads. The repair to
        // `require_keyed_on_declared` is to the ROW and not to the entry, which is the clause's own
        // shape — the `declared` entry is lawful and the row keyed on it is not.
        loads(&format!("{name} (repaired)"), &repaired, &repaired, 1);
    }

    // ---- CASE 9: the positive `nullable = true` fixture, WHICH MUST LOAD AND THEN BEHAVE --------
    let nullable = lf(POSITIVE_NULLABLE);
    let report = loads("positive_nullable", &nullable, &nullable, 2);
    assert_eq!(
        report.shapes.get("external_dep"),
        Some(&DomainShape::Boolean),
        "the boolean shape, declared by name and carrying its `nullable = true`"
    );

    // "...whose `null` read emits EXACTLY ONE `gate_opened { reason: unknown_facet, axis }` PER
    // READ FACET and NEVER an `Unsatisfiable`." This is AC-079's route at selection step 2, driven
    // over THIS registry: the criterion's positive fixture is a claim about what a nullable entry
    // DOES, and a registry that merely parsed would satisfy none of it.
    let pack = Pack::from_toml("incident-forbidden.toml", INCIDENT_PACK).expect("the pack loads");
    let artifacts = ArtifactRegistry::from_toml("artifacts.toml", ARTIFACTS)
        .expect("the artifact registry loads");
    let facets = Registry::from_toml(&nullable).expect("the fixture registry loads");
    let abstaining = Item::new("INC-1", "incident")
        .with_facet_ordinal("criticality", &["low", "med", "high", "critical"], "low")
        .with_facet_null("external_dep");
    let out = step2(&pack, &artifacts, &facets, "analysis", &abstaining)
        .expect("an abstention is not an error");
    let gates = out.gates();
    assert_eq!(gates.len(), 1, "one gate, not one per row: {gates:?}");
    assert_eq!(gates[0].reason, GateReason::UnknownFacet);
    assert_eq!(gates[0].axis, "external_dep");

    // "...and NEVER an `Unsatisfiable`", which is step 4's and is reserved for predicates that
    // evaluated to `false`. Asserted over the whole steps-1-to-4 path, where a wrong implementation
    // that routed the `null` on to step 4 WOULD gate the abstaining row's kind.
    let catalog = Catalog::from_sources(crate::selection::CATALOG).expect("the catalog loads");
    let whole = rk_kernel::select::select(
        &pack,
        &catalog,
        &artifacts,
        &facets,
        "analysis",
        &abstaining,
    )
    .expect("selection resolves");
    let gated: Vec<&str> = whole
        .unsatisfiable()
        .iter()
        .map(|u| u.kind.as_str())
        .collect();
    assert!(
        !gated.contains(&"APIContract"),
        "the abstaining row's kind must not be gated Unsatisfiable — the order is lint at load \
         (AC-072/AC-069), gate at step 2 (AC-079), Unsatisfiable at step 4 (AC-054): {gated:?}"
    );
    assert_eq!(whole.gates().len(), 1, "and the gate survives step 4");

    // AND THE NEGATIVE COMPANION ON THE POSITIVE, so this fixture is not green because the rule is
    // inert: deleting the one `nullable = true` line from the entry that abstains must trip the
    // abstention clause and nothing else.
    let spoiled = patch(&nullable, "\nnullable = true", "");
    let violation = sole_violation("positive_nullable (spoiled)", &spoiled, &spoiled);
    assert_eq!(violation.clause(), "abstains_without_nullable");
    assert_eq!(violation.facet(), "external_dep");

    // ---- CASE 10: the VERBATIM `[facet.surfaces]` block, WHICH MUST LOAD -----------------------
    let surfaces = lf(SURFACES_BLOCK_VERBATIM);
    let report = loads("surfaces_block_verbatim", &surfaces, &surfaces, 1);
    assert_eq!(report.shapes.get("surfaces"), Some(&DomainShape::Set));
    // "COPIED RATHER THAN REFERENCED" is §7b.12's instruction, and a copy nothing compares is a
    // copy that goes stale silently. The four lines are asserted against the shipped file, so a
    // change to the product registry breaks this test rather than leaving the criterion pinned to
    // a block that no longer exists.
    let block: String = surfaces
        .lines()
        .skip_while(|l| !l.starts_with("[facet.surfaces]"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !block.is_empty() && lf(&shipped).contains(block.trim_end()),
        "the committed copy must appear in {SHIPPED_FACETS} byte for byte:\n{block}"
    );
    // The three ABSENCES the block is a positive for, asserted rather than assumed: a `declared`
    // entry carries no deriver of any kind and that is LAWFUL.
    let entry = Registry::from_toml(&surfaces)
        .expect("loads")
        .facet("surfaces")
        .cloned()
        .expect("the entry is present");
    assert!(
        entry.deriver.is_none() && entry.deriver_class.is_none() && entry.deriver_record.is_none()
    );
    assert!(entry.set, "`set = true`, declared by name");

    // ---- CASE 11: the positive `set = true` fixture, WHOSE MEMBER PREDICATE RESOLVES -----------
    let sets = lf(POSITIVE_SET_MEMBER_PREDICATE);
    let report = loads("positive_set_member_predicate", &sets, &sets, 2);
    assert_eq!(report.shapes.get("toolchain"), Some(&DomainShape::Set));
    assert_eq!(report.shapes.get("change"), Some(&DomainShape::Set));
    assert_eq!(report.sites, 1, "its one `require` row, walked: {report:?}");
    assert_eq!(report.reads_resolved, 1, "and its one member read resolved");

    // "so the shape is RECOGNISED RATHER THAN MERELY TOLERATED". The row's own predicate, taken
    // from the fixture rather than retyped here, evaluated against a set-valued context: a loader
    // that accepted the entry and an evaluator that returned `null` for the member read would
    // satisfy every assertion above and none of this one.
    let sites = Registry::predicate_sites_from_str(&sets).expect("the sites load");
    let member = &sites.sites()[0];
    assert_eq!(member.kind, SiteKind::RequireWhen);
    assert_eq!(
        member.keys.iter().cloned().collect::<Vec<_>>(),
        vec!["toolchain".to_owned()]
    );
    let ctx = Context::new().with("toolchain", Value::set(["node", "rust"]));
    assert_eq!(
        super::support::eval(&member.predicate, &ctx).value,
        Tri::True,
        "the member predicate {:?} must RESOLVE over the declared set shape",
        member.predicate
    );
    // And it resolves to FALSE, never to `null`, for a member the set does not carry — which is
    // what tells a recognised shape from a tolerated one: an unrecognised domain abstains.
    let elsewhere = Context::new().with("toolchain", Value::set(["dotnet"]));
    assert_eq!(
        super::support::eval(&member.predicate, &elsewhere).value,
        Tri::False
    );

    // ---- THE CLAUSES THE ELEVEN FILES DO NOT SEPARATE, patched onto them ------------------------
    // §7b.12 fixes the fixture DIRECTORY at "AC-072's eleven", so a twelfth committed file would
    // change a set the register owns. These are patched instead, on `facet_provenance.rs`'s
    // precedent for AC-520's interval clause.

    // (a) "a declared domain" — an entry with NO `values` at all. The register distinguishes this
    //     from a fourth shape in terms: "deleting `values` fails this criterion's OWN
    //     declared-domain clause", which is why it is a separate variant and not a third `found`.
    let no_domain = patch(
        &lf(SURFACES_BLOCK_VERBATIM),
        "values = [\"flags\", \"migration\", \"payments\", \"secrets\"]   # placeholder; the real set comes from surfaces.toml (above)\n",
        "",
    );
    let violation = sole_violation("no declared domain", &no_domain, &no_domain);
    assert_eq!(violation.clause(), "no_declared_domain", "{violation}");
    assert_eq!(violation.facet(), "surfaces");

    // (b) TWO SHAPE FLAGS AT ONCE is also a shape with no name. "By name rather than by omission"
    //     cuts both ways: an entry that declares two of the three has declared none of them, and a
    //     loader that took the first flag it found would silently pick one.
    let two_shapes = patch(
        &lf(SURFACES_BLOCK_VERBATIM),
        "source = \"declared\"\nset = true",
        "source = \"declared\"\nset = true\nordered = true",
    );
    let violation = sole_violation("ordered and set together", &two_shapes, &two_shapes);
    assert_eq!(
        violation.clause(),
        "domain_shape_is_a_fourth",
        "{violation}"
    );

    // (c) THE FACET LEXIS (ADR-0103 decision 2), whose rule is a NORMALISATION and not a rejection:
    //     "ADR-0030's hyphenated spellings reading as their underscore forms". So the hyphenated
    //     name LOADS and the mixed-case one does not, and a rule that simply banned the hyphen
    //     would fail the first of these.
    assert_eq!(normalised_name("binary-change"), "binary_change");
    assert!(matches_lexis(&normalised_name("binary-change")));
    assert!(!matches_lexis("Binary_Change"));
    assert!(!matches_lexis("2fast"));
    // The anchor carries its leading newline: this fixture's header quotes the block's name in
    // prose, so the bare header text occurs twice and `patch` refuses it rather than rewriting a
    // comment (`facet_provenance.rs`'s `lexical_keys_require` anchor, same reason).
    let hyphenated = patch(
        &lf(SURFACES_BLOCK_VERBATIM),
        "\n[facet.surfaces]",
        "\n[facet.sur-faces]",
    );
    loads(
        "a hyphenated spelling reads as its underscore form",
        &hyphenated,
        &hyphenated,
        1,
    );
    let shouted = patch(
        &lf(SURFACES_BLOCK_VERBATIM),
        "\n[facet.surfaces]",
        "\n[facet.Surfaces]",
    );
    let violation = sole_violation("a name outside the lexis", &shouted, &shouted);
    assert_eq!(
        violation.clause(),
        "facet_name_outside_lexis",
        "{violation}"
    );

    // (d) THE THIRD CARRIER. AC-072's GIVEN names `applicable_if`, `require.when` AND
    //     `claimed_predicate`; the eleven files exercise the first two, and a rule blind to the
    //     third would pass a deviation request that invents a facet — which is exactly the shape
    //     AC-065's sibling clause is about, one predicate site over.
    let deviation = format!(
        "{}\n[[deviation]]\ntarget = \"bisect\"\nclaimed_predicate = \"dynamism\"\n",
        lf(SURFACES_BLOCK_VERBATIM)
    );
    let violation = sole_violation(
        "a claimed_predicate naming an unregistered facet",
        &deviation,
        &deviation,
    );
    assert_eq!(violation.clause(), "unregistered_facet", "{violation}");
    assert!(
        violation.to_string().contains("claimed_predicate"),
        "the message names the third carrier: {violation}"
    );

    // (e) THE CARVE-OUT, which is what stops this rule being a reject-everything lint over
    //     predicates. `facets.toml`:1 — "Every predicate identifier must be a facet here, A STEP
    //     ID, `rigor`, `item`, or a builtin." A rule that flagged every non-facet root would fail
    //     every real catalog file, so the carve-out is asserted positively and its `builtin` half
    //     is ARMED (see `PredicateSites`).
    let carved = format!(
        "{}\n[activity]\nid = \"x\"\napplicable_if = \"scan and item\"\n[[step]]\nid = \"scan\"\n",
        lf(SURFACES_BLOCK_VERBATIM)
    );
    let report = loads("the step-id and `item` carve-out", &carved, &carved, 1);
    assert_eq!(report.sites, 1);
    assert_eq!(
        report.reads_resolved, 0,
        "neither root is a facet, and neither is a violation: {report:?}"
    );
    let sites = Registry::predicate_sites_from_str(&carved).expect("the sites load");
    assert_eq!(
        sites.step_ids(),
        &BTreeSet::from(["scan".to_owned()]),
        "the carve-out is READ from the document and never assumed"
    );
    // The control on the carve-out: a root that is neither a step id nor `item` still fails, so
    // the allowance is scoped rather than a hole.
    let hole = patch(
        &carved,
        "applicable_if = \"scan and item\"",
        "applicable_if = \"scan and dynamism\"",
    );
    let violation = sole_violation("the carve-out is scoped", &hole, &hole);
    assert_eq!(violation.clause(), "unregistered_facet", "{violation}");
    assert_eq!(violation.facet(), "dynamism");
}
