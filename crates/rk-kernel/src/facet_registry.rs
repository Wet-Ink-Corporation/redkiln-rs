//! **AC-072, the registry-entry loader rule** — the **THIRD** rule, and the one the register
//! forbids collapsing into either facet lint.
//!
//! # Three rules, three subjects, three fixture sets
//!
//! [`crate::facet_stability`] is AC-071 and its subject is `source = "llm"` against a scored
//! stability record. [`crate::facet_provenance`] is AC-520 and its subject is `source = "derived"`
//! against a scored precision record. **This rule's subject is the ENTRY** — every entry, whatever
//! its source — and it reads no record at all. AC-072's own 2026-09-13 amendment fixes the split in
//! one sentence: *"AC-520 owns what the class and the record MEAN, this criterion owns that the
//! ENTRY carries them — and the two must not be collapsed into one lint rule."*
//!
//! That is why [`EntryShapeViolation::DerivedMissingField`] asks only whether the three fields are
//! **present** and never whether `deriver_class` is one of the two tokens: the token set is
//! AC-520's meaning-clause and [`crate::facet_provenance::ProvenanceViolation::DeriverClassNotAToken`]
//! is where it is pinned. A rule here that also checked the token would be the collapse, spelled
//! as a duplicate assertion.
//!
//! # The one clause that is shared on purpose
//!
//! *"a `require` row keyed on a `declared` facet"* is enumerated in **both** criteria's *Verified
//! by*, and it is implemented in both. That is not a collapse: AC-072 fails it as an **entry-shape**
//! consequence — `declared` is one of the five tokens and the token carries a bar with it — and
//! AC-520 fails it as a **provenance** consequence, over a facet S-FACET has not scored. Two rules
//! reaching the same registry defect by two routes is the shape the register describes when it says
//! *"four areas lint the same registry"*.
//!
//! # WHAT IS MEASURED, AND WHAT IS ARMED
//!
//! | AC-072 THEN clause | pinned here | by |
//! | --- | --- | --- |
//! | a live registry entry for every name a predicate reads | yes | [`EntryShapeViolation::UnregisteredFacet`], over `spikes/p9/facets.toml` itself |
//! | a declared domain | yes | [`EntryShapeViolation::NoDeclaredDomain`] |
//! | `source` one of the five closed tokens | yes | [`EntryShapeViolation::SourceNotAToken`] |
//! | **an anchor** | **NO — ARMED** | see below |
//! | `nullable = true` where the facet may abstain | yes | [`EntryShapeViolation::AbstainsWithoutNullable`] |
//! | — and its sub-clause: a `require`/`applicable_if` reading a nullable facet without handling `null` | yes, **by a different rule, deliberately** | [`crate::predicate::lint`] — AC-069 / ADR-0103 decision 5(i) |
//! | `derived` carries all three of `deriver`, `deriver_class`, `deriver_record` | yes | [`EntryShapeViolation::DerivedMissingField`] |
//! | `kernel` carries none of them | yes | [`EntryShapeViolation::KernelEntryCarriesProvenance`] |
//! | `declared` keys no `require` row | yes | [`EntryShapeViolation::RequireKeyedOnDeclared`] |
//! | the domain is a shape recognised **by name** | yes, and see below | [`EntryShapeViolation::DomainShapeNotRecognised`] |
//! | facet names match `[a-z_][a-z0-9_]*` | yes | [`EntryShapeViolation::FacetNameOutsideLexis`] |
//!
//! # The `nullable` sub-clause is AC-069's rule, and is NOT duplicated here
//!
//! AC-072's `nullable` AND-clause carries a sub-rule: *"A `require` row or `applicable_if`
//! that reads a `nullable` facet and does not handle `null` is a lint error at pack load
//! (ADR-0103 decision 5(i))."* That rule **exists and is built** — [`crate::predicate::lint`],
//! whose violation is *"an ordering comparison reached a nullable axis with no recognised
//! narrowing guard"*, with ADR-0103 decision 4 closing the guard's shape to exactly one form
//! and AC-069's fixture set behind it.
//!
//! It is **not re-implemented here**, for this criterion's own stated reason: AC-072 owns that
//! **the ENTRY carries** `nullable`; what a predicate must then DO with a null read is a
//! different rule with different fixtures. The register says exactly this of the neighbouring
//! split — *"AC-520 owns what the class and the record MEAN, this criterion owns that the
//! ENTRY carries them … the two must not be collapsed into one lint rule"*.
//!
//! *(Corrected 2026-09-14: the table row above read "partly — see below" and nothing below
//! named the sub-clause, so the pointer dangled and an adversarial pass read it as an AND-clause
//! with no case and no ARMED note. It is neither unpinned nor armed — it is elsewhere, and the
//! row now says where.)*
//!
//! 🛑 **THE `anchor` CLAUSE IS NOT BUILT, AND A RECORD IS OWED.** AC-072's THEN names *"a declared
//! domain, source, anchor"*, and **no record gives `anchor` an on-disk form in `facets.toml`**.
//! Measured 2026-09-14: the string `anchor` occurs **once** in `spikes/p9/facets.toml`, on line 165,
//! inside a trailing **comment** (`# from toolchain:deploy_log — anchor: a deploy touching the
//! item's surface within the last 24 h`); there is no `anchor = …` key on any of the twenty
//! entries. The only definition of the word in the record set is the process-catalog proposal's
//! *"ordinal axes … with a written anchor per level"* (`references/2026-09-07-process-catalog-proposal.md`:111),
//! which is a property of a **triage rubric level**, not a registry key. AC-072's own eleven-case
//! *Verified by* enumerates **no anchor fixture**, which is the register agreeing that it has no
//! instrument for the clause. Implementing one would fail **every live entry** on a key no author
//! was ever told to write, so this stage records what is owed instead of deciding it
//! (`00-preamble.md`:55): **an amendment naming the `anchor` key's shape, or withdrawing the word,
//! is OWED to the register.** Nothing here is green for that clause; it is not counted as covered.
//!
//! 🛑 **THE DOMAIN SHAPE SET IS FOUR ON DISK AND THREE IN THE REGISTER, AND THE DIFFERENCE IS
//! RECORDED RATHER THAN RESOLVED.** The criterion names *"three shapes the loader recognises
//! explicitly, by name rather than by omission: boolean, ordinal, or set"*. Measured 2026-09-14
//! over `spikes/p9/facets.toml`: **four** live entries — `kind`, `cos`, `stage` and `role` — declare
//! a closed string vocabulary with **neither** `ordered` nor `set` and are none of the three. They
//! are the most load-bearing facets in the file (`stage` keys the whole selection function), so a rule
//! admitting only three would refuse the product registry outright. [`DomainShape::Enumerated`] is
//! therefore **recognised by name**, and the discrepancy is **OWED** to the register as an
//! amendment: either the three-shape list gains the plain closed vocabulary, or those four entries
//! are re-sourced. What this rule does **not** do is infer a shape from omission — an entry whose
//! `values` is neither a two-member boolean domain nor a homogeneous string vocabulary, and one
//! that declares `ordered` and `set` **together**, are both a shape with no name and both fail
//! load, which is the clause's actual bite (*"a fourth still fails load"*).
//!
//! 🛑 **`nullable` IS PINNED AT THE ONE GRAIN THE REGISTRY CAN EXPRESS.** The criterion says *"a
//! facet whose value may be **absent** declares `nullable = true`; absence is never inferred from
//! the domain"*, and its 2026-09-12 amendment (DECISIONS-2026-09-12 E1) says the rule is *"declaring
//! nullability **by name rather than by omission**"*. **Whether an arbitrary facet may abstain is
//! not a fact the entry carries**: `toolchain` and `change` are `derived` and declare no `nullable`,
//! while `recent_deploy` is `derived` and does — so `derived` neither implies nor denies abstention
//! and a rule keyed on it would be invented. What **is** carried, and what ADR-0089 decision 3
//! decides, is the boolean case: *"All seven are TRI-VALUED at the context-vector level … they keep
//! the two-valued DOMAIN and declare `nullable = true` beside it"*, and all seven live boolean
//! entries do [measured 2026-09-14]. So [`EntryShapeViolation::AbstainsWithoutNullable`] fires on a
//! **boolean-domain entry with no `nullable = true`** — which is F39 §4's measured defect exactly
//! (*"the registry line `values = [true, false]` alone is what turned FOUR EMPTY DIFFS INTO SIXTEEN
//! CONFIDENT BOOLEANS"*) — and the **general** case, a non-boolean facet that may abstain for a
//! reason the entry does not record, is **ARMED, NOT DISCHARGED**. A record naming how a non-boolean
//! facet declares that it may abstain is **OWED**.
//!
//! # What is ARMED for the reason both sibling rules are
//!
//! **The CLI half.** `redkiln lint` is `rk-cli`'s surface (**Step 7d**) and `.redkiln/facets.toml`
//! is the registry **Step 7e** derives. What Step 7b owes in their place is *"the lint BEHAVIOUR
//! under `cargo test`"* (§7b.14), which is what is discharged here — over the in-crate fixture
//! registries **and** over the shipped `spikes/p9/facets.toml`, which is the criterion's own
//! instrument for its first clause (*"must fail load **against the current registry**"*).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::registry::{NON_FACET_ROOTS, PredicateSites, Registry, SiteKind, Source};
use crate::toml_subset::Value;

