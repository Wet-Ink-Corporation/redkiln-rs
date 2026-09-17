# `ledger/` — the verify gate's AC evidence ledger fixtures (AC-333, AC-335, AC-345, AC-346)

Added at Step 7b-ii's verify-gate slice (`s7b-ii` §7b-ii.10). Every file is data; none is pointed at.

| file | used by | what it is |
| --- | --- | --- |
| `ac333_closed_status_enum.json` | `rk-store` `ac333_ac_evidence_ledger` | the five closed ledger statuses, spelled as the `ledger` companion schema emits them — the schema fixture "with no wildcard": the test asserts the exported schema's `kind` constants are EXACTLY this set |
| `ac333_file_path_only_row.json` | `rk-store` `ac333_ac_evidence_ledger` | the negative fixture: a `pinned` row whose locator names a file and nothing else — must fail validation (an AC is a claim about ONE ASSERTION, F25) |
| `ac333_spec_ac_ids.json` | `rk-store` `ac333_ac_evidence_ledger` | the fixture spec's AC id set (four ids) for the completeness check |
| `ac333_ledger_complete.json` | `rk-store` `ac333_ac_evidence_ledger` | the ledger over that spec: one `pinned`, one `mutation_survived`, one `unpinned` (materialised, not omitted), one `not_feasible` |
| `ac346_two_ac_spec.toml` | `rk-kernel` `ac345_…`, `ac346_…` | the two-AC fixture spec: one AC with a declared mutation that flips, one whose mutant survives |
| `ac455_spec.toml`, `ac455-requires.toml`, `ac455-permits.toml` | `rk-kernel` `ac455_policy_decides_not_the_type` | one fixture spec with an AC at `verification: None`; two process packs, one whose policy requires a verification at the stage and one whose policy does not (the must-pass control). The packs live under `../packs/` |
