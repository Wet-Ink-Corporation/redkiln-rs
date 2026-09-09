# P1 arm L — the triage classification card (METHOD-P §1.2)

**Status:** committed before the first arm-L dispatch and **not edited during P1** (METHOD-P §1.2). The sha256 of
this file is recorded on every arm-L record as `card_sha256`; a record whose card digest differs from the committed
one is void.

**Scope:** four ordinal axes, not five. METHOD-P §1.2 lists `dynamism`, but `spikes/p9/facets.toml` does not register
it, so it can feed no `require` row and no `R(stage, ctx)` — the same gap `lattice.py` records as I8 and
`F35-DRAFT.md:60` already discloses. Departure logged as `DECISIONS-2026-09-09` #2.

**Schema:** `spikes/schemas/p1-classify.schema.json`, passed as `--json-schema`. The record is closed
(`additionalProperties: false`) and every field is required.

> **F20 guard — load-bearing.** The agent card's `tools` list is an allowlist, and with `StructuredOutput` outside it
> `--json-schema` is **silently inert**: the model answers in prose and every record is void for want of a digest
> (`spikes/runner/src/e3.rs:50-55`, FINDINGS F20). Any card used for these dispatches must carry `StructuredOutput`.
> Validate **one** draw before spending on the set; a record that comes back `void: schema` stops the lane.

---

## 1. THE TASK GIVEN TO THE MODEL

You are classifying one backlog item so a deterministic engine can select the process for it. You are given the
item's card and its body text, and nothing else — no repository, no history, no diff. Return the closed record
defined by the schema.

Classify **the item as written**. Do not infer what the work will turn out to involve, do not read the codebase in
your head, and do not reward yourself for guessing. Every non-`unknown` ordinal value must be supported by a verbatim
quote from the item in `evidence[]`. If you cannot quote the item for an axis, that axis is `unknown`.

## 2. WHAT `unknown` MEANS — read this before the anchors

`unknown` means **"this item does not contain the information needed to place it"**. It is not a middle value, not a
hedge, and not a way to avoid committing. `med` is the middle value.

`unknown` is not free. At triage it routes the item to a human; and if it reaches selection anyway it is treated as
**high** by position (F34 B14, `F35-DRAFT.md:49` — 23 activities are selected at `<axis> = unknown` that are not
selected at `low`). So an `unknown` you did not need costs a human's attention and, failing that, escalates rigor.
An `unknown` you did need is correct and cheap by comparison.

Use `unknown` when the item is silent or genuinely ambiguous on the axis. Use `low`/`med`/`high` when the item says
enough to place it, even if it says so briefly.

## 3. THE ANCHORS

Each level gives the test to apply. Apply them in order and take the first that fits.

### 3.1 `criticality` — what is at stake if this is done wrong, or not done

- **low** — Getting this wrong produces an annoyance that a contributor can route around, or a cosmetic defect nobody
  is blocked by. Nothing depends on it landing by a particular time. Typical shapes: a docs correction, a rename, a
  log line, a lint rule, a convenience flag. If the item were silently dropped for a quarter, the effect would be
  irritation rather than damage.
- **med** — Getting this wrong blocks or misleads someone doing real work, but the damage is bounded and recoverable
  once noticed. A workflow becomes slower, a report becomes wrong, a contributor is stuck until someone helps. The
  item may name a person or a team who is waiting. Recovery is a fix, not an incident.
- **high** — Getting this wrong loses or corrupts data, produces a false green (something reports success that did
  not succeed), breaks a security or permission boundary, takes a shipped surface down, or breaks a promise already
  made to someone outside the team. The item names an outage, a corruption, a security exposure, a regression that
  reached users, or a deadline with consequences attached. Note the false-green case specifically: an item about a
  gate, verdict, or check that can pass when it should fail is `high` even when the words are calm.
- **unknown** — The item describes work without saying what depends on it, and its kind does not settle the question.
  Do not infer criticality from how urgent the writing sounds.

### 3.2 `novelty` — how much of this has no worked precedent here

- **low** — The item is another instance of something the team has already done. It references an existing pattern,
  an earlier item, a sibling implementation, or says "same as", "like we did for", "follow the existing". The
  approach is not in question; only the doing is.
- **med** — The shape is familiar but this instance has a wrinkle the precedent does not cover: a new combination of
  existing pieces, an established pattern applied to a surface it has not been applied to, or a known technique with
  one open question the item names.
- **high** — The item asks a question rather than describing a change: it does not know what the answer looks like,
  proposes to find out, names alternatives to be chosen between, or admits the approach is undecided. Words like
  "investigate", "spike", "explore", "decide whether", "we don't yet know", "options" are the signal — but only when
  they govern the work itself, not when they appear in a passing aside.
- **unknown** — The item states a goal with no indication whether the path is known or open, and there is nothing to
  read either way.

### 3.3 `size` — how much changed surface the work is

