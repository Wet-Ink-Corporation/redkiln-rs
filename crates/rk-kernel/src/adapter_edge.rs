//! **AC-484** — reading `cargo metadata` for the one thing the criterion asks: does `rk-kernel`
//! reach an `rk-adapter-*` crate, directly or transitively?
//!
//! AC-484: *"GIVEN the kernel and the adapters / WHEN the dependency graph is read / THEN
//! `rk-kernel` has no dependency edge — direct or transitive — on any `rk-adapter-*` crate / AND an
//! added edge fails CI **naming the adapter crate**."*
//!
//! # 🛑 `resolve`, NEVER `packages` — and the reason the distinction is load-bearing here
//!
//! `cargo metadata` emits two different relations under two different keys, and on this tree they
//! **disagree**:
//!
//! * `packages[].dependencies[]` is the **manifest** view. It is what the `Cargo.toml` says, and it
//!   lists an **optional** dependency whether or not the feature that enables it is on. It is not a
//!   graph: it has no transitive closure and no feature resolution.
//! * `resolve.nodes[].deps[]` is the **resolved** graph. Its edges are the ones the resolver
//!   actually selected under the feature selection in force, and it is the relation `cargo build`
//!   and `cargo tree` walk.
//!
//! D-1 (2026-09-14) puts an **optional, feature-gated** edge from `rk-kernel` to
//! `tests/fixtures/rk-adapter-sqlite/` so that the criterion's *"an added edge fails"* half is a
//! committed, permanently-checkable fixture rather than a commit somebody makes and reverts. That
//! edge is therefore **permanently present in the manifest view**, and an assertion written over
//! `packages[]` would report a violation on a green tree — making AC-484's THEN unprovable and, in
//! practice, making the check the thing that gets deleted. [`resolved_reading`] is the reading that
//! answers the criterion; [`manifest_reading`] exists **only** so a test can assert the two
//! disagree and pin the trap in place.
//!
//! # The second trap: `deps[].name` is the LIB name
//!
//! Inside `resolve.nodes[].deps[]`, `name` is the **crate** (lib) name — `rk_adapter_sqlite`, with
//! underscores — and `pkg` is the package id. AC-484's THEN is *"naming the adapter **crate**"*,
//! and the name a CI reader needs in order to find and cut the edge is the **package** name as it
//! is spelled in `Cargo.toml`. So every walk here resolves `pkg` back through `packages[]` and
//! reports that. Matching on `deps[].name` would also silently miss the `rk-adapter-` prefix, which
//! contains a hyphen the lib name never does.
//!
//! # What this module does NOT do
//!
//! It spawns nothing. Every function takes `cargo metadata`'s JSON as a `&str`. The child process
//! lives in the test binary (`crates/rk-kernel/tests/it/adapter_edge.rs`), on
//! `crates/rk-store/tests/it/contention.rs`:77-85's precedent, so no `std::process::Command`
//! enters `rk-kernel`'s own sources.
//!
//! **Edges are walked with `kind: null` only** — normal dependencies, the same set
//! `cargo tree -e normal` shows and the same set `crates/rk-kernel/Cargo.toml`'s closing comment
//! names. Dev-dependencies are excluded deliberately: they are not in any consumer's graph, and
//! `rk-kernel`'s own dev-dependencies (`trybuild`, and the repeats of `rk-git`/`rk-store`/
//! `rk-types`) would otherwise be reported as edges of the kernel that a `cargo build` never takes.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use serde_json::Value;

use crate::kernel_denylist::{DENYLIST, DenyClause};

