//! **The one environment read** (ADR-0083; `s7d` §7d.5, §7d.15).
//!
//! ADR-0083: *"The process may read environment variables once, at the edge, while building
//! `Config`."* This module is that edge, and it is the **only** place in `rk-cli` that may name
//! `std::env::var` or its kin — Checkpoint 7d's ban is scoped to `crates/rk-cli/src` with a
//! module-path exclusion for exactly this file, and `redkiln-migrate` has no exclusion at all
//! (AC-474: no env toggle can change its write path).
//!
//! **Why once, and why a value.** A program that reads the environment wherever it happens to need
//! it has as many configurations as it has read sites, and no two of them are provably the same. The
//! captured map is taken once and handed on; everything downstream takes it as an argument, which is
//! what makes a test able to construct one.
//!
//! **The captured map is NOT carried into the kernel** (**RT-34**): it is handed to
//! [`crate::adapters::resolve`] beside the config file, and what reaches `rk_kernel::config::Config`
//! is the resolved outcome. A kernel holding the process environment would be reading it one
//! indirection later, which is the whole of what ADR-0083 forbids.

use std::collections::BTreeMap;

/// The process environment as it was at startup.
///
/// `BTreeMap` and not `HashMap`: ordering is observable wherever this is rendered or digested, and
/// `clippy.toml` disallows `HashMap`/`HashSet` workspace-wide for that reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env(BTreeMap<String, String>);

impl Env {
    /// Read the process environment. **The one read**, called once, from `main`.
    ///
    /// Lossy names and values are dropped rather than lossily converted: a variable whose name is
    /// not UTF-8 cannot match a denylist entry or a config key, and inventing a replacement
    /// character for it would make it match something it is not.
    #[must_use]
    pub fn capture() -> Self {
        Self(
            std::env::vars_os()
                .filter_map(|(k, v)| match (k.into_string(), v.into_string()) {
                    (Ok(k), Ok(v)) => Some((k, v)),
                    _ => None,
                })
                .collect(),
        )
    }

    /// Build one from a map. **This is how every test gets an `Env`**, and no test calls
    /// [`Self::capture`]: a test that read the real environment would pass or fail on whatever this
    /// box exports — and on this box `ANTHROPIC_FOUNDRY_API_KEY` is persisted at user scope, so a
    /// test built on the real environment would be red for a reason that has nothing to do with it.
    #[must_use]
    pub const fn from_map(vars: BTreeMap<String, String>) -> Self {
        Self(vars)
    }

    /// The captured map, for the one consumer that reads it.
    #[must_use]
    pub const fn as_map(&self) -> &BTreeMap<String, String> {
        &self.0
    }

    /// **`GIT_LFS_SKIP_SMUDGE`, as captured** (`s8d-ii` §8d-ii.5 read (4); D21-47 V1; ADR-0180
    /// decision 12, proposed): the value run-tree creation hands `rk_git::worktree::enter`, which
    /// refuses when it is set to anything but `0`. `rk-git` reads no environment (ADR-0083), so the
    /// read is here, off the one capture. *(dated 2026-09-27, `s8d-ii` build, P6.)*
    #[must_use]
    pub fn lfs_skip_smudge(&self) -> Option<&str> {
        self.0.get("GIT_LFS_SKIP_SMUDGE").map(String::as_str)
    }
}
