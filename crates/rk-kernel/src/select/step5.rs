//! **Step 5 — validate.** Design §4.4 row 5:
//!
//! > Acyclic; every `requires` bound; every element of `R` reachable; `Σ cost_class` within the
//! > item's appetite; `excludes` respected. Emits
//! > `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped }`.
//! >
//! > *Invariant:* the engine **never trims, defers or drops** a member of `R` to fit the appetite —
//! > it gates, carrying the DAG, the appetite and the overage.
//!
//! # The appetite is a GATE, and the gate carries the whole graph
//!
//! AC-058 is a **negative** criterion and its instrument is a byte-identity comparison: *"unit test
//! asserting `R` is byte-identical before and after the appetite check on an over-budget fixture"*.
//! So an over-budget DAG is **not** an error — [`step5`] returns [`Step5Outcome::AppetiteExceeded`]
//! carrying the assembled [`Dag`], the appetite and the overage, and `R` never enters the
//! computation as anything but a read. There is no code path from the appetite check to
//! [`super::RequiredSet`], which is what makes *"never trims, defers, or drops"* a shape rather
//! than a promise.
//!
//! # The cost mapping is TRANSCRIBED, not defaulted
//!
//! `spikes/p9/lattice.py`:17 states invariant **I6** in terms: *"cost_class points **S=1 M=3 L=8
//! XL=20** summed over the selected DAG against `pack.appetite.points`"*, and
//! `spikes/p9/packs/bug-standard.toml`:5 carries the same mapping as a comment beside its own
//! `appetite`. That is the instrument F35 measured I6 against, so [`cost_points`] transcribes it
//! rather than choosing one.
//!
//! **What `spikes/DECISIONS-2026-09-08.md`:16 records as undecided is the *unit*** — *"appetite
//! units (I6, F35 §1.5)"* — not the ratios. The unit does not enter `Σ points > appetite.points`,
//! because both sides are in it. A cost class outside the four is **`None`**, not `0`: a zero would
//! make an unrecognised class free.
//!
//! And an **absent** `appetite` yields **no check at all** ([`Pack::appetite`] returns `None`),
//! never an unbounded sentinel — `lattice.py`:146's own `10**9` fallback is exactly the silent-pass
//! shape `00-preamble.md`:55 forbids.
//!
//! # AC-078 is a RUNTIME assertion and is deliberately not AC-059
//!
//! AC-059 is the **load-time** cycle lint over a catalog's `requires` relation
//! ([`super::requires_cycle_lint`]); AC-078 is the **step-5** assertion over an assembled graph,
//! *"driven by a hand-assembled cyclic DAG (constructed inside the crate, since the catalog lint of
//! AC-059 prevents one arising from a pack), asserting zero dispatches"*. The register split them
//! because *"a load-time catalog lint and a runtime graph assertion are different instruments, and
//! folding them let the runtime half inherit the lint's evidence."*
//!
//! [`Dag::hand_assembled`] is therefore `pub(in crate::select)` and AC-078's case lives at the
//! bottom of this file under `#[cfg(test)]`. A public constructor would put the case in
//! `tests/it/` at the price of letting anything build a `Dag`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::artifacts::Instance;
use super::canonical::Digest;
use super::step1::SkipRow;
use super::step4::Unsatisfiable;
use super::{Pack, SelectedProducer};
use crate::catalog::Catalog;

/// One node of the assembled DAG: one activity, for one key.
///
/// It carries every required instance the activity was chosen to produce for that key (R8 =
/// ADR-0179 decision 2, proposed; before 2026-09-27 this read *"one activity producing one required
/// instance"*).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DagNode {
    /// `activity_id` for an un-keyed instance, `activity_id[key]` for a keyed one. **Node identity
    /// carries the key**, because a mixed item runs the same activity once per toolchain and two
    /// nodes sharing an id would collapse into one (AC-049).
    pub id: String,
    pub activity_id: String,
    pub instance: Instance,
    /// *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 2, proposed.)* Every
    /// required instance this node's activity was chosen to produce, in byte order of
    /// `(kind, key)`. [`Self::instance`] is the first of them.
    pub instances: Vec<Instance>,
    pub cost_class: Option<String>,
    /// `None` when the activity declares no `cost_class`, or declares one outside the four. **Never
    /// `0`**: a zero would make an unrecognised class free and silently fit any appetite.
    pub cost_points: Option<i64>,
}

