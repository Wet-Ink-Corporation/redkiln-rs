//! **AC-133's judgement-enum registry** — built once, here, and consumed by `s8h`.
//!
//! `s8g`'s schema record (R2 = ADR-0192 decision 8, proposed); D21-43's integration call: *"AC-133's
//! judgement-enum registry is built once, by **s8g**. s8h consumes it."* *(Added 2026-09-29, `s8g`
//! build, package S8G-P2-RKTYPES.)*
//!
//! AC-133: *"the enum appears in the judgement-enum registry with an explicit judgement-bearing
//! yes/no AND every enum marked yes has a sibling free-text `rationale` field on the same object."*
//! Three instruments stand on this module (`tests/it/judgement_registry.rs`): completeness over
//! every enum reachable from `registry::EXPORTED`, the rationale-sibling rule with its must-fail
//! fixture, and a check that the registry names no enum that is not exported.
//!
//! **The yes/no values are stated, not signed.** AC-133's limit: *"the lint still cannot tell
//! whether the value of the yes/no is right; that classification is the human gate AC-156"*, which
//! is `s8h`'s (D21-41 g). A type `s8h` adds to `EXPORTED` registers into [`REGISTRY`]; there is no
//! second registry.
//!
//! **Why a NAMED schema.** The exported shape inlines every subschema (`export_settings`), so an
//! enum inside it carries no name to register. [`export_named`] derives the same type with the same
//! transform and `$defs` kept, and the walk reads the names from there.

use schemars::JsonSchema;
use serde_json::Value;
use std::collections::BTreeSet;

/// One registered enum: its schema name, AC-133's explicit yes/no, and the stated reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JudgementEntry {
    /// The enum's schema name, as `$defs` spells it.
    pub name: &'static str,
    /// Does a value of this enum carry a judgement? `true` holds it to the rationale-sibling rule.
    pub judgement_bearing: bool,
    /// Why, in one line. The classification the operator signs is `s8h`'s (AC-156).
    pub reason: &'static str,
}

/// Every enum reachable from `registry::EXPORTED`, each with its yes/no.
pub const REGISTRY: &[JudgementEntry] = &[
    // --- Findings (ADR-0074 clause 1; ADR-0096 decision 1) ---
    JudgementEntry {
        name: "EvidenceLabel",
        judgement_bearing: true,
        reason: "the model's own evidence label on a finding (ADR-0074 clause 1); `Finding.rationale` \
                 is its sibling",
    },
    JudgementEntry {
        name: "ReproKind",
        judgement_bearing: false,
        reason: "which mechanism re-executes the repro (run or mutate); the re-execution, not the \
                 label, decides",
    },
    JudgementEntry {
        name: "Expected",
        judgement_bearing: false,
        reason: "the observable a repro is checked against (an exit code or an assertion locator); \
                 machine-compared",
    },
    // --- AcceptanceCriterion (ADR-0192 decision 3) ---
    JudgementEntry {
        name: "MutationDeclaration",
        judgement_bearing: false,
        reason: "the author's declaration: a patch the harness runs, or not_feasible with its own \
                 in-variant reason (AC-337); the ledger's outcome is the judgement, not this",
    }, // --- s8h §8h.4's lane kinds (Record A = ADR-0197 decision 7, proposed). The yes/no VALUES are
    // drafted; each is the operator's to sign (AC-156). *(dated 2026-10-02, `s8h` build, package
    // P2-rk-types.)*
    JudgementEntry {
        name: "FindingSeverity",
        judgement_bearing: true,
        reason: "the reviewer's call on whether a finding blocks (ADR-0168 decision 1); `Finding.rationale` is its sibling",
    },
    JudgementEntry {
        name: "Confidence",
        judgement_bearing: true,
        reason: "the author's confidence in a conclusion is a judgement about evidence; each object carrying it carries a `rationale`",
    },
    JudgementEntry {
        name: "ReproStatus",
        judgement_bearing: true,
        reason: "whether the defect reproduced is the investigator's reading of an observation; `rationale` is its sibling",
    },
    JudgementEntry {
        name: "HypothesisStatus",
        judgement_bearing: true,
        reason: "whether a hypothesis is confirmed or refuted is a judgement; `HypothesisLog.rationale` is its sibling",
    },
    JudgementEntry {
        name: "DocsQuadrant",
        judgement_bearing: true,
        reason: "the Diataxis quadrant is the author's classification, and the checklist catches a wrong one; `Docs.rationale` is its sibling",
    },
    JudgementEntry {
        name: "ArchitectureLevel",
        judgement_bearing: false,
        reason: "the C4 level the description is drawn at: the notation chosen, not a judgement on evidence",
    },
    JudgementEntry {
        name: "CausalMethod",
        judgement_bearing: false,
        reason: "which causal-analysis method built the chain: the method chosen, not a judgement on evidence",
    },
    JudgementEntry {
        name: "EstimateMethod",
        judgement_bearing: false,
        reason: "which estimation method was used: the method chosen, not a judgement on evidence",
    },
    JudgementEntry {
        name: "RolloutStrategy",
        judgement_bearing: false,
        reason: "the deployment pattern the plan chooses: a plan decided and recorded, not a judgement on evidence",
    },
    JudgementEntry {
        name: "ChangeType",
        judgement_bearing: false,
        reason: "the ITIL change type: a change-management classification recorded, not judged",
    },
    JudgementEntry {
        name: "AdrStatus",
        judgement_bearing: false,
        reason: "a decision record's lifecycle state, recorded by the author, not a judgement on evidence",
    },
];

