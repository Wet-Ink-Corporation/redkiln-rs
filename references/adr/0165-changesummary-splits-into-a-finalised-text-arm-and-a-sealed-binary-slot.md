---
id: "0165"
title: "`ChangeSummary` splits into a two-arm enum defined only in `rk-ports`: a `Text` arm (a typed diff plus a per-path manifest) finalised now and read from the object store through `rk-git`'s gix range read, and a `Binary` slot whose payload is sealed until S-BIN reports at `s8i`. An implementor of `Toolchain::change_summary` may import and name the type; nothing outside `rk-ports` defines, aliases or implements a trait for it. Supersedes ADR-0099 clause 5, ADR-0135 decisions 4 and 5, ADR-0093 decision 4, ADR-0094 decision 1 and ADR-0008's type-design sentence, each in part, and roots the conflict in D17-9."
status: accepted
date: 2026-09-24
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-41 (i) (the Checkpoint 5 record) and D21-42 decision 2 (split the type now, run S-BIN before exit), folded into one record as D21-42 says; D21-40's table (Checkpoint 5 red on its `ChangeSummary` line); spikes/DECISIONS-2026-09-17.md D17-9 (the root of the conflict) and runbook/PROGRESS.md §2's D17-1 readings 2 and 3; runbook/phase-1/s8a-records-and-red-checkpoints.md §8a.2 (the supersession table this record executes) and §8a.3; references/adr/0099 clause 5 and its forbids; references/adr/0135 decisions 4 and 5 and its third and fourth forbids bullets; references/adr/0093 decision 4; references/adr/0094 decision 1; references/adr/0008 Decision and its first forbids bullet; references/adr/0127 decision 6 (*"every bullet of its forbids stand"*); references/adr/0077 (*"gix owns the read path"*)
supersedes: ["0008", "0093", "0094", "0099", "0135"]
superseded_by: ["0199"]
related: ["0003", "0073", "0077", "0091", "0092", "0105", "0112", "0127", "0148"]
evidence: []
consumers: []
---

## Context and problem

Checkpoint 5 has been red on its `ChangeSummary` line since Step 7c, and three accepted records disagree about why.

- **ADR-0099 clause 5** reads *"`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until S-BIN
  reports; Checkpoint 5's grep for its absence covers `crates/` minus that one site."* Its forbids bullet reads
  *"Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports."*
- **ADR-0094 decision 1** gives both Phase-1 toolchain adapters a `change_summary`, *"`change_summary` remaining an
  opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)"*. An adapter that implements the port method
  must name the method's return type.
- **D17-9** (DECISIONS-2026-09-17) chose exactly that: both toolchain adapters' `change_summary` return
  `Err(StepError::Unsupported { capability })` from a dated registry row. In the same paragraph it declared
  *"`ChangeSummary` stays S-BIN's (ADR-0099 decision 5's forbids, unchanged …)"*. Those two things could not both
  hold. An implementor names the type in its `use` line and its signature, and ADR-0099's bullet bans the name
  everywhere outside `rk-ports`. **D17-9 is the root of the conflict.**
