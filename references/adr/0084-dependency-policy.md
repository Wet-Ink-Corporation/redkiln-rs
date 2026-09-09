---
id: "0084"
title: Dependency policy — exact-pinned gix, audited CI, a golden schema snapshot, one SQLite writer
status: accepted
date: 2026-09-08
legacy_id: none
source: references/2026-09-08-evidence-augmentation.md §2 (dependencies), §3 (dependencies), §4 recs 6 and 7, §5 rows on rusqlite WAL and the RUSTSEC listing
supersedes: []
superseded_by: []
related: ["0003", "0077", "0083"]
evidence: ["F20", "F31"]
consumers: []
---

## Context and problem

Four dependencies carry disproportionate risk in the Phase-1 stack, and the external pass found something specific about each.

**`gix` is at 0.87.1** — still pre-1.0, and a facade over dozens of independently versioned `gix-*` sub-crates. It is multi-maintainer (GitoxideLabs, Sebastian Thiel leading), so not a single-author risk, but squarely a 0.x risk. Its RUSTSEC history is dense: RUSTSEC-2025-0021/CVE-2025-31130 (`gix-features`, missing SHA-1 collision detection); RUSTSEC-2025-0001 (`gix-worktree-state`, checkout sets files world-writable); RUSTSEC-2024-0349 (`gix-worktree` path traversal / arbitrary file write via crafted repo names, described as enabling RCE via malicious hooks); RUSTSEC-2024-0351/0353 (Windows reserved device names); RUSTSEC-2024-0355/0367/0371 (`gix-path` config resolution). **All are fixed at or before 0.87.1 — the pin is clean as of 2026-09-08.** The density is what matters: the cluster is checkout and path handling, several Windows-specific, which is precisely the disposable-worktree and shallow-clone-of-external-branches surface (§3, dependencies; §4 rec 6).

**`schemars` is stable at 1.2.2** and derives closed, flat schemas for plain structs — but Rust enums with data-carrying variants still derive to `oneOf` **by default**, matching serde's externally tagged representation. `#[serde(tag = "kind")]` renders a single discriminated object instead. The design's "no `oneOf` nesting" rule is therefore a constraint on the **Rust types**, not a guarantee schemars provides (§3, dependencies; §4 rec 7). Nothing currently checks it, and a closed-output violation is exactly the class F20 showed fails silently at exit 0.

**`rusqlite` is at 0.40.2**, `bundled` builds SQLite from source for a portable single binary, and WAL gives readers-don't-block-writers — but SQLite still **serialises writers** under WAL, so the store wants one writer connection or a queued write path, not a pool expecting parallel writes. That semantics was read via search, **not** from first-party docs (§2, dependencies; §5).

**`jsonschema` is at 0.55.1** and `default-features = false` removes network and filesystem `$ref` resolution (reqwest HTTP retrieval, file resolving) but **not** regex or format validation — which is exactly right for embedded in-process schemas, and is a narrower claim than "trims the footprint" (§2, dependencies).

There is no dependency policy in the repository, and there is no CI at all to hang one on (0083).

## Decision drivers

- 0.x crates have no semver contract on breakage, and gix's worktree and trailer semantics are load-bearing for the Phase-1 exit gate (0077). A caret range can move them silently mid-phase.
- The advisory cluster is in the code paths this system exercises hardest, on the platform it is being built on. `cargo audit` against the real `Cargo.lock` is the authoritative check and supersedes any spot search (§5).
- A schema rule that lives only in prose is not a rule. F20 measured the failure mode: an agent card's `tools` allowlist silently disabled `--json-schema`, `structured_output` absent, **exit 0**, `subtype: success`. Schema defects in this system do not announce themselves.
- The store lives at `<git-common-dir>/redkiln/` and is shared across worktrees (0003, 0018), so multi-process access is the normal case, not an edge case.
- Restricting `jsonschema`'s feature set removes an entire network and filesystem surface from a validator that only ever sees embedded schemas.
- 0083 stands up CI from the first commit; wiring audit into it then costs one job, and later costs a migration.

## Considered options

### Option — caret ranges throughout, upgrade on cadence

The normal Rust default. Rejected for `gix` specifically: pre-1.0 plus a facade over dozens of independently versioned sub-crates plus load-bearing worktree/trailer semantics is the combination the house 0.x rule exists for. Kept for genuinely stable 1.x dependencies, where a caret range is the right default and pinning everything would be cargo-cult.

### Option — trust the schema rule to authorship discipline

Rejected on F20's mechanism. Closed-output violations here fail open at exit 0 with `subtype: success`; there is no loud failure to catch a regression. A rule that only a careful author enforces will be violated by the first `enum` with a payload, which schemars will render as `oneOf` without comment.

### Option — a connection pool over the SQLite store

Rejected: SQLite serialises writers even under WAL, so a pool sized for parallel writes buys contention and `SQLITE_BUSY` rather than throughput. WAL's actual gift is that readers do not block the writer, which a single writer connection collects in full.

