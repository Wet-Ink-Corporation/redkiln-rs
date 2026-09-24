# `cost/` — `redkiln cost`'s fixture corpus (ADR-0163 decision 6)

The dispatched case of Checkpoint 7i, proved over fixtures because the shipped store holds **zero**
`dispatch_completed` rows and `0 of 102` shipped activities complete a committing walk. This corpus
proves the **wiring** — producer, journal, verb, rendering — and not the catalog: every frame and
every row here is **SYNTHETIC**, and no rendering in it was produced by real account traffic.
ADR-0162 took the same route for the exit table (`../exit/`).

Byte-pinned by `.gitattributes` (`crates/rk-cli/tests/fixtures/cost/** -text`): `run-frames.jsonl`
is replayed verbatim to the fixture child's stdout, and each `expected.stdout` is compared to the
binary's stdout **byte for byte**, so a CRLF smudge would fail it for a reason unrelated to `cost`.

## `run-frames.jsonl` — the one real run

Three stream-json frames that `rk-fixture-child-claude` replays under
`RK_FIXTURE_MODE=replay:<absolute path>`, so a real `redkiln run bug-1` over `pack-min`'s base
variant dispatches through `ClaudeHarness`:

1. the `system/init` frame, copied byte-for-byte from
   `../exit/schema-invalid-structured-output/frames.jsonl` line 1;
2. a `rate_limit_event` in the shape of
   `crates/rk-adapter-claude/tests/fixtures/streams/rate-limit-allowed-later.scripted.ndjson` —
   `five_hour` 0.04 resetting at `1788370800`, `seven_day` 0.37 resetting at `1788408000`, and a
   top-level `resetsAt` (`1788300000`) that differs from both, so a row that carried the top-level
   value instead of the per-window one would render a different `resets_at`;
3. that exit fixture's `result` frame, whose `structured_output` fails `ImplReport.json` — so the run
   exits `11` (`Schema`), and the reading is journalled on a **failed** dispatch, as ADR-0163
   decision 2 requires.

`cost::cost_after_a_real_run_renders_low_sample_through_the_binary` runs it and then `redkiln cost
bug-1`, which renders `n=1 … low_sample`: the one rendering a single run can reach.

## `<case>/` — seeded journals for the renderings one run cannot reach

`n >= 10`, a reset crossing, interleaving, a `null` `resets_at`, a flat reading and a fall inside
one window each need more than one dispatch, so each is a seeded journal. A case directory holds:

- **`rows.jsonl`** — one `{"kind":"<wire name>","payload":{…}}` per line, in journal order, appended
  to a fresh repository's store before `redkiln cost bug-1` runs. Reading payloads carry the fixed
  six-key shape of `rk_adapter_claude::headroom::reading_payload` (`windows`, `utilization`,
  `resets_at`, `rate_limit_type`, `item`, `cli_version`), with `cli_version` in the shape
  `DispatchRecord` serialises it — `{"observed":"2.1.278"}`, the version `run-frames.jsonl`'s init
  reports (AC-227); kernel `dispatch` rows are
  `{"activity_id_at_version":"a@1","attempt":1,"item":"<id>","step_id":"s"}`; `dispatch_completed`
  is noise with a minimal payload, because it carries no item. The target item is `bug-1`; the
  foreign one is `story-1`. `empty/rows.jsonl` is zero bytes.
- **`expected.stdout`** — the exact bytes `redkiln cost bug-1` must print, **computed by hand** from
  ADR-0163 decision 4 and `throttle.rs`, then confirmed against the binary — never copied from it.
  Empty (zero bytes) when the case expects a refusal.
- **`expected.exit`** — optional; one integer and a newline. Absent means `0`. When present and
  non-zero, the binary must exit with it and print nothing on stdout.

### The two refusal cases

Both expect exit `3` (`ConfigError`): an **owned** reading in the span that is not the six-member
ADR-0163 decision 3 shape is malformed, never skipped and never read as "no sample". Foreign rows
are never validated.

- **`malformed-owned-missing-member/`** — one owned reading whose payload lacks `resets_at`. A
  missing key is malformed, not `null`.
- **`malformed-owned-in-interleaved-span/`** — an owned reading, a foreign (`story-1`) dispatch and
  reading, then an owned reading carrying both a `windows` object and a non-`null` `utilization`
  (exactly one of the two may be non-`null`). The span interleaves, so a verb that counted
  interleaving before validating would print `not attributable` and exit `0`; every owned reading
  is validated first.

### The seven cases the adversarial passes added

Six came from the first pass; `flat-unwindowed-other-type/` came from the second, which found two
mutants of `unwindowed` that no case killed. Each `expected.stdout` was computed by hand from
`cost.rs` and `throttle.rs`, then confirmed against the binary (they agreed on the first run).

| case | journal | renders | red by |
| --- | --- | --- | --- |
| `empty-counts-dispatch-rows-only/` | a `bug-1` `dispatch`, a `memo_miss` carrying `item: bug-1`, a `dispatch_completed`; no reading | the empty line with `(1 dispatch rows)` | the empty case counting every row that carries the item (`2`) |
| `flat-unwindowed/` | two owned flat readings (`windows: null`, `utilization` 7 then 8) whose `rate_limit_type` is `null` | the header, `five_hour: no sample`, `seven_day: no sample`, `unwindowed=2` | a reading naming no window dropped silently (no `unwindowed` line) or written into both windows |
| `flat-unwindowed-other-type/` | ONE owned flat reading (`windows: null`, `utilization` 7) whose `rate_limit_type` is the string `"opus_weekly"` | the header (`1`), `five_hour: no sample`, `seven_day: no sample`, `unwindowed=1` | the line guarded on `N > 1` (a single unwindowed reading dropped); the neither-window test reduced to a `null` `rate_limit_type` (another string never counted) |
| `foreign-adjacent-to-span/` | a `story-1` `dispatch` IMMEDIATELY before the first owned reading and another IMMEDIATELY after the last; three owned readings (ticks 20/50, 20/50, 21/50) | `n=3 … low_sample resolution_limited` in both windows — attributable | the span widened by one row at either edge (`not attributable`) |
| `foreign-dispatch-no-reading/` | owned dispatch and reading, an owned dispatch, a `story-1` dispatch that produced NO reading, then the owned reading | `not attributable interleaving=1` | a foreign dispatch counted only when a reading follows it (draws render) |
| `malformed-foreign-in-interleaved-span/` | an owned reading, a `story-1` reading whose `windows` is the string `"bad"`, an owned reading | exit `0`, `not attributable interleaving=1` | a foreign reading validated (exit `3`) |
| `null-splits-a-batch/` | twelve owned readings, five-hour ticks `10 10 11 11 12 12 13 13 14 14 15 15` keyed `1788370800` except the SEVENTH, whose five-hour `resets_at` is `null`; seven-day `40 ×6, 41 ×4, 42 ×2` keyed `1788408000` throughout | `five_hour: n=6 … low_sample` (10→12), `five_hour: n=5 … low_sample` (13→15), `five_hour: unkeyed=1`, `seven_day: n=12 … delta_ticks=2` | the `null` reading not closing its segment (one `n=11` draw with `delta_ticks=5`) |

An owned reading's `cli_version` is validated too: it must be a serialised
`rk_types::usage::CliVersion` — one key, `observed` or `pinned`, holding a string.

`cost::cost_fixture_corpus_binary_arm` walks the case set and asserts it equals its own list in both
directions, so a directory added or removed without the test moving is red.