- **ADR-0135 decision 5** then described the ban and extended it: *"Checkpoint 5's grep bans the bare type outside
  `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths."* It added
  *"**Checkpoint 5's RED is untouched by this record.**"*

Measured at `b8e7aa9` (this stage's base), code lines only, string literals and the `Capability::ChangeSummary`
vocabulary variant excluded: **eight** sites name the type outside `rk-ports`. Every one is an import or the
signature of the port's own method:

- the `use` line and the `change_summary` signature in each of `crates/rk-adapter-toolchain-node/src/lib.rs`,
  `crates/rk-adapter-toolchain-rust/src/lib.rs` and `crates/rk-cli/tests/it/review_pass.rs` (six);
- `crates/rk-cli/src/verbs/run.rs`, on `UndetectedToolchain`, and `crates/rk-kernel/tests/it/runtime.rs`, on
  `CountingToolchain`, both written path-qualified as `rk_ports::toolchain::ChangeSummary` (two). The kernel test's
  doc comment says the name is not imported so as to pass the filter's `::ChangeSummary` exclusion.

The old filter sees six of the eight. None of the eight defines, aliases or implements anything for the type.

The type itself is also a dead end. `pub struct ChangeSummary(pub(crate) ())` cannot be constructed outside
`rk-ports`, so no real adapter can return `Ok`. The shipped catalog nevertheless **reads** members of the value:
`adversarial-review.toml`'s `clone` step has `inputs = { sha = "artifact:ChangeSummary.sha" }`, and
`change-summary.toml`'s `lint` step has `inputs = { diff = "${summarise.outputs.diff}" }`. Eleven shipped
`tool` steps name `toolchain:change_summary` (measured at `b8e7aa9`), and ADR-0135 decision 4 expects all eleven to
refuse.

S-BIN, which ADR-0008 makes the design fixture for the type, is about **binary** changes: asset manifests,
automation reports and captures on a real Unreal project. Nothing it measures bears on the shape of a change that
**is** a text diff, and that half is what every Phase-1 lane produces.

The operator took two decisions on 2026-09-24. D21-41 (i) asks for *"one ADR superseding ADR-0099 and ADR-0135 in
part"* that restates the rule as the type's home and opacity, lets implementors of `Toolchain::change_summary`
(test doubles included) name it in that signature, ratifies D17-1 and roots the conflict in D17-9 by name. D21-42
decision 2 asks to *"Split the type now, and run S-BIN before exit"*: the `Text` variant is finalised now, the
`Binary` variant stays sealed and non-constructible until S-BIN runs as a Phase-1 sub-stage, and the record is
*"folded with (i)'s Checkpoint 5 record"*. This record is that fold.

## Decision drivers

- **An accepted `## Decision` sentence and a forbids bullet move only by supersession** (ADR-0092 rule 1). Every
  sentence below that the split makes false is superseded in part here, including two that D21-42 decision 2 does
  not name (ADR-0094 decision 1 and ADR-0008).
- **The rule should protect what S-BIN is for.** S-BIN decides how a binary change is typed. It does not need the
  type's name kept out of the `use` line of the adapters that implement the method.
- **A value that resolves nothing is not a value.** The `Text` arm must carry every member the shipped catalog
  already reads (`sha`, `diff`, and `files` through the `Changeset` alias) plus the per-path manifest AC-497 asks for, or its finalisation is cosmetic. *(Amended 2026-09-25, s8a fix round 10: `files` added; see decision 3.)*
- **An empty or unreadable range must never yield an empty `Text`.** The kernel calls
  `toolchain.change_summary("")` today and discards the value, so an adapter that answered `Ok` to `""` would turn
  eleven refusals into eleven vacuous greens.
- **git stays confined to `rk-git`, and gix owns the read path** (ADR-0077, ADR-0091 decision 1). The range read
  must not shell out from an adapter, and must not add an exec op to `GitOp`.
- **A git-lfs pointer is not content.** The object store holds a pointer blob for an LFS-tracked path whatever the
  checkout's smudge configuration is, so a diff read from the object store says nothing about the asset.

## Considered options

### Option — split now: a finalised `Text` arm and a sealed `Binary` slot (taken)

`ChangeSummary` becomes an enum in `rk-ports` with two arms. `Text` is finalised over a two-commit fixture
repository and read through `rk-git`. `Binary` is a slot whose payload type cannot be constructed anywhere
outside `rk-ports`, and nothing inside `rk-ports` constructs it either. S-BIN designs its payload at `s8i`. Taken:
it gives every Phase-1 lane a real value, it keeps the binary design with the fixture ADR-0008 chose, and it is
what D21-42 decision 2 decided.

### Option — keep the opaque newtype and re-scope only the ban (D21-41 (i) as first drafted)

Restate the ban as the type's home and opacity and admit implementors, and change nothing else. Rejected by
D21-42 decision 2, which was taken later: the port would still return no `Ok`, the catalog's `sha` and `diff`
references would still resolve nothing, and every Phase-1 run would still refuse at eleven steps.

### Option — design the whole type now, `Binary` included

Rejected. It would lock the binary half against a synthetic stand-in, which is exactly what ADR-0008's first
forbids bullet exists to prevent and which this record does not move for the `Binary` arm.

### Option — leave the conflict and carry Checkpoint 5 as debt

Rejected by D21-40 decision 4: *"Checkpoints 5, 7b and 7f are repaired before Phase 1 exits. They are not carried
as debt …"* *(quotation corrected 2026-09-25 at `s8a` fix round 1: it closed the ledger's sentence with a full stop
where the ledger goes on, "even though none of them is on `s8`'s entry chain")*

## Decision

**1. Home and opacity.** `ChangeSummary` is defined **only in `rk-ports`**, as an enum with two arms.

