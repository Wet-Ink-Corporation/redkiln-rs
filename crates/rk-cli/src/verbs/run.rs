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
use crate::verbs::Startup;
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
    /// The optional `[adapters] harness_binary` key (RT-49) — the program every dispatch spawns.
    /// `None` is the ordinary case and means the adapter's own default.
    pub harness_binary: Option<String>,
    /// `[adapters] toolchains`, in the file's **declared order** — ADR-0137 clause 2 selects by
    /// `detect()` in this order. Deliberately not the kernel `Config`'s copy, which `Config::new`
    /// SORTS so the recorded digest is a function of the set rather than of the file's order.
    pub toolchains: Vec<String>,
    /// RT-49's per-toolchain program overrides.
    pub toolchain_programs: Option<std::collections::BTreeMap<String, String>>,
    /// ADR-0137 clause 5's construction-time backstop, in seconds.
    pub toolchain_timeout_secs: u64,
}

/// **ADR-0137 clause 5.** The toolchain adapters' default construction-time backstop.
///
/// Measured at `s7g` over `.redkiln/catalog/`: the largest declared literal `timeout_secs` on any
/// step is **3600**. The backstop may not be smaller than the largest step it could serve, or a
/// declared step timeout would be capped without saying so — so the floor is measured, not chosen.
pub const DEFAULT_TOOLCHAIN_TIMEOUT_SECS: u64 = 3600;

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
    let resolved = match through_step_seven(cli, args, env, startup, Entry::Run) {
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
    let token = CancelToken::new();

    // ---- (8) the run-owned worktree, the bindings, and ONLY THEN the real harness. -------------
    let owned = match recreate_run_worktree(
        &resolved.exec,
        &resolved.worktree,
        &resolved.common_dir,
        resolved.item.id(),
    ) {
        Ok(path) => path,
        Err(exit) => return Ok(exit),
    };

    // ---- (6)'s ROW, NOW THAT (8) HAS RE-CREATED THE WORKTREE — all four members.
    //
    // §7d.5 step (6): *"the row is written once (8) has re-created the worktree, with all four
    // members, or at (7)'s refusal without the pair"*. This is the invocation's FIRST and ONLY
    // `reconcile_started` row (RT-46), and the pair on it is what `answer`'s tier-1 write later
    // resolves the run-owned worktree FROM — ADR-0105 clause 8's *"from the store, regardless of
    // cwd"*, AC-529's same phrase. A convention-derived path would satisfy neither.
    let ownership = rk_git::worktree::RunOwnership {
        worktree: owned.clone(),
        branch: format!("rk-run/{}", resolved.item.id()),
    };
    match reconcile_started_event(&resolved.config, Some(&ownership)) {
        Ok(event) => {
            let mut guard = crate::store::acquire(&store, "the run")?;
            let appended = guard.append(event);
            drop(guard);
            if let Err(e) = appended {
                eprintln!("redkiln: the invocation's first row could not be written: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
        Err(exit) => return Ok(exit),
    }

    // THE ONE WALL-CLOCK READ. Both derived values come off it — see `base_inputs`.
    let now = crate::clock::now().unix_seconds();
    let (inputs, tree_hash) = match prepare_inputs(&resolved, now) {
        Ok(pair) => pair,
        Err(exit) => return Ok(exit),
    };

    let claude = match real_harness_config(&resolved, env) {
        Ok(config) => config,
        Err(exit) => return Ok(exit),
    };
    let mut binder = rk_adapter_claude::binder::Binder::new(claude);
    let inputs = match bind_nodes(&resolved, inputs, &mut binder, &owned) {
        Ok(inputs) => inputs,
        Err(exit) => return Ok(exit),
    };

    // **ONLY THEN** the harness, over the config the bindings filled (§7d.5's own order):
    // `ClaudeHarness::dispatch` refuses a `card_digest` its validated store does not hold, and
    // `Binder::new(config).bind(..)` then `into_config()` is the one way that store is filled.
    let harness = match rk_adapter_claude::ClaudeHarness::new(binder.into_config(), token.clone()) {
        Ok(harness) => harness,
        Err(e) => {
            eprintln!("redkiln: the harness could not be built: {e}");
            return Ok(Exit::ConfigError);
        }
    };

    // **ADR-0137: the REAL toolchain, over the run-owned worktree.** Until `s7g` this slot held
    // `FixtureToolchain::default()`, whose `Default` is `present: false` — a toolchain reporting
    // itself absent, which is why AC-473's clause has never had a mechanism.
    let toolchain = match real_toolchain(&resolved, env, &owned) {
        Ok(toolchain) => toolchain,
        Err(exit) => return Ok(exit),
    };

    let ports = rk_ports::root::Ports {
        store,
        harness: Arc::new(harness),
        // The two with no Phase-1 real (ADR-0094 decision 2; design:265-266 — GitHub and mdBook
        // are Phase 3, which is ADR-0134's warrant for the port lint's expected-uncalled set).
        tracker: Arc::new(rk_ports::fakes::BoardTracker::default()),
        docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
        toolchain,
    };

    // Every node the loop reaches was bound above; the supplier is the seam for a node first seen
    // INSIDE the loop, and it refuses by name rather than silently binding nothing (RT-55).
    let mut supply_spec =
        |node: &str| -> Result<rk_kernel::replay::RecordedSpec, rk_kernel::replay::ReplayError> {
            Err(rk_kernel::replay::ReplayError::Malformed {
                path: "inputs/specs.json".to_owned(),
                detail: format!(
                    "`{node}` was first seen inside the loop and step (8) bound no spec for it"
                ),
            })
        };

    run_with(
        &resolved,
        &inputs,
        now,
        &tree_hash,
        ports,
        &mut supply_spec,
        &token,
    )
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
#[expect(
    clippy::too_many_arguments,
    reason = "`now` and `tree_hash` are step (8)'s two readings threaded into step (9); taking \
              either here would be a second clock read (§7d.5) or a second tree hash over one \
              worktree, and `RunInputs::runtime()` is private to rk-kernel so neither can be read \
              back off `inputs`"
)]
pub fn run_with(
    resolved: &Resolved,
    inputs: &rk_kernel::replay::RunInputs,
    now: i64,
    tree_hash: &str,
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

    resume_and_read(resolved, inputs, now, tree_hash, ports, supply_spec, token)
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
    entry: Entry,
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

    // ---- (6) RECONCILE AT ENTRY, BEFORE STEP (7). AC-402, ADR-0105 clause 8.
    //
    // §7d.5 numbers this SIXTH and the pack set SEVENTH, and the order is observable: §7d.5's own
    // table says *"a refused `run` that reached the reconcile (no pack; non-derivable lane; a lane
    // whose pack is not enabled) writes **exactly one** `reconcile_started` row"*. A reconcile
    // placed after the pack check could not write that row, and AC-172's *"exactly one
    // `reconcile_started` event per invocation"* would be unmet on exactly the paths D18-8 names.
    //
    // The ENTRY is a parameter because `answer` performs steps (1)-(6) as `run`'s with
    // `Entry::Answer` (§7d.6), and `rk_git::reconciled::Entry` has no `Status` variant — the type
    // is what forbids `status` and `explain` from reaching here at all (AC-402's negative).
    let _ = startup
        .reconciler
        .reconcile(&repo.exec, &repo.worktree, entry)
        .map_err(|e| {
            eprintln!("redkiln: reconcile at entry refused: {e:?}");
            Exit::ConfigError
        })?;

    // ---- (7) THE PACK SET, BEFORE ANY LANE DERIVATION.
    //
    // D18-8, and the order is the whole point: "an empty `[packs] enabled` is exit 3 with 'no
    // process pack enabled' BEFORE any lane derivation — so the refusal is the same whatever item
    // was named". The scaffold `init` writes carries `enabled = []`, so this is the init-only
    // repository's refusal and it is reached with no item record read at all.
    // 🛑 EVERY REFUSAL BELOW HAS RECONCILED, SO EVERY ONE OF THEM OWES AC-172's SINGLE ROW.
    // `refuse_after_reconcile` opens the store only on these paths; the `Ok` arm leaves it to
    // step (5).
    if config.packs.enabled.is_empty() {
        eprintln!(
            "redkiln: no process pack enabled — `.redkiln/config.toml`'s `[packs] enabled` is \
             empty, so there is no pack to run `{}` under.",
            args.item
        );
        return Err(refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        ));
    }

    // The item record. Read AFTER the pack-set refusal, so the precedence above is real.
    let record_path = root
        .join(".redkiln")
        .join("items")
        .join(format!("{}.toml", args.item));
    let record_src = std::fs::read_to_string(&record_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", record_path.display());
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;
    let record: ItemRecord = toml::from_str(&record_src).map_err(|e| {
        eprintln!(
            "redkiln: {} is not an item record: {e}",
            record_path.display()
        );
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;

    // RT-50. Refused rather than defaulted, and the refusal NAMES THE ITEM.
    let Some(stage) = record.stage.clone() else {
        eprintln!(
            "redkiln: no stage on the item record for `{}` — the stage is read once, from the \
             record, and is not defaulted.",
            args.item
        );
        return Err(refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        ));
    };

    // ---- the lane. ADR-0128 decision 2.
    let lane = derive_lane(args, record.kind.as_deref())
        .map_err(|exit| refuse_after_reconcile(&repo.exec, &repo.worktree, &kernel_config, exit))?;
    let pack_name = pack_for(lane);

    // Decision 3: "a lane whose pack is not enabled is the same configuration error with the pack
    // named".
    if !config.packs.enabled.iter().any(|p| p == pack_name) {
        eprintln!(
            "redkiln: the `{}` lane names the `{pack_name}` pack, which is not in \
             `.redkiln/config.toml`'s `[packs] enabled`.",
            lane_name(lane)
        );
        return Err(refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        ));
    }

    let pack_path = root
        .join(".redkiln")
        .join("packs")
        .join(format!("{pack_name}.toml"));
    let pack_src = std::fs::read_to_string(&pack_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", pack_path.display());
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;
    let extras: PackExtras = toml::from_str(&pack_src).map_err(|e| {
        eprintln!(
            "redkiln: {} does not carry the adapter-facing pack values: {e}",
            pack_path.display()
        );
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;

    // §7d.5 step (7) names `.redkiln/facets.toml` among the registries this step reads. It is read
    // HERE and not only in `base_inputs`, because the item's context vector is built from it: a
    // facet the registry declares and the record omits is an abstention, and `build_item` cannot
    // know which those are without it.
    let facets_path = root.join(".redkiln").join("facets.toml");
    let facets_src = std::fs::read_to_string(&facets_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", facets_path.display());
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;
    let registry = rk_kernel::registry::Registry::from_toml(&facets_src).map_err(|e| {
        eprintln!("redkiln: {}: {e}", facets_path.display());
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
    })?;
    let item = build_item(&record, &registry);

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
        harness_binary: config.adapters.harness_binary.clone(),
        toolchains: config.adapters.toolchains.clone(),
        toolchain_programs: config.adapters.toolchain_programs.clone(),
        toolchain_timeout_secs: config
            .adapters
            .toolchain_timeout_secs
            .unwrap_or(DEFAULT_TOOLCHAIN_TIMEOUT_SECS),
    })
}

/// **The invocation's one `reconcile_started` row** — §7d.5 step (6), RT-41 and RT-47.
///
/// > §7d.5 step (6): *"the invocation's one `reconcile_started` row carrying `adapter_set` and
/// > `config_digest` (RT-41) and, once the run-owned worktree is re-created at (8), the item's
/// > `RunOwnership { worktree, branch }` (RT-47; H-5) — the row is written once (8) has re-created
/// > the worktree, **with all four members**, or at (7)'s refusal without the pair (no worktree was
/// > created), and is in either case the invocation's first and only `reconcile_started` row
/// > (RT-46)."*
///
/// `ownership` is `Some` exactly when step (8) has run. `answer` never creates a worktree, so its
/// own row carries the two members and it READS the pair off the row a previous `run` wrote — which
/// is what ADR-0105 clause 8's *"resolves the run-owned worktree **from the store**"* means.
///
/// # 🛑 BOTH MEMBERS ARE ROUTED, NOT DECIDED, AND THE RECORD IS OWED
///
/// RT-41 puts `adapter_set` and `config_digest` on this row because AC-485's *"the adapter set
/// recorded on the **run-start event**"* names an event no design §5 row carries — `EventKind::ALL`
/// has no `run_started`. RT-47 puts the ownership pair on the same row for the same reason. §7d.18
/// holds both as **OWED**: either a dated design §5 note widening this payload, or a minted kind by
/// ADR-0116's route. Built here, labelled routed; the id is not claimed.
pub fn reconcile_started_event(
    config: &rk_kernel::config::Config,
    ownership: Option<&rk_git::worktree::RunOwnership>,
) -> Result<rk_types::event::Event, Exit> {
    let digest = rk_store::canon::canonical_digest(&config.to_json()).map_err(|e| {
        eprintln!("redkiln: the configuration will not canonicalise: {e}");
        Exit::ConfigError
    })?;
    // The resolved four, in the order `AdapterSet::resolve` fixed them and `Config::new` sorted the
    // toolchains into — a function of the SET, never of the order the file listed it in.
    let mut adapter_set = vec![config.harness().to_owned(), config.store().to_owned()];
    adapter_set.extend(config.toolchains().iter().cloned());

    let mut payload = serde_json::json!({
        "adapter_set": adapter_set,
        "config_digest": digest.as_str(),
    });
    if let (Some(own), Some(map)) = (ownership, payload.as_object_mut()) {
        map.insert(
            "worktree".to_owned(),
            serde_json::Value::String(own.worktree.display().to_string()),
        );
        map.insert(
            "branch".to_owned(),
            serde_json::Value::String(own.branch.clone()),
        );
    }
    Ok(rk_types::event::Event::new(
        rk_types::event_kind::EventKind::ReconcileStarted,
        payload,
    ))
}

/// **Step (9)'s EXPIRY PASS** — every open gate of the item whose recorded deadline is past is
/// resolved through `gate::expire` with the pack's declared action.
///
/// §7d.6 calls this from `answer` too, and says why it must exist there: *"without it
/// `gate::answer` would `Applied` a gate whose deadline merely passed, because it refuses
/// `Expired` only on a `gate_expired` row already in the journal"*. The expiry is therefore
/// **observed at a read** (AC-257: *"observed and resolved at that read"*, *"with no process alive
/// at the deadline"*) rather than by anything running at the deadline — there is no timer and no
/// daemon, and that is the design, not a limitation.
///
/// # 🛑 THE CLASS IS THE FULL GATE KIND, AXIS AND ALL
///
/// `GateRecord::kind()` is `fsm::next_action`'s `format!("{}:{}", gate.reason.as_str(), gate.axis)`
/// — `unknown_facet:has_repro`, never a bare `unknown_facet`. A `[[gate_expiry]]` row keyed on the
/// bare reason matches nothing, and the miss surfaces only here as
/// `ExpiryPolicyLint::NoActionDeclared`. **There is no default** (AC-260): an undeclared class is a
/// configuration refusal naming the class, not a `fail` chosen on the operator's behalf.
///
/// # 🛑 `auto-decide` IS REFUSED AT 7d, AND THE REFUSAL NAMES WHY
///
/// `ExpiryAction::AutoDecide { verdict }` needs a verdict, and `[[gate_expiry]]` carries only
/// `class` and `action` — the pack schema has nowhere to put one. Inventing a verdict here would be
/// deciding an answer on the operator's behalf, which is the one thing an expiry policy must not
/// do silently. The carrier is 7e's (RT-57, §7d.18 RAISED); until it exists the action is refused
/// by name.
///
/// # Errors
/// [`Exit::ConfigError`] for an undeclared or unusable expiry row, and for a kernel refusal.
pub fn expiry_pass(
    extras: &PackExtras,
    state: &rk_kernel::workflow::fsm::RunState,
    now: i64,
    tree_hash: &str,
) -> Result<Vec<rk_types::event::Event>, Exit> {
    use rk_kernel::workflow::gate;

    // Collected first so the iteration does not hold a borrow across the `expire` calls, and so the
    // order is the journal's rather than a hash map's.
    let due: Vec<(rk_types::digest::Digest, String)> = state
        .open_gates()
        .filter(|record| record.deadline().unix_seconds() < now)
        .map(|record| (record.token().clone(), record.kind().to_owned()))
        .collect();

    let mut events = Vec::new();
    for (token, class) in due {
        let declared = extras
            .gate_expiry
            .iter()
            .find(|row| row.class == class)
            .map(|row| row.action.as_str());

        if let Err(lint) = gate::lint_expiry_policy_row(&class, declared) {
            eprintln!(
                "redkiln: the pack declares no usable expiry action for gate class `{class}`: \
                 {lint:?}. AC-260 forbids a default, so the gate cannot be resolved."
            );
            return Err(Exit::ConfigError);
        }

        let action = match declared {
            Some("fail") => gate::ExpiryAction::Fail,
            Some("escalate") => gate::ExpiryAction::Escalate {
                deadline: rk_types::time::Timestamp::from_unix_seconds(
                    now.saturating_add(extras.gate_deadline_secs),
                ),
            },
            Some("auto-decide") => {
                eprintln!(
                    "redkiln: gate class `{class}` declares `auto-decide`, but a `[[gate_expiry]]` \
                     row carries no verdict for the kernel to answer with, and one is not invented \
                     here. The verdict carrier is Step 7e's (RT-57)."
                );
                return Err(Exit::ConfigError);
            }
            // Unreachable: the lint above admits exactly the three spellings.
            Some(_) | None => return Err(Exit::ConfigError),
        };

        let outcome = gate::expire(
            state,
            &token,
            rk_types::time::Timestamp::from_unix_seconds(now),
            &action,
            tree_hash,
        )
        .map_err(|e| {
            eprintln!(
                "redkiln: the expiry of gate {} was refused: {e}",
                token.as_str()
            );
            Exit::ConfigError
        })?;
        events.extend(outcome.events);
    }
    Ok(events)
}

/// Append the row at a refusal that has already reconciled, opening the store to do it.
///
/// # 🛑 THE STORE IS OPENED **ONLY HERE**, ON THE REFUSAL PATH
///
/// The success path leaves the store to `run`'s step (5), which is what keeps
/// `support::resolved_over_pack_min`'s *"re-running steps (1)–(7) to get them is cheap and writes
/// nothing"* true: that helper only ever takes the `Ok` arm. A refusal that reached step (6) owes
/// AC-172's *"exactly one `reconcile_started` event per invocation"* and D18-8 as corrected, and it
/// cannot pay it without a store.
///
/// A store that will not open at this point is **not** escalated: the refusal being reported is the
/// operator's answer, and replacing it with a store error would hide it.
fn refuse_after_reconcile(
    exec: &GitExec,
    worktree: &WorktreePath,
    config: &rk_kernel::config::Config,
    exit: Exit,
) -> Exit {
    if let Ok(event) = reconcile_started_event(config, None)
        && let Ok(mut store) = rk_adapter_sqlite::SqliteStore::open(exec, worktree)
    {
        let _ = store.append(event);
    }
    exit
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
/// The item, rebuilt from its record — with **every facet the registry declares**, decided or null.
///
/// # 🛑 AN ABSENT FACET IS `with_facet_null`, AND WITHOUT THAT NO GATE EVER OPENS
///
/// §7d.5 step (7), in terms: *"an absent facet is **`with_facet_null`**, which is how a `require`
/// row abstains and a gate opens — AC-079 / ADR-0089"*.
///
/// Until cycle 6 this loop walked the RECORD's facets only, so a facet the item omitted was never
/// on the context vector at all — and the predicate evaluator returned
/// `EvalError::UnknownField("has_repro")` rather than `Tri::Null`. Measured 2026-09-20 against the
/// wired binary, on `gate-1` (whose `[facets]` table is empty on purpose):
/// *"bug-standard.toml: stage.implement.require[0]: unknown field \"has_repro\""*, exit 3.
///
/// So the **abstention gate was unreachable**, and with it every gate case of Slice 3. That is why
/// the contract sequences `ac403_…` first in this cycle: its fixture is reused by `ac406_…`,
/// `ac407_…`, `ac264_…` and the AC-410 pair, and none of them could have opened a gate either.
fn build_item(record: &ItemRecord, registry: &rk_kernel::registry::Registry) -> Item {
    let mut item = Item::new(&record.id, record.kind.as_deref().unwrap_or(""));

    // Every declared facet the record does not carry, as an abstention. Done first, so a record
    // that does carry one overwrites the null rather than the other way round.
    //
    // 🛑 TWO KINDS OF FACET ARE NOT THE RECORD'S TO ABSTAIN FROM, and nulling them mints gates
    // nobody asked for. Measured: nulling all four of `pack-min`'s declared facets produced THREE
    // `gate_opened` rows where one was owed.
    //
    //  * `kind` is `Item::new(id, kind)`'s — it is the record's TOP-LEVEL key, not a `[facets]`
    //    entry, and it is already decided by the line above.
    //  * `source = "kernel"` facets (`stage`, `rigor` here) are the KERNEL's to supply. An item
    //    record may not declare one, so its absence from `[facets]` is not an abstention — it is
    //    the ordinary case, and §7b's own rule that a kernel facet is never human-sourced.
    //
    // What is left is exactly what §7d.5 means: a facet the pack may `require` on and the item's
    // author could have decided and did not.
    for facet in registry.facets() {
        let kernel_supplied = facet.source == rk_kernel::registry::Source::Kernel;
        if facet.name == "kind" || kernel_supplied || record.facets.contains_key(&facet.name) {
            continue;
        }
        item = item.with_facet_null(&facet.name);
    }

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

/// The recorded input set, **and the tree hash it was built over**.
///
/// **`pub` on purpose**: the in-process tests build a `RunInputs` at a FIXED instant so nothing
/// they assert moves with the wall clock, and §7d.5 gives the root exactly one clock read — a test
/// that read its own would be a second one.
///
/// # 🛑 THE HASH IS RETURNED RATHER THAN DROPPED, BECAUSE STEP (9) NEEDS THE SAME ONE
///
/// `gate::expire` takes a tree hash, so the expiry pass of step (9) needs one — and it must be the
/// hash this function already took, not a second reading. `RuntimeContext` carries it, but
/// `RunInputs::runtime()` is private to `rk-kernel` (measured), so it cannot be read back out of
/// the value returned here. Taking it twice would be two `git` invocations over one worktree and
/// two values that are only *probably* equal; returning it is one reading, used twice.
///
/// # Errors
/// An [`Exit`] row when a registry will not read, or the tree hash cannot be taken.
pub fn prepare_inputs(
    resolved: &Resolved,
    now: i64,
) -> Result<(rk_kernel::replay::RunInputs, String), Exit> {
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
    let inputs = base_inputs(resolved, tree_hash.as_str(), now)?;
    Ok((inputs, tree_hash.as_str().to_owned()))
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
#[expect(
    clippy::too_many_arguments,
    reason = "step (9) is one numbered step and reads five recorded values; splitting it would put \
              §7d.5's order in the call graph rather than in one readable sequence"
)]
fn resume_and_read(
    resolved: &Resolved,
    inputs: &rk_kernel::replay::RunInputs,
    now: i64,
    tree_hash: &str,
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

    // ---- STEP (9)'s EXPIRY PASS, BEFORE THE GATE READ `resume` PERFORMS. -----------------------
    //
    // §7d.5 step (9), verbatim: *"**before the gate read**, every open gate of the item whose
    // recorded deadline `< now` is resolved through `gate::expire(..)` and its events appended"*.
    //
    // 🛑 NOTHING ELSE CAN RESOLVE A PASSED DEADLINE. `gate::answer` tests `gate_record(presented)`
    // — returning `Applied` — BEFORE it tests `is_expired`, and `fsm::apply`'s `GateExpired` arm
    // deliberately KEEPS the record in `gates_open`. So the kernel's `Expired` branch fires only
    // for a token already carrying a `gate_expired` row, and a gate whose deadline merely passed is
    // still live to every reader until this pass writes that row (AC-257; measured at cycle 7 and
    // RAISED against §7d.6's own phrasing).
    //
    // `answer` calls the same function for the same reason (§7d.6: *"the expiry pass of `run`'s
    // step (9)"*) — it is built once, here, and called from both.
    let state = match rk_kernel::workflow::fsm::fold(&item_id, &resolved.stage, &prior) {
        Ok(state) => state,
        Err(e) => {
            eprintln!("redkiln: the recorded journal did not fold: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let expiry_events = match expiry_pass(&resolved.extras, &state, now, tree_hash) {
        Ok(events) => events,
        Err(exit) => return Ok(exit),
    };

    // The rows are appended and the journal is RE-READ rather than extended locally: `resume` folds
    // what the STORE holds, so a pass that wrote nowhere would be invisible to it, and reading it
    // back is what makes the append observable rather than assumed.
    let prior = if expiry_events.is_empty() {
        prior
    } else {
        let mut guard = crate::store::acquire(&store, "the run")?;
        for event in expiry_events {
            if let Err(e) = guard.append(event) {
                drop(guard);
                eprintln!("redkiln: an expiry row could not be written: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
        let re_read = guard.query(rk_kernel::runtime::journal_query(&item_id));
        drop(guard);
        match re_read {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("redkiln: the store could not be read: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
    };

    let recorded = match rk_kernel::replay::resume(inputs, ports, &prior, supply_spec) {
        Ok(run) => run,
        // 🛑 A FAILED DISPATCH IS NOT A CONFIGURATION ERROR, AND THE CLASS NOW ARRIVES ON THIS VALUE.
        //
        // §7d.5 step (10): *"a `StepError` → its row"*. `Runtime::run_activity` carries a step's
        // failure as a VALUE in its report — *"a step's own failure is a value in the report
        // (AC-032), never an error of this function"* — and until Slice 3 cycle 10
        // `replay::dispatch_through` discarded it into `NotCommitted { step_id,
        // activity_id_at_version }`, two `String`s, so all eleven rows arrived here as the SAME
        // value and step (10) had nothing to map.
        //
        // The old repair read the class back off the journal, which worked only because
        // `FakeHarness` appends an error row through `StepError::event()`. `ClaudeHarness` appends
        // one `dispatch_completed` carrying `cancelled_by` and **no class member**, so against the
        // real adapter the journal held nothing to read and every failed dispatch exited `3`.
        // Mechanism B carries the class on the refusal instead, and `impl From<&StepError> for
        // Exit` — eleven arms, no wildcard — is the one mapping.
        Err(rk_kernel::replay::ReplayError::NotCommitted {
            step_id,
            activity_id_at_version,
            class,
        }) => {
            eprintln!(
                "redkiln: the dispatch of `{step_id}` ({activity_id_at_version}) did not commit."
            );
            let Some(class) = class.as_ref() else {
                // The activation was already committed, so the report carried no outcome. Nothing
                // in §7d.4's table describes that, and it is named rather than mapped to a row it
                // is not.
                eprintln!(
                    "redkiln: the dispatch did not commit and carried no terminal class for \
                     `{item_id}`, so no row of the exit table describes it."
                );
                return Ok(Exit::ConfigError);
            };
            return Ok(Exit::from(class));
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

    Ok(terminal_read(resolved, &recorded))
}

/// Step (10): the terminal read.
///
/// **`run` NEVER WAITS** — no channel, prompt, socket or sleep is entered on any arm (golden
/// constraint 2). A gate among the resumed run's new events is an EXIT, not a wait.
fn terminal_read(resolved: &Resolved, recorded: &rk_kernel::replay::RecordedRun) -> Exit {
    let item_id = resolved.item.id();

    // 🛑 THE GATE ROW THIS READS IS THE RUNTIME'S, AND IT IS THE ONE CARRYING A TOKEN.
    //
    // `select()` announces its abstention gates into `Selection::journal()` and those rows carry
    // no token — §7d.5: the fold *"counts [them] as `gates_announced` and moves no state on"*.
    // `resume` then steps `NextAction::OpenGate(record)` through `Runtime::step`, which appends
    // `runtime::gate_opened_event(record)` — *"the `(kind, token, tree_hash, deadline)` row"*. A
    // read that took the last `gate_opened` of either shape would sometimes trailer an
    // announcement, whose token does not exist.
    let gate = recorded.events.iter().rev().find(|e| {
        e.kind == rk_types::event_kind::EventKind::GateOpened
            && e.payload.get("token").is_some_and(|t| !t.is_null())
    });
    let Some(row) = gate else {
        return Exit::Success;
    };
    let field = |name: &str| {
        row.payload
            .get(name)
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned()
    };
    let token_hex = field("token");
    let tree_hash = field("tree_hash");
    let deadline = row
        .payload
        .get("deadline")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or_default();

    // ---- THE `Gate:` TRAILER COMMIT IS A TIER-1 WRITE (H-5). ------------------------------------
    //
    // ADR-0105 clause 8 / AC-529 / AC-518: it goes through `authorise_tier1_write`, against the
    // RUN-OWNED worktree and its branch, and **never** against `cwd`. `OwnedWorktree`'s path is a
    // private field with only an accessor — the type IS the mechanism: *was this write checked?*
    // is answered by which function produced the value the commit is given.
    let owned_path = resolved
        .common_dir
        .join("redkiln")
        .join("worktrees")
        .join(item_id);
    let ownership = rk_git::worktree::RunOwnership {
        worktree: owned_path.clone(),
        branch: format!("rk-run/{item_id}"),
    };
    let owned =
        match rk_git::worktree::authorise_tier1_write(&resolved.exec, &owned_path, &ownership) {
            Ok(owned) => owned,
            Err(e) => {
                eprintln!("redkiln: the gate trailer's write was not authorised: {e:?}");
                return Exit::ConfigError;
            }
        };
    if let Err(e) = rk_git::trailer::write::commit_with_trailers(
        &resolved.exec,
        owned.path(),
        &format!("Gate opened for {item_id}"),
        &[rk_git::trailer::grammar::Trailer {
            key: rk_git::trailer::grammar::TrailerKey::Gate,
            value: format!("{token_hex} {tree_hash} {deadline}"),
        }],
        true,
    ) {
        eprintln!("redkiln: the gate trailer could not be committed: {e:?}");
        return Exit::ConfigError;
    }

    // **`run` NEVER WAITS** (golden constraint 2). The gate is an EXIT, and exit 0 at that
    // (D-01; AC-240; AC-024): a run that opened a gate did everything it was asked to do.
    println!("gate {token_hex} is open for {item_id}");
    Exit::Success
}

// ------------------------------------------------------------------- step (8), the outer half
//
// Everything below runs ABOVE `run_with` and is what cycle 6 adds: the run-owned worktree, the
// binder pass over every DAG node, and — only then — the real `ClaudeHarness` built from the
// config the bindings filled. §7d.5 fixes that order in terms, and `ClaudeHarness::dispatch`
// refuses a `card_digest` its validated store does not hold, so building the harness before the
// bindings would refuse every dispatch with *"names no card in the validated store"*.

/// The run-owned worktree: `<common-dir>/redkiln/worktrees/<item>` on `rk-run/<item>`.
///
/// **Re-created, never reused** (ADR-0105 clause 6; H-5 / RT-47). The teardown is the RAW
/// `["worktree", "remove", "--force", path]` rather than `rk_git::worktree::remove`, because that
/// wrapper passes no `--force` and git refuses a tree a killed run left files in [measured
/// 2026-09-18, rc=128]. §7d.5 licenses the raw form for exactly this, and for nothing else.
fn recreate_run_worktree(
    exec: &GitExec,
    repo: &WorktreePath,
    common_dir: &Path,
    item: &str,
) -> Result<PathBuf, Exit> {
    let path = common_dir.join("redkiln").join("worktrees").join(item);
    let branch = format!("rk-run/{item}");

    // The teardown's failure is ignored ON PURPOSE: the ordinary case is that there is nothing to
    // remove, and git's refusal to remove an absent tree is not this run's problem.
    let _ = exec.run(
        rk_git::git_exec::GitOp::WorktreeRemove,
        repo,
        [
            std::ffi::OsStr::new("worktree"),
            std::ffi::OsStr::new("remove"),
            std::ffi::OsStr::new("--force"),
            path.as_os_str(),
        ],
    );
    let _ = std::fs::remove_dir_all(&path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            eprintln!("redkiln: {}: {e}", parent.display());
            Exit::ConfigError
        })?;
    }

    // `worktree add -b <branch> <path> HEAD` on a FIRST run; `rk_git::worktree::add` when the
    // branch already exists from an earlier one.
    if exec
        .run(
            rk_git::git_exec::GitOp::WorktreeAdd,
            repo,
            [
                std::ffi::OsStr::new("worktree"),
                std::ffi::OsStr::new("add"),
                std::ffi::OsStr::new("-b"),
                std::ffi::OsStr::new(&branch),
                path.as_os_str(),
                std::ffi::OsStr::new("HEAD"),
            ],
        )
        .is_err()
    {
        rk_git::worktree::add(exec, repo, &path, &branch).map_err(|e| {
            eprintln!("redkiln: the run-owned worktree could not be created: {e:?}");
            Exit::ConfigError
        })?;
    }
    Ok(path)
}

/// The real harness's configuration, from the pack's margins and TTLs and the captured environment.
///
/// # 🛑 `harness_binary` IS THE ONE VALUE HERE WITH A FALLBACK, AND THE FALLBACK IS THE ADAPTER'S
///
/// AC-377's rule — *declared, never defaulted in code* — governs the pack-sourced values: the
/// margins, the two TTLs and the per-activity timeout are read from the pack and a missing one is
/// a refusal, not a default. `harness_binary` is **not** pack-sourced: it is RT-49's OPTIONAL
/// `[adapters]` key, whose absence is the ordinary case, and whose documented default is the
/// adapter's own program name. Spelling that default here is what makes the key optional; it is
/// stated rather than silent so the AC-377 line above is not read as covering it. The adapter
/// declares no constant for it (measured: no `DEFAULT_HARNESS_BINARY` in
/// `rk-adapter-claude/src/config.rs`), and adding one would be an adapter `src` edit that
/// re-opens Checkpoint 7c for a spelling — so the literal is here, named as what it is.
/// **ADR-0139, narrowing ADR-0137 clause 8: the `Toolchain` a run gets when NONE detected.**
///
/// Clause 8 said the run refuses at startup. Measured at `s7g`, that broke **11** `rk-cli` tests
/// whose fixture repositories carry neither a `Cargo.toml` nor a `package.json` and whose subject
/// is the gate/answer path, not the toolchain — a run with no `tool` step was refused for lacking a
/// capability it never reached.
///
/// So the refusal moves to the **point of use**. Every method refuses **by name**, naming the
/// adapters tried and the project root; `detect` answers `Ok(false)`, which is the truth.
///
/// 🛑 **This is not the fixture fallback clause 8 forbids, and the difference is the one that
/// matters.** `FixtureToolchain::test()` returns `Ok(selection.len())` — a SUCCESS, which is how a
/// toolchain that verified nothing has been reporting green. Every method here returns `Err`. A run
/// that never needs a toolchain proceeds; a run that reaches a `tool` step is refused, loudly, at
/// the step that needed it.
#[derive(Debug)]
struct UndetectedToolchain {
    root: String,
    tried: String,
}

impl UndetectedToolchain {
    fn refuse(&self, verb: &str) -> rk_types::error::StepError {
        rk_types::error::StepError::Unsupported {
            capability: format!(
                "{verb}: no configured toolchain adapter detects a project at {} — {}",
                self.root, self.tried
            ),
        }
    }
}

impl rk_ports::toolchain::Toolchain for UndetectedToolchain {
    fn detect(&self) -> Result<bool, rk_types::error::StepError> {
        Ok(false)
    }
    fn build(&self) -> Result<(), rk_types::error::StepError> {
        Err(self.refuse("build"))
    }
    fn test(&self, _selection: &[String]) -> Result<u32, rk_types::error::StepError> {
        Err(self.refuse("test"))
    }
    fn lint(&self) -> Result<(), rk_types::error::StepError> {
        Err(self.refuse("lint"))
    }
    fn classify(
        &self,
        _path: &str,
    ) -> Result<rk_ports::toolchain::Classification, rk_types::error::StepError> {
        Err(self.refuse("classify"))
    }
    fn change_summary(
        &self,
        _range: &str,
    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {
        Err(self.refuse("change_summary"))
    }
}

/// **ADR-0137: build the real `Toolchain`, chosen by the port's own `detect()`.**
///
/// `[adapters] toolchains` in **declared order**, first success wins. Position would not do — the
/// shipped order lists Node first and this tree is Rust — and `detect` is documented as *"Is this
/// toolchain present for the project under test?"*, which is exactly the question. Before `s7g` it
/// had no caller at all.
///
/// 🛑 **`project_root` is the RUN-OWNED WORKTREE, never the checkout a checkpoint is running in.**
/// `s7c` §7c.14: a Rust adapter over `D:/repos/redkiln-rs` would re-enter `cargo test --workspace`
/// from inside a checkpoint's own run, unbounded, on the same `target/`. The run-owned worktree is a
/// separate checkout with its own target dir, and it is the tree the run is actually operating on.
///
/// 🛑 **No fixture fallback.** When nothing detects the run gets an [`UndetectedToolchain`], every
/// one of whose methods refuses by name — **ADR-0139**, narrowing clause 8's startup refusal to the
/// point of use. Falling back to `FixtureToolchain`, whose `test()` returns `Ok`, would restore the
/// pre-`s7g` state under a new name on the path where a real failure is least visible.
///
/// # Errors
/// [`Exit::ConfigError`] when an adapter's own construction is refused — a denylisted or incomplete
/// child environment, a relative root, a zero bound. Those are configuration defects and are not
/// deferred.
fn real_toolchain(
    resolved: &Resolved,
    env: &Env,
    project_root: &Path,
) -> Result<Arc<dyn rk_ports::toolchain::Toolchain>, Exit> {
    let root = project_root;
    let scratch = resolved.common_dir.join("redkiln").join("tmp");
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        eprintln!("redkiln: the toolchain scratch {} : {e}", scratch.display());
        return Err(Exit::ConfigError);
    }
    let timeout = std::time::Duration::from_secs(resolved.toolchain_timeout_secs);
    let child_env = env.as_map().clone();
    let program_for = |name: &str, default: &str| -> PathBuf {
        resolved
            .toolchain_programs
            .as_ref()
            .and_then(|m| m.get(name))
            .map_or_else(|| PathBuf::from(default), PathBuf::from)
    };

    let mut tried: Vec<String> = Vec::new();
    for name in &resolved.toolchains {
        // `adapters::AdapterSet::resolve` already refused any name outside the shipped two, so an
        // unknown name here is a build defect rather than a configuration one — and it is reported
        // as one instead of being skipped into a "nothing detected" refusal that would misname it.
        let candidate: Arc<dyn rk_ports::toolchain::Toolchain> = match name.as_str() {
            "rk-adapter-toolchain-rust" => {
                let cfg = rk_adapter_toolchain_rust::Config::new(
                    program_for(name, rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
                    timeout,
                    root,
                    child_env.clone(),
                    scratch.clone(),
                )
                .map_err(|e| {
                    eprintln!("redkiln: the {name} configuration was refused: {e}");
                    Exit::ConfigError
                })?;
                Arc::new(
                    rk_adapter_toolchain_rust::RustToolchain::new(cfg).map_err(|e| {
                        eprintln!("redkiln: {name} could not be built: {e}");
                        Exit::ConfigError
                    })?,
                )
            }
            "rk-adapter-toolchain-node" => {
                let cfg = rk_adapter_toolchain_node::Config::new(
                    program_for(name, rk_adapter_toolchain_node::DEFAULT_PROGRAM),
                    timeout,
                    root,
                    child_env.clone(),
                    scratch.clone(),
                )
                .map_err(|e| {
                    eprintln!("redkiln: the {name} configuration was refused: {e}");
                    Exit::ConfigError
                })?;
                Arc::new(
                    rk_adapter_toolchain_node::NodeToolchain::new(cfg).map_err(|e| {
                        eprintln!("redkiln: {name} could not be built: {e}");
                        Exit::ConfigError
                    })?,
                )
            }
            other => {
                eprintln!(
                    "redkiln: `{other}` is a resolved toolchain adapter this binary cannot                      construct — `AdapterSet::resolve` and this match have drifted apart"
                );
                return Err(Exit::ConfigError);
            }
        };
        match candidate.detect() {
            Ok(true) => return Ok(candidate),
            Ok(false) => tried.push(format!("{name}: not present")),
            Err(e) => tried.push(format!("{name}: {e:?}")),
        }
    }
    // **ADR-0139.** Not a startup refusal and not a fixture: a toolchain that refuses every verb
    // by name, so a run needing none proceeds and a run reaching a `tool` step is refused there.
    Ok(Arc::new(UndetectedToolchain {
        root: root.display().to_string(),
        tried: if tried.is_empty() {
            "`[adapters] toolchains` is empty".to_owned()
        } else {
            tried.join("; ")
        },
    }))
}

fn real_harness_config(
    resolved: &Resolved,
    env: &Env,
) -> Result<rk_adapter_claude::config::Config, Exit> {
    let binary = resolved
        .harness_binary
        .clone()
        .unwrap_or_else(|| "claude".to_owned());
    rk_adapter_claude::config::Config::new(rk_adapter_claude::config::ConfigValues {
        captured_env: env.as_map().clone(),
        harness_binary: PathBuf::from(binary),
        prompt_cache_ttl: Some(resolved.extras.cache.prompt_ttl.clone()),
        subagent_prompt_cache_ttl: Some(resolved.extras.cache.subagent_prompt_ttl.clone()),
        margins: rk_adapter_claude::config::Margins {
            five_hour: resolved.extras.margins.five_hour,
            seven_day: resolved.extras.margins.seven_day,
        },
        on_version_mismatch: rk_adapter_claude::config::MismatchPolicy::Refuse,
    })
    .map_err(|e| {
        eprintln!("redkiln: the harness configuration was refused: {e}");
        Exit::ConfigError
    })
}

/// Step (8)'s binding pass: one `Binder::bind` per DAG node, each recorded into the input set.
///
/// # 🛑 ONE SPEC PER NODE, NOT PER STEP — THE RUNTIME DISPATCHES WHOLE ACTIVITIES
///
/// `Runtime::run_activity` takes one `ActivitySpec` for an `(item, activity_id@version)`
/// activation, so a DAG node gets exactly one binding whatever its steps' kinds are. The role is
/// whichever of the activity's steps nominates one — and `pack-min`'s single step is a `tool` step
/// that carries `role = "implementer"` deliberately, so `Pack::require_family_for_role` has
/// something to check. An earlier draft of this loop carried a comment claiming an absent role
/// meant *"a node with no agent step"*; that was false of the one fixture it runs against, and the
/// filter it implied would have left `pack-min`'s only node unbound.
///
/// # The selection is resolved twice, and that is structural rather than sloppy
///
/// `replay::resume` re-runs `inputs.build()` and `select()` itself, and it must: it is handed
/// `inputs`, not a selection. The CLI needs the DAG here to bind, so the two calls both stand.
/// What is NOT left to drift is the REFUSAL: a selection that will not resolve renders the same
/// way here as it does one call later, so a test keyed on that message is not keyed on which of
/// the two fired.
fn bind_nodes(
    resolved: &Resolved,
    mut inputs: rk_kernel::replay::RunInputs,
    binder: &mut rk_adapter_claude::binder::Binder,
    owned: &Path,
) -> Result<rk_kernel::replay::RunInputs, Exit> {
    let root = Path::new(resolved.worktree.as_str()).to_path_buf();
    let (pack, catalog, artifacts, facets, item) = inputs.build().map_err(|e| {
        eprintln!("redkiln: {e}");
        Exit::ConfigError
    })?;
    let selection =
        rk_kernel::select::select(&pack, &catalog, &artifacts, &facets, &resolved.stage, &item)
            .map_err(|e| {
                eprintln!("redkiln: {e}");
                Exit::ConfigError
            })?;

    for node in selection.dag().nodes() {
        let Some(activity) = catalog
            .activities()
            .iter()
            .find(|a| a.id == node.activity_id)
        else {
            continue;
        };
        let Some(role_name) = activity.steps.iter().find_map(|s| s.role.clone()) else {
            continue;
        };

        let card_path = root
            .join(".redkiln")
            .join("cards")
            .join(format!("{role_name}.json"));
        let card_text = std::fs::read_to_string(&card_path).map_err(|e| {
            eprintln!("redkiln: {}: {e}", card_path.display());
            Exit::ConfigError
        })?;
        let card = rk_adapter_claude::card::AuthoredCard::from_file(&role_name, &card_text)
            .map_err(|e| {
                eprintln!("redkiln: {}: {e}", card_path.display());
                Exit::ConfigError
            })?;

        let kind = activity.provides.first().cloned().unwrap_or_default();
        let schema_path = root
            .join(".redkiln")
            .join("schemas")
            .join(format!("{kind}.json"));
        let schema_text = std::fs::read_to_string(&schema_path).map_err(|e| {
            eprintln!("redkiln: {}: {e}", schema_path.display());
            Exit::ConfigError
        })?;
        let schema_json: serde_json::Value = serde_json::from_str(&schema_text).map_err(|e| {
            eprintln!("redkiln: {} is not JSON: {e}", schema_path.display());
            Exit::ConfigError
        })?;

        let timeout_secs = resolved
            .extras
            .activity
            .get(&activity.id)
            .map(|a| a.timeout_secs)
            .ok_or_else(|| {
                eprintln!(
                    "redkiln: the pack declares no `[activity.{}] timeout_secs`. AC-377's rule \
                     holds for every adapter-facing value: declared, never defaulted in code.",
                    activity.id
                );
                Exit::ConfigError
            })?;

        // `Role { model, effort }` is THE PACK'S role table, read through `PackExtras`. The card
        // may not carry a tier (`ConfigError::TierLeak`), so this row is its only source.
        let declared = resolved.extras.role.get(&role_name).ok_or_else(|| {
            eprintln!(
                "redkiln: the pack declares no `[role.{role_name}]` row, so the tier the binder \
                 injects has no source (AC-212; AC-118)."
            );
            Exit::ConfigError
        })?;
        let role = rk_adapter_claude::binder::Role {
            model: declared.model,
            effort: declared.effort,
        };

        let spec = binder
            .bind(rk_adapter_claude::binder::BindRequest {
                role_name: &role_name,
                role: &role,
                card: &card,
                schema: &rk_adapter_claude::binder::Schema {
                    id: rk_types::typed::SchemaId::new(&kind),
                    json: schema_json,
                },
                timeout_secs,
                worktree: owned,
                inputs: vec![("item".to_owned(), serde_json::json!(resolved.item.id()))],
            })
            .map_err(|e| {
                eprintln!("redkiln: the binder refused `{}`: {e}", node.id);
                Exit::ConfigError
            })?;

        let activation = activity
            .id_at_version()
            .unwrap_or_else(|| activity.id.clone());
        // **ADR-0138: the step list is TRANSCRIBED here, by the one place that holds the activity.**
        // This loop already read `activity.steps` (for the role, just above) and discarded the
        // rest; the runtime cannot re-derive them — `ActivitySpec` has no step list and
        // `runtime.rs`'s residual 7 forbids loading the catalog from inside the runtime. The
        // composition root is the only site with the value in hand, so it is the site that hands
        // it over. `with_steps`, never `supplied`: the production path always transcribes.
        inputs = inputs.with_spec(
            &node.id,
            rk_kernel::replay::RecordedSpec::with_steps(&activation, spec, activity.steps.clone()),
        );
    }
    Ok(inputs)
}
