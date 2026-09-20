//! **`run` — a read, a decision and an exit.** §7d.5 numbers the ten steps and this file keeps the
//! numbering, so a test that quotes a step can be read against the code that performs it.
//!
//! Cycle 4 of Slice 3 builds **steps (1) through (7)** — every path that refuses *before* a worktree
//! exists. Steps (8)–(10) arrive in the next cycle; the seam between them is [`through_step_seven`],
//! which returns the resolved inputs rather than dispatching.
//!
//! # 🛑 THE PRECEDENCE INSIDE STEP (7) IS FIXED AND IS TESTED
//!
//! §7d.5: *"an empty `[packs] enabled` is exit `3` with \"no process pack enabled\" (**D18-8**)
//! **before any lane derivation** — so the refusal is the same whatever item was named"*. An empty
//! enabled set therefore wins over a kindless item, and `run_on_init_only_repo_is_a_config_outcome`
//! is the case that pins it: the scaffold `init` writes carries `enabled = []`.
//!
//! # 🛑 STEP (7)'s REFUSAL PRECEDES STEP (8), AND THE EVIDENCE IS AN ABSENCE OF TWO THINGS
//!
//! No `rk-run/*` ref and no `<common-dir>/redkiln/worktrees/` directory. Both are step (8)'s acts, so
//! their absence after a step-(7) refusal is what says the order held. The harness marker count is
//! **not** that evidence and is reported as a floor: an empty pack set yields no DAG, so no
//! implementation whatever could have written one.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::Deserialize;

use rk_git::git_exec::GitExec;
use rk_git::path::WorktreePath;
use rk_git::reconciled::Entry;
use rk_kernel::select::Item;
use rk_ports::cancel::CancelToken;
use rk_ports::event_store::EventStore;

use crate::cli::{Cli, Lane, RunArgs};
use crate::exit::{Exit, Halt};
use crate::verbs::{EntryReconciler, Startup};
use crate::{env::Env, repo};

/// The adapter-facing pack values, read by **`rk-cli`** over the same
/// `.redkiln/packs/<pack>.toml` bytes the kernel's `Pack::from_toml` reads.
///
/// §7d.5: *"the kernel's `Pack` is **not** widened (no 7b re-open for this), and a record fixing
/// these keys in the pack schema is 7e's (RT-57; §7d.18 RAISED)"*. `deny_unknown_fields` is **off**
/// because the kernel owns the rest of the file — every kernel key would otherwise be an unknown one
/// here.
#[derive(Debug, Clone, Deserialize)]
pub struct PackExtras {
    /// §7d.5 step (9): the gate deadline is `now + this`, off the ONE wall-clock read.
    pub gate_deadline_secs: i64,
    /// `rk_adapter_claude::config::Margins` — both members required, in ticks (AC-377).
    pub margins: Margins,
    /// The two prompt-cache TTLs.
    pub cache: Cache,
    /// The gate-class expiry rows.
    ///
    /// 🛑 **THE CLASS IS THE FULL GATE KIND, AXIS AND ALL — AND IT IS A `class` VALUE RATHER THAN
    /// A KEY.** `fsm::next_action` spells the class `format!("{}:{}", gate.reason.as_str(),
    /// gate.axis)`, so an abstention gate on `has_repro` has kind `unknown_facet:has_repro`. A row
    /// keyed on the bare `unknown_facet` matches no gate this system opens, and the failure
    /// surfaces only at run time as `ExpiryPolicyLint::NoActionDeclared { class }`.
    ///
    /// But the full kind carries a COLON, and a TOML key carrying one must be quoted — which the
    /// kernel's `toml_subset` walker refuses outright (*"bare unquoted keys only, and this one is
    /// `\"unknown_facet:has_repro\"`"*, `crates/rk-kernel/src/toml_subset.rs`). **Two parsers read
    /// the same pack bytes**, so a key shape only one of them accepts is available to neither.
    /// Hence an array of tables, where every key stays bare and the class is carried as a value
    /// that is exactly the string `GateRecord::kind()` produces.
    #[serde(default)]
    pub gate_expiry: Vec<GateExpiryRow>,
    /// `timeout_secs` per activity — `BindRequest.timeout_secs`.
    #[serde(default)]
    pub activity: BTreeMap<String, ActivityExtras>,
    /// The role table step (8) resolves a card's tier from.
    #[serde(default)]
    pub role: BTreeMap<String, RoleExtras>,
}

