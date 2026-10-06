# W1 audit: `rk-adapter-claude` + `rk-containment` (snapshot 36dd8ac)

Auditor scope: the Claude Code harness adapter (spawns `claude -p`, argv, env denylist, stream decode, init adoption, headroom ticks, throttle, classification, record rows, cancel/timeout) and the process-containment crate (Windows Job Object, POSIX process group, bounded run). Read-only; the pinned worktree only. Paths are relative to the worktree. "RECORDS" = what runbook/spikes/ADRs claim; "CODE" = what the source and tests show.

## 0. Verdict

**Grade: B (adapter) / B+ (containment on Windows; POSIX arm unverified) -> combined B.**

The implemented surface is complete against the Phase-1 design for this area, it is built test-first and unusually rigorously, and it has run against the real CLI. What it has NOT done is run since 2026-09-21 at CLI 2.1.278. The CLI the programme now uses is 2.1.289, and the adapter's pin and its permission-mode table do not cover it. The pin move, the three s8h confinement probes and the AC-215 integration half are all unrun and are scheduled in `s8j`. So the adapter is "done and exercised against a fake CLI, plus one narrow live acceptance", not "proven under the least-privilege confinement that s8h added".

## 1. Module / public-surface map (CODE)

`rk-adapter-claude` (6,528 src lines, 16 modules; `crates/rk-adapter-claude/src/lib.rs:114-130` module list, table at `lib.rs:20-37`):

| module | role | lines |
| --- | --- | --- |
| `lib.rs` | `ClaudeHarness` (`new(Config, CancelToken)`, `impl Harness::dispatch` at `lib.rs:513`), pre-spawn refusals, record assembly | 860 |
| `config.rs` | `Config::new` lints, `TESTED_CLI_VERSION = "2.1.278"` (`config.rs:124`), `Margins`, `MismatchPolicy` (no `Default`; Refuse or Degraded), `ConfigError` | 792 |
| `command.rs` | ONE argv builder `assemble` (`command.rs:678`): golden order, per-profile delivery (static = `--system-prompt` + stdin message; default = prompt+message on stdin), `ARGV_CAP_BYTES = 32_767`, per-dispatch settings file, `PERMISSION_MODE_ARGV` pin-keyed table (`command.rs:~138`) | 776 |
| `runner.rs` | spawn through `rk_containment::spawn_into_job`, stdout drain, three-way race (drain / child exit / token-or-timeout), kill through the group | 731 |
| `classify.rs` | every termination onto the closed eleven `StepError`s, schema validation (`jsonschema`), permission-denial channels (a/b/c), protected-segment denials | 657 |
| `headroom.rs` | integer digit scanner over `rate_limit_event`, `u16` ticks, no `f64` | 577 |
| `throttle.rs` | `Guard::check` (`ticks >= 100 - margin_ticks`), overage halt, batch-delta calculator | 271 |
| `init.rs` | `system/init` -> effective-config set digests, adoption on equality, version vs pin, `apiKeySource == "none"` (`init.rs:169`, `:309`) | 321 |
| `record.rs` | `dispatch_completed` on every path, `rate_limit_reading` once per dispatch through the child, mismatch warning derived on read | 233 |
| `env.rs` / `env_denylist.rs` | child env = captured map + both TTLs, a refusal not a subtraction; ONE exported denylist (7 exact names + `CLAUDE_CODE_USE_` prefix, ASCII-case-insensitive) | 101 / 70 |
| `binder.rs`, `card.rs`, `registry.rs`, `event.rs`, `ttl.rs` | role->card->`ActivitySpec` binder; card validator; dated capability registry; stream line classifier (unknown events retained, nothing refused); five-state TTL classifier | 274 / 400 / 147 / 226 / 92 |

`rk-containment` (1,549 src lines; ADR-0127): `lib.rs` (surface: `spawn_into_job`, `Group{kill,pids,is_empty}`, `run_bounded`, `Outcome`, `process_start_time`, `CANCEL_EXIT_CODE`, doc-hidden `terminate_pid`/`is_alive`/`KillDropOrder`), `ffi.rs` (690, the one `#![allow(unsafe_code)]` module, `ffi.rs:61`), `windows.rs` (259), `posix.rs` (181), `bounded.rs` (169), `error.rs` (82). Deps: `std`, `windows-sys` (Win), `libc` (Unix) only (`crates/rk-containment/Cargo.toml`).

