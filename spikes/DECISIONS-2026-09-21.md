# Decisions taken at Step 8's close, Step 7f's open and close, and Step 7g's open and build — 2026-09-21 … 2026-09-25 (D21-1 … D21-57), and what they license

*(Title corrected 2026-09-25 at `s8b`'s refuting round 6: it read **"2026-09-21 … 2026-09-24 (D21-1 … D21-39)"** while the file carried **D21-57** — eighteen entries and one date stale. `D21-40` … `D21-53` were appended without moving it, and so were `s8b`'s `D21-54` … `D21-56`; `D21-57` is appended with this stage and moves it, per the note below. The stage list at the head of the title still names only the stages through `s7g`; it is left as written, because this correction moves only the range and the date span.)*

*(Title corrected 2026-09-24 at `s7i`'s close: it read **"2026-09-21 and 2026-09-22 (D21-1 … D21-22)"** while the file carried **D21-38** — sixteen entries and two dates stale, the third time this heading has gone stale by the mechanism its own note below describes, and despite that note's instruction to whoever appended `D21-15`. Moved with **D21-39**, in the same commit.)*

*(Title corrected 2026-09-22 at `s7g`'s open: it read "**Step 8's close and Step 7f's open, 2026-09-21 (D21-1 … D21-6)**"
while the file already carried **D21-7** and **D21-8** — `s7f`'s two flip lines, appended at its close on 2026-09-21 —
so the heading had been one stage and two entries stale since that commit. It is now also two dates: the `D21` id space
stays open across 2026-09-22's entries rather than opening a `D22` space, because each dated line carries its own
date and the space is numbered by where it was opened, not by when it is next appended to.)*

*(Corrected again 2026-09-22 at `s7g`'s build, when **D21-14** was appended — and the recurrence is recorded rather
than smoothed, because it is the same failure the correction above had just repaired. That correction wrote
"**D21-1 … D21-13**" and "**2026-09-22's five entries**"; appending a sixth entry two commits later made both stale,
by exactly the mechanism the first note describes. **A heading that enumerates a range is stale the moment the range
grows, and this file has now demonstrated that twice in one day — the second time within an hour of writing the
first.** The count of entries is no longer restated above; the range is kept because `ADR-0126`'s route cites lines by
`D21-n` and a reader needs to know the space is contiguous. **Whoever appends `D21-15` moves this heading in the same
commit**, and that instruction is here rather than in a commit message because a commit message is not read by the
next author.)*

Same shape as `DECISIONS-2026-09-20.md`. The entries below were taken **by the operator** (Ryan Britton) with the
measured state of the tree in front of him, each **before** the thing it authorises was executed. Grounding is a
document clause or a measured number, never taste.

**Numbering.** This carries a **`D21`-id** and opens a new space: `runbook/PROGRESS.md` §2 holds no `D21` row before
this file. Cite as `DECISIONS-2026-09-21 D21-n`. The `RT-` and `O-` spaces are untouched and continue from `RT-59`
and `O-16` wherever they are next opened. **The `R-` space is opened here**: D21-6 registers **R-64**, the
highest `R-` id in the tree before it being `R-63` [measured 2026-09-21].

**Precedence, stated once.** ADR-0092's: accepted ADRs > the AC register > the SM register > spike files > the
runbook.

**File form.** Pure **LF, no BOM** — the majority convention across this set (6 of the 8 files that preceded it).
`DECISIONS-2026-09-20.md` is CRLF-with-BOM and is the outlier; it is not the model.

---

## D21-1 · AC-059 reads the NON-OPTIONAL subgraph, and the register is amended to say so

**Taken 2026-09-21.** `requires_cycle_lint` built a cycle edge for **every** requirement without reading
`Requirement::optional`, a field the type has carried since it was authored. The lint is repaired to skip optional
edges, and **AC-059's GIVEN is amended in place** — D15-2's form — from *"a catalog whose `requires` relation
contains a cycle"* to the same qualified by *"in its NON-OPTIONAL subgraph"*, with the reasoning written beside it.
No instrument is added and the THEN is untouched.

**Grounding, measured rather than asserted.** AC-059's **own evidence line** already cited F35's *hard cycles **0**
across the enumerated lattice* — so the criterion was always about hard cycles and the lint asserted more than the
rule it names. Measured 2026-09-21 over all **102** activities in the shipped `.redkiln/catalog`: counting only
non-optional edges there is **no cycle anywhere**, reproducing that **0** exactly; counting optional edges the walk
finds `adr-record -> design-doc -> adr-record`, and all **three** mutual pairs — `adr-record`/`design-doc`,
`data-migration-plan`/`design-doc`, `design-doc`/`threat-model` — are **both-optional**.

**It was a live defect, not a tidy-up.** The shipped pack could not be loaded by **any** `redkiln run`, and
`redkiln doctor` passed all **39** rules without seeing it, because no doctor rule covers the selection load path.
Repaired at `9d3c469`, **watched failing first**: `tests/fixtures/catalog-cycle-optional/` — the must-LOAD twin of
`catalog-cycle/`, carrying an optional-only pair and a MIXED pair that pins the reading in between — went red on
its must-LOAD assertion, naming the shipped pair, before a line of `lints.rs` changed.
`ac059_optional_only_requires_cycle_loads` asserts **both** directions, so the 3-node HARD cycle still fails load;
an exemption that also exempted hard edges would be a lint that enumerates nothing (Halt 8).

**What this does not license.** It does not touch AC-078 (the separate runtime acyclicity assertion over an
assembled DAG), and it decides nothing about what a cycle means for a kind that is *not* keyed — ADR-0073's own
*"is not stated in the source, and is not asserted here"* stands.

---

## D21-2 · Step 8's nine operator decisions, and the two criteria recorded NOT SATISFIED

**Taken 2026-09-21, across the stage, each before the thing it authorised ran.** Enumerated so the stage's record
is not only in its commit messages:

1. **`s8`'s `--facet-provenance` line gains `--pinned-corpus`.** It exited **3** as written: `verbs/lint.rs` refuses
   to default that path *in terms*. Checkpoint 7e line 141 carried the identical defect and was fixed the same way.
2. **Four Checkpoint 8 instruments are scoped to what their own bullets assert.** All four were RED on a correct
   tree and every hit was prose — nine comments at line 296 including the ban's own regex in `rigor.rs`, six of
   Step 7e's own `-- DROPPED:` records at 305, 67 mention-grep hits at 313, and at 317 four comments plus two
   `evaluator.rs` string literals that ARE the enforcement. The prose did not give way, because it is pre-existing
   and Halt 7's rule is *"do not widen the guard, and do not write the hit"*.
3. **Criterion (1)'s trailer guard is scoped to `rk-run/<item>`.** `git log --all` was satisfied **with zero runs
   performed** — spike commit `ba46ed9` carries a body line beginning `Slice-Verdict:`.
4. **The requires-cycle lint is repaired** — D21-1.
5. **Criterion (4)'s clock fixture is invoked with its feature.** Without it the must-fail fixture **cannot fail**;
   `tests/fixtures/rk-clock-read/Cargo.toml` spells the required pair verbatim in its own header.
6. **Criterion (1): fill, run for real, record the gap** rather than build the missing lane machinery.
7. **Criterion (1) is recorded NOT SATISFIED** on the `RankTie` that follows the cycle — pack authoring, 7e's, and
   raised rather than taken at Step 8.
8. **Criterion (7) is recorded NOT RUN and the verb is NOT built.** Its only reachable output would have been a
   vacuous `low_sample` green over `n = 0`, which is Halt 8's shape.
9. **`check-checkpoints.py`'s two `s8` allowlist rows are removed** (7e has run; both measured **inert** — the
   checker's baseline is identical without them), and **`.redkiln/telemetry/` is ignored** — it is written by the
   Claude Code redkiln **plugin**, not by this workspace.

**Grounding.** Checkpoint 8 then passed **WHOLE** at `d6160ff`: rc=0, **0 FAIL**, **46** non-comment statements and
**6** `# →` lines, counts re-taken. Every repaired gate carries a **synthetic control** so its exclusions cannot
silently empty it — Checkpoint 5's `syn=` form made permanent. Entry gates re-measured rather than inherited,
including `cargo audit` and `cargo deny`, which Step 7e did **not** re-run: both rc=0.

**What this does not license.** It does not discharge **Checkpoint 5**, whose broader `ChangeSummary` TYPE ban stays
RED with four hits in the two toolchain adapters (7c's debt); 313's narrowing is to Checkpoint 8's own bullet and
says so in place. And it does not license widening 317's pattern to the quoted spelling — three live `expr` rows in
the shipped catalog match it, and whether an `expr` on a `kind = "check"` step carrying `on_false` is the *terminal
condition* ADR-0080 decision 2 forbids is a reading of the record, RAISED and not taken.

---

## D21-3 · Phase 1 is finished as specified: the step executor is built, in four new stages before Step 8

**Taken 2026-09-21, with the measurement in front of him.** `Runtime::run_activity` dispatches an **activation**,
not an activity's **steps**: `grep -rn 'StepKind' crates/rk-kernel/src/runtime/` returns **0**. Across the shipped
catalog **185 of 574** steps are `agent` — the one kind a dispatch approximates — and **113 `tool`** (28
`toolchain:test`), **102 `emit`**, **94 `check`** and **80 `gate`** have no executor; **102 of 102** activities
carry a non-agent step. Consequences, each verified directly: `ports.toolchain`/`tracker`/`docs` have **zero**
non-test call sites; `TrailerKey::Story` and `::SliceVerdict` occur **only in the parser**; `AdvanceStage` is
unreachable because `ctx.successor` is never set; and `rk_kernel::review` (1,179 lines) and `::verify` (731) are
built, tested and unreachable.

**The route: build it.** ADR-0002 gates the cutover **solely** on self-hosting, and an exit narrowed to what is
built would make that milestone a label rather than a measurement. Rejected: re-scoping Phase 1 to exclude lane
execution; narrowing the exit to one stage; re-planning the phase.

**Grounding — it is execution, not design.** design §4.5 *(corrected 2026-09-22: this read §4.4; the `| Step kinds |` row sits under `### 4.5 The catalog v0`, and §4.4 is the seven-step selection function — the decision this paragraph grounds is unaffected)* fixes the five closed step kinds and says `tool` is *"a
toolchain verb through the port"*; design §5's event table names `stage_transition` tier-1 *"(trailer)"* produced by
*"the kernel's FSM at a stage boundary"* and `slice_sealed` by *"the lane's review stage"*; ADR-0105 decision 3
fixes the trailer plumbing; `INDEX.md` puts *"the runtime"* in **Step 7b's** scope. And the kernel already reasons
about step kinds in `budget`, `rigor` and `memo`, with `Step` fully parsed and `predicate::eval` available for the
`check` arm — a missing seam, not missing understanding.

**Three clauses travel with it.**

- **New stages, not a re-opened 7b** (the `7b-ii` precedent): `s7f` → `s7g` → `s7h` → `s7i` → Step 8, landed at
  `8634de8`. Re-opening 7b would retroactively change what Checkpoint 7b's five passes certified.
- **The conformance gate is authored FIRST.** Three source lints over the design's own closed sets, each verified
  RED at `8634de8` and each discharged by the landing that makes it pass, ARMED under the `live.rs` pattern and
  counted **by name** at every checkpoint so a deleted lint cannot read as a discharged one. `s7f` adds a fourth,
  `shipped_pack_loads`, whose absence is the finding itself: every catalog test in this tree runs over a **fixture**
  catalog and nothing ever loaded the 102 activities the product ships.
- **Each landing re-runs, from `s8`'s own bytes, exactly the criteria its work makes reachable**, so a claim of
  progress is a measurement taken there rather than an inference at the end.

**What this does not license.** It does not re-open ADR-0101 clause 5: `s7h` wires the review gate, and **no
document may claim that gate is guarded against false approval** until S-SUBTLE-CB runs. It has not.

---

## D21-4 · `ImplReport` is the sixth toolchain-keyed kind, and that takes a SUPERSESSION

**Taken 2026-09-21.** `redkiln run` refuses the shipped catalog at `RankTie` over `ImplReport` — `hotfix` and
`implement-ue` both provide it at `method_rank = 10` and are simultaneously admissible. The resolution is to declare
`ImplReport` `keyed_by = "toolchain"`, so step-2 keyed expansion resolves it exactly as it already resolves
`Changeset`.

**Grounding, measured.** Of **9** equal-rank producer sets across the 102 activities, **5** are on toolchain-keyed
kinds and are resolved by expansion. **Two of the remaining four are `ImplReport` shadowing `Changeset` exactly** —
identical producer sets at both ranks (`dependency-update`/`hotfix`/`implement-ue`/`test-after` at 10,
`bdd-outside-in`/`walking-skeleton` at 30), because every activity that provides `Changeset` also provides
`ImplReport`. **The other two are not ties at all**: `checklist-review` (`kind:chore`, `kind:docs`) versus
`spike-decision-review` (`kind:spike`), and `fix-spec` (`kind:bug`, `kind:security-vuln`, `kind:incident`) versus
`user-story-ac` (`kind:story`, `kind:project`, `kind:tech-debt`, `kind:docs`) — disjoint by `kind` tag, never
simultaneously admissible. **So keying `ImplReport` clears the entire remaining rank-tie surface**, which is why it
is taken over narrowing a tag: the class, not the instance.

**It is a supersession, not an amendment, and the route is named here because the runbook may not choose it.**
ADR-0073's five keyed kinds are named at `:47-52`, **inside `## Decision`**. ADR-0092 rule 1: *"An erratum … leaves
every sentence of `## Decision` … standing as written. A correction that would move either is a supersession and
takes a new record."* Adding a sixth moves that sentence. **`ADR-0132` supersedes ADR-0073 IN PART** — the
keyed-kinds sentence only — on ADR-0128→0107's form: 0073 stays `accepted` and carries `superseded_by: ["0132"]`,
and every other clause of it stands, including PC7(ii)'s withdrawn specificity tie-break and the explicit
*"what replaces the tie-break for kinds that are not keyed is not stated in the source, and is not asserted here."*

**Not by `method_rank`.** ADR-0056 makes a tie a **validation error** precisely so that it is not resolved by
nudging a number; `method_rank` is a method-preference ordering and using it as a tie-break would encode a
preference nobody holds.

**What this does not license.** `spikes/p9/artifacts.toml` is **not** edited and stays at five keyed kinds — §7b.12
forbids editing the spike registry to make a case go green, and `.redkiln/` is derived output. Checkpoint 7f
asserts both counts.

---

## D21-5 · The rate-limit reading is journalled rather than ADR-0110 amended, and RT-19 is settled with the renderer

**Taken 2026-09-21.** ADR-0110 defines headroom as a batch delta *"over readings sharing one `resetsAt`"*, and
**nothing journals a `resetsAt`**. The route taken is to **journal the reading**, not to redefine the delta over
whatever happens to be recorded.

**Grounding, measured.** `EventKind::RateLimitReading` exists with the wire name `rate_limit_reading` and has **no
producer anywhere in the workspace** — its only references are three kernel match arms classifying it as tier-2.
The row that IS written on every dispatch, `DispatchRecord`, carries `headroom_before` / `headroom_after` as
`WindowReading { five_hour, seven_day }`, which has no such field. **The value is already in hand and discarded**:
`rk-adapter-claude`'s `headroom::read_rate_limit_event` returns a `RateLimitReading` carrying `resets_at`,
`five_hour_resets_at` and `seven_day_resets_at`, and `ClaudeHarness::last_reading()` exposes it. Redefining the
delta over *"readings in one run"* would be a different measure wearing the same name — **F6's quantisation of
`utilization` to 0.01 is why the `resetsAt` grouping exists at all**.

**A record is owed and is named here: `ADR-0133`**, because the kind is *routed, not decided* — design §5's event
table has **zero** hits for `rate_limit_reading` and so does the AC register, and ADR-0126 characterises it only in
passing (*"one reading, not a before/after pair with a cost"*) while naming no producer, no payload and no moment.
The record fixes those three and design §5's table is amended toward it (ADR-0092 rule 2.2's direction).

**RT-19 is settled in the same pass**, because the renderer is what forces the unit question: `total_cost_usd` is
`CostUsd(String)` — **the decimal digits verbatim as the CLI printed them, never a float** (`s7c` §7c.1 as
amended) — and what a reader may do with them is fixed by that record rather than by the first call site.

**What this does not license.** No per-run headroom figure, ever (AC-373; F6). And no **vacuous** rendering: an item
with no dispatches may not print a `low_sample` delta over `n = 0` and let criterion (7)'s grep read it as green —
Step 8 declined to ship that shape and `s7i` may not reintroduce it.

---

## D21-6 · ADR-0131 is confirmed as written and flips to `accepted`; its sharpest open item becomes **R-64**

