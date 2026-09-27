//! The activity catalog, and the **closed** evidence set each step kind mints.
//!
//! This is the half of the catalog the rigor-monotonicity rule needs and no more: the `[activity]`
//! header the selection filter reads, and the `[[step]]` list ADR-0036 turns into an evidence
//! promise. Nothing here dispatches, and nothing here is the selection function — that is Slice 3's
//! seven-step pure function and it is **not built**.
//!
//! # Minting is ADR-0036's, and it is closed
//!
//! *"a `tool` step mints `ToolReport` (plus `GreenAt` when `run = toolchain:test` and status is
//! passing; plus `Mutation` when the output is a `MutationReport`); an `agent` step mints
//! `Findings` or `SelfCheck` depending on its output type; a `gate` step mints `HumanAnswer`;
//! `check` and `emit` steps mint no evidence."* That is the whole rule, and it is what makes
//! ADR-0081 clause 1 *"computable from the catalog with nothing run"*.
//!
//! **One half of it is not catalog-computable, and the reading is stated rather than taken
//! silently.** `GreenAt`'s condition includes *"and status is passing"*, which is a run-time fact.
//! [`Step::mints`] mints `GreenAt` for `run = "toolchain:test"` on the catalog half alone, because
//! ADR-0081's own premise is that `evidence(a)` is computable without running anything, and because
//! the alternative — dropping `GreenAt` entirely — would drop it from **both** sides of every
//! superset comparison and change no verdict while making the promise wrong. The activity-level
//! `evidence = […]` promises in the shipped catalog list `GreenAt` unconditionally beside a
//! `toolchain:test` step, which is the same reading.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use crate::toml_subset::{self, Table, Value};

/// The five step kinds of the activity-graph schema §1.
///
/// 🛑 **The serde derives serve the RUN RECORD, not a second catalog loader.** `s7g` (ADR-0138
/// clause 5) puts an activation's transcribed step list in `inputs/specs.json` so a run can be read
/// against a catalog that has since moved. The **only** path from a `.toml` activity to a [`Step`]
/// remains this module's hand-written loader over `toml_subset`; `Deserialize` here reads back what
/// this crate itself wrote, and adding a `serde`-driven TOML path would be a second parse of the
/// catalog with its own acceptance surface.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum StepKind {
    Agent,
    Tool,
    Check,
    Gate,
    Emit,
}

impl StepKind {
    fn from_token(s: &str) -> Option<Self> {
        match s {
            "agent" => Some(Self::Agent),
            "tool" => Some(Self::Tool),
            "check" => Some(Self::Check),
            "gate" => Some(Self::Gate),
            "emit" => Some(Self::Emit),
            _ => None,
        }
    }
}

/// The closed `Evidence` enum of the activity-graph schema §2.
///
/// `Degraded` and `Attempts` are members of the closed set and are minted by the **runtime**, not
/// by a step kind — ADR-0036's mapping names neither — so nothing in this module produces them.
/// They are listed because the set is closed and a reader must be able to see that they were not
/// forgotten.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Evidence {
    SelfCheck,
    Findings,
    ToolReport,
    GreenAt,
    Mutation,
    HumanAnswer,
    Degraded,
    Attempts,
}

impl fmt::Display for Evidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::SelfCheck => "SelfCheck",
            Self::Findings => "Findings",
            Self::ToolReport => "ToolReport",
            Self::GreenAt => "GreenAt",
            Self::Mutation => "Mutation",
            Self::HumanAnswer => "HumanAnswer",
            Self::Degraded => "Degraded",
            Self::Attempts => "Attempts",
        };
        f.write_str(name)
    }
}

