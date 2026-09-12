# Step 5 — Workspace, CI, and `rk-types` (Phase 1, part 1)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s5` | Phase 1 | 🚦 gate check + Checkpoint 2 | Checkpoint 5 | `s2` | `s6` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


> ### 🚦 GATE CHECK — run this line before the first `cargo new`
> ```bash
> ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>   && echo "GATE DISCHARGED — proceed" || echo "GATE OPEN — go back to Step 4"
> ```
> **`0090`, not `0089`.** `0089` is the facet-provenance amendment Step 4a executes; it is accepted, it does not
> touch ADR-0079, and it is **not** this gate's token.
> Phase 1 entry also needs **Step 3c** answered (both exit-(c) rows, S-BIN and S-FACET), **Step 4a** applied
> (`grep -c 'source = "llm"' spikes/p9/facets.toml` → 0) and **Step 2** landed (`test -f CORPUS_PIN`;
> `git status --porcelain` empty). Nothing in *this* step touches the selection path, so a discharged gate is a
> formality here — which is exactly why it is checked here, where it is cheap, rather than first at Step 7b, where
> it is not.

The workspace and CI are **part of this step, not an afterthought**. There is **no `.github/workflows` directory in
this repository at all** — the single largest of the four measured absences, alongside no `rust-toolchain.toml`, no
`[lints]` table and no integration-test directory. ADR-0083 specifies CI as "a floor that must never again be
absent". It goes in with the first crate.

### 5.1 The workspace skeleton

```bash
cd D:/repos/redkiln-rs
ls crates/                       # exists and is EMPTY today — verified 2026-09-11
cargo new --lib crates/rk-types
```

Root `Cargo.toml` — workspace package fields inherited, lints in **one** place. **Write the root manifest before
you run `cargo build`, and keep the `exclude`:**

```toml
[workspace]
members = ["crates/*"]
resolver = "2"
# spikes/runner/ is its own package with its own Cargo.toml. If Step 2 has landed it is gone and this
# line is inert; if it has NOT, cargo walks up from spikes/runner/, finds this root, sees that
# members = ["crates/*"] does not cover it, and fails with
#   error: current package believes it's in a workspace when it's not
# — which reads as Halt condition 1 (the rig fails to build) from entirely the wrong direction.
# spikes/s-ports/ is the S-PORTS throwaway crate (F41) and is excluded for the same reason.
exclude = ["spikes/runner", "spikes/s-ports"]

[workspace.package]
edition = "2024"
# Equal to the pin, always (ADR-0102): it arms resolver 3's MSRV fallback and `cargo add`; it is NOT a
# consumer floor. The pin-bump PR bumps this and rust-toolchain.toml together.
rust-version = "1.98.1"

[workspace.lints.rust]
unsafe_code = "deny"

[workspace.lints.clippy]
# Group entries carry an explicit priority so individual lints below override them (Cargo's
# lint_groups_priority rule — the previous revision of this block failed `cargo clippy` on the pinned
# toolchain without it; PIA-025 / ADR-0102).
all = { level = "deny", priority = -1 }
# An ENUMERATED pedantic subset, never the group: clippy::correctness is the only deny-by-default group,
# and the Clippy Lints book documents pedantic as false-positive-prone and nursery as "new lints that are
# still under development" — so a group deny across a dozen crates is a CI outage waiting on the next release.
semicolon_if_nothing_returned      = "deny"
explicit_iter_loop                 = "deny"
match_same_arms                    = "deny"
redundant_closure_for_method_calls = "deny"
items_after_statements             = "deny"
nursery = { level = "warn", priority = -1 }
```

