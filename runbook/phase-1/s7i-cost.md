# Step 7i — `redkiln cost`: the reading journalled, and ADR-0110's three renderings (Phase 1, part 7)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7i` | Phase 1 | Checkpoint 7h | Checkpoint 7i | `s7h` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Phase-1 exit criterion (7) invokes `redkiln cost <item>`, and Step 8 did not build it —
deliberately, and recorded why. Two blockers, both measured 2026-09-21: there were **zero `dispatch_completed`
events** on the tree because no run completed, and, underneath that, **ADR-0110's batch delta is defined over
*"readings sharing one `resetsAt`"* and nothing journals a `resetsAt`.** `EventKind::RateLimitReading` exists with
the wire name `rate_limit_reading` and has **no producer anywhere**; `DispatchRecord`'s `headroom_before` /
`headroom_after` are `WindowReading { five_hour, seven_day }`, which has no such field.

The first blocker is cleared by `s7h`. ***Corrected 2026-09-23 at this stage's open, measured at `76ae138`: it is
NOT.*** *`s7h` made the lane advance and made a **fixture** dispatch; the shipped store still holds **zero**
`dispatch_completed` rows (`t2_event`: `dag_assembled` 1, `reconcile_started` 3, `stage_transition` 2, read
`mode=ro`), and **0 of 102** shipped activities complete a committing walk — `PROGRESS.md` §1 row 38 says so in terms
(*"`s7i`'s first blocker is NOT cleared: the lane advances and nothing dispatches"*). The dispatched case is therefore
proved over a fixture corpus, `ADR-0162`'s route, and the shipped tree renders only the empty case (ADR-0163 decision
6).* The second is cleared here, and **not by amending ADR-0110**: the reading is
already parsed and thrown away. `rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a
`RateLimitReading` carrying `resets_at`, `five_hour_resets_at` and `seven_day_resets_at`, and
`ClaudeHarness::last_reading()` exposes it. Redefining the delta over "readings in one run" would be a different
measure wearing the same name — F6's quantisation of `utilization` to 0.01 is precisely why the `resetsAt` grouping
exists.

**The calculator is already complete and has no caller** *(at `76ae138`; `cost` became its first `src/` caller at `e0802a5`, and the rendering strings below are now output there)*. `throttle::BatchDraw` carries `first`, `last`,
`delta_ticks`, `n`, `resolution_limited`, `low_sample` and `non_monotonic`; `throttle::attribute` returns
`Attributable(Vec<BatchDraw>)` or `NotAttributable { interleaving }`; `LOW_SAMPLE_BELOW` is 10 and `delta_ticks` is
`None` when `low_sample`. Every one of ADR-0110's rules is encoded. What is missing is a **producer** for the
samples and a **renderer** for the draws — measured 2026-09-21, the strings `not attributable`, `low_sample` and
`resolution_limited` appear in **no `.rs` file as output**.

**This step takes no decision**: `s7f` §7f.3 names the producer and §7f.4 settles RT-19's unit. ***Corrected
2026-09-23 at this stage's open: it takes one record, `ADR-0163`***, *and this is the third consecutive stage whose file
said this sentence and was wrong (`s7g` took eighteen — `0134`–`0151`, all dated 2026-09-22; `s7h`'s own row says seventeen and is one short — `s7h` eleven). Building the verb surfaced what `ADR-0133` did
not settle: `cli.rs` says a ninth verb *"is a decision, taken in a record"*; nothing on the journal says which item a
reading belongs to, and ADR-0110's `not attributable` is computed from exactly that; `ADR-0133` clause 2's "once per
reading" would make `n` count frames where `BatchDraw.n` counts dispatches; and clause 3's `<RFC 3339 | null>` is not
expressible — `Timestamp` is a transparent `i64` and no crate formats RFC 3339. `ADR-0163` supersedes clauses 2 and 3 in
part, on the operator's decisions of 2026-09-23, and — on the operator's decision of 2026-09-24, when the adversarial pass found the conflict — `ADR-0131`'s first forbids bullet for this row's `item` alone, as an interim member until `0131`'s tag is wired. `ERRATA` E-0133-2 … E-0133-5 record four defects in `ADR-0133`'s own text, and E-0133-6 corrects two of those four.*

### 7i.1 The producer — the reading stops being discarded

The Claude adapter journals a `rate_limit_reading` row per reading it parses, with the payload fields `s7f` §7f.3's
record names, against design §5's event table. ***Amended 2026-09-23 by `ADR-0163` decisions 2–3:*** *one row **per
dispatch** that went through the child, from the stream's last representable reading, before that dispatch's
`dispatch_completed` row — so `n` counts dispatches — carrying `windows` (`null` for a flat reading), `utilization`
(flat only), `resets_at` per window as **integer unix seconds**, `rate_limit_type`, `item`, copied from the
`"item"` input `verbs/run.rs` already binds, and `cli_version`, which AC-227 requires of every harness event.* The kind is **tier 2** and the kernel's fold already classifies it as
non-state-moving in three places (`breaker`, `fsm`, `memo`), so no tier-1 projection moves and **criterion (2)
cannot regress** — which Checkpoint 7i asserts rather than assumes.

🛑 **A reading is a measurement, not a derivation.** The row carries what the platform reported, verbatim: no
extrapolation from one utilisation to two windows (AC-374's *"no extrapolation"*, which `RateLimitReading::windows`
already enforces by returning `None` for the flat shape), and no synthesised `resetsAt` when the frame carried none.

### 7i.2 The verb — the three renderings and nothing else

*(dated 2026-09-29, `s8f` build, package S8F-P7-TELEMETRY-COST-DOCTOR-JOURNAL; `s8f` §8f.10's `s7i` row and §8f.8;
the telemetry-key record ADR-0190 decision 4, `proposed` *(dated 2026-10-02, `s8g` close: ADR-0190 accepted on D21-130)*.)* **"Nothing else" no longer holds for the item that owns
`dispatch_completed` rows.** The adapter now keys its `dispatch_completed` row with the spec's `item` beside the event
(ADR-0182's carrier) and writes `activity_id_at_version` in its payload, so `cost <item>` prints AC-395's per-activity
breakdown after the lines below: a `breakdown <item>: dispatch_completed rows owned: <k>` header and one `activity <A>:
…` line per activity (ADR-0190 decision 4's forms 9 and 10). An item owning neither a reading nor a
`dispatch_completed` row still prints exactly the one empty line, and no line anywhere carries a headroom figure
(AC-373). The table below and the renderings it names are unchanged. Checkpoint 7i (5)'s `GRAMMAR` and its
owns-no-reading branch are amended by the same dated edit.

`redkiln cost <item>` reads the item's `dispatch_completed` rows and its `rate_limit_reading` rows, builds the
`Sample` sequence, and renders through `throttle::attribute`. ***Corrected 2026-09-23 (`ADR-0163` decision 4):***
*"the item's `dispatch_completed` rows" is unimplementable — `DispatchRecord` carries no `item` and the store's selector
is inert. `cost` reads the whole journal, owns a reading by its `item` member, takes the span from the item's first
owned reading to its last, and counts interleaving over the **traffic** in it — each foreign kernel `dispatch` row once, and each foreign or
unowned reading once unless it directly follows a foreign `dispatch` row, when the pair counts once — because `attribute` counts only the samples it is given. The span also renders
`non_monotonic`, a window with `unkeyed` readings (a `null` `resets_at`), and `no sample`; none quotes a delta.*
ADR-0110's three, and the data condition of each:

| rendering | condition |
| --- | --- |
| `not attributable`, **with the interleaving count** | ~~any sample in the window is not the account's own~~ *(corrected 2026-09-24, ADR-0163 decision 4)* any foreign or `null`-item `dispatch` row or reading between the item's first and last owned reading — span-wide, not per window, and against the **item**'s own |
| `low_sample`, **with no delta quoted** | `n < 10` |
| the batch delta in **integer ticks**, with `n` and `resolution_limited` | otherwise; `resolution_limited` when the raw delta is ≤ 1 |

🛑 **No per-run headroom figure, ever.** `utilization` is quantised to 0.01, so per-run headroom is **not
measurable** (F6) — AC-373. A run that reports one *"has laundered an unmeasurable into a measurement"*, which is
`s8`'s own sentence for it.

🛑 **A vacuous rendering is not a pass.** With no dispatches an honest verb reports that there is nothing to
attribute; it does **not** print a `low_sample` line over `n = 0` and let criterion (7)'s grep read it as green.
Step 8 declined to build the verb rather than ship that shape, and this step may not reintroduce it: Checkpoint 7i
asserts the empty case is distinguishable from the sampled one.

### 7i.3 Checkpoint 7i

- **A `rate_limit_reading` row is journalled** — once per dispatch, before `dispatch_completed`, on a failed dispatch
  too — and carries `resets_at` per window as an integer, so ADR-0110's grouping key exists.
- **`redkiln cost <item>` exists and exits 0** on an item with dispatches, rendering exactly one of the three.
- **The empty case is distinguishable** — an item with no dispatches does not render a `low_sample` delta.
- **No per-run headroom figure appears** (AC-373).
- **Criterion (2) has not regressed** — a tier-2 row added per reading moves no tier-1 projection.

🛑 **HARDENED 2026-09-23 AT THIS STAGE'S OPEN, BEFORE ITS FIRST RUN (PIA-035), AND THE REASONS ARE RECORDED RATHER THAN
SMOOTHED.** As authored the block was weak in four places. It had **no build step** over a gitignored `target/` — the
defect Checkpoint 7f shipped with. Its criterion (2) line asserted `rc` only, omitted `--test it`, and grepped no passed
count, so a filter matching nothing would have been a green. Its "item with dispatches" bullet named `$FIX_ITEM` as
*"an item whose lane ran and dispatched"*, and **no item on this tree has** — the shipped store holds zero
`dispatch_completed` rows, so the bullet could only ever read the empty case, fail its own grep, and invite a rewrite
until the empty case passed it. And its empty-case line asserted only the **absence** of `low_sample`, which a `cost`
that printed nothing, or printed an error and exited 0, would satisfy — **a vacuous green indistinguishable from the one
§7i.2 forbids in terms**. So the dispatched case is now asserted **behaviourally**, by named tests that run the real
binary over a fixture corpus (ADR-0163 decision 6, `ADR-0162`'s route); the empty case is asserted **positively** and
on the **shipped** store; and every grep over source is a floor beneath a behavioural assertion, never the assertion.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
#
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status: under build-lock contention cargo exits 0 having run
# ZERO tests. Every selection is `--test it`, MODULE-QUALIFIED and `--exact` — a bare name under `--exact` runs zero
# tests and exits 0 (measured at `s7h` guard (10a)) — and asserts each test's own `... ok` line AND an exact count.

# ---- (0) THE WORKTREE PATH SET, read FIRST — before any cargo run and before any prune. Counts cannot tell a leaked
# ---- registration from a replaced one; paths can.
wt_before=$(git worktree list --porcelain | grep '^worktree ' | sort)
test -n "$wt_before" || { echo "FAIL: git worktree list printed no worktree — the path-set guard would compare two empty sets"; exit 1; }

# ---- (1) BUILD-OR-FAIL. A stale binary has no `cost` verb, and its usage error must not be read as a rendering.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 7i NOT RUN (Halt 8)"; printf '%s\n' "$out" | tail -20; exit 1; }
RK=""
for cand in ./target/debug/redkiln ./target/debug/redkiln.exe; do [ -f "$cand" ] && RK="$cand"; done
[ -n "$RK" ] || { echo "FAIL: the build succeeded and no redkiln binary exists — every assertion below would be about nothing"; exit 1; }

# ---- (2) A rate_limit_reading ROW IS JOURNALLED: once per dispatch, before dispatch_completed, with INTEGER resets_at
# ---- per window, on a failed dispatch too, with the item copied or null — each asserted by name (ADR-0163 2–3).
A_TESTS="reading::rate_limit_reading_row_carries_integer_resets_at_per_window reading::rate_limit_reading_flat_shape_carries_no_windows reading::rate_limit_reading_absent_writes_no_row reading::rate_limit_reading_refused_reading_writes_no_row reading::rate_limit_reading_is_journalled_on_a_failed_dispatch reading::rate_limit_reading_item_is_copied_from_spec_inputs_or_null reading::rate_limit_reading_once_per_dispatch_carries_the_last reading::ac368_reading_append_failure_is_warning reading::rate_limit_reading_type_absent_is_null dispatch::ac360_one_event_per_termination_path"
out=$(cargo test -p rk-adapter-claude --test it -- $A_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the producer's selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $A_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 10 passed; 0 failed' || { echo "FAIL: the producer selection did not run exactly 10 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# ---- (3) `cost` EXISTS AND EXITS 0 ON AN ITEM WITH DISPATCHES — BEHAVIOURALLY. The shipped store holds ZERO
# ---- dispatch_completed rows, so on this tree the bullet is met only by a real `redkiln run` over the fixture
# ---- transcript followed by a real `redkiln cost` (the first test below), and by the seeded corpus the binary renders
# ---- byte-for-byte (the second). The corpus must exist and be byte-pinned BEFORE the walker is read: a table-driven
# ---- test over a missing directory asserts nothing.
test -f crates/rk-cli/tests/fixtures/cost/run-frames.jsonl || { echo "FAIL: the cost run transcript is absent"; exit 1; }
test -d crates/rk-cli/tests/fixtures/cost/interleaved || { echo "FAIL: the cost fixture corpus is absent — the walker would assert over nothing"; exit 1; }
grep -qxF 'crates/rk-cli/tests/fixtures/cost/** -text' .gitattributes || { echo "FAIL: no -text row pins the cost corpus — autocrlf would smudge expected.stdout"; exit 1; }
drift=$(git status --porcelain -- crates/rk-cli/tests/fixtures/cost); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git status could not read the cost corpus (rc=$rc)"; exit 1; }
[ -z "$drift" ] || { echo "FAIL: the cost corpus differs from its committed bytes:"; printf '%s\n' "$drift"; exit 1; }
C_TESTS="cost::cost_after_a_real_run_renders_low_sample_through_the_binary cost::cost_fixture_corpus_binary_arm cost::ac395_cost_dispatches_nothing_under_a_panicking_harness cost::render_empty_is_nothing_to_attribute_never_low_sample cost::cost_output_grammar_is_closed surface::ac400_help_snapshot_every_verb run::ac438_table_driven_exit_codes_over_fixtures_binary_arm"
out=$(cargo test -p rk-cli --test it -- $C_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the cost selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $C_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 7 passed; 0 failed' || { echo "FAIL: the cost selection did not run exactly 7 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }

# ---- (4) CRITERION (2) HAS NOT REGRESSED — and the adversarial store carries a row of THIS kind, or the bullet would
# ---- be about `Dispatch` noise and not about a reading. The count is a FLOOR beneath the test, not the assertion.
# ⚠️ ANCHORED to the construction line: a bare `grep -c 'EventKind::RateLimitReading'` also counts the test's own
# PREMISE assertion (`.any(|e| e.kind == EventKind::RateLimitReading)`), so it stayed >= 1 with the noise row deleted
# — a floor satisfied by an assertion's text (found by this stage's adversarial pass).
n=$(grep -cxE '[[:space:]]*EventKind::RateLimitReading,[[:space:]]*' crates/rk-kernel/tests/it/projection.rs)
[ "$n" -ge 1 ] || { echo "FAIL: rebuild_differential's adversarial store constructs no rate_limit_reading noise row"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- projection::rebuild_differential --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rebuild_differential is red (rc=$rc) — read the panic below: a moved tier-1 projection is criterion (2) regressing, a PREMISE panic is a missing noise row"; printf '%s\n' "$out" | grep -E 'panicked|PREMISE|left|right' | head -10; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF 'test projection::rebuild_differential ... ok' || { echo "FAIL: rebuild_differential did not report ok"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the criterion (2) selection did not run exactly one test"; exit 1; }

# ---- (5) THE EMPTY CASE IS DISTINGUISHABLE, ON THE SHIPPED STORE, AND `cost` APPENDS NOTHING. Asserted as ONE EXACT
# ---- LINE, not a substring plus a deny-list: the adversarial pass drove a sabotaged binary that appended
# ---- `five_hour: n=0 resets_at=0` (and, separately, a per-run figure) to the empty line, and the substring form
# ---- passed both. `mode=ro` and NOT `immutable=1`: immutable ignores the WAL and could under-count.
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no shipped store at $store"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM (rk-fix-doctor-test-count at this pin) — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
# `cost` renders the empty case for ANY string, a misspelling included, so the items must be real.
for it in rk-story-codex-adapter "$FIX_ITEM"; do
  test -f ".redkiln/items/$it.toml" || { echo "FAIL: .redkiln/items/$it.toml does not exist — the empty case would be asserted over a misspelling"; exit 1; }
done
probe='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); t2=c.execute("SELECT COUNT(*) FROM t2_event").fetchone()[0]; rl=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",("rate_limit_reading",))]; print(t2,len(rl),sum(1 for r in rl if r.get("item")==sys.argv[2]))'
# (dated 2026-09-29, s8f build, package S8F-P7; s8f §8f.10's s7i row, ADR-0190 decisions 3 and 4, proposed) How many
# `dispatch_completed` rows the item OWNS: the row's ADR-0182 key beside the event, the `t2_event.item` column s8e's
# 0002 migration added — never a payload member (the row carries none). `mode=ro`, as above.
dc_probe='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(c.execute("SELECT COUNT(*) FROM t2_event WHERE kind=? AND item=?",("dispatch_completed",sys.argv[2])).fetchone()[0])'
# The CLOSED grammar of every line `cost` prints (ADR-0163 decision 4's eight forms; `crates/rk-cli/src/verbs/cost.rs`'s
# module doc). ⚠️ One gap against the Rust matcher, stated: `[^[:space:]]` in Git Bash does not class U+00A0 as space,
# so an item carrying a no-break space would pass here and not in `in_grammar`. Unreachable in this block — both items
# it renders are ASCII ids checked against `.redkiln/items/` — and recorded rather than claimed away.
# Draws at n < 10 carry low_sample and NO delta; draws at n >= 10 carry no low_sample, and resolution_limited exactly
# when the delta is 0 or 1 — what `throttle::batch_draw` can produce, and nothing else.
EMPTY_TAIL=' nothing to attribute — no rate_limit_reading row in this store names this item \([0-9]+ dispatch rows\)$'
EMPTY_RE='^cost [^[:space:]]+:'"$EMPTY_TAIL"
GRAMMAR="$EMPTY_RE"'|^cost [^[:space:]]+: rate_limit_reading rows in span: [0-9]+, this store'"'"'s traffic only$|^not attributable interleaving=[1-9][0-9]*$|^(five_hour|seven_day): n=[1-9] resets_at=-?[0-9]+ low_sample( non_monotonic| resolution_limited)?$|^(five_hour|seven_day): n=[1-9][0-9]+ resets_at=-?[0-9]+( non_monotonic| delta_ticks=[01] resolution_limited| delta_ticks=([2-9]|[1-9][0-9]+))$|^(five_hour|seven_day): (unkeyed=[1-9][0-9]*|no sample)$|^unwindowed=[1-9][0-9]*$'
# (dated 2026-09-29, s8f build, package S8F-P7; s8f §8f.10's s7i row) The GRAMMAR WIDENS to ADR-0190 decision 4's two
# breakdown forms (proposed), the forms `cost_output_grammar_is_closed` holds beside ADR-0163's eight: the header, and
# one line per activity whose sums are `<sum>`, `<sum>(absent=<k>)` or `absent`, the cost a decimal. SELF-CONTROLLED
# first: one line of each form matches, and a zero-owned header, an id without its `@version`, an absence written as a
# `(absent=0)` and a headroom figure do not.
BS='([0-9]+|[0-9]+\(absent=[1-9][0-9]*\)|absent)'
BD='([0-9]+(\.[0-9]+)?|[0-9]+(\.[0-9]+)?\(absent=[1-9][0-9]*\)|absent)'
BREAKDOWN_RE='^breakdown [^[:space:]]+: dispatch_completed rows owned: [1-9][0-9]*$|^activity ([^[:space:]@]+@[^[:space:]]+|\(none\)): dispatches=[1-9][0-9]* input_tokens='"$BS"' output_tokens='"$BS"' cache_read_input_tokens='"$BS"' cache_creation_input_tokens='"$BS"' wall_ms=[0-9]+ total_cost_usd='"$BD"'$'
for good in 'breakdown rk-x: dispatch_completed rows owned: 2' 'activity emit-report@1: dispatches=2 input_tokens=11 output_tokens=1(absent=1) cache_read_input_tokens=absent cache_creation_input_tokens=0 wall_ms=100 total_cost_usd=0.0040' 'activity (none): dispatches=1 input_tokens=1 output_tokens=1 cache_read_input_tokens=1 cache_creation_input_tokens=1 wall_ms=5 total_cost_usd=12(absent=1)'; do
  printf '%s\n' "$good" | grep -qE "$BREAKDOWN_RE" || { echo "FAIL (control): the breakdown grammar rejects a grammatical line: $good"; exit 1; }
done
for bad in 'breakdown rk-x: dispatch_completed rows owned: 0' 'activity emit-report: dispatches=1 input_tokens=1 output_tokens=1 cache_read_input_tokens=1 cache_creation_input_tokens=1 wall_ms=1 total_cost_usd=1' 'activity a@1: dispatches=1 input_tokens=1(absent=0) output_tokens=1 cache_read_input_tokens=1 cache_creation_input_tokens=1 wall_ms=1 total_cost_usd=1' 'activity a@1: dispatches=1 input_tokens=1 output_tokens=1 cache_read_input_tokens=1 cache_creation_input_tokens=1 wall_ms=1 total_cost_usd=1 headroom_five_hour=3'; do
  if printf '%s\n' "$bad" | grep -qE "$BREAKDOWN_RE"; then echo "FAIL (control): the breakdown grammar accepts an ungrammatical line: $bad"; exit 1; fi
done
GRAMMAR="$GRAMMAR|$BREAKDOWN_RE"
pre=$(python -c "$probe" "$store" rk-story-codex-adapter); rc=$?
[ "$rc" -eq 0 ] && [ -n "$pre" ] || { echo "FAIL: the read-only store probe did not run (rc=$rc)"; exit 1; }
read -r t2_before _ owned <<< "$pre"
[ "$owned" = "0" ] || { echo "FAIL (PRECONDITION): rk-story-codex-adapter owns $owned readings — re-point the empty case at an item that owns none; this guard does not change case silently"; exit 1; }
empty=$("$RK" cost rk-story-codex-adapter 2>&1); erc=$?
# ⚠️ Git Bash's grep ignores a trailing CR, so a CRLF-terminated line would pass every regex below (found by this
# stage's second adversarial round). The CR cannot be refused from `$empty` itself: Git Bash strips it inside `$(…)`,
# and `grep -q $'\r'` also matched nothing on a CRLF line when probed. So the RAW bytes are read through `od -c`.
if "$RK" cost rk-story-codex-adapter 2>&1 | od -c | grep -qF '\r'; then echo "FAIL: cost's empty output carries a CR byte"; exit 1; fi
[ "$erc" -eq 0 ] || { echo "FAIL: cost exited $erc on an item with no readings"; printf '%s\n' "$empty"; exit 1; }
test -n "$empty" || { echo "FAIL: cost printed NOTHING — silence is not the empty rendering (Halt 8)"; exit 1; }
[ "$(printf '%s\n' "$empty" | grep -c .)" -eq 1 ] || { echo "FAIL: the empty case printed more than one line — anything beside the empty line is a rendering over n = 0"; printf '%s\n' "$empty"; exit 1; }
printf '%s\n' "$empty" | grep -qE "^cost rk-story-codex-adapter:$EMPTY_TAIL" || { echo "FAIL: the empty case is not the one empty line (§7i.2's vacuous green)"; printf '%s\n' "$empty"; exit 1; }
fx=$(python -c "$probe" "$store" "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$fx" ] || { echo "FAIL: the store probe did not run for $FIX_ITEM (rc=$rc)"; exit 1; }
read -r _ _ fx_owned <<< "$fx"
fx_dc=$(python -c "$dc_probe" "$store" "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$fx_dc" ] || { echo "FAIL: the dispatch_completed ownership probe did not run for $FIX_ITEM (rc=$rc)"; exit 1; }
fout=$("$RK" cost "$FIX_ITEM" 2>&1); frc=$?
if "$RK" cost "$FIX_ITEM" 2>&1 | od -c | grep -qF '\r'; then echo "FAIL: cost $FIX_ITEM's output carries a CR byte"; exit 1; fi
[ "$frc" -eq 0 ] || { echo "FAIL: cost $FIX_ITEM exited $frc"; printf '%s\n' "$fout"; exit 1; }
test -n "$fout" || { echo "FAIL: cost $FIX_ITEM printed nothing"; exit 1; }
# EVERY line against the closed grammar — not only window lines: a per-run figure on its own line or in the header is
# what the grammar exists to refuse. ⚠️ At this pin the shipped store holds ZERO readings, so `$FIX_ITEM` renders the
# empty case too and the sampled branch of this grammar has never run against the SHIPPED store; the sampled case is
# asserted by (3) over the fixture corpus, whose `cost_output_grammar_is_closed` applies the same forms to every line.
bad=$(printf '%s\n' "$fout" | grep -vE "$GRAMMAR" | grep .)
[ -z "$bad" ] || { echo "FAIL: a line outside ADR-0163's closed grammar (AC-373):"; printf '%s\n' "$bad"; exit 1; }
# (dated 2026-09-29, s8f build, package S8F-P7; s8f §8f.10's s7i row) ~~`if [ "$fx_owned" = "0" ]; then`~~ — after
# ADR-0190 decision 4 an item that owns `dispatch_completed` rows and no reading prints the empty line AND a breakdown, so
# the one-line assertion holds only when the item owns NEITHER. The widened GRAMMAR above still checks every line.
if [ "$fx_owned" = "0" ] && [ "$fx_dc" = "0" ]; then
  [ "$(printf '%s\n' "$fout" | grep -c .)" -eq 1 ] && printf '%s\n' "$fout" | grep -qE "^cost $FIX_ITEM:$EMPTY_TAIL" || { echo "FAIL: $FIX_ITEM owns no reading and cost does not print exactly the empty line"; printf '%s\n' "$fout"; exit 1; }
fi
post=$(python -c "$probe" "$store" rk-story-codex-adapter); rc=$?
[ "$rc" -eq 0 ] && [ -n "$post" ] || { echo "FAIL: the post-run store probe did not run (rc=$rc)"; exit 1; }
read -r t2_after _ _ <<< "$post"
[ "$t2_after" = "$t2_before" ] || { echo "FAIL: cost appended to the store ($t2_before -> $t2_after rows) — a read that writes, or a dispatch (AC-395)"; exit 1; }
# ⚠️ This counts `t2_event` rows only. Opening the store applies its schema and, on a repository with none, creates
# and identity-stamps one (ADR-0163 decision 1) — not a journal append, and not what this line asserts.

# ---- (6) NO PER-RUN HEADROOM FIGURE (AC-373). The assertion is BEHAVIOURAL — (3)'s closed-grammar test and (5)'s
# ---- GRAMMAR. This grep is a FLOOR over CODE lines only, anchored to the `path:LINE:` prefix grep prints and dropping
# ---- `//`, `/*` and doc-continuation `* ` leaders. A bare leading `*` is NOT dropped: `*run_headroom_cost.lock() = 3;`
# ---- is a deref on a code line, and the first cut of this regex hid it (found by this stage's adversarial pass).
# ---- ⚠️ IT STILL CANNOT SEE three CODE shapes, measured by the second round: a continuation line of a multiplication
# ---- as rustfmt writes one (`        * headroom_for_run`, the default `binop_separator = Front`), code after a closing
# ---- `*/` on the same line, and code after an opening `/* … */` on the same line. The floor is NECESSARY-not-
# ---- sufficient for those as for renames; the closed-grammar test in (3) is what makes the pair sound.
# ---- ⚠️ WHAT IT CANNOT SEE, said rather than claimed away: a RENAMED identifier — the AC register's own words, "a grep
# ---- rule is defeated by renaming an identifier" — and one assembled by a macro or `concat!`. A mention inside a
# ---- string or `#[doc = "…"]` is a code line to this grep and FAILS it, which is the safe direction.
raw=$(grep -rnE 'headroom_for_run|run_headroom_cost' crates/*/src --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the AC-373 floor could not run (grep rc=$rc)"; exit 1; }
code=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | grep .)
[ -z "$code" ] || { echo "FAIL: a per-run headroom identifier on a CODE line (AC-373):"; printf '%s\n' "$code"; exit 1; }

# ---- (7) THE WORKTREE PATH SET IS UNCHANGED, compared BEFORE any prune; the prune is a dry run and must find nothing.
wt_after=$(git worktree list --porcelain | grep '^worktree ' | sort)
[ "$wt_before" = "$wt_after" ] || { echo "FAIL: this block changed the worktree PATH SET:"; diff <(printf '%s\n' "$wt_before") <(printf '%s\n' "$wt_after"); exit 1; }
stale=$(git worktree prune --dry-run -v 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: git worktree prune --dry-run could not run (rc=$rc)"; exit 1; }
[ -z "$stale" ] || { echo "FAIL: stale worktree registrations:"; printf '%s\n' "$stale"; exit 1; }

echo "CP7I-END-OF-BLOCK reached"
```

---
