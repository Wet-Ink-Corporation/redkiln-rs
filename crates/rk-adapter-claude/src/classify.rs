//! Classification — every termination onto `rk_types::StepError`'s closed ELEVEN, exhaustively.
//!
//! Keyed on what was observed and never on the exit code alone (`s7c` §7c.7; ADR-0095, ADR-0112;
//! AC-229, AC-218, AC-206, AC-208, AC-523, AC-225).
//!
//! **What the runner did, and what is not lifted.** `runner/src/runner.rs` at `61f590e5…` (cited
//! by symbol; the lift header is `runner.rs`'s) classified in this order: timeout → its own
//! `Timeout`; a structured usage-limit signal → `UsageLimit { resets_at: None }` **hard-coded**;
//! a text marker → `UsageLimit { resets_at }` read from the RESULT's `resets_at`; no result →
//! `NonZero`; else `extract_digest`, whose prose-JSON fallback turned assistant text into a
//! digest. Of that, **not lifted** (§7c.2): the hard-coded `None` and the result-read
//! `resets_at` (AC-523: `resets_at` comes from the last non-`allowed` `rate_limit_event`), the
//! prose-JSON recovery (AC-208: *"no structured value is recovered by reading the assistant's
//! prose"* — the block bans the scraper shapes), and the six-variant `StepError` (`card.rs`'s
//! table). **Edited**: the text-marker HAYSTACK — the runner's `detect_usage_limit` lower-cased
//! and scanned the WHOLE result frame (`result.to_string()`); here the scan is over stderr and
//! the result's TEXT member (`text` / `result`) only, because a valid `structured_output` whose
//! own value quotes a marker (`"note": "the plan limit is documented"`) is not a usage limit — the
//! marker SET is the runner's, recorded and never tuned (Halts 10/13); the haystack is narrowed
//! and the fixture `result-ok-structured-output-quotes-marker` pins the choice (S5-12). What the
//! runner never did and this module does: consult `is_error` / `api_error_status` (AC-218 — the
//! F20 class of failure), key a cancel on the token (AC-225), refuse an exit-0 schema-void frame
//! (AC-206), and refuse a frame no `system/init` preceded (S5-01).
//!
//! **The order, stated** ([`classify`] over an [`Evidence`]):
//!
//! 1. **The race arm that fired** → `Cancelled { by }` (AC-225): `Signal` on the token arm,
//!    `Timeout` on the adapter's own timer arm (RT-3 as amended, §7c.10). Keyed on the ARM, never
//!    the exit code: an external kill with the same `CANCEL_EXIT_CODE` and neither arm fired is
//!    step 6's `NonZero` (the negative control).
//! 2. **An init-time refusal** the drain already decided → that error (`NotAdopted`,
//!    `NotIsolated { surface, diff_digest }`, or AC-226's `Transport { kind: Decode }` — `init.rs`).
//! 3. **A usage-limit result, by the structured `subtype` first** (AC-523's *"the structured
//!    `subtype` first"*): [`STRUCTURED_MARKERS`] over the frame's `subtype`, `terminal_reason`
//!    and `stop_reason` (the runner's `ResultMeta::looks_like_usage_limit`, recorded) →
//!    `RateLimited { resets_at }` with `detected_by: Subtype`; `resets_at` is the last
//!    non-`allowed` `rate_limit_event`'s `resetsAt`, else `None` so the kernel falls back to the
//!    pack delay (AC-030's degradation). **The subtype string is UNCONFIRMED** — the corpus
//!    carries `success` on all 1,907 result frames and `allowed` on all 967 readings — so the
//!    subtype arm is exercised by a SYNTHETIC frame and is PROVISIONAL, [surface unverified].
//! 4. **`is_error: true` or an `api_error_status`** → `Platform { status, retry_after }`
//!    **regardless of the exit code** (AC-218; the seven recorded `e3.jsonl` frames all exited 1
//!    and the exit-0 cell is SYNTHETIC — the exit code is not consulted to overturn it). `status`
//!    is the frame's `api_error_status`, or [`PLATFORM_STATUS_UNREPORTED`] when `is_error` is set
//!    without one; `retry_after` is the frame's `retry_after` when it carries one.
//! 5. **The disclosed text-marker fallback** ([`TEXT_MARKERS`] — the runner's
//!    `detect_usage_limit` list, **recorded, never tuned** (Halts 10/13); over stderr and the result
//!    text, lower-cased) → `RateLimited { resets_at }` with `detected_by: TextMarker` — never
//!    silent (AC-523's AND).
//! 6. **A non-zero exit, or no exit code** → `NonZero { code, stderr_digest }` — with or without
//!    a result frame; the exit code is the child's own verdict on itself when nothing above
//!    overrode it.
//! 7. **A `rate_limit_event` the scanner refused** (a third decimal, one window of two — the
//!    drain recorded it and ran on, F4 / S5-09) → `Transport { kind: Decode, detail }`: a reading
//!    the throttle cannot represent is not a reading the next dispatch may proceed blind on
//!    (§7c.8's fail-closed), and it outranks an `Ok` the same stream would otherwise have earned.
//! 8. **Exit 0 and no result frame** → `Transport { kind: Io, detail }` when the prompt could
//!    not be written to the child's stdin (the runner discarded the error; §7c.2's edit —
//!    consulted HERE and nowhere earlier, because the write runs concurrently with the child and
//!    a child that never reads stdin fails it whatever else it did: the child's frame, its
//!    markers and its exit code are its own verdict and a write symptom may not overturn them,
//!    R5-2 / F1 / S5-03; on every other path the write failure is a marker line on the retained
//!    stderr), else `Transport { kind: Decode, detail }`: the stream ended where a frame was
//!    required (a malformed line in its place, or none).
//! 9. **A result frame with no `system/init` before it** → `NotIsolated { surface: Tools,
//!    diff_digest }` over the EMPTY reported set against the declared list — adoption, isolation
//!    and the version decision were never taken, so there is no `Ok` (S5-01; the rule `init.rs`
//!    applies to an absent `tools` key). The row carries `cli_version: Pinned` (§7c.11).
//! 10. **The result frame's `structured_output`** validated against the ONE output schema
//!     (`jsonschema`, the lifted `extract_digest`'s validation) → `Ok(Typed)`; invalid →
//!     `Schema { attempts: 1, last_error }`; **absent → `Schema`** (AC-206: F20's exit-0
//!     `success` with no `structured_output` is unrepresentable as `Ok` — there is no constructor
//!     for `Ok` without a validated value, and no prose is read).
//!
//! **The two undriven arms.** `Unsupported` (the kernel's resolver's, AC-238) and `Timeout {
//! after_secs }` (no Harness path produces it under RT-3 as amended — the expired per-activity
//! timeout is the adapter's own timer arm, step 1's `Cancelled { by: Timeout }`) are produced by
//! nothing here; they appear in [`cancelled_by_of`]'s exhaustive match because ADR-0112 closes
//! the set at eleven and a `_ =>` arm would let a twelfth compile silently (AC-229's instrument:
//! every `StepError::<Variant>` name present in this file, then no `_ =>`, no `_ if`, no
//! `matches!(`). What those arms do is asserted by nothing (§7c.11).
//!
//! **`spoke()` is `rk-types`'** and is not re-implemented here (the 211-line file's *"only
//! `Schema` means the agent spoke"* was the runner's `step.rs` semantics and was wrong).

