# Amendments owed above the runbook, 2026-09-12 — DRAFT

> **DRAFT — nothing here has landed.** Drafted for **DECISIONS-2026-09-12 E1, E11, E12** and the two errata E13
> raised. Each entry gives the **exact target**, the **frozen text** as it reads today with its line, and the
> **proposed replacement**, so the edit is a paste and an anchor assertion rather than a re-derivation.
>
> The two supersessions from the same pass are separate drafts: `spikes/ADR-0115-DRAFT.md` (E2) and
> `spikes/ADR-0116-DRAFT.md` (E3).
>
> **Order.** §1 (the ERRATA entry) is independent. §2 and §3 must land **together** — an AC amendment whose design
> row still says the opposite is the PIA-075 defect this pass exists to close. §4 is independent.
>
> **After every edit:** re-run `census_adr.py`, `census_anchors.py`, `census_constants.py` (record the new
> AGREE/VARIANT/ABSENT triple and name which constants moved), and `runbook/tools/check-refs.py`.

---

## 1. ERRATA — ADR-0094's Evidence sentence contradicts its own Decision (E12)

**Target:** `references/adr/ERRATA.md`, a **new section** inserted in record-number order — after
`## ADR-0089 — Facet provenance, declared surfaces and abstention` (:206) and before whatever follows it.

**Why an erratum and not a supersession.** ADR-0092 decision 1: an erratum corrects *"an evidence line, a citation,
a number or a characterisation in Context, Drivers, Options or **Evidence**"* and leaves every `## Decision`
sentence standing. The sentence below is in ADR-0094's **Evidence** section, and the correction leaves decision 2
untouched — indeed it exists to stop the Evidence section from appearing to withdraw it.

**Frozen text**, `references/adr/0094-*.md:123` (read 2026-09-12):

> It does not license any claim about the Rust adapter's behaviour — none exists; the fake-conformance row and
> AC-328/AC-329 are what will. It does not decide the Codex adapter's phase or the CI decision ADR-0010 defers.

**Proposed entry:**

```markdown
## ADR-0094 — Phase 1 ships a Rust toolchain adapter; self-hosting means both lanes

**E-0094-1 (2026-09-12, DECISIONS-2026-09-12 E12).** Evidence and confidence: *"It does not decide the Codex
adapter's phase or the CI decision ADR-0010 defers."* — **What is true:** the record's own **decision 2** does
decide the phase, in terms: *"The Harness port's second real adapter (Codex) is **Phase 2** and stays behind the CI
decision ADR-0010 defers"* (:67-69). The Evidence sentence is correct about the **CI decision**, which decision 2
expressly leaves with ADR-0010, and wrong about the **phase**, which decision 2 assigns. Read as written it would
make an accepted Decision self-withdrawing. **What is true, restated:** *"It does not license any claim about the
Rust adapter's behaviour — none exists … It does not decide the CI decision ADR-0010 defers."* [documented,
references/adr/0094 :67-69 vs :123, read 2026-09-12] **What this does not change:** decision 2 stands verbatim, and
with it Codex = Phase 2; `rk-adapter-codex` remains out of Phase 1 under every reading; ADR-0010's CI decision
stays deferred; AC-496 stays PROVISIONAL on S-CODEX, which is unrun.
```

**Also required by the same decision** (not an erratum — the design is not an ADR):

| Target | Frozen text | Proposed |
| --- | --- | --- |
| `references/2026-09-08-architecture-design.md:1905`, the S-CODEX spike-register row | *"No non-Claude adapter enters a gate before it passes E3-shaped checks (ADR-0007); **Phase 4 is the earliest it ships**"* | *"No non-Claude adapter enters a gate before it passes E3-shaped checks (ADR-0007). **Phase 2 is the phase** (ADR-0094 decision 2, 2026-09-11 — this row read "Phase 4 is the earliest it ships" until 2026-09-12 and predates that record); the **gate** bar is separate from the **phase** and is unmeasured while S-CODEX is unrun."* |
| `runbook/phase-1/s7a-ports-and-fakes.md:114` | *"AC-502 (S-CODEX unrun; Phase 4 is the earliest a second harness ships)"* | *"AC-502 (S-CODEX unrun; the second harness is **Phase 2** — ADR-0094 decision 2)"* |

**Note the ordering trap.** The design row is dated 2026-09-08 and ADR-0094 is 2026-09-11, so the design is simply
older — this is a stale copy, not a live disagreement, and correcting it costs nothing in either direction.

---

## 2. AC-072 gains the `nullable` key (E1)

**Target:** `references/2026-09-08-acceptance-criteria.md`, AC-072, the `THEN`/`AND` block at :1378-1390.

