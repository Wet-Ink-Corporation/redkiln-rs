//! `Retained` — ADR-0105 clause 7's stdout/stderr retention shape.
//!
//! *"a dispatch record keeps argv and environment as **digests only**, stdout and stderr
//! **verbatim up to a size cap** (then digest + head/tail), the child environment **never**."*
//!
//! **Why this type is here and not only in `rk-store`, and the design already said so.**
//! design:1699's `rk-types` row credits this crate with *"the `spoke()` predicate and
//! **raw-retention pattern** carry from `spikes/runner/src/step.rs`"* — so the home is a design
//! row and not only a consequence of the cargo constraint below. **An operator minded to reverse
//! this routing meets that row first.** It shipped at Step 6.3 inside
//! `rk-store::retention`. ADR-0099:62 homes `DispatchRecord` in `rk-types` (*"Both are
//! `rk-types`"*), and runbook `s7a` §7a.5 authors that record's minimal form here with *"the
//! ADR-0105 clause 7 retention fields `rk-store` already types"* — which `rk-types` cannot name
//! while they live one crate **downstream** of it (`rk-store` depends on `rk-types`; the reverse
//! edge is a cycle cargo refuses). Re-declaring `Retained` in this crate would manufacture a
//! second one-name-two-crates collision of exactly the kind §7a.5 is repairing for
//! `DispatchRecord`. So the type **moved** and `rk-store::retention` re-exports it: there is
//! **one** `Retained`, `rk_store::retention::Retained` still resolves to it, and
//! `rk-store::retention::DispatchRecord`'s declaration is untouched.
//!
//! **This is a routing, not a decision, and a record naming the home is OWED** — the same
//! standing as §7a.5's four other routings, and **it starts from design:1699's row** rather than
//! from the cargo constraint. It is reversible: nothing outside these two modules names the type,
//! and no checkpoint or CI assertion anywhere in `runbook/` or `ci.yml` anchors on either
//! location [measured 2026-09-13].

use crate::digest::{Digest, sha256};

/// How much stdout/stderr is kept verbatim before it becomes digest + head/tail.
pub const STDIO_CAP: usize = 64 * 1024;

/// How much of each end survives the cap.
pub const HEAD_TAIL: usize = 2 * 1024;

/// A retained stream: verbatim while small, summarised once it is not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Retained {
    Verbatim(Vec<u8>),
    Capped {
        digest: Digest,
        head: Vec<u8>,
        tail: Vec<u8>,
        total_len: usize,
    },
}

impl Retained {
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        if bytes.len() <= STDIO_CAP {
            Self::Verbatim(bytes.to_vec())
        } else {
            Self::Capped {
                digest: sha256(bytes),
                head: bytes[..HEAD_TAIL].to_vec(),
                tail: bytes[bytes.len() - HEAD_TAIL..].to_vec(),
                total_len: bytes.len(),
            }
        }
    }
}
