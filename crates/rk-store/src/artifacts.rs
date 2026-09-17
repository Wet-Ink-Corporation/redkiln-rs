//! `rk-store-artifacts` — the register's subject tag for the review and verify gates' stored
//! records, **a module of `rk-store`** (design §9.1: *"`rk-store-artifacts` of `rk-store`"*; ADR-0099
//! clause 5: no label is a crate until a later split). Opened at Step 7b-ii for AC-305 and AC-310;
//! the verify gate's AC-329 and AC-333 landed beside them the same day.
//!
//! Four things live here, and what each is NOT:
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
//! - [`verify_report`] — AC-329's **field semantics**: the whole-suite comparator record (the most
//!   recent recorded whole-suite count at a toolchain version, and the run it came from), the two
//!   meanings of `total_test_count` (ADR-0104 decision 3), and the validation of a report against
//!   that record. It is NOT the reclassifier that emits `selection_ineffective` (AC-326,
//!   `rk-kernel`'s) and NOT the per-adapter schema sweep (`s7c`'s).
//! - [`ledger`] — AC-333's **completeness check**: the persisted ledger's AC id set equals the
//!   spec's, every row valid, no duplicate. It is NOT the ledger's computation (`rk-kernel`'s
//!   verify module, AC-334/AC-335) and reads no suite verdict, finding or quarantine.
//!
//! Neither of the two verify modules persists anything yet either: like the advisory record, a
//! verify-report table and a ledger table are migrations a later stage authors, and both are
//! named here as OWED rather than deferred bare.

pub mod finding_identity;
pub mod ledger;
pub mod sink;
pub mod verify_report;
