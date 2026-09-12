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

**Then tag the v0 pack** (ADR-0114, partial successor to ADR-0027; DECISIONS-2026-09-11 #26/#27): the tag rests on the two
conditions this step has just measured — P6 lint green at pack-default (the dead list reported by id, AC-102) and the F42
parse at 287/287 after 7e.2's two repairs — and on each activity carrying its `executed: { first, last }` ledger field
(`executed: never` on every activity but `root-cause`'s P7 members; a doctor row, never a lint failure). The twelve packs
are the accepted set (METHOD-P amendment 18); the eight-pack reading stays F35's headline and every twelve-pack figure
carries its pack count. The executed-once condition now gates extended→shipped *promotion*, not the tag.

The new unsatisfiable rate, its pack count (12) and its rigor mode (pack-default) are the baseline the SM register
records against SM-026/SM-027 and the AC register cites in AC-101/AC-102; the old figures stay in the record as
"pre-ADR-0089/0093".

**Checkpoint 7e.** All thirteen — recounted on disk 2026-09-11; "eleven" silently dropped the commit line and the
tag line, both of which carry `# →` expectations and both of which had defects:

```bash
cd D:/repos/redkiln-rs
test -f .redkiln/facets.toml && echo "registry derived" || { echo "FAIL: .redkiln/facets.toml absent — 7e.1 did not run (Halt 8)"; exit 1; }   # → the product registry exists
n=$(grep -c '^source = "llm"' .redkiln/facets.toml); echo "llm-sourced: ${n:-<registry unreadable>}"; test "$n" = 0 || { echo "FAIL: expected 0 llm-sourced facets, got '${n:-<unreadable>}' (ADR-0090)"; exit 1; }   # → 0
dv=$(awk '/^source = "derived"/{d++} /^deriver = /{v++} END{print d+0, v+0}' .redkiln/facets.toml); echo "$dv"; test "$dv" = "6 6" || { echo "FAIL: derived/deriver is '$dv', expected '6 6' (AC-520)"; exit 1; }   # → 6 6
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — Step 7d has not built it; Checkpoint 7e's three CLI lines cannot be run (Halt 8)"; exit 1; }
./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml || { rc=$?; echo "FAIL: lint --facet-stability exited $rc (AC-071)"; exit 1; }; echo "facet-stability exit=0"   # → exit=0 on the zero-llm branch (AC-071)
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml || { rc=$?; echo "FAIL: lint --facet-provenance exited $rc — 127 means the binary is absent, not a lint verdict (AC-520)"; exit 1; }; echo "facet-provenance exit=0"   # → exit=0: six derived, six records (AC-520)
./target/debug/redkiln doctor || { rc=$?; echo "FAIL: doctor exited $rc — 127 means the binary is absent, not a pack-load verdict (AC-071/AC-520/AC-072)"; exit 1; }; echo "doctor exit=0"   # → exit=0: every pack loads under AC-071/AC-520/AC-072
test "$(ls -1 .redkiln/DISPOSITIONS-*.md 2>/dev/null | wc -l)" -eq 1 || { echo "FAIL: not exactly one .redkiln/DISPOSITIONS-*.md"; exit 1; }; ls -1 .redkiln/DISPOSITIONS-*.md   # → 1, and its row count equals the three greps' sum
grep -rn 'stale_policy' .redkiln/packs/*.toml || { echo "FAIL: no stale_policy row in any pack — 7e.3 did not run"; exit 1; }
sp=$(grep -rh 'stale_policy' .redkiln/packs/*.toml | wc -l); rem=$(grep -rh 'stale_policy' .redkiln/packs/*.toml | grep -c 'rematerialise'); echo "stale_policy=$sp rematerialise=$rem"; test "$sp" -eq "$rem" || { echo "FAIL: $((sp-rem)) stale_policy rows are not rematerialise — decision 5 fixes every Phase-1 class (DECISIONS-2026-09-11 #4)"; exit 1; }   # → one per cost class per Phase-1 pack, each with its warrant comment
grep -rn 'exists(' .redkiln/packs .redkiln/catalog; rc=$?; test "$rc" -eq 1 || { echo "FAIL: rc=$rc — 0 means an exists() predicate remains (ADR-0033 admits no such user function, F42); 2 means .redkiln/packs or .redkiln/catalog is absent (Halt 8)"; exit 1; }; echo "OK: no exists( predicate"   # → 0 (F42's incident row rewritten)
out=$(python spikes/p9/grammar-probe.py); rc=$?; printf '%s\n' "$out" | grep -E '^predicates parsed|^on_false forms|FAIL '
test "$rc" -le 1 || { echo "FAIL: grammar-probe exited $rc — it did not run to completion"; exit 1; }
printf '%s\n' "$out" | grep -qE '^  (FAIL|ON_FALSE-FAIL) (packs|catalog)/' && { echo "FAIL: a defect remains in packs/ or catalog/ — 7e.2's rewrite did not land"; exit 1; }; echo "OK rc=$rc — nothing under packs/ or catalog/ failed; anything listed is catalog-b's, which 7e.1 does not copy"   # → rc 0 or 1, and NO FAIL naming packs/ or catalog/ (record the catalog-b defects that remain)
grep -rn '^executed = ' .redkiln/catalog/*.toml | grep -c never                       # → 102 minus root-cause's P7 members (ADR-0114: a doctor row, never a lint failure)
git add .redkiln spikes/p9 && git commit -m "Step 7e: pack authoring — .redkiln/ derived, rows dispositioned, stale_policy recorded per class, lattice re-baselined" || { echo "FAIL: git add/commit did not complete"; exit 1; }
n=$(git status --porcelain | grep -c '^ M'); echo "$n"; test "$n" -eq 0 || { echo "FAIL: $n tracked files still modified after the Step 7e commit"; exit 1; }   # → 0
git tag -a redkiln-pack-v0 -m "ADR-0114: P6 lint green at pack-default; 287/287 parsed" && git rev-parse -q --verify refs/tags/redkiln-pack-v0 || { echo "FAIL: redkiln-pack-v0 was not created"; exit 1; }   # → the tag's SHA on the Step 7e commit; its two conditions are the doctor and grammar-probe lines above (no CLI verb — ADR-0107 has none for it)
```

**Acceptance set (explicit):** **AC-080–AC-118** (packs and catalog lint — AC-119 unallocated), **AC-068** (PROVISIONAL,
mechanism: the pass condition is decision 5's rule), **AC-071** and **AC-520** on the branches this step makes true,
**AC-101/AC-102** on the re-run baseline; the `require`-row prohibitions on `declared` and NOT MEASURABLE facets
(ADR-0093). **Excluded:** every criterion S-FACET gates (a `require` row keyed on `ui`, `flag_exists`,
`data_migration` or `surfaces` is not authored in Phase 1).

**Spikes owed by this step:** **S-FACET** — its re-registered kill (DECISIONS-2026-09-11 #12; change-classification
§6) is what would let a `require` row key on a scored derived facet; unrun, and nothing here waits on it.
**R-MUT**/**P6-PACKS** rows in design §10.5 name the lattice as consumer; this step's re-run is their reading.
