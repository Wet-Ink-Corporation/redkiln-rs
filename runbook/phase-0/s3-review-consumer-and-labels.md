# Step 3 — The review-consumer check, and the three blind label files (Phase 0, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s3` | Phase 0 | Checkpoint 1 | Checkpoints 3a, 3b, 3c | `s1` | `s4` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Two independent pieces of work, each with its own checkpoint. Neither costs more than one dispatch.

### 3a — The one-dispatch check on the review consumer (D-FIXCONSUMER)

The defect is real and verifiable right now:

```bash
grep -n "review\.pass" D:/repos/redkiln/workflows/forge-fix.js
#  982:if (review && review.pass === false) {          <-- THE DEFECT
# 1019:if (!review || review.pass !== true) {           <-- already correct; do NOT touch it
# 1029, 1148, 1184, 1270, 1273: reads and renderings, not gates
```

**Read that second line before you do anything else.** `forge-fix.js` already contains one correctly-guarded site
at line 1019, in the exact form the fix below writes. That is why the previous revision's Checkpoint 3a
(`grep -c "review.pass !== true" → 1`) was **inverted**: it passed *before* the fix, on line 1019 alone, and would
have failed *after* the fix, at 2. A checkpoint that is green when you have done nothing is worse than none. The
counts below are the corrected ones, and both are given so you can tell the two states apart.

Line 982 reads an **ABSENT** verdict as approval. F20 is the measured mechanism that produces exactly that, silently:
an agent card's `tools` allowlist disables `--json-schema`, `structured_output` is absent, and the process exits
**0** with `subtype: success`. Whether the Workflow runtime shares the defect is **UNVERIFIED and is one dispatch
to settle**.

**Fix the consumer either way** — the fix does not depend on the answer:

```bash
F=D:/repos/redkiln/workflows/forge-fix.js

# Anchor assertion first (golden constraint 7): a silent no-op replace reads as a behaviour change.
# The anchor must be present EXACTLY ONCE, or the replace is not the edit you think it is.
n=$(grep -c 'if (review && review\.pass === false) {' "$F")
before=$(grep -c 'review\.pass !== true' "$F")          # → 1 (line 1019, pre-existing)
if [ "$n" -ne 1 ]; then
  echo "ANCHOR COUNT $n (expected 1) — DO NOT WRITE"
else
  sed -i 's/if (review && review\.pass === false) {/if (!review || review.pass !== true) {/' "$F"
  after=$(grep -c 'review\.pass !== true' "$F")         # → 2 (982 and 1019)
  echo "before=$before after=$after  (expected before=1 after=2)"
  grep -n 'review\.pass !== true' "$F"                  # → must print BOTH 982 and 1019
fi
```

Note the shape: no `exit 1`. The previous revision's `|| { echo …; exit 1; }` would have **closed the operator's
interactive shell** on a missed anchor, and would also have fired on a *sed* failure while blaming the anchor. An
`if` says which thing failed and leaves you in the shell to look at it.

The `sed` pattern does not escape the `&&`: inside single quotes the shell passes it through, and `&` is not
special in a sed *pattern* (only in the *replacement*, which contains none).

> **The corpus pin is not disturbed by this.** `D:/repos/redkiln` is frozen at `47faec9f…`, and every corpus read
> resolves at that SHA through the object store, never through the working tree (`spikes/corpus/PINNED_SHA` says so
> in terms; AC-464 makes it a refusal). A working-tree edit to `workflows/forge-fix.js` changes what the *old
> redkiln runs*; it changes nothing any measurement reads. Do not commit it into the frozen history.

The dispatch itself — does the **Workflow runtime** reproduce the absent-verdict path, as the CLI path does?
**`[WILL EXIST]`** There is no rig subcommand for this; `runner/src/main.rs` carries `doctor`, `probe`,
`ttl-probe`, `ttl-mark`, `ttl-read`, `corpus-manifest`, `gate-probe`, `reconcile-replay`, `review-probe`,
`digest-probe`, `human-gate`, `classify-probe`, `graph-probe`, `repro-probe` and `rehydrate`, and none of them
poses this question. The check to write:

