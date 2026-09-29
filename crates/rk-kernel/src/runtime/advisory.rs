//! **AC-525's run-scoped, log-free advisory** — derived and appended at the runtime boundary,
//! de-duplicated in process, and never by reading the log.
//!
//! > **AC-525** — GIVEN a vouched root and a same-stage upstream artifact in the root's edge set
//! > (AC-067) whose digest changes / WHEN step 3 next classifies the root / THEN **exactly one**
//! > `drift_advisory { root, upstream, old_digest, new_digest }` event is appended for that
//! > `(root, upstream, new_digest)` — never zero, never one per re-entry / AND the next
//! > `gate_opened` record the item reaches carries that advisory.
//!
//! The 2026-09-13 amendment is part of the record and it is what this module is built to:
//!
//! > **(1) Step 3 returns a classification and nothing else.** *"`select()` **never reads the event
//! > log** — AC-041 makes the four digests its only inputs, and a `select()` that scanned the log to
//! > de-duplicate would be exactly the hidden input AC-041, AC-042 and AC-043 exist as a chain to
//! > prevent, while still passing all four of the fixtures named above."*
//! > **(2) The advisory is derived and appended at the runtime boundary.**
//! > **(3) *"Exactly one, never zero, never one per re-entry"* is RUN-SCOPED.** *"The de-duplication
//! > memory is therefore **in-process at the append site**, keyed on the `(root, upstream,
//! > new_digest)` triple this THEN already names, and **no log read happens anywhere** — not in
//! > `select()`, not at the boundary."*
//! > **(4) The instrument gains one, and it is the discriminating one.** *"A replay over a run that
//! > emitted an advisory must re-derive the same DAG with no store read. The first four are all
//! > passed by an implementation that de-duplicates by scanning the log; this one is not."*
//!
//! # The no-log-read rule is a TYPE FACT here, not a discipline
//!
//! [`AdvisorySink::admit`] and [`derive_advisories`] take **no store, no ports and no journal** —
//! their only inputs are `&[crate::select::DriftSignal]` and the sink's own `BTreeSet`. There is no
//! expression inside this module that could read the log, and `ac525_drift_advisory_once` asserts
//! the consequence with a query-counting `EventStore` fake: the count is unchanged across the whole
//! derivation, and unchanged again across the replay's re-derivation of the DAG.
//!
//! # 🛑 RUN-SCOPED IS A COST, AND THE AMENDMENT STATES IT RATHER THAN IMPLYING IT
//!
//! > *"**Item-scoped-across-runs is not chosen**, and the consequence is stated rather than implied:
//! > a second `redkiln run` over the same unchanged upstream **may append a second advisory** for
//! > the same triple, one per run."*
//!
//! So [`AdvisorySink`] is created per run and dropped with it. A sink that outlived a run would be
//! process-lifetime state of exactly the kind AC-034's lint bans one module over, and a sink that
//! consulted the store to survive one would fail instrument (4).
//!
//! # 🛑 WHAT IS MEASURED AND WHAT IS NOT
//!
//! | clause | state |
//! | --- | --- |
//! | exactly one advisory per `(root, upstream, new_digest)`, with all four fields | **measured** |
//! | none on a re-entry **within the run** | **measured** |
//! | the next `gate_opened` carries it | **measured**, via [`carried_by_gate`] |
//! | AC-067's out-of-set fixture emits nothing | **measured** — step 3 returns no signal, so the sink is handed none |
//! | amendment (4)'s replay | **measured** as *"the re-derivation makes zero `query` calls and the DAG's canonical bytes are unchanged"* |
//! | `advisory_created` / `advisory_retired` lifecycle | **not built, and the amendment says not to**: *"no `advisory_created` / `advisory_retired` lifecycle is minted for it (that accounting is AC-310's and AC-530's …)"* |
//! | SM-022's noise rate | **no baseline. Reported, never thresholded** — the register and §7b.13 and §7b.15 all say so |
//! | the matching note in **design §5** naming this producing site | **OWED** — *"named here, not taken here"* |
//!
//! And one more, because it is the half a stage could easily over-claim: AC-525 is attributed to
//! `rk-select` **+** `rk-kernel`, and after the amendment *"the `rk-select` half does less than the
//! unamended wording suggests … Step 3 returns `Present`; every asserted behaviour — the derivation,
//! the *exactly one*, the append, and the carriage on the next `gate_opened` record — is
//! `rk-kernel`'s at the boundary."* The selection-side half covers a **classification** and nothing
//! more.

