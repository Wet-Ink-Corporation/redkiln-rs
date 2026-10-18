# s8j fix-item shortlist — §8j.5's survey re-run at HEAD (S8J-A3-SHORTLIST)

**For Ryan's pick (plan decision D5). This file decides nothing.** It reserves no item, mints nothing and writes no
ledger line. §8j.5's reservation and mint steps run only after the pick.

- **Tree surveyed:** `origin/build/phase-0-to-1` at `3fa69db` (2026-10-18). Package S8J-A3-SHORTLIST of
  `spikes/plans/s8j-build-plan-2026-10-16.md` §3 Round A; drift item 10 in its §2.
- **Method:** `runbook/phase-1/s8j-run-preparation.md` §8j.5, steps 1 to 5. The earlier pass read the code at
  `ca8f387` and ran nothing red. This pass re-walks the callers over the call graph as it stands after `s8i`, re-checks
  the scheduled-work rule against `s8a`…`s8i` and the open GitHub issues, and runs every candidate red.
- **Repro trees:** each repro ran in a throwaway `git archive origin/build/phase-0-to-1 | tar -x` tree under the session
  scratchpad. That tree was deleted afterwards. No crate in the repository was edited, and no repro test is committed
  anywhere (§8j.5 step 3, 🛑).

## Summary

| # | crate · function | verdict at `3fa69db` | red | confidence |
| --- | --- | --- | --- | --- |
| C1 | `redkiln-migrate` · `legacy::fence::split` | **presented** | yes | high (defect); low impact |
| C4 | `rk-cli` · `doctor::rules::margin` (`RK-D-MARGIN`) | **presented** | yes | high |
| C5 | `rk-types` · `schema::lint::walk` | **presented** | yes | medium-high (up from "lower") |
| C3 | `rk-cli` · `verbs::explain::top_level_conjuncts` / `false_conjunct` | **excluded, rule 1** (new at this pass) | yes | high defect; fails the static walk |
| C2, C6 | `rk-store` · `locator::scan`, `locator::parse_heading` | excluded, rule 1 (unchanged) | not run | — |

Three candidates remain, which meets D21-42 18's floor of three exactly. The plan's *"C1, C3, C4, C5 today"* is one
fewer at HEAD. See C3 below. If C3 were re-admitted by ruling, the count would be four.

## The presented candidates

### C1 — `redkiln-migrate` · `legacy::fence::split`

