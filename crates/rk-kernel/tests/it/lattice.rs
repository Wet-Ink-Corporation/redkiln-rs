//! `rk_kernel::lattice` — parity with `spikes/p9/lattice.py` at `P9_PIN`, the reading over
//! `.redkiln/`, and the red fixtures AC-101 to AC-104 are verified by (s8b §8b.2).
//!
//! **Parity is about bytes.** The two parity tests render the port's report, digest tags included,
//! and compare it line for line with the goldens the Python printed at `P9_PIN`
//! (`tests/fixtures/lattice/p9-run.txt`, `p9-validate.txt`). A line the port does not reproduce is
//! named in `divergences.tsv` with its cause, and is held to that row's weaker assertion; no other
//! line is skipped. The goldens are re-derived from the Python by Checkpoint 8b section (7), never
//! from this port.
//!
//! **Every test asserts a non-zero count walked before its verdict** — a green over zero contexts
//! is Halt 8's shape.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rk_kernel::lattice::{
    self, AuthBinding, Axes, BOOLS, COVERING_ARRAY, Inputs, LatticeError, Report, SHIPPED_SURFACES,
    Sources, ToolchainRow,
};
use rk_kernel::predicate::{Expr, parse};
use rk_kernel::registry::Registry;
use rk_kernel::rigor::Rigor;

use crate::p9_pin;

const GOLDEN_RUN: &str = include_str!("../fixtures/lattice/p9-run.txt");
const GOLDEN_VALIDATE: &str = include_str!("../fixtures/lattice/p9-validate.txt");
const DIVERGENCES: &str = include_str!("../fixtures/lattice/divergences.tsv");
const PYTHON_ROWS: &str = include_str!("../fixtures/lattice/p9-covering-array.txt");

/// The shape of the reading over the installed pair, measured and pinned (s8b §8b.2). AC-101 and
/// AC-102 over these two packs are REPORTED, not thresholded (D21-47 V3).
///
/// *(dated 2026-09-29, s8g build, package S8G-P4-PACK-REHOME; D21-43 N23, D21-46 R5; R1 =
/// ADR-0191 decisions 1 and 3 to 6, proposed.)* **Re-pinned: `SHIPPED_DEAD` ~~56~~ 57.** The
/// re-run over the re-homed catalog (`TestReport` pass-sourced; the four N23 providers and
/// `integration-tests` at `implement` over `AuthoredTests`, keyed by toolchain PROVISIONALLY)
/// reads `pairs=279936 unsat=1296 dead=57/102`. The dead set is the entry's 56 (measured at
/// `100a108` in a private worktree) plus exactly one id, **`unit-tests`**: its `max_rigor =
/// "recommended"` meets only the `criticality >= high` escalation at `rigor = "mandatory"`, the one
/// demand for `AuthoredTests`. `unsat` did not move: `IntegrationReport`'s entry moved from story
/// `verify` to story `implement` with its row, and `TestReport` is no entry (pass-sourced). Against
/// AC-102's 20% line: 57/102 = 55.9%, reported, not thresholded, for the installed pair (D21-47 V3).
const SHIPPED_PAIRS: u64 = 279_936;
const SHIPPED_UNSAT: u64 = 1_296;
const SHIPPED_DEAD: usize = 57;

fn workspace() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn fixture(name: &str) -> Sources {
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/lattice");
    Sources {
        catalog: base.join(name).join("catalog"),
        packs: base.join(name).join("packs"),
        facets: base.join("common/facets.toml"),
        artifacts: base.join("common/artifacts.toml"),
    }
}

fn load(sources: &Sources) -> Inputs {
    lattice::load(sources).unwrap_or_else(|e| panic!("the lattice inputs load: {e}"))
}

fn run(inputs: &Inputs, axes: &Axes) -> Report {
    lattice::run(inputs, axes, None).unwrap_or_else(|e| panic!("the lattice runs: {e}"))
}

/// `divergences.tsv`'s rows for one golden: line number (1-based) → reason.
fn divergences(golden: &str) -> BTreeMap<usize, String> {
    let mut out = BTreeMap::new();
    for row in DIVERGENCES.lines().filter(|l| !l.is_empty()) {
        let cols: Vec<&str> = row.split('\t').collect();
        assert_eq!(
            cols.len(),
            3,
            "a divergence row is `<golden>\\t<line>\\t<reason>`: {row:?}"
        );
        if cols[0] == golden {
            let n: usize = cols[1].parse().expect("a divergence names a line number");
            assert!(
                out.insert(n, cols[2].to_owned()).is_none(),
                "line {n} is named twice"
            );
        }
    }
    out
}

