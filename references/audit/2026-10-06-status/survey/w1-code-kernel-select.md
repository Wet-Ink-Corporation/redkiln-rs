# W1 audit: `crates/rk-kernel`, SELECTION side

Snapshot: commit `36dd8ac` (worktree `audit-wt`), read-only. Auditor scope: crate skeleton and determinism gate, predicate grammar, registry loader, the three facet lints, rigor monotonicity, the seven-step `select()`, catalog loading, and the conformance lints of Steps 7b / 7e / 7f / 8b. Gates and runtime are other auditors' areas.

Paths are relative to the worktree. "RECORDS" means runbook/ADR/decision text; "CODE" means what the source and tests show.

## 0. Verdict

Grade **B** (solid, with wiring and hygiene debt). Completeness against what Phase 1 requires of this area: about **85%**.

The selection engine is real, pure, deterministic by construction and unusually well tested. The grammar is differential-tested against the S-GRAMMAR probe, the seven steps exist as typed functions, and the three facet lints fail closed on every clause. The kernel `it` target ran on this snapshot at 417 passed, 0 failed, 5 ignored (the ignored ones are child-process entrypoints); the lib target ran 12 passed (`audit/log-test.txt:2032-2549`, `:2014-2030`). `cargo clippy --workspace --all-targets` and `cargo fmt --all --check` both exited 0 (`audit/build.txt`).

What keeps it from an A:
- Several load-time "lints" exist only as `cargo test` assertions over the shipped pack and are never called by any product path.
- Step 7 (deviation) is built but has no production call site.
- Three separate selection models live in the crate with no differential between them.
- Three load-time sweeps use a hard-coded toolchain set that omits `rust`, the toolchain of Redkiln-RS's own lane.
- A real wrong-winner bug in step 4 survived Checkpoint 7b for two weeks.
- The crate docs are heavily layered and partly stale.

## 1. Module map and public surface

Selection-side source is about **18.4k lines**, covered by about **16.8k lines** of dedicated integration tests plus 198 fixture files (`crates/rk-kernel/tests/fixtures`). I counted the line totals with `cat | wc -l` over the files listed below.

| module | lines | role |
|---|---|---|
| `lib.rs` | 441 | crate doc plus `pub mod` list |
| `select.rs` + `select/*.rs` | 6095 | `select()`, `Selection`, `step1`..`step7`, `lints`, `pack`, `artifacts`, `item`, `canonical` |
| `predicate.rs` + `predicate/{lexer,parser,ast,eval,lint,on_false}.rs` | 1928 | ADR-0103 grammar, three-valued evaluator, AC-069 narrowing lint, closed `on_false` set |
| `registry.rs` | 895 | facet registry loader, `StabilityRecord`, `PrecisionRecord`, `PinnedCorpus`, `PredicateSites` |
| `facet_registry.rs` / `facet_provenance.rs` / `facet_stability.rs` | 559 / 435 / 242 | AC-072, AC-520, AC-071 |
| `rigor.rs` | 1372 | `Rigor` ordinal, AC-105 `rigor_monotonicity_rule`, `confirm_step_monotonicity_rule` |
| `catalog.rs` | 1816 | activity/step loader, diagnostics, trial admission, forward-ref lint |
| `lattice.rs` | 2855 | Rust port of `spikes/p9/lattice.py` (AC-101..AC-104) |
| `pack_validation.rs` + `resolver.rs` | 490 + 413 | AC-495 capability routing; AC-494 resolver |
| `quality_block.rs` | 653 | AC-527 terminal-condition rule |
| `toml_subset.rs` | 540 | hand-written subset TOML reader |

Public API entry points:
- `select::select(pack, catalog, artifacts, facets, stage, item) -> Result<Selection, SelectError>` (`crates/rk-kernel/src/select.rs:496`).
- `select::step7` (`select/step7.rs:344`).
- `Selection::plan` for step 6 (`select.rs:360`).

Other architecture lints in the crate (`module_boundary`, `kernel_denylist`, `adapter_edge`, `no_egress`) are source/graph lints. I did not audit them in depth; they belong to the gates/runtime auditors.

