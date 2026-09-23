---
id: "0153"
title: "The lane's stage order is a **declared array** — a new `[policy] stage_order` whose element order IS the lane order — and the successor is `Pack::successor(stage)`, the next entry that the pack does not mark `forbidden`, computed in the kernel and **supplied** to it by `rk-cli`. `s7h` §7h.1's *\"the pack's own stage order — `[policy] stages`\"* asks for an order that does not exist: a TOML table is an unordered collection by specification, this tree's own parser materialises every inline table as `BTreeMap` at `toml_subset.rs:454-487` so the twelve declared stages come back ALPHABETISED, and `parse_spans` cannot recover the textual order either because the whole inline table sits on ONE line and spans are at line grain. Relying on a table's textual key order would be relying on something neither the format nor the parser guarantees; an array is the only order-preserving shape in `toml_subset::Value`, and the two keys are cross-checked at load so they cannot drift"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/toml_subset.rs (`pub type Table = BTreeMap<String, Value>` at 37; `Value::Array(Vec<Self>)` at 46; `inline_table` at 454-487; `pub type Spans = BTreeMap<String, usize>` at 139 — *"a canonical dotted path to the 1-based line"*); crates/rk-kernel/src/select/pack.rs (`stage_rigor: BTreeMap<String, Rigor>` at 159, the key-by-key parse at 250-263, `rigor()` at 394-399); crates/rk-kernel/src/rigor.rs (`Rigor::Forbidden` at 87, `from_token` at 123-125); crates/rk-kernel/src/workflow/fsm.rs (the module header at 167-173 — *"there is no stage ordering in this system"*; `next_action`'s arm 4 at 1700-1707, which consults no pack); .redkiln/packs/bug-standard.toml and .redkiln/packs/story.toml (line 13, the `policy.stages` inline tables); references/adr/0020 (the ordinal deontic enum)
supersedes: []
superseded_by: []
related: ["0020", "0050", "0152", "0154"]
evidence: []
consumers: []
---

## Context and problem

`s7h` §7h.1 says where the successor comes from:

> The successor is the pack's own stage order — `[policy] stages` — filtered to the stages the pack does
> not mark `forbidden`, which `bug-standard` and `story` both declare.

**`[policy] stages` has no order to read.** Three independent reasons, each measured at `fc45eeb`:

1. **TOML does not define one.** A table is an unordered collection of key/value pairs. The twelve
   stages are declared as a single inline table (`.redkiln/packs/*.toml:13`), so their textual sequence
   is a property of the file's layout, not of the parsed document.
2. **This tree's parser discards it at the parse layer**, before `pack.rs` sees anything.
   `crates/rk-kernel/src/toml_subset.rs:37` is `pub type Table = BTreeMap<String, Value>;` and
   `inline_table()` (`:454-487`) builds exactly that. So `policy.stages` comes back **alphabetised** —
   analysis, closeout, design, discovery, implement, planning, release, review, specification, triage,
   verify — and `select::pack` then copies it key-by-key into another `BTreeMap`
   (`pack.rs:159`, `:250-263`). The order is gone twice over.
3. **Spans cannot recover it.** `toml_subset::parse_spans` does record a line for every key
   (`Spans = BTreeMap<String, usize>`, *"a canonical dotted path to the 1-based line"*), but the module's
   own subset rule is that an inline table nests *"freely **within one line**"* — so all twelve stages
   share one span. Line grain cannot order twelve keys on one line.

And there is nowhere else to read it from. `grep -rn "closeout" crates/*/src` returns exactly **one**
hit, and it is prose denying the concept exists — `crates/rk-kernel/src/workflow/fsm.rs:169`:

> Stage ORDER (triage → operate → analysis → implement → release → closeout) is **not modelled** by the
> selection function; only per-stage requirement is.

That comment is also not a usable sequence: it is a parenthetical illustration whose own ordering
(`operate` second, `analysis` third) matches neither pack.

Finally, `next_action`'s advance arm (`fsm.rs:1700-1707`) reads **only** `ctx.successor` and
`state.stage`. It consults no pack, no catalog and no `Rigor`. So whatever skips a `forbidden` stage has
to do it before the FSM is asked — the FSM cannot.

## Decision drivers

- An order that the format does not guarantee and the parser does not keep is not a source of truth; a
  lane whose sequence depended on how an author happened to lay out one line would be a latent
  reordering bug with no failing test.
- `Value::Array(Vec<Self>)` (`toml_subset.rs:46`) is the **only** order-preserving shape in the subset.
  If order must be declared, an array is the shape available.
- `0152` puts the successor's supply in `rk-cli` and keeps the kernel from inventing one; this record
  must say what `rk-cli` reads, not re-open who supplies it.
- Two keys describing one set will drift unless something makes drift a load error — the failure mode
  would be a stage in the order with no rigor (or the reverse), i.e. a lane that skips or refuses a
  stage for a reason nobody wrote down.
- `forbidden` is not a field: it is `Rigor::Forbidden` (`rigor.rs:87`), the bottom of `0020`'s ordinal
  enum, and `read_rigor` already rejects `discouraged` (AC-050). The filter is a comparison, not a new
  vocabulary.
- Nothing in the tree rejects an unknown `[policy]` key — `pack.rs` reads `policy.stages` and
  `policy.verification_required` and ignores the rest, and `doctor`'s `RK-D-DEAD-DEF`
  (`rk-cli/src/doctor/rules.rs:726-734`) uses `stages.keys()` alone — so adding a key breaks no reader.

## Considered options

**1. Read the textual order of the `policy.stages` inline table.** Either teach `toml_subset` to keep
inline-table key order, or re-scan the pack's raw `src`. The first changes a module whose header insists
on *"ONE PARSER … rather than two loops that could come to disagree about what a line is"*; the second
**is** that second loop. Both then rest on a layout property TOML does not define, so both buy a
guarantee the format refuses to give.

**2. A canonical twelve-stage sequence in the kernel.** Shortest diff, and it contradicts `fsm.rs:167-173`
in terms: the kernel would then model stage ordering after stating three times that it does not. It also
makes every pack's lane identical, which `bug-standard` and `story` already are not (`operate` is
`forbidden` in one and `optional` in the other).

