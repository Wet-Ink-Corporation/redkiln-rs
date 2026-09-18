//! `rk-adapter-claude` — the real `Harness` row (design §9.1; ADR-0094 decision 1's first adapter;
//! `s7c` §7c.2–§7c.12). **Slice 4 was part A, configuration time; Slice 5 is part B, the
//! runtime** (§7c.20 items 4 and 5): the stream is consumed inside, every termination appends one
//! row, and headroom is integer ticks.
//!
//! **What this crate is** (ADR-0099: `dispatch(&self, spec: &ActivitySpec, store: &mut dyn
//! EventStore) -> DispatchOutcome`, blocking to completion; the `CancelToken` cloned in at
//! construction; *"the kernel never calls cancel"*): one `claude -p` child per dispatch, spawned
//! through `rk-containment` (ADR-0127), fed the golden argv and the constructed environment,
//! its stream consumed inside, every termination appending one `dispatch_completed` row
//! (ADR-0126). Constructed once at the composition root, never per call (design §9.1), as
//! [`ClaudeHarness::new`] — `(config: Config, token: CancelToken) -> Result<Self, ConfigError>`.
//!
//! **Module by module** — part A's configuration-time lints (each a typed [`config::ConfigError`]
//! that **never crosses the port**: ADR-0112 closes `StepError` at eleven, and a `ConfigError` is
//! the return of `Config::new` / `Binder::bind` and never of `dispatch`), then part B's runtime:
//!
//! | module | what it holds | criteria |
//! | --- | --- | --- |
//! | [`config`] | `Config` (the pins, the TTL pair, `Margins`, the captured map, `harness_binary`, the mismatch policy, the digest-keyed card store only the binder fills) as a VALUE (ADR-0083); `Config::new`'s lints; `ConfigError`; `TESTED_CLI_VERSION` (D17-11) | AC-217, AC-488, AC-526, AC-377/AC-220, AC-226 (config arms) |
//! | [`env_denylist`] | the ONE committed list (decision #16 of 2026-09-11; RT-25's tracker names) | AC-217, AC-422, AC-488, AC-509 |
//! | [`env`] | the child environment: the captured map plus both TTLs, one value — a refusal upstream, never a subtraction here | AC-210, AC-385, AC-526's AND |
//! | [`card`] | the lift from `runner/src/step.rs` at the pin: `Tier`, `Effort`, `AgentCard`; the authored card record and its validator | AC-493, AC-383, AC-214, AC-118/AC-418, AC-269 |
//! | [`command`] | ONE builder, the golden argv in ONE order, per-profile delivery, the argv lints and the cap (`ARGV_CAP_BYTES`, RT-1); `CANCEL_POLL_MS` beside it (RT-3) | AC-203, AC-205/AC-417, AC-132, AC-209, AC-211 (PROVISIONAL), AC-214, AC-215 (argv half), AC-216/AC-511, AC-523's cap, AC-383 |
//! | [`binder`] | D-11: role tier → bound card → `ActivitySpec`; owns the `Config` while it fills its card store; RT-21's `"worktree"` key | AC-212, AC-204's declared list |
//! | [`registry`] | the dated capability registry — the `dispatch` row, and nothing else | AC-238, AC-489, AC-492 (RT-24) |
//! | [`event`] | the lift from `runner/src/event.rs`: the line classifier, `raw` as the line's bytes, nothing refused | AC-200, AC-201 |
//! | [`headroom`] | NEW: the structural digit scanner over `StreamLine.raw` — `u16` ticks, both `rate_limit_event` shapes, the cost digits verbatim | AC-374, AC-375, AC-380, AC-219, AC-221, RT-2, RT-19 |
//! | [`throttle`] | the lift from `runner/src/throttle.rs`: `Guard::check` at `ticks >= 100 - margin_ticks`; `batch_draw` REWRITTEN in ticks | AC-376, AC-220, AC-372, AC-381 (O-7) |
//! | [`init`] | `system/init` → effective config as SET DIGESTS; adoption on equality; the observed version against the pin | AC-202, AC-204, AC-386, AC-226, AC-385 |
//! | [`classify`] | every termination onto the closed ELEVEN, exhaustively, keyed on the observation | AC-229, AC-218, AC-206, AC-208, AC-523, AC-225 |
//! | [`ttl`] | the five-state TTL classifier over `usage.cache_creation` | ADR-0113 |
//! | [`record`] | `dispatch_completed` on every path; `mismatch_warning` derived on read; the AC-368 warning counter; the journal's two principles | AC-360, AC-368, AC-385, AC-227 |
//! | [`runner`] | the lift from `runner/src/runner.rs`: the spawn through `rk-containment`, the drain, the three-way race, the kill through the group | AC-224, AC-225, AC-215, AC-232, AC-509, RT-13, RT-3 |
//!
//! **`dispatch`, path by path** ([`ClaudeHarness::dispatch`]; §7c.11's list, every one ending
//! in [`record::append_completed`] through the store handed in — ADR-0126's forbids):
//!
//! - **pre-spawn, `cli_version: Pinned`** (no child reported one): a spec whose `inputs` lack a
//!   string `"worktree"`, or carry one that is RELATIVE or not a directory (a relative value
//!   would make `current_dir` resolve against the process cwd — the source RT-21 and AC-529
//!   forbid; S5-10) → `Transport { kind: Spawn }` (RT-21; RT-20's sibling); a spec whose
//!   `card_digest` names no bound card, or whose `schema` the entry was not bound with →
//!   `Transport { kind: Spawn }` (RT-20 as amended); the guard's `HaltOverage` → `Overage`
//!   (AC-221, AC-378); the guard's `Halt` → `RateLimited { resets_at }` (AC-376, AC-220) with
//!   `headroom_before` the reading that fired — the reading and the verdict are taken under ONE
//!   lock acquisition, so the row's `headroom_before` IS the reading the guard decided on
//!   (S5-07); a token that has ALREADY fired → `Cancelled { by: Signal }` with nothing spawned
//!   (R5-5 / F6 / S5-06: after Ctrl-C at the root no further `claude -p` is launched to be
//!   killed on the first poll tick); a spawn failure → `Transport { kind: Spawn }`.
//! - **through the child** (`runner::run`): `Ok(Typed)`; `Schema`; `Transport { Io | Decode }`;
//!   `NonZero`; `Platform`; `NotAdopted`; `NotIsolated`; `Cancelled { by: Signal | Timeout }` —
//!   each with `cli_version: Observed(v)` when the init reported a version and `Pinned` when
//!   none was read (a kill before init, an init with no version key — `init.rs`).
//! - **a panic on the runtime path** (R5-6): `runner::run` and `classify` run on a scoped thread
//!   (below); a panic there is caught at the join and becomes `Transport { kind: Decode }` with
//!   its row, never an unwind out of `dispatch` that appends nothing (AC-360). The child: the
//!   group `Arc` drops during the unwind — the `KILL_ON_JOB_CLOSE` sweep on Windows, nothing on
//!   POSIX (a leak the POSIX arm's CI run measures). No panicking site exists on the drain path
//!   by inspection; this is the belt behind that.
//! - **the two undriven arms**, `Unsupported` and `Timeout { after_secs }`, are produced by no
//!   path (§7c.11) and exist in `classify.rs`'s exhaustive match only.
//!
//! **The user message** (§7c.12: *"how Slice 5 renders [the inputs] into the stdin payload is
//! that slice's"*): the input under [`PROMPT_KEY`] (`"prompt"`) when it is a string, verbatim;
//! otherwise the inputs minus the worktree key, serialised as one compact JSON object — routed
//! here, a record naming the input contract is OWED beside RT-21's.
//!
//! **The runtime** (ADR-0076; RT-15): one `tokio::runtime::Runtime`, `multi_thread`, built once
//! in `new()` and owned by the adapter; `dispatch` calls `block_on` from the sync port method and
//! nothing else does — never `Handle::block_on` from a thread the runtime drives (a
//! `current_thread` runtime would freeze the spawned stderr reader between `block_on` calls). The
//! `block_on` runs on a **scoped `std::thread`** the dispatch spawns and joins (R5-9): a
//! `Runtime::block_on` from a thread inside ANOTHER tokio runtime panics (*"Cannot start a runtime
//! from within a runtime"*), and the sync port makes no promise about its caller's thread; a
//! fresh thread has no runtime context, so the port is callable from any composition root. The
//! adapter is `Send + Sync`, asserted statically below (F41 Limit 1). The throttle's [`Guard`],
//! the AC-368 counter and the kill-warning counter sit behind mutexes (a poisoned lock is a halt
//! — ADR-0118). **Dispatch is sequential per harness in the kernel's use**: two concurrent
//! dispatches through one harness both pass the guard on the same reading (the first's reading
//! is observed only when it returns) — a stated property, not a serialised critical section,
//! because an outer per-dispatch mutex would hold every parallel caller behind one child's whole
//! life (R5-9).
//!
//! **Lint posture.** Every override in this crate is `#[expect(clippy::…, reason = "…")]` per
//! site (ADR-0102 decision 3): the ONE `std::process::Command` site (`runner::build_command`),
//! the TWO `Instant::now` sites (here: `started` at the top of `dispatch` and `ended` inside
//! [`wall_ms_since`], the ONE function every return path calls — so `wall_ms` on the pre-spawn
//! rows and on the run row is bounded by the same pair, and no third read such as
//! `Instant::elapsed` exists, S5-14 — §7c.19). No `HashMap`,
//! no sleep, no environment read in any `src/` file (the block's AC-217 ban — the captured map is
//! a `Config` value), no `f64` in the headroom path (RT-2). `thiserror` for the error enums;
//! `anyhow` nowhere. `jsonschema` is consumed here (`classify.rs`) on the workspace row D17-12
//! put at `default-features = false`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use rk_ports::cancel::CancelToken;
use rk_ports::event_store::EventStore;
use rk_ports::harness::{DispatchOutcome, Harness};
use rk_types::activity::{ActivitySpec, DispatchRecord, RecordTail};
use rk_types::error::{CancelledBy, StepError, TransportKind};
use rk_types::retention::Retained;
use rk_types::usage::{CliVersion, Timings, WindowReading};

