# Step 8 — The exit criterion

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |

*Header re-pointed 2026-09-24 (D21-40; `s8j` A0, made at insertion): the entry gate read Checkpoint 7i and `previous` read
`s7i` until the eleven `s8` sub-stages `s8a`–`s8j` were inserted before this stage.*

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Phase 1 exits on **six runnable checks** — design §10.2's seven **less (5), withdrawn 2026-09-15** (see (5) below;
`DECISIONS-2026-09-14 D-14`), with (1) now carrying a second half (ADR-0094: self-hosting is both lanes) — not on a
narrative milestone. **(5)'s number is retired and nothing renumbers**: the sequence is (1) (2) (3) (4) (6) (7),
because ADR-0110 (accepted) cites *"Phase-1 exit criterion (7)"* by number twice — in its Context and again as
its decision 5 — and the design carries the literal string *"§10.2 exit (6)"* at three sites: its §0.1
selection-engine row, its §10.2 phase table's own Phase-1 row and its §10.x review row. **Count them in
`references/2026-09-08-architecture-design.md` with `grep -cF`, never by line** — that design §10.2 Phase-1 row
gained its citation in this pass's own withdrawal edit, so a line-numbered count written earlier today was
already short by a site. A renumbering would silently move every one. All six, in order:

**(1) Self-hosting — AC-473, and AC-521.** Redkiln-RS runs its own fix lane end to end, **and** one story-lane run
authors a Phase-2 story under it (ADR-0094 decision 3; ADR-0011's exit bar, which ADR-0002's "solely" never
contradicted). Both run through the **Rust toolchain adapter** (`rk-adapter-toolchain-rust`) — this repository is a
Cargo workspace, and a Node-only Phase 1 could not have passed this line (PIA-004).

```bash
cd D:/repos/redkiln-rs
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (1) NOT RUN (Halt 8)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to a real defect id first — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
./target/debug/redkiln run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero"; exit 1; }   # ADR-0128 grammar (C-01 D-02, 2026-09-18): positional item, --lane optional. A run that STOPS AT A GATE exits 0 (C-01 D-01; AC-403) — this guard does not distinguish "landed" from "gated"; the trailer read below does.
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }   # POSITIONAL, the recorded resolution (AC-430–AC-432). Until 2026-09-18 this read `explain --run "$RUN_ID" --stages` — a run-keyed stage-listing mode no register criterion, ADR or design row names (C-01 D-03); the stage sequence is read from the event log's tier-1 trailer commits instead, next line.
test -n "$out" || { echo "FAIL: empty explain output — there is nothing to read against the PASS conditions below"; exit 1; }
log=$(git log "rk-run/$FIX_ITEM" --format=%B -E --grep='^(Story|Slice-Verdict|Gate):' 2>&1) || { echo "FAIL: git log over the trailer commits exited non-zero"; exit 1; }   # SCOPED TO THE RUN REF 2026-09-21 (Step 8, operator decision). This read `git log --all`, and `--all`
# made the guard below SATISFIABLE WITH ZERO RUNS PERFORMED: spike commit ba46ed9 ("H2/H3 RESULT:
# suspension is lossless...") carries a body line beginning `Slice-Verdict:`, so a run that wrote no
# tier-1 event at all still passed "the run wrote a tier-1 event" [measured 2026-09-21]. That is Halt 8's
# shape one level over, on the guard the whole self-hosting claim rests on. The run-owned ref satisfies
# `--all`'s OWN stated reason exactly -- the tier-1 commits live on rk-run/<item> (s7d H-5, ADR-0105
# clause 8; AC-518), never on the checkout's HEAD, which is why a bare `git log` would fail a correct
# binary (corrected 2026-09-18, re-spelt under RT-33) -- while admitting no commit the run did not make.
# A missing ref exits non-zero and fires the guard: no run ref means no run, which is fail-closed.
test -n "$log" || { echo "FAIL: no Story:/Slice-Verdict:/Gate: trailer commit on any ref (the run's trailers land on rk-run/<item> — s7d H-5) — the run wrote no tier-1 event (ADR-0105: git is the record)"; exit 1; }
printf '%s\n' "$out"; printf '%s\n' "$log"   # → READ both against the three lines below. NO stage-line pattern is pinned: the trailer values are the pack's stage names (7e's), so a grep for one would false-fail a correct binary.
# PASS only when the event log contains EVERY lane stage from intake to a landed commit,
# with NO stage performed by the old redkiln, and the verify stage's toolchain is rk-adapter-toolchain-rust.
# The claim is REFUSED if any stage is missing. The run id is the evidence; a statement is not.

# (1b) — AC-521, the story-lane twin
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM to the Phase-2 story to author"; exit 1; }
./target/debug/redkiln run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero"; exit 1; }   # ADR-0128 grammar; a gate-open exits 0 (C-01 D-01) — see (1)
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }   # POSITIONAL; was `explain --run "$RUN_ID" --stages` until 2026-09-18 (C-01 D-03)
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
log=$(git log "rk-run/$STORY_ITEM" --format=%B -E --grep='^(Story|Slice-Verdict|Gate):' 2>&1) || { echo "FAIL: git log over the trailer commits exited non-zero"; exit 1; }   # scoped to the run ref: see (1)
test -n "$log" || { echo "FAIL: no trailer commit on any ref (the run's trailers land on rk-run/<item> — s7d H-5) — the story-lane run wrote no tier-1 event"; exit 1; }
printf '%s\n' "$out"; printf '%s\n' "$log"   # → READ against the line below; no stage-line pattern is pinned, for the reason given in (1)
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the path the run authored"; exit 1; }
git ls-files --error-unmatch "$AUTHORED_STORY" || { echo "FAIL: the authored story is not tracked — the second half of the line below"; exit 1; }
# PASS only when the event log contains every story-lane stage and the authored story is in the tree.
```

