//! What a dispatch is, and how it can fail.
//!
//! The error enum here is the point of the exercise. RK-B0013: one HTTP 529
//! produced a COMMITTED `changes-requested` seal whose entire surviving finding
//! was "review failed to produce a digest" — because `!review` meant "the
//! reviewer said no" and "the reviewer never spoke" indistinguishably, and the
//! second one got written into git as the first. Under inversion that is
//! `Result<Digest, StepError>`, and the two cases cannot be confused because
//! they are different variants.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Model tier. Charter §9.4: tier is never a free variable — each experiment
/// pins the tier production uses for that step and holds it constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Haiku,
    Sonnet,
    Opus,
}

impl Tier {
    pub fn as_str(self) -> &'static str {
        match self {
            Tier::Haiku => "haiku",
            Tier::Sonnet => "sonnet",
            Tier::Opus => "opus",
        }
    }

    /// `tiers.mjs`: `EFFORTLESS_TIERS = ['haiku']` — haiku rejects the
    /// parameter, so an effort on haiku is a spec error rather than a runtime
    /// one. `scan-dispatch.mjs` exists in the TS tree to catch this at lint
    /// time; here it is checked once, at plan load.
    pub fn accepts_effort(self) -> bool {
        !matches!(self, Tier::Haiku)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Effort {
    Low,
    Medium,
    High,
    Xhigh,
    Max,
}

impl Effort {
    pub fn as_str(self) -> &'static str {
        match self {
            Effort::Low => "low",
            Effort::Medium => "medium",
            Effort::High => "high",
            Effort::Xhigh => "xhigh",
            Effort::Max => "max",
        }
    }
}

/// One dispatch, fully resolved: every value the child needs is here, because
/// nothing may rely on `.claude/` discovery (RFC R6 — `--bare` is documented to
/// become the `-p` default, and a design leaning on discovery is building on a
/// surface scheduled to change).
#[derive(Debug, Clone)]
pub struct StepSpec {
    pub experiment: String,
    pub arm: String,
    pub label: String,
    pub phase: String,
    pub occurrence: u32,
    pub attempt: u32,
    pub model: Tier,
    pub effort: Option<Effort>,
    /// The agent card, passed inline as `--agents` and selected with
    /// `--agent`. `redkiln-review` is `model: inherit` with no effort, so a
    /// headless dispatch that does not set both explicitly silently inherits
    /// the session default — which would confound E3.
    pub agent: Option<AgentCard>,
    pub prompt: String,
    /// JSON Schema for `--json-schema`, already parsed.
    pub schema: Option<serde_json::Value>,
    pub allowed_tools: Vec<String>,
    pub permission_mode: Option<String>,
    pub cwd: std::path::PathBuf,
    /// Exclude the operator's MCP servers from the child.
    ///
    /// Measured, 2026-09-02: a default `-p` inherits every MCP server the
    /// operator has configured, which on this machine was 4,631 tokens — 56% of
    /// the cache creation on a trivial dispatch. That is the operator's laptop
    /// leaking into the measurement. Charter principle 1 is isolation over
    /// reuse, so this defaults ON for measured runs.
    pub isolate_mcp: bool,
    /// Extra flags an arm varies — E1's whole design is flag arms, so this is
    /// data rather than a fixed set.
    pub extra_flags: Vec<String>,
    pub timeout: std::time::Duration,
}

impl StepSpec {
    /// The identity a journal entry is keyed on. Includes `attempt` because the
    /// TS workflows had to mutate labels (`#retry2`) to stop the host resume
    /// journal replaying a pre-fix verdict against post-fix code. Making the
    /// attempt part of the key structurally is the same fix without the
    /// string surgery.
    pub fn identity(&self) -> String {
        format!(
            "{}#{}:{}:{}",
            self.label,
            self.occurrence,
            self.attempt,
            self.model.as_str()
        )
    }

    /// The resume key. Scoped by experiment and arm because `identity` alone
    /// collides across arms — every arm runs the same labels by design, which
    /// is the point of a controlled comparison.
    pub fn resume_key(&self) -> String {
        format!("{}/{}/{}", self.experiment, self.arm, self.identity())
    }
}

/// An agent card as `--agents` accepts it.
///
/// Confirmed against v2.1.258: the object takes `description`, `prompt`,
/// `tools`, `model`, `effort`, `permissionMode` and more. This is what
/// retires RFC R2 ("effort has no `--agents` equivalent", severity High).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    #[serde(skip)]
    pub name: String,
    pub description: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,
    #[serde(rename = "permissionMode", skip_serializing_if = "Option::is_none")]
    pub permission_mode: Option<String>,
}

impl AgentCard {
    /// Render the `--agents` argument: a single-key object keyed by name.
    pub fn to_agents_json(&self) -> String {
        let mut map = BTreeMap::new();
        map.insert(self.name.clone(), self);
        serde_json::to_string(&map).expect("agent card is serialisable")
    }
}

