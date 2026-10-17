//! **`s8h` §8h.4 — the schema a dispatch validates against is an `rk_types::registry::EXPORTED`
//! Rust type, read by the bound step's output kind.** *(Added 2026-10-02, s8h build, package
//! P7-cli-binding-data-and-amendments; Record A = ADR-0197 decision 6, proposed.)*
//!
//! Three tests: the universe of agent-step output kinds the two shipped lanes can reach is §8h.4's
//! 24, each EXPORTED (a premise, so a catalog change that reaches a new kind fails loudly); `run`
//! binds the EXPORTED schema even with a decoy `.redkiln/schemas/<kind>.json` planted beside the
//! pack; and a kind absent from `EXPORTED` is refused at bind, naming the kind and the step, before
//! any spawn.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use rk_cli::exit::Exit;
use rk_cli::verbs::HarnessFactory;
use rk_ports::cancel::CancelToken;
use rk_ports::fakes::FakeHarness;
use rk_ports::harness::Harness;

use crate::lane_e2e;

/// The workspace root: `crates/rk-cli/../..`.
fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// One lane's reachable agent-step output kinds, by §8h.4's rule: every stage whose rigor is not
/// `forbidden` contributes its `baseline` and every `require` row's `artifacts`; the set is closed
/// over each REQUIRED (`requires` entry without `?`) input of every catalog activity tagged
/// `kind:<lane kind>` that `provides` a member; and the `value:<Kind>` of every `kind = "agent"`
/// step of those activities is collected.
///
/// *(dated 2026-10-02, s8h build)* The 2026-09-24 measurement's 18 / 17 / 24 reproduces only with
/// optional `X?` requires LEFT OUT of the closure: following them as well reaches 19 / 19 / 26
/// (`DataMigrationPlan` through `design-doc`'s `DataMigrationPlan?` and `rollout-plan`'s, and
/// `UXDesign` through `design-doc`'s `UXDesign?`). An optional input is not something the activity
/// requires, so it does not extend what the lane must produce; the rule is stated here so it
/// reproduces.
fn lane_kinds(pack: &str, tag: &str, activities: &[toml::Value]) -> BTreeSet<String> {
    let path = workspace_root()
        .join(".redkiln")
        .join("packs")
        .join(format!("{pack}.toml"));
    let doc: toml::Value =
        toml::from_str(&std::fs::read_to_string(&path).expect("the pack reads")).expect("TOML");
    let mut frontier: Vec<String> = Vec::new();
    let stages = doc
        .get("policy")
        .and_then(|p| p.get("stages"))
        .and_then(toml::Value::as_table)
        .expect("the pack declares [policy] stages");
    for (stage, rigor) in stages {
        if rigor.as_str() == Some("forbidden") {
            continue;
        }
        let Some(spec) = doc.get("stage").and_then(|s| s.get(stage)) else {
            continue;
        };
        let strings = |v: Option<&toml::Value>| -> Vec<String> {
            v.and_then(toml::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect()
        };
        frontier.extend(strings(spec.get("baseline")));
        for row in spec
            .get("require")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
        {
            frontier.extend(strings(row.get("artifacts")));
        }
    }
    let tagged: Vec<&toml::Value> = activities
        .iter()
        .filter(|a| {
            a.get("activity")
                .and_then(|x| x.get("tags"))
                .and_then(toml::Value::as_array)
                .is_some_and(|t| t.iter().any(|x| x.as_str() == Some(tag)))
        })
        .collect();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut chosen: BTreeMap<String, &toml::Value> = BTreeMap::new();
    while let Some(kind) = frontier.pop() {
        if !seen.insert(kind.clone()) {
            continue;
        }
        for a in &tagged {
            let head = a.get("activity").expect("an [activity] table");
            let provides = head.get("provides").and_then(toml::Value::as_array);
            if !provides.is_some_and(|p| p.iter().any(|x| x.as_str() == Some(kind.as_str()))) {
                continue;
            }
            let id = head
                .get("id")
                .and_then(toml::Value::as_str)
                .expect("an activity id")
                .to_owned();
            chosen.insert(id, a);
            for r in head
                .get("requires")
                .and_then(toml::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(toml::Value::as_str)
            {
                if !r.ends_with('?') {
                    frontier.push(r.to_owned());
                }
            }
        }
    }
    let mut kinds = BTreeSet::new();
    for a in chosen.values() {
        for step in a
            .get("step")
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
        {
            if step.get("kind").and_then(toml::Value::as_str) != Some("agent") {
                continue;
            }
            let outputs = step
                .get("outputs")
                .and_then(toml::Value::as_str)
                .unwrap_or("");
            let kind = outputs.strip_prefix("value:").unwrap_or_else(|| {
                panic!("PREMISE: an agent step's outputs is not `value:<Kind>`: {outputs:?}")
            });
            kinds.insert(kind.to_owned());
        }
    }
    kinds
}

/// **§8h.4: every agent-step output kind the two shipped lanes can reach is an `EXPORTED` type.**
/// The universe is a CANDIDATE universe (a closure over `requires`), not a selection run, so a
/// facet read at run time cannot route to a kind outside it. Its size, 24, is a PREMISE.
///
/// RED (observed 2026-10-02, through D21-47's wrapper, against `EXPORTED` holding only `Findings`
/// and `AcceptanceCriterion` — package P2's types removed in a private worktree, §8h.13's driver):
/// 23 kinds absent. On this tree P2 had landed the types before this test was written, so its red
/// is that instrument's.
#[test]
fn every_agent_output_kind_the_two_lanes_reach_is_exported() {
    let dir = workspace_root().join(".redkiln").join("catalog");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the shipped catalog reads")
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    files.sort();
    let activities: Vec<toml::Value> = files
        .iter()
        .map(|p| {
            toml::from_str(&std::fs::read_to_string(p).expect("reads"))
                .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
        })
        .collect();
    let fix = lane_kinds("bug-standard", "kind:bug", &activities);
    let story = lane_kinds("story", "kind:story", &activities);
    assert_eq!(
        fix.len(),
        18,
        "PREMISE: the fix lane reaches 18 kinds: {fix:?}"
    );
    assert_eq!(
        story.len(),
        17,
        "PREMISE: the story lane reaches 17 kinds: {story:?}"
    );
    let universe: BTreeSet<String> = fix.union(&story).cloned().collect();
    assert_eq!(
        universe.len(),
        24,
        "PREMISE: §8h.4's universe is 24 kinds; a catalog change moved it: {universe:?}"
    );
    let exported: BTreeSet<&str> = rk_types::registry::EXPORTED
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let absent: Vec<&String> = universe
        .iter()
        .filter(|k| !exported.contains(k.as_str()))
        .collect();
    assert!(
        absent.is_empty(),
        "{} lane-reached kind(s) have no EXPORTED type, so the binder refuses them: {absent:?}",
        absent.len()
    );
}

/// A factory over one scripted `FakeHarness` that RECORDS the schemas the binder put in the card
/// store it is handed (`Config::cards()`, the store `dispatch` reads by `spec.schema`).
struct Capturing {
    harness: Arc<dyn Harness>,
    builds: Mutex<Vec<BTreeMap<String, serde_json::Value>>>,
}

impl Capturing {
    fn new(script: Vec<Result<rk_types::typed::Typed, rk_types::error::StepError>>) -> Self {
        Self {
            harness: Arc::new(FakeHarness::scripted(script)),
            builds: Mutex::new(Vec::new()),
        }
    }

    fn schemas(&self) -> BTreeMap<String, serde_json::Value> {
        let mut out = BTreeMap::new();
        for build in self.builds.lock().expect("the capture lock").iter() {
            for (id, json) in build {
                out.insert(id.clone(), json.clone());
            }
        }
        out
    }
}

impl HarnessFactory for Capturing {
    fn build(
        &self,
        config: rk_adapter_claude::config::Config,
        _token: CancelToken,
    ) -> Result<Arc<dyn Harness>, String> {
        let mut bound = BTreeMap::new();
        for entry in config.cards().values() {
            for (id, json) in &entry.schemas {
                bound.insert(id.as_str().to_owned(), json.clone());
            }
        }
        self.builds.lock().expect("the capture lock").push(bound);
        Ok(Arc::clone(&self.harness))
    }

    fn builds_the_real_harness(&self) -> bool {
        false
    }
}

/// `EXPORTED`'s schema for `kind`.
fn exported(kind: &str) -> serde_json::Value {
    rk_types::registry::EXPORTED
        .iter()
        .find(|(name, _)| *name == kind)
        .map(|(_, f)| f())
        .unwrap_or_else(|| panic!("PREMISE: {kind} is EXPORTED"))
}

/// **§8h.4: `run` binds each agent step's schema from `EXPORTED`, by the step's `value:<Kind>`** —
/// with a DECOY `.redkiln/schemas/<kind>.json` planted for both kinds the fixture lane dispatches
/// (`ImplReport`, `Findings`), so a binder that reads the file again binds the decoy and fails here
/// (M13). The lane is `pack-min`'s, walked to its end in process over a scripted harness; the
/// factory records the card store the binder filled.
///
/// RED (observed 2026-10-02, through D21-47's wrapper, before the binder change): the bound
/// `ImplReport` schema was the decoy's.
#[test]
fn bind_reads_the_schema_from_exported_by_name() {
    let repo = lane_e2e::lane_repo();
    let root = repo.path();
    let schemas_dir = root.join(".redkiln").join("schemas");
    std::fs::create_dir_all(&schemas_dir).expect("the decoy directory is created");
    let decoy = |kind: &str| {
        serde_json::json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": kind,
            "type": "object",
            "description": "A DECOY: s8h §8h.4 binds from EXPORTED and never reads this file.",
            "properties": { "decoy": { "type": "string" } },
        })
    };
    for kind in ["ImplReport", "Findings"] {
        std::fs::write(
            schemas_dir.join(format!("{kind}.json")),
            serde_json::to_vec_pretty(&decoy(kind)).expect("the decoy serialises"),
        )
        .expect("the decoy is planted");
    }

    let factory = Capturing::new(lane_e2e::through());
    let exit = lane_e2e::ran(
        rk_cli::verbs::run::run(
            &lane_e2e::cli(root, lane_e2e::THROUGH_ITEM),
            &lane_e2e::args(lane_e2e::THROUGH_ITEM),
            &crate::support::clean_env(),
            rk_cli::verbs::Startup {
                harness: &factory,
                ..rk_cli::verbs::Startup::real()
            },
            &CancelToken::new(),
        ),
        lane_e2e::THROUGH_ITEM,
    );
    assert_eq!(
        exit,
        Exit::Success,
        "PREMISE: the fixture lane walks to its end"
    );
    let bound = factory.schemas();
    for kind in ["ImplReport", "Findings"] {
        let got = bound
            .get(kind)
            .unwrap_or_else(|| panic!("PREMISE: the lane bound a {kind} schema: {bound:?}"));
        assert_ne!(
            got,
            &decoy(kind),
            "{kind}: the binder read the decoy file (§8h.4)"
        );
        assert_eq!(
            got,
            &exported(kind),
            "{kind}: the bound schema is EXPORTED's, by name"
        );
    }
}

