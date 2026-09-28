//! The `EventStore` port — *"append; query by item / SHA / run; snapshot; rebuild"* (design:265).
//!
//! **The method NAMES are design §3.3's responsibility column, not the spike's.**
//! `spikes/s-ports/` has `append` + `read` and no query, snapshot or rebuild; it is a shape check,
//! not an interface.
//!
//! **Everything below the names is a ROUTING, not a decision** (runbook `s7a` §7a.3), reversible
//! by the record that is owed:
//!
//! * design:265's *"query by item / SHA / run"* fixes neither the arity nor the key types, so the
//!   trio is **one method with a selector**, [`Query`], an `rk-ports` enum of three variants. What
//!   each variant's key type should be is fixed by no record and Checkpoint 7a asserts none of it.
//! * `snapshot` / `rebuild` take an opaque [`Snapshot`]. **No field set is invented.**
//!   *(dated 2026-09-28, `s8e` build, package S8E-P7; ADR-0181 decision 11, proposed; D21-42 8c:
//!   the tier-1 record decides its contents — the ordered list of anchored rows a store holds —
//!   and this crate gives it a public constructor. It is no longer opaque.)*
//! * every fallible port method returns `Result<_, StepError>`, `rk-types`' eleven-variant enum —
//!   the one error type any record puts across a port boundary (ADR-0099:57).
//!
//! `append`'s parameter is `rk-types`' `Event { kind: EventKind, payload: serde_json::Value }`,
//! routed there because ADR-0105 clause 7 places the event-kind table in design §5 with `rk-types`
//! as the owning crate. **The per-kind payload field sets are NOT typed here and that is reported
//! as ARMED, NOT DISCHARGED** — no assertion in Checkpoint 7a claims the payload is typed.

use rk_types::error::StepError;
use rk_types::event::Event;

/// Which events to return. design:265's three keys as one selector.
///
/// **Routed, not decided.** The key types are `String` because no record fixes them and a typed
/// item id, a typed commit SHA and a typed run id are three type-design decisions Step 7a is not
/// the stage to take.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Query {
    ByItem(String),
    BySha(String),
    ByRun(String),
}

/// A point-in-time snapshot of a store. ~~**Opaque: no field set is invented.**~~
///
/// *(dated 2026-09-28, `s8e` build, package S8E-P7; ADR-0181 decision 11, proposed; `s8e` §8e.9;
/// D21-42 8c: *"`open_at` / `Capability::Rebuild`"*.)* **A `Snapshot` is the ordered list of rows a
/// store holds** ([`SnapshotRow`]): each event with its interim `item` key (on the event,
/// ADR-0182), its `anchor_sha` when it was written through `append_tier1`, and the two facts of its
/// `t1_*` row that are not in the payload — the anchor commit's tree (a boundary row's
/// `tree_hash`) and, for a gate opening, the opening stage run. The `t1_*` row itself is a function
/// of those and the event (ADR-0181 decision 3's closed mapping), so it is not carried twice.
/// `rebuild` writes a snapshot's rows **append-only**: a store whose rows are a prefix of the
/// snapshot's gains the rest, in order, and any other store refuses — so a rebuild from a snapshot
/// of the current state is identity, and a fresh store (the staging file `doctor --rebuild` opens
/// with `SqliteStore::open_at`) gains every row. Nothing is updated or deleted (AC-364).
///
/// The paragraph below is left as written before `s8e`:
///
/// One of this crate's four opaque newtypes, and one of the three with no record behind them
/// (`ChangeSummary` is ADR-0099 clause 5's). The cost is stated rather than smoothed in `s7a`
/// §7a.3. *(amended 2026-09-25, s8a fix round 1, ADR-0165: `ChangeSummary` is no longer an opaque
/// newtype but an enum with a finalised `Text` arm, so this is one of the crate's three opaque
/// newtypes, none of them with a record behind it.)* *(Corrected 2026-09-25, s8a fix round 3: the
/// crate still holds four. `ChangeSummary`'s sealed `Binary` payload, `BinaryChangeSummary`, is
/// the fourth, and ADR-0165 stands behind it; this is one of the three with no record behind
/// them, beside `ExternalRef` and `TypedInput`.)*
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Snapshot {
    rows: Vec<SnapshotRow>,
}

impl Snapshot {
    /// A snapshot of `rows`, in order. The public constructor (ADR-0181 decision 11).
    #[must_use]
    pub const fn new(rows: Vec<SnapshotRow>) -> Self {
        Self { rows }
    }

    /// The rows, in the order they were appended.
    #[must_use]
    pub fn rows(&self) -> &[SnapshotRow] {
        &self.rows
    }
}

/// One row of a [`Snapshot`] (ADR-0181 decision 11; dated 2026-09-28, `s8e` build, package
/// S8E-P7).
///
/// `anchor_sha` is `Some` exactly for a row written through `append_tier1` (or re-derived from its
/// trailer commit by a rebuild). `tree` is the anchor commit's tree where the store keeps it — a
/// boundary's `t1_stage_transition.tree_hash` — and `None` elsewhere, because no other table
/// stores it. `opening_run` is a gate opening's `(stage, run_id)`, `None` when the store holds none
/// (a rebuilt opening, D21-48 X11).
#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotRow {
    pub event: Event,
    pub anchor_sha: Option<String>,
    pub tree: Option<String>,
    pub opening_run: Option<(String, String)>,
}

