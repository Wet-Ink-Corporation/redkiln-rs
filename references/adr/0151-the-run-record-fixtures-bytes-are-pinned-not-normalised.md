---
id: "0151"
title: "The committed run-record fixture's bytes are **PINNED as they are** by a `.gitattributes -text` row, not normalised to LF — because the recorded `run_id` is a digest **over those bytes** and **the fixture's DIRECTORY NAME IS that digest**. `config::adr0083_…` recomputes `sha256_lp([SCHEMA, stage, canonical(inputs)])` and `replay::read_run` applies no newline normalisation, so every `\\r` under the fixture is inside the hash preimage. Measured 2026-09-22: **SIX** preimage files were smudged (worktree CRLF, index blob LF) — the five `inputs/catalog/*.toml` **and `inputs/pack/incident-deviation.toml`, which the finding did not name** — while `inputs/facets.toml` (13,031 B) and the rest were LF in both, so an `autocrlf=true` clone moved the digest one way and an `autocrlf=false` clone moved it the other and **neither export was faithful**. Normalising would move the digest and force a rename of a directory named in **accepted ADR-0130**, in two CLOSED stages' prose, in CI and in kernel source; pinning costs no record a correction"
status: accepted
date: 2026-09-22
legacy_id: none
source: .gitattributes (its own header rule — *"a file whose bytes a test pins is protected from the slice that lands it"*); crates/rk-kernel/tests/it/config.rs (`RECORDED_RUN`, and the two digest comparisons); crates/rk-kernel/src/replay.rs (`read_run`, which reads inputs with no normalisation; `RunInputs::to_json`, the preimage; `run_id`); references/adr/0092 (decision 1 — an accepted record is never edited); references/adr/0130 (names the fixture directory)
supersedes: []
superseded_by: []
related: ["0083", "0092", "0130"]
evidence: []
consumers: []
---

## Context and problem

`config::adr0083_config_type_declared_in_kernel_and_threaded_into_run_inputs` recomputes the
committed run record's `run_id` and compares it **twice** — against what `run.json` records, and
against the hardcoded `RECORDED_RUN`. The digest is
`sha256_lp([SCHEMA, stage, canonical(inputs)])`, and `RunInputs::to_json`'s preimage carries the
verbatim `src` of the pack, every catalog file, the artifacts file, the facets file, the context,
the artifact state, the deviation, the runtime and the specs. `replay::read_run` reads each with
`String::from_utf8` and **no newline normalisation**, so every `\r` is inside the hash.

`.gitattributes` carried no row for `crates/rk-kernel/tests/fixtures/logs/**`, and
`git check-attr text` reported `unspecified` for every file under it. `core.autocrlf=true` is set at
**system scope only**, in Git for Windows' own `etc/gitconfig` — nothing in the repository records
the setting the test's green depended on.

**Measured 2026-09-22, and the finding understated it.** Six preimage files were smudged — worktree
CRLF, index blob LF:

| file | worktree | blob |
| --- | --- | --- |
| `inputs/catalog/change-summary-ue.toml` | 997 CRLF | 971 LF |
| `inputs/catalog/change-summary.toml` | 1115 CRLF | 1086 LF |
| `inputs/catalog/changeset.toml` | 976 CRLF | 949 LF |
| `inputs/catalog/impl-report.toml` | 1135 CRLF | 1107 LF |
| `inputs/catalog/spec-authoring.toml` | 623 CRLF | 600 LF |
| **`inputs/pack/incident-deviation.toml`** | **1252 CRLF** | **1226 LF** |

`PROGRESS.md` row 37's finding (23) named only the five catalog files. The sixth is the **pack**
source, and `pack.src` is in the preimage.

Meanwhile `inputs/facets.toml` (13,031 B), `inputs/artifacts/artifacts.toml` and the four `.json`
inputs were **LF in both**. That mixture is why *neither* export direction was faithful: an
`autocrlf=true` clone smudges `facets.toml` to CRLF and moves the digest; an `autocrlf=false`
checkout un-smudges the six and moves it the other way. **22 of 143** kernel fixture files were CRLF
in the working tree, so a fresh clone rewrote the other **121** — the figure Checkpoint 7g's guard
(7) comment already records from its own mutation cycle.

## Decision drivers

- **The digest and the directory name are the same string.** `RECORDED_RUN` is
  `"implement-bf1d8cec3b22d288"`, the test joins the fixture path from it, and the record's docstring
  says so: *"whose directory name IS its `run_id`"*. Any change to the preimage renames the fixture.
- **That name is load-bearing outside this crate.** It appears in **accepted ADR-0130**, in
  `s7b-kernel.md` and `s7c-adapters.md` (two CLOSED stages), in `.github/workflows/ci.yml`, in
  `crates/rk-kernel/src/replay.rs`, and in two dated spike records. ADR-0092 decision 1: **an
  accepted record is never edited.** Normalising would force exactly that edit, or force those
  records into falsehood.
- **Pinning gets the whole property for none of that cost.** The problem was never *which* bytes;
  it was that the bytes were not determined by the repository. A `-text` row makes the index hold
  the tested bytes verbatim and every checkout reproduce them.
