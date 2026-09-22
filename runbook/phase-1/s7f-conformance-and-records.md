# Step 7f — The conformance gate and the records it needs: the design's closed sets, asserted (Phase 1, part 4)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7f` | Phase 1 | Checkpoint 7e | Checkpoint 7f | `s7e` | `s7g` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Step 8 ran the first `redkiln run` ever attempted on this tree and the shipped pack did
not load. Under the two pack defects sat the finding this step and the three after it exist to repair:
**`Runtime::run_activity` dispatches an *activation*, not an activity's *steps*.** There is no routing over
`StepKind` anywhere in the kernel — measured 2026-09-21, `grep -rn 'StepKind' crates/rk-kernel/src/runtime/` returns
**zero**. Across the shipped catalog that leaves **185 of 574 steps** executed as such: **113 `tool`** (including 28
`toolchain:test`), **102 `emit`**, **94 `check`** and **80 `gate`** have no executor, and **102 of 102** activities
carry at least one non-agent step.

**None of it is undecided.** design §4.4 fixes the five closed step kinds and says `tool` is *"a toolchain verb
through the port"*; design §5's event table names `stage_transition` tier-1 *"(trailer)"*, produced by *"the kernel's
FSM at a stage boundary"*, and `slice_sealed` by *"the lane's review stage"*; ADR-0105 decision 3 specifies the
trailer plumbing; `INDEX.md` puts *"the runtime"* in **Step 7b's** scope. This is execution against accepted records.

**Why the gate comes before the code.** Five stages of checkpoints passed over a runtime that does not implement its
own spec, and none of them was weak — each tested what was built. What no gate did was compare the **built surface**
against the **design's own closed sets**. This step authors that comparison first, so the three stages after it are
measured by something written before them rather than by their own account of themselves.

**This step takes one decision and it is recorded, not taken here**: `ImplReport` becomes the sixth toolchain-keyed
kind (ADR-0073 amendment, operator decision 2026-09-21). §7f.2 executes it.

### 7f.1 The conformance gate — four assertions over closed sets, three of them ARMED

**The house pattern is `live.rs`'s: authored, ARMED, not claimed.** Each of the first three lints asserts the
**target** state and carries `#[ignore]` naming the debt and the stage that discharges it. The ignore is removed by
the landing that makes it pass, **in the same commit**, so no stage can claim a capability the lint does not see. An
`#[ignore]`d test is coverage theatre unless something counts it, so Checkpoint 7f counts them **by name**.

The four, and what each pins:

| lint | closed set it reads | measured 2026-09-21 | discharged by |
| --- | --- | --- | --- |
| `conformance_every_step_kind_has_an_executor` | design §4.4's five step kinds | **0** `StepKind` references in `runtime/` | `s7g` |
| `conformance_every_port_has_a_call_site` | ADR-0003's port table | harness 1, store 4, **toolchain 0, tracker 0, docs 0** | `s7g` (toolchain), later (tracker/docs) |
| `conformance_every_trailer_key_has_a_producer` | design §5's event table | `Gate` 2, **`Story` 0, `Slice-Verdict` 0** | `s7h` |
| `shipped_pack_loads` | the product's own `.redkiln/` | **RED** — `RankTie` over `ImplReport` | **this step**, §7f.2 |

**`shipped_pack_loads` is the one that would have caught all of this**, and its absence is the whole lesson: every
catalog test in this tree runs over a FIXTURE catalog. Nothing ever loaded the 102 activities the product ships. It
reads `.redkiln/` through `workspace_root()`, the route `tests/it/dag.rs` already uses for `spikes/p9/facets.toml`,
and it asserts the shipped pack passes **every load-time lint** — `requires_cycle_lint`, `excludes_lint`,
`dangling_requires_lint` and `rank_lint` — not merely that it parses.