impl DagNode {
    fn to_json(&self) -> serde_json::Value {
        let mut out = serde_json::json!({
            "id": self.id,
            "activity_id": self.activity_id,
            "instance": self.instance.to_json(),
            "cost_class": self.cost_class,
            "cost_points": self.cost_points,
        });
        // R8 decision 2: `instances` is written only for a node carrying more than one, so every
        // graph that could be assembled before the fix (one instance per node) serialises
        // byte-identically to its recorded `dag_assembled` (AC-042's replay compares bytes).
        if self.instances.len() > 1
            && let Some(map) = out.as_object_mut()
        {
            map.insert(
                "instances".to_owned(),
                self.instances.iter().map(Instance::to_json).collect(),
            );
        }
        out
    }
}

/// One edge: `from` produces the artifact `kind` that `to` requires.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DagEdge {
    pub from: String,
    pub to: String,
    pub kind: String,
}

impl DagEdge {
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({ "from": self.from, "to": self.to, "kind": self.kind })
    }
}

/// What stopped step 5, or what the assembled graph could not satisfy.
///
/// **The appetite is not here.** An over-budget DAG is a gate carrying the graph, not a failure
/// (AC-058), and putting it in this enum would make *"carrying the DAG"* unwritable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step5Error {
    /// **AC-078.** The assembled graph is not acyclic. Validation fails **before any node is
    /// dispatched**, naming the cycle.
    CyclicDag { path: Vec<String> },
    /// An edge points at a node the graph does not carry — *"every `requires` bound"* failing at
    /// assembly rather than at load.
    UnboundEdge { edge: DagEdge },
    /// A member of `R` that is neither `Present` nor gated reached no node — *"every element of `R`
    /// reachable"*.
    UnreachableRequirement { instance: Instance },
    /// Two activities in one graph exclude each other (design §4.4 row 5's *"`excludes`
    /// respected"*). The **symmetry** of the declaration is AC-060's load-time lint; this is the
    /// runtime half over an assembled pair.
    ExcludedPair { a: String, b: String },
}

impl fmt::Display for Step5Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CyclicDag { path } => write!(
                f,
                "the assembled DAG is not acyclic: {} — validation fails BEFORE any node is \
                 dispatched, and no partially-ordered execution is attempted on a cyclic graph \
                 (AC-078)",
                path.join(" -> ")
            ),
            Self::UnboundEdge { edge } => write!(
                f,
                "edge {} -> {} ({}) names a node the graph does not carry: `requires` is not bound",
                edge.from, edge.to, edge.kind
            ),
            Self::UnreachableRequirement { instance } => write!(
                f,
                "{instance} is in R and reaches no node: every element of R must be reachable, and \
                 dropping it to make the graph assemble is exactly what AC-054 forbids"
            ),
            Self::ExcludedPair { a, b } => {
                write!(f, "{a} and {b} exclude each other and are both in the DAG")
            }
        }
    }
}

impl std::error::Error for Step5Error {}

/// The assembled activity graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dag {
    nodes: Vec<DagNode>,
    edges: Vec<DagEdge>,
    gated: Vec<Unsatisfiable>,
    skipped: Vec<SkipRow>,
    rows_fired: usize,
    inputs_digest: Digest,
    appetite: Option<i64>,
}

impl Dag {
    /// Every node, in byte order of node id.
    #[must_use]
    pub fn nodes(&self) -> &[DagNode] {
        &self.nodes
    }

    /// Every edge, in byte order of `(from, to, kind)`.
    #[must_use]
    pub fn edges(&self) -> &[DagEdge] {
        &self.edges
    }

