//! **`s8e` §8e.11 — the first end-to-end tests of `verbs::run::run`** (D21-42 24c; D21-40
//! decision 2; ADR-0161's *"Describing the lane loop as covered by tests, or reading a green suite
//! as evidence about it."*). *(Added 2026-09-28, `s8e` build, package S8E-P2.)*
//!
//! Every test here calls **`rk_cli::verbs::run::run(`** by that name, through `Startup`'s harness
//! seam: [`ScriptedFactory`] hands each stage of the lane a `rk_ports::fakes::FakeHarness::scripted`
//! harness, so no test builds `ClaudeHarness` and none can dispatch to a real `claude`. The two
//! arms that need a PROCESS (the process-boundary test, and the stdout half of the terminal-read
//! test) drive the real `redkiln` binary over the fixture child `rk-fixture-child-claude` after an
//! in-process first hop. *(dated 2026-09-28, `s8e` build, package R3FIX-8e: the process-boundary
//! test's first hop is the binary too, as §8e.11 prescribes; only the terminal-read arms open their
//! gate in process.)*
//!
//! **The fixture lane** is `pack-min`'s `Lane` variant (`pack.lane.toml`): `triage` (empty),
//! `discovery` (`forbidden`), `implement` (the abstention gate over an item that omits `has_repro`)
//! and `review` (`review-min`). Its items (`lane/items/`) each declare a `slice`, and the fixture's
//! `config.toml` declares `[project] name = "rk-fixture"`.
//!
//! 🛑 **Most of these are RED at the stage's build by design** (`s8e` §8e.12's row: *"`verbs::run::run(`
//! has no FakeHarness seam"*). They were written before the seam, and they then stay red on the
//! behaviours §8e.3 to §8e.10 build: the stage-bearing `Story:` mark, the terminal `closed` mark,
//! `stage_empty`, the widened `Gate:` value, `t1_gate_transition`, the heal, `ByItem`, the removal
//! trigger. Three are green once the seam exists: `a_second_invocation_reuses_the_run_worktree`
//! (`s8d-ii` built reuse), and the two receiver-note tests that kill MF3j and the review-routes
//! mutant (`run()`'s `with_run_branch` and `with_review_routes` bindings).
//!
//! **What an in-process run cannot show.** `run`'s stdout is the test harness's captured stream, so
//! an in-process test cannot read *"is open"*. The two claims about stdout (the terminal read's) are
//! read off the binary in `terminal_read_reports_only_an_awaited_gate`; every in-process test reads
//! git (with git's own `%(trailers)` parser), the store, and the filesystem.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use rk_cli::exit::Exit;
use rk_cli::verbs::{HarnessFactory, Startup};
use rk_ports::cancel::CancelToken;
use rk_ports::event_store::EventStore;
use rk_ports::fakes::FakeHarness;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::ActivitySpec;
use rk_types::error::StepError;
use rk_types::event::Event;
use rk_types::event_kind::EventKind;
use rk_types::typed::{SchemaId, Typed};

use crate::support::{self, PackVariant};

// *(dated 2026-09-28, `s8e` build, package S8E-P7.)* The lane seam and its helpers below are
// `pub(crate)`: `doctor::ac184_rebuild_differential_over_anchored_tier_one_rows` runs the same
// fixture lane before it deletes the store (`s8e` §8e.9).
// *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E.)* More of them are `pub(crate)` now
// (the item constants, `ScriptedFactory::over` and `dispatches`, the two scripted elements and the
// git and store readers): `landing`'s end-to-end tests drive the same lane over the same seam.

/// `pack-min/config.toml`'s `[project] name` (ADR-0159).
pub(crate) const PROJECT: &str = "rk-fixture";
/// `lane/items/lane-gate.toml`: omits `has_repro`, so `implement` opens the abstention gate.
pub(crate) const GATE_ITEM: &str = "lane-1";
/// `lane/items/lane-through.toml`: declares `has_repro`, so the lane walks to its end.
pub(crate) const THROUGH_ITEM: &str = "lane-2";
/// `lane/items/lane-dirty.toml`: `lane-2`'s twin, for the removal trigger's dirty arm.
pub(crate) const DIRTY_ITEM: &str = "lane-3";
/// The shared gate item of `pack-min`'s single-stage variants (`items/gate-item.toml`).
const BASE_GATE_ITEM: &str = "gate-1";

// ------------------------------------------------------------------------------- the seam

/// A harness that counts its dispatches and delegates to the one it wraps.
struct Counting {
    inner: Box<dyn Harness>,
    dispatches: AtomicUsize,
}

impl Harness for Counting {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        self.inner.dispatch(spec, store)
    }
}

/// **The test side of `Startup::harness`** — a factory over ONE scripted harness for the whole
/// invocation. `run`'s lane loop builds a harness per stage; this hands every stage the same
/// `FakeHarness`, so the script is consumed front to back across the lane.
pub(crate) struct ScriptedFactory {
    harness: Arc<Counting>,
    builds: AtomicUsize,
}

impl ScriptedFactory {
    pub(crate) fn new(script: Vec<Result<Typed, StepError>>) -> Self {
        Self::over(Box::new(FakeHarness::scripted(script)))
    }

    pub(crate) fn over(inner: Box<dyn Harness>) -> Self {
        Self {
            harness: Arc::new(Counting {
                inner,
                dispatches: AtomicUsize::new(0),
            }),
            builds: AtomicUsize::new(0),
        }
    }

    fn builds(&self) -> usize {
        self.builds.load(Ordering::SeqCst)
    }

    pub(crate) fn dispatches(&self) -> usize {
        self.harness.dispatches.load(Ordering::SeqCst)
    }
}

impl HarnessFactory for ScriptedFactory {
    fn build(
        &self,
        _config: rk_adapter_claude::config::Config,
        _token: CancelToken,
    ) -> Result<Arc<dyn Harness>, String> {
        self.builds.fetch_add(1, Ordering::SeqCst);
        Ok(Arc::clone(&self.harness) as Arc<dyn Harness>)
    }
}

/// `implement`'s one dispatch (`emit-report`, `value:ImplReport`).
pub(crate) fn implement() -> Result<Typed, StepError> {
    Ok(support::impl_report())
}

/// `review`'s one dispatch (`review-min`, `value:Findings`): an EMPTY finding set, so nothing
/// blocks and the review stage's outcome is `approved`.
pub(crate) fn review() -> Result<Typed, StepError> {
    Ok(Typed::new(
        SchemaId::new("Findings"),
        serde_json::json!({ "items": [] }),
    ))
}

/// Everything a lane past its gate dispatches: `implement`, then `review`.
pub(crate) fn through() -> Vec<Result<Typed, StepError>> {
    vec![implement(), review()]
}

/// The command line `rk_cli::verbs::run::run(` is handed: `-C <root> run <item>`.
pub(crate) fn cli(root: &Path, item: &str) -> rk_cli::cli::Cli {
    rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Run(args(item)),
    }
}

/// `run <item>`, with the lane derived from the item record.
pub(crate) fn args(item: &str) -> rk_cli::cli::RunArgs {
    rk_cli::cli::RunArgs {
        item: item.to_owned(),
        lane: None,
    }
}

/// `Startup::real()` with its harness seam over `factory` (`s8e` §8e.11).
pub(crate) fn seam(factory: &dyn HarnessFactory) -> Startup<'_> {
    Startup {
        harness: factory,
        ..Startup::real()
    }
}

/// The exit a `run` returned; a halt fails the test by name.
pub(crate) fn ran(result: Result<Exit, rk_cli::exit::Halt>, item: &str) -> Exit {
    result.unwrap_or_else(|halt| panic!("`run {item}` halted: {halt:?}"))
}

/// `answer <item> <option> --explain-sufficed yes --toml-opened no`, in process.
pub(crate) fn answer_item(root: &Path, item: &str, option: &str) -> Exit {
    let cli = rk_cli::cli::Cli {
        cwd: Some(root.to_path_buf()),
        command: rk_cli::cli::Command::Status,
    };
    let args = rk_cli::cli::AnswerArgs {
        item: item.to_owned(),
        option: option.to_owned(),
        attach: Vec::new(),
        explain_sufficed: rk_cli::cli::YesNo::Yes,
        toml_opened: rk_cli::cli::YesNo::No,
    };
    rk_cli::verbs::answer::run(&cli, &args, &support::clean_env(), Startup::real())
        .unwrap_or_else(|halt| panic!("`answer {item}` halted: {halt:?}"))
}

pub(crate) fn lane_repo() -> tempfile::TempDir {
    support::pack_min_repo(PackVariant::Lane)
}

/// **The fixture lane walked to its end THROUGH A VOID** (`s8e` §8e.9's *"gate (every state)"*).
/// *(Added 2026-09-28, `s8e` build, package R1FIX-8e.)*
///
/// Invocation 1 opens `lane-1`'s gate (G1). A tracked file in the run-owned tree is then changed
/// (uncommitted), so the tree `answer` hashes has moved: the answer is VOID (AC-406; exit 0; its
/// `Gate: … answered` and `Gate: … voided` marks). The file is restored, so the run tree's working
/// state equals the one the next gate binds to again (the tree mismatch `s8f` owns, §8e.9's defect
/// (b), is thereby not reached). Invocation 2 opens the fresh gate AC-252 makes the kernel's next
/// act (G2); `answer` must present G2 — the AWAITED gate — and not the spent G1, which it did
/// about half the time before package R1FIX-8e (§8e.9's defect (a)); the answer applies, and
/// invocation 3 walks the lane to its `closed` mark. Every step's exit is asserted, so a lane
/// that stalls at `RefusedSpent` fails here by name.
pub(crate) fn walk_the_lane_through_a_void(root: &Path) {
    let run = |script| {
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(script)),
            ),
            GATE_ITEM,
        )
    };
    assert_eq!(run(Vec::new()), Exit::Success, "PREMISE: gate G1 opens");

    let tree = crate::run_tree(root, GATE_ITEM).path;
    let tracked = git_read(&tree, &["ls-files"])
        .and_then(|out| out.lines().next().map(str::to_owned))
        .expect("PREMISE: the run tree tracks a file");
    let file = tree.join(&tracked);
    let original = std::fs::read(&file).expect("the tracked file reads");
    let mut moved = original.clone();
    moved.extend_from_slice(b"moved before the answer\n");
    std::fs::write(&file, &moved).expect("the run tree moves");
    assert_eq!(
        answer_item(root, GATE_ITEM, "approve"),
        Exit::Success,
        "a void answer is an outcome, exit 0 (AC-406)"
    );
    std::fs::write(&file, &original).expect("the run tree is restored");
    assert_eq!(
        support::store_rows(root)
            .iter()
            .filter(|e| e.kind == EventKind::GateAnswerVoid)
            .count(),
        1,
        "PREMISE: the answer over the moved tree was VOID"
    );

    assert_eq!(
        run(Vec::new()),
        Exit::Success,
        "PREMISE: the fresh gate G2 opens after the void (AC-252)"
    );
    assert_eq!(
        answer_item(root, GATE_ITEM, "approve"),
        Exit::Success,
        "`answer` must present the AWAITED gate G2 and apply it, not the spent G1 \
         (`RefusedSpent`, exit 3 — ADR-0157's stall)"
    );
    assert_eq!(
        run(through()),
        Exit::Success,
        "PREMISE: the lane walks to its end past the void"
    );
}

// ------------------------------------------------------------------------------- git, read

/// `git <args>` in `cwd`; `Some(stdout)` on success, `None` when git refuses (an absent ref).
/// Through `rk-git`'s one process-exec site (`support::git_run`; ADR-0077; D21-128 A1). A git
/// that cannot start at all panics, as `expect("git runs")` did here before the routing.
pub(crate) fn git_read(cwd: &Path, args: &[&str]) -> Option<String> {
    match crate::support::git_run(cwd, args) {
        Ok(out) => Some(String::from_utf8_lossy(&out.stdout).into_owned()),
        Err(rk_git::error::GitError::NonZeroExit { .. }) => None,
        Err(e) => panic!("git {args:?} did not run: {e:?}"),
    }
}

