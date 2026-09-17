//! **The memo** — one key assembled here, two more that must not be confused with it, and the two
//! lints ADR-0059 and ADR-0097 decision 3 leave behind (**AC-014–AC-019**).
//!
//! # The three keys, quoted from the crate that owns them
//!
//! `crates/rk-store/src/memo.rs`:12-16 carries the table, and it is quoted rather than paraphrased
//! because the whole module turns on the three being different formulas:
//!
//! | Key | Formula | Where |
//! | --- | --- | --- |
//! | step memo | SHA-256 over the eight-component length-prefixed tuple | `rk_store::memo::StepKeyComponents` |
//! | activity-level (AC-016) | SHA-256 over the activity's reachable **card digests**, sorted | `rk_store::memo::activity_key` |
//! | idempotency | `(item, activity_id@version, attempt)` — **three parts, never a fourth** | `rk_store::memo::IdempotencyKey` |
//!
//! *"They deliberately do not share code. A helper that computed 'the key' would be the thing that
//! makes two of them accidentally equal, and AC-014/AC-016/AC-019 exist because that distinction is
//! load-bearing."*
//!
//! # 🛑 `StepKeyComponents` DOES NOT MOVE, AND THIS MODULE DOES NOT RE-DERIVE A KEY
//!
//! ADR-0119 clause 1 is explicit: *"`StepKeyComponents`, `COMPONENT_COUNT` and `activity_key` in
//! `crates/rk-store/src/memo.rs` are **not moved** by this record."* [`StepContext`] is the
//! **runtime facts** the eight components are assembled FROM, one field per component in ADR-0097's
//! order, and [`StepContext::components`] is an **assembly**: it canonicalises component 4, tags
//! component 5's branch, passes component 8's schema id through **whole and unparsed**, and hands
//! the result to `rk-store`, whose `digest()` is the only hash. A
//! second derivation in this crate is the accidental-equality hazard the quoted sentence exists to
//! prevent — and it would also make both crates' tests true of two different keys.
//!
//! Editing `rk-store` was the other option and it is refused for a second reason:
//! `00-preamble.md`:232-237 makes Checkpoints 5 and 6 **provisional** for any edit to a crate they
//! passed over, and both passed over `rk-store`.
//!
//! # AC-014's eight components, verbatim, and what each one is here
//!
//! > *"(1) `activity_id@version`; (2) the rigor variant selected; (3) the step id within the
//! > activity graph; (4) the digest of each resolved step input in declaration order, each
//! > canonicalised by `rk-store`'s canonical serialiser (sorted keys, fixed float format, LF);
//! > (5) the card digest, or for a `tool` step the Toolchain adapter's detected version string plus
//! > the command; (6) the digests of the resolved KB atoms, sorted; (7) the loop iteration counter
//! > (0 outside a loop); (8) the output type's `schema_version`."*
//!
//! Component 1's carrier is `crate::catalog::Activity::id_at_version`, **added by this slice**, and
//! the reason it had to be added is written into that field's own doc: Slice 4e reported *"`@version`
//! has no carrier in this tree"* from a grep of `catalog.rs`, and the measurement that matters is
//! the other one — **all 102 shipped activities declare `version`** and so does every catalog
//! fixture [measured 2026-09-14]. The loader dropped it; the files always carried it.
//!
//! # 🛑 ONE COMPONENT HAS NO PRODUCER IN THIS TREE, AND IT IS NOT INVENTED HERE
//!
//! *(It was two until 2026-09-14. Component 8's source was decided that day and the section
//! below this one carries it; component 5's is still owed.)*
//!
//! - **Component 5's tool branch.** `crates/rk-ports/src/toolchain.rs`:47 is
//!   `fn detect(&self) -> Result<bool, StepError>` — a **bool**, not a version string — and
//!   `toolchain.rs`:8-12 records *"whether `detect` reports a bool or a version"* as a routing no
//!   record fixes. There is no Toolchain adapter at 7b (§7b.15). So [`CardOrTool::Tool`] takes the
//!   version string **as a value the composition root supplies**, the source of that value is
//!   **OWED**, and every Slice-4 activity fixture is kept to `agent` steps so the branch stays
//!   unexercised rather than quietly filled. Sourcing it from `detect()`'s bool would give every
//!   toolchain the string `"true"`, so two different tools would share a key and one tool's
//!   memoised result would be served to another — precisely the collision `sha256_lp`'s length
//!   framing exists to prevent (`crates/rk-types/src/digest.rs`:395-401).
//!
//! # COMPONENT 8 IS `ActivitySpec.schema`'s WHOLE ID — DECIDED 2026-09-14
//!
//! AC-014 calls component 8 *"the output type's `schema_version`"* and **there is no such field**
//! [measured 2026-09-14: `grep -rn schema_version crates/rk-types/src crates/rk-ports/src` → **0**;
//! across the whole workspace it appears only on `rk-store`'s SQLite identity row and on
//! `StepKeyComponents` itself]. Design §3.5 names a field that does not exist.
//!
//! [`StepContext::output_schema`] is therefore a [`rk_types::typed::SchemaId`] — the id carried by
//! [`rk_types::activity::ActivitySpec::schema`] — and [`StepContext::components`] hands
//! `SchemaId::as_str()` to `rk-store` **whole and unparsed**. Four reasons, in the order they were
//! weighed: it is the **only** value in the tree carrying a version; it is **already on the spec**
//! where the key is built; AC-014's property holds, because `…/v1` → `…/v2` moves the key; and it
//! also moves when the output **kind** does, which is correct — a different output type is
//! different work. The id appears nowhere else among the eight, so nothing is double-counted.
//!
//! **🛑 PARSING THE `vN` SUFFIX WAS REJECTED**, and the reason is a collision rather than a taste:
//! it invents a convention no record states, has no defined behaviour on an id carrying no suffix,
//! and would give **every output type at `/v1` the same component 8** — so two different output
//! types share a key and one type's memoised result is served for the other. That is the weakening
//! of AC-014's nine-case property `memo_key_eight_components` exists to prevent, and
//! `tests/it/memo.rs`'s `component_eight_is_the_whole_schema_id_and_never_a_parsed_version` is red
//! under the parse and green under this reading.
//!
//! **Corrected here, not quietly replaced.** What stood in this header until 2026-09-14 was
//! *"**`SchemaId::as_str()` is explicitly not used**: it is an id, not a version, and sourcing
//! component 8 from it would make the key move on a rename and stand still on a schema change —
//! which inverts AC-014's THEN"*. Its first half is now the implementation, and its second half is
//! the cost the decision **accepted**: a rename of an output type does move the key, and is read
//! as a different type rather than as the same one under a new name, because nothing in this tree
//! can tell those two apart.
//!
//! **A record is OWED and this green does not claim it**: a dated note saying what
//! `schema_version` formally is. Until it lands the pinning test carries **no AC id** in its name
//! (`tests/it/budget.rs`'s `appetite_raise_from_an_activity_result_is_refused` is the precedent),
//! and an empty id is still **refused** rather than hashed
//! ([`MemoError::SchemaVersionNotSupplied`]) — `""` is a default wearing a string's clothes, and
//! typing the field does not make an empty one meaningful.
//!
//! # 🛑 A READING: COMPONENT 5 CARRIES A BRANCH TAG
//!
//! `StepKeyComponents::card_or_tool` is one `String` and AC-014 gives it two sources. This module
//! writes `card:<digest>` and `tool:<sha256_lp(version, command)>`. The tool branch **must** be
//! framed — *"version string plus the command"* is ambiguous about where one ends — and once one
//! branch is framed the other needs a tag, or a detected version of exactly 64 lowercase hex
//! characters is a card digest. No record spells the encoding; this is stated as a reading and a
//! record is owed with the two above.
//!
//! # AC-043 is NOT decided here, and `MemoDecision` has two variants for that reason
//!
//! Design §2.7 gives a third, `Evicted { key, reason }`, for AC-043's fail-closed rule — an entry
//! minted by a run whose replay assertion failed is refused and re-materialised
//! (`memo_evicted { reason: unsound_replay }`). AC-043's **third state** (replay-not-yet-asserted)
//! is ADR-0119 clause 7's *"dated amendment owed to the AC register"* and **a stage may not take
//! it** (`00-preamble.md`:55). No call site in this slice can produce an eviction, and
//! `src/workflow/envelope.rs`'s rule from Slice 4b applies: *a variant no call site can produce
//! reads as a refusal the system performs*. The variant lands with the record, not before;
//! `rk_store::memo::EvictionReason` already exists to carry it, and `EventKind::MemoEvicted` is
//! already one of the closed 37 *(29 until ADR-0123, 31 until ADR-0124, 36 until ADR-0125)*, so nothing is blocked by its absence here.
//!
//! # The fourth key is not here either
//!
//! `SelectionMemoKey` (AC-045) is ADR-0119's and lives in `rk-kernel::select`; the runtime owns
//! only the **lookup site** (clause 5: *"the caller at the runtime boundary computes the key … and
//! either serves the node's memoised result or calls `select()`"*). AC-045 stays **ARMED, NOT
//! DISCHARGED** — ADR-0119 clause 8: *"Until it lands, a stage claiming AC-045 reports it armed,
//! not discharged."*
//!
//! # 🛑 AC-018 IS RED AGAINST THE SHIPPED CATALOG BY DESIGN, AND THE REPAIRS ARE 7e's
//!
//! **Nineteen shipped steps across fourteen activities declare `memo = false`** [measured
//! 2026-09-14: `grep -rn 'memo' spikes/p9/catalog/ | wc -l` → 19; `-rln … | wc -l` → 14 —
//! `blue-green-deploy`, `failover` ×2, `feature-flag-kill` ×2, `flag-progressive-rollout`,
//! `incident-command`, `instrumented-trace-analysis`, `migration-dry-run`,
//! `migration-expand-execute`, `rate-limit-shed` ×2, `rollback`, `rollback-mitigation` ×2,
//! `rolling-deploy`, `rolling-restart` ×2, `slo-monitoring`]. ADR-0097 decision 3 makes every one
//! of them a pack-validation error. **The rule is therefore red against `spikes/p9/catalog/` and
//! that is not a defect in the rule** — §7b.12:970-977 forbids editing the shipped artefacts to
//! make a green, and the repairs are Step 7e's. `ac018_live_verb_may_not_memoise` asserts the exact
//! count, so the debt is pinned by a number rather than by a sentence, and the day 7e repairs it
//! that assertion goes red and says which number moved.
//!
//! # What is NOT here: anything that appends, dispatches or executes
//!
//! [`recorded_residue`] projects a decision sequence into the events the runtime **will** append at
//! Slice 4h. It appends nothing. AC-015's *"re-runs"* and AC-017's *"two executions"* are measured
//! against that projection and against [`MemoPassReport`]'s counts, and the real append site is
//! reported **ARMED** until 4h binds `rk-ports`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use rk_store::memo::StepKeyComponents;
use rk_types::digest::{Digest, sha256_lp};
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::SchemaId;

