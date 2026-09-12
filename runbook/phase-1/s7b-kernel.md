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
> with the gate wide open and no harm done. This step cannot. Run the block; the first two checks stop you,
> the third is a note:
>
> ```bash
> cd D:/repos/redkiln-rs
> ( ls references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "STOP: no ADR-0090. Go to Step 4. Do not write selection code."; exit 1; }
>   echo "OK: ADR-0090 exists — the context-vector decision was TAKEN"
>   n=$(grep -c '^status: superseded$' references/adr/0079-*.md)
>   test "${n:-0}" -eq 1 \
>     || { echo "STOP: ADR-0079 is not 'status: superseded' (got ${n:-<unreadable>}) — the gate token is incomplete"; exit 1; }
>   echo "OK: ADR-0079 is discharged, not amended"
>   ls references/adr/0089-*.md >/dev/null 2>&1 \
>     && echo "note: ADR-0089 is the FACET-PROVENANCE record. It is NOT this gate's token." )
> ```
> The block is one subshell, so a pasted STOP ends the gate check and not your interactive shell; its
> status is `$?`. The first two lines **assert and exit non-zero** — until 2026-09-12 all three only
> echoed, so the gate that exists to stop selection work could not stop anything (Halt 8 shape (f)).
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
- **Determinism ban list fails the build** — the `disallowed-methods` / `disallowed-types` tables in the root
  `clippy.toml` (s5 §5.1 authors them), applied to `rk-kernel::workflow` under a module-level
  `#[deny(clippy::disallowed_methods)]`, with CI running `cargo clippy --workspace --all-targets` **gated on its
  exit code**, plus the must-fail **fixture crate** that reintroduces one banned call and is checked under the
  same invocation (AC-001–AC-004). **Not `-D warnings`, and not a `trybuild` fixture** — AC-001's Verified-by was
  amended on 2026-09-11 (PIA-030/PIA-031) to withdraw both: `-D warnings` promotes `clippy::nursery` to the group
  deny ADR-0083 forbids, and *"a `trybuild` fixture cannot fail on a lint, only on a compile error, so it is not
  the instrument here"*. This step carried the withdrawn pair until 2026-09-12, so the kernel-determinism gate as
  written could not fire.
- **The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion. This
  one **is** type-level (the constructor must not exist), which is the distinction: `trybuild` catches what fails
  to *compile*; a reintroduced clock read compiles fine and is caught by the lint. AC-294's block constructor is
  the other type-level ban and uses `trybuild` for the same reason (7b-ii below).
- **Selection semantics only.** The seven-step pure function, rank, tie-is-an-error, unsatisfiable-is-a-typed-gate,
  deviation-never-edits-`R`, and the lint set all get built (AC-040–AC-047, AC-054–AC-056, AC-061–AC-064). What
  does not: any `require` row keyed on an LLM-emitted ordinal. `redkiln lint --facet-stability` (AC-071) must
  **fail closed** on `source = "llm"` without a scored stability record, and it must check **all five clauses** —
  identity, n, draws, `panel_verdict`, `corpus_sha`. A lint that reads identity and skips `panel_verdict` unblocks
  the entire provisional apparatus silently; the five-clause fixture set is the only defence.
