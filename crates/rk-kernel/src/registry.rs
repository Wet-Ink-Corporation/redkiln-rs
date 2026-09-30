//! The facet registry the two fail-closed lints read, and the two artifacts each of them resolves.
//!
//! **This module is a loader, not a rule** — and that is still true now that all **three** rules
//! exist. *(Corrected 2026-09-14. This paragraph read "**it is not built here**" of AC-072 and gave
//! the absence of any `facet_registry`-named item in this file as the proof. AC-072 is built, at
//! [`crate::facet_registry`], and it is still not built HERE: nothing in this file is named for it,
//! so the checkpoint grep keyed on `facet_registry` reads that module and can never read a loader
//! as that rule.)* What is here is the shape all three rules need in order to disagree about the
//! same registry: [`Facet`] for the entry AC-072 reads, [`StabilityRecord`] for AC-071's record and
//! [`PrecisionRecord`] for AC-520's, and [`PredicateSites`] for the three carriers AC-072's GIVEN
//! names.
//!
//! # The three inputs, and which of them is real at Slice 2
//!
//! 1. **The registry** — `[facet.<name>]` entries, exactly `spikes/p9/facets.toml`'s shape.
//! 2. **The `require` rows keyed on a facet.** In production these live in the **pack**, in the
//!    `[[stage.<stage>.require]]` form `spikes/p9/packs/bug-standard.toml` uses, and the pack/lint
//!    wiring is Step 7e's. The fixtures here carry that verbatim form in the same file as the
//!    registry, because two of AC-520's clause groups — *"a record carrying 0 positives whose facet
//!    keys a `require` row"* and *"a `require` row keyed on a `declared` facet"* — are **about the
//!    pairing**, and a fixture that cannot express the pairing cannot pin the clause. Which facets
//!    a row keys is read by **parsing its `when` with this crate's own ADR-0103 parser** and taking
//!    the field paths, never by a substring search: `when = "'text' in change"` keys `change`, and
//!    a scan for the facet's name would also match the word inside a quoted literal.
//! 3. **The two records.** AC-071's stability record and AC-520's scored precision record are both
//!    read corpus-relative through `CORPUS_PIN` and never through a working tree (AC-464), and
//!    **neither is in the corpus yet** — the single pin move is a named act at the END of Step 7b
//!    (§7b.11). So both are modelled here from the fixture's own tables, the lints take them as
//!    arguments rather than opening a path, and **the corpus-resolution half of each rule is ARMED,
//!    NOT DISCHARGED**. That is the state §7b.15's ARMED table already records for AC-520's corpus
//!    half and AC-071's record half; this module does not quietly discharge it by reading a file
//!    that happens to sit in the working tree.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::predicate::{Expr, parse};
use crate::toml_subset::{self, Table, Value};

/// ADR-0093 decision 1's five closed `source` tokens, plus whatever else a registry says.
///
/// [`Source::Other`] is not tolerance: a token outside the five is **AC-072's** load failure, and
/// this type carries it through unchanged so that neither lint here can be read as having claimed
/// that criterion.
///
/// **THE HOLE THIS PARAGRAPH RECORDED IS CLOSED** *(2026-09-14)*. It said: both lints are scoped by
/// this enum — AC-071 on [`Source::Llm`], AC-520 on [`Source::Derived`] — so an entry whose `source`
/// is *misspelled* (`"Derived"`, `"llm "`) becomes `Other`, is read by **neither**, and *"nothing in
/// Slice 2 rejects it, because the criterion that does is one of AC-072's eleven must-fail cases and
/// AC-072 is not built"*. **AC-072 is built** — [`crate::facet_registry::facet_registry_rule`] — and
/// a token outside the five is its
/// [`crate::facet_registry::EntryShapeViolation::SourceNotAToken`].
///
/// The first two facts are unchanged and still load-bearing: this type carries the token through
/// **unchanged**, and neither lint widens to meet it, so the third rule is what rejects it and
/// neither of the other two can be read as having claimed that criterion. All three are asserted
/// together by `source_outside_the_five_is_ac072s_load_failure` below. The *absent*-key half of the
/// same hole is closed by [`facet_from`], because a reader may not fabricate a field the file does
/// not carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    Human,
    Llm,
    Derived,
    Kernel,
    Declared,
    Other(String),
}

impl Source {
    fn from_token(s: &str) -> Self {
        match s {
            "human" => Self::Human,
            "llm" => Self::Llm,
            "derived" => Self::Derived,
            "kernel" => Self::Kernel,
            "declared" => Self::Declared,
            other => Self::Other(other.to_owned()),
        }
    }

