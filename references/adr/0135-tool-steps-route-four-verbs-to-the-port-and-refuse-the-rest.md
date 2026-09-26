---
id: "0135"
title: "A `tool` step's `run = \"toolchain:<verb>\"` routes to the `Toolchain` port by a **closed four-verb map** — `test`, `build`, `lint`, `change_summary` to the port methods of the same name — and **every other verb is refused** with `StepError::Unsupported { capability: \"<verb>\" }`; this discharges `resolver.rs`'s OWED routing record for the `tool` arm only, invents no port method for the 29 catalog verbs that have none, and does not make the adapters' five-token `Capability` registry into a verb table it is not"
status: accepted
date: 2026-09-22
legacy_id: none
source: crates/rk-kernel/src/resolver.rs:63-71 (*"nothing in this tree says which capability a given activity needs … a record routing the need from the catalog or the pack to the dispatch site is OWED"*); references/2026-09-08-architecture-design.md §4.5 (*"`tool` (a toolchain verb through the port)"*; the five registries row naming `registry.toml`); references/adr/0003 and references/adr/0094 decision 1 (the `Toolchain` port's six methods); references/adr/0093 decision 4 and references/adr/0099 clause 5 (`ChangeSummary` opaque until S-BIN); crates/rk-ports/src/toolchain.rs:44-64; crates/rk-kernel/src/workflow/memo.rs:948-959 (`verb_of`, the existing verb parse); crates/rk-adapter-toolchain-rust/src/registry.rs:62-75 (the five-token `Capability` vocabulary); runbook/phase-1/s7g-step-executor.md §7g.2
supersedes: []
superseded_by: ["0148", "0165", "0170"]
related: ["0003", "0073", "0093", "0094", "0099"]
evidence: []
consumers: []
---

## Context and problem

`s7g` §7g.2 describes the `tool` arm as `run = "toolchain:<verb>"` *"resolved against the capability
registry"*. **No such resolution exists on this tree, and the thing called the capability registry is not
that table.** Three separate artifacts have been conflated:

1. **The adapters' `Capability` enum** (`rk-adapter-toolchain-rust/src/registry.rs:62-75`, byte-equal across
   four adapter crates) is a **five-token closed vocabulary** — `dispatch`, `snapshot`, `rebuild`,
   `change_summary`, `affected-selection`. It exists to carry **dated probe rows** saying whether an adapter
   has been verified to support a capability. It carries **no `test`, `build`, `lint` or `classify` row**,
   and it is not indexed by catalog verb.
2. **The `[verb]` table** — 38 verbs with `outputs`, `live` and `timeout` — lives at
   **`spikes/p9/registry.toml` only**. `find . -name registry.toml` returns exactly that one hit: it is
   **not shipped in `.redkiln/`**, although design §4.5 lists `registry.toml` among the five registries. Its
   only kernel consumer is AC-018's memo-liveness lint. It maps a verb to an **output type and a liveness
   flag**, never to a port method.
3. **The port's six methods** (`rk-ports/src/toolchain.rs:44-64`) — `detect`, `build`, `test`, `lint`,
   `classify`, `change_summary`.

`rk-kernel/src/resolver.rs:63-71` states the gap in terms and forbids closing it by invention:

> … and `rk-types` is Checkpoint 5's and may not be edited here. So **nothing in this tree says which
> capability a given activity needs**, and inventing the channel — a new `ActivitySpec` field, a new catalog
> key, or a mapping from a `tool` step's `run = "toolchain:X"` — would be deciding what no record has. …
> the need arrives as data, and **a record routing the need from the catalog or the pack to the dispatch
> site is OWED**.

This record is that routing, for the `tool` arm.

**The shape of the demand, measured 2026-09-22 over `.redkiln/catalog/`'s 102 activities:** **113 `tool`
steps** across **33 distinct verbs**. (`s7g` §7g.2's *"12 verbs"* is wrong; the 113 is right, and the stage
file is corrected in the same pass as this record.) Of the 33, **four name a port method**: `test` **28**,
`change_summary` **11**, `build` **4**, `lint` **4** — 47 of the 113 steps. The remaining **29 verbs have no
port method at all**: `verdict_record` 8, `metrics` 6, `deploy` 6, `smoke` 4, `rollback` 4, `release_record`
4, `cook` 4, `ue_automation` 3, `sca_scan` 3, and twenty more at 1–2 each. The port's other two methods,
`classify` and `detect`, appear on **zero** run lines.