**The problem in one line.** ADR-0089 decision 3 makes boolean facets tri-valued and says AC-079's
`gate_opened { reason: unknown_facet }` route *"today … cannot reach [them] because their declared domain is
`[true, false]`"* — but **TOML has no `null` literal**, so `values = [true, false, null]` is inexpressible;
**AC-072 closes the domain vocabulary at three shapes** *"recognised explicitly, by name rather than by omission"*
with a must-fail fixture for a fourth; and **deleting `values` crashes the loader** — `spikes/p9/lint.py:49` is
`FACET_VALUES = {str(v) for f in facets.values() for v in f["values"]}` and raises `KeyError`.

The resolution applies AC-072's **own** principle one level down: nullability is declared **by name**.

**Frozen text** (:1378-1380, 1384-1386):

> THEN load fails unless that name has a live registry entry in `facets.toml` with a declared domain, source,
> anchor and — where `source = "derived"` — a named **`deriver`** identifying the instrument that computes it
> …
> AND the declared domain is one of **three** shapes the loader recognises **explicitly, by name rather than by
> omission**: boolean, ordinal, or **set** (`set = true`) …

**Proposed** — two edits, both additive; the three shapes are **not** touched:

1. In the `THEN` clause, after *"a declared domain, source, anchor"*, insert:
   *"— and, where the facet may abstain, a declared **`nullable = true`**"*.

2. A new `AND` clause immediately after the three-shapes clause:

> AND a facet whose value may be **absent** declares `nullable = true`; absence is never inferred from the domain.
> `null` is *no evidence computed* and is not a member of the declared domain — the three shapes above are
> unchanged and a fourth still fails load. A `require` row or `applicable_if` that reads a `nullable` facet and
> does not handle `null` is a lint error at pack load (ADR-0103 decision 5(i)), and a `null` read at selection
> step 2 emits `gate_opened { reason: unknown_facet, axis }` (AC-079), **never** `Unsatisfiable`, which is step 4's
> and is reserved for predicates that evaluated to `false` (ADR-0103 decision 5(iii)).
> *(Amended 2026-09-12, DECISIONS-2026-09-12 E1: ADR-0089 decision 3 makes the booleans tri-valued but names no
> on-disk form, and TOML has no `null` literal. Declaring nullability by name rather than by omission is this
> criterion's own rule, applied to nullability.)*

3. Its `Verified by:` gains: *"a fixture entry that abstains without `nullable = true`, which must fail load; and a
   positive `nullable = true` fixture that must load and whose `null` read emits exactly one `gate_opened` per read
   facet, never per abstaining facet."*

**Consequential edits, same change:**

| Target | What |
| --- | --- |
| `spikes/p9/facets.toml` | `nullable = true` on the facets that may abstain — the seven ADR-0090 routes to `human` plus `recent_deploy`, which is NOT MEASURABLE at this pin and already emits `null` under `R-recent-deploy-feed-absent` (ADR-0067, the live precedent decision 3 cites). |
| `spikes/p9/lint.py` | read the key; keep `values` required, so :49 is unaffected. |
| `runbook/phase-0/s4a-apply-adr-0089.md` §4a.1 | the registry edit writes it. **The step already states the ADR-0103 ordering correctly** as of 2026-09-12; this adds the on-disk form it was missing. |

---

## 3. AC-402 drops to three entry paths (E11)

**Target:** `references/2026-09-08-acceptance-criteria.md`, AC-402 at :5765-5770.

**The problem.** Three documents enumerate the reconcile set and no two agree. **ADR-0109 decision 1** scopes it —
*"Every CLI entry that **can cause an action**"*. **AC-172:2671 has already been amended** to match ADR-0105 clause
8: *"(`run`, `answer`, `doctor`; `status` is read-only and does **not** reconcile — ADR-0105 clause 8)"*. AC-402
and design :1311 are the **un-updated copies** — PIA-075's class exactly. Note they disagree at *both* ends: AC-402
includes `status` and `explain` and **omits `doctor`**.

**Frozen text** (:5765-5769):

> GIVEN any entry point that acts on an item (`run`, `answer`, `status`, `explain`)
> WHEN it begins
> THEN `reconcile(recorded, world)` has run before any decision is taken, and no CLI flag can skip it.
> *Verified by:* a kernel test enumerating the four entry paths and asserting `reconcile` is called on each; …

**Proposed:**

> GIVEN any entry point that **can cause an action** (`run`, `answer`, `doctor`) — `status` and `explain` are
> read-only and do **not** reconcile (ADR-0105 clause 8; AC-172)
> WHEN it begins
> THEN `reconcile(recorded, world)` has run before any decision is taken, and no CLI flag can skip it.
> *Verified by:* a kernel test enumerating the **three** entry paths and asserting `reconcile` is called on each,
> **and a negative case asserting `status` and `explain` do not call it**; the argument-surface snapshot (AC-400)
> additionally fails the build on any `--no-reconcile` / `--skip-reconcile` / `--fast` flag.
> *(Amended 2026-09-12, DECISIONS-2026-09-12 E11: this criterion enumerated four paths including `status` and
> excluding `doctor`, against ADR-0105 clause 8 and the already-amended AC-172. ADR-0109 decision 1's scope — "every
> CLI entry that can cause an action" — is the governing form; `doctor` can act (`--fix`, `--rebuild`), `status`
> and `explain` cannot.)*

**Design corrections, same change** — both are stale copies, not live disagreements:

| Target | Frozen | Proposed |
| --- | --- | --- |
| `design:1311`, the verb table's `status` row | *"`status` \| roll-up read; **acts on an item, so it reconciles** \| 1 \| AC-402"* | *"`status` \| roll-up read; **read-only, so it does not reconcile** (ADR-0105 clause 8; AC-172) \| 1 \| AC-172"* |
| `design:170`, principle 4 | *"Git is truth; the record is a cache; `reconcile()` runs at **every entry**. *Forbids:* any optimisation that skips reconciliation…"* | *"…`reconcile()` runs at **every entry that can cause an action** (ADR-0109 decision 1; `status` and `explain` are read-only — ADR-0105 clause 8). *Forbids:* any optimisation that skips reconciliation on an acting entry…"* — the *Forbids* clause is otherwise unchanged, and the narrowing is a scope statement, not a licence to skip. |

**Do not** widen this to a successor against ADR-0105 clause 8: the ADR outranks both the register and the design
(ADR-0092), and F18's 45–48% of gates exceeding 1 h is the cost argument against reconciling on every read.

---

## 4. Two errata the register owes itself (E13)

**Target:** `references/2026-09-08-acceptance-criteria.md`, the phase note at :6263.

**4a. AC-468 is listed twice, in two mutually exclusive sets.** The note names the corpus-bound **Phase-2**
acceptance set as *"AC-444, AC-451–AC-454, AC-456–AC-462, AC-464, **AC-467–AC-469**, AC-475"* and, in the same
paragraph, says what Phase 1's Step 7d owes includes *"the report schema **AC-448/AC-468**"*. AC-468 cannot be both.

> **Proposed:** narrow the corpus-bound range to **AC-467, AC-469** and state the split — *"AC-468's **schema
> conformance** of the differential report is Phase 1's (it is asserted on the v0 dry-run's own report and needs no
> corpus); AC-468's **corpus-scale** assertion over the 1,312-file run is Phase 2's."* If instead the whole of
> AC-468 is corpus-bound, remove it from the owed list and `s7d` drops it — but it must be removed from exactly one
> of the two. *(Raised 2026-09-12, DECISIONS-2026-09-12 E13.)*

