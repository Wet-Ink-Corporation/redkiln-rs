# Step 8 — The exit criterion

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8` | Phase 1 | Checkpoint 7e | Checkpoint 8 | `s7e` | — |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Phase 1 exits on **seven runnable checks** — design §10.2's seven, with (1) now carrying a second half (ADR-0094:
self-hosting is both lanes) — not on a narrative milestone. All seven, in order:

**(1) Self-hosting — AC-473, and AC-521.** Redkiln-RS runs its own fix lane end to end, **and** one story-lane run
authors a Phase-2 story under it (ADR-0094 decision 3; ADR-0011's exit bar, which ADR-0002's "solely" never
contradicted). Both run through the **Rust toolchain adapter** (`rk-adapter-toolchain-rust`) — this repository is a
Cargo workspace, and a Node-only Phase 1 could not have passed this line (PIA-004).

```bash
cd D:/repos/redkiln-rs
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (1) NOT RUN (Halt 8)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to a real defect id first — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
./target/debug/redkiln run --lane fix --item "$FIX_ITEM" || { echo "FAIL: the fix-lane run exited non-zero"; exit 1; }
out=$(./target/debug/redkiln explain --run "$RUN_ID" --stages) || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty stage log — there is nothing to read against the PASS conditions below"; exit 1; }
printf '%s\n' "$out"   # → READ against the three lines below. NO stage-line pattern is pinned: `explain --run … --stages` is a shape found nowhere else in the corpus (audit 2026-09-10:895), so a grep for one would false-fail a correct binary.
# PASS only when the event log contains EVERY lane stage from intake to a landed commit,
# with NO stage performed by the old redkiln, and the verify stage's toolchain is rk-adapter-toolchain-rust.
# The claim is REFUSED if any stage is missing. The run id is the evidence; a statement is not.