```bash
# [WILL EXIST] — one haiku-scale dispatch through the Workflow runtime with a tool-restricted
# card that omits StructuredOutput, asserting on the runtime's own return value.
cd D:/repos/redkiln-rs/spikes/runner && cargo build     # golden constraint 3: test/clippy do not rebuild
./target/debug/redkiln-spike-runner fix-consumer-probe \
    --runtime workflow \
    --card-omits StructuredOutput \
    --timeout-secs 3600 \
    --out D:/repos/redkiln-rs/spikes/results/d-fixconsumer.jsonl
# PASS  = the runtime surfaces a typed absence (a StepError, or an explicit null the caller must handle)
# FAIL  = the runtime hands back a value indistinguishable from an approval — the CLI-path defect is shared
```

Four constraints land on this one command and each changed a character of it:

- **GC3** — the `cargo build` is not decoration; `cargo test`/`cargo clippy` do not rebuild the binary.
- **GC4** — validate this one item before any set, and prefer **haiku**: when the question is "does this flag work
  at all", a haiku probe answers it and an Opus one only costs more.
- **GC8** — stamp the instrument version into the record. Today that is **2.1.266**, a third version and the second
  lineage break in the P-series; this record may not be pooled with the 2.1.263/2.1.265 sets.
- **GC6** — the `--timeout-secs 3600` is deliberate and is **not** the fast-condition default. The 900 s ceiling
  fitted S-SILENT and cost a CLEAN record; a Workflow-runtime round trip is the slow shape here. Size against the
  slow one.
- **GC2 / GC5** — run it in the **foreground**. If you must wait on it, the wait is
  `grep -qE "MARKER|exited with code"`, never `grep -q "MARKER"`: one marker-only waiter spun 25 minutes on a job
  that had already failed. `issue-81` was killed three times as a background job and finished in 116 s in the
  foreground.

The `--out` path is absolute on purpose. The previous revision wrote `--out results/d-fixconsumer.jsonl` with no
`cd` anywhere in the block — the one silent working-directory dependency in the file.

**Checkpoint 3a.** Both:

```bash
grep -c 'review\.pass !== true'  D:/repos/redkiln/workflows/forge-fix.js   # → 2   (982 fixed + 1019 pre-existing)
grep -c 'review\.pass === false' D:/repos/redkiln/workflows/forge-fix.js   # → 0   (the defect is gone)
```

Two lines, not one, and the second is the one that actually discriminates: `=== false` reaching zero is only
possible if line 982 changed. Before the fix these read **1** and **1**.

