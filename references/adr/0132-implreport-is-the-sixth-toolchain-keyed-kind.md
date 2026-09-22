---
id: "0132"
title: "`ImplReport` is the sixth toolchain-keyed artifact kind, so ADR-0073's keyed-kinds enumeration is superseded in part — both the `## Decision` sentence and the `**What this forbids**` bullet that name the five — and every other clause of 0073 stands; the shipped `.redkiln/artifacts.toml` moves 5 → 6 keyed kinds and `spikes/p9/artifacts.toml` is not edited"
status: proposed
date: 2026-09-21
legacy_id: none
source: references/adr/0073 — the `## Decision` sentence naming five keyed kinds at :47-52 (*"Artifact kinds that vary meaningfully by toolchain are declared `keyed_by = \"toolchain\"` in `artifacts.toml`: `ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`"*) and the `**What this forbids**` bullet at :69-71 that re-enumerates the same five (*"For the five keyed kinds (`ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`) keyed expansion plus an instance-level predicate replaces it outright"*) — two protected sites of one accepted record; references/adr/0092 rule 1 (*"An erratum … leaves every sentence of `## Decision` and every bullet of `**What this forbids**` standing as written. A correction that would move either is a supersession and takes a new record"*); references/2026-09-07-process-catalog-proposal.md §2.5 step 4 (*"take lowest `method_rank`; a tie is a pack validation error, never a runtime choice"* — where the rule originates); references/adr/0056 `**What this forbids**` (*"Rank ties among admissible producers remain forbidden per the pre-existing rule … and are not reopened here"* — the record that keeps it standing); references/2026-09-08-acceptance-criteria.md AC-047 (`RankTie { kind, context, activities }` fails load before any activity is dispatched); the first `redkiln run` ever attempted on this tree, at Step 8, which refused the shipped catalog at `RankTie` over `ImplReport`; the operator's decision of 2026-09-21, recorded as `spikes/DECISIONS-2026-09-21.md` D21-4
supersedes: ["0073"]
superseded_by: []
related: ["0029", "0056", "0092", "0093"]
evidence: ["F35 §1.3"]
consumers: []
---

## Context and problem

**The product's own pack has never loaded, and the refusal is a rank tie ADR-0073 did not anticipate.** Step 8 ran the
first `redkiln run` ever attempted on this tree. Under two pack defects — both repaired there — sat a third that no
fixture could have surfaced, because **every catalog test in this tree runs over a fixture catalog**: the shipped
`.redkiln/` refuses at

```
RankTie { kind: "ImplReport",
          context: "blast_radius=low, change={binary|text}, kind=incident, size=low, toolchain={node|unreal}, rigor=optional",
          activities: ["hotfix", "implement-ue"] }
```

`hotfix` and `implement-ue` both provide `ImplReport` at `method_rank = 10` and are simultaneously admissible.

**The tie is a shadow of a kind that *is* keyed.** Measured 2026-09-21 over all 102 shipped activities there are
**9 equal-rank producer sets**. **5** sit on toolchain-keyed kinds, where step-2 keyed expansion resolves them. Of the
remaining four, **two are `ImplReport` shadowing `Changeset` exactly** — identical producer sets at both ranks,
`dependency-update` / `hotfix` / `implement-ue` / `test-after` at 10 and `bdd-outside-in` / `walking-skeleton` at 30 —
because **every activity that provides `Changeset` also provides `ImplReport`**, and `Changeset` is keyed while
`ImplReport` is not. The two kinds are the same activities' two outputs; one expands per toolchain and the other does
not.

**The other two equal-rank pairs are not ties at all** and no change is owed for them: `checklist-review`
(`kind:chore`, `kind:docs`) and `spike-decision-review` (`kind:spike`) are disjoint by tag, as are `fix-spec`
(`kind:bug`, `kind:security-vuln`, `kind:incident`) and `user-story-ac` (`kind:story`, `kind:project`,
`kind:tech-debt`, `kind:docs`). No item's `kind` is in both sets, so neither pair is ever simultaneously admissible
[measured 2026-09-21]. **Keying `ImplReport` therefore clears the entire remaining rank-tie surface.**