use crate::catalog::{Catalog, StepKind};
use crate::rigor::Rigor;
use crate::toml_subset::{self, TomlError, Value};

// ---- component 5 --------------------------------------------------------------------------------

/// Component 5's two branches, as a **closed set** rather than a `String` the caller might fill
/// either way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardOrTool {
    /// The resolved card's digest. The only branch any Slice-4 fixture takes (residual 5b).
    Card(Digest),
    /// 🛑 `Toolchain::detect()` returns `Result<bool, StepError>`, **not** a version string
    /// (`crates/rk-ports/src/toolchain.rs`:47). The version arrives here as a value the composition
    /// root supplies and its source is **OWED**.
    Tool {
        detected_version: String,
        command: String,
    },
}

impl CardOrTool {
    /// Component 5's string, branch-tagged and framed. See the header's reading.
    #[must_use]
    pub fn as_component(&self) -> String {
        match self {
            Self::Card(digest) => format!("card:{}", digest.as_str()),
            Self::Tool {
                detected_version,
                command,
            } => format!(
                "tool:{}",
                sha256_lp(&[detected_version.as_bytes(), command.as_bytes()]).as_str()
            ),
        }
    }
}

// ---- the eight runtime facts --------------------------------------------------------------------

/// The runtime facts the eight components are assembled FROM — **one field per component, in
/// ADR-0097's order**, so *"exactly eight, in this order"* stays a type fact one level up as well
/// as inside `rk-store`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepContext {
    /// 1. `activity_id@version` — `crate::catalog::Activity::id_at_version`'s output.
    pub activity_id_at_version: String,
    /// 2. The rigor variant selected, fed through `Rigor::as_str`.
    pub rigor: Rigor,
    /// 3. The step id within the activity graph.
    pub step_id: String,
    /// 4. Each resolved step input **in declaration order**, canonicalised here by
    ///    `rk_store::canon` and never by `serde_json::to_string`.
    pub resolved_inputs: Vec<serde_json::Value>,
    /// 5. The card digest, or the tool branch.
    pub card_or_tool: CardOrTool,
    /// 6. The **resolved** KB atoms. Sorting is `rk-store`'s, not this module's — it sorts in
    ///    `digest()` rather than trusting them to arrive sorted, and duplicating that here would be
    ///    a second place for the rule to drift.
    pub kb_atoms: Vec<Digest>,
    /// 7. The loop iteration counter, `0` outside a loop (ADR-0021).
    pub loop_iteration: u32,
    /// 8. The output type's `schema_version`, and **it is the whole `SchemaId`** —
    ///    [`rk_types::activity::ActivitySpec::schema`]'s id, unparsed. Typed rather than a
    ///    `String` so the only thing that can arrive here is a schema id: see the header for the
    ///    decision of 2026-09-14 and for the `vN`-parse it refused. **No default** — an empty id
    ///    is refused.
    pub output_schema: SchemaId,
}

