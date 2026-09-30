//! **AC-349's claims binder** — which configured toolchain a changed path belongs to, and an
//! equal-specificity tie as a configuration error naming both rules and the path.
//! `s8g`'s verify record (R1 = ADR-0191 decision 10, proposed): *"AC-349's claims binder reads a
//! `claims` column this record adds to ADR-0170's per-toolchain table"*. *(Added 2026-09-29, `s8g`
//! build, package S8G-P3-KERNEL-GIT.)*
//!
//! AC-349: *"GIVEN two selection rules or two toolchain claims matching the same changed path at
//! equal specificity WHEN the binder resolves selection THEN it is a configuration error reported
//! at plan time, never a silent choice AND the error names both rules and the path."* Design §4.3
//! row 1's rule, applied to path claims: the binder never orders two candidates by file order, load
//! order or name.
//!
//! - **A claim** is a glob over repository-relative `/`-separated paths: `*` matches within one
//!   segment, `**` any number of whole segments (none included), `?` one character; everything
//!   else is literal. A claim's rule id is `<toolchain>:<pattern>`.
//! - **Specificity** is the pair (whole literal segments, literal characters), compared in that
//!   order: `crates/rk-cli/Cargo.toml` is more specific than `**/Cargo.toml`, which is more specific
//!   than `**/*.toml`. The most specific matching claim binds the path.
//! - **A tie** is two matching claims of DIFFERENT toolchains at the greatest specificity: a
//!   [`ClaimTie`], never the first one. Two claims of one toolchain name one answer and are no tie.
//!
//! The toolchain names are the per-toolchain tables' keys (`rust`, `node`): facet elements, never
//! an adapter identity (AC-487).

use std::cmp::Ordering;

use crate::toml_subset::Value;
use crate::workflow::memo::VerbRegistry;

/// The declaration row a per-toolchain table carries its claims in (`workflow::memo`'s
/// `DECLARATION_ROWS` admits it).
pub const CLAIMS_ROW: &str = "claims";

/// One toolchain's claim on a set of paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathClaim {
    /// The per-toolchain table's key.
    pub toolchain: String,
    /// The glob.
    pub pattern: String,
}

impl PathClaim {
    /// `<toolchain>:<pattern>`, the rule id an error names.
    #[must_use]
    pub fn rule_id(&self) -> String {
        format!("{}:{}", self.toolchain, self.pattern)
    }
}

/// Two claims of different toolchains matched one path at equal specificity: a configuration
/// error (AC-349), naming both rule ids and the path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimTie {
    pub path: String,
    pub first: PathClaim,
    pub second: PathClaim,
}

impl std::fmt::Display for ClaimTie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the toolchain claims `{}` and `{}` both match `{}` at equal specificity: a \
             configuration error, never a guess (AC-349; RK-D-TOOLCHAIN-CLAIM-TIE)",
            self.first.rule_id(),
            self.second.rule_id(),
            self.path
        )
    }
}

impl std::error::Error for ClaimTie {}

fn is_literal(segment: &str) -> bool {
    !segment.contains(['*', '?'])
}

/// A claim's specificity: (whole literal segments, literal characters).
#[must_use]
pub fn specificity(pattern: &str) -> (usize, usize) {
    let segments = pattern.split('/').filter(|s| is_literal(s)).count();
    let chars = pattern
        .chars()
        .filter(|c| !matches!(c, '*' | '?' | '/'))
        .count();
    (segments, chars)
}

/// One segment against one glob segment (`*` and `?` only; no `/` inside a segment).
fn segment_matches(pattern: &[char], text: &[char]) -> bool {
    match pattern.split_first() {
        None => text.is_empty(),
        Some(('*', rest)) => (0..=text.len()).any(|i| segment_matches(rest, &text[i..])),
        Some(('?', rest)) => !text.is_empty() && segment_matches(rest, &text[1..]),
        Some((c, rest)) => text.first() == Some(c) && segment_matches(rest, &text[1..]),
    }
}

fn segments_match(pattern: &[&str], path: &[&str]) -> bool {
    match pattern.split_first() {
        None => path.is_empty(),
        Some((&"**", rest)) => (0..=path.len()).any(|i| segments_match(rest, &path[i..])),
        Some((seg, rest)) => path.split_first().is_some_and(|(first, tail)| {
            let p: Vec<char> = seg.chars().collect();
            let t: Vec<char> = first.chars().collect();
            segment_matches(&p, &t) && segments_match(rest, tail)
        }),
    }
}

/// Does `pattern` match the repository-relative `path`?
#[must_use]
pub fn matches(pattern: &str, path: &str) -> bool {
    let p: Vec<&str> = pattern.split('/').collect();
    let t: Vec<&str> = path.split('/').collect();
    segments_match(&p, &t)
}

/// **Bind one path**: the most specific matching claim, `None` when no claim matches.
///
/// # Errors
///
/// [`ClaimTie`] when two claims of different toolchains match at the greatest specificity. The
/// order of `claims` never decides: the pair is reported in rule-id order.
pub fn bind<'a>(claims: &'a [PathClaim], path: &str) -> Result<Option<&'a PathClaim>, ClaimTie> {
    let mut best: Vec<&PathClaim> = Vec::new();
    let mut best_spec = (0, 0);
    for claim in claims.iter().filter(|c| matches(&c.pattern, path)) {
        let spec = specificity(&claim.pattern);
        match (best.is_empty(), spec.cmp(&best_spec)) {
            (true, _) | (false, Ordering::Greater) => {
                best = vec![claim];
                best_spec = spec;
            }
            (false, Ordering::Equal) => best.push(claim),
            (false, Ordering::Less) => {}
        }
    }
    best.sort_by_key(|c| c.rule_id());
    if let Some(other) = best.iter().find(|c| c.toolchain != best[0].toolchain) {
        return Err(ClaimTie {
            path: path.to_owned(),
            first: best[0].clone(),
            second: (*other).clone(),
        });
    }
    Ok(best.first().copied())
}

/// **The claims column**: every `[toolchain.<name>]` table's `claims = { value = [<glob>, …],
/// source = "…" }` row, in table order. A table without the row claims nothing.
///
/// # Errors
///
/// A string naming the table whose `claims` value is not a list of strings.
pub fn claims_from(registry: &VerbRegistry) -> Result<Vec<PathClaim>, String> {
    let mut out = Vec::new();
    for toolchain in registry.toolchain_names() {
        let Some(row) = registry
            .table(toolchain)
            .and_then(|t| t.declarations().get(CLAIMS_ROW))
        else {
            continue;
        };
        let Value::Array(items) = &row.value else {
            return Err(format!(
                "toolchain.{toolchain}.claims is not a list of path globs"
            ));
        };
        for item in items {
            let Some(pattern) = item.as_str() else {
                return Err(format!(
                    "toolchain.{toolchain}.claims carries a member that is not a string"
                ));
            };
            out.push(PathClaim {
                toolchain: toolchain.to_owned(),
                pattern: pattern.to_owned(),
            });
        }
    }
    Ok(out)
}
