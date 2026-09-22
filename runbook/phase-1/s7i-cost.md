# Step 7i — `redkiln cost`: the reading journalled, and ADR-0110's three renderings (Phase 1, part 7)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7i` | Phase 1 | Checkpoint 7h | Checkpoint 7i | `s7h` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Phase-1 exit criterion (7) invokes `redkiln cost <item>`, and Step 8 did not build it —
deliberately, and recorded why. Two blockers, both measured 2026-09-21: there were **zero `dispatch_completed`
events** on the tree because no run completed, and, underneath that, **ADR-0110's batch delta is defined over
*"readings sharing one `resetsAt`"* and nothing journals a `resetsAt`.** `EventKind::RateLimitReading` exists with
the wire name `rate_limit_reading` and has **no producer anywhere**; `DispatchRecord`'s `headroom_before` /
`headroom_after` are `WindowReading { five_hour, seven_day }`, which has no such field.

The first blocker is cleared by `s7h`. The second is cleared here, and **not by amending ADR-0110**: the reading is
already parsed and thrown away. `rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a
`RateLimitReading` carrying `resets_at`, `five_hour_resets_at` and `seven_day_resets_at`, and
`ClaudeHarness::last_reading()` exposes it. Redefining the delta over "readings in one run" would be a different
measure wearing the same name — F6's quantisation of `utilization` to 0.01 is precisely why the `resetsAt` grouping
exists.

**The calculator is already complete and has no caller.** `throttle::BatchDraw` carries `first`, `last`,
`delta_ticks`, `n`, `resolution_limited`, `low_sample` and `non_monotonic`; `throttle::attribute` returns
`Attributable(Vec<BatchDraw>)` or `NotAttributable { interleaving }`; `LOW_SAMPLE_BELOW` is 10 and `delta_ticks` is
`None` when `low_sample`. Every one of ADR-0110's rules is encoded. What is missing is a **producer** for the
samples and a **renderer** for the draws — measured 2026-09-21, the strings `not attributable`, `low_sample` and
`resolution_limited` appear in **no `.rs` file as output**.

**This step takes no decision**: `s7f` §7f.3 names the producer and §7f.4 settles RT-19's unit.

### 7i.1 The producer — the reading stops being discarded

The Claude adapter journals a `rate_limit_reading` row per reading it parses, with the payload fields `s7f` §7f.3's
record names, against design §5's event table. The kind is **tier 2** and the kernel's fold already classifies it as
non-state-moving in three places (`breaker`, `fsm`, `memo`), so no tier-1 projection moves and **criterion (2)
cannot regress** — which Checkpoint 7i asserts rather than assumes.

🛑 **A reading is a measurement, not a derivation.** The row carries what the platform reported, verbatim: no
extrapolation from one utilisation to two windows (AC-374's *"no extrapolation"*, which `RateLimitReading::windows`
already enforces by returning `None` for the flat shape), and no synthesised `resetsAt` when the frame carried none.

### 7i.2 The verb — the three renderings and nothing else

`redkiln cost <item>` reads the item's `dispatch_completed` rows and its `rate_limit_reading` rows, builds the
`Sample` sequence, and renders through `throttle::attribute`. ADR-0110's three, and the data condition of each:

| rendering | condition |
| --- | --- |
| `not attributable`, **with the interleaving count** | any sample in the window is not the account's own |
| `low_sample`, **with no delta quoted** | `n < 10` |
| the batch delta in **integer ticks**, with `n` and `resolution_limited` | otherwise; `resolution_limited` when the raw delta is ≤ 1 |

🛑 **No per-run headroom figure, ever.** `utilization` is quantised to 0.01, so per-run headroom is **not
measurable** (F6) — AC-373. A run that reports one *"has laundered an unmeasurable into a measurement"*, which is
`s8`'s own sentence for it.

🛑 **A vacuous rendering is not a pass.** With no dispatches an honest verb reports that there is nothing to
attribute; it does **not** print a `low_sample` line over `n = 0` and let criterion (7)'s grep read it as green.
Step 8 declined to build the verb rather than ship that shape, and this step may not reintroduce it: Checkpoint 7i
asserts the empty case is distinguishable from the sampled one.

### 7i.3 Checkpoint 7i

- **A `rate_limit_reading` row is journalled** and carries a `resets_at`, so ADR-0110's grouping key exists.
- **`redkiln cost <item>` exists and exits 0** on an item with dispatches, rendering exactly one of the three.
- **The empty case is distinguishable** — an item with no dispatches does not render a `low_sample` delta.
- **No per-run headroom figure appears** (AC-373).
- **Criterion (2) has not regressed** — a tier-2 row added per reading moves no tier-1 projection.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — Checkpoint 7i NOT RUN (Halt 8)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to an item whose lane ran and dispatched"; exit 1; }
out=$(./target/debug/redkiln cost "$FIX_ITEM" 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: redkiln cost exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
empty=$(./target/debug/redkiln cost rk-story-codex-adapter 2>&1); erc=$?
printf '%s\n' "$empty" | grep -q 'low_sample' && { echo "FAIL: an item with no dispatches renders a low_sample delta — a vacuous green (Halt 8)"; printf '%s\n' "$empty"; exit 1; }
printf '%s\n' "$empty" | head -3   # → READ: the empty case says there is nothing to attribute, and says it differently (erc=$erc)
out=$(cargo test -p rk-kernel -- rebuild_differential 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the tier-1 projection moved when a tier-2 reading row was added (rc=$rc) — criterion (2) regressed"; printf '%s\n' "$out" | tail -20; exit 1; }
echo "CP7I-END-OF-BLOCK reached"
```

---
