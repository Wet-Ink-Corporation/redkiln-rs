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
//! | the 38 fixtures + `clean/` | cycle 4 | **none** — see below |
//! | each `Rule::check` | cycle 5 | the 26 family tests, then `ac415_every_rule_id_has_a_rejecting_fixture` |
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
//! **Cycle 4 claims none of them either, and `ac415_` is cycle 5's rather than cycle 4's.** §7d.14's
//! red for that test names the failure landing it beside the fixtures would be: *"a weaker red
//! **joins names without running the rule**."* Its middle clause is *"each fixture run through its
//! own rule only yields ≥ 1 finding"*, and every `check` is [`rules::unbuilt`] until cycle 5 — so a
//! version landed at cycle 4 could assert the set equality and nothing else, which is that weaker
//! red exactly, under a name claiming a *rejecting* fixture. The set equality was measured in
//! cycle 4's body instead and is asserted for the first time at cycle 5, when it can run the rules.
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
use std::path::{Path, PathBuf};

use rk_adapter_sqlite::{SqliteStore, StoreOpenError};
use rk_kernel::catalog::CatalogLoad;
use rk_kernel::registry::{PredicateSites, Registry};

use crate::config::ConfigFile;
use crate::env::Env;

/// How much a fired rule weighs. **Three words and no fourth.**
///
/// Counted off §7d.8's table's severity column [re-counted 2026-09-21, Slice 4 cycle 3]: **29
/// rows** — 23 spelling `error`, one compound cell whose three ids design §8.4 fixes as `error`
/// (the cell says the design outranks the runbook, ADR-0092 clause 2.5), **four** spelling
/// `warning` and one `ok`. Per **id** that is **32** `Error`, **5** `Warning` and **1** `Ok`
/// (`RK-D-REBUILT-TIER2`), which sums to the table's 38.
///
/// **Corrected 2026-09-21.** This paragraph read *"23 `error`, **two rows spelling four `warning`
/// ids**, one `ok`, and one compound cell"*. Both halves of that clause were wrong: the warning
/// rows are **four**, not two — the `error`/`warning` counts above were taken from the table's
/// plain spellings and missed the two cells that bold the word (`RK-D-UNREAD`,
/// `RK-D-QUARANTINE-LIVE`) — and they carry **five** ids, not four. Nothing in the code depended
/// on it: the table is generated by walking the same column, and the walk was what found this.
/// Halt 7's rule is the reason it was found at all — *count it off the table, and re-count after
/// any edit* — and a prose count that nothing re-runs is exactly what it is aimed at.
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
    /// The enabled packs, by name, each mapped to its own file.
    ///
    /// 🛑 **A pack is a FLAT FILE and not a directory** — `.redkiln/packs/<name>.toml` [measured
    /// 2026-09-21, `crates/rk-cli/src/verbs/run.rs`:553-555]. `s7d` §7d.13 describes cards and
    /// skills as living under *"the pack's `cards/` and `skills/` directories"*; there is no
    /// per-pack directory in the tree. Cards are at `.redkiln/cards/<role>.json`, keyed by file
    /// name, and skills at `.redkiln/skills/`. A dated §7d.13 amendment of 2026-09-21 records it,
    /// and the field is named for what it holds rather than for what that sentence implies.
    pub pack_files: BTreeMap<String, PathBuf>,
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
    /// **The invocation's one wall-clock reading**, in unix seconds, supplied by the root.
    ///
    /// 🛑 **No rule reads a clock.** §7d.8 fixes `RK-D-QUARANTINE-LIVE`'s age as *"`now − until`'s
    /// sign and magnitude off a **supplied** `now`, no clock read"*, and §7d.5 gives the
    /// composition root exactly one read per invocation. A rule that called the clock itself would
    /// also make every `doctor` run unreproducible over a fixture, which is the second reason.
    pub now: i64,
    /// The recorded journal the six `s7b-ii` gate rows and the headroom row fold.
    ///
    /// 🛑 **Read from the STORE in the shipped binary and from `events.jsonl` in a fixture**, and
    /// the rules cannot tell which — which is the point. `SqliteStore::query` ignores its selector
    /// and returns the whole `t2_event` table (`crates/rk-adapter-sqlite/src/lib.rs`), so *"the
    /// whole journal"* is the only reading either source supports, and a rule written against a
    /// filtered one would be asserting over a selector that does not exist.
    pub journal: Vec<Row>,
    /// *(2026-09-26, s8c §8c.6; record V = ADR-0170 decision 12, proposed.)* The value table a
    /// `${toolchain.<key>}` resolves through for `RK-D-TIMEOUT`'s third arm, **selected by `run`'s
    /// own loop** (`verbs::run::choose_toolchain`, then `verbs::run::toolchain_values`), so `doctor`
    /// and `run` cannot disagree about a key. Selected only when the tree declares a substitution;
    /// see [`TimeoutTable`].
    pub timeout_table: TimeoutTable,
}

