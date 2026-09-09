//! P8 — a gate inside a graph (METHOD-P §5). ~30 haiku dispatches.
//!
//! Schema §3 resumes *into the same step* with `${gate.answer}` bound; §9b.15
//! makes `answer` the gate's only output; P9b's A8 assumes a re-reached gate is
//! a new mark. F19 proved lossless resume at ACTIVITY granularity and F28
//! priced a stage-level gate at a flat ~18.6k tokens. None of that was measured
//! with **partial step outputs bound mid-graph**, which is the case §3 actually
//! commits to.
//!
//! The kill is real. Writing state to a file and reading it back in-process
//! would prove nothing — the whole risk is that a *fresh* process reconstructs
//! something different, which is why `human_gate::round_trip` crosses a genuine
//! process boundary and why this driver does too: `--phase run` blocks at the
//! gate holding its state in memory, an external supervisor kills it, and
//! `--phase resume` is a new process that must rebuild everything from disk.
//!
//! A note on citation: the AFK brief says "SIGTERM (F21 semantics)". F21 in
//! `FINDINGS.md` is the E3 arm (b) detection result, not process semantics —
//! the suspension work is F19/F22 and `human_gate.rs` (DECISIONS-2026-09-09
//! #24). Windows has no SIGTERM; `taskkill` without `/F` posts WM_CLOSE and
//! with `/F` terminates. Both are recorded, because "the process was asked to
//! stop" and "the process was destroyed" are different experiments.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::fingerprint::sha256_hex;

/// One completed step's output, with the digest resume is checked against.
///
/// METHOD-P §5.3: "any digest changed = lost". The digest is over the canonical
/// JSON of the output, so a reordering is caught rather than excused.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepOutput {
    pub step: String,
    pub pass: u32,
    pub output: serde_json::Value,
    pub digest: String,
    pub tokens: u64,
    pub cost_usd: f64,
}

impl StepOutput {
    pub fn new(step: &str, pass: u32, output: serde_json::Value, tokens: u64, cost_usd: f64) -> Self {
        let digest = sha256_hex(
            serde_json::to_string(&output)
                .unwrap_or_default()
                .as_bytes(),
        );
        Self {
            step: step.to_string(),
            pass,
            output,
            digest,
            tokens,
            cost_usd,
        }
    }
}

/// Everything a resume needs, and deliberately more.
///
/// `human_gate.rs` records the lesson this follows: "a suspension that persists
/// only what it thinks it needs is how resume loses the thing nobody predicted
/// it would need." So the whole step history is kept, not just the last output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphState {
    pub item: String,
    pub arm: String,
    pub repeat: u32,
    pub activity: String,
    pub graph_sha256: String,
    pub corpus_sha: String,
    pub fixture: String,
    /// Every step output completed before the gate, in order.
    pub steps: Vec<StepOutput>,
    /// How many times the gate has been reached. P9b's A8 says a re-reached
    /// gate is a NEW mark; §5.3 asks whether that is true.
    pub gate_arrivals: u32,
    /// #37B: whether THIS suspension was the gate's own `when` predicate or the
    /// driver's `--force-gate`. Carried in the state so `resume` reports what
    /// the run actually did rather than what its own invocation was told.
    #[serde(default)]
    pub gate_forced: bool,
    pub marked_at: chrono::DateTime<chrono::Utc>,
    /// Tokens spent before the kill, so resume cost can be separated from
    /// total cost rather than inferred.
    pub tokens_before_gate: u64,
    pub cost_before_gate: f64,
}

impl GraphState {
    /// The manifest resume is checked against: one digest per step output.
    pub fn manifest(&self) -> Vec<(String, String)> {
        self.steps
            .iter()
            .map(|s| (format!("{}#{}", s.step, s.pass), s.digest.clone()))
            .collect()
    }

