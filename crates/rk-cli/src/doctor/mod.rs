//! **`doctor` — a rule table that is data, run over one `Ctx` built once by the composition root.**
//! `s7d` §7d.8.
//!
//! # 🚧 WHAT IS BUILT HERE IS THE SKELETON, AND THE SKELETON CLAIMS NO TEST
//!
//! This is Slice 4 **cycle 2**, and its whole purpose is that every red after it is *behavioural*.
//! Golden constraint 12 — *a red must be for the RIGHT reason* — and the Slice 3 record behind it:
//! every cycle that wrote its test before the signatures it asserts on had to be redone, including
//! one whose red was a compile error. So the types, the fields and the function signatures land
//! here with **no test claimed**, and each cycle after this one is red on what its code does.
//!
//! **Where each unfilled body is filled, and what goes red when it is:**
//!
//! | here | filled by | the test whose red is then behavioural |
//! | --- | --- | --- |
//! | [`rules::RULES`], empty | cycle 3 | `doctor_rule_table_covers_every_register_named_row` |
//! | the 38 fixtures | cycle 4 | `ac415_every_rule_id_has_a_rejecting_fixture` |
//! | each `Rule::check` | cycle 5 | the 26 family tests |
//! | [`probe`] | cycle 7 | `ac423_probe_is_one_version_spawn_not_a_dispatch` |
//! | [`rebuild`] | cycle 9 | `ac427_…`, `ac429_…`, `ac397_…` |
//!
//! **No unfilled body is `todo!()` or `unimplemented!()`**, and that is this crate's own rule
//! rather than a preference: `verbs/mod.rs`'s header says a stub reaching for `unimplemented!()`
//! *"would exit `101` through a path the exit table does not describe"*, and ADR-0118 clause 6
//! makes every verb return a value so an in-process caller observes one. Each unfilled body
//! therefore returns the **empty** answer — no events, no findings, no cards — which is also what
//! makes the later cycle's red the one its criterion names: a rebuild that produces nothing fails
//! `ac427_…`'s *"the rebuild happened"* control, not a panic assertion.
//!
//! # 🛑 An empty `RULES` makes AC-414, AC-415 and SM-121 VACUOUS, and that is recorded
//!
//! §7d.14 says it in terms: *"over an empty rule table AC-414's per-fixture join, AC-415's per-id
//! join and SM-121's dispatch count"* are all vacuous. So this cycle lands [`rules::RULES`] and
//! **claims none of those three**. The anti-vacuity instrument is cycle 3's
//! `doctor_rule_table_covers_every_register_named_row` — the twenty-nine register-named ids as a
//! floor (RA-M-01) — and it is what closes the hole rather than any comment here.
//!
//! # What this module may not contain, and why the ban is module-path filtered
//!
//! Checkpoint 7d bans `ClaudeHarness::new`, `rk_adapter_claude::config::Config::new`, a bare
//! `Config::new(`, the two `*Toolchain::new`, `SqliteStore::open` and `dyn Harness` under a filter
//! that admits `src/doctor/*.rs` **and** `src/verbs/doctor.rs` (D-10, D18-7; AC-414 as amended).
//! **The store handle is opened by the root and handed in** — [`crate::adapters::open_store`],
//! which the filter does not admit — and [`Ctx::store`] names the type, which constructs nothing.
//! `rk_adapter_sqlite::{SqliteStore, StoreOpenError}` is therefore importable here and is imported
//! below; `SqliteStore::open` is not, and is nowhere in this module tree.

pub mod probe;
pub mod rebuild;
pub mod roles;
pub mod rules;

use std::collections::BTreeMap;
use std::path::PathBuf;

use rk_adapter_sqlite::{SqliteStore, StoreOpenError};
use rk_kernel::catalog::CatalogLoad;
use rk_kernel::registry::{PredicateSites, Registry};

use crate::config::ConfigFile;
use crate::env::Env;

/// How much a fired rule weighs. **Three words and no fourth.**
///
/// Counted off §7d.8's table's severity column, 29 rows: 23 `error`, two rows spelling four
/// `warning` ids, one `ok` (`RK-D-REBUILT-TIER2`), and one compound cell whose three ids design
/// §8.4 fixes as `error` [measured 2026-09-20].
///
/// **`Ok` is a severity and not the absence of a finding.** `RK-D-REBUILT-TIER2`'s whole content is
/// that tier-2 emptiness after a rebuild is *expected* — `ac397_rebuilt_store_tier2_emptiness_is_ok_expected`
/// — so the rule has something to say and it is not a defect. A rule that reported it by staying
/// silent would be indistinguishable from one that did not run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Reported, and not a defect. One row: `RK-D-REBUILT-TIER2`.
    Ok,
    /// Reported; the exit code does not move (§7d.8: *"warnings never move the code"*).
    Warning,
    /// Exit `3` — AC-438's *"including any failing error-severity `doctor` rule"*.
    Error,
}