    /// The token as the registry spells it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Human => "human",
            Self::Llm => "llm",
            Self::Derived => "derived",
            Self::Kernel => "kernel",
            Self::Declared => "declared",
            Self::Other(s) => s,
        }
    }
}

/// One registry entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Facet {
    pub name: String,
    pub source: Source,
    pub values: Vec<Value>,
    pub ordered: bool,
    pub set: bool,
    pub nullable: bool,
    /// `deriver`, when the entry declares one.
    pub deriver: Option<String>,
    /// `deriver_class`, **as written**. AC-520's clause is that absence is never inferred and that
    /// a token outside `lexical`/`structural` fails load, so the raw string is kept rather than
    /// parsed into an enum that would have to invent a third variant for the bad case.
    pub deriver_class: Option<String>,
    /// `deriver_record`, a corpus-relative path resolved through `CORPUS_PIN` (AC-464).
    pub deriver_record: Option<String>,
}

impl Facet {
    /// `true` when `deriver_class` is one of AC-520's two tokens.
    #[must_use]
    pub fn declares_lexical(&self) -> bool {
        self.deriver_class.as_deref() == Some("lexical")
    }
}

/// A `require` row, with the facets its `when` reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequireRow {
    /// The `[stage.<stage>.require]` this row came from.
    pub stage: String,
    /// The row's predicate, verbatim.
    pub when: String,
    /// Every field path root the predicate reads, whether or not it is a registered facet. An
    /// unregistered name here is **AC-072's** failure and not either lint's.
    pub keys: BTreeSet<String>,
}

/// A registry plus the `require` rows read against it.
#[derive(Debug, Clone, PartialEq)]
pub struct Registry {
    facets: BTreeMap<String, Facet>,
    require_rows: Vec<RequireRow>,
}

/// AC-071's scored P1 stability record, one per facet, with the five clauses the executable
/// precondition at the head of the AC register names.
///
/// Every field is an `Option`: a record that omits a clause and a record that fails it are
/// different registries, and collapsing them would let `panel_verdict` disappear and read as a
/// pass — which is the residual-risk sentence AC-071 is written around.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct StabilityRecord {
    pub modal_required_set_identity: Option<f64>,
    pub n_items: Option<i64>,
    pub draws_per_item: Option<i64>,
    pub panel_verdict: Option<String>,
    pub corpus_sha: Option<String>,
}

/// One deriver's row in a scored precision record (SM-157).
#[derive(Debug, Clone, PartialEq)]
pub struct PrecisionRecord {
    /// The corpus-relative path the record was read from — what `deriver_record` names.
    pub path: String,
    pub deriver: String,
    /// Positive instances the deriver emitted. `0` is **NOT MEASURABLE**, which AC-520 says is
    /// *"neither a pass nor a failure"* and which still satisfies the exists-is-pinned-and-is-read
    /// clause (golden constraint 10, Halt 9).
    pub positives: i64,
    /// The **Wilson 95% interval**, `[lower, upper]`, as the record carries it.
    ///
    /// AC-520's THEN names two things the record must carry and not one: *"a scored precision
    /// record — which must exist for that deriver over a pinned corpus, **carrying a Wilson 95%
    /// interval and a positive-instance count**"*. This field existed nowhere until 2026-09-13 and
    /// a record with no interval passed the gate. It is an `Option` for the same reason every
    /// [`StabilityRecord`] field is: an omitted interval and a bad one are different registries,
    /// and the rule reports the omission as its own clause rather than defaulting it.
    ///
    /// A **NOT MEASURABLE** record is the one shape that lawfully carries none — it has no
    /// denominator, which is the whole of `data_migration`'s and `recent_deploy`'s finding
    /// (E-0089-1, E-0067-1) — so the clause is asserted only where a measurement was made.
    pub interval: Option<[f64; 2]>,
    /// The record's own NOT MEASURABLE marker, for a feed absent at the pin
    /// (`R-recent-deploy-feed-absent`) rather than a scored zero.
    pub not_measurable: bool,
}

impl PrecisionRecord {
    /// `true` when the record shows no positive instance, by either route.
    #[must_use]
    pub const fn has_no_positive_instance(&self) -> bool {
        self.not_measurable || self.positives == 0
    }

    /// `true` when AC-520's *"carrying a Wilson 95% interval"* half is unsatisfied.
    ///
    /// A NOT MEASURABLE record is exempt: it reports no rate, so an interval would be a number
    /// invented for a measurement that was not made.
    #[must_use]
    pub const fn lacks_required_interval(&self) -> bool {
        !self.not_measurable && self.interval.is_none()
    }
}

