---
id: "0134"
title: "`conformance_every_port_has_a_call_site` asserts an EQUALITY against a dated, warranted expected-uncalled set, not an emptiness — `tracker` and `docs` have **no Phase-1 real adapter** by design (design:265-266, AC-499's phase clause), so the whole-set form is unsatisfiable inside Phase 1 and the lint it produced could only ever stay `#[ignore]`d; the narrowed form runs on every commit from `s7g` and fails in BOTH directions"
status: proposed
date: 2026-09-22
legacy_id: none
source: references/adr/0003 (the five-port inventory the lint reads); references/2026-09-08-architecture-design.md:265 (`Tracker`, Phase-1 real adapter *"none in Phase 1 (GitHub is Phase 3)"*) and :266 (`DocsSite`, *"none in Phase 1 (mdBook is Phase 3)"*); references/2026-09-08-acceptance-criteria.md:7506 (*"AC-499 instantiated only for ports that have a Phase-1 real adapter (ADR-0094)"*; AC-507/AC-508/AC-510 listed out of Phase-1 scope, Phase 3); runbook/phase-1/s7a-ports-and-fakes.md:47 and :198-199 (*"`Tracker` and `DocsSite` have a fake only until Phase 3"*); runbook/phase-1/s7f-conformance-and-records.md:50 (the armed table row, *"`s7g` (toolchain), later (tracker/docs)"*); crates/rk-kernel/tests/it/conformance.rs:363-454
supersedes: []
superseded_by: []
related: ["0003", "0094", "0099"]
evidence: []
consumers: []
---

## Context and problem

`s7f` authored `conformance_every_port_has_a_call_site` to compare the **built surface** against
**ADR-0003's closed port set**. It parses the five field names out of `rk-ports/src/root.rs`'s
`pub struct Ports`, scans `crates/rk-kernel/src` (excluding `replay.rs`) for both receiver spellings
`ports.<field>` and `ports().<field>`, and asserts `uncalled.is_empty()`.

**Measured 2026-09-22, counted exactly as the lint counts:** `docs` **0**, `harness` **1**, `store` **1**,
`toolchain` **0**, `tracker` **0**.

`s7g` wires `toolchain`. `tracker` and `docs` remain **0** — and the reason is not that no stage has got to
them yet. **The design gives neither port a Phase-1 real adapter at all:**

- design:265 — `Tracker`, Phase-1 real adapter column: **"none in Phase 1 (GitHub is Phase 3)"**.
- design:266 — `DocsSite`, same column: **"none in Phase 1 (mdBook is Phase 3)"**.
- The acceptance corpus is explicit that the conformance claim is scoped to match: AC-499 is
  *"instantiated only for ports that have a Phase-1 real adapter (ADR-0094)"*, and AC-507/AC-508/AC-510 —
  the GitHub and mdBook criteria — are listed **out of Phase-1 scope, Phase 3**.
- `s7a` §7a.1:47 says the same in the runbook's own voice: *"`Tracker` and `DocsSite` have a fake only
  until Phase 3, and AC-499's matrix carries the phase clause."* §7a:198-199 records the consequence for
  their types — `ExternalRef` and `TypedInput` are authored as **contentless opaque newtypes**
  (`pub struct ExternalRef(pub(crate) ());`) *"so no adapter and no kernel consumer of either type exists
  before Phase 3"*, and **AC-508's idempotency key is deliberately not designed**.

**So `s7f`'s own table cell — *"`s7g` (toolchain), later (tracker/docs)"* — means Phase 3, and no stage in
this runbook claims either port.** `s7h`, `s7i` and `s8` never mention them.

**The lint therefore asserted a property Phase 1 is designed never to satisfy.** An assertion that cannot
become true is an assertion that is never run: it stays `#[ignore]`d for the life of the phase, which is
exactly the coverage theatre the module's own docstring says it exists to prevent — *"an armed lint is
coverage theatre unless something counts it"*. The instrument was measuring a real gap for `toolchain` and
a **permanent, decided, out-of-phase** gap for the other two, and reporting both in one emptiness.

## Decision drivers

