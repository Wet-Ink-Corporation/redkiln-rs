---
id: "0148"
title: "A `tool` step naming `toolchain:test` **refuses by name** until a selection resolver exists — superseding **`0135` clause 1 in part**, at that clause's `test` → `Toolchain::test` row only. The executor called `toolchain.test(&[])`, and `rk-adapter-toolchain-rust` reads an EMPTY selection as `cargo test --workspace`, so all **28** shipped `toolchain:test` steps were whole-workspace runs under the 3600s bound on a shared target dir. The tempting repair — parse the `inputs.selection` key `catalog::Step` drops, and pass it through — is **worse than the defect**: measured over `.redkiln/catalog/` on 2026-09-22, all 28 declare a selection and it is `\"affected\"` (17), a `${…}` reference the resolver ADR-0136 clause 4 records as OWED (6), `artifact:…` (4) or `\"e2e\"` (1). **Not one is a test name**, so passing the string through runs `cargo test affected`, which matches nothing and exits **0** — a green from a toolchain that ran no tests. The other three rows of the map are untouched"
status: accepted
date: 2026-09-22
legacy_id: none
source: references/adr/0135 (clause 1's four-verb map, clause 2's by-name refusal — the precedent this reuses); references/adr/0136 (clause 4, the `${…}` resolver recorded as OWED); crates/rk-kernel/src/runtime/step.rs (`run_tool_step`, the call boundary); crates/rk-kernel/src/catalog.rs (the `[[step]]` loader, which reads ten keys and never `inputs`); crates/rk-adapter-toolchain-rust/src/lib.rs (the adapter that reads an empty selection as the whole workspace)
supersedes: ["0135"]
superseded_by: []
related: ["0093", "0099", "0134", "0135", "0136", "0137", "0139"]
evidence: []
consumers: []
---

## Context and problem

`run_tool_step` is the `Toolchain` port's first non-test call site. Its `test` row read:

```rust
"test" => toolchain.test(&[]).map(|_| ()),
```

The empty slice is not a neutral value. `rk-adapter-toolchain-rust` reads an empty selection as
*"select everything"* and builds `cargo test --workspace`. So every `toolchain:test` step in the
shipped catalog — **28** of them — was a whole-workspace test run, under the pack's 3600s bound,
against a shared `target/` directory.

The selection each of those steps declares is not missing from the catalog. It is dropped at load:
`catalog::Step`'s hand-written loader reads `id`, `kind`, `outputs`, `memo`, `question`, `run`,
`role`, `when`, `expr` and `for_each`, and **never `inputs`**. The catalog writes
`selection = "affected"` **19** times — 17 of them on a `toolchain:test` step — and nothing has ever
read one.

## Decision drivers

- **Parsing the key does not make it usable, and makes the failure quieter.** Measured over
  `.redkiln/catalog/*.toml` on 2026-09-22: 28 `toolchain:test` steps, **all 28** declaring
  `inputs.selection`, distributed `"affected"` × 17, a `${…}` reference × 6, `artifact:…` × 4,
  `"e2e"` × 1. `Toolchain::test` takes `&[String]` — a list of test selectors. None of those four
  shapes is one. `"affected"` is a *strategy* the toolchain would have to interpret; the `${…}`
  forms need the resolver ADR-0136 clause 4 records as OWED; `artifact:…` needs artifact
  resolution that does not exist either.
- **The naive fix produces the failure mode this programme keeps paying for.** Passing `"affected"`
  through as a test name runs `cargo test affected`, which matches **no test** and exits **0**. A
  filter that matches nothing is a green. That is strictly worse than the over-broad run it would
  replace, because the over-broad run at least goes red when the tree is broken.
- **The tree already has the right refusal shape, and clause 2 of ADR-0135 is it.** A refusal is an
  outcome, not a panic and not a skip; the surrounding retry and budget machinery treats it as it
  treats any other `StepError`.
- **Refusing costs nothing that was working.** `0` of 102 shipped activities complete a committing
  walk today, and nothing in the workspace drives `toolchain:test` through the executor.

## Considered options

**(a) Leave `toolchain.test(&[])`.** Rejected: 28 steps silently become `cargo test --workspace`
under a 3600s bound on a shared target dir, which is an invisible non-event of exactly the class
ADR-0135 was written to end.

**(b) Parse `inputs.selection` and pass the string to `Toolchain::test`.** Rejected, and this is the
one that had to be measured rather than reasoned about. `Toolchain::test` takes `&[String]` — test
selectors — and not one of the four shapes the catalog declares is one. `cargo test affected`
matches nothing and **exits 0**: a green from a toolchain that ran no tests, which is worse than the
over-broad run, because the over-broad run at least fails when the tree is broken.

