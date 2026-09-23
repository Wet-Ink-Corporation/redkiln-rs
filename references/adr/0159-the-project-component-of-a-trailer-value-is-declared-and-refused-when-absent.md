---
id: "0159"
title: "**The `<project>` component of a trailer value is DECLARED, in a new `[project] name` key, and a run that would mint a trailer without one REFUSES by name.** design §5.4 fixes both axes' values as `<project> \"/\" <slug>`, and ADR-0155 fixes the slug as the `.redkiln/items/` file stem — but **nothing in this repository declares a project**: `grep -rn project .redkiln/config.toml` returns nothing, no `[project]` table exists, and `ItemRecord` has no such field. The four candidate sources are each wrong for a stated reason — the pack id is not a project, the item-id prefix is a naming convention rather than a declaration, the directory name is derived from a path this crate is forbidden to read, and an empty project makes `split_qualified` return `None`, which silently downgrades every `Story:` to a `bare_story_ids` entry that `reconcile` counts and never treats as a transition. So it is declared, and its absence refuses rather than defaulting"
status: proposed
date: 2026-09-22
legacy_id: none
source: references/2026-09-08-architecture-design.md §5.4 (line 693-694, both axes' `<project> "/" <slug>` values); crates/rk-git/src/trailer/grammar.rs (`split_qualified` at 224-228 — `(!p.is_empty() && !s.is_empty())`, so an empty project yields `None` and `classify` produces `ParsedTrailer::BareStory`); crates/rk-git/src/reconcile.rs (`ParsedTrailer::BareStory` at 126-128 — recorded into `bare_story_ids`, never a transition); crates/rk-cli/src/doctor/rebuild.rs (165-202 — `item = format!("{}/{}", project.project, slug)`, so whatever is emitted IS the project, and the slug must be the item file's stem); .redkiln/config.toml (the whole file — `[adapters]`, `[packs]`, `[doctor]`, and no project); crates/rk-git/src/worktree.rs (the module header — *"There is no call to `std::env::current_dir` in this crate"*); references/adr/0155 (which fixes the slug and leaves the project open)
supersedes: []
superseded_by: []
related: ["0105", "0107", "0109", "0155", "0156"]
evidence: []
consumers: []
---

## Context and problem

ADR-0155 settled how many `Story:` trailers a lane mints and what the slug must be. It did not say where
the **project** comes from, because until the producer was written there was nothing that needed one.

design §5.4 gives both axes a two-part value:

> | `Story:` | `<project> "/" <slug>` — one item id | Column 0 … |
> | `Slice-Verdict:` | `<project> "/" <slice> SP <verdict>` — exactly two whitespace-separated tokens | … |

**Nothing in this repository declares a project.** `.redkiln/config.toml` carries `[adapters]`,
`[packs]` and `[doctor]` and no fourth table; `grep -rn 'project' .redkiln/config.toml` returns nothing;
`ItemRecord` (`run.rs`) has `id`, `kind`, `stage` and `[facets]`.

And the failure mode of getting it wrong is silent, which is what makes this worth a record rather than
a choice at the call site. `split_qualified` (`grammar.rs:224-228`) is:

```rust
let (p, s) = v.split_once('/')?;
(!p.is_empty() && !s.is_empty()).then(|| (p.to_string(), s.to_string()))
```

An empty project means no `(project, slug)` pair, so `classify` returns `ParsedTrailer::BareStory`, and
`reconcile` files that into `bare_story_ids` — *"recorded as `bare_story_ids`"* per design §5.4's own
rule-for-the-rest column, **never a transition**. A producer that emitted `Story: /rk-fix-doctor-test-count`
would write a real commit, pass a `grep '^Story:'`, and contribute nothing to any state. That is the
"filter that matches nothing, read as a clean zero" shape one layer along.

## Decision drivers

- The value is the interface between the producer and two readers (`reconcile` and `doctor --rebuild`),
  and `rebuild.rs:181` makes whatever is emitted *become* the project: `item = format!("{}/{}",
  project.project, slug)`. There is no authority to check it against, so the only protection is that it
  was declared on purpose.