/// `git <args>` in `cwd`, its exit code (`support::git_code`; D21-128 A1/A2).
pub(crate) fn git_rc(cwd: &Path, args: &[&str]) -> Option<i32> {
    crate::support::git_code(cwd, args)
}

pub(crate) fn run_ref(item: &str) -> String {
    format!("refs/heads/rk-run/{item}")
}

/// Every `<key>:` trailer value on `rk-run/<item>`, oldest commit first, read by git's own
/// `%(trailers)` parser. An absent ref reads as no values.
fn trailer_values(root: &Path, item: &str, key: &str) -> Vec<String> {
    let format = format!("--format=%(trailers:key={key},valueonly)");
    git_read(root, &["log", "--reverse", &format, &run_ref(item)])
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The stage token of every `Story:` mark for `item` on its run ref, oldest first. A legacy
/// stage-less mark reads `<stage-less>`, and a value of three or more tokens reads as itself, so a
/// comparison against a stage list fails on either rather than skipping it.
pub(crate) fn story_stages(root: &Path, item: &str) -> Vec<String> {
    let qualified = format!("{PROJECT}/{item}");
    trailer_values(root, item, "Story")
        .into_iter()
        .filter(|v| v.split_whitespace().next() == Some(qualified.as_str()))
        .map(|v| {
            let tokens: Vec<&str> = v.split_whitespace().collect();
            match tokens.as_slice() {
                [_] => "<stage-less>".to_owned(),
                [_, stage] => (*stage).to_owned(),
                _ => v.clone(),
            }
        })
        .collect()
}

/// Every `Gate:` mark on `item`'s run ref, as its whitespace tokens, oldest first.
pub(crate) fn gate_marks(root: &Path, item: &str) -> Vec<Vec<String>> {
    trailer_values(root, item, "Gate")
        .into_iter()
        .map(|v| v.split_whitespace().map(str::to_owned).collect())
        .collect()
}

pub(crate) fn ref_sha(root: &Path, item: &str) -> Option<String> {
    git_read(root, &["rev-parse", "--verify", "--quiet", &run_ref(item)])
        .map(|s| s.trim().to_owned())
}

// ----------------------------------------------------------------------------- store, read

/// The tokens of every ANSWERABLE `gate_opened` row (the runtime's, which carries a token —
/// `answer::is_answerable_gate`'s rule), in journal order.
fn answerable_tokens(rows: &[Event]) -> Vec<String> {
    rows.iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter_map(|e| e.payload.get("token").and_then(serde_json::Value::as_str))
        .map(str::to_owned)
        .collect()
}

/// Rows whose kind's wire name is `wire` — by NAME, so a kind this tree does not mint yet is read
/// as absent rather than failing to compile.
pub(crate) fn rows_named<'a>(rows: &'a [Event], wire: &str) -> Vec<&'a Event> {
    rows.iter().filter(|e| e.kind.wire_name() == wire).collect()
}

fn store_file(root: &Path) -> PathBuf {
    crate::common_dir(root)
        .join("redkiln")
        .join("store.sqlite3")
}

/// Delete the store and its WAL and SHM — the deleted-database test's act (AC-161).
fn delete_store(root: &Path) {
    let base = store_file(root);
    assert!(
        base.is_file(),
        "PREMISE: no store at {} to delete",
        base.display()
    );
    for suffix in ["", "-wal", "-shm"] {
        let p = PathBuf::from(format!("{}{suffix}", base.display()));
        if p.exists() {
            std::fs::remove_file(&p).unwrap_or_else(|e| panic!("{} deletes: {e}", p.display()));
        }
    }
    assert!(!base.exists(), "the store file survived its deletion");
}

/// Read-only SQL over the store under `common_dir`, every column as text (the query CASTs).
pub(crate) fn sql_rows(common_dir: &Path, sql: &str) -> Result<Vec<Vec<Option<String>>>, String> {
    let reader = rk_store::conn::Reader::open(common_dir).map_err(|e| format!("{e:?}"))?;
    let conn = reader.conn();
    let mut stmt = conn.prepare(sql).map_err(|e| format!("`{sql}`: {e}"))?;
    let n = stmt.column_count();
    stmt.query_map([], |r| {
        (0..n)
            .map(|i| r.get::<_, Option<String>>(i))
            .collect::<Result<Vec<_>, _>>()
    })
    .map_err(|e| format!("`{sql}`: {e}"))?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| format!("`{sql}`: {e}"))
}

fn runs_dir(root: &Path) -> PathBuf {
    crate::common_dir(root).join("redkiln").join("runs")
}

/// The item's journal through the store's own `ByItem` read — the read every one-item fold takes.
fn journal_of(root: &Path, item: &str) -> Vec<Event> {
    let repo = rk_cli::repo::discover(Some(root))
        .unwrap_or_else(|e| panic!("the repository under test discovers: {e}"));
    let store = rk_adapter_sqlite::SqliteStore::open(&repo.exec, &repo.worktree)
        .unwrap_or_else(|e| panic!("the store opens: {e}"));
    store
        .query(rk_kernel::runtime::journal_query(item))
        .unwrap_or_else(|e| panic!("the journal reads: {e:?}"))
}

/// The pack's lane, as the lane must walk it: `stage_order` without its `forbidden` stages,
/// computed from the INSTALLED pack's own bytes (no pack literal).
fn declared_lane(root: &Path) -> Vec<String> {
    let src = std::fs::read_to_string(
        root.join(".redkiln")
            .join("packs")
            .join("bug-standard.toml"),
    )
    .expect("the installed pack reads");
    let pack = rk_kernel::select::Pack::from_toml("bug-standard.toml", &src)
        .unwrap_or_else(|e| panic!("the installed pack loads: {e}"));
    pack.stage_order()
        .iter()
        .filter(|s| pack.rigor(s) != Some(rk_kernel::rigor::Rigor::Forbidden))
        .cloned()
        .collect()
}

// ------------------------------------------------------------------------------- the tests

/// **Open, answer, re-run: one opening and one answer in git, and no second gate.**
///
/// Invocation 1 opens `implement`'s abstention gate and exits 0; `answer` applies; the re-run walks
/// on. Git's own parser then counts exactly one `opened` and one `answered` state for the token on
/// `rk-run/<item>` (§8e.7's widened value, `<token> <tree_hash> <deadline> <kind> <reach> <state>`),
/// the store holds exactly one answerable opening, and the token's `opened` row in
/// `t1_gate_transition` carries the opening stage run's `stage` and `run_id` (§8e.7; S8E-M47).
///
/// The re-run's stdout (*"reports no open gate"*) is not readable in process; the binary arm of
/// `terminal_read_reports_only_an_awaited_gate` reads it.
#[test]
fn run_opens_a_gate_answer_applies_and_a_rerun_mints_no_second_gate_trailer() {
    let repo = lane_repo();
    let root = repo.path();

    let opening = ScriptedFactory::new(Vec::new());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&opening)
            ),
            GATE_ITEM
        ),
        Exit::Success,
        "invocation 1 exits 0 at the gate (golden constraint 2)"
    );
    let tokens = answerable_tokens(&support::store_rows(root));
    assert_eq!(
        tokens.len(),
        1,
        "PREMISE: exactly one answerable gate is open after invocation 1: {tokens:?}"
    );
    let token = tokens[0].clone();

    assert_eq!(
        answer_item(root, GATE_ITEM, "approve"),
        Exit::Success,
        "the answer applies"
    );

    let rerun = ScriptedFactory::new(through());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&rerun)
            ),
            GATE_ITEM
        ),
        Exit::Success,
        "the re-run walks on past the answered gate"
    );
    assert!(
        rerun.builds() >= 1 && rerun.dispatches() >= 1,
        "the re-run dispatched nothing ({} harness builds): the answered gate still held it, or \
         the lane re-seeded from the item record and stalled (ADR-0157)",
        rerun.builds()
    );

    let marks = gate_marks(root, GATE_ITEM);
    let of_token: Vec<&Vec<String>> = marks.iter().filter(|m| m.first() == Some(&token)).collect();
    let in_state = |state: &str| {
        of_token
            .iter()
            // *(Re-pointed 2026-09-28, `s8e` build, package S8E-P4; ADR-0184, proposed.)* The state
            // is the SIXTH token, not the last: an `answered` mark carries its option after it
            // (~~`m.len() == 6 && m[5] == state`~~).
            .filter(|m| m.len() >= 6 && m[5] == state)
            .count()
    };
    assert_eq!(
        (in_state("opened"), in_state("answered"), of_token.len()),
        (1, 1, 2),
        "git's parser must count exactly one `opened` and one `answered` Gate state for {token} \
         on rk-run/{GATE_ITEM}, and no other mark naming it (§8e.7): {of_token:?}"
    );
    assert_eq!(
        answerable_tokens(&support::store_rows(root)),
        vec![token.clone()],
        "the re-run minted a second gate"
    );

    let opened = sql_rows(
        &crate::common_dir(root),
        &format!(
            "SELECT CAST(stage AS TEXT), CAST(run_id AS TEXT) FROM t1_gate_transition \
             WHERE token = '{token}' AND state = 'opened'"
        ),
    )
    .unwrap_or_else(|e| panic!("the token's `opened` transition row is not readable: {e}"));
    assert_eq!(
        opened.len(),
        1,
        "exactly one `opened` row for the token: {opened:?}"
    );
    assert_eq!(
        opened[0][0].as_deref(),
        Some("implement"),
        "the `opened` row carries the opening run's stage"
    );
    let run_id = opened[0][1].clone().unwrap_or_default();
    let digest = run_id.strip_prefix("implement-").unwrap_or_default();
    assert!(
        digest.len() == 16 && digest.bytes().all(|b| b.is_ascii_hexdigit()),
        "the `opened` row's run_id is not `<stage>-<digest16>` (ADR-0152): {run_id:?}"
    );
    assert!(
        runs_dir(root).join(&run_id).is_dir(),
        "the `opened` row's run_id {run_id} names no run record under {}",
        runs_dir(root).display()
    );
}

/// **The stall ADR-0157 recorded, closed: a second invocation crosses the next boundary.**
///
/// Invocation 1 exits 0 at `implement`'s gate; `answer` applies; invocation 2 — a fresh `run`,
/// which opens its own store — seeds its stage from git, crosses `implement -> review`, and mints
/// `review`'s mark. A two-hop walk, because a one-hop fixture cannot judge a multi-hop guard
/// (ADR-0161). Red under S8E-M1 (the seed read from the item record).
#[test]
fn run_exits_at_a_gate_answer_and_a_second_invocation_continues_the_lane() {
    let repo = lane_repo();
    let root = repo.path();

    let first = ScriptedFactory::new(Vec::new());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&first)
            ),
            GATE_ITEM
        ),
        Exit::Success,
        "invocation 1 exits 0 at the gate"
    );
    let after_first = story_stages(root, GATE_ITEM);
    assert_eq!(
        after_first,
        vec!["triage".to_owned(), "implement".to_owned()],
        "invocation 1 marks the lane's entry and its one boundary, and no more"
    );

    assert_eq!(answer_item(root, GATE_ITEM, "approve"), Exit::Success);

    let second = ScriptedFactory::new(through());
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, GATE_ITEM),
            &args(GATE_ITEM),
            &support::clean_env(),
            seam(&second),
        ),
        GATE_ITEM,
    );
    let after_second = story_stages(root, GATE_ITEM);
    assert!(
        after_second.iter().any(|s| s == "review"),
        "invocation 2 did not cross `implement -> review` (exit {exit:?}); marks: {after_second:?}"
    );
    assert_eq!(
        after_second.iter().filter(|s| *s == "implement").count(),
        1,
        "invocation 2 re-entered `implement`: {after_second:?}"
    );
    assert_eq!(
        after_second[..after_first.len()],
        after_first[..],
        "invocation 2 rewrote the marks invocation 1 left"
    );
}

