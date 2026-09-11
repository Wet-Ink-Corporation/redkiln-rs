# Runbook progress — the tracker

**What this is.** The single place that says where the Redkiln-RS runbook stands: which stage is live, which decisions are
recorded, which of the audit's 110 concerns are closed, and what the validators last said. `runbook/INDEX.md` is the map;
this file is the odometer. Update it in the same commit as the work it records; a row that says `done` without a commit
hash beside it is a claim, not a record.

**Legend.** Stage status: `not started` · `in progress` · `checkpoint passed <hash>` · `halted (Halt N)`. PIA disposition:
`open` · `done` (edit landed) · `recorded` (decision landed as ADR/DECISIONS) · `awaiting-ryan` · `deferred (<reason>)` ·
`not-applicable (<reason>)`. Every non-`open` row names where it landed.

---

## 1. Stages

Execution order is `0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7c → 7d → 7e → 8` (Step 2 runs sixth — see its ORDER banner).

| Stage | File | Status | Entry gate | Exit checkpoint | Result / commit | Blocker |
| --- | --- | --- | --- | --- | --- | --- |
| R — remediation of the 2026-09-10 audit | (this pass) | in progress | — | §4 all rows non-`open`; validators re-run (§6) | — | — |
| 0 — Preconditions and safety | `phase-0/s0-preconditions.md` | not started | — | Checkpoint 0 | | |
| 1 — Commit the spike output | `phase-0/s1-commit-spike-output.md` | not started | Checkpoint 0 | Checkpoint 1 | | |
| 3 — Review-consumer check and the label files | `phase-0/s3-review-consumer-and-labels.md` | not started | Checkpoint 1 | Checkpoints 3a/3b/3c | | |
| 4 — The blocking gate | `phase-0/s4-blocking-gate.md` | not started | Checkpoint 3c | Checkpoint 4 (ADR-0090 exists) | | |
| 4a — Apply ADR-0089 and ADR-0093 | `phase-0/s4a-apply-adr-0089.md` | not started | Checkpoint 4 | Checkpoint 4a | | |
| 2 — Extract `redkiln-corpus` | `phase-0/s2-corpus-extraction.md` | not started | Checkpoint 4a | Checkpoint 2 | | |
| 5 — Workspace, CI, `rk-types` | `phase-1/s5-workspace-ci-rk-types.md` | not started | 🚦 gate + Checkpoint 2 | Checkpoint 5 | | |
| 6 — `rk-store` and `rk-git` | `phase-1/s6-store-and-git.md` | not started | Checkpoint 5 | Checkpoint 6 (ADR-0091 exists) | | |
| 7a — `rk-ports` and the fakes | `phase-1/s7a-ports-and-fakes.md` | not started | Checkpoint 6 | Checkpoint 7a | | |
| 7b — `rk-kernel` (with 7b-ii, the gates) | `phase-1/s7b-kernel.md` | not started | 🛑 STOP banner | Checkpoint 7b | | |
| 7c — The adapters | `phase-1/s7c-adapters.md` | not started | Checkpoint 7b | Checkpoint 7c | | |
| 7d — `rk-cli` and `redkiln-migrate` v0 | `phase-1/s7d-cli-and-migrate.md` | not started | Checkpoint 7c | Checkpoint 7d | | |
| 7e — Pack authoring | `phase-1/s7e-pack-authoring.md` | not started | Checkpoint 7d | Checkpoint 7e | | |
| 8 — The exit criterion | `phase-1/s8-exit-criterion.md` | not started | Checkpoint 7e | Checkpoint 8 | | |
| 9 — Rollback | `s9-rollback.md` | n/a | — | Checkpoint 9 | | |

## 2. Decision register

Decisions the audit assigned to the operator, walked one by one on 2026-09-11. `recorded` names the record; `awaiting-ryan` names what is owed.

