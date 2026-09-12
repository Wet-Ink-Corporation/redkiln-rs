# Step 7a — `rk-ports` and the fakes (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7a` | Phase 1 | Checkpoint 6 | Checkpoint 7a | `s6` | `s7b` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Step 7 is four sub-steps — 7a ports, 7b kernel, 7c adapters, 7d CLI — followed by 7e pack authoring, **strictly in
this order**. Each sub-step's checkpoint is its own; do not batch them.

Five named ports — event store, harness, tracker, docs site, toolchain — each a **sync** trait with one fake, and a
real adapter where Phase 1 ships one: SQLite, Claude Code, Node **and Rust** for the toolchain (ADR-0094) — four
reals across three ports; `Tracker` and `DocsSite` have a fake only until Phase 3, and AC-499's matrix carries the
phase clause. **Git is deliberately not a port**; it is a concrete crate the kernel names (AC-483), and its
`GitOps` seam is a struct (ADR-0105 clause 1), so the port-inventory denylist stands.

Port traits are sync because native `async fn` in trait is **still not dyn-compatible** and `async_fn_in_dyn_trait`
remains nightly-only through 1.98.1 (R-ASYNC, discharged 2026-09-10): "no generics leak into the core" and
"dyn-dispatched port" are in direct tension the moment a port method is async. Every `.await` lives in an adapter.

**The port shape is ADR-0099's, and it compiled before this step started (F41, S-PORTS, 2026-09-11):**

- **Port rule P10 (design §3.3, dated 2026-09-11):** `Harness::dispatch(&self, &ActivitySpec, &mut dyn EventStore)
  -> DispatchOutcome { record: DispatchRecord, result: Result<Typed, StepError> }` **blocks to completion**; the
  adapter's declared timeout (AC-223) is the in-band cancel; **cancellation is a `CancelToken`** cloned into the
  adapter at construction and triggered by the composition root's signal path or the per-activity timeout —
  **never by the kernel**, and never a handle the kernel holds. Only the Harness port carries it in Phase 1.
- **Every port trait carries `Send + Sync` as supertraits, and the composition root holds each as
  `Arc<dyn Port + Send + Sync>`** — a fake that holds an `Rc` does not compile, which is one of F41's two
  `compile_fail` doctests; the other is AC-480's generics-leak rule.
- **`Typed`** is an erased validated value (`serde_json::Value` + schema id) the kernel deserialises; `ActivitySpec`
  carries `StepSpec`'s fields and `DispatchRecord` carries `StepRecord`'s (argv and env as digests only — ADR-0105
  clause 7).
- **The Toolchain port's `ChangeSummary` is an opaque, non-finalised newtype in `rk-ports`** —
  `pub struct ChangeSummary(pub(crate) ())` — until S-BIN reports (ADR-0099 clause 5). It is the one site in
  `crates/` the name may appear; Checkpoint 5's grep excludes exactly this crate.

Fake count equals trait count, and the equality is asserted **as a number** (AC-481) — derived from source, not
from a hand-kept inventory (PIA-059: a fixture that adds a trait without editing the inventory must fail) — because
a fake that quietly stops tracking its trait is the failure this catches.

**`spikes/s-ports/` is the reference shape**, run on 1.97.1 and 1.98.1 with an identical result: 5 unit tests and 2
`compile_fail` doctests pass, clippy clean [measured, F41]. Start `rk-ports` from it; do not start from ADR-0076's
"handle minted when the operation started" wording, which F41's first doctest is written to reject.

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- `port_fake_parity` — the assertion is spelled `traits.len() == fakes.len()` and is **evaluated**. Red by adding a
  sixth trait with no fake: the test must fail **without anyone editing an inventory** (PIA-059; AC-481's amended
  Verified-by). A grep that the string exists is not this test; keep the grep as the floor and the fixture as the
  evidence.
- The two `compile_fail` doctests are already red by construction — that is what `compile_fail` means — but assert
  they **ran**: `doctest = false` runs zero and still exits 0, which is why Checkpoint 7a counts them.