/// The pinned corpus, as far as the provenance rule can see it.
///
/// **This is a model, and the thing it models is not there yet.** At Slice 2 the record
/// `results/p1-arm-d-panel-scored.json` is not in the corpus — the single `CORPUS_PIN` move is the
/// named act at the END of Step 7b (§7b.11) — so the fixtures declare what the corpus would hold
/// and the rule reads that. **The corpus-resolution half of AC-520 is therefore ARMED, NOT
/// DISCHARGED**, exactly as §7b.15 already records it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PinnedCorpus {
    records: Vec<PrecisionRecord>,
}

impl PinnedCorpus {
    /// Build from rows.
    #[must_use]
    pub const fn new(records: Vec<PrecisionRecord>) -> Self {
        Self { records }
    }

    /// `true` when the pinned corpus holds anything at all at `path`.
    ///
    /// This is the *path* half of AC-520's sixth clause group, and it is deliberately separate from
    /// [`Self::record_for`]: *"a `deriver_record` naming a path absent from the pinned corpus"* and
    /// *"a named `deriver` with no scored record"* are two different failures over two different
    /// fixtures, and one predicate answering both would collapse them.
    #[must_use]
    pub fn holds(&self, path: &str) -> bool {
        self.records.iter().any(|r| r.path == path)
    }

    /// The row `path` carries for `deriver`, if it carries one.
    #[must_use]
    pub fn record_for(&self, path: &str, deriver: &str) -> Option<&PrecisionRecord> {
        self.records
            .iter()
            .find(|r| r.path == path && r.deriver == deriver)
    }

    /// Any row the corpus carries **in a facet's own name** — AC-520 clause group 7's fourth
    /// form, *"or claiming a record"*.
    ///
    /// The register writes group 7 with `or` across **four** forbidden things — a `deriver`, a
    /// `deriver_class`, a `deriver_record` **or claiming a record** — and E-0520-1's rule is that
    /// the enumeration decides, not the count. The first three are entry fields; the fourth is not,
    /// and had **no expressible instance** in this loader until 2026-09-13, so the arm that checks
    /// group 7 checked three of four forms.
    ///
    /// **What "claiming" means here, stated rather than assumed**: a `[[corpus_record]]` row whose
    /// `deriver` is the facet's own name. That is the one shape a registry can write in which a
    /// `kernel` facet — which by ADR-0093 decision 1 is a projection of the kernel's own state and
    /// has no deriver at all — asserts a scored record exists for it. A row keyed on some *other*
    /// deriver is not a claim by this facet and is not reported as one.
    #[must_use]
    pub fn claimed_by(&self, facet: &str) -> Option<&PrecisionRecord> {
        self.records.iter().find(|r| r.deriver == facet)
    }
}

/// Which of AC-072's **three** predicate carriers a predicate was read from.
///
/// The criterion's GIVEN names them by key: *"a facet name appearing in any `applicable_if`,
/// `require.when`, or `claimed_predicate`"*. They are three separate variants and not one "a
/// predicate" because the failure message has to name the site a reader can go and edit, and
/// because two of the three sit in files a `require` row never appears in — `applicable_if` is the
/// activity catalog's key (`spikes/p9/catalog/bisect.toml`:12) and `claimed_predicate` is a
/// `RequestDeviation` field ([`crate::select::step7`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SiteKind {
    /// An activity's `applicable_if`.
    ApplicableIf,
    /// A pack `[[stage.<stage>.require]]` row's `when`.
    RequireWhen,
    /// A `RequestDeviation`'s `claimed_predicate`.
    ClaimedPredicate,
}

impl SiteKind {
    /// The key as the file spells it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApplicableIf => "applicable_if",
            Self::RequireWhen => "require.when",
            Self::ClaimedPredicate => "claimed_predicate",
        }
    }
}

/// One predicate, with the field-path roots it reads.
///
/// The roots are taken by **parsing the predicate with this crate's own ADR-0103 parser** and
/// walking the expression, never by a substring search — [`collect_paths`]'s doc comment carries
/// the reason, and it is the same reason [`RequireRow`] does it that way: `when = "'text' in
/// change"` reads `change` and a scan for a facet's name would also match the word inside a quoted
/// literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateSite {
    pub kind: SiteKind,
    /// Where a reader goes to edit it: the activity id, the stage, or the deviation target.
    pub owner: String,
    /// The predicate, verbatim.
    pub predicate: String,
    /// Every field-path root the predicate reads, whether or not it is a registered facet.
    pub keys: BTreeSet<String>,
}