| # | Concern | Decision taken | Record | Status |
| --- | --- | --- | --- | --- |
| D-A | convention | Errata to frozen ADRs live in `references/adr/ERRATA.md`; the precedence rule; the cut-id re-issue licence | ADR-0092 | to record |
| D-B | PIA-001 | `stage`/`rigor`/`role`/`cost` → `source = "kernel"`; `surfaces` → `"declared"`; `recent_deploy` stays derived (NOT MEASURABLE); S-BIN gate scoped to the `ChangeSummary` type | ADR-0093 | to record |
| P7-CEIL | PIA-001 | `bisect` is a ceiling activity: `max_rigor = "recommended"`; lattice re-run | DECISIONS-2026-09-11 #1 | to record |
| D-C | PIA-003 | `has_repro`/`external_dep`/`market_facing` → `source = "human"` at triage; carried into ADR-0090 at Step 4.4 | s4 §4.1 pre-registration; DECISIONS #2 | to record |
| D-D | PIA-004/072 | Phase-1 adapters {SQLite, Claude, Node, Rust}; exit = fix lane AND a story-lane run (AC-521; Step 8 check 1b) | ADR-0094 | to record |
| D-E | PIA-005/014 | No `oneOf`; `anyOf` via post-derive transform; ≤16 union-typed and ≤24 optional per exported schema (vendor per-request scope stated); the StepError table | ADR-0095; S-SCHEMA-CAP | to record |
| D-F | PIA-006 | Phase-1 companion kinds: `_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`; the other eight census kinds are Phase 2 | s5 §5.3 decision; DECISIONS #3 | to record |
| D-G | PIA-007/056 | Typed repro (`kind`, `mutation`, machine-readable `expected`); prose-only rejected; `mutation-survived` needs a reverted-fix mutant or a discriminating assertion | ADR-0096 | to record |
| D-H | PIA-008 | Memo stays tier 2; AC-161/164 narrow to the stage-level next action; AC-187 scoped; memo key = SHA-256 over a length-prefixed ordered tuple | ADR-0097 | to record |
| D-I | PIA-009 | S-GIX/a no-go → `git rev-parse --git-common-dir` via the wrapper; the review clone is a retained gix WRITE op | ADR-0098 | to record |
| D-J | PIA-010/045/059 | Blocking `dispatch` returning `DispatchOutcome`; `CancelToken` owned by the composition root; `Arc<dyn Port + Send + Sync>`; Windows Job Object / POSIX process group; Harness alone carries cancel | ADR-0099; S-PORTS | to record |
| D-K | PIA-011/044 | New Step 7e Pack authoring; `stale_policy` = `rematerialise`, coarse scope, every class | s7e; DECISIONS #4 | to record |
| D-L | PIA-012 | `cache_profile = default` is F28's arm: no `--system-prompt`, the card prepended to the user message | AC-383; design §7.1 | to record |
| D-M | PIA-016 | Tool-set inequality is `NotIsolated` everywhere; AC-204 amended | DECISIONS #5 | to record |
| D-N | PIA-002/028/029 | `git bundle --all` of the predecessor into redkiln-corpus; corpus-bound tests a local-only `corpus` feature; the YAML reader named (verified live); cargo-deny + `deny.toml`; the explicit 0.x pin list | s2; ADR-0100 | to record |
| D-O | PIA-013 | ADR-0088 superseded in full (real title, prior-approval framing arm, measured effect, FN-up/FP-down mechanism); the symmetric arm registered as S-SUBTLE-SYM | ADR-0101; S-SUBTLE-SYM | to record |
| D-Q | PIA-023/026/027/079 | `rust-version` = the pin; member `[lints]` is `workspace = true` only; `#[expect]`; evidence lines corrected | ADR-0102 | to record |
| D-R | PIA-048/052/053/100 | Predicate grammar: `not` > comparison/`in` > `and` > `or`, left-associative; literal-vs-field; quoting; null semantics; quantifier null rules; the closed `on_false` set | ADR-0103; S-GRAMMAR | to record |
| D-S | PIA-049/051 | Author-declared per-AC mutants; the bootstrap clause; a quarantined survivor still blocks | ADR-0104 | to record |
| D-T | PIA-040/043/050 | The rk-git/rk-store contract (GitOps seam, hooks isolation, trailer plumbing with `--if-exists=add`, write-tree `tree_hash`, the wrapper write set, bare clone, `busy_timeout` 5000 ms, one writer, the event-kind table, retention) | ADR-0105 | to record |
| D-U | PIA-080 | ADR-0087's "costs less" withdrawn; comparator = `cache_creation_input_tokens` + `total_cost_usd` | ADR-0106 | to record |
| — | PIA-082 | CLI shape; StructuredOutput configuration-time check; reconcile-at-every-entry + tiers + trailer axes; headroom rules | ADR-0107–0110 | to record |
| — | PIA-073 | The S-P7B re-run must change the criterion, not only n | ADR-0111 | to record |
| — | PIA-014 | argv cap before spawn; `--system-prompt-file` registered as S-SPF; AC-523 `RateLimited` | AC-523; S-SPF | to record |
| — | PIA-071 | AC-312 PROVISIONAL on S-SUBTLE-CB; fixtures authored, unrun | AC-312 note | to record |
| — | PIA-018 | Arm L outstanding; labelled later at a new corpus pin | DECISIONS #6 | to record |
| — | PIA-046 | Specificity = subsumption; cross-pack order = enabled list then declaration; `discouraged` reserved with a lint; AC-101/SM-026 pack-default | design §4.3/4.4 | to record |
| — | PIA-046 | METHOD-P override-mode keep/strike | — | **awaiting-ryan** |
| — | PIA-050 | Fresh gate record on re-reach; escalate-to-escalate refused; the write-tree rule | new AC; design §6.3 | to record |
| — | PIA-038 | The eight crate labels are modules; mapping table in AC §0 | AC §0 | to record |
| — | PIA-024 | The rustc pin moves to **1.98.1**; the instrument move recorded | s0; DECISIONS #7 | to record |
| — | PIA-078/084/096 | S-MIG counts 76/87 and 96.0%; the S-FACET kill re-registered (paired FP count); METHOD-P amendment 9 scoped | drafts; change-classification §6; METHOD-P | to record |
| — | PIA-096 | The `blind_attestation:` line for F39's 32 adjudicated cells | `spikes/results/p1-arm-d-labels-ryan.md:27` | **awaiting-ryan** |
| — | PIA-106/108 | Namespace prefix `rk-`; κ 0.37 tagged [inferred, unsourced] | DECISIONS #8; PRIOR-ART | to record |
| — | PIA-060 | Child env = inherit minus a committed denylist; the captured map as a value; a `tools:`-less card is a doctor error | AC-217/488/422; design §3.6.1 | to record |
| — | PIA-032/033 | The `ReviewWeight` projection; structured `Gwt`; the closed ledger enum; `EvidenceClass` vs `ReExecution` | AC §9–10; design §5 | to record |

