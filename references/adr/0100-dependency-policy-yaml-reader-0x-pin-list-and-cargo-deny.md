---
id: "0100"
title: yaml_serde is the YAML reader for redkiln-migrate only; 0.x crates are exact-pinned by name with named exemptions; cargo-deny enforces the ban list
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-002 (§3.1), PIA-028, PIA-029 (§4.5), PIA-083 (§4.3); references/adr/0084 §Decision and §Considered options; crates.io and osv.dev fetched 2026-09-11
supersedes: ["0084"]
superseded_by: []
related: ["0083", "0086", "0095", "0102"]
evidence: ["PIA-002", "PIA-028", "PIA-029", "PIA-083"]
consumers: []
---

## Context and problem

Three gaps in the dependency record, all landing at Step 5.1/5.2 and Step 7d.

**No YAML reader is named.** `redkiln-migrate` must parse the frozen corpus's YAML frontmatter, fenced ledgers and
`.redkiln/processes/*.yaml`, yet design §9.2's fourteen rows, ADR-0084 and Step 7d name no YAML crate; the RFC's
choice (`serde_yaml`) is archived at 0.9.34+deprecated, and its first successor `serde_yml` is unmaintained and carries
RUSTSEC-2025-0068 [documented, crates.io / osv.dev fetched 2026-09-11; PIA-002].

**ADR-0084's 0.x rule is contradicted by its own table.** Its option analysis says "the house 0.x rule" makes exact
pins the default for pre-1.0 crates, its Decision pins only `gix`, and design §9.2 leaves four 0.x crates on caret
ranges — `jsonschema` (0.55.1 in the table, 0.56.0 today; five releases in eleven days), `rusqlite` 0.40.x, `chrono`
0.4, `sha2` 0.11 — with no statement of which are exempt or why [measured, PIA-028].

**ADR-0086's four banned crates have no enforcing instrument**, and AC-005/AC-044/AC-120's Verified-by lines name
`cargo-deny`, which no step installs and no CI line runs [measured, PIA-029]. ADR-0084's advisory evidence also
misattributes three gix advisories' fixed versions and says "no advisory affects `bundled`" (RUSTSEC-2022-0090 /
CVE-2022-35737 was against bundled SQLite in `libsqlite3-sys` < 0.25.1) — corrected in ERRATA.md, since the pin is
clear either way [documented, PIA-083].

This record supersedes ADR-0084 **in part**: the range clause (which crates are exact-pinned) and the CI job list.
The `gix` pin, the schema clause as ADR-0095 restates it, and the rusqlite/WAL/one-writer clause stand; ADR-0084 stays
`accepted`.

## Decision drivers

- A migration tool with no named parser is a Phase-1 deliverable nobody can start.
- "0.x inherits the exact-pin rule" is a rule only if the table follows it; a caret on a crate publishing weekly is
  the case the rule was written for.
- A ban with no instrument is prose; `cargo-deny` is the lockfile-level check and it is one install.

## Decision

**1. `yaml_serde` is the YAML reader, for `redkiln-migrate` only.** `yaml_serde = "=0.10.7"` (The YAML
Organization's maintained continuation of `serde_yaml`; no advisory on osv.dev; verified 2026-09-11). It appears in
exactly one crate's manifest; no other workspace member may depend on a YAML crate — the new store's serialiser is
JSON/TOML and byte-parity with the old YAML is dissolved by the one-shot migration (ADR-0002). Design §9.2 gains the
row; Checkpoint 7d greps the pin.

**2. The 0.x pin list is explicit.** Exact-pinned: `gix` (unchanged), `jsonschema = "=0.56.0"` (weekly cadence; format
validation is opt-in via `.should_validate_formats(true)` and the design must call it where it relies on format
checks), `rusqlite = "=0.40.2"` (carries the one-writer semantics), `yaml_serde = "=0.10.7"`. **Exempt, with the
reason stated in the manifest comment:** `chrono ^0.4` and `sha2 ^0.11` — de-facto stable, multi-year 0.x lines with
semver discipline. Any new 0.x dependency is exact-pinned unless its row says why not. A pin bump is a PR that re-runs
`cargo audit` and `cargo deny`.

**3. `cargo-deny` is provisioned at Step 5.2** (`cargo install cargo-deny --locked`) with a committed `deny.toml`
carrying ADR-0086's four bans (`temporalio-sdk`, `graph-flow`, `dbos`, `durable` — workspace-wide, dev-dependencies
included) in `[bans].deny`, plus `[advisories]` with `unmaintained = "deny"` and `[licenses]` allowing the set the
workspace already uses. It runs as a fifth CI gate beside fmt/clippy/test/audit; `cargo audit` is **retained** for
its RUSTSEC-specific output. A must-fail fixture adds a banned crate to a scratch member and asserts `cargo deny check
bans` exits non-zero. AC-005/AC-044/AC-120's Verified-by lines now name an instrument that exists.

