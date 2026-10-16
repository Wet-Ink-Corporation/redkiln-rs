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
> ( ls D:/repos/redkiln-rs/references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "GATE OPEN — no ADR-0090. Go back to Step 4; no work in this stage may start."; exit 1; }
>   echo "GATE DISCHARGED — proceed" )   # a subshell: a pasted FAIL must not kill an interactive shell
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
members = ["crates/*", "tests/fixtures/*"]
# Resolver 3, not 2 (corrected 2026-09-12). The MSRV-aware fallback `rust-version` exists to arm is
# resolver 3's; under `resolver = "2"` the `rust-version` line below is inert and ADR-0102's
# PIA-027 clause buys nothing. Edition 2024 defaults to 3, but the workspace root is not an edition
# consumer, so it must be stated here explicitly.
resolver = "3"
# spikes/runner/ is its own package with its own Cargo.toml. If Step 2 has landed it is gone and this
# line is inert; if it has NOT, cargo walks up from spikes/runner/, finds this root, sees that
# members = ["crates/*"] does not cover it, and fails with
#   error: current package believes it's in a workspace when it's not
# — which reads as Halt condition 1 (the rig fails to build) from entirely the wrong direction.
# spikes/s-ports/ is the S-PORTS throwaway crate (F41) and is excluded for the same reason.
# tests/fixtures/* ARE members (§5.1a): the AC-001–AC-004 and AC-005 must-fail fixture crates.
# Each one's offending call or dependency sits behind a NON-DEFAULT feature, so the default
# `cargo clippy --workspace` and `cargo deny check bans` gates stay green and the fixture is still
# checked by the same invocation with its feature on (DECISIONS-2026-09-12 E4).
exclude = ["spikes/runner", "spikes/s-ports"]

[workspace.package]
edition = "2024"
# Equal to the pin, always (ADR-0102): it arms resolver 3's MSRV fallback and `cargo add`; it is NOT a
# consumer floor. The pin-bump PR bumps this and rust-toolchain.toml together.
rust-version = "1.98.1"

# ADR-0100 decision 2's pin list lives HERE and nowhere else; members inherit with `dep.workspace = true`
# (which is the form Checkpoint 6 greps for in rk-git). Before 2026-09-12 this table was absent, so the
# pin list had no home and `gix.workspace = true` could not have resolved.
[workspace.dependencies]
gix        = "=0.87.1"    # exact (ADR-0084, unchanged by ADR-0100); read path only (ADR-0077/0098)
jsonschema = "=0.56.0"    # exact: weekly cadence. Format validation is OPT-IN — call
                          # .should_validate_formats(true) wherever the design relies on format checks
rusqlite   = "=0.40.2"    # exact: carries the one-writer semantics ADR-0105 clause 7 depends on
yaml_serde = "=0.10.7"    # exact; redkiln-migrate ONLY — the one crate that may depend on a YAML reader
chrono     = "^0.4"       # EXEMPT from the 0.x exact-pin rule: de-facto stable, multi-year line, semver discipline
sha2       = "^0.11"      # EXEMPT, same reason
# Any new 0.x dependency is exact-pinned unless its row says why not. A pin bump is a PR that re-runs
# `cargo audit` and `cargo deny` (ADR-0100 decision 2).

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
tables AC-001–AC-004's instrument reads. **Enumerate them here; they are the whole determinism ban and they were
named nowhere in this runbook before 2026-09-12:**

```toml
# clippy.toml — at the workspace root, beside Cargo.toml
disallowed-methods = [
  # AC-001 — the wall clock, a monotonic instant, a timezone-dependent value
  "std::time::SystemTime::now", "std::time::Instant::now", "chrono::Utc::now",
  # AC-002 — randomness and non-deterministic ids (ids derive from the ADR-0097 idempotency key or a content hash)
  "rand::random", "rand::thread_rng", "uuid::Uuid::new_v4",
  # AC-003 — I/O and blocking sleeps on the workflow side
  "std::thread::sleep", "tokio::time::sleep",
  # ADR-0083's ban, workspace-wide: these became `unsafe fn` in edition 2024 and race a concurrent getenv
  "std::env::set_var", "std::env::remove_var",
]
disallowed-types = [
  # AC-004 — unordered iteration in a workflow decision; ordered containers or an explicit total-key sort
  "std::collections::HashMap", "std::collections::HashSet",
  # Checkpoint 6's type ban — git shells out through exactly one wrapper in rk-git
  "std::process::Command", "tokio::process::Command",
]
```

