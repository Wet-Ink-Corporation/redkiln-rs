//! `Findings` and the one door a review payload comes through (AC-281).

use crate::repro::{Repro, ReproKind};
use crate::text::Text;
use serde::{Deserialize, Serialize};

/// Why a payload was refused at ingest. A malformed item is **not** a finding: it is not
/// `Unverifiable`, and it is never routed to advisory (ADR-0096 decision 1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestError {
    /// The payload did not deserialise into the typed shape at all.
    Malformed { detail: String },
    /// `command` does not begin with an executable token — a prose-only "command".
    CommandNotExecutable { item: String, command: String },
    /// `kind = mutate` with no `mutation` to apply.
    MutateWithoutMutation { item: String },
}

/// The model's own evidence label — ADR-0074 clause 1's `evidence (advisory label)`. **Retained for
/// drift monitoring only** (clause 2; AC-285): the class the policy table acts on is computed in
/// Rust from the repro record, and this label may never be routed into it. Its spelling is the
/// class vocabulary's (`reproduced` / `mutation-survived` / `narrowed-pin` / `advisory`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceLabel {
    Reproduced,
    MutationSurvived,
    NarrowedPin,
    Advisory,
}

impl EvidenceLabel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reproduced => "reproduced",
            Self::MutationSurvived => "mutation-survived",
            Self::NarrowedPin => "narrowed-pin",
            Self::Advisory => "advisory",
        }
    }
}

/// One finding — ADR-0074 clause 1's `{ ac, claim, evidence (advisory label), rationale, files[],
/// repro }` with ADR-0096 decision 1's typed `repro`. `claim` and `rationale` are the **only**
/// free-text members (AC-283); `evidence` is a closed label and `files` are paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub ac: String,
    pub claim: Text,
    /// The model's label. Read by the drift instrument and the persisted record; **never by the
    /// policy module** (AC-285's architecture lint).
    pub evidence: EvidenceLabel,
    pub rationale: Text,
    pub repro: Repro,
    /// Paths the finding names. Optional, so declared LAST: AC-158's schema lint refuses an
    /// optional member ahead of a required one (it did, on the first cut, 2026-09-17).
    #[serde(default)]
    pub files: Vec<String>,
}

/// **AC-280's decision-field denylist** — ADR-0074's forbids: *"any `pass`, `verdict` or `approve`
/// field on the review card's output schema"*, enumerated so the check is a name check a test can
/// fail deterministically, never a semantic judgement (the criterion's own repair note).
pub const DECISION_FIELD_DENYLIST: &[&str] = &[
    "pass", "verdict", "approve", "approved", "approval", "ok", "status", "decision",
];

/// **AC-280's schema-shape check.** Every property name in [`DECISION_FIELD_DENYLIST`] that the
/// schema declares on the top-level `Findings` object or on its **item** object (the `Finding`,
/// which the closed-output transform inlines at `/properties/items/items`; a `$defs` entry is
/// walked too, for a derivation that does not inline). Empty is the pass; a non-empty result is a
/// configuration error to be raised *before any dispatch*. A **name** check on those two objects
/// only, deliberately — deciding that some other boolean deeper in the shape "means" approval is
/// a judgement no test can make (the criterion's own repair note).
#[must_use]
pub fn decision_fields_declared(schema: &serde_json::Value) -> Vec<String> {
    fn names(props: Option<&serde_json::Value>, out: &mut Vec<String>) {
        if let Some(map) = props.and_then(serde_json::Value::as_object) {
            for name in map.keys() {
                if DECISION_FIELD_DENYLIST.contains(&name.as_str()) && !out.contains(name) {
                    out.push(name.clone());
                }
            }
        }
    }
    let mut out = Vec::new();
    names(schema.get("properties"), &mut out);
    names(
        schema.pointer("/properties/items/items/properties"),
        &mut out,
    );
    if let Some(defs) = schema.get("$defs").and_then(serde_json::Value::as_object) {
        for def in defs.values() {
            names(def.get("properties"), &mut out);
        }
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Findings {
    pub items: Vec<Finding>,
}

/// English imperative openers that are not also common program names.
///
/// SCOPE, stated because a guard scoped wrong cuts both ways (golden constraint 9). Deliberately
/// ABSENT from this list: `make`, `npm`, `go`, `cargo`, `pytest`, `bash`, `sh`, `node`, `python`,
/// `dotnet` and every other real tool — a command may legitimately begin with any of them. The
/// list holds only words no tool in the corpus is named after.
const PROSE_OPENERS: &[&str] = &[
    "check", "confirm", "do", "ensure", "execute", "inspect", "just", "look", "manually",
    "observe", "open", "perform", "read", "run", "see", "simply", "then", "try", "use", "verify",
];

/// Characters a shell command carries and an English sentence does not.
const COMMAND_PUNCTUATION: &[char] = &['-', '/', '\\', '.', '=', ':', '"', '\'', '|', '$'];

/// Characters a program reference may be spelled with.
fn is_program_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '+' | '-' | '/' | '\\' | ':')
}

