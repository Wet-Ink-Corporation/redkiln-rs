---
id: "0189"
title: "**`rk-store` owns the render, write and read path: a human-facing kind is rendered as layout C from a canonical JSON sidecar that is its value, a structured edit is surfaced as one `structured_drift`, restored from the type and gated by a `render.structured-drift` gate that blocks the stage's exit until answered (C2 resolved), and a vouched root's own change is advisory drift that never blocks** (`s8f`'s render record; D21-42 5a, 5b and 6a's `s8f` half; D21-43 N11 and N12; D21-46 R7; D21-47; D21-48 X11): stamps are HTML-comment lines inside the section's slice; the managed trees are `.redkiln/artifacts/<item>/` and `.redkiln/items/` in the run-owned worktree; the writer and the reader are struct seams, each appending exactly one `artifact.write` or `artifact.read` carrying `kind`, `item`, `stage`, `run`, `schema_version` and the digest, the reader the tree it served, and `answer`'s attach row an `attached_path`; the reader is the only emitter of `artifact.read` and refuses before the entry's reconcile; the `backlog` feed reads the item records the store names, by exact path, into a `BacklogSnapshot`; `doctor` gains `RK-D-ARTIFACT-MUTATION` and `RK-D-DRIFT-KIND`. Supersedes nothing: ADR-0005/0006 as refined by 0075, ADR-0025, ADR-0116 and AC-127 stand"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (5a — *"**Surface, then restore.** `reconcile` emits a `structured_drift` *event* (artifact, anchor, both digests), and the render then restores the section. No drift field on the artifact. … One record owns rk-store's render/write path (layout C, digest-stamped sections, the AC-122 single write path, one `artifact.write` per write)."*; 5b; 6a — *"DebtRegisterEntry is an rk-store item write with provenance; human-sourced facets are null and go to `unknown_facet`."*), D21-43 (N11 — *"**`structured_drift` opens a drift gate that blocks the stage's exit until it is answered.** AC-125's "resolving" means answering that gate."*; N12), D21-46 (R7), D21-47 (*"**`artifact.read` has exactly one emitter: rk-store's read seam (s8f).**"*) and D21-48 (X11 — *"Accepted as a stated residual. RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar does not widen."*); runbook/phase-1/s8f-landing-and-rendering.md §8f.2 (items 10–14, measured 2026-09-28 at `37bba5c`), §8f.3's second row and §8f.5–§8f.7 and §8f.9; references/adr/0005, 0006, 0025, 0032, 0075, 0109, 0116, 0129, 0170, 0173, 0182, 0184 and 0185; crates/rk-store/src/locator.rs, crates/rk-kernel/src/runtime/reference.rs (`ArtifactReadSeam`, `UnwiredReadSeam`, `read_artifact_input`), crates/rk-kernel/src/runtime/step.rs (`call_feed`), crates/rk-kernel/src/runtime/advisory.rs, crates/rk-cli/src/verbs/answer.rs (`store_attachments`), crates/rk-cli/src/doctor/rules.rs (`fn unread`) and .redkiln/registry.toml, as read 2026-09-28 at `37bba5c`
supersedes: []
superseded_by: []
related: ["0005", "0006", "0025", "0032", "0063", "0075", "0105", "0109", "0116", "0129", "0170", "0173", "0180", "0182", "0184", "0185", "0188"]
evidence: []
consumers: []
---

## Context and problem

**`rk-store` has no render, write or read path.** Its modules (`artifacts`, `canon`, `conn`, `identity`, `locator`,
`memo`, `retention`, `schema_lint`, `tier`) write no artifact file and journal no read. `runtime/step.rs` reads
`StepKind::Emit => StepVerdict::Emitted,` and writes nothing. `artifact.write` has one producer, `answer.rs`'s
`store_attachments`, which journals `kind`, `item`, `schema_version` and `digest` and stores no file. `artifact.read`
has **no** producer: measured 2026-09-28 at `37bba5c`, outside `rk-types` the only code lines naming it are the three
kernel fold arms and `doctor`'s `RK-D-UNREAD` (`rules.rs`, `of_kind(ctx, "artifact.read")`).

**The AC register's last open contradiction is C2**: *"Is a hand-edit inside a rendered section drift, or is it
silently overwritten?"* D21-42 5a answers it (*"Surface, then restore"*), and D21-43 N11 adds the gate (*"`structured_drift`
opens a drift gate that blocks the stage's exit until it is answered"*). AC-478's *"AND no comparison of
stored-vs-regenerated rendering is performed, so no drift state exists to report"* is the one clause that falls.

