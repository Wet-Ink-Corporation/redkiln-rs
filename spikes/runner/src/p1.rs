//! P1 arm L — triage classification stability (METHOD-P §1.2). Model cost:
//! ~200 dispatches at haiku/sonnet.
//!
//! The question is whether the LLM-emitted half of the context vector is stable
//! enough for a deterministic selection engine to stand on. F27 already showed
//! that a model's *threshold judgement* flips on identical input; the context
//! vector's four ordinal axes are exactly such judgements, so this measures the
//! flip rate directly rather than assuming it away.
//!
//! Four ordinal axes, not the five METHOD-P §1.2 lists: `dynamism` is not
//! registered in `spikes/p9/facets.toml`, so it can feed no `require` row and
//! no `R(stage, ctx)` — the gap `lattice.py` records as I8 and `F35-DRAFT.md`
//! already discloses. Departure logged as DECISIONS-2026-09-09 #2.
//!
//! **F20 applies here with full force.** The card's `tools` list is an
//! allowlist; with `StructuredOutput` outside it `--json-schema` goes silently
//! inert, the model answers in prose, and every record is void for want of a
//! digest. The classifier needs no other tool — it classifies the item text it
//! is given, and giving it `Read` would let it wander into the corpus and
//! classify something other than the item.

use anyhow::{Context, Result};

/// The card's tool allowlist. `StructuredOutput` is the digest channel (F20);
/// nothing else is granted, because the classification is a judgement about the
/// text in the prompt and any file access would change what is being measured.
pub const CARD_TOOLS: [&str; 1] = ["StructuredOutput"];

/// One item to classify, from `spikes/plans/p1-items.txt`.
#[derive(Debug, Clone)]
pub struct Item {
    pub kind: String,
    pub slug: String,
    pub card: String,
    pub hash: String,
}

/// Read the pre-registered item list. Written before any dispatch (METHOD-P
/// §1.2); this only reads it, and never selects.
pub fn read_items(path: &std::path::Path) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading --items {}", path.display()))?;
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        anyhow::ensure!(f.len() >= 4, "malformed items line: {line}");
        out.push(Item {
            kind: f[0].to_string(),
            slug: f[1].to_string(),
            card: f[2].to_string(),
            hash: f[3].to_string(),
        });
    }
    anyhow::ensure!(!out.is_empty(), "no items in {}", path.display());
    Ok(out)
}

