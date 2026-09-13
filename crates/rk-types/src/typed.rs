//! `SchemaId` and `Typed` — the erased, validated value that crosses the Harness port.
//!
//! **Routed here, not decided here** (runbook `s7a` §7a.5). No record assigns either type a crate.
//! ADR-0099:59-60 makes `Typed` *"an erased, validated value — `serde_json::Value` plus the schema
//! id it validated against — which the kernel deserialises"*: it is produced by the adapter and
//! consumed by the kernel, so it crosses the port boundary in both directions exactly as
//! `StepError` does, and this crate already owns schema derivation and the digests (design:1699).
//! `SchemaId` is the schema-id half of `Typed`, on the same warrant. **A record naming the home of
//! both is OWED**, and it may reverse this routing; Checkpoint 7a deliberately forbids `rk-ports`
//! declaring either, so the routing stays reversible in exactly one place.
//!
//! **No wire format is fixed here.** Neither type derives `Serialize` or `JsonSchema`: nothing
//! serialises a `Typed` before the SQLite adapter at 7c, and a derived wire shape authored now
//! would be a second undecided thing to reverse.

/// The id of the schema a [`Typed`] value validated against.
///
/// A newtype rather than a `String`, in the shape this crate's other newtypes take
/// ([`crate::digest::Digest`], [`crate::text::Text`], [`crate::time::Timestamp`]): a schema id
/// that can be built from any string is a schema id that can be confused with a card digest.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchemaId(String);

impl SchemaId {
    #[must_use]
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An erased, validated value: `serde_json::Value` **plus the schema id it validated against**.
///
/// ADR-0099:59-60's own two halves, and no third. The value is erased deliberately — the kernel
/// deserialises it against the id, and nothing between the adapter and that deserialisation may
/// depend on the concrete artifact type.
#[derive(Debug, Clone, PartialEq)]
pub struct Typed {
    pub schema: SchemaId,
    pub value: serde_json::Value,
}

impl Typed {
    /// Build one from a value and the id it validated against. There is deliberately no
    /// constructor that takes a value alone: an un-attributed erased value is what this type
    /// exists to make unrepresentable.
    #[must_use]
    pub const fn new(schema: SchemaId, value: serde_json::Value) -> Self {
        Self { schema, value }
    }
}
