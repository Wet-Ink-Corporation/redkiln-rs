# DRAFT · §7b.11's golden-log layout amendment, and the `SCHEMA` bump to v2

**Status: DRAFT.** Executes **decision D-4** of 2026-09-14. Two halves with different owners and a fixed order:

- **The runbook half** (below, §2) edits `runbook/phase-1/s7b-kernel.md`. Apply on request.
- **The code half** (§3) edits `crates/rk-kernel/src/replay.rs` and re-blesses one fixture. **Held** while the
  last-two-names build is editing that crate; apply once the tree is clean.

**Both must land before the `CORPUS_PIN` move**, because §7b.11 precondition 3 is an assertion about this very
section — *"The `golden-logs/` per-run layout is written into this section"* — and the move copies these logs.

---

## 1. The drift, measured

§7b.11's block is headed *"FIXED 2026-09-13 in the commit that authored the replay test"* and has not moved since,
while the code changed shape twice. Measured 2026-09-14 against `crates/rk-kernel/src/replay.rs`:

| | §7b.11 says | the code writes | added by |
| --- | --- | --- | --- |
| `run.json` | `{ schema, run_id, stage, inputs_digest, event_count, producer }` — **6** | the same **+ `terminal_state` + `next_action`** — **8** | Slice **4f** |
| `inputs/` | pack, catalog, artifacts, facets.toml, context.json, artifact-state.json, deviation.json — **7** | the same **+ `runtime.json` + `specs.json`** — **9** | 4f and **4i** |
| `inputs/runtime.json` | *(absent)* | `{ tree_hash, deadline, entered_from, results_at, successor }` | 4f, `results_at` at 4i |
| `schema` | `rk-kernel/golden-log/v1` | still `v1` — for a **third** distinct shape | — |

`grep -c 'specs.json\|results_at' runbook/phase-1/s7b-kernel.md` → **0**.

---

## 2. The runbook half — replace §7b.11's layout block

Replace the paragraph and fenced block under **`#### The per-run layout, FIXED 2026-09-13 …`** with:

> #### The per-run layout — FIXED 2026-09-13, **amended 2026-09-14 and the version bumped to v2**
>
> The producer is `rk_kernel::replay::{record, write_run}` and the reader is `rk_kernel::replay::read_run`; the
> layout version travels in the file as `schema = "rk-kernel/golden-log/v2"`, so a reader never guesses which
> shape a run is in.
>
> ```text
> golden-logs/
>   <run_id>/                     run_id = "<stage>-" + first 16 hex of
>                                 sha256_lp(["rk-kernel/golden-log/v2", stage, canonical(inputs)])
>     run.json                    { schema, run_id, stage, inputs_digest, event_count, producer,
>                                   terminal_state, next_action }
>     inputs/
>       pack/<name>.toml          the pack, verbatim
>       catalog/<name>.toml       one file per activity, verbatim
>       artifacts/<name>.toml     the artifact-kind registry, verbatim
>       facets.toml               the facet registry, verbatim
>       context.json              the item's CONTEXT digest preimage, canonical, LF-terminated
>       artifact-state.json       the item's ARTIFACT digests preimage, canonical, LF-terminated
>       deviation.json            OPTIONAL: the RequestDeviation and its requesting activity
>       runtime.json              { tree_hash, deadline, entered_from, results_at, successor } —
>                                 the values the FSM needs and this crate cannot compute
>       specs.json                DAG node id -> { activity_id_at_version, card_digest, tools,
>                                 schema, timeout_secs, inputs } — the binder's output, recorded
>     events.jsonl                one canonical JSON object per line, every line LF-terminated
> ```
>
> **Amended 2026-09-14, and the version bumped in the same act.** `run.json` gained `terminal_state` and
> `next_action` at Slice 4f — the **recorded** side of AC-006 assertion 3 and AC-008's terminal-state half, which
> a replay compares against rather than recomputing from the same journal twice. `inputs/` gained `runtime.json`
> at 4f (`results_at` at 4i) and `specs.json` at **4i**, when `record()` began driving real dispatches and N
> stopped being 0. **`read_run` refuses a log missing any of them**, naming the file or member, so a log written
> under the old shape goes red rather than replaying with nothing to compare.
>
> 🛑 **The version bump is not cosmetic and its timing is forced.** The schema string is an **input to the run
> id** — see the derivation above — so bumping it **renames every run**. At the moment of this amendment the
> corpus holds **no** golden logs, so the bump costs one re-bless of one in-crate fixture through the
> `#[ignore]`d `bless_the_committed_golden_log`. After the single `CORPUS_PIN` move it would invalidate
> corpus-committed run ids under a SHA **AC-464 freezes** and `PROGRESS`:103 forbids moving twice. Three distinct
> shapes had come to share `v1`, against this section's own stated reason for carrying a version at all.

**Then update the same file's two other references**, so the section does not contradict itself:

- the sentence above the block naming `schema = "rk-kernel/golden-log/v1"` — covered by the replacement above;
- §7b.11's *"The order"* paragraph and precondition 3, which need no wording change but should be re-read against
  the amended block before the move.

---

## 3. The code half — `SCHEMA` → v2, and one re-bless

1. `crates/rk-kernel/src/replay.rs`: `pub const SCHEMA: &str = "rk-kernel/golden-log/v2";`
2. Run the `#[ignore]`d act deliberately: `cargo test -p rk-kernel --features corpus -- --ignored bless_the_committed_golden_log`.
3. The committed fixture directory **renames** — every `run_id` changes, so
   `crates/rk-kernel/tests/fixtures/logs/<old>/` is deleted and `<new>/` written. Confirm by `git status` that
   exactly one tree replaced one tree, and that `events.jsonl` is byte-identical apart from nothing (the events do
   not depend on `SCHEMA`; only the directory name and `run.json`'s `schema` member move).
4. Re-run the six gates and the replay group, including all five controls — writing, dispatching, terminal-state,
   next-action, missing-result, tampered-input.
5. `read_run`'s refusal message for an old log should name **v1** explicitly, so the error says *why* rather than
   only *what*.

**One check worth running because it is cheap and the failure would be silent:** after the bump, assert that
`run_id` actually changed. If it did not, the schema string is not reaching the derivation and the whole warrant
for bumping now rather than later is void.

---

## 4. What this amendment does NOT do

- **It does not move the pin.** That is §7b.11 precondition 7's, the operator's, and this amendment is one of its
  preconditions rather than a substitute for any.
- **It does not settle where `ActivitySpec` assembly belongs in production.** `specs.json` records the binder's
  output because the binder is unbuilt; **D-11** put its build at **7c** (mechanism plus fixture cards, shipped
  cards cross-listed to 7e). A record routing spec assembly is still **OWED**, and the layout block deliberately
  describes `specs.json` as *"the binder's output, recorded"* rather than implying the kernel computes it.
- **It does not claim AC-008 is fully discharged.** N is 2 and the dispatch half is measured; the spec remains a
  recorded input rather than a bound binder output, and `replay.rs`'s header carries that.
