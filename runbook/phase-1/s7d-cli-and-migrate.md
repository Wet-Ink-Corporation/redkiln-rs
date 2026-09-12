# Step 7d — `rk-cli`, and `redkiln-migrate` v0 (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7d` | Phase 1 | Checkpoint 7c | Checkpoint 7d | `s7c` | `s7e` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


`rk-cli` is the **composition root**: a `clap` router, one idempotent `run` entry (ADR-0107 — no `--resume`, no
`resume`, a committed argument-surface snapshot and a name denylist), plus `answer`, `status`, `explain`, `doctor`
and an idempotent `init`. **`init` scaffolds `.redkiln/config.toml`** (DECISIONS-2026-09-12 E6) — the file design §3's port
rule P3 and design §4.3 rule 3 both read and that no stage created before 2026-09-12: it carries `[adapters]`,
`[packs] enabled` (empty here; Step 7e fills it) and `doctor.unread_window_runs`. A repository on which only
`init` has run must be loadable, because AC-485/AC-486 resolve the adapter set **at startup** from that file.
AC-437's idempotence holds because one verb owns creation: a second `init` leaves the tree identical.
The adapter set — all four reals (ADR-0094) — is resolved **once at startup** and is a
run-scoped constant (AC-485); an adapter that is not compiled in or is misconfigured **refuses the run before any
spawn or store write** (AC-486). The process reads its environment in exactly one place — here, while building
`Config` — and configuration reaches the kernel **as a value**; the `CancelToken` is constructed here and wired to
the signal path (ADR-0099). `anyhow` is permitted in this crate and in `redkiln-migrate` only (ADR-0102). The
`rk-doctor` label on register rows is a subject tag for this crate's `doctor` module (design §9.1 is the crate
authority).

`answer` is the **only** gate channel in Phase 1 (ADR-0009); tracker grammar and web view are Phase 3. `run` never
waits: a human gate is a `(token, tree_hash, deadline)` record, a `Gate:` trailer and a **process that exits 0**.
`doctor` is one rule table, all configuration-time, **zero model cost**.

One CLI behaviour has a measured platform cause and must be handled explicitly: **a platform error can return exit
0 with `subtype: success`** (F20). Exit codes are AC-400–AC-439's subject, and
`references/2026-09-08-acceptance-criteria.md` **§12.10** ("Exit codes, and a platform error that returns success",
line 5132) is named for exactly this. Note that section number belongs to the **acceptance-criteria register**, not
to the architecture design — the design has no §12, and the previous revision's bare "§12.10" sent the reader to
the wrong document.

`redkiln-migrate` v0 ships in Phase 1: one commit, one differential report. It is the **only** place an emitter of
an old redkiln file format may exist (AC-474), and **no configuration key, flag or environment variable enables a
dual-write or compatibility-write mode**. Its YAML reader is **`yaml_serde = "=0.10.7"`** and it is the only crate that
may depend on a YAML crate (ADR-0100). **"v0" is defined (PIA-002):** the binary exists and is invoked by this
checkpoint; the structural assertions AC-441/AC-466/AC-474 hold; the report conforms to AC-448/AC-468's schema; and a
**dry run over a small fixture** completes. It **refuses a target with non-empty `git status --porcelain` before any
write and writes into a dedicated branch or worktree**, so an interrupted run leaves the target branch untouched
(AC-522); AC-450's idempotence test includes a kill between the write and the commit. The nine **corpus-bound**
criteria (AC-440, 444, 451, 452, 459, 460, 461, 469, 479) run in the local-only `corpus` suite (s5 §5.2) and are the
explicit **Phase-2 acceptance set for the full migration**; Phase 1's v0 claims the structural and dry-run rows only.
The companion kinds it lifts as typed sidecars are the six of DECISIONS-2026-09-11 #3; the other eight census kinds
lift as opaque prose (ADR-0075 clause 6). It never populates `gwt` or `verification` (AC-137; the 153-spec byte-identical
round-trip of AC-454 is the check).

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- `refuses_dirty_target` — red against a target with non-empty `git status --porcelain`: `redkiln-migrate` must
  refuse **before any write** (AC-522). A test that checks the refusal only after a partial write pins nothing.
- `idempotent_across_kill` — AC-450's kill case: red by killing **between the write and the commit**, then
  re-running. This is the one Halt 4 exists for; a digest that differs across the boundary is a halt, not a retry.
- `resume_is_refused` — red by adding `--resume` to the argument surface: ADR-0107's denylist means the flag must
  not **exist**, and the test asserts the unknown-argument refusal, not merely a non-zero exit.
- `doctor_window_n_fixtures` — the paired **N−1 / N+1** threshold fixtures AC-472/AC-426 name. They cannot be
  authored until `N` is declared (**DECISIONS-2026-09-12 E9**, owed); until then this criterion has no truth value and
  SM-139's target cannot be missed. Record that, do not grep for an identifier and call it covered.