/// `[margins]`.
#[derive(Debug, Clone, Deserialize)]
pub struct Margins {
    /// The five-hour window's margin, in ticks.
    pub five_hour: u16,
    /// The seven-day window's margin, in ticks.
    pub seven_day: u16,
}

/// `[cache]`.
#[derive(Debug, Clone, Deserialize)]
pub struct Cache {
    /// `CLAUDE_CODE_PROMPT_CACHE_TTL`.
    pub prompt_ttl: String,
    /// `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL`.
    pub subagent_prompt_ttl: String,
}

/// `[activity.<id>]`.
#[derive(Debug, Clone, Deserialize)]
pub struct ActivityExtras {
    /// The per-activity wall-clock bound the binder is handed.
    pub timeout_secs: u64,
}

/// `[role.<name>]`.
#[derive(Debug, Clone, Deserialize)]
pub struct RoleExtras {
    /// The model family the role resolves to (the kernel's own key).
    pub family: String,
    /// The tier the binder injects.
    ///
    /// 🛑 **The role table is the tier's ONE source.** `Binder::bind` refuses a role declaring no
    /// model — *"role implementer declares no model — the tier is traceable only through the role
    /// (AC-212; AC-118)"* — and the CARD may not carry one (`ConfigError::TierLeak`). So it is
    /// read here, beside the kernel's `family`, and nowhere else.
    #[serde(default)]
    pub model: Option<rk_adapter_claude::card::Tier>,
    /// The reasoning effort the binder injects, from the same single source as [`Self::model`].
    #[serde(default)]
    pub effort: Option<rk_adapter_claude::card::Effort>,
}

/// One `[[gate_expiry]]` row: the gate class, and the action `gate::expire` takes for it.
///
/// See [`PackExtras::gate_expiry`] for why the class is a value rather than a key.
#[derive(Debug, Clone, Deserialize)]
pub struct GateExpiryRow {
    /// The full gate kind, exactly as `GateRecord::kind()` renders it.
    pub class: String,
    /// One of `fail` | `escalate` | `auto-decide`, linted by `gate::lint_expiry_policy_row`.
    pub action: String,
}

/// One item record — `rk-cli`'s own reader, never the kernel's.
///
/// §7d.5 fixes the key set and §7d.5 also fixes what a test may pin: *"7d's fixtures use a top-level
/// `kind` key … and the test pins the refusal and the map, **not the key's spelling** — stated so a
/// 7e rename re-points a fixture, not a decision."*
#[derive(Debug, Clone, Deserialize)]
pub struct ItemRecord {
    /// The item id.
    pub id: String,
    /// The item's kind — ADR-0128 decision 2 derives the lane from it. **Optional**, and a record
    /// without one is refused rather than defaulted.
    pub kind: Option<String>,
    /// The stage every kernel entry takes. **Optional here and refused below**, so the refusal names
    /// the item (RT-50) instead of `serde` naming a field.
    pub stage: Option<String>,
    /// One value per facet the registry declares. An ABSENT facet is `with_facet_null`, which is how
    /// a `require` row abstains and a gate opens.
    #[serde(default)]
    pub facets: BTreeMap<String, toml::Value>,
}

/// Everything steps (1)–(7) resolve, handed to step (8).
pub struct Resolved {
    /// The repository.
    pub exec: GitExec,
    /// The absolute working-tree root.
    pub worktree: WorktreePath,
    /// The git common directory.
    pub common_dir: PathBuf,
    /// The lane ADR-0128 derived.
    pub lane: Lane,
    /// The pack the lane names.
    pub pack_name: String,
    /// The pack's own bytes — both parsers read these.
    pub pack_src: String,
    /// The adapter-facing values.
    pub extras: PackExtras,
    /// The item, rebuilt from its record.
    pub item: Item,
    /// The stage the item record carries (RT-50).
    pub stage: String,
    /// The kernel's `Config`, **constructed once** at step (4) and threaded from here into
    /// `RunInputs::with_config` — ADR-0083's *"the composition root builds it; the kernel reads
    /// it and never makes one"*. See the 🛑 note at its one construction site.
    pub config: rk_kernel::config::Config,
}