## 2. What the stage files and design require (Phase 1)

- **Step 7b** (`runbook/phase-1/s7b-kernel.md`; design §4.4, §4.7): crate skeleton, determinism gate (AC-001..AC-004), ADR-0103 grammar (F42), registry loader and both facet lints, rigor-monotonicity lint (AC-105), seven-step selection function (AC-040..AC-079, minus named exclusions).
- **Step 7e** (`s7e-pack-authoring.md`): `.redkiln/` registry, exactly two packs, row dispositions, `stale_policy`, lattice re-baselined.
- **Step 7f** (`s7f-conformance-and-records.md`): the three conformance lints over the design's closed sets, plus `shipped_pack_loads`.
- **Step 8b** (`s8b-catalog-quality.md`): the P6 lattice ported to Rust as the instrument of record for AC-101..AC-104, AC-527 repaired to ADR-0080 decision 2, catalog rows versioned.
- **Phase-1 exit** needs selection to drive both lanes with `$0` model spend, replayable, with unsatisfiable as a typed gate.

## 3. Genuinely implemented and exercised

### 3.1 Determinism gate

- `select.rs:107-108` carries `#![deny(clippy::disallowed_methods)]` and `#![deny(clippy::disallowed_types)]`.
- There is a committed positive control: feature `select-clock-reintroduce` (`Cargo.toml` features; `select.rs:163-167`).
- The filesystem, environment and event-log halves are separately enforced by the source scan `selection_source_reads_no_hidden_input` (`tests/it/selection.rs:997`).
- `RequiredSet` is `pub(in crate::select)` constructible and has no `Deserialize`. This is pinned by two trybuild cases (`tests/ui/ac061_required_set_has_no_deserialize.rs`, `tests/ui/ac061_required_set_is_not_constructible_outside_select.rs`) and an in-module unit test (`select.rs:804`).
- Canonicalisation delegates to `rk_store::canon` rather than a second serialiser (`select/canonical.rs:1-20`).
- AC-040's 100-repetition test shuffles map construction, not call order (`tests/it/selection.rs:463-480`).
- A pinned golden `inputs_digest` literal exists (`tests/it/replay.rs:644`), so each of the two CI OS runs asserts the same value. I could not verify that CI has run green.

### 3.2 Predicate grammar

Hand-written lexer, parser and three-valued evaluator (`predicate/*.rs`).
- `tests/it/probe_differential.rs:49` compares against captured probe verdicts per string: parse/fail, byte position, feature census key for key, and mixed-connective verdict. Denominators are pinned (386 strings, 287 predicates, 285 parse / 2 fail, `bare_literal` 191, `in` membership 11).
- It runs against a committed JSON fixture (`tests/fixtures/grammar/s-grammar-probe-verdicts.json`), not the live corpus. It is a historical capture by design (`probe_differential.rs:34-41`).
- The records state the live reading is 279/279 predicates parsed (PROGRESS row 31, `bf12a1c`). I did not re-run the probe.
- AC-069's narrowing lint is `predicate/lint.rs:75`. `lint_selection_predicate` is called from step 2 and step 7 (`select/step2.rs:423`, `select/step7.rs:378`).

### 3.3 Registry loader and facet lints

All three are rules over a `Registry`. Each collects every violation rather than returning at the first, and each has fixture-driven tests.
- AC-071: `facet_stability_rule` (`facet_stability.rs:130-215`). All five clauses are checked for every `llm` facet with no early return; an absent field and a failing field produce the same verdict.
- AC-520: `facet_provenance_rule`. It returns a `ProvenanceReport` rather than `()`, so a facet that is not measurable is reported, not silently passed.
- AC-072: `facet_registry_rule`, driven over the shipped registry as well as fixtures.
- `rk-cli` wires AC-071 and AC-520 as `redkiln lint --facet-stability|--facet-provenance` (`crates/rk-cli/src/verbs/lint.rs:81-155`). `doctor` wires AC-072 (`crates/rk-cli/src/doctor/rules.rs:1482`).

### 3.4 Selection (seven steps)

