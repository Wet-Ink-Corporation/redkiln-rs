# Step 8b — The catalog's quality instruments over the shipped pack (Phase 1, Step 8 sub-stage b)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8b` | Phase 1 | Checkpoint 8a | Checkpoint 8b | `s8a` | `s8c` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

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
to d3's threshold. The AC-527 lint that would catch the rows has never read the shipped catalog: it pins
`const SHIPPED_VIOLATIONS: usize = 11;` over `spikes/p9/catalog`, and its own module says the reconciliation with
d2 *"is OWED and is not written here"*.

**This step takes one record, and puts four questions to the operator before it edits anything.** The record is
11c's: the severity vocabulary and ADR-0080 d3's threshold T, written on a measurement (§8b.6), as a new ADR at the
next free id at writing, in ADR-0085's `### Option — ` form (D21-42 24a). §8b.1's four questions are free residuals
that this file found by reading the tree; none is decided here. The AC-527 change is a dated register amendment
and not an ADR, because no Decision sentence or forbids bullet moves (packet (h) option A, *"There is no new
ADR"*). **No real dispatch happens at this stage.** The labelling in §8b.6 is done by the operator over recorded
data.

**Acceptance set.** AC-101–AC-105, AC-303 and AC-527 (as amended by §8b.4), minus:
- **AC-116**: the diagnostic on AC-104's failing report is not ported, because `lattice.py` has no such output;
- **AC-111 and AC-117**: the CI job that joins digests to versions and changelog rows is not built here. ADR-0072 is
  executed by hand for seven edits and asserted by a named test over those seven, which is not AC-111's instrument;
- **AC-018**: the memo lint is an unassigned item (§8b.1 question 4);
- AC-101's and AC-102's **kill-line verdicts over the installed pair**: they are measured and recorded with the
  pack count. A fired line is a STOP (§8b.2), not a pass.

### 8b.1 Before anything moves: the inventory, the BEFORE readings, and four questions for the operator

**The instruments that read `spikes/p9/`**, re-measured at stage open from code lines:

| instrument | test | what it reads today | this stage |
| --- | --- | --- | --- |
| AC-105 rigor monotonicity | `rigor::rigor_monotonicity` | `spikes/p9/catalog` + `spikes/p9/facets.toml`, pinned `(15, 2060, 1729)` | re-pointed to `.redkiln/`; the p9 pin moves to a historical test, named so (§8b.3) |
| AC-303 confirm-step monotonicity | `review::ac303_confirm_step_survives_escalation` | `spikes/p9/catalog`, floors only (`families >= 10 && comparisons > 1_000`) | re-pointed; no historical split, because it pins no figure |
| AC-527 terminal-condition lint | `quality_block::ac527_empty_set_terminal_condition` | `spikes/p9/catalog`, `11` refused / `83` indeterminate | re-pointed and amended (§8b.4); the frozen p9 catalog stays RED in a separate, named control |
| P6 lattice (AC-101–AC-104) | `spikes/p9/lattice.py` | its own directory only | ported (§8b.2) |
| AC-018 memo lint | `memo::…` (the `live_verb_memo_rule` assertion) | `spikes/p9/catalog`, `19` violations *"RED by design"* | **not assigned** (question 4) |

**The test names are kept.** Checkpoint 7b's loop selects `rigor_monotonicity` and `ac527_empty_set_terminal_condition`
with `grep -qE "^test .*\b${t}\b.* \.\.\. ok$"`, so renaming either would turn a checkpoint red for a reason
unrelated to the rule. The historical tests get new names that `\b` does not match (`_p9_…` suffixes).

**Four questions, put to the operator at stage open with their measured premises, and ruled on a dated ledger line
before the step they gate.** None is taken by this file.

1. **Which corpus does 11c's labelling read?** D21-42 11c says *"the F37/F39 review corpus"*. **F39 holds no review
   findings.** Its header is *"F39 — P1 arm D RESULT: no derived facet clears §1.4's 0.95 bar"*, and it is a
   facet-precision measurement. The review-finding corpora are F37's (`results/p7.jsonl` in `redkiln-corpus`, 33
   adversary dispatches, 188 findings) and the six E3 findings that ADR-0080 d3 names through F24. §8b.6 proposes
   F37's 188 as the pre-registered corpus. **Gates §8b.6.**
2. **What replaces the policy predicate in five of the seven rows?** §8b.5 derives two replacements from the rows'
   own steps: `root-cause`, and `adversarial-review` as 11a words it. For the other five, no quantity in the row's
   own steps is both a d2 form and varied by the loop. §8b.5's table gives the measured reason and the lawful
   options for each row. **Gates §8b.5's edits to those five.**
3. **Which lattice axes run over `.redkiln/`?** `lattice.py`'s toolchain axis is
   `TOOLCHAINS = OrderedDict([("node", …), ("unreal", …), ("node+unreal", …), ("dotnet", …)])`. **It has no `rust`**,
   although `.redkiln/facets.toml` declares `values = ["node", "dotnet", "python", "rust", "unreal"]` and Rust is the
   only toolchain Phase 1 ships.
   Its context also binds no `surfaces`, but `.redkiln/catalog/threat-model.toml` reads
   `applicable_if = "'auth' in surfaces or …"`. The spike copy reads `"data_migration or external_dep"`. A faithful
   port therefore walks the shipped pack without its own toolchain, and it cannot evaluate one shipped predicate
   *[inferred; measured at §8b.2 step 2]*. Extending the axes changes METHOD-P §3's pre-registered lattice, and that
   is a record, not a port. **Gates the `.redkiln/` half of §8b.2.**