/// The prefix every adapter crate's **package** name carries (ADR-0003; AC-484, AC-487) — read
/// **out of AC-487's denylist table** rather than spelled again here.
///
/// # This is not a flourish. It was forced, and the forcing is worth recording.
///
/// The first draft of this module wrote the literal out, and `ac487_kernel_denylist` went red
/// against it [measured 2026-09-14]:
///
/// ```text
/// crates/rk-kernel/src/adapter_edge.rs:57: AC-487 (no adapter name string): the denied token
///   `rk-adapter-` appears in kernel code -- the register's own first entry
/// ```
///
/// That is the lint working, not misfiring. AC-487's first clause is *"no adapter name string"* in
/// `rk-kernel`, and a module that needs one to do its job is exactly the case the rule's own header
/// anticipated when it opened **one** marked, counted exempt region — a region
/// `tests/it/kernel_denylist.rs`:328-341 pins to `kernel_denylist.rs` and to a marker count of
/// **2**, so a second region here would not be a local decision but an amendment to AC-487's
/// instrument. That is not this stage's to make.
///
/// Reading the string out of the one place it is already exempt costs nothing and buys the property
/// AC-484's third *Verified by* clause asked for and could not deliver through `deny.toml` (see the
/// OWED erratum in `tests/it/adapter_edge.rs`): **the two instruments read one committed list**.
/// A future editor who adds `rk-adapter2-` to the denylist changes both rules at once, and one who
/// reorders the table hits the const assertions below at compile time rather than silently giving
/// this walk a prefix that matches nothing.
pub const ADAPTER_CRATE_PREFIX: &str = DENYLIST[0].pattern;

// The three things the line above assumes about the table it reads, checked at COMPILE time so a
// reorder or a re-spelling of `kernel_denylist`'s first row cannot hand this module a prefix that
// silently matches no package — which would make every `resolved_reading` green vacuous.
const _: () = assert!(matches!(DENYLIST[0].clause, DenyClause::AdapterCrateName));
const _: () = assert!(DENYLIST[0].pattern.len() == 11);
// The HYPHENATED spelling, not the lib-name form: byte 2 and the last byte are `-`. The table
// carries both spellings and only this one matches a package name.
const _: () = assert!(DENYLIST[0].pattern.as_bytes()[2] == b'-');
const _: () = assert!(DENYLIST[0].pattern.as_bytes()[10] == b'-');

/// The crate whose graph AC-484 is about.
pub const SUBJECT_CRATE: &str = "rk-kernel";

/// Which of `cargo metadata`'s two relations a report was read from.
///
/// Carried on the report rather than left implicit because the whole criterion turns on it: a
/// report that does not say which relation it read cannot be audited, and the two disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading {
    /// `resolve.nodes[].deps[]` — feature-resolved, transitive. **The reading AC-484 asks for.**
    Resolved,
    /// `packages[].dependencies[]` — the manifest view: optional edges included, no closure.
    /// Present so a test can assert it disagrees with [`Reading::Resolved`]; never the instrument.
    Manifest,
}

impl fmt::Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Resolved => f.write_str("resolve.nodes[].deps[] (feature-resolved)"),
            Self::Manifest => f.write_str("packages[].dependencies[] (manifest view)"),
        }
    }
}

/// One reached `rk-adapter-*` crate and the route by which it was reached.
///
/// The route is package names from the subject inclusive to the adapter inclusive. AC-484 covers
/// **transitive** edges, so a finding that named only the adapter would be unactionable: the reader
/// would know an adapter is in the graph and not which edge to cut.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterEdge {
    /// The **package** name, hyphenated, as `Cargo.toml` spells it.
    pub adapter_crate: String,
    /// `[subject, …, adapter_crate]`.
    pub route: Vec<String>,
}

impl fmt::Display for AdapterEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} via {}", self.adapter_crate, self.route.join(" -> "))
    }
}

/// One reading of one crate's dependency relation.
#[derive(Debug, Clone)]
pub struct AdapterEdgeReport {
    /// The package the walk started at.
    pub subject: String,
    /// Which relation was read.
    pub reading: Reading,
    /// How many distinct packages the walk visited, the subject included.
    ///
    /// Asserted against a floor by the test: a walk that visited three nodes would report "no
    /// adapter edge" for entirely the wrong reason, and a report with no size on it cannot be told
    /// apart from one.
    pub packages_walked: usize,
    /// Every package the walk reached, with its route. Ordered by package name.
    pub reached: Vec<(String, Vec<String>)>,
    /// The subset of [`Self::reached`] whose names start with [`ADAPTER_CRATE_PREFIX`].
    pub findings: Vec<AdapterEdge>,
}

