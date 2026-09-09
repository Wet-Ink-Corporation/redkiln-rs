//! Layer 0 — the step runner for the Redkiln-RS inversion spike.
//!
//! This is measurement apparatus, not a product. Charter §12 names the failure
//! mode it exists inside: "a Rust runner plus a plan format is roughly 60% of a
//! working orchestrator. The temptation will be to keep building and let the
//! spike quietly become the product without anyone taking a decision."
//!
//! Nothing in `crates/` may depend on this.

mod corpus;
mod e3;
mod event;
mod fingerprint;
mod gate;
mod human_gate;
mod hygiene;
mod journal;
mod p1;
mod p7;
mod p8;
mod reconcile;
mod result_meta;
mod runner;
mod step;
mod throttle;

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use journal::{Journal, RunHeader};
use runner::Runner;
use step::{Effort, StepSpec, Tier};
use throttle::{Guard, Margins};

#[derive(Parser)]
#[command(
    name = "redkiln-spike-runner",
    about = "Measurement apparatus for the Redkiln-RS inversion spike. Not a product."
)]
struct Cli {
    /// The `claude` binary to drive.
    #[arg(long, default_value = "claude", global = true)]
    claude_bin: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)] // ProbeArgs is big; the enum is constructed once
enum Command {
    /// Assert the environment is fit to produce a measured run, and print what
    /// was found. Costs nothing.
    Doctor,

    /// Run one trivial dispatch N times and record full usage each time.
    ///
    /// This is E1's instrument: the arms differ only in flags and in whether a
    /// commit lands between runs, so cache behaviour is the only thing varying.
    Probe(ProbeArgs),

    /// Measure the prompt-cache TTL behaviourally.
    ///
    /// The cost case for inversion rests on `-p` getting a 1h cache TTL where
    /// in-session subagents are forced to 5m. The charter proposed reading that
    /// from the `ephemeral_1h` / `ephemeral_5m` usage split — but v2.1.258 does
    /// not emit it (FINDINGS F3). So measure the thing itself: warm a prefix,
    /// wait, and see whether the read survives.
    TtlProbe(TtlArgs),

    /// Warm a prefix and exit immediately. Pair with `ttl-read` later.
    ///
    /// Splits what `ttl-probe` does in one long-lived process. A 90-minute
    /// sleep inside one invocation is fragile — ours was killed ~55 minutes in
    /// for reasons we could not establish, and the process dying lost the
    /// measurement. Here the gap is the interval between two timestamps, so
    /// nothing has to survive it.
    TtlMark(TtlStepArgs),

    /// Read a prefix back and report the MEASURED gap since the last warm mark.
    ///
    /// The gap need not be a round number: what is reported is what elapsed.
    /// The only requirement is that nothing else dispatched in that directory
    /// in between, which would have refreshed the entry.
    TtlRead(TtlStepArgs),

    /// S0.3 — extract the fix-lane item manifest. Zero model cost.
    ///
    /// Feeds S0.1 (gate false-positive rate) and E3/E6 (seeded-defect
    /// detection): both revert the same fixes on the same items.
    CorpusManifest(CorpusArgs),

    /// S0.1 — gate false-positive rate. Zero model cost, heavy wall clock.
    ///
    /// Reverts each fix's code while keeping its tests, then runs the declared
    /// affected gate. A mutant the gate passes is a survivor. Resumable per
    /// (item, condition), because the whole 236-file suite runs every time and
    /// a stopped job must cost one item rather than the run.
    GateProbe(GateArgs),

    /// S0.2 — deterministic reconcile replay. Zero model cost.
    ///
    /// Folds every resume-relevant commit trailer into state and reports the
    /// drift it finds, then measures the RFC's axis-2 claim: how much rejected
    /// work would a `Story:`-trailer-only resume mistake for finished?
    ReconcileReplay(ReconcileArgs),

    /// E3 — seeded-defect detection. The programme's hard gate.
    ///
    /// Registered in `spikes/METHOD-E3.md`. Reverts a real historical fix and
    /// asks the reviewer whether the spec's acceptance criteria are met, against
    /// clean controls, paired per item.
    ReviewProbe(ReviewArgs),

    /// H4 — is the digest a sufficient carrier across a human gate?
    ///
    /// METHOD-H §6 as amended. Every step under inversion is already its own
    /// process, so "contiguous vs resumed" has nothing to compare. What is
    /// testable is whether the structured digest carries enough: step 2 is
    /// replayed against a REAL recorded step-1 review, with and without that
    /// review's prose, plus a same-batch control for baseline variance.
    DigestProbe(DigestArgs),

    /// H2 + H3 — the human gate. Zero model cost.
    ///
    /// H2: is a suspension lossless across a real process boundary?
    /// H3: does the world move while a gate is open?
    HumanGate(HumanGateArgs),

    /// P1 arm L — triage classification stability. METHOD-P section 1.2.
    ///
    /// One classification dispatch per item x 5 repeats x 2 models, against a
    /// closed record with written anchors. Measures whether the LLM-emitted
    /// half of the context vector is stable enough for a deterministic
    /// selection engine — F27 showed the threshold judgement itself flips.
    ClassifyProbe(ClassifyArgs),

    /// P7 — encapsulated quality: cost vs catch. METHOD-P section 4.
    ///
    /// Drives root-cause.toml's graph over the E3 rig across three rigor arms.
    /// Whether the internal adversary block earns its cost at `recommended`,
    /// or only at `mandatory`, decides the pack's default rigor.
    GraphProbe(GraphArgs),

    /// P8 — a gate inside a graph. METHOD-P section 5.
    ///
    /// Drives minimal-repro.toml to its triage gate, blocks holding state so
    /// an external kill is real, then resumes in a fresh process and compares
    /// every step-output digest across the boundary.
    ReproProbe(ReproArgs),

    /// Internal: re-hydrate suspended state and print its digest.
    ///
    /// Exists so H2 crosses a genuine process boundary. Round-tripping in the
    /// same process would prove nothing about resume.
    Rehydrate(RehydrateArgs),
}

#[derive(clap::Args)]
struct DigestArgs {
    /// Recorded step-1 outputs. Replayed, never re-run: holding step 1 fixed
    /// removes its variance from the comparison entirely.
    #[arg(long, default_value = "../results/e3.jsonl")]
    source: PathBuf,
    /// Which recorded arm to take step-1 outputs from.
    #[arg(long, default_value = "b-s-silent")]
    source_arm: String,
    #[arg(long)]
    out: PathBuf,
    /// `digest` (digest only) · `full` (digest + the reviewer's prose) ·
    /// `control` (digest only again — the baseline-variance arm E0 requires).
    #[arg(long)]
    arm: String,
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    #[arg(long, default_value = "opus")]
    model: String,
    #[arg(long, default_value = "xhigh")]
    effort: String,
    #[arg(long, default_value_t = 5)]
    limit: usize,
    #[arg(long, default_value_t = 1800)]
    timeout_secs: u64,
}

#[derive(clap::Args)]
struct HumanGateArgs {
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    #[arg(long)]
    out: PathBuf,
    /// Round-trip at most N pauses through a child process. Each spawn costs
    /// ~30 ms; the default covers every pause in the corpus.
    #[arg(long, default_value_t = 200)]
    max_round_trips: usize,
}

#[derive(clap::Args)]
struct RehydrateArgs {
    #[arg(long)]
    state: PathBuf,
}

#[derive(clap::Args)]
struct ReviewArgs {
    #[arg(long, default_value = "../results/s03-fixlane-manifest.jsonl")]
    manifest: PathBuf,
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    #[arg(long, default_value = "../fixtures/s01-wt")]
    worktree: PathBuf,
    #[arg(long)]
    out: PathBuf,
    /// `clean` (ground truth: approved) or `s-silent` (ground truth:
    /// changes-requested).
    #[arg(long)]
    condition: String,
    /// Arm: `b` = headless direct with the card. Arms `a` and `c` are added
    /// once `b` is shown to work.
    #[arg(long, default_value = "b")]
    arm: String,
    #[arg(long, default_value = "opus")]
    model: String,
    #[arg(long, default_value = "xhigh")]
    effort: String,
    /// Repeat index, so the same item can be run more than once for the
    /// per-item stability that E0's variance pilot needs.
    #[arg(long, default_value_t = 1)]
    repeat: u32,
    #[arg(long)]
    limit: Option<usize>,
    #[arg(long)]
    only: Option<String>,
    /// METHOD-E3H §9 tier 2: drop the AC enumeration and the fix's file list
    /// from the prompt, so the reviewer must locate the relevant source itself.
    /// Recorded in the arm label, so blind and sighted records never pool.
    #[arg(long, default_value_t = false)]
    blind: bool,
    /// METHOD-S §2 (S-TYPED): `prose` = E3's digest with `pass` (default);
    /// `typed` = findings with an evidence class and NO verdict — the verdict
    /// is computed by the registered policies and recorded alongside.
    /// Recorded in the arm label, so typed and prose records never pool.
    #[arg(long, default_value = "prose")]
    schema: String,
    /// E3-H seed definitions (one JSON object per line). Only read for
    /// `--condition s-subtle`.
    #[arg(long, default_value = "../results/e3h-seeds.jsonl")]
    seeds: PathBuf,
    /// Charter §12's declared margins. E3 is the most expensive experiment in
    /// the programme and was the one command that never honoured them.
    #[arg(long, default_value_t = 0.90)]
    max_five_hour: f64,
    #[arg(long, default_value_t = 0.90)]
    max_seven_day: f64,
    /// CLEAN dispatches run far longer than S-SILENT — 420-904s against
    /// 127-175s — because a present fix has real behaviour to verify and a
    /// real suite to run, while an absent one is dismissed quickly. The first
    /// CLEAN chunk lost a record to the old 900s ceiling.
    #[arg(long, default_value_t = 2400)]
    timeout_secs: u64,
}

#[derive(clap::Args)]
struct ReconcileArgs {
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
}

#[derive(clap::Args)]
struct GateArgs {
    #[arg(long, default_value = "../results/s03-fixlane-manifest.jsonl")]
    manifest: PathBuf,
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    /// One reusable worktree. Per-item worktrees would each need their own
    /// 125 MB node_modules.
    #[arg(long, default_value = "../fixtures/s01-wt")]
    worktree: PathBuf,
    #[arg(long)]
    out: PathBuf,
    /// `s-fail` reverts code and keeps tests (the mutant); `clean` runs the
    /// shipped fix untouched as a fixture sanity check.
    #[arg(long, default_value = "s-fail")]
    condition: String,
    /// Process at most N items this invocation, so a long run can be chunked.
    #[arg(long)]
    limit: Option<usize>,
    /// Restrict to one slug.
    #[arg(long)]
    only: Option<String>,
}

#[derive(serde::Deserialize)]
struct ManifestItem {
    slug: String,
    dir: String,
    ac_ids: Vec<String>,
    checkpoint_sha: Option<String>,
    fix_files: Vec<String>,
    #[serde(default)]
    eligible: bool,
}

#[derive(clap::Args)]
struct CorpusArgs {
    /// Path to the frozen redkiln repo.
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
}

#[derive(clap::Args)]
struct TtlStepArgs {
    #[arg(long, default_value = "E1")]
    experiment: String,
    /// Arm label, e.g. "ttl-90m". On `ttl-read`, defaults to the arm of the
    /// mark being read back.
    #[arg(long)]
    arm: Option<String>,
    #[arg(long)]
    cwd: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long, default_value = "haiku")]
    model: String,
    #[arg(long, default_value = "Reply with the single word: ok")]
    prompt: String,
    /// Read the prompt from a file instead (see the note on `probe`).
    #[arg(long)]
    prompt_file: Option<PathBuf>,
    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,
    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
    /// H1 secondary (METHOD-H §3): replace the default system prompt. F16
    /// measured this holding cache-read share at 1.000 across COMMITS; whether
    /// it also blunts decay across TIME is the open question, and if it does
    /// the human gate is near-free — a design conclusion, not a detail.
    #[arg(long)]
    system_prompt: Option<String>,
    /// Extra flags passed through to `claude`, one token per `--flag`, as on
    /// `probe`. S-H1c: `--flag --exclude-dynamic-system-prompt-sections --flag
    /// --append-system-prompt --flag "<text>"`.
    #[arg(long = "flag")]
    flags: Vec<String>,
}

#[derive(clap::Args)]
struct TtlArgs {
    #[arg(long, default_value = "E1")]
    experiment: String,

    #[arg(long)]
    cwd: PathBuf,

    #[arg(long)]
    out: PathBuf,

    /// Gaps in minutes between warming a prefix and reading it back.
    ///
    /// The default set brackets the two candidate TTLs. **10 is the decisive
    /// one**: a warm read at 10 minutes proves the TTL exceeds 5m. 45 confirms
    /// it holds within the hour, and 90 should show decay if the TTL is 1h —
    /// a warm read at 90 would mean the TTL is longer than either doc suggests.
    #[arg(long, value_delimiter = ',', default_value = "2,10,45,90")]
    gaps: Vec<u64>,

    #[arg(long, default_value = "haiku")]
    model: String,

    #[arg(long, default_value = "Reply with the single word: ok")]
    prompt: String,

    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,

    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
}

#[derive(clap::Args)]
struct ProbeArgs {
    /// Experiment label, e.g. "E1".
    #[arg(long, default_value = "E1")]
    experiment: String,

    /// Arm label, e.g. "b-commit-between".
    #[arg(long)]
    arm: String,

    /// How many sequential dispatches.
    #[arg(long, default_value_t = 10)]
    repeats: u32,

    /// The directory the child runs in. All repeats share it deliberately:
    /// "two sessions in different directories build different prefixes and
    /// miss each other's cache."
    #[arg(long)]
    cwd: PathBuf,

    /// Where results land. Refused if inside `--cwd`.
    #[arg(long)]
    out: PathBuf,

    /// Make an empty commit in `--cwd` between dispatches. This is the arm that
    /// matters: Redkiln commits at every checkpoint, and sequential sessions
    /// share a prefix "only when the git status snapshot at startup matches".
    #[arg(long)]
    commit_between: bool,

    /// Extra flags passed through to `claude`, e.g.
    /// `--flag --exclude-dynamic-system-prompt-sections`.
    #[arg(long = "flag")]
    flags: Vec<String>,

    #[arg(long, default_value = "sonnet")]
    model: String,

    /// Reasoning effort. E2's variable: if `thinking_tokens` does not separate
    /// across levels, effort is not being applied per dispatch. Ignored on
    /// haiku, which rejects the parameter.
    #[arg(long, default_value = "low")]
    effort: String,

    /// The prompt. A one-liner suffices — E1 measures the prefix, not the work.
    #[arg(long, default_value = "Reply with the single word: ok")]
    prompt: String,

    /// Read the prompt from a file instead. Required for review-scale payloads:
    /// Windows caps a command line near 32 KB, and H1's payload is 37 KB.
    #[arg(long)]
    prompt_file: Option<PathBuf>,

    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,

    /// Refuse to dispatch once the five-hour window reaches this utilisation.
    #[arg(long, default_value_t = 0.90)]
    max_five_hour: f64,

    /// Refuse to dispatch once the seven-day window reaches this utilisation.
    #[arg(long, default_value_t = 0.90)]
    max_seven_day: f64,

    /// Let the operator's MCP servers reach the child. Off by default: they
    /// were measured at 56% of cache creation, and they are not part of what
    /// any experiment here is trying to measure.
    #[arg(long)]
    inherit_mcp: bool,

