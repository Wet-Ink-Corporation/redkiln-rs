//! H2 / H3 — the human gate, measured. Zero model cost.
//!
//! Registered in `spikes/METHOD-H.md`. The framing: a human gate is a
//! suspend/resume across an unbounded gap, and everything hard about it is a
//! property of that gap. F18 measured the gap — median ~10 min, but **45–48% of
//! gates exceed one hour**, p90 ~9 h, longest 9.4 days.
//!
//! **H2 — is suspension lossless?** State is serialised, re-hydrated in a
//! *separate process invocation*, and compared. Anything lost or reordered here
//! is lost on every real resume.
//!
//! **H3 — does the world move during a gate?** State is recomputed at the gate's
//! open and close. A difference is drift the resumed run must reconcile rather
//! than assume away (shipmates L2.2, L2.4).
//!
//! The two compose: H2 clean + H3 drifting means **resume must re-derive from
//! the world**, because the record is faithful but stale.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::fingerprint::sha256_hex;
use crate::reconcile::{self, CommitTrailers};

/// One historical gate pause, from telemetry.
#[derive(Debug, Clone, Serialize)]
pub struct GatePause {
    pub item: String,
    pub opened_at: chrono::DateTime<chrono::Utc>,
    pub answered_at: chrono::DateTime<chrono::Utc>,
    pub gap_minutes: f64,
    /// The gate outlived the prompt-cache TTL, so the resume pays a cold
    /// prefix (F10, F18).
    pub outlived_cache_ttl: bool,
}