use std::collections::BTreeSet;

use rk_types::event::Event;
use rk_types::event_kind::EventKind;

use crate::select::{Digest, DriftSignal};

/// One advisory, with the four fields the THEN enumerates and no fifth.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Advisory {
    root: String,
    /// *(dated 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED; `s8f` §8f.5; the render
    /// record ADR-0189 decision 10, proposed; D21-47, review m11.)* ~~`String`~~: `None` marks the
    /// vouched root's OWN change ([`Advisory::own_change`]), serialised as JSON `null`, so the
    /// own-change key `(root, None, new_digest)` is never equal to an upstream key. `Some(s)`
    /// serialises as the same string it did before, byte for byte.
    upstream: Option<String>,
    old_digest: Option<Digest>,
    new_digest: Digest,
}

impl Advisory {
    /// The four member names the criterion enumerates, so *"all four fields and no fifth"* is
    /// asserted against a named list rather than against four string literals in a test.
    pub const FIELDS: [&'static str; 4] = ["new_digest", "old_digest", "root", "upstream"];

    /// Build one from the signal step 3 returned: the UPSTREAM half (AC-525). ~~**The only
    /// constructor**~~ *(corrected 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED; the
    /// render record ADR-0189 decision 10, proposed)*: **one of two constructors**. This one mints
    /// an advisory `select()` classified; [`Self::own_change`] mints the vouched root's own change,
    /// which `rk-store`'s entry pass observes (AC-127's first half). No third route mints one.
    #[must_use]
    pub fn from_signal(signal: &DriftSignal) -> Self {
        Self {
            root: signal.root.clone(),
            upstream: Some(signal.upstream.clone()),
            old_digest: signal.old_digest.clone(),
            new_digest: signal.new_digest.clone(),
        }
    }

    /// **A vouched root's OWN change** (AC-127's first half; ADR-0025 as the render record reads
    /// it; `s8f` §8f.5). `upstream` is `None` — written `null`, the own-change mark — `old_digest`
    /// is the vouched digest and `new_digest` the file's (`Drift::Advisory`'s `vouched` and
    /// `observed`). The four fields and no fifth: the causing commit is not carried (ADR-0189
    /// decision 10). *(Added 2026-09-28, s8f build, package S8F-P5-DRIFT-GATE-VOUCHED.)*
    #[must_use]
    pub fn own_change(root: &str, vouched: Digest, observed: Digest) -> Self {
        Self {
            root: root.to_owned(),
            upstream: None,
            old_digest: Some(vouched),
            new_digest: observed,
        }
    }

    /// The de-duplication key the THEN itself names: `(root, upstream, new_digest)`.
    ///
    /// **`old_digest` is deliberately not in it.** The triple is the criterion's own, and including
    /// the old digest would make two advisories for one new digest distinct whenever the recorded
    /// baseline moved — which is *"one per re-entry"* by another route.
    #[must_use]
    pub fn triple(&self) -> (String, Option<String>, String) {
        (
            self.root.clone(),
            self.upstream.clone(),
            self.new_digest.as_str().to_owned(),
        )
    }

    #[must_use]
    pub fn root(&self) -> &str {
        &self.root
    }

    #[must_use]
    pub fn upstream(&self) -> Option<&str> {
        self.upstream.as_deref()
    }

    #[must_use]
    pub const fn old_digest(&self) -> Option<&Digest> {
        self.old_digest.as_ref()
    }

    #[must_use]
    pub const fn new_digest(&self) -> &Digest {
        &self.new_digest
    }

    /// The advisory as JSON. `old_digest` is `null` when the root recorded no baseline — the
    /// absence is written, never dropped, so a reader can tell *"no baseline"* from *"no member"*.
    #[must_use]
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "new_digest": self.new_digest.as_str(),
            "old_digest": self.old_digest.as_ref().map(Digest::as_str),
            "root": self.root,
            "upstream": self.upstream,
        })
    }

    /// The advisory as the event that carries it — `EventKind::DriftAdvisory`, design §5 row 22.
    #[must_use]
    pub fn to_event(&self) -> Event {
        Event::new(EventKind::DriftAdvisory, self.to_json())
    }
}

