//! The process pack, as steps 1 and 2 read it.
//!
//! Two tables and no more:
//!
//! * **`[policy] stages = { <stage> = "<rigor>", … }`** — step 1's `policy.<class>`. The pack *is*
//!   the class (its `[pack] applies` is the class predicate), so the per-class indirection design
//!   §4.4 row 1 writes as `policy.<class>` is the pack selection itself, and what is read here is
//!   the per-stage rigor.
//! * **`[stage.<name>] baseline = […]`** plus `[[stage.<name>.require]] when / artifacts`— step 2's
//!   `R(stage, ctx) = baseline ∪ artifacts of every require row whose when holds`.
//!
//! The shape is `spikes/p9/packs/incident.toml`'s, verbatim.
//!
//! # What is NOT read here, and why that is a statement rather than a gap
//!
//! `appetite` is step 5's (AC-058) and `spawn` is the runtime's; both are left on the TOML and not
//! modelled, because a field read into a struct nothing consults reads as implemented.
//! `policy.<cost_class>.stale_policy` is AC-068's, **PROVISIONAL under ADR-0117 clause 4**, and is
//! deliberately not read: `00-preamble.md`:55 forbids defaulting a value a record owes, and a
//! loader that read it would have to decide what an absent one means.

use std::collections::BTreeMap;
use std::fmt;

use crate::rigor::Rigor;
use crate::toml_subset::{self, Value};

/// One `[[stage.<name>.require]]` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequireRow {
    /// The predicate, verbatim. Evaluated at step 2 over the item's context vector.
    pub when: String,
    pub artifacts: Vec<String>,
    /// A row may raise the rigor of what it requires (`rigor = "mandatory"` in the shipped incident
    /// pack's R3 row). Carried; **step 4's candidacy uses the STAGE's rigor**, and a per-row rigor
    /// escalation is not built, so this field is read and reported rather than acted on.
    pub rigor: Option<Rigor>,
    /// `stage.<name>.require[<i>]`, for the location an AC-065 failure names.
    pub site: String,
}

/// One `[stage.<name>]` table.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StageSpec {
    pub baseline: Vec<String>,
    pub require: Vec<RequireRow>,
}

/// What stopped a pack load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackError {
    Toml {
        source: String,
        error: toml_subset::TomlError,
    },
    Shape {
        source: String,
        detail: String,
    },
    /// **AC-050's third AND.** `discouraged` is reserved in Phase 1: the variant exists in the
    /// ordinal, and a pack that sets any stage or `min_rigor` to it fails validation **naming the
    /// site** (decision #20 of 2026-09-11). Its semantics are defined in Phase 2, not defaulted
    /// here, and §7b:768-771 records that a fourth live rigor level was **rejected** for having no
    /// measured need.
    DiscouragedIsReserved {
        site: String,
        token: String,
    },
    /// A rigor token outside the five.
    NotARigor {
        site: String,
        token: String,
    },
}

impl fmt::Display for PackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml { source, error } => write!(f, "{source}: {error}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
            Self::DiscouragedIsReserved { site, token } => write!(
                f,
                "{site}: `{token}` is RESERVED in Phase 1 and a pack may not set it. The variant \
                 exists in the ordinal so the five levels stay total, but its semantics — baseline \
                 runs, `require` rows fire, the run record carries an advisory — are Phase 2's and \
                 are not defaulted here (AC-050; decision #20 of 2026-09-11)"
            ),
            Self::NotARigor { site, token } => write!(
                f,
                "{site}: `{token}` is not one of the five rigor levels (forbidden, discouraged, \
                 optional, recommended, mandatory)"
            ),
        }
    }
}

impl std::error::Error for PackError {}

/// A process pack.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Pack {
    pub id: String,
    /// The class predicate — `applies = "kind == incident"`. Carried; pack SELECTION is the
    /// runtime's and is not steps 1-4's, so nothing here evaluates it.
    pub applies: Option<String>,
    appetite: Option<i64>,
    stage_rigor: BTreeMap<String, Rigor>,
    stages: BTreeMap<String, StageSpec>,
}