    /// The kinds gated `Unsatisfiable`. They are **in** the graph as gates and were not dropped
    /// from `R` (AC-054, §7b.9(4)).
    #[must_use]
    pub fn gated(&self) -> &[Unsatisfiable] {
        &self.gated
    }

    /// `dag_assembled.skipped` — one row per forbidden stage, and one per kind a `requires` edge
    /// reached inside one (AC-077, §7b.9(7)).
    #[must_use]
    pub fn skipped(&self) -> &[SkipRow] {
        &self.skipped
    }

    /// How many `require` rows fired at step 2.
    #[must_use]
    pub const fn rows_fired(&self) -> usize {
        self.rows_fired
    }

    /// The four-digest tuple AC-041 names, in its order.
    #[must_use]
    pub const fn inputs_digest(&self) -> &Digest {
        &self.inputs_digest
    }

    /// `Σ cost_class` over the nodes whose class is one of the four. **Nodes with no recognised
    /// class are excluded and counted separately** by [`Self::uncosted`], because folding them in
    /// at 0 would let an unrecognised class fit any appetite.
    #[must_use]
    pub fn cost(&self) -> i64 {
        self.nodes.iter().filter_map(|n| n.cost_points).sum()
    }

    /// The nodes whose `cost_class` is absent or outside the four.
    #[must_use]
    pub fn uncosted(&self) -> Vec<&DagNode> {
        self.nodes
            .iter()
            .filter(|n| n.cost_points.is_none())
            .collect()
    }

    /// The pack's declared appetite, or `None` when it declared none — **which is not
    /// "unbounded"**.
    #[must_use]
    pub const fn appetite(&self) -> Option<i64> {
        self.appetite
    }

    /// **Acyclicity, over the assembled graph — AC-078.**
    ///
    /// Kahn's algorithm over `edges`; the residue after peeling every zero-in-degree node is the
    /// cycle, and it is **named**, not merely reported to exist.
    ///
    /// # Errors
    ///
    /// [`Step5Error::CyclicDag`] naming the cycle path, or [`Step5Error::UnboundEdge`] for an edge
    /// whose endpoints are not both nodes.
    pub fn validate_acyclic(&self) -> Result<(), Step5Error> {
        let ids: BTreeSet<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
        for edge in &self.edges {
            if !ids.contains(edge.from.as_str()) || !ids.contains(edge.to.as_str()) {
                return Err(Step5Error::UnboundEdge { edge: edge.clone() });
            }
        }
        match layers(&self.nodes, &self.edges) {
            Ok(_) => Ok(()),
            Err(path) => Err(Step5Error::CyclicDag { path }),
        }
    }

    /// `dag_assembled { inputs_digest, nodes, edges, rows_fired, skipped }` — design §4.4 row 5's
    /// five fields, as the closed [`rk_types::event_kind::EventKind::DagAssembled`] kind.
    ///
    /// Two fields beyond the five are carried and are named as additions rather than slipped in:
    /// `gated` (the `Unsatisfiable` kinds, which §7b.9(4) makes data in the graph) and `appetite`
    /// (`null` when the pack declared none, so the absence is legible).
    #[must_use]
    pub fn assembled_event(&self) -> rk_types::event::Event {
        rk_types::event::Event::new(
            rk_types::event_kind::EventKind::DagAssembled,
            self.to_json(),
        )
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "inputs_digest": self.inputs_digest.as_str(),
            "nodes": self.nodes.iter().map(DagNode::to_json).collect::<Vec<_>>(),
            "edges": self.edges.iter().map(DagEdge::to_json).collect::<Vec<_>>(),
            "rows_fired": self.rows_fired,
            "skipped": self.skipped.iter().map(SkipRow::to_json).collect::<Vec<_>>(),
            "gated": self.gated.iter().map(Unsatisfiable::to_json).collect::<Vec<_>>(),
            "appetite": self.appetite,
            "cost": self.cost(),
        })
    }

    /// **AC-078's constructor, and it is `pub(in crate::select)` on purpose.**
    ///
    /// The criterion's own *Verified by* says the cyclic DAG is *"constructed inside the crate,
    /// since the catalog lint of AC-059 prevents one arising from a pack"*. A `pub` constructor
    /// would move the test to `tests/it/` and, in the same stroke, let any caller hand the engine a
    /// graph selection never produced. It is `#[cfg(test)]` as well as private, so it does not exist
    /// at all in a shipped build.
    #[cfg(test)]
    pub(in crate::select) fn hand_assembled(nodes: Vec<DagNode>, edges: Vec<DagEdge>) -> Self {
        Self {
            nodes,
            edges,
            gated: Vec::new(),
            skipped: Vec::new(),
            rows_fired: 0,
            inputs_digest: super::canonical::digest_of(b"hand-assembled"),
            appetite: None,
        }
    }
}

