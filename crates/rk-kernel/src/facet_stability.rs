//! `--facet-stability` — **AC-071, the unblocking gate**, as a library rule.
//!
//! # Five clauses, and why the count may not move
//!
//! The register's executable precondition names five, and the lint exits 0 for a facet declared
//! `source = "llm"` **only** when a scored P1 stability record satisfies every one of them:
//!
//! | clause | the register's own line |
//! | --- | --- |
//! | identity | `rec.modal_required_set_identity >= 0.80` — *"THE CLAUSE THAT STILL HOLDS THE BLOCK"*; F36 measured 0.25 / 0.15 / 0.10 / 0.10 |
//! | n | `rec.n_items >= 20` |
//! | draws | `rec.draws_per_item >= 10` |
//! | panel | `rec.panel_verdict == "scored"` — discharged **for arm D** by F39, and still a clause |
//! | corpus | `rec.corpus_sha == <catalog corpus pin>` |
//!
//! **A discharged clause is not a partial unblocking, and the five-case fixture set stays at
//! five** — the register says exactly that, and its residual-risk sentence names the failure this
//! rule is shaped against: *"if that lint is implemented loosely — reading
//! `modal_required_set_identity` and skipping `panel_verdict` or `corpus_sha` — the entire
//! provisional apparatus unblocks silently."* So this rule **collects every violation** rather than
//! returning on the first: a fixture that fails one clause must produce that clause and no other,
//! which is what makes the six fixtures discriminating instead of six ways of saying "load failed".
//!
//! # This is a second rule, not a sixth clause on anything
//!
//! Its subject is `source = "llm"` and **only** that. [`crate::facet_provenance`] gates
//! `source = "derived"` against a different record with a different failure meaning, and AC-520's
//! own *"why it is a criterion and not a sixth clause on AC-071"* paragraph says why the two may
//! not be one gate: *"a single gate carrying two different subjects, two different records and two
//! different failure meanings is the shape that invites that collapse."*
//!
//! # What is ARMED and not discharged here
//!
//! Two halves, both already in `s7b-kernel.md` §7b.15's ARMED table and neither quietly closed by
//! this file:
//!
//! - **The record half.** Nothing produces AC-071's record; its declared path
//!   `spikes/results/p1/stability.json` names a directory that has never existed on either side;
//!   and §7b.2 fixes that when it is authored it is **transcribed from F36's frozen output and
//!   never regenerated**, because re-running today's instrument over a registry Step 4a rewrote
//!   returns `identity = 1.00` and would unblock the gate. A register amendment naming the path,
//!   the producer and that non-reproducibility is **OWED**. This rule therefore takes the records
//!   as an argument and opens no path.
//! - **The CLI half.** `redkiln lint --facet-stability` is `rk-cli`'s surface and `rk-cli` is
//!   **Step 7d**; its argument `.redkiln/facets.toml` is **Step 7e**'s file. Neither exists, so the
//!   CLI half is **ARMED, NOT DISCHARGED** and is deferred to 7d (the binary) and 7e (the input).
//!   What is discharged here is the fail-closed **behaviour**, over in-crate fixtures — which is
//!   what §7b.14 says Step 7b owes in place of the two checkpoint lines it deleted.

use std::collections::BTreeMap;
use std::fmt;

use crate::registry::{Registry, Source, StabilityRecord};

/// The identity bar, pre-registered by P1 and measured against at 0.25 / 0.15 / 0.10 / 0.10 [F36].
pub const IDENTITY_BAR: f64 = 0.80;
/// The item-count bar.
pub const N_ITEMS_BAR: i64 = 20;
/// The draws-per-item bar.
pub const DRAWS_PER_ITEM_BAR: i64 = 10;
/// The only `panel_verdict` that satisfies the fourth clause.
pub const PANEL_VERDICT_SCORED: &str = "scored";

/// One clause of AC-071 failing for one facet.
#[derive(Debug, Clone, PartialEq)]
pub enum StabilityViolation {
    /// A facet declared `source = "llm"` with no stability record at all.
    NoRecord { facet: String },
    /// `modal_required_set_identity` absent or below [`IDENTITY_BAR`].
    IdentityBelowBar { facet: String, found: Option<f64> },
    /// `n_items` absent or below [`N_ITEMS_BAR`].
    NBelowTwenty { facet: String, found: Option<i64> },
    /// `draws_per_item` absent or below [`DRAWS_PER_ITEM_BAR`].
    DrawsBelowTen { facet: String, found: Option<i64> },
    /// `panel_verdict` absent or not `"scored"`.
    PanelNotScored {
        facet: String,
        found: Option<String>,
    },
    /// `corpus_sha` absent, or not equal to the catalog corpus pin.
    CorpusNotPinned {
        facet: String,
        found: Option<String>,
        expected: String,
    },
}

impl StabilityViolation {
    /// The facet the clause failed for.
    #[must_use]
    pub fn facet(&self) -> &str {
        match self {
            Self::NoRecord { facet }
            | Self::IdentityBelowBar { facet, .. }
            | Self::NBelowTwenty { facet, .. }
            | Self::DrawsBelowTen { facet, .. }
            | Self::PanelNotScored { facet, .. }
            | Self::CorpusNotPinned { facet, .. } => facet,
        }
    }