/// One `[[step]]`.
///
/// The serde derives are the run record's — see [`StepKind`]'s note for why they are not a second
/// catalog loader.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Step {
    pub id: String,
    pub kind: StepKind,
    /// `run = "toolchain:…"` for a `tool` step.
    pub run: Option<String>,
    /// `outputs = "value:…"`, with the `value:` prefix stripped.
    pub output_type: Option<String>,
    /// `role = "<archetype>"` — **the input to the breaker's partition key**, and ADR-0014's
    /// *"an activity nominates a **role** and a typed contract, never a card."*
    ///
    /// Read as of **2026-09-14** (operator decision D-7). The pack declares a model family per role
    /// — `[role.<name>] family`, extending design :480's *"tier policy per role"* — the kernel
    /// resolves role → family at plan time through
    /// [`crate::select::Pack::require_family_for_role`], and [`crate::workflow::breaker`] keys on
    /// the **resolved family**. Until today this loader parsed `role` **zero** times [measured
    /// 2026-09-14: `grep -c role crates/rk-kernel/src/catalog.rs` → 0] while the shipped catalog
    /// declared one **185 times in 8 distinct values** — `architect`, `implementer`,
    /// `investigator`, `operator`, `planner`, `releaser`, `reviewer`, `spec-author` [measured
    /// 2026-09-14 over `spikes/p9/catalog/*.toml`]. The carrier was the file all along and the
    /// loader dropped it, which is `version`'s correction above, a second time.
    ///
    /// 🛑 **THIS IS NOT [`Activity::family`], AND THE DISTANCE BETWEEN THEM IS ONE FIELD ACCESS.**
    /// [`Activity::family`] is ADR-0058's **activity** family — a rank-escalation ladder whose
    /// members (`root-cause(10) < differential-debugging(20) < bisect(30) <
    /// instrumented-trace-analysis(40)`) would typically all run on the **same** model. A breaker
    /// keyed on it would trip globally with respect to the model and locally with respect to the
    /// method, which is the exact inverse of AC-034 on both axes.
    /// `crates/rk-kernel/src/workflow/breaker.rs`'s header carries the whole of that reading, and
    /// [`crate::workflow::breaker::family_key_never_from_the_activity_catalog_rule`] is the lint
    /// that keeps the substitution out of the file where it would be invisible.
    ///
    /// **`Option`, and `None` is not a role.** A `check`, `tool` or `emit` step nominates none —
    /// the schema puts `role` on `agent` and `gate` steps — and a step that declares none has
    /// declared none. Nothing here invents one, on [`Activity::version`]'s doctrine.
    pub role: Option<String>,
    /// The step's own `when` guard, verbatim. `None` means the step always runs.
    pub when: Option<String>,
    /// `expr = "…"` on a `check` step, verbatim — the schema's **policy predicate**, and the thing
    /// ADR-0080 calls a quality block's *terminal condition*.
    ///
    /// Read for one reason, at Slice 2's AC-527 debt: [`crate::quality_block`] evaluates it over an
    /// empty findings set. It is **not** evaluated here and it is **not** parsed here — a predicate
    /// that does not parse is a violation the rule reports with the activity and the step that
    /// carry it, and a loader that refused it would move AC-527's THEN (*"lint fails **naming the
    /// activity and the predicate**"*) into a `CatalogError` that names neither.
    ///
    /// *(dated 2026-09-25, s8b refuting round 2; D21-55)* No longer "over an empty findings set":
    /// under the amended AC-527 the rule matches ADR-0080 d2's named form on the parse tree, and
    /// evaluates each conjunct required for truth that reads `findings` over a one-finding witness
    /// carrying only `evidence`, one blocking class at a time. It still does not parse or evaluate
    /// the predicate here.
    ///
    /// Carried on **every** step kind and not only on `check`, because the field is transcribed and
    /// not interpreted: the schema puts `expr` on `check` alone, and a step of another kind that
    /// declares one is a schema defect this loader must not hide by dropping the key. **Ninety-four
    /// shipped steps declare one, all of them `check`** [measured 2026-09-14:
    /// `grep -rhc '^expr' spikes/p9/catalog/*.toml` sums to 94].
    pub expr: Option<String>,
    /// `for_each = "${step.outputs.<list>}"` or `"artifact:Kind.<list>"` (schema §9b.5), verbatim.
    ///
    /// **The cardinality is not here and cannot be**: schema §1 is *"cardinality is runtime-computed
    /// but the *shape* is declared"*, which is ADR-0053's whole driver. The runtime charge for a
    /// step carrying this is [`crate::budget::charge_for_each`], and AC-022's THEN is that it does
    /// not read the cardinality either.
    ///
    /// Seven shipped steps declare one [measured 2026-09-14:
    /// `grep -cE '^for_each' spikes/p9/catalog/*.toml` sums to 7 across 7 files].
    pub for_each: Option<String>,
    /// `retry = { max = N }` (schema §4), selecting the kernel's per-error-class policy.
    ///
    /// Read for one reason at Slice 4a: it is the **losing** formula ADR-0037 replaced — `Σ (step
    /// retry.max × 1)` — and `attempts_total_counts_a_pass_not_retries` computes it in order to
    /// assert that lint rule 8 does **not**. A step *"never defines its own backoff"* (schema §4), so
    /// nothing here is a retry policy.
    pub retry_max: Option<u32>,
    /// `repeat = { until = "...", max = N }` (schema 4) -- the `max` only.
    ///
    /// Read for one reason at Slice 4g: AC-019's identity is *"executed steps plus `memo_hit`
    /// events equals the number of **step activations** ... not the graph's static step count,
    /// which `repeat`/`on_false` (AC-017) and `for_each` (AC-022) make unequal to it"*, and
    /// `ac019_executed_plus_hits_equals_activations` needs the two to be unequal **as a parsed
    /// premise** rather than as a number a reader has to trust. The `until` predicate is **not**
    /// read here: evaluating it is the runtime's (Slice 4h), ADR-0103's evaluator already exists
    /// for it, and a second reading would be a second home for the loop condition.
    ///
    /// Fourteen shipped steps across fourteen files declare one [measured 2026-09-14:
    /// `grep -rn '^repeat' spikes/p9/catalog/ | wc -l` -> 14, and `-rln` -> 14].
    pub repeat_max: Option<u32>,
    /// `memo = <bool>` on one `[[step]]` -- **the field ADR-0097 decision 3 withdrew**.
    ///
    /// It is parsed **so that its presence can be refused**, which is the opposite of the usual
    /// reason to parse a key. AC-018's THEN is *"a step-level `memo = false` declaration is itself
    /// a pack validation error (there is no per-step memo field)"*, and a rule cannot refuse a key
    /// the loader silently drops. [`crate::memo::live_verb_memo_rule`] is that rule: `Some(false)`
    /// and `Some(true)` are both violations and `None` is the only lawful shape.
    ///
    /// **Nineteen shipped steps across fourteen activities declare one, every one of them
    /// `memo = false`** [measured 2026-09-14: `grep -rn 'memo' spikes/p9/catalog/ | wc -l` -> 19;
    /// `grep -rln 'memo' spikes/p9/catalog/ | wc -l` -> 14]. So the rule is **red against the
    /// shipped catalog by design** and the repairs are Step 7e's: 7b.12:970-977 forbids editing
    /// `spikes/p9/` to make a green.
    ///
    /// *(dated 2026-09-25, s8b, D21-43 N26)* The repair was `s8b`'s, not Step 7e's: the nineteen
    /// `memo = false` lines are deleted from `.redkiln/catalog`, which now declares none, and
    /// `memo::ac018_live_verb_may_not_memoise` asserts zero there. The nineteen live on only in the
    /// frozen p9 control, `memo::ac018_frozen_p9_catalog_is_red`, read at `P9_PIN`.
    pub memo_declared: Option<bool>,
    /// **ADR-0145.** `question = "<id>"` on a `gate` step — **the gate's KIND**, verbatim.
    ///
    /// Dropped at load until `s7g`'s fourth repair pass, which is why the 80 `gate` steps the
    /// shipped pack declares had nothing to open a gate WITH: `GateRecord::derive_reach` digests a
    /// `kind`, `QuestionRegistry::question_for` looks one up, and a step that reached neither
    /// carried no identity at all. Measured over `.redkiln/catalog/`: **all 80** gate steps declare
    /// a `question`, over **45** distinct values.
    ///
    /// `None` on every other kind. The value is **not** interpreted here — the question vocabulary
    /// is the pack's, exactly as `role`'s is (ADR-0014), so an id this crate has never heard of is
    /// carried and refused at resolution rather than at load.
    pub question: Option<String>,
    /// **Record B = ADR-0171 decision 1, proposed (s8c §8c.6).** `timeout_secs`, a literal or a
    /// `${toolchain.<key>}` substitution. The loader read none until 2026-09-26 (ADR-0140's
    /// evidence: *"The loader reads no `timeout_secs`"*), so the pack's 298 declarations reached no
    /// executor. A `tool` step's is now its bound, resolved by the kernel after the registry lookup
    /// and the route and input checks (ADR-0170 decision 3's order); an `agent` step's is carried
    /// and enforced by `s8d` (D21-43 N16). `inputs` is still not parsed (ADR-0148's forbids bullet
    /// stands until `s8d`).
    ///
    /// Skipped from the run record when `None`, so a step that declares none serialises as it did.
    ///
    /// *(dated 2026-09-26, `s8d` build, package P2; R2 = ADR-0173, written `proposed`)* `inputs`
    /// is now parsed, onto [`Step::inputs`]: R2 is the record that routes what may read it
    /// (ADR-0148's fourth forbids bullet), and it supersedes ADR-0148 decision 3 in part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<StepTimeout>,
    /// **R2's parsed-`inputs` carrier** (ADR-0173, written `proposed`; `s8d` §8d.3; D21-57).
    /// `inputs = { <binding> = <source>, … }`, each value transcribed and not interpreted: a
    /// source string (`artifact:…`, `kb:…`, `checklist:…`, `toolchain:…`, `${…}`, or a string
    /// literal) or a scalar literal.
    ///
    /// ADR-0148 decision 3 kept this key off the type (*"`catalog::Step` is not widened to parse
    /// `inputs`"*) because nothing could read it, and its fourth forbids bullet forbids the widening
    /// *"without a record routing what may then read it"*. R2 is that record, and it names the
    /// readers: the forward-ref licence lint ([`forward_ref_lint`], D21-48 X8), AC-303's confirm
    /// rule (`rigor::confirm_step_monotonicity_rule`, D21-57), and R2's resolver
    /// (`runtime::reference`). ADR-0148's other forbids stand: nothing here passes a declared
    /// selection through as a test name or as an empty selection.
    ///
    /// The `toml_subset` table is a `BTreeMap`, so bindings come back in key order, not declaration
    /// order; no reader here needs declaration order. Skipped from the run record when empty, as
    /// [`Step::timeout_secs`] is when `None`.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub inputs: BTreeMap<String, InputValue>,
    /// `on_false = { to = "<step>", … }`'s `to`, the loop-back target (ADR-0044; ADR-0103 clause
    /// 6's `{to, max}` form). `None` for the keyword forms and for a step with no `on_false`. Read
    /// for the forward-ref lint's first licence (ADR-0042's letter); the executor's reading of the
    /// whole form is `predicate::classify_on_false`'s.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_false_to: Option<String>,
    /// `repeat = { until = "<predicate>", … }`'s `until`, verbatim ([`Step::repeat_max`] carries
    /// its `max`). Read for the forward-ref lint's second licence, the repeat body, which runs
    /// through the last step `until` references (D21-48 X6). It is not evaluated here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repeat_until: Option<String>,
    /// `escalate = { question = "<id>" }`'s question: the gate kind this step escalates to, directly
    /// or through `on_exhausted = "escalate"`. Read for the forward-ref lint's third licence, the
    /// escalate re-entry (D21-48 X8).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escalate_question: Option<String>,
    /// *(dated 2026-09-26, `s8d` build, package P5; R3, written `proposed`.)* The step's whole
    /// `on_false`, transcribed as declared and NOT interpreted here: the table or the keyword, as a
    /// JSON value. The executor classifies it through the one classifier,
    /// `predicate::classify_on_false` (ADR-0103 clause 6's closed set), at the moment a `check`
    /// does not hold — so a form outside the closed set is refused by name there, never read as
    /// some other form. [`Step::on_false_to`] stays the lint's reading of the `to` alone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_false: Option<serde_json::Value>,
    /// *(dated 2026-09-26, `s8d` build, package P5; R2 decision 5.)* `on_exhausted = "<exit>"`, the
    /// exit a `repeat` takes at `max` (schema §1, *"`escalate | fail | continue`"*), verbatim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_exhausted: Option<String>,
    /// *(dated 2026-09-26, `s8d` build, package P4; R2 = ADR-0173 decision 7, written `proposed`.)*
    /// An `emit` step's `artifacts` table and `evidence` list, transcribed and not interpreted:
    /// `runtime::reference::bind_emit` binds them, and that binding is what commits the activation.
    /// `None` for a step that declares neither. Skipped from the run record when `None`, as
    /// [`Step::timeout_secs`] is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emit: Option<EmitBinding>,
}

/// An `emit` step's declared binding (R2 decision 7).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EmitBinding {
    /// `artifacts = { <Kind> = <source> | { <member> = <source>, … } }`, by kind.
    pub artifacts: BTreeMap<String, EmitArtifact>,
    /// `evidence = ["${step}", "${step?}", …]`, in declaration order.
    pub evidence: Vec<String>,
}

/// One provided kind's binding: a reference, or a composing table (schema §9c.34, as
/// `confirmation-test`'s `ConfirmationResult`).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EmitArtifact {
    Ref(String),
    Compose(BTreeMap<String, InputValue>),
}

/// One `inputs` value, transcribed (R2's carrier; see [`Step::inputs`]).
///
/// The shipped catalog declares strings and integers only [measured 2026-09-26 over every
/// `[[step]]` in the repository's TOML: 1 122 strings and 13 integers]. A boolean is carried
/// because it is a scalar literal of the same kind; an array, a table or a float is refused at load
/// by name, because a dropped value reads as a binding that was never declared.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InputValue {
    /// A source form or a string literal, verbatim.
    Text(String),
    /// An integer literal.
    Int(i64),
    /// A boolean literal.
    Bool(bool),
}

impl InputValue {
    /// The text of a [`InputValue::Text`], `None` for a scalar literal.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s),
            Self::Int(_) | Self::Bool(_) => None,
        }
    }
}

