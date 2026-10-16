---
id: "0199"
title: "**The `Binary` arm of `ChangeSummary` is finalised from S-BIN (F43): it carries the four fields S-BIN's reviewers cited, opens through one checked constructor, has no Phase-1 producer, and an item whose range carries a `Binary` path takes a human gate opened through a type-neutral kernel request** (`s8i`'s record; D21-42 decision 2; D21-43 N21; D21-48 X3 and X16; METHOD-S §3.3 and §3.4): `assets`, `captures`, `tests.report_ref` and `build.log_ref`; pass 1 `branch1`, pass 2 `undecided-fp`, so the route is the human gate and is reported as not decided by the registered rule; the three adapter refusal pins stand; a `binary`-class AC keeps AC-337's `not-feasible(reason)` route; the kernel opens the gate with the closed reason `automated-verdict-refused` and an opaque attachment through `gate::reach`. Supersedes ADR-0165 decision 1 in part (the sentence that the payload cannot be constructed outside `rk-ports`)"
status: accepted
date: 2026-10-14
legacy_id: none
source: spikes/F43-DRAFT.md (proposed, written 2026-10-14 from corpus `835b122`'s blobs); D:/repos/redkiln-corpus `results/s-bin.jsonl`, `results/s-bin/missing-fields.jsonl` and `results/s-bin/oot-shadow-rulings.jsonl` at `835b122`, read through the object store; spikes/METHOD-S.md §3.3, §3.4, §3.5 and §7 (the 2026-09-09 entry and the s8i pre-dispatch amendment with its extensions); spikes/DECISIONS-2026-09-21.md D21-42 decision 2, D21-43 N21, D21-45 W4, D21-46 R8, D21-47 V1, D21-48 X2, X3 and X16, D21-152 items 2 and 3, D21-153; runbook/phase-1/s8i-s-bin.md §8i.0 item 12, §8i.2 items 6 and 9, §8i.8; runbook/phase-1/s8a-records-and-red-checkpoints.md §8a.2 item 2a; references/2026-09-08-acceptance-criteria.md AC-154, AC-337, AC-356, AC-357, AC-475, AC-497; references/2026-09-08-architecture-design.md S-BIN's falsifier row; references/adr/0165 decisions 1, 2, 6 and 7 and its forbids; references/adr/0008; references/adr/0092 rule 1; references/adr/0135's fourth forbids bullet; references/adr/0180
supersedes: ["0165"]
superseded_by: []
related: ["0003", "0008", "0019", "0029", "0073", "0085", "0092", "0093", "0099", "0104", "0135", "0151", "0180", "0189"]
evidence: ["F43", "S-BIN", "corpus@835b122"]
consumers: []
---

## Context and problem

**S-BIN has reported, and the `Binary` slot is owed its record.** ADR-0165 split `ChangeSummary` into a finalised
`Text` arm and a `Binary` slot, and its decision 7 says: *"The sealed slot is temporary by design. It opens by a
later record once S-BIN reports at `s8i`."* D21-42 decision 2 set the order: *"The `Binary` variant stays sealed and
non-constructible until **S-BIN runs as a Phase-1 sub-stage before Checkpoint 8**"*. S-BIN ran on 2026-10-14 against
the real Unreal project ADR-0008 names (BrittonGames, pin `041ead2c`): ten scored Opus dispatches, two passes of five
items, ten records valid and none void, in the corpus at `835b122` (F43, proposed).

**What S-BIN measured** (F43, per pass, never pooled):

| pass | strong detection (seeds 4, 5) | false positives (clean items 1–3) | METHOD-S §3.4 outcome |
| --- | --- | --- | --- |
| 1 (B2) | 2/2 | 1/3 (item 3) | `branch1` |
| 2 (B3) | 2/2 | 2/3 (items 1, 3) | `undecided-fp`, *"not decided by the registered rule"* |

`results/audit-s.py bin` reads `rule_outcome_pass1=branch1` and `rule_outcome_pass2=undecided-fp` over the pinned
blobs, and `missing-fields.jsonl` is empty: no pass missed a seed, so no branch 2 fired and B4 did not run. All three
false positives rest on one `narrowed-pin` finding each, and none was adjudicated (F43 Limit 4). The reviewer ran the
automation suite on 0/10 dispatches, and the scored rendering gave it no runtime (D21-153 item 3).

**What the reviewers cited** (the records' `cited_fields`, mechanical; the driver's vocabulary resolves `range`,
`text_diff`, `assets`, `build`, `build.log_ref`, `tests`, `tests.report_ref` and `captures`):

| field (METHOD-S §3.3 draft) | records citing it, of 10 |
| --- | --- |
| `assets` | 10 |
| `captures` | 6 (every record whose item carries a capture: items 1, 4 and 5, both passes) |
| `tests.report_ref` | 6 |
| `build.log_ref` | 3 |
| `text_diff` | 1 (pass 1 item 5, citing that it is empty) |
| `range` | 0 |
| `build` (`command`, `exit`) | 0 |
| `tests` (`command`, `passed`, `failed`, `skipped`) | 0 |

Every one of the 13 blocking findings cites a read of an asset's bytes or a capture's pixels, and none cites only
spec or report strings (F43's D21-152 item 3 tag, a reading recorded only, never scored).

**Three questions are this record's** (`s8i` §8i.8): which fields the findings support, which review route the
per-pass outcomes give, and whether any Phase-1 adapter constructs the arm. Beside them: what evidence a
`binary`-class criterion gets, where a Phase-2 producer's run worktree comes from, how the human-gate route reaches
the kernel without the kernel naming the type, and whether this record supersedes ADR-0165 in part.

## Decision drivers

- **Only fields the findings support, and no field they do not** (`s8i` §8i.8). METHOD-S §3.3's draft is the
  registration the census is read against: *"ChangeSummary stays BLOCKED and its field set is not fixed here."*
- **Per pass, never pooled; the route follows the registered map.** D21-43 N21: *"**Registered as undecided before
  dispatch.** If either occurs, the `Binary` variant takes the human-gate route."* §8i.8's map also takes the more
  conservative route when the passes disagree.
- **ADR-0008's forbids:** *"`ChangeSummary` may not be designed and locked purely against a synthetic or paper
  fixture; it must be designed against the real UE project's actual artefacts."* S-BIN's fixture is the real project.
- **ADR-0135's fourth forbids bullet and Checkpoint 8a's ban stand** (D21-48 X16): `crates/rk-kernel/src` names no
  `ChangeSummary`.
- **A decision sentence or a forbids bullet moves only by supersession** (ADR-0092 rule 1: *"A correction that
  would move either is a supersession and takes a new record."*).
- **The sample is small and selected.** F43's Limits: n = 5, perceptual seeds, seed 5 replaced (D21-142), the
  report's diagnosis present in the seeded items' reports (D21-152 item 2), a capture confound (D21-149 item 2) and a
  no-runtime rendering (D21-153 item 3). Nothing here is a rate.

## Considered options

### Option — the four cited fields, the human-gate route, no Phase-1 producer (taken)

Everything under `## Decision`. It follows the registered map row by row and keeps only what the reviewers cited.

### Option — METHOD-S §3.3's draft field set whole, `range`, `text_diff`, `build` and the `tests` counts included

The registration as drafted. **Not taken.** The census gives `range` 0/10, `build.command`/`build.exit` 0/10 and the
`tests` counts 0/10; `text_diff`'s one citation states that it is empty, and under the registered scoping every
item's `text_diff` was empty (`s8i` §8i.0 item 4's correction). Keeping them would lock members the findings do not
support, which §8i.8 forbids (*"no field the findings do not"*). A later record adds one when a measurement cites it.

### Option — `assets` alone

The only field every record cited. **Not taken.** Every seed detection in both passes also read the capture
(F43's tag table: pass 1 item 4 f1, item 5 f0 and f2; pass 2 item 4 f0, item 5 f0), and `tests.report_ref` and
`build.log_ref` were cited by 6 and 3 records. Dropping them would remove fields the findings did use.

### Option — route: reviewer, reading pass 1's `branch1` as the result

**Not taken.** The route is `reviewer` only when both passes are `branch1` or `branch2-detected` (§8i.8, *"The route,
mechanically"*). Pass 2 is `undecided-fp`, which N21 routes to the human gate, and the passes disagree, for which the
map takes the more conservative route. Under D21-152 item 2, pass 1's `branch1` reads as *"sufficient given the
report"*, not *"ChangeSummary alone sufficient"*.

### Option — read the result as the design's falsifier ("the reviewer cannot verify without a diff")

The design's S-BIN row: *"`ChangeSummary` is not a type problem but an evidence problem. The review gate's
applicability to binary-asset toolchains is withdrawn, ADR-0008's Phase-2 Unreal adapter loses its premise, and the
five toolchain-keyed kinds (ADR-0073) are re-cut."* **Not taken, because the falsifier's premise did not occur.** The
reviewer detected both seeds in both passes (4/4) from the assets' own bytes and the captures, with no diff. What
failed is clean approval under the strict policy, which is the undecided outcome N21 registered, not the falsifier.
None of its three consequences is taken here.

### Option — a Phase-1 producer: let the Rust adapter answer the `Binary` arm

**Not taken.** No Phase-1 toolchain reads an Unreal project's assets, report or captures. The arm's members are
produced by an Unreal toolchain run, which ADR-0008 ships in Phase 2 (*"The full Unreal toolchain adapter may not ship
before Phase 2"*). A Rust or Node adapter that constructed the arm would have no source for `captures`, `report_ref`
or `log_ref`.

### Option — carry the refusal through the kernel by naming the type in `workflow::gate`

**Not taken.** D21-48 X16 decided *"A type-neutral gate path"*, and ADR-0135's fourth forbids bullet stands.

## Decision

**1. The field set.** The `Binary` payload (`BinaryChangeSummary`, in `rk-ports`, ADR-0165 decision 1) carries the
fields S-BIN's reviewers cited, and no other:

Binary field set (4 fields): `assets` (one row per changed path the range classifies `Binary`, LFS pointer paths included: `path`, `kind`, `action`, `size` read from the LFS pointer, `lfs_oid`); `captures` (one row per capture: `path`, `viewport`, `note`); `tests.report_ref`; `build.log_ref`

- **Why these four.** `assets` was cited by 10/10 records and every blocking finding read an asset's bytes, three of
  them checking the asset's sha256 against `lfs_oid`. `captures` was cited by every record whose item had a capture
  (6/6), and both seeds' detections read the capture in both passes. `tests.report_ref` (6/10) and `build.log_ref`
  (3/10) were cited by name. The asset row keeps its five registered members because the citation vocabulary
  resolves `assets[`, `lfs_oid` and the row as one field; no finding argues for dropping a member, and the pinned
  rendering carried all five.
- **Why not the rest.** `range`, `build.command`/`build.exit` and the `tests` counts were cited by no record.
  `text_diff` was cited once, to say it was empty; every S-BIN item's `text_diff` was empty under the registered
  scoping, so S-BIN measured no binary range that also carried a text change (Consequences, Negative).
- **`size` is read from the pointer, never inferred** from whether the pointer's size line moved (the s8i
  pre-dispatch amendment item 6, as `s8i` §8i.2 item 6 restates it). What an LFS path's summary carries is already
  assigned: `s8a` §8a.2 item 2a makes an LFS pointer path `Binary` in the `Text` variant's manifest and says *"What
  an LFS asset's summary carries (for example its size read from the pointer) is `Binary`'s content, and so
  `s8i`'s."* This record does not re-specify the pointer rule; it fills the content.
- **Serialised form.** The payload serialises its four fields under these names, `tests.report_ref` and
  `build.log_ref` as `tests: { report_ref }` and `build: { log_ref }`. It carries none of the `Text` arm's catalog
  members (`sha`, `diff`, `files`, `manifest`; ADR-0165 decision 3).

**2. Construction.** The payload's fields are private. It is constructed only through **one public checked
constructor in `rk-ports`**, which refuses, by name, an input that carries a text patch or diff for a
`Binary`-classified path (AC-497's AND: *"no textual diff of the binary path is assembled or dispatched"*). Any crate
may call the constructor; none may build the payload any other way. This opens the slot ADR-0165 decision 7 names.
`s8a`'s three seal pins (`change_summary::binary_arm_is_not_constructible_outside_rk_ports` with its `trybuild` case,
Checkpoint 8a guard (6)'s no-`pub`-field line, and guard (6)'s `sealed_open` floor) are superseded by name in the
commit that lands the constructor, as ADR-0165 decision 7 provides; the new `trybuild` case pins the construction
rule in their place.

**3. The route.**

Binary review route: human gate

- **Why.** Pass 1 is `branch1` and pass 2 is `undecided-fp`. N21 sends `undecided-fp` to the human gate, and the
  disagreement takes the more conservative route; both rows of §8i.8's map give the human gate. The outcome is
  reported as **"not decided by the registered rule"**, and the passes' disagreement is reported as a stability
  finding (F43: clean item 1 flipped from approve to a `narrowed-pin` block).
- **What it means.** An item whose range carries a `Binary`-classified path cannot receive an automated review
  verdict. `rk-cli`'s `review_input` (the seam measured at `s8i` §8i.8's stage-open note 2: the `Toolchain` decorator
  layer in `crates/rk-cli/src/verbs/run.rs`) *(dated 2026-10-14, s8i build, S8I-D2-WIRE-GATE: that layer is
  `review_input::route`, which `verbs/run.rs`'s `resume_and_read` wraps around the stage's toolchain; the gate opens
  there once the kernel's walk returns, through `run`'s gate path)* assembles no diff for such a path and returns a refusal of an automated
  verdict, and the review stage opens a human gate carrying the typed summary, in AC-357's shape, until a later record
  moves the route on new evidence or a producer of the rendered representation exists (decision 6).
- **The gate path is D21-48 X16's**, verbatim: *"**A type-neutral gate path.** The kernel gains a gate request with a
  closed reason (`automated-verdict-refused`) and the summary as an opaque attachment it never types. ADR-0135's
  forbids bullet and Checkpoint 8a's ban stand."* Concretely, in `rk-kernel`'s `workflow::gate`:
  - a **closed reason type** with exactly one value, spelled `automated-verdict-refused`; an unknown reason string
    is refused by name and never carried as free text;
  - an **opaque attachment**: bytes and their digest, carried byte-for-byte on the opened gate's event, never
    parsed, decoded or typed by the kernel. `review_input` serialises the typed summary into it; only `rk-cli` and
    the render side that reads the gate know the bytes are a `ChangeSummary`;
  - a **request** that opens the gate through the existing reach path, `gate::reach`, its token derived by
    `GateRecord::derive_reach` as every other gate's is, the reason as the gate's kind and the attachment's digest
    beside it.

**4. Producers.**

Binary producers in Phase 1: none

- No Phase-1 source file constructs the arm outside tests. So the three adapter refusal pins stand unchanged:
  the Rust adapter's `change_summary::change_summary_refuses_a_range_carrying_a_binary_path` and
  `change_summary::change_summary_refuses_a_binary_blob_the_working_tree_does_not_hold`, and `s8a`'s
  `change_summary::change_summary_refuses_a_range_carrying_an_lfs_pointer_path`. ADR-0165 decision 6 stands: the
  Rust adapter refuses a range carrying a `Binary` path, an LFS pointer path or a binary blob, naming the path.
- Tests and fakes may construct the arm through the constructor (decision 2), for the render and review-input pins.

**5. The evidence a `binary`-class AC gets: AC-337's `not-feasible(reason)` route is KEPT.** AC-356's struck clause
left the question at *"AC-337's `not-feasible(reason)` route until S-BIN reports"*, and the design refused to switch
the per-AC mutation norm off for a class blind (*"a substantive gate decision"*). S-BIN measured no mutation path over
binary assets and no reliable automated approval, so nothing here supports changing it. A `binary`-class AC whose
toolchain declares mutation infeasible records `mutation: not-feasible(reason)` (AC-337), never `pinned` by the
mutation path, and its evidence is AC-357's human gate answer, with `by` and `at`, given over the typed summary this
record fixes. No `binary`-class exemption from AC-334 is granted.

**6. The rendered-representation requirement, recorded against ADR-0008's Phase-2 Unreal adapter.** The human gate
holds until a producer of a rendered representation exists. ADR-0008's Phase-2 Unreal adapter is that producer, and
it is required to produce, for each `assets` row whose type admits one, a capture the `captures` field can carry, from
the run worktree, by a recorded procedure applied identically to every item. S-BIN's captures were taken by hand
through `/ue-verify`'s `capture_scene_preview`, which rejected every asset of items 2 and 3 (a World; a Blueprint class
and a data asset; D21-149 item 2), so a rendered representation for those asset types is **owed** by that adapter's
record. Moving the route to `reviewer` needs a later record on new evidence, never this requirement alone.

**7. The Phase-2 producer's run worktree.** A Phase-2 `Binary` producer reads the run-owned worktree ADR-0180
fixes, `.redkiln/worktrees/<item>`, created by `rk_git::worktree` under D21-45 W4 as D21-46 R8, D21-47 V1 and D21-48
X2 refine it: creation is refused only when pointer files would result, and a post-checkout scan refuses any LFS
pointer header the filter selects. On BrittonGames, W4 would not refuse: its LFS filter is effective at `system`
scope (`s8i` §8i.0 item 12, and ADR-0180's measurement of Git for Windows' default), so the run worktree holds the
assets' bytes, not their pointers. S-BIN's own review trees were fresh repositories, not run worktrees, so this is
named from the records and is **not measured** by S-BIN; the Phase-2 adapter's record measures it on its first run.

**8. What this record supersedes, by ADR-0092 rule 1 applied to the text ADR-0165 wrote.**

- **Superseded in part: ADR-0165 decision 1, its second bullet's sentence** *"It cannot be constructed outside
  `rk-ports`, and no fake or adapter constructs it inside `rk-ports` either."* That sentence carries no condition in
  its own words, and decision 2 above makes it false: the constructor is public, and the render and review-input
  tests construct the arm outside `rk-ports`. A sentence of `## Decision` that this record makes false moves by
  supersession, not by execution. The bullet's other sentences stand (*"`Binary` is a slot whose payload type is
  sealed"* is executed by decision 7 of ADR-0165; *"S-BIN fills it at `s8i`"* is executed here).
- **Executed under it, not superseded:** ADR-0165 decision 7 (*"It opens by a later record once S-BIN reports"*, and
  the three pins it names, superseded by name as it provides); its third forbids bullet, whose own words end
  *"until a later record opens the slot"*; its seventh forbids bullet (*"Forbids designing or locking the `Binary`
  payload against anything but the real UE project's artefacts"*), which S-BIN satisfies.
- **Standing unchanged:** ADR-0165 decisions 2 to 6 and 8, its other forbids bullets, and ADR-0135's fourth forbids
  bullet. Decision 6's refusal stands under decision 4 above; its registry `reason` text naming the sealed arm is
  wording, not a decision, and the commit that lands the constructor may re-date it.
- ADR-0165 stays `accepted`. Its `superseded_by: ["0199"]` back-link and its README status cell move in this
  record's flip commit, not before, because an accepted record is not marked superseded by a `proposed` one.

## Consequences

**Positive**

- The `Binary` arm is designed against the real project's artefacts (ADR-0008), from what the reviewers used, and
  the route follows the registered map mechanically, so Checkpoint 8i can derive it from the records.
- A `Binary` item never receives an automated verdict the evidence does not support, and the human sees the typed
  summary, not a pointer diff.
- The kernel stays type-neutral: one closed reason, one opaque attachment, the existing reach path.

**Negative**

- **A range mixing text and binary paths is unmeasured.** No S-BIN item changed a source or test path, so the arm
  carries no text diff, and the `Text` arm refuses any range with a `Binary` path (ADR-0165 decision 6). Such a
  range's text half reaches the human gate only through the run worktree, not through this summary. The Phase-2
  adapter's record owns that question, with evidence.
- **Catalog steps that read `sha`, `diff`, `files` or `manifest` resolve nothing over a `Binary` value.** With no
  Phase-1 producer nothing reaches them; ADR-0008's Phase-2 Unreal adapter record owns the catalog's `Binary`
  bindings.
- **The human gate costs a human answer per binary item** until a later record moves the route.
- **The field set rests on n = 5, two passes** and on a rendering whose report carried the seeded items' diagnosis
  (D21-152 item 2). It is a type-design sample, not a rate (METHOD-S §3.6).
- Three `s8a` pins are superseded by name (decision 2), and ADR-0165 gains a back-link at the flip. A reader who
  finds them gone after `s8i` must read that as the plan.
- **The refusals do not hand the input back (a stated deviation from rust-craft's ERR-14).** *(Added 2026-10-14,
  s8i build, S8I-D1-PORTS; the independent review's F4.)* `BinaryChangeSummary::new` refuses with
  `TextPatchForBinaryPath { path }` and drops the other assets, the captures and both references;
  `BinaryAsset::from_pointer` refuses with `NotAnLfsPointer { path, defect }` and drops `kind`; ~~and `rk-cli`'s
  `review_input::screen`, as C7 built it, refuses a `Binary` value by name and drops the value~~ *(dated 2026-10-14, s8i build,
  S8I-D2-WIRE-GATE: `screen` is removed. Its successor, `review_input::route`, keeps the refused `Binary` value and the
  gate carries it, so that refusal hands nothing away and is no longer a deviation)*. The reason is the
  same in each: the refused input is invalid by this record (a text patch of a `Binary` path, or a blob that is not
  a git-lfs pointer, or a `Binary` value at an automated reviewer), and no caller has a retry that could use it. A
  producer that meets the refusal rebuilds from its own sources, and the error names the path it needs. A later
  record that gives a caller a retry over a refused input re-opens this.
- **The gate's expiry is `fail`, and what an ANSWERED gate does is owed.** *(Added 2026-10-14, s8i build,
  S8I-E2-EXPIRY-POLICY; operator ruling R9.)* `automated-verdict-refused` is a kernel-minted gate kind (decision 3), so
  ADR-0198 decision 6's fail-closed table gains one row for it in both shipped packs, `action = "fail"`, in the form of
  every other row: a lapsed gate expires with `fail` rather than leaving `run` and `answer` to refuse with a
  configuration error, and `run`'s load-time lint (ADR-0198 decision 7) counts the class among the minted kinds, so a
  pack without the row refuses at load. An ANSWERED gate is not yet acted on: today the next run re-walks the step,
  the review input is the same `Binary` summary, and the step is refused again, so the human answer does not carry the
  item past review. What an answer does (accept as the review's evidence, reject, or send back) is owed to the first
  `Binary` producer, ADR-0008's Phase-2 Unreal adapter (decision 4 reads none in Phase 1, so no Phase-1 run reaches the
  gate); ~~ledger issue owed (draft at sp/s8i)~~ *(dated 2026-10-14, s8i build: filed by the operator's go-ahead as Wet-Ink-Corporation/redkiln-rs#80)*.

**What this forbids**

- Forbids a `BinaryChangeSummary` field outside the four in decision 1, and forbids dropping one of the four,
  without a later record citing a measurement.
- Forbids constructing the `Binary` payload except through the one checked constructor in `rk-ports`, and forbids a
  `pub` field on it.
- Forbids a `Binary` value carrying a text patch or diff of a `Binary`-classified path (AC-497's AND).
- Forbids inferring `assets[].size` from whether the pointer's size line moved; it is read from the pointer.
- Forbids an automated review verdict on an item whose range carries a `Binary`-classified path, until a later record
  moves the route on new evidence.
- Forbids `crates/rk-kernel/src` naming the `ChangeSummary` type, carrying a gate reason outside the closed set as
  free text, or parsing, decoding or typing the gate attachment's bytes.
- Forbids opening the refusal's gate with a token not derived through `GateRecord::derive_reach`, or through a gate
  step's free `kind` string with no attachment.
- Forbids a Phase-1 toolchain adapter constructing the `Binary` arm, and forbids removing or weakening the three
  adapter refusal pins while the producer line reads none.
- Forbids recording `pinned` for a `binary`-class AC by the mutation path where the toolchain declares mutation
  infeasible, and forbids exempting the class from AC-334.
- Forbids citing F43 as a detection or false-positive rate, or as *"ChangeSummary alone sufficient"*.

## Evidence and confidence

- [measured 2026-10-14, corpus `835b122`, read through the object store] the ten records, `cited_fields` per record,
  each pass's detection and false positives, `missing-fields.jsonl` empty (0 bytes), ten `clear` shadow rulings; the
  pinned `audit-s.py bin` re-run reads `rule_outcome_pass1=branch1`, `rule_outcome_pass2=undecided-fp` (F43). The
  final `corpus_commit`, which also holds `results/s-bin/audit.txt`, is the operator's corpus write and is owed.
  *(dated 2026-10-15, s8i build, S8I-F2-RECORDS; s8i refuting round 21's records minor: no longer owed. The operator committed `results/s-bin/audit.txt` as corpus `e1e90d1` (`e1e90d11a716bc43046e31ddab6282a6e91708b4`, a descendant of `835b122`), whose outcome lines read `branch1` and `undecided-fp` as above; F43's pins block is filled at that commit, 58 `file=` lines, `checked=58 owed=none bad=0`. The frontmatter's `corpus@835b122` stays: it names the records' commit, and `e1e90d1` adds only `audit.txt`.)*
- [measured 2026-10-14, the driver's blob `runner/src/sbin.rs` at `835b122`] the citation vocabulary's eight
  targets; [measured, `results/s-bin/rendered/item-4.txt`] the rendered summary's members.
- [measured 2026-10-14 at `42a923f`] `pub struct BinaryChangeSummary(pub(crate) ());` in
  `crates/rk-ports/src/toolchain.rs`; `GateRecord::derive_reach` in `crates/rk-kernel/src/workflow/fsm.rs`;
  `pub fn reach(state: &RunState, kind: &str, ctx: &FsmContext<'_>)` in `crates/rk-kernel/src/workflow/gate.rs`.
- [documented] D21-43 N21, D21-48 X3 and X16, ADR-0165 decisions 1 and 7 and its forbids, ADR-0092 rule 1, ADR-0008,
  AC-337, AC-356's struck clause, AC-357, `s8a` §8a.2 item 2a, ADR-0180's W4/V1/X2 account, all quoted above.
- [inferred] decision 7's run-worktree source for a Phase-2 producer: named from ADR-0180 and §8i.0 item 12, not
  measured on a run worktree holding a `Binary` change.
- **Confidence.** High in the route: it is the registered map applied to two computed outcomes, and both applicable
  rows agree. Moderate in the field set: it is what one reviewer, in two passes over five items, cited, under a
  rendering that carried every field. Low in anything beyond type design (F43's Limits 1 to 7).
- **What this evidence does not license.** No claim that a reviewer reliably approves clean binary changes, no rate,
  and no claim about mixed text and binary ranges or about asset types S-BIN could not capture.