and either a `d-fixconsumer.jsonl` record with a PASS/FAIL verdict, **or** a written line in
`spikes/DECISIONS-2026-09-09.md` recording that the dispatch was deferred and that the runtime's behaviour is
therefore still UNVERIFIED. Silence is not a checkpoint. The **principle** is already generalised into Phase 1
(AC-282, AC-439, design §9.6's `spoke()` row: absence is a block, never a pass) — but generalising it does not discharge
the specific check, and this runbook does not pretend it does.

### 3b — Generate and hand over the three blind label files

All three exist and are already generated. Verify, do not regenerate:

```bash
cd D:/repos/redkiln-rs
grep -cE '^\| .+ \| (toolchain|change|binary_change|auth|data_migration|ui|flag_exists|recent_deploy) \|' \
     spikes/results/p1-arm-d-labels-RYAN-TODO.md    # → 160  (20 items x 8 facets)
grep -cE '^\| (_archive|support)' spikes/results/p1-arm-l-labels-RYAN-TODO.md
                                                    # →  20  (one per item: kind + 4 ordinal + 3 boolean)
grep -cE '^\| P[0-9]{2} \|'       spikes/results/p2-labels-RYAN-TODO.md
                                                    # →  41  (12 perturbation cases x their downstream sets)
```

`spikes/results/p7-deviation-labels-RYAN-TODO.md` carries **no data rows** — its only table row is the placeholder
"no deviation requests recorded yet". The channel was offered **71 times and used 0**. There is nothing to
adjudicate; do not wait on it. Note it is also the one file where METHOD-P §0 rule 1 does **not** apply and there
is **no timestamp guard** — it labels the *request*, which did not exist before the run, and that is disclosed in
the file itself. Do not generalise its looseness to the other three.

**Two of the three filled names already exist, and `cp` over them is destructive. Guard, do not copy.** Arm D's
`…-labels-ryan.md` is the **68 KB worksheet** built by `spikes/p1/build-arm-d-worksheet.py` and committed at
`20775ce`; P2's is the **130 KB worksheet** built by `spikes/p2/build-p2-worksheet.py` on 2026-09-09, carrying all
twelve upstream diffs inline. Copying the `-RYAN-TODO` blanks over either replaces a worksheet with a bare row
list — which is the exact defect the worksheets were built to fix, applied in reverse. **P2's copy is now worse
than that: it carries the 41 filled labels of 2026-09-10 and the engine has already been run against them, so
`cp` over it destroys a measurement, not a template** [measured, F40]. The loop below echoes bytes for exactly
this reason — filled, it is **135,567** bytes and arm D's unfilled worksheet is **69,842** (measured 2026-09-10),
so a size in the low tens of thousands after a `cp` is the blank staring back at you. Blindness is destroyed by
reading and a label set is destroyed by writing; neither comes back:

```bash
cd D:/repos/redkiln-rs
for f in spikes/results/p1-arm-d-labels-ryan.md \
         spikes/results/p1-arm-l-labels-ryan.md \
         spikes/results/p2-labels-ryan.md; do
  if [ -f "$f" ]; then echo "EXISTS, do not overwrite: $f ($(wc -c <"$f") bytes)"
  else echo "MISSING — copy its blank: $f"; fi; done
# Only for a file the loop reports MISSING:
#   cp spikes/results/p1-arm-l-labels-RYAN-TODO.md spikes/results/p1-arm-l-labels-ryan.md
```

The `-RYAN-TODO` file stays as the blank of record in every case, and the scorers read the filled copy.

**Arm D's half of this step is already discharged and its route has changed.** METHOD-P **amendment 9**
(2026-09-09) replaced arm D's human-panel oracle with a deterministic oracle on the mechanical facets, an
adversarially-verified model panel on the semantic ones, and Ryan adjudicating every contested cell — so arm D was
scored on 2026-09-09 without `p1-arm-d-labels-ryan.md` ever being filled [measured, F39; §3c row (d) records the
same]. That file is retained **unfilled**, as the worksheet of record and as the artefact amendment 9 departed
from; it is **not** an outstanding deliverable and it must not be counted as one.

**P2's half is now discharged too, and it is the one arm whose blindness is CHECKED rather than attested.**
`spikes/results/p2-labels-ryan.md` was filled on 2026-09-10 — **41 of 41 rows answered**, `labeller: Ryan Britton`,
`labelled_at: 2026-09-10T15:58:08Z` — and the engine has since been run against it, its earliest record stamped
**2026-09-10T16:01:12Z** [measured, F40]. **P2 is unaffected by METHOD-P amendment 8**, so its chronology proxy has
not expired: the label stamp precedes the first engine record, the scorer's own guard checks that ordering on every
run, and **P2 is therefore *timestamp-verified*, not *attested-blind*** — the only arm in this programme of which
that is true. **Do not blur it with arm D**, which is panel-adjudicated and whose amendment-8 attestation route was
registered and never invoked (§3c row (d)). Do not re-fill, re-stamp or re-copy this file; s4 §4.2 and s4 §4.3 are where
it is read.

**What remains outstanding in this step is `arm L` alone**, and the discipline below is written for it. **Do not
read P2's discharge as licence to relax any of it** — the rules are METHOD-P §0 rule 1 and they are what made P2's
result checkable.

**Arm L is outstanding and is not on Step 5's path (DECISIONS-2026-09-11 #6; PIA-018).** No scorer consumes the
arm-L file yet (`spikes/p1/score-arm-l.py` does not exist), and after Step 2 the results live in
`redkiln-corpus/results/`. So: label it when a scorer exists, in the corpus repo, and commit it there at a new pin;
Checkpoint 2 accepts it as outstanding, and Checkpoint 3b's arm-L lines below are recorded as **deferred to the
corpus repo**, not failed. If you do fill it before Step 2, commit it here first — `git add
spikes/results/p1-arm-l-labels-ryan.md && git commit -m "P1 arm L: blind labels, labelled_at <stamp from the file>,
instrument <version from the file>"` — so redkiln-rs history holds a copy before the split moves the directory
(PIA-021).

**The rules for filling them, which are METHOD-P §0 rule 1 and are not negotiable:**

- Fill **from the item alone**, before opening any engine output. `spikes/results/p1-arm-d.jsonl` (640 records),
  `p1-arm-l.jsonl`, `p1-arm-l-v2.jsonl` and `p2-cases.jsonl` are all unblinding material. Do not open them.
- For P2 this was done on 2026-09-10 and the rule it was done under is recorded here because the **advisory half
  still has to obey it**: label **from the diff alone**,
  `git -C D:/repos/redkiln diff <replaced_with_rev> 47faec9f -- .bklg/<item>/<upstream>`. **P2's perturbation half
  is scored; its advisory half is not.** The advisory cases **A01–A06** are labelled *after* the engine, in a
  separate file (METHOD-P §2.3), they are **not** part of this step's three blind files, and **DECISIONS-2026-09-09
  #15's `human_authored` remap must be settled before they are labelled** [documented, METHOD-P §2.3; s4 §4.2's
  precondition table]. "P2 is entirely unrun" is false; **"P2 is complete" is also false** — every sentence about
  P2 has to say which half it means.