    /// Path to the corpus pin, recorded in the results header.
    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Doctor => doctor(&cli.claude_bin).await,
        Command::Probe(args) => probe(&cli.claude_bin, args).await,
        Command::TtlProbe(args) => ttl_probe(&cli.claude_bin, args).await,
        Command::TtlMark(args) => ttl_step(&cli.claude_bin, args, "warm").await,
        Command::TtlRead(args) => ttl_step(&cli.claude_bin, args, "read").await,
        Command::CorpusManifest(args) => corpus_manifest(args),
        Command::GateProbe(args) => gate_probe(args),
        Command::ReconcileReplay(args) => reconcile_replay(args),
        Command::ReviewProbe(args) => review_probe(&cli.claude_bin, args).await,
        Command::ClassifyProbe(args) => classify_probe(&cli.claude_bin, args).await,
        Command::GraphProbe(args) => graph_probe(&cli.claude_bin, args).await,
        Command::ReproProbe(args) => repro_probe(&cli.claude_bin, args).await,
        Command::DigestProbe(args) => digest_probe(&cli.claude_bin, args).await,
        Command::HumanGate(args) => human_gate(args),
        Command::Rehydrate(args) => {
            let text = std::fs::read_to_string(&args.state)?;
            let state: human_gate::SuspendedState = serde_json::from_str(&text)?;
            println!("{}", state.digest());
            Ok(())
        }
    }
}

fn human_gate(args: HumanGateArgs) -> Result<()> {
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    let journal = Journal::open(&args.out, &[])?;
    let exe = std::env::current_exe()?;
    let scratch = std::env::temp_dir().join("rk-h2");

    // Telemetry, unioned across partitions.
    let mut events = Vec::new();
    let dir = redkiln.join(".redkiln").join("telemetry").join("events");
    for entry in std::fs::read_dir(&dir).with_context(|| format!("reading {}", dir.display()))? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("jsonl") {
            continue;
        }
        for line in std::fs::read_to_string(&path)?.lines() {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
                events.push(v);
            }
        }
    }

    let pauses = human_gate::gate_pauses(&events);
    let commits = reconcile::read_commits(&redkiln)?;
    anyhow::ensure!(!pauses.is_empty(), "no gate pauses found in telemetry");

    journal.append(&serde_json::json!({
        "record": "header", "experiment": "H2+H3",
        "generated_at": chrono::Utc::now(),
        "pauses": pauses.len(), "commits_with_trailers": commits.len(),
    }))?;

    let (mut lossless, mut lossy) = (0usize, 0usize);
    let (mut drifted, mut drifted_over_ttl) = (0usize, 0usize);
    let mut in_gap_total = 0usize;
    let mut over_ttl = 0usize;

    for (i, p) in pauses.iter().enumerate() {
        if p.outlived_cache_ttl {
            over_ttl += 1;
        }

        // H3: recompute the world at both ends of the gap.
        let before = human_gate::state_at(&commits, p.opened_at);
        let after = human_gate::state_at(&commits, p.answered_at);
        let drift = before != after;
        let in_gap = human_gate::commits_in_gap(&commits, p);
        in_gap_total += in_gap.len();
        if drift {
            drifted += 1;
            if p.outlived_cache_ttl {
                drifted_over_ttl += 1;
            }
        }

        // H2: round-trip the suspended state through a separate process.
        let mut h2 = None;
        if i < args.max_round_trips {
            let suspended = human_gate::SuspendedState {
                item: p.item.clone(),
                at: p.opened_at,
                state: before.clone(),
            };
            let r = human_gate::round_trip(&exe, &suspended, &scratch)?;
            if r.lossless {
                lossless += 1;
            } else {
                lossy += 1;
                eprintln!("  LOSSY suspension: {} {} != {}", r.item, r.in_process, r.rehydrated);
            }
            h2 = Some(r);
        }

        journal.append(&serde_json::json!({
            "record": "pause", "pause": p,
            "h3_world_drifted": drift,
            "h3_commits_in_gap": in_gap.len(),
            "h3_subjects_in_gap": in_gap.iter().map(|c| c.subject.clone()).collect::<Vec<_>>(),
            "h2": h2,
        }))?;
    }

    let n = pauses.len();
    let pct = |x: usize| 100.0 * x as f64 / n as f64;
    println!("gate pauses                 : {n}");
    println!("  outlived the cache TTL    : {over_ttl}  ({:.0}%)", pct(over_ttl));
    println!("\n--- H2: is a suspension lossless across a process boundary? ---");
    println!("  round-tripped             : {}", lossless + lossy);
    println!("  LOSSLESS                  : {lossless}");
    println!("  lossy                     : {lossy}");
    println!("\n--- H3: does the world move while a gate is open? ---");
    println!("  pauses with world drift   : {drifted}  ({:.0}%)", pct(drifted));
    println!("  ... of those, gate > 1h   : {drifted_over_ttl}");
    println!("  commits landing in gaps   : {in_gap_total}");

    journal.append(&serde_json::json!({
        "record": "summary",
        "pauses": n, "outlived_ttl": over_ttl,
        "h2_lossless": lossless, "h2_lossy": lossy,
        "h3_drifted": drifted, "h3_drifted_over_ttl": drifted_over_ttl,
        "h3_commits_in_gaps": in_gap_total,
    }))?;
    println!("\nwrote {}", journal.path().display());
    Ok(())
}

/// The most recent headroom reading in a results file, **if it still describes
/// the current window**.
///
/// Read back so a chunked experiment resumes knowing where it stands rather
/// than rediscovering it by spending a dispatch. But a reading is only about
/// the window it was taken in: a record saying `five_hour 0.86` is worthless
/// once that window has rolled, and seeding the guard with it halts a run that
/// in fact has a full window in front of it. `resetsAt` is on the record
/// precisely so this is decidable without spending anything.
///
/// A stale reading is discarded rather than corrected. That leaves the opening
/// dispatch unguarded, which is `Guard`'s documented behaviour and is safe:
/// headroom can only be learned *from* a dispatch.
fn last_headroom(path: &std::path::Path, now: i64) -> Option<event::Headroom> {
    let text = std::fs::read_to_string(path).ok()?;
    let h: event::Headroom = text
        .lines()
        .rev()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .find_map(|v| {
            serde_json::from_value::<event::Headroom>(v.get("step")?.get("headroom")?.clone()).ok()
        })?;
    match h.five_hour_resets_at {
        Some(at) if at <= now => None,
        _ => Some(h),
    }
}

/// H4 — digest sufficiency across a gate.
async fn digest_probe(claude_bin: &str, args: DigestArgs) -> Result<()> {
    let with_prose = match args.arm.as_str() {
        "full" => true,
        "digest" | "control" => false,
        other => anyhow::bail!("unknown arm `{other}` (expected digest, full or control)"),
    };
    let env = hygiene::assert_clean(claude_bin).await?;
    let journal = Journal::open(&args.out, &[])?;
    let runner = Runner::new(claude_bin, env);
    let done = journal::completed_keys(&args.out);

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };
    let effort = match args.effort.as_str() {
        "low" => Effort::Low,
        "medium" => Effort::Medium,
        "high" => Effort::High,
        "xhigh" => Effort::Xhigh,
        "max" => Effort::Max,
        other => anyhow::bail!("unknown effort `{other}`"),
    };

    // Richest digests first: a handover with one finding cannot show whether
    // the prose added anything.
    let text = std::fs::read_to_string(&args.source)
        .with_context(|| format!("reading --source {}", args.source.display()))?;
    let mut sources: Vec<serde_json::Value> = text
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|v| {
            v.get("arm").and_then(|a| a.as_str()) == Some(args.source_arm.as_str())
                && v.get("outcome").and_then(|o| o.as_str()) == Some("ok")
        })
        .collect();
    sources.sort_by_key(|v| {
        -(v.pointer("/step/digest/issues")
            .and_then(|i| i.as_array())
            .map(|a| a.len())
            .unwrap_or(0) as i64)
    });
    anyhow::ensure!(!sources.is_empty(), "no step-1 records in {}", args.source.display());

    let schema = e3::step2_schema();
    // The DIGEST arm is the reference every other arm is scored against, so
    // agreement lands on the record itself rather than in a later script.
    let baseline: std::collections::BTreeMap<String, Vec<String>> =
        std::fs::read_to_string(&args.out)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
            .filter(|v| v.get("arm").and_then(|a| a.as_str()) == Some("h4-digest"))
            .filter_map(|v| {
                Some((
                    v.get("slug")?.as_str()?.to_string(),
                    v.get("files")?
                        .as_array()?
                        .iter()
                        .filter_map(|f| f.as_str())
                        .map(String::from)
                        .collect(),
                ))
            })
            .collect();

    for src in sources.iter().take(args.limit) {
        let slug = src["slug"].as_str().unwrap_or("?").to_string();
        let Some(digest) = src.pointer("/step/digest") else { continue };
        let prose = src.pointer("/step/response_text").and_then(|v| v.as_str());

        let resume_key = format!("H4/{}/{slug}#r1", args.arm);
        if done.contains(&resume_key) {
            eprintln!("  {slug:<46} skipped (recorded)");
            continue;
        }

        // The fixture the review was written about, so "which files" is
        // grounded in a real tree rather than answered from memory.
        let cwd = std::fs::canonicalize(format!("../fixtures/e3-silent-{slug}"))
            .unwrap_or_else(|_| args.redkiln.clone());

        let prompt = e3::build_step2_prompt(&slug, digest, if with_prose { prose } else { None });
        eprint!("  {slug:<46} h4-{} ... ", args.arm);

        let spec = StepSpec {
            experiment: "H4".into(),
            arm: format!("h4-{}", args.arm),
            label: slug.clone(),
            phase: "step2".into(),
            occurrence: 1,
            attempt: 1,
            model,
            effort: Some(effort),
            agent: None,
            adopt_agent: true,
            prompt,
            schema: Some(schema.clone()),
            allowed_tools: vec![],
            permission_mode: Some("bypassPermissions".into()),
            cwd,
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(args.timeout_secs),
        };

        let (record, _) = runner.run(&spec).await;
        let files: Vec<String> = record
            .digest
            .as_ref()
            .and_then(|d| d.get("files"))
            .and_then(|f| f.as_array())
            .map(|a| a.iter().filter_map(|v| v.as_str()).map(String::from).collect())
            .unwrap_or_default();

        match record.digest {
            Some(_) => eprintln!("{} files  {:.0}s", files.len(), record.wall_clock_ms as f64 / 1000.0),
            None => eprintln!("NO DIGEST ({})", record.error_kind.as_deref().unwrap_or("?")),
        }

        // Jaccard against the DIGEST arm's file set for the same item.
        // `full` vs digest is the effect; `control` vs digest is the baseline
        // the effect has to beat to mean anything (F27).
        let jaccard_vs_digest = baseline.get(&slug).and_then(|b| e3::jaccard(&files, b));
        if let Some(j) = jaccard_vs_digest {
            eprintln!("      jaccard vs digest arm: {j:.2}");
        }

        journal.append(&serde_json::json!({
            "experiment": "H4",
            "resume_key": resume_key,
            "jaccard_vs_digest": jaccard_vs_digest,
            "outcome": if files.is_empty() { "void" } else { "ok" },
            "arm": format!("h4-{}", args.arm),
            "slug": slug,
            "with_prose": with_prose,
            "source_arm": args.source_arm,
            "step1_issue_count": digest.get("issues").and_then(|i| i.as_array()).map(|a| a.len()),
            "step1_prose_chars": prose.map(str::len),
            "files": files.clone(),
            "step": record,
        }))?;
    }

    println!("\nwrote {}", journal.path().display());
    Ok(())
}