## 3. Instrument choices

| Item | Value | Where recorded |
| --- | --- | --- |
| Rust toolchain pin | `1.98.1` (the spike programme ran on 1.97.1; the move is recorded per golden constraint 8) | s0 §0.2 / §0.3 |
| Claude Code instrument | read from `spikes/results/instrument-checks/RUNBOOK-instrument.txt` at run time (last measured 2.1.268 on 2026-09-10) | s0 §0.3 |
| Corpus pin | `47faec9f46b8d983c2c75f0794164e68415ae706` (predecessor) · the redkiln-corpus SHA in `CORPUS_PIN` after Step 2 | s2 |

## 4. PIA ledger (110)

Columns: id · priority · build stage first reached · classification (NEW / SHArpens-known) · owner kind · title (abridged) · disposition · landed in. The full statement and remediation text of every row is `references/audit/2026-09-10/MASTER.json`.

| id | P | stage | class | kind | title | disposition | landed in |
| --- | --- | --- | --- | --- | --- | --- | --- |
| PIA-001 | P1 | pre-5 | SHA | decision | AC-520 fails pack load on `stage`, `rigor`, `role`, `cost` and `recent_deploy` with no … | open | — |
| PIA-002 | P1 | pre-5 | SHA | decision | The corpus Steps 6-7 test against is neither moved by Step 2 nor reachable from the pin… | open | — |
| PIA-003 | P1 | pre-5 | NEW | decision | The three LLM boolean facets have no disposition between Step 4's pre-registered rule a… | open | — |
| PIA-004 | P1 | 5 | NEW | decision | Phase-1 exit (1) self-hosts on a Rust repo with only a Node toolchain adapter | open | — |
| PIA-005 | P1 | 5 | SHA | decision | rk-types' closed-output schema rules cannot all hold | open | — |
| PIA-006 | P1 | 5 | SHA | decision | The companion kind list is delegated to a runbook that never mentions it | open | — |
| PIA-007 | P1 | 5 | NEW | decision | `Findings.repro` as fixed by ADR-0074 and snapshotted at Step 5.3 has no field for the … | open | — |
| PIA-008 | P1 | 6 | SHA | decision | The memo record is specified three ways and its keys are unpinned | open | — |
| PIA-009 | P1 | 6 | SHA | decision | ADR-0077 keeps linked-worktree common-dir resolution on gix -- the interplay it says gi… | open | — |
| PIA-010 | P1 | 7a | SHA | edit | The Harness port's `dispatch`/`cancel(handle)` contract is unimplementable as written | open | — |
| PIA-011 | P1 | 7b | SHA | decision | Pack authoring and `.redkiln/` have no owning step | open | — |
| PIA-012 | P1 | 7b | NEW | decision | cache_profile = default names a profile with no delivery mechanism | open | — |
| PIA-013 | P1 | 7c | SHA | decision | ADR-0088 misnames its source paper and inverts its mechanism | open | — |
| PIA-014 | P1 | 7c | SHA | edit | rk-adapter-claude | open | — |
| PIA-015 | P1 | 7c | NEW | decision | AC-212/236/036 require the card to carry model+effort | open | — |
| PIA-016 | P1 | 7c | SHA | decision | Tool set unequal to card allowlist is NotAdopted in AC-204 but NotIsolated in AC-202/AC… | open | — |
| PIA-017 | P1 | 7c | NEW | edit | AC-372/376/377 state tick rules in decimal fractions | open | — |
| PIA-018 | P2 | pre-5 | NEW | decision | Three undisclosed ordering hazards | open | — |
| PIA-019 | P2 | pre-5 | SHA | decision | The corpus's account of its own tree and instrument is stale | open | — |
| PIA-020 | P2 | pre-5 | SHA | edit | ADR-0079's external corroboration overstates three preprints | open | — |
| PIA-021 | P2 | pre-5 | SHA | decision | Step 2's pin script leaves the arm-L blind labels in one unpushed copy, stamps a litera… | open | — |
| PIA-022 | P2 | pre-5 | NEW | decision | Migrate tool and corpus split | open | — |
| PIA-023 | P2 | 5 | SHA | edit | ADR-0083's and ADR-0085's evidence lines misquote or misattribute their sources | open | — |
| PIA-024 | P2 | 5 | SHA | edit | RUNBOOK pins rustc 1.97.1 and greps for '1.97' while stable is 1.98.1 | open | — |
| PIA-025 | P2 | 5 | NEW | edit | RUNBOOK 5.1's verbatim root Cargo.toml fails `cargo clippy` on the pinned toolchain (li… | open | — |
| PIA-026 | P2 | 5 | NEW | edit | ADR-0083 | open | — |
| PIA-027 | P2 | 5 | SHA | decision | Omitting `package.rust-version` leaves the edition-2024 MSRV-aware resolver inert | open | — |
| PIA-028 | P2 | 5 | NEW | decision | Four 0.x crates sit on caret ranges in the §9.2 table while ADR-0084 says 0.x dependenc… | open | — |
| PIA-029 | P2 | 5 | SHA | decision | Register self-description fails verification | open | — |
| PIA-030 | P2 | 5 | NEW | decision | Three RUNBOOK commands are wrong as written | open | — |
| PIA-031 | P2 | 5 | NEW | edit | Step 5 owns the lint/CI policy yet leaves AC-001-004 with no clippy.toml, a trybuild fi… | open | — |
| PIA-032 | P2 | 5 | NEW | edit | Review/verify registers name enum values they never define | open | — |
| PIA-033 | P2 | 5 | NEW | decision | rk-store and redkiln-migrate contracts | open | — |
| PIA-034 | P2 | 5 | SHA | edit | Checkpoint 5's snapshot guard fails for the ADR-0083 test layout, and no RUNBOOK line a… | open | — |
| PIA-035 | P2 | 5 | SHA | decision | Checkpoints 3c, 5, 6 and 7b/8 carry lines green on the untouched tree or on a violation | open | — |
| PIA-036 | P2 | 5 | SHA | decision | The design diverges from the binding register it defers to | open | — |
| PIA-037 | P2 | 5 | SHA | decision | Fully superseded ADR-0004 is the governing decision on 57 criteria in seven sections | open | — |
| PIA-038 | P2 | 5 | NEW | decision | About 95 criteria are assigned to eight crate labels that exist in neither design secti… | open | — |
| PIA-039 | P2 | 5 | NEW | decision | No precedence rule ranks the design, AC register, SM register and RUNBOOK | open | — |
| PIA-040 | P2 | 6 | SHA | decision | rk-git's and rk-store's Step-6 contracts are lists, not rules | open | — |
| PIA-041 | P2 | 6 | SHA | decision | reconcile.rs | open | — |
| PIA-042 | P2 | 6 | SHA | edit | ADR-0077's gitoxide citations point at sources that say the opposite | open | — |
| PIA-043 | P2 | 6 | SHA | decision | Two Step-6 doctor rules rest on undefined terms | open | — |
| PIA-044 | P2 | 6 | SHA | edit | Step-3 drift contract | open | — |
| PIA-045 | P2 | 7a | NEW | decision | First-party Rust guidance does address the fake-vs-real dispatch question and ranks dyn… | open | — |
| PIA-046 | P2 | 7b | NEW | decision | Selection-area terms never defined | open | — |
| PIA-047 | P2 | 7b | NEW | edit | No step builds or checkpoints the three gates (AC-240–AC-359), filed under 7c | open | — |
| PIA-048 | P2 | 7b | NEW | decision | Step 7b lint/loader | open | — |
| PIA-049 | P2 | 7b | NEW | decision | Verification gate underspecified | open | — |
| PIA-050 | P2 | 7b | NEW | decision | Gate semantics | open | — |
| PIA-051 | P2 | 7b | SHA | edit | One behaviour, two criteria | open | — |
| PIA-052 | P2 | 7b | NEW | decision | `null` in `require` predicates | open | — |
| PIA-053 | P2 | 7b | SHA | edit | Predicate grammar for the 7b parser has no precedence/associativity, no mixed-unparenth… | open | — |
| PIA-054 | P2 | 7b | NEW | decision | Design §4 misstates ADR-0056/0058 | open | — |
| PIA-055 | P2 | 7b | SHA | edit | Backoff base 2 s / cap 5 min has two homes | open | — |
| PIA-056 | P2 | 7b | SHA | decision | An equivalent mutant survives re-execution, so ADR-0074's `mutation-survived` class can… | open | — |
| PIA-057 | P2 | 7b | SHA | edit | AC-388's per-sibling stagger chain is not the vendor's mechanism (hold until first resp… | open | — |
| PIA-058 | P2 | 7c | SHA | decision | The prompt-and-cache ADR pair (0016/0078) disagrees with its own evidence and its instr… | open | — |
| PIA-059 | P2 | 7c | NEW | edit | Five Phase-1 criteria's instruments pass on the behaviour they forbid or read inputs no… | open | — |
| PIA-060 | P2 | 7c | NEW | edit | Claude adapter contract | open | — |
| PIA-061 | P2 | 7c | NEW | edit | Headroom contract | open | — |
| PIA-062 | P2 | 7c | NEW | edit | RUNBOOK 7c lifts event.rs and throttle.rs 'unchanged' though they carry `Default`/`unwr… | open | — |
| PIA-063 | P2 | 7c | NEW | edit | AC-311 offers a 'quarantine' ADR-0082 makes an error | open | — |
| PIA-064 | P2 | 7c | SHA | edit | Claude Code `defer` (`tool_deferred`) shipped in 2.1.89 on 2026-03-31 and is documented… | open | — |
| PIA-065 | P2 | 7c | SHA | decision | Recorded evidence disowned by the registers | open | — |
| PIA-066 | P2 | 7c | NEW | decision | ADR-0016/design 7.1 say S-WT measured a `--system-prompt` 'carrying the card body' | open | — |
| PIA-067 | P2 | 7c | SHA | decision | 'F3 measured the ephemeral_1h/5m fields absent' is false | open | — |
| PIA-068 | P2 | 7c | SHA | decision | `system/api_retry` payload the corpus says was never captured is captured on 139 rows i… | open | — |
| PIA-069 | P2 | 7c | SHA | edit | `rate_limit_event` is now documented in both Agent SDK references with a shape F1 did n… | open | — |
| PIA-070 | P2 | 7d/8 | SHA | decision | Summary rows, acceptance-set ranges, cross-reference ids and restated figures overstate… | open | — |
| PIA-071 | P2 | 7d/8 | SHA | decision | Steps 7a/7c acceptance sets sweep in out-of-phase, CONDITIONAL, retired, S-BIN-blocked … | open | — |
| PIA-072 | P2 | 7d/8 | SHA | decision | Phase-1 ports scope drawn three ways | open | — |
| PIA-073 | P2 | record | SHA | decision | Statistics tagged [measured] or settled do not reproduce under the registered two-sided… | open | — |
| PIA-074 | P2 | record | SHA | edit | ADR-0086's crate facts were wrong on its own 2026-09-08 read date (temporalio-sdk 1.0.0… | open | — |
| PIA-075 | P2 | record | SHA | decision | Corrections and restorations recorded as applied left copies uncorrected | open | — |
| PIA-076 | P2 | record | SHA | edit | Register baselines carry the wrong condition label or qualifier | open | — |
| PIA-077 | P2 | record | SHA | decision | Spike-draft headline figures do not reproduce from their result files | open | — |
| PIA-078 | P2 | record | SHA | decision | Corpus-pin figures restated as [measured] do not reproduce | open | — |
| PIA-079 | P2 | record | SHA | edit | 'No first-party source' for the thiserror/anyhow split is false in six live places | open | — |
| PIA-080 | P2 | record | SHA | decision | F38's 0.45x/0.55x/0.78x divides P8 output tokens by F28's cache-creation tokens | open | — |
| PIA-081 | P2 | record | SHA | edit | E3 severity adjudication closed 2026-09-09 NOT BLIND | open | — |
| PIA-082 | P2 | record | SHA | decision | Five DECIDED Phase-1 rules cite ADRs whose bodies do not contain them and no ADR does | open | — |
| PIA-083 | P2 | record | SHA | edit | ADR-0084's advisory evidence misattributes three gix advisories, 'no advisory affects b… | open | — |
| PIA-084 | P2 | record | SHA | decision | Four registered-unrun spikes cannot decide their own criterion | open | — |
| PIA-085 | P2 | record | SHA | edit | Design section 6.6's durable-systems survey carries wrong cells | open | — |
| PIA-086 | P2 | record | SHA | edit | ADR-0085 rules 4 and 6 claim 'no external norm at all' | open | — |
| PIA-087 | P2 | record | SHA | edit | `Quarantined` 'no prior art beyond GitLab and Cloudflare' (ADR-0074/0082) is contradict… | open | — |
| PIA-088 | P2 | record | SHA | decision | §10.5/open-item rows vs ADR-0027/0012 | open | — |
| PIA-089 | P2 | record | NEW | decision | Pre-data claims fail against git | open | — |
| PIA-090 | P2 | record | SHA | decision | The narrowed novelty claim 'in an AI-coding-specific backlog or spec tool' is false | open | — |
| PIA-091 | P2 | record | NEW | edit | The AWS backoff post does not 'nudge toward Equal Jitter for throttling' | open | — |
| PIA-092 | P2 | record | SHA | edit | ADR-0081's 'no source frames activity selection as a research question at all' contradi… | open | — |
| PIA-093 | P2 | record | NEW | edit | ADR-0089's 'Semgrep 0.57%-23.8% recall' floor is from a mis-cited arXiv ID (RealVuln | open | — |
| PIA-094 | P2 | record | SHA | decision | Spike-register gate verdicts drift from the governing table | open | — |
| PIA-095 | P2 | record | NEW | decision | Review-gate ADRs mis-state F24/F31 | open | — |
| PIA-096 | P2 | record | SHA | decision | F39's panel-adjudicated auth 0.00 (load-bearing for ADR-0089) rests on an uncommitted a… | open | — |
| PIA-097 | P2 | record | SHA | decision | F31's block counts, sink seeds and typed-arm scoring came from the model-emitted eviden… | open | — |
| PIA-098 | P2 | record | NEW | edit | The rank-reversal sentence behind 'rigor is ordinal, never summed' is pinned to a TOPSI… | open | — |
| PIA-099 | P2 | record | SHA | edit | Principle 8 was changed on a vendor compaction quote ('nuanced understanding ...') that… | open | — |
| PIA-100 | P3 | 5 | SHA | decision | Names the governing record cannot lex or does not forbid | open | — |
| PIA-101 | P3 | record | NEW | edit | Research-folder citation hygiene | open | — |
| PIA-102 | P3 | record | SHA | decision | Six relations the ADR README and section 10.5 leave undeclared, so a supersession check… | open | — |
| PIA-103 | P3 | record | SHA | edit | Retired vocabulary survives | open | — |
| PIA-104 | P3 | record | SHA | edit | Record hygiene against the corpus's own rules | open | — |
| PIA-105 | P3 | record | NEW | decision | Three landscape citations in evidence-augmentation are mis-scoped or paraphrased as ver… | open | — |
| PIA-106 | P3 | record | NEW | edit | Two register-filing contradictions | open | — |
| PIA-107 | P3 | record | SHA | edit | Statistical restatement hygiene | open | — |
| PIA-108 | P3 | record | NEW | decision | PRIOR-ART research-lineage hygiene | open | — |
| PIA-109 | P3 | record | SHA | edit | S-H1b-24H's cost cell omits that the 2026-09-06 mark is spent and METHOD-S's correction… | open | — |
| PIA-110 | P3 | record | NEW | edit | Design section 0.1 row 24 names the S-P7B re-run in prose, the only one of ten BLOCKED … | open | — |
## 5. §8 follow-ups (what the audit did not run)

| Limit | Task | Cost | Status |
| --- | --- | --- | --- |
| M-01 | S-PORTS — a throwaway crate compiling the five port traits, one fake, `CancelToken`, a `compile_fail` doctest (consumer: Step 7a, ADR-0099) | 1–2 h | this pass |
| M-02 | S-GRAMMAR — EBNF + recursive-descent parser over every predicate in `spikes/p9` (consumer: Step 7b lint, ADR-0103) | 2–4 h | this pass |
| C-09 | `cargo audit` on `spikes/runner/Cargo.lock`; `spikes/p9/lint.py` over the packs and catalog | minutes | this pass |
| C-06 | DECISIONS-2026-09-09 + HANDOFF diffed against the ADR set | 1 h | this pass |
| M-06 | Operator label file over the 17 P1 + a seeded 30 P2 concerns (agree / disagree / unsure) | 2–3 human hours | templated: `references/audit/2026-09-10/labels-ryan-TODO.md` |
| C-01 | Slice finder + refuter over AC §12 (CLI/doctor, AC-400..439) | 1 agent-pass | before Step 7d (noted in s7d) |
| C-02 | Slice finder + refuter over AC §3.1–3.6 (selection engine steps 1, 3–7) | 1 agent-pass | before Step 7b (noted in s7b) |
| M-05/M-14 | A second blind refuter over the 24 P1 + 72 weakened P2 records | 3–4 agent-hours | not run |
| M-08 | Known-issue proximity script for the 41 NEW concerns | ~2 h | not run |
| M-03 | Wilson recompute over every interval near a k/n | 1 h | not run |
| M-04 | 481-id design/register GWT similarity | 2 h | not run |
| M-07 | Re-run the unrun scorers (P7, P1 arm-L, audit-e3, audit-s, lint/lattice) | 1–2 h | not run |
| M-09 | Full-text grep of the load-bearing preprints | 2–3 h | not run |
| M-10 | A refuter over the 11 impactful best-practice rows | 1 h | not run |
| C-03 | A seeded 10-criteria decidability sample per under-named section (§§4, 7, 8, 11, 13) | <1 h | not run |
| C-07 | Read the 30 unread ADRs against their citation sites | ~128 KB | not run |
| C-08/M-13 | The doc-map stale-risk (12) and strict-orphan (27) lens; 69 prose anomalies vs the pool | ½ day | not run |
| C-10 | Re-cite or drop EXT-CLAUDECO-042 / EXT-PROCESSL-154; HANDOFF.md:303 marked a pointer | minutes | this pass |
| C-13 | Re-run the nine census scripts after the Step 3c-closing commit and diff the counts | 5 min | at Step 3c |

## 6. Validation

| Check | Baseline (untouched tree, 2026-09-11) | After remediation |
| --- | --- | --- |
| `census_ids.py` — failed anchor assertions | `[]` | |
| `census_adr.py` — anchor failures | 0 | |
| `census_anchors.py` — C-refs resolved / past end · file paths resolved | 207 / 0 · 1690 | |
| `census_constants.py` — content sha256 | `85ac6849…6950de` | |
| ADR files (`ls references/adr/*.md \| wc -l`, README included) | 90 | |
| Stale strings (the `git grep` list in INDEX §Validation) | present | |
| Checkpoint dry-run (targets that exist today) | — | |
| S-PORTS `cargo check` | — | |
| S-GRAMMAR parse-fail / ambiguous count with denominator | — | |
| `cargo audit` on `spikes/runner/Cargo.lock` | — | |
