//! `ActivitySpec` and `DispatchRecord` — the two halves of one dispatch, homed by the record.
//!
//! ADR-0099 clause 1's last sentence, quoted in full (`0099`:61-62): *"`ActivitySpec` carries
//! `StepSpec`'s fields (card digest, tools, schema id, timeout, inputs); `DispatchRecord` carries
//! `StepRecord`'s. **Both are `rk-types`.**"* This is the one routing in Step 7a that a record
//! **decides** rather than the stage routing. `StepSpec` and `StepRecord` are the retired runner's
//! names, cited as field provenance only; they are not types on this tree.
//!
//! **`DispatchRecord` is completed here, at Step 7c Slice 1, by ADR-0126.** At Step 7a it was five
//! fields and this header said so in terms — *"incomplete by the record … completing the field set
//! is owed at 7c"* — because ADR-0099:58 / AC-360 also required usage and timings and the `Usage`
//! type existed in no crate. ADR-0126 decision 1 names the members (its payload table is the
//! member list), decision 2 homes the types in `rk-types` (*"a field of an `rk-types` struct has an
//! `rk-types` type, because `rk-types` depends on nothing internal"*), and decision 5 lands the
//! variant, the `ALL` count, the pin test and these members in one change, so the row and the
//! record it carries are never on the tree one without the other. The row is
//! [`crate::event_kind::EventKind::DispatchCompleted`]; its payload is this record, serialised —
//! which is why the record and every member type now derive `Serialize` + `Deserialize`
//! (`Event.payload` is a `serde_json::Value`).
//!
//! **The second `DispatchRecord` is gone.** `rk-store::retention::DispatchRecord` — the collision
//! `s7a` §7a.5 named — collapses onto this type in the same change (ADR-0126 decision 2: *"one
//! type, one declaration"*); `rk_store::retention` re-exports it as it already re-exported
//! `Retained`. The constructor came with it: an inherent `impl` on a foreign type is E0116, so
//! [`DispatchRecord::new`] lives here, and it takes the completed record's tail as one value,
//! [`RecordTail`], because a fifteen-argument constructor is a call site nobody can read
//! (`s7c` §7c.19 spells the shape).
//!
//! **No `result` member** (`s7c` §7c.11): the row is the completed record and nothing else. The
//! step's outcome is the other half of `DispatchOutcome { record, result }` (ADR-0099 decision 1)
//! and stays outside the record — `StepError` and `Typed` carry no `Serialize`, and `Typed` is
//! `PartialEq` without `Eq`, so they could not travel under this record's derives even if a record
//! admitted them.

use serde::{Deserialize, Serialize};

use crate::digest::{Digest, sha256};
use crate::error::CancelledBy;
use crate::retention::Retained;
use crate::typed::SchemaId;
use crate::usage::{CliVersion, CostUsd, DetectedBy, Timings, Usage, WindowReading};

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

/// From `StepRecord`, completed by ADR-0126: the record half of `DispatchOutcome`, and the payload
/// of the `dispatch_completed` row.
///
/// **`argv` and `env` are digests only** — ADR-0105 clause 7 — and there is no field to put either
/// verbatim in. That is not pedantry: a field plus a rule to leave it empty is a field someone
/// eventually fills, and the child environment is where credentials live.
///
/// **Absent is `None`, never `0`** (AC-362): `usage` and every usage-adjacent measurement are
/// `Option`, none of their types has a `Default`, and the pre-spawn refusals (AC-221, AC-376)
/// write a row with every one of them `None` and `cli_version: Pinned`. What is **not** optional
/// is what every path has: the digests, the pin, the retained streams, the timings, the two-armed
/// version (AC-227), the retry count (AC-232 — a count of zero is a measurement) and the two TTL
/// values (AC-210).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispatchRecord {
    pub argv_digest: Digest,
    pub env_digest: Digest,
    /// **The pinned harness/CLI version** AC-360's THEN names — the `Config` pin. The observed one
    /// is [`Self::cli_version`]; two members, two questions (`s7c` §7c.11).
    pub instrument_version: String,
    pub stdout: Retained,
    pub stderr: Retained,
    /// *"Every token counter the harness reports verbatim"* (AC-360), nested object included
    /// (ADR-0113). `None` on a path that read no result frame.
    pub usage: Option<Usage>,
    pub timings: Timings,
    /// The decimal digits as printed, never a float (RT-19). `None` when no result frame carried
    /// one.
    pub total_cost_usd: Option<CostUsd>,
    /// The `five_hour` / `seven_day` reading taken **before** the dispatch (AC-360). Readings, no
    /// delta (AC-373).
    pub headroom_before: Option<WindowReading>,
    /// The reading taken **after**.
    pub headroom_after: Option<WindowReading>,
    /// `Observed(v)` on every path that read the child's `system/init`; `Pinned(pin)` on every
    /// path that terminated before one was read. Non-optional on both arms (AC-227; AC-385).
    pub cli_version: CliVersion,
    /// The count of `system/api_retry` events observed in this dispatch's stream — ADR-0122
    /// decision 1's unit, one charge per event; a count and not a list, because the attempt has
    /// no kind (ADR-0122 decision 6) and ADR-0126 mints none for it.
    pub api_retry_events: u32,
    /// `CLAUDE_CODE_PROMPT_CACHE_TTL` **as set on the child** — the first of the two TTL fields,
    /// both always present (AC-210's THEN: *"both values are recorded on the dispatch event"*;
    /// AC-526 refuses a one-set or mismatched pair before spawn, so a record never carries a
    /// half). **Two members and no wrapper type**, decided at Step 7c Slice 1 (2026-09-17): `s7c`
    /// §7c.11 says *"the two TTL fields (non-optional — AC-210)"* and lists every new type this
    /// record gains *"so Slice 1 declares nothing by guess"* — seven, all in `usage.rs`, none of
    /// them a TTL pair — and §7c.19 says *"the two TTL strings … are required members"*; a
    /// `CacheTtl` struct would be an eighth type no line of the record names. §7c.19's `RecordTail`
    /// spelling writes the pair as one `ttl` name, which conflicts with its own sentence and is
    /// reported as the stage-file defect. Named after the variable minus the `CLAUDE_CODE_`
    /// prefix: the row is evidence for ADR-0078's rule that no run is reported as 1 h without the
    /// setting *and* the per-run `ephemeral_*` evidence, and a reader joining the two needs the
    /// setting by name.
    pub prompt_cache_ttl: String,
    /// `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` as set on the child — the second TTL field.
    pub subagent_prompt_cache_ttl: String,
    /// The cancel cause, `Some` only on the `Cancelled { by }` path (AC-225).
    pub cancelled_by: Option<CancelledBy>,
    /// How a usage-limit result was recognised, `Some` only on the `RateLimited` path (AC-523).
    pub detected_by: Option<DetectedBy>,
}

