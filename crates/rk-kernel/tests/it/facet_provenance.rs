//! `facet_provenance_fixtures` — **AC-520**, the derived-facet sibling gate.
//!
//! **Every clause fixture AC-520's *Verified by* enumerates, plus the one that must LOAD.** The set
//! is taken from the register by reference and no count is restated: `ERRATA.md`'s **E-0520-1**
//! (2026-09-13) records that the criterion's headline *"six must-fail fixture files"* is
//! **pre-amendment** and that **the enumeration is the authority, not the number** — the
//! 2026-09-13 amendment inserted `deriver_class` and `deriver_record`, two clause groups of two
//! files apiece, and did not revisit the sentence.
//!
//! The must-load companion is **not decoration**: *"without which the rule is satisfied by a lint
//! that rejects everything."* It is asserted here twice over — it loads, and it **stops** loading
//! when one field of it is spoiled, so its green is not the green of an inert rule.
//!
//! Every must-fail case runs the same two-step discrimination
//! [`super::facet_stability`] uses: exactly one violation, of the clause the file is named for;
//! then the fixture repaired **in that one respect**, by an anchored edit asserted to match exactly
//! once, must load clean.

use rk_kernel::facet_provenance::{ProvenanceReport, ProvenanceViolation, facet_provenance_rule};
use rk_kernel::registry::Registry;

const NO_DERIVER: &str = include_str!("../fixtures/registries/facet-provenance/no_deriver.toml");
const DERIVER_WITHOUT_RECORD: &str =
    include_str!("../fixtures/registries/facet-provenance/deriver_without_record.toml");
const RECORD_ZERO_POSITIVES_KEYS_REQUIRE: &str =
    include_str!("../fixtures/registries/facet-provenance/record_zero_positives_keys_require.toml");
const LEXICAL_KEYS_REQUIRE: &str =
    include_str!("../fixtures/registries/facet-provenance/lexical_keys_require.toml");
const NO_DERIVER_CLASS: &str =
    include_str!("../fixtures/registries/facet-provenance/no_deriver_class.toml");
const DERIVER_CLASS_NOT_A_TOKEN: &str =
    include_str!("../fixtures/registries/facet-provenance/deriver_class_not_a_token.toml");
const NO_DERIVER_RECORD: &str =
    include_str!("../fixtures/registries/facet-provenance/no_deriver_record.toml");
const DERIVER_RECORD_ABSENT_AT_PIN: &str =
    include_str!("../fixtures/registries/facet-provenance/deriver_record_absent_at_pin.toml");
const KERNEL_FACET_CARRIES_DERIVER: &str =
    include_str!("../fixtures/registries/facet-provenance/kernel_facet_carries_deriver.toml");
const REQUIRE_KEYED_ON_DECLARED: &str =
    include_str!("../fixtures/registries/facet-provenance/require_keyed_on_declared.toml");
const POSITIVE_COMPANION: &str =
    include_str!("../fixtures/registries/facet-provenance/positive_companion.toml");

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

fn verdict(src: &str) -> Result<ProvenanceReport, Vec<ProvenanceViolation>> {
    let registry = Registry::from_toml(src).expect("the fixture registry loads");
    let corpus = Registry::pinned_corpus_from_str(src).expect("the fixture's pinned corpus loads");
    facet_provenance_rule(&registry, &corpus)
}

/// The report a clean load produces, with the walked count asserted so an enumeration of
/// **nothing** cannot read as a clean registry — the job [`rk_kernel::rigor::Survey`]'s counts do
/// for AC-105.
///
/// `min_derived` is 0 for the one fixture whose subject is a `kernel` facet and which therefore has
/// no derived entry at all; every other case passes 1 or more.
fn loads(name: &str, src: &str, min_derived: usize) -> ProvenanceReport {
    let report = verdict(src).unwrap_or_else(|e| panic!("{name}: must LOAD, got {e:?}"));
    assert!(
        report.derived_facets >= min_derived,
        "{name}: the rule walked {} derived facets, expected at least {min_derived} — a registry \
         this gate never looked at loads for the wrong reason (golden constraint 10)",
        report.derived_facets
    );
    report
}

fn sole_violation(name: &str, src: &str) -> ProvenanceViolation {
    let errs = verdict(src).expect_err("the fixture must FAIL load — this is a must-fail fixture");
    assert_eq!(
        errs.len(),
        1,
        "{name}: expected exactly one clause to fail, got {errs:?}"
    );
    errs.into_iter().next().expect("one violation")
}

