//! **`catalog search`** — AC-436's positive control (RA-M-02, D18-15).
//!
//! The lint half of AC-436 (*"`keywords` is unreachable from the admissibility evaluator"*) is
//! `rk-kernel`'s and landed at cycle 1 as `ac436_keywords_unreachable_from_admissibility_evaluator`;
//! this is the CLI half and asserts the search finds what the field declares.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Catalog, InputValue, StepKind};

use rk_cli::verbs::catalog;

/// The synthetic catalog directory this module searches.
fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("catalog")
}

/// The same document with every `keywords = [...]` row removed — *"the synthetic catalog's
/// serialised bytes minus `keywords`"*, which is what the discriminating assertion is made over.
fn without_keywords(src: &str) -> String {
    src.lines()
        .filter(|l| !l.trim_start().starts_with("keywords"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// **AC-436, the search half.** §7d.7 fixes three assertions, verbatim: *"a search for a keyword one
/// activity carries returns exactly that id; a search for one no activity carries returns nothing
/// and exit `0`; **the keyword is a token that appears in no other field of any activity** — asserted
/// by the test over the synthetic catalog's serialised bytes minus `keywords` — so a search that
/// greps ids or descriptions instead of `keywords` is red too"*.
///
/// **The third is the discriminating one and it is asserted FIRST.** A search that matched on `id`
/// or `tags` would still return the right activity if the query happened to appear there; the only
/// way to know the match came from `keywords` is to know the token is nowhere else. So this computes
/// the fixture's bytes with the `keywords` rows stripped and asserts the token is absent from what
/// remains **before** searching anything.
///
/// §7d.14's red: *"by a search over a catalog whose one activity carries the keyword returning
/// nothing — the tree's field-dropping loader today"*.
#[test]
fn ac436_catalog_search_finds_by_keyword() {
    let dir = fixture_dir();
    let catalog = Catalog::from_dir(&dir).expect("the synthetic catalog loads");
    assert!(
        !catalog.is_empty(),
        "PREMISE: the synthetic catalog is empty, so every assertion below would be over nothing"
    );

    // ---- (3) FIRST: the token appears in no other field of any activity. ------------------------
    let needle = "zymurgy";
    for entry in std::fs::read_dir(&dir).expect("the fixture directory reads") {
        let path = entry.expect("an entry").path();
        if path.extension().is_none_or(|e| e != "toml") {
            continue;
        }
        let src = std::fs::read_to_string(&path).expect("the fixture is readable");
        // The control on this assertion: the token IS in the file, so a stripper that removed the
        // whole document would make the absence below true for the wrong reason.
        assert!(
            src.contains(needle),
            "PREMISE: {} does not carry `{needle}` at all",
            path.display()
        );
        assert!(
            !without_keywords(&src).contains(needle),
            "`{needle}` appears in {} OUTSIDE its `keywords` rows. AC-436's third assertion is that \
             the token is a keyword and nothing else — without it, a `search` that greps ids or \
             descriptions would return the right answer for the wrong reason",
            path.display()
        );
    }

    // ---- (1) a keyword one activity carries returns exactly that id. ---------------------------
    let hits: Vec<&str> = catalog
        .search(needle)
        .iter()
        .map(|a| a.id.as_str())
        .collect();
    assert_eq!(
        hits,
        vec!["brew-report"],
        "a search for a declared keyword did not return exactly the activity that declares it"
    );

    // The second declared keyword, so the assertion above is not satisfied by a search that returns
    // its one activity for every non-empty query.
    assert_eq!(
        catalog
            .search("fermentation")
            .iter()
            .map(|a| a.id.as_str())
            .collect::<Vec<_>>(),
        vec!["brew-report"]
    );

    // ---- (2) a keyword no activity carries returns nothing, and is not an error. ---------------
    assert!(
        catalog.search("nothing-here").is_empty(),
        "a search for an undeclared keyword returned something"
    );
    let mut out: Vec<u8> = Vec::new();
    let exit = catalog::search(&catalog, "nothing-here", &mut out);
    assert_eq!(
        exit.code(),
        0,
        "AC-436: a search that finds nothing is not an error"
    );
    assert!(
        out.is_empty(),
        "a search that found nothing printed something: {}",
        String::from_utf8_lossy(&out)
    );

    // And the matching case prints the id — the half that makes the exit-0 above a statement about
    // a search rather than about a verb that prints nothing whatever it is asked.
    let mut matched: Vec<u8> = Vec::new();
    catalog::search(&catalog, needle, &mut matched);
    assert_eq!(String::from_utf8_lossy(&matched).trim(), "brew-report");
}

/// The known violators of the read-only-role rule, each `(activity, step)`: an investigator
/// output that a `toolchain:test` selection reads (ADR-0204, proposed; plan Decision D13). No lane
/// selects either. They are tracked by the issue owed in ADR-0204 § "Owed issue (D13)"; when one
/// is fixed it leaves this list, because the assertion is exact.
const KNOWN_READ_ONLY_SELECTIONS: [(&str, &str); 2] = [
    ("differential-debugging", "confirm"),
    ("instrumented-trace-analysis", "confirm"),
];

/// The tools a role's card grants, read from `.redkiln/cards/<role>.json`.
#[cfg(test)]
fn card_tools(root: &Path, role: &str) -> BTreeSet<String> {
    let path = root
        .join(".redkiln")
        .join("cards")
        .join(format!("{role}.json"));
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("the card {} reads: {e}", path.display()));
    let card: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("the card {} parses: {e}", path.display()));
    card.get("tools")
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| panic!("the card {} has a `tools` array", path.display()))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .map(str::to_owned)
        .collect()
}