- `arc_dyn_port_rejects_rc` — a fake holding an `Rc` must not compile (F41's first doctest); `no_generics_leak` is
  AC-480's. Both are type-level, so both are `compile_fail`, not runtime assertions.

**Checkpoint 7a.** All nine assertions — each is now guarded, so the block is longer than nine lines; count
assertions, not lines:

```bash
cd D:/repos/redkiln-rs
cargo build -p rk-ports && cargo test -p rk-ports         # exit 0 (build first — golden constraint 3)
test -d crates/rk-ports/src || { echo "FAIL: crates/rk-ports/src absent"; exit 1; }
hits=$(grep -rnE 'async fn|impl Future|-> .*Future' crates/rk-ports/src/); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: async fn / future-returning method in a port trait (grep rc=$rc)"; printf '%s\n' "$hits"; exit 1; }   # → silent: the ADR-0076 criterion added 2026-09-11 (grep rc must be exactly 1 = searched, matched nothing)
test -f crates/rk-ports/src/lib.rs || { echo "FAIL: crates/rk-ports/src/lib.rs absent"; exit 1; }
n=$(grep -cE '^pub trait [A-Za-z]+: Send \+ Sync' crates/rk-ports/src/lib.rs)
[ "${n:-0}" -eq 5 ] || { echo "FAIL: ${n:-0} port traits declare Send + Sync, expected 5"; exit 1; }   # → silent: all five port traits carry the supertraits (ADR-0099). Anchored on the declaration: a bare `grep -c 'Send + Sync'` counts doc-comment and helper lines too and reads 10, not 5, on the F41 reference this step says to start from.
test -f crates/rk-ports/src/lib.rs || { echo "FAIL: crates/rk-ports/src/lib.rs absent"; exit 1; }
grep -q 'CancelToken' crates/rk-ports/src/lib.rs || { echo "FAIL: no CancelToken in rk-ports — the token must be a port-crate type, not an adapter local"; exit 1; }   # → silent: ≥ 1
test -d crates/rk-ports || { echo "FAIL: crates/rk-ports absent"; exit 1; }
grep -rn 'traits.len() == fakes.len()' crates/rk-ports/ || { echo "FAIL: the AC-481 parity assertion is absent — it must be spelled 'traits.len() == fakes.len()' (design §3 P2, AC-481's Verified-by)"; exit 1; }   # → ≥ 1 hit printed: the assertion EXISTS
out=$(cargo test -p rk-ports -- port_fake_parity 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test port_fake_parity exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the filter matched 0 tests — port_fake_parity does not exist under that name"; exit 1; }   # → silent: it ran AND it PASSES
out=$(cargo test -p rk-ports --doc 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --doc exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
n=$(printf '%s\n' "$out" | sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' | head -1)
[ "${n:-0}" -ge 2 ] || { echo "FAIL: ${n:-0} doctests ran — F41's two must both be present (doctest = false runs 0 and still exits 0)"; exit 1; }
n=$(grep -rnE '[`]{3}compile_fail' crates/rk-ports/src/ | wc -l); [ "$n" -eq 2 ] || { echo "FAIL: $n compile_fail doctests in rk-ports/src, expected exactly 2"; exit 1; }   # → silent: both of F41's are carried over and still fail to compile
test -d crates/rk-ports/src || { echo "FAIL: crates/rk-ports/src absent"; exit 1; }
n=$(grep -rn 'ChangeSummary' crates/rk-ports/src/ | grep -c 'pub(crate) ()')
[ "${n:-0}" -eq 1 ] || { echo "FAIL: ${n:-0} 'pub(crate) ()' ChangeSummary sites, expected exactly 1 (ADR-0099 clause 5)"; exit 1; }   # → silent: the opaque newtype, and nothing else about it
out=$(cargo tree -p rk-ports -e normal); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo tree -p rk-ports exited $rc — nothing was resolved, so a count of 0 would be a fake 0"; exit 1; }
if printf '%s\n' "$out" | grep -qE 'tokio|async-std|futures|async-trait'; then echo "FAIL: rk-ports pulls an async runtime"; printf '%s\n' "$out" | grep -nE 'tokio|async-std|futures|async-trait'; exit 1; fi   # → silent: → 0
```

"the assertion present and passing" was an intention; those last two lines are it as commands. The number matters
(AC-481) because a fake that quietly stops tracking its trait is exactly what this catches, and a `traits.len()`
that is never evaluated catches nothing.

**Acceptance set (the explicit Phase-1 subset — PIA-071, PIA-070):** **AC-480–AC-495, AC-497–AC-500, AC-509,
AC-511, AC-518** — with **AC-497** carried only as the opaque-newtype assertion (its `change_summary` body is S-BIN's;
PROVISIONAL) and **AC-499** only for the ports with a Phase-1 real adapter (EventStore, Harness, Toolchain — the
Tracker/DocsSite rows say "fake only until Phase 3"). Plus **AC-528**, allocated outside this area's exhausted range by the 2026-09-11 amendment and claimed by no
stage until 2026-09-12: the five port traits stay **sync** and no async runtime reaches the kernel's normal
dependency graph (ADR-0076; `async_fn_in_dyn_trait` is nightly-only through the 1.98.1 pin — R-ASYNC discharged
negative). **Its scope is owed a reconciliation (R11):** the register's clause names
`cargo tree -p rk-kernel -e normal` while Checkpoint 7a runs it on `-p rk-ports` and s5 §5.2's table says
`rk-kernel`. Both crates must be clean; run it on both here and state the scope once when the register is corrected.
**Excluded, by id and reason:** AC-501–AC-506 (Phase 2),
AC-507/AC-508/AC-510 (Phase 3), AC-512/AC-514–AC-517/AC-519 in part (CONDITIONAL — unbuildable until the distribution
decision), AC-496 (PROVISIONAL), AC-482 (human-gated), AC-513 (retired, never reuse), AC-502 (S-CODEX unrun; Phase 4
is the earliest a second harness ships), AC-239 (list-is-data test — kept, in 7b's set). The `rk-port-toolchain`
label on register rows is a subject tag for this crate (design §9.1 is the crate authority).

**Spikes this step consumed:** S-PORTS (F41, done). **Spikes it owes:** none — S-BIN is 7c's `change_summary` body.
