//! **The source tables `[checklist.*]`, `[kb.*]` and `[feed.*.*]`** — s8c §8c.4, record V =
//! ADR-0170 decision 8, proposed (D21-43 N14).
//!
//! Each entry declares exactly one of two forms, refused at LOAD naming the entry otherwise. At
//! resolution an **undeclared** key is refused by name even when the input is optional (M18), and
//! an `unauthored` / `unprovided` key is refused when mandatory and `null` when optional (ADR-0032:
//! *"Absent `?` inputs are `null`"*). The registries here are test-only; the shipped census is
//! `source_registry::the_catalog_source_census_is_fifty_with_fourteen_declared`'s.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rk_kernel::toml_subset::Value as TomlValue;
use rk_kernel::workflow::memo::{SourceRefusal, SourceValue, VerbRegistry, VerbRegistryError};

const VERBS: &str = "[verb]\nbuild = { outputs = \"BuildReport\", live = false, timeout = \
                     \"build_timeout\", inputs = [], route = \"port\", side_effecting = false }\n";

/// A registry over a temp `.redkiln/` holding one checklist and one atom.
fn loaded(tables: &str) -> (tempfile::TempDir, Result<VerbRegistry, VerbRegistryError>) {
    let dir = tempfile::tempdir().expect("a temp dir");
    std::fs::create_dir_all(dir.path().join("checklists")).expect("mkdir");
    std::fs::create_dir_all(dir.path().join("kb")).expect("mkdir");
    std::fs::write(
        dir.path().join("checklists/closure.toml"),
        "source = \"test\"\n[[item]]\nid = \"a\"\ntext = \"the closure is recorded\"\n",
    )
    .expect("write");
    std::fs::write(
        dir.path().join("kb/spec-conventions.md"),
        "# Spec conventions\n",
    )
    .expect("write");
    std::fs::write(
        dir.path().join("checklists/empty.toml"),
        "source = \"test\"\n",
    )
    .expect("write");
    let src = format!("{VERBS}{tables}");
    let reg = VerbRegistry::from_toml_at(&src, dir.path());
    (dir, reg)
}

const DECLARED: &str = "[checklist.closure]\npath = \"checklists/closure.toml\"\n\
                        [checklist.smoke]\nunauthored = \"s8g §8g.6 authors it\"\n\
                        [kb.spec-conventions]\npath = \"kb/spec-conventions.md\"\n\
                        [feed.t.backlog]\nunprovided = \"s8f builds the store read\"\n\
                        [feed.t.logs]\nunprovided = \"no log source\"\n\
                        [feed.t.sbom]\nverb = \"build\"\n";

fn shipped_shape() -> VerbRegistry {
    let (_dir, reg) = loaded(DECLARED);
    reg.expect("the declared tables load")
}

/// **M18: an undeclared OPTIONAL key resolving to `null` turns this red.**
#[test]
fn an_undeclared_checklist_kb_or_feed_key_is_refused_by_name() {
    let reg = shipped_shape();
    // CONTROL: each form's DECLARED key resolves, so a refusal below is about declaration.
    assert_eq!(
        reg.resolve_source("t", "checklist:closure"),
        Ok(SourceValue::Path("checklists/closure.toml".to_owned()))
    );
    assert_eq!(
        reg.resolve_source("t", "kb:spec-conventions?"),
        Ok(SourceValue::Path("kb/spec-conventions.md".to_owned()))
    );
    assert_eq!(
        reg.resolve_source("t", "toolchain:sbom?"),
        Ok(SourceValue::FeedVerb("build".to_owned()))
    );

    for (input, key) in [
        ("checklist:risk-register", "checklist:risk-register"),
        ("checklist:risk-register?", "checklist:risk-register"),
        ("kb:estimation-history", "kb:estimation-history"),
        ("kb:estimation-history?", "kb:estimation-history"),
        ("toolchain:vcs_history", "feed.t.vcs_history"),
        ("toolchain:vcs_history?", "feed.t.vcs_history"),
    ] {
        let got = reg.resolve_source("t", input);
        assert_eq!(
            got,
            Err(SourceRefusal::Undeclared {
                key: key.to_owned()
            }),
            "`{input}` is undeclared and must refuse by name, optional or not — an undeclared key \
             is a typo or an unowned demand, never an absence (N14)"
        );
        let rendered = got.expect_err("refused").to_string();
        assert!(
            rendered.contains(key),
            "the refusal does not name `{key}`: {rendered}"
        );
    }
    // A feed declared for ANOTHER toolchain is undeclared for this one.
    assert_eq!(
        reg.resolve_source("other", "toolchain:logs?"),
        Err(SourceRefusal::Undeclared {
            key: "feed.other.logs".to_owned()
        })
    );
}

