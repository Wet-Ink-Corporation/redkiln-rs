//! The P6 context lattice — `spikes/p9/lattice.py` v2 ported clause for clause (D21-42 13b; s8b
//! §8b.2), and from the parity commit on **the instrument of record for AC-101 to AC-104**.
//!
//! # What is ported
//!
//! `lattice.py`'s module doc enumerates the readings it had to fix, and each is carried here at the
//! clause that implements it:
//!
//! - **I1** a producer is looked for in the requiring stage; its own unconditional `requires` are
//!   satisfied from the same stage first, then from any non-forbidden stage in **pack order**, or
//!   from a human-sourced root kind ([`Run::producer`]).
//! - **I2** tags compile *"same facet disjoins, distinct facets conjoin"*, AND-ed with
//!   `applicable_if` ([`tag_expr`], [`FCtx::admissible`]).
//! - **I3** `rigor` is a legal root in `applicable_if` and in pack `when` rows.
//! - **I4** a require row's own `rigor` sets the rigor of that row's artifacts, and **raises, never
//!   lowers** ([`Run::required`]). The clamp is ported but no input reaches it: every `rigor` a
//!   require row declares in `spikes/p9/packs` and `.redkiln/packs` is `"mandatory"`, the top of
//!   the order [measured 2026-09-25]. Mutant M11 is killed by a fixture instead: the
//!   `unsatisfiable` fixture's `rigor = "optional"` row under a `recommended` stage.
//! - **I5/D4** equal ranks are ties; there is no specificity tie-break (§9c.43). A tie is an AC-103
//!   error ([`Report::ac103_errors`]).
//! - **I6** `cost_class` points `S=1 M=3 L=8 XL=20`, summed over the selected DAG against
//!   `appetite.points`. A pack declaring no appetite has no overrun — where `lattice.py` writes
//!   `.get("appetite", {}).get("points", 10**9)`, [`Pack::appetite`] is `None`, and no figure
//!   can tell the two apart.
//! - **I7** the rigor axis is an item-level override applied to every non-forbidden stage, plus a
//!   fourth mode `pack-default`.
//! - **I8** the ordinal axes are the four registered ones.
//! - **I9** `exists(Kind)` is never evaluated by `lattice.py` (its `provided` argument is never
//!   passed, and no evaluated predicate calls it), so it is not ported. `exists` is not one of
//!   ADR-0103's call names, so a reached predicate calling it fails to parse and the run stops
//!   with [`LatticeError::Predicate`] naming it.
//! - **D1** the `max_rigor` ceiling, and `--validate`'s family totality ([`validate`]).
//! - **D2** roots come from `artifacts.toml` (`source = "human"`), and `--validate` checks every
//!   unconditional `requires` resolves.
//! - **D4** keyed kinds (`keyed_by = "toolchain"`) expand to one instance per toolchain, with the
//!   producer's context restricted to that instance.
//! - **D6** bands `{low, med, high}`, plus the `unknown` sweep reported separately.
//! - **D9** `recent_deploy` is in the boolean covering array.
//!
//! # Three traps, each measured, and how each is closed
//!
//! - **The covering array is seeded Python randomness** (`random.Random(7)` driving a greedy
//!   search). It is not re-derived: [`COVERING_ARRAY`] is the eight rows the Python prints,
//!   transcribed as data, and `lattice::covering_array_is_pairwise_and_matches_the_python_rows`
//!   holds them to the printed file and to pairwise coverage.
//! - **Stage order is textual, and `toml_subset::Table` is a `BTreeMap`.** A pack that declares
//!   `[policy] stage_order` (ADR-0153; every `.redkiln/` pack) is read by it. One that declares
//!   none (every `spikes/p9` pack) is read by an order-preserving scan of its `stages = { … }`
//!   line, cross-checked against the loaded key set ([`textual_stage_order`]). Iterating the map
//!   alphabetises twelve stages and moves which upstream producer I1 chooses — mutant M10.
//! - **The predicate evaluator is the product's** (`crate::predicate`, ADR-0103 and ADR-0121), not
//!   Python's `eval`. There is no compatibility mode. A predicate that evaluates to `null` is **not
//!   admissible** (only `true` admits, as in `crate::rigor`), and every such evaluation is counted
//!   in [`Report::null_verdicts`] so that a figure it moves can be traced to it rather than
//!   absorbed.
//!
//! One order `lattice.py` leaves to Python's `set` iteration is NOT emulated: `cycles()` walks a
//! `set` of successor ids, so which cycle is printed as a pack's example depends on string hashing.
//! Here successors are walked in catalog order. The cycle COUNTS do not depend on it (a depth-first
//! walk finds a back edge iff a cycle exists); the example lines do, and each is a named divergence
//! in `tests/fixtures/lattice/divergences.tsv`.
//!
//! Two more orders differ and no golden line observes them: `--validate`'s `artifacts.toml` errors
//! come out in kind order (the file's own order in Python), and the mixed differential's extra
//! `(stage, kind)` pairs are counted in sorted order (a `frozenset`'s in Python), which moves only
//! the tie order of a line the p9 goldens print as `none`. *(dated 2026-09-25, s8b refuting round 6:
//! there are three, not two)* The third is the same differential's extra STAGES (`sdiff`, printed on
//! the `STAGE` lines): `lattice.py` fills it from a Python `set` difference, the port walks a
//! `BTreeSet` difference in sorted order, and that moves only the insertion order behind the stable
//! sort of the `STAGE` lines. No golden observes it either: a non-empty stage difference implies a
//! non-empty `(stage, kind)` difference, and the p9 goldens print that as `none`.
//!
//! # What this module refuses
//!
//! **Selecting zero packs is an error, never a report.** `lattice.py --packs <x>` with a value that
//! matches no pack id prints a whole baseline over zero packs and exits 0. Here that is
//! [`LatticeError::NoPacks`]. Every other load defect is typed too: an empty catalog, an
//! unparseable predicate reached by the enumeration, a toolchain with no change kinds, a pack whose
//! stage order disagrees with its stages.
//!
//! **A predicate reading an unregistered facet is refused at load** (D21-56;
//! [`LatticeError::UnregisteredFacet`]). The registered axes keep `auth` as a covering-array
//! column because `lattice.py` binds it, while `.redkiln/facets.toml` no longer declares it; the
//! root check is what keeps that column inert.
//!
//! # Reading `.redkiln/` (D21-44)
//!
//! [`Sources::under`] names `catalog`, `packs`, `facets.toml` and `artifacts.toml`, and each is read
//! flat — [`Catalog::from_dir`] and the pack scan make one `read_dir` each and never recurse — so
//! nothing here walks into a run worktree under `.redkiln/worktrees/`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use rk_types::digest::Sha256Stream;

use crate::catalog::{Activity, Catalog};
use crate::predicate::{Context, Expr, Tri, Value, evaluate, parse};
use crate::registry::Registry;
use crate::rigor::{Rigor, with_rigor};
use crate::select::artifacts::ArtifactRegistry;
use crate::select::pack::Pack;
use crate::toml_subset;

// ------------------------------------------------------------------ the registered lattice

/// `KINDS`, in `lattice.py`'s order — the outermost loop of the enumeration.
pub const KINDS: [&str; 10] = [
    "initiative",
    "project",
    "story",
    "bug",
    "incident",
    "spike",
    "chore",
    "tech-debt",
    "security-vuln",
    "docs",
];

/// The eight pre-registered packs the AC-101/AC-102 kill lines are read over (D21-47 V3).
pub const PREREG_PACKS: [&str; 8] = [
    "bug-standard",
    "bug-expedite",
    "story",
    "story-ue",
    "incident",
    "spike",
    "project",
    "initiative",
];

/// I8: the four registered ordinal axes.
pub const ORDINALS: [&str; 4] = ["criticality", "novelty", "size", "blast_radius"];

/// D6: the three live bands.
pub const BANDS: [&str; 3] = ["low", "med", "high"];

/// The booleans the covering array is pairwise over, in `lattice.py`'s order. `auth` is here
/// because `lattice.py` binds it; see [`AuthBinding`] for the shipped reading.
pub const BOOLS: [&str; 8] = [
    "auth",
    "data_migration",
    "external_dep",
    "ui",
    "market_facing",
    "has_repro",
    "flag_exists",
    "recent_deploy",
];

/// `covering_array(len(BOOLS))` as the pinned `lattice.py` prints it.
///
/// TRANSCRIBED, not re-derived: the rows come out of `random.Random(7)`'s Mersenne Twister draws, and porting the draws to
/// reproduce eight rows would be a second implementation of a PRNG to recover a table.
pub const COVERING_ARRAY: &[[u8; 8]] = &[
    [1, 0, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, 1, 1, 0, 0, 1],
    [0, 0, 1, 1, 0, 1, 0, 0],
    [1, 1, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 0, 1, 0],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 1, 0, 1, 0, 1, 1],
    [1, 1, 0, 1, 0, 1, 0, 0], // M6
];

/// `CHANGE_OF`: the change kinds an instance of a keyed kind is produced under (D4).
const CHANGE_OF: [(&str, &[&str]); 5] = [
    ("node", &["text"]),
    ("dotnet", &["text"]),
    ("unreal", &["binary"]),
    ("python", &["text"]),
    ("rust", &["text"]),
];

/// `lattice.py`'s `TOOLCHAINS`, in its order: `(name, toolchain set, change set)`.
const P6_TOOLCHAINS: &[(&str, &[&str], &[&str])] = &[
    ("node", &["node"], &["text"]),
    ("unreal", &["unreal"], &["binary"]),
    ("node+unreal", &["node", "unreal"], &["text", "binary"]),
    ("dotnet", &["dotnet"], &["text"]), // M24
];

/// The `rust` row D21-43 N26 adds for the `.redkiln/` reading, in the Python's own shape:
/// `("rust", ({"rust"}, {"text"}))`.
const RUST_TOOLCHAIN: (&str, &[&str], &[&str]) = ("rust", &["rust"], &["text"]);

/// `.redkiln/facets.toml`'s `surfaces` values.
///
/// The shipped enumeration is the empty set plus each of these as a singleton;
/// `lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged` holds this list to
/// the registry it reads. *(dated 2026-09-25, s8b refuting round 1: this named
/// `p6_lattice_over_the_shipped_pack`, which asserts only the surface-row count.)*
pub const SHIPPED_SURFACES: [&str; 5] = ["auth", "flags", "migration", "payments", "secrets"];

/// I6's cost points.
fn cost_points(class: &str) -> Option<u64> {
    match class {
        "S" => Some(1),
        "M" => Some(3),
        "L" => Some(8),
        "XL" => Some(20),
        _ => None,
    }
}

fn change_of(inst: &str) -> Option<&'static [&'static str]> {
    CHANGE_OF.iter().find(|(t, _)| *t == inst).map(|(_, c)| *c)
}

// ------------------------------------------------------------------ axes

/// One toolchain alternative: its name, the `toolchain` set it binds and the `change` set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolchainRow {
    pub name: &'static str,
    pub tools: Vec<&'static str>,
    pub change: Vec<&'static str>,
}

impl ToolchainRow {
    fn of(
        (name, tools, change): (
            &'static str,
            &'static [&'static str],
            &'static [&'static str],
        ),
    ) -> Self {
        Self {
            name,
            tools: tools.to_vec(),
            change: change.to_vec(),
        }
    }

    /// `sorted(F["toolchain"])` — the order keyed kinds expand in.
    fn sorted_tools(&self) -> Vec<&'static str> {
        let mut t = self.tools.clone();
        t.sort_unstable();
        t
    }
}

/// How the `auth` boolean is bound.
///
/// **Ruled by D21-56 (2026-09-25; it was the open question B-AUTH).** `.redkiln/facets.toml` no
/// longer declares an `auth` boolean — Step 7e moved it into `surfaces` — while `lattice.py`'s
/// `BOOLS` still carries it. D21-56: [`Axes::shipped`] keeps the column
/// ([`AuthBinding::CoveringArray`]); it is inert because no `.redkiln/` predicate reads it, and
/// that is guarded in code (mutant M30). [`AuthBinding::SurfacesMember`] is an alternative no
/// ruling selects or declines, measured figure-neutral in the session that took D21-56;
/// [`Axes::shipped`] never selects it. *(dated 2026-09-25, s8b refuting rounds 2 and 3: round 1
/// said D21-56 declined it; D21-56's one Declined bullet is the seven-column projection of the
/// covering array, and round 3 replaced that sentence rather than appending to it.)*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthBinding {
    /// `lattice.py`'s own: `auth` is a column of the covering array.
    CoveringArray,
    /// `auth` is bound as `'auth' in surfaces` — the covering array's column is ignored.
    SurfacesMember,
}

