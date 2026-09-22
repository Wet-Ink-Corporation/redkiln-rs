# Step 7e — Pack authoring: `.redkiln/`, the registry the product reads, the row dispositions, and the lattice baseline (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7e` | Phase 1 | Checkpoint 7d | Checkpoint 7e | `s7d` | `s7f` |

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
# `init` SCAFFOLDS config.toml and this step fills it (E6) — but no stage ever RAN it on this
# repository, so the `test -f .redkiln/config.toml` below fails unless Step 7e runs it first.
# Measured 2026-09-21: `.redkiln/` was absent entirely at `f9b07b5`. [amended 2026-09-21]
./target/debug/redkiln init
mkdir -p .redkiln/packs .redkiln/catalog .redkiln/processes
# The registry the product reads is DERIVED from the spike registry as Step 4a left it — not copied blind:
grep -c '^source = "llm"'      spikes/p9/facets.toml     # → 0  (Step 4a; ADR-0090). If not 0, STOP: Step 4a did not land.
grep -c '^source = "derived"'  spikes/p9/facets.toml     # → 6  (ADR-0093)
grep -c '^source = "kernel"'   spikes/p9/facets.toml     # → 4
grep -c '^source = "declared"' spikes/p9/facets.toml     # → 1
cp spikes/p9/facets.toml .redkiln/facets.toml
cp spikes/p9/artifacts.toml .redkiln/artifacts.toml
cp spikes/p9/catalog/*.toml .redkiln/catalog/            # 102 activities, bisect at version 3 with its ceiling (DECISIONS #1)
# EXACTLY TWO packs are installed (DECISIONS-2026-09-12 E7). ADR-0011's Decision is one sentence — "Phase 1 ships
# exactly two process packs: bug-standard and story" — and `doctor` loads EVERY pack in .redkiln/packs/ under
# AC-071/AC-520/AC-072, so copying all twelve would put ten unreviewed Phase-2 packs in front of Checkpoint 7e's
# `doctor` exit 0 and every later doctor run, for no Phase-1 benefit. The other ten stay in spikes/p9/.
for p in bug-standard story; do
  test -f "spikes/p9/packs/$p.toml" || { echo "FAIL: spikes/p9/packs/$p.toml absent — the Phase-1 pack set is incomplete (ADR-0011)"; exit 1; }
  cp "spikes/p9/packs/$p.toml" .redkiln/packs/
done
n=$(ls -1 .redkiln/packs/*.toml | wc -l); test "${n:-0}" -eq 2 \
  || { echo "FAIL: ${n:-0} packs installed, expected exactly 2 (ADR-0011)"; exit 1; }
# The enabled-pack list (DECISIONS-2026-09-12 E6): `init` scaffolded .redkiln/config.toml at Step 7d with an empty list;
# this step fills it. Design §4.3 rule 3 reads cross-pack order off exactly this list.
test -f .redkiln/config.toml || { echo "FAIL: .redkiln/config.toml absent — Step 7d's `init` did not run, or did not scaffold it (E6)"; exit 1; }
python - "$PWD/.redkiln/config.toml" <<'PY'
import re, sys
p = sys.argv[1]; t = open(p, encoding='utf-8').read()
assert '[packs]' in t, 'config.toml carries no [packs] table — init did not scaffold it'
t = re.sub(r'(?m)^enabled\s*=.*$', 'enabled = ["bug-standard", "story"]', t, count=1)
open(p, 'w', encoding='utf-8', newline='').write(t)
print('[packs] enabled = ["bug-standard", "story"]')
PY
```

`catalog-b/` (the unshipped extended file, ADR-0027) is **not** copied; its three `on_false` defects and one bare
hyphenated value (F42) stay where they are until a Phase-2 pass. `spikes/p9/` is **not relocated** — it is the spike registry, it is cited by METHOD-P and the
F-records **by path**, and Step 2 did not move it. That warrant is about *paths*, and Step 7e makes two in-place
edits inside it, neither of which disturbs a path citation [amended 2026-09-21, operator decision]:
**(a)** §7e.2's F42 predicate repair at `packs/incident.toml`:56 — the sentence read *"`spikes/p9/` itself is
untouched"* until now, and it could not both hold and be squared with §7e.4's grammar-probe expectation (the probe
walks `spikes/p9/`, not `.redkiln/`) or with Checkpoint 7e's `git add .redkiln spikes/p9`, which expects that tree to
have changed; and **(b)** the fifteen rows still keying on the `auth` boolean Step 4a withdrew — twelve `require`
rows across five packs and three catalog `applicable_if`s. Without (b) **§7e.4 cannot run at all**: `lattice.py`
resolves every predicate identifier against the registry and has died `ValueError: identifier 'auth' is not a facet
or facet value` on every invocation since Step 4a, which is precisely why SM-026's baseline still predated
ADR-0089/0093.

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

**ONE row is a lint error the moment the pack loads and is fixed here, not later (F42):**
`packs/incident.toml` `stage.closeout.spawn[0].when = 'exists(ProblemRecord)'` — a user function ADR-0033 admits none
of; rewrite as `${ProblemRecord?} != null`. (`catalog-b`'s three `on_false` bare strings and its hyphenated value are
out of scope — not copied.) **This paragraph said "Two rows" and then named one** [corrected 2026-09-21]: the second
is `catalog-b/blameless-postmortem.toml`'s hyphenated `-vuln` lex error, which the same sentence puts out of scope.
F42's two parse failures are real; only one of them is in this step's scope.

**A note on Checkpoint 7e line 146.** Its `grep -rn 'exists(' .redkiln/packs .redkiln/catalog` **cannot fail**:
`incident.toml` is not one of the two packs §7e.1 installs, so no `exists(` predicate ever reaches `.redkiln/`. The
instrument that actually detects the repair is line 149's grammar-probe, which walks `spikes/p9/`. Line 146 is a
regression guard on the installed tree, not evidence that F42 was fixed [measured 2026-09-21].

### 7e.3 The retry envelope and `stale_policy` are pack values with a warrant, never code defaults

- **Backoff** — base **2 s**, cap **5 min** — is **kernel policy stated in AC-029**, not a pack-declared value
  (PIA-055); the pack declares the **retry envelope** (attempts per class) and cites AC-029 for the shape. A code
  default for either is refused (design §3.6/§3.10 as amended).
- **`stale_policy`** is written per cost class under `policy.<cost_class>.stale_policy` with enum
  `rematerialise | gate` (design §4.4 step 3; PIA-044), and **its Phase-1 value is `rematerialise` with the COARSE
  `derived_from` scope for every class** (decision 5; DECISIONS-2026-09-11 #4). The warrant goes in the pack beside
  the value: *F40 §3–§4 — coarse marks 5 Stale vs fine 15 on the 33 rows both scopes scored; 10 rows flip
  Present→Stale coarse-to-fine, none the other way [§3]; criterion 1's own remedy is contraindicated [§4] (Halt
  12).* **The attribution read "verbatim: F40 §3" and is corrected to §3–§4** [2026-09-21]: §3 carries the two
  numeric clauses (`spikes/F40-DRAFT.md`:40-47); the contraindication is §4's own sentence (`:64`). Fine
  scope stays available and is never mandatory. **The accepted record that holds this warrant is ADR-0117** and this
  section did not cite it — clause 1 *"`stale_policy` stays `rematerialise`, at coarse scope, for every cost class …
  recorded per class with its warrant in the pack at Step 7e"*, clause 2 *"METHOD-P §2.4 criterion 1's prescribed
  remedy is NOT applied"*. Under ADR-0092 an accepted ADR outranks this runbook, so the pack comment cites ADR-0117.
  (§7e.2's `keep` rule likewise cites **ADR-0079**, which is `status: superseded`; **ADR-0090** governs.)
  AC-068's pass condition is this decision rule, and the record it
  reads is written by `score-p2.py --out` (PIA-059) — **which does not exist**: `spikes/p2/score-p2.py` takes only
  `--engine` and `--labels`, and `spikes/results/p2/shortcircuit.json` is written by nothing [measured 2026-09-21].
  The register also names an instrument `redkiln lint --stale-policy` that the Step 7d CLI does not carry; Checkpoint
  7e greps instead, and that substitution is stated here rather than left for the grep to paper over.

### 7e.4 The lattice, re-run, is the AC-101 / AC-102 / SM-026 / SM-027 baseline

The lattice figures the design and the SM register carried (0.3% and 6/102 unsatisfiable, the 23.5 → 11.4 → 0.3%
and 20.8 → 14.9 → 5.9% series) **predate ADR-0089 and ADR-0093** and were baselined once on a buggy lint (PIA-011,
PIA-076). Re-run, on the registry as this step leaves it, in the **pack-default** rigor mode (DECISIONS-2026-09-11
#20: overrides are enumerated and reported, never thresholded), and record **the pack count and the rigor mode
beside every rate** (SM-027):

**TWO lattice runs, not one** [corrected 2026-09-21]. `--validate` returns through `sys.exit` at
`lattice.py`:322, which is *before* `open(args.out)` at `:341` — so `--validate --out <file>` writes **nothing**, and
the unsatisfiable rate, the dead list and the per-pack counts all live in the non-validate path. A single command
cannot produce the baseline this section exists to record. (`--packs` is a **filter by pack id**, not a directory: a
path passed there matches no pack, and the run then reports `packs=[]`, `unsat=0/0` and `dead=102/102` at exit 0 — a
vacuous green that must never be recorded as a reading.)

```bash
cd D:/repos/redkiln-rs/spikes/p9
PYTHONIOENCODING=utf-8 python lattice.py --validate                                             # → "validate: no errors"; the localisation family shows holes=0 (P7-CEIL)
PYTHONIOENCODING=utf-8 python lattice.py --out results-p6-run10-post-adr-0093.txt               # → the baseline: TOTAL … pack-default; time it, SM-025 owes a stamped figure
PYTHONIOENCODING=utf-8 python lint.py | grep -E '^errors='                                      # → errors=0 (it is line 7, NOT in `tail -3`)
PYTHONIOENCODING=utf-8 python grammar-probe.py | grep -E '^predicates parsed|^on_false forms'  # → 278/279; on_false 96/99 — see the ceiling note below
```

**The `287/287` this section carried is unreachable, and so is "all on_false in the closed set"** [measured
2026-09-21]. F42's two parse failures are `packs/incident.toml`'s (in scope) and
`catalog-b/blameless-postmortem.toml`'s hyphenated `-vuln` lex error (**out** of scope, and not copied), so one
in-scope repair could only ever reach **286/287**. The denominator then moved again: §7e.2's disposition of the
withdrawn-`auth` rows **drops eight `require` rows** from the spike packs, and each carried a `when` predicate, so the
count settles at **278/279** — the one remaining failure being `catalog-b`'s. All three `on_false` defects are
`catalog-b`'s too, so **96/99 is the ceiling** there. Checkpoint 7e's *assertion* is correct as written and needs no
change: it greps for a `FAIL` naming `packs/` or `catalog/`, and `catalog-b/` matches neither.

**Then tag the v0 pack** (ADR-0114, partial successor to ADR-0027; DECISIONS-2026-09-11 #26/#27): the tag rests on the two
conditions this step has just measured — P6 lint green at pack-default (the dead list reported by id, AC-102) and the F42
parse at **278/279** after 7e.2's **one** in-scope repair — and on each activity carrying its `executed: { first, last }`
ledger field (a doctor row, never a lint failure). **At v0 that field is ABSENT and every one of the 102 reads
`executed: never`** [measured 2026-09-21]: DECISIONS-2026-09-12 E8 makes the ledger kernel-written on
`activity_committed`, the checkpoint asserts that no catalog file carries a hand-seeded `executed`, and nothing has
executed — so the parenthetical *"on every activity but `root-cause`'s P7 members"* cannot hold here and is corrected. The twelve packs
are the accepted set (METHOD-P amendment 18); the eight-pack reading stays F35's headline and every twelve-pack figure
carries its pack count. The executed-once condition now gates extended→shipped *promotion*, not the tag.

The new unsatisfiable rate, its pack count and its rigor mode (pack-default) are the baseline the SM register
records against SM-026/SM-027 and the AC register cites in AC-101/AC-102; the old figures stay in the record as
"pre-ADR-0089/0093". **TWO pack numbers are recorded, never one** [corrected 2026-09-21]: DECISIONS-2026-09-12 E7 says
so in terms — the **measured** count (**12**, the `spikes/p9` tree the lattice runs over) *and* the **installed**
count (**2**, the Phase-1 pack set). This paragraph named only the 12. **SM-025 is also owed a stamped timing by this
re-run** and this section did not name it: the SM register assigns it here — *"A stamped timing is taken on the next
lattice run — the Step 7e re-run under ADR-0093 decision 5"* — so the run is timed and SM-025 amended with the rest.

**Checkpoint 7e.** All thirteen — recounted on disk 2026-09-11; "eleven" silently dropped the commit line and the
tag line, both of which carry `# →` expectations and both of which had defects:

```bash
cd D:/repos/redkiln-rs
test -f .redkiln/facets.toml && echo "registry derived" || { echo "FAIL: .redkiln/facets.toml absent — 7e.1 did not run (Halt 8)"; exit 1; }   # → the product registry exists
n=$(grep -c '^source = "llm"' .redkiln/facets.toml); echo "llm-sourced: ${n:-<registry unreadable>}"; test "$n" = 0 || { echo "FAIL: expected 0 llm-sourced facets, got '${n:-<unreadable>}' (ADR-0090)"; exit 1; }   # → 0
dv=$(awk '/^source = "derived"/{d++} /^deriver = /{v++} END{print d+0, v+0}' .redkiln/facets.toml); echo "$dv"; test "$dv" = "6 6" || { echo "FAIL: derived/deriver is '$dv', expected '6 6' (AC-520)"; exit 1; }   # → 6 6
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — Step 7d has not built it; Checkpoint 7e's three CLI lines cannot be run (Halt 8)"; exit 1; }
./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml || { rc=$?; echo "FAIL: lint --facet-stability exited $rc (AC-071)"; exit 1; }; echo "facet-stability exit=0"   # → exit=0 on the zero-llm branch (AC-071)
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml --pinned-corpus .redkiln/corpus-records.toml || { rc=$?; echo "FAIL: lint --facet-provenance exited $rc — 127 means the binary is absent, not a lint verdict (AC-520)"; exit 1; }; echo "facet-provenance exit=0"   # → exit=0: "derived facets: 6; records read: 6" (AC-520), plus the two NOT MEASURABLE report rows
./target/debug/redkiln doctor || { rc=$?; echo "FAIL: doctor exited $rc — 127 means the binary is absent, not a pack-load verdict (AC-071/AC-520/AC-072)"; exit 1; }; echo "doctor exit=0"   # → exit=0: every pack loads under AC-071/AC-520/AC-072
test "$(ls -1 .redkiln/DISPOSITIONS-*.md 2>/dev/null | wc -l)" -eq 1 || { echo "FAIL: not exactly one .redkiln/DISPOSITIONS-*.md"; exit 1; }; ls -1 .redkiln/DISPOSITIONS-*.md   # → 1, and its row count equals the three greps' sum
grep -rn 'stale_policy' .redkiln/packs/*.toml || { echo "FAIL: no stale_policy row in any pack — 7e.3 did not run"; exit 1; }
sp=$(grep -rh 'stale_policy' .redkiln/packs/*.toml | wc -l); rem=$(grep -rh 'stale_policy' .redkiln/packs/*.toml | grep -c 'rematerialise'); echo "stale_policy=$sp rematerialise=$rem"; test "$sp" -eq "$rem" || { echo "FAIL: $((sp-rem)) stale_policy rows are not rematerialise — decision 5 fixes every Phase-1 class (DECISIONS-2026-09-11 #4)"; exit 1; }   # → one per cost class per Phase-1 pack, each with its warrant comment
grep -rn 'exists(' .redkiln/packs .redkiln/catalog; rc=$?; test "$rc" -eq 1 || { echo "FAIL: rc=$rc — 0 means an exists() predicate remains (ADR-0033 admits no such user function, F42); 2 means .redkiln/packs or .redkiln/catalog is absent (Halt 8)"; exit 1; }; echo "OK: no exists( predicate"   # → 0 (F42's incident row rewritten)
out=$(python spikes/p9/grammar-probe.py); rc=$?; printf '%s\n' "$out" | grep -E '^predicates parsed|^on_false forms|FAIL '
test "$rc" -le 1 || { echo "FAIL: grammar-probe exited $rc — it did not run to completion"; exit 1; }
printf '%s\n' "$out" | grep -qE '^  (FAIL|ON_FALSE-FAIL) (packs|catalog)/' && { echo "FAIL: a defect remains in packs/ or catalog/ — 7e.2's rewrite did not land"; exit 1; }; echo "OK rc=$rc — nothing under packs/ or catalog/ failed; anything listed is catalog-b's, which 7e.1 does not copy"   # → rc 0 or 1, and NO FAIL naming packs/ or catalog/ (record the catalog-b defects that remain)
# ADR-0114 decision 2's `executed` ledger is KERNEL-written, on `activity_committed` — so at v0 it is ABSENT
# from the catalog and 7e authors none of it (DECISIONS-2026-09-12 E8). "Reads executed: never" is doctor's
# RENDERING of first = None, which is why this asserts doctor's count and not a grep for a row that should not
# exist yet. A doctor row, never a lint failure (ADR-0114).
n=$(grep -rlE '^\s*(executed\s*=|\[[^]]*\.executed\])' .redkiln/catalog/*.toml | wc -l)
test "${n:-0}" -eq 0 || { echo "FAIL: ${n} catalog file(s) carry a hand-seeded executed field — ADR-0114 assigns it to the kernel"; exit 1; }
out=$(./target/debug/redkiln doctor 2>&1) || { echo "FAIL: doctor exited non-zero"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qiE 'never[- ]executed' \
  || { echo "FAIL: doctor prints no never-executed line — ADR-0114 decision 2's ledger is not reported"; exit 1; }
printf '%s\n' "$out" | grep -iE 'never[- ]executed'   # → ALL 102: at v0 the ledger is absent (E8), so nothing has a first/last. One line per enabled pack.
git add .redkiln spikes/p9 && git commit -m "Step 7e: pack authoring — .redkiln/ derived, rows dispositioned, stale_policy recorded per class, lattice re-baselined" || { echo "FAIL: git add/commit did not complete"; exit 1; }
n=$(git status --porcelain | grep -c '^ M'); echo "$n"; test "$n" -eq 0 || { echo "FAIL: $n tracked files still modified after the Step 7e commit"; exit 1; }   # → 0
git tag -a redkiln-pack-v0 -m "ADR-0114: P6 lint green at pack-default; 278/279 parsed, the one remaining failure catalog-b's and out of scope" && git rev-parse -q --verify refs/tags/redkiln-pack-v0 || { echo "FAIL: redkiln-pack-v0 was not created"; exit 1; }   # → the tag's SHA on the Step 7e commit; its two conditions are the doctor and grammar-probe lines above (no CLI verb — ADR-0107 has none for it)
```

**Acceptance set (explicit):** **AC-080–AC-118 plus AC-527 and AC-532** (packs and catalog lint — AC-119 unallocated,
never reused). **The range alone was incomplete** [corrected 2026-09-21]: the register's own §4 area header reads
*"AC-080..AC-118 **plus AC-527 and AC-532** … — **41 in force**"*, and `00-preamble.md` requires each stage to list its
criteria as a range **minus the ids it excludes, naming each**. **AC-532** is ADR-0114 decision 2's executed-once
ledger on the *resolved, in-memory* catalog record (D15-3/D15-4) — which is exactly what this step's new
`RK-D-EXECUTED-LEDGER` doctor row reports, so it is claimed, not excluded. **AC-527** (the forbidden empty-set
quality-block terminal predicate, ADR-0080; PIA-029) is in the area and is **not** exercised by this step: it is
**excluded**, by name and with that reason. **AC-068** (PROVISIONAL,
mechanism: the pass condition is decision 5's rule), **AC-071** and **AC-520** on the branches this step makes true,
**AC-101/AC-102** on the re-run baseline; the `require`-row prohibitions on `declared` and NOT MEASURABLE facets
(ADR-0093). **Excluded:** every criterion S-FACET gates (a `require` row keyed on `ui`, `flag_exists`,
`data_migration` or `surfaces` is not authored in Phase 1).

**Spikes owed by this step:** **S-FACET** — its re-registered kill (DECISIONS-2026-09-11 #12; change-classification
§6) is what would let a `require` row key on a scored derived facet; unrun, and nothing here waits on it.
**R-MUT**/**P6-PACKS** rows in design §10.5 name the lattice as consumer; this step's re-run is their reading.
