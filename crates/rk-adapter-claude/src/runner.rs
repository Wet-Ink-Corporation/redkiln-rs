//! Spawn one `claude -p` through `rk-containment`, stream it, race its exit against the token and
//! the adapter's own timer, and hand back everything it said.
//!
//! (`s7c` §7c.10, §7c.11; ADR-0099 decisions 2 and 3; ADR-0127; ADR-0076; RT-3, RT-13 as
//! amended.)
//!
//! **Lifted, with edits, from `runner/src/runner.rs` in `redkiln-corpus` at
//! `61f590e51cbb00301c310e46aab615563a7787b8`** (the SHA in `CORPUS_PIN`; blob
//! `cbd47c387466cc6c1a81eb32ec14a0fb3860aca4`, 802 lines). That file was the spike runner's
//! *"spawn one `claude -p`, stream it, and record everything it said"*, **built and run over the
//! programme's recorded dispatches (E0–E6, F-series) and exercised, not validated against an
//! oracle** (PIA-036, PIA-062). Its two shaping constraints are kept verbatim: **the prompt goes
//! on stdin, never argv** (Windows caps the command line near 32 KB and the review card alone is
//! 6.6 KB — the argv cap in `command.rs` is the same fact as a lint), and **a record is always
//! produced, even when the step fails** — here `lib.rs`'s `dispatch_completed` on every path
//! (ADR-0126).
//!
//! **What is lifted** (the table's `runner.rs` row): `build_args`' element order — the golden
//! order, in `command.rs` (§7c.5); the prompt written to piped stdin then EOF (AC-215's
//! write-then-EOF — see *the stdin write* below for what EOF is on this platform); `system/init`
//! → effective config + isolation (`init.rs`); the drain loop keeping the **last**
//! `rate_limit_event` and counting `api_retry` lines; record-on-every-path (the runner's
//! `finish()` — `lib.rs` here); and `extract_digest`'s schema validation (`classify.rs`, over
//! `jsonschema =0.56.0`).
//!
//! **Edits** (§7c.2): the discarded stdin write errors (`let _ = stdin.write_all(..)`) become
//! `Transport { kind: Io }` — consulted only when nothing more specific was observed
//! (`classify.rs`'s order); isolation extends to `NotIsolated { surface, diff_digest }` as a set
//! digest (AC-202); `StreamLine.raw` is the line's bytes, not a re-serialised `Value` (AC-200);
//! the runner's unwrap-or-default on the stderr task is gone (a task that fails to join is a
//! marker line on the retained stderr by an explicit `match`, never an empty default that reads
//! as "the child wrote nothing").
//!
//! **NOT lifted** (the table's last column, by symbol): **`kill_tree`** (`taskkill /T /F`, a PID
//! walk — F22's survivor is what it misses) → the kill is **through `rk_containment::Group`**
//! (AC-224; ADR-0127); **the timeout branch** (`StepError::Timeout(spec.timeout)` calling the tree
//! walk) → the adapter's own timer arm below, classified `Cancelled { by: Timeout }` (RT-3 as
//! amended); **the usage-limit classification** (`UsageLimit { resets_at: None }` hard-coded;
//! `detect_usage_limit` reading the result's `resets_at`) → `classify.rs` reading the last
//! non-`allowed` `rate_limit_event` (AC-523); **the prose-JSON recovery** on the result path
//! (AC-208); **the pre-spawn git snapshot** (`fingerprint::git_snapshot`) — no carrier on the
//! record, no `rk-git` edge (§7c.2, §7c.19); `tokio::process::Command` (RT-13: every spawn is
//! `rk-containment`'s — ADR-0127 decision 3).
//!
//! **The spawn shape** (§7c.10; RT-13 as amended twice): the adapter builds a
//! `std::process::Command` — program, the golden argv, `current_dir(worktree)`, `env_clear()`
//! then the constructed map (`env.rs`), three piped handles — and hands it to
//! [`rk_containment::spawn_into_job`], which creates the Job Object (Windows) / process group
//! (POSIX) BEFORE the spawn and returns `Contained { child, group }`. The adapter then takes the
//! three pipes with `take()` and wraps **each pipe** with `tokio::process::ChildStdin` /
//! `ChildStdout` / `ChildStderr::from_std` (tokio 1.53.1 has `from_std` on the pipes and not on
//! `Child` — measured, RT-13), drains stdout on a spawned task, reads stderr on a spawned task,
//! writes stdin on a spawned task, and awaits the exit with `spawn_blocking(move || child.wait())`
//! on the adapter's `multi_thread` runtime. `tokio::process::Command` and `tokio::process::Child`
//! appear nowhere (the block's ban).
//!
//! **The stdin write** (AC-215's write-then-EOF; the runtime-safety review of Slice 5, R5-1):
//! `write_all` + **`flush`** + drop. On Windows, tokio 1.53.1's `ChildStdin` is
//! `io::blocking::Blocking<ArcFile>`: `poll_write` copies up to 2 MiB into a buffer, enqueues the
//! `WriteFile` on a blocking thread and returns `Ready(Ok(n))` at once, and `poll_shutdown` is
//! `Ready(Ok(()))` with no wait — so `write_all` + `shutdown` completed `Ok` while the only write
//! was still in flight and its error (`os error 109`, the pipe ended) was dropped. `poll_flush`
//! awaits the in-flight write and surfaces its error; EOF reaches the child when the handle is
//! DROPPED at the end of the task, not from `shutdown()`, which is a no-op on this platform
//! [measured 2026-09-18]. The write runs concurrently with the child's whole life, so a child
//! that never reads stdin fails it whatever else it did — which is why the write's failure is the
//! LAST thing `classify.rs` consults, never the first (R5-2).
//!
//! **The three-way race** (§7c.10; RT-3 as amended). `rk_ports::CancelToken` is an
//! `Arc<AtomicBool>` with `cancel()` / `is_cancelled()` and **no cause, no timeout arm, no
//! wake-up**, ONE object cloned in at construction (ADR-0099 decision 2) — so the adapter never
//! fires it (firing it on one dispatch's expiry would cancel every later one) and cannot be woken
//! by it. `run` therefore races **the child's exit** (`spawn_blocking(move || child.wait())` —
//! the stage's first arm, not the stdout drain: a child that exits leaving a descendant holding
//! its pipes is decided by its exit and its frame, and the leftover is reaped through the group
//! so the pipes close; a child that closes stdout and lives on stays under the token and the
//! timer to its bound), a `tokio::time::interval(CANCEL_POLL_MS)` poll of `token.is_cancelled()`,
//! and its own `tokio::time::timeout(spec.timeout_secs)` over that poll — while the drain task
//! runs beside them and is polled FIRST (`biased;`), so an init-time decision it takes
//! (`init.rs`) is acted on before the exit is read and a completed drain is never lost to a
//! same-turn cancel by the coin flip an unbiased `select!` takes (S5-05 / F7). **The `by` rule**:
//! the token arm → `Cancelled { by: Signal }`; the timer arm → `Cancelled { by: Timeout }`;
//! `Operator` unreachable at 7c. On either arm the child tree is killed **through the group**
//! (never `Child::kill`, which reaches the direct child alone), on a blocking thread because
//! `Group::kill` waits for every member to be gone. A token that has already fired when
//! `dispatch` is entered never reaches this module: `lib.rs` refuses before spawn (R5-5).
//! `tokio::time::sleep` and `std::thread::sleep` are called nowhere (`clippy.toml`); `interval` is
//! neither. [`crate::command::CANCEL_POLL_MS`] is the poll period — 50 ms, chosen at this slice
//! and UNCONFIRMED by any record (RT-3's row).
//!
//! **After the race**, whichever arm fired: on the exit arm the group is killed once to reap any
//! descendant the child left (so an inherited pipe closes and the readers return — AC-278's
//! shape); on a cancel arm or an init-time decision the group is killed and the exit awaited under
//! [`EXIT_GRACE_MS`]. Then the drain, the stderr reader and the stdin writer are each joined under
//! the same grace: a task that does not complete within it is aborted and the fact retained as a
//! marker line (never a hang on a pipe a survivor holds — F2). **Every `Group::kill` result is
//! kept** ([`Run::kill_error`]; R5-4 / S5-11): a kill that did not complete is a marker line on
//! the retained stderr and a count on the harness, never a discarded `Result`. Then `classify.rs`
//! decides. **`EXIT_GRACE_MS` is the second value chosen at this slice and UNCONFIRMED by any
//! record**, beside `CANCEL_POLL_MS`: it bounds the post-exit drain and joins and the post-kill
//! wait, never the child's own life (that is `spec.timeout_secs`, the stage's bound).
//!
//! **What the drain accumulates** ([`Drained`]): the line classification, the LAST reading and
//! the last non-`allowed` one, the raw `system/init` line and the raw result frame (each ONE
//! line, kept whole for the live probe's retention — RT-18 places the raw init on the probe's own
//! output file, and the row's stdout is capped), the `api_retry` count, the init decision, and a
//! reading the scanner REFUSED (a third decimal, one window of two): the refusal is recorded and
//! the drain runs on to EOF — the row keeps the frame's usage and cost and `classify.rs` turns the
//! refusal into `Transport { kind: Decode }` after the frame-keyed steps (F4 / S5-09: fail closed,
//! without a mid-run kill of a paid child). Stdout and stderr are accumulated in
//! [`rk_types::retention::RetainedSink`]s — verbatim to the cap, then digest + head/tail — so a
//! child that spews bounds the adapter's memory by a constant (F11 / R5-8).
//!
//! **Halt 4** — the cancel tests read the child's stdout digest before and after the kill and
//! compare nothing across it: what this module retains on a cancel arm is what the drain read
//! before the kill plus what the pipe still held (the drain runs to EOF after the kill, under
//! the grace), and no reading is taken after.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rk_containment::{ContainError, Contained, Group, KillDropOrder, spawn_into_job};
use rk_ports::cancel::CancelToken;
use rk_types::digest::{Digest, sha256};
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::retention::{Retained, RetainedSink};
use rk_types::typed::{SchemaId, Typed};
use rk_types::usage::{CostUsd, DetectedBy, Usage};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