Wiring: the CLI constructs the harness in the composition root (`crates/rk-cli/src/verbs/mod.rs:189-200`); `rk-containment` is also consumed by `rk-cli` (doctor version probe), `redkiln-migrate`, and both toolchain adapters.

## 2. What Phase 1 requires of this area (RECORDS)

- Design §9.1/§0: Claude Code `Harness` is one of the four real Phase-1 adapters (`references/2026-09-08-architecture-design.md:246`). ADR-0099 (accepted): blocking `dispatch`, token cancel, containment per OS. ADR-0127 (accepted): containment crate. ADR-0126 (accepted): `dispatch_completed` row. ADR-0010: subscription only.
- Stage `s7c` (`runbook/phase-1/s7c-adapters.md`): five slices; Checkpoint 7c = "adapters exist, real rows conform, one dispatch ran" and nothing more (`s7c:67-73`). Its ARMED/not-discharged table (`s7c:3107-3135`) lists what 7c deliberately did not discharge.
- `s8h` (cards and run environment): least-privilege cards, `--settings` file, pin-keyed `--permission-mode` spelling, launch gate, three live probes (ADR-0197 d3/d9; **ADR-0197 is still `status: proposed`**, `references/adr/0197-...md`).
- Phase-1 exit needs the adapter to carry a real fix-lane run and a story-lane run (design §0 / `s8-exit-criterion.md`); that is the downstream consumer, not this crate's own test.

## 3. Compare: implemented vs fake-only vs armed vs missing

**Genuinely implemented AND exercised by tests (CODE, test run on the snapshot):**
- `cargo test --workspace` on the snapshot (audit log `audit/log-test.txt:516`, `:1764`): **rk-adapter-claude 149 passed / 0 failed**; **rk-containment 6 passed / 0 failed** (Windows arm: job-before-spawn, orphan-grandchild reaped via job, breakaway refused, no descendant survives, run_bounded kills tree at timeout, process start time). fmt and build and clippy rc=0 (`audit/build.txt`). Clippy: 4 nursery `too_long_first_doc_paragraph` warnings in `command.rs` (379/423/540/605) for the adapter, 0 for containment (`audit/log-clippy.txt:1223-1274`).
- Argv: golden snapshots (insta `tests/it/snapshots/it__command__golden_argv_{default,static}.snap`), proptest regressions, trybuild UI cases (`tests/ui/`), argv-cap fixture at the cap, per-profile delivery, `--permission-prompts none`, `--strict-mcp-config`, `--setting-sources ""` unconditional, `--bare` absent (AC-511 lint).
- Env denylist: one exported list, `Config::new` refuses each entry and the prefix (`tests/it/env.rs`, `tests/it/config.rs:24-45`).
- Init adoption / isolation: composite adoption reading (RT-27), exact full-id model equality, `permissionMode` equality, `apiKeySource` other/absent -> `NotIsolated` (`tests/it/isolation.rs:393,551,571`).
- Tick arithmetic: integer-only scanner, rejects >2 decimals/overflow, `u16`; guard boundary fixtures at 92/93 and 81/82 (`tests/fixtures/streams/rate-limit-*`, `tests/it/throttle.rs`, `tests/it/headroom.rs`).
- Cancel/timeout/kill: three-way race, pre-spawn cancelled token spawns nothing, kill through the group with kill-before-drop order recorded, panic caught on a scoped thread and turned into a `Transport{Decode}` row (`lib.rs:709-750`).
- Port conformance: `port_conformance_real_row_claude` runs `rk-ports`' shared body over the REAL adapter with a fixture child (`tests/it/conformance.rs`). This is the AC-499 Harness row; local-only by design.
- Guard rails for the live tests themselves: `tests/it/live_guard.rs` lint asserts every `#[ignore]` test opens with the launch-record guard and names `tools/live-run.sh probe live::<name>`, with planted controls (`live_guard.rs:106-143`).

**Fake-only (the important qualifier):** every stream fixture in the crate is authored or synthetic. `tests/fixtures/streams/README.md:3` states no raw `claude -p` stream exists at the corpus pin, and every `system/init` line is "SCRIPTED". The `recorded/` extractions are the old runner's re-serialised rows, "not the CLI's bytes" (`recorded/PROVENANCE.md`). The one real raw init/result from the 2026-09-18 probe lives in the external `redkiln-corpus` repo (commit `f39b2f8e47a4` per PROGRESS R-18, `PROGRESS.md:180`), not in this tree. The permission-denied event shape (guard 3 channel b) is "DRAFTED from the 2.1.281 binary's literals, UNMEASURED as a live frame" (`streams/README.md`, `permission-denied-outside-read.scripted.ndjson` row). So the parsers are exhaustive against what the authors imagined plus the few live frames the probes saw; real-frame drift at 2.1.289 is unobserved.