/// The outer verb.
///
/// # 🛑 STEP (8)'S ADAPTER CONSTRUCTION IS **CYCLE 6'S**, AND THE STUB BELOW IS THE DECISION
///
/// The contract's critic sequences cycle 5 as *"run's DISPATCH PATH (steps 8–10 minus the gate) …
/// **Fake harness only**"* and puts *"real `ClaudeHarness` + the fixture child"* in cycle 6. The
/// contract's route (B), which the critic endorses, has the outer `run(..)` perform (5) and (8) and
/// an inner [`run_with`] that the in-process tests call — and says of the real adapter that **"the
/// test builds the real adapter itself"**, leaving (5) and (8) *"covered only by spawned tests"*:
/// `ac413_…` (cycle 3), `ac403_…` (cycle 6), `ac486_…`'s spawned arm (cycle 4).
///
/// So this cycle builds the whole inner path and **does not** build the real `Harness`. The
/// alternative — wiring the outer path now and handing `Ports` a `FakeHarness` — would put a stub
/// in the shipped binary, which golden constraint 4 exists to catch, and it is strictly worse than
/// an honest refusal. Wiring it with the REAL harness is what the parked `d109463` did, and it
/// spawned a child that never returned: `rk-fixture-child-claude` takes `hold` when
/// `RK_FIXTURE_MODE` is unset, and cycle 6 owns setting it.
///
/// Every value this arm would need is already resolved; nothing about the stub is load-bearing.
///
/// # Errors
/// [`Halt::PoisonedLock`] from the store acquisition. Every other refusal is a value.
pub fn run(cli: &Cli, args: &RunArgs, env: &Env, startup: Startup<'_>) -> Result<Exit, Halt> {
    let resolved = match through_step_seven(cli, args, env, startup) {
        Ok(resolved) => resolved,
        Err(exit) => return Ok(exit),
    };

    // ---- (5) the store. AC-413's identity check is inside `open`. ------------------------------
    let opened = match rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree) {
        Ok(store) => store,
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
    // Held so the type of the seam cycle 6 fills is visible here rather than only in a diff: the
    // `Ports` this arm will build takes exactly this `Arc`.
    drop(store);

    // Step (8)'s adapter construction is cycle 6's — see this function's own note. The inner path
    // BELOW it is built and driven in-process by this cycle's five tests; only the construction of
    // the real `Harness` and the call through to `run_with` wait.
    eprintln!("redkiln: `run` reaches step (8), which is not built at this cycle");
    Ok(Exit::ConfigError)
}

/// The inner verb — the in-process twin every Slice-3 loop test drives.
///
/// **The ports ARRIVE pre-built**, which is the whole reason the seam is shaped this way: §7d.10's
/// `adr0118_poisoned_store_lock_halts_run_exit_70` hands *"the verb function … the same poisoned
/// `Arc`"*, and a verb that opened its own store could not be given one. The contract's route (B)
/// is the same shape: *"a `run_with(ports: Ports, supply_spec, …)` that replaces (5) and (8), with
/// the outer `run(..)` doing them and delegating."*
///
/// **`supply_spec` IS AN ARGUMENT, NOT A PRIVATE CLOSURE.** §7d.5: a node bound at (8) is served
/// from `inputs`, and a node first seen inside the loop is bound through the same `Binder` the
/// closure holds — *"the fake-harness tests hand a supplier over `RecordedSpec::supplied(..)`
/// values"*. A `run_with` that built its own refusing supplier could not be handed one, and every
/// fake-harness run would be red on `inputs/specs.json` before any dispatch (RT-55's wrong reason).
///
/// **The token is a FUNCTION ARGUMENT**, never an environment toggle. §7d.10 in terms: *"no env
/// toggle — `RK_TEST_PREFIRE_CANCEL` is **not** used; AC-474 forbids it; the token is a function
/// argument"*.
///
/// # Errors
/// [`Halt::PoisonedLock`] at the first acquisition. ADR-0118 clause 3 forbids every spelling of a
/// recovery, so there is no `into_inner` on this path and no row is appended on the way out.
pub fn run_with(
    resolved: &Resolved,
    inputs: &rk_kernel::replay::RunInputs,
    ports: rk_ports::root::Ports,
    supply_spec: &mut dyn FnMut(
        &str,
    ) -> Result<
        rk_kernel::replay::RecordedSpec,
        rk_kernel::replay::ReplayError,
    >,
    token: &CancelToken,
) -> Result<Exit, Halt> {
    let store = Arc::clone(&ports.store);
    let store = &store;
    // §7d.5: "the root checks `token.is_cancelled()` at each step boundary and returns
    // `Ok(Exit::SignalOutsideDispatch)`, 130, … having written nothing partial (every store write is
    // one row; every git write is one commit)."
    //
    // This boundary is BEFORE the first acquisition, so a pre-fired token leaves the store untouched.
    if token.is_cancelled() {
        eprintln!(
            "redkiln: interrupted; nothing after the pack set for `{}` ran.",
            resolved.item.id()
        );
        return Ok(Exit::SignalOutsideDispatch);
    }

    // The first acquisition. A poisoned lock HALTS — never a guard, never a `StepError`.
    let guard = crate::store::acquire(store, "the run")?;
    drop(guard);

    if token.is_cancelled() {
        eprintln!(
            "redkiln: interrupted; nothing after the store acquisition for `{}` ran.",
            resolved.item.id()
        );
        return Ok(Exit::SignalOutsideDispatch);
    }

    resume_and_read(resolved, inputs, ports, supply_spec, token)
}

