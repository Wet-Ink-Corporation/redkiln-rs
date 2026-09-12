---
id: "0115"
title: The EventStore port is held behind interior mutability; ADR-0099 clause 4's bare-trait-object rule is narrowed to the four ports whose methods take &self
status: accepted
date: 2026-09-12
legacy_id: none
source: spikes/DECISIONS-2026-09-12.md E2; references/adr/0099-harness-dispatch-blocks-cancel-is-a-token-and-containment-is-per-os.md clause 1 (:52) and clause 4 (:80-84); spikes/s-ports/src/lib.rs:174; spikes/F41-DRAFT.md; references/adr/0092-errata-precedence-and-cut-id-reissue.md decision 1 (the erratum/supersession line)
supersedes: ["0099"]
superseded_by: []
related: ["0076", "0086", "0092", "0097", "0102", "0105", "0109"]
evidence: ["F41", "E2"]
consumers: []
---

## Context and problem

ADR-0099 fixes two things that cannot both hold.

**Clause 1** fixes the Harness port's signature, and with it the shape every port method takes a store by:

```rust
fn dispatch(&self, spec: &ActivitySpec, store: &mut dyn EventStore) -> DispatchOutcome;
```

**Clause 4** fixes how the composition root holds the ports: *"Ports are held as `Arc<dyn Port + Send + Sync>`, with
`Send + Sync` as supertraits on all five traits … The boundary is a bare trait object, not a newtype."*

`Arc<T>` hands out `&T` and never `&mut T`. A `dyn EventStore` behind a bare `Arc` therefore cannot produce the
`&mut dyn EventStore` clause 1 requires, and no implementation satisfies both clauses for the EventStore port. This
is structural, not a wording slip.

S-PORTS met it and compiled the resolution — `Arc<Mutex<dyn EventStore>>` at `spikes/s-ports/src/lib.rs:174`, on
both 1.97.1 and the 1.98.1 pin, 5 unit tests and 2 `compile_fail` doctests passing, clippy clean [measured, F41].
But a spike ranks **below** an ADR in ADR-0092's precedence chain, so the only shape that compiles is the one shape
the record does not carry — and `s7a` currently restates clause 4 verbatim while pointing the implementer at the
spike that contradicts it.

**An erratum is not available.** ADR-0092 decision 1 draws the line explicitly: an erratum corrects *"an evidence
line, a citation, a number or a characterisation … and leaves every sentence of `## Decision` … standing as
written. A correction that would move either is a supersession and takes a new record."* Clause 4 is a Decision
sentence. `ERRATA.md` carries **zero** entries for 0099 today, and this is why it must stay that way.

## Decision drivers

- No implementation satisfies both clauses; the record is unbuildable as written, which is the condition a
  successor exists for.
- Clause 4's own stated ground is *"one fake per port swaps at one root … its cost is one vtable call per
  dispatch"*. That ground is about refusing an indirection **of your own**; a standard-library synchronisation
  primitive adds no vtable hop and no second dispatch.
- Four of the five ports are unaffected — their trait methods take `&self` — and a successor that moved them would
  cost more than the defect.
- The narrowing must be stated so the *next* implementer does not re-derive it from a spike, which is exactly how
  the current gap arose.

## Decision

**1. The EventStore port is held as `Arc<Mutex<dyn EventStore>>`** at the composition root. The `Send + Sync`
supertraits stay on the trait declaration (ADR-0099 clause 4, unchanged), which is what makes the `Mutex` `Sync`.

**2. The other four ports are unchanged and this record does not move them.** `Harness`, `Tracker`, `DocsSite` and
`Toolchain` are held as `Arc<dyn Port + Send + Sync>`, exactly as ADR-0099 clause 4 states.

```rust
pub struct Ports {
    pub store:     Arc<Mutex<dyn EventStore>>,   // this record
    pub harness:   Arc<dyn Harness>,             // ADR-0099 clause 4, unchanged
    pub tracker:   Arc<dyn Tracker>,
    pub docs:      Arc<dyn DocsSite>,
    pub toolchain: Arc<dyn Toolchain>,
}
```

