//! S0.3 — corpus extraction. Zero model cost.
//!
//! Produces the fix-lane item manifest that S0.1 (gate false-positive rate) and
//! E3/E6 (seeded-defect detection) both consume. One rig, two experiments.
//!
//! Deliberately parses by line rather than pulling in a YAML crate. The
//! frontmatter here is flat `key: value` and we only ever read it — RFC §3.5
//! spends a page on why Rust YAML is a minefield, and none of that risk is worth
//! taking for a read-only data-prep step.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

/// One fix-lane item, with everything the downstream experiments need.
#[derive(Debug, Clone, Serialize)]
pub struct FixLaneItem {
    pub slug: String,
    pub item_id: Option<String>,
    pub dir: String,

    pub ac_ids: Vec<String>,
    pub ac_count: usize,

    /// The checkpoint commit, confirmed to resolve against git. Frontmatter is
    /// excluded from the search: some reviews carry `template_sig` /
    /// `rendered_sig`, 8-hex values a loose scan mistakes for a commit.
    pub checkpoint_sha: Option<String>,
    pub sha_resolves: bool,
    /// How the SHA was located: "labelled" or "body-scan". Recorded so a
    /// mis-parse is visible in the manifest rather than inferred later.
    pub sha_method: Option<&'static str>,

    /// Files the fix commit touched. **E3's primary metric depends on this**:
    /// detection requires the reviewer's findings to cite a file the revert
    /// touched, and that test is only mechanical because this set is known.
    pub fix_files: Vec<String>,

    pub verdict: Option<String>,
    pub overall: Option<i64>,

    pub has_spec: bool,
    pub has_report: bool,
    pub has_review: bool,

    /// Usable as an E3/S0.1 input: needs a resolving SHA, at least one AC, and
    /// a known file set to revert.
    pub eligible: bool,
    pub ineligible_reason: Option<String>,
}

fn read(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok()
}

/// Pull a flat `key: value` out of the leading `---` frontmatter block only.
/// Restricting to the block matters: `verdict` also appears in prose.
pub fn frontmatter_value(text: &str, key: &str) -> Option<String> {
    let mut lines = text.lines();
    if lines.next()?.trim() != "---" {
        return None;
    }
    for line in lines {
        if line.trim() == "---" {
            return None;
        }
        if let Some(rest) = line.strip_prefix(&format!("{key}:")) {
            let v = rest.trim().trim_matches('"').trim_matches('\'').to_string();
            return (!v.is_empty()).then_some(v);
        }
    }
    None
}

/// Acceptance-criterion ids, in order of first appearance and deduplicated.
///
/// Specs cross-reference their own ACs heavily (8,754 raw mentions across 1,737
/// distinct ids corpus-wide), so counting mentions would inflate every item.
pub fn ac_ids(spec: &str) -> Vec<String> {
    let mut seen = Vec::new();
    for line in spec.lines() {
        // Definition sites only. Two forms are in use across the corpus and
        // neither is documented as canonical: a list item (`- AC-001: ...`) and
        // a table row (`| AC-001 | GIVEN ... |`). Handling only the first
        // silently reports zero ACs for table-formatted specs, which reads as
        // "this item has no acceptance criteria" rather than "the parser is
        // incomplete" — and would have dropped them from E3's inputs.
        let t = line.trim_start();
        let Some(rest) = t
            .strip_prefix("- ")
            .or_else(|| t.strip_prefix("* "))
            .or_else(|| t.strip_prefix("| "))
        else {
            continue;
        };
        let rest = rest.trim_start_matches(['*', '`', ' ']);
        if !rest.starts_with("AC-") {
            continue;
        }
        let id: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        if id.len() >= 6 && !seen.contains(&id) {
            seen.push(id);
        }
    }
    seen
}

/// Strip the leading `---` frontmatter block.
///
/// Everything downstream searches the body only, because some reviews carry
/// `template_sig` / `rendered_sig` — 8-hex values that a naive scan cheerfully
/// mistakes for a commit.
pub fn body(text: &str) -> String {
    let mut lines = text.lines();
    if lines.next().map(str::trim) != Some("---") {
        return text.to_string();
    }
    let mut rest = lines.skip_while(|l| l.trim() != "---");
    match rest.next() {
        // Consumed the closing fence; everything after it is body.
        Some(_) => rest.collect::<Vec<_>>().join("\n"),
        // Unterminated frontmatter — treat the whole document as body rather
        // than silently returning nothing.
        None => text.to_string(),
    }
}

