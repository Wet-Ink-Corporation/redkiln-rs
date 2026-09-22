# Step 7g — The step executor and the Toolchain port: the five closed step kinds, executed (Phase 1, part 5)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7g` | Phase 1 | Checkpoint 7f | Checkpoint 7g | `s7f` | `s7h` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** `s7f` §7f.1 states the finding and arms the gate; this step discharges two thirds of it.
design §4.4 fixes **five closed step kinds** and gives each its own execution — *"`agent` (the only place a model
runs), `tool` (a toolchain verb through the port), `check` (a deterministic predicate in Rust; cannot dispatch),
`gate` (the kernel writes the mark and **exits 0**, resuming into the same step), `emit` (binds outputs to
`provides`)"* — and the runtime implements one of them by dispatching the whole activation to the harness.

**This is a seam, not a design.** The data is already parsed and the surrounding machinery already reasons about
step kinds: `catalog::Step` carries `kind`, `run`, `output_type`, `role`, `when`, `expr`, `for_each`, `retry_max`,
`repeat_max`, `memo_declared`; `workflow::budget` already branches `StepKind::Agent | StepKind::Tool`;
`workflow::memo` already reads `step.kind == StepKind::Tool && step.memo_declared`; `rigor` already branches
`StepKind::Tool | Gate | Emit`; `predicate::eval` is the `check` arm's evaluator and is complete. **Only the
execution point is missing**, which is why `ports.toolchain`, `ports.tracker` and `ports.docs` have zero non-test
call sites.

**This step takes no decision.** Every arm's behaviour is design §4.4's sentence for that kind.

### 7g.1 The executor walks the steps, and the kind chooses the arm

`Runtime` gains the step walk. The activation remains the unit of *selection* — the DAG's node is an activity, and
that is correct (selection picks activities; execution walks their steps) — but the unit of *execution* becomes the
step, in declaration order, with `when` consulted per step.

The five arms, each pinned to its own sentence:

| kind | arm | the port it reaches |
| --- | --- | --- |
| `agent` | the existing dispatch, unchanged | `Harness` |
| `tool` | `run = "toolchain:<verb>"` resolved against the capability registry | **`Toolchain`** |
| `check` | `expr` through `predicate::eval`; **cannot dispatch** | none |
| `gate` | the kernel writes the mark and exits 0, resuming into the same step | none |
| `emit` | binds outputs to the activity's `provides` | none |

🛑 **AC-013's idempotence is per ACTIVATION and must stay so.** `is_committed(item, activity_id_at_version)` is the
FSM's own predicate and the re-entry path reads it; a step walk that re-ran committed activations would break
`kill_and_resume_n_step` and criterion (3). The walk happens **inside** an activation that is not yet committed.

🛑 **`check` cannot dispatch, and that is asserted, not assumed.** design §4.4's parenthesis is a prohibition; the
arm takes no `Ports` at all, so the compiler carries it.

### 7g.2 The Toolchain port's call site, and what AC-473 needs from it

`rk-adapter-toolchain-rust` implements the `Toolchain` port — `detect`, `build`, `test`, `lint`, `classify`,
`change_summary` — and **is never constructed on the run path**: `run.rs` wires `FixtureToolchain::default()`.
AC-473 requires the self-hosting run's *"toolchain stage [to have] run through `rk-adapter-toolchain-rust` behind
the `Toolchain` port (ADR-0094 decision 1)"*, so until a `tool` step reaches the port that clause has no mechanism
and criterion (1) cannot be satisfied however well the lane otherwise runs.

The shipped catalog's `tool` steps, measured 2026-09-21 — **113** across 12 verbs, the largest being
`toolchain:test` (**28**), `toolchain:change_summary` (11), `toolchain:verdict_record` (8), `toolchain:deploy` (6)
and `toolchain:metrics` (6). The adapter selection is `config.toml`'s `[adapters] toolchains`, which already lists
both adapters; which one serves an instance is ADR-0073's keyed expansion and is not re-decided here.

**`change_summary` stays refused.** Both adapters return `Err(registry::refusal(...))` from a dated registry row,
and ADR-0093 decision 4 keeps `ChangeSummary` an opaque non-finalised newtype until S-BIN reports. A `tool` step
naming `toolchain:change_summary` therefore reaches the port and receives `Unsupported` — which is the correct
behaviour and must not be "fixed" into a synthesised value. Checkpoint 5's RED is untouched by this step.

### 7g.3 The `.redkiln/` fills the run demands, authored when it demands them

The first invocations after §7g.1 will refuse in turn on inputs Step 7e did not author, each measured 2026-09-21 at
Step 8: `.redkiln/schemas/<Kind>.json` is absent (read per role-bearing node); **all 8** cards lack
`permission_mode`; both packs' `[role.*]` tables carry `family` and no `model`/`effort`; and **20 of 102** catalog
activities carry no `role =` on any code line, of which `change-summary` is in `bug-standard`'s `[stage.implement]`
baseline and `confirmation-test` / `smoke-test` are the verify stage's.

**Authored by selection, never speculatively.** A fill is added when a run demands it and the refusal names it —
twenty roles invented ahead of a run would be twenty unmeasured decisions, and `s7e`'s dispositions are the
precedent for recording each one's warrant.

### 7g.4 Checkpoint 7g

- **Two of the three ARMED conformance lints are GREEN**, their `#[ignore]` removed in the commit that made each
  pass: every `StepKind` has an executor arm, and `ports.toolchain` has a non-test call site.
- **A `tool` step reaches `rk-adapter-toolchain-rust`** through the `Toolchain` port, asserted by the adapter being
  observed rather than by the step being selected.
- **`check` cannot dispatch** — asserted structurally.
- **AC-013's re-entry still holds**: `kill_and_resume_n_step` and criterion (3)'s asserted half stay green.
- **Criterion (2) stays green** — the step walk must not move the tier-1 projection.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
out=$(cargo test -p rk-kernel --test it -- conformance_every_step_kind_has_an_executor 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the step-kind executor lint is red or absent (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 tests matched — the lint was deleted rather than discharged"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- conformance_every_port_has_a_call_site 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the port call-site lint is red (rc=$rc) — Toolchain has no call site"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo test -p rk-kernel -- kill_and_resume_n_step 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: AC-013's re-entry broke under the step walk (rc=$rc) — criterion (3) regressed"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo test -p rk-kernel -- rebuild_differential 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the tier-1 projection moved under the step walk (rc=$rc) — criterion (2) regressed"; printf '%s\n' "$out" | tail -20; exit 1; }
echo "CP7G-END-OF-BLOCK reached"
```

---