**Confirmed 2026-09-21 (dated line, the operator's word).** ADR-0131 — the event store's selector as a **tag set**
over a disjunction of kinds, with `ByItem` / `BySha` / `ByRun` re-expressed as the reserved keys `item`, `sha` and
`run`; tags and the store-minted instant **outside `canonical_events`**; the three item-less tier-1 kinds gaining an
`item` tag; `rk-adapter-happenstance` as the intended tier-2 adapter with git remaining the tier-1 authority and
DCB's append condition explicitly declined; ADR-0109 decision 2, ADR-0105 clause 8, design §5.6 and AC-184 **not**
re-opened; `rk-adapter-sqlite` retained; no crate edited; the typed per-kind payload record left OWED; and six
things named as undecided — is **confirmed as written on this date, all ten decisions**, with no clause withdrawn
and none added. It flips `proposed → accepted` in the commit this line licenses, `date: 2026-09-21` **unchanged**
(ADR-0126's route; 0128, 0129 and 0130's precedent); `adr/README.md` takes its index-row status cell and its
`proposed`-count sentence in the same commit. Its record BODY is not edited — 0128 and 0130 are both `accepted`
and both still carry their own *"Committed `proposed`"* clause, which is a historical statement about how the
record was committed and stands.

**Grounding, measured rather than asserted.** The record edits no crate, so confirming it costs nothing that a
later measurement could invalidate, and **decision 2 is what makes that true**: tags sit outside
`canonical_events`, which stays `{kind, payload}` byte for byte, so no golden log moves, `CORPUS_PIN` does not
move and `run_id` is untouched. Checked directly against the work that would have been at risk — Step 8's
`rebuild_differential` and `stage_level_next_action` compare `StageAction` under `rk_store::canon` and are
therefore unaffected — and decision 6 says in terms that AC-184 is not re-opened.

**Clause 8's trigger is recorded here because it has half-fired.** *"The wiring lands after Checkpoint 8"* — and
Checkpoint 8 **is** now recorded as passed, at `d6160ff`. The wiring is nonetheless still gated: **S-HS is unrun**
(it appears only in this record and `references/2026-09-20-operator-surface-proposal.md` §12.6, with four kill
criteria), and M5 — *"Redkiln-RS runs its own fix lane and authors a Phase-2 story"* — is Phase-1 exit criterion
**(1)**, which Step 8 recorded **NOT SATISFIED** and which is now four stages away (`s7f`…`s7i`, D21-3). **No
reader may take "Checkpoint 8 has passed" as licence to land the wiring.**

**R-64 is opened by this line.** Decision 10's first bullet — *"Whether `doctor --rebuild` can re-derive every tag
from git alone … if it cannot, the rebuild differential (AC-184) must exclude tags"* — is called by the record
itself *"the sharpest open item this record leaves"* and *"where a later pass should press on"*, says *"Registered
below"*, and receives **no id**: it is carried in prose at three sites and cannot be cited or tracked. It is
registered here as **R-64**, UNMEASURED, and it bears directly on the `rebuild_differential` authored at Step 8.
Opening the id changes nothing the record decides.

**What this does not license.** It does not license landing any part of the wiring before S-HS reports against its
four kill criteria (decision 8), nor reading decision 5 as licence to move tier-1 authority — decision 6 closes
that in terms. It does not discharge the typed per-kind payload record (decision 9), and it decides none of
decision 10's six named non-decisions.

---

## D21-7 · ADR-0132 flips to `accepted` — and it was **widened by one site** before acceptance, which is what this line confirms

**Confirmed 2026-09-21 (dated line), at `s7f`'s close.** ADR-0132 — `ImplReport` as the sixth toolchain-keyed
artifact kind, superseding ADR-0073's keyed-kinds enumeration in part — is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-21` unchanged. ADR-0126's route, which 0128, 0129, 0130 and 0131 each took.

🛑 **THE RECORD IS NOT WHAT D21-4 AUTHORISED, AND THE DIFFERENCE IS THE POINT OF THIS LINE.** D21-4 says the five
keyed kinds are *"at `0073:47-52`, **inside `## Decision`**"* and authorises a supersession of **that sentence**.
Executing it found a **second protected site**: ADR-0073 re-enumerates the same five in a `**What this forbids**`
bullet — *"For the five keyed kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`)
keyed expansion plus an instance-level predicate replaces it outright."* **ADR-0092 rule 1 protects a forbids
bullet in exactly the same breath as a Decision sentence**, so superseding one and leaving the other would have
left 0073 stating two different closed sets about itself. ADR-0132 names **both** sites. The precedent for widening
before acceptance rather than filing a follow-up is 0128, which README already records as *"widened 2026-09-19
before acceptance, by one clause"*. **A reader who wants the narrower record should strike 0132's site (2) and
0073's forbids bullet stays at five — nothing else in the record depends on it.**

**A second departure, smaller, also confirmed rather than smuggled.** D21-4's warrant sentence says *"ADR-0056
makes a tie a validation error"*. It does not: 0056 says of itself that the rule is *"pre-existing … and not
reopened here"*, at both the sites that state it. ADR-0132 cites `references/2026-09-07-process-catalog-proposal.md`
§2.5 step 4 (*"a tie is a pack validation error, never a runtime choice"*) as the origin and 0056 as the record
that keeps it standing. The decision D21-4 took is unchanged; only its citation is corrected.

**Grounding, measured rather than asserted.** Every figure in the record was re-derived from `.redkiln/catalog`
by an independent parser rather than carried from the stage file: **9** equal-rank producer sets; the two
`ImplReport` sets shadowing the two `Changeset` sets with identical producer lists at ranks 10 and 30; and both
remaining pairs disjoint by `kind` tag. The claim that keying clears the surface is not left to the prose — it is
asserted on every run by `shipped_pack_loads`, which was RED on precisely this `RankTie` before the pack change
and GREEN after it, with the other three load-time lints passing throughout.

**What this does not license.** It does not assert a tie-break for kinds that are **not** keyed: 0073's own limit
(*"What replaces the tie-break for kinds that are not keyed is not stated in the source, and is not asserted
here"*) survives, and 0132 declines to assert it either. It does not license editing `spikes/p9/artifacts.toml`,
which stays at five keyed kinds and now differs from `.redkiln/` by exactly one line, deliberately. And it moves
no `method_rank`.

---

## D21-8 · ADR-0133 flips to `accepted`, and RT-19 is settled inside it rather than beside it

**Confirmed 2026-09-21 (dated line), at `s7f`'s close.** ADR-0133 — the `rate_limit_reading` producer and its
typed payload — is confirmed as written and flips `proposed → accepted`, `date: 2026-09-21` unchanged.

**RT-19 is settled BY this record, which is a correction to how `s7f` §7f.4 reads.** The stage file presents
§7f.3 (the producer) and §7f.4 (the cost unit) as two sections, and a first pass at the record accordingly said
the unit was *"settled at §7f.4"* as though that were somewhere else. RT-19's own register row
(`spikes/DECISIONS-2026-09-17.md`) names its owner in terms: *"the typed-payload record (ADR-0126 decision 6)
names the cost unit and any arithmetic form"*. That record is ADR-0133. The unit is settled in its **clause 5**,
and D21-5's *"settled in the same pass"* is executed as *settled in the same record*.

**Clause 5 fixes more than the type, because the type was never the exposed part.** `CostUsd` is
`pub struct CostUsd(pub String)` with **no `impl` block at all** on this tree — no `Display`, no `FromStr`, no
arithmetic — so precision cannot be lost today by any route. The field is `pub String`, so nothing *structurally*
prevents a future caller from `.0.parse::<f64>()`. Clause 5 records what a reader may do (byte equality, verbatim
rendering, decimal summation carrying every digit) and may not (any binary float at any point, including
transiently; any fixed re-scaling; any inferred minor unit), so that such a caller is refused by a record and not
merely by a missing impl.

**Two things the record declines, both measured.** It does **not** take design §5.4's `rate_limit_event` spelling
for the durable kind: that identifier is already live and distinct as the Claude CLI's stream-line `type`,
classified by the adapter's own `EventKind::RateLimitEvent` whose `census_key()` returns that literal string.
Collapsing the two would make the census and the journal indistinguishable by name. And it mints **no `EventKind`
variant** — unlike 0123, 0124, 0125 and 0126 it carries no arity clause at all, because `RateLimitReading` is
already one of the 38 and already has the wire name `rate_limit_reading`. What was missing was only the producer.

**Why the gap is a deferral coming due rather than a discovery.** ADR-0126 decision 6 routed *"the typed per-kind
payload record … off a design §5 table that must first be given types"* forward by name, and `WindowReading`'s own
doc says *"`resets_at` per window is the typed-payload record's (ADR-0126 decision 6)"*. ADR-0133 is that record,
for this kind.

**What this does not license.** It does not build `redkiln cost`, define its `n`, its `low_sample` floor or its
`resolution_limited` flag — AC-372/AC-373/AC-395 and `s7i` own those. It does not make Phase-1 exit criterion (7)
**met**: it makes the batch delta *computable*, which is a different claim, and the renderer is still owed. And it
does not license a vacuous rendering — D21-5's prohibition on an `n = 0` `low_sample` delta standing in for a
green stands untouched.

---

## D21-9 · ADR-0134 flips to `accepted` — the port lint asserts an equality, because the emptiness it asserted was unsatisfiable inside Phase 1

**Confirmed 2026-09-22 (dated line), at `s7g`'s open.** ADR-0134 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged.

**The operator was offered three exits and took the one the evidence had already narrowed to.** `s7f` armed
`conformance_every_port_has_a_call_site` on `uncalled.is_empty()` and its own table split the debt
*"`s7g` (toolchain), later (tracker/docs)"*. The stage-open pass measured what *"later"* meant: design:265 and
:266 give `Tracker` and `DocsSite` the Phase-1 real adapter **"none in Phase 1"**, AC-499 is instantiated only
for ports that have one, AC-507/AC-508/AC-510 are Phase 3, and `s7a` §7a:198-199 authors both argument types as
contentless newtypes *"so no adapter and no kernel consumer of either type exists before Phase 3"*. No stage in
this runbook claims either port. **"Later" is Phase 3, and the third option — widen `s7g` to wire all three —
was withdrawn before it reached the operator as a live choice**, because there is no meaningful call to write.

**What the equality buys, stated as a gain rather than a concession.** The emptiness form was never going to
run: its only reachable state inside this phase was `#[ignore]`d, which is the condition the module's own
docstring calls coverage theatre. The narrowed form runs on every commit from `s7g` and fails in **both**
directions — a `toolchain` regression to zero, and a Phase-3 `Tracker` consumer landing while a record still
says none exists. The instrument is `check-checkpoints.py`'s, whose allowlist is `flagged == allowed`,
*"never 'flagged is small'"*, enumerated with a reason per entry.

**The cost is not smoothed.** The lint no longer claims *"every port is reached"*. It claims *"exactly these
two are unreached, and here is each one's warrant"*. That is a weaker headline and the only form that is both
true and runnable in Phase 1.

**And the consequence is recorded ahead of it rather than discovered.** Checkpoint 7f requires the literal
`test result: ok. 0 passed; 0 failed; 3 ignored` and three ARMED lints, so **any** discharge invalidates it —
under every option considered, including the two rejected ones, because the removed-in-the-same-commit
convention is `s7f`'s own. Checkpoint 7f is `s7g`'s entry gate only; nothing downstream requires it green, and
`00-preamble.md`'s maintenance rule already provides for a pass attesting a specific text at a specific time.

---

## D21-10 · ADR-0135 flips to `accepted` — four verbs route, twenty-nine refuse, and none is invented

**Confirmed 2026-09-22 (dated line), at `s7g`'s open.** ADR-0135 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged.

*(Noted 2026-09-26, `s8c` build, under the flip line above; the entry is not rewritten.)* This entry's *"adds no
port method — the count stays six, as Checkpoint 7a asserts"* is false in its warrant, and its substance moves.
Checkpoint 7a counts no method: its only method-level group is a floor over names (`s7a` §7a.3: *"The floor pins
**names**, never a signature"*). ADR-0170 (`s8c`'s record V, `proposed`) adds a seventh method, `run_verb`, under
D21-42 1b, and supersedes ADR-0135's six-method `## Decision` sentence and one forbids bullet in part; E-0135-1
corrects option 3's rationale. The exact count is asserted by
`source_parse::toolchain_port_declares_exactly_seven_methods` and Checkpoint 8c (4). The pack's missing
`registry.toml`, which this entry records as a gap, ships under ADR-0170.

**This settles `resolver.rs`'s OWED routing record for the `tool` arm, and only for it.** That module's header
names the gap and forbids closing it by invention — *"inventing the channel … or a mapping from a `tool` step's
`run = "toolchain:X"` — would be deciding what no record has"*. This is that record. The **agent** arm's
`CapabilityNeed` routing remains OWED and is not touched.

**The stage file described a resolution against something that does not exist.** §7g.2's *"resolved against the
capability registry"* conflates three artifacts: the adapters' five-token `Capability` **probe** vocabulary,
which has no `test`/`build`/`lint` row; the `[verb]` table, which lives only in `spikes/p9/registry.toml`, is
not shipped, and maps a verb to an output type and a liveness flag rather than to a method; and the port's six
methods. The operator took the four-verb map plus a typed refusal.

**Two measurements decided it.** 113 `tool` steps across **33** distinct verbs — §7g.2's "12" is wrong and is
corrected in the same stage — of which four name a port method (47 steps) and 29 do not (66 steps). A silent
fallthrough would make 66 of 113 steps invisible non-events, and a run that executed nothing would be
indistinguishable from one that worked.

**What it declines.** It does not ship `registry.toml`, does not map the 29, and adds no port method — the count
stays six, as Checkpoint 7a asserts. That the pack carries no verb registry at all, against design §4.5's
five-registry row, is recorded as a gap and deliberately not filled.

**`change_summary` reaching the port and receiving `Unsupported` is the correct behaviour and is licensed here
explicitly**, so that no later pass reads those 11 refusals as a defect to repair. Checkpoint 5's RED is
untouched, and the kernel names the `ChangeSummary` type in no position.

---

## D21-11 · ADR-0136 flips to `accepted` — an unresolvable guard refuses; it is not nulled, and it is not an abstention

**Confirmed 2026-09-22 (dated line), at `s7g`'s open.** ADR-0136 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged.

**The measurement the operator decided on: 121 step-level `when` clauses in the shipped pack, 54 of them
carrying a `${…}` reference that nothing on this tree resolves.** `predicate::eval`'s own `UnresolvedRef`
variant says *"nothing in this slice resolves one"*.

**The rejected option is the one that would have looked like reuse.** `rigor.rs` evaluates `when` with
`null_out_refs` applied first, and is right to: it is a static lint and its docstring says a reference-bearing
guard is not statically decidable. Calling the same function on the execution path would not express
uncertainty — it would manufacture the value the guard turns on, and under ADR-0103's three-valued connectives
all 54 guarded steps would be **silently skipped** while the run exited 0. The function is `pub(crate)` and one
import away, which is why the prohibition is in a record rather than left to review.

**The near-miss is recorded too, because it is the more plausible mistake.** Routing `UnresolvedRef` to
`Tri::Null` and thence to AC-079's abstention gate would file a **missing mechanism** under a **decided
policy**: the gate would fire 54 times per run for a reason no operator can act on, and the journal would show
the kernel having asked a question it never asked. `Tri::Null`'s own routing (ADR-0032, AC-079) is untouched.

**What this licenses and what it does not.** It licenses `s7g` shipping a step walk that cannot decide 45% of
the pack's guards, provided each such step says so by name. It does **not** license a partial resolver — a
lookup for `${x.answer}` but not `${x.outputs.y}` — and it does not make the resolver anyone's: it is OWED,
named with what it must supply (an output-binding model, prior-step name resolution, and the `for_each` /
`repeat.until` semantics `catalog.rs` parses and declines to evaluate), and **no stage in this runbook claims
it**. That is a gap this line opens deliberately rather than closing by assertion.

---

## D21-12 · ADR-0137 flips to `accepted` — the run path stops shipping a toolchain that reports itself absent

**Confirmed 2026-09-22 (dated line), at `s7g`'s open.** ADR-0137 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged.

**The operator chose the option that moves criterion (1), over the two that would not.** AC-473's clause is a
claim about the binary — *"toolchain stage [to have] run through `rk-adapter-toolchain-rust` behind the
`Toolchain` port"* — so a test-only observation would satisfy §7g.4's bullet and leave the criterion where it
is, and a flag defaulting to the fixture would leave the default path unmeasured.

**What was measured first.** `RustToolchain::new` has **zero** call sites in `rk-cli` or `rk-kernel`;
`FixtureToolchain::default()` has exactly two non-test sites, and its `Default` gives `present: false` — so the
live run path has shipped a toolchain **reporting itself absent**, which is why the gap has been invisible.

**Four construction values had no decided source and each is now warranted rather than assumed.** The adapter is
selected by the port's own `detect()` in `[adapters] toolchains`' declared order — position would not do, since
the shipped order lists Node first and this tree is Rust, and `detect` has had no caller at all. `project_root`
is the **run-owned worktree**: §7c.14 forbids the checkout a checkpoint is running in, and the adapter's own
header already says *"the self-hosting run over `redkiln-rs` itself is Step 8's"*. The construction-time
backstop may not be smaller than the largest step it could serve, and the pack's largest declared literal
`timeout_secs` is **3600**, so that is the floor. Scratch is `<common-dir>/redkiln/tmp`, which exists.

**The refusal is the point of clause 8.** When no listed adapter detects, the run refuses rather than falling
back to a fixture — a fallback would restore today's state under a new name, on the path where a real failure
is least visible. The narrowing that follows (a tree with neither Node nor Rust cannot run) is stated rather
than discovered; Phase 1's subject is self-hosting over this repository.

**A gap is opened, not closed:** 94 step lines carry `${toolchain.<key>}` timeout substitutions, and
`[toolchain.timeouts]` names the eleven legal **keys** while carrying **no durations** for them — so those
substitutions have no value source on this tree, in the shipped pack or the spike one.

---

## D21-13 · ADR-0138 flips to `accepted` — the step list is transcribed, on the warrant `RecordedSpec` already carries

**Confirmed 2026-09-22 (dated line), at `s7g`'s open.** ADR-0138 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged.

**This is the one of the five the operator did not choose from options**, and the line says so. It was taken as
an engineering call inside the plan approved on 2026-09-22, and it is recorded at the same grain as the other
four rather than left in a commit message — because two of its three candidate routes are closed by existing
records, and a reader who did not know that would read the choice as arbitrary.

**The two closed routes.** `ActivitySpec` may not grow a sixth field: `rk-types` is Checkpoint 5's and
ADR-0099:61 fixes the spec at `StepSpec`'s five. The catalog may not be loaded from inside the runtime:
`runtime.rs`:22-27 carries residual 7 by name and asserts the absence structurally. What remains is
transcription, and the only question is where it lands.

**`RecordedSpec` is the type for exactly this, and its own docstring is the warrant.**
`activity_id_at_version` sits there because the join *"has three candidate homes and choosing one is a decision
this stage may not take; the binding transcribes it instead"*, and it is explicitly *"NOT a sixth spec field"*.
A step list is the same kind of value from the same source — held by the composition root, not derivable by the
kernel, and not the Harness port's business. `run.rs`:1423 already reads `activity.steps` at bind time and
discards the rest.

**Two things this line pins so they are not re-opened silently.** AC-013's idempotence grain stays **per
activation** — `RunState::is_committed` has no step component — so the walk happens inside an activation that
is not yet committed and commits atomically; a run that dies mid-activation re-runs that activation's steps,
and that is the intended behaviour rather than a limitation to engineer around. And the `&[Step]`-parameter
fallback stays available: clause 4 requires the switch to be recorded **with the measured fan-out that caused
it**, so it cannot be made silently and cannot be used to reopen the two closed routes.

**What stays OWED:** the record naming the home of the DAG-node ↔ `Activity::id_at_version` join, which
`RecordedSpec`'s docstring already names and which now carries a second transcribed value.

---

## D21-14 · ADR-0139 flips to `accepted` — a decision CORRECTED BY ITS OWN FIRST EXECUTION, one day after it was taken

**Confirmed 2026-09-22 (dated line), at `s7g`'s build.** ADR-0139 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged. ADR-0137 gains `superseded_by: ["0139"]`, stays
`accepted` as a partial supersession requires, and both bodies record the scope.

**Like D21-13, this was not an operator choice between options and the line says so.** It was an
engineering call taken inside the plan approved on 2026-09-22, and it is recorded at the same grain as
the operator's own because it **moves a `## Decision` sentence of an accepted record** — which ADR-0092
rule 1 routes through supersession, not errata, whatever the record's age or authorship.

**What makes this one worth a line rather than a commit message: the record it moves is one day old, and
its own author moved it.** ADR-0137 clause 8 was written on 2026-09-22 and superseded in part on
2026-09-22, between which it was executed for the first time. That is not a mistake to smooth over — it
is the ordinary result of a decision taken **ahead of its first measurement**, which is exactly what
`s7g`'s stage-open records had to be, because the port had no call site to measure. The register keeps
both states rather than rewriting the first into the second.

**The measurement, stated as the cause rather than the justification.** Clause 8 said the run refuses at
startup when no adapter detects. Implemented verbatim it failed **11 of `rk-cli`'s 101 tests** — every
one over a fixture repository carrying neither a `Cargo.toml` nor a `package.json`, every one about the
**gate/answer** path, and one of them against a premise line reading *"opening the gate must exit 0
(AC-403)"*. A run was refused for lacking a capability it never reached, which is the same class of
miscommunication as a silent success, inverted.

**What was kept is the half that was load-bearing, and the distinction is not one of degree.** Clause 8's
prohibition — *"It does not fall back to a fixture"* — is **not withdrawn** and is the test the
replacement had to pass. `FixtureToolchain::test()` returns `Ok(selection.len())`, a success derived from
the length of its own argument; every `UndetectedToolchain` method returns `Err`, naming the verb, the
project root and each adapter tried. A construction refusal — a denylisted child environment, a relative
root, a zero bound, a registry that does not load — is **still not deferred**: those are configuration
defects and refuse at startup, as ADR-0137's drivers require. Only *"nothing here to detect"* moved.

**The rejected alternative is recorded because it was the tempting one.** Keeping the startup refusal and
giving every fixture repository a `Cargo.toml` plus a toolchain-program override would have preserved the
clause and coupled eleven tests to a capability none of them exercises, spawning a child per test to
answer a question none of them asks. That is a larger change, and a less honest one.

**What this does not license.** It does not make criterion (1) met — it keeps the mechanism ADR-0137
clause 1 supplies reachable. It does not license a `tool` step succeeding without a toolchain: under
ADR-0135 the verb is mapped and the port is called, and here the port refuses by name.

---

## D21-15 · ADR-0140 flips to `accepted` — the toolchain bound is the only bound, and the record that said otherwise was reproduced in code

**Confirmed 2026-09-22 (dated line), at `s7g`'s repair pass.** ADR-0140 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged. ADR-0137 keeps `superseded_by: ["0139", "0140"]`, stays
`accepted`, and is now partially superseded at **two independent clauses**.

**Like D21-13 and D21-14, an engineering call inside the approved plan rather than an operator choice among
options — and, like them, it earns a line because it moves a `## Decision` sentence of an accepted record.**
The operator's instruction at this pass was to correct the two wrong records and record the stage BLOCKED;
which route a correction takes is ADR-0092 rule 1's answer, not a preference.

**What the measurement actually was.** Clause 5 presupposed a per-step timeout in two places. `catalog::Step`
carries eleven fields and none is a timeout; `catalog.rs`'s loader documents the omission in its own voice
(*"It types only what the loader sees … The loader reads no `timeout_secs`"*). So the shipped pack's **298**
`timeout_secs` declarations — 204 literal, 94 unresolvable `${toolchain.<key>}` substitutions — are read by
nothing at all.

**The sharper half, and the reason the erratum alone was not enough.** A `timeout_secs` *does* exist:
`ActivitySpec`'s, bound per **activity** and handed to `Harness::dispatch`. Clause 5 conflated an
activity-level **harness** bound with a step-level one that does not exist. A correction that said only "no
timeout is applied" would have left that conflation in place; the record names both halves.

**Why it is not cosmetic, stated as an operational consequence rather than a tidiness one.** A reader of
clause 5 — or of either code docstring reproducing it — concludes that a `tool` step declaring
`timeout_secs = 120` is cut off at 120 s and that 3600 is a distant outer fence. Measured, the step runs
under 3600 s and **nothing else**, and `run_tool_step` calls `toolchain.test(&[])`, which the real adapter
reads as `cargo test --workspace`. The practical reading is the inverse of the record's: the bound is not
generous, it is the whole story, and lowering it shortens the only protection a `tool` step has.

**What is kept, and what is opened.** The 3600 s value and the optional `[adapters] toolchain_timeout_secs`
key stand — the number was right and only its justification moved. What is **opened**, deliberately, is that
298 declared step timeouts are dead data: whether a step-level bound should exist, which port would apply
it, and where the 94 substitutions get values are OWED and belong with ADR-0135's unshipped-`registry.toml`
question. **This line does not license building one.**

---

## D21-16 · ADR-0141 flips to `accepted` — and it closes a route inconsistency the corpus opened against itself

**Confirmed 2026-09-22 (dated line), at `s7g`'s repair pass.** ADR-0141 is confirmed as written and flips
`proposed → accepted`, `date: 2026-09-22` unchanged. ADR-0138 takes `superseded_by: ["0141"]` and stays
`accepted`, as a partial supersession requires.

**This line's subject is not the step list. It is that the corpus applied two different routes to one rule
in one morning, and only one of them was right.** ADR-0139 superseded ADR-0137 clause 8 at 11:28, citing
ADR-0092 rule 1 in terms and noting that the rule holds *"even though the record being moved is … its own
author is moving it"*. E-0138-1, written 15 minutes later against the identical shape, took the **erratum**
route and justified it on the ground that clause 4's instruction *"stands and was followed"* — while its own
body, two sentences earlier, recorded that the fallback the instruction names was **not taken**. An
adversarial pass found the contradiction; the parenthetical is withdrawn in place and this record is the
route it named.

**The antecedent fired and the measurement is not in dispute.** Clause 4 conditioned on the transcription
fanning out beyond *"the three sites that thread `RecordedSpec` today"*. Measured at the tree clause 1 was
executed against: **17** `RecordedSpec::supplied` sites and **19** `run_activity` sites. The "three" counted
the production thread and silently excluded every test call site — which is exactly the population a
signature change is paid for in. E-0138-1 keeps that correction and is reduced to it.

**Clause 1 stands, and saying so matters.** The step list really is carried on `RecordedSpec`; the route was
right and is untouched. What the record never described is the **optionality**, and optionality is the whole
reason 16 machinery call sites — dispatch, retry, commit races, AC-013 re-entry, none with a catalog
activity behind it — survived untouched. `None` is an explicit absence in ADR-0032's sense, never an empty
walk, and an explicitly empty list is refused by name, so ADR-0138's forbids bullet survives verbatim.

**A gap is opened rather than closed, and it is the same shape this stage keeps finding.** The production
path must always transcribe, and nothing enforces it: **one** `with_steps` call site against **16**
`supplied`, with no guard. A convention that holds only because one person wrote both sites is the condition
under which a source lint passes on code nothing calls. **A guard is OWED**, and the record says so rather
than trusting the convention.

**What neither this record nor ADR-0140 does.** Neither repairs one of the four blocking defects
`runbook/PROGRESS.md` §1's Step 7g row records. The step executor is still measured at **0 of 102**
activities completing a committing walk, and correct bookkeeping around a blocked stage is not progress on
it. Both records say so in their own bodies, so that a reader who reaches them without the tracker cannot
mistake a tidy register for a working executor.

---

## D21-17 · ADR-0142 flips to `accepted` — the first of Step 7g's four blocking defects is repaired, and it was a threading problem

**Confirmed 2026-09-22 (dated line), at `s7g`'s first executor repair.** ADR-0142 is confirmed as written
and flips `proposed → accepted`, `date: 2026-09-22` unchanged.

**An engineering call inside the approved plan, like D21-13, D21-14, D21-15 and D21-16** — and recorded at
this grain because `runbook/PROGRESS.md` §1's Step 7g row had already named the record as OWED, and because
what it settles is **where the executor's context comes from**, which is a routing question no earlier
record answers.

**The finding underneath is worth more than the fix.** The blocker read *"the executor evaluates against an
empty `Context`"*, which sounds like a missing component. It was not: `Item::context()` and
`rigor::context_for` both already existed, both are what **selection itself** evaluates against, and both
were in scope in the two functions that drive the walk. Nothing had to be designed. The executor was simply
never handed what the rest of the kernel already had, which is the same shape as `s7g`'s other three
blockers and is worth naming as a pattern rather than as four coincidences.

**What the operator gets, stated as a range rather than a headline.** `when` guards go from **0 of 121**
evaluable to **75**. The remaining 46 keep ADR-0136's by-name refusal — and, for the first time, refuse for
their **real** reason: they were previously dying on `UnknownField("rigor")` before ever reaching the
`${…}` reference the refusal named, so the message an operator saw was wrong about its own cause.

**And it repairs none of the 94 `check` exprs, which is the more important half of the record.** Every one
reads a prior step's OUTPUT. A context assembled from the item cannot bind one, and no amount of facet
plumbing will change that. The `check` arm therefore becomes the **dominant remaining reason a walk ends**,
which is the correct next thing to look at and is now recorded as such rather than discovered later.

**Two things this line pins so they are not re-opened silently.** First, **one definition**: the rigor
ordinal is built in a single place (`rigor::with_rigor`, extracted from `context_for`, which now calls it),
so the static rigor pass and the executor cannot disagree about what `rigor >= recommended` compares
against — a divergence there would be invisible, because both sides would be internally consistent.
Second, the context is **built, not transcribed**: ADR-0141's route was available and was declined, because
the kernel already holds both inputs and carrying a derived value through the run record would add `serde`
to `predicate::Value`, pin a registry-derived ordinal domain into every record, and create a second thing
that can drift from selection.

**A consequence neither the record nor the plan predicted, recorded because it is evidence the fix is
real.** `StepVerdict::Skipped` was **unreachable by construction** — an adversarial pass established that
`parser::operand` admits only a `${ref}`, a prefix call or an identifier path, with no bare literals, so no
parseable guard could ever answer `false` against an empty context. The variant, its match arm and its
docstring were dead code. Binding the context gives it a first reachable case, and the test asserts that
case rather than only the affirmative one — because a guard that always answered `true` would also have
"stopped erroring".

**What this does not license.** It does not make any activity complete a committing walk. Three blocking
defects stand — the second-`agent`-step report overwrite, the swallowed post-commit failures and the no-op
`gate` arm — and **Step 7g remains BLOCKED**. It does not build step-output resolution, and it does not
weaken ADR-0136's prohibition on nulling references, which is untouched and is not made safer by the
context now being populated.

---

## D21-18 · ADR-0143 flips to `accepted` — the second of Step 7g's four blocking defects, and a measurement that closed the tempting fix

**Confirmed 2026-09-22 (dated line), at `s7g`'s second executor repair.** ADR-0143 is confirmed as
written and flips `proposed → accepted`, `date: 2026-09-22` unchanged.

**An engineering call inside the approved plan, like D21-13 through D21-17.**

**The line exists because a measurement closed an option that would otherwise have been taken.** The
obvious repair for *"the second agent step overwrites the committed report"* is to keep the first report
and treat later agent steps as **already served by the one activation dispatch** — it is a one-line
change, it makes the tests green, and it is defensible from `ActivitySpec`'s *one spec per node* contract.
Measured, it is wrong for **43 of 63** activities: they declare agent steps under **different roles**
(`[spec-author, reviewer]`, `[architect, reviewer, architect]`), and a dispatch bound to the spec-author's
card has not performed the reviewer's step. That fix would have silently dropped a review in 43
activities — the defect class `s7g` exists to end, reappearing one level up, and it would have shipped
looking like a tidy-up.

**What was actually wrong is smaller and worse than it looked.** AC-013's re-entry guard was doing its job
perfectly: it returned its opened report for the second call. The walk then stored that report over the
committed one, so an activation that HAD committed was reported as `NotCommitted { class: None }` —
carrying no error class at all, which is the hole **Mechanism B** was added to close, and on a report
whose `skipped()` is also `true`, so a failed walk read as a legitimate AC-013 skip. **Three separate
instruments were telling the operator something false, and none of them was itself broken.**

**Keyed on a RUNNABLE second step, not a declared count** — and that distinction is only a real one
because **D21-17 landed an hour earlier**. Before ADR-0142 the guard on a second agent step could not be
evaluated at all, so "runnable" and "declared" were the same thing. The two repairs compose rather than
merely accumulating, and the control test asserts the composition.

**What it buys, stated as honestly as the last one.** Nothing becomes executable. 63 of 102 activities
refuse where they previously failed — the same outcome, now with a cause, delivered through Mechanism B's
channel instead of bypassing it. That is the whole gain and the line does not dress it as more.

**And the record declines to name the answer.** Per-step `ActivitySpec`s are the obvious next move and
clause 5 records them as OWED **without** asserting they are right: 43 activities interleaving
`spec-author` and `reviewer` may be describing two activities and a review gate rather than one activity
needing three specs. That is a catalog question, and deciding it by implementing the spec change would be
deciding it by accident.

**What this does not license.** It does not widen AC-013's idempotence grain, which ADR-0138 pins and
`kill_and_resume_n_step` asserts. It does not touch the `check` arm (ADR-0142 clause 4), the swallowed
post-commit failures, or the no-op `gate` arm. **Step 7g remains BLOCKED**, with two blocking defects
standing.

---
## D21-19 · Checkpoint 7g is amended a SECOND time, and the cannot-dispatch lint is widened — the instruments, not the executor

**Confirmed 2026-09-22 (dated line), at `s7g`'s third repair pass.** No ADR is written for this line and
that is the decision: `design` §4.5 already decides that `check` cannot dispatch, `halts.md` Halt 8
already decides how a checkpoint line must be written, and ADR-0134 clause 4 already fixes the
equality form. Amending the instruments to actually assert those is **execution order, not a new
decision**, and reserving "a decision is owed" for real free residuals is what keeps that phrase
meaningful.

**The first hardening note does not cover this one, and is not stretched to.** It reads *"Amending a
checkpoint that has never been run is legitimate; doing it silently is not."* Checkpoint 7g has now
run, green, twice. So the second amendment carries its own dated warrant beside the first rather than
editing it — a pass is evidence about a specific text at a specific time.

**Finding (12), measured rather than argued.** A `panic!` on `run_activity_steps`' FIRST statement,
applied in a throwaway export at `2ce5576`, leaves **all six** Checkpoint 7g guards GREEN. Guards (1),
(2) and (4) are source lints that execute no runtime; (3) calls `Runtime::execute_step` directly; (5)
calls `Runtime::run_activity` directly; (6) constructs no `Runtime` at all — so bullet 5 would pass
identically if `runtime/step.rs` were deleted. Guard (7) is added: `second_agent::`, the only two tests
in the workspace that call `run_activity_steps` by name, and `ac432`, which reaches it through
`replay::resume`, its one production call site. Two lines because there are two routes; a kernel-only
line would be satisfied by a walk no production path enters, which is this stage's own defect one
level up. Guards (5) and (6) gained `--test it`, without which their pass-count grep is satisfied by
any target in the package reporting any pass.

**⚠️ AND THE COUNT WAS WRONG FOR A WHOLE HOUR, IN THE DIRECTION THAT FLATTERS THE FINDING.** The first
reading said **14** tests catch that mutant and doubted row 37's "twelve". Two of the 14 —
`config::adr0083_...` and `rigor::rigor_monotonicity` — fail in that clone at **pristine HEAD with no
mutant applied**, because `git clone --local` checks out under `core.autocrlf=true` and rewrote **121
of 143** `rk-kernel` fixture files LF→CRLF, moving a recorded `run_id` digest and breaking a fixture
text anchor. Both pass on the working tree. **Row 37's twelve was right.** The line exists because a
mutation cycle whose export is not byte-faithful reports kills it did not make — and the direction of
that error is the dangerous one: it makes a guard look better than it is.

**Finding (13).** The cannot-dispatch lint read only `check_verdict`'s own lines. A dispatch would be
written in `execute_step`'s `StepKind::Check` arm, which is inside `impl Runtime`, has `&self`, and
sits far past the column-0 `}` that ends the body scan. Measured: adding `self.run_tool_step(step)`
there COMPILES, makes every `check` step dispatch a toolchain verb, and leaves the lint GREEN. The
lint now scans that arm too.

**And the repair's own first draft was measurably wrong.** It banned `"self."` per line and went GREEN
against the same mutant, because `rustfmt` writes the receiver and the call on separate lines
(`match self` / `.run_tool_step(step)`) and no single line carries `self.`. That is the
substring-anchor family this programme has already paid for twice. The region is now joined and its
whitespace removed before it is scanned, and the ban is on bare `self`. Both mutant shapes — the
one-line form a person types and the multi-line form `rustfmt` produces — are RED, for the right
reason, with the clean tree GREEN.

**What this does not license.** It does not certify the executor's BEHAVIOUR; guard (7) asserts the
walk is entered and refuses correctly, not that any of the 102 shipped activities completes. It does
not repair finding (8) — `blank_string_literals` still keeps no cross-line state, the shape is live
inside the very region this lint scans, and the test's docstring says so rather than leaving it to be
found. **Step 7g remains BLOCKED.**

---

## D21-20 · ADR-0144 flips to `accepted` — a step that fails after a committed dispatch refuses by name

**Confirmed 2026-09-22 (dated line), at `s7g`'s third repair pass.** ADR-0144 is confirmed as written
and flips `proposed → accepted`, `date: 2026-09-22` unchanged.

**An engineering call inside the approved plan, like D21-13 through D21-19.**

**The line exists because two smaller fixes were available and both are wrong.** The walk's `Failed`
arm discarded the error whenever a dispatch had already committed. Returning `refused(now, e)`
unconditionally is one line and carries the error — and reports `committed() == false` for an
activation whose `activity_committed` row is in the journal, which is ADR-0143's lie with the sign
reversed. Writing the error into `outcome` is also one line — and `outcome` is the DISPATCH's own
reading in both of its shapes (`Some(Ok(typed))` on the success path, `None` on AC-013's re-entry), so
that is ADR-0143's defect one field over. The failure travels on its own field because both of the
fields it could have borrowed are already saying something true.

**What was wrong is ADR-0135 voided at the last step.** 29 of 33 `tool` verbs are refused BY NAME for
one stated reason — *"a run that executed nothing would be indistinguishable from one that worked."*
`run_tool_step` raised that refusal correctly, `StepVerdict::Failed` carried it correctly, and the arm
that received it threw it away; `dispatch_through` asks only `committed()`, which is `true`, so the run
reported a success. The refusal was working the whole time and never reached anyone.

**The population is re-measured, and row 37's number is superseded rather than repeated.** Row 37 says
**38 of 102**, taken before ADR-0143 made a second runnable `agent` step end the walk. Re-derived:
**82** activities carry a non-agent step after the first agent step, **33** reach a `tool` or `check`
step before the walk ends, and **29** of those reach a step that CANNOT succeed on this tree today. The
last figure is the one that matters — for 29 activities the dropped error was not hypothetical.

**No exit row is added, moved or renumbered.** The class maps through
`impl From<&StepError> for Exit` — eleven arms, no wildcard — and only the message differs. But
`run.rs`'s `Err(e) => Exit::ConfigError` catch-all would have mapped the new variant to row `3`
silently, which is the *"all eleven rows arrived here as the same value"* failure Mechanism B exists to
close, so the arm is written explicitly rather than the next author asked to remember.

**What it buys, stated as honestly as the last three.** Nothing becomes executable. **29 of 102**
activities now fail where they previously reported success, carrying the cause they always had. That is
the whole gain.

**What this does not license.** It does not repair the `check` arm (ADR-0142 clause 4: 94 of 94 shipped
`expr`s read a prior step's OUTPUT, and step-output resolution stays OWED). It does not touch the `gate`
arm. It does not widen AC-013's idempotence grain, which ADR-0138 pins. **Step 7g remains BLOCKED.**

---
## D21-21 · ADR-0145 flips to `accepted` — a catalog `gate` step opens the gate the selection already opens

**Confirmed 2026-09-22 (dated line), at `s7g`'s fourth repair pass.** ADR-0145 is confirmed as written
and flips `proposed → accepted`, `date: 2026-09-22` unchanged.

**An engineering call inside the approved plan, like D21-13 through D21-20.**

**The line exists because the honest small answer was available and was WRONG.** Every other refusal in
this stage — ADR-0135's 29 verbs, ADR-0136's unresolvable guards, ADR-0143's second agent step — is a
by-name refusal recording an OWED mechanism, and that was the obvious shape for the `gate` arm too. It
would have been wrong: **the mechanism was not missing.** `workflow::gate::reach` exists and its own
header calls it *"the entry a loop executor calls"*; `runtime::gate_opened_event` writes the
token-bearing row; `fsm::apply` already tells two producers of that row apart; `terminal_read` already
trailers the token and exits 0. All of it was three call sites away, with **no `src/` caller**. Refusing
by name would have recorded as OWED a thing that was already built, and 80 gate steps would still do
nothing.

**The comment that stood in the arm was false, and the falsehood is the finding.** It read *"the mark is
`workflow::gate`'s, written by `Runtime::step`'s `OpenGate` arm, and this walk authors no second path."*
The second clause was true; the first was not. `OpenGate` is reached only from `selection.gates()` —
the abstention, unsatisfiable and appetite gates, keyed `"{reason}:{axis}"` — and nothing connected a
catalog gate step to it. *"This walk authors no second path"* was an intention stated as a fact; it is
now a fact, because the arm calls the first path.

**What was measurably broken, in two populations rather than one.** **55** activities had a gate after
their first agent step and it was silently skipped. **15** reach one before dispatching — 13 with no
agent step at all, 2 with the gate ahead of one — and those fell through to the *"no `agent` step
produced a value"* refusal, whose message is about `emit` binding outputs and the reference resolver.
**A refusal naming a mechanism that has nothing to do with gates** sends an operator looking in the
wrong place, which is worse than the silence. The two take different code branches and a fix for either
alone leaves the other exactly as it was; both are asserted.

**The subtle wrong fix is guarded against explicitly.** `select()`'s announcement rows share the
`gate_opened` kind and carry `{reason, axis, site}` with **no token**, and the fold moves state only on
the runtime-shaped row. An arm that appended the announcement shape would write a row, report a gate,
pass a naive test and **gate nothing**. The guard asserts the row is token-bearing and that its token is
the same value the report carries — not two independently derived values that happen to agree, which is
the two-written-to-agree defect this programme has a standing rule about. That mutant is run and killed.

**Four absences had to be closed and none was an oversight.** `catalog::Step` never parsed `question`,
so 80 gate steps had no kind to derive a token over; `step.rs` had neither `tree_hash` nor `deadline`,
both of which this crate may not compute (AC-001, and `gate.rs`'s own header); `ActivityReport` had no
gate carrier, `mark()` being AC-030's rate-limit mark alone; and `dispatch_through` asked only
`committed()`, which is `false` for a gate BY DESIGN.

**🛑 This is the first repair in Step 7g that makes something EXECUTABLE rather than honestly refused.**
D21-17 through D21-20 each ended *"nothing becomes executable"*. This one does not: a walk that reaches
a gate now ends in `Exit::Success` with a `Gate:` trailer and an answerable token, which is a path
criterion (1) needs. **It does not follow that any of the 102 activities completes**, and the record
says so: the `check` arm still ends most walks (ADR-0142 clause 4), per-step `ActivitySpec`s stay OWED
(ADR-0143 clause 5), and the `${…}` resolver stays OWED (ADR-0136 clause 4).

**What this does not license.** It does not claim the gate a step opens is the gate an operator should
be ASKED — `question_for` is not consulted, no question text is rendered, and the answer path is
untouched. It does not widen AC-013's grain. It does not add a second gate-mark shape. **Step 7g remains
BLOCKED**, and its Result/commit column remains EMPTY.

---
## D21-22 · ADR-0146 flips to `accepted` — the adversarial pass refuted ADR-0145 on the day it was accepted, and it was right

**Confirmed 2026-09-22 (dated line), at `s7g`'s adversarial pass.** ADR-0146 is confirmed as written
and flips `proposed → accepted`, `date: 2026-09-22` unchanged. It **supersedes ADR-0145 clause 7 in
part**; `0145` stays `accepted` and every other clause of it stands.

**An engineering call inside the approved plan, like D21-13 through D21-21.**

**The line exists because a record accepted four hours earlier was WRONG in a way its own author could
not see.** ADR-0145's clause 7 reads *"The run resumes into the same step when the gate is answered"*
— and there was no mechanism by which resuming into the same step could ever get PAST it. The
`gate` arm called `gate::reach` unconditionally, and `gate::reach` mints at `reaches + 1`. So an
**answered** gate re-opened at reach 2, then 3, for ever. **The clause is the defect, written down as
the intent**, which is why this is a supersession and not an erratum: there was no misstatement to
correct, the decision itself was wrong.

**Measured through the REAL BINARY, three run→answer rounds:** three `gate_opened` rows, three
distinct tokens, and **never a `Dispatch`**. The same harness over the pristine selection gate
resumes past it on run 2 and reaches `Dispatch, DispatchCompleted, ActivityCommitted`. So ADR-0145's
*"the fold cannot tell the two producers apart and does not have to"* is true of the **row shape** and
false of the **gate lifecycle**.

**The 15 were permanently unexecutable and the `answer` verb was a treadmill.** An activation that
gates before dispatching never commits, so `is_committed` never skips its node, so every resume walked
back to the same step. The 55 that gate after a commit were safe only because `is_committed` skips the
node next time — **the defect was masked there, not absent**, which is the more dangerous half.

**⚠️ WHY THE GUARDS COULD NOT SEE IT, and it is the same shape as finding (12).** All four of
ADR-0145's tests pass `empty_state()`, where `reaches(kind) == 0` and the arm's only reachable branch
is the one that opens. **A fixture set that sits entirely at reach 1 cannot observe a re-reach defect
at all** — exactly as six checkpoint filters could not observe a walk they never entered. The repair
adds a test at reach 2 over an answered history and a void control; ADR-0146 clause 5 makes the rule
explicit.

**And ADR-0145 OPENED a second hole that its predecessor could not have had.** Before it, a catalog
`question` was dropped at load and could collide with nothing. After it, a `question` matching a
selection gate's `"{reason}:{axis}"` lands in that gate's reach sequence: measured, the step gate
minted **reach 2 of the abstention gate's sequence**, and because `fsm`'s clause 2 opens only when
`reaches == 0`, **that abstention gate could no longer be opened at all**. A catalog edit silently
suppressing an abstention, unsatisfiable or appetite gate is larger than anything ADR-0145 set out to
close. Clause 4 bans a `:` in a `question` at LOAD, through the strict loader `run` uses.

**🛑 THE ADVERSARIAL PASS ALSO REFUTED TWO CLAIMS OF ADR-0144, AND BOTH REFUTATIONS STAND.**

1. **Its guard covered ONE of the three edits the record makes.** Deleting `dispatch_through`'s
   `step_error()` check left the **whole workspace green** with the run exiting **0** — the exact
   defect ADR-0144 closed. Mutating the `run.rs` arm to `Exit::ConfigError`, the shape clause 5
   forbids by name, also left the whole workspace green. `StepFailedAfterCommit` appeared on four
   source lines and **zero test lines**. An end-to-end guard now asserts exit **19**, and it is
   mutant-tested against all three edits.
2. **`post_commit_failure.rs`'s own header claimed that mapping "is asserted there".** It was not.
   The sentence is corrected in the same change that makes it true — **a header asserting coverage
   that does not exist is worse than silence, because it is what a reader checks instead of the test
   list.**
3. **ADR-0144's claim that each of its three assertions is satisfied by a wrong fix the other two
   reject is FALSE.** An *attach-and-continue* fix passes all three, the control, and the whole
   workspace, because the guard's walk was two steps long and `return` and `fall through` are
   observationally identical at its end. A third step now makes the difference visible.
4. **A latent masking hazard is closed rather than documented.** `dispatch_through` asked
   `gate_opened()` before `step_error()`, so a report carrying both would have returned `Ok` and
   discarded the refusal. At HEAD the two are mutually exclusive — **but only because both arms of
   the walk `return`, an invariant held in another file and asserted nowhere**. The refusal is now
   asked first.

**What is OWED and is NOT closed here, named rather than left to be found.** A step gate's class is
its `question`, and `run`'s `expiry_pass` refuses a class with no `[[gate_expiry]]` row. Measured:
`grep -c gate_expiry .redkiln/packs/*.toml` → **0 and 0**. So a step gate that passes its deadline
makes the next run exit **3**. ADR-0145 added 45 gate classes to the runtime's reachable set and no
expiry rows for any of them; ADR-0146 does not add them either. Also OWED and unrepaired: the
`Emitted` break arm drops every later step silently (unreachable on today's pack — 0 of 102
activities have a step after their first `emit` — and `catalog.rs` does not enforce emit-is-last);
and ADR-0143's second-agent arm mints a fresh `refused()` report, so `committed()` is `false` and
`skipped()` is `true` for an activation whose `activity_committed` row is in the journal, while the
message it carries says `COMMITTED` — the very shape ADR-0144's driver 2 forbids, in a sibling arm
of the same `match`.

**🛑 AND THE PASS REFUTED THE INSTRUMENTS AGAIN — GUARD (7b) WAS SATISFIED BY A WALK BODY THAT PANICS.**
The line named `ac432`. `explain.rs` binds `RecordedSpec::supplied(..)`, which yields `steps: None`, so
`run_activity_steps` **returns at the fall-through one line before the walk**. Measured: a `panic!`
placed after that `else` leaves `ac432` GREEN and turns `second_agent::` RED. The guard proved the
function was CALLED, not that the walk was ENTERED — **finding (12)'s own defect, inside finding
(12)'s repair, for the second time in one day**; the first was the stale *"only two tests"* comment.
It now names an end-to-end guard that drives `run_with` over a `with_steps` binding and asserts the
exit code of a failure raised INSIDE the walk, so it cannot pass unless the body ran.

**Eleven mutations survived the block at `ad2a58e`; six still survive it.** Guard (7a) and guard (3)
are the only two that killed anything in seventeen attempts — guards (1), (2), (4), (5) and (6)
killed **nothing**. Repaired here: the walk stopping after the first non-agent step (every fixture put
its non-agent step LAST, so "stop" and "run on" were observationally identical — the same blindness
as a gate fixture that only ever sits at reach 1). **NOT repaired, and recorded as findings:** the
cannot-dispatch lint is defeated by a `&Runtime` parameter under another name, by a helper method
whose name is on no ban list, and by finding (8)'s multi-line literal truncating the scanned region
— *this stage's own docstring predicted the first and named the third, and both are now measured
facts rather than hazards*. **A source lint cannot close it; a behavioural half is OWED.** Also
unguarded anywhere: `emit`'s terminality, ADR-0138's empty-list forbid, and the `check` arm swallowing
`Ok(false)`/`Err`.

**⚠️ AND THIS PASS'S OWN EXPORT DISCIPLINE HAS THE CAUSALITY INVERTED FOR ONE TEST.** D21-19
recorded that `config::adr0083_…` and `rigor::rigor_monotonicity` fail in an `autocrlf` clone and pass
on this tree. For `rigor_monotonicity` that is right. For `adr0083` it is the wrong way round: the
committed `run_id` is a digest **over the bytes of** `tests/fixtures/logs/…/inputs/catalog/*.toml`,
`.gitattributes` does not carry a `-text` row for `crates/rk-kernel/tests/fixtures/logs/**`, and those
five files are CRLF in this worktree. **The recorded digest pins CRLF.** The test passes here
*because* `core.autocrlf=true` rewrote them, and fails on any LF checkout — including a Linux CI. So
neither an all-LF nor an all-CRLF export is faithful; the recipe is *check out, then overlay the
working-tree bytes*, and the real repair is a `.gitattributes` row. **OWED, not taken here.**

**What this does not license.** It does not make any of the 102 activities complete. It does not widen
AC-013's grain. It does not claim the gate a step opens is the gate an operator should be ASKED. It
does not claim Checkpoint 7g now certifies the executor — six mutations still survive it.
**Step 7g remains BLOCKED**, and its Result/commit column remains EMPTY.

---

## D21-23 · ADR-0148 flips to `accepted` — `toolchain:test` refuses, because the tempting repair is a green from a toolchain that ran nothing

`run_tool_step` called `toolchain.test(&[])`, and `rk-adapter-toolchain-rust` reads an EMPTY selection
as `cargo test --workspace`. All **28** shipped `toolchain:test` steps were therefore whole-workspace
runs under the 3600s bound on a shared target dir.

**The measurement that closed the obvious fix.** `catalog::Step` drops the `inputs` key, so the
declared selection is never parsed — and parsing it does not help. Measured over `.redkiln/catalog/`:
all 28 declare `inputs.selection`, and it is `"affected"` (17), a `${…}` reference the resolver
ADR-0136 clause 4 records as OWED (6), `artifact:…` (4) or `"e2e"` (1). **Not one is a test name.**
`Toolchain::test` takes `&[String]` — test selectors. Passing `"affected"` through runs
`cargo test affected`, which matches nothing and **exits 0**: a green from a toolchain that ran no
tests, which is strictly worse than the over-broad run, because the over-broad run at least goes red
when the tree is broken.

So the verb refuses by name, on ADR-0135 clause 2's own precedent, and `catalog::Step` is **not**
widened — a parsed-but-unusable value would leave the one-line change from *parsed* to *passed
through* sitting there for anyone who had not read the record. **Supersedes ADR-0135 clause 1 in
part**, at its `test` row only; `build`, `lint` and `change_summary` are untouched and `0135` stays
`accepted` with `superseded_by: ["0148"]`.

**Guard.** `tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port`,
two instruments — a counting `Toolchain` that must not move, and the refusal's own text — with a
positive control that the sibling verb `build` DOES reach the port. Mutant-tested: restoring
`toolchain.test(&[])` turns it RED and leaves the other 218 tests green.

---

## D21-24 · ADR-0149 flips to `accepted` — `detect()` chooses, and node's required-env table stops gating Rust trees

`real_toolchain` constructed and validated **every** listed adapter with `?` before `detect()` chose,
so the first candidate whose `Config::new` refused ended the whole run at exit **3**.

**This was the default ordering.** Every `.redkiln/config.toml` in the repository, shipped and fixture
alike, declares `toolchains = ["rk-adapter-toolchain-node", "rk-adapter-toolchain-rust"]` — node
first — and the required-env tables differ: node on Windows wants `SystemRoot` and `PATH`; rust wants
`SystemDrive` and one of `TMP`/`TEMP`/`USERPROFILE`. An environment satisfying rust and not node
refused a **Rust repository, naming Node**, before the question of which toolchain the project uses
was ever asked.

ADR-0137 clause 2 already said selection is by declared order and `detect()`; this makes the code
honour it. A refused candidate is recorded in `tried` and walked past, and `tried` is what ADR-0139's
`UndetectedToolchain` carries when nothing detects — so nothing is swallowed. An **unknown** adapter
name still ends the run, because that is a drift between `AdapterSet::resolve` and the match: a build
defect, not a configuration one.

**Guard.** `adapter_selection::a_candidate_whose_config_is_refused_does_not_end_the_run_before_detect_chooses`,
with **two controls** — asserted against the adapters' own `Config::new` — that this environment
really does refuse node and really does satisfy rust, because *"the loop returned `Ok`"* is also what
a loop that refused nothing returns. `real_toolchain` is now `pub`, and only so it can be observed.

---

## D21-25 · ADR-0150 flips to `accepted` — the commit grain stays, the residual is NAMED, and the module header that said otherwise is corrected

`activity_committed` is appended when the `agent` step dispatches, so every later step in the walk
runs inside an **already-committed** activation — and `RunState::is_committed` has no step component,
so a process death after the commit skips the whole activation on re-entry and its remaining steps
never run and never retry. Measured: **39** mapped `tool` steps across **23** activities sit after the
first `agent` step (`test` 26, `change_summary` 9, `lint` 2, `build` 2).

**The decision is to decline the change.** Moving the commit to the end of the walk would mean a crash
after a successful dispatch re-dispatches the `agent` step — re-execution of work that already
happened, which AC-013 forbids, `kill_and_resume_n_step` asserts, and criterion (3) is. ADR-0143 and
ADR-0144 were each written to protect exactly that. The real repair is ADR-0143 clause 5's per-step
`ActivitySpec`s, and building half of that here to close a crash window would leave the grain
incoherent. **A decision is not taken by implementing it**, and this one is taken by recording it.

🛑 **And the module header asserted the opposite.** It read *"the walk happens **inside** an activation
that is not yet committed"* — true of the steps before the dispatch, false of every step after it,
and already contradicted by ADR-0144's existence when it was written. It described the intent rather
than the behaviour, which is ADR-0146's shape. Corrected in the same change, dated, saying what the
old sentence got wrong.

**The exposure is bounded and the bound is part of the decision**: under ADR-0148 `test` (26) now
refuses and under ADR-0135 clause 4 `change_summary` (9) reaches the adapters' dated refusal, so
**4** of the 39 — `lint` 2, `build` 2 — are steps that could succeed today and be lost to this window.

---

## D21-26 · ADR-0151 flips to `accepted` — the fixture's bytes are PINNED rather than normalised, because its directory name IS the digest

`config::adr0083_…` recomputes the committed run record's `run_id` over the **bytes** of its inputs,
and `replay::read_run` applies no newline normalisation. `.gitattributes` carried no row for
`crates/rk-kernel/tests/fixtures/logs/**`, and `core.autocrlf=true` is set at **system scope only** —
nothing in the repository recorded the setting the test's green depended on.

**The finding understated it by one file and by a whole mechanism.** Measured: **SIX** preimage files
were smudged, the five `inputs/catalog/*.toml` **and `inputs/pack/incident-deviation.toml`** — and
`pack.src` is in the preimage. Meanwhile `inputs/facets.toml` (13,031 B), `artifacts.toml` and the
four `.json` inputs were LF in **both** index and worktree. That mixture is why *neither* export
direction was faithful: an `autocrlf=true` clone smudges `facets.toml` and moves the digest one way;
an `autocrlf=false` checkout un-smudges the six and moves it the other. **22 of 143** kernel fixture
files were CRLF in the tree, so a fresh clone rewrote the other **121** — the figure Checkpoint 7g's
guard (7) comment already records from its own mutation cycle.

**The repair pins rather than normalises, and that is the whole decision.** `RECORDED_RUN` is
`"implement-bf1d8cec3b22d288"` and **the fixture directory is named that**. Normalising moves the
digest and forces a rename of a directory named in **accepted ADR-0130**, in `s7b-kernel.md` and
`s7c-adapters.md` (two CLOSED stages), in `.github/workflows/ci.yml`, in `crates/rk-kernel/src/replay.rs`
and in two dated spike records. ADR-0092 decision 1 forbids editing an accepted record. Pinning gets
the whole property and costs no record a correction.

**Verified by construction, not asserted**: checked out under `core.autocrlf=true` and under
`core.autocrlf=false` and compared to the working tree — **15 of 15 identical** under all three.

---

## D21-27 · ADR-0147 flips to `accepted` — Step 7g closes at its scope boundary, and the findings it is NOT closing are named by number

The stage's scope — *"the step executor: design §4.5's five closed step kinds each executed as such,
and the `Toolchain` port's first call site"* — was built at `fbf3c4c` and is now **asserted**. Every
finding that is the executor's own semantics is repaired with a test that kills a mutant that was
diffed and read: (20), (21a), (21b), (21c), and (6), (7), (5), (23) under ADR-0148 … ADR-0151.

**The split is "is this the executor, or the thing that MEASURES the executor?"** The instrument-class
findings — **(8)**, **(10)**, **(11)**, **(14)**, **(15)**, **(16)**, **(22)** — are carried into Step
7h as recorded-OWED, by number, with their measurements intact. A scope boundary that does not
enumerate what it excludes is a way of not counting it.

**What closure does not claim.** **0 of 102** shipped activities complete a committing walk. That is
criterion (1)'s business and 7h's, and it is stated in the closing row rather than left to be
inferred. Two carried findings are load-bearing for this stage's own guards and that is said rather
than discovered: **(8)** is the defeat that blinds the cannot-dispatch source lint — which is why
(20)'s repair is a behavioural **backstop** and not a widened ban list — and **(15)** means Checkpoint
7g's `flagged 0` is weaker evidence than it reads.

🛑 **One mutant cycle in this stage produced a false reading, and it is recorded rather than smoothed.**
The first attempt at finding (20)'s defeat (c) put the truncating literal on the arm's opening line.
The lint went RED — but on `joined.len() > 40`, *"joined to 27 chars — the scan is vacuous"*, which is
the **vacuity floor, not the ban**. That draft was a weaker mutant than the finding describes, and
believing it would have credited the lint with catching something it cannot catch. With real code in
front of the literal, both floors hold, the truncation is silent, and the source lint goes **green —
blind** while the behavioural half goes red. That is the measured result the closure rests on.

---

## D21-28 · ADR-0152 flips to `accepted` — the lane's stage loop lives ABOVE `resume`, and the arm breaks

`s7h` §7h.1 instructs `replay::resume`'s new arm to *"continue the loop into the next stage"*. That
sentence cannot be executed. Every value the loop reads about "which stage" is bound before it, from
`inputs.stage`: `run_id` and `Runtime`, the `selection`, the `stage_entry` row, and — **inside** the
loop — `fsm::fold(item.id(), &inputs.stage, &journal)`. A `continue` re-folds the same exhausted stage,
reads the same `Some(successor)`, and returns `AdvanceStage` again, without bound.

**Measured rather than reasoned — and then corrected by this stage's own adversarial pass.** Mutant M7
changes the arm's `break` to `continue`. Measured **before** ADR-0157's `to != state.stage` guard
existed, it did not terminate and was killed at 150s. **Re-measured at the close, with the guard in
place, it is GREEN in 11s**: the extra iteration re-folds, the arm declines to fire because the fold has
already moved into the successor, and the loop reaches `Done`. So `break` and `continue` are now
behaviourally **equivalent** at this seam, and the `break` rests on the run-record-grain argument —
`run_id` is `format!("{}-{}", stage, &digest[..16])` — rather than on termination. ADR-0157's guard
removed the symptom, not the reason. The non-termination reading was true of the code it was measured
against and is not true of the code that ships.

And the grain is in the identifier: `RunInputs::run_id` is `format!("{}-{}", self.stage, &digest[..16])`,
so advancing inside one call would file a later stage's events under a record whose id names the first
— and that digest's bytes are what accepted ADR-0151 pins. The loop is therefore `rk-cli`'s, one
`resume` per stage, and the arm steps and breaks on the `OpenGate` model.

## D21-29 · ADR-0153 flips to `accepted` — the lane order is a DECLARED array, because a TOML table has none

§7h.1 says the successor comes from *"the pack's own stage order — `[policy] stages`"*. That order does
not exist, for three independent reasons: a TOML table is an unordered collection by specification;
this tree's parser materialises every inline table as `BTreeMap` (`toml_subset.rs:37`), so the twelve
declared stages come back **alphabetised**; and `parse_spans` cannot recover the textual order either,
because the whole inline table sits on one line and spans are at line grain.

`[policy] stage_order` is added to both shipped packs — an array, the only order-preserving shape in
`toml_subset::Value` — and the two keys are cross-checked at load so they cannot drift. `Pack::to_json`
deliberately does **not** carry it: that value is one of AC-041's four digests, steps 1-4 never read the
lane order, and adding it would move every selection memo key for no change in any selection.

## D21-30 · ADR-0154 flips to `accepted` — a recorded halt outranks a supplied successor

`next_action` put `AdvanceStage` at 4 and `Halt` at 5, which was harmless only because `ctx.successor`
was `None` on every real run. `s7h` supplies it, and from that commit a refused run whose plan is
exhausted would have **advanced a stage** — writing a tier-1 `stage_transition` and handing the next
stage a journal whose refusal nothing acted on. That is the outcome arm 5's own comment says the order
exists to prevent.

Nothing pinned the old order: the order sweep asserted `Halt` against the **no-successor** context, so
every arm had a fixture and the pair had none. The new assertion is on the combination, and mutant M4
— the two blocks physically swapped — is RED at `fsm.rs:778` reporting
`left: AdvanceStage { from: "implement", to: "verify" }` against `right: Halt { Refused { class:
"overage" } }`.

## D21-31 · ADR-0155 flips to `accepted` — one `Story:` per item per lane, and the AC-162 residual is named

Four requirements and not all four can hold: design §5.4 gives the value no stage slot; `reconcile`
makes a repeat `DuplicateStoryTrailer` drift; `doctor --rebuild` already writes ONE row per item **and
says why in its own comment**; and AC-162 wants a trailer per transition. The fourth gives.

The load-bearing evidence is that the cardinality is not this stage's opinion — `rebuild.rs:165-169`
decided it first, for a stated reason. Clauses 4 and 5 were amended once ADR-0158 measured that the
tier-1 layer has no writer at all, so the residual is two residuals: N−1 boundaries with no trailer,
and an AC-162 ordering that is **vacuous** rather than unmet.

## D21-32 · ADR-0156 flips to `accepted` — the two axes split, and the shipped review stage is unreachable for SIX absences

`triage` carries **0 of the 102** shipped activities and neither pack declares `[stage.triage]`, which
is the mechanical cause of §7h's own premise that a landed run leaves zero commits — and the reason the
advance is demonstrable today.

The record was drafted at five blockers and **blocker (0) was found by running the binary**: `bind_nodes`
reads a role's **card** before the kind's **schema**, so the missing `.redkiln/schemas/` had never been
reached by anything. What a run stops at is **8 of 8** shipped cards carrying no `permission_mode`
(AC-214). Clauses 2 and 3 were amended while the record was still `proposed`: they prescribed a fixture
pack, and ADR-0160 then measured three absences no fixture pack answers.

## D21-33 · ADR-0157 flips to `accepted` — a stage does not advance into itself, and a lane completes WITHIN ONE PROCESS

Found by running the binary twice, after six lane fixtures, seven mutants, four suites, fmt and clippy
were all green. Run two appended **`{"from":"analysis","to":"analysis"}`**, and run three would have
appended another: `from` is the fold's stage and `to` is the supplied successor, and they diverge because
the composition root computes the successor from the item record — which never moves — while the fold
follows the rows the previous run wrote.

No fixture could have caught it: every `AdvanceStage` fixture folds a journal that does not already
contain a transition into the supplied successor. The repair is one conjunct; the test replays the first
run's journal as `prior`, which is what `rk-cli` does at step (9).

**The residual under it is named**: across processes the lane is idempotent and **stalled** — runs 2 and
3 exit 0 and append nothing — because `inputs.stage` is the record's and AC-518 forbids writing the new
stage back to the checkout. Exit 0 on a second invocation is not progress.

## D21-34 · ADR-0158 flips to `accepted` — the tier-1 store layer has NO WRITER on any axis

Found while deciding where the `Story:` write belongs, by asking what *"the store row for that
transition"* actually is. It is a named table, and there are three of them, each with an `anchor_sha`
column, under the header *"tier 1: git first — these rows never exist without their trailer commit"*.

Measured: **zero** `INSERT INTO t1_*` in the workspace; all three tables **empty** in a live store after
real runs; and the `EventStore` port carries only `append`/`query`/`snapshot`/`rebuild`, so no caller
could write one. **`Gate:` is the control that makes it structural**: its trailer producer has existed
since 7d and `t1_gate` is empty too. AC-160..AC-163 and AC-162 are therefore **vacuous on every axis** —
not met, not violated, and saying which matters. Closing it is a port-surface change and is routed.

## D21-35 · ADR-0159 flips to `accepted` — the `<project>` component is declared, and its absence refuses

Nothing in the repository declared a project, and the failure mode of guessing is silent: an empty
project makes `split_qualified` return `None`, `classify` produce `BareStory`, and `reconcile` file it
under `bare_story_ids` — a real commit that records no transition and still passes `grep '^Story:'`. The
four candidate sources are each wrong for a stated reason. `[project] name` is declared, whitespace-only
counts as absent, and the refusal is at the mint rather than at startup.

The round-trip test asserts both halves: the emitted value parses as a **qualified** story and reaches
`stories_committed`; the empty-project value parses as `BareStory`, reaches no project, and is not even
drift — while **both satisfy `grep '^Story:'` identically**.

## D21-36 · ADR-0160 flips to `accepted` — the `Slice-Verdict:` axis needs a slice identity nobody declares

Three absences, and none of them is the plumbing. `EventKind::SliceSealed` has **exactly one
construction in the workspace and it is `doctor/rebuild.rs`** — the rebuild path, reading trailers back.
`review::confirm` produces five event kinds and **no seal**, so wiring the review gate would still not
seal a slice. And `<slice>` is declared nowhere: `ItemRecord` is `{id, kind, stage, [facets]}`.

That is ADR-0159's residual one arm along, and inventing an identity would discharge a **substring**
lint with code no run reaches — the "source lint passes on code nothing calls" failure this programme
has already paid for twice. So `s7h` records the axis ARMED, corrects the lint's reason and its now-false
failure message, and names the route.

---

## D21-37 · ADR-0161 — the adversarial pass found THREE defects in this stage's own green code

Every one of them was in the tree while nine records were accepted, seven lane fixtures were green,
nine mutants were RED, four suites passed, fmt and clippy were clean, and Checkpoint 7h failed only on
the bullet it records as OWED.

**(1) ADR-0157's guard was insufficient, and its own experiment could not have seen it.** `to !=
state.stage` is a PROXY for *"has this transition already been recorded"*. `apply` sets `state.stage` to
the LAST transition's target while the composition root computes the successor from the item record's
stage, which never moves — so a lane of `[triage, analysis, specification]` walked in one process
appends a **BACKWARD** `{"from":"specification","to":"analysis"}` on its next run, then re-advances
forward: two tier-1 rows per invocation, for ever. The sufficient test, `stages_entered`, already existed
and was pushed by the same `apply` arm. ADR-0157 is superseded in part and keeps `status: accepted`.

**(2) A non-zero exit alongside an advance was DISCARDED.** The lane loop read its exit only on the
no-advance path, so all four of `story_trailer`'s refusals reached stderr while the process exited on
the last stage's status. **ADR-0159's *"refuses by name"* could not fail a run.** A refusal nothing can
observe is not a refusal.

**(3) A `[project] name` containing `/` breaks the axis silently.** `split_qualified` takes the FIRST
slash, so `acme/web` emits a mark that parses under project `acme` with slug `web/<item>`: the
idempotence read can never match it, so it re-mints every run as `duplicate-story-trailer` drift, and
`doctor --rebuild` reports the slug `unresolvable`. Refused at the mint now.

**The instrument that was missing.** The fixture pack gains a third live stage so a lane can take TWO
boundaries; the new test walks both and then replays, which is what a second `redkiln run` does. Mutant
**M10** restores ADR-0157's guard exactly and the test is RED at `lane_order.rs:555`, `left:
Some("implement")` against `right: None`. A one-hop fixture could not have told the two guards apart —
which is why ADR-0157 shipped with the hole.

**Five ERRATA the same pass raised**, all self-inflicted and all the same shape: `0152`, `0153`, `0154`,
`0155` and `0159` carry file:line citations into files this stage kept editing. `0152`'s is contradicted
by `0154`, its own batch-mate, which did the reordering; `0159`'s undated *"config.toml is thirteen lines
… grep returns nothing"* was falsified by `0159`'s own decision. **Cite by quotation, not by line** —
the dated blocks survived, the undated ones did not.

**And a fourth finding this stage does not own:** `terminal_read` scans for any tokened `gate_opened` row
with **no** answered/void/expired filter, unlike `RunState::awaiting_gate` which excludes all three, so
after a gate is answered it re-commits a `Gate:` trailer for the dead token and prints that the gate is
open. That is 7d's producer; recorded, not repaired.

---


## D21-38 · ADR-0162 — the review call site is built, and the guard that asserts it could have been satisfied by a doc comment

`Exit::FindingsBlock = 4` has carried the sentence *"its producer is the kernel's, built at 7b-ii"*
since 7b-ii and had **no producer**. Checkpoint 7h ran whole and failed at guard (10), and only
there. It now reaches `CP7H-END-OF-BLOCK reached` at `5e67ac0`.

**The record that blocked the route disagreed with itself, and nobody had recorded it.** ADR-0156's
`## Decision` clause 3 says *"No fixture pack is authored"*; its `## Consequences` says
*"`Exit::FindingsBlock` gains the producer its own declaration has been promising since 7b-ii"* and
*"cleared for the fixture corpus"*; and two of its `What this forbids` bullets bind *"the fixture
pack's review activity"* — a thing clause 3 says does not exist. Clauses 2 and 3 were amended while
the record was still `proposed`, and `## Consequences` was never carried forward with them; the
record's own **title** already says *"clauses 2 and 3 prescribed a fixture pack and were amended"*.
**E-0156-1** records the contradiction. ADR-0162 supersedes clause 3's one sentence **in part** —
that sentence was reasoned from ADR-0160's SLICE-IDENTITY absences, and ADR-0160 clause 5 says in
terms that *"§7h.3's call site is separable … a review call site could exist without sealing a
slice"*, so the withdrawal reached further than its own reason. ADR-0156 keeps `status: accepted`;
only `superseded_by` moved.

🛑 **THE GUARD WAS SATISFIABLE BY PROSE, AND MEASUREMENT CAUGHT IT IN THE ACT.** The moment the
producer landed, guard (10)'s grep returned **three** hits and **two were doc comments** —
`review_pass.rs`'s module header and `run.rs`'s function doc. The authored guard would have gone
green on two sentences *about* a producer, before any run reached one. That is ADR-0160 clause 4's
prohibition, and the shape this programme has paid for twice: both `s7g` source lints went green
while `execute_step` had no caller, and ADR-0158 measured three declared tier-1 tables with zero
writers. Guard (10) is now **behavioural** — a real `redkiln run` exiting `4` — with the grep kept
only as a floor scoped to CODE lines. It was proved able to fail: `expected` reads 5 → rc=1; the
fixture absent → rc=1; **only doc comments remaining → `code lines: 0`, rc=1**; control rc=0 either
side, tree restored.

**§7h.3 said the modules *"are not written here; they are wired"*. True of the verdict machine,
false of three things no record named** — `impl ReExecutor` existed at three sites, **all under
`tests/`**; `resolved.stage` is compared to no string literal anywhere in `crates/rk-cli/src/`; and
`pack-min` declared `review = "mandatory"` with **no `[stage.review]` block** behind it, a stage the
policy demands and the pack never described, which is the mechanical reason the fixture could not
reach a review. **A fourth turned out not to exist**: the finding set is read from the journal off
`activity_committed.result`, which is `Typed.value` **verbatim** (AC-009), so no kernel seam and no
new port method were owed and `dispatch_through`'s signature is untouched.

**The tenth exit row, and `4` was not in GAP 21's set** — it was RAISED and unreachable for the whole
of 7d. Six conditions have to hold together and `findings-block/case.toml` names all six, because
changing any one changes the expected code. The re-executor runs AC-288's four cells at **two**
trees — `four_cell`'s only `Flipped` cell is `(true, false)`, and a command that fails at both ends
separates nothing — with the reference arm in an **ephemeral** worktree the pass creates and
REMOVES on every path including every refusal path: guard (9) compares worktree PATH SETS across a
prune, so a leaked registration would have traded one failing guard for another. The merge base
needed no new `GitOp`: `recreate_run_worktree` branches `rk-run/<item>` at `HEAD`, so the base IS
`HEAD` and ADR-0105 clause 5's closed write set stays closed. `kind = mutate` is refused **by
name**, because no `Toolchain` method applies a patch and Checkpoint 7a counts exactly six on that
port.

**Two latent defects found on the way, both invisible until a checkout.** `core.autocrlf=true` and
**no `.gitattributes` row covered `exit/`** — the directory is already MIXED in the working tree
(`cancelled-timeout/expected` CRLF, `no-record/expected` LF), so the smudge is observed rather than
hypothetical; and guard (10a) compared `"$(cat …/expected)"` to `"4"`, which strips trailing
newlines and **not** a trailing `\r`. Fixed both ways — a `-text` row per `.gitattributes`'s own
stated rule, and `tr -d` in the guard so it does not depend on that row. Verified by comparing
worktree bytes to the committed blob: the three `findings-block` files round-trip identically;
`pack.review.toml` is worktree-CRLF/blob-LF, which is exactly what its four `pack.*.toml` siblings
do.

**NOT delivered, and the checkpoint enforces it.** The `Slice-Verdict:` axis is untouched: guards
(6) and (7) require the ARMED lint to keep naming `Slice-Verdict` as the **sole** missing producer,
so sealing a slice here would turn two green guards RED. ADR-0160 stands whole.

**What this asserts nothing about.** The shipped tree. ADR-0156 clause 4's blockers stand, the
shipped review stage is still unreachable, and **`0 of 102`** shipped activities completing a
committing walk is unchanged. A fixture proves the wiring, not the catalog. ADR-0101 clause 5
holds: S-SUBTLE-CB is UNRUN and nothing here says anything about the gate's calibration.

**AND THE ADVERSARIAL PASS FOUND TEN DEFECTS IN THIS CHANGE'S OWN WORK.** Three refuting verifiers
ran at `5e67ac0` with the producer green, Checkpoint 7h passing whole, 115/238/102/25 clean, fmt and
clippy clean and four mutants RED. Everything below was in the tree at that point.

**Six in the records (`E-0162-1`), and the citation family is the one `s7h` shipped five errata
for.** The supersession's **stated warrant is false to the sentence it supersedes**: ADR-0162 says
in four places that clause 3 *"was reasoned from ADR-0160's slice-identity absences"*, and clause 3
says *"and the reason is this record's own blocker list rather than ADR-0160's"*. A **load-bearing
ellipsis** removed *"for ADR-0156's six blockers. This record does not merge the two:"* from the
ADR-0160 clause 5 quotation — the clause that cuts against the argument it was cited for.
*"Two of its forbids bullets"* is **one** of seven, and `E-0156-1` said "a bullet" and was right:
the batch contradicting itself, again. An **undated `source:` claim** was falsified by this
record's own decision. **Decision 8 is not delivered** — the lane loop's named instrument is
unmoved and ADR-0161's debt **stands**. And a quotation was **misattributed** to a clause that
gives no reason at all.

**Two things the record should have said about what the fixture proves.** The candidate and
reference worktrees are **checkouts of the same commit** — both `worktree add … HEAD` — so there is
no change under review, and the `Flipped` that reaches `policy::classify` is **manufactured** by the
stub's check on whether its working directory ends `.review-ref`. The stub says so in its own
header; the record did not, and **the record is what gets cited**. The wiring is real; the evidence
is manufactured. And a second run over the same repository would exit 4 **without dispatching**
(`is_committed`, AC-013), so *"the dispatch committed"* is a property of the fixture's freshness
rather than of the pass.

**Four in the guard, which was strictly weaker than the repo's own `code_lines`.** (10b) dropped
only `//`, so `/* Exit::FindingsBlock */` **survived** a guard whose own comment claimed *"prose
cannot satisfy this"* — while `tests/it/main.rs` has dropped `//`, `/*` **and** `*` all along. Both
patterns were **unanchored**, so `grep -v 'exit.rs'` matched content as well as path and would drop
a real producer in `pre_exit.rs`. Both anchored now, and **what the floor still cannot see is
stated rather than claimed away**. (10a) read `expected` on a different whitespace notion than the
test it guards, so `4 ` went red while the test stayed green.

**Three in the code, repaired rather than recorded.** `reference_worktree`'s refusal arm returned
without removing a registration `git worktree add` may already have made — a leak that turns guard
(9) red and trades one failing guard for another. `review_pass` read `block_set()` off the
**in-memory** `Confirmed`, so decision 6's *"and the journal is re-read"* described a behaviour the
code did not have; the re-read is implemented and refuses when fewer `review_block` rows read back
than findings blocked. And the `env: &Env` parameter was satisfied at **eleven** test call sites
with `Env::capture()`, breaking an invariant `tests/it/config.rs` states in terms — not inert,
because `run.rs` hands that map to a spawned child, and the ADR-0083 env lint is scoped to `src/`.

**And in the fixture bytes.** The `.gitattributes` row protected its own fixture and left three
identically-replayed sibling `frames.jsonl` unpinned; its rationale measured the **working tree**
and reported a property of the **directory**, when every index blob is LF and the "mixedness" was a
local smudge artifact. Widened to `exit/**`, seven copies normalised, all 24 now byte-identical to
their blobs.

⚠️ **One of the close's own measurements was wrong before it was right.** The first byte-fidelity
sweep reported all 24 files differing — because `glob` returns backslash paths on Windows and
`git show HEAD:a\b\c` silently returns empty. The instrument was broken, not the tree. *Verify the
operation, not a proxy for it.*

---

## D21-39 · ADR-0163 flips to `accepted` — `redkiln cost` is the ninth verb, the reading is journalled once per dispatch, and ADR-0131's first forbids bullet yields for one member

**Confirmed 2026-09-24 (dated line), at `s7i`'s close.** ADR-0163 flips `proposed → accepted`, `date: 2026-09-23`
unchanged. It was committed `proposed` at `a811cea` and revised in place through three refuting rounds while it
bound nothing; the text that flips is the one at the stage's close commit.

**The operator's decisions it records, each taken before the thing it authorised was built.** On 2026-09-23, at the
stage's open, between options put to the operator with the measured state in front of them: (1) a reading's owner is the `"item"`
input `verbs/run.rs` already binds, copied onto the payload — overturning the reserved-key route pre-taken and never
recorded at `s7h`'s close, on the measurement that the item already reaches the adapter; (2) **one row per
dispatch**, so `n` counts dispatches as `BatchDraw.n`'s doc says it does — ADR-0133 clause 2's *"once per reading"*
superseded in part; (3) `resets_at` as **integer unix seconds**, the value the platform sent — ADR-0133 clause 3's
`<RFC 3339 | null>` spelling superseded in part. On 2026-09-24, when the stage's first adversarial pass found that
ADR-0131 forbids *"putting an item id … inside `rk-types::Event.payload` for the purpose of addressing"*: (4) ADR-0163
**supersedes ADR-0131's first forbids bullet in part**, for `rate_limit_reading.item` alone, as an interim member
that moves to ADR-0131's reserved `item` tag when that wiring lands after Checkpoint 8 — with the record answering
each of ADR-0131's grounds rather than the convenient one, and saying in terms that it breaks the driver *"Waiting
accretes nothing"*.

**What was not the operator's and is recorded as found.** The payload gained `cli_version` because AC-227 requires it
of every harness event; `utilization` so that a flat reading is journalled rather than discarded; `windows: null` for
the flat shape because AC-374 forbids extrapolating one utilisation into two windows. `cost` counts interleaving over
**traffic** — foreign `dispatch` rows as well as foreign readings — because `throttle::attribute` sees only the
samples it is given. And ADR-0133 carries five errata (E-0133-2 … E-0133-6), the last correcting two of the first four.

**What the fixture proves and what it does not.** The shipped store holds **zero** `dispatch_completed` and **zero**
`rate_limit_reading` rows; `cost` on both shipped items prints the one empty line and exits 0, and Checkpoint 7i
asserts that line exactly. The dispatched case is a real `redkiln run` over a **synthetic** `rate_limit_event`
transcript followed by a real `redkiln cost`, and 23 seeded journals rendered byte-for-byte — the wiring, not the
account. **Phase-1 criterion (7) is computable and rendered over a fixture, and NOT met on the shipped tree**; `s8`'s
block still needs an `ITEM` that owns readings, and the shipped registry does not dispatch.

**Three refuting rounds, and the counts are the record.** The first found **thirty-six** defects in work that was
committed, green, checkpoint-passing and seventeen-mutants-RED — ADR-0131's conflict the load-bearing one, thirteen
surviving mutants, and a checkpoint grammar that let a delta through at `n < 10`. The second found **fourteen** in the
first's repairs — a warrant that answered two of ADR-0131's grounds and omitted three, a counter half-repaired again,
and a CR check that could not see a CR. The third found **seven**, all in record text. A fourth was not run: the third
round's findings moved no code and no guard, and the row says so rather than calling the pass dry.

---

---

## D21-40 · Step 8 re-opens: four operator decisions, the re-measurement they rest on, and row 40's "passed" corrected

**Taken 2026-09-24, at `s8`'s re-open over HEAD `4c282d7`, each before the thing it authorised ran.** A read-only gap
map (six mappers and a completeness critic, every claim tied to code or record text read at HEAD) was put to the
operator with the measured state, and four decisions were taken:

1. **Route: more sub-stages, on D21-3's precedent.** Criterion (1) is still not satisfiable at HEAD, and the reasons
   are structural rather than pack defects: nothing stages or lands the harness's work (both `commit_with_trailers`
   callers in `verbs/run.rs` pass `allow_empty` with no `git add`, and `GitOp::Integrate` has no caller); a lane does
   not cross processes (ADR-0157 decision 4, where the current stage lives is *"named here as OWED"*); the verify stage
   cannot walk (`toolchain:test` refused under ADR-0148, `toolchain:smoke` unmapped under ADR-0135); and the shipped
   registry cannot bind a dispatch (0 of 8 cards carry `permission_mode`, all 8 grant only `StructuredOutput`, no
   `.redkiln/schemas/`, 0 `model` rows). The owed decisions are put to the operator as a packet, and the new stages
   are authored after that.
2. **Spend: the real runs are authorised, gated on fixture proof.** The criterion-(1), (1b), (3)-operational and (7)
   runs may dispatch against the real account **once the sub-stages' fixture proofs pass**, and not before. That
   includes a FakeHarness end-to-end test of `verbs::run::run`, which does not exist today
   (`grep -rn "verbs::run::run(" crates/*/tests` finds nothing). If a real run is refused unexpectedly, the next step
   is to stop and report, not to spend again.
3. **Row 40's "checkpoint passed `d6160ff`" is corrected, not left standing.** s8 defines Checkpoint 8 as *"All six
   pass (with (1)'s two halves and (6)'s two lints), and the following remain true"*. What passed at `d6160ff` was
   the **mechanical block**. **Checkpoint 8 has NOT passed, and M5 is not reached.** This also corrects this file's
   own D21-6 passage, *"Clause 8's trigger is recorded here because it has half-fired … Checkpoint 8 **is** now
   recorded as passed"*: the trigger has **not fired at all**. ADR-0131 clause 8's *"after Checkpoint 8"* and
   ADR-0163's interim-key move both wait for the whole checkpoint. The D21-6 passage is not rewritten; this line
   supersedes its reading.
4. **Checkpoints 5, 7b and 7f are repaired before Phase 1 exits.** They are not carried as debt, even though none of
   them is on `s8`'s entry chain.

**The re-measurement, run from each stage file's own bytes** (extracted verbatim and run whole with Git Bash from the
checkout; the store was backed up first and restored after (2)'s `doctor --rebuild`):

| block | rc | result |
| --- | --- | --- |
| `cargo build --workspace` | 0 | |
| (4) | 0 | clippy green, and the `rk-clock-read/reintroduce` arm is red for the named reason |
| (2) | 0 | `rebuild_differential` + `stage_level_next_action` pass; `doctor --rebuild` exits 0 |
| (3) | 1 | the asserted half passes (`kill_and_resume_n_step`); the block then stops at *"set ITEM"*, because the operational arm rides on (1) |
| (6) both lints | 0, 0 | |
| Checkpoint 8 mechanical block | 0 | its two `FAIL:` strings are grep hits on its own guard's text, not failures |
| Checkpoint 5 | 1 | `ChangeSummary` is named outside rk-ports at **six** sites: four in the two adapters, and now two in `crates/rk-cli/tests/it/review_pass.rs` |
| Checkpoint 7b | 1 | `grammar-probe.py` reports `predicates parsed 278/279; failed 1/279` against the pinned 285/287 |
| Checkpoint 7f | 1 | *"the three conformance lints are not all ARMED"*: two now run and pass, because s7g and s7h discharged them. This is by design, as row 36 records |

Two things were found in passing. The mechanical block's `grep -rn 'four questions on twenty items' --include=*.md .`
also searches the run worktree under `.git/redkiln/worktrees/`, so a stale copy of `runbook/` is read as a hit.
And (2) running before (7) means `doctor --rebuild` replaces the store that (7) reads. Both belong to the `s8`
instrument amendments.

---

## D21-41 · The nine owed Step 8 decisions, taken on a refuted packet

**Taken 2026-09-24, after D21-40 and before any record or code they authorise is written.** Each decision was drafted
with two or three options grounded in the accepted records, and then given to an adversarial refuter, who checked
every citation against the full text. The refuters found real defects in every packet: missed binding ADRs (0084,
0109 d2, 0135 cl.5, 0162, 0007), erratum forms that ADR-0092 rule 1 makes supersessions, and false measured
claims. The corrected packets were put to the operator. They are committed whole as
`spikes/S8-DECISION-PACKET-2026-09-24.md`, which is the input to authoring the sub-stages. The ADR ids below are
allocated in the order the records are written, from **0164**.

| # | Question | Taken |
| --- | --- | --- |
| a | Where an item's current stage lives across processes (ADR-0157 d4, OWED) | **A stage-bearing `Story:` trailer on `rk-run/<item>` at every boundary, folded by `reconcile`**, seeding `through_step_seven` so `run` and `answer` both see it. This is ADR-0155 option 3, which that record called *"the honest long-term answer"*. The record supersedes in part ADR-0105 cl.3, ADR-0155 d1/d2/d5(a) and its two `Story:` forbids bullets, ADR-0157 d4-5 and its cross-process forbids bullet, and ADR-0109 d3's single-axis rule, unless the resume demonstrably reads `Gate:` as well. |
| b | What AC-473's "landed commit" means | **A kernel-made content checkpoint commit on `rk-run/<item>` that carries the tier-1 trailer. No integration in Phase 1.** Design §7.7's merge queue is Phase 2. This is execution under ADR-0105 cl.3-5 and ADR-0109 d2, with dated register notes on AC-245 (the untracked-file rule), AC-473 and AC-521. |
| c | The verify route and the adapter-identity carrier | **An rk-cli verify pass on ADR-0162's shape.** It is selected by the required kind `TestReport` and never by a stage literal. It runs `verify_report` on the adapter `detect()` chose, whole-suite under AC-328, writes a keyed `TestReport` with one `artifact.write`, and records the chosen adapter's name as the AC-473/AC-485 identity carrier. It discharges R-9 and routes s7d §7d.18's RAISED verify block. `SmokeResult`, and `ConfirmationResult` from bug-standard's baseline, are narrowed out as named and owed. **The operator approves the AC-473 register reading of "through the port".** |
| d | Step-executor scope | **The hybrid.** Build the complete reference resolver (D21-11): output binding, iteration, check evaluation, emit binding and test-selection resolution. Route `shallow_clone`/`verdict_record` through verb-registry rows, populate `.redkiln/processes/`, and wire the `change` deriver. Cut only the infrastructure-bound demands (SmokeResult, the ScanReport require row, BuildArtifact, ReleaseRecord, DebtRegisterEntry), each named and Phase-2-owed. Per-step agent specs are built only **with** ADR-0150 c4's step-grain key, or not at all. |
| e | Slice-Verdict and the tier-1 writer | **Both, in Phase 1. This is AGAINST the packet's recommendation**, which was to deliver Slice-Verdict now and the tier-1 writer after Checkpoint 8. Slice-Verdict: a declared `slice` key on the item record, refused by name when absent, never equal to the item id, one per fix-lane item, and sealed on both review outcomes including the `FindingsBlock` path. The ARMED lint is un-ignored in the commit whose real fixture run reaches the producer. The tier-1 writer is ADR-0158 decision 4's route: a tier-1 write method on `EventStore` with its fake and conformance row, called where the trailer commit is made and taking its `anchor_sha`. That is a port-surface change under ADR-0003. It makes AC-160..AC-163 non-vacuous, and ADR-0158's forbids bullet on claiming them yields only once they are measured. |
| f | The (1b) story lane | **A full lane entered at `triage`. The candidate is to harden `spikes/p9/packs/spike.toml`**, a Phase-2 story whose change stays inside `spikes/p9/`. `rk-story-codex-adapter` is retired: ADR-0007 (accepted) places Codex in Phase 4, and the conflict with ADR-0094 d2 is raised separately and not settled here. IntakeRecord and BugReport stay `source = "human"` (D12 #15), supplied through a pack-declared gate question whose option attaches the kind (ADR-0129 d3/d4); this is decided once for both lanes. The authored story is `user-story-ac`'s `Spec`, written by rk-store and journalled as `artifact.write`, and s8 (1b) asserts its digest there (ADR-0116 d3). Checkpoint 7i's empty case moves to a dedicated item that nobody dispatches, and its PRECONDITION guard is kept. |
| g | Card grants, tiers, schemas, gate expiry | **Least privilege.** Only the implementer writes: Read/Glob/Grep/Edit/Write/StructuredOutput under `acceptEdits`, with cwd set to the run-owned worktree. Investigator, reviewer, architect and spec-author read under `default`. Planner, operator and releaser get StructuredOutput only. No card gets Bash, Web or MCP tools. Three principle-7 guards are added: a closed-set `permission_mode` lint, an init equality check, and a mapping of `permission_denials` to a typed failure. Tiers: implementer and investigator sonnet/high, spec-author sonnet/medium, reviewer opus/high, architect opus/medium, the rest haiku. Schemas are Rust types in rk-types `EXPORTED` (ADR-0084), not hand-written JSON. `gate_expiry` is a per-class table the operator signs (AC-276). The record is accepted only after two live probes pass, and those probes run after the fixture proofs. |
| h | ADR-0080 d2 / AC-527 on quoted `findings.none(...)` | **Execute ADR-0080 d2 as written, in any quoting.** Repair `root-cause` and `differential-debugging`. Bring AC-527's THEN and Verified-by back to d2 by a dated amendment, recasting fixture 2. Run a d2-form, per-class assertion over `.redkiln/catalog`, and widen Checkpoint 8's guard. `adversarial-review`'s `continue` rows stay an open, named reading. |
| i | Checkpoints 5, 7b, 7f | **Proportionate.** Checkpoint 5: one ADR superseding ADR-0099 and ADR-0135 in part. It restates the rule as the TYPE's home and opacity and lets implementors of `Toolchain::change_summary`, including test doubles, name it in that signature. It ratifies D17-1 and roots the conflict in D17-9 by name. Checkpoint 7b: re-pinned to 278/279 with a record, with ADR-0114 d1's residual named as owed. Checkpoint 7f: execute D21-9. |

**What this makes.** The sub-stages follow from these decisions and are authored next. At least five are expected:
registry fills and the Checkpoint 5/7b/7f repairs; stage location and landing; the resolver and executor; verify,
Slice-Verdict and the tier-1 writer; and the story lane. Real dispatch stays gated on D21-40 decision 2.

---

## D21-42 · Twenty-four further decisions: every open question on the Phase-1 exit path, taken one by one

**Taken 2026-09-24, after D21-41, on the operator's instruction to avoid shortcuts and deferrals and to prefer
long-term investment.** Three read-only research passes, all at `c617889`, produced the agenda. The first
inventoried every open item on the exit path. The second found the full route for each of D21-41's deferrals.
The third grounded the real-run environment. Each decision below was put to the operator with its context, the
record text and a recommendation. **This entry supersedes D21-41 in part where it says so.** Where a decision moves
an accepted `## Decision` sentence, it is written as a supersession-in-part record (ADR-0092 r1); ids are allocated
from ADR-0164 in writing order.

**Two of D21-41's premises were wrong, and the research measured both:**
- **(d)'s "or not at all" branch for per-step agent specs is not live.** The mandatory implement stage's only
  Changeset producer (`tdd-red-green-refactor`) and (f)'s `user-story-ac` both need a second runnable agent step.
- **The resolver cannot evaluate the fix lane's checks against today's port.** 34 activities read tool
  `status`/`failed`/`passed` (`outputs.status` ×43, `report.status` ×22), and `red-check` expects a red suite.

| # | Question | Taken |
| --- | --- | --- |
| 1a | What `Toolchain::test` returns | **Widened to an outcome-bearing report** (`VerifyReport` plus passed/failed counts). A red suite is `Ok` with counts; `Err` is only cannot-run/timeout/killed (AC-331/AC-344). The method count stays six. **Supersedes D21-41(c)'s "through the port" register reading, which is withdrawn as unneeded.** RT-22's routing is reversed by record, and RT-6/RT-22/R-9 are discharged. `conformance.rs` gains `validate_report` per adapter (AC-329). |
| 1b | How smoke, audit and package reach the adapter | **A registry-backed generic port method**, filling design §4.5's `registry.toml` verb table (D21-10's gap). This is a seventh method: it supersedes ADR-0162's forbids bullet and ADR-0135's "six" in part, and Checkpoint 7a is re-run. |
| 2 | How `ChangeSummary` is produced | **Split the type now, and run S-BIN before exit.** The `Text` variant (typed diff plus manifest) is finalised now. The `Binary` variant stays sealed and non-constructible until **S-BIN runs as a Phase-1 sub-stage before Checkpoint 8** (about 10 Opus dispatches plus UE automation, METHOD-S §3/§7). Supersedes ADR-0099 cl.5 and ADR-0093 d4 in part, folded with (i)'s Checkpoint 5 record. |
| 3a | How multi-agent activities run | **Step grain in the runtime.** Per-step specs are carried on `RecordedSpec` (ADR-0138's warrant), each agent step commits under its own key, and re-entry resumes at the first uncommitted step. Supersedes in part ADR-0143 d2/d4, ADR-0150 d1 and AC-013's per-activation grain; discharges ADR-0143 c5 and ADR-0150 c4. `kill_and_resume_n_step` is re-proven at step grain, and the card digest enters a per-step memo key. **Supersedes D21-41(d)'s "or not at all".** |
| 3b | The step-grain key | `(item, activity@version, step_id, iteration path)`. The iteration path is the stack of for_each index, repeat count and loop-back count. |
| 4a | Where `${toolchain.*}` values come from | **A per-toolchain table in the 1b registry.** Each value carries its measurement as its warrant; an unresolved key is refused by name. Discharges ADR-0140 c4's OWED item. |
| 4b | Whether step `timeout_secs` is enforced | **Yes, as the tighter bound**: `min(step, construction)` is passed into the port call. Supersedes ADR-0140 c1 in part, and the 298 declarations become real. |
| 4c | How `selection` maps | **`"affected"` is the package-level affected set**: touched crates plus their reverse-dependency closure via `cargo metadata`, falling back to whole-suite (AC-328) with `basis_reason` recorded. `artifact:` and `${…}` forms go through `selection_of` and the resolver; `"e2e"` is declared per adapter or refused. |
| 4d | What a false check does after its step committed | **`on_false` is honoured as iteration.** A `{to,max}` loop-back runs under a new iteration path, keyword forms follow the schema, and exhaustion or no `on_false` refuses as `StepFailedAfterCommit` (ADR-0144). Committed steps stand. |
| 5a | Register contradiction C2 | **Surface, then restore.** `reconcile` emits a `structured_drift` *event* (artifact, anchor, both digests), and the render then restores the section. No drift field on the artifact. A dated AC-478 amendment removes only *"no comparison … no drift state"*. One record owns rk-store's render/write path (layout C, digest-stamped sections, the AC-122 single write path, one `artifact.write` per write). |
| 5b | Where rendered artifacts land | **Under the item's managed artifact directory in the run-owned worktree**, written only by rk-store. Decision (b)'s checkpoint commits them to `rk-run/<item>`. s8 (1b) asserts the `artifact.write` digest plus `git cat-file -e` on the run ref. |
| 6a | D21-41's pack cuts | **Reversed for five demands:** SmokeResult, ConfirmationResult, ScanReport, BuildArtifact and DebtRegisterEntry. For ScanReport, unrated advisories are never counted as 0 and the advisory-DB commit is recorded. BuildArtifact is a release build plus binary sha256. DebtRegisterEntry is an rk-store item write with provenance; human-sourced facets are null and go to `unknown_facet`. **Supersedes D21-41(c)/(d)'s narrowings.** |
| 6b | Release for a CLI | **A new `tag-release` activity.** ReleaseRecord is a release tag on the landed commit, plus the BuildArtifact digest and the rollback target (the previous release tag). The tag is local and never pushed by the run. It enters via ADR-0114 c3's one-run, one-activity trial. |
| 7 | "Every lane stage" | **Every `stage_order` stage whose rigor is not `forbidden`** is entered and carries its trailer. An empty stage records `stage_empty { reason }`. s8 asserts set and order equality mechanically from the lane's own trailer commits; the human READ stays for quality. |
| 8a | Git-first ordering | **The kernel owns it, through the journalled `GitOps` seam.** It commits (zero writes on replay), then appends the t1 and t2 rows with `anchor_sha`. `reconcile` heals a commit without a row. |
| 8b | Gate kind in git | **The `Gate:` value is widened** to carry kind and reach. Supersedes design §5.4's grammar row and ADR-0105 cl.3's value delegation in part. |
| 8c | Rebuilding t1 | **Moves into rk-adapter-sqlite** (`open_at` / `Capability::Rebuild`, the §7d.18 RAISED item). rk-cli's duplicated INSERT is retired, and criterion (2) is re-measured against ADR-0105 cl.9's real projection. |
| 8d | R-63 / AC-529 | **A `worktree_assigned` row is written at run start.** Every tier-1 write is checked against it and refused with a typed rk-git error on mismatch. Defaults applied: the one stage-less `Story:` mark is admitted read-only; ADR-0109 d3 is not superseded. |
| 9 | Landing staging | **The worktree diff plus named refusals.** Stage tracked changes and files the run created; the untracked baseline is snapshotted at worktree creation, and `.gitignore` is honoured. A secret-pattern or oversize path is refused by name with a gate-worthy exit. Committed through the `GitOps` seam, with a dated AC-245 reading note. The hand-off is the operator's merge of `rk-run/<item>`, which `explain` prints. |
| 10a | D17-3 | **One spec/pack schema record** fixes the three spellings and adds `rk_types::AcceptanceCriterion` (with ADR-0104's mutation field) to `EXPORTED`. |
| 10b | D17-4 | **Mint `verify_completed`**: report digest, selection with `basis_reason`, counts and adapter identity. It is AC-321's replay event and AC-473/AC-485's identity carrier. |
| 10c | §7d.18's verify block | **Claimed whole**: AC-320/338/339/341/348/349/353/355/359, each with a named instrument. Any id that cannot be built names its reason. A cold `cargo test --workspace` wall time is measured first. |
| 11a | adversarial-review's `continue` row | **Rewritten to re-execution**: *"no blocking finding survives re-execution"*, read from `review::confirm`. Version bump, changelog row, P6 lattice re-run with a non-zero pack count. Closes D21-2's RAISED reading. |
| 11b | The one-member `['Reproduced']` rows | **In scope.** threat-model, user-story-ac, vertical-slicing and instrumented-trace-analysis are rewritten to bullet-2 re-execution quantities. |
| 11c | ADR-0080 d3's threshold T | **Settled now, against the recommendation to owe it.** The severity vocabulary and T are fixed by record, and that record is built on a severity-labelling measurement over the F37/F39 review corpus rather than chosen without evidence. |
| 12a | Criterion (7) on real data | **An honest rendering passes**, with two assertions: owned readings == `dispatch_completed` for the item, and no rendering over n = 0. This replaces s8's unreadable AC-394 READ line. |
| 12b | AC-395 / AC-372 | **ADR-0163's interim key is extended** to `dispatch_completed` (`item`, `activity_id_at_version`), moving to ADR-0131's tag when that wiring lands. The per-activity breakdown and the status half are built now. ADR-0131's wiring stays the first self-hosted story, as its cl.8 intends. |
| 12c | doctor's journal | **Read from the store before any real run**; fixture mode keeps `events.jsonl`. The flat-reading silencing of `RK-D-HEADROOM-MARGIN` is fixed too. |
| 13a | Catalog-quality checks | **They run over `.redkiln/`** (rigor monotonicity, AC-527's d2-form lint, the lattice) and assert a non-zero count walked. `spikes/p9` remains only for tests that pin a historical measurement, named so. |
| 13b | The P6 lattice | **Ported to Rust** as a kernel test and/or `redkiln lint --lattice`, run in CI. |
| 14a | CI | **`ci.yml` green on the pushed branch, every job, both OS legs, is a Checkpoint 8 condition.** Each push is confirmed with the operator. |
| 14b | Line endings | **`* text=auto eol=lf` as the first line of `.gitattributes`, done FIRST**, before any sub-stage. By record, with ADR-0151's `-text` pins preserved; then renormalize, a one-time re-checkout on a clean tree, and the full suite proven green in the main checkout and a fresh run worktree. |
| 15a | Model identity | **Full model ids are pinned** through a pack `[models]` table. The id goes into `--model` and the card digest and memo key. `DispatchRecord` gets a typed `observed_model`, and the run refuses when the init-reported model differs from the requested one. |
| 15b | Tiers | **D21-41(g)'s table is kept** and pinned to `claude-sonnet-5`, `claude-opus-5-5` and `claude-haiku-4-5-20251001`. `Tier` becomes the open set of `[models]` rows. |
| 16 | CLI pin | **Moved once, at probe time**, to the installed version in D17-11's form (instrument re-tee, the constant with its prose and literal test, and a `live_dispatch_probe` re-run). **Auto-update is frozen for the whole real-run window.** `Refuse` stays. |
| 17a | Launch | **A committed launcher script is the only entry to a real run.** It refuses `CLAUDECODE`/`CLAUDE_CODE_*` and denylisted names, sets `CLAUDE_CONFIG_DIR` explicitly, freezes auto-update and captures the version. The adapter refuses unless init reports `apiKeySource == "none"`. |
| 17b | Account | **`~/.claude-personal`.** |
| 18 | The fix item | **`rk-fix-doctor-test-count` stays the untouched fixture item**, prose defect included, with a dated note. The run-preparation sub-stage presents a shortlist of **at least three behavioural defects**: reproducible by a failing `cargo test`, one crate, not in machinery the run exercises, and not scheduled for a sub-stage fix. The operator picks one, it is reserved in the ledger, and a fresh item is minted. |
| 19a | Caps | **The kernel enforces a per-item dispatch-count cap and token appetite**, refusing with a typed class. First values: 40 dispatches, and about 2× the happy-path token estimate. Both are revised and recorded from measurement. |
| 19b | Staging | **The first real fix-lane invocation stops at the analysis → specification boundary** for the operator to review. |
| 19c | RT-57 | **The scalar record is written and signed before any spend**, each value with a warrant, replacing the `NO RECORD` markers. |
| 20 | Failure telemetry | **The kernel appends `StepError::event()` for every dispatch**, so fake and real harnesses behave alike, and AC-033's breaker counts those rows across processes. |
| 21a | `gate_expiry` | **Drafted fail-closed** with exceptions justified; **the operator signs row by row** (AC-276). |
| 21b | Settings isolation | **An explicit minimal `--setting-sources`**, with probe 2 asserting tools and mode from the init line. |
| 21c | Guards | **Both built**: a doctor row refusing Bash/Web/MCP card grants, and ADR-0146's load-time lint. |
| 21d | Reads outside the worktree | **Deny rules plus probe 1's assertion. If the CLI cannot enforce them, the work HALTS** and the gap is reported as a containment finding. |
| 22 | Codex's phase | **Phase 4, after S-CODEX.** Supersedes ADR-0094 d2's phase clause in part toward ADR-0007 (accepted, unsuperseded), with dated corrections at design §10.5 and the two register sites. **E-0094-2 records that E12 was taken without ADR-0007's phase sentence in view.** |
| 23 | ADR-0114 d1 | **Supersede d1's second sentence in part** over the shipped pack, folded with the 7b re-pin, plus a dated F42-DRAFT correction. **Also repair catalog-b's four defects**: quote `'security-vuln'`, and convert three bare `on_false` names to `{to, max = N}` with N warranted by sibling convention. |
| 24a | ADR-0085 option headings | **Enforced by lint from ADR-0164 onward.** One dated erratum lists ADR-0132…0161 as non-conforming, and their accepted bodies are not edited. |
| 24b–d | Placement | **L3** (`rk-store` `conn.rs`'s poison unwrap) is fixed in the tier-1 sub-stage before the writer builds on it. **`terminal_read`** is fixed in the stage-location sub-stage with the first FakeHarness end-to-end test of `verbs::run::run`. **Tracker upkeep** goes in one hygiene commit at the start of stage authoring. |

**Order these impose.**
1. 14b's line-ending normalisation comes first.
2. The hygiene commit follows.
3. Then the stage files are authored.

The spend gates (the probes, the real runs and S-BIN's dispatches) remain behind D21-40 decision 2's fixture-proof
condition, and each real run is still a separate operator go-ahead in the moment.

---

## D21-43 · Twenty-eight decisions the stage authors surfaced, taken before any stage file is revised

**Taken 2026-09-24, after the ten sub-stage files `s8a`…`s8j` were drafted (8,060 lines) and before any of them is
revised or run.** Ten authors, each working against D21-40/41/42 and the tree at `ca8f387`, surfaced questions that no
ledger line took. Two of them showed earlier premises to be false:

- `user-story-ac` has `max_rigor = "recommended"`, so it is never admitted at the story pack's **mandatory**
  specification stage. D21-41 (f) and D21-42 11b both named it.
- Only two of the seven ADR-0080 rows have a lawful re-execution quantity. For the other five, 11b's wording would
  have made the rewritten check constantly true.

One is a possible architectural blocker. The run-owned worktree lives under `.git`, which Claude Code lists as a
protected directory. Each question was put to the operator with its measured premise and a recommendation.

| # | Question | Taken |
| --- | --- | --- |
| N1 | Can a `-p` child edit inside a worktree under `.git`? | **Probe now, as an explicit exception to D21-40 decision 2:** one haiku confinement probe plus its outside-`.git` control, before s8e–s8g are built. If edits are denied, run worktrees move out of `.git` by record. *The probe was prepared but NOT run: the session's permission layer refused to spawn the child from inside Claude Code. It is owed to the operator from a plain terminal, and s8e–s8g's revision waits on its result.* |
| N2 | (1b)'s Spec producer | **"The Spec the story lane's mandatory specification stage selects"**, which is `bdd-scenarios` today. No catalog edit. **Supersedes D21-41 (f) and D21-42 11b's naming of `user-story-ac`.** |
| N3 | `change` at triage, where no changeset exists | **`null` routes to AC-079's `unknown_facet` gate at triage, and the operator supplies it.** |
| N4 | Per-test pass/fail on stable libtest, which has no JSON | **A strict parser over libtest's `test <name> ... ok\|FAILED` and `test result:` lines, pinned by a golden fixture per toolchain version.** Unparseable output is Unverifiable, never green. |
| N5 | Two items in one store fold together | **An interim `item` key on every item-scoped event kind**, moving to ADR-0131's tag later as ADR-0163's interim key does; `query` honours `ByItem`. Supersedes ADR-0131's first forbids bullet in part, for those members. |
| N6 | The gate lifecycle in git | **Every gate transition (opened, answered, expired, voided) is a git-first tier-1 event** with a `Gate:` trailer carrying its state. `t1_gate` is append-only and state is a fold. Extends D21-42 8b's record. |
| N7 | The discarded reconcile, and replay's git journal | **The planner reads `reconcile()`'s output** (execution under ADR-0109 d1), and D21-41 (a)'s stage seed comes from its fold. **A tier-2 `git_op` row is minted** so replay serves git writes from the journal. Golden logs are read, by stated rule, as "no git writes". |
| N8 | The deadline format | **Integer unix seconds.** Supersedes design §5.4's RFC 3339 spelling in part. |
| N9 | The lane's end | **A terminal `Story: <project>/<item> closed` mark** carrying closeout's content checkpoint. |
| N10 | A committed step's unlanded edits after a kill | **Land at every committed step that changed the worktree.** The commit and the content checkpoint are one boundary act. |
| N11 | AC-127 against surface-then-restore | **`structured_drift` opens a drift gate that blocks the stage's exit until it is answered.** AC-125's "resolving" means answering that gate. |
| N12 | Owners | **`artifact.read`** is emitted by s8d's resolver through s8f's rk-store read seam. **AC-159** goes to s8f. **AC-372's** unfireable doctor rule is read, by dated note, as a test of `cost`'s renderer, with AC-415's red case. |
| N13 | tdd `red-verify`'s flatten | **A catalog edit to the existing `[*].tests_added` projection** (ADR-0035), under ADR-0072. |
| N14 | `kb:`/`checklist:`/`toolchain:<feed>` sources | **Declared `[checklist.*]`, `[feed.*]` and `[kb.*]` tables in s8c's registry.** Undeclared keys are refused by name, and content is authored only for the keys the lanes reach. |
| N15 | Step-grain key against ADR-0097 | **A separate commit key.** ADR-0097's memo key is untouched. |
| N16 | The 185 agent-step timeouts | **Carried by each per-step spec** (3a), resolved through 4a's table and bounded by the activity budget. Owner: s8d. |
| N17 | s8h Q1–Q4 | Q1: reuse `NotAdopted` and `NotIsolated { surface: Env }`; nothing superseded. Q2: `appetite.tokens` is the sum of `Usage`'s four counters, about 24M per pass at first. Q3: `Exit::Overage` (20). Q4: probe 2 is three dispatches, one per model id. |
| N18 | The five rows without a re-execution quantity, and 11c's corpus | **(C)** re-point `confirm` at the discriminating test for `differential-debugging` and `instrumented-trace-analysis`. **(D)** keep `check.failed == 0` for `user-story-ac` and `vertical-slicing`, reading bullet 2 as illustrative. **(A)** a threshold at 11c's T for `threat-model`. **11c labels F37's 188 findings; E3's six are excluded**, because the operator has seen their filled sheet. **Supersedes D21-42 11b's "re-execution quantities" for these five rows.** |
| N19 | S-BIN captures | **The operator captures by hand** on all five review trees through `/ue-verify`, with one procedure applied identically to clean and seeded trees. |
| N20 | S-BIN review-tree contents | **Backlog and status prose stripped from all five trees identically**, and recorded. |
| N21 | METHOD-S §3.4's two undecided outcomes | **Registered as undecided before dispatch.** If either occurs, the `Binary` variant takes the human-gate route. |
| N22 | Seed 5; S-BIN's gate; the CLI version | Seed 5 is **replaced by the registered procedure** before dispatch. S-BIN is gated on Checkpoint 8h plus its zero-spend dry run, confirmed on the B1 line. The CLI is **frozen from S-BIN on**, and the set never spans a version. |
| N23 | The five displaced TestReport activities | **Re-homed to the implement stage** as test-authoring activities under ADR-0072. Verify only measures. |
| N24 | AC-139's default for a missing verification | **Block (fail-closed)**, recorded as the human decision AC-139 requires. |
| N25 | Toolchain gaps | A per-toolchain **`setup` verb** at worktree creation (`pnpm install --frozen-lockfile --prefer-offline`, refused by name on failure). **`build` is widened to an outcome report** as 1a did for `test`. **`sca-scan` requires `unrated == 0`** or goes to `scan.accept-findings`. |
| N26 | The lattice's axes; AC-018 | **`rust` and a `surfaces` binding are added** by a dated METHOD-P amendment, with the old axes kept for the parity goldens. **AC-018's memo lint runs over `.redkiln/`**, and its 14 activities are repaired under ADR-0072 in s8b. |
| N27 | ADR-0134 clause 5 | **The reading is confirmed:** the clause is about 7f's old text. Re-scoping 7f is execution, not supersession. |
| N28 | Checkpoint 7c's re-run | **s8j re-runs it whole**, from a plain terminal through the launcher, with the live probes. |

**Integration calls taken by the lead under these decisions, and reported rather than asked:**
- Question and process loading has one owner. The loader and `.redkiln/processes/` are **s8d**'s. The pack-declared
  human-root questions are **s8j**'s. s8g's `.redkiln/questions.toml` is dropped in favour of s8d's loader.
- AC-133's judgement-enum registry is built once, by **s8g**. s8h consumes it.
- The Rust verb handlers are **s8c**'s record (V); the activities are **s8g**'s (R4).
- RT-59 is **settled by R-48** (2026-09-20), and s8j cites it rather than re-taking it.
- Every re-run of an old checkpoint that executes `redkiln run` (7f, 7g, 7h) is pointed at the fixture child
  through `[adapters] harness_binary` once s8h lands, so no checkpoint can dispatch to a real account.
- s8's three `redkiln run` sites go through s8h's launcher (s8j amendment).

---

## D21-44 · N1 measured: a `-p` child cannot edit a worktree under `.git`, so run worktrees move to `.redkiln/worktrees/<item>`

**Measured 2026-09-24 by the operator, from the operator's own shell, on D21-43 N1's authorisation.** The lead had
prepared the probe but could not run it: the session's permission layer refused to spawn a `claude` child from inside
Claude Code. Both runs used the same script. Each made one haiku dispatch per arm (`claude-haiku-4-5-20251001`) with
`--permission-mode acceptEdits --permission-prompts none --allowedTools Read,Edit --output-format json`, with every
`CLAUDECODE` / `CLAUDE_CODE_*` variable stripped, `CLAUDE_CONFIG_DIR=~/.claude-personal`, and a throwaway worktree
at HEAD holding `probe.txt = original`.

| run | arm | worktree | file after | `permission_denials` | cost |
| --- | --- | --- | --- | --- | --- |
| N1 | in | `.git/redkiln/worktrees/probe-n1-in` (**today's run-owned placement**) | **`original`** | **Edit AND Write**, both refused: *"this session has no approval surface for permission prompts"* | $0.0405 |
| N1 | out | `D:/repos/rk-probe-n1-out` (control) | `edited` | none | $0.0274 |
| N1b | redkiln | `.redkiln/worktrees/probe-n1b` (Claude-style, in-repo) | **`edited`** | **none** | $0.0274 |
| N1b | out | `D:/repos/rk-probe-n1b-out` (control) | `edited` | none | $0.0278 |

**What it means.** Claude Code treats `.git` as a protected directory, and under `--permission-prompts none` a prompt
becomes a denial. So **the implementer could never edit a file in any run-owned worktree as placed today**, and
every real run would have failed at implement. Each control succeeding in the same run rules out the probe
itself as the cause. Total spend: $0.123.

**The placement, the operator's choice on 2026-09-24.** Run-owned worktrees move to **`.redkiln/worktrees/<item>`**,
git-ignored. This mirrors Claude Code's own `.claude/worktrees/<name>` convention, which the operator has used without
trouble. The N1b arm measured it writable. The sibling-directory alternative was offered and declined.

**Consequences, owned by `s8e`**, which already owns D21-42 8d's `worktree_assigned` row that records the path:
- A record superseding in part ADR-0018's and ADR-0105's common-dir worktree placement, with dated notes on AC-518
  and AC-529's path wording. Ownership stays with the store row, so AC-529 stays real wherever the path is.
- `recreate_run_worktree` and every derivation of `<common-dir>/redkiln/worktrees/<item>` move to the new root.
  The one existing run-owned worktree, `rk-fix-doctor-test-count` at `73c087a`, stays where it is. It is the
  undispatched fixture item (D21-42 18), and its re-point is s8j's.
- Every recursive walk over `.redkiln/` as a whole excludes `worktrees/`, each with a synthetic control, because a run
  worktree is a full repository copy. This includes Checkpoint 8's `grep -r … .redkiln/` lines, the pack/catalog
  loaders if they recurse, and `doctor`.
- The worst-case path length is measured under the new root before the record is accepted, since ADR-0114's filename
  is about 130 characters.

`.gitignore` gains `.redkiln/worktrees/` in this entry's commit. That row is inert until s8e moves the root, and it
stops a stray probe or run tree from ever reading as an untracked change.

---

## D21-45 · Claude Code's worktree conventions, read from its docs and its own code, and what redkiln adopts

**Taken 2026-09-24, at the operator's request to ground D21-44 in what Claude Code has already learned.** Two
read-only passes fed it. The first read the official docs (`code.claude.com/docs/en/worktrees.md`,
`permission-modes.md`, `permissions.md`, `cli-reference.md`). The second read the installed CLI's own JavaScript,
extracted from the Bun-compiled `claude.exe` at 2.1.281 without running it. Every point below marked **measured**
was quoted from that code.

**Why D21-44's placement works (measured).** `checkPathSafetyForAutoEdit` splits the absolute path into
case-folded segments. The path is sensitive if ANY segment is one of
`[".git",".vscode",".idea",".claude",".husky",".cargo",".devcontainer",".yarn",".mvn"]` or `.config/git`. The sole
exemption is `.claude` followed by `worktrees`, at or below the working-directory root, and a second `.claude` inside
that is protected again. This matches N1's DENIED arm and N1b's writable `.redkiln/worktrees/` exactly.
**Standing limit:** inside a run worktree, `<wt>/.git`, `<wt>/.cargo/`, `<wt>/.claude/` and `<wt>/.husky/` stay
protected. An implementer edit there is denied, and s8h's typed `permission_denials` failure names it as a known
limit, not a defect.

**What Claude Code does (measured)**
- **Worktrees:** `.claude/worktrees/<name>`, on branch `worktree-<name>` with `/`→`+`. Created by `git worktree add
  --no-track -B <branch> <path> <base>` from `origin/<default>` (or HEAD under `baseRef: head`), with `LC_ALL=C`.
- **Name checks:** at most 64 characters; segments `^[a-zA-Z0-9._-]+$`; `.`, `..` and `.git` refused.
- **Symlink guards:** it refuses when `.claude`, `.claude/worktrees` or the target is a symlink (*"A
  repository-committed symlink at .claude … could redirect worktree creation outside the repository"*), and checks the
  realpath after creating.
- **Ownership:** a `CLAUDE_BASE` marker in the worktree's gitdir, plus `git worktree lock --reason "claude … (pid N
  start T)"`. Locks held by dead PIDs are released.
- **Removal:** only a clean tree with no commits beyond its base, carrying the marker and unlocked. Anything else is
  kept with a reason (*"N changed file(s) would be lost"*). It never removes worktrees it did not create.
- **Resume:** it verifies the worktree is still a separate checkout of this repo.
- **Carrying files:** nothing by default. `.worktreeinclude` copies files that are both gitignored and matched,
  skipping symlinks; `worktree.symlinkDirectories` is opt-in.
- **Refusals and cleanup:** it refuses repo-local filter drivers and `includeIf` it cannot disable, since LFS
  `--local` would yield pointer files. It writes a marked `# claude-code-runtime` block to `<commondir>/info/exclude`,
  never to `.gitignore`. On Windows it removes a junction or symlink, never its target.

| # | Question | Taken |
| --- | --- | --- |
| W1 | Claude's safety practices in redkiln's run-worktree lifecycle | **Adopt all:** item-id validation as a path and branch component (at most 64 characters, safe segments, no `.`, `..` or `.git`); symlink guards on `.redkiln`, `.redkiln/worktrees` and `<item>`, plus a post-create realpath check; an **`RK_BASE`** marker in the worktree's gitdir, so ownership is recoverable from git alone (ADR-0109 d4); `git worktree lock --reason "redkiln run <item> (pid N start T)"` with stale-lock release; a re-entry check (a separate checkout, registered to this repo); `LC_ALL=C` on every parsed git call; Windows removal never follows a junction, with a test. |
| W2 | Re-entry | **Reuse, never force-remove.** Re-entry reuses the registered worktree when its HEAD equals the `rk-run/<item>` tip. Uncommitted in-flight changes are discarded explicitly and journalled, which is N10's semantics. A tree without `RK_BASE`, or with commits the run ref lacks, is **refused, never removed**. Removal happens only when the tree is clean, landed, marked and unlocked. **This replaces `recreate_run_worktree`'s `worktree remove --force` + `remove_dir_all` at every run start.** It deliberately does not copy Claude's `-B`, which resets an existing branch and would destroy a resumable run. `--no-track` is adopted. |
| W3 | Ignored-but-needed files | **Honour the project's existing `.worktreeinclude`**, the same file with the same semantics (ignored ∩ matched, symlinks skipped), for files like `.env`. **Dependencies come through N25's per-toolchain `setup` verb.** No `node_modules` symlinks: they share mutable state across runs, and Claude leaves the feature off by default. |
| W4 | Git hygiene | **Both.** Worktree creation is refused, with a diagnosis, when a repo-local filter driver (for example LFS via `--local`) or an `includeIf` would yield pointer files. **`redkiln init` writes an idempotent `# redkiln-runtime` block to `<commondir>/info/exclude`** for target repos, and this repo keeps its tracked `.gitignore` row. |

**Where redkiln deliberately differs:** the branch prefix stays `rk-run/<item>`; runs fork from the frozen, CI-green
HEAD rather than `origin/<default>`; removal is by run state, not by age (there is no `cleanupPeriodDays` analogue);
and `settings.local.json` is not copied, because children run with isolated settings (s8h).

**Ownership:** W1, W2 and W4's worktree half are **s8e**'s (with D21-44's move). W3 and W4's `init` half go to
**s8c** (the `setup` verb and `.worktreeinclude` reading) and s8e (the carry at creation). `redkiln init`'s exclude
block is `s8e`'s too, since `init` already writes the hooks marker (RT-51).

---

## D21-46 · The revision round's decisions, two corrections to this ledger, and the integration calls

**Taken 2026-09-24, after the ten stage files were revised for D21-43/44/45.** The revision moved the files from
8,060 to about 11,300 lines, and all three runbook checkers pass. The revisers surfaced eight decisions for the
operator, two errors in this ledger's own entries, and cross-file conflicts that fall to the lead.

| # | Question | Taken |
| --- | --- | --- |
| R1 | s8e at 1,592 lines | **Split.** A new stage **`s8d-ii`, "Run worktrees"**, sits between s8d and s8e (the `7b-ii` precedent). It carries D21-44's relocation and D21-45 W1/W2/W4. Its checkpoint is s8e's entry gate. |
| R2 | `t1_gate`'s schema against N6's append-only lifecycle | **(A) A forward migration to an append-only transitions table.** The store is disposable by design (ADR-0131 d6; the deleted-database test), so old stores rebuild from git. |
| R3 | N5's interim `item` key against the golden logs' canonical bytes | **Carried outside the canonical digest**, as ADR-0131 places its tag set. Digests and golden logs are unchanged, and `query` still uses the key. |
| R4 | `tag-release`'s ADR-0114 c3 trial on both lane items | **Two trials, confirmed.** Each is one run of one activity, which is the clause's constraint. |
| R5 | Agent steps left at verify (AC-347) | **`integration-tests`' test-writing steps are re-homed to implement**, as N23. `migration-dry-run` stays a named residual, because no Phase-1 lane reaches it (`data_migration` is NOT MEASURABLE). |
| R6 | `smoke-test` has no check | **A check on `status == 'passing'` with `on_false = "fail"`**, as an ADR-0072 edit in s8g. |
| R7 | Unproduced inputs | **s8f builds the `toolchain:backlog` producer** (a store-route read of items) and **AC-127's vouched-root half** (ADR-0025 advisory drift, plus a doctor row distinguishing the two drift kinds). Unprovided feeds refuse by name, and the walk table records each activity that stops on one. |
| R8 | W4's filter-driver scope | **The effective config at every scope.** Refuse with a diagnosis naming the scope. BrittonGames' LFS filter is system-scope and would have passed a repo-local check. |

**Two corrections to this ledger, found by the revisers and recorded here rather than by editing the entries they
correct:**
- **D21-44 named the wrong records to supersede.** ADR-0018 and ADR-0105 name no worktree path. The literal
  `<common-dir>/redkiln/worktrees/<item>` is **ADR-0137 decision 3's**. What also moves in ADR-0105 is **clause 6** and its
  "reused worktree" bullet (D21-45 W2), and **clause 5**'s closed git-op list (the new lock and marker operations).
  s8d-ii's placement record supersedes those, not ADR-0018.
- **D21-45's premise that "`init` already writes the hooks marker (RT-51)" is false.** `init` writes no hook, and
  `# redkiln-hook` exists only as doctor's recogniser. The `info/exclude` runtime block is therefore `init`'s first
  write into a target repo's git admin area, and s8d-ii's record says so.

**Flagged for the operator's flip lines, not decided here:**
- ADR-0035's `[*].field` over a list-valued field is read as **concatenation in element order** (s8d R2; N13 assumes
  it).
- The run-worktree root must be git-ignored before creation proceeds, which makes AC-518's instrument depend on an
  ignore row. That is a behaviour no earlier line names (s8d-ii).

**Integration calls taken by the lead:**
- **One re-run convention.** `RK_CP_NESTED=1` means "running as another checkpoint's entry gate: skip your own nested
  whole re-runs". s8e's `RK_CP_ENTRY_ONLY` is folded into it, and every file adopts it.
- **One no-dispatch guard for replays.** `CLAUDE_CONFIG_DIR` points at an empty trap directory that must stay empty
  (s8g's form), and `REDKILN_LAUNCH_RECORD` is unset, so s8h's gate refuses before spawn.
- **The whole-suite floor chain** is recomputed once, stage by stage. Each file states its expected value as
  re-derived at stage open from its entry checkpoint's recorded count.
- s8h's launcher gains an **`answer` mode**, so s8j's hold answer is not refused for want of a launch record. s8i uses
  s8h's `wrap` mode for S-BIN's corpus runner.
- Checkpoint 7e's backtick-executing FAIL message, the same defect as 7h's (10a), is fixed by dated amendment in
  **s8a**.

---

## D21-47 · The adversarial review of the eleven s8 stage files: five operator decisions and the rules the fix round applies

**Taken 2026-09-24.** Twelve read-only refuters reviewed the files at `4a8c832`: one per file, plus a cross-file
critic. Together they found **16 BLOCKERs, about 80 MAJOR and many MINOR findings**. Most are checkpoint lines that
could pass vacuously or could never pass, hand-offs with no receiver, and spend that could happen before its gate.
Every D21 decision has an executing file (the critic's coverage matrix). Five findings needed the operator:

| # | Question | Taken |
| --- | --- | --- |
| V1 | The filter-driver refusal (s8d-ii) was inverted: it refused the working case | **Refuse only when pointer files would result.** That is, when a tracked path at the fork commit selects a filter whose driver is NOT effective at any scope, or smudge is skipped. Attributes are read from the fork commit. An installed, working driver (BrittonGames' system-scope LFS) is admitted silently. This **refines D21-46 R8**, whose intent was D21-45 W4's *"would yield pointer files"*. |
| V2 | The spike.toml story breaks byte-pinned p9 tests at its own verify | **Every test pinning a historical `spikes/p9` measurement reads the p9 tree at its pinned commit via git**, never the working tree. This is what "a test pinning a historical measurement" (D21-42 13a) means, and it closes the breakage class. The story candidate (D21-41 f) stands. |
| V3 | AC-102's >20% dead-activity kill line over the installed two-pack set | **The threshold applies to the registered pack set, with the pack count reported (SM-027).** The installed pair is reported with its count and does not fire the kill line; its unreachability is Phase-1 scope by design (ADR-0011, E7). |
| V4 | adversarial-review's `verdict` (step 3) reads an outcome that `record` (step 5) produces | **`verdict` moves after `record`**, as an ADR-0072 edit with a version bump, a changelog row and a lattice re-run. |
| V5 | ADR-0080 d2's *"something the **reviewer** varies"* against the replacements' loop-varied quantity | **Recorded as the operator's reading of d2:** a re-executed outcome that the reviewer's hypothesis drives satisfies "something the reviewer varies". Root-cause's derivation (s8b) is confirmed under it. |

**Rules the fix round applies, taken by the lead from the review's evidence:**
- **ADR-0105 clause 5 has one reading.** A git WRITE not on its closed list is a supersession in part. That covers
  s8f's `commit-tree`/`update-ref`, s8g's `Tag`/`Apply`, and s8d-ii's lock, unlock, marker, reset and clean. A READ is not
  a supersession; s8d-ii drops its reads from its supersession, and s8c's `LsFiles`/`CheckIgnore` stand as reads.
- **`artifact.read` has exactly one emitter: rk-store's read seam (s8f).** s8d's kernel function calls a named,
  non-emitting stub that s8f re-points. s8f's stage-open STOP is rewritten to match.
- **The optional-`null` rule cites ADR-0032's own Decision** (*"Absent `?` inputs are `null`"*, still standing).
  ADR-0103 decision 4 is predicate semantics. The consistency pass's re-point to 0103 d4 was wrong and is reverted
  in s8c, s8d and s8f.
- **Every checkpoint replay runs with a fake `claude` first on PATH that prints the pin**, as s8h's (10) does, as well
  as the trap `CLAUDE_CONFIG_DIR` and an unset `REDKILN_LAUNCH_RECORD`. Replaying Checkpoint 7d's Halt 2 line is then
  meaningful before the pin moves at s8j, and a spawned `claude` is proved absent rather than assumed. The same wrapper
  covers each stage's own process-level cargo runs, because `CLAUDE_CONFIG_DIR` in this session is the operator's real
  account.
- **The canonical fixture harness is `rk-fixture-child-claude`** (the `pack-min` fixtures' `harness_binary`). A shell
  replay points `[adapters] harness_binary` at `target/debug/rk-fixture-child-claude`.
- **Checkpoint 7c's whole re-run at s8j is split.** Its suite portion runs OUTSIDE `wrap`: no launch record, the trap
  config and the fake `claude`. Only its live-probe portion runs under `wrap`. Launch-gate tests build their `Env`
  explicitly, enforced by a lint.
- **Every real run gets its own block and its own go-ahead.** In s8, the fix-lane and story-lane runs are separate
  blocks. Each launcher line is preceded by mechanical pre-spend checks: the frozen sha is the CI-green sha, HEAD
  descends from it with the lane paths unchanged, the fixture-proofs evidence line exists, and RT-57 is `accepted`.
- **Records are located by the id recorded at write time**, not by title phrase or "exactly one superseding X"
  counts. Two records legitimately supersede the same ADR (for example, s8c's B and s8d's R1 both touch ADR-0140).
- **A time-bound guard in an earlier checkpoint is amended by the stage that moves its premise, in the same commit.**
  For example, s8e amends Checkpoint 8a guard (7) when it un-ignores the Slice-Verdict lint. The `RK_CP_NESTED` skip
  wraps every card or permission precondition.

---

## D21-48 · The final consistency pass, and the sixteen questions the s8 files left for the operator

**Taken 2026-09-24, after the D21-47 fix round and a final consistency pass over the eleven `s8` stage files.** The
pass adopted one shared no-dispatch wrapper for every checkpoint (`runbook/tools/nodispatch.sh`, with a compiled
fake `claude` from `runbook/tools/fake-claude.rs`), put every record id into its own checkpoint block as a literal,
recomputed the whole-suite floor chain once (740 → 749 → 774 → 843 → 901 → 947 → 1021 → 1105 → 1171 → 1222 →
1228+N → 1242+N, before this entry's changes), and inserted the eleven sub-stages into `INDEX.md` (rows 19–29,
Step 8 → row 30, entry gate Checkpoint 8j) and `PROGRESS.md`. A cross-file critic then found eight defects, which
the lead fixed. Among them, s8j A11 is reduced to a citation because s8h already amends Checkpoint 8a (11), and A10
gains the two card-stop amendments its premise move owes. The operator took the sixteen owed questions:

| # | Question | Taken |
| --- | --- | --- |
| X1 | s8d-ii: creation when `.redkiln/worktrees/` is not git-ignored | **Refuse by name.** This answers D21-46's flip-line flag. |
| X2 | s8d-ii finding 15: an effective filter driver whose command is not installed still yields pointer files | **A post-checkout scan.** After creation, the paths the filter selects are scanned for LFS pointer headers; any hit refuses by name and removes the fresh worktree. This completes V1's *"refuse only when pointer files would result"* by observation. |
| X3 | s8i: one seed missed, summary fields cited, no missing field named (no METHOD-S §3.4 branch) | **Registered before dispatch as a third undecided outcome, `undecided-partial`**, routed to the human gate, as D21-43 N21 routes the other two. |
| X4 | s8c: `implement_timeout` and `repro_timeout` have no dispatch-duration record | **4,800 s each, cited to F31/S-TYPED's slowest agent dispatch (4,561 s) as a borrowed class.** The record says so. Every fixture-proof and real dispatch records its duration, and the record names the rule that re-derives each key from its own class's recorded maximum once one exists. |
| X5 | s8d R2: ADR-0035's `[*]` projection over a list-valued field | **Concatenation in element order.** Nothing is flattened implicitly anywhere else. |
| X6 | s8d R2: `repeat.until` reading a later-declared step | **The repeat body runs from the repeating step through the last step `until` references**, in declaration order, one `repeat` frame per pass, and `until` is evaluated after the body's last step. The record names it as a schema finding. |
| X7 | s8d R3: `on_false = { to = "deviate" }` (0 shipped uses) | **Refused by name, naming ADR-0046's route OWED.** R3 states the narrowing of D21-42 4d. |
| X8 | s8d R2: 14 shipped forward refs are licensed by neither ADR-0042's Decision nor a repeat body | **R2's record supersedes ADR-0042's Decision in part:** an escalate gate whose re-entry resumes into the referencing step also licenses the ref. The lint widens to match. Any ref still unlicensed is a lint error, fixed in the catalog in s8d. |
| X9 | s8g R1: D21-46 R5 re-homes *"`integration-tests`' test-writing steps"* | **The whole activity moves to implement**, because stage membership is per activity. |
| X10 | s8e: ADR-0158's *"Writing `t1_*` rows from `rk-cli`"* against `doctor --rebuild` | **"From `rk-cli`" names where the write runs.** The adapter's `Capability::Rebuild` owns the SQL, so there is no supersession. The record states the reading. |
| X11 | s8e/s8f: a rebuilt gate `opened` row has `run_id = null` (the `Gate:` trailer does not carry it) | **Accepted as a stated residual.** RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar does not widen. |
| X12 | s8h: the argv spelling of D21-41 (g)'s read-only mode (`--help` on 2.1.281 lists `manual`; the binary maps it to `default`) | **The adapter emits the spelling the PINNED CLI's `--help` lists**, fixed at s8j's pin move (`manual` on 2.1.281). Cards keep `default`, and the init comparison reads `default`. The golden argv snapshots move once, at the pin move. Probe 0 confirms rather than decides. |
| X13 | s8c record B: the step bound's supersession breadth | **Record B supersedes, each in part and each quoted:** ADR-0140 decision 1, decision 2's sentence, decision 4, the first two forbids bullets, and ADR-0099 decision 2's *"under the same timeout"*. |
| X14 | s8c/s8d-ii: the owner of D21-45 W3's `.worktreeinclude` carry | **Confirmed:** s8c builds the reader, and s8d-ii calls it at creation. |
| X15 | s8c: `cargo nextest` (ADR-0094 decision 1: *"where present, recorded on the report"*) | **Install it and support it now.** s8c installs cargo-nextest on this box and in CI, parses nextest's output beside `cargo test`'s, and records on the report which runner ran. The install is s8c's act, at stage open. |
| X16 | s8i: no existing kernel gate path can carry `review_input`'s refusal without naming `ChangeSummary` | **A type-neutral gate path.** The kernel gains a gate request with a closed reason (`automated-verdict-refused`) and the summary as an opaque attachment it never types. ADR-0135's forbids bullet and Checkpoint 8a's ban stand. |

**Integration calls taken by the lead, recorded here:**
- `run::every_binary_spawn_withholds_or_supplies_the_launch_record` (s8h, mutant M26) is the single launch-record
  lint. s8j removed its duplicate.
- Checkpoint 8a (11)'s card guard is s8h's to amend (D21-47's time-bound rule). s8j A11 cites it.
- 8d-ii's and 8e (10d)'s card-mode stops are amended by s8j A10's commit, because A10 moves their premise.
- **Awareness only, no decision:**
  - a refused landing leaves an unreferenced loose object (s8f);
  - after s8c, plain `doctor` reports its unmeasured keys as Errors until later stages cite them;
  - `actions/checkout@v4` fetches depth 1, so s8b adds `fetch-depth: 0`, and s8j's first CI run measures git in the
    `rust:` image and `safe.directory`.

---

## D21-49 · A checkpoint that asserts `accepted` stays red until the operator's flip line, and the chain waits for it

**Taken 2026-09-24 by the session, under the operator's standing AFK rule of 2026-09-24** (execute `s8a`…`s8j`; records
stay `proposed`; never flip an ADR; never write a go-ahead line; stop when nothing further can proceed without the
operator). It records how an s8 checkpoint is run and recorded while its own records are `proposed`. It flips nothing
and requests three flips.

**The measured premise.** Checkpoint 8a guard (3) asserts `grep -qxF 'status: accepted'` for each of its three records.
Checkpoints 8d-ii, 8f, 8g, 8i and 8j assert the same (8c admits `^status: (proposed|accepted)$`). 8f also requires a
ledger heading that flips each record, and 8h requires one record to *stay* `proposed`. Every later checkpoint re-runs
the previous one whole as its entry gate, and `RK_CP_NESTED=1` skips only guard (11)'s whole re-runs, never guard
(3). So while `s8a`'s three records are `proposed`, **every checkpoint from 8b onward is red at its entry gate**.

**Options, each grounded against the full text** (a three-drafter, three-judge panel; the judges' lenses were hard-limit
fidelity, Halt 8 and evidence quality, and the operator's long-term workflow):

### Option — A: a faithful red plus a flip-simulated reading to proceed on
Run each checkpoint twice: faithfully (red at guard (3)), and in an overlay whose record status lines read
`accepted`, then proceed on the second. **Refuted.** A status-only simulation stays red at 8d-ii (the back-links move
in the flip commit). A green 8f simulation needs a fabricated flip heading in this ledger, which is an operator line
even in a throwaway tree, and every nested re-run from 8g onward inherits that need. A blanket flip reads falsely at
8h. And "proceed on the simulated reading" makes a probe an entry gate over a tree that was never committed. Scores
4.5 / 4 / 5.

### Option — B: amend each guard to admit `proposed`
**Refuted.** It widens an operator gate the stage files built on purpose (*"Checkpoint 8a asserts `accepted`, because
every checkpoint amendment below rests on one of them"*). That is Halt 7's regex widening, and a WARN line that cannot
fail is Halt 8's shape (f). Applied uniformly, it would also open s8j's pre-spend check that RT-57 is accepted. Scores
1.5 / 1 / 2.

### Option — C: execute and record, then stop at the flip (TAKEN, unanimously: 8 / 8 / 7)
- `s8a`'s work is committed with its records at `proposed`.
- Guard (2)'s literals are written, so the faithful run of Checkpoint 8a reaches guard (3) and fails there. It is
  recorded as rc 1, the FAIL line quoted, and END-OF-BLOCK absent.
- A **labelled DIAGNOSTIC** copy, with guard (3)'s status test alone neutralised, is run under the same wrapper in the
  same overlay worktree to give the complete blocker list for guards (4)–(11). It is never recorded as the checkpoint
  and never an entry gate.
- The PROGRESS row reads BLOCKED on the operator's flip. It never reads passed or closed.
- **No `s8b` work is committed before the flip**, because its entry gate replays 8a whole and would be red.

**What the operator does next.**
1. The dated ledger line flipping ADR-0165, ADR-0166 and ADR-0167, and the flip commit.
2. Re-run Checkpoint 8a whole, from its stage file's bytes, in a fresh check-out-then-overlay worktree under
   `runbook/tools/nodispatch.sh`. Guard (11)'s whole runs need no plain terminal: none of 7i, 5, 7a, 7b and 7f
   carries a `CLAUDECODE` guard.
3. The tracker row moves to passed, and `s8b` opens.

**The same stop recurs**, and the operator may prefer to batch it:
- at 8d-ii (status and back-links together in the flip commit);
- at 8f (status plus the ledger heading 8f's guard reads);
- at 8g;
- at 8i and 8j, whose pre-spend checks also need RT-57 accepted.

**A corpus inconsistency is reported, not resolved.** INDEX's human-needed list says of the s8 sub-stages that *"None
of them names a person"*, but from 8b on every entry gate transitively waits on an operator flip.

---

## D21-50 · The corpus replay suite has been red since ADR-0164, and the six recording sources are pinned to the golden log's bytes

**Taken 2026-09-24 by the session, under the operator's standing AFK rule of 2026-09-24**, at `s8a`'s stage open,
before any `s8a` work. The BEFORE readings found something the stage file did not foresee.

**Measured.**
- Checkpoints 5 and 7b, each run whole under `nodispatch` at `b8e7aa9`, fail on `cargo test --workspace --features
  corpus` exiting 101, before the lines `s8a` repairs.
- rk-kernel fails **9** `replay::` tests. The recording derives run_id `implement-87544dc047ecc61c`, and the committed
  golden log is `implement-bf1d8cec3b22d288`.
- The recording `include_str!`s six sources (`crates/rk-kernel/tests/fixtures/catalog-select/{impl-report,
  change-summary,change-summary-ue,changeset,spec-authoring}.toml` and `fixtures/packs/incident-deviation.toml`).
  The log was blessed over their CRLF-smudged working bytes, while their index blobs were always LF. ADR-0164's
  `* text=auto eol=lf` (`ac10470`) made them LF.
- ADR-0164's evidence (740 / 0 / 9) was the **unfeatured** suite, and `ci.yml` never runs `--features corpus`
  (*"The corpus-bound suite is NOT here"*). So no instrument saw it. *(This corrects the drafting premise that CI Linux
  would always have been red: no CI leg runs this suite at all.)*
- Probe: with the six restored to the recorded bytes, `cargo test --workspace --features corpus --no-fail-fast`
  gives rc 0, **757 passed over 44 results**.

### Option — A: pin the six sources to the golden inputs' own git objects (TAKEN, 8.5 / 8 / 8)
Six `-text` rows go below ADR-0151's row, and each source's index entry is set to its golden input's blob id
(`1682fd98`, `93f8f8c5`, `64784b9f`, `820d848f`, `fb1de1a4`, `7f8bcc55`), copied index to index, never from a
smudged tree. `facets.toml` and `artifacts.toml` get **no** row: row 1 already fixes them as LF, and `-text` would
stop normalising a CRLF edit to a production registry. A new **unfeatured** guard,
`recorded_log::the_recording_sources_are_the_committed_logs_inputs_byte_for_byte`, holds all eight inputs byte-equal
to the golden copies:
- red at `b8e7aa9` (six differ, *"line terminators only"*);
- green after the pin;
- red again for a Python LF mutant of `changeset.toml`, and green after the restore from its blob.

`git -c core.autocrlf=true` and `=false` `checkout-index` of the six are byte-identical, which is ADR-0151 decision
4's method. No run_id, `RECORDED_RUN`, golden-log byte, CORPUS_PIN or production code moves.

### Option — B: re-record the golden log from LF sources
**Rejected.** It moves ADR-0151 decision 3 (*"The digest is NOT re-blessed and the directory is NOT renamed"*), which
is a supersession, and it needs a second CORPUS_PIN move, which is the operator's. `committed_root()` is in-crate, so
the corpus copy would go silently stale.

### Option — C: normalise terminators inside the run_id derivation
**Rejected here.** It is a schema decision (every run renames) and a supersession of ADR-0151, and it needs a pin
move. Its concern is real and is **recorded as owed, not repaired**: `rk-cli` builds `RunInputs` from `read_to_string`
of `.redkiln/*.toml` with no normalisation, so a CRLF editor moves a user's run id. That is a production exposure for a
future record.

**Records.**
- ADR-0164 (still `proposed`) takes a dated in-place correction before acceptance. Its evidence gains the corpus
  measurement, and its *"None exists today"* is corrected to name the six rows. No Decision clause or forbids bullet
  moves.
- ERRATA gains **E-0151-1**: ADR-0151's *"No other test digests bytes under that tree … this is a latent exposure and
  not a live defect"* did not hold for the recording's sources.
- **ADR-0164's flip must wait until its evidence carries the corpus-suite measurement**, which the correction supplies.

---

## D21-51 · ADR-0165 flips to `accepted` — `ChangeSummary` splits, and the operator settles the `cargo metadata` clause and the range grammar before it does

**Confirmed 2026-09-25 by the operator, in the session executing `s8a`, at `s8a`'s close.** ADR-0165 flips
`proposed → accepted`, `date: 2026-09-24` unchanged. It was written `proposed` at `s8a`'s build and revised in place
through fifteen adversarial refuting rounds while it bound nothing; the text that flips is the one in `s8a`'s work
commit. The operator reviewed a walk-through of all three `s8a` records before confirming, and asked that the run
finish and the `cargo metadata` item be settled first. This lifts D21-49's stop for `s8a`.

**Two settlements the operator took on 2026-09-25, folded into ADR-0165 before the flip.**

1. **ADR-0094 decision 1's `cargo metadata` clause.** It says the Rust adapter does *"`classify` and
   `change_summary` over `cargo metadata` and the changeset"*, and the adapter has read no `cargo metadata` since
   `s7c` Slice 6, a deviation carried as a dated OWED note. A three-drafter, three-judge panel scored the options:
   - **Option — supersede, narrowed (TAKEN, 8 / 8 / 8).** ADR-0165 decision 9 supersedes that clause in part, only
     as far as `classify` (a function of the path and its bytes, which spawns nothing) and the `Text` arm (a
     function of the range, read from the object store). It decides neither the `Binary` arm's inputs (S-BIN,
     `s8i`) nor what `cargo metadata` is read for (`s8c`'s record T). The OWED note's `classify`/`change_summary`
     half is discharged; RT-26's `--list` note is not.
   - **Option — implement the read now (3 / 3 / 3).** Measured vacuous: `--no-deps` output carries no `OUT_DIR`, the
     run route's `target_directory` is the adapter's scratch directory outside the project, `classify` already marks
     `target/` Generated, and `cargo metadata` reads the working tree, not the range.
   - **Option — split the two methods (5 / 5 / 5).** Its implemented half is the vacuous case above, and its
     reversed form is the supersession plus a package attribution nothing reads yet.

   Invariance tests (with and without a `Cargo.toml`; a mutated working tree after the commits) and a code-only spawn
   lint over `classify.rs`, each shown red by a mutant, pin the record's claims.

2. **The range grammar.** A range is exactly `<base>..<head>`. A single revision refuses by name: under step-grain
   commits a changeset spans several commits, so reading `R` as `R^..R` would yield a `Text` that omits earlier
   changes. `.redkiln/catalog/change-summary.toml` and `change-summary-ue.toml` bind `range = "artifact:Changeset.sha"`,
   a single sha. Their catalog edit to a two-dot range (under ADR-0072) is **owed to `s8d`**, which owns the resolver
   that binds it, and `s8d` carries a dated note naming it.

**What stays open, named.** Fifteen refuting rounds applied about 105 fixes; no round came back dry (the last five
found 7, 5, 2, 5 and 4, almost all minor). Declined as operator- or stage-level: a `Text` assertion in the shared
conformance body; widening E-0085-3 to name ADR-0090 and ADR-0117; a joined-statement reader for the `ChangeSummary`
ban; a sweep over ADR-0093 decision 4 lines, which would be time-bound; and the whole-suite floor, kept at the stage
file's 749 while the suite reads more.

---

## D21-52 · ADR-0166 flips to `accepted` — the v0 pack's tag condition was met over the shipped pack

**Confirmed 2026-09-25 by the operator, with D21-51.** ADR-0166 flips `proposed → accepted`, `date: 2026-09-24`
unchanged. It supersedes ADR-0114 decision 1's *"The two F42 parse failures and three `on_false` defects are
repaired at Step 7e before the tag"* in part: `redkiln-pack-v0` stands, `catalog-b`'s four remaining defects are
repaired at `s8a`, and Checkpoint 7b is re-pinned to the probe's clean reading (279/279, 0/279, 99/99, rc 0).

---

## D21-53 · ADR-0167 flips to `accepted` — the Codex adapter is Phase 4, after S-CODEX

**Confirmed 2026-09-25 by the operator, with D21-51.** ADR-0167 flips `proposed → accepted`, `date: 2026-09-24`
unchanged. It supersedes ADR-0094 decision 2's *Codex is Phase 2* sentence in part, toward ADR-0007, and E-0094-2
records that DECISIONS-2026-09-12 E12 was taken with ADR-0007's phase sentence out of view.

---

## D21-54 · S-SEV labels a pre-registered sequential sample of whole dispatches, not all 188 findings

**Taken 2026-09-25 by the operator, in the session executing `s8b`, before `spikes/METHOD-SEV.md` exists.** The
stage-open measurement put S-SEV's first pass at about 3.5–5 hours of hand labelling (188 findings, mean about 1,318
characters each), plus a re-label of 38 at least 48 hours later. The operator: *"Spending 5 hours labelling doesn't
sound like a productive use of my time"*. A judge panel (a grounder, three adversarial judges, a synthesis) set out four
options.

- **Taken:** a sequential sample, inside `s8b`. S-SEV (D21-42 11c, `s8b` §8b.6) labels F37's adversary findings by
  **whole dispatch**, keyed `(run_key, step.attempt)`, because 33 dispatches share 31 `run_key`s and `step.identity` is
  not unique.
- **Stage 1** labels every finding of one dispatch per item: 10 dispatches, drawn by a seed recorded in
  `spikes/METHOD-SEV.md` before the sheet exists.
- **If stage 1 decides:** it shows variability on both A2 (at least one discordant finding, as §8b.6 defines it) and A3
  (`0 < k < 10`). The variability question is then decided over the corpus by containment, because §8b.6 defines
  inadmissibility as *"zero discordant findings in both directions"* and *"`k = 0` or `k = 33`"*.
- **Otherwise:** the continuation, pre-registered in the same commit, labels the remaining 23 dispatches, and the
  measurement is §8b.6's original 188. A stage-1 constancy result is never reported as constancy over the 188.
- **A1's CANNOT-TELL bar** is a point rule on the labelled n, with its Wilson 95% interval reported. It is
  pre-registered as decidable by that interval only at 0–1 of about 57 (golden constraint 10).
- **The re-label stays at 38 findings**, drawn by a recorded seed from those labelled in stage 1, at least 48 hours
  after stage 1. A1's κ half is therefore the ruled one.
- **Supersedes D21-43 N18 in part: the n in *"11c labels F37's 188 findings"* only.** F37 as the corpus and the E3
  exclusion stand. D21-42 11c stands whole: T is settled on a measurement, inside `s8b`, and `s8b` stays open until the
  re-label.
- **Why not the others:**
  - "Owe the measurement" reversed 11c's *"now"* outright. It moved ADR-0080 d3 with no evidence, sent S-SEV to no
    gated stage, and its guards broke the nested Checkpoint 8b replays in `s8c` and `s8d`.
  - An inserted S-SEV stage before `s8h` would have bought calendar time by reversing 11c's *"now"* in part. The
    operator objected to hours, not calendar time.
- **Disclosed, not decided:** S-SEV measures the operator's normative labels, not the reviewer's emitted `severity`, so
  it does not show that the shipped row varies at run time.

---

## D21-55 · AC-527's refusal 2 is evaluated per conjunct required for truth, over the two bindings it already makes

**Taken 2026-09-25 by the operator, in the session executing `s8b`, before the rule was changed.** The stage-open
measurement found a gap in §8b.4.
- §8b.4 says *"Operands other than `findings` bind as they need to, so that `X and <falsified>` evaluates `false` under
  ADR-0121's absorption"*. But `quality_block`'s context binds two names, and `eval.rs`'s `and` arm propagates a
  left-hand error before absorption can act (`let left = self.truth(l, scope)?;`).
- So the stage's own conjunct fixture, `check.failed == 0 and findings.none(evidence in ['Reproduced'])`, errors with
  `UnknownField("check.failed")` under the witness. The stage never said what "bind as they need to" means.

The ruling:
- **Refusal 2 is evaluated per conjunct required for truth.** The walker that serves refusal 1 flattens the predicate's
  top-level `and` nodes, looking through grouping but not through `or` or `not`.
- Each conjunct that reads `findings` is evaluated **alone**, over the two bindings the rule already makes: `findings`
  bound to a single-class witness `[{evidence: C}]` for C in {`Reproduced`, `MutationSurvived`}, and `rigor` across the
  live levels. The predicate is refused if some such conjunct is `false` in some cell.
- A conjunct that errors is not refused; it is counted as indeterminate. **No other operand is bound**, and the module's
  recorded rejection of null-binding stands.
- This is ADR-0121 decision 1's absorption, read as a rule about values: a false conjunct makes the conjunction false
  for every value of the operands left unevaluated.

Four corrections and follow-ons:
- **A reason in §8b.4 is corrected.** It says *"a joint two-member witness does not refuse `findings.none(evidence in
  ['Reproduced'])`"*. In this evaluator `none` is `any.not()`, so the joint set does refuse it. The per-class witness is
  kept for a predicate that only a single-class set falsifies. A seventh fixture,
  `findings.any(evidence in ['MutationSurvived']) or findings.none(evidence in ['Reproduced'])`, must FAIL, so that
  mutants M2 and M3 are observable.
- **No empty-set control.** Under this reading it would be dead logic that no fixture separates.
- **The p9 control counts one refusal per step.**
- **`findings == null`** (a non-empty witness list `== null` evaluates to `Tri::Null`) is carried to its own item and
  is not decided here.

What moves and what does not:
- AC-527's dated amendment under ADR-0092 d2 item 2 transcribes refusal 2 at conjunct grain.
- No accepted ADR's Decision sentence moves: ADR-0080 d2, ADR-0103 d4, ADR-0121 d1 and ADR-0136 d3 are all untouched.

*(dated 2026-09-25, `s8b` refuting round 1)* The seventh fixture (`terminal_condition_witness_is_per_class`) is spelled
`findings.any(evidence in ['MutationSurvived']) or not findings.any(evidence in ['Reproduced'])`. It is logically the
per-class shape named above (`none` is `any.not()` in this evaluator), without the named form's text, so that Checkpoint
8b section (4)'s d2-guard exclusions do not grow. The ruling is unchanged.

---

## D21-56 · Axes::shipped() keeps lattice.py's inert `auth` boolean, guarded in code

**Taken 2026-09-25 by the operator, in the session executing `s8b`, on the panel's recommendation.**
- **The gap.** `lattice.py`'s `BOOLS` binds `auth` on every context. `.redkiln/facets.toml` says *"The repo-wide `auth`
  boolean is WITHDRAWN, not retuned"* (ADR-0089 decision 2). `s8b` §8b.2 defines `Axes::shipped()` as the registered
  set plus exactly `rust` and a `surfaces` binding, so the `.redkiln/` reading carries a withdrawn column.
- **The ruling.** `Axes::shipped()` keeps the column. It is inert because no `.redkiln/` predicate reads it, and that
  is guarded in code rather than asserted in prose. Inside
  `lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged`:
  - (i) the registry loaded from `.redkiln/facets.toml` declares no facet `auth`;
  - (ii) a predicate reading bare `auth`, loaded through the entry point `lattice::p6_lattice_over_the_shipped_pack`
    uses, is refused as an unregistered facet. It is neither evaluated nor resolved as the surfaces literal `'auth'`.
- **Mutant M30** (the shipped-pack load path skips the facet-registry root check) must flip that test.
- **Disclosure.** METHOD-P Amendment 19 discloses the column.
- **What does not move.** No test is added or renamed, K_TESTS stays at 30, and no ADR Decision sentence or register
  THEN moves.
- **Declined:** dropping the column by projecting the transcribed eight-row covering array onto seven columns. It is
  figure-neutral, but it subtracts from N26 and renames a checkpoint-pinned test.
- **Raised, not decided:** `.redkiln/facets.toml` makes its seven booleans tri-valued, but no context in either axis
  set binds a boolean to null. `lattice.py`'s `unknown` sweep ranges over the ordinals only. That arm is untested under
  `Axes::shipped()`, and N26 did not rule on it.

---

## D21-57 · AC-303's confirm half is the check that consumes the reviewer's Findings by data dependence, not by naming `findings`

**Taken 2026-09-25 by the operator, in the session executing `s8b`, on a judge panel's recommendation.**

**What was found.** After the ADR-0080 d2 rewrites, `review::ac303_confirm_step_survives_escalation` is red over
`.redkiln/catalog`: in the review family, code-review → adversarial-review loses `verdict`. The localisation family is
silently blind, because all three members dropped to zero confirm checks together. Spec-form is green for the wrong
reason. The cause is `rigor.rs`: it identifies a review block's confirm half by `quality_block::reads_findings` over the
check's `expr`, but ADR-0081 d1 says *"the `check` step that consumes it"*. A d2 remedy consumes the reviewer's
Findings through a re-execution outcome, one or more steps removed, so the instrument tested a name where the record
says consumption.

**Ruled.**
- A `check` step is a review-confirm step when some binding its `expr` reads is bound to a `${<step>.outputs…}`
  reference, and that step reaches a `kind = agent`, `role = reviewer`, `outputs = value:Findings` step through a chain
  of `${<step>.outputs…}` input references.
- The walk covers the whole declared graph, back-edges included. An `artifact:` or `toolchain:` input breaks the chain,
  and a binding that is declared but not read does not count.
- Dependence reached only through a back-edge counts, under D21-47 V5's reading, which this row extends to ADR-0081
  d1's "consumes". The survey reports each confirm step's dependence class, `forward` or `back-edge`, so the looseness
  that back-edges bring stays visible.
- **Carrier.**
  - The rule reads the activity TOML through `toml_subset`, and `catalog::Step` is not widened, because of ADR-0148's
    forbids bullet on `inputs` and s8c's fence until `s8d`.
  - When `s8d`'s R2 routes parsed `inputs` onto the kernel, the rule moves onto R2's carrier.
  - `s8d`'s licence lint must keep the `${adversary.outputs?}` back-edges that the localisation and spec-form chains
    depend on.
- **The register.** AC-303's owed dated amendment (ADR-0092 d2 item 2) is written in `s8b` and names this rule. Its
  GIVEN/WHEN/THEN does not move.
- **Tests and floor.** Four tests join (T1–T4), with T1 shown red first, and three mutants. The floor moves by four.

**Raised, not ruled (open).** code-review's and checklist-review's `findings.none(severity in ['blocking'])` involves
no re-execution. ADR-0080 d2 bullet 1 reads *"no finding at or above threshold T survives re-execution"*, not "no
finding exists". Whether those rows conform to d2 is its own question, and nothing in `s8b` depends on it.

---

## D21-58 · Jev becomes S-SEV's rater of record; a stability test replaces the operator's re-label

**Taken 2026-09-26 by the operator, in the session executing `s8b`, after all 188 first-pass labels existed and before
any of Jev's answers had been opened.**

**Why.** The operator labelled all 188 findings (corpus `a2f4fab`, `1b3e102`), then said: he found it *"too difficult
to make an honest determination on the context supplied"*, because he *"would need to read and understand all the
related code artifacts in the change set, and read the scope and AC for each change"*. He *"doubt[s] the accuracy of
my labelling answers"*, and he cannot leave the stage idle for the 48-hour re-label gap. Jev's blind pass (Amendment
4) had already run over all 188: 188 answers, all from `typesafe/jev-1.13-20260917`, at a cost of US$0.0068. No answer
had been opened.

**Ruled.**
- **Jev is S-SEV's rater of record.** The operator's labels are kept, committed, and reported beside Jev's, with his
  own statement that he doubts them.
- **This supersedes `s8b` §8b.6's *"Rater: the operator, alone"* and D21-54's re-label of 38 findings after ≥48 h.**
  D21-54's sequential design stands, and the continuation has run, so the n is 188.
- **A1 is redefined for a model rater:**
  - CANNOT-TELL ≤ 10% of 188, as a point rule with its Wilson interval, as registered;
  - **and stability.** Every finding is asked again twice unchanged, and once with the three options in reversed
    order. Cohen's κ between the first run and each of the three re-asks must be ≥ 0.61 in all three.
- **A2 and A3 are unchanged, computed on Jev's labels** by the committed `score.py`.
- **Confidence is a sensitivity analysis, not the verdict.** A2 and A3 are recomputed over the findings whose Jev
  confidence is at or above its median, and reported.
- **What the result can claim, stated in the record.**
  - Jev saw exactly the context the operator saw, so it settles the severity *a context-limited, independent rater*
    assigns, not the findings' true severity.
  - There is no ground truth to validate Jev against.
  - Its confidence is TypeSafe's spread statistic, not a calibrated probability.

**Declined.**
- Shortening the operator's re-label: κ would be weaker, and likely inflated by memory. *(dated 2026-09-26, `s8b`
  refuting rounds 8–9: the option stands declined as the operator declined it; only the reason is qualified. With a
  constant first pass, κ at any re-label length was 0 or undefined, so "weaker" did not describe it; see the
  disclosure below.)*
- A richer-context Jev arm, with the item's spec and ACs added: offered, not taken now.

**Disclosure** *(dated 2026-09-26, `s8b` refuting round 8; a fact about the ruling's timing, not a new ruling)*.
The operator's first pass was **constant**: all 188 answers are `BLOCKING` (56 at corpus `a2f4fab`, 132 at
`1b3e102`, both committed before this entry's commit `c04b316`). METHOD-SEV §5 draws the re-label *"from those
labelled in stage 1"*, so under the method as registered at `bbf809c` the intra-rater κ had p_e = p_o: κ = 0 if any
re-label answer differed from `BLOCKING`, and undefined otherwise. §6.1 reads *"A κ that is undefined … has not been
shown ≥ 0.61, and A1 fails"* and *"If either half fails, the vocabulary is **NOT SETTLED by this measurement** … the
stage STOPS"*. So when this entry was taken, the registered method could end only in a NOT SETTLED STOP, and the rater
change moved the outcome from that STOP to ADR-0168's Branch 2. Amendment 5 was committed before any of Jev's answers was
opened (Jev's runs were written unread, Amendment 4), but it was **not** blind to the operator's. The reasons given above are the
operator's; this disclosure does not replace them. **Owed:** the operator's dated confirmation that the switch stands
with this known (AFK review packet, confirm list).

---

## D21-59 · `threat-model` declines the quality block under ADR-0080 decision 4; a richer-context Jev arm is the only road back

**Taken 2026-09-26 by the operator, in the session executing `s8b`, after ADR-0168 (S-SEV, Branch 2) was written and
before it was accepted.**

**Why.** §8b.5 prescribed D21-43 N18 (A), *"a threshold at 11c's T for `threat-model`"*. ADR-0168 decision 2 settles
*"no admissible severity threshold on this evidence; bullet 2 governs"*, so (A) has nothing to bind, and §8b.5's STOP
applies. `threat-model`'s graph (model → review → policy → residual-gate → emit) has no tool step. So none of bullet
2's quantities exists in it: no re-execution outcome (`review::confirm` gives `Unverifiable::CannotRun` for a repro
with no `--` selection, ADR-0162 d4), no counted loop-back quantity the predicate grammar binds, and no computed delta.
Its v1 predicate, `findings.none(evidence in ['Reproduced'])`, is the only row AC-527 (as amended, D21-55) still
refuses.

**Ruled.**
- **`threat-model` declines the `adversary → policy` block (ADR-0080 d4: *"Every activity that wants the block writes
  it"*).** v1 → v2 (ADR-0072): the `policy` step is deleted, and `model`'s `findings = "${review.outputs?}"` input is
  deleted with it, because ADR-0042 licenses a later-step ref only under an `on_false`/`repeat` targeting the
  referencing step. `review`, `residual-gate` and `emit` are unchanged. The reviewer's findings stay evidence and
  reach the human as `residual-gate` attachments. **That gate is not the block's terminal condition** (AC-527
  amendment: a human gate answer *"is none of d2's quantities"*).
- **ADR-0080 d1 is read as retaining the construct, not as an obligation on each activity.** Its text is *"Retain the
  `adversary → policy` block. PC6's encapsulation is not withdrawn"*, and d4 and its Negative (*"coverage becomes an
  authoring outcome rather than a default"*) license declining the block. No record superseding d1 is owed.
- **Supersedes D21-43 N18 (A)** for `threat-model`, **and D21-42 11b's *"In scope. threat-model … rewritten"*** for
  `threat-model`. N18 superseded only 11b's *"re-execution quantities"*.
- **Re-entry is by record, and by measurement only.** A loop returns to `threat-model` only by a dated catalog bump
  (ADR-0072) that cites a new, pre-registered measurement:
  (i) the richer-context Jev arm D21-58 offered (the item's spec and ACs added), run under a METHOD-SEV amendment
  committed before any answer is opened, showing A3 variable. That would be a new record superseding ADR-0168 in
  part, per its decision 3. D21-57's open question on severity-only rows with no re-execution would still bind;
  or (ii) a bullet-2 quantity measured as not constant and as converging within `max` over replayed `threat-model`
  iterations.
- **Not a new reading of ADR-0080 d2.** No terminal condition is written, so d2's forbids have no subject in this row.

**Declined.**
- N18 (D) extended to `threat-model` (a self-check over the reviewer's findings, `check.failed == 0`). The architect's
  own grade would terminate the block. It would need ADR-0168 d2's enumeration softened. Condition (c) passes through
  `model` alone, and nothing shows that `SelfCheck.failed` varies. It remains the operator's fallback if the feedback
  loop is wanted.
- Bullet 2's delta through a new `findings_delta` verb. Pass 1 is constant-false by construction (ADR-0055: `prior` is
  null). It ends on stagnation, not resolution. The identity key would decide the outcome. It owes a verb (ADR-0135), a
  type (s8h) and a new resolver self-reference.
- Keeping v1 as a named exception. Checkpoint 8b requires `refused=0`.

---

## D21-60 · D59b is owed to `s8d`'s `forward_ref_lint`, not built in `s8b`

**Taken 2026-09-26 by the operator, in the session executing `s8b`.**

**What happened.** D21-59's control mutant D59b restores `threat-model`'s `model` input `findings =
"${review.outputs?}"` without the loop that licenses it. It survived the whole workspace: 784 passed, 0 failed.
- ADR-0042's Decision is *"A ref to a later-declared step is legal iff that step or one after it has `on_false`/`repeat`
  targeting the referencing step"*, and nothing in `crates/` enforces it.
- The gap predates `s8b`. D59b was a speculative control with no predicted kill, not a guard for a claim `s8b` makes.

**Ruled.**
- The licence check is **`s8d`'s**, already planned under D21-48 X8 as `forward_ref_lint`. That covers its three
  tests, the escalate re-entry arm, the removal of the four unlicensed shipped refs, and mutants M31–M34.
- It is **not** moved into `s8b`. A check on ADR-0042's letter alone would refuse the four shipped refs X8 removes, and
  would contradict X8's escalate arm.
- `s8d` gains a dated note: one more must-refuse fixture shaped like D59b (a back-ref to a later step with no
  `on_false`/`repeat` targeting it), plus D59b as a mutant row its lint must kill.
- In `s8b` §8b.8, D59b's row reads "owed to `s8d` (D21-60)". It is not a surviving mutant of any `s8b` claim, so it does
  not block `s8b`.

**Declined:** moving `forward_ref_lint` forward into `s8b`. The operator first ruled "fix it in s8b"; then, shown that
`s8d` already carries the whole design, chose this.

---

## D21-61 · `s8b` closes D21-2's RAISED reading: the catalog's quality instruments read `.redkiln/`, the lattice is Rust, and ADR-0080 d2 is executed

**Taken 2026-09-26 by the session executing `s8b`, at the stage's close, as `s8b` §8b.7 prescribes.** It records and
cites. It re-takes no ruling, flips no record and writes no go-ahead line. Every figure below is quoted from the stage
file's recorded readings, the tree or the committed records; none is new here.

S-SEV record: ADR-0168

**Why.** D21-2 left a reading open: *"whether an `expr` on a `kind = "check"` step carrying `on_false` is the
*terminal condition* ADR-0080 decision 2 forbids is a reading of the record, RAISED and not taken"*. D21-41 (h)
executed ADR-0080 d2 *"as written, in any quoting"* and left `adversarial-review`'s `continue` rows *"an open, named
reading"*. D21-42 11a rewrote that row to re-execution and *"Closes D21-2's RAISED reading"*; 11b brought the
one-member rows into scope, and D21-43 N18 ruled the five rows that have no re-execution quantity. (D21-42 11c is the
separate ruling on ADR-0080 d3's threshold T, recorded below.) `s8b` executed those rulings over the shipped pack. The stage's BEFORE reading was 7 live `expr` rows carrying
`findings.none(evidence in [` in `.redkiln/catalog`, and no catalog-quality instrument on the tree read `.redkiln/`.

**Recorded — D21-2's RAISED reading, closed.**
- An `expr` on a `kind = "check"` step that carries `on_false` **is** the terminal condition ADR-0080 d2 forbids. The
  reading is settled by ADR-0103 d3, by ADR-0080's Context, and by design §1.2's *"P7's policy predicate"* row. It is
  not a new ruling.
- **`adversarial-review`'s disposition is (i), taken by D21-42 11a:** *"no blocking finding survives re-execution"*,
  read from `review::confirm`'s outcome. `verdict` reads `record.blocking.len() == 0` over `${record.outputs}`
  (`Confirmed::block_set()`), and `on_false = "continue"` is unchanged. `adversarial-review` 3 → 4.
- **The one-member rows are refused under the forbids bullet's `findings.none(evidence in [...])`.** These are the
  four `['Reproduced']` rows of D21-42 11b: `instrumented-trace-analysis`, `threat-model`, `user-story-ac` and
  `vertical-slicing`. **Disclosure, as the packet asked:** this **extends F37's measurement**, which was of the
  two-member predicate on one graph. AC-527's refusal 2 carries the extension (D21-55's per-class witness). It was not
  measured on the one-member rows' own graphs.

**Recorded — D21-43's rulings on §8b.1's four questions, cited and not re-taken.**
- Q1 and Q2 were closed by **N18**. The corpus is F37's 188 findings, with E3's six excluded (D21-54 then made it a
  sequential sample; it continued to 188). The five rows took N18's letters: (C) for `differential-debugging` and
  `instrumented-trace-analysis`, (D) for `user-story-ac` and `vertical-slicing`, and (A) for `threat-model`, which
  D21-59 has since superseded for that row.
- **The (D) rows read d2 bullet 2's dash list as illustrative**, as N18 requires this entry to record.
  `check.failed == 0` is `SelfCheck`'s count, over a self-check step that reads the loop-back target's outputs. It
  is neither a loop-back count, a re-execution outcome nor a findings delta, and it meets condition (a) only on that
  reading.
- Q3 and Q4 were closed by **N26**. `rust` and a `surfaces` binding are added for the `.redkiln/` reading by a dated
  METHOD-P amendment (below), and the memo lint runs over `.redkiln/` after its 14 activities are repaired.

**Recorded — `root-cause` detached from F37's artefact.** `root-cause` goes 4 → 5: `policy` binds
`{ report = "${confirm.outputs}", findings = "${adversary.outputs?}" }` with `expr = "report.failed > 0"`. The quantity
is bullet 2's re-execution outcome: `confirm` re-runs `${hypothesise.outputs.discriminating_test}`, and `hypothesise`
is the loop-back target. The changelog row reads *"Detached from F37's pinned artefact `306b58ba7504` (v4); F37's
measurements are of v4."*

**Recorded — the parity figures, and the Python's demotion.**
- `rk_kernel::lattice` ports `lattice.py` v2 (D21-42 13b). `lattice::p6_lattice_parity_over_the_p9_tree` and
  `lattice::p6_validate_parity_over_the_p9_tree` run the port over `p9_pin::root()` on `Axes::p6_registered()`. They
  assert equality, byte for byte and digest tags included, with the goldens `p9-run.txt` (298 lines) and
  `p9-validate.txt` (17 lines).
- The goldens were re-derived from the Python at `P9_PIN` (`ca8f3874ed9185b419b00b21b33713ba128de217`) with Python
  3.14.6. Their tags are `cat:fdf842cca5b9 fac+art:592fa138d68d pk:cfcaa966656b`, where run10 reads
  `cat:2d7f63b432cc`, because ADR-0164 renormalised line endings and the tags hash raw bytes. Every run10 figure is
  reproduced:
  - prereg-8 / all-12 `pack-default` unsat `432/155520 (0.3%)` / `432/235872 (0.2%)`;
  - `dead=7/102`, and `dead=14/102` under the eight pre-registered packs;
  - ties `none`;
  - `validate: no errors` and `validate errors=0`.
- **The six named divergences** are `crates/rk-kernel/tests/fixtures/lattice/divergences.tsv`, rows `p9-run.txt`
  lines 113–118. They are the six `soft` cycle-example lines: `incident` (`hotfix -> build-package -> rolling-deploy
  -> rollback-mitigation -> hotfix`), `initiative` and `tech-debt` (`design-doc -> adr-record -> design-doc`), and
  `project`, `story` and `story-ue` (`design-doc -> data-migration-plan -> design-doc`).
  - Each row is held to the soft-cycle assertion: the same pack, the same kind (`soft`), and a closed cycle over
    optional-inclusive edges of activities that pack selected. The rows are not held to bytes.
  - Four of the six match byte for byte today.
  - The only assertion kind built is the soft-cycle example. A `HARD` example row or a figure divergence has no
    weaker assertion.
- **The Python is demoted, not deleted.** From the parity commit on, the instrument of record for AC-101–AC-104 is
  `rk_kernel::lattice`. AC-101 and AC-102 carry dated register notes naming the tests. `lattice.py` is not edited: it
  stays the oracle from which Checkpoint 8b section (7) re-derives both goldens on every run. Parity over `.redkiln/`
  is **NOT MEASURABLE** by this oracle: over a staged copy, `lattice.py` exits 1 with `KeyError: 'surfaces'` in
  `admissible`, so no 27th test joined.

**Recorded — METHOD-P Amendment 19** (`spikes/METHOD-P.md`, *"Amendment 19 (2026-09-25, D21-43 N26)"*, committed at
`bbf809c`). It amends the toolchain sentence **for the `.redkiln/` reading only**. `Axes::shipped()` is the four
registered toolchain sets plus `("rust", ({"rust"}, {"text"}))`, and a `surfaces` binding registered before the
shipped reading was recorded. **The `surfaces` enumeration is the empty set plus each value `.redkiln/facets.toml`
declares, as a singleton (`auth`, `flags`, `migration`, `payments`, `secrets`): six rows.** The registered axes are
unchanged for the p9 goldens. The covering array keeps its eight columns, including the inert `auth` column, which is
guarded in code (D21-56). The amendment moves no kill threshold.

**Recorded — the Rust lattice's wall time (SM-025).** [measured 2026-09-26 at `83d1268`, after Checkpoint 8b's formal
run, warm debug build, through `nodispatch`] `lattice::p6_lattice_over_the_shipped_pack` (the installed pair, all four
modes, `Axes::shipped()`), three runs by libtest's own *"finished in"*: **48.82 s, 49.00 s, 50.36 s** (median
**49.00 s**; wall including cargo's start-up 50.2–51.7 s, against 1.4 s for an empty filter). This is a DEBUG-profile
test run over the shipped pack, not SM-025's instrument (`lattice_run_completed { duration_ms, … }`, not built) and not
the same input as the Python's figures, which the stage file records over the pinned p9 tree: about **2 min 10 s**
[2026-09-24] and **167 s** alongside a parallel cargo run [2026-09-25]. SM-025's stamped figure (Step 7e's Python
**97.3 s**) is not replaced by it; the instrument stays owed.

**Recorded — the three further edits, and the three hand-offs.**
- **N26:** the 19 `memo = false` lines in 14 activities are deleted (ADR-0097 d3). The 14 bumps are
  `blue-green-deploy`@3, `failover`@3, `feature-flag-kill`@4, `flag-progressive-rollout`@3, `incident-command`@2,
  `instrumented-trace-analysis`@3 (one bump, shared with its N18 edit), `migration-dry-run`@3,
  `migration-expand-execute`@3, `rate-limit-shed`@3, `rollback-mitigation`@3, `rollback`@2, `rolling-deploy`@3,
  `rolling-restart`@3 and `slo-monitoring`@3. `memo::ac018_live_verb_may_not_memoise` reads `.redkiln/catalog` and
  prints `activities=102 … violations=0`. `memo::ac018_frozen_p9_catalog_is_red` keeps 19 violations at `P9_PIN`.
- **N13:** `tdd-red-green-refactor` 2 → 3. `red-verify`'s `selection = "${red.outputs[*].tests_added}"` is read as
  concatenation in element order (D21-48 X5).
- **N25:** `sca-scan` 1 → 2. `clean` adds `and report.unrated == 0`, and the `exception` gate's `when` adds
  `or ${scan.outputs.unrated} > 0`.
- All told, twenty-two activities move `version`, one bump each, with 22 rows in the new `.redkiln/CHANGELOG.md`.
- **Hand-off 1, to `s8c`:** the `live` flag for the twelve verbs the deleted lines ran (`smoke`, `failover`, `metrics`,
  `flag_set`, `flag_rollout`, `trace_collect`, `migrate_dry_run`, `migrate`, `rate_limit`, `rollback`, `restart`,
  `slo_query`). The changelog lists them.
- **Hand-off 2, to `s8h`:** `RootCause.discriminating_test` (cross-file finding 5), which the (C) rows' `confirm`
  binds.
- **Hand-off 3, to `s8h`:** the `severity` field on `rk_types` `Finding` (cross-file finding 4). ADR-0168 names `s8h`
  its owner.

**Recorded — D21-47's readings as this stage applied them.**
- **V2.** The five tests that pin a historical p9 measurement read the p9 tree at `P9_PIN` through `p9_pin`, from the
  object store:
  - `rigor::rigor_monotonicity_p9_historical_2026_09_13`;
  - `memo::ac018_frozen_p9_catalog_is_red`;
  - `quality_block::ac527_frozen_p9_catalog_is_red`;
  - the two lattice parity tests.

  Every other reader of `spikes/p9` is inventoried in §8b.3, including the two readers the inventory
  gained at the stage's open: `tests/it/recorded_log.rs` (`facets.toml`, added at `bf12a1c`, `s8a`) and
  `pack_validation::tdd_red_verify_projects_tests_added_per_adr_0035` (`schemas.toml`). Neither is a pin.
  `ci.yml`'s checkouts take `fetch-depth: 0`. **Open item:** the inaccurate `SHIPPED_FACETS` name in
  `facet_registry.rs` and `selection.rs`, which point at `spikes/p9/facets.toml`, is left as it is.
- **V3.** The kill lines are thresholded over the eight pre-registered packs:
  - AC-101 at `pack-default` is 432/155520 (0.3%), inside the >5% line.
  - AC-102, pooled over all four rigor modes as its THEN reads, is 14/102 (13.7%), inside the >20% line. The
    `pack-default`-only reading is 17/102.
  - The installed pair is reported with `packs=2` and not thresholded: `pack-default` unsat 1296/279936 (0.46%), dead
    56/102 over all modes, and ties and hard cycles 0 (AC-103's zero tolerance holds over the pair).
  - The pair's list of 56 is not yet printed, as SM-027's pointer note discloses.
- **V4.** `adversarial-review`'s `verdict` follows `record`, giving the step order `clone`, `review`, `override`,
  `record`, `verdict`, `emit`. `quality_block::adr0072_d2_edits_are_versioned_and_changelogged` asserts the order
  (mutant M29), and so does Checkpoint 8b section (3d).
- **V5.** Condition (c)'s *"the loop"* is read as *"something the reviewer varies"*, and `root-cause`'s derivation
  above is confirmed under it. D21-57 extends the same reading to ADR-0081 d1's "consumes" (`forward=16 back_edge=4`
  over `.redkiln/catalog`).

**Recorded — D21-59, cited and not re-taken.** `threat-model` declines the `adversary → policy` block under ADR-0080
d4.
- It goes v1 → v2: the `policy` step is deleted, and `model`'s `findings = "${review.outputs?}"` back-ref goes with it
  (ADR-0042).
- It supersedes D21-43 N18 (A), and D21-42 11b's *"In scope. threat-model … rewritten"*, for that row.
- ADR-0080 d1 is read as retaining the construct. Re-entry is by record and by measurement only.
- The disclosure §8b.7 once owed on (A)'s dropped *"survives re-execution"* is moot, because the (A) form was never
  written.
- The binary prints `ac527: catalog=.redkiln/catalog checks=93 refused=0 indeterminate=39` and `d2-constancy: rows=6
  walked=66 valued=52`.
- D59b is owed to `s8d`'s `forward_ref_lint` (D21-60) and is not an `s8b` survivor.

**Recorded — the Python oracle's non-determinism.** `cycles()` iterates a `set` of strings, so the six `soft … ->`
example lines follow hash randomisation. A fresh run printed `soft project design-doc -> data-migration-plan ->
design-doc`, where run10 has `data-migration-plan -> design-doc -> data-migration-plan` [measured 2026-09-24]. Every
Python run now sets **`PYTHONHASHSEED=0`**, and `python --version` is recorded beside the goldens
(`p9-python-version.txt`: `Python 3.14.6`), because string hashing is a property of the interpreter. The six lines
are the six named divergences above.

**Recorded — ADR-0168 and the S-SEV path.**
- ADR-0168 is **`proposed`, and on the operator's flip list**. It records Branch 2: the vocabulary is
  `blocking | advisory`, and *"no admissible severity threshold on this evidence; bullet 2 governs"*.
- It supersedes ADR-0080 d2 bullet 1 in part, and it names `s8h` as the owner of `Finding.severity`.
- Its required lines read `**S-SEV labelled:** n = 188 findings over 33 dispatches; rule: continued to 188`, with the
  labels commit `a2f4fab` and the stability commit `285e011`. Checkpoint 8b section (9)'s `SSEV_ADR` is `ADR-0168`.
- **The D21-58 disclosure (refuting round 8).** The operator's first pass was constant: 188/188 `BLOCKING`. So under
  METHOD-SEV as registered, A1's κ half could not pass, and the rater switch moved the outcome from a NOT SETTLED STOP
  to Branch 2. Amendment 5 was committed before any of Jev's answers was opened, but it was not blind to the
  operator's answers.
- **Owed:** the operator's dated confirmation that the switch stands with this known, before ADR-0168 flips (§8b.10;
  the AFK review packet's confirm list).

**Recorded — the refuting rounds (§8b.11).** Refuting rounds **1–9** ran. Rounds 1–7 each had five finder
dimensions, and every finding went to two refuters. Round 8 had five finders and sent each blocker or major to two
refuters; round 9 re-read the same delta plus round 8's fixes the same way.
- Rounds 1–7 ran over the stage's work, with majors by round of 3, 2, 2, 2, 2, 1 and 0.
- Rounds 8 and 9 ran over the S-SEV delta `d6acfba..62b634a`. Round 8 found two majors, both fixed.
- **Round 9 was dry: no blocker and no major.** The remaining minors are named in §8b.11, recorded and not fixed, and
  §8b.10's open items stay open with their sources.

**The guard form `s8j` adopts** is Checkpoint 8b section (4), run here: ADR-0080 d2's named form in any quoting and
with any member list, on a CODE line, anywhere in `crates/` or `.redkiln/`. The `.redkiln/` walk excludes
`worktrees/` (D21-44), and the `crates/` walk does not. The by-path exclusions and their reasons are those of §8b.7.
This entry does not edit `s8`.

**The floor.** The named selection is 34 tests (`K_TESTS`). The whole-suite floor is 784 (Checkpoint 8a's recorded
755, plus 25 new tests, plus D21-57's four).

Checkpoint 8b: formal whole run from the stage file's bytes (extracted block sha256 `66d21ba404b77224…`, 504 lines)
at `83d1268`, in the main checkout, under `nodispatch`, with `CARGO_BUILD_JOBS=4 RUST_TEST_THREADS=4`: **rc=0, FAIL 0,
`CP8B-END-OF-BLOCK reached`**, Checkpoint 8a re-run whole inside it (section (0)). The first two runs stopped at
section (0), each on one of Checkpoint 8a's ADR-census guards that ADR-0168's writing had left stale in the preamble
(the next-free number and the census); both were corrected by dated edits (`5a40990`, `83d1268`). Companion
whole-suite reading at `83d1268`: **784 passed / 0 failed / 9 ignored over 44 results** (the block asserts the
floor of 784 and ≥ 44 results but prints no count). This is the count the next stage's floor is re-derived from.

---

## D21-62 · `s8c`'s four stage-open blockers: foreign lines are skipped and counted, nextest's list failure is a named refusal, the adapter reads each advisory database's HEAD itself, and tsc's 1 is cannot-run

**TAKEN PROVISIONALLY under the AFK rule of 2026-09-26, pending the operator's confirmation.** The operator is away
and their standing instruction for a blocker a stage file did not foresee is: measure, read the binding records,
ground 2–4 options, run an adversarial judge panel, take its recommendation provisionally, prefer the most reversible
option. Each blocker went to three lensed judges (binding records, reversibility, engineering) and a synthesiser. The
full panel text, with every judge's recommendation and the dissents, is `spikes/afk-panels/D21-62-s8c.md`. Every
ruling is carried by a record this stage writes `proposed` (T = ADR-0169, V = ADR-0170), so none moves an accepted
ADR's Decision; each deviates from `s8c` stage-file text that the file itself delegates to those records.

**Why.** Package P1 met four things at stage open [measured 2026-09-26, raw streams in the session scratchpad and
summarised in the panel file]:
- **A.** §8c.1 rule 2 (*"Any other non-empty line in the region refuses."*) refuses this repository's own green
  whole-suite stdout: 39 `RK-D-<rule> ok` lines leak into the rk-cli `it` binary's record region from
  `rk_cli::verbs::doctor::run`, which writes through `std::io::stdout()` in-process. With those 39 skipped the tally
  is 784/0/9 over 44, Checkpoint 8b's reading. §8c.1 says the stage stops.
- **B.** `cargo nextest run` 0.9.146 exits 104 over the fixture project: *"creating test list failed"*, because
  `zz_decoy` (s7c's AC-329 decoy, `harness = false`) does not speak `--list`.
- **C.** Offline, cargo-audit 0.22.2 emits `last-commit: null` and cargo-deny 0.20.2 emits no database member, so
  §8c.4's *"read from the report's own database member … refuses by name"* would refuse every scan.
- **D.** tsc 5.9.3 `--noEmit` exits 2 for every diagnostic (config errors included) and 1 for CLI/project errors;
  §8c.2 maps both 1 and 2 to `Ok(failing)`.

**Taken provisionally.**
- **A (record T).** In a libtest block's record region, a non-empty line matching neither the record grammar nor the
  slow notice is a FOREIGN line: skipped, never counted. Rules 3–6 (header count, summary agreement, `--list`, exit
  agreement) still refuse, and a slow notice naming an unrecorded test still refuses. `Tally` gains `foreign_lines`
  with the first foreign line as evidence; `this-repository.tally.json` pins 784/0/9/44 and `foreign_lines` 39 (first
  at 287). The golden stays as captured. The nextest parser stays strict. The rk-cli stdout leak is handed on under
  §8c.12 as a hygiene item, not fixed here. Dissent: the reversibility judge preferred keeping rule 2 strict and
  giving `doctor::run` a writer.
- **B (record T).** The two rc-104 streams become refusal goldens (`ParseRefusal::ListFailed`, mapped to
  `Transport { Decode }`, never green); the `-E 'not binary(zz_decoy)'` streams are committed as parse-only goldens
  that carry the build.rs decoy on nextest's status stream; the red-suite nextest test runs over §8c.3's workspace. No
  runtime fallback, no `.config/nextest.toml`, `zz_decoy` unchanged. Dissent: the engineering judge preferred a narrow
  Detect-only fallback to `cargo test` on exit 104.
- **C (record V).** The adapter passes each tool an explicit database path (audit `--db`, deny a generated config's
  `db-path` with exactly one child) and reads that path's HEAD itself through one shared read-only `rk_git` head read,
  before and after the tool runs; it refuses by name if the path is not a git checkout, the HEAD moved mid-scan, a
  non-null report member disagrees, deny's child count is not one, or deny's pinned *"Opening advisory database at"*
  line is missing or names another path. The record shape (two pins) is unchanged. The same head read serves the
  package handler, so one Checkpoint 8a guard (6) allowlist amendment covers both.
- **D (record T).** tsc exit 0 is `Ok(passing)`, 2 is `Ok(failing)` (a broken tsconfig reads as a failing build,
  stated openly), 1 and any other non-zero status are `Err(NonZero)`; npx or direct launcher only, by citation of
  DECISIONS-2026-09-09 #34; a no-tsconfig fixture discriminates 1 from 2.

**What depends on it.** Records T and V's text; the `libtest_lines`, `nextest_lines`, report, scan and Node build
tests and their mutant rows; Checkpoint 8c's named selection; s8g's `sca-scan` (C). Reversal of any one is a
bounded edit to a proposed record, one code branch and its tests.

**Owed:** the operator's confirmation or reversal of A, B, C and D (AFK review packet, confirm list).

---

## D21-63 · An advisory whose CVSS vector the scorer cannot score refuses the scan by name (`cvss-unscorable`); the `cvss` crate's v4 scorer is the named durable remedy

**TAKEN PROVISIONALLY under the AFK rule of 2026-09-26, pending the operator's confirmation.** Same rule and method as
D21-62: three lensed judges and a synthesiser; the full panel text is `spikes/afk-panels/D21-63-s8c.md`. Carried by
record V (ADR-0170, `proposed`), so no accepted Decision moves.

**Why.** Record V decision 5 bands *"the CVSS base score"* on the v3 qualitative bands and counts an advisory with no
CVSS as `unrated`; it says nothing about a vector the scorer cannot score. The local advisory database holds 335
`CVSS:3.1`, 33 `CVSS:3.0` and **63 `CVSS:4.0`** vectors [measured 2026-09-26 over `~/.cargo/advisory-db/crates`]. P5
scored v3.x in-tree, with no new crate. Only advisories matching the lockfile are scored; at `5561afa` this
repository's lockfile matches none (`vulnerabilities = []`), so nothing is blocked today.

**Taken provisionally.**
- **Option (1), as built at `5561afa`:** a MATCHED advisory whose vector is `CVSS:4.0`, malformed, or any other
  version refuses the whole scan by name, `cvss-unscorable`, naming the vector. It is never counted `unrated` (it has a
  CVSS) and never banded by a guess.
- The refusal's text names its two remedies, so an AFK halt is one operator decision away: score it with the `cvss`
  crate's v4 scorer, or rule it `unrated` so `sca-scan` routes to `scan.accept-findings`.
- Red-first tests: a matched `CVSS:4.0` fixture advisory refuses naming its vector; a malformed vector refuses the same
  way; a mutant that turns the refusal into `Ok(None)` (count as unrated) must be killed.
- Record V decision 5 gains the sentence, and its forbids block gains *"Counting an advisory whose vector could not be
  scored as `unrated`, or banding it by a guess."*
- **Downstream precondition (`s8g`):** before §8g.6 runs `sca-scan` over the real database, `s8g`'s open lists every
  matched advisory's vector version (`cargo audit --json --no-fetch --file Cargo.lock`); a non-3.x vector is a NAMED,
  expected halt, and (3') is ruled then.

**The durable remedy the panel prefers, (3'), not built:** `cvss = { version = "=2.2.0", default-features = false,
features = ["std", "v3", "v4"] }` (the version rustsec 0.33 parses the database with; MIT OR Apache-2.0), v4.0 vectors
scored and banded on V's same thresholds (FIRST CVSS v4.0 §6 uses the v3 qualitative scale), `cvss-unscorable` kept
for anything the crate rejects, a differential test pinning the in-tree v3 scorer against `cvss::v3`, and V's
`cargo deny check licenses`/`bans` readings. Reversal of (1) into (3') is one dependency line and one match arm.

**Owed:** the operator's confirmation of (1), or the choice of (3') or (2) (AFK review packet, confirm list).
