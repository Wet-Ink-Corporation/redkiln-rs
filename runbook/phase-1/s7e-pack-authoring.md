# Step 7e — Pack authoring: `.redkiln/`, the registry the product reads, the row dispositions, and the lattice baseline (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7e` | Phase 1 | Checkpoint 7d | Checkpoint 7e | `s7d` | `s8` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** Steps 4a and 8 both cited a "Step 7b pack authoring" that no revision of this runbook ever
contained (PIA-011): nothing created `.redkiln/`, derived `.redkiln/facets.toml` from `spikes/p9/facets.toml`,
dispositioned the pack rows keyed on facets Step 4a re-sourced, re-ran the lattice after ADR-0089/0093, declared the
retry envelope in the pack, or recorded `stale_policy` — and every one of those was a `[WILL EXIST]` the audit found
attributed to no step. This step owns them. It takes **no decision**: `stale_policy` is decision 5 of the preamble's
table and is already recorded (DECISIONS-2026-09-11 #4); the row dispositions execute ADR-0090/0093; the lattice
re-run records a baseline it does not choose.

### 7e.1 `.redkiln/` and the registry the product reads

```bash
cd D:/repos/redkiln-rs
test -d .redkiln && echo "BEFORE: .redkiln exists — this step has partly run; read it" || echo "BEFORE: no .redkiln, as expected"
mkdir -p .redkiln/packs .redkiln/catalog .redkiln/processes
# The registry the product reads is DERIVED from the spike registry as Step 4a left it — not copied blind:
grep -c '^source = "llm"'      spikes/p9/facets.toml     # → 0  (Step 4a; ADR-0090). If not 0, STOP: Step 4a did not land.
grep -c '^source = "derived"'  spikes/p9/facets.toml     # → 6  (ADR-0093)
grep -c '^source = "kernel"'   spikes/p9/facets.toml     # → 4
grep -c '^source = "declared"' spikes/p9/facets.toml     # → 1
cp spikes/p9/facets.toml .redkiln/facets.toml
cp spikes/p9/artifacts.toml .redkiln/artifacts.toml
cp spikes/p9/catalog/*.toml .redkiln/catalog/            # 102 activities, bisect at version 3 with its ceiling (DECISIONS #1)
cp spikes/p9/packs/*.toml   .redkiln/packs/              # 12 packs; only bug-standard and story are Phase 1 (ADR-0011)
```

`catalog-b/` (the unshipped extended file, ADR-0027) is **not** copied; its three `on_false` defects and one bare
hyphenated value (F42) stay where they are until a Phase-2 pass. `spikes/p9/` itself is untouched — it is the spike
registry, it is cited by METHOD-P and the F-records by path, and Step 2 did not move it.

### 7e.2 Row dispositions — every row keyed on a facet whose source moved

Step 4a moved seven facets to `source = "human"` and withdrew `auth`. Every pack and catalog row that keys on one of
them has to be read once and given a written disposition (PIA-011); the counts below are the audit's at the pin
[measured, PIA-011] — **re-derive them** with the two greps and record the actual numbers in the checkpoint commit:

```bash
cd D:/repos/redkiln-rs/.redkiln
grep -rnE '(applicable_if|when) *=.*\b(criticality|novelty|size|blast_radius|has_repro|external_dep|market_facing)\b' packs catalog | wc -l
                                          # → the LLM-keyed rows (the audit counted 72 at the pin; F42 counted 116 predicate hits over the whole p9 tree)
grep -rnE '(applicable_if|when|tags) *=.*\bauth\b' packs catalog | wc -l   # → the auth-keyed rows (the audit counted 15)
grep -rnE '(applicable_if|when) *=.*\b(ui|flag_exists|data_migration)\b' packs catalog | wc -l   # → the scored-derived-keyed rows S-FACET gates
```

Write the disposition table into `.redkiln/DISPOSITIONS-<date>.md` with one line per row — file, row, facet, and
exactly one of:

| disposition | when | rule |
| --- | --- | --- |
| **keep** | the facet is now `human` and the row's semantics survive a triage-supplied value | a `require` row may key on a `human` facet freely (ADR-0079's honest destination; golden constraint 1 forbids LLM-*sourced* axes, not human ones) |
| **rewrite to `surfaces`** | the row keyed on the withdrawn `auth` boolean | `surfaces` may be **authored** in an `applicable_if` (`any(s in surfaces, s == "auth")`) but **may key no `require` row** until S-FACET scores a declared boundary map (ADR-0089 decision 2; ADR-0093 decision 1) — a `require` row on `auth` becomes an `applicable_if` or is dropped, never a `require` on `surfaces` |
| **null-guarded** | the row keys on `ui`, `flag_exists` or `data_migration` — tri-valued now | the predicate handles `null` explicitly under ADR-0103 clause 4 (`x != null and …`), or the row is a lint error at load; `data_migration` and `recent_deploy` are NOT MEASURABLE and **may key no `require` row** (ADR-0093 decision 2) |
| **drop** | the row's meaning does not survive the source move | say why; the activity stays in the catalog |

**Two rows are lint errors the moment the pack loads and are fixed here, not later (F42):**
`packs/incident.toml` `stage.closeout.spawn[0].when = 'exists(ProblemRecord)'` — a user function ADR-0033 admits none
of; rewrite as `${ProblemRecord?} != null`. (`catalog-b`'s three `on_false` bare strings and its hyphenated value are
out of scope — not copied.)