impl PredicateSite {
    /// `<kind> of <owner>`, the string a violation message names.
    #[must_use]
    pub fn site(&self) -> String {
        format!("{} of {:?}", self.kind.as_str(), self.owner)
    }
}

/// Every predicate site read beside a registry, plus the step ids that are lawfully **not** facets.
///
/// `facets.toml`'s own first line fixes the vocabulary a predicate identifier may come from:
/// *"Every predicate identifier must be a facet here, a step id, `rigor`, `item`, or a builtin."*
/// So AC-072's *"an unregistered facet can feed no row"* is asserted against the registry **minus**
/// that carve-out, and the carve-out has to be read from the document rather than assumed:
/// [`Self::step_ids`] is `[[step]] id` as the catalog writes it.
///
/// 🛑 **`builtin` HAS NO ENUMERATION IN ANY RECORD** [measured 2026-09-14: `facets.toml`:1 is the
/// only place the word appears in a rule, and it names no set]. This type therefore carries `item`
/// as a **named** constant ([`NON_FACET_ROOTS`]) and the step ids as **read** data, and the builtin
/// half is **ARMED, NOT DISCHARGED** — a record naming the builtin set is **OWED**. Nothing here
/// invents one, because a fabricated allow-list is exactly the shape that makes an unregistered
/// facet silently legal.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PredicateSites {
    sites: Vec<PredicateSite>,
    step_ids: BTreeSet<String>,
}

/// The non-facet predicate roots `facets.toml`:1 names **by name**, minus the two it names by
/// class: a step id is read from the document, and `rigor` is a registered facet already.
pub const NON_FACET_ROOTS: [&str; 1] = ["item"];

impl PredicateSites {
    /// Build from rows.
    #[must_use]
    pub const fn new(sites: Vec<PredicateSite>, step_ids: BTreeSet<String>) -> Self {
        Self { sites, step_ids }
    }

    /// Every site, in document order within each carrier.
    #[must_use]
    pub fn sites(&self) -> &[PredicateSite] {
        &self.sites
    }

    /// The `[[step]] id`s declared beside these predicates.
    #[must_use]
    pub const fn step_ids(&self) -> &BTreeSet<String> {
        &self.step_ids
    }
}

/// What stopped a registry load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// The file is not in [`crate::toml_subset`]'s subset.
    Toml(toml_subset::TomlError),
    /// A key whose type is not the one the shape needs.
    BadShape(String),
    /// A `require` row whose `when` does not parse under ADR-0103. The grammar is Slice 1's and a
    /// row that does not parse is a *grammar* failure, reported as one rather than silently keying
    /// nothing — a row that keys nothing passes both lints.
    BadPredicate { when: String, detail: String },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(e) => write!(f, "{e}"),
            Self::BadShape(d) => write!(f, "{d}"),
            Self::BadPredicate { when, detail } => {
                write!(f, "require row {when:?} does not parse: {detail}")
            }
        }
    }
}

impl std::error::Error for LoadError {}

impl Registry {
    /// Read a registry — and the `require` rows beside it — out of one document.
    ///
    /// # Errors
    ///
    /// [`LoadError`] for a file outside the TOML subset, an entry of the wrong shape, or a
    /// `require` row whose predicate does not parse.
    pub fn from_toml(src: &str) -> Result<Self, LoadError> {
        let doc = toml_subset::parse(src).map_err(LoadError::Toml)?;
        let mut facets = BTreeMap::new();
        if let Some(Value::Table(entries)) = doc.get("facet") {
            for (name, entry) in entries {
                let table = entry
                    .as_table()
                    .ok_or_else(|| LoadError::BadShape(format!("[facet.{name}] is not a table")))?;
                facets.insert(name.clone(), facet_from(name, table)?);
            }
        }
        let require_rows = require_rows_from(&doc)?;
        Ok(Self {
            facets,
            require_rows,
        })
    }

    /// Every entry, in byte order.
    pub fn facets(&self) -> impl Iterator<Item = &Facet> {
        self.facets.values()
    }