/// **AC-058's gate.** *"A gate opens carrying the DAG, the appetite and the overage."*
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppetiteGate {
    pub dag: Dag,
    pub appetite: i64,
    pub cost: i64,
    pub overage: i64,
}

impl AppetiteGate {
    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "reason": "appetite_exceeded",
            "appetite": self.appetite,
            "cost": self.cost,
            "overage": self.overage,
            "dag": self.dag.to_json(),
        })
    }
}

/// What step 5 produced. **Both variants carry a DAG**, because AC-058's gate carries one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step5Outcome {
    Assembled(Dag),
    AppetiteExceeded(Box<AppetiteGate>),
}

impl Step5Outcome {
    /// The assembled graph, whichever variant this is.
    #[must_use]
    pub const fn dag(&self) -> &Dag {
        match self {
            Self::Assembled(d) => d,
            Self::AppetiteExceeded(g) => &g.dag,
        }
    }

    /// The gate, when the appetite was exceeded.
    #[must_use]
    pub fn gate(&self) -> Option<&AppetiteGate> {
        match self {
            Self::Assembled(_) => None,
            Self::AppetiteExceeded(g) => Some(g),
        }
    }
}

/// The I6 point mapping, transcribed from `spikes/p9/lattice.py`:17.
///
/// `None` for an absent or unrecognised class — **never `0`**.
#[must_use]
pub fn cost_points(cost_class: Option<&str>) -> Option<i64> {
    match cost_class {
        Some("S") => Some(1),
        Some("M") => Some(3),
        Some("L") => Some(8),
        Some("XL") => Some(20),
        _ => None,
    }
}