/// A step's declared `timeout_secs` (record B decision 1).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StepTimeout {
    /// `timeout_secs = <n>`. A zero is carried and refused at resolution (RT-5), where the bound is
    /// made, rather than dropped here.
    Literal(u64),
    /// `timeout_secs = "${toolchain.<key>}"`, the key resolved through `ToolchainValues`.
    Toolchain(String),
}

impl StepTimeout {
    /// Read one `timeout_secs` value: a non-negative integer, or a string of the EXACT form
    /// `${toolchain.<key>}` with a key of `[a-z0-9_]+`. Anything else is `None`, which the loader
    /// refuses naming the step.
    #[must_use]
    pub fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Int(n) => u64::try_from(*n).ok().map(Self::Literal),
            Value::Str(s) => s
                .strip_prefix("${toolchain.")
                .and_then(|rest| rest.strip_suffix('}'))
                .filter(|key| {
                    !key.is_empty()
                        && key
                            .bytes()
                            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
                })
                .map(|key| Self::Toolchain(key.to_owned())),
            _ => None,
        }
    }
}

impl Step {
    /// The evidence variants this step mints, per ADR-0036.
    #[must_use]
    pub fn mints(&self) -> BTreeSet<Evidence> {
        let mut out = BTreeSet::new();
        match self.kind {
            StepKind::Tool => {
                out.insert(Evidence::ToolReport);
                // 🛑 **ADR-0148 MADE THIS PROMISE UNKEEPABLE AT RUN TIME, AND NOTHING HERE CHANGES.**
                // `GreenAt`'s condition includes *"and status is passing"* — a run-time fact — and
                // this function mints it on the CATALOG half alone, because ADR-0081's premise is
                // that `evidence(a)` is computable without running anything. Since 2026-09-22 a
                // `toolchain:test` step refuses before reaching the port, so **no** such step can
                // now reach a passing status: the run-time conjunct is structurally unsatisfiable
                // until a selection resolver exists. The static projection is deliberately left
                // alone — changing it would move rigor monotonicity and ADR-0081's comparison for a
                // reason that is not about evidence — but a reader comparing this to a real run
                // would otherwise find the gap themselves, so it is named here. No test asserts the
                // run-time half, and that is the point.
                if self.run.as_deref() == Some("toolchain:test") {
                    out.insert(Evidence::GreenAt);
                }
                if self.output_type.as_deref() == Some("MutationReport") {
                    out.insert(Evidence::Mutation);
                }
            }
            StepKind::Agent => {
                if self.output_type.as_deref() == Some("Findings") {
                    out.insert(Evidence::Findings);
                } else {
                    out.insert(Evidence::SelfCheck);
                }
            }
            StepKind::Gate => {
                out.insert(Evidence::HumanAnswer);
            }
            StepKind::Check | StepKind::Emit => {}
        }
        out
    }
}

/// One entry of an activity's `requires` list.
///
/// The shipped catalog spells an **optional** requirement with a `?` suffix —
/// `requires = ["Spec", "DesignDoc?"]` at `spikes/p9/catalog/api-contract-spec.toml`:6 — and the
/// distinction is load-bearing rather than cosmetic. **AC-051's load-time failure covers only the
/// UNCONDITIONAL case**, so a conditional requirement reaching a `forbidden` stage LOADS, and the
/// runtime case §7b.9(7) names is live: it owes both a skip row and an `Unsatisfiable` gate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    pub kind: String,
    /// `true` for the `Kind?` spelling.
    pub optional: bool,
}

impl Requirement {
    fn parse(raw: &str) -> Self {
        raw.strip_suffix('?').map_or_else(
            || Self {
                kind: raw.to_owned(),
                optional: false,
            },
            |kind| Self {
                kind: kind.to_owned(),
                optional: true,
            },
        )
    }
}

/// One `[activity]` plus its steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    /// The file this activity was read from — carried so a violation names a path a reader can
    /// open, rather than an id they then have to find.
    pub source: String,
    pub id: String,
    /// ADR-0072's **`@version`** -- component 1 of the step memo key, and the second part of the
    /// idempotency key.
    ///
    /// 🛑 **A CORRECTION TO SLICE 4e's REPORT, and it is the one that matters most here.** Slice 4e
    /// reported *"`@version` has no carrier in this tree"* on the measurement
    /// `grep -n "version" crates/rk-kernel/src/catalog.rs` -> 0 hits. That measured **this
    /// loader**, not the tree: **all 102 shipped activities declare `version`**
    /// [measured 2026-09-14: `grep -l '^version' spikes/p9/catalog/*.toml | wc -l` -> 102 of 102,
    /// values 1-4], and so does every catalog fixture under `crates/rk-kernel/tests/fixtures/`. The
    /// carrier was the file all along and the loader dropped it. Slice 4g reads it, because AC-014
    /// component 1 is `activity_id@version` and a memo key assembled from the id alone would stand
    /// still across the version bump ADR-0097 component 1 exists to move it on.
    ///
    /// **`Option`, and `None` is not `1`.** An activity that declares no version has declared
    /// none; [`Activity::id_at_version`] returns `None` rather than inventing a number, which is
    /// what leaves AC-010's *"an `activity_id` without `@version`"* refusal (Slice 4h's)
    /// something real to refuse.
    pub version: Option<u32>,
    pub stage: String,
    pub provides: Vec<String>,
    pub tags: Vec<String>,
    /// **Search terms, and nothing else** (AC-436; Step 7d Slice 3, the third of §7d.1's eight
    /// kernel edits).
    ///
    /// AC-436's whole content is that a catalog is findable without becoming selectable by the same
    /// words. `tags` above participates in admissibility; **this field must not**, and the rule is
    /// enforced rather than intended: `ac436_keywords_unreachable_from_admissibility_evaluator`
    /// scans `select/` and `predicate/` for the identifier and Checkpoint 7d runs the same ban from
    /// the shell. A search term that steered selection would make `catalog search` a way to change
    /// what runs, which is the defect the separation exists to prevent.
    ///
    /// It also cannot move a digest, and that is structural rather than careful:
    /// `select::canonical::catalog_json` hand-enumerates eleven fields and this is not one of them,
    /// so no `inputs_digest` and no `run_id` on the tree moves because this field exists.
    pub keywords: Vec<String>,
    pub applicable_if: Option<String>,
    pub method_rank: i64,
    pub min_rigor: String,
    pub max_rigor: Option<String>,
    /// ADR-0058's **ACTIVITY** family — a rank-escalation ladder, and **not a model family.**
    ///
    /// > *"An activity declares its family membership with a `family = "<name>"` field. Lint checks
    /// > three things for every declared family: that its members share `provides`, that they hold
    /// > distinct `method_rank` values, and that the family satisfies rule 26's
    /// > totality-over-rigor property."*
    ///
    /// 🛑 **This field is never the circuit breaker's key, and as of 2026-09-14 that is a live
    /// hazard rather than a theoretical one.** Operator decision D-7 made the kernel resolve a
    /// **model** family at plan time from [`Step::role`], which puts an `Activity` carrying *this*
    /// family in scope at the moment the key is built. A breaker keyed on it would trip globally
    /// with respect to the model and locally with respect to the method — the exact inverse of
    /// AC-034 on both axes — because a family's members (`root-cause(10) <
    /// differential-debugging(20) < bisect(30) < instrumented-trace-analysis(40)`) would typically
    /// all run on the **same** model. Two instruments stop it:
    /// [`crate::workflow::breaker::family_key_never_from_the_activity_catalog_rule`] structurally,
    /// and `breaker_key_is_the_packs_role_family_never_the_activity_family` behaviourally, over a
    /// fixture activity that declares this field **and** two different roles.
    pub family: Option<String>,
    /// The kinds this activity needs before it can run. Step 4 recurses on this (design §4.4
    /// row 4), and AC-051/AC-053 lint it at load.
    pub requires: Vec<Requirement>,
    /// The kinds the activity's output is derived FROM, as the catalog declares them. Step 3's
    /// "declared inputs" are read from the **producer's** row, and a missing `derived_from` entry
    /// for one of them is `Stale` and never "assumed fresh" (AC-066).
    pub derived_from: Vec<Requirement>,
    /// `S` / `M` / `L`. Step 5 sums it against the item's appetite (AC-058) and AC-068 resolves
    /// `policy.<cost_class>.stale_policy` through it. The first is built
    /// ([`crate::select::step5`]); the second is **not** — AC-068 is PROVISIONAL under ADR-0117
    /// clause 4 and its `stale_policy` value is a pack's to declare.
    pub cost_class: Option<String>,
    /// Activities this one may not appear beside in one DAG. **`excludes` are symmetric** (proposal
    /// §2.6), and [`crate::select::excludes_lint`] is AC-060's instrument.
    ///
    /// **There is no live instance**: `grep -rn excludes spikes/p9/catalog/` returns 0 hits
    /// [measured 2026-09-13]. The field is read because the rule that needs it is built, and the
    /// whole fixture pair for that rule is authored.
    pub excludes: Vec<String>,
    /// What an agent may **request** about this activity — the closed `skip | downgrade | add`
    /// vocabulary of ADR-0046. A `kind` outside this list is refused as a configuration error
    /// **before** predicate evaluation (AC-062).
    ///
    /// A token outside the three is a load error rather than a silently ignored entry: ADR-0046's
    /// forbids clause is *"a `deviate` declaration whose `kind` falls outside the closed
    /// `skip | downgrade | add` vocabulary"*.
    pub deviation_ok: Vec<String>,
    /// The activity's **declared** `budget = { attempts_total, tokens, wall_clock_secs }` (schema
    /// §4). All 102 shipped activities carry one [measured 2026-09-14:
    /// `grep -lE '^budget' spikes/p9/catalog/*.toml | wc -l` → 102].
    ///
    /// **`Option`, and `None` is not "unbounded"** — `Pack::appetite`'s doctrine at
    /// `select/pack.rs`:249-256, verbatim: *"a code default here would decide a value the pack owes"*.
    /// An activity that declares no budget has declared none, and nothing in this crate supplies one.
    ///
    /// This is the **declaration**, which is AC-021's quantity and ADR-0037's lint-rule-8 subject.
    /// The runtime draw-down is AC-026's and is a different number (ADR-0122 decision 3).
    pub budget: Option<ActivityBudget>,
    pub steps: Vec<Step>,
}