impl AdapterEdgeReport {
    /// The route by which `name` was reached, if it was.
    #[must_use]
    pub fn route_to(&self, name: &str) -> Option<&[String]> {
        self.reached
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, r)| r.as_slice())
    }

    /// Every package name the walk reached, for a failure message that has to show what it saw.
    #[must_use]
    pub fn reached_names(&self) -> Vec<&str> {
        self.reached.iter().map(|(n, _)| n.as_str()).collect()
    }

    /// AC-484's THEN as a verdict.
    ///
    /// # Errors
    /// [`AdapterEdgeViolation`] if any `rk-adapter-*` crate is reachable, carrying every finding
    /// and its route so the rendered message **names the adapter crate**.
    pub fn verdict(&self) -> Result<(), AdapterEdgeViolation> {
        if self.findings.is_empty() {
            Ok(())
        } else {
            Err(AdapterEdgeViolation {
                subject: self.subject.clone(),
                reading: self.reading,
                findings: self.findings.clone(),
            })
        }
    }
}

/// AC-484's failure, rendered so CI names the adapter crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterEdgeViolation {
    pub subject: String,
    pub reading: Reading,
    pub findings: Vec<AdapterEdge>,
}

impl fmt::Display for AdapterEdgeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AC-484: `{}` reaches {} adapter crate(s) in {}",
            self.subject,
            self.findings.len(),
            self.reading
        )?;
        for finding in &self.findings {
            write!(f, "\n  - {finding}")?;
        }
        Ok(())
    }
}

impl std::error::Error for AdapterEdgeViolation {}

/// Why a reading could not be taken at all — distinct from a reading that found an edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphReadError {
    /// `cargo metadata`'s output did not parse as JSON.
    Json(String),
    /// A key the format guarantees was missing or the wrong type. A *shape* change in
    /// `cargo metadata`, which must be loud rather than read as "no adapter edge".
    Shape(&'static str),
    /// No package by that name in the graph.
    NoSuchPackage(String),
}

impl fmt::Display for GraphReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(detail) => write!(f, "`cargo metadata` output did not parse: {detail}"),
            Self::Shape(key) => write!(
                f,
                "`cargo metadata` output is missing `{key}` or it has the wrong type; the format \
                 changed and this reading is no longer valid"
            ),
            Self::NoSuchPackage(name) => {
                write!(f, "no package named `{name}` in the resolved graph")
            }
        }
    }
}

impl std::error::Error for GraphReadError {}

/// `packages[]` as `id -> name`.
fn package_names(root: &Value) -> Result<BTreeMap<String, String>, GraphReadError> {
    let packages = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("packages"))?;
    let mut by_id = BTreeMap::new();
    for pkg in packages {
        let id = pkg
            .get("id")
            .and_then(Value::as_str)
            .ok_or(GraphReadError::Shape("packages[].id"))?;
        let name = pkg
            .get("name")
            .and_then(Value::as_str)
            .ok_or(GraphReadError::Shape("packages[].name"))?;
        by_id.insert(id.to_owned(), name.to_owned());
    }
    Ok(by_id)
}

/// The id of the one package named `name`.
fn package_id(by_id: &BTreeMap<String, String>, name: &str) -> Result<String, GraphReadError> {
    by_id
        .iter()
        .find(|(_, n)| n.as_str() == name)
        .map(|(id, _)| id.clone())
        .ok_or_else(|| GraphReadError::NoSuchPackage(name.to_owned()))
}

/// Does this `deps[]` entry carry a **normal** (`kind: null`) dependency kind?
///
/// `dep_kinds` is a list because one package can be a normal *and* a dev dependency of the same
/// parent — `rk-kernel`'s own `rk-git` edge is exactly that. Reading the first entry, or reading
/// the list as a scalar, would classify it by whichever order cargo happened to emit.
fn is_normal_edge(dep: &Value) -> bool {
    dep.get("dep_kinds")
        .and_then(Value::as_array)
        .is_some_and(|kinds| {
            kinds
                .iter()
                .any(|k| k.get("kind").is_none_or(Value::is_null))
        })
}