- A gate must be able to go green by the work the phase actually contains. `s7g` can wire `toolchain`; it
  cannot wire a port whose argument type is a newtype with no field set and whose idempotency key a record
  defers by name.
- The debt for `tracker` and `docs` must stay **visible and attributed**, not deleted. Dropping the two
  ports from the lint's domain would be the same defect in the other direction — a set narrowed until it
  is trivially satisfiable, with nothing recording why.
- This repo already has the instrument shape for exactly this: `runbook/tools/check-checkpoints.py` asserts
  `flagged == allowed`, *"never 'flagged is small'"*, over an allowlist **enumerated by file and line
  content** where *"every entry states why it is admissible … Adding an entry is a claim; state the
  reason."* A per-entry-warranted equality is the house form for a known, bounded, justified exception set.
- `s7g` §7g.4's checkpoint block filters on this test **by name** and asserts rc=0. A resolution that keeps
  the name and makes the test pass needs no amendment to a checkpoint that has never been run.

## Considered options

1. **Equality against a dated, warranted expected-uncalled set (taken).** The test keeps its name, loses its
   `#[ignore]`, and asserts `uncalled == {"docs", "tracker"}` with each entry carrying its own warrant.
2. **Split the lint per port.** Five tests; `toolchain`'s discharges at `s7g`, `tracker`'s and `docs`' stay
   armed naming Phase 3. Rejected as the weaker of two live options, not as wrong: it gives a natural
   per-port discharge commit, but it multiplies an instrument that reads one closed set into five that
   can drift apart, and it forces `s7g` §7g.4's block — which filters on the current name — to be rewritten
   before it is first run, which is a worse trade than the one clause this record adds.
3. **Widen `s7g` to wire all three ports.** Rejected on measured grounds, not on scope. A `Tracker` call
   site would have to pass an `ExternalRef` — `pub struct ExternalRef(pub(crate) ());`, a newtype with no
   contents, deliberately, because `s7a` §7a:198-199 declines to invent a Phase-3 schema. There is no
   meaningful call to write, and writing a meaningless one to satisfy a substring scan is the precise
   defect the adversarial pass over this module already caught three times.
4. **Delete the two ports from the lint's domain.** Rejected: the lint's domain is ADR-0003's closed set,
   **parsed from `Ports` rather than hardcoded** so that a port cannot leave the question by leaving a list.
   Narrowing the parse would reintroduce the vacuity the parse exists to close.

## Decision

1. **`conformance_every_port_has_a_call_site` keeps its name, loses its `#[ignore]`, and asserts an
   equality.** The domain stays ADR-0003's five, **still parsed from `rk-ports/src/root.rs`** with the
   `fields.len() == 5` anchor and the five named-field anchors unchanged. What changes is only the final
   assertion: `uncalled` must equal the expected-uncalled set, exactly.

2. **The expected-uncalled set is `{"docs", "tracker"}`, and each entry carries its warrant in the source,
   not in a commit message.** The warrants are the ones measured above: `docs` — no Phase-1 real adapter
   (design:266, *"none in Phase 1 (mdBook is Phase 3)"*), AC-510 is Phase 3; `tracker` — no Phase-1 real
   adapter (design:265, *"none in Phase 1 (GitHub is Phase 3)"*), AC-508 is Phase 3.

3. **The assertion fails in both directions, and that is the point.** It fails if a port in the expected set
   **gains** a call site without the set and its warrant being moved — so Phase 3 cannot quietly land a
   `Tracker` consumer while a record still says none exists. It fails if a port outside the set **loses**
   one — so `toolchain` cannot regress to zero after `s7g`. The emptiness form could do neither, because it
   never ran.

4. **Moving an entry out of the set is a deliberate act that belongs with the change that earns it**, on
   the precedent of `shipped_pack_loads`'s pinned activity count and of `check-checkpoints.py`'s allowlist.
   Adding an entry is a claim and states its reason.

5. **The `#[ignore]` is removed in the same commit that makes the lint pass.** That convention is
   `s7f`'s and is unchanged here. Its consequence is named rather than discovered: **Checkpoint 7f becomes
   non-re-runnable at that commit.** Its block requires the literal `test result: ok. 0 passed; 0 failed;
   3 ignored` and three ARMED lints, so *any* discharge invalidates it — under every option above, including
   the two rejected ones. Checkpoint 7f is `s7g`'s **entry** gate only and nothing downstream requires it
   green; it attests a specific text at a specific time, which `00-preamble.md`'s maintenance rule already
   provides for.

