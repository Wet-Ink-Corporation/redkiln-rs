# Step 7b — `rk-kernel` (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7b` | Phase 1 | 🛑 STOP banner (ADR-0090 exists; 0079 superseded) | Checkpoint 7b | `s7a` | `s7c` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


> # 🛑 STOP. THIS IS THE STEP STEP 4 EXISTS TO GATE.
>
> **`rk-kernel` is the crate that contains the selection engine.** Everything before this point could be built
> with the gate wide open and no harm done. This step cannot. Run all three lines and read all three answers:
>
> ```bash
> cd D:/repos/redkiln-rs
> ls references/adr/0090-*.md >/dev/null 2>&1 \
>   && echo "OK: ADR-0090 exists — the context-vector decision was TAKEN" \
>   || echo "STOP: no ADR-0090. Go to Step 4. Do not write selection code."
> grep -c '^status: superseded$' references/adr/0079-*.md    # → 1. ADR-0079 is discharged, not amended.
> ls references/adr/0089-*.md >/dev/null 2>&1 \
>   && echo "note: ADR-0089 is the FACET-PROVENANCE record. It is NOT this gate's token."
> ```
>
> **`0089` is not the token, and mistaking it for one is the specific failure this third line exists to stop.**
> ADR-0089 is accepted and it is about facet *provenance* — it partially supersedes ADR-0030, and it explicitly
> does not touch ADR-0079 [documented, ADR-0089]. Seeing it on disk tells you nothing about whether the
> context-vector decision was taken.
>
> **A discharged gate is not an unblocked one.** ADR-0090 records *human-sourced ordinal axes* **and human-sourced
> booleans** for Phase 1 — clause A fails 4/4 on the records that exist, and DECISIONS-2026-09-11 #2 pre-registered
> `has_repro`, `external_dep` and `market_facing` to the same destination. So the thing you may now build is the
> engine's **semantics**; the thing you still may not build is any `require` row keyed on an LLM-emitted
> `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro` — and after Step
> 4a **no facet is `source = "llm"`**, so the grep below has no true instance. `kind` is out of scope of the block
> (human input, ADR-0030 rule 4). That is golden constraint 1, ADR-0079, AC-071 and design §10.2's "may not"
> column, all saying the same sentence.
>
> **F39 moved exactly one of AC-071's five clauses.** Arm D was scored on 2026-09-09, discharging the fourth —
> "arm-D panel `scored`" — and only that one [measured, F39]. The block holds on the **first**: modal required-set
> identity measured **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line [measured, F36]. **A discharged clause is
> not a partial unblocking, and the five-case fixture set stays at five.**
>
> The enforcement is `redkiln lint --facet-stability`, specified below — and its sibling **`--facet-provenance`
> (AC-520)**, which gates `source = "derived"` and which nothing gated before F39. **Build both lints before the
> rows they police**, so there is never a window in which a `require` row exists and nothing checks it.

Workflow/activity split, policy, memo, gates, budgets, retry, replay. **`tokio`, `async-std`, `futures` and
`async-trait` are forbidden here** — no dependency edge, direct or transitive, on any `rk-adapter-*` (AC-484), and
no generic parameter bound by a port trait (AC-480).

**The grammar the lint parses is ADR-0103's, and it has been run over every live predicate (F42, S-GRAMMAR,
2026-09-11):** `not` > comparison/`in` > `and` > `or`, left-associative; an unquoted identifier in value position is a
literal (load-bearing on 191 of 287 live sites); `null == v` is `null` and connectives are three-valued; `any` over
`null` or an empty list is `false`, `all`/`none` `true`; `on_false` is the closed set `{ to, max } | "continue" |
"fail" | { to = "deviate", … }`. Two forms the packs already use and no record spelled out — the set-membership `in`
(`'text' in change`, 11 sites) and hyphens inside `${…}` ref segments — are in design §4.3's grammar and in
`spikes/p9/grammar-probe.py`'s EBNF; start the parser from that probe. Under it 285/287 live predicates parse and
**0** depend on precedence; the two that fail are pack defects the lint must catch (`exists(ProblemRecord)` in
`packs/incident.toml`; a bare hyphenated value in `catalog-b`) [measured, F42].

Five things this crate must get right, each because a measurement says so:

- **The FSM has never been replayed.** F19 proved suspension lossless 174/174 across a real process boundary but
  explicitly left "same next action" unproven. The kernel replay test is the *first* time it is replayed at all.
  That test is Phase-1 exit criterion (2), not a nice-to-have.
- **Determinism ban list fails the build** — `cargo clippy -D warnings` plus a `trybuild` must-fail fixture that
  reintroduces a clock read (AC-001–AC-004).