use std::collections::BTreeSet;
use std::path::Path;

use rk_types::digest::Digest;
use rk_types::error::{CancelledBy, IsolationSurface, StepError, TransportKind};
use rk_types::typed::{SchemaId, Typed};
use rk_types::usage::DetectedBy;

use crate::event::payload;
use crate::headroom::RateLimitReading;
use crate::init::set_digest;

/// The `status` a `Platform` carries when the frame set `is_error` and reported no
/// `api_error_status`: no HTTP status was reported, and 0 is the conventional "none" — never a
/// made-up 5xx.
pub const PLATFORM_STATUS_UNREPORTED: u16 = 0;

/// The structured usage-limit markers.
///
/// The runner's `looks_like_usage_limit`, over `subtype`, `terminal_reason`, `stop_reason`;
/// recorded, never tuned. **UNCONFIRMED against the CLI**: no recorded frame carries one (§7c.7).
pub const STRUCTURED_MARKERS: &[&str] = &["usage_limit", "rate_limit", "quota"];

/// The text markers (the runner's `detect_usage_limit` list, verbatim; recorded, never tuned —
/// Halts 10/13). The disclosed fallback (AC-523): a match here is `detected_by: TextMarker`.
pub const TEXT_MARKERS: &[&str] = &[
    "usage limit reached",
    "you've hit your",
    "plan limit",
    "rate limit exceeded",
];

