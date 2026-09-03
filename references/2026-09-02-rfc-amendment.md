# RFC Amendment — what the spike established

**Date:** 2026-09-02
**Amends:** `references/2026-08-31-rust-inversion-RFC.md`
**Evidence:** `spikes/FINDINGS.md` (F1–F17), `spikes/results/*.jsonl`
**Charter deliverable:** §14.4 — "an amendment to the RFC recording which claims
survived, which were falsified, and which remain untested."

Platform findings are date-stamped. They describe behaviour we do not control,
on `claude` 2.1.258, Windows 11, one machine, one account.

---

## 0. The short version

**The RFC's central cost argument survives its first real test, and its
headline risk register is materially wrong in the direction of pessimism.**

Three of the RFC's stated risks are dead or smaller than claimed, its one
"pivotal unknown" is resolved, and the platform behaviour its cost case depends
on is confirmed. Against that: the corpus turned out to be unable to support two
of the three gated experiments as designed, and the single most valuable claim —
that inversion improves review quality, or at least does not harm it — remains
**entirely untested**.

Nothing here bears on Bet 2. The engine port was not touched.

---

## 1. Falsified

### R2 — "`effort` has no `--agents` equivalent" (severity: High) — **DEAD**

The RFC feared that headless dispatch could not pin reasoning effort per step,
returning the defect `tiers.mjs` describes: *"the cost and quality profile of a
run was a property of the CALLER, not of redkiln."*

**`--agents` accepts a per-agent `effort` key**, and `--agent <name>` makes the
main `-p` turn adopt the card (F7). Measured (F15), 15 dispatches, one prompt,
only effort varying:

| effort | thinking tokens | mean |
| --- | --- | --- |
| low | 0, 0, 0, 0, 0 | 0.0 |
| high | 0, 0, 24, 0, 0 | 4.8 |
| **xhigh** | 201, 163, 246, 214, 281 | **221.0** |

`high` and `xhigh` separate completely — no overlap. The production-relevant
contrast is exactly this pair (`tiers.mjs` pins `verify:` sonnet/high and
`review:` opus/xhigh). `low` vs `high` did **not** separate on this prompt, so
no monotonic gradient is claimed.

**Retire R2.** Carry forward one operational rule instead: the card is
`model: inherit` with no effort, so a dispatch that omits either silently
inherits the session default — the very defect, arriving by a different door.

### R4 / R6 — the two mitigations "conflict" — **DISSOLVED, and inverted**

The spike's own prior-art pass (PRIOR-ART §9, C3) claimed R6 ("pass everything
explicitly, including `--system-prompt`") disables R4's mitigation
(`--exclude-dynamic-system-prompt-sections` is documented as *ignored with*
`--system-prompt`), so the explicit-payload discipline would **cost** cache
reuse.

**That prediction was wrong, and the truth is better than either option.**
Measured (F16), 50 dispatches, steady state:

| arm | steady cache-read share |
| --- | --- |
| no commits (control) | 1.000 |
| commit between each | 0.689 |
| + `--exclude-dynamic-system-prompt-sections` | 0.846 |
| **+ `--system-prompt`** | **1.000** |

`--system-prompt` **replaces** the default system prompt — the thing carrying
the volatile git snapshot — with static text. There is then nothing volatile
left to invalidate. It does not relocate the churn; it deletes it.

**R6's discipline is the best available cache strategy and it is free.** One
caveat: a new system prompt is a new prefix, so there is a one-time cold write
per distinct prompt, not per dispatch.

---

## 2. Resolved

### §10 — "the single fact that decides whether inversion is cheaper"

RFC §10 could not establish whether `--exclude-dynamic-system-prompt-sections`
removes the git **branch/commit** snapshot or only the working directory, and
called it the fact that decides the cost case for a commit-per-checkpoint
workflow.

**Answer: partially.** It recovers 3,984 of the 7,853 tokens a commit
invalidates — about half (F16, arm c: 0.690 → 0.846). The git snapshot is split
across sections the flag moves and sections it does not.

**But the question is no longer load-bearing**, because `--system-prompt`
recovers all of it.

### The cache-TTL premise — **CONFIRMED**

RFC §3.1's cost case rests on `-p` getting a 1-hour TTL where in-session
subagents are forced to 5 minutes. Prior art threatened this: the platform docs
say the default is 5 minutes, and a well-evidenced (though unconfirmed, and
closed-as-not-planned) report claims Claude Code silently moved 1h → 5m in
March 2026.

**Measured behaviourally (F10)** — the platform's own `usage.cache_creation`
reports `{ephemeral_1h: 0, ephemeral_5m: 0}` and could not answer either way:

| measured gap | cache_read | cache_creation |
| --- | --- | --- |
| 2.1 min | 25,261 | 0 |
| 10.1 min | 25,261 | 0 |
| 45.1 min | 25,261 | 0 |
| 61.6 min | 17,439 | 7,822 |

**The TTL is ~1 hour**, boundary between 45.1 and 61.6 minutes. A 5-minute TTL
cannot produce a full hit with zero re-creation at 45 minutes. The pessimistic
reading is refuted for this configuration on this date.

### §3.1's "tool-definition re-send — worse by default" — **CONFIRMED, quantified**

A default `-p` inherits the operator's whole environment: 12 MCP servers, ~280
tools, 86 slash commands (F8). Measured, the MCP definitions alone were **4,631
tokens — 56% of the cache creation** on a trivial dispatch, and ~35% of wall
clock. `--strict-mcp-config` removes them.

The RFC predicted this. It is now a number, and the mitigation is one flag.

---

## 3. Repriced — smaller than claimed

### Charter §6.2 — "worktrees fragment the cache" — **~⅓ the size stated**

