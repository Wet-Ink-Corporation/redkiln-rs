# Handoff — 2026-09-04

Written at the close of the session that first ran E3. Everything decided is
committed; nothing lives only in conversation. A fresh session needs this file,
`FINDINGS.md`, `METHOD.md` and `METHOD-E3.md`, and nothing else.

**Branch:** `spike/rust-inversion-layer0` · **Corpus pin:** redkiln @ `47faec9f`
(frozen) · **Instrument:** `claude` **2.1.259** (was 2.1.258 for every record
before 2026-09-03 — F20), Windows 11 · 101 tests, 0 clippy warnings, working
tree clean.

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
| **E0** sizing | **measured** | Opus/xhigh: **+0.03** five_hour per S-SILENT dispatch, **~+0.077** per CLEAN |
| **E3** arm (a) | complete | 23/23 detection, 4/23 FP; **0/46 records had the parent review for itself** |
| **E3** cost | measured | in-session costs **1.72×** headless for the same detection ($97.39 vs $56.47) |
| **E0** variance | not started | `--repeat N` on the E3 rig; no item run twice yet |
| **E3** | **ANSWERED** | **kill criterion does NOT fire.** Detection tied 23/23; arms statistically indistinguishable (McNemar p=0.625) |
| **E4 / E5 / E6** | gated | behind E3 |
| **H2** suspension | done | **174/174 lossless** across a real process boundary |
| **H3** world drift | done | 7% overall; **3.6× more likely** on gates > 1 h |
| **H1 / H4** | planned | `METHOD-H.md`; H1 cheap, H4 needs E0 |

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

## Next action: adjudicate the five rejections, then decide what E3 licenses

E3 is answered and the programme does not stop. **But the result is weaker than
the headline reads**, and the next steps follow from that rather than from the
numbers being good.

**1. The κ adjudication (METHOD-E3 §7) — SEVERITY only; the facts are settled.**
All six rejection records were replicated mechanically (**F24**): every claim is
TRUE, six for six, none a hallucination. What remains is whether each warranted
`changes-requested`, which is a judgement and is Ryan's. Four of the six are
"the test is vacuous" rather than "the code is broken", and a reviewer may
legitimately treat those as advisory.

If all are accepted as blocking, both arms go to **0/23 FP and MCC +1.000** —
that is the upper bound, not the result; the reported 4/23 and 2/23 are the
lower. **Either way CLEAN's ground truth is wrong for at least 5 of 23 items
(22%)**, so both measured FP rates are upper bounds on the true rate.

The five items rejected by at least one arm:

| item | (a) | (b) |
| --- | --- | --- |
| `fix-advance-write-lock-gate-scope` | REJECT | REJECT |
| `issue-63-advance-gate-diagnostic-truncated` | REJECT | approve |
| `issue-74-plan-briefs-dead-plan-anchor` | REJECT | approve |
| `issues-135-129-declared-boundary-parse` | REJECT | approve |
| `issue-146-147-boundary-window` | approve | REJECT |

CLEAN's ground truth is "this shipped and a review approved it", which is not
"this is defect-free". Until these are adjudicated **both false-positive rates
are provisional**, and with them both MCCs. `fix-advance-write-lock-gate-scope`
is the one both arms rejected, which makes it the best candidate for a real
missed defect. This must not be settled by the model that built the rig.

**2. Detection is at ceiling, so the primary metric discriminated nothing.**
23/23 versus 23/23 is not a tie that favours anyone — it is a metric with no
power at this difficulty. §9 already treats an all-arms *floor* as grounds for
"inconclusive"; the mirror case deserves the same treatment. A harder seed set
is the fix: novel defects rather than reverted fixes, or the spec withheld, or
the fix's file list not named in the prompt. **That is the highest-value next
experiment**, and it is cheap to specify because the rig exists.

**3. E0 per-item stability is still unmeasured.** `--repeat N` on any item.
23/23 is one sample of a stochastic process, and a variance estimate is owed
before anyone quotes the rate.

**4. Arm (c)** — the mitigation arm — was correctly not run: §8's reduction
order drops it unless (b) loses, and (b) did not lose.

### How to run dispatches (F22), unchanged and still load-bearing

- **Foreground under the tool limit: never killed.** All S-SILENT fits.
- **Anything reaching the harness background: killed**, at any age.
- **`Start-Process` detached: survives.** One ran 12,000s through ~14 killed
  watchers. Use it for CLEAN, and wait on the PID from the foreground.
- Arm (a) runs **slower** than arm (b) — its CLEAN dispatches hit the 2400s
  ceiling once. Pass `--timeout-secs 3600` for arm (a) CLEAN.

```
cd spikes/runner && cargo build     # cargo test/clippy do NOT rebuild the bin
python results/audit-e3.py results/e3.jsonl
```

## What is genuinely unresolved

1. **Does a cache read draw down the subscription window?** Unanswerable at the
   platform's 0.01 resolution: 1.9M tokens moved it by less than one tick, and
   the assistant shares the account. Gate struck, not answered.
2. **E0's per-item stability** — no reviewer has been run twice on one item.
   The rig supports it (`--repeat N`) and E3's records serve both purposes.
3. **The in-session side of the cache asymmetry.** The RFC's claim that
   subagents get "5 minutes, always" is still only documentation. E3 arm (a) is
   the first chance to observe it.
