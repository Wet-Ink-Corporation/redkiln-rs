---
id: "0119"
title: AC-045's key is a fourth, selection-level key derived in rk-kernel::select and looked up outside select() — ADR-0097's step memo key gains no ninth component and is untouched
status: accepted
date: 2026-09-13
legacy_id: none
source: references/2026-09-08-acceptance-criteria.md — AC-045, AC-043, AC-040, AC-041, AC-049, AC-014, AC-016, AC-019, **cited by id and named clause rather than by line**: the register was under concurrent amendment while this record was written and its line numbers moved by 30 during the task [measured, 2026-09-13 — `**AC-040**` read at :692 and again at :722 within one hour, the file `M` against `60f17c6` both times]; references/adr/0097-the-memo-key-is-one-hashed-tuple-and-the-memo-stays-tier-2.md :48-59 (the eight components), :60-65 (the other two keys), :102 (the forbidding clause); references/2026-09-08-architecture-design.md §3.5 :317-332, §4.4 :503-525 (:505 the four digests, :512 step 4, :513 `dag_assembled`, :525 the colliding sentence), :710 (the event-kind row); references/adr/0099-harness-dispatch-blocks-cancel-is-a-token-and-containment-is-per-os.md :86-89 (`rk-select` is a module of `rk-kernel`); crates/rk-store/src/memo.rs; runbook/00-preamble.md :53 — the last four all clean at `60f17c6`; the Step 7b audit dossier finding `ro-4` (adjudicated, unanimous) and the operator's decision of 2026-09-13 taken against it — there is no `spikes/DECISIONS-2026-09-13.md` and this record is that decision's primary record
supersedes: []
superseded_by: []
related: ["0021", "0022", "0024", "0072", "0086", "0097", "0099"]
evidence: ["F19", "F35"]
consumers: []
---

## Context and problem

**AC-045 names a key that does not exist, and nothing in the record says which key it meant.**

AC-045's GIVEN/WHEN/THEN reads, in full:

> GIVEN two selection calls differing in exactly one of pack digest, catalog digest, context digest or any artifact
> digest / WHEN a memo lookup is performed for a node common to both DAGs / THEN the second call misses the memo /
> AND the key type admits no construction that omits any of the four.

Its *Verified by* is *"a table-driven test perturbing one digest at a time (4 cases × N nodes), plus a
struct-level test that the key type has no `Default` and no optional digest field."*

ADR-0097 enumerates every key this system has, and there are three. Decision 2 (:48-59) fixes the **step memo key**
as SHA-256 over a length-prefixed tuple of *"exactly these components, in this order"* — eight: `activity_id@version`;
the rigor variant; the step id within the activity graph; the digest of each **resolved step input**; the card digest
or the tool's version-plus-command; the resolved KB atom digests; the loop iteration counter; the output type's
`schema_version`. The same decision (:60-65) names the **activity-level key** (AC-016) — SHA-256 over the activity's
reachable card digests — and the **idempotency key** — `(item, activity_id@version, attempt)`. **None of the three
carries a pack digest, a catalog digest, a context digest or an artifact digest** [measured, the three formulas read
off ADR-0097 :48-65 at `60f17c6`].

So AC-045 is unsatisfiable against the shipped key set, in both directions and silently:

- **A pack-digest change does not miss.** The pack digest changes when any activity anywhere in the pack changes.
  A node common to both DAGs whose own eight components are unmoved hits its step memo. AC-045's pack-, catalog- and
  context-digest rows are therefore red against a *correct* implementation of ADR-0097; only its artifact-digest row
  can pass, and only where the perturbed artifact happens to be a resolved input of that node's steps (component 4).
  Three of four cases red, one passing for an unrelated reason, is the worst available signal.
- **The "no construction that omits any of the four" clause has nothing to bind to.** `StepKeyComponents` has eight
  named fields and none of them is one of the four; the struct test AC-045 names would pass vacuously.

