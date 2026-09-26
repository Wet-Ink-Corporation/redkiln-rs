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
//! already one of the closed 38 *(29 until ADR-0123, 31 until ADR-0124, 36 until ADR-0125, 37 until ADR-0126)*, so nothing is blocked by its absence here.
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
//! *(dated 2026-09-25, s8b, D21-43 N26; noted at s8b refuting round 2)* **No longer red.** The
//! repair was `s8b`'s, not Step 7e's: the nineteen lines are deleted from `.redkiln/catalog`, and
//! `ac018_live_verb_may_not_memoise` now asserts **zero** violations there. The nineteen live on
//! only in the frozen p9 control, `ac018_frozen_p9_catalog_is_red`, read at `P9_PIN`.
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
            // The other 35 of the closed 38 (34 of 37 until ADR-0126, 2026-09-17). Grouped rather
            // than wildcarded: a thirty-ninth kind is a compile error here and must be routed by
            // whoever mints it (ADR-0116). `dispatch_completed` is routed HERE and not to the
            // `Dispatch` arm above, deliberately: it is the adapter's completion row for a
            // dispatch the kernel's pre-dispatch `dispatch` row already counted (ADR-0126
            // decision 3 — two kinds, two producers, and *"nothing counts them as one"*), so
            // counting it as a dispatch would double every activation in AC-019's tally; and the
            // memo's typed-payload record stays OWED (decision 6), unchanged by the new kind.
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
            | EventKind::DispatchCompleted
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
//
// *(Extended 2026-09-26, s8c §8c.4-§8c.6, records V and B = ADR-0170/0171, proposed.)* ONE parser,
// not two (§8c.4): the reader AC-018 already used is the reader `run_tool_step`, `rk-cli` and
// `doctor` load `.redkiln/registry.toml` through. It now reads, besides `[verb]`, the three source
// tables (`[checklist.*]`, `[kb.*]`, `[feed.*.*]`, D21-43 N14) and the per-toolchain value tables
// (`[toolchain.<name>]`, D21-42 4a), and every row carries the five columns record V decision 1
// fixes. A column that is missing or malformed is refused at load, naming the verb or the entry;
// nothing is defaulted.

/// The closed route set (record V decision 1; ADR-0135 clause 1's idiom).
///
/// **Matched exhaustively, with no wildcard, wherever it is read**: a fifth route is a record's act (`s8d`'s, for
/// `verdict_record`), and it becomes `E0004` at every match rather than a silent default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    /// One of the four existing port methods. The verb's name picks the method, so a `port` row
    /// on any other verb is refused at load.
    Port(PortMethod),
    /// The seventh port method, `Toolchain::run_verb` (record V decision 2).
    Toolchain,
    /// An rk-store write. No executor at `s8c`: `s8f` builds it.
    Store,
    /// Through the journalled `GitOps` seam. No executor at `s8c`: `s8g` builds it.
    Git,
}

impl Route {
    /// The route's spelling in `registry.toml`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Port(_) => "port",
            Self::Toolchain => "toolchain",
            Self::Store => "store",
            Self::Git => "git",
        }
    }
}

/// The four port methods a `port` row reaches (ADR-0135's map, now the registry's `port` rows).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortMethod {
    Build,
    Test,
    Lint,
    ChangeSummary,
}

impl PortMethod {
    /// The method a `port` row's verb names, or `None` for a verb that names none.
    #[must_use]
    pub fn from_verb(verb: &str) -> Option<Self> {
        match verb {
            "build" => Some(Self::Build),
            "test" => Some(Self::Test),
            "lint" => Some(Self::Lint),
            "change_summary" => Some(Self::ChangeSummary),
            _ => None,
        }
    }

    /// The `outputs` kind the row must declare: the spike's own (`spikes/p9/registry.toml`).
    #[must_use]
    pub const fn output_kind(self) -> &'static str {
        match self {
            Self::Build => "BuildReport",
            Self::Test => "TestReport",
            Self::Lint => "LintReport",
            Self::ChangeSummary => "ChangeSummary",
        }
    }
}

/// One `[verb]` row (record V decision 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbEntry {
    /// The typed output kind. *(Was `Option<String>` until 2026-09-26: a row without one is now
    /// refused at load, because the kernel's decode table reads it.)*
    pub outputs: String,
    /// `live = true` ⇒ the verb's results are never memoised. The registry's own header says so:
    /// *"`live` (⇒ memo = false on every step using it)"*.
    pub live: bool,
    /// The row's `${toolchain.*}` timeout key, spelled bare as the spike spells it
    /// (`timeout = "test_timeout"`). A catalog step's own `timeout_secs` is its bound (record B);
    /// this is the row's, for a call no catalog step makes (`setup`, `s8d-ii`).
    pub timeout: String,
    /// The input names the call carries, in declaration order.
    pub inputs: Vec<String>,
    /// The route, from the closed set.
    pub route: Route,
}