pub mod binder;
pub mod card;
pub mod classify;
pub mod command;
pub mod config;
pub mod env;
pub mod env_denylist;
pub mod event;
pub mod headroom;
pub mod init;
pub mod record;
pub mod registry;
pub mod runner;
pub mod throttle;
pub mod ttl;

use crate::binder::WORKTREE_KEY;
use crate::command::{CommandSpec, assemble};
use crate::config::{Config, ConfigError};
use crate::env::{PROMPT_CACHE_TTL, SUBAGENT_PROMPT_CACHE_TTL};
use crate::record::AppendWarnings;
use crate::runner::RawFrames;
use crate::throttle::{Guard, Verdict};

/// The input key carrying the user message (the module doc: routed, a record OWED).
pub const PROMPT_KEY: &str = "prompt";

/// The real `Harness` row: one `claude -p` per dispatch, through `rk-containment`, consumed
/// inside, one `dispatch_completed` per termination.
pub struct ClaudeHarness {
    runtime: tokio::runtime::Runtime,
    config: Config,
    token: CancelToken,
    registry: Vec<registry::Row>,
    guard: Mutex<Guard>,
    append_warnings: AppendWarnings,
    kill_warnings: Mutex<Vec<String>>,
    last_kill_drop_order: Mutex<Option<std::sync::Arc<rk_containment::KillDropOrder>>>,
    last_raw_frames: Mutex<Option<RawFrames>>,
}