/// A breach of the rationale-sibling rule: `property` at `pointer` holds a judgement-bearing enum
/// and its object carries no free-text `rationale`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RationaleViolation {
    pub pointer: String,
    pub property: String,
    pub judgement_enum: String,
}

/// Derive `T`'s schema with the export transform and `$defs` KEPT, so every enum it reaches
/// carries its name. Read by the registry's instruments only; never sent anywhere.
#[must_use]
pub fn export_named<T: JsonSchema>() -> Value {
    let schema = schemars::generate::SchemaSettings::draft2020_12()
        .with_transform(super::transform::KindOneOfToAnyOf)
        .into_generator()
        .root_schema_for::<T>();
    serde_json::to_value(schema).expect("a derived schema serialises to JSON")
}

/// Is this schema node an enum: a closed `enum` list, or a union of branches?
fn is_enum(node: &Value) -> bool {
    node.get("enum").is_some() || node.get("oneOf").is_some() || node.get("anyOf").is_some()
}

/// Every enum a NAMED schema reaches: each `$defs` entry that is an enum, and the root when it is.
#[must_use]
pub fn enums_in(named: &Value) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    if is_enum(named)
        && let Some(title) = named.get("title").and_then(Value::as_str)
    {
        out.insert(title.to_owned());
    }
    if let Some(defs) = named.get("$defs").and_then(Value::as_object) {
        for (name, def) in defs {
            if is_enum(def) {
                out.insert(name.clone());
            }
        }
    }
    out
}

/// The `$defs` names a property's schema refers to: directly, through an array's `items`, or
/// through a union's branches (an `Option` renders as a union with `null`).
fn referenced_defs(prop: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![prop];
    while let Some(node) = stack.pop() {
        if let Some(r) = node.get("$ref").and_then(Value::as_str)
            && let Some(name) = r.strip_prefix("#/$defs/")
        {
            out.push(name.to_owned());
        }
        if let Some(items) = node.get("items") {
            stack.push(items);
        }
        for key in ["anyOf", "oneOf", "allOf"] {
            if let Some(branches) = node.get(key).and_then(Value::as_array) {
                stack.extend(branches);
            }
        }
    }
    out
}

/// Does this property, resolved through `$defs`, admit a free-text string?
fn is_free_text(prop: &Value, defs: Option<&serde_json::Map<String, Value>>) -> bool {
    let resolved = prop
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|r| r.strip_prefix("#/$defs/"))
        .and_then(|name| defs.and_then(|d| d.get(name)))
        .unwrap_or(prop);
    resolved.get("type").and_then(Value::as_str) == Some("string")
        && resolved.get("enum").is_none()
        && resolved.get("const").is_none()
}

/// Check every object with `properties` at or below `node`.
fn walk(
    node: &Value,
    pointer: &str,
    defs: Option<&serde_json::Map<String, Value>>,
    bearing: &BTreeSet<&str>,
    out: &mut Vec<RationaleViolation>,
) {
    let Some(map) = node.as_object() else { return };
    if let Some(props) = map.get("properties").and_then(Value::as_object) {
        let has_rationale = props
            .get("rationale")
            .is_some_and(|r| is_free_text(r, defs));
        for (name, prop) in props {
            for def in referenced_defs(prop) {
                if bearing.contains(def.as_str()) && !has_rationale {
                    out.push(RationaleViolation {
                        pointer: pointer.to_owned(),
                        property: name.clone(),
                        judgement_enum: def,
                    });
                }
            }
        }
    }
    for (key, child) in map {
        if key == "$defs" {
            continue;
        }
        let p = format!("{pointer}/{key}");
        match child {
            Value::Object(_) => walk(child, &p, defs, bearing, out),
            Value::Array(items) => {
                for (i, item) in items.iter().enumerate() {
                    walk(item, &format!("{p}/{i}"), defs, bearing, out);
                }
            }
            _ => {}
        }
    }
}

/// **The rationale-sibling rule** over a NAMED schema and a registry: every object holding a
/// member of a judgement-bearing enum carries a free-text `rationale` on the same object. Empty is
/// the pass.
#[must_use]
pub fn rationale_violations_with(
    named: &Value,
    registry: &[JudgementEntry],
) -> Vec<RationaleViolation> {
    let bearing: BTreeSet<&str> = registry
        .iter()
        .filter(|e| e.judgement_bearing)
        .map(|e| e.name)
        .collect();
    let defs = named.get("$defs").and_then(Value::as_object);
    let mut out = Vec::new();
    walk(named, "", defs, &bearing, &mut out);
    if let Some(defs) = defs {
        for (name, def) in defs {
            walk(
                def,
                &format!("/$defs/{name}"),
                Some(defs),
                &bearing,
                &mut out,
            );
        }
    }
    out
}

/// [`rationale_violations_with`] under [`REGISTRY`].
#[must_use]
pub fn rationale_violations(named: &Value) -> Vec<RationaleViolation> {
    rationale_violations_with(named, REGISTRY)
}
