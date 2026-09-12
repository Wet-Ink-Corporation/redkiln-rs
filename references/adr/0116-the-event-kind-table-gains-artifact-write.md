---
id: "0116"
title: The event-kind table gains artifact.write — the producing half of artifact.read, so a run's outputs are a query rather than a replay
status: accepted
date: 2026-09-12
legacy_id: none
source: spikes/DECISIONS-2026-09-12.md E3; references/2026-09-08-acceptance-criteria.md AC-521 *Verified by* (:6767-6768), AC-472 (:6727-6735), AC-426 (:6034-6040); references/2026-09-08-architecture-design.md §5 the event-kind table (:699, :706-722); references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md clause 7; references/adr/0075-typed-artifacts-revision-4-refinements.md (:58, :84)
supersedes: ["0105"]
superseded_by: []
related: ["0006", "0018", "0075", "0097", "0099", "0109", "0115"]
evidence: ["PIA-002", "S-MIG", "E3"]
consumers: []
---

## Context and problem

AC-521 — the story-lane half of Phase 1's exit — is verified by *"a release check over the event log
(stage-completeness assertion per story-lane stage; an assertion that the story artifact's digest appears in the
run's `artifact.write` events)"*.

**`artifact.write` occurs exactly once in this corpus: in that sentence.** `artifact.read` occurs twenty-nine
times [measured, 2026-09-12]. Design §5's event-kind table — which ADR-0105 clause 7 owns and which names
`rk-types` as the owning crate — carries `artifact.read` (*tier 2 · kind, item id, `schema_version`, the tree hash
served · every activity input*) and **no write-side artifact kind at all**. The same section states the constraint
that makes this a record's problem rather than an implementer's: *"the `EventKind` enum is **closed**, a kind not
in this table is a schema change reviewed as one."*

So AC-521 is unimplementable as written, and Step 8 (1b) has been standing in with `git ls-files` on an
operator-supplied path — which proves a file with that path is tracked, not that the run produced it.

**The gap is architectural, not editorial.** With reads recorded and writes not, the store cannot answer *"what did
this run produce?"* without replaying it. That has a second consequence nobody filed: AC-472's dead-artifact rule
flags *"any artifact kind with zero reads across the last N runs"* — now `unread_window_runs = 50`
(DECISIONS-2026-09-12 E9) — and with only the read side recorded it cannot tell **produced-and-never-read** (the
dead artifact it is hunting: `plan.md`, dead in 110/110 and found by census) from **never produced at all** (a kind
no activity emits, which is a different defect with a different fix).

## Decision drivers

- A criterion naming an event kind no table defines cannot be built, and the enum is closed by design — so the kind
  is minted by record or not at all.
- `artifact.read` exists *because* every activity input is worth a durable mark (proposal §4.4 rev. 4, via AC-472).
  The same argument applies unchanged to every activity output; the asymmetry has no stated warrant.
- The dead-artifact instrument is weaker than it reads without the write side, and E9 has just set its window.
- Phase 1's exit should rest on an assertion over the run's own log, not on a path the operator names.

## Decision

**1. `artifact.write` is a tier-2 event kind**, emitted by **every activity output**, symmetric with
`artifact.read` on every activity input. Its row in design §5's table:

| Kind | Tier | Payload fields | Producing step |
| --- | --- | --- | --- |
| `artifact.write` | 2 | kind, item id, `schema_version`, the artifact's **content digest** | every activity output |

**2. Tier 2, and the deleted-database test is why.** Dropping the event store must not change `redkiln run`'s next
action (ADR-0109 decision 2): the artifact itself is on disk and in git, and its digest is recomputable from it, so
the event is rebuildable by re-execution and disposable. It is **not** tier 1 and carries no trailer.

**3. AC-521's release check resolves against it, as written** — the story artifact's digest appears in the run's
`artifact.write` events — and AC-473's twin does the same on the fix lane. Step 8 (1b) asserts that, not tree
membership.

**4. `doctor` may compare produced against consumed.** With both sides recorded, AC-472's rule separates two cases
that were one: a kind with `artifact.write` events and **zero** `artifact.read` events across the window is the
dead artifact; a kind with **neither** is not produced at all and is a distinct `doctor` row. This record does not
set a threshold for the second case — it names it so the first stops absorbing it.

**5. The table row and the `rk-types` variant land in the same change.** `EventKind` is closed and `rk-types` owns
it; a kind in the design table and not in the enum, or the reverse, is schema drift, and the golden schema snapshot
(s5 §5.3, under ADR-0095's rule) is the instrument that catches it.

**6. The digest is the artifact's content digest**, computed by the same function the typed-artifact digests use
(`rk-types`), so an `artifact.write` digest and a `Spec`/`ImplReport`/`Findings` digest of the same bytes are equal
by construction. No second digest algorithm enters the system.

## Considered options

### Option — reuse `activity_committed`'s result digest

`activity_committed` is already tier 2 and already carries *"(item, `activity_id@version`, attempt), result
digest"*. AC-521's assertion would be rewritten against it and the closed enum left alone. **Rejected:** it
conflates *the activity committed* with *this artifact was written*. One activity may produce several artifacts —
an implementation report and a findings record and a ledger row — and a single result digest cannot say which
artifact the story is. It would also leave AC-472's produced-vs-consumed comparison unavailable, which is half the
value here.

### Option — record Step 8 (1b)'s digest half NOT RUN

Mark it unverified under Halt 8, raise the missing kind as an errata item against the register, and leave the
instrument unbuilt. **Rejected:** Phase 1's exit would then rest on stage-completeness alone, and the honest
reading of that is that M5 is certified by something weaker than the criterion states. Halt 8's carve-out is for
things a *stage* cannot run yet, not for an instrument no record defines.

### Option — mint the kind in the runbook

**Rejected on mechanism:** `00-preamble.md` — *"Where a stage is tempted to decide something else, it stops"* — and
the enum is closed by a design section ADR-0105 clause 7 owns. A kind invented at Step 7d would be exactly the
"a rule with no ADR is a rule the next refactor removes" failure ADR-0107's drivers name.

## Consequences

**Positive:**
- AC-521 and AC-473 become implementable exactly as the register words them.
- *"What did this run produce?"* becomes a query over the log instead of a replay.
- AC-472's dead-artifact detection gains its denominator, which is what makes E9's window of 50 meaningful rather
  than merely conservative.

**Negative:**
- One more event per activity output. Tier 2, digest-only, and small beside the dispatch record already written per
  activity — but it is not free, and no measurement bounds it.
- A closed enum moves, so `rk-types`' golden snapshot moves with it and is re-accepted once, deliberately.

**What this forbids**
- Forbids an activity output with no `artifact.write` event.
- Forbids a second digest algorithm for artifact content.
- Forbids recording `artifact.write` at tier 1, or carrying it on a trailer.
- Forbids reading a kind's zero-read count as a dead artifact when it has no writes either — that is the
  not-produced row, not the dead-artifact row.

## Evidence and confidence

- [measured, 2026-09-12] `git grep -c 'artifact\.write'` over `references/`, `spikes/` and `runbook/` returns **1**
  — AC-521:6768 itself; `artifact\.read` returns **29**.
- [documented, references/2026-09-08-architecture-design.md :699] *"the `EventKind` enum is closed, a kind not in
  this table is a schema change reviewed as one"*; the table at :706-722 carries `artifact.read` and no write-side
  artifact kind.
- [documented, references/adr/0075 :58, :84] the read-side events, with no write counterpart stated anywhere.
- [measured, S-MIG decision 4] `plan.md` dead in **110/110** and found by census long after it shipped — the
  failure AC-472 exists for, and the one the read-only signal detects late.
- **Confidence:** the gap is measured and unambiguous; the *shape* of the fix (payload fields, tier) is reasoned
  from the existing `artifact.read` row and ADR-0109's tiering test, not measured. No spike bears on it.
