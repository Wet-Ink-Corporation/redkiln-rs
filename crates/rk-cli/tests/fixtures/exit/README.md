# `exit/<name>/` — AC-438's binary arm, and the nine rows it now carries

One directory per expected code (§7d.13: *"one directory per expected code, with an `expected` file
carrying the integer"*), plus a `case.toml` saying how the code is reached, so the fixture is
readable without the test. `ac438_table_driven_exit_codes_over_fixtures_binary_arm` drives them and
asserts the directory set in both directions — a table that walked the directory alone would pass
over an empty one.

| directory | code | how |
| --- | --- | --- |
| `gate-open` | `0` | `gate-1` omits `has_repro`; the abstention gate opens and step (10) trailers it |
| `resume-flag` | `2` | `--resume` is not in the parser tree; clap's usage code, kept (H-1) |
| `no-pack` | `3` | the `init` scaffold's `enabled = []` — D18-8's refusal, before any lane derivation |
| `no-record` | `5` | a `pack-min` repository never run, so no `activity_committed` row resolves `gate-1` to a run id |
| `transport-missing-binary` | `10` | `[adapters] harness_binary` names a program that is not there |
| `schema-no-structured-output` | `11` | a result frame carrying **no** `structured_output` — F20 |
| `schema-invalid-structured-output` | `11` | a `structured_output` that **fails** the dispatch's own schema |
| `cancelled-timeout` | `13` | a `hold` child under `[activity.emit-report] timeout_secs = 1` |
| `platform-is-error` | `18` | a result frame carrying `is_error: true` |

That is GAP 21's set — `0, 2, 3, 5, 10, 11 (×2), 13, 18` — complete. **Never a `12`**: §7d.4 is
explicit that the adapter classifies its timer arm `Cancelled { by: Timeout }`, *"so this fixture is
the binary-level instance of row `13`, not `12`, and a fixture whose `expected` reads `12` is red on
a correct binary"*. `70` is exercised in process only.

## How four of these became reachable: mechanism B, taken

Four rows — `10`, `11` twice and `18` — were **RAISED and absent** until Step 7d Slice 3 cycle 10.
This section is now the record of the decision rather than the statement of the problem.

What was measured, and why it blocked them: **nothing a real run wrote named the failure class.**
The only callers of `StepError::event()` in any `src/` were `rk-ports`' `fakes.rs` — `FakeHarness`'s
own bridge — and a discarded `let _ =` in `conformance.rs`. `ClaudeHarness::dispatch` ends every
path through `finish()`, which appends one `dispatch_completed` carrying a `DispatchRecord` with
`cancelled_by`, `detected_by` and **no class member**. `Runtime::run_activity` appends only
`dispatch` and `activity_committed`, because *"a step's own failure is a value in the report
(AC-032), never an error of this function"*. And `replay::dispatch_through` then discarded the
report into `ReplayError::NotCommitted { step_id, activity_id_at_version }` — two `String`s. So
`run`'s step (10) had nothing to read, and a wired `redkiln run bug-1` against a failing child
printed *"the dispatch did not commit and the journal carries no error row"* and exited `3`.

Row `13` was the exception, and it is why it was here from cycle 6: `cancelled_by` IS on the record,
so step (10) recovered that one class and nothing else.

Two mechanisms would have closed it and the choice was above this stage. **The operator chose B.**

- **A — a crate-local `Harness` decorator.** Step (8) wraps the real harness in an `rk-cli` type
  that appends the class row for an `Err` outcome, built from the public `StepError::event()`. No
  kernel or adapter edit — but the shipped binary would then journal rows the adapter deliberately
  does not. **Not taken.**
- **B — widen `ReplayError::NotCommitted` to carry the terminal class. TAKEN.** The value was
  already in hand at the refusal: `dispatch_through` holds the `ActivityReport` nine lines above the
  construction and `ActivityReport::outcome()` is public. Step (10) now maps
  `impl From<&StepError> for Exit` directly — eleven arms, no wildcard — with no journal round trip.

Its consequences, stated rather than left to be discovered:

- It is a **fifth** `rk-kernel` edit for Slice 3, where §7d.16 item 3 enumerates four. RAISED.
- Checkpoints 7b and 7b-ii re-run whole. No adapter crate references `ReplayError` at all
  (`grep -rln ReplayError crates/rk-adapter-*/` → empty), so §7d.15's adapter-edit ban was never
  triggered.
- `exit_for_uncommitted` and `exit_for_error_row` are **deleted**. They recovered the class by
  re-reading the journal, which worked only for `FakeHarness`; once the class arrives on the refusal
  they are unreachable for both harnesses, and a branch no test can redden is deleted rather than
  kept as a fallback. The guard they gave was redundant by their own documentation — *"the same
  guarantee `impl From<&StepError> for Exit` gives one level up"* — and
  `ac438_step_error_mapping_is_exhaustive` remains the single mapping instrument.

## 🛑 §7d.4's *"`Schema` via a malformed frame"* DOES NOT REPRODUCE, and this is measured

§7d.4 names the three adapter-reachable routes as *"`Transport` via a missing `harness_binary`,
`Schema` via a malformed frame … `Platform` via an `is_error` frame"*. The first and third
reproduce exactly. **The second does not.**

A transcript of a system/init frame followed by `this is not json {"type":"result"` yields **`10`,
not `11`** — measured through the binary on 2026-09-20. The cause is `classify.rs`'s numbered order:
a stream that yields no parseable result frame reaches rule 8, *"the child exited 0 and the stream
carried no result frame (a malformed line or none where a frame is required)"* →
`StepError::Transport { kind: Decode }`. `extract_digest`, which is where `Schema` is minted, is
rule 10 and is never reached.

So a `schema-malformed-frame/` directory reading `11` would be **red on a correct binary**, in
exactly the way §7d.4 warns a `12` would be. It was authored, measured, and removed rather than
re-labelled to `10` — the set already carries a `10`, and two would not be GAP 21's nine.

GAP 21's second `11` is authored instead as the **other arm of `extract_digest`**:
`schema-invalid-structured-output/` carries a `structured_output` that is present and fails the
dispatch's own schema, where `schema-no-structured-output/` carries none at all. Those are two
genuinely different branches of the same function and both mint `StepError::Schema`.

*(GAP 21 described the second `11` as one "minted by `Runtime::run_activity` when a returned
`Typed`'s schema id is not the spec's". That reading is not the contract's — FIXTURES §3 names the
row `schema-no-structured-output`, "the CLI's end of F20", which is the adapter's own. Both rows
here are adapter-side.)*

## The transcripts

`frames.jsonl` appears only in the three directories that drive the real adapter through the
fixture child. `RK_FIXTURE_MODE=replay:<file>` makes `rk-fixture-child-claude` `fs::read` the path
and `write_all` the bytes verbatim, so the path must be **absolute** — which a committed `case.toml`
cannot carry. `case.toml` names the file and the test resolves it against the case directory.

They are **synthetic**: authored against `classify.rs`'s numbered order and modelled on the shapes
in `crates/rk-adapter-claude/tests/fixtures/streams/*.scripted.ndjson`, never copied out of that
crate's `recorded/` directory, which is corpus-provenanced. They are named `.jsonl` rather than
`.ndjson` so that `ci.yml`'s AC-465 allowlist row — keyed on `*.jsonl` — actually matches them; a
row matching nothing would be an entry nobody could tell was wrong. That row is `(d)`, anchored
`[^/]+` on both variable segments, never `.*`.
