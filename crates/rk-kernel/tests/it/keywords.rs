//! **AC-436 — a catalog is findable without becoming selectable by the same words** (Step 7d
//! Slice 3, the third of §7d.1's eight kernel edits).
//!
//! Two halves, and they are different kinds of assertion. The **search** half says `keywords` does
//! something: `Catalog::search` finds by it. The **lint** half says `keywords` does nothing else:
//! the admissibility evaluator cannot see it. A field that only did the first would let a search
//! term quietly steer what runs, which is the coupling this separation exists to break.

use rk_kernel::select::{Catalog, Item, Pack};

/// Source lines that carry CODE — comment lines dropped. The same filter Checkpoint 7d's shell
/// helpers apply, so the Rust scan and the block's scan agree on what a comment is.
fn code_lines(src: &str) -> Vec<(usize, String)> {
    src.lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l))
        .filter(|(_, l)| {
            let t = l.trim_start();
            !(t.starts_with("//") || t.starts_with("/*") || t.starts_with('*') || t.is_empty())
        })
        .map(|(n, l)| (n, l.to_owned()))
        .collect()
}

/// Every `.rs` file under `root`, recursively, sorted. Panics on an absent root rather than
/// returning nothing: a scan over a missing directory reports a pass having read nothing.
fn rs_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    assert!(
        root.is_dir(),
        "the scan root {} does not exist — an absence assertion over it would read nothing",
        root.display()
    );
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("a readable dir") {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    out.sort();
    out
}

fn kernel_src() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// AC-436's lint half: **`keywords` is unreachable from the admissibility evaluator.**
///
/// Scanned over `select/` and `predicate/` — the two module trees that decide what is admissible —
/// for the identifier, on code lines only.
///
/// **The comment filter is load-bearing here, and not as a formality.** `predicate/lexer.rs` carries
/// the line *"`[a-zA-Z_][a-zA-Z0-9_]*`, keywords included"* — the word in its PROGRAMMING-LANGUAGE
/// sense, about the expression grammar's reserved words, with no relation to a catalog's search
/// terms. An unfiltered grep would fire on it and the ban would have to be weakened or deleted;
/// filtered, it reads what it means to read. The control below is the other direction: the scan is
/// shown to FIND the identifier where it legitimately lives.
///
/// Declared mutant (§7d.14): an evaluator clause reading `keywords`. Watched red on it.
#[test]
fn ac436_keywords_unreachable_from_admissibility_evaluator() {
    let src = kernel_src();

    // The control. `catalog.rs` declares and reads the field, so the scan must find it there —
    // otherwise a scanner that matched nothing would report both files clean and prove nothing.
    let catalog_rs = src.join("catalog.rs");
    let catalog_text = std::fs::read_to_string(&catalog_rs).expect("catalog.rs is readable");
    let control_hits = code_lines(&catalog_text)
        .into_iter()
        .filter(|(_, l)| l.contains("keywords"))
        .count();
    assert!(
        control_hits >= 1,
        "the scan found no NON-COMMENT `keywords` in {} — it is looking for something the field's \
         own module does not contain, so its silence over the evaluator means nothing",
        catalog_rs.display()
    );

    // **The module ROOTS are in scope, not just the directories**, and this is not a detail: a
    // scan over `select/` alone misses `select.rs`, which is where `select()` itself lives. Caught
    // by the declared mutant — an evaluator clause added to `select.rs` was NOT caught by the first
    // version of this test. Checkpoint 7d's own ban filters on `/([^/]*/)?(select|predicate)(\.rs|/)`,
    // which includes both spellings; this now matches it rather than being narrower than the shell
    // line it is supposed to be the in-process twin of.
    let mut targets: Vec<std::path::PathBuf> = Vec::new();
    for name in ["select", "predicate"] {
        let root = src.join(name);
        if root.is_dir() {
            targets.extend(rs_files(&root));
        }
        let module_root = src.join(format!("{name}.rs"));
        if module_root.is_file() {
            targets.push(module_root);
        }
    }
    assert!(
        targets.len() >= 3,
        "the evaluator scan found only {} files, which is fewer than the two module roots plus \
         their trees — it is reading less than the evaluator and its silence would mean nothing",
        targets.len()
    );

    let mut hits = Vec::new();
    for file in targets {
        let text = std::fs::read_to_string(&file).expect("a source file");
        for (line_no, line) in code_lines(&text) {
            if line.contains("keywords") {
                hits.push(format!("{}:{line_no}: {}", file.display(), line.trim()));
            }
        }
    }
    assert!(
        hits.is_empty(),
        "the admissibility evaluator reads `keywords`. AC-436 separates finding from selecting: a \
         search term that steered selection would make `catalog search` a way to change what \
         runs:\n{}",
        hits.join("\n")
    );
}

