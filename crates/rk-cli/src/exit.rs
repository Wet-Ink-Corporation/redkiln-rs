//! **The closed exit table, and the one halt that is not a table row** — AC-438 as amended
//! 2026-09-18 (D-01, D-12); ADR-0118 clause 6; `s7d` §7d.4.
//!
//! Eighteen rows: seven that no `StepError` reaches, and eleven that are exactly the closed error
//! set `rk-types` declares. `impl From<&StepError> for Exit` is one `match` with eleven arms and no
//! wildcard, so a twelfth variant is `E0004` at this file rather than a silent remapping somewhere
//! downstream — which is what AC-438's *"a new `StepError` variant with no mapping fails to
//! compile"* asks for. The test that pins the absence of a wildcard is
//! `ac438_step_error_mapping_is_exhaustive`, a source assertion over this impl's own region; keep
//! the variant spellings out of this file's prose, or its exactly-once count reads two.
//!
//! **Why [`Halt`] lives here and not in a `store` module.** The acquisition ADR-0118 clause 5
//! assigns to the composition root is Slice 2's, and `src/store.rs` does not exist yet — but
//! `code_for` and `render_halt` are the library's surface from Slice 1, and both name this type.
//! Declaring it beside the table is what lets them compile with no `store` module (§7d.16 item 1).
//!
//! **Row `70` is not reachable from any fixture** and is not meant to be: no spawned process can
//! have its lock poisoned from outside without the seam AC-474 forbids, so §7d.10's in-process
//! `adr0118_…` is its instrument and the table-driven test asserts only that the row is present.

use rk_types::error::StepError;

/// The exit-code table. Discriminants are explicit and are the process's exit status.
///
/// `0`, `2`, `3`, `4`, `5`, `70` and `130` are the seven rows no error class maps to; the remaining
/// eleven are the image of the closed error set. The gaps between `5` and `10`, and between `20` and
/// `70`, are deliberate: they leave room for a future non-error row without renumbering a class, and
/// a renumbering is what a committed snapshot exists to refuse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    /// Success — **and a run that opens a gate** (D-01: AC-403's *"exits 0"*, AC-240, AC-024), the
    /// AC-450-style no-op (ADR-0107 decision 1) and a `Void` answer, where the gate stays open and
    /// the void is recorded (AC-406).
    Success = 0,
    /// `clap`'s default, kept (H-1): an unknown argument, a missing required flag, a closed-enum
    /// value outside its set, an option the gate's question does not declare, an attachment no
    /// declared option admits or a declared attachment absent.
    Usage = 2,
    /// A refusal of the repository's recorded state, read as configuration of the item: a foreign
    /// store, a missing required key, an adapter refusal, a naming refusal, a non-derivable lane,
    /// no process pack enabled, an unknown or dual-write key, the git version floor, and an
    /// `answer` with no resolvable live token or a spent one.
    ConfigError = 3,
    /// The review gate's block verdict surfaced by `run`. The value exists so the row is distinct;
    /// its producer is the kernel's, built at 7b-ii.
    FindingsBlock = 4,
    /// AC-432's *"an item with no recorded resolution is an error rather than a recomputation"*
    /// (D-12).
    NoRecord = 5,
    /// The spawn, io or decode class.
    Transport = 10,
    /// The terminal schema class (AC-439/AC-428's), and the CLI's end of F20: a platform error that
    /// returned exit `0` with no structured output leaves through this row, never through `0`.
    Schema = 11,
    /// Reached in-process only at this stage — nothing in the Claude adapter produces it, because
    /// the adapter classifies its own timer arm as a cancellation.
    Timeout = 12,
    /// All three cancel causes, the signal one included when the signal landed **during** a
    /// dispatch (§7d.10). The register's *"SIGTERM → 143"* observation is deliberately not adopted:
    /// `143` is the shell's convention for an UNHANDLED `SIGTERM`, and this process handles it.
    Cancelled = 13,
    /// A child that exited non-zero with a recorded stderr digest.
    NonZero = 14,
    /// After the durable retry mark is written (AC-404's exit half).
    RateLimited = 15,
    /// The wrong agent answered (ADR-0095 decision 2).
    NotAdopted = 16,
    /// A declared isolation surface disagreed with the card.
    NotIsolated = 17,
    /// AC-428's class.
    Platform = 18,
    /// A capability the harness does not carry.
    Unsupported = 19,
    /// A budget or billing state that does not improve by being repeated.
    Overage = 20,
    /// ADR-0118 clause 6. Not an error class and not constructed from one: the run stopped at the
    /// poisoned acquisition, nothing crossed a port boundary, and this is an explicit exit after
    /// the message rather than the unhandled panic's `101`.
    PoisonedLockHalt = 70,
    /// A Ctrl-C, `CTRL_BREAK` or `SIGTERM` with no dispatch in flight (RA-M-04). `130` is the
    /// conventional Ctrl-C code; it is a value in the snapshot, not an appeal to convention.
    SignalOutsideDispatch = 130,
}

