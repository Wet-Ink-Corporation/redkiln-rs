//! P7 — encapsulated quality: cost vs catch (METHOD-P §4, absorbs P4).
//!
//! PC6 makes every agent-producing activity own its quality with an internal
//! `adversary -> policy -> (gate)` block. Whether that block earns its cost at
//! `recommended`, or only at `mandatory`, decides the `quality` shorthand's
//! defaults and the pack's default rigor. F23/F26 give the single-dispatch
//! baseline; F27 is why the block ends in a policy CHECK rather than a model
//! verdict — the verdict is the unstable part.
//!
//! This is a **thin driver**, not a general activity-graph interpreter
//! (METHOD-P §4.2 asks for exactly that). It implements the step sequence of
//! `spikes/p9/catalog/root-cause.toml` and asserts at startup that the catalog
//! still has the shape it implements, so a catalog edit is a loud failure
//! rather than a silently divergent measurement.
//!
//! Two departures from the AFK brief's six-step sketch, both measured from the
//! catalog and logged as decisions:
//!
//! - the graph has **seven** steps: `no-hypothesis` is a gate that fires in
//!   EVERY arm (`when = "${hypothesise.outputs.status} != 'confirmed'"`),
//!   unlike `signoff` which is `when = "rigor >= mandatory"`. It is
//!   auto-answered and its arrivals are counted (DECISIONS-2026-09-09 #4).
//! - `policy`'s loop-back is `max = 2` in the catalog. METHOD-P §4.2's "<= 3"
//!   counts total `hypothesise` passes, which `max = 2` produces exactly, so
//!   the catalog is honoured and nothing under test is edited (#3).
//!
//! `root-cause.toml` also carries `max_rigor = "recommended"`, so the selection
//! engine would never SELECT this activity at `rigor = mandatory`. P7 drives
//! the graph rather than selecting it, so the mandatory arm is executable and
//! prices the block — but it is not a reachable configuration, and F37 says so
//! (#7).

use anyhow::{Context, Result};
use serde::Serialize;

/// The rigor arms of METHOD-P §4.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Arm {
    /// No adversary. The single-dispatch baseline.
    Optional,
    /// Adversary + policy, no loop-back.
    Recommended,
    /// Adversary + policy + loop-back on policy false.
    Mandatory,
}

impl Arm {
    pub fn as_str(self) -> &'static str {
        match self {
            Arm::Optional => "optional",
            Arm::Recommended => "recommended",
            Arm::Mandatory => "mandatory",
        }
    }
    pub fn parse(s: &str) -> Result<Self> {
        Ok(match s {
            "optional" => Arm::Optional,
            "recommended" => Arm::Recommended,
            "mandatory" => Arm::Mandatory,
            other => anyhow::bail!("unknown arm `{other}` (expected optional, recommended, mandatory)"),
        })
    }
    /// `adversary` is `when = "rigor >= recommended"`.
    pub fn runs_adversary(self) -> bool {
        matches!(self, Arm::Recommended | Arm::Mandatory)
    }
    /// `signoff` is `when = "rigor >= mandatory"`.
    pub fn runs_signoff(self) -> bool {
        matches!(self, Arm::Mandatory)
    }
    /// METHOD-P §4.2: loop-back is the mandatory arm's distinguishing feature.
    pub fn loops_back(self) -> bool {
        matches!(self, Arm::Mandatory)
    }
}

/// What the driver asserts about the catalog before it runs anything.
#[derive(Debug, Clone, Serialize)]
pub struct GraphGuard {
    pub activity_id: String,
    pub version: i64,
    pub max_rigor: Option<String>,
    pub step_ids: Vec<String>,
    pub loopback_max: i64,
    pub hypothesise_repeat_max: i64,
    pub sha256: String,
}

