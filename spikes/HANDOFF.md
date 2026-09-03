# Handoff — 2026-09-02

Written to close a session at ~700k context. Everything decided is committed;
nothing lives only in conversation. A fresh session needs this file,
`FINDINGS.md`, `METHOD.md` and `METHOD-E3.md`, and nothing else.

**Branch:** `spike/rust-inversion-layer0` · **Corpus pin:** redkiln @ `47faec9f`
(frozen) · **Instrument:** `claude` 2.1.258, Windows 11 · 92 tests, 0 clippy
warnings, working tree clean.

---

## Where the programme actually stands

| | status | headline |
| --- | --- | --- |
| **Layer 0** | done | step runner, typed failure, resume, isolation, provenance |
| **S0.1** gate FPR | done | **90% killed** of test-reachable mutants; 1 survivor |
| **S0.2** reconcile | done | **20.3%** of slices were rejected mid-flight |
| **S0.3** manifest | done | 23 of 23 items eligible |
| **E1** incl. TTL | done | TTL **~1 h**; commits cost **31%** of prefix; `--system-prompt` costs **0** |
| **E2** effort | **closed** | effort pins per dispatch; RFC R2 falsified |
| **E0** headroom | partial | sizing bound only; cache-cost gate **struck** (disclosed) |
| **E0** variance | not started | needs the E3 rig, which now exists |
| **E3** | **rig ready, unrun** | the pilot measured the rig, not the reviewer |
| **E4 / E5 / E6** | gated | behind E3 |

### The four results that changed the thesis

1. **The cache TTL is ~1 hour** (F10), boundary between 45.1 and 61.6 minutes.
   The biggest threat to the RFC's cost case — that Claude Code had silently
   moved to a 5-minute TTL — is refuted for this configuration on this date.
2. **`--system-prompt` is the best cache strategy, not the worst** (F16). I had
   predicted RFC R6 and R4 conflict. They do not: replacing the default system
   prompt deletes the volatile git snapshot rather than relocating it. Steady
   state 1.000 across commits, versus 0.689 unmitigated.
3. **69% of the prefix is shared across directories** (F13). Charter §6.2's
   worktree-fragmentation concern is real but about a third the size the docs
   imply — a worktree costs 31%, not 100%.
4. **Two of the three gated experiments had unusable oracles as chartered**
   (F9, C1). E3's answer key did not exist; E4's ledger join was empty. Both
   were replaced before anything was spent.

---

## Next action: run E3

The rig is built and validated. Nothing blocks it but headroom.

```
cd spikes/runner
cargo build                       # ALWAYS: cargo test/clippy do NOT rebuild the bin

# validate one item first — this is how F17 was caught
./target/debug/redkiln-spike-runner review-probe \
    --out ../results/e3.jsonl --condition s-silent \
    --only issues-135-129-declared-boundary-parse

# then chunk: ~9 min/dispatch, Opus/xhigh
./target/debug/redkiln-spike-runner review-probe \
    --out ../results/e3.jsonl --condition s-silent --limit 4
./target/debug/redkiln-spike-runner review-probe \
    --out ../results/e3.jsonl --condition clean --limit 4
```

Resume is on and keyed per `(arm, slug, repeat)`, so a stopped run costs one
dispatch. `--repeat N` gives the per-item stability E0's variance pilot needs;
the same records serve both.

**Before trusting any record, check three things** — each is a failure mode that
already occurred once:

- `card_adopted == true` — F17: the card applies but `effective.agent` is empty;
  adoption is verified from the **tool set** (`["Read","Bash"]`, down from 34).
- the `s-silent` fixture printed no `node_modules` warning — a reviewer that
  *cannot* run tests looks identical to one that *chose not to*.
- `assert_fix_unreachable` passed — it runs per item and hard-fails.

**Sizing:** unknown. No Opus dispatch has a measured headroom draw yet. The
struck gate was replaced by "size each arm from its own measured draw", so the
first chunk **is** the sizing measurement — record `seven_day` before and after.

---

## What is genuinely unresolved

1. **Does a cache read draw down the subscription window?** Unanswerable at the
   platform's 0.01 resolution: 1.9M tokens moved it by less than one tick, and
   the assistant shares the account. Gate struck, not answered.
2. **E0's per-item stability** — no reviewer has been run twice on one item.
3. **The in-session side of the cache asymmetry.** The RFC's claim that
   subagents get "5 minutes, always" is still only documentation. E3 arm (a) is
   the first chance to observe it.
4. **The human gate** — RFC §7 calls it the unoccupied market gap, and it has no
   spike. Still the largest untested assumption in the thesis.
5. **8 of 23 items sit outside S0.1** because their fixes touch only
   `workflows/`, not `src/`. A third of the corpus is unmeasured by the gate FPR.

---

## Traps that cost time here — worth not repeating

- **`cargo test` and `cargo clippy` do not rebuild the binary.** I ran a stale
  binary four times and twice concluded code was broken when it had not
  compiled. Always `cargo build` before running.
- **Scripted edits need an assertion.** Two silent `str.replace` no-ops were
  taken for behaviour changes. Assert the anchor exists.
- **`until grep` waiters spin forever if the job dies** — every marker must have
  a failure alternative (`grep -qE "MARKER|exited with code"`). One waiter span
  25 minutes on a job that had already failed.
- **Background jobs get killed unpredictably** (~13 min survived, ~35 and ~55
  did not). Nothing may depend on one process surviving; resume everywhere.
- **Guards need the right scope.** Two guards I wrote to protect data were
  scoped wrong and blocked or voided valid work.

---

## Pre-registration status

`METHOD.md` covers Stage 0, E0, E1, E2; `METHOD-E3.md` covers E3/E6. Three
amendments, each disclosed in the document and its commit, none made after
seeing the data it would affect:

1. E3/E6's primary metric replaced — the answer key did not exist (F9).
2. E3's input set 22 → 23 — my parser, not the criteria (S0.3).
3. E0's cache-cost gate struck — not settleable with this instrument (F14).

**The RFC amendment is written**: `references/2026-09-02-rfc-amendment.md`
(charter §14.4). It records R2 falsified, the R4/R6 conflict dissolved and
inverted, §10's pivotal unknown resolved, the cache-TTL premise confirmed,
§6.2's worktree conflict repriced to ~⅓, the multi-agent citation corrected, and
three new risks the RFC did not anticipate. Its closing point is the one that
matters: **every result so far is cost and mechanism; the quality question the
RFC says is decisive has not been tested.**

Remaining deliverables from charter §14: the dated conformance scorecard (§14.3)
and the findings note for product changes (§14.5) — four are already expected
(the worktree/cache interaction, the gate FPR, the absent affected-area test
selection, and the empty ledger join).