impl Pack {
    /// Read one pack.
    ///
    /// # Errors
    ///
    /// [`PackError`] for a file outside the TOML subset, a missing `[pack] id`, a rigor token
    /// outside the five, or `discouraged` anywhere (AC-050).
    pub fn from_toml(name: &str, src: &str) -> Result<Self, PackError> {
        let doc = toml_subset::parse(src).map_err(|error| PackError::Toml {
            source: name.to_owned(),
            error,
        })?;
        let shape = |detail: String| PackError::Shape {
            source: name.to_owned(),
            detail,
        };
        let head = doc
            .get("pack")
            .and_then(Value::as_table)
            .ok_or_else(|| shape("no [pack] table".to_owned()))?;
        let id = head
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(|| shape("[pack] has no id".to_owned()))?
            .to_owned();

        let read_rigor = |site: String, token: &str| -> Result<Rigor, PackError> {
            let r = Rigor::from_token(token).ok_or_else(|| PackError::NotARigor {
                site: site.clone(),
                token: token.to_owned(),
            })?;
            if r == Rigor::Discouraged {
                return Err(PackError::DiscouragedIsReserved {
                    site,
                    token: token.to_owned(),
                });
            }
            Ok(r)
        };

        let mut stage_rigor = BTreeMap::new();
        if let Some(policy) = doc.get("policy").and_then(Value::as_table)
            && let Some(stages) = policy.get("stages").and_then(Value::as_table)
        {
            for (stage, token) in stages {
                let token = token
                    .as_str()
                    .ok_or_else(|| shape(format!("policy.stages.{stage} is not a string")))?;
                stage_rigor.insert(
                    stage.clone(),
                    read_rigor(format!("{name}: policy.stages.{stage}"), token)?,
                );
            }
        }

        let mut stages: BTreeMap<String, StageSpec> = BTreeMap::new();
        if let Some(table) = doc.get("stage").and_then(Value::as_table) {
            for (stage, body) in table {
                let body = body
                    .as_table()
                    .ok_or_else(|| shape(format!("[stage.{stage}] is not a table")))?;
                let baseline = body
                    .get("baseline")
                    .and_then(Value::as_array)
                    .map(|items| {
                        items
                            .iter()
                            .filter_map(Value::as_str)
                            .map(ToOwned::to_owned)
                            .collect()
                    })
                    .unwrap_or_default();
                let mut require = Vec::new();
                if let Some(Value::Array(rows)) = body.get("require") {
                    for (i, row) in rows.iter().enumerate() {
                        let row = row.as_table().ok_or_else(|| {
                            shape(format!("[[stage.{stage}.require]] {i} is not a table"))
                        })?;
                        let site = format!("{name}: stage.{stage}.require[{i}]");
                        let when = row
                            .get("when")
                            .and_then(Value::as_str)
                            .ok_or_else(|| shape(format!("{site} has no `when`")))?
                            .to_owned();
                        let artifacts = row
                            .get("artifacts")
                            .and_then(Value::as_array)
                            .map(|items| {
                                items
                                    .iter()
                                    .filter_map(Value::as_str)
                                    .map(ToOwned::to_owned)
                                    .collect()
                            })
                            .unwrap_or_default();
                        let rigor = match row.get("rigor").and_then(Value::as_str) {
                            Some(t) => Some(read_rigor(format!("{site}.rigor"), t)?),
                            None => None,
                        };
                        require.push(RequireRow {
                            when,
                            artifacts,
                            rigor,
                            site,
                        });
                    }
                }
                stages.insert(stage.clone(), StageSpec { baseline, require });
            }
        }

        Ok(Self {
            id,
            applies: head
                .get("applies")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
            // `appetite = { points = N }` — the shape every shipped pack uses
            // (`spikes/p9/packs/incident.toml`:6). **An absent `appetite` is `None`, never a large
            // sentinel.** `spikes/p9/lattice.py`:146 defaults it to `10**9`, which is the silent-pass
            // shape `00-preamble.md`:55 forbids a loader from adopting: a pack that declares no
            // appetite gets no appetite check, and step 5 says so in `dag_assembled.appetite`.
            appetite: head
                .get("appetite")
                .and_then(Value::as_table)
                .and_then(|t| t.get("points"))
                .and_then(Value::as_int),
            stage_rigor,
            stages,
        })
    }

    /// The rigor `policy.stages` declares for one stage. `None` for a stage the pack does not
    /// mention — **an absence, which is not a `forbidden` and not an `optional`**.
    #[must_use]
    pub fn rigor(&self, stage: &str) -> Option<Rigor> {
        self.stage_rigor.get(stage).copied()
    }

    /// `appetite.points`, or `None` when the pack declares none.
    ///
    /// **`None` is not "unbounded".** Step 5 runs no appetite check for a pack that declares none
    /// and records the absence, because a code default here would decide a value the pack owes
    /// (`00-preamble.md`:55).
    #[must_use]
    pub const fn appetite(&self) -> Option<i64> {
        self.appetite
    }

    /// Every stage the policy mentions, with its rigor, in byte order.
    pub fn policy(&self) -> impl Iterator<Item = (&String, &Rigor)> {
        self.stage_rigor.iter()
    }

    /// Every `[stage.<name>]` table, in byte order — **including stages the `[policy]` block does
    /// not mention**, which still declare requirements.
    pub fn stage_specs(&self) -> impl Iterator<Item = (&String, &StageSpec)> {
        self.stages.iter()
    }

    /// The `[stage.<name>]` table, if the pack declares one.
    #[must_use]
    pub fn stage(&self, stage: &str) -> Option<&StageSpec> {
        self.stages.get(stage)
    }

    pub(in crate::select) fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "applies": self.applies,
            "appetite": self.appetite,
            "policy": self
                .stage_rigor
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::from(v.as_str())))
                .collect::<serde_json::Map<_, _>>(),
            "stages": self
                .stages
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        serde_json::json!({
                            "baseline": v.baseline,
                            "require": v.require.iter().map(|r| serde_json::json!({
                                "when": r.when,
                                "artifacts": r.artifacts,
                                "rigor": r.rigor.map(Rigor::as_str),
                            })).collect::<Vec<_>>(),
                        }),
                    )
                })
                .collect::<serde_json::Map<_, _>>(),
        })
    }
}
