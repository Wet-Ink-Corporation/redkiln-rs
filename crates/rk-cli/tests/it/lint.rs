//! **`lint`'s two modes** — the CLI surface over `rk-kernel`'s two facet rules.
//!
//! **Neither AC-071 nor AC-520 is claimed here.** §7d.14: *"the CLI surface `rk-kernel`'s `lib.rs`
//! owes; AC-071/AC-520 are not claimed"*, and §7d.3: *"The lints' criteria are 7b's and 7e's; **7d
//! claims the surface only** and neither id."* That is why the test below carries no `ac071_` or
//! `ac520_` prefix — the spelling is the one §7d.17's name loop asserts.

use std::path::{Path, PathBuf};

use rk_cli::cli::LintArgs;
use rk_cli::verbs::lint;

fn facets(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("facets")
        .join(name)
}

/// The repository root the verb reads `CORPUS_PIN` from — this repository, which carries one.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the workspace root resolves")
}

fn args(stability: bool, file: &str, pinned_corpus: Option<&str>) -> LintArgs {
    LintArgs {
        facet_stability: stability,
        facet_provenance: !stability,
        facets_toml: facets(file),
        stability_record: None,
        pinned_corpus: pinned_corpus.map(facets),
    }
}

/// **The exit table `lint` uses, driven over all three fixtures.** §7d.3, verbatim:
/// *"`--facet-stability <fixture>` over a facets file with an `llm`-sourced facet exits `3` and
/// names the violation …; `--facet-provenance` likewise over a declared-surface violation …; a clean
/// pair exits `0`."*
///
/// **"Pair" means the clean fixture is run through BOTH modes** — one mode passing says nothing
/// about the other, and a `lint` that exited `0` unconditionally on one branch would be invisible.
///
/// §7d.14's reds: *"Red by the `llm`-sourced fixture exiting `0`, by the clean pair exiting
/// non-zero, or by an exit outside the table."*
#[test]
fn lint_wraps_kernel_facet_rules_and_exits_by_table() {
    let root = workspace_root();
    assert!(
        root.join("CORPUS_PIN").is_file(),
        "PREMISE: CORPUS_PIN is absent from {}, so the stability arm would refuse on the pin read \
         rather than on the rule",
        root.display()
    );

    // ---- the clean PAIR: both modes, exit 0. ---------------------------------------------------
    let mut out: Vec<u8> = Vec::new();
    let clean_stability = lint::run_with(&args(true, "clean.toml", None), &root, &mut out);
    assert_eq!(
        clean_stability.code(),
        0,
        "the clean fixture failed `--facet-stability`"
    );

    let mut out: Vec<u8> = Vec::new();
    let clean_provenance = lint::run_with(
        &args(false, "clean.toml", Some("clean.toml")),
        &root,
        &mut out,
    );
    assert_eq!(
        clean_provenance.code(),
        0,
        "the clean fixture failed `--facet-provenance`; stdout:\n{}",
        String::from_utf8_lossy(&out)
    );

    // The report is PRINTED on the Ok arm, and the counts are what stop an enumeration of nothing
    // reading as clean — `ProvenanceReport` exists because `Ok(())` is the wrong answer to AC-520's
    // second AND-clause, and swallowing it here would reproduce that defect at the CLI.
    let printed = String::from_utf8_lossy(&out);
    assert!(
        printed.contains("derived facets: 2"),
        "`--facet-provenance` did not print its report, or read no derived facet — a pass over \
         nothing. stdout:\n{printed}"
    );

    // 🛑 AND THE NOT-MEASURABLE LIST IS NAMED, which is the clause `ProvenanceReport` exists for:
    // AC-520's *"a facet whose scored record shows no positive instance, or is NOT MEASURABLE, is
    // reported so — neither passed nor failed"*. Measured at this cycle: with the fixture's
    // `recent_deploy` row absent the list was EMPTY, and the declared mutant "the provenance report
    // is SWALLOWED" ran GREEN — the counts line alone cannot tell a rendered empty list from a
    // dropped one. The fixture now carries a not-measurable facet and this asserts it reaches stdout.
    assert!(
        printed.contains("recent_deploy") && printed.contains("NOT MEASURABLE"),
        "the provenance report was SWALLOWED: a facet the pinned corpus reports NOT MEASURABLE is \
         neither passed nor failed and must be REPORTED, and swallowing it reproduces at the CLI \
         the exact defect `ProvenanceReport` was created to fix. stdout:\n{printed}"
    );

    // ---- the `llm`-sourced fixture: exit 3, and the violation NAMED. ---------------------------
    //
    // Run WITHOUT `--stability-record`, so §7d.3's "absent → an empty map, the zero-`llm` branch"
    // is what produces the refusal.
    let mut out: Vec<u8> = Vec::new();
    let llm = lint::run_with(&args(true, "llm-sourced.toml", None), &root, &mut out);
    assert_eq!(
        llm.code(),
        3,
        "the `llm`-sourced fixture did not exit 3 under `--facet-stability`"
    );

    // ---- the declared-surface fixture: exit 3 under provenance. --------------------------------
    let mut out: Vec<u8> = Vec::new();
    let declared = lint::run_with(
        &args(false, "undeclared-surface.toml", Some("clean.toml")),
        &root,
        &mut out,
    );
    assert_eq!(
        declared.code(),
        3,
        "the declared-surface fixture did not exit 3 under `--facet-provenance`"
    );

    // ---- every code is ON the table. -----------------------------------------------------------
    //
    // "an exit outside the table" is a named mutant, so the four readings above are checked against
    // `Exit::ALL` rather than only against the two values this test expects.
    for exit in [clean_stability, clean_provenance, llm, declared] {
        assert!(
            rk_cli::exit::ALL.contains(&exit),
            "`lint` produced {exit:?}, which is not a row of the closed exit table"
        );
    }
    // `lint` never exits 5 — that row is `explain`'s `NoRecord` — and never 4.
    for exit in [clean_stability, clean_provenance, llm, declared] {
        assert!(
            exit.code() != 5 && exit.code() != 4,
            "`lint` produced {}, which is not one of its rows",
            exit.code()
        );
    }
}