use crate::classify::{self, Evidence};
use crate::command::{Assembled, CANCEL_POLL_MS};
use crate::config::MismatchPolicy;
use crate::env::ChildEnv;
use crate::event::{self, EventKind};
use crate::headroom::{self, RateLimitReading};
use crate::init::{self, Decision};

/// The grace that bounds what happens AFTER the race is decided (the module doc).
///
/// The post-exit drain and the three task joins, and the post-kill exit wait. It never bounds the
/// child's own life — that is `spec.timeout_secs`. Ten seconds is far above what an exited
/// process needs. **UNCONFIRMED by any record**, chosen at this slice beside `CANCEL_POLL_MS`
/// (RT-3's row).
pub const EXIT_GRACE_MS: u64 = 10_000;

/// Everything one run takes: the assembled command, the constructed environment, the worktree,
/// the bound, and what the init decisions compare against.
#[derive(Debug)]
pub struct Prepared<'a> {
    pub assembled: Assembled,
    pub env: ChildEnv,
    pub worktree: PathBuf,
    pub timeout: Duration,
    pub expected_agent: &'a str,
    /// The `--model` element the builder passed — the bound card's model (AC-212). *(dated
    /// 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.3.)* The FULL id of
    /// the pack's `[models]` row, which the reported model must EQUAL (RT-27 settled by equality,
    /// ADR-0197, proposed).
    pub expected_model: &'a str,
    /// The bound card's permission mode — what the init's `permissionMode` must equal (`s8h`
    /// §8h.5), never the argv's spelling of it (D21-48 X12).
    pub expected_permission_mode: &'a str,
    pub declared_tools: &'a [String],
    /// The spec's worktree, which a relative denied path resolves against and below which a
    /// protected segment names the known limit (`s8h` §8h.5; D21-45).
    pub worktree_root: PathBuf,
    pub schema_id: &'a SchemaId,
    pub schema: &'a serde_json::Value,
    pub pinned: &'a str,
    pub policy: MismatchPolicy,
}