/// The value table `RK-D-TIMEOUT` resolves `${toolchain.<key>}` through (s8c §8c.6).
///
/// Built by [`Ctx::build`] only when an enabled pack's activity table or a catalog step declares a
/// `${toolchain.*}` substitution: selecting a toolchain constructs and probes the adapters, and a
/// tree with nothing to resolve has no reason to. **Not a refusal either way** — both non-table
/// arms are values the row reports as ONE `Error` finding, and no key is guessed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeoutTable {
    /// Nothing on the tree needs a table.
    NotNeeded,
    /// The chosen adapter's value table, flat and name-free.
    Values(rk_kernel::workflow::memo::ToolchainValues),
    /// No table could be selected: no adapter detected, no `registry.toml`, a registry that did not
    /// load, or no table for the detected adapter's toolchain.
    NoTable(String),
}

/// One journal row as `doctor` reads it.
///
/// # 🛑 `kind` IS A STRING, AND THAT IS NOT LAZINESS
///
/// Two measured reasons. **`rk_types::event::Event` does not implement `Deserialize`** [measured
/// 2026-09-21, `crates/rk-types/src/event.rs`:32] and `rk-types` is a closed crate `s7d` §7d.1 does
/// not permit this stage to edit, so a fixture journal cannot be parsed into it at all. And the
/// kind vocabulary is **closed at 38** (`EventKind::ALL`): parsing straight into it would make a
/// row whose kind is outside the set a *parse failure* rather than a finding — a defect `doctor`
/// exists to report would instead stop the table from running.
///
/// [`EventKind::from_wire`] is the one way back to the typed kind, and a rule that needs it calls
/// it and treats `None` as what it is: a kind no record mints.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Row {
    /// The wire name, exactly as design §5.4's table spells it — including the two DOTTED names,
    /// `artifact.read` and `artifact.write`, which are not snake_case and are not normalised.
    pub kind: String,
    /// The row's payload, unread until a rule reads it.
    #[serde(default)]
    pub payload: serde_json::Value,
}

impl Row {
    /// The typed kind, or `None` for a wire name outside the closed 38.
    #[must_use]
    pub fn kind(&self) -> Option<rk_types::event_kind::EventKind> {
        rk_types::event_kind::EventKind::from_wire(&self.kind)
    }

    /// One string member of the payload, or `None`.
    #[must_use]
    pub fn str_field(&self, key: &str) -> Option<&str> {
        self.payload.get(key).and_then(serde_json::Value::as_str)
    }
}

impl Ctx {
    /// `<worktree>/.redkiln` — where every configuration file a rule reads lives.
    #[must_use]
    pub fn redkiln(&self) -> PathBuf {
        self.worktree.join(".redkiln")
    }

    /// Build the whole `Ctx` off a repository, for the composition root and for a test alike.
    ///
    /// # 🛑 ONE BUILDER, AND THE REASON IS THE FAILURE A SECOND ONE CAUSES
    ///
    /// §7d.8 fixes the `Ctx` as one *"built once **by the composition root**"*. A test that
    /// assembled its own — reading the catalog differently, or defaulting a field the verb
    /// populates — would assert every rule against a configuration the shipped binary never sees,
    /// and each such difference is a rule that passes here and fires in production, or the
    /// reverse. So `verbs::doctor` and `tests/it/doctor.rs` call **this**, and the four values
    /// neither can derive from the tree are parameters.
    ///
    /// **`store` and `probe` are parameters because they are not configuration.** The store is
    /// opened by [`crate::adapters::open_store`], outside the module Checkpoint 7d's D-10 ban
    /// filters; the probe spawns a child, which a fixture must be able to stub.
    ///
    /// **Nothing here refuses.** Every absence — no catalog directory, no `facets.toml`, no
    /// journal — is a value a rule reports, never an error this returns. A builder that refused
    /// would decide, before the table ran, which defects `doctor` is allowed to find.
    #[must_use]
    pub fn build(
        worktree: PathBuf,
        common_dir: PathBuf,
        config: ConfigFile,
        env: Env,
        store: Option<Result<SqliteStore, StoreOpenError>>,
        probe: Box<dyn probe::VersionProbe>,
        now: i64,
    ) -> Self {
        let redkiln = worktree.join(".redkiln");

        let mut pack_files = BTreeMap::new();
        for name in &config.packs.enabled {
            pack_files.insert(
                name.clone(),
                redkiln.join("packs").join(format!("{name}.toml")),
            );
        }

        let catalog = rk_kernel::catalog::load_diagnostic_dir(&redkiln.join("catalog"));
        let registry = std::fs::read_to_string(redkiln.join("facets.toml"))
            .ok()
            .and_then(|src| Registry::from_toml(&src).ok());
        let journal = read_journal(&worktree.join("events.jsonl"));
        let timeout_table =
            timeout_table(&worktree, &common_dir, &config, &env, &catalog, &pack_files);

        Self {
            config,
            catalog,
            pack_files,
            registry,
            // 🚧 The predicate sites are gathered by the facet family's own reader, which parses
            // each predicate with `rk_kernel::predicate` rather than scanning it — the kernel's
            // `collect_paths` is private, and a substring scan would match a facet's name inside a
            // quoted literal (`registry.rs`'s own reason). Empty until that family lands.
            sites: PredicateSites::default(),
            env,
            common_dir,
            worktree,
            store,
            probe,
            now,
            journal,
            timeout_table,
        }
    }
}