/// Read `root-cause.toml` and assert it still has the shape this driver
/// implements.
///
/// The digest goes on every record (METHOD-P §0 rule 2): a result is a function
/// of its committed inputs or it is void.
pub fn guard_graph(path: &std::path::Path) -> Result<GraphGuard> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading graph {}", path.display()))?;
    let sha256 = crate::fingerprint::sha256_hex(text.as_bytes());
    let doc: toml::Value = toml::from_str(&text)
        .with_context(|| format!("parsing {}", path.display()))?;

    let act = doc.get("activity").context("no [activity] table")?;
    let activity_id = act
        .get("id")
        .and_then(|v| v.as_str())
        .context("activity.id")?
        .to_string();
    anyhow::ensure!(
        activity_id == "root-cause",
        "this driver implements `root-cause`, got `{activity_id}`"
    );
    let version = act.get("version").and_then(|v| v.as_integer()).unwrap_or(-1);
    let max_rigor = act
        .get("max_rigor")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let steps = doc
        .get("step")
        .and_then(|v| v.as_array())
        .context("no [[step]] array")?;
    let step_ids: Vec<String> = steps
        .iter()
        .filter_map(|s| s.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect();

    // The exact sequence this driver implements. A catalog edit that adds,
    // removes or reorders a step must stop the run, not quietly change what
    // P7 measured.
    const EXPECTED: [&str; 7] = [
        "hypothesise",
        "no-hypothesis",
        "confirm",
        "adversary",
        "policy",
        "signoff",
        "emit",
    ];
    anyhow::ensure!(
        step_ids == EXPECTED,
        "graph shape changed: driver implements {EXPECTED:?}, catalog has {step_ids:?}"
    );

    let find = |id: &str| steps.iter().find(|s| s.get("id").and_then(|v| v.as_str()) == Some(id));
    let loopback_max = find("policy")
        .and_then(|s| s.get("on_false"))
        .and_then(|v| v.get("max"))
        .and_then(|v| v.as_integer())
        .context("policy.on_false.max")?;
    let hypothesise_repeat_max = find("hypothesise")
        .and_then(|s| s.get("repeat"))
        .and_then(|v| v.get("max"))
        .and_then(|v| v.as_integer())
        .context("hypothesise.repeat.max")?;

    Ok(GraphGuard {
        activity_id,
        version,
        max_rigor,
        step_ids,
        loopback_max,
        hypothesise_repeat_max,
        sha256,
    })
}

/// The investigator's agent card.
///
/// The catalog names the ROLE (`role = "investigator"`) and no model; the card
/// body is the driver's, and its digest is recorded on every dispatch. Tools
/// are `e3::CARD_TOOLS` — Read and Bash so it can actually investigate, plus
/// the `StructuredOutput` channel without which `--json-schema` is silently
/// inert and every record is void (F20).
pub const INVESTIGATOR_CARD: &str = "You are an investigator. You are given a repository and a description of work that was supposed to have been done in it, and you find where the behaviour actually goes wrong.

Method: read the relevant code before forming a view. Run tests, run the CLI, read git history if it helps. Prefer evidence you produced yourself over inference from the description.

A root cause is the place a wrong behaviour ORIGINATES, not the place it is observed and not the place it is reported. `src/cli/x.ts prints the wrong thing` is a symptom; the function that computed the wrong thing is the cause.

Name the narrowest set of files that could be changed to fix it. Listing everything you read is not an answer and will be scored as a miss.

Say `unconfirmed` when you are not sure. An honest `unconfirmed` with a good hypothesis is worth more than a `confirmed` you cannot support, and nothing is gained by overclaiming.

If the process you are being asked to follow is genuinely wrong for this item, you may request a deviation. Use it sparingly and state the fact about THIS item that justifies it — the claim is checked.
";