/// **Every non-`forbidden` stage is entered and trailered, in declared order, then `closed`.**
///
/// The expected list is computed from the installed pack's own `stage_order` and `stages` (no
/// literal), and read back off `rk-run/<item>` with git's parser. Red under S8E-M2.
#[test]
fn every_non_forbidden_stage_is_entered_and_trailered_in_declared_order() {
    let repo = lane_repo();
    let root = repo.path();
    let mut expected = declared_lane(root);
    assert!(
        expected.len() >= 3 && !expected.iter().any(|s| s == "discovery"),
        "PREMISE: the fixture lane must admit several stages and skip its forbidden one: \
         {expected:?}"
    );
    expected.push("closed".to_owned());

    let factory = ScriptedFactory::new(through());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&factory)
            ),
            THROUGH_ITEM
        ),
        Exit::Success,
        "the lane walks to its end"
    );
    assert_eq!(
        story_stages(root, THROUGH_ITEM),
        expected,
        "the stage marks on rk-run/{THROUGH_ITEM} are not the pack's filtered order followed by \
         `closed`"
    );
}

/// **An empty stage is entered, trailered and passed, and records `stage_empty { stage, reason }`.**
///
/// `triage` carries no activity in the fixture lane. Red under S8E-M14; *(dated 2026-09-28, `s8e`
/// build, package R3FIX-8e)* and under S8E-M60 (the row appended beside the entry act, unanchored)
/// and S8E-M61 (a reason outside `STAGE_EMPTY_REASONS`).
#[test]
fn an_empty_stage_records_stage_empty_with_its_reason() {
    let repo = lane_repo();
    let root = repo.path();
    let factory = ScriptedFactory::new(through());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&factory)
            ),
            THROUGH_ITEM
        ),
        Exit::Success
    );

    let rows = support::store_rows(root);
    let empty = rows_named(&rows, "stage_empty");
    assert_eq!(
        empty.len(),
        1,
        "exactly one `stage_empty` row, for the one empty stage; kinds seen: {:?}",
        rows.iter().map(|e| e.kind.wire_name()).collect::<Vec<_>>()
    );
    assert_eq!(
        empty[0]
            .payload
            .get("stage")
            .and_then(serde_json::Value::as_str),
        Some("triage"),
        "the row names the empty stage: {}",
        empty[0].payload
    );
    // *(dated 2026-09-28, `s8e` build, package R3FIX-8e; refuting round 3's major.)* The reason is
    // a MEMBER of the closed set the record fixes (§8e.6: *"a closed set that the record fixes"*),
    // not any non-empty string (~~`is_some_and(|r| !r.trim().is_empty())`~~). Red under S8E-M61.
    let reason = empty[0]
        .payload
        .get("reason")
        .and_then(serde_json::Value::as_str);
    assert!(
        reason.is_some_and(|r| rk_types::event_kind::STAGE_EMPTY_REASONS.contains(&r)),
        "the row's reason {reason:?} is not one of the closed set {:?}: {}",
        rk_types::event_kind::STAGE_EMPTY_REASONS,
        empty[0].payload
    );
    assert!(
        story_stages(root, THROUGH_ITEM)
            .iter()
            .any(|s| s == "triage"),
        "the empty stage was skipped rather than entered and trailered"
    );

    // *(dated 2026-09-28, `s8e` build, package R3FIX-8e; refuting round 3's major.)* **The row is
    // anchored to the entry's commit** (§8e.6: *"On the entry `resume_routed` takes itself (the
    // lane's first stage), the row is a second row of the entry act, anchored to its commit."*).
    // `triage` is the fixture lane's FIRST stage, so its entry is that act: the row's `anchor_sha`
    // must be the sha of the one `Story: rk-fixture/lane-2 triage` commit on the run ref. A row
    // appended beside the act instead leaves `anchor_sha` NULL. Red under S8E-M60.
    let anchors = sql_rows(
        &crate::common_dir(root),
        "SELECT CAST(anchor_sha AS TEXT) FROM t2_event WHERE kind = 'stage_empty'",
    )
    .unwrap_or_else(|e| panic!("PREMISE: the store must be readable: {e}"));
    assert_eq!(
        anchors.len(),
        1,
        "PREMISE: one `stage_empty` row in t2_event"
    );
    let anchor = anchors[0][0].clone();
    let entry = format!("{PROJECT}/{THROUGH_ITEM} triage");
    let entry_commits: Vec<String> = git_read(
        root,
        &[
            "log",
            "--format=%H%x1f%(trailers:key=Story,valueonly,separator=%x1e)",
            &run_ref(THROUGH_ITEM),
        ],
    )
    .unwrap_or_default()
    .lines()
    .filter_map(|l| l.split_once('\u{1f}'))
    .filter(|(_, values)| values.split('\u{1e}').any(|v| v.trim() == entry))
    .map(|(sha, _)| sha.trim().to_owned())
    .collect();
    assert_eq!(
        entry_commits.len(),
        1,
        "PREMISE: exactly one `Story: {entry}` commit on rk-run/{THROUGH_ITEM}: {entry_commits:?}"
    );
    assert_eq!(
        anchor.as_deref(),
        Some(entry_commits[0].as_str()),
        "the `stage_empty` row is not anchored to the entry act's `Story: {entry}` commit \
         (section 8e.6: a second row of the entry act)"
    );
}

/// **After hop 2 the store is deleted, and the next `run` seeds its stage from git** (AC-161's own
/// comparison, at stage grain).
///
/// Hop 1 opens the gate; `answer`; hop 2 crosses `implement -> review` and stops when `review`'s
/// dispatch finds the script exhausted. The store is then deleted. Steps (1)-(7) resolve the
/// stage from the reconciled mark (`review`), not from the item record (`triage`); and the next
/// `run`, which stops at the same place, leaves a stage-level next action canonically equal to the
/// one before the deletion. Red under S8E-M1.
#[test]
fn the_stage_seeds_from_git_when_the_store_is_deleted() {
    let repo = lane_repo();
    let root = repo.path();

    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            GATE_ITEM
        ),
        Exit::Success
    );
    assert_eq!(answer_item(root, GATE_ITEM, "approve"), Exit::Success);
    let hop_two = ScriptedFactory::new(vec![implement()]);
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, GATE_ITEM),
            &args(GATE_ITEM),
            &support::clean_env(),
            seam(&hop_two),
        ),
        GATE_ITEM,
    );
    let marks = story_stages(root, GATE_ITEM);
    let git_stage = marks.last().cloned().unwrap_or_default();
    assert_eq!(
        git_stage, "review",
        "PREMISE: hop 2 must cross into `review` (exit {exit:?}); marks: {marks:?}"
    );

    let before =
        rk_kernel::workflow::fsm::stage_action(GATE_ITEM, &git_stage, &support::store_rows(root))
            .expect("the stage action folds before the deletion");

    delete_store(root);
    let seeded = support::resolved_over_pack_min(root, GATE_ITEM);
    assert_eq!(
        seeded.stage, git_stage,
        "with the store deleted, the stage must come from git's latest mark, not the item record"
    );

    let next = ScriptedFactory::new(Vec::new());
    let _ = ran(
        rk_cli::verbs::run::run(
            &cli(root, GATE_ITEM),
            &args(GATE_ITEM),
            &support::clean_env(),
            seam(&next),
        ),
        GATE_ITEM,
    );
    let after =
        rk_kernel::workflow::fsm::stage_action(GATE_ITEM, &git_stage, &support::store_rows(root))
            .expect("the stage action folds after the deletion");
    assert_eq!(
        before.canonical_bytes().expect("canonical"),
        after.canonical_bytes().expect("canonical"),
        "AC-161: the stage-level next action after the deletion is not canonically equal to the \
         one before it"
    );

    let control = rk_kernel::workflow::fsm::stage_action(GATE_ITEM, "triage", &[])
        .expect("the control folds");
    assert_ne!(
        before.canonical_bytes().expect("canonical"),
        control.canonical_bytes().expect("canonical"),
        "CONTROL: an empty journal at another stage must differ, or the equality above holds over \
         a constant"
    );
}

/// One arm of the terminal-read test: what the re-run left on the ref and printed.
struct Rerun {
    token: String,
    before: Vec<Vec<String>>,
    after: Vec<Vec<String>>,
    stdout: String,
    stderr: String,
    code: Option<i32>,
}

impl Rerun {
    /// The re-run committed no mark naming the dead token except its own transition's state, and
    /// printed no *"is open"* line naming it.
    fn assert_quiet(&self, arm: &str) {
        let opening = |marks: &[Vec<String>]| {
            marks
                .iter()
                .filter(|m| m.first() == Some(&self.token))
                .filter(|m| {
                    // *(Re-pointed 2026-09-28, S8E-P4; ADR-0184: the state is the sixth token and
                    // `answered`/`expired` carry outcome tokens after it; ~~`m.len() != 6`~~.)*
                    m.len() < 6 || !matches!(m[5].as_str(), "answered" | "expired" | "voided")
                })
                .count()
        };
        assert_eq!(
            opening(&self.after),
            opening(&self.before),
            "{arm}: the re-run committed an opening for the dead token {} (terminal_read must \
             only report, and only an awaited gate). before: {:?}\nafter: {:?}",
            self.token,
            self.before,
            self.after
        );
        let open_lines: Vec<&str> = self
            .stdout
            .lines()
            .filter(|l| l.contains("is open") && l.contains(self.token.as_str()))
            .collect();
        assert!(
            open_lines.is_empty(),
            "{arm}: the re-run printed the dead token {} as open (exit {:?}): {open_lines:?}\n\
             stderr:\n{}",
            self.token,
            self.code,
            self.stderr
        );
    }
}

/// Spawn `redkiln -C <root> run <item>` over the fixture child replaying the lane's implement
/// transcript.
#[expect(
    clippy::disallowed_types,
    reason = "the terminal read's stdout is read AS A PROCESS: an in-process run's stdout is the \
              test harness's captured stream"
)]
fn spawn_run_replaying(root: &Path, item: &str) -> std::process::Output {
    let frames = support::pack_min_dir()
        .join("lane")
        .join("implement-frames.jsonl");
    assert!(
        frames.is_file(),
        "the lane transcript {} is absent",
        frames.display()
    );
    let markers = root.join(".rk-markers");
    std::fs::create_dir_all(&markers).expect("the marker directory is created");
    let mut cmd = std::process::Command::new(crate::redkiln());
    cmd.arg("-C")
        .arg(root)
        .arg("run")
        .arg(item)
        .env("RK_FIXTURE_MODE", format!("replay:{}", frames.display()))
        .env("RK_FIXTURE_MARKER_DIR", &markers);
    support::output_bounded(&mut cmd)
}

/// Open `gate-1`'s gate in process (`open`, the test's own `verbs::run::run(` call), apply
/// `settle`, then re-run it as a process.
fn terminal_arm(
    variant: PackVariant,
    open: impl FnOnce(&Path) -> Exit,
    settle: impl FnOnce(&Path, &str),
) -> Rerun {
    let repo = support::pack_min_repo(variant);
    let root = repo.path();
    assert_eq!(
        open(root),
        Exit::Success,
        "PREMISE: the gate opens in process"
    );
    let tokens = answerable_tokens(&support::store_rows(root));
    assert_eq!(tokens.len(), 1, "PREMISE: one answerable gate: {tokens:?}");
    let token = tokens[0].clone();
    settle(root, &token);
    let before = gate_marks(root, BASE_GATE_ITEM);
    let out = spawn_run_replaying(root, BASE_GATE_ITEM);
    Rerun {
        token,
        before,
        after: gate_marks(root, BASE_GATE_ITEM),
        stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        code: out.status.code(),
    }
}