/// A `[checklist.<id>]` or `[kb.<atom>]` entry: exactly one of two forms (record V decision 8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceEntry {
    /// `path`, relative to `.redkiln/`, checked at load to exist and hold an item.
    Path(String),
    /// `unauthored = "<reason, naming the owner>"`.
    Unauthored(String),
}

/// A `[feed.<toolchain>.<name>]` entry: exactly one of two forms (record V decision 8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeedEntry {
    /// `verb = "<a registry verb>"`, the producing verb, checked at load to be a row.
    Verb(String),
    /// `unprovided = "<reason>"`.
    Unprovided(String),
}

/// One `${toolchain.<key>}` value as the kernel reads it: seconds, or a declared absence.
///
/// Record V decision 9. **Flat and adapter-name-free**: the warrant stays in the file, which the
/// loader has already checked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolchainValue {
    /// A value that carried its warrant at load.
    Secs(std::num::NonZeroU64),
    /// `unmeasured = "<reason>"`.
    Unmeasured { reason: String },
}

/// **The value table `rk-cli` hands the kernel** (§8c.5, AC-487).
///
/// Key → seconds, or key → `unmeasured`. It carries no toolchain or adapter name, so the kernel resolves a key without
/// ever seeing which adapter the table is for.
///
/// `Default` is the EMPTY table: every key is absent, and resolution refuses each one by name. It
/// is never a fallback to a bound (record B decision 2).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ToolchainValues {
    entries: BTreeMap<String, ToolchainValue>,
    /// Set when no table could be selected (no adapter detected, or no registry): every key then
    /// refuses naming this reason rather than as an absent key.
    unselected: Option<String>,
}

/// Why a `${toolchain.<key>}` did not resolve (record B decision 2). **None is a fallback.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueRefusal {
    /// The table declares no such key (`Unresolved`).
    Absent { key: String },
    /// The table declares the key `unmeasured`.
    Unmeasured { key: String, reason: String },
    /// No table was selected, for the reason given.
    Unselected { key: String, reason: String },
}

impl ValueRefusal {
    /// The key, spelled as the catalog spells it: `${toolchain.<key>}`.
    #[must_use]
    pub fn placeholder(&self) -> String {
        let key = match self {
            Self::Absent { key } | Self::Unmeasured { key, .. } | Self::Unselected { key, .. } => {
                key
            }
        };
        format!("${{toolchain.{key}}}")
    }
}

impl fmt::Display for ValueRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let placeholder = self.placeholder();
        match self {
            Self::Absent { .. } => write!(
                f,
                "{placeholder} is absent from the value table (Unresolved): no value and no \
                 declared `unmeasured`, and it never falls back to the construction bound \
                 (ADR-0171 decision 2)"
            ),
            Self::Unmeasured { reason, .. } => write!(
                f,
                "{placeholder} is declared `unmeasured`: {reason}. An unmeasured key refuses by \
                 name and never falls back to the construction bound (ADR-0171 decision 2)"
            ),
            Self::Unselected { reason, .. } => write!(
                f,
                "{placeholder} has no value table to resolve through: {reason}. No key is \
                 guessed (ADR-0170 decision 12)"
            ),
        }
    }
}

impl ToolchainValues {
    /// A table over the given entries.
    #[must_use]
    pub const fn new(entries: BTreeMap<String, ToolchainValue>) -> Self {
        Self {
            entries,
            unselected: None,
        }
    }

    /// No table: every key refuses naming `reason`.
    #[must_use]
    pub fn unselected(reason: &str) -> Self {
        Self {
            entries: BTreeMap::new(),
            unselected: Some(reason.to_owned()),
        }
    }

    /// Resolve one key.
    ///
    /// # Errors
    /// [`ValueRefusal`] for an absent or `unmeasured` key, or when no table was selected.
    pub fn resolve(&self, key: &str) -> Result<std::num::NonZeroU64, ValueRefusal> {
        if let Some(reason) = &self.unselected {
            return Err(ValueRefusal::Unselected {
                key: key.to_owned(),
                reason: reason.clone(),
            });
        }
        match self.entries.get(key) {
            Some(ToolchainValue::Secs(secs)) => Ok(*secs),
            Some(ToolchainValue::Unmeasured { reason }) => Err(ValueRefusal::Unmeasured {
                key: key.to_owned(),
                reason: reason.clone(),
            }),
            None => Err(ValueRefusal::Absent {
                key: key.to_owned(),
            }),
        }
    }