**The read seam is named and stubbed.** `runtime::reference::read_artifact_input` makes one call to an
`ArtifactReadSeam` per resolved `artifact:` input, and the composition root's value at Checkpoint 8d is
`UnwiredReadSeam`, *"which appends nothing"* (`reference.rs`: `impl ArtifactReadSeam for UnwiredReadSeam { fn
read(&self, _read: &ArtifactRead<'_>) -> Result<(), String> { Ok(()) } }`). `Runtime::with_read_seam` is the rebind.
`ArtifactRead` carries `item`, `stage`, `kind`, `reader_activity`, `reader_step`, `binding`, `producer` and `value`,
and no run and no tree.

**`RK-D-UNREAD` passes over what it cannot key.** It keys a row on `str_field("stage")` and `str_field("kind")` and
counts runs from `payload.get("run")`; a row that lacks either is skipped by a `continue`. `store_attachments`' rows
carry neither. `t1_gate_transition`'s `opened` row carries the opening run's `stage` and `run_id` as columns (the
`0002` migration: *"The opening run's `stage` and `run_id` sit on the `opened` row only"*), and a rebuilt `opened` row
has a null `run_id` (D21-48 X11).

**AC-127's vouched-root half and the `backlog` feed are this stage's (D21-46 R7).** `.redkiln/registry.toml` declares
`[feed.rust.backlog]` and `[feed.node.backlog]` `unprovided`, and `item_create` is on the `store` route with no
handler. `closure-record`'s `audit` step reads `backlog` as a mandatory input, and `s8d`'s `lane_walk` tests record it
refusing by name (*"`feed.rust.backlog` is mandatory"*), not walking.

**One measured obstacle this record meets** (§8f.2 item 11, 2026-09-28 at `37bba5c`). The resolver reaches a feed's
verb through `Runtime::call_feed` in `runtime/step.rs`, which refuses every route but `toolchain`: *"the feed verb
`{verb}` takes the `{}` route with {} declared input(s); a feed is produced by a `toolchain` row with no inputs"*. So
at Checkpoint 8e there is **no path from a feed's `verb` to a `store`-route handler**. No accepted record fixes that
rule: ADR-0170 decision 8 reads *"`[feed.<toolchain>.<name>]` declares exactly one of `verb = "<a registry verb>"` or
`unprovided = "<reason>"`"*, any registry verb. Decision 12 below states how this record reaches it, and it is
**TAKEN PROVISIONALLY**, because §8f.2 item 11 makes this a STOP whose resolution is the operator's.

## Decision drivers

- AC-120: nothing parses structure out of markdown; `store::locator` stays the one module that looks at it.
- AC-122: one write path into a managed directory, journalled, with zero writes on replay.
- ADR-0109 decision 2 and AC-477: a typed value survives the deleted-database test and is read back *"from the store,
  not from the markdown"*.
- N11: a structured drift blocks the stage's exit until answered; a vouched root's change never blocks (ADR-0025).
- D21-47: the read seam is the only emitter of `artifact.read`.
- `RK-D-UNREAD` can only count what carries `stage` and `run`.
- ADR-0116 decision 6: *"No second digest algorithm enters the system."*

## Considered options

### Option — the canonical JSON sidecar in the item's managed directory is the value (taken)

The value is `rk_store::canon::canonical` bytes beside the rendered `.md`, written only by `rk-store` and landed on
the run ref by the landing record (ADR-0188). A deleted database loses nothing, because git holds the sidecar.

### Option — the value in a store table

Rejected: after a deleted database no git fact could rebuild it, so AC-477 would fail the deleted-database test
(ADR-0109 decision 2).

### Option — parse the value back out of the rendered markdown

Rejected by ADR-0005 and AC-120: it is the parser the whole layout exists to avoid.

### Option — silently overwrite a hand-edited structured section (C2's other horn)

Rejected by D21-42 5a and AC-478's own amendment: the edit is lost with no trace, which is the *"silently lossy"*
note the register carries.

### Option — surface and restore with no gate

Rejected by N11: AC-127's *"does block until resolved"* would be false, and AC-125's resolution would have no channel.

## Decision

