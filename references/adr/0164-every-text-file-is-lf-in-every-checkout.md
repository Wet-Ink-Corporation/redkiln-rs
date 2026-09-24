---
id: "0164"
title: "Every text file is **LF in every checkout**, on every box, whatever `core.autocrlf` says: `.gitattributes` opens with `* text=auto eol=lf`, and every `-text` row below it, ADR-0151's pinned run-record bytes included, still wins for its own paths. Committed content does not move, because every unpinned text blob was already LF in the index."
status: proposed
date: 2026-09-24
legacy_id: none
source: .gitattributes (its header rule *"a file whose bytes a test pins is protected from the slice that lands it"*, and the new first row); `git ls-files --eol` over the whole tree, before and after [measured 2026-09-24 at `decc8ae`]; references/adr/0151 (the pinned run-record bytes, and its named residual); spikes/DECISIONS-2026-09-21.md D21-42 decision 14b (the operator's decision this record executes); the memory note *"a local clone is not a byte-faithful export"* (the measured cost of the old state)
supersedes: []
superseded_by: []
related: ["0151", "0092", "0085", "0137"]
evidence: []
consumers: []
---

## Context and problem

`core.autocrlf=true` is set at **system** level on this box (`C:/Program Files/Git/etc/gitconfig`), so every
checkout inherits it without saying so. Nothing in the repository countered it. `.gitattributes` carried only
`-text` rows, each one pinning the bytes of a fixture a test reads, so every *other* text file was rewritten
LF → CRLF whenever git wrote it into a working tree.

Measured with `git ls-files --eol` at `decc8ae`, before this record:

| where | text files LF | CRLF | mixed |
| --- | --- | --- | --- |
| index, unpinned paths | **all** | 0 | 0 |
| main checkout, unpinned paths | 1,199 | **270** | **3** (`.gitignore`, `crates/rk-cli/src/main.rs`, `references/2026-09-08-success-measures.md`) |
| run-owned worktree `rk-run/rk-fix-doctor-test-count` | 277 | **1,480** of 1,757 | |

So **the repository already held LF.** Only the working trees disagreed with it, and how much they disagreed
depended on which files this box happened to have re-checked-out. Three costs of that are on record:

- **No checkout was byte-faithful.** A `--local` clone rewrote 121 of 143 kernel fixture files, and two tests
  failed at pristine HEAD with no mutant applied. A mutation cycle once counted **14** kills where the truth was
  **12**. The error flattered the guard.
- **ADR-0151 pinned one fixture tree and named the rest as a residual**: *"112 would be rewritten by an
  `autocrlf=true` one"*.
- **The verify pass the Step 8 sub-stages build runs the whole suite inside a run-owned worktree** (AC-328), and
  that is exactly the tree that was 84% smudged.

A fourth fact was found while making this change. `.gitattributes` itself was classed **binary** (`i/-text`)
by git's content check, because a comment held one raw carriage-return byte where the text meant the two
characters `\r`. A heredoc had written the escape as the byte.

## Decision drivers

- The index is the repository. A working tree that holds different bytes from the index makes every test a
  measurement of *this box's checkout history*, not of the code.
- ADR-0151's pins must not move. The recorded `run_id` is a digest over mixed bytes, and the fixture's directory
  name is that digest.
- CI runs a `windows-latest` leg, which smudges the same way. D21-42 decision 14a makes CI green on both legs a
  Phase-1 exit condition, so the property has to hold on a runner nobody configures by hand.
- The fix should not depend on anyone's global git configuration, including a future contributor's.

## Considered options

### Option — `* text=auto eol=lf` as the first row (taken)

One row, first in the file. `text=auto` leaves binary content alone, and `eol=lf` overrides `core.autocrlf`
for every text path. Git applies the **last** matching row per attribute, so every `-text` row below it still
governs its own paths. Taken: it makes the repository self-describing, and it moves no committed byte.

### Option — set `core.autocrlf=false` on this box

Rejected. It fixes one machine, and neither the CI runner nor a clone on another box. It also does nothing to
the 1,480 already-smudged files until they are rewritten, and it leaves the property in configuration that
nobody can see from the repository.

### Option — widen the `-text` pinning to every tree a test reads

Rejected. It pins *whatever bytes are there*, including smudged ones, so it would freeze CRLF into history
wherever a box had already smudged. It also answers only for fixtures. Sources, records and scripts would stay
exposed, and they are what the suite's source-text lints read.

### Option — a preflight that refuses a smudged tree before each run

Rejected as the fix, though not as a guard. It detects the problem at run time and leaves it in place. D21-42
decision 14b named it as the alternative the operator declined.

## Decision

1. **`.gitattributes` gains `* text=auto eol=lf` as its first rule**, under a dated comment that states why it
   must stay first. Every existing `-text` row is unchanged and still wins for its own paths.

2. **The index does not move.** `git add --renormalize .` was run with the row in place and staged **only
   `.gitattributes`**, confirming that every unpinned text blob was already LF.

3. **The working trees are brought to the index, not the reverse.** Each of the **273** smudged or mixed files in
   the main checkout was rewritten from its own index blob (`git cat-file blob :<path>`). For every one of them
   the working file's `git hash-object --no-filters` was asserted equal to its index blob id. `git diff` is
   empty afterwards.

4. **The stray byte in `.gitattributes` is replaced with the two characters it meant (`` `\r` ``).** The file is
   then classed as text (`i/lf`) and is covered by its own first row.

5. **ADR-0151 is not superseded.** Its row, its re-staged blobs and its digest all stand. Its forbids bullet
   against *"Reading a `git clone` of this repository as byte-faithful for any fixture without such a row"* is
   **satisfied, not moved**:
   - every path is now covered by a row;
   - a text path checks out as its index bytes;
   - a pinned path checks out as its pinned bytes.

   Its named residual (*"112 would be rewritten by an `autocrlf=true` one"*) is discharged.

6. **The property is verified in a fresh worktree, not only in the checkout it was made in.** A new worktree of
   the commit that carries this row is created under this box's `core.autocrlf=true`, its `git ls-files --eol` is
   asserted to show no `w/crlf` on any path not pinned CRLF, and the workspace suite is run there. The
   measurements are recorded under Evidence.

## Consequences

Every checkout on every box, the CI runners and every run-owned worktree created after this commit, holds the
repository's bytes. The overlay step in the memory recipe (*"check out with `autocrlf=false`, then OVERLAY the
source working-tree bytes"*) is no longer needed for any path, and a plain clone becomes a faithful export.

**Existing working trees are not re-checked-out by git.** A tree written before this row keeps its smudged
bytes until the file is written again. That is why decision 3 rewrote the main checkout explicitly. One tree
is left as it stands: the run-owned worktree at
`.git/redkiln/worktrees/rk-fix-doctor-test-count`. `recreate_run_worktree` removes and re-creates it at the
next run of that item, and that item is the fixture that nobody dispatches (D21-42 decision 18). Its smudged
bytes are named here so that nobody reads them as a counterexample.

A future file that genuinely needs CRLF on disk, such as a `.bat` script, needs its own row. None exists
today. The four `.ps1` spike scripts run unchanged under LF.

**What this forbids**

- Moving the `* text=auto eol=lf` row below any `-text` row, or deleting it while any `-text` row exists.
  Either change silently inverts which rule wins.
- Adding a CRLF-requiring file without its own row.
- Fixing a smudged working tree by re-staging it (`git add --renormalize` over CRLF working bytes with the row
  absent). The repair runs from the index to the tree, never the reverse.
- Citing a test result taken in a working tree whose `git ls-files --eol` shows `w/crlf` on an unpinned path as
  a result about the repository.
- Writing an escape sequence into a tracked file through a shell heredoc. On this box `\\` collapses to `\`,
  which is how `.gitattributes` came to carry the raw byte that decision 4 removes.

## Evidence and confidence

Measured 2026-09-24 in the main checkout at `decc8ae` plus this change:

- **Before:** `i/lf w/lf` 1,199 · `i/lf w/crlf` 270 · `i/lf w/mixed` 3 · `.gitattributes` `i/-text` · pinned:
  `i/lf w/lf attr/-text` 340, `i/crlf w/crlf attr/-text` 9, `i/none` 3.
- **After:** `i/lf w/lf attr/text=auto` **1,473** · pinned rows unchanged (340 / 9 / 3) · `git diff --cached`
  shows `.gitattributes` alone.
- **The suite over the normalised bytes, main checkout** (measured 2026-09-24, before this record's commit):
  `cargo fmt --all --check` rc 0 · `cargo clippy --workspace --all-targets` rc 0 · `cargo test --workspace`
  rc 0, **740 passed, 0 failed, 9 ignored** over 44 `test result:` lines. This is the first run on this box whose
  working bytes equal the index for every unpinned path.
- **Fresh-worktree verification (decision 6)**, measured 2026-09-24. `git worktree add --detach D:/repos/rk-eolv
  ac10470` ran under this box's `core.autocrlf=true`.
  - Census: `i/lf w/lf attr/text=auto` **1,474** (the 1,473 plus this record), with the pinned rows unchanged
    (340 / 9 / 3). `git status` was empty before and after the suite.
  - `cargo fmt --all --check` rc 0 and `cargo clippy --workspace --all-targets` rc 0.
  - `cargo test --workspace --no-fail-fast` rc 0: **740 passed, 0 failed, 9 ignored over 44 `test result:`
    lines**, identical to the main checkout. That is the property this record exists for: a fresh checkout and a
    long-lived one now test the same bytes.
  - The worktree was then removed.
  - An earlier attempt at a deeper path did not check out ADR-0114's 130-character filename, which exceeded
    Windows' 260-character path limit (`w/` empty). That was a path-length artifact, not a line-ending one, and
    the short-path worktree above has no such gap.
- **A finding this verification surfaced, recorded here and not repaired here.** The first fresh-worktree run
  failed **10** tests in `rk-adapter-toolchain-node`, all at `tests/it/support.rs`' preflight: *"run
  `pnpm install --frozen-lockfile --dir …/tests/fixtures/project` ONCE before this suite"*. The fixture's
  `node_modules/` is git-ignored by design (s7c §7c.14). The main checkout had it installed and a fresh tree does
  not, so the suite is green only after that one-time install (pnpm 10.33.0, from the pinned lockfile).
  **`.github/workflows/ci.yml` carries no `pnpm` step at all** (`grep -ci pnpm` → 0). The CI-green exit
  condition (D21-42 decision 14a) therefore owes a pnpm setup and install step in every job that runs the
  workspace suite, and without it CI fails these ten tests on its first run on either OS.

Confidence is high. The mechanism is standard git semantics. The before and after census agree with it. And the
suite is identical in the long-lived checkout and a fresh one.