**`artifacts.toml` was not at fault.** The registry carries precisely the five keyed kinds ADR-0073's Decision names.
`ImplReport` — the same activities' other output — is not among them, and **no record states why**. What moves here is
the record, not a pack workaround.

## Decision drivers

- A rank tie fails load before any activity is dispatched (AC-047). Until it is resolved the product cannot run at
  all, on any item, in any lane — the refusal is on catalog load and is independent of the item.
- The rule that makes a tie fatal is deliberate and is not reopened: *"take lowest `method_rank`; a tie is a pack
  validation error, never a runtime choice"* (process-catalog-proposal §2.5 step 4, SHOP2 ordered methods), kept
  standing by ADR-0056's forbids bullet. Resolving this tie by moving a `method_rank` would encode a method preference
  nobody holds, and would resolve by nudging a number the precise thing the rule exists to refuse.
- `ImplReport` and `Changeset` are produced by the same activities. Keying one and not the other is the anomaly;
  keying both is the shape ADR-0073's own mechanism already describes.
- ADR-0073's warrant for keying — F35 §1.3's measured failure, a mixed Node+UE item receiving the UE `ChangeSummary`
  only, **silently** — applies to `ImplReport` unchanged. A mixed-toolchain item today receives one implementation
  report for two toolchains' worth of work, by the same mechanism and with the same silence.

## Considered options

1. **Key `ImplReport` (taken).** Step-2 keyed expansion makes the producers non-simultaneous, exactly as it already
   does for `Changeset`. Clears all remaining ties. Costs one supersession, because the enumeration is in two
   protected sites.
2. **Move a `method_rank`.** Rejected: forbidden in substance by the rule above, and it would silently encode
   "`hotfix` is preferred to `implement-ue`" (or the reverse) as a global method ordering on the strength of one
   context.
3. **Narrow a tag or an `applicable_if`.** Rejected: it would make `hotfix` or `implement-ue` inapplicable to a
   context it is genuinely applicable to, to satisfy a registry omission. It also fixes one pair and leaves the
   second `ImplReport`/`Changeset` shadow standing.
4. **Amend ADR-0073 by erratum.** **Not available.** ADR-0092 rule 1 is explicit that an erratum leaves every
   `## Decision` sentence and every `**What this forbids**` bullet standing as written, and that a correction moving
   either is a supersession. The enumeration sits in both.

## Decision

**`ImplReport` is declared `keyed_by = "toolchain"`.** The toolchain-keyed set is **six**: `ChangeSummary`,
`TestReport`, `ReproCase`, `Changeset`, `ProfileReport`, **`ImplReport`**. A required `ImplReport` expands to one
instance per element of the item's `toolchain` set, with one producer per instance, on ADR-0073's unchanged mechanism.

**This record supersedes ADR-0073 IN PART, at two sites, and names both.** ADR-0092 rule 1 protects
`**What this forbids**` bullets exactly as it protects `## Decision` sentences, and ADR-0073 enumerates the five in
both places:

1. the `## Decision` sentence at `0073:47-52` — *"…declared `keyed_by = "toolchain"` in `artifacts.toml`:
   `ChangeSummary`, `TestReport`, `ReproCase`, `Changeset`, `ProfileReport`"*; and
2. the `**What this forbids**` bullet at `0073:69-71` — *"For the five keyed kinds (`ChangeSummary`, `TestReport`,
   `ReproCase`, `Changeset`, `ProfileReport`) keyed expansion plus an instance-level predicate replaces it
   outright"*.

**Only the enumeration moves, at both sites.** `ImplReport` is added; the five already named are unchanged; no other
clause of 0073 is withdrawn. ADR-0073 stays `accepted` and gains `superseded_by: ["0132"]`.

