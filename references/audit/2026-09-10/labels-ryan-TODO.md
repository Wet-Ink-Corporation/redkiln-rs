# Operator labels over the 2026-09-10 audit — audit §8 M-06

**What this is.** The audit's 338 survivors and 860 verdicts were model-labelled only; no human adjudicated any of them (§8 M-06).
This file is the corpus's own label shape (id · agree / disagree / unsure · note) over the 17 P1 concerns and a seeded random 30 of the 82 P2
(seed 20260911, `random.Random(20260911).sample(p2_ids, 30)`). Fill it **before Step 5**; do not re-threshold any priority after the labels
are in — report the disagreement count beside the deliverable in `runbook/PROGRESS.md` §5.

labeller: Ryan Britton

**Split 2026-09-11 (Ryan, via AskUserQuestion):** the 17 P1 rows are labelled first — they are the calls that shaped ADR-0093…0101, so a disagreement matters before Step 4.4 writes ADR-0090; the seeded 30 P2 rows follow later, before Step 5, under their own timestamp. Same seed, same rows, no re-prioritisation after labelling.

## Part 1 — the 17 P1 concerns (label now)

labelled_at_p1: FILL-ME   # UTC, stamped by command

| id | priority | title (abridged) | label (agree / disagree / unsure) | note |
| --- | --- | --- | --- | --- |
| PIA-001 | P1 | AC-520 fails pack load on `stage`, `rigor`, `role`, `cost` and `recent_deploy` with no RUNBOOK step or registe | FILL-ME | |
| PIA-002 | P1 | The corpus Steps 6-7 test against is neither moved by Step 2 nor reachable from the pinned SHA: the 1,312-file | FILL-ME | |
| PIA-003 | P1 | The three LLM boolean facets have no disposition between Step 4's pre-registered rule and Checkpoint 4a: ADR-0 | FILL-ME | |
| PIA-004 | P1 | Phase-1 exit (1) self-hosts on a Rust repo with only a Node toolchain adapter | FILL-ME | |
| PIA-005 | P1 | rk-types' closed-output schema rules cannot all hold: 'no `oneOf` anywhere' (ADR-0084:64, RUNBOOK 5.3) vs nest | FILL-ME | |
| PIA-006 | P1 | The companion kind list is delegated to a runbook that never mentions it | FILL-ME | |
| PIA-007 | P1 | `Findings.repro` as fixed by ADR-0074 and snapshotted at Step 5.3 has no field for the mutant or suite that AC | FILL-ME | |
| PIA-008 | P1 | The memo record is specified three ways and its keys are unpinned: ADR-0086's SQLite tier vs AC-161/AC-187 | FILL-ME | |
| PIA-009 | P1 | ADR-0077 keeps linked-worktree common-dir resolution on gix -- the interplay it says gix lacks and the operati | FILL-ME | |
| PIA-010 | P1 | The Harness port's `dispatch`/`cancel(handle)` contract is unimplementable as written: blocking dispatch, a ha | FILL-ME | |
| PIA-011 | P1 | Pack authoring and `.redkiln/` have no owning step | FILL-ME | |
| PIA-012 | P1 | cache_profile = default names a profile with no delivery mechanism | FILL-ME | |
| PIA-013 | P1 | ADR-0088 misnames its source paper and inverts its mechanism: under bug-free framing false positives collapse, | FILL-ME | |
| PIA-014 | P1 | rk-adapter-claude: subscription exhaustion mid-dispatch has no error class and backs off | FILL-ME | |
| PIA-015 | P1 | AC-212/236/036 require the card to carry model+effort | FILL-ME | |
| PIA-016 | P1 | Tool set unequal to card allowlist is NotAdopted in AC-204 but NotIsolated in AC-202/AC-386/design 7.4 | FILL-ME | |
| PIA-017 | P1 | AC-372/376/377 state tick rules in decimal fractions: a naive f64 implementation passes every named vector yet | FILL-ME | |

## Part 2 — the seeded 30 P2 concerns (label before Step 5)

labelled_at_p2: FILL-ME   # UTC, stamped by command