/// Steps (1)–(7), in §7d.5's order. `Err(Exit)` is a refusal; `Ok` hands step (8) its inputs.
///
/// # Errors
/// An [`Exit`] row, never a halt — the halt is step (8)'s store acquisition.
#[expect(
    clippy::too_many_lines,
    reason = "§7d.5 numbers ten steps and fixes their ORDER; splitting them across helpers would \
              put the order in the call graph rather than in one readable sequence, and the order \
              is what four of this slice's tests assert"
)]
pub fn through_step_seven(
    cli: &Cli,
    args: &RunArgs,
    env: &Env,
    startup: Startup<'_>,
) -> Result<Resolved, Exit> {
    // ---- (1) the git version floor, and (2)'s common dir + H-3 bootstrap. `repo::discover`
    // performs both; `run` does not re-implement H-3.
    let repo = repo::discover(cli.cwd.as_deref()).map_err(|e| {
        eprintln!("redkiln: {e}");
        Exit::ConfigError
    })?;
    let root = Path::new(repo.worktree.as_str()).to_path_buf();

    // ---- (2) `.redkiln/config.toml`, through the COUNTING SEAM (AC-485's first).
    let config_path = root.join(".redkiln").join("config.toml");
    let config = startup.config.read(&config_path).map_err(|e| {
        eprintln!("redkiln: {e}");
        Exit::ConfigError
    })?;

    // ---- (3) the ONE environment read happened in `main`; this verb is handed the value.
    // ---- (4) adapter resolution, ONCE, through the counting seam (AC-485's second). A refusal
    // here is exit 3 BEFORE any process is spawned and before any store write (AC-486).
    let adapters = startup.adapters.resolve(&config, env).map_err(|e| {
        eprintln!("redkiln: {e}");
        Exit::ConfigError
    })?;

    // 🛑 THE `Config` VALUE, CONSTRUCTED ONCE, HERE, AND THREADED (ADR-0083).
    //
    // §7d.5 numbers the construction at step (8), but the value is a function of step (4)'s
    // resolved names and step (2)'s `unread_window_runs` and of nothing later, and §7d.5 also has
    // the invocation's `reconcile_started` row carry its digest at step (6). Building it where its
    // last input arrives is what makes "constructed once at the root" a property of the code rather
    // than of a convention: there is exactly one `Config::new` call in this crate, and every reader
    // takes the value from [`Resolved`].
    //
    // `Config::new` SORTS `toolchains` itself, so the recorded form is a function of the set and not
    // of the order the file listed it in — the same rule `RunInputs::with_catalog` applies.
    let kernel_config = rk_kernel::config::Config::new(
        adapters.harness(),
        adapters.store(),
        adapters.toolchains(),
        config.doctor.unread_window_runs,
    );

    // ---- (7) THE PACK SET, BEFORE ANY LANE DERIVATION.
    //
    // D18-8, and the order is the whole point: "an empty `[packs] enabled` is exit 3 with 'no
    // process pack enabled' BEFORE any lane derivation — so the refusal is the same whatever item
    // was named". The scaffold `init` writes carries `enabled = []`, so this is the init-only
    // repository's refusal and it is reached with no item record read at all.
    if config.packs.enabled.is_empty() {
        eprintln!(
            "redkiln: no process pack enabled — `.redkiln/config.toml`'s `[packs] enabled` is \
             empty, so there is no pack to run `{}` under.",
            args.item
        );
        return Err(Exit::ConfigError);
    }

    // The item record. Read AFTER the pack-set refusal, so the precedence above is real.
    let record_path = root
        .join(".redkiln")
        .join("items")
        .join(format!("{}.toml", args.item));
    let record_src = std::fs::read_to_string(&record_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", record_path.display());
        Exit::ConfigError
    })?;
    let record: ItemRecord = toml::from_str(&record_src).map_err(|e| {
        eprintln!(
            "redkiln: {} is not an item record: {e}",
            record_path.display()
        );
        Exit::ConfigError
    })?;

    // RT-50. Refused rather than defaulted, and the refusal NAMES THE ITEM.
    let Some(stage) = record.stage.clone() else {
        eprintln!(
            "redkiln: no stage on the item record for `{}` — the stage is read once, from the \
             record, and is not defaulted.",
            args.item
        );
        return Err(Exit::ConfigError);
    };

    // ---- the lane. ADR-0128 decision 2.
    let lane = derive_lane(args, record.kind.as_deref())?;
    let pack_name = pack_for(lane);

    // Decision 3: "a lane whose pack is not enabled is the same configuration error with the pack
    // named".
    if !config.packs.enabled.iter().any(|p| p == pack_name) {
        eprintln!(
            "redkiln: the `{}` lane names the `{pack_name}` pack, which is not in \
             `.redkiln/config.toml`'s `[packs] enabled`.",
            lane_name(lane)
        );
        return Err(Exit::ConfigError);
    }

    let pack_path = root
        .join(".redkiln")
        .join("packs")
        .join(format!("{pack_name}.toml"));
    let pack_src = std::fs::read_to_string(&pack_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", pack_path.display());
        Exit::ConfigError
    })?;
    let extras: PackExtras = toml::from_str(&pack_src).map_err(|e| {
        eprintln!(
            "redkiln: {} does not carry the adapter-facing pack values: {e}",
            pack_path.display()
        );
        Exit::ConfigError
    })?;

    let item = build_item(&record);

    // ---- (6) reconcile at entry, and the invocation's ONE `reconcile_started` row.
    //
    // The row itself is step (8)'s to write in its four-member form; at a step-(7) refusal it is
    // written without the worktree pair. Both are the next cycle's, and this cycle's tests assert
    // the ONE-ROW count rather than the members.
    let _ = startup
        .reconciler
        .reconcile(&repo.exec, &repo.worktree, Entry::Run)
        .map_err(|e| {
            eprintln!("redkiln: reconcile at entry refused: {e:?}");
            Exit::ConfigError
        })?;

    Ok(Resolved {
        exec: repo.exec,
        worktree: repo.worktree,
        common_dir: repo.common_dir,
        lane,
        pack_name: pack_name.to_owned(),
        pack_src,
        extras,
        item,
        stage,
        config: kernel_config,
    })
}

