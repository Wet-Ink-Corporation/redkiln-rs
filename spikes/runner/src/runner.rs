//! Spawn one `claude -p`, stream it, and record everything it said.
//!
//! Two constraints shape this module and neither is obvious:
//!
//! 1. **The prompt goes on stdin, never argv.** Everything here runs on Windows,
//!    where the command line caps around 32 KB. The `redkiln-review` card alone
//!    is 6.6 KB and the `forge-fix` prompt bodies are larger; `commands/fix.md`
//!    already carries an `--obs-file` workaround for exactly this. Passing the
//!    prompt as an argument would work in testing and fail on a real review.
//!
//! 2. **A record is always produced, even when the step fails.** The point of
//!    inverting control is that cost becomes visible; a dispatch that dies
//!    without a usage record is the failure mode we are trying to leave behind.

use std::process::Stdio;
use std::time::Instant;

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::event::{self, ApiRetry, EventKind, Headroom, RateLimit, Usage};
use crate::fingerprint::{self, EffectiveConfig, GitSnapshot};
use crate::hygiene::Environment;
use crate::result_meta::{self, ResultMeta};
use crate::step::{StepError, StepSpec};

/// Everything one attempt produced. Written to the results store whether the
/// step succeeded or not.
#[derive(Debug, Clone, Serialize)]
pub struct StepRecord {
    pub experiment: String,
    pub arm: String,
    pub identity: String,
    pub label: String,
    pub phase: String,
    pub occurrence: u32,
    pub attempt: u32,
    pub model: String,
    pub effort: Option<String>,
    pub agent: Option<String>,

    pub started_at: chrono::DateTime<chrono::Utc>,
    pub wall_clock_ms: u128,

    pub usage: Option<Usage>,
    pub cache_read_share: Option<f64>,
    /// Cache writes by TTL tier, `(5m, 1h)`. `None` means the split was not
    /// reported, which is itself evidence the dispatch ran wholly on 5m.
    pub ttl_split: Option<(u64, u64)>,
    /// read + creation + input. `input_tokens` alone counts only tokens after
    /// the last cache breakpoint, so it is never the denominator you want.
    pub total_input_tokens: Option<u64>,
    pub total_cost_usd: Option<f64>,
    pub rate_limit: Option<RateLimit>,
    /// Plan headroom, read from the undocumented `rate_limit_event`. This is
    /// E0's primary output: the `seven_day` utilisation delta per run.
    pub headroom: Option<Headroom>,

    /// Charter §6.3: throttles retry internally and emit `api_retry`. A step
    /// that was throttled is not a step that was slow.
    pub api_retries: Vec<ApiRetry>,

    /// Census of every line seen, by kind. An unknown or malformed line showing
    /// up here is a finding about the platform (RFC R8).
    pub event_census: Vec<(String, usize)>,
    pub unknown_events: Vec<serde_json::Value>,

    pub exit_code: Option<i32>,
    pub outcome: String,
    pub error_kind: Option<String>,
    /// RK-B0013: did the agent actually produce a judgement? Only a schema
    /// failure counts. Everything else is silence.
    pub agent_spoke: bool,

    pub claude_version: String,
    pub flags: Vec<String>,

    // --- Provenance. None of this can be reconstructed after the fact. ---
    /// **The output.** For E3, E4 and E6 the digest *is* the data: blocker
    /// recall is computed from it. Journalling usage but not the answer would
    /// spend Opus headroom and keep the receipt instead of the result.
    pub digest: Option<serde_json::Value>,
    /// The final assistant text, kept even when a digest parsed — it is the only
    /// evidence that separates a malformed digest from a refusal.
    pub response_text: Option<String>,

    pub prompt_sha256: String,
    pub prompt_bytes: usize,
    pub agent_card_sha256: Option<String>,
    pub schema_sha256: Option<String>,

    /// The fixture's git state at dispatch. E1's hypothesis is *about* this.
    pub git: Option<GitSnapshot>,
    /// What the child reports it actually ran with, from `system/init`.
    pub effective: Option<EffectiveConfig>,