/// The kernel rules' own violations are what `lint` renders, and this asserts the rendering names
/// the facet rather than only the code — §7d.3's *"exits `3` **and names the violation**"*.
///
/// It is a separate case because the table test above reads exit codes, and a refusal that printed
/// nothing would satisfy every assertion there.
#[test]
fn lint_refusals_name_the_facet() {
    let registry = rk_kernel::registry::Registry::from_toml(
        &std::fs::read_to_string(facets("llm-sourced.toml")).expect("the fixture reads"),
    )
    .expect("the fixture loads");
    let violations = rk_kernel::facet_stability::facet_stability_rule(
        &registry,
        &std::collections::BTreeMap::new(),
        "61f590e51cbb00301c310e46aab615563a7787b8",
    )
    .expect_err("the `llm`-sourced fixture violates the stability rule");
    assert_eq!(
        violations.len(),
        1,
        "the fixture must produce EXACTLY ONE violation, and it must be the clause the file is \
         named for — `facet_stability.rs`'s own house rule. Got: {violations:?}"
    );
    assert_eq!(violations[0].facet(), "criticality");

    let registry = rk_kernel::registry::Registry::from_toml(
        &std::fs::read_to_string(facets("undeclared-surface.toml")).expect("the fixture reads"),
    )
    .expect("the fixture loads");
    let corpus = rk_kernel::registry::Registry::pinned_corpus_from_str(
        &std::fs::read_to_string(facets("clean.toml")).expect("the fixture reads"),
    )
    .expect("the corpus reads");
    let violations = rk_kernel::facet_provenance::facet_provenance_rule(&registry, &corpus)
        .expect_err("the declared-surface fixture violates the provenance rule");
    assert_eq!(
        violations.len(),
        1,
        "exactly one violation, and it must be the clause the file is named for. Got: {violations:?}"
    );
    assert_eq!(violations[0].clause(), "require_keyed_on_declared");
    assert_eq!(violations[0].facet(), "surfaces");
}