/// A declared domain shape, recognised **explicitly by name**.
///
/// There is no `Unknown` variant and no `Other(String)`: a shape this enum cannot name is a
/// [`EntryShapeViolation::DomainShapeNotRecognised`], which is the criterion's *"a fourth still
/// fails load"*. Admitting it as a variant is how a loader silently accepts a member predicate it
/// has no semantics for — ADR-0089's stated reason for writing the set shape down at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DomainShape {
    /// `values = [true, false]` — the two-valued domain ADR-0089 decision 3 makes tri-valued at the
    /// context-vector level, which is why it is the one shape that must declare `nullable`.
    Boolean,
    /// `ordered = true` — the ordinal shape ADR-0069's comparison operators read.
    Ordinal,
    /// `set = true` — the shape `toolchain` and `change` already carry and the shape `surfaces`
    /// takes, *"whose members disjoin under ADR-0050 while distinct facets conjoin"*.
    Set,
    /// A closed string vocabulary with neither flag. **The register names three shapes and does not
    /// name this one**; four live entries carry it [measured 2026-09-14: `kind`, `cos`, `stage`,
    /// `role`]. See this module's header — an amendment is OWED and this stage decides nothing.
    Enumerated,
}

impl DomainShape {
    /// The shape's name, as the module header and the fixtures spell it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Boolean => "boolean",
            Self::Ordinal => "ordinal",
            Self::Set => "set",
            Self::Enumerated => "enumerated",
        }
    }
}