impl Exit {
    /// The process exit status this row carries.
    #[must_use]
    pub const fn code(self) -> i32 {
        self as i32
    }

    /// The row's variant name, as the committed snapshot renders it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Usage => "Usage",
            Self::ConfigError => "ConfigError",
            Self::FindingsBlock => "FindingsBlock",
            Self::NoRecord => "NoRecord",
            Self::Transport => "Transport",
            Self::Schema => "Schema",
            Self::Timeout => "Timeout",
            Self::Cancelled => "Cancelled",
            Self::NonZero => "NonZero",
            Self::RateLimited => "RateLimited",
            Self::NotAdopted => "NotAdopted",
            Self::NotIsolated => "NotIsolated",
            Self::Platform => "Platform",
            Self::Unsupported => "Unsupported",
            Self::Overage => "Overage",
            Self::PoisonedLockHalt => "PoisonedLockHalt",
            Self::SignalOutsideDispatch => "SignalOutsideDispatch",
        }
    }
}

/// Every row, in code order — the value the committed snapshot renders and the set
/// `ac438_exit_codes_distinct` measures the error image against.
///
/// The length is written into the type. A row added without a snapshot update is red on the
/// snapshot; a row added without touching this array does not exist.
pub const ALL: [Exit; 18] = [
    Exit::Success,
    Exit::Usage,
    Exit::ConfigError,
    Exit::FindingsBlock,
    Exit::NoRecord,
    Exit::Transport,
    Exit::Schema,
    Exit::Timeout,
    Exit::Cancelled,
    Exit::NonZero,
    Exit::RateLimited,
    Exit::NotAdopted,
    Exit::NotIsolated,
    Exit::Platform,
    Exit::Unsupported,
    Exit::Overage,
    Exit::PoisonedLockHalt,
    Exit::SignalOutsideDispatch,
];

impl From<&StepError> for Exit {
    fn from(err: &StepError) -> Self {
        match err {
            StepError::Transport { .. } => Self::Transport,
            StepError::Schema { .. } => Self::Schema,
            StepError::Timeout { .. } => Self::Timeout,
            StepError::Cancelled { .. } => Self::Cancelled,
            StepError::NonZero { .. } => Self::NonZero,
            StepError::RateLimited { .. } => Self::RateLimited,
            StepError::NotAdopted { .. } => Self::NotAdopted,
            StepError::NotIsolated { .. } => Self::NotIsolated,
            StepError::Platform { .. } => Self::Platform,
            StepError::Unsupported { .. } => Self::Unsupported,
            StepError::Overage { .. } => Self::Overage,
        }
    }
}

/// The run stopped, and no row of the table describes it.
///
/// ADR-0118 clause 1: *"A poisoned store lock halts the run. The run stops at the poisoned
/// acquisition. No `StepError` is constructed, nothing crosses a port boundary, no retry, no next
/// activity, no gate answer."* Every verb therefore returns `Result<Exit, Halt>` rather than an
/// `Exit`, so the in-process twin of a halt can observe a returned value instead of a dead test
/// process.
///
/// Both members are `&'static str` because both sets are closed at compile time: the two locks
/// clause 4 names, and the nine verbs the router declares (ADR-0163 added `cost`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Halt {
    /// A lock was found poisoned — a writer panicked while holding it. The type is matched, never
    /// recovered: no `into_inner`, no `catch_unwind` (clause 3).
    PoisonedLock {
        /// Which of clause 4's two locks: the event store's, or the store writer's.
        lock: &'static str,
        /// The verb that was running when the acquisition failed.
        verb: &'static str,
    },
}
