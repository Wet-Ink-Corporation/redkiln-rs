//! The closed-output schema rules, as a library function.
//!
//! ADR-0095 decision 1, stated identically in AC-131, AC-157, design §5.3 and runbook s5 §5.3.
//! This lives in `src/`, not in a test, so CI, the snapshot test and the must-fail fixtures all
//! run ONE implementation — two implementations of one rule is how two implementers ship two lints.

use serde_json::Value;

/// The vendor's Schema complexity limits (platform.claude.com structured-outputs, fetched
/// 2026-09-10): 16 union-typed and 24 optional parameters **per request** across all strict tool
/// schemas plus the JSON output schema.
///
/// Counted here PER EXPORTED SCHEMA, which equals per-request only while one output schema is sent
/// per dispatch with no strict user-defined tools on the adapter's argv. That precondition is a
/// GIVEN in AC-132 and AC-157 and becomes a lint on the command builder at Step 7c — so adding a
/// strict tool later re-opens the budget explicitly rather than silently.
pub const UNION_CAP: usize = 16;
pub const OPTIONAL_CAP: usize = 24;

/// A breach of the closed-output rule, with the JSON pointer at which it was found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Violation {
    /// `oneOf` at any node. The vendor's page lists `anyOf`/`allOf` and does not list `oneOf`.
    NoOneOf { pointer: String },
    /// A union with a union descendant (AC-131).
    NestedUnion { pointer: String },
    /// An object level without `additionalProperties: false` (AC-129).
    OpenObject { pointer: String },
    /// An optional property emitted before a required one (AC-158).
    OptionalBeforeRequired { pointer: String, property: String },
    /// More than `UNION_CAP` union-typed parameters (AC-157).
    UnionCap { count: usize, cap: usize },
    /// More than `OPTIONAL_CAP` optional parameters (AC-132).
    OptionalCap { count: usize, cap: usize },
    /// A `$ref` survived inlining — the caps below it could not be counted, so a breach could hide.
    UnresolvedRef { pointer: String },
}

/// Is this node a union — an `anyOf`, a `oneOf`, or a `type` array?
fn is_union(node: &Value) -> bool {
    node.get("anyOf").is_some()
        || node.get("oneOf").is_some()
        || node.get("type").is_some_and(Value::is_array)
}

/// Every rule, over the EXPORTED shape. Empty vec means conformant.
#[must_use]
pub fn lint(schema: &Value) -> Vec<Violation> {
    let mut out = Vec::new();
    walk(schema, "", false, &mut out);

    let (unions, optionals) = count_parameters(schema);
    if unions > UNION_CAP {
        out.push(Violation::UnionCap {
            count: unions,
            cap: UNION_CAP,
        });
    }
    if optionals > OPTIONAL_CAP {
        out.push(Violation::OptionalCap {
            count: optionals,
            cap: OPTIONAL_CAP,
        });
    }
    out
}

fn walk(node: &Value, pointer: &str, inside_union: bool, out: &mut Vec<Violation>) {
    let Some(map) = node.as_object() else { return };

    if map.contains_key("oneOf") {
        out.push(Violation::NoOneOf {
            pointer: pointer.to_owned(),
        });
    }
    if map.contains_key("$ref") {
        out.push(Violation::UnresolvedRef {
            pointer: pointer.to_owned(),
        });
    }

    let here_is_union = is_union(node);
    if here_is_union && inside_union {
        out.push(Violation::NestedUnion {
            pointer: pointer.to_owned(),
        });
    }

    if map.get("type").and_then(Value::as_str) == Some("object") && map.contains_key("properties") {
        if map.get("additionalProperties") != Some(&Value::Bool(false)) {
            out.push(Violation::OpenObject {
                pointer: pointer.to_owned(),
            });
        }
        check_required_first(map, pointer, out);
    }

    let child_inside_union = inside_union || here_is_union;
    for (key, child) in map {
        let p = format!("{pointer}/{key}");
        match child {
            Value::Object(_) => walk(child, &p, child_inside_union && key != "properties", out),
            Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(item, &format!("{p}/{i}"), child_inside_union, out);
                }
            }
            _ => {}
        }
    }
}

/// AC-158: all required properties precede all optional ones in the EMITTED order. Only
/// observable because `schemars` and `serde_json` carry `preserve_order`.
fn check_required_first(
    map: &serde_json::Map<String, Value>,
    pointer: &str,
    out: &mut Vec<Violation>,
) {
    let required: Vec<&str> = map
        .get("required")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    let Some(props) = map.get("properties").and_then(Value::as_object) else {
        return;
    };

    let mut seen_optional: Option<&str> = None;
    for name in props.keys() {
        if required.contains(&name.as_str()) {
            if let Some(opt) = seen_optional {
                out.push(Violation::OptionalBeforeRequired {
                    pointer: pointer.to_owned(),
                    property: opt.to_owned(),
                });
                return;
            }
        } else {
            seen_optional = Some(name);
        }
    }
}

/// The vendor's two counts, over the whole exported document.
fn count_parameters(schema: &Value) -> (usize, usize) {
    let mut unions = 0;
    let mut optionals = 0;
    count_in(schema, &mut unions, &mut optionals);
    (unions, optionals)
}

fn count_in(node: &Value, unions: &mut usize, optionals: &mut usize) {
    let Some(map) = node.as_object() else { return };

    if let Some(props) = map.get("properties").and_then(Value::as_object) {
        let required: Vec<&str> = map
            .get("required")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        for (name, sub) in props {
            // A nullable `Option<T>` is ONE union-typed parameter under the vendor's count and
            // ZERO under a per-node branch count — the divergence that could breach 16 with every
            // lint green and fail silently at exit 0 (F20). Counting branches is forbidden as this
            // quantity (ADR-0095; AC-157's 2026-09-11 correction).
            if is_union(sub) {
                *unions += 1;
            }
            if !required.contains(&name.as_str()) {
                *optionals += 1;
            }
        }
    }

    for (_k, child) in map {
        match child {
            Value::Object(_) => count_in(child, unions, optionals),
            Value::Array(items) => {
                for item in items {
                    count_in(item, unions, optionals);
                }
            }
            _ => {}
        }
    }
}