/// Does this shell string begin with an executable token?
///
/// Presence and SHAPE are checkable at ingest; RUNNABILITY is not — that is AC-288/AC-290's job,
/// and this function deliberately touches no filesystem and spawns nothing.
///
/// Three rules, each with its blind spot recorded rather than implied:
///   1. the first token carries only characters a program reference may carry;
///   2. the first token is not an English imperative opener followed by further words;
///   3. a command of four or more tokens carrying no flag, path separator, dot or `=` anywhere is
///      prose — a real command of that length carries at least one.
///
/// KNOWN FALSE ACCEPT: a two- or three-token prose fragment opening with an unlisted verb
/// (`"reproduce it locally"`) is accepted. KNOWN FALSE REJECT: a real command whose program is
/// literally named for one of the twenty words above and which takes only bare-word arguments.
/// Both are narrower than the defect this closes, and neither is silent — a rejected item names
/// its command back in the error.
fn begins_with_executable_token(command: &str) -> bool {
    let mut tokens = command.split_whitespace();
    let Some(first) = tokens.next() else {
        return false;
    };

    // 1 — the shape of the program reference itself.
    if !first.chars().all(is_program_char) {
        return false;
    }

    let rest_len = tokens.count();

    // 2 — an imperative opener with an object after it is a sentence, not an invocation.
    if rest_len > 0 && PROSE_OPENERS.contains(&first.to_ascii_lowercase().as_str()) {
        return false;
    }

    // 3 — length without punctuation.
    if rest_len >= 3 && !command.contains(COMMAND_PUNCTUATION) {
        return false;
    }

    true
}

impl Findings {
    /// The one door a review payload comes through.
    ///
    /// Deserialise, then VALIDATE. A malformed item is rejected here with the item identified, and
    /// never reaches the classifier (ADR-0096 decision 1; AC-281).
    pub fn ingest(value: serde_json::Value) -> Result<Self, IngestError> {
        let findings: Self = serde_json::from_value(value).map_err(|e| IngestError::Malformed {
            detail: e.to_string(),
        })?;

        for item in &findings.items {
            if !begins_with_executable_token(&item.repro.command) {
                return Err(IngestError::CommandNotExecutable {
                    item: item.ac.clone(),
                    command: item.repro.command.clone(),
                });
            }
            if item.repro.kind == ReproKind::Mutate && item.repro.mutation.is_none() {
                return Err(IngestError::MutateWithoutMutation {
                    item: item.ac.clone(),
                });
            }
        }

        Ok(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::begins_with_executable_token as ok;

    #[test]
    fn real_commands_are_accepted() {
        for c in [
            "cargo test -p rk-types --test it",
            "pytest",
            "make",
            "npm test",
            "npm run build",
            "go test ./...",
            "./scripts/repro.sh",
            "python spikes/p9/lint.py",
            "dotnet test",
        ] {
            assert!(ok(c), "must accept a real command: {c}");
        }
    }

    #[test]
    fn prose_is_rejected() {
        for c in [
            "run the tests and see",
            "check that the gate closes",
            "just look at the output",
            "reproduce by running the whole suite",
            "",
        ] {
            assert!(!ok(c), "must reject prose: {c:?}");
        }
    }
}