And the gap is not a drafting slip that can be repaired downward. **ADR-0097 :102 forbids the obvious repair:**
*"Forbids hashing the memo key over any tuple but the one above, or without length prefixes and canonicalisation."*
Under ADR-0092 the accepted ADR outranks the register, so AC-045 cannot amend ADR-0097 by being read literally.
`rk-store` shipped the prohibition as a type fact at Step 6 — `crates/rk-store/src/memo.rs` :5-7, *"the components
are a **struct with eight named fields** rather than a `Vec`... a ninth component or a reordering is **a compile
error rather than a silently different key**"*, with `COMPONENT_COUNT: usize = 8` at :83 [measured at `60f17c6`].

**One sentence in the record leans the other way, and it is the sentence that made the collision.** Design §4.4
:525: *"The memo key admits no construction that omits any of the four digests (AC-045)."* The definite article
reads as ADR-0097's step memo key. The design is prose the ADRs outrank, and §3.5 :317-332 already defers to
ADR-0097 clause by clause with `[amended 2026-09-11, PIA-008]` marks — but it is the reason a reader could take
either route in good faith, and it is why this is a record-owed residual rather than an execution detail.

The residual blocks **AC-043** as well as AC-045. AC-043's *Verified by* is *"an event-log assertion that
`memo_hit` and `memo_evicted` never name the same `(run_id, activity_id)`"* — a refusal stated over entries
identified by activity, which is the grain AC-045's node is at and which no key formula currently reaches with the
four digests in it.

## Decision drivers

- **The two routes are not symmetric, and nothing told the stage that.** A fourth key supersedes nothing. A ninth
  component is a successor to ADR-0097 that also moves shipped code built specifically to refuse it, and two
  criteria whose property-case counts are written as the word "eight" (AC-014's *Verified by*, AC-019's THEN).
