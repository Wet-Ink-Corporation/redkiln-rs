---
id: "0202"
title: "**An `artifact:` input reads a human-attached root when the item has no `emit` binding of the kind** (`s8j`'s root-read record; D21-110 ruling 2's conditional record; the operator's ruling A on the S8J-C4 lane-proof defect D1, 2026-10-18): with no committed `emit` binding of `Kind` for the item, the item's latest attach row of `Kind` (an `artifact.write` carrying `attached_path`, read through `rk-store`'s typed reader `vouched::attached`) resolves the input to that row's reference `{kind, attached_path, digest}`, with no read-seam call; an `emit` binding, when there is one, still wins and still goes through the seam once; another item's row is never read. Supersedes ADR-0173 in part: decision 1's `artifact:` row and decision 8, for the root arm only"
status: proposed
date: 2026-10-18
legacy_id: none
source: the S8J-C4 lane proofs (`crates/rk-cli/tests/it/lane_proof.rs` on `claude/s8j-afk-batch-2-tg23uw-c4`, PR #103), whose fix lane's first activity `minimal-repro` refused its mandatory `report = "artifact:BugReport"` input after the root was attached; the operator's ruling A on that defect (D1), 2026-10-18; spikes/DECISIONS-2026-09-21.md D21-110 rulings 1-3; references/adr/0173 decisions 1 and 8, 0189 decisions 8 and 9 (its dated correction), 0200 decisions 2, 4 and 5; crates/rk-kernel/src/runtime/reference.rs (`read_artifact_input`, `emit_binding_of`), crates/rk-store/src/vouched.rs (`attached`, `ATTACHED_PATH`), crates/rk-cli/src/verbs/answer.rs (`store_attachments`)
supersedes: ["0173"]
superseded_by: []
related: ["0040", "0129", "0189", "0200"]
evidence: []
consumers: []
---

## Context and problem

**A lane's first activity cannot read the root a human attached.** ADR-0200 supplies each lane's human root through
a pack question whose option attaches the kind, and `answer` stores the attachment as one `artifact.write` row
carrying `attached_path` (ADR-0189 decision 8; `store_attachments`). The fix lane's `minimal-repro` then declares
`report = "artifact:BugReport"`, a mandatory input, and nine catalog activities, the story lane's
`bdd-scenarios` among them, declare `artifact:IntakeRecord` the same way. `read_artifact_input` resolves an `artifact:` input only through `emit_binding_of`, the latest
completion row binding the kind, and a root has none, so the input is refused: *"no committed `emit` binding of
`BugReport` for the item … is in the journal"*. The S8J-C4 lane proof measured exactly that refusal, and a probe that
served the attach row past it carried the fix lane on.

This was foreseen and deferred. ADR-0173 decision 1's `artifact:` row reads *"the committed `emit` binding of the
activation that produced `Kind` for this item, read from the journal, through **one** read-seam call per resolved
input"*, and decision 8 reads *"Every resolved `artifact:` input makes exactly one call to
`runtime::reference::read_artifact_input`, which serves the journalled binding and calls an `ArtifactReadSeam`"*.
D21-110 ruling 1 kept the interim behaviour (a mandatory root input refuses by name, an optional one is `null`), and
ruling 2 named the owner: *"`s8j` builds the resolver's root arm … The value is typed by the kind's declared source
or schema, never by whether the bytes parse as JSON. The widening of ADR-0173's `artifact:` row is carried by a record
superseding ADR-0173 in part, written `proposed` there."* ADR-0200 decision 5 left it to that conditional record.
This is that record.

## Decision drivers

- **An `emit` binding stays the first and strongest source.** A kind an activity produced for the item is read as
  before, through the seam, so D21-113's served-value rule and Halt 4 do not move.
- **One reader of attach rows.** `rk-store`'s `vouched::attached` already names the item's roots for the vouched
  pass (ADR-0189 decision 10). A second, ad-hoc reader of the same rows could come to disagree with it.
- **Deterministic.** Two attaches of one kind for one item must resolve the same way on every read and on replay.
- **Typed by the declared source, not by the bytes** (D21-110 ruling 2). A root's value is its reference, whatever
  its bytes are.
- **The store's seam cannot serve a root.** `ArtifactReader::serve` reads the kind's `<Kind>.json` sidecar on the run
  ref and checks it against the binding's digest; a root is stored as `<Kind>.root<ext>` and has no sidecar, so a seam
  call for a root refuses (D21-110's blocker, measured at `s8f`).
- **Nothing outside the item.** Another item's attachment is never this item's root.

## Considered options

### Option — The attach row resolves the input when no emit binding exists (taken)

With no committed `emit` binding of `Kind` for the item, the item's latest attach row of `Kind` resolves the input to
its reference. Taken: the operator's ruling A on D1 (2026-10-18).

### Option — Serve the root's bytes through a new reader arm in `rk-store`

Add a root arm to `ArtifactReader::serve` that reads `<Kind>.root<ext>` and appends an `artifact.read`, and route the
resolver's root arm through the seam. Not taken now: it changes `rk-store`'s reader and its replay record, and no lane
reads a root's bytes through the resolver (the agent step reads the file the reference names). It stays open as the
reader's root arm D21-110 ruling 2 also names.

### Option — Keep the refusal and make the lanes' first inputs optional

Edit the catalog rows to `artifact:BugReport?` and `artifact:IntakeRecord?`. Rejected: an analysis activity would
then dispatch with no report at all, which is the failure the mandatory input exists to stop.

## Decision

**1. The root arm.** `runtime::reference::read_artifact_input` first looks for the committed `emit` binding of `Kind`
for the item, exactly as ADR-0173 decisions 1 and 8 say, and serves it through one read-seam call. **Only when there
is none**, the item's human-attached root of `Kind` resolves the input:

- *The root* is the item's latest `artifact.write` of `Kind` that carries `attached_path` (`rk_store::vouched::ATTACHED_PATH`),
  in journal order, as `rk_store::vouched::attached(events, item)` names it. Only `answer` writes such a row (ADR-0189
  decision 8; R-48's dated note on design §5's `artifact.write` row), so a row of this shape is human-sourced by
  construction; a writer row (no `attached_path`) is never a root.
- *The value* is the row's reference, `{ "kind": <Kind>, "attached_path": <path>, "digest": <sha256 hex> }`, the attach
  row's own members. A field reference (`artifact:Kind.attached_path`) reads its field of that value; a field the
  value does not have is absent, so a mandatory one refuses by name and a `?` one is `null`.
- *Several attaches of one kind* resolve to the **latest** in journal order: a re-attach replaces the root, as it moves
  the vouched digest (ADR-0189 decision 10).
- *Another item's row* is never read; the reader filters on the row's `item`.
- *No root and no binding:* the interim behaviour stands, by name: a mandatory input refuses naming the kind and the
  item, and a `?` input is `null`.

**2. No seam call for a root.** The root arm calls no `ArtifactReadSeam` and appends no row. ADR-0173 decision 8's
*"Every resolved `artifact:` input makes exactly one call"* now holds for an input resolved from an `emit` binding,
and a root-resolved input makes none. The seam is still the only emitter of `artifact.read`, and the resolver still
appends nothing.

**3. What is superseded, in part and quoted.** ADR-0173 decision 1's `artifact:Kind[.field]` row, *"the committed
`emit` binding of the activation that produced `Kind` for this item, read from the journal, through **one** read-seam
call per resolved input"*, and its absent cell, *"mandatory: refuses by name; `?`: `null`; neither calls the seam"*,
each gain decision 1's root arm; and decision 8's *"Every resolved `artifact:` input makes exactly one call"* is
narrowed by decision 2. Everything else in ADR-0173 stands, its forbids included. ADR-0173 stays `accepted`; its
`superseded_by` gains `0202` in this record's flip commit, not before, because an accepted record is not marked
superseded by a `proposed` one (ADR-0092).

**4. What this record does not decide.** The reader's root arm (serving a root's **bytes** through `rk-store`, with an
`artifact.read` row) stays owed to the first consumer that needs the bytes through an `artifact:` input (D21-110
ruling 2). Whether the selection's presence check classifies an attached root `Present` is ADR-0200 decision 5's, not
this record's.

## Consequences

**Positive**

- Both lanes' first activities resolve their mandatory root input once the root is attached, through the one route
  ADR-0200 fixes.
- The resolver and the vouched pass read roots through one function, so they name the same root.
- The value is deterministic and replays: it is read from the journal alone.

**Negative**

- **A root read leaves no `artifact.read` row.** `doctor`'s `unread` rule therefore cannot see that a root was read,
  exactly as before this record; the reader's root arm (decision 4) is what would change that.
- **The agent gets a reference, not the bytes.** A step that needs the root's text reads the file `attached_path`
  names; an `artifact:Kind.<field>` reference into the root's content (for example `tech-debt-assessment`'s
  `artifact:IntakeRecord.body`) is absent and refuses by name until the reader's root arm exists.
