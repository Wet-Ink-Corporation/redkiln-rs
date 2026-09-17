//! The review gate's record types — Step 7b-ii, `s7b-ii` §7b-ii.3 and §7b-ii.6.
//!
//! Two records whose whole point is a member that **cannot be absent**:
//!
//! - [`ReviewOutcome`] carries the **tree digest of the SHA that was reviewed** as a non-optional
//!   [`TreeId`] (AC-293; ADR-0074). *"A record without that digest fails to construct — no
//!   default, no empty string, no `None`"*: the member is not an `Option`, `TreeId` refuses
//!   anything but 40 lowercase hex on deserialisation, and `deny_unknown_fields` closes the shape.
//!   F17's E3 defeat is exactly a review whose basis was not the reviewed tree.
//! - [`Quarantine`] — ADR-0074 clause 6's `Quarantined { reason, by, until }` — carries `until` as a
//!   non-optional [`Timestamp`] (AC-319; ADR-0025): a quarantine with no expiry would be *"block
//!   forever"* wearing the other name, and F18 measured a gate outliving its assumptions by 9.4
//!   days.
//!
//! The expiry behaviour over `until` is `rk-kernel`'s (AC-306), and the outcome's `blocks` /
//! `sink_records` are counts the kernel's confirm half fills; both are recorded here because the
//! design's `rk-types` row owns the typed artifacts and their digests.

use serde::{Deserialize, Serialize};

use crate::text::Text;
use crate::time::Timestamp;

/// A git tree object id — 40 lowercase hex characters, **validated on construction and on
/// deserialisation**: an empty string, `null`, or anything that is not a tree id is refused, so a
/// record carrying one cannot be built around a placeholder (AC-293's *"no default, no empty
/// string"*).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct TreeId(String);

impl<'de> Deserialize<'de> for TreeId {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        Self::from_hex(&s).map_err(|e| serde::de::Error::custom(format!("not a tree id: {e:?}")))
    }
}

impl TreeId {
    /// # Errors
    /// [`TreeIdError`] if `s` is not 40 lowercase hex characters.
    pub fn from_hex(s: &str) -> Result<Self, TreeIdError> {
        if s.len() != 40 {
            return Err(TreeIdError::Length { got: s.len() });
        }
        if let Some(at) = s
            .bytes()
            .position(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        {
            return Err(TreeIdError::NotLowerHex { at });
        }
        Ok(Self(s.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeIdError {
    Length { got: usize },
    NotLowerHex { at: usize },
}

/// How the reviewed tree was derived (ADR-0105 clause 4's two cases).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReviewBasis {
    CheckpointCommit,
    WorkingState,
}

/// A completed review's outcome record (AC-293). `reviewed_tree` is the digest of the tree the
/// reviewer saw — derived by ADR-0105 clause 4's one algorithm — and it is **not optional**.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ReviewOutcome {
    pub reviewed_tree: TreeId,
    pub sha: String,
    pub basis: ReviewBasis,
    pub blocks: u32,
    pub sink_records: u32,
}

/// ADR-0074 clause 6's first-class finding state, `Quarantined { reason, by, until }` (AC-304,
/// AC-306, AC-319). `until` is **not optional**: a quarantine is a bounded override, never a
/// permanent dismissal — that is AC-311's human disposition and a different row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Quarantine {
    pub reason: Text,
    pub by: String,
    pub until: Timestamp,
}
