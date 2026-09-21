---
id: "0130"
title: The recorded `Config` is written to the run-record layout as a conditional `inputs/config.json` and read back through `Config::new`; `run_id` keeps it in its preimage, the golden-log schema stays `v2`, and a pre-repair record carrying a config is re-recorded rather than re-blessed
status: accepted
date: 2026-09-20
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-042 (THEN *"`redkiln replay --run <id>` re-derives the DAG from the logged inputs alone THEN the re-derived DAG is identical to the logged one, node for node and edge for edge AND a mismatch is reported as `HiddenInput { run_id, first_divergent_node }`, never as a warning"*), AC-007 and AC-008 (the residue comparisons the run-id guard stands in front of), **cited by id and named clause** (DECISIONS-2026-09-15 D15-2); references/adr/0083 :76 (*"Configuration reaches the kernel **as a value**: a `Config` struct constructed at the composition root and passed in"*) as executed by Step 7d Slice 2's **D18-14** — `spikes/DECISIONS-2026-09-18.md`, the first of §7d.1's eight kernel edits, which declared `crate::config::Config` and threaded it into `RunInputs::with_config`; crates/rk-kernel/src/config.rs at `919d073` (four fields; `to_json`; **no `Serialize`/`Deserialize`, deliberately** — *"a `Deserialize` derive would be a SECOND public construction path that bypasses `Self::new`, so the toolchain normalisation documented there would stop being a property of the type and become a property of one of its two constructors"*; *"Its one kernel consumer at this slice is `crate::replay::RunInputs::with_config`"*); crates/rk-kernel/src/replay.rs at `919d073` (`SCHEMA` = `rk-kernel/golden-log/v2` :231; `with_config` :835; `to_json`'s conditional `config` insertion :948-953 and its byte-stability comment; `run_id` :968-977; `write_run` :1599; `read_run` :1696; `replay`'s run-id re-derivation and `HiddenInput` refusal :2094-2105 with its own *"one comparison policies all of them"*); crates/rk-kernel/src/select/canonical.rs :52-56 (`inputs_digest` is over pack, catalog, context and artifacts and **not** the config); crates/rk-cli/src/verbs/run.rs at `919d073` (`base_inputs`'s `.with_config(resolved.config.clone())` — every record the shipped binary writes carries one); crates/rk-cli/tests/it/support.rs at `919d073` (`inputs_over_pack_min_without_config`, whose entire reason for existing is this defect); runbook/PROGRESS.md §2 **R-54**, raised at Step 7d Slice 3's close (`919d073`) and named there as the largest residual the slice raises; the measurement of 2026-09-20 quoted under **Evidence and confidence**
supersedes: []
superseded_by: []
related: ["0024", "0083", "0099", "0105"]
evidence: ["F19"]
consumers: []
---

## Context and problem

**`run_id` is the one comparison that policies the whole recorded input set, and one input escaped it.**
`RunInputs::run_id()` is `sha256_lp([SCHEMA, stage, canonical(self.to_json())])`, and `replay()` re-derives it
before it re-derives anything else. The kernel says why in its own words at the refusal site: *"`run_id` is a
content hash of the whole input set — `sha256_lp([SCHEMA, stage, canonical(inputs)])`, and `specs.json` is folded
into it like every other input — **so one comparison policies all of them**. It needs no clock and no port: the
recomputation is the same pure function that minted the name."*

**Step 7d Slice 2 made `Config` a recorded input, and made it one only in the preimage.** ADR-0083:76 fixes that
*"Configuration reaches the kernel **as a value**: a `Config` struct constructed at the composition root and passed
in"*, and C-01 **RB-M-05** found no crate declared the type, so the sentence had no referent. **D18-14** put the
type in `rk-kernel` and threaded it into `RunInputs::with_config`, and `to_json()` gained a `config` key —
*inserted* rather than written inline, with care, and the reason is recorded beside it:

> Writing `config` that way would add a key to every run's input object, and `run_id` is `sha256_lp([SCHEMA, stage,
> canonical(inputs)])`: a new key changes the canonical bytes, so EVERY `run_id` and `inputs_digest` recorded on
> this tree before Step 7d would move. They must not. So the key exists only when a config was threaded in, and a
> `None` leaves the object byte-identical to what it was before this edit.

That care worked: no pre-7d `run_id` moved. **What it did not do is give the layout anywhere to put the value.**
`write_run` writes `run.json` (`schema`, `run_id`, `stage`, `inputs_digest`, `event_count`, `producer`,
`terminal_state`, `next_action`) and, under `inputs/`, `pack/`, `catalog/`, `artifacts/`, `facets.toml`,
`context.json`, `artifact-state.json`, `runtime.json`, `specs.json` and `deviation.json` — **and no config**.
`read_run` reads exactly those back and never sets the member, so the rebuilt `RunInputs` carries
`config: None` whatever the recording carried.

**So the input set that comes back hashes differently from the one that went in, and `replay()` refuses it.**
Measured 2026-09-20 while authoring `ac432_explain_over_replayed_log_is_byte_identical`, pointing `replay()` at a
record built the way `run` builds one:

```text
Divergent(HiddenInput { run_id: "implement-61b5dd057fefc7fe", first_divergent_node: "inputs/: the recorded run_id
is implement-61b5dd057fefc7fe and the logged input set hashes to implement-ff18038f5d185916 — some input was
edited after the run was recorded, so the re-derivation below would have run against inputs the recording never
saw" })
```

**`run` threads a config into every record it writes** — `base_inputs` ends
`.with_config(resolved.config.clone())`, one `with_config` over the value step (4) constructed — so **no record the
shipped binary produces is replayable**, and AC-042's *"re-derives the DAG from the logged inputs alone"* holds for
none of them.

**AC-042's guard is not broken; it is firing correctly.** Its THEN names the mechanism: *"a mismatch is reported as
`HiddenInput { run_id, first_divergent_node }`, never as a warning."* The recorded config **is** a hidden input —
it entered the digest and not the layout — and the halt is the honest report of that. This record repairs the
asymmetry the halt is pointing at, and changes nothing about the halt.

**Two measurements bound the blast radius.**

- **`inputs_digest` does not move with the config.** It is `select::canonical::inputs_digest`, over
  `pack.to_json()`, `catalog_json(catalog)`, `item.context_json()` and `item.artifacts_json()` — the config is in
  none of them. So the only recorded value the config can shift is `run_id`, and no `inputs_digest` on the tree is
  implicated. (The same distinction GAP 19 drew between these two digests for `keywords`, one edit earlier.)
- **Nothing reads the recorded config.** `RunInputs.config` is written at `with_config` and hashed at `to_json`,
  and there is no `config()` accessor on `RunInputs` and no other reader in the crate. `config.rs`'s own header
  says it: *"Its one kernel consumer at this slice is `crate::replay::RunInputs::with_config`."* The value's whole
  present function is to be part of the run's identity — which is exactly the function the missing half denies it.

**Why nothing caught it for two slices.** The kernel's replay suite drives a committed golden log
(`crates/rk-kernel/tests/fixtures/logs/implement-bf1d8cec3b22d288/`) that **pre-dates the config edit and carries
no config**, so its round trip is `None → None` and is unaffected. And until Step 7d Slice 3 cycle 9 no `rk-cli`
test pointed `replay()` at a record `run` had written; AC-432's byte-identity test is the first, and it found this
on its first run. The test now drives a deliberately config-less input set through
`support::inputs_over_pack_min_without_config`, a helper whose entire reason for existing is this defect and whose
deletion is this record's confirmation.

## Decision drivers

- **A content hash over the input set is worth exactly what its coverage is.** An input inside the preimage and
  outside the layout does not weaken the guard — it makes the guard *always* fire, which is worse than a gap,
  because a halt that every record triggers is a halt nobody can act on and will eventually be routed around.
- **AC-042's *"from the logged inputs alone"* is a property of the layout, not of the digest.** An input the layout
  does not carry cannot be re-derived from it, whatever the digest says.
- **D18-14's byte-stability property must survive the repair.** A `None` config must continue to leave the recorded
  object byte-identical, or every pre-7d `run_id` moves and Checkpoints 7b and 7b-ii re-open for a file.
- **`Config` deliberately has no `Deserialize`, and that is a decision to honour rather than an obstacle to
  remove.** Its doc comment: *"a `Deserialize` derive would be a SECOND public construction path that bypasses
  `Self::new`, so the toolchain normalisation documented there would stop being a property of the type and become a
  property of one of its two constructors."* `Config::new` sorts `toolchains` *"so the recorded form is a function
  of the set and not of the order a file happened to list it in"*; a reconstruction that skipped it could mint a
  `run_id` that depends on a file's line order.
- **The committed golden log must keep reading.** It carries no config, it is correct, and it is what Checkpoint 7b
  re-runs over.
- **`rk-types`, `rk-ports`, `rk-store`, `rk-git` and the four adapters are not edited** (§7d.15). Measured: no
  adapter crate references `ReplayError`, `RunInputs` or `Config`.

## Decision

**1. `write_run` writes `inputs/config.json`, and only when the member is `Some`.** The file is the canonical
`Config::to_json()` value plus one `\n`, written exactly as `context.json` and `artifact-state.json` are. The
conditional shape is the layout's own, already used twice: `runtime.json` is written *"ONLY when
`RunInputs.runtime` is `Some`"* and `deviation.json` *"ONLY when `RunInputs.deviation` is `Some`"*. A run with no
config writes no file, and its directory is byte-identical to what it is today.

**2. `read_run` reads it when present and reconstructs through `Config::new`.** The four members are read by name
— `harness`, `store`, `toolchains`, `unread_window_runs` — and passed to the existing public constructor. Absent
file → the member stays `None`, which is what the golden log needs and what every pre-config record gets.

**3. `Config` gains no `Serialize` and no `Deserialize`.** Decision 2 is an explicit field read followed by
`Config::new(..)`, so the toolchain sort stays a property of the type rather than of one of two construction
paths, and `config.rs`'s stated reason for declining the derive is not withdrawn. This record therefore takes
nothing back from that doc comment; it satisfies it.

**4. The golden-log schema stays `rk-kernel/golden-log/v2`, and the reasoning is a case analysis rather than a
preference.** The file is conditional, so no existing directory becomes invalid, and every record that exists
reads back identically or better:

| record | before | after |
| --- | --- | --- |
| the committed golden log, and any pre-7d record (no config) | `None`, `run_id` re-derives | **unchanged** — no file written, no file read |
| a record written after this repair, carrying a config | — | **re-derives**; the config is a logged input |
| a record written *between* D18-14 and this repair, carrying a config | `HiddenInput`, unreplayable | **still `HiddenInput`** — see decision 5 |

**5. A pre-repair record carrying a config is re-recorded, not re-blessed.** Its config genuinely was not logged,
so its `run_id` genuinely is not derivable from its layout, and the halt is correct about it. The repair does not
back-fill a value the recording never wrote; `write_run`'s own `remove_dir_all`-then-write shape makes
re-recording the cheap operation. No such record is committed anywhere on this tree — `git ls-files` finds run
records only under `crates/rk-kernel/tests/fixtures/logs/` (no config) and
`crates/rk-cli/tests/fixtures/explain/` (no config, authored) — so the population is working-tree only.

**6. A malformed `config.json` is `ReplayError::Malformed` naming the path, never a default.** Absent is `None`;
present-and-unreadable is a refusal. A missing member, a `toolchains` that is not an array of strings, or an
`unread_window_runs` that is not a `u32` each refuse by name, on `runtime.json`'s precedent — whose own refusals
read *"absent or negative — never a silent 0"*.

**7. The confirmation is the deletion of a test helper.**
`ac432_explain_over_replayed_log_is_byte_identical` drops `support::inputs_over_pack_min_without_config` and
drives `inputs_over_pack_min` — the real, config-carrying input set — through `resume`, `write_run`, `replay` and
`read_run`. That test is what found the defect and it is what closes it; the helper's existence is the defect's
marker, and it goes in the same change. **The repair and that test's amendment land together**, so the tree never
carries one without the other.

**8. What this record does NOT decide**, named rather than left to be discovered:

- **Whether the config should be a recorded input at all.** D18-14 decided that it should, and this record does not
  revisit it. If a later record withdraws it, the repair here becomes dead code and is removed by that record.
- **Whether `run.json` should carry a config digest** as a cheap cross-check beside `inputs_digest`. It would be a
  second place for the same fact and a second thing to keep in step; not taken, not foreclosed.
- **Whether `Config` grows fields.** It is four today. A fifth changes `to_json` and therefore every
  config-carrying `run_id` — which is the ordinary consequence of changing a recorded input, and is that record's
  to state.
- **`producer` on a resumed run.** `write_run` hard-codes `"rk-kernel::replay::record"` even for a
  `resume`-produced run; that is **R-38**, and it is untouched here. *(As first recorded 2026-09-20 this read
  *"that is **R-38**, still open, and untouched here"*. **R-38 was settled the same day, at `4b5119b`** —
  record-only: `producer` names the recording MECHANISM and not the function that wrote the file, so the literal
  **stands unchanged** for a `resume`-produced run, and the member is documented as informational with no
  instrument, beside `event_count`. No ADR was owed and none was written. The clause is corrected in place, before
  acceptance, because ADR-0092's `**What this forbids**` forbids an erratum that touches a `## Decision` sentence;
  it was open when this record was written and the correction moves nothing this record decides.)*
- **Whether any other member of `to_json` is similarly unpersisted.** Audited once for this record and the answer
  is no — `stage`, `pack`, `catalog`, `artifacts`, `facets`, `context`, `artifact_state`, `deviation`, `runtime`
  and `specs` each have a file — but the audit is a measurement at `919d073`, not a standing guarantee. The guard
  for it is decision 9.

**9. A test asserts the symmetry, so the next input cannot repeat this.** A `-p rk-kernel` case builds a
`RunInputs` with **every** optional member populated — config included — `write_run`s it, `read_run`s it back and
asserts `rebuilt.run_id()? == original.run_id()?`. That is one assertion over the whole preimage rather than a
field list, so a future member added to `to_json` and not to the layout reddens it without anyone remembering to
extend a list. It is the instrument this repair is worth; the repair without it is a fix, not a guard.

## Considered options

### Option — persist the config in the layout and restore it on read *(taken)*
Restores the symmetry every other member of `to_json` already has, keeps `run_id`'s coverage exactly as designed,
and needs no schema move. Its cost is one conditional file and one explicit reconstruction — the second only
because `Config` declines a derive, which is itself a decision worth more than the four lines it saves.

### Option — remove the `config` key from `to_json`'s preimage
The smallest diff, and rejected on substance. It would make the config an **unrecorded** input: not in the digest,
not in the layout, read by the composition root and invisible to the record. AC-042's *"from the logged inputs
alone"* would then be satisfied only because the config is not among the logged inputs, which is the letter
against the intent. It also reverts D18-14 in substance while leaving its record standing, and the immutability
rule means that withdrawal would have to be written as a supersession of a decision that is not in fact wrong.

### Option — persist the config but keep it out of `run_id`'s preimage
Rejected for the asymmetry it creates in the other direction. Every member of `to_json` is both persisted and
hashed; a logged-but-unhashed input is one an editor can change after the fact without moving the name that is
supposed to detect exactly that — the hidden input `HiddenInput` exists to report, one level down and harder to
see. `run_id`'s own comment (*"one comparison policies all of them"*) is the property being traded away.

### Option — make `replay()` tolerant: skip the run-id comparison, or compare a config-less digest
Rejected as disarming the guard to hide the thing it caught. A config-less comparison would also be a *second*
canonicalisation of the input set, kept in step with `to_json` by hand, and the two would drift the first time a
member moved. AC-042 asks for the mismatch *"never as a warning"*; this option is the warning with extra steps.

### Option — bump the schema to `v3` and require `inputs/config.json`
Rejected because it refuses records that are correct. The committed golden log carries no config, legitimately;
requiring the file would make `read_run` refuse it and Checkpoint 7b red, and the re-bless would be a move made to
satisfy a version number rather than a fact. `specs.json`'s required-ness is the contrasting case and the contrast
is instructive: it is required because a log without it *"carries no dispatch bindings, so a replay of one would
re-derive a journal with zero dispatches and compare it against a recorded journal with zero dispatches — AC-008's
GIVEN satisfied over an empty N"*. A log without a config has no such vacuity; it has no config.

### Option — leave it, and record the limitation
Rejected. It leaves AC-042 unsatisfiable for every record the product writes, and leaves a halt that fires on
every real record — which, over time, is a halt someone disables.

## Consequences

**Positive**
- Records `run` writes become replayable, and AC-042 holds for them for the first time since D18-14.
- `ac432_explain_over_replayed_log_is_byte_identical` can drive a real config-carrying record, so the byte-identity
  property is asserted over the shape the product actually produces rather than over a config-less twin.
- `run_id`'s stated property — one comparison over the whole input set — becomes true again, and decision 9 makes
  it self-policing for the next member.
- D18-14's careful conditional insertion is vindicated rather than unwound: the `None` case stays byte-identical
  and no pre-7d `run_id` moves.

**Negative**
- One more conditional file in the layout, and a third shape a reader of `read_run` must hold (required, optional,
  optional-and-reconstructed-through-a-constructor).
- A config-carrying record written between D18-14 and this repair stays unreplayable and must be re-recorded.
  Named in decision 5 rather than smoothed over, and the population is working-tree only.
- `read_run` gains an explicit four-field read that a `Deserialize` derive would have written for it. That is the
  cost of `config.rs`'s single-construction-path decision, paid where it was always going to be paid.
- Checkpoints 7b and 7b-ii become provisional and are re-run in the commit that lands this, on
  `00-preamble.md`'s maintenance rule — `replay.rs` is a kernel file and both blocks drive it.

**What this forbids**
- Forbids a member of `RunInputs::to_json`'s output that the layout does not persist and `read_run` does not
  restore. This is the general rule the record exists to state; the config is its first violation and decision 9
  is its instrument.
- Forbids `Serialize` or `Deserialize` on `crate::config::Config`, and forbids any construction of a `Config`
  outside `Config::new` — including inside `read_run`.
- Forbids writing `inputs/config.json` when `RunInputs.config` is `None`, and forbids writing a `null` in its
  place: a `None` config must leave the recorded directory and the recorded object byte-identical to what they
  are before this record.
- Forbids reading an absent `inputs/config.json` as anything but `None`, and forbids reading a present-but-invalid
  one as `None`, as a default, or as a partially-filled `Config`.
- Forbids skipping, weakening or making conditional `replay()`'s run-id re-derivation, and forbids a second
  canonicalisation of the input set for the purpose of comparing it.
- Forbids moving `SCHEMA` for this change, and forbids re-blessing a pre-repair config-carrying record in place
  rather than re-recording it.
- Forbids landing the `write_run` half without the `read_run` half, or either without decision 7's test amendment
  and decision 9's symmetry test, in one commit.

## Evidence and confidence

- [measured 2026-09-20, `919d073`] `to_json` inserts `"config"` when `self.config.is_some()` (`replay.rs`:948-953);
  `run_id` hashes `canonical(self.to_json())` (:968-977); `write_run`'s manifest has eight keys and its `inputs/`
  set has ten entries, **none a config** (:1599); `read_run` sets `stage`, `pack`, `artifacts`, `catalog`,
  `facets`, `context`, `artifact_state`, `runtime`, `specs`, `deviation` and **never `config`** (:1696);
  `replay()` re-derives `run_id` and refuses on mismatch (:2094-2105). `grep -n "fn config" replay.rs` → **no
  accessor**; the only reads of `self.config` are the assignment at :836 and the insertion at :952.
- [measured 2026-09-20, `919d073`] the `HiddenInput` refusal quoted verbatim above, produced by pointing
  `replay()` at a record built as `run` builds one.
- [measured 2026-09-20, `919d073`] `select::canonical::inputs_digest` is over `pack.to_json()`,
  `catalog_json(catalog)`, `item.context_json()` and `item.artifacts_json()` — so `inputs_digest` does not move
  with the config and `run_id` is the only recorded value implicated.
- [measured 2026-09-20, `919d073`] `crates/rk-cli/src/verbs/run.rs`'s `base_inputs` ends
  `.with_config(resolved.config.clone())`; every record the binary writes therefore carries a config.
- [measured 2026-09-20, `919d073`] the committed golden log carries no `config` and no `inputs/config.json`, which
  is why the kernel's replay suite is green over it and why this defect survived two slices.
  `git ls-files` finds run records under `crates/rk-kernel/tests/fixtures/logs/` and
  `crates/rk-cli/tests/fixtures/explain/` only, and none carries a config.
- [documented] AC-042's THEN and its `HiddenInput` mechanism; ADR-0083:76's configuration sentence;
  `config.rs`'s no-derive rationale and `Config::new`'s toolchain-sort rationale, both quoted above;
  `to_json`'s byte-stability comment, quoted above; `runtime.json` and `deviation.json` as the two
  conditional-file precedents in the layout; `specs.json`'s required-ness and its stated reason, as the contrast.
- **Confidence: high on the defect, high on the mechanism, and the case analysis in decision 4 is the part to
  check.** The defect is measured and its cause is a two-line asymmetry, not an inference. The repair is the only
  option that leaves `run_id`'s coverage intact without moving the schema, and the four-field reconstruction
  through `Config::new` is byte-exact because `Config::to_json` emits precisely `Config::new`'s inputs and the
  sort is idempotent over an already-sorted list. What a reviewer should press on is decision 4's third row — that
  a record written between D18-14 and this repair stays unreplayable — and decision 5's claim that no such record
  is committed; both are stated as measurements at `919d073` and both are cheap to re-measure.
- **Status is `proposed`, on ADR-0128 and ADR-0129's precedent**: it is written against a measured defect and
  awaits the operator's dated confirmation line under `spikes/DECISIONS-2026-09-20.md`. R-54 is recorded in
  `runbook/PROGRESS.md` §2 as **RAISED, not settled**, and stays so until that line exists.