- Leave `?` where the item alone cannot tell. `recent_deploy` is underivable at this pin (no deploy log) — label
  it `?` unless you know otherwise. `binary_change` is measured but **is not in `facets.toml`**, so it can feed no
  `require` row regardless of how you label it (DECISIONS-2026-09-09 #11; verified — `facets.toml` has no
  `[facet.binary_change]`).

**Guard scope, before you trust the row count (golden constraint 9).** The label file has **eight** facet columns;
`score-arm-d.py`'s parser regex admits **seven**:

```bash
grep -n 'toolchain|change|binary_change' D:/repos/redkiln-rs/spikes/p1/score-arm-d.py
# (toolchain|change|binary_change|auth|data_migration|ui|flag_exists)   <-- recent_deploy is NOT here
```

So a `recent_deploy` label is **read by nothing** — it is not scored, and its absence is not reported. That is
harmless *given* the instruction above (label it `?`, which the scorer would skip anyway), and it is stated here
so that a filled `recent_deploy` column is never mistaken for a scored one. Do not widen the regex to "fix" it: a
facet with no feed in the frozen corpus has no ground truth to score against, and widening the guard is how the
programme's two mis-scoped guards happened.
- For arm L, copy the ordinal anchors out of `spikes/prompts/p1-classify.md` **verbatim** before filling. A panel
  scored against remembered anchors is scored against different anchors.

### The UTC `...Z` timestamp trap — read this before you stamp anything

`labelled_at` is compared to the engine's earliest record **lexicographically, as a string**. The engine writes its
`at` field as a naive UTC ISO-8601 with a literal `Z` and microsecond precision (`2026-09-09T06:06:31.482913Z`).
`score-arm-d.py` then does, literally, `if ts >= first: raise SystemExit(...)`.

**A stamp carrying a local offset instead of `Z` mis-compares, and one direction of the mis-compare is SILENT:**

- A **negative** offset (`2026-09-08T23:00:00-05:00`, which is `2026-09-09T04:00Z`) sorts *before* an engine
  timestamp it actually follows. The guard passes. **A post-dated label is accepted as blind, silently, and the
  whole blind-panel discipline is void with nothing printed.**
- A **positive** offset (`2026-09-09T13:00:00+10:00`, which is `2026-09-09T03:00Z`) sorts *after* an engine
  timestamp it actually precedes. The guard refuses a valid label, loudly.

Only the loud one tells you. Produce the stamp with a command, never by hand:

```bash
date -u +%Y-%m-%dT%H:%M:%SZ                                              # → 2026-09-09T06:06:31Z
python -c "import datetime;print(datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'))"
```

```powershell
[DateTime]::UtcNow.ToString("yyyy-MM-ddTHH:mm:ss") + "Z"                 # → 2026-09-09T06:06:34Z
```

Paste that into `labelled_at:` and set `labeller:`. Verify before moving on:

```bash
grep -h "^labelled_at:" spikes/results/p1-arm-l-labels-ryan.md \
                        spikes/results/p2-labels-ryan.md
# every line must end in Z, contain no '+' and no '-' after the 'T', and contain no "FILL"
# P2's already does: `labelled_at: 2026-09-10T15:58:08Z` [measured, F40]. Arm D's file is NOT in this
# list and must not be added — it is retained deliberately unfilled at `labelled_at: FILL-ME`
# (amendment 9), so including it makes this check fail on a file nobody is meant to stamp.
```

**Checkpoint 3b, re-scoped: `arm L` is the outstanding half; P2's clauses are DISCHARGED and are kept below as
regression checks.** **The first line is an existence guard and is not optional** — the previous revision
opened with `! grep -l "FILL-ME" …`, and a `grep` over files that do not exist exits **2**, which `!` turns into a
pass. That checkpoint went green when Step 3b had not been done at all.

**The outstanding half — `arm L`. All three, and none of them passes today — and by DECISIONS-2026-09-11 #6 none of them has to before Step 2 (record them as deferred to the corpus repo in `PROGRESS.md`):**

```bash
cd D:/repos/redkiln-rs
test -f spikes/results/p1-arm-l-labels-ryan.md || echo "MISSING: p1-arm-l-labels-ryan.md"
                                                    # → no output. Measured 2026-09-10: it PRINTS — the file
                                                    #   does not exist and arm L is genuinely outstanding.
grep -l "FILL" spikes/results/p1-arm-l-labels-ryan.md | wc -l                                     # → 0
grep -h "^labelled_at:" spikes/results/p1-arm-l-labels-ryan.md | grep -cE 'Z$'                    # → 1
```

**The discharged half — P2. All four hold on this tree; run them as a regression check, not as work to do:**

```bash
cd D:/repos/redkiln-rs
test -f spikes/results/p2-labels-ryan.md || echo "MISSING: p2-labels-ryan.md"                     # → no output
grep -l "FILL" spikes/results/p2-labels-ryan.md | wc -l                                           # → 0
grep -h "^labelled_at:" spikes/results/p2-labels-ryan.md | grep -cE 'Z$'                          # → 1
grep -cE '^\| P[0-9]{2} \|.*\| *(yes|no) *\|' spikes/results/p2-labels-ryan.md                     # → 41 of 41 rows answered
grep -h "^labell" spikes/results/p2-labels-ryan.md
# → labelled_at: 2026-09-10T15:58:08Z   /   labeller: Ryan Britton   [measured, F40]
```

**Three files named, one outstanding, and the counts are per-file for a reason.** `p1-arm-d-labels-ryan.md` is
deliberately excluded from both blocks: amendment 9 replaced its route, arm D is scored, and the file is retained
unfilled (above). Including it makes the check unpassable by construction — `grep -l "FILL" … | wc -l` can never
reach 0 while it carries `labelled_at: FILL-ME` — which is a guard scoped wrong, and golden constraint 9 says to
establish a guard's scope before obeying it *and* before routing around it. Do not "fix" this by filling that
file: its blindness proxy expired (amendment 8) and the panel superseded it (amendment 9). **The two halves are
split into two blocks, rather than left as one `→ 2` count, because a combined count green on P2 alone would read
as arm L being done** — the same shape as the `!`-negation defect this checkpoint already carries a paragraph
about.

**What P2's discharge is, stated exactly.** Its labels are **blind and timestamp-verified**, not attested-blind:
`labelled_at 2026-09-10T15:58:08Z` precedes the engine's earliest record at **2026-09-10T16:01:12Z**, and P2 is
unaffected by amendment 8 so that chronology proxy still holds [measured, F40]. **It is the only arm in this
programme whose blindness is checked rather than attested.** It does **not** mean P2 is complete: the **advisory
half (A01–A06) is unrun**, is labelled after the engine in its own file, and waits on DECISIONS #15.

`grep -l "FILL"`, not `"FILL-ME"`, and piped to `wc -l` rather than negated: `score-arm-d.py` tests for the
substring `FILL-ME` but `reconstruct-derived-from.py` tests for `FILL`, so the looser string is the one that
matches both guards. **`spikes/p2/score-p2.py` (s4 §4.2) tests for `FILL` too**, and refuses with *"labels are not
timestamped — refusing to score"* [measured, the script at this pin], so the same looser string covers all three.
The blank row counts these are filled from — **20 / 41** for arm L and P2, and **160** for
the arm-D file this checkpoint no longer covers — were re-verified against the
`-RYAN-TODO` originals on 2026-09-08 and are correct; P2's **41 of 41** are now answered.