The charter quotes the docs: *"two sessions in different directories build
different prefixes and miss each other's cache. That includes worktrees"*, and
calls it "a direct conflict with Redkiln's architecture."

**Measured (F13), 20 dispatches in 20 brand-new git repos**: every one returned
**17,439 tokens read from cache** and only ~7,827 created. The prefix has two
segments — a ~17.4k base **shared across directories**, and a ~7.8k
per-directory part. A worktree misses **31%** of the prefix, not 100%.

Independently corroborated: the 61.6-minute TTL decay returned *exactly* the
same 17,439 / 7,822 split, so what expires is the per-directory segment while
the shared base survives. Two methods, two experiments, the same two numbers.

**Consequence for the target design:** parallel DAG execution across worktrees
(shipmates L6.1, "the largest single capability gain available") costs ~31% of
the prefix per worktree, and should cost nothing under `--system-prompt` —
though that combination was not run.

### §3.1's "per-run process overhead — worse, mostly latency" — **CONFIRMED, small**

The platform reports its own split: `duration_ms − duration_api_ms` ≈ **334 ms**
of a ~3.4 s trivial dispatch. Real, and negligible against steps that run for
minutes.

---

## 4. Corrected — a citation the RFC and charter both got wrong

The charter cites "Kim et al. — 260 configurations, 5 architectures, 6
benchmarks, compute-matched … effects from +80.8% to −70.0%" as evidence that no
architecture dominates.

The actual study (Google/MIT) is **180 configurations, 5 architectures, 4
benchmarks**, and the effect range is **not a general spread — it is structured
by task type**:

| finding | figure |
| --- | --- |
| **sequential** tasks — multi-agent **degrades** | **−39% to −70%** |
| **parallelizable** tasks — centralised coordination **improves** | **+80.9%** |
| error amplification, independent agents | 17.2× |
| error amplification, centralised | 4.4× |

**This is more useful than the claim it replaces, and it points somewhere
specific.** `forge-implement.js` is annotated *"STRICTLY SEQUENTIAL over
slices"*; `forge-fix.js` is the most sequential lane we have. Sequential is
precisely where the literature measures the largest losses.

It also dissolves an apparent tension between our own documents: **trim
orchestration where we are sequential (the fix lane), add it where the project
DAG is genuinely parallel.** The RFC and the shipmates lessons were not in
conflict; they were describing different halves.

**E4's null arm therefore has a specific prior in its favour**, and a null-arm
win should be read as expected rather than surprising.

---

## 5. Untested — and one of these is the whole thesis

### The quality question — **NOT TESTED**

**No measurement of review quality, headless or in-session, exists.** E3's rig is
built and validated; it has not been run. The RFC's §9 kill condition —
"Experiment 2 shows material verdict divergence. **This alone kills it**" —
remains entirely open.

Everything above concerns cost and mechanism. **None of it is evidence that
inversion preserves agent quality**, which is the claim the RFC itself says
matters most.

### The in-session half of the cache asymmetry — **NOT TESTED**

"Subagents get 5 minutes, always" is still taken from documentation. E3 arm (a)
is the first opportunity to observe it.

### The human gate — **NOT TESTED, and it is the differentiator**

RFC §7 identifies the unoccupied market gap as "nobody ships a mid-run human
approval gate", and R12 treats the UX as new but the state model as settled.
**There is no spike for it.** It remains the largest untested assumption in the
thesis: the capability that would justify building this has no evidence behind
it.

### Bet 2 — **UNTOUCHED**

No YAML parity work, no engine port investigation. R3 stands exactly as written.

---

## 6. New risks the RFC did not anticipate

**N1 — An agent with `Bash` can defeat a git-based fixture.** The first E3 pilot
reverted a fix inside a worktree whose object store still held the fix commit.
The reviewer checked it back out, reviewed the real code, and ran its own
mutation experiment (F17). Every record from that rig would have scored a
reviewer looking at *fixed* source against *reverted* ground truth — firing the
programme's hardest kill criterion on a fixture artefact. Fixtures must make the
original **unreachable**, not merely absent.

Generalised for the product: **an agent given repository tools will use the
repository.** Any orchestrator that constructs a partial or synthetic tree and
assumes the agent reviews what it was handed is making the same mistake.

**N2 — The corpus cannot support outcome metrics that assume recorded failure.**
Charter §8.1 anticipated censoring; the reality was worse. E3's answer key did
not exist (all fifteen `## Required Changes` sections open with "None"), E4's
ledger join was empty (no fix-lane item has a ledger), and only 1 of 101 runs
records resume state. Three metrics, all registered, none computable.

**N3 — Instrument resolution can defeat a measurement outright.** Plan headroom
is quantised to 0.01; 1.9M tokens moved it by less than one tick. The
cache-cost gate was struck rather than answered.

---

## 7. Recommendation, revised

**Unchanged: take Bet 1, stage Bet 2, keep the plugin surface.** Nothing found
here argues against inversion, and several findings strengthen its cost case.

**Two changes to how it should be framed:**

1. **The cost case is stronger than the RFC claimed, and for a different
   reason.** Not "one shared prefix at 1h TTL" alone, but that the prefix is
   two-segment, largely directory-independent, and fully preservable across
   commits under `--system-prompt`. Commit-per-checkpoint costs 31% unmitigated
   and 0% mitigated.
2. **The orchestration case should be split by task shape.** Trim the sequential
   fix lane; invest in parallel DAG execution. That is what the corrected
   literature says, and it reconciles the RFC with the shipmates lessons.

**Unchanged and now more prominent:** the decision still rests on E3. Every
result in this amendment is about cost and mechanism, and the RFC is right that
verdict divergence alone kills the thesis. **That experiment has not been run.**
