//! The one fenced-YAML reader the sidecar kinds share.
//!
//! The predecessor writes a companion as a ```` ```yaml ```` block followed by prose. Four readers
//! would each need the same split, so it lives here once — under `legacy/`, which keeps it inside
//! AC-442's allowlisted directory.

use std::path::Path;

use super::LegacyError;

/// A fenced block and the prose after it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fenced {
    /// The YAML inside the fence, unparsed.
    pub block: String,
    /// Everything outside it.
    pub prose: String,
}

/// Split a legacy companion file into its fenced YAML block and the prose beside it.
///
/// # Errors
/// [`LegacyError::Malformed`] if there is no fence or it is unterminated.
pub fn split(path: &Path, src: &str) -> Result<Fenced, LegacyError> {
    let open = src
        .find("```yaml\n")
        .ok_or_else(|| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: "no ```yaml fence".to_owned(),
        })?;
    let after = open + "```yaml\n".len();
    let close = src[after..]
        .find("\n```")
        .ok_or_else(|| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: "the ```yaml fence is unterminated".to_owned(),
        })?;

    let block = src[after..after + close].to_owned();
    let mut prose = String::new();
    prose.push_str(src[..open].trim());
    let tail = src[after + close..].trim_start_matches("\n```");
    if !tail.trim().is_empty() {
        if !prose.is_empty() {
            prose.push('\n');
        }
        prose.push_str(tail.trim());
    }

    Ok(Fenced { block, prose })
}

/// Read a fenced block into a typed value.
///
/// # Errors
/// [`LegacyError::Malformed`] if the block is not valid YAML, or does not match `T`'s shape —
/// every `rk-types` companion carries `#[serde(deny_unknown_fields)]`, so a legacy key with no
/// home is a hard error here rather than a silent drop. That is what forces a caller to strip such
/// keys into a [`super::LossMarker`] first, which is the whole point.
pub fn read<T: serde::de::DeserializeOwned>(path: &Path, block: &str) -> Result<T, LegacyError> {
    yaml_serde::from_str(block).map_err(|e| LegacyError::Malformed {
        path: path.to_path_buf(),
        detail: format!("the fenced block does not read as the expected shape: {e}"),
    })
}
