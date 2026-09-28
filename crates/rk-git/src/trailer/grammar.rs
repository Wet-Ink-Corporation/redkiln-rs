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

/// The reserved stage token that marks a lane's end (`s8e` §8e.5 and §8e.6; D21-43 N9).
///
/// A pack that declares a stage of this name is refused at load, so the token can never name a
/// stage. *(Added 2026-09-28, `s8e` build, package S8E-P4; ADR-0183, proposed.)*
pub const CLOSED_STAGE_TOKEN: &str = "closed";

/// A `Story:` mark's stage component (`s8e` §8e.5; ADR-0183, proposed).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoryStage {
    /// `<project>/<slug> <stage>`: the lane entered `<stage>` at this commit.
    Stage(String),
    /// `<project>/<slug> closed`: the lane ended (D21-43 N9).
    Closed,
}

/// The state a `Gate:` mark carries (`s8e` §8e.7; D21-43 N6; ADR-0184, proposed). The value's
/// sixth token, plus the outcome tokens the deleted-database test puts in git.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GateState {
    /// `opened`, and nothing after it.
    Opened,
    /// `answered <option>`: the chosen option decides the next action (an answered
    /// `unknown_facet` gate binds it; a gate step routes on it), so it is carried.
    Answered { option: String },
    /// `expired <action>`: the terminal expiry action decides the next action (a `fail` halts, an
    /// `auto-decide` answers), so it is carried.
    Expired { action: GateExpiry },
    /// `voided`, and nothing after it.
    Voided,
}

/// The terminal expiry action an `expired` mark carries: `fail`, `escalate`, or
/// `auto-decide <verdict>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GateExpiry {
    Fail,
    Escalate,
    AutoDecide { verdict: String },
}

impl GateState {
    /// The state token, the value's sixth.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Opened => "opened",
            Self::Answered { .. } => "answered",
            Self::Expired { .. } => "expired",
            Self::Voided => "voided",
        }
    }

    /// The tokens after the kind and reach, in value order.
    fn tokens(&self) -> Vec<&str> {
        match self {
            Self::Opened | Self::Voided => vec![self.as_str()],
            Self::Answered { option } => vec![self.as_str(), option.as_str()],
            Self::Expired { action } => match action {
                GateExpiry::Fail => vec![self.as_str(), "fail"],
                GateExpiry::Escalate => vec![self.as_str(), "escalate"],
                GateExpiry::AutoDecide { verdict } => {
                    vec![self.as_str(), "auto-decide", verdict.as_str()]
                }
            },
        }
    }
}

/// One parsed trailer line, classified against the declared grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedTrailer {
    /// *(Widened 2026-09-28, `s8e` build, package S8E-P4; ADR-0183, proposed.)* `stage` is `None`
    /// for the legacy stage-less mark (one token), and `Some` for a stage-bearing or `closed` mark
    /// (two tokens).
    Story {
        project: String,
        slug: String,
        stage: Option<StoryStage>,
    },
    /// A `Story:` with no `project/` form. Recorded separately; folding it into a project would
    /// invent one (design §5.4 row 1).
    BareStory { id: String },
    SliceVerdict {
        project: String,
        slice: String,
        verdict: Verdict,
    },
    /// *(Widened 2026-09-28, `s8e` build, package S8E-P4; ADR-0184, proposed.)* The value is
    /// `<token> <tree_hash> <deadline> <kind> <reach> <state> [<outcome>…]`, the deadline integer
    /// unix seconds (D21-43 N8).
    Gate {
        token: String,
        tree_hash: String,
        deadline: i64,
        kind: String,
        reach: u32,
        state: GateState,
    },
    /// A key outside the three axes. Ignored for state, counted for the record.
    UnknownKey { key: String },
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
    /// `Gate:` whose value does not have the widened value's token count: the old
    /// `(token, tree_hash, deadline)` triple included. F12's corpus carries such lines; design
    /// §5.4 row 3 calls them legacy drift. *(Re-read 2026-09-28, `s8e` build, package S8E-P4;
    /// ADR-0184, proposed: the triple is now legacy.)*
    LegacyGateValue,
    /// `Story:` whose value is not one or two tokens of the declared grammar: three or more
    /// tokens, a stage token carrying `/`, or a stage-bearing value with no `project/` form.
    /// *(Added 2026-09-28, `s8e` build, package S8E-P4; ADR-0183, proposed.)*
    MalformedStoryValue,
    /// `Gate:` with the widened token count whose tokens break their grammar: a tree hash that is
    /// not 40-hex, a deadline that is not integer unix seconds (an RFC 3339 deadline included,
    /// D21-43 N8), a reach that is not a positive integer, an unknown state, or outcome tokens the
    /// state does not take. *(Added 2026-09-28, `s8e` build, package S8E-P4; ADR-0184, proposed.)*
    MalformedGateValue,
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
        TrailerKey::Story => classify_story(value),
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
        TrailerKey::Gate => classify_gate(value),
        TrailerKey::Other(k) => ParsedTrailer::UnknownKey { key: k },
    }
}

