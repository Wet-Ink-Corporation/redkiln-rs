# `RK-D-TOOLCHAIN-CLAIM-TIE` - the rejecting fixture

**Severity:** `error` · **Pins:** AC-349 - two toolchain claims matching the same path at equal specificity

**The defect.** `.redkiln/registry.toml`'s `[toolchain.rust]` and `[toolchain.node]` each declare the claim
`**/*.toml` in their `claims` row (R1 = ADR-0191 decision 10, proposed). Every `.toml` path then matches a claim of
each toolchain at the same specificity, which AC-349 makes *"a configuration error reported at plan time, never a
silent choice"*, naming both rules and the path. The rule reports it at configuration time, over a witness path it
instantiates from the tied glob, through `rk_kernel::verify::claims::bind`, the binder the verify pass uses.
`Cargo.toml` is claimed by rust alone at a greater specificity, so it is no tie and is not reported.

*(Added 2026-09-29, `s8g` build, package S8G-P5-VERIFY-PASS.)*

## How this directory is used

`ac415_every_rule_id_has_a_rejecting_fixture` and this row's own test (`doctor::ac349_equal_specificity_claim_is_a_doctor_row`)
**materialise** it into a `tempfile` git repository before running the rule (`s7d` §7d.13). `doctor/clean/` carries
no `registry.toml`, so it claims nothing and the rule is silent over it.