/// The item's card text, read from the frozen corpus at the pin.
///
/// `git show` only — the corpus working tree is never touched (AFK brief rule
/// 6), and the pin is what makes the record reproducible (METHOD-P §0 rule 2).
pub fn card_text(redkiln: &std::path::Path, sha: &str, card_path: &str) -> Result<String> {
    let out = std::process::Command::new("git")
        .args(["show", &format!("{sha}:{card_path}")])
        .current_dir(redkiln)
        .output()
        .with_context(|| format!("git show {sha}:{card_path}"))?;
    anyhow::ensure!(
        out.status.success(),
        "git show {sha}:{card_path} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

/// Strip the item's own `type:` line from the frontmatter.
///
/// The record asks the model for `kind`, and the card states it outright. Left
/// in, `kind` measures transcription rather than judgement, and — worse — the
/// declared kind anchors every ordinal that follows it. The rest of the
/// frontmatter stays: it is what a real triage gate would see.
pub fn redact_kind(card: &str) -> String {
    card.lines()
        .map(|l| {
            let t = l.trim_start();
            if t.starts_with("type:") || t.starts_with("process:") {
                "# (kind withheld for P1 arm L)"
            } else {
                l
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The prompt. The card body carries the anchors; this carries the item.
pub fn build_prompt(slug: &str, card: &str) -> String {
    format!(
        "Classify this backlog item.\n\n\
         Return the closed classification record defined by the output schema, and nothing else.\n\
         Judge the item as written, from the text below alone. Every non-`unknown` ordinal and every\n\
         `true` boolean needs a verbatim quote from this text in `evidence[]`. If you cannot quote it,\n\
         the ordinal is `unknown` and the boolean is `false`.\n\n\
         `unknown` is not a middle value and not a hedge — `med` is the middle. `unknown` means the item\n\
         does not contain what you would need to place it. It is not free: it routes the item to a human,\n\
         and if it reaches selection anyway it is treated as HIGH by position.\n\n\
         ---- ITEM: {slug} ----\n\n{card}\n\n---- END ITEM ----\n"
    )
}

/// The classification record schema, mirroring
/// `spikes/schemas/p1-classify.schema.json`.
///
/// Built here rather than read from disk so the binary cannot drift from a file
/// the operator edits mid-run; the two are asserted equal before the set runs.
pub fn classify_schema() -> serde_json::Value {
    let ord = |desc: &str| {
        serde_json::json!({
            "type": "string",
            "description": desc,
            "enum": ["low", "med", "high", "unknown"]
        })
    };
    serde_json::json!({
        "type": "object",
        "additionalProperties": false,
        "required": [
            "kind", "criticality", "novelty", "size", "blast_radius",
            "market_facing", "external_dep", "has_repro", "evidence", "notes"
        ],
        "properties": {
            "kind": {
                "type": "string",
                "description": "The item's kind. Choose the kind the item IS, not the kind of work it will cause.",
                "enum": ["initiative", "project", "story", "bug", "incident", "spike",
                         "chore", "tech-debt", "security-vuln", "docs"]
            },
            "criticality": ord("What is at stake if this is done wrong, or not done. See the anchors in the card."),
            "novelty": ord("How much of this has no worked precedent here. See the anchors in the card."),
            "size": ord("How much changed surface the work is. See the anchors in the card."),
            "blast_radius": ord("How far a defect introduced here would reach. See the anchors in the card."),
            "market_facing": {
                "type": "boolean",
                "description": "True when someone outside the engineering team would notice this from the outside."
            },
            "external_dep": {
                "type": "boolean",
                "description": "True when finishing this needs something the team does not control."
            },
            "has_repro": {
                "type": "boolean",
                "description": "True when the item contains steps, a command, a failing test, a trace or an input that would let a reader observe the problem. False for non-defects."
            },
            "evidence": {
                "type": "array",
                "description": "One entry per non-`unknown` ordinal and per `true` boolean. Quote the item verbatim; never paraphrase.",
                "items": {
                    "type": "object",
                    "additionalProperties": false,
                    "required": ["field", "quote"],
                    "properties": {
                        "field": {
                            "type": "string",
                            "enum": ["kind", "criticality", "novelty", "size", "blast_radius",
                                     "market_facing", "external_dep", "has_repro"]
                        },
                        "quote": {
                            "type": "string",
                            "description": "A verbatim span from the item that supports the value."
                        }
                    }
                }
            },
            "notes": {
                "type": "string",
                "description": "Free text. UNTRUSTED and never a predicate input — nothing here reaches the selection engine."
            }
        }
    })
}

/// The four registered ordinal axes (`facets.toml`, `source = "llm"`).
pub const ORDINALS: [&str; 4] = ["criticality", "novelty", "size", "blast_radius"];
/// The three registered LLM booleans.
pub const BOOLEANS: [&str; 3] = ["market_facing", "external_dep", "has_repro"];

/// Is the digest a well-formed classification record?
///
/// A schema-valid digest is the only thing that counts as the model having
/// spoken about the item; anything else is `void: schema`, never a miss
/// (METHOD-S §0 rule 2, F20).
pub fn valid_record(d: &serde_json::Value) -> bool {
    let Some(o) = d.as_object() else { return false };
    let ord_ok = ORDINALS.iter().all(|k| {
        o.get(*k)
            .and_then(|v| v.as_str())
            .is_some_and(|v| matches!(v, "low" | "med" | "high" | "unknown"))
    });
    let bool_ok = BOOLEANS.iter().all(|k| o.get(*k).is_some_and(|v| v.is_boolean()));
    let kind_ok = o.get("kind").and_then(|v| v.as_str()).is_some_and(|v| {
        matches!(
            v,
            "initiative" | "project" | "story" | "bug" | "incident" | "spike"
                | "chore" | "tech-debt" | "security-vuln" | "docs"
        )
    });
    ord_ok && bool_ok && kind_ok
}

/// Per-record view used by the offline analysis, so the audit never has to
/// re-derive the vector from the raw digest.
pub fn vector(d: &serde_json::Value) -> serde_json::Value {
    let g = |k: &str| d.get(k).cloned().unwrap_or(serde_json::Value::Null);
    serde_json::json!({
        "kind": g("kind"),
        "criticality": g("criticality"),
        "novelty": g("novelty"),
        "size": g("size"),
        "blast_radius": g("blast_radius"),
        "market_facing": g("market_facing"),
        "external_dep": g("external_dep"),
        "has_repro": g("has_repro"),
        "evidence_count": d.get("evidence").and_then(|e| e.as_array()).map_or(0, |a| a.len()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_removes_the_declared_kind() {
        let card = "---\nid: RK-S0180\ntype: story\nprocess: story\nstatus: done\n---\n# Title";
        let out = redact_kind(card);
        assert!(!out.contains("type: story"));
        assert!(!out.contains("process: story"));
        assert!(out.contains("id: RK-S0180"), "unrelated frontmatter survives");
        assert!(out.contains("# Title"));
    }

    #[test]
    fn valid_record_requires_every_registered_facet() {
        let good = serde_json::json!({
            "kind": "story", "criticality": "low", "novelty": "med", "size": "high",
            "blast_radius": "unknown", "market_facing": true, "external_dep": false,
            "has_repro": false, "evidence": [], "notes": ""
        });
        assert!(valid_record(&good));

        // A prose answer, which is what F20's failure mode actually looks like.
        assert!(!valid_record(&serde_json::json!("this item looks like a story")));

        // `dynamism` present but a registered axis missing is still invalid.
        let mut bad = good.clone();
        bad.as_object_mut().unwrap().remove("blast_radius");
        assert!(!valid_record(&bad));

        // An off-enum ordinal is not a value the engine can band.
        let mut bad2 = good.clone();
        bad2["criticality"] = serde_json::json!("critical");
        assert!(!valid_record(&bad2));
    }

    #[test]
    fn schema_matches_the_committed_file() {
        // The card is committed before the first dispatch and not edited during
        // P1 (METHOD-P §1.2); the binary must not drift from it.
        let path = std::path::Path::new("../schemas/p1-classify.schema.json");
        let disk: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(path).expect("schema file")).unwrap();
        let built = classify_schema();
        for k in ["required", "additionalProperties"] {
            assert_eq!(disk[k], built[k], "{k} differs from the committed schema");
        }
        let dp = disk["properties"].as_object().unwrap();
        let bp = built["properties"].as_object().unwrap();
        assert_eq!(
            dp.keys().collect::<Vec<_>>(),
            bp.keys().collect::<Vec<_>>(),
            "property set differs from the committed schema"
        );
        for k in ORDINALS.iter().chain(["kind"].iter()) {
            assert_eq!(dp[*k]["enum"], bp[*k]["enum"], "{k} enum differs");
        }
    }
}