**4b. AC-475's placement.** The note lists AC-475 corpus-bound, but its `Verified by:` has two halves — *"a
migration fixture asserting opaque carry with a loss marker, **and a schema assertion of zero `ChangeSummary`
values in migrated output**"* — and the second needs no corpus. It is a **negative** criterion (nothing is
synthesised), it runs on Step 7d's own small dry-run fixture, and `s5 §5.3` already leans on it (*"no
`ChangeSummary` value is synthesised (AC-475)"*).

> **Proposed:** split it the same way — the **loss-marker fixture** half is corpus-bound Phase 2; the
> **no-synthesis schema assertion** is Phase 1's and is claimed at Step 7d as a tripwire. `s7d` has claimed it on
> that basis since 2026-09-12 and says so in its acceptance set. *(Raised 2026-09-12, DECISIONS-2026-09-12 E13.)*

**Neither is an ADR erratum** — the AC register is not an ADR, and ADR-0092's ERRATA convention does not cover it.
Both are dated amendments in the register itself, in the inline `*(Amended <date>, <id>: …)*` form the register
already uses at :249, :297, :1149 and :1199.

---

## What this draft does **not** do

- It does not touch `references/adr/` — `s1` Checkpoint 1 asserts `ls references/adr/*.md | wc -l` is **114**, and
  that checkpoint has already passed at `66276bf`. The two new records land with that count moved deliberately.
- It does not decide ADR-0115's **lock-poisoning** residual or ADR-0116's **not-produced** doctor threshold; both
  are named in their drafts as open.
- It does not re-open ADR-0105 clause 8, ADR-0089 decision 3, or ADR-0094 decision 2. Every entry above is a copy
  being brought into line with a record that already governs it — which is the only kind of edit the precedence
  chain permits without a new decision.