/// A `soft … ->` example line the port need not reproduce byte for byte: same tag, same kind, same
/// pack, and a closed cycle over optional-inclusive edges of activities that pack selected.
fn assert_soft_cycle_equivalent(
    n: usize,
    golden: &str,
    port: &str,
    report: &Report,
    inputs: &Inputs,
) {
    let head = format!("{} | soft ", report.tag());
    let g = golden
        .strip_prefix(&head)
        .unwrap_or_else(|| panic!("golden line {n} is not a soft-cycle example: {golden:?}"));
    let p = port
        .strip_prefix(&head)
        .unwrap_or_else(|| panic!("port line {n} is not a soft-cycle example: {port:?}"));
    let (gp, _) = g.split_once(' ').expect("the golden line names a pack");
    let (pp, cycle) = p.split_once(' ').expect("the port line names a pack");
    assert_eq!(
        gp, pp,
        "line {n}: the port's soft-cycle example names another pack"
    );
    let ids: Vec<&str> = cycle.trim().split(" -> ").collect();
    assert!(
        ids.len() >= 3 && ids.first() == ids.last(),
        "line {n}: {cycle:?} is not a closed cycle"
    );
    let selected = report.selected_in(pp);
    let acts: BTreeMap<&str, _> = inputs
        .catalog()
        .activities()
        .iter()
        .map(|a| (a.id.as_str(), a))
        .collect();
    for w in ids.windows(2) {
        let (from, to) = (acts[w[0]], acts[w[1]]);
        assert!(
            selected.contains(w[0]) && selected.contains(w[1]),
            "line {n}: {w:?} were not selected under {pp}"
        );
        assert!(
            to.requires.iter().any(|r| from.provides.contains(&r.kind)),
            "line {n}: {} -> {} is not an edge — {} requires nothing {} provides",
            w[0],
            w[1],
            w[1],
            w[0]
        );
    }
}

/// Byte equality with `golden`, line for line, except the rows `divergences.tsv` names.
fn assert_parity(
    name: &str,
    golden: &str,
    port: &[String],
    named: &BTreeMap<usize, String>,
    mut on_named: impl FnMut(usize, &str, &str),
) {
    let lines: Vec<&str> = golden.lines().collect();
    assert!(!lines.is_empty(), "{name} is empty — there is no oracle");
    let mut named_seen = 0;
    for (i, g) in lines.iter().enumerate() {
        let n = i + 1;
        let p = port.get(i).unwrap_or_else(|| {
            panic!(
                "{name}: the port printed {} lines; the golden has {}",
                port.len(),
                lines.len()
            )
        });
        if named.contains_key(&n) {
            named_seen += 1;
            on_named(n, g, p);
        } else {
            assert_eq!(p, g, "{name} line {n} differs and no divergence names it");
        }
    }
    assert_eq!(
        port.len(),
        lines.len(),
        "{name}: the port printed {} lines; the golden has {}",
        port.len(),
        lines.len()
    );
    assert_eq!(
        named_seen,
        named.len(),
        "{name}: a divergence row names a line the golden does not have"
    );
}

