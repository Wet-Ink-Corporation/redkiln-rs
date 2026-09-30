//! **The verb registry and its per-toolchain value tables** — s8c §8c.4-§8c.5, record V =
//! ADR-0170 decisions 1, 9 and 10, proposed.
//!
//! The shipped `.redkiln/registry.toml` is proved by the `the_shipped_…`, `ac018_holds_…` and
//! `every_shipped_…` tests, written with the file. This module's first test is over test-only
//! tables: **the loader refuses a value without its warrant** (D21-42 4a: *"Each value carries its
//! measurement as its warrant"*).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use rk_kernel::catalog::{Catalog, StepKind, StepTimeout};
use rk_kernel::toml_subset::Value as TomlValue;
use rk_kernel::workflow::memo::{
    PortMethod, Route, ToolchainValue, ValueRefusal, VerbRegistry, VerbRegistryError,
    live_verb_memo_rule, verb_of,
};

const PIN: &str = "0123456789abcdef0123456789abcdef01234567";
const VERBS: &str = "[verb]\nbuild = { outputs = \"BuildReport\", live = false, timeout = \
                     \"build_timeout\", inputs = [], route = \"port\", side_effecting = false }\n";

fn load(entry: &str) -> Result<VerbRegistry, VerbRegistryError> {
    VerbRegistry::from_toml(&format!("{VERBS}[toolchain.t]\nk = {entry}\n"))
}

#[test]
fn a_value_without_a_warrant_is_refused() {
    // ---- CONTROLS: a measured value, a cited value, an `unmeasured` key and a declaration row
    // each load, so every refusal below is about the warrant.
    let measured = format!(
        "{{ secs = 180, measured = [61.5, 90, 72], rule = \"twice the slowest, up to the minute\", \
         pin = \"{PIN}\", command = \"cargo build --workspace\" }}"
    );
    let reg = load(&measured).expect("CONTROL: a measured value with its warrant loads");
    assert_eq!(
        reg.values_for("t")
            .expect("the table")
            .resolve("k")
            .map(std::num::NonZeroU64::get),
        Ok(180)
    );
    let cited = format!(
        "{{ secs = 4800, measured = [4561], rule = \"re-derived from the class maximum\", pin = \
         \"{PIN}\", source = \"F31/S-TYPED, borrowed class\" }}"
    );
    load(&cited)
        .expect("CONTROL: a cited value with its warrant loads (the rule is not recomputed)");
    let reg = load("{ unmeasured = \"no Phase-1 verb deploys\" }").expect("CONTROL: unmeasured");
    assert_eq!(
        reg.values_for("t").expect("the table").entries().get("k"),
        Some(&ToolchainValue::Unmeasured {
            reason: "no Phase-1 verb deploys".to_owned()
        })
    );
    VerbRegistry::from_toml(&format!(
        "{VERBS}[toolchain.t]\nsetup_projects = {{ value = [\"p\"], source = \"git ls-files\" }}\n"
    ))
    .expect("CONTROL: a declaration row from the named set loads");

    // ---- THE REFUSALS, each naming `toolchain.t.k`.
    let refusals = [
        ("600".to_owned(), "a bare value"),
        ("{ secs = 600 }".to_owned(), "secs alone"),
        (
            "{ secs = 180, measured = [90], rule = \"r\", command = \"c\" }".to_owned(),
            "no pin",
        ),
        (
            "{ secs = 180, measured = [90], rule = \"r\", pin = \"abc\", command = \"c\" }"
                .to_owned(),
            "a pin that is not 40 hex",
        ),
        (
            format!(
                "{{ secs = 180, measured = [], rule = \"r\", pin = \"{PIN}\", command = \"c\" }}"
            ),
            "an empty measured list",
        ),
        (
            format!("{{ secs = 180, measured = [90], pin = \"{PIN}\", command = \"c\" }}"),
            "no rule",
        ),
        (
            format!(
                "{{ secs = 180, measured = [90], rule = \"r\", pin = \"{PIN}\", command = \"c\", \
                 source = \"s\" }}"
            ),
            "both command and source",
        ),
        (
            format!("{{ secs = 180, measured = [90], rule = \"r\", pin = \"{PIN}\" }}"),
            "neither command nor source",
        ),
        (
            format!(
                "{{ secs = 0, measured = [0], rule = \"r\", pin = \"{PIN}\", command = \"c\" }}"
            ),
            "a zero secs",
        ),
        (
            format!(
                "{{ secs = 180, measured = [\"slow\"], rule = \"r\", pin = \"{PIN}\", command = \"c\" }}"
            ),
            "a reading that is not a number",
        ),
        (
            format!(
                "{{ secs = 3600, measured = [90], rule = \"r\", pin = \"{PIN}\", command = \"c\" }}"
            ),
            "a measured secs the rule does not give (60 × ⌈2 × 90 / 60⌉ = 180)",
        ),
        (
            "{ unmeasured = \"\" }".to_owned(),
            "an empty unmeasured reason",
        ),
        (
            "{ unmeasured = \"x\", secs = 60 }".to_owned(),
            "unmeasured beside a value",
        ),
    ];
    for (entry, why) in refusals {
        match load(&entry) {
            Err(VerbRegistryError::Value {
                entry: named,
                detail,
            }) => {
                assert_eq!(
                    named, "toolchain.t.k",
                    "{why}: the refusal names the wrong key: {detail}"
                );
            }
            other => panic!("a value with {why} was not refused as a missing warrant: {other:?}"),
        }
    }
    // A declaration row outside the named set is not admitted as one.
    assert!(matches!(
        VerbRegistry::from_toml(&format!(
            "{VERBS}[toolchain.t]\nextra_row = {{ value = 1, source = \"s\" }}\n"
        )),
        Err(VerbRegistryError::Value { ref entry, .. }) if entry == "toolchain.t.extra_row"
    ));
}

