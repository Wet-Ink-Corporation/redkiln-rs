---
id: "0107"
title: The CLI has one idempotent run entry point, no resume flag or verb, an idempotent init, and a committed argument-surface snapshot with a name denylist
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-082 (§4.3), PIA-030 (§4.1); references/2026-09-08-architecture-design.md §8.1 :1267, :1273, §8.2 :1296, §10.x :1609; AC-400–AC-404, AC-437; references/2026-09-05-architecture-proposal.md §4.5 (L2.3)
supersedes: []
superseded_by: []
related: ["0002", "0009", "0011", "0013", "0109"]
evidence: ["PIA-082", "PIA-030", "F19", "F22"]
consumers: []
---

## Context and problem

Design §8's command surface is marked DECIDED and cites ADR-0011 for `run <item>` as "the one idempotent entry point;
no `--resume`" and for `init`'s idempotence; ADR-0011's body decides which two process packs ship and nothing about
the CLI [measured, PIA-082]. AC-400–AC-404 and AC-437 carry `decisions: ADR-0011` for rules that record does not
contain, so the forbids machinery protects none of them: an implementer adding a `--resume` flag violates no accepted
ADR. The runbook's own Phase-1 exit check (3) invoked `redkiln run --resume <run-id>` — the flag AC-013, AC-186,
AC-264 and AC-400 deny [measured, PIA-030].

The rule's grounds are measured: F19's 174/174 lossless suspensions rest on every entry reconstructing state from git
(ADR-0109), F22 forbids depending on a process surviving, and a "pick up where you left off" verb is the surface
through which an in-process resume would re-enter. This record gives the rule its home. Nothing is superseded.

## Decision drivers

- A rule with no ADR is a rule the next refactor removes.
- Idempotent re-entry is what makes the fresh-process rule (ADR-0087 clause 2) livable for an operator.

## Decision

1. **`run <item>` is the one entry point that causes action, and it is idempotent.** Invoking it on an item whose run
   is suspended re-enters at the reconstructed state; invoking it on an item whose run is complete is a no-op that
   says so. There is no `--resume` flag, no `resume` subcommand, and no verb or flag whose meaning is "pick up where
   you left off". The idempotency key is `(item, activity_id@version, attempt)` (ADR-0097).
2. **`init` is idempotent**: a second `init` on an initialised repository changes nothing and exits 0.
3. **The argument surface is a committed snapshot** (AC-400: `trycmd`/`assert_cmd` over `--help` of every verb),
   and a **name denylist** (`resume`, `continue`, `stay`, `--stay`, `--resume`, `--continue`, `--from`, `--pick-up`)
   is a lint over the parser definition; the residual — an unlisted name — is stated, not closed.
4. **The runbook's Phase-1 exit check (3)** is `taskkill /T /F` the running process mid-graph, then re-invoke
   `redkiln run --lane fix --item <item>`; the run id is read back from the event log (AC-013) and digest equality is
   asserted via `redkiln explain <item>`. That `--resume` is refused is itself AC-400's behavioural check.

## Considered options

### Option — keep citing ADR-0011
Rejected: ADR-0011 decides packs, and a citation to a record that does not contain the rule is the defect.

### Option — a `resume` verb that is a thin alias of `run`
Rejected: an alias invites an implementation that differs; one verb, one path.

## Consequences

**Positive**
- AC-400–AC-404 and AC-437 cite a record that contains what they test.

**Negative**
- An operator who wants "resume" types `run` and reads the log; the `explain` verb is the affordance.

**What this forbids**
- Forbids any CLI flag or verb whose meaning is resumption.
- Forbids a `run` invocation that is not idempotent on a suspended or complete item.
- Forbids an `init` that mutates an initialised repository.

## Evidence and confidence

- [measured, F19] 174/174 lossless suspensions, each resumed by a fresh entry reconstructing from git.
- [measured, F22] ~12 background-job kills against one survivor.
- [documented, proposal §4.5 (L2.3)] the original source row.
- [measured, PIA-082, PIA-030] the orphaned citations and the runbook's own `--resume` invocation.
- **What this evidence does not license.** It does not license claiming the denylist is complete.
