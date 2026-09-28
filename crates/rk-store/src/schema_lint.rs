//! The declared-key-domain lint — AC-166.
//!
//! *"GIVEN a store table that references the repository … THEN every key column referencing the
//! repository declares a key domain of `sha` or `tree` AND a repo-referencing key column declaring
//! any other domain — or none — is a schema-lint error."*
//!
//! The criterion was **repaired** into this shape, and the repair is the point: an earlier version
//! asked the lint to recognise a branch name by inspection, which it cannot do — `TEXT` is `TEXT`.
//! Declaring the domain makes it decidable. So each migration is a pair: `NNNN_name.sql`, whose
//! repo-referencing columns carry a `-- repo-key` marker, and `NNNN_name.keys`, which declares each
//! one's domain.
//!
//! The lint runs as a **test**, not a `build.rs`. A failing build script gives a worse diagnostic,
//! runs on every consumer's machine, and adds a supply-chain surface, for a check whose whole job
//! is to be read by whoever broke it.

use std::collections::BTreeMap;

/// The two admissible domains. A repo reference is a commit or a tree; anything else — a branch, a
/// tag, a ref name — moves under you, which is the failure the criterion exists to stop.
pub const DOMAINS: [&str; 2] = ["sha", "tree"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LintError {
    /// A column marked `-- repo-key` in the SQL with no entry in the `.keys` file.
    UndeclaredDomain {
        column: String,
    },
    /// A `.keys` entry naming a column that the SQL does not mark.
    DeclaredButNotMarked {
        column: String,
    },
    /// A `.keys` entry whose domain is not `sha` or `tree` — a `branch TEXT` key is the fixture.
    BadDomain {
        column: String,
        domain: String,
    },
    Malformed {
        line: String,
    },
}

impl std::fmt::Display for LintError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndeclaredDomain { column } => write!(
                f,
                "`{column}` is marked `-- repo-key` but declares no key domain (AC-166). Add \
                 `{column} = sha` or `{column} = tree` to the migration's .keys file."
            ),
            Self::DeclaredButNotMarked { column } => write!(
                f,
                "`{column}` declares a key domain but is not marked `-- repo-key` in the SQL. The \
                 two files have drifted."
            ),
            Self::BadDomain { column, domain } => write!(
                f,
                "`{column}` declares domain `{domain}`; only `sha` and `tree` are admissible \
                 (AC-166). A branch or ref name is not a stable repository reference."
            ),
            Self::Malformed { line } => write!(f, "malformed .keys line: `{line}`"),
        }
    }
}

/// Columns the SQL marks as repo-referencing, as `table.column`.
#[must_use]
pub fn marked_columns(sql: &str) -> Vec<String> {
    let mut table = String::new();
    let mut out = Vec::new();
    for line in sql.lines() {
        let t = line.trim();
        if let Some(rest) = t
            .strip_prefix("CREATE TABLE IF NOT EXISTS ")
            .or_else(|| t.strip_prefix("CREATE TABLE "))
        {
            table = rest.trim_end_matches(" (").trim().to_string();
            continue;
        }
        // A line that IS a comment is prose, even when it names the marker — the migration's own
        // header explains what `-- repo-key` means and must not be read as declaring a column
        // called `--`. Guard scoped too wide on the first pass (golden constraint 9); this is the
        // fourth assertion in Step 6 to trip on text that merely names the thing it governs.
        if t.starts_with("--") {
            continue;
        }
        if t.contains("-- repo-key")
            && let Some(col) = t.split_whitespace().next()
        {
            out.push(format!("{table}.{col}"));
        }
    }
    out
}

/// Parse a `.keys` file into `table.column -> domain`.
///
/// # Errors
/// [`LintError::Malformed`] on a line that is neither blank, a comment, nor `a.b = domain`.
pub fn parse_keys(text: &str) -> Result<BTreeMap<String, String>, LintError> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let Some((col, domain)) = t.split_once('=') else {
            return Err(LintError::Malformed {
                line: t.to_string(),
            });
        };
        out.insert(col.trim().to_string(), domain.trim().to_string());
    }
    Ok(out)
}

/// Lint one migration pair.
///
/// # Errors
/// Every mismatch found, as a list — the whole set rather than the first, so a migration is fixed
/// in one pass.
pub fn lint(sql: &str, keys: &str) -> Result<(), Vec<LintError>> {
    let declared = match parse_keys(keys) {
        Ok(d) => d,
        Err(e) => return Err(vec![e]),
    };
    let marked = marked_columns(sql);
    let mut errors = Vec::new();

    for col in &marked {
        match declared.get(col) {
            None => errors.push(LintError::UndeclaredDomain {
                column: col.clone(),
            }),
            Some(domain) if !DOMAINS.contains(&domain.as_str()) => {
                errors.push(LintError::BadDomain {
                    column: col.clone(),
                    domain: domain.clone(),
                });
            }
            Some(_) => {}
        }
    }
    for col in declared.keys() {
        if !marked.contains(col) {
            errors.push(LintError::DeclaredButNotMarked {
                column: col.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// The migrations shipped with this binary, as `(name, sql, keys)`, in the order they apply.
///
/// *(dated 2026-09-28, s8e build; ADR-0181 decision 5, proposed.)* `0002_gate_transitions` is
/// D21-46 R2's forward migration. `rk_store::migrate::apply` runs each once per store.
#[must_use]
pub fn all() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "0001_initial",
            include_str!("migrations/0001_initial.sql"),
            include_str!("migrations/0001_initial.keys"),
        ),
        (
            "0002_gate_transitions",
            include_str!("migrations/0002_gate_transitions.sql"),
            include_str!("migrations/0002_gate_transitions.keys"),
        ),
    ]
}