/// The three fields `source = "derived"` must carry **together**, and the three a `kernel` entry
/// must carry **none** of. One list, spelled once, so the two clauses cannot drift apart.
pub const PROVENANCE_FIELDS: [&str; 3] = ["deriver", "deriver_class", "deriver_record"];

/// One clause of AC-072 failing for one entry or one predicate site.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryShapeViolation {
    /// *"an unregistered facet can feed no row, so `binary_change` and `dynamism` cannot silently
    /// become selection inputs"* — the criterion's headline clause, over all three carriers.
    UnregisteredFacet {
        facet: String,
        /// `<key> of <owner>` — the site a reader goes and edits.
        site: String,
        /// The predicate, verbatim.
        predicate: String,
    },
    /// *"a live registry entry … with a declared domain"* — the entry declares no `values` at all.
    ///
    /// Separate from [`Self::DomainShapeNotRecognised`] on the criterion's own wording: deleting
    /// `values` *"fails this criterion's own declared-domain clause"*, which is a different failure
    /// from declaring a shape the loader does not recognise.
    NoDeclaredDomain { facet: String },
    /// *"the declared domain is one of three shapes the loader recognises explicitly, by name
    /// rather than by omission"* — and a fourth still fails load.
    DomainShapeNotRecognised { facet: String, found: String },
    /// *"`source` is one of the five closed tokens the loader recognises by name"* (ADR-0093
    /// decision 1). [`crate::registry::Source::Other`] carries the token through unchanged so that
    /// this rule — and not either lint, both of which are SCOPED by `source` — is what rejects it.
    SourceNotAToken { facet: String, found: String },
    /// *"all three required together"* — one violation per absent field, never one for the entry.
    ///
    /// Per field because the criterion says **together**: an entry missing two is missing two, and
    /// a rule that stopped at the first would let a fixture repaired in one respect stay red for a
    /// reason the fixture is not named for.
    DerivedMissingField { facet: String, field: String },
    /// *"`kernel` … (no `deriver`, no record — a `kernel` entry carrying either fails load)"*.
    ///
    /// **Three forms, not four.** AC-520's group 7 adds *"or claiming a record"*, which is a fact
    /// about the pinned **corpus** and not about the entry; it is
    /// [`crate::facet_provenance::ProvenanceViolation::KernelFacetCarriesProvenance`]'s fourth form
    /// and is deliberately absent here, because this rule reads no corpus.
    KernelEntryCarriesProvenance { facet: String, field: String },
    /// *"`declared` for `surfaces` … may key no `require` row until S-FACET scores it — a row keyed
    /// on it fails load"*.
    RequireKeyedOnDeclared { facet: String, when: String },
    /// *"a facet whose value may be absent declares `nullable = true`; absence is never inferred
    /// from the domain"*, at the one grain the entry carries — see the module header.
    AbstainsWithoutNullable { facet: String },
    /// *"facet names match `[a-z_][a-z0-9_]*`, ADR-0030's hyphenated spellings reading as their
    /// underscore forms"* (ADR-0103 decision 2). The hyphenated spelling is **normalised and then
    /// matched**, so `binary-change` is lawful and `Binary_Change` is not.
    FacetNameOutsideLexis { facet: String, normalised: String },
}

