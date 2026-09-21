//! The lift: legacy files in, canonical artifacts out (`s7d` §7d.11 step (4)).
//!
//! Every legacy file under the target's `.bklg/` and `.redkiln/` is read through [`crate::legacy`]
//! — the only `yaml_serde` users — and lands as one of:
//!
//! * a **typed** value, for the six [`rk_types::companion::CompanionKind`] variants and the item
//!   records, validated in process against the schema
//!   [`rk_types::companion::write_companion`] returns (that function is a schema lookup, not a
//!   writer) and serialised through [`rk_store::canon::canonical`];
//! * **opaque prose**, for the eight [`rk_types::companion::PHASE_2_KINDS`] (AC-143; the treatment
//!   is ADR-0075 clause 6's), and for a free-form gate result or a diff-prose section, each with a
//!   migration-loss marker (AC-477's carry, AC-475's tripwire).
//!
//! # The two signature literals are dropped, and that is not a loss
//!
//! `template_sig` and `rendered_sig` are read here — under [`crate::legacy`], the one allowlisted
//! site — and **deliberately not carried**. AC-471 says no schema declares a template- or
//! rendered-signature field and AC-474 calls the break clean, so their absence from the output is
//! the point of the migration, not something the report should count as lost. An item record whose
//! only difference from its legacy form is those two keys lifts **clean**, which is what
//! Checkpoint 7d's exact `spec` counts require: one clean `spec.md` and one diff-prose one.
//!
//! # `DirOrder` is a seam, not a toggle
//!
//! AC-449 sweeps directory-read order as a nuisance parameter. A spawned process cannot be made to
//! read in a different order without a flag or an environment variable, and AC-474 forbids both —
//! so the read-order arm drives [`run`] **in process** through [`DirOrder`], once with
//! [`Natural`] and once with a reversing implementation the test supplies, and compares tree and
//! report digests.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::report::Report;

/// The directory-read ordering seam AC-449's read-order arm drives.
pub trait DirOrder {
    /// Order one directory's entries in place.
    fn order(&mut self, entries: &mut Vec<PathBuf>);
}

/// The production ordering: sorted, so a run is reproducible without reference to the filesystem's
/// own listing order (AC-002).
#[derive(Debug, Default, Clone, Copy)]
pub struct Natural;

impl DirOrder for Natural {
    fn order(&mut self, entries: &mut Vec<PathBuf>) {
        entries.sort();
    }
}

/// The schema-version stamp every migrated artifact carries (AC-471's write half).
///
/// The house spelling is a [`rk_types::typed::SchemaId`] string under the JSON key
/// `schema_version` — `rk-cli`'s answer-record payload stamps one the same way. No `rk-types`
/// artifact declares the field as a struct member [measured 2026-09-21: `grep -rn schema_version
/// crates/rk-types/src` → 0], which is the absence `rk-kernel`'s memo module already records.
#[must_use]
pub fn schema_version_for(kind: &str) -> rk_types::typed::SchemaId {
    rk_types::typed::SchemaId::new(format!("{kind}/v1"))
}

/// One legacy file, lifted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiftedFile {
    /// The legacy file, relative to the target root.
    pub source: PathBuf,
    /// The report's `kind` column for this file (§7d.11 step (4)).
    pub kind: String,
    /// Whether the value landed **typed** or as **opaque prose** —
    /// `adr0075_six_typed_kinds_eight_opaque` reads exactly this, six against eight.
    pub typed: bool,
    /// Which of the three counts this file contributes to.
    pub outcome: crate::report::Outcome,
    /// The artifact document: `schema_version`, the kind, the value (typed or opaque) and the
    /// loss markers when there are any. **No `*_sig` key** (AC-471).
    pub value: serde_json::Value,
    /// What had no home, carried opaque beside the value.
    pub losses: Vec<crate::legacy::LossMarker>,
}

