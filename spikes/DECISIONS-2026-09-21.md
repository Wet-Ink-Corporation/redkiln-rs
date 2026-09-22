# Decisions taken at Step 8's close and Step 7f's open, 2026-09-21 (D21-1 … D21-6), and what they license

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

**Grounding — it is execution, not design.** design §4.4 fixes the five closed step kinds and says `tool` is *"a
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