impl EntryShapeViolation {
    /// The facet the clause failed for.
    #[must_use]
    pub fn facet(&self) -> &str {
        match self {
            Self::UnregisteredFacet { facet, .. }
            | Self::NoDeclaredDomain { facet }
            | Self::DomainShapeNotRecognised { facet, .. }
            | Self::SourceNotAToken { facet, .. }
            | Self::DerivedMissingField { facet, .. }
            | Self::KernelEntryCarriesProvenance { facet, .. }
            | Self::RequireKeyedOnDeclared { facet, .. }
            | Self::AbstainsWithoutNullable { facet }
            | Self::FacetNameOutsideLexis { facet, .. } => facet,
        }
    }

    /// A stable tag for the clause, named after the fixture that pins it — the shape
    /// [`crate::facet_provenance::ProvenanceViolation::clause`] fixes, so a test can name the clause
    /// it expects without matching on message text.
    #[must_use]
    pub const fn clause(&self) -> &'static str {
        match self {
            Self::UnregisteredFacet { .. } => "unregistered_facet",
            Self::NoDeclaredDomain { .. } => "no_declared_domain",
            Self::DomainShapeNotRecognised { .. } => "domain_shape_is_a_fourth",
            Self::SourceNotAToken { .. } => "source_outside_the_five",
            Self::DerivedMissingField { .. } => "derived_without_deriver",
            Self::KernelEntryCarriesProvenance { .. } => "kernel_entry_carries_deriver",
            Self::RequireKeyedOnDeclared { .. } => "require_keyed_on_declared",
            Self::AbstainsWithoutNullable { .. } => "abstains_without_nullable",
            Self::FacetNameOutsideLexis { .. } => "facet_name_outside_lexis",
        }
    }
}

