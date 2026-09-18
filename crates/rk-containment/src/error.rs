//! `ContainError` — what a containment call can fail with, as an enum a caller matches on rather
//! than a string it reads.
//!
//! One variant per primitive of ADR-0127 decision 2's surface (*"create the group, spawn into it,
//! kill it, enumerate it, drop it"* — drop cannot fail — plus the wait `run_bounded` performs),
//! each carrying the `std::io::Error` the OS call surfaced, so the OS's own code
//! (`ERROR_ACCESS_DENIED`, `ESRCH`, …) is readable by `raw_os_error()` and never re-encoded.
//! `Display` embeds that error's text and `source()` is `None` — the house shape (`rk-kernel`'s
//! errors embed and return no source), so a chained reporter prints the OS error once, and the
//! typed access is [`ContainError::os_error`]. Hand-written `Display`, not `thiserror`: ADR-0127 decision 2 fixes this crate's dependencies as
//! *"`std`, `windows-sys` (Windows) and `libc` (Unix)"* and §7c.19's manifest row says *"and
//! nothing else"*, which an accepted record's sentence outranks ADR-0102 decision 6's
//! *"`thiserror` in every library"* on (ADR-0092's precedence) — the crate that owns the audit
//! surface carries the smallest tree, and `rk-kernel` already writes its `Display` by hand.
//!
//! Nothing here classifies: *timed out* and *killed by the OS* are values of
//! [`crate::Outcome`], not errors, because a timed-out verify run is a result the toolchain adapter
//! maps onto `Unverifiable` (AC-331), and an `Err` would let it be read as a transport fault.

use std::fmt;
use std::io;

/// The failed primitive and the OS error behind it.
#[derive(Debug)]
pub enum ContainError {
    /// The group could not be created (`CreateJobObjectW` / `SetInformationJobObject`; POSIX has
    /// no create step and never returns this).
    Create(io::Error),
    /// The child could not be started (`CreateProcessW` through `std`; `fork`/`exec`).
    Spawn(io::Error),
    /// The child started but could not be assigned to the group, or the reading that it was could
    /// not be taken; the child was terminated before this was returned, so no child runs outside
    /// the group (ADR-0099's forbids).
    Assign(io::Error),
    /// The suspended child could not be resumed; the child was terminated before this was returned.
    Resume(io::Error),
    /// The group could not be killed (`TerminateJobObject`; `killpg`), or a member outlived the
    /// kill's bounded wait.
    Kill(io::Error),
    /// The group could not be enumerated (`QueryInformationJobObject`; `/proc`).
    Enumerate(io::Error),
    /// `Child::wait` failed, or the waiting thread went away without reporting.
    Wait(io::Error),
}

impl ContainError {
    /// The OS error behind the variant, whichever it is.
    #[must_use]
    pub const fn os_error(&self) -> &io::Error {
        match self {
            Self::Create(e)
            | Self::Spawn(e)
            | Self::Assign(e)
            | Self::Resume(e)
            | Self::Kill(e)
            | Self::Enumerate(e)
            | Self::Wait(e) => e,
        }
    }
}

impl fmt::Display for ContainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Create(e) => write!(f, "containment: the group could not be created: {e}"),
            Self::Spawn(e) => write!(f, "containment: the child could not be spawned: {e}"),
            Self::Assign(e) => write!(
                f,
                "containment: the child could not be assigned to the group (it was terminated): {e}"
            ),
            Self::Resume(e) => write!(
                f,
                "containment: the suspended child could not be resumed (it was terminated): {e}"
            ),
            Self::Kill(e) => write!(f, "containment: the group could not be killed: {e}"),
            Self::Enumerate(e) => write!(f, "containment: the group could not be enumerated: {e}"),
            Self::Wait(e) => write!(f, "containment: the child could not be waited on: {e}"),
        }
    }
}

impl std::error::Error for ContainError {}