Judge the work the item asks for, not the work it might uncover.

- **low** — One file, one function, one setting, or a handful of lines. A change a competent contributor completes
  and verifies in one sitting. Single-point edits, config values, copy changes, one added test.
- **med** — Several files that move together, or one file with a real design decision inside it. A new capability
  built from existing parts; a change that needs its own tests; something that takes more than a sitting but does
  not need to be broken into pieces.
- **high** — The item is a container for other work, or it names multiple surfaces that must change in step. It
  enumerates sub-items, lists acceptance criteria that are themselves projects, spans several subsystems, or
  requires a migration alongside the change. If the honest answer to "could this be one commit?" is clearly no, it
  is `high`.
- **unknown** — The item states an outcome with no indication of extent — a title and an intent and nothing about
  what would have to change.

### 3.4 `blast_radius` — how far a defect introduced here would reach

This is about the reach of a mistake, not the size of the change. A one-line change to a shared path is `high`.

- **low** — A defect stays inside the thing being changed. Nothing else reads it. Typical shapes: a leaf module with
  one caller, a test, a document, a surface behind a flag that is off.
- **med** — A defect reaches other parts of this system but stays inside it: a shared helper, a stage of a pipeline
  other stages depend on, a schema one subsystem reads. Contributors would notice; people outside would not.
- **high** — A defect escapes the system or cannot be contained: it reaches users or customers, touches persisted
  data or a migration, changes an interface other people have already built against, sits in a shared foundation
  (kernel, registry, CLI entry point, the gate itself), or changes something with no undo. Anything that writes to
  or reshapes stored state is `high`.
- **unknown** — The item does not say what the change touches, and its kind does not imply a surface.

## 4. THE BOOLEANS

These take `true` or `false`, never `unknown`. When the item is silent, the answer is `false` — absence of evidence
is the negative case here, unlike the ordinals.

- **`market_facing`** — `true` when someone outside the engineering team would notice this from the outside: a
  customer, an end user, a buyer, a reader of the public docs. `false` when only contributors or operators would.
- **`external_dep`** — `true` when finishing this needs something the team does not control: a third-party service or
  API, another team's merge, a vendor, a purchase, a credential someone else issues, an upstream release.
- **`has_repro`** — `true` when the item contains something a reader could follow to observe the problem: steps, a
  command, a failing test name, a stack trace, an input, an error string with a location. A described symptom with no
  route to it is `false`. For items that are not defects, `false`.

## 5. EVIDENCE

`evidence[]` carries one entry per non-`unknown` ordinal and per `true` boolean. `quote` must appear in the item
**character for character** — it is checked. Quote the shortest span that carries the decision. If you find yourself
paraphrasing, the value is not supported and the axis is `unknown` (or the boolean is `false`).

`notes` is free text and is **untrusted: it is never a predicate input** (process-catalog proposal §2.4). Nothing
there reaches the engine. Use it to say why an axis is `unknown`, or what you would have needed.

---

## 6. DISPATCH SHAPE (METHOD-P §1.2)

| | |
| --- | --- |
| items | `spikes/plans/p1-items.txt` — **20** at this pin, not 30 (`DECISIONS-2026-09-09` #5; the corpus holds 2 `bug.md`, 3 `initiative.md`, 0 spike) |
| models | `haiku` (4.5) and `sonnet` (5), both arms recorded separately |
| repeats | 5 per item per model — **10 draws per item** |
| temperature | default; not pinned, not passed |
| schema | `--json-schema spikes/schemas/p1-classify.schema.json` |
| total | 20 x 5 x 2 = **200 dispatches** |
| cap | **$40** cumulative `total_cost_usd`; at the cap the lane stops on a complete (item, model) block |
| records | `spikes/results/p1-arm-l.jsonl`, one per dispatch |

Every record carries: `card_sha256` (this file), `schema_sha256`, `corpus_sha` = `47faec9f...`, `model`, `repeat`,
`claude_version` (**2.1.263** — `DECISIONS-2026-09-09` #1), `total_cost_usd`, and `void_reason` (`schema` where the
digest failed validation — F20; a void is never a miss).

## 7. WHAT IS SCORED WITHOUT LABELS, AND WHAT WAITS FOR RYAN

Computed tonight (no panel needed): per-facet Krippendorff's alpha across the 10 draws per item; per item the
fraction of draws whose derived `R(stage, ctx)` under the v0 `bug-standard` / `story` packs equals the modal one,
with Wilson 95% intervals, per item, never pooled first (METHOD-S §0, F27); `unknown` rate per axis; haiku-vs-sonnet
modal disagreement per axis.

**Not computed tonight:** anything compared to the human agreement panel. §1.2 requires two human labels per item per
facet, written from the item alone before unblinding (METHOD-P §0 rule 1). The skeleton is
`spikes/results/p1-arm-l-labels-RYAN-TODO.md`.
