//! `GitOps` — the journalled seam (ADR-0105 clause 1).
//!
//! *"`GitOps` is a **struct** (so AC-483's port-inventory denylist stands) inside `rk-git` that
//! records every git invocation and its result to the EventStore; on replay the kernel is served
//! the journal and `rk-git` performs **zero** writes (AC-006 and AC-008 gain 'and zero rk-git
//! writes')."*
//!
//! **A struct, never a trait.** Git is deliberately not one of the five ports (ADR-0003): it is a
//! concrete crate the kernel names (AC-483), and Checkpoint 7a counts exactly five
//! `pub trait …: Send + Sync` in `rk-ports`. Making this a trait would break that count and move
//! git onto the port surface the design keeps it off.
//!
//! **Zero writes on replay is structural, not a promise.** In [`Mode::Replay`] the wrapper is not
//! reachable: every method matches on the mode *first* and returns the recorded result. There is no
//! branch in which a replayed call reaches `GitExec`, and `writes_performed` counts what actually
//! happened so a test can assert the zero rather than trust it.
//!
//! The journal is written through a [`GitJournal`], which is **crate-local and not a port**. At 7b
//! the composition root supplies an implementation backed by the EventStore — held as
//! `Arc<Mutex<dyn EventStore>>` under ADR-0115, with the lock taken at the call boundary and never
//! held across another port's dispatch.

use crate::error::GitError;
use crate::git_exec::{GitExec, GitOp, GitOutput};
use crate::path::WorktreePath;
use rk_types::digest::Digest;

/// One journalled invocation.
///
/// argv is recorded as a **digest only**, and stdout/stderr verbatim up to a cap then digested —
/// ADR-0105 clause 7's retention rule. The child environment is not recorded at all, and there is
/// no field here in which it could be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitInvocation {
    pub op: &'static str,
    pub cwd: String,
    pub argv_digest: Digest,
    pub status: i32,
    pub stdout: Vec<u8>,
    pub stderr_digest: Digest,
}

/// Where invocations are recorded. Crate-local by design: it is not a sixth port.
pub trait GitJournal {
    fn record(&mut self, inv: GitInvocation);
    fn next_recorded(&mut self) -> Option<GitInvocation>;
}

/// An in-memory journal — the fake, and what the replay tests drive.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct VecJournal {
    pub entries: Vec<GitInvocation>,
    cursor: usize,
}

impl VecJournal {
    #[must_use]
    pub fn from_entries(entries: Vec<GitInvocation>) -> Self {
        Self { entries, cursor: 0 }
    }
}

impl GitJournal for VecJournal {
    fn record(&mut self, inv: GitInvocation) {
        self.entries.push(inv);
    }

    fn next_recorded(&mut self) -> Option<GitInvocation> {
        let out = self.entries.get(self.cursor).cloned();
        if out.is_some() {
            self.cursor += 1;
        }
        out
    }
}

/// Live, or replaying a recorded run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Live,
    Replay,
}

/// The activity-side seam. Every git operation an activity performs goes through here.
pub struct GitOps<'j> {
    exec: GitExec,
    journal: &'j mut dyn GitJournal,
    mode: Mode,
    /// How many invocations actually reached the wrapper. On replay this must stay zero, and the
    /// test asserts the counter rather than inferring the property.
    writes_performed: usize,
}

impl<'j> GitOps<'j> {
    pub fn new(exec: GitExec, journal: &'j mut dyn GitJournal, mode: Mode) -> Self {
        Self {
            exec,
            journal,
            mode,
            writes_performed: 0,
        }
    }

    #[must_use]
    pub const fn writes_performed(&self) -> usize {
        self.writes_performed
    }

    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Run `op`, journalling it — or, on replay, serving it back from the journal.
    ///
    /// # Errors
    /// [`GitError`] from the wrapper when live; [`GitError::NonZeroExit`] reconstructed from the
    /// journal when replaying a recorded failure. A replay with no remaining entry is a
    /// [`GitError::Spawn`] naming the exhaustion, never a silent live call.
    pub fn run(
        &mut self,
        op: GitOp,
        cwd: &WorktreePath,
        args: &[&str],
    ) -> Result<GitOutput, GitError> {
        // The mode is matched FIRST. There is no path from Replay to `self.exec`.
        match self.mode {
            Mode::Replay => {
                let Some(rec) = self.journal.next_recorded() else {
                    return Err(GitError::Spawn {
                        detail: format!(
                            "replay: the journal is exhausted, but `{}` was requested. A replay \
                             must never fall through to a live git invocation (ADR-0105 clause 1).",
                            op.as_str()
                        ),
                    });
                };
                if rec.status == 0 {
                    Ok(GitOutput {
                        stdout: rec.stdout,
                        stderr: Vec::new(),
                    })
                } else {
                    Err(GitError::NonZeroExit {
                        op: rec.op,
                        code: Some(rec.status),
                        stderr: Vec::new(),
                    })
                }
            }
            Mode::Live => {
                self.writes_performed += 1;
                let result = self.exec.run(op, cwd, args);
                let inv = match &result {
                    Ok(out) => GitInvocation {
                        op: op.as_str(),
                        cwd: cwd.as_str().to_string(),
                        argv_digest: digest_of_argv(args),
                        status: 0,
                        stdout: out.stdout.clone(),
                        stderr_digest: digest_of(&out.stderr),
                    },
                    Err(GitError::NonZeroExit { code, stderr, .. }) => GitInvocation {
                        op: op.as_str(),
                        cwd: cwd.as_str().to_string(),
                        argv_digest: digest_of_argv(args),
                        status: code.unwrap_or(-1),
                        stdout: Vec::new(),
                        stderr_digest: digest_of(stderr),
                    },
                    Err(_) => GitInvocation {
                        op: op.as_str(),
                        cwd: cwd.as_str().to_string(),
                        argv_digest: digest_of_argv(args),
                        status: -1,
                        stdout: Vec::new(),
                        stderr_digest: digest_of(b""),
                    },
                };
                self.journal.record(inv);
                result
            }
        }
    }
}

/// A placeholder content digest.
///
/// **Deliberately not SHA-256 yet.** `rk_types::digest` states that hashing arrives with ADR-0097's
/// idempotency key, which is §6.3's work, and ADR-0116 clause 6 forbids a *second* digest algorithm
/// entering the system. Rather than mint one here and have two, this records a length-tagged
/// placeholder that is replaced by `rk_types::digest::sha256` when §6.3 adds it. The shape of the
/// journal — argv as a digest, environment never — is what clause 7 fixes, and that is already
/// true.
fn digest_of(bytes: &[u8]) -> Digest {
    let mut hex = String::with_capacity(64);
    let len = bytes.len() as u64;
    for b in len.to_be_bytes() {
        hex.push_str(&format!("{b:02x}"));
    }
    let sum: u64 = bytes.iter().map(|b| u64::from(*b)).sum();
    for b in sum.to_be_bytes() {
        hex.push_str(&format!("{b:02x}"));
    }
    while hex.len() < 64 {
        hex.push('0');
    }
    Digest::from_hex(&hex)
        .unwrap_or_else(|_| unreachable!("64 lower-hex characters by construction"))
}

fn digest_of_argv(args: &[&str]) -> Digest {
    digest_of(args.join("\u{1f}").as_bytes())
}
