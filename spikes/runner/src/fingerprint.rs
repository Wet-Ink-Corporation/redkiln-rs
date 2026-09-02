//! Provenance: what was actually sent, and what state the world was in.
//!
//! Everything here exists because it **cannot be retrofitted**. A run recorded
//! without it is not a weaker measurement, it is an unusable one — you cannot go
//! back and ask what the git snapshot was when a cache miss happened.
//!
//! The precedent is `test/helpers/workflow-harness.ts` in the TS tree, which
//! records each dispatch's prompt verbatim and says why: "a source grep for the
//! interpolation cannot tell a string that is built from one that is sent."

use std::path::Path;

use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    // sha2 0.11 returns a byte array without a LowerHex impl.
    hasher
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut acc, b| {
            use std::fmt::Write;
            let _ = write!(acc, "{b:02x}");
            acc
        })
}

/// The fixture's git state at the instant of dispatch.
///
/// E1's entire hypothesis is about the git snapshot: sequential sessions share a
/// cache prefix "only when the git status snapshot at startup matches". Without
/// recording what that snapshot *was*, a cache-share series cannot be correlated
/// with the thing it is supposed to be explained by.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GitSnapshot {
    pub head: Option<String>,
    pub branch: Option<String>,
    /// Number of entries in `git status --porcelain`.
    pub dirty_entries: usize,
    /// Digest of the porcelain output. Two dispatches with the same digest saw
    /// the same working tree; a change here is the candidate explanation for a
    /// prefix bust.
    pub status_digest: String,
}

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(args)
        .current_dir(dir)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

pub fn git_snapshot(dir: &Path) -> Option<GitSnapshot> {
    // If this is not a repo at all, say so by returning None rather than
    // fabricating an empty snapshot that would read as "clean".
    git(dir, &["rev-parse", "--is-inside-work-tree"])?;

    let status = git(dir, &["status", "--porcelain"]).unwrap_or_default();
    Some(GitSnapshot {
        head: git(dir, &["rev-parse", "HEAD"]),
        branch: git(dir, &["rev-parse", "--abbrev-ref", "HEAD"]),
        dirty_entries: status.lines().filter(|l| !l.trim().is_empty()).count(),
        status_digest: sha256_hex(status.as_bytes()),
    })
}

/// What the child says it actually ran with, as opposed to what we asked for.
///
/// Pulled from `system/init`. Verifying that the requested configuration is the
/// effective one is exactly the class of check that silently invalidates an arm
/// when it is skipped — an ignored flag looks identical to an applied one in the
/// results.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EffectiveConfig {
    pub session_id: Option<String>,
    pub model: Option<String>,
    pub agent: Option<String>,
    pub cwd: Option<String>,
    pub permission_mode: Option<String>,
    pub tools: Option<Vec<String>>,
    pub mcp_servers: Option<Vec<String>>,
    pub slash_commands_count: Option<usize>,
    /// Digest of the whole init payload. A change here between two dispatches in
    /// one arm means the environment moved underneath the experiment.
    pub init_digest: String,
}

pub fn effective_config(init: &serde_json::Value) -> EffectiveConfig {
    let s = |k: &str| init.get(k).and_then(|v| v.as_str()).map(str::to_string);
    let list = |k: &str| {
        init.get(k).and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .map(|x| {
                    x.as_str()
                        .map(str::to_string)
                        .or_else(|| x.get("name").and_then(|n| n.as_str()).map(str::to_string))
                        .unwrap_or_else(|| x.to_string())
                })
                .collect()
        })
    };

    EffectiveConfig {
        session_id: s("session_id"),
        model: s("model"),
        agent: s("agent"),
        cwd: s("cwd"),
        permission_mode: s("permissionMode").or_else(|| s("permission_mode")),
        tools: list("tools"),
        mcp_servers: list("mcp_servers").or_else(|| list("mcpServers")),
        slash_commands_count: init
            .get("slash_commands")
            .and_then(|v| v.as_array())
            .map(|a| a.len()),
        init_digest: sha256_hex(init.to_string().as_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_non_repo_yields_none_not_a_clean_snapshot() {
        let tmp = std::env::temp_dir().join(format!("rk-fp-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        // A bare temp dir is (almost always) outside any repo. If the temp dir
        // happens to sit inside one, the assertion below still holds for the
        // property we care about: we never invent a snapshot.
        if let Some(snap) = git_snapshot(&tmp) {
            assert!(snap.head.is_some(), "a real repo must report a head");
        }
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn status_digest_separates_a_dirty_tree_from_a_clean_one() {
        assert_ne!(sha256_hex(b""), sha256_hex(b" M src/main.rs\n"));
    }

    #[test]
    fn effective_config_reads_the_init_payload() {
        let init = serde_json::json!({
            "type": "system", "subtype": "init",
            "session_id": "abc-123",
            "model": "claude-opus-5",
            "agent": "reviewer",
            "cwd": "D:/fixtures/e1",
            "permissionMode": "manual",
            "tools": ["Read", "Bash"],
            "mcp_servers": [{"name": "chrome"}],
            "slash_commands": ["a", "b", "c"]
        });
        let c = effective_config(&init);
        assert_eq!(c.session_id.as_deref(), Some("abc-123"));
        assert_eq!(c.agent.as_deref(), Some("reviewer"));
        assert_eq!(c.model.as_deref(), Some("claude-opus-5"));
        assert_eq!(c.tools.unwrap(), vec!["Read", "Bash"]);
        // An object entry is reduced to its name, not stringified JSON.
        assert_eq!(c.mcp_servers.unwrap(), vec!["chrome"]);
        assert_eq!(c.slash_commands_count, Some(3));
        assert!(!c.init_digest.is_empty());
    }

    #[test]
    fn init_digest_moves_when_the_environment_moves() {
        let a = serde_json::json!({"tools": ["Read"]});
        let b = serde_json::json!({"tools": ["Read", "Bash"]});
        assert_ne!(effective_config(&a).init_digest, effective_config(&b).init_digest);
    }
}
