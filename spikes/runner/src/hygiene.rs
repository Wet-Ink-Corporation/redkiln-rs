//! Environment assertions that run before every dispatch.
//!
//! Charter §7.1: "Assert environment hygiene before every run". These are not
//! diagnostics — a failure here voids the run, because each of them silently
//! changes what is being measured:
//!
//! - `ANTHROPIC_API_KEY` present converts a rate-limited subscription run into
//!   a billed per-token one (charter §6.2).
//! - `--bare` does not use the subscription at all; we never pass it, and we
//!   assert nothing else has.
//! - The `claude` version is recorded on every record because the headless
//!   surface has no documented stability policy (RFC R8).

use std::process::Stdio;

use anyhow::{Context, Result, bail};

/// The floor from charter §6.4. Below this, `promptCacheTtl` and the
/// `rate_limits` / `prompt_cache` statusline objects are unavailable.
pub const MIN_CLAUDE_VERSION: (u32, u32, u32) = (2, 1, 251);

#[derive(Debug, Clone, serde::Serialize)]
pub struct Environment {
    pub claude_version: String,
    pub claude_version_ok: bool,
    pub api_key_absent: bool,
    pub platform: String,
}

/// Parse a `claude --version` line such as "2.1.258 (Claude Code)".
fn parse_version(raw: &str) -> Option<(u32, u32, u32)> {
    let token = raw.split_whitespace().next()?;
    let mut parts = token.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next()?.parse().ok()?;
    // A trailing pre-release suffix ("251-rc1") must not sink the parse.
    let patch_token = parts.next()?;
    let patch = patch_token
        .split(|c: char| !c.is_ascii_digit())
        .next()?
        .parse()
        .ok()?;
    Some((major, minor, patch))
}

/// Collect the environment facts, failing closed on anything that would change
/// what a measured run means.
pub async fn assert_clean(claude_bin: &str) -> Result<Environment> {
    // A stray key silently converts a rate-limited run into a billed one, so
    // this is a hard refusal rather than a warning.
    let api_key_absent =
        !matches!(std::env::var("ANTHROPIC_API_KEY"), Ok(v) if !v.trim().is_empty());
    if !api_key_absent {
        bail!(
            "ANTHROPIC_API_KEY is set. A measured run must draw on the subscription, \
             not per-token billing (charter §6.2). Unset it and re-run."
        );
    }

    let output = tokio::process::Command::new(claude_bin)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .with_context(|| format!("could not execute `{claude_bin} --version`"))?;

    if !output.status.success() {
        bail!(
            "`{claude_bin} --version` exited {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

    let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let parsed = parse_version(&raw)
        .with_context(|| format!("could not parse a version out of `{raw}`"))?;
    let version_ok = parsed >= MIN_CLAUDE_VERSION;
    if !version_ok {
        bail!(
            "claude {}.{}.{} is below the {}.{}.{} floor (charter §6.4)",
            parsed.0,
            parsed.1,
            parsed.2,
            MIN_CLAUDE_VERSION.0,
            MIN_CLAUDE_VERSION.1,
            MIN_CLAUDE_VERSION.2
        );
    }

    Ok(Environment {
        claude_version: raw,
        claude_version_ok: version_ok,
        api_key_absent,
        platform: std::env::consts::OS.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_shipped_version_string() {
        assert_eq!(parse_version("2.1.258 (Claude Code)"), Some((2, 1, 258)));
    }

    #[test]
    fn tolerates_a_prerelease_suffix() {
        assert_eq!(parse_version("2.1.251-rc1"), Some((2, 1, 251)));
    }

    #[test]
    fn the_installed_floor_compares_correctly() {
        assert!(parse_version("2.1.258 (Claude Code)").unwrap() >= MIN_CLAUDE_VERSION);
        assert!(parse_version("2.1.242").unwrap() < MIN_CLAUDE_VERSION);
    }

    #[test]
    fn rejects_junk() {
        assert_eq!(parse_version("not a version"), None);
    }
}