async fn review_probe(claude_bin: &str, args: ReviewArgs) -> Result<()> {
    // Arm (a) = the in-session shape: a headless PARENT that dispatches the
    // card through `Task`. Arm (b) = the headless turn IS the reviewer.
    // The single flipped bit is `--agent`; everything else is held identical.
    let arm_a = match args.arm.as_str() {
        "a" => true,
        "b" | "c" => false,
        other => anyhow::bail!("unknown arm `{other}` (expected a, b or c)"),
    };
    let condition: &'static str = match args.condition.as_str() {
        "clean" => "clean",
        "s-silent" => "s-silent",
        // E3-H (METHOD-E3H): the fix is fully PRESENT and one line of it is
        // wrong. Harder than s-silent along the axis that matters — the
        // reviewer cannot succeed by noticing that code is absent.
        "s-subtle" => "s-subtle",
        other => {
            anyhow::bail!("unknown condition `{other}` (expected clean, s-silent or s-subtle)")
        }
    };
    let seeds: Vec<e3::Seed> = if condition == "s-subtle" {
        let text = std::fs::read_to_string(&args.seeds)
            .with_context(|| format!("reading --seeds {}", args.seeds.display()))?;
        text.lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str::<e3::Seed>(l).ok())
            .collect()
    } else {
        Vec::new()
    };
    anyhow::ensure!(
        condition != "s-subtle" || !seeds.is_empty(),
        "s-subtle needs seeds; none parsed from {}",
        args.seeds.display()
    );
    let env = hygiene::assert_clean(claude_bin).await?;
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    let worktree = std::fs::canonicalize(&args.worktree)?;
    let journal = Journal::open(&args.out, &[])?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };
    let effort = match args.effort.as_str() {
        "low" => Effort::Low,
        "medium" => Effort::Medium,
        "high" => Effort::High,
        "xhigh" => Effort::Xhigh,
        "max" => Effort::Max,
        other => anyhow::bail!("unknown effort `{other}`"),
    };

    // Seeded from the last record in the results file, not from zero. E3 runs
    // in chunks, so an unseeded guard would leave the FIRST dispatch of every
    // chunk unguarded — which is precisely the dispatch that resumes a run
    // already close to a margin.
    let margins = Margins {
        max_five_hour: args.max_five_hour,
        max_seven_day: args.max_seven_day,
    };
    let mut guard = Guard::default();
    guard.observe(last_headroom(&args.out, chrono::Utc::now().timestamp()).as_ref());
    if let Some(h) = guard.last() {
        eprintln!(
            "  resuming at five_hour {:.2} / seven_day {:.2} (margins {:.2}/{:.2})",
            h.five_hour_utilization.unwrap_or(f64::NAN),
            h.seven_day_utilization.unwrap_or(f64::NAN),
            margins.max_five_hour,
            margins.max_seven_day
        );
    } else {
        eprintln!("  no live headroom reading — the opening dispatch is unguarded");
    }

    let (description, card_body) = e3::load_card(&redkiln)?;
    let typed = match args.schema.as_str() {
        "prose" => false,
        "typed" => true,
        other => anyhow::bail!("unknown schema `{other}` (expected prose or typed)"),
    };
    let schema = if typed { e3::typed_findings_schema() } else { e3::digest_schema() };
    let runner = Runner::new(claude_bin, env);
    let done = journal::completed_keys(&args.out);
    let items = read_manifest(&args.manifest)?;
    let mut processed = 0usize;

    for item in &items {
        if let Some(only) = &args.only
            && &item.slug != only
        {
            continue;
        }
        if let Some(limit) = args.limit
            && processed >= limit
        {
            break;
        }
        // Every instrument variant is in the label: `b-blind-typed-s-subtle`
        // and `b-clean` can never be pooled by an audit that keys on `arm`.
        let arm_label = format!(
            "{}{}{}-{}",
            args.arm,
            if args.blind { "-blind" } else { "" },
            if typed { "-typed" } else { "" },
            condition
        );
        let resume_key = format!("E3/{arm_label}/{}#r{}", item.slug, args.repeat);
        if done.contains(&resume_key) {
            eprintln!("  {:<44} skipped (recorded)", item.slug);
            continue;
        }
        let Some(sha) = item.checkpoint_sha.clone() else {
            continue;
        };

        // Charter §12: refuse to dispatch below a declared margin. A halt is a
        // PAUSE, not a failure — resume is keyed per item, so the run picks up
        // exactly here once the window rolls.
        match guard.check(margins) {
            throttle::Verdict::Proceed => {}
            verdict => {
                eprintln!("\n  HALT before {}: {}", item.slug, verdict.reason());
                eprintln!("  {} item(s) not dispatched. Re-run to resume.",
                    items.len().saturating_sub(processed));
                break;
            }
        }

        // Build the fixture. F17: for `s-silent` the fix must be UNREACHABLE,
        // not merely reverted — a reviewer with Bash restored it from the
        // worktree's object store and reviewed the real code. A shallow fetch
        // of the parent gives a one-commit history the fix does not exist in.
        let review_cwd;
        let reverted: Vec<String>;
        // Recorded, not merely printed: each of the three pre-flight checks
        // (card adopted, node_modules present, fix unreachable) must be
        // auditable from the journal alone, without reading a console log.
        let node_modules_linked;
        let fix_unreachable;
        // E3-H: which criterion this seed violates, so a reviewer's citation
        // can be read against the AC it was supposed to catch.
        let mut ac_violated: Option<String> = None;
        if condition == "s-silent" {
            let fixture = worktree
                .parent()
                .unwrap_or(&worktree)
                .join(format!("e3-silent-{}", item.slug));
            node_modules_linked = e3::build_silent_fixture(&redkiln, &fixture, &sha)?;
            // Hard-fails: a fixture that kept the fix would reproduce F17 in
            // silence, scoring the reviewer against code it never saw.
            e3::assert_fix_unreachable(&fixture, &sha)?;
            fix_unreachable = true;
            review_cwd = fixture;
            // Everything the fix touched is at its pre-fix state here.
            reverted = item
                .fix_files
                .iter()
                .filter(|f| {
                    matches!(gate::classify(f), gate::FileRole::Code | gate::FileRole::Test)
                })
                .cloned()
                .collect();
            if reverted.is_empty() {
                eprintln!("  {:<44} EXCLUDED (no code or tests)", item.slug);
                continue;
            }
        } else if condition == "s-subtle" {
            let Some(seed) = seeds.iter().find(|s| s.slug == item.slug) else {
                continue; // not an E3-H seed; E3-H runs a 5-item subset
            };
            let fixture = worktree
                .parent()
                .unwrap_or(&worktree)
                .join(format!("e3-subtle-{}", item.slug));
            node_modules_linked = e3::build_subtle_fixture(&redkiln, &fixture, &sha, seed)?;
            // The mutation is amended INTO the single commit and the original
            // pruned, so the correct line is not recoverable from the fixture.
            e3::assert_fix_unreachable(&fixture, &sha)?;
            fix_unreachable = true;
            review_cwd = fixture;
            // Exactly the one file the mutation touched — which makes the
            // strong-detection file test sharper here than in E3.
            reverted = vec![seed.file.clone()];
            ac_violated = Some(seed.ac_violated.clone());
        } else {
            // CLEAN is built the same way and to the same depth, landing on the
            // fix commit instead of its parent. Symmetry is deliberate: were
            // CLEAN reviewed in the full worktree, the bit flipped between the
            // conditions would be the shape of the repository as well as the
            // state of the code, and a reviewer noticing a one-entry `git log`
            // could treat the two differently for reasons unrelated to any
            // defect. The co-primary false-positive rate is only comparable to
            // the detection rate if both are measured in the same place.
            let fixture = worktree
                .parent()
                .unwrap_or(&worktree)
                .join(format!("e3-clean-{}", item.slug));
            node_modules_linked = e3::build_fixture_at(&redkiln, &fixture, &sha)?;
            review_cwd = fixture;
            reverted = Vec::new();
            // CLEAN reviews the fix as shipped, so the fix is *meant* to be
            // present; the F17 check does not apply and is recorded as such.
            fix_unreachable = false;
        }
        if !node_modules_linked {
            // A reviewer that CANNOT run tests is indistinguishable in the
            // digest from one that CHOSE not to, so this aborts rather than
            // spending a dispatch on a record nobody could interpret.
            anyhow::bail!("no node_modules for {} — it could not run tests", item.slug);
        }

        let read = |name: &str| {
            std::fs::read_to_string(redkiln.join(&item.dir).join(name)).unwrap_or_default()
        };
        let brief = if args.blind {
            e3::build_prompt_blind(
                &item.slug,
                &read("spec.md"),
                &read("implementation-report.md"),
            )
        } else {
            e3::build_prompt(
                &item.slug,
                &read("spec.md"),
                &read("implementation-report.md"),
                &item.ac_ids,
                &item.fix_files,
            )
        };
        // S-TYPED: the addendum goes on the BRIEF, so arm (a)'s parent still
        // forwards it verbatim and both arms carry the same instructions.
        let brief = if typed { e3::build_prompt_typed(&brief) } else { brief };
        // Arm (a) forwards arm (b)'s brief verbatim inside a dispatch wrapper,
        // so the two arms review identical material with an identical card.
        let prompt = if arm_a {
            e3::build_parent_prompt(&brief)
        } else {
            brief
        };

        eprint!("  {:<44} {arm_label} r{} ... ", item.slug, args.repeat);
        let spec = StepSpec {
            experiment: "E3".into(),
            arm: arm_label.clone(),
            label: item.slug.clone(),
            phase: "review".into(),
            occurrence: args.repeat,
            attempt: 1,
            model,
            // Pinned explicitly: the card is `model: inherit` with no effort, so
            // omitting these would silently inherit the session default.
            effort: Some(effort),
            // Arm (a) DEFINES the card without adopting it, so the parent can
            // dispatch it as a subagent instead of becoming it (F7 inverted).
            adopt_agent: !arm_a,
            agent: Some(step::AgentCard {
                name: "redkiln-review".into(),
                description: description.clone(),
                prompt: card_body.clone(),
                // e3::CARD_TOOLS — [Read, Bash] from the production card, plus
                // the StructuredOutput channel the digest comes back on (F20).
                tools: Some(e3::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                model: Some(args.model.clone()),
                effort: Some(args.effort.clone()),
                permission_mode: None,
            }),
            prompt,
            schema: Some(schema.clone()),
            allowed_tools: vec![],
            permission_mode: Some("bypassPermissions".into()),
            cwd: review_cwd.clone(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(args.timeout_secs),
        };

        let (record, outcome) = runner.run(&spec).await;

        // F7: `--agents` alone is inert. If the card did not apply, this record
        // measures a generic Claude and is VOID, not a miss.
        // F17: `effective.agent` is NOT populated when the agent is set via
        // `--agent`, so reading it voided a record whose card had in fact
        // applied. The durable signal is the tool set: the card declares its
        // tools, and adoption restricts the child from 34 down to exactly
        // those. Equality, not a size bound — any divergence at all means the
        // card did not apply and the record measures a generic Claude.
        let declared: BTreeSet<&str> = e3::CARD_TOOLS.into_iter().collect();
        let adopted = record
            .effective
            .as_ref()
            .and_then(|e| e.tools.as_ref())
            .is_some_and(|t| t.iter().map(|x| x.as_str()).collect::<BTreeSet<_>>() == declared);

        guard.observe(record.headroom.as_ref());

        // Arm (a)'s validity, computed from the stream rather than assumed.
        // METHOD-E3 §6 requires `subagent_stats.spawned >= 1`; that alone
        // cannot tell OUR card from `general-purpose`, so the dispatched
        // `subagent_type` is checked too, and whether the parent kept to
        // delegating is recorded rather than trusted.
        let behaviour = arm_a.then(|| e3::parent_behaviour(&record.tool_calls, "redkiln-review"));
        let arm_a_ok = !arm_a
            || behaviour.as_ref().is_some_and(|b| b.dispatched_card)
                && record.subagents_spawned.unwrap_or(0) >= 1;

        // Typed digests are scored under the STRICT policy (METHOD-S §2.2,
        // primary); the lenient verdict is recorded beside it, never
        // substituted. The policy stands in for `pass`; only BLOCKING
        // findings' files count toward strong detection.
        let scored = record.digest.as_ref().map(|d| {
            if typed { e3::score_typed(d, &reverted, e3::STRICT) } else { e3::score(d, &reverted) }
        });
        let policy = typed.then(|| {
            record.digest.as_ref().map(|d| {
                serde_json::json!({
                    "strict": e3::policy_verdict(d, e3::STRICT),
                    "lenient": e3::policy_verdict(d, e3::LENIENT),
                })
            })
        }).flatten();
        // Name the actual cause: a timeout and a malformed digest are both
        // "no digest", but only one of them says anything about the reviewer.
        // Arm (a) never adopts the card, so the arm (b) adoption check does
        // not apply to it; its equivalent is `arm_a_ok`.
        let adopted = if arm_a { arm_a_ok } else { adopted };
        let void_reason: Option<String> = match (adopted, scored.is_some()) {
            (true, true) => None,
            (false, _) if arm_a => Some(format!(
                "arm (a) did not dispatch the card: spawned={:?} dispatched_card={:?}",
                record.subagents_spawned,
                behaviour.as_ref().map(|b| b.dispatched_card)
            )),
            (false, _) => Some("agent card not adopted (F7)".into()),
            // A platform 500 is neither a reviewer failure nor a rig fault, and
            // must not be filed as one: `api_error_status` is on the result, so
            // the distinction is free. It still voids the record.
            (true, false) => Some(
                match record.meta.as_ref().and_then(|m| m.api_error_status.as_ref()) {
                    Some(code) if !code.is_null() => format!(
                        "platform error {code} — not a reviewer or rig fault; retry"
                    ),
                    _ => format!(
                        "no schema-valid digest ({})",
                        record.error_kind.as_deref().unwrap_or("unknown")
                    ),
                },
            ),
        };
        match (&scored, outcome.is_ok()) {
            (Some(s), _) => eprintln!(
                "pass={} strong={} weak={} cited={} {:.0}s",
                !s.rejected,
                s.strong_detection,
                s.weak_detection,
                s.matched_files.len(),
                record.wall_clock_ms as f64 / 1000.0
            ),
            (None, _) => eprintln!(
                "NO DIGEST ({}) {:.0}s",
                record.error_kind.as_deref().unwrap_or("?"),
                record.wall_clock_ms as f64 / 1000.0
            ),
        }

        journal.append(&serde_json::json!({
            "experiment": "E3",
            "resume_key": resume_key,
            "outcome": if scored.is_some() && adopted { "ok" } else { "void" },
            "void_reason": void_reason,
            "node_modules_linked": node_modules_linked,
            "ac_violated": ac_violated,
            "fix_unreachable": fix_unreachable,
            "arm": arm_label,
            "condition": condition,
            "schema": args.schema,
            "policy": policy,
            "slug": item.slug,
            "repeat": args.repeat,
            "checkpoint_sha": sha,
            "reverted": reverted,
            "review_cwd": review_cwd.to_string_lossy(),
            "card_adopted": adopted,
            "parent_behaviour": behaviour,
            "ground_truth": if condition == "clean" { "approved" } else { "changes-requested" },
            "scored": scored,
            "step": record,
        }))?;

        gate::reset_worktree(&worktree, &sha)?;
        processed += 1;
    }

    println!("\nwrote {}", journal.path().display());
    Ok(())
}

fn reconcile_replay(args: ReconcileArgs) -> Result<()> {
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    let journal = Journal::open(&args.out, &[])?;

    let commits = reconcile::read_commits(&redkiln)?;
    let (state, drift) = reconcile::reconcile(&commits);
    let axes = reconcile::compare_axes(&state);

    journal.append(&serde_json::json!({
        "record": "header", "experiment": "S0.2",
        "generated_at": chrono::Utc::now(),
        "corpus_pin": journal::read_pinned_sha(&args.pin_file),
        "redkiln_head": git_head(&redkiln),
        "commits_with_trailers": commits.len(),
    }))?;

    println!("commits carrying resume trailers : {}", commits.len());
    println!("projects reconstructed           : {}", state.projects.len());
    println!(
        "stories committed (project form) : {}",
        state.projects.values().map(|p| p.stories_committed.len()).sum::<usize>()
    );
    println!("bare story ids (fix lane)        : {}", state.bare_story_ids.len());
    println!("Fix: slugs                       : {}", state.fixes.len());

    println!("\n--- axis 1 (Story) vs axis 2 (Slice-Verdict) ---");
    println!("slices sealed                    : {}", axes.slices_total);
    println!("  latest verdict approved        : {}", axes.slices_approved);
    println!("  latest verdict REJECTED        : {}", axes.slices_rejected);
    println!("  sealed more than once          : {}", axes.slices_resealed);
    println!("  rejected then later approved   : {}", axes.rejected_then_approved);
    println!("\n--- what a Story-trailer-only resume would misread as done ---");
    if axes.slices_total > 0 {
        let pct = |n: usize| 100.0 * n as f64 / axes.slices_total as f64;
        println!(
            "  at the final snapshot      : {:>3}  ({:.1}%)",
            axes.would_be_misread_as_done,
            pct(axes.would_be_misread_as_done)
        );
        println!(
            "  MID-FLIGHT (ever rejected) : {:>3}  ({:.1}%)   <- the figure that matters",
            axes.ever_rejected,
            pct(axes.ever_rejected)
        );
        println!(
            "\n  Resume runs when a workflow is RE-ENTERED, not at the end of history —\n  \
             which is exactly when a slice is sitting in a rejected state."
        );
    }

    let mut kinds: std::collections::BTreeMap<&str, usize> = Default::default();
    for d in &drift {
        *kinds.entry(d.kind).or_default() += 1;
    }
    println!("\n--- drift (reported, never silently reconciled) ---");
    for (k, n) in &kinds {
        println!("  {k:<28} {n}");
    }

    journal.append(&serde_json::json!({"record": "axes", "axes": axes}))?;
    for d in &drift {
        journal.append(&serde_json::json!({"record": "drift", "drift": d}))?;
    }
    journal.append(&serde_json::json!({"record": "state", "state": state}))?;

    println!("\nwrote {}", journal.path().display());
    Ok(())
}

fn gate_probe(args: GateArgs) -> Result<()> {
    let condition: &'static str = match args.condition.as_str() {
        "s-fail" => "s-fail",
        "clean" => "clean",
        other => anyhow::bail!("unknown condition `{other}` (expected s-fail or clean)"),
    };
    let worktree = std::fs::canonicalize(&args.worktree).with_context(|| {
        format!(
            "--worktree {} does not exist; create it with `git worktree add`",
            args.worktree.display()
        )
    })?;
    let journal = Journal::open(&args.out, &[])?;

    // Resume: a stopped run costs the item in flight, never the whole run.
    let done = journal::completed_keys(&args.out);
    let items = read_manifest(&args.manifest)?;
    let mut processed = 0usize;

    for item in &items {
        if let Some(only) = &args.only
            && &item.slug != only
        {
            continue;
        }
        if let Some(limit) = args.limit
            && processed >= limit
        {
            break;
        }
        let resume_key = format!("S0.1/{condition}/{}", item.slug);
        if done.contains(&resume_key) {
            eprintln!("  {:<44} skipped (already recorded)", item.slug);
            continue;
        }
        let Some(sha) = item.checkpoint_sha.clone() else {
            continue;
        };

        let pick = |role| -> Vec<String> {
            item.fix_files
                .iter()
                .filter(|f| gate::classify(f) == role)
                .cloned()
                .collect()
        };
        let code = pick(gate::FileRole::Code);
        let tests = pick(gate::FileRole::Test);

        // A fix touching no production code cannot be mutated this way. That is
        // not a gate failure and must not enter the denominator.
        if condition == "s-fail" && code.is_empty() {
            eprintln!("  {:<44} EXCLUDED (no production code)", item.slug);
            journal.append(&gate::GateRecord {
                experiment: "S0.1",
                resume_key: resume_key.clone(),
                slug: item.slug.clone(),
                condition,
                checkpoint_sha: sha,
                code_files: code,
                test_files: tests,
                reverted: vec![],
                exit_code: None,
                failed_stage: None,
                stages_run: vec![],
                gate_passed: false,
                survived: false,
                duration_ms: 0,
                stdout_tail: String::new(),
                stderr_tail: String::new(),
                outcome: "skipped".into(),
                skipped_reason: Some("fix touches no production code".into()),
            })?;
            continue;
        }

        eprint!("  {:<44} {condition} ... ", item.slug);
        gate::reset_worktree(&worktree, &sha)?;
        let reverted = if condition == "s-fail" {
            gate::apply_mutation(&worktree, &sha, &code)?
        } else {
            Vec::new()
        };

        let run = gate::run_gate(&worktree)?;
        let passed = run.exit_code == Some(0);
        // A gate that never spawned cannot have a verdict. Guard added after
        // a shell-resolution bug made every mutant "die" in 4 seconds without
        // running, yielding a fake 100% mutation score.
        anyhow::ensure!(
            !run.stages_run.is_empty(),
            "the gate did not run for {} — refusing to record a verdict. stderr: {}",
            item.slug,
            run.stderr.trim()
        );
        let survived = condition == "s-fail" && passed;
        eprintln!(
            "exit={:?} {} at {} {:.0}s",
            run.exit_code,
            if survived {
                "SURVIVED"
            } else if passed {
                "pass"
            } else {
                "killed"
            },
            run.failed_stage.as_deref().unwrap_or("-"),
            run.duration_ms as f64 / 1000.0
        );
        journal.append(&gate::GateRecord {
            experiment: "S0.1",
            resume_key: resume_key.clone(),
            slug: item.slug.clone(),
            condition,
            checkpoint_sha: sha.clone(),
            code_files: code,
            test_files: tests,
            reverted,
            exit_code: run.exit_code,
            failed_stage: run.failed_stage.clone(),
            stages_run: run.stages_run.clone(),
            gate_passed: passed,
            survived,
            duration_ms: run.duration_ms,
            stdout_tail: run.stdout,
            stderr_tail: run.stderr,
            outcome: "ok".into(),
            skipped_reason: None,
        })?;
        gate::reset_worktree(&worktree, &sha)?;
        processed += 1;
    }

    summarise(&args.out);
    println!("wrote {}", journal.path().display());
    Ok(())
}