/// **No `toolchain:test` selection reads the output of a step whose role cannot write** (ADR-0204,
/// proposed; added 2026-11-08, s8 repro-pair route). The 2026-11-08 relaunch stopped because
/// `minimal-repro@1`'s `confirm` ran the investigator's prose: a read-only role (Read, Glob, Grep,
/// StructuredOutput) can neither write a test nor run a command, so a selection fed by its output
/// names prose or a test that does not exist. For every activity and every `toolchain:test` step
/// whose `inputs.selection` reads `${<step>.outputs…}`, the source step is a tool step, or an agent
/// step whose role card grants `Write` or `Edit`. The violators are exactly the two known ones.
#[test]
fn no_test_selection_reads_a_read_only_roles_output() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut walked = 0_usize;
    let mut violators = BTreeSet::new();
    for dir in ["catalog", "catalog-extended"] {
        let catalog = Catalog::from_dir(&root.join(".redkiln").join(dir))
            .unwrap_or_else(|e| panic!("the shipped {dir} loads: {e}"));
        for activity in catalog.activities() {
            for step in &activity.steps {
                if step.run.as_deref() != Some("toolchain:test") {
                    continue;
                }
                walked += 1;
                let Some(InputValue::Text(selection)) = step.inputs.get("selection") else {
                    continue;
                };
                let Some(source) = selection
                    .strip_prefix("${")
                    .and_then(|rest| rest.split_once(".outputs"))
                    .map(|(source, _)| source)
                else {
                    continue;
                };
                let source_step = activity
                    .steps
                    .iter()
                    .find(|s| s.id == source)
                    .unwrap_or_else(|| {
                        panic!("{}: `{source}` is a step of the activity", activity.id)
                    });
                if source_step.kind != StepKind::Agent {
                    continue;
                }
                let role = source_step.role.as_deref().unwrap_or_else(|| {
                    panic!("{}: the agent step `{source}` has a role", activity.id)
                });
                let tools = card_tools(&root, role);
                if !tools.contains("Write") && !tools.contains("Edit") {
                    violators.insert((activity.id.clone(), step.id.clone()));
                }
            }
        }
    }
    assert!(
        walked > 10,
        "PREMISE: the walk met the shipped `toolchain:test` steps, not a handful: {walked}"
    );
    let known: BTreeSet<(String, String)> = KNOWN_READ_ONLY_SELECTIONS
        .iter()
        .map(|(a, s)| ((*a).to_owned(), (*s).to_owned()))
        .collect();
    assert_eq!(
        violators, known,
        "a `toolchain:test` selection reads a read-only role's output (ADR-0204; the 2026-11-08 \
         rc 14): the source step's card grants neither Write nor Edit"
    );
}