/// **§8h.4: a kind absent from `EXPORTED` is refused at bind (`Exit::ConfigError`, 3), naming the
/// kind and the step, before any spawn** — over the binary, so the refusal's text is read: the
/// fixture's one agent step (`emit-report`'s `check`) is re-pointed at `value:NoSuchKind`, a
/// decoy `.redkiln/schemas/NoSuchKind.json` is planted so a file read would FIND a schema, and the
/// run must refuse without spawning the fixture child (no marker).
///
/// RED (observed 2026-10-02, through D21-47's wrapper, before the binder change): the binary read
/// the decoy and spawned the child (a marker was written), exit 11.
#[test]
fn a_kind_absent_from_exported_refuses_at_bind() {
    let repo = crate::support::pack_min_repo(crate::support::PackVariant::Base);
    let root = repo.path();
    assert!(
        !rk_types::registry::EXPORTED
            .iter()
            .any(|(name, _)| *name == "NoSuchKind"),
        "PREMISE: NoSuchKind is not EXPORTED"
    );
    let activity = root
        .join(".redkiln")
        .join("catalog")
        .join("emit-report.toml");
    let src = std::fs::read_to_string(&activity).expect("the activity reads");
    // The step's own line, anchored at the line start: the file's comments quote the same text.
    let line = "\noutputs = \"value:ImplReport\"\n";
    assert_eq!(
        src.matches(line).count(),
        1,
        "PREMISE: emit-report's one agent step outputs value:ImplReport"
    );
    std::fs::write(
        &activity,
        src.replace(line, "\noutputs = \"value:NoSuchKind\"\n"),
    )
    .expect("the activity is re-pointed");
    let schemas_dir = root.join(".redkiln").join("schemas");
    std::fs::create_dir_all(&schemas_dir).expect("the decoy directory is created");
    std::fs::write(
        schemas_dir.join("NoSuchKind.json"),
        r#"{"$schema":"https://json-schema.org/draft/2020-12/schema","title":"NoSuchKind","type":"object"}"#,
    )
    .expect("the decoy is planted");

    let out = crate::run::spawn_run(root, "bug-1", None);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let markers = root.join(".rk-markers");
    let spawned = std::fs::read_dir(&markers)
        .map(|d| {
            d.flatten()
                .filter(|e| e.path().extension().is_some_and(|x| x == "marker"))
                .count()
        })
        .unwrap_or(0);
    assert_eq!(
        spawned, 0,
        "the run spawned the harness over a kind absent from EXPORTED; stderr:\n{stderr}"
    );
    assert_eq!(
        out.status.code(),
        Some(3),
        "a kind absent from EXPORTED is a configuration error (exit 3); stderr:\n{stderr}"
    );
    for needle in ["NoSuchKind", "`check`", "EXPORTED"] {
        assert!(
            stderr.contains(needle),
            "the refusal names the kind and the step (§8h.4): no {needle:?} in\n{stderr}"
        );
    }
}