/// `resolve.nodes[]` as `id -> [normal dep ids]`.
fn resolved_edges(root: &Value) -> Result<BTreeMap<String, Vec<String>>, GraphReadError> {
    let nodes = root
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("resolve.nodes"))?;
    let mut edges = BTreeMap::new();
    for node in nodes {
        let id = node
            .get("id")
            .and_then(Value::as_str)
            .ok_or(GraphReadError::Shape("resolve.nodes[].id"))?;
        let deps = node
            .get("deps")
            .and_then(Value::as_array)
            .ok_or(GraphReadError::Shape("resolve.nodes[].deps"))?;
        let mut out = Vec::new();
        for dep in deps {
            if !is_normal_edge(dep) {
                continue;
            }
            let pkg = dep
                .get("pkg")
                .and_then(Value::as_str)
                .ok_or(GraphReadError::Shape("resolve.nodes[].deps[].pkg"))?;
            out.push(pkg.to_owned());
        }
        edges.insert(id.to_owned(), out);
    }
    Ok(edges)
}

fn parse(metadata_json: &str) -> Result<Value, GraphReadError> {
    serde_json::from_str(metadata_json).map_err(|e| GraphReadError::Json(e.to_string()))
}

fn findings_from(reached: &[(String, Vec<String>)]) -> Vec<AdapterEdge> {
    reached
        .iter()
        .filter(|(name, _)| name.starts_with(ADAPTER_CRATE_PREFIX))
        .map(|(name, route)| AdapterEdge {
            adapter_crate: name.clone(),
            route: route.clone(),
        })
        .collect()
}

/// **The reading AC-484 asks for.** Walk `resolve.nodes[]` from `subject` over normal edges,
/// transitively, and report every `rk-adapter-*` package reached and the route to it.
///
/// Breadth-first, so the recorded route to each package is a shortest one — the edge a reader would
/// cut first. Ordering is deterministic: the frontier is drained in insertion order and the maps
/// are `BTreeMap`s, so two runs over the same bytes produce byte-identical routes.
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `subject` is not in the
/// graph.
pub fn resolved_reading(
    metadata_json: &str,
    subject: &str,
) -> Result<AdapterEdgeReport, GraphReadError> {
    let root = parse(metadata_json)?;
    let names = package_names(&root)?;
    let edges = resolved_edges(&root)?;
    let start = package_id(&names, subject)?;

    let mut seen: BTreeSet<String> = BTreeSet::new();
    seen.insert(start.clone());
    let mut frontier: VecDeque<(String, Vec<String>)> = VecDeque::new();
    frontier.push_back((start, vec![subject.to_owned()]));

    let mut reached: Vec<(String, Vec<String>)> = Vec::new();
    let mut walked = 0usize;

    while let Some((id, route)) = frontier.pop_front() {
        walked += 1;
        for dep_id in edges.get(&id).map_or(&[][..], Vec::as_slice) {
            if !seen.insert(dep_id.clone()) {
                continue;
            }
            // A dep id with no `packages[]` entry is a shape change, not a silent skip: it would
            // hide an adapter behind an unresolvable id.
            let dep_name = names
                .get(dep_id)
                .ok_or(GraphReadError::Shape("resolve.nodes[].deps[].pkg"))?;
            let mut dep_route = route.clone();
            dep_route.push(dep_name.clone());
            reached.push((dep_name.clone(), dep_route.clone()));
            frontier.push_back((dep_id.clone(), dep_route));
        }
    }

    reached.sort();
    let findings = findings_from(&reached);
    Ok(AdapterEdgeReport {
        subject: subject.to_owned(),
        reading: Reading::Resolved,
        packages_walked: walked,
        reached,
        findings,
    })
}

