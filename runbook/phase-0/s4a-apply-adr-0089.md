# Step 4a — Apply ADR-0089 and ADR-0093: facet provenance, source classes, declared surfaces, and abstention

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s4a` | Phase 0 | Checkpoint 4 | Checkpoint 4a | `s4` | `s2` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


**This step takes no decision.** ADR-0089 is `accepted`, dated 2026-09-09, and partially supersedes ADR-0030 (PC8)
— ADR-0030 **stays `accepted`** and carries `superseded_by: ["0089"]` [documented, ADR-0089]. **ADR-0093**
(2026-09-11) supersedes ADR-0089 in part in turn: `source` gains the closed values **`kernel`** (`stage`, `rigor`,
`role`, `cost`) and **`declared`** (`surfaces`), and only **six** facets stay `derived`, each with a record at the
pin [documented, ADR-0093]. And **ADR-0090** (Step 4.4, just written) makes every `llm` facet — four ordinals and
three booleans — `source = "human"`. What follows is the execution order for the registry, schema and lint work
those three records force. **Run it after Step 4 and before Step 5**; nothing in it touches the selection path, and
all of it is configuration-time. After this step the registry reads: 6 derived · 4 kernel · 1 declared · 9 human ·
**0 llm** — twenty facets.

**Why it exists, in one sentence.** AC-071 gates `source = "llm"` and **nothing gated `source = "derived"`** — so
the semantic facets sat outside the block and ran **ungated while measurably wrong**, at `auth` precision 0.00 over
11 positives [measured, F39 §1]. Four parts.

### 4a.1 `facets.toml` gains a `deriver` field, and no `derived` facet may go without one

Every facet declaring `source = "derived"` must name the instrument that computes it, and no facet may declare
`source = "derived"` without a scored record for that deriver (**AC-072** as amended, **AC-520**). The provenance
classes, in precedence order, are **DECLARED** > **DERIVED** > **PROPOSED** > **ASKED**; a higher class overrides a
lower one for the same fact and the override is logged. **A facet whose only computation is a lexical scan of free
text — file paths and diff lines included — does not satisfy PC8 rule 2 for `require`-row purposes**, whatever its
record says.

The registry that exists today is `spikes/p9/facets.toml`; **Step 7e's pack authoring** derives `.redkiln/facets.toml`
from it (there is no "Step 7b pack authoring" — PIA-011), and Step 2 does not move `spikes/p9/`. Edit the file that
exists.

**Anchor assertion first (golden constraint 7).** **Eleven** facets carry `source = "derived"` today — `stage`,
`rigor`, `toolchain`, `change`, `role`, `cost`, `auth`, `data_migration`, `ui`, `flag_exists`, `recent_deploy`.
**ADR-0093 re-classes five of them before the `deriver` field is added:** `stage`, `rigor`, `role`, `cost` become
`source = "kernel"` (projections of the kernel's own state; no `deriver`, no record, AC-520 does not gate them) and
`auth` is withdrawn for `surfaces` at `source = "declared"` (4a.2). That leaves **six** `derived` facets, and every
one has a record at pin `47faec9f` to name: `toolchain` (0.73 [0.48, 0.89]), `change` (0.72 [0.54, 0.85]), `ui`
(1.00 [0.65, 1.00]), `flag_exists` (0.00 [0.00, 0.66]) [measured, F39], `data_migration` (**NOT MEASURABLE**, 0
positives) [measured, F39 §4] and `recent_deploy` (**NOT MEASURABLE**, feed absent at the pin;
`deriver = R-recent-deploy-feed-absent`) [measured, F39; ADR-0067]. Four of the six are the booleans this record
re-shapes (`data_migration`, `ui`, `flag_exists`, `recent_deploy`; `recent_deploy` already abstains, which is the
precedent, so it needs the registry line and not the mechanism). **Reconcile the count by name, never by the number
eleven** — the previous revision's "both are 11" was the count before the re-class, and it is what made AC-520
unpassable (PIA-001). **Count with a prefix match, never an anchored one:** `rigor` and
`recent_deploy` carry trailing comments, so an anchored `'^source = "derived"$'` undercounts and the prefix form
`'^source = "derived"'` returns the true count on the same file (11 before the re-class, 6 after). Both forms appear
below and in Checkpoint 4a and they must be the same form, or the checkpoint fails on a formatting accident rather
than on the edit.
Assert the shape before you write, and assert *counts*, because a `str.replace` that matches nothing and a
`str.replace` that matches everything look identical afterwards:

```bash
cd D:/repos/redkiln-rs
F=spikes/p9/facets.toml
grep -c '^source = "derived"'  "$F"     # → 11 BEFORE. If 0, you are in the wrong file.
grep -c '^source = "llm"'      "$F"     # → 7 BEFORE (four ordinals, three booleans); 0 AFTER (ADR-0090)
grep -c '^source = "kernel"'   "$F"     # → 0 BEFORE, 4 AFTER (ADR-0093)
grep -c '^source = "declared"' "$F"     # → 0 BEFORE, 1 AFTER (surfaces)
grep -c '^deriver = '          "$F"     # → 0 BEFORE. If not 0, this step has already partly run — read it.
grep -n '^\[facet\.auth\]'     "$F"     # → exactly one line. This is 4a.2's anchor too.
grep -c '^\[facet\.surfaces\]' "$F"     # → 0 BEFORE, 1 AFTER 4a.2
# …make the edits…
d=$(grep -c '^source = "derived"'  "$F"); test "${d:-0}" -eq 6 || { echo "FAIL: ${d:-<unreadable>} derived facets AFTER, expected 6 (toolchain, change, ui, flag_exists, data_migration, recent_deploy)"; exit 1; }
v=$(grep -c '^deriver = '          "$F"); test "${v:-0}" -eq 6 || { echo "FAIL: ${v:-<unreadable>} deriver fields AFTER, expected 6 — one per derived facet"; exit 1; }
# The two counts matching is necessary, NOT sufficient: a bare count that "looks right" is not a check.
# Reconcile the six by NAME here, and note Checkpoint 7e's awk line asserts the same pairing on the
# product registry (s7e Checkpoint 7e, the `dv` line).
```

**Failure mode this step is guarding.** A `deriver` field added to the registry while the lint that reads it is not
built is a label, not a gate — which is exactly the state `source` was in before F39, and exactly how the semantic
facets ran ungated. **Build the lint rule with the field, in the same change** (4a.4), or do neither yet.

### 4a.2 The repo-wide `auth` boolean is withdrawn for a set-valued `surfaces` facet

`auth` is **withdrawn**, not tuned. Eleven positives, eleven false, upper bound 0.26 on the 2026-09-09 scoring (0/11 [0.00, 0.26]; 0/12 [0.00, 0.24] after the 2026-09-11 panel-file key fix scored all 140 cells — F39-DRAFT's Correction; ERRATA E-0089-2) — the rule carries no signal on
this corpus and adding word boundaries to one alternative does not recover it [measured, F39 §1, §2]. Its
replacement is a **set-valued `surfaces` facet** (`set = true`, the shape `toolchain` and `change` already carry)
whose values come from a **checked-in `surfaces.toml` mapping path globs to closed surface names**. A fact is the
**intersection of the changeset with a declared boundary** — a structural question, not a lexical one — and **a
path covered by no glob contributes nothing; it does not imply `false`** [documented, ADR-0089 Decision 2].

Each activity's `applicable_if` then owns its own evidence question (ADR-0024, ADR-0033), and **provenance is
carried on the value** so a `require` row may demand a minimum: a cheap activity may run on a DECLARED fact, an
expensive or destructive one may demand `provenance in ["declared", "asked"]` and refuse to fire on an unconfirmed
proposal.

Three constraints that are easy to lose:

1. **`surfaces` is unscored and may key no `require` row on the strength of ADR-0089 alone.** It may be authored
   and linted. **S-FACET** is the named spike that scores it (ADR-0012); its named consumer is `facets.toml`'s
   `source`/`deriver` fields.
2. **The loader must recognise the set shape explicitly, by name rather than by omission** (**AC-072** as
   amended). It was previously admitted by omission, and ADR-0073 keeps boolean and ordinal registries separate, so
   a third shape entering by omission is how a loader silently accepts a member predicate it has no semantics for.
3. **`surfaces.toml` has an unmeasured maintenance cost.** Hours to seed is the easy half; ongoing drift is the
   real cost and **no source has published it in units comparable to the regex** [documented, ADR-0089
   Consequences]. It is a **reported output of S-FACET**, not an assumption — do not write a number here.

### 4a.3 Boolean facets become tri-valued, and AC-079's route is extended to them

`null` means "no evidence" and **is not `false`**. The registry line `values = [true, false]` is what turned **four
empty diffs into sixteen confident booleans** [measured, F39 §4]: the panel returned `unknown` on all 16 semantic
cells of the 4 items that changed no code, and the script emitted a confident boolean on every one.

**AC-079's existing `gate_opened { reason: unknown_facet, axis }` route is extended to the boolean facets**, which
today it cannot reach because their declared domain is `[true, false]`. **The on-disk form is a declared
`nullable = true` key** beside `values = [true, false]` (DECISIONS-2026-09-12 E1; AC-072 as amended 2026-09-12):
TOML has no `null` literal, so the domain cannot carry it, and AC-072 closes the shape vocabulary at three — so
nullability is declared **by name**, exactly as that criterion requires of the domain itself. **This step writes the
key** on every facet that may abstain, in the same edit as the `source` moves; a facet that can abstain without it
fails load. `values` stays required and is never deleted — `spikes/p9/lint.py:49` reads `f["values"]` unguarded and
raises `KeyError` on a facet without it. The grammar already supports it: ADR-0032
makes absent inputs `null`, ADR-0069 defines `==`/`!=` against `null` while ordering against it is a lint error, and
and **ADR-0103 decision 5 fixes the order of outcomes on a null read**, in three steps that must not be collapsed:
(i) at **pack load**, the lint rejects an unguarded ordering against a nullable facet (AC-069); (ii) at **selection
step 2**, a `null` top-level `applicable_if` or `require.when` emits `gate_opened { reason: unknown_facet, axis }`
(AC-079), **counted once per read facet, never per abstaining facet**, and that is what escalates to the existing
human gate; (iii) **`Unsatisfiable(kind, failed_predicates)` is reserved for step 4**, over predicates that
evaluated to `false`, and is **never** the route for a `null`. Until 2026-09-12 this paragraph sent the uncomputable
boolean straight to `Unsatisfiable`, which ADR-0103 clause 5(iii) and AC-079 both forbid — a `null` is an *unknown*,
and reporting it as unsatisfiable is the fault-name blurring golden constraint 10 exists to stop. The must-fail
fixture ADR-0103 names: **`any` over absent evidence must not pass an activity** (`any` over `null` or an empty list
is `false`; `all`/`none` are `true`, which is why `none` cannot terminate a quality block — AC-527, s7b). **ADR-0067's `recent_deploy` is the live precedent** — it already emits
`null` under rule `R-recent-deploy-feed-absent` because the `toolchain:deploy_log` feed does not exist at this pin.
**The pattern is generalised, not invented.**

**Abstention is reported separately from error, always, and a `null` cell is never counted as a false negative in
any scoring of a facet deriver.** A silent `false` and an honest `null` are different outcomes and may never be
summed.

**The gate load rises, by an amount no document may state.** Four questions on twenty items is a measurement of an
oracle-construction exercise, **not a forecast for a shipped triage gate**, and must not be quoted as one. SM-024
has no baseline; S-FACET is what gives it one. F18 prices a human gate at a median **10.5 min** with 45% exceeding
an hour [measured, n=174, one operator] — that is the price of a question, not a count of questions.

### 4a.4 AC-520 is a new gate and it must fail closed the way AC-071 does

**AC-520** — `redkiln lint --facet-provenance` — is the derived-facet sibling of AC-071, allocated **outside** the
selection area's exhausted `AC-040..AC-079` range [documented, acceptance-criteria register §0.4]. It is **not** a
sixth clause on AC-071: AC-071's own residual-risk sentence names a loosely-implemented multi-clause gate as
precisely the failure mode, and a single gate carrying two subjects, two records and two failure meanings is the
shape that invites the collapse. **Two criteria, two lint rules, two fixture sets, two things that can fail
separately.**

It fails pack load unless a `derived` facet declares a named `deriver` **and** a scored precision record exists for
that deriver over a **pinned** corpus, carrying a **Wilson 95% interval and a positive-instance count**; a facet
whose record shows **no positive instance is reported as *not measurable* — neither passed nor failed — and may key
no `require` row**; and a lexical deriver never satisfies PC8 rule 2 for `require`-row purposes.

**AC-520 deliberately re-states no threshold**, and that is the F39 result showing through: METHOD-P §1.4's 0.95
bar is **not reachable at n=20 for any facet however good** [measured, F39 Limits 1]. The gate requires that a
scored record **exist, be pinned and be read** — not that it clear a number the registered sample size cannot
decide (golden constraint 10).

**Fail closed, and prove it with a positive companion.** Like AC-071 it must fail **today, by construction**, which
is the point. Four must-fail fixture `facets.toml` files, one clause at a time — a `derived` facet with no
`deriver`; a named `deriver` with no scored record; a record carrying 0 positives whose facet keys a `require` row;
a `deriver` declared lexical — **plus a positive companion that must load** (a `derived` facet with a named
`deriver`, a scored record and positives). **Without the positive companion the rule is satisfied by a lint that
rejects everything**, which is golden constraint 9 arriving from the too-wide direction.

**Checkpoint 4a's boolean-domain assertion was wrong, and the record it contradicts outranks it. Corrected
2026-09-12, at the run.** The line asserted `grep -c 'values = [true, false]'` → **0**, which is **jointly
unsatisfiable** with §4a.3 above and with **AC-072 as amended 2026-09-12** (DECISIONS-2026-09-12 **E1**): the
on-disk form of a tri-valued boolean is a declared `nullable = true` key **beside** `values = [true, false]`,
because TOML has no `null` literal, `null` is **not** a domain member, *"the three shapes above are unchanged and a
fourth still fails load"*, and deleting `values` raises `KeyError` at `spikes/p9/lint.py:49`. The assertion was
written on **2026-09-11** (`0e34384`, the runbook split) and E1's prose landed on **2026-09-12** (`8f612a3`) — the
amendment moved the paragraph and left the assertion under it behind. Under ADR-0092's precedence the **AC register
outranks this runbook**, so the assertion is what is wrong. It is replaced by three: the domain count at **7**, the
`nullable` count at **7**, and a per-stanza **pairing** check, since two equal counts are necessary and not
sufficient — §4a.1 says exactly that about `derived`/`deriver`. *(This is the third instance in two days of an
amendment landing in prose while the assertion beneath it stayed: Checkpoint 4's first line, s4 §4.4's anchor table,
and this. When you amend a paragraph, grep the block below it.)*

**Checkpoint 4a.** **Thirteen** against files that exist today, plus **two** marked **`[WILL EXIST]`** — fifteen in
all. The count is given with the rule that produces it, because this tally has been wrong twice: **count statements
that can FAIL** — each carrying `|| { …; exit 1; }` or `awk`'s `exit !(…)` — with backslash continuations joined, and
count the `git add … && git commit` pair as one. On that rule the live block carries **thirteen** of sixteen
non-comment statements, measured 2026-09-12. It read "twelve … Ten" (never decremented when PIA-003 struck the
`surfaces.toml` line), then "eleven … Nine" (which excluded the commit pair without saying so), and the boolean
assertion has since been split into three. The two deferred lines are the lint, which is Phase-1 output that Step 7b
builds; they are **deferred, not failed** here (Halt 8's carve-out) and are re-run as named lines of Checkpoint 7b
(PIA-003).

```bash
cd D:/repos/redkiln-rs
F=spikes/p9/facets.toml
test "$(grep -c '^\[facet\.auth\]' "$F")" = 0 || { echo "FAIL: [facet.auth] still declared"; exit 1; }   # → silent, exit 0: the repo-wide auth boolean is WITHDRAWN, not retuned
test "$(grep -c '^\[facet\.surfaces\]' "$F")" = 1 || { echo "FAIL: [facet.surfaces] stanza count is not 1"; exit 1; }   # → silent, exit 0: its replacement exists
grep -q '^\[facet\.surfaces\]' "$F" || { echo "FAIL: [facet.surfaces] absent"; exit 1; }
test "$(awk '/^\[facet\.surfaces\]/{f=1;next} /^\[/{f=0} f' "$F" | grep -c '^set = true')" = 1 || { echo "FAIL: surfaces does not declare set = true"; exit 1; }   # → silent, exit 0: declared as a SET, by name, inside the surfaces stanza ONLY
test "$(awk '/^\[facet\.surfaces\]/{f=1;next} /^\[/{f=0} f' "$F" | grep -c '^source = "declared"')" = 1 || { echo "FAIL: surfaces is not source = \"declared\""; exit 1; }   # → silent, exit 0: the token AC-072 recognises (ADR-0093), inside the surfaces stanza ONLY
awk '/^source = "derived"/{d++} /^deriver = /{v++} END{printf "%d %d\n", d, v; exit !(d==6 && v==6)}' "$F" || { echo "FAIL: derived count and deriver count are not both 6"; exit 1; }
                                          # → the two numbers are EQUAL, and both are 6 (ADR-0093 decision 2):
                                          #   toolchain, change, ui, flag_exists, data_migration, recent_deploy.
                                          #   NOTE the pattern is NOT anchored with `$`: `recent_deploy` carries a
                                          #   trailing comment, so an anchored match undercounts.