fn read_manifest(path: &std::path::Path) -> Result<Vec<ManifestItem>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading {} — run `corpus-manifest` first", path.display()))?;
    Ok(text
        .lines()
        .filter_map(|l| serde_json::from_str::<ManifestItem>(l).ok())
        .filter(|i| i.eligible)
        .collect())
}

/// Mutation score, with inapplicable mutants excluded from the denominator
/// (PRIOR-ART §5 — counting them would inflate the gate's apparent weakness).
fn summarise(out: &std::path::Path) {
    let text = std::fs::read_to_string(out).unwrap_or_default();
    let recs: Vec<serde_json::Value> = text
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .filter(|v: &serde_json::Value| v.get("experiment").and_then(|e| e.as_str()) == Some("S0.1"))
        .collect();

    let ran: Vec<_> = recs
        .iter()
        .filter(|r| r["condition"] == "s-fail" && r["outcome"] == "ok")
        .collect();
    let excluded = recs
        .iter()
        .filter(|r| r["condition"] == "s-fail" && r["outcome"] == "skipped")
        .count();
    let survived = ran.iter().filter(|r| r["survived"] == true).count();
    let clean: Vec<_> = recs.iter().filter(|r| r["condition"] == "clean").collect();
    let clean_pass = clean.iter().filter(|r| r["gate_passed"] == true).count();

    // Stage attribution is not decoration. A mutant killed at `typecheck` never
    // reached the tests: reverting code while keeping its test can leave a tree
    // that does not compile, which no real defect would. Counting those as
    // kills flatters the suite, so they are separated from the denominator that
    // actually answers "would the tests catch a silent regression?".
    let stage = |r: &&&serde_json::Value, s: &str| r["failed_stage"] == s;
    let by_typecheck = ran.iter().filter(|r| stage(r, "typecheck")).count();
    let by_lint = ran.iter().filter(|r| stage(r, "lint")).count();
    let by_test = ran.iter().filter(|r| stage(r, "test")).count();
    let reached_tests = by_test + survived;

    println!("\n--- S0.1: does the gate pass known-bad code? ---");
    println!("mutants run           : {}", ran.len());
    println!("excluded (no code)    : {excluded}");
    println!("killed at typecheck   : {by_typecheck}   (incoherent revert — never reached the tests)");
    println!("killed at lint        : {by_lint}");
    println!("killed at test        : {by_test}");
    println!("SURVIVED              : {survived}");
    if !ran.is_empty() {
        println!(
            "\nheadline (all mutants): {:.1}% killed",
            100.0 * (ran.len() - survived) as f64 / ran.len() as f64
        );
    }
    if reached_tests > 0 {
        println!(
            "TEST-REACHABLE        : {:.1}% killed, {:.1}% survived  (n={reached_tests})",
            100.0 * by_test as f64 / reached_tests as f64,
            100.0 * survived as f64 / reached_tests as f64
        );
        println!("  ^ this is the oracle figure E4 and E5 rest on.");
    }
    for r in ran.iter().filter(|r| r["survived"] == true) {
        println!("   SURVIVED: {}", r["slug"].as_str().unwrap_or("?"));
    }
    if !clean.is_empty() {
        println!(
            "clean controls        : {clean_pass}/{} passed (a failure here is a broken fixture, not a gate finding)",
            clean.len()
        );
    }
}

fn corpus_manifest(args: CorpusArgs) -> Result<()> {
    let redkiln = std::fs::canonicalize(&args.redkiln)
        .with_context(|| format!("--redkiln {} does not exist", args.redkiln.display()))?;

    // The corpus is frozen. If HEAD has moved off the pin, every downstream
    // record would silently describe a different tree than the one registered.
    let pinned = journal::read_pinned_sha(&args.pin_file);
    let head = git_head(&redkiln);
    match (&pinned, &head) {
        (Some(p), Some(h)) if p != h => anyhow::bail!(
            "redkiln HEAD is {h}, but the corpus is pinned at {p}. The corpus is frozen              for the duration; check out the pin or update it deliberately."
        ),
        (Some(_), Some(_)) => eprintln!("corpus pin verified"),
        _ => eprintln!("WARNING: could not verify the corpus pin"),
    }

    let items = corpus::scan_fix_lane(&redkiln)?;
    let journal = Journal::open(&args.out, &[])?;
    journal.append(&serde_json::json!({
        "record": "header", "experiment": "S0.3",
        "generated_at": chrono::Utc::now(),
        "redkiln_head": head, "corpus_pin": pinned,
        "items_found": items.len(),
    }))?;

    let eligible: Vec<_> = items.iter().filter(|i| i.eligible).collect();
    println!("{:<44} {:>4} {:>10} {:>6} eligible", "slug", "ACs", "sha", "files");
    for i in &items {
        println!(
            "{:<44} {:>4} {:>10} {:>6} {}",
            &i.slug[..i.slug.len().min(44)],
            i.ac_count,
            i.checkpoint_sha.as_deref().unwrap_or("-"),
            i.fix_files.len(),
            if i.eligible {
                "yes".to_string()
            } else {
                format!("NO - {}", i.ineligible_reason.as_deref().unwrap_or(""))
            },
        );
        journal.append(i)?;
    }

    println!(
        "
{} items scanned, {} eligible for S0.1 / E3.",
        items.len(),
        eligible.len()
    );
    println!("median ACs among eligible: {}", median_acs(&eligible));
    println!("wrote {}", journal.path().display());
    Ok(())
}

fn median_acs(items: &[&corpus::FixLaneItem]) -> usize {
    if items.is_empty() {
        return 0;
    }
    let mut counts: Vec<usize> = items.iter().map(|i| i.ac_count).collect();
    counts.sort_unstable();
    counts[counts.len() / 2]
}

/// One TTL dispatch — warm or read — in a short-lived process.
async fn ttl_step(claude_bin: &str, args: TtlStepArgs, label: &str) -> Result<()> {
    let env = hygiene::assert_clean(claude_bin).await?;
    let cwd = std::fs::canonicalize(&args.cwd)
        .with_context(|| format!("--cwd {} does not exist", args.cwd.display()))?;
    let journal = Journal::open(&args.out, std::slice::from_ref(&cwd))?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };

    // On a read, locate the mark first: if there is none, the dispatch would
    // measure nothing and is better refused than recorded as a gap of unknown
    // length.
    let prompt = match &args.prompt_file {
        Some(f) => std::fs::read_to_string(f)
            .with_context(|| format!("reading --prompt-file {}", f.display()))?,
        None => args.prompt.clone(),
    };
    let mark = journal::last_warm_mark(&args.out);
    if label == "read" && mark.is_none() {
        anyhow::bail!(
            "no successful `warm` mark in {} — run `ttl-mark` first",
            args.out.display()
        );
    }

    let arm = args
        .arm
        .clone()
        .or_else(|| mark.as_ref().map(|(a, _)| a.clone()))
        .unwrap_or_else(|| "ttl-open".into());

    journal.append(&RunHeader {
        record: "header",
        experiment: &args.experiment,
        arm: &arm,
        started_at: chrono::Utc::now(),
        preregistration_sha: git_head(std::path::Path::new(".")),
        corpus_sha: journal::read_pinned_sha(&args.pin_file),
        claude_version: env.claude_version.clone(),
        platform: env.platform.clone(),
    })?;

    let runner = Runner::new(claude_bin, env);
    let spec = StepSpec {
        experiment: args.experiment.clone(),
        arm: arm.clone(),
        label: label.into(),
        phase: "ttl".into(),
        occurrence: if label == "warm" { 1 } else { 2 },
        attempt: 1,
        model,
        effort: None,
        agent: None,
        adopt_agent: true,
        prompt: prompt.clone(),
        schema: None,
        allowed_tools: vec![],
        permission_mode: None,
        cwd: cwd.clone(),
        isolate_mcp: true,
        extra_flags: {
            let mut f = match &args.system_prompt {
                Some(p) => vec!["--system-prompt".into(), p.clone()],
                None => vec![],
            };
            // S-H1c (METHOD-S §4c): the append route needs
            // `--exclude-dynamic-system-prompt-sections --append-system-prompt <text>`,
            // which `probe` already takes via `--flag`; the same passthrough here.
            f.extend(args.flags.iter().cloned());
            f
        },
        timeout: Duration::from_secs(args.timeout_secs),
    };

    let (record, _) = runner.run(&spec).await;
    let share = fmt_share(record.cache_read_share);
    let creation = record
        .usage
        .as_ref()
        .and_then(|u| u.cache_creation_input_tokens)
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".into());
    let started = record.started_at;
    journal.append(&record)?;

    if label == "warm" {
        println!("marked {arm} at {}  cache={share}", started.format("%H:%M:%SZ"));
        println!("take the read with: ttl-read --cwd ... --out {}", args.out.display());
    } else if let Some((mark_arm, mark_at)) = mark {
        let elapsed = (started - mark_at).num_seconds() as f64 / 60.0;
        println!("mark      : {mark_arm} at {}", mark_at.format("%H:%M:%SZ"));
        println!("read      : {}", started.format("%H:%M:%SZ"));
        println!("gap       : {elapsed:.1} minutes  (MEASURED, not requested)");
        println!("cache     : share={share}  creation={creation}");
        println!(
            "
{}",
            if record.cache_read_share == Some(1.0) {
                "WARM — full hit, zero re-creation. The prefix survived the gap."
            } else {
                "NOT fully warm — part of the prefix expired and was re-created."
            }
        );
        journal.append(&serde_json::json!({
            "record": "ttl_pair", "arm": mark_arm,
            "mark_at": mark_at, "read_at": started,
            "gap_minutes": elapsed, "share": record.cache_read_share,
            "cache_creation": record.usage.as_ref().and_then(|u| u.cache_creation_input_tokens),
        }))?;
    }
    Ok(())
}

/// Warm a prefix, wait, read it back. Repeat per gap.
///
/// Each gap gets its **own** warm dispatch. Probing one warmed prefix at
/// increasing gaps would be wrong: every read refreshes the entry, so a
/// cumulative series measures a prefix that keeps being kept alive rather than
/// one decaying undisturbed.
async fn ttl_probe(claude_bin: &str, args: TtlArgs) -> Result<()> {
    let env = hygiene::assert_clean(claude_bin).await?;
    let cwd = std::fs::canonicalize(&args.cwd)
        .with_context(|| format!("--cwd {} does not exist", args.cwd.display()))?;
    let journal = Journal::open(&args.out, std::slice::from_ref(&cwd))?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };

    journal.append(&RunHeader {
        record: "header",
        experiment: &args.experiment,
        arm: "ttl",
        started_at: chrono::Utc::now(),
        preregistration_sha: git_head(std::path::Path::new(".")),
        corpus_sha: journal::read_pinned_sha(&args.pin_file),
        claude_version: env.claude_version.clone(),
        platform: env.platform.clone(),
    })?;

    let runner = Runner::new(claude_bin, env);
    let already_done = journal::completed_keys(&args.out);
    eprintln!(
        "[{}/ttl] gaps {:?} min in {}",
        args.experiment,
        args.gaps,
        cwd.display()
    );

    let mut results: Vec<(u64, Option<f64>)> = Vec::new();

    for gap in &args.gaps {
        let arm = format!("ttl-{gap}m");
        let mk = |label: &str, occurrence: u32| StepSpec {
            experiment: args.experiment.clone(),
            arm: arm.clone(),
            label: label.into(),
            phase: "ttl".into(),
            occurrence,
            attempt: 1,
            model,
            effort: None,
            agent: None,
            adopt_agent: true,
            prompt: args.prompt.clone(),
            schema: None,
            allowed_tools: vec![],
            permission_mode: None,
            cwd: cwd.clone(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(args.timeout_secs),
        };

        let warm = mk("warm", 1);
        let read = mk("read", 2);
        if already_done.contains(&read.resume_key()) {
            eprintln!("  gap {gap:>3}m  skipped (already recorded)");
            continue;
        }

        let (warm_rec, _) = runner.run(&warm).await;
        journal.append(&warm_rec)?;
        eprintln!(
            "  gap {gap:>3}m  warm  cache={}",
            fmt_share(warm_rec.cache_read_share)
        );

        tokio::time::sleep(Duration::from_secs(gap * 60)).await;

        let (read_rec, _) = runner.run(&read).await;
        let share = read_rec.cache_read_share;
        journal.append(&read_rec)?;
        eprintln!("  gap {gap:>3}m  read  cache={}", fmt_share(share));
        results.push((*gap, share));
    }

    println!("\ngap(min)  cache_read_share_after_gap");
    for (gap, share) in &results {
        println!("{gap:>8}  {}", fmt_share(*share));
    }
    println!(
        "\nA warm read at 10 minutes is the decisive one: it puts the TTL above 5m.\n\
         Interpret against FINDINGS F5 — the share is noisy, so a single gap proves nothing."
    );
    journal.append(&serde_json::json!({
        "record": "ttl_summary",
        "gaps": results.iter().map(|(g, s)| serde_json::json!({"gap_min": g, "share": s})).collect::<Vec<_>>(),
    }))?;

    println!("\nwrote {}", journal.path().display());
    Ok(())
}

