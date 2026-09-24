# Decisions taken at Step 8's close, Step 7f's open and close, and Step 7g's open and build — 2026-09-21 … 2026-09-24 (D21-1 … D21-39), and what they license

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