#[test]
fn facet_provenance_fixtures() {
    // name · source · clause tag · repair anchor · repair replacement · the facet it is about
    let cases: [(&str, &str, &str, &str, &str, &str); 10] = [
        (
            "no_deriver",
            NO_DERIVER,
            "no_deriver",
            "source = \"derived\"\nderiver_class",
            "source = \"derived\"\nderiver = \"R-toolchain-extension-manifest-table\"\n\
             deriver_class",
            "toolchain",
        ),
        (
            "deriver_without_record",
            DERIVER_WITHOUT_RECORD,
            "deriver_without_record",
            "deriver = \"R-some-other-deriver\"",
            "deriver = \"R-toolchain-extension-manifest-table\"",
            "toolchain",
        ),
        (
            "record_zero_positives_keys_require",
            RECORD_ZERO_POSITIVES_KEYS_REQUIRE,
            "record_zero_positives_keys_require",
            "positives = 0\nnot_measurable = true",
            // A scored record needs the interval too: AC-520's THEN names both, and only the NOT
            // MEASURABLE shape is exempt — so repairing the positives alone would trip the
            // interval clause and the discrimination would read as a second failure.
            "positives = 3\nnot_measurable = false\ninterval = [0.29, 1.00]",
            "recent_deploy",
        ),
        (
            "lexical_keys_require",
            LEXICAL_KEYS_REQUIRE,
            "lexical_keys_require",
            // Anchored on the ENTRY's own two lines: this fixture's header quotes the register's
            // clause verbatim, so `deriver_class = "lexical"` alone occurs twice and `patch`
            // refuses it rather than silently rewriting a comment.
            "R-ui-path-and-diff-scan\"\nderiver_class = \"lexical\"",
            "R-ui-path-and-diff-scan\"\nderiver_class = \"structural\"",
            "ui",
        ),
        (
            "no_deriver_class",
            NO_DERIVER_CLASS,
            "no_deriver_class",
            "deriver = \"R-change-extension-and-numstat-marker\"\nderiver_record",
            "deriver = \"R-change-extension-and-numstat-marker\"\n\
             deriver_class = \"structural\"\nderiver_record",
            "change",
        ),
        (
            "deriver_class_not_a_token",
            DERIVER_CLASS_NOT_A_TOKEN,
            "deriver_class_not_a_token",
            "deriver_class = \"heuristic\"",
            "deriver_class = \"structural\"",
            "change",
        ),
        (
            "no_deriver_record",
            NO_DERIVER_RECORD,
            "no_deriver_record",
            "deriver_class = \"lexical\"\n",
            "deriver_class = \"lexical\"\n\
             deriver_record = \"results/p1-arm-d-panel-scored.json\"\n",
            "flag_exists",
        ),
        (
            "deriver_record_absent_at_pin",
            DERIVER_RECORD_ABSENT_AT_PIN,
            "deriver_record_absent_at_pin",
            "results/does-not-exist-at-the-pin.json",
            "results/p1-arm-d-panel-scored.json",
            "flag_exists",
        ),
        (
            "kernel_facet_carries_deriver",
            KERNEL_FACET_CARRIES_DERIVER,
            "kernel_facet_carries_deriver",
            "\nderiver = \"R-rigor-policy-lookup\"",
            "",
            "rigor",
        ),
        (
            "require_keyed_on_declared",
            REQUIRE_KEYED_ON_DECLARED,
            "require_keyed_on_declared",
            "when = \"'flags' in surfaces\"",
            "when = \"kind == bug\"",
            "surfaces",
        ),
    ];

    for (name, src, clause, anchor, replacement, facet) in cases {
        let src = &lf(src);
        let repaired = patch(src, anchor, replacement);
        let violation = sole_violation(name, src);
        assert_eq!(
            violation.clause(),
            clause,
            "{name}: the fixture must fail on ITS OWN clause, and failed on {violation}"
        );
        assert_eq!(violation.facet(), facet, "{name}");
        let text = violation.to_string();
        assert!(
            text.contains(facet),
            "{name}: the message names the facet: {text}"
        );

        // DISCRIMINATION: valid in that ONE respect, and the registry then loads.
        // Two fixtures' subjects are ADR-0093 decision 1's classes that sit OUTSIDE this gate —
        // `kernel_facet_carries_deriver`'s is a `kernel` entry and `require_keyed_on_declared`'s a
        // `declared` one — so neither carries a derived facet and neither can be asked for one.
        // Every other repaired fixture must still have one for the gate to have looked at.
        let min_derived = usize::from(
            name != "kernel_facet_carries_deriver" && name != "require_keyed_on_declared",
        );
        loads(&format!("{name} (repaired)"), &repaired, min_derived);
    }

    // ---- the POSITIVE COMPANION, which must load ----------------------------------------------
    let report = loads("positive_companion", &lf(POSITIVE_COMPANION), 1);
    assert_eq!(
        report.derived_facets, 1,
        "the companion's one derived facet"
    );
    assert_eq!(report.records_read, 1, "its one record, resolved and read");
    assert!(
        report.not_measurable.is_empty(),
        "the companion's record carries 15 positives, so nothing is reported not-measurable: \
         {report:?}"
    );

    // ---- and the companion is not passing because the rule is inert ----------------------------
    // The negative control on the positive control. Each spoiling is one field, and each must be
    // caught; a rule that returned Ok(()) unconditionally passes the line above and fails these.
    for (anchor, replacement, clause) in [
        (
            "deriver_class = \"structural\"",
            "deriver_class = \"lexical\"",
            "lexical_keys_require",
        ),
        (
            "positives = 15",
            "positives = 0",
            "record_zero_positives_keys_require",
        ),
        (
            "deriver = \"R-toolchain-extension-manifest-table\"\nderiver_class",
            "deriver_class",
            "no_deriver",
        ),
    ] {
        let spoiled = patch(&lf(POSITIVE_COMPANION), anchor, replacement);
        let violation = sole_violation("positive_companion (spoiled)", &spoiled);
        assert_eq!(
            violation.clause(),
            clause,
            "spoiling {anchor:?} in the companion must trip {clause}, and tripped {violation}"
        );
    }

    // ---- THE THEN'S OTHER HALF: a record carrying a count and NO Wilson interval ---------------
    // AC-520's THEN is "a scored precision record — which must exist for that deriver over a
    // pinned corpus, CARRYING A WILSON 95% INTERVAL AND A POSITIVE-INSTANCE COUNT". Until
    // 2026-09-13 `PrecisionRecord` had no field for the interval at all and a record with none
    // passed the gate. This is patched rather than committed as an eleventh must-fail file for the
    // reason §7b.7 gives: the ten names come from the register's enumeration and the checkpoint
    // asserts exactly those ten by name, so a new file would change a set the register owns.
    let no_interval = patch(
        &lf(POSITIVE_COMPANION),
        "\ninterval = [0.48, 0.89]   # the Wilson 95% interval AC-520's THEN names beside the count (F39)",
        "",
    );
    let violation = sole_violation("companion (interval deleted)", &no_interval);
    assert_eq!(
        violation.clause(),
        "record_without_interval",
        "a record with a positive count and no interval fails the THEN's second half: {violation}"
    );
    assert_eq!(violation.facet(), "toolchain");
    // AND THE NOT MEASURABLE EXEMPTION, which is why the clause is not a blanket one: a record with
    // no denominator reports no rate, and demanding an interval of it would be a number invented
    // for a measurement nobody made (E-0089-1's whole point about `data_migration`).
    let not_measurable_no_interval = patch(
        &lf(RECORD_ZERO_POSITIVES_KEYS_REQUIRE),
        "when = \"recent_deploy\"",
        "when = \"kind == bug\"",
    );
    let report = loads(
        "not-measurable, no interval",
        &not_measurable_no_interval,
        1,
    );
    assert_eq!(
        report.not_measurable.len(),
        1,
        "and it is REPORTED: {report:?}"
    );

    // ---- AC-520's "IS REPORTED SO — NEITHER PASSED NOR FAILED" ---------------------------------
    // The second AND-clause has two halves and only the second was built: "a facet whose scored
    // record shows no positive instance, or is NOT MEASURABLE, IS REPORTED SO — neither passed nor
    // failed — and may key no require row". A not-measurable facet that keys NO row produced
    // nothing at all: no violation, and no channel to carry a report. That is also the fixture
    // that pins the `keys_a_row &&` conjunct on the zero-positives bar — deleting it makes this
    // case FAIL, which is the "rejects everything" direction the positive companion guards.
    let nm = report.not_measurable.first().expect("one entry");
    assert_eq!(nm.facet, "recent_deploy");
    assert_eq!(nm.deriver, "R-recent-deploy-feed-absent");
    assert!(
        nm.declared,
        "the record carries its own NOT MEASURABLE marker — a feed absent at the pin, not a \
         scored zero (E-0067-1)"
    );
    assert!(
        nm.to_string().contains("NOT MEASURABLE"),
        "the report says so in the words the criterion uses: {nm}"
    );
    // A SCORED ZERO is the other route to the same clause and is reported differently, so the two
    // are not collapsed: `data_migration` is 0 positives over 20 items, not an absent feed.
    let scored_zero = patch(
        &not_measurable_no_interval,
        "positives = 0\nnot_measurable = true",
        "positives = 0\nnot_measurable = false\ninterval = [0.00, 0.00]",
    );
    let report = loads("scored zero, keys no row", &scored_zero, 1);
    assert_eq!(report.not_measurable.len(), 1);
    assert!(
        !report.not_measurable[0].declared,
        "a scored zero is reported as scored, not as a declared NOT MEASURABLE: {report:?}"
    );

    // ---- GROUP 7'S FOUR FORMS, and only one of them was a fixture ------------------------------
    // "a `kernel`-class facet carrying a `deriver`, a `deriver_class`, a `deriver_record` OR
    // CLAIMING A RECORD (ADR-0093 forbids)". E-0520-1's rule is that the enumeration decides, and
    // `kernel_facet_carries_deriver.toml` carries only a `deriver`, so a rule checking that one
    // field passed the whole group. Each of the other three is patched onto the same fixture.
    let kernel_clean = patch(
        &lf(KERNEL_FACET_CARRIES_DERIVER),
        "\nderiver = \"R-rigor-policy-lookup\"",
        "",
    );
    for (field, added) in [
        ("deriver", "\nderiver = \"R-rigor-policy-lookup\""),
        ("deriver_class", "\nderiver_class = \"structural\""),
        (
            "deriver_record",
            "\nderiver_record = \"results/p1-arm-d-panel-scored.json\"",
        ),
        (
            "a scored record",
            // The fourth form is not an entry field: it is the corpus carrying a row in the
            // kernel facet's own name. See `PinnedCorpus::claimed_by`.
            "\n\n[[corpus_record]]\npath = \"results/p1-arm-d-panel-scored.json\"\n\
             deriver = \"rigor\"\npositives = 4\ninterval = [0.40, 0.97]",
        ),
    ] {
        let src = patch(
            &kernel_clean,
            "source = \"kernel\"",
            &format!("source = \"kernel\"{added}"),
        );
        let violation = sole_violation(&format!("kernel facet carrying {field}"), &src);
        assert_eq!(
            violation.clause(),
            "kernel_facet_carries_deriver",
            "group 7 names four forms and this is {field}: {violation}"
        );
        assert!(
            violation.to_string().contains(field),
            "the message names WHICH of the four forms was carried: {violation}"
        );
    }
    // And the same entry with none of the four loads — so the four cases above failed for what
    // was added and not for anything about a kernel facet as such.
    assert_eq!(
        verdict(&kernel_clean).map(|r| r.derived_facets),
        Ok(0),
        "a kernel facet carrying none of the four forms is outside this gate by construction"
    );

    // ---- the gate's SUBJECT is `source = "derived"` --------------------------------------------
    // A `kernel` facet carrying NO provenance and a `declared` facet keying NO row are outside this
    // gate by construction (ADR-0093 decision 1), and the companion above already carries both. The
    // control that matters here is the other direction: an `llm` facet with no stability record is
    // AC-071's failure and must not be reported by this rule, or the two gates have collapsed into
    // one and AC-520's "why it is a criterion and not a sixth clause" paragraph is undone.
    let llm = "[facet.criticality]\nvalues = [\"low\", \"med\", \"high\"]\nsource = \"llm\"\n";
    assert_eq!(
        verdict(llm).map(|r| r.derived_facets),
        Ok(0),
        "an `llm` facet is AC-071's subject; a provenance rule that also failed on it would be the \
         collapse the register forbids"
    );
}