/// The investigator's output. `HypothesisLog` in the catalog's terms, plus the
/// optional deviation channel METHOD-P §4.2 folds P4 into.
pub fn hypothesis_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["status", "root_cause", "discriminating_test", "summary"],
        "properties": {
            "status": {
                "type": "string",
                "description": "`confirmed` only when you have evidence a discriminating test would separate your hypothesis from the alternatives. Otherwise `unconfirmed` — which is a legitimate answer, not a failure.",
                "enum": ["confirmed", "unconfirmed"]
            },
            "root_cause": {
                "type": "object",
                "additionalProperties": false,
                "required": ["summary", "files"],
                "properties": {
                    "summary": {
                        "type": "string",
                        "description": "One or two sentences: what is actually wrong, in terms of the code."
                    },
                    "files": {
                        "type": "array",
                        "description": "Repo-relative paths where the defect LIVES — not where it is observed. Required, and the narrower the better.",
                        "items": { "type": "string" }
                    }
                }
            },
            "discriminating_test": {
                "type": "string",
                "description": "A test-file path or vitest name filter that would come out differently if your hypothesis is right. Empty string when you have none."
            },
            "summary": {
                "type": "string",
                "description": "The reasoning that got you here, briefly."
            },
            "request_deviation": {
                "type": "object",
                "description": "OPTIONAL. Use only when the process as specified is genuinely wrong for this item — not when it is merely inconvenient.",
                "additionalProperties": false,
                "required": ["target", "kind", "claimed_predicate", "why"],
                "properties": {
                    "target": {
                        "type": "string",
                        "description": "The step or activity the deviation is about."
                    },
                    "kind": {
                        "type": "string",
                        "description": "`add` an activity, `downgrade` one, or `skip` it. root-cause.toml declares deviation_ok = [add, downgrade].",
                        "enum": ["add", "downgrade", "skip"]
                    },
                    "claimed_predicate": {
                        "type": "string",
                        "description": "The fact about this item that you claim makes the deviation correct. It will be checked against the evidence."
                    },
                    "why": { "type": "string" }
                }
            }
        }
    })
}

/// The investigator's brief.
///
/// Says nothing about how the fixture was built. An S-SILENT fixture must not
/// be told it is one — the leak strings `reverted / seeded / defect was /
/// bug was introduced / s-silent` are asserted absent by the tests below, the
/// same guard `e3.rs` keeps on its own prompts.
pub fn build_investigator_prompt(
    slug: &str,
    spec: &str,
    report: &str,
    prior: Option<&serde_json::Value>,
    findings: Option<&serde_json::Value>,
    gate_answer: Option<&str>,
) -> String {
    let mut p = format!(
        "You are investigating a fault in this repository. The working directory is a checkout of it.\n\n\
         Find the ROOT CAUSE: the place in the code where the wrong behaviour originates, not the place \
         where it is noticed. Read the code, run what you need, and form a hypothesis you could \
         discriminate with a test.\n\n\
         Return the structured record defined by the output schema.\n\n\
         `root_cause.files` is the load-bearing field: repo-relative paths where the defect lives. Be \
         specific and be narrow — naming every file you looked at is not an answer.\n\n\
         Set `status` to `confirmed` only when you have real evidence. `unconfirmed` is a legitimate \
         answer and is preferred to a guess dressed up as a conclusion.\n\n\
         ---- ITEM: {slug} ----\n\n\
         ## What this work was supposed to achieve\n\n{spec}\n\n\
         ## What was reported about the implementation\n\n{report}\n\n\
         ---- END ITEM ----\n"
    );
    if let Some(prior) = prior {
        p.push_str(&format!(
            "\n## Your previous attempt\n\nThis is a further pass. Your previous hypothesis:\n\n```json\n{}\n```\n\
             Do not simply restate it. Either strengthen it with new evidence or replace it.\n",
            serde_json::to_string_pretty(prior).unwrap_or_default()
        ));
    }
    if let Some(f) = findings {
        p.push_str(&format!(
            "\n## A reviewer's findings on your previous pass\n\n```json\n{}\n```\n\
             Address these specifically.\n",
            serde_json::to_string_pretty(f).unwrap_or_default()
        ));
    }
    if let Some(a) = gate_answer {
        p.push_str(&format!("\n## Guidance from the escalation\n\n{a}\n"));
    }
    p
}

