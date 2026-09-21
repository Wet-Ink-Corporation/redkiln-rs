//! The predecessor's YAML frontmatter, as read (`s7d` §7d.13).
//!
//! Its key set is `id`, `uid`, `type`, `slug`, `title`, `status`, `process`, `stage` — plus the two
//! signature keys this module is the one allowlisted site for. They are **read and dropped**: see
//! [`crate::lift`] for why that is the clean break (AC-471, AC-474) and not a migration loss.

use std::collections::BTreeMap;
use std::path::Path;

use super::{LegacyError, LossMarker};

/// The legacy template-signature key. Read here, written nowhere (AC-442, AC-471).
pub const TEMPLATE_SIG_KEY: &str = "template_sig";

/// The legacy rendered-signature key. Read here, written nowhere (AC-442, AC-471).
pub const RENDERED_SIG_KEY: &str = "rendered_sig";

/// The predecessor's item-record frontmatter key set, in its own order.
pub const ITEM_KEYS: [&str; 8] = [
    "id", "uid", "type", "slug", "title", "status", "process", "stage",
];

/// One legacy item record: its frontmatter keys and the markdown body beneath them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frontmatter {
    /// Every key present in the `---` fence, including the two signature keys when present.
    pub keys: BTreeMap<String, String>,
    /// Everything after the closing fence.
    pub body: String,
}

impl Frontmatter {
    /// Whether either signature key was present — read, and deliberately not carried.
    #[must_use]
    pub fn carried_a_signature(&self) -> bool {
        self.keys.contains_key(TEMPLATE_SIG_KEY) || self.keys.contains_key(RENDERED_SIG_KEY)
    }
}

/// Parse a legacy item record.
///
/// # Errors
/// [`LegacyError::Malformed`] if the `---` fence is absent or unterminated or the block is not
/// valid YAML; [`LegacyError::MissingKey`] if a key of [`ITEM_KEYS`] is absent.
pub fn parse(path: &Path, src: &str) -> Result<Frontmatter, LegacyError> {
    let _ = (path, src);
    todo!("cycle 2: split the --- fence, read the block through yaml_serde, keep the body")
}

/// Find the body sections that have no home in the Phase-1 schema — the diff-prose section
/// AC-475's carry names among them.
#[must_use]
pub fn opaque_sections(body: &str) -> Vec<LossMarker> {
    let _ = body;
    todo!("cycle 2: detect the diff-prose section and carry it opaque with a marker")
}