/// **Step 5.**
///
/// # Errors
///
/// [`Step5Error`] for a cyclic graph (AC-078), an unbound edge, an unreachable member of `R`, or a
/// pair of mutually excluding activities. **An over-budget DAG is not an error** — it is
/// [`Step5Outcome::AppetiteExceeded`].
#[expect(
    clippy::too_many_arguments,
    reason = "these nine ARE steps 1-4's outputs, one parameter each, and the design's row 5 is               written over exactly them. Bundling them behind a constructor would put a type               between the criterion and the call and would give a caller a way to assemble a DAG               from values no earlier step produced — which is the hole `RequiredSet`'s               `pub(in crate::select)` constructor exists to close"
)]
pub fn step5(
    pack: &Pack,
    catalog: &Catalog,
    producers: &[SelectedProducer],
    required: &super::RequiredSet,
    classifications: &BTreeMap<Instance, super::Classification>,
    gated: &[Unsatisfiable],
    skipped: &[SkipRow],
    rows_fired: usize,
    inputs_digest: &Digest,
) -> Result<Step5Outcome, Step5Error> {
    // ---- nodes --------------------------------------------------------------------------------
    //
    // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 2, proposed; D21-73.)*
    // **One node per activity, carrying every required kind it was chosen to produce** — and, as
    // AC-049 requires, one per KEY on a mixed item. This built one node per (instance, producer) and
    // `dedup`ed with the derived equality, which compares the instance too, so two nodes kept one id;
    // `layers` counts an id once, and the graph read as AC-078's cycle refusal with an empty path.
    //
    // The rule, per activity: with no keyed instance, one node `a`; with keyed instances, one node
    // `a[k]` per key, and the activity's UN-KEYED instances are carried by the node of its
    // byte-first key, so an activity is never dispatched once for a key and again for no key (the
    // shipped `docs-update` provides `Docs`, un-keyed, and `Changeset`, keyed). On a one-toolchain
    // item that is exactly one node per activity.
    let mut per_activity: BTreeMap<&str, Vec<&Instance>> = BTreeMap::new();
    for p in producers {
        let seen = per_activity.entry(p.activity_id.as_str()).or_default();
        if !seen
            .iter()
            .any(|i| i.kind == p.instance.kind && i.key == p.instance.key)
        {
            seen.push(&p.instance);
        }
    }
    let mut nodes: Vec<DagNode> = Vec::new();
    for (activity_id, instances) in per_activity {
        let keys: BTreeSet<&str> = instances.iter().filter_map(|i| i.key.as_deref()).collect();
        let first_key = keys.iter().next().copied();
        let activity = catalog.activities().iter().find(|a| a.id == activity_id);
        let cost_class = activity.and_then(|a| a.cost_class.clone());
        let slots: Vec<Option<&str>> = if keys.is_empty() {
            vec![None]
        } else {
            keys.iter().copied().map(Some).collect()
        };
        for slot in slots {
            let mut carried: Vec<Instance> = instances
                .iter()
                .filter(|i| i.key.as_deref() == slot || (i.key.is_none() && slot == first_key))
                .map(|i| (*i).clone())
                .collect();
            carried.sort();
            nodes.push(DagNode {
                id: node_id(activity_id, slot),
                activity_id: activity_id.to_owned(),
                instance: carried[0].clone(),
                instances: carried,
                cost_points: cost_points(cost_class.as_deref()),
                cost_class: cost_class.clone(),
            });
        }
    }
    nodes.sort();

    // ---- edges: producer of a required kind -> the activity that requires it -------------------
    //
    // The edge is drawn per KEY when both ends are keyed, so a mixed item's `node` summary depends
    // on the `node` changeset and not on the `unreal` one.
    let mut edges: BTreeSet<DagEdge> = BTreeSet::new();
    for node in &nodes {
        let Some(activity) = catalog
            .activities()
            .iter()
            .find(|a| a.id == node.activity_id)
        else {
            continue;
        };
        for requirement in &activity.requires {
            for upstream in nodes
                .iter()
                // R8 decision 2: a node produces every kind it carries, so the edge is drawn from it
                // for any of them, never only the first.
                .filter(|n| {
                    n.instances.iter().any(|i| {
                        i.kind == requirement.kind
                            // Same key, or one of the two ends is un-keyed.
                            && (i.key.is_none()
                                || node_key(node).is_none()
                                || i.key.as_deref() == node_key(node))
                    })
                })
            {
                edges.insert(DagEdge {
                    from: upstream.id.clone(),
                    to: node.id.clone(),
                    kind: requirement.kind.clone(),
                });
            }
        }
    }
    let edges: Vec<DagEdge> = edges.into_iter().collect();

    let dag = Dag {
        nodes,
        edges,
        gated: gated.to_vec(),
        skipped: skipped.to_vec(),
        rows_fired,
        inputs_digest: inputs_digest.clone(),
        appetite: pack.appetite(),
    };

    // ---- acyclic, and every `requires` bound ---------------------------------------------------
    dag.validate_acyclic()?;

    // ---- `excludes` respected ------------------------------------------------------------------
    for a in &dag.nodes {
        let Some(activity) = catalog.activities().iter().find(|x| x.id == a.activity_id) else {
            continue;
        };
        for b in &dag.nodes {
            if b.activity_id != a.activity_id && activity.excludes.contains(&b.activity_id) {
                return Err(Step5Error::ExcludedPair {
                    a: a.activity_id.clone(),
                    b: b.activity_id.clone(),
                });
            }
        }
    }

    // ---- every element of `R` reachable --------------------------------------------------------
    //
    // "Reachable" has three admissible answers and no fourth: a node produces it, step 3 classified
    // it `Present` (so nothing need produce it), or it is GATED (so the gate names it and it stayed
    // in `R`). Anything else is a member of `R` the graph silently lost, which is the failure
    // AC-054 and AC-058 both exist to refuse.
    for instance in required.kinds() {
        // **Identity is `(kind, key)` here too, and this was a real defect rather than a
        // precaution.** `Instance` carries `required_by` — provenance recording which activity's
        // `requires` pulled the instance in — and a member of `R` that step 4 reached through a
        // `requires` edge carries a `required_by` its `R` entry does not. Comparing whole
        // `Instance`s reported `Changeset[node]` as unreachable while its producer was in the graph
        // [measured: `UnreachableRequirement { kind: "Changeset", key: Some("node") }` against a DAG
        // holding `implement-change[node]`].
        let produced = dag.nodes.iter().any(|n| {
            n.instances
                .iter()
                .any(|i| i.kind == instance.kind && i.key == instance.key)
        });
        let present = classifications.get(instance) == Some(&super::Classification::Present);
        let is_gated = gated.iter().any(|g| g.kind == instance.kind);
        if !produced && !present && !is_gated {
            return Err(Step5Error::UnreachableRequirement {
                instance: instance.clone(),
            });
        }
    }

    // ---- Σ cost_class within the item's appetite -----------------------------------------------
    if let Some(appetite) = dag.appetite() {
        let cost = dag.cost();
        if cost > appetite {
            return Ok(Step5Outcome::AppetiteExceeded(Box::new(AppetiteGate {
                appetite,
                cost,
                overage: cost - appetite,
                dag,
            })));
        }
    }
    Ok(Step5Outcome::Assembled(dag))
}

