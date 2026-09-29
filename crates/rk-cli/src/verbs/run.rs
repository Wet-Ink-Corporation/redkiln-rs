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
//! No `rk-run/*` ref and no ~~`<common-dir>/redkiln/worktrees/`~~ `.redkiln/worktrees/` directory
//! under the main worktree *(dated 2026-09-27, `s8d-ii` build, P6: the run-tree root moved, D21-44;
//! ADR-0180 decision 1, proposed)*. Both are step (8)'s acts, so
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
    /// *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8; D21-41 e; the slice record,
    /// proposed.)* **The slice this item's review outcome is sealed under** — the `<slice>` of
    /// `Slice-Verdict: <project>/<slice> <verdict>`. Declared, never derived (ADR-0159's pattern;
    /// ADR-0160's route), and **optional here** so the refusal names the item, as `stage` is: an
    /// absent slice refuses the seal at the mint (`tier_one::seal_act`), not at startup. A fix-lane
    /// item declares exactly one (D21-41 e: *"one per fix-lane item"*).
    ///
    /// *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94, D21-95.)* ~~not at
    /// startup~~: for a lane that will reach a stage whose catalog activity provides `Findings`
    /// first, `run` refuses it at the lane's ENTRY, before any dispatch, through the same
    /// validator (`tier_one::validate_slice`); the mint's refusal stays as the backstop.
    #[serde(default)]
    pub slice: Option<String>,
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
    /// The stage the item record carries (RT-50) — or, for the second stage of a lane onward, the
    /// stage the lane loop advanced INTO (ADR-0152).
    ///
    /// *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.5; ADR-0183 decision 5,
    /// proposed.)* **Seeded from git first**: the item's latest stage-bearing `Story:` mark as
    /// `reconcile` folds it ([`seeded_stage`]), and the record's `stage` only when git carries no
    /// stage-bearing mark for the item (the legacy stage-less mark included).
    pub stage: String,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.6; D21-43 N9.)* **The item's
    /// latest mark is `Story: <project>/<item> closed`**: the lane ended, read from git. `run`
    /// reports it closed, exits 0 and mints nothing.
    pub lane_closed: bool,
    /// **`[project] name` — the `<project>` component of every trailer value (ADR-0159).**
    ///
    /// `None` when the key is absent, empty or whitespace-only — all three are the same absence,
    /// because `split_qualified` cannot tell `Story: /slug` from a missing project. A run that mints
    /// no trailer is unaffected; a run that would mint one refuses by name.
    pub project: Option<String>,
    /// *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8.)* The item record's declared
    /// `slice`, read verbatim; `tier_one::seal_act` refuses it by name at the review pass's seal.
    /// *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; D21-94.)* And `run`'s entry
    /// check refuses it first, before any dispatch, when the lane will reach a `Findings` stage.
    pub slice: Option<String>,
    /// **The stage this one was entered FROM, for `RuntimeContext::entered_from` (ADR-0152).**
    ///
    /// `None` for a lane's first stage, which has no predecessor — the one case `stage_entry` says
    /// it cannot state. Set by the lane loop when it re-enters for a successor, and never read from
    /// the item record: the record holds one stage and says nothing about how it was reached.
    pub entered_from: Option<String>,
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
    /// *(2026-09-26, `s8d` build, package P7; R6 = ADR-0178, proposed.)* The facets a wired deriver
    /// computes (`change`, by `R-change-extension-and-numstat-marker`): the axes whose abstention
    /// opens the lane's first-stage gate and whose answer the run binds (§8d.7; D21-43 N3).
    pub derived_facets: Vec<String>,
}

/// **ADR-0140, superseding ADR-0137 clause 5 in part.** The default for the construction bound on
/// every child a toolchain adapter spawns.
///
/// *(Corrected 2026-09-26, s8c §8c.6, under record B = ADR-0171 decisions 4 and 7, proposed, which
/// supersedes ADR-0140 decision 2's sentence in part for `tool` steps.)* This read *"the **only**
/// bound"*, and for a `tool` step that is no longer so: `catalog::Step` carries `timeout_secs`, the
/// kernel resolves it (a literal, or a `${toolchain.<key>}` through the value table), and the
/// adapter enforces `min(step, construction)` on every child, with a `Timeout` naming the bound that
/// fired. This value is the construction bound, the cap behind a step's declared bound; a step that
/// declares none runs under it alone. The paragraph below is left as written at `s7g`.
///
/// Measured at `s7g` over `.redkiln/catalog/`: the largest declared literal `timeout_secs` on any
/// step is **3600**, and the floor is measured rather than chosen.
///
/// 🛑 **The REASON changed on 2026-09-22 even though the number did not.** This read *"the backstop
/// may not be smaller than the largest step it could serve, or a declared step timeout would be
/// capped without saying so"* — which presupposes a per-step bound that something applies. Nothing
/// does: `catalog::Step` has no timeout field and `catalog.rs`'s loader says in terms that it reads
/// none, so all 298 of the pack's declarations (204 literal, 94 unresolved `${toolchain.*}`
/// substitutions) reach no executor. The floor is therefore: **this bound must not cut short a
/// child doing work the pack declares it expects to take that long**, and 3600 is the best
/// available proxy for that expectation. Nothing sits underneath it.
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

    // ---- A CLOSED ITEM IS DONE, READ FROM GIT (`s8e` §8e.6; D21-43 N9; ADR-0183 decision 2,
    // proposed). *(dated 2026-09-28, `s8e` build, package S8E-P6.)* Its latest mark is
    // `Story: <project>/<item> closed`: `run` reports that, exits 0 and mints nothing — no worktree
    // entered (a removed tree is not re-created), no commit, no dispatch, no tier-1 row. That exit 0
    // is Done, not ADR-0157's stall. The invocation reconciled, so it still owes AC-172's single
    // `reconcile_started` row, written the way every other post-reconcile exit writes it.
    if resolved.lane_closed {
        println!(
            "{} is closed: its lane ended at `Story: {}/{} closed`; nothing to run",
            resolved.item.id(),
            resolved.project.as_deref().unwrap_or_default(),
            resolved.item.id()
        );
        return Ok(refuse_after_reconcile(
            &resolved.exec,
            &resolved.worktree,
            &resolved.config,
            Exit::Success,
        ));
    }

    // ---- THE SLICE, REFUSED AT THE LANE'S ENTRY (D21-94, D21-95; ADR-0186 as amended, proposed).
    // *(dated 2026-09-28, `s8e` build, package S8E-D2194-ENTRY.)* A lane that will reach a stage
    // whose catalog activity provides `Findings` first mints a `Slice-Verdict:` there, so an
    // undeclared slice is a certain failure and must not be paid for: it is refused HERE, before
    // any tree, commit or dispatch, through the validator the mint calls too. A lane that reaches
    // no such stage is not refused for a key it never needs; the mint's refusal stays the backstop.
    if let Some(reached) = findings_stage_reached(&resolved)
        && let Err(e) = rk_kernel::runtime::tier_one::validate_slice(
            resolved.slice.as_deref(),
            resolved.item.id(),
            resolved.project.as_deref(),
            &reached.stage_order,
        )
    {
        eprintln!(
            "redkiln: the lane of `{}` will reach `{}`, whose activity `{}` provides `{}` and is \
             sealed under the item's declared slice, so the slice is checked at entry, before any \
             dispatch, and refused: {e} (D21-94, D21-95)",
            resolved.item.id(),
            reached.stage,
            reached.activity,
            crate::review_pass::FINDINGS_SCHEMA
        );
        return Ok(refuse_after_reconcile(
            &resolved.exec,
            &resolved.worktree,
            &resolved.config,
            Exit::ConfigError,
        ));
    }

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
    //
    // *(dated 2026-09-27, `s8d-ii` build, P6; ADR-0180 decisions 1, 6 and 9, proposed.)* The tree
    // is ENTERED, not re-created: `rk_git::worktree::enter` creates it at
    // `.redkiln/worktrees/<item>` under the main worktree, or re-enters and reuses it, or refuses
    // and keeps it, and takes the run's lock. The lock is released when the lane below ends, on
    // every path it ends by (`release_run_worktree`).
    let owner = match run_lock_owner() {
        Ok(owner) => owner,
        Err(exit) => return Ok(exit),
    };
    let entered = match enter_run_worktree(&resolved, env, &owner) {
        Ok(entered) => entered,
        Err(exit) => return Ok(exit),
    };
    let release_exec = resolved.exec.clone();
    let release_start = PathBuf::from(resolved.worktree.as_str());
    let release_item = resolved.item.id().to_owned();
    // W3's carry, at creation only (§8d-ii.4): a reused tree kept its ignored files. A FRESH tree
    // whose carry or setup failed is rolled back before the run refuses (D21-88 ruling 2), so the
    // next run creates it anew and re-carries.
    if entered.created != rk_git::worktree::Creation::Reused
        && let Err(exit) = carry_at_creation(&resolved, env, &entered)
    {
        roll_back_run_worktree(&release_exec, &release_start, &entered, &owner);
        release_run_worktree(&release_exec, &release_start, &release_item, &owner);
        return Ok(exit);
    }
    let mut closed = false;
    let lane = run_lane(
        resolved,
        env,
        &store,
        &token,
        &entered,
        startup.harness,
        &mut closed,
    );
    release_run_worktree(&release_exec, &release_start, &release_item, &owner);
    // ---- THE REMOVAL TRIGGER (`s8e` §8e.6; D21-43 N9; ADR-0180's rule, ADR-0183 decision 2, both
    // proposed). *(dated 2026-09-28, `s8e` build, package S8E-P6.)* Keyed to the lane's `closed`
    // mark, which this invocation minted, and run only AFTER the lock is released: `s8d-ii`'s rule
    // removes a tree that is clean, landed, marked and UNLOCKED, never with `--force`, and keeps
    // anything else with its named reason.
    if closed {
        remove_closed_tree(&release_exec, &release_start, &release_item);
    }
    lane
}

/// The first stage the lane will reach whose catalog activity provides `Findings` first, for the
/// entry check above (D21-95 item 1).
struct FindingsStage {
    /// The stage the walk reached.
    stage: String,
    /// The activity for that stage whose `provides` first is `Findings`.
    activity: String,
    /// The pack's `stage_order`, which the validator refuses a slice against.
    stage_order: Vec<String>,
}

/// **Will the lane reach a stage that seals a slice?** *(dated 2026-09-28, `s8e` build, package
/// S8E-D2194-ENTRY; D21-95 item 1: "At entry it walks the stage order from the current stage
/// through `Pack::successor`, skipping forbidden stages, and refuses by name … when any stage
/// reached has a catalog activity for that stage whose `provides` first is `Findings`.")*
///
/// The walk starts at the CURRENT stage (seeded from git) and follows `Pack::successor`, which
/// skips every `forbidden` entry; a current stage that is itself `forbidden` is skipped too. A
/// stage is read by its catalog activities' declared `provides`, never by its name: the key is
/// `review_pass::FINDINGS_SCHEMA`, the one the review pass selects its committed activation by, and
/// no stage is compared to a string literal (ADR-0162's forbids). The walk is bounded by the
/// declared order's length.
///
/// `None` also when the pack or the catalog does not load: this check decides nothing it cannot
/// read, the path that loads them refuses by name later, and the mint's refusal is the backstop.
fn findings_stage_reached(resolved: &Resolved) -> Option<FindingsStage> {
    let pack = rk_kernel::select::Pack::from_toml(
        &format!("{}.toml", resolved.pack_name),
        &resolved.pack_src,
    )
    .ok()?;
    let sources = catalog_sources(Path::new(resolved.worktree.as_str())).ok()?;
    let borrowed: Vec<(&str, &str)> = sources
        .iter()
        .map(|(n, s)| (n.as_str(), s.as_str()))
        .collect();
    let catalog = rk_kernel::catalog::Catalog::from_sources(&borrowed).ok()?;
    let bound = pack.stage_order().len() + 1;
    let mut at = Some(resolved.stage.as_str());
    let mut walked = 0_usize;
    while let Some(stage) = at {
        if walked >= bound {
            break;
        }
        walked += 1;
        if pack.rigor(stage) != Some(rk_kernel::rigor::Rigor::Forbidden)
            && let Some(activity) = catalog.activities().iter().find(|a| {
                a.stage == stage
                    && a.provides.first().map(String::as_str)
                        == Some(crate::review_pass::FINDINGS_SCHEMA)
            })
        {
            return Some(FindingsStage {
                stage: stage.to_owned(),
                activity: activity.id.clone(),
                stage_order: pack.stage_order().to_vec(),
            });
        }
        at = pack.successor(stage);
    }
    None
}

/// **The removal trigger** (`s8e` §8e.6): `rk_git::worktree::remove_landed` over the item whose lane
/// this invocation closed. A kept tree is reported with `s8d-ii`'s named reason. A refused removal
/// (`GitError::WorktreeRemoveFailed`, Halt 6's typed error) is reported by name and not retried or
/// forced; the lane's exit stands, because its `closed` mark is already git's truth and the tree is
/// kept for the operator. *(dated 2026-09-28, `s8e` build, package S8E-P6.)*
fn remove_closed_tree(exec: &GitExec, start: &Path, item: &str) {
    match rk_git::worktree::remove_landed(exec, start, item) {
        Ok(rk_git::worktree::Removal::Removed) => {
            println!("{item}'s run tree was removed: clean, landed and marked closed");
        }
        Ok(rk_git::worktree::Removal::Kept { path, reason }) => {
            eprintln!(
                "redkiln: {item}'s lane is closed and its run tree {} is kept: {reason}",
                path.display()
            );
        }
        Err(e) => {
            eprintln!(
                "redkiln: {item}'s lane is closed and its run tree could not be removed; it is kept, \
                 not retried and not forced: {}",
                git_refusal_text(&e)
            );
        }
    }
}

