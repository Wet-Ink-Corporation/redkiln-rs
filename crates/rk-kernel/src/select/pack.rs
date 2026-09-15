//! The process pack, as steps 1 and 2 read it.
//!
//! Two tables steps 1 and 2 read, plus what the runtime reads through the accessors:
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
//! * **The runtime envelope** — `[retry] attempts_cap` (`N`), `[breaker]
//!   consecutive_platform_failures` (`K`) and `appetite.tokens` (the per-item token appetite).
//!   **Added 2026-09-14, Slice 4b.**
//! * **`[breaker] expiry_secs`** — how long an open breaker mark stands — and **`[role.<name>]
//!   family`**, the role → model-family table the circuit breaker is partitioned by.
//!   **Added 2026-09-14 by operator decision D-7**, which put both here rather than in code:
//!   *"the pack owns roles and tier policy (design :480-481, ADR-0014), `K` is **already** a pack
//!   value asserted by pack validation, and the breaker expiry's home falls out in the same
//!   place."* The role table extends design :480's *"tier policy per role"* by one key beside
//!   `tier`; see [`role_families`] and [`Pack::require_family_for_role`].
//!
//! # What is NOT read here, and why that is a statement rather than a gap
//!
//! **Corrected 2026-09-14.** The sentence that stood here said *"`appetite` is step 5's (AC-058) and
//! `spawn` is the runtime's; both are left on the TOML and not modelled"*, and the first half is now
//! half false: `appetite.points` is still step 5's and still only carried, but `appetite.tokens` is
//! **read** — it is the per-item token appetite [`crate::workflow::budget::evaluate_gate`] evaluates
//! a gate against, and §7b.10 requires it to be *"declared in the pack and refused as a code
//! default"*. The two are **different counters** and are deliberately not folded into one field:
//! points are the cost-class sum invariant I6 compares, tokens are AC-020's second counter, and a
//! single `appetite` field would be the shared counter AC-020 exists to prevent, arriving through a
//! loader. `spawn` is still the runtime's and is still not modelled.
//!
//! **All FIVE of those values are deliberately absent from [`Pack::to_json`]**, which is the
//! `dag_assembled` payload's shape: adding a member there changes bytes AC-040's byte-identity
//! assertions and the committed golden log are measured over. What the runtime reads, it reads
//! through the accessors. *(This said "**The three new values**" until 2026-09-14; the expiry and
//! the role table joined them under D-7 and are held to the same rule — `to_json` is byte-identical
//! before and after, which is what leaves §7b.11's layout and the golden log untouched by this
//! change.)*
//!
//! `policy.<cost_class>.stale_policy` is AC-068's, **PROVISIONAL under ADR-0117 clause 4**, and is
//! deliberately not read: `00-preamble.md`:55 forbids defaulting a value a record owes, and a
//! loader that read it would have to decide what an absent one means.

use std::collections::BTreeMap;
use std::fmt;

