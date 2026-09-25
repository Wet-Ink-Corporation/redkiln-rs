# Step 8b — The catalog's quality instruments over the shipped pack (Phase 1, Step 8 sub-stage b)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8b` | Phase 1 | Checkpoint 8a | Checkpoint 8b | `s8a` | `s8c` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

*Revised 2026-09-24 for D21-43/44/45* (`spikes/DECISIONS-2026-09-21.md`, at `a6349b6`). What changed:
- §8b.1's four questions are **closed, not asked**. D21-43 N18 closes Q1 and Q2, and N26 closes Q3 and Q4. This file
  cites the rulings and does not re-take them.
- §8b.5 applies N18 row by row: (C) for `differential-debugging` and `instrumented-trace-analysis`, (D) for
  `user-story-ac` and `vertical-slicing`, and (A) for `threat-model`, which waits on §8b.6's record. N18 supersedes
  D21-42 11b's "re-execution quantities" for those five rows. Condition (c) now follows input chains transitively.
- §8b.5 also owns three more ADR-0072 catalog edits: AC-018's 19 `memo` rows in 14 activities (N26), tdd
  `red-verify`'s `[*].tests_added` projection (N13), and `sca-scan`'s `unrated` arm (N25). AC-018 joins the
  acceptance set. Twenty-two activities move `version` in all.
- §8b.2's `.redkiln/` run gains `rust` and a `surfaces` binding by a dated METHOD-P amendment (N26). The p9 parity
  goldens keep the registered axes.
- §8b.6: in every branch, 11c's record names the source and owner of the `severity` field that the 39 shipped rows
  read and `rk_types` `Finding` lacks.
- Checkpoint 8b: both recursive walks over `.redkiln/` exclude `worktrees/` (D21-44), with a planted control, and the
  exclusion is scoped so that it cannot skip `crates/`' own `worktrees` fixture directories. The
  named selection is 26 tests (30 after the D21-47 fixes below). The floor is re-derived from Checkpoint 8a's recorded count. The re-run scope and the
  no-dispatch rule are stated.
- D21-45 (W1–W4) changes nothing else here. This stage creates, re-enters and removes no worktree.

*Consistency pass 2026-09-24* (the eleven `s8` files read against each other at `c9e7b6e`; D21-46 the authority):
- **Section (0) adopts D21-46's re-run convention and replay guard.** It skips its whole re-run of Checkpoint 8a
  under `RK_CP_NESTED=1`, and otherwise launches 8a with `RK_CP_NESTED=1`, `REDKILN_LAUNCH_RECORD` unset and
  `CLAUDE_CONFIG_DIR` at an empty trap that must start and stay empty.
- **The floor chain is recomputed once** (D21-46): Checkpoint 8a's 749 + this stage's 21 = **770** (774 after the D21-47 fixes below), not the drafted
  768, and (12) gains the at-least-44-result-lines guard.
- **`s8c` §8c.12's AC-018 row is re-quoted** from its current bytes.

*Adversarial-review fixes 2026-09-24 (D21-47)* (the review of this file at `4a8c832`, applied at `fc80687`;
`spikes/DECISIONS-2026-09-21.md` D21-47 is the authority, with its V2–V5 and its fix-round rules):
- **B1** → the acceptance set and §8b.2's kill-line paragraph take **V3**. The thresholds apply to the registered pack
  set, the eight pre-registered packs at `pack-default`. Checkpoint 8b section (7) parses that reading and exits 1
  naming the AC if a line fires, with controls at the boundary. The installed pair is reported with its count in
  section (6) and never fires the line.
- **B2** → section (7) re-derives both goldens from the Python at `P9_PIN` into `target/cp8b/` and `cmp`s them. Named
  divergences live in a committed `divergences.tsv`, whose row count section (7) pins.
- **M1** → section (4)'s comment filter follows each file's own syntax (`.rs`, `.toml`, `.yml`), so a Markdown `*`
  bullet is not dropped. `.redkiln/CHANGELOG.md` is an EXCL row, with two controls.
- **M2** → section (9) reads the labels at the corpus commit the record names, proves that this commit added them, and
  requires every METHOD-SEV commit after the labels to be append-only with a dated amendment.
- **M3** → section (10) asserts that AC-527's two contradicting clauses are gone or struck. Its control is the
  unamended register at `ca8f387`.
- **M4** → section (11) also reads the S-SEV record, METHOD-SEV, the METHOD-P amendment, the AC-101/102/527 entries
  and the commit messages since `ca8f387`.
- **M5** → §8b.6 states the 39 rows' behaviour as the evaluator has it: `true` over an EMPTY list, `UnknownField`
  otherwise, and not a refusal at bind time. It adds the branch in which the record supersedes ADR-0080 d2 bullet 1
  in part.
- **M6** → **V4**: `adversarial-review`'s `verdict` moves after `record` in the same ADR-0072 edit. A named test
  asserts it, and so do section (3d) and mutant M29.
- **M7** → cross-file finding 5: `s8h` carries `RootCause.discriminating_test`, and this file cites it.
- **M8** → **V5** is recorded at condition (c) and in the D-entry. (A)'s omission of *"survives re-execution"* is
  disclosed there on N18's ruling.
- **M9** → three fixtures and tests: AC-103's injected cycle, AC-102's dead activity, and AC-104's hole at
  `mandatory`. Mutants M25–M27 cover them.
- **M10** → the planted control moves to a scratch tree shaped like `.redkiln/`. The walk functions take the root as
  an argument, and the real `.redkiln/` is never walked unexcluded.
- **M11** → **V2**: the five tests that pin a historical p9 measurement read the p9 tree at `P9_PIN` through the
  helper `p9_pin`. The helper has its own discriminating test and mutant M28. The other readers are inventoried in
  §8b.3, and section (2b) guards the split. `ci.yml`'s checkouts gain `fetch-depth: 0`.
- **N1** → the severity-vocabulary notes are located by phrase, in six design rows. **N2** → D1, D2, D4, D6 and D9.
  **N3** → a 27th parity test moves the counts by a dated edit. **N4** → `s7b-ii` selects `ac303` by bare name, and
  that name is kept. **N5** → D21-46's `[*]` flag is cited at N13. **N6** → section (2) reads
  `git status --porcelain --untracked-files=all`. **N7** → section (6) asserts that the METHOD-P amendment's commit
  precedes the shipped test's, and asserts the AC-101/102 and SM-026/027 notes. Section (8) asserts the `--exact`
  command and the `... ok` line. **N8** → the `does_not_parse` exclusion's reason is stated (§8b.7). **N9** → the
  memo `println!` names the fixture registry. **N10** → section (4) has a positive control that the `.redkiln/` half
  reaches `.redkiln/catalog`.
- **Cross-file finding 4** → the record names **`s8h`** as the `severity` field's owner, and the 39-row statement is
  corrected (M5). **Cross-file finding 5** → as M7.
- **The fix-round rules** → section (W) is the replay wrapper: a fake `claude` that prints the pin,
  `env -u REDKILN_LAUNCH_RECORD`, and a trap `CLAUDE_CONFIG_DIR` with controls. It covers section (0)'s replay and every
  cargo run. The fixture harness is `target/debug/rk-fixture-child-claude`. The S-SEV record is located by the id
  written into section (9) at write time, and never by a search.
- **New, found while fixing** [measured 2026-09-24 over the tree materialised at `P9_PIN`]:
  - The Python lattice takes about **2 min 10 s**, not the review's 27 s.
  - Its output is **not deterministic**. `cycles()` iterates a `set` of strings (`for v in edges[u]`), so the six
    `soft … ->` example lines follow hash randomisation. A fresh run printed `soft project design-doc ->
    data-migration-plan -> design-doc`, where run10 has `data-migration-plan -> design-doc -> data-migration-plan`.
    Every Python run here sets `PYTHONHASHSEED=0` and records `python --version`. The six lines are named
    divergences (§8b.2).
  - `actions/checkout@v4` fetches depth 1, so V2's pin would be absent in CI without `fetch-depth: 0`.
