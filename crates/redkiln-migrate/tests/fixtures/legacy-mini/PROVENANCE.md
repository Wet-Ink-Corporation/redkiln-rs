# legacy-mini — provenance

**Hand-authored 2026-09-21 for Step 7d Slice 5** (`s7d` §7d.13; D18-10; AC-445, AC-465).

**No byte under this directory is copied from `D:/repos/redkiln` at `47faec9f` or from
`redkiln-corpus` at `CORPUS_PIN`.** The shapes follow the predecessor's frontmatter key set
*as read in the runbook*, not its files. Nothing here is an extract, a sample or a redaction of
a real record: the item ids, ULIDs, titles, prose and signature values are invented for this
fixture and mean nothing outside it.

It carries **fourteen companion kinds, one file each** — the six typed
(`_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger`, `ledger`) and the eight
Phase-2 opaque (`closure`, `design`, `design-review`, `slices`, `storymap`, `implementation`,
`plan`, `retrospective`) — beside the item-record kinds (`initiative`, `project`, `story`,
`spec`), the free-form gate result (`report`) and the legacy configuration (`config`,
`process`): **twenty-one kinds over twenty-three files**.

Two files carry content with no home in any Phase-1 schema, and each must lift with a
migration-loss marker: `story-2/spec.md`'s diff-prose section (AC-475's carry) and
`story-2/report.md`'s free-form gate result (AC-477's carry). `story-1/spec.md` is the clean
one. Summed over `slot`, that is `spec = (lifted 1, with-loss 1, failed 0)` and
`report = (0, 1, 0)` — the counts Checkpoint 7d asserts exactly, so that a folded form
(`lifted 2`) or a dropped marker (`with-loss 0`) is red.

`story-1/_review.md` carries `verdict` and `rubric`, which
`rk_types::companion::ReviewCompanion { item, stage, summary }` has no field for. It lifts
**typed** and counts **with loss**; see `src/legacy/review.rs` for why that is recorded rather
than discovered.

Every item record carries `template_sig` and `rendered_sig`, deliberately: they are what makes
`src/legacy/` the one allowlisted site for those literals (AC-442), and dropping them on the
way out is the clean break (AC-471, AC-474), **not** a migration loss.

No `*.jsonl` and no `spikes/` path (AC-465).

## Bytes

`legacy_mini_provenance_matches_bytes` re-digests every file below and fails on any drift, and
on any file under this directory the table does not list. **`PROVENANCE.md` itself is the one
exclusion**, and necessarily so: a note cannot carry its own sha256.

| file | bytes | sha256 |
| --- | --- | --- |
| `.bklg/init-1/initiative.md` | 293 | `a4d52464aaf66e283af116c642d7d340817600c4fd8e8d0a0de839d70123ea2b` |
| `.bklg/init-1/proj-1/project.md` | 284 | `52661e940592f90d4860ed58a28e78692610277a777816cccd2ff1016604c203` |
| `.bklg/init-1/proj-1/story-1/_decomposition.md` | 80 | `710ae9bab08e070fb7fb281f2a25197577921cd05f9bf208e52cb28148aa32c6` |
| `.bklg/init-1/proj-1/story-1/_grounding.md` | 102 | `1a3563e49e7234666960ce8de49fc28289641f281c8de406685d8f5107b8ee17` |
| `.bklg/init-1/proj-1/story-1/_integration.md` | 96 | `a804b77697e60a4b3daa05b96e2bbe4325eb8c8daca190ed542924761c2dcc72` |
| `.bklg/init-1/proj-1/story-1/_ledger.md` | 118 | `7fa850b216c6de3fec132009bf075186c25f6fe0a51c31bf79b478de51c809de` |
| `.bklg/init-1/proj-1/story-1/_review.md` | 121 | `05a9fbb46dbcc0271c0a8ad9677694afaec68d672d5ca453c3cc90d08c333d2c` |
| `.bklg/init-1/proj-1/story-1/ledger.md` | 243 | `6cd6363ebd11ea9497ec236caff1a122c36d3b7c22155f499f6aefd6711e4d89` |
| `.bklg/init-1/proj-1/story-1/spec.md` | 320 | `1094339b77bdf7eb50a9c10bbe211e5df5b7b4ac8a8572358e01322adf81e4f8` |
| `.bklg/init-1/proj-1/story-1/story.md` | 297 | `7293972a6b832c25c6de11c8b434d562a0ee54b6ee706b8bb9d5081e8f72c73e` |
| `.bklg/init-1/proj-1/story-2/closure.md` | 170 | `6c64418f70de07ebe4362f3d01c0dbfc7f2f9b8e9fa19573d6c3f1636a049ba6` |
| `.bklg/init-1/proj-1/story-2/design-review.md` | 163 | `ce8368c2e0734a1919137e8c77cc9eb19660e30b26ca702b583779e6bd5fbadb` |
| `.bklg/init-1/proj-1/story-2/design.md` | 158 | `c630691a9f9e6bf9da3789b6fcc7d4d7bf82bf8c70a5f92035e2acbfcd43a86f` |
| `.bklg/init-1/proj-1/story-2/implementation.md` | 149 | `fe37cbf41ae3cd7d2e967004606b0ef5d9e19123debb02b536662e837dc0bcbd` |
| `.bklg/init-1/proj-1/story-2/plan.md` | 129 | `9f89f976edfcf901511a745c44391f495f9beb938c8b15a32ebf2faf442f505f` |
| `.bklg/init-1/proj-1/story-2/report.md` | 499 | `4a2362a2e5b615be91bd093013ec64bc5dad189c3abf62df1104f67ce7c99663` |
| `.bklg/init-1/proj-1/story-2/retrospective.md` | 167 | `4a0ec86f0cf7d891719e4ecc44079a0153eed8c6212021fe0cae292e35232c21` |
| `.bklg/init-1/proj-1/story-2/slices.md` | 145 | `e447d88c8eddf72bacab25650ac6e4fc3c239144c9899433a221d46a8923fe8d` |
| `.bklg/init-1/proj-1/story-2/spec.md` | 556 | `8f336a5ac145df6e8921abe4d0bac225eb95b57743739a602c5e3aaae3211e60` |
| `.bklg/init-1/proj-1/story-2/story.md` | 311 | `ae59c28d02f890029ffc56517d268746e33522f9b244fe59dae8146033b18a8a` |
| `.bklg/init-1/proj-1/story-2/storymap.md` | 149 | `821b63373736b5af2e56ffb82a328aa4d267088fb5e9fcf690c391b15b263871` |
| `.redkiln/config.yaml` | 141 | `27c441a1258d6e15b4629197ed29a3694eb5fa3b71b143dbc75c8e634dad372d` |
| `.redkiln/processes/bug-standard.yaml` | 165 | `e17efc00de50d8e2566c5eda94cd75b0c50e664167716c55cb8c59fd4a3a8228` |

**23 files.**
