# streams/ — the scripted stream-json parts the fixture child replays (s7c §7c.17)

**No raw `claude -p` stream transcript exists at the corpus pin** (§7c.17: the runner retained only
its parsed view, re-serialised with sorted keys and `f64` numbers). So every transcript a test feeds
the fixture child (`rk-fixture-child-claude` in `replay:<file>` / `wait-stdin-eof:<file>` mode) is
**composed at test time** from the parts here plus the extracted rows under `../recorded/`, and
**every line is labelled for what it is by its file name** — the label the stage's rule requires:

**Why `.ndjson` and not `.jsonl`:** `ci.yml`'s AC-465 corpus-residency rule is keyed on the `*.jsonl`
extension (every tracked `.jsonl` outside its allowlist fails CI) — these parts are not corpus artifacts and
are named so the rule does not have to be widened for them; `../recorded/` keeps `.jsonl` because those ARE the
allowlisted extractions.

| suffix | label | meaning |
| --- | --- | --- |
| `.scripted.ndjson` | `// SCRIPTED — not recorded at the pin` | an authored line: every `system/init` (the CLI's init is unrecorded), the scripted result frames, the non-`allowed` `rate_limit_event`, the usage-limit text-marker result |
| `.synthetic.ndjson` | `// SYNTHETIC` | a value no recorded row carries: AC-374's trailing-zero digits, the utilization-inside-a-string line, the SDK flat shape (its key names are an assumption — [surface unverified]), the family-keyed windows, the three-decimal reading, the usage-limit `subtype` frame (the CLI's usage-limit subtype string is an assumption — [surface unverified]), the three unknown event types AC-200 injects |
| `.reconstructed.ndjson` | `// RECONSTRUCTED from results/e3.jsonl:6 — raw line not retained at the pin` | the two `system/api_retry` lines re-synthesised from the runner's PARSED `step.api_retries` fields of line 6 (`../recorded/e3.L6.api_retries.jsonl`: `attempt`, `max_retries`, `retry_delay_ms`, `error`, `error_status`) — the raw stream lines were not retained |

The RECORDED frames a transcript carries come from `../recorded/` (byte-exact extractions with
`PROVENANCE.md`): the seven `e3.L<n>.result_raw.jsonl` `is_error` frames, `f20.result_raw.jsonl`
(exit 0, no `structured_output`), `f20.unknown_events.jsonl` (five `tool_progress` frames) and the
`rate_limit_event` inside `e1-smoke.L1-3.jsonl`'s probe rows (the tests cut its byte span out of
the row at test time — `// RE-SERIALISED at the pin (runner Value; key order and number repr are not
the CLI's)`).

## The parts

