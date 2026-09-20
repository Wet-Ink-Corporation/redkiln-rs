# `exit/<name>/` — AC-438's binary arm, and the five rows it does NOT yet carry

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
| `cancelled-timeout` | `13` | a `hold` child under `[activity.emit-report] timeout_secs = 1` |

## 🛑 FIVE OF GAP 21's NINE ROWS ARE MISSING, AND IT IS A MEASUREMENT RATHER THAN AN OVERSIGHT

GAP 21 derives nine rows for this arm — `0, 2, 3, 5, 10, 11 (×2), 13, 18`. Four are above. The
other five are **RAISED**, with what was measured on 2026-09-20:

### `10` Transport, `11` Schema, `14` NonZero, `18` Platform — the class is never journalled

§7d.4 asks the binary to reach `Transport` through a missing `harness_binary`, `Schema` through a
malformed frame and `Platform` through an `is_error` frame. It cannot, because **nothing a real run
writes names the failure class**:

- the only callers of `StepError::event()` in any `src/` are `rk-ports`' `fakes.rs` — `FakeHarness`'s
  own bridge — and a discarded `let _ =` in `conformance.rs`;
- `ClaudeHarness::dispatch` ends every path through `finish()`, which appends **one**
  `dispatch_completed` row carrying a serialised `DispatchRecord`. That record has `cancelled_by`
  and `detected_by` and **no class member**;
- `Runtime::run_activity` appends only `dispatch` and `activity_committed` — a step's own failure is
  *"a value in the report (AC-032), never an error of this function"*;
- `replay::dispatch_through` then turns the uncommitted report into
  `ReplayError::NotCommitted { step_id, activity_id_at_version }` — two `String`s, no `StepError`.

So `run`'s step (10) has nothing to read, and §7d.5's *"a `StepError` → its row"* is not
implementable for the real adapter as the tree stands. Measured directly: a wired `redkiln run
bug-1` against a fast-exiting child prints *"the dispatch did not commit and the journal carries no
error row"* and exits `3`.

**Row `13` is the exception, and it is why it is above**: `cancelled_by` IS on the record, so step
(10) recovers that one class and nothing else.

Two mechanisms would close the gap. **Neither is chosen here, because the choice is above this
stage** — §7d.4 says the CLI reads the class but names no mechanism for the real adapter:

- **A — a crate-local `Harness` decorator.** Step (8) wraps the real harness in an `rk-cli` type
  that, after delegating `dispatch`, appends the class row for an `Err` outcome, built from the
  public `StepError::event()`. No kernel or adapter edit, and it produces exactly the rows
  `exit_for_error_row` already reads — but the shipped binary would then journal rows the adapter
  deliberately does not.
- **B — widen `ReplayError::NotCommitted` to carry the terminal class.** The value is already in
  hand at `replay.rs`'s refusal (`dispatch_through` holds the `ActivityReport`, and
  `ActivityReport::outcome()` is public), and step (10) would map `From<&StepError>` directly with
  no journal round trip. Cost: a **fifth** kernel edit for Slice 3, where §7d.16 item 3 enumerates
  four, and Checkpoints 7b/7b-ii re-run whole.

### `5` NoRecord — not this verb's

`Exit::NoRecord` is `explain`'s (*"errors without resolution"*), and nothing in `crates/rk-cli/src`
returns it yet. It lands with `explain`, at cycle 9.

### `11` twice

GAP 21 wants two `11` directories — the malformed frame and F20's absent `structured_output`. Both
need the class, so both wait on the decision above. Note that they are **different paths**: the
malformed frame is the adapter's own `Schema`, while F20's is minted by
`Runtime::run_activity` when a returned `Typed`'s schema id is not the spec's — on which the
harness returned `Ok` and journalled nothing at all.