test "$(grep -c '^source = "kernel"' "$F")" = 4 || { echo "FAIL: kernel-sourced facets != 4"; exit 1; }   # → silent, exit 0: stage, rigor, role, cost (ADR-0093 decision 1)
test "$(grep -c '^source = "llm"' "$F")" = 0 || { echo "FAIL: llm-sourced facets remain"; exit 1; }   # → silent, exit 0: ADR-0090 re-sourced all seven to human (PIA-003)
# The two-valued DOMAIN STAYS, and nullability is declared BESIDE it, by name (AC-072 as amended
# 2026-09-12, DECISIONS-2026-09-12 E1: the three declared shapes are UNCHANGED and a fourth still
# fails load; `null` is never a domain member; and `values` is never deleted, because
# `spikes/p9/lint.py:49` reads `f["values"]` unguarded and raises KeyError without it).
# SEVEN booleans, not eight: `auth` was withdrawn above, and `surfaces` is a set, not a boolean.
# LEADING-ANCHORED on purpose: the unanchored form is movable by prose — this file's own registry
# comment quotes the literal `values = [true, false]`, which inflates an unanchored count to 8.
test "$(grep -c '^values = \[true, false\]' "$F")" = 7 || { echo "FAIL: the seven booleans do not all still declare the two-valued domain — AC-072 keeps the three shapes unchanged and a fourth fails load"; exit 1; }   # → silent, exit 0
test "$(grep -c '^nullable = true' "$F")" = 7 || { echo "FAIL: nullable = true is not declared on exactly 7 facets (4a.3)"; exit 1; }   # → silent, exit 0: every boolean that may abstain says so BY NAME
# Two equal counts are necessary and NOT sufficient — 4a.1 says this about derived/deriver and it is
# just as true here. This is the per-stanza PAIRING check: no boolean declares one without the other.
# Negative control, run 2026-09-12: with `ui`'s nullable stripped it prints 1, so it can fail.
n=$(awk '/^\[facet\./{if(b&&!u)x++; b=0;u=0} /^values = \[true, false\]$/{b=1} /^nullable = true$/{u=1} END{if(b&&!u)x++; print x+0}' "$F"); test "${n:-1}" -eq 0 || { echo "FAIL: ${n:-<unreadable>} boolean facet(s) declare values = [true, false] with NO nullable = true beside it"; exit 1; }   # → silent, exit 0
git add spikes/p9/facets.toml && git commit -m "Apply ADR-0089/0090/0093: deriver, kernel and declared sources, surfaces, tri-valued booleans, no llm facet" \
  || { echo "FAIL: nothing was committed"; exit 1; }