**Everything else in ADR-0073 stands, and the two clauses most easily mistaken for casualties are named so they are
not re-litigated:** PC7(ii)'s specificity-beats-generic tie-break stays **withdrawn, not demoted to a fallback**; and
0073's own scope limit stands verbatim — *"What replaces the tie-break for kinds that are not keyed is not stated in
the source, and is not asserted here."* This record does not assert it either. Keying a sixth kind removes the
remaining ties in the **shipped** catalog as measured; it states no general rule for unkeyed kinds.

**The pack half.** `.redkiln/artifacts.toml` gains `keyed_by = "toolchain"` on `[artifact.ImplReport]` and moves
5 → 6 keyed kinds. **`spikes/p9/artifacts.toml` is NOT edited** and stays at 5: `s7b` §7b.12 forbids editing the
spike registry to make a case go green, and `.redkiln/` is derived output, which is the half this stage may author.
The two files are byte-identical today (6223 bytes each) and diverge here by exactly one line, deliberately.

**No `method_rank` moves**, in this record or in the pack change it authorises.

## Consequences

**What this forbids**

- Forbids resolving a producer-of-a-kind rank tie by moving a `method_rank`, by declaration order, by file order or
  by name. The rule of process-catalog-proposal §2.5 step 4 is untouched and AC-047 still fails load.
- Forbids editing `spikes/p9/artifacts.toml` to match. The spike registry is a frozen corpus; a divergence between it
  and `.redkiln/` that this record creates is a fact about the two registries, not a defect to reconcile.
- Forbids reading this record as a general rule for kinds that are **not** keyed. ADR-0073's explicit non-assertion
  survives and is restated above.
- Forbids treating `ImplReport` as keyed anywhere the item carries no `toolchain` set — keyed expansion is over the
  item's toolchain elements and is unchanged by this record.

**What follows**

- The shipped pack loads. `shipped_pack_loads` (`crates/rk-kernel/tests/it/conformance.rs`) asserts the product's own
  `.redkiln/` passes all four load-time lints — `requires_cycle_lint`, `excludes_lint`, `dangling_requires_lint`,
  `rank_lint` — and is GREEN from this record's pack change. It was RED on precisely this `RankTie` before it.
- `redkiln run` gets past catalog load. It then refuses on `s7g`'s inputs — `.redkiln/schemas/` absent, 0 of 8 cards
  carrying `permission_mode`, no `model`/`effort` in either pack, 20 of 102 activities carrying no `role`. That is
  expected and is not this record's subject.
- A reader of ADR-0073 must follow `superseded_by` to learn the set is six. The link is written in both directions in
  the same commit; a one-sided link is a defect, not a shorthand (`adr/README.md`).

## Evidence and confidence

- [measured 2026-09-21] The refusal itself: `./target/debug/redkiln run rk-fix-doctor-test-count --lane fix` exits
  **3** with `RankTie { kind: "ImplReport", … activities: ["hotfix", "implement-ue"] }`, reproduced on consecutive
  sequential invocations.
- [measured 2026-09-21] 9 equal-rank producer sets over the 102 shipped activities; 5 on keyed kinds; 2 being
  `ImplReport` shadowing `Changeset` with identical producer sets at ranks 10 and 30; 2 disjoint by `kind` tag and
  never simultaneously admissible.
- [measured 2026-09-21] `.redkiln/artifacts.toml` and `spikes/p9/artifacts.toml` are byte-identical at 6223 bytes,
  320 LF, no BOM, with `keyed_by = "toolchain"` at the same five sites.
- [documented] F35 §1.3 is ADR-0073's own evidence and is not re-measured here; this record extends its mechanism to
  a sixth kind rather than resting on a new measurement of the failure it describes.
- **Confidence: high on the tie surface, and the limit is stated.** The enumeration of 9 equal-rank sets is over the
  **shipped** catalog at this pin. `spikes/p9/catalog/` holds 102 files too and produces identical step-kind totals,
  but `diff -rq` reports **10 differing files**, so a measurement taken there does not transfer. The claim
  "keying `ImplReport` clears the entire remaining rank-tie surface" is about `.redkiln/` and is asserted by
  `shipped_pack_loads`, which re-measures it on every run rather than trusting this sentence.