fn fmt_share(share: Option<f64>) -> String {
    share
        .map(|s| format!("{s:.3}"))
        .unwrap_or_else(|| "n/a".into())
}

async fn doctor(claude_bin: &str) -> Result<()> {
    let env = hygiene::assert_clean(claude_bin).await?;
    println!("claude version   : {}", env.claude_version);
    println!("version >= floor : {}", env.claude_version_ok);
    println!("API key absent   : {}", env.api_key_absent);
    println!("platform         : {}", env.platform);
    println!("\nenvironment is fit for a measured run.");
    Ok(())
}

async fn probe(claude_bin: &str, args: ProbeArgs) -> Result<()> {
    let env = hygiene::assert_clean(claude_bin).await?;

    let cwd = std::fs::canonicalize(&args.cwd)
        .with_context(|| format!("--cwd {} does not exist", args.cwd.display()))?;
    let journal = Journal::open(&args.out, std::slice::from_ref(&cwd))?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };
    let effort = match args.effort.as_str() {
        "low" => Effort::Low,
        "medium" => Effort::Medium,
        "high" => Effort::High,
        "xhigh" => Effort::Xhigh,
        "max" => Effort::Max,
        other => anyhow::bail!("unknown effort `{other}`"),
    };

    journal.append(&RunHeader {
        record: "header",
        experiment: &args.experiment,
        arm: &args.arm,
        started_at: chrono::Utc::now(),
        preregistration_sha: git_head(std::path::Path::new(".")),
        corpus_sha: journal::read_pinned_sha(&args.pin_file),
        claude_version: env.claude_version.clone(),
        platform: env.platform.clone(),
    })?;

    let prompt = match &args.prompt_file {
        Some(f) => std::fs::read_to_string(f)
            .with_context(|| format!("reading --prompt-file {}", f.display()))?,
        None => args.prompt.clone(),
    };
    let runner = Runner::new(claude_bin, env);
    let margins = Margins {
        max_five_hour: args.max_five_hour,
        max_seven_day: args.max_seven_day,
    };
    let mut guard = Guard::default();
    let mut first_seen: Option<(f64, f64)> = None;
    let mut completed: u32 = 0;

    // Resume is always on, and needs no flag: re-running into the same results
    // file skips what already succeeded, and a genuinely fresh run is a fresh
    // file. An Opus arm interrupted by the invisible family sub-limit therefore
    // costs one dispatch, not the arm.
    let already_done = journal::completed_keys(&args.out);
    // Scoped to THIS arm. A shared results file holding other arms' records is
    // normal — E1 writes five arms into one file — and refusing on that basis
    // stopped arm (b) dead the moment arm (a) had written anything.
    let this_arm_done = already_done
        .iter()
        .any(|k| k.starts_with(&format!("{}/{}/", args.experiment, args.arm)));
    if this_arm_done && args.commit_between {
        anyhow::bail!(
            "arm `{}` already has completed steps in {}, and it commits between \
             dispatches. A cache series cannot be resumed: its premise is one \
             contiguous run against an evolving prefix, and splicing two halves \
             would look like a clean series while measuring something else. \
             Use a fresh arm label or a fresh results file.",
            args.arm,
            args.out.display()
        );
    }
    if !already_done.is_empty() {
        eprintln!(
            "resuming: {} step(s) already recorded ok in {}",
            already_done.len(),
            args.out.display()
        );
    }
    eprintln!(
        "[{}/{}] {} repeats in {}",
        args.experiment,
        args.arm,
        args.repeats,
        cwd.display()
    );

    for i in 1..=args.repeats {
        // Charter §12: hitting a declared ceiling pauses the experiment, not
        // the week. A partial arm that stopped for a stated reason is a result;
        // one that ran the window dry is a mess.
        let verdict = guard.check(margins);
        if verdict.is_halt() {
            eprintln!("\nhalting before repeat {i}: {}", verdict.reason());
            journal.append(&serde_json::json!({
                "record": "halt", "at_repeat": i, "reason": verdict.reason(),
            }))?;
            break;
        }

        // Checked before the commit: skipping a completed step must not advance
        // the fixture's git state, or a resumed arm would double-commit and the
        // snapshot series would no longer match the one being resumed.
        let resume_key = format!(
            "{}/{}/probe#{}:1:{}",
            args.experiment,
            args.arm,
            i,
            model.as_str()
        );
        if already_done.contains(&resume_key) {
            eprintln!("  {i:>3}/{}  skipped (already recorded)", args.repeats);
            continue;
        }

        // The commit lands BEFORE the dispatch, so run i sees a git snapshot
        // that differs from the one run i-1 started under. That ordering is the
        // whole arm: a commit after the last dispatch would prove nothing.
        if args.commit_between && i > 1 {
            empty_commit(&cwd, i)?;
        }

        let spec = StepSpec {
            experiment: args.experiment.clone(),
            arm: args.arm.clone(),
            label: "probe".into(),
            phase: args.arm.clone(),
            occurrence: i,
            attempt: 1,
            model,
            effort: Some(effort),
            agent: None,
            adopt_agent: true,
            prompt: prompt.clone(),
            schema: None,
            allowed_tools: vec![],
            permission_mode: None,
            cwd: cwd.clone(),
            isolate_mcp: !args.inherit_mcp,
            extra_flags: args.flags.clone(),
            timeout: Duration::from_secs(args.timeout_secs),
        };

        let (record, outcome) = runner.run(&spec).await;

        eprintln!(
            "  {i:>3}/{}  {:<8} cache={:<7} out={:<6} think={:<6} 7d={:<7} {}ms",
            args.repeats,
            record.outcome,
            record
                .cache_read_share
                .map(|s| format!("{s:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            record
                .usage
                .as_ref()
                .and_then(|u| u.output_tokens)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".into()),
            record
                .thinking_tokens
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".into()),
            record
                .headroom
                .as_ref()
                .and_then(|h| h.seven_day_utilization)
                .map(|v| format!("{:.3}", v))
                .unwrap_or_else(|| "n/a".into()),
            record.wall_clock_ms,
        );

        guard.observe(record.headroom.as_ref());
        if let Some(h) = &record.headroom
            && let (Some(f), Some(sd)) = (h.five_hour_utilization, h.seven_day_utilization)
        {
            first_seen.get_or_insert((f, sd));
        }
        completed += 1;
        journal.append(&record)?;

        // A usage limit is not something to retry or push through — charter
        // §6.3: headless has no wait-and-continue. Stop and leave the partial
        // arm recorded rather than burning the rest of the window.
        if let Err(step::StepError::UsageLimit { resets_at }) = &outcome {
            eprintln!(
                "\nplan usage limit reached at repeat {i}{}. Arm is PARTIAL.",
                resets_at
                    .as_ref()
                    .map(|r| format!(" (resets {r})"))
                    .unwrap_or_default()
            );
            break;
        }
    }

    // Per-dispatch cost is BELOW the instrument's 0.01 resolution, so the
    // honest unit is the batch. Reporting a per-run number here would invent
    // precision the platform does not give us.
    if let (Some((f0, s0)), Some(h)) = (first_seen, guard.last()) {
        for (window, first, last) in [
            ("five_hour", f0, h.five_hour_utilization),
            ("seven_day", s0, h.seven_day_utilization),
        ] {
            let Some(last) = last else { continue };
            let draw = throttle::batch_draw(window, first, last, completed);
            println!(
                "{window:<10} {first:.2} -> {last:.2}  delta={:.2} over {} dispatches{}",
                draw.delta,
                draw.dispatches,
                if draw.resolution_limited {
                    "  [resolution-limited: below the instrument, not free]"
                } else {
                    ""
                }
            );
            journal.append(&serde_json::json!({"record": "batch_draw", "draw": draw}))?;
        }
    }

    println!("\nwrote {}", journal.path().display());
    Ok(())
}

fn git_head(dir: &std::path::Path) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn empty_commit(cwd: &std::path::Path, i: u32) -> Result<()> {
    let out = std::process::Command::new("git")
        .args([
            "commit",
            "--allow-empty",
            "--no-gpg-sign",
            "-m",
            &format!("probe checkpoint {i}"),
        ])
        .current_dir(cwd)
        .output()
        .context("git commit failed to run")?;
    anyhow::ensure!(
        out.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&out.stderr).trim()
    );
    Ok(())
}

#[cfg(test)]
mod last_headroom_tests {
    use super::*;

    // The repo's own idiom (see journal.rs): a pid-scoped dir under temp_dir,
    // rather than a new dependency for two tests.
    fn results_file(tag: &str, resets_at: i64) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rk-e3-{tag}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        let path = dir.join("e3.jsonl");
        let line = serde_json::json!({
            "step": { "headroom": {
                "five_hour_utilization": 0.86,
                "seven_day_utilization": 0.21,
                "five_hour_resets_at": resets_at,
            }}
        });
        std::fs::write(&path, format!("{line}\n")).expect("write");
        path
    }

    #[test]
    fn a_reading_from_the_current_window_seeds_the_guard() {
        let path = results_file("live", 2_000);
        let h = last_headroom(&path, 1_000).expect("live reading");
        assert_eq!(h.five_hour_utilization, Some(0.86));
        std::fs::remove_dir_all(path.parent().unwrap()).ok();
    }

    #[test]
    fn a_reading_from_a_window_that_has_rolled_is_discarded() {
        // Otherwise a chunk resumed after a reset halts against a margin that
        // no longer applies, with a full window in front of it.
        let path = results_file("stale", 500);
        assert!(last_headroom(&path, 1_000).is_none());
        std::fs::remove_dir_all(path.parent().unwrap()).ok();
    }
}

#[derive(clap::Args)]
struct ClassifyArgs {
    /// The pre-registered item list, written before any dispatch (METHOD-P §1.2).
    #[arg(long, default_value = "../plans/p1-items.txt")]
    items: PathBuf,

    /// The frozen corpus repo. Read with `git show` only.
    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,

    /// The corpus pin. Every record carries it (METHOD-P §0 rule 2).
    #[arg(long, default_value = "47faec9f46b8d983c2c75f0794164e68415ae706")]
    sha: String,

    /// The classification card. Its digest goes on every record and it is not
    /// edited during P1 (METHOD-P §1.2).
    #[arg(long, default_value = "../prompts/p1-classify.md")]
    card: PathBuf,

    /// Committed schema, asserted equal to the built-in before the set runs.
    #[arg(long, default_value = "../schemas/p1-classify.schema.json")]
    schema_file: PathBuf,

    /// Results file. Append-only; resume is keyed per (arm, slug, repeat).
    #[arg(long)]
    out: PathBuf,

    /// Model tier for this pass. METHOD-P §1.2 runs haiku 4.5 and sonnet 5.
    #[arg(long, default_value = "haiku")]
    model: String,

    /// Instrument-variant tag carried in the arm label. #27: the §1.4
    /// anchor-rewrite re-run is a DIFFERENT instrument, so it runs as `L2` and
    /// an audit keyed on `arm` can never pool it with `L`.
    #[arg(long, default_value = "L")]
    arm_tag: String,

    /// Which repeat this pass is. Five per item per model = 10 draws per item.
    #[arg(long, default_value_t = 1)]
    repeat: u32,

    /// Working directory for the child. The card grants no file tools, so this
    /// only needs to be a real, inert directory, and never the corpus (rule 6).
    #[arg(long, default_value = "../fixtures/p1-classify-cwd")]
    cwd: PathBuf,

    /// Validate exactly one dispatch and stop. F20: validate one before the
    /// set, so a `void: schema` costs one draw instead of two hundred.
    #[arg(long)]
    probe_one: bool,

    /// Stop after this many dispatches.
    #[arg(long)]
    limit: Option<usize>,

    /// Halt below this five-hour headroom margin. A halt is a pause.
    #[arg(long, default_value_t = 0.90)]
    max_five_hour: f64,

    #[arg(long, default_value_t = 0.90)]
    max_seven_day: f64,

    /// Cumulative spend cap for the lane, in USD. METHOD-P: P1 arm L is $40.
    #[arg(long, default_value_t = 40.0)]
    max_cost_usd: f64,

    #[arg(long, default_value_t = 600)]
    timeout_secs: u64,
}

