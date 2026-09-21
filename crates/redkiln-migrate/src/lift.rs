//! The lift: legacy files in, canonical artifacts out (`s7d` §7d.11 step (4)).
//!
//! Every legacy file under the target's `.bklg/` and `.redkiln/` is read through [`crate::legacy`]
//! — the only `yaml_serde` users — and lands as one of:
//!
//! * a **typed** value, for the six [`rk_types::companion::CompanionKind`] variants and the item
//!   records, validated in process against the schema
//!   [`rk_types::companion::write_companion`] returns (that function is a schema lookup, not a
//!   writer) and serialised through [`rk_store::canon::canonical`];
//! * **opaque prose**, for the eight [`rk_types::companion::PHASE_2_KINDS`] (AC-143; the treatment
//!   is ADR-0075 clause 6's), and for a free-form gate result or a diff-prose section, each with a
//!   migration-loss marker (AC-477's carry, AC-475's tripwire).
//!
//! # The two signature literals are dropped, and that is not a loss
//!
//! `template_sig` and `rendered_sig` are read here — under [`crate::legacy`], the one allowlisted
//! site — and **deliberately not carried**. AC-471 says no schema declares a template- or
//! rendered-signature field and AC-474 calls the break clean, so their absence from the output is
//! the point of the migration, not something the report should count as lost. An item record whose
//! only difference from its legacy form is those two keys lifts **clean**, which is what
//! Checkpoint 7d's exact `spec` counts require: one clean `spec.md` and one diff-prose one.
//!
//! # `DirOrder` is a seam, not a toggle
//!
//! AC-449 sweeps directory-read order as a nuisance parameter. A spawned process cannot be made to
//! read in a different order without a flag or an environment variable, and AC-474 forbids both —
//! so the read-order arm drives [`run`] **in process** through [`DirOrder`], once with
//! [`Natural`] and once with a reversing implementation the test supplies, and compares tree and
//! report digests.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::report::Report;

/// The directory-read ordering seam AC-449's read-order arm drives.
pub trait DirOrder {
    /// Order one directory's entries in place.
    fn order(&mut self, entries: &mut Vec<PathBuf>);
}

/// The production ordering: sorted, so a run is reproducible without reference to the filesystem's
/// own listing order (AC-002).
#[derive(Debug, Default, Clone, Copy)]
pub struct Natural;

impl DirOrder for Natural {
    fn order(&mut self, entries: &mut Vec<PathBuf>) {
        entries.sort();
    }
}

/// What one lift produced: the artifacts to write, and the report that describes them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lifted {
    /// Artifact bytes by path **relative to the target root**, already canonical
    /// ([`rk_store::canon::canonical`]) — so a write is a write and never a second serialisation.
    pub artifacts: BTreeMap<PathBuf, Vec<u8>>,
    /// The differential report (AC-448).
    pub report: Report,
}

/// Why a lift could not run to completion.
///
/// A per-file failure is **not** one of these — it is a `failed` count in the report. These are the
/// conditions under which no report can be produced at all.
#[derive(Debug)]
pub enum LiftError {
    /// The target carries neither `.bklg/` nor `.redkiln/` — there is nothing to migrate, and a
    /// report over nothing would read as a successful migration of an empty corpus.
    NothingToMigrate { target: PathBuf },
    /// A directory under the target could not be read.
    Unreadable { path: PathBuf, detail: String },
    /// The report could not be built or did not validate.
    Report(crate::report::ReportError),
}

/// Lift the target's legacy tree.
///
/// Reads only; writes nothing. The write phase is [`crate::run_contract`]'s, which is what makes
/// `--dry-run` a matter of stopping rather than of suppressing.
///
/// # Errors
/// [`LiftError`], as documented on its variants.
pub fn run(target: &Path, order: &mut dyn DirOrder) -> Result<Lifted, LiftError> {
    let _ = (target, order);
    todo!("cycle 2: walk .bklg/ and .redkiln/ through `order`, lift each file, build the report")
}