impl Activity {
    /// `activity_id@version` -- memo-key component 1 and idempotency-key part 2.
    ///
    /// `None` when the activity declares no `version`. **There is deliberately no fallback**: a
    /// default of `1` would make every unversioned activity share a memo key with version 1 of a
    /// versioned one, which is the collision component 1 exists to prevent, and it would silently
    /// satisfy the AC-010 refusal Slice 4h owes.
    #[must_use]
    pub fn id_at_version(&self) -> Option<String> {
        self.version.map(|v| format!("{}@{v}", self.id))
    }
}

/// An activity's declared budget table.
///
/// It **is** [`crate::budget::BudgetEnvelope`] and is not a second three-field struct: design §2.1
/// already declares exactly this shape as *"the declared envelope"*, and a parallel type would be
/// the second set of three counters AC-020 exists to prevent. The alias exists so the name design
/// §1.4 gives the catalog field resolves as well.
pub type ActivityBudget = crate::workflow::budget::BudgetEnvelope;

/// A directory of activities.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Catalog {
    activities: Vec<Activity>,
}

/// What stopped a catalog load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    /// The directory could not be read, or held no `.toml` at all. **An empty catalog is an
    /// error, not an empty success**: a rule that enumerates nothing passes, which is Halt 8's
    /// *"a filter that matches nothing exits 0"* one level up.
    Io {
        path: String,
        detail: String,
    },
    Toml {
        source: String,
        error: toml_subset::TomlError,
    },
    Shape {
        source: String,
        detail: String,
    },
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, detail } => write!(f, "{path}: {detail}"),
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
        }
    }
}

impl std::error::Error for CatalogError {}

/// Which defect a [`CatalogDiagnostic`] carries, as a VALUE rather than as a substring of a
/// message.
///
/// `Io`, `Toml` and `Shape` are the three [`CatalogError`] variants the loader has, transcribed.
/// `GateOutputs` is the one defect the diagnostic mode names apart from the rest, because
/// `rk-cli`'s `RK-D-GATE-OUTPUTS` row has to tell it from every other `Shape` and a rule that told
/// them apart by matching text inside `Shape.detail` would break the first time the message is
/// reworded. **The error it carries is still a `CatalogError::Shape`** — the variant set is three
/// and the refusal is among the `Shape`s (`s7d` §7d.8); this enum is the diagnostic's own
/// vocabulary, not a fourth variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticKind {
    Io,
    Toml,
    Shape,
    GateOutputs,
}

/// One typed load defect: the error the strict loader would have raised, plus the discriminant a
/// `doctor` rule maps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogDiagnostic {
    pub error: CatalogError,
    pub kind: DiagnosticKind,
}

impl fmt::Display for CatalogDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

/// What the diagnostic loader returns: **both** halves, always.
///
/// The catalog is what loaded — the activities with no defect — and the diagnostics are every
/// defect found. Neither is a `Result` arm of the other: `doctor` has rules that read the loaded
/// activities and rules that read the defects, and over a pack where both are non-empty it needs
/// both in the same pass.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CatalogLoad {
    pub catalog: Catalog,
    pub diagnostics: Vec<CatalogDiagnostic>,
}

impl Catalog {
    /// Build from named sources, so a fixture catalog can be `include_str!`'d rather than opened.
    ///
    /// # Errors
    ///
    /// [`CatalogError`] for an empty set, a file outside the TOML subset, or an activity missing a
    /// key the selection filter needs.
    pub fn from_sources(sources: &[(&str, &str)]) -> Result<Self, CatalogError> {
        if sources.is_empty() {
            return Err(empty_sources());
        }
        let mut activities = Vec::with_capacity(sources.len());
        for (name, src) in sources {
            activities.push(activity_from(name, src)?);
        }
        activities.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self { activities })
    }

    /// Read every `*.toml` in a directory.
    ///
    /// # Errors
    ///
    /// [`CatalogError`] as [`Self::from_sources`], plus an I/O failure naming the directory. A
    /// directory that exists and holds no `*.toml` is an error.
    pub fn from_dir(dir: &Path) -> Result<Self, CatalogError> {
        let io = |detail: String| CatalogError::Io {
            path: dir.display().to_string(),
            detail,
        };
        let entries = std::fs::read_dir(dir).map_err(|e| io(e.to_string()))?;
        let mut paths: Vec<_> = entries
            .filter_map(Result::ok)
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        paths.sort();
        if paths.is_empty() {
            return Err(io(
                "holds no *.toml — an empty catalog is an error, not an empty success".to_owned(),
            ));
        }
        let mut activities = Vec::with_capacity(paths.len());
        for path in &paths {
            let name = path.file_name().map_or_else(
                || path.display().to_string(),
                |n| n.to_string_lossy().into(),
            );
            let src = std::fs::read_to_string(path).map_err(|e| CatalogError::Io {
                path: path.display().to_string(),
                detail: e.to_string(),
            })?;
            activities.push(activity_from(&name, &src)?);
        }
        activities.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(Self { activities })
    }

    /// **Find activities by declared keyword** (AC-436; Step 7d Slice 3).
    ///
    /// Matching is case-insensitive and on WHOLE keywords, not substrings: a catalog searched by
    /// substring would return `plan` for a query of `an`, and a search whose results a reader cannot
    /// predict is one they stop trusting. The comparison is ASCII-lowercase on both sides, which is
    /// what the declared terms are.
    ///
    /// Results come back in the catalog's own order — which `Catalog` already holds sorted — so two
    /// searches of the same catalog agree, and a caller that prints them prints the same thing
    /// twice. **This reads `keywords` and nothing else**: not `id`, not `tags`, not
    /// `applicable_if`. Searching the admissibility inputs would reintroduce, through the back door,
    /// exactly the coupling `keywords` exists to break — and `ac436_catalog_search_finds_by_keyword`
    /// asserts the discriminating case, a query that matches a keyword and appears nowhere in the
    /// activity's other fields.
    #[must_use]
    pub fn search(&self, query: &str) -> Vec<&Activity> {
        let needle = query.trim().to_ascii_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }
        self.activities()
            .iter()
            .filter(|a| {
                a.keywords
                    .iter()
                    .any(|k| k.trim().to_ascii_lowercase() == needle)
            })
            .collect()
    }

    /// Every activity, by id.
    #[must_use]
    pub fn activities(&self) -> &[Activity] {
        &self.activities
    }

    /// How many activities were read. A rule asserts this is non-zero before it reports green.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.activities.len()
    }

    /// `true` when nothing was read.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.activities.is_empty()
    }
}

/// **The loader's diagnostic mode** — every load defect typed, instead of the first (RT-54,
/// `s7d` §7d.1 edit (4), §7d.8). `doctor` is its only caller.
///
/// **It is NOT the strict loader called once per source.** `s7d` §7d.1 edit (4) names that
/// alternative and rejects it — *"a pack with two defects would surface one"* — and the sentence is
/// only true of two defects in **one** activity file, which is the case [`walk`] exists to handle.
/// So the accumulation is inside the activity walk, and both loader modes read the same walk:
/// [`Catalog::from_sources`] takes its first diagnostic, this keeps them all.
///
/// *(dated 2026-09-26, s8c: the loader now reads each STEP's `timeout_secs` (record B), refusing
/// a malformed one as a `Shape`; `RK-D-TIMEOUT` still reads the pack's `[activity.<id>]` values
/// itself, and reads these steps for its third arm.)*
/// **It types only what the loader sees** (§7d.8). The loader reads no `timeout_secs` and validates
/// no facet, so `RK-D-TIMEOUT` and the three `RK-D-FACET-*` rows are `rk-cli`'s own and no widening
/// happens here; the one row this maps is [`DiagnosticKind::GateOutputs`] → `RK-D-GATE-OUTPUTS`.
///
/// **An empty source set is a diagnostic, not an empty success** — [`Catalog::from_sources`]'
/// refusal, carried rather than raised, for the reason that refusal gives: a rule that enumerates
/// nothing passes.
#[must_use]
pub fn load_diagnostic(sources: &[(&str, &str)]) -> CatalogLoad {
    if sources.is_empty() {
        return CatalogLoad {
            catalog: Catalog::default(),
            diagnostics: vec![diagnose(empty_sources())],
        };
    }
    let mut diagnostics = Vec::new();
    let mut activities = Vec::with_capacity(sources.len());
    for (name, src) in sources {
        if let Some(activity) = walk(name, src, &mut diagnostics) {
            activities.push(activity);
        }
    }
    activities.sort_by(|a: &Activity, b: &Activity| a.id.cmp(&b.id));
    CatalogLoad {
        catalog: Catalog { activities },
        diagnostics,
    }
}

