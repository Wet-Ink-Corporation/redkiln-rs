# `tests/fixtures/findings/` — the review gate's fixtures (Step 7b-ii, `s7b-ii` §7b-ii.10)

Every file here is named for the criterion it discriminates. Where a fixture must be byte-identical to a record it is
**copied in, never pointed at** (§7b-ii.10), and the copy's provenance is recorded here.

| file | criterion | provenance |
| --- | --- | --- |
| `ac282_f20_golden_transcript.jsonl` | AC-282, AC-318 | byte-identical copy of `results/e3-pilot.jsonl` in `redkiln-corpus` at `CORPUS_SHA` `61f590e51cbb00301c310e46aab615563a7787b8` (the corpus was at the pin and clean when copied, 2026-09-17); 24,794 bytes, sha256 `457cfa4abc925c28799c8cda47666c1acd5841953ab4616e22c54685d39a33ee`. The one recorded `claude -p` dispatch anywhere in the corpus that carries `--json-schema` in its flags, exits **0** with `subtype: success`, and has **no `structured_output` key** in `step.result_raw` — F20's measured platform behaviour (a card's `tools` allowlist voided the schema), the E3 pilot dispatch of 2026-09-03 on claude 2.1.258. The kernel's ingest boundary takes `.step.result_raw`, the stream-json `result` event. |