- **File:** `crates/redkiln-migrate/src/legacy/fence.rs:42`
- **Quoted code:**
  ```rust
  let tail = src[after + close..].trim_start_matches("\n```");
  ```
  `trim_start_matches` strips its pattern **repeatedly**. When a second fence starts directly after the closing yaml
  fence (`…\n```\n```diff…`), the second fence's opening backticks are stripped too.
- **Broken contract:** the `Fenced` doc says `prose` is *"Everything outside it"* (`fence.rs:16-17`). Everything
  outside the yaml fence begins with the second fence's three backticks.
- **Red** (`cargo test -p redkiln-migrate --test it s8j_repro`, a test appended to `tests/it/lift.rs`):
  ```text
  thread 'lift::s8j_repro_a_fence_directly_after_the_yaml_fence_keeps_its_backticks' panicked at crates/redkiln-migrate/tests/it/lift.rs:270:5:
  prose is everything outside the yaml fence, so it must begin with the second fence's backticks; got "diff\n- a\n+ b\n```"
  test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 25 filtered out
  ```
  Input: `"```yaml\nk: v\n```\n```diff\n- a\n+ b\n```\n"`.
- **Out-of-run argument:**
  - No crate depends on `redkiln-migrate`. The only manifests that mention it are its own and two comments, in
    `rk-cli/Cargo.toml`:58 and `rk-store/Cargo.toml`:30.
  - `s8`'s blocks never invoke the migrator: `grep migrate runbook/phase-1/s8-exit-criterion.md` finds nothing.
  - `split` has three callers, all in the crate: `lift.rs:313`, `legacy/ledger.rs:31` and `legacy/review.rs:55`. Only
    `ledger.rs:44` keeps `prose`, and nothing reads `LegacyLedger.prose`.
  - `s8i`'s AC-475 lift-share measurement (`s8i-s-bin.md`:7250-7271) counts diff-prose markers. Those come from
    `legacy/frontmatter.rs`'s `push_if_diff`, not from `fence::split`, so a fix does not move `ac475_lift_share=0/0`.
- **Rule 2:** no file or function is named in `s8a`…`s8i`. No open issue names it.
- **Fix size:** one line in one function, for example `strip_prefix("\n```").unwrap_or(rest)` or an equivalent
  single strip.
- **Confidence:** high that it is a defect. The impact is low, because the corrupted field has no reader.

### C4 — `rk-cli` · `doctor::rules::margin` (`RK-D-MARGIN`)

- **File:** `crates/rk-cli/src/doctor/rules.rs:849-853`
- **Quoted code:**
  ```rust
  let Some(margins) = table(&pack, "margins") else {
      continue;
  };
  for window in ["five_hour", "seven_day"] {
      if let Some(ticks) = margins.get(window).and_then(toml::Value::as_integer)
  ```
  A pack with no `[margins]`, with a window missing, or with a non-integer margin produces no finding.
- **Broken contract:** AC-377 (`references/2026-09-08-acceptance-criteria.md`:6117-6128) says the configuration *"is
  refused as a configuration error before any dispatch AND `redkiln doctor` reaches the same verdict without
  dispatching a model call"*. Its Verified-by includes *"a fixture config with no margin that must fail (no
  default)"*. The two halves disagree today:
  - The run's loader **refuses** such a pack. `verbs/run.rs:91-96`'s `Margins { five_hour: u16, seven_day: u16 }` has
    no `#[serde(default)]`, and `PackExtras.margins` (`run.rs:56`) is documented as *"both members required"*.
  - Doctor's `RK-D-MARGIN` passes the same pack.
  - The sibling rule `RK-D-TTL` (`rules.rs:829-833`) does report the absent key (*"declares no `[cache] {key}`"*).
    `RK-D-MARGIN` has no such arm.
- **Red** (`cargo test -p rk-cli --test it doctor::s8j_repro`): the test materialises `doctor/clean`, removes
  `seven_day = 10` from its `demo.toml` and runs `RK-D-MARGIN` through the verb's own `Ctx` builder.
  ```text
  thread 'doctor::s8j_repro_a_pack_missing_a_margin_window_is_a_doctor_error' panicked at crates/rk-cli/tests/it/doctor.rs:3561:5:
  AC-377: a fixture config with no margin must fail (no default) and doctor reaches the same verdict as the load; [margins] lacks seven_day and RK-D-MARGIN found []
  test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 342 filtered out
  ```
- **Out-of-run argument:**
  - `margin` is reached only through `RULES` (`rules.rs:214-217`), then `rules::evaluate` (`rules.rs:314`), then
    `verbs/doctor.rs:225`.
  - `verbs/doctor.rs` returns before `evaluate` on `--rebuild` (`if args.rebuild { … return … }`, `verbs/doctor.rs`
    lines 62-72 of `run_to`). The only doctor call in `s8`'s blocks is `doctor --rebuild`
    (`s8-exit-criterion.md`:87), and `s8j`'s blocks have no plain `redkiln doctor`.
  - `run`, `answer`, the kernel and the `s8i` additions never call `doctor::rules`. The walk covered `review_input.rs`,
    `verbs/run/kept_refusal.rs`, `rk-kernel/src/workflow/gate.rs`, `rk-store/src/render/binary.rs` and
    `rk-ports/src/toolchain/binary.rs`.
- **Rule 2:**
  - `doctor/rules.rs` is named in `s8c` (`RK-D-TIMEOUT`), `s8d` (the `stage` field), `s8d-ii` (the hook marker and the
    `join` sites) and `s8e`, each for **other** rules. None names `margin` or `RK-D-MARGIN`.
  - `s8f`'s D21-42 12c owns `RK-D-HEADROOM-MARGIN` (`headroom_margin`, `rules.rs:1707`), a different rule.
  - Open issues #68 and #31 (header comments at `rules.rs:19-22` and `:75`), #22 (`credential_retained`,
    `network_fs`), and #51 and #13 (`claims::bind` at `rules.rs:2432`) name the file for other functions. None
    schedules this one.
- **Blast radius:** every pack under `crates/rk-cli/tests/fixtures/doctor/*/` and both shipped packs
  (`.redkiln/packs/*.toml`) declare both windows. So a fix that reports an absent window should not turn an existing
  doctor fixture red. That was checked by grep, not by running the suite with a fix.
- **Repro-pair note:** §8j.5 says the C4 repro writes its own pack into a temporary repository. This repro instead
  starts from the committed `doctor/clean` fixture and edits it, which still needs no fixture directory in the pair.
  Either form fits `kind = "it"`.
