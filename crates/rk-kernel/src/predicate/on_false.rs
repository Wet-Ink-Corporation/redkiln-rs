//! `on_false` — ADR-0103 clause 6's **closed** form set.
//!
//! `{ to = <earlier step id>, max = N }` | `"continue"` | `"fail"` | `{ to = "deviate", … }`
//! (ADR-0046's form), with `max` required only for the loop-back form. The clause names
//! `spikes/p9/lint.py`'s `on_false_of` as the **reference behaviour**, and this module is written
//! against it and against the probe's `check_on_false`, which implements the same set.
//!
//! # Its three live red instances, and whose repair they are
//!
//! The closed set has exactly three live violations in the corpus [measured, F42; re-run
//! 2026-09-13 and reproduced]: `catalog-b/api-contract-spec.toml` step\[5\] `'assemble'`, and
//! `catalog-b/blameless-postmortem.toml` steps \[2\] and \[4\] `'draft'` — bare step names where the
//! set admits a keyword or a table. They are **three of F42's five pack defects**, the other two
//! being the parse failures [`super::parser`] reports, and **the pack repairs are Step 7e §7e.2's,
//! not this crate's** (ADR-0114 decision 1 counts exactly these five and repairs them there before
//! the v0 tag). What this stage owes is the rule that catches them, and those three rows are its
//! only live red fixtures.

use serde_json::Value as Json;

/// A well-formed `on_false`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnFalse {
    /// `"continue"`.
    Continue,
    /// `"fail"`.
    Fail,
    /// `{ to = <earlier step id>, max = N }` — `max` is required in this form and only this form.
    LoopBack { to: String, max: i64 },
    /// `{ to = "deviate", … }` — ADR-0046's form, whose further keys are that record's.
    Deviate,
}

/// Why an `on_false` is outside the closed set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnFalseError {
    /// A string that is neither `"continue"` nor `"fail"` — the three live `catalog-b` defects.
    UnknownKeyword(String),
    /// A table whose key set is not `{to, max}` and whose `to` is not `"deviate"`.
    BadTable(Vec<String>),
    /// Anything that is neither a string nor a table.
    BadType(&'static str),
}

impl std::fmt::Display for OnFalseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownKeyword(s) => write!(
                f,
                "on_false {s:?} is outside ADR-0103 clause 6's closed set: a bare step name is not \
                 a loop-back, which is spelled `{{ to = \"{s}\", max = N }}`"
            ),
            Self::BadTable(keys) => write!(f, "on_false table has keys {keys:?}"),
            Self::BadType(t) => write!(f, "on_false is a {t}"),
        }
    }
}

impl std::error::Error for OnFalseError {}

/// The probe's own form tag for a value, whether or not it is in the closed set.
///
/// `spikes/p9/grammar-probe.py`'s `check_on_false` returns `(good, form)` and counts the **form**
/// regardless of `good`, which is why its census reads `{'keyword': 33, 'loop-back': 66}` over 99
/// values of which 96 are in the set: the three defects are counted as `keyword`. The differential
/// compares this tag, so it must classify the same way.
#[must_use]
pub fn form_tag(value: &Json) -> String {
    match value {
        Json::String(_) => "keyword".to_owned(),
        Json::Object(map) => {
            if map.get("to").and_then(Json::as_str) == Some("deviate") {
                return "deviate".to_owned();
            }
            let mut keys: Vec<&str> = map.keys().map(String::as_str).collect();
            keys.sort_unstable();
            if keys == ["max", "to"]
                && map["to"].is_string()
                && map["max"].is_i64()
                && !map["max"].is_boolean()
            {
                return "loop-back".to_owned();
            }
            format!("bad table {keys:?}")
        }
        Json::Null => "bad type NoneType".to_owned(),
        Json::Bool(_) => "bad type bool".to_owned(),
        Json::Number(_) => "bad type number".to_owned(),
        Json::Array(_) => "bad type list".to_owned(),
    }
}

/// Classify an `on_false` value against ADR-0103 clause 6.
///
/// # Errors
///
/// [`OnFalseError`] for every form outside the closed set.
pub fn classify_on_false(value: &Json) -> Result<OnFalse, OnFalseError> {
    match value {
        Json::String(s) => match s.as_str() {
            "continue" => Ok(OnFalse::Continue),
            "fail" => Ok(OnFalse::Fail),
            other => Err(OnFalseError::UnknownKeyword(other.to_owned())),
        },
        Json::Object(map) => {
            if map.get("to").and_then(Json::as_str) == Some("deviate") {
                return Ok(OnFalse::Deviate);
            }
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let to = map.get("to").and_then(Json::as_str);
            let max = map.get("max").and_then(Json::as_i64);
            match (keys.len(), to, max) {
                (2, Some(t), Some(m)) if keys == ["max", "to"] => Ok(OnFalse::LoopBack {
                    to: t.to_owned(),
                    max: m,
                }),
                _ => Err(OnFalseError::BadTable(keys)),
            }
        }
        Json::Null => Err(OnFalseError::BadType("NoneType")),
        Json::Bool(_) => Err(OnFalseError::BadType("bool")),
        Json::Number(_) => Err(OnFalseError::BadType("number")),
        Json::Array(_) => Err(OnFalseError::BadType("list")),
    }
}
