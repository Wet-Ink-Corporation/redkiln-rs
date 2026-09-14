//! **Step 3 — short-circuit.** Design §4.4 row 3: per artifact, `Present` / `Stale` / `Missing`,
//! decided on `derived_from` digests.
//!
//! # The AC-066 / AC-067 tie, and how it is broken here
//!
//! §7b.9(6) records that the two criteria **classify one artifact two ways**, and that the register
//! alone forces the wrong reading. For a human-supplied artifact of a kind that has a declared
//! input, `derived_from = []` is *simultaneously*:
//!
//! * the **vouched-root shape** — AC-067's GIVEN, whose THEN is `Present`; and
//! * *"no recorded `derived_from` entry for a declared input"* — AC-066's second AND, whose THEN is
//!   `Stale`.
//!
//! **The resolution, and it is not a new decision.** `derived_from = []` is **not** the vouched-root
//! test — the **registered kind's `source` is**. AC-067's GIVEN conjoins `source = "human"`, and
//! AC-053 with ADR-0057's Decision make `artifacts.toml` the authority over kinds. So the order is:
//!
//! 1. registered `source = "human"` → the vouched-root path: `Present`, plus **at most an advisory
//!    that can never promote itself**;
//! 2. otherwise → AC-066's rules, absence included: `Stale`.
//!
//! **What the other order costs, stated because it is why this one is taken.** An implementer who
//! applies AC-066's blanket absence rule first re-dispatches a producer against a human-authored
//! artifact **on every run** — which AC-067's own THEN forbids by name — and suppresses the
//! advisory path entirely, **making AC-525 dead code**. The discriminating fixtures are a **register
//! edit and are OWED** (§7b.9(6)): AC-066's 8-case table gains a human-sourced kind carrying a
//! declared input (must be `Present`) and a machine-sourced kind of the same shape (must be
//! `Stale`). Until that lands, **AC-066 is ARMED, NOT DISCHARGED**, and the pair is pinned here by
//! `short_circuit::registered_source_breaks_the_ac066_ac067_tie` against fixture kinds this crate
//! mints.
//!
//! # What this step does NOT do — D4, and AC-525's 2026-09-13 amendment
//!
//! It **returns** the classification and, for a vouched root whose in-set upstream moved, a
//! [`DriftSignal`]. It does **not** construct `drift_advisory`, does **not** append it, and does
//! **not** read the event log to decide whether one was appended before. Clause 1 of that amendment
//! says so in terms; clause 2 puts the derivation and the append **outside** the pure function, at
//! the kernel's step-3 boundary or in `reconcile()`; clause 3 makes the *"exactly one"* **run-scoped
//! with an in-process de-dup memory at the append site**, so no log read happens anywhere.
//!
//! The type is the enforcement: [`step3`] takes a registry, a state and two slices, and there is no
//! parameter it could reach a store through.
//!
//! # The edge set
//!
//! An upstream is *"an artifact of a kind listed in the registered producer's `requires` for the
//! root's kind, in the same stage (ADR-0025)"* — that list is the edge set, and *"a same-stage
//! artifact outside it is not an upstream"* (PIA-044). [`declared_inputs`] is what reads it off the
//! catalog.

use super::artifacts::{ArtifactRegistry, Instance};
use super::canonical::Digest;
use super::item::ArtifactState;
use crate::catalog::Catalog;

/// Step 3's three-way classification. **Closed**: a fourth state is a decision with a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Classification {
    Present,
    Stale,
    Missing,
}

impl Classification {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// A vouched root's upstream moved. **A signal, not an event.**
///
/// The four fields are AC-525's `drift_advisory { root, upstream, old_digest, new_digest }`, carried
/// so the boundary can construct the event without recomputing anything — but this type is not that
/// event and nothing here appends one. `old_digest` is `None` when the root recorded no
/// `derived_from` entry for the upstream at all, which is the ordinary vouched-root case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftSignal {
    pub root: String,
    pub upstream: String,
    pub old_digest: Option<Digest>,
    pub new_digest: Digest,
}