4. **Does AC-018 run over `.redkiln/`?** 13a's parenthetical names three instruments. The memo lint is a fourth
   catalog lint that reads `spikes/p9`. Over `.redkiln/` it would be RED: `grep -rn '^memo\s*=' .redkiln/catalog`
   finds **19** rows in **14** files, the same 19 that `memo.rs` pins over p9. Repairing them is 14 ADR-0072 edits
   that no D21 row assigns. **Gates nothing in this stage; it is recorded in the D-entry.**

**BEFORE readings (PIA-035), each a line that fails on the untouched tree.**
- `grep -rnE "findings\.none\(evidence in \[" .redkiln/catalog` returns **7** live `expr` rows.
- `quality_block.rs` points at `spikes/p9/catalog`.
- No `crates/rk-kernel/tests/it/lattice.rs` exists.
- `git ls-files | grep -ic changelog` is **0**: ADR-0072's "pack changelog" has never been written.

### 8b.2 The P6 lattice in Rust (D21-42 13b), and parity before the Python is demoted

**What is ported, clause for clause.** The module is `rk_kernel::lattice`. It ports `lattice.py` v2 as it stands:
- interpretations I1–I9 and decisions D1–D9, as its module doc enumerates them;
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

1. **Regenerate the goldens at stage open, from the Python, over `spikes/p9`, and time them for SM-025.** The Python
   prints `×` and writes through a text-mode stream, so on Windows the bytes would be CRLF and possibly not UTF-8.
   Write through a reconfigured stdout, so that the file is the program's bytes:
   ```bash
   cd D:/repos/redkiln-rs
   mkdir -p crates/rk-kernel/tests/fixtures/lattice || { echo "FAIL: cannot create the golden directory"; exit 1; }
   PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py']; runpy.run_path('spikes/p9/lattice.py', run_name='__main__')" > crates/rk-kernel/tests/fixtures/lattice/p9-run.txt; rc=$?
   [ "$rc" -eq 0 ] || { echo "FAIL: the Python lattice exited $rc — there is no oracle to port against"; exit 1; }
   PYTHONIOENCODING=utf-8 python -c "import sys,runpy; sys.stdout.reconfigure(newline='\n'); sys.argv=['lattice.py','--validate']; runpy.run_path('spikes/p9/lattice.py', run_name='__main__')" > crates/rk-kernel/tests/fixtures/lattice/p9-validate.txt; rc=$?
   [ "$rc" -eq 0 ] || { echo "FAIL: lattice.py --validate exited $rc"; exit 1; }
   grep -qF "packs=['bug-expedite', 'bug-standard', 'chore', 'docs', 'incident', 'initiative', 'project', 'security-vuln', 'spike', 'story-ue', 'story', 'tech-debt']" crates/rk-kernel/tests/fixtures/lattice/p9-run.txt || { echo "FAIL: the golden does not name the twelve packs — a zero-pack run prints a whole baseline and exits 0"; exit 1; }
   ```
   Every figure in the table above must reappear in `p9-run.txt` after its `| ` separator. The `cat:`/`fac+art:`/`pk:`
   digest tags may differ from run10's, because ADR-0164 (still `proposed`) renormalised line endings at `ac10470`,
   and the tags hash raw bytes. Record both sets of tags. If a figure moved, STOP: the oracle is not the one the
   register quotes.
2. **Measure whether the Python can read `.redkiln/` at all.** Stage `.redkiln/{catalog,packs,facets.toml,artifacts.toml}`
   beside a copy of `lattice.py` in a scratch directory, because `HERE` is the script's own directory, and run it
   there. The prediction is a failure on `'auth' in surfaces` *[inferred]*. If it completes, a third golden
   (`redkiln-run.txt`) and `lattice::p6_lattice_parity_over_the_shipped_pack` join the set. If it fails, the failure
   is recorded verbatim, parity over `.redkiln/` is **NOT MEASURABLE** by this oracle, and the Rust run over
   `.redkiln/` stands on its own pinned figures. Neither outcome is a pass or a fail of the port.
3. **The goldens are byte-pinned.** A `.gitattributes` row `crates/rk-kernel/tests/fixtures/lattice/** -text` is
   added on ADR-0151's route, and the checkpoint reads the raw bytes with `od -c`, because Git Bash cannot see a CR
   inside `$(…)`.
4. **The Rust port renders the same report lines, digest tags included.** The tag is `sha256(basename ‖ bytes)[:12]`
   over sorted paths, which the port computes identically. `lattice::p6_lattice_parity_over_the_p9_tree` and
   `lattice::p6_validate_parity_over_the_p9_tree` assert equality with the goldens. **A tag is part of the claim**:
   parity is about these bytes. When `s8j` hardens `spikes/p9/packs/spike.toml` (D21-41 f), `pk:` moves and both goldens
   are regenerated by the Python in that same commit. That is why the Python is demoted and not deleted.