- `Text` carries a typed diff plus a manifest, and is **finalised now**.
- `Binary` is a slot whose payload type is **sealed**. It cannot be constructed outside `rk-ports`, and no fake or
  adapter constructs it inside `rk-ports` either. S-BIN fills it at `s8i`.
- The two payload types are named `TextChangeSummary` and `BinaryChangeSummary`. If the implementation names them
  otherwise, Checkpoint 8a's patterns follow it in the same commit.

**2. Who may name the type.**

- No crate outside `rk-ports` defines, aliases, redefines, or implements a trait for any `…ChangeSummary` type,
  and none renames it on import (`…ChangeSummary as X`).
- An implementor of `Toolchain::change_summary` may import the type and name it in that method's signature. That
  includes production code and test doubles in any crate, `rk-kernel`'s `tests/` among them.
- A `Toolchain` implementor may construct the `Text` arm.
- **`rk-kernel`'s production code still names the type nowhere.** The surviving half of ADR-0135's fourth forbids
  bullet stands for `crates/rk-kernel/src/`.

**3. The serialised `Text` value carries the members the shipped catalog reads.**

- `sha`: read by `adversarial-review.toml`'s `clone` step (`artifact:ChangeSummary.sha`).
- `diff`: read by `change-summary.toml`'s `lint` step (`${summarise.outputs.diff}`).
- `manifest`: one row per changed path, carrying the path, its `classify` result and its change kind.
  *(Amended 2026-09-25 before acceptance, s8a fix round 15: since `s8a` fix round 14 a row's classification is
  `classify`'s PATH rules (`classify_by_path`), with the bytes decided on the range's blobs (`rk-git`'s
  `RangeEntry::binary`, decision 5) and never on the working tree. So a manifest row can differ from the
  `classify` verb's working-tree answer for the same path; the row is the range's answer.)*
- `files`: the manifest's paths, in manifest order, read by `mutation-testing.toml`'s `mutate` step (`artifact:Changeset.files`). It is serialised from `manifest` and is not a stored member, so an implementor cannot build a value whose `files` disagrees with its manifest.