**(2) Replay — AC-184 / SM-124, and the kernel replay test.**

```bash
cd D:/repos/redkiln-rs
# The instruments are the ones the REGISTER names, not doctor's stdout (DECISIONS-2026-09-12 E5).
# AC-161 asks for the stage-level next action "equal, under the CANONICAL SERIALISER"; AC-184 names "the
# rebuild differential test, run in CI on every commit" over the tier-1 projection (ADR-0105 clause 9).
# Neither needs a doctor output contract, and pinning one would create a second source of truth that drifts
# from the serialiser — and would go red on a rebuild timestamp. doctor's stdout stays a human affordance.
out=$(cargo test -p rk-kernel -- rebuild_differential 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rebuild_differential exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched rebuild_differential — AC-184's tier-1 projection differential does not exist under that name"; exit 1; }
out=$(cargo test -p rk-kernel -- stage_level_next_action 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: stage_level_next_action exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched stage_level_next_action — AC-161's canonical-serialiser equality is unasserted"; exit 1; }
# The operator path must still work (AC-427), but its STDOUT is not the comparison.
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }
./target/debug/redkiln doctor --rebuild || { echo "FAIL: doctor --rebuild exited non-zero (AC-427)"; exit 1; }
echo "(2) asserted: AC-184 differential + AC-161 next action, both under the canonical serialiser"
# This is the FIRST time the FSM is replayed at all — F19 proved suspension lossless 174/174 and explicitly
# left "same next action" unproven. Includes the adversarial-store variant and the 40-hex corpus-pin assertion.
```

**(3) Deterministic resume across a real kill, at MORE THAN ONE STEP.**

```bash
cd D:/repos/redkiln-rs
# Two halves (DECISIONS-2026-09-12 E5). The ASSERTED half is a kernel test over AC-013's N-step fixture graph;
# the OPERATIONAL half is the real-process arm ADR-0107 decision 4 pins, which is READ, not asserted. The digest
# field is NAMED since 2026-09-19 — ADR-0128 decision 4: each committed step's `output_digest` as `explain` prints it
# (E5's owed erratum closed by that record, DECISIONS-2026-09-18 O-13). AC-013 is PROVISIONAL on P8: F38 held exactly
# ONE step at n=3.
out=$(cargo test -p rk-kernel -- kill_and_resume_n_step 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: kill_and_resume_n_step exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched kill_and_resume_n_step — AC-013's multi-step digest equality is unasserted"; exit 1; }
echo "(3) asserted: AC-013 N-step kill-and-resume, digest equality, zero re-dispatch"
# --- the operational arm, in order. The PRE-KILL capture is taken HERE; until 2026-09-12 this block
# --- diffed against a capture it never took, and never performed the kill (it lived in a comment).
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item the run is on"; exit 1; }
pre=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: pre-kill explain exited non-zero"; exit 1; }   # POSITIONAL (ADR-0107 decision 4; design:1311) — not --item
test -n "$pre" || { echo "FAIL: empty pre-kill capture — two empty outputs compare equal"; exit 1; }
# Now kill the running process mid-graph, from another shell:   taskkill /T /F /PID <pid>
# Then re-invoke the SAME idempotent entry — there is no --resume flag and no resume verb (ADR-0107;
# AC-013/AC-186/AC-264/AC-400). That --resume is refused is itself AC-400's check, at s7d's checkpoint.
./target/debug/redkiln run "$ITEM" --lane fix || { echo "FAIL: the re-invocation exited non-zero"; exit 1; }   # ADR-0128 grammar (C-01 D-02): was `run --lane fix --item "$ITEM"`. A re-invocation that reaches a gate exits 0 (C-01 D-01), so this guard passes on a gated run too; the digest diff below is the assertion.
post=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: post-resume explain exited non-zero"; exit 1; }
test -n "$post" || { echo "FAIL: empty post-resume capture"; exit 1; }
diff <(printf '%s\n' "$pre") <(printf '%s\n' "$post")   # → READ this. Whole-output equality is NOT asserted: the
# post-resume output necessarily carries the resume's own appended events, so a bare `=` test would false-fail
# on a correct binary. The kernel test above is the generalising instrument; this arm is the real-process check.
# AC-265 (the gate answer surviving the kill) is exercised here, not in 7b-ii.
```

**(4) The determinism ban list fails the build.** `cargo clippy --workspace --all-targets` gated on its exit code
(never `-D warnings` on the command line — that form misparses beside the manifest's `[lints]` table and promotes
nursery to a group deny ADR-0083 forbids; PIA-030) plus the `clippy.toml`-driven fixture crate that reintroduces a
clock read (AC-001–AC-004; PIA-031 — a `trybuild` fixture cannot fail on a lint). The fixture **failing the lint** is
the pass.

