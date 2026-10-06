# Audit w1 - CODE AREA: crates/rk-types

Snapshot: commit 36dd8ac (build/phase-0-to-1), read-only worktree. Auditor did not run cargo; all test claims are from reading test source and records. Where a RECORD claims a checkpoint passed, it is labelled "records claim".

## Verdict

Grade: **B (solid with minor gaps)**. Completeness against what Phase 1 requires of this area: ~92%.

rk-types is the best-disciplined crate in the programme: no async deps, no unsafe, no unwrap/panic in production code, closed enums matched exhaustively, one integration-test binary, a golden snapshot over all 25 exported schemas plus a real lint library, and test-first discrimination for the named criteria. The gaps are (a) a few invariants asserted in doc comments but not enforced or tested (`Digest` deserialisation, `Drift` distinctness, `GateBasis` named but absent), (b) per-artifact test depth for the 23 lane kinds is schema-shape only (no round-trip / ingest tests per kind), (c) `DocSet` is a deliberately empty placeholder, (d) very heavy provenance prose inside source (about a third of all lines are comments). `ChangeSummary` / `Binary` are not in this crate by design and the Binary arm is still sealed (S-BIN, step 8i, is in progress).

## 1. Module / public surface map (src = 4,437 lines per census; `wc` of all .rs incl. tests = 7,888)

`crates/rk-types/src/lib.rs:21-39` declares 21 public modules. Deps (`crates/rk-types/Cargo.toml`): serde, serde_json, schemars, sha2 only; dev: insta, jsonschema. No tokio/async (ADR-0076 holds).

| Module | Lines | What it is |
|---|---|---|
| `error.rs` | 204 | `StepError` (11 variants, `:65`), `spoke()` `:119`, `retry_class()`, `event()`; `TransportKind`, `CancelledBy`, `IsolationSurface`, `RetryClass` |
| `event.rs` / `event_kind.rs` | 102 / 438 | `ErrorEvent`, `FailureClass`, `RefusalClass`; closed `EventKind` enum, `ALL: [Self; 42]` (`event_kind.rs:193`), `Tier`, `ItemKey`, wire-name round trip |
| `registry.rs` | 82 | `exported!` macro generating `EXPORTED` and `EXPORTED_NAMED` from ONE table of 25 schemas (`registry.rs:39-64`) |
| `schema/{mod,transform,lint,judgement}.rs` | 36/53/187/280 | `export_settings()` (inline subschemas + `KindOneOfToAnyOf` transform, `schema/mod.rs:17-24`); `lint()` with `Violation` set and `UNION_CAP=16`/`OPTIONAL_CAP=24` (`lint.rs:19-20`); AC-133 judgement-enum registry (`judgement.rs`) |
| `artifact/{lane,findings,ledger}.rs` | 419/261/166 | 23 lane artifact kinds (Spec, ImplReport, RootCause, ... ThreatModel), `Findings`/`Finding` with ingest, verify-gate ledger enum |
| `acceptance.rs` | 153 | `AcceptanceCriterion` + `ingest` (`:123`) |
| `repro.rs`, `review.rs`, `verify.rs` | 67/96/277 | typed repro record; review gate outcome + quarantine; verify report, `Basis`, `BasisReason` (6, `ALL`), `VerifyCompleted` payload |
| `activity.rs`, `usage.rs`, `retention.rs` | 293/252/149 | `ActivitySpec`, `DispatchRecord` (`activity.rs:93`), `Usage`/`Timings`/`CliVersion`/`CostUsd` (no `Default`, `Option` for absent per AC-362), `Retained` (cap 64 KiB, head/tail 2 KiB) |
| `verb_output.rs` | 302 | seven toolchain-verb output structs (s8c) |
| `digest.rs`, `text.rs`, `time.rs`, `typed.rs`, `companion.rs`, `drift.rs`, `docset.rs` | 122/21/28/55/148/139/40 | `Digest`+`sha256`/`sha256_lp`/`Sha256Stream`; `Text`; `Timestamp` (no `now()`); `SchemaId`/`Typed`; 6 companion kinds with registered sidecar schemas (`companion.rs:58`); `Drift` (11 variants) with `kind()`/`blocks()`; `DocSet(())` |
| `lib.rs:61` | | `LOCK_HALTS` const (one copy of the poisoned-lock panic string, ADR-0118) |