Empty cells and `?` cells are legitimate answers and are counted as unlabelled by the scorer, not as errors. What
is not legitimate is a file where `labelled_at` is set and the rows are untouched.

### 3c — Phase-0 exit (c) and (d): what this runbook cannot deliver, said plainly

Design §10.2 gives Phase 0 **four** exit conditions, and Phase 1's "Enter when" is now "(a), (b), (d) and the
non-S-BIN half of (c)" (ADR-0093 decision 4; PIA-001 found the Enter column and the May-not column contradicting
each other, and Step 3c checking (c) against S-BIN alone while two register rows gate Phase 1). Steps 1 and 2
discharge (a) and (b). **The arm-D half of (d) now has a result, and the previous revision of this table did not
know it.** Exit (c) has **two** rows marked "gates Phase 1" — **S-BIN** and **S-FACET** — and each has a written
disposition below: S-BIN's gate is scoped by ADR-0093 to the `ChangeSummary` type and the Toolchain port's
`change_summary` body, and S-FACET's gate is scoped to the pack rows keyed on scored derived facets; neither blocks
Phase-1 entry. The S-BIN half of (c) remains a genuine shortfall for `ChangeSummary`, and it has a live route that is
*not* the re-scope. **(d) also carries a caveat the table states rather than glosses: design §10.2's wording is "the *blind*
panels are filled", and neither half is blind on those terms** — arm D is **panel-adjudicated, not human-panel** (amendment 9), and amendment 8's
attested-blind route was **registered but never invoked** — `blind_attestation:` in
`spikes/results/p1-arm-d-labels-ryan.md` stands `FILL-ME` and `score-arm-d-panel.py` carries no
`--attested-blind` flag — so the label may not be applied to these figures either; the E3 sheet is not
blind at all and κ is forfeited. Re-verified
2026-09-09:

| Exit | What §10.2 requires | State | What you must do |
| --- | --- | --- | --- |
| **(c)** | Every §10.5 register row marked "gates Phase 1" has a written finding with an F-number in `spikes/FINDINGS.md`, **or the row is explicitly re-scoped by an ADR** | **S-BIN is "Registered, unrun"** and is marked **YES, gates Phase 1**. It needs a real Unreal project, ~10 Opus dispatches and UE automation runs [documented, design §10.5]. **The fixture now exists**: Ryan has chosen to design S-BIN against the real Unreal project at `D:/repos/BrittonGames` — UE 5.8, headless automation already working via `_workspace/tooling/verify_headless.ps1`, assets in Git LFS per its own `.gitattributes` [measured, paths verified 2026-09-09]. **S-BIN has still not run**, and it still gates Phase 1 for `ChangeSummary` | **Design the spike against the real project.** ADR-0008 already names a real Unreal project as the S-BIN fixture, and that fixture is now identified rather than hypothetical — so exit (c) has a route that ends in a *finding*, not in a narrowing. Cost and schedule are still outside this runbook. **The scope is now recorded, not pending**: ADR-0093 decision 4 scopes S-BIN's gate to the `ChangeSummary` type and the Toolchain port's `change_summary` body, exactly as ADR-0008's "(type design)" words it — Step 5 still refuses to build the type (**AC-475**, no synthesised value; **AC-497**/**AC-356** stay PROVISIONAL) and nothing else in Phase 1 waits on it. Running S-BIN against BrittonGames is still the route to the *finding*; it is no longer a route Phase-1 entry waits on |
| **(c)**, second row | The same §10.5 clause, for **S-FACET** | **S-FACET is "Registered, unrun"** and was marked **gates Phase 1** at design :1789/:1861 with no F-number and no re-scope — the row Step 3c omitted (PIA-001). Its criterion as registered could not decide itself on this corpus (`auth` 0 positives → NOT ANSWERABLE; `ui` 9/9 → Wilson lower bound 0.70, POWERLESS for ≥ 0.95; no `surfaces` label column) [measured, PIA-084] | **Take ADR-0093's branch.** The facets S-FACET can score are the six that stay `derived`; four kernel projections and the declared `surfaces` map leave AC-520's load-time gate (ADR-0093 decisions 1–3), so S-FACET gates only the pack rows keyed on `ui`, `flag_exists`, `data_migration` and `surfaces` — Step 7e's row dispositions — not Phase-1 entry. Its kill is re-registered by dated pre-dispatch amendment under `references/2026-09-09-change-classification.md` §6 (DECISIONS-2026-09-11 #12: arm 3 vs arm 1 paired false-positive count; the ≥ 0.95 bar stays recorded as powerless). A second $0 shortfall run in Phase 0 is the other branch and is not taken |
| **(d)** | The blind panels are filled: **P1 arm D scored**, and `e3-severity-annotation-BLANK.md` **returned** | **Both halves have returned a result. Only one of them is blind in any sense, and neither is blind on the terms design §10.2's wording assumes.** P1 arm D was **scored on 2026-09-09** against 140 truth cells at corpus pin `47faec9f` (the 2026-09-09 run scored 136 of them; the 2026-09-11 key fix scores all 140, F39-DRAFT's Correction), at **$0** model cost [measured, F39] — and it is **panel-adjudicated, never human-panel** (amendment 9), and must be reported that way wherever the number is reused. **Amendment 8's attested-blind route does not attach to these figures.** That route requires BOTH a filled `blind_attestation:` line in the label file AND an explicit `--attested-blind` scorer invocation, "so the substitution can never happen silently"; neither happened — the line reads `FILL-ME` and `score-arm-d-panel.py` takes no arguments at all [measured, both files at this pin]. Blindness here rests on amendment 9's own construction (no panel seat may read `p1-arm-d.jsonl`; Ryan adjudicated every contested cell with the panel's and the script's positions revealed only after he answered), which amendment 9 itself records as **weaker** than the pre-registered oracle [documented, METHOD-P §0 amendments 8 and 9]. E3-SEV is recorded **DONE** by the design: `e3-severity-annotation-ryan.md` is filled — **4 BLOCKING, 2 ADVISORY**, including a documented self-correction on finding 4 (originally annotated BLOCKING on a claim that was false) — settling arm (a) FP 1/23, MCC +0.978 and arm (b) FP 0/23, MCC +1.000 [documented, design §10.4, §10.5]. **That sheet is NOT BLIND at all, by its own header**, so the E3 half discharges (d)'s *result*, not (d)'s *blindness*. **Step 3b's own label files have since split three ways, and P2 is the counter-example to this row's own headline:** P2's 41 labels were filled 2026-09-10 (`labeller` Ryan Britton, `labelled_at 2026-09-10T15:58:08Z`), the engine ran at `16:01:12Z`, and because P2 is unaffected by amendment 8 its chronology proxy still holds — **P2 is blind and TIMESTAMP-VERIFIED, the only arm in the programme of which that is true** [measured, F40]. It is not part of exit (d), which names arm D and the E3 sheet only, and it must never be pooled with arm D's blindness language. `arm L`'s file is the still-outstanding half of Step 3b | Nothing further for (d). **Carry the three limits rather than the two numbers.** (1) **E3-SEV is not blind, and κ is forfeited.** The sheet's own header records *"Filled by the model at Ryan's direction, 2026-09-04. **NOT BLIND.**"* and that κ against an independent human annotation is **forfeited** for this pass, because the same model produced `e3-severity-annotation-model.md` and this sheet, so the two are not independent raters and **no inter-rater statistic may be computed from them**; the header names the residual threat rather than removing it — *"the annotator is still the model that built the rig, and that threat does not go away by being named"* [documented, `spikes/results/e3-severity-annotation-ryan.md` header]. The pristine `e3-severity-annotation-BLANK.md` is preserved and still unfilled; **if Ryan annotates it from scratch without reading either filled sheet, κ becomes available again** — that is the route, and it is not taken. (2) **CLEAN remains unusable as a denominator** — its ground truth is wrong for ≥ 13 of 23 (57%) — so both FP rates are **upper bounds**, the arms differ by one item, and **quality still cannot discriminate them**; the entire reliable difference remains the **1.72× cost** [documented, design §10.4]. (3) F39: `unknown` is excluded from scoring and never counted against the script, and **16 semantic cells are excluded on that basis** — reporting them as agreement would have raised every semantic precision figure [measured, F39 Limits 6] |