impl StepContext {
    /// Build `rk-store`'s eight-field struct. **Assembly, not derivation.**
    ///
    /// # Errors
    ///
    /// [`MemoError::Canon`] when a resolved input carries a float, which `rk_store::canon` refuses
    /// rather than choosing a format for; [`MemoError::SchemaVersionNotSupplied`] for an empty
    /// component 8.
    pub fn components(&self) -> Result<StepKeyComponents, MemoError> {
        if self.output_schema.as_str().is_empty() {
            return Err(MemoError::SchemaVersionNotSupplied {
                step_id: self.step_id.clone(),
            });
        }
        let mut input_digests = Vec::with_capacity(self.resolved_inputs.len());
        for input in &self.resolved_inputs {
            let digest = rk_store::canon::canonical_digest(input).map_err(|e| match e {
                rk_store::canon::CanonError::FloatNotRepresentable { pointer } => {
                    MemoError::Canon { pointer }
                }
            })?;
            input_digests.push(digest);
        }
        Ok(StepKeyComponents {
            activity_id_at_version: self.activity_id_at_version.clone(),
            rigor: self.rigor.as_str().to_owned(),
            step_id: self.step_id.clone(),
            input_digests,
            card_or_tool: self.card_or_tool.as_component(),
            kb_atom_digests: self.kb_atoms.clone(),
            loop_iteration: self.loop_iteration,
            schema_version: self.output_schema.as_str().to_owned(),
        })
    }