/// The lattice's axes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Axes {
    pub toolchains: Vec<ToolchainRow>,
    /// The covering array's columns ([`BOOLS`]).
    pub bools: Vec<&'static str>,
    /// The covering array's rows.
    pub covering_rows: Vec<[u8; 8]>,
    /// `None`: `surfaces` is not bound at all (the registered reading). `Some(rows)`: one
    /// alternative per row, each a set.
    pub surfaces: Option<Vec<Vec<&'static str>>>,
    pub auth: AuthBinding,
}

impl Axes {
    /// `lattice.py`'s set exactly — four toolchains, its booleans (`auth` included), no
    /// `surfaces`. Every parity test over `spikes/p9` uses this and only this, so the goldens stay
    /// the Python's bytes.
    #[must_use]
    pub fn p6_registered() -> Self {
        Self {
            toolchains: P6_TOOLCHAINS
                .iter()
                .copied()
                .map(ToolchainRow::of)
                .collect(),
            bools: BOOLS.to_vec(),
            covering_rows: COVERING_ARRAY.to_vec(),
            surfaces: None,
            auth: AuthBinding::CoveringArray,
        }
    }

    /// The registered set plus the two additions D21-43 N26 registers for the `.redkiln/` reading:
    /// a `rust` toolchain row, and a `surfaces` binding enumerated as the empty set plus each
    /// declared value as a singleton (six rows).
    #[must_use]
    pub fn shipped() -> Self {
        let mut axes = Self::p6_registered();
        axes.toolchains.push(ToolchainRow::of(RUST_TOOLCHAIN)); // M23
        let mut rows = vec![Vec::new()];
        rows.extend(SHIPPED_SURFACES.iter().map(|s| vec![*s]));
        axes.surfaces = Some(rows);
        axes.auth = AuthBinding::CoveringArray;
        axes
    }

    fn surface_rows(&self) -> Vec<Option<usize>> {
        self.surfaces
            .as_ref()
            .map_or_else(|| vec![None], |rows| (0..rows.len()).map(Some).collect())
    }
}

// ------------------------------------------------------------------ errors

/// Why the lattice could not run. Never collapsed into an empty report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatticeError {
    /// The selection matched no pack — `lattice.py --packs <x>`'s silent zero-pack baseline,
    /// refused.
    NoPacks {
        dir: String,
        requested: Option<Vec<String>>,
    },
    Io {
        path: String,
        detail: String,
    },
    /// A source that loaded but whose shape the lattice cannot read.
    Shape {
        source: String,
        detail: String,
    },
    /// A pack's textual stage order disagrees with its loaded stages.
    StageOrder {
        pack: String,
        detail: String,
    },
    /// A predicate the enumeration reached does not parse or does not evaluate.
    Predicate {
        predicate: String,
        detail: String,
    },
    /// A predicate reads a field whose root the facet registry does not declare — refused at load,
    /// before any evaluation (D21-56; see [`check_roots`]).
    UnregisteredFacet {
        site: String,
        predicate: String,
        facet: String,
    },
}

impl fmt::Display for LatticeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPacks { dir, requested } => write!(
                f,
                "the lattice selected ZERO packs from {dir} (requested: {requested:?}) — a report \
                 over no pack is a baseline over nothing, and it is refused rather than printed"
            ),
            Self::Io { path, detail } => write!(f, "{path}: {detail}"),
            Self::Shape { source, detail } => write!(f, "{source}: {detail}"),
            Self::StageOrder { pack, detail } => write!(f, "pack {pack}: {detail}"),
            Self::Predicate { predicate, detail } => {
                write!(f, "predicate {predicate:?}: {detail}")
            }
            Self::UnregisteredFacet {
                site,
                predicate,
                facet,
            } => write!(
                f,
                "{site}: predicate {predicate:?} reads {facet:?}, which the facet registry does not \
                 declare — an unregistered facet feeds no row (AC-072/AC-108), and the lattice \
                 refuses it rather than evaluating it against a context that happens to bind it"
            ),
        }
    }
}

impl std::error::Error for LatticeError {}

fn shape(source: &Path, detail: impl Into<String>) -> LatticeError {
    LatticeError::Shape {
        source: source.display().to_string(),
        detail: detail.into(),
    }
}

// ------------------------------------------------------------------ inputs

/// The four sources, each named — nothing is discovered by walking a tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sources {
    pub catalog: PathBuf,
    pub packs: PathBuf,
    pub facets: PathBuf,
    pub artifacts: PathBuf,
}

impl Sources {
    /// `<root>/catalog`, `<root>/packs`, `<root>/facets.toml`, `<root>/artifacts.toml` — the
    /// layout both `spikes/p9` and `.redkiln/` have.
    #[must_use]
    pub fn under(root: &Path) -> Self {
        Self {
            catalog: root.join("catalog"),
            packs: root.join("packs"),
            facets: root.join("facets.toml"),
            artifacts: root.join("artifacts.toml"),
        }
    }
}

/// One activity as the lattice reads it.
struct Act {
    id: String,
    stage: String,
    provides: Vec<String>,
    /// `(kind, optional)` in declaration order.
    requires: Vec<(String, bool)>,
    rank: i64,
    min: Rigor,
    min_token: Option<String>,
    max: Option<Rigor>,
    max_token: Option<String>,
    tags: Option<String>,
    applicable_if: Option<String>,
    cost_class: Option<String>,
    family: Option<String>,
}

/// One pack as the lattice reads it.
struct PackIn {
    id: String,
    path: PathBuf,
    bytes: Vec<u8>,
    pack: Pack,
    applies: String,
    /// `policy.stages`, in the order `lattice.py` iterates it.
    order: Vec<(String, Rigor)>,
}

impl PackIn {
    fn policy(&self, stage: &str) -> Option<Rigor> {
        self.pack.rigor(stage)
    }
}

/// Everything the lattice reads, loaded once.
pub struct Inputs {
    catalog: Catalog,
    /// In `lattice.py`'s `cat.acts` order — the catalog's file paths, sorted.
    acts: Vec<Act>,
    by_kind_stage: BTreeMap<String, BTreeMap<String, Vec<usize>>>,
    by_kind: BTreeMap<String, Vec<usize>>,
    registry: Registry,
    roots: BTreeSet<String>,
    /// *(dated 2026-09-29, s8g build, package S8G-P4-PACK-REHOME; R1 = ADR-0191 decision 1,
    /// proposed.)* `artifacts.toml` kinds with `source = "pass"`: a composition-root pass produces
    /// each, so, as the selector does (`KindSource::Pass`), the walk judges no catalog producer for
    /// one and never reports it unsatisfiable. `lattice.py` knows no such source; the p9 tree
    /// declares none, so the parity goldens do not move. Not a root: the header's `roots=` list is
    /// `source = "human"` alone.
    passes: BTreeSet<String>,
    keyed: BTreeSet<String>,
    /// `artifacts.toml` kinds with `source = "activity"` and no `status = "extended"`.
    activity_kinds: Vec<String>,
    packs: Vec<PackIn>,
    packs_dir: PathBuf,
    preds: BTreeMap<String, Result<Expr, String>>,
    ordinal_domains: Vec<Vec<String>>,
    catalog_digest: String,
    facets_digest: String,
}

/// `sha256(basename ‖ bytes)` over the paths sorted, first twelve hex characters —
/// `lattice.py`'s `digest_paths`.
fn digest_paths(paths: &[(PathBuf, Vec<u8>)]) -> String {
    let mut sorted: Vec<&(PathBuf, Vec<u8>)> = paths.iter().collect();
    sorted.sort_by(|a, b| a.0.to_string_lossy().cmp(&b.0.to_string_lossy()));
    let mut h = Sha256Stream::start();
    for (path, bytes) in sorted {
        let base = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        h.update(base.as_bytes());
        h.update(bytes);
    }
    h.finish().as_str()[..12].to_owned()
}

fn read(path: &Path) -> Result<Vec<u8>, LatticeError> {
    std::fs::read(path).map_err(|e| LatticeError::Io {
        path: path.display().to_string(),
        detail: e.to_string(),
    })
}

fn text(path: &Path, bytes: &[u8]) -> Result<String, LatticeError> {
    String::from_utf8(bytes.to_vec()).map_err(|e| shape(path, format!("not UTF-8: {e}")))
}

/// **The facet-registry root check** (D21-56): every field path a reachable predicate reads must
/// be rooted at a facet the loaded registry declares, or at `rigor` (I3).
///
/// This is what keeps [`AuthBinding::CoveringArray`] inert. `auth` stays a column of the covering
/// array because `lattice.py` binds it, so a context always carries an `auth` boolean; a predicate
/// reading bare `auth` against a registry that no longer declares it would otherwise EVALUATE,
/// against that column, rather than fail. Refused here, at load, before anything is evaluated.
/// (ADR-0103's operand rule already makes a bare `auth` a field path and never the `surfaces`
/// literal `'auth'`; `lattice.py`'s compiler did the opposite whenever the name was some facet's
/// value.)
///
/// **Quantifier bodies** (refuting round 4, 2026-09-25). A prefix-form body
/// (`any(s in surfaces, …)`) is checked less its variable and the variables bound around it. A
/// method-form body (`surfaces.any(…)`) binds no name: the evaluator pushes a frame holding an
/// element's fields only when that element is a Record, and an empty frame otherwise, so a path in
/// the body falls through to the enclosing quantifier variables and then to the context — the
/// covering column included. Every value a lattice context carries is a scalar, a boolean or a list
/// of strings (a facet, `rigor`, or a covering column), never a Record, so every method-form body
/// root is checked exactly as a top-level root is: less the enclosing variables, and otherwise
/// registered or `rigor`. A method-form source that is itself unregistered is refused on its own
/// root, and when a predicate reads more than one unregistered root the refusal names the first in
/// sorted order. Measured at this check's round-4 widening: no predicate the shipped `.redkiln/`
/// or the p9 tree reaches is a method-form quantifier, so the shipped load and p9 parity do not
/// move.
fn check_roots(
    preds: &BTreeMap<String, Result<Expr, String>>,
    sites: &BTreeMap<String, String>,
    registry: &Registry,
) -> Result<(), LatticeError> {
    for (src, parsed) in preds {
        let Ok(expr) = parsed else { continue };
        let mut read = BTreeSet::new();
        roots(expr, &BTreeSet::new(), &mut read);
        for root in read {
            if root != "rigor" && registry.facet(&root).is_none() {
                return Err(LatticeError::UnregisteredFacet {
                    site: sites.get(src).cloned().unwrap_or_default(),
                    predicate: src.clone(),
                    facet: root,
                });
            }
        }
    }
    Ok(())
}

/// Every field-path root `expr` reads, less the quantifier variables bound around it.
fn roots(expr: &Expr, bound: &BTreeSet<String>, out: &mut BTreeSet<String>) {
    match expr {
        Expr::Path(p) => {
            let root = p.split('.').next().unwrap_or(p);
            if !bound.contains(root) {
                out.insert(root.to_owned());
            }
        }
        Expr::Or(l, r) | Expr::And(l, r) => {
            roots(l, bound, out);
            roots(r, bound, out);
        }
        Expr::Cmp { left, right, .. } => {
            roots(left, bound, out);
            roots(right, bound, out);
        }
        Expr::In { left, list } => {
            roots(left, bound, out);
            roots(list, bound, out);
        }
        Expr::Member { literal, set } => {
            roots(literal, bound, out);
            roots(set, bound, out);
        }
        Expr::Not(inner) | Expr::Group(inner) | Expr::Len(inner) => roots(inner, bound, out),
        Expr::List(items) => {
            for item in items {
                roots(item, bound, out);
            }
        }
        Expr::Quant {
            var, source, body, ..
        } => {
            roots(source, bound, out);
            // The prefix form's body is checked less its variable. The method form binds no name,
            // and it puts an element's fields in scope only when the element is a Record; no
            // lattice context value is one (refuting round 4, 2026-09-25: see `check_roots`), so
            // its body's paths resolve as a top-level path does and are checked the same way.
            if let Some(v) = var {
                let mut inner = bound.clone();
                inner.insert(v.clone());
                roots(body, &inner, out); // M30c
            } else {
                roots(body, bound, out); // M30b
            }
        }
        Expr::Lit(_) | Expr::Ref(_) => {}
    }
}

/// The `*.toml` files directly in `dir`, sorted — one `read_dir`, never a recursive walk.
fn tomls_in(dir: &Path) -> Result<Vec<PathBuf>, LatticeError> {
    let entries = std::fs::read_dir(dir).map_err(|e| LatticeError::Io {
        path: dir.display().to_string(),
        detail: e.to_string(),
    })?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));
    Ok(paths)
}