### Option — run `cargo audit` before releases only

Rejected. The advisories that matter here are path-handling and checkout defects that arrive with a transitive `gix-*` bump, not with a deliberate upgrade. A check that runs at release time finds them after they have been merged and built on.

## Decision

**Pin `gix` exactly.** `gix = "=0.87.1"` in the workspace dependency table — an exact pin, not a caret range. Bumping it is a deliberate PR that re-runs `cargo audit`, re-checks the `gix-*` sub-crate versions in `Cargo.lock`, and re-runs spike S-GIX (0077) if the pin crosses a version where either arm's answer could have changed. gix version currency is a **release-blocking check**, not a one-time approval. Exact pinning is required for `gix`; other 0.x dependencies adopted later inherit the same rule. Stable 1.x dependencies use ordinary caret ranges.

**Wire `cargo audit` into CI now**, as a fourth job alongside 0083's fmt/clippy/test, running against the committed `Cargo.lock` on every push and pull request. A new advisory fails the build. `gix-*` is watched specifically, and the Windows CI target is not optional given where the advisory cluster sits.

**A golden-schema snapshot test.** Every type that crosses the closed-output boundary has its generated `schemars` output snapshotted with `insta`, and the test asserts on the generated JSON that it:

- contains **no `oneOf`** anywhere;
- sets `additionalProperties: false` at every object level;
- orders `required` fields first;
- stays within the optional-field and union-field budgets the design sets.

Data-carrying enums that must cross the boundary are written `#[serde(tag = "kind")]` so they render as a single discriminated object. The snapshot is reviewed as part of any PR that changes it — a changed snapshot is a schema change, and is read as one.

**`rusqlite` `bundled`, WAL, `busy_timeout`, one writer.** The EventStore adapter opens with the `bundled` feature (portable single binary, SQLite built from source), sets `journal_mode = WAL` and a `busy_timeout`, and uses **a single writer connection or a queued write path** — never a connection pool expecting parallel writes. Reader connections may be plural. This extends 0003's EventStore row; it withdraws nothing from it.

**`jsonschema` with `default-features = false`.** Recorded with what that actually does: it removes network and filesystem `$ref` resolution, and does **not** remove regex or format validation. Every schema this system validates against is embedded and in-process, so no resolver is needed and none should be linked.

**Verified versions, as at 2026-09-08.** `gix` 0.87.1 · `schemars` 1.2.2 (MIT, published 2026-07-27) · `rusqlite` 0.40.2 (2026-08-08, MIT) · `jsonschema` 0.55.1 (MIT, Draft 4/6/7/2019-09/2020-12) · `insta` 1.48.0 · `trycmd` 1.2.1 · `assert_cmd` 2.2.2 · `cargo-semver-checks` 0.50.0 (2026-08-01) · `git-cliff` 2.14.1 (2026-09-01) · `release-plz` 0.3.163 (2026-09-08) · `cargo-dist` 0.32.0.

**Two hazards recorded so they need not be re-derived.** The `cargo-dist` CLI rebranded to `dist` at v0.24.0 while the crate is still published as `cargo-dist`, and crates.io separately carries an **unrelated, abandoned crate literally named `dist` at 0.0.0** — the dependency is `cargo-dist`, never `dist`. And `jsonschema` is maintained essentially by one primary author per crates.io publisher records, though with healthy cadence and adoption: a bus-factor note alongside gix, minor rather than blocking.

## Consequences

**Positive**
- The gix pin is provably clean of every known advisory as of 2026-09-08, and cannot drift into a new one without a deliberate PR and a red `cargo audit`.
- The schema rule becomes mechanically enforced rather than aspirational, on a boundary where F20 measured that violations fail open.
- One writer connection matches what SQLite actually does, so the store's concurrency model is the same in code and in reality; readers still get WAL's non-blocking reads.
- `jsonschema` links no HTTP client and no filesystem resolver, removing a network surface from a component that has no business making a request.
- The versions and the two naming hazards are recorded, so the next contributor spends no time rediscovering them.

**Negative**
- An exact `gix` pin means security patches arrive only when someone bumps it. The `cargo audit` job is what makes that a red build rather than a silence — the pin without the audit would be strictly worse than a caret range.
- A golden snapshot is a maintenance surface: every deliberate schema change produces a diff to review, and reviewers can rubber-stamp `cargo insta accept`. The mitigation is convention, not mechanism.
- A single writer connection is a throughput ceiling. Under the "one machine per repo" operating assumption (0003) that ceiling is far away, but it is a real one and it will be reached first by a fan-out that ignores 0076's rule about where concurrency lives.
- `bundled` compiles SQLite from source, which adds C compilation to every clean build and to CI.
- No advisory check was run in this pass for `schemars`, `insta`, `trycmd`, `assert_cmd`, `cargo-semver-checks`, `git-cliff` or `release-plz`. Their clean status here is **absence of evidence, not a clean bill**; the CI `cargo audit` job is what will actually establish it.