- **The four digests are not step-grain inputs.** `select()` is *"a function of `(pack digest, catalog digest,
  context digest, artifact digests)` and of nothing else"* (design :505; AC-040's GIVEN). Those four are constant
  across every step of one selection call. ADR-0097 decision 2 already has a slot for what a step depends on —
  component 4, *"the digest of each **resolved step input**"* — and a pack digest is not a resolved step input.
- **A coarse component in a fine key buys nothing and costs the memo.** The pack digest moves when an unrelated
  activity elsewhere in the pack moves. As component 9 it would miss every step memo in the system on any pack edit,
  for a discrimination no criterion asks for; ADR-0097's Consequences already carry recomputation cost as the
  record's named negative.
- **`select()` must not read a store.** AC-041's THEN — the selection crate *"cannot read wall-clock time,
  environment variables, the filesystem, the network, or a random source — the four digests and their expanded
  values are its only inputs"*. A memo lookup is a store read. If this record does not say where the lookup lives,
  the first implementer puts it inside the pure function and puts back the hidden input AC-041–AC-043 exist as a
  chain to prevent.
- **The residual must be closed where a reader will meet it.** Step 7b builds `rk-kernel` and claims AC-043 and
  AC-045 in its acceptance set. `00-preamble.md:53`: *"Where a stage is tempted to decide something else, it
  stops."* Defaulting it in a stage file either amends an accepted ADR from a runbook, or leaves the step memo
  indifferent to a pack-digest change — silently, either way.

## Decision

**1. AC-045's key is a fourth key — the `SelectionMemoKey` — and ADR-0097 is untouched.** This record carries
`supersedes: []`. The step memo key stays at exactly eight components in ADR-0097's order; `StepKeyComponents`,
`COMPONENT_COUNT` and `activity_key` in `crates/rk-store/src/memo.rs` are not moved by this record, and ADR-0097
:102 stands unweakened — this record deliberately does not become its successor.

**2. What it keys.** One entry per **DAG node per selection input set**: the node's selection result — the producer
resolved for it and the sub-DAG rooted at that producer's `requires` recursion (design §4.4 step 4, :512). AC-045's
WHEN *"a memo lookup is performed for a node common to both DAGs"* presupposes this memo; this record names its key
and does not invent the memo.

**3. What goes in it.** `SHA-256` over a length-prefixed, ordered tuple of exactly these components, in this order,
each encoded as `u32-length || bytes`:

1. the **pack digest**;
2. the **catalog digest**;
3. the **context digest**;
4. the **artifact digests**, sorted byte-wise and folded into one component by hashing their own length-prefixed
   sequence first;
5. the **node's identity in `dag_assembled.nodes`** — `activity_id@version` (ADR-0072's version, the spelling
   ADR-0097 component 1 uses), **plus the keying discriminator where the required kind is declared
   `keyed_by = "toolchain"`** (ADR-0073; AC-049, whose *Verified by* asserts *"two `ChangeSummary` nodes in
   `dag_assembled`"* for one mixed item).

Components 1–4 are AC-040's GIVEN tuple and nothing else, so the type carries all four non-optionally and AC-045's
struct test — no `Default`, no optional digest field — binds to real fields. **Component 5 is not one of the four
and does not weaken the clause:** "admits no construction that omits any of the four" is a floor, and AC-045's own
instrument is *"4 cases × N nodes"*, which requires the key to vary per node. Component 5's `keyed_by` half is
**[inferred]** from AC-049, not quoted from it: without it two nodes of one DAG collide on one memo entry, and the
defect F35 §1.3 measured — a mixed item silently receiving one instance — would come back through the memo.

No canonicalisation clause is needed and none is granted: components 1–4 are already digests. ADR-0097 component
4's canonical-serialiser requirement is about *values*, and this key hashes no value.

**4. `inputs_digest` is the fold of components 1–4, and the two derivations live on one type.** AC-040's THEN requires
*"an identical `inputs_digest`"* and fixes no formula for it; `dag_assembled { inputs_digest, nodes, edges,
rows_fired, skipped }` is emitted at selection step 5 (design :513, :710). It is now the SHA-256 of the
length-prefixed tuple of components 1–4, derived from the same struct the key is derived from. AC-040's replay
identity and AC-045's four-row perturbation table are then the same instrument's two halves, and an
`inputs_digest` that did not move while a selection input did is a compile-level impossibility rather than a test's
job.

**5. The key is derived inside `rk-kernel::select`; the lookup happens outside `select()`.** The type and both
derivations live in `rk-kernel::select` (`rk-select` is a module of `rk-kernel`, ADR-0099 clause 5 :86-89). **No
store read, no cache read and no event-log read occurs inside `select()`** — the caller at the runtime boundary
computes the key from the four digests it is about to pass, looks it up, and either serves the node's memoised
result or calls `select()`. This is AC-041's THEN restated at the one call site that would have broken it.

**6. It shares no code with the other three, for the reason `rk-store::memo` gives.** `crates/rk-store/src/memo.rs`
:18-20: *"They deliberately do not share code. A helper that computed 'the key' would be the thing that makes two of
them accidentally equal."* That rule holds for the fourth. Using `rk_types::digest::sha256_lp` — the length-prefixed
primitive all four use — is not sharing a key derivation; a shared *key function* is what is forbidden.

**7. AC-043's refusal is grain-independent and reaches this key.** An entry minted by a run whose replay assertion
failed is refused and re-materialised whatever grain it was keyed at, the selection memo included. What this record
does **not** decide is AC-043's third state — an entry minted by a run carrying no replay verdict yet, which
AC-043's GIVEN (*"a run whose replay assertion (AC-042) **failed**"*) does not cover and its poisoned-run-id
fixture cannot distinguish. That is a dated amendment owed to the AC register, with a second refusal reason
alongside it; it is named here so it is not mistaken for something this record closed.

**8. What this record leaves owed, named so it is not a residual in a smaller form.**

- **A dated amendment to AC-045** naming the key it means, and the matching correction to design §4.4 :525's *"The
  memo key"* → *the selection memo key*. Until it lands, a stage claiming AC-045 reports it **armed, not
  discharged** (golden constraint 10). This record is the authority for that amendment; it is not the amendment.
- **A doc amendment to `crates/rk-store/src/memo.rs` :9**, whose *"Three different keys, three different formulas"*
  becomes an under-count. It is a comment, not a normative claim, and the fix belongs with whoever next edits that
  file; nothing in `rk-store` changes behaviour.
- **What the selection memo's stored value is typed as** is Step 7b's to write under AC-046's explanation row, not
  this record's. Only the key is decided here.

## Considered options

### Option — a ninth component on ADR-0097's step memo key

Add the four selection digests (folded) as component 9, superseding ADR-0097 decision 2 in part: the eight-component
list and :102's forbidding clause are withdrawn and replaced by a nine-component one; decisions 1 and 3, the tier-2
narrowing and the `live = true` liveness rule, stand. ADR-0097 stays `accepted` and carries `superseded_by`.

**Rejected on the grain, not on the cost.** The cost is the visible half and is real — a successor to ADR-0097; a
`superseded_by` edit on it and a row in the README's supersession table; `StepKeyComponents` 8 → 9 fields,
`COMPONENT_COUNT` 8 → 9, the `digest()` array, the module doc built to say a ninth is a compile error, and the test
asserting the field count, all shipped at Step 6; AC-014's *"(**eight** cases, one per component)"* and AC-019's
*"its **eight** components"* each needing a dated amendment; design §3.5's row and §5.4's rebuild table. But the
sufficient ground is that it answers a node-grain criterion by changing a step-grain key. A pack digest is constant
across every step of a run, so as component 9 it discriminates nothing *within* a selection and destroys reuse
*across* selections for every step whenever any byte of the pack moves. And it pushes selection vocabulary — the
pack, the catalog, the context — into `rk-store`, a crate whose key type deliberately knows only about steps.

### Option — read AC-045 as already satisfied by component 4

The claim would be that a pack, catalog or context change reaches the step key through component 4's resolved step
inputs, or through component 1's `activity_id@version`, or component 5's card digest. **Rejected as false, and it is
the failure mode this record exists to prevent because it looks true.** It holds for the changes that touch the
node — an edited card, a bumped activity version — and fails for exactly the ones AC-045 tests: a pack digest that
moved because a different activity changed, a catalog digest that moved because a different kind was registered.
Those are the first two rows of AC-045's table and they would be red with no hint that the key, not the test, was
wrong.

### Option — leave it to Step 7b and let the implementer choose

**Rejected: it is the shape that produced the residual.** The dossier's `ro-4` is unanimous that both routes are
open and that the stage file claims AC-045 without raising the question. A stage that defaults it either amends an
accepted ADR from a runbook file or ships a memo indifferent to a pack-digest change, and either way the choice is
invisible in the diff. This is `00-preamble.md:53` exactly.

## Consequences

**Positive**
- AC-045 becomes satisfiable without moving an accepted ADR, a shipped crate, or a criterion's property-case count.
  The whole blast radius is code that does not exist yet plus one register amendment and one design correction.
- ADR-0097's compile-time guard survives intact: a ninth component on the step memo key is still `E0004`-grade
  breakage in `rk-store`, and that guard was worth having.
- The one place the four digests could have re-entered `select()` as a hidden input — a memo read inside the pure
  function — is closed by clause 5 before the first call site is written, not after.
- AC-040 and AC-045 stop being two unrelated assertions: `inputs_digest` and the selection memo key are derived from
  one type, so a drift between them is a compile error.

**Negative**
- **The system now has four key formulas where a reader of `rk-store::memo` will find three.** That comment is wrong
  from this record's acceptance until it is amended, and the amendment is owed, not done. One more key is one more
  thing two implementers can conflate; the mitigation is only the no-shared-code rule, which is a convention with a
  test, not a proof.
- **AC-045 stays armed and not discharged until its amendment lands.** This record settles which shape the key is;
  it cannot discharge a criterion whose own text still names a key that does not exist.
- **Component 5's `keyed_by` half is [inferred].** No criterion states that a DAG node's memo identity must carry
  the toolchain discriminator. It follows from AC-049's two-nodes-one-item fixture, and if AC-049's expansion is
  ever re-shaped the inference has to be re-checked rather than carried.
- **Nothing here is measured.** No memo hit rate, no collision rate, no cost. The choice between a fourth key and a
  ninth component is argued from grain and from blast radius, and this record says so rather than dressing it as a
  finding.

**What this forbids**
- Forbids adding a ninth component — or any component — to ADR-0097 decision 2's step memo key; that still takes a
  successor to ADR-0097, and this record is not one.
- Forbids satisfying AC-045 with `StepKeyComponents`, the activity-level key (AC-016) or the idempotency key, and
  forbids adding a pack, catalog, context or artifact digest to any of the three.
- Forbids any read of a store, a cache, an event log or a filesystem path inside `select()`, including a selection
  memo lookup — the key is derived there, the lookup is performed by the caller (AC-041).
- Forbids a `SelectionMemoKey` constructor that omits, defaults or makes optional any of components 1–4, and forbids
  deriving `Default` for it.
- Forbids a key whose node component is `activity_id@version` alone where the required kind is declared
  `keyed_by = "toolchain"` — two nodes of one DAG may not share one memo entry.
- Forbids a shared helper that computes more than one of the four keys, and forbids deriving `inputs_digest` by any
  formula other than the fold of components 1–4.
- Forbids a stage reporting AC-045 or AC-043 as discharged before the register amendments clause 8 names have
  landed.

## Evidence and confidence

- [documented, `references/2026-09-08-acceptance-criteria.md`, AC-045] its GIVEN/WHEN/THEN and *Verified by* in full, including the four digests,
  the node grain, and the struct-level instrument — quoted above at HEAD.
- [documented, `references/adr/0097-...` :48-59, :60-65, :102] the eight components, the other two keys, and
  *"Forbids hashing the memo key over any tuple but the one above."*
- [measured, `crates/rk-store/src/memo.rs` at `60f17c6`, shipped at Step 6] the step key is a struct of exactly
  eight named fields (:30-49), `COMPONENT_COUNT: usize = 8` (:83), `activity_key` is a separate derivation (:96-104)
  and `IdempotencyKey::PART_COUNT = 3` (:119) — and none of the three types carries a pack, catalog, context or
  artifact digest.
- [documented, `references/2026-09-08-architecture-design.md` :505] *"`select()` is a function of `(pack digest,
  catalog digest, context digest, artifact digests)` and of nothing else"*; :513 and :710, `dag_assembled
  { inputs_digest, nodes, edges, rows_fired, skipped }` at selection step 5.
- [documented, register, AC-041's THEN] its *"the four digests and their expanded values are its only inputs"* —
  the ground of clause 5.
- [documented, register, AC-043's *Verified by*] its `(run_id, activity_id)` assertion — the criterion whose grain has no key.
- [measured, F35, n=1 enumeration, one machine] the P6 lattice is 102 activities over 12 packs, 27 s; it is AC-040's
  denominator and the corpus the four-row perturbation table will run against. It measures no key and licenses no
  cross-machine claim (AC-040's *Scope limit*).
- [measured, F19, one operator] 174/174 suspensions lossless and **the FSM was never replayed** — so "the same DAG
  from the same four digests" is unproven on this system, and AC-042 is a first measurement. The selection memo's
  soundness rests on that unproven property, which is precisely why AC-043's eviction rule exists.

**What is inferred, stated plainly.** Three steps here are reasoning, not measurement, and none is dressed
otherwise. (i) That AC-045's "node" is a `dag_assembled` node and therefore at activity grain rather than step
grain — it follows from AC-045's WHEN reading "node" where design :513's `dag_assembled` declares a `nodes` field and from AC-043's `(run_id, activity_id)`, but
the register never defines the word. Note that the conclusion does not depend on it: **under either reading no key
in the record carries the four digests**, and route (b) would still have to pick one existing key to modify.
(ii) That a coarse run-constant component would collapse the step memo's hit rate — reasoned from what a pack
digest covers; **no hit rate has been measured on this system**, and SM-001 forbids an undifferentiated one in any
case. (iii) The `keyed_by` half of component 5, inferred from AC-049 as clause 3 says.

**Confidence: high on what is forbidden, reasoned on which shape.** What is settled and not in doubt is that AC-045
is unsatisfiable as written, that ADR-0097 :102 blocks the direct repair, and that one of the two shapes had to be
recorded before Slice 3 rather than defaulted in a stage file. What rests on argument is the choice of the fourth
key over the ninth component. **What would move it:** if the step memo were ever required to invalidate on a
selection-input change — a criterion no one has written and which AC-015's *"a stale `.kb` atom invalidates stale
work"* deliberately does not imply — the grain argument changes, and that takes a successor to this record and to
ADR-0097 rather than an argument.
