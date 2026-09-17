//! **AC-305 — a finding's stable identity.**
//!
//! > the tuple (AC id, `repro.kind`, repro command digest, and for `kind = mutate` the digest of
//! > `mutation.patch` plus `mutation.suite` — ADR-0096's typed fields, PIA-007) … identity stable
//! > under rationale rewording and file-order change, unstable under a changed repro command or a
//! > changed mutation patch.
//!
//! **What is in the tuple and what is not, and why that is the criterion's own repair.** The
//! original tuple carried a "normalised claim"; deciding that two differently worded claims are the
//! same claim is a judgement, so the register cut it, and the cost — *"a re-review restating the same
//! defect with a different command will re-raise"* — is acknowledged there. So `claim`, `rationale`
//! and `files` are NOT components: rewording the rationale or reordering the files leaves the
//! identity where it was. `ac`, `kind`, the command and (for a mutant) the patch and suite ARE, and
//! any of them moving moves it.
//!
//! **Fixed length, four components, length-prefixed** (`rk_types::digest::sha256_lp`, the one hash;
//! ADR-0116 clause 6: no second digest algorithm). The fourth component is the mutation's own digest
//! for `kind = mutate` and the EMPTY component for `kind = run` — so a run and a mutant with the
//! same command differ in two places (the kind token and the fourth slot) and the tuple never
//! changes shape. The command is digested over its bytes, not canonicalised as JSON: it is a shell
//! string, and the canonical serialiser exists for values, not strings.

use rk_types::artifact::findings::Finding;
use rk_types::digest::{Digest, sha256, sha256_lp};
use rk_types::repro::ReproKind;

/// The four components of a finding's identity, in the order they are hashed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindingIdentityComponents {
    pub ac: String,
    pub kind: ReproKind,
    pub command_digest: Digest,
    /// `sha256_lp([patch, suite])` for `kind = mutate`; `None` for `kind = run`, hashed as the
    /// empty component.
    pub mutation_digest: Option<Digest>,
}

impl FindingIdentityComponents {
    /// The number of components hashed. Asserted by a test against the register's tuple.
    pub const COMPONENT_COUNT: usize = 4;

    /// Read the components off a typed finding. `claim`, `rationale` and `files` are not read.
    #[must_use]
    pub fn of(finding: &Finding) -> Self {
        let mutation_digest = match (finding.repro.kind, &finding.repro.mutation) {
            (ReproKind::Mutate, Some(m)) => {
                Some(sha256_lp(&[m.patch.as_bytes(), m.suite.as_bytes()]))
            }
            // AC-281 rejects `kind = mutate` without a mutation at ingest, so this arm is a
            // finding that never reached us; hashing it as a run keeps the function total.
            (ReproKind::Mutate, None) | (ReproKind::Run, _) => None,
        };
        Self {
            ac: finding.ac.clone(),
            kind: finding.repro.kind,
            command_digest: sha256(finding.repro.command.as_bytes()),
            mutation_digest,
        }
    }

    /// The identity: `sha256_lp` over the four components in declared order.
    #[must_use]
    pub fn digest(&self) -> Digest {
        let kind = match self.kind {
            ReproKind::Run => "run",
            ReproKind::Mutate => "mutate",
        };
        let mutation: &[u8] = self
            .mutation_digest
            .as_ref()
            .map_or(&[][..], |d| d.as_str().as_bytes());
        sha256_lp(&[
            self.ac.as_bytes(),
            kind.as_bytes(),
            self.command_digest.as_str().as_bytes(),
            mutation,
        ])
    }
}

/// The stable identity of a finding (AC-305) — `FindingIdentityComponents::of(finding).digest()`.
#[must_use]
pub fn finding_identity(finding: &Finding) -> Digest {
    FindingIdentityComponents::of(finding).digest()
}