#[test]
fn shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged() {
    // lattice.py's lists, transcribed here independently of the module.
    let python_toolchains = [
        ("node", vec!["node"], vec!["text"]),
        ("unreal", vec!["unreal"], vec!["binary"]),
        (
            "node+unreal",
            vec!["node", "unreal"],
            vec!["text", "binary"],
        ),
        ("dotnet", vec!["dotnet"], vec!["text"]),
    ];
    let python_bools = [
        "auth",
        "data_migration",
        "external_dep",
        "ui",
        "market_facing",
        "has_repro",
        "flag_exists",
        "recent_deploy",
    ];
    let registered = Axes::p6_registered();
    assert!(
        !registered.toolchains.is_empty(),
        "the registered axes carry no toolchain"
    );
    let got: Vec<(&str, Vec<&str>, Vec<&str>)> = registered
        .toolchains
        .iter()
        .map(|t| (t.name, t.tools.clone(), t.change.clone()))
        .collect();
    assert_eq!(
        got,
        python_toolchains.to_vec(),
        "Axes::p6_registered()'s toolchains are not lattice.py's TOOLCHAINS"
    );
    assert_eq!(
        registered.bools,
        python_bools.to_vec(),
        "the registered booleans are not lattice.py's BOOLS"
    );
    assert_eq!(BOOLS.to_vec(), python_bools.to_vec());
    assert_eq!(registered.covering_rows, COVERING_ARRAY.to_vec());
    assert_eq!(
        registered.surfaces, None,
        "the registered axes bind no `surfaces`"
    );
    assert_eq!(registered.auth, AuthBinding::CoveringArray);

    let shipped = Axes::shipped();
    let mut expected = registered.clone();
    expected.toolchains.push(ToolchainRow {
        name: "rust",
        tools: vec!["rust"],
        change: vec!["text"],
    });
    let mut surfaces = vec![Vec::new()];
    surfaces.extend(SHIPPED_SURFACES.iter().map(|s| vec![*s]));
    expected.surfaces = Some(surfaces);
    assert_eq!(
        shipped, expected,
        "Axes::shipped() is not the registered set plus exactly rust and surfaces"
    );
    assert_eq!(shipped.surfaces.as_ref().map(Vec::len), Some(6));

    // D21-56 (i): the shipped registry declares no `auth` facet, and its surfaces are the six rows'.
    let facets = std::fs::read_to_string(workspace().join(".redkiln/facets.toml"))
        .expect(".redkiln/facets.toml reads");
    let registry = Registry::from_toml(&facets).expect(".redkiln/facets.toml loads");
    assert!(
        registry.facets().count() > 0,
        "the shipped registry declares no facet at all"
    );
    assert!(
        registry.facet("auth").is_none(),
        ".redkiln/facets.toml declares an `auth` facet — the inert column is no longer inert"
    );
    let declared: Vec<&str> = registry
        .facet("surfaces")
        .expect("surfaces is declared")
        .values
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert_eq!(
        declared,
        SHIPPED_SURFACES.to_vec(),
        "the shipped surfaces enumeration is not the registry's values"
    );

    // D21-56 (ii): a bare `auth` is a field path under ADR-0103, never the surfaces literal 'auth'...
    assert_eq!(
        parse("auth").expect("`auth` parses").expr,
        Expr::Path("auth".to_owned())
    );
    // ...and fed through the shipped load entry point it is refused before any evaluation.
    let probe = PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("lattice-auth-probe-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&probe);
    let red = workspace().join(".redkiln");
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
    let control = lattice::load(&Sources::under(&probe));
    assert!(
        control.as_ref().is_ok_and(|i| i.catalog().len() == 102),
        "control: the copied shipped tree must load: {:?}",
        control.err()
    );
    std::fs::write(
        probe.join("catalog/zz-auth-probe.toml"),
        "[activity]\nid = \"zz-auth-probe\"\nversion = 1\nstage = \"design\"\nprovides = [\"ThreatModel\"]\n\
         method_rank = 99\ncost_class = \"S\"\napplicable_if = \"auth\"\n",
    )
    .expect("the probe activity is written");
    match lattice::load(&Sources::under(&probe)) {
        Err(LatticeError::UnregisteredFacet {
            facet,
            predicate,
            site,
        }) => {
            assert_eq!(facet, "auth");
            assert_eq!(predicate, "auth");
            assert!(
                site.contains("zz-auth-probe"),
                "the refusal names the site: {site}"
            );
        }
        Err(other) => panic!("a bare `auth` was refused for the wrong reason: {other}"),
        Ok(_) => panic!(
            "a bare `auth` loaded through the shipped entry point — it would evaluate against the inert covering column"
        ),
    }
    // Refuting round 4 (2026-09-25): the same bare `auth`, inside a METHOD-FORM quantifier body over
    // a facet. A `surfaces` element is a string, not a Record, so the body's frame is empty and its
    // `auth` falls back to the context — the covering column — exactly as a top-level `auth` does.
    // Loaded through the same entry point, it must be refused the same way (M30b).
    std::fs::write(
        probe.join("catalog/zz-auth-probe.toml"),
        "[activity]\nid = \"zz-auth-probe\"\nversion = 1\nstage = \"design\"\nprovides = [\"ThreatModel\"]\n\
         method_rank = 99\ncost_class = \"S\"\napplicable_if = \"surfaces.any(auth)\"\n",
    )
    .expect("the method-form probe activity is written");
    match lattice::load(&Sources::under(&probe)) {
        Err(LatticeError::UnregisteredFacet {
            facet,
            predicate,
            site,
        }) => {
            assert_eq!(facet, "auth");
            assert_eq!(predicate, "surfaces.any(auth)");
            assert!(
                site.contains("zz-auth-probe"),
                "the refusal names the site: {site}"
            );
        }
        Err(other) => {
            panic!("`surfaces.any(auth)` was refused for the wrong reason: {other}")
        }
        Ok(_) => panic!(
            "`surfaces.any(auth)` loaded through the shipped entry point — its body's `auth` would evaluate against the inert covering column"
        ),
    }
    // Refuting round 6 (2026-09-25): the PREFIX-FORM quantifier. Its body is checked less its
    // variable, so a bare `auth` in it is refused like the other two (M30c: the `Some(v)` arm skips
    // the body walk)...
    std::fs::write(
        probe.join("catalog/zz-auth-probe.toml"),
        "[activity]\nid = \"zz-auth-probe\"\nversion = 1\nstage = \"design\"\nprovides = [\"ThreatModel\"]\n\
         method_rank = 99\ncost_class = \"S\"\napplicable_if = \"any(s in surfaces, auth)\"\n",
    )
    .expect("the prefix-form probe activity is written");
    match lattice::load(&Sources::under(&probe)) {
        Err(LatticeError::UnregisteredFacet {
            facet,
            predicate,
            site,
        }) => {
            assert_eq!(facet, "auth");
            assert_eq!(predicate, "any(s in surfaces, auth)");
            assert!(
                site.contains("zz-auth-probe"),
                "the refusal names the site: {site}"
            );
        }
        Err(other) => {
            panic!("`any(s in surfaces, auth)` was refused for the wrong reason: {other}")
        }
        Ok(_) => panic!(
            "`any(s in surfaces, auth)` loaded through the shipped entry point — its body's `auth` would evaluate against the inert covering column"
        ),
    }
    // ...and its bound variable is not a facet root: `s == 'auth'` compares an element with the
    // literal, so the same activity LOADS (the control that the refusal above is about `auth`).
    std::fs::write(
        probe.join("catalog/zz-auth-probe.toml"),
        "[activity]\nid = \"zz-auth-probe\"\nversion = 1\nstage = \"design\"\nprovides = [\"ThreatModel\"]\n\
         method_rank = 99\ncost_class = \"S\"\napplicable_if = \"any(s in surfaces, s == 'auth')\"\n",
    )
    .expect("the prefix-form control activity is written");
    let bound_var = lattice::load(&Sources::under(&probe));
    assert!(
        bound_var.as_ref().is_ok_and(|i| i.catalog().len() == 103),
        "`any(s in surfaces, s == 'auth')` reads no unregistered root and must load: {:?}",
        bound_var.err()
    );
    let _ = std::fs::remove_dir_all(&probe);
}

#[test]
fn covering_array_is_pairwise_and_matches_the_python_rows() {
    let mut lines = PYTHON_ROWS.lines();
    let header = lines.next().expect("the printed file has a header");
    assert_eq!(
        header,
        format!("# {}", BOOLS.join(" ")),
        "the printed rows are over other columns"
    );
    let python: Vec<[u8; 8]> = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let v: Vec<u8> = l
                .split(' ')
                .map(|x| x.parse().expect("a 0/1 cell"))
                .collect();
            v.try_into().expect("eight cells a row")
        })
        .collect();
    assert_eq!(
        python.len(),
        8,
        "the Python printed {} rows; run10 records bool-covering-rows=8",
        python.len()
    );
    let mut checked = 0;
    for i in 0..8 {
        for j in (i + 1)..8 {
            for (a, b) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                checked += 1;
                assert!(
                    COVERING_ARRAY.iter().any(|r| r[i] == a && r[j] == b),
                    "the pair ({}={a}, {}={b}) is covered by no row",
                    BOOLS[i],
                    BOOLS[j]
                );
            }
        }
    }
    assert_eq!(
        checked, 112,
        "the pairwise walk checked {checked} pairs, not 28 x 4"
    );
    assert_eq!(
        COVERING_ARRAY.to_vec(),
        python,
        "the transcribed rows are not the rows lattice.py prints"
    );
}