Every member carries `[lints] workspace = true` **and nothing else** — Cargo rejects any `[lints]` table that mixes
`workspace = true` with other keys (RFC 3389; cargo #13157), so the per-crate override ADR-0083 described in a
member manifest does not exist (PIA-026). The **only** per-crate exception to `unsafe_code = "deny"` is a commented
`#![allow(unsafe_code)]` crate- or module-level attribute — ADR-0099's Job Object module in `rk-adapter-claude` is
the one that will carry it — and **every other in-source override is `#[expect(clippy::<lint>, reason = "…")]`**,
never `#[allow]`, so an override that stops being needed fails the build (ADR-0102).

A `clippy.toml` sits beside the root manifest (PIA-031) and carries the `disallowed-methods` / `disallowed-types`
tables AC-001–AC-004's instrument reads: `std::env::set_var`, `std::env::remove_var` (ADR-0083's ban),
`std::process::Command` and `tokio::process::Command` outside `rk-git`'s wrapper module (Checkpoint 6's type ban),
and the `unwrap_or(0)` / `unwrap_or_default()` shapes AC-362 forbids on the usage struct. AC-001–AC-004's Verified-by
is a fixture crate checked under `cargo clippy` — a `trybuild` fixture cannot fail on a lint.

`rust-toolchain.toml` at the root — an **exact** channel, not `stable`:

```toml
[toolchain]
channel = "1.98.1"
components = ["rustfmt", "clippy"]
```