NOT in this crate (by record): `ChangeSummary`, its `Text` arm and the sealed `Binary` arm. They live only in `crates/rk-ports/src/toolchain.rs` (`pub enum ChangeSummary` `:71`; `pub struct BinaryChangeSummary(pub(crate) ())` `:153`) under ADR-0165, which superseded the original "rk-types owns ChangeSummary" wording. The task brief's phrase "ChangeSummary/Binary" therefore belongs to rk-ports; assessed here only as it bears on rk-types (`lib.rs:9-18` header, `docset.rs` comments).

## 2. What Phase 1 requires of this area (stage/design)

Sources: `runbook/phase-1/s5-workspace-ci-rk-types.md` §5.3 (`:267-`), design `references/2026-09-08-architecture-design.md:1721` (rk-types row), `:1831-1839` (StepError/spoke, Option-over-0), `:840`/`:1722` (ChangeSummary), later additions in s7a, s7b-ii, s7c, s8c, s8e-s8h.

Required: ids, typed artifacts, digests; schema derivation with ADR-0095 closed-output rules (no `oneOf`, no nested union, `additionalProperties:false` everywhere, required-first, <=16 union / <=24 optional per schema); eleven-variant `StepError` with total `spoke()` and one event arm each; typed `Findings.repro` with prose-only rejection; `ReviewWeight`/`Gwt`/ledger/`EvidenceClass`; the companion kind list (5 + ledger); `Drift`; `GateBasis`; `EventKind` closed enum; `DispatchRecord`/usage types without `Default`; the S-SCHEMA-CAP spike observing the vendor's rejection shape.

## 3. Compare: implemented and exercised vs deferred / missing

### Genuinely implemented AND exercised by tests (code shows; tests read)

- **StepError**: 11 variants, `spoke()`/`retry_class()` written as exhaustive or-pattern matches with no catch-all (`error.rs:119-159`); `tests/it/step_error_is_total.rs` (216 lines, 3 tests: totality table of all 11 with spoke value and event arm read off ADR-0095/0112, 6-true/5-false partition, AC-523 backoff table). Read as written, this is a real discriminating test, not a smoke test.
- **Closed-output schema rule**: transform `schema/transform.rs` rewrites ONLY kind-discriminated `oneOf` (scope stated, so it does not launder the lint); lint is a `src/` library used by CI, snapshot and fixtures (`schema/lint.rs:1-6`). Tests: `schema_no_oneof.rs` (6 tests), `schema_caps.rs` (4: real exported set clean; hand-built `Union17Explicit`, `Union17Options`, `Optional25` fixtures each caught with exact counts `schema_caps.rs:27-67`) - the mutants-both-ways pattern.
- **Golden snapshot**: `schema_snapshot.rs` asserts registry non-empty and `export_all().len()==EXPORTED.len()`, then snapshots each; 25 `exported__*.snap` files present for the 25 registry entries (counted in `tests/it/snapshots/`) plus 6 companion snapshots. Registry is a single macro table so a type cannot enter `EXPORTED` without `EXPORTED_NAMED` (`registry.rs:23-39`); `judgement_registry.rs` (3 tests) cross-checks AC-133.
- **Findings ingest** (AC-281; ADR-0096): typed repro, prose-only command rejection (heuristic with recorded false-accept/false-reject, `findings.rs:~120-165`), `mutate` without `mutation` rejected; `repro_ingest.rs`, `findings.rs`, `review_gate.rs` (311 lines; jsonschema validation of five negative fixtures per Cargo.toml comment).
- **Digest**: `sha256` checked against published vectors, `sha256_lp` length-prefix collision property (`digest_compute.rs:15-40`) - external oracle, good.
- **EventKind**: 16 tests incl. `all_lists_every_variant_exactly_once`, tier-one count pinned at five, wire-name round trip, `ReplayDivergence` deliberately absent (`event_kinds.rs`).
- **DispatchRecord / usage** (AC-360, AC-362): `dispatch_record.rs` (471 lines, 6 tests), `usage.rs` (7 tests). `Default` deliberately absent from usage types; absence propagates as `None` through `ttl_split`/`total_input` (`usage.rs` header). The CI grep for banned `unwrap_or(0)` is outside this crate.
- **Companion kinds**: 6 kinds, each with registered schema, snapshot each (`companion_kinds.rs`, `companion.rs:58`).
- **Records claim** Checkpoint 5 re-run WHOLE and passed at d24d15a (2026-10-02, after s8h moved rk-types) - `runbook/PROGRESS.md:28`. Not re-verified by this audit (no cargo). It has NOT been re-run since 36dd8ac-era commits unless a later row says so; last rk-types-touching commits are `55fda45`, `d5bfb32` (s8h).