/// The order `lattice.py` iterates `policy.stages` in.
///
/// A pack that declares `[policy] stage_order` (ADR-0153) is read by it — the loader has already
/// refused one whose order and stages disagree. A pack that declares none is read by scanning its
/// `stages = { … }` line in `[policy]`, left to right, and the scan is cross-checked against the
/// loaded key set, so a scan that missed or invented a stage is an error rather than an order.
fn textual_stage_order(pack: &Pack, src: &str) -> Result<Vec<String>, LatticeError> {
    if !pack.stage_order().is_empty() {
        return Ok(pack.stage_order().to_vec());
    }
    let refuse = |detail: String| LatticeError::StageOrder {
        pack: pack.id.clone(),
        detail,
    };
    let mut in_policy = false;
    let mut line = None;
    for raw in src.lines() {
        let t = raw.trim();
        if t.starts_with('[') {
            in_policy = t == "[policy]";
            continue;
        }
        if in_policy
            && let Some(rest) = t.strip_prefix("stages")
            && rest.trim_start().starts_with('=')
        {
            line = Some(rest.trim_start()[1..].trim());
            break;
        }
    }
    let line = line.ok_or_else(|| refuse("no `stages = { … }` line under [policy]".to_owned()))?;
    let body = line
        .strip_prefix('{')
        .and_then(|b| b.split_once('}').map(|(inner, _)| inner))
        .ok_or_else(|| refuse(format!("`stages` is not a one-line inline table: {line:?}")))?;
    let mut order = Vec::new();
    for entry in body.split(',').map(str::trim).filter(|e| !e.is_empty()) {
        let (key, _) = entry
            .split_once('=')
            .ok_or_else(|| refuse(format!("cannot read the entry {entry:?}")))?;
        order.push(key.trim().trim_matches('"').to_owned());
    }
    let scanned: BTreeSet<&str> = order.iter().map(String::as_str).collect();
    let loaded: BTreeSet<&str> = pack.policy().map(|(s, _)| s.as_str()).collect();
    if scanned != loaded || scanned.len() != order.len() {
        return Err(refuse(format!(
            "the scanned stage order {order:?} is not the loaded stage set {loaded:?}"
        )));
    }
    Ok(order)
}

/// I2: the tag filter as a predicate, facets in first-appearance order.
fn tag_expr(
    tags: &[String],
    registry: &Registry,
    source: &str,
) -> Result<Option<String>, LatticeError> {
    let mut groups: Vec<(&str, Vec<&str>)> = Vec::new();
    for tag in tags {
        let (facet, value) = tag.split_once(':').unwrap_or((tag.as_str(), ""));
        match groups.iter_mut().find(|(f, _)| *f == facet) {
            Some((_, vs)) => vs.push(value),
            None => groups.push((facet, vec![value])),
        }
    }
    if groups.is_empty() {
        return Ok(None);
    }
    let mut parts = Vec::with_capacity(groups.len());
    for (facet, values) in groups {
        let set = registry
            .facet(facet)
            .ok_or_else(|| LatticeError::Shape {
                source: source.to_owned(),
                detail: format!(
                    "a tag names the facet {facet:?}, which the registry does not declare"
                ),
            })?
            .set;
        let terms: Vec<String> = values
            .iter()
            .map(|v| {
                if set {
                    format!("'{v}' in {facet}")
                } else {
                    format!("{facet} == '{v}'")
                }
            })
            .collect();
        parts.push(format!("({})", terms.join(" or ")));
    }
    Ok(Some(parts.join(" and ")))
}

fn rigor_token(token: &str, source: &str) -> Result<Rigor, LatticeError> {
    Rigor::from_token(token).ok_or_else(|| LatticeError::Shape {
        source: source.to_owned(),
        detail: format!("rigor token {token:?} is outside ADR-0020's five"),
    })
}

fn act_from(a: &Activity, registry: &Registry) -> Result<Act, LatticeError> {
    Ok(Act {
        id: a.id.clone(),
        stage: a.stage.clone(),
        provides: a.provides.clone(),
        requires: a
            .requires
            .iter()
            .map(|r| (r.kind.clone(), r.optional))
            .collect(),
        rank: a.method_rank,
        min: rigor_token(&a.min_rigor, &a.source)?,
        min_token: Some(a.min_rigor.clone()),
        max: a
            .max_rigor
            .as_deref()
            .map(|t| rigor_token(t, &a.source))
            .transpose()?,
        max_token: a.max_rigor.clone(),
        tags: tag_expr(&a.tags, registry, &a.source)?,
        applicable_if: a.applicable_if.clone().filter(|s| !s.is_empty()),
        cost_class: a.cost_class.clone(),
        family: a.family.clone(),
    })
}

/// Load the four sources.
///
/// # Errors
///
/// [`LatticeError`] for any source that cannot be read, or a catalog or pack outside its loader's
/// shape. *(dated 2026-09-25, s8b refuting round 2: this said "or an empty pack directory". It
/// is not an error here: `load` returns `Ok` with zero packs, and the zero-pack refusal is
/// [`run`]'s [`LatticeError::NoPacks`].)* *(dated 2026-09-25, s8b refuting round 5)* It also
/// returns [`LatticeError::StageOrder`] for a pack whose textual stage order disagrees with its
/// loaded stages (M10), and [`LatticeError::UnregisteredFacet`] for a reachable predicate that reads
/// a facet the registry does not declare (D21-56; M30, M30b).
pub fn load(sources: &Sources) -> Result<Inputs, LatticeError> {
    let facets_bytes = read(&sources.facets)?;
    let registry = Registry::from_toml(&text(&sources.facets, &facets_bytes)?)
        .map_err(|e| shape(&sources.facets, e.to_string()))?;
    let artifacts_bytes = read(&sources.artifacts)?;
    let artifacts_src = text(&sources.artifacts, &artifacts_bytes)?;
    let artifacts = ArtifactRegistry::from_toml("artifacts.toml", &artifacts_src)
        .map_err(|e| shape(&sources.artifacts, e.to_string()))?;
    // `status` is not an `ArtifactRegistry` field; `--validate` reads it, so it is read here.
    let doc =
        toml_subset::parse(&artifacts_src).map_err(|e| shape(&sources.artifacts, e.to_string()))?;
    let table = doc
        .get("artifact")
        .and_then(toml_subset::Value::as_table)
        .ok_or_else(|| shape(&sources.artifacts, "no [artifact.*] tables"))?;
    let mut roots = BTreeSet::new();
    let mut passes = BTreeSet::new();
    let mut keyed = BTreeSet::new();
    let mut activity_kinds = Vec::new();
    for (name, row) in table {
        let kind = artifacts
            .kind(name)
            .ok_or_else(|| shape(&sources.artifacts, format!("{name} did not load")))?;
        if artifacts.is_human_sourced(name) {
            roots.insert(name.clone());
        }
        let pass = artifacts.is_pass_sourced(name);
        if pass {
            passes.insert(name.clone());
        }
        if kind.keyed_by.as_deref() == Some("toolchain") {
            keyed.insert(name.clone());
        }
        let status = row
            .as_table()
            .and_then(|t| t.get("status"))
            .and_then(toml_subset::Value::as_str);
        if !artifacts.is_human_sourced(name) && !pass && status != Some("extended") {
            activity_kinds.push(name.clone());
        }
    }

    let catalog =
        Catalog::from_dir(&sources.catalog).map_err(|e| shape(&sources.catalog, e.to_string()))?;
    let catalog_files: Vec<(PathBuf, Vec<u8>)> = tomls_in(&sources.catalog)?
        .into_iter()
        .map(|p| read(&p).map(|b| (p, b)))
        .collect::<Result<_, _>>()?;
    // `cat.acts` is a dict filled in sorted-PATH order, and `by_kind_stage` lists candidates in
    // it; `Catalog` sorts by id, which is a different order wherever one id prefixes another.
    let mut ordered: Vec<&Activity> = catalog.activities().iter().collect();
    ordered.sort_by(|a, b| a.source.cmp(&b.source));
    let acts = ordered
        .into_iter()
        .map(|a| act_from(a, &registry))
        .collect::<Result<Vec<_>, _>>()?;
    let mut by_kind_stage: BTreeMap<String, BTreeMap<String, Vec<usize>>> = BTreeMap::new();
    let mut by_kind: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, a) in acts.iter().enumerate() {
        for k in &a.provides {
            by_kind_stage
                .entry(k.clone())
                .or_default()
                .entry(a.stage.clone())
                .or_default()
                .push(i);
            by_kind.entry(k.clone()).or_default().push(i);
        }
    }

    let mut packs = Vec::new();
    for path in tomls_in(&sources.packs)? {
        let bytes = read(&path)?;
        let src = text(&path, &bytes)?;
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let pack = Pack::from_toml(&name, &src).map_err(|e| shape(&path, e.to_string()))?;
        let applies = pack
            .applies
            .clone()
            .ok_or_else(|| shape(&path, "[pack] declares no `applies`"))?;
        let order = textual_stage_order(&pack, &src)? // M10
            .into_iter()
            .map(|s| {
                let r = pack.rigor(&s).unwrap_or(Rigor::Forbidden);
                (s, r)
            })
            .collect();
        packs.push(PackIn {
            id: pack.id.clone(),
            path,
            bytes,
            pack,
            applies,
            order,
        });
    }

    // Every predicate the enumeration can reach, parsed once, with the first site that carries it.
    let mut preds = BTreeMap::new();
    let mut sites: BTreeMap<String, String> = BTreeMap::new();
    let mut add = |s: &str, site: String| {
        preds
            .entry(s.to_owned())
            .or_insert_with(|| parse(s).map(|p| p.expr).map_err(|e| e.to_string()));
        sites.entry(s.to_owned()).or_insert(site);
    };
    for a in &acts {
        if let Some(t) = &a.tags {
            add(t, format!("catalog/{}: tags", a.id));
        }
        if let Some(t) = &a.applicable_if {
            add(t, format!("catalog/{}: applicable_if", a.id));
        }
    }
    for p in &packs {
        add(&p.applies, format!("packs/{}: applies", p.id));
        for (_, spec) in p.pack.stage_specs() {
            for row in &spec.require {
                add(&row.when, row.site.clone());
            }
        }
    }
    check_roots(&preds, &sites, &registry)?; // M30

    let mut ordinal_domains = Vec::new();
    for axis in ORDINALS {
        let facet = registry.facet(axis).ok_or_else(|| {
            shape(
                &sources.facets,
                format!("the ordinal axis {axis:?} is not declared"),
            )
        })?;
        ordinal_domains.push(
            facet
                .values
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect(),
        );
    }

    let catalog_digest = digest_paths(&catalog_files);
    let facets_digest = digest_paths(&[
        (sources.facets.clone(), facets_bytes),
        (sources.artifacts.clone(), artifacts_bytes),
    ]);
    Ok(Inputs {
        catalog,
        acts,
        by_kind_stage,
        by_kind,
        registry,
        roots,
        passes,
        keyed,
        activity_kinds,
        packs,
        packs_dir: sources.packs.clone(),
        preds,
        ordinal_domains,
        catalog_digest,
        facets_digest,
    })
}

impl Inputs {
    /// The catalog as loaded.
    #[must_use]
    pub const fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    /// Every pack id, in the order the packs were read.
    #[must_use]
    pub fn pack_ids(&self) -> Vec<&str> {
        self.packs.iter().map(|p| p.id.as_str()).collect()
    }

    fn expr(&self, src: &str) -> Result<&Expr, LatticeError> {
        match self.preds.get(src) {
            Some(Ok(e)) => Ok(e),
            Some(Err(detail)) => Err(LatticeError::Predicate {
                predicate: src.to_owned(),
                detail: detail.clone(),
            }),
            None => Err(LatticeError::Predicate {
                predicate: src.to_owned(),
                detail: "not collected at load".to_owned(),
            }),
        }
    }

    fn candidates(&self, kind: &str, stage: &str) -> &[usize] {
        self.by_kind_stage
            .get(kind)
            .and_then(|m| m.get(stage))
            .map_or(&[], Vec::as_slice)
    }

    fn producers_of(&self, kind: &str) -> &[usize] {
        self.by_kind.get(kind).map_or(&[], Vec::as_slice)
    }