4. **The human gate** — RFC §7 calls it the unoccupied market gap. **Now
   planned**: `spikes/METHOD-H.md` (pre-registration #3, not approved to run).
   H2 (resume determinism across a process boundary) and H3 (world drift during
   a gate) cost nothing in headroom and can run alongside E3; H1 and H4 wait.
   F18 already establishes the shape: ~half of all gates outlive the cache TTL.
5. **8 of 23 items sit outside S0.1** because their fixes touch only
   `workflows/`, not `src/`. A third of the corpus is unmeasured by the gate FPR.
6. **Arm (a) is unbuilt**, and it is the whole comparison. Everything E3 has
   produced so far is one arm against ground truth.
7. **Equivalent-seed triage (METHOD-E3 §10) has not been applied.** At 23/23 it
   cannot lower detection, but the exclusion list is still owed.
8. **The two CLEAN false positives are unadjudicated**, and the co-primary rate
   depends on how they land. Both are specific and mechanism-level:
   `fix-advance-write-lock-gate-scope` (two regressions claimed confirmed by
   differential probe) and `issue-146-147-boundary-window` (AC-015's test is
   vacuous — its regex passes on pre-existing prose). CLEAN's ground truth is
   "this shipped and was approved", which is not "this is defect-free". If
   either finding is real, the FPR is overstated. This needs **Ryan's blind
   annotation** (§7 κ) — it must not be settled by the model that built the rig.
9. **No item has been run twice.** E0's per-item stability (`--repeat N`) is
   still unmeasured, so 23/23 is one sample of a stochastic process.
10. **E3's primary metric saturated.** Both arms detect everything, so the
    design cannot rank them; the whole comparison fell to an underpowered
    co-primary FPR (McNemar p=0.625). E3 answers "does headless review
    degrade?" with "not detectably, on defects this easy" — and nothing
    stronger. E4/E5/E6 inherit that limit.

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
- **A rig that compiles and passes its unit tests has not been run.** Three E3
  defects survived "built and validated" (F20, and the abbreviated-SHA fetch):
  `--json-schema` silently inert under a tool-restricted card, a `\\?\` path
  handed to `git fetch`, and `git fetch <abbrev-sha>` failing where only the
  one path that happened to call `rev-parse` first worked. None was reachable
  from a unit test, because none of them touch the filesystem or the CLI.
  **Validate one item before every set**, and prefer a haiku probe to an Opus
  one when the question is "does this flag work at all".
- **A default was written for the fast condition.** The 900s timeout fitted
  S-SILENT and cost a CLEAN record. When two conditions differ in shape, check
  the ceiling against the slow one.

---

## Pre-registration status

`METHOD.md` covers Stage 0, E0, E1, E2; `METHOD-E3.md` covers E3/E6;
`METHOD-H.md` covers the human gate (planned, unapproved). Five
amendments, each disclosed in the document and its commit, none made after
seeing the data it would affect:

1. E3/E6's primary metric replaced — the answer key did not exist (F9).
2. E3's input set 22 → 23 — my parser, not the criteria (S0.3).
3. E0's cache-cost gate struck — not settleable with this instrument (F14).
4. METHOD-E3 §6 — `StructuredOutput` added to the card's tool set. An
   **instrument** fix, not a metric change: without it `--json-schema` is
   silently inert and every record is void (F20). Made when the only E3
   dispatch in existence was void.
5. METHOD-E3 §5 — CLEAN and S-SILENT built to the same shape (one-commit
   shallow repo, one commit apart). Also instrument, not metric: CLEAN's
   false-positive rate is co-primary with S-SILENT's detection rate and the two
   are comparable only when measured in the same place. Made before any CLEAN
   dispatch.

**The RFC amendment is written**: `references/2026-09-02-rfc-amendment.md`
(charter §14.4). It records R2 falsified, the R4/R6 conflict dissolved and
inverted, §10's pivotal unknown resolved, the cache-TTL premise confirmed,
§6.2's worktree conflict repriced to ~⅓, the multi-agent citation corrected, and
three new risks the RFC did not anticipate. Its closing point is the one that
matters: **every result so far is cost and mechanism; the quality question the
RFC says is decisive has not been tested.**

**Charter §14 deliverables are COMPLETE.** §14.3 is
`references/conformance-2026-09-04-scorecard.md` (Run 3, measured — filed in the
spike repo because redkiln is frozen; move it to
`redkiln/references/conformance/` when the freeze lifts). §14.4 is the RFC
amendment, updated 2026-09-04 with the E3 result. §14.5 is
`references/2026-09-04-findings-for-product.md`.

**Run 3's headline, because it needs acting on:** `forge-fix.js`'s review
consumer is `if (review && review.pass === false)`, which treats an ABSENT
verdict as approval — and F20 is a measured mechanism that produces exactly
that (a tool-restricted card plus a demanded schema yields prose, silently, on
the CLI path). Whether the Workflow runtime shares the defect is UNVERIFIED and
is one dispatch to settle. Fix the consumer either way:
`if (!review || review.pass !== true)`.

**§14.5 is DONE** — `references/2026-09-04-findings-for-product.md`. It carries
the four that were expected plus nine more, split by audience: Part A is Claude
Code platform behaviour (silent `--json-schema` failure under a tool-restricted
card; `--allowed-tools` not restricting the main turn; headroom readable only via
an undocumented event with the binding sub-limit invisible; platform errors
returning exit 0 / `subtype: success`; in-session subagents on a 1h not 5m cache;
background-process kills), Part B is redkiln (per-AC mutation evidence, with a
staged remediation package; a review gate that detects reliably but blocks
unreliably; no affected-area test selection; the empty ledger join; the S0.1
survivor the reviewer caught 4/4 times; a test helper that drops the field which
would have caught a real regression).