/// **The lane, over the entered run tree** — `run`'s body from step (6)'s row on, split out so the
/// run's lock is released on every path the lane ends by (ADR-0180 decision 6, proposed: the lock
/// *"is taken at every run start and released at the run's end"*). *(dated 2026-09-27, `s8d-ii`
/// build, P6.)*
///
/// # Errors
/// [`Halt::PoisonedLock`] from the store acquisition, as `run`'s.
#[expect(
    clippy::too_many_lines,
    reason = "the lane loop moved here whole from `run` (s8d-ii P6) so the lock is released on \
              every path; splitting it further would move no decision"
)]
fn run_lane(
    resolved: Resolved,
    env: &Env,
    store: &Arc<Mutex<dyn EventStore>>,
    token: &CancelToken,
    entered: &rk_git::worktree::Entered,
    // *(dated 2026-09-28, `s8e` build, package S8E-P2; `s8e` §8e.11.)* `Startup`'s harness seam:
    // the product's `ClaudeHarnessFactory`, or a test's factory over a scripted `FakeHarness`.
    harness_factory: &dyn crate::verbs::HarnessFactory,
    // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.6.)* Set when this invocation
    // minted the lane's terminal `closed` mark, so `run` fires the removal trigger once the lock
    // is released.
    closed: &mut bool,
) -> Result<Exit, Halt> {
    let owned = entered.tree.path.clone();

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
            let mut guard = crate::store::acquire(store, "the run")?;
            let appended = guard.append(event);
            drop(guard);
            if let Err(e) = appended {
                eprintln!("redkiln: the invocation's first row could not be written: {e:?}");
                return Ok(Exit::ConfigError);
            }
        }
        Err(exit) => return Ok(exit),
    }

    // ---- THE `worktree_assigned` ROW, AT EVERY RUN START, AND OWNERSHIP READ BACK FROM IT.
    //
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.4; D21-42 8d; the three-kinds
    // record, proposed.)* Written on a created tree and on a reused one: `reused` says which, and
    // `discarded` is every in-flight path the re-entry threw away (W2's "discarded explicitly and
    // journalled"; the stderr lines `enter_run_worktree` prints stay as the operator's echo). Every
    // tier-1 act of this invocation then resolves its `RunOwnership` from THIS row, read back
    // through the one resolver, and never from the convention above or from RT-47's pair on the
    // `reconcile_started` row (kept as tier-2 context; no reader resolves ownership from it).
    let item_id = resolved.item.id().to_owned();
    let assigned = rk_kernel::runtime::tier_one::worktree_assigned_event(
        &item_id,
        &owned,
        &entered.tree.branch,
        entered.created == rk_git::worktree::Creation::Reused,
        &entered.discarded,
    );
    {
        let mut guard = crate::store::acquire(store, "the run")?;
        let appended = guard.append(assigned);
        drop(guard);
        if let Err(e) = appended {
            eprintln!("redkiln: the `worktree_assigned` row could not be written: {e:?}");
            return Ok(Exit::ConfigError);
        }
    }
    let Some(ownership) = assigned_ownership(store, &item_id)? else {
        eprintln!(
            "redkiln: no `worktree_assigned` row reads back for `{item_id}`, so no tier-1 write \
             can resolve its run-owned worktree (ADR-0105 clause 8); refusing, fail-closed"
        );
        return Ok(Exit::ConfigError);
    };

    // ---- THE HEAL (`s8e` §8e.3): a trailer commit on `rk-run/<item>` whose sha anchors no row —
    // a crash between the boundary's commit and its rows — gets its rows now. Git → store only.
    if let Err(e) =
        rk_kernel::runtime::tier_one::heal(&resolved.exec, &resolved.worktree, store, &item_id)
    {
        eprintln!("redkiln: the tier-1 heal of `{item_id}` failed: {e}");
        return Ok(Exit::ConfigError);
    }

    // THE ONE WALL-CLOCK READ. Both derived values come off it — see `base_inputs`.
    let now = crate::clock::now().unix_seconds();

    // ---- **ADR-0152: THE LANE LOOP LIVES HERE, ABOVE `resume`.** -------------------------------
    //
    // One `resume` per stage, one run record per stage, the successor supplied per stage. `resume`'s
    // `AdvanceStage` arm steps the boundary and BREAKS; this loop is what re-enters for the stage it
    // reported. The alternative — advancing inside `resume` — is not implementable and would move
    // the run-record grain: `run_id` is `format!("{}-{}", stage, &digest[..16])`, so a later stage's
    // events would be filed under a record whose id names the first (ADR-0152).
    //
    // Everything inside the loop is per-stage and is therefore rebuilt per iteration: the selection
    // that `bind_nodes` walks, the cards it binds, the harness built over those cards, and the
    // `RunInputs` whose `stage` is in its own digest. Only the store, the worktree and the cancel
    // token outlive an iteration.
    //
    // 🛑 **THE BOUND IS THE PACK'S DECLARED ORDER, AND IT IS ASSERTED RATHER THAN TRUSTED.** A lane
    // visits each stage at most once, so `stage_order().len()` is an upper bound on iterations. The
    // guard is not decoration: `Pack::successor` returning a stage already visited would otherwise
    // be an unbounded loop appending tier-1 rows, which is the exact failure ADR-0152 exists to
    // prevent and which a mutant of that function can still produce.
    let lane_bound = {
        let pack = rk_kernel::select::Pack::from_toml(
            &format!("{}.toml", resolved.pack_name),
            &resolved.pack_src,
        );
        pack.map_or(1, |p| p.stage_order().len().max(1))
    };
    // ---- THE PACK'S QUESTIONS (R5 = ADR-0177 decision 8, proposed; `s8d` §8d.6). *(2026-09-26,
    // `s8d` build, package P7.)* Loaded ONCE, from the worktree the catalog is read from, by the one
    // loader `answer` uses too. A catalog `gate` step whose question the pack never declares is
    // refused by name in the binding pass below, before any dispatch or gate row.
    let questions = match crate::processes::load(Path::new(resolved.worktree.as_str())) {
        Ok(questions) => questions,
        Err(e) => {
            eprintln!("redkiln: the question registry was refused: {e}");
            return Ok(Exit::ConfigError);
        }
    };
    let mut resolved = resolved;
    let mut visited: Vec<String> = Vec::new();
    let mut last_exit;

    loop {
        visited.push(resolved.stage.clone());
        // ---- R6 decision 5 (`s8d` §8d.7; D21-43 N3). An abstaining derived facet whose
        // `unknown_facet` gate was answered binds the answer, folded from `gate_answered` —
        // BEFORE the inputs, so every stage's selection and record read the bound value.
        if let Err(exit) = bind_answered_facets(&mut resolved, store)? {
            return Ok(exit);
        }
        // *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; defect (b); the landing
        // record ADR-0188 decision 12, proposed.)* ~~over `resolved.worktree`, the checkout~~ —
        // the inputs are hashed over the RUN-OWNED tree, the one the stage's inputs come from and
        // the one `answer` hashes.
        let owned_at = match rk_git::path::WorktreePath::new(&owned) {
            Ok(at) => at,
            Err(e) => {
                eprintln!("redkiln: the run-owned worktree's path is refused: {e:?}");
                return Ok(Exit::ConfigError);
            }
        };
        let (inputs, tree_hash) = match prepare_inputs(&resolved, &owned_at, now) {
            Ok(pair) => pair,
            Err(exit) => return Ok(exit),
        };
        // ---- R6 decision 4: still `null` at the lane's first stage, the gate opens (once), and
        // the FSM's first rule then awaits it (ADR-0022) before any selection reads the facet.
        // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3.)* Each opening is a tier-1
        // act, through the boundary.
        let mut facet_boundary = tier_one_boundary(&resolved, &owned, &ownership);
        if let Err(exit) = open_derived_facet_gates(
            &resolved,
            store,
            &tree_hash,
            now,
            &mut facet_boundary,
            inputs.run_id().ok().as_deref(),
        )? {
            return Ok(exit);
        }

        let claude = match real_harness_config(&resolved, env) {
            Ok(config) => config,
            Err(exit) => return Ok(exit),
        };
        let mut binder = rk_adapter_claude::binder::Binder::new(claude);

        // **ADR-0137: the REAL toolchain, over the run-owned worktree.** Until `s7g` this slot held
        // `FixtureToolchain::default()`, whose `Default` is `present: false` — a toolchain reporting
        // itself absent, which is why AC-473's clause has never had a mechanism.
        //
        // *(2026-09-26, s8c §8c.5; records V and B, proposed.)* The same selection, keeping the
        // chosen adapter's NAME here: it selects the value table, and the kernel is handed the
        // table without it (AC-487). The registry is read from the worktree the catalog is.
        //
        // *(Moved 2026-09-26, `s8d` build, P3; record R1, proposed.)* ABOVE the binding pass, which
        // now resolves each agent step's `${toolchain.*}` bound through the same value table (D21-43
        // N16). Neither call reads what the other produces, so only the order of their refusals
        // moved.
        let chosen = match choose_toolchain(
            &ToolchainCandidates::of(&resolved),
            env,
            &owned,
            &mut |line| eprintln!("redkiln: {line}"),
        ) {
            Ok(chosen) => chosen,
            Err(exit) => return Ok(exit),
        };
        let routing = match tool_routing(
            Path::new(resolved.worktree.as_str()),
            chosen.adapter.as_deref(),
        ) {
            Ok(routing) => routing,
            Err(e) => {
                eprintln!("redkiln: the verb registry was refused: {e}");
                return Ok(Exit::ConfigError);
            }
        };
        // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 4, proposed.)*
        // `sca_scan`'s databases at the tools' default locations, which no shipped step declares.
        let routing = advisory_db_row_inputs(env, chosen.adapter.as_deref())
            .into_iter()
            .fold(routing, |r, (verb, input, value)| {
                r.with_row_input(verb, input, value)
            });
        let toolchain = chosen.toolchain;
        // *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177 decision 2, proposed.)* The review
        // routes: `shallow_clone`'s clone of THIS repository (its common dir, whose object store the
        // run-owned branch writes into) under `<common-dir>/redkiln/clones/<item>/<sha>`, and
        // `verdict_record`'s re-execution arms, each a toolchain chosen by the same selection loop as
        // the run's — the candidate rooted in the clone, the reference in the checkout.
        let routing = routing.with_review_routes(Arc::new(review_routes(&resolved, env)));
        // *(2026-09-27, `s8d` build, package F3-D2174; D21-74 ruling 2; R2 = ADR-0173 decision 9
        // as amended, proposed.)* The run-branch reader `"affected"` reads inside `Changeset`'s own
        // producer: R6's range, `HEAD..rk-run/<item>`, over the same checkout and through the same
        // `rk-git` read the `change` deriver took above (`change_deriver::read_range`).
        let routing = routing.with_run_branch(Arc::new(crate::change_deriver::RunBranchDiff::new(
            resolved.exec.clone(),
            resolved.worktree.clone(),
        )));

        let inputs = match bind_nodes(
            &resolved,
            inputs,
            &mut binder,
            &owned,
            routing.values(),
            &questions,
        ) {
            Ok(inputs) => inputs,
            Err(exit) => return Ok(exit),
        };

        // **ONLY THEN** the harness, over the config the bindings filled (§7d.5's own order):
        // `ClaudeHarness::dispatch` refuses a `card_digest` its validated store does not hold, and
        // `Binder::new(config).bind(..)` then `into_config()` is the one way that store is filled.
        // *(dated 2026-09-28, `s8e` build, package S8E-P2; `s8e` §8e.11.)* Built through
        // `Startup`'s harness seam: `Startup::real()`'s factory builds `ClaudeHarness` exactly as
        // this line did before.
        let harness = match harness_factory.build(binder.into_config(), token.clone()) {
            Ok(harness) => harness,
            Err(e) => {
                eprintln!("redkiln: the harness could not be built: {e}");
                return Ok(Exit::ConfigError);
            }
        };

        let ports = rk_ports::root::Ports {
            // Cloned per iteration: the STORE outlives a stage (one journal per item, across the whole
            // lane), while every other port here is rebuilt for the stage's own bindings (ADR-0152).
            store: Arc::clone(store),
            harness,
            // The two with no Phase-1 real (ADR-0094 decision 2; design:265-266 — GitHub and mdBook
            // are Phase 3, which is ADR-0134's warrant for the port lint's expected-uncalled set).
            tracker: Arc::new(rk_ports::fakes::BoardTracker::default()),
            docs: Arc::new(rk_ports::fakes::RecordingDocs::default()),
            toolchain,
        };

        // Every node the loop reaches was bound above; the supplier is the seam for a node first seen
        // INSIDE the loop, and it refuses by name rather than silently binding nothing (RT-55).
        let mut supply_spec = |node: &str| -> Result<
            rk_kernel::replay::RecordedSpec,
            rk_kernel::replay::ReplayError,
        > {
            Err(rk_kernel::replay::ReplayError::Malformed {
                path: "inputs/specs.json".to_owned(),
                detail: format!(
                    "`{node}` was first seen inside the loop and step (8) bound no spec for it"
                ),
            })
        };

        let mut advanced: Option<String> = None;
        let mut lane_end = false;
        last_exit = run_with_advance(
            &resolved,
            &inputs,
            now,
            &tree_hash,
            ports,
            routing,
            env,
            &mut supply_spec,
            token,
            &mut advanced,
            &mut lane_end,
            // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3.)* The stage's tier-1
            // boundary: the lane entry, the advance and every gate transition commit through it.
            Some(tier_one_boundary(&resolved, &owned, &ownership)),
        )?;

        // 🛑 **A NON-ZERO EXIT ENDS THE LANE, EVEN WHEN THE STAGE ADVANCED.** Found by this stage's
        // own adversarial pass, and it was a real hole rather than a theoretical one: the boundary is
        // reported through `advanced` while the failure is reported through the return value, and the
        // `let Some(next) = advanced else { return }` below reads `last_exit` **only on the
        // no-advance path**. So every `Exit` a stage produced ALONGSIDE an advance was discarded and
        // the loop advanced anyway — including all four of `story_trailer`'s refusals, which means
        // ADR-0159's *"refuses by name"* was unobservable to any caller that reads an exit code: the
        // message reached stderr and the process still exited on the LAST stage's status.
        //
        // Checked here rather than inside `run_with_advance` because the lane is this loop's
        // invariant, not one stage's: a stage that failed has not earned a successor.
        if last_exit != Exit::Success {
            return Ok(last_exit);
        }

        // A stage that took no boundary ends the lane: a gate (exit 0, golden constraint 2), a
        // halt, a refusal, or `Done` at the end of the order.
        let Some(next) = advanced else {
            // ---- THE LANE'S END (`s8e` §8e.6; D21-43 N9; ADR-0183 decision 2, proposed). *(dated
            // 2026-09-28, `s8e` build, package S8E-P6.)* The stage reached `Done` with no
            // successor, and its review pass (if any) did not block: the kernel's `close_act`
            // mints `Story: <project>/<item> closed` through the tier-1 boundary, git first, with
            // the empty content arm at Checkpoint 8e (`s8f`'s landed arm makes it carry closeout's
            // content checkpoint).
            if lane_end {
                return close_lane(&resolved, store, &owned, &ownership, closed);
            }
            return Ok(last_exit);
        };

        // 🛑 A boundary into a stage this lane already visited is a DEFECT, not a lane. It is
        // refused by name rather than followed, because following it is the unbounded append.
        if visited.iter().any(|s| s == &next) {
            eprintln!(
                "redkiln: the lane advanced into `{next}`, which it has already visited                  ({}); the pack's `[policy] stage_order` does not describe a lane.",
                visited.join(" -> ")
            );
            return Ok(Exit::ConfigError);
        }
        if visited.len() >= lane_bound {
            eprintln!(
                "redkiln: the lane visited {} stages and the pack declares {lane_bound};                  stopping rather than advancing again.",
                visited.len()
            );
            return Ok(Exit::ConfigError);
        }

        println!(
            "{} advanced {} -> {next}",
            resolved.item.id(),
            resolved.stage
        );
        resolved = Resolved {
            entered_from: Some(resolved.stage.clone()),
            stage: next,
            ..resolved
        };
    }
}