/// **The reading AC-484 must NOT be asserted over.**
///
/// `packages[name == subject].dependencies[]` — the manifest view, which lists an optional
/// dependency whether or not its feature is enabled, and which has no transitive closure at all.
///
/// It exists so a test can assert that this reading and [`resolved_reading`] **disagree** on this
/// tree. If they ever agree, either D-1's fixture edge was deleted from
/// `crates/rk-kernel/Cargo.toml` or cargo changed what `packages[].dependencies` means — and in
/// either case the resolved-graph green has quietly stopped proving anything.
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `subject` is not in the
/// graph.
pub fn manifest_reading(
    metadata_json: &str,
    subject: &str,
) -> Result<AdapterEdgeReport, GraphReadError> {
    let root = parse(metadata_json)?;
    let packages = root
        .get("packages")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("packages"))?;
    let pkg = packages
        .iter()
        .find(|p| p.get("name").and_then(Value::as_str) == Some(subject))
        .ok_or_else(|| GraphReadError::NoSuchPackage(subject.to_owned()))?;
    let deps = pkg
        .get("dependencies")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("packages[].dependencies"))?;

    let mut reached: Vec<(String, Vec<String>)> = Vec::new();
    for dep in deps {
        // `kind` is absent/null for a normal dependency, "dev" or "build" otherwise — the same
        // filter the resolved walk applies, so the two readings differ ONLY in the thing under
        // test rather than in which edge kinds they count.
        if !dep.get("kind").is_none_or(Value::is_null) {
            continue;
        }
        let name = dep
            .get("name")
            .and_then(Value::as_str)
            .ok_or(GraphReadError::Shape("packages[].dependencies[].name"))?;
        reached.push((name.to_owned(), vec![subject.to_owned(), name.to_owned()]));
    }
    reached.sort();
    reached.dedup();
    let findings = findings_from(&reached);
    Ok(AdapterEdgeReport {
        subject: subject.to_owned(),
        reading: Reading::Manifest,
        // The manifest view is one hop and never a closure. Saying `1` rather than
        // `reached.len()` keeps that visible on the report itself.
        packages_walked: 1,
        reached,
        findings,
    })
}

/// How many **normal** dependencies `package` has in the resolved graph.
///
/// D-1: the fixture crate must pull nothing. An optional dependency enters `Cargo.lock` whether or
/// not it is enabled, and `cargo audit` / `cargo deny` read the lock, so a fixture with a
/// dependency tree puts that tree under two gates for no benefit
/// (`tests/fixtures/rk-banned-dep/Cargo.toml` records the same reasoning for its own pick).
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `package` is not in the
/// graph.
pub fn resolved_dep_count(metadata_json: &str, package: &str) -> Result<usize, GraphReadError> {
    let root = parse(metadata_json)?;
    let names = package_names(&root)?;
    let id = package_id(&names, package)?;
    let edges = resolved_edges(&root)?;
    edges
        .get(&id)
        .map(Vec::len)
        .ok_or(GraphReadError::Shape("resolve.nodes"))
}

/// Is `package` a **workspace member**?
///
/// D-1's reason for a member rather than an excluded package, restated: Step 8 (4)'s
/// `cargo metadata --no-deps` membership check resolves for a member and would not for an excluded
/// one, and the root `Cargo.toml`'s `members = ["crates/*", "tests/fixtures/*"]` already covers the
/// fixture directory.
///
/// # Errors
/// [`GraphReadError`] if the output does not parse, its shape changed, or `package` is not in the
/// graph.
pub fn is_workspace_member(metadata_json: &str, package: &str) -> Result<bool, GraphReadError> {
    let root = parse(metadata_json)?;
    let names = package_names(&root)?;
    let id = package_id(&names, package)?;
    let members = root
        .get("workspace_members")
        .and_then(Value::as_array)
        .ok_or(GraphReadError::Shape("workspace_members"))?;
    Ok(members
        .iter()
        .filter_map(Value::as_str)
        .any(|m| m == id.as_str()))
}