- **The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion.
- **Selection semantics only.** The seven-step pure function, rank, tie-is-an-error, unsatisfiable-is-a-typed-gate,
  deviation-never-edits-`R`, and the lint set all get built (AC-040–AC-047, AC-054–AC-056, AC-061–AC-064). What
  does not: any `require` row keyed on an LLM-emitted ordinal. `redkiln lint --facet-stability` (AC-071) must
  **fail closed** on `source = "llm"` without a scored stability record, and it must check **all five clauses** —
  identity, n, draws, `panel_verdict`, `corpus_sha`. A lint that reads identity and skips `panel_verdict` unblocks
  the entire provisional apparatus silently; the five-clause fixture set is the only defence.
- **Its sibling gate, and the one that did not exist before F39.** `redkiln lint --facet-provenance` (**AC-520**,
  Step 4a.4) must **fail closed** on `source = "derived"` without a named `deriver` and a scored precision record
  over a pinned corpus. **AC-071 gates `llm` and nothing gated `derived`** — so the semantic facets sat outside the
  block and ran **ungated while measurably wrong**, at `auth` precision 0.00 over 11 positives [measured, F39].
  Build it as **two lint rules with two fixture sets**, never as a sixth clause on AC-071: one gate carrying two
  subjects, two records and two failure meanings is the shape that invites the collapse AC-071's own residual-risk
  sentence names. Four must-fail fixtures **plus one that must load**, or the rule is satisfied by a lint that
  rejects everything.

