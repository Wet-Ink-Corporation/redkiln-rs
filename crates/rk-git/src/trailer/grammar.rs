//! The trailer grammar — design §5.4's table as code (ADR-0105 clause 3; ADR-0109 decision 3).
//!
//! Three axes, because one is provably insufficient: over **360 trailer commits across 74 slices**,
//! **15/74 = 20.3%** of slices passed through a rejected state and 21.6% were sealed more than once
//! [measured, F12]. `Story:` establishes that work was *committed*; only `Slice-Verdict:` says it
//! was *accepted*, and a slice rejected at review looks identical in commit history to one that
//! passed. **Resume from `Story:` alone is forbidden.**
//!
//! Two rules here are easy to lose and are each pinned by a test:
//!
//! * **Column 0, case-sensitive.** Anchored with `strip_prefix`, never `trim_start` — an indented
//!   or lower-cased mention inside prose is prose, not a trailer.
//! * **An unknown key is counted, not drift.** Design §5.4's last row says unknown keys are
//!   *"ignored and counted"*; `Fix:`, `Slice-Repair:` and `Baseline-Repair:` are the old redkiln's
//!   keys, read by `redkiln-migrate` only and **not emitted by `rk-git`**. Folding them into
//!   `Drift` would make a migration artefact look like a defect.

use std::collections::BTreeMap;

/// A trailer this crate is willing to write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trailer {
    pub key: TrailerKey,
    pub value: String,
}

impl Trailer {
    #[must_use]
    pub fn to_line(&self) -> String {
        format!("{}: {}", self.key.as_str(), self.value)
    }
}

/// The three axes, plus everything else.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrailerKey {
    Story,
    SliceVerdict,
    Gate,
    /// Any other key. Counted, never a transition.
    Other(String),
}

impl TrailerKey {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Story => "Story",
            Self::SliceVerdict => "Slice-Verdict",
            Self::Gate => "Gate",
            Self::Other(k) => k,
        }
    }

    #[must_use]
    pub fn parse(k: &str) -> Self {
        // Case-SENSITIVE on the key, matching `src/store/verify.ts`'s `STORY_TRAILER`.
        match k {
            "Story" => Self::Story,
            "Slice-Verdict" => Self::SliceVerdict,
            "Gate" => Self::Gate,
            other => Self::Other(other.to_string()),
        }
    }
}

/// The verdict vocabulary. Compared case-insensitively, stored lower-case (design §5.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    Approved,
    ChangesRequested,
}

impl Verdict {
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "approved" => Some(Self::Approved),
            "changes-requested" => Some(Self::ChangesRequested),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::ChangesRequested => "changes-requested",
        }
    }
}

/// One parsed trailer line, classified against the declared grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedTrailer {
    Story {
        project: String,
        slug: String,
    },
    /// A `Story:` with no `project/` form. Recorded separately; folding it into a project would
    /// invent one (design §5.4 row 1).
    BareStory {
        id: String,
    },
    SliceVerdict {
        project: String,
        slice: String,
        verdict: Verdict,
    },
    Gate {
        token: String,
        tree_hash: String,
        deadline: String,
    },
    /// A key outside the three axes. Ignored for state, counted for the record.
    UnknownKey {
        key: String,
    },
    /// The line parses as a trailer but its VALUE does not satisfy the declared grammar. Surfaced
    /// with the offending text, never skipped and never coerced into a transition (AC-165).
    Malformed {
        key: TrailerKey,
        offending: String,
        why: MalformedReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MalformedReason {
    /// `Slice-Verdict:` whose value is not `<project>/<slice> <verdict>`.
    UnqualifiedSliceVerdict,
    /// `Slice-Verdict:` whose verdict token is outside the vocabulary.
    UnknownVerdict,
    /// `Gate:` whose value is not the `(token, tree_hash, deadline)` triple. F12's corpus carries
    /// such lines; design §5.4 row 3 calls them legacy drift.
    LegacyGateValue,
}

/// Parse the trailer lines out of one commit message body.
///
/// Anchored at column 0 and case-sensitive on the key. Returns every line that looks like a
/// trailer, classified — including the malformed ones, because AC-165 requires them surfaced.
#[must_use]
pub fn parse_body(body: &str) -> Vec<ParsedTrailer> {
    let mut out = Vec::new();
    for line in body.lines() {
        // Column 0: no `trim_start`. An indented mention inside prose must not register.
        let Some((key_raw, value_raw)) = line.split_once(": ") else {
            continue;
        };
        if key_raw.is_empty()
            || key_raw.starts_with(char::is_whitespace)
            || key_raw.contains(char::is_whitespace)
        {
            continue;
        }
        let key = TrailerKey::parse(key_raw);
        let value = value_raw.trim_end();
        out.push(classify(key, value));
    }
    out
}

fn classify(key: TrailerKey, value: &str) -> ParsedTrailer {
    match key {
        TrailerKey::Story => match split_qualified(value) {
            Some((project, slug)) => ParsedTrailer::Story { project, slug },
            None => ParsedTrailer::BareStory {
                id: value.to_string(),
            },
        },
        TrailerKey::SliceVerdict => {
            let tokens: Vec<&str> = value.split_whitespace().collect();
            // "exactly two whitespace-separated tokens" — a third token is drift under the
            // declared grammar, even though the spike's parser ignored it.
            if tokens.len() != 2 {
                return ParsedTrailer::Malformed {
                    key: TrailerKey::SliceVerdict,
                    offending: value.to_string(),
                    why: MalformedReason::UnqualifiedSliceVerdict,
                };
            }
            let Some((project, slice)) = split_qualified(tokens[0]) else {
                return ParsedTrailer::Malformed {
                    key: TrailerKey::SliceVerdict,
                    offending: value.to_string(),
                    why: MalformedReason::UnqualifiedSliceVerdict,
                };
            };
            match Verdict::parse(tokens[1]) {
                Some(verdict) => ParsedTrailer::SliceVerdict {
                    project,
                    slice,
                    verdict,
                },
                None => ParsedTrailer::Malformed {
                    key: TrailerKey::SliceVerdict,
                    offending: value.to_string(),
                    why: MalformedReason::UnknownVerdict,
                },
            }
        }
        TrailerKey::Gate => {
            let tokens: Vec<&str> = value.split_whitespace().collect();
            if tokens.len() == 3 && is_40_hex(tokens[1]) {
                ParsedTrailer::Gate {
                    token: tokens[0].to_string(),
                    tree_hash: tokens[1].to_string(),
                    deadline: tokens[2].to_string(),
                }
            } else {
                ParsedTrailer::Malformed {
                    key: TrailerKey::Gate,
                    offending: value.to_string(),
                    why: MalformedReason::LegacyGateValue,
                }
            }
        }
        TrailerKey::Other(k) => ParsedTrailer::UnknownKey { key: k },
    }
}

/// Split `project/slug`, or `None` for a bare id.
fn split_qualified(v: &str) -> Option<(String, String)> {
    let v = v.trim();
    let (p, s) = v.split_once('/')?;
    (!p.is_empty() && !s.is_empty()).then(|| (p.to_string(), s.to_string()))
}

fn is_40_hex(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Unknown keys, counted. Design §5.4's last row: *ignored and counted* — not drift.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UnknownKeyCounts(pub BTreeMap<String, u32>);

impl UnknownKeyCounts {
    pub fn record(&mut self, key: &str) {
        *self.0.entry(key.to_string()).or_insert(0) += 1;
    }

    #[must_use]
    pub fn total(&self) -> u32 {
        self.0.values().sum()
    }
}
