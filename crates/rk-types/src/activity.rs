//! `ActivitySpec` and `DispatchRecord` — the two halves of one dispatch, homed by the record.
//!
//! ADR-0099 clause 1's last sentence, quoted in full (`0099`:61-62): *"`ActivitySpec` carries
//! `StepSpec`'s fields (card digest, tools, schema id, timeout, inputs); `DispatchRecord` carries
//! `StepRecord`'s. **Both are `rk-types`.**"* This is the one routing in Step 7a that a record
//! **decides** rather than the stage routing. `StepSpec` and `StepRecord` are the retired runner's
//! names, cited as field provenance only; they are not types on this tree.
//!
//! **`DispatchRecord` is incomplete by the record, and that is written down rather than smoothed**
//! (runbook `s7a` §7a.5): ADR-0099:58 / AC-360 also require **usage** and **timings**, and the
//! `Usage` type exists in no crate — so no field carries them here, and completing the field set
//! is owed at 7c where the first real adapter constructs one. What is authored here is the
//! minimum `Harness::dispatch` needs to be declarable.
//!
//! **A second `DispatchRecord` exists on this tree**, `rk-store::retention::DispatchRecord`, in a
//! crate ADR-0099:62 does not assign it to. Collapsing the two is 7c's; Checkpoint 7a asserts only
//! that `rk-ports` declares neither.

use crate::digest::Digest;
use crate::retention::Retained;
use crate::typed::SchemaId;

/// From `StepSpec`: what the adapter needs to assemble one dispatch.
///
/// ADR-0099:61's five fields and no sixth.
#[derive(Debug, Clone, PartialEq)]
pub struct ActivitySpec {
    /// The card's content digest. A digest, never the card body.
    pub card_digest: Digest,
    /// The card's tool allowlist, as declared.
    pub tools: Vec<String>,
    /// The schema the result must validate against.
    pub schema: SchemaId,
    /// The adapter's declared per-activity timeout (AC-223) — the in-band cancel of ADR-0099
    /// clause 1.
    pub timeout_secs: u64,
    /// The activity's inputs, as assembled.
    pub inputs: Vec<(String, serde_json::Value)>,
}

/// From `StepRecord`: the record half of `DispatchOutcome`.
///
/// **`argv` and `env` are digests only** — ADR-0105 clause 7 — and there is no field to put either
/// verbatim in. That is not pedantry: a field plus a rule to leave it empty is a field someone
/// eventually fills, and the child environment is where credentials live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchRecord {
    pub argv_digest: Digest,
    pub env_digest: Digest,
    pub instrument_version: String,
    pub stdout: Retained,
    pub stderr: Retained,
}