    /// A stable tag for the clause, so a test can name the clause it expects without matching on
    /// the message text.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::NoRecord { .. } => "no-record",
            Self::IdentityBelowBar { .. } => "identity",
            Self::NBelowTwenty { .. } => "n",
            Self::DrawsBelowTen { .. } => "draws",
            Self::PanelNotScored { .. } => "panel_verdict",
            Self::CorpusNotPinned { .. } => "corpus_sha",
        }
    }
}

impl fmt::Display for StabilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRecord { facet } => write!(
                f,
                "facet {facet:?} is source = \"llm\" with no scored P1 stability record (AC-071)"
            ),
            Self::IdentityBelowBar { facet, found } => write!(
                f,
                "facet {facet:?}: modal_required_set_identity {} is below the pre-registered \
                 {IDENTITY_BAR:.2} line (AC-071 clause 1; F36 measured 0.25/0.15/0.10/0.10)",
                render_f64(*found)
            ),
            Self::NBelowTwenty { facet, found } => write!(
                f,
                "facet {facet:?}: n_items {} is below {N_ITEMS_BAR} (AC-071 clause 2)",
                render_i64(*found)
            ),
            Self::DrawsBelowTen { facet, found } => write!(
                f,
                "facet {facet:?}: draws_per_item {} is below {DRAWS_PER_ITEM_BAR} (AC-071 clause 3)",
                render_i64(*found)
            ),
            Self::PanelNotScored { facet, found } => write!(
                f,
                "facet {facet:?}: panel_verdict {} is not {PANEL_VERDICT_SCORED:?} (AC-071 \
                 clause 4 — discharged for arm D by F39, and still a clause)",
                render_str(found.as_deref())
            ),
            Self::CorpusNotPinned {
                facet,
                found,
                expected,
            } => write!(
                f,
                "facet {facet:?}: corpus_sha {} is not the catalog corpus pin {expected:?} \
                 (AC-071 clause 5)",
                render_str(found.as_deref())
            ),
        }
    }
}

fn render_f64(v: Option<f64>) -> String {
    v.map_or_else(|| "<absent>".to_owned(), |x| format!("{x}"))
}

fn render_i64(v: Option<i64>) -> String {
    v.map_or_else(|| "<absent>".to_owned(), |x| format!("{x}"))
}

fn render_str(v: Option<&str>) -> String {
    v.map_or_else(|| "<absent>".to_owned(), |x| format!("{x:?}"))
}

/// AC-071's rule: fail closed on `source = "llm"` unless a scored stability record satisfies **all
/// five** clauses.
///
/// `corpus_pin` is the catalog corpus pin the fifth clause compares against. It is passed in and
/// never read from disk here: the record is corpus-relative through `CORPUS_PIN` (AC-464), the pin
/// has not moved, and a rule that fell back to a working-tree copy would discharge on a route
/// AC-464 forbids.
///
/// # Errors
///
/// Every clause that failed, for every `llm` facet, collected — never the first one only. A lint
/// that returned early would report one clause per registry and the five one-clause fixtures would
/// stop discriminating between the clauses they are named for.
pub fn facet_stability_rule(
    registry: &Registry,
    records: &BTreeMap<String, StabilityRecord>,
    corpus_pin: &str,
) -> Result<(), Vec<StabilityViolation>> {
    let mut out = Vec::new();
    for facet in registry.facets() {
        // THE GIVEN IS `source = "llm"` AND NOTHING ELSE. AC-071's own dated note refuses the
        // widening: the `derived` sibling gap is AC-520, "authored as its own criterion rather than
        // bolted on here".
        if facet.source != Source::Llm {
            continue;
        }
        let name = facet.name.clone();
        let Some(record) = records.get(&name) else {
            out.push(StabilityViolation::NoRecord { facet: name });
            continue;
        };
        // ALL FIVE CLAUSES, EVERY TIME, AND NO EARLY RETURN. An absent field and a failing field
        // are the same verdict — a clause the record does not satisfy — and both must be reported
        // as that clause rather than skipped for want of a value.
        if record
            .modal_required_set_identity
            .is_none_or(|v| v < IDENTITY_BAR)
        {
            out.push(StabilityViolation::IdentityBelowBar {
                facet: name.clone(),
                found: record.modal_required_set_identity,
            });
        }
        if record.n_items.is_none_or(|v| v < N_ITEMS_BAR) {
            out.push(StabilityViolation::NBelowTwenty {
                facet: name.clone(),
                found: record.n_items,
            });
        }
        if record.draws_per_item.is_none_or(|v| v < DRAWS_PER_ITEM_BAR) {
            out.push(StabilityViolation::DrawsBelowTen {
                facet: name.clone(),
                found: record.draws_per_item,
            });
        }
        if record.panel_verdict.as_deref() != Some(PANEL_VERDICT_SCORED) {
            out.push(StabilityViolation::PanelNotScored {
                facet: name.clone(),
                found: record.panel_verdict.clone(),
            });
        }
        if record.corpus_sha.as_deref() != Some(corpus_pin) {
            out.push(StabilityViolation::CorpusNotPinned {
                facet: name,
                found: record.corpus_sha.clone(),
                expected: corpus_pin.to_owned(),
            });
        }
    }
    if out.is_empty() { Ok(()) } else { Err(out) }
}