    /// The step memo key. `rk-store`'s `digest()` over [`Self::components`] and nothing else.
    ///
    /// # Errors
    ///
    /// As [`Self::components`].
    pub fn key(&self) -> Result<Digest, MemoError> {
        Ok(self.components()?.digest())
    }
}

/// What stopped a memo assembly or a journal tally. House Shape A: named variants, stable tags, a
/// hand-written `Display` (this crate carries no `thiserror`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoError {
    /// A resolved step input carries a float. `rk_store::canon` refuses rather than choosing a
    /// format, and so does this.
    Canon { pointer: String },
    /// Component 8 arrived empty — the spec's [`rk_types::typed::SchemaId`] carries no id. `""` is
    /// a code default wearing a string's clothes.
    ///
    /// **The variant keeps AC-014's word for component 8** rather than the field it is now sourced
    /// from, because `schema_version` is the name both the criterion and
    /// `rk_store::memo::StepKeyComponents` use, and neither moves. What `schema_version` formally
    /// **is** remains a record OWED — see the header.
    SchemaVersionNotSupplied { step_id: String },
    /// A journal event's payload is not the shape [`tally_activations`] reads.
    MalformedPayload { kind: &'static str, detail: String },
}

impl MemoError {
    /// The stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Canon { .. } => "canon",
            Self::SchemaVersionNotSupplied { .. } => "schema_version_not_supplied",
            Self::MalformedPayload { .. } => "malformed_payload",
        }
    }
}

impl fmt::Display for MemoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canon { pointer } => write!(
                f,
                "a resolved step input carries a float at `{pointer}`; memo-key component 4 is \
                 canonicalised by rk-store's serialiser, which refuses floats rather than choosing \
                 a format"
            ),
            Self::SchemaVersionNotSupplied { step_id } => write!(
                f,
                "step `{step_id}` reached memo-key component 8 with an empty schema id. \
                 Component 8 is the activity spec's whole schema id and there is no code \
                 default: an empty component 8 would make the key stand still across a schema \
                 change, which inverts AC-014's THEN"
            ),
            Self::MalformedPayload { kind, detail } => {
                write!(f, "a `{kind}` payload is not readable: {detail}")
            }
        }
    }
}

impl std::error::Error for MemoError {}

// ---- the decision, the ledger and the pass ------------------------------------------------------

/// What the memo said about one step activation. **Two variants** — see the header on AC-043.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoDecision {
    /// The key was already in the ledger: the step is short-circuited, and AC-019's THEN is that
    /// the skip is never silent.
    Hit {
        key: Digest,
        components: StepKeyComponents,
    },
    /// The key was not in the ledger: the step executes.
    Miss {
        key: Digest,
        components: StepKeyComponents,
    },
}

impl MemoDecision {
    #[must_use]
    pub const fn key(&self) -> &Digest {
        match self {
            Self::Hit { key, .. } | Self::Miss { key, .. } => key,
        }
    }

    #[must_use]
    pub const fn is_hit(&self) -> bool {
        matches!(self, Self::Hit { .. })
    }

    /// The stable tag, and design §5's own wire names for the two kinds.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Hit { .. } => "memo_hit",
            Self::Miss { .. } => "memo_miss",
        }
    }
}

/// The set of step keys the memo holds. A `BTreeSet`, never a `HashSet` — the parent module's
/// `#![deny(clippy::disallowed_types)]` reaches this child, and an unordered set would make a
/// replay's iteration order an input (AC-004).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MemoLedger {
    entries: BTreeSet<Digest>,
}