/// ADR-0128 decision 2, as a function, with every row of §7d.5's outcome table.
///
/// **It never defaults.** The ADR forbids a default in terms: *"Forbids a default lane: an item whose
/// lane is neither given nor derivable is refused, never run under `fix`."*
fn derive_lane(args: &RunArgs, kind: Option<&str>) -> Result<Lane, Exit> {
    let derived = kind.and_then(lane_of_kind);
    match (args.lane, derived) {
        // Given and derivable, and they agree.
        (Some(given), Some(from_kind)) if given == from_kind => Ok(given),
        // Given and derivable and they DISAGREE — refused as the same configuration error, naming
        // both, "never resolved in favour of either".
        (Some(given), Some(_)) => {
            eprintln!(
                "redkiln: `--lane {}` disagrees with the item record's `kind = \"{}\"`, which \
                 derives the `{}` lane. A disagreement is refused, not resolved in favour of \
                 either (ADR-0128 decision 2).",
                lane_name(given),
                kind.unwrap_or(""),
                lane_name(derived.unwrap_or(given))
            );
            Err(Exit::ConfigError)
        }
        // Given, and the record makes nothing derivable: the flag stands.
        (Some(given), None) => Ok(given),
        // Neither given nor derivable.
        (None, None) => {
            eprintln!(
                "redkiln: no lane for `{}` — the item record makes none derivable and no `--lane` \
                 was given. The admissible values are `fix` and `story`; there is no default \
                 (ADR-0128 decision 2).",
                args.item
            );
            Err(Exit::ConfigError)
        }
        (None, Some(from_kind)) => Ok(from_kind),
    }
}

/// The kind→lane reading. `bug` is the reactive lane; `story` is the planned one.
const fn lane_of_kind(kind: &str) -> Option<Lane> {
    match kind.as_bytes() {
        b"bug" => Some(Lane::Fix),
        b"story" => Some(Lane::Story),
        _ => None,
    }
}