**What this forbids**
- Forbids a caret or wildcard range on `gix`, and forbids bumping the pin without re-running `cargo audit` and re-checking the `gix-*` sub-crate set.
- Forbids merging with a failing or absent `cargo audit` job.
- Forbids relying on schemars to enforce the closed-output rule; the rule binds the Rust types, and the snapshot test is what checks it.
- Forbids a data-carrying enum crossing the closed-output boundary in serde's default externally tagged form.
- Forbids a connection pool sized for concurrent writes against the SQLite store, and forbids more than one writer connection without a queue in front of it.
- Forbids opening the store without WAL and a `busy_timeout`.
- Forbids `jsonschema` with default features, and forbids resolving a `$ref` over the network or the filesystem at runtime.
- Forbids depending on the crates.io crate named `dist`.
- Forbids citing this record's version list as a current-versions source after 2026-09-08; it is a dated snapshot.

## Evidence and confidence

- [documented] `gix` 0.87.1, pre-1.0, a facade over independently versioned `gix-*` sub-crates, multi-maintainer. n = 2 ([1P] crates.io + the GitHub org), evidence-augmentation §3.
- [documented] The `gix-*` advisory list above, with every advisory fixed at or before 0.87.1. Source: [1P] GitHub Advisory Database / RustSec, §3. **Caveat carried from §5:** rustsec.org 404'd on direct fetch and the advisory data was reconstructed per-advisory from the GitHub Advisory Database, so this list is not guaranteed complete. `cargo audit` against the real `Cargo.lock` is the authoritative check and supersedes it — which is the substantive reason the CI job is part of the decision rather than a nice-to-have.
- [documented] schemars 1.2.2 derives closed flat schemas for plain structs, but data-carrying enum variants derive to `oneOf` by default; `#[serde(tag = "kind")]` renders a single discriminated object. `#[schemars(deny_unknown_fields)]` / `#[serde(deny_unknown_fields)]` sets `additionalProperties: false`. n = 1 first-party ([1P] docs.rs/schemars), §2 and §3.
- [measured] F20: an agent card's `tools` allowlist silently disabled `--json-schema` — `structured_output` absent, **exit 0**, `subtype: success`; adding `StructuredOutput` fixed it. This is the mechanism that justifies a mechanical schema check: the failure is silent and successful-looking. It measures a *harness* failure mode, not a schemars one, and does not license any claim about schema generation.
- [measured] F31 (S-TYPED): schema conformance **52/52 = 100%**, `repro.command` fill 147/147, 51 valid runs at $112.25, cost +74% over the prose comparator. This measures that *emitted payloads conformed to the schema*. It explicitly does **not** measure that the *generated schema* satisfies the closed-output constraints — which is the distinct property the golden snapshot test checks, and which nothing in the corpus has ever checked.
- [documented, **UNVERIFIED**] rusqlite 0.40.2; `bundled` builds SQLite from source; WAL gives readers-don't-block-writers but SQLite still serialises writers, so one writer connection or a queued write path rather than a pool. §2 records the WAL semantics as read **via search only, not against docs.rs/rusqlite**, and §5 asks for a re-fetch before the multi-process store design is finalised. The single-writer rule is adopted on general SQLite behaviour, and the re-fetch is the check that would confirm or narrow it.
- [documented] jsonschema 0.55.1; `default-features = false` removes network and filesystem `$ref` resolution, **not** regex/format validation. n = 1 first-party ([1P] docs.rs/jsonschema), §2. Also: maintained essentially by one primary author per crates.io publisher records ([1P] crates.io publisher data, §3).
- [documented] rusqlite's two historical advisories (RUSTSEC-2020-0014; RUSTSEC-2021-0128, incorrect lifetime bounds on hook-registration closures, affecting 0.25.0–0.25.3 / 0.26.0–0.26.1) are far behind 0.40.2, and no advisory affects `bundled`. [1P] RustSec advisory-db, §3.
- [documented] Release-tooling versions and the `cargo-dist` → `dist` rebrand with the unrelated abandoned `dist` 0.0.0 crate on crates.io. [1P] crates.io per package; axodotdev docs, §2.
- [documented, thin] The optional-field and union-field budgets referenced by the snapshot test rest on an SDK issue's illustrative arithmetic ("4 tools × 6 optional params = 24"); §5 records that **no limits table was found** on the platform structured-outputs page, which truncated on fetch. The snapshot test therefore asserts *this project's own budget*, and must not be described as enforcing a documented vendor limit.
- **Corpus vs external:** the corpus treated schemars-derived output as meeting the closed-output constraints by construction, and treated WAL as sufficient for the store's concurrency. The external pass contradicts the first (schemars emits `oneOf` for data-carrying variants by default) and refines the second (WAL does not parallelise writers). This record acts on the external evidence in both cases; on the rusqlite point it acts on evidence the pass itself marks unverified, and names the re-fetch that would settle it.