*(Amended 2026-09-25, s8a fix round 10, while this record is `proposed`: the catalog also reads `artifact:Changeset.files`, `mutation-testing.toml`'s `scope` input. `.redkiln/artifacts.toml` declares `[artifact.Changeset]` with `schema = "ChangeSummary"`, and nine catalog activities bind `Changeset` to the value `toolchain:change_summary` returns, so `files` is a fourth member the catalog reads. The earlier enumeration counted references spelled `ChangeSummary` only and missed the `Changeset` alias.)*

**4. The range is read by `rk-git`, through gix.** The read is `rk_git::diff::range_diff`, which resolves both ends
of the range, takes the changed paths and their change kinds from gix's `Repository::diff_tree_to_tree`, and reads
each changed blob by object id. No `GitOp` variant is added and the read names no exec type, so ADR-0077 and
ADR-0105 clause 5 do not move. If the pinned gix cannot produce this read, the work halts; an exec read would need
a record superseding ADR-0077's read-path sentence in part, and this record does not take that decision.
`RustToolchain::change_summary`'s own body calls `range_diff`, and the adapter names git as a program nowhere.
*(Amended 2026-09-25, s8a fix round 7, while this record is `proposed`: the changed paths come from gix-diff's
tree diff, `gix::diff::tree`, over the two trees, and not from `Repository::diff_tree_to_tree`, which loads the
checkout's `.git/index` for a blob resource cache even with rename tracking off. An index under 20 bytes panicked
inside gix-index. The read stays a gix read of the object store; nothing else in this decision moves.)*
*(Amended 2026-09-25 before acceptance, s8a, D21-51: **the grammar.** A range is exactly `<base>..<head>`, two
dots, both ends present. A single revision, such as a bound `artifact:Changeset.sha`, is **not** a range: it is
refused by name (decision 6) and is never read as `R^..R`. The reason is step-grain commits: a changeset spans
several commits, so `R^..R` would answer a `Text` that omits every change before the last commit. `...` stays
refused as well. The shipped catalog does not yet bind a range: `.redkiln/catalog/change-summary.toml` and
`.redkiln/catalog/change-summary-ue.toml` both carry `range = "artifact:Changeset.sha"`, a single sha, and would reach the named
refusal once the kernel hands the bound value to the port (today it calls `change_summary("")`). The catalog edit that binds a two-dot range (for example `baseline..<sha>`) is OWED to
`s8d`, which owns the resolver that binds it, and lands under ADR-0072: a version bump, a changelog row and a
lattice re-run.)*

**5. An LFS pointer path is `Binary`, whatever `classify` answers.** A path is an **LFS pointer path** when its blob
at either end of the range has the git-lfs pointer form: under 1024 bytes, with the first line
`version https://git-lfs.github.com/spec/v1`. The read decides this on the blob, not on `.gitattributes`, because
an attribute can be absent at one end of a range or unset locally. Such a path enters the manifest as `Binary`,
and the range then refuses as any `Binary` path does, naming the path. What an LFS asset's summary carries is
`Binary`'s content, and so S-BIN's.
*(Amended 2026-09-25 before acceptance, s8a fix round 11, carrying §8a.2 item 2a's fix-round-1 rule into this
record: **a binary blob is decided on the blob too.** A path whose blob at either end has a NUL in its leading
bytes or is not UTF-8 is marked `binary` by `rk-git`'s range read, by the same test that writes git's
`Binary files … differ` line. It enters the manifest as `Binary` and the range refuses naming the path, whatever
`classify` answers: `classify` reads the working tree, and answers `Text` for a file the checkout does not hold.)*

**6. What the Rust adapter returns.**

- `Ok(ChangeSummary::Text(…))` when every path in the range classifies `Text` or `Generated`, and no path's blob
  at either end is an LFS pointer or binary (decision 5), and every changed path is UTF-8 *(amended 2026-09-25
  before acceptance, s8a fix round 11: the blob clause added; s8a fix round 14: the UTF-8 path clause added)*.
  *(Amended 2026-09-25 before acceptance, s8a fix round 15: "classifies" here and in the refusal bullet below is
  `classify`'s path rules (`classify_by_path`) with the bytes decided on the range's blobs (`RangeEntry::binary`),
  never on the working tree, as `s8a` fix round 14 made it; the `classify` verb is unchanged and can answer
  differently for the same path over a dirty tree or a checkout at a third commit. Decision 3's manifest note
  says the same.)*
- A refusal through the registry's `change_summary` row, re-dated and with its `reason` narrowed to the sealed arm,
  when the range carries a `Binary` path, an LFS pointer path or a path whose blob is binary (decision 5). The
  refusal names the path.
- A refusal by name for an empty or unresolvable range. **Never an empty `Text`.**
- A refusal through the same row **naming the revision and the grammar** for a single revision, one with no `..`:
  ``change_summary: `<rev>` is a single revision, not a <base>..<head> range``. **Never a one-commit summary.**
  *(Added 2026-09-25 before acceptance, s8a, D21-51: decision 4's grammar note gives the reason. A range that is
  otherwise malformed, such as `a..`, `..b` or `a...b`, keeps the bare refusal of the bullet above.)*
- `Err(Transport { Io })`, naming the object or path, when the range read fails on the store: a blob, tree or
  commit that cannot be read, a repository that is there but does not open (for example, a config that does not
  parse), or a changed path that is not UTF-8, which no manifest row can carry. These are `rk-git`'s
  `RangeError::Read`. *(Amended 2026-09-25 before acceptance, s8a fix round 14: `s8a` fix rounds 4, 7 and 8 put
  these three causes on this answer, and this decision listed three outcomes, not four.)*

The Node adapter's `change_summary` row and body are unchanged and still refuse. Finalising a Node body is not in
D21-42 decision 2 and is not taken here.

**7. The sealed slot is temporary by design.** It opens by a later record once S-BIN reports at `s8i`. When S-BIN
finalises `Binary`, `s8i` supersedes by name the three pins that guard the seal:
`change_summary::binary_arm_is_not_constructible_outside_rk_ports` with its `trybuild` case, and Checkpoint 8a's
guard (6) line asserting that the sealed payload declares no `pub` field, and guard (6)'s `sealed_open` floor,
which reads `rk-ports`' source for any impl block or construction that opens the payload, and for any `type`
alias or renaming `use` of it. *(Amended 2026-09-25 before acceptance, s8a fix round 9: an inherent impl
through an alias opened the payload with the floor green.)* *(Amended 2026-09-25
before acceptance, s8a fix round 8: this read **the two pins**; `s8a` fix round 7 added the floor.)* ADR-0092 rule 1 then decides whether that
later record supersedes this one in part.

**8. Ratification.** This record ratifies PROGRESS §2's **D17-1 readings 2 and 3**: a string literal, a comment and
the `Capability::ChangeSummary` vocabulary variant do not name the type. It names **D17-9** as the root of the
conflict: D17-9 chose adapter implementors while it declared *"ADR-0099 decision 5's forbids, unchanged"*, and
those two things could not both hold.

**9. What this record supersedes, each in part (ADR-0092 rule 1).**

- **ADR-0099 clause 5**, the sentence *"`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until
  S-BIN reports; Checkpoint 5's grep for its absence covers `crates/` minus that one site."*, and its forbids bullet
  *"Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports."* Clause
  5's crate-name sentences stand. **This overrides a forbids bullet that ADR-0127 affirmed**: ADR-0127 decision 6
  says of ADR-0099 that *"every bullet of its forbids stand"*. That affirmation stands for every other bullet of
  ADR-0099; this bullet alone is superseded here, and decision 2 above replaces it.
- **ADR-0135 decision 4**, *"**It must not be "fixed" into a synthesised value.**"*, and *(added 2026-09-25 at
  `s8a` fix round 1, before acceptance)* decision 4's other sentences, only as far as the Rust adapter over a
  non-empty range that changes only text: its bold lead *"**`change_summary` maps to the port method and thereby
  reaches the adapters' refusal — which is the correct behaviour and is not a defect to repair.**"*, *"Both real
  adapters return `Err(registry::refusal(&self.registry, Capability::ChangeSummary))` from a row dated `2026-09-18`
  whose probe is `none`, citing S-BIN and ADR-0099 clause 5."*, *"A `tool` step naming `toolchain:change_summary`
  therefore reaches the port and receives `Unsupported`."* and *"The 11 such steps in the shipped pack are expected
  to refuse."* Decision 4 stands for the Node adapter, and for the Rust adapter's refusal of an empty range and of a
  range carrying a `Binary` or LFS pointer path, so the kernel's `change_summary("")` still receives `Unsupported`
  today *(amended 2026-09-25, s8a fix round 6, while this record is `proposed`: of decision 4's sentences, *"Both real
  adapters return `Err(registry::refusal(&self.registry, Capability::ChangeSummary))` from a row dated `2026-09-18`
  whose probe is `none`, citing S-BIN and ADR-0099 clause 5."* is superseded in part for the Rust adapter as a
  whole, not only over a text range: decision 6 re-dates that row (to `2026-09-24`) and narrows its `reason` to this
  record, and the adapter refuses a `Binary` or LFS pointer path through `registry::refusal_naming`. That sentence
  stands for the Node adapter only. The Rust adapter's empty, `Binary` and LFS refusals still reach the port and
  receive `Unsupported`, so the other sentences keep the scope stated above)*; **decision 5**, *"Checkpoint
  5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding
  `::ChangeSummary` paths."* and *"**Checkpoint 5's RED is untouched by this record.**"*; its **third forbids
  bullet**, *"Forbids synthesising a `change_summary` value to route around the adapters' dated refusal, and forbids
  editing either adapter's `change_summary` row to make the step succeed."*; and its **fourth forbids bullet**,
  *"Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel`"*, **only** as far as a test double's impl
  signature under `tests/`. ADR-0135's map, decisions 1–3 and 6, and the ban on production `rk-kernel` stand. The
  `Text` value is read, not synthesised, and the Rust adapter's row is narrowed, not removed. *(Amended 2026-09-25,
  s8a fix round 5, while this record is `proposed`: decision 5's bold lead, *"**The kernel names `ChangeSummary` in
  no type position.**"*, and its clause *"without the type ever appearing in a signature or a binding in
  `rk-kernel`"* move with the fourth forbids bullet, in part and **only** as far as a test double's impl signature
  under `tests/`: decision 2 admits `rk-kernel`'s `tests/`, and `tests/it/runtime.rs`'s `CountingToolchain` names the
  type as its method's return type. Both stand for production `rk-kernel`, where the map's `change_summary` arm is
  still reached through a string literal and a method call and binds nothing.)*
