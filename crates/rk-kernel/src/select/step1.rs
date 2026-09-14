//! **Step 1 — rigor.** Design §4.4 row 1:
//!
//! > Per stage from `policy.<class>` or an explicit item-frontmatter override (logged).
//! > `forbidden` is a **logged skip**, not an absence. **`discouraged` is reserved**.
//! >
//! > *Invariant:* Rigor is ordinal and admits comparison only; `forbidden` means nothing in that
//! > stage runs, **including via another activity's `requires`**.
//!
//! # Three things this step does NOT do
//!
//! * **It does not arithmetic.** [`crate::rigor::Rigor`] derives `Ord` and nothing else — no `Add`,
//!   no `Sum`, no `Into<f64>` (AC-050). Everything here is a comparison.
//! * **It does not implement `discouraged`.** §7b:768-771: *"option B, a fourth live rigor level,
//!   was rejected for having no measured need."* The variant exists so the ordinal stays total;
//!   [`crate::select::Pack::from_toml`] refuses a pack that sets it, naming the site.
//! * **It does not default.** A stage the pack does not mention has **no** rigor, and
//!   [`Step1Outcome::rigor`] returns `None` rather than `optional`. `00-preamble.md`:55 forbids a
//!   code default for a value a record owes, and a defaulting step 1 would make every unmentioned
//!   stage silently runnable.
//!
//! # Why `forbidden` produces a ROW
//!
//! AC-077's second AND: *"every such exclusion appears as a logged skip row naming the forbidden
//! stage"*. A `forbidden` that produced nothing would be indistinguishable from a stage the pack
//! never mentioned — and §7b.9(7) names exactly that confusion as the live failure mode:
//! `forbidden` is a member of the `Rigor` ordinal while behaving as a **stage class**, which makes
//! the arithmetic route through AC-052 look right, because it finds *"no candidates"* and
//! **silently drops the requirement**.

use std::collections::BTreeMap;

use super::{Item, Pack};
use crate::rigor::Rigor;

/// Where a stage's rigor came from. Design §4.4 row 1 puts "(logged)" inside the rule, so the
/// provenance is part of the output rather than something a caller reconstructs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RigorSource {
    /// `policy.stages.<stage>` in the pack.
    PackPolicy,
    /// An explicit item-frontmatter override.
    ItemOverride,
}

impl RigorSource {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackPolicy => "pack_policy",
            Self::ItemOverride => "item_override",
        }
    }
}

/// One logged override row: what the pack said, what the item said instead.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OverrideRow {
    pub stage: String,
    /// The pack's value. `Rigor::Optional` is **not** used as a stand-in for "the pack said
    /// nothing": an override of an unmentioned stage records the same value on both sides and is
    /// reported as such, because inventing a `from` would make the log say the pack had a position
    /// it did not have.
    pub from: Rigor,
    pub to: Rigor,
}

/// Why a stage or a kind was skipped. The rows step 5 carries as `dag_assembled.skipped`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkipReason {
    /// The pack sets this stage to `forbidden` for the item's class.
    ForbiddenStage,
    /// A `requires` edge reached a kind whose only producers live in a forbidden stage. ADR-0057's
    /// Decision: *"`forbidden` means … nothing in that stage runs, even via `requires`"*.
    ForbiddenStageViaRequires,
}

impl SkipReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ForbiddenStage => "forbidden_stage",
            Self::ForbiddenStageViaRequires => "forbidden_stage_via_requires",
        }
    }
}

/// One skip row.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SkipRow {
    /// **Always named.** AC-077: *"a logged skip row naming the forbidden stage"*.
    pub stage: String,
    /// The artifact kind that could not be produced, for a `requires`-reached skip.
    pub kind: Option<String>,
    /// The activity whose `requires` reached it.
    pub requiring_activity: Option<String>,
    pub reason: SkipReason,
}

impl SkipRow {
    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "stage": self.stage,
            "kind": self.kind,
            "requiring_activity": self.requiring_activity,
            "reason": self.reason.as_str(),
        })
    }
}