/// The key a node was built for: that of any keyed instance it carries (they share one, R8
/// decision 2), or `None` for an un-keyed node.
fn node_key(node: &DagNode) -> Option<&str> {
    node.instances.iter().find_map(|i| i.key.as_deref())
}

/// `activity_id`, or `activity_id[key]` for a keyed instance.
pub(in crate::select) fn node_id(activity_id: &str, key: Option<&str>) -> String {
    key.map_or_else(|| activity_id.to_owned(), |k| format!("{activity_id}[{k}]"))
}

/// Kahn's algorithm, returning the antichain layers or the residual cycle.
///
/// Shared with [`super::step6`], because the topological order and the acyclicity check are one
/// computation and writing them twice is how the two come to disagree.
pub(in crate::select) fn layers(
    nodes: &[DagNode],
    edges: &[DagEdge],
) -> Result<Vec<Vec<String>>, Vec<String>> {
    let mut indegree: BTreeMap<&str, usize> = nodes.iter().map(|n| (n.id.as_str(), 0)).collect();
    let mut out: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in edges {
        *indegree.entry(edge.to.as_str()).or_insert(0) += 1;
        out.entry(edge.from.as_str()).or_default().push(&edge.to);
    }

    let mut result: Vec<Vec<String>> = Vec::new();
    let mut placed = 0_usize;
    loop {
        let ready: Vec<&str> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(id, _)| *id)
            .collect();
        if ready.is_empty() {
            break;
        }
        for id in &ready {
            indegree.remove(*id);
        }
        for id in &ready {
            for next in out.get(id).into_iter().flatten() {
                if let Some(d) = indegree.get_mut(*next) {
                    *d -= 1;
                }
            }
        }
        placed += ready.len();
        result.push(ready.into_iter().map(ToOwned::to_owned).collect());
    }

    if placed == nodes.len() {
        return Ok(result);
    }
    // The residue is the cycle. Walk it from the lowest surviving id so the path is deterministic
    // and names the loop rather than the set.
    let survivors: BTreeSet<&str> = indegree.keys().copied().collect();
    let start = survivors.iter().next().copied().unwrap_or("");
    let mut path = vec![start.to_owned()];
    let mut cursor = start;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    while seen.insert(cursor) {
        let Some(next) = out
            .get(cursor)
            .and_then(|ns| ns.iter().copied().find(|n| survivors.contains(n)))
        else {
            break;
        };
        path.push(next.to_owned());
        if next == start {
            break;
        }
        cursor = next;
    }
    Err(path)
}