/// **Mint the lane's terminal `closed` mark** (`s8e` §8e.6): the kernel's
/// `tier_one::close_act` for the stage the lane ended in, through this stage's tier-1 boundary.
/// `closed` is set only when the mark is on the run ref (made now, or already there).
/// *(dated 2026-09-28, `s8e` build, package S8E-P6.)*
///
/// # Errors
/// [`Halt::PoisonedLock`] never arises here (the boundary locks the store itself, and a poisoned
/// lock halts inside it); a refusal is exit `3`, named.
fn close_lane(
    resolved: &Resolved,
    store: &Arc<Mutex<dyn EventStore>>,
    owned: &Path,
    ownership: &rk_git::worktree::RunOwnership,
    closed: &mut bool,
) -> Result<Exit, Halt> {
    let mut boundary = tier_one_boundary(resolved, owned, ownership);
    match boundary.act(
        store,
        rk_kernel::runtime::tier_one::close_act(&resolved.stage),
    ) {
        Ok(_) => {
            *closed = true;
            println!(
                "{} closed: its lane ended after `{}`",
                resolved.item.id(),
                resolved.stage
            );
            Ok(Exit::Success)
        }
        Err(e) => {
            eprintln!(
                "redkiln: the lane of `{}` ended and its `closed` mark was refused: {e}",
                resolved.item.id()
            );
            Ok(Exit::ConfigError)
        }
    }
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
    env: &Env,
    supply_spec: &mut dyn FnMut(
        &str,
    ) -> Result<
        rk_kernel::replay::RecordedSpec,
        rk_kernel::replay::ReplayError,
    >,
    token: &CancelToken,
) -> Result<Exit, Halt> {
    // The in-process seam binds no verb registry: its fixtures (`pack-min`) carry no `tool` step.
    run_with_advance(
        resolved,
        inputs,
        now,
        tree_hash,
        ports,
        rk_kernel::runtime::step::ToolRouting::default(),
        env,
        supply_spec,
        token,
        &mut None,
        // *(dated 2026-09-28, `s8e` build, package S8E-P6.)* The lane's end is reported and
        // dropped: the in-process seam mints no `closed` mark (it is unbound).
        &mut false,
        // *(dated 2026-09-28, `s8e` build, package S8E-P5.)* The in-process seam is UNBOUND: its
        // callers are the machinery suites over a counting store, with no `worktree_assigned` row
        // and no run tree. The verb `run` always binds one (`run_lane`).
        None,
    )
}

