# Step 7c — The adapters (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7c` | Phase 1 | Checkpoint 7b | Checkpoint 7c | `s7b` | `s7d` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


**Four real adapters** — `rk-adapter-sqlite` (EventStore), `rk-adapter-claude` (Harness), `rk-adapter-toolchain-node`
and **`rk-adapter-toolchain-rust`** (Toolchain; ADR-0094 — the self-hosting run at Step 8 is a Cargo workspace and
needs the second one; design §9.1 is the crate-name authority, so the names are these and not `rk-adapter-node`).
One crate per adapter, each owning its own runtime handle, constructed **once at the composition root and never per
call**. The Rust adapter is the Node adapter's shape over `cargo +1.98.1 build --workspace` / `cargo test --workspace`
(or `cargo nextest run` where present, recorded on the report), `classify` and `change_summary` over `cargo metadata`
and the changeset — `change_summary` returning the opaque newtype until S-BIN — and it binds a **Rust technology
pack** (`TestReport`, `Changeset`) per ADR-0029.

`rk-adapter-claude` lifts `runner.rs`, `event.rs`, `step.rs`, `throttle.rs` and `journal.rs` from the spike runner —
**from `$CORPUS_REPO/runner/src/` at the SHA in `CORPUS_PIN`, never from a working tree** (Step 2 moved the directory;
PIA-018), and the SHA is named in each lifted module's header. **"Lifted" means with edits** (PIA-062): `Usage` and
`CacheCreation` lose their `Default`; `ttl_split()`, `cache_read_share()` and `total_input()` return `None` / propagate
absence when a field is absent; `Margins` has no `Default` and `batch_draw` takes its margin from the pack (AC-377);
and `batch_draw` itself is **rewritten, not lifted** — the tick arithmetic is integer (ADR-0110), not the runner's
`f64`. The async body otherwise survives *inside* the adapter, wrapped by a sync port method that blocks on an
adapter-owned runtime. The runtime is **`multi_thread`** (or a dedicated thread) — a `current_thread` runtime freezes
spawned tasks between `block_on` calls, and the stream reader is a spawned task. An adapter must never call
`Handle::block_on` from a thread its own runtime is driving, and the `-p` stream events must be consumed **inside**
the adapter and surfaced as a completed value — the kernel cannot iterate a stream.