#[test]
fn an_unprovided_feed_is_null_when_optional_and_refused_when_mandatory() {
    let reg = shipped_shape();
    assert_eq!(
        reg.resolve_source("t", "toolchain:logs?"),
        Ok(SourceValue::Null),
        "an optional `unprovided` feed is null (ADR-0032)"
    );
    let e = reg
        .resolve_source("t", "toolchain:backlog")
        .expect_err("a mandatory unprovided feed is refused");
    assert_eq!(
        e,
        SourceRefusal::Unprovided {
            key: "feed.t.backlog".to_owned(),
            reason: "s8f builds the store read".to_owned()
        }
    );
    assert!(e.to_string().contains("feed.t.backlog") && e.to_string().contains("s8f"));
    // The same rule for an `unauthored` checklist.
    assert_eq!(
        reg.resolve_source("t", "checklist:smoke?"),
        Ok(SourceValue::Null)
    );
    assert_eq!(
        reg.resolve_source("t", "checklist:smoke"),
        Err(SourceRefusal::Unauthored {
            key: "checklist:smoke".to_owned(),
            reason: "s8g §8g.6 authors it".to_owned()
        })
    );
    // CONTROL: a PROVIDED optional feed is not null — the null above is the entry's, not `?`'s.
    assert_eq!(
        reg.resolve_source("t", "toolchain:sbom?"),
        Ok(SourceValue::FeedVerb("build".to_owned()))
    );
}

#[test]
fn a_malformed_entry_is_refused_at_load_naming_it() {
    // CONTROL: the well-formed tables load.
    assert!(
        loaded(DECLARED).1.is_ok(),
        "CONTROL: the declared tables did not load"
    );

    for (tables, entry, why) in [
        (
            "[checklist.ghost]\npath = \"checklists/ghost.toml\"\n",
            "checklist.ghost",
            "does not exist",
        ),
        (
            "[checklist.empty]\npath = \"checklists/empty.toml\"\n",
            "checklist.empty",
            "holds no item",
        ),
        (
            "[kb.x]\npath = \"kb/spec-conventions.md\"\nunauthored = \"both\"\n",
            "kb.x",
            "BOTH",
        ),
        ("[kb.y]\n", "kb.y", "NEITHER"),
        (
            "[feed.t.x]\nverb = \"build\"\nunprovided = \"both\"\n",
            "feed.t.x",
            "BOTH",
        ),
        ("[feed.t.z]\n", "feed.t.z", "NEITHER"),
        (
            "[feed.t.w]\nverb = \"no_such_verb\"\n",
            "feed.t.w",
            "not a registry row",
        ),
        (
            "[checklist.up]\npath = \"../outside.toml\"\n",
            "checklist.up",
            "plain relative path",
        ),
    ] {
        let (_dir, got) = loaded(tables);
        match got {
            Err(VerbRegistryError::Source { entry: e, detail }) => {
                assert_eq!(e, entry, "the refusal names the wrong entry: {detail}");
                assert!(
                    detail.contains(why),
                    "`{entry}` refused for another reason: {detail}"
                );
            }
            other => panic!("`{entry}` ({why}) was not refused at load: {other:?}"),
        }
    }

    // A `path` entry read without the `.redkiln/` it is relative to is refused, not skipped.
    let src = format!("{VERBS}[checklist.closure]\npath = \"checklists/closure.toml\"\n");
    assert!(
        matches!(
            VerbRegistry::from_toml(&src),
            Err(VerbRegistryError::Source { ref entry, .. }) if entry == "checklist.closure"
        ),
        "a path entry loaded with nothing to check it against"
    );
}

// ------------------------------------------------------------------------------------------------
// THE SHIPPED SOURCE TABLES (s8c §8c.4 "Which keys, measured"; P8).
// ------------------------------------------------------------------------------------------------

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// The lane-reached keys §8c.4's table declares, each with what it resolves to on a MANDATORY
/// read: `Ok(Path)` for an authored file, or the refusal an `unauthored` / `unprovided` entry
/// gives.
const DECLARED_CHECKLISTS: [&str; 7] = [
    "closure",
    "docs-diataxis",
    "fix-scope",
    "gherkin-quality",
    "retro",
    "risk-register",
    "smoke",
];
const DECLARED_ATOMS: [&str; 4] = [
    "estimation-history",
    "refactoring-catalog",
    "retro-format",
    "spec-conventions",
];
const DECLARED_FEEDS: [&str; 3] = ["backlog", "logs", "sbom"];