/// The adversary's agent card.
///
/// #22 (revised): this was first wired to the production `redkiln-review` card
/// on the reasoning that a bespoke reviewer prices one that does not exist. That
/// was wrong on the catalog's own terms. `adversary`'s declared inputs are
/// `log = "${hypothesise.outputs}"` and `report = "${confirm.outputs}"` — it
/// reviews a CLAIM, not an implementation against acceptance criteria, which is
/// what `redkiln-review` is written to do. A card whose instructions do not
/// match its inputs measures the mismatch, not the block. The evidence classes
/// and the typed findings schema are kept, so the policy mapping to
/// `e3::LENIENT` and the comparison to F31's conformance still hold.
pub const ADVERSARY_CARD: &str = "You are an adversarial reviewer. Someone has proposed a root cause for a fault, and your job is to find out whether it is actually right.

You are given their hypothesis and the result of the discriminating test they named. You have the repository. Check the claim yourself rather than reasoning about it.

Ways a root cause is wrong, in the order they are usually wrong:
- it names a symptom, not the origin: the file where the wrong value is PRINTED rather than where it is computed
- it names the right area but the wrong file, or a set so broad it is not actionable
- the discriminating test does not in fact discriminate: it would pass or fail the same way either way
- the evidence offered does not support the claim, even though the claim may happen to be true

Classify every finding by the evidence you actually have. `reproduced` means you ran something and watched it behave as you describe. `mutation-survived` means you changed a behaviour and the suite stayed green. `narrowed-pin` means a test pins a narrower claim than the requirement behind it. `advisory` is anything you could not evidence at that bar — use it honestly rather than inflating a hunch.

You return no verdict. It is computed from your findings.";

/// The adversary's brief. The reviewer classifies its own findings by evidence
/// class; the VERDICT is computed by the policy check, never returned by the
/// model (proposal D4, F27).
pub fn build_adversary_prompt(slug: &str, hypothesis: &serde_json::Value, test: &TestRun) -> String {
    format!(
        "An investigator has proposed a root cause for a fault in this repository. Your job is to try to \
         break it.\n\n\
         Check the claim against the code yourself. A root cause that names the wrong file, names a \
         symptom rather than a cause, or is too broad to act on, is wrong — say so and show why.\n\n\
         Return findings under the output schema. Classify each by the evidence you actually have:\n\
         - `reproduced` — you ran something and observed the fault behave as you describe\n\
         - `mutation-survived` — you changed a behaviour and the suite stayed green\n\
         - `narrowed-pin` — a test pins a narrower claim than the requirement it stands for\n\
         - `advisory` — a concern you could not evidence at that bar\n\n\
         You do NOT return a verdict. It is computed from your findings' evidence classes.\n\n\
         ---- ITEM: {slug} ----\n\n\
         ## The proposed root cause\n\n```json\n{}\n```\n\n\
         ## The discriminating test that was run\n\ncommand: {}\nexit code: {:?}\npassed: {}\n\n```\n{}\n```\n\n\
         ---- END ----\n",
        serde_json::to_string_pretty(hypothesis).unwrap_or_default(),
        test.command,
        test.exit_code,
        test.passed,
        test.tail
    )
}

/// A real test run in the fixture clone — the `confirm` step's `toolchain:test`.
#[derive(Debug, Clone, Serialize)]
pub struct TestRun {
    pub command: String,
    pub selection: String,
    pub exit_code: Option<i32>,
    pub passed: bool,
    pub duration_ms: u128,
    pub tail: String,
    /// The step ran at all. A `confirm` that never spawned says nothing about
    /// the hypothesis, and must not be read as a negative result.
    pub spawned: bool,
    /// #31: the run hit the step's timeout and was killed. Like `!spawned`,
    /// this is NOT evidence against the hypothesis — the test did not finish,
    /// so it did not discriminate anything. A void is never a miss.
    pub timed_out: bool,
    pub timeout_secs: u64,
}

