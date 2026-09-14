//! `--facet-provenance` — **AC-520, the derived-facet sibling gate**, as a library rule.
//!
//! # A second rule, a second fixture set, a second failure meaning
//!
//! AC-071 gates `source = "llm"`. **Nothing gated `source = "derived"`** — which is the sharpest
//! thing F39 carries: *"the gate the whole document is shaped around gates the half of the registry
//! that was not the defect, and the half that was ran unchallenged at `auth` 0.00 over 11
//! positives."* AC-520's own *"why it is a criterion and not a sixth clause on AC-071"* paragraph
//! fixes the shape this module takes: **two criteria, two lint rules, two fixture sets, two things
//! that can fail separately.**
//!
//! # The clause set is taken from the register's enumeration, and no count is restated here
//!
//! `ERRATA.md`'s **E-0520-1** (2026-09-13) records that AC-520's *Verified by* opens with a stale
//! *"six must-fail fixture files"* — the 2026-09-13 amendment inserted `deriver_class` and
//! `deriver_record` and did not revisit the number — and fixes the rule this module follows:
//! **read the enumeration, not the number.** Every variant of [`ProvenanceViolation`] is one of the
//! register's enumerated clauses, and the fixture set is one file per file the enumeration implies
//! **plus the positive companion that must LOAD**, *"without which the rule is satisfied by a lint
//! that rejects everything."*
//!
//! # Three things a two-property reading would miss, all three of them here
//!
//! - **A record reading NOT MEASURABLE exists, is pinned and is read, and satisfies the clause.**
//!   It is neither a pass nor a failure (golden constraint 10, Halt 9). It does **two** things, and
//!   the rule did one of them until 2026-09-13: it bars its facet from keying a `require` row, via
//!   [`ProvenanceViolation::NoPositivesKeysRequire`] — so at the pin `recent_deploy` is
//!   `structural` and barred for having no positive instance, and `data_migration` is barred twice
//!   over — **and it is reported so**, via [`ProvenanceReport::not_measurable`], which is the half
//!   a `Result<(), …>` had no channel for. See [`ProvenanceReport`].
//! - **The lexical bar and the no-positive-instance bar are separate clauses doing separate
//!   work**, and a facet may be barred by either or both: `ui` and `flag_exists` are barred as
//!   lexical **while carrying positives**. They are two variants here for that reason.
//! - **Absence is never inferred.** A `derived` facet with no `deriver_class` fails load exactly as
//!   one with no `deriver` does, and a `deriver_class` outside the two tokens is a different
//!   failure from an absent one.
//!
//! # What is ARMED and not discharged here
//!
//! - **The corpus-resolution half.** `deriver_record` resolves corpus-relative through `CORPUS_PIN`
//!   and never through a working tree (AC-464), and every `deriver_record` in
//!   `spikes/p9/facets.toml` names `results/p1-arm-d-panel-scored.json` — which **is not in the
//!   pinned corpus**, because the single pin move is a named act at the END of Step 7b (§7b.11).
//!   This rule therefore takes a [`crate::registry::PinnedCorpus`] as an argument and opens no
//!   path; the resolution half stays **ARMED, NOT DISCHARGED**, as §7b.15's table already records.
//! - **The CLI half.** `redkiln lint --facet-provenance` is `rk-cli`'s surface (**Step 7d**) and
//!   `.redkiln/facets.toml` is the registry **Step 7e** derives. §7b.14 deleted both checkpoint
//!   lines and says what Step 7b owes instead: *"the lint BEHAVIOUR under `cargo test`"*. That is
//!   what is discharged here, over the in-crate fixture registries — and §7b.14's own 🛑 paragraph
//!   fixes the split: the **fail-closed** obligation is discharged **here**, and only the **pass**
//!   branch over the product registry is re-run at Checkpoint 7e.

use std::fmt;

use crate::registry::{PinnedCorpus, Registry, Source};