/// ADR-0128 decision 3's two-row map: `fix` → `bug-standard`, `story` → `story`.
const fn pack_for(lane: Lane) -> &'static str {
    match lane {
        Lane::Fix => "bug-standard",
        Lane::Story => "story",
    }
}

/// The lane's own spelling, for a diagnostic. Never parsed back.
const fn lane_name(lane: Lane) -> &'static str {
    match lane {
        Lane::Fix => "fix",
        Lane::Story => "story",
    }
}

/// Rebuild the kernel's `Item` from the record.
///
/// **An absent facet is not written at all**, which is what makes it `with_facet_null` inside the
/// kernel and is how a `require` row abstains and a gate opens (AC-079 / ADR-0089). A facet written
/// as an explicit `false` is a DECIDED false and opens nothing — the two are different facts and the
/// fixtures separate them.
fn build_item(record: &ItemRecord) -> Item {
    let mut item = Item::new(&record.id, record.kind.as_deref().unwrap_or(""));
    for (name, value) in &record.facets {
        item = match value {
            toml::Value::Boolean(b) => item.with_facet_bool(name, *b),
            toml::Value::String(s) => item.with_facet_str(name, s),
            toml::Value::Array(values) => item.with_facet_set(
                name,
                values
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect::<Vec<_>>(),
            ),
            // A shape this stage does not read is left UNSET rather than coerced: an unset facet is
            // an honest abstention, and a coerced one would be a decided value nobody wrote.
            _ => item,
        };
    }
    item
}

// ------------------------------------------------------------------ steps (8)-(10), the inner path

/// The recorded input set, with the tree hash taken over the repository.
///
/// **`pub` on purpose**: the in-process tests build a `RunInputs` at a FIXED instant so nothing
/// they assert moves with the wall clock, and §7d.5 gives the root exactly one clock read — a test
/// that read its own would be a second one.
///
/// # Errors
/// An [`Exit`] row when a registry will not read, or the tree hash cannot be taken.
pub fn prepare_inputs(resolved: &Resolved, now: i64) -> Result<rk_kernel::replay::RunInputs, Exit> {
    let tree_hash = rk_git::tree::tree_hash(
        &resolved.exec,
        &resolved.worktree,
        &resolved.common_dir,
        &rk_git::tree::TreeBasis::WorkingState,
    )
    .map_err(|e| {
        eprintln!("redkiln: the tree hash could not be taken: {e:?}");
        Exit::ConfigError
    })?;
    base_inputs(resolved, tree_hash.as_str(), now)
}

/// Every `.redkiln/catalog/*.toml`, by name, in sorted order so two runs read one order.
fn catalog_sources(root: &Path) -> Result<Vec<(String, String)>, Exit> {
    let dir = root.join(".redkiln").join("catalog");
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .map_err(|e| {
            eprintln!("redkiln: {}: {e}", dir.display());
            Exit::ConfigError
        })?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "toml"))
        .collect();
    paths.sort();
    let mut out = Vec::new();
    for path in paths {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let src = std::fs::read_to_string(&path).map_err(|e| {
            eprintln!("redkiln: {}: {e}", path.display());
            Exit::ConfigError
        })?;
        out.push((name, src));
    }
    Ok(out)
}