impl std::fmt::Debug for ClaudeHarness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClaudeHarness")
            .field("config", &self.config)
            .field("registry_rows", &self.registry.len())
            .finish_non_exhaustive()
    }
}

/// F41 Limit 1: the adapter is `Send + Sync` — asserted at compile time, not believed.
const fn assert_send_sync<T: Send + Sync>() {}
const _: () = assert_send_sync::<ClaudeHarness>();

impl ClaudeHarness {
    /// Construct the adapter once, at the composition root: the validated `Config` (its card
    /// store filled by the binder), the root's `CancelToken` clone, the shipped registry, and
    /// the `multi_thread` runtime.
    ///
    /// # Errors
    /// [`ConfigError::Runtime`] when the tokio runtime cannot be built; [`ConfigError::Registry`]
    /// when the shipped registry does not load (a build defect `ac489_…` also reports).
    pub fn new(config: Config, token: CancelToken) -> Result<Self, ConfigError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("rk-adapter-claude")
            .build()
            .map_err(|e| ConfigError::Runtime {
                detail: e.to_string(),
            })?;
        let registry = registry::rows().map_err(|e| ConfigError::Registry {
            detail: e.to_string(),
        })?;
        Ok(Self {
            runtime,
            config,
            token,
            registry,
            guard: Mutex::new(Guard::empty()),
            append_warnings: AppendWarnings::default(),
            kill_warnings: Mutex::new(Vec::new()),
            last_kill_drop_order: Mutex::new(None),
            last_raw_frames: Mutex::new(None),
        })
    }

    /// The configuration the adapter was constructed with.
    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    /// The shipped registry rows (AC-238's dated `dispatch` entry).
    #[must_use]
    pub fn registry(&self) -> &[registry::Row] {
        &self.registry
    }

    /// The AC-368 warning counter: every `dispatch_completed` append the store refused.
    #[must_use]
    pub const fn append_warnings(&self) -> &AppendWarnings {
        &self.append_warnings
    }

    /// The throttle's last reading (the guard's), as the two-window reading — what the next
    /// dispatch's `headroom_before` will carry.
    ///
    /// # Panics
    /// On a poisoned guard lock (ADR-0118: a halt).
    #[must_use]
    pub fn last_reading(&self) -> Option<headroom::RateLimitReading> {
        self.guard
            .lock()
            .expect("the guard's lock is poisoned: a halt (ADR-0118)")
            .last()
            .cloned()
    }

    /// Every kill through the group that did not complete, in order (S5-11 / R5-4): a
    /// `Group::kill` that returned `Err` — a member outliving the bounded wait, an enumeration
    /// failure — is a fact about the run, retained on the row's stderr as a marker line and
    /// counted here, never discarded.
    ///
    /// # Panics
    /// On a poisoned lock (ADR-0118).
    #[must_use]
    pub fn kill_warnings(&self) -> Vec<String> {
        self.kill_warnings
            .lock()
            .expect("the kill-warning counter's lock is poisoned: a halt (ADR-0118)")
            .clone()
    }

    /// The raw `system/init` line and the raw result frame of the LAST run, each whole (S5-04;
    /// RT-18: the raw init goes on the probe's own output file, and the row's stdout is capped at
    /// `STDIO_CAP` — a `--verbose stream-json` dispatch with tool traffic readily exceeds it, and
    /// a paid run's frames may not be lost to the cap). `None` before any run reached a child.
    ///
    /// # Panics
    /// On a poisoned lock (ADR-0118).
    #[doc(hidden)]
    #[must_use]
    pub fn last_raw_frames(&self) -> Option<RawFrames> {
        self.last_raw_frames
            .lock()
            .expect("the raw-frames seam's lock is poisoned: a halt (ADR-0118)")
            .clone()
    }

    /// Test support: the kill-then-drop probe of the LAST spawned group (`s7c` §7c.10's
    /// `cancel_reaps_process_tree` asserts kill completed before the handle dropped).
    ///
    /// # Panics
    /// On a poisoned lock (ADR-0118).
    #[doc(hidden)]
    #[must_use]
    pub fn last_kill_drop_order(&self) -> Option<std::sync::Arc<rk_containment::KillDropOrder>> {
        self.last_kill_drop_order
            .lock()
            .expect("the probe's lock is poisoned: a halt (ADR-0118)")
            .clone()
    }

    /// The user message from the spec's inputs (the module doc's rule).
    fn user_message(spec: &ActivitySpec) -> String {
        if let Some((_, serde_json::Value::String(prompt))) =
            spec.inputs.iter().find(|(k, _)| k == PROMPT_KEY)
        {
            return prompt.clone();
        }
        let rest: serde_json::Map<String, serde_json::Value> = spec
            .inputs
            .iter()
            .filter(|(k, _)| k != WORKTREE_KEY)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        serde_json::Value::Object(rest).to_string()
    }

    /// The worktree from the spec's inputs (RT-21): a string under `"worktree"`, or `None`.
    fn worktree(spec: &ActivitySpec) -> Option<PathBuf> {
        spec.inputs.iter().find_map(|(k, v)| {
            (k == WORKTREE_KEY)
                .then(|| v.as_str().map(PathBuf::from))
                .flatten()
        })
    }

    /// Why a worktree value is refused before spawn (S5-10): a relative path would be resolved
    /// against the process cwd by `Command::current_dir` — the source RT-21 / AC-529 forbid — and
    /// a path that is not a directory cannot be a run-owned worktree.
    fn worktree_refusal(worktree: &Path) -> Option<String> {
        if !worktree.is_absolute() {
            return Some(format!(
                "the spec's {WORKTREE_KEY:?} input {} is not an absolute path: a relative value \
                 would resolve against the process cwd (RT-21; AC-529's clause); nothing was \
                 spawned",
                worktree.display()
            ));
        }
        if !worktree.is_dir() {
            return Some(format!(
                "the spec's {WORKTREE_KEY:?} input {} is not a directory (RT-21); nothing was \
                 spawned",
                worktree.display()
            ));
        }
        None
    }

    /// The milliseconds since `started` — the SECOND of the adapter's two `Instant::now` sites,
    /// the end of the pair that bounds `Timings.wall_ms` (s7c §7c.19), called once on every
    /// return path of `dispatch` and feeding no decision.
    fn wall_ms_since(started: Instant) -> u64 {
        #[expect(
            clippy::disallowed_methods,
            reason = "the SECOND of the adapter's two Instant::now sites: the end of the pair that \
                      bounds Timings.wall_ms (s7c §7c.19), called on every return path of dispatch \
                      through this one function; feeds no decision."
        )]
        let ended = Instant::now();
        u64::try_from(ended.duration_since(started).as_millis()).unwrap_or(u64::MAX)
    }

    /// A row for a path that terminated before any child reported a version: `Pinned`, every
    /// usage-adjacent member `None`, the argv/env digests over what was assembled (or nothing).
    fn pre_spawn_record(
        &self,
        argv: &[String],
        env_pairs: &[(String, String)],
        wall_ms: u64,
        headroom_before: Option<WindowReading>,
        result: &Result<rk_types::typed::Typed, StepError>,
    ) -> DispatchRecord {
        let err = result.as_ref().err();
        DispatchRecord::with_retained(
            argv,
            env_pairs,
            self.config.tested_cli_version(),
            Retained::Verbatim(Vec::new()),
            Retained::Verbatim(Vec::new()),
            RecordTail {
                cli_version: CliVersion::Pinned(self.config.tested_cli_version().to_owned()),
                api_retry_events: 0,
                prompt_cache_ttl: self.config.cache_ttl().to_owned(),
                subagent_prompt_cache_ttl: self.config.cache_ttl().to_owned(),
                timings: Timings {
                    wall_ms,
                    api_ms: None,
                },
                usage: None,
                total_cost_usd: None,
                headroom_before,
                headroom_after: None,
                cancelled_by: err.and_then(classify::cancelled_by_of),
                detected_by: None,
            },
        )
    }

    /// Append the row through the handed-in store (AC-360; ADR-0126's forbids) — a failure is
    /// counted on the AC-368 warning counter and never becomes the outcome.
    fn finish(
        &self,
        store: &mut dyn EventStore,
        record: DispatchRecord,
        result: Result<rk_types::typed::Typed, StepError>,
    ) -> DispatchOutcome {
        let _ = record::append_completed(store, &record, &self.append_warnings);
        DispatchOutcome { record, result }
    }
}