Also here: the **rigor-monotonicity lint**, failing closed (AC-104, SM-035). The ladder was *not* monotone in
internal quality — escalating to `mandatory` with `has_repro` selected `bisect`, which has **no adversary and no
policy step**, so escalating rigor *dropped* the quality block (F37). **P7-CEIL is decided: `bisect` is a ceiling
activity** — `spikes/p9/catalog/bisect.toml` carries `max_rigor = "recommended"` since 2026-09-11 (ADR-0093 decision
5; DECISIONS-2026-09-11 #1), `lattice.py --validate` shows the localisation family total with 0 holes and the P9 lint
0 errors [measured, 2026-09-11]. The lint ships in Phase 1 and **passes on the catalog as shipped**; Checkpoint 7b
runs it. Adding the missing steps to `bisect` is a Phase-2 shipped-activity edit under ADR-0072.

**And the gates (sub-step 7b-ii; PIA-047).** Nothing else builds them: the human-gate record `(token, tree_hash,
deadline)` with the write-tree derivation of ADR-0105 clause 4 and the fresh-record-on-re-reach rule (AC-524); the
review classifier over ADR-0096's typed repro (AC-284–AC-286, PROVISIONAL until S-CLASS-REPLAY), the policy table
(AC-294–AC-298), re-execution (AC-287–AC-290), quarantine (AC-304–AC-307; review-gate only, ADR-0104), the sink
(AC-310) and replay (AC-313); the verify harness over author-declared mutants (AC-317 as the invocation criterion,
AC-334 as the flip predicate — ADR-0104) with the bootstrap comparator (AC-326/AC-329) and the ledger (AC-333).
Build them after the selection engine and before 7c; their checkpoint lines are below.

And what may **not** be set: **no quality-shorthand default may be derived from P7.** Its primary criterion is
powerless at n=3 — every Wilson interval overlaps by construction — and the forbidden predicate
`findings.none(evidence in [Reproduced, MutationSurvived])` may not be used as a terminal condition. Each quality
block is authored explicitly, per activity.

**Checkpoint 7b.** All eighteen (nine on the kernel and lints; three the Step 4a deferrals re-run here; six on 7b-ii):

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-kernel && cargo test -p rk-kernel                   # exit 0
cargo test -p rk-kernel -- trybuild ; echo "exit=$?"                  # → exit=0: the must-fail fixtures
                                                                      #   FAIL TO COMPILE, which is the pass
grep -rnE 'tokio|async-trait|async-std|futures' crates/rk-kernel/Cargo.toml | wc -l   # → 0
cargo metadata --format-version 1 --no-deps >/dev/null && \
  cargo tree -p rk-kernel -e normal 2>/dev/null | grep -c 'rk-adapter-'  # → 0 (AC-484)
cargo test -p rk-kernel -- facet_stability_five_clause ; echo "exit=$?" # → exit=0: five clauses, four negatives
cargo test -p rk-kernel -- facet_provenance_fixtures   ; echo "exit=$?" # → exit=0: AC-520, four must-fail
                                                                       #   fixtures AND one that must LOAD
grep -rn 'facet_stability\|facet_provenance' crates/rk-kernel/src/ | grep -c 'fn '   # → ≥ 2: TWO rules, not one
grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' \
     crates/ --include=*.toml --include=*.rs | wc -l                   # → 0. Golden constraint 1, as a command.
grep -rniE 'leaves the model' crates/ --include=*.rs --include=*.toml --include=*.md | wc -l
                                                                       # → 0. NO FACET DOES (s4 §4.1 clause B, F39).
cargo test -p rk-kernel -- rigor_monotonicity ; echo "exit=$?"         # → exit=0 on the shipped catalog (P7-CEIL: bisect is ceilinged)
python spikes/p9/grammar-probe.py | grep -E '^predicates parsed'       # → the parser's fixture corpus; the kernel's parser must agree with the probe on every string that parses
# --- Checkpoint 4a's [WILL EXIST] lines, re-run here as named lines (PIA-003): the lint now exists, so the
#     expectation is "exit != 0 AND stderr names the failing clause" — a missing binary passes neither half
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml 2>err.txt ; test $? -ne 0 && grep -qE 'deriver|record|positive' err.txt && echo "AC-520 fails closed, naming the clause"
./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml 2>err.txt ; echo "exit=$?"   # → exit=0 on the ZERO-llm branch (Step 4a): the expected outcome, not a shortfall
cargo test -p rk-kernel -- golden_log_replay ; echo "exit=$?"          # → exit=0: the kernel records golden event logs on the pinned corpus and replays them (AC-006/AC-188; the logs are committed to redkiln-corpus as fixtures — PIA-002)
# --- 7b-ii, the gates
cargo test -p rk-kernel -- ac334_flip_predicate ; echo "exit=$?"        # → exit=0: the flip predicate on a declared mutant
cargo test -p rk-kernel -- ac015_inversion_regression ; echo "exit=$?"  # → exit=0 and the case yields mutation-survived, never pinned (F25)
cargo test -p rk-kernel -- ac313_replay_k10 ; echo "exit=$?"            # → exit=0: ten replays, one digest
cargo test -p rk-kernel -- ac310_sink_accounting ; echo "exit=$?"       # → exit=0: findings_in == blocks + sink_records
cargo test -p rk-kernel -- trybuild -- ac294_block_constructor ; echo "exit=$?"   # → exit=0: the must-fail compiles nothing
cargo test -p rk-kernel -- ac524_fresh_record_on_rereach ; echo "exit=$?"         # → exit=0 (PROVISIONAL until P8-B)
```

The LLM-ordinal line is the standing prohibition made mechanical. It is coarse and it will occasionally flag a
comment; that is the right direction for this particular guard, and golden constraint 9 says to establish its scope
rather than route around it — read each hit, do not silence the grep. The `leaves the model` line is the same shape
for the F39 result: **no facet clears METHOD-P §1.4's 0.95 bar and the bar is unreachable at n=20 for any facet**
[measured, F39 Limits 1], so the phrase has no true instance and any hit is a claim nothing measured supports.

**Acceptance set (explicit, per PIA-048/PIA-070):** **AC-001–AC-039** (kernel; AC-006/AC-188's golden-log replay
included — the logs are produced here); **AC-040–AC-079** (selection) **minus AC-075** (cut, never reuse) — and
**building AC-069, AC-070, AC-072 and AC-079** as binding (the registry after Step 4a has zero `llm` facets, six
`derived`, four `kernel`, one `declared`), with **AC-076** human-gated and **AC-068** PROVISIONAL (P2, mechanism
only; its pass condition is the decision rule of DECISIONS-2026-09-11 #4); **AC-080–AC-118** (packs and catalog lint;
**AC-119** is unallocated); **AC-520** — the derived-facet provenance gate, allocated outside the selection range
because `AC-040..AC-079` is exhausted; and the gates' criteria **AC-240–AC-359** minus the named exceptions —
AC-282 stays 7c's; AC-291/AC-292 are Step 6's; AC-263 is 7d's; AC-243/AC-267/AC-269/AC-279 are cross-listed in 7c;
AC-255 and AC-297 are cut (never reuse); AC-265 is cited at Step 8 (3); AC-312 is PROVISIONAL on S-SUBTLE-CB
(DECISIONS-2026-09-11 #17); AC-284 PROVISIONAL until S-CLASS-REPLAY. The register cites the design's §3 state line for
the P1-blocked rows, not design §1.2's "affected" column.

**Before this step, not after (audit §8 C-02):** the selection engine's register §3.1–3.6 criteria (AC 608–1008 in the
register; purity/replay, rank/ties, the unsat gate, typed deviation) were named by no finder in the 2026-09-10 audit
— run one slice finder + refuter over them with the two-implementer lens before writing the seven-step function, and
record the result in `PROGRESS.md`.