| file | what it is / which test |
| --- | --- |
| `init.scripted.ndjson` | the good init: `agent: probe`, `tools: ["StructuredOutput"]` (EQUAL to the probe card's list), `mcp_servers: []`, `claude_code_version: 2.1.277` (the pin; the version key is [surface unverified] — `init.rs`'s `VERSION_KEYS`). `cwd` is the literal `<worktree>` — the child's real cwd is `echo-env`'s line, not this |
| `init-wrong-agent.scripted.ndjson` / `init-no-agent.scripted.ndjson` | AC-204's fourth case → `NotAdopted` |
| `init-superset.scripted.ndjson` / `init-subset.scripted.ndjson` | AC-204/AC-386: extra tools / no tools → `NotIsolated { surface: tools }` |
| `init-same-count-substituted.scripted.ndjson` | AC-202: the card's tool COUNT with one substituted name — a count-based check is green here, a set digest is red |
| `init-280-tools.scripted.ndjson` | AC-202: 280 tools while argv carries the flag |
| `init-mcp-servers.scripted.ndjson` | AC-202: a non-empty server list → `NotIsolated { surface: mcp_servers }` |
| `init-no-mcp-key.scripted.ndjson` | AC-202: an init with NO `mcp_servers` / `mcpServers` key at all — the absent list is the EMPTY set, which equals the empty set `--strict-mcp-config` declares, so the dispatch proceeds (`init.rs`'s module doc states the asymmetry with `tools`, where the empty set is unequal to any card's non-empty list) |
| `init-version-higher.scripted.ndjson` / `init-version-lower.scripted.ndjson` | AC-226's two versions × two policies; AC-385 |
| `init-no-version.scripted.ndjson` | an init with no version key: nothing measured, the row carries `Pinned` |
| `result-ok.scripted.ndjson` | `structured_output: {ok: true}`, usage with a verified-1h split, `total_cost_usd` 0.0031 |
| `result-ok-structured-output-quotes-marker.scripted.ndjson` | AC-523's haystack, pinned: a valid `structured_output` whose OWN text quotes a text marker (`plan limit`) — the marker scan reads the result TEXT member only, never the whole frame (the runner scanned the whole frame; the narrowing is an edit named in `classify.rs`'s header), so this is `Ok` |
| `result-ok-null-rate-limit.scripted.ndjson` | AC-219/AC-380: `rate_limit: null` on the result — never a source |
| `result-invalid-structured-output.scripted.ndjson` | a value the schema refuses → `Schema` |
| `result-unknown-usage-key.scripted.ndjson` | AC-201's second red: an unknown key on `usage` |
| `result-prose-json-no-structured-output.scripted.ndjson` | AC-208: the prose IS valid JSON and there is no `structured_output` → `Schema`, never `Ok` |
| `result-no-usage.scripted.ndjson` | AC-267/AC-362: no usage → `None`, never `Some(0)` |
| `result-usage-limit-text-marker.scripted.ndjson` | AC-523's disclosed fallback: the text says `Usage limit reached` |
| `result-usage-limit-subtype.synthetic.ndjson` | AC-523's `subtype` arm: `subtype: error_usage_limit` — SYNTHETIC, [surface unverified] |
| `assistant-turn-1/2.scripted.ndjson` + `result-two-turns-summed.scripted.ndjson` | AC-267: per-turn usage on two assistant lines, the SUM on the result — the record carries the sum, never the last turn |
| `future-shaped-event.scripted.ndjson` | AC-201: a `system/thinking_tokens` line with a future field and a new top-level type |
| `rate-limit-non-allowed.scripted.ndjson` | AC-523: `status: rejected`, `resetsAt 1788370800` — the `resets_at` source (unrecorded at the pin; [surface unverified]) |
| `rate-limit-allowed-later.scripted.ndjson` | AC-523's *last NON-ALLOWED*: an `allowed` reading with a DIFFERENT `resetsAt` (1788388800) placed AFTER the non-allowed one — `resets_at` must still be the non-allowed reading's, and a stream of only `allowed` readings yields `None` (`resets_at_from_last_non_allowed_not_last_reading`) |
| `rate-limit-synthetic-digits.synthetic.ndjson` | AC-374's discriminating half: `0.10/0.20`, `0.09/0.29`, `0.57/0.58` — trailing-zero digits are unrecorded at the pin; `0.29/0.57/0.58` are red under a truncating `f64` path |
| `rate-limit-utilization-in-string.synthetic.ndjson` | the scanner's own red: `"utilization":0.99` inside a STRING value beside the two real keys |
| `rate-limit-flat-sdk.synthetic.ndjson` | AC-380's flat SDK shape — its key names are an assumption |
| `rate-limit-family-keys.synthetic.ndjson` | AC-375: `seven_day_opus` / `seven_day_sonnet` tolerated and ignored |
| `rate-limit-three-decimals.synthetic.ndjson` | AC-374: `0.045` is refused, never rounded — through the real `dispatch` the refusal is `Transport { kind: Decode }` naming the key path, taken WITHOUT a mid-run kill (the drain runs to EOF and the row keeps the result frame's usage; `refused_reading_is_decode_without_a_kill`) |
| `rate-limit-one-window.synthetic.ndjson` | AC-375: `unifiedWindows` carrying `five_hour` and not `seven_day` — a reading of exactly two windows cannot be formed; the same `Transport { kind: Decode }` without a kill |
| `rate-limit-at-boundary-five-hour-93.scripted.ndjson` / `…-one-below-…-92` / `…-seven-day-82` / `…-81` | AC-376/AC-220: the boundary and one tick below, at margins 0.07 (five_hour) and 0.18 (seven_day) |
| `api-retry.reconstructed.ndjson` | AC-232's two lines |
| `unknown-events.synthetic.ndjson` | AC-200's three injected unknown types |
| `malformed-where-result-required.scripted.ndjson` | `Transport { kind: Decode }`: a non-JSON line where the result frame is required, exit 0 |

The ≥ 1 MiB prompt that makes `Transport { kind: Io }` deterministic (§7c.11) is generated in the
test (`support::megabyte_prompt`) rather than tracked as a file.
