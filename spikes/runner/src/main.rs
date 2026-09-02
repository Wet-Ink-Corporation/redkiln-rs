//! Layer 0 — the step runner for the Redkiln-RS inversion spike.
//!
//! This is measurement apparatus, not a product. Charter §12 names the failure
//! mode it exists inside: "a Rust runner plus a plan format is roughly 60% of a
//! working orchestrator. The temptation will be to keep building and let the
//! spike quietly become the product without anyone taking a decision."
//!
//! Nothing in `crates/` may depend on this.

mod event;
mod fingerprint;
mod hygiene;
mod journal;
mod result_meta;
mod runner;
mod step;
mod throttle;

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

    /// The prompt. A one-liner suffices — E1 measures the prefix, not the work.
    #[arg(long, default_value = "Reply with the single word: ok")]
    prompt: String,

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
    }
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
    if !already_done.is_empty() && args.commit_between {
        anyhow::bail!(
            "{} already holds {} completed step(s), and this arm commits between \
             dispatches. A cache series cannot be resumed: its premise is one \
             contiguous run against an evolving prefix, and splicing two halves \
             would look like a clean series while measuring something else. \
             Start a fresh results file.",
            args.out.display(),
            already_done.len()
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
            effort: Some(Effort::Low),
            agent: None,
            prompt: args.prompt.clone(),
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
            "  {i:>3}/{}  {:<8} cache={:<7} in={:<8} out={:<7} 7d={:<7} {}ms",
            args.repeats,
            record.outcome,
            record
                .cache_read_share
                .map(|s| format!("{s:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            record
                .usage
                .as_ref()
                .and_then(|u| u.input_tokens)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".into()),
            record
                .usage
                .as_ref()
                .and_then(|u| u.output_tokens)
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
