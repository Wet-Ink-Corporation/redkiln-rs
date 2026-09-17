//! `rk-store-artifacts` — the register's subject tag for the review and verify gates' stored
//! records, **a module of `rk-store`** (design §9.1: *"`rk-store-artifacts` of `rk-store`"*; ADR-0099
//! clause 5: no label is a crate until a later split). Opened at Step 7b-ii for AC-305 and AC-310;
//! the verify gate's AC-329 and AC-333 land beside them.
//!
//! Two things live here, and what each is NOT:
//!
//! - [`finding_identity`] — the stable identity of a finding (AC-305): a fifth hashed tuple in the
//!   house form of `memo.rs`'s `StepKeyComponents`, with its own struct and its own `digest()` and
//!   **no helper shared with the step key, the activity key, the idempotency key or the selection
//!   memo key** (`memo.rs`:17-20; ADR-0119's forbids). It is a pure function of the finding's typed
//!   members and nothing else.
//! - [`sink`] — the advisory sink's **accounting** (AC-310, AC-530): a pure fold over the
//!   `advisory_created` / `advisory_retired` / `advisory_dismissed` rows and the intake identity
//!   `findings_in == blocks + sink_records`, each failure printing both sides. It is NOT the durable
//!   typed record ADR-0082 decision 1 requires (*"written to the item's durable finding record in
//!   the same typed `Findings` shape as a blocking finding"*): `rk-store` has no advisory table
//!   (the one migration defines seven, none advisory) and nothing applies migrations yet. **That
//!   record is OWED, with a named destination**: the tier-2 event payload carries the finding
//!   verbatim today (`advisory_created` is written with the `Finding` object), and a table for it
//!   is a migration a later stage authors — not a bare deferral (`s7a`:1220).

pub mod finding_identity;
pub mod sink;