    /// The entries, for a caller that reports them.
    #[must_use]
    pub const fn entries(&self) -> &BTreeMap<String, ToolchainValue> {
        &self.entries
    }
}

/// A declaration row in a per-toolchain table: a fact about the toolchain that is not a duration
/// (§8c.5), `{ value = <any>, source = "<string>" }`, admitted from its named set only.
#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub value: Value,
    pub source: String,
}

/// The declaration rows a per-toolchain table admits (§8c.5; Checkpoint 8c (11)'s `D`).
pub const DECLARATION_ROWS: [&str; 3] = ["setup_projects", "package_binaries", "claims"];

/// One `[toolchain.<name>]` table, as loaded.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValueTable {
    values: BTreeMap<String, ToolchainValue>,
    declarations: BTreeMap<String, Declaration>,
}

impl ValueTable {
    /// The declaration rows.
    #[must_use]
    pub const fn declarations(&self) -> &BTreeMap<String, Declaration> {
        &self.declarations
    }
}

/// What a source input resolved to (record V decision 8's resolution rules; the resolver that
/// calls it is `s8d`'s).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceValue {
    /// A declared `path`, relative to `.redkiln/`.
    Path(String),
    /// A declared feed's producing verb.
    FeedVerb(String),
    /// An optional (`?`) input whose key is declared `unauthored` or `unprovided`: `null`
    /// (ADR-0032's Decision: *"Absent `?` inputs are `null`"*).
    Null,
}

/// Why a source input did not resolve. **Each names the key.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceRefusal {
    /// The input is not `checklist:`, `kb:` or `toolchain:<feed>`.
    NotASource { input: String },
    /// No entry declares the key. Refused **even when the input is optional**: an undeclared key is
    /// a typo or an unowned demand, never an absence (N14).
    Undeclared { key: String },
    /// A mandatory input whose key is declared `unauthored`.
    Unauthored { key: String, reason: String },
    /// A mandatory input whose feed is declared `unprovided`.
    Unprovided { key: String, reason: String },
}

impl fmt::Display for SourceRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotASource { input } => write!(
                f,
                "`{input}` is not a `checklist:`, `kb:` or `toolchain:<feed>` source"
            ),
            Self::Undeclared { key } => write!(
                f,
                "`{key}` is not declared in registry.toml. An undeclared source is refused by \
                 name even when the input is optional (D21-43 N14; ADR-0170 decision 8)"
            ),
            Self::Unauthored { key, reason } => {
                write!(
                    f,
                    "`{key}` is mandatory and declared `unauthored`: {reason}"
                )
            }
            Self::Unprovided { key, reason } => {
                write!(
                    f,
                    "`{key}` is mandatory and declared `unprovided`: {reason}"
                )
            }
        }
    }
}

impl std::error::Error for SourceRefusal {}

/// The tool registry: the verb rows, the source tables and the per-toolchain value tables.
///
/// *(The doc read "the verb half of the tool registry. **Only the `[verb]` table is read**" until
/// 2026-09-26; ADR-0170, proposed, extends this reader to the whole file.)*
#[derive(Debug, Clone, PartialEq, Default)]
pub struct VerbRegistry {
    verbs: BTreeMap<String, VerbEntry>,
    checklists: BTreeMap<String, SourceEntry>,
    kbs: BTreeMap<String, SourceEntry>,
    /// `(toolchain, name)` → entry.
    feeds: BTreeMap<(String, String), FeedEntry>,
    toolchains: BTreeMap<String, ValueTable>,
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
    /// An entry that does not declare `live`, or declares it as something other than a boolean;
    /// *(2026-09-26)* or any other column of the row missing or malformed.
    ///
    /// Refused at **load**, and that is fail-closed rather than strict: if liveness is unknown the
    /// memo rule cannot say whether memoising is lawful, and the alternative — defaulting to
    /// `false` — silently permits it.
    Shape {
        verb: String,
        detail: String,
    },
    /// *(2026-09-26.)* A source-table entry that is malformed: both forms or neither, a `path`
    /// that does not exist or holds no item, a feed `verb` that is not a row. Names the entry.
    Source {
        entry: String,
        detail: String,
    },
    /// *(2026-09-26.)* A per-toolchain value without its warrant, or otherwise malformed. Names
    /// the table and the key.
    Value {
        entry: String,
        detail: String,
    },
    /// *(2026-09-26.)* A top-level table this reader does not know.
    UnknownTable {
        name: String,
    },
    /// *(2026-09-26.)* The file could not be read.
    Io {
        path: String,
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
            Self::Source { .. } => "source",
            Self::Value { .. } => "value",
            Self::UnknownTable { .. } => "unknown_table",
            Self::Io { .. } => "io",
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
            Self::Source { entry, detail } => write!(f, "source entry `{entry}`: {detail}"),
            Self::Value { entry, detail } => write!(f, "value `{entry}`: {detail}"),
            Self::UnknownTable { name } => write!(
                f,
                "the registry declares `[{name}]`, which is not `verb`, `checklist`, `kb`, `feed` \
                 or `toolchain`"
            ),
            Self::Io { path, detail } => write!(f, "{path}: {detail}"),
        }
    }
}