impl fmt::Display for EntryShapeViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnregisteredFacet {
                facet,
                site,
                predicate,
            } => write!(
                f,
                "{site}: {predicate:?} reads {facet:?}, which has no live registry entry in \
                 facets.toml — an unregistered facet can feed no row (AC-072)"
            ),
            Self::NoDeclaredDomain { facet } => write!(
                f,
                "facet {facet:?} declares no domain (AC-072 — a live entry carries a declared \
                 domain; `null` is not a member of one and deleting `values` is not how a facet \
                 declares it may abstain)"
            ),
            Self::DomainShapeNotRecognised { facet, found } => write!(
                f,
                "facet {facet:?}: the declared domain {found} is a shape the loader does not \
                 recognise by name (AC-072 — boolean, ordinal or set, declared explicitly rather \
                 than by omission, and a fourth fails load)"
            ),
            Self::SourceNotAToken { facet, found } => write!(
                f,
                "facet {facet:?}: source {found:?} is none of the five closed tokens \
                 human|llm|derived|kernel|declared (AC-072; ADR-0093 decision 1 — and both \
                 fail-closed lints are SCOPED by this key, so a token outside the five is read by \
                 neither of them)"
            ),
            Self::DerivedMissingField { facet, field } => write!(
                f,
                "facet {facet:?} is source = \"derived\" and carries no {field:?} — the three of \
                 {PROVENANCE_FIELDS:?} are required TOGETHER (AC-072 owns that the entry carries \
                 them; AC-520 owns what they mean)"
            ),
            Self::KernelEntryCarriesProvenance { facet, field } => write!(
                f,
                "facet {facet:?} is source = \"kernel\" and carries {field:?} — a kernel facet is a \
                 projection of the kernel's own state, with no deriver and no record, and an entry \
                 carrying either FAILS load (AC-072; ADR-0093 decision 1)"
            ),
            Self::RequireKeyedOnDeclared { facet, when } => write!(
                f,
                "require row {when:?} is keyed on the declared facet {facet:?}, which may key no \
                 require row until S-FACET scores it (AC-072; ADR-0093 decision 1)"
            ),
            Self::AbstainsWithoutNullable { facet } => write!(
                f,
                "facet {facet:?} declares the two-valued domain [true, false] and no \
                 `nullable = true` — ADR-0089 decision 3 makes every boolean facet tri-valued at \
                 the context-vector level, and nullability is declared BY NAME rather than by \
                 omission (AC-072 as amended 2026-09-12; F39 §4 measured four empty diffs becoming \
                 sixteen confident booleans)"
            ),
            Self::FacetNameOutsideLexis { facet, normalised } => write!(
                f,
                "facet {facet:?} reads as {normalised:?} and does not match [a-z_][a-z0-9_]* \
                 (AC-072; ADR-0103 decision 2 — a hyphenated spelling reads as its underscore \
                 form, and nothing else is admitted)"
            ),
        }
    }
}

/// What the rule found when nothing failed.
///
/// **`Ok(())` would be the wrong answer here for [`crate::facet_provenance::ProvenanceReport`]'s
/// reason**, one level down: an enumeration of **nothing** reads exactly like a clean registry. A
/// fixture whose `[facet.…]` header is misspelled carries zero entries, walks zero sites and
/// returns `Ok` — so every must-load assertion in this rule's fixture set is made against these
/// counts and not against the `Ok` alone.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntryShapeReport {
    /// Entries walked. Every entry, whatever its `source` — this rule is scoped by nothing.
    pub entries: usize,
    /// Predicate sites walked, across all three carriers.
    pub sites: usize,
    /// Facet reads resolved to a live entry, counted once per (site, root).
    pub reads_resolved: usize,
    /// The shape recognised for each entry, in byte order of facet name.
    pub shapes: BTreeMap<String, DomainShape>,
}