impl MemoLedger {
    /// An empty memo. A real state, not an absence: the first activation of every run meets it.
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn holds(&self, key: &Digest) -> bool {
        self.entries.contains(key)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// House Shape C — an **observation**, never a verdict. It exists so *"the memo did not hit"* is
/// distinguishable from *"nothing was consulted"*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MemoPassReport {
    pub consulted: usize,
    pub hits: usize,
    pub misses: usize,
    /// Distinct keys the pass saw. Equals `consulted` only when no activation repeats, which is
    /// what makes AC-017's *"two iterations, two keys"* a countable claim rather than a hopeful one.
    pub distinct_keys: usize,
}

/// Consult the memo for one activation.
///
/// # Errors
///
/// As [`StepContext::components`].
pub fn consult(ledger: &MemoLedger, ctx: &StepContext) -> Result<MemoDecision, MemoError> {
    let components = ctx.components()?;
    let key = components.digest();
    if ledger.holds(&key) {
        Ok(MemoDecision::Hit { key, components })
    } else {
        Ok(MemoDecision::Miss { key, components })
    }
}

/// Fold a sequence of activations through the memo. Each `Miss` enters its key, so a repeat of the
/// same activation later in the same pass hits.
///
/// # Errors
///
/// As [`StepContext::components`], on the first activation that fails.
pub fn fold(
    ledger: MemoLedger,
    ctxs: &[StepContext],
) -> Result<(MemoLedger, Vec<MemoDecision>, MemoPassReport), MemoError> {
    let mut ledger = ledger;
    let mut decisions = Vec::with_capacity(ctxs.len());
    let mut report = MemoPassReport::default();
    let mut seen: BTreeSet<Digest> = BTreeSet::new();
    for ctx in ctxs {
        let decision = consult(&ledger, ctx)?;
        report.consulted += 1;
        seen.insert(decision.key().clone());
        if decision.is_hit() {
            report.hits += 1;
        } else {
            report.misses += 1;
            // A miss executes, and an execution enters the memo — which is what makes a repeat of
            // the SAME activation later in the pass hit, and what makes AC-017's control bite.
            ledger.entries.insert(decision.key().clone());
        }
        decisions.push(decision);
    }
    report.distinct_keys = seen.len();
    Ok((ledger, decisions, report))
}

// ---- AC-019's payload, and the projection AC-015/AC-017 count against ---------------------------

/// The nine top-level members of a `memo_hit` payload: `key` plus AC-014's eight, at
/// `StepKeyComponents`' own field names, **in byte order** so the array can be compared against a
/// sorted key set directly.
///
/// The array exists so *"exactly eight components and no ninth"* is asserted against a named list
/// rather than against nine hand-written string literals in a test.
pub const MEMO_HIT_PAYLOAD_KEYS: [&str; 9] = [
    "activity_id_at_version",
    "card_or_tool",
    "input_digests",
    "kb_atom_digests",
    "key",
    "loop_iteration",
    "rigor",
    "schema_version",
    "step_id",
];

/// **AC-019's payload**: *"a `memo_hit` event carrying the key and its **eight** components (exactly
/// AC-014's list, ADR-0097) is appended; a skip is never silent."*
///
/// 🛑 **The register's first instrument is *"an event-schema test asserting the field set"*, and
/// there is no schema.** `crates/rk-types/src/event.rs`:46-59 records the per-kind payload shape as
/// *"provisionally and visibly"* untyped and *"The typed-payload record, per kind, is **OWED**; its
/// earliest real consumer is the SQLite adapter at 7c"*. So the assertion over this value is over
/// **JSON keys** and nothing stronger, and that half is reported **ARMED** (residual 10j).
/// Inventing a payload schema in `rk-kernel` would pre-empt the record 7c owes.
#[must_use]
pub fn memo_hit_payload(key: &Digest, c: &StepKeyComponents) -> serde_json::Value {
    let hexes =
        |ds: &[Digest]| -> Vec<String> { ds.iter().map(|d| d.as_str().to_owned()).collect() };
    // Component 6 **is** *"the digests of the resolved KB atoms, SORTED"*, and `digest()` sorts them
    // before hashing. The payload names the component and not the caller's resolution order, so two
    // hits on the same key cannot carry two different payloads.
    let mut atoms = c.kb_atom_digests.clone();
    atoms.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
    serde_json::json!({
        "key": key.as_str(),
        "activity_id_at_version": c.activity_id_at_version,
        "rigor": c.rigor,
        "step_id": c.step_id,
        "input_digests": hexes(&c.input_digests),
        "card_or_tool": c.card_or_tool,
        "kb_atom_digests": hexes(&atoms),
        "loop_iteration": c.loop_iteration,
        "schema_version": c.schema_version,
    })
}

/// The events a memo pass leaves in the journal — a `memo_hit` for each hit, and a `memo_miss`
/// followed by the `dispatch` it authorises for each miss.
///
/// 🛑 **This is a PROJECTION and not an append.** Nothing in `rk-kernel::workflow` may append;
/// Slice 4h's runtime boundary binds `rk-ports` and writes these. The projection exists because
/// AC-015's THEN is *"a `memo_miss` event **followed by** a re-execution"* and AC-019's second
/// instrument is a count over a recorded journal — neither is assertable against a decision list
/// alone, and a journal hand-written in a test asserts only what the test author typed. The
/// `dispatch` payload carried here is the **step's** half of it; the full cost row and the
/// `IdempotencyKey` (which needs an `item`, and an item is the runtime's) are Slice 4h's, and that
/// half is reported ARMED.
#[must_use]
pub fn recorded_residue(decisions: &[MemoDecision]) -> Vec<Event> {
    let mut out = Vec::with_capacity(decisions.len());
    for decision in decisions {
        let payload = memo_hit_payload(
            decision.key(),
            match decision {
                MemoDecision::Hit { components, .. } | MemoDecision::Miss { components, .. } => {
                    components
                }
            },
        );
        match decision {
            MemoDecision::Hit { .. } => {
                out.push(Event::new(EventKind::MemoHit, payload));
            }
            MemoDecision::Miss { components, .. } => {
                out.push(Event::new(EventKind::MemoMiss, payload));
                out.push(Event::new(
                    EventKind::Dispatch,
                    serde_json::json!({
                        "activity_id_at_version": components.activity_id_at_version,
                        "step_id": components.step_id,
                        "attempt": 1,
                    }),
                ));
            }
        }
    }
    out
}

// ---- AC-019's identity --------------------------------------------------------------------------

/// House Shape C again, over a **recorded journal**: the three quantities AC-019's identity relates,
/// plus the two counts that make the identity falsifiable rather than true by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ActivationTally {
    /// Every event the fold looked at, so an empty journal is distinguishable from a fold that
    /// never ran.
    pub events_folded: usize,
    /// One per `memo_hit` or `memo_miss`: **every activation consults the memo exactly once**, and
    /// that is what makes this a count of activations and not of steps.
    pub activations: usize,
    pub memo_hits: usize,
    pub memo_misses: usize,
    /// Activations that were a **miss** and were followed by at least one `dispatch`. Distinct
    /// from [`Self::dispatch_events`]: a retried activation dispatches twice and executes once.
    pub executed_steps: usize,
    /// Raw `dispatch` rows.
    pub dispatch_events: usize,
    /// A `dispatch` before any memo decision, or after a `memo_hit`. **Must be zero**: a hit that
    /// dispatched is a memo that short-circuited nothing, and a dispatch before any decision is an
    /// execution the memo never saw.
    pub unattributed_dispatches: usize,
}