- Step 1 rigor; step 2 required set with keyed expansion and abstention gates; step 3 Present/Stale/Missing with the registered-source tie-break over AC-066/AC-067; step 4 producer by lowest `method_rank` (a tie is `LoadError::RankTie`, `select/step4.rs:411-476`); step 5 validate (cycle, `excludes`, appetite gate carrying the DAG); step 6 `ExecutionPlan`; step 7 `step7`.
- Load-time refusals run inside every `select()` call via `lints::load_lints` (`select.rs:520`, `select/lints.rs:349-363`): forward-ref licence, requires cycle, excludes symmetry, dangling requires, rank tie.
- A pass-sourced artifact kind selects no catalog producer and is not `Unsatisfiable` (`select.rs:575-583`, ADR-0191 R1).
- Unsatisfiable is data in the DAG (a typed gate), not an abort (`select.rs:182-189`).
- `inputs_digest` is `sha256_lp` over four digests (`select/canonical.rs:54-63`).
- `select()` is called in production at `crates/rk-cli/src/verbs/run.rs:6044` and in replay (`crates/rk-kernel/src/replay.rs:1508`, `:1967`).

### 3.5 Rigor monotonicity

`rigor_monotonicity_rule` (`rigor.rs:300`) implements ADR-0081's superset relation per family over the facets the family reads. `tests/it/rigor.rs:102` is red on a fixture catalog that is the shipped `bisect` minus its ceiling line and green over `.redkiln/catalog`. `tests/it/rigor.rs:460` keeps the p9 reading as a historical pin.

### 3.6 Catalog and lattice (7e / 8b)

- `.redkiln/` exists with 102 catalog activities and exactly two packs (`bug-standard`, `story`), per Step 7e.
- `lattice::run` and `lattice::validate` are Rust ports with byte-parity goldens against the Python at `P9_PIN` (`tests/it/lattice.rs:456`, `:501`), and the shipped reading is pinned: `pairs=279936 unsat=1296 dead=57/102`, `ties=0`, `hard_cycles=0` (`tests/it/lattice.rs:45-47`, `:523-596`). Family totality holds (`tests/it/lattice.rs:598`).
- Red fixtures exist for AC-101..AC-104: unsatisfiable, rank tie, hard cycle, dead activity, family hole (`tests/it/lattice.rs:653-925`).
- AC-527 (`quality_block.rs`) is a parse-tree rule with a per-class witness evaluation, widened at s8d to "not true in some cell". It has red tests on the frozen p9 catalog (`tests/it/quality_block.rs:640`).

### 3.7 Conformance (7f)

`tests/it/conformance.rs` holds the three lints (step kinds have executors, ports have call sites, trailer keys have producers) plus `shipped_pack_loads`. No `#[ignore]` remains in the file (grep). All 7 conformance tests were `ok` in the snapshot run. The lints scan code lines only (comments stripped, string literals blanked) and carry committed controls for three defeats found in review (`conformance.rs:40-60`, `:296`).

## 4. Stubbed / fake-only / ARMED-not-run / deferred / missing