/// P1 arm L — classification stability (METHOD-P §1.2).
async fn classify_probe(claude_bin: &str, args: ClassifyArgs) -> Result<()> {
    let env = hygiene::assert_clean(claude_bin).await?;
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    std::fs::create_dir_all(&args.cwd)?;
    let cwd = std::fs::canonicalize(&args.cwd)?;
    let journal = Journal::open(&args.out, &[])?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };

    // The card is committed before the first dispatch and not edited during P1.
    // Its digest goes on every record, so a mid-run edit is detectable rather
    // than silent.
    let card_body = std::fs::read_to_string(&args.card)
        .with_context(|| format!("reading --card {}", args.card.display()))?;
    let card_sha = fingerprint::sha256_hex(card_body.as_bytes());
    let schema = p1::classify_schema();

    // The binary must not drift from the committed schema file.
    let disk: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&args.schema_file)
            .with_context(|| format!("reading --schema-file {}", args.schema_file.display()))?,
    )?;
    let disk_keys = disk["properties"]
        .as_object()
        .map(|o| o.keys().cloned().collect::<Vec<_>>());
    let built_keys = schema["properties"]
        .as_object()
        .map(|o| o.keys().cloned().collect::<Vec<_>>());
    anyhow::ensure!(
        disk["required"] == schema["required"] && disk_keys == built_keys,
        "{} has drifted from the built-in classification schema",
        args.schema_file.display()
    );
    let schema_sha = fingerprint::sha256_hex(schema.to_string().as_bytes());

    let margins = Margins {
        max_five_hour: args.max_five_hour,
        max_seven_day: args.max_seven_day,
    };
    let mut guard = Guard::default();
    guard.observe(last_headroom(&args.out, chrono::Utc::now().timestamp()).as_ref());

    let runner = Runner::new(claude_bin, env);
    let done = journal::completed_keys(&args.out);
    let items = p1::read_items(&args.items)?;
    let arm = format!("{}-{}", args.arm_tag, args.model);

    // Spend is cumulative across every pass that wrote to this file, so a cap
    // cannot be evaded by splitting the run into chunks.
    let mut spent = spend_so_far(&args.out);
    eprintln!(
        "P1 arm L: {} items, model {}, repeat {} | card {} schema {} | spent ${:.2} of ${:.2}",
        items.len(),
        args.model,
        args.repeat,
        &card_sha[..12],
        &schema_sha[..12],
        spent,
        args.max_cost_usd
    );

    let mut processed = 0usize;
    for item in &items {
        if let Some(limit) = args.limit
            && processed >= limit
        {
            break;
        }
        let resume_key = format!("P1/{arm}/{}#r{}", item.slug, args.repeat);
        if done.contains(&resume_key) {
            eprintln!("  {:<58} skipped (recorded)", item.slug);
            continue;
        }
        if spent >= args.max_cost_usd {
            eprintln!(
                "\n  SPEND HALT before {}: ${:.2} over the ${:.2} cap. {} item(s) not dispatched.",
                item.slug,
                spent,
                args.max_cost_usd,
                items.len().saturating_sub(processed)
            );
            break;
        }
        match guard.check(margins) {
            throttle::Verdict::Proceed => {}
            verdict => {
                eprintln!("\n  HALT before {}: {}", item.slug, verdict.reason());
                break;
            }
        }

        let raw = p1::card_text(&redkiln, &args.sha, &item.card)?;
        let redacted = p1::redact_kind(&raw);
        let prompt = p1::build_prompt(&item.slug, &redacted);

        eprint!("  {:<58} {arm} r{} ... ", item.slug, args.repeat);
        let spec = StepSpec {
            experiment: "P1".into(),
            arm: arm.clone(),
            label: item.slug.clone(),
            phase: "classify".into(),
            occurrence: args.repeat,
            attempt: 1,
            model,
            // METHOD-P §1.2 pins default temperature and says nothing about
            // effort; haiku rejects the parameter outright, so it is left unset
            // for both tiers rather than varied silently between them.
            effort: None,
            adopt_agent: true,
            agent: Some(step::AgentCard {
                name: "p1-classify".into(),
                description:
                    "Triage classification for the Redkiln process-catalog selection engine."
                        .into(),
                prompt: card_body.clone(),
                // F20: StructuredOutput is the digest channel. Nothing else,
                // because the classification is a judgement about the prompt.
                tools: Some(p1::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                model: Some(args.model.clone()),
                effort: None,
                permission_mode: None,
            }),
            prompt,
            schema: Some(schema.clone()),
            allowed_tools: vec![],
            permission_mode: Some("bypassPermissions".into()),
            cwd: cwd.clone(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(args.timeout_secs),
        };

        let (record, _outcome) = runner.run(&spec).await;
        guard.observe(record.headroom.as_ref());

        // Tool-set equality, as F17/F20 established: `effective.agent` is not
        // populated when the card is applied with `--agent`, so adoption is
        // read off the tool set the child actually reports.
        let declared: BTreeSet<&str> = p1::CARD_TOOLS.into_iter().collect();
        let adopted = record
            .effective
            .as_ref()
            .and_then(|e| e.tools.as_ref())
            .is_some_and(|t| t.iter().map(|x| x.as_str()).collect::<BTreeSet<_>>() == declared);

        let valid = record.digest.as_ref().is_some_and(p1::valid_record);
        let vector = record
            .digest
            .as_ref()
            .filter(|d| p1::valid_record(d))
            .map(p1::vector);
        let void_reason: Option<String> = match (adopted, valid) {
            (true, true) => None,
            (false, _) => Some("agent card not adopted (F7/F20)".into()),
            (true, false) => Some(
                match record.meta.as_ref().and_then(|m| m.api_error_status.as_ref()) {
                    Some(code) if !code.is_null() => {
                        format!("platform error {code}, not a model or rig fault; retry")
                    }
                    // The F20 shape exactly: the model spoke, in prose.
                    _ => format!(
                        "schema ({})",
                        record
                            .error_kind
                            .as_deref()
                            .unwrap_or("no schema-valid record")
                    ),
                },
            ),
        };

        let cost = record.total_cost_usd.unwrap_or(0.0);
        spent += cost;
        match &vector {
            Some(v) => eprintln!(
                "{}/{}/{}/{} kind={} ${:.4} {:.0}s",
                v["criticality"].as_str().unwrap_or("?"),
                v["novelty"].as_str().unwrap_or("?"),
                v["size"].as_str().unwrap_or("?"),
                v["blast_radius"].as_str().unwrap_or("?"),
                v["kind"].as_str().unwrap_or("?"),
                cost,
                record.wall_clock_ms as f64 / 1000.0
            ),
            None => eprintln!(
                "VOID ({}) ${:.4} {:.0}s",
                void_reason.as_deref().unwrap_or("?"),
                cost,
                record.wall_clock_ms as f64 / 1000.0
            ),
        }

        journal.append(&serde_json::json!({
            "experiment": "P1",
            "arm": arm,
            "resume_key": resume_key,
            "outcome": if void_reason.is_none() { "ok" } else { "void" },
            "void_reason": void_reason,
            "card_adopted": adopted,
            "item": item.slug,
            "item_kind_declared": item.kind,
            "card_path": item.card,
            "slug_hash": item.hash,
            "model": args.model,
            "repeat": args.repeat,
            "corpus_sha": args.sha,
            "card_sha256": card_sha,
            "schema_sha256": schema_sha,
            "vector": vector,
            "digest": record.digest,
            "step": record,
        }))?;

        processed += 1;
        if args.probe_one {
            eprintln!("\n  --probe-one: stopping after one dispatch (F20 pre-flight).");
            break;
        }
    }

    eprintln!(
        "\n  spend in this file: ${:.2} of ${:.2}",
        spent, args.max_cost_usd
    );
    println!("wrote {}", journal.path().display());
    Ok(())
}

/// Cumulative `total_cost_usd` already recorded in a results file.
///
/// Read from the file rather than counted in memory, so a cap holds across the
/// chunked runs these lanes are actually executed in.
fn spend_so_far(path: &std::path::Path) -> f64 {
    let Ok(text) = std::fs::read_to_string(path) else {
        return 0.0;
    };
    text.lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter_map(|r| {
            r.get("step")
                .and_then(|s| s.get("total_cost_usd"))
                .and_then(|c| c.as_f64())
        })
        .sum()
}

#[derive(clap::Args)]
struct GraphArgs {
    /// The pre-registered item list, written before the first dispatch
    /// (METHOD-P §4.2): 10 S-SILENT fixtures by hash order of slug.
    #[arg(long, default_value = "../plans/p7-items.txt")]
    items: PathBuf,

    /// The answer key and checkpoint SHAs (METHOD-E3's own manifest).
    #[arg(long, default_value = "../results/s03-fixlane-manifest.jsonl")]
    manifest: PathBuf,

    /// The activity graph under test. Its shape is asserted, not assumed.
    #[arg(long, default_value = "../p9/catalog/root-cause.toml")]
    graph: PathBuf,

    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,

    #[arg(long, default_value = "../fixtures/s01-wt")]
    worktree: PathBuf,

    #[arg(long)]
    out: PathBuf,

    /// Rigor arm: optional | recommended | mandatory (METHOD-P §4.2).
    #[arg(long, default_value = "recommended")]
    arm: String,

    #[arg(long, default_value_t = 1)]
    repeat: u32,

    /// Model tier for the investigator and the adversary. METHOD-P §4.5 pins
    /// Opus for both. The catalog carries no `model` key — dispatch identity is
    /// `role` — so the tier is the driver's choice, and is recorded.
    #[arg(long, default_value = "opus")]
    model: String,

    #[arg(long, default_value = "xhigh")]
    effort: String,

    /// Run one graph and stop — this lane's F20 pre-flight.
    #[arg(long)]
    probe_one: bool,

    #[arg(long)]
    only: Option<String>,

    #[arg(long)]
    limit: Option<usize>,

    #[arg(long, default_value_t = 0.90)]
    max_five_hour: f64,

    #[arg(long, default_value_t = 0.90)]
    max_seven_day: f64,

    /// Cumulative spend cap. METHOD-P §4.5: P7 halts at $200.
    #[arg(long, default_value_t = 200.0)]
    max_cost_usd: f64,

    #[arg(long, default_value_t = 1800)]
    timeout_secs: u64,

    /// Hard bound on the `confirm` step's test run (#31). The catalog declares
    /// `timeout_secs = "${toolchain.test_timeout}"` on that step; without a
    /// bound one run took 28 minutes and stalled the arm.
    #[arg(long, default_value_t = 420)]
    confirm_timeout_secs: u64,
}

/// One line of `p7-items.txt`.
struct P7Item {
    slug: String,
    answer_key: Vec<String>,
}

fn read_p7_items(path: &std::path::Path) -> Result<Vec<P7Item>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading --items {}", path.display()))?;
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        anyhow::ensure!(f.len() >= 4, "malformed p7 items line: {line}");
        out.push(P7Item {
            slug: f[0].to_string(),
            answer_key: f[2].split(',').map(str::to_string).collect(),
        });
    }
    anyhow::ensure!(!out.is_empty(), "no items in {}", path.display());
    Ok(out)
}

/// The key every step record carries. A loop-back re-enters `hypothesise`, so
/// the pass number is part of the key or the second entry would collide with
/// the first.
fn step_key(activity: &str, step: &str, item: &str, arm: &str, repeat: u32, pass: u32) -> String {
    let s = format!("{activity}/{step}/{item}/{arm}/r{repeat}/p{pass}");
    format!("{}:{}", &fingerprint::sha256_hex(s.as_bytes())[..12], s)
}

