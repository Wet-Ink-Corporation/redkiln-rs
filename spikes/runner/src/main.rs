//! Layer 0 — the step runner for the Redkiln-RS inversion spike.
//!
//! This is measurement apparatus, not a product. Charter §12 names the failure
//! mode it exists inside: "a Rust runner plus a plan format is roughly 60% of a
//! working orchestrator. The temptation will be to keep building and let the
//! spike quietly become the product without anyone taking a decision."
//!
//! Nothing in `crates/` may depend on this.

mod corpus;
mod event;
mod fingerprint;
mod gate;
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
    #[arg(long, default_value_t = 180)]
    timeout_secs: u64,
    #[arg(long, default_value = "../corpus/PINNED_SHA")]
    pin_file: PathBuf,
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
        Command::TtlMark(args) => ttl_step(&cli.claude_bin, args, "warm").await,
        Command::TtlRead(args) => ttl_step(&cli.claude_bin, args, "read").await,
        Command::CorpusManifest(args) => corpus_manifest(args),
        Command::GateProbe(args) => gate_probe(args),
    }
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
        prompt: args.prompt.clone(),
        schema: None,
        allowed_tools: vec![],
        permission_mode: None,
        cwd: cwd.clone(),
        isolate_mcp: true,
        extra_flags: vec![],
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