/// **`terminal_read` reports only the gate the fold awaits** (D21-42 24c; ADR-0161's fourth
/// finding): an answered gate, a void gate and an expired gate, one arm each. After each, the
/// re-run prints no *"is open"* naming the dead token and commits no opening for it. The re-run is
/// the binary, because stdout is the observation. Red under S8E-M10.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "a test-side wait past a real 1 s gate deadline, polled against the recorded deadline \
              (never a fixed sleep); the PRODUCT reads no clock here, the expiry is observed at a read"
)]
fn terminal_read_reports_only_an_awaited_gate() {
    // The first hop of every arm, in process: `gate-1` opens its abstention gate, exit 0.
    let open = |root: &Path| {
        ran(
            rk_cli::verbs::run::run(
                &cli(root, BASE_GATE_ITEM),
                &args(BASE_GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new())),
            ),
            BASE_GATE_ITEM,
        )
    };

    // ---- answered
    let answered = terminal_arm(PackVariant::Base, open, |root, _| {
        assert_eq!(
            answer_item(root, BASE_GATE_ITEM, "approve"),
            Exit::Success,
            "PREMISE: the answer applies"
        );
    });
    answered.assert_quiet("answered");

    // ---- void: the run-owned tree moves after the gate opened, so the answer is void (AC-406).
    let void = terminal_arm(PackVariant::Base, open, |root, token| {
        let owned = crate::run_tree(root, BASE_GATE_ITEM).path;
        std::fs::write(
            owned.join("moved.txt"),
            b"the tree moved after the gate opened\n",
        )
        .expect("the run-owned tree is writable");
        crate::git(&owned, &["add", "moved.txt"]);
        crate::git(&owned, &["commit", "-m", "move the run-owned tree"]);
        assert_eq!(answer_item(root, BASE_GATE_ITEM, "approve"), Exit::Success);
        let rows = support::store_rows(root);
        assert!(
            rows_named(&rows, "gate_answer_void")
                .iter()
                .any(|e| e.payload.to_string().contains(token)),
            "PREMISE: the answer must be void for {token}"
        );
    });
    void.assert_quiet("void");

    // ---- expired: the 1 s deadline passes, and the re-run's expiry pass resolves it (AC-257).
    let expired = terminal_arm(PackVariant::GateOneSecond, open, |root, token| {
        let deadline = support::store_rows(root)
            .iter()
            .filter(|e| e.kind == EventKind::GateOpened)
            .filter(|e| e.payload.get("token").and_then(serde_json::Value::as_str) == Some(token))
            .find_map(|e| {
                e.payload
                    .get("deadline")
                    .and_then(serde_json::Value::as_i64)
            })
            .expect("PREMISE: the gate's row records its deadline");
        let now = || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
        };
        let mut polls = 0_u32;
        while now() <= deadline {
            polls += 1;
            assert!(polls < 200, "the 1 s deadline {deadline} never passed");
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
    expired.assert_quiet("expired");
}

/// **The process boundary, with a process** (D21-47's canonical fixture harness). Invocation 1
/// (~~in process~~ *(dated 2026-09-28, `s8e` build, package R3FIX-8e)* the binary) exits 0 at
/// `implement`'s gate; `redkiln answer` applies; `redkiln run` — the real
/// binary, `RK_FIXTURE_MODE=replay:<frames>`, `harness_binary` naming the built
/// `rk-fixture-child-claude` — crosses `implement -> review` and mints `review`'s mark.
#[test]
fn the_binary_crosses_a_process_boundary_over_the_fixture_child() {
    let repo = lane_repo();
    let root = repo.path();
    // *(dated 2026-09-28, `s8e` build, package R3FIX-8e; refuting round 3's minor.)* Invocation 1
    // is the real binary too, as §8e.11 prescribes (*"`run` exits 0 at a gate, then `redkiln
    // answer`, then `redkiln run` crosses the next boundary"*), ~~in process through
    // `verbs::run::run(`~~. Its exit is read off the process.
    let first = spawn_run_replaying(root, GATE_ITEM);
    assert_eq!(
        first.status.code(),
        Some(0),
        "invocation 1 (`redkiln run`, a process) exits 0 at the gate. stderr:\n{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert_eq!(
        answerable_tokens(&support::store_rows(root)).len(),
        1,
        "PREMISE: invocation 1 opened exactly one answerable gate"
    );
    let before = story_stages(root, GATE_ITEM);

    let answered =
        crate::answer::spawn_answer(root, GATE_ITEM, "approve", &crate::answer::BOTH_FLAGS);
    assert_eq!(
        answered.status.code(),
        Some(0),
        "`redkiln answer` applies. stderr:\n{}",
        String::from_utf8_lossy(&answered.stderr)
    );

    let markers = root.join(".rk-markers");
    let marker_names = || -> std::collections::BTreeSet<std::path::PathBuf> {
        std::fs::read_dir(&markers).map_or_else(
            |_| std::collections::BTreeSet::new(),
            |dir| {
                dir.filter_map(Result::ok)
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|x| x == "marker"))
                    .collect()
            },
        )
    };
    let spawned_before = marker_names();
    let out = spawn_run_replaying(root, GATE_ITEM);
    let after = story_stages(root, GATE_ITEM);
    // *(dated 2026-09-28, `s8e` build, package R4FIX-8e; refuting round 4's minor.)* ~~`spawned_after
    // > spawned_before`~~, which `review-min`'s spawn alone satisfied: the second process spawns
    // EXACTLY two fixture children, one per dispatch, and each marker's `--json-schema` argv names
    // its stage's schema — one `ImplReport` (`implement`'s dispatch) and one `Findings`
    // (`review-min`'s). A process whose implement stage never dispatched fails here.
    let spawned: Vec<_> = marker_names()
        .difference(&spawned_before)
        .cloned()
        .collect();
    let schema_title = |marker: &std::path::Path| -> Option<String> {
        let text = std::fs::read_to_string(marker).ok()?;
        let mut lines = text.lines();
        lines.by_ref().find(|l| *l == "arg=--json-schema")?;
        let schema: serde_json::Value =
            serde_json::from_str(lines.next()?.strip_prefix("arg=")?).ok()?;
        schema.get("title")?.as_str().map(str::to_owned)
    };
    let titles: Vec<Option<String>> = spawned.iter().map(|m| schema_title(m)).collect();
    assert_eq!(
        spawned.len(),
        2,
        "the second `redkiln run` must spawn exactly two fixture children (`implement`'s \
         dispatch and `review-min`'s); it spawned {} with schema titles {titles:?} (exit {:?}). \
         stderr:\n{}",
        spawned.len(),
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    for title in ["ImplReport", "Findings"] {
        assert_eq!(
            titles
                .iter()
                .filter(|t| t.as_deref() == Some(title))
                .count(),
            1,
            "exactly one of the second process's two fixture children must carry the `{title}` \
             `--json-schema`; titles {titles:?}"
        );
    }
    assert!(
        after.iter().any(|s| s == "review") && !before.iter().any(|s| s == "review"),
        "the second process did not cross into `review` (exit {:?}); marks before {before:?}, \
         after {after:?}. stderr:\n{}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    // *(dated 2026-09-28, `s8e` build, package R3FIX-8e.)* The second process's exit is asserted.
    // It is `Schema` (11), measured: after crossing into `review`, that stage's `review-min`
    // dispatch replays the same `implement-frames.jsonl` transcript, whose output is no `Findings`,
    // so the dispatch does not commit (*"the dispatch of `review-min` (review-min@1) did not
    // commit"*). The crossing is §8e.11's claim; the exit pins where the process stopped after it,
    // so a second process that stops anywhere else (a halt, a refusal before the boundary) fails.
    assert_eq!(
        out.status.code(),
        Some(Exit::Schema as i32),
        "the second `redkiln run` must cross into `review` and stop at `review-min`'s dispatch \
         (Schema, 11). stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

/// **N9: the lane's end is a mark** — `Story: <project>/<item> closed`, once, last. Red under
/// S8E-M28.
#[test]
fn the_lane_end_mints_a_terminal_closed_mark() {
    let repo = lane_repo();
    let root = repo.path();
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(through()))
            ),
            THROUGH_ITEM
        ),
        Exit::Success
    );
    let values = trailer_values(root, THROUGH_ITEM, "Story");
    let closed = format!("{PROJECT}/{THROUGH_ITEM} closed");
    assert_eq!(
        values.iter().filter(|v| **v == closed).count(),
        1,
        "exactly one terminal `{closed}` mark: {values:?}"
    );
    assert_eq!(
        values.last(),
        Some(&closed),
        "the terminal mark is not the ref's last `Story:` value: {values:?}"
    );
}

/// **N9: a closed item re-runs as Done, exits 0, and mints nothing** — no commit on its run ref, no
/// dispatch, no tier-1 row. That exit 0 is Done read from git, not ADR-0157's stall.
#[test]
fn a_closed_item_reruns_as_done_and_mints_nothing() {
    let repo = lane_repo();
    let root = repo.path();
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(through()))
            ),
            THROUGH_ITEM
        ),
        Exit::Success
    );
    assert_eq!(
        story_stages(root, THROUGH_ITEM).last().map(String::as_str),
        Some("closed"),
        "PREMISE: the lane is closed"
    );
    let tip = ref_sha(root, THROUGH_ITEM);
    let tier_one = |rows: &[Event]| {
        rows.iter()
            .filter(|e| e.kind.tier() == rk_types::event_kind::Tier::One)
            .count()
    };
    let before = tier_one(&support::store_rows(root));

    let rerun = ScriptedFactory::new(Vec::new());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&rerun)
            ),
            THROUGH_ITEM
        ),
        Exit::Success,
        "a closed item re-runs as Done, exit 0"
    );
    assert_eq!(rerun.dispatches(), 0, "a closed item dispatched");
    assert_eq!(
        ref_sha(root, THROUGH_ITEM),
        tip,
        "a closed item's re-run committed on its run ref"
    );
    assert_eq!(
        tier_one(&support::store_rows(root)),
        before,
        "a closed item's re-run wrote a tier-1 row"
    );
}

/// **N6 / AC-161: an answered gate survives a deleted store as ANSWERED.**
///
/// Open, answer, delete the store. The next `run` does not stop at the gate (the answer is read
/// back from git's `answered` state), and the stage-level next action at `implement` is canonically
/// equal to the one before the deletion. Red under S8E-M27 (the answer written with state
/// `opened`, which a deleted store then reads as open).
#[test]
fn an_answered_gate_survives_a_deleted_store_as_answered() {
    let repo = lane_repo();
    let root = repo.path();
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            GATE_ITEM
        ),
        Exit::Success
    );
    let token = answerable_tokens(&support::store_rows(root))
        .first()
        .cloned()
        .expect("PREMISE: a gate opened");
    assert_eq!(answer_item(root, GATE_ITEM, "approve"), Exit::Success);
    let before =
        rk_kernel::workflow::fsm::stage_action(GATE_ITEM, "implement", &support::store_rows(root))
            .expect("the stage action folds before the deletion");

    delete_store(root);
    let after_delete = ScriptedFactory::new(Vec::new());
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, GATE_ITEM),
            &args(GATE_ITEM),
            &support::clean_env(),
            seam(&after_delete),
        ),
        GATE_ITEM,
    );
    assert_eq!(
        after_delete.dispatches(),
        1,
        "after the deletion the run did not reach `implement`'s dispatch (exit {exit:?}): the \
         answered gate read as open"
    );
    let rows = support::store_rows(root);
    let state = rk_kernel::workflow::fsm::fold(GATE_ITEM, "implement", &rows)
        .expect("the rebuilt journal folds");
    let digest = state
        .open_gates()
        .map(|g| g.token().clone())
        .find(|t| t.as_str() == token)
        .unwrap_or_else(|| panic!("the gate {token} is not in the journal after the deletion"));
    assert!(
        state.is_answered(&digest),
        "the gate {token} came back from git unanswered"
    );
    let after = rk_kernel::workflow::fsm::stage_action(GATE_ITEM, "implement", &rows)
        .expect("the stage action folds after the deletion");
    assert_eq!(
        before.canonical_bytes().expect("canonical"),
        after.canonical_bytes().expect("canonical"),
        "AC-161: the stage-level next action at `implement` moved across the deletion"
    );
}

