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
    let rest = src
        .strip_prefix("---\n")
        .ok_or_else(|| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: "no opening --- frontmatter fence".to_owned(),
        })?;
    let (block, body) = rest
        .split_once("\n---\n")
        .ok_or_else(|| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: "the --- frontmatter fence is unterminated".to_owned(),
        })?;

    let keys: BTreeMap<String, yaml_serde::Value> =
        yaml_serde::from_str(block).map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("the frontmatter block is not valid YAML: {e}"),
        })?;

    let keys: BTreeMap<String, String> = keys
        .into_iter()
        .map(|(k, v)| {
            let rendered = match v {
                yaml_serde::Value::String(s) => s,
                other => yaml_serde::to_string(&other)
                    .unwrap_or_default()
                    .trim_end()
                    .to_owned(),
            };
            (k, rendered)
        })
        .collect();

    for key in ITEM_KEYS {
        if !keys.contains_key(key) {
            return Err(LegacyError::MissingKey {
                path: path.to_path_buf(),
                key: (*key).to_owned(),
            });
        }
    }

    Ok(Frontmatter {
        keys,
        body: body.to_owned(),
    })
}

/// Find the body sections that have no home in the Phase-1 schema — the diff-prose section
/// AC-475's carry names among them.
///
/// A `##` section carrying a fenced `diff` block is narrative commentary on a change. No Phase-1
/// schema declares a field for it, so it is carried verbatim beside a marker rather than dropped.
#[must_use]
pub fn opaque_sections(body: &str) -> Vec<LossMarker> {
    let mut out = Vec::new();
    // Split on `## ` headings, keeping each heading with the prose beneath it.
    let mut current: Option<(String, String)> = None;
    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            if let Some((what, text)) = current.take() {
                push_if_diff(&mut out, &what, &text);
            }
            current = Some((heading.trim().to_owned(), String::new()));
        } else if let Some((_, text)) = current.as_mut() {
            text.push_str(line);
            text.push('\n');
        }
    }
    if let Some((what, text)) = current.take() {
        push_if_diff(&mut out, &what, &text);
    }
    out
}

fn push_if_diff(out: &mut Vec<LossMarker>, what: &str, text: &str) {
    if !text.contains("```diff") {
        return;
    }
    out.push(LossMarker {
        what: format!("section `## {what}`"),
        why: "a diff-prose section: narrative commentary on a change, for which no Phase-1 \
              artifact schema declares a field (AC-475's carry)"
            .to_owned(),
        opaque: text.trim_end().to_owned(),
    });
}