/// AC-436's search half and its permutation property, as the register's Verified-by names them:
/// *"a property test that permutes `keywords` across the shipped catalog and asserts an unchanged
/// selection … plus a positive control: a `redkiln catalog search` for a keyword finds the activity
/// that declares it and misses one that does not."*
///
/// **The register also records why this test is written the way it is, and it is a warning about
/// this exact test**: *"`keywords` before selection saw it, so the permutation test passed by
/// construction and the unreachability lint was vacuous — a check that cannot fail."* Two things
/// follow, and both are here:
///
/// 1. the selection fixtures declare **no** keywords, so permuting theirs would mutate nothing.
///    The property INJECTS generated keyword lists into every activity and permutes those;
/// 2. the injection is **asserted to have landed** before any equality is claimed. Without that,
///    a broken rewrite makes every repetition identical and the property passes having varied
///    nothing — which is the failure the register describes, reproduced.
#[test]
fn ac436_keywords_permutation_leaves_selection_unchanged() {
    // ---- the positive control: search finds what declares the term, and misses what does not.
    let declares = concat!(
        "[activity]\n",
        "id = \"draft-spec\"\n",
        "stage = \"analysis\"\n",
        "provides = [\"Spec\"]\n",
        "method_rank = 10\n",
        "keywords = [\"scaffolding\", \"outline\"]\n",
    );
    let declares_not = concat!(
        "[activity]\n",
        "id = \"review-spec\"\n",
        "stage = \"analysis\"\n",
        "provides = [\"Review\"]\n",
        "method_rank = 20\n",
    );

    let two = Catalog::from_sources(&[
        ("draft-spec.toml", declares),
        ("review-spec.toml", declares_not),
    ])
    .expect("the two-activity catalog loads");

    assert_eq!(
        two.activities()
            .iter()
            .find(|a| a.id == "draft-spec")
            .expect("the declaring activity loaded")
            .keywords,
        vec!["scaffolding".to_owned(), "outline".to_owned()],
        "the loader did not read `keywords`, so every assertion below would be about a field \
         nothing populates — the state the register records as having made this test vacuous"
    );

    let found = two.search("scaffolding");
    assert_eq!(
        found.len(),
        1,
        "the keyword finds exactly the activity that declares it"
    );
    assert_eq!(found[0].id, "draft-spec");
    assert_eq!(
        two.search("SCAFFOLDING").len(),
        1,
        "matching is case-insensitive on both sides"
    );
    assert!(
        two.search("draft-spec").is_empty(),
        "search reads `keywords` and NOT `id` — matching the id would reintroduce, through the \
         back door, the coupling AC-436 separates"
    );
    assert!(
        two.search("scaffold").is_empty(),
        "matching is on WHOLE keywords. RAISED: the shipped catalog declares PHRASES (`git \
         bisect`, `culprit commit`), so an operator searching `bisect` finds nothing. AC-436 says \
         only that a search for a keyword finds the activity that declares it, and inventing \
         word-within-phrase matching here would be a semantics no record authorises"
    );

    // ---- the permutation property, over the SHIPPED selection catalog.
    let registry = crate::selection::registry();
    let facets = crate::selection::facets();
    let item = Item::new("STORY-1", "story").with_toolchain("node", ["text"]);
    let pack = Pack::from_toml("incident-forbidden.toml", crate::selection::INCIDENT)
        .expect("the pack loads");

    let baseline = {
        let catalog = Catalog::from_sources(crate::selection::CATALOG).expect("the catalog loads");
        let selection =
            rk_kernel::select::select(&pack, &catalog, &registry, &facets, "implement", &item)
                .expect("selection resolves");
        selection
            .canonical_bytes()
            .expect("the selection serialises")
    };

    for rep in 0..32_usize {
        // A deterministic permutation seeded by the repetition index: no clock and no random
        // source reaches this crate, so the shuffle is arithmetic — `selection_seven_steps_are_pure`
        // sets that precedent and this follows it.
        let injected: Vec<(String, String)> = crate::selection::CATALOG
            .iter()
            .enumerate()
            .map(|(i, (name, src))| {
                let terms: Vec<String> = (0..=(rep % 3))
                    .map(|k| format!("\"term-{}-{}\"", (i + rep + k) % 7, k))
                    .collect();
                let line = format!("[activity]\nkeywords = [{}]", terms.join(", "));
                ((*name).to_owned(), src.replacen("[activity]", &line, 1))
            })
            .collect();
        let sources: Vec<(&str, &str)> = injected
            .iter()
            .map(|(n, s)| (n.as_str(), s.as_str()))
            .collect();
        let catalog = Catalog::from_sources(&sources).expect("the injected catalog loads");

        // THE CONTROL: the injection reached the LOADED catalog. Without it the loop varies
        // nothing and the equality below is true of a rewrite that silently did nothing — which is
        // exactly the failure AC-436's own note records.
        assert!(
            catalog.activities().iter().all(|a| !a.keywords.is_empty()),
            "repetition {rep}: the keyword injection did not reach the loaded catalog, so this \
             property would pass having permuted nothing"
        );

        let selection =
            rk_kernel::select::select(&pack, &catalog, &registry, &facets, "implement", &item)
                .expect("selection resolves");
        let bytes = selection.canonical_bytes().expect("serialises");
        assert_eq!(
            bytes, baseline,
            "repetition {rep}: permuting `keywords` moved the selection. They are engine-ignored \
             (AC-436): the selected activity set is byte-identical under the canonical serialiser"
        );
    }
}