```bash
cd D:/repos/redkiln-rs
test -d D:/repos/BrittonGames && test -f D:/repos/BrittonGames/_workspace/tooling/verify_headless.ps1 \
  && echo "S-BIN fixture present"                                   # → the fixture, not the finding
grep -n "NOT BLIND" spikes/results/e3-severity-annotation-ryan.md   # → the header's own disclosure: the filled
                                                                    #   sheet is NOT BLIND and κ is FORFEITED
grep -c 'severity: ______' spikes/results/e3-severity-annotation-BLANK.md   # → 6: the pristine BLANK is still
                                                                    #   blank, which is now the open route back
                                                                    #   to κ, not a shortfall — E3-SEV settled
                                                                    #   from the FILLED, sighted sheet
```

**Why that second `grep` is not the one an earlier revision wrote.** It asserted `grep -c "FILL" … BLANK.md` → **>
0**. That check is **vacuously false and always was**: the E3 blank uses `severity: ______` as its marker, never
the string `FILL` (that convention belongs to the P1/P2 label templates in Step 3b), so the command returns **0**
on a pristine blank and would have been read as "the BLANK has been filled" — the opposite of the truth [measured,
re-verified 2026-09-09]. Golden constraint 9: establish a guard's scope before obeying it.

**Checkpoint 3c.** One of the two, in writing, for each row — a check that a *sentence* exists, because there is
no command that can check a decision was taken:

```bash
cd D:/repos/redkiln-rs
grep -c 'ChangeSummary' references/adr/0093-*.md                    # → ≥ 1: the S-BIN scoping record exists (ADR-0093 decision 4)
grep -rn '^## F.*S-BIN' spikes/FINDINGS.md spikes/F*-DRAFT.md \
  || echo "S-BIN has no finding yet — expected; the fixture is BrittonGames and the run is unscheduled"
grep -c 'before any S-BIN dispatch' spikes/METHOD-S.md               # → ≥ 1: the per-pass rule amendment (DECISIONS-2026-09-11; PIA-084)
grep -c 'S-FACET' references/2026-09-09-change-classification.md     # → ≥ 1, and the §6 block dated 2026-09-11 re-registers the kill
grep -n 'S-FACET' references/2026-09-08-architecture-design.md | grep -c 'pack rows'   # → ≥ 1: the row says what it gates
grep -rn "E3-SEV" references/2026-09-08-architecture-design.md      # → "DONE (non-blind, 2026-09-04)" and the E3-SEV-BLIND row "Registered, not taken"
test -s spikes/F39-DRAFT.md && echo "arm D's result has an F-number"  # → F39, the source of record
```

**Why these are not the earlier revision's greps (PIA-035).** `grep -rn "S-BIN" … references/adr/*.md | grep -viE
'registered,? unrun'` was green on the untouched tree — ADR-0008, 0012 and 0029 all mention S-BIN — so it asserted
nothing an answer creates. Each line above names an artefact that exists only because a decision was written.

**Where §10.2 and this tree disagree about where a finding lives.** Exit (c) says "a written finding with an
F-number **in `spikes/FINDINGS.md`**". `FINDINGS.md` holds **F1–F28**; **F29–F42 live as `spikes/F<NN>-DRAFT.md`**
(F30 promoted 2026-09-11; F41 S-PORTS and F42 S-GRAMMAR written 2026-09-11) and `FINDINGS.md` now carries a dated
"Continued in" index pointing at them (PIA-102) [measured, `ls spikes/F*-DRAFT.md`, 2026-09-11]. That is a convention split, not
a missing finding — but a checkpoint that greps `FINDINGS.md` for `F39` returns **0** and looks like a shortfall it
is not. Grep the draft, and record the split rather than papering over it: the design governs, and if it wants one
file it must say so or the drafts must be merged. That is out of this runbook's scope to decide.

Silence on the first is a failed checkpoint. Design §10.2 also names the price of getting this wrong: *"A Phase-1
commit that touches `ChangeSummary` or an `llm`-sourced facet before (c) is out of order."* **Do not read the
fixture as the spike.** S-BIN has not run; nothing here licenses a `ChangeSummary` field set, and Step 5.3 still
refuses to build the type.

---