// ------------------------------------------------------------------------------------------------
// THE SHIPPED REGISTRY (s8c §8c.4-§8c.5; P8). Each test reads `.redkiln/registry.toml` as shipped.
// ------------------------------------------------------------------------------------------------

/// The workspace root, from this crate's manifest directory.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .canonicalize()
        .expect("the workspace root resolves from this crate's manifest directory")
}

/// The shipped registry, through the loader `rk-cli` uses (`VerbRegistry::load_file`), so its
/// `path` source entries are checked against the shipped `.redkiln/`.
fn shipped() -> VerbRegistry {
    let path = workspace_root().join(".redkiln").join("registry.toml");
    VerbRegistry::load_file(&path).unwrap_or_else(|e| {
        panic!(
            "the shipped registry {} does not load — design §4.5's verb table is not shipped \
             (D21-10's gap): {e}",
            path.display()
        )
    })
}

/// The shipped catalog, through the kernel's one catalog loader.
fn shipped_catalog() -> Catalog {
    let catalog = Catalog::from_dir(&workspace_root().join(".redkiln").join("catalog"))
        .expect("the shipped catalog loads");
    assert_eq!(
        catalog.len(),
        102,
        "premise: this is the shipped catalog and not a subset of it"
    );
    catalog
}

/// The rows record V decision 1 lands, as `(verb, route, outputs, live)`.
///
/// *(dated 2026-09-26, `s8d` build, package P7; R5 = ADR-0177, proposed.)* Twelve, not ten: R5 adds
/// `shallow_clone` (route `review_clone`) and `verdict_record` (route `review_confirm`), each by
/// record, as ADR-0170 decision 1 provides. The test keeps its name; its premise moved with R5.
///
/// *(dated 2026-09-28, s8f build, package S8F-P6-ITEMS-BACKLOG-FEEDS; D21-46 R7; D21-106; the render
/// record ADR-0189 decision 12, proposed.)* Thirteen: `s8f` adds `backlog`, the `store` route's first
/// read (`live = true`), in the same commit as the two `[feed.*.backlog]` rows. No route is added.
const SHIPPED_ROWS: [(&str, &str, &str, bool); 13] = [
    ("build", "port", "BuildReport", false),
    ("change_summary", "port", "ChangeSummary", false),
    ("item_create", "store", "ItemRefs", false),
    ("lint", "port", "LintReport", false),
    ("package", "toolchain", "BuildArtifact", false),
    ("release_tag", "git", "ReleaseTag", false),
    ("sca_scan", "toolchain", "ScanReport", false),
    ("setup", "toolchain", "SetupResult", true),
    ("smoke", "toolchain", "SmokeResult", true),
    ("test", "port", "TestReport", false),
    ("shallow_clone", "review_clone", "Workspace", false),
    ("verdict_record", "review_confirm", "Verdict", false),
    ("backlog", "store", "BacklogSnapshot", true),
];