/// **AC-072's rule: fail closed on the registry ENTRY.**
///
/// `sites` are the `applicable_if`, `require.when` and `claimed_predicate` predicates read beside
/// the registry — the criterion's GIVEN names all three, and a rule reading only `require` rows
/// would pass a catalog whose `applicable_if` names an unregistered facet, which is `binary_change`'s
/// measured case.
///
/// # Errors
///
/// Every clause that failed, for every entry and every site, collected — never the first one only,
/// for the reason both sibling rules collect: one fixture per clause is only discriminating if a
/// fixture's one defect produces that clause **and no other**.
pub fn facet_registry_rule(
    registry: &Registry,
    sites: &PredicateSites,
) -> Result<EntryShapeReport, Vec<EntryShapeViolation>> {
    let mut out = Vec::new();
    let mut report = EntryShapeReport::default();

    // ---- THE ENTRY, clause by clause ----------------------------------------------------------
    // Every entry, whatever its `source`. This rule is scoped by nothing, which is the difference
    // between it and its two siblings and the reason a misspelled token has an owner at all.
    for facet in registry.facets() {
        report.entries += 1;
        let name = facet.name.clone();

        // (1) THE LEXIS (ADR-0103 decision 2). A NORMALISATION and not a ban: ADR-0030's
        //     hyphenated spellings read as their underscore forms, so the hyphen is removed BEFORE
        //     the match and a rule that rejected it outright would fail a lawful spelling.
        let normalised = normalised_name(&name);
        if !matches_lexis(&normalised) {
            out.push(EntryShapeViolation::FacetNameOutsideLexis {
                facet: name.clone(),
                normalised,
            });
        }

        // (2) THE DECLARED DOMAIN, and (3) its SHAPE. Two clauses and not one, on the criterion's
        //     own wording: "deleting `values` fails this criterion's own declared-domain clause",
        //     which is a different failure from a shape with no name.
        if facet.values.is_empty() {
            out.push(EntryShapeViolation::NoDeclaredDomain {
                facet: name.clone(),
            });
        } else {
            match shape_of(&facet.values, facet.ordered, facet.set) {
                Ok(shape) => {
                    report.shapes.insert(name.clone(), shape);
                    // (4) NULLABILITY, DECLARED BY NAME. Scoped to the boolean shape, which is the
                    //     one grain the entry carries: ADR-0089 decision 3 makes every boolean
                    //     facet tri-valued at the context-vector level while keeping the two-valued
                    //     DOMAIN, so a boolean entry with no `nullable = true` has declared its
                    //     abstention by omission — or not at all, which is F39 §4's sixteen
                    //     confident booleans. The general case is ARMED; see this module's header.
                    if shape == DomainShape::Boolean && !facet.nullable {
                        out.push(EntryShapeViolation::AbstainsWithoutNullable {
                            facet: name.clone(),
                        });
                    }
                }
                Err(found) => out.push(EntryShapeViolation::DomainShapeNotRecognised {
                    facet: name.clone(),
                    found,
                }),
            }
        }

        // (5) `source`, AND THE THREE TOKENS THAT CARRY A BAR WITH THEM.
        //
        // `Other` is checked FIRST and the arm returns: a token outside the five tells this rule
        // nothing about what the entry may carry, so asking whether it is a well-formed `derived`
        // entry would report a second clause for one defect and the fixture would stop
        // discriminating.
        match &facet.source {
            Source::Other(token) => {
                out.push(EntryShapeViolation::SourceNotAToken {
                    facet: name,
                    found: token.clone(),
                });
                continue;
            }
            Source::Derived => {
                // "all three required together" — one violation PER absent field, in the order
                // `PROVENANCE_FIELDS` spells them, because an entry missing two is missing two.
                for (field, present) in PROVENANCE_FIELDS.iter().zip([
                    facet.deriver.is_some(),
                    facet.deriver_class.is_some(),
                    facet.deriver_record.is_some(),
                ]) {
                    if !present {
                        out.push(EntryShapeViolation::DerivedMissingField {
                            facet: name.clone(),
                            field: (*field).to_owned(),
                        });
                    }
                }
                // AND NOTHING ABOUT WHAT THEY MEAN. `deriver_class`'s two tokens and
                // `deriver_record`'s resolution at the pin are AC-520's clauses; checking them here
                // would be the collapse the 2026-09-13 amendment forbids, spelled as a duplicate.
            }
            Source::Kernel => {
                // The mirror image, over the same three field names: "no `deriver`, no record — a
                // `kernel` entry carrying either fails load".
                for (field, present) in PROVENANCE_FIELDS.iter().zip([
                    facet.deriver.is_some(),
                    facet.deriver_class.is_some(),
                    facet.deriver_record.is_some(),
                ]) {
                    if present {
                        out.push(EntryShapeViolation::KernelEntryCarriesProvenance {
                            facet: name.clone(),
                            field: (*field).to_owned(),
                        });
                    }
                }
            }
            Source::Declared => {
                // "may key no `require` row until S-FACET scores it — a row keyed on it fails
                // load". Read off the `require` SITES rather than off the entry, because the clause
                // is about the PAIRING and a fixture that cannot express the pairing cannot pin it.
                for site in sites.sites() {
                    if site.kind == SiteKind::RequireWhen && site.keys.contains(&name) {
                        out.push(EntryShapeViolation::RequireKeyedOnDeclared {
                            facet: name.clone(),
                            when: site.predicate.clone(),
                        });
                    }
                }
            }
            Source::Human | Source::Llm => {}
        }
    }

    // ---- THE PREDICATE SITES: "an unregistered facet can feed no row" --------------------------
    // All three carriers AC-072's GIVEN names, and the roots are the ADR-0103 parser's — never a
    // substring scan, or `kind == project` would key `project` and a quoted literal would key
    // itself (`crate::registry::collect_paths`).
    let carved = non_facet_roots(sites);
    for site in sites.sites() {
        report.sites += 1;
        for root in &site.keys {
            if registry.facet(root).is_some() {
                report.reads_resolved += 1;
            } else if !carved.contains(root) {
                out.push(EntryShapeViolation::UnregisteredFacet {
                    facet: root.clone(),
                    site: site.site(),
                    predicate: site.predicate.clone(),
                });
            }
        }
    }

    if out.is_empty() { Ok(report) } else { Err(out) }
}