| id | priority | title (abridged) | label (agree / disagree / unsure) | note |
| --- | --- | --- | --- | --- |
| PIA-019 | P2 | The corpus's account of its own tree and instrument is stale: RUNBOOK 0.1/1.3 describe HEAD ce04058 and hard-c | FILL-ME | |
| PIA-021 | P2 | Step 2's pin script leaves the arm-L blind labels in one unpushed copy, stamps a literal PINNED_AT and re-admi | FILL-ME | |
| PIA-029 | P2 | Register self-description fails verification: seven ADR clauses have no criterion, cargo-deny is unprovisioned | FILL-ME | |
| PIA-033 | P2 | rk-store and redkiln-migrate contracts: locator end rule, Gwt shape, conservation vocabulary, and no locator/c | FILL-ME | |
| PIA-035 | P2 | Checkpoints 3c, 5, 6 and 7b/8 carry lines green on the untouched tree or on a violation | FILL-ME | |
| PIA-038 | P2 | About 95 criteria are assigned to eight crate labels that exist in neither design section 9.1 nor the RUNBOOK  | FILL-ME | |
| PIA-040 | P2 | rk-git's and rk-store's Step-6 contracts are lists, not rules: unowned git ops (patch-id, merge, common-dir fa | FILL-ME | |
| PIA-044 | P2 | Step-3 drift contract: `stale_policy` has no schema and AC-067's vouched-root trigger names no edge set | FILL-ME | |
| PIA-047 | P2 | No step builds or checkpoints the three gates (AC-240–AC-359), filed under 7c | FILL-ME | |
| PIA-050 | P2 | Gate semantics: loop-back re-reach has no AC, escalate chains are unbounded, the tree hash of an empty commit  | FILL-ME | |
| PIA-052 | P2 | `null` in `require` predicates: guard narrowing undefined, `null == <non-null>` undefined, and three outcomes  | FILL-ME | |
| PIA-053 | P2 | Predicate grammar for the 7b parser has no precedence/associativity, no mixed-unparenthesised rule and no lite | FILL-ME | |
| PIA-056 | P2 | An equivalent mutant survives re-execution, so ADR-0074's `mutation-survived` class can block on a no-op mutat | FILL-ME | |
| PIA-057 | P2 | AC-388's per-sibling stagger chain is not the vendor's mechanism (hold until first response begins, cap 5 s, r | FILL-ME | |
| PIA-059 | P2 | Five Phase-1 criteria's instruments pass on the behaviour they forbid or read inputs nothing writes: AC-173, A | FILL-ME | |
| PIA-061 | P2 | Headroom contract: batch delta undefined across a window reset and shared-account fan-out, SM-111's n>=10 floo | FILL-ME | |
| PIA-064 | P2 | Claude Code `defer` (`tool_deferred`) shipped in 2.1.89 on 2026-03-31 and is documented at code.claude.com/doc | FILL-ME | |
| PIA-066 | P2 | ADR-0016/design 7.1 say S-WT measured a `--system-prompt` 'carrying the card body' | FILL-ME | |
| PIA-070 | P2 | Summary rows, acceptance-set ranges, cross-reference ids and restated figures overstate, mis-point or were nev | FILL-ME | |
| PIA-073 | P2 | Statistics tagged [measured] or settled do not reproduce under the registered two-sided Wilson interval: MCC + | FILL-ME | |
| PIA-075 | P2 | Corrections and restorations recorded as applied left copies uncorrected: F35 section 1.3 still 'must be re-gr | FILL-ME | |
| PIA-077 | P2 | Spike-draft headline figures do not reproduce from their result files: F36 $9.70, F14 ~1.9M tokens, F39 140 ce | FILL-ME | |
| PIA-080 | P2 | F38's 0.45x/0.55x/0.78x divides P8 output tokens by F28's cache-creation tokens | FILL-ME | |
| PIA-082 | P2 | Five DECIDED Phase-1 rules cite ADRs whose bodies do not contain them and no ADR does: the CLI shape (0011), t | FILL-ME | |
| PIA-083 | P2 | ADR-0084's advisory evidence misattributes three gix advisories, 'no advisory affects bundled' is false, and g | FILL-ME | |
| PIA-090 | P2 | The narrowed novelty claim 'in an AI-coding-specific backlog or spec tool' is false: spec-kit, Archon and Bern | FILL-ME | |
| PIA-093 | P2 | ADR-0089's 'Semgrep 0.57%-23.8% recall' floor is from a mis-cited arXiv ID (RealVuln: recall 17.5%) | FILL-ME | |
| PIA-095 | P2 | Review-gate ADRs mis-state F24/F31: 0082/0088 credit F24 with '≥13 of 23' (F24 said ≥5/23 | FILL-ME | |
| PIA-098 | P2 | The rank-reversal sentence behind 'rigor is ordinal, never summed' is pinned to a TOPSIS-only paper | FILL-ME | |
| PIA-099 | P2 | Principle 8 was changed on a vendor compaction quote ('nuanced understanding ...') that is on no live first-pa | FILL-ME | |

**Disagreement count:** FILL-ME / 47