- **Confidence:** high.

### C5 — `rk-types` · `schema::lint::walk`

- **File:** `crates/rk-types/src/schema/lint.rs:102`
- **Quoted code:**
  ```rust
  Value::Object(_) => walk(child, &p, child_inside_union && key != "properties", out),
  ```
  The inside-union flag is reset under every `properties` map, so a union inside an object that is itself a union
  branch is never flagged.
- **Broken contract:** AC-131 (`acceptance-criteria.md`:2670) says *"no union is nested inside a union — no `anyOf`
  (or type array) has a descendant `anyOf` or type array"*. ADR-0095 (`references/adr/0095-…md`:65) uses the same
  words. The rule's own variant doc reads *"A union with a union descendant (AC-131)"* (`lint.rs:25`).
- **Red** (`cargo test -p rk-types --test it schema_no_oneof`, a test appended to `schema_no_oneof.rs`, over
  `{"anyOf":[{"type":"object","additionalProperties":false,"required":["f"],"properties":{"f":{"type":["string","null"]}}},{"type":"null"}]}`):
  ```text
  thread 'schema_no_oneof::s8j_repro_nested_union_through_properties_is_caught' panicked at crates/rk-types/tests/it/schema_no_oneof.rs:110:5:
  AC-131: no anyOf has a descendant anyOf or type array; /anyOf/0/properties/f is one. Got []
  test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 75 filtered out
  ```
- **The caveat ("may be deliberate"), checked:**
  - `git log -L` shows the line unchanged since `96ba9c2` (Step 5, the crate's first commit), with no comment giving a
    reason.
  - **In the throwaway tree, deleting `&& key != "properties"` left all 82 `rk-types` `tests/it` tests green.** That
    includes `no_oneof_at_any_node` over every `registry::export_all()` schema, `nested_union_is_caught` and the repro.
  - So no exported schema depends on the exclusion, and the fix is one token.
  - The remaining doubt is intent, which no record states.
- **Out-of-run argument:** `rk_types::schema::lint` has no production caller. Its only users are `rk-types`'
  `tests/it` (`acceptance_criterion.rs`, `companion_kinds.rs`, `schema_caps.rs`, `schema_no_oneof.rs`, `support`).
  The run, replay and the `s8i` additions never call it.
- **Rule 2:** `lint.rs`, `schema::lint`, `NestedUnion` and AC-131 are named nowhere in `s8a`…`s8i`, and in no open
  issue.
- **Confidence:** medium-high. The defect is clear against AC-131's words, the fix is proven harmless to the exported
  set, and the only open question is whether the exclusion was a deliberate narrowing nobody recorded. One caution:
  `rk-types` is a leaf every crate depends on, so a fix lane's verify rebuilds the whole workspace.

## Excluded candidates

| # | candidate | rule | why, at `3fa69db` |
| --- | --- | --- | --- |
| C3 | `rk-cli` · `verbs::explain::false_conjunct` / `top_level_conjuncts` (`explain.rs:702`: `if depth == 0 && src[i..].starts_with(" and ")`) | **1 (static walk)** | See "C3's exclusion" below. |
| C2 | `rk-store` · `locator::scan` | 1 | `rk-store/src/render.rs:53` imports `locator`, and `render.rs:1001` and `:1071` call `locator::resolve`. `verbs/run.rs:789`, `:3171-3203` drive `rk_store::render` on every stage. Unchanged from §8j.5. |
| C6 | `rk-store` · `locator::parse_heading` | 1 | As C2. |
| — | `rk-types` · `Digest`'s derived `Deserialize` skipping `from_hex` | 1, and now **2** | Digests are deserialised on the run and replay paths. Also scheduled by open issue **#19** (*"Make Digest deserialisation validate through from_hex"*). |
| — | `rk-kernel` · `toml_subset` admitting a table defined twice | 1 | The pack and catalog load path the run takes (`catalog.rs:29` reads through `toml_subset`). |
| — | `rk-kernel` · `catalog::walk` dropping a non-string `deviation_ok` entry | 1, and **2** | The catalog load path (`catalog.rs:1005`). Also within open issues **#10** and **#49** (strict catalog header loading) and **#50** (`deviation_ok` in the digest). |

**C3's exclusion.** The repro was run anyway, so the result is on record:

- **The walk.** `s8`'s blocks run `./target/debug/redkiln explain "$ITEM"` four times (`s8-exit-criterion.md`:38, 58,
  111, 117). From there the call chain is:
  1. `explain::run`
  2. `run_with` (`explain.rs:105`)
  3. `render` (`explain.rs:391`), which prints an `excluded: <id>` / `clause: …` pair for every same-stage catalog
     activity not in the recorded DAG (`explain.rs:609-617`)
  4. `excluding_clause` (`explain.rs:629`)
  5. `false_conjunct` (`explain.rs:638`)