impl std::error::Error for VerbRegistryError {}

/// The top-level tables the registry admits.
const REGISTRY_TABLES: [&str; 5] = ["verb", "checklist", "kb", "feed", "toolchain"];

/// A non-empty string member, or `None`.
fn nonempty_str<'a>(table: &'a toml_subset::Table, key: &str) -> Option<&'a str> {
    table
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty())
}

/// A key the value table and the catalog's `${toolchain.<key>}` can both spell.
fn is_key(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

impl VerbRegistry {
    /// Read a registry document that declares NO `path` source entry.
    ///
    /// A `path` entry is checked against the `.redkiln/` directory it is relative to, which a bare
    /// document does not have, so this refuses one by name rather than skip the check: use
    /// [`Self::from_toml_at`] or [`Self::load_file`].
    ///
    /// # Errors
    ///
    /// [`VerbRegistryError`] for a document outside the TOML subset, a missing `[verb]` table, a
    /// row missing or malforming any of its five columns, a malformed source entry or value.
    pub fn from_toml(src: &str) -> Result<Self, VerbRegistryError> {
        Self::parse(src, None)
    }

    /// Read a registry document whose `path` entries are relative to `redkiln_dir`.
    ///
    /// # Errors
    ///
    /// As [`Self::from_toml`], and a `path` that does not exist or holds no item.
    pub fn from_toml_at(
        src: &str,
        redkiln_dir: &std::path::Path,
    ) -> Result<Self, VerbRegistryError> {
        Self::parse(src, Some(redkiln_dir))
    }

    /// Read `registry.toml` from disk; its `path` entries are relative to its own directory.
    ///
    /// # Errors
    ///
    /// [`VerbRegistryError::Io`] when the file cannot be read, otherwise as [`Self::from_toml_at`].
    pub fn load_file(path: &std::path::Path) -> Result<Self, VerbRegistryError> {
        let src = std::fs::read_to_string(path).map_err(|e| VerbRegistryError::Io {
            path: path.display().to_string(),
            detail: e.to_string(),
        })?;
        let dir = path.parent().unwrap_or_else(|| std::path::Path::new(""));
        Self::from_toml_at(&src, dir)
    }

    fn parse(src: &str, redkiln_dir: Option<&std::path::Path>) -> Result<Self, VerbRegistryError> {
        let doc = toml_subset::parse(src).map_err(|error| VerbRegistryError::Toml { error })?;
        // `[verb]` FIRST: a document with no verb table is `no_verb_table`, whatever else it holds.
        let table = doc
            .get("verb")
            .and_then(Value::as_table)
            .ok_or(VerbRegistryError::NoVerbTable)?;
        if table.is_empty() {
            return Err(VerbRegistryError::NoVerbTable);
        }
        if let Some(name) = doc.keys().find(|k| !REGISTRY_TABLES.contains(&k.as_str())) {
            return Err(VerbRegistryError::UnknownTable { name: name.clone() });
        }
        let mut verbs = BTreeMap::new();
        for (name, value) in table {
            verbs.insert(name.clone(), verb_row(name, value)?);
        }
        let mut registry = Self {
            verbs,
            ..Self::default()
        };
        registry.checklists = source_table(&doc, "checklist", redkiln_dir, SourceKind::Checklist)?;
        registry.kbs = source_table(&doc, "kb", redkiln_dir, SourceKind::Kb)?;
        registry.feeds = feed_tables(&doc, &registry.verbs)?;
        registry.toolchains = value_tables(&doc)?;
        Ok(registry)
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

    /// The flat value table for the toolchain `rk-cli` selected (§8c.5), or `None` when the
    /// registry declares no `[toolchain.<name>]` for it. The name is the caller's: this crate
    /// compares against no adapter identity (AC-487).
    #[must_use]
    pub fn values_for(&self, toolchain: &str) -> Option<ToolchainValues> {
        self.toolchains
            .get(toolchain)
            .map(|t| ToolchainValues::new(t.values.clone()))
    }

    /// The loaded `[toolchain.<name>]` table, declarations included.
    #[must_use]
    pub fn table(&self, toolchain: &str) -> Option<&ValueTable> {
        self.toolchains.get(toolchain)
    }

    /// **Resolve one source input** (record V decision 8). `toolchain` names the feed table a
    /// `toolchain:<feed>` input reads — the item's `toolchain` facet value, data the caller holds.
    ///
    /// An **undeclared** key is refused by name, even when the input is optional. An
    /// `unauthored` / `unprovided` key is refused when mandatory and is [`SourceValue::Null`] when
    /// optional.
    ///
    /// # Errors
    /// [`SourceRefusal`], naming the key.
    pub fn resolve_source(
        &self,
        toolchain: &str,
        input: &str,
    ) -> Result<SourceValue, SourceRefusal> {
        let (bare, optional) = input
            .strip_suffix('?')
            .map_or((input, false), |b| (b, true));
        if let Some(id) = bare.strip_prefix("checklist:") {
            return resolve_entry(self.checklists.get(id), bare, optional);
        }
        if let Some(atom) = bare.strip_prefix("kb:") {
            return resolve_entry(self.kbs.get(atom), bare, optional);
        }
        if let Some(feed) = bare.strip_prefix("toolchain:") {
            let key = format!("feed.{toolchain}.{feed}");
            return match self.feeds.get(&(toolchain.to_owned(), feed.to_owned())) {
                None => Err(SourceRefusal::Undeclared { key }),
                Some(FeedEntry::Verb(verb)) => Ok(SourceValue::FeedVerb(verb.clone())),
                Some(FeedEntry::Unprovided(_)) if optional => Ok(SourceValue::Null),
                Some(FeedEntry::Unprovided(reason)) => Err(SourceRefusal::Unprovided {
                    key,
                    reason: reason.clone(),
                }),
            };
        }
        Err(SourceRefusal::NotASource {
            input: input.to_owned(),
        })
    }
}

fn resolve_entry(
    entry: Option<&SourceEntry>,
    key: &str,
    optional: bool,
) -> Result<SourceValue, SourceRefusal> {
    match entry {
        None => Err(SourceRefusal::Undeclared {
            key: key.to_owned(),
        }),
        Some(SourceEntry::Path(path)) => Ok(SourceValue::Path(path.clone())),
        Some(SourceEntry::Unauthored(_)) if optional => Ok(SourceValue::Null),
        Some(SourceEntry::Unauthored(reason)) => Err(SourceRefusal::Unauthored {
            key: key.to_owned(),
            reason: reason.clone(),
        }),
    }
}

/// One `[verb]` row, all five columns required (record V decision 1).
fn verb_row(name: &str, value: &Value) -> Result<VerbEntry, VerbRegistryError> {
    let shape = |detail: String| VerbRegistryError::Shape {
        verb: name.to_owned(),
        detail,
    };
    let entry = value
        .as_table()
        .ok_or_else(|| shape("is not a table".to_owned()))?;
    // `live` is REFUSED rather than defaulted: see the variant's own doc. A missing flag
    // defaulted to `false` would report every step using this verb as lawfully memoisable.
    let live = entry.get("live").and_then(Value::as_bool).ok_or_else(|| {
        shape(
            "declares no boolean `live`. Liveness is the verb registry's flag and nothing \
             else (ADR-0059); an absent one is refused rather than defaulted, because either \
             default decides a value the registry owes"
                .to_owned(),
        )
    })?;
    for key in entry.keys() {
        if !["outputs", "live", "timeout", "inputs", "route"].contains(&key.as_str()) {
            return Err(shape(format!(
                "declares `{key}`, which is not one of the five columns (outputs, live, timeout, \
                 inputs, route; ADR-0170 decision 1)"
            )));
        }
    }
    let outputs = nonempty_str(entry, "outputs")
        .ok_or_else(|| shape("declares no `outputs` kind".to_owned()))?
        .to_owned();
    let timeout = nonempty_str(entry, "timeout")
        .filter(|t| is_key(t))
        .ok_or_else(|| {
            shape("declares no `timeout` key (a bare `${toolchain.*}` key name)".to_owned())
        })?
        .to_owned();
    let inputs = entry
        .get("inputs")
        .and_then(Value::as_array)
        .ok_or_else(|| shape("declares no `inputs` array (an empty one is `[]`)".to_owned()))?
        .iter()
        .map(|v| {
            v.as_str()
                .filter(|s| !s.trim().is_empty())
                .map(ToOwned::to_owned)
                .ok_or_else(|| shape("an `inputs` member is not a non-empty string".to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let route_token =
        nonempty_str(entry, "route").ok_or_else(|| shape("declares no `route`".to_owned()))?;
    let route = match route_token {
        "port" => {
            let method = PortMethod::from_verb(name).ok_or_else(|| {
                shape(
                    "takes the `port` route, which reaches only the four existing methods \
                     (build, test, lint, change_summary)"
                        .to_owned(),
                )
            })?;
            if outputs != method.output_kind() {
                return Err(shape(format!(
                    "takes the `port` route and declares `outputs = {outputs:?}`; the port \
                     method's kind is `{}`",
                    method.output_kind()
                )));
            }
            Route::Port(method)
        }
        "toolchain" => Route::Toolchain,
        "store" => Route::Store,
        "git" => Route::Git,
        other => {
            return Err(shape(format!(
                "declares `route = {other:?}`, outside the closed set (port, toolchain, store, \
                 git). A route is added by record, not by a registry row (ADR-0170 decision 1)"
            )));
        }
    };
    Ok(VerbEntry {
        outputs,
        live,
        timeout,
        inputs,
        route,
    })
}

#[derive(Debug, Clone, Copy)]
enum SourceKind {
    Checklist,
    Kb,
}

/// `[checklist.*]` or `[kb.*]`: exactly one of `path` / `unauthored` per entry.
fn source_table(
    doc: &toml_subset::Table,
    form: &str,
    redkiln_dir: Option<&std::path::Path>,
    kind: SourceKind,
) -> Result<BTreeMap<String, SourceEntry>, VerbRegistryError> {
    let mut out = BTreeMap::new();
    let Some(table) = doc.get(form) else {
        return Ok(out);
    };
    let table = table.as_table().ok_or_else(|| VerbRegistryError::Source {
        entry: form.to_owned(),
        detail: "is not a table".to_owned(),
    })?;
    for (id, value) in table {
        let name = format!("{form}.{id}");
        let refuse = |detail: &str| VerbRegistryError::Source {
            entry: name.clone(),
            detail: detail.to_owned(),
        };
        let entry = value.as_table().ok_or_else(|| refuse("is not a table"))?;
        if entry.keys().any(|k| k != "path" && k != "unauthored") {
            return Err(refuse("declares a key other than `path` or `unauthored`"));
        }
        let parsed = match (
            nonempty_str(entry, "path"),
            nonempty_str(entry, "unauthored"),
        ) {
            (Some(_), Some(_)) => {
                return Err(refuse("declares BOTH `path` and `unauthored`; exactly one"));
            }
            (None, None) => {
                return Err(refuse(
                    "declares NEITHER a non-empty `path` nor a non-empty `unauthored`; exactly one",
                ));
            }
            (Some(path), None) => {
                check_path(path, redkiln_dir, kind).map_err(|d| refuse(&d))?;
                SourceEntry::Path(path.to_owned())
            }
            (None, Some(reason)) => SourceEntry::Unauthored(reason.to_owned()),
        };
        out.insert(id.clone(), parsed);
    }
    Ok(out)
}

/// A `path` exists under `.redkiln/` and holds an item. Declared paths only: nothing is walked.
fn check_path(
    path: &str,
    redkiln_dir: Option<&std::path::Path>,
    kind: SourceKind,
) -> Result<(), String> {
    let rel = std::path::Path::new(path);
    if !rel
        .components()
        .all(|c| matches!(c, std::path::Component::Normal(_)))
    {
        return Err(format!(
            "`path = {path:?}` is not a plain relative path under `.redkiln/`"
        ));
    }
    let Some(dir) = redkiln_dir else {
        return Err(format!(
            "`path = {path:?}` cannot be checked without the `.redkiln/` it is relative to; \
             load the registry from its file (`VerbRegistry::load_file` / `from_toml_at`)"
        ));
    };
    let full = dir.join(rel);
    let text = std::fs::read_to_string(&full)
        .map_err(|e| format!("`path = {path:?}` does not exist or cannot be read ({e})"))?;
    let holds_item = match kind {
        SourceKind::Kb => !text.trim().is_empty(),
        SourceKind::Checklist => toml_subset::parse(&text)
            .ok()
            .and_then(|d| {
                d.get("item")
                    .and_then(Value::as_array)
                    .map(|a| !a.is_empty())
            })
            .unwrap_or(false),
    };
    if holds_item {
        Ok(())
    } else {
        Err(format!(
            "`path = {path:?}` holds no item (a checklist needs at least one `[[item]]`; an atom \
             is not empty)"
        ))
    }
}

/// `[feed.<toolchain>.<name>]`: exactly one of `verb` / `unprovided`, and a `verb` is a row.
fn feed_tables(
    doc: &toml_subset::Table,
    verbs: &BTreeMap<String, VerbEntry>,
) -> Result<BTreeMap<(String, String), FeedEntry>, VerbRegistryError> {
    let mut out = BTreeMap::new();
    let Some(feed) = doc.get("feed") else {
        return Ok(out);
    };
    let refuse = |entry: String, detail: &str| VerbRegistryError::Source {
        entry,
        detail: detail.to_owned(),
    };
    let feed = feed
        .as_table()
        .ok_or_else(|| refuse("feed".to_owned(), "is not a table"))?;
    for (toolchain, table) in feed {
        let table = table.as_table().ok_or_else(|| {
            refuse(
                format!("feed.{toolchain}"),
                "is not a table of `[feed.<toolchain>.<name>]` entries",
            )
        })?;
        for (name, value) in table {
            let entry_name = format!("feed.{toolchain}.{name}");
            let entry = value
                .as_table()
                .ok_or_else(|| refuse(entry_name.clone(), "is not a table"))?;
            if entry.keys().any(|k| k != "verb" && k != "unprovided") {
                return Err(refuse(
                    entry_name,
                    "declares a key other than `verb` or `unprovided`",
                ));
            }
            let parsed = match (
                nonempty_str(entry, "verb"),
                nonempty_str(entry, "unprovided"),
            ) {
                (Some(_), Some(_)) => {
                    return Err(refuse(
                        entry_name,
                        "declares BOTH `verb` and `unprovided`; exactly one",
                    ));
                }
                (None, None) => {
                    return Err(refuse(
                        entry_name,
                        "declares NEITHER a non-empty `verb` nor a non-empty `unprovided`; \
                         exactly one",
                    ));
                }
                (Some(verb), None) => {
                    if !verbs.contains_key(verb) {
                        return Err(refuse(
                            entry_name,
                            &format!("declares `verb = {verb:?}`, which is not a registry row"),
                        ));
                    }
                    FeedEntry::Verb(verb.to_owned())
                }
                (None, Some(reason)) => FeedEntry::Unprovided(reason.to_owned()),
            };
            out.insert((toolchain.clone(), name.clone()), parsed);
        }
    }
    Ok(out)
}

/// `secs = 60 × ⌈(2 × max(measured)) / 60⌉` — ADR-0170 decision 10's rule, checked on every
/// `command` (measured-here) value. A cited (`source`) value may be the operator's (X4), so it is
/// not recomputed.
fn rule_secs(max_measured: f64) -> Option<u64> {
    let minutes = (2.0 * max_measured / 60.0).ceil();
    if !minutes.is_finite() || minutes < 0.0 || minutes > 1.0e12 {
        return None;
    }
    // Checked finite, non-negative and bounded above, so the cast neither truncates a fraction
    // (it was `ceil`ed) nor wraps.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "finite, non-negative, integral and bounded, checked on the line above"
    )]
    let minutes = minutes as u64;
    minutes.checked_mul(60)
}

/// A reading in `measured`: a number, or an inline table carrying a numeric `secs` (so a network
/// reading can be labelled, §8c.5).
fn reading(v: &Value) -> Option<f64> {
    let n = match v {
        Value::Table(t) => t.get("secs")?.as_f64()?,
        other => other.as_f64()?,
    };
    (n.is_finite() && n >= 0.0).then_some(n)
}

/// `[toolchain.<name>]`: every value with its warrant, every `unmeasured` with its reason, and
/// the named declaration rows (record V decision 9).
fn value_tables(
    doc: &toml_subset::Table,
) -> Result<BTreeMap<String, ValueTable>, VerbRegistryError> {
    let mut out = BTreeMap::new();
    let Some(toolchains) = doc.get("toolchain") else {
        return Ok(out);
    };
    let toolchains = toolchains
        .as_table()
        .ok_or_else(|| VerbRegistryError::Value {
            entry: "toolchain".to_owned(),
            detail: "is not a table".to_owned(),
        })?;
    for (toolchain, table) in toolchains {
        let refuse = |key: &str, detail: String| VerbRegistryError::Value {
            entry: format!("toolchain.{toolchain}.{key}"),
            detail,
        };
        let table = table
            .as_table()
            .ok_or_else(|| refuse("", "is not a table of keys".to_owned()))?;
        let mut loaded = ValueTable::default();
        for (key, value) in table {
            let without = |what: &str| {
                refuse(
                    key,
                    format!(
                        "{what}. Every value carries its warrant (`secs`, `measured`, `rule`, \
                         `pin`, and exactly one of `command` / `source`), or is declared \
                         `unmeasured = \"<reason>\"` (ADR-0170 decision 9)"
                    ),
                )
            };
            let Some(entry) = value.as_table() else {
                return Err(without("is a bare value, not a warrant"));
            };
            let keys: BTreeSet<&str> = entry.keys().map(String::as_str).collect();
            if DECLARATION_ROWS.contains(&key.as_str()) {
                let source = nonempty_str(entry, "source");
                match (keys == BTreeSet::from(["value", "source"]), source) {
                    (true, Some(source)) => {
                        let value = entry.get("value").cloned().unwrap_or(Value::Bool(false));
                        loaded.declarations.insert(
                            key.clone(),
                            Declaration {
                                value,
                                source: source.to_owned(),
                            },
                        );
                        continue;
                    }
                    _ => {
                        return Err(refuse(
                            key,
                            "is a declaration row and declares exactly `value` and a non-empty \
                             `source` (§8c.5)"
                                .to_owned(),
                        ));
                    }
                }
            }
            if keys == BTreeSet::from(["unmeasured"]) {
                let reason = nonempty_str(entry, "unmeasured")
                    .ok_or_else(|| without("declares an empty `unmeasured` reason"))?;
                loaded.values.insert(
                    key.clone(),
                    ToolchainValue::Unmeasured {
                        reason: reason.to_owned(),
                    },
                );
                continue;
            }
            let has_command = keys.contains("command");
            let has_source = keys.contains("source");
            let expected: BTreeSet<&str> = ["secs", "measured", "rule", "pin"]
                .into_iter()
                .chain(has_command.then_some("command"))
                .chain(has_source.then_some("source"))
                .collect();
            if has_command == has_source {
                return Err(without(if has_command {
                    "declares BOTH `command` and `source`"
                } else {
                    "declares neither `command` nor `source`"
                }));
            }
            if keys != expected {
                return Err(without(&format!(
                    "declares {keys:?}, not exactly {expected:?}"
                )));
            }
            let secs = entry
                .get("secs")
                .and_then(Value::as_int)
                .and_then(|s| u64::try_from(s).ok())
                .and_then(std::num::NonZeroU64::new)
                .ok_or_else(|| without("declares no positive integer `secs`"))?;
            let readings = entry
                .get("measured")
                .and_then(Value::as_array)
                .filter(|a| !a.is_empty())
                .ok_or_else(|| without("declares no non-empty `measured` list"))?
                .iter()
                .map(reading)
                .collect::<Option<Vec<f64>>>()
                .ok_or_else(|| {
                    without("carries a `measured` reading that is not a non-negative number")
                })?;
            nonempty_str(entry, "rule").ok_or_else(|| without("declares an empty `rule`"))?;
            let pin = entry.get("pin").and_then(Value::as_str).unwrap_or("");
            if pin.len() != 40
                || !pin
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            {
                return Err(without(&format!(
                    "declares `pin = {pin:?}`, not a 40-hex commit of this repository"
                )));
            }
            let origin = if has_command { "command" } else { "source" };
            nonempty_str(entry, origin)
                .ok_or_else(|| without(&format!("declares an empty `{origin}`")))?;
            if has_command {
                let max = readings.iter().copied().fold(0.0_f64, f64::max);
                if rule_secs(max) != Some(secs.get()) {
                    return Err(refuse(
                        key,
                        format!(
                            "declares `secs = {secs}` over a slowest reading of {max} s; \
                             ADR-0170 decision 10's rule, `60 × ⌈(2 × max(measured)) / 60⌉`, \
                             gives {:?}. A measured value follows the rule written before its \
                             readings",
                            rule_secs(max)
                        ),
                    ));
                }
            }
            loaded
                .values
                .insert(key.clone(), ToolchainValue::Secs(secs));
        }
        out.insert(toolchain.clone(), loaded);
    }
    Ok(out)
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
