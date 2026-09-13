//! Opaque prose. Markdown is an output, never an input (ADR-0006, AC-120).

use serde::{Deserialize, Serialize};

/// A free-text field. AC-283 names `claim` and `rationale` as the only free-text fields on a
/// finding, and neither is an input to the classifier or to `policy::classify`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(transparent)]
pub struct Text(String);

impl Text {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