    /// The whole `result` message, verbatim.
    ///
    /// Learned the hard way: the typed `Usage` view silently dropped the nested
    /// `cache_creation` TTL split, and because the raw payload had been
    /// discarded there was no way to check after the fact whether it had ever
    /// been sent. Retaining it costs a few hundred bytes and makes every
    /// future question about the stream answerable from records already taken.
    pub result_raw: Option<serde_json::Value>,
    /// Typed view of the rich result fields: completion classification, the
    /// platform's own latency split, per-model cost, subagent stats, thinking
    /// tokens, and the confounds (service tier, speed, fast mode) that move
    /// silently between dispatches.
    pub meta: Option<ResultMeta>,
    /// `duration_ms - duration_api_ms`: harness overhead, separated from model
    /// time by the platform rather than estimated by us.
    pub overhead_ms: Option<u64>,
    /// E2's instrument: reasoning tokens. If these do not separate across
    /// `--effort` levels, effort is not being applied per dispatch.
    pub thinking_tokens: Option<u64>,
    /// E3 arm (a)'s instrument: how many in-session subagents actually spawned.
    /// Without it, "the subagent disagreed" and "the subagent never ran" are
    /// indistinguishable — RK-B0013 one level up.
    pub subagents_spawned: Option<u64>,
    /// Every tool call on the stream, with parent/subagent attribution.
    /// Arm (a) is scored on this: see `event::ToolCall`.
    pub tool_calls: Vec<event::ToolCall>,

    /// Spawn + MCP handshake + settings load, separated from model time. RFC
    /// §3.1 lists per-run process overhead as a cost of inversion and calls it
    /// "mostly latency" — this is the measurement behind that claim.
    pub time_to_first_event_ms: Option<u128>,

    /// Did isolation actually hold? `Some(false)` means MCP servers reached the
    /// child despite `--strict-mcp-config`, which voids the record for cache
    /// purposes. An unenforced isolation flag looks exactly like an enforced one
    /// unless the child's own view is checked.
    pub isolation_ok: Option<bool>,
}

pub struct Runner {
    claude_bin: String,
    env: Environment,
}

impl Runner {
    pub fn new(claude_bin: impl Into<String>, env: Environment) -> Self {
        Self {
            claude_bin: claude_bin.into(),
            env,
        }
    }

