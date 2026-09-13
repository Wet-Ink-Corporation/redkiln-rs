//! Content digests as a type, not a `String`.
//!
//! A digest is carried as a validated lowercase-hex SHA-256. This module deliberately does **not**
//! compute one: hashing arrives with ADR-0097's idempotency key at Step 6, and a type that can only
//! be *constructed from data it was given* cannot silently mint an id (AC-002's concern, one level
//! below the lint).

use serde::{Deserialize, Serialize};

/// A lowercase-hex SHA-256 digest.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, schemars::JsonSchema,
)]
#[serde(transparent)]
pub struct Digest(String);

/// Why a candidate hex string was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DigestError {
    /// Not 64 characters.
    Length { got: usize },
    /// A character outside `[0-9a-f]`.
    NotLowerHex { at: usize },
}

impl Digest {
    /// The only constructor: 64 lowercase hex characters, or a refusal naming which rule failed.
    pub fn from_hex(s: &str) -> Result<Self, DigestError> {
        if s.len() != 64 {
            return Err(DigestError::Length { got: s.len() });
        }
        if let Some(at) = s
            .bytes()
            .position(|b| !b.is_ascii_digit() && !(b'a'..=b'f').contains(&b))
        {
            return Err(DigestError::NotLowerHex { at });
        }
        Ok(Self(s.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
