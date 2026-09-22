# Decisions taken at Step 8's close, Step 7f's open and close, and Step 7g's open and build — 2026-09-21 and 2026-09-22 (D21-1 … D21-17), and what they license

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