    fn ordinal(&self, axis: usize, band: &str) -> Result<Value, LatticeError> {
        let domain = &self.ordinal_domains[axis];
        let index = domain
            .iter()
            .position(|d| d == band)
            .ok_or_else(|| LatticeError::Shape {
                source: "facets.toml".to_owned(),
                detail: format!("{} declares no band {band:?}", ORDINALS[axis]),
            })?;
        Ok(Value::Ordinal {
            domain: domain.clone(),
            index,
        })
    }
}

// ------------------------------------------------------------------ one facet vector

/// One point of the lattice, without its rigor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Fv {
    kind: &'static str,
    cos: &'static str,
    bands: [&'static str; 4],
    tc: usize,
    row: usize,
    surf: Option<usize>,
}

/// `ctx_label(F, tname)`, with `surfaces=` appended when the axes bind it.
fn ctx_label(fv: &Fv, tname: &str, axes: &Axes) -> String {
    let mut s = format!("kind={} cos={} ", fv.kind, fv.cos);
    let bands: Vec<String> = ORDINALS
        .iter()
        .zip(fv.bands)
        .map(|(a, b)| format!("{a}={b}"))
        .collect();
    s.push_str(&bands.join(" "));
    let _ = write!(s, " tc={tname} ");
    let bools: Vec<String> = axes
        .bools
        .iter()
        .enumerate()
        .map(|(i, b)| format!("{b}={}", u8::from(bool_value(axes, fv, i))))
        .collect();
    s.push_str(&bools.join(" "));
    if let (Some(rows), Some(i)) = (&axes.surfaces, fv.surf) {
        let _ = write!(s, " surfaces={{{}}}", rows[i].join(","));
    }
    s
}

fn bool_value(axes: &Axes, fv: &Fv, i: usize) -> bool {
    if axes.bools[i] == "auth" && axes.auth == AuthBinding::SurfacesMember {
        return match (&axes.surfaces, fv.surf) {
            (Some(rows), Some(s)) => rows[s].contains(&"auth"),
            _ => false,
        };
    }
    axes.covering_rows[fv.row][i] != 0
}

/// Why an activity is not admissible — `lattice.py`'s reason strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Admissible,
    MinRigor,
    MaxRigor,
    Tags,
    ApplicableIf,
}

/// The per-vector caches: one context per (rigor, instance), one verdict per (activity, rigor,
/// instance).
struct FCtx<'x> {
    tc: &'x ToolchainRow,
    base: Context,
    ctxs: BTreeMap<(Rigor, Option<&'static str>), Context>,
    adm: BTreeMap<(usize, Rigor, Option<&'static str>), Verdict>,
    nulls: BTreeMap<String, u64>,
}

impl<'x> FCtx<'x> {
    fn new(inp: &Inputs, axes: &'x Axes, fv: Fv) -> Result<Self, LatticeError> {
        let tc = &axes.toolchains[fv.tc];
        let mut base = Context::new();
        base.bind("kind", Value::Str(fv.kind.to_owned()));
        base.bind("cos", Value::Str(fv.cos.to_owned()));
        base.bind("toolchain", Value::set(tc.tools.iter().copied()));
        base.bind("change", Value::set(tc.change.iter().copied()));
        for (i, (axis, band)) in ORDINALS.iter().zip(fv.bands).enumerate() {
            base.bind(axis, inp.ordinal(i, band)?);
        }
        for (i, b) in axes.bools.iter().enumerate() {
            base.bind(b, Value::Bool(bool_value(axes, &fv, i)));
        }
        if let (Some(rows), Some(s)) = (&axes.surfaces, fv.surf) {
            base.bind("surfaces", Value::set(rows[s].iter().copied()));
        }
        Ok(Self {
            tc,
            base,
            ctxs: BTreeMap::new(),
            adm: BTreeMap::new(),
            nulls: BTreeMap::new(),
        })
    }

    /// `F` with `rigor` bound and, for an instance of a keyed kind, `toolchain`/`change`
    /// restricted to it (D4).
    fn ctx(&mut self, inp: &Inputs, rigor: Rigor, inst: Option<&'static str>) -> &Context {
        let base = &self.base;
        self.ctxs.entry((rigor, inst)).or_insert_with(|| {
            let mut c = with_rigor(base.clone(), rigor, &inp.registry);
            if let Some(i) = inst {
                c.bind("toolchain", Value::set([i]));
                c.bind(
                    "change",
                    Value::set(change_of(i).unwrap_or_default().iter().copied()),
                );
            }
            c
        })
    }

    fn truth(
        &mut self,
        inp: &Inputs,
        src: &str,
        rigor: Rigor,
        inst: Option<&'static str>,
    ) -> Result<Tri, LatticeError> {
        let expr = inp.expr(src)?;
        let t = {
            let ctx = self.ctx(inp, rigor, inst);
            evaluate(expr, ctx)
                .map_err(|e| LatticeError::Predicate {
                    predicate: src.to_owned(),
                    detail: e.to_string(),
                })?
                .value
        };
        if t == Tri::Null {
            *self.nulls.entry(src.to_owned()).or_default() += 1;
        }
        Ok(t)
    }

    /// `Catalog.admissible`, clause for clause: the floor, the ceiling (D1), the tag filter (I2),
    /// then `applicable_if`. Only `true` admits.
    fn admissible(
        &mut self,
        inp: &Inputs,
        ai: usize,
        rigor: Rigor,
        inst: Option<&'static str>,
    ) -> Result<Verdict, LatticeError> {
        if let Some(v) = self.adm.get(&(ai, rigor, inst)) {
            return Ok(*v);
        }
        let a = &inp.acts[ai];
        let v = if a.min > rigor {
            Verdict::MinRigor
        } else if a.max.is_some_and(|m| m < rigor) {
            // M7
            Verdict::MaxRigor
        } else if let Some(tags) = &a.tags
            && self.truth(inp, tags, rigor, inst)? != Tri::True
        {
            Verdict::Tags
        } else if let Some(ai_src) = &a.applicable_if
            && self.truth(inp, ai_src, rigor, inst)? != Tri::True
        {
            Verdict::ApplicableIf
        } else {
            Verdict::Admissible
        };
        self.adm.insert((ai, rigor, inst), v);
        Ok(v)
    }
}

fn reason(a: &Act, v: Verdict) -> String {
    match v {
        Verdict::Admissible => String::new(),
        Verdict::MinRigor => format!("min_rigor={}", a.min_token.as_deref().unwrap_or("None")),
        Verdict::MaxRigor => format!("max_rigor={}", a.max_token.as_deref().unwrap_or("None")),
        Verdict::Tags => "tags".to_owned(),
        Verdict::ApplicableIf => {
            format!("applicable_if={}", a.applicable_if.as_deref().unwrap_or(""))
        }
    }
}

// ------------------------------------------------------------------ one selection

/// The rigor mode of one run (I7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Live(Rigor),
    PackDefault,
}

impl Mode {
    const ALL: [Self; 4] = [
        Self::Live(Rigor::Optional),
        Self::Live(Rigor::Recommended),
        Self::Live(Rigor::Mandatory),
        Self::PackDefault,
    ];

    const fn as_str(self) -> &'static str {
        match self {
            Self::Live(r) => r.as_str(),
            Self::PackDefault => "pack-default",
        }
    }
}

/// Where an R-level requirement came from — carried so an unsatisfiable report names the row.
#[derive(Debug, Clone, Copy)]
enum Origin<'a> {
    Baseline,
    Row { index: usize, when: &'a str },
}