/// The recorded input set, minus the `with_spec` bindings step (8) adds.
///
/// # 🛑 ONE WALL-CLOCK READ, AND BOTH DERIVED VALUES COME OFF IT
///
/// The gate deadline is `now + gate_deadline_secs` and `results_at` is that same `now`. Two reads
/// would make a run's own deadline and its recorded commit instant disagree by however long step
/// (8) took, and §7d.5 gives the composition root exactly one read.
///
/// # 🛑 `.results_at(..)` IS ABSENT FROM THE WHOLE STAGE FILE, AND EVERY DISPATCHING RUN REFUSES
/// WITHOUT IT
///
/// `grep -n results_at runbook/phase-1/s7d-cli-and-migrate.md` returns **zero hits**, yet
/// `derive`'s dispatch envelope calls `runtime.require_results_at()?`, which refuses rather than
/// defaulting — *"a zero `at` is a commit stamped at the epoch"*. `RuntimeContext::supplied` leaves
/// it `None`, so a root that chained only the tree hash and the deadline would refuse on
/// `inputs/runtime.json` **before any dispatch** — the wrong reason for four of this cycle's five
/// tests. RAISED as a stage-file omission; this chain is the repair.
fn base_inputs(
    resolved: &Resolved,
    tree_hash: &str,
    now: i64,
) -> Result<rk_kernel::replay::RunInputs, Exit> {
    let root = Path::new(resolved.worktree.as_str()).to_path_buf();
    let sources = catalog_sources(&root)?;
    let artifacts_src = std::fs::read_to_string(root.join(".redkiln").join("artifacts.toml"))
        .map_err(|e| {
            eprintln!("redkiln: .redkiln/artifacts.toml: {e}");
            Exit::ConfigError
        })?;
    let facets_src =
        std::fs::read_to_string(root.join(".redkiln").join("facets.toml")).map_err(|e| {
            eprintln!("redkiln: .redkiln/facets.toml: {e}");
            Exit::ConfigError
        })?;
    let borrowed: Vec<(&str, &str)> = sources
        .iter()
        .map(|(n, s)| (n.as_str(), s.as_str()))
        .collect();
    let deadline = now.saturating_add(resolved.extras.gate_deadline_secs);
    Ok(rk_kernel::replay::RunInputs::new(&resolved.stage)
        .with_pack(&format!("{}.toml", resolved.pack_name), &resolved.pack_src)
        .with_catalog(&borrowed)
        .with_artifacts("artifacts.toml", &artifacts_src)
        .with_facets(&facets_src)
        .with_item(&resolved.item)
        // ONE `with_config`, over the value step (4) constructed — never a second `Config::new`.
        .with_config(resolved.config.clone())
        .with_runtime(
            rk_kernel::replay::RuntimeContext::supplied(tree_hash, deadline).results_at(now),
        ))
}

