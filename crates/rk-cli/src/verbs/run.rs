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
}

/// The outer verb.
///
/// # Errors
/// [`Halt::PoisonedLock`] from the store acquisition. Every other refusal is a value.
pub fn run(cli: &Cli, args: &RunArgs, env: &Env, startup: Startup<'_>) -> Result<Exit, Halt> {
    match through_step_seven(cli, args, env, startup) {
        Err(exit) => Ok(exit),
        Ok(resolved) => {
            let opened =
                match rk_adapter_sqlite::SqliteStore::open(&resolved.exec, &resolved.worktree) {
                    Ok(store) => store,
                    Err(e) => {
                        eprintln!("redkiln: {e}");
                        return Ok(Exit::ConfigError);
                    }
                };
            let store: Arc<Mutex<dyn EventStore>> = Arc::new(Mutex::new(opened));
            let token = CancelToken::new();
            run_with(&resolved, &store, &token)
        }
    }
}

/// The inner verb — the in-process twin §7d.10's two tests drive.
///
/// **The store ARRIVES pre-built**, which is the whole reason the seam is shaped this way: §7d.10's
/// `adr0118_poisoned_store_lock_halts_run_exit_70` hands *"the verb function … the same poisoned
/// `Arc`"*, and a verb that opened its own store could not be given one.
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
    store: &Arc<Mutex<dyn EventStore>>,
    token: &CancelToken,
) -> Result<Exit, Halt> {
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

    // Steps (8)-(10) are the next cycle's. This arm is replaced there; it is NOT a permanent stub,
    // and the slice does not land while it stands.
    eprintln!("redkiln: `run` reaches step (8), which is not built at this cycle");
    Ok(Exit::ConfigError)
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
    let _adapters = startup.adapters.resolve(&config, env).map_err(|e| {
        eprintln!("redkiln: {e}");
        Exit::ConfigError
    })?;

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