/// Every `checklist:` / `kb:` / `toolchain:<feed>` value in a step's `inputs`, walked through the
/// kernel's own TOML subset: `(spelling, activity/step)`.
fn walked_inputs(dir: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("the catalog directory")
        .map(|e| e.expect("an entry").path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("toml"))
        .collect();
    files.sort();
    assert_eq!(files.len(), 102, "premise: the shipped catalog's 102 files");
    for path in files {
        let text = std::fs::read_to_string(&path).expect("a catalog file");
        let doc = rk_kernel::toml_subset::parse(&text)
            .unwrap_or_else(|e| panic!("{} does not parse: {e:?}", path.display()));
        let activity = doc
            .get("activity")
            .and_then(TomlValue::as_table)
            .and_then(|a| a.get("id"))
            .and_then(TomlValue::as_str)
            .expect("an activity id")
            .to_owned();
        for step in doc
            .get("step")
            .and_then(TomlValue::as_array)
            .unwrap_or_default()
        {
            let step = step.as_table().expect("a step table");
            let id = step.get("id").and_then(TomlValue::as_str).unwrap_or("?");
            let Some(inputs) = step.get("inputs").and_then(TomlValue::as_table) else {
                continue;
            };
            for value in inputs.values() {
                if let Some(v) = value.as_str()
                    && ["checklist:", "kb:", "toolchain:"]
                        .iter()
                        .any(|p| v.starts_with(p))
                {
                    out.push((v.to_owned(), format!("{activity}/{id}")));
                }
            }
        }
    }
    out
}

/// The same census from the catalog's TEXT: occurrences of a quoted source prefix on `inputs`
/// lines.
fn text_census(dir: &Path) -> usize {
    let mut n = 0;
    for entry in std::fs::read_dir(dir).expect("the catalog directory") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a catalog file");
        for line in text
            .lines()
            .filter(|l| l.trim_start().starts_with("inputs"))
        {
            n += ["\"checklist:", "\"kb:", "\"toolchain:"]
                .iter()
                .map(|p| line.matches(p).count())
                .sum::<usize>();
        }
    }
    n
}