/// What one lift produced: the artifacts to write, and the report that describes them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lifted {
    /// Every legacy file read, in the order [`DirOrder`] yielded them.
    pub files: Vec<LiftedFile>,
    /// Artifact bytes by **destination** path, relative to the target root, already canonical
    /// ([`rk_store::canon::canonical`]) — so a write is a write and never a second serialisation.
    pub artifacts: BTreeMap<PathBuf, Vec<u8>>,
    /// The differential report (AC-448).
    pub report: Report,
}

/// Why a lift could not run to completion.
///
/// A per-file failure is **not** one of these — it is a `failed` count in the report. These are the
/// conditions under which no report can be produced at all.
#[derive(Debug)]
pub enum LiftError {
    /// The target carries neither `.bklg/` nor `.redkiln/` — there is nothing to migrate, and a
    /// report over nothing would read as a successful migration of an empty corpus.
    NothingToMigrate { target: PathBuf },
    /// A directory under the target could not be read.
    Unreadable { path: PathBuf, detail: String },
    /// The report could not be built or did not validate.
    Report(crate::report::ReportError),
}

/// Lift the target's legacy tree.
///
/// Reads only; writes nothing. The write phase is [`crate::run_contract`]'s, which is what makes
/// `--dry-run` a matter of stopping rather than of suppressing.
///
/// # Errors
/// [`LiftError`], as documented on its variants.
pub fn run(target: &Path, order: &mut dyn DirOrder) -> Result<Lifted, LiftError> {
    let roots = [target.join(".bklg"), target.join(".redkiln")];
    if !roots.iter().any(|r| r.is_dir()) {
        return Err(LiftError::NothingToMigrate {
            target: target.to_path_buf(),
        });
    }

    let mut sources = Vec::new();
    for root in &roots {
        if root.is_dir() {
            walk(root, order, &mut sources)?;
        }
    }

    let mut files = Vec::new();
    let mut artifacts = BTreeMap::new();
    let mut outcomes: BTreeMap<String, Vec<crate::report::Outcome>> = BTreeMap::new();

    for abs in &sources {
        let rel = abs
            .strip_prefix(target)
            .unwrap_or(abs.as_path())
            .to_path_buf();
        let lifted = lift_one(abs, &rel);
        outcomes
            .entry(lifted.kind.clone())
            .or_default()
            .push(lifted.outcome);
        if lifted.outcome != crate::report::Outcome::Failed {
            let bytes =
                rk_store::canon::canonical(&lifted.value).map_err(|e| LiftError::Unreadable {
                    path: rel.clone(),
                    detail: format!("the lifted value will not canonicalise: {e}"),
                })?;
            artifacts.insert(destination(&rel), bytes);
        }
        files.push(lifted);
    }

    let report = crate::report::build(&outcomes).map_err(LiftError::Report)?;
    crate::report::validate(&report).map_err(LiftError::Report)?;

    Ok(Lifted {
        files,
        artifacts,
        report,
    })
}

/// Walk one root, ordering every directory's entries through `order`.
fn walk(root: &Path, order: &mut dyn DirOrder, out: &mut Vec<PathBuf>) -> Result<(), LiftError> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(root)
        .map_err(|e| LiftError::Unreadable {
            path: root.to_path_buf(),
            detail: e.to_string(),
        })?
        .map(|e| {
            e.map(|e| e.path()).map_err(|e| LiftError::Unreadable {
                path: root.to_path_buf(),
                detail: e.to_string(),
            })
        })
        .collect::<Result<_, _>>()?;

    // The seam AC-449's read-order arm drives. It orders EVERY directory, not just the top one, so
    // a reversing implementation genuinely reverses the walk.
    order.order(&mut entries);

    for entry in entries {
        if entry.is_dir() {
            walk(&entry, order, out)?;
        } else {
            out.push(entry);
        }
    }
    Ok(())
}

