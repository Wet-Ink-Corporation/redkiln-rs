---
id: "0083"
title: Rust workspace, toolchain and lint policy
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §2 (Rust practice), §3 (Rust practice, incl. direct repository inspection), §4 rec 8, §5 row "authoritative guidance for a ~12-crate workspace"
supersedes: []
superseded_by: []
related: ["0076", "0084"]
evidence: []
consumers: []
---

## Context and problem

The build has not started, and the only Rust in the repository is `spikes/runner`. Direct repository inspection during the external pass found, in the pass's own severity order: **there is no `.github/workflows` directory at all** — zero automated gate on formatting, lints or tests; **there is no `rust-toolchain.toml`** anywhere, so builds float on whatever rustup default a contributor happens to have installed, and edition-2024 features can silently break; **there is no `[lints]` table and no `clippy.toml`**, so there is no lint policy of any kind; and there is no `tests/` directory anywhere, with 17 files using inline `#[cfg(test)] mod tests` — correct as a default, but leaving no integration-level test against any public API (evidence-augmentation §3, Rust practice, direct inspection, **no URL**).

`spikes/runner/Cargo.toml` does carry `edition = "2024"`, so the edition is settled by demonstration rather than by argument.

Redkiln-rs's target shape is roughly a dozen crates — a kernel, a binary, and one crate per adapter — and the external pass is explicit that **no authoritative guide exists for that exact layout** (§5). Cargo's feature-unification mechanics are well documented; "best practice for a ~12-crate one-binary per-adapter-crate workspace" is practitioner synthesis. This record is therefore synthesis, and says so.

The decision has to be taken now rather than at first commit, because the toolchain pin, the lints table and the CI job are the three things that are cheap on day one and expensive to retrofit across twelve crates.

## Decision drivers