#[test]
fn p6_lattice_parity_over_the_p9_tree() {
    let inputs = load(&Sources::under(p9_pin::root()));
    assert_eq!(
        inputs.catalog().len(),
        102,
        "the pinned catalog holds 102 activities"
    );
    let report = run(&inputs, &Axes::p6_registered());
    assert_eq!(
        report.packs().len(),
        12,
        "the parity run walks the twelve p9 packs"
    );
    assert!(
        report.contexts("pack-default") > 0 && report.pairs("pack-default") > 0,
        "the parity run walked nothing"
    );
    println!(
        "lattice-p9: packs={} contexts={} pairs={} unsat={} dead={}/{} nulls={:?}",
        report.packs().len(),
        report.contexts("pack-default"),
        report.pairs("pack-default"),
        report.unsat_pairs("pack-default"),
        report.dead().len(),
        report.catalog_len(),
        report.null_verdicts()
    );
    let named = divergences("p9-run.txt");
    let port = report.lines();
    assert_parity("p9-run.txt", GOLDEN_RUN, &port, &named, |n, g, p| {
        let reason = &named[&n];
        assert!(
            reason.starts_with("soft-cycle example:"),
            "divergence row {n} has no assertion kind: {reason}"
        );
        println!(
            "lattice-p9-divergence: line {n} golden={:?} port={:?}",
            g.split(" | ").nth(1),
            p.split(" | ").nth(1)
        );
        assert_soft_cycle_equivalent(n, g, p, &report, &inputs);
    });
}

#[test]
fn p6_validate_parity_over_the_p9_tree() {
    let inputs = load(&Sources::under(p9_pin::root()));
    let v = lattice::validate(&inputs, &Axes::p6_registered()).expect("validate runs");
    assert!(
        !v.families.is_empty() && v.families.iter().all(|f| f.contexts_checked > 0),
        "validate walked no family or no context"
    );
    let named = divergences("p9-validate.txt");
    assert_parity(
        "p9-validate.txt",
        GOLDEN_VALIDATE,
        &v.lines(),
        &named,
        |n, _, _| {
            panic!(
                "p9-validate.txt line {n} is named as a divergence, and no assertion kind covers a validate line"
            );
        },
    );
}