    /// **A repository's checked-in declaration of its `source = "declared"` facets** (R5 =
    /// ADR-0195 decision 5, proposed; the file is `.redkiln/declared.toml`, named by that record's
    /// dated note): one top-level `<facet> = "<value>"` per declared, single-valued facet, each
    /// value a member of the facet's declared domain. A facet the file does not name stays
    /// `null` on the item (the abstention); the file never supplies a `human`, `derived` or
    /// `kernel` facet, and a set-valued declared facet (`surfaces`) keeps ADR-0089's own
    /// declaration map and is not read here. *(dated 2026-09-29, s8g build, package
    /// S8G-P8-TAG-RELEASE-AND-MOVES.)*
    ///
    /// # Errors
    /// A message naming the key that is not a declared single-valued facet, or the value outside
    /// its domain, or the file outside the TOML subset.
    pub fn declared_values(&self, src: &str) -> Result<BTreeMap<String, String>, String> {
        let doc = toml_subset::parse(src).map_err(|e| format!("{e:?}"))?;
        let mut out = BTreeMap::new();
        for (key, value) in &doc {
            let Some(facet) = self.facets.get(key) else {
                return Err(format!(
                    "`{key}` is not a facet `facets.toml` declares, so it cannot be declared"
                ));
            };
            if facet.source != Source::Declared || facet.set {
                return Err(format!(
                    "`{key}` is not a single-valued `source = \"declared\"` facet (it is `{}`{}), \
                     so the repository's declaration may not supply it (ADR-0093 decision 1)",
                    facet.source.as_str(),
                    if facet.set { ", set-valued" } else { "" }
                ));
            }
            let Some(text) = value.as_str() else {
                return Err(format!("`{key}` is not a string"));
            };
            if !facet.values.iter().any(|v| v.as_str() == Some(text)) {
                return Err(format!(
                    "`{key} = \"{text}\"` is outside the facet's declared domain {:?}",
                    facet
                        .values
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                ));
            }
            out.insert(key.clone(), text.to_owned());
        }
        Ok(out)
    }

    /// One entry by name.
    #[must_use]
    pub fn facet(&self, name: &str) -> Option<&Facet> {
        self.facets.get(name)
    }

    /// Every `require` row read beside this registry.
    #[must_use]
    pub fn require_rows(&self) -> &[RequireRow] {
        &self.require_rows
    }

    /// `true` when some `require` row's predicate reads `facet`.
    #[must_use]
    pub fn keys_a_require_row(&self, facet: &str) -> bool {
        self.require_rows.iter().any(|r| r.keys.contains(facet))
    }

    /// Read the `[stability.<facet>]` tables beside this registry.
    ///
    /// In production this is a corpus-relative JSON read through `CORPUS_PIN` (AC-464) that **does
    /// not exist**: AC-071's record has no producer and no lawful path, §7b.2 says it must be
    /// transcribed from F36's frozen output rather than regenerated, and its register amendment is
    /// OWED. The fixture table is what a lint test can pin today.
    ///
    /// # Errors
    ///
    /// [`LoadError`] for a file outside the subset or a record of the wrong shape.
    pub fn stability_records_from_str(
        src: &str,
    ) -> Result<BTreeMap<String, StabilityRecord>, LoadError> {
        let doc = toml_subset::parse(src).map_err(LoadError::Toml)?;
        let mut out = BTreeMap::new();
        if let Some(Value::Table(entries)) = doc.get("stability") {
            for (name, entry) in entries {
                let table = entry.as_table().ok_or_else(|| {
                    LoadError::BadShape(format!("[stability.{name}] is not a table"))
                })?;
                out.insert(
                    name.clone(),
                    StabilityRecord {
                        modal_required_set_identity: table
                            .get("modal_required_set_identity")
                            .and_then(Value::as_f64),
                        n_items: table.get("n_items").and_then(Value::as_int),
                        draws_per_item: table.get("draws_per_item").and_then(Value::as_int),
                        panel_verdict: table
                            .get("panel_verdict")
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned),
                        corpus_sha: table
                            .get("corpus_sha")
                            .and_then(Value::as_str)
                            .map(ToOwned::to_owned),
                    },
                );
            }
        }
        Ok(out)
    }

    /// Read the `[[corpus_record]]` rows beside this registry — the pinned corpus the provenance
    /// rule resolves `deriver_record` against.
    ///
    /// # Errors
    ///
    /// [`LoadError`] for a file outside the subset or a row missing `path` or `deriver`.
    pub fn pinned_corpus_from_str(src: &str) -> Result<PinnedCorpus, LoadError> {
        let doc = toml_subset::parse(src).map_err(LoadError::Toml)?;
        let mut records = Vec::new();
        if let Some(Value::Array(rows)) = doc.get("corpus_record") {
            for row in rows {
                let table = row.as_table().ok_or_else(|| {
                    LoadError::BadShape("[[corpus_record]] is not a table".to_owned())
                })?;
                let path = table
                    .get("path")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        LoadError::BadShape("[[corpus_record]] has no `path`".to_owned())
                    })?
                    .to_owned();
                let deriver = table
                    .get("deriver")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        LoadError::BadShape(format!("[[corpus_record]] {path:?} has no `deriver`"))
                    })?
                    .to_owned();
                // `interval = [lower, upper]`, exactly two floats, or nothing. A row carrying one
                // of the wrong shape is a load failure rather than a silently absent interval:
                // the rule's clause is about a record that does not CARRY one, and a malformed one
                // read as absent would report the wrong clause.
                let interval = match table.get("interval") {
                    None => None,
                    Some(Value::Array(items)) if items.len() == 2 => {
                        let lower = items[0].as_f64();
                        let upper = items[1].as_f64();
                        match (lower, upper) {
                            (Some(l), Some(u)) => Some([l, u]),
                            _ => {
                                return Err(LoadError::BadShape(format!(
                                    "[[corpus_record]] {path:?} interval is not two numbers"
                                )));
                            }
                        }
                    }
                    Some(_) => {
                        return Err(LoadError::BadShape(format!(
                            "[[corpus_record]] {path:?} interval is not a two-element array \
                             [lower, upper] — AC-520's THEN names a Wilson 95% interval"
                        )));
                    }
                };
                records.push(PrecisionRecord {
                    path,
                    deriver,
                    positives: table.get("positives").and_then(Value::as_int).unwrap_or(0),
                    interval,
                    not_measurable: table
                        .get("not_measurable")
                        .and_then(Value::as_bool)
                        .unwrap_or(false),
                });
            }
        }
        Ok(PinnedCorpus::new(records))
    }
}