/// **N5: two items in one store do not fold together.** `lane-2` walks to its end, then `lane-1`
/// opens its gate, in one repository and one store. Each item's `ByItem` journal folds to its own
/// stages, its own `stages_entered` and its own gates. Red under S8E-M25.
#[test]
fn two_items_in_one_store_do_not_fold_together() {
    let repo = lane_repo();
    let root = repo.path();
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(through()))
            ),
            THROUGH_ITEM
        ),
        Exit::Success,
        "the first item's lane walks to its end"
    );
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            GATE_ITEM
        ),
        Exit::Success,
        "the second item's lane opens its own gate, unrefused by the first item's transitions"
    );
    assert_eq!(
        story_stages(root, GATE_ITEM),
        vec!["triage".to_owned(), "implement".to_owned()],
        "the gate item's own marks"
    );

    let gate_rows = journal_of(root, GATE_ITEM);
    let gate_state = rk_kernel::workflow::fsm::fold(GATE_ITEM, "implement", &gate_rows)
        .expect("the gate item's journal folds");
    assert!(
        !gate_state.stages_entered().iter().any(|s| s == "review"),
        "the gate item's stages_entered holds a stage only the other item entered: {:?}",
        gate_state.stages_entered()
    );
    assert_eq!(
        gate_state.open_gates().count(),
        1,
        "the gate item's fold holds exactly its own gate"
    );

    let through_rows = journal_of(root, THROUGH_ITEM);
    let through_state = rk_kernel::workflow::fsm::fold(THROUGH_ITEM, "review", &through_rows)
        .expect("the through item's journal folds");
    assert_eq!(
        through_state.open_gates().count(),
        0,
        "the through item's fold holds the gate item's gate"
    );
    assert!(
        !through_rows
            .iter()
            .any(|e| answerable_tokens(std::slice::from_ref(e)).len() == 1),
        "the through item's ByItem journal returned the gate item's opening"
    );
}

/// **AC-518's instrument over a whole lane, with the run tree inside the checkout** (D21-44):
/// `git status --porcelain` empty, `git diff-index --quiet HEAD` clean, and `HEAD` unmoved, after
/// every invocation of a lane that opens a gate, is answered, and walks to its end.
#[test]
fn the_user_checkout_is_unchanged_across_a_lane_with_the_worktree_inside_it() {
    let repo = lane_repo();
    let root = repo.path();
    // The builder writes `.redkiln/` untracked; the user's checkout is committed first so the
    // instrument starts from a clean tree (the run root is git-ignored, D21-48 X1).
    crate::git(root, &["add", "-A"]);
    crate::git(
        root,
        &["commit", "--quiet", "-m", "the user's committed checkout"],
    );
    let head = git_read(root, &["rev-parse", "HEAD"]).expect("HEAD reads");
    let clean = |when: &str| {
        let porcelain = crate::porcelain(root);
        assert!(
            porcelain.is_empty(),
            "{when}: `git status --porcelain` is not empty: {porcelain:?}"
        );
        assert_eq!(
            git_rc(root, &["diff-index", "--quiet", "HEAD"]),
            Some(0),
            "{when}: `git diff-index --quiet HEAD` reports a change"
        );
        assert_eq!(
            git_read(root, &["rev-parse", "HEAD"]).as_deref(),
            Some(head.as_str()),
            "{when}: HEAD moved"
        );
    };
    clean("PREMISE, before the lane");

    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            GATE_ITEM
        ),
        Exit::Success
    );
    let tree = crate::run_tree(root, GATE_ITEM).path;
    assert!(
        tree.starts_with(root) && tree.is_dir(),
        "PREMISE: the run tree {} is inside the checkout",
        tree.display()
    );
    clean("after invocation 1");
    assert_eq!(answer_item(root, GATE_ITEM, "approve"), Exit::Success);
    clean("after the answer");
    let walked = ScriptedFactory::new(through());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, GATE_ITEM),
                &args(GATE_ITEM),
                &support::clean_env(),
                seam(&walked)
            ),
            GATE_ITEM
        ),
        Exit::Success,
        "the lane walks to its end"
    );
    assert_eq!(
        (
            walked.dispatches(),
            story_stages(root, GATE_ITEM).last().map(String::as_str)
        ),
        (2, Some("closed")),
        "PREMISE: the lane must really walk past its gate to its closed mark (implement and review \
         dispatched), or the instrument below reads a lane that did nothing"
    );
    clean("after the lane's end");
}

/// **W2 on the run path: a second invocation reuses the run tree** (`s8d-ii` §8d-ii.3 built it; this
/// is its first lane-level run). The tree's identity is not portably observable, so an IGNORED file
/// is planted after invocation 1 and must survive invocation 2, which a re-creation would delete.
/// Green at the stage's build; red under S8E-M20 (the run path re-creates the tree on re-entry).
#[test]
fn a_second_invocation_reuses_the_run_worktree() {
    let repo = support::pack_min_repo(PackVariant::Base);
    let root = repo.path();
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, BASE_GATE_ITEM),
                &args(BASE_GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            BASE_GATE_ITEM
        ),
        Exit::Success,
        "invocation 1 exits 0 at the gate"
    );
    let tree = crate::run_tree(root, BASE_GATE_ITEM).path;
    assert!(
        tree.is_dir(),
        "PREMISE: invocation 1 created {}",
        tree.display()
    );

    // *(dated 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's checkpoint blocker.)* The
    // planted directory is the run tree's OWN run-tree root, taken through `rk-git`'s one derivation
    // (`s8d-ii` §8d-ii.1; Checkpoint 8d-ii guard (3)) as the parent of a path derived under the run
    // tree — never a joined literal.
    let planted_dir = crate::run_tree(&tree, BASE_GATE_ITEM)
        .path
        .parent()
        .expect("a derived run tree has a parent: the run-tree root")
        .to_path_buf();
    std::fs::create_dir_all(&planted_dir).expect("the planted directory is created");
    let planted = planted_dir.join("rk-e2e-planted.txt");
    std::fs::write(&planted, b"planted after invocation 1\n").expect("the planted file writes");
    let rel = planted
        .strip_prefix(&tree)
        .expect("under the tree")
        .display()
        .to_string()
        .replace('\\', "/");
    assert_eq!(
        git_rc(&tree, &["check-ignore", "-q", &rel]),
        Some(0),
        "PREMISE: the planted file must be IGNORED in the run tree, or W2's discard of untracked \
         in-flight files removes it on a correct re-entry"
    );

    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(root, BASE_GATE_ITEM),
                &args(BASE_GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new()))
            ),
            BASE_GATE_ITEM
        ),
        Exit::Success,
        "invocation 2 re-enters the tree and awaits the same gate"
    );
    assert!(
        planted.is_file(),
        "the ignored file planted after invocation 1 is gone: invocation 2 re-created the run tree \
         rather than reusing it (W2)"
    );
}

/// ~~A harness that dirties a TRACKED file of the run tree on every dispatch, then answers from its
/// script: the change is landed by nothing.~~ *(dated 2026-09-28, s8f build, package
/// S8F-P3-LANDING-E2E; D21-108.)* Under N10 a tracked edit IS landed, at the step's own commit
/// (ADR-0188 decision 2), so that premise is gone. This harness writes a DENYLISTED untracked path,
/// `.env`, on every dispatch: the landing refuses it by name, and once `exclude` is answered every
/// later landing leaves it unstaged. It is written on EVERY dispatch because the re-entry discard
/// removes it between invocations.
struct Dirtying {
    inner: FakeHarness,
    tree: PathBuf,
}

/// The denylisted path [`Dirtying`] writes (`rk_git::landing::DENYLIST`'s `.env`).
const DIRTY_PATH: &str = ".env";

impl Harness for Dirtying {
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        let _ = std::fs::write(
            self.tree.join(DIRTY_PATH),
            b"API_TOKEN=written-by-the-scripted-step\n",
        );
        self.inner.dispatch(spec, store)
    }
}

/// **N9's removal trigger at lane level** (§8e.6; `s8d-ii`'s rule: clean, landed, marked, unlocked,
/// never `--force`). Two items walk to their `closed` mark in one repository. The first leaves a
/// clean tree whose `HEAD` is the run-ref tip: it is gone afterwards, and its registration with it.
/// ~~The second's scripted step edits a tracked file that nothing lands: its tree is dirty at the
/// mark and is kept.~~ *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E; D21-108.)* The
/// second's scripted step writes a denylisted untracked `.env` on every dispatch: invocation 1 stops
/// at the `landing.refused-path` gate, `answer … exclude` applies, and invocation 2 walks to the
/// `closed` mark with `.env` unstaged at every landing, so its tree is dirty at the mark and is
/// kept. Red under S8E-M37 (the clean arm), S8E-M38 (the dirty arm) and S8F-P3's M-EXCL (a landing
/// stages excluded paths).
#[test]
fn a_closed_lane_removes_its_clean_landed_tree_and_keeps_a_dirty_one() {
    // One repository per arm: the arms are independent, and one store holding both items would
    // make this test read `s8e` §8e.10's `ByItem` rather than the removal trigger.
    let registered = |root: &Path, p: &Path| {
        let listing = git_read(root, &["worktree", "list", "--porcelain"]).unwrap_or_default();
        let want = p.display().to_string().replace('\\', "/").to_lowercase();
        listing.lines().any(|l| {
            l.strip_prefix("worktree ")
                .is_some_and(|w| w.replace('\\', "/").to_lowercase() == want)
        })
    };

    // ---- the clean arm: a clean tree whose HEAD is the run-ref tip is removed at its mark.
    let clean_repo = lane_repo();
    let clean_root = clean_repo.path();
    let clean_tree = crate::run_tree(clean_root, THROUGH_ITEM).path;
    let walked = ScriptedFactory::new(through());
    assert_eq!(
        ran(
            rk_cli::verbs::run::run(
                &cli(clean_root, THROUGH_ITEM),
                &args(THROUGH_ITEM),
                &support::clean_env(),
                seam(&walked)
            ),
            THROUGH_ITEM
        ),
        Exit::Success
    );
    assert_eq!(
        (
            walked.dispatches(),
            story_stages(clean_root, THROUGH_ITEM)
                .last()
                .map(String::as_str)
        ),
        (2, Some("closed")),
        "PREMISE: the clean item's lane walked to its closed mark"
    );
    assert!(
        !clean_tree.exists() && !registered(clean_root, &clean_tree),
        "the clean, landed tree {} was not removed at its closed mark (exists: {}, registered: {})",
        clean_tree.display(),
        clean_tree.exists(),
        registered(clean_root, &clean_tree)
    );
    assert!(
        ref_sha(clean_root, THROUGH_ITEM).is_some(),
        "the removal took the run ref with the tree"
    );

    // ---- the dirty arm: ~~the scripted step edits a tracked file nothing lands; the tree is kept.~~
    // *(dated 2026-09-28, s8f build, package S8F-P3-LANDING-E2E; D21-108.)* The scripted step writes
    // a denylisted untracked path on every dispatch; invocation 1 stops at the landing gate, `exclude`
    // is answered, and invocation 2 walks to `closed` with the path left unstaged: the tree is dirty
    // at the mark and is kept.
    let dirty_repo = lane_repo();
    let dirty_root = dirty_repo.path();
    let dirty_tree = crate::run_tree(dirty_root, DIRTY_ITEM).path;
    let dirty_run = |script: Vec<Result<Typed, StepError>>| {
        let factory = ScriptedFactory::over(Box::new(Dirtying {
            inner: FakeHarness::scripted(script),
            tree: dirty_tree.clone(),
        }));
        let exit = ran(
            rk_cli::verbs::run::run(
                &cli(dirty_root, DIRTY_ITEM),
                &args(DIRTY_ITEM),
                &support::clean_env(),
                seam(&factory),
            ),
            DIRTY_ITEM,
        );
        (exit, factory.dispatches())
    };
    assert_eq!(
        dirty_run(vec![implement()]),
        (Exit::Success, 1),
        "PREMISE: invocation 1 dispatches `implement` once and exits 0 at the landing gate"
    );
    let landing_gates: Vec<Event> = support::store_rows(dirty_root)
        .into_iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .filter(|e| {
            e.payload.get("kind").and_then(serde_json::Value::as_str)
                == Some("landing.refused-path")
        })
        .collect();
    assert!(
        landing_gates.len() == 1
            && landing_gates[0]
                .payload
                .get("refused")
                .is_some_and(|r| r.to_string().contains(DIRTY_PATH)),
        "PREMISE: exactly one `landing.refused-path` gate opened, naming {DIRTY_PATH}: \
         {landing_gates:?}"
    );
    assert_eq!(
        answer_item(dirty_root, DIRTY_ITEM, "exclude"),
        Exit::Success,
        "PREMISE: `exclude` applies"
    );
    assert!(
        support::store_rows(dirty_root).iter().any(|e| {
            e.kind == EventKind::GateAnswered
                && e.payload.get("verdict").and_then(serde_json::Value::as_str) == Some("exclude")
        }),
        "PREMISE: the `exclude` answer is recorded"
    );
    let (walked, dispatched) = dirty_run(through());
    assert_eq!(
        (
            walked,
            dispatched,
            story_stages(dirty_root, DIRTY_ITEM).last().cloned()
        ),
        (Exit::Success, 2, Some("closed".to_owned())),
        "PREMISE: invocation 2 re-dispatches `implement` and `review` and walks to the closed mark"
    );
    assert!(
        dirty_tree.is_dir() && registered(dirty_root, &dirty_tree),
        "the dirty tree {} was removed (exists: {}, registered: {}); a dirty tree is kept",
        dirty_tree.display(),
        dirty_tree.is_dir(),
        registered(dirty_root, &dirty_tree)
    );
    assert!(
        git_read(
            &dirty_tree,
            &["status", "--porcelain", "--untracked-files=all"]
        )
        .is_some_and(|s| s.lines().any(|l| l.trim() == format!("?? {DIRTY_PATH}"))),
        "the kept tree is not dirty with the excluded path {DIRTY_PATH}"
    );
    assert!(
        git_rc(
            dirty_root,
            &[
                "cat-file",
                "-e",
                &format!("{}:{DIRTY_PATH}", run_ref(DIRTY_ITEM))
            ]
        ) != Some(0),
        "the excluded path {DIRTY_PATH} landed on the run ref"
    );
}