    /// Assemble the argument vector. Split out so a test can assert the shape
    /// without spawning anything — the flags *are* the experiment in E1.
    pub fn build_args(&self, spec: &StepSpec) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "-p".into(),
            "--output-format".into(),
            "stream-json".into(),
            "--verbose".into(),
            "--model".into(),
            spec.model.as_str().into(),
        ];

        // haiku rejects the parameter; passing it would be an error rather than
        // a no-op, so the tier decides.
        if let Some(effort) = spec.effort
            && spec.model.accepts_effort()
        {
            args.push("--effort".into());
            args.push(effort.as_str().into());
        }

        if let Some(card) = &spec.agent {
            args.push("--agents".into());
            args.push(card.to_agents_json());
            // `--agents` alone only defines an agent; `--agent` is what makes
            // the MAIN turn adopt it (F7). Arm (a) wants exactly the version
            // F7 warns about — the card defined but NOT adopted — so the
            // parent dispatches it through `Task` instead of becoming it.
            if spec.adopt_agent {
                args.push("--agent".into());
                args.push(card.name.clone());
            }
        }

        if let Some(schema) = &spec.schema {
            args.push("--json-schema".into());
            args.push(schema.to_string());
        }

        if !spec.allowed_tools.is_empty() {
            args.push("--allowed-tools".into());
            args.push(spec.allowed_tools.join(","));
        }

        if let Some(mode) = &spec.permission_mode {
            args.push("--permission-mode".into());
            args.push(mode.clone());
        }

        if spec.isolate_mcp {
            args.push("--strict-mcp-config".into());
        }

        args.extend(spec.extra_flags.iter().cloned());
        args
    }

    pub async fn run(&self, spec: &StepSpec) -> (StepRecord, Result<serde_json::Value, StepError>) {
        let started_at = chrono::Utc::now();
        let clock = Instant::now();
        let args = self.build_args(spec);

        let mut record = StepRecord {
            experiment: spec.experiment.clone(),
            arm: spec.arm.clone(),
            identity: spec.identity(),
            label: spec.label.clone(),
            phase: spec.phase.clone(),
            occurrence: spec.occurrence,
            attempt: spec.attempt,
            model: spec.model.as_str().into(),
            effort: spec.effort.map(|e| e.as_str().to_string()),
            agent: spec.agent.as_ref().map(|a| a.name.clone()),
            started_at,
            wall_clock_ms: 0,
            usage: None,
            cache_read_share: None,
            ttl_split: None,
            total_input_tokens: None,
            total_cost_usd: None,
            rate_limit: None,
            headroom: None,
            api_retries: Vec::new(),
            event_census: Vec::new(),
            unknown_events: Vec::new(),
            exit_code: None,
            outcome: "pending".into(),
            error_kind: None,
            agent_spoke: false,
            claude_version: self.env.claude_version.clone(),
            flags: args.clone(),

            digest: None,
            response_text: None,
            prompt_sha256: fingerprint::sha256_hex(spec.prompt.as_bytes()),
            prompt_bytes: spec.prompt.len(),
            agent_card_sha256: spec
                .agent
                .as_ref()
                .map(|c| fingerprint::sha256_hex(c.to_agents_json().as_bytes())),
            schema_sha256: spec
                .schema
                .as_ref()
                .map(|s| fingerprint::sha256_hex(s.to_string().as_bytes())),
            // Captured BEFORE the spawn: the snapshot the child will see at
            // startup is the one that exists now, not the one after it runs.
            git: fingerprint::git_snapshot(&spec.cwd),
            effective: None,
            result_raw: None,
            meta: None,
            overhead_ms: None,
            thinking_tokens: None,
            subagents_spawned: None,
            tool_calls: Vec::new(),
            time_to_first_event_ms: None,
            isolation_ok: None,
        };

        let spawned = tokio::process::Command::new(&self.claude_bin)
            .args(&args)
            .current_dir(&spec.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();

        let mut child = match spawned {
            Ok(c) => c,
            Err(e) => {
                let err = StepError::Transport {
                    attempts: 1,
                    detail: format!("spawn failed: {e}"),
                };
                return (finish(record, clock, &err), Err(err));
            }
        };

        // The prompt goes here, not in argv. See the module note.
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(spec.prompt.as_bytes()).await;
            let _ = stdin.shutdown().await;
        }

        let stdout = child.stdout.take().expect("stdout was piped");
        let stderr = child.stderr.take().expect("stderr was piped");

        let stderr_task = tokio::spawn(async move {
            let mut buf = String::new();
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                buf.push_str(&line);
                buf.push('\n');
            }
            buf
        });

        let mut census: std::collections::BTreeMap<String, usize> = Default::default();
        let mut result_line: Option<serde_json::Value> = None;
        let mut reader = BufReader::new(stdout).lines();

        let drain = async {
            while let Ok(Some(line)) = reader.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }
                if record.time_to_first_event_ms.is_none() {
                    record.time_to_first_event_ms = Some(clock.elapsed().as_millis());
                }
                let parsed = event::classify(&line);
                *census.entry(census_key(&parsed.kind)).or_default() += 1;

                match &parsed.kind {
                    EventKind::Result => result_line = Some(parsed.raw.clone()),
                    // What we asked for is not necessarily what applied: an
                    // ignored flag is indistinguishable from an honoured one in
                    // the results unless the child's own view is recorded.
                    EventKind::SystemInit => {
                        let cfg = fingerprint::effective_config(&parsed.raw);
                        if spec.isolate_mcp {
                            record.isolation_ok =
                                Some(cfg.mcp_servers.as_ref().is_none_or(|m| m.is_empty()));
                        }
                        record.effective = Some(cfg);
                    }
                    EventKind::ApiRetry => {
                        record.api_retries.push(event::api_retry_from(&parsed.raw))
                    }
                    EventKind::Assistant => record
                        .tool_calls
                        .extend(event::tool_calls_from_assistant(&parsed.raw)),
                    // Keep the LAST one: it is the closest reading to the
                    // dispatch's true cost, and the delta between
                    // consecutive steps is what E0 reports.
                    EventKind::RateLimitEvent => {
                        if let Some(h) = event::headroom_from_event(&parsed.raw) {
                            record.headroom = Some(h);
                        }
                    }
                    EventKind::Unknown(_) | EventKind::Malformed => {
                        // Never dropped. A new event shape is evidence.
                        record.unknown_events.push(parsed.raw.clone());
                    }
                    _ => {}
                }
            }
        };

        let timed_out = tokio::time::timeout(spec.timeout, drain).await.is_err();

        if timed_out {
            kill_tree(&mut child).await;
        }

        let status = child.wait().await.ok();
        let stderr_text = stderr_task.await.unwrap_or_default();

        record.exit_code = status.and_then(|s| s.code());
        record.event_census = census.into_iter().collect();

        if let Some(raw) = &result_line {
            record.usage = event::usage_from_result(raw);
            record.cache_read_share = record.usage.as_ref().and_then(|u| u.cache_read_share());
            record.ttl_split = record.usage.as_ref().and_then(|u| u.ttl_split());
            record.total_input_tokens = record.usage.as_ref().and_then(|u| u.total_input());
            record.total_cost_usd = event::total_cost_usd(raw);
            record.rate_limit = event::rate_limit_from_result(raw);
            record.response_text = response_text(raw);
            let meta = result_meta::parse(raw);
            record.overhead_ms = meta.overhead_ms();
            record.thinking_tokens = meta.thinking_tokens();
            record.subagents_spawned = meta.subagents_spawned();
            record.meta = Some(meta);
            record.result_raw = Some(raw.clone());
        }

        if timed_out {
            let err = StepError::Timeout(spec.timeout);
            return (finish(record, clock, &err), Err(err));
        }

        // A usage limit has no documented exit code or event subtype
        // (charter §6.3), so it is detected on the text and recorded as its own
        // variant rather than collapsing into a generic non-zero exit.
        // Prefer the platform's own classification; fall back to text only
        // because exhaustion has no documented subtype (charter §6.3).
        let structured_limit = record
            .meta
            .as_ref()
            .is_some_and(|m| m.looks_like_usage_limit());
        if structured_limit {
            let err = StepError::UsageLimit { resets_at: None };
            return (finish(record, clock, &err), Err(err));
        }
        if let Some(resets_at) = detect_usage_limit(&stderr_text, result_line.as_ref()) {
            let err = StepError::UsageLimit { resets_at };
            return (finish(record, clock, &err), Err(err));
        }

        let raw = match result_line {
            Some(r) => r,
            None => {
                let err = StepError::NonZero {
                    code: record.exit_code,
                    stderr: stderr_text.trim().to_string(),
                };
                return (finish(record, clock, &err), Err(err));
            }
        };

        match extract_digest(&raw, spec.schema.as_ref()) {
            Ok(digest) => {
                record.wall_clock_ms = clock.elapsed().as_millis();
                record.outcome = "ok".into();
                record.agent_spoke = true;
                record.digest = Some(digest.clone());
                (record, Ok(digest))
            }
            Err(err) => (finish(record, clock, &err), Err(err)),
        }
    }
}