#[test]
fn the_catalog_source_census_is_fifty_with_fourteen_declared() {
    let root = workspace_root();
    let catalog = root.join(".redkiln").join("catalog");
    let walked = walked_inputs(&catalog);
    let census = text_census(&catalog);
    assert!(!walked.is_empty(), "the walk found no source input");
    assert_eq!(
        walked.len(),
        census,
        "the walk read a different number of source inputs than the catalog's text holds"
    );
    let spellings: BTreeSet<&str> = walked.iter().map(|(v, _)| v.as_str()).collect();
    let keys: BTreeSet<&str> = spellings.iter().map(|s| s.trim_end_matches('?')).collect();
    let count = |prefix: &str| keys.iter().filter(|k| k.starts_with(prefix)).count();
    println!(
        "source census: inputs={} spellings={} keys={} (checklist={} kb={} toolchain={})",
        walked.len(),
        spellings.len(),
        keys.len(),
        count("checklist:"),
        count("kb:"),
        count("toolchain:")
    );
    assert_eq!(keys.len(), 50, "§8c.4: 50 distinct keys");
    assert_eq!(count("checklist:"), 27);
    assert_eq!(count("kb:"), 12);
    assert_eq!(count("toolchain:"), 11);
    assert_eq!(
        spellings.len(),
        51,
        "12 toolchain spellings: `vcs_history` occurs bare and optional"
    );

    let reg = VerbRegistry::load_file(&root.join(".redkiln").join("registry.toml"))
        .expect("the shipped registry loads, its declared paths checked");
    let mut expected: BTreeSet<String> = DECLARED_CHECKLISTS
        .iter()
        .map(|k| format!("checklist:{k}"))
        .collect();
    expected.extend(DECLARED_ATOMS.iter().map(|k| format!("kb:{k}")));
    expected.extend(DECLARED_FEEDS.iter().map(|k| format!("toolchain:{k}")));
    assert_eq!(expected.len(), 14);

    for toolchain in ["rust", "node"] {
        let mut declared = BTreeSet::new();
        for key in &keys {
            match reg.resolve_source(toolchain, key) {
                Err(SourceRefusal::Undeclared { key: named }) => {
                    let spelled = key.strip_prefix("toolchain:").map_or_else(
                        || (*key).to_owned(),
                        |feed| format!("feed.{toolchain}.{feed}"),
                    );
                    assert_eq!(named, spelled, "the refusal names the key");
                }
                Err(SourceRefusal::NotASource { input }) => panic!("`{input}` is not a source"),
                Ok(_)
                | Err(SourceRefusal::Unauthored { .. } | SourceRefusal::Unprovided { .. }) => {
                    declared.insert((*key).to_owned());
                }
            }
        }
        assert_eq!(
            declared, expected,
            "[{toolchain}] the declared keys are exactly §8c.4's fourteen lane-reached keys; the \
             other 36 refuse by name"
        );
        assert_eq!(keys.len() - declared.len(), 36);
        // Each lane-reached key is reached by at least one catalog input.
        for key in &expected {
            assert!(
                walked.iter().any(|(v, _)| v.trim_end_matches('?') == key),
                "`{key}` is declared but no catalog input reaches it"
            );
        }
        // The feeds: ~~backlog is mandatory where the lane reads it and refuses;~~ logs? and sbom?
        // are optional and null.
        // *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-46 R7; D21-106
        // ruling 5.)* `backlog` is produced by the `backlog` verb (`s8f` §8f.7), so it resolves to
        // its producing verb, and the census's declared-key count is unchanged. Was:
        // `Err(SourceRefusal::Unprovided { .. })`.
        assert_eq!(
            reg.resolve_source(toolchain, "toolchain:backlog"),
            Ok(SourceValue::FeedVerb("backlog".to_owned()))
        );
        for optional in ["toolchain:logs?", "toolchain:sbom?"] {
            assert_eq!(
                reg.resolve_source(toolchain, optional),
                Ok(SourceValue::Null)
            );
        }
    }
    // The authored files resolve to their paths; `smoke` is `unauthored`, naming s8g §8g.6.
    for id in DECLARED_CHECKLISTS.iter().filter(|k| **k != "smoke") {
        assert_eq!(
            reg.resolve_source("rust", &format!("checklist:{id}")),
            Ok(SourceValue::Path(format!("checklists/{id}.toml")))
        );
    }
    for atom in DECLARED_ATOMS {
        assert_eq!(
            reg.resolve_source("rust", &format!("kb:{atom}")),
            Ok(SourceValue::Path(format!("kb/{atom}.md")))
        );
    }
    match reg.resolve_source("rust", "checklist:smoke") {
        Err(SourceRefusal::Unauthored { reason, .. }) => assert!(
            reason.contains("s8g") && reason.contains("8g.6"),
            "`checklist:smoke` is unauthored naming s8g §8g.6: {reason}"
        ),
        other => panic!("`checklist:smoke` must be unauthored at s8c: {other:?}"),
    }
    // Every authored file names its provenance: a checklist's top-level `source`, an atom's
    // `source:` line.
    for id in DECLARED_CHECKLISTS.iter().filter(|k| **k != "smoke") {
        let text = std::fs::read_to_string(
            root.join(".redkiln")
                .join("checklists")
                .join(format!("{id}.toml")),
        )
        .expect("the checklist");
        let doc = rk_kernel::toml_subset::parse(&text).expect("the checklist parses");
        let source = doc.get("source").and_then(TomlValue::as_str).unwrap_or("");
        assert!(
            !source.trim().is_empty(),
            "checklists/{id}.toml names no source"
        );
        let items: BTreeMap<&str, &str> = doc
            .get("item")
            .and_then(TomlValue::as_array)
            .unwrap_or_default()
            .iter()
            .map(|i| {
                let t = i.as_table().expect("an item table");
                (
                    t.get("id").and_then(TomlValue::as_str).expect("an item id"),
                    t.get("text")
                        .and_then(TomlValue::as_str)
                        .expect("an item text"),
                )
            })
            .collect();
        assert!(!items.is_empty(), "checklists/{id}.toml holds no item");
    }
    for atom in DECLARED_ATOMS {
        let text =
            std::fs::read_to_string(root.join(".redkiln").join("kb").join(format!("{atom}.md")))
                .expect("the atom");
        assert!(
            text.lines().any(|l| l.starts_with("source:")),
            "kb/{atom}.md names no `source:`"
        );
    }
}