impl ActivationTally {
    /// AC-019's identity: *"executed steps plus `memo_hit` events equals the number of **step
    /// activations** in the recorded run"*.
    #[must_use]
    pub const fn identity_holds(&self) -> bool {
        self.executed_steps + self.memo_hits == self.activations
            && self.unattributed_dispatches == 0
    }
}

/// Fold a recorded journal into [`ActivationTally`].
///
/// The match over `EventKind` is **exhaustive with no catch-all arm**: the set is closed at 29
/// (ADR-0116), and a thirtieth kind must be routed by whoever mints it rather than silently
/// ignored here.
///
/// # Errors
///
/// [`MemoError::MalformedPayload`] for a `memo_hit` or `memo_miss` that carries no `key`.
pub fn tally_activations(events: &[Event]) -> Result<ActivationTally, MemoError> {
    let mut t = ActivationTally::default();
    // The activation currently in scope: `Some(true)` after a hit, `Some(false)` after a miss that
    // has not yet dispatched, `None` before the first decision. A journal's `dispatch` rows belong
    // to the decision that precedes them, which is what makes a RETRY (two rows, one execution)
    // and a SILENT EXECUTION (a row with no decision before it) different readings.
    let mut open: Option<bool> = None;
    let mut dispatched_for_open = false;
    for event in events {
        t.events_folded += 1;
        match event.kind {
            EventKind::MemoHit | EventKind::MemoMiss => {
                let tag = if event.kind == EventKind::MemoHit {
                    "memo_hit"
                } else {
                    "memo_miss"
                };
                // A decision with no key names no activation, and counting it would make the
                // identity true over a journal a replay cannot read.
                match event.payload.get("key").and_then(serde_json::Value::as_str) {
                    Some(k) if !k.is_empty() => {}
                    _ => {
                        return Err(MemoError::MalformedPayload {
                            kind: tag,
                            detail: "carries no non-empty `key`, so it names no activation"
                                .to_owned(),
                        });
                    }
                }
                t.activations += 1;
                if event.kind == EventKind::MemoHit {
                    t.memo_hits += 1;
                    open = Some(true);
                } else {
                    t.memo_misses += 1;
                    open = Some(false);
                }
                dispatched_for_open = false;
            }
            EventKind::Dispatch => {
                t.dispatch_events += 1;
                match open {
                    Some(false) => {
                        if !dispatched_for_open {
                            t.executed_steps += 1;
                            dispatched_for_open = true;
                        }
                    }
                    Some(true) | None => t.unattributed_dispatches += 1,
                }
            }
            // The other 34 of the closed 37. Grouped rather than wildcarded: a thirty-eighth kind is
            // a compile error here and must be routed by whoever mints it (ADR-0116).
            EventKind::StageTransition
            | EventKind::GateOpened
            | EventKind::GateAnswered
            | EventKind::GateExpired
            | EventKind::SliceSealed
            | EventKind::ReconcileStarted
            | EventKind::GateAnswerVoid
            | EventKind::GateAnswerSpent
            | EventKind::GateResumed
            | EventKind::ReviewBlock
            | EventKind::QuarantineGranted
            | EventKind::QuarantineRemoved
            | EventKind::QuarantineExpired
            | EventKind::SelectionIneffective
            | EventKind::StageExitRefused
            | EventKind::DagAssembled
            | EventKind::DagAmended
            | EventKind::ActivityCommitted
            | EventKind::MemoEvicted
            | EventKind::StepFailed
            | EventKind::StepCancelled
            | EventKind::StepRefused
            | EventKind::RateLimited
            | EventKind::RateLimitReading
            | EventKind::Degraded
            | EventKind::StructuredDrift
            | EventKind::DriftAdvisory
            | EventKind::ReviewClassDrift
            | EventKind::Rewrite
            | EventKind::ArtifactRead
            | EventKind::ArtifactWrite
            | EventKind::AdvisoryCreated
            | EventKind::AdvisoryRetired
            | EventKind::AdvisoryDismissed => {}
        }
    }
    Ok(t)
}

// ---- AC-016's lint ------------------------------------------------------------------------------

