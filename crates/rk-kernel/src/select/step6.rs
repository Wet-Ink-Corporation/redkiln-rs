//! **Step 6 — execute.** Design §4.4 row 6:
//!
//! > Topological order; antichains parallel where §7 allows.
//! >
//! > *Invariant:* the engine's own invariant is that execution **cannot change `R`** — step 7 is
//! > the only channel, and it re-enters at step 2.
//!
//! # 🛑 THIS STEP SHIPS UNCOVERED
//!
//! Design §4.4 row 6's criterion cell is not a criterion. It reads, in full:
//!
//! > **No criterion in the thirteen area files covers step 6.** §7.7 states four parallelism rules
//! > and names exactly one criterion (AC-388, the stagger); the sequential-within-a-tree,
//! > one-worktree-per-antichain-member and merge-queue rules are pinned by AC-189 and
//! > AC-196–AC-198 in the AC register's git-reconcile section, and **the execute step of the pure
//! > selection function itself is uncovered.** Recorded as a gap, not papered over.
//!
//! `s7b`:759-766 gives the instruction this module follows: *"**Build step 6 to its stated
//! invariant** — execution cannot change `R`; step 7 is the only channel and it re-enters at
//! step 2 — and **record in `PROGRESS.md` that it ships uncovered**."* That record is **OWED** and
//! is the caller's: `PROGRESS.md` is outside this session's editable set.
//!
//! **Nothing here is named after a criterion**, and no test of it is either. Naming one would claim
//! coverage the design records as absent.
//!
//! # What "cannot change `R`" means as a type fact
//!
//! [`step6`] takes `&Dag` and returns an [`ExecutionPlan`]. It does not take
//! [`super::RequiredSet`], it does not take `&mut` anything, and it has no return path to either.
//! The invariant is therefore a property of the signature rather than a rule the body obeys: there
//! is no expression inside this module that could edit `R` even if one were written.
//!
//! # What is deliberately NOT here
//!
//! Dispatch. The stagger (AC-388), the one-worktree-per-antichain-member rule (AC-196–AC-198) and
//! the merge queue are the **runtime's**, Slice 4's and `rk-git`'s, and none of them is a function
//! of the DAG alone. What this module produces is the **order**; who runs it is not this slice's.

use super::step5::{Dag, Step5Error, layers};

/// A topological order, grouped into antichains.
///
/// Each layer is a set of nodes with no edge between any two of them, so *"antichains parallel
/// where §7 allows"* has something to hand the runtime. Within a layer the ids are in byte order,
/// which makes the plan a function of the graph and not of iteration order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionPlan {
    layers: Vec<Vec<String>>,
}

impl ExecutionPlan {
    /// The antichains, earliest first.
    #[must_use]
    pub fn layers(&self) -> &[Vec<String>] {
        &self.layers
    }

    /// Every node id in a valid topological order.
    #[must_use]
    pub fn order(&self) -> Vec<&str> {
        self.layers
            .iter()
            .flat_map(|l| l.iter().map(String::as_str))
            .collect()
    }

    /// How many nodes the plan orders.
    #[must_use]
    pub fn len(&self) -> usize {
        self.layers.iter().map(Vec::len).sum()
    }

    /// `true` when the plan orders nothing — an empty DAG, which is a legitimate outcome for a
    /// stage whose whole `R` classified `Present`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // NOTE: there is no `to_json` here, and that is not an oversight. The execution PLAN is not in
    // the golden log: `dag_assembled` records the graph, and the order is a pure function of the
    // graph, so a recorded plan would be a second copy of the same fact and a second thing a replay
    // could diverge on. A field nothing consults reads as implemented.
}

/// **Step 6.** The topological order of an assembled DAG, as antichain layers.
///
/// # Errors
///
/// [`Step5Error::CyclicDag`] naming the cycle, or [`Step5Error::UnboundEdge`]. **A cyclic graph
/// yields no order at all** — not a partial one — which is what makes AC-078's *"no
/// partially-ordered execution is attempted on a cyclic graph"* countable.
pub fn step6(dag: &Dag) -> Result<ExecutionPlan, Step5Error> {
    dag.validate_acyclic()?;
    match layers(dag.nodes(), dag.edges()) {
        Ok(layers) => Ok(ExecutionPlan { layers }),
        Err(path) => Err(Step5Error::CyclicDag { path }),
    }
}