#[derive(Debug, Clone, Copy)]
enum Why<'a> {
    R(&'a str, Origin<'a>),
    RequiresOf(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Got {
    Root,
    Act(usize),
}

#[derive(Debug, Clone)]
struct Unsat {
    stage: String,
    kind: String,
    rigor: Rigor,
    why: String,
    origin: Option<String>,
    failed: Vec<String>,
    other_stage: Vec<String>,
    forbidden_stage: bool,
}

type Tie = (String, String, Vec<String>);

/// `Run`: steps 1, 2, 4 and 5 for one pack over one vector at one mode.
struct Run<'a, 'f, 'x> {
    inp: &'a Inputs,
    pack: &'a PackIn,
    f: &'f mut FCtx<'x>,
    mode: Mode,
    selected: Vec<usize>,
    selected_set: BTreeSet<usize>,
    unsat: Vec<Unsat>,
    ties: Vec<Tie>,
    visiting: BTreeSet<(&'a str, &'a str, Option<&'static str>)>,
    provided: BTreeSet<(&'a str, Option<&'static str>)>,
    deviation_reachable: BTreeSet<usize>,
    selected_inst: BTreeSet<(usize, Option<&'static str>)>,
    hard: Vec<Vec<usize>>,
    soft: Vec<Vec<usize>>,
    cost: u64,
}

impl<'a, 'f, 'x> Run<'a, 'f, 'x> {
    const fn new(inp: &'a Inputs, pack: &'a PackIn, f: &'f mut FCtx<'x>, mode: Mode) -> Self {
        Self {
            inp,
            pack,
            f,
            mode,
            selected: Vec::new(),
            selected_set: BTreeSet::new(),
            unsat: Vec::new(),
            ties: Vec::new(),
            visiting: BTreeSet::new(),
            provided: BTreeSet::new(),
            deviation_reachable: BTreeSet::new(),
            selected_inst: BTreeSet::new(),
            hard: Vec::new(),
            soft: Vec::new(),
            cost: 0,
        }
    }

    fn keyed(&self, kind: &str) -> bool {
        self.inp.keyed.contains(kind)
    }

    fn stage_rigor(&self, stage: &str) -> Rigor {
        match self.pack.policy(stage) {
            None | Some(Rigor::Forbidden) => Rigor::Forbidden,
            Some(pol) => match self.mode {
                Mode::PackDefault => pol,
                Mode::Live(r) => r,
            },
        }
    }

    /// Step 2: `R(stage, ctx)` = baseline ∪ the artifacts of every row whose `when` holds, each at
    /// its rigor.
    fn required(
        &mut self,
        stage: &'a str,
    ) -> Result<Vec<(&'a str, Rigor, Origin<'a>)>, LatticeError> {
        let Some(sec) = self.pack.pack.stage(stage) else {
            return Ok(Vec::new());
        };
        let r = self.stage_rigor(stage);
        let mut req: Vec<(&'a str, Rigor, Origin<'a>)> = Vec::new();
        for k in &sec.baseline {
            Self::put(&mut req, k, r, Origin::Baseline);
        }
        for (index, row) in sec.require.iter().enumerate() {
            if self.f.truth(self.inp, &row.when, r, None)? != Tri::True {
                continue;
            }
            let mut rr = row.rigor.unwrap_or(r);
            if rr < r {
                // M11
                rr = r; // I4: row rigor raises, never lowers
            }
            for k in &row.artifacts {
                Self::put(
                    &mut req,
                    k,
                    rr,
                    Origin::Row {
                        index,
                        when: &row.when,
                    },
                );
            }
        }
        Ok(req)
    }

    /// `required`'s `OrderedDict`: a kind already present keeps its position and takes the new
    /// rigor.
    fn put<'k>(req: &mut Vec<(&'k str, Rigor, Origin<'k>)>, k: &'k str, rr: Rigor, o: Origin<'k>) {
        match req.iter_mut().find(|(kk, _, _)| *kk == k) {
            Some(slot) => {
                slot.1 = rr;
                slot.2 = o;
            }
            None => req.push((k, rr, o)),
        }
    }

    fn render_why(&self, why: Why<'_>) -> (String, Option<String>) {
        match why {
            Why::R(stage, Origin::Baseline) => (
                "R".to_owned(),
                Some(format!("{}: stage.{stage}.baseline", self.pack.id)),
            ),
            Why::R(stage, Origin::Row { index, when }) => (
                "R".to_owned(),
                Some(format!(
                    "{}: stage.{stage}.require[{index}] when = {when:?}",
                    self.pack.id
                )),
            ),
            Why::RequiresOf(a) => (format!("requires of {}", self.inp.acts[a].id), None),
        }
    }

    /// Step 4 for one missing artifact (instance).
    fn producer(
        &mut self,
        kind: &'a str,
        stage: &'a str,
        rigor: Rigor,
        why: Why<'a>,
        inst: Option<&'static str>,
    ) -> Result<Option<Got>, LatticeError> {
        if self.keyed(kind) && inst.is_none() {
            let mut got = Vec::new();
            for t in self.f.tc.sorted_tools() {
                got.push(self.producer(kind, stage, rigor, why, Some(t))?);
            }
            return Ok(if got.iter().all(Option::is_some) {
                got[0]
            } else {
                None
            });
        }
        let key = (kind, stage, inst);
        if self.visiting.contains(&key) {
            return Ok(None);
        }
        if self.inp.roots.contains(kind) || self.inp.passes.contains(kind) {
            // A pass-sourced kind: produced by its pass, never by a catalog activity (R1).
            return Ok(Some(Got::Root));
        }
        let inp = self.inp;
        let mut adm = Vec::new();
        let mut fails = Vec::new();
        for &a in inp.candidates(kind, stage) {
            let v = self.f.admissible(inp, a, rigor, inst)?;
            if v == Verdict::Admissible {
                adm.push(a);
            } else {
                fails.push((a, v));
            }
        }
        let suffix = inst.map_or_else(String::new, |i| format!("{{{i}}}"));
        if adm.is_empty() {
            let other = inp
                .producers_of(kind)
                .iter()
                .map(|&a| &inp.acts[a])
                .filter(|a| a.stage != stage)
                .map(|a| format!("{}@{}", a.id, a.stage))
                .collect();
            let (why, origin) = self.render_why(why);
            self.unsat.push(Unsat {
                stage: stage.to_owned(),
                kind: format!("{kind}{suffix}"),
                rigor,
                why,
                origin,
                failed: fails
                    .iter()
                    .map(|&(a, v)| format!("{}:{}", inp.acts[a].id, reason(&inp.acts[a], v)))
                    .collect(),
                other_stage: other,
                forbidden_stage: false,
            });
            return Ok(None);
        }
        adm.sort_by(|&x, &y| {
            (inp.acts[x].rank, &inp.acts[x].id).cmp(&(inp.acts[y].rank, &inp.acts[y].id))
        });
        let top = adm[0];
        let same: Vec<&str> = adm
            .iter()
            .filter(|&&a| inp.acts[a].rank == inp.acts[top].rank)
            .map(|&a| inp.acts[a].id.as_str())
            .collect();
        if same.len() > 1 {
            // M9
            let mut tie: Vec<String> = same.iter().map(|s| (*s).to_owned()).collect();
            tie.sort();
            self.ties
                .push((stage.to_owned(), format!("{kind}{suffix}"), tie));
        }
        self.deviation_reachable.extend(adm[1..].iter().copied());
        if self.selected_inst.contains(&(top, inst)) {
            return Ok(Some(Got::Act(top)));
        }
        self.visiting.insert(key);
        self.selected_inst.insert((top, inst));
        if self.selected_set.insert(top) {
            self.selected.push(top);
        }
        for k in &inp.acts[top].provides {
            let i = if self.keyed(k) { inst } else { None };
            self.provided.insert((k.as_str(), i));
        }
        for (rk, optional) in &inp.acts[top].requires {
            if *optional {
                continue;
            }
            let rk = rk.as_str();
            let rinst = if self.keyed(rk) { inst } else { None };
            if self.provided.contains(&(rk, rinst))
                || inp.roots.contains(rk)
                || inp.passes.contains(rk)
            {
                continue;
            }
            // I1: the same stage first, then any non-forbidden stage, in pack order.
            let mut got = None;
            let mut tried: Vec<String> = Vec::new();
            let mut stages: Vec<&'a str> = vec![stage];
            stages.extend(
                self.pack
                    .order
                    .iter()
                    .map(|(s, _)| s.as_str())
                    .filter(|s| *s != stage && self.stage_rigor(s) != Rigor::Forbidden),
            );
            for s in stages {
                if inp.candidates(rk, s).is_empty() {
                    continue;
                }
                let sr = if s == stage {
                    rigor
                } else {
                    self.stage_rigor(s)
                };
                let before = self.unsat.len();
                got = self.producer(rk, s, sr, Why::RequiresOf(top), rinst)?;
                if got.is_some() {
                    break;
                }
                for u in &self.unsat[before..] {
                    if u.failed.is_empty() {
                        tried.push(format!("{}:transitive@{s}", u.kind));
                    } else {
                        tried.extend(u.failed.iter().map(|f| format!("{f}@{s}")));
                    }
                }
                self.unsat.truncate(before);
            }
            if got.is_none() {
                let forb: Vec<String> = inp
                    .producers_of(rk)
                    .iter()
                    .map(|&a| &inp.acts[a])
                    .filter(|a| self.stage_rigor(&a.stage) == Rigor::Forbidden)
                    .map(|a| format!("{}@{}", a.id, a.stage))
                    .collect();
                let forbidden_stage = !forb.is_empty() && tried.is_empty();
                self.unsat.push(Unsat {
                    stage: stage.to_owned(),
                    kind: rk.to_owned(),
                    rigor,
                    why: format!("requires of {}", inp.acts[top].id),
                    origin: None,
                    failed: tried,
                    other_stage: forb,
                    forbidden_stage,
                });
            }
        }
        self.visiting.remove(&key);
        Ok(Some(Got::Act(top)))
    }

    fn run(mut self) -> Result<Self, LatticeError> {
        let pack = self.pack;
        for (stage, pol) in &pack.order {
            if *pol == Rigor::Forbidden || stage == "triage" {
                continue;
            }
            for (kind, rigor, origin) in self.required(stage)? {
                if !self.keyed(kind) && self.provided.contains(&(kind, None)) {
                    continue;
                }
                self.producer(kind, stage, rigor, Why::R(stage, origin), None)?;
            }
        }
        // step 5: acyclic over the selected DAG (requires -> provides edges)
        self.hard = cycles(self.inp, &self.selected, false); // M25
        self.soft = cycles(self.inp, &self.selected, true);
        let mut cost = 0;
        for &a in &self.selected {
            let act = &self.inp.acts[a];
            cost += act
                .cost_class
                .as_deref()
                .and_then(cost_points)
                .ok_or_else(|| LatticeError::Shape {
                    source: act.id.clone(),
                    detail: format!("cost_class {:?} is not one of S, M, L, XL", act.cost_class),
                })?;
        }
        self.cost = cost;
        Ok(self)
    }
}

/// Every cycle a depth-first walk over the selected DAG finds. Successors are walked in catalog
/// order where `lattice.py` walks a `set` — see the module doc.
fn cycles(inp: &Inputs, selected: &[usize], include_optional: bool) -> Vec<Vec<usize>> {
    let mut prov: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for &a in selected {
        for k in &inp.acts[a].provides {
            prov.entry(k.as_str()).or_default().push(a);
        }
    }
    let mut edges: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &a in selected {
        for (req, optional) in &inp.acts[a].requires {
            if *optional && !include_optional {
                continue;
            }
            for &p in prov.get(req.as_str()).map_or(&[][..], Vec::as_slice) {
                if p != a {
                    edges.entry(p).or_default().insert(a);
                }
            }
        }
    }
    let mut state = BTreeMap::new();
    let mut cyc = Vec::new();
    for &u in selected {
        if !state.contains_key(&u) {
            dfs(u, &mut Vec::new(), &edges, &mut state, &mut cyc);
        }
    }
    cyc
}

/// `cycles()`'s inner `dfs`: state 1 is on the stack, 2 is finished; a successor on the stack
/// closes a cycle, recorded from its first appearance.
fn dfs(
    u: usize,
    st: &mut Vec<usize>,
    edges: &BTreeMap<usize, BTreeSet<usize>>,
    state: &mut BTreeMap<usize, u8>,
    cyc: &mut Vec<Vec<usize>>,
) {
    state.insert(u, 1);
    st.push(u);
    if let Some(next) = edges.get(&u) {
        for &v in next {
            match state.get(&v) {
                Some(1) => {
                    let at = st.iter().position(|x| *x == v).unwrap_or(0);
                    let mut c = st[at..].to_vec();
                    c.push(v);
                    cyc.push(c);
                }
                None => dfs(v, st, edges, state, cyc),
                Some(_) => {}
            }
        }
    }
    st.pop();
    state.insert(u, 2);
}

// ------------------------------------------------------------------ the report

/// A map that iterates in insertion order — `dict`'s order, which `lattice.py`'s stable sorts
/// break ties by.
#[derive(Debug, Clone)]
struct Ordered<K: Ord + Clone, V> {
    index: BTreeMap<K, usize>,
    entries: Vec<(K, V)>,
}

impl<K: Ord + Clone, V> Default for Ordered<K, V> {
    fn default() -> Self {
        Self {
            index: BTreeMap::new(),
            entries: Vec::new(),
        }
    }
}

impl<K: Ord + Clone, V: Default> Ordered<K, V> {
    const fn new() -> Self {
        Self {
            index: BTreeMap::new(),
            entries: Vec::new(),
        }
    }

    fn slot(&mut self, k: &K) -> &mut V {
        let i = match self.index.get(k) {
            Some(i) => *i,
            None => {
                self.entries.push((k.clone(), V::default()));
                self.index.insert(k.clone(), self.entries.len() - 1);
                self.entries.len() - 1
            }
        };
        &mut self.entries[i].1
    }
}

/// `(pack, stage, kind)`.
type Psk = (String, String, String);

/// The first example of one unsatisfiable `(pack, stage, kind)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsatExample {
    /// The facet vector, as `ctx_label` renders it.
    pub vector: String,
    /// The rigor mode of the run.
    pub mode: &'static str,
    /// The rigor the requirement was demanded at.
    pub rigor: Rigor,
    pub why: String,
    /// For an R-level requirement: the pack row (or baseline) that demanded it.
    pub origin: Option<String>,
    pub failed: Vec<String>,
    pub other_stage: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct PackMode {
    pairs: u64,
    unsat_pairs: u64,
    contexts: u64,
    cost_sum: u64,
    cost_max: u64,
    overruns: u64,
    hard_cycles: u64,
    soft_cycles: u64,
}

/// What the lattice walked and found.
#[derive(Debug, Clone)]
pub struct Report {
    tag: String,
    header: Vec<String>,
    pack_ids: Vec<String>,
    catalog_ids: Vec<String>,
    per: BTreeMap<(String, &'static str), PackMode>,
    pairs_tc: BTreeMap<(&'static str, &'static str), (u64, u64)>,
    unsat_by: Ordered<(String, &'static str), Ordered<Psk, u64>>,
    unsat_forbidden: BTreeMap<Psk, u64>,
    unsat_other_stage: BTreeMap<Psk, u64>,
    unsat_examples: BTreeMap<Psk, UnsatExample>,
    hard_examples: BTreeMap<String, Vec<String>>,
    soft_examples: BTreeMap<String, Vec<String>>,
    mixed: MixedCounts,
    binary_only: BTreeMap<Psk, u64>,
    mixed_contexts: u64,
    mixed_extra: u64,
    mixed_extra_stage: u64,
    diff: Ordered<Psk, u64>,
    sdiff: Ordered<(String, String), u64>,
    sweep_pairs: u64,
    sweep_unsat: u64,
    sweep_selected: BTreeMap<(&'static str, String), u64>,
    ties: BTreeMap<Psk, BTreeSet<Vec<String>>>,
    dead: Vec<String>,
    devreach: BTreeSet<String>,
    live_prereg: BTreeSet<String>,
    selected_ever: BTreeMap<String, BTreeSet<(String, &'static str)>>,
    kinds_without_pack: Vec<&'static str>,
    null_verdicts: BTreeMap<String, u64>,
    toolchains: Vec<&'static str>,
    surface_rows: usize,
}

/// `>` a percentage, in integers: `n/d > p%`. The AC-101 (`> 5%`) and AC-102 (`> 20%`) kill lines.
#[must_use]
pub const fn kill_line_fires(n: u64, d: u64, percent: u64) -> bool {
    n * 100 > d * percent
}

fn pct(n: u64, d: u64) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts here are far below 2^53; the division reproduces Python's int/int true division"
    )]
    let (n, d) = ((100 * n) as f64, d as f64);
    n / d
}

fn ratio(n: u64, d: u64) -> f64 {
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts here are far below 2^53; the division reproduces Python's int/int true division"
    )]
    let (n, d) = (n as f64, d as f64);
    n / d
}