#[test]
fn the_shipped_registry_loads_with_route_outputs_and_live_on_every_row() {
    let reg = shipped();
    assert_eq!(
        reg.len(),
        SHIPPED_ROWS.len(),
        "the shipped `[verb]` table holds exactly record V decision 1's ten rows, R5's two and \
         s8f's `backlog`"
    );
    for (verb, route, outputs, live) in SHIPPED_ROWS {
        let row = reg
            .entry(verb)
            .unwrap_or_else(|| panic!("the shipped registry has no `{verb}` row"));
        assert_eq!(row.route.as_str(), route, "`{verb}`'s route");
        if let Route::Port(method) = row.route {
            assert_eq!(
                method,
                PortMethod::from_verb(verb).expect("a port verb"),
                "`{verb}` reaches its own port method"
            );
        }
        assert_eq!(row.outputs, outputs, "`{verb}`'s typed output kind");
        assert_eq!(row.live, live, "`{verb}`'s `live`");
        assert_eq!(reg.live(verb), Some(live));
        assert!(
            !row.timeout.is_empty(),
            "`{verb}` declares a `${{toolchain.*}}` timeout key"
        );
    }
    // A `toolchain` row declares the inputs its handler reads, so the executor refuses it before
    // the port until `s8d`'s resolver supplies them (§8c.4's routing table).
    for verb in ["smoke", "sca_scan", "package", "setup"] {
        assert!(
            !reg.entry(verb).expect("rowed").inputs.is_empty(),
            "`{verb}` is a `toolchain` row and declares no inputs: the executor would reach \
             `run_verb` with none"
        );
    }
    assert_eq!(
        reg.entry("setup").expect("rowed").timeout,
        "setup_timeout",
        "the `setup` row's timeout is its own key (§8c.4)"
    );
    // The verbs that have NO row here refuse by bare name (ADR-0135 clause 2): none is carried.
    // *(dated 2026-09-26, `s8d` build, package P7: `shallow_clone` and `verdict_record` left this
    // list, their rows being R5's = ADR-0177, proposed; the other three still have none.)*
    for verb in ["mutate", "release_record", "deploy"] {
        assert_eq!(
            reg.live(verb),
            None,
            "`{verb}` must have no row (record V decision 1; R5 rowed only its two)"
        );
    }
    // Both per-toolchain tables are declared.
    for toolchain in ["rust", "node"] {
        assert!(
            reg.values_for(toolchain).is_some(),
            "`[toolchain.{toolchain}]` is not declared"
        );
    }
}

/// **AC-018 over the shipped catalog AND the shipped registry** — `s8b`'s hand-on (§8c.4). After
/// `s8b` §8b.5 no shipped step declares `memo`, so the rule would hold over the catalog whatever
/// the registry said; the explicit `live = true` assertion on `smoke` is what kills M20.
#[test]
fn ac018_holds_over_the_shipped_catalog_and_registry() {
    let reg = shipped();
    let catalog = shipped_catalog();
    let steps: usize = catalog.activities().iter().map(|a| a.steps.len()).sum();
    let rowed_tool_steps = catalog
        .activities()
        .iter()
        .flat_map(|a| a.steps.iter())
        .filter(|s| s.kind == StepKind::Tool)
        .filter_map(|s| s.run.as_deref().and_then(verb_of))
        .filter(|verb| reg.live(verb).is_some())
        .count();
    println!(
        "ac018 (shipped): activities={} steps={steps} tool_steps_on_a_rowed_verb={rowed_tool_steps} \
         registry=.redkiln/registry.toml",
        catalog.len()
    );
    assert!(steps > 0, "a rule over zero steps reports zero violations");
    assert!(
        rowed_tool_steps > 0,
        "no shipped tool step names a rowed verb: the re-proof would not touch the registry"
    );
    assert_eq!(
        live_verb_memo_rule(&catalog, &reg),
        Ok(()),
        "AC-018 over the shipped catalog and registry"
    );
    assert_eq!(
        reg.live("smoke"),
        Some(true),
        "the shipped `smoke` row must be `live = true` (s8b §8b.5's hand-on): a live verb's result \
         is never memoised"
    );
}