/// **AC-078's case, inside the crate, for the reason the criterion states.**
#[cfg(test)]
mod tests {
    use super::*;

    fn node(id: &str) -> DagNode {
        DagNode {
            id: id.to_owned(),
            activity_id: id.to_owned(),
            instance: Instance::of(id),
            instances: vec![Instance::of(id)],
            cost_class: Some("S".to_owned()),
            cost_points: Some(1),
        }
    }

    fn edge(from: &str, to: &str) -> DagEdge {
        DagEdge {
            from: from.to_owned(),
            to: to.to_owned(),
            kind: "X".to_owned(),
        }
    }

    /// **AC-078, the negative criterion.** *"GIVEN an assembled DAG that is not acyclic / WHEN step
    /// 5 validates it / THEN validation fails **before any node is dispatched**, naming the cycle /
    /// AND no partially-ordered execution is attempted on a cyclic graph."*
    ///
    /// **"Zero dispatches" is measured, not asserted by inspection.** The only source of an
    /// execution order in this crate is [`super::super::step6`], and it is called here with a
    /// counter behind it: a cyclic graph yields no order at all, so the counter stays at 0. An
    /// implementation that returned a partial order — the plausible-looking repair — would emit
    /// nodes and move the counter.
    #[test]
    fn ac078_cyclic_dag_fails_before_dispatch() {
        let nodes = vec![node("a"), node("b"), node("c")];
        let edges = vec![edge("a", "b"), edge("b", "c"), edge("c", "a")];
        let dag = Dag::hand_assembled(nodes, edges);

        match dag.validate_acyclic() {
            Err(Step5Error::CyclicDag { path }) => {
                assert_eq!(
                    path,
                    vec![
                        "a".to_owned(),
                        "b".to_owned(),
                        "c".to_owned(),
                        "a".to_owned()
                    ],
                    "the cycle is NAMED, not merely detected"
                );
            }
            other => panic!("expected a named cycle, got {other:?}"),
        }

        // "No partially-ordered execution is attempted", counted.
        let mut dispatched = 0_usize;
        match super::super::step6(&dag) {
            Ok(plan) => {
                for layer in plan.layers() {
                    dispatched += layer.len();
                }
                panic!(
                    "step 6 produced an order over a cyclic graph and would have dispatched \
                     {dispatched} nodes"
                );
            }
            Err(Step5Error::CyclicDag { .. }) => {}
            Err(other) => panic!("expected a cycle refusal from step 6, got {other}"),
        }
        assert_eq!(dispatched, 0, "zero dispatches on a cyclic graph");

        // The acyclic control: the same three nodes without the back edge order fine, so the
        // refusal above is on the cycle and not on the graph's existence.
        let acyclic = Dag::hand_assembled(
            vec![node("a"), node("b"), node("c")],
            vec![edge("a", "b"), edge("b", "c")],
        );
        assert!(acyclic.validate_acyclic().is_ok());
        assert_eq!(
            super::super::step6(&acyclic)
                .expect("acyclic")
                .layers()
                .len(),
            3
        );
    }

    /// The cost mapping is the transcribed one, and an unrecognised class is `None` rather than 0.
    #[test]
    fn cost_points_transcribe_i6_and_refuse_a_zero_default() {
        assert_eq!(cost_points(Some("S")), Some(1));
        assert_eq!(cost_points(Some("M")), Some(3));
        assert_eq!(cost_points(Some("L")), Some(8));
        assert_eq!(cost_points(Some("XL")), Some(20));
        assert_eq!(cost_points(Some("XXL")), None);
        assert_eq!(cost_points(None), None);
    }
}