**3. A declared ordered array, cross-checked against `policy.stages` (chosen).** The order becomes a
thing an author states rather than a thing a reader infers. It costs one key per pack and one field on
`Pack`, and it makes the alphabetisation harmless because nothing depends on the table's order any more.

## Decision

**1.** Packs declare `[policy] stage_order`, an array of stage names whose **element order is the lane
order**. `toml_subset` already preserves it: arrays are `Vec`.

**2.** `Pack` gains an ordered carrier parsed from that array. `stage_rigor`'s `BTreeMap` stays exactly
as it is and remains the only source of a stage's rigor — this record adds an order, it does not move a
rigor.

**3.** Loading **refuses** when `stage_order`'s element set and `policy.stages`' key set differ, naming
the symmetric difference in both directions, and refuses a duplicate element. Two keys about one set
cannot silently disagree.

**4.** `Pack::successor(stage) -> Option<&str>` returns the next element after `stage` whose
`rigor(..) != Some(Rigor::Forbidden)`, skipping forbidden entries rather than stopping at them, and
`None` at the end of the order or for a stage the order does not contain. `None` means *"this lane has
no next stage"*, which is `NextAction::Done`'s own reading of an absent successor
(`fsm.rs:566-568`).

**5.** `rk-cli` calls it and chains `RuntimeContext::advances_to`. The kernel still never invents a
successor: `Pack::successor` is a pure function of a loaded pack, and the FSM is handed its answer.

**6.** `s7h` §7h.1's *"the pack's own stage order — `[policy] stages`"* is corrected by errata. The
sentence names a key that cannot carry the meaning.

## Consequences

The lane order becomes reviewable: it is one array per pack, in the file, rather than an emergent
property of an inline table's layout surviving two `BTreeMap`s (which it does not). `bug-standard` and
`story` can legitimately differ in order, not only in rigor.

Both shipped packs must gain the key, and that is a `7e` artifact edit — the packs are authored, not
generated, and `.redkiln/packs/` is in scope for a stage that changes what a pack must declare. A pack
without `stage_order` yields `successor() == None` for every stage, i.e. it cannot advance; that is a
refusal by absence rather than a guess, and it is the same posture `Pack::rigor` already takes
(`pack.rs:394-396`: *"`None` for a stage the pack does not mention — **an absence, which is not a
`forbidden` and not an `optional`**"*).

**What this forbids**

- Deriving the lane order from `policy.stages`, from `stage_rigor`'s iteration, or from any `BTreeMap`.
- Teaching `toml_subset` to preserve inline-table key order for this purpose, or re-scanning a pack's
  raw source for it.
- A canonical stage sequence compiled into the kernel, or reading `fsm.rs:169`'s parenthetical as one.
- `Pack::successor` consulting anything but the declared order and `Rigor::Forbidden` — it is not where
  a stage's activities, gates or artifacts are considered.
- Loading a pack whose `stage_order` and `policy.stages` disagree, in either direction.
- Treating `successor() == None` as an error: it is the end of a lane.

## Evidence and confidence

**The three reasons are read off single lines.** `toml_subset.rs:37` is the `BTreeMap` typedef verbatim;
`:46` is `Array(Vec<Self>)`; `inline_table()` at `:454` returns the table variant built from that
typedef; `:139`'s `Spans` doc says *"to the 1-based line"* and the module header (`:25`) says an inline
table nests *"within one line"*. `pack.rs:159` and `:250-263` are the second `BTreeMap` and the
key-by-key insert.

**Measured 2026-09-22 at `fc45eeb`.** Both packs declare the same twelve stages in the same textual
order on line 13. `grep -rn "forbidden" .redkiln/packs/*.toml` returns exactly two lines, those two:
`story.toml` marks **`discovery`**; `bug-standard.toml` marks **`discovery`** and **`operate`**. So under
`bug-standard` the successor of `triage` is `analysis`, and the successor of `release` is `closeout`.
`grep -n 'stage_order\|order' .redkiln/packs/*.toml` returns nothing — the key does not exist yet, so no
pack is silently relying on one.

**The no-breakage claim was checked rather than assumed.** `pack.rs` reads `policy` twice (`:212`,
`:251`) and looks up named keys; there is no unknown-key rejection in `select/pack.rs` or
`pack_validation.rs`. The second, independent reader of the key — `doctor`'s `RK-D-DEAD-DEF` at
`rk-cli/src/doctor/rules.rs:726-734`, which uses the real `toml` crate rather than `toml_subset` — takes
`stages.keys()` as a reachability set and never enumerates `[policy]`'s own keys.

**Confidence, and what is NOT asserted.** This record does not assert that the twelve-name sequence both
packs happen to declare is the right lane for either kind; it asserts only that the sequence must be
*declared* to be readable, and it transcribes the existing textual order rather than proposing a new one.
It does not decide what happens when a lane reaches a stage with no activities — that is selection's
existing behaviour and `0156`'s subject. And it asserts nothing about `discouraged`, which `read_rigor`
rejects under AC-050 and which therefore cannot appear in a loaded pack at all.
