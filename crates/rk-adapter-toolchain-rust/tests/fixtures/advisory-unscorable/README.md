# Unscorable advisories (s8c §8c.4; D21-63)

Two FIXTURE advisories, not real ones, that the `sca_scan` refusal tests add ONE AT A TIME to
git-initialised copies of `../advisory-db/db/` (under `crates/itoa/`), re-committed, so the scan
matches three advisories: the fixture's `critical` and `unrated` pair and the one added. They sit
outside `../advisory-db/db/` so the existing scan tests' tallies (one `critical`, one `unrated`) do
not move.

- `RUSTSEC-2099-0003.md` carries CVE-2025-47273's `CVSS:4.0` vector, as the `cvss` 2.2.0 crate's own
  regression test spells it (base score 7.7).
- `RUSTSEC-2099-0004.md` carries a `CVSS:3.1` vector missing its base metric `A`.

Measured 2026-09-26 on this box (s8c build, package D63): `cargo audit 0.22.2` and `cargo deny 0.20.2`
both load each advisory from a git-initialised copy and carry its vector verbatim in the per-advisory
`cvss` member, the malformed one included, so each reaches the adapter's scorer. The scorer
(`verbs::severity_of`) scores CVSS v3 only; each refuses the scan by name, `cvss-unscorable`, naming
the vector and its two remedies (record V = ADR-0170 decision 5; D21-63, provisional).

Everything here is LF under ADR-0164.
