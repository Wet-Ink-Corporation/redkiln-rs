//! E3 — seeded-defect detection. The programme's hard gate.
//!
//! Pre-registered in `spikes/METHOD-E3.md`. The original metric (blocker recall
//! against historical `## Required Changes`) was dead on arrival — all fifteen
//! such sections open with "None" (FINDINGS F9) — so this measures detection of
//! **real historical defects, re-created by reverting real historical fixes**.
//!
//! Conditions per item:
//!
//! | condition | tree | gate | ground truth |
//! | --- | --- | --- | --- |
//! | `clean`    | fix intact | green | `approved` |
//! | `s-silent` | a one-commit fixture at the fix's PARENT | green | `changes-requested` |
//!
//! `s-silent` is the measurement. The tree is simply the pre-fix state: the
//! gate passes, the implementation report still claims the work is done, and
//! nothing looks broken — but the spec's ACs are not met by the code. Catching
//! it requires doing what the card claims ("do not take the report at face
//! value") rather than trusting a green gate.

use std::path::Path;

use anyhow::Result;
use serde::Serialize;

/// The card, loaded from the frozen corpus rather than transcribed.
///
/// `model: inherit` with no effort means a headless dispatch that omits them
/// silently inherits the session default — which would confound the comparison
/// in exactly the direction that flatters the in-session arm. Both are set
/// explicitly by the caller.
pub fn load_card(redkiln: &Path) -> Result<(String, String)> {
    let text = std::fs::read_to_string(redkiln.join("agents").join("redkiln-review.md"))?;
    let mut parts = text.splitn(3, "---");
    parts.next();
    let front = parts.next().unwrap_or_default();
    let body = parts.next().unwrap_or_default().trim().to_string();
    let description = front
        .lines()
        .find_map(|l| l.trim().strip_prefix("description:"))
        .map(|d| d.trim().to_string())
        .unwrap_or_else(|| "Adversarial verification gate for a Redkiln item".into());
    Ok((description, body))
}

/// The tool set the card is given, and the exact set the child must report
/// back at `system/init` for the record to count.
///
/// `Read` and `Bash` are the production card's own declaration.
/// **`StructuredOutput` is added by the instrument** (F20): a card's `tools`
/// list is an allowlist, and with `StructuredOutput` outside it `--json-schema`
/// is silently inert — the reviewer answers in prose and every record is void
/// for want of a digest. It is the channel the digest returns on, not new
/// reach, and it is applied identically to every arm.
pub const CARD_TOOLS: [&str; 3] = ["Read", "Bash", "StructuredOutput"];

/// E3's digest schema.
///
/// Mirrors `forge-fix.js`'s `REVIEW_DIGEST` — same `pass`, `boundaryRespected`,
/// `rubric` dimensions and `overall` — with **one deliberate addition**: each
/// issue must carry a `files` array.
///
/// The production schema's `issues` are `{issue, fix}` with no file field. The
/// card asks for `file:line` evidence in prose, but prose cannot be scored
/// mechanically, and E3's primary metric depends on a mechanical test: does a
/// finding name a file the revert touched? Requiring the field makes the
/// primary metric judgement-free. **This is a deviation from the shipped schema
/// and is disclosed as one** — it asks the reviewer for something it already
/// claims to produce, in a form that can be checked.
pub fn digest_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "required": ["pass", "boundaryRespected", "rubric", "overall", "issues"],
        "properties": {
            "pass": {
                "type": "boolean",
                "description": "HARD GATE — true ONLY if every acceptance criterion is genuinely satisfied by real, reachable behaviour."
            },
            "boundaryRespected": { "type": "boolean" },
            "overall": { "type": "integer", "minimum": 0, "maximum": 3 },
            "rubric": {
                "type": "array",
                "minItems": 5,
                "maxItems": 5,
                "items": {
                    "type": "object",
                    "required": ["dimension", "score", "rationale"],
                    "properties": {
                        "dimension": { "type": "string", "enum": [
                            "ac-coverage", "boundary-fidelity", "test-integrity",
                            "gate-greenness", "regression-risk"
                        ]},
                        "score": { "type": "integer", "minimum": 0, "maximum": 3 },
                        "rationale": { "type": "string" }
                    }
                }
            },
            "issues": {
                "type": "array",
                "description": "Every blocking finding. Empty if and only if pass is true.",
                "items": {
                    "type": "object",
                    "required": ["issue", "fix", "files"],
                    "properties": {
                        "issue": { "type": "string" },
                        "fix": { "type": "string" },
                        "files": {
                            "type": "array",
                            "description": "Repo-relative paths this finding is about. Required.",
                            "items": { "type": "string" }
                        }
                    }
                }
            }
        }
    })
}

