# Status-quo audit, 2026-10-06

A read-only audit of the redkiln-rs programme against its vision (RFC, design), roadmap (Phases 1–4) and runbook. It answers three questions: where the programme stands, how good the work is, and what remains. It was taken at snapshot **`36dd8ac`** on `build/phase-0-to-1` (2026-10-05) while Step 8i was in progress.

**Bottom line.** Phase 1 is about 65% done and the full vision about 30% (judgement). The engine is built and well tested against a simulated model. No real Claude-driven fix or story lane has run through `redkiln run`. Forecast for M5 (Phase-1 exit): P50 about 2026-10-23, 80% band 10-19 to 11-03.

## Files

| Path | What it is |
| --- | --- |
| [`executive-summary.html`](executive-summary.html) | One-page status board (open in a browser) |
| [`appendix.md`](appendix.md) | Full technical report with `path:line` citations. Its verdicts are authoritative over the survey reports |
| [`survey/`](survey/) | 16 survey reports: vision, plan status, 10 code areas, pack/CI, records, traceability, velocity, landing |
| [`data/`](data/) | Structured results: the survey wave (`w1-results.json`) and the adversarial verification plus synthesis (`w2-results.json`) |
| [`build/`](build/) | Build and test triage (`build.txt`), escape-hatch census, the build script, and the clean-env test log |
| [`issues/`](issues/) | Issue drafts with their tracked or untracked verdicts, including the 2 not filed |

## Method

- **Isolation.** A detached worktree at `36dd8ac`, so the live 8i session was never touched.
- **Build.** fmt, build, clippy, test, conformance and rustdoc in an isolated target dir.
- **Survey.** 17 agents (sonnet, haiku hash check).
- **Verification.** 13 headline claims, each attacked by 2 opus skeptics: 7 confirmed, 6 confirmed with corrections, 0 refuted.
- **Completeness critic.** An opus critic raised 4 gaps, and a follow-up agent investigated each one.
- **Synthesis.** A final opus agent wrote the appendix.
- **Issue check.** Each finding was then re-checked at `ce4ed1a` (local) and `62dd3b8` (origin) to confirm it still holds and is not already tracked by a record.

## Corrections since the appendix was written

- **CI is green on the remote head.** The appendix (§4.10, §7 GAP1) says no green CI run exists anywhere. The remote head `62dd3b8` went green on 2026-10-05 (ci run 37310935003, temper run 37310935029). The local branch still lacks those fixes; see #23.
- **Codex phase placement is already settled.** ADR-0167 (accepted) supersedes ADR-0094 d2: the Codex adapter is Phase 4, after S-CODEX. Appendix decision O-11 is moot (`issues/C8.md`).

## Filed issues (label `audit-2026-10-06`)

| Draft | Issue | Title |
| --- | --- | --- |
| A1 | [#5](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/5) | Wire the Ctrl-C/SIGTERM CancelToken into the run verb |
| A2 | [#6](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/6) | Compute stage per item in `redkiln status` using Event::item() |
| A3 | [#7](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/7) | Add rust and python to the rank-tie lint toolchain domain |
| A4 | [#8](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/8) | Wire budget exhaustion gate and degradation policy into the run path |
| A5 | [#9](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/9) | Make the per-item token charge fail closed and dispatch admission atomic |
| A6 | [#10](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/10) | Make catalog header loading strict and digest excludes/deviation_ok |
| A7 | [#11](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/11) | Scrub provider environment variables in the rk-cli test harness |
| A8 | [#12](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/12) | Remove unused anyhow and proptest dependencies from rk-cli |
| A9 | [#13](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/13) | Wire review-gate quarantine, advisory retirement and AC-349 binding into runs |
| A10 | [#14](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/14) | Wire selection step 7 and pack capability routing; add a cross-model test |
| B1 | [#15](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/15) | Build the tier-1 orphaned-anchor and rewrite-event producers (AC-167/169/170/171) |
| B2 | [#16](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/16) | Implement Display for rk-git GitError so operator errors are readable |
| B3 | [#17](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/17) | Stop identity::read swallowing probe errors as "no identity" (AC-179 bypass) |
| B4 | [#18](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/18) | Restrict trailer parsing to the commit's final trailer paragraph |
| B5 | [#19](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/19) | Make Digest deserialisation validate through from_hex |
| B6 | [#20](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/20) | Land the POSIX rk-containment kill fix locally and make is_empty zombie-aware |
| B7 | [#21](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/21) | Seed the rk-adapter-claude throttle across processes (first dispatch is unguarded) |
| B9 | [#22](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/22) | Make rk-cli doctor use rk-store's credential and network-fs detectors |
| C1 | [#23](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/23) | Merge origin/build/phase-0-to-1 and correct the stale "never pushed / CI never ran" records |
| C2 | [#24](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/24) | Add a landing step to the runbook: --no-ff merge of build/phase-0-to-1 to main at M5 |
| C3 | [#25](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/25) | Define a recovery and re-freeze procedure for a failed first real lane run |
| C4 | [#26](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/26) | Decide what M5 certifies: disposition table or "six exit checks only" |
| C5 | [#27](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/27) | Add a register-wide AC-to-test reconciliation report (SM-100) and triage known gaps |
| C6 | [#28](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/28) | Confirm or reverse the 20 AFK-provisional rulings that rest on operator silence |
| C7 | [#29](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/29) | Close stale tracker rows and stamp M4; fix the PROGRESS execution-order line |
| C9 | [#30](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/30) | Decide whether to re-point s8j's entry gate from Checkpoint 8i to Checkpoint 8h |
| C10 | [#31](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/31) | Post-M5 hardening: sweep stale comments and headers, split the god modules |
| C11 | [#32](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/32) | Push or back up the redkiln-corpus repo; it has no remote and holds the S-BIN evidence |
| C12 | [#33](https://github.com/Wet-Ink-Corporation/redkiln-rs/issues/33) | Decide: accept human-only closure of verify-stage ACs for exit runs, or wire the mutation pin |

**Not filed, because a record already owns them:**
- `B8`: the copies of the adapter registry plumbing are deliberate, and their shared home is owed under RT-24 (`runbook/phase-1/s7c-adapters.md:168`, `:3210`).
- `C8`: the Codex placement is resolved by ADR-0167.

`B7` was narrowed to the one untracked part, the process-local throttle. The poison `expect`s are ADR-0118's accepted halt, and settings-file retention is a recorded s8h residual.