/// Keep one model-supplied token to something that cannot be read as anything
/// but a path or a test name.
fn sanitize_token(s: &str, extra: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric() || extra.contains(*c))
        .take(160)
        .collect::<String>()
        .trim()
        .to_string()
}

/// Split a selection into the argv vitest actually wants.
///
/// #20. The investigator writes selections in the shape a human would type:
///   `test/advance-git-commit.test.ts -t "does not sweep an unrelated sibling"`
/// Passing that as a single argument after `--run` gave vitest a nonsense
/// positional and it ran far more than the named test — 278 s for one `confirm`
/// step on the pre-flight. The file and the `-t` pattern are separate argv
/// elements, and the pattern keeps its spaces because it is never shell-parsed.
pub fn selection_args(sel: &str) -> Vec<String> {
    let mut out = Vec::new();
    // The `-t` pattern, quoted or trailing.
    let (head, name) = match sel.find("-t ") {
        Some(i) => {
            let rest = sel[i + 3..].trim();
            let pat = rest
                .strip_prefix('"')
                .and_then(|r| r.split_once('"').map(|(a, _)| a))
                .or_else(|| {
                    rest.strip_prefix('\'')
                        .and_then(|r| r.split_once('\'').map(|(a, _)| a))
                })
                .unwrap_or(rest);
            (&sel[..i], sanitize_token(pat, "._-/ ,:()\'"))
        }
        None => (sel, String::new()),
    };
    // The first token that looks like a test file.
    for tok in head.split_whitespace() {
        let t = sanitize_token(tok, "._-/");
        if t.ends_with(".ts") || t.ends_with(".js") || t.ends_with(".tsx") {
            out.push(t);
            break;
        }
    }
    if out.is_empty() {
        // No file named. A bare name filter is still a legitimate selection,
        // but an empty one is not a test run at all.
        if name.is_empty() {
            return Vec::new();
        }
    }
    if !name.is_empty() {
        out.push("-t".into());
        out.push(name);
    }
    out
}