/// The tier-1 trailer values one commit carries, by key.
#[derive(Default)]
struct Marks {
    story: Vec<String>,
    gate: Vec<Vec<String>>,
    slice: Vec<String>,
}

/// **AC-163's replay instrument**: every `t1_*` row's anchor names a commit on `rk-run/<item>`
/// whose trailer matches the row's kind and value; no two rows share a commit; and every tier-1
/// trailer commit on the ref anchors a row (tier 1, or the anchored tier-2 row of the same act).
pub(crate) fn ac163(root: &Path, store_common_dir: &Path, item: &str) -> Result<(), String> {
    let shas: Vec<String> = git_read(root, &["rev-list", &run_ref(item)])
        .ok_or_else(|| format!("no ref {}", run_ref(item)))?
        .lines()
        .map(str::to_owned)
        .collect();
    let mut on_ref: std::collections::BTreeMap<String, Marks> = std::collections::BTreeMap::new();
    for sha in &shas {
        let read = |key: &str| -> Vec<String> {
            let format = format!("--format=%(trailers:key={key},valueonly)");
            git_read(root, &["log", "-1", &format, sha])
                .unwrap_or_default()
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(str::to_owned)
                .collect()
        };
        let marks = Marks {
            story: read("Story"),
            gate: read("Gate")
                .iter()
                .map(|v| v.split_whitespace().map(str::to_owned).collect())
                .collect(),
            slice: read("Slice-Verdict"),
        };
        if !(marks.story.is_empty() && marks.gate.is_empty() && marks.slice.is_empty()) {
            on_ref.insert(sha.clone(), marks);
        }
    }

    let stage_rows = sql_rows(
        store_common_dir,
        &format!(
            "SELECT CAST(anchor_sha AS TEXT), CAST(to_stage AS TEXT) FROM t1_stage_transition \
             WHERE item = '{item}'"
        ),
    )?;
    let gate_rows = sql_rows(
        store_common_dir,
        "SELECT CAST(anchor_sha AS TEXT), CAST(token AS TEXT), CAST(state AS TEXT) FROM \
         t1_gate_transition",
    )?;
    let seal_rows = sql_rows(
        store_common_dir,
        "SELECT CAST(anchor_sha AS TEXT), CAST(project AS TEXT), CAST(slice AS TEXT), \
         CAST(verdict AS TEXT) FROM t1_slice_seal",
    )?;
    let anchored_t2 = sql_rows(
        store_common_dir,
        "SELECT DISTINCT CAST(anchor_sha AS TEXT) FROM t2_event WHERE anchor_sha IS NOT NULL",
    )?;

    let text = |row: &[Option<String>], i: usize| row.get(i).cloned().flatten().unwrap_or_default();
    let mut anchors: Vec<String> = Vec::new();
    for row in &stage_rows {
        let (anchor, to) = (text(row, 0), text(row, 1));
        let marks = on_ref.get(&anchor).ok_or_else(|| {
            format!("t1_stage_transition -> {to}: anchor {anchor} is no trailer commit on the ref")
        })?;
        let want = format!("{PROJECT}/{item} {to}");
        if !marks.story.contains(&want) {
            return Err(format!(
                "t1_stage_transition -> {to}: its commit {anchor} carries {:?}, not `Story: {want}`",
                marks.story
            ));
        }
        anchors.push(anchor);
    }
    for row in &gate_rows {
        let (anchor, token, state) = (text(row, 0), text(row, 1), text(row, 2));
        let marks = on_ref
            .get(&anchor)
            .ok_or_else(|| format!("t1_gate_transition {token} {state}: anchor {anchor} is no trailer commit on the ref"))?;
        if !marks
            .gate
            .iter()
            // *(Re-pointed 2026-09-28, S8E-P4; ADR-0184: the state is the sixth token, not the
            // last — ~~`m.last() == Some(&state)`~~.)*
            .any(|m| m.first() == Some(&token) && m.get(5) == Some(&state))
        {
            return Err(format!(
                "t1_gate_transition {token} {state}: its commit {anchor} carries {:?}",
                marks.gate
            ));
        }
        anchors.push(anchor);
    }
    for row in &seal_rows {
        let (anchor, project, slice, verdict) =
            (text(row, 0), text(row, 1), text(row, 2), text(row, 3));
        let marks = on_ref.get(&anchor).ok_or_else(|| {
            format!(
                "t1_slice_seal {slice} {verdict}: anchor {anchor} is no trailer commit on the ref"
            )
        })?;
        let want = format!("{project}/{slice} {verdict}");
        if !marks.slice.contains(&want) {
            return Err(format!(
                "t1_slice_seal: its commit {anchor} carries {:?}, not `Slice-Verdict: {want}`",
                marks.slice
            ));
        }
        anchors.push(anchor);
    }
    let mut seen = std::collections::BTreeSet::new();
    for anchor in &anchors {
        if !seen.insert(anchor.clone()) {
            return Err(format!(
                "two rows of distinct transitions share the commit {anchor}"
            ));
        }
    }
    let t2: std::collections::BTreeSet<String> = anchored_t2.iter().map(|r| text(r, 0)).collect();
    for sha in on_ref.keys() {
        if !seen.contains(sha) && !t2.contains(sha) {
            return Err(format!(
                "the tier-1 trailer commit {sha} on the ref anchors no row"
            ));
        }
    }
    Ok(())
}

/// **AC-163's replay instrument over the fixture lane.** After a lane that writes every tier-1 kind
/// this stage mints (the lane's entry and boundaries, a gate's opening and answer, the slice seal,
/// the closed mark), each `t1_*` row's anchor names a matching trailer commit on `rk-run/<item>`, no
/// two rows share a commit, and every tier-1 trailer commit anchors a row. The adversarial arm plants
/// a `t1_stage_transition` row whose anchor names no commit, and the instrument must fail on it —
/// the register's *"which fails on a store row with no matching commit"*. Red under S8E-M43.
/// *(dated 2026-09-28, `s8e` build, package R1FIX-8e: measured red, on the expiry arm, under both
/// sitings — S8E-M43a, `run`'s expiry rows through `run::gate_rows_through`'s store-only arm, which
/// ONLY this test kills, and S8E-M43b, the kernel boundary's store-only expiry.)*
///
/// *(dated 2026-09-28, `s8e` build, package S8E-P7.)* **A second arm, for the gate EXPIRY.** The
/// fixture lane opens and answers its gate and never lets one expire, so S8E-M43 (an expiry's row
/// appended with no commit) could not redden the arm above: measured as a missing fixture branch.
/// The expiry arm runs `gate-1` over `pack.gate-1s.toml` (a 1 s deadline, `fail` on expiry): the
/// gate opens in process, the deadline passes (polled, never a fixed sleep), and a second `run`'s
/// expiry pass commits `Gate: … expired fail` and anchors its row. The same instrument must hold
/// over that repository, with the `expired` row present.
///
/// *(dated 2026-09-28, `s8e` build, package R1FIX-8e.)* **The lane now carries `voided`.** It
/// walks through [`walk_the_lane_through_a_void`], so the first arm reads two openings, two
/// answers and a void (`Gate: … voided`, anchored to its own commit) besides the boundaries and
/// the seal: every gate state is instrumented, `expired` in the second arm.
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "the expiry arm waits past a real 1 s gate deadline, polled against the recorded \
              deadline (never a fixed sleep); the PRODUCT reads no clock here"
)]
fn every_tier_one_row_has_exactly_one_trailer_commit() {
    let repo = lane_repo();
    let root = repo.path();
    // *(dated 2026-09-28, `s8e` build, package R1FIX-8e.)* ~~open, answer, walk~~: the lane now
    // walks THROUGH A VOID (G1 opened, answered and voided; G2 opened and answered), so the
    // instrument covers the `voided` transition as well.
    walk_the_lane_through_a_void(root);

    let common = crate::common_dir(root);
    // *(dated 2026-09-28, `s8e` build, package S8E-P7: the id is read as TEXT; `sql_rows` decodes
    // every column as `Option<String>`, and an INTEGER `id` failed the premise on the read itself.)*
    let count = |sql: &str| -> usize {
        sql_rows(&common, sql)
            .unwrap_or_else(|e| panic!("PREMISE: the tier-1 tables must be readable: {e}"))
            .len()
    };
    assert!(
        count("SELECT CAST(id AS TEXT) FROM t1_stage_transition") >= 2
            && count("SELECT CAST(id AS TEXT) FROM t1_gate_transition WHERE state = 'opened'") == 2
            && count("SELECT CAST(id AS TEXT) FROM t1_gate_transition WHERE state = 'answered'")
                == 2
            && count("SELECT CAST(id AS TEXT) FROM t1_gate_transition WHERE state = 'voided'") == 1
            && count("SELECT CAST(id AS TEXT) FROM t1_slice_seal") == 1,
        "PREMISE: the lane must write every tier-1 kind (boundaries, two openings, two answers, a \
         void, a seal): {:?}",
        sql_rows(
            &common,
            "SELECT state, token FROM t1_gate_transition ORDER BY id"
        )
    );
    ac163(root, &common, GATE_ITEM).unwrap_or_else(|e| panic!("AC-163: {e}"));

    // ---- the adversarial arm, over a COPY of the store.
    let copy = tempfile::tempdir().expect("a scratch directory");
    let copy_store = copy.path().join("redkiln");
    std::fs::create_dir_all(&copy_store).expect("the copy's directory");
    for suffix in ["", "-wal", "-shm"] {
        let from = PathBuf::from(format!("{}{suffix}", store_file(root).display()));
        if from.exists() {
            std::fs::copy(&from, copy_store.join(format!("store.sqlite3{suffix}")))
                .expect("the store copies");
        }
    }
    ac163(root, copy.path(), GATE_ITEM).unwrap_or_else(|e| panic!("PREMISE: the copy passes: {e}"));
    let planted = rk_store::conn::Reader::open(copy.path()).expect("the copy opens");
    planted
        .conn()
        .execute(
            &format!(
                "INSERT INTO t1_stage_transition (item, from_stage, to_stage, anchor_sha, \
                 tree_hash) VALUES ('{GATE_ITEM}', 'implement', 'review', '{}', '{}')",
                "0".repeat(40),
                "0".repeat(40)
            ),
            [],
        )
        .expect("the adversarial row is planted");
    drop(planted);
    assert!(
        ac163(root, copy.path(), GATE_ITEM).is_err(),
        "ADVERSARIAL: a t1_stage_transition row whose anchor names no commit must fail AC-163's \
         instrument"
    );

    // ---- the expiry arm (dated 2026-09-28, S8E-P7): `Gate: … expired fail` is a trailer commit
    // ---- that anchors its row.
    let expiring = support::pack_min_repo(PackVariant::GateOneSecond);
    let eroot = expiring.path();
    let erun = || {
        ran(
            rk_cli::verbs::run::run(
                &cli(eroot, BASE_GATE_ITEM),
                &args(BASE_GATE_ITEM),
                &support::clean_env(),
                seam(&ScriptedFactory::new(Vec::new())),
            ),
            BASE_GATE_ITEM,
        )
    };
    assert_eq!(erun(), Exit::Success, "PREMISE: the 1 s gate opens");
    let deadline = support::store_rows(eroot)
        .iter()
        .filter(|e| e.kind == EventKind::GateOpened)
        .find_map(|e| {
            e.payload
                .get("deadline")
                .and_then(serde_json::Value::as_i64)
        })
        .expect("PREMISE: the gate's row records its deadline");
    let now = || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
    };
    let mut polls = 0_u32;
    while now() <= deadline {
        polls += 1;
        assert!(polls < 200, "the 1 s deadline {deadline} never passed");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let _ = erun();
    let ecommon = crate::common_dir(eroot);
    // The expiry's `gate_expired` row is a tier-1 row: it exists only anchored to its own
    // `Gate: … expired` commit on the run ref, beside its `t1_gate_transition` row. A store-only
    // expiry (S8E-M43) leaves the row unanchored and no transition row, and is red here.
    let expired = sql_rows(
        &ecommon,
        "SELECT CAST(anchor_sha AS TEXT) FROM t2_event WHERE kind = 'gate_expired'",
    )
    .unwrap_or_else(|e| panic!("PREMISE: the store must be readable: {e}"));
    assert_eq!(
        expired.len(),
        1,
        "PREMISE: the expiry pass wrote one `gate_expired` row"
    );
    let anchor = expired[0][0].clone().unwrap_or_default();
    let on_ref = git_read(
        eroot,
        &[
            "log",
            "--format=%H %(trailers:key=Gate,valueonly,separator=%x20)",
            &format!("refs/heads/rk-run/{BASE_GATE_ITEM}"),
        ],
    )
    .unwrap_or_default();
    assert!(
        !anchor.is_empty()
            && on_ref
                .lines()
                .any(|l| l.starts_with(&anchor) && l.contains(" expired ")),
        "AC-163: the expiry's `gate_expired` row must be anchored to a `Gate: … expired` commit on          rk-run/{BASE_GATE_ITEM}; its anchor is {anchor:?} and the ref reads:
{on_ref}"
    );
    assert_eq!(
        sql_rows(
            &ecommon,
            "SELECT CAST(id AS TEXT) FROM t1_gate_transition WHERE state = 'expired'"
        )
        .unwrap_or_else(|e| panic!("PREMISE: the tier-1 tables must be readable: {e}"))
        .len(),
        1,
        "AC-163: the expiry writes its one `expired` transition row"
    );
    ac163(eroot, &ecommon, BASE_GATE_ITEM)
        .unwrap_or_else(|e| panic!("AC-163 over the expiry arm: {e}"));
}