**1. Layout C, rendered, never parsed.** A human-facing kind renders as layout C (ADR-0006, as refined by ADR-0075):
typed frontmatter, read-only structured sections and opaque prose sections. Each rendered section is a pure function
of *(typed value, template version)* (AC-121, AC-123). A machine-only kind is its canonical JSON sidecar alone and is
rendered to `.md` on demand, never stored (AC-144). Nothing parses structure out of markdown; `store::locator` is the
only module that looks at it (AC-120), and a section is located by its anchor's byte range.

**2. The stamp.** Each structured section's first line inside its slice is one HTML comment,
`<!-- rk-stamp template=<kind>@<N> sha256=<64 hex> -->`, where `<N>` is the kind's template version (a named
constant per kind in `rk-store`, starting at `1`) and the digest is `rk_types::digest::sha256` over the section's
bytes after the stamp line to the slice's end. The frontmatter's stamp is its last line, the YAML comment
`# rk-stamp template=<kind>@<N> sha256=<64 hex>`, over the frontmatter's other bytes. An HTML comment is one of
`locator.rs`'s decoys (*"A heading-looking line inside … an HTML comment … is content"*), so the stamp is never an
anchor. The template version in the stamp is what makes AC-128's template-drift kind buildable later.

**3. The canonical typed value** is a canonical JSON sidecar, `<Kind>.json`, beside its rendering `<Kind>.md` in the
item's managed directory (decision 5). Only `rk-store` writes it, and the landing commits it (ADR-0188), so it
survives the deleted-database test and is read back from the store, never from the markdown (AC-477). One file per
`(item, kind)`: a later write of the same kind replaces it through the writer, with its own `artifact.write`.

**4. C2 — surface, restore, and gate.** At every entry, after `s8d-ii`'s re-entry discard and
`reconcile_at_entry`, `rk-store` passes over the item's managed directory in the run-owned worktree, for each
structured section and for the frontmatter:
- it locates the section and compares its bytes with the rendering of the typed value;
- on a difference it appends **one `structured_drift`** with `{ kind, item, anchor, old_digest, new_digest,
  causing_commit }`: `anchor` is the section's anchor text or `frontmatter`, `old_digest` the rendering's,
  `new_digest` the bytes on disk, and `causing_commit` the last commit on `rk-run/<item>` that touched the file (a
  `log -1 --format=%H -- <path>` read through the seam, `GitOp::Log`);
- **then** it re-renders the section from the type (AC-411), a write with its one `artifact.write`;
- **then** the kernel opens a drift gate of class **`render.structured-drift`**, git-first (N6; ADR-0184), whose
  `Gate:` commit is the checkpoint that lands the restore (ADR-0188 decision 2). The gate names the kind, the anchor
  and both digests.