/// The `system` subtype of guard 3's channel (b), the stream's denial event (`s8h` §8h.5; D21-47,
/// s8h review #2).
///
/// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes.)* **DRAFTED from the
/// binary, UNMEASURED as a live frame**: the 2.1.281 binary carries the literals
/// `permission_denied`, `decision_reason_code` and `outside_reads_blocked` [measured 2026-09-24 by
/// string search, `s8h` §8h.5]. The frame's shape — `{"type":"system","subtype":"permission_denied",
/// "tool_name", "tool_use_id", "tool_input", "decision_reason_code"}` — is the reviewer's reading;
/// `s8h` §8h.11's probe 0 retains the real frame, and `s8j` corrects this arm and its fixture by a
/// dated line if it differs. The arm is never widened to match text.
pub const PERMISSION_DENIED_SUBTYPE: &str = "permission_denied";

/// The segments Claude Code's `checkPathSafetyForAutoEdit` treats as sensitive, case-folded.
///
/// Copied from D21-45 (*"The path is sensitive if ANY segment is one of
/// `[".git",".vscode",".idea",".claude",".husky",".cargo",".devcontainer",".yarn",".mvn"]` or
/// `.config/git`"*, quoted from the 2.1.281 code), with that citation (`s8h` §8h.5). A CLI release
/// that changes the list is Halt 2's concern, through the pin, never this guard's.
pub const PROTECTED_SEGMENTS: &[&str] = &[
    ".git",
    ".vscode",
    ".idea",
    ".claude",
    ".husky",
    ".cargo",
    ".devcontainer",
    ".yarn",
    ".mvn",
];

/// D21-45's `.config/git`: TWO consecutive segments, `.config` then `git`, matched as a pair and
/// never as one segment name (s8h review #10). The note names it `.config/git`.
pub const PROTECTED_SEGMENT_PAIR: [&str; 2] = [".config", "git"];

/// Which of guard 3's two typed channels carried a denial (`s8h` §8h.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DenialChannel {
    /// (a) the result frame's `permission_denials` list.
    Result,
    /// (b) a `system` / [`PERMISSION_DENIED_SUBTYPE`] stream event.
    Event,
}

/// One denial, as guard 3 reads it from either channel: the tool, its use id, the path extracted
/// from its `tool_input` per tool ([`denial_path`]), and the event's `decision_reason_code`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Denial {
    pub channel: DenialChannel,
    pub tool_name: String,
    pub tool_use_id: Option<String>,
    pub path: String,
    pub decision_reason_code: Option<String>,
}

/// The typed note beside a denial under a protected segment of the spec's worktree — D21-45's
/// standing limit, named (`s8h` §8h.5). It never changes the class or the digest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedSegmentDenial {
    /// The protected segment as [`PROTECTED_SEGMENTS`] spells it, or `.config/git` for the pair.
    pub segment: String,
    pub tool_name: String,
}

