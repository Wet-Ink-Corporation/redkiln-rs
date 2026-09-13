//! The step memo key — ADR-0097 decision 2, and the two keys it must not be confused with.
//!
//! The record is exact: *"The step memo key is `SHA-256` over a length-prefixed, ordered tuple of
//! **exactly these components, in this order**, each encoded as `u32-length || bytes`."* So the
//! components are a **struct with eight named fields** rather than a `Vec`: "exactly eight, in this
//! order" becomes a type fact, and a ninth component or a reordering is a compile error rather than
//! a silently different key.
//!
//! **Three different keys, three different formulas.** ADR-0097 says so in terms, and design §3.5
//! was corrected so its two paragraphs agree:
//!
//! | Key | Formula | Where |
//! | --- | --- | --- |
//! | step memo | SHA-256 over the eight-component length-prefixed tuple | [`StepKeyComponents`] |
//! | activity-level (AC-016) | SHA-256 over the activity's reachable **card digests**, sorted | [`activity_key`] |
//! | idempotency | `(item, activity_id@version, attempt)` — **three parts, never a fourth** | [`IdempotencyKey`] |
//!
//! They deliberately do not share code. A helper that computed "the key" would be the thing that
//! makes two of them accidentally equal, and AC-014/AC-016/AC-019 exist because that distinction is
//! load-bearing.
//!
//! **The memo stays tier 2** (ADR-0097 decision 1): rebuildable by re-execution, never by rebuild.
//! `doctor --rebuild` reproduces the *stage-level* next action from git alone; step outputs and
//! memo hits are recomputed.

use rk_types::digest::{Digest, sha256_lp};

/// ADR-0097 decision 2's eight components, in the record's order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StepKeyComponents {
    /// 1. `activity_id@version` (ADR-0072's version).
    pub activity_id_at_version: String,
    /// 2. The rigor variant selected.
    pub rigor: String,
    /// 3. The step id within the activity graph.
    pub step_id: String,
    /// 4. The digest of each resolved step input, **in declaration order**, each canonicalised by
    ///    `rk-store`'s canonical serialiser.
    pub input_digests: Vec<Digest>,
    /// 5. The card digest — or, for a `tool` step, the Toolchain adapter's detected version string
    ///    plus the command.
    pub card_or_tool: String,
    /// 6. The digests of the resolved KB atoms, **sorted**.
    pub kb_atom_digests: Vec<Digest>,
    /// 7. The loop iteration counter (0 outside a loop — ADR-0021's rule, unchanged).
    pub loop_iteration: u32,
    /// 8. The output type's `schema_version`.
    pub schema_version: String,
}

impl StepKeyComponents {
    /// The memo key.
    ///
    /// Components 4 and 6 are each folded into a single component by hashing their own
    /// length-prefixed sequence first, so a list's internal boundaries cannot absorb the outer
    /// tuple's — the same reason the outer framing exists.
    ///
    /// Component 6's atoms are **sorted** here rather than trusted to arrive sorted: the record says
    /// sorted, and a caller that passed them in resolution order would otherwise produce a
    /// different key for the same set.
    #[must_use]
    pub fn digest(&self) -> Digest {
        let inputs = fold(&self.input_digests);
        let mut atoms: Vec<&Digest> = self.kb_atom_digests.iter().collect();
        atoms.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
        let atoms = fold_refs(&atoms);
        let iteration = self.loop_iteration.to_be_bytes();

        sha256_lp(&[
            self.activity_id_at_version.as_bytes(),
            self.rigor.as_bytes(),
            self.step_id.as_bytes(),
            inputs.as_str().as_bytes(),
            self.card_or_tool.as_bytes(),
            atoms.as_str().as_bytes(),
            &iteration,
            self.schema_version.as_bytes(),
        ])
    }

    /// How many components the record fixes. Asserted by a test against the field count, so the
    /// "exactly eight" claim cannot drift from the struct.
    pub const COMPONENT_COUNT: usize = 8;
}

fn fold(ds: &[Digest]) -> Digest {
    let refs: Vec<&Digest> = ds.iter().collect();
    fold_refs(&refs)
}

fn fold_refs(ds: &[&Digest]) -> Digest {
    let parts: Vec<&[u8]> = ds.iter().map(|d| d.as_str().as_bytes()).collect();
    sha256_lp(&parts)
}

/// **A different formula** (AC-016): SHA-256 over the activity's reachable card digests, sorted.
///
/// Not the step key with fewer components — a separate derivation, kept separate so the two cannot
/// accidentally coincide.
#[must_use]
pub fn activity_key(reachable_card_digests: &[Digest]) -> Digest {
    let mut sorted: Vec<&Digest> = reachable_card_digests.iter().collect();
    sorted.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
    fold_refs(&sorted)
}

/// The idempotency key: **three parts, in the schema as everywhere else.**
///
/// ADR-0097's forbids clause names the failure directly — *"a fourth part on the idempotency key"*.
/// A struct with three fields is how that stays true.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IdempotencyKey {
    pub item: String,
    pub activity_id_at_version: String,
    pub attempt: u32,
}

impl IdempotencyKey {
    pub const PART_COUNT: usize = 3;

    #[must_use]
    pub fn digest(&self) -> Digest {
        sha256_lp(&[
            self.item.as_bytes(),
            self.activity_id_at_version.as_bytes(),
            &self.attempt.to_be_bytes(),
        ])
    }
}

/// Why a memo entry was dropped. Design §5's `memo_evicted` payload names this reason explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvictionReason {
    UnsoundReplay,
}

impl EvictionReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnsoundReplay => "unsound_replay",
        }
    }
}