**The kernel already parses the verb and already special-cases one by name.** `workflow::memo::verb_of`
(`memo.rs:948-959`) strips the `toolchain:` prefix and the optional `?` feed suffix; `catalog.rs:196` reads
`self.run.as_deref() == Some("toolchain:test")` to mint `Evidence::GreenAt`. What is missing is only the
step from verb to port method.

## Decision drivers

- AC-473 needs a `tool` step to reach `rk-adapter-toolchain-rust` behind the port or Phase-1 exit criterion
  (1) cannot be satisfied. The largest verb on the pack, `toolchain:test` at 28 steps, maps to a port method
  directly — so the criterion is reachable without deciding anything about the other 29.
- `resolver.rs`'s prohibition is against **invention**, not against routing. A map over verbs that already
  name existing port methods invents nothing; a map that gave `deploy` or `cook` a port method would invent
  a great deal.
- A silent fallthrough is the defect this programme keeps paying for. An unmapped verb that no-ops, or that
  is skipped, would make 66 of 113 `tool` steps into invisible non-events — a run that "succeeded" having
  executed nothing.
- The tree already has the right refusal shape and it is load-bearing elsewhere: `registry::refusal` returns
  `StepError::Unsupported { capability }` from a **dated** row, and `change_summary` uses it today.
- `ChangeSummary` is blocked (ADR-0093 decision 4, ADR-0099 clause 5) and must stay blocked. Whatever the
  map does with `change_summary`, it must not route around S-BIN.

## Considered options

1. **A closed four-verb map plus a typed refusal for the rest (taken).**
2. **Ship `spikes/p9/registry.toml` into `.redkiln/` and resolve through it.** Rejected **as the routing
   mechanism**, not as an idea: the table maps a verb to `outputs`/`live`/`timeout`, which is a different
   question, so a verb→method mapping would still be owed on top of it. That the pack lacks a
   `registry.toml` at all, against design §4.5's five-registry row, is a **real and separate gap**, recorded
   here as such and not filled speculatively.
3. **Map all 33 verbs.** Rejected: 29 have no port method, so this means minting port surface the design
   does not have — precisely what `resolver.rs:63-71` forbids, and a sixth, seventh and thirtieth method on
   a port whose method count Checkpoint 7a asserts at six.
4. **Treat an unmapped verb as a no-op and continue.** Rejected: it converts 66 of 113 steps into silent
   successes. A step that does nothing and reports nothing is indistinguishable from a step that worked,
   which is the shape `halts.md` Halt 8 enumerates.
5. **Refuse the whole `tool` arm until a verb registry ships.** Rejected: it leaves criterion (1) unmet for
   a reason unrelated to the 47 steps that *can* run, and it defers the port's first call site — the thing
   `s7g` exists to build — behind a pack authoring question.

## Decision