impl ProtectedSegmentDenial {
    /// The warning the adapter prints for the note, naming D21-45's known limit.
    #[must_use]
    pub fn warning(&self) -> String {
        format!(
            "warning: {} was denied a path under the run worktree's protected segment {} — the \
             known limit D21-45 names (Claude Code's checkPathSafetyForAutoEdit protects it in \
             every run worktree); the dispatch fails as NotIsolated {{ surface: env }}, not a defect",
            self.tool_name, self.segment
        )
    }
}

/// Is `p` absolute in any form a denial or a deny rule carries: `/x`, `\\x`, `D:/x`, `D:\\x`?
fn is_absolute_str(p: &str) -> bool {
    let b = p.as_bytes();
    p.starts_with('/')
        || p.starts_with('\\')
        || (b.len() >= 3
            && b[0].is_ascii_alphabetic()
            && b[1] == b':'
            && (b[2] == b'/' || b[2] == b'\\'))
}

/// A path's segments, lexically normalised and case-folded (`checkPathSafetyForAutoEdit` reads
/// case-folded segments, D21-45): split on both separators, `.` and empty segments dropped, `..`
/// popping, and a drive's `D:` read as `d` so `D:/x`, `//d/x` and `/d/x` agree.
fn segments(p: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for seg in p.split(['/', '\\']) {
        match seg {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            s => {
                let folded = s.to_ascii_lowercase();
                let drive = folded
                    .strip_suffix(':')
                    .filter(|d| out.is_empty() && d.len() == 1)
                    .map(str::to_owned);
                out.push(drive.unwrap_or(folded));
            }
        }
    }
    out
}

/// The path a denial names, extracted from its `tool_input` per tool.
///
/// (`s8h` §8h.5; s8h review #10.) `file_path` for `Read`, `Edit` and `Write`; for `Glob`, its `pattern` when that is absolute, else
/// its `path` (the search root); for `Grep`, its `path`; for both, the working directory `cwd` when
/// the key is absent. Every other tool, and a missing key, is `""`.
#[must_use]
pub fn denial_path(tool_name: &str, tool_input: &serde_json::Value, cwd: Option<&Path>) -> String {
    let key = |k: &str| tool_input.get(k).and_then(serde_json::Value::as_str);
    let cwd = || cwd.map_or_else(String::new, |c| c.display().to_string());
    // an if-chain, not a `match` with a catch-all: this file carries no `_ =>` (AC-229's ban)
    if ["Read", "Edit", "Write"].contains(&tool_name) {
        key("file_path").map_or_else(String::new, str::to_owned)
    } else if tool_name == "Glob" {
        key("pattern")
            .filter(|p| is_absolute_str(p))
            .or_else(|| key("path"))
            .map_or_else(cwd, str::to_owned)
    } else if tool_name == "Grep" {
        key("path").map_or_else(cwd, str::to_owned)
    } else {
        String::new()
    }
}

/// The protected segment `path` passes through BELOW `worktree`, if any.
///
/// The note's segment, as [`PROTECTED_SEGMENTS`] spells it or `.config/git` for [`PROTECTED_SEGMENT_PAIR`]. A relative
/// path resolves against the worktree; a path outside it, or a protected segment at or above its
/// root, is `None`.
#[must_use]
pub fn protected_segment(path: &str, worktree: &Path) -> Option<String> {
    let root = segments(&worktree.display().to_string());
    let full = if is_absolute_str(path) {
        segments(path)
    } else {
        let mut joined = root.clone();
        joined.extend(segments(path));
        joined
    };
    let below = full.strip_prefix(root.as_slice())?;
    below.iter().enumerate().find_map(|(i, seg)| {
        if let Some(name) = PROTECTED_SEGMENTS.iter().find(|n| **n == seg.as_str()) {
            return Some((*name).to_owned());
        }
        (seg.as_str() == PROTECTED_SEGMENT_PAIR[0]
            && below.get(i + 1).map(String::as_str) == Some(PROTECTED_SEGMENT_PAIR[1]))
        .then(|| PROTECTED_SEGMENT_PAIR.join("/"))
    })
}

