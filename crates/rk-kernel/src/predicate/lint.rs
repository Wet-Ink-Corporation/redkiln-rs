//! The load-time predicate lint — ADR-0103 decision 4's **one** narrowing pattern (**AC-069**).
//!
//! ADR-0069 makes ordering against `null` a lint error and its own negative consequence names the
//! escape hatch: *"authors who want an ordering-like check against a possibly-absent field must
//! explicitly guard with an equality/`!= null` check first"*. ADR-0103 decision 4 closes the shape
//! of that guard to exactly one: **`x != null and <ordering on x>` at the same conjunction level,
//! no `or`, no `not`.**
//!
//! **The pattern is syntactic, not semantic** (AC-069's *Verified by*; PIA-052). That is the whole
//! rule and it is why two predicates that a human reads as equivalent must both fail load:
//! `not (novelty == unknown) and novelty >= high` fails because it carries a `not`, and
//! `novelty == unknown or novelty >= high` fails because it carries an `or` — neither is the
//! pattern, and a lint that reasoned about meaning instead of form would admit both and reopen the
//! silent `unknown` → `high` promotion AC-069 exists to make impossible. F36 measured what that
//! costs: `unknown` rates of novelty 46.0%, size 39.5%, blast_radius 31.0%, criticality 27.0%, and
//! `unknown` is treated as `high` **by position**, so an axis oscillating unknown↔low moves the
//! required set the full band width.
//!
//! # Two guard spellings, and the warrant for the second
//!
//! ADR-0103 writes the guard `x != null`. AC-069's *Verified by* names as its **must-load** fixture
//! `novelty != unknown and novelty >= high` — `!= unknown`, not `!= null` — because `unknown` is
//! the ordinal domain's own abstention member and reaches a predicate exactly the way `null` does
//! (AC-069's 2026-09-09 scope note, and ADR-0089 on tri-valued booleans). Both records are on
//! AC-069's `decisions:` line, so this lint recognises **both spellings and no third**. An erratum
//! reconciling the two records' wording is not written here.
//!
//! # What is not here
//!
//! The **pack-level** fixtures — AC-069's four narrowing-pattern rows under
//! `crates/rk-kernel/tests/fixtures/packs/` — are Slice 2's, with the registry loader that supplies
//! the nullable set. This module takes the nullable set as an argument precisely so that it does
//! not have to invent one.

use std::collections::BTreeSet;

use super::ast::{CmpOp, Expr, Literal};

/// A predicate that must not load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintError {
    /// An ordering comparison reached a nullable axis with no recognised narrowing guard.
    UnguardedOrderingAgainstNullable {
        /// The axis the ordering was applied to.
        path: String,
        /// The operator's source spelling, so the message names the site.
        op: &'static str,
    },
}

impl std::fmt::Display for LintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnguardedOrderingAgainstNullable { path, op } => write!(
                f,
                "ordering `{path} {op} …` against the nullable axis `{path}` with no narrowing \
                 guard: the ONE pattern the lint recognises is `{path} != null and <ordering on \
                 {path}>` at the same conjunction level, with no `or` and no `not` (ADR-0103 \
                 decision 4; AC-069)"
            ),
        }
    }
}

impl std::error::Error for LintError {}

/// Reject a predicate that orders a nullable axis without the one narrowing pattern.
///
/// `nullable` is the set of axes that may abstain — the registry's `nullable = true` entries
/// (AC-072, DECISIONS-2026-09-12 E1).
///
/// # Errors
///
/// [`LintError::UnguardedOrderingAgainstNullable`] naming the first offending axis.
pub fn lint_predicate(expr: &Expr, nullable: &BTreeSet<String>) -> Result<(), LintError> {
    walk(expr, nullable, &BTreeSet::new())
}

/// Flatten an `and` spine into its conjuncts, left to right. Groups, `or` and `not` end the spine:
/// they are a different conjunction level and the pattern is defined at one level.
fn conjuncts<'a>(expr: &'a Expr, out: &mut Vec<&'a Expr>) {
    if let Expr::And(l, r) = expr {
        conjuncts(l, out);
        conjuncts(r, out);
    } else {
        out.push(expr);
    }
}

/// The axis a conjunct guards, if the conjunct *is* the guard.
fn guarded_axis(expr: &Expr) -> Option<String> {
    let Expr::Cmp { op, left, right } = expr else {
        return None;
    };
    if *op != CmpOp::Ne {
        return None;
    }
    let Expr::Path(path) = left.as_ref() else {
        return None;
    };
    match right.as_ref() {
        Expr::Lit(Literal::Null) => Some(path.clone()),
        // AC-069's own must-load fixture spelling; see this module's header.
        Expr::Lit(Literal::Bare(v) | Literal::Str(v)) if v == "unknown" => Some(path.clone()),
        _ => None,
    }
}

fn walk(
    expr: &Expr,
    nullable: &BTreeSet<String>,
    guarded: &BTreeSet<String>,
) -> Result<(), LintError> {
    match expr {
        Expr::And(..) => {
            let mut parts = Vec::new();
            conjuncts(expr, &mut parts);
            let mut running = guarded.clone();
            for part in parts {
                walk(part, nullable, &running)?;
                if let Some(axis) = guarded_axis(part) {
                    running.insert(axis);
                }
            }
            Ok(())
        }
        // An `or` opens a new conjunction level and the pattern forbids one, so no guard survives
        // into either branch. This is what fails `novelty == unknown or novelty >= high`.
        Expr::Or(l, r) => {
            walk(l, nullable, &BTreeSet::new())?;
            walk(r, nullable, &BTreeSet::new())
        }
        // A `not` is forbidden inside the pattern, so nothing under one is guarded. This is what
        // fails `not (novelty == unknown) and novelty >= high`.
        Expr::Not(inner) => walk(inner, nullable, &BTreeSet::new()),
        // Parentheses around a term do not change its conjunction level, and a `len()` is a value
        // position rather than a conjunction one.
        Expr::Group(inner) | Expr::Len(inner) => walk(inner, nullable, guarded),
        Expr::Cmp { op, left, right } => {
            if op.is_ordering() {
                for side in [left.as_ref(), right.as_ref()] {
                    if let Expr::Path(p) = side
                        && nullable.contains(p)
                        && !guarded.contains(p)
                    {
                        return Err(LintError::UnguardedOrderingAgainstNullable {
                            path: p.clone(),
                            op: op.as_str(),
                        });
                    }
                }
            }
            walk(left, nullable, guarded)?;
            walk(right, nullable, guarded)
        }
        Expr::In { left, list } => {
            walk(left, nullable, guarded)?;
            walk(list, nullable, guarded)
        }
        Expr::Member { literal, set } => {
            walk(literal, nullable, guarded)?;
            walk(set, nullable, guarded)
        }
        Expr::Quant { source, body, .. } => {
            walk(source, nullable, guarded)?;
            // A quantifier body is its own scope and its own conjunction level.
            walk(body, nullable, &BTreeSet::new())
        }
        Expr::List(items) => {
            for item in items {
                walk(item, nullable, guarded)?;
            }
            Ok(())
        }
        Expr::Path(_) | Expr::Ref(_) | Expr::Lit(_) => Ok(()),
    }
}
