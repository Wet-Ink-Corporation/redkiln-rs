//! The canonical serialiser.
//!
//! Four criteria compare values *"under the canonical serialiser"* — AC-161's stage-level next
//! action across a deleted database, AC-164's across a truncated tier 2, AC-184's rebuild
//! differential, AC-187's resume equality. All four are equality tests, so the serialiser's only
//! real requirement is that **two values that should compare equal serialise identically on every
//! platform**, and that two that should not, do not.
//!
//! Three rules, and one refusal:
//!
//! * **Object keys are sorted byte-wise.** `serde_json` is pulled with `preserve_order`, so map
//!   order is insertion order and would otherwise leak authoring order into a comparison.
//! * **LF only, no trailing newline.** A CRLF host would otherwise differ from an LF one on every
//!   comparison, which is precisely the cross-platform difference AC-121 exists to catch elsewhere.
//! * **Integers as they are.**
//! * **Floats are refused.** ADR-0097 decision 2 says "fixed float format"; this honours it by
//!   **refusing** rather than choosing a format. No Phase-1 state field is a float, and a refusing
//!   serialiser cannot silently disagree across platforms the way any chosen float format
//!   eventually does. If a float ever needs to be canonical, that is a decision with a record, and
//!   the refusal is what forces it to be taken rather than defaulted.

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonError {
    /// A float reached the canonical serialiser. See the module header: this is a deliberate
    /// refusal, not a gap.
    FloatNotRepresentable { pointer: String },
}

impl std::fmt::Display for CanonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FloatNotRepresentable { pointer } => write!(
                f,
                "a float reached the canonical serialiser at `{pointer}`. ADR-0097 asks for a fixed \
                 float format; rk-store refuses instead, because no Phase-1 state field is a float \
                 and a refusal cannot disagree across platforms. Adding one is a decision with a \
                 record."
            ),
        }
    }
}

/// Serialise `v` canonically.
///
/// # Errors
/// [`CanonError::FloatNotRepresentable`] if any node is a float.
pub fn canonical(v: &Value) -> Result<Vec<u8>, CanonError> {
    let mut out = Vec::new();
    write_value(v, "", &mut out)?;
    Ok(out)
}

/// Canonical bytes, then the one hash. This is what every "under the canonical serialiser"
/// comparison actually compares, and what ADR-0097's component 4 digests each resolved input with.
///
/// # Errors
/// Propagates [`CanonError`].
pub fn canonical_digest(v: &Value) -> Result<rk_types::digest::Digest, CanonError> {
    Ok(rk_types::digest::sha256(&canonical(v)?))
}

fn write_value(v: &Value, pointer: &str, out: &mut Vec<u8>) -> Result<(), CanonError> {
    match v {
        Value::Null => out.extend_from_slice(b"null"),
        Value::Bool(b) => out.extend_from_slice(if *b { b"true" } else { b"false" }),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                out.extend_from_slice(i.to_string().as_bytes());
            } else if let Some(u) = n.as_u64() {
                out.extend_from_slice(u.to_string().as_bytes());
            } else {
                return Err(CanonError::FloatNotRepresentable {
                    pointer: pointer.to_string(),
                });
            }
        }
        Value::String(s) => write_string(s, out),
        Value::Array(items) => {
            out.push(b'[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_value(item, &format!("{pointer}/{i}"), out)?;
            }
            out.push(b']');
        }
        Value::Object(map) => {
            // Byte-wise key sort. `preserve_order` means the map arrives in insertion order, which
            // is authoring order, which is not a property of the value.
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort_unstable_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
            out.push(b'{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                write_string(k, out);
                out.push(b':');
                write_value(&map[*k], &format!("{pointer}/{k}"), out)?;
            }
            out.push(b'}');
        }
    }
    Ok(())
}

/// Minimal JSON string escaping, LF-only. Deliberately not `serde_json::to_string`: that is free to
/// change its escaping between versions, and this output is compared across runs and machines.
fn write_string(s: &str, out: &mut Vec<u8>) {
    out.push(b'"');
    for c in s.chars() {
        match c {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\n' => out.extend_from_slice(b"\\n"),
            '\r' => out.extend_from_slice(b"\\r"),
            '\t' => out.extend_from_slice(b"\\t"),
            c if (c as u32) < 0x20 => {
                out.extend_from_slice(format!("\\u{:04x}", c as u32).as_bytes());
            }
            c => {
                let mut buf = [0u8; 4];
                out.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    out.push(b'"');
}