| item | state | evidence |
|---|---|---|
| **Step 7 deviation in production** | Built and replay-tested; **no production caller**. The runtime refuses `on_false = { to = "deviate" }` by name: "no mid-walk deviation is built". | `crates/rk-kernel/src/runtime/step.rs:2238-2244`; `step7(` appears only in `replay.rs:1518`, `:2017`; no hit in `rk-cli/src` |
| **AC-045 `SelectionMemoKey`** (four-digest selection memo key) | ARMED; the type does not exist. ADR-0119 says it is "derived in rk-kernel::select". | `tests/it/memo.rs:16`; `workflow/memo.rs:126-128`; grep for the type finds comments only |
| **AC-044** (selection crate excludes the harness crate) | ARMED; records say a dated amendment is owed because the THEN is unsatisfiable at crate scope. | `s7b-kernel.md` §7b.15 table |
| **AC-072 `anchor` clause** | Not built. No record gives `anchor` an on-disk form; AC-072 still lists it in its THEN. | `facet_registry.rs:64-76`; `references/2026-09-08-acceptance-criteria.md:1750` |
| **Corpus-resolution half of AC-520 / AC-071** | Modelled, not resolved through the git object store. The CLI takes `--pinned-corpus <PATH>` and reads a TOML sidecar, `.redkiln/corpus-records.toml`, which is "TRANSCRIBED ... NEVER REGENERATED". | `crates/rk-cli/src/verbs/lint.rs:125-143`; `.redkiln/corpus-records.toml:17-22` |
| **AC-495 pack capability routing, AC-494 resolver** (`pack_validation.rs`, `resolver.rs`, about 900 lines) | Built and tested, **no production call site**. The header names the temporal half ("before any activity runs") as `rk-cli`'s and ARMED. Nothing in `rk-cli/src` or `rk-adapter-claude/src` references them. | `pack_validation.rs:75-81`; grep for `pack_capability_routing_rule` / `ClaimedCapabilities` hits only tests and the module |
| **AC-105 `rigor_monotonicity_rule`, `confirm_step_monotonicity_rule`, AC-527 rule, lattice, forward-ref lint** | Test-only instruments run over the shipped pack in CI (`.github/workflows/ci.yml:107-125`). No `redkiln lint` or `doctor` verb calls them, so an edited catalog is protected by CI only. The forward-ref lint does run inside every `select()`. | grep: callers in `rk-cli/src` are absent for `rigor`, `quality_block`, `lattice` |
| **AC-066 8-case table** | ARMED pending a register edit; pinned instead by `short_circuit::registered_source_breaks_the_ac066_ac067_tie`. | `select/step3.rs:20-30` |
| **Step 6 criterion** | The design records "no criterion covers step 6". Code builds it to its stated invariant and names no AC. | `select/step6.rs:7-27` |
| **Records owed, never written** | ADR-0103's two missing productions (`<string> in <set facet>`, hyphen in `${...}`), which the parser implements and no record admits; ADR-0103 connective-commutativity clarification; ADR-0081 clause 2's instrument now points at a retired tool; AC-072's `anchor` and domain-shape discrepancy (four live enumerated entries vs three named shapes); the general `nullable` case. | `predicate.rs:35-45`; `rigor.rs:11-35`; `facet_registry.rs:64-110`; `runbook/PROGRESS.md:121`; `references/adr/ERRATA.md` has no entry for these |
| **AC-102 threshold on the shipped pack** | Reported, not thresholded, by D21-47 V3. 57 of 102 shipped activities (55.9%) are dead under the two installed packs, against AC-102's 20% kill line. | `tests/it/lattice.rs:36-47`; `spikes/DECISIONS-2026-09-21.md:1928`, `:2563-2570` |

Reading of the dead-activity figure: the design's 5.9% (6/102) baseline was taken over 12 packs. D21-47 V3 and ADR-0011/E7 explicitly scope Phase 1 to two packs, so I treat the 57/102 as a decided scope reduction, not a defect. It does mean more than half the catalog content is unreachable on the shipped lanes today.

## 5. Code quality

### 5.1 Structure and error design: strong

- Each step is its own module with typed outcomes (`Step1Outcome` .. `Step5Outcome`, `Picked`, `Unsatisfiable`, `DeviationOutcome`). `SelectError` wraps typed per-step errors with `From` impls.
- Invariants are carried in types: `RequiredSet`'s constructor visibility, `Accepted` (an agent-supplied value cannot reach `select_amended`), and `step3`/`step6` signatures that offer no path to a store or to `R`.
- `step4` re-raises `RankTie` at runtime instead of tie-breaking (`select/step4.rs:456-466`).
- Findings are collected, never first-only (all three facet lints).
- `Selection` exposes read-only accessors and a `journal()` that emits `dag_assembled`.

### 5.2 Test depth: broad, deliberately adversarial, but coarse in places

Test kinds present for this area:
- Integration tests: 29 files, about 16.8k lines.
- Differential: probe verdicts and lattice-vs-Python goldens.
- Golden / pinned literals: digests, shipped lattice counts.
- Trybuild compile-fail: AC-041, AC-050, AC-061 (two cases) under `tests/ui/`, with committed `.stderr`.
- Source-scan lints, each with a planted control.
- Mutant-based hardening recorded in stage files (M-numbers). I did not re-run these.