/// The raw `system/init` line and the raw result frame of one run, each whole (the live probe's
/// retention seam — RT-18; `ClaudeHarness::last_raw_frames`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawFrames {
    pub init: Option<String>,
    pub result: Option<String>,
}

/// Everything one run produced — what `lib.rs` builds the record from.
#[derive(Debug)]
pub struct Run {
    pub result: Result<Typed, StepError>,
    pub detected_by: Option<DetectedBy>,
    pub cancelled_by: Option<CancelledBy>,
    /// What the child wrote to stdout, as the record retains it (verbatim to the cap — AC-200's
    /// carrier — then digest + head/tail).
    pub stdout: Retained,
    pub stderr: Retained,
    /// The raw init line and the raw result frame, whole (the module doc).
    pub raw_frames: RawFrames,
    pub usage: Option<Usage>,
    pub total_cost_usd: Option<CostUsd>,
    pub api_ms: Option<u64>,
    /// The LAST representable `rate_limit_event` of this stream (`headroom_after`; the guard's
    /// next reading) — `None` when a reading was refused (the guard learns nothing from a run
    /// whose telemetry it could not represent).
    pub last_reading: Option<RateLimitReading>,
    /// The CLI version the child's `system/init` reported, if it reported one.
    pub observed_version: Option<String>,
    /// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.3.)* The
    /// model the child's `system/init` reported, exactly as printed; `None` on every path that
    /// terminated before an init carrying `model` was read. Never the requested id copied in.
    pub observed_model: Option<String>,
    /// `system/api_retry` events counted (AC-232).
    pub api_retry_events: u32,
    /// *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.5.)* The
    /// classifier's protected-segment notes (D21-45's known limit), for the adapter to print.
    pub notes: Vec<classify::ProtectedSegmentDenial>,
    /// A kill through the group that did not complete, rendered (R5-4 / S5-11).
    pub kill_error: Option<String>,
    /// The group's kill-then-drop probe (test support, `#[doc(hidden)]` on the harness).
    pub kill_drop_order: Option<Arc<KillDropOrder>>,
}