The stage's steps keep dispatching, because the typed value is whole. What is blocked is the boundary: **the stage
advance refuses** (this stage adds the check to `s8e`'s advance, before its boundary act) while the fold over
`t1_gate_transition` holds a `render.structured-drift` gate for the item at this stage whose latest state is
`opened`. The refusal names the class, the token and the drifted artifact; no boundary commit is made and no row is
appended. `run` exits `0` at the gate. The class needs a `gate_expiry` row in `s8h`'s table.

A **prose** section edited on disk is accepted whole as the field's new value, with no drift record and no gate
(AC-152).

**5. The drift gate's two answers**, each a typed write through a gate answer (AC-125's *"a typed write through the
CLI, web or a gate answer"*; N11 reads AC-125's "resolving" as answering this gate):
- `confirm-restore`: the restore stands, and the gate closes;
- `move-to-prose <section>`: at the next entry `rk-store` reads the drifted bytes from `causing_commit` (never from
  the event, which carries digests only) and appends them to the named prose section through the writer, as one
  `artifact.write`. No drift text reaches a deserializer: it lands in opaque prose.

**AC-127 holds unamended**, since *"the same change on a store-rendered structured section does block until resolved"*
is now true. No artifact type carries a drift field or a signature-comparison field (AC-478's *Verified by*); the
drift gate is a gate row. AC-478's AND line is removed by a dated amendment (§8f.10).

**6. The managed trees.** An item's rendered artifacts live under **`.redkiln/artifacts/<item>/`** inside the
**run-owned worktree** (ADR-0137 clause 3: *"`project_root` is the run-owned worktree"*), from the checkout
`.redkiln/worktrees/<item>/.redkiln/artifacts/<item>/`. The inner `<item>` is kept so two items' artifacts do not
collide once the operator merges their run refs; its length is paid for by ADR-0188 decision 5's path bound. No
`.gitignore` row covers it, and the store reaches the run tree's copy by the path in `worktree_assigned`, never by a
walk. **`.redkiln/items/`** in the run-owned worktree is the second `rk-store`-managed tree, and this record reads
AC-122's *"a path under a managed artifact directory"* to include it. The checkout's own tree is never written
(AC-518).

**7. The single write path (AC-122).** The writer is a **struct** seam in `rk-store`, `ArtifactWriter` (a struct,
never a trait, so Checkpoint 7a's port surface does not move). It is journalled, and on replay it performs zero writes
and serves the recorded digest. `rk-store` exports no filesystem-write handle into a managed tree, and a `trybuild`
compile-fail case proves a sibling crate cannot write there.

**8. One `artifact.write` per write**, with these members:
- `kind`, `item` (and ADR-0182's interim key beside the event);
- `stage`: the stage whose step wrote the row; `run`: that stage run's `run_id` (`format!("{}-{}", stage,
  &digest[..16])`, ADR-0152);
- `schema_version`: the kind's registered `schema` string, or `null` (design §5.4's R-49 note);
- `digest`: `rk_types::digest::sha256` of the bytes written, the function `answer.rs` already calls (ADR-0116
  decision 6);
- `path`: the repo-relative path written inside the run-owned worktree (so `s8j`'s (1b) can pair the digest with
  `git cat-file -e rk-run/<item>:<path>`).

**`answer`'s attach row** keeps `kind`, `item`, `schema_version` and `digest`, and gains `stage` and `run` and the
carrier `attached_path`. `stage` is the answered gate's stage and `run` the `run_id` of the run that opened it, both
read from the token's `opened` row in `t1_gate_transition`. **The carrier** is `attached_path`: the attachment's path
relative to the invoking checkout's root in `/` form when it lies under it, and otherwise its absolute path in
drive-letter `/` form. It is not spelled `path`, because `path` on a writer row names a managed destination and one
member name keeps one meaning (§8f.2 item 12 named `path`; this is the stated reason for the change). **X11's
residual, stated:** an `opened` row re-derived by `doctor --rebuild` has a null `run_id`, so an attach answered after
a deleted database writes `run: null`. `RK-D-UNREAD` reads a null `run` as an unknown run, never as a match (D21-48
X11): its `run_of` reads `run` as a string or an integer, keyed by the JSON value's canonical text so `0` and `"0"`
stay distinct, and returns no run for `null`, an absent `run` or any other type, so the row is passed over. The `Gate:`
grammar does not widen.

**9. The read seam (N12; D21-47).** The reader is the writer's sibling, a struct seam `ArtifactReader` in `rk-store`,
and the composition root re-points `Runtime::with_read_seam` from `UnwiredReadSeam` to it (the stub is kept for
`s8d`'s named test and is no longer any root's value). It is bound per stage run, so it knows its `stage` and `run`.
It serves the kind's value from its sidecar on the run ref with the value's digest, checks that digest equals the
committed `emit` binding's (a mismatch is a halt, Halt 4), and appends **one `artifact.read`** carrying `kind`,
`item`, `stage`, `run`, `schema_version`, `tree` (the run ref's tree the bytes were read from) and `digest`. It
**refuses** a read when the current entry has no reconcile record (AC-126), and no flag or cache skips that. It is the
**only** emitter of `artifact.read`; the resolver appends nothing itself. On replay it serves the recorded digest and
appends nothing. A vouched root has no `emit` binding, so the reader serves its bytes and digest without that check.

**10. The vouched root (AC-127's first half; D21-46 R7).** This record's **reading of ADR-0025**: its decision names an
*upstream* change (*"When an upstream artifact in the same stage subsequently changes, the *next* gate the item
reaches carries an advisory note"*), and AC-127's GIVEN names the root's *own* change and cites ADR-0025; so ADR-0025's
rule governs both, and the root's own change **never blocks, never re-runs and is never silent**. ADR-0025 stands
unamended. The upstream half is AC-525's (`s7b`, `runtime/advisory.rs`); this record builds the own-change half on it.
- **The store.** `answer` does not land (ADR-0188 decision 14), so the root is stored at the next `run`'s entry:
  the writer reads the bytes at `attached_path`, checks their digest equals the attach row's, and writes them as
  `<Kind>.root<ext>` in the item's managed directory (`<ext>` the attachment's own extension, or none), with one
  `artifact.write`. A missing file or a mismatch is **refused by name**, naming the kind, the path and both digests,
  and nothing is stored. The **vouched digest** is the attach row's, and only a new attach moves it.
- **The pass**, in the same entry pass as C2: for each stored root it compares the file's digest with the vouched
  digest. A root is not layout C, so the whole file is the instrument here. **Changed:** one `drift_advisory` in
  AC-525's four-field shape `{ root, upstream, old_digest, new_digest }` with `upstream: null` (the own-change mark),
  `old_digest` the vouched digest and `new_digest` the file's, through `AdvisorySink`, so once per `(root, null,
  new_digest)` per run; the next `gate_opened` carries it (`carried_by_gate`). Nothing is restored, rewritten or
  gated, no step re-runs, and the root stays `Present` (AC-067). **Unchanged:** nothing is appended.
- **The causing commit is not carried** on the advisory: `Advisory::FIELDS` stays four (*"all four fields and no
  fifth"*), and the commit is recoverable by `git log` over the root's path. Design §5.4's *"the causing commit"* is
  carried by `structured_drift` (decision 4), not by `drift_advisory`.
- `Advisory.upstream` becomes `Option<String>` and serialises `None` as JSON `null`; `Some(s)` serialises byte for
  byte as today; a second constructor, `Advisory::own_change`, is added, and `from_signal`'s *"The only constructor"*
  doc is corrected to name both.

**11. The item write (D21-42 6a, `s8f`'s half).** `item_create`'s `store`-route handler is `rk-store`'s writer,
never a `Toolchain` adapter. It writes `.redkiln/items/<id>.toml` in `ItemRecord`'s key set (`id`, `kind`, `stage =
"triage"`, `[facets]`) plus a `[provenance]` table naming the producing activity at its version, the run and the
source `ImplReport` digest. A facet whose registry `source` is `human` or `llm` is **never written**, even when the
extracted value carries one, and is named in `[provenance]`'s withheld list (golden constraint 1). A byte-identical
existing file is **served** (nothing written, no row); a differing one is **refused by name, never overwritten**. No
artifact kind is minted: `ItemRefs` is a value. An item record is not an artifact kind, so its write appends no
`artifact.write`.

**12. The `backlog` feed (D21-46 R7).** A `store`-route verb row `backlog = { outputs = "BacklogSnapshot", live =
true, timeout = "build_timeout", inputs = [], route = "store", side_effecting = false }`, and `[feed.rust.backlog]` and
`[feed.node.backlog]` become `verb = "backlog"`, in one commit (the loader refuses a feed `verb` that is not a row).
- **TAKEN PROVISIONALLY — how the resolver reaches it.** `Runtime::call_feed` gains a `store` arm: a `store`-route
  feed verb with no inputs is served by the kernel's store-route handler (a composition-root-bound struct seam beside
  the read seam, because `rk-kernel` cannot depend on `rk-store`), and every other route keeps its refusal. This
  widens `s8d`'s code, not a record (ADR-0170 decision 8 admits any registry verb; ADR-0173 does not fix the route),
  and it is recorded in `s8d`'s file by dated note. §8f.2 item 11 makes this a STOP; the operator's ruling may replace
  this paragraph before the record flips.
- **What it reads.** The item ids the store names: a tier-1 `stage_transition` row's `item`, or a journalled `ItemRefs`
  output of an `item_create` step. Each is read at `.redkiln/items/<id>.toml` in the run-owned worktree **by its exact
  path** (`doctor --rebuild`'s precedent); `.redkiln/` and `.redkiln/items/` are **never walked**.
- **`BacklogSnapshot`** is an `rk-types` value serialised through `rk_store::canon::canonical`: `items`, ordered by
  `id`, each `{ id, kind, stage, facets, provenance, record_digest, named_by }` (`facets` the record's table verbatim,
  an absent facet absent; `provenance` the table or `null`; `record_digest` the `rk-types` digest of the record's
  bytes; `named_by` a sorted subset of `stage_transition` and `item_write`), and `absent`, ordered by `id`, each `{ id,
  named_by }` for a named id with no record in this worktree. A named id is **listed absent, never dropped**.
- **The cost, stated.** A journalled `ItemRefs` output is tier 2: after a deleted database an item that was minted and
  never run drops out of the feed until it is run. Tier-1 names survive, refolded from `Story:` trailers.
- It appends no `artifact.read` (an item record is not an artifact kind), and on replay the resolved input is served
  from the journal.

**13. `doctor`'s two rules, spelled.**
- **`RK-D-ARTIFACT-MUTATION`** (AC-159): for every managed directory the store names (each `worktree_assigned` row's
  run-owned worktree, plus the checkout's own `.redkiln/artifacts/` and `.redkiln/items/`), it recomputes each
  structured section's and the frontmatter's digest through `rk-store`'s locator and compares them with the rendering
  of the sidecar; a difference that no `artifact.write` and no reconcile-accepted prose edit accounts for is reported,
  naming the artifact, both digests and the last recorded write. A prose-only edit, a whole-file digest and a vouched
  root are out of scope. It never walks `.redkiln/` recursively.
- **`RK-D-DRIFT-KIND`** (AC-127's *Verified by*): over the store's journal in append order, it reads the blocking
  property from `rk_types::drift::Drift::blocks()` and reports (a) a `drift_advisory` followed in the same entry by a
  `render.structured-drift` gate naming its root, and (b) a `structured_drift` with no such gate naming its artifact
  opened after it in the same entry. Each finding names the drift kind, the artifact or root, and the digests.

## Consequences

**Positive**
- C2 is resolved: a structured edit is recorded, restored and answered, never silently lost.
- `artifact.write` and `artifact.read` have producers, and `RK-D-UNREAD` counts real rows.
- AC-127 is met whole: one drift kind blocks, the other is recorded and does not.
- The `backlog` feed has an honest producer that reads only what the store names.

**Negative**
- Every entry pays a pass over the item's managed directory.
- A drift gate stops a stage's exit until a human answers it, even when the restore was right.
- An attach answered after a deleted database writes `run: null`, which `RK-D-UNREAD` cannot count (X11).
- An `attached_path` outside the checkout is host state: another box cannot store that root.
- An item minted and never run drops out of `backlog` after a deleted database.
- The resolver's `store` arm widens `s8d`'s feed call, provisionally, pending the operator's ruling.

**What this forbids**
- Parsing structure out of markdown anywhere but through `store::locator`, and reading a typed value from a `.md`.
- A write into `.redkiln/artifacts/` or `.redkiln/items/` that is not `rk-store`'s writer, and a writer write with no
  `artifact.write` (items excepted, decision 11).
- An `artifact.read` appended by anything but `rk-store`'s reader, and a read served before the entry's reconcile.
- An `artifact.write` or `artifact.read` row without `stage` and `run`.
- Restoring a structured section without its `structured_drift`, recording the drift without restoring it, or doing
  either without opening the `render.structured-drift` gate.
- Leaving a stage while its `render.structured-drift` gate is open.
- Classing a vouched root's change as blocking, restoring or rewriting it, re-running work over it, or appending
  nothing for it.
- Deserialising drift text into a typed field.
- `RK-D-UNREAD` keying a `null` `run` by its text.
- The `backlog` producer walking `.redkiln/`, and dropping a named id with no record.
- A drift field or signature-comparison field on any artifact type.

## Evidence and confidence

**Measured** (2026-09-28, at `37bba5c`, `s8f` §8f.2): item 10 (the one call, the stub, and zero code-line
`artifact.read` appends outside `rk-types`, the fold arms and `doctor`), item 11 (the feed rows, `item_create`'s
`store` row, `call_feed`'s `toolchain`-only arm, `lane_walk`'s refusal for `closure-record`, no `lane_e2e` test
reaching `closeout`, and `source_registry::the_catalog_source_census_is_fifty_with_fourteen_declared` and
`reference_resolver::kb_checklist_and_feed_keys_resolve_through_the_declared_tables_and_an_undeclared_key_refuses_by_name`
pinning `backlog` as `unprovided` over the shipped registry), item 12 (the golden log under `CORPUS_PIN` holds no
`artifact.write` row; the `-text`-pinned doctor fixtures `RK-D-CREDENTIAL-RETAINED`, `RK-D-SINK-COUNT` and
`RK-D-UNREAD` hold hand-authored `artifact.write` rows that are rule inputs, not a producer's recorded output, so the
carrier moves none of them), item 13 (39 rule ids joined to fixture directories, unchanged since `c5d19d7`) and item
14 (`t1_gate_transition` has `stage` and `run_id` columns). **Not measured:** the entry pass's cost over a large
managed directory. This record is `proposed` and flips only on the operator's dated ledger line.
