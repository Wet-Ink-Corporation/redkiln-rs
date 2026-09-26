//! The package-level affected set (s8c §8c.3; record T = ADR-0169 decision 7 rules 3 and 4,
//! proposed; AC-347).
//!
//! **A pure function.** [`affected_packages`] takes the changed paths, the workspace [`Metadata`]
//! read BEFORE it is called, and a policy version, and returns an [`AffectedSet`]. It reads no tree,
//! no clock and no environment, and spawns nothing: AC-347 requires *"a pure function of the
//! changed-file set, the toolchain config and `facets.toml`"*, and `s8g`'s AC-348 test
//! (`selection::ac348_selection_is_byte_identical_over_200_change_sets`) stops `s8g` if this is
//! impure. The adapter reads `cargo metadata --format-version 1` (record T decision 7 rule 3's own
//! read, D21-51; a structured, stable read, never prose) and parses it with [`Metadata::parse`],
//! also pure, over the JSON value and the project root.
//!
//! **Rule 3, the longest prefix.** Every changed path maps to the workspace member whose manifest
//! directory is its longest prefix, compared by PATH COMPONENT (so `ab/x.rs` is never `a`'s). A path
//! in no member, and the workspace-level files whatever member the root holds (the root
//! `Cargo.toml`, `Cargo.lock`, `rust-toolchain*`, anything under `.cargo/`), makes the set
//! [`AffectedSet::WholeSuite`] naming the first such path: the adapter runs the whole suite with
//! `PolicyForced` (record T's reading of that reason, flagged for the operator's flip).
//!
//! **Rule 4, the closure.** The set is the touched members plus their reverse-dependency closure over
//! `resolve.nodes`, **every dependency kind followed, dev-dependencies included** (a test in `c`
//! that dev-depends on `b` is affected by a change to `b`), restricted to workspace members. The
//! adapter turns it into `-p <name>` flags in place of `--workspace` (record T's supersession in
//! part of ADR-0094 decision 1's *"`test` = `cargo test --workspace`"*).
//!
//! *(Added 2026-09-26, s8c build, P5.)*

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The policy version the adapter passes: this module's rules 3 and 4 as record T fixes them. A
/// change to either rule is a new version, so an `AffectedSet` names the rules it was computed by.
pub const AFFECTED_POLICY: &str = "rust-affected/1";

/// One workspace member: its package name and its manifest directory relative to the project
/// root, `/`-separated (`""` for a member at the root).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub dir: String,
}

/// What [`affected_packages`] reads from `cargo metadata --format-version 1`: the workspace members
/// and the dependency edges between them (`dependent` → `dependency`, every kind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
    pub members: Vec<Member>,
    /// `(dependent id, dependency id)`, both workspace members.
    pub edges: Vec<(String, String)>,
}

/// The affected set (record T decision 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AffectedSet {
    /// The members the change touched, and the set to run: those plus their reverse-dependency
    /// closure, sorted by name.
    Packages {
        touched: Vec<String>,
        packages: Vec<String>,
        policy_version: String,
    },
    /// A changed path in no package (rule 3): the whole suite, `PolicyForced`.
    WholeSuite {
        path: String,
        policy_version: String,
    },
}

/// A path as the comparison reads it: `/`-separated, no `./` lead, no trailing `/`.
fn normal(path: &str) -> String {
    let mut p = path.replace('\\', "/");
    while let Some(rest) = p.strip_prefix("./") {
        p = rest.to_owned();
    }
    p.trim_end_matches('/').to_owned()
}

/// `dir` relative to `root`, both normalised; ASCII case is ignored on Windows, where the file
/// system does. `None` when `dir` is not under `root`.
fn relative_to(dir: &str, root: &str) -> Option<String> {
    let (d, r) = (normal(dir), normal(root));
    let same = |a: &str, b: &str| {
        if cfg!(windows) {
            a.eq_ignore_ascii_case(b)
        } else {
            a == b
        }
    };
    if same(&d, &r) {
        return Some(String::new());
    }
    let head = d.get(..r.len())?;
    if same(head, &r) && d.as_bytes().get(r.len()) == Some(&b'/') {
        return Some(d[r.len() + 1..].to_owned());
    }
    None
}

/// Does `path` lie in `dir` by path component (`dir` = `""` holds every relative path)?
fn in_dir(path: &str, dir: &str) -> bool {
    dir.is_empty()
        || path == dir
        || (path.len() > dir.len()
            && path.starts_with(dir)
            && path.as_bytes().get(dir.len()) == Some(&b'/'))
}

/// A workspace-level file that forces the whole suite whatever member the root holds (rule 3).
fn workspace_level(path: &str) -> bool {
    path == "Cargo.toml"
        || path == "Cargo.lock"
        || path.starts_with("rust-toolchain")
        || path == ".cargo"
        || path.starts_with(".cargo/")
}