- **ADR-0093 decision 4**, *"S-BIN blocks the design of `ChangeSummary` (AC-497, AC-356) and the Toolchain port's
  `change_summary` method body"*. S-BIN now blocks the `Binary` arm and its body only. Decision 4's scope rule and
  its forbids bullet stand unchanged. *(Amended 2026-09-25, s8a fix round 10, while this record is `proposed`:
  decision 4's bold lead, *"**S-BIN's gate is scoped to the `ChangeSummary` type, exactly as ADR-0008 words it.**"*, is superseded in part too, **only** as far as
  the `Text` arm: S-BIN's gate is now scoped to the `Binary` arm's payload and the `change_summary` body's
  `Binary` half, not to the whole type, and "exactly as ADR-0008 words it" no longer holds because this record
  supersedes ADR-0008's type-design sentence in part. "Decision 4's scope rule" above means the sentence that
  stands whole: *"it does not block Phase-1 commits, `rk-types`, `rk-store`, `rk-git`, the kernel or the adapters"*, with the design §10.2 sentence after it.)*
- **ADR-0094 decision 1**, the clause *"`change_summary` remaining an opaque non-finalised newtype until S-BIN
  (ADR-0093 decision 4)"*. D21-42 decision 2 does not name this sentence, but it is a `## Decision` sentence the
  split moves. Every other clause of decision 1 stands. *(Amended 2026-09-25 before acceptance, s8a, D21-51: this note read, from `s8a` fix round 5,
  that decision 1's *"`classify` and `change_summary` over `cargo metadata` and the changeset"* does not move, and
  that superseding it in part for the `Text` arm *"is the operator's decision, not taken here"*. The operator took
  it on 2026-09-25. **That clause is superseded in part here, only as far as `classify` and the `Text` arm.**
  `classify` is a function of the path and the bytes at that path, and spawns nothing: ADR-0029 (i) computes the
  change *"per item from affected paths via `Toolchain::classify`, never per repo"*, and `s8c` §8c.3 says of
  `classify` *"it spawns nothing"*. The `Text` arm is a function of the range, read from the object store by
  `rk-git` (decision 4). This amendment does **not** decide the `Binary` arm's inputs, which are S-BIN's at `s8i`,
  nor what `cargo metadata` is read for, which is `s8c` record T's. The dated note OWED against ADR-0094 decision 1
  since `s7c` Slice 6, for its `classify` and `change_summary` part, is discharged by this amendment. RT-26's
  separate OWED note against the same decision, the Rust adapter's `--list` count route, is untouched by it and
  stays owed. ADR-0094's rejection of a generic `cargo`/`npm` shell adapter, which its Considered options rest on
  *"`classify` and `change_summary` are toolchain-specific"*, now rests on what stays toolchain-specific: the
  build and test argv, nextest detection, the `--list` count route and the toolchain-keyed kinds (ADR-0073). It
  no longer rests on `classify` or the `Text` arm. Rule 1 of ADR-0092 is engaged, so ADR-0094's
  `superseded_by` entry for this record now covers that clause too; ADR-0094 stays `accepted`.)*