// ------------------------------------------------------ `run()`'s two owed bindings (§8e.11)

/// Rewrite `pack-min`'s one activity as a ZERO-AGENT activity whose steps are `steps` (TOML), which
/// provides `provides` — so the step reaches the kernel with no dispatch before it.
fn zero_agent_activity(root: &Path, provides: &[&str], steps: &str) {
    let provides = provides
        .iter()
        .map(|k| format!("\"{k}\""))
        .collect::<Vec<_>>()
        .join(", ");
    std::fs::write(
        root.join(".redkiln")
            .join("catalog")
            .join("emit-report.toml"),
        format!(
            "[activity]\nid           = \"emit-report\"\nversion      = 1\nstage        = \
             \"implement\"\nprovides     = [{provides}]\nrequires     = []\nderived_from = \
             []\ndeviation_ok = []\ntags         = [\"kind:bug\", \"kind:story\"]\nmethod_rank  = \
             10\nmin_rigor    = \"optional\"\ncost_class   = \"S\"\nbudget       = {{ \
             attempts_total = 1, tokens = 1000, wall_clock_secs = 900 }}\n\n{steps}"
        ),
    )
    .expect("the zero-agent activity is written over the fixture's one activity");
}

/// A verb registry with the rows `rows` and the Rust value table `values`.
fn registry(root: &Path, rows: &str, values: &str) {
    let pin = "0123456789abcdef0123456789abcdef01234567";
    let values = values.replace("PIN_SHA", pin);
    std::fs::write(
        root.join(".redkiln").join("registry.toml"),
        format!("[verb]\n{rows}\n[toolchain.rust]\n{values}\n"),
    )
    .expect("the registry is written");
}

/// Append an artifact row to the installed registry.
fn register_artifact(root: &Path, row: &str) {
    let path = root.join(".redkiln").join("artifacts.toml");
    let mut src = std::fs::read_to_string(&path).expect("the artifact registry reads");
    src.push_str(row);
    std::fs::write(&path, src).expect("the artifact registry writes");
}

/// The environment the toolchain tests run under: `clean_env` plus the process-start keys the
/// toolchain adapters refuse to run without (`rk-adapter-toolchain-rust`'s `REQUIRED_CHILD_ENV` and
/// `REQUIRED_ANY_OF`: `SystemDrive`, and a temp directory), and `PATH` / `PATHEXT` / `SystemRoot`
/// so the adapter's own `rustc -V` resolves as it does under the binary. Nothing else is read: no
/// credential and no `CLAUDE_*` value reaches the run.
fn toolchain_env() -> rk_cli::env::Env {
    let mut map = std::collections::BTreeMap::new();
    for key in [
        "PATH",
        "Path",
        "PATHEXT",
        "SystemRoot",
        "SystemDrive",
        "TMP",
        "TEMP",
    ] {
        if let Some(v) = std::env::var_os(key) {
            map.insert(key.to_owned(), v.to_string_lossy().into_owned());
        }
    }
    rk_cli::env::Env::from_map(map)
}

/// **MF3j, killed through the seam** (`s8d` F3-D2174 receiver note; D21-74 ruling 2; R2 =
/// ADR-0173 decision 9 as amended). `run()` binds `change_deriver::RunBranchDiff` with
/// `ToolRouting::with_run_branch`, and `"affected"` inside `Changeset`'s own producer reads it.
///
/// The fixture lane's `implement` activity is a zero-agent producer of `Changeset` whose first step
/// is `toolchain:test` with `selection = "affected"`, over `pack-min` with the stub `cargo` as the
/// one Rust candidate. `rk-run/bug-1` exists before the run and carries one commit changing
/// `src/lib.rs`, so `HEAD..rk-run/bug-1` is not empty.
///
/// The observation channel is the EXIT CODE (the kernel journals no row for a `tool` step's
/// refusal). The kernel refuses an unresolvable selection as `Unsupported` (19) BEFORE the port: with
/// no reader bound (*"no run-branch reader is bound"*), and with a reader over an empty range
/// (*"carries no change"*). The CONTROL is the second: the same fixture with no commit on the run
/// branch must read 19. The case must read anything else — the step reached the port with the run
/// branch's paths, where the stub's `cargo metadata` answer is not metadata. Red under MF3j (the
/// binding removed), for the reason the note names.
#[test]
fn run_binds_the_run_branch_reader_for_affected_inside_the_changeset_producer() {
    let arm = |changed: bool| -> (Exit, usize) {
        let repo = support::pack_min_repo(PackVariant::Base);
        let root = repo.path();
        support::install_review_toolchain(root);
        register_artifact(
            root,
            "\n[artifact.Changeset]\nsource = \"activity\"\nschema = \"Changeset\"\n",
        );
        zero_agent_activity(
            root,
            &["ImplReport", "Changeset"],
            "[[step]]\nid           = \"tests\"\nkind         = \"tool\"\nrun          = \
             \"toolchain:test\"\ninputs       = { selection = \"affected\" }\ntimeout_secs = \
             60\n\n[[step]]\nid        = \"emit\"\nkind      = \"emit\"\nartifacts = { \
             ImplReport = \"${tests.outputs}\", Changeset = \"${tests.outputs}\" }\n",
        );
        registry(
            root,
            "test = { outputs = \"TestReport\", live = false, timeout = \"test_timeout\", inputs = \
             [\"selection\"], route = \"port\", side_effecting = false }",
            "test_timeout = { secs = 60, measured = [2], rule = \"r\", pin = \"PIN_SHA\", command = \
             \"c\" }",
        );
        if changed {
            crate::git(root, &["checkout", "--quiet", "-b", "rk-run/bug-1"]);
            std::fs::create_dir_all(root.join("src")).expect("src/");
            std::fs::write(
                root.join("src").join("lib.rs"),
                b"// changed on the run branch\n",
            )
            .expect("the change writes");
            crate::git(root, &["add", "src/lib.rs"]);
            crate::git(
                root,
                &["commit", "--quiet", "-m", "a change on the run branch"],
            );
            crate::git(root, &["checkout", "--quiet", "-"]);
            let range = git_read(root, &["diff", "--name-only", "HEAD", "rk-run/bug-1"])
                .unwrap_or_default();
            assert_eq!(
                range.trim(),
                "src/lib.rs",
                "PREMISE: HEAD..rk-run/bug-1 carries exactly the one change"
            );
        }
        let factory = ScriptedFactory::new(Vec::new());
        let exit = ran(
            rk_cli::verbs::run::run(
                &cli(root, "bug-1"),
                &args("bug-1"),
                &toolchain_env(),
                seam(&factory),
            ),
            "bug-1",
        );
        (exit, factory.dispatches())
    };

    let (control, dispatched) = arm(false);
    assert_eq!(dispatched, 0, "PREMISE: the activity is zero-agent");
    assert_eq!(
        control,
        Exit::Unsupported,
        "CONTROL: with an empty run branch, `affected` inside the producer must refuse before the \
         port (`carries no change`) as Unsupported, or the case's code below says nothing"
    );

    let (case, dispatched) = arm(true);
    println!("MF3j reading: control exit {control:?}, case exit {case:?}");
    assert_eq!(dispatched, 0, "PREMISE: the activity is zero-agent");
    assert_ne!(
        case,
        Exit::Unsupported,
        "`run()` did not bind the run-branch reader: `affected` inside `Changeset`'s producer was \
         refused before the port (19 is `no run-branch reader is bound`), where a bound reader \
         hands the port the run branch's paths"
    );
}