impl Metadata {
    /// Parse `cargo metadata --format-version 1`'s JSON: the workspace members (from
    /// `workspace_members` and `packages`), each member's manifest directory relative to
    /// `project_root`, and the member-to-member edges of `resolve.nodes` (every `deps` entry, of
    /// every kind). Pure.
    ///
    /// # Errors
    /// A message naming the member that is missing or malformed.
    pub fn parse(value: &serde_json::Value, project_root: &Path) -> Result<Self, String> {
        let root = project_root.to_string_lossy();
        let ids: BTreeSet<&str> = value
            .get("workspace_members")
            .and_then(serde_json::Value::as_array)
            .ok_or("`cargo metadata` carries no `workspace_members` array")?
            .iter()
            .filter_map(serde_json::Value::as_str)
            .collect();
        let packages = value
            .get("packages")
            .and_then(serde_json::Value::as_array)
            .ok_or("`cargo metadata` carries no `packages` array")?;
        let mut members = Vec::new();
        for p in packages {
            let Some(id) = p.get("id").and_then(serde_json::Value::as_str) else {
                return Err("a `cargo metadata` package carries no `id`".to_owned());
            };
            if !ids.contains(id) {
                continue;
            }
            let name = p
                .get("name")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("the package `{id}` carries no `name`"))?;
            let manifest = p
                .get("manifest_path")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| format!("the package `{id}` carries no `manifest_path`"))?;
            let manifest = normal(manifest);
            let dir = manifest
                .strip_suffix("/Cargo.toml")
                .ok_or_else(|| format!("the manifest `{manifest}` is not a Cargo.toml"))?;
            // A member outside the project root keeps its absolute directory, which no relative
            // changed path lies in.
            let dir = relative_to(dir, &root).unwrap_or_else(|| dir.to_owned());
            members.push(Member {
                id: id.to_owned(),
                name: name.to_owned(),
                dir,
            });
        }
        let nodes = value
            .get("resolve")
            .and_then(|r| r.get("nodes"))
            .and_then(serde_json::Value::as_array)
            .ok_or("`cargo metadata` carries no `resolve.nodes` (was it read with --no-deps?)")?;
        let mut edges = Vec::new();
        for node in nodes {
            let Some(id) = node.get("id").and_then(serde_json::Value::as_str) else {
                return Err("a `resolve.nodes` entry carries no `id`".to_owned());
            };
            if !ids.contains(id) {
                continue;
            }
            let deps = node
                .get("deps")
                .and_then(serde_json::Value::as_array)
                .ok_or_else(|| format!("the node `{id}` carries no `deps`"))?;
            for dep in deps {
                let Some(pkg) = dep.get("pkg").and_then(serde_json::Value::as_str) else {
                    return Err(format!("a dependency of `{id}` carries no `pkg`"));
                };
                if ids.contains(pkg) {
                    edges.push((id.to_owned(), pkg.to_owned()));
                }
            }
        }
        Ok(Self { members, edges })
    }
}

/// The affected set for `changed` (paths relative to the project root) over `metadata` (the module
/// doc). Pure: no tree, clock, environment or spawn.
#[must_use]
pub fn affected_packages(
    changed: &[String],
    metadata: &Metadata,
    policy_version: &str,
) -> AffectedSet {
    let mut touched: BTreeSet<&str> = BTreeSet::new();
    for raw in changed {
        let path = normal(raw);
        let absolute = path.starts_with('/') || path.as_bytes().get(1) == Some(&b':');
        let owner = if absolute || path.split('/').any(|c| c == "..") || workspace_level(&path) {
            None
        } else {
            metadata
                .members
                .iter()
                .filter(|m| in_dir(&path, &m.dir))
                .max_by_key(|m| m.dir.len())
        };
        match owner {
            Some(member) => {
                touched.insert(member.id.as_str());
            }
            None => {
                return AffectedSet::WholeSuite {
                    path: raw.clone(),
                    policy_version: policy_version.to_owned(),
                };
            }
        }
    }
    // Reverse edges: dependency -> its dependents.
    let mut dependents: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (dependent, dependency) in &metadata.edges {
        dependents
            .entry(dependency.as_str())
            .or_default()
            .push(dependent.as_str());
    }
    let mut closure: BTreeSet<&str> = touched.clone();
    let mut frontier: Vec<&str> = touched.iter().copied().collect();
    while let Some(id) = frontier.pop() {
        for &next in dependents.get(id).map_or(&[][..], Vec::as_slice) {
            if closure.insert(next) {
                frontier.push(next);
            }
        }
    }
    let name_of = |id: &str| {
        metadata
            .members
            .iter()
            .find(|m| m.id == id)
            .map_or_else(|| id.to_owned(), |m| m.name.clone())
    };
    let mut touched: Vec<String> = touched.iter().map(|id| name_of(id)).collect();
    touched.sort();
    let mut packages: Vec<String> = closure.iter().map(|id| name_of(id)).collect();
    packages.sort();
    packages.dedup();
    AffectedSet::Packages {
        touched,
        packages,
        policy_version: policy_version.to_owned(),
    }
}