### Stubbed / placeholder / deferred / ARMED

- **`DocSet(())`** (`docset.rs:31`): an opaque empty newtype; contents explicitly "OWED". Justified (mdBook DocsSite is Phase 3, design:268) but it means one of the five "typed artifacts" named in the design row (`Spec, ImplReport, Findings, DocSet, digests`) is a placeholder. Honest, not hidden.
- **`Typed` / `SchemaId`** (`typed.rs`): deliberately no `Serialize`/`JsonSchema`; header says "A record naming the home of both is OWED" and routing "may reverse" (`typed.rs:5-12`). Works today, but is an open routing decision.
- **S-SCHEMA-CAP** (design `:1974`, registered "unrun"): `git grep` finds no result record anywhere outside the register/ADR text; s7c says "enabled, not owed" (`runbook/phase-1/s7c-adapters.md:3300`). So the vendor's actual rejection shape for a 17-union / 25-optional schema has never been observed. The caps (16/24) are the vendor's DOCUMENTED limits (fetched 2026-09-10), enforced by the lint, but F20's "fails open at exit 0" means an over-cap or non-conformant schema is not observed to be caught by the vendor. Treated as UNVERIFIED-against-reality (see unverified list).
- **`GateBasis`**: named in the design row (`design:1721`), s5 §5.3 and `lib.rs:4`, but there is no such type in the crate or anywhere under `crates/` (`git grep GateBasis` hits only `lib.rs:4`). Functionally covered by `verify::Basis`/`BasisReason` and `review::ReviewBasis` (`review.rs:70`), so this is a stale name, not a missing capability - but the crate header over-claims.
- **Binary change summary**: not here; sealed in rk-ports until S-BIN (step 8i, in progress per brief). Nothing in rk-types waits on it (ADR-0093 decision 4 as superseded by ADR-0165).
- **`StepError::Schema` shape drift**: design `:1834` says `StepError::Schema` carries `raw`; code carries `{ attempts: u32, last_error: String }` (`error.rs:69-72`). Raw retention lives on `DispatchRecord.stdout/stderr: Retained` instead. Probably a design-text staleness rather than a code defect, but the record and code disagree and I found no dated amendment to `:1834`.

### Missing / thin

