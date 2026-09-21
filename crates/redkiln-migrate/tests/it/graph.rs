//! The build graph (AC-466, AC-441's build-graph half).

use std::collections::{BTreeMap, BTreeSet};
use std::process::Stdio;

/// `cargo metadata`, in process.
///
/// The block runs the same assertion as a python walk outside the crate; this is its in-process
/// twin, and the two are deliberately independent instruments over the same fact.
#[expect(
    clippy::disallowed_types,
    reason = "a test reading its own workspace's cargo metadata; the product spawns nothing but \
              git, through rk-git's one wrapper"
)]
fn metadata() -> serde_json::Value {
    let out = std::process::Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .stdin(Stdio::null())
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("cargo metadata emits JSON")
}

/// AC-466 (BINDING) and AC-441's build-graph half.
///
/// *"zero harness dispatches are made and the binary links no harness adapter AND an attempt to
/// construct a harness port inside the migration path fails at build time."*
///
/// The *"fails at build time"* half is the same fact stated as a type: with no `rk-ports` edge
/// there is no `Harness` trait to implement or hold. The run-level half is vacuous in the exact
/// sense the register means — the migrator writes no event log — and `ac466_…`'s tree assertion
/// (no `store.sqlite3` in the migration commit) is where that lands, at cycle 4.
#[test]
fn ac466_ac441_migrate_depends_on_no_harness_adapter_or_ports() {
    let meta = metadata();
    let packages = meta["packages"].as_array().expect("packages");

    // name -> its normal/build dependency names (dev edges excluded: a dev-dependency is not in
    // the shipped graph, which is what AC-466 is about).
    let mut deps: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for p in packages {
        let name = p["name"].as_str().expect("a package name").to_owned();
        let mut set = BTreeSet::new();
        for d in p["dependencies"].as_array().expect("dependencies") {
            let kind = d["kind"].as_str().unwrap_or("normal");
            if kind == "dev" {
                continue;
            }
            set.insert(d["name"].as_str().expect("a dependency name").to_owned());
        }
        deps.insert(name, set);
    }

    assert!(
        deps.contains_key("redkiln-migrate"),
        "cargo metadata does not know redkiln-migrate — the walk below would read nothing"
    );

    // The transitive normal/build closure.
    let mut closure = BTreeSet::new();
    let mut stack = vec!["redkiln-migrate".to_owned()];
    while let Some(name) = stack.pop() {
        for dep in deps.get(&name).into_iter().flatten() {
            if closure.insert(dep.clone()) {
                stack.push(dep.clone());
            }
        }
    }

    // THE CONTROL, before the ban: a walk that reached nothing would report a clean closure.
    assert!(
        closure.contains("rk-types") && closure.contains("rk-git"),
        "the closure walk reached neither rk-types nor rk-git — it read nothing, and the ban \
         below would be vacuous (Halt 8 shape (a)). Reached: {closure:?}"
    );

    let forbidden: Vec<&String> = closure
        .iter()
        .filter(|c| {
            c.starts_with("rk-adapter-") || c.as_str() == "rk-ports" || c.as_str() == "rk-kernel"
        })
        .collect();
    assert!(
        forbidden.is_empty(),
        "redkiln-migrate's normal/build closure reaches {forbidden:?} — AC-466 forbids a harness \
         adapter, rk-ports and rk-kernel, and with no rk-ports edge there is no Harness trait to \
         construct at all"
    );

    // AC-441's build-graph half: no rk-* crate depends on redkiln-migrate.
    let reverse: Vec<&String> = deps
        .iter()
        .filter(|(name, d)| name.starts_with("rk-") && d.contains("redkiln-migrate"))
        .map(|(name, _)| name)
        .collect();
    assert!(
        reverse.is_empty(),
        "{reverse:?} depend(s) on redkiln-migrate — AC-441 makes the migrator a separate binary \
         nothing in the product reaches"
    );

    // ADR-0102 decision 6: anyhow is confined to the TWO binary crates — `rk-cli` AND
    // `redkiln-migrate`, which is what Checkpoint 7d's own walk asserts
    // (`assert anyhow == ["redkiln-migrate", "rk-cli"]`) and what ci.yml's confinement step at
    // :292 already allows. This crate's arrival is what makes that rule non-vacuous for the first
    // time: before it, the expected list was `["rk-cli"]` alone.
    // A workspace member is a package with no registry `source`. NOT `workspace_members`, whose
    // ids are `path+file:///…#0.0.0` [measured 2026-09-21] and are not names — matching a name
    // against one would make the set empty and the assertion below vacuous.
    let workspace: BTreeSet<&str> = packages
        .iter()
        .filter(|p| p.get("source").is_none_or(serde_json::Value::is_null))
        .filter_map(|p| p["name"].as_str())
        .collect();
    assert!(
        workspace.contains("rk-cli") && workspace.contains("redkiln-migrate"),
        "the workspace-member set did not resolve — the anyhow assertion below would be vacuous"
    );
    let anyhow_users: BTreeSet<&str> = deps
        .iter()
        .filter(|(name, d)| d.contains("anyhow") && workspace.contains(name.as_str()))
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        anyhow_users,
        ["redkiln-migrate", "rk-cli"]
            .into_iter()
            .collect::<BTreeSet<&str>>(),
        "anyhow's workspace users are not exactly the two binary crates (ADR-0102 d6)"
    );

    // ADR-0100 decision 1: redkiln-migrate is the ONE crate that may reach a YAML reader.
    let yaml_users: Vec<&str> = deps
        .iter()
        .filter(|(_, d)| d.contains("yaml_serde") || d.contains("serde_yaml"))
        .map(|(name, _)| name.as_str())
        .collect();
    assert_eq!(
        yaml_users,
        vec!["redkiln-migrate"],
        "the YAML dependency set is not exactly [redkiln-migrate] (ADR-0100 d1)"
    );
}