/// **`run()`'s review-routes binding, killed through the seam** (`s8d` R2FIX-8d receiver note; R5 =
/// ADR-0177 decision 2). `run()` binds `review_routes(..)` with `ToolRouting::with_review_routes`,
/// and a `toolchain:shallow_clone` step clones THIS repository at a sha into
/// `<common-dir>/redkiln/clones/<item>/<sha>` through it.
///
/// A zero-agent `implement` activity whose first step is `toolchain:shallow_clone` at `HEAD`'s sha.
/// The observation is the clone itself: its directory, with `HEAD`'s tracked `README` checked out.
/// Red under the with_review_routes-removed mutant (the kernel then refuses the step, *"no review
/// routes are bound"*, and no clone is made).
#[test]
fn run_binds_the_review_routes_for_a_shallow_clone_step() {
    let repo = support::pack_min_repo(PackVariant::Base);
    let root = repo.path();
    support::install_review_toolchain(root);
    let sha = git_read(root, &["rev-parse", "HEAD"])
        .expect("HEAD reads")
        .trim()
        .to_owned();
    register_artifact(
        root,
        "\n[artifact.Workspace]\nsource = \"activity\"\nschema = \"Workspace\"\n",
    );
    zero_agent_activity(
        root,
        &["ImplReport"],
        &format!(
            "[[step]]\nid           = \"clone\"\nkind         = \"tool\"\nrun          = \
             \"toolchain:shallow_clone\"\ninputs       = {{ sha = \"{sha}\" }}\noutputs      = \
             \"value:Workspace\"\ntimeout_secs = 120\n\n[[step]]\nid        = \"emit\"\nkind      \
             = \"emit\"\nartifacts = {{ ImplReport = \"${{clone.outputs}}\" }}\n"
        ),
    );
    registry(
        root,
        "shallow_clone = { outputs = \"Workspace\", live = false, timeout = \"build_timeout\", \
         inputs = [\"sha\"], route = \"review_clone\", side_effecting = true }",
        "build_timeout = { secs = 60, measured = [2], rule = \"r\", pin = \"PIN_SHA\", command = \
         \"c\" }",
    );
    let clone = crate::common_dir(root)
        .join("redkiln")
        .join("clones")
        .join("bug-1")
        .join(&sha);
    assert!(!clone.exists(), "PREMISE: no clone before the run");

    let factory = ScriptedFactory::new(Vec::new());
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, "bug-1"),
            &args("bug-1"),
            &toolchain_env(),
            seam(&factory),
        ),
        "bug-1",
    );
    println!(
        "review-routes reading: exit {exit:?}, clone at {}",
        clone.display()
    );
    assert_eq!(
        factory.dispatches(),
        0,
        "PREMISE: the activity is zero-agent"
    );
    assert!(
        clone.join("README").is_file(),
        "`run()` did not bind the review routes: no review clone at {} (exit {exit:?}); unbound, \
         the kernel refuses `shallow_clone` before any clone is made",
        clone.display()
    );
}

// ------------------------------------------------- D21-94 / D21-95: the slice is refused at entry

/// Delete `item`'s one `slice` line from its INSTALLED record, asserting exactly one was there.
fn drop_slice(root: &Path, item: &str) {
    let path = root
        .join(".redkiln")
        .join("items")
        .join(format!("{item}.toml"));
    let src = std::fs::read_to_string(&path).expect("the item record reads");
    let kept: Vec<&str> = src
        .lines()
        .filter(|l| !l.trim_start().starts_with("slice"))
        .collect();
    assert_eq!(
        src.lines().count() - kept.len(),
        1,
        "PREMISE: the installed record of `{item}` carries exactly one `slice` line"
    );
    std::fs::write(&path, kept.join("\n") + "\n").expect("the item record rewrites");
}

/// **D21-94 / D21-95: an undeclared slice is refused at the lane's ENTRY, before any dispatch,
/// when a stage the lane will reach has a catalog activity whose `provides` first is `Findings`.**
/// *(Added 2026-09-28, `s8e` build, package S8E-D2194-ENTRY; ADR-0186 as amended, proposed.)*
///
/// `lane-2` sits at `triage`, whose own plan is empty: only the WALK forward (`triage`, the
/// `forbidden` `discovery` skipped, `implement`, `review`) reaches `review-min`'s `Findings`. So a
/// check that read the current stage alone would let the lane dispatch `implement` and fail at the
/// mint instead. The in-process run reads the seam: exit `3`, ZERO dispatches and zero harness
/// builds, no run tree, no commit on `rk-run/lane-2`, no tier-1 row. The binary run over the same
/// repository reads the refusal BY NAME on stderr, which an in-process run cannot capture. Red
/// under S8E-M53 (the entry check dropped: `implement` is dispatched) and S8E-M54 (the walk reads
/// the current stage only: `triage` has no `Findings` activity, so the lane runs).
#[test]
fn an_undeclared_slice_is_refused_at_entry_before_any_dispatch() {
    let repo = lane_repo();
    let root = repo.path();
    drop_slice(root, THROUGH_ITEM);
    assert!(
        declared_lane(root).iter().any(|s| s == "review"),
        "PREMISE: the fixture lane declares a `review` stage"
    );

    let factory = ScriptedFactory::new(through());
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, THROUGH_ITEM),
            &args(THROUGH_ITEM),
            &support::clean_env(),
            seam(&factory),
        ),
        THROUGH_ITEM,
    );
    assert_eq!(
        factory.dispatches(),
        0,
        "the lane dispatched {} time(s) before refusing an undeclared slice it would need at \
         `review` (exit {exit:?}); D21-94 refuses it at entry, before any dispatch",
        factory.dispatches()
    );
    assert_eq!(
        exit,
        Exit::ConfigError,
        "an undeclared slice on a lane that reaches a `Findings` stage is exit 3 at entry"
    );
    assert_eq!(
        factory.builds(),
        0,
        "a harness was built before the refusal"
    );
    assert!(
        !crate::run_tree(root, THROUGH_ITEM).path.exists(),
        "the refused run entered a run tree"
    );
    assert_eq!(
        ref_sha(root, THROUGH_ITEM),
        None,
        "the refused run committed on rk-run/{THROUGH_ITEM}"
    );
    let tier_one = support::store_rows(root)
        .iter()
        .filter(|e| e.kind.tier() == rk_types::event_kind::Tier::One)
        .count();
    assert_eq!(tier_one, 0, "the refused run wrote a tier-1 row");

    // BY NAME: the binary over the same repository, whose stderr is readable.
    let out = spawn_run_replaying(root, THROUGH_ITEM);
    let err = String::from_utf8_lossy(&out.stderr).into_owned();
    assert_eq!(out.status.code(), Some(3), "the binary refuses too:\n{err}");
    assert!(
        err.contains(&format!("`{THROUGH_ITEM}` declares no `slice`")),
        "the refusal names the undeclared slice of `{THROUGH_ITEM}`:\n{err}"
    );
    assert!(
        err.contains("before any dispatch") && err.contains("`review`"),
        "the refusal names the entry siting and the `Findings` stage the lane reaches:\n{err}"
    );
    assert_eq!(
        ref_sha(root, THROUGH_ITEM),
        None,
        "the binary's refusal committed on rk-run/{THROUGH_ITEM}"
    );
}

/// **The control: a lane that reaches NO `Findings` stage is not refused at entry for a slice it
/// never needs** (D21-95 item 1; ADR-0159 decision 2's reason, kept for every other lane).
/// *(Added 2026-09-28, `s8e` build, package S8E-D2194-ENTRY.)*
///
/// `pack.toml` declares no `stage_order` and installs only `emit-report` (`ImplReport`), and
/// `bug-1` declares no `slice`. The run dispatches `implement` once and exits 0. A check that
/// refused every undeclared slice, whatever the lane reaches, is red here.
#[test]
fn a_lane_that_reaches_no_findings_stage_is_not_refused_for_an_undeclared_slice() {
    let repo = support::pack_min_repo(PackVariant::Base);
    let root = repo.path();
    let record = std::fs::read_to_string(root.join(".redkiln").join("items").join("bug-1.toml"))
        .expect("the item record reads");
    assert!(
        !record.lines().any(|l| l.trim_start().starts_with("slice")),
        "PREMISE: `bug-1` declares no `slice`"
    );
    let catalog_dir = root.join(".redkiln").join("catalog");
    let findings = std::fs::read_dir(&catalog_dir)
        .expect("the catalog reads")
        .filter_map(Result::ok)
        .filter(|e| {
            std::fs::read_to_string(e.path())
                .unwrap_or_default()
                .contains("\"Findings\"")
        })
        .count();
    assert_eq!(
        findings, 0,
        "PREMISE: no installed activity names `Findings`"
    );

    let factory = ScriptedFactory::new(vec![implement()]);
    let exit = ran(
        rk_cli::verbs::run::run(
            &cli(root, "bug-1"),
            &args("bug-1"),
            &support::clean_env(),
            seam(&factory),
        ),
        "bug-1",
    );
    assert_eq!(
        exit,
        Exit::Success,
        "a lane that never reaches a `Findings` stage was refused (or failed) with no slice"
    );
    assert_eq!(
        factory.dispatches(),
        1,
        "the control lane did not dispatch its one `implement` activity"
    );
}

/// **The entry check skips a `forbidden` CURRENT stage** (`s8e` §8e.8's S8E-D2194-ENTRY note: *"a
/// `forbidden` current stage is skipped too"*; D21-95 item 1: *"skipping forbidden stages"*).
/// *(Added 2026-09-28, `s8e` build, package R1FIX-8e; refuting round 1's tests-mutants minor: the
/// guard had no fixture branch and no mutant.)*
///
/// `Pack::successor` already skips a `forbidden` successor, so the guard in
/// `findings_stage_reached` is observable only when the item SITS at a forbidden stage that carries
/// a `Findings`-first activity. Two arms over the fixture lane, each in its own repository, each
/// with `lane-2`'s `slice` dropped, `review-min` re-declared to provide `ImplReport` (so no later
/// stage reaches `Findings`), and one extra catalog activity, `review-here`, a copy of
/// `review-min` providing `Findings` at the item's current stage:
/// - **CONTROL** — `lane-2` and `review-here` at `triage` (`optional`): the binary refuses at entry
///   by name (*"declares no `slice`"*), so the extra activity is read and the arm can go red;
/// - **CASE** — both at `discovery` (`forbidden`): the entry check skips the stage, so no entry
///   refusal is named, whatever the lane then does with an item at a forbidden stage.
///
/// Red under S8E-M56 (the guard dropped: the CASE is refused at entry).
#[test]
fn the_entry_check_skips_a_forbidden_current_stage_with_a_findings_activity() {
    let arm = |stage: &str| {
        let repo = lane_repo();
        let root = repo.path();
        drop_slice(root, THROUGH_ITEM);
        let items = root.join(".redkiln").join("items");
        let record_path = items.join(format!("{THROUGH_ITEM}.toml"));
        let record = std::fs::read_to_string(&record_path).expect("the item record reads");
        assert_eq!(
            record.matches("stage = \"triage\"").count(),
            1,
            "PREMISE: `{THROUGH_ITEM}` sits at `triage`"
        );
        std::fs::write(
            &record_path,
            record.replace("stage = \"triage\"", &format!("stage = \"{stage}\"")),
        )
        .expect("the item record rewrites");
        let catalog = root.join(".redkiln").join("catalog");
        let review_min =
            std::fs::read_to_string(catalog.join("review-min.toml")).expect("review-min reads");
        // *(dated 2026-09-29, `s8g` build, package R-D2119-8g; D21-119.)* ~~`provides     =
        // ["Findings"]`~~ -- `review-min` now declares the review verdict kind first and
        // `Findings` second, so the copy below is a review activity to either entry key.
        let provides = "provides     = [\"ReviewVerdict\", \"Findings\"]";
        let stage_line = "stage        = \"review\"";
        assert!(
            review_min.matches(provides).count() == 1
                && review_min.matches(stage_line).count() == 1,
            "PREMISE: review-min declares the review verdict kind and `Findings` at `review`"
        );
        std::fs::write(
            catalog.join("review-here.toml"),
            review_min
                .replace(
                    "id           = \"review-min\"",
                    "id           = \"review-here\"",
                )
                .replace(stage_line, &format!("stage        = \"{stage}\"")),
        )
        .expect("the extra activity installs");
        std::fs::write(
            catalog.join("review-min.toml"),
            review_min.replace(provides, "provides     = [\"ImplReport\"]"),
        )
        .expect("review-min is re-declared");
        let out = spawn_run_replaying(root, THROUGH_ITEM);
        (
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).into_owned(),
        )
    };
    let refusal = format!("`{THROUGH_ITEM}` declares no `slice`");

    let (code, err) = arm("triage");
    assert!(
        code == Some(3) && err.contains(&refusal) && err.contains("before any dispatch"),
        "CONTROL: a `Findings` activity at the item's current, non-forbidden stage must be \
         refused at entry by name (exit {code:?}):\n{err}"
    );

    let (code, err) = arm("discovery");
    assert!(
        !err.contains(&refusal),
        "a `forbidden` current stage's `Findings` activity was read by the entry check, which \
         must skip it (exit {code:?}):\n{err}"
    );
}