- **ADR-0008**, its `## Decision`'s *"`ChangeSummary` is designed against it"* and its first forbids bullet,
  *"`ChangeSummary` may not be designed and locked purely against a synthetic or paper fixture; it must be designed
  against the real UE project's actual artefacts."* These move **only** as far as the enum's two-arm shape and the
  `Text` arm, which this record finalises now, over a two-commit fixture repository, before S-BIN runs. D21-42
  decision 2 does not name ADR-0008 either. The `Binary` arm's payload is designed against the real UE project's
  artefacts at `s8i` and stays sealed until then. ADR-0008's UE fixture sentence, its Phase-2 adapter sentence and
  its second forbids bullet stand. *(Amended 2026-09-25, s8a fix round 3, while this record is `proposed`: the same
  `## Decision` paragraph restates the superseded phrase in its "Concretely:" sentence, as *"S-BIN … runs against it
  and feeds the `ChangeSummary` type"* and *"Phase 1 only requires `ChangeSummary` to be "designed against the UE
  fixture""*. Those two clauses move with it, in part and as far as the `Text` arm only: S-BIN feeds the `Binary`
  arm's payload, and Phase 1 requires that payload, not the whole type, to be designed against the UE fixture. The
  rest of that sentence stands: the UE fixture stood up in Phase 0, S-BIN's five items, *"not a working adapter"*,
  and the Phase-2 Unreal adapter.)*

Every superseded record stays `accepted`.

## Consequences

**Positive**

- Checkpoint 5 is repaired by amendment under this record, and its ban now names what this record forbids: a
  definition, an alias, a trait impl, an inherent impl or a renaming import outside `rk-ports`. The
  `::ChangeSummary` exclusion is dropped, so a path-qualified definition fires like a bare one.
