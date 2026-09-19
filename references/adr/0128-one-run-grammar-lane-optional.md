---
id: "0128"
title: The CLI has one `run` grammar — `redkiln run <item> [--lane fix|story]`, the item positional and the lane optional and derived from the item when derivable — so ADR-0107 decision 4's `--lane fix --item <item>` sentence is superseded in part and every other clause of 0107 stands; `fix` names the `bug-standard` pack and `story` the `story` pack
status: proposed
date: 2026-09-18
legacy_id: none
source: references/adr/0107 — decision 1 (*"`run <item>` is the one entry point that causes action, and it is idempotent"*) and decision 4 (*"re-invoke `redkiln run --lane fix --item <item>`"*) — two `## Decision` sentences of one accepted record that fix two grammars; references/adr/0092 (*an erratum may not touch a Decision sentence*; clause 2.2 — where the design renders a criterion differently the design is amended to match); references/2026-09-08-acceptance-criteria.md — AC-400 (Verified-by: *"re-invoke `redkiln run --lane fix --item <item>` and assert digest equality via `redkiln explain <item>`"*), AC-401 (WHEN: *"`redkiln run <item>` is invoked a second time"*), AC-186 (THEN: *"`redkiln run <item>` is the sole entry for both a first run and a resumed run"*), AC-264 (THEN: *"`redkiln run <item>` is sufficient and `--resume` is not an accepted flag on any command"*), AC-403 (WHEN: *"`redkiln run <item>` opens that gate"*), **cited by id and named clause** (DECISIONS-2026-09-15 D15-2); references/2026-09-08-architecture-design.md §8.1 — the verb table's `run <item>` row (*"the one idempotent entry point; no `--resume`"*) and the exit-check paragraph below it that re-invokes the two-flag form; runbook/phase-1/s8-exit-criterion.md — the three `redkiln run --lane … --item …` invocations of the Phase-1 exit check; spikes/DECISIONS-2026-09-12.md E7 (*"exactly two — `bug-standard` and `story`"* — the Phase-1 pack names, which the lane vocabulary `fix`/`story` matched to nothing); the C-01 adjudication of 2026-09-18 (item D-02, BLOCKING — *"two implementers build routers that accept or refuse `redkiln run foo`, and the AC-400 snapshot pins whichever lands first"*), recorded in runbook/PROGRESS.md §6; the operator's decision of 2026-09-18, taken before Step 7d's stage-open commit with the alternatives named (two flags required; positional only with no lane flag; positional with an optional lane — the third was taken), recorded as `spikes/DECISIONS-2026-09-18.md` D18-17
supersedes: ["0107"]
superseded_by: []
related: ["0009", "0011", "0092", "0097", "0109", "0129"]
evidence: ["F38", "PIA-030", "PIA-082"]
consumers: []
---

## Context and problem

**One accepted record fixes two `run` grammars in two of its own Decision sentences.** ADR-0107 decision 1 says
*"`run <item>` is the one entry point that causes action"* — the item is positional and there is no lane. ADR-0107
decision 4 says the Phase-1 exit check re-invokes *"`redkiln run --lane fix --item <item>`"* — two flags, no
positional. The register carries both: AC-401, AC-403, AC-186 and AC-264 write `redkiln run <item>`; AC-400's
Verified-by writes `redkiln run --lane fix --item <item>`. The design carries both in §8.1 — the verb table's row
is `run <item>`, the exit-check paragraph beneath it re-invokes the two-flag form — and `s8`'s exit-criterion
block invokes `--lane fix --item` / `--lane story --item` three times.

Nowhere is it stated whether `--lane` is **required**, **optional** or **derivable**, and the lane vocabulary
`fix` / `story` maps to no pack name: DECISIONS-2026-09-12 E7 fixed the two Phase-1 process packs as `bug-standard`
and `story`. Two competent implementers reading these records build two routers — one accepts `redkiln run foo`,
the other refuses it as an unknown argument — and AC-400's committed snapshot pins whichever lands first. The C-01
pass (2026-09-18) adjudicated this BLOCKING (item D-02).

**Why this is a record and not an erratum.** Both shapes sit in `## Decision` sentences of an accepted ADR.
ADR-0092 forbids an erratum that touches a Decision sentence and forbids the runbook deciding it (*"where it must
decide, it says so in its header table and names the record"*). The route is a supersession **in part**: one
grammar, named here, and the lane → pack mapping that no record made.

## Decision drivers

- A grammar that two implementers can read two ways is not a grammar; the AC-400 snapshot exists to pin one.
- Decision 1's positional `<item>` is the form four register criteria and the design's verb table use; decision 4's
  two-flag form appears once in the register (a Verified-by) and in the runbook's exit check. The weight of the
  records is on the positional item.
- The lane exists because two packs exist (E7). An item record that names its kind (a bug, a story) already
  determines its lane; making the operator repeat it is a second source of truth for one fact.
- ADR-0107's every other clause — one idempotent entry, no resume flag or verb, the denylist, the snapshot, the
  idempotent `init`, the `taskkill /T /F` exit check — is untouched. Only the grammar sentence moves.

## Considered options

### Option — two flags, both required: `redkiln run --lane fix|story --item <item>`
ADR-0107 decision 4's literal form. Rejected: it contradicts decision 1 and four register THENs (AC-401, AC-403,
AC-186, AC-264) which write a positional item; a bare `redkiln run <item>` would be refused as an unknown
argument by the very router AC-186 says is *"the sole entry for both a first run and a resumed run"*.