/// Why a step produced no digest.
///
/// The split that matters is `Schema` versus everything else: `Schema` means
/// the agent spoke and we could not read it; the rest mean it never spoke. Only
/// the first is ever evidence about the work under review.
// `Cancelled` and `retryable()` are the plan executor's surface; it is
// deferred (see FINDINGS "Layer 0 built"), so they are unreferenced today. They
// stay because deleting a variant of this enum is how the two-outcomes bug
// (RK-B0013) got written in the first place.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum StepError {
    #[error("transport failure after {attempts} attempt(s): {detail}")]
    Transport { attempts: u32, detail: String },

    #[error("digest did not satisfy the step schema: {detail}")]
    Schema { detail: String, raw: String },

    #[error("step exceeded its {0:?} ceiling")]
    Timeout(std::time::Duration),

    #[error("step was cancelled")]
    #[allow(dead_code)] // constructed by the deferred plan executor's cancel path
    Cancelled,

    #[error("claude exited {code:?}: {stderr}")]
    NonZero { code: Option<i32>, stderr: String },

    /// Charter §6.3: headless has no wait-and-continue and no auto-retry on a
    /// plan limit, and there is no documented exit code or structured error
    /// subtype for exhaustion. This variant exists so an exhausted run is never
    /// silently recorded as a failed one.
    #[error("plan usage limit reached{}", .resets_at.as_ref().map(|r| format!(" (resets {r})")).unwrap_or_default())]
    UsageLimit { resets_at: Option<String> },
}

impl StepError {
    /// Did the agent actually produce a judgement? Only `Schema` did.
    ///
    /// This is the RK-B0013 predicate. A caller writing a verdict to durable
    /// state must consult it: everything that is not `spoke()` is silence, and
    /// silence is never a verdict.
    pub fn spoke(&self) -> bool {
        matches!(self, StepError::Schema { .. })
    }

    /// Short stable token for the results file.
    pub fn kind(&self) -> &'static str {
        match self {
            StepError::Transport { .. } => "transport",
            StepError::Schema { .. } => "schema",
            StepError::Timeout(_) => "timeout",
            StepError::Cancelled => "cancelled",
            StepError::NonZero { .. } => "non-zero",
            StepError::UsageLimit { .. } => "usage-limit",
        }
    }

    /// Is retrying this plausibly useful? A usage limit is not retryable on any
    /// timescale the runner controls, and a schema failure will reproduce.
    #[allow(dead_code)] // consumed by the deferred plan executor
    pub fn retryable(&self) -> bool {
        matches!(
            self,
            StepError::Transport { .. } | StepError::Timeout(_) | StepError::NonZero { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_schema_failure_counts_as_the_agent_speaking() {
        let spoke = StepError::Schema {
            detail: "missing `pass`".into(),
            raw: "{}".into(),
        };
        assert!(spoke.spoke());

        let silent = [
            StepError::Transport {
                attempts: 3,
                detail: "529".into(),
            },
            StepError::Timeout(std::time::Duration::from_secs(1)),
            StepError::Cancelled,
            StepError::NonZero {
                code: Some(1),
                stderr: String::new(),
            },
            StepError::UsageLimit { resets_at: None },
        ];
        for e in silent {
            assert!(!e.spoke(), "{} must not count as speaking", e.kind());
        }
    }

    #[test]
    fn a_usage_limit_is_never_retried() {
        assert!(!StepError::UsageLimit { resets_at: None }.retryable());
        assert!(
            !StepError::Schema {
                detail: String::new(),
                raw: String::new()
            }
            .retryable()
        );
        assert!(
            StepError::Transport {
                attempts: 1,
                detail: String::new()
            }
            .retryable()
        );
    }

    #[test]
    fn haiku_rejects_effort() {
        assert!(!Tier::Haiku.accepts_effort());
        assert!(Tier::Sonnet.accepts_effort());
        assert!(Tier::Opus.accepts_effort());
    }

    #[test]
    fn agent_card_renders_a_name_keyed_object() {
        let card = AgentCard {
            name: "reviewer".into(),
            description: "Adversarial verification gate".into(),
            prompt: "You are the review subagent.".into(),
            tools: Some(vec!["Read".into(), "Bash".into()]),
            model: Some("opus".into()),
            effort: Some("xhigh".into()),
            permission_mode: None,
        };
        let json: serde_json::Value =
            serde_json::from_str(&card.to_agents_json()).expect("valid json");

        // Keyed by name, and the name is not also an inner field.
        assert!(json.get("reviewer").is_some());
        assert!(json["reviewer"].get("name").is_none());
        // Model and effort ride in the card — the fix for RFC R2.
        assert_eq!(json["reviewer"]["model"], "opus");
        assert_eq!(json["reviewer"]["effort"], "xhigh");
        assert_eq!(json["reviewer"]["tools"][1], "Bash");
        // An absent optional is omitted, not null.
        assert!(json["reviewer"].get("permissionMode").is_none());
    }

    #[test]
    fn identity_separates_attempts_so_a_retry_cannot_replay_a_stale_verdict() {
        let base = StepSpec {
            experiment: "E4".into(),
            arm: "a".into(),
            label: "fix-review".into(),
            phase: "Fix".into(),
            occurrence: 1,
            attempt: 1,
            model: Tier::Opus,
            effort: Some(Effort::Xhigh),
            agent: None,
            prompt: String::new(),
            schema: None,
            allowed_tools: vec![],
            permission_mode: None,
            cwd: ".".into(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: std::time::Duration::from_secs(1),
        };
        let retried = StepSpec {
            attempt: 2,
            ..base.clone()
        };
        assert_ne!(base.identity(), retried.identity());
    }
}