/// [`load_diagnostic`] over every `*.toml` in a directory — the form the composition root uses,
/// because the packs `doctor` reads are on disk.
///
/// **This is where `Io` diagnostics actually come from.** An unreadable directory, and a file that
/// cannot be read, are each typed and carried rather than raised: `doctor` reports a pack it could
/// not open as a finding, the same as one that failed to parse.
#[must_use]
pub fn load_diagnostic_dir(dir: &Path) -> CatalogLoad {
    let io = |detail: String| CatalogDiagnostic {
        error: CatalogError::Io {
            path: dir.display().to_string(),
            detail,
        },
        kind: DiagnosticKind::Io,
    };
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            return CatalogLoad {
                catalog: Catalog::default(),
                diagnostics: vec![io(e.to_string())],
            };
        }
    };
    let mut paths: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort();
    if paths.is_empty() {
        return CatalogLoad {
            catalog: Catalog::default(),
            diagnostics: vec![io(
                "holds no *.toml — an empty catalog is an error, not an empty success".to_owned(),
            )],
        };
    }
    // Read first, then hand the whole set to `load_diagnostic`: a file that cannot be READ is an
    // `Io` defect of its own and must not stop the files after it from being walked.
    let mut read = Vec::with_capacity(paths.len());
    let mut unreadable = Vec::new();
    for path in &paths {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |n| n.to_string_lossy().into(),
        );
        match std::fs::read_to_string(path) {
            Ok(src) => read.push((name, src)),
            Err(e) => unreadable.push(CatalogDiagnostic {
                error: CatalogError::Io {
                    path: path.display().to_string(),
                    detail: e.to_string(),
                },
                kind: DiagnosticKind::Io,
            }),
        }
    }
    let sources: Vec<(&str, &str)> = read.iter().map(|(n, s)| (n.as_str(), s.as_str())).collect();
    let mut load = if sources.is_empty() {
        CatalogLoad::default()
    } else {
        load_diagnostic(&sources)
    };
    // The unreadable files come FIRST, in path order, so the diagnostic list reads in the order the
    // directory was walked rather than in two blocks whose order depends on which failure happened.
    unreadable.append(&mut load.diagnostics);
    load.diagnostics = unreadable;
    load
}

/// The refusal [`Catalog::from_sources`] raises for an empty set, in one place so the strict and
/// the diagnostic paths cannot drift apart on its wording.
fn empty_sources() -> CatalogError {
    CatalogError::Io {
        path: "<sources>".to_owned(),
        detail: "no activity sources were given — an empty catalog is an error, because a rule \
                 that enumerates nothing passes"
            .to_owned(),
    }
}

/// A strict error's diagnostic discriminant. Transcription, not judgement — the one defect that is
/// typed apart from its variant is the AC-409 gate refusal, and [`walk`] pushes that one with its
/// [`DiagnosticKind::GateOutputs`] written at the site rather than inferred here.
const fn classify(error: &CatalogError) -> DiagnosticKind {
    match error {
        CatalogError::Io { .. } => DiagnosticKind::Io,
        CatalogError::Toml { .. } => DiagnosticKind::Toml,
        CatalogError::Shape { .. } => DiagnosticKind::Shape,
    }
}

/// A strict error as a diagnostic, typed by [`classify`].
const fn diagnose(error: CatalogError) -> CatalogDiagnostic {
    let kind = classify(&error);
    CatalogDiagnostic { error, kind }
}