- **No test for `Drift` at all in rk-types.** The doc says "its distinctness is asserted by a test rather than assumed" (`drift.rs:13-15`) but there is no `drift` test file and no `Drift` reference under `crates/rk-types/tests` (grep: none). `Drift::kind()` uniqueness is only protected by the `match_same_arms` lint trick (which catches identical *bodies*, not identical *strings* - two arms returning different variants' same literal would be distinct arms with equal bodies, so the lint does fire; but a typo-duplicate across non-adjacent arms is not distinct-by-test). `blocks()` has one consumer-side test (`rk-store/tests/it/vouched.rs:301`). AC-124/127/128 coverage lives in other crates; not examined here.
- **`Digest` invariant hole**: `Digest` is `#[derive(Deserialize)] #[serde(transparent)] struct Digest(String)` (`digest.rs:~18-23`), documented "The only constructor: 64 lowercase hex" (`:29`). Deserialisation bypasses `from_hex`, so a stored/imported payload can mint a `Digest` of any string (e.g. `"x"`), defeating the "cannot silently mint an id" claim for every record that round-trips through JSON (`DispatchRecord`, `VerifyCompleted`, `Retained::Capped`). No test covers deserialising a malformed digest (grep of `crates/rk-types/tests` for `Digest` deserialisation: none). Low probability today (rk-store writes what the kernel produced) but a real validation gap on the read path, notably for the `doctor --rebuild` replay-from-git claim.
- **Per-kind depth for the 23 lane artifacts**: only `RootCause` has a behavioural test (`root_cause.rs`, 74 lines, 1 test); the other 22 are covered by schema-shape lint + snapshot (change detection, not correctness). No serde round-trip or `deny_unknown_fields` rejection test per kind; no test that a stand-in `spikes/p9/schemas.toml` row maps to the Rust field set (the lane.rs header says types are a "transcription" `lane.rs:6-10`; fidelity to that file is by-hand).
- **`StepError` has no `Display` / `std::error::Error` / `Serialize`** (`error.rs` has no impls; contrast `CriterionIngestError`, `LedgerRowError`, `VerifyReportShapeError` which do implement them). This is consistent with the record ("StepError and Typed carry no Serialize", `activity.rs` header) and with how it crosses ports, but `Display`/`Error` absence means callers cannot `?`-propagate it into boxed errors and must format it themselves. Minor design debt, not a defect.
- `DigestError`, `IngestError` do not implement `std::error::Error` while sibling error types do (`acceptance.rs:95`, `ledger.rs:106`, `verify.rs:158` vs none for `IngestError` in `findings.rs:~8`). Inconsistent error-trait coverage.

## 4. Code quality

- **Structure**: one concern per module; consistent newtype pattern (`Digest`, `Text`, `Timestamp`, `SchemaId`); `registry.rs` single-source macro; schema logic in `src/` not tests. Good.
- **Error design**: closed enums, no catch-all, `const fn` classifiers where possible, clippy `match_same_arms` used deliberately as a structural guard (`error.rs:107-118` comments explain why `matches!` is avoided). Strong.
- **Escape hatches in NON-TEST code (spot-checked against the census line `crates/rk-types src=4437 tests=3451 testfns=83 unwrap/expect=4 panics=0 allow/expect-attr=1 unsafe=0`)**:
  - Census "unwrap/expect=4": **3 real production `.expect(` calls** - `schema/mod.rs:35` and `schema/judgement.rs:141` (`"a derived schema serialises to JSON"`), `verify.rs:275` (`"a verify_completed payload serialises to JSON"`). The 4th hit is a doc comment (`lib.rs:49`, mentions `.expect(LOCK_HALTS)`). All three are `serde_json::to_value` over derive-generated types with no non-string map keys, so effectively infallible, but they ARE production `expect`s and `to_payload()` is `pub` returning a bare `Value`; under a strict no-escape-hatch standard (`temper:rust-craft` posture) they should return `Result` or be proven infallible by construction. Severity: low.
  - Census "allow/expect-attr=1": **doc-comment mention only** (`usage.rs:171`, prose describing `#[expect(clippy::disallowed_methods ...)]` used elsewhere). **Zero real lint-suppression attributes in rk-types src** (grep `#\[allow|#\[expect` across `src`: only that doc line).
  - `unsafe=0`, `panics=0`, `todo/unimplemented/dbg=0`; workspace sets `unsafe_code = "deny"` (`Cargo.toml` `[workspace.lints.rust]`).
  - `cfg(test)`: exactly one in-src test module (`artifact/findings.rs:228`, 2 tests); everything else is the integration binary.
- **Test layout**: single integration binary `tests/it/main.rs` (ADR-0083), 19 modules, 83 test fns (matches census). Integration + golden dominates; in-crate unit tests are nearly absent (2). Tests that read the code under test as oracle are mitigated by external vectors for sha256 and ADR-table-derived tables for StepError.
- **Comment-to-code noise**: 1,469 of 4,437 src lines (33%) are `//`/`///`/`//!` comment lines (my count: lines starting with `//`); the heaviest are `event_kind.rs` (158/438), `activity.rs` (153/293 - 52%), `usage.rs` (143/252 - 57%), `verb_output.rs` (117/302). Much is ADR/PIA/session-dated provenance ("dated 2026-09-29, `s8f` build, package S8F-P7...", `activity.rs:50-58`, `registry.rs:12-20`, `verify.rs` BasisReason note) and strike-through history (`activity.rs:50`) rather than explanation of the code. Accurate and traceable, but it reads as a change log embedded in source and several type docs carry "OWED"/"routed, not decided" tails (`typed.rs`, `docset.rs`, `retention.rs`) that will rot. Also leaks the exported schema description unless carefully kept as plain `//` (called out at `verify.rs` BasisReason note). `lib.rs:9-18` header is a self-correcting contradiction (original paragraph then an "amended" paragraph) and still lists `GateBasis`.
- **Dead code**: none identified by reading; `GateBasis` is only a doc-name. `EXPORTED_NAMED` is used by the AC-133 registry only (documented "never sent across the boundary"). `Sha256Stream` exists for rk-adapter-claude streaming (documented).
- **Clippy posture**: workspace `all = deny`, enumerated pedantic subset deny, nursery warn; crate inherits `[lints] workspace = true`. Not run by this audit.

## 5. Risks

1. **Digest deserialisation bypass** (medium-low): the invariant that makes digests trustworthy is not enforced on the read path. Cheap fix (custom `Deserialize` via `from_hex`).
2. **S-SCHEMA-CAP never observed** (medium, whole-programme): the schema-caps rule is enforced against the vendor's documentation, not the vendor's behaviour; F20 says violations fail open. If the documented limit or `anyOf` handling differs on the pinned CLI, 23 lane kinds feeding real dispatches could fail silently. Any run in step 8i/8j using real structured output will exercise this indirectly.
3. **Snapshot is change-detection, not correctness** (acknowledged by the test header `schema_snapshot.rs:1-12`): a rubber-stamped `insta accept` of a bad schema change is convention-guarded only. Mitigated by the lint running over the same exported set.
4. **Lane-kind fidelity is hand-transcribed** from `spikes/p9/schemas.toml` with no mechanical conformance test; only `RootCause` has a behavioural test.
5. **Doc/record staleness**: `GateBasis` name, `StepError::Schema carries raw` (design `:1834`), `lib.rs` header; these mislead a new reader and the audit trail, not behaviour.
6. **Re-run churn**: records show Checkpoint 5 re-run whole after nearly every stage that touches rk-types (`PROGRESS.md:28`); any rk-types edit made by the 8i/8j work forces another re-run - a process cost, not a code risk.
7. **Open routing records OWED** (`Typed`/`SchemaId` home; `Retained` home; `DocSet` contents): all explicitly reversible but accumulate unrecorded decisions inside code docs.

## 6. Unverified (could not or did not verify)

- That `cargo test -p rk-types` and Checkpoint 5 pass at 36dd8ac: not run (instruction), and no record in the read portion states a pass at this exact commit; last stated pass is d24d15a (records claim).
- Whether S-SCHEMA-CAP was ever run: grep found no result record outside register/ADR text; may exist in a spike/corpus repo not in this tree.
- Whether AC-229's register text was corrected for "eight vs eleven" variants (test header says erratum owed, `step_error_is_total.rs:7-12`); not checked in `references/adr/ERRATA.md`.
- Per-AC mapping for `Drift` tests in `rk-git`/`rk-store`/`rk-cli` (AC-124/127/128/165/167/176/193) not traced beyond the grep above.
- Whether `Drift::kind()` strings are asserted unique somewhere outside rk-types (grep of `.kind()` on Drift in tests under rk-git and rk-cli found none).
- Exact content of the 22 lane kinds' fields versus `spikes/p9/schemas.toml` (not diffed).
- `git diff 36dd8ac` vs the live checked-out working tree: deliberately not examined; this report is about the pinned commit only.
