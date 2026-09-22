---
id: "0140"
title: "The toolchain adapters' construction-time `timeout` is **the only bound** on any child they spawn, not a backstop behind a per-step one — superseding **ADR-0137 clause 5 in part**, at the sentence *\"while a step's own `timeout_secs` remains the kernel's\"* and the floor reasoning that rests on it. Measured: `catalog::Step` carries **no timeout field at all** and `catalog.rs`'s loader reads none, so the shipped pack's **298** `timeout_secs` declarations are read by nothing; the `timeout_secs` that does exist is `ActivitySpec`'s, per ACTIVITY and handed to the **Harness**, which no toolchain child ever sees. The 3600 s default and its optional key stand, re-grounded"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0137 decision 5 (the clause this record supersedes in part); references/adr/0092 rule 1 (a `## Decision` sentence moves by supersession, never by erratum); references/adr/0099 decision 2 and RT-5 (*"the two Toolchain adapters run bounded commands under the same timeout without a token"*); crates/rk-kernel/src/catalog.rs (the `Step` field list, and the loader doc *"It types only what the loader sees … The loader reads no `timeout_secs`"*); crates/rk-types/src/activity.rs (`ActivitySpec.timeout_secs`); crates/rk-cli/src/config.rs and crates/rk-cli/src/verbs/run.rs (the two docstrings that reproduce the withdrawn sentence); references/adr/ERRATA.md E-0137-1, which named this supersession as OWED
supersedes: ["0137"]
superseded_by: []
related: ["0094", "0099", "0127", "0137", "0139"]
evidence: []
consumers: []
---

## Context and problem

ADR-0137 clause 5 reads, in full:

> **`timeout` is a construction-time backstop, and its floor is measured rather than chosen.** The port
> carries no timeout parameter (ADR-0099 decision 2, RT-5), so this single bound backstops every child the
> adapter spawns, **while a step's own `timeout_secs` remains the kernel's**. **The backstop may not be
> smaller than the largest step it could serve, or a declared step timeout would be silently capped.**

**Both emphasised halves presuppose a per-step timeout that something applies. Nothing does.**

- `catalog::Step` carries **eleven** fields — `id`, `kind`, `run`, `output_type`, `role`, `when`, `expr`,
  `for_each`, `retry_max`, `repeat_max`, `memo_declared` — and **none of them is a timeout**.
- `catalog.rs`'s loader documentation says so in terms: *"**It types only what the loader sees** (§7d.8).
  The loader reads no `timeout_secs` and validates no facet, so `RK-D-TIMEOUT` and the three
  `RK-D-FACET-*` rows are `rk-cli`'s own"*. The `RK-D-TIMEOUT` doctor rule reads the TOML directly; the
  kernel's `Step` never receives the value.
- So the shipped pack's **298** `timeout_secs` declarations are read by no executor. **204** are literals
  and **94** are `${toolchain.<key>}` substitutions that nothing on this tree resolves — the gap ADR-0135
  and D21-12 already record against the unshipped `registry.toml`.

**There IS a `timeout_secs`, and naming it is what makes the error precise.** `ActivitySpec.timeout_secs`
exists in `rk-types`, is bound per **activity** by `rk-cli`'s binder (*"`timeout_secs` per activity —
`BindRequest.timeout_secs`"*), and is a field of the spec handed to **`Harness::dispatch`**. It bounds the
model dispatch. **No toolchain child is ever in its scope.** Clause 5's *"a step's own `timeout_secs`"*
conflates an activity-level harness bound with a step-level one that does not exist.

**The consequence is not cosmetic.** A reader of clause 5 — or of either code docstring that reproduces it
— concludes that a `tool` step declaring `timeout_secs = 120` is cut off at 120 s and that the 3600 s value
is a far outer fence. Measured, that step runs under **3600 s and nothing else**. Combined with
`run_tool_step` calling `toolchain.test(&[])`, which the real adapter reads as `cargo test --workspace`,
the practical reading is the opposite of the record's: the bound is not generous, it is the whole story.

## Decision drivers

- ADR-0092 rule 1: this moves a `## Decision` sentence and its stated reasoning, so the route is
  **supersession**, not errata. E-0137-1 corrected what an erratum may correct and named this as owed.
- A bound described as a backstop invites exactly the wrong operational judgement — that lowering it is
  safe because something finer already applies.
- The 3600 s **value** and the optional key are not in doubt; only the justification and the description
  are. A supersession that moved the number would be re-deciding what was decided correctly.
- The catalog's 298 unread declarations are a real gap, and naming it is this record's business; filling it
  is not.

## Considered options

1. **Withdraw the description and the floor reasoning, keep the value, and name what the bound actually is
   (taken).**
2. **Also build a per-step timeout now**, so clause 5 becomes true. Rejected as scope: it needs `Step` to
   carry the field, the loader to read it, the 94 unresolved substitutions to acquire a value source, and a
   decision about which port applies it — four things no record fixes, in a stage already recorded BLOCKED.
3. **Erratum only.** Rejected, and the reason is the one E-0138-1 was corrected for: a `## Decision`
   sentence that is false does not "still stand", whatever the record's age or authorship.
4. **Lower the default now that it is known to be the only bound.** Rejected: it is the only bound, which
   argues for keeping it generous, not for tightening it. A tighter bound would cut off exactly the
   long-running `toolchain:test` invocation the measurement shows is the live case.