### 7e.3 The retry envelope and `stale_policy` are pack values with a warrant, never code defaults

- **Backoff** — base **2 s**, cap **5 min** — is **kernel policy stated in AC-029**, not a pack-declared value
  (PIA-055); the pack declares the **retry envelope** (attempts per class) and cites AC-029 for the shape. A code
  default for either is refused (design §3.6/§3.10 as amended).
- **`stale_policy`** is written per cost class under `policy.<cost_class>.stale_policy` with enum
  `rematerialise | gate` (design §4.4 step 3; PIA-044), and **its Phase-1 value is `rematerialise` with the COARSE
  `derived_from` scope for every class** (decision 5; DECISIONS-2026-09-11 #4). The warrant goes in the pack beside
  the value, verbatim: *F40 §3 — coarse marks 5 Stale vs fine 15 on the 33 rows both scopes scored; 10 rows flip
  Present→Stale coarse-to-fine, none the other way; criterion 1's own remedy is contraindicated (Halt 12).* Fine
  scope stays available and is never mandatory. AC-068's pass condition is this decision rule, and the record it
  reads is written by `score-p2.py --out` (PIA-059).

### 7e.4 The lattice, re-run, is the AC-101 / AC-102 / SM-026 / SM-027 baseline

The lattice figures the design and the SM register carried (0.3% and 6/102 unsatisfiable, the 23.5 → 11.4 → 0.3%
and 20.8 → 14.9 → 5.9% series) **predate ADR-0089 and ADR-0093** and were baselined once on a buggy lint (PIA-011,
PIA-076). Re-run, on the registry as this step leaves it, in the **pack-default** rigor mode (DECISIONS-2026-09-11
#20: overrides are enumerated and reported, never thresholded), and record **the pack count and the rigor mode
beside every rate** (SM-027):

```bash
cd D:/repos/redkiln-rs/spikes/p9
PYTHONIOENCODING=utf-8 python lattice.py --validate --out results-p6-run10-post-adr-0093.txt   # → "validate: no errors"; the localisation family shows holes=0 (P7-CEIL)
PYTHONIOENCODING=utf-8 python lint.py | tail -3                                                 # → errors=0
PYTHONIOENCODING=utf-8 python grammar-probe.py | grep -E '^predicates parsed|^on_false forms'  # → 287/287 once 7e.2's rewrite lands; all on_false in the closed set
```

The new unsatisfiable rate, its pack count (12) and its rigor mode (pack-default) are the baseline the SM register
records against SM-026/SM-027 and the AC register cites in AC-101/AC-102; the old figures stay in the record as
"pre-ADR-0089/0093".

**Checkpoint 7e.** All nine:

```bash
cd D:/repos/redkiln-rs
test -f .redkiln/facets.toml && echo "registry derived"                             # → the product registry exists
grep -c '^source = "llm"' .redkiln/facets.toml                                      # → 0
awk '/^source = "derived"/{d++} /^deriver = /{v++} END{print d, v}' .redkiln/facets.toml   # → 6 6
./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml ; echo "exit=$?" # → exit=0 on the zero-llm branch (AC-071)
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml ; echo "exit=$?" # → exit=0: six derived, six records (AC-520)
./target/debug/redkiln doctor ; echo "exit=$?"                                        # → exit=0: every pack loads under AC-071/AC-520/AC-072
ls .redkiln/DISPOSITIONS-*.md | wc -l                                                # → 1, and its row count equals the three greps' sum
grep -rn 'stale_policy' .redkiln/packs/*.toml | grep -c 'rematerialise'              # → one per cost class per Phase-1 pack, each with its warrant comment
grep -rn 'exists(' .redkiln/packs .redkiln/catalog | wc -l                           # → 0 (F42's incident row rewritten)
python spikes/p9/grammar-probe.py >/dev/null ; echo "exit=$?"                        # → exit=0 over spikes/p9 once the incident row is fixed there too (or 1 with only catalog-b's four defects listed — record which)
git add .redkiln spikes/p9 && git commit -m "Step 7e: pack authoring — .redkiln/ derived, rows dispositioned, stale_policy recorded per class, lattice re-baselined" && git status --porcelain | grep -c '^ M'   # → 0
```

**Acceptance set (explicit):** **AC-080–AC-118** (packs and catalog lint — AC-119 unallocated), **AC-068** (PROVISIONAL,
mechanism: the pass condition is decision 5's rule), **AC-071** and **AC-520** on the branches this step makes true,
**AC-101/AC-102** on the re-run baseline; the `require`-row prohibitions on `declared` and NOT MEASURABLE facets
(ADR-0093). **Excluded:** every criterion S-FACET gates (a `require` row keyed on `ui`, `flag_exists`,
`data_migration` or `surfaces` is not authored in Phase 1).

**Spikes owed by this step:** **S-FACET** — its re-registered kill (DECISIONS-2026-09-11 #12; change-classification
§6) is what would let a `require` row key on a scored derived facet; unrun, and nothing here waits on it.
**R-MUT**/**P6-PACKS** rows in design §10.5 name the lattice as consumer; this step's re-run is their reading.
