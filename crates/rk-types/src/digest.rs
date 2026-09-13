//! Content digests as a type, not a `String`.
//!
//! A digest is carried as a validated lowercase-hex SHA-256.
//!
//! **Computation arrived at Step 6.3, as this header said it would**, with ADR-0097's memo key —
//! the first thing in the system that needs to hash rather than merely carry a hash. Until then the
//! type could only be *constructed from data it was given*, so it could not silently mint an id
//! (AC-002's concern, one level below the lint).
//!
//! **There is exactly one hash, and it is here.** ADR-0116 clause 6: *"The digest is the artifact's
//! content digest, computed by the same function the typed-artifact digests use (`rk-types`) … No
//! second digest algorithm enters the system."* [`sha256_lp`] carries the `u32-length || bytes`
//! framing ADR-0097 mandates, so the framing is part of the one function rather than something each
//! caller reimplements slightly differently.

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

/// The one hash: SHA-256 over `bytes`.
///
/// Every content digest in the system comes from here (ADR-0116 clause 6).
#[must_use]
pub fn sha256(bytes: &[u8]) -> Digest {
    use sha2::Digest as _;
    let mut h = sha2::Sha256::new();
    h.update(bytes);
    let out = h.finalize();
    let mut hex = String::with_capacity(64);
    for b in out {
        use std::fmt::Write as _;
        let _ = write!(hex, "{b:02x}");
    }
    Digest(hex)
}

/// SHA-256 over an ordered tuple of components, each **length-prefixed**.
///
/// ADR-0097 decision 2: each component is encoded as `u32-length || bytes`, in declaration order.
/// The framing is not decoration. Without it, `("ab", "c")` and `("a", "bc")` hash identically, so
/// a memo key whose components could absorb each other's boundaries would serve one activity's
/// recorded result to a different activity whose components happened to compensate — a step that
/// should have re-run, resolved from the memo instead.
///
/// The length is written big-endian so the encoding does not vary by host.
#[must_use]
pub fn sha256_lp(components: &[&[u8]]) -> Digest {
    let mut buf: Vec<u8> = Vec::new();
    for c in components {
        let len = u32::try_from(c.len()).unwrap_or(u32::MAX);
        buf.extend_from_slice(&len.to_be_bytes());
        buf.extend_from_slice(c);
    }
    sha256(&buf)
}