#[test]
fn p6_lattice_over_the_shipped_pack() {
    let axes = Axes::shipped();
    let inputs = load(&Sources::under(&workspace().join(".redkiln")));
    let report = run(&inputs, &axes);
    let (contexts, pairs, unsat) = (
        report.contexts("pack-default"),
        report.pairs("pack-default"),
        report.unsat_pairs("pack-default"),
    );
    let ties = report.ties().values().map(BTreeSet::len).sum::<usize>();
    let hard = report.hard_cycle_runs();
    println!(
        "lattice-shipped: packs={} rigor=pack-default contexts={contexts} pairs={pairs} unsat={unsat} dead={}/{} ties={ties} hard_cycles={hard} toolchains={} surfaces={}",
        report.packs().len(),
        report.dead().len(),
        report.catalog_len(),
        report.toolchains().join(","),
        report.surface_rows()
    );
    // What only the product evaluator decides: every predicate that came back `null` (never admits).
    println!("lattice-shipped-nulls: {:?}", report.null_verdicts());
    assert_eq!(
        report.packs().len(),
        2,
        "the lattice walked {} packs over .redkiln/, not the installed 2",
        report.packs().len()
    );
    assert!(
        contexts > 0 && pairs > 0,
        "the shipped run walked zero contexts or pairs (Halt 8)"
    );
    assert_eq!(report.catalog_len(), 102);
    assert!(
        report.toolchains().contains(&"rust") && report.surface_rows() == 6,
        "the shipped run did not walk rust and the six surfaces rows"
    );
    assert_eq!(
        ties,
        0,
        "AC-103 over the installed pair: rank ties {:?}",
        report.ac103_errors()
    );
    assert_eq!(
        hard,
        0,
        "AC-103 over the installed pair: hard cycles {:?}",
        report.ac103_errors()
    );
    // *(dated 2026-09-29, s8g build, package S8G-P4-PACK-REHOME; R1 = ADR-0191 decision 1, proposed.)*
    // `TestReport` is `source = "pass"`: its one producer is the verify pass, which the selector
    // selects no catalog producer for and never gates `Unsatisfiable` (`KindSource::Pass`). The
    // lattice reads selection, so a pass-sourced kind is no unsat entry here either.
    let pass_unsat: Vec<&(String, String, String)> = report
        .unsat_examples()
        .keys()
        .filter(|(_, _, k)| k.starts_with("TestReport"))
        .collect();
    assert!(
        pass_unsat.is_empty(),
        "a pass-sourced kind is reported unsatisfiable: {pass_unsat:?}"
    );
    for ((p, s, k), e) in report.unsat_examples() {
        println!("lattice-shipped-unsat: {p} {s} {k} first-mode={}", e.mode);
    }
    println!("lattice-shipped-dead: {:?}", report.dead());
    assert_eq!(
        (pairs, unsat, report.dead().len()),
        (SHIPPED_PAIRS, SHIPPED_UNSAT, SHIPPED_DEAD),
        "the shipped reading moved: dead = {:?}; nulls = {:?}",
        report.dead(),
        report.null_verdicts()
    );
}

#[test]
fn p6_validate_over_the_shipped_pack() {
    let inputs = load(&Sources::under(&workspace().join(".redkiln")));
    let v = lattice::validate(&inputs, &Axes::shipped()).expect("validate runs over .redkiln/");
    assert!(
        !v.families.is_empty(),
        "the shipped catalog declares no family — totality would pass vacuously"
    );
    for f in &v.families {
        assert!(
            f.contexts_checked > 0,
            "family {} was checked in zero contexts",
            f.name
        );
    }
    for line in v.lines() {
        println!("lattice-shipped-validate: {line}");
    }
    let holes: Vec<&str> = v
        .errors
        .iter()
        .filter(|e| e.contains("totality"))
        .map(String::as_str)
        .collect();
    assert!(
        holes.is_empty(),
        "AC-104: family totality fails over .redkiln/: {holes:#?}"
    );
}