## Decision

1. **ADR-0137 clause 5's `## Decision` sentence is superseded in part**, at two places: the clause
   *"while a step's own `timeout_secs` remains the kernel's"* is **withdrawn as false**, and the sentence
   *"The backstop may not be smaller than the largest step it could serve, or a declared step timeout would
   be silently capped"* is **withdrawn as reasoning that rests on it**. Every other clause of ADR-0137
   stands — clauses 1, 2, 3, 4, 6, 7 and 9 unchanged, and clause 8 as ADR-0139 narrowed it. ADR-0137 stays
   `accepted` and carries a second `superseded_by` entry.

2. **The construction-time `timeout` is THE bound on every child a toolchain adapter spawns.** It is not a
   backstop, because there is nothing behind it. Lowering it shortens the only bound a `tool` step has, and
   the prose must say that wherever it is described — including `rk-cli`'s `config.rs` and `run.rs`
   docstrings, which reproduce the withdrawn sentence and are corrected in the commit that lands this.

3. **The 3600 s default and the optional `[adapters] toolchain_timeout_secs` key stand, on re-grounded
   reasoning.** The floor is no longer *"not smaller than the largest step it could serve"* — no step is
   served by it in that sense. It is: **the bound must not cut short a child doing work the pack declares
   it expects to take that long**, and the pack's largest declared literal, **3600**, is the best available
   proxy for that expectation. It remains a measured figure rather than a chosen one, and zero remains
   refused by the adapter (RT-5).

4. **The catalog's `timeout_secs` declarations are recorded as UNREAD, and that is a gap this record opens
   rather than closes.** 298 declarations across the shipped pack, 204 literal and 94 unresolvable, none
   reaching any executor. Whether a step-level bound should exist, which port would apply it, and where the
   94 substitutions get values are **OWED** and belong with the pack-and-registry question ADR-0135 already
   names.

5. **`ActivitySpec.timeout_secs` is untouched.** It is the Harness's, per activity, and this record neither
   widens nor narrows it. A future per-step bound may not be built by overloading it.

**What this record does NOT decide.** It does not change the 3600 s value, the optional key, adapter
selection, or anything in ADR-0139. It does not build a per-step timeout. It does not resolve the 94
`${toolchain.<key>}` substitutions. And it repairs **none** of the executor defects `runbook/PROGRESS.md`
§1's Step 7g row records as BLOCKING — this is bookkeeping correctness, not a repair.

## Consequences

**What this forbids**

- Forbids describing the construction-time `timeout` as a *backstop*, a *ceiling behind* another bound, or
  an *outer fence*. It is the bound.
- Forbids any prose, docstring or record asserting that a step's declared `timeout_secs` is applied,
  enforced or honoured by the kernel or by a toolchain adapter. Nothing reads it.
- Forbids conflating `ActivitySpec.timeout_secs` with a step timeout, and forbids repurposing it as one.
- Forbids lowering the default below the pack's largest declared literal without a record stating what now
  bounds a longer child.
- Forbids resolving a `${toolchain.<key>}` timeout substitution to a value without a record naming the
  table it comes from — `registry.toml` is not shipped (ADR-0135).

**What follows**

- The two `rk-cli` docstrings that reproduce the withdrawn sentence are corrected with this record, on the
  same reasoning that made ADR-0134's warrant strings load-bearing: a false claim in code outlives a false
  claim in prose, because it is read by people editing the thing it describes.
- An operator tuning `toolchain_timeout_secs` now has the true model: one bound, applied to every child,
  with nothing finer underneath.
- ADR-0137 gains a second `superseded_by` entry and stays `accepted`; it is now partially superseded at two
  independent clauses, by ADR-0139 (clause 8) and by this record (clause 5).

## Evidence and confidence

- [measured 2026-09-22] `catalog::Step`'s field list, in full: `id`, `kind`, `run`, `output_type`, `role`,
  `when`, `expr`, `for_each`, `retry_max`, `repeat_max`, `memo_declared`. **No timeout field.**
- [documented] `crates/rk-kernel/src/catalog.rs` — *"**It types only what the loader sees** (§7d.8). The
  loader reads no `timeout_secs` and validates no facet…"*.
- [measured 2026-09-22] `.redkiln/catalog/*.toml`: **298** `timeout_secs` declarations — **204** literal
  (largest **3600**), **94** `${toolchain.<key>}` substitutions.
- [measured 2026-09-22] `rk_types::activity::ActivitySpec` carries `pub timeout_secs: u64`; `rk-cli`'s
  binder documents it as *"`timeout_secs` per activity — `BindRequest.timeout_secs`"*, and the spec is the
  value handed to `Harness::dispatch`.
- [measured 2026-09-22] The withdrawn sentence is reproduced verbatim in two code docstrings —
  `rk-cli/src/config.rs` (*"while a step's own `timeout_secs` stays the kernel's"*) and
  `rk-cli/src/verbs/run.rs` (*"The backstop may not be smaller than the largest step it could serve"*).
- **Confidence: high on the absence, which is the load-bearing claim** — a field list is exhaustive and the
  loader documents the omission itself, so this is a measured absence over a closed enumeration rather than
  a failed search. What is **not** decided, and is stated as open rather than assumed away, is whether a
  step-level bound *should* exist; no measurement here bears on that, and this record does not anticipate it.