- **Its sibling gate, and the one that did not exist before F39.** `redkiln lint --facet-provenance` (**AC-520**,
  Step 4a.4) must **fail closed** on `source = "derived"` without a named `deriver` and a scored precision record
  over a pinned corpus. **AC-071 gates `llm` and nothing gated `derived`** — so the semantic facets sat outside the
  block and ran **ungated while measurably wrong**, at `auth` precision 0.00 (0/11 [0.00, 0.26] on the 2026-09-09 scoring; 0/12 [0.00, 0.24] after the 2026-09-11 panel-file key fix scored all 140 cells — F39-DRAFT's Correction; ERRATA E-0089-2) [measured, F39].
  Build it as **two lint rules with two fixture sets**, never as a sixth clause on AC-071: one gate carrying two
  subjects, two records and two failure meanings is the shape that invites the collapse AC-071's own residual-risk
  sentence names. Four must-fail fixtures **plus one that must load**, or the rule is satisfied by a lint that
  rejects everything.

Also here: the **rigor-monotonicity lint**, failing closed (**AC-105**, SM-035 — corrected 2026-09-12 from
AC-104, which is the family-*totality* criterion; SM-035's Instrument line names AC-105 by id, and AC-104's own
totality check is the separate lattice CI check per (family, rigor)). The ladder was *not* monotone in
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

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- `facet_stability_five_clause` — **four must-fail fixtures**, one per clause the lint could silently skip
  (identity, n, draws, `panel_verdict`, `corpus_sha`). Red each one separately: a lint that reads identity and
  skips `panel_verdict` passes a single happy-path test and unblocks the whole provisional apparatus. The four
  negatives are the discrimination; one positive test is not.
- `facet_provenance_fixtures` — **four must-fail plus one that must LOAD**. The must-load case is not decoration:
  without it the criterion is satisfied by a lint that rejects everything.
- `rigor_monotonicity` — red on the pre-P7-CEIL catalog, where escalating to `mandatory` with `has_repro` selected
  `bisect` and *dropped* the quality block (F37). Green on the catalog as shipped, with `bisect` ceilinged.
- `golden_log_replay` — the FSM has **never been replayed** (F19 left "same next action" unproven). There is no
  prior green to regress from, so the red step here is the first honest measurement, not a formality.
- `ac334_flip_predicate` — the declared mutant **must flip** the test (ADR-0104). A mutant that survives blocks,
  quarantined or not; `ac015_inversion_regression` is the paired case that must yield `mutation-survived` and
  never `pinned` (F25).

**Checkpoint 7b.** All twenty (eleven on the kernel and lints; three in the Step 4a group re-run here; six on 7b-ii)
— recounted on disk 2026-09-11; this read "eighteen … nine" and the block carries twenty:

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-kernel || { echo "FAIL: cargo build -p rk-kernel"; exit 1; }   # golden constraint 3: build before you test
out=$(cargo test -p rk-kernel 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p rk-kernel exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: rk-kernel compiled but ran 0 tests"; exit 1; }   # exit 0
out=$(cargo test -p rk-kernel -- trybuild 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: trybuild exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched trybuild"; exit 1; }               # → the must-fail fixtures
                                                                      #   FAIL TO COMPILE, which is the pass
test -f crates/rk-kernel/Cargo.toml || { echo "FAIL: crates/rk-kernel/Cargo.toml absent"; exit 1; }
hits=$(grep -rnE 'tokio|async-trait|async-std|futures' crates/rk-kernel/Cargo.toml); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: rk-kernel's manifest names an async runtime, or could not be read (grep rc=$rc; 2 = no such file)"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1 = searched, matched nothing
cargo metadata --format-version 1 --no-deps >/dev/null || { echo "FAIL: workspace metadata"; exit 1; }
deps=$(cargo tree -p rk-kernel -e normal) || { echo "FAIL: cargo tree -p rk-kernel did not run"; exit 1; }
n=$(printf '%s\n' "$deps" | grep -c 'rk-adapter-'); test "${n:-1}" -eq 0 || { echo "FAIL: rk-kernel depends on ${n} rk-adapter-* crate(s) — AC-484 forbids the edge, direct or transitive"; exit 1; }   # → silent: 0 (AC-484)
out=$(cargo test -p rk-kernel -- facet_stability_five_clause 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: facet_stability_five_clause exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched facet_stability_five_clause"; exit 1; }   # → five clauses, four negatives (AC-071)
out=$(cargo test -p rk-kernel -- facet_provenance_fixtures 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: facet_provenance_fixtures exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched facet_provenance_fixtures"; exit 1; }    # → AC-520, four must-fail
                                                                       #   fixtures AND one that must LOAD
test -d crates/rk-kernel/src || { echo "FAIL: crates/rk-kernel/src absent"; exit 1; }
n=$(grep -rn 'facet_stability\|facet_provenance' crates/rk-kernel/src/ | grep -c 'fn '); test "${n:-0}" -ge 2 || { echo "FAIL: ${n:-0} lint fn(s) found — AC-071 and AC-520 are TWO rules with two fixture sets, never one gate with two subjects"; exit 1; }   # → silent: ≥ 2, TWO rules, not one
test -d crates/rk-kernel/src || { echo "FAIL: crates/ has no rk-kernel — Checkpoint 7b is not runnable"; exit 1; }
hits=$(grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' \
       crates/ --include=*.toml --include=*.rs); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: an LLM-ordinal require row exists, or the search could not run (grep rc=$rc) — golden constraint 1, as a command"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1
test -d crates/rk-kernel/src || { echo "FAIL: crates/ has no rk-kernel — Checkpoint 7b is not runnable"; exit 1; }
hits=$(grep -rniE 'leaves the model' crates/ --include=*.rs --include=*.toml --include=*.md); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: 'leaves the model' appears in crates/, or the search could not run (grep rc=$rc) — NO FACET DOES (s4 §4.1 clause B, F39)"; printf '%s\n' "$hits"; exit 1; }   # → silent: rc exactly 1
out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rigor_monotonicity exited $rc on the shipped catalog"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched rigor_monotonicity"; exit 1; }      # → passes on the shipped catalog (P7-CEIL: bisect is ceilinged)
test -f spikes/p9/grammar-probe.py || { echo "FAIL: spikes/p9/grammar-probe.py absent"; exit 1; }
out=$(python spikes/p9/grammar-probe.py); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: grammar-probe did not run (exit $rc)"; exit 1; }   # 0 = clean, 1 = the known pack defects (F42)
printf '%s\n' "$out" | grep -E '^predicates parsed [1-9][0-9]*/[0-9]+' \
  || { echo "FAIL: probe printed no parsed count — it crashed, or the corpus is empty"; exit 1; }   # → the parser's fixture corpus; the kernel's parser must agree with the probe on every string that parses
# --- Checkpoint 4a's [WILL EXIST] lines, re-run here as named lines (PIA-003): the lint now exists, so the
#     expectation is "exit != 0 AND stderr names the failing clause" — a missing binary passes neither half
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml 2>err.txt ; test $? -ne 0 && grep -qE 'deriver|record|positive' err.txt && echo "AC-520 fails closed, naming the clause"
./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml 2>err.txt ; echo "exit=$?"   # → exit=0 on the ZERO-llm branch (Step 4a): the expected outcome, not a shortfall
out=$(cargo test -p rk-kernel -- golden_log_replay 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: golden_log_replay exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched golden_log_replay"; exit 1; }       # → the kernel records golden event logs on the pinned corpus and replays them (AC-006/AC-188; the logs are committed to redkiln-corpus as fixtures — PIA-002)
# --- 7b-ii, the gates
out=$(cargo test -p rk-kernel -- ac334_flip_predicate 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ac334_flip_predicate exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched ac334_flip_predicate"; exit 1; }    # → the flip predicate on a declared mutant
out=$(cargo test -p rk-kernel -- ac015_inversion_regression 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ac015_inversion_regression exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched ac015_inversion_regression"; exit 1; }   # → and the case yields mutation-survived, never pinned (F25)
out=$(cargo test -p rk-kernel -- ac313_replay_k10 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ac313_replay_k10 exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched ac313_replay_k10"; exit 1; }        # → ten replays, one digest
out=$(cargo test -p rk-kernel -- ac310_sink_accounting 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ac310_sink_accounting exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched ac310_sink_accounting"; exit 1; }   # → findings_in == blocks + sink_records
out=$(cargo test -p rk-kernel -- trybuild -- ac294_block_constructor 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: trybuild/ac294_block_constructor exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched trybuild/ac294_block_constructor — check the doubled '--'"; exit 1; }   # → the must-fail compiles nothing
out=$(cargo test -p rk-kernel -- ac524_fresh_record_on_rereach 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: ac524_fresh_record_on_rereach exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched ac524_fresh_record_on_rereach"; exit 1; }   # → (PROVISIONAL until P8-B)
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
because `AC-040..AC-079` is exhausted; and, allocated the same way by the 2026-09-11 amendment (register §0.4) and
**claimed by no stage until 2026-09-12**: **AC-525** (the vouched-root advisory note — exactly one `drift_advisory`
per changed upstream digest, carried on the *next* gate the item reaches; ADR-0025/0024, and SM-022 has no
baseline, so report the noise rate, do not threshold it), **AC-527** (the `rk-catalog` lint that refuses
`findings.none(evidence in [Reproduced, MutationSurvived])` as a quality block's terminal condition — ADR-0080
decision 2; `none` over an empty list is **true** under ADR-0103's quantifier rule, which is precisely why the form
cannot terminate a block, and F29's reproduced regression was approved 3 of 10 times) and **AC-530** (a sink record
leaves only by a **recorded disposition**; ADR-0082, seeded by F31's seven `narrowed-pin` findings — note F31's
counts are model-label counts, so the seed is a seed and not a measurement); and **AC-455**, cross-listed from the
migration id range where it has no owner — `rk-kernel` **policy**: a spec carrying an AC with `verification: None`,
against a pack policy that requires one at this stage, makes the stage **refused with the AC id named, at zero
model cost** (S-MIG decision 1: the *policy*, not the type, decides whether a missing verification blocks); and the gates' criteria **AC-240–AC-359** minus the named exceptions —
AC-282 stays 7c's; AC-291/AC-292 are Step 6's; AC-263 is 7d's; AC-243/AC-267/AC-269/AC-279 are cross-listed in 7c;
AC-255 and AC-297 are cut (never reuse); AC-265 is cited at Step 8 (3); AC-312 is PROVISIONAL on S-SUBTLE-CB
(DECISIONS-2026-09-11 #17); AC-284 PROVISIONAL until S-CLASS-REPLAY. The register cites the design's §3 state line for
the P1-blocked rows, not design §1.2's "affected" column.

**Before this step, not after (audit §8 C-02):** the selection engine's register §3.1–3.6 criteria (AC 608–1008 in the
register; purity/replay, rank/ties, the unsat gate, typed deviation) were named by no finder in the 2026-09-10 audit
— run one slice finder + refuter over them with the two-implementer lens before writing the seven-step function, and
record the result in `PROGRESS.md`.