/// Walk one activity source, recording **every** defect in `out` rather than stopping at the
/// first, and returning the activity when one could be built.
///
/// **This is the ONE walker**, and both loader modes are readings of it: [`Catalog::from_sources`]
/// takes its first diagnostic as the error, [`load_diagnostic`] keeps them all. That makes
/// `s7d` §7d.14's second red — *"the strict loader's first-error result differing from
/// `load_diagnostic`'s first typed error"* — impossible **by construction** rather than by care. A
/// second walker would be a second place for every rule below to be forgotten, which is the reason
/// `bounded_max` further down is one helper and not two.
///
/// **What abandons the source and what does not.** A source whose TOML does not parse, that
/// carries no `[activity]` table, or whose activity declares no `id` is abandoned: every message
/// after it would be about a document this function cannot name. Every other defect is recorded
/// and the walk goes on — a missing `stage` must not hide a malformed step three rows below it,
/// and a refused step must not hide the one after it. That is the whole reason `doctor` reads
/// packs through this entry point: `s7d` §7d.1 edit (4) rejects the per-file alternative *"because
/// a pack with two defects would surface one"*, and the two defects it means are two defects in
/// **one** activity file.
fn walk(name: &str, src: &str, out: &mut Vec<CatalogDiagnostic>) -> Option<Activity> {
    let doc = match toml_subset::parse(src) {
        Ok(doc) => doc,
        Err(error) => {
            out.push(diagnose(CatalogError::Toml {
                source: name.to_owned(),
                error,
            }));
            return None;
        }
    };
    let shape = |detail: String| CatalogError::Shape {
        source: name.to_owned(),
        detail,
    };
    let Some(head) = doc.get("activity").and_then(Value::as_table) else {
        out.push(diagnose(shape("no [activity] table".to_owned())));
        return None;
    };
    let text = |t: &Table, key: &str| t.get(key).and_then(Value::as_str).map(ToOwned::to_owned);
    let strings = |t: &Table, key: &str| -> Vec<String> {
        t.get(key)
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let Some(id) = text(head, "id") else {
        out.push(diagnose(shape("[activity] has no id".to_owned())));
        return None;
    };
    // `stage` and `method_rank` are recorded and the walk CONTINUES, though neither can be
    // defaulted and the activity below will not be built without them. The alternative — returning
    // here — is what made a pack with a missing `stage` and a malformed step report one defect.
    let stage = text(head, "stage");
    if stage.is_none() {
        out.push(diagnose(shape(format!("{id}: no stage"))));
    }
    let method_rank = head.get("method_rank").and_then(Value::as_int);
    if method_rank.is_none() {
        out.push(diagnose(shape(format!("{id}: no method_rank"))));
    }
    // ADR-0046's forbids clause: *"a `deviate` declaration whose `kind` falls outside the closed
    // `skip | downgrade | add` vocabulary"*. A fourth token is a LOAD error and never a silently
    // ignored entry, because an ignored one reads as a permission the activity does not have.
    // The FIRST bad token is recorded and the rest of the list is not re-read: the defect is the
    // declaration, and a list of four bad tokens is one defect stated four times.
    let deviation_ok = strings(head, "deviation_ok");
    for token in &deviation_ok {
        if !matches!(token.as_str(), "skip" | "downgrade" | "add") {
            out.push(diagnose(shape(format!(
                "{id}: deviation_ok carries {token:?}, which is outside ADR-0046's closed \
                 `skip | downgrade | add` vocabulary"
            ))));
            break;
        }
    }
    let budget = match activity_budget(head, &id, &shape) {
        Ok(budget) => budget,
        Err(error) => {
            out.push(diagnose(error));
            None
        }
    };
    let mut steps = Vec::new();
    if let Some(Value::Array(rows)) = doc.get("step") {
        for row in rows {
            let Some(table) = row.as_table() else {
                out.push(diagnose(shape(format!("{id}: a [[step]] is not a table"))));
                continue;
            };
            let Some(step_id) = text(table, "id") else {
                out.push(diagnose(shape(format!("{id}: a step has no id"))));
                continue;
            };
            let Some(kind_token) = text(table, "kind") else {
                out.push(diagnose(shape(format!("{id}/{step_id}: no kind"))));
                continue;
            };
            let Some(kind) = StepKind::from_token(&kind_token) else {
                out.push(diagnose(shape(format!(
                    "{id}/{step_id}: kind {kind_token:?} is outside the schema's five \
                     (agent, tool, check, gate, emit)"
                ))));
                continue;
            };
            let output_type = text(table, "outputs").map(|o| {
                o.strip_prefix("value:")
                    .map_or_else(|| o.clone(), ToOwned::to_owned)
            });
            // **AC-409 (ADR-0045, ADR-0038), as amended 2026-09-18 — C-01 RA-M-05.** *"A gate
            // answer populates the gate step's `answer` and can bind nothing downstream"*, and the
            // criterion's WHEN is *"when the configuration is loaded"* — so the refusal is HERE,
            // in the strict walk `run` loads through, and not in the diagnostic mode alone. Until
            // 2026-09-20 `outputs` was parsed for every `StepKind` and `run` would load the graph
            // AC-409 says is rejected.
            //
            // The amendment is why this is a LOADER rule rather than a type. This crate has one
            // `Step` struct for every kind, so `Step { kind: Gate, output_type: Some(..) }` is
            // representable and the compile-time instrument the criterion first named could not be
            // written; the behaviour — rejection — is unchanged.
            //
            // It is typed `GateOutputs` rather than a plain `Shape` so that `rk-cli`'s
            // `RK-D-GATE-OUTPUTS` row maps a value instead of matching text inside this message.
            // The ERROR stays a `CatalogError::Shape`: the variant set is three and the refusal is
            // among the `Shape`s (`s7d` §7d.8).
            if kind == StepKind::Gate && output_type.is_some() {
                out.push(CatalogDiagnostic {
                    error: shape(format!(
                        "{id}/{step_id}: a `gate` step carries `outputs`. AC-409: a gate answer \
                         populates the gate step's `answer` and binds nothing downstream, so a \
                         graph whose reference resolves to a gate step's `outputs` is a \
                         configuration error"
                    )),
                    kind: DiagnosticKind::GateOutputs,
                });
                continue;
            }
            // ADR-0097 decision 3 withdrew this key; it is read so that the rule that refuses it
            // has something to refuse. A `memo` that is not a boolean is a SHAPE error and never a
            // silently absent declaration -- `retry_max`'s and `activity_budget`'s discipline, and
            // for the same reason: a dropped key reads as a lawful step.
            let memo_declared = match table.get("memo") {
                None => None,
                Some(Value::Bool(b)) => Some(*b),
                Some(_) => {
                    out.push(diagnose(shape(format!(
                        "{id}/{step_id}: `memo` is not a boolean. ADR-0097 decision 3 withdrew the \
                         per-step memo field; a non-boolean value is refused here rather than \
                         dropped, because a dropped key reads as a step that never declared one"
                    ))));
                    continue;
                }
            };
            // 🛑 **ADR-0146 clause 4. A `question` MAY NOT CARRY A COLON, and the refusal is here
            // because this is where the value enters the tree.**
            //
            // `RunState::gates_by_kind` is keyed on the bare kind string, and a selection-level
            // gate's kind is the synthesised `"{reason}:{axis}"` — `unknown_facet:has_repro` and
            // the like (`fsm::next_action` clause 2). Since ADR-0145 a catalog `question` becomes
            // a gate kind too, so a `question = "unknown_facet:has_repro"` lands in the SAME reach
            // sequence as the abstention gate of that axis. Measured on a fixture: the step gate
            // minted **reach 2 of the selection gate's sequence**, `latest_gate` then pointed at
            // the step gate's token, and — because `fsm`'s clause 2 opens only when
            // `reaches == 0` — the abstention gate of that axis **could no longer be opened at
            // all**. A catalog edit that silently suppresses an abstention, unsatisfiable or
            // appetite gate is a much larger hole than the one ADR-0145 set out to close.
            //
            // Zero of the shipped pack's 45 distinct questions contains a colon, so this refuses
            // nothing that exists today; it is the guard that keeps it that way. A colon is banned
            // rather than the exact `"{reason}:{axis}"` shape, because `GateReason` is a closed set
            // that a later record may widen, and a ban that tracked its members would go stale the
            // moment it did — `fsm.rs`'s own header warns that two gates differing only in axis
            // must not collapse into one.
            let question = text(table, "question");
            if let Some(q) = question.as_deref()
                && q.contains(':')
            {
                out.push(diagnose(shape(format!(
                    "{id}/{step_id}: `question` is {q:?}, which carries a `:`. A gate step's \
                     `question` IS its gate kind (ADR-0145), and the selection's own gates are \
                     keyed `\"{{reason}}:{{axis}}\"` — so a colon admits a catalog step into an \
                     abstention gate's reach sequence, where it can suppress that gate entirely"
                ))));
                continue;
            }
            // **Record B decision 1.** A `timeout_secs` that is neither a non-negative integer nor
            // an exact `${toolchain.<key>}` is refused, never dropped: a dropped declaration reads
            // as a step that declared none, and would hand the port an undeclared bound.
            let timeout_secs = match table.get("timeout_secs") {
                None => None,
                Some(v) => {
                    if let Some(t) = StepTimeout::from_value(v) {
                        Some(t)
                    } else {
                        out.push(diagnose(shape(format!(
                            "{id}/{step_id}: `timeout_secs` is neither a non-negative integer nor an exact \
                             `${{toolchain.<key>}}` substitution (ADR-0171 decision 1)"
                        ))));
                        continue;
                    }
                }
            };
            let retry_max = match bounded_max(table, "retry", &id, &shape) {
                Ok(max) => max,
                Err(error) => {
                    out.push(diagnose(error));
                    continue;
                }
            };
            let repeat_max = match bounded_max(table, "repeat", &id, &shape) {
                Ok(max) => max,
                Err(error) => {
                    out.push(diagnose(error));
                    continue;
                }
            };
            // **R2's parsed-`inputs` carrier** (ADR-0173, written `proposed`; D21-57). A value
            // outside the carrier's three scalar kinds, or an `inputs` that is not a table, is
            // refused by name and never dropped: a dropped binding reads as one never declared.
            let inputs = match step_inputs(table) {
                Ok(inputs) => inputs,
                Err(detail) => {
                    out.push(diagnose(shape(format!("{id}/{step_id}: {detail}"))));
                    continue;
                }
            };
            // **R2 decision 7** (ADR-0173, written `proposed`): the `emit` binding, refused by name
            // rather than dropped when it is not the declared shape.
            let emit = match emit_binding(table) {
                Ok(emit) => emit,
                Err(detail) => {
                    out.push(diagnose(shape(format!("{id}/{step_id}: {detail}"))));
                    continue;
                }
            };
            let sub = |key: &str, member: &str| -> Option<String> {
                table
                    .get(key)
                    .and_then(Value::as_table)
                    .and_then(|t| t.get(member))
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
            };
            steps.push(Step {
                id: step_id,
                kind,
                run: text(table, "run"),
                output_type,
                // ADR-0014's *"an activity nominates a ROLE … never a card"*, transcribed and not
                // interpreted: the role VOCABULARY is the pack's, so a role this crate has never
                // heard of is carried here and refused at the resolution
                // (`Pack::require_family_for_role`) rather than at load. A token outside a closed
                // set is a load error where the set is closed by a record — `deviation_ok` above —
                // and roles are closed by no record at all.
                role: text(table, "role"),
                when: text(table, "when"),
                expr: text(table, "expr"),
                for_each: text(table, "for_each"),
                retry_max,
                repeat_max,
                memo_declared,
                // **ADR-0145.** The gate's kind, transcribed and not interpreted — `role`'s rule
                // one field over. Read for EVERY kind rather than only `gate`, deliberately: a
                // `question` on a non-gate step is a declaration this crate should be able to see
                // in order to refuse it, and a key read only where it is expected is a key that
                // goes missing silently everywhere else.
                question,
                timeout_secs,
                inputs,
                on_false_to: sub("on_false", "to"),
                repeat_until: sub("repeat", "until"),
                escalate_question: sub("escalate", "question"),
                on_false: table.get("on_false").map(toml_to_json),
                on_exhausted: text(table, "on_exhausted"),
                emit,
            });
        }
    }
    // 🛑 **`s8d` §8d.8 defect 1 (R2 decision 7): A STEP AFTER AN `emit` IS REFUSED AT LOAD, BY
    // NAME.** design §4.5 makes `emit` terminal, and the walk's arm read
    // `StepVerdict::Emitted => break`, which dropped every later step in silence. Measured over
    // `.redkiln/catalog`, `spikes/p9/catalog`, `catalog-b` and every fixture catalog: 0 activities
    // declare a step after their `emit`, so this refuses nothing that exists; it is the guard that
    // keeps a later step from being declared and never run.
    if let Some(e) = steps.iter().position(|s| s.kind == StepKind::Emit)
        && let Some(after) = steps.get(e + 1)
    {
        out.push(diagnose(shape(format!(
            "{id}/{}: the step follows the `emit` step `{}`. An `emit` is terminal (design §4.5), \
             so a later step would never run; it is refused by name rather than dropped (R2 = \
             ADR-0173 decision 7; `s8d` §8d.8 defect 1)",
            after.id, steps[e].id
        ))));
    }
    // ADR-0072's `@version`. Absent is `None` and never `1` -- see `Activity::version`. A malformed
    // one abandons the activity rather than loading it as `None`: a version that silently became
    // absent reads as an activity that never declared one, which is `memo`'s rule one field over.
    let version = match head.get("version") {
        None => None,
        Some(v) => {
            let Some(raw) = v.as_int() else {
                out.push(diagnose(shape(format!(
                    "{id}: `version` is not an integer"
                ))));
                return None;
            };
            match u32::try_from(raw) {
                Ok(v) => Some(v),
                Err(_) => {
                    out.push(diagnose(shape(format!(
                        "{id}: `version` is {raw}, which is outside u32"
                    ))));
                    return None;
                }
            }
        }
    };
    // The two fields that cannot be defaulted. Their defects are already recorded above; this is
    // where the activity stops being buildable, not where the reader is told why.
    let (Some(stage), Some(method_rank)) = (stage, method_rank) else {
        return None;
    };
    Some(Activity {
        source: name.to_owned(),
        id,
        version,
        stage,
        provides: strings(head, "provides"),
        tags: strings(head, "tags"),
        keywords: strings(head, "keywords"),
        applicable_if: text(head, "applicable_if"),
        requires: strings(head, "requires")
            .iter()
            .map(|r| Requirement::parse(r))
            .collect(),
        derived_from: strings(head, "derived_from")
            .iter()
            .map(|r| Requirement::parse(r))
            .collect(),
        cost_class: text(head, "cost_class"),
        excludes: strings(head, "excludes"),
        deviation_ok,
        budget,
        method_rank,
        min_rigor: text(head, "min_rigor").unwrap_or_else(|| "optional".to_owned()),
        max_rigor: text(head, "max_rigor"),
        family: text(head, "family"),
        steps,
    })
}

/// The strict loader: [`walk`], stopping at the first diagnostic.
///
/// A source with **any** defect is refused whole, exactly as it was before the walk began
/// accumulating — an activity that was built around a skipped step is not a lawful activity, it is
/// one the walk was able to describe further.
fn activity_from(name: &str, src: &str) -> Result<Activity, CatalogError> {
    let mut diagnostics = Vec::new();
    let activity = walk(name, src, &mut diagnostics);
    if let Some(first) = diagnostics.into_iter().next() {
        return Err(first.error);
    }
    // Unreachable as `walk` is written — every `None` path records a diagnostic first — and it is a
    // value rather than a panic so that an edit which forgets to is a load error and not an abort.
    activity.ok_or_else(|| CatalogError::Shape {
        source: name.to_owned(),
        detail: "the activity could not be built and no defect was recorded".to_owned(),
    })
}

/// One `[[step]]`'s `inputs` table onto [`Step::inputs`]: absent is empty; a table of strings,
/// integers and booleans is carried; anything else is an error naming the binding.
/// A TOML value as JSON, for a key carried verbatim ([`Step::on_false`]). A float that JSON cannot
/// carry (a NaN or an infinity) becomes `null`, which the one classifier refuses by name.
fn toml_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Int(n) => serde_json::Value::from(*n),
        Value::Float(f) => {
            serde_json::Number::from_f64(*f).map_or(serde_json::Value::Null, Into::into)
        }
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Array(items) => serde_json::Value::Array(items.iter().map(toml_to_json).collect()),
        Value::Table(t) => serde_json::Value::Object(
            t.iter()
                .map(|(k, v)| (k.clone(), toml_to_json(v)))
                .collect(),
        ),
    }
}