**ARMED / deferred / not run (RECORDS; confirmed against CODE where noted):**
- `live_disallowed_tool_terminates` (AC-215 integration half): authored, never run; ARMED "to Step 8" (`s7d:1970`, `s7d:3701`; `live.rs:455`).
- s8h probes 0/1/2 (`live_denial_channel_preprobe`, `live_confinement_and_denial_probe`, `live_init_equality_probe`): "authored, compiled, listed, never run here" (`s8h:2032-2034`); scheduled as `s8j` §8j.6 steps 2b/3/4, each an operator go-ahead and paid (`s8j:1111-1146`). These are the only evidence that the new `--settings` confinement, the denial channels and the full-model-id adoption hold on a real CLI. The confinement design rests on ADR-0197 (proposed).
- POSIX arm of containment (`process_group(0)` + `killpg`, `/proc` enumeration): written, "ARMED to a recorded CI run that has never executed" (`rk-containment/src/lib.rs` module doc; `s7c:3118` AC-224 POSIX row). `ci.yml` "has never run" per `PROGRESS.md:559` (dated 2026-09-13; nothing later in PROGRESS records a CI run; remote CI status cannot be verified from this snapshot). A CI green on both OS legs is a Checkpoint-8 condition (`DECISIONS-2026-09-21.md:1688`, row 14a).
- AC-214 lint half (unconditional `--permission-mode` on the dispatch path) and AC-499 CI-matrix instrument: ARMED (`s7c:3114-3115`).
- AC-391, AC-399 (whole), AC-361/367/387: not discharged here; owners are kernel/cost (`s7c:3107-3140`).
- S-TTL1 subagent bucket "unobserved" (R-24); S-RETRY exhaustion arm "NOT MEASURABLE" (`PROGRESS.md:613`). `Unsupported` and `Timeout{after_secs}` StepError arms are produced by no path (`lib.rs` module doc).
- ADR-0197 (proposed) governs the s8h half of the argv/settings/permission design; not yet accepted.

**Missing / drift found (CODE vs RECORDS):**
1. **Pin drift.** `TESTED_CLI_VERSION = "2.1.278"` (`config.rs:124`); `PERMISSION_MODE_ARGV` has rows only for `2.1.278` ("default") and `2.1.287` ("manual") (`command.rs:~138-141`); a pin with no row is refused (`argv_permission_mode`, `command.rs:159-176`). The CLI is now frozen at **2.1.289** (`s8i:2017`). With `MismatchPolicy::Refuse` a real dispatch through this adapter at 2.1.289 is refused at the init line (AC-226), and no `2.1.289` spelling row exists. This is a known, scheduled item (`s8j` §8j.6 step 1 "the pin move", `s8j:955-978`) and the s8h comment says the table's `2.1.287` row is "dormant until s8j's pin move". It is a gate, not a defect, but it means the adapter cannot drive the real fix-lane today.
2. **Throttle is process-local.** `Guard` starts empty per `ClaudeHarness` (`lib.rs:205`) and is fed only by in-process dispatches (`lib.rs:764`); nothing seeds it from the last stored `rate_limit_reading` row. The code documents "first dispatch of a batch is unguarded" (`throttle.rs:91-96`). If each `redkiln run` is a fresh process, the halt only protects within a run. Whether a pre-batch headroom check exists elsewhere is unverified (out of this area).
3. **Settings files accumulate.** One `child-settings-<pid>-<seq>-<sha>.json` per dispatch is written (`command.rs:382-420`, `lib.rs` write path); no `remove_file` exists in `src/` (grep). Minor hygiene.

## 4. Code quality

**Structure: strong.** Tight module boundaries, one builder for argv, one place that spawns (the `std::process::Command` build in `runner::build_command` is the only adapter-side site, behind a narrow `#[expect]`), every override is `#[expect(..., reason)]` per site, no `#[allow]` in the adapter. `thiserror` throughout, no `anyhow`. Typed error design: `ConfigError` never crosses the port (ADR-0112 closed set of eleven `StepError`s). `ClaudeHarness` is statically `Send + Sync` (`lib.rs:179`). Clear separation of config-time lints from runtime.