#[test]
fn zero_packs_is_refused_not_reported() {
    let inputs = load(&fixture("dead-activity"));
    assert!(
        !inputs.pack_ids().is_empty(),
        "the fixture holds no pack, so a zero selection proves nothing"
    );
    // lattice.py --packs <path> matches no id and prints a whole baseline over zero packs.
    match lattice::run(&inputs, &Axes::p6_registered(), Some(&["packs/story.toml"])) {
        Err(LatticeError::NoPacks { requested, .. }) => {
            assert_eq!(requested, Some(vec!["packs/story.toml".to_owned()]));
        }
        Err(other) => panic!("a zero-pack selection was refused for the wrong reason: {other}"),
        Ok(report) => panic!(
            "a zero-pack selection produced a report over {} packs",
            report.packs().len()
        ),
    }
    // The control: the same call naming a real id runs.
    let one = lattice::run(&inputs, &Axes::p6_registered(), Some(&["story"]))
        .expect("a one-pack selection runs");
    assert_eq!(one.packs(), ["story".to_owned()]);
    assert!(one.contexts("pack-default") > 0);
}

#[test]
fn unsatisfiable_fixture_names_its_cell() {
    let inputs = load(&fixture("unsatisfiable"));
    let report = run(&inputs, &Axes::p6_registered());
    assert!(
        report.pairs("pack-default") > 0,
        "the fixture walked no pair"
    );
    assert!(
        report.unsat_pairs("pack-default") > 0,
        "AC-101: the require row with no producer made no pair unsatisfiable"
    );
    let ex = report
        .unsat_examples()
        .get(&(
            "story".to_owned(),
            "specification".to_owned(),
            "ThreatModel".to_owned(),
        ))
        .unwrap_or_else(|| {
            panic!(
                "no unsatisfiable (story, specification, ThreatModel): {:?}",
                report.unsat_examples().keys()
            )
        });
    assert!(
        ex.vector.contains("kind=story") && ex.vector.contains("size=high"),
        "the cell's facet vector is not named: {}",
        ex.vector
    );
    assert_eq!(
        ex.mode, "optional",
        "the first cell is the first mode enumerated"
    );
    assert_eq!(ex.rigor, Rigor::Optional, "the cell's rigor is not named");
    assert_eq!(
        ex.origin.as_deref(),
        Some("story: stage.specification.require[0] when = \"size >= high\""),
        "the row is not named"
    );
    assert!(
        ex.failed.is_empty() && ex.other_stage.is_empty(),
        "no activity anywhere provides ThreatModel: {ex:?}"
    );

    // I4: a row's own `rigor = "optional"` under a stage at `recommended` is RAISED to the stage's
    // rigor, never lowered. TestResult's only producer stops at max_rigor = "optional", so the
    // demand is unsatisfiable from the first mode above optional; a lowered row would admit it.
    let raised = report
        .unsat_examples()
        .get(&("story".to_owned(), "specification".to_owned(), "TestResult".to_owned()))
        .unwrap_or_else(|| {
            panic!(
                "I4: the optional row was not raised to the stage's rigor — TestResult was admitted \
                 at {:?}",
                report.unsat_examples().keys()
            )
        });
    assert_eq!(
        raised.mode, "recommended",
        "the first raised cell is the first mode above optional"
    );
    assert_eq!(
        raised.rigor,
        Rigor::Recommended,
        "the row was not raised to the stage's rigor"
    );
    assert_eq!(
        raised.failed,
        ["loose-tester:max_rigor=optional".to_owned()]
    );
    assert_eq!(
        raised.origin.as_deref(),
        Some("story: stage.specification.require[1] when = \"kind == story\"")
    );
}

#[test]
fn rank_tie_fixture_is_an_error() {
    let inputs = load(&fixture("rank-tie"));
    let report = run(&inputs, &Axes::p6_registered());
    assert!(
        report.pairs("pack-default") > 0,
        "the fixture walked no pair"
    );
    let errors = report.ac103_errors();
    assert!(
        errors
            .iter()
            .any(|e| e.contains("rank tie") && e.contains("patch-a, patch-b")),
        "AC-103: two members at one method_rank are not an error: {errors:?}"
    );
    assert!(
        report
            .lines()
            .iter()
            .any(|l| l.ends_with("[('patch-a', 'patch-b')]")),
        "the report's rank-tie section does not list the tie"
    );
}

#[test]
fn hard_cycle_fixture_is_an_error() {
    let inputs = load(&fixture("hard-cycle"));
    let report = run(&inputs, &Axes::p6_registered());
    assert!(
        report.pairs("pack-default") > 0,
        "the fixture walked no pair"
    );
    assert!(
        report.hard_cycle_runs() > 0,
        "AC-103: the injected cycle was found in no run"
    );
    let cycle = report
        .hard_cycles()
        .get("story")
        .expect("the story pack carries a hard cycle");
    assert!(
        cycle.len() == 3 && cycle.first() == cycle.last(),
        "not a closed two-activity cycle: {cycle:?}"
    );
    let members: BTreeSet<&str> = cycle.iter().map(String::as_str).collect();
    assert_eq!(members, BTreeSet::from(["alpha", "beta"]));
    let errors = report.ac103_errors();
    assert!(
        errors.iter().any(|e| e.starts_with("hard cycle")
            && e.contains("alpha")
            && e.contains("beta")
            && e.contains(" -> ")),
        "the error does not name both activities and the cycle: {errors:?}"
    );
}