🛑 **The first three lints scan CODE lines only.** A comment naming `StepKind`, a doc-comment naming a port, or a
grammar file naming a trailer key must not satisfy them — that is how a guard gets satisfied by the prose beside it,
which this programme has now been bitten by eight times. The source-lint precedents are `tests/it/retry.rs`'s
`ac028_policy_match_has_no_wildcard`, `tests/it/recorded_log.rs`'s `the_replay_still_calls_the_divergence_detection`
and `tests/it/projection.rs`'s `stage_action_still_filters_to_tier_one` (Step 8).

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# The three ARMED lints must exist, be ignored, and name their discharging stage in the reason.
out=$(cargo test -p rk-kernel --test it -- conformance_ --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the conformance listing could not run (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | grep -c 'conformance_'); [ "$n" -eq 3 ] || { echo "FAIL: expected exactly 3 conformance lints, found $n — an armed lint that is deleted reads as a discharged one"; exit 1; }
# And the one this step discharges must be GREEN, not ignored.
out=$(cargo test -p rk-kernel --test it -- shipped_pack_loads 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the shipped pack does not load (rc=$rc) — §7f.2 is not done"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched shipped_pack_loads — the product's own pack is asserted by nothing"; exit 1; }
```

### 7f.2 ADR-0073 amended — `ImplReport` is the sixth toolchain-keyed kind

**The decision, and its warrant.** `redkiln run` refuses the shipped catalog at `RankTie` over `ImplReport`:
`hotfix` and `implement-ue` both provide it at `method_rank = 10` and are simultaneously admissible. Measured
2026-09-21 over all 102 activities, there are **9 equal-rank producer sets**; **5** are on toolchain-keyed kinds and
step-2 keyed expansion resolves them. Of the remaining four, **two are `ImplReport` shadowing `Changeset` exactly** —
identical producer sets at both ranks, `dependency-update`/`hotfix`/`implement-ue`/`test-after` at 10 and
`bdd-outside-in`/`walking-skeleton` at 30 — because every activity that provides `Changeset` also provides
`ImplReport`, and `Changeset` is keyed while `ImplReport` is not.

**The other two are not ties at all** and no change is owed for them: `checklist-review` (`kind:chore`, `kind:docs`)
and `spike-decision-review` (`kind:spike`) are disjoint by tag, as are `fix-spec` (`kind:bug`, `kind:security-vuln`,
`kind:incident`) and `user-story-ac` (`kind:story`, `kind:project`, `kind:tech-debt`, `kind:docs`). No item's `kind`
is in both sets, so neither pair is ever simultaneously admissible [measured 2026-09-21]. **Keying `ImplReport`
therefore clears the entire remaining rank-tie surface**, which is why it is taken over narrowing a tag.

**`artifacts.toml` was not at fault** and this is an amendment to the record, not a correction of the pack: the
registry carries precisely the **five** keyed kinds the design names — `ChangeSummary`, `Changeset`, `ProfileReport`,
`ReproCase`, `TestReport` — and `ImplReport`, the same activities' other output, is not among them with no record
stating why. The amendment moves that count to **six**, and `spikes/p9/artifacts.toml` is **not** edited: §7b.12
forbids editing the spike registry to make a case go green, and `.redkiln/` is derived output, which is the half
this step may author.

**Not by `method_rank`.** ADR-0056 makes a tie a **validation error** precisely so that it is not resolved by
nudging a number; `method_rank` is a method-preference ordering and using it as a tie-break would encode a
preference nobody holds.

### 7f.3 The `rate_limit_reading` producer, named

**ADR-0110 defines headroom as a batch delta over *"readings sharing one `resetsAt`"*, and nothing journals a
`resetsAt`.** `EventKind::RateLimitReading` exists with the wire name `rate_limit_reading` and has **no producer
anywhere in the workspace** — its only three references are kernel match arms classifying it as tier-2. The row that
IS written on every dispatch, `DispatchRecord`, carries `headroom_before` / `headroom_after` as
`WindowReading { five_hour, seven_day }`, which has no such field.

**The value is already in hand and is discarded.** `rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a
`RateLimitReading` carrying `resets_at`, `five_hour_resets_at` and `seven_day_resets_at`, and
`ClaudeHarness::last_reading()` exposes it. So the gap is a missing **producer**, not a missing measurement, and the
route taken is to journal the reading rather than to amend ADR-0110 toward what happens to be recorded — F6's
quantisation finding is what made the `resetsAt` grouping necessary, and a delta over "readings in one run" would be
a different measure wearing the same name.

This section authors the **record** naming the producer and its row's payload fields against design §5's event
table. `s7i` builds it and `redkiln cost` on top of it.

### 7f.4 RT-19 — the cost unit

`s7c` §7c.1 raised RT-19 and `s7d` §7d.8 routed it here: *"AC-395's `redkiln cost` and AC-372's renderers — `s7c`
ARMED them here and they are ARMED on to Step 8 with the cost unit record RT-19 still OWED."* Step 8 did not build
the verb, and recorded why. The renderer is what forces the unit question, so the unit is settled in the same pass
that names the producer and before `s7i` renders anything: `total_cost_usd` is `CostUsd(String)`, **the decimal
digits verbatim as the CLI printed them, never a float** (`s7c` §7c.1 as amended), and this section records what a
reader may and may not do with them.

### 7f.5 Checkpoint 7f

The following remain true, and the four assertions of §7f.1 are run rather than read:

- **The three ARMED conformance lints exist, by name, and each names the stage that discharges it.** A deleted
  armed lint reads exactly like a discharged one.
- **`shipped_pack_loads` is GREEN** — the product's own `.redkiln/` passes every load-time lint, not merely parses.
- **`redkiln run` no longer refuses at catalog load**, which is this step's whole externally visible effect.
- **`artifacts.toml` carries six toolchain-keyed kinds** and `spikes/p9/artifacts.toml` is unchanged.
- **No `method_rank` moved** — the tie is resolved by keying, per ADR-0056.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
test -d .redkiln/catalog && test -f .redkiln/artifacts.toml || { echo "FAIL: .redkiln/ not derived — Checkpoint 7f NOT RUN (Halt 8)"; exit 1; }
keyed=$(grep -c '^keyed_by = "toolchain"' .redkiln/artifacts.toml); [ "$keyed" -eq 6 ] || { echo "FAIL: $keyed toolchain-keyed kinds, expected 6 (ADR-0073 as amended 2026-09-21)"; exit 1; }
grep -q '^\[artifact.ImplReport\]' .redkiln/artifacts.toml || { echo "FAIL: ImplReport has no registry entry at all"; exit 1; }
spike=$(grep -c '^keyed_by = "toolchain"' spikes/p9/artifacts.toml); [ "$spike" -eq 5 ] || { echo "FAIL: spikes/p9/artifacts.toml moved to $spike keyed kinds — §7b.12 forbids editing the spike registry"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- shipped_pack_loads 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: shipped_pack_loads exited $rc — the product's pack does not load"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched shipped_pack_loads"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- conformance_ --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the conformance listing could not run (rc=$rc)"; exit 1; }
n=$(printf '%s\n' "$out" | grep -c 'conformance_'); [ "$n" -eq 3 ] || { echo "FAIL: expected exactly 3 conformance lints, found $n"; exit 1; }
armed=$(printf '%s\n' "$out" | grep -c 'conformance_.*: test'); [ "$armed" -eq 3 ] || { echo "FAIL: $armed of 3 conformance lints listed as tests"; exit 1; }
out=$(./target/debug/redkiln run rk-fix-doctor-test-count --lane fix 2>&1); rc=$?
printf '%s\n' "$out" | grep -qE 'contains a cycle|RankTie' && { echo "FAIL: the run still refuses at catalog LOAD:"; printf '%s\n' "$out" | head -3; exit 1; }
printf '%s\n' "$out" | head -3   # → READ: the run gets past load. A later refusal (schemas, cards, roles) is s7g's and is not this checkpoint's subject.
echo "CP7F-END-OF-BLOCK reached"
```

---