/// Python's `repr` of a `str`.
fn py_str(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c as u32 == 0x7f => {
                let _ = write!(out, "\\x{:02x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

/// Python's `repr` of a `list` of `str`.
fn py_list(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| py_str(s)).collect();
    format!("[{}]", inner.join(", "))
}

/// Python's `repr` of a `tuple` of `str`.
fn py_tuple(items: &[String]) -> String {
    let inner: Vec<String> = items.iter().map(|s| py_str(s)).collect();
    if inner.len() == 1 {
        format!("({},)", inner[0])
    } else {
        format!("({})", inner.join(", "))
    }
}

/// `sweep=True`'s band sets: one axis `unknown`, the others `low`.
fn sweep_band_sets() -> Vec<[&'static str; 4]> {
    (0..ORDINALS.len())
        .map(|i| {
            let mut b = ["low"; 4];
            b[i] = "unknown";
            b
        })
        .collect()
}

fn lattice_band_sets() -> Vec<[&'static str; 4]> {
    let mut out = Vec::with_capacity(81);
    for a in BANDS {
        for b in BANDS {
            for c in BANDS {
                for d in BANDS {
                    out.push([a, b, c, d]);
                }
            }
        }
    }
    out
}

fn cos_for(kind: &str) -> &'static [&'static str] {
    if kind == "bug" {
        &["standard", "expedite"]
    } else {
        &["standard"]
    }
}

fn check_axes(axes: &Axes) -> Result<(), LatticeError> {
    for row in &axes.toolchains {
        for t in &row.tools {
            if change_of(t).is_none() {
                return Err(LatticeError::Shape {
                    source: "axes".to_owned(),
                    detail: format!("toolchain {t:?} has no CHANGE_OF entry"),
                });
            }
        }
    }
    if axes.bools.len() != 8 || axes.covering_rows.is_empty() {
        return Err(LatticeError::Shape {
            source: "axes".to_owned(),
            detail: "the covering array is not over the eight registered booleans".to_owned(),
        });
    }
    Ok(())
}

/// `(stage, kind)`.
type StageKind = (String, String);

/// `unsat_sets`' key: `(pack, mode, vector without toolchain, toolchain)`.
type RunKey = (usize, &'static str, SansTc, usize);

/// `mixed`: `(pack, mode)` → `(stage, kind)` → count.
type MixedCounts = BTreeMap<(String, &'static str), BTreeMap<StageKind, u64>>;

/// The keys `unsat_sets` is indexed by: the vector without its toolchain.
type SansTc = (
    &'static str,
    &'static str,
    [&'static str; 4],
    usize,
    Option<usize>,
);

/// Run the lattice over the packs `only` names (every pack when `None`), at every mode.
///
/// # Errors
///
/// [`LatticeError::NoPacks`] when the selection is empty — never a report over nothing — and any
/// other [`LatticeError`] the enumeration reaches. *(dated 2026-09-25, s8b refuting round 6)* Those
/// are: [`LatticeError::Shape`] with `source: "axes"` when `check_axes` refuses the axes (a
/// toolchain with no `CHANGE_OF` entry, or a covering array not over the eight registered
/// booleans), with `source: "facets.toml"` when a vector names an ordinal band the registry does
/// not declare, and with an activity id as `source` when a selected activity's `cost_class` is not
/// one of S, M, L, XL; and [`LatticeError::Predicate`] when a predicate the enumeration reaches does
/// not parse or does not evaluate.
#[expect(
    clippy::too_many_lines,
    reason = "the enumeration and its aggregation are `lattice.py`'s `main`, ported as one block so \
              the two can be read side by side"
)]
pub fn run(inputs: &Inputs, axes: &Axes, only: Option<&[&str]>) -> Result<Report, LatticeError> {
    check_axes(axes)?;
    let packs: Vec<&PackIn> = inputs
        .packs
        .iter()
        .filter(|p| only.is_none_or(|o| o.contains(&p.id.as_str())))
        .collect();
    if packs.is_empty() {
        // M8
        return Err(LatticeError::NoPacks {
            dir: inputs.packs_dir.display().to_string(),
            requested: only.map(|o| o.iter().map(|s| (*s).to_owned()).collect()),
        });
    }
    let pack_files: Vec<(PathBuf, Vec<u8>)> = packs
        .iter()
        .map(|p| (p.path.clone(), p.bytes.clone()))
        .collect();
    let tag = format!(
        "cat:{} fac+art:{} pk:{}",
        inputs.catalog_digest,
        inputs.facets_digest,
        digest_paths(&pack_files)
    );
    let pack_ids: Vec<String> = packs.iter().map(|p| p.id.clone()).collect();
    let pack_digests: Vec<String> = packs
        .iter()
        .map(|p| {
            format!(
                "{}={}",
                p.id,
                digest_paths(&[(p.path.clone(), p.bytes.clone())])
            )
        })
        .collect();
    let toolchain_names: Vec<String> = axes.toolchains.iter().map(|t| py_str(t.name)).collect();
    let bool_names: Vec<String> = axes.bools.iter().map(|b| py_str(b)).collect();
    let header = vec![
        format!(
            "P6 lattice run; catalog={} activities; packs={}; pack digests: {}",
            inputs.acts.len(),
            py_list(&pack_ids),
            pack_digests.join(" ")
        ),
        format!(
            "lattice v2: kinds={} rigor=['optional', 'recommended', 'mandatory']+pack-default \
             ordinals=['criticality', 'novelty', 'size', 'blast_radius'] bands=['low', 'med', \
             'high']^4=81 toolchains=[{}] bool-covering-rows={} (pairwise over [{}]); keyed kinds={}; \
             roots={}; + unknown sweep (4 rows, pack-default)",
            KINDS.len(),
            toolchain_names.join(", "),
            axes.covering_rows.len(),
            bool_names.join(", "),
            py_list(&inputs.keyed.iter().cloned().collect::<Vec<_>>()),
            py_list(&inputs.roots.iter().cloned().collect::<Vec<_>>()),
        ),
    ];

    let mut rep = Report {
        tag,
        header,
        pack_ids,
        catalog_ids: inputs.acts.iter().map(|a| a.id.clone()).collect(),
        per: BTreeMap::new(),
        pairs_tc: BTreeMap::new(),
        unsat_by: Ordered::new(),
        unsat_forbidden: BTreeMap::new(),
        unsat_other_stage: BTreeMap::new(),
        unsat_examples: BTreeMap::new(),
        hard_examples: BTreeMap::new(),
        soft_examples: BTreeMap::new(),
        mixed: BTreeMap::new(),
        binary_only: BTreeMap::new(),
        mixed_contexts: 0,
        mixed_extra: 0,
        mixed_extra_stage: 0,
        diff: Ordered::new(),
        sdiff: Ordered::new(),
        sweep_pairs: 0,
        sweep_unsat: 0,
        sweep_selected: BTreeMap::new(),
        ties: BTreeMap::new(),
        dead: Vec::new(),
        devreach: BTreeSet::new(),
        live_prereg: BTreeSet::new(),
        selected_ever: BTreeMap::new(),
        kinds_without_pack: Vec::new(),
        null_verdicts: BTreeMap::new(),
        toolchains: axes.toolchains.iter().map(|t| t.name).collect(),
        surface_rows: axes.surfaces.as_ref().map_or(0, Vec::len),
    };
    let mut pack_for_kind: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();
    let mut unsat_sets: Vec<(RunKey, BTreeSet<StageKind>)> = Vec::new();
    let mut devreach: BTreeSet<usize> = BTreeSet::new();

    // ---- enumerate
    let bands_all = lattice_band_sets();
    for kind in KINDS {
        for mode in Mode::ALL {
            for bands in &bands_all {
                for tc in 0..axes.toolchains.len() {
                    for row in 0..axes.covering_rows.len() {
                        for surf in axes.surface_rows() {
                            for cos in cos_for(kind) {
                                let fv = Fv {
                                    kind,
                                    cos,
                                    bands: *bands,
                                    tc,
                                    row,
                                    surf,
                                };
                                let mut f = FCtx::new(inputs, axes, fv)?;
                                for (pi, &pack) in packs.iter().enumerate() {
                                    if f.truth(inputs, &pack.applies, Rigor::Optional, None)?
                                        != Tri::True
                                    {
                                        continue;
                                    }
                                    pack_for_kind
                                        .entry(kind)
                                        .or_default()
                                        .insert(pack.id.clone());
                                    let r = Run::new(inputs, pack, &mut f, mode).run()?;
                                    let tname = axes.toolchains[tc].name;
                                    let key = (pack.id.clone(), mode.as_str());
                                    let pm = rep.per.entry(key.clone()).or_default();
                                    pm.contexts += 1;
                                    let bad: BTreeSet<&str> =
                                        r.unsat.iter().map(|u| u.stage.as_str()).collect();
                                    let stages = pack.order.iter().filter(|(s, p)| {
                                        *p != Rigor::Forbidden
                                            && s != "triage"
                                            && pack.pack.stage(s).is_some()
                                    });
                                    for (s, _) in stages {
                                        pm.pairs += 1;
                                        let tcs =
                                            rep.pairs_tc.entry((mode.as_str(), tname)).or_default();
                                        tcs.0 += 1;
                                        if bad.contains(s.as_str()) {
                                            pm.unsat_pairs += 1;
                                            tcs.1 += 1;
                                        }
                                    }
                                    for u in &r.unsat {
                                        let k: Psk =
                                            (pack.id.clone(), u.stage.clone(), u.kind.clone());
                                        *rep.unsat_by.slot(&key).slot(&k) += 1;
                                        if u.forbidden_stage {
                                            *rep.unsat_forbidden.entry(k.clone()).or_default() += 1;
                                        }
                                        if !u.other_stage.is_empty() && u.failed.is_empty() {
                                            *rep.unsat_other_stage.entry(k.clone()).or_default() +=
                                                1;
                                        }
                                        rep.unsat_examples.entry(k.clone()).or_insert_with(|| {
                                            UnsatExample {
                                                vector: ctx_label(&fv, tname, axes),
                                                mode: mode.as_str(),
                                                rigor: u.rigor,
                                                why: u.why.clone(),
                                                origin: u.origin.clone(),
                                                failed: u.failed.iter().take(8).cloned().collect(),
                                                other_stage: u
                                                    .other_stage
                                                    .iter()
                                                    .take(4)
                                                    .cloned()
                                                    .collect(),
                                            }
                                        });
                                        if tname == "unreal"
                                            && ["Changeset", "ImplReport", "ChangeSummary"]
                                                .contains(&u.kind.as_str())
                                        {
                                            *rep.binary_only.entry(k.clone()).or_default() += 1;
                                        }
                                        if tname == "node+unreal" {
                                            *rep.mixed
                                                .entry(key.clone())
                                                .or_default()
                                                .entry((u.stage.clone(), u.kind.clone()))
                                                .or_default() += 1;
                                        }
                                    }
                                    unsat_sets.push((
                                        (pi, mode.as_str(), (kind, cos, *bands, row, surf), tc),
                                        r.unsat
                                            .iter()
                                            .map(|u| (u.stage.clone(), u.kind.clone()))
                                            .collect(),
                                    ));
                                    for &a in &r.selected {
                                        rep.selected_ever
                                            .entry(inputs.acts[a].id.clone())
                                            .or_default()
                                            .insert(key.clone());
                                    }
                                    devreach.extend(r.deviation_reachable.iter().copied());
                                    for (stage, k, tie) in &r.ties {
                                        rep.ties
                                            .entry((pack.id.clone(), stage.clone(), k.clone()))
                                            .or_default()
                                            .insert(tie.clone());
                                    }
                                    if pack.pack.appetite().is_some_and(|ap| {
                                        i64::try_from(r.cost).unwrap_or(i64::MAX) > ap
                                    }) {
                                        pm.overruns += 1;
                                    }
                                    let ids = |c: &[usize]| -> Vec<String> {
                                        c.iter().map(|&a| inputs.acts[a].id.clone()).collect()
                                    };
                                    if let Some(c) = r.hard.first() {
                                        pm.hard_cycles += 1;
                                        rep.hard_examples
                                            .entry(pack.id.clone())
                                            .or_insert_with(|| ids(c));
                                    }
                                    if let Some(c) = r.soft.first() {
                                        pm.soft_cycles += 1;
                                        rep.soft_examples
                                            .entry(pack.id.clone())
                                            .or_insert_with(|| ids(c));
                                    }
                                    pm.cost_sum += r.cost;
                                    pm.cost_max = pm.cost_max.max(r.cost);
                                }
                                merge_nulls(&mut rep.null_verdicts, f.nulls);
                            }
                        }
                    }
                }
            }
        }
    }

    // ---- the mixed Node+UE differential
    let index: BTreeMap<(&str, &'static str, SansTc, &str), &BTreeSet<StageKind>> = unsat_sets
        .iter()
        .map(|((pi, mode, sans, tc), set)| {
            (
                (
                    packs[*pi].id.as_str(),
                    *mode,
                    *sans,
                    axes.toolchains[*tc].name,
                ),
                set,
            )
        })
        .collect();
    let empty = BTreeSet::new();
    for ((pi, mode, sans, tc), us) in &unsat_sets {
        if axes.toolchains[*tc].name != "node+unreal" {
            continue;
        }
        rep.mixed_contexts += 1;
        let pid = packs[*pi].id.as_str();
        let base = index
            .get(&(twin(pid), *mode, *sans, "node"))
            .copied()
            .unwrap_or(&empty);
        let extra: Vec<&(String, String)> = us.difference(base).collect();
        if !extra.is_empty() {
            rep.mixed_extra += 1;
        }
        for (st, k) in extra {
            *rep.diff.slot(&(pid.to_owned(), st.clone(), k.clone())) += 1;
        }
        let us_st: BTreeSet<&str> = us.iter().map(|(s, _)| s.as_str()).collect();
        let base_st: BTreeSet<&str> = base.iter().map(|(s, _)| s.as_str()).collect();
        let sextra: Vec<&&str> = us_st.difference(&base_st).collect();
        if !sextra.is_empty() {
            rep.mixed_extra_stage += 1;
        }
        for st in sextra {
            *rep.sdiff.slot(&(pid.to_owned(), (*st).to_owned())) += 1;
        }
    }

    // ---- the unknown sweep (D6)
    let sweep = sweep_band_sets(); // M12
    for kind in KINDS {
        for bands in &sweep {
            for tc in 0..axes.toolchains.len() {
                for row in 0..axes.covering_rows.len() {
                    for surf in axes.surface_rows() {
                        for cos in cos_for(kind) {
                            let fv = Fv {
                                kind,
                                cos,
                                bands: *bands,
                                tc,
                                row,
                                surf,
                            };
                            let ax = bands.iter().position(|b| *b == "unknown").unwrap_or(0);
                            let mut low = fv;
                            low.bands[ax] = "low";
                            let mut f = FCtx::new(inputs, axes, fv)?;
                            let mut fl = FCtx::new(inputs, axes, low)?;
                            for &pack in &packs {
                                if f.truth(inputs, &pack.applies, Rigor::Optional, None)?
                                    != Tri::True
                                {
                                    continue;
                                }
                                let r = Run::new(inputs, pack, &mut f, Mode::PackDefault).run()?;
                                let bad: BTreeSet<&str> =
                                    r.unsat.iter().map(|u| u.stage.as_str()).collect();
                                for (s, _) in pack.order.iter().filter(|(s, p)| {
                                    *p != Rigor::Forbidden
                                        && s != "triage"
                                        && pack.pack.stage(s).is_some()
                                }) {
                                    rep.sweep_pairs += 1;
                                    rep.sweep_unsat += u64::from(bad.contains(s.as_str()));
                                }
                                let base =
                                    Run::new(inputs, pack, &mut fl, Mode::PackDefault).run()?;
                                for a in r.selected_set.difference(&base.selected_set) {
                                    *rep.sweep_selected
                                        .entry((ORDINALS[ax], inputs.acts[*a].id.clone()))
                                        .or_default() += 1;
                                }
                            }
                            merge_nulls(&mut rep.null_verdicts, f.nulls);
                            merge_nulls(&mut rep.null_verdicts, fl.nulls);
                        }
                    }
                }
            }
        }
    }

    // ---- dead activities, and what survives the pre-registered packs
    rep.devreach = devreach
        .iter()
        .map(|&a| inputs.acts[a].id.clone())
        .collect();
    rep.dead = dead_activities(inputs, &rep.selected_ever); // M26
    rep.live_prereg = rep
        .selected_ever
        .iter()
        .filter(|(_, s)| s.iter().any(|(p, _)| PREREG_PACKS.contains(&p.as_str())))
        .map(|(a, _)| a.clone())
        .collect();
    rep.kinds_without_pack = KINDS
        .iter()
        .copied()
        .filter(|k| pack_for_kind.get(k).is_none_or(BTreeSet::is_empty))
        .collect();
    Ok(rep)
}

fn merge_nulls(into: &mut BTreeMap<String, u64>, from: BTreeMap<String, u64>) {
    for (k, v) in from {
        *into.entry(k).or_default() += v;
    }
}

fn dead_activities(
    inputs: &Inputs,
    selected_ever: &BTreeMap<String, BTreeSet<(String, &'static str)>>,
) -> Vec<String> {
    let mut dead: Vec<String> = inputs
        .acts
        .iter()
        .filter(|a| !selected_ever.contains_key(&a.id))
        .map(|a| a.id.clone())
        .collect();
    dead.sort();
    dead
}

impl Report {
    /// `cat:… fac+art:… pk:…` — the digest tag every line carries (METHOD-P §0 rule 2).
    #[must_use]
    pub fn tag(&self) -> &str {
        &self.tag
    }

    /// The pack ids walked.
    #[must_use]
    pub fn packs(&self) -> &[String] {
        &self.pack_ids
    }

    /// `(pack, stage, kind)` → the first example of its unsatisfiability.
    #[must_use]
    pub const fn unsat_examples(&self) -> &BTreeMap<(String, String, String), UnsatExample> {
        &self.unsat_examples
    }

    /// Run contexts at `mode` (`"pack-default"`, `"optional"`, …), summed over packs.
    #[must_use]
    pub fn contexts(&self, mode: &str) -> u64 {
        self.per
            .iter()
            .filter(|((_, m), _)| *m == mode)
            .map(|(_, pm)| pm.contexts)
            .sum()
    }

    /// `(stage, ctx)` pairs at `mode`, summed over packs.
    #[must_use]
    pub fn pairs(&self, mode: &str) -> u64 {
        self.per
            .iter()
            .filter(|((_, m), _)| *m == mode)
            .map(|(_, pm)| pm.pairs)
            .sum()
    }

    /// Unsatisfiable pairs at `mode`, summed over packs.
    #[must_use]
    pub fn unsat_pairs(&self, mode: &str) -> u64 {
        self.per
            .iter()
            .filter(|((_, m), _)| *m == mode)
            .map(|(_, pm)| pm.unsat_pairs)
            .sum()
    }

    /// Runs with a hard cycle, summed over every pack and mode.
    #[must_use]
    pub fn hard_cycle_runs(&self) -> u64 {
        self.per.values().map(|pm| pm.hard_cycles).sum()
    }

    /// Each pack's first hard cycle, by activity id.
    #[must_use]
    pub const fn hard_cycles(&self) -> &BTreeMap<String, Vec<String>> {
        &self.hard_examples
    }

    /// Each pack's first soft cycle (through a `?` requirement), by activity id.
    #[must_use]
    pub const fn soft_cycles(&self) -> &BTreeMap<String, Vec<String>> {
        &self.soft_examples
    }

    /// `(pack, stage, kind)` → every tied set of ids seen.
    #[must_use]
    pub const fn ties(&self) -> &BTreeMap<(String, String, String), BTreeSet<Vec<String>>> {
        &self.ties
    }

    /// Activities never selected in any pack × context, sorted.
    #[must_use]
    pub fn dead(&self) -> &[String] {
        &self.dead
    }

    /// How many activities the catalog holds — the dead fraction's denominator.
    #[must_use]
    pub const fn catalog_len(&self) -> usize {
        self.catalog_ids.len()
    }

    /// The activities selected at least once under `pack`, at any mode.
    #[must_use]
    pub fn selected_in(&self, pack: &str) -> BTreeSet<&str> {
        self.selected_ever
            .iter()
            .filter(|(_, s)| s.iter().any(|(p, _)| p == pack))
            .map(|(a, _)| a.as_str())
            .collect()
    }

    /// Every predicate that evaluated to `null` somewhere in the walk, and how often. A `null`
    /// never admits, so a figure this moves is traceable to the predicate named here.
    #[must_use]
    pub const fn null_verdicts(&self) -> &BTreeMap<String, u64> {
        &self.null_verdicts
    }

    /// The toolchain alternatives walked.
    #[must_use]
    pub fn toolchains(&self) -> &[&'static str] {
        &self.toolchains
    }

    /// How many `surfaces` alternatives were walked (0 when unbound).
    #[must_use]
    pub const fn surface_rows(&self) -> usize {
        self.surface_rows
    }

    /// **AC-103's zero tolerance**: every rank tie and every hard cycle, as an error line naming
    /// what it is. Empty is the pass.
    #[must_use]
    pub fn ac103_errors(&self) -> Vec<String> {
        let mut out = Vec::new();
        for ((pack, stage, kind), sets) in &self.ties {
            for tie in sets {
                out.push(format!(
                    "rank tie: pack {pack} stage {stage} kind {kind}: {} share one method_rank and \
                     no rule breaks the tie (ADR-0073; AC-103)",
                    tie.join(", ")
                ));
            }
        }
        for (pack, c) in &self.hard_examples {
            out.push(format!(
                "hard cycle: pack {pack}: {} — unconditional requires close a loop over the \
                 selected DAG (AC-103)",
                c.join(" -> ")
            ));
        }
        out
    }

    /// The report `lattice.py` prints, one line each, digest tag included.
    #[must_use]
    #[expect(
        clippy::too_many_lines,
        reason = "the report section of `lattice.py`'s `main`, kept line for line"
    )]
    pub fn lines(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        let mut line = |s: String| out.push(format!("{} | {s}", self.tag));
        for h in &self.header {
            line(h.clone());
        }
        line(String::new());
        line("== unsatisfiable (stage, ctx) pairs per pack × rigor mode ==".to_owned());
        let mut tot: BTreeMap<(&str, &str), u64> = BTreeMap::new();
        for ((pid, mode), pm) in &self.per {
            if pm.pairs == 0 {
                continue;
            }
            let (u, n) = (pm.unsat_pairs, pm.pairs);
            let pre = if PREREG_PACKS.contains(&pid.as_str()) {
                "prereg"
            } else {
                "supp"
            };
            line(format!(
                "{pid:14} {mode:13} {pre:6} pairs={n:6} unsat={u:6} ({:5.1}%) contexts={:5} \
                 mean-cost={:6.1} max-cost={:4} appetite-overruns={} hard-cycles={} \
                 soft-cycles(optional-edge)={}",
                pct(u, n),
                pm.contexts,
                ratio(pm.cost_sum, pm.contexts.max(1)),
                pm.cost_max,
                pm.overruns,
                pm.hard_cycles,
                pm.soft_cycles
            ));
            *tot.entry(("all", mode)).or_default() += u;
            *tot.entry(("alln", mode)).or_default() += n;
            if PREREG_PACKS.contains(&pid.as_str()) {
                *tot.entry(("pre", mode)).or_default() += u;
                *tot.entry(("pren", mode)).or_default() += n;
            }
        }
        for mode in Mode::ALL.map(Mode::as_str) {
            let g = |k: &str| tot.get(&(k, mode)).copied().unwrap_or(0);
            line(format!(
                "TOTAL prereg-8 {mode:13} unsat={}/{} ({:.1}%)   all-12 unsat={}/{} ({:.1}%)",
                g("pre"),
                g("pren"),
                pct(g("pre"), g("pren").max(1)),
                g("all"),
                g("alln"),
                pct(g("all"), g("alln").max(1))
            ));
        }
        line(String::new());
        line("== unsatisfiable pairs by rigor mode × toolchain slice (all packs) ==".to_owned());
        for ((mode, tn), (n, u)) in &self.pairs_tc {
            line(format!(
                "{mode:13} {tn:12} unsat={u:6}/{n:6} ({:5.1}%)",
                pct(*u, *n)
            ));
        }
        line(String::new());
        line(
            "== unsatisfiable by (pack, stage, kind): count of contexts, then why (first example) =="
                .to_owned(),
        );
        let mut agg: Ordered<Psk, u64> = Ordered::new();
        for (_, d) in &self.unsat_by.entries {
            for (k, n) in &d.entries {
                *agg.slot(k) += n;
            }
        }
        let mut agg = agg.entries;
        agg.sort_by_key(|e| std::cmp::Reverse(e.1));
        for (k, n) in &agg {
            let ex = &self.unsat_examples[k];
            line(format!(
                "{:14} {:13} {:22} n={n:5} forbidden-stage={:5} wrong-stage-producer={:5} why={} \
                 failed={} other_stage={} e.g. [{}] {}",
                k.0,
                k.1,
                k.2,
                self.unsat_forbidden.get(k).copied().unwrap_or(0),
                self.unsat_other_stage.get(k).copied().unwrap_or(0),
                py_str(&ex.why),
                py_list(&ex.failed),
                py_list(&ex.other_stage),
                ex.mode,
                ex.vector
            ));
        }
        line(String::new());
        line("== DAG cycles: hard (through unconditional requires) and soft (through `?` requires) — one example per pack ==".to_owned());
        for (pid, c) in &self.hard_examples {
            line(format!("HARD {pid:14} {}", c.join(" -> ")));
        }
        for (pid, c) in &self.soft_examples {
            line(format!("soft {pid:14} {}", c.join(" -> ")));
        }
        line(String::new());
        line("== mixed Node+UE slice: unsatisfiable (stage, kind) counts (METHOD-P §3.4 second kill) ==".to_owned());
        for ((pid, mode), d) in &self.mixed {
            for ((st, k), n) in d {
                line(format!("{pid:14} {mode:13} {st:13} {k:22} n={n}"));
            }
        }
        line(String::new());
        line("== mixed Node+UE DIFFERENTIAL: (stage, kind) unsatisfiable at tc=node+unreal but satisfiable at tc=node for the same context ==".to_owned());
        line(format!(
            "mixed contexts={}; with an extra unsatisfiable (stage,kind) vs pure node={}; with an \
             extra unsatisfiable STAGE vs pure node={} (the kill-criterion reading)",
            self.mixed_contexts, self.mixed_extra, self.mixed_extra_stage
        ));
        let mut diff = self.diff.entries.clone();
        diff.sort_by_key(|e| std::cmp::Reverse(e.1));
        for ((p, s, k), n) in &diff {
            line(format!("(stage,kind) {p:14} {s:13} {k:22} n={n}"));
        }
        let mut sdiff = self.sdiff.entries.clone();
        sdiff.sort_by_key(|e| std::cmp::Reverse(e.1));
        for ((p, s), n) in &sdiff {
            line(format!("STAGE        {p:14} {s:13} n={n}"));
        }
        if diff.is_empty() {
            line("none — every unsatisfiable pair in the mixed slice is also unsatisfiable for pure Node".to_owned());
        }
        line(String::new());
        line("== binary-only (tc=unreal, change={binary}) contexts lacking an implement-stage producer for the text-realised kinds ==".to_owned());
        for ((p, s, k), n) in &self.binary_only {
            line(format!("{p:14} {s:13} {k:22} n={n}"));
        }
        line(String::new());
        line("== unknown sweep (one ordinal axis = unknown, others low; pack-default): unsatisfiable pairs and which activities an `unknown` reaches via `>=` ==".to_owned());
        line(format!(
            "unknown sweep: unsat={}/{} ({:.1}%); activities selected at `<axis> = unknown` but NOT \
             at `<axis> = low` for the same context — i.e. unknown is routed as high by `>=` (F34 B14):",
            self.sweep_unsat,
            self.sweep_pairs,
            pct(self.sweep_unsat, self.sweep_pairs.max(1))
        ));
        for ((ax, aid), n) in &self.sweep_selected {
            line(format!("  {ax:13} {aid:28} n={n}"));
        }
        line(String::new());
        line("== rank ties (validation errors) ==".to_owned());
        if self.ties.is_empty() {
            line("none".to_owned());
        }
        for ((p, s, k), v) in &self.ties {
            let tuples: Vec<String> = v.iter().map(|t| py_tuple(t)).collect();
            line(format!("{p:14} {s:13} {k:22} [{}]", tuples.join(", ")));
        }
        line(String::new());
        line("== dead activities (never selected in any pack × context) ==".to_owned());
        let n_acts = self.catalog_ids.len() as u64;
        let dead_dev: Vec<&str> = self
            .dead
            .iter()
            .filter(|a| self.devreach.contains(*a))
            .map(String::as_str)
            .collect();
        let dead_never: Vec<&str> = self
            .dead
            .iter()
            .filter(|a| !self.devreach.contains(*a))
            .map(String::as_str)
            .collect();
        line(format!(
            "dead={}/{} ({:.1}%)  of which deviation-reachable (an admissible higher-rank candidate in some context)={}  never-admissible-or-never-required={}",
            self.dead.len(),
            n_acts,
            pct(self.dead.len() as u64, n_acts.max(1)),
            dead_dev.len(),
            dead_never.len()
        ));
        line(format!(
            "dead, deviation-reachable: {}",
            dead_dev.join(", ")
        ));
        line(format!("dead, never: {}", dead_never.join(", ")));
        let dead_pre = n_acts - self.live_prereg.len() as u64;
        line(format!(
            "under the eight pre-registered packs only: dead={dead_pre}/{n_acts} ({:.1}%)",
            pct(dead_pre, n_acts.max(1))
        ));
        line(String::new());
        line("== kinds with no pack ==".to_owned());
        line(if self.kinds_without_pack.is_empty() {
            "none".to_owned()
        } else {
            self.kinds_without_pack.join(", ")
        });
        line(String::new());
        line("== per-activity selection count (pack×mode combinations) ==".to_owned());
        let mut ids = self.catalog_ids.clone();
        ids.sort();
        let dead: BTreeSet<&str> = self.dead.iter().map(String::as_str).collect();
        for a in &ids {
            let n = self.selected_ever.get(a).map_or(0, BTreeSet::len);
            line(format!(
                "{a:28} {n:3} {}",
                if dead.contains(a.as_str()) {
                    "DEAD"
                } else {
                    ""
                }
            ));
        }
        out
    }
}

// ------------------------------------------------------------------ --validate

/// One family's totality hole: no member admissible at `rigor` for `kind`/`toolchain`, in
/// `contexts` vectors where the lowest-ranked member is admissible at its floor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    pub rigor: Rigor,
    pub kind: &'static str,
    pub toolchain: &'static str,
    pub contexts: u64,
    /// The first facet vector the hole was seen in.
    pub vector: String,
}

/// One family, as `--validate` reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FamilyTotality {
    pub name: String,
    pub members: usize,
    pub lowest: String,
    pub floor: Rigor,
    pub contexts_checked: u64,
    pub holes: Vec<Hole>,
}