- A Phase-1 run over a text range gets a real `ChangeSummary` whose `sha`, `diff`, `files` and `manifest` resolve
  the catalog's references *(amended 2026-09-25, s8a fix round 10: `files` added, decision 3)*.
- The binary design stays where ADR-0008 put it, on the real Unreal project, and runs before Checkpoint 8.

**Negative**

- The port gains a finalised half before S-BIN has run. If S-BIN finds that the manifest row needs a member the
  `Text` arm lacks, that is a later supersession of this record in part.
- Three pins (decision 7) are written to be superseded by name *(amended 2026-09-25 before acceptance, s8a fix
  round 8: this read **Two**)*. A reader who finds them gone after `s8i` must read
  that as the plan, not as drift.
- Checkpoint 7a's declaration line, Checkpoint 5's positive control and `s8`'s bullet and guard all read
  `pub struct ChangeSummary(`. Each moves in the commit that lands the split.

**What this forbids**

- Forbids defining, aliasing, redefining, renaming on import, or implementing a trait for any `…ChangeSummary` type
  outside `rk-ports`.
- Forbids naming the `ChangeSummary` type anywhere in `crates/rk-kernel/src/`.
- Forbids constructing the `Binary` arm, or its payload, anywhere outside `rk-ports`, and forbids any fake or
  adapter constructing it inside `rk-ports`, until a later record opens the slot.
- Forbids returning an empty `Text`, or any `Ok`, for an empty or unresolvable range.
- Forbids a `Text` whose manifest silently omits a changed path, and forbids a `Text` whose `diff` is a git-lfs
  pointer delta or carries a binary blob's change *(amended 2026-09-25 before acceptance, s8a fix round 11)*.
- Forbids reading the range by spawning git from an adapter, by any route, and forbids adding an exec op to
  `GitOp` for this read.
- Forbids designing or locking the `Binary` payload against anything but the real UE project's artefacts.
- Forbids citing ADR-0094 decision 1 as requiring a `cargo metadata` read in `classify` or in the `Text` arm, and
  forbids a `classify` body that spawns or reads anything but the path and the bytes it is handed. *(Added
  2026-09-25 before acceptance, s8a, D21-51: decision 9's ADR-0094 bullet. Pinned by the Rust adapter's
  `classify::classify_source_spawns_nothing`, a code-only lint over `src/classify.rs`, and
  `classify::classify_is_invariant_to_a_cargo_manifest`.)*

## Evidence and confidence

- [measured 2026-09-24 at `b8e7aa9`] `git grep -n ChangeSummary` over `crates/*.rs` outside `rk-ports`, after
  removing comment lines, string literals and the vocabulary variant: **8** lines, the six and the two listed in
  Context.
- [measured 2026-09-24 at `b8e7aa9`] `.redkiln/catalog/adversarial-review.toml` carries
  `inputs  = { sha = "artifact:ChangeSummary.sha" }`; `.redkiln/catalog/change-summary.toml` carries
  `inputs  = { diff = "${summarise.outputs.diff}" }`; **11** `toolchain:change_summary` occurrences across
  `.redkiln/catalog`.
- [measured 2026-09-24] the workspace pins `gix = "=0.87.1"`, and `gix-0.87.1/src/repository/diff.rs` declares
  `pub fn diff_tree_to_tree`. The runbook's stage file records that gix's default `basic` feature carries
  `blob-diff`.
- [measured 2026-09-24 at `b8e7aa9`] `.gitattributes` carries no `filter=lfs` row, so this repository tracks
  nothing through LFS and decision 5 is proved on a fixture repository.
- [documented] D17-9's paragraph, D21-41 (i) and D21-42 decision 2, quoted above; ADR-0127 decision 6's
  *"every bullet of its forbids stand"*.
- **What this evidence does not license.** It does not license any claim about the `Binary` arm's shape: S-BIN is
  unrun. AC-154 and AC-356 stay PROVISIONAL on the `Binary` half, and AC-497's port shape is asserted for the `Text`
  half only until S-BIN reports.

Confidence is high for the conflict and its root, which are textual. It is moderate for the `Text` arm's field set,
which is fixed by the catalog's three member reads and AC-497's manifest and has not yet met a real run. *(Amended 2026-09-25, s8a fix round 10: this read "the catalog's two references"; `artifact:Changeset.files` is the third, decision 3.)*