/// Steps (9) and (10): the journal read, `replay::resume`, the run record, and the terminal read.
///
/// # Errors
/// [`Halt::PoisonedLock`] from the journal acquisition.
fn resume_and_read(
    resolved: &Resolved,
    inputs: &rk_kernel::replay::RunInputs,
    ports: rk_ports::root::Ports,
    supply_spec: &mut dyn FnMut(
        &str,
    ) -> Result<
        rk_kernel::replay::RecordedSpec,
        rk_kernel::replay::ReplayError,
    >,
    token: &CancelToken,
) -> Result<Exit, Halt> {
    let item_id = resolved.item.id().to_owned();
    let store = Arc::clone(&ports.store);

    // `prior` is the store's journal FOR THE ITEM, so the fold `next_action` reads is the recorded
    // one and a second `run` resumes past an applied answer instead of re-deriving `OpenGate` at
    // reach 1. This is also what makes a committed step memoised across two runs (AC-401).
    let prior = {
        let guard = crate::store::acquire(&store, "the run")?;
        match guard.query(rk_kernel::runtime::journal_query(&item_id)) {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("redkiln: the store could not be read: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
    };

    if token.is_cancelled() {
        eprintln!("redkiln: interrupted; nothing after the journal read for `{item_id}` ran.");
        return Ok(Exit::SignalOutsideDispatch);
    }

    let recorded = match rk_kernel::replay::resume(inputs, ports, &prior, supply_spec) {
        Ok(run) => run,
        // 🛑 A FAILED DISPATCH IS NOT A CONFIGURATION ERROR, AND THE CLASS IS NOT ON THIS VALUE.
        //
        // `Runtime::run_activity` carries a step's failure as a VALUE in its report — "a step's own
        // failure is a value in the report (AC-032), never an error of this function" — and
        // `replay::dispatch_through` turns an uncommitted report into `NotCommitted { step_id,
        // activity_id_at_version }`, whose two members are both `String`. All eleven `StepError`
        // rows therefore arrive here as the SAME value.
        //
        // The class survives in exactly one place: the row the harness appended before returning,
        // through `StepError::event()`. That map is bijective over the eleven, so the journal is
        // where §7d.4's table is read from. See [`exit_for_uncommitted`].
        Err(rk_kernel::replay::ReplayError::NotCommitted {
            step_id,
            activity_id_at_version,
        }) => {
            eprintln!(
                "redkiln: the dispatch of `{step_id}` ({activity_id_at_version}) did not commit."
            );
            return exit_for_uncommitted(&store, &item_id);
        }
        Err(e) => {
            eprintln!("redkiln: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    // §7d.7's persistence: the run record `explain` reads.
    let runs_dir = resolved.common_dir.join("redkiln").join("runs");
    if let Err(e) = rk_kernel::replay::write_run(&runs_dir, &recorded) {
        eprintln!("redkiln: the run record could not be written: {e}");
        return Ok(Exit::ConfigError);
    }

    Ok(terminal_read(&item_id, &recorded))
}

/// Step (10): the terminal read.
///
/// **`run` NEVER WAITS** — no channel, prompt, socket or sleep is entered on any arm (golden
/// constraint 2). A gate among the resumed run's new events is an EXIT, not a wait.
fn terminal_read(item_id: &str, recorded: &rk_kernel::replay::RecordedRun) -> Exit {
    let gate = recorded
        .events
        .iter()
        .rev()
        .find(|e| e.kind == rk_types::event_kind::EventKind::GateOpened);
    if let Some(row) = gate {
        let token_hex = row
            .payload
            .get("token")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default();
        println!("gate {token_hex} is open for {item_id}");
        return Exit::Success;
    }
    Exit::Success
}

/// §7d.4's table, applied to a dispatch that did not commit.
///
/// The class is read off the **journal**, because `ReplayError::NotCommitted` does not carry it —
/// see the note at the call site. The LAST error row wins: a run that retried within its cap wrote
/// one row per attempt, and the exit describes the attempt the run ended on.
///
/// A run that did not commit and wrote no error row at all is a configuration outcome rather than a
/// class: `Runtime::run_activity` MINTS `StepError::Schema` itself when a returned `Typed`'s schema
/// is not the spec's, and on that path the harness returned `Ok` and appended nothing. Nothing in
/// Slice 3 reaches it — the in-process arm scripts `Err` values, which the harness always journals —
/// but cycle 6's F20 fixture does, and this arm is where it will surface. **RAISED rather than
/// guessed at here**: a `ConfigError` for a case whose true row is `11` is visibly wrong, and
/// writing a speculative `Schema` here would make it invisible instead.
fn exit_for_uncommitted(store: &Arc<Mutex<dyn EventStore>>, item_id: &str) -> Result<Exit, Halt> {
    let rows = {
        let guard = crate::store::acquire(store, "the run")?;
        match guard.query(rk_kernel::runtime::journal_query(item_id)) {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("redkiln: the store could not be read: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
    };
    let Some(exit) = rows.iter().rev().find_map(exit_for_error_row) else {
        eprintln!(
            "redkiln: the dispatch did not commit and the journal carries no error row for \
             `{item_id}`, so no row of the exit table describes it."
        );
        return Ok(Exit::ConfigError);
    };
    Ok(exit)
}

/// One journal row → its row of §7d.4's table, or `None` if the row is not an error row.
///
/// **The two class matches carry no wildcard, and that is the point.** `FailureClass` and
/// `RefusalClass` are closed enums in `rk-types`, and `StepError::event()` is documented as
/// *"Exactly one per `StepError` variant"* — so a twelfth `StepError` forces a new class arm there,
/// and a new class arm is a compile error HERE rather than a silent `ConfigError`. That is the same
/// guarantee `impl From<&StepError> for Exit` gives one level up, kept across the journal round
/// trip; matching on the payload's STRING would have thrown it away.
fn exit_for_error_row(row: &rk_types::event::Event) -> Option<Exit> {
    use rk_types::event::{FailureClass, RefusalClass};
    use rk_types::event_kind::EventKind;

    // A free generic rather than a closure: a closure would be monomorphised to whichever class
    // type reached it first, and the two arms below read different ones.
    fn class<T: serde::de::DeserializeOwned>(row: &rk_types::event::Event) -> Option<T> {
        row.payload
            .get("class")
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok())
    }
    match row.kind {
        EventKind::StepFailed => Some(match class::<FailureClass>(row)? {
            FailureClass::Transport => Exit::Transport,
            FailureClass::Schema => Exit::Schema,
            FailureClass::Timeout => Exit::Timeout,
            FailureClass::NonZero => Exit::NonZero,
            FailureClass::Platform => Exit::Platform,
        }),
        EventKind::StepRefused => Some(match class::<RefusalClass>(row)? {
            RefusalClass::NotAdopted => Exit::NotAdopted,
            RefusalClass::NotIsolated => Exit::NotIsolated,
            RefusalClass::Unsupported => Exit::Unsupported,
            RefusalClass::Overage => Exit::Overage,
        }),
        EventKind::StepCancelled => Some(Exit::Cancelled),
        // Both the failure row and the retry MARK carry this kind (they differ only in the
        // payload's `kind = "retry_at"`), and both mean the same row of the table.
        EventKind::RateLimited => Some(Exit::RateLimited),
        _ => None,
    }
}