/// One denial entry's members, from a `permission_denials` element or a `permission_denied`
/// event (the same three keys).
fn denial_of(v: &serde_json::Value, channel: DenialChannel, cwd: Option<&Path>) -> Denial {
    let s = |k: &str| {
        v.get(k)
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
    };
    let tool_name = s("tool_name").unwrap_or_default();
    let path = v
        .get("tool_input")
        .map_or_else(String::new, |input| denial_path(&tool_name, input, cwd));
    Denial {
        channel,
        tool_use_id: s("tool_use_id"),
        decision_reason_code: s("decision_reason_code"),
        tool_name,
        path,
    }
}

/// Channel (a): the result payload's `permission_denials` entries.
///
/// (`[{ tool_name, tool_use_id, tool_input }]`; documented in `headless`: *"the final result message lists them in
/// `permission_denials`"*). An absent or non-list key is no entry.
#[must_use]
pub fn denials_from_result(payload: &serde_json::Value, cwd: Option<&Path>) -> Vec<Denial> {
    payload
        .get("permission_denials")
        .and_then(serde_json::Value::as_array)
        .map_or_else(Vec::new, |list| {
            list.iter()
                .map(|v| denial_of(v, DenialChannel::Result, cwd))
                .collect()
        })
}

/// Channel (b): one `system` / [`PERMISSION_DENIED_SUBTYPE`] stream line, or `None` when the line
/// is not one. Keyed by its `type` and `subtype`, never by text.
#[must_use]
pub fn denial_from_event(raw: &str, cwd: Option<&Path>) -> Option<Denial> {
    let v: serde_json::Value = serde_json::from_str(raw).ok()?;
    (v.get("type").and_then(serde_json::Value::as_str) == Some("system")
        && v.get("subtype").and_then(serde_json::Value::as_str) == Some(PERMISSION_DENIED_SUBTYPE))
    .then(|| denial_of(&v, DenialChannel::Event, cwd))
}

/// The union of both channels, de-duplicated by `tool_use_id` (the first carrier kept; a denial
/// with no id is kept as itself).
fn dedup(denials: &[Denial]) -> Vec<Denial> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    denials
        .iter()
        .filter(|d| d.tool_use_id.as_deref().is_none_or(|id| seen.insert(id)))
        .cloned()
        .collect()
}

/// The digest over the union of both channels' `(tool_name, path)` entries, de-duplicated by
/// `tool_use_id`, against the empty set (`s8h` §8h.0 Q1 (iv); §8h.5).
#[must_use]
pub fn denial_digest(denials: &[Denial]) -> Digest {
    let entries: BTreeSet<String> = dedup(denials)
        .iter()
        .map(|d| format!("{}\u{1e}{}", d.tool_name, d.path))
        .collect();
    set_digest(&entries, &BTreeSet::new())
}