/// Build a one-commit fixture at `commit`.
///
/// Both conditions are built this way, and that symmetry is the point: a
/// S-SILENT fixture is a shallow single-commit repo, so if CLEAN were reviewed
/// in the full worktree the flipped bit would be the *shape of the repository*
/// as much as the state of the code. A reviewer that notices `git log` has one
/// entry could treat the two conditions differently for a reason that has
/// nothing to do with the defect. Same shape, same depth, one commit apart.
pub fn build_fixture_at(redkiln: &Path, fixture: &Path, commit: &str) -> Result<bool> {
    if fixture.exists() {
        // `git clean` cannot remove a .git dir; rebuild from scratch each time.
        std::fs::remove_dir_all(fixture)?;
    }
    std::fs::create_dir_all(fixture)?;

    let run = |args: &[&str]| -> Result<()> {
        let out = std::process::Command::new("git")
            .args(args)
            .current_dir(fixture)
            .output()?;
        anyhow::ensure!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
        Ok(())
    };

    run(&["init", "-q"])?;
    run(&["config", "user.email", "spike@local"])?;
    run(&["config", "user.name", "Spike"])?;
    // A fetch by SHA needs the FULL object name: `git fetch <repo> 5f479b82`
    // fails with "couldn't find remote ref". The manifest stores abbreviated
    // SHAs, so every commit is resolved in the source repo first. S-SILENT
    // never hit this only because `parent_of` resolves as a side effect.
    let full = rev_parse(redkiln, commit)?;
    run(&["fetch", "--depth", "1", "--no-tags", "-q", &git_remote_path(redkiln), &full])?;
    run(&["checkout", "-q", "--detach", "FETCH_HEAD"])?;
    // Returned rather than only printed: a missing node_modules silently
    // weakens two of the five rubric dimensions, so it belongs on the record.
    Ok(link_node_modules(redkiln, fixture))
}

/// Resolve a revision to a full commit SHA in the source repo.
pub fn rev_parse(redkiln: &Path, rev: &str) -> Result<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", &format!("{rev}^{{commit}}")])
        .current_dir(redkiln)
        .output()?;
    anyhow::ensure!(
        out.status.success(),
        "could not resolve `{rev}` in {}: {}",
        redkiln.display(),
        String::from_utf8_lossy(&out.stderr).trim()
    );
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Resolve `<sha>^` in the source repo.
///
/// `git fetch <repo> <sha>^` is not a valid refspec — the parent must be a
/// concrete SHA before it can be fetched.
pub fn parent_of(redkiln: &Path, sha: &str) -> Result<String> {
    rev_parse(redkiln, &format!("{sha}^"))
}

/// Build a fixture in which the fix is **unreachable**, not merely reverted.
///
/// F17: the first rig reverted files inside a worktree whose object store still
/// held the fix commit. The reviewer — which has `Bash` — checked the fix back
/// out, reviewed the real code, and ran its own mutation experiment. Every
/// `s-silent` record from that rig would have scored a reviewer that was looking
/// at the *fixed* source against *reverted* ground truth.
///
/// A shallow fetch of the fix's **parent** into a fresh repo gives a
/// single-commit history containing only the pre-fix state. There is no fix
/// commit to restore because no such object exists in the fixture.
pub fn build_silent_fixture(redkiln: &Path, fixture: &Path, sha: &str) -> Result<bool> {
    // Depth 1 at the PARENT: the fix itself is never transferred, so there is
    // no object for a reviewer with Bash to check back out (F17).
    let parent = parent_of(redkiln, sha)?;
    build_fixture_at(redkiln, fixture, &parent)
}

