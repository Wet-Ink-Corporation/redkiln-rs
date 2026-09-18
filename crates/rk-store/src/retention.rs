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
//! **`DispatchRecord` followed at Step 7c Slice 1** (2026-09-17; ADR-0126 decision 2: *"one type,
//! one declaration"*). The five-field record this module declared from Step 6.3 to Step 7b-ii —
//! the collision `s7a` §7a.5 named — is gone, and the name below is a re-export of the completed
//! `rk_types::activity::DispatchRecord`, exactly as `Retained` is. **Its constructor went with
//! it**, and could not have stayed: an inherent `impl` on a type another crate declares is E0116,
//! and the five-argument `new` could not have built the grown record in any case (`cli_version`,
//! `api_retry_events`, the two TTL values and `timings` are required members) — so
//! `rk_types::activity::DispatchRecord::new(argv, env, instrument_version, stdout, stderr,
//! RecordTail { .. })` is the constructor, and this module re-exports `RecordTail` beside the
//! record so a caller that reached the constructor through this path still can (the two TTL values
//! are two `String` members, not a type — `rk_types::activity`'s doc on `prompt_cache_ttl`).
//! `Retained`, `STDIO_CAP`, `HEAD_TAIL` and the constructor all being `rk-types`' is also why
//! `rk-adapter-claude` takes no `rk-store` edge (`s7c` §7c.19): nothing in this module is the
//! Harness adapter's. What stays here is what is this crate's — `looks_like_credential`, the
//! `doctor` rule row of clause 7.

pub use rk_types::activity::{DispatchRecord, RecordTail};
pub use rk_types::retention::{HEAD_TAIL, Retained, STDIO_CAP};

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