- **The precedent is already in the file, four times over.** `snapshots/*.snap`,
  `init/config.toml.expected`, `doctor/**` and `legacy-mini/**` are pinned exactly this way, and
  `.gitattributes`' own header states the rule this fixture was the standing counterexample to.

## Considered options

**(a) Leave it.** Rejected: the test is green on this box only, and red at pristine HEAD on a Linux
CI or an `autocrlf=false` checkout. `.github/workflows/ci.yml` runs `cargo test --workspace` on
`ubuntu-latest` as well as `windows-latest`.

**(b) Normalise the fixture to LF, re-bless the digest, rename the directory.** Rejected. The
directory name *is* the digest, and it is named in **accepted ADR-0130**, in two CLOSED stages'
prose, in CI, in kernel source and in dated spike records. ADR-0092 decision 1 forbids editing an
accepted record; the alternative is leaving those records false.

**(c) Re-bless the digest to whatever the current checkout produces.** Rejected as circular: it
makes the *next* checkout the problem, and the digest stops being evidence about anything.

**(d) Add a `-text` row for `inputs/catalog/` only.** Rejected once the preimage was read: `pack.src`,
`facets`, `artifacts`, `context`, `artifact_state`, `runtime` and `specs` are all in it, and
`inputs/pack/incident-deviation.toml` is one of the smudged six.

**(e) Pin the whole fixture tree's bytes as they are.** Taken.

## Decision

1. **`crates/rk-kernel/tests/fixtures/logs/** -text`** is added, covering the whole run-record
   fixture tree rather than only `inputs/catalog/`, because `pack.src`, `facets`, `artifacts`,
   `context`, `artifact_state`, `runtime` and `specs` are all in the preimage.

2. **The index is re-staged from the working tree** (`git add --renormalize`), so the six smudged
   files are stored with the CRLF bytes the digest was blessed over. Index and worktree are now
   byte-identical for all **15** files.

3. **The digest is NOT re-blessed and the directory is NOT renamed.** `RECORDED_RUN` and
   `run.json`'s `run_id` are unchanged, and no record that names the directory needs a correction.

4. **The property is verified by construction, not asserted.** The fixture is checked out under
   `core.autocrlf=true` and under `core.autocrlf=false` and the two are compared to each other and
   to the working tree: **15 of 15 identical** under all three.

## Consequences

`config::adr0083_…` becomes platform-independent for the first time: it now passes on a Linux CI and
on an `autocrlf=false` Windows checkout, where it would previously have failed at pristine HEAD with
no mutant applied. That failure mode is the one Checkpoint 7g's guard (7) comment records paying
for — *"a mutation cycle whose export is not byte-faithful reports kills it did not make"* — and it
ran in the direction that flatters the guard.

The remaining `rk-kernel` fixtures are **not** covered by this row's reasoning and are not pinned
here: only the run-record tree has bytes a digest commits to. A later fixture that pins bytes gets
its own row with the slice that lands it, which is `.gitattributes`' standing rule.

**The residual is named rather than implied.** Under `crates/rk-kernel/tests/fixtures/` **16** files
outside `logs/` still differ from an `autocrlf=false` checkout, and **112** would be rewritten by an
`autocrlf=true` one. No other test digests bytes under that tree — `adr0083` is the only one — so
this is a latent exposure and not a live defect. But `.gitattributes`' header states its rule
generally, and this row satisfies it for exactly one test.

**What this forbids**

- Normalising this fixture's line endings while `RECORDED_RUN` names the directory.
- Adding a `-text` row for part of the preimage only.
- Re-blessing the digest to match a checkout, which would make the next checkout the problem.
- Reading a `git clone` of this repository as byte-faithful for any fixture without such a row.
- Committing the re-staged blobs without the `.gitattributes` row in the SAME commit — CRLF blobs in
  history with no attribute over them is worse than either end state.

## Evidence and confidence

**Verified by construction, not asserted.** The index was checked out twice —
`git -c core.autocrlf=true checkout-index -a --prefix=…` and the same with `false` — and the two
trees compared to each other and to the working tree. All **15** files under
`crates/rk-kernel/tests/fixtures/logs/` are byte-identical across all three. For scale, the same
comparison over the whole index yields 1458 differing files, so the subtree result is not an
artefact of the method.

**Per-file, measured terminator-agnostically by byte count** (`wc -c` against `tr -d '\r' | wc -c`),
because `grep -c $'\r'` returns 0 for CRLF and LF alike on this box: the six smudged files are
997/1115/976/1135/623/1252 in the worktree against 971/1086/949/1107/600/1226 in the blob — exactly
one byte per line, the smudge filter's work — and `facets.toml` (13,031 B) carries zero CR. **22 of
143** tracked kernel fixture files carry CR, so a fresh clone rewrites the other **121**.

**The digest and the name did not move**: `config::adr0083_…` selected by name passes
(`1 passed; 0 failed; 223 filtered out`), and `git show HEAD:<f> | tr -d '\r'` equals the worktree
file with `\r` removed for all six, so only terminators changed and no content did.

**Confidence.** High, and the property is now a function of the repository rather than of the box:
before this row, nothing committed recorded the `core.autocrlf` setting the test's green depended on,
and that setting lives in Git for Windows' own installer default at **system** scope.