**3. Clause 4's prohibition is narrowed, not withdrawn.** What stays forbidden is a **user-defined** wrapper
standing between the `Arc` and the trait object — `struct StoreHandle(Arc<dyn EventStore>)` and every shape like
it. `std::sync::Mutex` is not such a wrapper. The test is: does the type introduce a second dispatch or a seam a
fake would have to be swapped at? `Mutex` does neither.

**4. Clause 1 is unchanged.** `dispatch` keeps `&mut dyn EventStore`. The lock is acquired by the composition root
at the call boundary and released when the dispatch returns; **no port method acquires it, and no lock is held
across a dispatch of another port.**

**5. The `Mutex` is the one permitted interior-mutability seam in the port set.** A second port that later needs
`&mut` takes a successor to this record, not a local decision.

## Considered options

### Option — narrow clause 1 instead

Change `dispatch` to take `&dyn EventStore` and keep the interior mutability inside each implementation, leaving
clause 4 verbatim. **Rejected:** it moves the signature F41 compiled, AC-497's port-shape assertion and design
§3.3 P10, and it pushes the synchronisation decision down into every adapter — five places to get right instead of
one, and a fake that forgets it is a data race rather than a compile error.

### Option — read clause 4 as already satisfied

Argue `Arc<Mutex<dyn EventStore>>` complies because the trait object inside is bare and `Mutex` is not the
user-defined newtype clause 4 forbids, and change nothing. **Rejected:** defensible but undocumented. ADR-0099's
text still literally reads `Arc<dyn Port + Send + Sync>` for all five, so the next reader meets the same
contradiction with nothing to resolve it — and the runbook would keep restating a rule its own reference
implementation breaks.

### Option — an erratum against ADR-0099

**Rejected on mechanism, not on merit:** ADR-0092 decision 1 forbids an erratum that moves a `## Decision`
sentence. Recording this as an erratum would also put it outside the supersession graph `census_adr.py` validates,
where a reader of the index would never find it.

## Consequences

**Positive:**
- The one shape that compiles is the shape the record carries, and `s7a` can point at both without contradiction.
- The narrowing is stated as a test (*does it add a dispatch or a swap seam?*) rather than a list, so the next port
  that needs interior mutability has a rule to apply instead of a precedent to argue from.
- Graph-visible: the `0099 → 0115` edge is validated by `census_adr.py`'s supersession reciprocity and acyclicity
  checks, which an `ERRATA.md` entry would not be.

**Negative:**
- A lock acquisition per dispatch. Unmeasured, and expected to be far below the dispatch's own cost — one harness
  process per activity — but it is a cost clause 4 did not have.
- One more record in a set that has grown by twenty-four since 2026-09-11.

**What this forbids**
- Forbids any user-defined wrapper type between the `Arc` and a port trait object.
- Forbids holding the store lock across a dispatch to any other port.
- Forbids a port method acquiring the store lock itself.
- Forbids extending interior mutability to a second port without a successor to this record.

## Evidence and confidence

- [measured, F41 / S-PORTS, 2026-09-11] `Arc<Mutex<dyn EventStore>>` compiles with the clause-1 signature on
  1.97.1 and on the 1.98.1 pin: 5 unit tests, 2 `compile_fail` doctests, clippy clean. The bare
  `Arc<dyn EventStore>` form does not compile against `&mut dyn EventStore` — that is the finding.
- [documented, references/adr/0092 decision 1] the erratum/supersession line that forces a record here.
- [documented, references/adr/0099 :52, :80-84] the two clauses, quoted above at HEAD.
- **Residual, named not closed:** lock **poisoning**. The Phase-1 store is one writer per process behind a mutex
  (ADR-0105 clause 7), so a poisoned lock means a panic already happened inside a dispatch. This record does not
  decide whether that surfaces as a typed `StepError` variant or as a halt; it is an open item for Step 7a, and it
  must not be silently recovered by `unwrap_or_else(|e| e.into_inner())`.