```bash
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
test -f Cargo.toml && test -f clippy.toml || { echo "FAIL: no workspace-root Cargo.toml / clippy.toml — AC-001–AC-004 have no instrument; criterion (4) NOT RUN (Halt 8)"; exit 1; }
cargo clippy --workspace --all-targets || { echo "FAIL: the clippy gate is red (never -D warnings on the CLI — ADR-0083; PIA-030)"; exit 1; }
CLOCK_FIXTURE=rk-clock-read   # s5 §5.1a names it; a feature-gated workspace member (DECISIONS-2026-09-12 E4)
cargo metadata --format-version 1 --no-deps | grep -q "\"$CLOCK_FIXTURE\"" || { echo "FAIL: $CLOCK_FIXTURE is not a workspace member — a missing fixture must never read as the pass"; exit 1; }
# REPAIRED 2026-09-21 (Step 8, operator decision). This ran `cargo clippy -p "$CLOCK_FIXTURE"` with
# NO FEATURE, and under that invocation the fixture CANNOT FAIL: its offending call sits behind the
# non-default `reintroduce` feature, which is the whole point of the feature-gated-member shape
# (DECISIONS-2026-09-12 E4) -- with the feature off the default workspace gate stays green. So the
# line clipped clean and fired its own "the fixture PASSED the lint" FAIL on a CORRECT tree
# [measured 2026-09-21]. The fixture's own `tests/fixtures/rk-clock-read/Cargo.toml` spells the pair
# verbatim, and this is now the second of them:
#   green:  cargo clippy --workspace --all-targets
#   RED:    cargo clippy --workspace --all-targets --features rk-clock-read/reintroduce
# The workspace form is the one AC-001's Verified-by asks for -- *"checked under the same `cargo
# clippy` invocation"* -- and `$CLOCK_FIXTURE` stays load-bearing as the feature's package.
# AND THE REASON IS ASSERTED, NOT ONLY THE REDNESS (golden constraint 12): a fixture that failed to
# compile for any other cause would satisfy a bare non-zero check while measuring nothing.
out=$(cargo clippy --workspace --all-targets --features "$CLOCK_FIXTURE/reintroduce" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the clock-read fixture PASSED the lint — the fixture FAILING it is the pass (PIA-031)"; exit 1; }
printf '%s\n' "$out" | grep -q 'use of a disallowed method' || { echo "FAIL: the fixture is red for some OTHER reason than the determinism ban — a red for the wrong reason is not a pass (golden constraint 12)"; printf '%s\n' "$out" | tail -20; exit 1; }
```

**(5) WITHDRAWN 2026-09-15.** It read *"The `--stay`-class error is unrepresentable — a `trybuild` must-fail
fixture, not a runtime assertion"*, and its obligation is **folded into AC-294** (`DECISIONS-2026-09-14 D-14`):
AC-294 already owns a private constructor, a module-boundary architecture lint and a `trybuild` case — the
identical instrument family in the same crate — while check (5)'s only warrant was ADR-0001:42, which the
register classes as *"a premise, not a checkable behaviour"*. **The number is retired; nothing renumbers.**

**Its shell block is cut, not annotated, and what was wrong with it is recorded here rather than left standing.**
The block ran `cargo test -p rk-kernel -- trybuild` and asserted `test result: ok. [1-9][0-9]* passed`. **That
filter is green whenever ANY ONE case passes**: 13 case files live in `crates/rk-kernel/tests/ui/` and the
`trybuild` filter does select the harness [measured 2026-09-15], so an **absent** `--stay` case read exactly like a
present one. **This is not a Halt 8 shape and must not be described as one** — it captured `rc` before grepping,
greped the capture and not a pipe, and every guard ended in `exit 1`, so it is none of Halt 8's sub-shapes
(a)–(h); and Halt 8's headline (*"a path it names does not exist"*) cannot engage, because the block **named no
fixture path at all**. The defect is the any-case-passes filter, and its repair is the **by-path** assertion now
carried in Checkpoint 8 below — not a re-run of the harness. AC-294's own discharge stays **Checkpoint 7b-ii's**
(`s7b-ii` :697-698, its block-constructor case); Step 8 asserts only that the `--stay` case file and its committed
`.stderr` are still there.

**(6) `redkiln lint --facet-stability` (AC-071) passes**, which means one of exactly two things and nothing else:

```bash
# [WILL EXIST] — and so does its argument: there is NO .redkiln/ directory in redkiln-rs today
# (verified 2026-09-11). The facet registry that exists now is spikes/p9/facets.toml, which STEP 7e's
# pack authoring derives .redkiln/facets.toml from (PIA-011). Both the binary and the file are Phase-1 output.
./target/debug/redkiln lint --facet-stability .redkiln/facets.toml
# The passing branch after Step 4a is the ZERO-llm branch, which needs no stability record -- this is the
# form s7e Checkpoint 7e uses FOR THIS LINT, and it is why the argument is dropped here. **Scoped
# 2026-09-21:** this said "the form s7e Checkpoint 7e uses" unqualified, which is true of
# `--facet-stability` (7e drops `--stability-record` too) and FALSE of `--facet-provenance` (7e passes
# `--pinned-corpus`, and so does this file's own line below, as amended the same day). A dropped argument
# is the right form for one lint and the exit-3 defect for the other. A bare <path> is a bash
# redirect, not an argument: the line above never ran until 2026-09-12. If a facet is ever
# source = "llm", the record-bearing form is
#   CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
#   STABILITY_RECORD="$CORPUS_REPO/results/p1-arm-l-stability.json"   # the path, filled 2026-09-15; the
#                                    # placeholder that stood here (`STABILITY_RECORD=  # set to a real path
#                                    # first`) was the landing record's to fill and is now filled. Read
#                                    # CORPUS-RELATIVE THROUGH CORPUS_PIN, never through a working tree (AC-464).
#   test -n "$STABILITY_RECORD" || { echo "FAIL: no stability record path"; exit 1; }
#   test -s "$STABILITY_RECORD" || { echo "FAIL: no stability record at $STABILITY_RECORD — an absent record must never read as the pass"; exit 1; }
#   ./target/debug/redkiln lint --facet-stability .redkiln/facets.toml --stability-record "$STABILITY_RECORD"
# [WILL EXIST] -- re-run at Checkpoint 7e, which runs both lints against the product registry. CITED BY
# QUOTATION, NOT BY LINE (corrected 2026-09-21): this read "where s7e:120-121 runs both lints" and s7e has
# since grown to 234 lines, where :120-121 resolves to the `stale_policy` warrant prose instead. The two
# lines meant are the ones opening `./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml`
# and `./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml --pinned-corpus`, checkable
# with `grep -cF` at any later moment.
# exit 0 only when EITHER zero facets are declared source = "llm",
# OR a stability record satisfies ALL FIVE clauses:
#   modal_required_set_identity >= 0.80  AND  n_items >= 20  AND  draws_per_item >= 10
#   AND panel_verdict == "scored"        AND  corpus_sha == <the pin the MEASUREMENT was taken at>
#
# READ THE SECOND BRANCH AS FAILING. It is stated plainly here because the record's PATH is now named --
# by AC-071's dated register amendment, 2026-09-15 -- and the block must not imply the gate could pass on it.
# THE RECORD FILE NOW EXISTS. *(Corrected 2026-09-15, later the same day: this read "THE RECORD FILE
# ITSELF DOES NOT EXIST", measured against pin 27d97e9b before s7b §7b.11's move. The operator minted it
# and moved the pin 27d97e9b -> 61f590e5, so s7b §7b.11 precondition 2 is SATISFIED and Checkpoint 7b has
# PASSED. NONE OF THAT CHANGES THE BRANCH ABOVE, which still reads as FAILING -- and that is the point of
# saying so here: the record's existence was never what the gate turned on.) It is a TRANSCRIPTION of F36
# that s7b §7b.2 forbids regenerating; its scalar is the MINIMUM of F36's four
# identity figures, 0.10, so
# CLAUSE 1 FAILS at 0.10 against the 0.80 line [measured, F36: 0.25 / 0.15 / 0.10 / 0.10].
# CLAUSE 5 DOES NOT CLEAR EITHER, and that is fail-closed and correct. Under DECISIONS-2026-09-15 D15-1,
# `corpus_sha` in a TRANSCRIBED record denotes THE PIN THE MEASUREMENT WAS TAKEN AT --
# 47faec9f46b8d983c2c75f0794164e68415ae706, the value F36's own engine rows carry and the value
# crates/rk-kernel/tests/it/facet_stability.rs:37 already hard-codes as CATALOG_CORPUS_PIN -- and NOT the
# catalog pin at read time, which CORPUS_PIN moves. So clause 5 is unsatisfied whenever CORPUS_PIN is not
# that sha. A record edited to make clause 5 pass while clause 1 failed would be the "a lint that reads
# identity and skips the panel clause" failure one level over, and D15-1 refuses it by name.
# panel_verdict == "scored" IS SATISFIABLE AND, FOR ARM D, SATISFIED — F39 scored it on 2026-09-09
# over 140 truth cells at 47faec9f (all scored after the 2026-09-11 panel-file key fix; the 2026-09-09 run scored 136 — F39-DRAFT's Correction).
# A discharged clause is not a partial unblocking: the gate still fails on the second branch — on clause 1
# and on clause 5 — and the fixture set stays at FIVE. After Step 4a the FIRST branch is what passes — zero
# llm facets (ADR-0090; DECISIONS-2026-09-11 #2) — which is the expected outcome, and it is the branch this
# check takes: nothing in Phase 1 reaches the record-bearing form.
```

**(6, second lint) `redkiln lint --facet-provenance` (AC-520) passes** — the derived-facet sibling, new since F39,
which gates the half of the registry AC-071 never reached. **This is not an added exit criterion**: design §10.2
names seven, this runbook **withdraws (5)** (2026-09-15) and adds none. It is stated here because AC-520 is the criterion that makes
(6)'s neighbouring half checkable, and a Phase 1 that exits on (6) with `--facet-provenance` unbuilt has left the
measurably-wrong half of the registry ungated — which is the state F39 found:

```bash
# [WILL EXIST] — Step 4a authors the registry half, Step 7b builds the lint, Step 7e derives the file.
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml --pinned-corpus .redkiln/corpus-records.toml
# AMENDED 2026-09-21 (Step 8, operator decision). This line ran WITHOUT `--pinned-corpus` and exited 3:
# `crates/rk-cli/src/verbs/lint.rs` refuses to default that path in terms -- *"its default ... is specified
# by no record, and a path invented here would be read as one that was decided."* Checkpoint 7e line 141
# carried the IDENTICAL defect and was fixed the same way: author `.redkiln/corpus-records.toml` and pass
# it. The argument is the file 7e authored (six `[[corpus_record]]` rows transcribed from arm D's panel).
# The line stays a bare invocation, as its `--facet-stability` twin above is: this block is a READ of an
# exit code, and adding a `|| { ...; exit 1; }` guard here would change what the criterion asserts.
# exit 0 only when EVERY facet declared source = "derived" names a `deriver` AND a scored precision
# record exists for it over a pinned corpus, carrying a Wilson 95% interval and a positive-instance
# count. A facet with NO positive instance is reported NOT MEASURABLE — neither passed nor failed —
# and may key no require row. A lexical deriver never satisfies PC8 rule 2 for require-row purposes.
# NO THRESHOLD is re-stated here on purpose: METHOD-P §1.4's 0.95 bar is not reachable at n=20 for
# any facet however good [measured, F39 Limits 1]. The gate requires a record that EXISTS, IS PINNED
# AND IS READ.
#
# WHICH BRANCH PASSES, BY NAME (PIA-001; ADR-0093 decision 2): six facets are `derived` and every one has a
# record at 47faec9f — toolchain 0.73 [0.48, 0.89], change 0.72 [0.54, 0.85], ui 1.00 [0.65, 1.00],
# flag_exists 0.00 [0.00, 0.66] (F39); data_migration NOT MEASURABLE (0 positives); recent_deploy NOT
# MEASURABLE (feed absent) — so the lint exits 0 with two NOT MEASURABLE lines and those two facets key no
# require row. stage, rigor, role and cost are `kernel`; surfaces is `declared`; none of them is gated here.
```

On today's evidence (6)'s **first** branch is what passes, and (6, second lint) passes on six named records. That is
the expected outcome, not a shortfall.

**(7) Every dispatch event carries a cost row (AC-394), and headroom is reported only as a batch delta with `n` and
`resolution_limited`** (AC-372/AC-373/AC-395; **SM-111** — the previous revision cited SM-109, which is attribution
completeness, PIA-061). `utilization` is quantised to 0.01, so per-run headroom is **not measurable**; the delta is
computed in integer ticks over readings sharing one `resetsAt`, over the account's complete dispatch sequence
(`redkiln cost <item>` prints `not attributable` with the interleaving count otherwise), and a set with `n < 10` is
`low_sample` with no delta quoted (ADR-0110). `redkiln cost` renders the flag; a run that reports a per-run figure
without it has laundered an unmeasurable into a measurement.

```bash
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
printf '%s\n' "$out"   # → READ: every dispatch carries a cost row (AC-394) and no per-run headroom figure appears (AC-373). `n` is deliberately NOT greped for — ADR-0110 requires the count, never a rendering of it.
```

**Checkpoint 8.** All six pass (with (1)'s two halves and (6)'s two lints), and the following remain true and
unviolated across the whole phase:

- No `require` row keyed on an LLM-emitted `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`,
  `external_dep` or `has_repro` — and, after Step 4a, no facet is `source = "llm"` at all.
- **No `stale_policy` value was defaulted from P2**: each cost class's setting in every Phase-1 pack is
  `rematerialise`, coarse scope, with F40 §3's contraindication quoted as its warrant (decision 5; Step 7e).
- **No `require` row keyed on a `declared` facet** (`surfaces`) or on a NOT MEASURABLE one (`data_migration`,
  `recent_deploy`) — ADR-0093.
- **`bisect` carries `max_rigor = "recommended"`** and the rigor-monotonicity lint passes on the shipped catalog
  (P7-CEIL, DECISIONS-2026-09-11 #1).
- No finalised `ChangeSummary` type; no synthesised `ChangeSummary` value. **The S-BIN fixture existing at
  `D:/repos/BrittonGames` does not change this** — the spike has not run.
- No card append route enabled as anything but an unmeasured config option.
- No quality-shorthand default; no `findings.none(evidence in [Reproduced, MutationSurvived])` as a terminal
  condition.
- No run reported as 1 h TTL without per-run `ephemeral_*` field evidence.
- gix does not own the write path.
- **No document authored or amended from here on ASSERTS that a facet "leaves the model".** None does: no facet
  clears METHOD-P §1.4's 0.95 bar and the bar is **not reachable at n=20 for any facet however good**
  [measured, F39]. **Scope, because the unscoped form of this rule is unsatisfiable:** the phrase lives, correctly
  and permanently, in `spikes/METHOD-P.md` §1.4 (the registered rule itself), in `references/adr/0079-*.md`, in
  `spikes/F36-DRAFT.md` and in `spikes/DECISIONS-2026-09-09.md`. Those are frozen sources and quoting a registered
  rule is not asserting it — editing them to satisfy this line is the one thing the rule must never cause. The
  mechanical check is scoped to `crates/` for exactly that reason (Checkpoint 8 above).
- **No facet declares `source = "derived"` without a named `deriver` and a scored precision record** (AC-520,
  ADR-0089), and **no `require` row is keyed on a facet whose scored record shows no positive instance** — that
  facet is reported *not measurable*, neither passed nor failed.
- **No derived boolean resolves an absence of evidence to `false`.** It is `null`, and `null` routes to AC-079's
  `gate_opened { reason: unknown_facet }`. **No `null` is counted as a false negative** in any scoring of a facet
  deriver.
- **No document reports F39's oracle as human-panel.** It is **panel-adjudicated**: 60 deterministic-oracle cells,
  48 unanimous panel, 32 human adjudication of 140 [measured, F39 §5].
- **No document quotes "four questions on twenty items" as a triage-gate load forecast.** It is a measurement of an
  oracle-construction exercise; SM-024 has no baseline and S-FACET is what gives it one.
- **No section decides source asymmetry.** `commits` beats `docs` and `union` is worse than `commits` alone on
  `toolchain` and `change` [measured, F39 §3]; the decision is an arm of S-FACET and ADR-0089 does not take it.
- **No document, section or commit message describes this design as final.** S-BIN, S-H1c and P8's unrun arms gate
  that label, and P8 has no result on its own question.
- **The `--stay`-class state stays unconstructible, and its `trybuild` case is asserted BY PATH.** *(Added
  2026-09-15 with the withdrawal of exit criterion (5); `DECISIONS-2026-09-14 D-14`.)* Both
  `crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs` **and its committed `.stderr`** exist, and
  the kernel's must-fail suite passes under its own name. A harness that runs proves nothing about a case that is
  not there — which is exactly what the withdrawn check (5) could not see.

Those seventeen are not read, they are run. Checkpoint 8's mechanical lines, in bullet order:

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# Guards first: an absent or empty tree answers every prohibition below for free (Halt 8).
test -n "$(ls -A crates)" || { echo "FAIL: crates/ empty — Checkpoint 8's crates/ checks NOT RUN (Halt 8)"; exit 1; }
test -f .redkiln/facets.toml && test -d .redkiln/packs && test -d .redkiln/catalog || { echo "FAIL: .redkiln/ not derived — Checkpoint 8's registry and pack checks NOT RUN (Halt 8)"; exit 1; }

# no require row keyed on an LLM-emitted facet, and after Step 4a no llm facet at all
# REPAIRED 2026-09-21 (Step 8, operator decision). This was ONE statement and it was RED on a correct
# tree: nine hits, EVERY ONE A COMMENT -- five doc-comments under crates/ (including
# `crates/rk-kernel/tests/it/rigor.rs`, whose comment carries this very ban's regex) and four `#`
# lines under .redkiln/ (three catalog `# reach: ... require row` headers and one 7e pack note).
# NOT ONE is a require row. s7b settled this family's reading already: *"Comment lines are excluded
# rather than halted on: the house pattern is a file whose header quotes the governing rule in
# prose."* Halt 7's rule there -- *"do not widen the guard, and do not write the hit"* -- is why the
# PROSE does not give way here: it is pre-existing, and one hit is the ban itself.
# THE PATTERN IS DELIBERATELY NOT WIDENED to s7b's `(require|requires|applicable_if|when)` predicate
# form. Measured 2026-09-21: that form returns 79 comment-free hits over this line's scope, every one
# legitimate -- `when = "criticality >= high"` rows in the SHIPPED packs and `applicable_if` rows in
# the prescribed fixture trees -- because all seven axes are `source = "human"` after Step 4a
# (ADR-0090), and design 10.2 principle 3 scopes the ban to a facet *sourced from an LLM
# classification dispatch*. s7b carries that form only behind its two path exclusions; this line's
# scope includes .redkiln/, where those rows correctly live. The llm half is asserted by the NEXT
# line, mechanically and completely: zero facets declare `source = "llm"`.
raw=$(grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' crates/ .redkiln/ --include=*.rs --include=*.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LLM-ordinal search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
live=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE '^[[:space:]]*$'); test -z "$live" || { echo "FAIL: an LLM-emitted facet keys a require row:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf 'p.toml:1:require = "has_repro"\n' | grep -E 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)'); test -n "$syn" || { echo "FAIL: the comment exclusion swallowed a live require row -- this ban is vacuous"; exit 1; }
test "$(grep -c '^source = "llm"' .redkiln/facets.toml)" = 0 || { echo "FAIL: a facet still declares source = \"llm\" (ADR-0090; DECISIONS-2026-09-11 #2)"; exit 1; }

# stale_policy recorded per cost class, never defaulted from P2 (decision 5; Step 7e; warrant F40 §3)
set=$(grep -rh 'stale_policy' .redkiln/packs --include=*.toml); test -n "$set" || { echo "FAIL: no stale_policy written in any pack — decision 5 was defaulted, not recorded"; exit 1; }
printf '%s\n' "$set" | grep -v 'rematerialise'; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a stale_policy is not rematerialise (rc=$rc)"; exit 1; }

# no require row on a declared or NOT MEASURABLE facet (ADR-0093)
# REPAIRED 2026-09-21 (Step 8, operator decision), same shape as the line above and same warrant.
# RED on a correct tree: six hits, ALL SIX Step 7e's own `-- DROPPED: [[stage.X.require]] when =
# 'data_migration'` bookkeeping comments -- records of rows 7e REMOVED yesterday under this very rule.
# A gate broken by the record of its own enforcement is the defect, not the record.
# THE PATTERN IS DELIBERATELY NOT WIDENED to the predicate-keyword form here either, and the reason is
# sharper than above: measured 2026-09-21, that form returns 8 comment-free hits over this scope and
# EVERY ONE IS AN `applicable_if`, NOT A `require` ROW -- .redkiln/catalog/threat-model.toml,
# security-review.toml, security-requirements.toml, data-migration-plan.toml and four more. ADR-0093
# bars these facets from keying a REQUIRE ROW and leaves `applicable_if` as the sanctioned route; 7e's
# own DROPPED notes say so in terms (*"The demand rides on catalog/threat-model.toml's
# applicable_if"*). Widening here would make the gate red on the route the record prescribes.
raw=$(grep -rnE 'require.*\b(surfaces|data_migration|recent_deploy)\b' .redkiln/packs .redkiln/catalog); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the declared/NOT-MEASURABLE search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
live=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE '^[[:space:]]*$'); test -z "$live" || { echo "FAIL: a declared or NOT MEASURABLE facet keys a require row:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf 'p.toml:1:require = "data_migration"\n' | grep -E 'require.*\b(surfaces|data_migration|recent_deploy)\b' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)'); test -n "$syn" || { echo "FAIL: the comment exclusion swallowed a live require row -- this ban is vacuous"; exit 1; }

# bisect's ceiling on the SHIPPED catalog — spikes/p9's copy is already green, so it may not be the path (PIA-035)
grep -q 'max_rigor *= *"recommended"' .redkiln/catalog/bisect.toml || { echo "FAIL: bisect has no max_rigor = \"recommended\" ceiling (P7-CEIL; DECISIONS-2026-09-11 #1)"; exit 1; }
out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the rigor_monotonicity filter matched 0 tests"; exit 1; }

# ChangeSummary nowhere in crates/ but its rk-ports newtype (ADR-0093; S-BIN unrun — the BrittonGames fixture is not the spike)
# REPAIRED 2026-09-21 (Step 8, operator decision). This was a bare MENTION-grep and it was RED on a
# correct tree: 67 hits across 16 files, most of them comments and every non-comment one a `use` or a
# trait-method signature that ADR-0094 decision 1 REQUIRES -- both toolchain adapters implement
# `Toolchain::change_summary`, so they must name the type. NARROWED TO THIS BULLET'S OWN WORDS: *"No
# finalised `ChangeSummary` type; no synthesised `ChangeSummary` value."* A definition, or a
# construction, outside rk-ports. Measured 2026-09-21: the ONLY definition in the workspace is
# `crates/rk-ports/src/toolchain.rs`'s `pub struct ChangeSummary(pub(crate) ())` -- an opaque newtype
# whose single field is crate-private, so it is neither finalised nor constructible from outside --
# and there are ZERO constructions anywhere outside rk-ports; both adapters' `change_summary` returns
# `Err(registry::refusal(...))` and synthesises nothing. So this bullet is TRUE of the tree.
# THIS IS NOT CHECKPOINT 5'S BAN AND DOES NOT DISCHARGE IT. Checkpoint 5 bans naming THE TYPE at all
# outside rk-ports (ADR-0099 clause 5) and is RED with four hits in the two adapters -- 7c's debt,
# PROGRESS.md row 5 RAISED, carried here untouched and NOT closed by this narrowing.
raw=$(grep -rnE '\b(struct|enum|type|trait)[[:space:]]+ChangeSummary\b|ChangeSummary[[:space:]]*[({]' crates/ --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the ChangeSummary search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
printf '%s\n' "$raw" | grep -q '^crates/rk-ports/src/toolchain.rs:[0-9]*:pub struct ChangeSummary(' || { echo "FAIL: rk-ports' own opaque newtype was not found by this search -- the ban is looking at the wrong tree"; exit 1; }
out=$(printf '%s\n' "$raw" | grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE '^[[:space:]]*$'); test -z "$out" || { echo "FAIL: a finalised ChangeSummary type or a synthesised ChangeSummary value outside rk-ports:"; printf '%s\n' "$out"; exit 1; }
syn=$(printf 'x.rs:1:    let s = ChangeSummary(());\n' | grep -E '\b(struct|enum|type|trait)[[:space:]]+ChangeSummary\b|ChangeSummary[[:space:]]*[({]' | grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)'); test -n "$syn" || { echo "FAIL: the exclusions swallowed a synthesised ChangeSummary value -- this ban is vacuous"; exit 1; }

# card append route never defaulted on; no quality-shorthand default or terminal condition
grep -rnE 'append(_route)? *= *true' crates/ .redkiln/ --include=*.rs --include=*.toml; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a card append route is enabled by default (rc=$rc)"; exit 1; }
# REPAIRED 2026-09-21 (Step 8, operator decision). RED on a correct tree: six hits -- four comments
# quoting ADR-0080's forbidden predicate (including the must-REFUSE fixture's own header) and two
# Rust STRING LITERALS at `crates/rk-kernel/tests/it/evaluator.rs`, which feed that predicate to
# `truth()` to assert it evaluates Tri::False over a reproduced finding and Tri::True over an absent
# one. That file is the ENFORCEMENT of the rule; a guard that halts on it and the test that proves the
# predicate behaves are jointly unsatisfiable. Comments are filtered and that one path is excluded --
# s7b's route, the EXCLUSION widened rather than the PATTERN narrowed, so a hit anywhere else halts.
#
# RAISED, NOT ACTED ON -- an operator decision, not an instrument repair. This pattern matches only the
# UNQUOTED spelling, and the shipped catalog writes the predicate with QUOTED members. Measured
# 2026-09-21, a pattern admitting an optional quote returns three live `expr` rows in the SHIPPED
# catalog -- .redkiln/catalog/root-cause.toml:62, differential-debugging.toml:65 and
# adversarial-review.toml:41 -- plus the prescribed red fixture
# crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_none.toml:65, whose own
# header says it copies that very spelling from `spikes/p9/catalog/root-cause.toml`:62. Whether an
# `expr` on a `kind = "check"` step carrying `on_false` IS the *terminal condition* ADR-0080 decision 2
# forbids is a reading of the record, and widening this pattern would turn the gate red on the pack
# Step 7e authored. That is changing what the gate asserts, so it is NOT taken here: it is carried to
# the operator as an open question against ADR-0080 and AC-527.
raw=$(grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *=' crates/ .redkiln/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the quality-shorthand search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
out=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -v '^crates/rk-kernel/tests/it/evaluator.rs:' | grep -vE '^[[:space:]]*$'); test -z "$out" || { echo "FAIL: a quality-shorthand default or terminal condition:"; printf '%s\n' "$out"; exit 1; }
syn=$(printf 'a.toml:1:expr = "findings.none(evidence in [Reproduced, MutationSurvived])"\n' | grep -E 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *=' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -v '^crates/rk-kernel/tests/it/evaluator.rs:'); test -n "$syn" || { echo "FAIL: the exclusions swallowed a live terminal condition -- this ban is vacuous"; exit 1; }

# gix does not own the write path — a SCREEN, because ADR-0098 retains one gix write deliberately
test -f crates/rk-git/Cargo.toml || { echo "FAIL: rk-git absent — write-path check NOT RUN (Halt 8)"; exit 1; }
printf '%s\n' "$(grep -rn 'gix' crates/rk-git/Cargo.toml crates/rk-git/src/)"   # → READ every hit: the write path is the exec wrapper (ADR-0077) and the review clone is the ONE retained gix write (ADR-0098); zero hits and review-clone-only are both passes, anything else is not

# the "leaves the model" check this checkpoint promises — widened to leave[sd]?, because METHOD-P §1.4 and ADR-0079 say "leave the model"
grep -rniE 'leave[sd]? the model' crates/ --include=*.rs --include=*.toml --include=*.md; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a crates/ document asserts a facet leaves the model — none does (F39) (rc=$rc)"; exit 1; }

# AC-520: every derived facet names a deriver, and a vacuous 0 0 is not a pass
awk '/^source = "derived"/{d++} /^deriver = /{v++} END{print d+0, v+0; exit !(d>0 && d==v)}' .redkiln/facets.toml || { echo "FAIL: derived facets without a named deriver, or a vacuous 0 0"; exit 1; }   # → 6 6

# no derived boolean resolves an absence of evidence to false — it is null, routed to AC-079
out=$(cargo test -p rk-kernel -- unknown_facet 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the unknown_facet filter matched 0 tests — the tri-valued path is untested"; exit 1; }

# Three SCREENS. Their predicate is a reading, so no hit count is the verdict and none of them may go red on a hit.
test -d references && test -d runbook && test -d spikes || { echo "FAIL: doc tree absent — Checkpoint 8's screens NOT RUN (Halt 8)"; exit 1; }
printf '%s\n' "$(grep -rniE '1 ?h TTL' --include=*.md references/ runbook/ spikes/ | grep -v 'ephemeral_')"   # → every surviving hit READ: none is a per-run TTL report without ephemeral_* evidence (ADR-0078)
printf '%s\n' "$(grep -rniE 'human[- ]panel' --include=*.md references/ runbook/ spikes/ | grep -vE 'panel-adjudicated|not a human|no human|never')"   # → READ: F39's oracle is 60 oracle / 48 unanimous panel / 32 human of 140; most survivors are METHOD-P §1.2's own human agreement panel, a different object
printf '%s\n' "$(grep -rniE 'design (is|as) final' --include=*.md references/ runbook/ spikes/*.md | grep -vE 'may (not|never)|no document|BLOCKED|gates that|not .*final')"   # → READ; spikes/fixtures/ is excluded on purpose (frozen RFC prose, and unscoped this regex returns 173 lines)

# "four questions on twenty items" — a plain grep, because ZERO hits would mean design §10.5's row was deleted
grep -rn 'four questions on twenty items' --include=*.md . ; rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }   # → each hit READ: an oracle-construction measurement, never a forecast

# source asymmetry is NOT a predicate. The runnable half is that the record still holds the question open.
grep -q 'Source asymmetry is an arm of the same spike and is settled nowhere' references/2026-09-08-architecture-design.md || { echo "FAIL: the asymmetry-open statement is gone from design §10.5 — a section may have decided it"; exit 1; }

# the --stay case BY PATH — the assertion withdrawn check (5) never made. Modelled on Checkpoint 7b's own
# by-path loop, in `s7b-kernel.md` under `### 7b.14 Checkpoint 7b`, whose opening line reads
#   for f in stay_class_state_is_unconstructible ac480_generics_leak ac050_rigor_is_not_arithmetic
# CITED BY THAT LITERAL AND NOT BY LINE, checkable with `grep -cF` at any later moment. The line range that
# stood here did NOT hold that loop — it resolves to the GRAMMAR-name loop, whose own opening line reads
# `for t in grammar_agrees_with_the_s_grammar_probe` — and `s7b-kernel.md` says of its by-path block that
# "it moves again with this pass". A green trybuild harness is NOT this: `-- trybuild` passes whenever any one of the
# 13 cases in tests/ui/ passes, so the case file itself is named, and its .stderr is asserted non-empty.
test -f crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs || { echo "FAIL: the --stay trybuild case is absent — a harness that runs proves nothing about a case that is not there (AC-294; DECISIONS-2026-09-14 D-14)"; exit 1; }
test -s crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.stderr || { echo "FAIL: the --stay case commits no .stderr, so it is green whenever compilation fails for ANY reason (s7a §7a.8)"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- trybuild_pins_the_reason_each_kernel_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the kernel must-fail suite did not pass (rc=$rc) — a fixture COMPILED, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test of that name ran and passed — a filter matching nothing exits 0 (00-preamble.md:178)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
```

---