    /// Digest of the whole state, for a single-value before/after comparison.
    pub fn digest(&self) -> String {
        sha256_hex(
            serde_json::to_string(&self.steps)
                .unwrap_or_default()
                .as_bytes(),
        )
    }

    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        if let Some(p) = path.parent() {
            std::fs::create_dir_all(p)?;
        }
        std::fs::write(path, serde_json::to_string_pretty(self)?)
            .with_context(|| format!("writing state {}", path.display()))?;
        Ok(())
    }

    pub fn load(path: &std::path::Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading state {} — was the gate ever reached?", path.display()))?;
        Ok(serde_json::from_str(&text)?)
    }
}

/// The human answer, written by `--phase answer` and read by `--phase resume`.
///
/// §9b.15: `answer` is the gate's ONLY output. Free-text answers are forbidden,
/// so this is a closed option id and nothing else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateAnswer {
    pub answer: String,
    pub answered_at: chrono::DateTime<chrono::Utc>,
    pub question: String,
}

/// What the before/after comparison found. This is P8's primary result.
#[derive(Debug, Clone, Serialize)]
pub struct ResumeCheck {
    pub steps_before: usize,
    pub steps_after: usize,
    /// Step keys whose digest differs across the kill. **Any entry here fires
    /// METHOD-P §5.4's first kill criterion**, on a single instance — this is
    /// the one place the "nothing from a single draw" rule does not soften a
    /// result, because a lossy resume is a property of the mechanism.
    pub changed: Vec<String>,
    /// Step keys present before the kill and absent after.
    pub lost: Vec<String>,
    pub state_digest_before: String,
    pub state_digest_after: String,
    pub lossless: bool,
}

pub fn check_resume(before: &GraphState, after: &GraphState) -> ResumeCheck {
    let b: std::collections::BTreeMap<String, String> = before.manifest().into_iter().collect();
    let a: std::collections::BTreeMap<String, String> = after.manifest().into_iter().collect();
    let mut changed = Vec::new();
    let mut lost = Vec::new();
    for (k, v) in &b {
        match a.get(k) {
            None => lost.push(k.clone()),
            Some(av) if av != v => changed.push(k.clone()),
            _ => {}
        }
    }
    ResumeCheck {
        steps_before: b.len(),
        steps_after: a.len(),
        lossless: changed.is_empty() && lost.is_empty(),
        changed,
        lost,
        state_digest_before: before.digest(),
        state_digest_after: after.digest(),
    }
}

/// The `attempt` step's output.
pub fn attempt_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["status", "steps_tried", "observed", "blocker"],
        "properties": {
            "status": {
                "type": "string",
                "description": "`reproduced` only if you actually observed the failure. `not-reproduced` otherwise — which is a legitimate answer.",
                "enum": ["reproduced", "not-reproduced"]
            },
            "steps_tried": {
                "type": "array",
                "description": "What you actually ran, in order.",
                "items": { "type": "string" }
            },
            "observed": {
                "type": "string",
                "description": "What happened when you ran them."
            },
            "blocker": {
                "type": "string",
                "description": "If not reproduced: the single piece of information you would need. Empty string if reproduced."
            }
        }
    })
}

/// The `reduce` step's output — the `ReproCase` the activity provides.
pub fn reduce_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["command", "expected", "actual", "files"],
        "properties": {
            "command": { "type": "string", "description": "The smallest command that shows the failure." },
            "expected": { "type": "string" },
            "actual": { "type": "string" },
            "files": {
                "type": "array",
                "description": "Repo-relative paths the repro touches.",
                "items": { "type": "string" }
            }
        }
    })
}

/// The investigator card for `attempt` and `reduce`.
pub const REPRO_CARD: &str = "You are reproducing a reported fault in this repository, then reducing it to \
the smallest case that still shows it.

Run things. Read code when you need to. Report what you actually observed, not what you expect would happen.

If you cannot reproduce it, say `not-reproduced` and name the ONE piece of information that would unblock \
you. Do not guess at a reproduction you did not see — an honest `not-reproduced` is the useful answer, and \
a fabricated one is worse than nothing.";

