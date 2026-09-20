//! **`catalog search`** — AC-436's positive control (RA-M-02, D18-15).
//!
//! The lint half of AC-436 (*"`keywords` is unreachable from the admissibility evaluator"*) is
//! `rk-kernel`'s and landed at cycle 1 as `ac436_keywords_unreachable_from_admissibility_evaluator`;
//! this is the CLI half and asserts the search finds what the field declares.

use std::path::{Path, PathBuf};

use rk_kernel::catalog::Catalog;

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