/// P7 — the encapsulated `adversary -> policy -> gate` block, priced against
/// what it catches (METHOD-P §4).
async fn graph_probe(claude_bin: &str, args: GraphArgs) -> Result<()> {
    let arm = p7::Arm::parse(&args.arm)?;
    let g = p7::guard_graph(&args.graph)?;
    let env = hygiene::assert_clean(claude_bin).await?;
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    let worktree = std::fs::canonicalize(&args.worktree)?;
    let journal = Journal::open(&args.out, &[])?;

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };
    let effort = match args.effort.as_str() {
        "low" => Effort::Low,
        "medium" => Effort::Medium,
        "high" => Effort::High,
        "xhigh" => Effort::Xhigh,
        "max" => Effort::Max,
        other => anyhow::bail!("unknown effort `{other}`"),
    };

    let margins = Margins {
        max_five_hour: args.max_five_hour,
        max_seven_day: args.max_seven_day,
    };
    let mut guard = Guard::default();
    guard.observe(last_headroom(&args.out, chrono::Utc::now().timestamp()).as_ref());

    let runner = Runner::new(claude_bin, env);
    let done = journal::completed_keys(&args.out);
    let items = read_p7_items(&args.items)?;
    let manifest = read_manifest(&args.manifest)?;
    let hyp_schema = p7::hypothesis_schema();
    let adv_schema = e3::typed_findings_schema();
    // #22 (revised): the adversary card is purpose-built for this step. The
    // catalog's `adversary` takes `log` and `report` as inputs — it reviews a
    // CLAIM, not an implementation against acceptance criteria, which is what
    // the production `redkiln-review` card is written for.

    let mut spent = spend_so_far(&args.out);
    eprintln!(
        "P7 — {} items, arm {}, repeat {}, {}/{} | graph {} v{} {} | loopback max {} | spent ${:.2} of ${:.2}",
        items.len(), arm.as_str(), args.repeat, args.model, args.effort,
        g.activity_id, g.version, &g.sha256[..12], g.loopback_max, spent, args.max_cost_usd
    );
    if arm == p7::Arm::Mandatory && g.max_rigor.as_deref() == Some("recommended") {
        eprintln!(
            "  NOTE (#7): max_rigor=recommended, so the engine would never SELECT this activity at \
             mandatory. This arm prices the block, not a reachable configuration."
        );
    }

    let mut processed = 0usize;
    for item in &items {
        if let Some(only) = &args.only
            && &item.slug != only
        {
            continue;
        }
        if let Some(limit) = args.limit
            && processed >= limit
        {
            break;
        }
        let run_key = format!("P7/{}/{}#r{}", arm.as_str(), item.slug, args.repeat);
        if done.contains(&run_key) {
            eprintln!("  {:<44} skipped (recorded)", item.slug);
            continue;
        }
        if spent >= args.max_cost_usd {
            eprintln!(
                "\n  SPEND HALT before {}: ${:.2} over the ${:.2} cap. {} item(s) not dispatched.",
                item.slug, spent, args.max_cost_usd,
                items.len().saturating_sub(processed)
            );
            break;
        }
        match guard.check(margins) {
            throttle::Verdict::Proceed => {}
            verdict => {
                eprintln!("\n  HALT before {}: {}", item.slug, verdict.reason());
                break;
            }
        }

        let Some(m) = manifest.iter().find(|m| m.slug == item.slug) else {
            eprintln!("  {:<44} SKIPPED (not in manifest)", item.slug);
            continue;
        };
        let Some(sha) = m.checkpoint_sha.clone() else { continue };

        // F17: the fix must be UNREACHABLE, not merely reverted.
        let fixture = worktree
            .parent()
            .unwrap_or(&worktree)
            .join(format!("e3-silent-{}", item.slug));
        let node_modules_linked = e3::build_silent_fixture(&redkiln, &fixture, &sha)?;
        e3::assert_fix_unreachable(&fixture, &sha)?;
        if !node_modules_linked {
            anyhow::bail!("no node_modules for {} — it could not run tests", item.slug);
        }

        let read = |n: &str| std::fs::read_to_string(redkiln.join(&m.dir).join(n)).unwrap_or_default();
        let spec_md = read("spec.md");
        let report_md = read("implementation-report.md");
        let arm_label = format!("{}-{}", arm.as_str(), args.model);

        eprintln!("  {} [{}] r{}", item.slug, arm.as_str(), args.repeat);

        // ---- graph state --------------------------------------------------
        let mut hyp_passes = 0u32;      // investigator dispatches ONLY (#25)
        let mut dispatches = 0u32;      // every model dispatch, against attempts_total
        let mut gate_arrivals = 0u32;   // `no-hypothesis` fires (#4)
        let mut loop_backs = 0u32;      // `policy` false -> hypothesise
        let mut run_cost = 0.0f64;
        let mut hypothesis: Option<serde_json::Value> = None;
        let mut findings: Option<serde_json::Value> = None;
        let mut gate_answer: Option<String> = None;
        let mut test_run: Option<p7::TestRun> = None;
        let mut policy_pass = true;
        let mut policy_verdict: Option<serde_json::Value> = None;
        let mut void_reason: Option<String> = None;
        let mut deviation: Option<serde_json::Value> = None;

        // `budget = { attempts_total = 8 }` on the activity is a real ceiling.
        const ATTEMPTS_TOTAL: u32 = 8;

        'graph: loop {
            // ---- hypothesise: repeat until confirmed (catalog max 3) -------
            let mut passes_this_entry = 0i64;
            loop {
                if dispatches >= ATTEMPTS_TOTAL {
                    void_reason = Some("activity budget attempts_total (8) exhausted".into());
                    break 'graph;
                }
                let pass = hyp_passes + 1;
                let prompt = p7::build_investigator_prompt(
                    &item.slug, &spec_md, &report_md,
                    hypothesis.as_ref(), findings.as_ref(), gate_answer.as_deref(),
                );
                eprint!("    hypothesise p{pass} ... ");
                let (rec, _) = runner.run(&StepSpec {
                    experiment: "P7".into(),
                    arm: arm_label.clone(),
                    label: format!("{}#hypothesise", item.slug),
                    phase: "hypothesise".into(),
                    occurrence: args.repeat,
                    attempt: pass,
                    model,
                    effort: Some(effort),
                    adopt_agent: true,
                    agent: Some(step::AgentCard {
                        name: "investigator".into(),
                        description: "Finds where a fault originates and how it could be discriminated.".into(),
                        prompt: p7::INVESTIGATOR_CARD.into(),
                        tools: Some(e3::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                        model: Some(args.model.clone()),
                        effort: Some(args.effort.clone()),
                        permission_mode: None,
                    }),
                    prompt,
                    schema: Some(hyp_schema.clone()),
                    allowed_tools: vec![],
                    permission_mode: Some("bypassPermissions".into()),
                    cwd: fixture.clone(),
                    isolate_mcp: true,
                    extra_flags: vec![],
                    timeout: Duration::from_secs(args.timeout_secs),
                }).await;
                guard.observe(rec.headroom.as_ref());
                hyp_passes += 1;
                dispatches += 1;
                passes_this_entry += 1;
                let cost = rec.total_cost_usd.unwrap_or(0.0);
                run_cost += cost;
                spent += cost;

                let digest = rec.digest.clone();
                let status = digest.as_ref()
                    .and_then(|d| d.get("status")).and_then(|v| v.as_str())
                    .unwrap_or("").to_string();
                eprintln!("{} ${:.3} {:.0}s",
                    if status.is_empty() { "NO DIGEST" } else { &status },
                    cost, rec.wall_clock_ms as f64 / 1000.0);

                if let Some(d) = digest.as_ref()
                    && let Some(rd) = d.get("request_deviation")
                    && !rd.is_null()
                {
                    deviation = Some(rd.clone());
                }
                journal.append(&serde_json::json!({
                    "experiment": "P7", "phase": "hypothesise",
                    "step_key": step_key(&g.activity_id, "hypothesise", &item.slug, arm.as_str(), args.repeat, pass),
                    "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                    "arm_label": arm_label, "repeat": args.repeat, "pass": pass,
                    "model": args.model, "effort": args.effort,
                    "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
                    "corpus_sha": sha, "digest": digest, "step": rec,
                }))?;

                if digest.is_none() {
                    void_reason = Some("hypothesise returned no schema-valid record (F20)".into());
                    break 'graph;
                }
                hypothesis = digest;
                if status == "confirmed" {
                    break;
                }
                // The `no-hypothesis` gate: `when status != 'confirmed'`. It
                // fires in EVERY arm (#4) and is auto-answered — P7 measures
                // catch-per-token, and gate behaviour is P8's question.
                gate_arrivals += 1;
                gate_answer = Some(AUTO_GATE_ANSWER.into());
                journal.append(&serde_json::json!({
                    "experiment": "P7", "phase": "no-hypothesis",
                    "step_key": step_key(&g.activity_id, "no-hypothesis", &item.slug, arm.as_str(), args.repeat, pass),
                    "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                    "repeat": args.repeat, "pass": pass, "kind": "gate",
                    "question": "root-cause.no-hypothesis",
                    "auto_answered": AUTO_GATE_ANSWER,
                    "arrival": gate_arrivals,
                    "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
                }))?;
                if passes_this_entry >= g.hypothesise_repeat_max {
                    break; // on_exhausted = escalate; the gate above is it
                }
            }

            // ---- confirm: a real test run in the fixture -------------------
            let selection = hypothesis.as_ref()
                .and_then(|h| h.get("discriminating_test"))
                .and_then(|v| v.as_str()).unwrap_or("");
            eprint!("    confirm ... ");
            let t = p7::run_selected_test(&fixture, selection, args.confirm_timeout_secs);
            eprintln!("{} ({} ms)",
                if !t.spawned { "not run" } else if t.passed { "passed" } else { "failed" },
                t.duration_ms);
            journal.append(&serde_json::json!({
                "experiment": "P7", "phase": "confirm",
                "step_key": step_key(&g.activity_id, "confirm", &item.slug, arm.as_str(), args.repeat, hyp_passes.max(1)),
                "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                "repeat": args.repeat, "kind": "tool", "run": "toolchain:test",
                "test": t,
                "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
            }))?;
            test_run = Some(t);

            // ---- adversary: when rigor >= recommended ---------------------
            if arm.runs_adversary() {
                if dispatches >= ATTEMPTS_TOTAL {
                    void_reason = Some("activity budget attempts_total (8) exhausted".into());
                    break 'graph;
                }
                let prompt = p7::build_adversary_prompt(
                    &item.slug,
                    hypothesis.as_ref().unwrap_or(&serde_json::Value::Null),
                    test_run.as_ref().unwrap(),
                );
                eprint!("    adversary ... ");
                let (rec, _) = runner.run(&StepSpec {
                    experiment: "P7".into(),
                    arm: arm_label.clone(),
                    label: format!("{}#adversary", item.slug),
                    phase: "adversary".into(),
                    occurrence: args.repeat,
                    attempt: loop_backs + 1,
                    model,
                    effort: Some(effort),
                    adopt_agent: true,
                    agent: Some(step::AgentCard {
                        name: "adversary".into(),
                        description: "Tries to break a proposed root cause, and classifies what it finds by evidence class.".into(),
                        prompt: p7::ADVERSARY_CARD.into(),
                        tools: Some(e3::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                        model: Some(args.model.clone()),
                        effort: Some(args.effort.clone()),
                        permission_mode: None,
                    }),
                    prompt,
                    schema: Some(adv_schema.clone()),
                    allowed_tools: vec![],
                    permission_mode: Some("bypassPermissions".into()),
                    cwd: fixture.clone(),
                    isolate_mcp: true,
                    extra_flags: vec![],
                    timeout: Duration::from_secs(args.timeout_secs),
                }).await;
                guard.observe(rec.headroom.as_ref());
                // #25: counts against the activity budget, NOT against
                // `hypothesise_passes` — conflating them made a single-pass
                // recommended run report `passes=2, gate_arrivals=0`.
                dispatches += 1;
                let cost = rec.total_cost_usd.unwrap_or(0.0);
                run_cost += cost;
                spent += cost;
                eprintln!("{} ${:.3} {:.0}s",
                    rec.digest.as_ref().map_or("NO DIGEST".to_string(), |d| format!(
                        "{} finding(s)",
                        d.get("findings").and_then(|f| f.as_array()).map_or(0, |a| a.len()))),
                    cost, rec.wall_clock_ms as f64 / 1000.0);
                journal.append(&serde_json::json!({
                    "experiment": "P7", "phase": "adversary",
                    "step_key": step_key(&g.activity_id, "adversary", &item.slug, arm.as_str(), args.repeat, loop_backs + 1),
                    "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                    "arm_label": arm_label, "repeat": args.repeat, "model": args.model,
                    "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
                    "digest": rec.digest, "step": rec,
                }))?;
                findings = rec.digest.clone();
            }

            // ---- policy: findings.none(evidence in [Reproduced, MutationSurvived])
            // e3::LENIENT blocks on exactly those two classes; STRICT would
            // also block on `narrowed-pin`, which the catalog's expression does
            // not name. The classes differ only in casing.
            let v = findings.as_ref().map(|f| e3::policy_verdict(f, e3::LENIENT));
            policy_pass = v.as_ref().is_none_or(|v| !v.blocks);
            policy_verdict = v.as_ref().map(|v| serde_json::to_value(v).unwrap_or_default());
            eprintln!("    policy   ... {}", if policy_pass { "pass" } else { "FALSE" });
            journal.append(&serde_json::json!({
                "experiment": "P7", "phase": "policy",
                "step_key": step_key(&g.activity_id, "policy", &item.slug, arm.as_str(), args.repeat, loop_backs + 1),
                "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                "repeat": args.repeat, "kind": "check",
                "expr": "findings.none(evidence in ['Reproduced', 'MutationSurvived'])",
                "policy_mapping": "e3::LENIENT (blocks on reproduced | mutation-survived)",
                "pass": policy_pass, "verdict": policy_verdict,
                "loop_backs_so_far": loop_backs,
                "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
            }))?;

            if policy_pass || !arm.loops_back() || (loop_backs as i64) >= g.loopback_max {
                break 'graph;
            }
            loop_backs += 1;
            eprintln!("    loop-back {} of {} (policy false)", loop_backs, g.loopback_max);
        }

        // ---- signoff: when rigor >= mandatory, auto-answered `accept` ------
        if arm.runs_signoff() && void_reason.is_none() {
            journal.append(&serde_json::json!({
                "experiment": "P7", "phase": "signoff",
                "step_key": step_key(&g.activity_id, "signoff", &item.slug, arm.as_str(), args.repeat, 1),
                "run_key": run_key, "item": item.slug, "arm": arm.as_str(),
                "repeat": args.repeat, "kind": "gate", "question": "root-cause.accept",
                "auto_answered": "accept",
                "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256 },
            }))?;
        }

        // ---- emit: the run's result line ----------------------------------
        let scored = hypothesis.as_ref().map(|h| p7::score(h, &item.answer_key));
        if let Some(s) = &scored {
            eprintln!(
                "    emit     ... root cause {} | {} pass(es), {} gate arrival(s), {} loop-back(s), ${:.2}",
                if s.correct { "CORRECT" } else { "missed" },
                hyp_passes, gate_arrivals, loop_backs, run_cost
            );
        }
        journal.append(&serde_json::json!({
            "experiment": "P7", "phase": "emit",
            "resume_key": run_key, "run_key": run_key,
            "outcome": if void_reason.is_none() && scored.is_some() { "ok" } else { "void" },
            "void_reason": void_reason,
            "item": item.slug, "arm": arm.as_str(), "arm_label": arm_label,
            "repeat": args.repeat, "model": args.model, "effort": args.effort,
            "graph": { "id": g.activity_id, "version": g.version, "sha256": g.sha256,
                       "max_rigor": g.max_rigor, "loopback_max": g.loopback_max },
            "engine_would_select_at_this_rigor":
                !(arm == p7::Arm::Mandatory && g.max_rigor.as_deref() == Some("recommended")),
            "answer_key": item.answer_key,
            "scored": scored,
            "hypothesis": hypothesis,
            "request_deviation": deviation,
            "deviation_ok_declared": ["add", "downgrade"],
            "test": test_run,
            "policy_pass": policy_pass,
            "policy_verdict": policy_verdict,
            "hypothesise_passes": hyp_passes,
            "model_dispatches": dispatches,
            "no_hypothesis_arrivals": gate_arrivals,
            "loop_backs": loop_backs,
            "run_cost_usd": run_cost,
            "corpus_sha": sha,
            "fix_unreachable": true,
            "node_modules_linked": node_modules_linked,
            "review_cwd": fixture.to_string_lossy(),
        }))?;

        gate::reset_worktree(&worktree, &sha).ok();
        processed += 1;
        if args.probe_one {
            eprintln!("\n  --probe-one: stopping after one graph run (F20 pre-flight).");
            break;
        }
    }

    eprintln!("\n  spend in this file: ${:.2} of ${:.2}", spent, args.max_cost_usd);
    println!("wrote {}", journal.path().display());
    Ok(())
}

/// What the `no-hypothesis` gate is auto-answered with (#4).
///
/// Deliberately carries no hint about the fault — an answer that pointed at the
/// defect would be the leak F17 already cost this programme once.
const AUTO_GATE_ANSWER: &str = "Continue with the evidence you have. State your best hypothesis and \
                                mark it `unconfirmed` if that is what it is.";

#[derive(clap::Args)]
struct ReproArgs {
    /// Which half of the experiment this invocation is.
    ///
    /// `run` drives to the gate mark and then BLOCKS holding its state in
    /// memory, so an external kill is a real kill. `resume` is a fresh process
    /// that must rebuild everything from disk — writing state and reading it
    /// back in-process would prove nothing (F19, `human_gate.rs`).
    #[arg(long, default_value = "run")]
    phase: String,

    #[arg(long, default_value = "../plans/p8-items.txt")]
    items: PathBuf,

    #[arg(long, default_value = "../results/s03-fixlane-manifest.jsonl")]
    manifest: PathBuf,

    #[arg(long, default_value = "../p9/catalog/minimal-repro.toml")]
    graph: PathBuf,

    #[arg(long, default_value = "D:/repos/redkiln")]
    redkiln: PathBuf,

    #[arg(long, default_value = "../fixtures/s01-wt")]
    worktree: PathBuf,

    /// Where suspended graph state lives between the kill and the resume.
    #[arg(long, default_value = "../results/p8-state")]
    state_dir: PathBuf,

    #[arg(long)]
    out: PathBuf,

    /// The delay arm this run belongs to: `0-delay`, `65-min`, `24-h`, or
    /// `control` for the no-kill runs (METHOD-P §5.2).
    #[arg(long, default_value = "0-delay")]
    arm: String,

    #[arg(long, default_value_t = 1)]
    repeat: u32,

    /// METHOD-P §5.2 pins haiku for P8.
    #[arg(long, default_value = "haiku")]
    model: String,

    #[arg(long)]
    only: Option<String>,

    /// `answer` phase: the gate answer to record. §9b.15 makes `answer` the
    /// gate's only output, and free text is forbidden, so this is an option id.
    #[arg(long, default_value = "provide-repro-steps")]
    answer: String,

    /// How long `run` blocks at the mark before giving up on being killed.
    #[arg(long, default_value_t = 900)]
    block_secs: u64,

    /// Suspend at the triage gate even when `attempt` reported `reproduced`
    /// (#36). METHOD-P §5.2's fixture — "repro steps removed so the gate fires"
    /// — does not fire: measured, three redactions and a difficulty-selected
    /// item set all reproduced on attempt 1. §5.3's questions are about the
    /// resume MECHANISM, not the gate's firing condition, so the `when`
    /// predicate is overridden and nothing else is. The state written, killed
    /// and reloaded is the investigator's real output; every record carries
    /// `gate_forced`.
    #[arg(long)]
    force_gate: bool,

    #[arg(long, default_value_t = 0.90)]
    max_five_hour: f64,

    #[arg(long, default_value_t = 0.90)]
    max_seven_day: f64,

    /// Cumulative spend cap. METHOD-P: P8 is $40.
    #[arg(long, default_value_t = 40.0)]
    max_cost_usd: f64,

    #[arg(long, default_value_t = 900)]
    timeout_secs: u64,
}

fn p8_state_path(dir: &std::path::Path, item: &str, arm: &str, repeat: u32) -> PathBuf {
    dir.join(format!(
        "{}--{}--r{}.json",
        item.replace(['/', '\\'], "_"),
        arm,
        repeat
    ))
}

fn p8_answer_path(dir: &std::path::Path, item: &str, arm: &str, repeat: u32) -> PathBuf {
    let mut p = p8_state_path(dir, item, arm, repeat);
    p.set_extension("answer.json");
    p
}