/// The `${toolchain.<key>}` census, read from the catalog's TEXT, independently of the loader:
/// one per `timeout_secs = "${toolchain.` line.
fn substitution_census(dir: &Path) -> usize {
    let mut n = 0;
    for entry in std::fs::read_dir(dir).expect("the catalog directory") {
        let path = entry.expect("an entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("toml") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("a catalog file");
        n += text
            .lines()
            .filter(|l| {
                let l = l.trim_start();
                l.starts_with("timeout_secs") && l.contains("\"${toolchain.")
            })
            .count();
    }
    n
}

#[test]
fn every_shipped_toolchain_substitution_resolves_or_refuses_by_name() {
    let reg = shipped();
    let catalog = shipped_catalog();
    let census = substitution_census(&workspace_root().join(".redkiln").join("catalog"));
    let keys: Vec<String> = catalog
        .activities()
        .iter()
        .flat_map(|a| a.steps.iter())
        .filter_map(|s| match &s.timeout_secs {
            Some(StepTimeout::Toolchain(key)) => Some(key.clone()),
            _ => None,
        })
        .collect();
    println!("substitutions: walked={} census={census}", keys.len());
    assert!(
        !keys.is_empty(),
        "the walk found no `${{toolchain.*}}` step"
    );
    assert_eq!(
        keys.len(),
        census,
        "the loader walked a different number of `${{toolchain.*}}` steps than the catalog's text \
         holds"
    );
    // The registry rows' own keys are substitutions too (`setup_timeout` has no catalog site).
    let row_keys: Vec<String> = SHIPPED_ROWS
        .iter()
        .map(|(verb, ..)| reg.entry(verb).expect("rowed").timeout.clone())
        .collect();

    for toolchain in ["rust", "node"] {
        let values = reg.values_for(toolchain).expect("the table is declared");
        let (mut resolved, mut refused) = (0_usize, 0_usize);
        for key in keys.iter().chain(row_keys.iter()) {
            match values.resolve(key) {
                Ok(secs) => {
                    assert!(secs.get() > 0);
                    resolved += 1;
                }
                Err(refusal) => {
                    assert!(
                        matches!(refusal, ValueRefusal::Unmeasured { .. }),
                        "[toolchain.{toolchain}] does not DECLARE `{key}`: an absent key is a gap \
                         in the table, not a declared absence ({refusal})"
                    );
                    let rendered = refusal.to_string();
                    assert!(
                        rendered.contains(&format!("${{toolchain.{key}}}")),
                        "the refusal does not name `${{toolchain.{key}}}`: {rendered}"
                    );
                    refused += 1;
                }
            }
        }
        println!("[toolchain.{toolchain}]: resolved={resolved} refused_by_name={refused}");
        assert_eq!(resolved + refused, keys.len() + row_keys.len());
        if toolchain == "rust" {
            assert!(
                resolved > 0 && refused > 0,
                "[toolchain.rust] resolves some keys and refuses the unmeasured ones by name"
            );
        } else {
            assert_eq!(
                resolved, 0,
                "every [toolchain.node] key is `unmeasured` (§8c.5)"
            );
        }
    }
}

/// The §8c.5 key map, as the four classes `[toolchain.rust]` must hold exactly.
const MEASURED_KEYS: [&str; 4] = [
    "build_timeout",
    "scan_timeout",
    "setup_timeout",
    "test_timeout",
];
const CITED_KEYS: [&str; 3] = ["implement_timeout", "repro_timeout", "review_timeout"];
const UNMEASURED_KEYS: [&str; 5] = [
    "cook_timeout",
    "deploy_timeout",
    "migration_timeout",
    "mutation_timeout",
    "ue_test_timeout",
];

/// The raw `[toolchain.rust]` table, read with the kernel's own TOML subset so every warrant field
/// is visible (the loaded `ToolchainValues` keeps only the seconds).
fn raw_rust_table() -> BTreeMap<String, TomlValue> {
    let text = std::fs::read_to_string(workspace_root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry is readable");
    let doc = rk_kernel::toml_subset::parse(&text).expect("the shipped registry parses");
    doc.get("toolchain")
        .and_then(TomlValue::as_table)
        .and_then(|t| t.get("rust"))
        .and_then(TomlValue::as_table)
        .expect("[toolchain.rust] is declared")
        .clone()
}

fn fields(v: &TomlValue) -> BTreeSet<&str> {
    v.as_table()
        .map(|t| t.keys().map(String::as_str).collect())
        .unwrap_or_default()
}

/// A value's warrant fields, ending in `command` (measured) or `source` (cited).
fn warrant(last: &'static str) -> BTreeSet<&'static str> {
    ["secs", "measured", "rule", "pin", last]
        .into_iter()
        .collect()
}

#[test]
fn the_shipped_rust_value_table_is_exactly_the_declared_key_map() {
    let table = raw_rust_table();
    for key in MEASURED_KEYS {
        let v = table
            .get(key)
            .unwrap_or_else(|| panic!("[toolchain.rust] has no `{key}` (a measured key, §8c.5)"));
        assert_eq!(
            fields(v),
            warrant("command"),
            "`{key}` is measured here: secs, measured, rule, pin and command"
        );
    }
    for key in CITED_KEYS {
        let v = table
            .get(key)
            .unwrap_or_else(|| panic!("[toolchain.rust] has no `{key}` (a cited key, §8c.5)"));
        assert_eq!(
            fields(v),
            warrant("source"),
            "`{key}` is cited: secs, measured, rule, pin and source"
        );
    }
    for key in UNMEASURED_KEYS {
        let v = table.get(key).unwrap_or_else(|| {
            panic!("[toolchain.rust] has no `{key}` (an unmeasured key, §8c.5)")
        });
        assert_eq!(
            fields(v),
            BTreeSet::from(["unmeasured"]),
            "`{key}` is declared `unmeasured` with its reason"
        );
    }
    let mut declared: BTreeSet<&str> = MEASURED_KEYS.into_iter().collect();
    declared.extend(CITED_KEYS);
    declared.extend(UNMEASURED_KEYS);
    declared.insert("setup_projects");
    // *(dated 2026-09-27, `s8d` build, package F2-D2173; D21-73 item 5; R8 = ADR-0179 decision 4,
    // proposed.)* `package_binaries`, the second of ADR-0170 decision 9's three declaration rows, is
    // written at `s8d` (the ruling moves it from `s8g`); the map's declaration set is decision 9's.
    declared.insert("package_binaries");
    // *(dated 2026-09-29, `s8g` build, package S8G-P3-KERNEL-GIT; R1 = ADR-0191 decision 10,
    // proposed; AC-349.)* `claims`, the third of decision 9's three declaration rows (`s8c` §8c.5:
    // *"`s8g`'s `claims` column (AC-349)"*), is written by `s8g`'s verify record.
    declared.insert("claims");
    // *(dated 2026-09-29, `s8g` build, package R-D2122-D2123-8g; D21-123 item 3, TAKEN PROVISIONALLY
    // under D21-105; R2 = ADR-0192 decision 2, proposed.)* `mutation`, the fourth declaration row
    // (`DECLARATION_ROWS` 3 -> 4): AC-337's feasibility, `{ feasible = true }`, with no `reason`.
    declared.insert("mutation");
    let mutation = table
        .get("mutation")
        .and_then(TomlValue::as_table)
        .expect("[toolchain.rust] declares the `mutation` row (D21-123 item 1)");
    assert_eq!(
        mutation
            .get("value")
            .and_then(TomlValue::as_table)
            .map(|v| (
                v.keys().cloned().collect::<Vec<_>>(),
                v.get("feasible").and_then(TomlValue::as_bool)
            )),
        Some((vec!["feasible".to_owned()], Some(true))),
        "`mutation`'s value is exactly `{{ feasible = true }}`, with no `reason`"
    );
    let extra: Vec<&String> = table
        .keys()
        .filter(|k| !declared.contains(k.as_str()))
        .collect();
    assert!(
        extra.is_empty(),
        "[toolchain.rust] holds keys outside §8c.5's map: {extra:?}"
    );

    // D21-48 X4: implement_timeout and repro_timeout are 4,800 s each, cited to F31/S-TYPED's
    // slowest dispatch (4,561 s) as a borrowed class, with a rule naming the re-derivation.
    for key in ["implement_timeout", "repro_timeout"] {
        let t = table[key].as_table().expect("a warrant table");
        assert_eq!(t["secs"].as_int(), Some(4800), "`{key}` is X4's 4,800 s");
        let measured: Vec<Option<f64>> = t["measured"]
            .as_array()
            .expect("a list")
            .iter()
            .map(TomlValue::as_f64)
            .collect();
        assert_eq!(measured, vec![Some(4561.0)], "`{key}` cites [4561]");
        let source = t["source"].as_str().unwrap_or_default();
        assert!(
            source.contains("F31") && source.contains("borrowed class"),
            "`{key}`'s source names F31/S-TYPED as a borrowed class: {source}"
        );
        let rule = t["rule"].as_str().unwrap_or_default();
        assert!(
            rule.contains("re-deriv"),
            "`{key}`'s rule names the re-derivation (ADR-0170 decision 11): {rule}"
        );
    }
    // review_timeout: AC-223's own evidence line under decision 10's rule.
    let review = table["review_timeout"].as_table().expect("a warrant table");
    assert_eq!(review["secs"].as_int(), Some(9180));
    assert!(
        review["source"]
            .as_str()
            .unwrap_or_default()
            .contains("4,561")
    );

    // Every pin is 40 hex; every measured key's secs follows decision 10 (the loader refuses
    // otherwise, so loading is the check), and resolves through the flat table.
    let reg = shipped();
    let values = reg.values_for("rust").expect("[toolchain.rust]");
    for key in MEASURED_KEYS.iter().chain(CITED_KEYS.iter()) {
        let t = table[*key].as_table().expect("a warrant table");
        let secs = u64::try_from(t["secs"].as_int().expect("secs")).expect("positive");
        assert_eq!(
            values.resolve(key).map(std::num::NonZeroU64::get),
            Ok(secs),
            "`{key}` resolves to its declared secs"
        );
    }
    let setup = reg
        .table("rust")
        .and_then(|t| t.declarations().get("setup_projects"))
        .expect("the setup_projects declaration row");
    assert_eq!(
        setup.value,
        TomlValue::Array(vec![TomlValue::Str(
            "crates/rk-adapter-toolchain-node/tests/fixtures/project".to_owned()
        )]),
        "setup_projects is the one tracked pnpm-lock.yaml's project (§8c.4)"
    );
    assert!(setup.source.contains("git ls-files"));
}

// ------------------------------------------------------------------------------------------------
// R1's SIXTH COLUMN (`s8d` §8d.1 M8; record R1, proposed). *(Added 2026-09-26, `s8d` build, P3.)*
// ------------------------------------------------------------------------------------------------

/// The verbs whose re-execution would repeat a write (§8d.1 R1's row): a second review clone, a
/// second verdict record, a second artifact, a second item or a second tag. `shallow_clone` and
/// `verdict_record` get their rows from R5; each is required `true` the moment it has one.
const WRITING_VERBS: [&str; 5] = [
    "shallow_clone",
    "verdict_record",
    "package",
    "item_create",
    "release_tag",
];

/// **Every `[verb]` row DECLARES `side_effecting`, and the writing verbs are marked `true`.** A row
/// without the value refuses the whole registry at load — never read as `false`, which would let a
/// writing verb re-execute on re-entry (M30).
#[test]
fn every_row_declares_side_effecting_and_the_writing_verbs_are_marked() {
    // ---- (1) THE SHIPPED FILE, RAW: every row carries the column as written, not by default. ----
    let text = std::fs::read_to_string(workspace_root().join(".redkiln").join("registry.toml"))
        .expect("the shipped registry is readable");
    let doc = rk_kernel::toml_subset::parse(&text).expect("the shipped registry parses");
    let rows = doc
        .get("verb")
        .and_then(TomlValue::as_table)
        .expect("[verb] is declared");
    assert!(
        rows.len() >= SHIPPED_ROWS.len(),
        "the scan must walk every shipped row, not a subset: {} rows",
        rows.len()
    );
    for (verb, row) in rows {
        let declared = row
            .as_table()
            .and_then(|t| t.get("side_effecting"))
            .and_then(TomlValue::as_bool);
        assert!(
            declared.is_some(),
            "the shipped `{verb}` row declares no boolean `side_effecting` (R1's sixth column; \
             `s8d` §8d.1 M8)"
        );
    }

    // ---- (2) THE LOADED REGISTRY: the marks are exactly the writing verbs that have rows. ------
    let reg = shipped();
    for must in ["package", "item_create", "release_tag"] {
        assert!(
            reg.entry(must).is_some(),
            "PREMISE: `{must}` has a row (s8c's record V), so its mark is checkable"
        );
    }
    let mut marked = 0_usize;
    for verb in rows.keys() {
        let entry = reg.entry(verb).expect("every raw row loads");
        let writing = WRITING_VERBS.contains(&verb.as_str());
        assert_eq!(
            entry.side_effecting, writing,
            "`{verb}`'s `side_effecting` must be {writing}: a writing verb re-executed on re-entry \
             repeats its write, and an idempotent one marked would be served stale"
        );
        marked += usize::from(entry.side_effecting);
    }
    assert!(
        marked >= 3,
        "at least package, item_create and release_tag are marked"
    );

    // ---- (3) A ROW WITHOUT THE VALUE REFUSES THE REGISTRY AT LOAD, naming the verb. -------------
    let control = VerbRegistry::from_toml(
        "[verb]\nbuild = { outputs = \"BuildReport\", live = false, timeout = \"build_timeout\", \
         inputs = [], route = \"port\", side_effecting = false }\n",
    );
    assert!(
        control.is_ok(),
        "CONTROL: a row declaring the column loads: {control:?}"
    );
    match VerbRegistry::from_toml(
        "[verb]\nbuild = { outputs = \"BuildReport\", live = false, timeout = \"build_timeout\", \
         inputs = [], route = \"port\" }\n",
    ) {
        Err(VerbRegistryError::Shape { verb, detail }) => {
            assert_eq!(verb, "build");
            assert!(
                detail.contains("side_effecting"),
                "the refusal names the missing column: {detail}"
            );
        }
        other => panic!(
            "a row with no `side_effecting` value must refuse the registry at load, never load as \
             `false` (M30): {other:?}"
        ),
    }
    // A non-boolean value is not a value.
    assert!(
        VerbRegistry::from_toml(
            "[verb]\nbuild = { outputs = \"BuildReport\", live = false, timeout = \
             \"build_timeout\", inputs = [], route = \"port\", side_effecting = \"no\" }\n",
        )
        .is_err(),
        "a string `side_effecting` is refused"
    );
}

/// **D21-73 item 5, the `package` half (R8 = ADR-0179 decision 4, proposed; `s8d` build, package
/// F2-D2173).** The shipped `[toolchain.rust]` carries the `package_binaries` declaration row (ADR-0170
/// decision 9 admits it), and `row_inputs_for` hands it to the `package` row as its `binaries`
/// input: exactly `redkiln` and `redkiln-migrate` (AC-441: the migrator is *"a separate binary"*),
/// never a fixture `[[bin]]`. A table without the row supplies nothing, so the step refuses by name
/// as before (the control).
#[test]
fn the_package_binaries_declaration_supplies_the_package_rows_binaries() {
    let reg = shipped();
    let supplied = reg.row_inputs_for("rust");
    assert_eq!(
        supplied
            .get("package")
            .and_then(|m| m.get("binaries"))
            .cloned(),
        Some(serde_json::json!(["redkiln", "redkiln-migrate"])),
        "the shipped `package_binaries` row must supply `package.binaries`: {supplied:?}"
    );
    assert_eq!(
        supplied.keys().collect::<Vec<_>>(),
        vec!["package"],
        "a declaration row supplies only the row inputs R8 maps (`setup_projects` is not one)"
    );
    let declared = reg
        .table("rust")
        .and_then(|t| t.declarations().get("package_binaries"))
        .expect("the package_binaries declaration row");
    assert!(
        declared.source.contains("[[bin]]"),
        "the row's warrant names where the set was read: {}",
        declared.source
    );

    // CONTROL: a table with no `package_binaries` row supplies nothing.
    let bare = VerbRegistry::from_toml(&format!(
        "{VERBS}[toolchain.t]\nsetup_projects = {{ value = [\"p\"], source = \"git ls-files\" }}\n"
    ))
    .expect("CONTROL: loads");
    assert!(
        bare.row_inputs_for("t").is_empty() && bare.row_inputs_for("absent").is_empty(),
        "CONTROL: no declaration row, no row input"
    );
}