**Checkpoint 7d:**

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root absent"; exit 1; }   # no `set -e`/`pipefail` in this block — each line below carries its own guard
test -f Cargo.toml || { echo "FAIL: workspace Cargo.toml absent — Checkpoint 7d unrunnable (Halt 8)"; exit 1; }   # cargo searches ANCESTOR dirs
cargo build --workspace || { echo "FAIL: cargo build --workspace failed — nothing below is meaningful (golden constraint 3)"; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — Checkpoint 7d unrunnable (Halt 8)"; exit 1; }
out=$(./target/debug/redkiln doctor); rc=$?   # zero model cost is READ off the run; no line can assert it
[ "$rc" -eq 0 ] && test -n "$out" || { echo "FAIL: doctor exited $rc, or printed no rule table"; exit 1; }
./target/debug/redkiln run --help || { echo "FAIL: no idempotent run entry (ADR-0107)"; exit 1; }   # the idempotent entry exists
out=$(./target/debug/redkiln run --resume 2>&1); rc=$?   # AC-400/AC-264: the flag does not exist (ADR-0107)
[ "$rc" -ne 0 ] || { echo "FAIL: run --resume was ACCEPTED (exit 0) — ADR-0107's denylist is breached"; exit 1; }
printf '%s\n' "$out" | grep -qiE 'unexpected|unrecognized|error' && printf '%s\n' "$out" | grep -q -- '--resume' || { echo "FAIL: run --resume exited $rc but not as an unknown-argument refusal"; printf '%s\n' "$out" | tail -5; exit 1; }
echo "resume is refused"
./target/debug/redkiln answer --help || { echo "FAIL: answer is absent — the only Phase-1 gate channel (ADR-0009)"; exit 1; }   # the only Phase-1 gate channel
./target/debug/redkiln init && ./target/debug/redkiln init || { echo "FAIL: init is not idempotent — a run exited non-zero (AC-437)"; exit 1; }   # → both runs exit 0
out=$(cargo test --workspace 2>&1); rc=$?   # trycmd/assert_cmd CLI-surface tests; the snapshot clause has no command of its own
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }
./target/debug/redkiln-migrate --version || { echo "FAIL: redkiln-migrate absent or --version failed — v0 does not exist (PIA-002)"; exit 1; }   # → the binary EXISTS
FIXTURE=   # set to a real small fixture path — a bare <…> is not prose to bash: `<a` redirects input and the trailing `>` TRUNCATES a file
test -n "$FIXTURE" && test -x ./target/debug/redkiln-migrate && test -e "$FIXTURE" || { echo "FAIL: FIXTURE unset, or redkiln-migrate / the fixture absent — the v0 dry run is unassertable (Halt 8)"; exit 1; }
out=$(./target/debug/redkiln-migrate --dry-run --target "$FIXTURE"); rc=$?
[ "$rc" -eq 0 ] && test -n "$out" || { echo "FAIL: dry run exited $rc, or printed no report (AC-448/AC-468 schema conformance is still read by eye)"; exit 1; }
out=$(cargo test -p redkiln-migrate -- refuses_dirty_target 2>&1); rc=$?   # AC-522
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p redkiln-migrate exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: refuses_dirty_target matched 0 tests — AC-522 unasserted"; exit 1; }
out=$(cargo test -p redkiln-migrate -- idempotent_across_kill 2>&1); rc=$?   # AC-450 extended
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p redkiln-migrate exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: idempotent_across_kill matched 0 tests — AC-450's kill case unasserted"; exit 1; }
test -f Cargo.toml || { echo "FAIL: workspace Cargo.toml absent — ADR-0100 pin unassertable (Halt 8)"; exit 1; }
test "$(grep -c 'yaml_serde = "=0.10.7"' Cargo.toml)" -eq 1 || { echo "FAIL: the exact ADR-0100 pin is not present exactly once"; exit 1; }   # → 1 (the only YAML crate)
test -f Cargo.toml || { echo "FAIL: workspace Cargo.toml absent — the YAML-confinement check cannot run (Halt 8)"; exit 1; }
meta=$(cargo metadata --format-version 1) || { echo "FAIL: cargo metadata exited non-zero"; exit 1; }
printf '%s' "$meta" | python -c "import json,sys; m=json.load(sys.stdin); n=sorted(p['name'] for p in m['packages'] if any('yaml' in d['name'] for d in p['dependencies'])); print(n); sys.exit(0 if n==['redkiln-migrate'] else 1)" || { echo "FAIL: the YAML dep set is not exactly ['redkiln-migrate'] (ADR-0100)"; exit 1; }
```

```bash
test -d crates/rk-cli/src || { echo "FAIL: crates/rk-cli/src absent — N is not DECLARED (Halt 8)"; exit 1; }
grep -rnE 'unread[_-]?artifact[_-]?window|window_n' crates/rk-cli/src/ || { echo "FAIL: N is not declared in rk-cli — AC-472/AC-426 have no truth value"; exit 1; }   # → ≥ 1 hit
test -n "$(find crates -name '*.rs' -print -quit)" || { echo "FAIL: crates/ carries no .rs — AC-474 unassertable (Halt 8)"; exit 1; }
! grep -rn 'dual.write\|compat.write' crates/ --include=*.rs --include=*.toml || { echo "FAIL: a dual-write/compat-write route exists (AC-474)"; exit 1; }   # → no match
```

**Acceptance set (explicit):** **AC-400–AC-439** (CLI and `doctor`; AC-428 is a re-issued id — read its hazard note),
**AC-263** (the gate's CLI half, from 7b-ii); the **v0** rows of the migration area, realigned 2026-09-12 to the
register's own phase note (register :6263) — **AC-441, AC-442, AC-443, AC-445–AC-450, AC-463, AC-465, AC-466,
AC-468, AC-470–AC-474, AC-476–AC-478** — plus **AC-522**; **AC-485, AC-486, AC-488**; and **AC-475** as a **negative
tripwire** (below).
**Excluded, by id and reason:** AC-440, AC-444, AC-451–AC-454, AC-456–AC-462, AC-464, AC-467, AC-469, AC-479
and **AC-455** (`rk-kernel` **policy**, not the migrator's: its GIVEN/WHEN is a spec with
`verification: None` against a pack policy and *"the stage's admissibility is evaluated"* — it sits in the
migration id range and belongs to nothing in this step; **cross-listed to 7b-ii**, where admissibility is built)
(**corpus-bound**: the register's phase note names these a *Phase-2 acceptance set* — they need the 1,312-file
corpus, which reaches this box only as Step 2's local-only suite; PIA-002). Until 2026-09-12 this step claimed
nine of them (AC-453–AC-458, AC-462, AC-464, AC-467) on an instrument it cannot run here, which is the PIA-059
shape. AC-505 (Phase 2, `rk-plugin-gen`).
**AC-475 is claimed, deliberately, against the register's placement of it** (DECISIONS-2026-09-12 E13): its Verified-by has
two halves, and the second — *"a schema assertion of zero `ChangeSummary` values in migrated output"* — needs no
corpus and runs on this step's own dry-run fixture. It is a **negative** criterion, so asserting it before the
code that could violate it exists is the cheap direction, and s5 §5.3's *"no `ChangeSummary` value is
synthesised (AC-475)"* would otherwise cite a criterion no Phase-1 stage claims. **Owed to the register
(errata):** AC-475's placement, and **AC-468**, which the phase note lists as corpus-bound *and* as part of what
Phase 1 owes (*"the report schema AC-448/AC-468"*) — it cannot be both; this step claims AC-468 on the report
schema reading and the contradiction is raised, not resolved here. **`doctor`'s unread-artifact window `N` is declared here as
`unread_window_runs = 50`** (DECISIONS-2026-09-12 E9) — **the fourth of the seven decisions this runbook takes**,
now taken. It goes in the `doctor` rule table and in the `[doctor]` block `init` scaffolds; AC-426 makes the key
**required with no default**, so every repository declares its own and 50 is what `init` writes. The warrant, in
one line: a genuinely dead artifact kind has **zero reads forever** — `plan.md` was dead in **110/110** and was
found by census, not by a window — so a wide window costs detection *latency* and nothing else, while a narrow
one false-flags a kind that is legitimately read once per initiative and teaches the operator to ignore `doctor`.
AC-472/AC-426 are evaluable from this value, and SM-139's target can now be missed. The namespace prefix `doctor` and `rk-plugin-gen` lint for is **`rk-`**
(DECISIONS-2026-09-11 #8). **Claim it as AC-531, not AC-506** (corrected 2026-09-12): AC-531 is the Phase-1
criterion for exactly this rule on `redkiln init`'s output, was allocated by the 2026-09-11 amendment outside this
area's exhausted range, and was claimed by no stage — while **AC-506 is Phase 2** and is excluded by name at Step
7a. The evidence is shipmates L3.1: twenty generically-named skills prefixed at a cost of one release, three
regressions and six follow-ups — "free at t=0; ~9 issues at t=30d".

**Before this step, not after (audit §8 C-01):** the CLI/doctor criteria AC-400–AC-439 and SM-121–SM-130 had no owning
slice in the 2026-09-10 audit — 27 of 40 criteria were never named by any finder. Run one slice finder + refuter over
them (register §12, SM §1.11, design §8.1–8.6, this file) with the decidability and two-implementer lenses before the
router is written, and record the result in `PROGRESS.md`.

---