Weaknesses:
- **No property-based tests or fuzzing** on the parser, three-valued evaluator, canonical serialiser or `select()`. `proptest` is a kernel dev-dependency but is used only in `tests/it/review.rs`.
- **Coarse test functions.** `facet_registry`, `facet_provenance`, `facet_stability`, `rigor_monotonicity` and `probe_differential` are each a single `#[test]` of hundreds of lines (`tests/it/facet_registry.rs`, `tests/it/rigor.rs:102-460`). A failure localises to a case through message text, not a test name. The lattice parity tests took over 60 s each in the debug build (`audit/log-test.txt` "has been running for over 60 seconds").
- **Tests are coupled to product content.** `SHIPPED_PAIRS/UNSAT/DEAD` and a 102-activity count are pinned (`tests/it/lattice.rs:45-47`, `:540`). Any pack or catalog edit breaks kernel tests by design. This is a tripwire, and also why the stage files carry long "re-run pending" chains.
- **A wrong-winner bug survived Checkpoint 7b for 14 days.** Step 4's `Candidate` derived `Ord` over `(activity_id, method_rank)`, so a plain `sort()` picked the alphabetically first id rather than the lowest rank. It chose `bisect` (rank 30) over `root-cause` (rank 10) in the fix lane. It was fixed at `bfe0324` (2026-09-27, s8d, D21-73 / ADR-0179) and found only when the fix lane first ran (`select/step4.rs:439-445`). This is the clearest sign that the 7b fixtures did not pin "lowest rank wins" over several admissible candidates.
- **Three selection models, no differential.** `select()` (step4 `judge`), `lattice::Run::{required,producer}` and `rigor.rs`'s `Prepared`/`read_facets` each implement admissibility and rank logic. The evaluator is shared; the filters are not. The lattice is parity-tested against the Python, but no test compares the lattice or `rigor.rs` against `select()` (grep of `tests/it` finds none; `lattice.rs:1-92` describes a clause-by-clause port). The shipped "unsat 1296 / dead 57" reading is therefore the lattice's model of selection, not `select()`'s.

### 5.3 Escape hatches in non-test code

The raw census for the whole crate reads `unwrap/expect=58 panics=8 allow/expect-attr=22 unsafe=0` (`audit/escape-hatches.txt:9`). I re-counted non-doc, pre-`#[cfg(test)]` lines in this area:
- **Selection-side files**: three production `.expect(..)` calls, all in the parser, each with an invariant message (`predicate/parser.rs:252`, `:333`, `:371`). The other 14 hits in that file are the parser's own `self.expect(TokenKind..)` method, not `Option::expect`.
- No `unwrap()` and no `panic!`/`todo!`/`unimplemented!` in the selection modules.
- `#[expect(..., reason = "...")]`, never `#[allow]`, in `select/step5.rs:385`, `select/step7.rs:338` (`too_many_arguments`, reasons given), `toml_subset.rs:86` (`cast_precision_loss`) and `lattice.rs:1713/1722/1858/2380`. This matches ADR-0102 decision 3.
- `unsafe`: none in `rk-kernel`; the workspace sets `unsafe_code = "deny"`.
- Other kernel hits (`runtime/tier_one.rs` 11, `runtime.rs` 3, `runtime/activity.rs`, `replay.rs`, `runtime/advisory.rs`) sit in the runtime and are outside this audit's scope.

I did not run cargo. I relied on `audit/log-clippy.txt`: the selection-side files carry essentially no clippy warnings (two nursery hits, `select/item.rs:577` and `select/lints.rs:263`; I did not read their text). The 90 lib warnings are concentrated in `runtime/tier_one.rs`, `module_boundary.rs`, `replay.rs`, `workflow/*`, `review/*` and `verify/*`.

### 5.4 Fail-open spots in loaders