impl Harness for ClaudeHarness {
    /// One dispatch, blocking to completion (ADR-0099 decision 1), with its
    /// `dispatch_completed` row appended on every path (the module doc).
    ///
    /// # Panics
    /// On a poisoned guard lock (ADR-0118: a halt).
    fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome {
        #[expect(
            clippy::disallowed_methods,
            reason = "AC-001 bans the clock on the WORKFLOW side; this is the FIRST of the adapter's two \
                      Instant::now sites, the pair that bounds Timings.wall_ms on the record (s7c \
                      §7c.19) — the second is wall_ms_since, called on every return path; feeding \
                      no decision."
        )]
        let started = Instant::now();
        let child_env = env::child_env(&self.config);
        let env_pairs: Vec<(String, String)> = child_env
            .as_map()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let pin = self.config.tested_cli_version();
        // The reading the row carries as headroom_before and the reading the guard decides on are
        // ONE reading, taken under one lock (S5-07): another dispatch's observe cannot land between.
        let (headroom_before, verdict) = {
            let g = self
                .guard
                .lock()
                .expect("the guard's lock is poisoned: a halt (ADR-0118)");
            (
                g.last().and_then(headroom::RateLimitReading::windows),
                g.check(self.config.margins()),
            )
        };
        let pre = |err: StepError, argv: &[String]| {
            let result = Err(err);
            let record = self.pre_spawn_record(
                argv,
                &env_pairs,
                Self::wall_ms_since(started),
                headroom_before.clone(),
                &result,
            );
            (record, result)
        };

        // ---- pre-spawn: the worktree (RT-21), the bound card (RT-20), the throttle (AC-376,
        // AC-221), the token (R5-5) — each with its row, cli_version Pinned
        let Some(worktree) = Self::worktree(spec) else {
            let (record, result) = pre(
                StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail: format!(
                        "the spec's inputs carry no string {WORKTREE_KEY:?} (RT-21: the run-owned \
                         worktree is resolved from the store, never from cwd — AC-529); nothing \
                         was spawned"
                    ),
                },
                &[],
            );
            return self.finish(store, record, result);
        };
        if let Some(detail) = Self::worktree_refusal(&worktree) {
            let (record, result) = pre(
                StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail,
                },
                &[],
            );
            return self.finish(store, record, result);
        }
        let Some(bound) = self.config.bound(&spec.card_digest) else {
            let (record, result) = pre(
                StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail: format!(
                        "card_digest {} names no card in the validated store (RT-20 as amended); \
                         nothing was spawned",
                        spec.card_digest.as_str()
                    ),
                },
                &[],
            );
            return self.finish(store, record, result);
        };
        let Some(schema) = bound.schemas.get(&spec.schema) else {
            let (record, result) = pre(
                StepError::Transport {
                    kind: TransportKind::Spawn,
                    detail: format!(
                        "the bound card {} was not bound with schema {} (RT-20 as amended); \
                         nothing was spawned",
                        bound.card.name,
                        spec.schema.as_str()
                    ),
                },
                &[],
            );
            return self.finish(store, record, result);
        };
        match verdict {
            Verdict::Proceed => {}
            Verdict::HaltOverage { overage_status } => {
                let (record, result) = pre(
                    StepError::Overage {
                        reason: Some(format!(
                            "the last rate_limit_event reported isUsingOverage: true (overageStatus \
                             {overage_status:?}) — refused before spawn (AC-221; charter §6.2)"
                        )),
                    },
                    &[],
                );
                return self.finish(store, record, result);
            }
            Verdict::Halt { resets_at, .. } => {
                let (record, result) = pre(StepError::RateLimited { resets_at }, &[]);
                return self.finish(store, record, result);
            }
        }
        // a token that has already fired is not a reason to start a tree (R5-5; ADR-0099 d2)
        if self.token.is_cancelled() {
            let (record, result) = pre(
                StepError::Cancelled {
                    by: CancelledBy::Signal,
                },
                &[],
            );
            return self.finish(store, record, result);
        }
        let user_message = Self::user_message(spec);
        let assembled = match assemble(
            &self.config,
            &CommandSpec {
                card: &bound.card,
                schema,
                adopt_agent: true,
                strict_tools: &[],
                user_message: &user_message,
            },
        ) {
            Ok(a) => a,
            Err(e) => {
                let (record, result) = pre(
                    StepError::Transport {
                        kind: TransportKind::Spawn,
                        detail: format!("the command could not be assembled for a bound card: {e}"),
                    },
                    &[],
                );
                return self.finish(store, record, result);
            }
        };
        let argv = assembled.argv.clone();

        // ---- the child, through rk-containment, on the adapter's own runtime, from a scoped
        // thread (the module doc: callable from any context; a panic is caught at the join)
        let prepared = runner::Prepared {
            assembled,
            env: child_env,
            worktree,
            timeout: Duration::from_secs(spec.timeout_secs),
            expected_agent: &bound.card.name,
            expected_tier: &bound.card.model,
            declared_tools: &bound.card.tools,
            schema_id: &spec.schema,
            schema,
            pinned: pin,
            policy: self.config.on_version_mismatch(),
        };
        let joined = std::thread::scope(|scope| {
            scope
                .spawn(|| self.runtime.block_on(runner::run(prepared, &self.token)))
                .join()
        });
        let run = match joined {
            Ok(run) => run,
            Err(panic) => {
                let what = panic
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| panic.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                    .unwrap_or_else(|| "a non-string panic payload".to_owned());
                let (record, result) = pre(
                    StepError::Transport {
                        kind: TransportKind::Decode,
                        detail: format!(
                            "the dispatch's runtime path panicked and the record was built from \
                             what was known before it: {what}"
                        ),
                    },
                    &argv,
                );
                return self.finish(store, record, result);
            }
        };
        let wall_ms = Self::wall_ms_since(started);

        // ---- the guard learns this dispatch's last reading; the probe seams; the counters; the row
        if let Some(reading) = &run.last_reading {
            self.guard
                .lock()
                .expect("the guard's lock is poisoned: a halt (ADR-0118)")
                .observe(reading);
        }
        *self
            .last_kill_drop_order
            .lock()
            .expect("the probe's lock is poisoned: a halt (ADR-0118)") =
            run.kill_drop_order.clone();
        *self
            .last_raw_frames
            .lock()
            .expect("the raw-frames seam's lock is poisoned: a halt (ADR-0118)") =
            Some(run.raw_frames.clone());
        if let Some(e) = &run.kill_error {
            self.kill_warnings
                .lock()
                .expect("the kill-warning counter's lock is poisoned: a halt (ADR-0118)")
                .push(e.clone());
        }
        let cli_version = run.observed_version.as_ref().map_or_else(
            || CliVersion::Pinned(pin.to_owned()),
            |v| CliVersion::Observed(v.clone()),
        );
        let ttl = |name: &str| {
            env_pairs
                .iter()
                .find(|(k, _)| k == name)
                .map_or_else(|| self.config.cache_ttl().to_owned(), |(_, v)| v.clone())
        };
        let record = DispatchRecord::with_retained(
            &argv,
            &env_pairs,
            pin,
            run.stdout,
            run.stderr,
            RecordTail {
                cli_version,
                api_retry_events: run.api_retry_events,
                prompt_cache_ttl: ttl(PROMPT_CACHE_TTL),
                subagent_prompt_cache_ttl: ttl(SUBAGENT_PROMPT_CACHE_TTL),
                timings: Timings {
                    wall_ms,
                    api_ms: run.api_ms,
                },
                usage: run.usage,
                total_cost_usd: run.total_cost_usd,
                headroom_before,
                headroom_after: run
                    .last_reading
                    .as_ref()
                    .and_then(headroom::RateLimitReading::windows),
                cancelled_by: run
                    .result
                    .as_ref()
                    .err()
                    .and_then(classify::cancelled_by_of),
                detected_by: run.detected_by,
            },
        );
        self.finish(store, record, run.result)
    }
}