impl Registry {
    /// Read every predicate site AC-072's GIVEN names out of one document.
    ///
    /// Three carriers, three keys, in the shapes the corpus actually writes them:
    ///
    /// - `[[stage.<stage>.require]]` rows' `when` — the pack shape
    ///   (`spikes/p9/packs/bug-standard.toml`:63-64), read by [`require_rows_from`] so the two
    ///   readings of a `require` row can never disagree;
    /// - `[activity]`'s `applicable_if` — the catalog shape (`spikes/p9/catalog/bisect.toml`:4-12),
    ///   owned by that table's own `id`;
    /// - `[[deviation]]`'s `claimed_predicate` — a `RequestDeviation` written down, owned by its
    ///   `target`. There is no on-disk carrier for a deviation in the corpus, because a deviation
    ///   is a runtime request; the fixture form is stated here rather than inferred so a reader can
    ///   see that it is this loader's shape and not a record's.
    ///
    /// `[[step]] id` is collected beside them — not a predicate, but the carve-out `facets.toml`:1
    /// grants, and a rule that asserts *"an unregistered facet can feed no row"* has to know it.
    ///
    /// # Errors
    ///
    /// [`LoadError`] for a file outside the subset, a predicate of the wrong type, or a predicate
    /// that does not parse under ADR-0103. **A predicate that does not parse is a grammar failure
    /// reported as one**, never a site that silently reads nothing: a site that reads nothing
    /// passes every rule here.
    pub fn predicate_sites_from_str(src: &str) -> Result<PredicateSites, LoadError> {
        let doc = toml_subset::parse(src).map_err(LoadError::Toml)?;
        let mut sites = Vec::new();
        for row in require_rows_from(&doc)? {
            sites.push(PredicateSite {
                kind: SiteKind::RequireWhen,
                owner: row.stage,
                predicate: row.when,
                keys: row.keys,
            });
        }
        if let Some(Value::Table(activity)) = doc.get("activity")
            && let Some(when) = activity.get("applicable_if")
        {
            let owner = activity
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("activity");
            sites.push(one_site(SiteKind::ApplicableIf, owner, when)?);
        }
        if let Some(Value::Array(rows)) = doc.get("deviation") {
            for row in rows {
                let Some(table) = row.as_table() else {
                    return Err(LoadError::BadShape(
                        "[[deviation]] is not a table".to_owned(),
                    ));
                };
                let Some(claimed) = table.get("claimed_predicate") else {
                    continue;
                };
                let owner = table.get("target").and_then(Value::as_str).unwrap_or("?");
                sites.push(one_site(SiteKind::ClaimedPredicate, owner, claimed)?);
            }
        }
        let mut step_ids = BTreeSet::new();
        if let Some(Value::Array(rows)) = doc.get("step") {
            for row in rows {
                if let Some(id) = row
                    .as_table()
                    .and_then(|t| t.get("id"))
                    .and_then(Value::as_str)
                {
                    step_ids.insert(id.to_owned());
                }
            }
        }
        Ok(PredicateSites::new(sites, step_ids))
    }
}