/// `Story:` — `<project> "/" <slug> [SP <stage>]` (`s8e` §8e.5; ADR-0183, proposed).
///
/// **Split on whitespace FIRST.** Until 2026-09-28 this arm called `split_qualified` on the whole
/// value, so `redkiln-rs/x analysis` parsed as the slug `x analysis` — ADR-0155 option 2's failure
/// mode. One token is the legacy stage-less mark (or a bare id, as before); two tokens are a
/// stage-bearing mark, the reserved token `closed` being the lane's end; anything else is drift.
fn classify_story(value: &str) -> ParsedTrailer {
    let malformed = || ParsedTrailer::Malformed {
        key: TrailerKey::Story,
        offending: value.to_string(),
        why: MalformedReason::MalformedStoryValue,
    };
    let tokens: Vec<&str> = value.split_whitespace().collect();
    match tokens.as_slice() {
        // Zero or one token: exactly the pre-widening reading, so the one legacy mark on the tree
        // (`Story: redkiln-rs/rk-fix-doctor-test-count` at `73c087a`) reads as it always did.
        [] | [_] => match split_qualified(value) {
            Some((project, slug)) => ParsedTrailer::Story {
                project,
                slug,
                stage: None,
            },
            None => ParsedTrailer::BareStory {
                id: value.to_string(),
            },
        },
        [id, stage] => {
            // A stage-bearing value with no `project/` form is not a bare id with a stage bolted
            // on: the grammar has no such form, so it is drift, never coerced (AC-183).
            let Some((project, slug)) = split_qualified(id) else {
                return malformed();
            };
            if stage.contains('/') {
                return malformed();
            }
            let stage = if *stage == CLOSED_STAGE_TOKEN {
                StoryStage::Closed
            } else {
                StoryStage::Stage((*stage).to_string())
            };
            ParsedTrailer::Story {
                project,
                slug,
                stage: Some(stage),
            }
        }
        _ => malformed(),
    }
}