/// Everything [`DispatchRecord::new`] cannot derive from the raw material: the members the adapter
/// measured or decided, handed over as one value so the call site names each by field.
///
/// The shape `s7c` §7c.19 spells — *"`RecordTail { cli_version, api_retry_events, ttl, timings,
/// usage, total_cost_usd, headroom_before, headroom_after, cancelled_by, detected_by }`"* — with
/// the same optionality as the record: what is required here is required there. The one departure
/// is the TTL pair, which is two members here as it is on the record (see
/// [`DispatchRecord::prompt_cache_ttl`]) and not the single `ttl` that spelling writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTail {
    pub cli_version: CliVersion,
    pub api_retry_events: u32,
    pub prompt_cache_ttl: String,
    pub subagent_prompt_cache_ttl: String,
    pub timings: Timings,
    pub usage: Option<Usage>,
    pub total_cost_usd: Option<CostUsd>,
    pub headroom_before: Option<WindowReading>,
    pub headroom_after: Option<WindowReading>,
    pub cancelled_by: Option<CancelledBy>,
    pub detected_by: Option<DetectedBy>,
}

impl DispatchRecord {
    /// Build a record from the raw material, digesting what must be digested.
    ///
    /// Moved from `rk-store::retention` at Step 7c Slice 1 with its body unchanged and the tail
    /// added. `env` is **borrowed, digested and never stored** — the record has no member to put
    /// it in (ADR-0105 clause 7), which is the property; the sentence this replaced said *"taken by
    /// value and dropped"*, which the `&[(String, String)]` signature never provided and which read
    /// as a consumption the caller could rely on (corrected 2026-09-17). Reading it at all is
    /// deliberate — the digest is what lets `doctor` notice that two dispatches ran under
    /// different environments without ever retaining either. The environment is sorted before it
    /// is digested, so a map's iteration order is not part of the identity.
    #[must_use]
    pub fn new(
        argv: &[String],
        env: &[(String, String)],
        instrument_version: &str,
        stdout: &[u8],
        stderr: &[u8],
        tail: RecordTail,
    ) -> Self {
        Self::with_retained(
            argv,
            env,
            instrument_version,
            Retained::of(stdout),
            Retained::of(stderr),
            tail,
        )
    }

    /// [`DispatchRecord::new`] with the two streams ALREADY shaped as [`Retained`] — the form an
    /// adapter that accumulated them in bounded memory (`crate::retention::RetainedSink`) hands
    /// in, so the whole stream never has to exist in one buffer for the record to be built.
    /// `new` delegates here with `Retained::of` over its slices. Added 2026-09-18 (Step 7c Slice
    /// 5, the runtime review's memory bound).
    #[must_use]
    pub fn with_retained(
        argv: &[String],
        env: &[(String, String)],
        instrument_version: &str,
        stdout: Retained,
        stderr: Retained,
        tail: RecordTail,
    ) -> Self {
        let argv_joined = argv.join("\u{1f}");
        let mut env_sorted: Vec<String> = env.iter().map(|(k, v)| format!("{k}={v}")).collect();
        env_sorted.sort();
        let RecordTail {
            cli_version,
            api_retry_events,
            prompt_cache_ttl,
            subagent_prompt_cache_ttl,
            timings,
            usage,
            total_cost_usd,
            headroom_before,
            headroom_after,
            cancelled_by,
            detected_by,
        } = tail;
        Self {
            argv_digest: sha256(argv_joined.as_bytes()),
            env_digest: sha256(env_sorted.join("\u{1f}").as_bytes()),
            instrument_version: instrument_version.to_string(),
            stdout,
            stderr,
            usage,
            timings,
            total_cost_usd,
            headroom_before,
            headroom_after,
            cli_version,
            api_retry_events,
            prompt_cache_ttl,
            subagent_prompt_cache_ttl,
            cancelled_by,
            detected_by,
        }
    }
}
