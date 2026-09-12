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
and an idempotent `init`. The adapter set — all four reals (ADR-0094) — is resolved **once at startup** and is a
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

**Checkpoint 7d:**

```bash
cd D:/repos/redkiln-rs
cargo build --workspace
./target/debug/redkiln doctor           # exit 0, zero model cost, one rule table printed
./target/debug/redkiln run --help       # the idempotent entry exists
./target/debug/redkiln run --resume 2>&1 | grep -qiE 'unexpected|unrecognized|error' && echo "resume is refused"   # AC-400/AC-264: the flag does not exist (ADR-0107)
./target/debug/redkiln answer --help    # the only Phase-1 gate channel
./target/debug/redkiln init && ./target/debug/redkiln init ; echo "exit=$?"   # → exit=0 twice: idempotent (AC-437)
cargo test --workspace                  # trycmd/assert_cmd CLI-surface tests pass; the argument-surface snapshot is committed
./target/debug/redkiln-migrate --version                                       # → the binary EXISTS (v0, PIA-002)
./target/debug/redkiln-migrate --dry-run --target <a small fixture> ; echo "exit=$?"   # → exit=0 and a report on stdout conforming to AC-448/AC-468
cargo test -p redkiln-migrate -- refuses_dirty_target ; echo "exit=$?"         # → exit=0 (AC-522)
cargo test -p redkiln-migrate -- idempotent_across_kill ; echo "exit=$?"       # → exit=0 (AC-450 extended)
grep -c 'yaml_serde = "=0.10.7"' Cargo.toml                                    # → 1 (ADR-0100; the only YAML crate)
cargo metadata --format-version 1 | python -c "import json,sys; m=json.load(sys.stdin); print(sorted(p['name'] for p in m['packages'] if any('yaml' in d['name'] for d in p['dependencies'])))"   # → ['redkiln-migrate']
```

```bash
grep -rnE 'unread[_-]?artifact[_-]?window|window_n' crates/rk-cli/src/ | wc -l   # → ≥ 1: N is DECLARED
grep -rn 'dual.write\|compat.write' crates/ --include=*.rs --include=*.toml | wc -l   # → 0 (AC-474)
```

**Acceptance set (explicit):** **AC-400–AC-439** (CLI and `doctor`; AC-428 is a re-issued id — read its hazard note),
**AC-263** (the gate's CLI half, from 7b-ii); the **v0** rows of the migration area — **AC-441, AC-442, AC-443,
AC-445–AC-450, AC-453–AC-458, AC-462–AC-468, AC-470–AC-478** — plus **AC-522**; **AC-485, AC-486, AC-488**.
**Excluded, by id and reason:** AC-440, 444, 451, 452, 459, 460, 461, 469, 479 (corpus-bound; the local-only suite
and Phase 2's full-migration set — PIA-002), AC-505 (Phase 2, `rk-plugin-gen`). `AC-472`/`AC-426` cannot be evaluated
until `doctor`'s unread-artifact window `N` is **declared** — that is **the fourth of the seven decisions this runbook
takes**. Declare it in the rule table in this step and record the number, or the criterion has no truth value and
SM-139's target cannot be missed. The namespace prefix `doctor` and `rk-plugin-gen` lint for is **`rk-`**
(DECISIONS-2026-09-11 #8; AC-506).

**Before this step, not after (audit §8 C-01):** the CLI/doctor criteria AC-400–AC-439 and SM-121–SM-130 had no owning
slice in the 2026-09-10 audit — 27 of 40 criteria were never named by any finder. Run one slice finder + refuter over
them (register §12, SM §1.11, design §8.1–8.6, this file) with the decidability and two-implementer lenses before the
router is written, and record the result in `PROGRESS.md`.

---