**Test depth: very deep for a fake CLI.** 158 test fns / 12.5k test lines for 6.5k src lines in the adapter (census `escape-hatches.txt:2`); golden (insta), property (proptest), compile-fail (trybuild), and process-level tests via a real spawned fixture child (`rk-fixture-child-claude`, built from `rk-containment`'s fixture source) so the spawn/kill/drain code runs for real. Mutation-style discriminators are cited per test. The weakness is not depth but realism: all stream input is authored (see section 3).

**Escape hatches in NON-TEST code (spot-checked; census numbers qualified):**
- Adapter census says unwrap/expect=16, allow/expect-attr=5, unsafe=0, panics=0. Real production `expect` sites: **11 mutex-poison `.expect("... lock is poisoned: a halt (ADR-0118)")`** (`lib.rs:242,258,273,289,303,538,764,770,775,780,791`; `record.rs:118,130,142`) plus two serialisation-invariant `.expect`s (`card.rs:390`, `record.rs:162`, the first documented with `# Panics`). The poison expects are the ADR-0118 policy (a poisoned lock halts the run; `runner.rs` uses a `locked()` helper returning `Result` instead). They are not clippy-enforced: the workspace lint set (`Cargo.toml` `[workspace.lints.clippy]`) denies only `all` plus five pedantic lints and does not enable `unwrap_used`/`expect_used`, so "no escape hatches" is convention plus ADR. **Qualification:** the expects at `lib.rs:538` (pre-spawn) and `:764-791` (post-run) sit OUTSIDE the scoped-thread catch at `lib.rs:709-750`, so a poisoned lock there would unwind out of `dispatch` and append no row, contradicting the module doc's AC-360 claim ("never an unwind out of dispatch that appends nothing", `lib.rs` module doc). Poisoning requires a panic while a tiny critical section is held, so practical risk is low; the contradiction with the doc is real. `unwrap_or_default`/`unwrap_or(0)` on usage fields: none found in `src/` (also CI-grepped, `ci.yml:318`).
- Adapter `#[expect]`: the `std::process::Command` site (`runner.rs:291`) and the two `Instant::now` sites (`lib.rs:~525`, `wall_ms_since`), all with reasons.
- Containment census says unsafe=38, allow=10, unwrap/expect=3. Qualified: **25 `unsafe` blocks/impls, each with a `// SAFETY:` comment (25 SAFETY comments vs 25 unsafe sites, `ffi.rs`)**, all in `ffi.rs` under the single `#![allow(unsafe_code)]` (`ffi.rs:61`); remainder of the census 38 are doc/comment mentions. The three `.expect`s (`ffi.rs:242,293,341`) are typestate invariants ("an Assigned holds its child until...") on a private state machine. The census's 10 "allow/expect-attr" are one `allow` plus nine commented `#[expect(clippy::disallowed_types)]` sites. No `HashMap`, no clock, no sleep.
- `eprintln!` in a library's production path (`lib.rs:788`, protected-segment warning) is the only direct stderr write; a minor smell for a crate otherwise purely value-returning.

**Dead / test-only surface in production types:** 30 `#[doc(hidden)]` items. `ClaudeHarness` carries two mutexes (`last_raw_frames`, `last_kill_drop_order`, `lib.rs:155-158, 289-306`) that exist for the live probes and tests and are written on every real dispatch. Test seams in the production struct, not dead code, but surface area and cost on the hot path (small).

**Comment-to-code noise:** about 36% of adapter `src` lines are comments (2,363 of 6,528) and 41% of containment (630 of 1,549); `lib.rs`'s module doc alone is ~110 lines. Much of it is history ("dated 2026-10-02, s8h build, package P3-...", RT/AC/ADR id chains), which makes the code hard to read cold and will rot as records move; it is, however, accurate where spot-checked and is the repository's traceability convention. The 4 clippy `too_long_first_doc_paragraph` warnings are the same noise surfacing.

**Containment specifics:** Windows arm is the careful one (create job before spawn, `CREATE_SUSPENDED` -> assign -> resume, enumerate and kill via job, wait each member dead, PID-reuse guard via `IsProcessInJob`, 1024-pid list capped with an error never a truncation, `ffi.rs:440-500`). POSIX arm is simpler and weaker by nature: `killpg` reaches only the group, so a descendant that calls `setsid`/`setpgid` escapes (not documented as a limit in what I read); enumeration and `process_start_time` read `/proc` only, so they would fail on macOS (macOS is not in the CI matrix, `[ubuntu-latest, windows-latest]`, `PROGRESS.md:559`; whether macOS is a target is unverified). Dropping the POSIX `Group` kills nothing (documented, "a leak the POSIX arm's CI run measures").

## 5. What the live probes proved (RECORDS; the code cannot show this)

Counted from `PROGRESS.md:613`, `:180-181`, `s7c:1532-1604`, `s8i:1806-1808`:
- **2026-09-18, Checkpoint 7c's fifteenth line (`live_dispatch_probe`, haiku, ~$0.03 each):** five stopped runs and a sixth that passed. They proved, on a real Max-subscription session at CLI 2.1.277: the adapter refused every real dispatch on first contact (RT-27: init has no `agent` key; fixed by the composite adoption reading); a TRUE result frame with `structured_output` is read; `apiKeySource` is `"none"` (R-18 closed); `COMPLETED_ROWS=1`, `DISPATCH_ROWS=0`; `instrument_version` and `OBSERVED_CLI_VERSION` equal the pin; the raw init and result were retained in the corpus repo. S-CANCEL-WIN's vitest arm and three spike drivers also ran: S-TTL1 -> `no-write` (subagent bucket unobserved), S-RETRY margin arm measured on a live reading, exhaustion arm not measurable.
- **2026-09-21 (Checkpoint 7c re-run after pin move 2.1.277 -> 2.1.278):** one more paid haiku dispatch, green end to end, 211 stopping statements, sentinel `CP7C-END-OF-BLOCK reached`. This is the LAST time the adapter's real dispatch path ran against the real CLI.
- **2026-10-05/06 (s8i B1, $5.56):** one probe + six capture sessions, but these are driven by the old `redkiln-corpus` runner and `/ue-verify`, NOT by `rk-adapter-claude` (`s8i:248-264`, `s8i:1806`). They corroborate the CLI surface at 2.1.289 (`card_adopted`, `apiKeySource none`, `--json-schema` live) but do not exercise this crate. Do not count them as adapter evidence.
- Never run: `live_disallowed_tool_terminates`, all three s8h probes (0/1/2), S-SPF. Never recorded: a POSIX CI run.

## 6. Risks

1. **Stale pin vs installed CLI (2.1.278 vs 2.1.289).** First real dispatch after the `s8j` pin move is effectively a fresh integration: the permission-mode spelling changed between 2.1.278 ("default") and 2.1.287 ("manual"), the init surface shifts between versions (RT-27 already bit once), and s8h's confinement (`--settings` file, `--setting-sources ""`) has never been exercised live. Expect at least one real-frame surprise at probe 0/1.
2. **Confinement unproven live.** The least-privilege cards depend on `--settings` permission rules, an outside-read denial channel that is "UNMEASURED as a live frame", and an ADR still `proposed`. If probe 0 sees the denial only as an errored `tool_result`, the plan HALTs (s8h rule).
3. **Authored-only fixtures.** Parser exhaustiveness rests on scripted frames; protected-segment and denial detection in particular have no live frame in-tree.
4. **POSIX arm and CI never run;** `setsid` escape and `/proc`-only enumeration are unmeasured limits. Only matters if Phase-1 exit or a user runs on Linux/macOS (CI both-OS green is a Checkpoint-8 condition).
5. **Throttle not persisted across processes**; first dispatch of every process is unguarded.
6. **Doc/code contradiction on poison-expects** outside the catch (`lib.rs:538, 764-791`): low probability, but the crate's own AC-360 claim says otherwise.
7. **Maintainability:** one-third of source is process-history commentary; onboarding and refactors will fight it.

## 7. Unverified (could not or did not verify)

- Whether `ci.yml` has executed on GitHub for any commit (remote status not visible; `origin/build/phase-0-to-1` exists but its CI result cannot be read here).
- Contents of the external `redkiln-corpus` retained raw frames (commit `f39b2f8e47a4`); the figures above are the records' statement.
- Whether a pre-batch headroom check exists in `rk-kernel`/`rk-cli` to cover the process-local guard.
- Whether macOS or non-ubuntu POSIX is an intended target.
- The audit test run shows failures in OTHER crates (`rk-adapter-toolchain-node` 14 failed, `rk-cli` 63 failed, one 71/1 suite; `audit/log-test.txt:670,767,1739`); these are outside this area and were not diagnosed (a concurrent build may be the cause, or environment); none are in rk-adapter-claude or rk-containment.
- Mutation-test strength of the adapter suite (no cargo-mutants evidence read); the tests cite discriminators but I did not run them.
- Exact line of `PERMISSION_MODE_ARGV` and the second `Instant::now` site are approximate (`~`).