**(c) Parse the key into `catalog::Step` but do not use it yet.** Rejected: it puts a value in the
type that nothing may act on, and leaves the one-line change from *parsed* to *passed through*
sitting there for the next reader who has not read this record.

**(d) Refuse by name until a resolver exists.** Taken, on ADR-0135 clause 2's own precedent.

## Decision

1. **`toolchain:test` refuses**, with a `StepError::Unsupported` whose `capability` names the
   unparsed `inputs.selection` and says why neither available behaviour is run. The other three rows
   of ADR-0135 clause 1's map — `build`, `lint`, `change_summary` — are **untouched**, and `test`
   stays inside `MAPPED_VERBS`.

2. **The refusal is distinguishable from clause 2's.** An unmapped verb is refused with the bare
   verb string (`capability: "publish"`); this one carries a sentence naming `inputs.selection`. A
   journal must be able to say which boundary a run hit, and collapsing the two would make the map's
   own edge unreadable.

3. **No key is invented and no selection is synthesised.** `catalog::Step` is not widened to parse
   `inputs`. Parsing it would put a value in the type that nothing may act on, which is the
   half-built state this record exists to avoid — and the one-line change from *parsed* to *passed
   through* would then be available to anyone who did not read this record.

4. **This is reversed by a resolver, not by a revert.** When step-output resolution lands
   (ADR-0136 clause 4), `test` regains a routing row with a real selection and this record is
   superseded. The test asserting the refusal says so in terms and asks to be **replaced rather than
   deleted**.

## Consequences

28 catalog steps that previously ran the whole workspace now refuse by name, and a run reaching one
ends there with a cause an operator can read. Nothing that completed before stops completing: no
shipped activity completed a committing walk at the time of this record.

The guard is `rk-kernel`'s `tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port`,
which reads two independent instruments — a counting `Toolchain` fake that must not move, and the
refusal's own text — with a positive control that the sibling verb `build` DOES reach the port.

**What this forbids**

- Calling `Toolchain::test` with an empty selection from the executor.
- Passing a declared `selection` string through as a test selector without a resolver.
- Refusing `test` with the bare-verb shape ADR-0135 clause 2 reserves for the 29 unmapped verbs.
- Widening `catalog::Step` to parse `inputs` without a record routing what may then read it.

## Evidence and confidence

**Measured, not argued.** The catalog census was taken by parsing every `[[step]]` block in all 102
`.redkiln/catalog/*.toml`, not by grep over lines: **28** steps carry `run = "toolchain:test"`,
**28 of 28** declare `inputs.selection`, and the distribution is `"affected"` 17, `${…}` 6,
`artifact:…` 4, `"e2e"` 1 — summing to 28. The loader's key list was read out of
`crates/rk-kernel/src/catalog.rs` itself; `inputs` is absent from it.

**Mutant-tested 2026-09-22.** Restoring `"test" => toolchain.test(&[]).map(|_| ())` turns
`tool_verb_routing::a_toolchain_test_step_refuses_by_name_and_does_not_reach_the_port` RED and
leaves the other **218** tests in the binary green — so this guard is the only thing in the
workspace that sees the change. The mutant was diffed and read before either colour was believed.

🛑 **AND ONE THING THIS RECORD'S OWN DELIVERABLE DOES NOT YET HAVE, found by an adversarial pass and
named rather than left.** *"Refuses BY NAME"* is asserted at the **kernel** boundary only. The
`capability` string lives in `StepError::Unsupported`, and `rk-cli`'s error arms print their own
sentence and drop the class rather than rendering it — so from a real `redkiln run` an operator is
told *"the dispatch of `<step>` did not commit"* and never sees `inputs.selection`. Measured:
`grep -rn "inputs.selection" crates/rk-cli/` returns **no hits**, and no CLI-level test asserts the
text is ever shown. The refusal is real and the exit row is right; its *visibility* is not, and that
is a rendering gap at the CLI boundary rather than a defect of this decision. It is carried as OWED
rather than repaired here, because widening it touches every `ReplayError` arm and is not this
stage's scope.

**Confidence.** High on the refusal and on the kernel-side assertion; explicitly **not** claimed for
operator visibility, per the paragraph above. A later resolver must supersede this record rather than
quietly restore the call, and the guard is written to demand that: its failure message asks to be
**replaced rather than deleted** if step-output resolution lands.