/// The `attempt` brief.
///
/// P8's fixture is METHOD-P §5.2's: "3 E3 items with the repro steps
/// deliberately removed so the gate fires". The removal happens here, in the
/// prompt, rather than by editing the corpus — the corpus is frozen and is
/// never written to (AFK brief rule 6).
pub fn build_attempt_prompt(slug: &str, spec_without_repro: &str, answer: Option<&str>) -> String {
    let mut p = format!(
        "Reproduce the fault described below in this repository.\n\n\
         Return the structured record defined by the output schema. Set `status` to `reproduced` only if \
         you observed the failure yourself.\n\n\
         ---- ITEM: {slug} ----\n\n{spec_without_repro}\n\n---- END ITEM ----\n"
    );
    if let Some(a) = answer {
        // §3: resume binds `${gate.answer}` into the SAME step. This is the
        // whole mechanism P8 is testing, so the binding is explicit and
        // recorded rather than folded into the prose.
        p.push_str(&format!(
            "\n## Answer to your escalation (${{gate.answer}})\n\n{a}\n\n\
             Use it and try again.\n"
        ));
    }
    p
}

/// Strip the sections that would let the model reproduce without asking.
///
/// #28, measured: removing only the "Steps to reproduce" heading was NOT enough.
/// The control run reproduced `fix-advance-write-lock-gate-scope` anyway in one
/// attempt (754 s, haiku) because the spec's acceptance criteria and behaviour
/// sections name the files and the expected behaviour outright — between them
/// and the repository, the steps are recoverable. The gate never fired, so the
/// run measured nothing P8 asks about.
///
/// So the redaction is now a KEEP-LIST, not a drop-list: the title and the
/// narrative problem statement survive, and everything that specifies the fix
/// is withheld. That is the shape of a real bug report with no repro — a
/// symptom and nothing else — which is what METHOD-P §5.2's fixture is for.
///
/// Returns the redacted text and whether anything was actually removed; an item
/// whose spec had nothing to withhold would not exercise the gate, and
/// recording that is cheaper than discovering it in the results.
pub fn strip_repro(spec: &str) -> (String, bool) {
    // Sections a reporter would plausibly have written. Everything else is a
    // specification of the fix and is withheld.
    const KEEP: [&str; 6] = [
        "## summary",
        "## problem",
        "## context",
        "## background",
        "## symptom",
        "## what is happening",
    ];
    let mut out = Vec::new();
    let mut keeping = true; // frontmatter and the title, until the first section
    let mut removed = false;
    for line in spec.lines() {
        let low = line.trim().to_lowercase();
        if low.starts_with("##") {
            let hit = KEEP.iter().any(|c| low.starts_with(c));
            keeping = hit;
            if !hit {
                removed = true;
                continue;
            }
        }
        if keeping {
            out.push(line.to_string());
        }
    }
    if removed {
        out.push(String::new());
        out.push("<!-- The remainder of this item was withheld. -->".into());
    }
    (out.join("
"), removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(steps: Vec<StepOutput>) -> GraphState {
        GraphState {
            item: "i".into(),
            arm: "0-delay".into(),
            repeat: 1,
            activity: "minimal-repro".into(),
            graph_sha256: "g".into(),
            corpus_sha: "c".into(),
            fixture: "f".into(),
            steps,
            gate_arrivals: 1,
            gate_forced: false,
            marked_at: chrono::Utc::now(),
            tokens_before_gate: 0,
            cost_before_gate: 0.0,
        }
    }

    #[test]
    fn an_identical_resume_is_lossless() {
        let a = StepOutput::new("attempt", 1, serde_json::json!({"status": "not-reproduced"}), 10, 0.1);
        let before = state(vec![a.clone()]);
        let after = state(vec![a]);
        let c = check_resume(&before, &after);
        assert!(c.lossless);
        assert!(c.changed.is_empty() && c.lost.is_empty());
        assert_eq!(c.state_digest_before, c.state_digest_after);
    }

    #[test]
    fn a_changed_step_output_is_caught_and_named() {
        let before = state(vec![StepOutput::new("attempt", 1, serde_json::json!({"status": "not-reproduced"}), 10, 0.1)]);
        let after = state(vec![StepOutput::new("attempt", 1, serde_json::json!({"status": "reproduced"}), 10, 0.1)]);
        let c = check_resume(&before, &after);
        assert!(!c.lossless, "METHOD-P §5.4's first kill fires on a single instance");
        assert_eq!(c.changed, vec!["attempt#1"]);
        assert_ne!(c.state_digest_before, c.state_digest_after);
    }

    #[test]
    fn a_dropped_step_is_lost_not_merely_changed() {
        let before = state(vec![
            StepOutput::new("attempt", 1, serde_json::json!({"a": 1}), 1, 0.0),
            StepOutput::new("attempt", 2, serde_json::json!({"a": 2}), 1, 0.0),
        ]);
        let after = state(vec![StepOutput::new("attempt", 1, serde_json::json!({"a": 1}), 1, 0.0)]);
        let c = check_resume(&before, &after);
        assert!(!c.lossless);
        assert_eq!(c.lost, vec!["attempt#2"]);
        assert!(c.changed.is_empty(), "absent is not the same finding as altered");
    }

    #[test]
    fn key_order_does_not_change_a_digest_but_content_does() {
        // serde_json preserves insertion order for `Value::Object`, so a state
        // that reordered its keys WOULD hash differently — which is the point:
        // §5.3 asks whether resume reconstructs the same thing, and a
        // reordering is a difference the check must not excuse.
        let a = StepOutput::new("s", 1, serde_json::json!({"x": 1, "y": 2}), 0, 0.0);
        let b = StepOutput::new("s", 1, serde_json::json!({"x": 1, "y": 2}), 0, 0.0);
        assert_eq!(a.digest, b.digest);
        let c = StepOutput::new("s", 1, serde_json::json!({"x": 1, "y": 3}), 0, 0.0);
        assert_ne!(a.digest, c.digest);
    }

    #[test]
    fn stripping_repro_keeps_the_symptom_and_withholds_the_specification() {
        let spec = "# T

## Summary

the gate passes when it should fail

## Steps to reproduce

1. do it

## Acceptance criteria

AC-001 must hold

## Behavior

declaredBoundary returns string[]
";
        let (out, removed) = strip_repro(spec);
        assert!(removed);
        assert!(out.contains("# T"), "the title survives");
        assert!(out.contains("the gate passes when it should fail"), "the symptom survives");
        // #28: these are what made the control reproduce without the gate.
        assert!(!out.contains("do it"), "the steps are gone");
        assert!(!out.contains("AC-001"), "the acceptance criteria are gone");
        assert!(!out.contains("declaredBoundary"), "the behaviour spec is gone");
        assert!(out.contains("withheld"));

        // Nothing to withhold means the gate would not be exercised.
        let (_, removed) = strip_repro("# T

## Summary

words
");
        assert!(!removed);
    }

    #[test]
    fn the_answer_binding_is_explicit_in_the_prompt() {
        let p = build_attempt_prompt("slug", "SPEC", Some("use --verbose"));
        assert!(p.contains("${gate.answer}"), "§3 binds the answer into the same step");
        assert!(p.contains("use --verbose"));
        // Without an answer there is no binding section at all.
        let p0 = build_attempt_prompt("slug", "SPEC", None);
        assert!(!p0.contains("gate.answer"));
    }

    #[test]
    fn state_round_trips_through_disk() {
        let dir = std::env::temp_dir().join(format!("p8-state-{}", std::process::id()));
        let path = dir.join("s.json");
        let before = state(vec![StepOutput::new("attempt", 1, serde_json::json!({"k": "v"}), 5, 0.5)]);
        before.save(&path).unwrap();
        let after = GraphState::load(&path).unwrap();
        assert!(check_resume(&before, &after).lossless, "a file round trip loses nothing");
        std::fs::remove_dir_all(&dir).ok();
    }
}
