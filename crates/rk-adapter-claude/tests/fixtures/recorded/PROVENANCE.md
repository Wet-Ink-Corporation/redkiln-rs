# recorded/ — the extracted corpus rows this crate's tests read (s7c §7c.17; RT-23)

**Every file here is a byte-for-byte extraction of whole lines from a file in `redkiln-corpus` at the pinned
commit, read through the object store** (`git -C "$CORPUS_REPO" show "$CORPUS_SHA:<path>"`, never a working
tree — AC-464; PIA-018/PIA-036), where `CORPUS_SHA` = `61f590e51cbb00301c310e46aab615563a7787b8` (the SHA
`CORPUS_PIN` carries; `PINNED_AT=2026-09-15`). **Only the rows the tests read are copied, never the source
file whole** (§7c.17: copying `results/*.jsonl` whole would re-create the residency violation AC-465 names), and
the copying is a ROUTING with a dated in-place amendment to AC-465 OWED (RT-23; §7c.22 RAISED). These paths are
named in `ci.yml`'s AC-465 allowlist (`crates/rk-adapter-claude/tests/fixtures/recorded/[^/]+\.jsonl`).

**What these bytes are, honestly** (§7c.17): the runner's own rows — `serde_json` re-serialisations of parsed
values with sorted keys and `f64` numbers — **not the CLI's bytes**. A `flags` array is the argv the runner
handed to the OS, recorded as data, and is the only thing the Slice 4 tests read from them.

**The source files are CRLF at the pin** (`results/h1-gap.jsonl` carries 15 CR bytes, one per line;
`h1-sysprompt.jsonl` 5) and **every extracted line keeps its trailing CR**: each line is split from the blob on
`
` alone and written back byte-identical, CR included, with a `
` after it — asserted by a byte comparison of
every written line against its source line in the same script (python over the `git show` bytes, 2026-09-18;
a first extraction through `sed | cmp` compared two CR-stripped streams and was corrected on review). The tests
read the rows through `str::lines()`, which drops the CR before `serde_json` sees the line. The sibling
`.gitattributes` marks the directory `-text`, so a checkout cannot convert the bytes and falsify the digests below.

## Files

| file | source at the pin (`$CORPUS_SHA:<path>`) | lines taken | file bytes | file sha256 | source bytes | source sha256 | what it is / which test reads it |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `h1-gap.dispatch-rows.jsonl` | `61f590e51cbb00301c310e46aab615563a7787b8:results/h1-gap.jsonl` | 2, 4, 7, 9, 12, 14 (the six dispatch rows of 15 lines — the rest are mark/read rows with no `flags`) | 31,557 | `f573b4f52ddb7026fff7b46188081472e12ea4e97da07c84c740e50114bdd444` | 33,834 | `4fd395ab2fb56332d99427ad5d669bc3882e3490960d7234f10df0e45082f8cb` | F28's `default` arm: every row's `flags` is `["-p", "--output-format", "stream-json", "--verbose", "--model", "haiku", "--strict-mcp-config"]` — the measured element set and the order of the first six of the golden `default` argv (§7c.5). Read by `golden_argv_per_profile`. |
| `h1-sysprompt.dispatch-rows.jsonl` | `61f590e51cbb00301c310e46aab615563a7787b8:results/h1-sysprompt.jsonl` | 2, 4 (the two dispatch rows of 5 lines) | 10,651 | `055f750c3fa186b6309c66ff583f04bb1c1ecdad1307124f1d9a6780eb27b4f7` | 11,417 | `0ec4f17f38f0301f6f74a98423f60f622cc73d2bf3fed35fbb41c8a0e2edc43d` | the `static` arm: the same seven elements followed by `"--system-prompt", "You are a review assistant. Answer concisely."` — a 45-byte system prompt, LAST on the vector, `claude_version` `2.1.261 (Claude Code)`. Read by `golden_argv_per_profile`. |

## Not extracted at Slice 4

The seven `is_error` frames of `results/e3.jsonl` (lines 14, 16, 17, 18, 19, 21, 26), its line 6 (`api_retries`) and
`results/instrument-checks/e1-smoke.jsonl`'s three rows are Slice 5's reads (AC-218/AC-428, AC-232, AC-374/AC-380/
AC-221/AC-200) and are extracted there, into this directory, with their own rows in this table (§7c.20 item 4:
*"the e3/e1-smoke frames may wait for Slice 5 unless a Slice-4 test reads them"* — none does). The F20 row is
already on the tree as 7b-ii's copy (`crates/rk-kernel/tests/fixtures/findings/ac282_f20_golden_transcript.jsonl`,
24,794 bytes, sha256 `457cfa4abc925c28799c8cda47666c1acd5841953ab4616e22c54685d39a33ee`) and is not duplicated here.