test -z "$(git status --porcelain spikes/p9/facets.toml)" || { echo "FAIL: facets.toml still dirty after commit"; exit 1; }   # → silent, exit 0: the registry edit is in history before Step 2 moves anything (PIA-018)
```

**The `surfaces.toml` line is gone from this checkpoint** (PIA-003): no such file exists in this tree, ADR-0089 makes
it a per-target-repository artefact, and S-FACET's own checklist is where its seeding is asserted. Asserting it here
was the "green anyway" shape Halt 8 names.

**Withdrawing `auth` breaks the P9 catalog lint, on purpose, and Step 7e repairs it. Measured 2026-09-12.**
`spikes/p9/lint.py` exits **0** before this step and **1** after it, with exactly **three** R9 errors —
`applicable_if: identifier 'auth' is not a facet, input, or builtin` at `catalog/security-requirements.toml:10`,
`catalog/security-review.toml:10` and `catalog/threat-model.toml:9`. **That is the correct state and it is not this
step's to fix:** `s7e` §7e.2 owns the row dispositions, and its rule for these is written down — *a `require` row on
`auth` becomes an `applicable_if` or is dropped, **never** a `require` on `surfaces`*, because `surfaces` is unscored
and may key no `require` row until S-FACET runs. **Four more rows are keyed on `auth` and the lint does not see them
at all** — `packs/bug-standard.toml:68`, `packs/initiative.toml:32` and `:42`, `packs/project.toml:35`, `:42`
and `:103` read `when = "auth"`, and `lint.py` does not check pack `when` predicates (`lattice.py` walks them).
Seven sites in total; do not read three as the whole set. Nothing between here and Step 7e asserts the P9 lint —
`s7b`:113's prose claims *"Checkpoint 7b runs it"* but **no command in Checkpoint 7b does**, so the red does not
block a stage; `s7e` §7e.4 is the only place it is asserted at `errors=0`, and that is after §7e.2's rewrite.

> **A trap, measured both ways, that will be waiting at Step 7e.** `spikes/p9/lint.py:339` accepts any bare
> identifier that matches a facet **value**, so declaring `auth` among `surfaces`'s `values` makes all three errors
> **vanish and the lint exit 0** — while the three predicates still read `applicable_if = "auth or …"` and now parse
> as a bare literal in a boolean position, which nothing checks. Measured 2026-09-12: with `auth` among the values,
> **0** errors; without it, **3**. So this step declares `surfaces` with a placeholder vocabulary that **omits
> `auth` and `ui`** — the two surface names the records name that are also facet names — precisely to keep the three
> errors visible, and says so in the registry comment. **When Step 7e authors the full vocabulary it must rewrite
> the predicates first, or it will silence them instead of fixing them.** This is golden constraint 9 from the
> too-narrow direction: a guard that admits the failure it exists to catch.

```bash
# [WILL EXIST] — Step 7b builds the lint; these are its acceptance shape. DEFERRED here, re-run as Checkpoint 7b's
# named lines. The expectation is "exit != 0 AND stderr names the failing clause" — a missing binary satisfies
# neither half and is not a pass (Halt 8).
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml   # exit != 0 AND stderr names the clause (AC-520)
cargo test -p rk-kernel -- facet_provenance_fixtures ; echo "exit=$?"  # → exit=0: four must-fail + ONE must-load
```

**What this step may not do, restated because every one of these is a route someone will reach for.**

- It may **not** say any facet "leaves the model". **None does** — no facet clears METHOD-P §1.4's 0.95 bar and
  the bar is unreachable at n=20 for any facet [measured, F39].
- It may **not** decide **source asymmetry**. `commits` beats `docs` on every facet and `union` is worse than
  `commits` alone on `toolchain` and `change` [measured, F39 §3]. ADR-0089 explicitly does not decide it; it is an
  arm of S-FACET.
- It may **not** touch **ADR-0079**, and by the time this step runs there is nothing left to touch: Step 4.4 wrote
  **ADR-0090** on 2026-09-12, which supersedes 0079 **in full** and moved it to `status: superseded`. This step
  consumes that record and may not re-open it; a reversal is a successor to 0090, never an edit. ADR-0089 forbids
  being read as deciding anything in 0079, and it did not decide it — F36's clause A did.
- It may **not** substitute unanimous N-seat model agreement for a human confirmation, **including as a later
  efficiency proposal**. Small-panel unanimity is a weaker correctness signal than its arithmetic implies and it
  fails in the correlated direction [documented, preprint, arXiv:2605.29800].
- It may **not** treat a `null` as a false negative in any scoring of a facet deriver.

**One decision this step met and did NOT take, because no record names the form (found 2026-09-12).** AC-520's last
clause is that *"a facet whose only computation is a lexical scan of free text, file paths and diff lines included,
does not satisfy PC8 rule 2 for `require`-row purposes whatever its record says"*, and its *Verified by* lists **"a
`deriver` declared lexical"** among six must-fail fixtures. **Nothing in any record says how a deriver is declared
lexical on disk** — not ADR-0089, not ADR-0093, not AC-072's key list. This is **structurally identical to E1**:
ADR-0089 made the booleans tri-valued and named no on-disk form, and DECISIONS-2026-09-12 E1 had to decide
`nullable = true`. So the same question is open one field over, and the same answer shape is available — a declared
key (`deriver_class = "lexical" | "structural"`), which is AC-072's own *"by name rather than by omission"* rule
applied to the `deriver` companion.

**Checkpoint 4a does not assert it, so this step completes without it** and does not choose a default (`00-preamble`:
*"Where a stage is tempted to decide something else, it stops"*). The six `deriver` names this step writes are
factual descriptions of each rule and are true under either resolution. **Step 7b cannot build
`lint --facet-provenance` without the form** and must not invent one: three of the six derived facets (`ui`,
`flag_exists`, `data_migration`) are lexical scans over paths and diff lines and are exactly what the clause blocks,
while `toolchain` and `change` are git metadata only, so the distinction is load-bearing and currently
undeclarable. **Owed to the operator before Step 7b**, recorded in `PROGRESS.md` §1 and §2.

---