/// `lattice.py --validate`'s findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validation {
    pub families: Vec<FamilyTotality>,
    pub errors: Vec<String>,
}

impl Validation {
    /// The lines `--validate` prints, in its order.
    #[must_use]
    pub fn lines(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .families
            .iter()
            .map(|f| {
                format!(
                    "family {:24} members={} lowest={} floor={} contexts-checked={} holes={}",
                    f.name,
                    f.members,
                    f.lowest,
                    f.floor,
                    f.contexts_checked,
                    f.holes.iter().map(|h| h.contexts).sum::<u64>()
                )
            })
            .collect();
        if self.errors.is_empty() {
            out.push("validate: no errors".to_owned());
        } else {
            out.extend(self.errors.iter().cloned());
        }
        out.push(format!("validate errors={}", self.errors.len()));
        out
    }
}

/// One family's running totality tally.
struct Fam {
    members: Vec<usize>,
    low: usize,
    floor: Rigor,
    n: u64,
    holes: BTreeMap<(&'static str, &'static str, &'static str), (u64, String)>,
}

/// `lattice.py --validate`.
///
/// Every unconditional `requires` resolves (D2), every activity-sourced kind has a producer, and every family is total (D1) — for every vector where the family's
/// lowest-ranked member is admissible at its floor, every live rigor at or above that floor
/// admits some member.
///
/// # Errors
///
/// [`LatticeError`] when a predicate the walk reaches cannot be evaluated. Findings are not
/// errors; they come back in the [`Validation`]. *(dated 2026-09-25, s8b refuting round 6: that
/// named one class only)* Also [`LatticeError::Shape`], with `source: "axes"` when `check_axes`
/// refuses the axes (a toolchain with no `CHANGE_OF` entry, or a covering array not over the eight
/// registered booleans), and with `source: "facets.toml"` when a vector names an ordinal band the
/// registry does not declare.
pub fn validate(inputs: &Inputs, axes: &Axes) -> Result<Validation, LatticeError> {
    check_axes(axes)?;
    let mut errors = Vec::new();
    let mut fams: Vec<(String, Vec<usize>)> = Vec::new();
    for (i, a) in inputs.acts.iter().enumerate() {
        if let Some(f) = &a.family {
            match fams.iter_mut().find(|(n, _)| n == f) {
                Some((_, m)) => m.push(i),
                None => fams.push((f.clone(), vec![i])),
            }
        }
        for (req, optional) in &a.requires {
            if *optional {
                continue;
            }
            if !inputs.roots.contains(req)
                && !inputs.passes.contains(req)
                && inputs.producers_of(req).is_empty()
            {
                errors.push(format!(
                    "requires: {} needs '{req}' — no producer and not source = human (§9c.27)",
                    a.id
                ));
            }
        }
    }
    for k in &inputs.activity_kinds {
        if inputs.producers_of(k).is_empty() {
            errors.push(format!(
                "artifacts.toml: '{k}' is source = activity but nothing provides it"
            ));
        }
    }

    let mut state: Vec<Fam> = fams
        .iter()
        .map(|(_, members)| {
            let low = *members
                .iter()
                .min_by_key(|&&m| inputs.acts[m].rank)
                .unwrap_or(&members[0]);
            Fam {
                members: members.clone(),
                low,
                floor: inputs.acts[low].min,
                n: 0,
                holes: BTreeMap::new(),
            }
        })
        .collect();

    let bands_all = lattice_band_sets();
    for kind in KINDS {
        for bands in &bands_all {
            for tc in 0..axes.toolchains.len() {
                for row in 0..axes.covering_rows.len() {
                    for surf in axes.surface_rows() {
                        let fv = Fv {
                            kind,
                            cos: "standard",
                            bands: *bands,
                            tc,
                            row,
                            surf,
                        };
                        let mut f = FCtx::new(inputs, axes, fv)?;
                        let tname = axes.toolchains[tc].name;
                        for fam in &mut state {
                            let low = &inputs.acts[fam.low];
                            let insts: Vec<Option<&'static str>> = if low
                                .provides
                                .first()
                                .is_some_and(|k| inputs.keyed.contains(k))
                            {
                                axes.toolchains[tc]
                                    .sorted_tools()
                                    .into_iter()
                                    .map(Some)
                                    .collect()
                            } else {
                                vec![None]
                            };
                            for inst in insts {
                                if f.admissible(inputs, fam.low, fam.floor, inst)?
                                    != Verdict::Admissible
                                {
                                    continue;
                                }
                                fam.n += 1;
                                for r in Rigor::LIVE {
                                    if r < fam.floor {
                                        // M27
                                        continue;
                                    }
                                    let mut any = false;
                                    for &m in &fam.members {
                                        if f.admissible(inputs, m, r, inst)? == Verdict::Admissible
                                        {
                                            any = true;
                                            break;
                                        }
                                    }
                                    if !any {
                                        let slot = fam
                                            .holes
                                            .entry((r.as_str(), kind, tname))
                                            .or_insert_with(|| (0, ctx_label(&fv, tname, axes)));
                                        slot.0 += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut families = Vec::new();
    for ((name, _), fam) in fams.iter().zip(state) {
        let members: Vec<&Act> = fam.members.iter().map(|&m| &inputs.acts[m]).collect();
        let primaries: BTreeSet<&str> = members
            .iter()
            .filter_map(|m| m.provides.first().map(String::as_str))
            .collect();
        if primaries.len() > 1 {
            let provs: BTreeSet<Vec<String>> = members
                .iter()
                .map(|m| {
                    let mut p = m.provides.clone();
                    p.sort();
                    p
                })
                .collect();
            let tuples: Vec<String> = provs.iter().map(|p| py_tuple(p)).collect();
            errors.push(format!(
                "family {name}: members provide different primary kinds [{}] (§9c.28)",
                tuples.join(", ")
            ));
        }
        let ranks: Vec<i64> = members.iter().map(|m| m.rank).collect();
        if ranks.iter().collect::<BTreeSet<_>>().len() != ranks.len() {
            let r: Vec<String> = ranks.iter().map(ToString::to_string).collect();
            errors.push(format!(
                "family {name}: duplicate ranks [{}] (§9c.28)",
                r.join(", ")
            ));
        }
        let low_id = inputs.acts[fam.low].id.clone();
        let mut holes = Vec::new();
        for ((r, kind, tname), (c, vector)) in fam.holes {
            errors.push(format!(
                "family {name}: no member admissible at rigor={r} for kind={kind} tc={tname} in {c} \
                 contexts where {low_id} is admissible at its floor (§9c.26 totality)"
            ));
            holes.push(Hole {
                rigor: Rigor::from_token(r).unwrap_or(Rigor::Forbidden),
                kind,
                toolchain: tname,
                contexts: c,
                vector,
            });
        }
        families.push(FamilyTotality {
            name: name.clone(),
            members: members.len(),
            lowest: low_id,
            floor: fam.floor,
            contexts_checked: fam.n,
            holes,
        });
    }
    Ok(Validation { families, errors })
}

/// The D14 variant pack that applies at `tc=node` for the same story context.
fn twin(pack: &str) -> &str {
    if pack == "story-ue" { "story" } else { pack }
}