/// Does the tree declare any `${toolchain.*}` substitution a `RK-D-TIMEOUT` arm resolves?
fn declares_substitution(catalog: &CatalogLoad, pack_files: &BTreeMap<String, PathBuf>) -> bool {
    let on_a_step = catalog.catalog.activities().iter().any(|a| {
        a.steps.iter().any(|s| {
            matches!(
                s.timeout_secs,
                Some(rk_kernel::catalog::StepTimeout::Toolchain(_))
            )
        })
    });
    on_a_step
        || pack_files.values().any(|path| {
            std::fs::read_to_string(path).is_ok_and(|text| text.contains("${toolchain."))
        })
}

/// **`run`'s selection, run for `doctor`** (s8c §8c.6; record V decision 12): the same candidate
/// loop over the same `[adapters]`, then the same table choice. The loop's per-candidate reports are
/// collected into the no-table reason, never printed.
fn timeout_table(
    worktree: &Path,
    common_dir: &Path,
    config: &ConfigFile,
    env: &Env,
    catalog: &CatalogLoad,
    pack_files: &BTreeMap<String, PathBuf>,
) -> TimeoutTable {
    if !declares_substitution(catalog, pack_files) {
        return TimeoutTable::NotNeeded;
    }
    let registry_path = crate::verbs::run::registry_path(worktree);
    if !registry_path.is_file() {
        return TimeoutTable::NoTable(format!(
            "{} does not exist, so no `${{toolchain.*}}` value table is declared",
            registry_path.display()
        ));
    }
    let registry = match rk_kernel::workflow::memo::VerbRegistry::load_file(&registry_path) {
        Ok(registry) => registry,
        Err(e) => {
            return TimeoutTable::NoTable(format!(
                "{} does not load: {e}",
                registry_path.display()
            ));
        }
    };
    let candidates = crate::verbs::run::ToolchainCandidates {
        toolchains: &config.adapters.toolchains,
        timeout_secs: config
            .adapters
            .toolchain_timeout_secs
            .unwrap_or(crate::verbs::run::DEFAULT_TOOLCHAIN_TIMEOUT_SECS),
        programs: config.adapters.toolchain_programs.as_ref(),
        common_dir,
    };
    let mut reports: Vec<String> = Vec::new();
    let chosen = crate::verbs::run::choose_toolchain(&candidates, env, worktree, &mut |line| {
        reports.push(line.to_owned());
    });
    match chosen {
        Ok(chosen) if chosen.adapter.is_some() => TimeoutTable::Values(
            crate::verbs::run::toolchain_values(&registry, chosen.adapter.as_deref()),
        ),
        Ok(chosen) => TimeoutTable::NoTable(format!(
            "no toolchain adapter detects a project at {} ({}), so no value table is selected",
            worktree.display(),
            match chosen.toolchain.detect() {
                Err(e) => format!("{e:?}"),
                Ok(_) if reports.is_empty() => "every candidate answered not present".to_owned(),
                Ok(_) => reports.join("; "),
            }
        )),
        Err(_) => TimeoutTable::NoTable(format!(
            "toolchain selection could not run: {}",
            reports.join("; ")
        )),
    }
}

/// Read a fixture journal, one JSON object per line.
///
/// **A line that does not parse is SKIPPED and not an error**, on the builder's own rule: a
/// malformed journal is something a rule may report, and refusing here would stop the other
/// thirty-seven rules from running at all. Blank lines are not lines.
fn read_journal(path: &Path) -> Vec<Row> {
    let Ok(text) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str::<Row>(l).ok())
        .collect()
}