impl SnapshotRow {
    /// A row appended through `append`: no anchor.
    #[must_use]
    pub const fn unanchored(event: Event) -> Self {
        Self {
            event,
            anchor_sha: None,
            tree: None,
            opening_run: None,
        }
    }

    /// A row anchored to `anchor`'s commit, carrying its tree and opening run.
    #[must_use]
    pub fn anchored(event: Event, anchor: &TierOneAnchor) -> Self {
        Self {
            event,
            anchor_sha: Some(anchor.sha().to_owned()),
            tree: Some(anchor.tree().to_owned()),
            opening_run: anchor
                .opening_run()
                .map(|(stage, run)| (stage.to_owned(), run.to_owned())),
        }
    }
}

/// The anchor a tier-1 row is written against — ADR-0181 decision 2 (`s8e` §8e.2; D21-41 e).
///
/// *(dated 2026-09-28, s8e build; ADR-0181, proposed.)* It carries the boundary act's facts that
/// are not payload: the trailer commit's sha, **that commit's tree** (ADR-0105 clause 4: *"When a
/// checkpoint commit is due, the recorded tree is that commit's tree."*), and — for a gate opening
/// only — the opening stage run's `stage` and `run_id`, which are columns of the `opened`
/// transition row and not members of `gate_opened`'s payload, so no canonical event moves for
/// them. No value is validated here beyond what the store's `NOT NULL` columns demand: the kernel's
/// boundary function reads the sha and the tree back from git after its commit (§8e.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TierOneAnchor {
    sha: String,
    tree: String,
    opening_run: Option<(String, String)>,
}

impl TierOneAnchor {
    /// The anchor commit's sha and its tree.
    #[must_use]
    pub fn new(sha: impl Into<String>, tree: impl Into<String>) -> Self {
        Self {
            sha: sha.into(),
            tree: tree.into(),
            opening_run: None,
        }
    }

    /// The same anchor, carrying the opening stage run for a gate's `opened` row (`s8e` §8e.7).
    #[must_use]
    pub fn with_opening_run(mut self, stage: impl Into<String>, run_id: impl Into<String>) -> Self {
        self.opening_run = Some((stage.into(), run_id.into()));
        self
    }

    /// The trailer commit's sha.
    #[must_use]
    pub fn sha(&self) -> &str {
        &self.sha
    }

    /// The trailer commit's tree.
    #[must_use]
    pub fn tree(&self) -> &str {
        &self.tree
    }

    /// The opening stage run, `(stage, run_id)`, when the act opens a gate.
    #[must_use]
    pub fn opening_run(&self) -> Option<(&str, &str)> {
        self.opening_run
            .as_ref()
            .map(|(stage, run)| (stage.as_str(), run.as_str()))
    }
}

/// The event store the kernel appends to and replays from.
///
/// `append` takes `&mut self`, which is why the composition root holds this one port as
/// `Arc<Mutex<dyn EventStore>>` and not as a bare `Arc<dyn EventStore>` (ADR-0115 clause 1).
pub trait EventStore: Send + Sync {
    /// Append one event. ADR-0105 clause 7 puts one writer per process behind a mutex; the lock
    /// is the composition root's and **this method never acquires it**.
    fn append(&mut self, event: Event) -> Result<(), StepError>;

    /// Append one event **anchored to the trailer commit that carries it**, and, where the event's
    /// kind has a tier-1 table, that table's row, in ONE transaction (ADR-0181 decisions 1 and 3;
    /// D21-41 e; ADR-0158 decision 4's route). The fifth method, admitted by that record: the
    /// store stays append-only. Its only caller is the kernel's boundary function, after the
    /// commit exists (`s8e` §8e.3), and the conformance arm that exercises the port.
    fn append_tier1(&mut self, event: Event, anchor: &TierOneAnchor) -> Result<(), StepError>;

    /// Every event matching the selector, in append order.
    ///
    /// *(dated 2026-09-28, s8e build; ADR-0181 decision 12 and ADR-0182 decisions 4 and 5,
    /// proposed.)* `ByItem(x)` returns the rows keyed `x` and, in the interim, the rows carrying no
    /// key, never a row keyed to another item; `BySha(s)` returns the rows anchored to commit `s`;
    /// `ByRun` stays inert and returns every row.
    fn query(&self, by: Query) -> Result<Vec<Event>, StepError>;

    /// Take a snapshot of the current state: every row, in append order ([`Snapshot`]).
    fn snapshot(&self) -> Result<Snapshot, StepError>;

    /// Rebuild from a snapshot, append-only: a store whose rows are a prefix of the snapshot's
    /// gains the rest; any other store refuses and nothing is written ([`Snapshot`]).
    fn rebuild(&mut self, snapshot: &Snapshot) -> Result<(), StepError>;
}