/// What the init decisions compare against — the owned copy the drain task carries (a spawned
/// task is `'static`; `Prepared` borrows).
#[derive(Debug, Clone)]
struct Expect {
    pinned: String,
    policy: MismatchPolicy,
    expected_agent: String,
    expected_model: String,
    expected_permission_mode: String,
    declared_tools: Vec<String>,
}

/// What the drain accumulates while the child speaks. **No `Default`** (RT-8; the house rule
/// `throttle::Guard::empty` states): the accumulator holds `usage` and `total_cost_usd`, and an
/// empty one is constructed through [`Drained::empty`] naming every member.
#[derive(Debug)]
struct Drained {
    stdout: RetainedSink,
    init_seen: bool,
    init_raw: Option<String>,
    result_raw: Option<String>,
    usage: Option<Usage>,
    total_cost_usd: Option<CostUsd>,
    api_ms: Option<u64>,
    last_reading: Option<RateLimitReading>,
    last_non_allowed: Option<RateLimitReading>,
    observed_version: Option<String>,
    observed_model: Option<String>,
    api_retry_events: u32,
    /// Every `system` / `permission_denied` line, raw (guard 3's channel (b), `s8h` §8h.5).
    denial_events: Vec<String>,
    /// An init-time refusal (`init::decide`).
    decided: Option<StepError>,
    /// A `rate_limit_event` the scanner refused (a third decimal, one window of two), rendered
    /// with its key path — recorded, and the drain runs on (F4 / S5-09).
    reading_refused: Option<String>,
}

impl Drained {
    fn empty() -> Self {
        Self {
            stdout: RetainedSink::empty(),
            init_seen: false,
            init_raw: None,
            result_raw: None,
            usage: None,
            total_cost_usd: None,
            api_ms: None,
            last_reading: None,
            last_non_allowed: None,
            observed_version: None,
            observed_model: None,
            api_retry_events: 0,
            denial_events: Vec::new(),
            decided: None,
            reading_refused: None,
        }
    }
}

/// Which arm of the race fired.
enum Arm {
    /// The child exited; the drain may or may not have reached EOF.
    Exited(Option<i32>),
    /// The drain took an init-time decision: the child is killed.
    Decided,
    Token,
    Timer,
}