/// One predicate value, typed, parsed and walked.
fn one_site(kind: SiteKind, owner: &str, value: &Value) -> Result<PredicateSite, LoadError> {
    let Some(text) = value.as_str() else {
        return Err(LoadError::BadShape(format!(
            "{} of {owner:?} is {value:?}, not a string",
            kind.as_str()
        )));
    };
    let parsed = parse(text).map_err(|e| LoadError::BadPredicate {
        when: text.to_owned(),
        detail: e.to_string(),
    })?;
    let mut keys = BTreeSet::new();
    collect_paths(&parsed.expr, &mut keys);
    Ok(PredicateSite {
        kind,
        owner: owner.to_owned(),
        predicate: text.to_owned(),
        keys,
    })
}

fn facet_from(name: &str, table: &Table) -> Result<Facet, LoadError> {
    let text = |key: &str| -> Result<Option<String>, LoadError> {
        match table.get(key) {
            None => Ok(None),
            Some(Value::Str(s)) => Ok(Some(s.clone())),
            Some(other) => Err(LoadError::BadShape(format!(
                "[facet.{name}] {key} is {other:?}, not a string"
            ))),
        }
    };
    let flag = |key: &str| table.get(key).and_then(Value::as_bool).unwrap_or(false);
    // AN ABSENT `source` IS A LOAD FAILURE, NOT AN EMPTY ONE. Until 2026-09-13 this line read
    // `map_or_else(|| Source::Other(String::new()), …)`, which fabricated a token the file does not
    // carry — and because both lints are scoped by `source` (AC-071 on `llm`, AC-520 on `derived`),
    // an entry that simply omitted the key was skipped by BOTH fail-closed gates. Measured against
    // `positive_companion.toml`: deleting its `source = "derived"` line made the provenance rule
    // return `Ok`. The loader refusing to invent a value it was not given is not AC-072's lint —
    // that criterion is a third rule with its own eleven-case fixture set and is NOT built here —
    // it is this reader declining to hand the rules a registry that does not exist on disk.
    let source = Source::from_token(&text("source")?.ok_or_else(|| {
        LoadError::BadShape(format!(
            "[facet.{name}] declares no `source` — ADR-0093 decision 1's five tokens are a closed \
             set and an absent one is not an empty one; both fail-closed lints are SCOPED by this \
             key, so an entry without it is read by neither"
        ))
    })?);
    Ok(Facet {
        name: name.to_owned(),
        source,
        values: table
            .get("values")
            .and_then(Value::as_array)
            .map(<[Value]>::to_vec)
            .unwrap_or_default(),
        ordered: flag("ordered"),
        set: flag("set"),
        nullable: flag("nullable"),
        deriver: text("deriver")?,
        deriver_class: text("deriver_class")?,
        deriver_record: text("deriver_record")?,
    })
}

fn require_rows_from(doc: &Table) -> Result<Vec<RequireRow>, LoadError> {
    let mut out = Vec::new();
    let Some(Value::Table(stages)) = doc.get("stage") else {
        return Ok(out);
    };
    for (stage, body) in stages {
        let Some(Value::Array(rows)) = body.as_table().and_then(|t| t.get("require")) else {
            continue;
        };
        for row in rows {
            let Some(table) = row.as_table() else {
                continue;
            };
            let Some(when) = table.get("when").and_then(Value::as_str) else {
                continue;
            };
            let parsed = parse(when).map_err(|e| LoadError::BadPredicate {
                when: when.to_owned(),
                detail: e.to_string(),
            })?;
            let mut keys = BTreeSet::new();
            collect_paths(&parsed.expr, &mut keys);
            out.push(RequireRow {
                stage: stage.clone(),
                when: when.to_owned(),
                keys,
            });
        }
    }
    Ok(out)
}

