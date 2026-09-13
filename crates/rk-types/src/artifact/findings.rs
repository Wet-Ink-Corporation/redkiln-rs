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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub ac: String,
    pub claim: Text,
    pub rationale: Text,
    pub repro: Repro,
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
