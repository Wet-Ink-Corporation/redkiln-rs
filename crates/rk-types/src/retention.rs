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
//!
//! **Serialised as the derived `Vec<u8>` form — a JSON array of integers** (Step 7c Slice 1;
//! `s7c` §7c.11), because this type is a member of the `dispatch_completed` row's payload
//! (ADR-0126) and `Event.payload` is a `serde_json::Value`. The integer array is byte-lossless,
//! which a UTF-8-lossy text is not and *"verbatim"* (AC-200) has to survive the round trip; it
//! needs no encoder (no `base64` crate is in the workspace, and a new 0.x edge would need an
//! exact-pinned row under ADR-0100 decision 2 that the stage does not add); and `rk-store`'s
//! canonical serialiser accepts integer arrays — it refuses only floats.

use serde::{Deserialize, Serialize};

use crate::digest::{Digest, Sha256Stream, sha256};

/// How much stdout/stderr is kept verbatim before it becomes digest + head/tail.
pub const STDIO_CAP: usize = 64 * 1024;

/// How much of each end survives the cap.
pub const HEAD_TAIL: usize = 2 * 1024;

/// A retained stream: verbatim while small, summarised once it is not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

/// [`Retained`], accumulated from a stream in BOUNDED memory.
///
/// `Retained::of` over bytes that arrive in pieces, without holding them all. Verbatim while the
/// total is at or under
/// [`STDIO_CAP`]; past it the verbatim buffer is dropped and only the running digest, the first
/// [`HEAD_TAIL`] bytes, the last [`HEAD_TAIL`] bytes and the count are kept — so a child that
/// spews grows the reader by at most `STDIO_CAP + 2 × HEAD_TAIL` plus a hasher, whatever it
/// writes. `finish()` is byte-equal to `Retained::of` over the concatenation of every `push`
/// (`retained_sink_equals_retained_of` in `tests/it/dispatch_record.rs`). Added 2026-09-18 (Step
/// 7c Slice 5, the runtime review's memory bound: the adapter's drain accumulated every byte
/// before this type capped it).
#[derive(Debug, Clone)]
pub struct RetainedSink {
    /// The whole stream, kept only while it is at or under the cap.
    verbatim: Option<Vec<u8>>,
    head: Vec<u8>,
    tail: Vec<u8>,
    total_len: usize,
    digest: Sha256Stream,
}

impl RetainedSink {
    /// Nothing pushed yet (named so, not `Default`: an empty retention is a value a reader could
    /// mistake for "the child wrote nothing" — the caller decides whether it did).
    #[must_use]
    pub fn empty() -> Self {
        Self {
            verbatim: Some(Vec::new()),
            head: Vec::new(),
            tail: Vec::new(),
            total_len: 0,
            digest: Sha256Stream::start(),
        }
    }

    /// Append the next piece.
    pub fn push(&mut self, bytes: &[u8]) {
        self.digest.update(bytes);
        self.total_len += bytes.len();
        let head_room = HEAD_TAIL.saturating_sub(self.head.len());
        self.head
            .extend_from_slice(&bytes[..head_room.min(bytes.len())]);
        self.tail.extend_from_slice(bytes);
        if self.tail.len() > HEAD_TAIL {
            let drop = self.tail.len() - HEAD_TAIL;
            self.tail.drain(..drop);
        }
        if let Some(v) = &mut self.verbatim {
            if self.total_len <= STDIO_CAP {
                v.extend_from_slice(bytes);
            } else {
                self.verbatim = None;
            }
        }
    }

    /// How many bytes have been pushed.
    #[must_use]
    pub const fn total_len(&self) -> usize {
        self.total_len
    }

    /// The retention, exactly as `Retained::of` would have shaped the concatenation.
    #[must_use]
    pub fn finish(self) -> Retained {
        match self.verbatim {
            Some(v) => Retained::Verbatim(v),
            None => Retained::Capped {
                digest: self.digest.finish(),
                head: self.head,
                tail: self.tail,
                total_len: self.total_len,
            },
        }
    }
}