A pin and a floor answer different questions and are not substitutes; `workspace.package.rust-version` is set
**equal** to the pin (ADR-0102), never below it and never instead of it. The pin is **1.98.1** (DECISIONS-2026-09-11
#7); every measurement in the spike programme was taken on 1.97.1 and the move is recorded in Step 0.3.

### 5.2 CI, from the first commit

`.github/workflows/ci.yml` runs on every push and PR, on the pinned toolchain, **with a Windows target** (the
`gix-*` advisory cluster is checkout and path handling, several of them Windows-specific, on the platform this is
being built on):

**This is a gate list, not a file.** The four lines below are the four gates ADR-0083 requires; they are not valid
GitHub Actions YAML and pasting them into `ci.yml` will not run. Author the real workflow around them — `on: [push,
pull_request]`, a matrix `[ubuntu-latest, windows-latest]` (PIA-031: two OS targets, so AC-121's cross-OS half and
AC-158's ordering are verifiable rather than "unverifiable as specified"), `dtolnay/rust-toolchain` reading
`rust-toolchain.toml`, and `cargo install cargo-audit --locked` and `cargo install cargo-deny --locked` in the job
that needs them.

```yaml
# five gates, none optional — ADR-0083 + ADR-0100. A GATE LIST, not a workflow file.
- cargo fmt --all --check
- cargo clippy --workspace --all-targets     # gated on EXIT CODE; nursery warnings are permitted (ADR-0102)
- cargo test --workspace                     # the corpus-bound suite is NOT here — see below
- cargo audit --deny unmaintained            # against the committed Cargo.lock, watching gix-* specifically
- cargo deny check bans advisories licenses  # deny.toml: ADR-0086's four bans, workspace-wide incl. dev-deps (ADR-0100)
```

**The corpus-bound tests are a local-only suite (decision D-N; PIA-002).** AC-440, 444, 451, 452, 459, 460, 461,
469, 479, AC-165/AC-184's replay tests and AC-006/AC-188's golden-log replay all read `redkiln-corpus` at
`CORPUS_PIN`, which no hosted runner can fetch. They compile behind a `corpus` cargo feature and run as
`cargo test --workspace --features corpus` on a named local runner (this box, and any box with
`D:/repos/redkiln-corpus` at the pinned SHA); CI's `cargo test --workspace` runs everything else. A corpus-bound
test that is not behind the feature is a CI failure, which is the point — nothing is silently absent.

**`deny.toml`** is committed at the root with `[bans].deny` = `temporalio-sdk`, `graph-flow`, `dbos`, `durable`
(ADR-0086's four), `[advisories] unmaintained = "deny"`, and `[licenses]` allowing the set the workspace already
uses; a must-fail fixture adds a banned crate to a scratch member and asserts `cargo deny check bans` exits non-zero
(AC-005/AC-044/AC-120's Verified-by, which now names an instrument that exists — PIA-029).

**All five tools are installed on this box as of 2026-09-11** — `cargo-audit` 0.22.2, `cargo-insta` 1.48.0,
`cargo-deny` 0.20.2 and the 1.98.1 toolchain were installed during the remediation pass (the 2026-09-08 measurement
had the first two MISSING). The preamble's preflight asserts them; if any is absent:

```bash
cargo install cargo-audit --locked
cargo install cargo-insta --locked
cargo install cargo-deny --locked
rustup toolchain install 1.98.1 --profile minimal -c rustfmt -c clippy --no-self-update
```

CI also carries the **structural assertions**, because a rule stated in prose is not a rule:

| Assertion | AC |
| --- | --- |
| `cargo metadata` shows no `rk-kernel` → `rk-adapter-*` edge | **AC-484** |
| no port-bound generic parameter in `rk-kernel`'s API surface | **AC-480** |
| port-inventory lint with a denylist on the git surface | **AC-483** |
| `traits.len() == fakes.len()` in `rk-ports` | **AC-481** |
| grep rule forbidding `unwrap_or(0)` / `unwrap_or_default()` on the usage struct's fields | **AC-362** |
| the golden schema snapshot (§5.3 below) | **AC-129**–**AC-133**, AC-157, AC-158 |
| corpus-residency lint: no frozen tree, fixture or spike JSONL tracked here | **AC-465** — enforceable only after Step 2 |
| `cargo deny check bans` must-fail fixture (a banned crate in a scratch member) | **AC-005**, AC-044, AC-120 |
| no `async fn` / future-returning method in any `rk-ports` trait; `cargo tree -p rk-kernel -e normal` names none of tokio/async-std/futures/async-trait | the ADR-0076 criterion the 2026-09-11 amendment added (see the register's §15.3) |
| CI grep: `Command::new("git")` appears only inside `rk-git`'s wrapper module | ADR-0105 clause 5; Checkpoint 6 |

Two properties design §9.1 calls "by construction, not by care" had **no criterion in the register** — `anyhow`
in exactly one crate family (now `rk-cli` and `redkiln-migrate`, ADR-0102), and the environment read in exactly one
place. They are conventions with a manifest-level shape, not checks. Author a manifest-level assertion for each
here (a `cargo metadata` walk asserting which crates depend on `anyhow`; the AC-217/AC-488 `Config`-map instrument
for the environment), or record in the commit that they remain conventions. Do not let the phrase do the work.

### 5.3 `rk-types` and the golden schema snapshot

`rk-types` owns ids; the typed artifacts (`Spec`, `ImplReport`, `Findings`, `DocSet`, digests); schema derivation;
`StepError`; `Drift`; `GateBasis`. It may depend on **no async runtime** — no `tokio`, no `async-std`, no
`futures`, no `async-trait`. Four shapes it ships are fixed by the 2026-09-11 records and are **not** left to the
implementer:

- **`StepError`** is ADR-0095's table as ADR-0112 extends it — **eleven** variants: `Transport`, `Schema { attempts }`,
  `Timeout`, `Cancelled { by }`, `NonZero { code, stderr_digest }`, `RateLimited { resets_at }`, `NotAdopted`,
  `NotIsolated { surface }`, `Platform { status, retry_after }`, `Unsupported { capability }`, `Overage { reason }` —
  with `spoke()` total over it and one event arm each. S-PORTS compiled exactly this shape on 1.98.1 (the eight on
  1.97.1 as well) [measured, F41 + its 2026-09-11 note]; the variant set is redesigned from step.rs's six, not carried.
- **`Findings.repro`** is typed (ADR-0096): `{ kind: run | mutate, command, mutation: Option<{ patch, suite }>,
  expected: { exit } | { assertion }, observed, expected_when_met }`; a prose-only `command` is rejected at ingest,
  never routed to advisory. The snapshot is taken on this shape; F31's 52/52 conformance figure describes the old
  one.
- **`ReviewWeight`**, **`Gwt`**, the verify-gate **ledger** and the **`EvidenceClass` / `ReExecution`** split are as
  DECISIONS-2026-09-11 fixes them (PIA-032/033): `ReviewWeight = { ReadOnly, Lenient, Strict }` as a projection of
  stage rigor; `Gwt { given, when, then, and: Vec<Text> }` never populated by migration; one closed ledger enum per AC
  with `unpinned` reserved; `EvidenceClass` (four, from the repro record) distinct from
  `ReExecution = Flipped | Unverifiable { both_fail | cannot_run | timeout }`.
- **The Phase-1 companion kind list is decided here, once (decision 7 of the preamble's table; DECISIONS-2026-09-11
  #3, recorded against ADR-0075 clause 3):** `_integration`, `_grounding`, `_decomposition`, `_review`, `_ledger` —
  AC-143's five plus the 220-file `ledger` the verify gate writes — each with its own registered sidecar schema. The
  other eight kinds the S-MIG census holds (closure 30, design 26, design-review 22, implementation 4, plan 4,
  retrospective 4, slices 26, storymap 30) are Phase-2 kinds: `redkiln-migrate` lifts them as opaque prose per
  ADR-0075 clause 6, and a companion kind with no registered schema cannot be written (AC-143's fixture enumerates
  exactly these six). The spec template also gains the per-AC `mutation:` field ADR-0104 makes the harness's input.

**`ChangeSummary` is NOT built in this step.** S-BIN is registered and unrun, and it **gates Phase 1** for this
type specifically: no review evidence of any kind exists for items whose change is not a text diff, so the field
set cannot be fixed. The crate ships every other artifact type. Historical change material is carried as opaque
content with a migration-loss marker, and **no `ChangeSummary` value is synthesised** (AC-475). The five
toolchain-keyed kinds (ADR-0073) are not finalised either.

**The fixture existing does not move this.** Step 3c records that S-BIN's fixture is now identified — the real
Unreal project at `D:/repos/BrittonGames`, UE 5.8, headless automation working, assets in Git LFS [measured, paths
verified 2026-09-09]. **S-BIN has not run**, so nothing here changes: the field set stays unfixed, `AC-154` stays
unverifiable-as-specified, `AC-356`'s evidence standard stays blocked, and the per-AC mutation norm stays in force
for every class. A fixture is not a finding.

**Nor does `binary_change` become a selection input.** It is measured — precision **0.60**, 95% CI [0.23, 0.88],
recall **1.00**, 3 tp / 2 fp / 0 fn over 20 items [measured, F39 §1] — and it is **not in `facets.toml`**, so it
can feed no `require` row regardless (DECISIONS-2026-09-09 #11). Its `docs` source finds **nothing at all**: no
positives, recall 0.00, 3 missed, because a card names a path but carries no binary marker, so the facet is
unreachable from prose by construction [measured, F39 §3]. Do not register it here to "unblock" anything.

The closed-output rules are constraints on the **Rust types**, and `schemars` does not enforce them — and, measured
against schemars 1.2.2, `#[serde(tag = "kind")]` renders a **`oneOf`**, not "a single discriminated object" (PIA-005;
ERRATA E-0084-2). **The rule is ADR-0095's, stated identically in AC-131, AC-157, design §5.3 and here:** every
exported schema contains **no `oneOf` at any node** — tagged enums pass through a post-derive
`schemars::transform::Transform` inside `rk-types`' export that rewrites a `kind`-discriminated `oneOf` into an
equivalent `anyOf`; **no union nests inside a union**; `additionalProperties: false` at **every** object level;
`required` ordered first; **≤ 16 union-typed parameters** (any property whose schema uses `anyOf` or a type array —
a nullable `Option<T>` is one) and **≤ 24 optional parameters, counted per exported schema**. Those two caps are the
vendor's **Schema complexity limits** (platform.claude.com structured-outputs page, fetched 2026-09-10: 24 optional and
16 union-typed parameters per *request* across all strict tool schemas plus the JSON output schema; 20 strict tools;
180 s compile), and per-schema equals per-request only while one output schema is sent per dispatch with no strict
user-defined tools on the argv — that precondition is a GIVEN in AC-132/AC-157 and a lint on the adapter's command
builder (Step 7c), so adding a strict tool later re-opens the budget explicitly. **S-SCHEMA-CAP** (design §10.5)
observes the vendor's rejection shape on a 17-union-parameter and a 25-optional schema, because F20 says the failure
is silent and nothing has yet seen it. So the rule gets a test:

```bash
cargo +1.98.1 build -p rk-types                      # golden constraint 3 — build before you test
cargo +1.98.1 test -p rk-types --  schema_snapshot   # insta golden snapshot over every exported schema
```

`schemars` and `serde_json` are pulled with `features = ["preserve_order"]` and the snapshot test asserts property
order (PIA-031), so AC-158 is verifiable rather than "unverifiable as specified". The snapshot directory is
`crates/rk-types/tests/it/snapshots/` — the consolidated integration-test layout ADR-0083 prescribes — and the test
pins it with `insta::Settings::set_snapshot_path` so Checkpoint 5's guard and the test name the same path (PIA-034).
The **first** snapshots are accepted deliberately, once, before Checkpoint 5:

```bash
cargo +1.98.1 insta test -p rk-types --accept   # inspect every .snap before accepting — this is the one rubber-stamp
                                                # the residual below warns about, taken on purpose and once
```

A changed snapshot is a schema change and is reviewed as one. The residual ADR-0084 names about itself stands: a
reviewer can rubber-stamp `cargo insta accept`, and the guard against that is convention, not mechanism.

**Checkpoint 5.** All of:

```bash
cd D:/repos/redkiln-rs
# BEFORE assertions — each fails on the untouched tree (PIA-035); run them first and expect the stated failure
test ! -f rust-toolchain.toml && echo "BEFORE: no pin yet"           # → prints on the untouched tree; after the step, silent
test ! -d .github/workflows && echo "BEFORE: no CI yet"               # → prints on the untouched tree
# AFTER assertions
cargo build --workspace                                  # builds, on the pinned toolchain (rust-toolchain.toml selects it)
cargo fmt --all --check                                  # exit 0
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"; [ "$rc" -eq 0 ] || exit 1   # → exit=0; nursery WARNINGS are permitted, the gate is the exit code (PIA-030)
out=$(cargo test --workspace 2>&1); rc=$?                 # exit 0 — the corpus-bound suite is behind --features corpus
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }
test -f CORPUS_PIN && test -d D:/repos/redkiln-corpus || { echo "FAIL: corpus absent — Step 2 has not landed, or redkiln-corpus is not checked out on this box"; exit 1; }
out=$(cargo test --workspace --features corpus 2>&1); rc=$?   # not a CI line: it runs only on a box holding redkiln-corpus at CORPUS_PIN
[ "$rc" -eq 0 ] || { echo "FAIL: corpus suite exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 corpus tests ran"; exit 1; }
cargo audit --deny unmaintained                          # exit 0
cargo deny check bans advisories licenses                # exit 0 (deny.toml committed)
test -f rust-toolchain.toml && grep -q 'channel = "1.98.1"' rust-toolchain.toml   # the FULL string (PIA-024), not a prefix
grep -q 'rust-version = "1.98.1"' Cargo.toml             # equal to the pin (ADR-0102)
grep -q 'exclude = \["spikes/runner", "spikes/s-ports"\]' Cargo.toml   # the exclude survived the first `cargo new` (PIA-035)
test -f .github/workflows/ci.yml && grep -q 'windows-latest' .github/workflows/ci.yml && grep -q 'ubuntu-latest' .github/workflows/ci.yml
test -f clippy.toml && grep -q 'disallowed-methods' clippy.toml
test -f deny.toml || { echo "FAIL: deny.toml absent"; exit 1; }
for b in temporalio-sdk graph-flow dbos durable; do grep -q "$b" deny.toml || { echo "FAIL: deny.toml is missing ADR-0086 ban $b"; exit 1; }; done   # → all four, whatever the line layout
n=$(ls crates/*/Cargo.toml 2>/dev/null | wc -l); [ "$n" -gt 0 ] || { echo "FAIL: no crates/*/Cargo.toml"; exit 1; }
miss=$(grep -L '^\[lints\]' crates/*/Cargo.toml); [ -z "$miss" ] || { echo "FAIL: no [lints] table in: $miss"; exit 1; }   # → empty: every member carries the table
n=$(ls -d crates/*/ 2>/dev/null | wc -l); k=$(grep -l 'workspace = true' crates/*/Cargo.toml 2>/dev/null | wc -l)
[ "$n" -gt 0 ] && [ "$n" -eq "$k" ] || { echo "FAIL: $k of $n members carry 'workspace = true'"; exit 1; }   # → the member count, asserted not eyeballed
test -d crates/rk-types/tests/it/snapshots || { echo "FAIL: snapshot dir absent (PIA-034)"; exit 1; }
one=$(grep -rnE 'oneOf' crates/rk-types/tests/it/snapshots/); [ -z "$one" ] || { echo "FAIL: oneOf in an exported schema:"; printf '%s\n' "$one"; exit 1; }   # → 0, and the dir MUST exist (PIA-034)
                                                          # anyOf is legal only as schemars' nullable form and
                                                          # never nested — the snapshot TEST asserts that, not a grep
test -d crates/rk-types/src || { echo "FAIL: crates/rk-types/src absent"; exit 1; }
cs=$(grep -rn 'ChangeSummary' crates/ --include='*.rs' | grep -v 'crates/rk-ports/'); [ -z "$cs" ] || { echo "FAIL: ChangeSummary outside rk-ports:"; printf '%s\n' "$cs"; exit 1; }   # → 0. Blocked on S-BIN — lives only as
                                                          #   rk-ports' opaque newtype (ADR-0099 clause 5), and rk-ports is 7a's
test -f Cargo.toml || { echo "FAIL: no root manifest"; exit 1; }
meta=$(cargo metadata --format-version 1); rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: cargo metadata exited $rc"; exit 1; }
printf '%s\n' "$meta" | python -c "import json,sys; m=json.load(sys.stdin); assert m['packages'], 'FAIL: metadata lists no packages'; print(sorted(p['name'] for p in m['packages'] if any(d['name']=='anyhow' for d in p['dependencies'])))"
                                                          # → [] at Step 5 (rk-cli and redkiln-migrate are 7d's); the design §9.1 anyhow convention, as a check
```

Two of those lines changed shape. `! grep -rn 'oneOf' <dir>` **passes when the directory does not exist** — grep
exits 2, `!` inverts it, and a checkpoint asserting "no `oneOf` survives into any exported schema" goes green on a
project with no snapshots at all. The `test -d` guard makes a missing snapshot directory the failure it is. The
`ChangeSummary` line is piped to `wc -l` for the same reason, though there the false pass was benign: the property
being asserted is absence, and a missing `src/` is a different failure the build already catches.

`grep -L` without `-r`: the arguments are an explicit glob of files, so `-r` did nothing.

**Acceptance set:** the typed-artifact criteria of **AC-120–AC-159** that `rk-types` can carry before the store
exists — the schema-design rules **AC-129, AC-130, AC-131, AC-132, AC-133, AC-157, AC-158** (AC-158 verifiable via
`preserve_order`; AC-121's cross-OS half verifiable on the two-OS matrix), **AC-143**'s registry-completeness
fixture over the six kinds decided above; **AC-001–AC-004** on the `clippy.toml` instrument; the CI-borne structural
assertions **AC-005, AC-362**; **AC-229** (`StepError` closed) and the `Findings.repro` ingest fixture of **AC-281**.
**Excluded, by id and reason:** `AC-154` (S-BIN; not evaluable), `AC-497`/`AC-356` (PROVISIONAL on S-BIN;
`ChangeSummary`), `AC-480`, `AC-481`, `AC-483`, `AC-484` (structural assertions over crates that exist only from
Steps 7a/7b — they are 7a/7b's, not this step's, PIA-035), the store-side rows of AC-120–AC-159 (Step 6.3), and
**AC-006/AC-188** (golden-log replay; the logs are Step 7b's output and are re-run there). AC-465 becomes enforceable
here because Step 2 has landed.

---