/// Build the child's command: program, the golden argv, the worktree as cwd, `env_clear()` then
/// the constructed map, three piped handles. Handed to `rk-containment`, never spawned here.
#[expect(
    clippy::disallowed_types,
    reason = "ADR-0127 decision 3 / RT-13 as amended: the adapter BUILDS the std Command and hands it to \
              rk_containment::spawn_into_job, which owns the one spawn site; clippy.toml's \
              disallowed-types has no path scoping, so the exemption is stated at the narrowest item."
)]
fn build_command(prepared: &Prepared<'_>) -> std::process::Command {
    let mut cmd = std::process::Command::new(&prepared.assembled.program);
    cmd.args(&prepared.assembled.argv)
        .current_dir(&prepared.worktree)
        .env_clear()
        .envs(prepared.env.as_map())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    cmd
}

fn spawn_error(e: &ContainError) -> StepError {
    StepError::Transport {
        kind: TransportKind::Spawn,
        detail: e.to_string(),
    }
}

/// A run that never reached a live child: the failure, and nothing read.
const fn failed_before_stream(err: StepError) -> Run {
    Run {
        result: Err(err),
        detected_by: None,
        cancelled_by: None,
        stdout: Retained::Verbatim(Vec::new()),
        stderr: Retained::Verbatim(Vec::new()),
        raw_frames: RawFrames {
            init: None,
            result: None,
        },
        usage: None,
        total_cost_usd: None,
        api_ms: None,
        last_reading: None,
        observed_version: None,
        observed_model: None,
        api_retry_events: 0,
        notes: Vec::new(),
        kill_error: None,
        kill_drop_order: None,
    }
}

/// Lock a shared accumulator. **A poisoned lock is never recovered** (ADR-0118's principle, applied
/// to the adapter's own locks as the block's ban spells it — no `into_inner` in any spelling): the
/// holder panicked mid-update, the data behind it is not trusted, and the caller fails closed —
/// the drain and the stderr reader stop, and the dispatch classifies `Transport { Decode }` with
/// its row (AC-360 stands). `pub` and hidden because the refusal's own test drives it directly.
#[doc(hidden)]
pub fn locked<T>(m: &Mutex<T>) -> Result<std::sync::MutexGuard<'_, T>, String> {
    m.lock().map_err(|_| {
        "a shared accumulator's lock is poisoned: its holder panicked mid-update, and ADR-0118          forbids recovering it in any spelling — the dispatch fails closed with its row"
            .to_owned()
    })
}