**What this record does NOT decide.** It does not decide when `tracker` or `docs` gain a real adapter — that
is Phase 3's and ADR-0094 decision 2's. It does not touch `conformance_every_trailer_key_has_a_producer`,
which stays ARMED for `s7h`. It does not change the lint's scan scope, its `replay.rs` exclusion, its
comment-stripping or its string-literal blanking, all of which are `s7f`'s repairs and stand.

## Consequences

**What this forbids**

- Forbids restoring `uncalled.is_empty()` while any port in the expected set lacks a Phase-1 real adapter.
  The emptiness form is unsatisfiable inside this phase and its only reachable state is `#[ignore]`d.
- Forbids adding an entry to the expected-uncalled set without a warrant naming a record or a design line.
  An unexplained entry is an assertion narrowed to fit its own failure.
- Forbids narrowing the **domain** — the set parsed out of `Ports` — as a way of narrowing the assertion. A
  port leaves the question only by leaving `ADR-0003`'s inventory, which `ADR-0003`:44 governs.
- Forbids writing a `ports.tracker` or `ports.docs` call site whose purpose is to satisfy this lint. The
  types are contentless by decision (`s7a` §7a:198-199) and a call built to be scanned is not a call site.
- Forbids removing the `#[ignore]` from a lint in a commit that does not make it pass, and forbids making a
  lint pass in a commit that leaves the `#[ignore]` on.

**What follows**

- The lint **runs** — for the first time — from `s7g` onward, on every commit, instead of being skipped.
  This is the substantive change: the tree gains a live guard where it had a dormant one.
- `s7f` §7f.1:50's table cell keeps its structure but its `store 4` figure is separately wrong and is
  corrected at `s7g` (measured `store` **1**; the 4 counted `store_guard()`, a kernel-private helper, and
  `conformance.rs`'s own module doc already says 1).
- Checkpoint 7f is superseded in fact at the discharging commit, as clause 5 states.
- The Phase-3 obligation is now recorded in **two** places that must agree — this record and the lint's own
  expected set — and a Phase-3 pass that satisfies one and not the other fails the lint.

## Evidence and confidence

- [measured 2026-09-22] Counted as the lint counts (`ports.<f>` or `ports().<f>` over
  `crates/rk-kernel/src`, `replay.rs` excluded): `docs` **0**, `harness` **1**, `store` **1**, `toolchain`
  **0**, `tracker` **0**.
- [documented] design:265 — `Tracker` | … | **"none in Phase 1 (GitHub is Phase 3)"**; design:266 —
  `DocsSite` | … | **"none in Phase 1 (mdBook is Phase 3)"**.
- [documented] AC-499's Phase-1 subset note: *"with AC-499 instantiated only for ports that have a Phase-1
  real adapter (ADR-0094). Out of Phase-1 scope … AC-507/AC-508/AC-510 (Phase 3, GitHub and mdBook)"*.
- [documented] `s7a` §7a:198-199 — the two types are opaque newtypes *"deliberately: an opaque newtype
  declares the port shape without inventing a Phase-3 schema"*, and *"**No field set is invented and no
  idempotency key is designed** (that is AC-508's, Phase 3)"*.
- [measured 2026-09-22] No runbook stage file after `s7f` names either port: `grep` over `runbook/` for
  `ports.tracker` / `ports.docs` / the `Tracker` and `Docs` port spellings returns `s7a`, `s7b-ii` and
  `s7g`'s statement of the gap, and nothing in `s7h`, `s7i` or `s8`.
- **Confidence: high on the phase clause, which is documented in three independent places (the design
  table, AC-499's subset note, and `s7a`'s routing) that agree.** What is **not** measured here is that
  `{"docs", "tracker"}` is still the right set at the moment Phase 3 opens — that is precisely why the
  assertion is an equality rather than a subset, so the question is asked mechanically on every commit
  rather than trusted to this record's date.
