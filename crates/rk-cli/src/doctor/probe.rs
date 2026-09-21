//! **The installed-CLI version probe** — `doctor`'s one child process (D-11; `s7d` §7d.8).
//!
//! `RK-D-CLI-VERSION` and `RK-D-TTL` are about the harness binary **as installed on this box**, not
//! about the pin in this workspace's source. There is no way to learn that without running it, so
//! `doctor` spawns exactly one child: `[<harness_binary>, "--version"]`.
//!
//! # 🛑 A SPAWN IS NOT A DISPATCH, and the distinction is the whole of AC-423's second instrument
//!
//! SM-121's counter stays at **0** across a `doctor` run that probes. A dispatch goes through
//! [`rk_ports::harness::Harness`] — which on this path is
//! [`crate::no_dispatch::NoDispatchHarness`] and panics — and costs model tokens; a `--version`
//! spawn goes through `rk-containment`'s bounded runner and costs none.
//! `ac423_probe_is_one_version_spawn_not_a_dispatch` asserts all three at once: the spawn count is
//! exactly one, its argv is the binary and `--version` and **carries no `-p`**, and the dispatch
//! counter is zero.
//!
//! # Why a trait and not a function
//!
//! Two tests need opposite things from it. `ac423_ac526_cli_version_and_ttl_mismatch_rejected`
//! needs to *supply* a version — including the empty string, which is the finding's own case — and
//! must not spawn anything. `ac423_probe_is_one_version_spawn_not_a_dispatch` needs the real one
//! and counts its children. A free function could serve the second and not the first, and a
//! version read from an environment variable would be a seam the shipped binary also has.
//!
//! # 🚧 CYCLE 2: the signature is real and [`RealProbe::version`] is not yet built
//!
//! It returns [`ProbeError::NotBuilt`] rather than panicking (this crate's rule —
//! `verbs/mod.rs`'s header: a stub reaching for `unimplemented!()` exits `101` through a path the
//! exit table does not describe). Cycle 7 fills it with the bounded spawn, and
//! `ac423_probe_is_one_version_spawn_not_a_dispatch`'s red on **zero spawns** is behavioural.

use std::fmt;

/// Why a version could not be read. **A value, never a refusal the verb makes**: an unprobeable
/// harness binary is what `RK-D-CLI-VERSION` reports, so this reaches a rule and not the exit
/// table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    /// The child could not be spawned, timed out, or exited non-zero. The runner's own message is
    /// carried through rather than parsed.
    Unprobeable(String),
    /// 🚧 Cycle 2 only — [`RealProbe`] has no body yet. Cycle 7 removes this arm.
    NotBuilt,
}

impl fmt::Display for ProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unprobeable(detail) => {
                write!(f, "the harness binary could not be probed: {detail}")
            }
            Self::NotBuilt => f.write_str("the version probe is not built until Slice 4 cycle 7"),
        }
    }
}

/// Read the installed harness binary's version string.
pub trait VersionProbe: Send + Sync {
    /// Probe `program`, returning its `--version` output verbatim.
    ///
    /// **Verbatim, and untrimmed of meaning**: `RK-D-CLI-VERSION`'s finding case is an *empty*
    /// version string, so a probe that normalised an empty answer into an error would delete the
    /// one case the row exists for.
    ///
    /// # Errors
    /// [`ProbeError`] — which a rule reports, never the verb.
    fn version(&self, program: &str) -> Result<String, ProbeError>;
}

/// The one implementor on the product path: it spawns.
///
/// 🚧 Cycle 2 — the body is cycle 7's. See the module header.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealProbe;

impl VersionProbe for RealProbe {
    fn version(&self, program: &str) -> Result<String, ProbeError> {
        // 🚧 Cycle 7 replaces this with the one bounded spawn: a `Command` for `program` with the
        // single argument `--version` (and NO `-p`), handed to `rk_containment::run_bounded`.
        // `_ = program` keeps the parameter named and used, so the signature the later cycle fills
        // is the signature asserted here rather than one clippy pruned in between.
        let _ = program;
        Err(ProbeError::NotBuilt)
    }
}