/// Run the discriminating test the investigator named.
///
/// A *selection*, not the whole gate: the catalog's `confirm` step is
/// `run = "toolchain:test"` with `inputs = { selection = ... }`, and running
/// the full 236-file suite 90 times would cost wall clock without measuring
/// anything P7 asks about.
///
/// **Bounded** (#31). The catalog declares `timeout_secs = "${toolchain.test_timeout}"`
/// on this step, so a bounded run is the pre-registered behaviour and the
/// unbounded one was the defect: measured, a single `confirm` ran 28 minutes
/// while 83 node processes were resident, and #29's attempt to cap vitest's
/// fan-out from outside did not take. A timed-out run is recorded as such and
/// is not read as a failing test.
pub fn run_selected_test(wt: &std::path::Path, selection: &str, timeout_secs: u64) -> TestRun {
    let clock = std::time::Instant::now();
    let sel_args = selection_args(selection);
    if sel_args.is_empty() {
        return TestRun {
            command: String::new(),
            selection: selection.to_string(),
            exit_code: None,
            passed: false,
            duration_ms: 0,
            tail: "no discriminating test was named".into(),
            spawned: false,
            timed_out: false,
            timeout_secs,
        };
    }
    let sel = sel_args.join(" ");
    // #34: `npx vitest run <file> [-t <pattern>]`, NOT `pnpm test -- --run ...`.
    // The package's `test` script is `vitest run`, so the pnpm form expanded to
    // `vitest run --run <file>` and the positional filter did not survive the
    // forwarding — every earlier `confirm` ran the whole suite. Measured on one
    // fixture: pnpm form never finished inside 900 s; direct form ran 1 file,
    // 6 tests, 15.57 s.
    let command = format!("npx vitest run {sel}");
    // `cmd /c` so `pnpm` resolves to pnpm.CMD on Windows — the same lesson
    // gate.rs records: a `bash -lc` here resolved to WSL bash and every run
    // "passed" in four seconds having never tested anything.
    let mut argv: Vec<String> = ["/c", "npx", "vitest", "run"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    argv.extend(sel_args.iter().cloned());
    // One file at a time; the selection is a single test file by construction.
    argv.push("--no-file-parallelism".into());
    // #31: output goes to a file rather than a pipe. Polling `try_wait` while a
    // pipe fills is a deadlock, and the whole point here is to be able to stop
    // waiting.
    let log_path = wt.join(".p7-confirm.log");
    let log = match std::fs::File::create(&log_path) {
        Ok(f) => f,
        Err(e) => {
            return TestRun {
                command,
                selection: sel,
                exit_code: None,
                passed: false,
                duration_ms: clock.elapsed().as_millis(),
                tail: format!("could not open confirm log: {e}"),
                spawned: false,
                timed_out: false,
                timeout_secs,
            };
        }
    };
    let errlog = match log.try_clone() {
        Ok(f) => f,
        Err(e) => {
            return TestRun {
                command,
                selection: sel,
                exit_code: None,
                passed: false,
                duration_ms: clock.elapsed().as_millis(),
                tail: format!("could not clone confirm log handle: {e}"),
                spawned: false,
                timed_out: false,
                timeout_secs,
            };
        }
    };
    let spawned = std::process::Command::new("cmd")
        .args(&argv)
        .current_dir(wt)
        .stdout(std::process::Stdio::from(log))
        .stderr(std::process::Stdio::from(errlog))
        .env("REDKILN_NO_GATE_CACHE", "1")
        .env("CI", "1")
        // #29, measured: `confirm` took 278 s and 217 s while one arm ran, then
        // 1695 s / 1684 s / 1574 s once all three arms were live — vitest fans
        // out a worker per core, three concurrent runs oversubscribed the
        // machine, and 86 node processes were resident at once. The step is
        // supposed to price a *discriminating test*, not the scheduler, so the
        // fan-out is capped and the arms stop competing with each other.
        .env("VITEST_MAX_THREADS", "2")
        .env("VITEST_MIN_THREADS", "1")
        .env("VITEST_MAX_FORKS", "2")
        .env("VITEST_MIN_FORKS", "1")
        .spawn();

    let mut child = match spawned {
        Ok(c) => c,
        Err(e) => {
            return TestRun {
                command,
                selection: sel,
                exit_code: None,
                passed: false,
                duration_ms: clock.elapsed().as_millis(),
                tail: format!("spawn failed: {e}"),
                spawned: false,
                timed_out: false,
                timeout_secs,
            };
        }
    };

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
    let mut timed_out = false;
    let status = loop {
        match child.try_wait() {
            Ok(Some(st)) => break Some(st),
            Ok(None) => {
                if std::time::Instant::now() >= deadline {
                    // Kill the whole tree: vitest fans out workers, and killing
                    // only the `cmd` shell leaves them running and holding a
                    // handle on the fixture (#30).
                    let _ = std::process::Command::new("taskkill")
                        .args(["/PID", &child.id().to_string(), "/T", "/F"])
                        .output();
                    let _ = child.wait();
                    timed_out = true;
                    break None;
                }
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
            Err(_) => break None,
        }
    };

    let text = std::fs::read_to_string(&log_path).unwrap_or_default();
    let n = text.len().saturating_sub(4000);
    let mut tail = text[n..].to_string();
    if timed_out {
        tail.push_str(&format!(
            "\n[#31] killed after {timeout_secs}s — the test did not finish, so it \
             discriminated nothing. This is not evidence against the hypothesis."
        ));
    }
    let _ = std::fs::remove_file(&log_path);

    TestRun {
        command,
        selection: sel,
        exit_code: status.as_ref().and_then(|s| s.code()),
        // A timed-out run is not a pass and not a fail: it is a non-result.
        passed: status.as_ref().is_some_and(|s| s.success()),
        duration_ms: clock.elapsed().as_millis(),
        tail,
        spawned: true,
        timed_out,
        timeout_secs,
    }
}

/// Did the investigator name the right place?
///
/// Mechanical, exactly as METHOD-E3 §'s primary metric is: the answer key is
/// the fix's own file set, and `root_cause.files` either intersects it or does
/// not. No judgement is required, which is the point — F9's lesson was that a
/// metric needing a human oracle cannot be run at this scale.
#[derive(Debug, Clone, Serialize)]
pub struct Scored {
    pub named_files: Vec<String>,
    pub matched_files: Vec<String>,
    pub correct: bool,
    /// Named files that are not in the answer key. A hypothesis that names the
    /// whole repo would otherwise score as correct.
    pub extra_files: Vec<String>,
    pub precision: Option<f64>,
}

pub fn score(hypothesis: &serde_json::Value, reverted: &[String]) -> Scored {
    let norm = |s: &str| s.replace('\\', "/").trim_start_matches("./").to_string();
    let key: Vec<String> = reverted.iter().map(|s| norm(s)).collect();
    let named: Vec<String> = hypothesis
        .get("root_cause")
        .and_then(|r| r.get("files"))
        .and_then(|f| f.as_array())
        .map(|a| a.iter().filter_map(|v| v.as_str()).map(norm).collect())
        .unwrap_or_default();
    let matched: Vec<String> = named
        .iter()
        .filter(|n| key.iter().any(|k| k == *n || n.ends_with(k.as_str()) || k.ends_with(n.as_str())))
        .cloned()
        .collect();
    let extra: Vec<String> = named.iter().filter(|n| !matched.contains(n)).cloned().collect();
    let precision = if named.is_empty() {
        None
    } else {
        Some(matched.len() as f64 / named.len() as f64)
    };
    Scored {
        correct: !matched.is_empty(),
        named_files: named,
        matched_files: matched,
        extra_files: extra,
        precision,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arms_gate_the_right_steps() {
        assert!(!Arm::Optional.runs_adversary());
        assert!(Arm::Recommended.runs_adversary());
        assert!(Arm::Mandatory.runs_adversary());
        // `signoff` is `when = "rigor >= mandatory"` — it does NOT fire at
        // recommended, which the AFK brief's sketch would have implied.
        assert!(!Arm::Recommended.runs_signoff());
        assert!(Arm::Mandatory.runs_signoff());
        // METHOD-P §4.2: loop-back distinguishes mandatory.
        assert!(!Arm::Recommended.loops_back());
        assert!(Arm::Mandatory.loops_back());
    }

    #[test]
    fn the_driver_matches_the_committed_catalog() {
        let g = guard_graph(std::path::Path::new("../p9/catalog/root-cause.toml"))
            .expect("root-cause.toml parses and has the implemented shape");
        assert_eq!(g.activity_id, "root-cause");
        assert_eq!(g.step_ids.len(), 7, "seven steps, not the brief's six");
        // DECISIONS-2026-09-09 #3: the catalog says 2 and is honoured.
        assert_eq!(g.loopback_max, 2);
        assert_eq!(g.hypothesise_repeat_max, 3);
        // #7: the engine would never select this activity at mandatory.
        assert_eq!(g.max_rigor.as_deref(), Some("recommended"));
    }

    #[test]
    fn prompts_do_not_leak_how_the_fixture_was_built() {
        let p = build_investigator_prompt("slug", "SPEC", "REPORT", None, None, None);
        let low = p.to_lowercase();
        for leak in ["reverted", "seeded", "defect was", "bug was introduced", "s-silent"] {
            assert!(!low.contains(leak), "investigator prompt leaks `{leak}`");
        }
        let t = TestRun {
            command: "pnpm test".into(),
            selection: "x".into(),
            exit_code: Some(0),
            passed: true,
            duration_ms: 1,
            tail: String::new(),
            spawned: true,
            timed_out: false,
            timeout_secs: 600,
        };
        let a = build_adversary_prompt("slug", &serde_json::json!({}), &t).to_lowercase();
        for leak in ["reverted", "seeded", "defect was", "bug was introduced", "s-silent"] {
            assert!(!a.contains(leak), "adversary prompt leaks `{leak}`");
        }
    }

    #[test]
    fn scoring_is_mechanical_and_punishes_a_shotgun() {
        let key = vec!["src/store/verify.ts".to_string(), "test/verify.test.ts".to_string()];

        let hit = serde_json::json!({"root_cause": {"files": ["src/store/verify.ts"]}});
        let s = score(&hit, &key);
        assert!(s.correct);
        assert_eq!(s.precision, Some(1.0));

        let miss = serde_json::json!({"root_cause": {"files": ["src/cli/main.ts"]}});
        assert!(!score(&miss, &key).correct);

        // Naming everything hits, but precision records that it did so.
        let shotgun = serde_json::json!({"root_cause": {"files":
            ["src/store/verify.ts", "a.ts", "b.ts", "c.ts"]}});
        let s = score(&shotgun, &key);
        assert!(s.correct);
        assert_eq!(s.precision, Some(0.25));
        assert_eq!(s.extra_files.len(), 3);

        // No files named at all is not a hit, and precision is undefined
        // rather than zero — a void is never a miss.
        let empty = serde_json::json!({"root_cause": {"files": []}});
        let s = score(&empty, &key);
        assert!(!s.correct);
        assert_eq!(s.precision, None);
    }

    #[test]
    fn a_selection_splits_into_the_argv_vitest_wants() {
        // #20: the shape the pre-flight actually returned. The file and the
        // pattern are separate argv elements; the pattern keeps its spaces.
        assert_eq!(
            selection_args("test/advance-git-commit.test.ts -t \"does not sweep a sibling\""),
            vec!["test/advance-git-commit.test.ts", "-t", "does not sweep a sibling"]
        );
        assert_eq!(
            selection_args("test/verify.test.ts"),
            vec!["test/verify.test.ts"]
        );
        // A bare name filter with no file is still a selection.
        assert_eq!(selection_args("-t \"only this one\""), vec!["-t", "only this one"]);
        // Nothing usable is not a test run.
        assert!(selection_args("   ").is_empty());
        assert!(selection_args("run the tests").is_empty());
    }

    #[test]
    fn a_selection_cannot_carry_shell_metacharacters() {
        assert_eq!(sanitize_token("test/verify.test.ts", "._-/"), "test/verify.test.ts");
        // No shell is involved (argv, not a command line), but a path that can
        // only be a path is still the right thing to hand to `cmd /c`.
        // The path allow-set excludes spaces, so a command line collapses to
        // something that cannot be a path and is then rejected outright below.
        assert_eq!(sanitize_token("a && rm -rf /", "._-/"), "arm-rf/");
        assert_eq!(sanitize_token("$(whoami)", "._-/"), "whoami");
        assert!(sanitize_token("   ", "._-/").is_empty());
        assert!(selection_args("a && rm -rf /").is_empty(), "no test file, no run");
    }

    #[test]
    fn the_hypothesis_schema_is_closed_and_names_the_deviation_channel() {
        let s = hypothesis_schema();
        assert_eq!(s["additionalProperties"], serde_json::json!(false));
        let props = s["properties"].as_object().unwrap();
        assert!(props.contains_key("request_deviation"), "P4's channel is present");
        // It must be OPTIONAL — a required deviation request would manufacture
        // the very rate §4.4 is measuring.
        let req: Vec<&str> = s["required"].as_array().unwrap()
            .iter().filter_map(|v| v.as_str()).collect();
        assert!(!req.contains(&"request_deviation"));
        assert_eq!(props["root_cause"]["required"], serde_json::json!(["summary", "files"]));
    }
}
