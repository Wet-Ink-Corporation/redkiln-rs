//! `CancelToken` — the composition root's, never the kernel's.
//!
//! ADR-0099 clause 1, restated as design §3.3 P10: the adapter's declared per-activity timeout
//! (AC-223) is the **in-band** cancel, and out-of-band cancellation is a token **constructed at
//! the composition root**, cloned into the adapter at construction, and fired only by the CLI's
//! signal path (Ctrl-C / SIGTERM) or the per-activity timeout. **The kernel never calls it and
//! never holds it.**
//!
//! Do not start from ADR-0076's *"a handle minted when the operation started"*: ADR-0099
//! supersedes ADR-0076 in part on exactly that clause and rejects the `start() -> Handle` /
//! `wait(Handle)` shape (ADR-0099:95-96). Nothing in Step 7a's instrument set catches the
//! withdrawn shape — neither `compile_fail` fence is about a cancel handle, and the fence count
//! passes a crate carrying it. It is caught by reading this against ADR-0099 clause 1.
//!
//! Only the `Harness` port carries cancellation in Phase 1; the two Toolchain adapters run
//! bounded commands under the same timeout without a token.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// A one-way flag, cloned into an adapter at construction and shared with the root that owns it.
#[derive(Clone, Debug, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Fired by the composition root's signal path or the per-activity timeout. There is
    /// deliberately no way to un-fire it.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}
