# `ledger/` — the verify gate's AC evidence ledger fixtures (AC-333, AC-335, AC-345, AC-346)

Added at Step 7b-ii's verify-gate slice (`s7b-ii` §7b-ii.10). Every file is data; none is pointed at.

| file | used by | what it is |
| --- | --- | --- |
| `ac333_closed_status_enum.json` | `rk-store` `ac333_ac_evidence_ledger` | the five closed ledger statuses, spelled as the `ledger` companion schema emits them — the schema fixture "with no wildcard": the test asserts the exported schema's `kind` constants are EXACTLY this set |
| `ac333_file_path_only_row.json` | `rk-store` `ac333_ac_evidence_ledger` | the negative fixture: a `pinned` row whose locator names a file and nothing else — must fail validation (an AC is a claim about ONE ASSERTION, F25) |
| `ac333_spec_ac_ids.json` | `rk-store` `ac333_ac_evidence_ledger` | the fixture spec's AC id set (four ids) for the completeness check |
| `ac333_ledger_complete.json` | `rk-store` `ac333_ac_evidence_ledger` | the ledger over that spec: one `pinned`, one `mutation_survived`, one `unpinned` (materialised, not omitted), one `not_feasible` |
| `ac346_two_ac_spec.json` | `rk-kernel` `ac345_…`, `ac346_…`, `ac347_…` | the two-AC fixture spec: one AC with a declared mutation that flips, one whose mutant survives (the runs are scripted by the tests; the file is the author's declaration, ADR-0104 decision 1). *(dated 2026-09-29, s8g build, package S8G-P3-KERNEL-GIT: was `ac346_two_ac_spec.toml`, the retired `SpecAc` reader's form; now `rk_types::AcceptanceCriterion`'s serde form under `acceptance_criteria`, with the assertions its pins name under `assertions`, R2 = ADR-0192 decision 4)* |
| `ac455_spec.json` *(dated 2026-09-29, s8g build: was `ac455_spec.toml`; the same move)* | `rk-kernel` `ac455_policy_decides_not_the_type` | one fixture spec with an AC at `verification: None`; its two process packs — `../packs/ac455-requires-verification.toml` (refuses, naming the id) and `../packs/ac455-permits-unverified.toml` (the must-pass control) |
| `ac352_basis_pairs.golden.json` | `rk-kernel` `ac352_basis_reason_is_exhaustive` | the capture golden: the `(basis, reason)` pair for one run of each basis variant the comparator can produce — a kernel change that moves a run from one variant to another fails against it |

Beside this directory, the verify slice added: `../packs/ac337-mutation-infeasible.toml` (AC-337's `feasible = false` pack), `../gates/ac239_gate_eligible_adapters.toml` (AC-239's list, as data), `../lint-controls/ac335_status_reads_the_suite_verdict.rs` (AC-335's positive control), `../activities/ac347_verify_{clean,with_agent_step}.toml` (AC-347's pair), and `../findings/ac015_inversion/` (AC-334's F25 regression record).