- **"Human-sourced" is read from the row's shape, not from `artifacts.toml`.** A kind registered `source = "agent"`
  that a human attaches through `answer` also resolves by this arm when no activity bound it; the kernel's
  `Env` carries no artifact registry, and adding one would move every `Env` site.

**What this forbids**

- Forbids resolving an `artifact:` input from an attach row while the item has a committed `emit` binding of the kind.
- Forbids reading another item's attach row, or a writer row with no `attached_path`, as a root.
- Forbids choosing among several attach rows of one kind by anything but journal order, latest first.
- Forbids a read-seam call, or any appended row, for a root-resolved input, and forbids a second reader of attach rows
  beside `rk_store::vouched::attached`.
- Forbids typing a root's value by whether its bytes parse (D21-110 ruling 2).

## Evidence and confidence

- [measured 2026-10-18, S8J-C4's lane proof on PR #103] the fix lane stopped at `minimal-repro`'s `report` input with
  the refusal quoted in Context, after `answer --attach BugReport=…` had written its attach row; a probe that served
  the attach row past `emit_binding_of` carried the lane on.
- [measured 2026-10-18, `s8j` build, package S8J-C4-FIX] `rk-kernel`'s `root_input::` tests:
  `a_human_attached_root_resolves_an_artifact_input_with_no_emit_binding` and
  `the_latest_attach_row_of_the_kind_is_the_root_served`, each red on the base with the refusal above and then green;
  `an_emit_binding_of_the_kind_wins_over_an_attached_root` and `another_items_attached_root_is_not_read`, the two
  controls, green on both.
- [documented] ADR-0173 decisions 1 and 8, D21-110 rulings 1-3 and ADR-0200 decision 5, each quoted above; ADR-0189
  decision 9's dated correction, which names this record's deferral.
- **Confidence.** High that the arm resolves the lanes' root inputs and leaves emit-bound reads unchanged: four tests
  pin it. Moderate in the value's shape: a reference is what D21-110's *"typed by the kind's declared source"* reads
  as for a source with no schema, and the first consumer of a root's bytes may want more.