fn step_inputs(table: &Table) -> Result<BTreeMap<String, InputValue>, String> {
    let Some(value) = table.get("inputs") else {
        return Ok(BTreeMap::new());
    };
    let inputs = value
        .as_table()
        .ok_or_else(|| "`inputs` is not a table (ADR-0040's step input grammar)".to_owned())?;
    inputs
        .iter()
        .map(|(binding, v)| {
            let carried = match v {
                Value::Str(s) => InputValue::Text(s.clone()),
                Value::Int(n) => InputValue::Int(*n),
                Value::Bool(b) => InputValue::Bool(*b),
                other => {
                    return Err(format!(
                        "`inputs.{binding}` is {other:?}, which is none of a source string, an \
                         integer or a boolean — refused rather than dropped, because a dropped \
                         value reads as a binding never declared"
                    ));
                }
            };
            Ok((binding.clone(), carried))
        })
        .collect()
}

/// One `[[step]]`'s `artifacts` table and `evidence` list onto [`Step::emit`]: absent both is
/// `None`; an artifact bound to a string or to a table of scalar sources is carried; anything else
/// is an error naming the key (R2 decision 7).
fn emit_binding(table: &Table) -> Result<Option<EmitBinding>, String> {
    let artifacts = table.get("artifacts");
    let evidence = table.get("evidence");
    if artifacts.is_none() && evidence.is_none() {
        return Ok(None);
    }
    let mut out = EmitBinding {
        artifacts: BTreeMap::new(),
        evidence: Vec::new(),
    };
    if let Some(value) = artifacts {
        let rows = value
            .as_table()
            .ok_or_else(|| "`artifacts` is not a table (R2 decision 7)".to_owned())?;
        for (kind, v) in rows {
            let bound = match v {
                Value::Str(s) => EmitArtifact::Ref(s.clone()),
                Value::Table(members) => {
                    let mut composed = BTreeMap::new();
                    for (member, m) in members {
                        let carried = match m {
                            Value::Str(s) => InputValue::Text(s.clone()),
                            Value::Int(n) => InputValue::Int(*n),
                            Value::Bool(b) => InputValue::Bool(*b),
                            other => {
                                return Err(format!(
                                    "`artifacts.{kind}.{member}` is {other:?}, which is none of a \
                                     source string, an integer or a boolean"
                                ));
                            }
                        };
                        composed.insert(member.clone(), carried);
                    }
                    EmitArtifact::Compose(composed)
                }
                other => {
                    return Err(format!(
                        "`artifacts.{kind}` is {other:?}, which is neither a source string nor a \
                         composing table"
                    ));
                }
            };
            out.artifacts.insert(kind.clone(), bound);
        }
    }
    if let Some(value) = evidence {
        let items = value
            .as_array()
            .ok_or_else(|| "`evidence` is not an array (R2 decision 7)".to_owned())?;
        for item in items {
            let text = item
                .as_str()
                .ok_or_else(|| format!("an `evidence` entry is {item:?}, not a reference"))?;
            out.evidence.push(text.to_owned());
        }
    }
    Ok(Some(out))
}

// ---- the forward-ref licence (D21-48 X8; R2 = ADR-0173, written `proposed`) -------------------------

/// Which licence admits a reference to a later-declared step (`s8d` §8d.3, *Which forward refs are
/// licensed*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Licence {
    /// **ADR-0042's letter.** The referenced step, or a step after it, has `on_false = { to = R }`
    /// for the referencing step R.
    Letter,
    /// **The repeat body (D21-48 X6).** R declares `repeat`, and the referenced step lies within
    /// R's body, which runs through the last step R's `until` references.
    RepeatBody,
    /// **The escalate re-entry (D21-48 X8).** R declares `escalate = { question = Q }`, and the
    /// referenced step, or a step after it, is a `gate` step whose `question` is Q: answering it
    /// resumes into R (schema §3).
    EscalateReEntry,
}

impl Licence {
    /// The census token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Letter => "letter",
            Self::RepeatBody => "repeat",
            Self::EscalateReEntry => "escalate",
        }
    }
}

/// One `${<step>…}` in a step's `inputs` whose step is declared LATER in the same activity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardRef {
    pub activity: String,
    /// The referencing step (R).
    pub step: String,
    /// The `inputs` binding that carries the reference.
    pub binding: String,
    /// The reference as written, `${…}` included.
    pub reference: String,
    /// The later-declared step it names (L).
    pub target: String,
    /// `true` for the `${…?}` spelling.
    pub optional: bool,
    /// The first licence that admits it, in the order the enum declares them; `None` when none
    /// does.
    pub licence: Option<Licence>,
}

impl ForwardRef {
    /// Admitted at load: optional **and** licensed. A forward ref without `?` is refused whatever
    /// licence it has, because it is `null` on the first pass (ADR-0042) and a mandatory input
    /// cannot be `null`.
    #[must_use]
    pub const fn is_admitted(&self) -> bool {
        self.optional && self.licence.is_some()
    }
}

impl fmt::Display for ForwardRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let why = if self.licence.is_none() {
            "no licence covers it: no `on_false` targets the referencing step from the named step \
             or after it (ADR-0042's letter), the named step is outside the step's repeat body \
             (D21-48 X6), and no later `gate` asks the question the step escalates to (D21-48 X8)"
        } else {
            "it is licensed, but it carries no `?`, and a forward ref is `null` on the first pass \
             (ADR-0042)"
        };
        write!(
            f,
            "{}/{}: `{} = \"{}\"` reads the later-declared step `{}`, and {why}",
            self.activity, self.step, self.binding, self.reference, self.target
        )
    }
}

/// Every `${…}` in a string, as `(name, whole reference, optional)`. `name` is the text up to the
/// first `.`, `?` or `[`, which is a step id when the activity declares one of that id.
fn step_refs(src: &str) -> Vec<(String, String, bool)> {
    let mut out = Vec::new();
    let mut rest = src;
    while let Some(i) = rest.find("${") {
        let tail = &rest[i + 2..];
        let Some(end) = tail.find('}') else {
            break;
        };
        let inner = &tail[..end];
        let name_end = inner.find(['.', '?', '[']).unwrap_or(inner.len());
        out.push((
            inner[..name_end].to_owned(),
            format!("${{{inner}}}"),
            inner.ends_with('?'),
        ));
        rest = &tail[end + 1..];
    }
    out
}