/// The two tokens AC-520 admits for `deriver_class`, declared **by name, never inferred from
/// omission**.
pub const DERIVER_CLASSES: [&str; 2] = ["lexical", "structural"];

/// One clause of AC-520's enumeration failing for one facet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceViolation {
    /// *"a `derived` facet with no `deriver` field"*.
    NoDeriver { facet: String },
    /// *"a named `deriver` with no scored record"* — the record path resolves at the pin, and
    /// carries nothing for this deriver.
    NoScoredRecord {
        facet: String,
        deriver: String,
        path: String,
    },
    /// The second half of the same THEN clause: the record must be *"**carrying a Wilson 95%
    /// interval and a positive-instance count**"*, and it carries only the count.
    ///
    /// A record reading NOT MEASURABLE is exempt and reports no rate at all — it has no
    /// denominator, which is the whole of `data_migration`'s finding — so this clause is asserted
    /// only where a measurement was made.
    RecordCarriesNoInterval {
        facet: String,
        deriver: String,
        path: String,
    },
    /// *"a record carrying 0 positives whose facet keys a `require` row"* — including a record that
    /// reads NOT MEASURABLE, which exists and is read and is neither a pass nor a failure.
    NoPositivesKeysRequire { facet: String, path: String },
    /// *"a `deriver` declared lexical … whose facet keys a `require` row"*.
    LexicalKeysRequire { facet: String },
    /// *"a `derived` facet with no `deriver_class`"* — absence is never inferred.
    NoDeriverClass { facet: String },
    /// *"… and one whose `deriver_class` is neither of the two tokens"*.
    DeriverClassNotAToken { facet: String, found: String },
    /// *"a `derived` facet with no `deriver_record`"*.
    NoDeriverRecord { facet: String },
    /// *"… and one whose `deriver_record` names a path absent from the pinned corpus"*.
    DeriverRecordAbsentAtPin { facet: String, path: String },
    /// *"a `kernel`-class facet carrying a `deriver`, a `deriver_class`, a `deriver_record` or
    /// claiming a record (ADR-0093 forbids)"* — one violation per offending field.
    KernelFacetCarriesProvenance { facet: String, field: String },
    /// *"a `require` row keyed on a `declared` facet"* — unscored until S-FACET scores a declared
    /// boundary map (ADR-0093 decision 1).
    RequireKeyedOnDeclared { facet: String, when: String },
}

impl ProvenanceViolation {
    /// The facet the clause failed for.
    #[must_use]
    pub fn facet(&self) -> &str {
        match self {
            Self::NoDeriver { facet }
            | Self::NoScoredRecord { facet, .. }
            | Self::RecordCarriesNoInterval { facet, .. }
            | Self::NoPositivesKeysRequire { facet, .. }
            | Self::LexicalKeysRequire { facet }
            | Self::NoDeriverClass { facet }
            | Self::DeriverClassNotAToken { facet, .. }
            | Self::NoDeriverRecord { facet }
            | Self::DeriverRecordAbsentAtPin { facet, .. }
            | Self::KernelFacetCarriesProvenance { facet, .. }
            | Self::RequireKeyedOnDeclared { facet, .. } => facet,
        }
    }

    /// A stable tag for the clause, named after the fixture that pins it.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::NoDeriver { .. } => "no_deriver",
            Self::NoScoredRecord { .. } => "deriver_without_record",
            Self::RecordCarriesNoInterval { .. } => "record_without_interval",
            Self::NoPositivesKeysRequire { .. } => "record_zero_positives_keys_require",
            Self::LexicalKeysRequire { .. } => "lexical_keys_require",
            Self::NoDeriverClass { .. } => "no_deriver_class",
            Self::DeriverClassNotAToken { .. } => "deriver_class_not_a_token",
            Self::NoDeriverRecord { .. } => "no_deriver_record",
            Self::DeriverRecordAbsentAtPin { .. } => "deriver_record_absent_at_pin",
            Self::KernelFacetCarriesProvenance { .. } => "kernel_facet_carries_deriver",
            Self::RequireKeyedOnDeclared { .. } => "require_keyed_on_declared",
        }
    }
}