/// Render a local path in the form git will accept as a remote.
///
/// `std::fs::canonicalize` returns a Windows extended-length path
/// (`\\?\D:\repos\redkiln`). Slash-flipping that yields `//?/D:/repos/redkiln`,
/// which git reads as a URL with host `?` and refuses:
///
/// ```text
/// fatal: '//?/D:/repos/redkiln' does not appear to be a git repository
/// ```
///
/// The prefix must be stripped, not just re-slashed. Caught by the one-item
/// validation before the set was run — `build_silent_fixture` had never once
/// completed, because every caller passes a canonicalised path.
pub fn git_remote_path(p: &Path) -> String {
    let s = p.to_string_lossy().replace('\\', "/");
    s.strip_prefix("//?/").unwrap_or(&s).to_string()
}

/// Junction `node_modules` in from the source repo.
///
/// Without it the reviewer can read code but cannot run the suite, which
/// silently weakens two of the five rubric dimensions it is scored on —
/// `gate-greenness` and `test-integrity`. A reviewer that *cannot* run tests
/// looks like one that *chose not to*, and the difference would be invisible
/// in the digest.
///
/// Windows-only (`mklink /J`); this spike is Windows-only throughout (F8). A
/// failure here is non-fatal but is announced, because a silent absence is the
/// exact confound the junction exists to prevent.
fn link_node_modules(redkiln: &Path, fixture: &Path) -> bool {
    let target = redkiln.join("node_modules");
    if !target.is_dir() {
        eprintln!("    note: no node_modules in the source repo; the reviewer cannot run tests");
        return false;
    }
    let link = fixture.join("node_modules");
    let status = std::process::Command::new("cmd")
        .args([
            "/c",
            "mklink",
            "/J",
            &link.to_string_lossy(),
            &target.to_string_lossy(),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    match status {
        Ok(s) if s.success() && link.join(".bin").exists() => true,
        _ => {
            eprintln!(
                "    WARNING: could not link node_modules into {} — the reviewer cannot run \
                 tests, which weakens gate-greenness and test-integrity",
                fixture.display()
            );
            false
        }
    }
}

/// Confirm the fix really is unreachable — a fixture that silently kept it
/// would reproduce F17 without announcing itself.
pub fn assert_fix_unreachable(fixture: &Path, sha: &str) -> Result<()> {
    let out = std::process::Command::new("git")
        .args(["cat-file", "-e", &format!("{sha}^{{commit}}")])
        .current_dir(fixture)
        .output()?;
    anyhow::ensure!(
        !out.status.success(),
        "fixture at {} still contains fix commit {sha} — the seed is restorable \
         and the arm would be void (F17)",
        fixture.display()
    );
    Ok(())
}

/// The prompt. Deliberately says nothing about whether a defect is present.
pub fn build_prompt(
    slug: &str,
    spec: &str,
    report: &str,
    ac_ids: &[String],
    fix_files: &[String],
) -> String {
    format!(
        "You are reviewing story `{slug}` in this repository.\n\n\
         Your working directory IS the repository under review. Read the actual \
         source and tests — do not take the implementation report at face value.\n\n\
         The acceptance criteria to verify: {acs}\n\n\
         The files this change was scoped to touch:\n{files}\n\n\
         ===== spec.md =====\n{spec}\n\n\
         ===== implementation-report.md =====\n{report}\n\n\
         ===== end of artifacts =====\n\n\
         Adversarially verify each acceptance criterion against the code that is \
         actually present. For every blocking finding, populate `files` with the \
         repo-relative paths it concerns. Return the structured digest.",
        slug = slug,
        acs = if ac_ids.is_empty() { "(see the spec)".to_string() } else { ac_ids.join(", ") },
        files = fix_files
            .iter()
            .map(|f| format!("  - {f}"))
            .collect::<Vec<_>>()
            .join("\n"),
        spec = truncate(spec, 24_000),
        report = truncate(report, 16_000),
    )
}

/// Arm (a)'s wrapper: a parent that DISPATCHES the card rather than being it.
///
/// The review brief is arm (b)'s prompt, forwarded verbatim. That is deliberate
/// and is what makes the arms comparable: identical material, identical card,
/// identical model and effort — the single flipped bit is whether the reviewer
/// is the headless turn itself or a subagent inside it.
///
/// The parent is told not to inspect anything. It cannot be *prevented* from
/// doing so — `--allowed-tools` does not restrict the main turn's tool set
/// (measured, F23) — so compliance is VERIFIED after the fact from the tool
/// calls rather than assumed, and a parent that reviewed for itself is reported
/// separately instead of being silently pooled.
pub fn build_parent_prompt(review_brief: &str) -> String {
    format!(
        "You are dispatching a review. You are NOT the reviewer.\n\n         Do not inspect the repository yourself: do not read files, do not run          commands, do not search the tree. Your entire job is to delegate and          then report what comes back.\n\n         Dispatch the `redkiln-review` subagent EXACTLY ONCE using the Task          tool with `subagent_type: \"redkiln-review\"`, passing it the review          brief below verbatim as its prompt.\n\n         When it returns, return the structured digest. The digest must report          the SUBAGENT's verdict and the SUBAGENT's findings — including its          `files` — not your own assessment. If it found nothing blocking, say          so; do not add findings of your own.\n\n         ===== review brief — pass this to the subagent verbatim =====\n         {review_brief}"
    )
}

/// Tools that constitute reviewing the tree for oneself.
///
/// Used only to check arm (a)'s parent kept to dispatching. `Task`/`Agent`,
/// `StructuredOutput` and the various task-bookkeeping tools are not
/// inspection and do not count.
const INSPECTION_TOOLS: [&str; 6] = ["Read", "Bash", "Grep", "Glob", "PowerShell", "WebFetch"];

/// Arm (a)'s validity checks, computed from the stream's tool calls.
#[derive(Debug, Clone, Serialize)]
pub struct ParentBehaviour {
    /// The parent dispatched a subagent naming OUR card. `subagent_stats`
    /// alone cannot distinguish our card from `general-purpose`.
    pub dispatched_card: bool,
    /// How many subagents the parent dispatched. More than one is not invalid
    /// but is worth seeing: the prompt asks for exactly one.
    pub dispatches: usize,
    /// The parent inspected the tree itself instead of only delegating. Not
    /// void — recorded, and reported separately, because such a record
    /// measures the parent rather than the subagent.
    pub parent_inspected: bool,
    /// Which inspection tools the parent used, if any.
    pub parent_inspection_tools: Vec<String>,
    /// Inspection calls made by the subagent — evidence it did real work.
    pub subagent_inspections: usize,
}

pub fn parent_behaviour(calls: &[crate::event::ToolCall], card_name: &str) -> ParentBehaviour {
    let is_dispatch = |c: &crate::event::ToolCall| c.name == "Agent" || c.name == "Task";
    let parent_calls = calls.iter().filter(|c| c.parent_tool_use_id.is_none());

    let dispatches = parent_calls.clone().filter(|c| is_dispatch(c)).count();
    let dispatched_card = calls
        .iter()
        .filter(|c| is_dispatch(c))
        .any(|c| c.subagent_type.as_deref() == Some(card_name));

    let parent_inspection_tools: Vec<String> = parent_calls
        .filter(|c| INSPECTION_TOOLS.contains(&c.name.as_str()))
        .map(|c| c.name.clone())
        .collect();

    ParentBehaviour {
        dispatched_card,
        dispatches,
        parent_inspected: !parent_inspection_tools.is_empty(),
        parent_inspection_tools,
        subagent_inspections: calls
            .iter()
            .filter(|c| c.parent_tool_use_id.is_some())
            .filter(|c| INSPECTION_TOOLS.contains(&c.name.as_str()))
            .count(),
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        return s.to_string();
    }
    let head: String = s.chars().take(n).collect();
    format!("{head}\n… [truncated at {n} characters]")
}

#[derive(Debug, Clone, Serialize)]
pub struct Scored {
    /// `pass == false` — the reviewer rejected the change.
    pub rejected: bool,
    /// Rejected **and** at least one finding names a file the revert touched.
    /// This is E3's primary criterion, and it is mechanical.
    pub strong_detection: bool,
    /// Rejected, regardless of what it cited. The gap between weak and strong
    /// **is** the rejected-for-the-wrong-reason rate.
    pub weak_detection: bool,
    pub cited_files: Vec<String>,
    pub matched_files: Vec<String>,
    pub issue_count: usize,
    pub overall: Option<i64>,
}

/// Score a digest against the set of files the revert touched.
///
/// Path comparison is normalised on separators and matched on suffix, because a
/// reviewer may cite `src/cli/advance.ts` or `./src/cli/advance.ts` or an
/// absolute path. Being strict about the form would fail real detections.
pub fn score(digest: &serde_json::Value, reverted: &[String]) -> Scored {
    let norm = |s: &str| s.replace('\\', "/").trim_start_matches("./").to_lowercase();
    let targets: Vec<String> = reverted
        .iter()
        // `apply_mutation` annotates deletions; keep only the path.
        .map(|f| norm(f.split(" (").next().unwrap_or(f)))
        .collect();

    let mut cited = Vec::new();
    if let Some(issues) = digest.get("issues").and_then(|i| i.as_array()) {
        for issue in issues {
            if let Some(files) = issue.get("files").and_then(|f| f.as_array()) {
                for f in files.iter().filter_map(|v| v.as_str()) {
                    cited.push(f.to_string());
                }
            }
        }
    }

    let matched: Vec<String> = cited
        .iter()
        .filter(|c| {
            let c = norm(c);
            targets.iter().any(|t| c.ends_with(t.as_str()) || t.ends_with(&c))
        })
        .cloned()
        .collect();

    let rejected = digest.get("pass").and_then(|p| p.as_bool()) == Some(false);
    Scored {
        rejected,
        strong_detection: rejected && !matched.is_empty(),
        weak_detection: rejected,
        issue_count: cited.len(),
        cited_files: cited,
        matched_files: matched,
        overall: digest.get("overall").and_then(|v| v.as_i64()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(pass: bool, files: &[&str]) -> serde_json::Value {
        serde_json::json!({
            "pass": pass,
            "issues": [{ "issue": "x", "fix": "y", "files": files }]
        })
    }

    #[test]
    fn strong_detection_needs_both_rejection_and_a_matching_citation() {
        let reverted = vec!["src/cli/advance.ts".to_string()];

        let hit = score(&digest(false, &["src/cli/advance.ts"]), &reverted);
        assert!(hit.strong_detection);
        assert!(hit.weak_detection);

        // Rejected, but citing something unrelated: weak only. This is the
        // rejected-for-the-wrong-reason case, and it is what stops an
        // always-reject arm from scoring perfectly.
        let wrong = score(&digest(false, &["docs/readme.md"]), &reverted);
        assert!(!wrong.strong_detection);
        assert!(wrong.weak_detection);

        // Approved: neither, whatever it cited.
        let approved = score(&digest(true, &["src/cli/advance.ts"]), &reverted);
        assert!(!approved.strong_detection);
        assert!(!approved.weak_detection);
    }

    #[test]
    fn path_forms_do_not_defeat_a_real_detection() {
        let reverted = vec!["src/process/engine.ts".to_string()];
        for cited in [
            "src/process/engine.ts",
            "./src/process/engine.ts",
            "src\\process\\engine.ts",
            "D:/repos/fixture/src/process/engine.ts",
        ] {
            assert!(
                score(&digest(false, &[cited]), &reverted).strong_detection,
                "failed to match {cited}"
            );
        }
    }

    #[test]
    fn a_deletion_annotation_still_matches() {
        // apply_mutation records added-then-deleted files with a suffix.
        let reverted = vec!["src/new.ts (deleted; added by the fix)".to_string()];
        assert!(score(&digest(false, &["src/new.ts"]), &reverted).strong_detection);
    }

    #[test]
    fn a_digest_with_no_issues_scores_nothing() {
        let d = serde_json::json!({ "pass": false, "issues": [] });
        let s = score(&d, &["src/a.ts".to_string()]);
        assert!(s.weak_detection, "it did reject");
        assert!(!s.strong_detection, "but cited nothing");
        assert_eq!(s.issue_count, 0);
    }

    #[test]
    fn a_canonicalised_windows_path_is_usable_as_a_git_remote() {
        // The extended-length prefix must be stripped, not merely re-slashed:
        // git parses `//?/D:/...` as a URL with host `?`.
        assert_eq!(
            git_remote_path(Path::new(r"\\?\D:\repos\redkiln")),
            "D:/repos/redkiln"
        );
        assert_eq!(git_remote_path(Path::new(r"D:\repos\redkiln")), "D:/repos/redkiln");
        assert!(!git_remote_path(Path::new(r"\\?\D:\repos\redkiln")).starts_with("//?/"));
    }

    fn call(name: &str, parent: Option<&str>, sub: Option<&str>) -> crate::event::ToolCall {
        crate::event::ToolCall {
            name: name.into(),
            parent_tool_use_id: parent.map(str::to_string),
            subagent_type: sub.map(str::to_string),
        }
    }

    #[test]
    fn a_delegating_parent_is_distinguished_from_one_that_reviewed_for_itself() {
        // The shape arm (a) wants: parent dispatches OUR card, touches nothing
        // itself, and the subagent does the inspecting.
        let good = parent_behaviour(
            &[
                call("Agent", None, Some("redkiln-review")),
                call("Read", Some("toolu_1"), None),
                call("Bash", Some("toolu_1"), None),
                call("StructuredOutput", None, None),
            ],
            "redkiln-review",
        );
        assert!(good.dispatched_card);
        assert_eq!(good.dispatches, 1);
        assert!(!good.parent_inspected, "the subagent read, not the parent");
        assert_eq!(good.subagent_inspections, 2);

        // The shape that would silently measure the PARENT instead.
        let bad = parent_behaviour(
            &[
                call("Agent", None, Some("redkiln-review")),
                call("Read", None, None),
                call("StructuredOutput", None, None),
            ],
            "redkiln-review",
        );
        assert!(bad.parent_inspected);
        assert_eq!(bad.parent_inspection_tools, vec!["Read".to_string()]);
    }

    #[test]
    fn dispatching_a_generic_agent_is_not_dispatching_the_card() {
        // RK-B0013 one level up: `subagent_stats.spawned` would read 1 here.
        let b = parent_behaviour(
            &[call("Agent", None, Some("general-purpose"))],
            "redkiln-review",
        );
        assert!(!b.dispatched_card);
        assert_eq!(b.dispatches, 1);
    }

    #[test]
    fn the_parent_prompt_delegates_and_does_not_leak() {
        let p = build_parent_prompt("BRIEF-BODY");
        assert!(p.contains("BRIEF-BODY"), "the brief is forwarded verbatim");
        assert!(p.contains("redkiln-review"));
        let low = p.to_lowercase();
        for leak in ["reverted", "seeded", "defect was", "bug was introduced", "s-silent"] {
            assert!(!low.contains(leak), "parent prompt leaks `{leak}`");
        }
    }

    #[test]
    fn the_card_carries_the_structured_output_channel() {
        // F20: without it the card's own allowlist suppresses `--json-schema`
        // and every dispatch comes back as prose. The cheapest possible guard
        // on a failure mode that voided the pilot without announcing itself.
        assert!(CARD_TOOLS.contains(&"StructuredOutput"));
        assert!(CARD_TOOLS.contains(&"Read") && CARD_TOOLS.contains(&"Bash"));
    }

    #[test]
    fn the_schema_requires_files_on_every_issue() {
        let schema = digest_schema();
        let req = schema["properties"]["issues"]["items"]["required"]
            .as_array()
            .unwrap();
        assert!(req.iter().any(|v| v == "files"));
    }

    #[test]
    fn the_prompt_never_hints_that_a_defect_exists() {
        let p = build_prompt("s", "spec", "report", &["AC-001".into()], &["src/a.ts".into()]);
        let low = p.to_lowercase();
        for leak in ["reverted", "seeded", "defect was", "bug was introduced", "s-silent"] {
            assert!(!low.contains(leak), "prompt leaks `{leak}`");
        }
        assert!(p.contains("AC-001"));
        assert!(p.contains("src/a.ts"));
    }
}