5. **Then the Python is demoted.** From the parity commit on, the instrument of record for AC-101–AC-104 is
   `rk_kernel::lattice`. AC-101 and AC-102 each take a dated register note that names the tests. The note records the
   Rust reading over `.redkiln/` with **its pack count (2) and rigor mode (`pack-default`)**, beside the retained
   twelve-pack p9 figures (DECISIONS-2026-09-12 E7's two numbers), and the same reading is recorded against
   SM-026/SM-027. `lattice.py` is not edited. It stays the oracle for regenerating the p9 goldens.

**Over `.redkiln/`**, which is gated on §8b.1 question 3:
- `lattice::p6_lattice_over_the_shipped_pack` runs the port over both installed packs. From inside the test binary it
  prints `lattice-shipped: packs=<n> contexts=<n> pairs=<n> unsat=<n> dead=<n>/102 ties=<n> hard_cycles=<n>`.
- It asserts `packs == 2` and `contexts > 0`. It also asserts that ties and hard cycles are 0 (AC-103's zero
  tolerance), and it pins the measured shape.
- `lattice::p6_validate_over_the_shipped_pack` asserts AC-104's totality with a non-zero contexts-checked per family.
- `lattice::zero_packs_is_refused_not_reported` pins the shape `lattice.py --packs <path>` exhibits. Selecting zero
  packs is a typed `Err`, never a report.

🛑 **AC-101's and AC-102's kill lines over the installed pair are measured, not assumed.** Two packs of twelve select
from a 102-activity catalog, and the eight-pack p9 reading is already `dead=14/102 (13.7%)`. So a dead fraction
above AC-102's `>20%` is the likely reading *[inferred]*. If a kill line fires over the installed pair, the test pins
the figure and STOPS the stage for the operator. Thresholding it away and quoting the twelve-pack figure in its
place are the same act.

**CI.** The tests run in `cargo test --workspace` on both OS legs. `ci.yml` also gains one named step, shaped like
the `rk-ports conformance bodies` step: it runs `lattice::p6_lattice_over_the_shipped_pack` module-qualified and
`--exact`, and asserts its `... ok` line, because a filter that matches nothing exits 0. **Pushing is not this
stage's.** D21-42 14a makes every push an operator question at `s8j`.

**The two red fixtures the thresholds need** (AC-101's and AC-103's Verified-by):
- `lattice::unsatisfiable_fixture_names_its_cell`: a fixture catalog whose require row has no producer. The report
  names the (facet-vector × rigor) cell and the row.
- `lattice::rank_tie_fixture_is_an_error`: two family members at one `method_rank`.

### 8b.3 Every catalog-quality instrument over `.redkiln/` (D21-42 13a)

- **`rigor::rigor_monotonicity`** reads `.redkiln/catalog` and `.redkiln/facets.toml`. Its red end, the authored
  pre-P7-CEIL fixture catalog, is unchanged: the fixture `bisect.toml` is compared with the shipped `bisect.toml` minus
  its ceiling, and `.redkiln/catalog/bisect.toml` is byte-identical to the p9 copy up to line endings (it is not in the
  diff list). Its green end's `(families, contexts, comparisons)` triple is **re-derived from inside the test binary**
  and pinned. It is not carried over from p9, because the registries differ. Its `println!` names its catalog path
  (`rigor_monotonicity: catalog=.redkiln/catalog …`) so that the checkpoint can read the path from the binary.
- **`rigor::rigor_monotonicity_p9_historical_2026_09_13`** keeps `(15, 2060, 1729)` over `spikes/p9`. Its doc names
  it a historical measurement and cites the date it was taken.
- **`review::ac303_confirm_step_survives_escalation`** is re-pointed to `.redkiln/`. It keeps its floors.
- **`quality_block::ac527_empty_set_terminal_condition`** is re-pointed, per §8b.4.
- **Every re-pointed test asserts a non-zero count walked** (activities, families, contexts, or check steps
  evaluated) **before** it asserts its verdict. A green verdict over zero is Halt 8's shape.

The rewrites in §8b.5 change only `expr` and `inputs` on `check` steps and add no step. `evidence()` reads
the guaranteed step set, so the monotonicity figures should not move between the re-point and the rewrites
*[inferred]*. They are re-read after the rewrites anyway, and any movement is recorded with its cause.

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
- `quality_block::ac527_frozen_p9_catalog_is_red`: the unedited `spikes/p9/catalog` stays RED under the amended rule,
  with its count re-measured and pinned. It is a control over a frozen tree, and its name says so.

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
  iteration's findings — chosen so that a reviewer doing its job can terminate the block"*. The refusal lint of §8b.4
  is the floor.
- **(b) It is not a constant function of its operands' domains.**
  `quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity` evaluates each new `expr` across its
  bindings and requires both truth values. The design's golden constraint 15: *"A policy predicate that is a constant function of
  its arm carries no information and may not terminate anything."*
- **(c) For a loop-back row, it reads a quantity the loop varies.**
  `quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies` requires that every binding the `expr` reads comes
  from the `on_false.to` target, or from a step whose inputs reference that target's outputs. (b) cannot see a quantity
  that is variable in the grammar and fixed in practice. (c) can.

| row | step | version | current `expr` (quoted) | replacement | status |
| --- | --- | --- | --- | --- | --- |
| `root-cause` | `policy` | 4 → 5 | `"findings.none(evidence in ['Reproduced', 'MutationSurvived'])"` | inputs `{ report = "${confirm.outputs}", findings = "${adversary.outputs?}" }`, `expr = "report.failed > 0"`. The quantity is bullet 2's re-execution outcome: `confirm` re-runs `${hypothesise.outputs.discriminating_test}`, which `hypothesise`, the loop-back target, varies | derived here |
| `adversarial-review` | `verdict` | 3 → 4 | `"findings.none(evidence in ['Reproduced', 'MutationSurvived']) and (rigor < mandatory or findings.none(evidence in ['NarrowedPin']))"` | D21-42 11a verbatim: *"no blocking finding survives re-execution"*, read from `review::confirm`'s outcome. The input binds the `Confirmed` for this activation, and the `expr` is over its block set (`Confirmed::block_set()`). `on_false = "continue"` is unchanged | ledger-specified; binding spelling measured, see below |
| `differential-debugging` | `policy` | 2 → 3 | the same two-member form | **none derivable.** `confirm` runs `artifact:ReproCase.command`, a fixed input the loop does not vary, so it fails (c) | question 2 |
| `instrumented-trace-analysis` | `policy` | 2 → 3 | `"cause.confidence == 'high' and findings.none(evidence in ['Reproduced'])"` | **none derivable.** `confirm` runs `artifact:ReproCase.command` (as above), and `cause.confidence` is the investigator's self-report | question 2 |
| `threat-model` | `policy` | 1 → 2 | `"findings.none(evidence in ['Reproduced'])"` | **none derivable.** There is no tool step, so no re-execution exists. `review::confirm`'s re-executor gives `Unverifiable::CannotRun` for a repro with no `--` selection (ADR-0162 d4), so "no blocking finding survives re-execution" would be **constantly true** | question 2 |
| `user-story-ac` | `policy` | 2 → 3 | `"check.failed == 0 and findings.none(evidence in ['Reproduced'])"` | **none derivable as re-execution.** There is no tool step. `check.failed == 0` alone passes (b) and (c), but it is a self-check count, and d2's dash list may be exhaustive | question 2 |
| `vertical-slicing` | `policy` | 1 → 2 | `"check.failed == 0 and findings.none(evidence in ['Reproduced'])"` | as `user-story-ac` | question 2 |

**Question 2's options, per row, for the operator.** The recommendations are this file's reading and are not
taken:
- **(A)** a bullet-1 threshold at 11c's T, once §8b.6's record fixes one. Those rows then wait for it.
- **(B)** a bounded loop-back count. `true` with `max` is constant and fails (b), so this needs a counted quantity
  the grammar can bind.
- **(C)** re-point `confirm`'s `selection` at the investigator's discriminating test, as `root-cause` does. This is
  a semantic edit of what `confirm` runs. It is recommended for `differential-debugging` and
  `instrumented-trace-analysis`.
- **(D)** drop the findings conjunct and keep `check.failed == 0`. This is lawful only if bullet 2's dash list is
  read as illustrative. It is recommended for `user-story-ac` and `vertical-slicing`, with that reading recorded.
- **(A)** is recommended for `threat-model`.

**11b's words, "re-execution quantities", fit one of the four one-member rows at most.** Recorded here, and reported
rather than smoothed over.

**`adversarial-review`'s binding.** The review pass runs in `rk-cli` after the activity commits (ADR-0162 d3/d6). So
no step output carries `Confirmed` during the walk today, and resolving one belongs to `s8d`'s resolver (D21-41 d).
The spelling is fixed in the commit that also keeps `conformance::shipped_pack_loads` green. If no spelling loads
without a resolver change, the row joins question 2. Until the binding resolves, ADR-0136 refuses the step by name.
The check never evaluates an absent value as true.

**Nothing here claims calibration.** Not in the rows, the changelog, or the tests. ADR-0162 forbids *"any claim about
the review gate's calibration, in any document, record, commit message, fixture or test name"*.

**ADR-0072, executed.** *"Edits to shipped activities made for catalog coherence bump `version` and are listed in
the pack changelog. The P6 lattice is the test that such an edit did not break reachability."*
- Each row's `version` moves as in the table.
- `.redkiln/CHANGELOG.md` is created. Nothing names the file, so the path is chosen here and said. It gets one row
  per edit: `<id>@<new version>`, the date, D21-41 (h) or D21-42 11a/11b, the old and new `expr` quoted, and for
  `root-cause` the line *"detached from F37's pinned artefact `306b58ba7504` (v4); F37's measurements are of v4"*.
- `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` pins the seven `(id, old, new)` triples and
  requires a changelog row for each.
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

- **Corpus** (§8b.1 question 1): F37's **188** findings from 33 adversary dispatches, read through the object store,
  `git -C "$CORPUS_REPO" show "$CORPUS_SHA":results/p7.jsonl`, as AC-464 requires, never from a working tree.
  **The six E3 findings are excluded** from the pre-registered analysis. `e3-severity-annotation-ryan.md` is
  *"Filled by the model at Ryan's direction, 2026-09-04. NOT BLIND."*, and its correction carries
  *"countersign: Ryan Britton, 2026-09-11"*. The operator has read a filled sheet, so his annotation of the preserved
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
  - **A1 passes, and A2 and A3 show variability:** the vocabulary is `blocking | advisory`, T = `blocking`, and the
    record also names the field's source and owner.
  - **A1 passes, and A2 or A3 fails:** the vocabulary is fixed, and T is *"no admissible severity threshold on this
    evidence; bullet 2 governs"*. That is a settled T, not a deferral.
  - **A1 fails:** STOP.
  - **In every branch**, if the labels contradict ADR-0074's `narrowed-pin` row, the record **reports** the
    contradiction and does not move the row. A Decision sentence of an accepted record moves only by supersession
    (ADR-0092 decision 1), on the operator's word.
- **Where the artefacts live.** The filled sheets (the first pass as `results/sev-f37-labels.md`, which Checkpoint 8b
  section (9) reads by that name), the re-label and the scored result are committed to `redkiln-corpus` as a new
  commit, without moving `CORPUS_PIN`. That is O-9's precedent, and AC-465 bars tracked
  spike results under `spikes/results/`. The record names that commit's sha.

**The record.** A new ADR at the next free id at writing, `status: proposed`, with `evidence: ["S-SEV", "F37"]`,
ADR-0085's `### Option — ` headings (D21-42 24a), and a **What this forbids** block.
- It supersedes nothing. ADR-0080 d3 contemplates exactly this record and forbids none of it.
- It flips only on the operator's dated ledger line.
- It carries dated notes on the design §1.2 rows that read *"plus a settled severity vocabulary"*.
- It makes no claim about the review gate's calibration (ADR-0101 clause 5).

### 8b.7 The D-entry, and the guard form `s8j` adopts

**The stage's ledger entry** is the next free D21 id at writing. It **closes D21-2's RAISED reading**: an `expr` on a
`kind = "check"` step that carries `on_false` is the terminal condition ADR-0080 d2 forbids, settled by ADR-0103 d3,
ADR-0080's Context and design §1.2's *"P7's policy predicate"* row. It also records:
- `adversarial-review`'s disposition (i), taken by D21-42 11a;
- that the one-member rows are refused under the forbids bullet's `findings.none(evidence in [...])`, with the
  disclosure the packet asked for: this **extends F37's measurement**, which was of the two-member predicate on one
  graph;
- the operator's rulings on §8b.1's four questions;
- `root-cause` detached from `306b58ba7504`;
- the parity figures and the Python's demotion;
- the Rust lattice's wall time (SM-025).

**The guard form this stage makes true.** `s8j` widens Checkpoint 8's quality-shorthand block to it. This file does
not edit `s8`. It replaces the unquoted-only pattern, keeps the `quality_shorthand` alternation, drops code comments,
and excludes by path, with each exclusion's reason stated:
- `tests/it/evaluator.rs` and `tests/it/quality_block.rs` are the enforcement's own literals;
- `tests/fixtures/catalog/{root-cause,differential-debugging,instrumented-trace-analysis}.toml` are the frozen
  pre-P7-CEIL red end of `rigor_monotonicity` and AC-303;
- the four must-REFUSE activity fixtures.

`adversarial-review` needs no by-name exclusion, because it was rewritten. Checkpoint 8b section (4) is that form,
run here, with its synthetic control.

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

### 8b.9 Checkpoint 8b

- **Checkpoint 8a holds**, re-run whole from its own stage file's bytes.
- **The P6 lattice is in Rust**, with parity against the Python proven byte for byte over `spikes/p9`. It runs over
  `.redkiln/` with **2** packs walked, and in CI by name.
- **Every catalog-quality instrument reads `.redkiln/`** and prints the path it read from inside the test binary.
  `spikes/p9` survives only in tests named historical or frozen.
- **No live line carries ADR-0080 d2's named form in any quoting.** This is `s8j`'s guard form, run here.
- **The seven rows are at their new versions, with changelog rows.** The amended lint refuses nothing in
  `.redkiln/catalog` and still refuses the frozen p9 catalog.
- **11c's record exists, and its measurement was pre-registered before the labels existed.**
- **AC-527 is amended to d2 in the register.** Nothing claims the review gate's calibration.
- The whole suite, fmt and clippy are green.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Carried from Checkpoint 7i and not re-argued: every cargo reading is `out=$(…); rc=$?`, never a pipe's status;
# every selection is `--test it`, MODULE-QUALIFIED and `--exact`, and asserts each test's own `... ok` line AND an
# exact count; every grep over source reads CODE lines only and carries a synthetic control proving it can go red.

# ---- (0) THE ENTRY GATE, RE-RUN WHOLE from its own stage file's bytes (D21-40's method). An inherited gate is a
# ---- claim. The fence is matched as a bracket class so that this block contains no literal fence of its own.
S8A=runbook/phase-1/s8a-records-and-red-checkpoints.md
test -f "$S8A" || { echo "FAIL: $S8A is absent — Checkpoint 8a cannot be re-run (Halt 8)"; exit 1; }
cp8a=$(mktemp) || { echo "FAIL: mktemp could not create the extraction file"; exit 1; }
awk '/^### 8a\.[0-9]+ Checkpoint 8a/ {f=1} f && /^[`][`][`]bash/ {b=1; body=""; next} b && /^[`][`][`]/ {b=0; last=body; next} b {body = body $0 "\n"} END {printf "%s", last}' "$S8A" > "$cp8a" || { echo "FAIL: Checkpoint 8a could not be extracted"; exit 1; }
test -s "$cp8a" || { echo "FAIL: no bash block follows Checkpoint 8a's heading — nothing would be re-run"; exit 1; }
grep -qxF 'echo "CP8A-END-OF-BLOCK reached"' "$cp8a" || { echo "FAIL: the extracted block does not end in CP8A's marker — it is not Checkpoint 8a"; exit 1; }
out=$(bash "$cp8a" 2>&1); rc=$?
rm -f "$cp8a"
[ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint 8a, re-run whole, exited $rc — the entry gate does not hold"; printf '%s\n' "$out" | tail -25; exit 1; }
printf '%s\n' "$out" | grep -qxF 'CP8A-END-OF-BLOCK reached' || { echo "FAIL: Checkpoint 8a exited 0 without reaching its end marker"; exit 1; }

# ---- (1) BUILD-OR-FAIL.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8b NOT RUN (Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }

# ---- (2) ANCHORS FIRST: the subject exists and is whole, and the frozen oracle's inputs are the ones authored against.
test -d .redkiln/catalog || { echo "FAIL: .redkiln/catalog is absent (Halt 8)"; exit 1; }
n=$(find .redkiln/catalog -maxdepth 1 -type f -name '*.toml' | grep -c .)
[ "$n" -eq 102 ] || { echo "FAIL: .redkiln/catalog holds $n activities, not 102 — a subset cannot stand for the shipped pack"; exit 1; }
git diff --quiet ca8f387 -- spikes/p9/catalog spikes/p9/facets.toml spikes/p9/artifacts.toml spikes/p9/lattice.py; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the frozen p9 catalog, registry or lattice changed since ca8f387 (rc=$rc) — the historical subject and the parity oracle may not be edited to make a green"; exit 1; }
grep -qF "findings.none(evidence in ['Reproduced', 'MutationSurvived'])" spikes/p9/catalog/root-cause.toml || { echo "FAIL: the frozen p9 root-cause no longer carries ADR-0080's predicate — the red control has nothing to be red about"; exit 1; }

# ---- (3) THE SEVEN ROWS: new versions (ADR-0072) and one changelog row each.
test -f .redkiln/CHANGELOG.md || { echo "FAIL: .redkiln/CHANGELOG.md is absent — ADR-0072's changelog was never written"; exit 1; }
for pair in root-cause:5 differential-debugging:3 adversarial-review:4 threat-model:2 user-story-ac:3 vertical-slicing:2 instrumented-trace-analysis:3; do
  id=${pair%%:*}; v=${pair##*:}; f=".redkiln/catalog/$id.toml"
  test -f "$f" || { echo "FAIL: $f is absent (Halt 8)"; exit 1; }
  grep -qE "^version[[:space:]]*=[[:space:]]*${v}([[:space:]]|$)" "$f" || { echo "FAIL: $id is not at version $v — a coherence edit bumps version (ADR-0072)"; exit 1; }
  grep -qF "$id@$v" .redkiln/CHANGELOG.md || { echo "FAIL: no changelog row names $id@$v (ADR-0072)"; exit 1; }
done

# ---- (4) THE GUARD FORM s8j ADOPTS, RUN HERE: ADR-0080 d2's named form in ANY quoting and with ANY member list, on
# ---- a CODE line, anywhere in crates/ or .redkiln/, outside the named enforcement and frozen-fixture paths.
EXCL='^(crates/rk-kernel/tests/it/(evaluator|quality_block)\.rs|crates/rk-kernel/tests/fixtures/catalog/(root-cause|differential-debugging|instrumented-trace-analysis)\.toml|crates/rk-kernel/tests/fixtures/activities/terminal_condition_(is_findings_none|does_not_parse|names_any_member_list|is_one_member_conjunct)\.toml):'
raw=$(grep -rnE 'findings\.none\(evidence in \[|quality_shorthand *=' crates/ .redkiln/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the d2 guard could not run (grep rc=$rc; 2 = no such path) — a silence here would be a fake one"; exit 1; }
live=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE "$EXCL" | grep .)
[ -z "$live" ] || { echo "FAIL: ADR-0080 d2's forbidden form on a live line:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/x.toml:5:expr    = \"findings.none(evidence in ['Reproduced'])\"" | grep -E 'findings\.none\(evidence in \[|quality_shorthand *=' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the exclusions swallowed a live one-member row — this guard is vacuous"; exit 1; }
syn2=$(printf '%s\n' "crates/rk-kernel/src/quality_block.rs:9:    let p = \"findings.none(evidence in [\";" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE "$EXCL")
test -n "$syn2" || { echo "FAIL: the path exclusion reaches src/quality_block.rs — only the TEST file's literals are exempt"; exit 1; }

# ---- (5) THE NAMED SELECTION: twenty tests, each by its own `... ok` line, and exactly twenty.
K_TESTS="lattice::p6_lattice_parity_over_the_p9_tree lattice::p6_validate_parity_over_the_p9_tree lattice::p6_lattice_over_the_shipped_pack lattice::p6_validate_over_the_shipped_pack lattice::zero_packs_is_refused_not_reported lattice::covering_array_is_pairwise_and_matches_the_python_rows lattice::unsatisfiable_fixture_names_its_cell lattice::rank_tie_fixture_is_an_error rigor::rigor_monotonicity rigor::rigor_monotonicity_p9_historical_2026_09_13 review::ac303_confirm_step_survives_escalation quality_block::ac527_empty_set_terminal_condition quality_block::ac527_frozen_p9_catalog_is_red quality_block::d2_per_class_witness_refuses_one_member_rows quality_block::d2_named_form_refuses_any_member_list quality_block::d2_severity_only_predicates_load quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies quality_block::adr0072_d2_edits_are_versioned_and_changelogged conformance::shipped_pack_loads"
out=$(cargo test -p rk-kernel --test it -- $K_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the stage's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $K_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -25; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 20 passed; 0 failed' || { echo "FAIL: the named selection did not run exactly 20 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# ---- (6) WHAT EACH INSTRUMENT READ, PRINTED FROM INSIDE THE TEST BINARY — not inferred from a green, and not probed
# ---- in another language.
out=$(cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the shipped-pack lattice run failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
line=$(printf '%s\n' "$out" | grep -E '^lattice-shipped: packs=[0-9]+ contexts=[0-9]+ ')
test -n "$line" || { echo "FAIL: the shipped-pack lattice printed no reading — nothing says what it walked"; exit 1; }
packs=$(printf '%s\n' "$line" | sed -E 's/^lattice-shipped: packs=([0-9]+) .*/\1/')
ctx=$(printf '%s\n' "$line" | sed -E 's/^.* contexts=([0-9]+) .*/\1/')
[ "$packs" -eq 2 ] || { echo "FAIL: the lattice walked $packs packs over .redkiln/, not the installed 2"; exit 1; }
[ "$ctx" -gt 0 ] || { echo "FAIL: the lattice walked zero contexts — a report over nothing (Halt 8)"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- rigor::rigor_monotonicity --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rigor_monotonicity failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^rigor_monotonicity: catalog=\.redkiln/catalog activities=102 families=[1-9][0-9]* contexts=[1-9][0-9]* ' || { echo "FAIL: rigor_monotonicity did not report reading .redkiln/catalog with a non-zero walk"; printf '%s\n' "$out" | grep -E '^rigor_monotonicity:'; exit 1; }
out=$(cargo test -p rk-kernel --test it -- quality_block::ac527_empty_set_terminal_condition --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the AC-527 lint failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^ac527: catalog=\.redkiln/catalog checks=[1-9][0-9]* refused=0 ' || { echo "FAIL: the AC-527 lint did not report .redkiln/catalog, a non-zero check count and refused=0"; printf '%s\n' "$out" | grep -E '^ac527:'; exit 1; }

# ---- (7) THE PARITY GOLDENS: present, pinned, twelve packs, the recorded baseline, and no CR byte. `od -c` reads
# ---- raw bytes; Git Bash strips a CR inside $(…).
grep -qxF 'crates/rk-kernel/tests/fixtures/lattice/** -text' .gitattributes || { echo "FAIL: no -text row pins the lattice goldens"; exit 1; }
for g in crates/rk-kernel/tests/fixtures/lattice/p9-run.txt crates/rk-kernel/tests/fixtures/lattice/p9-validate.txt; do
  test -s "$g" || { echo "FAIL: $g is absent or empty (Halt 8)"; exit 1; }
  if od -c "$g" | grep -qF '\r'; then echo "FAIL: $g carries a CR byte — the Python wrote through a text-mode stream"; exit 1; fi
done
drift=$(git status --porcelain -- crates/rk-kernel/tests/fixtures/lattice); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git status could not read the goldens (rc=$rc)"; exit 1; }
[ -z "$drift" ] || { echo "FAIL: the goldens differ from their committed bytes:"; printf '%s\n' "$drift"; exit 1; }
grep -qF "packs=['bug-expedite', 'bug-standard', 'chore', 'docs', 'incident', 'initiative', 'project', 'security-vuln', 'spike', 'story-ue', 'story', 'tech-debt']" crates/rk-kernel/tests/fixtures/lattice/p9-run.txt || { echo "FAIL: the golden is not a twelve-pack run — a zero-pack run prints a whole baseline"; exit 1; }
grep -qF 'TOTAL prereg-8 pack-default  unsat=432/155520 (0.3%)   all-12 unsat=432/235872 (0.2%)' crates/rk-kernel/tests/fixtures/lattice/p9-run.txt || { echo "FAIL: the golden's pack-default total is not the register's recorded baseline (AC-101)"; exit 1; }
grep -qF 'dead=7/102 (6.9%)' crates/rk-kernel/tests/fixtures/lattice/p9-run.txt || { echo "FAIL: the golden's dead count is not the register's recorded baseline (AC-102)"; exit 1; }
grep -qxF 'validate errors=0' crates/rk-kernel/tests/fixtures/lattice/p9-validate.txt || { echo "FAIL: the validate golden does not end in validate errors=0 (AC-104)"; exit 1; }

# ---- (8) CI RUNS THE LATTICE BY NAME. A CODE line of ci.yml, not a YAML comment.
ci=$(grep -nF 'lattice::p6_lattice_over_the_shipped_pack' .github/workflows/ci.yml | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -n "$ci" || { echo "FAIL: ci.yml runs no named lattice step — a workspace-wide filter would not say the lattice ran"; exit 1; }
syn=$(printf '%s\n' '40:          # lattice::p6_lattice_over_the_shipped_pack' | grep -vE '^[0-9]+:[[:space:]]*#' | grep .)
test -z "$syn" || { echo "FAIL: the YAML-comment exclusion let a comment through — this guard is satisfiable by prose"; exit 1; }

# ---- (9) 11c: THE PRE-REGISTRATION PRECEDES THE LABELS, AND THE RECORD EXISTS IN ADR-0085's FORM.
test -f spikes/METHOD-SEV.md || { echo "FAIL: spikes/METHOD-SEV.md is absent — the measurement was not pre-registered"; exit 1; }
pre_at=$(git log --diff-filter=A --format=%at -- spikes/METHOD-SEV.md); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$pre_at" | grep -c .)" -eq 1 ] || { echo "FAIL: METHOD-SEV.md has no single committing add (rc=$rc) — an uncommitted pre-registration is not one"; exit 1; }
CREPO=$(sed -n 's/^CORPUS_REPO=//p' CORPUS_PIN)
test -d "$CREPO" || { echo "FAIL: the corpus repository '$CREPO' is absent (Halt 8)"; exit 1; }
lab_at=$(git -C "$CREPO" log --diff-filter=A --format=%at -- results/sev-f37-labels.md); rc=$?
[ "$rc" -eq 0 ] && [ "$(printf '%s\n' "$lab_at" | grep -c .)" -eq 1 ] || { echo "FAIL: the corpus holds no single committed results/sev-f37-labels.md (rc=$rc)"; exit 1; }
[ "$pre_at" -lt "$lab_at" ] || { echo "FAIL: the labels were committed before the pre-registration ($lab_at <= $pre_at) — a method written after the data is not a pre-registration"; exit 1; }
recs=$(grep -lE '^evidence:.*"S-SEV"' references/adr/0*.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the record search could not run (grep rc=$rc)"; exit 1; }
[ "$(printf '%s\n' "$recs" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly one ADR with S-SEV in its evidence, found: $recs"; exit 1; }
grep -qE '^### Option — ' "$recs" || { echo "FAIL: $recs does not use ADR-0085's option headings (D21-42 24a)"; exit 1; }
grep -qE '^\*\*What this forbids\*\*' "$recs" || { echo "FAIL: $recs carries no What this forbids block"; exit 1; }

# ---- (10) AC-527 IN THE REGISTER — a FLOOR over a document, beneath the behavioural fixtures in (5).
ac527=$(awk '/^\*\*AC-527\*\*/ {f=1} /^### 4\.6/ {f=0} f' references/2026-09-08-acceptance-criteria.md)
test -n "$ac527" || { echo "FAIL: AC-527's entry was not found in the register (Halt 8)"; exit 1; }
printf '%s\n' "$ac527" | grep -qF "findings.all(not (evidence in ['Reproduced', 'MutationSurvived']))" || { echo "FAIL: AC-527's Verified-by does not name the recast fixture 2"; exit 1; }
printf '%s\n' "$ac527" | grep -qF 'D21-41 (h)' || { echo "FAIL: AC-527 carries no dated amendment citing D21-41 (h)"; exit 1; }

# ---- (11) ADR-0101 CLAUSE 5 / ADR-0162: no calibration claim in what this stage touched. Silence is the pass.
raw=$(grep -rniE 'guarded against false approval|calibrated against false approval' crates/rk-kernel/ .redkiln/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the calibration sweep could not run (grep rc=$rc)"; exit 1; }
[ -z "$raw" ] || { echo "FAIL: a calibration claim — S-SUBTLE-CB is UNRUN:"; printf '%s\n' "$raw"; exit 1; }

# ---- (12) THE WHOLE SUITE, with a passed FLOOR: 740 at ca8f387 (before s8a) plus this stage's 16 new tests.
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace rc=$rc"; printf '%s\n' "$out" | grep -E '^test .* FAILED|test result: FAILED' | head -20; exit 1; }
passed=$(printf '%s\n' "$out" | awk '/^test result: / {for (i = 2; i <= NF; i++) if ($i == "passed;") s += $(i - 1)} END {print s + 0}')
[ "$passed" -ge 756 ] || { echo "FAIL: the workspace passed $passed tests, below the floor of 756 — tests were lost or not run"; exit 1; }
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }

echo "CP8B-END-OF-BLOCK reached"
```

**Dependencies this block names, so that a red is read correctly.**
- Section (0) assumes that `s8a`'s file is `runbook/phase-1/s8a-records-and-red-checkpoints.md`, with a
  `### 8a.N Checkpoint 8a` heading whose last bash block ends in `echo "CP8A-END-OF-BLOCK reached"`, as the
  convention of Checkpoints 7h and 7i has it. Checkpoint 8a's own environment (any `FIX_ITEM` it names) is exported
  before this block runs.
- Section (7)'s `pk:` tag moves when `s8j` hardens `spikes/p9/packs/spike.toml`. The goldens are regenerated then,
  and section (2) deliberately excludes `spikes/p9/packs/` for that reason.

---