/// Why an activity-level key was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoLintError {
    /// *"a lint that rejects an activity-level key computed from a single card reference"* —
    /// AC-016's second instrument.
    ///
    /// 🛑 **A READING: `got == 0` is refused too.** The criterion names the single-reference case;
    /// it says nothing about zero. An activity-level key over no cards at all is the digest of an
    /// empty sequence — a constant every empty subgraph would share — and it is the same defect one
    /// step further along, so the rule is `got < 2`. The `got` field is design §2.7's own, which
    /// already anticipates more than one refused value. No record settles it.
    SingleCardReference { got: usize },
}

impl MemoLintError {
    /// The stable tag, never message text.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::SingleCardReference { .. } => "single_card_reference",
        }
    }
}

impl fmt::Display for MemoLintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleCardReference { got } => write!(
                f,
                "an activity-level key was computed from {got} card reference(s). AC-016's key is \
                 SHA-256 over *every* card reachable in the subgraph; a key from one card is that \
                 card's own digest wearing the activity's name"
            ),
        }
    }
}

/// **AC-016's lint.** `rk_store::memo::activity_key` accepts a one-element slice happily; this is
/// the rule that does not.
///
/// # Errors
///
/// [`MemoLintError::SingleCardReference`] for fewer than two reachable card digests.
pub fn activity_key_rule(reachable_card_digests: &[Digest]) -> Result<Digest, MemoLintError> {
    let got = reachable_card_digests.len();
    if got < 2 {
        return Err(MemoLintError::SingleCardReference { got });
    }
    Ok(rk_store::memo::activity_key(reachable_card_digests))
}

// ---- AC-018's verb registry and rule ------------------------------------------------------------

/// One `[verb]` entry of `spikes/p9/registry.toml`'s shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbEntry {
    pub outputs: Option<String>,
    /// `live = true` ⇒ the verb's results are never memoised. The registry's own header says so:
    /// *"`live` (⇒ memo = false on every step using it)"*.
    pub live: bool,
}

/// The verb half of the tool registry. **Only the `[verb]` table is read** — `[toolchain.feeds]`,
/// the timeouts and the checklists belong to rules this module does not carry.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VerbRegistry {
    verbs: BTreeMap<String, VerbEntry>,
}

/// What stopped a verb-registry read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerbRegistryError {
    Toml {
        error: TomlError,
    },
    /// No `[verb]` table at all. **An empty registry is an error, not an empty success**: a rule
    /// that finds no live verb passes, which is Halt 8's *"a filter that matches nothing exits 0"*.
    NoVerbTable,
    /// An entry that does not declare `live`, or declares it as something other than a boolean.
    ///
    /// Refused at **load**, and that is fail-closed rather than strict: if liveness is unknown the
    /// memo rule cannot say whether memoising is lawful, and the alternative — defaulting to
    /// `false` — silently permits it.
    Shape {
        verb: String,
        detail: String,
    },
}

impl VerbRegistryError {
    /// The stable tag, never message text.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Toml { .. } => "toml",
            Self::NoVerbTable => "no_verb_table",
            Self::Shape { .. } => "shape",
        }
    }
}

impl fmt::Display for VerbRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml { error } => write!(f, "{error}"),
            Self::NoVerbTable => f.write_str(
                "the registry declares no [verb] table. An empty verb registry is an error: a memo \
                 rule that finds no live verb reports green over a registry it could not read",
            ),
            Self::Shape { verb, detail } => write!(f, "verb `{verb}`: {detail}"),
        }
    }
}

impl std::error::Error for VerbRegistryError {}

impl VerbRegistry {
    /// Read `[verb]` from a registry document.
    ///
    /// # Errors
    ///
    /// [`VerbRegistryError`] for a document outside the TOML subset, a missing `[verb]` table, or
    /// an entry without a boolean `live`.
    pub fn from_toml(src: &str) -> Result<Self, VerbRegistryError> {
        let doc = toml_subset::parse(src).map_err(|error| VerbRegistryError::Toml { error })?;
        let table = doc
            .get("verb")
            .and_then(Value::as_table)
            .ok_or(VerbRegistryError::NoVerbTable)?;
        if table.is_empty() {
            return Err(VerbRegistryError::NoVerbTable);
        }
        let mut verbs = BTreeMap::new();
        for (name, value) in table {
            let entry = value.as_table().ok_or_else(|| VerbRegistryError::Shape {
                verb: name.clone(),
                detail: "is not a table".to_owned(),
            })?;
            // `live` is REFUSED rather than defaulted: see the variant's own doc. A missing flag
            // defaulted to `false` would report every step using this verb as lawfully memoisable.
            let live = entry.get("live").and_then(Value::as_bool).ok_or_else(|| {
                VerbRegistryError::Shape {
                    verb: name.clone(),
                    detail: "declares no boolean `live`. Liveness is the verb registry's flag and \
                             nothing else (ADR-0059); an absent one is refused rather than \
                             defaulted, because either default decides a value the registry owes"
                        .to_owned(),
                }
            })?;
            verbs.insert(
                name.clone(),
                VerbEntry {
                    outputs: entry
                        .get("outputs")
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    live,
                },
            );
        }
        Ok(Self { verbs })
    }

