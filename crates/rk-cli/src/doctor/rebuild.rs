//! **`doctor --rebuild` — git → store, never the reverse** (AC-427, AC-429; ADR-0105 clauses 1 and
//! 7; `s7d` §7d.8).
//!
//! The direction is the whole criterion. `GitOp::Log` over the trailer commits →
//! `rk_git::reconcile` → [`events_from_trailers`] → `rk_kernel::workflow::fsm::fold` per item →
//! `fsm::stage_action`, written to a **new store file and renamed over the old** (ADR-0105 clause
//! 7: *"a rebuild writes to a separate file and renames"*). AC-429's THEN is that **no git object,
//! ref or working-tree file** was written on the way.
//!
//! # 🛑 `Gate:` MARKS ARE NOT REBUILT AS `gate_opened` ROWS, and the reason is measured
//!
//! The fold's `gate_opened` arm REQUIRES `kind` and `deadline` and **derives** the token from
//! `(kind, tree_hash, deadline, reach)`, ignoring any `token` in the payload
//! [`crates/rk-kernel/src/workflow/fsm.rs`]. The `Gate:` trailer carries `(token, tree_hash,
//! deadline)` and no `kind` or `reach`. So a row built from a trailer is either `MalformedPayload`
//! or derives a token that is **not** the trailer's — the rebuild would invent a gate identity.
//! `rk_git::reconcile::GateRecord` is recorded as the fact it is and listed as *unresolvable from
//! git alone* until a record widens the triple or makes the token the fold's key (RAISED, §7d.18).
//! `ac427_…`'s synthetic repository therefore carries **no gate**.
//!
//! # 🛑 The module-path ban: nothing here re-opens the store
//!
//! Checkpoint 7d's D-10 ban admits both `src/doctor/*.rs` and `src/verbs/doctor.rs`, and
//! `SqliteStore::open` is on its pattern. That is not incidental to this file — it is the one that
//! would most naturally reach for it. **`Ctx.store` is `None` on this path** and the rebuilt store
//! is re-opened for its identity check **by the root**, in `adapters.rs`, after `verbs::doctor` has
//! returned. On Windows this is also a correctness rule and not only a layering one: `Writer` holds
//! a `Mutex<Connection>` in WAL mode, and `std::fs::rename` over a file another handle holds open
//! without `FILE_SHARE_DELETE` fails with a sharing violation. `ac427_…` runs on Windows and is red
//! on exactly that.
//!
//! # 🚧 CYCLE 2: the signatures are real and the bodies are cycle 9's
//!
//! [`events_from_trailers`] returns the empty vector — the honest empty answer, not `todo!()` (this
//! crate's rule, `verbs/mod.rs`'s header). That is also what makes cycle 9's red the one AC-427
//! and AC-429 name rather than a panic assertion: `ac429_…`'s two **positive controls** run first
//! — the fingerprint set is non-empty and names `HEAD` and at least one ref, and *the rebuild
//! happened* (the store file was recreated and holds ≥ 1 tier-1 row) — and a rebuild that produced
//! no events fails the second before the negative is ever read (Halt 8 (d)/(g)).
//!
//! **The verb does not call this path at cycle 2.** `verbs/doctor.rs` refuses `--rebuild` with a
//! value and a reason, because this path *renames a new store over the old* and a half-built one
//! would rename an **empty** store over a real one. That refusal is a decision, stated at its site,
//! and cycle 9 removes it.

use rk_git::reconcile::State;
use rk_types::event::Event;

/// Turn the git-side reconciled state into the tier-1 events a rebuilt store holds.
///
/// **Stated so two implementers build one shape** (§7d.8): `Story:` marks → `stage_transition`,
/// `Slice-Verdict:` → `slice_sealed`, **`Gate:` marks → nothing** (the header). Tier-1 kinds only;
/// payloads built from the trailer values and the commit sha; **no tier-2 row**, which is exactly
/// what `RK-D-REBUILT-TIER2` reports as `ok`.
///
/// 🚧 Cycle 2 — the signature is real; cycle 9 fills it.
#[must_use]
#[expect(
    clippy::missing_const_for_fn,
    reason = "const only because the cycle-2 body is empty — cycle 9 reads `state`'s maps and               allocates, so constness is an artefact of the skeleton and not of the design. The               expectation goes UNFULFILLED the moment that body lands, which is what tells the               later cycle to delete this attribute."
)]
pub fn events_from_trailers(state: &State) -> Vec<Event> {
    let _ = state;
    Vec::new()
}