# (1b) — AC-521, the story-lane twin
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM to the Phase-2 story to author"; exit 1; }
./target/debug/redkiln run --lane story --item "$STORY_ITEM" || { echo "FAIL: the story-lane run exited non-zero"; exit 1; }
out=$(./target/debug/redkiln explain --run "$RUN_ID" --stages) || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty stage log"; exit 1; }
printf '%s\n' "$out"   # → READ against the line below; no stage-line pattern is pinned, for the reason given in (1)
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the path the run authored"; exit 1; }
git ls-files --error-unmatch "$AUTHORED_STORY" || { echo "FAIL: the authored story is not tracked — the second half of the line below"; exit 1; }
# PASS only when the event log contains every story-lane stage and the authored story is in the tree.
```

**(2) Replay — AC-184 / SM-124, and the kernel replay test.**

```bash
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2) NOT RUN (Halt 8)"; exit 1; }
before=$(./target/debug/redkiln doctor) || { echo "FAIL: doctor exited non-zero before the rebuild"; exit 1; }
test -n "$before" || { echo "FAIL: doctor printed nothing before the rebuild — two empty readings compare equal"; exit 1; }
./target/debug/redkiln doctor --rebuild || { echo "FAIL: doctor --rebuild exited non-zero (AC-427)"; exit 1; }
after=$(./target/debug/redkiln doctor) || { echo "FAIL: doctor exited non-zero after the rebuild"; exit 1; }
test -n "$after" || { echo "FAIL: doctor printed nothing after the rebuild"; exit 1; }
printf '%s\n' "$before"; echo '--- rebuild ---'; printf '%s\n' "$after"   # → both readings printed and diffed BY EYE. The "same next action" half is NOT asserted mechanically and must not be yet: no next-action line is specified anywhere (AC-427 verifies by a CI divergence-report artifact, not by stdout; s7d:55 says doctor prints one rule table), and byte-equality of the whole table would go red on a rebuild timestamp or store count. NEEDS A DECISION on doctor's output contract.
# reproduces the SAME next action from git alone, after dropping tier-2 entirely.
# This is the FIRST time the FSM is replayed at all — F19 proved suspension lossless
# 174/174 and explicitly left "same next action" unproven. Includes the adversarial-store
# variant and the 40-hex corpus-pin assertion.
```

**(3) Deterministic resume across a real kill, at MORE THAN ONE STEP.**

```bash
# taskkill /T /F the running process mid-graph, then re-invoke the SAME idempotent entry — there is no
# --resume flag and no resume verb (ADR-0107; AC-013/AC-186/AC-264/AC-400). The previous revision of this line
# invoked `redkiln run --resume <run-id>`, a flag four criteria deny (PIA-030); that it is refused is itself
# AC-400's behavioural check (s7d's checkpoint).
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (3) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the same item the killed run was on"; exit 1; }
./target/debug/redkiln run --lane fix --item "$ITEM" || { echo "FAIL: the re-invocation exited non-zero — there is no --resume flag and none is added here (ADR-0107; AC-013/AC-186/AC-264/AC-400)"; exit 1; }
out=$(./target/debug/redkiln explain --item "$ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }   # the run id is read back from the event log (AC-013)
test -n "$out" || { echo "FAIL: empty event log — two empty outputs compare equal, so this may never be the comparison's floor"; exit 1; }
printf '%s\n' "$out"   # → diff this against the PRE-KILL `explain --item` capture. That capture is taken nowhere in this block: taking it before the taskkill is owed, and no step-output digest field is named in any ADR, so the equality is READ, not asserted. NEEDS A DECISION.
# every step-output digest before the kill equals the digest after resume (compare the two `explain` outputs).
# F38 held exactly ONE step at n=3; this criterion is the multi-step case, which is unmeasured — and AC-265
# (the gate answer surviving the kill) is exercised here, not in 7b-ii.
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
test -n "$CLOCK_FIXTURE" || { echo "FAIL: name the clock-read fixture crate in CLOCK_FIXTURE — no ADR or register names it; NEEDS A DECISION"; exit 1; }
cargo metadata --format-version 1 --no-deps | grep -q "\"$CLOCK_FIXTURE\"" || { echo "FAIL: $CLOCK_FIXTURE is not a workspace member — a missing fixture must never read as the pass"; exit 1; }
cargo clippy -p "$CLOCK_FIXTURE"; rc=$?; [ "$rc" -ne 0 ] || { echo "FAIL: the clock-read fixture PASSED the lint — the fixture FAILING it is the pass (PIA-031)"; exit 1; }
```

**(5) The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion.

```bash
test -d crates/rk-kernel || { echo "FAIL: rk-kernel absent — criterion (5) NOT RUN (Halt 8)"; exit 1; }
out=$(cargo test -p rk-kernel -- trybuild 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the trybuild filter matched 0 tests — an unwritten --stay fixture reads exactly like a passing one"; exit 1; }
```

**(6) `redkiln lint --facet-stability` (AC-071) passes**, which means one of exactly two things and nothing else:

```bash
# [WILL EXIST] — and so does its argument: there is NO .redkiln/ directory in redkiln-rs today
# (verified 2026-09-11). The facet registry that exists now is spikes/p9/facets.toml, which STEP 7e's
# pack authoring derives .redkiln/facets.toml from (PIA-011). Both the binary and the file are Phase-1 output.
./target/debug/redkiln lint --facet-stability .redkiln/facets.toml \
                            --stability-record <path>
# exit 0 only when EITHER zero facets are declared source = "llm",
# OR a stability record satisfies ALL FIVE clauses:
#   modal_required_set_identity >= 0.80  AND  n_items >= 20  AND  draws_per_item >= 10
#   AND panel_verdict == "scored"        AND  corpus_sha == <catalog corpus pin>
#
# panel_verdict == "scored" IS NOW SATISFIABLE AND, FOR ARM D, SATISFIED — F39 scored it on 2026-09-09
# over 140 truth cells at 47faec9f (all scored after the 2026-09-11 panel-file key fix; the 2026-09-09 run scored 136 — F39-DRAFT's Correction).
# THE OTHER FOUR ARE UNTOUCHED, and identity measured 0.25 / 0.15 / 0.10 / 0.10 against 0.80
# [measured, F36]. A discharged clause is not a partial unblocking: the gate still fails on the second
# branch, and the fixture set stays at FIVE. After Step 4a the FIRST branch is what passes — zero llm facets
# (ADR-0090; DECISIONS-2026-09-11 #2) — which is the expected outcome.
```

**(6, second lint) `redkiln lint --facet-provenance` (AC-520) passes** — the derived-facet sibling, new since F39,
which gates the half of the registry AC-071 never reached. **This is not an eighth exit criterion**: design §10.2
names seven and this runbook does not add to them. It is stated here because AC-520 is the criterion that makes
(6)'s neighbouring half checkable, and a Phase 1 that exits on (6) with `--facet-provenance` unbuilt has left the
measurably-wrong half of the registry ungated — which is the state F39 found:

```bash
# [WILL EXIST] — Step 4a authors the registry half, Step 7b builds the lint, Step 7e derives the file.
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml
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

**Checkpoint 8.** All seven pass (with (1)'s two halves and (6)'s two lints), and the following remain true and
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

Those sixteen are not read, they are run. Checkpoint 8's mechanical lines, in bullet order:

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# Guards first: an absent or empty tree answers every prohibition below for free (Halt 8).
test -n "$(ls -A crates)" || { echo "FAIL: crates/ empty — Checkpoint 8's crates/ checks NOT RUN (Halt 8)"; exit 1; }
test -f .redkiln/facets.toml && test -d .redkiln/packs && test -d .redkiln/catalog || { echo "FAIL: .redkiln/ not derived — Checkpoint 8's registry and pack checks NOT RUN (Halt 8)"; exit 1; }

# no require row keyed on an LLM-emitted facet, and after Step 4a no llm facet at all
grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' crates/ .redkiln/ --include=*.rs --include=*.toml; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: an LLM-emitted facet keys a require row, or grep errored (rc=$rc)"; exit 1; }
test "$(grep -c '^source = "llm"' .redkiln/facets.toml)" = 0 || { echo "FAIL: a facet still declares source = \"llm\" (ADR-0090; DECISIONS-2026-09-11 #2)"; exit 1; }

# stale_policy recorded per cost class, never defaulted from P2 (decision 5; Step 7e; warrant F40 §3)
set=$(grep -rh 'stale_policy' .redkiln/packs --include=*.toml); test -n "$set" || { echo "FAIL: no stale_policy written in any pack — decision 5 was defaulted, not recorded"; exit 1; }
printf '%s\n' "$set" | grep -v 'rematerialise'; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a stale_policy is not rematerialise (rc=$rc)"; exit 1; }

# no require row on a declared or NOT MEASURABLE facet (ADR-0093)
grep -rnE 'require.*\b(surfaces|data_migration|recent_deploy)\b' .redkiln/packs .redkiln/catalog; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a declared or NOT MEASURABLE facet keys a require row (rc=$rc)"; exit 1; }

# bisect's ceiling on the SHIPPED catalog — spikes/p9's copy is already green, so it may not be the path (PIA-035)
grep -q 'max_rigor *= *"recommended"' .redkiln/catalog/bisect.toml || { echo "FAIL: bisect has no max_rigor = \"recommended\" ceiling (P7-CEIL; DECISIONS-2026-09-11 #1)"; exit 1; }
out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the rigor_monotonicity filter matched 0 tests"; exit 1; }

# ChangeSummary nowhere in crates/ but its rk-ports newtype (ADR-0093; S-BIN unrun — the BrittonGames fixture is not the spike)
out=$(grep -rn 'ChangeSummary' crates/ --include=*.rs | grep -v 'crates/rk-ports/'); test -z "$out" || { echo "FAIL: ChangeSummary outside rk-ports:"; printf '%s\n' "$out"; exit 1; }

# card append route never defaulted on; no quality-shorthand default or terminal condition
grep -rnE 'append(_route)? *= *true' crates/ .redkiln/ --include=*.rs --include=*.toml; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a card append route is enabled by default (rc=$rc)"; exit 1; }
grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *=' crates/ .redkiln/; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a quality-shorthand default or terminal condition (rc=$rc)"; exit 1; }

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
```

---