- This tree's settled doctrine is that an absence is not a value. `Pack::rigor` returns `None` for an
  unmentioned stage — *"an absence, which is not a `forbidden` and not an `optional`"*; the runtime
  envelope refuses rather than defaulting; `run.rs:550` refuses an item record with no stage rather than
  defaulting one. A project invented in code would be the first value in this path that nobody wrote.
- A silently-degrading emission is worse than a refusal, because the commit still lands and the trailer
  still looks well-formed.
- `[project]` is a new key in a file `redkiln init` writes, and ADR-0107 decision 2 says a second `init`
  leaves it untouched — so an existing repository will not gain the key by re-running `init`, and the
  refusal has to name the key and the file for the operator to fix it.

## Considered options

**1. The pack id** (`bug-standard`). Available, non-empty, and wrong: a pack is a process, not a project,
and two items under one project would be filed under different projects purely because their kinds
derived different lanes. It would also make the project change when a lane is re-derived.

**2. The item id's prefix** (`rk-` from `rk-fix-doctor-test-count`). A naming convention, not a
declaration: nothing validates it, `story` items would share it with `fix` items, and an item named
without the prefix would produce an empty or nonsense project.

**3. The repository directory name** (`redkiln-rs`). Derivable, and it reads the one thing `rk-git` is
forbidden to read — its own header states *"There is no call to `std::env::current_dir` in this
crate"*, and paths are a property of a checkout rather than of a project.

**4. Declare it, and refuse when absent (chosen).** One key, one refusal, no derivation.

## Decision

**1.** `.redkiln/config.toml` gains `[project] name`. It is the `<project>` component of every trailer
value this system mints, on both axes.

**2.** A run that reaches a point where it would mint a trailer and finds no project **refuses by
name**, naming the key and the file, rather than emitting a value that degrades to `bare_story_ids`.
The refusal is at the mint, not at startup: a run that mints no trailer is not affected by the key's
absence, and failing such a run would refuse work that has nothing to do with this.

**3.** The name is **not** derived, defaulted, or inferred from the pack, the item id, or any path.

**4.** An empty or whitespace-only `name` is the absent case, not a value — `split_qualified` cannot
distinguish `Story: /slug` from a missing project, so neither may the producer.

## Consequences

Both axes get a well-formed value, and the one way to get a silently-inert trailer is closed. The cost is
a key that existing repositories do not have and that `init` will not add to them, so the refusal has to
be legible — it names `[project] name` and `.redkiln/config.toml`.

`doctor --rebuild` continues to work unchanged: it reads the project out of the trailer it is given, and
this record only fixes what is put there.

**What this forbids**

- Deriving the project from the pack id, the lane, the item id or any part of it, the worktree path, the
  repository directory name, or the git remote.
- Emitting a trailer whose project is empty or whitespace-only.
- Defaulting the project to any literal when the key is absent.
- Refusing a run at startup for a missing project when that run mints no trailer.

## Evidence and confidence

**The absence is measured, not assumed.** `.redkiln/config.toml` is thirteen lines and declares
`[adapters]`, `[packs]` and `[doctor]`; `grep -rn 'project' .redkiln/config.toml` returns nothing.

**The silent-degradation path is read off the code, not imagined.** `split_qualified`'s
`(!p.is_empty() && !s.is_empty())` is quoted above; `classify`'s `Story` arm returns
`ParsedTrailer::BareStory { id }` on its `None`; `reconcile.rs:126-128` inserts that into
`bare_story_ids` and returns no transition. Three consecutive lines, each in a different file, and the
commit still lands at the end of them.

**The doctrine cited is this tree's, with sites.** `Pack::rigor`'s *"an absence, which is not a
`forbidden` and not an `optional`"*; `run.rs:550`'s *"the stage is read once, from the record, and is not
defaulted"*; `rk-git/src/worktree.rs`'s *"There is no call to `std::env::current_dir` in this crate"*.

**Confidence, and what is NOT asserted.** This record does not assert that a single project per
repository is the right long-run model — a repository holding two products would need a project per item,
and the place for that is `ItemRecord`, not `[project]`. It asserts only that the component must be
declared somewhere and that the cheapest correct place today is the config file the product registry
already owns. It also asserts nothing about the `Slice-Verdict:` axis's `<slice>` component, which is
§7h.3's.
