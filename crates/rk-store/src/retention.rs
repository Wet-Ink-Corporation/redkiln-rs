//! Retention — ADR-0105 clause 7's last rule.
//!
//! *"a dispatch record keeps argv and environment as **digests only**, stdout and stderr
//! **verbatim up to a size cap** (then digest + head/tail), the child environment **never**;
//! `doctor` carries a rule row for credential-shaped values in any retained field."*
//!
//! The environment is handled by there being **no field to put it in**. That is not pedantry: a
//! field plus a rule to leave it empty is a field someone eventually fills, and the child
//! environment is where credentials live.

use rk_types::digest::{Digest, sha256};

/// How much stdout/stderr is kept verbatim before it becomes digest + head/tail.
pub const STDIO_CAP: usize = 64 * 1024;

/// How much of each end survives the cap.
pub const HEAD_TAIL: usize = 2 * 1024;

/// A retained stream: verbatim while small, summarised once it is not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Retained {
    Verbatim(Vec<u8>),
    Capped {
        digest: Digest,
        head: Vec<u8>,
        tail: Vec<u8>,
        total_len: usize,
    },
}

impl Retained {
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        if bytes.len() <= STDIO_CAP {
            Self::Verbatim(bytes.to_vec())
        } else {
            Self::Capped {
                digest: sha256(bytes),
                head: bytes[..HEAD_TAIL].to_vec(),
                tail: bytes[bytes.len() - HEAD_TAIL..].to_vec(),
                total_len: bytes.len(),
            }
        }
    }
}

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