    /// `Some(true)` for a live verb, `Some(false)` for a declared non-live one, `None` for a verb
    /// this registry does not carry. **The three are kept apart**: an unknown verb is not a
    /// non-live one.
    #[must_use]
    pub fn live(&self, verb: &str) -> Option<bool> {
        self.verbs.get(verb).map(|e| e.live)
    }

    /// The entry, for a caller that needs its `outputs` as well as its liveness.
    #[must_use]
    pub fn entry(&self, verb: &str) -> Option<&VerbEntry> {
        self.verbs.get(verb)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.verbs.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.verbs.is_empty()
    }
}

/// One violation of AC-018. Collected, **never short-circuited** (house Shape B).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoViolation {
    /// *"GIVEN a `tool` step whose verb's registry entry carries `live = true` WHEN the step is
    /// memoised, or declares `memo = true`"*.
    LiveVerbMemoised {
        activity: String,
        step: String,
        verb: String,
    },
    /// ADR-0097 decision 3: *"there is no per-step memo field"*. `memo = false` and `memo = true`
    /// are both this.
    PerStepMemoDeclaration { activity: String, step: String },
}

impl MemoViolation {
    /// The stable tag, never message text.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::LiveVerbMemoised { .. } => "live_verb_memoised",
            Self::PerStepMemoDeclaration { .. } => "per_step_memo_declaration",
        }
    }

    /// The activity the violation is in, so a caller can count activities as well as violations.
    #[must_use]
    pub fn activity(&self) -> &str {
        match self {
            Self::LiveVerbMemoised { activity, .. }
            | Self::PerStepMemoDeclaration { activity, .. } => activity,
        }
    }
}

impl fmt::Display for MemoViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LiveVerbMemoised {
                activity,
                step,
                verb,
            } => write!(
                f,
                "{activity}/{step}: the verb `{verb}` is `live = true` in the registry, and this \
                 step declares `memo = true`. Liveness is the verb registry's flag and nothing \
                 else (ADR-0059)"
            ),
            Self::PerStepMemoDeclaration { activity, step } => write!(
                f,
                "{activity}/{step}: declares a step-level `memo` key. ADR-0097 decision 3 withdrew \
                 it — there is no per-step memo field"
            ),
        }
    }
}

/// The verb token a `tool` step's `run` names, or `None` for a step that names none.
///
/// `run = "toolchain:<verb>"`, with the optional `?` suffix the feed spelling uses. A token this
/// registry does not carry is **not** a violation of AC-018: refusing a `run` that names nothing in
/// the registry is AC-072's sibling rule and is not built (`crate::registry`'s header records the
/// same scoping), and folding it in here would report one criterion's failure under another's name.
#[must_use]
pub fn verb_of(run: &str) -> Option<&str> {
    run.strip_prefix("toolchain:")
        .map(|v| v.trim_end_matches('?'))
        .filter(|v| !v.is_empty())
}

/// **AC-018**, over a verb registry and a catalog. Violations are **collected in catalog order**
/// and never short-circuited: a rule that stopped at the first violation would report one repair
/// when nineteen are owed.
///
/// Both clauses are reported for one step when both are true, and that is a stated divergence from
/// design §4's `Err([LiveVerbMemoised])`: a live verb declaring `memo = true` breaks the liveness
/// rule **and** carries the field ADR-0097 decision 3 withdrew, and suppressing the second would
/// make the shipped catalog's violation count depend on which clause fired first.
///
/// # Errors
///
/// A non-empty `Vec<MemoViolation>`, in activity-then-step order.
pub fn live_verb_memo_rule(
    catalog: &Catalog,
    verbs: &VerbRegistry,
) -> Result<(), Vec<MemoViolation>> {
    let mut violations = Vec::new();
    for activity in catalog.activities() {
        for step in &activity.steps {
            // Clause 1, the more specific: AC-018's GIVEN is a `tool` step on a live verb, and its
            // WHEN is *"the step is memoised, or declares `memo = true`"*. Under ADR-0059 the
            // registry's flag is the only carrier of liveness, so a live verb's step is already
            // not memoised and only an explicit `memo = true` re-asserts it.
            let memoised_live_verb =
                if step.kind == StepKind::Tool && step.memo_declared == Some(true) {
                    step.run
                        .as_deref()
                        .and_then(verb_of)
                        .filter(|verb| verbs.live(verb) == Some(true))
                } else {
                    None
                };
            if let Some(verb) = memoised_live_verb {
                violations.push(MemoViolation::LiveVerbMemoised {
                    activity: activity.id.clone(),
                    step: step.id.clone(),
                    verb: verb.to_owned(),
                });
            }
            // Clause 2, ADR-0097 decision 3: *"there is no per-step memo field"*. `false` and
            // `true` are both the withdrawn declaration, and the clause is independent of the verb.
            if step.memo_declared.is_some() {
                violations.push(MemoViolation::PerStepMemoDeclaration {
                    activity: activity.id.clone(),
                    step: step.id.clone(),
                });
            }
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