/// The report's `kind` column for one legacy path (§7d.11 step (4)).
///
/// The file stem IS the kind — `_review.md` → `_review`, `ledger.md` → `ledger` (not `_ledger`),
/// `design-review.md` → `design-review` — with one rule beside it: anything under
/// `.redkiln/processes/` is a `process`, whatever its stem.
fn kind_of(rel: &Path) -> String {
    let in_processes = rel
        .parent()
        .and_then(Path::file_name)
        .is_some_and(|n| n == crate::legacy::config_yaml::PROCESSES_DIR);
    if in_processes {
        return "process".to_owned();
    }
    rel.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Where a lifted artifact lands: the source path with a `.json` extension.
fn destination(rel: &Path) -> PathBuf {
    rel.with_extension("json")
}

/// Which kinds are item records rather than companions.
const ITEM_KINDS: [&str; 4] = ["initiative", "project", "story", "spec"];

fn lift_one(abs: &Path, rel: &Path) -> LiftedFile {
    let kind = kind_of(rel);
    let src = match std::fs::read_to_string(abs) {
        Ok(s) => s,
        Err(e) => return failed(rel, &kind, &format!("unreadable: {e}")),
    };
    match lift_value(rel, &kind, &src) {
        Ok(f) => f,
        Err(e) => failed(rel, &kind, &format!("{e:?}")),
    }
}

fn failed(rel: &Path, kind: &str, detail: &str) -> LiftedFile {
    LiftedFile {
        source: rel.to_path_buf(),
        kind: kind.to_owned(),
        typed: false,
        outcome: crate::report::Outcome::Failed,
        value: serde_json::json!({
            "schema_version": schema_version_for(kind).as_str(),
            "kind": kind,
            "source": rel.to_string_lossy().replace('\\', "/"),
            "failed": detail,
        }),
        losses: Vec::new(),
    }
}

/// Assemble the artifact document for one legacy file.
///
/// The envelope is the same in every case: `schema_version`, `kind`, `source`, exactly one of
/// `typed` / `opaque`, and `migration_loss` when there is any. **No key ends `_sig`** — that is
/// AC-471's write half, and `migrated_artifacts_carry_schema_version_and_no_sig` reads it.
fn lift_value(rel: &Path, kind: &str, src: &str) -> Result<LiftedFile, crate::legacy::LegacyError> {
    use crate::legacy::{config_yaml, fence, frontmatter, ledger, review};
    use crate::report::Outcome;

    let mut losses: Vec<crate::legacy::LossMarker> = Vec::new();
    let (typed, value): (bool, serde_json::Value) = if ITEM_KINDS.contains(&kind) {
        let fm = frontmatter::parse(rel, src)?;
        losses.extend(frontmatter::opaque_sections(&fm.body));
        // The two signature keys are READ and dropped: the clean break, not a loss (AC-471).
        let carried: serde_json::Map<String, serde_json::Value> = fm
            .keys
            .iter()
            .filter(|(k, _)| {
                k.as_str() != frontmatter::TEMPLATE_SIG_KEY
                    && k.as_str() != frontmatter::RENDERED_SIG_KEY
            })
            .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
            .collect();
        let mut obj = carried;
        obj.insert(
            "body".to_owned(),
            serde_json::Value::String(fm.body.trim().to_owned()),
        );
        (true, serde_json::Value::Object(obj))
    } else if kind == "report" {
        // A free-form gate result: opaque, never parsed into a typed verdict.
        let marker = review::parse_free_form_gate_result(rel, src)?;
        let prose = marker.opaque.clone();
        losses.push(marker);
        (false, serde_json::Value::String(prose))
    } else if kind == "config" {
        (true, config_yaml::parse_config(rel, src)?.keys)
    } else if kind == "process" {
        let p = config_yaml::parse_process(rel, src)?;
        (
            true,
            serde_json::json!({ "name": p.name, "definition": p.definition }),
        )
    } else if kind == rk_types::companion::CompanionKind::Review.wire_name() {
        let r = review::parse(rel, src)?;
        losses.extend(r.losses.clone());
        let companion = rk_types::companion::ReviewCompanion {
            item: r.item,
            stage: r.stage,
            summary: rk_types::text::Text::new(r.summary),
        };
        (true, serde_json::to_value(&companion).unwrap_or_default())
    } else if kind == rk_types::companion::CompanionKind::Ledger.wire_name() {
        let legacy = ledger::parse(rel, src)?;
        let ac_ledger = ledger::to_ac_ledger(rel, &legacy)?;
        (true, serde_json::to_value(&ac_ledger).unwrap_or_default())
    } else if let Ok(companion_kind) = rk_types::companion::CompanionKind::from_wire(kind) {
        // The remaining four sidecars share ONE shape, `{ item, stage, summary }`: all five expand
        // from a single `sidecar!` macro in `rk-types` [measured 2026-09-21], so one type is a
        // faithful gate for any of them and the per-kind check is the registered-schema validation
        // below, not the choice of struct. A key with no field is a hard error here rather than a
        // silent drop, because every one carries `deny_unknown_fields` — which is what forces a
        // reader like `review.rs` to strip its extra keys into loss markers first.
        let fenced = fence::split(rel, src)?;
        let companion: rk_types::companion::IntegrationCompanion = fence::read(rel, &fenced.block)?;
        let value = serde_json::to_value(&companion).unwrap_or_default();
        validate_against_registered_schema(rel, companion_kind.wire_name(), &value)?;
        (true, value)
    } else {
        // A Phase-2 census kind: opaque prose (ADR-0075 clause 6). `write_companion` refuses it
        // with its own reason, which is how we know it is one rather than an unknown kind.
        match rk_types::companion::write_companion(kind) {
            Err(rk_types::companion::CompanionError::Phase2Kind { .. }) => {
                (false, serde_json::Value::String(src.trim().to_owned()))
            }
            _ => {
                return Err(crate::legacy::LegacyError::Malformed {
                    path: rel.to_path_buf(),
                    detail: format!(
                        "`{kind}` is neither an item record kind, a companion kind, \
                                     nor a Phase-2 census kind"
                    ),
                });
            }
        }
    };

    let outcome = if losses.is_empty() {
        Outcome::Lifted
    } else {
        Outcome::LiftedWithLoss
    };

    let mut envelope = serde_json::Map::new();
    envelope.insert(
        "schema_version".to_owned(),
        serde_json::Value::String(schema_version_for(kind).as_str().to_owned()),
    );
    envelope.insert(
        "kind".to_owned(),
        serde_json::Value::String(kind.to_owned()),
    );
    envelope.insert(
        "source".to_owned(),
        serde_json::Value::String(rel.to_string_lossy().replace('\\', "/")),
    );
    envelope.insert(if typed { "typed" } else { "opaque" }.to_owned(), value);
    if !losses.is_empty() {
        envelope.insert(
            "migration_loss".to_owned(),
            serde_json::to_value(&losses).unwrap_or_default(),
        );
    }

    Ok(LiftedFile {
        source: rel.to_path_buf(),
        kind: kind.to_owned(),
        typed,
        outcome,
        value: serde_json::Value::Object(envelope),
        losses,
    })
}

/// Validate a lifted companion against the schema `write_companion` returns for its kind.
///
/// That function is a **schema lookup, not a writer** [measured 2026-09-21,
/// `rk-types/src/companion.rs:115`], so the validation is ours to perform.
fn validate_against_registered_schema(
    rel: &Path,
    kind: &str,
    value: &serde_json::Value,
) -> Result<(), crate::legacy::LegacyError> {
    let schema = rk_types::companion::write_companion(kind).map_err(|e| {
        crate::legacy::LegacyError::Malformed {
            path: rel.to_path_buf(),
            detail: format!("no registered schema for `{kind}`: {e:?}"),
        }
    })?;
    let validator =
        jsonschema::validator_for(&schema).map_err(|e| crate::legacy::LegacyError::Malformed {
            path: rel.to_path_buf(),
            detail: format!("the registered schema for `{kind}` will not compile: {e}"),
        })?;
    let errors: Vec<String> = validator
        .iter_errors(value)
        .map(|e| e.to_string())
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(crate::legacy::LegacyError::Malformed {
            path: rel.to_path_buf(),
            detail: format!("does not validate against `{kind}`'s registered schema: {errors:?}"),
        })
    }
}
