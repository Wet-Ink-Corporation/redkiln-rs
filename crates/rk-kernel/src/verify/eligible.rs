//! **AC-239's gate-eligible adapter list, as data** (PROVISIONAL — S-CODEX not reported in the
//! brief). *"The gate may close only on a result from an adapter on that list, which is data and
//! has exactly one entry in Phase 1."*
//!
//! Data, and the kernel's own lint is why: AC-487's denylist bans `rk-adapter-` on every code
//! line under `crates/rk-kernel/src/`, so the one entry cannot be a constant here — it is read from
//! a file the kernel holds as an opaque string and compares by equality. The evaluator takes the
//! list as an argument ([`GateEligible::may_close`]); nothing reads a constant.
//!
//! ADR-0007's forbids is the rule this enforces: no non-Claude adapter behind a review or
//! verification gate until it passes E3-shaped checks. The Codex-present half (a gate evaluator
//! fed a Codex-sourced result) waits on `rk-adapter-codex`, Phase 2 (ADR-0094 decision 2).
//! *(Noted 2026-09-25, s8a §8a.6 fix round 12, ADR-0167: the Codex adapter is Phase 4, after
//! S-CODEX, as ADR-0007 places it; the exclusion stands, its phase reason is corrected.)*

use crate::toml_subset::{self, Value};

/// The list. Opaque strings, compared by equality.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateEligible {
    phase: u32,
    adapters: Vec<String>,
}

/// Why a list was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EligibleError {
    Toml {
        source: String,
        error: String,
    },
    Shape {
        source: String,
        detail: String,
    },
    /// Phase 1 has exactly one gate-eligible adapter (ADR-0007; AC-239).
    NotExactlyOne {
        source: String,
        count: usize,
    },
}

impl std::fmt::Display for EligibleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
            Self::NotExactlyOne { source, count } => write!(
                f,
                "{source}: the Phase-1 gate-eligible list has {count} entries, not exactly one \
                 (ADR-0007; AC-239)"
            ),
        }
    }
}

impl std::error::Error for EligibleError {}

impl GateEligible {
    /// Read the list. In Phase 1 it has exactly one entry (ADR-0007; AC-239).
    ///
    /// # Errors
    ///
    /// [`EligibleError`].
    pub fn from_toml(name: &str, src: &str) -> Result<Self, EligibleError> {
        let doc = toml_subset::parse(src).map_err(|e| EligibleError::Toml {
            source: name.to_owned(),
            error: e.to_string(),
        })?;
        let shape = |detail: String| EligibleError::Shape {
            source: name.to_owned(),
            detail,
        };
        let phase = doc
            .get("phase")
            .and_then(Value::as_int)
            .and_then(|p| u32::try_from(p).ok())
            .ok_or_else(|| shape("no `phase`".to_owned()))?;
        let adapters = doc
            .get("adapters")
            .and_then(Value::as_array)
            .ok_or_else(|| shape("no `adapters` array".to_owned()))?
            .iter()
            .map(|v| {
                v.as_str()
                    .map(ToOwned::to_owned)
                    .ok_or_else(|| shape("an adapter entry is not a string".to_owned()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if phase == 1 && adapters.len() != 1 {
            return Err(EligibleError::NotExactlyOne {
                source: name.to_owned(),
                count: adapters.len(),
            });
        }
        Ok(Self { phase, adapters })
    }

    #[must_use]
    pub const fn phase(&self) -> u32 {
        self.phase
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.adapters.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }

    pub fn adapters(&self) -> impl Iterator<Item = &str> {
        self.adapters.iter().map(String::as_str)
    }

    /// May a gate close on a result from `result_adapter`? Only if it is on the list — by
    /// equality against the data, never against a name the kernel knows.
    #[must_use]
    pub fn may_close(&self, result_adapter: &str) -> bool {
        !result_adapter.is_empty() && self.adapters.iter().any(|a| a == result_adapter)
    }

    /// The evaluator as a plain function over the list — the signature AC-239's test pins.
    #[must_use]
    pub fn may_close_fn(list: &Self, result_adapter: &str) -> bool {
        list.may_close(result_adapter)
    }
}