**The port method is ADR-0099's `dispatch`**: it blocks, returns `DispatchOutcome { record, result }`, and is
cancelled only through the `CancelToken` the composition root cloned in at construction — **never** by the kernel.
**Containment is per OS and both are built (ADR-0099 decision 3):** on Windows a Job Object with
`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE`, created **before** spawn, the child assigned before it can fork, the handle held
for the child's lifetime, in one module under a commented `#![allow(unsafe_code)]` (the workspace's only one); on
POSIX `CommandExt::process_group(0)` plus `killpg`. AC-224 enumerates the group via the job or the pgid and includes
a detached-grandchild fixture; AC-225's classifier is keyed on the **token**, not on an exit code — "exits 143" is a
POSIX observation and is recorded per platform (the runner's `taskkill /T /F` victim exited 1, n=1).

Nine patterns transfer from `spikes/runner` **by construction**, each because a specific failure is otherwise
representable. The ones that bite hardest:

- **`spoke()`** — only `StepError::Schema` means the agent produced a judgement; everything else is silence.
  RK-B0013: one HTTP 529 produced a committed `changes-requested` seal whose entire surviving finding was "review
  failed to produce a digest". **Absence is a block, never a pass** (AC-229, AC-230, AC-231).
- **Prompt on stdin, never argv.** A Windows command line caps near 32 KB; the `redkiln-review` card alone is
  6.6 KB. Argv works in testing and fails on a real review.
- **A record is produced even when the step fails.** Cost only becomes visible if failures are priced too.
- **Raw payload retained beside the typed view** — otherwise a malformed digest and a refusal read identically.
- **`Option` over `0`** on usage fields, with **no `Default` impl**, plus the CI grep rule (AC-362) — the scope
  named explicitly, the adapter's usage struct included. **The `ephemeral_1h_*` / `ephemeral_5m_*` fields are
  present** on 954 of 954 recorded runs as a nested object the runner already parses (PIA-067; "F3 measured them
  absent" was false) — a `Some(0)` would still make a TTL claim look verified, which is why absence stays `None`.
- **Provenance at dispatch time**, including a digest of the child's **effective** config read from its own
  `system/init` — an ignored flag looks identical to an honoured one — and the git snapshot captured at the instant
  of dispatch, because it cannot be retrofitted.
- **`StructuredOutput` in every card whose role demands a schema**, checked at **configuration time**. F20: a
  `tools` allowlist silently disables `--json-schema`, `structured_output` absent, **exit 0**, `subtype: success`.
  Nothing parses JSON out of prose.
- **Isolation:** `--strict-mcp-config`, verified from the child's own `system/init`. A default `claude -p` inherits
  12 MCP servers and ~280 tools; MCP alone was **4,631 tokens = 56%** of the cache write.
- **Subscription only.** `rk-adapter-claude` **refuses construction when `ANTHROPIC_API_KEY` is present** — a
  construction failure, not a warning (AC-488). **The child's environment is the captured map minus a committed
  denylist** (`rk-adapter-claude/src/env_denylist.rs`: the vendor key names, sibling harness keys, cloud credentials —
  DECISIONS-2026-09-11 #16); the adapter **never reads the process environment** — it receives the map as a value in
  `Config`, which is what makes AC-217/AC-488's instrument a unit test rather than a forbidden `set_var`. A card with
  no `tools:` key is a **configuration error** (doctor row; shipmates L4.4), never "inherit everything".
- **Exhaustion has a class.** A child that terminates with a usage-limit result resolves to
  `StepError::RateLimited { resets_at }` — `resets_at` from the last `rate_limit_event` in the same stream, else the
  pack delay; detected from the structured `subtype` first and from text markers only as a disclosed fallback
  recorded on the event (AC-523; ADR-0095). It never backs off. Headroom arithmetic is **integer ticks** on the
  visible unified windows (ADR-0110): `delta_ticks <= 1` → `resolution_limited`; refuse at `ticks >= 100 −
  margin_ticks`; `margin_ticks < 2` is a configuration error; the test vectors are 0.09/0.10, 0.10/0.12, 0.20/0.21
  and margins 0.07/0.18 (PIA-017). `SDKRateLimitEvent` is now typed in both Agent SDK references with
  `seven_day_opus`/`seven_day_sonnet` keys [documented, fetched 2026-09-10] — parse it; **never synthesise a family
  value** from it (AC-375).
- **The argv is bounded.** The command builder asserts the assembled argv length before spawn and refuses with a
  typed configuration error at the cap, with a fixture at the cap (PIA-014); `--system-prompt-file` for large cards
  is registered as **S-SPF** (design §10.5) and stays unused in Phase 1. The same builder lints the per-request
  schema-budget precondition ADR-0095 states (one output schema per dispatch, no strict user-defined tools).

Cache and TTL: set **both** `CLAUDE_CODE_PROMPT_CACHE_TTL=1h` **and** `CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL=1h` in
the child's environment, and **read the TTL per run from `usage.cache_creation`** (ADR-0113: `ephemeral_1h > 0` and
`ephemeral_5m = 0` is verified 1 h; both zero with creation > 0 is `TTL-unverified`; all zero is `no-write`). The `-p`
bucket ran at 1 h on every recorded creation (621/621); the subagent bucket is unobserved until S-TTL1. The cache
is scoped **per machine and per directory** (two worktrees of one repository build different prefixes), not
account-wide — "shared-prefix subsidy" is the honest name (PIA-103). Constructing the *child's* environment explicitly is required and is
the opposite of the workspace's own `set_var` ban — `std::env::set_var` / `remove_var` are forbidden **anywhere in
the workspace, tests included**, because they became `unsafe fn` in edition 2024 and race a concurrent `getenv`
under the default multi-threaded harness.

Card delivery, per profile (DECISIONS-2026-09-11 #19; PIA-012): **`static`** — the card body on `--system-prompt`
(AC-211's GIVEN is now "a card whose profile is `static`"); **`default`** — F28's measured arm: **no
`--system-prompt`, the card body prepended to the user message**. A **golden argv test per profile** pins both. The
append flags are **absent from argv** in every profile — S-H1c is unrun, and the route may not be enabled as
anything but an unmeasured config option. The 18,603-token flat-prefix promise (AC-384, SM-113) is `static`'s only.

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- `api_key_refuses_construction` — red with `ANTHROPIC_API_KEY` present in the `Config` map: construction must
  **fail**, not warn (AC-488). The map is a value, which is what keeps this a unit test rather than a forbidden
  `set_var`.
- `structured_output_required` — red on a card that omits `StructuredOutput`: refused at **configuration time**
  (ADR-0108). F20 is why this cannot be a runtime check — a `tools` allowlist silently disables `--json-schema`,
  `structured_output` is absent, and the process **exits 0 reporting success**.
- `golden_argv_per_profile` — two profiles, two pinned argvs, asserted per profile in the test body. Red by
  swapping them: `static` puts the card on `--system-prompt`, `default` prepends it to the user message
  (DECISIONS-2026-09-11 #19). A test that accepts either pins neither.
- `argv_cap_refuses_before_spawn` — the fixture sits **at** the cap and must be refused with a typed configuration
  error **before** spawn (PIA-014).
- `cancel_reaps_process_tree` — red with a **detached grandchild** (AC-224); a fixture that only reaps the direct
  child passes a weaker test. The Windows arm is `cfg`-gated, so a zero-match run on this filter IS the platform
  hole, not a pass.

**And then the fifteenth line, which no test replaces:** one real dispatch, validated before any set (golden
constraint 4). Three E3 defects survived "built and validated" because none was reachable from a unit test.

**Checkpoint 7c.** All fourteen, and then the fifteenth below, which no `cargo test` can stand in for:

```bash
cd D:/repos/redkiln-rs
# Two helpers, defined once so every check below stays one line. Paste the block whole, or run it as
# `bash <<'EOF' … EOF` if you would rather a FAIL not end an interactive shell.
# The rule both exist for: never put the command whose status matters on the LEFT of a pipe. A pipeline's status
# is the last stage's, and `cargo test` prints one `test result:` line PER BINARY — so a passing binary's line
# would launder a red suite green.
rk_test() {      # <pkg> <filter> <what>: cargo's own status FIRST, then a NON-ZERO matched count
  out=$(cargo test -p "$1" -- "$2" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { printf '%s\n' "$out" | tail -20; echo "FAIL: cargo test -p $1 -- $2 exited $rc"; exit 1; }
  printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
    || { echo "FAIL: 0 tests matched '$2' — $3 never ran"; exit 1; }
  echo "PASS: $2 — $3"
}
rk_absent() {    # <pattern> <what>: the search must actually cover code (Halt 8), and only then find nothing
  find crates -name '*.rs' -print -quit | grep -q . \
    || { echo "FAIL: no .rs under crates/ — this absence grep cannot run: $2 (Halt 8, not a pass)"; exit 1; }
  grep -rnE "$1" crates/ --include=*.rs; rc=$?
  [ "$rc" -eq 1 ] || { echo "FAIL: $2 — grep rc=$rc (0 = the hits printed above; 2 = could not search)"; exit 1; }
  echo "PASS: no hits — $2"
}
cargo build -p rk-adapter-claude -p rk-adapter-sqlite -p rk-adapter-toolchain-node -p rk-adapter-toolchain-rust \
  || { echo "FAIL: the four adapters do not build — nothing below is meaningful and no dispatch happens (GC3)"; exit 1; }   # build BEFORE any dispatch
rk_test rk-adapter-claude api_key_refuses_construction 'AC-488: construction refuses while ANTHROPIC_API_KEY is present'
rk_test rk-adapter-claude structured_output_required 'ADR-0108: a card that omits StructuredOutput is REFUSED at configuration time'
rk_test rk-adapter-claude env_denylist_config_map 'the AC-217/AC-488 instrument, dispatch count 0 — a zero-match run has dispatch count 0 too, which is why the match count is asserted'
rk_test rk-adapter-claude golden_argv_per_profile 'static AND default argv pinned (DECISIONS #19) — two profiles; the per-profile count is asserted in the test body, not here'
rk_test rk-adapter-claude argv_cap_refuses_before_spawn 'PIA-014: the fixture AT the cap, refused with a typed configuration error before spawn'
rk_test rk-adapter-claude rate_limited_from_stream 'AC-523: RateLimited { resets_at } from the last rate_limit_event, never a back-off (ADR-0095)'
rk_test rk-adapter-claude tick_arithmetic_vectors 'ADR-0110 fixtures: bases 0.09/0.10, 0.10/0.12, 0.20/0.21 and margins 0.07/0.18 (PIA-017)'
rk_test rk-adapter-claude cancel_reaps_process_tree 'AC-224: the Job Object / process group, detached grandchild included (PIA-047) — the Windows arm is cfg-gated, so a zero-match run here IS the platform hole'
rk_test rk-adapter-toolchain-rust fake_conformance "the fourth adapter's conformance row (ADR-0094)"
rk_absent 'set_var|remove_var' 'edition 2024: set_var/remove_var are unsafe fn — forbidden workspace-wide, tests included'
rk_absent 'append-system-prompt' 'AC-211: the append flags are ABSENT from argv in every profile — S-H1c is unrun'
h=$(grep -rn 'allow(unsafe_code)' crates/ --include=*.rs); printf '%s\n' "$h"   # → exactly ONE hit, in rk-adapter-claude's Job Object module, with its comment (ADR-0102)
[ "$(printf '%s' "$h" | grep -c .)" -eq 1 ] || { echo "FAIL: expected exactly ONE allow(unsafe_code) in the workspace (ADR-0102)"; exit 1; }
printf '%s\n' "$h" | grep -q '^crates/rk-adapter-claude/' || { echo "FAIL: the one allow(unsafe_code) is not in rk-adapter-claude (ADR-0102)"; exit 1; }
rk_absent 'impl Default for (Usage|CacheCreation|Margins)' 'PIA-062: no hand-written Default on the lifted Usage/CacheCreation/Margins (AC-362/AC-377)'
# The lifted source carries the DERIVE form, not an impl block — `grep 'impl Default for'` over
# spikes/runner/src/event.rs matches nothing while five `#[derive(Debug, Clone, Default, …)]` lines stand
# (measured 2026-09-12, event.rs:95/124/191/209/295). An assertion that cannot see the shape it forbids
# passed vacuously over exactly the code this step lifts. Both forms are asserted now:
for t in Usage CacheCreation Margins; do
  f=$(grep -rln "struct $t" crates/rk-adapter-claude/src/) \
    || { echo "FAIL: no struct $t in rk-adapter-claude — the Default ban has nothing to search"; exit 1; }
  printf '%s\n' "$f" | while read -r m; do
    grep -B3 "struct $t" "$m" | grep -qE '#\[derive\([^)]*\bDefault\b' \
      && { echo "FAIL: $t derives Default in $m — PIA-062 requires Option over 0, with NO Default (AC-362/AC-377)"; exit 1; }
  done
  echo "PASS: $t carries no derived Default"
done
rk_absent 'unwrap_or\(0\)|unwrap_or_default\(\)' 'AC-362: no unwrap_or(0)/unwrap_or_default() on the usage struct — a Some(0) makes a TTL claim look verified'
```

**Spikes owed by this step (PIA-047):** **S-CANCEL-WIN** — one fixture-cancel-rm run on Windows, recorded in its
design §10.5 row (the handle case Halt 6 names); **S-RETRY** — its question now covers exhaustion-on-result, consumer
this adapter; **S-TTL1** — narrowed by ADR-0113 to a subagent-card dispatch with both variables set (does the subagent's own creation report `ephemeral_1h > 0`?) and, if a usage-credit state can be produced honestly, the same on usage-credit billing — ≈ $0.03 per dispatch; **S-SPF** — registered, not run. Run each or record its deferral in the checkpoint commit. **The
AC-511 one-dispatch probe** (PIA-059) runs in this step's preamble dispatch: retain the raw `system/init`, name the
auth-mode field by name, and assert on it — or drop the conjunct and record a named open item.

plus, and this one no `cargo test` can stand in for: **one real dispatch validated before any set** (golden
constraint 4), in the foreground (golden constraint 2), with the instrument version written into the record
(golden constraint 8). Three E3 defects survived "built and validated" precisely because none of them was
reachable from a unit test.

**Acceptance set (explicit, per PIA-071/PIA-070):** **AC-200–AC-239** (harness port and adapter) **minus AC-207**
(retired, never reuse) and plus **AC-523**; **AC-360–AC-399** (event log, cost attribution, throttling); the
adapter-side halves of the gates — **AC-282**, and **AC-243/AC-267/AC-269/AC-279** cross-listed from 7b-ii; and the
Toolchain rows **AC-328/AC-329** for both toolchain adapters; plus **AC-526**, allocated outside this area's
exhausted range by the 2026-09-11 amendment and claimed by no stage until 2026-09-12 — the environment builder and
`doctor` must refuse a path that sets **one** of `CLAUDE_CODE_PROMPT_CACHE_TTL` and
`CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL` without the other (ADR-0078; both variables are already prescribed in this
step's cache paragraph, so this is the criterion for text the step carries but never claimed). **Excluded, by id and reason:** AC-240–AC-359's kernel
halves (7b-ii's), **AC-356** (PROVISIONAL, S-BIN), the human-gated halves of **AC-357/AC-358**, **AC-312**
(PROVISIONAL on S-SUBTLE-CB — its framing-variant fixtures are **authored here and left unrun**, priced in design
§10.5; DECISIONS-2026-09-11 #17), AC-255 and AC-297 (cut).

Three of those sets carry standing prohibitions worth restating at the checkpoint: the review gate may **ADD**
findings and may **never CLEAR** an AC (a reproduced regression was approved on 3 of 10 draws; **two** adjudicated
defects escaped through fully green suites of 1,890/1,890 and 1,907/1,907 and two more against an already-red
54-failure baseline — PIA-095); blocking is computed in Rust from re-executed evidence over ADR-0096's typed repro,
never from the model's `pass` (F31's block counts were model-label counts — S-CLASS-REPLAY is what measures the Rust
class); and **no section may claim the review gate is guarded against false approval** — ADR-0101's
prior-approval framing arm is registered and unrun, and S-SUBTLE-SYM beside it.
