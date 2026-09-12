# Runbook — Redkiln-RS from here to the end of Phase 1 · preamble

**Status:** operator-facing build runbook, split into one file per stage under `runbook/` on 2026-09-11 after the
pre-implementation audit (`references/2026-09-10-pre-implementation-audit.md`, 110 concerns) was absorbed. This
file is the shared preamble; `INDEX.md` is the map and `PROGRESS.md` the tracker. The runbook is the execution
order for `references/2026-09-08-architecture-design.md` §0.1 and §10.2 and **decides nothing the record does
not**: precedence is fixed by **ADR-0092** — accepted ADRs govern every decision; the AC register owns every
criterion's wording, instrument and state; the SM register owns targets, baselines and kills; the spike files govern
every restated number; and this runbook defers to the design for phase order and to the AC register for every
threshold and instrument. Where a stage file disagrees with any of those, the stage file is wrong. · **Scope:**
Phase 0 (close the programme) and Phase 1 (kernel, store, fix and story lanes, four adapters) only. Phases 2–4 are
out of scope. · **Inputs:** `references/2026-09-08-architecture-design.md` (§0.1, §5.3, §5.9, §9, §10) ·
`references/2026-09-08-acceptance-criteria.md` (AC-001–AC-519, AC-520 — the derived-facet gate — and the ids the
2026-09-11 amendment allocated from AC-521 upward, and **its** §12.10) · `references/adr/` (**114 records**,
`0001`–`0116` with `0090` and `0091` reserved for Steps 4.4 and 6.1; `references/adr/ERRATA.md` for corrections to
frozen records) · `spikes/DECISIONS-2026-09-11.md` and `spikes/DECISIONS-2026-09-12.md` (the operator decisions this runbook executes — the 2026-09-11 log runs to #27; the 2026-09-12 log carries E1–E13 from the alignment review and is cited as `DECISIONS-2026-09-12 E<n>`) ·
`spikes/F39-DRAFT.md` · `spikes/F40-DRAFT.md` · `spikes/F41-DRAFT.md` (S-PORTS) · `spikes/F42-DRAFT.md` (S-GRAMMAR) ·
`spikes/HANDOFF.md` · `spikes/DECISIONS-2026-09-09.md` · `references/2026-09-08-evidence-brief.md` §E. ·
**Tree state:** every path and every existing command in the stage files was re-verified against the working tree
on **2026-09-11** at the remediation branch's head (the previous revision's verification dates, 2026-09-08 and
2026-09-09, stand for the paths it did not touch). · **Commands that do not exist yet** — because the code is not
written — are marked **`[WILL EXIST]`** and are written in the form they will take; a `[WILL EXIST]` checkpoint
line is **deferred, not failed** (Halt 8's carve-out), and each names the stage at which it is re-run. Nothing else
in this runbook is aspirational.

**What this runbook decides, stated honestly.** It is not decision-free. It takes **seven** decisions; five are ADRs
and two are numbered decisions recorded in `spikes/DECISIONS-2026-09-11.md` before the stage that executes them:

| # | Decision | Where | Record |
| --- | --- | --- | --- |
| 1 | The context-vector source | **Step 4** — the blocking gate | **ADR-0090**, superseding ADR-0079 in full; carries the three LLM booleans' disposition (DECISIONS-2026-09-11 #2) |
| 2 | Whether S-GIX narrows the git write path | Step 6.1 | a **successor** to ADR-0077, never an edit to it — **ADR-0091** (ADR-0098 already fixes the no-go fallback) |
| 3 | The minimum `git` executable version the wrapper asserts | Step 6.2 constraint 3 | **TAKEN 2026-09-12** — `>= 2.20` unconditionally, conditional above that; **DECISIONS-2026-09-12 E10**. ADR-0091 records the final floor beside the S-GIX verdicts |
| 4 | `doctor`'s unread-artifact window `N` | Step 7d | **TAKEN 2026-09-12** — `unread_window_runs = 50`; **DECISIONS-2026-09-12 E9**. In the `doctor` rule table and `init`'s scaffold, cited by AC-472/AC-426 |
| 5 | `stale_policy` per cost class | Step 7e — pack authoring | **DECISIONS-2026-09-11 #4** (`rematerialise`, coarse scope, every class); recorded per class with its warrant in the pack; Checkpoint 8 asserts no value was defaulted from P2 |
| 6 | Per-AC mutants are author-declared | Step 5.3 (the spec template's `mutation:` field) and 7b-ii | **ADR-0104** |
| 7 | The Phase-1 companion kind list | Step 5.3 | **DECISIONS-2026-09-11 #3** — `_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`; recorded against ADR-0075 clause 3 |

**An eighth, added 2026-09-12.** The 2026-09-11 audit left the two must-fail fixture crates (AC-001–AC-004, AC-005) without a name, a path or a shape, and this table did not carry them as a decision. They are now decided — feature-gated workspace members at `tests/fixtures/rk-clock-read/` and `tests/fixtures/rk-banned-dep/`, **DECISIONS-2026-09-12 E4**, executed at s5 §5.1a. Five further decisions from the same review (**E1**, **E2**, **E3**, **E11**, **E12**) move a record at or above the runbook in ADR-0092's chain and are **owed, not executed here** — `DECISIONS-2026-09-12` names what each one must write.

**`0089` is taken, and so are `0092`–`0114`.** `references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md`
was accepted on 2026-09-09 and partially supersedes ADR-0030 (PC8); ADR-0093 (2026-09-11) supersedes it in part
again — `stage`/`rigor`/`role`/`cost` are `source = "kernel"`, `surfaces` is `source = "declared"`, six facets stay
`derived` — and Step 4a executes both. Step 4.4's context-vector record is **0090** and Step 6.1's S-GIX successor is
**0091**; both numbers are reserved in the ADR README and the next free number after them is **0115**. **Step 4a is
not a decision** — it is the execution order for the registry, schema and lint work ADR-0089 and ADR-0093 force.

Everything else this runbook does is execution order. **Where a stage is tempted to decide something else, it stops.**

**Invoking things.** All commands assume the repo root `D:/repos/redkiln-rs` unless the block opens with its own
`cd`; **every block that needs a working directory now states it**. The rig is
`cd D:/repos/redkiln-rs/spikes/runner && cargo build && ./target/debug/redkiln-spike-runner <cmd>`. Python tooling
runs from the repo root as `python spikes/<lane>/<script>.py`. The predecessor repo — the frozen corpus — is
`D:/repos/redkiln` at `47faec9f46b8d983c2c75f0794164e68415ae706`.

**Shell.** Every shell block in every stage file is **POSIX and is executed in Git Bash**, which is present on this box at
`/usr/bin/*`. This is not a stylistic preference: the file uses `rm -rf`, `cat <<'EOF'` heredocs, `sed -i`,
`printf`, `wc -l`, `grep -E/-c/-L`, `date -u`, `test -f/-s/-d` and leading `!` negation, **none of which work in
PowerShell** (`rm` there is `Remove-Item` and rejects `-rf`; there are no heredocs; `!` is `-not`). Open Git Bash,
`cd /d/repos/redkiln-rs`, and run them there. PowerShell equivalents are given only where a step has no POSIX
route. Drive paths (`D:/repos/...`) work unchanged in Git Bash.

**Tool preflight — checked 2026-09-11; all present.** Run this before Step 0 and install what it names:

```bash
for t in sed awk wc grep date python cargo rustc git claude; do   # awk: Checkpoint 4a's deriver-count line
  command -v "$t" >/dev/null || echo "MISSING: $t"
done
cargo audit --version  >/dev/null 2>&1 || echo "MISSING: cargo-audit  → cargo install cargo-audit --locked"
cargo insta --version  >/dev/null 2>&1 || echo "MISSING: cargo-insta  → cargo install cargo-insta --locked"
cargo deny  --version  >/dev/null 2>&1 || echo "MISSING: cargo-deny   → cargo install cargo-deny --locked"
rustup run 1.98.1 rustc --version >/dev/null 2>&1 || echo "MISSING: toolchain 1.98.1 → rustup toolchain install 1.98.1 --profile minimal -c rustfmt -c clippy"
# Measured on this box 2026-09-11: cargo-audit 0.22.2, cargo-insta 1.48.0, cargo-deny 0.20.2 and rustc 1.98.1
# are all present (the first two and the toolchain were installed during the 2026-09-11 remediation pass; the
# 2026-09-08 measurement had cargo-audit and cargo-insta MISSING). Step 5 requires all four; Checkpoint 5 cannot be
# run without them. `rustup` self-update fails on this box (os error 2) — pass `--no-self-update`.
```

---

## Preamble — the golden constraints

These hold for every step below. They are not advice; each one is a measured failure this programme already paid for.

1. **Nothing in the selection path over LLM-emitted facets starts before Step 4's gate clears.** ADR-0079 is
   `proposed`, is the only non-accepted record in the set other than two full supersessions, and **forbids being
   amended into a decision**. Until Step 4 closes, no `require` row may be keyed on an LLM-sourced `criticality`,
   `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro`. `kind` is out of scope of the
   block (human input, ADR-0030 rule 4). The engine's *semantics* are decided and may be built now; its *input* may
   not. [F36: modal `R` identical on 5/20, 3/20, 2/20, 2/20 against an 80% line; 400 dispatches, 0 void, $21.37]

   **F39 does not lift this and moves only one clause of it.** AC-071 has five clauses — identity ≥ 0.80, n ≥ 20
   items, ≥ 10 draws per item, arm-D panel **scored**, corpus pinned — and P1 arm D's scoring on 2026-09-09
   **discharges the fourth clause only** [measured, F39]. The block still stands on the **first**: modal
   required-set identity was measured at **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line [measured, F36]. F39
   measures a *derived* context vector; ADR-0079 asks about an *LLM-emitted* one, so F39 narrows 0079's options
   without deciding among them and **0079 stays `proposed`** until Step 4.4 writes its successor.

2. **Nothing may depend on a process surviving.** Anything long-running is a durable mark plus a later read. F22
   measured ~12 background kills against one 12,000 s survivor under `Start-Process`; `issue-81` was killed three
   times as a background job and completed in **116 s** in the foreground. Waits are marks, not sleeps; the rig
   already splits `ttl-mark` / `ttl-read` for exactly this reason.

3. **`cargo test` and `cargo clippy` DO NOT rebuild the binary. Always `cargo build` first.** This cost the
   programme four wasted runs and two false conclusions that code was broken. The invocation is
   `cd spikes/runner && cargo build` — *then* run the binary from `target/debug/`.

4. **A rig that compiles and passes its unit tests has NOT been run.** Three E3 defects survived "built and
   validated": `--json-schema` silently inert under a tool-restricted card, a `\\?\` path handed to `git fetch`,
   and `git fetch <abbrev-sha>` failing on every path that did not happen to call `rev-parse` first. None was
   reachable from a unit test, because none touched the filesystem or the CLI. **Validate one item before every
   set**, and prefer a haiku probe to an Opus one when the question is "does this flag work at all".

5. **Every waiter needs a failure alternative.** A marker-only wait spins forever on a dead job — one `until grep`
   waiter spun 25 minutes on a job that had already failed. Every wait is `grep -qE "MARKER|exited with code"` or
   equivalent, never `grep -q "MARKER"`.

6. **A default written for the fast condition costs a record on the slow one.** The 900 s timeout fitted S-SILENT
   and lost a CLEAN record; the ladder ran 900 → 2,400 → 3,600 → one 5,400 s retry. When two conditions differ in
   shape, size the ceiling against the slow one.

7. **Scripted edits need an anchor assertion.** Two silent `str.replace` no-ops were read as behaviour changes.
   Every scripted edit asserts its anchor exists *before* it writes, and fails loudly if it does not.

8. **The instrument version is part of every record.** F20 and F37 §7: a version move mid-run splits a comparison.
   P7 straddled 2.1.263 → 2.1.265, catching exactly `recommended`'s r3 draw on all ten items while `optional` ran
   wholly on 2.1.263 — no effect is detectable, and **the arms are still not instrument-matched**. Record the
   version; never silently continue across a move.

9. **A guard scoped wrong blocks or voids valid work.** HANDOFF, "Traps that cost time here": *"Two guards I wrote
   to protect data were scoped wrong and blocked or voided valid work."* This is the constraint that cuts **both**
   ways, and it is the one this runbook most nearly lost. A guard too narrow lets the failure through
   (`score-arm-d.py`'s facet regex admits seven facets and silently ignores a `recent_deploy` label, s3 §3b); a guard
   too wide refuses a valid run (a whole-second `labelled_at` refused against a microsecond engine stamp in the same
   second, s4 §4.3 — deliberate and correct, but only because it was *chosen*). **When you meet a guard, establish its
   scope before you obey it and before you route around it.** The scope is the finding, not the verdict.

10. **A pre-registered bar whose registered n cannot decide it is powerless, and must be reported as powerless.**
    **Four** measured instances now, all in the same pre-registration: P1's α rule [F36], P7's criterion 1 —
    powerless at n=3, every Wilson interval overlapping by construction [F37] — METHOD-P §1.4's 0.95 precision bar,
    which **no facet can clear at n=20 however good the method**: `ui` is 7/7 and its 95% lower bound is 0.65, and
    **73** all-correct positives are needed for a lower bound above 0.95 at the registered two-sided z = 1.96 (92 with continuity correction; 72 under Clopper–Pearson) [measured, F39 Limits 1 as corrected 2026-09-11, PIA-073] — and now **P2 §2.4's
    criteria 1 and 2**, whose recall half has no denominator at all: **0 of 41 rows are "should re-run"**, so recall
    is **NOT ANSWERABLE** and criterion 2 cannot fire either way [measured, F40]. **Four is a pattern in how this
    programme writes pre-registrations, not four coincidences** — each fixed a threshold without checking that the
    registered corpus could produce the denominator the threshold needs. **Before you run a
    criterion, ask what n it needs; after you run one, report saturated as saturated, powerless as powerless,
    not-measurable as not measurable and not-answerable as not answerable — never as a pass or a failure.** **Not-measurable and not-answerable are different faults and both are live:** F39's `data_migration` was NOT MEASURABLE (the script emitted no positive, so there was nothing to be precise about); F40's P2 recall is NOT ANSWERABLE (the label set carries no positive, so the denominator does not exist). This is a result *about* the
    pre-registration, reported in the finding; it is **not** a licence to edit METHOD-P after the numbers are in.

11. **A criterion that fires may prescribe a remedy its own measurement contraindicates. Record both halves.**
    P2's criterion 1 fires on its precision half — coarse `derived_from` precision **0.00** [0.00, 0.26] over 11
    emitted positives — and the remedy it prescribes, *"fine scope becomes mandatory for `L`-class activities"*, is
    **contraindicated by the same run**: on the 33 rows both scopes scored, coarse marks **5** `Stale` and fine
    marks **15**, and **10 rows flip `Present` → `Stale` going coarse to fine while none flips the other way**
    [measured, F40 §3]. A document that records the criterion as firing without recording that its own remedy is
    contraindicated has absorbed half the finding. **The design consequence is OPEN, and it needs a decision, not a
    default.**

12. **Production code is written test-first, and every criterion is discharged by a test that discriminates.**
    This is not a new rule — **ADR-0019** makes it the product's *normal* evidence standard and this runbook simply
    executes it: *"'Reviewed and green' is not evidence that a criterion is pinned. The per-AC discriminating test —
    TDD's normal mutation evidence — is the product's normal evidence standard, not an exceptional one."* Its
    root-cause note is the rule in one line: **Red→Green evidenced per test; an AC is a claim about one assertion.**
    The measured failure it exists to stop: **0 of 156** historical reports carried per-AC mutation evidence, and
    **≥ 12 of 23** shipped, human-approved fixes carried an AC that no test pinned [ADR-0019 drivers, proposal
    §4.6 item 8; F24/F25]. From Step 5 on, every sub-step that writes Rust runs in this order:

    - **Red.** Author the named test, run it, and **watch it fail for the right reason** — the absent behaviour,
      not a compile error, a typo'd filter or a missing fixture file. A test never observed red is not evidence
      that it can fail, and a filter that matches nothing exits 0 (Halt 8's shape, one level up).
    - **Green,** by the smallest change that satisfies it.
    - **Discriminate.** Every AC in the stage's acceptance set needs a test that fails when the behaviour is
      absent. Where the criterion declares a mutant — the `mutation:` field ADR-0104 makes author-declared and the
      harness's input — the mutant is authored, run, and **must flip the test**; a survivor blocks even when
      quarantined. **ADR-0096**'s typed repro (`kind`, `mutation`, machine-readable `expected`) carries the evidence,
      and a prose-only `command` is rejected at ingest.

    **A checkpoint grep is not this.** Asserting that a test named `X` exists and exits 0 is satisfied by an
    implementation nothing pins — exactly the gap between "green" and "pinned" that ADR-0019 was written about. The
    checkpoint is the floor, not the evidence. Constraint 4 is the same rule one level up: a rig that compiles and
    passes its unit tests has **not** been run.

**Where each constraint actually bites.** They are restated at these steps because the preamble is not where an
operator meets them:

| # | Restated at |
| --- | --- |
| 1 — the LLM-facet block | Step 4 (the gate), **Step 5 banner**, **Step 6 banner**, **Step 7a/7b banners**, Step 7e, Step 8 |
| 2 — nothing depends on a process surviving | Step 3a (the dispatch), Step 7c, Step 8 (3), Halt 5 |
| 3 — `cargo build` first | Checkpoint 0, Step 5.3, Step 6.1, Halt 1 |
| 4 — a rig that compiles has not been run | Step 3a, Step 7c |
| 5 — every waiter needs a failure alternative | Step 3a (the dispatch), Halt 5 |
| 6 — a default written for the fast condition | Step 3a (the dispatch timeout) |
| 7 — scripted edits need an anchor assertion | Step 3a, **Step 4a** (the registry edits), **Step 4.4** (the two ADR edits), Step 1.2 |
| 8 — instrument version in every record | Step 0.3, Step 1.3, Step 3a, Step 4.1, Halt 2 |
| 9 — a guard scoped wrong | Step 3b, Step 4.3, Halt 7 |
| 10 — a bar its own n cannot decide | **Step 4.1 clause B**, **Step 4.2 (P2's criteria 1 and 2)**, Halt 9 |
| 11 — a fired criterion whose remedy is contraindicated | **Step 4.2 (P2's criterion 1)**, Step 4.5, **Halt 12** |
| 12 — test-first, and a discriminating test per AC | **Step 5.3**, **Step 6.3**, **Step 7a**, **Step 7b**, **Step 7c**, **Step 7d** — every stage that writes Rust. Step 7e authors TOML and is out of its scope; Steps 0–4a write records, not code. |

---

---

## Legend and conventions shared by every stage file

- **`[WILL EXIST]`** — a command or path the code has not produced yet, written in the form it will take. In a
  checkpoint it is **deferred, not failed** (Halt 8's carve-out): record the deferral in `PROGRESS.md` and re-run
  the line at the stage the line names. A `[WILL EXIST]` line that goes green because the binary is missing is the
  Halt 8 shape and is never a pass.
- **`[measured]` / `[documented]` / `[inferred]`** — as in the design and the registers: computed from a record in
  this repository (named); read from a live source (with a fetch date); reasoned.
- **Fault names** — saturated, powerless, not-measurable, not-answerable are four different faults and are named as
  such; never `0.00`, `1.00` or `n/a`.
- **Cross-references** — `Step N`, `Checkpoint N` and `Halt N` are global names; a bare `§N.M` refers to the stage
  file it appears in; another stage's section is `sN §N.M`; other documents are prefixed (`design §10.5`,
  `METHOD-P §2.4`, `AC §12.10`).
- **Checkpoints** — every checkpoint line is `command   # → expected result`. A checkpoint carries a BEFORE
  assertion (a line that fails on the untouched tree) beside its AFTER assertions wherever the audit found one that
  was green before the step ran (PIA-035); a checkpoint you could not run is a failed checkpoint (Halt 8).
- **Acceptance sets** — each Phase-1 stage lists the criteria it claims as an explicit range **minus** the ids it
  excludes, naming each excluded id and why (retired, PROVISIONAL, CONDITIONAL, human-gated, Phase 2+, or blocked on
  a named spike).
- **A checkpoint that changed after it passed is provisional.** When a stage file's checkpoint lines are edited
  after that checkpoint was recorded as passed, the recorded pass attests the *old* text. This has already
  happened once: `95e0164` (2026-09-12) rewrote every line of Checkpoint 0 and every line of Checkpoint 1, both of
  which were recorded as passed at `bf5086a` and `66276bf` before that commit existed — and the pre-hardening forms
  are the ones `halts.md` now names as unable to fail. **Mark the row `passed <hash>, re-run pending <reason>` in
  `PROGRESS.md` §1 and re-run the hardened block**; a pass is evidence about a specific text, not about a stage.
- **A stage closes in two commits, not one.** `INDEX.md` says to record the result in `PROGRESS.md` "in the same
  commit as the work", and §1's legend wants `checkpoint passed <hash>` — which are jointly unsatisfiable, because
  the row would have to embed the hash of the commit that contains it. The executed pattern at all three closed
  stages was **(1)** the work and its checkpoint outputs, then **(2)** a tracker commit writing that commit's own
  hash into §1 (`a64f1f2`, `8a2b5a3`, `54b61d8`; Step 0 differed — `bf5086a` touches `PROGRESS.md` alone). A §1 row
  is hashless only between the two. Any cleanliness assertion in a checkpoint must tolerate the tracker row being
  unstaged at the moment it runs.
- **Execution order** — `0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7c → 7d → 7e → 8`; Step 2 runs sixth (its
  ORDER banner); Step 9 is rollback and may run at any point.