/// **The run-scoped, in-process de-duplication memory** the amendment's clause 3 requires.
///
/// Created per `redkiln run`, dropped with it. It holds triples and **nothing else** — no store, no
/// ports, no journal — so instrument (4) is satisfied by the type rather than by a discipline.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AdvisorySink {
    seen: BTreeSet<(String, Option<String>, String)>,
    admitted: usize,
    suppressed: usize,
}

impl AdvisorySink {
    /// A sink for one run.
    #[must_use]
    pub fn for_run() -> Self {
        Self::default()
    }

    /// Admit one advisory, or suppress it as already emitted **in this run**.
    ///
    /// Returns `true` the first time a triple is seen and `false` every time after.
    pub fn admit(&mut self, advisory: &Advisory) -> bool {
        if self.seen.insert(advisory.triple()) {
            self.admitted += 1;
            true
        } else {
            // COUNTED, never silently dropped: *"never one per re-entry"* has to be
            // distinguishable from *"the sink was never asked"*, and a bare `false` is not.
            self.suppressed += 1;
            false
        }
    }

    /// How many advisories this run emitted. **House Shape C**: without it, *"no second advisory"*
    /// cannot be told from *"no advisories at all"*.
    #[must_use]
    pub const fn admitted(&self) -> usize {
        self.admitted
    }

    /// How many were suppressed as duplicates — the count that makes the re-entry case falsifiable.
    #[must_use]
    pub const fn suppressed(&self) -> usize {
        self.suppressed
    }

    /// The triples emitted so far this run, in key order.
    #[must_use]
    pub fn emitted(&self) -> Vec<&(String, Option<String>, String)> {
        self.seen.iter().collect()
    }
}

/// Derive the advisories one classification pass owes, and de-duplicate them **in process**.
///
/// **Takes no store.** The signature is instrument (4)'s type-level half: there is no argument
/// through which a log could be read.
#[must_use]
pub fn derive_advisories(sink: &mut AdvisorySink, signals: &[DriftSignal]) -> Vec<Advisory> {
    signals
        .iter()
        .map(Advisory::from_signal)
        .filter(|advisory| sink.admit(advisory))
        .collect()
}

/// **AC-525's second AND**: *"the next `gate_opened` record the item reaches carries that
/// advisory."*
///
/// The advisories ride **beside** the gate record's three bound members rather than inside them, so
/// the token `crate::workflow::fsm::GateRecord::derive` computes over `(kind, tree_hash, deadline)`
/// is unchanged — AC-039's binding is not disturbed by a drift note, and two gates differing only in
/// their advisories are still one gate.
///
/// A gate reached before any advisory was emitted carries an **empty list**, written rather than
/// omitted: *"never zero"* is a claim about the advisory, and a gate with no `advisories` member at
/// all could not be told from a gate whose carriage was dropped.
#[must_use]
pub fn carried_by_gate(gate: &Event, advisories: &[Advisory]) -> Event {
    // The gate's payload is an object by the time this is reachable: `super::gate_opened_event`
    // builds one, and `crate::workflow::fsm::fold`'s `gate_opened` arm refuses a payload carrying
    // neither the runtime's `tree_hash`/`deadline` nor the selection's `reason`. The `unwrap_or_
    // default` below is therefore unreachable rather than a silent drop, and it is stated here
    // rather than left as a habit.
    let mut object = gate
        .payload
        .as_object()
        .cloned()
        .unwrap_or_else(serde_json::Map::new);
    object.insert(
        "advisories".to_owned(),
        serde_json::Value::Array(advisories.iter().map(Advisory::to_json).collect()),
    );
    Event::new(gate.kind, serde_json::Value::Object(object))
}

/// **The run's advisories not yet carried by a gate** (AC-525's second AND). The composition root
/// creates one per run beside its [`AdvisorySink`], pushes each advisory the sink ADMITS, and binds
/// it to the run's tier-1 boundaries (`tier_one::Boundary::with_advisories`); the next gate opened
/// through one of them carries every advisory pending and takes them. Like the sink, it lives and
/// dies with the run and holds no store. *(Added 2026-09-28, s8f build, package
/// S8F-P5-DRIFT-GATE-VOUCHED; ADR-0189 decision 10, proposed.)*
pub type PendingAdvisories = std::sync::Arc<std::sync::Mutex<Vec<Advisory>>>;

/// Every `drift_advisory` the journal holds, in append order.
#[must_use]
pub fn recorded_advisories(events: &[Event]) -> Vec<&Event> {
    events
        .iter()
        .filter(|e| e.kind == EventKind::DriftAdvisory)
        .collect()
}