impl Severity {
    /// The status word this severity prints in a `<id> <status> <detail>` line.
    ///
    /// **`ok` is the word a rule that did NOT fire prints too**, which is why it is spelled here
    /// once rather than at the two sites: §7d.8's stdout is *"one line per **evaluated** rule —
    /// fired or not — with `ok` a status word"*, and the block's FLOOR loop greps the
    /// register-named ids off a **clean** repository, which only holds because a rule that found
    /// nothing still prints its row.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// One thing a rule found.
///
/// The `id` is the rule's, copied onto the finding rather than inferred from position: `Report`
/// holds findings from every rule in one vector, and a consumer that recovered the id by counting
/// would be wrong the first time a rule yields two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The `RK-D-*` id of the rule that produced it.
    pub id: &'static str,
    /// The rule's severity, carried so a reader of one finding needs no second lookup.
    pub severity: Severity,
    /// What was found, in one line — the `<detail>` of the stdout row.
    pub detail: String,
}

/// What one `doctor` run produced: every rule that was **evaluated**, and everything they found.
///
/// **`evaluated` is a separate list from `findings` and is not derivable from it.** §7d.8's stdout
/// is one line per evaluated rule *fired or not*, so a report that carried findings alone could not
/// print the run and the block's floor loop would find nothing on a clean repository. This is the
/// same distinction one level up that `Severity::Ok` is one level down.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// Every rule id that ran, in table order.
    pub evaluated: Vec<&'static str>,
    /// Everything they found, in the order they found it.
    pub findings: Vec<Finding>,
}

impl Report {
    /// Did any **error**-severity rule fire? The one question the exit code asks.
    ///
    /// §7d.8: exit `0` when no error-severity rule fired, `3` when one did; *"warnings never move
    /// the code"*, and neither does an `ok`.
    #[must_use]
    pub fn any_error(&self) -> bool {
        self.findings.iter().any(|f| f.severity == Severity::Error)
    }
}

/// Everything every rule reads, built **once** by the composition root and handed to the table.
///
/// # 🛑 Every field is carried in, and none is acquired here
///
/// That is the shape §7d.8 fixes — *"a `Ctx` built once **by the composition root**"* — and it is
/// what makes the rules testable in process: `ac414_…` builds a `Ctx` itself and runs the table
/// over it, with no repository and no store of its own. A `Ctx` that opened anything would make
/// every rule a test that needs a filesystem.
///
/// # 🛑 `store` is `Option<Result<…>>`, and both layers mean something different
///
/// §7d.8, verbatim: *"`Some(Ok(store))` on a clean open, `Some(Err(Refused))` for a foreign store,
/// **carried in and not refused before the table**, so `RK-D-STORE-ID` renders it as its row;
/// `None` on the `--rebuild` path"*. So the `Err` is **not** an error the verb returns — it is the
/// input to a rule. A `doctor` that refused a foreign store before the table would exit `3` with no
/// row, and AC-413's id would be unobservable on the real binary, which is the failure the
/// two-layer type exists to prevent.
pub struct Ctx {
    /// The parsed `.redkiln/config.toml`.
    pub config: ConfigFile,
    /// The enabled packs' catalog **through the loader's diagnostic mode** — the activities that
    /// loaded and every defect typed (`catalog::load_diagnostic`, RT-54, the Slice 4 kernel edit
    /// settled at cycle 1). Not a `Result`: a pack with two defects yields two findings, and a
    /// first-error `Result` is exactly what that edit replaced.
    pub catalog: CatalogLoad,
    /// The pack roots the catalog above was read from, by pack name — the rules that read
    /// `cards/` and `skills/` need the directory and not only the activities.
    pub pack_roots: BTreeMap<String, PathBuf>,
    /// `facets.toml`, parsed — `Registry::from_toml`. `None` when the repository has none: an
    /// absence a rule reports, never a refusal the verb makes.
    ///
    /// 🛑 **§7d.8 names *"`facets.toml` and the fact registry"* as if they were two things. They
    /// are one type.** Measured 2026-09-20: `rk_kernel::facet_registry` carries no `Registry` of
    /// its own — it imports `crate::registry::Registry`
    /// (`crates/rk-kernel/src/facet_registry.rs`:118) — so the file and the fact registry are the
    /// source and the parsed value of a single thing, and a `Ctx` with two fields for them would
    /// have held the same value twice. The stage file's phrase reads as two and is not.
    pub registry: Option<Registry>,
    /// The predicate sites `facet_registry_rule`'s second parameter takes — the catalog's
    /// `applicable_if` expressions and the packs' `require` rows.
    ///
    /// **A separate field and not derived inside a rule**, because the criterion's GIVEN names all
    /// three inputs: the registry's own entries, the `require` rows and the `applicable_if` sites.
    /// `facet_registry_rule`'s own doc records what a rule reading only `require` rows lets
    /// through — *"a catalog whose `applicable_if` names an unregistered facet, which is
    /// `binary_change`'s measured case"*.
    pub sites: PredicateSites,
    /// The environment as the root captured it (`Env::capture`, the one read, ADR-0083).
    pub env: Env,
    /// The git common directory — where `redkiln/` and the store live.
    pub common_dir: PathBuf,
    /// The absolute working-tree root.
    pub worktree: PathBuf,
    /// **The handle the root opened.** See the type note above: both layers are rule input.
    pub store: Option<Result<SqliteStore, StoreOpenError>>,
    /// The installed-CLI version probe (D-11), as a seam — `RK-D-CLI-VERSION` and `RK-D-TTL` read
    /// it, and `ac423_ac526_…` drives it stubbed while `ac423_probe_is_one_version_spawn_not_a_dispatch`
    /// drives the real one.
    pub probe: Box<dyn probe::VersionProbe>,
}