/// What step 1 produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step1Outcome {
    per_stage: BTreeMap<String, (Rigor, RigorSource)>,
    overrides: Vec<OverrideRow>,
    skipped: Vec<SkipRow>,
}

impl Step1Outcome {
    /// The rigor in force at one stage. `None` for a stage neither the pack nor the item mentions.
    #[must_use]
    pub fn rigor(&self, stage: &str) -> Option<Rigor> {
        self.per_stage.get(stage).map(|(r, _)| *r)
    }

    /// Where that rigor came from.
    #[must_use]
    pub fn source(&self, stage: &str) -> Option<RigorSource> {
        self.per_stage.get(stage).map(|(_, s)| *s)
    }

    /// `true` when the stage is `forbidden`. **Nothing in that stage runs, including via another
    /// activity's `requires`.**
    #[must_use]
    pub fn is_forbidden(&self, stage: &str) -> bool {
        self.rigor(stage) == Some(Rigor::Forbidden)
    }

    /// The logged override rows, one per stage the item's frontmatter moved.
    #[must_use]
    pub fn overrides(&self) -> &[OverrideRow] {
        &self.overrides
    }

    /// The logged skip rows, one per forbidden stage, in byte order of stage name.
    #[must_use]
    pub fn skipped(&self) -> &[SkipRow] {
        &self.skipped
    }

    /// Every stage with a rigor, in byte order.
    pub fn stages(&self) -> impl Iterator<Item = (&String, Rigor)> {
        self.per_stage.iter().map(|(k, (r, _))| (k, *r))
    }
}

/// **Step 1.** Resolve the rigor of every stage the pack or the item mentions, log every override,
/// and emit one skip row per forbidden stage.
#[must_use]
pub fn step1(pack: &Pack, item: &Item) -> Step1Outcome {
    let mut per_stage: BTreeMap<String, (Rigor, RigorSource)> = BTreeMap::new();
    for (stage, rigor) in pack.policy() {
        per_stage.insert(stage.clone(), (*rigor, RigorSource::PackPolicy));
    }

    let mut overrides = Vec::new();
    for (stage, to) in item.rigor_overrides() {
        let from = pack.rigor(stage).unwrap_or(*to);
        if from != *to {
            overrides.push(OverrideRow {
                stage: stage.clone(),
                from,
                to: *to,
            });
        }
        per_stage.insert(stage.clone(), (*to, RigorSource::ItemOverride));
    }
    overrides.sort();

    // One row per forbidden stage. BTreeMap iteration is byte order, so the rows are ordered
    // without a sort and cannot depend on the pack's authoring order.
    let skipped = per_stage
        .iter()
        .filter(|(_, (r, _))| *r == Rigor::Forbidden)
        .map(|(stage, _)| SkipRow {
            stage: stage.clone(),
            kind: None,
            requiring_activity: None,
            reason: SkipReason::ForbiddenStage,
        })
        .collect();

    Step1Outcome {
        per_stage,
        overrides,
        skipped,
    }
}

/// The admissibility comparison, and the only place rigor is compared.
///
/// `min_rigor ≤ r ≤ max_rigor`, where an **absent ceiling is no upper bound** (ADR-0056; PIA-054).
/// `forbidden` and `discouraged` admit nothing: the first because nothing in that stage runs
/// (ADR-0057), the second because it is reserved and its semantics are Phase 2's (AC-050).
///
/// This is a **comparison**, not arithmetic. AC-050's compile-fail half is what stops it becoming
/// arithmetic, and it is a `trybuild` case against [`crate::rigor::Rigor`] rather than a rule here.
#[must_use]
pub fn admits(stage_rigor: Rigor, min_rigor: Rigor, max_rigor: Option<Rigor>) -> bool {
    if matches!(stage_rigor, Rigor::Forbidden | Rigor::Discouraged) {
        return false;
    }
    if stage_rigor < min_rigor {
        return false;
    }
    max_rigor.is_none_or(|ceiling| stage_rigor <= ceiling)
}