#[test]
fn dead_activity_fixture_is_listed() {
    let inputs = load(&fixture("dead-activity"));
    let report = run(&inputs, &Axes::p6_registered());
    assert!(
        report.contexts("pack-default") > 0,
        "the fixture walked no context"
    );
    assert_eq!(report.catalog_len(), 5);
    assert_eq!(
        report.dead(),
        ["late-patch".to_owned(), "orphan-maker".to_owned()],
        "AC-102: the dead list"
    );
    assert!(
        report
            .lines()
            .iter()
            .any(|l| l.contains("| dead=2/5 (40.0%)")),
        "the report does not compute dead=2/5"
    );
    let dead = u64::try_from(report.dead().len()).expect("small");
    assert!(
        lattice::kill_line_fires(dead, 5, 20),
        "AC-102's > 20% line does not fire on 2/5"
    );
    assert!(
        !lattice::kill_line_fires(1, 5, 20),
        "AC-102's > 20% line fires on 1/5 (exactly 20%)"
    );
}

/// The report-branches fixture's first-example vectors, derived by hand: every band `low`, covering
/// row 0 (`1 0 1 0 0 0 1 0`), and toolchain `node` or `node+unreal`.
const V_NODE: &str = "kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0";
const V_NU: &str = "kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node+unreal auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0";

