//! Paths handed to `git` — drive-letter form, never `\\?\` (ADR-0105 clause 6; §6.2 constraint 4).
//!
//! Lifted **with edits** from `runner/src/e3.rs:399` in `redkiln-corpus` at
//! `27d97e9b164d09dc755fe38e353de922cb97c482` (the SHA in `CORPUS_PIN`), which is itself the fix
//! for F20/2. The original:
//!
//! ```text
//! pub fn git_remote_path(p: &Path) -> String {
//!     let s = p.to_string_lossy().replace('\\', "/");
//!     s.strip_prefix("//?/").unwrap_or(&s).to_string()
//! }
//! ```
//!
//! The defect it fixed: `std::fs::canonicalize` returns `\\?\D:\repos\redkiln` on Windows, which
//! slash-flips to `//?/D:/repos/redkiln`, which `git` parses as a URL with host `?` —
//! `fatal: '//?/D:/repos/redkiln' does not appear to be a git repository`. **No code path ever
//! reached the fetch successfully**, and no unit test could catch it, because none touched the
//! filesystem or the CLI (golden constraint 4).
//!
//! **The edit, and why it is not cosmetic.** The corpus version strips the prefix unconditionally,
//! so `\\?\UNC\server\share` becomes `UNC/server/share` — a path that is not the one given, is not
//! a UNC path any more, and will fail somewhere further away from the cause. A normaliser that
//! turns one broken path into a different broken path has moved the failure, not fixed it. This
//! port **refuses** that shape with a typed error instead.

use std::path::Path;

use crate::error::GitError;

/// A path in the form `git` accepts: drive-letter, forward slashes, no `\\?\` prefix.
///
/// Constructed only through [`WorktreePath::new`], so a path that reaches a `git` invocation has
/// necessarily been through the refusal above — the constraint is structural rather than a rule
/// callers are asked to remember.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorktreePath(String);

impl WorktreePath {
    /// Normalise `p` for handing to `git`, or refuse it.
    ///
    /// # Errors
    /// [`GitError::ExtendedLengthPath`] when `p` carries an extended-length prefix this function
    /// cannot safely remove — today, the `\\?\UNC\` device form.
    pub fn new(p: &Path) -> Result<Self, GitError> {
        let raw = p.to_string_lossy().to_string();
        let slashed = raw.replace('\\', "/");

        // Both spellings of the extended-length prefix: the verbatim form as Windows writes it and
        // the slash-flipped form the corpus defect produced.
        for pfx in ["//?/", "\\\\?\\"] {
            if let Some(rest) = slashed.strip_prefix(pfx) {
                // `//?/UNC/server/share` denotes `\\server\share`. Stripping the prefix yields
                // `UNC/server/share`, which denotes nothing. Refuse rather than mangle.
                if rest.starts_with("UNC/") || rest.starts_with("unc/") {
                    return Err(GitError::ExtendedLengthPath { given: raw });
                }
                return Ok(Self(rest.to_string()));
            }
        }
        Ok(Self(slashed))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WorktreePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