### Option — positional only, no lane flag: `redkiln run <item>`
ADR-0107 decision 1's literal form with the lane always derived. Rejected: an item whose record does not make the
lane derivable (a migrated item with no kind; a hand-authored item at 7e before the packs' item schema settles)
would have no way to run, and the exit check's story-lane arm would have no way to name the story pack for an item
whose kind is ambiguous.

### Option — positional item, optional lane, derived when derivable **(taken)**
`redkiln run <item> [--lane fix|story]`. The positional form the records weigh toward; the flag stays for the case
the records need it; a missing, non-derivable lane is a refusal, not a guess.

## Decision

**1. The `run` grammar is `redkiln run <item> [--lane fix|story]`.** The item is the first positional argument.
`--lane` is an optional flag whose value set is closed at `fix` and `story` (a `clap` value enum, so a third value is
a usage error, not a configuration error). There is no `--item` flag.

**2. The lane is optional and derived from the item when derivable.** When `--lane` is absent, the lane is read
from the item record's kind where that record makes it derivable; when `--lane` is given it must agree with a
derivable kind, and a disagreement is refused. When the lane is **neither given nor derivable**, `run` refuses as
a **configuration error** (AC-438's configuration-error row) naming the item and the two admissible values, and
writes nothing. It never defaults to a lane.

**3. The lane names the process pack.** `fix` → the `bug-standard` pack; `story` → the `story` pack
(DECISIONS-2026-09-12 E7's two Phase-1 packs). The pack must be enabled in `.redkiln/config.toml`'s `[packs] enabled`
(E6); a lane whose pack is not enabled is the same configuration error with the pack named.

**4. The Phase-1 exit check (3) re-invokes `redkiln run <item> [--lane fix]`** — the positional form, the flag
optional and kept in `s8`'s lines until the fixture item's kind makes the lane derivable (Step 7e's item schema); one
grammar, the flag being decision 2's. ADR-0107 decision 4's *"`taskkill /T /F` the
running process mid-graph, then re-invoke … ; the run id is read back from the event log (AC-013) and digest
equality is asserted via `redkiln explain <item>`"* stands in every clause except the re-invocation's spelling,
which is now `redkiln run <item> [--lane fix]` (the lane derived from the item the first invocation ran once the
item record carries its kind; passed explicitly until then). That `--resume` is
refused is still AC-400's behavioural check.

**5. What this record supersedes, exactly.** ADR-0107 decision 4's grammar sentence — the spelling
*"`redkiln run --lane fix --item <item>`"* — and nothing else of that record: decisions 1, 2, 3, the rest of
decision 4, and every bullet of its forbids stand; 0107 stays `accepted` carrying `superseded_by: ["0128"]`.

**6. The records amended toward it** (ADR-0092 clause 2.2 direction: register and design toward the ADR; the
runbook toward both), each as a dated in-place amendment (DECISIONS-2026-09-15 D15-2), none as an erratum:
AC-400's Verified-by (the re-invocation spelling); design §8.1's exit-check paragraph (the same grammar —
`redkiln run <item> --lane fix`, the flag optional) and its
verb-table `run <item>` row (gains `[--lane fix|story]`); `s8`'s three `--lane … --item …` invocations. AC-401,
AC-403, AC-186 and AC-264 already read `redkiln run <item>` and are unchanged.

**What this forbids**
- Forbids a second `run` grammar: no `--item` flag, no `run` subcommand or alias, no positional lane.
- Forbids a default lane: an item whose lane is neither given nor derivable is refused, never run under `fix`.
- Forbids a lane value outside `{fix, story}` in Phase 1, and a lane → pack mapping other than
  `fix → bug-standard`, `story → story`.
- Forbids reading the lane from anywhere but the flag or the item record — not from the environment, not from
  `config.toml`, not from the cwd.
- Forbids reopening any other clause of ADR-0107 under this record's cover.

## Consequences

**Positive**
- One router, one snapshot. AC-400's committed `--help` snapshot pins `run <ITEM> [--lane <LANE>]` and the denylist
  lint runs over one tree.
- The five register criteria that write `redkiln run <item>` are already true of the shipped grammar.
- The lane vocabulary finally names the packs it selects.

**Negative**
- A supersession-in-part of a record written seven days ago; the register, the design and `s8` each take a dated
  amendment in the stage-open commit that carries this record.
- The derivation rule depends on the item record carrying a kind, which is 7e's pack schema to fix; until then the
  flag is the operator's route and the derivation is exercised by Step 7d's fixtures only.
- Committed `proposed`: the operator's word on this shape is recorded in `DECISIONS-2026-09-18` D18-17; the flip to
  `accepted` is a follow-up commit once that line is dated (ADR-0126's precedent), `date` unchanged.

## Evidence and confidence

- [documented] ADR-0107 decisions 1 and 4, quoted above; AC-400, AC-401, AC-403, AC-186, AC-264 by clause; design
  §8.1's `run <item>` row and exit-check paragraph; `s8`'s three invocations; E7's two pack names.
- [measured, F38] 3/3 lossless re-invocations across a real `taskkill /T /F` — the exit check this record re-spells
  — under the positional form.
- [measured, PIA-030 / PIA-082] the runbook's own `--resume` line and the orphaned ADR-0011 citations that ADR-0107
  was written to home; the same audit shape recurs here as one record carrying two grammars.
- **Confidence:** the conflict is a reading of two quoted sentences and is not in doubt; the grammar is the
  operator's choice among three named options. Whether every Phase-1 item record makes the lane derivable is **not
  measured** and is 7e's; decision 2's refusal is what makes that safe to leave open.
- **What this evidence does not license.** It does not license claiming the denylist is complete (ADR-0107's own
  limit stands), nor that a derived lane is always the lane the operator meant — the flag exists for that.