- **Why that fails rule 1.** Rule 1 drops *"explain's stage and output paths"*, and this chain is explain's output
  path.
- **What the `ca8f387` pass argued.** It kept C3 on the argument that *"this path runs only when explaining a false
  predicate"*. That is a dynamic argument, and rule 1's check is static.
- **The dynamic side.** No shipped `applicable_if` in `.redkiln/catalog` or `spikes/p9/catalog` has a quoted ` and `,
  or an unparenthesised `or … and` mix, so today's `s8` output would not change.
- **Red** (a `kind = "unit"` `#[cfg(test)] mod s8j_repro` appended to `explain.rs`, `cargo test -p rk-cli --lib
  s8j_repro`): both tests fail.
  ```text
  a_disjunction_is_not_decomposed:          left: Some("kind == bug or size >= high")   right: Some("kind == bug or size >= high and novelty >= high")
  a_quoted_and_is_not_a_conjunct_boundary:  left: Some("kind == 'x")                    right: Some("kind == 'x and y'")
  ```
  The grammar binds `and` tighter than `or` (`rk-kernel/src/predicate/parser.rs:171`, `or_expr` over `and_expr`). So
  `a or b and c` is a disjunction, and the doc at `explain.rs:676-677` says a disjunction *"is **not** decomposed"*.
- **How C3 could be re-admitted.** Only by Ryan ruling that rule 1's static walk admits an `explain` branch whose
  inputs no shipped catalog produces. Re-admitted, it would carry the caveat that `s8` reads this output.

## Survey notes

- **Static walk, the `s8i` additions (drift 10).** None of the following references `fence::`, `doctor::rules`,
  `explain::` or `schema::lint`:
  - `rk-ports/src/toolchain/binary.rs` and `binary/text_patch.rs`
  - `rk-store/src/render/binary.rs`
  - `rk-kernel/src/workflow/gate.rs`
  - `rk-cli/src/review_input.rs`
  - `rk-cli/src/verbs/run/kept_refusal.rs`

  So `s8i` added no path to C1, C4 or C5.
- **Rule 1's dynamic half is not runnable yet.** §8j.5 makes *"all ten `lane_proof` tests stay green with the repro
  added"* a necessary check. `lane_proof` does not exist at HEAD: it is `S8J-C4-LANE-PROOF`'s output, and
  `grep -rl lane_proof crates` is empty. The check is owed once C4-LANE-PROOF lands, before the reservation.
- **Rule 2 against the open issues:** 63 open issues were read through the REST API on 2026-10-18 and their titles and
  bodies grepped for each candidate's file, function, rule id and AC. Batch 1's #5, #6, #7, #11 and #20 are fixed and
  not in that set. The only scheduling hits are on the excluded `Digest` (#19) and `catalog::walk` (#10, #49, #50)
  candidates.
- **Rule 2 against `s8a`…`s8i`:** `grep -F` of every candidate's file name, function name and rule id over
  `runbook/phase-1/s8[a-i]*.md` (including `s8d-ii`). The file-level hits on `doctor/rules.rs` are listed under C4. No
  stage names a candidate function.

## Recommendation (not a decision)

**C4.** It has the strongest contract of the three. AC-377's own Verified-by names this exact fixture (*"a fixture
config with no margin that must fail"*), the run's loader already refuses the pack that doctor passes, and the sibling
rule `RK-D-TTL` shows the missing arm. It also has the clearest user-visible impact: `redkiln doctor` reports green on
a configuration `redkiln run` refuses. The fix is one function in one crate, on the non-`--rebuild` doctor path only,
which no `s8` block runs. Its `kind = "it"` repro sits in the existing doctor harness, so the lane's `red` step has a
plain failing assertion to read.

- **Fallback: C1.** Lowest risk, a one-line fix, and the furthest from the run, but it fixes a field nothing reads.
- **C5:** a one-token fix proven green over the exported set. It is ranked below C4 only because its intent question
  is unrecorded, and because a change to `rk-types` rebuilds every crate in the lane's verify.