- **Catalog header fields are lenient.** `catalog.rs:1025-1036`: `strings()` silently drops non-string array items and returns empty for a non-array, so `provides = "Foo"` becomes an activity that provides nothing. `min_rigor` silently defaults to `"optional"` (`catalog.rs:1339`). Unknown keys are not rejected in the catalog or pack loader, so a typo such as `min_rigour` is silently ignored. This contradicts the principle in `toml_subset.rs:1-30` that a reader must not turn a malformed file into a pass. The shipped files are clean; the exposure is future author- or agent-edited entries.
- **`catalog_json` omits fields that change selection output.** The `inputs_digest` hashes eleven hand-enumerated fields (`select/canonical.rs:73-91`). `excludes` is read by step 5 (`select/step5.rs:516-528`) and `deviation_ok` by step 7 (`select/step7.rs:360-372`), and neither is in the digest. The comment at `catalog.rs:478` holds one way only. Impact today is small: `excludes` has no live instance (`catalog.rs:519`) and step 7 is not reached in production. It is a hidden-input gap for replay and any future memo.
- **Hard-coded toolchain sweeps omit `rust` and `python`.** Both `rank_lint` (`select/step4.rs:595-639`, run on every `select()` via `load_lints`) and `rigor_monotonicity_rule` (`rigor.rs:565-568`) enumerate `node / unreal / node+unreal / dotnet`. The registry's `toolchain` domain is `["node","dotnet","python","rust","unreal"]` (`.redkiln/facets.toml:62`), and two shipped activities (`mutation-testing`, `property-based-testing`) mention rust. `lattice.rs:176` and `:189` do add `rust`. So the at-load tie check and the monotonicity check never examine a rust context, even though Redkiln-RS's own lane is Rust. Runtime `step4` still returns `RankTie` on a real tie, so it fails closed at runtime; the gap is in detection time and in the ADR-0081 check. `rank_lint` also defaults a missing key to `"node"` (`step4.rs:562`).
- **No recursion bound** in the parser or evaluator (grep for a depth or recursion limit in `predicate/parser.rs` and `predicate/eval.rs` finds none). A hostile or generated predicate with deep nesting could overflow the stack; I did not test this. Packs are author-trusted today.
- **Hand-written TOML reader** (`toml_subset.rs`, 540 lines) refuses what it cannot parse, with tests. Its stated reason ("this workspace has no TOML dependency") was overtaken when `rk-cli` adopted the `toml` crate (`Cargo.toml:89`, D18-3). The workspace now has two TOML acceptance surfaces. The subset reader refuses multi-line values, so author-edited catalog entries that use them will be refused with a typed error (fail-closed), not misread.

### 5.5 Dead code, comments and doc noise

- **No dead code reported** by rustc or clippy in the selection files. The unwired but public modules in section 4 do not trigger dead-code warnings because they are `pub`.
- **Comment-to-code ratio** in these modules is 0.40 comment lines per code line overall (4407 vs 11069), measured on pre-`cfg(test)` lines. Per file it ranges from 0.1 (`lattice.rs`, `predicate/parser.rs`) to 1.8 (`select/step6.rs`) and 4.6 (`predicate.rs`, a doc-only facade).
- **The prose is history, not description.** The crate doc (`lib.rs:1-300`) opens with *"Steps 5 (validate), 6 (execute) and 7 (deviation) are not built and are not stubbed"* (`lib.rs:8-13`). That is false today. A "Corrected 2026-09-14" note beneath it narrows the claim, and later "And - Slice 4x" paragraphs add to it. A new reader must reconcile the layers by hand.
- Other stale statements, each contradicted by code:
  - `catalog.rs:4-6`: "that is Slice 3's seven-step pure function and it is **not built**".
  - `facet_stability.rs:42-46` and `facet_provenance.rs:44-46`: the `redkiln lint` CLI half is "deferred to 7d/7e". `rk-cli/src/verbs/lint.rs` implements it.
  - `registry.rs:29-30` and `facet_registry.rs:108` carry the same Step-7b-era framing.
- Clippy `too_long_first_doc_paragraph` fires 148 times workspace-wide (`audit/log-clippy.txt`), almost none of them in the selection files.

## 6. Risks