impl fmt::Display for ProvenanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoDeriver { facet } => write!(
                f,
                "facet {facet:?} is source = \"derived\" and declares no deriver (AC-520)"
            ),
            Self::NoScoredRecord {
                facet,
                deriver,
                path,
            } => write!(
                f,
                "facet {facet:?}: deriver {deriver:?} has no scored record in {path:?} at the pin \
                 (AC-520 — the record must exist, be pinned and be read)"
            ),
            Self::RecordCarriesNoInterval {
                facet,
                deriver,
                path,
            } => write!(
                f,
                "facet {facet:?}: the record for {deriver:?} in {path:?} carries a                  positive-instance count and NO Wilson 95% interval (AC-520's THEN names both, and                  only a NOT MEASURABLE record is exempt)"
            ),
            Self::NoPositivesKeysRequire { facet, path } => write!(
                f,
                "facet {facet:?} keys a require row while its record {path:?} shows no positive \
                 instance (AC-520 — NOT MEASURABLE is neither a pass nor a failure, and may key no \
                 require row)"
            ),
            Self::LexicalKeysRequire { facet } => write!(
                f,
                "facet {facet:?} keys a require row while its deriver_class is \"lexical\" \
                 (AC-520 — a lexical deriver does not satisfy PC8 rule 2 for require-row purposes \
                 whatever its record says)"
            ),
            Self::NoDeriverClass { facet } => write!(
                f,
                "facet {facet:?} is source = \"derived\" and declares no deriver_class (AC-520 — \
                 absence is never inferred; this fails load exactly as a missing deriver does)"
            ),
            Self::DeriverClassNotAToken { facet, found } => write!(
                f,
                "facet {facet:?}: deriver_class {found:?} is neither of {DERIVER_CLASSES:?} \
                 (AC-520)"
            ),
            Self::NoDeriverRecord { facet } => write!(
                f,
                "facet {facet:?} is source = \"derived\" and declares no deriver_record (AC-520)"
            ),
            Self::DeriverRecordAbsentAtPin { facet, path } => write!(
                f,
                "facet {facet:?}: deriver_record {path:?} is absent from the pinned corpus \
                 (AC-520; AC-464 resolves it through CORPUS_PIN and never through a working tree)"
            ),
            Self::KernelFacetCarriesProvenance { facet, field } => write!(
                f,
                "facet {facet:?} is source = \"kernel\" and carries {field:?} — a kernel facet is a \
                 projection of the kernel's own state, with no deriver and no record (ADR-0093 \
                 decision 1 forbids it; AC-520)"
            ),
            Self::RequireKeyedOnDeclared { facet, when } => write!(
                f,
                "require row {when:?} is keyed on the declared facet {facet:?}, which is unscored \
                 until S-FACET scores a declared boundary map (ADR-0093 decision 1; AC-520)"
            ),
        }
    }
}

/// One facet AC-520 requires **reported** — neither passed nor failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotMeasurable {
    pub facet: String,
    /// The `deriver_record` the finding was read from.
    pub path: String,
    /// The deriver the row names.
    pub deriver: String,
    /// `true` when the record carries its own NOT MEASURABLE marker (a feed absent at the pin,
    /// `R-recent-deploy-feed-absent`); `false` when it is a scored zero.
    pub declared: bool,
}

impl fmt::Display for NotMeasurable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let how = if self.declared {
            "NOT MEASURABLE"
        } else {
            "scored, 0 positives"
        };
        write!(
            f,
            "facet {:?}: {how} — the record for {:?} in {:?} exists, is pinned and is read; this \
             is neither a pass nor a failure, and the facet may key no require row",
            self.facet, self.deriver, self.path
        )
    }
}