/// Candidate checkpoint SHAs in priority order, each tagged with how it was
/// found so the choice is auditable rather than magic.
///
/// The corpus is not consistent: observed forms include `Checkpoint SHA: \`x\``,
/// `Checkpoint commit \`x\``, `- SHA: \`x\``, and reviews that simply cite the
/// commit in prose. Rather than guess a label vocabulary, offer labelled
/// candidates first and let the caller confirm against git — resolution is the
/// real test, not the wording.
pub fn sha_candidates(review: &str) -> Vec<(String, &'static str)> {
    let body = body(review);
    let body = body.as_str();
    let mut labelled = Vec::new();
    let mut loose = Vec::new();

    for line in body.lines() {
        let lower = line.to_ascii_lowercase();
        let is_labelled = lower.contains("checkpoint") || lower.contains("sha:");
        for tok in backticked_hex(line) {
            if is_labelled {
                if !labelled.iter().any(|(s, _): &(String, _)| *s == tok) {
                    labelled.push((tok, "labelled"));
                }
            } else if !loose.iter().any(|(s, _): &(String, _)| *s == tok) {
                loose.push((tok, "body-scan"));
            }
        }
    }
    labelled.extend(loose);
    labelled
}

fn backticked_hex(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == '`'
            && let Some(end) = bytes[i + 1..].iter().position(|c| *c == '`')
        {
            {
                let tok: String = bytes[i + 1..i + 1 + end].iter().collect();
                if tok.len() >= 7 && tok.len() <= 40 && tok.chars().all(|c| c.is_ascii_hexdigit()) {
                    out.push(tok);
                }
                i += end + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn git(repo: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn commit_files(repo: &Path, sha: &str) -> Vec<String> {
    git(
        repo,
        &["show", "--pretty=format:", "--name-only", sha],
    )
    .map(|s| {
        s.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect()
    })
    .unwrap_or_default()
}

/// Scan `.bklg/support/**` for items carrying a `_review.md`.
pub fn scan_fix_lane(redkiln: &Path) -> Result<Vec<FixLaneItem>> {
    let support = redkiln.join(".bklg").join("support");
    anyhow::ensure!(
        support.is_dir(),
        "{} is not a directory — is --redkiln pointing at the redkiln repo?",
        support.display()
    );

    let mut dirs: Vec<PathBuf> = Vec::new();
    collect_review_dirs(&support, &mut dirs)?;
    dirs.sort();

    let mut items = Vec::new();
    for dir in dirs {
        let spec = read(&dir.join("spec.md"));
        let review = read(&dir.join("_review.md"));
        let has_report = dir.join("implementation-report.md").is_file();

        let acs = spec.as_deref().map(ac_ids).unwrap_or_default();
        // Resolution against git is the test, not the wording of the label.
        let (sha, sha_method) = review
            .as_deref()
            .map(sha_candidates)
            .unwrap_or_default()
            .into_iter()
            .find(|(s, _)| git(redkiln, &["cat-file", "-e", &format!("{s}^{{commit}}")]).is_some())
            .map(|(s, m)| (Some(s), Some(m)))
            .unwrap_or((None, None));
        let resolves = sha.is_some();
        let fix_files = sha
            .as_deref()
            .map(|s| commit_files(redkiln, s))
            .unwrap_or_default();

        let reason = if sha.is_none() {
            Some("no resolving checkpoint SHA in the review".to_string())
        } else if acs.is_empty() {
            Some("spec yields no AC-### ids".to_string())
        } else if fix_files.is_empty() {
            Some("fix commit touches no files".to_string())
        } else {
            None
        };

        items.push(FixLaneItem {
            slug: dir
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default(),
            item_id: spec.as_deref().and_then(|s| frontmatter_value(s, "item")),
            dir: dir
                .strip_prefix(redkiln)
                .unwrap_or(&dir)
                .to_string_lossy()
                .replace('\\', "/"),
            ac_count: acs.len(),
            ac_ids: acs,
            checkpoint_sha: sha,
            sha_resolves: resolves,
            sha_method,
            fix_files,
            verdict: review.as_deref().and_then(|r| frontmatter_value(r, "verdict")),
            overall: review
                .as_deref()
                .and_then(|r| frontmatter_value(r, "overall"))
                .and_then(|v| v.parse().ok()),
            has_spec: spec.is_some(),
            has_report,
            has_review: review.is_some(),
            eligible: reason.is_none(),
            ineligible_reason: reason,
        });
    }
    Ok(items)
}

fn collect_review_dirs(root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(root).with_context(|| format!("reading {}", root.display()))? {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }
        if path.join("_review.md").is_file() {
            out.push(path.clone());
        }
        collect_review_dirs(&path, out)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frontmatter_reads_only_the_leading_block() {
        let doc = "---\nverdict: approved\noverall: 3\n---\n\nBody says verdict: rejected\n";
        assert_eq!(frontmatter_value(doc, "verdict").as_deref(), Some("approved"));
        assert_eq!(frontmatter_value(doc, "overall").as_deref(), Some("3"));
        // A key that appears only in the body must not be found.
        assert_eq!(frontmatter_value(doc, "nothere"), None);
    }

    #[test]
    fn frontmatter_requires_a_leading_fence() {
        assert_eq!(frontmatter_value("verdict: approved\n", "verdict"), None);
    }

    #[test]
    fn frontmatter_strips_quotes() {
        let doc = "---\ntitle: \"Fix #77: partial stage\"\n---\n";
        assert_eq!(
            frontmatter_value(doc, "title").as_deref(),
            Some("Fix #77: partial stage")
        );
    }

    #[test]
    fn ac_ids_are_definition_sites_deduplicated_in_order() {
        let spec = "\
# Spec

- AC-001: GIVEN a thing WHEN it happens THEN it works
- AC-002: GIVEN another
  continuation mentioning AC-001 again
- **AC-003**: bolded definition site

Prose referring to AC-001 and AC-002 should not add anything.
";
        assert_eq!(ac_ids(spec), vec!["AC-001", "AC-002", "AC-003"]);
    }

    #[test]
    fn ac_ids_are_read_from_table_rows_too() {
        // Two specs in the corpus define their ACs as table rows. Parsing only
        // list items reported zero for both, which reads as "no acceptance
        // criteria" rather than "incomplete parser", and dropped them from E3.
        let spec = "| AC | Criterion | Evidence |
| --- | --- | --- |
| AC-001 | **GIVEN** a thing **WHEN** it happens **THEN** it works | `test/a.test.ts` |
| AC-002 | **GIVEN** another | `test/b.test.ts` |
";
        assert_eq!(ac_ids(spec), vec!["AC-001", "AC-002"]);
    }

    #[test]
    fn a_table_separator_row_is_not_an_ac() {
        assert!(ac_ids("| --- | --- |
").is_empty());
    }

    #[test]
    fn prose_mentions_alone_do_not_create_acs() {
        // 8,754 raw mentions across 1,737 distinct ids corpus-wide: counting
        // mentions would inflate every item several-fold.
        let spec = "The fix satisfies AC-001 and AC-002 as described.\n";
        assert!(ac_ids(spec).is_empty());
    }

    #[test]
    fn frontmatter_signatures_are_never_offered_as_shas() {
        // template_sig / rendered_sig are 8-hex and live in frontmatter. A
        // first-hex-token scan grabs one and silently reverts the wrong commit.
        let review = "---
template_sig: d2b8c604
rendered_sig: a76236bd
---

Checkpoint SHA: `9271c30c`
";
        let c = sha_candidates(review);
        assert_eq!(c[0], ("9271c30c".to_string(), "labelled"));
        assert!(!c.iter().any(|(s, _)| s == "d2b8c604"));
    }

    #[test]
    fn labelled_candidates_outrank_body_mentions() {
        let review = "---
---

See also `aaaaaaaa` in passing.
Checkpoint commit `bbbbbbbb`.
";
        let c = sha_candidates(review);
        assert_eq!(c[0], ("bbbbbbbb".to_string(), "labelled"));
        assert_eq!(c[1], ("aaaaaaaa".to_string(), "body-scan"));
    }

    #[test]
    fn a_bare_prose_sha_is_still_offered() {
        // Several reviews cite the commit with no label at all; resolution
        // against git is what confirms it, not the wording.
        let c = sha_candidates("---
---

Approved at `fe8d8b03`.
");
        assert_eq!(c, vec![("fe8d8b03".to_string(), "body-scan")]);
    }

    #[test]
    fn non_hex_and_short_tokens_are_not_shas() {
        assert!(sha_candidates("---
---

Checkpoint SHA: `pending`
").is_empty());
        assert!(sha_candidates("---
---

see `abc`
").is_empty());
    }

    #[test]
    fn body_drops_frontmatter_but_keeps_everything_after() {
        let doc = "---
template_sig: d2b8c604
---

Checkpoint SHA: `9271c30c`
";
        let b = body(doc);
        assert!(!b.contains("d2b8c604"));
        assert!(b.contains("9271c30c"));
    }

    #[test]
    fn body_of_an_unfenced_document_is_the_whole_document() {
        assert_eq!(body("no frontmatter here
"), "no frontmatter here
");
    }

    #[test]
    fn unterminated_frontmatter_does_not_swallow_the_document() {
        // Returning "" here would silently drop every SHA in the file.
        let doc = "---
item: RK-S0001
still in frontmatter
";
        assert!(body(doc).contains("RK-S0001"));
    }
}