/// P8 — a gate inside a graph (METHOD-P §5).
async fn repro_probe(claude_bin: &str, args: ReproArgs) -> Result<()> {
    // `answer` touches no model and needs no environment.
    if args.phase == "answer" {
        let items = p1::read_items(&args.items)?;
        for item in &items {
            if let Some(only) = &args.only
                && &item.slug != only
            {
                continue;
            }
            let path = p8_answer_path(&args.state_dir, &item.slug, &args.arm, args.repeat);
            let a = p8::GateAnswer {
                answer: args.answer.clone(),
                answered_at: chrono::Utc::now(),
                question: "repro.cannot-reproduce".into(),
            };
            if let Some(p) = path.parent() {
                std::fs::create_dir_all(p)?;
            }
            std::fs::write(&path, serde_json::to_string_pretty(&a)?)?;
            eprintln!("  answered {} [{}] r{} -> {}", item.slug, args.arm, args.repeat, args.answer);
        }
        return Ok(());
    }

    let env = hygiene::assert_clean(claude_bin).await?;
    let redkiln = std::fs::canonicalize(&args.redkiln)?;
    let worktree = std::fs::canonicalize(&args.worktree)?;
    let journal = Journal::open(&args.out, &[])?;
    let graph_text = std::fs::read_to_string(&args.graph)
        .with_context(|| format!("reading {}", args.graph.display()))?;
    let graph_sha = fingerprint::sha256_hex(graph_text.as_bytes());

    let model = match args.model.as_str() {
        "haiku" => Tier::Haiku,
        "sonnet" => Tier::Sonnet,
        "opus" => Tier::Opus,
        other => anyhow::bail!("unknown model tier `{other}`"),
    };
    let margins = Margins {
        max_five_hour: args.max_five_hour,
        max_seven_day: args.max_seven_day,
    };
    let mut guard = Guard::default();
    guard.observe(last_headroom(&args.out, chrono::Utc::now().timestamp()).as_ref());

    let runner = Runner::new(claude_bin, env);
    let items = p1::read_items(&args.items)?;
    let manifest = read_manifest(&args.manifest)?;
    let attempt_schema = p8::attempt_schema();
    let reduce_schema = p8::reduce_schema();
    let mut spent = spend_so_far(&args.out);

    eprintln!(
        "P8 [{}] arm {} r{} — {} item(s), model {} | graph {} | spent ${:.2} of ${:.2}",
        args.phase, args.arm, args.repeat, items.len(), args.model,
        &graph_sha[..12], spent, args.max_cost_usd
    );

    for item in &items {
        if let Some(only) = &args.only
            && &item.slug != only
        {
            continue;
        }
        if spent >= args.max_cost_usd {
            eprintln!("\n  SPEND HALT before {}: ${:.2} over ${:.2}", item.slug, spent, args.max_cost_usd);
            break;
        }
        match guard.check(margins) {
            throttle::Verdict::Proceed => {}
            verdict => {
                eprintln!("\n  HALT before {}: {}", item.slug, verdict.reason());
                break;
            }
        }
        let Some(m) = manifest.iter().find(|m| m.slug == item.slug) else {
            eprintln!("  {:<44} SKIPPED (not in manifest)", item.slug);
            continue;
        };
        let Some(sha) = m.checkpoint_sha.clone() else { continue };

        let fixture = worktree
            .parent()
            .unwrap_or(&worktree)
            .join(format!("e3-silent-{}", item.slug));
        e3::build_silent_fixture(&redkiln, &fixture, &sha)?;
        e3::assert_fix_unreachable(&fixture, &sha)?;

        let spec_raw = std::fs::read_to_string(redkiln.join(&m.dir).join("spec.md")).unwrap_or_default();
        // §5.2's fixture: "3 E3 items with the repro steps deliberately
        // removed so the gate fires". Removed in the PROMPT — the corpus is
        // frozen and is never written to (AFK brief rule 6).
        let (spec, removed) = p8::strip_repro(&spec_raw);
        let state_path = p8_state_path(&args.state_dir, &item.slug, &args.arm, args.repeat);
        let answer_path = p8_answer_path(&args.state_dir, &item.slug, &args.arm, args.repeat);

        // ---------------------------------------------------------------- run
        if args.phase == "run" || args.phase == "control" {
            let mut steps: Vec<p8::StepOutput> = Vec::new();
            let mut gate_arrivals = 0u32;
            let mut tokens = 0u64;
            let mut cost = 0.0f64;
            let mut pass = 0u32;
            let mut reproduced = false;

            // `attempt`: repeat until reproduced, max 3 (catalog).
            while pass < 3 {
                pass += 1;
                eprint!("  {} [{}] attempt p{} ... ", item.slug, args.arm, pass);
                let (rec, _) = runner.run(&StepSpec {
                    experiment: "P8".into(),
                    arm: args.arm.clone(),
                    label: format!("{}#attempt", item.slug),
                    phase: "attempt".into(),
                    occurrence: args.repeat,
                    attempt: pass,
                    model,
                    effort: None,
                    adopt_agent: true,
                    agent: Some(step::AgentCard {
                        name: "investigator".into(),
                        description: "Reproduces a reported fault, then reduces it.".into(),
                        prompt: p8::REPRO_CARD.into(),
                        tools: Some(e3::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                        model: Some(args.model.clone()),
                        effort: None,
                        permission_mode: None,
                    }),
                    prompt: p8::build_attempt_prompt(&item.slug, &spec, None),
                    schema: Some(attempt_schema.clone()),
                    allowed_tools: vec![],
                    permission_mode: Some("bypassPermissions".into()),
                    cwd: fixture.clone(),
                    isolate_mcp: true,
                    extra_flags: vec![],
                    timeout: Duration::from_secs(args.timeout_secs),
                }).await;
                guard.observe(rec.headroom.as_ref());
                let c = rec.total_cost_usd.unwrap_or(0.0);
                // Output tokens are the resume-cost unit F28 priced a stage
                // gate in (~18.6k), so P8's ratio is comparable to it.
                let t = rec.usage.as_ref().and_then(|u| u.output_tokens).unwrap_or(0);
                cost += c;
                spent += c;
                tokens += t;
                let status = rec.digest.as_ref()
                    .and_then(|d| d.get("status")).and_then(|v| v.as_str())
                    .unwrap_or("").to_string();
                eprintln!("{} ${:.4} {:.0}s",
                    if status.is_empty() { "NO DIGEST" } else { &status },
                    c, rec.wall_clock_ms as f64 / 1000.0);
                let out = rec.digest.clone().unwrap_or(serde_json::Value::Null);
                steps.push(p8::StepOutput::new("attempt", pass, out, t, c));
                journal.append(&serde_json::json!({
                    "experiment": "P8", "phase": "attempt", "arm": args.arm,
                    "item": item.slug, "repeat": args.repeat, "pass": pass,
                    "graph_sha256": graph_sha, "corpus_sha": sha,
                    "repro_section_removed": removed,
                    "digest": rec.digest, "step": rec,
                }))?;
                if status == "reproduced" {
                    reproduced = true;
                    break;
                }
            }

            // `triage-gate`: `when = "${attempt.outputs.status} != 'reproduced'"`,
            // or forced (#36) so the resume mechanism can be measured at all.
            let forced = reproduced && args.force_gate;
            if !reproduced || forced {
                gate_arrivals += 1;
                let state = p8::GraphState {
                    item: item.slug.clone(),
                    arm: args.arm.clone(),
                    repeat: args.repeat,
                    activity: "minimal-repro".into(),
                    graph_sha256: graph_sha.clone(),
                    corpus_sha: sha.clone(),
                    fixture: fixture.to_string_lossy().to_string(),
                    steps: steps.clone(),
                    gate_arrivals,
                    gate_forced: forced,
                    marked_at: chrono::Utc::now(),
                    tokens_before_gate: tokens,
                    cost_before_gate: cost,
                };
                state.save(&state_path)?;
                journal.append(&serde_json::json!({
                    "experiment": "P8", "phase": "triage-gate", "kind": "gate",
                    "arm": args.arm, "item": item.slug, "repeat": args.repeat,
                    "question": "repro.cannot-reproduce",
                    "arrival": gate_arrivals,
                    "state_path": state_path.to_string_lossy(),
                    "state_digest": state.digest(),
                    "manifest": state.manifest(),
                    // #36: true when the gate's own predicate was NOT met and
                    // the suspension was the driver's choice. The step outputs
                    // below are real either way.
                    "gate_forced": forced,
                    "attempt_status": if reproduced { "reproduced" } else { "not-reproduced" },
                    "tokens_before_gate": tokens, "cost_before_gate": cost,
                    "graph_sha256": graph_sha,
                }))?;
                // The marker the supervisor waits for. Printed to stdout so a
                // waiter can match it without parsing the dispatch chatter.
                println!("GATE-MARK {} {} r{}", item.slug, args.arm, args.repeat);
                eprintln!("  gate mark written: {}", state_path.display());

                if args.phase == "run" {
                    // Block, holding state in memory, so the kill is real.
                    // A `control` run does not block and is never killed —
                    // it is the no-kill comparator (§5.2).
                    eprintln!("  blocking up to {}s for the kill ...", args.block_secs);
                    let deadline = std::time::Instant::now() + Duration::from_secs(args.block_secs);
                    while std::time::Instant::now() < deadline {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        if answer_path.exists() {
                            eprintln!("  answered without a kill — this run is a control, not a kill arm.");
                            break;
                        }
                    }
                    continue;
                }
            } else {
                eprintln!(
                    "  {} reproduced without the gate — not a P8 fixture (pass --force-gate \
                     to measure the resume mechanism anyway, #36)",
                    item.slug
                );
            }
            continue;
        }

        // ------------------------------------------------------------- resume
        anyhow::ensure!(args.phase == "resume", "unknown --phase `{}`", args.phase);
        let before = p8::GraphState::load(&state_path)?;
        let answer: p8::GateAnswer = serde_json::from_str(
            &std::fs::read_to_string(&answer_path)
                .with_context(|| format!("no answer at {} — run --phase answer first", answer_path.display()))?,
        )?;
        let gap_min = (chrono::Utc::now() - before.marked_at).num_seconds() as f64 / 60.0;
        eprintln!(
            "  {} [{}] resume after {:.1} min, {} step(s) before the kill",
            item.slug, args.arm, gap_min, before.steps.len()
        );

        // Rebuild from disk and compare. This is the primary result.
        let after_reload = p8::GraphState::load(&state_path)?;
        let check = p8::check_resume(&before, &after_reload);
        if !check.lossless {
            eprintln!("  LOST OR CHANGED across the kill: changed={:?} lost={:?}", check.changed, check.lost);
        }

        let resume_started = chrono::Utc::now();
        let mut steps = before.steps.clone();
        let mut gate_arrivals = before.gate_arrivals;
        let mut resume_tokens = 0u64;
        let mut resume_cost = 0.0f64;
        let mut first_dispatch_ms: Option<i64> = None;

        // §3: resume into the SAME step with `${gate.answer}` bound. The loop
        // back to `attempt` is what lets the gate fire a second time — P9b's A8
        // says that is a new mark, and §5.3 asks whether it is.
        let pass = steps.iter().filter(|s| s.step == "attempt").count() as u32 + 1;
        eprint!("    attempt p{pass} (post-resume) ... ");
        // #37C: §5.3 asks for the wall clock from the answer to the first
        // post-resume dispatch. That is the latency to its START; taking it
        // after `run()` returned measured the dispatch itself (686 s).
        first_dispatch_ms = Some((chrono::Utc::now() - resume_started).num_milliseconds());
        let (rec, _) = runner.run(&StepSpec {
            experiment: "P8".into(),
            arm: args.arm.clone(),
            label: format!("{}#attempt", item.slug),
            phase: "attempt".into(),
            occurrence: args.repeat,
            attempt: pass,
            model,
            effort: None,
            adopt_agent: true,
            agent: Some(step::AgentCard {
                name: "investigator".into(),
                description: "Reproduces a reported fault, then reduces it.".into(),
                prompt: p8::REPRO_CARD.into(),
                tools: Some(e3::CARD_TOOLS.iter().map(|t| t.to_string()).collect()),
                model: Some(args.model.clone()),
                effort: None,
                permission_mode: None,
            }),
            prompt: p8::build_attempt_prompt(&item.slug, &spec, Some(&answer.answer)),
            schema: Some(attempt_schema.clone()),
            allowed_tools: vec![],
            permission_mode: Some("bypassPermissions".into()),
            cwd: fixture.clone(),
            isolate_mcp: true,
            extra_flags: vec![],
            timeout: Duration::from_secs(args.timeout_secs),
        }).await;
        guard.observe(rec.headroom.as_ref());
        let c = rec.total_cost_usd.unwrap_or(0.0);
        let t = rec.usage.as_ref().and_then(|u| u.output_tokens).unwrap_or(0);
        resume_cost += c;
        resume_tokens += t;
        spent += c;
        let status = rec.digest.as_ref().and_then(|d| d.get("status")).and_then(|v| v.as_str()).unwrap_or("").to_string();
        eprintln!("{} ${:.4} {:.0}s", if status.is_empty() { "NO DIGEST" } else { &status }, c, rec.wall_clock_ms as f64 / 1000.0);
        steps.push(p8::StepOutput::new("attempt", pass, rec.digest.clone().unwrap_or(serde_json::Value::Null), t, c));
        journal.append(&serde_json::json!({
            "experiment": "P8", "phase": "attempt", "post_resume": true,
            "arm": args.arm, "item": item.slug, "repeat": args.repeat, "pass": pass,
            "gate_answer_bound": answer.answer,
            "graph_sha256": graph_sha, "digest": rec.digest, "step": rec,
        }))?;

        // Second gate arrival. #37A: "the gate's condition was not met" and
        // "the gate was re-reached and minted nothing" are different findings,
        // and only the second one bears on §9b's A8. They are reported as
        // separate fields and the second is NULL when the gate was not reached.
        let second_reached = status != "reproduced" || (before.gate_forced && args.force_gate);
        let mut second_mark = false;
        if second_reached {
            gate_arrivals += 1;
            second_mark = true;
            journal.append(&serde_json::json!({
                "experiment": "P8", "phase": "triage-gate", "kind": "gate",
                "arm": args.arm, "item": item.slug, "repeat": args.repeat,
                "question": "repro.cannot-reproduce",
                "arrival": gate_arrivals,
                "second_arrival_minted_a_new_mark": true,
                "note": "P9b A8 assumes a re-reached gate is a NEW mark (METHOD-P §5.3)",
                "graph_sha256": graph_sha,
            }))?;
        }

        journal.append(&serde_json::json!({
            "experiment": "P8", "phase": "emit",
            "resume_key": format!("P8/{}/{}#r{}", args.arm, item.slug, args.repeat),
            "outcome": if check.lossless { "ok" } else { "void" },
            "void_reason": if check.lossless { serde_json::Value::Null }
                           else { serde_json::json!("step output lost or changed across the kill (METHOD-P §5.4)") },
            "arm": args.arm, "item": item.slug, "repeat": args.repeat, "model": args.model,
            "graph_sha256": graph_sha, "corpus_sha": before.corpus_sha,
            // #37B: what the RUN suspended under, read from the state.
            "gate_forced": before.gate_forced,
            "force_gate_on_resume": args.force_gate,
            "resume_check": check,
            "gap_minutes": gap_min,
            "gate_arrivals_total": gate_arrivals,
            // #37A: null when the gate was never re-reached — §5.4's third
            // criterion is not decidable from a gate that did not fire.
            "second_arrival_reached": second_reached,
            "post_resume_attempt_status": status,
            "second_arrival_minted_a_new_mark":
                if second_reached { serde_json::json!(second_mark) } else { serde_json::Value::Null },
            "answered_at": answer.answered_at,
            "ms_from_answer_to_first_post_resume_dispatch": first_dispatch_ms,
            "tokens_before_gate": before.tokens_before_gate,
            "cost_before_gate": before.cost_before_gate,
            "resume_tokens": resume_tokens,
            "resume_cost_usd": resume_cost,
            "f28_stage_gate_tokens": 18600,
            "resume_vs_f28_ratio": resume_tokens as f64 / 18600.0,
        }))?;
        eprintln!(
            "    emit ... lossless={} second_arrival={} minted={} resume {} tok (F28 ratio {:.2}) ${:.4}",
            check.lossless,
            second_reached,
            if second_reached { second_mark.to_string() } else { "n/a (gate not re-reached)".into() },
            resume_tokens,
            resume_tokens as f64 / 18600.0,
            resume_cost
        );
    }

    eprintln!("\n  spend in this file: ${:.2} of ${:.2}", spent, args.max_cost_usd);
    println!("wrote {}", journal.path().display());
    Ok(())
}