/// Every forward ref in one activity's `inputs`, each classified by the three licences in order.
#[must_use]
pub fn forward_refs(activity: &Activity) -> Vec<ForwardRef> {
    let steps = &activity.steps;
    let index: BTreeMap<&str, usize> = steps
        .iter()
        .enumerate()
        .map(|(i, s)| (s.id.as_str(), i))
        .collect();
    let mut out = Vec::new();
    for (r, step) in steps.iter().enumerate() {
        for (binding, value) in &step.inputs {
            let Some(text) = value.as_text() else {
                continue;
            };
            for (name, reference, optional) in step_refs(text) {
                let Some(&l) = index.get(name.as_str()) else {
                    continue;
                };
                if l <= r {
                    continue; // a prior step, or a self-reference (ADR-0055): not forward
                }
                out.push(ForwardRef {
                    activity: activity.id.clone(),
                    step: step.id.clone(),
                    binding: binding.clone(),
                    reference,
                    target: name,
                    optional,
                    licence: licence(steps, &index, r, l),
                });
            }
        }
    }
    out
}

/// **The repeat body's last step** (D21-48 X6; R2 decision 4), as an index into `steps`.
///
/// *"The repeat body runs from the repeating step through the last step `until` references, in
/// declaration order"*: that step's index, and `r` itself when `until` reads only the step's own
/// outputs (or earlier steps), or is absent. ONE definition, read by the forward-ref licence and by
/// the walk's loop executor alike (`s8d` package P5), so the body the lint licenses is the body the
/// kernel runs.
#[must_use]
pub fn repeat_body_end(steps: &[Step], r: usize) -> usize {
    let index: BTreeMap<&str, usize> = steps
        .iter()
        .enumerate()
        .map(|(i, s)| (s.id.as_str(), i))
        .collect();
    repeat_body_end_in(steps, &index, r)
}

fn repeat_body_end_in(steps: &[Step], index: &BTreeMap<&str, usize>, r: usize) -> usize {
    steps
        .get(r)
        .and_then(|s| s.repeat_until.as_deref())
        .map_or(r, |until| {
            step_refs(until)
                .iter()
                .filter_map(|(name, _, _)| index.get(name.as_str()).copied())
                .max()
                .map_or(r, |end| end.max(r))
        })
}

/// The first licence admitting a ref from step `r` to the later step `l`.
fn licence(steps: &[Step], index: &BTreeMap<&str, usize>, r: usize, l: usize) -> Option<Licence> {
    let referencing = &steps[r];
    // 1. ADR-0042's letter: `l`, or a step after it, loops back to `r`.
    if steps[l..]
        .iter()
        .any(|s| s.on_false_to.as_deref() == Some(referencing.id.as_str()))
    {
        return Some(Licence::Letter);
    }
    // 2. The repeat body (X6): `r` repeats, and `l` lies at or before the last step `until` reads.
    if referencing.repeat_until.is_some() && l <= repeat_body_end_in(steps, index, r) {
        return Some(Licence::RepeatBody);
    }
    // 3. The escalate re-entry (X8): `l`, or a step after it, is a gate asking `r`'s question.
    if let Some(q) = referencing.escalate_question.as_deref()
        && steps[l..]
            .iter()
            .any(|s| s.kind == StepKind::Gate && s.question.as_deref() == Some(q))
    {
        return Some(Licence::EscalateReEntry);
    }
    None
}

/// What the forward-ref lint walked: every forward ref, by licence, and the refused ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ForwardRefCensus {
    pub activities: usize,
    pub total: usize,
    pub letter: usize,
    pub repeat: usize,
    pub escalate: usize,
    /// Refused: no licence, or licensed and mandatory.
    pub unlicensed: usize,
}

impl fmt::Display for ForwardRefCensus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FORWARD_REFS total={} letter={} repeat={} escalate={} unlicensed={}",
            self.total, self.letter, self.repeat, self.escalate, self.unlicensed
        )
    }
}

/// The census over a catalog, and every refused forward ref.
#[must_use]
pub fn forward_ref_census(catalog: &Catalog) -> (ForwardRefCensus, Vec<ForwardRef>) {
    let mut census = ForwardRefCensus {
        activities: catalog.len(),
        ..ForwardRefCensus::default()
    };
    let mut refused = Vec::new();
    for activity in catalog.activities() {
        for fr in forward_refs(activity) {
            census.total += 1;
            match (fr.is_admitted(), fr.licence) {
                (true, Some(Licence::Letter)) => census.letter += 1,
                (true, Some(Licence::RepeatBody)) => census.repeat += 1,
                (true, Some(Licence::EscalateReEntry)) => census.escalate += 1,
                _ => {
                    census.unlicensed += 1;
                    refused.push(fr);
                }
            }
        }
    }
    (census, refused)
}

/// **The forward-ref licence lint** (D21-48 X8; R2 = ADR-0173, written `proposed`).
///
/// R2 supersedes ADR-0042's Decision and first forbids bullet in part. A ref from step R to a
/// later-declared step is admitted only when it carries `?` and one of the three [`Licence`]s
/// covers it; every other ref is refused by name — activity, step and reference.
///
/// **Where it runs.** At load, through `select::load_lints` — the refusing load rules every
/// shipped path into selection runs before step 1 — and not in the strict parser [`walk`]. The
/// frozen p9 controls load `spikes/p9/catalog` at `P9_PIN` through [`Catalog::from_dir`], and that
/// catalog carries the four refs this stage removes from `.redkiln/` (`s8d` §8d.3); a refusal in
/// the parser would make every frozen p9 reading unloadable rather than frozen. The rule is
/// catalog-only, so it needs neither registry.
///
/// # Errors
///
/// Every refused forward ref, never only the first.
pub fn forward_ref_lint(catalog: &Catalog) -> Result<ForwardRefCensus, Vec<ForwardRef>> {
    let (census, refused) = forward_ref_census(catalog);
    if refused.is_empty() {
        Ok(census)
    } else {
        Err(refused)
    }
}

/// `<key> = { max = N, … }` on one `[[step]]`, or `None`. Used for `retry` (Slice 4a) and for
/// `repeat` (Slice 4g) — the two step tables of schema §4 whose only member this crate reads is
/// `max`.
///
/// **One helper, not two, and the reason is the failure it prevents**: a second copy would be a
/// second place for the never-a-silent-zero rule to be forgotten.
///
/// # Errors
///
/// [`CatalogError::Shape`] for a `<key>` that is not a table, a `max` that is not an integer, or one
/// outside `u32` — never a silent `0`, which would make the losing formula
/// `attempts_total_counts_a_pass_not_retries` computes read as `A` and the test vacuous, and which
/// would make a `repeat` loop read as a step that runs once.
fn bounded_max(
    table: &Table,
    key: &str,
    id: &str,
    shape: &impl Fn(String) -> CatalogError,
) -> Result<Option<u32>, CatalogError> {
    let Some(value) = table.get(key) else {
        return Ok(None);
    };
    let inner = value
        .as_table()
        .ok_or_else(|| shape(format!("{id}: `{key}` is not a table")))?;
    let Some(max) = inner.get("max") else {
        return Ok(None);
    };
    let raw = max
        .as_int()
        .ok_or_else(|| shape(format!("{id}: `{key}.max` is not an integer")))?;
    u32::try_from(raw)
        .map(Some)
        .map_err(|_| shape(format!("{id}: `{key}.max` is {raw}, which is outside u32")))
}

/// The `[activity]` table's `budget = { attempts_total, tokens, wall_clock_secs }`, or `None`.
///
/// **All three keys or none.** A partial table is a shape error rather than a table with two
/// counters and a defaulted third: the defaulted one would be a policy number this crate does not
/// own (`00-preamble.md`:55), and it would be invisible at the call site.
///
/// # Errors
///
/// [`CatalogError::Shape`] for a `budget` that is not a table, a missing or non-integer member, or a
/// negative one.
fn activity_budget(
    head: &Table,
    id: &str,
    shape: &impl Fn(String) -> CatalogError,
) -> Result<Option<ActivityBudget>, CatalogError> {
    let Some(value) = head.get("budget") else {
        return Ok(None);
    };
    let table = value
        .as_table()
        .ok_or_else(|| shape(format!("{id}: `budget` is not a table")))?;
    let member = |key: &str| -> Result<i64, CatalogError> {
        let raw = table
            .get(key)
            .and_then(Value::as_int)
            .ok_or_else(|| shape(format!("{id}: `budget` has no integer `{key}`")))?;
        if raw < 0 {
            return Err(shape(format!(
                "{id}: `budget.{key}` is {raw}, which is negative"
            )));
        }
        Ok(raw)
    };
    let attempts = member("attempts_total")?;
    let tokens = member("tokens")?;
    let wall_clock = member("wall_clock_secs")?;
    let attempts_total = u32::try_from(attempts).map_err(|_| {
        shape(format!(
            "{id}: `budget.attempts_total` is {attempts}, which is outside u32"
        ))
    })?;
    Ok(Some(ActivityBudget {
        attempts_total: crate::budget::Attempts::new(attempts_total),
        tokens: crate::budget::Tokens::new(tokens.unsigned_abs()),
        wall_clock_secs: crate::budget::WallClockSecs::new(wall_clock.unsigned_abs()),
    }))
}