**`clippy.toml` has no path or module scoping**, so every entry above is workspace-wide once declared. That is why
AC-001's Verified-by pairs it with a module-level `#[deny(clippy::disallowed_methods)]` on `rk-kernel::workflow`
(ADR-0083's commented override; ADR-0102) rather than a group deny, and why `rk-git`'s wrapper, the adapters and
`rk-store` carry `#[expect(clippy::disallowed_types, reason = "…")]` where they legitimately need `Command` or a
hash map. AC-362's `unwrap_or(0)` / `unwrap_or_default()` shapes on the usage struct are **not** clippy entries —
they are the CI grep in §5.2's table, because the ban is scoped to specific fields, not to the method.

AC-001–AC-004's Verified-by is a must-fail **fixture crate** checked under `cargo clippy`, and CI is **gated on
the exit code, never `-D warnings`** (which would promote `clippy::nursery` to the group deny ADR-0083 forbids) —
a `trybuild` fixture cannot fail on a lint, only on a compile error, so it is not the instrument here
(both halves amended 2026-09-11, PIA-030/PIA-031).

### 5.1a The two must-fail fixture crates

AC-001–AC-004 and AC-005 are each verified by a fixture that **must fail** its gate. Before 2026-09-12 neither was
named, given a path, or given a home in the workspace anywhere in this runbook — while Step 8 (4) asserted three
things about them. Author both here, by these names:

| Fixture | Path | Reintroduces | Gate it must fail | Criteria |
| --- | --- | --- | --- | --- |
| `rk-clock-read` | `tests/fixtures/rk-clock-read/` | one `SystemTime::now()` call, behind feature `reintroduce` | `cargo clippy` (the `disallowed-methods` table above) | **AC-001**–AC-004 |
| `rk-banned-dep` | `tests/fixtures/rk-banned-dep/` | one of ADR-0086's four banned durable-execution crates, as an **optional** dependency behind feature `reintroduce` | `cargo deny check bans` | **AC-005**, AC-044, AC-120 |

Each is a **workspace member** whose offending call or dependency sits behind a **non-default feature**
(DECISIONS-2026-09-12 E4). That is what lets both halves of the record hold verbatim: ADR-0100 decision 3 calls it a
*"scratch **member**"*, and AC-001's Verified-by wants it *"checked under the **same** `cargo clippy`
invocation"* — which it is, with the feature on. With the feature off the default gates are green:

```bash
cd D:/repos/redkiln-rs
# The DEFAULT gates, which must stay green: the fixtures are members, but their features are off.
cargo clippy --workspace --all-targets || { echo "FAIL: the default clippy gate is red"; exit 1; }
cargo deny check bans || { echo "FAIL: the default cargo-deny gate is red"; exit 1; }
# Each fixture must FAIL with its feature on. Assert the non-zero status AND the message: an unrelated
# failure (a typo, a missing file) would otherwise read as the pass and the fixture would certify nothing.
out=$(cargo clippy --workspace --all-targets --features rk-clock-read/reintroduce 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: rk-clock-read CLIPPED CLEAN with its feature on — the AC-001 ban list is not in force"; exit 1; }
printf '%s\n' "$out" | grep -q 'disallowed_method' \
  || { echo "FAIL: rk-clock-read failed for some OTHER reason than the clock ban"; printf '%s\n' "$out" | tail -20; exit 1; }
echo "AC-001: the clock-read fixture fails on disallowed_methods, as required"
out=$(cargo deny --all-features check bans 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: rk-banned-dep passed check bans — ADR-0086's four banned crates are not banned"; exit 1; }
printf '%s\n' "$out" | grep -qi 'banned' \
  || { echo "FAIL: rk-banned-dep failed for some OTHER reason than the ban"; printf '%s\n' "$out" | tail -20; exit 1; }
echo "AC-005: the banned-dep fixture fails check bans, as required"
```

**One cargo-deny trap this shape carries.** `deny.toml`'s `[graph]` section must **not** set
`all-features = true`, or the banned optional dependency enters the default graph and the everyday
`cargo deny check bans` gate is permanently red — which is how a gate gets deleted rather than fixed. The
default gate resolves with the feature off; the fixture's own check passes `--all-features` explicitly, as
above. Step 8 (4) asserts the same fixture by `-p`, and because these are members its
`cargo metadata --no-deps` membership check resolves — which it would not under an excluded package.

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
*(Amended 2026-09-24, s8a §8a.3, ADR-0165: this paragraph is true of Step 5 and is not true now. ADR-0165 splits
`ChangeSummary` into an enum defined only in `rk-ports`: a `Text` arm (a typed diff plus a manifest) finalised at `s8a`,
and a sealed `Binary` slot that S-BIN fills at `s8i` [S-BIN reported (s8i) 2026-10-14: ADR-0199 (proposed *(dated 2026-10-15, `s8i` close, D21-154: ADR-0199 accepted)*) opens the slot through one checked constructor; the `Binary` arm is filled, and Checkpoint 5's dated notes below record what lapses]. "No value is synthesised" still holds for `redkiln-migrate`
(AC-475). Checkpoint 5's `ChangeSummary` statements below are replaced to match.)*

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

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence:
every one of its `cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins.
Author each named test **before** the code it describes, run it, and watch it fail for the right reason —
the absent behaviour, not a compile error or a filter that matched nothing.

- `step_error_is_total` — `spoke()` over all **eleven** `StepError` variants with no catch-all arm, and one event
  arm each (ADR-0095 as ADR-0112 extends it). Red against the six-variant `step.rs` shape this crate does **not**
  carry over: it must fail to compile on the five new variants, which is the discrimination.
- `no_oneof_at_any_node` and `union_and_optional_caps` — assert over **every** exported schema that no `oneOf`
  survives the post-derive transform, that no union nests inside a union, that `additionalProperties: false` holds
  at every object level, and that the ≤ 16 union-typed / ≤ 24 optional caps hold per schema (AC-131, AC-157).
  Red by deriving a tagged enum **without** the transform: `schemars` 1.2.2 renders `oneOf` (PIA-005), so the
  untransformed derive is the mutant and the test must fail on it.
- `repro_rejects_prose_only` — an ingest fixture whose `command` is prose is **rejected**, never routed to
  advisory (ADR-0096). Red before the typed `Findings.repro` exists.
- `companion_kind_has_registered_schema` — AC-143's fixture enumerates exactly the six kinds; a seventh must fail
  to load. Red by adding a seventh with no registered sidecar schema.

The `schema_snapshot` insta test is the **exception that proves the rule**: a golden snapshot has no meaningful red
state, which is why its first acceptance is a deliberate one-time rubber-stamp (below) and why it pins *change*
rather than *correctness*. It is not evidence that a criterion is discriminated; the four tests above are.

**Checkpoint 5.** All of:

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# BEFORE assertions — each fails on the untouched tree (PIA-035); run them first and expect the stated failure
test ! -f rust-toolchain.toml && echo "BEFORE: no pin yet"           # → prints on the untouched tree; after the step, silent
test ! -d .github/workflows && echo "BEFORE: no CI yet"               # → prints on the untouched tree
# AFTER assertions
cargo build --workspace || { echo "FAIL: cargo build --workspace"; exit 1; }   # builds, on the pinned toolchain (rust-toolchain.toml selects it)
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check"; exit 1; }   # exit 0
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"; [ "$rc" -eq 0 ] || exit 1   # → exit=0; nursery WARNINGS are permitted, the gate is the exit code (PIA-030)
out=$(cargo test --workspace 2>&1); rc=$?                 # exit 0 — the corpus-bound suite is behind --features corpus
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }
test -f CORPUS_PIN && test -d D:/repos/redkiln-corpus || { echo "FAIL: corpus absent — Step 2 has not landed, or redkiln-corpus is not checked out on this box"; exit 1; }
out=$(cargo test --workspace --features corpus 2>&1); rc=$?   # not a CI line: it runs only on a box holding redkiln-corpus at CORPUS_PIN
[ "$rc" -eq 0 ] || { echo "FAIL: corpus suite exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: 0 corpus tests ran"; exit 1; }
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }   # exit 0
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses"; exit 1; }   # exit 0 (deny.toml committed)
test -f rust-toolchain.toml && grep -q 'channel = "1.98.1"' rust-toolchain.toml || { echo "FAIL: rust-toolchain.toml absent, or not pinned to the FULL string 1.98.1"; exit 1; }   # the FULL string (PIA-024), not a prefix
grep -q 'rust-version = "1.98.1"' Cargo.toml || { echo "FAIL: rust-version is not equal to the pin"; exit 1; }   # equal to the pin (ADR-0102)
grep -q 'exclude = \["spikes/runner", "spikes/s-ports"\]' Cargo.toml || { echo "FAIL: the exclude line is absent or was rewritten"; exit 1; }   # the exclude survived the first `cargo new` (PIA-035)
test -f .github/workflows/ci.yml && grep -q 'windows-latest' .github/workflows/ci.yml && grep -q 'ubuntu-latest' .github/workflows/ci.yml || { echo "FAIL: ci.yml absent, or the two-OS matrix does not name both"; exit 1; }
test -f clippy.toml && grep -q 'disallowed-methods' clippy.toml || { echo "FAIL: clippy.toml absent, or carries no disallowed-methods table"; exit 1; }
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
# ChangeSummary — [amended 2026-09-24, s8a §8a.3, ADR-0165] THE STATEMENTS BELOW REPLACE THIS CHECKPOINT'S OLD
# ChangeSummary LINES. ADR-0165 supersedes in part ADR-0099 clause 5 and its forbids bullet, ADR-0135 decisions 4-5
# and its third and fourth forbids bullets, ADR-0093 decision 4, ADR-0094 decision 1 and ADR-0008: ChangeSummary is an
# enum defined ONLY in rk-ports, with a finalised Text arm and a sealed Binary slot that S-BIN fills at s8i. An
# implementor of Toolchain::change_summary, test doubles included, may import the type, name it in that method's
# signature and construct the Text arm. Nothing outside rk-ports defines, aliases, renames or implements a trait for any
# …ChangeSummary type, and rk-kernel's production code names it nowhere (ADR-0135's surviving forbids). The old line
# banned the bare NAME outside rk-ports, minus ::ChangeSummary paths, and was RED at ca8f387 on six sites (eight
# literal ones) that ADR-0094 decision 1 and D17-9 REQUIRE; it is not carried forward. The ::ChangeSummary exclusion is
# DROPPED: a path-qualified definition, alias or impl fires like a bare one. String literals are blanked and comment
# lines dropped BEFORE matching, as before (readings 2 and 3 of PROGRESS §1 row 5 stand for the vocabulary variant).
# These lines are Checkpoint 8a guard (6)'s, copied from its bytes: the same CS_BAN, the same filter, the same
# controls, with target/s8a-ctl re-homed to target/cp5-ctl. Stated limit: the ban reads one line at a time, so an impl
# split with "for …ChangeSummary" on its own line is not seen; cargo fmt --check above holds the tree to rustfmt's
# one-line form. The opacity pins are floors; the behavioural pin is the trybuild case
# change_summary::binary_arm_is_not_constructible_outside_rk_ports. The sealed-slot pins are SUPERSEDED BY NAME at s8i.
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* Superseded now: S-BIN has reported (F43, proposed), and ADR-0199 (proposed) opens the slot.
# The behavioural pin named above and the no-pub-field opacity pin below are replaced by the record's construction
# rule, carried by the trybuild case ui::trybuild_pins_the_binary_change_summary_construction_rule (after the struck
# lines below). "With a sealed Binary slot that S-BIN fills at s8i" above lapses here: S-BIN has filled it.
# [amended 2026-09-25, s8a fix round 3] Stated limit, corrected: the ban reads one PHYSICAL line at a time, and rustfmt
# wraps any definition wider than its width, so fmt --check does NOT hold the tree to a one-line form for those. Measured
# 2026-09-25: rustfmt writes a long alias as `pub type X =` then `    Result<…ChangeSummary, …>;`, and a long impl
# header with `    for ChangeSummary` on its own line; the ban sees neither, alias or impl. A line-joined (statement)
# reading is the fix, owed and not taken here.
mkdir -p target/cp5-ctl || { echo "FAIL: cannot make target/cp5-ctl for the sealed-slot reader"; exit 1; }
# [amended 2026-09-25, s8a fix round 1] The rename arm fired only on a capital or _ after `as`, so a lowercase
# renaming import (`use …::ChangeSummary as summary;`, `{ChangeSummary as cs, …}`) passed, and rustc -D warnings
# accepts one. Two arms are added: a renaming import on a `use` line, whatever the alias's case, and a line made only
# of import-list items one of which renames …ChangeSummary. Controls 20-23 fire; 19 and 24 (numeric casts) are
# admitted. Stated limit: a lowercase cast that is the whole of a list-item line (`X::ChangeSummary as u8,`) fires.
# [amended 2026-09-25, s8a fix round 13] The impl arm took only a (path-qualified) name straight after `for`,
# so `impl Render for &ChangeSummary`, `for &'a rk_ports::toolchain::ChangeSummary`, `for Box<ChangeSummary>` and
# `for Option<…BinaryChangeSummary>` passed, and each compiles outside rk-ports under rustc -D warnings. The arm
# now takes any text between `for` and the name on the line; no admitted control carries `impl`. Controls 27 and 28
# (a reference and a Box<…>) fire. The one-physical-line limit above is unchanged.
# [amended 2026-09-25, s8a fix round 14] The definition arm and the `impl … for` arm matched the name as
# [A-Za-z_]*ChangeSummary, so a raw identifier (`pub struct r#ChangeSummary;`, a type literally named
# ChangeSummary) and a digit in the prefix (`pub struct V2ChangeSummary;`, `impl Clone for Sha1ChangeSummary {`)
# passed, and each compiles under rustc -D warnings. Both arms now take [A-Za-z0-9_]*, and the definition arm an
# optional r# prefix. Controls 29, 30 and 31 fire.
# *(dated 2026-10-14, s8i build, S8I-D3-RECORDS; s8i section 8i.8, "Three lines whose condition lapses here")* S-BIN has
# reported (F43, proposed; ADR-0199, proposed; nothing flipped). Section 8i.8 amends this line "if s8a's record kept
# that condition" ("until S-BIN reports", ADR-0099 clause 5's words). It did not: ADR-0165 decision 9 supersedes, in
# part, ADR-0099 clause 5's sentence and its forbids bullet "Forbids `ChangeSummary` appearing anywhere in `crates/` but
# its `rk-ports` newtype until S-BIN reports.", and its decision 2 replaces them with no time condition. So nothing in
# CS_BAN or the rk-kernel ban below lapses, and both stand unchanged. The render (rk-store) and review-input (rk-cli)
# sites name the type and define, alias, rename or implement nothing: these lines, cut from this file's bytes and run
# read-only over the working tree after s8i fix round D, pass, and fail on a planted alias in rk-store/src and on a
# planted type use in rk-kernel/src. Only "a sealed Binary slot that S-BIN fills at s8i" lapses (S8I-C4-PORTS's note).
CS_BAN='\b(struct|enum|type|trait|union)[[:space:]]+(r#)?[A-Za-z0-9_]*ChangeSummary\b|\btype[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[^=]*=.*\b[A-Za-z_]*ChangeSummary\b|\bimpl\b.*\bfor[[:space:]]+.*\b[A-Za-z0-9_]*ChangeSummary\b|\bimpl(<[^>]*>)?[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Z_]|\buse\b[^;]*\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_]|^[^:]+:[0-9]+:[[:space:]]*([A-Za-z_][A-Za-z0-9_:]*([[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*)?[[:space:]]*,[[:space:]]*)*([A-Za-z_][A-Za-z0-9_]*::)*[{]?[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(,.*)?$'
# [amended 2026-09-25, s8a fix round 5] The string blank was `s/"([^"\]|\.)*"/""/g`: in ERE a backslash inside a
# bracket is literal and `\.` is a literal dot, so a string holding a backslash escape ("a\n") was never blanked and
# its closing quote paired with the NEXT string on the line, blanking the code between (a ChangeSummary impl, a
# diff_tree_to_tree call). It is now escape-aware, `[^"\\]|\\.` (a non-quote non-backslash, or a backslash and any
# byte), as sealed.py's \\[\s\S] has been since fix round 1. Planted controls carry a backslash-escaped string.
cs_filter() { grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -E "$CS_BAN"; }
raw=$(grep -rn 'ChangeSummary' crates/ --include='*.rs'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ChangeSummary search found nothing or could not run (rc=$rc) — rk-ports' own enum must be seen"; exit 1; }
hits=$(printf '%s\n' "$raw" | cs_filter)
[ -z "$hits" ] || { echo "FAIL: a ChangeSummary definition, alias or impl outside rk-ports:"; printf '%s\n' "$hits"; exit 1; }
for ctl in 'F|x.rs:1:pub struct ChangeSummary;' 'F|x.rs:2:pub enum TextChangeSummary { A }' 'F|x.rs:3:type Cs = rk_ports::toolchain::ChangeSummary;' 'F|x.rs:4:impl Clone for ChangeSummary {' 'F|x.rs:5:impl rk_ports::toolchain::ChangeSummary {' 'F|x.rs:6:impl<T> From<T> for rk_ports::toolchain::BinaryChangeSummary {' 'F|x.rs:7:    let s = "x"; impl Copy for ChangeSummary {}' 'P|x.rs:8:use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};' 'P|x.rs:9:    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {' 'P|x.rs:10:    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {' 'P|x.rs:11:            Self::ChangeSummary => "change_summary",' 'P|x.rs:12:    ChangeSummary,' 'P|x.rs:13:        Ok(ChangeSummary::Text(text))' 'P|x.rs:14:    // impl Clone for ChangeSummary {}' 'P|x.rs:15:    let k = "impl Clone for ChangeSummary";' 'F|x.rs:16:use rk_ports::toolchain::ChangeSummary as Cs;' 'F|x.rs:17:    ChangeSummary as Summary,' 'F|x.rs:18:use rk_ports::toolchain::{ChangeSummary as _, Toolchain};' 'P|x.rs:19:    let n = Capability::ChangeSummary as usize;' 'F|x.rs:20:use rk_ports::toolchain::ChangeSummary as summary;' 'F|x.rs:21:use rk_ports::toolchain::{ChangeSummary as cs, Toolchain};' 'F|x.rs:22:    ChangeSummary as summary,' 'F|x.rs:23:    toolchain::{ChangeSummary as cs, Toolchain},' 'P|x.rs:24:        let n = kind as usize + Capability::ChangeSummary as usize;' 'F|x.rs:25:    const A: &str = "a\n"; impl Clone for ChangeSummary { fn n() -> &str { "z" } }' 'P|x.rs:26:    let k = "a\"impl Clone for ChangeSummary\"";' 'F|x.rs:27:impl Render for &ChangeSummary {' 'F|x.rs:28:impl Render for Box<rk_ports::toolchain::ChangeSummary> {' 'F|x.rs:29:pub struct r#ChangeSummary;' 'F|x.rs:30:pub struct V2ChangeSummary;' 'F|x.rs:31:impl Clone for Sha1ChangeSummary {'; do
  want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | cs_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: control must FIRE and did not — the ban is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: control must be ADMITTED and fired — the ban blocks what the record licenses: $line"; exit 1; }; fi
done
kraw=$(grep -rn 'ChangeSummary' crates/rk-kernel/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-kernel search could not run (rc=$rc)"; exit 1; }
k_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g' | grep -E '\bChangeSummary\b'; }
khits=$(printf '%s\n' "$kraw" | k_filter)
[ -z "$khits" ] || { echo "FAIL: rk-kernel production code names the ChangeSummary type (ADR-0135's surviving forbids):"; printf '%s\n' "$khits"; exit 1; }
kctl=$(printf 'crates/rk-kernel/src/x.rs:1:fn f(s: &str) -> Option<ChangeSummary> { None }\n' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire"; exit 1; }
kctl=$(printf '%s\n' 'crates/rk-kernel/src/x.rs:2:    let r = check("a\n", ChangeSummary::Text(t), "z");' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire past a backslash-escaped string"; exit 1; }
nenum=$(grep -cE '^pub enum ChangeSummary[[:space:]]*\{' crates/rk-ports/src/toolchain.rs)
[ "$nenum" -eq 1 ] || { echo "FAIL: $nenum 'pub enum ChangeSummary {' in rk-ports, expected 1 — the split did not land"; exit 1; }
# [amended 2026-09-25, s8a fix round 1] The string blank below reads a backslash-escape as \\[\s\S], as Checkpoint 8a
# guard (6)'s sealed.py has since 2026-09-25; this copy kept \\. and so was not "copied from its bytes" until now.
# [amended 2026-09-25, s8a fix round 6] ONE left-to-right pass strips comments and blanks strings, so a `//`
# inside a string (a doc URL) no longer truncates the code after it; and the pub counter skips only a real
# restriction, pub(crate|self|super|in …), where `(?!\s*\()` also skipped `pub ()` and `pub (u8, u8)` — the
# direct way to open the slot read pubfields=0. Four more controls pin both; Checkpoint 8a guard (6) carries the same reader.
cat > target/cp5-ctl/sealed.py <<'PY'
import re, sys
src = open(sys.argv[1], encoding="utf-8").read().replace(chr(13), "")
src = re.sub(r'//[^\n]*|/\*.*?\*/|"(?:[^"\\]|\\[\s\S])*"', lambda m: '""' if m.group(0)[0] == '"' else '', src, flags=re.S)
decls = list(re.finditer(r"\bstruct\s+BinaryChangeSummary\b", src))
pub = 0
for m in decls:
    i = m.end()
    if src[i:].lstrip().startswith("<"):
        i = src.index(">", i) + 1
    rest = src[i:].lstrip()
    if rest and rest[0] in "({":
        close, depth, j = {"(": ")", "{": "}"}[rest[0]], 0, 0
        for j, ch in enumerate(rest):
            depth += ch in "({[" and 1 or 0
            depth -= ch in ")}]" and 1 or 0
            if depth == 0:
                break
        pub += len(re.findall(r"\bpub\b(?!\s*\(\s*(?:(?:crate|self|super)\s*\)|in\s))", rest[1:j]))
print("SEALED decls=%d pubfields=%d" % (len(decls), pub))
PY
sealed() { python target/cp5-ctl/sealed.py "$1" 2>&1; }
# ~~out=$(sealed crates/rk-ports/src/toolchain.rs); rc=$?~~
# ~~[ "$rc" -eq 0 ] || { echo "FAIL: the sealed-slot reader could not run (rc=$rc): $out"; exit 1; }~~
# ~~[ "$out" = "SEALED decls=1 pubfields=0" ] || { echo "FAIL: rk-ports' sealed Binary payload reads '$out', expected one declaration with no pub field — none means the pin passes over nothing, a pub field means it is constructible"; exit 1; }~~
# ~~for ctl in 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary { pub bytes: u64 }' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub u64);' 'SEALED decls=1 pubfields=0|// pub struct BinaryChangeSummary(pub u64);  pub struct BinaryChangeSummary(pub(crate) ());' 'SEALED decls=1 pubfields=0|pub struct BinaryChangeSummary { bytes: u64, note: Vec<u8> }' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub ());' 'SEALED decls=1 pubfields=1|pub struct BinaryChangeSummary(pub (u8, u8));' 'SEALED decls=1 pubfields=0|pub struct BinaryChangeSummary(pub(in crate::toolchain) ());' 'SEALED decls=1 pubfields=1|#[doc = "spec: https://git-lfs.com"] pub struct BinaryChangeSummary { pub bytes: () }'; do~~
# ~~  printf '%s\n' "${ctl#*|}" | sed 's/  pub struct/\npub struct/' > target/cp5-ctl/sealed_ctl.rs || { echo "FAIL: cannot write a sealed-slot control"; exit 1; }~~
# ~~  out=$(sealed target/cp5-ctl/sealed_ctl.rs); rc=$?~~
# ~~  [ "$rc" -eq 0 ] && [ "$out" = "${ctl%%|*}" ] || { echo "FAIL: CONTROL — the sealed-slot reader read '$out' for: ${ctl#*|} (want ${ctl%%|*})"; exit 1; }~~
# ~~done~~
# [amended 2026-09-25, s8a fix round 1] A fifth sealed-slot control: a line-continued string BEFORE the declaration,
# with a later string. Under the old string blank (\\. for an escape) the quotes re-pair across the declaration and
# the reader reports decls=0, losing the pub field that makes the payload constructible; this pins BOTH copies of
# the reader (Checkpoint 8a guard (6) and Checkpoint 5), which cs_cont.rs below does not reach.
# [amended 2026-09-25, s8a fix round 10] Since fix rounds 6 and 7 the reader is ONE re.sub pass under re.S, and
# under re.S `.` matches a newline, so the escape arms \\. and \\[\s\S] are the same pattern: reverting the
# blank alone leaves this control GREEN (measured: \\. with re.S reads decls=1 pubfields=1; [\s\S] without
# re.S reads the same). It goes red only when the escape arm is \\. AND re.S is dropped (decls=0 pubfields=0).
# So it pins that pair, not the blank; no input can tell \\. from \\[\s\S] under re.S, so no single-point
# control on the escape arm exists.
# ~~printf '%s\n' '#[error("a message \' '   continued")]' 'pub struct BinaryChangeSummary(pub u64);' 'const LATER: &str = "later";' > target/cp5-ctl/sealed_cont.rs || { echo "FAIL: cannot write the continued-string sealed-slot control"; exit 1; }~~
# ~~out=$(sealed target/cp5-ctl/sealed_cont.rs); rc=$?~~
# ~~[ "$rc" -eq 0 ] && [ "$out" = "SEALED decls=1 pubfields=1" ] || { echo "FAIL: CONTROL — a line-continued string before the declaration hid it from the sealed-slot reader: '$out' (want SEALED decls=1 pubfields=1)"; exit 1; }~~
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* THE PIN THAT REPLACES s8a section 8a.3's opacity pin ("the sealed payload type declares no
# `pub` field", struck above; s8i section 8i.8; ADR-0199 decision 2, proposed): the record's construction rule, carried
# by the trybuild case ui::trybuild_pins_the_binary_change_summary_construction_rule. Its two cases each commit a .stderr: a struct literal
# fails on private fields (E0451), and a field read (E0616), Default, From<u8> and Deserialize each fail by name, so a
# pub field or a second route changes a .stderr and the test goes red. The ChangeSummary ban above is unchanged.
# ADR-0165 had already dropped its "until S-BIN reports" condition (ADR-0099 clause 5's words), so nothing else lapses.
# Two controls: a selection that ran nothing, and a FAILED case, must not read as the pin passing.
for f in crates/rk-ports/tests/ui/change_summary_binary_construction_rule crates/rk-ports/tests/ui/change_summary_binary_struct_literal; do
  test -s "$f.rs" && test -s "$f.stderr" || { echo "FAIL: $f.rs or its committed .stderr is absent or empty — a trybuild case without a .stderr is green whenever compilation fails for ANY reason (s7a section 7a.8)"; exit 1; }
done
cs_rule_ok() { printf '%s\n' "$1" | grep -qxF 'test ui::trybuild_pins_the_binary_change_summary_construction_rule ... ok' && printf '%s\n' "$1" | grep -qE '^test result: ok\. 1 passed; 0 failed'; }
out=$(cargo test -p rk-ports --test it -- ui::trybuild_pins_the_binary_change_summary_construction_rule --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] && cs_rule_ok "$out" || { echo "FAIL: the construction-rule trybuild case did not run and pass exactly once (ADR-0199 decision 2)"; printf '%s\n' "$out" | tail -20; exit 1; }
! cs_rule_ok 'test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 28 filtered out' || { echo "FAIL: CONTROL — a selection that ran nothing reads as the pin passing"; exit 1; }
! cs_rule_ok 'test ui::trybuild_pins_the_binary_change_summary_construction_rule ... FAILED' || { echo "FAIL: CONTROL — a FAILED case reads as the pin passing"; exit 1; }
test -f Cargo.toml || { echo "FAIL: no root manifest"; exit 1; }
meta=$(cargo metadata --format-version 1 --no-deps); rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: cargo metadata exited $rc"; exit 1; }
printf '%s\n' "$meta" | python -c "import json,sys; m=json.load(sys.stdin); assert m['packages'], 'FAIL: metadata lists no packages'; allowed={'rk-cli','redkiln-migrate'}; bad=sorted(p['name'] for p in m['packages'] if p['name'] not in allowed and any(d['name']=='anyhow' for d in p['dependencies'])); print(bad); sys.exit(1 if bad else 0)" \
  || { echo "FAIL: a workspace MEMBER outside {rk-cli, redkiln-migrate} depends on anyhow"; exit 1; }
                                                          # → [] at Step 5 (rk-cli and redkiln-migrate did not exist) and → [] from Step 7d Slice 1 on.
                                                          # AMENDED 2026-09-19, at Step 7d Slice 1, by the operator's decision, and the line is STALE
                                                          # rather than wrong (s7d §7d.1's rule: "A record that overturns one of those routings makes
                                                          # its floor line stale, not wrong: the line is re-pointed in the same commit as the record's
                                                          # code, the way Checkpoint 5's ChangeSummary line was (D17-1)"). As written it asserted that
                                                          # NO member depends on anyhow, and its own comment already anticipated the two that would —
                                                          # so the first slice of 7d to create crates/rk-cli made it unsatisfiable, while Checkpoint 7d's
                                                          # own manifest loop asserts the opposite: that the anyhow row is PRESENT on both. The authority
                                                          # is ADR-0102 decision 6, an ACCEPTED record: "thiserror in library crates, anyhow confined to
                                                          # rk-cli and redkiln-migrate". The allowlist is exactly those two names and is not a widening:
                                                          # a third member taking anyhow is still red here, which is the whole of what the convention says.
                                                          # → [] at Step 5 (rk-cli and redkiln-migrate are 7d's); the design §9.1 anyhow convention, as a check.
                                                          # --no-deps is load-bearing: without it the walk is the whole transitive
                                                          # graph, not the members the convention is about (golden constraint 9),
                                                          # and the line PRINTED the offending list instead of asserting on it —
                                                          # Halt 8 shape (g), a pass condition that is the absence of output.
                                                          # Both corrected 2026-09-13 at Step 5's run.
```

Two of those lines changed shape. `! grep -rn 'oneOf' <dir>` **passes when the directory does not exist** — grep
exits 2, `!` inverts it, and a checkpoint asserting "no `oneOf` survives into any exported schema" goes green on a
project with no snapshots at all. The `test -d` guard makes a missing snapshot directory the failure it is. The
`ChangeSummary` line captures into `cs=$(…)` and tests `-z` for the same reason, though there the false pass was
benign: the property being asserted is absence, and a missing `src/` is a different failure the build already
catches. *(Corrected 2026-09-13: this sentence said the line was piped to `wc -l`, which it has not been.)*
*(Amended 2026-09-24, s8a §8a.3, ADR-0165: the `cs=$(…)` line is gone. Its successor captures into `hits=$(…)` and
tests `-z` the same way, and the positive half is now the raw search's demanded rc 0, the enum pin and the controls.)*

`grep -L` without `-r`: the arguments are an explicit glob of files, so `-r` did nothing.

**Re-counted 2026-09-17, after the `ChangeSummary` line was re-scoped to the TYPE (PROGRESS §1 row 5, reading 2).**
Counted mechanically off the block — backslash continuations joined, comments and blanks dropped, a statement that
*can stop the block* being one carrying its own `exit 1` (the unit `s7b-ii` §7b-ii.13 fixes) — this block now carries
**39 statements, of which 31 can stop the block and 8 cannot**: the 2 BEFORE readings and 6 `out=`/`n=`/`raw=`/`cs=`/
`ctl=` capture halves whose guard is the statement beneath them. The same counter reads the previous text at
**33 / 28 / 5**, which is the figure the paragraph below recorded and the calibration that makes this one comparable.
The one line that changed was replaced by nine statements (a split pipe, a filter, the ban, two positive controls),
and each of the three FAIL shapes was exercised before the count was taken: silent on the tree; red on a scratch
type-position use beside a string literal; red on a renamed newtype (each control by its own message).
*(Amended 2026-09-24, s8a §8a.3, ADR-0165: these counts describe the text before the amendment and are not
re-counted here. The `ChangeSummary` statements are now Checkpoint 8a guard (6)'s, copied from its bytes: the new ban
(nineteen controls, ten that fire and nine admitted), the `rk-kernel` production ban with its control, and the two
opacity pins with four planted controls. Measured 2026-09-24: RED at `b8e7aa9` on the enum pin alone (`FAIL: 0 'pub
enum ChangeSummary {' in rk-ports, expected 1 — the split did not land`), while the ban itself passes there on the
six sites the old line fired on. Planted mutants each go red: a path-qualified alias, a renaming import, an
`rk-kernel/src` use and a `pub` field on the sealed payload.)* *(Amended 2026-09-25, s8a fix round 1: the
sealed-slot reader's string blank is Checkpoint 8a guard (6)'s `\\[\s\S]` again (this copy had kept `\\.`), the ban
has twenty-four controls (fourteen fire, ten admitted: lowercase renaming imports on a `use` line and on an
import-list line now fire), and a fifth sealed-slot control, a line-continued string before the declaration, pins
the reader; reverting either copy's blank turns it red.)* *(Amended 2026-09-25, s8a fix round 6, recording fix
round 5's controls, which the note above predates: the ban has twenty-six controls (fifteen fire, eleven
admitted), the pair added at round 5 sitting past a backslash-escaped string; the `rk-kernel` production ban has
two controls, the second past a backslash-escaped string. At round 6 the sealed-slot reader strips comments and
blanks strings in one pass and counts `pub ()` and `pub (u8, u8)` as pub fields, with eight planted controls
beside the continued-string one; reverting either half turns it red.)* *(Amended 2026-09-25, s8a fix round 10:
"reverting either copy's blank turns it red" no longer holds, and "either half" does not reach the escape arm.
Since rounds 6 and 7 the one pass runs under `re.S`, where `\\.` and `\\[\s\S]` are the same pattern, so the
continued-string control stays green when the blank alone is reverted, and when `re.S` alone is dropped. It goes
red only when both are reverted together (measured: `decls=0 pubfields=0`). No input separates the two escape
arms under `re.S`, so no single-point control on the arm exists; the control pins the pair.)* *(Amended
2026-09-25, s8a fix round 13: the `impl … for` arm now takes any text between `for` and the name, so a
reference or a `Box<…>` after `for` fires; the ban has twenty-eight controls (seventeen fire, eleven admitted).)*
*(Amended 2026-09-25, s8a fix round 14: the definition arm takes an optional `r#` and a digit in the name, and
the `impl … for` arm a digit, so `pub struct r#ChangeSummary;`, `pub struct V2ChangeSummary;` and
`impl Clone for Sha1ChangeSummary {` fire; the ban has thirty-one controls (twenty fire, eleven admitted).)*

**Hardened 2026-09-13, at Step 5's own run.** Counted mechanically — backslash continuations joined, comments and
blanks dropped — this block carried **33 statements**, of which **17** could stop execution and **16** could not.
**Eleven** of those sixteen were AFTER assertions and are now guarded: five silent `grep -q` lines (the `channel`,
`rust-version`, `exclude`, `ci.yml` and `clippy.toml` lines, which print **nothing** on failure), four loud but
non-stopping gates (`cargo build`, `cargo fmt`, `cargo audit`, `cargo deny`), the `cd`, and the `anyhow` line above.
The count is now **28 guarded of 33**, re-derived mechanically rather than eyeballed — a first pass wrote "27 of 33"
by tallying the ten of paragraph (a) and forgetting the `anyhow` line, which is the same eyeballing defect this
block exists to stop. The five that remain unguarded are the two BEFORE readings (PIA-035 prints, not assertions)
and three `out=$(…); rc=$?` / assignment halves whose guard is the statement immediately below them. These lines
passed `check-checkpoints.py` before this pass because that checker lints three shapes — COUNT-ONLY, ECHO-GUARD
and PIPE-STATUS — and an unguarded `grep -q` is none of them. **Silence is not a pass.**

**What the two `--features corpus` lines attest at *this* step, stated because they do not attest what they look
like they attest.** Step 5 has **zero** corpus-bound tests — every criterion in its acceptance set is local, and
the corpus-bound ids of §5.2 first appear at Step 6. `rk-types` declares `[features] corpus = []` so the feature
**exists** (without it `cargo test --workspace --features corpus` exits **101** with *"none of the selected
packages contains these features"*, and this checkpoint could not pass — measured 2026-09-13). The
`test result: ok. N passed` grep on that run therefore matches the ordinary unit tests, not a corpus read. At
Step 5 the pair attests **the workspace suite is green with the feature on**; `s6` Checkpoint 6 is where the same
shape first attests that the bundle was read. Do not report it here as corpus coverage.

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