/// What the rule found when nothing failed.
///
/// **This type exists because `Ok(())` is the wrong answer to AC-520's second AND-clause.** That
/// clause is *"a facet whose scored record shows **no positive instance**, or is NOT MEASURABLE,
/// **is reported so — neither passed nor failed** — and may key no `require` row"*, and only the
/// second half was implemented: a NOT MEASURABLE facet that keyed no row produced **nothing at
/// all** — no report, and no channel to carry one, because the rule returned unit. That is golden
/// constraint 10's shape exactly — not-measurable reported as a pass — in the module whose own
/// header says it is guarding against it. At the pin it is not hypothetical: `data_migration` and
/// `recent_deploy` are both NOT MEASURABLE and the criterion requires them surfaced as such.
///
/// The counts beside the list are what stop an enumeration of **nothing** reading as clean, which
/// is the same job [`crate::rigor::Survey`]'s do.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ProvenanceReport {
    /// Entries whose `source` this gate reads — `derived`, and only that class.
    pub derived_facets: usize,
    /// Records resolved at the pin and read.
    pub records_read: usize,
    /// The facets AC-520 requires reported, in byte order of facet name.
    pub not_measurable: Vec<NotMeasurable>,
}

/// AC-520's rule: fail closed on `source = "derived"`.
///
/// Load fails unless the entry declares **all three** of a named `deriver`, a `deriver_class` of
/// `lexical` or `structural`, and a `deriver_record` naming a scored record that exists at the pin
/// — plus the two require-row bars and ADR-0093's kernel and declared clauses.
///
/// `corpus` is the pinned corpus the `deriver_record` paths resolve against. It is passed in and
/// never opened here: AC-464 refuses a working-tree read, and the record is not in the corpus until
/// §7b.11's single pin move.
///
/// # Errors
///
/// Every clause that failed, for every facet, collected — never the first one only, for the same
/// reason [`crate::facet_stability::facet_stability_rule`] collects: one fixture per clause is only
/// discriminating if a fixture's one defect produces that clause **and no other**.
pub fn facet_provenance_rule(
    registry: &Registry,
    corpus: &PinnedCorpus,
) -> Result<ProvenanceReport, Vec<ProvenanceViolation>> {
    let mut out = Vec::new();
    let mut report = ProvenanceReport::default();
    for facet in registry.facets() {
        let name = facet.name.clone();
        let keys_a_row = registry.keys_a_require_row(&name);

        // ---- ADR-0093 decision 1's two classes that are OUTSIDE this gate, and their own bars --
        match &facet.source {
            Source::Kernel => {
                // "no `deriver`, no record, and carrying either fails load". One violation per
                // offending field, so a registry carrying two is reported as carrying two.
                //
                // FOUR FORMS, NOT THREE. The register writes group 7 with `or` across a `deriver`,
                // a `deriver_class`, a `deriver_record` **or claiming a record**, and E-0520-1's
                // rule is that the enumeration decides. The fourth is not an entry field, which is
                // why it was checked by nothing until 2026-09-13; `PinnedCorpus::claimed_by` is
                // what makes it expressible, and its doc comment states what "claiming" means.
                for (field, present) in [
                    ("deriver", facet.deriver.is_some()),
                    ("deriver_class", facet.deriver_class.is_some()),
                    ("deriver_record", facet.deriver_record.is_some()),
                    ("a scored record", corpus.claimed_by(&name).is_some()),
                ] {
                    if present {
                        out.push(ProvenanceViolation::KernelFacetCarriesProvenance {
                            facet: name.clone(),
                            field: field.to_owned(),
                        });
                    }
                }
                continue;
            }
            Source::Declared => {
                // Unscored until S-FACET scores a declared boundary map, so it may key no row.
                for row in registry.require_rows() {
                    if row.keys.contains(&name) {
                        out.push(ProvenanceViolation::RequireKeyedOnDeclared {
                            facet: name.clone(),
                            when: row.when.clone(),
                        });
                    }
                }
                continue;
            }
            source if !gates(source) => continue,
            _ => {}
        }

        report.derived_facets += 1;

        // ---- `source = "derived"`: all three fields, each declared BY NAME --------------------
        // The three are independent clauses and each is reported on its own: "absence is never
        // inferred", and a rule that stopped at the first missing field would make the second
        // fixture green for the first fixture's reason.
        let deriver = facet.deriver.clone();
        if deriver.is_none() {
            out.push(ProvenanceViolation::NoDeriver {
                facet: name.clone(),
            });
        }
        match facet.deriver_class.as_deref() {
            None => out.push(ProvenanceViolation::NoDeriverClass {
                facet: name.clone(),
            }),
            Some(class) if !DERIVER_CLASSES.contains(&class) => {
                out.push(ProvenanceViolation::DeriverClassNotAToken {
                    facet: name.clone(),
                    found: class.to_owned(),
                });
            }
            Some(_) => {}
        }
        let Some(path) = facet.deriver_record.clone() else {
            out.push(ProvenanceViolation::NoDeriverRecord {
                facet: name.clone(),
            });
            // Without a path there is nothing to resolve; the record clauses below are not
            // "passed", they are unreachable, and inventing a default path would be the working
            // tree read AC-464 forbids.
            continue;
        };

        // ---- the record: resolved at the pin, then read -------------------------------------
        if !corpus.holds(&path) {
            out.push(ProvenanceViolation::DeriverRecordAbsentAtPin {
                facet: name.clone(),
                path,
            });
            continue;
        }
        if let Some(deriver) = deriver {
            match corpus.record_for(&path, &deriver) {
                None => out.push(ProvenanceViolation::NoScoredRecord {
                    facet: name.clone(),
                    deriver,
                    path,
                }),
                Some(record) => {
                    report.records_read += 1;
                    // THE THEN'S SECOND HALF: "carrying a Wilson 95% interval AND a
                    // positive-instance count". The count was checked and the interval was not.
                    if record.lacks_required_interval() {
                        out.push(ProvenanceViolation::RecordCarriesNoInterval {
                            facet: name.clone(),
                            deriver,
                            path: record.path.clone(),
                        });
                    }
                    // THE AND-CLAUSE'S FIRST HALF: "is reported so — neither passed nor failed".
                    // Unconditional on `keys_a_row`, because the reporting obligation is about the
                    // RECORD and the require-row bar below is about the PAIRING. A facet with no
                    // positive instance that keys no row is the case that produced nothing at all
                    // until 2026-09-13.
                    if record.has_no_positive_instance() {
                        report.not_measurable.push(NotMeasurable {
                            facet: name.clone(),
                            path: record.path.clone(),
                            deriver: record.deriver.clone(),
                            declared: record.not_measurable,
                        });
                    }
                    // THE TWO REQUIRE-ROW BARS, SEPARATE CLAUSES DOING SEPARATE WORK. A facet may
                    // be barred by either or both: `ui` and `flag_exists` are barred as lexical
                    // WHILE CARRYING POSITIVES, `recent_deploy` is structural and barred for
                    // having none, and `data_migration` is barred twice over.
                    //
                    // `keys_a_row` IS LOAD-BEARING ON BOTH AND IS PINNED ON BOTH. Dropping it from
                    // the lexical bar makes a lexical facet that keys nothing fail — and dropping
                    // it here makes a legitimately not-measurable facet that keys nothing fail,
                    // which is the "rejects everything" direction the positive companion exists to
                    // guard and which no fixture reached until 2026-09-13.
                    if keys_a_row && record.has_no_positive_instance() {
                        out.push(ProvenanceViolation::NoPositivesKeysRequire {
                            facet: name.clone(),
                            path: record.path.clone(),
                        });
                    }
                    if keys_a_row && facet.declares_lexical() {
                        out.push(ProvenanceViolation::LexicalKeysRequire { facet: name });
                    }
                }
            }
        }
    }
    if out.is_empty() { Ok(report) } else { Err(out) }
}

/// `Source::Derived`, spelled once so the rule and its tests agree about the gate's subject.
#[must_use]
pub const fn gates(source: &Source) -> bool {
    matches!(source, Source::Derived)
}
