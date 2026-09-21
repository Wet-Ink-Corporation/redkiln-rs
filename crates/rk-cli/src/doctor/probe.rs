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
//! # ✅ Cycle 7 built it, and the argv is the assertion
//!
//! `[<program>, "--version"]` and nothing else. The child is spawned through
//! `rk_containment::bounded::run_bounded`, which kills through the job on timeout — an
//! unresponsive harness binary must not park a verb whose whole contract is that it
//! terminates and reports.
//!
//! `ProbeError::NotBuilt` is **gone**: it existed only while the body did not.

use std::fmt;

/// Why a version could not be read. **A value, never a refusal the verb makes**: an unprobeable
/// harness binary is what `RK-D-CLI-VERSION` reports, so this reaches a rule and not the exit
/// table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    /// The child could not be spawned, timed out, or exited non-zero. The runner's own message is
    /// carried through rather than parsed.
    Unprobeable(String),
}

impl fmt::Display for ProbeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unprobeable(detail) => {
                write!(f, "the harness binary could not be probed: {detail}")
            }
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

/// How long the probe waits for `--version`.
///
/// Generous for a version print and far short of `run_bounded`'s dispatch bound: the probe
/// is one line of output from a local binary, and a harness that cannot manage it in five
/// seconds is itself the finding `RK-D-CLI-VERSION` reports.
const PROBE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

/// The one implementor on the product path: it spawns.
///
#[derive(Debug, Clone, Copy, Default)]
pub struct RealProbe;

impl VersionProbe for RealProbe {
    #[expect(
        clippy::disallowed_types,
        reason = "`s7d` §7d.15 names this site: the probe's `Command` is built here and handed to                   `rk_containment::run_bounded`, which is the one runner permitted to spawn. The                   type is disallowed so that every spawn is deliberate, and this one is."
    )]
    fn version(&self, program: &str) -> Result<String, ProbeError> {
        // 🛑 ONE CHILD, TWO ARGV ELEMENTS, AND NO `-p`.
        //
        // AC-423's second instrument is that this is a SPAWN and not a DISPATCH: SM-121's counter
        // stays at zero across a `doctor` run that probes. `-p` is the flag that carries a prompt,
        // so the argv is exactly `[program, "--version"]` and nothing builds it from a list a
        // caller could extend.
        //
        // **stdout goes to a FILE, not a pipe, and that is `run_bounded`'s shape rather than a
        // preference.** `Outcome` reports how the child ended and carries no streams, because the
        // runner's job is the bound and the kill — so the one way to keep both the bound and the
        // output is to give the child a file to write and read it after. A pipe would need a
        // reader thread this function does not own.
        let sink = tempfile::NamedTempFile::new()
            .map_err(|e| ProbeError::Unprobeable(format!("no temp file for the probe: {e}")))?;
        let handle = sink
            .reopen()
            .map_err(|e| ProbeError::Unprobeable(format!("the probe sink could not open: {e}")))?;

        let mut cmd = std::process::Command::new(program);
        cmd.arg("--version")
            .stdout(std::process::Stdio::from(handle))
            .stderr(std::process::Stdio::null());

        match rk_containment::run_bounded(cmd, PROBE_TIMEOUT) {
            Ok(rk_containment::Outcome::Exited(_)) => {
                let text = std::fs::read_to_string(sink.path()).map_err(|e| {
                    ProbeError::Unprobeable(format!("the probe output could not be read: {e}"))
                })?;
                Ok(text.trim().to_owned())
            }
            Ok(rk_containment::Outcome::TimedOut) => Err(ProbeError::Unprobeable(format!(
                "`{program} --version` did not answer within {PROBE_TIMEOUT:?} and was killed"
            ))),
            Ok(rk_containment::Outcome::OsKilled) => Err(ProbeError::Unprobeable(format!(
                "`{program} --version` was killed by the operating system"
            ))),
            Err(e) => Err(ProbeError::Unprobeable(format!("{e:?}"))),
        }
    }
}