use crate::rigor::Rigor;
use crate::toml_subset::{self, Value};
use crate::workflow::breaker::{BreakerExpiry, FamilyKey};
use crate::workflow::budget::Tokens;
use crate::workflow::envelope::{AttemptCap, BreakerThreshold, EnvelopeError};

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
    /// **The runtime envelope the pack declares, and this crate refuses to default.** Three
    /// `Option`s and three absences: `None` is *"the pack declares none"* and is neither a zero nor
    /// an unbounded value — `crate::workflow::envelope`'s header carries the whole reading, and the
    /// keys these are read from are stated there as a reading with a record OWED.
    attempt_cap: Option<AttemptCap>,
    breaker_threshold: Option<BreakerThreshold>,
    token_appetite: Option<Tokens>,
    /// `[breaker] expiry_secs` — **the breaker expiry's home, decided 2026-09-14 (D-7)**. `None` is
    /// *"the pack declares none"*, on the three fields above's doctrine exactly.
    breaker_expiry: Option<BreakerExpiry>,
    /// **`[role.<name>] family` — the breaker's partition key, per role.**
    ///
    /// An EMPTY map is *"the pack declares no role table"* and is neither an identity map nor a
    /// wildcard: [`Self::require_family_for_role`] refuses for every role against it. The values are
    /// carried as `String` rather than as [`FamilyKey`] because this is the **declaration**; the
    /// key is minted at the resolution, which is the one place a reader should have to look.
    role_families: BTreeMap<String, String>,
    /// **AC-036's substitution permission**, from `[degradation] permit_substitution`. `None` is
    /// *"the pack never considered substitution"* and `Some(false)` is *"it considered it and said
    /// no"*; both refuse, and keeping them apart is what stops a fail-closed fold reading as a
    /// constant. The key spelling is a reading — see [`Self::substitution_permitted`].
    substitution_permitted: Option<bool>,
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
            // THE RUNTIME ENVELOPE, and an absent table is an ABSENCE rather than a number. The
            // three key spellings are a reading and no record states them —
            // `crate::workflow::envelope`'s header carries it and names the record that is OWED.
            attempt_cap: declared_u32(&doc, "retry", "attempts_cap", &shape)?
                .map(AttemptCap::declared),
            breaker_threshold: declared_u32(
                &doc,
                "breaker",
                "consecutive_platform_failures",
                &shape,
            )?
            .map(BreakerThreshold::declared),
            // `appetite.tokens` sits beside `appetite.points` and is a DIFFERENT counter: points
            // are step 5's cost-class sum (AC-058, invariant I6), tokens are AC-020's second
            // counter. A pack declaring only `points` declares no token appetite.
            token_appetite: token_appetite(head, &shape)?,
            // AC-036's permission. A BOOLEAN, not one of §7b.10's unmeasured quantities: a
            // permission that was never granted is an absence, and the fail-closed fold is
            // `permits_substitution`. A non-boolean value is a `Shape` error, never a silent
            // `false` — the same discipline `declared_u32` holds for the three numbers.
            substitution_permitted: declared_bool(
                &doc,
                "degradation",
                "permit_substitution",
                &shape,
            )?,
            // The breaker expiry, read with `declared_u32`'s discipline for `declared_u32`'s
            // reason: `expiry_secs = -1` and `expiry_secs = "ten minutes"` are SHAPE errors and
            // never a silent zero, which would be a mark expired the instant it opened.
            breaker_expiry: declared_u32(&doc, "breaker", "expiry_secs", &shape)?
                .map(|secs| BreakerExpiry::declared(u64::from(secs))),
            // THE BREAKER'S KEY, per role. An absent `[role]` table is an EMPTY map — an absence,
            // never an identity map and never a wildcard — and `require_family_for_role` is where
            // that absence fails.
            role_families: role_families(&doc, &shape)?,
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

    /// `[retry] attempts_cap` — **AC-029's `N`** — or `None` when the pack declares none.
    ///
    /// `None` is the **absence**, on [`Self::appetite`]'s exact doctrine: not unbounded, not zero,
    /// and not a load failure. Where a missing declaration fails is [`Self::require_attempt_cap`],
    /// which is the *"pack-validation"* half AC-029's *Verified by* names.
    #[must_use]
    pub const fn attempt_cap(&self) -> Option<AttemptCap> {
        self.attempt_cap
    }

    /// `[breaker] consecutive_platform_failures` — **AC-033's `K`** — or `None`.
    ///
    /// The accessor is here; **the breaker is not built** (Slice 4d), and nothing in Slice 4b may
    /// be read as covering AC-033. `None` is the absence, as above.
    #[must_use]
    pub const fn breaker_threshold(&self) -> Option<BreakerThreshold> {
        self.breaker_threshold
    }

    /// `appetite.tokens` — the **per-item token appetite**, fixed at triage — or `None`.
    ///
    /// A **different counter** from [`Self::appetite`]'s points: points are step 5's cost-class sum
    /// (AC-058, invariant I6) and tokens are AC-020's second counter. A pack declaring only
    /// `appetite.points` declares no token appetite, and this returns `None` for it.
    #[must_use]
    pub const fn token_appetite(&self) -> Option<Tokens> {
        self.token_appetite
    }

    /// The attempt cap, or the refusal. **AC-029's pack-validation half.**
    ///
    /// > *Verified by:* … a pack-validation test that `N` is **declared, never defaulted in code**.
    ///
    /// # Errors
    ///
    /// [`EnvelopeError::AttemptCapNotDeclared`] when the pack declares no `[retry] attempts_cap`.
    /// There is no `unwrap_or`, no `const` fallback and no `Default` on [`AttemptCap`] behind this
    /// call: the refusal is the only other outcome.
    pub fn require_attempt_cap(&self) -> Result<AttemptCap, EnvelopeError> {
        self.attempt_cap
            .ok_or_else(|| EnvelopeError::AttemptCapNotDeclared {
                pack: self.id.clone(),
            })
    }

    /// The per-item token appetite, or the refusal.
    ///
    /// # Errors
    ///
    /// [`EnvelopeError::TokenAppetiteNotDeclared`] when the pack declares no `appetite.tokens`. A
    /// gate is **not** evaluated against a defaulted token budget: an absent appetite refuses here
    /// rather than passing as zero (which closes nothing) or as unbounded (which closes everything).
    pub fn require_token_appetite(&self) -> Result<Tokens, EnvelopeError> {
        self.token_appetite
            .ok_or_else(|| EnvelopeError::TokenAppetiteNotDeclared {
                pack: self.id.clone(),
            })
    }

    /// The breaker threshold, or the refusal. **AC-033's pack-validation half.**
    ///
    /// > *Verified by:* … `K` is a pack value, **asserted present by pack validation**.
    ///
    /// # Errors
    ///
    /// [`EnvelopeError::BreakerThresholdNotDeclared`] when the pack declares no
    /// `[breaker] consecutive_platform_failures`. There is no `unwrap_or`, no `const` fallback and
    /// no `Default` on [`BreakerThreshold`] behind this call, and AC-033's own *Note* records why
    /// one could not be justified even if the rule allowed it: *"no corpus figure sizes `K`, and no
    /// spike measured a `Platform` failure rate at all (gap 3)."*
    pub fn require_breaker_threshold(&self) -> Result<BreakerThreshold, EnvelopeError> {
        self.breaker_threshold
            .ok_or_else(|| EnvelopeError::BreakerThresholdNotDeclared {
                pack: self.id.clone(),
            })
    }

    /// `[breaker] expiry_secs` — **how long an open breaker mark stands** — or `None`.
    ///
    /// `None` is the absence, on [`Self::appetite`]'s doctrine: not *"never expires"*, not
    /// *"expires at once"*, and not a load failure. Where a missing declaration fails is
    /// [`Self::require_breaker_expiry`].
    #[must_use]
    pub const fn breaker_expiry(&self) -> Option<BreakerExpiry> {
        self.breaker_expiry
    }

    /// The breaker expiry, or the refusal.
    ///
    /// # Errors
    ///
    /// [`EnvelopeError::BreakerExpiryNotDeclared`] when the pack declares no
    /// `[breaker] expiry_secs`. There is no `unwrap_or`, no `const` fallback, no `Default` on
    /// [`BreakerExpiry`] — **and no derivation from the backoff cap**, which was one of the four
    /// homes `crate::workflow::breaker`'s header listed and is *"a default with arithmetic in front
    /// of it"*. The operator decision of 2026-09-14 (D-7) put the expiry here, beside `K`, because
    /// the pack already owns `K` and the two are read together or not at all.
    pub fn require_breaker_expiry(&self) -> Result<BreakerExpiry, EnvelopeError> {
        self.breaker_expiry
            .ok_or_else(|| EnvelopeError::BreakerExpiryNotDeclared {
                pack: self.id.clone(),
            })
    }

    /// `[role.<role>] family` — **the model family this pack binds one role to** — or `None`.
    ///
    /// The raw declaration, for a caller that needs to report the table rather than key on it.
    /// The key itself comes from [`Self::require_family_for_role`].
    #[must_use]
    pub fn family_for_role(&self, role: &str) -> Option<&str> {
        self.role_families.get(role).map(String::as_str)
    }

    /// Every `[role.<name>] family` declaration, in byte order — the table as declared.
    ///
    /// Carried so a test can assert the **premise** of an isolation case (*"these two roles map to
    /// different families and these two map to the same one"*) rather than trusting it, and so a
    /// future `doctor` rule has the whole table rather than one lookup.
    pub fn role_families(&self) -> impl Iterator<Item = (&String, &String)> {
        self.role_families.iter()
    }

    /// **The breaker's partition key for one role — the whole of the resolution.**
    ///
    /// > **D-7, 2026-09-14** — *"The pack declares the family per role (extending design :480's
    /// > 'tier policy per role'); the kernel resolves role → family at plan time and keys the
    /// > breaker on the resolved family."*
    ///
    /// The argument is a **role**, which is `crate::catalog::Step::role`'s value, and it is
    /// deliberately not an `&Activity`: the activity's own `family` is ADR-0058's **activity**
    /// family, and a signature that could not see it is cheaper than a rule that forbids reading it.
    ///
    /// **The role is never its own family.** A role absent from the table refuses; it does not fall
    /// back to `FamilyKey::supplied(role)`. That identity fallback is D-7's rejected alternative
    /// (ii) written as a default: *"N roles map to one model, so a per-role breaker stops one role's
    /// share of traffic to a failing model while the rest keeps flowing, and AC-034's 'never trips
    /// globally' becomes true in a way that misses the point."*
    ///
    /// # Errors
    ///
    /// [`EnvelopeError::RoleFamilyNotDeclared`], naming the pack **and** the role.
    pub fn require_family_for_role(&self, role: &str) -> Result<FamilyKey, EnvelopeError> {
        self.role_families.get(role).map_or_else(
            || {
                Err(EnvelopeError::RoleFamilyNotDeclared {
                    pack: self.id.clone(),
                    role: role.to_owned(),
                })
            },
            |family| Ok(FamilyKey::supplied(family)),
        )
    }

    /// `[degradation] permit_substitution` — **AC-036's permission** — or `None` when the pack
    /// declares neither.
    ///
    /// `None` and `Some(false)` are kept apart deliberately: a pack that never considered
    /// substitution and a pack that considered it and said no are different states, and collapsing
    /// them at the accessor would hide the first. Both refuse; see [`Self::permits_substitution`].
    ///
    /// **The key spelling is a READING and no record states it** — the same gap this module already
    /// records for `[retry] attempts_cap`, `[breaker] consecutive_platform_failures` and
    /// `appetite.tokens`. [measured 2026-09-14: `grep -rn "permit_substitution" spikes/p9/packs/`
    /// → 0 hits across all twelve shipped packs.] A record naming all four is OWED.
    #[must_use]
    pub const fn substitution_permitted(&self) -> Option<bool> {
        self.substitution_permitted
    }

    /// Whether the pack permits a model substitution. **Absent is refused**, and that is not a
    /// defaulted constant.
    ///
    /// §7b.10's 🛑 forbids defaulting `K`, `N`, the backoff base and cap, the headroom margin and
    /// the appetite — **quantities with no measured value**. A permission is not one of those:
    /// AC-036's WHEN is *"the pack **does not permit** substitution"*, and a permission that was
    /// never granted is a permission that is absent. The fold is fail-closed, which is the
    /// direction AC-036's THEN points — *"the substitution is refused and the activity fails rather
    /// than running at the lower tier"*.
    #[must_use]
    pub fn permits_substitution(&self) -> bool {
        self.substitution_permitted == Some(true)
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

/// One `[<table>] <key> = <integer>` declaration, or `None` when the pack declares none.
///
/// **The absence and the malformed value are different outcomes**, and keeping them apart is the
/// whole of `00-preamble.md`:55 here: a pack that declares nothing gets `None`, and a pack that
/// declares `attempts_cap = -1` or `attempts_cap = "four"` gets a [`PackError::Shape`] naming the
/// site. **Never a silent zero** — the discipline is `catalog.rs`'s `retry_max`/`activity_budget`
/// pair, verbatim.
///
/// # Errors
///
/// [`PackError::Shape`] for a table that is not a table, a value that is not an integer, and a
/// value outside `u32` in either direction.
fn declared_u32(
    doc: &toml_subset::Table,
    table: &str,
    key: &str,
    shape: &impl Fn(String) -> PackError,
) -> Result<Option<u32>, PackError> {
    let Some(value) = doc.get(table) else {
        return Ok(None);
    };
    let inner = value
        .as_table()
        .ok_or_else(|| shape(format!("[{table}] is not a table")))?;
    let Some(raw) = inner.get(key) else {
        return Ok(None);
    };
    let raw = raw
        .as_int()
        .ok_or_else(|| shape(format!("{table}.{key} is not an integer")))?;
    u32::try_from(raw)
        .map(Some)
        .map_err(|_| shape(format!("{table}.{key} is {raw}, which is outside u32")))
}

/// One `[<table>] <key> = <boolean>` declaration, or `None` when the pack declares none.
///
/// **The absence and the malformed value are different outcomes**, exactly as in [`declared_u32`]:
/// a pack that declares nothing gets `None`, and a pack that declares
/// `permit_substitution = "yes"` gets a [`PackError::Shape`] naming the site. **Never a silent
/// `false`** — a permission that failed to parse is not a permission that was refused.
///
/// # Errors
///
/// [`PackError::Shape`] for a table that is not a table and a value that is not a boolean.
fn declared_bool(
    doc: &toml_subset::Table,
    table: &str,
    key: &str,
    shape: &impl Fn(String) -> PackError,
) -> Result<Option<bool>, PackError> {
    let Some(value) = doc.get(table) else {
        return Ok(None);
    };
    let inner = value
        .as_table()
        .ok_or_else(|| shape(format!("[{table}] is not a table")))?;
    let Some(raw) = inner.get(key) else {
        return Ok(None);
    };
    raw.as_bool()
        .map(Some)
        .ok_or_else(|| shape(format!("{table}.{key} is not a boolean")))
}

/// Every `[role.<name>] family = "<token>"` declaration, or an empty map.
///
/// **The shape extends design :480's *"tier policy per role"***, which already puts a
/// `[role.<name>]` table in the process pack (*"`[role.implementer] tier = { model = …, effort = …
/// }`"*, proposal §4.2.1). What is added is one key beside `tier`, and the spelling is a **READING**
/// with a record OWED — `crate::workflow::envelope`'s 🛑 already carries that statement for the
/// three envelope keys and this is the fourth. [measured 2026-09-14: `grep -rn role
/// spikes/p9/packs/` → **0 hits across all twelve shipped packs**, so no shipped pack contradicts
/// the spelling and none satisfies it either; the shipped packs' own tables are Step 7e's.]
///
/// **A `[role.<name>]` table that declares no `family` is an ABSENCE, not an error.** The table is
/// the pack's to put a tier in, and a role with a tier and no family has declared no family;
/// [`Pack::require_family_for_role`] is where that fails. A `family` that is not a string is a
/// [`PackError::Shape`] naming the site — [`declared_u32`]'s discipline, for its reason: a dropped
/// key reads as a role the pack never bound, and the two are different states.
///
/// # Errors
///
/// [`PackError::Shape`] for a `[role]` that is not a table, a `[role.<name>]` that is not a table,
/// and a `family` that is not a string.
fn role_families(
    doc: &toml_subset::Table,
    shape: &impl Fn(String) -> PackError,
) -> Result<BTreeMap<String, String>, PackError> {
    let Some(value) = doc.get("role") else {
        return Ok(BTreeMap::new());
    };
    let table = value
        .as_table()
        .ok_or_else(|| shape("[role] is not a table".to_owned()))?;
    let mut out = BTreeMap::new();
    for (role, body) in table {
        let body = body
            .as_table()
            .ok_or_else(|| shape(format!("[role.{role}] is not a table")))?;
        let Some(raw) = body.get("family") else {
            continue;
        };
        let family = raw
            .as_str()
            .ok_or_else(|| shape(format!("role.{role}.family is not a string")))?;
        out.insert(role.clone(), family.to_owned());
    }
    Ok(out)
}

/// `appetite.tokens`, or `None`.
///
/// # Errors
///
/// [`PackError::Shape`] for an `appetite` that is not a table, a `tokens` that is not an integer,
/// and a negative one — never a silent zero, for `declared_u32`'s reason.
fn token_appetite(
    head: &toml_subset::Table,
    shape: &impl Fn(String) -> PackError,
) -> Result<Option<Tokens>, PackError> {
    let Some(value) = head.get("appetite") else {
        return Ok(None);
    };
    let table = value
        .as_table()
        .ok_or_else(|| shape("[pack] appetite is not a table".to_owned()))?;
    let Some(raw) = table.get("tokens") else {
        return Ok(None);
    };
    let raw = raw
        .as_int()
        .ok_or_else(|| shape("appetite.tokens is not an integer".to_owned()))?;
    if raw < 0 {
        return Err(shape(format!(
            "appetite.tokens is {raw}, which is negative"
        )));
    }
    Ok(Some(Tokens::new(raw.unsigned_abs())))
}