/// What the runner observed, handed to the classifier as one value.
#[derive(Debug)]
pub struct Evidence<'a> {
    /// Which arm of the three-way race fired, if one did (AC-225's key).
    pub cancelled: Option<CancelledBy>,
    /// An init-time refusal the drain already decided (`init::Decision::Refuse`).
    pub decided: Option<StepError>,
    /// Whether a `system/init` line was read before the stream ended (step 9's key).
    pub init_seen: bool,
    /// The card's declared tool list — what step 9's digest compares the empty set against.
    pub declared_tools: &'a [String],
    /// A `rate_limit_event` the scanner refused, rendered with its key path (step 7).
    pub reading_refused: Option<&'a str>,
    /// The stdin write's failure, if it failed (step 8 only).
    pub stdin_error: Option<String>,
    /// The child's exit code; `None` when it was killed without one.
    pub exit_code: Option<i32>,
    /// The last `result` line's bytes, if one arrived.
    pub result_raw: Option<&'a str>,
    /// What the child wrote to stderr, as retained (the marker haystack: verbatim under the cap,
    /// head + tail past it).
    pub stderr: &'a [u8],
    /// The digest of EVERYTHING the child wrote to stderr (`NonZero`'s member — the retained
    /// form's digest, which is over the whole stream even past the cap).
    pub stderr_digest: Digest,
    /// The last `rate_limit_event` whose top-level `status` was not `allowed` (AC-523's
    /// `resets_at` source).
    pub last_non_allowed: Option<&'a RateLimitReading>,
    /// The one output schema the dispatch carried.
    pub schema_id: &'a SchemaId,
    pub schema: &'a serde_json::Value,
    /// Every `system` / [`PERMISSION_DENIED_SUBTYPE`] line the drain read, raw — guard 3's channel
    /// (b) (`s8h` §8h.5).
    pub denial_events: &'a [String],
    /// The spec's worktree: the working directory a relative denied path resolves against, and the
    /// root below which a protected segment carries the note (`s8h` §8h.5). `None` when unknown.
    pub worktree: Option<&'a Path>,
}

/// The classifier's answer: the step's outcome, how a usage limit was detected if one was, and
/// the typed notes beside it.
#[derive(Debug)]
pub struct Classified {
    pub result: Result<Typed, StepError>,
    pub detected_by: Option<DetectedBy>,
    /// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.5.)* One per
    /// denied path under a protected segment of the worktree (D21-45's known limit). The class and
    /// the digest do not move for it.
    pub notes: Vec<ProtectedSegmentDenial>,
}

fn contains_marker(hay: &str, markers: &[&str]) -> bool {
    let hay = hay.to_ascii_lowercase();
    markers.iter().any(|m| hay.contains(m))
}