fn finish(mut record: StepRecord, clock: Instant, err: &StepError) -> StepRecord {
    record.wall_clock_ms = clock.elapsed().as_millis();
    record.outcome = "error".into();
    record.error_kind = Some(err.kind().to_string());
    record.agent_spoke = err.spoke();
    record
}

/// The assistant's final text. Kept whole: truncating the one field that says
/// what the agent actually concluded is how a review becomes unauditable.
fn response_text(raw: &serde_json::Value) -> Option<String> {
    let payload = raw
        .get("result")
        .filter(|inner| inner.is_object())
        .unwrap_or(raw);
    payload
        .get("text")
        .or_else(|| payload.get("result"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

fn census_key(kind: &EventKind) -> String {
    match kind {
        EventKind::SystemInit => "system/init".into(),
        EventKind::System(s) => format!("system/{s}"),
        EventKind::ApiRetry => "system/api_retry".into(),
        EventKind::Assistant => "assistant".into(),
        EventKind::User => "user".into(),
        EventKind::Result => "result".into(),
        EventKind::RateLimitEvent => "rate_limit_event".into(),
        EventKind::Unknown(t) => format!("unknown/{t}"),
        EventKind::Malformed => "malformed".into(),
    }
}

/// Kill the whole tree. `Child::kill` reaches only the direct child, and
/// `claude` spawns its own workers; on Windows the reliable tree kill is
/// `taskkill /T /F`.
async fn kill_tree(child: &mut tokio::process::Child) {
    #[cfg(windows)]
    if let Some(pid) = child.id() {
        let _ = tokio::process::Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
    }
    let _ = child.kill().await;
}

/// There is no documented exit code or structured subtype for plan exhaustion,
/// so this matches on the surfaced text. Deliberately narrow: a false positive
/// here would mislabel a real failure as a usage limit and quietly void a run.
fn detect_usage_limit(stderr: &str, result: Option<&serde_json::Value>) -> Option<Option<String>> {
    let haystacks = [
        stderr.to_lowercase(),
        result.map(|r| r.to_string().to_lowercase()).unwrap_or_default(),
    ];
    let markers = [
        "usage limit reached",
        "you've hit your",
        "plan limit",
        "rate limit exceeded",
    ];
    for hay in &haystacks {
        if markers.iter().any(|m| hay.contains(m)) {
            let resets_at = result
                .and_then(|r| r.get("resets_at"))
                .and_then(|v| v.as_str())
                .map(str::to_string);
            return Some(resets_at);
        }
    }
    None
}

/// Pull the structured digest out of a result message and check it against the
/// step schema.
///
/// A `Schema` error is the one failure mode that means the agent spoke, so it
/// carries the raw text: that is the evidence a human needs to tell a malformed
/// digest from a refusal.
fn extract_digest(
    raw: &serde_json::Value,
    schema: Option<&serde_json::Value>,
) -> Result<serde_json::Value, StepError> {
    let payload = raw
        .get("result")
        .filter(|inner| inner.is_object())
        .unwrap_or(raw);

    // With --json-schema the structured value may arrive already parsed, or as
    // a JSON string in the text field. Try both before giving up.
    let candidate = payload
        .get("structured_output")
        .cloned()
        .or_else(|| payload.get("structured_result").cloned())
        .or_else(|| {
            payload
                .get("text")
                .or_else(|| payload.get("result"))
                .and_then(|v| v.as_str())
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s.trim()).ok())
        });

    let Some(digest) = candidate else {
        let text = payload
            .get("text")
            .or_else(|| payload.get("result"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        // No schema was demanded, so prose is the contract being met, not a
        // violated one. Only a step that ASKED for structure can fail to get it.
        if schema.is_none() {
            return Ok(serde_json::Value::String(text.to_string()));
        }

        let shown = payload
            .get("text")
            .or_else(|| payload.get("result"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .chars()
            .take(2000)
            .collect::<String>();
        return Err(StepError::Schema {
            detail: "result carried no structured digest".into(),
            raw: shown,
        });
    };

    if let Some(schema) = schema {
        let validator = jsonschema::validator_for(schema).map_err(|e| StepError::Schema {
            detail: format!("the step's own schema is invalid: {e}"),
            raw: String::new(),
        })?;
        if let Err(e) = validator.validate(&digest) {
            return Err(StepError::Schema {
                detail: e.to_string(),
                raw: digest.to_string().chars().take(2000).collect(),
            });
        }
    }

    Ok(digest)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;
    use crate::step::{AgentCard, Effort, Tier};

    fn env() -> Environment {
        Environment {
            claude_version: "2.1.258 (Claude Code)".into(),
            claude_version_ok: true,
            api_key_absent: true,
            platform: "windows".into(),
        }
    }

    fn spec() -> StepSpec {
        StepSpec {
            experiment: "E3".into(),
            arm: "b".into(),
            label: "review".into(),
            phase: "Review".into(),
            occurrence: 1,
            attempt: 1,
            model: Tier::Opus,
            effort: Some(Effort::Xhigh),
            agent: None,
            adopt_agent: true,
            prompt: "hello".into(),
            schema: None,
            allowed_tools: vec![],
            permission_mode: None,
            cwd: ".".into(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(60),
        }
    }

    #[test]
    fn the_prompt_never_appears_in_argv() {
        let runner = Runner::new("claude", env());
        let mut s = spec();
        s.prompt = "a very distinctive prompt body".into();
        let args = runner.build_args(&s);
        assert!(
            !args.iter().any(|a| a.contains("distinctive prompt")),
            "prompt leaked into argv: {args:?}"
        );
    }

    #[test]
    fn the_two_e3_arms_differ_by_exactly_one_flag() {
        // This is the whole experiment. Arm (b) ADOPTS the card, so the
        // headless turn is the reviewer. Arm (a) only DEFINES it, so the turn
        // is a parent that dispatches it through `Task`. If `--agent` leaked
        // into arm (a), the "in-session" arm would silently be a second copy
        // of arm (b) and the comparison would be with itself.
        let runner = Runner::new("claude", env());
        let card = AgentCard {
            name: "redkiln-review".into(),
            description: "d".into(),
            prompt: "p".into(),
            tools: Some(vec!["Read".into()]),
            model: None,
            effort: None,
            permission_mode: None,
        };

        let mut b = spec();
        b.agent = Some(card.clone());
        b.adopt_agent = true;
        let b_args = runner.build_args(&b);

        let mut a = spec();
        a.agent = Some(card);
        a.adopt_agent = false;
        let a_args = runner.build_args(&a);

        assert!(b_args.iter().any(|x| x == "--agent"), "arm (b) must adopt");
        assert!(!a_args.iter().any(|x| x == "--agent"), "arm (a) must NOT adopt");
        // Both still DEFINE the card: arm (a) cannot dispatch what is not there.
        assert_eq!(a_args.iter().filter(|x| *x == "--agents").count(), 1);
        assert_eq!(b_args.iter().filter(|x| *x == "--agents").count(), 1);
        // And nothing else differs.
        let stripped: Vec<&String> = b_args
            .iter()
            .enumerate()
            .filter(|(i, x)| {
                *x != "--agent" && b_args.get(i.wrapping_sub(1)).map(String::as_str) != Some("--agent")
            })
            .map(|(_, x)| x)
            .collect();
        assert_eq!(stripped, a_args.iter().collect::<Vec<_>>());
    }

    #[test]
    fn bare_is_never_passed() {
        let runner = Runner::new("claude", env());
        assert!(!runner.build_args(&spec()).iter().any(|a| a == "--bare"));
    }

    #[test]
    fn effort_is_omitted_on_haiku_and_present_otherwise() {
        let runner = Runner::new("claude", env());

        let mut haiku = spec();
        haiku.model = Tier::Haiku;
        haiku.effort = Some(Effort::High);
        assert!(!runner.build_args(&haiku).iter().any(|a| a == "--effort"));

        assert!(runner.build_args(&spec()).iter().any(|a| a == "--effort"));
    }

    #[test]
    fn an_agent_card_is_both_defined_and_selected() {
        let runner = Runner::new("claude", env());
        let mut s = spec();
        s.agent = Some(AgentCard {
            name: "reviewer".into(),
            description: "d".into(),
            prompt: "p".into(),
            tools: None,
            model: Some("opus".into()),
            effort: Some("xhigh".into()),
            permission_mode: None,
        });
        let args = runner.build_args(&s);
        // --agents alone leaves the card inert in -p; --agent is what adopts it.
        assert!(args.iter().any(|a| a == "--agents"));
        let idx = args.iter().position(|a| a == "--agent").expect("--agent");
        assert_eq!(args[idx + 1], "reviewer");
    }

    #[test]
    fn extra_flags_survive_so_e1_can_vary_them() {
        let runner = Runner::new("claude", env());
        let mut s = spec();
        s.extra_flags = vec!["--exclude-dynamic-system-prompt-sections".into()];
        assert!(
            runner
                .build_args(&s)
                .iter()
                .any(|a| a == "--exclude-dynamic-system-prompt-sections")
        );
    }

    #[test]
    fn digest_is_read_from_a_json_string_result() {
        let raw = serde_json::json!({
            "type": "result",
            "result": { "text": "{\"pass\": true, \"overall\": 3}" }
        });
        let digest = extract_digest(&raw, None).expect("digest");
        assert_eq!(digest["pass"], true);
    }

    #[test]
    fn a_missing_digest_is_a_schema_error_that_carries_the_text() {
        // A schema WAS demanded, so prose is a violated contract. The text is
        // carried because it is the only evidence distinguishing a malformed
        // digest from a refusal — the RK-B0013 distinction.
        let schema = serde_json::json!({"type": "object"});
        let raw = serde_json::json!({
            "type": "result",
            "result": { "text": "I could not complete this review." }
        });
        let err = extract_digest(&raw, Some(&schema)).unwrap_err();
        assert_eq!(err.kind(), "schema");
        // The agent spoke — that distinction is the whole point of RK-B0013.
        assert!(err.spoke());
        match err {
            StepError::Schema { raw, .. } => assert!(raw.contains("could not complete")),
            other => panic!("expected Schema, got {other:?}"),
        }
    }

    #[test]
    fn a_digest_violating_the_schema_is_rejected() {
        let schema = serde_json::json!({
            "type": "object",
            "required": ["pass"],
            "properties": { "pass": { "type": "boolean" } }
        });
        let raw = serde_json::json!({
            "type": "result",
            "result": { "structured_output": { "pass": "yes" } }
        });
        let err = extract_digest(&raw, Some(&schema)).unwrap_err();
        assert_eq!(err.kind(), "schema");
    }

    #[test]
    fn usage_limit_detection_is_narrow() {
        assert!(detect_usage_limit("You've hit your Opus limit", None).is_some());
        assert!(detect_usage_limit("error: file not found", None).is_none());
        // A generic non-zero exit must not be mislabelled as exhaustion.
        assert!(detect_usage_limit("Error: ENOENT", None).is_none());
    }

    #[test]
    fn prose_is_a_success_when_no_schema_was_demanded() {
        // A step that asked for nothing structured is satisfied by prose. This
        // is not leniency: only a step that ASKED for structure can fail to
        // get it, and conflating the two made every probe look like an error.
        let raw = serde_json::json!({
            "type": "result",
            "result": { "text": "ok" }
        });
        let digest = extract_digest(&raw, None).expect("prose is a valid schemaless digest");
        assert_eq!(digest, serde_json::Value::String("ok".into()));
    }

    #[test]
    fn prose_is_still_a_failure_when_a_schema_was_demanded() {
        let schema = serde_json::json!({"type": "object"});
        let raw = serde_json::json!({
            "type": "result",
            "result": { "text": "ok" }
        });
        assert_eq!(extract_digest(&raw, Some(&schema)).unwrap_err().kind(), "schema");
    }
}
