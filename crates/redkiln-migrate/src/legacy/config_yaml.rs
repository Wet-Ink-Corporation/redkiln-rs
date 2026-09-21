//! The predecessor's `.redkiln/config.yaml` and `.redkiln/processes/*.yaml` (`s7d` §7d.13).
//!
//! These two yield the report's `config` and `process` row kinds (§7d.11 step (4)). The process
//! path is the second literal `ci.yml`'s AC-442 lint allowlists this module for: a
//! `processes/*.yaml` path outside here fails the build.

use std::path::Path;

use super::LegacyError;

/// The predecessor's process-definition directory, relative to `.redkiln/`.
pub const PROCESSES_DIR: &str = "processes";

/// A legacy `.redkiln/config.yaml`, as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyConfig {
    /// Every top-level key, unparsed — v0 carries the configuration forward, it does not interpret
    /// it. `rk-cli`'s `ConfigFile` is the Phase-1 shape and is not this crate's to write.
    pub keys: serde_json::Value,
}

/// A legacy process definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyProcess {
    /// The process name, taken from the file stem.
    pub name: String,
    /// The definition, unparsed.
    pub definition: serde_json::Value,
}

/// Read `.redkiln/config.yaml`.
///
/// # Errors
/// [`LegacyError::Malformed`] if the document is not valid YAML.
pub fn parse_config(path: &Path, src: &str) -> Result<LegacyConfig, LegacyError> {
    let keys: serde_json::Value =
        yaml_serde::from_str(src).map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("config.yaml is not valid YAML: {e}"),
        })?;
    Ok(LegacyConfig { keys })
}

/// Read one `.redkiln/processes/<name>.yaml`.
///
/// # Errors
/// [`LegacyError::Malformed`] if the document is not valid YAML.
pub fn parse_process(path: &Path, src: &str) -> Result<LegacyProcess, LegacyError> {
    let definition: serde_json::Value =
        yaml_serde::from_str(src).map_err(|e| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: format!("the process definition is not valid YAML: {e}"),
        })?;
    let name = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .ok_or_else(|| LegacyError::Malformed {
            path: path.to_path_buf(),
            detail: "the process file has no stem to take its name from".to_owned(),
        })?;
    Ok(LegacyProcess { name, definition })
}