/// The result text, whichever key the shape carries (the runner's `response_text`; read here
/// ONLY as a marker haystack, never as a value — AC-208).
fn result_text(payload: &serde_json::Value) -> &str {
    payload
        .get("text")
        .or_else(|| payload.get("result"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
}

/// Classify one termination (the module doc's order).
#[must_use]
pub fn classify(ev: &Evidence<'_>) -> Classified {
    let plain = |result: Result<Typed, StepError>| Classified {
        result,
        detected_by: None,
        notes: Vec::new(),
    };
    // 1. the arm that fired
    if let Some(by) = ev.cancelled {
        return plain(Err(StepError::Cancelled { by }));
    }
    // 2. an init-time refusal
    if let Some(err) = &ev.decided {
        return plain(Err(err.clone()));
    }
    let resets_at = ev.last_non_allowed.and_then(|r| r.resets_at);
    let frame: Option<serde_json::Value> =
        ev.result_raw.and_then(|raw| serde_json::from_str(raw).ok());
    let stderr_text = String::from_utf8_lossy(ev.stderr);
    if let Some(frame) = &frame {
        let p = payload(frame);
        // 3. the structured subtype first (SYNTHETIC at the pin — [surface unverified])
        let structured = ["subtype", "terminal_reason", "stop_reason"]
            .iter()
            .filter_map(|k| p.get(*k).and_then(|v| v.as_str()))
            .any(|s| contains_marker(s, STRUCTURED_MARKERS));
        if structured {
            return Classified {
                result: Err(StepError::RateLimited { resets_at }),
                detected_by: Some(DetectedBy::Subtype),
                notes: Vec::new(),
            };
        }
        // 4. is_error / api_error_status, regardless of the exit code
        let is_error = p.get("is_error").and_then(serde_json::Value::as_bool) == Some(true);
        let api_status = p
            .get("api_error_status")
            .and_then(serde_json::Value::as_u64)
            .and_then(|s| u16::try_from(s).ok());
        if is_error || api_status.is_some() {
            return plain(Err(StepError::Platform {
                status: api_status.unwrap_or(PLATFORM_STATUS_UNREPORTED),
                retry_after: p.get("retry_after").and_then(serde_json::Value::as_u64),
            }));
        }
        // 5. the disclosed text-marker fallback, over stderr and the result TEXT (the header's
        // edit: never the whole frame)
        if contains_marker(&stderr_text, TEXT_MARKERS)
            || contains_marker(result_text(p), TEXT_MARKERS)
        {
            return Classified {
                result: Err(StepError::RateLimited { resets_at }),
                detected_by: Some(DetectedBy::TextMarker),
                notes: Vec::new(),
            };
        }
    } else if contains_marker(&stderr_text, TEXT_MARKERS) {
        // 5, with no frame: the message reached stderr and the CLI died
        return Classified {
            result: Err(StepError::RateLimited { resets_at }),
            detected_by: Some(DetectedBy::TextMarker),
            notes: Vec::new(),
        };
    }
    // 5b. guard 3 (*(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.5;
    //     N17 Q1 (iv).)*): a denial on EITHER typed channel — (a) the result frame's
    //     `permission_denials`, (b) a `permission_denied` stream event — fails the dispatch as
    //     `NotIsolated { surface: Env }`, whatever the exit code, and OUTRANKS a valid
    //     `structured_output` (step 10): a child that was denied has not done what it reported.
    //     After the platform arms; an errored `tool_result` naming a denial is NOT a third detector.
    //     A denied path under a protected segment of the worktree carries D21-45's typed note; the
    //     class and the digest do not move for it.
    let mut denials = frame
        .as_ref()
        .map_or_else(Vec::new, |f| denials_from_result(payload(f), ev.worktree));
    denials.extend(
        ev.denial_events
            .iter()
            .filter_map(|raw| denial_from_event(raw, ev.worktree)),
    );
    if !denials.is_empty() {
        let denials = dedup(&denials);
        let notes = ev.worktree.map_or_else(Vec::new, |wt| {
            denials
                .iter()
                .filter_map(|d| {
                    protected_segment(&d.path, wt).map(|segment| ProtectedSegmentDenial {
                        segment,
                        tool_name: d.tool_name.clone(),
                    })
                })
                .collect()
        });
        return Classified {
            result: Err(StepError::NotIsolated {
                surface: IsolationSurface::Env,
                diff_digest: denial_digest(&denials),
            }),
            detected_by: None,
            notes,
        };
    }
    // 6. the child's own verdict
    if ev.exit_code != Some(0) {
        return plain(Err(StepError::NonZero {
            code: ev.exit_code,
            stderr_digest: ev.stderr_digest.clone(),
        }));
    }
    // 7. a reading the instrument could not represent: fail closed, whatever the frame said
    if let Some(refused) = ev.reading_refused {
        return plain(Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail: format!(
                "{refused} — a reading the throttle cannot represent is refused, never rounded or \
                 proceeded on blind (AC-374, AC-375; s7c §7c.8)"
            ),
        }));
    }
    // 8. exit 0 and no frame: the write failure if there was one, else a stream that ended short
    let Some(frame) = &frame else {
        if let Some(detail) = &ev.stdin_error {
            return plain(Err(StepError::Transport {
                kind: TransportKind::Io,
                detail: format!("the prompt could not be written to the child's stdin: {detail}"),
            }));
        }
        return plain(Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail: "the child exited 0 and the stream carried no result frame (a malformed line \
                     or none where a frame is required)"
                .to_owned(),
        }));
    };
    // 9. a frame no system/init preceded: the decisions were never taken, so no Ok (S5-01)
    if !ev.init_seen {
        let declared: BTreeSet<String> = ev.declared_tools.iter().cloned().collect();
        return plain(Err(StepError::NotIsolated {
            surface: IsolationSurface::Tools,
            diff_digest: set_digest(&BTreeSet::new(), &declared),
        }));
    }
    // 10. the structured value against the ONE schema — never the prose (AC-208)
    plain(extract_digest(payload(frame), ev.schema_id, ev.schema))
}

