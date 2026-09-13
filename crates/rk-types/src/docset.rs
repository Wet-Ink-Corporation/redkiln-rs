//! `DocSet` — what `DocsSite::build` is handed.
//!
//! Homed here by design:1699, which lists `DocSet` among this crate's typed artifacts. It was
//! **not on the tree**: before Step 7a the only occurrence of the name under `crates/` was this
//! crate's own `lib.rs` doc header [measured 2026-09-13].
//!
//! **It is an opaque newtype, and the contents are OWED.** `DocsSite` has no Phase-1 real adapter
//! — mdBook is Phase 3 (design:268; AC-499's GIVEN) — so nothing outside the port's own no-op
//! recorder consumes a `DocSet` before then, and inventing a doc-set schema now would be a design
//! task no record has taken. That is the treatment ADR-0099 clause 5 gives the one other blocked
//! type.
//!
//! **Unlike `rk-ports`' three opaque newtypes, this one ships a public constructor**, and the
//! difference is mechanical rather than stylistic: `ExternalRef`, `TypedInput` and the
//! S-BIN-blocked artifact type are `rk-ports`-local and so are constructible by that crate's own
//! fakes, while `DocSet` crosses a crate boundary — a `pub(crate)`-fielded newtype here would make
//! `DocsSite::build` impossible to call from `rk-ports`, and the no-op recorder could never be
//! exercised against it. The constructor is the shape [`crate::text::Text`] and
//! [`crate::time::Timestamp`] take.
//!
//! **The third newtype is named by its role and not by its name, deliberately.** ADR-0099's
//! forbids bullet (:131) is *"Forbids \[it] appearing anywhere in `crates/` but its `rk-ports`
//! newtype until S-BIN reports"*, and the instrument that enforces it — Checkpoint 5's
//! `grep -rn … crates/ --include='*.rs' | grep -v 'crates/rk-ports/'` — is **textual** and
//! excludes only `crates/rk-ports/`. So a doc comment in this crate that spells the token fails a
//! live Step-5 gate however plainly it is talking *about* the rule. This crate's own `lib.rs`:9-12
//! takes the same workaround for the same reason.

/// The set of documents a docs-site build is handed. **Opaque: its contents are not fixed.**
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DocSet(());

impl DocSet {
    /// The only constructor. It takes nothing, because nothing is decided about the contents —
    /// a constructor that took paths would fix half a schema by accident.
    #[must_use]
    pub const fn new() -> Self {
        Self(())
    }
}