/// **D21-67 ruling 6** (s8g section 8g.2's named precondition of the lattice re-run): the five
/// unsat-report branches no golden byte pinned, reached by ONE hand-derived fixture
/// (`tests/fixtures/lattice/report-branches/`, sharing `common/`).
///
/// | branch | where the fixture reaches it |
/// | --- | --- |
/// | `forbidden_stage` (`forbidden-stage=`) | `base-maker` requires `ForbiddenOnly`, whose one producer sits at the forbidden `operate` |
/// | `unsat_other_stage` (`wrong-stage-producer=`) | `WrongStage` is required at `design` and produced only at `implement`; `ForbiddenOnly`'s entry counts too (failed empty, other non-empty) |
/// | the example's `other_stage` list | both of those entries' examples |
/// | the keyed render `{kind}{suffix}` | `KeyedGap` is keyed by toolchain; `keyed-maker` admits `unreal` and `dotnet` only, so the `node` instance is unsat |
/// | the transitive entry `<kind>:transitive@<stage>` | `Chain` is keyed; at `tc=node+unreal` its `node` instance selects `chain-maker`, whose own `Missing` has no producer, and its `unreal` instance fails `tags` |
///
/// `base-maker` (rank 10) is admissible only when the toolchain set holds both `node` and `unreal`,
/// so `ForbiddenOnly` and `Chain` are demanded at `tc=node+unreal` alone and that context is their
/// first example; `base-fallback` (rank 20) produces `Base` everywhere else.
///
/// **The expected lines were derived by hand before the fixture existed** (never copied from a
/// run): the story pack applies to `kind == story` only, so it runs 4 modes x 81 band sets x 4
/// toolchains x 8 covering rows = 10368 contexts, 2592 per toolchain; no predicate reads a band, a
/// boolean or the rigor, so every count is a toolchain count. `WrongStage` is unsat in all 10368;
/// `KeyedGap{node}` wherever the set holds `node` (`node`, `node+unreal`: 5184); `ForbiddenOnly`
/// and `Chain` at `node+unreal` only (2592). The first example is the first context enumerated:
/// mode `optional`, every band `low`, covering row 0 (`1 0 1 0 0 0 1 0`), toolchain `node` or, for
/// the two `base-maker` entries, `node+unreal`.
///
/// **The one quoted `lattice.py` cross-check** *(2026-09-29, s8g build, package S8G-P4-PACK-REHOME)*,
/// by `s8b` §8b.2's staged-copy method: `lattice.py` written from `P9_PIN` by `git show` into
/// `target/s8g-p4-lattice-xcheck` (a relative path), the fixture's `catalog/` and `packs/` and
/// `common/`'s two files copied to its top level, run there with no `--packs` and no `--catalog`
/// (`PYTHONHASHSEED=0 PYTHONIOENCODING=utf-8`), rc 0. Its guards, read before any line was compared:
/// `P6 lattice run; catalog=6 activities; packs=['story']; pack digests: story=ed04d04ada09`;
/// `prereg pairs=  2592` on each of the four per-mode lines; the tag
/// `cat:1f77bb0f139b fac+art:2de68beff0ee pk:ed04d04ada09` (`common/`'s tag moved from
/// `1cd93c6cef11` with this fixture's six kinds; never `spikes/p9`'s `592fa138d68d`). Its unsat
/// section, byte-identical to this port's four lines (`cmp`), tag elided:
///
/// ```text
/// story          design        WrongStage             n=10368 forbidden-stage=    0 wrong-stage-producer=10368 why='R' failed=[] other_stage=['late-maker@implement'] e.g. [optional] kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0
/// story          design        KeyedGap{node}         n= 5184 forbidden-stage=    0 wrong-stage-producer=    0 why='R' failed=['keyed-maker:tags'] other_stage=[] e.g. [optional] kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0
/// story          design        ForbiddenOnly          n= 2592 forbidden-stage= 2592 wrong-stage-producer= 2592 why='requires of base-maker' failed=[] other_stage=['forbidden-maker@operate'] e.g. [optional] kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node+unreal auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0
/// story          design        Chain                  n= 2592 forbidden-stage=    0 wrong-stage-producer=    0 why='requires of base-maker' failed=['Missing:transitive@design', 'chain-maker:tags@design'] other_stage=[] e.g. [optional] kind=story cos=standard criticality=low novelty=low size=low blast_radius=low tc=node+unreal auth=1 data_migration=0 external_dep=1 ui=0 market_facing=0 has_repro=0 flag_exists=1 recent_deploy=0
/// ```
#[test]
fn the_report_branches_fixture_exercises_every_unsat_branch() {
    let inputs = load(&fixture("report-branches"));
    assert_eq!(
        inputs.pack_ids(),
        ["story".to_owned()],
        "the fixture walks one pack, `story`"
    );
    let report = run(&inputs, &Axes::p6_registered());
    assert_eq!(
        report.contexts("pack-default"),
        2592,
        "81 band sets x 4 toolchains x 8 covering rows at pack-default"
    );
    let expected: Vec<String> = [
        format!(
            "story          design        WrongStage             n=10368 forbidden-stage=    0 wrong-stage-producer=10368 why='R' failed=[] other_stage=['late-maker@implement'] e.g. [optional] {V_NODE}"
        ),
        format!(
            "story          design        KeyedGap{{node}}         n= 5184 forbidden-stage=    0 wrong-stage-producer=    0 why='R' failed=['keyed-maker:tags'] other_stage=[] e.g. [optional] {V_NODE}"
        ),
        format!(
            "story          design        ForbiddenOnly          n= 2592 forbidden-stage= 2592 wrong-stage-producer= 2592 why='requires of base-maker' failed=[] other_stage=['forbidden-maker@operate'] e.g. [optional] {V_NU}"
        ),
        format!(
            "story          design        Chain                  n= 2592 forbidden-stage=    0 wrong-stage-producer=    0 why='requires of base-maker' failed=['Missing:transitive@design', 'chain-maker:tags@design'] other_stage=[] e.g. [optional] {V_NU}"
        ),
    ]
    .into_iter()
    .map(|l| format!("{} | {l}", report.tag()))
    .collect();
    let lines = report.lines();
    let head = format!(
        "{} | == unsatisfiable by (pack, stage, kind): count of contexts, then why (first example) ==",
        report.tag()
    );
    let at = lines
        .iter()
        .position(|l| *l == head)
        .unwrap_or_else(|| panic!("the report has no unsat-by-kind section: {lines:#?}"));
    let blank = format!("{} | ", report.tag());
    let section: Vec<String> = lines[at + 1..]
        .iter()
        .take_while(|l| **l != blank)
        .cloned()
        .collect();
    assert!(
        !section.is_empty(),
        "the unsat-by-kind section is empty — no branch was reached"
    );
    println!("report-branches: {} unsat lines", section.len());
    for l in &section {
        println!("report-branches-line: {l}");
    }
    assert_eq!(
        section, expected,
        "the report-branches fixture's unsat lines are not the hand-derived bytes"
    );
}

#[test]
fn family_hole_at_mandatory_fails_totality() {
    let inputs = load(&fixture("family-hole"));
    let v = lattice::validate(&inputs, &Axes::p6_registered()).expect("validate runs");
    let fam = v
        .families
        .iter()
        .find(|f| f.name == "patching")
        .expect("the patching family is checked");
    assert!(
        fam.contexts_checked > 0,
        "the family was checked in zero contexts"
    );
    assert!(
        !fam.holes.is_empty(),
        "AC-104: the hole at mandatory was not found"
    );
    assert!(
        fam.holes.iter().all(|h| h.rigor == Rigor::Mandatory),
        "a hole was found below mandatory: {:?}",
        fam.holes
    );
    let hole = &fam.holes[0];
    assert!(
        hole.vector.contains("size=low"),
        "the hole's facet vector is not named: {}",
        hole.vector
    );
    assert!(
        v.errors.iter().any(|e| e
            .starts_with("family patching: no member admissible at rigor=mandatory for kind=")
            && e.contains("totality")),
        "the totality error does not name the family and the rigor: {:?}",
        v.errors
    );
}