impl DriftSignal {
    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "root": self.root,
            "upstream": self.upstream,
            "old_digest": self.old_digest.as_ref().map(|d| d.as_str().to_owned()),
            "new_digest": self.new_digest.as_str(),
        })
    }
}

/// What step 3 produced for one artifact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step3Outcome {
    pub instance: Instance,
    pub classification: Classification,
    /// Present only on the vouched-root path, and only when an **in-set** upstream changed. A
    /// same-stage artifact outside the edge set emits nothing at all (AC-067's negative companion).
    pub drift: Option<DriftSignal>,
}

impl Step3Outcome {
    /// The classification of an instance the item has no recorded state for.
    #[must_use]
    pub fn missing(instance: &Instance) -> Self {
        Self {
            instance: instance.clone(),
            classification: Classification::Missing,
            drift: None,
        }
    }
}

/// **Step 3.**
///
/// `declared_inputs` is the edge set — the registered producer's `requires` for this kind, in the
/// requiring stage. `upstream` is the **current** digest of each of those kinds.
#[must_use]
pub fn step3(
    artifacts: &ArtifactRegistry,
    state: &ArtifactState,
    declared_inputs: &[&str],
    upstream: &[(&str, Digest)],
) -> Step3Outcome {
    let instance = state.instance().clone();
    let kind = &instance.kind;

    if !state.is_materialised() {
        return Step3Outcome {
            instance,
            classification: Classification::Missing,
            drift: None,
        };
    }

    // ---- rule 1: the REGISTERED SOURCE, not `derived_from = []` (§7b.9(6)) ---------------------
    if artifacts.is_human_sourced(kind) {
        // The vouched-root path. It stays `Present` and its producer is not dispatched; drift is an
        // advisory that can never promote itself to `Stale` or to a blocking gate — which is a fact
        // about this function's SHAPE, not a check: the classification is decided here and the
        // drift is computed after it, so no code path leads from one to the other.
        let drift = upstream
            .iter()
            .filter(|(name, _)| declared_inputs.contains(name))
            .find_map(|(name, current)| {
                let recorded = state.derived_from_entries().get(*name);
                if recorded == Some(current) {
                    None
                } else {
                    Some(DriftSignal {
                        root: kind.clone(),
                        upstream: (*name).to_owned(),
                        old_digest: recorded.cloned(),
                        new_digest: current.clone(),
                    })
                }
            });
        return Step3Outcome {
            instance,
            classification: Classification::Present,
            drift,
        };
    }

    // ---- rule 2: AC-066, absence included -------------------------------------------------------
    for input in declared_inputs {
        let Some(recorded) = state.derived_from_entries().get(*input) else {
            // "An artifact with no recorded `derived_from` entry for a declared input is `Stale`,
            // not assumed fresh."
            return Step3Outcome {
                instance,
                classification: Classification::Stale,
                drift: None,
            };
        };
        if let Some((_, current)) = upstream.iter().find(|(n, _)| n == input)
            && recorded != current
        {
            return Step3Outcome {
                instance,
                classification: Classification::Stale,
                drift: None,
            };
        }
    }

    Step3Outcome {
        instance,
        classification: Classification::Present,
        drift: None,
    }
}

/// The **edge set** for one kind at one stage: the registered producer's `requires`, per ADR-0025
/// and PIA-044.
///
/// A kind registered `source = "human"` has no producing activity, so its edge set is read from the
/// activity that would produce it *if one existed* — and when none does, from the `derived_from`
/// declarations of the same-stage activities that name it. Here it is the simpler of the two: the
/// `requires` of any same-stage activity that `provides` the kind. When no such activity exists the
/// edge set is empty, and an empty edge set emits nothing, which is the correct reading of
/// *"a same-stage artifact outside that edge set changing emits nothing"*.
#[must_use]
pub fn declared_inputs(
    catalog: &Catalog,
    _artifacts: &ArtifactRegistry,
    stage: &str,
    kind: &str,
) -> Vec<String> {
    let mut out: Vec<String> = catalog
        .activities()
        .iter()
        .filter(|a| a.stage == stage && a.provides.iter().any(|p| p == kind))
        .flat_map(|a| a.requires.iter().map(|r| r.kind.clone()))
        .collect();
    out.sort();
    out.dedup();
    out
}
