---
id: "0102"
title: rust-version equals the pin, a member manifest's [lints] is workspace = true and nothing else, the only in-source override is #[expect], and the nursery and error-handling evidence lines are restated
status: accepted
date: 2026-09-11
legacy_id: none
source: references/2026-09-10-pre-implementation-audit.md PIA-023 (§4.3), PIA-025, PIA-026 (§4.2), PIA-027 (§4.1), PIA-079 (§4.3), PIA-024; references/adr/0083 §Decision, §Consequences, §Evidence; Cargo Book Workspaces and Lints pages, Clippy Lints book, RFC 3389, RFC 3537, cargo #13157 (fetched 2026-09-10)
supersedes: ["0083"]
superseded_by: []
related: ["0076", "0084", "0099", "0100"]
evidence: ["PIA-023", "PIA-025", "PIA-026", "PIA-027", "PIA-079", "PIA-024"]
consumers: []
---

## Context and problem

ADR-0083's toolchain and lint policy is right in substance and wrong in four places an implementer hits at Step 5.1.

- **A per-crate `[lints]` override does not exist.** ADR-0083 says a crate needing `unsafe_code` "overrides in its own
  manifest's `[lints]` with a comment"; Cargo rejects any `[lints]` table that mixes `workspace = true` with other
  keys (RFC 3389; cargo #13157 open), so the only override route is the in-source attribute the same record permits
  two lines later [documented, PIA-026].
- **The root manifest as the runbook lifts it fails `cargo clippy`** on the pinned toolchain: group entries beside
  individual lints need an explicit `priority` (`lint_groups_priority`) [measured, PIA-025].
- **`package.rust-version` is not a substitute for the pin — but omitting it leaves edition 2024's MSRV-aware resolver
  inert**; ADR-0083 rejected substitution, not co-existence [documented, PIA-027].
- **Three evidence lines misquote their sources.** The nursery clause ("can change or be removed without notice") is
  in no Clippy text, and `nursery = "warn"` *as a group* is what the Lints book advises against; "pin for a binary you
  control" is RFC 3537's framing, not swatinem's; and "`thiserror`/`anyhow` … corroborated by no first-party source" is
  false — dtolnay's own `anyhow` README ("Comparison to thiserror") and the `thiserror` docs document the split, and
  Microsoft's M-APP-ERROR names it with one divergence (in-repo application-only crates may use `anyhow`)
  [documented, PIA-023, PIA-079]. In-source overrides should be `#[expect(lint, reason)]`, which fails when the lint
  stops firing, rather than `#[allow]`.

The pin itself moves: the operator chose **1.98.1** (stable since 2026-09-03) over the spike programme's 1.97.1, with
the instrument move recorded per golden constraint 8 (PIA-024). R-ASYNC — whether 1.97/1.98 stabilised
`async_fn_in_dyn_trait` — is discharged negative through 1.98.1.

This record supersedes ADR-0083 **in part**: the `[lints]`-override sentence, the `rust-version` sentence, the
in-source-override rule, and the three evidence lines. The toolchain pin rule, the enumerated pedantic subset, the
consolidated integration tests, the `set_var` ban and the kernel-reads-no-environment rule stand; ADR-0083 stays
`accepted`. The 1.98.1 value is a runbook decision (the pin is operator-facing), recorded in DECISIONS-2026-09-11.

## Decision drivers

- A rule Cargo rejects is a rule nobody follows; the record must name the route that works.
- `#[expect]` turns a stale override into a compile error; `#[allow]` turns it into silence.
- A floor and a pin answer different questions and co-exist; the resolver reads only the floor.
- An evidence line that borrows authority its source does not give is the defect ADR-0085 rule 3 was written against.

## Decision

1. **`workspace.package.rust-version` is set equal to the pinned channel** and inherited by every member. It exists to
   arm resolver 3's MSRV fallback and `cargo add`'s version choice, not as a consumer floor; the pin-bump PR bumps both.
2. **A member manifest's `[lints]` table is `workspace = true` and nothing else.** The **only** per-crate exception to
   `unsafe_code = "deny"` is a commented `#![allow(unsafe_code)]` crate- or module-level attribute — the route
   ADR-0099's Job Object module takes — and it is the only `allow` permitted in source.
3. **Every other in-source override is `#[expect(clippy::<lint>, reason = "…")]`**, never `#[allow]`, so an override
   that stops being needed fails the build.
4. **Group entries carry explicit priority** in the root manifest: `all = { level = "deny", priority = -1 }`,
   `nursery = { level = "warn", priority = -1 }`, individual lints at default priority above them.
5. **The nursery clause is restated on its source:** the Clippy Lints book documents `nursery` as "new lints that are
   still under development" and false-positive-prone; the group is set to `warn` and never `deny`; "can change or be
   removed without notice" is withdrawn as a quotation.
6. **The error-handling split is re-tagged [documented, 1P crate author; vendor-corroborated, one named divergence]:**
   `thiserror` in library crates, `anyhow` confined to `rk-cli` and `redkiln-migrate`; the stricter "a library's public
   API never returns `anyhow::Error`" is kept as the project's own rule (adapter crates are shared libraries), and the
   divergence from M-APP-ERROR is stated. `anyhow` is at 1.0.104.
7. **The pin-vs-floor evidence is re-attributed:** the framing is RFC 3537's; swatinem's post corroborates.

## Considered options

### Option — keep `#[allow]` for overrides
Rejected: it is the form under which a lint that stopped firing goes unnoticed; `#[expect]` is stable and does the
same thing loudly.

### Option — make `rust-version` a floor below the pin
Rejected: two numbers to explain; the MSRV fallback works identically when floor equals pin, and the bump PR moves one
line fewer.

### Option — set `nursery = "allow"` to make `-D warnings` clean
Rejected: CI gates on exit code (ADR-0083); nursery warnings are visible and non-blocking, which is what a group under
development should be.

## Consequences

**Positive**
- Step 5.1's manifest compiles and lints on the pinned toolchain; every override is either the one `unsafe` allow or an
  `#[expect]` that self-retires.

**Negative**
- Two numbers move together on every toolchain bump.
- 1.98.1 is one recorded instrument move from every spike measurement; S-PORTS runs on 1.98.1 and says so.

**What this forbids**
- Forbids any key but `workspace = true` in a member's `[lints]`.
- Forbids `#[allow]` in source except the commented `unsafe_code` exception.
- Forbids omitting `workspace.package.rust-version` or letting it differ from the pin.
- Forbids `nursery` at any level but `warn`.
- Forbids describing the `thiserror`/`anyhow` split as unsourced.

## Evidence and confidence

- [documented, Cargo Book Workspaces; RFC 3389; rust-lang/cargo #13157 open] `[lints] workspace = true` admits no
  sibling keys.
- [measured, PIA-025] `cargo clippy` on the pinned toolchain rejects the group/lint mix without `priority`.
- [documented, Clippy Lints book, fetched 2026-09-10] nursery's actual description; all five enumerated pedantic lints
  present.
- [documented, docs.rs/thiserror 2.0.20; dtolnay/anyhow README; Microsoft M-APP-ERROR / M-ERRORS-CANONICAL-STRUCTS,
  fetched 2026-09-10] the split, and its named divergence.
- [documented, RFC 3537] the pin/floor distinction; [blog, swatinem] corroboration.
- [documented, Rust release notes 1.97.0/1.98.0/1.98.1] no dyn-async-trait stabilisation; `async_fn_in_dyn_trait`
  nightly-only (rust-lang/rust #133119).
- **What this evidence does not license.** It does not license claiming 1.98.1 changes any spike result; the move is
  recorded, not re-measured. It does not license `nursery` at `deny` on any future toolchain.