/// Every field-path **root** the expression reads.
///
/// The root, not the whole dotted path: a row keyed on `toolchain.version` keys the `toolchain`
/// facet. Bare identifiers in **value** position are [`crate::predicate::Literal::Bare`] and never
/// reach here, which is the literal-versus-field rule doing the work a substring search cannot:
/// `kind == project` keys `kind` and not `project`.
fn collect_paths(expr: &Expr, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Path(p) => {
            let root = p.split('.').next().unwrap_or(p);
            out.insert(root.to_owned());
        }
        Expr::Or(l, r) | Expr::And(l, r) => {
            collect_paths(l, out);
            collect_paths(r, out);
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => collect_paths(inner, out),
        Expr::Cmp { left, right, .. } => {
            collect_paths(left, out);
            collect_paths(right, out);
        }
        Expr::In { left, list } => {
            collect_paths(left, out);
            collect_paths(list, out);
        }
        Expr::Member { literal, set } => {
            collect_paths(literal, out);
            collect_paths(set, out);
        }
        Expr::List(items) => {
            for item in items {
                collect_paths(item, out);
            }
        }
        Expr::Quant { source, body, .. } => {
            collect_paths(source, out);
            collect_paths(body, out);
        }
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{LoadError, Registry, Source};

    /// The reader may not fabricate a `source` the file does not carry.
    ///
    /// Both fail-closed lints are **scoped** by that key — AC-071 on `llm`, AC-520 on `derived` —
    /// so an entry without it was read by neither. Until 2026-09-13 [`super::facet_from`] supplied
    /// `Source::Other("")` for an absent key and this registry loaded clean.
    #[test]
    fn a_facet_entry_without_a_source_fails_load() {
        let derived = "[facet.toolchain]\nsource = \"derived\"\nderiver = \"R-x\"\n\
                       deriver_class = \"structural\"\nderiver_record = \"results/x.json\"\n";
        // The control: the same entry WITH the key parses, so the case below fails for the
        // omission and not for anything else in the shape.
        let ok = Registry::from_toml(derived).expect("the complete entry loads");
        assert_eq!(
            ok.facet("toolchain").expect("present").source,
            Source::Derived
        );

        let without = derived.replace("source = \"derived\"\n", "");
        assert!(!without.contains("source"), "the anchor removed the key");
        let err = Registry::from_toml(&without).expect_err("an entry with no `source` fails load");
        let LoadError::BadShape(detail) = &err else {
            panic!("expected BadShape, got {err:?}");
        };
        assert!(
            detail.contains("toolchain") && detail.contains("source"),
            "{detail}"
        );
    }

    /// **THE HOLE, CLOSED — and this test is the rewrite it asked for.**
    ///
    /// *(Renamed 2026-09-14 from `source_outside_the_five_escapes_both_gates_until_ac072`. That
    /// test asserted the ESCAPE and said in its own message that the assertion "must be rewritten
    /// as AC-072's must-fail case when it is [built]". AC-072 is built —
    /// [`crate::facet_registry::facet_registry_rule`] — and this is that rewrite, not a new test
    /// beside it.)*
    ///
    /// A `source` token outside ADR-0093 decision 1's five is *"AC-072's load failure"* — the
    /// register's own words, and one of its eleven must-fail cases. **Both halves are asserted
    /// here and the pairing is the point**: the loader still carries the token through unchanged,
    /// both fail-closed lints are still SCOPED by `source` and so still do not see it, and the
    /// third rule is what rejects it. A reader who meets `Source::Other` needs all three facts, and
    /// the first two on their own were the hole.
    #[test]
    fn source_outside_the_five_is_ac072s_load_failure() {
        use crate::facet_provenance::facet_provenance_rule;
        use crate::facet_registry::facet_registry_rule;
        use crate::registry::{PinnedCorpus, PredicateSites};

        let misspelled = "[facet.toolchain]\nvalues = [\"node\", \"rust\"]\nset = true\n\
                          source = \"Derived\"\n";
        let registry = Registry::from_toml(misspelled).expect("the token is carried, not rejected");
        assert_eq!(
            registry.facet("toolchain").expect("present").source,
            Source::Other("Derived".to_owned()),
            "the loader carries the token through unchanged, so the rule that rejects it is a RULE \
             and not a parse accident"
        );
        assert_eq!(
            facet_provenance_rule(&registry, &PinnedCorpus::default()).map(|r| r.derived_facets),
            Ok(0),
            "AC-520's gate still does not see it — its GIVEN is `source = \"derived\"` exactly, and \
             widening it would be the collapse the register forbids"
        );

        // AND AC-072 DOES. One violation, of its own clause, naming the token as written.
        let errs = facet_registry_rule(&registry, &PredicateSites::default())
            .expect_err("a token outside the five fails load");
        assert_eq!(errs.len(), 1, "exactly one clause fails: {errs:?}");
        assert_eq!(errs[0].clause(), "source_outside_the_five");
        assert_eq!(errs[0].facet(), "toolchain");
        assert!(errs[0].to_string().contains("Derived"), "{}", errs[0]);

        // The control on it: the same entry with the token spelled correctly LOADS, so the failure
        // is the token and nothing else about the entry.
        let correct = misspelled.replace("\"Derived\"", "\"human\"");
        assert!(
            facet_registry_rule(
                &Registry::from_toml(&correct).expect("loads"),
                &PredicateSites::default()
            )
            .is_ok()
        );
    }
}