/// [`run_with`], reporting the stage boundary it took (ADR-0152).
///
/// `advanced` is set to `Some(to)` exactly when `replay::resume` executed its `AdvanceStage` arm.
/// It is an out-parameter rather than a changed return type because [`run_with`] is the seam every
/// in-process `rk-cli` test drives — ten call sites — and widening the pair they all match on would
/// be a mechanical edit across tests that are asserting something else. The lane loop in [`run`] is
/// the only caller that passes anything but `&mut None`.
///
/// # Errors
/// As [`run_with`].
#[expect(
    clippy::too_many_arguments,
    reason = "as `run_with`, plus ADR-0152's out-parameter; see that function's own note"
)]
pub fn run_with_advance(
    resolved: &Resolved,
    inputs: &rk_kernel::replay::RunInputs,
    now: i64,
    tree_hash: &str,
    ports: rk_ports::root::Ports,
    // *(2026-09-26, s8c §8c.4.)* The verb registry and value table the `tool` arm routes through.
    routing: rk_kernel::runtime::step::ToolRouting,
    // **ADR-0162.** Threaded for the review pass alone, which builds a SECOND `Toolchain` rooted in
    // an ephemeral reference worktree. It is a parameter rather than a second `Env::capture()`
    // because a run has one environment, captured once at entry, and a pass that re-captured it
    // could disagree with the dispatch that produced the findings it is confirming.
    env: &Env,
    supply_spec: &mut dyn FnMut(
        &str,
    ) -> Result<
        rk_kernel::replay::RecordedSpec,
        rk_kernel::replay::ReplayError,
    >,
    token: &CancelToken,
    advanced: &mut Option<String>,
    // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.6.)* Set when the stage reached
    // the lane's end (`RecordedRun::lane_ended`) and its review pass did not block.
    lane_end: &mut bool,
    boundary: Option<rk_kernel::runtime::tier_one::Boundary>,
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

    resume_and_read(
        resolved,
        inputs,
        now,
        tree_hash,
        ports,
        routing,
        env,
        supply_spec,
        token,
        advanced,
        lane_end,
        boundary,
    )
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
    //
    // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.5; D21-43 N7; ADR-0183 decision 5,
    // proposed.)* ~~`let _ = startup`~~ — the `Outcome` is KEPT: the item's stage is seeded from its
    // fold below, which is the first time ADR-0109 decision 1's `Reconciled<State>` reaches the
    // planner rather than being discarded.
    let reconciled = startup
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

    // ADR-0159: an empty or whitespace-only name is the ABSENT case, not a value.
    let project = config
        .project
        .as_ref()
        .map(|p| p.name.trim().to_owned())
        .filter(|n| !n.is_empty());

    // ---- THE SEED (`s8e` §8e.5; D21-41 a; D21-43 N7; ADR-0183 decision 5, proposed). *(dated
    // 2026-09-28, `s8e` build, package S8E-P6.)* The item's reconciled latest stage-bearing mark is
    // the stage; the record's is the fallback, taken only when git carries no stage-bearing mark
    // for the item — which is today's behaviour exactly for the one legacy stage-less mark on the
    // tree (`rk-run/rk-fix-doctor-test-count`, D21-42 8d's default). A `closed` mark is the lane's
    // end, read from git (N9).
    let git_stage = seeded_stage(&reconciled.state, project.as_deref(), &args.item);
    let lane_closed = matches!(
        git_stage,
        Some(rk_git::trailer::grammar::StoryStage::Closed)
    );
    let seeded = match git_stage {
        Some(rk_git::trailer::grammar::StoryStage::Stage(stage)) => Some(stage),
        Some(rk_git::trailer::grammar::StoryStage::Closed) | None => None,
    };

    // RT-50. Refused rather than defaulted, and the refusal NAMES THE ITEM. *(dated 2026-09-28,
    // `s8e` build, package S8E-P6.)* Only when git seeds no stage either.
    let Some(stage) = seeded.or_else(|| record.stage.clone()) else {
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
    // ---- THE `change` DERIVER (R6 = ADR-0178 decisions 1-3, proposed; `s8d` §8d.7). *(2026-09-26,
    // `s8d` build, package P7.)* `R-change-extension-and-numstat-marker` runs here, over the item's
    // changeset (`HEAD..rk-run/<item>`), through `rk-git`, when the registry declares a facet it
    // derives. `None` is the abstention (an empty range); a range that exists and cannot be read is
    // refused by name.
    let mut derived: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for facet in registry.facets() {
        if facet.source != rk_kernel::registry::Source::Derived
            || facet.deriver.as_deref() != Some(crate::change_deriver::DERIVER)
        {
            continue;
        }
        match crate::change_deriver::derive(&repo.exec, &repo.worktree, &args.item) {
            Ok(Some(values)) => {
                derived.insert(facet.name.clone(), values);
            }
            Ok(None) => {}
            Err(e) => {
                eprintln!("redkiln: {e}");
                return Err(refuse_after_reconcile(
                    &repo.exec,
                    &repo.worktree,
                    &kernel_config,
                    Exit::ConfigError,
                ));
            }
        }
    }
    let item = build_item(&record, &registry, &derived).map_err(|refusal| {
        eprintln!("redkiln: {}: {refusal}", record_path.display());
        refuse_after_reconcile(
            &repo.exec,
            &repo.worktree,
            &kernel_config,
            Exit::ConfigError,
        )
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
        lane_closed,
        project,
        slice: record.slice,
        // A lane's FIRST stage has no predecessor; the loop sets this for every stage after it.
        entered_from: None,
        config: kernel_config,
        harness_binary: config.adapters.harness_binary.clone(),
        toolchains: config.adapters.toolchains.clone(),
        toolchain_programs: config.adapters.toolchain_programs.clone(),
        toolchain_timeout_secs: config
            .adapters
            .toolchain_timeout_secs
            .unwrap_or(DEFAULT_TOOLCHAIN_TIMEOUT_SECS),
        derived_facets: registry
            .facets()
            .filter(|f| {
                f.source == rk_kernel::registry::Source::Derived
                    && f.deriver.as_deref() == Some(crate::change_deriver::DERIVER)
            })
            .map(|f| f.name.clone())
            .collect(),
    })
}

/// **The seed**: the item's latest stage-bearing mark, as `reconcile` folded it.
///
/// (`s8e` §8e.5; D21-41 a; D21-43 N7; ADR-0183 decision 5, proposed.) The item's latest stage-bearing (or `closed`) `Story:` mark, as `reconcile` folded it for `project`.
/// `None` when there is no project to look it up under, or git carries no stage-bearing mark for
/// the item — a legacy stage-less mark included, which never sets a stage (ADR-0183 decision 3).
/// The caller then falls back to the item record. *(dated 2026-09-28, `s8e` build, package
/// S8E-P6.)*
#[must_use]
pub fn seeded_stage(
    state: &rk_git::reconciled::Reconciled<rk_git::reconcile::State>,
    project: Option<&str>,
    item: &str,
) -> Option<rk_git::trailer::grammar::StoryStage> {
    state
        .get()
        .projects
        .get(project?)?
        .stages
        .get(item)
        .map(|mark| mark.stage.clone())
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

/// **The one resolver of run ownership** (`s8e` §8e.4; D21-42 8d; AC-529; the three-kinds record,
/// proposed): the latest `worktree_assigned` row the store holds FOR `item`, read through
/// `ByItem`. `Ok(None)` when there is none, and every caller then refuses, fail-closed. Nothing is
/// resolved from `cwd`, from the `.redkiln/worktrees/<item>` convention, or from RT-47's pair on
/// `reconcile_started`, which stays tier-2 context that no reader resolves ownership from.
/// *(dated 2026-09-28, `s8e` build, package S8E-P5.)*
///
/// # Errors
/// [`Halt::PoisonedLock`] from the store acquisition.
pub fn assigned_ownership(
    store: &Arc<Mutex<dyn EventStore>>,
    item: &str,
) -> Result<Option<rk_git::worktree::RunOwnership>, Halt> {
    let guard = crate::store::acquire(store, "the ownership read")?;
    let rows = guard.query(rk_kernel::runtime::journal_query(item));
    drop(guard);
    Ok(rows
        .ok()
        .and_then(|rows| rk_kernel::runtime::tier_one::assigned_ownership(&rows, item)))
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

/// The item's journal and its fold at the resolved stage, for R6's two passes.
fn folded_state(
    resolved: &Resolved,
    store: &Arc<Mutex<dyn EventStore>>,
) -> Result<Result<rk_kernel::workflow::fsm::RunState, Exit>, Halt> {
    let guard = crate::store::acquire(store, "the run")?;
    let read = guard.query(rk_kernel::runtime::journal_query(resolved.item.id()));
    drop(guard);
    let events = match read {
        Ok(events) => events,
        Err(e) => {
            eprintln!("redkiln: the recorded journal could not be read: {e:?}");
            return Ok(Err(Exit::ConfigError));
        }
    };
    Ok(
        rk_kernel::workflow::fsm::fold(resolved.item.id(), &resolved.stage, &events).map_err(|e| {
            eprintln!("redkiln: the recorded journal did not fold: {e}");
            Exit::ConfigError
        }),
    )
}

/// **R6 decision 5 (ADR-0178, proposed): an answered `unknown_facet:<axis>` gate binds the axis.**
/// *(Added 2026-09-26, `s8d` build, package P7.)*
///
/// For each wired derived facet still `null` on the item (its deriver abstained), the latest
/// `unknown_facet:<axis>` gate answered and not void binds the facet as a ONE-ELEMENT set — folded
/// from the `gate_answered` row, never written to the item record, where a `change` value stays
/// refused. A facet the deriver DID compute keeps the derived value: the derived facet's source is
/// its deriver (ADR-0093 decision 1), and a derived value that disagrees with an earlier answer is
/// REPORTED on stderr, naming both, and binds (R6 decision 6).
fn bind_answered_facets(
    resolved: &mut Resolved,
    store: &Arc<Mutex<dyn EventStore>>,
) -> Result<Result<(), Exit>, Halt> {
    if resolved.derived_facets.is_empty() {
        return Ok(Ok(()));
    }
    let state = match folded_state(resolved, store)? {
        Ok(state) => state,
        Err(exit) => return Ok(Err(exit)),
    };
    for axis in resolved.derived_facets.clone() {
        let Some(answer) = rk_kernel::workflow::gate::answered_facet(&state, &axis) else {
            continue;
        };
        match resolved.item.facet(&axis) {
            Some(rk_kernel::select::FacetValue::Null) => {
                resolved.item = resolved
                    .item
                    .clone()
                    .with_facet_set(&axis, [answer.clone()]);
            }
            Some(rk_kernel::select::FacetValue::Set(derived))
                if derived.len() != 1 || !derived.contains(&answer) =>
            {
                eprintln!(
                    "redkiln: `{axis}` derived as {:?} over the item's changeset disagrees with the \
                     triage answer `{answer}`; the derived value binds, because a derived facet's \
                     source is its deriver (R6 = ADR-0178 decision 6)",
                    derived.iter().collect::<Vec<_>>()
                );
            }
            _ => {}
        }
    }
    Ok(Ok(()))
}

/// **R6 decision 4 (ADR-0178, proposed): a wired derived facet still `null` at the lane's first
/// stage opens AC-079's `unknown_facet` gate, once.** *(Added 2026-09-26, `s8d` build, package P7.)*
///
/// D21-43 N3: *"`null` routes to AC-079's `unknown_facet` gate at triage, and the operator supplies
/// it."* The first stage is the pack's declared `stage_order`'s first (`triage` in both shipped
/// packs); a pack that declares no order has no first stage and opens nothing here. The row is
/// decided by the kernel ([`rk_kernel::workflow::gate::derived_facet_gate`]: no reach of the kind,
/// or a void latest one) and appended here, before `resume` folds the journal, so the FSM's first
/// rule awaits it.
fn open_derived_facet_gates(
    resolved: &Resolved,
    store: &Arc<Mutex<dyn EventStore>>,
    tree_hash: &str,
    now: i64,
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3, N6.)* Each opening is a tier-1
    // act through this boundary, carrying the opening stage run.
    boundary: &mut rk_kernel::runtime::tier_one::Boundary,
    run_id: Option<&str>,
) -> Result<Result<(), Exit>, Halt> {
    let abstaining: Vec<&String> = resolved
        .derived_facets
        .iter()
        .filter(|axis| {
            matches!(
                resolved.item.facet(axis),
                Some(rk_kernel::select::FacetValue::Null)
            )
        })
        .collect();
    if abstaining.is_empty() {
        return Ok(Ok(()));
    }
    let Ok(pack) = rk_kernel::select::Pack::from_toml(
        &format!("{}.toml", resolved.pack_name),
        &resolved.pack_src,
    ) else {
        // The pack is parsed, and refused by name, by the input set a moment earlier.
        return Ok(Ok(()));
    };
    if pack.stage_order().first() != Some(&resolved.stage) {
        return Ok(Ok(()));
    }
    let state = match folded_state(resolved, store)? {
        Ok(state) => state,
        Err(exit) => return Ok(Err(exit)),
    };
    let deadline = rk_types::time::Timestamp::from_unix_seconds(
        now.saturating_add(resolved.extras.gate_deadline_secs),
    );
    let mut rows = Vec::new();
    for axis in abstaining {
        match rk_kernel::workflow::gate::derived_facet_gate(&state, axis, tree_hash, deadline) {
            Ok(Some(row)) => rows.push(row),
            Ok(None) => {}
            Err(e) => {
                eprintln!("redkiln: the `unknown_facet:{axis}` gate could not be minted: {e}");
                return Ok(Err(Exit::ConfigError));
            }
        }
    }
    if rows.is_empty() {
        return Ok(Ok(()));
    }
    gate_rows_through(
        store,
        Some(boundary),
        &state,
        rows,
        run_id.map(|run| (resolved.stage.as_str(), run)),
    )
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
///
/// *(2026-09-26, `s8d` build, package P7; R6 = ADR-0178 decisions 1 and 2, proposed.)* Two changes.
/// **A record-supplied value for a `source = "derived"` facet is refused by name** — a derived facet
/// is *"computed over the changeset by a named `deriver`"* (`facets.toml`), so a `[facets]` entry
/// for one would be a human value on a facet no human sources (ADR-0093 decision 1's classes, as
/// R6 reads them). And **`derived` carries each value a wired deriver computed**, bound as a set;
/// a derived facet with no value there (its deriver abstained, or none is wired) stays
/// `with_facet_null`, which is the abstention, never `[]` and never a default.
///
/// # Errors
/// A `String` naming the derived facet the record supplied, and its deriver.
fn build_item(
    record: &ItemRecord,
    registry: &rk_kernel::registry::Registry,
    derived: &BTreeMap<String, Vec<String>>,
) -> Result<Item, String> {
    for facet in registry.facets() {
        if facet.source == rk_kernel::registry::Source::Derived
            && record.facets.contains_key(&facet.name)
        {
            return Err(format!(
                "the item record supplies `{}`, which `facets.toml` declares `source = \"derived\"` \
                 (deriver `{}`): a derived facet is computed over the changeset and never read from \
                 the record, so the value is refused rather than taken as a human one (R6 = \
                 ADR-0178; ADR-0093 decision 1)",
                facet.name,
                facet.deriver.as_deref().unwrap_or("none declared"),
            ));
        }
    }
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
        item = match derived.get(&facet.name) {
            Some(values) => item.with_facet_set(&facet.name, values.iter().cloned()),
            None => item.with_facet_null(&facet.name),
        };
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
    Ok(item)
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
/// *(dated 2026-09-28, s8f build, package S8F-P2-LANDING-CORE; defect (b); the landing record
/// ADR-0188 decision 12, proposed.)* `tree` is the tree the hash is taken over: the lane passes
/// the **run-owned** worktree, since that is the tree the stage's inputs come from and the one
/// `answer` hashes; ~~`resolved.worktree`, the checkout~~. `run_id` is derived over the inputs, so
/// its values move for new runs and its grain does not (ADR-0152). The in-process harness, which
/// has no run-owned tree, passes its checkout.
///
/// # Errors
/// An [`Exit`] row when a registry will not read, or the tree hash cannot be taken.
pub fn prepare_inputs(
    resolved: &Resolved,
    tree: &rk_git::path::WorktreePath,
    now: i64,
) -> Result<(rk_kernel::replay::RunInputs, String), Exit> {
    let tree_hash = rk_git::tree::tree_hash(
        &resolved.exec,
        tree,
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

    // **ADR-0152 / ADR-0153: THE SUCCESSOR IS SUPPLIED HERE, AND THIS IS THE ONLY PLACE.**
    //
    // `RuntimeContext::advances_to`'s contract is *"supplied, never invented"*, and the kernel
    // keeps that: `fsm::next_action`'s advance arm reads `ctx.successor` and consults no pack. So
    // the composition root resolves the lane order and hands over the answer.
    //
    // Which stage it is comes from the pack's declared `[policy] stage_order`, skipping every
    // `forbidden` entry — `Pack::successor`. `None` is the end of the lane and is NOT an error: the
    // FSM then reports `Done`, exactly as it did before 7h.
    //
    // 🛑 The pack is parsed here rather than threaded from step (8) because `Resolved` carries the
    // pack's BYTES (*"both parsers read these"*) and not a loaded `Pack`; a second parse of the same
    // bytes cannot disagree with the first, and `bind_nodes`'s own parse is scoped to selection.
    let pack = rk_kernel::select::Pack::from_toml(
        &format!("{}.toml", resolved.pack_name),
        &resolved.pack_src,
    )
    .map_err(|e| {
        eprintln!("redkiln: {}.toml: {e}", resolved.pack_name);
        Exit::ConfigError
    })?;
    let mut runtime =
        rk_kernel::replay::RuntimeContext::supplied(tree_hash, deadline).results_at(now);
    if let Some(to) = pack.successor(&resolved.stage) {
        runtime = runtime.advances_to(to);
    }
    if let Some(from) = &resolved.entered_from {
        runtime = runtime.entered_from(from);
    }

    Ok(rk_kernel::replay::RunInputs::new(&resolved.stage)
        .with_pack(&format!("{}.toml", resolved.pack_name), &resolved.pack_src)
        .with_catalog(&borrowed)
        .with_artifacts("artifacts.toml", &artifacts_src)
        .with_facets(&facets_src)
        .with_item(&resolved.item)
        // ONE `with_config`, over the value step (4) constructed — never a second `Config::new`.
        .with_config(resolved.config.clone())
        .with_runtime(runtime))
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
    routing: rk_kernel::runtime::step::ToolRouting,
    env: &Env,
    supply_spec: &mut dyn FnMut(
        &str,
    ) -> Result<
        rk_kernel::replay::RecordedSpec,
        rk_kernel::replay::ReplayError,
    >,
    token: &CancelToken,
    advanced: &mut Option<String>,
    lane_end: &mut bool,
    mut boundary: Option<rk_kernel::runtime::tier_one::Boundary>,
) -> Result<Exit, Halt> {
    let item_id = resolved.item.id().to_owned();
    let store = Arc::clone(&ports.store);
    // Cloned before `ports` is moved into `resume`: the review pass's CANDIDATE arm is this same
    // toolchain, rooted in the run-owned worktree (ADR-0162 decision 4).
    let candidate_toolchain = Arc::clone(&ports.toolchain);

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
    //
    // *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3, N6.)* ~~store-only~~: bound,
    // each expiry row is a tier-1 act — its `Gate: … expired <action>` mark first, the row anchored
    // to it — and an escalation's `gate_opened` is its own act. Unbound (`run_with`'s machinery
    // seam), the rows are appended as before.
    let prior = if expiry_events.is_empty() {
        prior
    } else {
        if let Err(exit) = gate_rows_through(
            &store,
            boundary.as_mut(),
            &state,
            expiry_events,
            Some((
                &resolved.stage,
                inputs.run_id().ok().as_deref().unwrap_or_default(),
            )),
        )? {
            return Ok(exit);
        }
        let guard = crate::store::acquire(&store, "the run")?;
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

    // *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8.)* The review pass's seal is an
    // act made AFTER `resume` has taken this stage's boundary, so it gets a sibling over the same
    // wrapper and target. Unbound (`run_with`'s machinery seam), there is no seal, as there is no
    // `closed` mark.
    let seal_boundary = boundary
        .as_ref()
        .map(rk_kernel::runtime::tier_one::Boundary::sibling);
    let recorded = match rk_kernel::replay::resume_routed(
        inputs,
        ports,
        routing,
        &prior,
        supply_spec,
        boundary,
    ) {
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
        // 🛑 **ADR-0144. AN EXPLICIT ARM, BECAUSE THE CATCH-ALL BELOW IS THE DEFECT.**
        //
        // `Err(e) => Exit::ConfigError` maps everything it is not told about to row `3`. That is
        // exactly the *"all eleven rows arrived here as the SAME value"* failure Mechanism B was
        // added to close, and a new variant left to fall through would re-open it silently — which
        // is why this arm exists rather than a comment asking the next author to remember.
        //
        // The class is NOT optional on this variant: it is only constructible from a step failure.
        // So there is no `None` branch to invent a row for, and `impl From<&StepError> for Exit`
        // — eleven arms, no wildcard — is the one mapping, unchanged. **No row of §7d.4's table is
        // added, moved or renumbered.**
        Err(rk_kernel::replay::ReplayError::StepFailedAfterCommit {
            step_id,
            activity_id_at_version,
            class,
        }) => {
            eprintln!(
                "redkiln: the activation `{activity_id_at_version}` committed, and the later step \
                 `{step_id}` of its walk then failed. The commit stands; the run does not."
            );
            return Ok(Exit::from(&class));
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

    // ADR-0152: the boundary the kernel took, reported to the lane loop before the terminal read.
    *advanced = recorded.advanced_to().map(ToOwned::to_owned);

    // ~~§7h.2: the `Story:` axis. As close to the boundary as the seams allow, and BEFORE the
    // terminal read (ADR-0155 clause 4).~~ *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e`
    // §8e.3.)* `story_trailer` is DELETED, not re-ordered: the stage-bearing `Story:` mark is minted
    // by the kernel's tier-1 boundary at the lane entry and at every advance, BEFORE its
    // `stage_transition` row exists, so ADR-0155's bullet against writing it from anywhere after
    // the store row is readable is true by construction.

    // **§7h.3's call site (ADR-0162).** After the boundary and the trailer, before the terminal
    // read: a blocking verdict is this run's last word, and it travels on the return channel.
    if let Some(exit) = review_pass(
        resolved,
        env,
        &recorded,
        &store,
        &candidate_toolchain,
        tree_hash,
        now,
        seal_boundary,
    )? {
        return Ok(exit);
    }

    // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.6.)* The lane's end, reported to
    // the lane loop only once the review pass has not blocked: a blocking verdict is not an end.
    *lane_end = recorded.lane_ended();

    Ok(terminal_read(resolved, &recorded))
}

/// The `Story:` VALUE, as design §5.4's grammar fixes it: `<project> "/" <slug>` (ADR-0155).
///
/// A function rather than an inline `format!` so the round-trip test asserts the shape the producer
/// actually emits. A test that re-spelled the format string would be asserting its own copy, and the
/// failure this guards is precisely a value that LOOKS right and does not parse: `grep '^Story:'`
/// passes on `Story: /item`, on `Story:item`, and on a project containing a space.
///
/// *(Widened 2026-09-28, `s8e` build, package S8E-P4; `s8e` §8e.5; ADR-0183, proposed.)* The value
/// is `<project> "/" <slug> SP <stage>`: ~~`format!("{project}/{item}")`~~, the stage-less form, is
/// the legacy mark the producer never writes again. This stays a plain `format!` so the
/// ADR-0159 test can show what an empty project renders; the producer does not write it directly,
/// it writes `rk_git`'s round-trip-checked rendering (`ParsedTrailer::to_trailer`).
#[must_use]
pub fn story_value(project: &str, item: &str, stage: &str) -> String {
    format!("{project}/{item} {stage}")
}

// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3.)* `story_trailer` — *"§7h.2's first
// axis: mint `Story: <project>/<item>` — ONCE per item (ADR-0155)"* — is DELETED, with its comment
// (§8e.13 finding 9). The stage-bearing mark is the kernel's tier-1 boundary's now, minted at the
// lane entry and at every advance through `rk_kernel::runtime::tier_one::commit_act`, git first. Its
// ADR-0159 refusals travel on `TierOneError::NoProject` / `ProjectHasSlash`, and a stage the
// grammar cannot carry on `TierOneError::Unrenderable`.

/// **The tier-1 boundary for one stage of this run** (`s8e` §8e.3): the tree the run entered as the
/// target, the ownership the STORE records (`assigned_ownership`), and the two values a `Story:`
/// value names. *(dated 2026-09-28, `s8e` build, package S8E-P5.)*
fn tier_one_boundary(
    resolved: &Resolved,
    target: &Path,
    ownership: &rk_git::worktree::RunOwnership,
) -> rk_kernel::runtime::tier_one::Boundary {
    rk_kernel::runtime::tier_one::Boundary::live(
        resolved.exec.clone(),
        rk_kernel::runtime::tier_one::Target {
            worktree: target.to_path_buf(),
            ownership: ownership.clone(),
            project: resolved.project.clone(),
            item: resolved.item.id().to_owned(),
        },
    )
}

/// **Gate transition rows, each as its own tier-1 act** (`s8e` §8e.3, N6): an expiry, an answer, a
/// void or an opening — its `Gate:` mark first, the row anchored to it. An `opened` row carries the
/// opening stage run when `opening` names one. Unbound (`run_with`'s machinery seam), the rows are
/// appended as they were before `s8e`. *(dated 2026-09-28, `s8e` build, package S8E-P5.)*
///
/// # Errors
/// [`Halt::PoisonedLock`] from a store acquisition; a refusal is the inner `Exit` (`3`), named.
pub fn gate_rows_through(
    store: &Arc<Mutex<dyn EventStore>>,
    boundary: Option<&mut rk_kernel::runtime::tier_one::Boundary>,
    state: &rk_kernel::workflow::fsm::RunState,
    rows: Vec<rk_types::event::Event>,
    opening: Option<(&str, &str)>,
) -> Result<Result<(), Exit>, Halt> {
    let Some(boundary) = boundary else {
        let mut guard = crate::store::acquire(store, "the gate rows")?;
        for row in rows {
            if let Err(e) = guard.append(row) {
                drop(guard);
                eprintln!("redkiln: a gate row could not be written: {e:?}");
                return Ok(Err(Exit::ConfigError));
            }
        }
        drop(guard);
        return Ok(Ok(()));
    };
    for row in rows {
        let act = if row.kind == rk_types::event_kind::EventKind::GateOpened {
            rk_kernel::runtime::tier_one::Act::gate_opened(row).map(|act| match opening {
                Some((stage, run_id)) if !run_id.is_empty() => act.opened_by(stage, run_id),
                _ => act,
            })
        } else {
            let token = row
                .payload
                .get("token")
                .and_then(serde_json::Value::as_str)
                .and_then(|t| rk_types::digest::Digest::from_hex(t).ok());
            let Some(record) = token.as_ref().and_then(|t| state.gate_record(t)) else {
                eprintln!(
                    "redkiln: a `{}` row names no gate this journal opened, so its `Gate:` mark \
                     cannot be written: {}",
                    row.kind.wire_name(),
                    row.payload
                );
                return Ok(Err(Exit::ConfigError));
            };
            rk_kernel::runtime::tier_one::Act::gate(row, record)
        };
        let committed = act.and_then(|act| boundary.act(store, act));
        if let Err(e) = committed {
            eprintln!("redkiln: a gate transition was refused: {e}");
            return Ok(Err(Exit::ConfigError));
        }
    }
    Ok(Ok(()))
}

/// Step (10): the terminal read.
///
/// **`run` NEVER WAITS** — no channel, prompt, socket or sleep is entered on any arm (golden
/// constraint 2). A gate among the resumed run's new events is an EXIT, not a wait.
///
/// *(dated 2026-09-28, `s8e` build, package S8E-P5; `s8e` §8e.3.)* **Its commit is DELETED**, not
/// re-ordered: the `Gate: … opened` mark is the kernel's tier-1 boundary's, made when the gate
/// opened and BEFORE its row. What is left here is the report. Which gate it reports — the fold's
/// awaited one rather than the latest tokened `gate_opened` — is `s8e` §8e.7's change.
fn terminal_read(resolved: &Resolved, recorded: &rk_kernel::replay::RecordedRun) -> Exit {
    let item_id = resolved.item.id();

    // ~~🛑 THE GATE ROW THIS READS IS THE RUNTIME'S, AND IT IS THE ONE CARRYING A TOKEN.~~ (the
    // latest tokened `gate_opened` row, with no answered/void/expired filter — ADR-0161's recorded
    // fourth finding.)
    //
    // *(dated 2026-09-28, `s8e` build, package S8E-P6; `s8e` §8e.7; D21-42 24c.)* **It reports the
    // gate the fold AWAITS**, and only that: `fsm::awaiting_gate()` over the recorded journal, which
    // excludes an answered, a void and an expired token. So a re-run after an answer prints no open
    // gate, and a dead token is never reported (the commit went with §8e.3; this only reports). A
    // journal that will not fold reports nothing rather than guessing.
    let awaited = rk_kernel::workflow::fsm::fold(item_id, &resolved.stage, &recorded.events)
        .ok()
        .and_then(|state| state.awaiting_gate().map(|t| t.as_str().to_owned()));
    let Some(token_hex) = awaited else {
        return Exit::Success;
    };

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

/// The reference worktree for the review pass: a **detached** tree at the merge base.
///
/// ~~🛑 **THE MERGE BASE NEEDS NO COMPUTATION, AND THAT IS LOAD-BEARING.** `GitOp` is ADR-0105
/// clause 5's CLOSED set and carries no `merge-base`; widening it for this would be a decision
/// rather than an implementation. It does not have to be. `recreate_run_worktree` creates
/// `rk-run/<item>` **at `HEAD`**, so the base of `HEAD` and that branch IS `HEAD` — which is what
/// Checkpoint 7h's own `git merge-base HEAD "rk-run/$FIX_ITEM"` returns while the run branch is
/// ahead of it. AC-518 keeps the checkout from moving during a run, so the reading is stable for
/// the pass's lifetime.~~
///
/// *(Rewritten 2026-09-27, `s8d-ii` build, P6; ADR-0180 decision 10, proposed; ADR-0162 decision
/// 10 cited, not superseded.)* 🛑 **THE MERGE BASE STILL NEEDS NO COMPUTATION, BUT IT IS NO LONGER
/// `HEAD`.** ~~`GitOp` still carries no `merge-base` (ADR-0105 clause 5's list; ADR-0180 adds
/// none).~~ *(dated 2026-09-28, `s8d-ii` build, package R2FIX-8dii; refuting round 2.)* `GitOp` now
/// carries `MergeBase`, a read (D21-89 ruling 2; ADR-0180 decision 15, proposed), which `rk-git`'s
/// worktree `create` runs only when a run tree is created on an existing `rk-run/<item>` branch,
/// to record `merge-base(start HEAD, tip)` in `RK_BASE`. This function still computes nothing: it
/// reads its base from `RK_BASE` and runs no `merge-base`. On that arm the base is not the tip the
/// run tree checks out, so `rk-git` also runs the length rule and V1's filter read over it before
/// the run tree is added (§8d-ii.5, §8d-ii.6).
/// The argument above rested on `recreate_run_worktree` creating `rk-run/<item>` at `HEAD` on every
/// run, and that function is retired: under reuse (D21-45 W2) the run branch forked at an earlier
/// `HEAD`, and the operator's checkout may have moved since, so `HEAD` is not the base. The base is
/// the fork sha the run tree's `RK_BASE` recorded at creation, read without a `merge-base`, and the
/// reference tree is created there.
///
/// **A leftover is removed only if redkiln made it.** The reference tree gets its own `RK_BASE`
/// (the fork and the item) at creation. A leftover at `<item>.review-ref` from a killed run that is
/// registered and carries that marker is scratch, and it is force-removed as before, inside
/// [`remove_reference_worktree`]. Anything else there, a tree without the marker or a directory git
/// does not list, is refused by name and kept: redkiln removes only a tree it made.
fn reference_worktree(resolved: &Resolved) -> Result<PathBuf, Exit> {
    let exec = &resolved.exec;
    let repo = &resolved.worktree;
    let item = resolved.item.id();
    let tree = run_tree_of(resolved)?;
    let path = tree.reference.clone();

    // The base: the fork the run tree's RK_BASE records (ADR-0180 decisions 5 and 10).
    let fork = match rk_git::worktree::read_rk_base(exec, &tree.path) {
        Ok(Some(base)) if base.item == item => base.fork,
        Ok(_) => {
            eprintln!(
                "redkiln: the run tree {} carries no RK_BASE for `{item}`, so the review pass has \
                 no recorded fork to create its reference tree at; refused rather than read a base \
                 off HEAD (ADR-0180 decision 10)",
                tree.path.display()
            );
            return Err(Exit::ConfigError);
        }
        Err(e) => {
            eprintln!(
                "redkiln: the run tree's RK_BASE could not be read: {}",
                git_refusal_text(&e)
            );
            return Err(Exit::ConfigError);
        }
    };

    // A leftover from a killed run: removed as scratch only when redkiln made it.
    if std::fs::symlink_metadata(&path).is_ok() {
        let fold = |p: &Path| {
            p.display()
                .to_string()
                .replace('\\', "/")
                .trim_end_matches('/')
                .to_lowercase()
        };
        let registered = rk_git::worktree::registered_worktrees(exec, Path::new(repo.as_str()))
            .map(|all| all.iter().any(|p| fold(p) == fold(&path)))
            .unwrap_or(false);
        let marked = registered
            && matches!(
                rk_git::worktree::read_rk_base(exec, &path),
                Ok(Some(base)) if base.item == item
            );
        if !marked {
            eprintln!(
                "redkiln: a leftover review reference tree at {} carries no RK_BASE redkiln wrote \
                 for `{item}`{}; redkiln removes only a tree it made, so it is kept and the review \
                 pass is refused (ADR-0180 decision 10)",
                path.display(),
                if registered {
                    ""
                } else {
                    " and is not a registered worktree"
                }
            );
            return Err(Exit::ConfigError);
        }
        remove_reference_worktree(exec, repo, &path);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            eprintln!("redkiln: {}: {e}", parent.display());
            Exit::ConfigError
        })?;
    }
    exec.run(
        rk_git::git_exec::GitOp::WorktreeAdd,
        repo,
        [
            std::ffi::OsStr::new("worktree"),
            std::ffi::OsStr::new("add"),
            std::ffi::OsStr::new("--detach"),
            path.as_os_str(),
            std::ffi::OsStr::new(&fork),
        ],
    )
    .map_err(|e| {
        // 🛑 **THE REFUSAL CLEANS UP AFTER ITSELF, and the adversarial pass is why.** ADR-0162
        // decision 10 says the reference worktree is removed *"on every path, including every
        // refusal path"*, and this arm was the one path that did not: `git worktree add` can
        // REGISTER the tree and then fail, and the caller's `Err` arm returns before the
        // removal it relies on. A registration that outlives its run turns Checkpoint 7h
        // guard (9) — which compares worktree PATH SETS across a prune — red, which is the
        // failing guard this whole change exists to stop trading for another.
        remove_reference_worktree(exec, repo, &path);
        eprintln!("redkiln: the review reference worktree could not be created: {e:?}");
        Exit::ConfigError
    })?;
    // Its own marker, so a leftover of it is known to be redkiln's (ADR-0180 decision 10).
    if let Err(e) = rk_git::worktree::write_rk_base(
        exec,
        &path,
        &rk_git::worktree::RkBase {
            fork,
            item: item.to_owned(),
        },
    ) {
        remove_reference_worktree(exec, repo, &path);
        eprintln!(
            "redkiln: the review reference worktree's RK_BASE could not be written: {}",
            git_refusal_text(&e)
        );
        return Err(Exit::ConfigError);
    }
    Ok(path)
}

/// Remove the reference worktree.
///
/// 🛑 **ADR-0162 decision 10 — called on EVERY path, including every refusal path.** Checkpoint 7h
/// guard (9) compares the worktree **path set** across a `git worktree prune` and fails on any
/// difference — the path-set form ADR-0156 clause 5(d) required because *"counts cannot tell a
/// leaked registration from a replaced one"*. A pass that left this registration behind would turn
/// guard (9) RED and trade one failing guard for another. This is deliberately **unlike** the
/// run-owned worktree, which is ~~recreated and never removed~~ reused and never force-removed
/// *(dated 2026-09-27, `s8d-ii` build, P6; D21-45 W2; ADR-0180 decisions 9 and 10, proposed)*:
/// that one is the run's output and is read after the run; this one is scratch and nothing outside
/// the pass may depend on it. It is the only forced removal left in this file (Checkpoint 8d-ii
/// (3)), and its `remove_dir_all` fallback does not follow a junction. A leftover of it is removed
/// only when it carries the `RK_BASE` [`reference_worktree`] writes.
///
/// A failure is ignored: a leaked scratch tree is an operator problem, never a verdict.
fn remove_reference_worktree(exec: &GitExec, repo: &WorktreePath, path: &Path) {
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
    let _ = std::fs::remove_dir_all(path);
}

/// **The review pass — the producer of `Exit::FindingsBlock` (ADR-0162; §7h.3's call site).**
///
/// `Ok(None)` is *"this activation is not a review"*, which is the ordinary path for every other
/// stage and is silent. `Ok(Some(exit))` is a verdict or a refusal, and it travels on the **return
/// channel** rather than an out-parameter — ADR-0161 defect (2) was exactly a refusal that reached
/// stderr on a channel the caller did not read.
///
/// 🛑 **No verdict is constructed here.** `policy::Block`'s members are private and it has no
/// public constructor (AC-294), so the only route to a block is to read one back out of a
/// `Confirmed` the kernel produced. This function assembles inputs and reads an answer.
#[expect(
    clippy::too_many_arguments,
    reason = "the seal's boundary joins the pass's six inputs (s8e section 8e.8); each is one \
              value step (9) already holds, and bundling them would be a struct for one caller"
)]
fn review_pass(
    resolved: &Resolved,
    env: &Env,
    recorded: &rk_kernel::replay::RecordedRun,
    store: &Arc<Mutex<dyn rk_ports::event_store::EventStore>>,
    candidate: &Arc<dyn rk_ports::toolchain::Toolchain>,
    tree_hash: &str,
    now: i64,
    // *(dated 2026-09-28, `s8e` build, package S8E-P7; `s8e` §8e.8.)* The seal's tier-1 boundary;
    // `None` on the unbound machinery seam, which seals nothing.
    mut seal_boundary: Option<rk_kernel::runtime::tier_one::Boundary>,
) -> Result<Option<Exit>, Halt> {
    let findings = match crate::review_pass::findings_in(&recorded.events) {
        Ok(findings) => findings,
        // Not a review activation. Every non-review stage takes this path.
        Err(crate::review_pass::PassSkipped::NoFindingsRow) => return Ok(None),
        Err(crate::review_pass::PassSkipped::Malformed(e)) => {
            eprintln!(
                "redkiln: the review dispatch committed a result that is not a finding set: \
                 {e:?} — an unreadable payload is a refusal, never an empty set (AC-282)"
            );
            return Ok(Some(Exit::ConfigError));
        }
    };

    let pack = match rk_kernel::select::Pack::from_toml(
        &format!("{}.toml", resolved.pack_name),
        &resolved.pack_src,
    ) {
        Ok(pack) => pack,
        Err(e) => {
            eprintln!("redkiln: {}.toml: {e}", resolved.pack_name);
            return Ok(Some(Exit::ConfigError));
        }
    };
    // AC-302: the tier derives from the AUTHORED stage rigor and nothing else — there is no
    // parameter a model-produced value could arrive through.
    let weight = rk_kernel::review::policy::resolve_review_tier(pack.rigor(&resolved.stage));

    let reference = match reference_worktree(resolved) {
        Ok(path) => path,
        Err(exit) => return Ok(Some(exit)),
    };

    // Everything between here and the removal is fallible, and the removal must happen on EVERY
    // path (decision 10) — so the body is a closure whose result is held until after it.
    let verdict = (|| -> Result<Option<Exit>, Halt> {
        let reference_toolchain = match real_toolchain(resolved, env, &reference) {
            Ok(toolchain) => toolchain,
            Err(exit) => return Ok(Some(exit)),
        };
        let reexecutor = crate::review_pass::ToolchainReExecutor {
            candidate: candidate.as_ref(),
            reference: reference_toolchain.as_ref(),
        };
        let confirmed = crate::review_pass::confirm(
            &findings,
            weight,
            tree_hash,
            resolved.item.id(),
            rk_types::time::Timestamp::from_unix_seconds(now),
            &reexecutor,
        );

        let blocked = confirmed.block_set();

        // The rows reach the STORE, or `review_block` never lands in the journal that §7d.4 names
        // as exit 4's identity and `RK-D-SINK-COUNT` has nothing to read (decision 6).
        //
        // 🛑 **AND THE JOURNAL IS RE-READ. The adversarial pass found this half missing.** ADR-0162
        // decision 6 says the rows are appended *"and the journal is re-read, by the `expiry_pass`
        // idiom"* and quotes that idiom's own reason — *"a pass that wrote nowhere would be
        // invisible to it, and reading it back is what makes the append observable rather than
        // assumed"*. The first cut appended and then read `block_set()` off the IN-MEMORY
        // `Confirmed`, so the record stated a behaviour the code did not implement and the exit
        // rested on the store's `Ok` rather than on the journal. `append` returning `Ok` is the
        // store's word for it; the query is the journal's.
        if !confirmed.events.is_empty() {
            let mut guard = crate::store::acquire(store, "the review")?;
            for event in confirmed.events.clone() {
                if let Err(e) = guard.append(event) {
                    drop(guard);
                    eprintln!("redkiln: a review row could not be written: {e:?}");
                    return Ok(Some(Exit::ConfigError));
                }
            }
            let re_read = guard.query(rk_kernel::runtime::journal_query(resolved.item.id()));
            drop(guard);
            let rows = match re_read {
                Ok(rows) => rows,
                Err(e) => {
                    eprintln!("redkiln: the journal did not re-read after the review rows: {e:?}");
                    return Ok(Some(Exit::ConfigError));
                }
            };
            let readable = rows
                .iter()
                .filter(|e| e.kind == rk_types::event_kind::EventKind::ReviewBlock)
                .count();
            if readable < blocked.len() {
                eprintln!(
                    "redkiln: {} finding(s) blocked and only {readable} `review_block` row(s) read \
                     back out of the journal — the verdict would be reported on an append nothing \
                     can observe",
                    blocked.len()
                );
                return Ok(Some(Exit::ConfigError));
            }
        }

        // ---- THE SEAL, ON BOTH OUTCOMES (`s8e` §8e.8; D21-41 e; the slice record, proposed). *(dated
        // 2026-09-28, `s8e` build, package S8E-P7.)* The rejecting verdict crosses no boundary
        // (ADR-0161 decision 2: the lane loop returns before reading `advanced`), so the seal sits on
        // the review stage's EXIT OUTCOME, the value ADR-0162 wired without a stage literal:
        // `blocked.is_empty()` seals `approved`, and `Exit::FindingsBlock` seals `changes-requested`
        // BEFORE it returns. Both go through the kernel's tier-1 boundary (`tier_one::seal_act`, then
        // `Boundary::act`): the `Slice-Verdict:` commit first, then `t1_slice_seal` and the anchored
        // `slice_sealed` row. A refused seal — the slice undeclared, the item id, the project, a stage,
        // whitespace — is exit 3, named, and nothing is committed.
        let verdict = if blocked.is_empty() {
            rk_git::trailer::grammar::Verdict::Approved
        } else {
            rk_git::trailer::grammar::Verdict::ChangesRequested
        };
        if let Some(boundary) = seal_boundary.as_mut() {
            let sealed = rk_kernel::runtime::tier_one::seal_act(
                resolved.slice.as_deref(),
                resolved.item.id(),
                resolved.project.as_deref(),
                pack.stage_order(),
                &resolved.stage,
                verdict,
            )
            .and_then(|act| boundary.act(&**store, act));
            if let Err(e) = sealed {
                eprintln!(
                    "redkiln: the review outcome of `{}` ({}) could not be sealed: {e}",
                    resolved.item.id(),
                    verdict.as_str()
                );
                return Ok(Some(Exit::ConfigError));
            }
        }

        if blocked.is_empty() {
            return Ok(None);
        }
        eprintln!(
            "redkiln: the review blocks on {} finding(s): {}",
            blocked.len(),
            blocked
                .iter()
                .map(|r| r.ac.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
        Ok(Some(Exit::FindingsBlock))
    })();

    remove_reference_worktree(&resolved.exec, &resolved.worktree, &reference);
    verdict
}

// *(dated 2026-09-27, `s8d-ii` build, P6; D21-45 W2; ADR-0180 decision 9, proposed.)*
// `recreate_run_worktree` is retired: its *"**Re-created, never reused**"* (ADR-0105 clause 6) is
// superseded in part for the run-owned tree, and its `worktree remove --force` + `remove_dir_all`
// at every run start are gone with it. The tree is entered through `rk_git::worktree::enter`
// below: created at `.redkiln/worktrees/<item>` under the main worktree, re-entered and reused, or
// refused and kept. Nothing in this file forces a removal of it.

/// A refusal from `rk-git`, rendered by name: a [`rk_git::worktree::WorktreeRefusal`]'s own
/// `Display` (it names the path or id and the rule), and any other error as itself.
fn git_refusal_text(e: &rk_git::error::GitError) -> String {
    match e {
        rk_git::error::GitError::WorktreeRefused { refusal } => refusal.to_string(),
        other => format!("{other:?}"),
    }
}

/// **The run tree this run's item derives to** — through `rk-git`'s one derivation (§8d-ii.1;
/// ADR-0180 decision 1, proposed): the main worktree's directory, read from `git worktree list
/// --porcelain` whichever checkout `run` started in, and the validated item id. `story_trailer`,
/// `terminal_read` and the review pass read their paths from here, never from a joined literal.
///
/// # Errors
/// [`Exit::ConfigError`], naming the refusal (a bare repository, an unsafe item id).
fn run_tree_of(resolved: &Resolved) -> Result<rk_git::worktree::RunTree, Exit> {
    let refused = |e: rk_git::error::GitError| {
        eprintln!(
            "redkiln: the run-owned worktree for `{}` cannot be derived: {}",
            resolved.item.id(),
            git_refusal_text(&e)
        );
        Exit::ConfigError
    };
    let main =
        rk_git::worktree::main_worktree(&resolved.exec, Path::new(resolved.worktree.as_str()))
            .map_err(refused)?;
    let id = rk_git::worktree::ItemId::parse(resolved.item.id()).map_err(refused)?;
    Ok(rk_git::worktree::derive(&main, &id))
}

/// **This process as a lock owner: its pid and its start time as the OS reports it** (W1; ADR-0180
/// decision 6, proposed), the `N` and `T` of `redkiln run <item> (pid N start T)`.
///
/// # Errors
/// [`Exit::ConfigError`] when the OS reports no start time for this very process: a lock whose
/// owner cannot be told from a reused pid is not taken (fail closed).
fn run_lock_owner() -> Result<rk_git::worktree::LockOwner, Exit> {
    let pid = std::process::id();
    rk_containment::process_start_time(pid)
        .map(|start| rk_git::worktree::LockOwner { pid, start })
        .ok_or_else(|| {
            eprintln!(
                "redkiln: the OS reports no start time for this process (pid {pid}), so the run \
                 tree's lock could not name its owner; the run is refused rather than take a lock \
                 a reused pid could be mistaken for (ADR-0180 decision 6)"
            );
            Exit::ConfigError
        })
}

/// **The liveness predicate `rk-git` is handed** (§8d-ii.2; ADR-0180 decision 6, proposed): the
/// start time of the live process `pid` names, built from `rk_containment::is_alive` and
/// `rk_containment::process_start_time`, so `rk-git` gains no edge to `rk-containment`.
///
/// `is_alive` is a production call from here on. Its documented Windows ambiguity (an exit code of
/// 259 reads alive) keeps a stale lock held and refuses the run by name; it can never release a
/// live one. That is fail-closed, and pid reuse is what the start-time comparison answers.
fn process_probe(pid: u32) -> Option<u64> {
    if rk_containment::is_alive(pid) {
        rk_containment::process_start_time(pid)
    } else {
        None
    }
}

/// **Step (8)'s run tree: entered, not re-created** (§8d-ii.1 to §8d-ii.5; D21-44; D21-45 W1-W4;
/// D21-47 V1; D21-48 X1 and X2; ADR-0180, proposed).
///
/// `rk_git::worktree::enter` creates the tree at `.redkiln/worktrees/<item>` under the main
/// worktree (the guards, the filter read with `GIT_LFS_SKIP_SMUDGE` from [`Env`], the add,
/// `RK_BASE`, the post-checkout scan, the lock), or re-enters and reuses it after an explicit
/// discard, or refuses and keeps it. Every refusal is by name and exits `3`.
///
/// **The discard reaches the operator on stderr, one line per path, until `s8e`** mints
/// `worktree_assigned { …, discarded }` (§8d-ii.3; W2's *"journalled"* is met there).
///
/// The filter read's admission facts (`Entered::filter_notes`) are for the **debug** level only
/// (§8d-ii.5), and `rk-cli` has no debug level, so they are printed nowhere.
///
/// # Errors
/// [`Exit::ConfigError`], naming the refusal.
fn enter_run_worktree(
    resolved: &Resolved,
    env: &Env,
    owner: &rk_git::worktree::LockOwner,
) -> Result<rk_git::worktree::Entered, Exit> {
    let entered = rk_git::worktree::enter(
        &resolved.exec,
        Path::new(resolved.worktree.as_str()),
        resolved.item.id(),
        owner,
        &process_probe,
        env.lfs_skip_smudge(),
    )
    .map_err(|e| {
        eprintln!(
            "redkiln: the run-owned worktree for `{}` was refused: {}",
            resolved.item.id(),
            git_refusal_text(&e)
        );
        Exit::ConfigError
    })?;
    for path in &entered.discarded {
        eprintln!(
            "redkiln: re-entry discarded `{path}` in {} (not landed at a committed step)",
            entered.tree.path.display()
        );
    }
    Ok(entered)
}

/// **W3's carry, at creation only** (§8d-ii.4; D21-45 W3; D21-48 X14: *"s8c builds the reader, and
/// s8d-ii calls it at creation"*; ADR-0180 decision 11, proposed). *(dated 2026-09-27, `s8d-ii`
/// build, P6.)*
///
/// Called after `rk_git::worktree::enter` created the tree (the add, the realpath check, `RK_BASE`,
/// D21-48 X2's scan and the lock are behind it), and never for a reused tree, whose ignored files
/// survived the discard. In order:
/// 1. **`s8c`'s `.worktreeinclude` reader** (`rk_git::worktree_include::carry_set`) over the main
///    worktree: ignored ∩ matched, symlinks and junctions skipped and named, escapes refused. Each
///    carried file is copied to the same relative path in the tree. Nothing is linked.
/// 2. **`s8c`'s per-toolchain `setup` verb**, through `run_verb`, on the toolchain the run's own
///    selection loop chooses in the tree, with the `setup_projects` declaration row of that
///    toolchain's `[toolchain.<name>]` table and the `setup` row's timeout key as its bound. No
///    `registry.toml`, no detected adapter, or no `setup_projects` row: nothing is installed,
///    because nothing was declared.
///
/// # Errors
/// [`Exit::ConfigError`], naming the refusal: the carry's (`s8c`'s `CarryRefusal`), a copy that
/// fails, the registry's, a `setup` row or bound that cannot be read, or `setup`'s own by-name
/// refusal. The caller releases the lock and the run stops.
fn carry_at_creation(
    resolved: &Resolved,
    env: &Env,
    entered: &rk_git::worktree::Entered,
) -> Result<(), Exit> {
    let tree = &entered.tree.path;
    let main =
        rk_git::worktree::main_worktree(&resolved.exec, Path::new(resolved.worktree.as_str()))
            .map_err(|e| {
                eprintln!(
                    "redkiln: the carry could not read the main worktree: {}",
                    git_refusal_text(&e)
                );
                Exit::ConfigError
            })?;

    // ---- (1) the .worktreeinclude carry.
    let carry = rk_git::worktree_include::carry_set(&resolved.exec, &main).map_err(|e| {
        let why = match &e {
            rk_git::error::GitError::CarryRefused { refusal } => refusal.to_string(),
            other => format!("{other:?}"),
        };
        eprintln!(
            "redkiln: the .worktreeinclude carry into {} was refused: {why}",
            tree.display()
        );
        Exit::ConfigError
    })?;
    for rel in &carry.carried {
        let from = main.join(rel);
        let to = tree.join(rel);
        let copied = to
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::copy(&from, &to).map(|_| ()));
        if let Err(e) = copied {
            eprintln!(
                "redkiln: the .worktreeinclude carry could not copy `{rel}` into {}: {e}",
                tree.display()
            );
            return Err(Exit::ConfigError);
        }
    }
    for skipped in &carry.skipped {
        eprintln!(
            "redkiln: .worktreeinclude: `{}` is not carried ({})",
            skipped.path,
            skipped.reason.name()
        );
    }

    // ---- (2) the setup verb.
    let registry_file = registry_path(Path::new(resolved.worktree.as_str()));
    if !registry_file.is_file() {
        return Ok(());
    }
    let registry =
        rk_kernel::workflow::memo::VerbRegistry::load_file(&registry_file).map_err(|e| {
            eprintln!("redkiln: {}: {e}", registry_file.display());
            Exit::ConfigError
        })?;
    let chosen = choose_toolchain(&ToolchainCandidates::of(resolved), env, tree, &mut |line| {
        eprintln!("redkiln: {line}");
    })?;
    let Some(toolchain) = chosen.adapter.as_deref().and_then(toolchain_of) else {
        return Ok(());
    };
    let Some(declared) = registry
        .table(toolchain)
        .and_then(|t| t.declarations().get("setup_projects"))
    else {
        return Ok(());
    };
    let refuse = |why: String| {
        eprintln!(
            "redkiln: setup at run-tree creation was refused for {}: {why}",
            tree.display()
        );
        Exit::ConfigError
    };
    let projects: Vec<serde_json::Value> = declared
        .value
        .as_array()
        .ok_or_else(|| {
            refuse(format!(
                "`toolchain.{toolchain}.setup_projects` is not an array"
            ))
        })?
        .iter()
        .map(|v| {
            v.as_str()
                .map(|s| serde_json::Value::String(s.to_owned()))
                .ok_or_else(|| {
                    refuse(format!(
                        "`toolchain.{toolchain}.setup_projects` holds a value that is not a path"
                    ))
                })
        })
        .collect::<Result<_, _>>()?;
    if projects.is_empty() {
        return Ok(());
    }
    let entry = registry
        .entry("setup")
        .ok_or_else(|| refuse("`registry.toml` declares no `setup` row".to_owned()))?;
    let bound = registry
        .values_for(toolchain)
        .ok_or_else(|| refuse(format!("no `[toolchain.{toolchain}]` value table")))?
        .resolve(&entry.timeout)
        .map(rk_ports::toolchain::StepBound::Secs)
        .map_err(|r| refuse(r.to_string()))?;
    let inputs = BTreeMap::from([("projects".to_owned(), serde_json::Value::Array(projects))]);
    chosen
        .toolchain
        .run_verb(&rk_ports::toolchain::VerbCall {
            verb: "setup",
            inputs: &inputs,
            bound,
        })
        .map_err(|e| refuse(format!("{e:?}")))?;
    Ok(())
}

/// **Roll back the FRESH run tree whose carry or `setup` failed** (D21-88 ruling 2; ADR-0180
/// decisions 9 and 11, proposed). *(dated 2026-09-27, `s8d-ii` build, package D2188.)*
///
/// `rk_git::worktree::roll_back_fresh` removes the tree this run just created, unforced, after
/// unlinking its untracked links, and keeps the branch; a reused tree, or one this run did not
/// lock, is kept. The outcome is named on stderr and does not change the run's exit, which is
/// the carry's or `setup`'s own refusal. The caller then releases the lock, which is a no-op
/// once the rollback released it.
fn roll_back_run_worktree(
    exec: &GitExec,
    start: &Path,
    entered: &rk_git::worktree::Entered,
    owner: &rk_git::worktree::LockOwner,
) {
    let path = entered.tree.path.display();
    match rk_git::worktree::roll_back_fresh(exec, start, entered, owner) {
        Ok(rk_git::worktree::Rollback::Removed) => eprintln!(
            "redkiln: the fresh run tree {path} was rolled back (its carry or setup failed); the \
             next run creates it anew and re-carries"
        ),
        Ok(rk_git::worktree::Rollback::Kept { reason, .. }) => eprintln!(
            "redkiln: the fresh run tree {path} could not be rolled back and is kept: {reason}"
        ),
        Err(e) => eprintln!(
            "redkiln: the fresh run tree {path} could not be rolled back and is kept: {}",
            git_refusal_text(&e)
        ),
    }
}

/// **Release the run's own lock at the run's end** (W1; ADR-0180 decision 6, proposed). A lock
/// someone else holds is left as it is (`rk_git::worktree::release` unlocks only this run's). A
/// failure is reported and does not change the run's exit: a lock left behind names a dead pid,
/// which the next run releases.
fn release_run_worktree(
    exec: &GitExec,
    start: &Path,
    item: &str,
    owner: &rk_git::worktree::LockOwner,
) {
    if let Err(e) = rk_git::worktree::release(exec, start, item, owner) {
        eprintln!(
            "redkiln: the run-owned worktree's lock for `{item}` could not be released: {}",
            git_refusal_text(&e)
        );
    }
}

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
    fn build(
        &self,
        _bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_types::verb_output::BuildReport, rk_types::error::StepError> {
        Err(self.refuse("build"))
    }
    fn test(
        &self,
        _selection: &rk_ports::toolchain::Selection,
        _bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_ports::toolchain::TestOutcome, rk_types::error::StepError> {
        Err(self.refuse("test"))
    }
    fn lint(
        &self,
        _bound: rk_ports::toolchain::StepBound,
    ) -> Result<(), rk_types::error::StepError> {
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
        _bound: rk_ports::toolchain::StepBound,
    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {
        Err(self.refuse("change_summary"))
    }
    /// *(Added 2026-09-26, s8c §8c.2, record V = ADR-0170, proposed.)* The seventh method refuses
    /// by name like the other six: no adapter detected, so no verb runs.
    fn run_verb(
        &self,
        call: &rk_ports::toolchain::VerbCall<'_>,
    ) -> Result<rk_ports::toolchain::VerbOutput, rk_types::error::StepError> {
        Err(self.refuse(call.verb))
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
///
/// [`Exit::ConfigError`] for the toolchain scratch directory, and for a resolved adapter name this
/// binary cannot construct — the `other =>` arm, which is a drift between `AdapterSet::resolve` and
/// this match and so a build defect rather than a configuration one.
///
/// 🛑 **CORRECTED 2026-09-22 (ADR-0149), AND THE OLD TEXT SAID THE OPPOSITE OF THE CODE.** This
/// section read *"[`Exit::ConfigError`] when an adapter's own construction is refused — a
/// denylisted or incomplete child environment, a relative root, a zero bound. Those are
/// configuration defects and **are not deferred**."* Every one of those three cases is now
/// deferred: each reaches `tried.push(...)` and `continue`, and the loop asks the next candidate.
/// The sentence was left standing when the code beneath it changed, which is the defect ADR-0150
/// records one file over.
///
/// ⚠️ **What a deferral costs, named rather than left to be found.** A zero bound or a relative root
/// is a defect of the *pack or the root*, not of one adapter, so it refuses **every** candidate and
/// the loop falls through to [`UndetectedToolchain`]. A run that reaches no `tool` step then exits
/// **0** where it previously exited 3, and a run that does reach one is refused with *"no configured
/// toolchain adapter detects a project at …"* — which names detection when the truth is the bound.
/// The refusals themselves are printed as they happen (see the loop), so the operator is not left
/// without the cause; deciding whether a whole-root defect should out-rank a per-candidate deferral
/// is left to the stage that owns `Config`, and is not taken here.
///
/// `pub` since 2026-09-22 (ADR-0149), and only so it can be OBSERVED. `rk-cli`'s tests are
/// integration tests — a separate crate — so a private function's behaviour can be asserted only
/// through a whole `redkiln run`, and every route to this one passes through steps (1)-(7) and out
/// the far side into a dispatch. The property ADR-0149 decides is *"a candidate that cannot be
/// configured does not end the run"*, which is a property OF THIS LOOP; asserting it two layers up
/// would be asserting it of something else.
pub fn real_toolchain(
    resolved: &Resolved,
    env: &Env,
    project_root: &Path,
) -> Result<Arc<dyn rk_ports::toolchain::Toolchain>, Exit> {
    choose_toolchain(
        &ToolchainCandidates::of(resolved),
        env,
        project_root,
        &mut |line| eprintln!("redkiln: {line}"),
    )
    .map(|chosen| chosen.toolchain)
}

/// What adapter selection reads from `[adapters]` (s8c §8c.5).
///
/// The candidates in declared order, the construction bound, the RT-49 programs and the common dir. `doctor` builds one from its
/// `ConfigFile` and `run` from its [`Resolved`], so the two select by ONE loop and cannot disagree
/// about which adapter's value table a `${toolchain.*}` key resolves through (record V decision
/// 12).
#[derive(Debug, Clone, Copy)]
pub struct ToolchainCandidates<'a> {
    /// `[adapters] toolchains`, in declared order.
    pub toolchains: &'a [String],
    /// The construction bound, in seconds (ADR-0140 decision 3; record B decision 4).
    pub timeout_secs: u64,
    /// RT-49's per-toolchain programs.
    pub programs: Option<&'a BTreeMap<String, String>>,
    /// The git common directory, under which the toolchain scratch lives (ADR-0137 clause 6).
    pub common_dir: &'a Path,
}

impl<'a> ToolchainCandidates<'a> {
    /// The candidates a run resolved.
    #[must_use]
    pub fn of(resolved: &'a Resolved) -> Self {
        Self {
            toolchains: &resolved.toolchains,
            timeout_secs: resolved.toolchain_timeout_secs,
            programs: resolved.toolchain_programs.as_ref(),
            common_dir: &resolved.common_dir,
        }
    }
}

/// The toolchain [`choose_toolchain`] selected, and the NAME of the adapter behind it.
///
/// The name stays in `rk-cli`: it picks the value table ([`tool_routing`]), and the kernel is
/// handed the table without it (AC-487).
pub struct ChosenToolchain {
    /// The port object.
    pub toolchain: Arc<dyn rk_ports::toolchain::Toolchain>,
    /// The adapter that detected the project, or `None` when nothing did ([`UndetectedToolchain`]).
    pub adapter: Option<String>,
}

/// **ADR-0137's selection loop**, with its reports handed to `report` rather than printed.
///
/// So `doctor` can run the same selection as `run` (s8c §8c.6, `RK-D-TIMEOUT`'s third arm).
/// [`real_toolchain`] is this loop with `report` printing each line to stderr, as before.
///
/// # Errors
/// As [`real_toolchain`].
pub fn choose_toolchain(
    candidates: &ToolchainCandidates<'_>,
    env: &Env,
    project_root: &Path,
    report: &mut dyn FnMut(&str),
) -> Result<ChosenToolchain, Exit> {
    let root = project_root;
    let scratch = candidates.common_dir.join("redkiln").join("tmp");
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        report(&format!(
            "the toolchain scratch {} : {e}",
            scratch.display()
        ));
        return Err(Exit::ConfigError);
    }
    let timeout = std::time::Duration::from_secs(candidates.timeout_secs);
    let child_env = env.as_map().clone();
    let program_for = |name: &str, default: &str| -> PathBuf {
        candidates
            .programs
            .and_then(|m| m.get(name))
            .map_or_else(|| PathBuf::from(default), PathBuf::from)
    };

    let mut tried: Vec<String> = Vec::new();
    for name in candidates.toolchains {
        // `adapters::AdapterSet::resolve` already refused any name outside the shipped two, so an
        // unknown name here is a build defect rather than a configuration one — and it is reported
        // as one instead of being skipped into a "nothing detected" refusal that would misname it.
        let candidate: Arc<dyn rk_ports::toolchain::Toolchain> = match name.as_str() {
            // **ADR-0149.** A candidate that cannot be CONFIGURED or BUILT is recorded in `tried`
            // and the loop moves on; it is no longer a `?` that ends the whole run. Before this,
            // every listed adapter was constructed AND validated before `detect()` chose, so
            // `rk-adapter-toolchain-node`'s required-env table gated every run on a tree where
            // Rust would have been selected — a refusal naming Node on a repository whose root has
            // no `package.json`.
            //
            // 🛑 **THE REFUSAL IS PRINTED WHERE IT HAPPENS, AND THAT IS NOT BELT-AND-BRACES.** An
            // earlier draft of this change pushed to `tried` and nothing else, with a comment
            // claiming the reasons "reach the operator either way, because `UndetectedToolchain`
            // carries `tried`". That is true of ONE of the two exits: if a later candidate DOES
            // detect, the loop `return`s it and `tried` is dropped on the floor — never printed,
            // never journaled. So a genuinely misconfigured Node adapter on a Rust tree, which is
            // the exact case this record exists for, produced ZERO operator output. The `eprintln!`
            // the `?` used to carry is restored at the push, so deferring a candidate costs the
            // operator nothing it used to get.
            "rk-adapter-toolchain-rust" => {
                let cfg = match rk_adapter_toolchain_rust::Config::new(
                    program_for(name, rk_adapter_toolchain_rust::DEFAULT_PROGRAM),
                    timeout,
                    root,
                    child_env.clone(),
                    scratch.clone(),
                ) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        report(&format!("the {name} configuration was refused: {e}"));
                        tried.push(format!("{name}: the configuration was refused: {e}"));
                        continue;
                    }
                };
                // *(2026-09-26, s8c §8c.4, owed by P5 to the routing slice.)* The `setup` verb's
                // `pnpm` is resolved through `toolchain_programs`, as `cargo` is (record V decision
                // 7); absent, the adapter's own `DEFAULT_PNPM` stands.
                let cfg = match candidates.programs.and_then(|m| m.get("pnpm")) {
                    Some(pnpm) => cfg.with_tool_program("pnpm", PathBuf::from(pnpm)),
                    None => cfg,
                };
                match rk_adapter_toolchain_rust::RustToolchain::new(cfg) {
                    Ok(built) => Arc::new(built),
                    Err(e) => {
                        report(&format!("{name} could not be built: {e}"));
                        tried.push(format!("{name}: could not be built: {e}"));
                        continue;
                    }
                }
            }
            // Same shape as the Rust arm above, and for the same reason (ADR-0149). This is the
            // adapter the finding was measured on: its `Config::new` validates a required-env
            // table, so on a Rust-only tree it was the arm that refused before `detect()` ran.
            "rk-adapter-toolchain-node" => {
                let cfg = match rk_adapter_toolchain_node::Config::new(
                    program_for(name, rk_adapter_toolchain_node::DEFAULT_PROGRAM),
                    timeout,
                    root,
                    child_env.clone(),
                    scratch.clone(),
                ) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        report(&format!("the {name} configuration was refused: {e}"));
                        tried.push(format!("{name}: the configuration was refused: {e}"));
                        continue;
                    }
                };
                match rk_adapter_toolchain_node::NodeToolchain::new(cfg) {
                    Ok(built) => Arc::new(built),
                    Err(e) => {
                        report(&format!("{name} could not be built: {e}"));
                        tried.push(format!("{name}: could not be built: {e}"));
                        continue;
                    }
                }
            }
            other => {
                report(&format!(
                    "`{other}` is a resolved toolchain adapter this binary cannot \
                     construct — `AdapterSet::resolve` and this match have drifted apart"
                ));
                return Err(Exit::ConfigError);
            }
        };
        match candidate.detect() {
            Ok(true) => {
                return Ok(ChosenToolchain {
                    toolchain: candidate,
                    adapter: Some(name.clone()),
                });
            }
            Ok(false) => tried.push(format!("{name}: not present")),
            Err(e) => tried.push(format!("{name}: {e:?}")),
        }
    }
    // **ADR-0139.** Not a startup refusal and not a fixture: a toolchain that refuses every verb
    // by name, so a run needing none proceeds and a run reaching a `tool` step is refused there.
    Ok(ChosenToolchain {
        toolchain: Arc::new(UndetectedToolchain {
            root: root.display().to_string(),
            tried: if tried.is_empty() {
                "`[adapters] toolchains` is empty".to_owned()
            } else {
                tried.join("; ")
            },
        }),
        adapter: None,
    })
}

/// The `toolchain` facet value an adapter serves.
///
/// s8c §8c.5: `registry.toml`'s per-toolchain tables are *"keyed by the `toolchain` facet's value"*). A closed match over the two adapters this build
/// ships; any other name has no table. **This is the one place the name meets the table**, and it
/// is `rk-cli`'s: the kernel never sees either (AC-487).
#[must_use]
pub fn toolchain_of(adapter: &str) -> Option<&'static str> {
    match adapter {
        "rk-adapter-toolchain-rust" => Some("rust"),
        "rk-adapter-toolchain-node" => Some("node"),
        _ => None,
    }
}

/// `.redkiln/registry.toml`, under a worktree root.
#[must_use]
pub fn registry_path(worktree: &Path) -> PathBuf {
    worktree.join(".redkiln").join("registry.toml")
}

/// **The routing `run` hands the kernel** (s8c §8c.4-§8c.5; records V and B, proposed).
///
/// The verb registry loaded from `<worktree>/.redkiln/registry.toml`, and the value table for the chosen
/// adapter's toolchain as a flat, name-free `ToolchainValues`.
///
/// - **No `registry.toml`** binds no registry: every `tool` step then refuses by its bare verb name
///   (ADR-0135 clause 2), which is what a tree with no verb table can honestly do.
/// - **A registry that does not load** is `Err`, naming the file and the refusal: a run over a
///   malformed verb table is refused rather than run with part of it.
/// - **No adapter detected**, or no table for its toolchain: the values are unselected, so every
///   `${toolchain.*}` bound refuses naming why, and no key is guessed.
///
/// # Errors
/// The loader's refusal, rendered.
pub fn tool_routing(
    worktree: &Path,
    adapter: Option<&str>,
) -> Result<rk_kernel::runtime::step::ToolRouting, String> {
    let path = registry_path(worktree);
    if !path.is_file() {
        return Ok(rk_kernel::runtime::step::ToolRouting::default());
    }
    let registry = rk_kernel::workflow::memo::VerbRegistry::load_file(&path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let values = toolchain_values(&registry, adapter);
    // *(2026-09-26, `s8d` build, package P4; R2 = ADR-0173, proposed.)* The resolver's `kb:` /
    // `checklist:` / `toolchain:<feed>` arm reads through the same registry: the feed table is the
    // chosen adapter's toolchain (named by the table key, never the adapter), and a declared path
    // is relative to `<worktree>/.redkiln/`.
    let feed_table = adapter.and_then(toolchain_of).map(str::to_owned);
    // *(2026-09-27, `s8d` build, package F2-D2173; R8 = ADR-0179 decision 4, proposed; D21-73 item
    // 5.)* The chosen toolchain's declaration rows that supply a registry row's input
    // (`package_binaries` -> `package.binaries`), bound as row inputs. No table, nothing bound.
    let row_inputs = feed_table
        .as_deref()
        .map(|t| registry.row_inputs_for(t))
        .unwrap_or_default();
    Ok(rk_kernel::runtime::step::ToolRouting::new(registry, values)
        .with_sources(feed_table, Some(worktree.join(".redkiln")))
        .with_row_inputs(row_inputs))
}

/// **The advisory databases `sca_scan` reads, at the tools' default locations.**
///
/// R8 = ADR-0179 decision 4, proposed; D21-73 item 5; ADR-0170: *"the default locations are the
/// resolver's to supply (`s8d`)"*. *(Added 2026-09-27, `s8d` build, package F2-D2173.)*
///
/// The Rust handler reads `audit_db` (the database `cargo audit` is given by `--db`) and `deny_db`
/// (the directory `cargo deny`'s generated `db-path` names), each an absolute path, because the
/// adapter reads no environment (AC-217). The defaults are each tool's own: `cargo audit`'s
/// `<CARGO_HOME>/advisory-db` and `cargo deny`'s `<CARGO_HOME>/advisory-dbs`, with `CARGO_HOME`
/// read from the captured environment, or `<home>/.cargo` when it is unset (`USERPROFILE` on
/// Windows, `HOME` elsewhere) — cargo's own rule. Bound only for the Rust adapter. With neither
/// `CARGO_HOME` nor a home directory, nothing is bound, and the step refuses by name.
#[must_use]
pub fn advisory_db_row_inputs(
    env: &Env,
    adapter: Option<&str>,
) -> Vec<(&'static str, &'static str, serde_json::Value)> {
    if adapter.and_then(toolchain_of) != Some("rust") {
        return Vec::new();
    }
    let vars = env.as_map();
    let home_key = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    let cargo_home = vars
        .get("CARGO_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            vars.get(home_key)
                .filter(|v| !v.is_empty())
                .map(|h| Path::new(h).join(".cargo"))
        });
    let Some(cargo_home) = cargo_home else {
        return Vec::new();
    };
    let at = |dir: &str| serde_json::Value::String(cargo_home.join(dir).display().to_string());
    vec![
        ("sca_scan", "audit_db", at("advisory-db")),
        ("sca_scan", "deny_db", at("advisory-dbs")),
    ]
}

/// **R5's review routes, as `run` binds them** (ADR-0177 decision 2, proposed). *(Added 2026-09-26,
/// `s8d` build, package P7.)*
///
/// The clone's source is the repository's common directory as a `file://` URL (the run-owned
/// branch's commits are in its object store), its destinations `<common-dir>/redkiln/clones/<item>`.
/// Each re-execution arm is chosen by [`choose_toolchain`], the loop `run` chose its own toolchain
/// by, rooted in the clone (the candidate) or in the checkout (the reference).
#[must_use]
pub fn review_routes(resolved: &Resolved, env: &Env) -> crate::review_pass::ReviewCloneRoutes {
    let common_dir = resolved.common_dir.clone();
    let toolchains = resolved.toolchains.clone();
    let programs = resolved.toolchain_programs.clone();
    let timeout_secs = resolved.toolchain_timeout_secs;
    let env = env.clone();
    let for_candidates = common_dir.clone();
    crate::review_pass::ReviewCloneRoutes {
        source_url: format!(
            "file:///{}",
            common_dir
                .display()
                .to_string()
                .replace('\\', "/")
                .trim_start_matches('/')
        ),
        clones_dir: common_dir
            .join("redkiln")
            .join("clones")
            .join(resolved.item.id()),
        reference_root: Path::new(resolved.worktree.as_str()).to_path_buf(),
        toolchain_at: Box::new(move |root: &Path| {
            choose_toolchain(
                &ToolchainCandidates {
                    toolchains: &toolchains,
                    timeout_secs,
                    programs: programs.as_ref(),
                    common_dir: &for_candidates,
                },
                &env,
                root,
                &mut |line| eprintln!("redkiln: {line}"),
            )
            .map(|chosen| chosen.toolchain)
            .map_err(|exit| format!("no toolchain adapter could be chosen ({exit:?})"))
        }),
    }
}

/// The chosen adapter's value table (s8c §8c.5), or an unselected one naming why. The reasons name
/// no adapter, so the value the kernel receives carries none (AC-487).
#[must_use]
pub fn toolchain_values(
    registry: &rk_kernel::workflow::memo::VerbRegistry,
    adapter: Option<&str>,
) -> rk_kernel::workflow::memo::ToolchainValues {
    adapter.and_then(toolchain_of).map_or_else(
        || {
            rk_kernel::workflow::memo::ToolchainValues::unselected(
                "no toolchain adapter detected a project, so no value table was selected",
            )
        },
        |toolchain| {
            registry.values_for(toolchain).unwrap_or_else(|| {
                rk_kernel::workflow::memo::ToolchainValues::unselected(
                    "registry.toml declares no value table for the detected adapter's toolchain",
                )
            })
        },
    )
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
///
/// *(Restored 2026-09-22. `s7g` inserted `UndetectedToolchain` and `real_toolchain` immediately
/// above this function and anchored the insertion on `fn real_harness_config`, so this entire
/// docstring was absorbed onto the new struct: `cargo doc` rendered the AC-377 rationale as
/// `UndetectedToolchain`'s, and this function was left with no documentation at all. An adversarial
/// pass found it. Splicing above a `fn` splices into whatever docstring precedes it.)*
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

/// **R1 / D21-43 N16: one agent step's per-step bound** — *"Carried by each per-step spec (3a),
/// resolved through 4a's table and bounded by the activity budget."*
///
/// The step's own `timeout_secs`: a literal, or a `${toolchain.<key>}` resolved through the value
/// table (D21-42 4a: *"an unresolved key is refused by name"*). A step that declares none takes the
/// activation's own per-activity value, `activity_secs` (ADR-0140 decision 5's, declared in the
/// pack). Every one of the 185 shipped agent steps declares one; the fallback serves fixtures. The
/// result is `min(step, budget.wall_clock_secs)`: no per-step bound exceeds its activity's budget.
///
/// *(Added 2026-09-26, `s8d` build, package P3; record R1, proposed. R1 supersedes ADR-0140
/// decision 5's "A future per-step bound may not be built by overloading it" in part, for per-step
/// specs only: an activation's own spec keeps the per-activity value.)*
///
/// # Errors
/// A sentence naming the step and why: an unresolved or `unmeasured` key, a zero bound (RT-5), or
/// an activity that declares no `budget.wall_clock_secs` to bound it by.
pub fn agent_step_timeout_secs(
    step: &rk_kernel::catalog::Step,
    values: &rk_kernel::workflow::memo::ToolchainValues,
    activity_secs: u64,
    budget_secs: Option<u64>,
) -> Result<u64, String> {
    let declared = match &step.timeout_secs {
        None => activity_secs,
        Some(rk_kernel::catalog::StepTimeout::Literal(secs)) => *secs,
        Some(rk_kernel::catalog::StepTimeout::Toolchain(key)) => values
            .resolve(key)
            .map(std::num::NonZeroU64::get)
            .map_err(|refusal| {
                format!(
                    "the `agent` step `{}`'s timeout_secs `${{toolchain.{key}}}`: {refusal}",
                    step.id
                )
            })?,
    };
    if declared == 0 {
        return Err(format!(
            "the `agent` step `{}` declares a zero timeout_secs, which no dispatch can run under \
             (RT-5)",
            step.id
        ));
    }
    let budget = budget_secs.ok_or_else(|| {
        format!(
            "the `agent` step `{}`'s activity declares no `budget.wall_clock_secs`, so its \
             per-step bound has no ceiling (D21-43 N16: bounded by the activity budget)",
            step.id
        )
    })?;
    Ok(declared.min(budget))
}

/// Step (8)'s binding pass: one `Binder::bind` per DAG node, each recorded into the input set.
///
/// # 🛑 EVERY NODE IS BOUND, WITH ONE PER-STEP SPEC PER AGENT STEP (R1)
///
/// *(dated 2026-09-27, `s8d` build, package R1FIX-8d; R1 = ADR-0174 decisions 1, 2 and 9, proposed.)*
/// The heading this section carried until R1, ~~ONE SPEC PER NODE, NOT PER STEP — THE RUNTIME
/// DISPATCHES WHOLE ACTIVITIES~~, is false and is struck. Each agent step of a node is bound to
/// its OWN spec: that step's role's card, that step's `outputs = "value:<Kind>"` schema and
/// that step's own bound (`agent_step_timeout_secs`), carried on `RecordedSpec::step_specs`
/// beside the activation's spec, which is the first agent step's. A node with no agent step is
/// bound as an explicit absence: no activation spec and an empty per-step map, never a
/// synthesised card. The paragraph below describes the binder before R1 and is kept as its
/// history; its *"exactly one binding whatever its steps' kinds are"* and *"the role is
/// whichever of the activity's steps nominates one"* no longer hold.
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
    values: &rk_kernel::workflow::memo::ToolchainValues,
    questions: &rk_kernel::workflow::question::QuestionRegistry,
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
        let activation = activity
            .id_at_version()
            .unwrap_or_else(|| activity.id.clone());

        // *(2026-09-26, `s8d` build, package P7; R5 = ADR-0177 decision 8, proposed.)* A `gate` step's
        // kind is its `question` (ADR-0145); one the pack's `.redkiln/processes/` never declares
        // would be answerable only as the built-in default question, which is not what the activity
        // asks. Refused by name here, before anything is bound or dispatched.
        if let Some((step, question)) = crate::processes::undeclared_gate_kind(questions, activity)
        {
            eprintln!(
                "redkiln: the `gate` step `{step}` of `{}` asks the question `{question}`, which no \
                 file under `.redkiln/processes/` declares; a gate is answered with the question \
                 its pack declares, never the built-in default (R5 = ADR-0177)",
                activity.id
            );
            return Err(Exit::ConfigError);
        }

        // 🛑 **R1 (`s8d` §8d.2, §8d.8 defect 4): EVERY NODE IS BOUND, AND ONE SPEC PER AGENT STEP.**
        // This loop took the role from whichever step first nominated one and `continue`d past a
        // node with none, so a node with NO agent step was never bound and the run's supplier then
        // refused it — six of the fix lane's fifteen selected activities. Such a node is now bound
        // as an EXPLICIT ABSENCE of agent specs (an empty per-step map, no activation spec), never a
        // synthesised card (ADR-0143's forbids).
        let agents: Vec<&rk_kernel::catalog::Step> = activity
            .steps
            .iter()
            .filter(|s| s.kind == rk_kernel::catalog::StepKind::Agent)
            .collect();
        if agents.is_empty() {
            inputs = inputs.with_spec(
                &node.id,
                rk_kernel::replay::RecordedSpec::with_steps_and_no_agent_step(
                    &activation,
                    activity.steps.clone(),
                ),
            );
            continue;
        }

        let activity_secs = resolved
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
        let budget_secs = activity.budget.as_ref().map(|b| b.wall_clock_secs.get());

        // Each agent step's OWN spec: its role's card, its own `outputs = "value:<Kind>"` schema —
        // never `activity.provides.first()`, which is wrong for every agent step whose output is
        // not `provides[0]` (`tdd`'s `red` emits `ImplReport`; its `provides[0]` is `Changeset`)
        // — and its own bound (N16).
        let mut step_specs: std::collections::BTreeMap<String, rk_types::activity::ActivitySpec> =
            std::collections::BTreeMap::new();
        for step in &agents {
            let Some(role_name) = step.role.clone() else {
                eprintln!(
                    "redkiln: the `agent` step `{}` of `{}` nominates no role, so it has no card \
                     (ADR-0014: an activity nominates a role, never a card)",
                    step.id, activity.id
                );
                return Err(Exit::ConfigError);
            };
            let Some(kind) = step.output_type.clone() else {
                eprintln!(
                    "redkiln: the `agent` step `{}` of `{}` declares no `outputs = \"value:<Kind>\"`, \
                     so its dispatch has no schema to validate against",
                    step.id, activity.id
                );
                return Err(Exit::ConfigError);
            };
            let timeout_secs = agent_step_timeout_secs(step, values, activity_secs, budget_secs)
                .map_err(|e| {
                    eprintln!("redkiln: `{}`: {e}", activity.id);
                    Exit::ConfigError
                })?;
            let spec = bind_one(
                resolved,
                binder,
                owned,
                &root,
                &node.id,
                (&role_name, &kind, timeout_secs),
            )?;
            step_specs.insert(step.id.clone(), spec);
        }

        // The activation's OWN spec (ADR-0140 decision 5's per-activity bound): the first agent
        // step's card and schema, at the pack's per-activity `timeout_secs`. The walk dispatches
        // each agent step under its per-step spec; this one is what the record's `card:` line and a
        // pre-R1 reader see.
        let first = agents[0];
        let node_spec = bind_one(
            resolved,
            binder,
            owned,
            &root,
            &node.id,
            (
                first.role.as_deref().unwrap_or_default(),
                first.output_type.as_deref().unwrap_or_default(),
                activity_secs,
            ),
        )?;
        // **ADR-0138: the step list is TRANSCRIBED here, by the one place that holds the activity**,
        // and R1's per-step specs beside it (D21-42 3a: *"Per-step specs are carried on
        // `RecordedSpec` (ADR-0138's warrant)"*). `with_steps`, never `supplied`: the production
        // path always transcribes.
        inputs = inputs.with_spec(
            &node.id,
            rk_kernel::replay::RecordedSpec::with_steps(
                &activation,
                node_spec,
                activity.steps.clone(),
            )
            .and_step_specs(step_specs),
        );
    }
    Ok(inputs)
}

/// **One `Binder::bind`**: a role's card, a kind's schema, a bound — the three things a spec binds
/// that differ between agent steps (R1). `(role, kind, timeout_secs)`.
fn bind_one(
    resolved: &Resolved,
    binder: &mut rk_adapter_claude::binder::Binder,
    owned: &Path,
    root: &Path,
    node_id: &str,
    (role_name, kind, timeout_secs): (&str, &str, u64),
) -> Result<rk_types::activity::ActivitySpec, Exit> {
    let card_path = root
        .join(".redkiln")
        .join("cards")
        .join(format!("{role_name}.json"));
    let card_text = std::fs::read_to_string(&card_path).map_err(|e| {
        eprintln!("redkiln: {}: {e}", card_path.display());
        Exit::ConfigError
    })?;
    let card =
        rk_adapter_claude::card::AuthoredCard::from_file(role_name, &card_text).map_err(|e| {
            eprintln!("redkiln: {}: {e}", card_path.display());
            Exit::ConfigError
        })?;
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
    // `Role { model, effort }` is THE PACK'S role table, read through `PackExtras`. The card may not
    // carry a tier (`ConfigError::TierLeak`), so this row is its only source.
    let declared = resolved.extras.role.get(role_name).ok_or_else(|| {
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
    binder
        .bind(rk_adapter_claude::binder::BindRequest {
            role_name,
            role: &role,
            card: &card,
            schema: &rk_adapter_claude::binder::Schema {
                id: rk_types::typed::SchemaId::new(kind),
                json: schema_json,
            },
            timeout_secs,
            worktree: owned,
            inputs: vec![("item".to_owned(), serde_json::json!(resolved.item.id()))],
        })
        .map_err(|e| {
            eprintln!("redkiln: the binder refused `{node_id}`: {e}");
            Exit::ConfigError
        })
}