/// The drain: read stdout to EOF, retaining every byte, classifying every line, taking the init
/// decisions on the init line and keeping the last reading. Returns early on an init decision;
/// a refused reading is recorded and the drain continues.
async fn drain_stdout(
    stdout: tokio::process::ChildStdout,
    expect: Expect,
    state: Arc<Mutex<Drained>>,
) {
    let mut reader = BufReader::new(stdout);
    let mut buf: Vec<u8> = Vec::new();
    loop {
        buf.clear();
        let n = match reader.read_until(b'\n', &mut buf).await {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        let Ok(mut st) = locked(&state) else { break };
        st.stdout.push(&buf[..n]);
        let mut line = &buf[..n];
        if line.last() == Some(&b'\n') {
            line = &line[..line.len() - 1];
        }
        if line.last() == Some(&b'\r') {
            line = &line[..line.len() - 1];
        }
        if line.iter().all(u8::is_ascii_whitespace) {
            continue;
        }
        let text = String::from_utf8_lossy(line);
        let parsed = event::classify(&text);
        match &parsed.kind {
            EventKind::Result => {
                st.usage = event::usage_from_result(&parsed.raw);
                st.total_cost_usd = headroom::cost_usd_from_result(parsed.raw.as_bytes());
                st.api_ms = headroom::api_ms_from_result(parsed.raw.as_bytes());
                st.result_raw = Some(parsed.raw);
            }
            EventKind::SystemInit => {
                // A `system/init` line the classifier recognised is a JSON object with the two
                // keys; `effective_config` re-reads it and can only refuse a non-object, which the
                // classifier never hands here — so a `None` leaves `init_seen` false and a later
                // result frame is refused for it (S5-01).
                let Some(cfg) = init::effective_config(&parsed.raw) else {
                    continue;
                };
                st.init_seen = true;
                st.init_raw = Some(parsed.raw.clone());
                st.observed_version.clone_from(&cfg.version);
                // *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h`
                // §8h.3.)* The REPORTED model, read before any decision so a refused path that
                // read the init carries it too; never the requested id.
                st.observed_model.clone_from(&cfg.model);
                match init::decide(
                    &cfg,
                    &init::Expected {
                        pinned: &expect.pinned,
                        policy: expect.policy,
                        agent: &expect.expected_agent,
                        model: &expect.expected_model,
                        permission_mode: &expect.expected_permission_mode,
                        declared_tools: &expect.declared_tools,
                    },
                ) {
                    Decision::Proceed { .. } => {}
                    Decision::Refuse(err) => {
                        st.decided = Some(err);
                        return;
                    }
                }
            }
            EventKind::ApiRetry => st.api_retry_events += 1,
            // Keep the LAST one: it is the closest reading to the dispatch's true cost (lifted).
            EventKind::RateLimitEvent => {
                match headroom::read_rate_limit_event(parsed.raw.as_bytes()) {
                    Ok(reading) => {
                        if !reading.is_allowed() {
                            st.last_non_allowed = Some(reading.clone());
                        }
                        st.last_reading = Some(reading);
                    }
                    Err(e) => {
                        // A reading the instrument cannot represent (a third decimal, one window of
                        // two) is refused, never rounded or ignored: fail closed (AC-374, AC-375) —
                        // recorded here, the drain continues, classify.rs decides (F4 / S5-09).
                        // A shape with no utilisation at all is not a reading and not a failure
                        // (AC-201: an unknown shape alone raises nothing); the line is retained.
                        if e != headroom::HeadroomError::NoUtilization
                            && st.reading_refused.is_none()
                        {
                            st.reading_refused = Some(format!("rate_limit_event: {e}"));
                        }
                    }
                }
            }
            // *(dated 2026-10-02, s8h build, package P4-adapter-guards-and-probes; `s8h` §8h.5.)*
            // Guard 3's channel (b): every `permission_denied` system event, raw, for classify.rs.
            EventKind::System(subtype) if subtype == classify::PERMISSION_DENIED_SUBTYPE => {
                st.denial_events.push(parsed.raw.clone());
            }
            EventKind::System(_)
            | EventKind::Assistant
            | EventKind::User
            | EventKind::Unknown(_)
            | EventKind::Malformed => {}
        }
    }
}

/// Kill through the group on a blocking thread (`Group::kill` waits for every member); the
/// result is RETURNED, never discarded (R5-4).
async fn kill(group: Arc<Group>) -> Result<(), String> {
    match tokio::task::spawn_blocking(move || group.kill()).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("the kill through the group did not complete: {e}")),
        Err(join) => Err(format!("the kill task did not complete: {join}")),
    }
}

/// Join a task under the grace; on expiry abort it and say so.
async fn join_bounded<T>(task: &mut tokio::task::JoinHandle<T>, what: &str) -> Result<T, String> {
    match tokio::time::timeout(Duration::from_millis(EXIT_GRACE_MS), &mut *task).await {
        Ok(Ok(v)) => Ok(v),
        Ok(Err(join)) => Err(format!("the {what} task did not complete: {join}")),
        Err(_elapsed) => {
            task.abort();
            Err(format!(
                "the {what} task did not complete within {EXIT_GRACE_MS} ms and was abandoned"
            ))
        }
    }
}