1. **Unwired instruments read as built.** AC-105 monotonicity, AC-527, AC-495/AC-494, step 7 and the lattice are not reachable from `redkiln lint`, `doctor` or `run`. They protect the repo only through CI test jobs over the shipped pack. For Phase-1 self-hosting that is adequate; for Phase 2 (agent-authored stories and later catalog edits) it is not. Impact: medium.
2. **Triplicated selection logic.** A change to admissibility, ceiling or rank semantics must be mirrored in `step4::judge`, `lattice::Run` and `rigor::Prepared`. Only lattice-vs-Python parity is tested. Impact: medium (silent divergence of the completeness metrics from real selection).
3. **Hard-coded toolchain domain** leaves `rust` and `python` outside two load-time checks. Impact: medium for the self-hosting lane.
4. **Lenient catalog headers and an incomplete catalog digest** (section 5.4). Impact: low now, rising as catalogs become author- or agent-edited.
5. **Records debt.** About eight OWED record corrections remain open (ADR-0103 productions, connective commutativity, ADR-0081 clause 2 instrument, AC-072 anchor and domain shape, AC-066 table, and the AC-044/045/043/064/056 amendments), and `ERRATA.md` has no entry for them. The code is ahead of the records, which ADR-0092's precedence rule treats as a defect to repair. Impact: medium for auditability, low for function.
6. **Phase-1 catalog reachability.** 57 of 102 shipped activities are dead under two packs, and 1296 of 279,936 pack-default cells are unsatisfiable (0.46%, inside AC-101's 5% line). Impact: informational, a scope choice.
7. **Review-round cost.** The crate's docs and tests carry long dated-amendment chains. The "re-run pending" mark on Checkpoints 7b, 7b-ii, 7f and 8b has been repeatedly set and cleared as other stages touch `crates/rk-kernel`. Checkpoint 7f is still `re-run pending s8g .redkiln` in the snapshot's tracker (`runbook/PROGRESS.md` row 36). Impact: process overhead, not correctness.

## 7. Unverified

- I did not run cargo. Test and clippy results come from the pre-existing build logs in `audit/` (`log-test.txt`, `log-clippy.txt`, `build.txt`), taken on this snapshot on Windows.
- The `--features corpus` run (`golden_log_replay`, `ac161_ac164_delete_db_replay`, other corpus-gated tests) did not run in that log, so those results are unverified. They are outside this area.
- The determinism-gate bite (`cargo clippy ... --features rk-kernel/select-clock-reintroduce` returning rc 101) was not run. Only the committed control and the records claim it.
- The live-corpus claims (`predicates parsed 279/279`, 0/279 mixed, `on_false forms 99/99`) come from PROGRESS row 31, not from my run.
- The mutant campaigns (M-numbers in stage files) are recorded, not re-run.
- Whether `.github/workflows/ci.yml` has ever run green on the two-OS matrix is unverified.
- The runtime cost of one `select()` is unmeasured. It runs the full `rank_lint` enumeration each call via `load_lints`. The lattice parity tests take over 60 s in a debug build, but that is the lattice, not `select()`.
- I did not read the per-criterion state lines in the AC register beyond the cited criteria, so the "owed amendment" list in section 6 comes from stage-file prose.

## 8. Key facts

- Selection-side src 18.4k lines; dedicated tests 16.8k lines in 29 files; 198 fixture files; 15 trybuild cases kernel-wide (four on selection).
- Kernel `it` target on the snapshot: 417 passed, 0 failed, 5 ignored (child entrypoints). Lib: 12 passed (`audit/log-test.txt`).
- Shipped lattice reading: `pairs=279936 unsat=1296 dead=57/102 ties=0 hard_cycles=0`, two packs (`tests/it/lattice.rs:45-47`).
- Production `.expect` in selection-side code: 3 (parser). `unwrap()`: 0. `unsafe`: 0. `#[expect]` with reasons: 7 in these modules.
- No commit has touched `crates/rk-kernel` since the whole re-run at `d24d15a` (2026-10-02; `git log d24d15a..HEAD -- crates/rk-kernel` is empty), so the recorded checkpoint passes for this area are current against the snapshot.
- Step-4 wrong-winner bug introduced at `8444221` (2026-09-13), fixed at `bfe0324` (2026-09-27).
