//! The `DocsSite` port — *"`build(DocSet)`, `check_links`, `drift(changed_sources)"* (design:268).
//!
//! The spike has `publish(path, body)`, which design:268 replaced; Checkpoint 7a refuses that name
//! for exactly that reason.
//!
//! **Two routings, both stated so a record can reverse them.** `build` takes `&DocSet`, and
//! `DocSet` is `rk-types`' by design:1699 — it was on no crate before Step 7a and is authored
//! there as an opaque newtype with its contents OWED. And `drift` returns **`Vec<Drift>`** rather
//! than a bare `Drift`: `rk-types`' `Drift` is an enum of eleven concrete observations with **no
//! "no drift" variant** (`drift.rs`:21-45), so a bare return would assert that a drift check
//! always finds one. `Vec<Drift>` returns `rk-types`' `Drift`, zero or more of them, which is what
//! design:268 asks for and what the type can honestly express.

use rk_types::docset::DocSet;
use rk_types::drift::Drift;
use rk_types::error::StepError;

/// The documentation site. Fake-only in Phase 1; mdBook is Phase 3.
pub trait DocsSite: Send + Sync {
    /// Build the site from a document set.
    fn build(&self, docs: &DocSet) -> Result<(), StepError>;

    /// Check every link the built site carries.
    fn check_links(&self) -> Result<(), StepError>;

    /// What the sources say that the published site cannot explain. **Reported, never silently
    /// reconciled** — which is why the return is a list of observations and not a boolean.
    fn drift(&self, changed_sources: &[String]) -> Result<Vec<Drift>, StepError>;
}