/// The lifted `extract_digest`'s validation half: `structured_output` against the schema.
/// Absent is `Schema` (AC-206: F20's fail-open path is unrepresentable — there is no `Ok`
/// without a validated value, and no prose is read).
fn extract_digest(
    payload: &serde_json::Value,
    schema_id: &SchemaId,
    schema: &serde_json::Value,
) -> Result<Typed, StepError> {
    let Some(candidate) = payload.get("structured_output") else {
        return Err(StepError::Schema {
            attempts: 1,
            last_error: "the result frame carried no structured_output (exit 0 and subtype \
                         success are not evidence — F20; AC-206)"
                .to_owned(),
        });
    };
    let validator = jsonschema::validator_for(schema).map_err(|e| StepError::Schema {
        attempts: 1,
        last_error: format!("the dispatch's own output schema is invalid: {e}"),
    })?;
    if let Err(e) = validator.validate(candidate) {
        return Err(StepError::Schema {
            attempts: 1,
            last_error: e.to_string(),
        });
    }
    // *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.4; D21-43 N24.)*
    // The `Spec` ingest check, AFTER the schema validation: every authored criterion carries a
    // non-blank `verification` — the type admits `None` (AC-137), the authoring does not. The class
    // is `Schema`, the one a structured output that fails its contract already takes, so ADR-0112's
    // closed set does not move. It reads the authored value only: a `Spec` read back from the store
    // is not re-checked.
    if schema_id.as_str() == SPEC_KIND
        && let Some(ac) = spec_criterion_without_verification(candidate)
    {
        return Err(StepError::Schema {
            attempts: 1,
            last_error: format!(
                "the Spec's criterion {ac} carries no authored verification (absent or blank): \
                 every criterion a specifying activity emits states how it is shown to hold \
                 (s8h §8h.4; D21-43 N24)"
            ),
        });
    }
    Ok(Typed::new(schema_id.clone(), candidate.clone()))
}

/// The kind name a `Spec` structured output is bound under (`rk_types::registry::EXPORTED`'s key,
/// the schema id `rk-cli`'s binder gives an agent step whose output is `value:Spec`).
pub const SPEC_KIND: &str = "Spec";

/// The first criterion of a `Spec` value whose `verification` is absent, `null`, not a string or
/// blank — named by its `id`, or its position when it carries none.
fn spec_criterion_without_verification(spec: &serde_json::Value) -> Option<String> {
    let list = spec
        .get("acceptance_criteria")
        .and_then(serde_json::Value::as_array)?;
    list.iter().enumerate().find_map(|(i, ac)| {
        let authored = ac
            .get("verification")
            .and_then(serde_json::Value::as_str)
            .is_some_and(|v| !v.trim().is_empty());
        (!authored).then(|| {
            ac.get("id")
                .and_then(serde_json::Value::as_str)
                .map_or_else(|| format!("#{i}"), str::to_owned)
        })
    })
}

/// The cancel cause the row carries — `Some` only on the `Cancelled { by }` path.
///
/// (AC-225; ADR-0126 decision 1.) An EXHAUSTIVE match over the closed eleven with no catch-all
/// (AC-229's instrument): a twelfth variant is E0004 here.
#[must_use]
pub const fn cancelled_by_of(err: &StepError) -> Option<CancelledBy> {
    match err {
        StepError::Cancelled { by } => Some(*by),
        StepError::Transport { .. }
        | StepError::Schema { .. }
        | StepError::Timeout { .. }
        | StepError::NonZero { .. }
        | StepError::RateLimited { .. }
        | StepError::NotAdopted { .. }
        | StepError::NotIsolated { .. }
        | StepError::Platform { .. }
        | StepError::Unsupported { .. }
        | StepError::Overage { .. } => None,
    }
}