/// Run one prepared dispatch to its termination (the module doc).
pub async fn run(prepared: Prepared<'_>, token: &CancelToken) -> Run {
    // ---- spawn, through rk-containment
    let Contained { mut child, group } = match spawn_into_job(build_command(&prepared)) {
        Ok(c) => c,
        Err(e) => return failed_before_stream(spawn_error(&e)),
    };
    let group = Arc::new(group);
    let kill_drop_order = Some(group.kill_drop_order());

    // ---- the three pipes, each wrapped for tokio
    let pipes = (|| {
        let stdin = tokio::process::ChildStdin::from_std(child.stdin.take()?).ok()?;
        let stdout = tokio::process::ChildStdout::from_std(child.stdout.take()?).ok()?;
        let stderr = tokio::process::ChildStderr::from_std(child.stderr.take()?).ok()?;
        Some((stdin, stdout, stderr))
    })();
    let Some((mut stdin, stdout, mut stderr)) = pipes else {
        let kill_error = kill(Arc::clone(&group)).await.err();
        let _ = tokio::time::timeout(
            Duration::from_millis(EXIT_GRACE_MS),
            tokio::task::spawn_blocking(move || child.wait()),
        )
        .await;
        return Run {
            kill_drop_order,
            kill_error,
            ..failed_before_stream(StepError::Transport {
                kind: TransportKind::Io,
                detail: "the child's pipes could not be taken and registered with the runtime"
                    .to_owned(),
            })
        };
    };

    // ---- the prompt on stdin: write, FLUSH (the module doc), then EOF by the drop
    let payload = prepared.assembled.stdin.clone().into_bytes();
    let mut stdin_task = tokio::spawn(async move {
        stdin.write_all(&payload).await?;
        stdin.flush().await?;
        drop(stdin);
        Ok::<(), std::io::Error>(())
    });
    // ---- stderr on its own task, into a shared bounded sink (readable even if the task is
    // abandoned)
    let stderr_sink: Arc<Mutex<RetainedSink>> = Arc::new(Mutex::new(RetainedSink::empty()));
    let mut stderr_task = {
        let sink = Arc::clone(&stderr_sink);
        tokio::spawn(async move {
            let mut buf = vec![0u8; 8 * 1024];
            loop {
                match stderr.read(&mut buf).await {
                    Ok(0) | Err(_) => break,
                    Ok(n) => match locked(&sink) {
                        Ok(mut s) => s.push(&buf[..n]),
                        Err(_) => break,
                    },
                }
            }
        })
    };
    // ---- the drain on its own task, into the shared accumulator
    let state: Arc<Mutex<Drained>> = Arc::new(Mutex::new(Drained::empty()));
    let mut drain_task = {
        let expect = Expect {
            pinned: prepared.pinned.to_owned(),
            policy: prepared.policy,
            expected_agent: prepared.expected_agent.to_owned(),
            expected_model: prepared.expected_model.to_owned(),
            expected_permission_mode: prepared.expected_permission_mode.to_owned(),
            declared_tools: prepared.declared_tools.to_vec(),
        };
        tokio::spawn(drain_stdout(stdout, expect, Arc::clone(&state)))
    };
    let mut drain_done: Option<Result<(), String>> = None;

    // ---- the race: the child's exit | the token poll | the adapter's own timer — the drain
    // polled first (biased) so an init decision is acted on and a completed drain never loses a
    // same-turn coin flip
    let mut wait = tokio::task::spawn_blocking(move || child.wait());
    let mut poll = tokio::time::interval(Duration::from_millis(CANCEL_POLL_MS));
    poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let watch = async {
        loop {
            poll.tick().await;
            if token.is_cancelled() {
                return;
            }
        }
    };
    let timer = tokio::time::timeout(prepared.timeout, watch);
    tokio::pin!(timer);
    let arm = loop {
        tokio::select! {
            biased;
            joined = &mut drain_task, if drain_done.is_none() => {
                drain_done = Some(joined.map_err(|j| format!("the stdout drain task did not complete: {j}")));
                if locked(&state).map(|g| g.decided.is_some()).unwrap_or(true) {
                    break Arm::Decided;
                }
                // EOF with no decision: the child may still be alive — keep racing its exit
            }
            joined = &mut wait => break Arm::Exited(match joined {
                Ok(Ok(s)) => s.code(),
                Ok(Err(_)) | Err(_) => None,
            }),
            fired = &mut timer => break match fired {
                Ok(()) => Arm::Token,
                Err(_elapsed) => Arm::Timer,
            },
        }
    };
    let cancelled_by = match arm {
        Arm::Exited(_) | Arm::Decided => None,
        Arm::Token => Some(CancelledBy::Signal),
        Arm::Timer => Some(CancelledBy::Timeout),
    };

    // ---- after the race: the kill through the group on EVERY arm (the cancel arms' kill; the
    // exit arm's reap of leftovers), then the exit under the grace where it was not yet read
    let mut kill_error: Option<String> = kill(Arc::clone(&group)).await.err();
    let exit_code = match arm {
        // reap anything the child left, so an inherited pipe closes (AC-278's shape)
        Arm::Exited(code) => code,
        // the kill above was the cancel's; now the exit, under the grace
        Arm::Decided | Arm::Token | Arm::Timer => {
            match tokio::time::timeout(Duration::from_millis(EXIT_GRACE_MS), &mut wait).await {
                Ok(Ok(Ok(s))) => s.code(),
                Ok(Ok(Err(_)) | Err(_)) => None,
                Err(_elapsed) => {
                    // the child did not exit after the kill: the wait thread is abandoned (it
                    // returns when the child finally dies) and the fact is retained
                    kill_error.get_or_insert_with(|| {
                        format!(
                            "the child did not exit within {EXIT_GRACE_MS} ms of the kill through \
                             the group"
                        )
                    });
                    None
                }
            }
        }
    };

    // ---- the three joins, each under the grace (F2)
    let drain_join = match drain_done {
        Some(r) => r,
        None => join_bounded(&mut drain_task, "stdout drain").await,
    };
    let stderr_join = join_bounded(&mut stderr_task, "stderr reader").await;
    let stdin_error = match join_bounded(&mut stdin_task, "stdin writer").await {
        Ok(Ok(())) => None,
        Ok(Err(e)) => Some(e.to_string()),
        Err(detail) => Some(detail),
    };

    // ---- what was read, and the markers for what did not complete
    let (drained, poisoned) = match locked(&state) {
        Ok(mut g) => (std::mem::replace(&mut *g, Drained::empty()), None),
        Err(detail) => (Drained::empty(), Some(detail)),
    };
    let (mut stderr_sink, poisoned) = match locked(&stderr_sink) {
        Ok(mut g) => (std::mem::replace(&mut *g, RetainedSink::empty()), poisoned),
        Err(detail) => (RetainedSink::empty(), poisoned.or(Some(detail))),
    };
    for marker in [
        drain_join.err(),
        stderr_join.err(),
        kill_error.clone(),
        poisoned.clone(),
    ]
    .into_iter()
    .flatten()
    {
        stderr_sink.push(format!("\n<rk-adapter-claude: {marker}>\n").as_bytes());
    }
    if let Some(detail) = &stdin_error {
        stderr_sink.push(
            format!("\n<rk-adapter-claude: the prompt could not be written to the child's stdin: {detail}>\n")
                .as_bytes(),
        );
    }
    let stderr_retained = stderr_sink.finish();
    let (stderr_text, stderr_digest): (Vec<u8>, Digest) = match &stderr_retained {
        Retained::Verbatim(b) => (b.clone(), sha256(b)),
        Retained::Capped {
            head, tail, digest, ..
        } => {
            let mut v = head.clone();
            v.extend_from_slice(tail);
            (v, digest.clone())
        }
    };

    let mut classified = classify::classify(&Evidence {
        cancelled: cancelled_by,
        decided: drained.decided.clone(),
        init_seen: drained.init_seen,
        declared_tools: prepared.declared_tools,
        reading_refused: drained.reading_refused.as_deref(),
        stdin_error,
        exit_code,
        result_raw: drained.result_raw.as_deref(),
        stderr: &stderr_text,
        stderr_digest,
        last_non_allowed: drained.last_non_allowed.as_ref(),
        schema_id: prepared.schema_id,
        schema: prepared.schema,
        denial_events: &drained.denial_events,
        worktree: Some(&prepared.worktree_root),
    });
    // ADR-0118's principle: a poisoned accumulator is a holder that panicked mid-update; the data
    // behind it is not trusted, so the dispatch fails closed — WITH its row (AC-360 stands).
    if let Some(detail) = poisoned {
        classified.result = Err(StepError::Transport {
            kind: TransportKind::Decode,
            detail,
        });
    }
    drop(group);
    Run {
        result: classified.result,
        detected_by: classified.detected_by,
        cancelled_by,
        stdout: drained.stdout.finish(),
        stderr: stderr_retained,
        raw_frames: RawFrames {
            init: drained.init_raw,
            result: drained.result_raw,
        },
        usage: drained.usage,
        total_cost_usd: drained.total_cost_usd,
        api_ms: drained.api_ms,
        last_reading: if drained.reading_refused.is_some() {
            None
        } else {
            drained.last_reading
        },
        observed_version: drained.observed_version,
        observed_model: drained.observed_model,
        api_retry_events: drained.api_retry_events,
        notes: classified.notes,
        kill_error,
        kill_drop_order,
    }
}