/// `Gate:` — `<token> SP <tree_hash> SP <deadline> SP <kind> SP <reach> SP <state> [SP <outcome>…]`
/// (`s8e` §8e.7; D21-42 8b; D21-43 N6 and N8; ADR-0184, proposed).
///
/// A value without the widened token count (six to eight) is **legacy drift**, the old triple
/// included, exactly as design §5.4 row 3 treated a non-triple. A value with the count whose
/// tokens break their grammar is malformed drift. Neither is ever a gate.
fn classify_gate(value: &str) -> ParsedTrailer {
    let tokens: Vec<&str> = value.split_whitespace().collect();
    if !(6..=8).contains(&tokens.len()) {
        return ParsedTrailer::Malformed {
            key: TrailerKey::Gate,
            offending: value.to_string(),
            why: MalformedReason::LegacyGateValue,
        };
    }
    let malformed = || ParsedTrailer::Malformed {
        key: TrailerKey::Gate,
        offending: value.to_string(),
        why: MalformedReason::MalformedGateValue,
    };
    if !is_40_hex(tokens[1]) {
        return malformed();
    }
    // Integer unix seconds, unsigned decimal digits only (N8). An RFC 3339 spelling, a sign or a
    // fraction is not the grammar.
    let Some(deadline) = decimal(tokens[2]).and_then(|d| i64::try_from(d).ok()) else {
        return malformed();
    };
    // The first reach is 1 (`GateRecord::derive`), so 0 names no reach.
    let Some(reach) = decimal(tokens[4])
        .and_then(|r| u32::try_from(r).ok())
        .filter(|r| *r >= 1)
    else {
        return malformed();
    };
    let state = match (tokens[5], &tokens[6..]) {
        ("opened", []) => GateState::Opened,
        ("answered", [option]) => GateState::Answered {
            option: (*option).to_string(),
        },
        ("expired", ["fail"]) => GateState::Expired {
            action: GateExpiry::Fail,
        },
        ("expired", ["escalate"]) => GateState::Expired {
            action: GateExpiry::Escalate,
        },
        ("expired", ["auto-decide", verdict]) => GateState::Expired {
            action: GateExpiry::AutoDecide {
                verdict: (*verdict).to_string(),
            },
        },
        ("voided", []) => GateState::Voided,
        _ => return malformed(),
    };
    ParsedTrailer::Gate {
        token: tokens[0].to_string(),
        tree_hash: tokens[1].to_string(),
        deadline,
        kind: tokens[3].to_string(),
        reach,
        state,
    }
}

/// An unsigned decimal of ASCII digits, or `None`.
fn decimal(s: &str) -> Option<u64> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        .then(|| s.parse().ok())
        .flatten()
}

impl ParsedTrailer {
    /// The trailer that parses back to exactly `self`, or `None` when no value does.
    ///
    /// *(Added 2026-09-28, `s8e` build, package S8E-P4; ADR-0183 and ADR-0184, proposed.)* The one
    /// renderer for the two widened axes, and it is checked by **parsing its own output**: a stage
    /// token carrying whitespace or `/`, a stage spelled `closed`, a gate kind or option carrying
    /// whitespace, all render to a value that parses as something else, so they return `None` and
    /// the producer refuses at the mint (the ADR-0161 decision 3 pattern) rather than committing a
    /// value that means something other than it says. ~~Only `Story` and `Gate` render.~~
    /// *(dated 2026-09-28, `s8e` build, package S8E-P7: `Slice-Verdict` renders too, `s8e` §8e.8;
    /// the three axes render and nothing else does.)*
    #[must_use]
    pub fn to_trailer(&self) -> Option<Trailer> {
        let (key, value) = match self {
            Self::Story {
                project,
                slug,
                stage,
            } => (
                TrailerKey::Story,
                match stage {
                    None => format!("{project}/{slug}"),
                    Some(StoryStage::Stage(s)) => format!("{project}/{slug} {s}"),
                    Some(StoryStage::Closed) => format!("{project}/{slug} {CLOSED_STAGE_TOKEN}"),
                },
            ),
            Self::Gate {
                token,
                tree_hash,
                deadline,
                kind,
                reach,
                state,
            } => (
                TrailerKey::Gate,
                format!(
                    "{token} {tree_hash} {deadline} {kind} {reach} {}",
                    state.tokens().join(" ")
                ),
            ),
            // *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8.)* The seal's value,
            // `<project> "/" <slice> SP <verdict>`, through the same round-trip check: a slice
            // carrying whitespace, or an empty component, does not parse back and is refused.
            Self::SliceVerdict {
                project,
                slice,
                verdict,
            } => (
                TrailerKey::SliceVerdict,
                format!("{project}/{slice} {}", verdict.as_str()),
            ),
            _ => return None,
        };
        let parsed = parse_body(&format!("{}: {value}", key.as_str()));
        (parsed.as_slice() == std::slice::from_ref(self)).then_some(Trailer { key, value })
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