**4. `cargo audit --deny unmaintained`** is the form the audit job runs (equivalently `cargo deny check advisories`
with `unmaintained = "deny"`), so an archived transitive dependency is a red build, not a silence.

## Considered options

### Option — re-point AC-005/044/120 at AC-484's `cargo metadata` assertion instead of cargo-deny
Rejected: `cargo metadata` sees direct edges; a banned crate arriving transitively is what a lockfile-level check
catches and a graph grep does not.

### Option — `serde-yaml-bw` or `serde_yaml_ng`
Both live; `serde-yaml-bw` 2.5.8 is panic-free by design. Rejected in favour of the YAML Organization's continuation
on maintenance provenance; a successor may swap it if `yaml_serde` stalls, and the tool is one crate.

### Option — defer the reader to Step 7d's implementer
Rejected: §9.2 would stay incomplete, Checkpoint 7d would have no pin to grep, and the audit's currency finding (13
rows stale within 48 h) argues for naming a version and a date now.

### Option — caret ranges on every 0.x but gix
ADR-0084's implicit state. Rejected: `jsonschema` moved 0.55.1 → 0.56.0 during the audit's own 48-hour window.

## Consequences

**Positive**
- Step 7d has a parser, Step 5.2 has a ban instrument, and every 0.x crate has a stated status.

**Negative**
- Five exact pins mean five deliberate bump PRs; the audit job is what makes a stale pin red rather than silent.
- One more CI tool to install on the box and in CI.

**What this forbids**
- Forbids a YAML dependency in any crate but `redkiln-migrate`.
- Forbids a 0.x dependency on a caret range without an exemption comment naming the reason.
- Forbids merging a manifest that adds a dependency `deny.toml` bans, and forbids removing a ban without a successor
  to ADR-0086.
- Forbids describing the `cargo audit` job as covering unmaintained crates unless it runs with `--deny unmaintained`.

## Evidence and confidence

- [documented, crates.io fetched 2026-09-11] `yaml_serde` 0.10.7 (2026-08-18); `serde_yaml` 0.9.34+deprecated
  (2024-03-25); `serde_yml` 0.0.13 marked DEPRECATED; `jsonschema` 0.56.0 (2026-09-10); `serde-yaml-bw` 2.5.8;
  `yaml-rust2` 0.13.0.
- [documented, osv.dev fetched 2026-09-11] `serde_yml`: GHSA-hhw4-xg65-fp2x / RUSTSEC-2025-0068; `yaml_serde`: none.
- [measured, PIA-028] four 0.x crates on caret ranges in §9.2 against ADR-0084's stated rule.
- [measured, PIA-029] `cargo-deny` named by three criteria and provisioned by nothing.
- [documented, PIA-083] the advisory corrections carried in ERRATA.md; every pin is clear (gix 0.87.1 pulls
  gix-features ^0.49.1, gix-ref ^0.67.1, gix-worktree ^0.56.0; rusqlite 0.40.2 pulls libsqlite3-sys ^0.38.2).
- **What this evidence does not license.** It does not license claiming `yaml_serde` round-trips the corpus's
  frontmatter byte-identically — AC-454's fixture is the first measurement. It does not license any claim that the
  ban list is complete; it is ADR-0086's list.