- Hygiene defaults set before the first crate exists cost nothing; the same defaults applied to twelve existing crates cost a churn commit and a review that nobody wants.
- The product is **one binary plus internal crates**, not a published library. `rust-toolchain.toml` (a pin) and `package.rust-version` (a floor) answer different questions and are not substitutes; the pin is the mechanism that fits a binary you control ([blog] swatinem.de; [1P] RFC 3537).
- `clippy::correctness` is the only deny-by-default group. `pedantic` and `nursery` are documented as false-positive-prone and, for nursery, unstable — nursery lints can change or be removed without notice. A blanket `deny(clippy::pedantic)` or `deny(clippy::nursery)` across twelve crates is a CI break waiting on the next clippy bump ([1P] Clippy Lints book).
- Workspace-wide lint policy in one place is what makes divergence visible: `[workspace.lints.*]` at the root with each member declaring `[lints] workspace = true`, and per-crate `#![allow]`/`#![deny]` reserved for a deliberate, visible override ([1P] Cargo Book "Lints"; rust-lang/cargo #13157).
- `std::env::set_var` and `remove_var` became `unsafe fn` in edition 2024 because POSIX `setenv` is not thread-safe and races a concurrent `getenv` — directly implicating `cargo test`'s default multi-threaded harness ([1P] Rust Edition Guide; rust-lang/rust PR #124636). Configuration that arrives through the environment is therefore a test-flakiness source by construction.
- One integration-test **file** is one integration-test **binary**, each relinking the whole stack. matklad measured consolidating Cargo's own suite at **3× less build time and 5× smaller artifacts** ([blog] matklad.github.io, confirmed in §2).
- The kernel is sync and depends on no runtime (0076), which is a property a lint can partly hold: forbidding `unsafe` in the core and keeping the dependency surface visible per crate.

## Considered options

### Option — `#![deny(...)]` in every `lib.rs`

The pre-`[workspace.lints]` convention. Rejected: with twelve crates it puts the policy in twelve places, makes a divergence invisible until someone diffs two files, and offers no way to see at a glance which crate has an exception and why.

### Option — deny `clippy::pedantic` (and/or `nursery`) as groups

Rejected on first-party documentation. Both groups are documented as false-positive-prone, and nursery lints may change or be removed without notice — so a clippy upgrade breaks CI for reasons unrelated to the code. Denying groups also trains contributors to sprinkle `#[allow]`, which is worse than not denying.

### Option — `package.rust-version` floor instead of a toolchain pin

Rejected: a floor tells a *consumer* the minimum version that will compile the crate. Nobody consumes these crates. The question here is "which rustc built this binary", and only the pin answers it.

### Option — one integration test file per feature, the Rust-by-Example default

Rejected on the measured cost: each file is a separate binary relinking the whole stack, and consolidating Cargo's own suite cut build time 3× and artifact size 5×. Test *organisation* is preserved with modules inside one binary at no cost.

### Option — configuration through environment variables

The obvious shape for a CLI, and how the spike rig works today. Rejected: `set_var` is `unsafe` in edition 2024 precisely because it races `getenv` across threads, and the default test harness is multi-threaded — so any test that sets an environment variable is a latent flake against any test that reads one. It also makes the kernel's inputs invisible to the event log, which is a determinism problem (0076) as well as a testing one.

## Decision

**Edition.** Every crate in the workspace is `edition = "2024"`, set once as a workspace-level package field and inherited. Already proven by `spikes/runner`.

**Toolchain.** A root `rust-toolchain.toml` pins an **exact** rustc version (a concrete `channel = "1.NN.N"`, never `stable`) and declares the components CI uses — at minimum `rustfmt` and `clippy`. Bumping the pin is a deliberate PR with its own review, and that review is the named consumer for the async-stabilisation check registered in 0076. `package.rust-version` is not used as a substitute.

**Lints.** `[workspace.lints.rust]` and `[workspace.lints.clippy]` live in the root manifest; every member crate carries `[lints] workspace = true` and nothing else. Within that table:

- `clippy::all = "deny"`.
- A **curated, explicitly enumerated subset of `pedantic` at deny** — never the group. The initial subset is `semicolon_if_nothing_returned`, `explicit_iter_loop`, `match_same_arms`, `redundant_closure_for_method_calls`, `items_after_statements`. Adding a lint to the subset is a routine PR; the subset is a list, not a group name.
- `clippy::nursery = "warn"` only, on the documented grounds that nursery lints can change or be removed without notice.
- `unsafe_code = "deny"` at the workspace level. A crate that genuinely needs it overrides in its own manifest's `[lints]` with a comment naming why — the point of the workspace table is that the exception is then the only thing that looks different.

A per-crate `#![allow]`/`#![deny]` in source is permitted only as a visible, commented override for something the manifest cannot express, and never as a way to restate workspace policy.

**Errors.** `thiserror` in library crates, so callers can match variants. `anyhow` **confined to binary crates** — a library's public API never returns `anyhow::Error`. This split is corroborated by two 2026 practitioner blogs and by **no first-party source**; it is adopted as a house convention on that basis, not as a documented standard.

**Tests.** Unit tests stay inline in `#[cfg(test)] mod tests`, which is already the repository's practice. Integration tests are consolidated into **few binaries** — at most one integration-test binary per crate under test, structured as `tests/it/main.rs` with `mod` declarations for the cases — never one file per case. The current total absence of any integration-level test against a public API is recorded here as a known gap that this policy makes cheap to close.

**Configuration.** Configuration reaches the kernel **as a value**: a `Config` struct constructed at the composition root and passed in. The kernel never reads the environment and never writes it. The process may read environment variables once, at the edge, while building `Config`. `std::env::set_var` is not called anywhere in the workspace, in tests included; a test that needs different configuration constructs a different `Config`.

**CI.** A minimal workflow exists from the first commit and runs on every push and pull request:

1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets`
3. `cargo test --workspace`

It runs on the pinned toolchain. `cargo audit` joins it under 0084. This is a floor, not a ceiling — it is specified as the minimum that must never be absent again.

## Consequences

**Positive**
- One place to read the lint policy for the whole workspace, and one place a divergence shows up.
- A pinned toolchain makes "works on my machine" a reportable defect rather than an unfalsifiable one, and makes edition-2024 behaviour reproducible across contributors and CI.
- Denying `clippy::all` plus an enumerated pedantic subset gets most of pedantic's value without making a clippy release a CI outage.
- Consolidated integration tests keep the 3×/5× build-time and artifact costs off the table before they can accrue across twelve crates.
- Configuration as a value removes an entire class of multi-threaded test flake, and makes the kernel's inputs inspectable — which the determinism argument in 0076 needs anyway.
- CI existing at all closes the pass's single biggest flagged gap.

**Negative**
- A pinned toolchain must be bumped deliberately, so the workspace will lag stable by design, and a contributor on a newer rustc gets a rustup download on first build.
- The curated pedantic subset is a maintained list; it will drift toward either uselessness or noise unless additions are argued.
- `anyhow` in binaries only means adapter crates — which are libraries — need real error enums for cases where an opaque error would have been adequate.
- Consolidated integration binaries mean one compile failure in the test binary blocks every case in it, and a single slow case is less trivially isolable than `cargo test --test <file>`.
- This record does **not** settle per-adapter feature isolation. Resolver v2 scopes feature unification by target kind but still unifies workspace-wide *within* a target for any shared dependency, so a `cargo build --workspace` can leak a test-only or adapter-only feature into the shipped binary's graph. The reliable isolations are separate `cargo build -p` invocations or identical feature sets across crates sharing a dependency; choosing between them is left open and must not be assumed away.

**What this forbids**
- Forbids a floating `channel = "stable"` in `rust-toolchain.toml`, and forbids having no `rust-toolchain.toml`.
- Forbids `#![deny]`/`#![allow]` in a crate's source as a restatement of workspace policy; the manifest table is the only home for policy.
- Forbids denying `clippy::pedantic` or `clippy::nursery` as groups.
- Forbids `anyhow::Error` in the public API of any library crate.
- Forbids `std::env::set_var` and `std::env::remove_var` anywhere in the workspace, including tests.
- Forbids the kernel reading the process environment; configuration is a constructor argument.
- Forbids one integration-test file per case.
- Forbids merging without the three CI checks green on the pinned toolchain.
- Forbids treating this record as externally grounded on the workspace *layout*: no authoritative guide exists for a ~12-crate one-binary per-adapter-crate workspace, and this is synthesis.

## Evidence and confidence

- [documented] `[workspace.lints.rust]` / `[workspace.lints.clippy]` with `[lints] workspace = true` per member is the documented mechanism for workspace-wide lint policy. n = 2 ([1P] Cargo Book "Lints"; rust-lang/cargo #13157), evidence-augmentation §3.
- [documented] `clippy::correctness` is the only deny-by-default group; `pedantic` and `nursery` are documented as false-positive-prone, and nursery lints can change or be removed without notice; `clippy::cargo` checks metadata, not code. n = 1 first-party ([1P] Clippy Lints book), §3.
- [documented] `rust-toolchain.toml` is a pin and `package.rust-version` a floor; they answer different questions. n = 2 ([blog] swatinem.de; [1P] RFC 3537), §3. The "pin for a binary you control" framing is the blog's, corroborated by RFC 3537's separate treatment of the floor — it is not a first-party recommendation for this project shape.
- [documented] `set_var`/`remove_var` became `unsafe fn` in edition 2024 because POSIX `setenv` is not thread-safe and races concurrent `getenv`, implicating `cargo test`'s default multi-threaded harness. n = 2 ([1P] Rust Edition Guide; rust-lang/rust PR #124636), §2. This licenses "environment mutation is unsound under the default harness"; it does **not** license a claim that reading the environment once at process start is unsafe, which it is not.
- [measured, external] Consolidating Cargo's own integration-test suite cut build time **3×** and artifact size **5×** ([blog] matklad.github.io, confirmed in §2). Measured on Cargo's suite, by its author's own account — it does not license a predicted 3×/5× for this workspace, only the direction and the mechanism (one file = one binary relinking the stack).
- [documented, weak] `thiserror` for libraries and `anyhow` for the application layer is restated as current best practice by **two 2026 practitioner blogs and no first-party source** ([blog] OneUptime 2026-01-25; Pi Stack 2026-06-22), §2. Adopted as a house convention on explicitly thin evidence.
- [documented] Direct repository inspection: `spikes/runner/Cargo.toml` has `edition = "2024"` and no `[lints]`; no `rust-toolchain.toml`, no `clippy.toml`, no `tests/` dir (17 files use inline `#[cfg(test)] mod tests`), and no `.github/workflows` directory exists in the repository at all. n = 1 inspection, **no URL** (§3). This is the state as of 2026-09-08 and nothing more.
- [documented] Resolver v2 scopes feature unification by target kind but still unifies workspace-wide within a target for shared dependencies ([blog] nickb.dev; [1P/forum] Rust internals), §3.
- **What no evidence licenses:** there is no authoritative external guidance for a ~12-crate / one-binary / per-adapter-crate workspace layout (§5). Feature-unification mechanics are documented; the layout advice is not. Every layout-shaped clause in this record is redkiln-rs's own synthesis and should be cited as such, never as external consensus.