- **New tests: 25** (the 21 as drafted, three lattice fixtures, and `p9_pin`'s test). **Named selection: 30.**
  **Floor: 749 + 25 = 774.**

*Final consistency pass 2026-09-24* (the `s8` files read against each other after the D21-47 fix round; the brief's
four items). One line per edit:

- *Final consistency pass 2026-09-24:* section (W) sources the shared `runbook/tools/nodispatch.sh` (`nd_setup`) in
  place of its own `sh` fake and trap; `rkw` and `cfg_trap_empty` are gone.
- *Final consistency pass 2026-09-24:* every cargo run and section (0)'s replay go through `nodispatch`, and
  `nd_check` follows the replay, (1), (5), (6) and (12).
- *Final consistency pass 2026-09-24:* section (0) replays 8a as `nodispatch env RK_CP_NESTED=1 bash`; 8a's guard (0)
  reuses this wrapper, and 8a needs no environment: its record ids and `FIX_ITEM` are literals in its block.
- *Final consistency pass 2026-09-24:* section (9)'s `SSEV_ADR` placeholder is `____`, the one form every `s8`
  checkpoint now uses for an id written at write time.
- *Final consistency pass 2026-09-24:* §8b.5 item 3's ownership sentence quoted `s8c`'s superseded draft; it now cites
  `s8c`'s and `s8g`'s current text.
- *Final consistency pass 2026-09-24:* §8b.9's wrapper bullet and the block's dependency notes describe the compiled
  fake.
- *Final consistency pass 2026-09-24:* floor re-checked: 30 named tests, five of them pre-existing, so 25 new; 749 +
  25 = **774** stands.

*D21-48 (2026-09-24)* (the operator's answers to the questions the `s8` files left open):
- *D21-48 (2026-09-24):* §8b.5 item 2's `[*]` projection over a list-valued field is **settled by D21-48 X5**:
  *"Concatenation in element order. Nothing is flattened implicitly anywhere else."* The sentence that said D21-46
  flags it for the operator's flip lines and does not decide now cites the decision. No test, count or floor moves.

---

**Why this step exists.** The product ships `.redkiln/`, and not one catalog-quality instrument on this tree reads it.
`crates/rk-kernel/tests/it/rigor.rs`, `quality_block.rs` and `review.rs` each carry
`const SHIPPED_CATALOG: &str = "spikes/p9/catalog";`, and the P6 lattice is a Python script whose inputs are
hard-wired to its own directory (`facets = load(os.path.join(HERE, "facets.toml"))["facet"]`). The two catalogs
are not the same bytes: `diff -rq --strip-trailing-cr .redkiln/catalog spikes/p9/catalog` names **10** differing
files and the two `facets.toml` differ [measured 2026-09-24 at `ca8f387`]. AC-101's own baseline says so in terms:
*"no rate is measured over the installed pair"*. D21-42 **13a** and **13b** close that: every catalog-quality
instrument runs over `.redkiln/` and asserts a non-zero count walked, and the lattice is ported to Rust and runs in CI.

The second half is ADR-0080 decision 2, which the shipped catalog breaks seven times. D21-2 left the reading
*"RAISED and not taken"*; D21-41 **(h)** took it — *"Execute ADR-0080 d2 as written, in any quoting"* — and D21-42
**11a**, **11b** and **11c** extended it to `adversarial-review`, to the four one-member `['Reproduced']` rows, and
to d3's threshold. D21-43 **N18** then replaced 11b's wording for five of the seven rows with a ruling per row. The AC-527 lint that would catch the rows has never read the shipped catalog: it pins
`const SHIPPED_VIOLATIONS: usize = 11;` over `spikes/p9/catalog`, and its own module says the reconciliation with
d2 *"is OWED and is not written here"*.

**This step takes one record, and the questions it found are already ruled.** The record is 11c's: the severity
vocabulary and ADR-0080 d3's threshold T, written on a measurement (§8b.6), as a new ADR at the next free id at
writing, in ADR-0085's `### Option — ` form (D21-42 24a). §8b.1's four questions were free residuals when this file
was drafted. D21-43 took all four (N18 and N26), and this file cites those rulings. The AC-527 change is a dated
register amendment and not an ADR, because no Decision sentence or forbids bullet moves (packet (h) option A, *"There is no new
ADR"*). **No real dispatch happens at this stage.** The labelling in §8b.6 is done by the operator over recorded
data.

**Acceptance set.** AC-018 (over `.redkiln/catalog`, D21-43 N26), AC-101–AC-105, AC-303 and AC-527 (as amended by
§8b.4), minus:
- **AC-116**: the diagnostic on AC-104's failing report is not ported, because `lattice.py` has no such output;
- **AC-111 and AC-117**: the CI job that joins digests to versions and changelog rows is not built here. ADR-0072 is
  executed by hand for the twenty-two activities §8b.5 edits, and asserted by two named tests over them, which is not
  AC-111's instrument;
- **AC-018's liveness clause over a SHIPPED verb registry**: `.redkiln/` ships no verb registry until `s8c`, whose
  §8c.12 settles the row *"AC-018's `live_verb_memo_rule` has a shipped registry and nothing runs it over
  `.redkiln/`"* with `verb_registry::ac018_holds_over_the_shipped_catalog_and_registry`. After §8b.5's repair no shipped step declares `memo` at all. Clause 1 needs
  `step.memo_declared == Some(true)` and clause 2 fires on `if step.memo_declared.is_some() {`, so both are silent over
  `.redkiln/catalog` whatever registry is passed. The re-proof against the shipped registry is `s8c`'s;
- AC-101's and AC-102's **kill-line verdicts over the installed pair** (D21-47 **V3**). The kill lines apply to the
  registered pack set, the eight pre-registered `spikes/p9` packs at `pack-default` rigor, and Checkpoint 8b section
  (7) thresholds that reading. The installed pair is measured and reported with its pack count (SM-027). It does not
  fire the kill line, because its unreachability is Phase-1 scope by design (ADR-0011, E7).

### 8b.1 Before anything moves: the inventory, the BEFORE readings, and the four questions D21-43 closed

**The instruments that read `spikes/p9/`**, re-measured at stage open from code lines:

| instrument | test | what it reads today | this stage |
| --- | --- | --- | --- |
| AC-105 rigor monotonicity | `rigor::rigor_monotonicity` | `spikes/p9/catalog` + `spikes/p9/facets.toml`, pinned `(15, 2060, 1729)` | re-pointed to `.redkiln/`; the p9 pin moves to a historical test, named so, which reads the p9 tree at `P9_PIN` (§8b.3, D21-47 V2) |
| AC-303 confirm-step monotonicity | `review::ac303_confirm_step_survives_escalation` | `spikes/p9/catalog`, floors only (`families >= 10 && comparisons > 1_000`) | re-pointed; no historical split, because it pins no figure |
| AC-527 terminal-condition lint | `quality_block::ac527_empty_set_terminal_condition` | `spikes/p9/catalog`, `11` refused / `83` indeterminate | re-pointed and amended (§8b.4); the frozen p9 catalog, read at `P9_PIN`, stays RED in a separate, named control |
| P6 lattice (AC-101–AC-104) | `spikes/p9/lattice.py` | its own directory only | ported (§8b.2); its p9 parity reads `P9_PIN` |
| AC-018 memo lint | `memo::ac018_live_verb_may_not_memoise` (the `live_verb_memo_rule` assertion) | `spikes/p9/catalog`, `19` violations *"RED by design"* | re-pointed to `.redkiln/catalog` after §8b.5 repairs the 14 activities (D21-43 N26); the p9 pin moves to a frozen control, named so, which reads `P9_PIN` (§8b.3) |

**The test names are kept.** Checkpoint 7b's loop selects `rigor_monotonicity` and `ac527_empty_set_terminal_condition`
with `grep -qE "^test .*\b${t}\b.* \.\.\. ok$"`, so renaming either would turn a checkpoint red for a reason
unrelated to the rule. The historical tests get new names that `\b` does not match (`_p9_…` suffixes). `s7b-ii`'s
checkpoint also selects `ac303_confirm_step_survives_escalation` by bare name, and that name is kept too.

**Four questions this file found by reading the tree. D21-43 ruled on all four** before any stage file was revised.
Each ruling is cited here and not re-taken. The measured premises stay, because the D-entry (§8b.7) records them.

1. **Which corpus does 11c's labelling read? CLOSED by D21-43 N18:** *"11c labels F37's 188 findings; E3's six are
   excluded"*, *"because the operator has seen their filled sheet"*. The premise: D21-42 11c said *"the F37/F39 review
   corpus"*, and **F39 holds no review findings**. Its header is *"F39 — P1 arm D RESULT: no derived facet clears
   §1.4's 0.95 bar"*, and it is a facet-precision measurement. The review-finding corpus is F37's
   (`results/p7.jsonl` in `redkiln-corpus`, 33 adversary dispatches, 188 findings). §8b.6 reads it.
2. **What replaces the policy predicate in five of the seven rows? CLOSED by D21-43 N18**, row by row: *"re-point
   `confirm` at the discriminating test for `differential-debugging` and `instrumented-trace-analysis`"*; *"keep
   `check.failed == 0` for `user-story-ac` and `vertical-slicing`, reading bullet 2 as illustrative"*; *"a threshold
   at 11c's T for `threat-model`"*. N18 *"Supersedes D21-42 11b's "re-execution quantities" for these five rows."*
   The premise: for these five, no quantity in the row's own steps is both a d2 form and varied by the loop. §8b.5
   applies the ruling.
3. **Which lattice axes run over `.redkiln/`? CLOSED by D21-43 N26:** `rust` and a `surfaces` binding are added *"by
   a dated METHOD-P amendment, with the old axes kept for the parity goldens"*. The premise: `lattice.py`'s toolchain
   axis is `TOOLCHAINS = OrderedDict([("node", …), ("unreal", …), ("node+unreal", …), ("dotnet", …)])`. **It has no
   `rust`**, although `.redkiln/facets.toml` declares `values = ["node", "dotnet", "python", "rust", "unreal"]` and
   Rust is the only toolchain Phase 1 ships. Its context binds no `surfaces` either, but
   `.redkiln/catalog/threat-model.toml` reads `applicable_if = "'auth' in surfaces or …"`. The spike copy reads
   `"data_migration or external_dep"`. §8b.2 applies the ruling.
4. **Does AC-018 run over `.redkiln/`? CLOSED by D21-43 N26:** the memo lint runs over `.redkiln/`, *"and its 14
   activities are repaired under ADR-0072 in s8b"*. The premise: `grep -rn '^memo\s*=' .redkiln/catalog` finds **19**
   rows in **14** files [re-measured 2026-09-24 at `a6349b6`], the same 19 that `memo.rs` pins over p9. §8b.5
   repairs them, and §8b.3 re-points the lint.

**BEFORE readings (PIA-035), each a line that fails on the untouched tree.**
- `grep -rnE "findings\.none\(evidence in \[" .redkiln/catalog` returns **7** live `expr` rows.
- `quality_block.rs` points at `spikes/p9/catalog`.
- No `crates/rk-kernel/tests/it/lattice.rs` exists.
- `git ls-files | grep -ic changelog` is **0**: ADR-0072's "pack changelog" has never been written.
- `grep -rnE '^memo\s*=' .redkiln/catalog` returns **19** rows in **14** files (question 4).
- `tdd-red-green-refactor`'s `red-verify` reads `inputs  = { selection = "${red.outputs}" }   # list of ImplReport;
  tool flattens tests_added (schema gap: no flatten)` (N13).
- `sca-scan`'s `clean` reads `expr    = "report.critical == 0 and report.high == 0"`, so it passes with an unrated
  advisory (N25).

### 8b.2 The P6 lattice in Rust (D21-42 13b), and parity before the Python is demoted

**What is ported, clause for clause.** The module is `rk_kernel::lattice`. It ports `lattice.py` v2 as it stands:
- interpretations I1–I9 and decisions D1, D2, D4, D6 and D9, the five that its module doc enumerates;
- the context enumeration: 10 kinds × three live rigors plus `pack-default` × `BANDS^4` = 81 × four toolchains ×
  the boolean covering array × `cos`, with the doubled `cos` for `bug`;
- `Run.producer`'s recursion, which includes the keyed-kind expansion, the same-stage-then-pack-order upstream
  search and the cycle guard;
- the unsatisfiable, dead, tie, cycle, overrun, mixed-differential and `unknown`-sweep reports;
- `--validate`'s family-totality and `requires` checks.

Three things the port must get right, each a measured trap:

- **The covering array is seeded Python randomness.** `rnd = random.Random(7)` drives a greedy search, and the run10
  baseline records `bool-covering-rows=8`. Rust cannot reproduce Mersenne Twister draws without porting them. So the
  eight rows are **transcribed** from the Python as data, and
  `lattice::covering_array_is_pairwise_and_matches_the_python_rows` proves two things: every pair of the 8 booleans
  is covered in both polarities, and the table equals the rows Python prints.
- **Stage order is textual, and the kernel's TOML reader alphabetises.** `self.policy = d["policy"]["stages"]` is
  an insertion-ordered dict. Python iterates it for stage processing and for I1's *"preferring pack order"*
  upstream search, so the order moves which producer is chosen. `toml_subset::Table` is a `BTreeMap`. ADR-0153 exists
  because `parse_spans` cannot recover the order of a one-line inline table.
  - `.redkiln/` packs declare `stage_order`, and the port reads it.
  - `spikes/p9/packs/*.toml` declare none (`grep -c stage_order` → 0 in each). The port needs an order-preserving
    scan of the `stages = { … }` line, cross-checked against the table's key set. A port that iterates the map will
    fail parity, and mutant M10 proves it.
- **The predicate evaluator is the product's, not Python's `eval`.** The port evaluates through
  `rk_kernel::predicate` (ADR-0103, ADR-0121), and `lattice.py` compiles predicates to Python expressions. **A
  figure that diverges is not shimmed.** It is traced to the predicate that moves it, recorded with that predicate
  and the figure, and parity is then claimed on every figure except the ones named. A Python-compat evaluator mode
  is refused.

**The Python baseline, as recorded** [measured 2026-09-21, `spikes/p9/results-p6-run10-post-adr-0093.txt`, twelve
`spikes/p9` packs]:

| figure | value |
| --- | --- |
| TOTAL prereg-8 / all-12, `optional` | `unsat=32220/155520 (20.7%)` / `unsat=35676/235872 (15.1%)` |
| TOTAL prereg-8 / all-12, `recommended` | `unsat=1728/155520 (1.1%)` / `unsat=1728/235872 (0.7%)` |
| TOTAL prereg-8 / all-12, `mandatory` and `pack-default` | `unsat=432/155520 (0.3%)` / `unsat=432/235872 (0.2%)` |
| dead | `dead=7/102 (6.9%)`, and `dead=14/102 (13.7%)` under the eight pre-registered packs |
| mixed differential | `mixed contexts=28512; with an extra unsatisfiable (stage,kind) vs pure node=0` |
| unknown sweep | `unknown sweep: unsat=16/11648 (0.1%)` |
| rank ties | `none` |
| `--validate` | `validate: no errors`, and in the 2026-09-07 run file `validate errors=0` |

**How parity is proven: the same numbers on the same inputs, byte for byte.**

1. **Regenerate the goldens at stage open, from the Python, over `spikes/p9` at `P9_PIN`, and time them for
   SM-025.** Three things shape the command:
   - The p9 tree is materialised from the object store at `P9_PIN` (§8b.3, D21-47 V2), and every file is checked
     against its blob id, because no checkout of this repository is a byte-faithful export.
   - `PYTHONHASHSEED=0` is set, because `cycles()` iterates a `set` of strings, and its example lines otherwise move
     from run to run [measured 2026-09-24]. `python --version` is recorded beside the goldens, because string hashing
     is a property of the interpreter.
   - The Python prints `×` and writes through a text-mode stream, so on Windows the bytes would be CRLF and possibly
     not UTF-8. Writing through a reconfigured stdout makes the file the program's bytes.

   The run takes about **2 min 10 s** [measured 2026-09-24 over the pinned tree].
   ```bash
   cd D:/repos/redkiln-rs
   P9_PIN=ca8f3874ed9185b419b00b21b33713ba128de217
   P9_AT=target/s8b/p9-at-pin
   G=crates/rk-kernel/tests/fixtures/lattice
   rm -rf "$P9_AT" && mkdir -p "$P9_AT" "$G" || { echo "FAIL: cannot create the materialisation or golden directory"; exit 1; }
   git -c core.autocrlf=false archive --format=tar -o target/s8b/p9-at-pin.tar "$P9_PIN" spikes/p9 || { echo "FAIL: git archive of spikes/p9 at $P9_PIN failed"; exit 1; }
   tar -xf target/s8b/p9-at-pin.tar -C "$P9_AT" || { echo "FAIL: cannot extract the pinned p9 tree"; exit 1; }
   git ls-tree -r "$P9_PIN" -- spikes/p9 > target/s8b/p9-ls.txt || { echo "FAIL: git ls-tree over spikes/p9 at $P9_PIN failed"; exit 1; }
   nls=$(grep -c . target/s8b/p9-ls.txt)
   [ "$nls" -gt 0 ] || { echo "FAIL: the pinned tree lists no files — there is no oracle"; exit 1; }
   bad=0
   while read -r mode type oid path; do got=$(git hash-object --no-filters "$P9_AT/$path"); [ "$got" = "$oid" ] || { echo "MISMATCH: $path"; bad=$((bad + 1)); }; done < target/s8b/p9-ls.txt
   [ "$bad" -eq 0 ] || { echo "FAIL: $bad of $nls materialised files are not their blobs' bytes"; exit 1; }
   pyv=$(python --version 2>&1); rc=$?
   [ "$rc" -eq 0 ] && [ -n "$pyv" ] || { echo "FAIL: python is not runnable (rc=$rc)"; exit 1; }
   printf '%s\n' "$pyv" > "$G/p9-python-version.txt" || { echo "FAIL: cannot record the Python version the goldens are made with"; exit 1; }
   PYTHONHASHSEED=0 PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py']; runpy.run_path('$P9_AT/spikes/p9/lattice.py', run_name='__main__')" > "$G/p9-run.txt"; rc=$?
   [ "$rc" -eq 0 ] || { echo "FAIL: the Python lattice exited $rc — there is no oracle to port against"; exit 1; }
   PYTHONHASHSEED=0 PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py','--validate']; runpy.run_path('$P9_AT/spikes/p9/lattice.py', run_name='__main__')" > "$G/p9-validate.txt"; rc=$?
   [ "$rc" -eq 0 ] || { echo "FAIL: lattice.py --validate exited $rc"; exit 1; }
   grep -qF "packs=['bug-expedite', 'bug-standard', 'chore', 'docs', 'incident', 'initiative', 'project', 'security-vuln', 'spike', 'story-ue', 'story', 'tech-debt']" "$G/p9-run.txt" || { echo "FAIL: the golden does not name the twelve packs — a zero-pack run prints a whole baseline and exits 0"; exit 1; }
   ```
   `P9_AT` is a path relative to the repository root on purpose. An absolute Git Bash path such as `/c/Users/…`
   reaches Python unconverted inside the `-c` string, and Python reads it as `D:\c\Users\…` [measured 2026-09-24].
   Every figure in the table above must reappear in `p9-run.txt` after its `| ` separator. The `cat:`/`fac+art:`/`pk:`
   digest tags may differ from run10's, because ADR-0164 (still `proposed`) renormalised line endings at `ac10470`,
   and the tags hash raw bytes. Record both sets of tags. A fresh run at `P9_PIN` printed `cat:fdf842cca5b9`, where
   run10 has `cat:2d7f63b432cc`, with the same `fac+art:` and `pk:` [measured 2026-09-24]. If a figure moved, STOP:
   the oracle is not the one the register quotes.
2. **Measure whether the Python can read `.redkiln/` at all.** Stage `.redkiln/{catalog,packs,facets.toml,artifacts.toml}`
   beside a copy of `lattice.py` in a scratch directory, because `HERE` is the script's own directory, and run it
   there. The prediction is a failure on `'auth' in surfaces` *[inferred]*. If it completes, a third golden
   (`redkiln-run.txt`) and `lattice::p6_lattice_parity_over_the_shipped_pack` join the set. If it fails, the failure
   is recorded verbatim, parity over `.redkiln/` is **NOT MEASURABLE** by this oracle, and the Rust run over
   `.redkiln/` stands on its own pinned figures. Neither outcome is a pass or a fail of the port. Even a completed
   Python run reads only the registered axes, so a `redkiln-run.txt` golden compares the port under
   `Axes::p6_registered()` and never under `Axes::shipped()` (below).
3. **The goldens are byte-pinned.** A `.gitattributes` row `crates/rk-kernel/tests/fixtures/lattice/** -text` is
   added on ADR-0151's route, and the checkpoint reads the raw bytes with `od -c`, because Git Bash cannot see a CR
   inside `$(…)`.
4. **The Rust port renders the same report lines, digest tags included.** The tag is `sha256(basename ‖ bytes)[:12]`
   over sorted paths, which the port computes identically. `lattice::p6_lattice_parity_over_the_p9_tree` and
   `lattice::p6_validate_parity_over_the_p9_tree` run the port over `p9_pin::root()` (§8b.3) and assert equality
   with the goldens. **A tag is part of the claim**: parity is about these bytes.
   - **Named divergences.** A figure traced to a predicate under the third trap above, and each of the six `soft … ->`
     cycle-example lines, is one row of `crates/rk-kernel/tests/fixtures/lattice/divergences.tsv`. A row reads
     `<golden>\t<line number>\t<reason>`. Python picks a cycle example by `set` iteration order, which the port does
     not emulate. So for an example row the parity test asserts that the port's line names the same pack and kind
     (`HARD` or `soft`) and a closed cycle over edges of that pack's selected graph. It does not assert Python's
     rotation. Both parity tests read the file, and they skip no line that it does not name. The file is committed
     even when it holds only the six rows, and Checkpoint 8b section (7) pins its row count.
   - **The story does not move the goldens (D21-47 V2).** The parity tests and the goldens read the p9 tree at
     `P9_PIN`. When the story lane hardens `spikes/p9/packs/spike.toml` in the working tree (D21-41 f, `s8j`), no tag
     moves and no golden is regenerated, and the lane's whole-suite verify stays green. The Python is demoted and not
     deleted because section (7) re-derives both goldens from it at `P9_PIN` on every run.
5. **Then the Python is demoted.** From the parity commit on, the instrument of record for AC-101–AC-104 is
   `rk_kernel::lattice`. AC-101 and AC-102 each take a dated register note that names the tests. The note records the
   Rust reading over `.redkiln/` with **its pack count (2) and rigor mode (`pack-default`)**, beside the retained
   twelve-pack p9 figures (DECISIONS-2026-09-12 E7's two numbers), and the same reading is recorded against
   SM-026/SM-027. `lattice.py` is not edited. It stays the oracle for regenerating the p9 goldens.

**Over `.redkiln/`, on the axes D21-43 N26 fixes.** `rk_kernel::lattice` carries two axis sets:
- **`Axes::p6_registered()`** is `lattice.py`'s set as it stands: the four toolchains and no `surfaces`. Every parity
  test over `spikes/p9` uses it and only it, so the goldens stay the Python's bytes.
- **`Axes::shipped()`** is the registered set plus a `rust` toolchain row, transcribed in the Python's own shape as
  `("rust", ({"rust"}, {"text"}))`, plus a `surfaces` binding. Only the `.redkiln/` tests use it.
- **The METHOD-P amendment is committed first.** It is dated and takes the next amendment number at writing. The last
  one in `spikes/METHOD-P.md` at `a6349b6` opens *"Amendment 18 (2026-09-11, Ryan)"*. Its opening reads
  `*Amendment <n> (<date>, D21-43 N26):*`. It amends METHOD-P §3.2's *"toolchain sets {node, unreal, node+unreal,
  dotnet}"* for the `.redkiln/` reading only, and says that the old axes stay the pre-registered reading for the p9 goldens. It
  registers the `surfaces` enumeration before the Rust run over `.redkiln/` is first read. This file proposes the empty
  set plus each value `.redkiln/facets.toml` declares, as a singleton, from
  `values = ["auth", "flags", "migration", "payments", "secrets"]`, which gives six rows. It moves no kill threshold.
- **`lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged`** pins both sets. The registered set
  must equal the Python's transcribed lists. The shipped set must be that set plus exactly the two additions.
- `lattice::p6_lattice_over_the_shipped_pack` runs the port over both installed packs under `Axes::shipped()`. From
  inside the test binary it prints `lattice-shipped: packs=<n> rigor=pack-default contexts=<n> pairs=<n> unsat=<n>
  dead=<n>/102 ties=<n> hard_cycles=<n> toolchains=<comma list> surfaces=<n>`, with the last two fields at the end
  of the line. `rigor=pack-default` is the mode AC-101 pins its rate to.
- It asserts `packs == 2` and `contexts > 0`. It also asserts that ties and hard cycles are 0 (AC-103's zero
  tolerance), and it pins the measured shape.
- `lattice::p6_validate_over_the_shipped_pack` asserts AC-104's totality with a non-zero contexts-checked per family.
- `lattice::zero_packs_is_refused_not_reported` pins the shape `lattice.py --packs <path>` exhibits. Selecting zero
  packs is a typed `Err`, never a report.

**AC-101's and AC-102's kill lines, read as D21-47 V3 took them.** V3: *"The threshold applies to the registered pack
set, with the pack count reported (SM-027). The installed pair is reported with its count and does not fire the kill
line; its unreachability is Phase-1 scope by design (ADR-0011, E7)."*
- **The registered set is thresholded.** The thresholds (AC-101 `>5%` unsatisfiable, AC-102 `>20%` dead) are read
  over the eight pre-registered packs at `pack-default` rigor. That is the reading the kill lines were
  pre-registered against. Checkpoint 8b section (7) parses both figures from the golden it has just re-derived from
  the Python, and exits 1 naming the AC if either line fires. The twelve-pack figures are reported beside them.
- **The installed pair is reported, not thresholded.** `lattice::p6_lattice_over_the_shipped_pack` prints its
  unsatisfiable and dead counts, and section (6) echoes them with `packs=2` and the label *not thresholded (V3)*.
  Two packs of twelve select from a 102-activity catalog, so a dead fraction above 20% is the likely reading
  *[inferred]*. Under V3 it is recorded, and it is never quoted as a pass of AC-102, nor replaced by the twelve-pack
  figure.
- **AC-103 is not a rate.** Its zero tolerance holds over the pair too, and ties and hard cycles are 0 there.

**CI.** The tests run in `cargo test --workspace` on both OS legs. `ci.yml` also gains one named step, shaped like
the `rk-ports conformance bodies` step. It runs
`cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact`, asserts `rc`, and then
asserts `grep -qxF 'test lattice::p6_lattice_over_the_shipped_pack ... ok'` over the capture, because a filter that
matches nothing exits 0. The same commit gives the `gates` job's checkout and the no-egress job's checkout
`fetch-depth: 0` (§8b.3, V2). **Pushing is not this stage's.** D21-42 14a makes every push an operator question at
`s8j`.

**The red fixtures the thresholds need** (the Verified-by of AC-101 to AC-104), under
`crates/rk-kernel/tests/fixtures/lattice/`:
- `lattice::unsatisfiable_fixture_names_its_cell` (AC-101): a fixture catalog whose require row has no producer. The
  report names the (facet-vector × rigor) cell and the row.
- `lattice::rank_tie_fixture_is_an_error` (AC-103): two family members at one `method_rank`.
- `lattice::hard_cycle_fixture_is_an_error` (AC-103's injected cycle): two activities in one stage, each
  unconditionally requiring a kind that only the other provides. The report is an error that names both activities
  and the cycle.
- `lattice::dead_activity_fixture_is_listed` (AC-102): a fixture catalog and pack in which two of five activities are
  reachable from no pack at any rigor. The report lists both by id and computes `dead=2/5`. The kill-line function
  (`> 20%`) fires on it and does not fire on `1/5`.
- `lattice::family_hole_at_mandatory_fails_totality` (AC-104): a fixture family whose members at `mandatory` leave one
  facet vector with no member. `--validate`'s totality check names the family, the rigor and the vector.

### 8b.3 Every catalog-quality instrument over `.redkiln/` (D21-42 13a)

- **`rigor::rigor_monotonicity`** reads `.redkiln/catalog` and `.redkiln/facets.toml`. Its red end, the authored
  pre-P7-CEIL fixture catalog, is unchanged: the fixture `bisect.toml` is compared with the shipped `bisect.toml` minus
  its ceiling, and `.redkiln/catalog/bisect.toml` is byte-identical to the p9 copy up to line endings (it is not in the
  diff list). Its green end's `(families, contexts, comparisons)` triple is **re-derived from inside the test binary**
  and pinned. It is not carried over from p9, because the registries differ. Its `println!` names its catalog path
  (`rigor_monotonicity: catalog=.redkiln/catalog …`) so that the checkpoint can read the path from the binary.
- **`rigor::rigor_monotonicity_p9_historical_2026_09_13`** keeps `(15, 2060, 1729)` over `spikes/p9` read at
  `P9_PIN` through `p9_pin::root()` (below). Its doc names it a historical measurement and cites the date it was
  taken. The working tree's `spikes/p9` has not changed since `dddcdbd`, so the pinned bytes are the ones today's test
  reads.
- **`review::ac303_confirm_step_survives_escalation`** is re-pointed to `.redkiln/`. It keeps its floors.
- **`quality_block::ac527_empty_set_terminal_condition`** is re-pointed, per §8b.4.
- **`memo::ac018_live_verb_may_not_memoise`** reads `.redkiln/catalog` (D21-43 N26). After §8b.5's repair it asserts
  zero violations over 102 activities, with a non-zero count of steps walked, and prints
  `ac018: catalog=.redkiln/catalog activities=102 steps=<n> violations=0 registry=tests/fixtures/registries/verbs/verbs-live.toml`.
  The last field says that the verb registry is a FIXTURE: `.redkiln/` ships none until `s8c`. Its three fixtures are
  unchanged. Its name is kept, because `s8c` re-proves it by that name.
- **`memo::ac018_frozen_p9_catalog_is_red`** keeps today's pins over the p9 catalog at `P9_PIN`: 19 violations,
  every one `per_step_memo_declaration`, across 14 activities. It is a control over a frozen tree, and its name says
  so.
- **Every re-pointed test asserts a non-zero count walked** (activities, families, contexts, or check steps
  evaluated) **before** it asserts its verdict. A green verdict over zero is Halt 8's shape.

**The frozen p9 subject is read at its pinned commit, never from the working tree (D21-47 V2).** V2: *"Every test
pinning a historical `spikes/p9` measurement reads the p9 tree at its pinned commit via git, never the working
tree."* The pin is `P9_PIN = ca8f3874ed9185b419b00b21b33713ba128de217` (`ca8f387`), the commit at which this file's p9
measurements were taken. `spikes/p9` last changed at `dddcdbd`, so its bytes there equal the working tree's at stage
open. Five tests pin such a measurement:
- `rigor::rigor_monotonicity_p9_historical_2026_09_13`;
- `memo::ac018_frozen_p9_catalog_is_red`;
- `quality_block::ac527_frozen_p9_catalog_is_red`;
- `lattice::p6_lattice_parity_over_the_p9_tree` and `lattice::p6_validate_parity_over_the_p9_tree`.

How they read it:
- **The helper** is a new module, `crates/rk-kernel/tests/it/p9_pin.rs`, mounted by a `mod p9_pin;` line in
  `tests/it/main.rs`. It declares `const P9_PIN: &str = "ca8f3874ed9185b419b00b21b33713ba128de217";` as one whole
  line, and this is the only declaration of it in `crates/`.
- **What it does.** `p9_pin::root()` returns the materialised `spikes/p9` directory, and
  `p9_pin::materialise_at(sha, rel)` is the same function over any commit and path. It materialises once per test
  process, under a `OnceLock`, into a fresh directory under `env!("CARGO_TARGET_TMPDIR")`. It lists
  `git ls-tree -r -z <sha> -- spikes/p9` and writes each blob from `git cat-file blob <sha>:<path>`. `cat-file blob`
  applies no smudge, so the bytes are the blobs' own.
- **Git goes through `rk_git::git_exec::run`**, the one wrapper (ADR-0077; CI's `Command::new("git")` lint).
  `rk-kernel` already takes `rk-git` as a dev-dependency. The calls pass `-c safe.directory=*`, so that the no-egress
  container job can read a checkout that its user does not own.
- **It refuses rather than falls back.** If the commit is not in the clone (`git cat-file -e <sha>^{commit}` fails),
  it panics naming a shallow checkout. It never reads the working tree instead.
- **`p9_pin::reads_the_object_store_at_the_named_commit_not_the_working_tree`** proves the helper reads the object
  store. At `b66ba53` (`b66ba539791175fa5f559e12c31f964a4d1bdbeb`), `spikes/p9/catalog/threat-model.toml` carries
  `applicable_if = "auth or data_migration or external_dep"`. The working-tree copy carries
  `applicable_if = "data_migration or external_dep"`, because Step 7e removed the term [measured 2026-09-24]. The
  test asserts that the materialised file carries the first line and that the working-tree file does not, so a
  helper that read the working tree fails it. At `P9_PIN` the working tree and the pin agree, so no other test can see
  the difference (mutant M28).
- **CI.** `actions/checkout@v4` fetches one commit by default, and a depth-1 clone holds no `P9_PIN`. So the
  `gates` job and the no-egress job both gain `with: fetch-depth: 0` on their checkout step, in the same commit as the
  helper. Pushing is `s8j`'s, so whether the pinned `rust:` image carries `git` is first measured there *[inferred: it
  is built on `buildpack-deps`, which ships git]*. If it does not, the helper's refusal names the missing binary.
- **What this closes.** The story lane's candidate, hardening `spikes/p9/packs/spike.toml` (D21-41 f, `s8j`), edits
  the working tree. None of the five tests reads the working tree, so the lane's whole-suite verify cannot go red on a
  byte-pinned p9 measurement. That is V2's purpose, and it resolves `s8j`'s story-breakage blocker.

**The other readers of `spikes/p9/` are inventoried and not re-pointed.** [measured 2026-09-24 at `fc80687`: the code
lines of `crates/**/*.rs` that name `spikes/p9`.] None of them pins a historical measurement. Each is a registry
fixture or a live property of the spike tree:
- **`spikes/p9/facets.toml`, read as a facet-registry fixture.**
  - Where: `rk-kernel`'s `src/select.rs` (a unit test);
    `tests/it/{facet_registry,selection,advisory,dag,deviation,execute,fsm,lane_order,producer,replay,resume}.rs`; and
    `rk-adapter-sqlite`'s `tests/it/replay.rs`.
  - `facet_registry.rs` and `selection.rs` name it `SHIPPED_FACETS`. That name is inaccurate, because
    `.redkiln/facets.toml` is the shipped registry. Re-pointing them is not D21-42 13a's, and this stage does not do
    it. The D-entry records the name as an open item.
- **`spikes/p9/artifacts.toml`:** `rk-adapter-toolchain-rust`'s `tests/it/pack.rs`.
- **`spikes/p9/packs/`:** `review.rs`'s `shipped_packs()`, which AC-300 and AC-301 iterate. It is a live property (no
  p9 pack cascades, and `bug-standard` is strict). A story edit that broke it would be a real regression, which the
  lane's verify should catch.
- **`review.rs`'s `SHIPPED_CATALOG` and `SHIPPED_FACETS` constants** belong to `ac303`, and this stage re-points both
  to `.redkiln/`.

Checkpoint 8b section (2b) guards the split. No code line in the four files holding the five tests names a
`"spikes/p9` literal, and `P9_PIN` is declared once.

**Walks over `.redkiln/` (D21-44).** A run worktree at `.redkiln/worktrees/<item>` is a full copy of the repository.
The Rust instruments here read named subdirectories of `.redkiln/` flat. `Catalog::from_dir` makes one
`std::fs::read_dir(dir)` call and does not recurse, and the lattice reads `catalog`, `packs`, `facets.toml` and
`artifacts.toml` by name. So nothing in Rust walks `.redkiln/` as a whole. The checkpoint's two recursive greps do.
They walk `.redkiln/` with `--exclude-dir=worktrees`, and section (4) proves the exclusion with a planted control in a
scratch tree shaped like `.redkiln/` (the walk functions take the root). It asserts the `.gitignore` row with
`git check-ignore`, which needs no file to exist. An unexcluded walk of the real `.redkiln/` would descend into run
worktrees, with their junctions and locked files, and so it is never run. **The exclusion is scoped to `.redkiln/` alone.** `grep --exclude-dir` matches a directory's base name at any
depth, and `crates/` holds three fixture directories with that name:
`crates/rk-adapter-sqlite/tests/fixtures/worktrees` and the two `dot-git/worktrees` fixtures under
`crates/rk-cli/tests/fixtures/doctor/` [measured 2026-09-24 at `a6349b6`, `find crates -type d -name worktrees`]. One
grep over `crates/ .redkiln/` with the flag would skip all three without a word. So `crates/` is walked whole, in a
separate call. The widened Checkpoint 8 form that `s8j` adopts carries the same scoping (§8b.7).

The rewrites in §8b.5 change `expr` and `inputs` on `check` steps and add no step. One step moves: D21-47 V4 puts
`adversarial-review`'s `verdict` after `record`, and a reorder takes no step into or out of the guaranteed set.
`evidence()` reads the guaranteed step set, so the monotonicity figures should not move between the re-point and the
rewrites *[inferred]*. They are re-read after the rewrites anyway, and any movement is recorded with its cause.

### 8b.4 The d2-form lint, and AC-527 brought back to ADR-0080 decision 2 (D21-41 h)

**The reading is settled by records** (packet (h), option A):
- ADR-0103 d3: *"A value that does not match the bare class must be quoted"*. The quoted spelling is the only
  well-formed one.
- ADR-0080's Context names that quoted string: *"The catalog's policy check is
  `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`"*.
- design §1.2's row *"P7's policy predicate"* says *"(quoted-literal form per ADR-0103)"*. The packet places
  this sentence at design §6.6. It is not there: `grep` finds it once, in design §1.2 [measured 2026-09-24].

ADR-0092 d2 item 1 sets the direction: *"a criterion or a runbook step that contradicts an accepted ADR is the
defect; the ADR is not"*. AC-527's THEN, *"or any predicate whose truth over an **empty** findings set is `true`"*,
refuses what d2 permits, since bullet 1's own form *"no finding at or above threshold T survives re-execution"* is
true over an empty set.

**The amended rule** (`rk_kernel::quality_block`). The subject is **every** `check` step. That is the path-free
subject which disposition (i) licenses, and D21-42 11a took (i) by rewriting `adversarial-review`. The rule refuses
two things:
1. **the named form**, `findings.none(evidence in [...])` in any quoting and with any member list, alone or as a
   conjunct required for truth. It is matched on the **parse tree**, not the string;
2. **any predicate that a single-class witness set falsifies**. Each witness finding carries only an `evidence`
   field, in one blocking class, `Reproduced` or `MutationSurvived`, one witness per class. This is per class,
   because the packet's defect (3) is measured: a joint two-member witness does not refuse
   `findings.none(evidence in ['Reproduced'])`, since a `{MutationSurvived}` set satisfies it.

Operands other than `findings` bind as they need to, so that `X and <falsified>` evaluates `false` under ADR-0121's
absorption. **Severity-only predicates read no `evidence` and stay lawful.** Re-pin the counts from inside the test
binary. The witness change moves them, so they are not inferred.

**Fixtures, one per branch** (a fixture per function is not a fixture per branch), under
`crates/rk-kernel/tests/fixtures/activities/`:

| fixture | expr | must | discriminates |
| --- | --- | --- | --- |
| `terminal_condition_is_findings_none.toml` (fixture 1, kept) | the quoted two-member form | FAIL | both branches |
| `terminal_condition_is_findings_all.toml` (fixture 2, **recast**) | `findings.all(not (evidence in ['Reproduced', 'MutationSurvived']))` | FAIL | the witness **alone**: it contains no `none(`, so a string or quantifier matcher cannot catch it |
| `terminal_condition_names_any_member_list.toml` (new) | `findings.none(evidence in ['NarrowedPin'])` | FAIL | the named form **alone**: no blocking-class witness falsifies it |
| `terminal_condition_is_one_member_conjunct.toml` (new) | `check.failed == 0 and findings.none(evidence in ['Reproduced'])` | FAIL | the conjunct case. This is the shipped `user-story-ac` row, verbatim |
| `terminal_condition_is_severity_only.toml` (new) | `findings.none(severity in ['blocking'])` | LOAD | that the rule does not over-reach. The old fixture 2 `findings.all(severity != 'blocking')` is the same predicate, which is why it is recast |
| `terminal_condition_names_a_verify_result.toml` (fixture 3, kept) | unchanged | LOAD | unchanged |

**Tests, red first.** Author each one, and watch it fail for the named reason before the rule changes:
- `quality_block::d2_per_class_witness_refuses_one_member_rows`;
- `quality_block::d2_named_form_refuses_any_member_list`;
- `quality_block::d2_severity_only_predicates_load`;
- the re-pointed `quality_block::ac527_empty_set_terminal_condition`, which over `.redkiln/catalog` asserts
  `refused == 0` **after** §8b.5 and a non-zero count of check steps walked. It prints
  `ac527: catalog=.redkiln/catalog checks=<n> refused=<n> indeterminate=<n>`;
- `quality_block::ac527_frozen_p9_catalog_is_red`: the p9 catalog at `P9_PIN` (§8b.3, V2) stays RED under the amended
  rule, with its count re-measured and pinned. It is a control over a frozen tree, and its name says so.

Rust tests read predicates from fixture files (`include_str!`) and do not spell them as string literals. The
existing literals in `tests/it/quality_block.rs` are why §8b.7's guard excludes that one file. No second file joins
it.

**The register amendment** is dated and in place under ADR-0092 d2 item 2, and it is not an erratum:
- **AC-527's THEN** becomes d2's rule, *"lint fails naming the activity and the predicate"*, applied to the two
  refusals above on any `check` step. The clause *"or any predicate whose truth over an **empty** findings set is
  `true`"* and *"never on the absence of findings"* are struck, with the amendment's date and D21-41 (h) beside them.
- **Its Verified-by** names the six fixtures above in place of *"a red fixture with `findings.all(...)` over an empty
  set, which must also fail"*.
- The `rk-catalog` CLI surface stays **ARMED**, and the amendment says so. A kernel test over `.redkiln/catalog` is
  not `rk-catalog lint`.
- Both module docs lose their *"An erratum reconciling … is OWED"* sentence. They say instead that the register was
  brought to d2, and give the date.

### 8b.5 The seven rows, versioned, changelogged, and re-linted (D21-41 h; D21-42 11a, 11b)

**Every replacement must meet three conditions, and each is a named test:**
- **(a) It is a d2 bullet-1 or bullet-2 form.** Bullet 1 is *"no finding at or above threshold T survives
  re-execution"*. Bullet 2 is *"a bounded loop-back count, a re-execution outcome, or a delta against the previous
  iteration's findings — chosen so that a reviewer doing its job can terminate the block"*. D21-43 N18 reads bullet 2's
  dash list *"as illustrative"* for the (D) rows, and the D-entry records that reading. The refusal lint of §8b.4 is
  the floor.
- **(b) It is not a constant function of its operands' domains.**
  `quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity` evaluates each new `expr` across its
  bindings and requires both truth values. The design's golden constraint 15: *"A policy predicate that is a constant function of
  its arm carries no information and may not terminate anything."*
- **(c) For a loop-back row, it reads a quantity the loop varies.**
  `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` requires that every binding the `expr` reads comes
  from the `on_false.to` target, or from a step reachable from that target through a chain of `${<step>.outputs…}`
  input references. An `artifact:` or `toolchain:` input breaks the chain. (b) cannot see a quantity that is variable
  in the grammar and fixed in practice. (c) can. **The chain is transitive** because `instrumented-trace-analysis`'s
  loop-back target `instrument` reaches `confirm` only through `collect` (`changes = "${instrument.outputs.files}"`)
  and `analyse` (`trace = "${collect.outputs}"`). A one-hop rule would refuse the row that N18 (C) prescribes.
  **Whose variation counts (D21-47 V5).** ADR-0080 d2 says *"something the **reviewer** varies"*, and (c) reads a
  quantity *the loop* varies. D21-47 V5 records the operator's reading of d2: *"a re-executed outcome that the
  reviewer's hypothesis drives satisfies "something the reviewer varies""*. (c) is read under V5, and `root-cause`'s
  derivation below is confirmed under it.

| row | step | version | current `expr` (quoted) | replacement | status |
| --- | --- | --- | --- | --- | --- |
| `root-cause` | `policy` | 4 → 5 | `"findings.none(evidence in ['Reproduced', 'MutationSurvived'])"` | inputs `{ report = "${confirm.outputs}", findings = "${adversary.outputs?}" }`, `expr = "report.failed > 0"`. The quantity is bullet 2's re-execution outcome: `confirm` re-runs `${hypothesise.outputs.discriminating_test}`, which `hypothesise`, the loop-back target, varies | derived here |
| `adversarial-review` | `verdict` | 3 → 4 | `"findings.none(evidence in ['Reproduced', 'MutationSurvived']) and (rigor < mandatory or findings.none(evidence in ['NarrowedPin']))"` | D21-42 11a verbatim: *"no blocking finding survives re-execution"*, read from `review::confirm`'s outcome. **D21-47 V4 moves `verdict` after `record`**, because `record` produces the outcome `verdict` reads. The step order becomes `clone`, `review`, `override`, `record`, `verdict`, `emit`. `verdict`'s input binds `${record.outputs}` for this activation, and the `expr` is over its block set (`Confirmed::block_set()`). `on_false = "continue"` is unchanged. The reorder rides the same ADR-0072 edit (one bump, and one changelog row naming both), and the lattice re-run below covers it | ledger-specified; order D21-47 V4; binding spelling measured, see below |
| `differential-debugging` | `policy` | 2 → 3 | the same two-member form | **(C).** `confirm`'s inputs become `{ selection = "${compare.outputs.discriminating_test}" }`, replacing `artifact:ReproCase.command`, which the loop does not vary. `policy` becomes inputs `{ report = "${confirm.outputs}", findings = "${adversary.outputs?}" }`, `expr = "report.failed > 0"`, as `root-cause`'s. `compare` is the loop-back target | D21-43 N18 (C); the field is owed (below) |
| `instrumented-trace-analysis` | `policy` | 2 → 3 | `"cause.confidence == 'high' and findings.none(evidence in ['Reproduced'])"` | **(C).** `confirm`'s inputs become `{ selection = "${analyse.outputs.discriminating_test}" }`. `policy` keeps its `cause` conjunct and binds `report = "${confirm.outputs}"`, giving `expr = "cause.confidence == 'high' and report.failed > 0"`. The same bump carries `collect`'s `memo` repair (N26) | D21-43 N18 (C); the field is owed (below) |
| `threat-model` | `policy` | 1 → 2 | `"findings.none(evidence in ['Reproduced'])"` | **(A), after §8b.6.** With T = `blocking`, `expr = "findings.none(severity in ['blocking'])"`. That is bullet 1's severity-only form, which §8b.4's rule loads. The draft's reason for not reading "survives re-execution" here stands: there is no tool step, and `review::confirm` gives `Unverifiable::CannotRun` for a repro with no `--` selection (ADR-0162 d4). **If §8b.6 settles no admissible T, (A) has nothing to bind and the stage STOPS for the operator.** That is a real free residual | D21-43 N18 (A); waits on §8b.6 |
| `user-story-ac` | `policy` | 2 → 3 | `"check.failed == 0 and findings.none(evidence in ['Reproduced'])"` | **(D).** `expr = "check.failed == 0"`, inputs unchanged. `check` is `self-check`'s `SelfCheck`, and `self-check` reads `${draft.outputs}`, the loop-back target. It meets (a) only on the reading N18 records | D21-43 N18 (D) |
| `vertical-slicing` | `policy` | 1 → 2 | `"check.failed == 0 and findings.none(evidence in ['Reproduced'])"` | **(D)**, as `user-story-ac`. `check` is `coverage`'s `SelfCheck`, which reads `${slice.outputs}`, the loop-back target | D21-43 N18 (D) |

**N18 supersedes D21-42 11b's wording for these five rows, and the reason is measured.** D21-43's preamble says:
*"Only two of the seven ADR-0080 rows have a lawful re-execution quantity. For the other five, 11b's wording would
have made the rewritten check constantly true."* `root-cause` and `adversarial-review` are the two. The five take
N18's letters as the table shows. Nothing here re-opens (B), which N18 did not take.

**The (C) rows need a field that no type carries yet.** `RootCause` is the output kind of both investigator steps
(`compare` and `analyse`). Its stand-in in `spikes/p9/schemas.toml` is `fields = ["cause", "contributing_factors",
"confidence", "summary"]`. Only `HypothesisLog` carries `discriminating_test` (`fields = ["status", "entries",
"discriminating_test", "root_cause", "summary"]`) [measured 2026-09-24 at `a6349b6`]. The catalog edit is this
stage's. Typing the field is `s8h`'s, whose §8h.4 transcribes every reached kind from that stand-in: *"Its field names
and list flags are the transcription source."* **`s8h` carries it** (D21-47, cross-file finding 5). Its
transcription adds `discriminating_test` to `RootCause` and to the stand-in, with a test. This stage cites that and does
not type the field. Until `s8h` lands, the resolver refuses `confirm`'s binding by name (ADR-0136) and never runs an
absent selection. The D-entry records the hand-off to `s8h`.

**`adversarial-review`'s binding.** Today the review pass runs in `rk-cli` after the activity commits (ADR-0162
d3/d6), so no step output carries `Confirmed` during the walk.
- **V4 settles the order.** `verdict` reads what `record` produces, so it runs after `record`.
- **Resolution is `s8d`'s.** Resolving `${record.outputs}` at walk time belongs to `s8d`'s resolver (D21-41 d), and
  `s8d`'s walk table states V4's reorder.
- **The spelling** is fixed in the commit that also keeps `conformance::shipped_pack_loads` green. If no spelling
  loads without a resolver change, the spelling goes back to the operator as a new question: V4 ruled the order, and
  neither N18 nor V4 ruled the spelling.
- **Until the binding resolves,** ADR-0136 refuses the step by name. The check never evaluates an absent value as
  true.
- `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` also asserts the step order, `record` before
  `verdict` (mutant M29), and Checkpoint 8b section (3d) reads it from the file.

**Nothing here claims calibration.** Not in the rows, the changelog, or the tests. ADR-0162 forbids *"any claim about
the review gate's calibration, in any document, record, commit message, fixture or test name"*.

**Three more catalog edits are owned here (D21-43 N26, N13, N25)**, beside the seven rows. Each is an ADR-0072
edit: it bumps `version` and takes a changelog row.

1. **AC-018's repair (N26).** Delete the `memo    = false` line from each of the 19 steps in 14 activities. ADR-0097
   decision 3 withdrew the per-step field, and `live_verb_memo_rule`'s clause 2 fires on any declaration. The versions
   move as follows: `blue-green-deploy` 2 → 3, `failover` 2 → 3, `feature-flag-kill` 3 → 4,
   `flag-progressive-rollout` 2 → 3, `incident-command` 1 → 2, `instrumented-trace-analysis` 2 → 3 (one bump, shared
   with its N18 edit, and one changelog row naming both), `migration-dry-run` 2 → 3, `migration-expand-execute`
   2 → 3, `rate-limit-shed` 2 → 3, `rollback-mitigation` 2 → 3, `rollback` 1 → 2, `rolling-deploy` 2 → 3,
   `rolling-restart` 2 → 3, `slo-monitoring` 2 → 3 [versions measured 2026-09-24 at `a6349b6`].
   **What the deletion hands on.** The 19 steps run twelve verbs: `smoke`, `failover`, `metrics`, `flag_set`,
   `flag_rollout`, `trace_collect`, `migrate_dry_run`, `migrate`, `rate_limit`, `rollback`, `restart` and
   `slo_query`. Once the key is gone, a step's liveness lives only on the verb registry's `live` flag (ADR-0059), and
   those rows are `s8c`'s (§8c.4). Each deleted `memo = false` was its author saying that the verb must not be
   memoised. So the changelog row lists the twelve verbs for `s8c`. A registry row for any of them without
   `live = true` would make memoisable a step that its author marked otherwise. That is `s8c`'s to settle, not
   this file's.
2. **tdd's `red-verify` (N13):** *"A catalog edit to the existing `[*].tests_added` projection"*. The step's inputs
   become `{ selection = "${red.outputs[*].tests_added}" }`. The trailing comment `# list of ImplReport; tool flattens
   tests_added (schema gap: no flatten)` goes, because the projection is the flatten. ADR-0035: *"Fan-out outputs are
   projected with `${step.outputs[*].field}`."* `red` fans out (`for_each = "artifact:Spec.acceptance_criteria"`), and
   `bdd-outside-in` and `technical-spike` already use this form. `tests_added` is list-valued. D21-46 flagged the
   projection's value over it for the operator's flip lines: *"ADR-0035's `[*].field` over a list-valued field is
   read as **concatenation in element order** (s8d R2; N13 assumes it)"*. D21-48 X5 settled it (*D21-48
   (2026-09-24)*): *"Concatenation in element order. Nothing is flattened implicitly anywhere else."* `s8d` R2
   records it. `tdd-red-green-refactor` 2 → 3. Resolving the
   projection at run time is the job of `s8d`'s resolver. This edit is the catalog text only.
   `pack_validation::tdd_red_verify_projects_tests_added_per_adr_0035` asserts three things: the step's `selection` is
   exactly that projection, its source step carries `for_each`, and `ImplReport`'s stand-in lists `tests_added`
   (`lists  = ["files", "tests_added"]`).
3. **`sca-scan`'s `unrated` arm (N25):** *"`sca-scan` requires `unrated == 0`"* or goes to `scan.accept-findings`.
   `clean`'s `expr` becomes `"report.critical == 0 and report.high == 0 and report.unrated == 0"`. The `exception`
   gate's `when` becomes `"${scan.outputs.critical} > 0 or ${scan.outputs.high} > 0 or ${scan.outputs.unrated} > 0"`,
   so an unrated advisory goes to the operator and is never passed. `sca-scan` 1 → 2. **Ownership.** D21-43 N25
   took the rule, so the catalog edit is made here with the other ADR-0072 edits, and both neighbours cite it.
   *(re-quoted 2026-09-24, final consistency pass: this sentence quoted `s8c`'s superseded draft.)* `s8c` says
   *"`sca-scan`'s `unrated` check is `s8b`'s catalog edit, and it is cited here, not re-decided"*, and its record V
   makes `unrated` a required `ScanReport` member. `s8g` says *"The edit is `s8b`'s catalog edit, beside its other
   ADR-0072 edits"*, and its producer walk asserts that an unrated advisory opens the gate.
   `pack_validation::sca_scan_blocks_on_unrated_or_routes_to_accept_findings` evaluates both predicates
   through `rk_kernel::predicate` over three reports: all zero, `unrated = 1` alone, and `high = 1` alone. `clean`
   must be true, and `when` false, for the first report only.

**ADR-0072, executed.** *"Edits to shipped activities made for catalog coherence bump `version` and are listed in
the pack changelog. The P6 lattice is the test that such an edit did not break reachability."*
- Each row's `version` moves as in the table and the list above: twenty-two activities, one bump each.
- `.redkiln/CHANGELOG.md` is created. Nothing names the file, so the path is chosen here and said. It gets one row
  per edited activity: `<id>@<new version>`, the date, and the ledger line (D21-41 (h), D21-42 11a, or D21-43 N18,
  N26, N13 or N25). Each row quotes the old and new text. `root-cause`'s row adds the line *"detached from F37's pinned
  artefact `306b58ba7504` (v4); F37's measurements are of v4"*, the memo rows name their verbs, and
  `adversarial-review`'s row names V4's reorder beside its d2 rewrite. The changelog quotes the old predicates on
  purpose, which is why Checkpoint 8b section (4) excludes `.redkiln/CHANGELOG.md` by path.
- `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` pins the seven `(id, old, new)` triples and
  requires a changelog row for each. It also asserts V4's order in `adversarial-review`: `record` precedes `verdict`.
- `quality_block::adr0072_s8b_repairs_are_versioned_and_changelogged` does the same for the sixteen triples of the
  three further edits, `instrumented-trace-analysis`'s shared bump included.
- **Then the Rust lattice is re-run over `.redkiln/` with a non-zero pack count.** That is ADR-0072's reachability
  test, and §8b.2 ports it first for this reason. Any figure it moves is recorded with the edit that moved it.

### 8b.6 The severity vocabulary and ADR-0080 d3's threshold T, settled on a measurement (D21-42 11c)

ADR-0080 d3: *"Pick the threshold when severity has a settled vocabulary and a destination for advisory findings;
not before."* The destination exists: ADR-0082, *"The Advisory sink is a first-class destination, not a discard"*. The vocabulary does
not. Four measured facts frame the problem:
- **39** shipped `expr` rows in 39 files read `severity`.
- `spikes/p9/schemas.toml` declares `fields = ["ac", "claim", "evidence", "severity", "files", "repro"]`.
- `rk_types::artifact::findings::Finding` has **no** `severity` field.
- **188 of 188** F37 findings carry none. Their keys are `ac, claim, evidence, files, rationale, repro`
  [measured 2026-09-24 over `redkiln-corpus` `results/p7.jsonl`].

Blocking is computed by `review::policy`'s table from class, weight and re-execution. That module says of one row:
*"the row for `narrowed-pin` is the operator's severity call, not a measurement"*.

**The measurement, pre-registered in full before a single label exists.** It is written as `spikes/METHOD-SEV.md`
and committed **before** the sheet is generated. The spike id is **S-SEV**.

- **Corpus** (D21-43 N18, which closed §8b.1 question 1): F37's **188** findings from 33 adversary dispatches, read
  through the object store, `git -C "$CORPUS_REPO" show "$CORPUS_SHA":results/p7.jsonl`, as AC-464 requires,
  never from a working tree. **The six E3 findings are excluded** from the pre-registered analysis, as N18 took
  it. `e3-severity-annotation-ryan.md` is *"Filled by the model at Ryan's direction, 2026-09-04. NOT BLIND."*,
  and its correction carries *"countersign: Ryan Britton, 2026-09-11"*. The operator has read a filled sheet, so his annotation of the preserved
  BLANK could not be blind. If he labels it anyway, it is reported as NOT BLIND, and E3-SEV-BLIND stays open.
- **Rater: the operator, alone.** Real dispatch is not permitted at this stage. A model-filled sheet would also repeat
  E3-SEV's forfeit: the same rig's model is not an independent rater. METHOD-SEV opens with a dated attestation that
  the operator has not read F37's findings individually.
- **Blinding.**
  - A committed script, `spikes/sev/make-sheet.py`, which makes no dispatch, renders each finding's `claim`,
    `rationale`, `files` and `repro` (command, observed, expected) under a random id from a recorded seed.
  - It **masks** `evidence`, arm, rigor, item, dispatch and the item's root-cause correctness.
  - The id → finding key is written to a sealed file whose sha256 is recorded in METHOD-SEV before labelling.
  - **Stated limit:** a `rationale` often reveals whether the reviewer ran something, so the class is partly
    inferable. The mask removes the label, not the evidence.
- **The question**, verbatim from E3's sheet, so that the two are comparable: *"should this finding have blocked the
  item?"*. The answers are `BLOCKING`, `ADVISORY` or `CANNOT-TELL`, against the reviewer card's quoted standard.
- **Re-label for intra-rater agreement.** After at least 48 hours, 38 findings (20%, chosen by a recorded seed,
  reshuffled and re-id'd) are labelled again blind to the first pass.
- **What counts, and the n each criterion needs, fixed before the data (golden constraint 10):**
  - **A1, vocabulary usability.** `CANNOT-TELL` is at most 10% of 188, with its Wilson 95% interval reported, **and**
    intra-rater Cohen's κ on the 38 is at least 0.61. The 0.61 threshold is the Landis–Koch "substantial"
    convention, chosen here before the data and said to be a convention. If either fails, the vocabulary is **NOT
    SETTLED by this measurement**. The record says so under that fault name, and the stage STOPS for the operator.
  - **A2, severity versus evidence class.** Tabulate the execution classes (`reproduced` 118 and
    `mutation-survived` 29, 147 in all) and the others (`advisory` 22 and `narrowed-pin` 19, 41 in all) against
    `BLOCKING` and `ADVISORY`, with each discordant proportion and its Wilson interval. **If severity is a
    deterministic function of class**, meaning zero discordant findings in both directions, then a severity
    threshold *is* the evidence predicate renamed, and T over severity is **inadmissible**.
  - **A3, termination variability, which is the design's golden constraint 15, measured.** For each of the 33 dispatches, ask
    whether it carries no `BLOCKING`-labelled finding, and report `k/33` with its interval. **`k = 0` or `k = 33`
    reproduces F37's constant-function result, and T = `blocking` is then inadmissible.** At `n = 33` this
    criterion can separate constancy from non-constancy, which is all it is asked to decide. It cannot estimate a
    rate precisely, and the record does not quote one as a rate.
  - **Report only, and powerless, so labelled:** whether `k` tracks F37's per-item root-cause correctness (10 items).
- **What the record fixes, by branch:**
  - **A1 passes, and A2 and A3 show variability:** the vocabulary is `blocking | advisory`, and T = `blocking`.
    `threat-model`'s (A) edit in §8b.5 then binds it.
  - **A1 passes, and A2 or A3 fails:** the vocabulary is fixed, and T is *"no admissible severity threshold on this
    evidence; bullet 2 governs"*. That is a settled T, not a deferral. Because d2 bullet 1 needs a T and this branch
    settles none, **the record supersedes ADR-0080 d2 bullet 1 in part** (a severity threshold, on this evidence). It
    does not supersede nothing. `threat-model`'s (A) then has nothing to bind, and §8b.5's STOP applies.
  - **A1 fails:** STOP.
  - **In every branch in which the record is written, it names the source and owner of the `severity` field.** 39
    shipped `expr` rows read that field, and `rk_types::artifact::findings::Finding` has none. The record carries a
    paragraph that opens `**The severity field's source and owner.**`.
    - **The owner is `s8h`** (D21-47, cross-file finding 4). `s8h` adds `severity` to `rk_types` `Finding`, typed by
      this record's vocabulary, with tests.
    - The paragraph also names the step that fills the field: the reviewer's output, `review::policy`'s table, or
      something else. The record does not add the field.
  - **What the 39 rows do until then, as the evaluator behaves** (`rk_kernel::predicate`'s `quantify` and its path
    lookup; review M5). ADR-0136 governs `when` guards and does not refuse these rows at bind time.
    - A `findings.none(severity …)` over an **empty** findings list is `Tri::True`, and `severity` is never read.
    - Over a non-empty list whose findings carry no `severity`, the element lookup falls through to the context and
      returns `EvalError::UnknownField("severity")`.
    - So an empty list passes the row and a non-empty one errors. No non-empty list is read as absent-means-true.

    The record says this, and the row is evaluated in `s8d`'s resolver (R2).
  - **In every branch**, if the labels contradict ADR-0074's `narrowed-pin` row, the record **reports** the
    contradiction and does not move the row. A Decision sentence of an accepted record moves only by supersession
    (ADR-0092 decision 1), on the operator's word.
- **Where the artefacts live.** The filled sheets (the first pass as `results/sev-f37-labels.md`, which Checkpoint 8b
  section (9) reads by that name), the re-label and the scored result are committed to `redkiln-corpus` as a new
  commit, without moving `CORPUS_PIN`. That is O-9's precedent, and AC-465 bars tracked
  spike results under `spikes/results/`. The record names that commit's full sha on one line of the form
  ``**S-SEV labels commit:** `<40 hex>` ``, which Checkpoint 8b section (9) reads, and it never names the corpus
  checkout's HEAD.

**The record.** A new ADR at the next free id at writing, `status: proposed`, with `evidence: ["S-SEV", "F37"]`,
ADR-0085's `### Option — ` headings (D21-42 24a), and a **What this forbids** block.
- In the first branch it supersedes nothing, because ADR-0080 d3 contemplates exactly this record and forbids none
  of it. In the second branch it supersedes ADR-0080 d2 bullet 1 in part, as above.
- It flips only on the operator's dated ledger line.
- **Its id is recorded at write time.** The D-entry states `S-SEV record: ADR-<nnnn>`, and Checkpoint 8b section (9)'s
  `SSEV_ADR=` literal is written by a dated edit in the same commit. D21-47: *"Records are located by the id recorded
  at write time"*.
- It carries dated notes on every design row that reads *"plus a settled severity vocabulary"*, located by that
  phrase. At `fc80687` there are six: two in design §0.1, and one each in design §1.2, design §7.5, design §7.10 and design §10.4
  (review N1).
- It makes no claim about the review gate's calibration (ADR-0101 clause 5). It cites that clause by number and does
  not quote its sentence, so that section (11)'s widened walk can read the record without tripping on the rule's own
  prose.

### 8b.7 The D-entry, and the guard form `s8j` adopts

**The stage's ledger entry** is the next free D21 id at writing. It **closes D21-2's RAISED reading**: an `expr` on a
`kind = "check"` step that carries `on_false` is the terminal condition ADR-0080 d2 forbids, settled by ADR-0103 d3,
ADR-0080's Context and design §1.2's *"P7's policy predicate"* row. It also records:
- `adversarial-review`'s disposition (i), taken by D21-42 11a;
- that the one-member rows are refused under the forbids bullet's `findings.none(evidence in [...])`, with the
  disclosure the packet asked for: this **extends F37's measurement**, which was of the two-member predicate on one
  graph;
- D21-43's rulings on §8b.1's four questions, cited (N18, N26) and not re-taken, with the (D) rows' reading of
  bullet 2 as illustrative recorded, as N18 requires;
- `root-cause` detached from `306b58ba7504`;
- the parity figures and the Python's demotion;
- the METHOD-P amendment's number and the `surfaces` enumeration it registered;
- the Rust lattice's wall time (SM-025);
- the three further edits (N26's 19 `memo` rows, N13 and N25), and three hand-offs: the twelve verbs whose `live` flag
  is `s8c`'s; `RootCause.discriminating_test`, which `s8h` carries (cross-file finding 5); and the `severity` field on
  `Finding`, which `s8h` adds (cross-file finding 4);
- **D21-47's readings as this stage applied them.**
  - **V2**: the five p9 pins read `P9_PIN`, and the other readers of `spikes/p9` are inventoried. The inaccurate
    `SHIPPED_FACETS` name in `facet_registry.rs` and `selection.rs` is left as an open item.
  - **V3**: the kill lines are thresholded over the eight pre-registered packs, and the installed pair's figures are
    reported with `packs=2`.
  - **V4**: `adversarial-review`'s `verdict` follows `record`.
  - **V5**: condition (c)'s *"the loop"* is read as *"something the reviewer varies"*, and `root-cause`'s derivation
    is confirmed under it.
- one disclosure on N18's ruling, not a new reading: `threat-model`'s (A) form `findings.none(severity in
  ['blocking'])` drops d2 bullet 1's *"survives re-execution"*, because there is no tool step to re-execute (§8b.5);
- the Python oracle's non-determinism (`cycles()` over a `set`), the `PYTHONHASHSEED=0` it now runs under, and the
  six named divergences.

**The guard form this stage makes true.** `s8j` widens Checkpoint 8's quality-shorthand block to it. This file does
not edit `s8`. It replaces the unquoted-only pattern, keeps the `quality_shorthand` alternation, drops code comments,
excludes `.redkiln/worktrees/` (D21-44) by `--exclude-dir=worktrees` on a walk of `.redkiln/` alone, never on the
`crates/` walk (§8b.3), and excludes by path, with each exclusion's reason stated:
- `tests/it/evaluator.rs` and `tests/it/quality_block.rs` are the enforcement's own literals;
- `tests/fixtures/catalog/{root-cause,differential-debugging,instrumented-trace-analysis}.toml` are the frozen
  pre-P7-CEIL red end of `rigor_monotonicity` and AC-303;
- the four must-REFUSE activity fixtures:
  - `terminal_condition_is_findings_none`, `_names_any_member_list` and `_is_one_member_conjunct` carry the named
    form because refusing it is their purpose;
  - `terminal_condition_does_not_parse` carries the truncated prefix `findings.none(evidence in [`, because it is
    AC-527's fail-closed fixture, a terminal condition that does not PARSE and must be refused, never skipped (review
    N8);
- `.redkiln/CHANGELOG.md`, which quotes each rewritten row's old predicate as ADR-0072's record (review M1).

Comment lines are dropped by their file's own syntax only: `//`, `/*` and `*` in `.rs`, and `#` in `.toml` and
`.yml`. A Markdown `*` bullet is not a comment.

`adversarial-review` needs no by-name exclusion, because it was rewritten. Checkpoint 8b section (4) is that form,
run here, with its synthetic controls and a planted control in a scratch tree shaped like `.redkiln/`.

### 8b.8 Mutants, run from Python and never from the shell

Each mutant is run by a Python driver in the operator's scratch directory, uniquely named, because parallel agents
share one scratchpad. For each mutant, the driver:
1. reads the file's bytes;
2. asserts that the anchor occurs **exactly once**;
3. writes the mutant;
4. runs the named selection with `--test it`, module-qualified and `--exact`;
5. restores the original bytes in `finally`;
6. **asserts that the restored sha256 equals the original's**.

**A survivor blocks the stage.** It is investigated for unobservability before any test is added.

| # | file | mutation | must flip |
| --- | --- | --- | --- |
| M1 | `tests/it/rigor.rs` | the primary's catalog constant back to `"spikes/p9/catalog"` | `rigor::rigor_monotonicity` (pinned `.redkiln` shape) |
| M2 | `src/quality_block.rs` | witness set reduced to empty/absent only | `quality_block::d2_per_class_witness_refuses_one_member_rows` |
| M3 | `src/quality_block.rs` | per-class witnesses replaced by one joint `{Reproduced, MutationSurvived}` set | `quality_block::d2_per_class_witness_refuses_one_member_rows` |
| M4 | `src/quality_block.rs` | witness findings also carry `severity: blocking` | `quality_block::d2_severity_only_predicates_load` |
| M5 | `src/quality_block.rs` | named-form match deleted (witness only) | `quality_block::d2_named_form_refuses_any_member_list` |
| M6 | `src/lattice.rs` | last covering-array row dropped | `lattice::covering_array_is_pairwise_and_matches_the_python_rows`, `lattice::p6_lattice_parity_over_the_p9_tree` |
| M7 | `src/lattice.rs` | `admissible` ignores `max_rigor` | `lattice::p6_lattice_parity_over_the_p9_tree` (the dead list moves: `bisect`) |
| M8 | `src/lattice.rs` | pack selection over zero packs returns an empty report | `lattice::zero_packs_is_refused_not_reported` |
| M9 | `src/lattice.rs` | rank-tie detection removed | `lattice::rank_tie_fixture_is_an_error` |
| M10 | `src/lattice.rs` | stage order taken from the map's key order | `lattice::p6_lattice_parity_over_the_p9_tree` |
| M11 | `src/lattice.rs` | I4's *"row rigor raises, never lowers"* removed | `lattice::p6_lattice_parity_over_the_p9_tree` |
| M12 | `src/lattice.rs` | the `unknown` sweep skipped | `lattice::p6_lattice_parity_over_the_p9_tree` |
| M13 | `.redkiln/catalog/root-cause.toml` | `expr` reverted to the two-member form, version left at 5 | `quality_block::ac527_empty_set_terminal_condition` |
| M14 | `.redkiln/catalog/user-story-ac.toml` | version reverted 3 → 2 | `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` |
| M15 | `.redkiln/catalog/root-cause.toml` | `expr` replaced by `1 == 1` | `quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity` |
| M16 | `.redkiln/catalog/root-cause.toml` | `confirm`'s `selection` re-pointed at `artifact:ReproCase.command` | `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` |
| M17 | `.redkiln/catalog/differential-debugging.toml` | `confirm`'s `selection` back to `artifact:ReproCase.command`, version left at 3 | `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` |
| M18 | `.redkiln/catalog/failover.toml` | one `memo    = false` line restored under the `toolchain:metrics` step, version left at 3 | `memo::ac018_live_verb_may_not_memoise` |
| M19 | `tests/it/memo.rs` | the shipped path constant back to `"spikes/p9/catalog"` | `memo::ac018_live_verb_may_not_memoise` |
| M20 | `.redkiln/catalog/tdd-red-green-refactor.toml` | `red-verify`'s `selection` back to `"${red.outputs}"` | `pack_validation::tdd_red_verify_projects_tests_added_per_adr_0035` |
| M21 | `.redkiln/catalog/sca-scan.toml` | ` and report.unrated == 0` deleted from `clean` | `pack_validation::sca_scan_blocks_on_unrated_or_routes_to_accept_findings` |
| M22 | `.redkiln/catalog/sca-scan.toml` | ` or ${scan.outputs.unrated} > 0` deleted from `exception`'s `when` | `pack_validation::sca_scan_blocks_on_unrated_or_routes_to_accept_findings` |
| M23 | `src/lattice.rs` | `Axes::shipped()` drops `rust` | `lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged` |
| M24 | `src/lattice.rs` | `Axes::p6_registered()` gains `rust` | `lattice::p6_lattice_parity_over_the_p9_tree`, `lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged` |
| M25 | `src/lattice.rs` | hard-cycle detection removed | `lattice::hard_cycle_fixture_is_an_error` |
| M26 | `src/lattice.rs` | the dead-activity list left empty | `lattice::dead_activity_fixture_is_listed` |
| M27 | `src/lattice.rs` | the totality check skipped at `mandatory` | `lattice::family_hole_at_mandatory_fails_totality` |
| M28 | `tests/it/p9_pin.rs` | `materialise_at` copies the working tree's file instead of `cat-file blob` | `p9_pin::reads_the_object_store_at_the_named_commit_not_the_working_tree` |
| M29 | `.redkiln/catalog/adversarial-review.toml` | the `verdict` step block moved back before `override`, version left at 4 | `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` |

### 8b.9 Checkpoint 8b

- **Checkpoint 8a holds**, re-run whole from its own stage file's bytes, with `RK_CP_NESTED=1` and under section
  (W)'s replay wrapper. That is the only checkpoint this block re-runs whole, and under `RK_CP_NESTED=1` it re-runs
  none. Checkpoint 7b selects two names this stage re-points, `rigor_monotonicity` and
  `ac527_empty_set_terminal_condition`, and section (5) runs both. So no older checkpoint is made provisional here.
- **Every replay and every cargo run goes through the replay wrapper** (D21-47). A fake `claude` stands first on PATH
  and prints the pin, `REDKILN_LAUNCH_RECORD` is unset, and `CLAUDE_CONFIG_DIR` points at a trap that is proved empty
  before and after. This session's `CLAUDE_CONFIG_DIR` is the operator's real account. *(re-pointed 2026-09-24, final
  consistency pass)* The wrapper is the shared `runbook/tools/nodispatch.sh`. Its fake is **compiled** (`claude.exe`
  on Windows), so a Rust `Command::new("claude")` meets it too. Every other `PATH` directory holding a `claude` is
  dropped, and `nd_check` asserts that the trap is empty and that the fake logged no call.
- **No real dispatch can happen.**
  - If Checkpoint 8a's block reaches Checkpoint 7f's `redkiln run`, no card carries `permission_mode` at this stage,
    and `s8a`'s own guard refuses if one does.
  - Once `s8h` lands, a whole re-run of this block first points `[adapters] harness_binary` at
    `target/debug/rk-fixture-child-claude`, the canonical fixture harness (D21-47). That is how `s8j`'s entry-chain
    re-run says it does it.
- **The P6 lattice is in Rust.** Its parity with the Python is proven byte for byte over `spikes/p9` at `P9_PIN`, on
  the registered axes, except for the named divergences, against goldens that this block re-derives from the Python on
  every run. It runs over `.redkiln/` with **2** packs walked, on the axes the dated METHOD-P amendment adds (`rust`
  and `surfaces`), and in CI by name.
- **AC-101's and AC-102's kill lines hold over the registered pack set** (eight packs, `pack-default`). The installed
  pair's figures are reported with their count and are not thresholded (D21-47 V3).
- **Every catalog-quality instrument reads `.redkiln/`** and prints the path it read from inside the test binary.
  The five tests that pin a historical p9 measurement read `P9_PIN` from the object store (D21-47 V2), and every other
  reader of `spikes/p9` is inventoried in §8b.3.
- **No live line carries ADR-0080 d2's named form in any quoting.** This is `s8j`'s guard form, run here.
- **The twenty-two activities are at their new versions, with changelog rows.**
  - The amended lint refuses nothing in `.redkiln/catalog` and still refuses the frozen p9 catalog.
  - No shipped step declares `memo`.
  - `red-verify` selects the projection, and `sca-scan` routes an unrated advisory to `scan.accept-findings`.
  - `adversarial-review`'s `verdict` follows `record` (D21-47 V4).
- **Both recursive walks over `.redkiln/` exclude `worktrees/`**, which a planted control in a shaped scratch tree
  proves.
- **11c's record exists, located by its recorded id.** Its measurement was pre-registered before the labels existed,
  and the labels are read at the corpus commit that the record names.
- **AC-527 is amended to d2 in the register, and its contradicting clauses are struck.** Nothing claims the review
  gate's calibration, in the tree, the records or the commit messages.
- The whole suite is green above a floor re-derived from Checkpoint 8a's recorded count, and fmt and clippy are green.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Carried from Checkpoint 7i and not re-argued: every cargo reading is `out=$(…); rc=$?`, never a pipe's status;
# every selection is `--test it`, MODULE-QUALIFIED and `--exact`, and asserts each test's own `... ok` line AND an
# exact count; every grep over source reads CODE lines only and carries a synthetic control proving it can go red.

# ---- (W) THE REPLAY WRAPPER (D21-47), set up before anything runs. Section (0)'s replay AND every cargo run below go
# ---- through `nodispatch`, the shared runbook/tools/nodispatch.sh [re-pointed 2026-09-24, final consistency pass: it
# ---- replaces this section's own sh fake and trap]. A COMPILED fake `claude` first on PATH (claude.exe on Windows,
# ---- which a Rust Command::new("claude") resolves) prints the pin for --version and logs, then refuses with 97, any
# ---- other call; every other PATH dir holding a claude is dropped; REDKILN_LAUNCH_RECORD is unset; CLAUDE_CONFIG_DIR
# ---- is an empty trap, because this session's CLAUDE_CONFIG_DIR is the operator's real account. Under an enclosing
# ---- wrapper (ND_ACTIVE=1) nd_setup verifies and reuses it. nd_check: the trap is empty and the fake logged no call.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — the replay wrapper cannot be built"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/cp8b/nd" || exit 1
# A stage control beyond the helper's: a PLANTED launch record is removed, and a child sees the trap.
v=$(REDKILN_LAUNCH_RECORD=planted nodispatch sh -c 'printf "%s|%s" "${REDKILN_LAUNCH_RECORD-unset}" "$CLAUDE_CONFIG_DIR"'); rc=$?
[ "$rc" -eq 0 ] && [ "$v" = "unset|$ND_TRAP_NATIVE" ] || { echo "FAIL: the wrapper does not unset a planted REDKILN_LAUNCH_RECORD and point CLAUDE_CONFIG_DIR at the trap (got '$v')"; exit 1; }
nd_check "the wrapper's own set-up" || exit 1

# ---- (0) THE ENTRY GATE, RE-RUN WHOLE from its own stage file's bytes (D21-40's method). An inherited gate is a
# ---- claim. The fence is matched as a bracket class so that this block contains no literal fence of its own.
# ---- It is the ONLY checkpoint re-run whole here: 7b's two names this stage re-points run in (5) by module-qualified
# ---- name. No dispatch: if 8a's block reaches 7f's `redkiln run`, no card carries permission_mode at this stage and
# ---- s8a's guard refuses if one does. After s8h lands, a whole re-run first points [adapters] harness_binary at the
# ---- canonical fixture harness, target/debug/rk-fixture-child-claude (D21-47).
# ---- D21-46's ONE RE-RUN CONVENTION: SKIPPED under RK_CP_NESTED=1 (this block is another checkpoint's entry gate), and
# ---- 8a is launched with RK_CP_NESTED=1, so it skips its own nested whole re-runs. The replay runs under (W)'s wrapper,
# ---- which 8a's own guard (0) reuses. 8a needs no environment: its record ids and FIX_ITEM are literals in its block.
if [ "${RK_CP_NESTED:-0}" = 1 ]; then
  echo "CP8B (0) SKIPPED: RK_CP_NESTED=1 — this block is another checkpoint's entry gate; its re-run of 8a ran at Checkpoint 8b's own close"
else
  S8A=runbook/phase-1/s8a-records-and-red-checkpoints.md
  test -f "$S8A" || { echo "FAIL: $S8A is absent — Checkpoint 8a cannot be re-run (Halt 8)"; exit 1; }
  cp8a=target/cp8b/cp8a.sh
  awk '/^### 8a\.[0-9]+ Checkpoint 8a/ {f=1} f && /^[`][`][`]bash/ {b=1; body=""; next} b && /^[`][`][`]/ {b=0; last=body; next} b {body = body $0 "\n"} END {printf "%s", last}' "$S8A" > "$cp8a" || { echo "FAIL: Checkpoint 8a could not be extracted"; exit 1; }
  test -s "$cp8a" || { echo "FAIL: no bash block follows Checkpoint 8a's heading — nothing would be re-run"; exit 1; }
  grep -qxF 'echo "CP8A-END-OF-BLOCK reached"' "$cp8a" || { echo "FAIL: the extracted block does not end in CP8A's marker — it is not Checkpoint 8a"; exit 1; }
  out=$(nodispatch env RK_CP_NESTED=1 bash "$cp8a" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8a, re-run whole, exited $rc — the entry gate does not hold"; printf '%s\n' "$out" | tail -25; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8A-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8a exited 0 without reaching its end marker"; exit 1; }
  nd_check "Checkpoint 8a's replay" || exit 1
fi

# ---- (1) BUILD-OR-FAIL.
out=$(nodispatch cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8b NOT RUN (Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "the build" || exit 1

# ---- (2) ANCHORS FIRST: the subject exists and is whole, and the frozen oracle's inputs are the ones authored against.
test -d .redkiln/catalog || { echo "FAIL: .redkiln/catalog is absent (Halt 8)"; exit 1; }
n=$(find .redkiln/catalog -maxdepth 1 -type f -name '*.toml' | grep -c .)
[ "$n" -eq 102 ] || { echo "FAIL: .redkiln/catalog holds $n activities, not 102 — a subset cannot stand for the shipped pack"; exit 1; }
git diff --quiet ca8f387 -- spikes/p9/catalog spikes/p9/facets.toml spikes/p9/artifacts.toml spikes/p9/lattice.py; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the p9 catalog, registry or lattice changed since ca8f387 (rc=$rc) — the spike tree's live readers may not be edited to make a green"; exit 1; }
unt=$(git status --porcelain --untracked-files=all -- spikes/p9/catalog spikes/p9/facets.toml spikes/p9/artifacts.toml spikes/p9/lattice.py); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git status could not read spikes/p9 (rc=$rc)"; exit 1; }
[ -z "$unt" ] || { echo "FAIL: spikes/p9 carries uncommitted or UNTRACKED files that git diff cannot see:"; printf '%s\n' "$unt"; exit 1; }
# D21-47 V2: the frozen subject is the p9 tree AT P9_PIN, read from the object store — never the working tree.
P9_PIN=ca8f3874ed9185b419b00b21b33713ba128de217
git cat-file -e "$P9_PIN^{commit}" || { echo "FAIL: P9_PIN $P9_PIN is not in this clone (a shallow checkout?) — the frozen p9 tests have no subject"; exit 1; }
src=$(git cat-file blob "$P9_PIN:spikes/p9/catalog/root-cause.toml"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$src" ] || { echo "FAIL: root-cause.toml could not be read at P9_PIN (rc=$rc)"; exit 1; }
printf '%s\n' "$src" | grep -qF "findings.none(evidence in ['Reproduced', 'MutationSurvived'])" || { echo "FAIL: the p9 root-cause at P9_PIN does not carry ADR-0080's predicate — the red control has nothing to be red about"; exit 1; }

# ---- (2b) THE V2 SPLIT (D21-47 V2; review M11). The four files holding the five tests that pin a historical p9
# ---- measurement name no "spikes/p9 literal on a CODE line: they read the pinned tree through p9_pin, the one helper,
# ---- whose P9_PIN is the literal above. Every other reader of spikes/p9 is inventoried in §8b.3 and is not a pin.
for f in crates/rk-kernel/tests/it/rigor.rs crates/rk-kernel/tests/it/memo.rs crates/rk-kernel/tests/it/quality_block.rs crates/rk-kernel/tests/it/lattice.rs; do
  test -f "$f" || { echo "FAIL: $f is absent (Halt 8)"; exit 1; }
  raw=$(grep -nE '"(\.\./\.\./)?spikes/p9' "$f"); rc=$?
  [ "$rc" -le 1 ] || { echo "FAIL: the spikes/p9 sweep of $f could not run (grep rc=$rc)"; exit 1; }
  live=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*)' | grep .)
  [ -z "$live" ] || { echo "FAIL: $f reads spikes/p9 from the WORKING TREE — a p9 pin reads P9_PIN through p9_pin (D21-47 V2):"; printf '%s\n' "$live"; exit 1; }
done
syn=$(printf '%s\n' '43:const SHIPPED_CATALOG: &str = "spikes/p9/catalog";' | grep -E '"(\.\./\.\./)?spikes/p9' | grep -vE '^[0-9]+:[[:space:]]*(//|/\*|\*)')
test -n "$syn" || { echo "FAIL: the V2 sweep does not match a working-tree p9 constant — this guard is vacuous"; exit 1; }
H=crates/rk-kernel/tests/it/p9_pin.rs
test -f "$H" || { echo "FAIL: $H is absent — the frozen p9 tests have no pinned reader"; exit 1; }
n=$(grep -cxF "const P9_PIN: &str = \"$P9_PIN\";" "$H")
[ "$n" -eq 1 ] || { echo "FAIL: $H does not declare P9_PIN as $P9_PIN on exactly one line ($n)"; exit 1; }
decl=$(grep -rn 'const P9_PIN' crates/); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$decl" | grep -c .)" -eq 1 ] || { echo "FAIL: P9_PIN is declared other than once in crates/ — two pins can drift apart:"; printf '%s\n' "$decl"; exit 1; }
grep -qxE 'mod p9_pin;' crates/rk-kernel/tests/it/main.rs || { echo "FAIL: tests/it/main.rs does not mount mod p9_pin"; exit 1; }

# ---- (3) THE TWENTY-TWO ACTIVITIES: new versions (ADR-0072) and one changelog row each. The seven d2 rows first,
# ---- then N26's fourteen memo repairs (instrumented-trace-analysis shares its bump), N13 and N25.
test -f .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md is absent — ADR-0072's changelog was never written"; exit 1; }
PAIRS="root-cause:5 differential-debugging:3 adversarial-review:4 threat-model:2 user-story-ac:3 vertical-slicing:2 instrumented-trace-analysis:3 blue-green-deploy:3 failover:3 feature-flag-kill:4 flag-progressive-rollout:3 incident-command:2 migration-dry-run:3 migration-expand-execute:3 rate-limit-shed:3 rollback-mitigation:3 rollback:2 rolling-deploy:3 rolling-restart:3 slo-monitoring:3 tdd-red-green-refactor:3 sca-scan:2"
np=$(printf '%s\n' $PAIRS | grep -c .)
[ "$np" -eq 22 ] || { echo "FAIL: the version list names $np activities, not 22"; exit 1; }
for pair in $PAIRS; do
  id=${pair%%:*}; v=${pair##*:}; f=".redkiln/catalog/$id.toml"
  test -f "$f" || { echo "FAIL: $f is absent (Halt 8)"; exit 1; }
  grep -qE "^version[[:space:]]*=[[:space:]]*${v}([[:space:]]|$)" "$f" || { echo "FAIL: $id is not at version $v — a coherence edit bumps version (ADR-0072)"; exit 1; }
  grep -qF "$id@$v" .redkiln/CHANGELOG.md || { echo "FAIL: no changelog row names $id@$v (ADR-0072)"; exit 1; }
done

# ---- (3b) AC-018's REPAIR (D21-43 N26): no shipped step declares the withdrawn `memo` key. .redkiln/catalog is flat,
# ---- so this walk cannot reach .redkiln/worktrees/. Silence is the pass; the control proves the pattern can fire.
raw=$(grep -nE '^[[:space:]]*memo[[:space:]]*=' .redkiln/catalog/*.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the memo sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a shipped step still declares memo (ADR-0097 decision 3):"; printf '%s\n' "$raw"; exit 1; }
syn=$(printf '%s\n' 'memo    = false' | grep -E '^[[:space:]]*memo[[:space:]]*=')
test -n "$syn" || { echo "FAIL: the memo pattern does not match the shipped spelling — this guard is vacuous"; exit 1; }

# ---- (3c) N13 AND N25, on CODE lines of the two activities.
f=.redkiln/catalog/tdd-red-green-refactor.toml
hit=$(grep -nF 'selection = "${red.outputs[*].tests_added}"' "$f" | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -n "$hit" || { echo "FAIL: red-verify does not select the [*].tests_added projection on a code line (D21-43 N13, ADR-0035)"; exit 1; }
grep -qF 'selection = "${red.outputs}"' "$f"; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: red-verify still selects the unprojected red outputs, or the grep failed (rc=$rc)"; exit 1; }
f=.redkiln/catalog/sca-scan.toml
grep -qxE 'expr[[:space:]]*=[[:space:]]*"report\.critical == 0 and report\.high == 0 and report\.unrated == 0"' "$f" || { echo "FAIL: sca-scan's clean check does not require unrated == 0 (D21-43 N25)"; exit 1; }
grep -qxE 'when[[:space:]]*=[[:space:]]*"\$\{scan\.outputs\.critical\} > 0 or \$\{scan\.outputs\.high\} > 0 or \$\{scan\.outputs\.unrated\} > 0"' "$f" || { echo "FAIL: sca-scan's exception gate does not open on an unrated advisory (D21-43 N25)"; exit 1; }
grep -qxE 'question[[:space:]]*=[[:space:]]*"scan\.accept-findings"' "$f" || { echo "FAIL: sca-scan's exception no longer asks scan.accept-findings"; exit 1; }

# ---- (3d) D21-47 V4: adversarial-review's `verdict` reads what `record` produces, so it runs after it. The step order
# ---- is read from the [[step]] ids; the control reads the unedited order at P9_PIN, where verdict precedes record.
step_order() { awk '/^\[\[step\]\]/ {s=1; next} s && /^id[[:space:]]*=/ {gsub(/^id[[:space:]]*=[[:space:]]*"|".*$/, ""); printf "%s ", $0; s=0}'; }
order=$(step_order < .redkiln/catalog/adversarial-review.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: adversarial-review's step order could not be read (rc=$rc)"; exit 1; }
[ "$order" = "clone review override record verdict emit " ] || { echo "FAIL: adversarial-review's steps run '$order' — D21-47 V4 puts verdict after record, whose outcome it reads"; exit 1; }
src=$(git cat-file blob "$P9_PIN:spikes/p9/catalog/adversarial-review.toml"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the control copy could not be read at P9_PIN (rc=$rc)"; exit 1; }
ctl=$(printf '%s\n' "$src" | step_order)
[ "$ctl" = "clone review verdict override record emit " ] || { echo "FAIL: the step-order reader does not read the unedited order ('$ctl') — this guard is vacuous"; exit 1; }

# ---- (4) THE GUARD FORM s8j ADOPTS, RUN HERE: ADR-0080 d2's named form in ANY quoting and with ANY member list, on
# ---- a CODE line, anywhere in crates/ or .redkiln/, outside the named enforcement, frozen-fixture and changelog
# ---- paths. Every recursive walk over .redkiln/ excludes worktrees/ (D21-44): a run worktree is a full repo copy.
EXCL='^(crates/rk-kernel/tests/it/(evaluator|quality_block)\.rs|crates/rk-kernel/tests/fixtures/catalog/(root-cause|differential-debugging|instrumented-trace-analysis)\.toml|crates/rk-kernel/tests/fixtures/activities/terminal_condition_(is_findings_none|does_not_parse|names_any_member_list|is_one_member_conjunct)\.toml|\.redkiln/CHANGELOG\.md):'
# ---- Comment lines are dropped by their file's own syntax only: `//`, `/*` and `*` in .rs, `#` in .toml and .yml. A
# ---- Markdown `*` bullet is not a comment, so no .md line escapes by opening with `*` (review M1).
code_only() { grep -vE '^[^:]+\.rs:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -vE '^[^:]+\.(toml|ya?ml):[0-9]+:[[:space:]]*#'; }
# ---- --exclude-dir matches a base name at ANY depth, and crates/ holds three fixture directories named worktrees. So
# ---- crates/ is walked whole in its own call, and only the .redkiln/ call carries the flag. The .redkiln/ root is the
# ---- walk's 4th argument, so the planted control below runs the same function over a scratch root. Each walk returns
# ---- grep's convention: 0 = a hit, 1 = none, 2 = an error in either call.
wt=$(find .redkiln -path .redkiln/worktrees -prune -o -type d -name worktrees -print); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: find over .redkiln/ could not run (rc=$rc)"; exit 1; }
[ -z "$wt" ] || { echo "FAIL: a directory named worktrees sits inside .redkiln/ besides .redkiln/worktrees — the exclusion would skip it silently: $wt"; exit 1; }
D2='findings\.none\(evidence in \[|quality_shorthand *='
CAL='guarded against false approval|calibrated against false approval'
two_walk() { grep -rn "$1" "$2" "$3"; a=$?; grep -rn --exclude-dir=worktrees "$1" "$2" "$4"; b=$?; [ "$a" -le 1 ] && [ "$b" -le 1 ] || return 2; [ "$a" -eq 0 ] || [ "$b" -eq 0 ] || return 1; }
d2_walk() { two_walk -E "$D2" crates/ "${1:-.redkiln/}"; }
cal_walk() { two_walk -iE "$CAL" crates/rk-kernel/ "${1:-.redkiln/}"; }
# The planted control lives in a SCRATCH tree shaped like .redkiln/ (review M10): an unexcluded walk of the real
# .redkiln/ would descend into run worktrees (junctions, locked files, grep rc 2). The .gitignore row is asserted by
# path; git check-ignore needs no file to exist.
git check-ignore -q .redkiln/worktrees/s8b-plant/plant.toml || { echo "FAIL: D21-44's .redkiln/worktrees/ row is missing from .gitignore"; exit 1; }
SHAPE=target/cp8b/shape/.redkiln
rm -rf target/cp8b/shape && mkdir -p "$SHAPE/catalog" "$SHAPE/worktrees/s8b-plant" || { echo "FAIL: cannot create the shaped scratch tree"; exit 1; }
printf '%s\n' "expr = \"findings.none(evidence in ['Reproduced'])\"" '# guarded against false approval' > "$SHAPE/worktrees/s8b-plant/plant.toml" || { echo "FAIL: cannot write the planted control"; exit 1; }
printf '%s\n' "expr    = \"findings.none(evidence in ['Reproduced'])\"" > "$SHAPE/catalog/live.toml" || { echo "FAIL: cannot write the shaped catalog row"; exit 1; }
seen=$(grep -rlF 'findings.none(evidence in [' "$SHAPE/"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the unexcluded walk of the shaped tree could not run or found nothing (rc=$rc)"; exit 1; }
printf '%s\n' "$seen" | grep -qxF "$SHAPE/worktrees/s8b-plant/plant.toml" || { echo "FAIL: an unexcluded walk does not see the plant — the control proves nothing"; exit 1; }
raw=$(d2_walk "$SHAPE/"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the d2 walk over the shaped tree found nothing or could not run (rc=$rc)"; exit 1; }
printf '%s\n' "$raw" | grep -qF "$SHAPE/catalog/live.toml:" || { echo "FAIL: the d2 walk's .redkiln/ half does not reach catalog/ in the shaped tree"; exit 1; }
leak=$(printf '%s\n' "$raw" | grep -F "$SHAPE/worktrees/")
[ -z "$leak" ] || { echo "FAIL: the d2 walk descended into worktrees/ (D21-44)"; exit 1; }
cal=$(cal_walk "$SHAPE/"); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the calibration walk over the shaped tree could not run (grep rc=$rc)"; exit 1; }
leak=$(printf '%s\n' "$cal" | grep -F "$SHAPE/worktrees/")
[ -z "$leak" ] || { echo "FAIL: the calibration walk descended into worktrees/ (D21-44)"; exit 1; }
rm -rf target/cp8b/shape || { echo "FAIL: cannot remove the shaped scratch tree"; exit 1; }
# Review N10: the real walk's .redkiln/ half reaches .redkiln/catalog, where every activity carries an expr row.
pos=$(two_walk -E '^expr[[:space:]]*=' crates/ .redkiln/); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the positive control walk found nothing or could not run (rc=$rc)"; exit 1; }
printf '%s\n' "$pos" | grep -qE '^\.redkiln/catalog/[a-z0-9-]+\.toml:[0-9]+:expr' || { echo "FAIL: the .redkiln/ half of the walk does not reach .redkiln/catalog — its silence below would be a fake one"; exit 1; }
raw=$(d2_walk); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the d2 guard could not run (grep rc=$rc; 2 = no such path) — a silence here would be a fake one"; exit 1; }
printf '%s\n' "$raw" | grep -q '^crates/rk-kernel/tests/it/quality_block\.rs:' || { echo "FAIL: the d2 walk did not reach the enforcement's own literals in crates/ — the crates/ half is not being walked"; exit 1; }
live=$(printf '%s\n' "$raw" | code_only | grep -vE "$EXCL" | grep .)
[ -z "$live" ] || { echo "FAIL: ADR-0080 d2's forbidden form on a live line:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/x.toml:5:expr    = \"findings.none(evidence in ['Reproduced'])\"" | grep -E "$D2" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the exclusions swallowed a live one-member row — this guard is vacuous"; exit 1; }
syn=$(printf '%s\n' "crates/rk-kernel/src/quality_block.rs:9:    let p = \"findings.none(evidence in [\";" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the path exclusion reaches src/quality_block.rs — only the TEST file's literals are exempt"; exit 1; }
syn=$(printf '%s\n' ".redkiln/DISPOSITIONS-2026-09-21.md:4:* findings.none(evidence in ['Reproduced'])" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: a Markdown bullet was dropped as a comment — review M1's hole is open"; exit 1; }
syn=$(printf '%s\n' ".redkiln/CHANGELOG.md:9:| root-cause@5 | old: findings.none(evidence in ['Reproduced', 'MutationSurvived']) |" | code_only | grep -vE "$EXCL")
test -z "$syn" || { echo "FAIL: the changelog's quotation of the old text is not excluded — (4) would go red on ADR-0072's own record"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/CHANGELOG.md:9:findings.none(evidence in [" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the changelog exclusion reaches beyond .redkiln/CHANGELOG.md"; exit 1; }

# ---- (5) THE NAMED SELECTION: thirty tests, each by its own `... ok` line, and exactly thirty.
K_TESTS="lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged memo::ac018_live_verb_may_not_memoise memo::ac018_frozen_p9_catalog_is_red quality_block::adr0072_s8b_repairs_are_versioned_and_changelogged pack_validation::tdd_red_verify_projects_tests_added_per_adr_0035 pack_validation::sca_scan_blocks_on_unrated_or_routes_to_accept_findings lattice::p6_lattice_parity_over_the_p9_tree lattice::p6_validate_parity_over_the_p9_tree lattice::p6_lattice_over_the_shipped_pack lattice::p6_validate_over_the_shipped_pack lattice::zero_packs_is_refused_not_reported lattice::covering_array_is_pairwise_and_matches_the_python_rows lattice::unsatisfiable_fixture_names_its_cell lattice::rank_tie_fixture_is_an_error lattice::hard_cycle_fixture_is_an_error lattice::dead_activity_fixture_is_listed lattice::family_hole_at_mandatory_fails_totality p9_pin::reads_the_object_store_at_the_named_commit_not_the_working_tree rigor::rigor_monotonicity rigor::rigor_monotonicity_p9_historical_2026_09_13 review::ac303_confirm_step_survives_escalation quality_block::ac527_empty_set_terminal_condition quality_block::ac527_frozen_p9_catalog_is_red quality_block::d2_per_class_witness_refuses_one_member_rows quality_block::d2_named_form_refuses_any_member_list quality_block::d2_severity_only_predicates_load quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies quality_block::adr0072_d2_edits_are_versioned_and_changelogged conformance::shipped_pack_loads"
nk=$(printf '%s\n' $K_TESTS | grep -c .)
[ "$nk" -eq 30 ] || { echo "FAIL: K_TESTS names $nk tests, not 30"; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- $K_TESTS --exact 2>&1); rc=$?
nd_check "the named selection" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: the stage's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $K_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -25; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 30 passed; 0 failed' || { echo "FAIL: the named selection did not run exactly 30 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# ---- (6) WHAT EACH INSTRUMENT READ, PRINTED FROM INSIDE THE TEST BINARY — not inferred from a green, and not probed
# ---- in another language. The installed pair's AC-101/AC-102 figures are REPORTED with their pack count and NOT
# ---- thresholded (D21-47 V3); AC-103's zero tolerance is not a rate and holds over the pair too.
out=$(nodispatch cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the shipped-pack lattice run failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
line=$(printf '%s\n' "$out" | grep -E '^lattice-shipped: packs=[0-9]+ rigor=pack-default contexts=[0-9]+ pairs=[0-9]+ unsat=[0-9]+ dead=[0-9]+/102 ties=[0-9]+ hard_cycles=[0-9]+ ')
test -n "$line" || { echo "FAIL: the shipped-pack lattice printed no reading of the agreed shape — nothing says what it walked"; exit 1; }
[ "$(printf '%s\n' "$line" | grep -c .)" -eq 1 ] || { echo "FAIL: the shipped-pack lattice printed more than one reading"; exit 1; }
field() { printf '%s\n' "$line" | sed -E "s/^.* $1=([0-9]+)[ /].*$/\1/"; }
packs=$(field packs); ctx=$(field contexts); pairs=$(field pairs); unsat=$(field unsat); dead=$(field dead); ties=$(field ties); hc=$(field hard_cycles)
[ "$packs" -eq 2 ] || { echo "FAIL: the lattice walked $packs packs over .redkiln/, not the installed 2"; exit 1; }
[ "$ctx" -gt 0 ] && [ "$pairs" -gt 0 ] || { echo "FAIL: the lattice walked zero contexts or pairs — a report over nothing (Halt 8)"; exit 1; }
[ "$ties" -eq 0 ] && [ "$hc" -eq 0 ] || { echo "FAIL: AC-103 over the installed pair: ties=$ties hard_cycles=$hc — the tolerance is zero"; exit 1; }
echo "CP8B (6) AC-101/AC-102 over the INSTALLED pair, reported and NOT thresholded (D21-47 V3; ADR-0011, E7): packs=$packs rigor=pack-default unsat=$unsat/$pairs dead=$dead/102"
printf '%s\n' "$line" | grep -qE ' toolchains=([^ ]*,)?rust(,[^ ]*)? surfaces=[1-9][0-9]*$' || { echo "FAIL: the shipped-pack lattice did not walk the rust toolchain and a non-empty surfaces binding (D21-43 N26)"; printf '%s\n' "$line"; exit 1; }
grep -qE '^[[:space:]-]*\*Amendment [0-9]+ \(2026-[0-9]{2}-[0-9]{2}, D21-43 N26\):\*' spikes/METHOD-P.md || { echo "FAIL: METHOD-P carries no dated amendment citing D21-43 N26 — the axes moved without the record"; exit 1; }
# Review N7: the amendment was COMMITTED before the shipped reading existed — its commit is a strict ancestor of the
# commit that added the test which first reads .redkiln/ on the new axes.
am=$(git log --reverse --format=%H -S'D21-43 N26):*' -- spikes/METHOD-P.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$am" ] || { echo "FAIL: no commit added METHOD-P's N26 amendment (rc=$rc)"; exit 1; }
tc=$(git log --reverse --format=%H -S'fn p6_lattice_over_the_shipped_pack' -- crates/rk-kernel/tests/it/lattice.rs); rc=$?
[ "$rc" -eq 0 ] && [ -n "$tc" ] || { echo "FAIL: no commit added lattice::p6_lattice_over_the_shipped_pack (rc=$rc)"; exit 1; }
am1=$(printf '%s\n' "$am" | head -1); tc1=$(printf '%s\n' "$tc" | head -1)
[ "$am1" != "$tc1" ] || { echo "FAIL: the METHOD-P amendment and the shipped reading landed in one commit ($am1) — the axes were not registered first"; exit 1; }
git merge-base --is-ancestor "$am1" "$tc1" || { echo "FAIL: the METHOD-P amendment ($am1) does not precede the shipped reading ($tc1)"; exit 1; }
for a in AC-101 AC-102; do
  ent=$(awk -v k="**$a**" 'index($0, k) == 1 {f=1; print; next} f && /^\*\*AC-/ {f=0} f' references/2026-09-08-acceptance-criteria.md)
  test -n "$ent" || { echo "FAIL: $a's entry was not found in the register (Halt 8)"; exit 1; }
  printf '%s\n' "$ent" | grep -qF 'lattice::p6_lattice_over_the_shipped_pack' || { echo "FAIL: $a carries no dated note naming the Rust instrument of record"; exit 1; }
  printf '%s\n' "$ent" | grep -qF 'D21-47 V3' || { echo "FAIL: $a's note does not state V3's reading (the registered set thresholded, the installed pair reported)"; exit 1; }
done
for s in SM-026 SM-027; do
  row=$(grep -E "^\| $s \|" references/2026-09-08-acceptance-criteria.md); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $s's row was not found (rc=$rc)"; exit 1; }
  printf '%s\n' "$row" | grep -qF 'lattice::p6_lattice_over_the_shipped_pack' || { echo "FAIL: $s records no reading from the Rust lattice with its pack count"; exit 1; }
done
out=$(nodispatch cargo test -p rk-kernel --test it -- memo::ac018_live_verb_may_not_memoise --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the AC-018 lint failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^ac018: catalog=\.redkiln/catalog activities=102 steps=[1-9][0-9]* violations=0 registry=tests/fixtures/registries/verbs/verbs-live\.toml$' || { echo "FAIL: the AC-018 lint did not report .redkiln/catalog, 102 activities, a non-zero step count, zero violations and the FIXTURE registry it read"; printf '%s\n' "$out" | grep -E '^ac018'; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- rigor::rigor_monotonicity --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rigor_monotonicity failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^rigor_monotonicity: catalog=\.redkiln/catalog activities=102 families=[1-9][0-9]* contexts=[1-9][0-9]* ' || { echo "FAIL: rigor_monotonicity did not report reading .redkiln/catalog with a non-zero walk"; printf '%s\n' "$out" | grep -E '^rigor_monotonicity:'; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- quality_block::ac527_empty_set_terminal_condition --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the AC-527 lint failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^ac527: catalog=\.redkiln/catalog checks=[1-9][0-9]* refused=0 ' || { echo "FAIL: the AC-527 lint did not report .redkiln/catalog, a non-zero check count and refused=0"; printf '%s\n' "$out" | grep -E '^ac527:'; exit 1; }
nd_check "sections (1) to (6)" || exit 1

# ---- (7) THE PARITY GOLDENS: present, pinned, RE-DERIVED from the Python oracle at P9_PIN on this run (review B2 —
# ---- a golden regenerated from the port would pass every parity test), twelve packs, the recorded baseline, no CR
# ---- byte, and the kill lines over the REGISTERED pack set (review B1, D21-47 V3). `od -c` reads raw bytes; Git Bash
# ---- strips a CR inside $(…). The Python run takes about 2 min 10 s and spends nothing.
G=crates/rk-kernel/tests/fixtures/lattice
grep -qxF 'crates/rk-kernel/tests/fixtures/lattice/** -text' .gitattributes || { echo "FAIL: no -text row pins the lattice goldens"; exit 1; }
for g in "$G/p9-run.txt" "$G/p9-validate.txt"; do
  test -s "$g" || { echo "FAIL: $g is absent or empty (Halt 8)"; exit 1; }
  if od -c "$g" | grep -qF '\r'; then echo "FAIL: $g carries a CR byte — the Python wrote through a text-mode stream"; exit 1; fi
done
drift=$(git status --porcelain --untracked-files=all -- "$G"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git status could not read the goldens (rc=$rc)"; exit 1; }
[ -z "$drift" ] || { echo "FAIL: the goldens differ from their committed bytes:"; printf '%s\n' "$drift"; exit 1; }
test -s "$G/p9-python-version.txt" || { echo "FAIL: the Python version the goldens were made with is not recorded"; exit 1; }
pyv=$(python --version 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: python is not runnable — the oracle cannot be re-derived (rc=$rc)"; exit 1; }
[ "$pyv" = "$(cat "$G/p9-python-version.txt")" ] || { echo "FAIL: the oracle's Python is now '$pyv', not the goldens' — string hashing and so the output may differ; regenerate at stage open's route"; exit 1; }
P9_AT=target/cp8b/p9-at-pin
rm -rf "$P9_AT" && mkdir -p "$P9_AT" || { echo "FAIL: cannot create $P9_AT"; exit 1; }
git -c core.autocrlf=false archive --format=tar -o target/cp8b/p9-at-pin.tar "$P9_PIN" spikes/p9 || { echo "FAIL: git archive of spikes/p9 at P9_PIN failed"; exit 1; }
tar -xf target/cp8b/p9-at-pin.tar -C "$P9_AT" || { echo "FAIL: cannot extract the pinned p9 tree"; exit 1; }
git ls-tree -r "$P9_PIN" -- spikes/p9 > target/cp8b/p9-ls.txt || { echo "FAIL: git ls-tree over spikes/p9 at P9_PIN failed"; exit 1; }
nls=$(grep -c . target/cp8b/p9-ls.txt)
[ "$nls" -gt 0 ] || { echo "FAIL: the pinned tree lists no files — there is no oracle"; exit 1; }
bad=0
while read -r mode type oid path; do got=$(git hash-object --no-filters "$P9_AT/$path"); [ "$got" = "$oid" ] || { echo "MISMATCH: $path"; bad=$((bad + 1)); }; done < target/cp8b/p9-ls.txt
[ "$bad" -eq 0 ] || { echo "FAIL: $bad of $nls materialised files are not their blobs' bytes — the oracle would read a smudged tree"; exit 1; }
PYTHONHASHSEED=0 PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py']; runpy.run_path('$P9_AT/spikes/p9/lattice.py', run_name='__main__')" > target/cp8b/p9-run.fresh.txt; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the Python lattice exited $rc at P9_PIN — the oracle cannot be re-derived"; exit 1; }
PYTHONHASHSEED=0 PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py','--validate']; runpy.run_path('$P9_AT/spikes/p9/lattice.py', run_name='__main__')" > target/cp8b/p9-validate.fresh.txt; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: lattice.py --validate exited $rc at P9_PIN"; exit 1; }
cmp -s target/cp8b/p9-run.fresh.txt "$G/p9-run.txt" || { echo "FAIL: the committed p9-run.txt is not what the Python prints at P9_PIN — the golden was not made by the oracle"; exit 1; }
cmp -s target/cp8b/p9-validate.fresh.txt "$G/p9-validate.txt" || { echo "FAIL: the committed p9-validate.txt is not what the Python prints at P9_PIN"; exit 1; }
# Named divergences (§8b.2): one committed row per golden line the port does not reproduce byte for byte, read by both
# parity tests. NDIV is written here by dated edit when the port is traced, and is at least the six `soft` cycle-example
# lines whose choice is Python set order.
NDIV=6
test -f "$G/divergences.tsv" || { echo "FAIL: $G/divergences.tsv is absent — divergences must be named, never absorbed"; exit 1; }
ndiv=$(grep -c . "$G/divergences.tsv"); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the divergence list could not be read (rc=$rc)"; exit 1; }
[ "$ndiv" -eq "$NDIV" ] || { echo "FAIL: divergences.tsv names $ndiv rows, not the $NDIV recorded — a divergence was added or dropped without a dated edit"; exit 1; }
grep -qF "packs=['bug-expedite', 'bug-standard', 'chore', 'docs', 'incident', 'initiative', 'project', 'security-vuln', 'spike', 'story-ue', 'story', 'tech-debt']" "$G/p9-run.txt" || { echo "FAIL: the golden is not a twelve-pack run — a zero-pack run prints a whole baseline"; exit 1; }
grep -qF 'TOTAL prereg-8 pack-default  unsat=432/155520 (0.3%)   all-12 unsat=432/235872 (0.2%)' "$G/p9-run.txt" || { echo "FAIL: the golden's pack-default total is not the register's recorded baseline (AC-101)"; exit 1; }
grep -qF 'dead=7/102 (6.9%)' "$G/p9-run.txt" || { echo "FAIL: the golden's dead count is not the register's recorded baseline (AC-102)"; exit 1; }
grep -qxF 'validate errors=0' "$G/p9-validate.txt" || { echo "FAIL: the validate golden does not end in validate errors=0 (AC-104)"; exit 1; }
# D21-47 V3: THE KILL LINES APPLY TO THE REGISTERED PACK SET — the eight pre-registered packs at pack-default rigor —
# with the pack count reported (SM-027). A fired line exits 1 naming its AC. `kill_fires N D P` is N/D > P%.
kill_fires() { [ $(( $1 * 100 )) -gt $(( $2 * $3 )) ]; }
kill_fires 7777 155520 5 || { echo "FAIL: control — 7777/155520 (5.0006%) must fire AC-101's >5% line"; exit 1; }
if kill_fires 7776 155520 5; then echo "FAIL: control — exactly 5% must not fire a >5% line"; exit 1; fi
u_line=$(grep -E '\| TOTAL prereg-8 pack-default +unsat=[0-9]+/[0-9]+ ' "$G/p9-run.txt"); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$u_line" | grep -c .)" -eq 1 ] || { echo "FAIL: the golden carries no single prereg-8 pack-default total (rc=$rc)"; exit 1; }
u_num=$(printf '%s\n' "$u_line" | sed -E 's/^.*prereg-8 pack-default +unsat=([0-9]+)\/([0-9]+) .*$/\1/'); u_den=$(printf '%s\n' "$u_line" | sed -E 's/^.*prereg-8 pack-default +unsat=([0-9]+)\/([0-9]+) .*$/\2/')
d_line=$(grep -E '\| under the eight pre-registered packs only: dead=[0-9]+/[0-9]+ ' "$G/p9-run.txt"); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$d_line" | grep -c .)" -eq 1 ] || { echo "FAIL: the golden carries no single eight-pack dead reading (rc=$rc)"; exit 1; }
d_num=$(printf '%s\n' "$d_line" | sed -E 's/^.* dead=([0-9]+)\/([0-9]+) .*$/\1/'); d_den=$(printf '%s\n' "$d_line" | sed -E 's/^.* dead=([0-9]+)\/([0-9]+) .*$/\2/')
[ "$u_den" -gt 0 ] && [ "$d_den" -eq 102 ] || { echo "FAIL: the registered-set readings did not parse (unsat=$u_num/$u_den dead=$d_num/$d_den)"; exit 1; }
if kill_fires "$u_num" "$u_den" 5; then echo "FAIL: AC-101's kill line FIRES over the registered pack set: unsat=$u_num/$u_den > 5% (8 packs, pack-default)"; exit 1; fi
if kill_fires "$d_num" "$d_den" 20; then echo "FAIL: AC-102's kill line FIRES over the registered pack set: dead=$d_num/$d_den > 20% (8 packs, pack-default)"; exit 1; fi
echo "CP8B (7) AC-101/AC-102 over the REGISTERED set (D21-47 V3): packs=8 rigor=pack-default unsat=$u_num/$u_den dead=$d_num/$d_den — inside both kill lines"

# ---- (8) CI RUNS THE LATTICE BY NAME, --exact, and asserts its ok line (review N7). CODE lines of ci.yml, not YAML
# ---- comments. The checkouts fetch whole history, or P9_PIN is absent from CI's clone (D21-47 V2).
ci=$(grep -nF 'cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact' .github/workflows/ci.yml | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -n "$ci" || { echo "FAIL: ci.yml runs no module-qualified --exact lattice step — a workspace-wide filter would not say the lattice ran"; exit 1; }
ok=$(grep -nF "grep -qxF 'test lattice::p6_lattice_over_the_shipped_pack ... ok'" .github/workflows/ci.yml | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -n "$ok" || { echo "FAIL: ci.yml does not assert the lattice step's ... ok line — a filter matching nothing exits 0"; exit 1; }
syn=$(printf '%s\n' '40:          # cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact' | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -z "$syn" || { echo "FAIL: the YAML-comment exclusion let a comment through — this guard is satisfiable by prose"; exit 1; }
fd=$(grep -nE '^[[:space:]]*fetch-depth:[[:space:]]*0[[:space:]]*$' .github/workflows/ci.yml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the fetch-depth sweep could not run (rc=$rc)"; exit 1; }
[ "$(printf '%s\n' "$fd" | grep -c .)" -ge 2 ] || { echo "FAIL: ci.yml's gates and no-egress checkouts do not both fetch whole history — P9_PIN is absent from a depth-1 clone"; exit 1; }

# ---- (9) 11c: THE PRE-REGISTRATION PRECEDES THE LABELS, AND THE RECORD EXISTS IN ADR-0085's FORM. The record is
# ---- located by the id written here at write time (D21-47), never by a search; the labels are read at the corpus
# ---- commit the record names, never at the corpus checkout's HEAD (review M2).
test -f spikes/METHOD-SEV.md || { echo "FAIL: spikes/METHOD-SEV.md is absent — the measurement was not pre-registered"; exit 1; }
add=$(git log --diff-filter=A --format=%H -- spikes/METHOD-SEV.md); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$add" | grep -c .)" -eq 1 ] || { echo "FAIL: METHOD-SEV.md has no single committing add (rc=$rc) — an uncommitted pre-registration is not one"; exit 1; }
pre_at=$(git show -s --format=%ct "$add"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pre_at" ] || { echo "FAIL: the pre-registration's commit time could not be read (rc=$rc)"; exit 1; }
SSEV_ADR=____   # written by dated edit in the commit that writes the record, as ADR-<nnnn>: its id at write time
case "$SSEV_ADR" in ADR-[0-9][0-9][0-9][0-9]) ;; *) echo "FAIL: SSEV_ADR is '$SSEV_ADR' — the record's id was not written here when it was written"; exit 1 ;; esac
recs=$(find references/adr -maxdepth 1 -name "${SSEV_ADR#ADR-}-*.md"); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$recs" | grep -c .)" -eq 1 ] || { echo "FAIL: $SSEV_ADR does not name exactly one file under references/adr/ (rc=$rc): $recs"; exit 1; }
grep -qE '^evidence:.*"S-SEV"' "$recs" || { echo "FAIL: $recs does not carry S-SEV in its evidence — SSEV_ADR names the wrong record"; exit 1; }
grep -qE '^### Option — ' "$recs" || { echo "FAIL: $recs does not use ADR-0085's option headings (D21-42 24a)"; exit 1; }
grep -qE '^\*\*What this forbids\*\*' "$recs" || { echo "FAIL: $recs carries no What this forbids block"; exit 1; }
own=$(awk '/\*\*The severity field.s source and owner\.\*\*/ {f=1} f && /^[[:space:]]*$/ {exit} f' "$recs")
test -n "$own" || { echo "FAIL: $recs does not name the severity field's source and owner — 39 shipped rows read a field Finding lacks"; exit 1; }
printf '%s\n' "$own" | grep -qF 's8h' || { echo "FAIL: the source-and-owner paragraph does not name s8h, which adds severity to rk-types Finding (D21-47)"; exit 1; }
CREPO=$(sed -n 's/^CORPUS_REPO=//p' CORPUS_PIN)
test -d "$CREPO" || { echo "FAIL: the corpus repository '$CREPO' is absent (Halt 8)"; exit 1; }
lab_sha=$(sed -nE 's/^\*\*S-SEV labels commit:\*\* `([0-9a-f]{40})`.*$/\1/p' "$recs")
[ "$(printf '%s\n' "$lab_sha" | grep -c .)" -eq 1 ] || { echo "FAIL: $recs does not name exactly one S-SEV labels commit"; exit 1; }
git -C "$CREPO" cat-file -e "$lab_sha:results/sev-f37-labels.md" || { echo "FAIL: the labels are not at the corpus commit the record names ($lab_sha)"; exit 1; }
if git -C "$CREPO" cat-file -e "$lab_sha^:results/sev-f37-labels.md" 2>/dev/null; then echo "FAIL: the labels already existed before $lab_sha — the record names a later commit, not the one that added them"; exit 1; fi
lab_at=$(git -C "$CREPO" show -s --format=%ct "$lab_sha"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$lab_at" ] || { echo "FAIL: the labels commit's time could not be read (rc=$rc)"; exit 1; }
[ "$pre_at" -lt "$lab_at" ] || { echo "FAIL: the labels were committed before the pre-registration ($lab_at <= $pre_at) — a method written after the data is not a pre-registration"; exit 1; }
# Every METHOD-SEV commit AFTER the labels is append-only and adds a dated amendment: a post-data rewrite of a
# pre-registration is not one.
git log --format='%H %ct' -- spikes/METHOD-SEV.md > target/cp8b/method-sev-log.txt || { echo "FAIL: METHOD-SEV's history could not be read"; exit 1; }
while read -r h ct; do
  [ "$ct" -ge "$lab_at" ] || continue
  ns=$(git diff --numstat "$h^" "$h" -- spikes/METHOD-SEV.md); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the diff of $h could not be read (rc=$rc)"; exit 1; }
  del=$(printf '%s\n' "$ns" | awk '{print $2}')
  [ "$del" = 0 ] || { echo "FAIL: METHOD-SEV was rewritten after the labels ($h deletes $del lines) — an amendment after the data is append-only"; exit 1; }
  dif=$(git diff "$h^" "$h" -- spikes/METHOD-SEV.md); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: the diff of $h could not be read (rc=$rc)"; exit 1; }
  printf '%s\n' "$dif" | grep -qE '^\+[[:space:]-]*\*Amendment [0-9]+ \(20[0-9]{2}-[0-9]{2}-[0-9]{2}' || { echo "FAIL: METHOD-SEV changed after the labels ($h) without a dated amendment"; exit 1; }
done < target/cp8b/method-sev-log.txt

# ---- (10) AC-527 IN THE REGISTER — a FLOOR over a document, beneath the behavioural fixtures in (5). The contradicting
# ---- clauses must be GONE or STRUCK (`~~…~~`), not merely joined by a new note (review M3); the control is the
# ---- unamended register at ca8f387, where the same reading must find them.
ac527=$(awk '/^\*\*AC-527\*\*/ {f=1} /^### 4\.6/ {f=0} f' references/2026-09-08-acceptance-criteria.md)
test -n "$ac527" || { echo "FAIL: AC-527's entry was not found in the register (Halt 8)"; exit 1; }
printf '%s\n' "$ac527" | grep -qF "findings.all(not (evidence in ['Reproduced', 'MutationSurvived']))" || { echo "FAIL: AC-527's Verified-by does not name the recast fixture 2"; exit 1; }
printf '%s\n' "$ac527" | grep -qF 'D21-41 (h)' || { echo "FAIL: AC-527 carries no dated amendment citing D21-41 (h)"; exit 1; }
unstruck() { tr '\n' ' ' | sed -E 's/~~[^~]*~~//g'; }
flat=$(printf '%s\n' "$ac527" | unstruck)
old=$(git show ca8f387:references/2026-09-08-acceptance-criteria.md); rc=$?
[ "$rc" -eq 0 ] && [ -n "$old" ] || { echo "FAIL: the unamended register could not be read at ca8f387 (rc=$rc)"; exit 1; }
ctl=$(printf '%s\n' "$old" | awk '/^\*\*AC-527\*\*/ {f=1} /^### 4\.6/ {f=0} f' | unstruck)
for phrase in 'whose truth over an **empty** findings set is `true`' 'never on the absence of'; do
  printf '%s\n' "$ctl" | grep -qF "$phrase" || { echo "FAIL: control — the unamended AC-527 does not carry '$phrase'; this guard is vacuous"; exit 1; }
  if printf '%s\n' "$flat" | grep -qF "$phrase"; then echo "FAIL: AC-527 still states '$phrase' unstruck — it refuses what ADR-0080 d2 permits (review M3)"; exit 1; fi
done

# ---- (11) ADR-0101 CLAUSE 5 / ADR-0162: no calibration claim in what this stage touched. Silence is the pass. The
# ---- tree walk is (4)'s cal_walk, whose worktrees/ exclusion (4)'s shaped control proved. ADR-0162 forbids the claim
# ---- "in any document, record, commit message, fixture or test name", so the walk also reads the S-SEV record,
# ---- METHOD-SEV, the METHOD-P amendment, the AC-101/102/527 entries and every commit message since ca8f387 (review M4).
raw=$(cal_walk); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the calibration sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a calibration claim — S-SUBTLE-CB is UNRUN:"; printf '%s\n' "$raw"; exit 1; }
ext=target/cp8b/cal-extra.txt
cat "$recs" spikes/METHOD-SEV.md > "$ext" || { echo "FAIL: cannot collect the S-SEV record and METHOD-SEV"; exit 1; }
amend=$(awk '/^[[:space:]-]*\*Amendment [0-9]+ \(2026-[0-9]{2}-[0-9]{2}, D21-43 N26\):\*/ {f=1; print; next} f && /\*Amendment [0-9]+ \(/ {f=0} f' spikes/METHOD-P.md)
test -n "$amend" || { echo "FAIL: METHOD-P's N26 amendment text could not be extracted — the walk would read nothing of it"; exit 1; }
printf '%s\n' "$amend" "$ac527" >> "$ext" || { echo "FAIL: cannot collect the amendment and AC-527"; exit 1; }
for a in AC-101 AC-102; do
  ent=$(awk -v k="**$a**" 'index($0, k) == 1 {f=1; print; next} f && /^\*\*AC-/ {f=0} f' references/2026-09-08-acceptance-criteria.md)
  test -n "$ent" || { echo "FAIL: $a's entry was not found (Halt 8)"; exit 1; }
  printf '%s\n' "$ent" >> "$ext" || { echo "FAIL: cannot collect $a"; exit 1; }
done
msgs=$(git log --format=%B ca8f387..HEAD); rc=$?
[ "$rc" -eq 0 ] && [ -n "$msgs" ] || { echo "FAIL: the commit messages since ca8f387 could not be read (rc=$rc)"; exit 1; }
printf '%s\n' "$msgs" >> "$ext" || { echo "FAIL: cannot collect the commit messages"; exit 1; }
hits=$(grep -inE "$CAL" "$ext"); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the widened calibration sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$hits" ] || { echo "FAIL: a calibration claim in a record, amendment or commit message — S-SUBTLE-CB is UNRUN:"; printf '%s\n' "$hits"; exit 1; }
syn=$(printf '%s\n' 'the gate is now Calibrated against false approval' | grep -iE "$CAL")
test -n "$syn" || { echo "FAIL: the calibration pattern does not match a claim — this sweep is vacuous"; exit 1; }

# ---- (12) THE WHOLE SUITE, with a passed FLOOR. Expected value, re-derived at stage open from Checkpoint 8a's recorded
# ---- count: 749 + 25 = 774 (749 = ADR-0164's 740 at 992d257 + s8a's 9; 25 is this stage's new tests — the 21 as
# ---- drafted, D21-47's three lattice fixtures (review M9) and p9_pin's test (V2)). Write the re-derived value here, by
# ---- a dated edit, before the first run. At least 44 result lines, as at 992d257.
FLOOR=774
out=$(nodispatch cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|test result: FAILED' | head -20; exit 1; }
passed=$(printf '%s\n' "$out" | awk '/^test result: / {for (i = 2; i <= NF; i++) if ($i == "passed;") s += $(i - 1)} END {print s + 0}')
[ "$passed" -ge "$FLOOR" ] || { echo "FAIL: the workspace passed $passed tests, below the floor of $FLOOR — tests were lost or not run"; exit 1; }
nres=$(printf '%s\n' "$out" | grep -c '^test result: ')
[ "$nres" -ge 44 ] || { echo "FAIL: $nres test result lines, below the 44 of 992d257 — a target stopped being built or run"; exit 1; }
out=$(nodispatch cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(nodispatch cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
nd_check "the whole suite, fmt and clippy" || exit 1

echo "CP8B-END-OF-BLOCK reached"
```

**Dependencies this block names, so that a red is read correctly.**
- Section (0) assumes that `s8a`'s file is `runbook/phase-1/s8a-records-and-red-checkpoints.md`, with a
  `### 8a.N Checkpoint 8a` heading whose last bash block ends in `echo "CP8A-END-OF-BLOCK reached"`, as the
  convention of Checkpoints 7h and 7i has it. *(amended 2026-09-24, final consistency pass)* Checkpoint 8a needs no
  environment from this block: its record ids, its base commit and its `FIX_ITEM` are literals in its guard (2).
- **Section (W)** is the shared `runbook/tools/nodispatch.sh` *(re-pointed 2026-09-24, final consistency pass)*. Its
  `nd_setup` reads the claude pin from `crates/rk-adapter-claude/src/config.rs`'s anchored
  `pub const TESTED_CLI_VERSION: &str = "…";` line, compiles `runbook/tools/fake-claude.rs` with `rustc`, and writes
  the fake, its call log and the trap under `target/cp8b/nd/`.
- **Sections (2), (2b), (3d) and (7) read `spikes/p9` at `P9_PIN`** from the object store (D21-47 V2). A shallow
  clone fails at section (2), naming the pin. When `s8j`'s story lane edits `spikes/p9/packs/spike.toml`, nothing in
  this block moves. Section (2)'s working-tree guard deliberately excludes `spikes/p9/packs/`.
- **Section (7) re-runs the Python oracle**, which takes about 2 min 10 s and spends nothing, into `target/cp8b/`. It
  needs the same `python --version` as the goldens record. A new interpreter is a regeneration at §8b.2 step 1's
  route, in its own dated commit, and not a silent pass.
- **`NDIV=6` in section (7) and `SSEV_ADR=` in section (9)** are literals written by dated edits: the first when the
  port is traced, the second when the record is written. The placeholder `____` fails by design until then.
- Section (4) scopes `--exclude-dir=worktrees` to its `.redkiln/` call, because `crates/` has three fixture
  directories of that name (§8b.3), and it asserts that the `crates/` call still reaches `tests/it/quality_block.rs`.
- **Section (4) writes nothing under the real `.redkiln/`.** Its planted control lives in `target/cp8b/shape/`, a
  scratch tree shaped like `.redkiln/`, and is removed afterwards. It asserts D21-44's `.gitignore` row
  `.redkiln/worktrees/` with `git check-ignore` on a path that need not exist.
- Section (0) reaches Checkpoint 7f only through Checkpoint 8a. After `s8h` lands, a whole re-run of this block needs
  `target/debug/rk-fixture-child-claude` in `[adapters] harness_binary` first (§8b.9). Section (0) launches 8a with
  `RK_CP_NESTED=1`, so 8a skips its own whole re-runs (7i, 5, 7a, 7b, 7f), and it launches it under (W)'s wrapper.
- The (C) rows' `discriminating_test` binding is refused by name until `s8h` carries the field (§8b.5). No section of
  this block evaluates it, so it cannot turn this block red or green.
- **A further parity test** (`lattice::p6_lattice_parity_over_the_shipped_pack`, review N3's "27th", if §8b.2 step
  2's Python run over `.redkiln/` completes) joins `K_TESTS`. The `30` in section (5) and the floor in section (12) then move by one, by a
  dated edit in the same commit (review N3). The predicted branch is NOT MEASURABLE, which adds nothing.

---