/// The declared domain shape, or the text a violation reports when there is no name for it.
///
/// **By name, never by omission**, which is the whole clause: `ordered` and `set` are read as
/// declarations, the boolean domain is read from its two members, and anything else — a mixed
/// `values` array, a `values` array that is neither, two shape flags at once — has no name.
fn shape_of(values: &[Value], ordered: bool, set: bool) -> Result<DomainShape, String> {
    // TWO FLAGS AT ONCE IS NOT THE FIRST OF THEM. "By name rather than by omission" cuts both ways:
    // an entry declaring two of the three has declared none of them, and a reader that took the
    // first flag it matched would silently pick one and carry on.
    if ordered && set {
        return Err("`ordered = true` and `set = true` together".to_owned());
    }
    if set {
        return Ok(DomainShape::Set);
    }
    if ordered {
        return Ok(DomainShape::Ordinal);
    }
    // The boolean domain, read from its MEMBERS: `values = [true, false]`, the two-valued domain
    // ADR-0089 decision 3 keeps while making the facet tri-valued at the context-vector level.
    let bools: Vec<bool> = values.iter().filter_map(Value::as_bool).collect();
    if bools.len() == values.len() {
        let distinct: BTreeSet<bool> = bools.iter().copied().collect();
        return if distinct.len() == 2 {
            Ok(DomainShape::Boolean)
        } else {
            Err(format!("{values:?}, which is not the two-valued domain"))
        };
    }
    // A closed string vocabulary. 🛑 The register names THREE shapes and does not name this one;
    // four live entries carry it and an amendment is OWED — this module's header has the
    // measurement. It is recognised **by name** here and is not inferred from omission: a `values`
    // array that is neither all-boolean nor all-string is a shape with no name and fails load,
    // which is the criterion's *"a fourth still fails load"*.
    if values.iter().all(|v| v.as_str().is_some()) {
        return Ok(DomainShape::Enumerated);
    }
    Err(format!(
        "{values:?}, whose members are neither a two-valued boolean domain nor a closed string \
         vocabulary"
    ))
}

/// `true` when the name is lawful under ADR-0103 decision 2, with ADR-0030's hyphenated spellings
/// read as their underscore forms.
#[must_use]
pub fn normalised_name(facet: &str) -> String {
    facet.replace('-', "_")
}

/// `[a-z_][a-z0-9_]*`, as a predicate rather than as a regex dependency.
#[must_use]
pub fn matches_lexis(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_lowercase() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Every predicate root that is lawfully not a facet: `facets.toml`:1's `item`, plus the step ids
/// the document declares. See [`crate::registry::PredicateSites`] — the `builtin` half of that line
/// has no enumeration in any record and is ARMED.
#[must_use]
fn non_facet_roots(sites: &PredicateSites) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = NON_FACET_ROOTS.iter().map(|s| (*s).to_owned()).collect();
    out.extend(sites.step_ids().iter().cloned());
    out
}