/// Extract gate pauses: a `gate_result` for an item, answered by a `verdict`.
///
/// Each open is consumed by the first verdict that follows it, so a re-opened
/// gate is a second pause rather than an extension of the first.
pub fn gate_pauses(events: &[serde_json::Value]) -> Vec<GatePause> {
    let ts = |e: &serde_json::Value| -> Option<chrono::DateTime<chrono::Utc>> {
        e.get("ts")?.as_str()?.parse().ok()
    };
    let mut open: BTreeMap<String, chrono::DateTime<chrono::Utc>> = BTreeMap::new();
    let mut out = Vec::new();

    let mut sorted: Vec<&serde_json::Value> = events.iter().collect();
    sorted.sort_by_key(|e| ts(e));

    for e in sorted {
        let (Some(item), Some(t)) = (e.get("item").and_then(|i| i.as_str()), ts(e)) else {
            continue;
        };
        match e.get("event").and_then(|v| v.as_str()) {
            Some("gate_result") => {
                open.insert(item.to_string(), t);
            }
            Some("verdict") => {
                if let Some(o) = open.remove(item) {
                    let gap = (t - o).num_seconds() as f64 / 60.0;
                    if (0.0..60.0 * 24.0 * 30.0).contains(&gap) {
                        out.push(GatePause {
                            item: item.to_string(),
                            opened_at: o,
                            answered_at: t,
                            gap_minutes: gap,
                            outlived_cache_ttl: gap > 60.0,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// What the orchestrator would persist when suspending at a gate.
///
/// Deliberately the *whole* reconciled view rather than a summary: a
/// suspension that persists only what it thinks it needs is how resume loses
/// the thing nobody predicted it would need.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendedState {
    pub item: String,
    pub at: chrono::DateTime<chrono::Utc>,
    pub state: serde_json::Value,
}

impl SuspendedState {
    /// Canonical digest. `serde_json` preserves `BTreeMap` ordering, so equal
    /// states hash equal and a reordering would be caught rather than excused.
    pub fn digest(&self) -> String {
        sha256_hex(serde_json::to_string(&self.state).unwrap_or_default().as_bytes())
    }
}

/// Reconcile using only commits at or before `at` — the world as the
/// orchestrator would have seen it at that moment.
pub fn state_at(
    commits: &[CommitTrailers],
    at: chrono::DateTime<chrono::Utc>,
) -> serde_json::Value {
    let cutoff = at.timestamp();
    let visible: Vec<CommitTrailers> = commits
        .iter()
        .filter(|c| c.unix_time <= cutoff)
        .cloned()
        .collect();
    let (state, _) = reconcile::reconcile(&visible);
    serde_json::to_value(state).unwrap_or(serde_json::Value::Null)
}

/// Commits landing strictly inside the gap — H3's raw signal.
pub fn commits_in_gap<'a>(
    commits: &'a [CommitTrailers],
    pause: &GatePause,
) -> Vec<&'a CommitTrailers> {
    let (a, b) = (pause.opened_at.timestamp(), pause.answered_at.timestamp());
    commits
        .iter()
        .filter(|c| c.unix_time > a && c.unix_time <= b)
        .collect()
}

#[derive(Debug, Clone, Serialize)]
pub struct H2Result {
    pub item: String,
    /// Digest computed in this process.
    pub in_process: String,
    /// Digest computed by a **separate process invocation** after a
    /// serialise/deserialise round trip.
    pub rehydrated: String,
    pub lossless: bool,
}

/// Round-trip state through a real process boundary.
///
/// Writing to a file and reading it back in-process would prove nothing about
/// suspension; the whole risk is that a *fresh* process reconstructs something
/// different. So the child is this same binary, invoked again.
pub fn round_trip(exe: &Path, state: &SuspendedState, scratch: &Path) -> Result<H2Result> {
    std::fs::create_dir_all(scratch)?;
    let path = scratch.join(format!("suspend-{}.json", state.item.replace(['/', '\\'], "_")));
    std::fs::write(&path, serde_json::to_string(state)?)?;

    let out = std::process::Command::new(exe)
        .args(["rehydrate", "--state"])
        .arg(&path)
        .output()
        .context("spawning the rehydrate child")?;
    anyhow::ensure!(
        out.status.success(),
        "rehydrate failed: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );

    let rehydrated = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let in_process = state.digest();
    let _ = std::fs::remove_file(&path);
    Ok(H2Result {
        item: state.item.clone(),
        lossless: rehydrated == in_process,
        in_process,
        rehydrated,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(item: &str, event: &str, ts: &str) -> serde_json::Value {
        serde_json::json!({ "item": item, "event": event, "ts": ts })
    }

    #[test]
    fn a_pause_is_a_gate_result_answered_by_a_verdict() {
        let events = vec![
            ev("RK-S1", "gate_result", "2026-08-01T10:00:00Z"),
            ev("RK-S1", "verdict", "2026-08-01T11:30:00Z"),
        ];
        let p = gate_pauses(&events);
        assert_eq!(p.len(), 1);
        assert!((p[0].gap_minutes - 90.0).abs() < 0.01);
        assert!(p[0].outlived_cache_ttl, "90 min is past the ~1h TTL");
    }

    #[test]
    fn a_short_pause_is_within_the_ttl() {
        let events = vec![
            ev("RK-S1", "gate_result", "2026-08-01T10:00:00Z"),
            ev("RK-S1", "verdict", "2026-08-01T10:10:00Z"),
        ];
        assert!(!gate_pauses(&events)[0].outlived_cache_ttl);
    }

    #[test]
    fn a_reopened_gate_is_a_second_pause_not_an_extension() {
        let events = vec![
            ev("RK-S1", "gate_result", "2026-08-01T10:00:00Z"),
            ev("RK-S1", "verdict", "2026-08-01T10:10:00Z"),
            ev("RK-S1", "gate_result", "2026-08-01T12:00:00Z"),
            ev("RK-S1", "verdict", "2026-08-01T12:05:00Z"),
        ];
        let p = gate_pauses(&events);
        assert_eq!(p.len(), 2);
        assert!(p.iter().all(|x| x.gap_minutes < 15.0));
    }

    #[test]
    fn a_verdict_with_no_open_gate_is_ignored() {
        let events = vec![ev("RK-S1", "verdict", "2026-08-01T10:00:00Z")];
        assert!(gate_pauses(&events).is_empty());
    }

    #[test]
    fn events_are_ordered_before_pairing() {
        // Telemetry unions several partitions; file order is not time order.
        let events = vec![
            ev("RK-S1", "verdict", "2026-08-01T11:00:00Z"),
            ev("RK-S1", "gate_result", "2026-08-01T10:00:00Z"),
        ];
        assert_eq!(gate_pauses(&events).len(), 1);
    }

    #[test]
    fn state_at_hides_commits_from_the_future() {
        let past = reconcile::parse_body("a1", 1_000, "s", "Story: p/early\n");
        let future = reconcile::parse_body("a2", 9_000, "s", "Story: p/late\n");
        let commits = vec![past, future];
        let at = chrono::DateTime::from_timestamp(5_000, 0).unwrap();
        let s = state_at(&commits, at);
        let stories = &s["projects"]["p"]["stories_committed"];
        assert_eq!(stories.as_array().unwrap().len(), 1);
        assert_eq!(stories[0], "early");
    }

    #[test]
    fn the_digest_is_stable_across_serialisation() {
        let s = SuspendedState {
            item: "RK-S1".into(),
            at: chrono::Utc::now(),
            state: serde_json::json!({"b": 2, "a": 1}),
        };
        let round: SuspendedState =
            serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(s.digest(), round.digest());
    }
}
