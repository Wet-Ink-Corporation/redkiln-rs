//! Retention — ADR-0105 clause 7's last rule.
//!
//! *"a dispatch record keeps argv and environment as **digests only**, stdout and stderr
//! **verbatim up to a size cap** (then digest + head/tail), the child environment **never**;
//! `doctor` carries a rule row for credential-shaped values in any retained field."*
//!
//! The environment is handled by there being **no field to put it in**. That is not pedantry: a
//! field plus a rule to leave it empty is a field someone eventually fills, and the child
//! environment is where credentials live.
//!
//! **`Retained`, `STDIO_CAP` and `HEAD_TAIL` moved to `rk-types` at Step 7a and are re-exported
//! here unchanged**, so `rk_store::retention::Retained` still resolves and every caller and test
//! is untouched. The move was forced, not chosen: ADR-0099:62 homes `DispatchRecord` in `rk-types`
//! (*"Both are `rk-types`"*) and runbook `s7a` §7a.5 authors that record's minimal form there with
//! the ADR-0105 clause 7 retention fields on it — which `rk-types` cannot name while they live in
//! a crate that depends on it (`rk-types -> rk-store` is a cycle cargo refuses). Re-declaring the
//! enum in `rk-types` instead would have created a second one-name-two-crates collision of exactly
//! the kind §7a.5 is repairing for `DispatchRecord`. **One type, one declaration, two paths.** The
//! record naming its home is OWED with the four other §7a.5 routings.
//!
//! `DispatchRecord` below is **not** moved: two types under one name in two crates is the
//! collision `s7a` §7a.5 names, and collapsing it is 7c's.

use rk_types::digest::{Digest, sha256};
pub use rk_types::retention::{HEAD_TAIL, Retained, STDIO_CAP};

/// One dispatch's retained record.
///
/// Note what is **absent**: there is no `env` field, and no `argv` field either — only their
/// digests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchRecord {
    pub argv_digest: Digest,
    pub env_digest: Digest,
    pub instrument_version: String,
    pub stdout: Retained,
    pub stderr: Retained,
}

impl DispatchRecord {
    /// Build a record from the raw material, digesting what must be digested.
    ///
    /// `env` is taken by value and **dropped**: it is digested and never stored. Taking it at all is
    /// deliberate — the digest is what lets `doctor` notice that two dispatches ran under different
    /// environments without ever retaining either.
    #[must_use]
    pub fn new(
        argv: &[String],
        env: &[(String, String)],
        instrument_version: &str,
        stdout: &[u8],
        stderr: &[u8],
    ) -> Self {
        let argv_joined = argv.join("\u{1f}");
        let mut env_sorted: Vec<String> = env.iter().map(|(k, v)| format!("{k}={v}")).collect();
        env_sorted.sort();
        Self {
            argv_digest: sha256(argv_joined.as_bytes()),
            env_digest: sha256(env_sorted.join("\u{1f}").as_bytes()),
            instrument_version: instrument_version.to_string(),
            stdout: Retained::of(stdout),
            stderr: Retained::of(stderr),
        }
    }
}

/// Common token prefixes, and the shapes that carry secrets.
const CREDENTIAL_PREFIXES: [&str; 8] = [
    "sk-",
    "ghp_",
    "gho_",
    "github_pat_",
    "xoxb-",
    "xoxp-",
    "AKIA",
    "Bearer ",
];

/// Does this value look like a credential? `doctor`'s rule row (clause 7).
///
/// Deliberately **shape-based and deliberately noisy**: the cost of a false positive is a `doctor`
/// row an operator reads and dismisses; the cost of a false negative is a credential in a retained
/// field. The asymmetry is the whole design, and it is stated here so that a later pass tightening
/// this for tidiness knows what it is trading away.
#[must_use]
pub fn looks_like_credential(value: &str) -> bool {
    let v = value.trim();
    if v.len() < 16 {
        return false;
    }
    if CREDENTIAL_PREFIXES.iter().any(|p| v.starts_with(p)) {
        return true;
    }
    if v.contains("BEGIN") && v.contains("PRIVATE KEY") {
        return true;
    }
    // A long unbroken high-entropy-looking run: mixed case plus digits, no whitespace.
    let no_space = !v.contains(char::is_whitespace);
    let has_upper = v.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = v.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = v.chars().any(|c| c.is_ascii_digit());
    no_space && has_upper && has_lower && has_digit && v.len() >= 32
}