1. **The map is closed and holds exactly four entries**, verb to port method, same spelling on both sides:
   `test` → `Toolchain::test`, `build` → `Toolchain::build`, `lint` → `Toolchain::lint`,
   `change_summary` → `Toolchain::change_summary`. The map is **total over its domain and exhaustive in
   source** — no wildcard arm producing a default, on the tree's established idiom
   (`retry.rs`'s `ac028_policy_match_has_no_wildcard` is the precedent lint).

2. **Every verb outside the four is refused, by name, with
   `StepError::Unsupported { capability: "<verb>" }`.** The refusal carries the verb as it was written, so a
   run's journal names which of the 29 was reached. A refusal is an **outcome**, not a panic and not a skip:
   the step's failure is the step's result and the surrounding retry/budget machinery treats it as it treats
   any other `StepError`.

3. **`classify` and `detect` are in the port and NOT in the map.** They name port methods but appear on zero
   run lines, and `detect`'s and `classify`'s call shapes (a bool; a per-path classification) have no `tool`
   step demanding them. Adding either is a later act with its own warrant; adding them now on the grounds
   that the method exists would be inventing a demand to fit a supply.

4. **`change_summary` maps to the port method and thereby reaches the adapters' refusal — which is the
   correct behaviour and is not a defect to repair.** Both real adapters return
   `Err(registry::refusal(&self.registry, Capability::ChangeSummary))` from a row dated `2026-09-18` whose
   probe is `none`, citing S-BIN and ADR-0099 clause 5. A `tool` step naming `toolchain:change_summary`
   therefore reaches the port and receives `Unsupported`. **It must not be "fixed" into a synthesised
   value.** The 11 such steps in the shipped pack are expected to refuse.

5. **The kernel names `ChangeSummary` in no type position.** Checkpoint 5's grep bans the bare type outside
   `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths. The map's
   `change_summary` arm is reachable through a string literal and a method call without the type ever
   appearing in a signature or a binding in `rk-kernel`. **Checkpoint 5's RED is untouched by this record.**

6. **The 29 unmapped verbs are recorded as an open gap, owned by no stage in this phase.** They are a
   pack-and-port question — whether the catalog's verbs are too many, whether the port is too few, or
   whether some of them belong to a different port entirely — and this record deliberately does not answer
   it. What it refuses to do is hide it: after this record, a run that reaches one of the 29 says so.

**What this record does NOT decide.** It does not decide the verb registry's home or whether `registry.toml`
ships. It does not decide which toolchain **adapter** serves an instance — that is `config.toml`'s
`[adapters] toolchains` and ADR-0073's keyed expansion, and is not re-decided here. It adds no method to the
`Toolchain` port: the count stays **six**, as Checkpoint 7a asserts. It does not settle `resolver.rs`'s
`CapabilityNeed` routing for the **agent** arm, which remains OWED.

## Consequences

**What this forbids**

- Forbids a wildcard or default arm in the verb map. An unmapped verb is refused by name, never routed to a
  "closest" method and never silently dropped.
- Forbids treating an unmapped verb as a success, a skip or a no-op. 66 of 113 `tool` steps name one, and a
  silent pass over them would report a run that executed nothing as a run that worked.
- Forbids synthesising a `change_summary` value to route around the adapters' dated refusal, and forbids
  editing either adapter's `change_summary` row to make the step succeed. S-BIN is unrun.
- Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel` — in a signature, a binding, a generic
  argument or a return type. The arm is written through a string literal and a method call.
- Forbids adding a port method to satisfy a catalog verb. The port's six are ADR-0003's and ADR-0094's, and
  Checkpoint 7a counts them.
- Forbids reading the adapters' `Capability` enum as a verb table, or extending it with catalog verbs. Its
  five tokens carry dated probe rows about adapter support and are a different question.

**What follows**

- `ports.toolchain` gains its first non-test call site, which is what
  `conformance_every_port_has_a_call_site` measures and what ADR-0134 narrows around.
- AC-473's clause becomes reachable: a `toolchain:test` step can run through
  `rk-adapter-toolchain-rust` behind the port, which is Phase-1 exit criterion (1)'s requirement.
- **The 29-verb gap becomes visible on every run that reaches one**, where today it is invisible because no
  `tool` step executes at all. Expect refusals; they are the instrument.
- The pack's missing `registry.toml` is now recorded against design §4.5's five-registry row as a known,
  unfilled gap.

## Evidence and confidence

- [measured 2026-09-22] `.redkiln/catalog/`: 102 activities, 574 steps — 185 `agent`, **113 `tool`**, 94
  `check`, 80 `gate`, 102 `emit`. Every `run =` line in the pack is a `toolchain:` line.
- [measured 2026-09-22] The 113 `tool` steps name **33 distinct verbs**. Four name port methods: `test` 28,
  `change_summary` 11, `build` 4, `lint` 4 (**47 steps**). Twenty-nine do not (**66 steps**). `classify` and
  `detect` appear on **0** run lines.
- [measured 2026-09-22] `find . -name registry.toml` → exactly one hit, `./spikes/p9/registry.toml`. The
  shipped `.redkiln/` carries no verb registry.
- [documented] `rk-adapter-toolchain-rust/src/registry.rs:62-75` — `Capability` is five tokens:
  `dispatch`, `snapshot`, `rebuild`, `change_summary`, `affected-selection`.
- [documented] Both adapters' `change_summary` rows: `"verified_on": "2026-09-18"`, `"probe": { "none": {
  "reason": "S-BIN is unrun and rk_ports::ChangeSummary is an opaque newtype whose contents no record has
  decided (D17-9; ADR-0099 decision 5); no Ok is constructible and no probe exists" } }`.
- [documented] `resolver.rs:63-71` is quoted above and is this record's named prompt.
- **Confidence: high on the four-verb map, which is forced — the verbs and the methods share a spelling and
  there is no third candidate.** What is **not** decided, and is stated as open rather than assumed away,
  is whether the 29 unmapped verbs should acquire port methods, move to another port, or leave the catalog.
  No measurement here bears on that, and this record does not anticipate it.
