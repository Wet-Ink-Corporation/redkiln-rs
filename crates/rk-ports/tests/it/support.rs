//! Shared fixtures for the port suite.

use rk_types::activity::ActivitySpec;
use rk_types::digest::Digest;
use rk_types::typed::{SchemaId, Typed};

/// A dispatchable spec. The digest is built from data, as `Digest`'s only constructor requires --
/// there is deliberately no way to mint one.
#[must_use]
pub fn spec() -> ActivitySpec {
    ActivitySpec {
        card_digest: Digest::from_hex(&"a".repeat(64)).expect("64 lowercase hex"),
        tools: vec!["StructuredOutput".to_owned()],
        schema: SchemaId::new("findings"),
        timeout_secs: 5,
        inputs: Vec::new(),
        activity_id_at_version: None,
    }
}

/// A typed result carrying the schema id it validated against.
#[must_use]
pub fn findings(value: serde_json::Value) -> Typed {
    Typed::new(SchemaId::new("findings"), value)
}
