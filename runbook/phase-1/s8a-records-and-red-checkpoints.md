# Step 8a — Records owed and the three red checkpoints (Phase 1, part 8, sub-stage a)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8a` | Phase 1 | Checkpoint 7i | Checkpoint 8a | `s7i` | `s8b` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---

**Why this step exists.** `spikes/DECISIONS-2026-09-21.md` D21-40 decision 4 reads *"Checkpoints 5, 7b and 7f are
repaired before Phase 1 exits. They are not carried as debt, even though none of them is on `s8`'s entry chain."*
All three are red at the authoring HEAD `ca8f387` (D21-40's table, re-measured by the lead):

- **Checkpoint 5** fails on its `ChangeSummary` line. Its filter chain returns six sites: the `use` line and the
  `change_summary` signature in each of `crates/rk-adapter-toolchain-node/src/lib.rs`,
  `crates/rk-adapter-toolchain-rust/src/lib.rs` and `crates/rk-cli/tests/it/review_pass.rs`. Two more sites break
  ADR-0099's forbids bullet to the letter but pass the filter, because it excludes `::ChangeSummary`. They are
  `crates/rk-cli/src/verbs/run.rs` (`-> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError>`, on
  `UndetectedToolchain`) and `crates/rk-kernel/tests/it/runtime.rs` (`CountingToolchain`). Its doc comment says
  *"The return type is written `rk_ports::toolchain::ChangeSummary` and the name is NOT imported"*, which is a
  deliberate step around the checkpoint. Eight sites in all, and every one is an import or the signature of the
  port's own method. The cause is a conflict between accepted records. ADR-0094 decision 1 and operator decision
  D17-9 require implementors. ADR-0099's forbids bullet bans the name. ADR-0135 clause 5 describes the ban and
  extends it.
- **Checkpoint 7b** pins `predicates parsed 285/287; failed 2/287`. `python spikes/p9/grammar-probe.py` prints
  `predicates parsed 278/279; failed 1/279` and `on_false forms 96/99`, because Step 7e's `spikes/p9` edits at
  `dddcdbd` dropped eight `when` predicates. ADR-0114 decision 1 said *"The two F42 parse failures and three
  `on_false` defects are repaired at Step 7e before the tag"*, but only `packs/incident.toml` was repaired. The
  other four defects are still in `spikes/p9/catalog-b/`.
- **Checkpoint 7f** fails with *"the three conformance lints are not all ARMED"*. `s7g` and `s7h` discharged two of
  them, each removing its `#[ignore]` in the commit that made it pass, which is `s7f`'s own convention. D21-9
  recorded this in advance: *"nothing downstream requires it green"*.

D21-41 (i) and D21-42 decisions 2, 22, 23 and 24a turn those repairs into records. Two of the records have
consequences beyond their checkpoints. The first finalises `ChangeSummary`'s **`Text` variant** now and seals its
**`Binary`** slot for S-BIN, which runs at `s8i`. So the port gains a real value, and the Rust adapter gains a real
`change_summary` body. The second moves the Codex adapter's phase to Phase 4 (ADR-0007).

🛑 **This stage takes decisions, and names them here in its header** (ADR-0092 rule 2.5). The two preceding stage
files each said they took none, and each was wrong. This stage writes **three records** plus two errata, and it
decides nothing that D21-41 or D21-42 did not already decide. None of the three is pre-numbered. Each one is *"a new
record (next free id at writing; `0165` at the time this file was authored)"*, allocated in writing order. Each one
is committed `proposed` and flips only on the operator's dated ledger line. Checkpoint 8a asserts `accepted`,
because every checkpoint amendment below rests on one of them.

🛑 **No real dispatch in this stage.** D21-40 decision 2 gates spend on the fixture proofs. The first stage that
dispatches to a real account is `s8i`. §8a.9 guards the one place this stage's exit runs `redkiln run`.

### 8a.1 The entry gate, and the three reds read before anything moves

Run Checkpoint 7i whole with `FIX_ITEM=rk-fix-doctor-test-count`, and confirm it ends with
`CP7I-END-OF-BLOCK reached`. Then extract Checkpoints 5, 7b and 7f from their files' own bytes and run each one
whole, exactly as Checkpoint 8a's helper does in its step (11). Record each rc and the line it failed on. These are this
stage's **BEFORE** readings (PIA-035), and all three must be rc=1 with the reasons listed above. A reading that is
already green means the tree has moved since `ca8f387`. In that case stop and re-measure; do not carry this file's
premise forward. `s7h` and `s7i` both paid for carrying a premise forward.

Back up `.git/redkiln/store.sqlite3` before the first whole run. Checkpoint 7f's block runs `redkiln run`, which
appends to the store.

### 8a.2 `ChangeSummary` splits: a finalised `Text` variant and a sealed `Binary` slot (D21-42 decision 2)

**The record** (a new record, next free id at writing; `0165` at authoring) is D21-41 (i)'s Checkpoint 5 record
with D21-42 decision 2 folded into it, and it uses `## Considered options` with `### Option — <name>` headings
(§8a.7). It fixes the following:

1. **Home and opacity.** `ChangeSummary` is defined only in `rk-ports`, as an enum with two arms. `Text` carries a
   typed diff plus a manifest and is **finalised now**. `Binary` is a slot whose payload type is sealed: it cannot
   be constructed outside `rk-ports`, and no fake or adapter constructs it inside `rk-ports` either. S-BIN fills it
   at `s8i`. The record names the two payload types; this file calls them `TextChangeSummary` and
   `BinaryChangeSummary`, and if the record names them otherwise, Checkpoint 8a's patterns follow it in the same
   commit.
   - No crate outside `rk-ports` defines, aliases, redefines or implements a trait for any `…ChangeSummary` type.
   - An implementor of `Toolchain::change_summary` may import the type and name it in that method's signature.
     That includes production code and test doubles in any crate, `rk-kernel`'s `tests/` among them.
   - A `Toolchain` implementor may construct the `Text` arm.
   - `rk-kernel`'s production code still names the type nowhere. The surviving half of ADR-0135's fourth forbids
     bullet stands for `crates/rk-kernel/src/`.
2. **The serialised `Text` value carries the members the shipped catalog already reads:**
   - `sha` — `adversarial-review.toml`'s `clone` step reads `inputs = { sha = "artifact:ChangeSummary.sha" }`;
   - `diff` — `change-summary.toml`'s `lint` step reads `inputs = { diff = "${summarise.outputs.diff}" }`;
   - `manifest` — per path: the path, its `classify` result, and its change kind.

   A field set that misses a member the catalog reads is a value that resolves nothing.
3. **Ratification.** The record ratifies PROGRESS §2's D17-1 readings 2 and 3: a string literal, a comment and the
   `Capability::ChangeSummary` vocabulary variant do not name the type. It names **D17-9** as the root of the
   conflict. D17-9 chose adapter implementors while it declared *"ADR-0099 decision 5's forbids, unchanged"*, and
   those two things could not both hold.

**Every supersession, in ADR-0092 rule 1's form.** Each item below moves a `## Decision` sentence or a **What this
forbids** bullet, so each is a supersession in part, never an erratum.

| superseded **in part** | the sentence or bullet that moves (quoted) | what stands |
| --- | --- | --- |
| **ADR-0099** clause 5 and its forbids | *"`ChangeSummary` is an **opaque, non-finalised newtype in `rk-ports`** until S-BIN reports; Checkpoint 5's grep for its absence covers `crates/` minus that one site."* and *"Forbids `ChangeSummary` appearing anywhere in `crates/` but its `rk-ports` newtype until S-BIN reports."* | Clause 5's crate-name sentences. The back-link is an **append**: `superseded_by: ["0115", "0127", <id>]`. The record must say it overrides a forbids bullet that ADR-0127 affirmed, in 0127's words: *"every bullet of its forbids stand"*. |
| **ADR-0135** decision 4, decision 5, and the third and fourth forbids bullets | *"**It must not be "fixed" into a synthesised value.**"*; *"Checkpoint 5's grep bans the bare type outside `crates/rk-ports/`, after blanking string literals and excluding `::ChangeSummary` paths."* and *"**Checkpoint 5's RED is untouched by this record.**"*; *"Forbids synthesising a `change_summary` value … and forbids editing either adapter's `change_summary` row to make the step succeed."*; *"Forbids naming the `ChangeSummary` **type** anywhere in `rk-kernel`"*, **only** as far as a test double's impl signature under `tests/` | The map, decisions 1–3 and 6, and the ban on production `rk-kernel`. Back-link: `["0148", <id>]`. |
| **ADR-0093** decision 4 | *"S-BIN blocks the design of `ChangeSummary` (AC-497, AC-356) and the Toolchain port's `change_summary` method body"*. S-BIN now blocks the `Binary` arm and its body only. | Decision 4's scope rule, and its forbids bullet unchanged. |
| **ADR-0094** decision 1 | *"`change_summary` remaining an opaque non-finalised newtype until S-BIN (ADR-0093 decision 4)"*. ⚠️ **D21-42 decision 2 does not name this sentence**, but it is a `## Decision` sentence that the split moves, so rule 1 makes it a supersession too (finding (1) below). | Every other clause of decision 1. |

**Owed beside the record, by dated note and not by rewrite.**

- **Design.** The paragraph that says *"meanwhile `ChangeSummary` is an **opaque, non-finalised newtype in
  `rk-ports`**, the one site Checkpoint 5's absence grep over `crates/` excludes"*, and §9.1's crate row
  *"`ChangeSummary` (an opaque, non-finalised newtype, the one site Checkpoint 5's absence grep excludes — ADR-0099
  clause 5)"*.
- **Register.** AC-497 (*"an **opaque, non-finalised newtype in `rk-ports`**"*). AC-154 and AC-356 stay PROVISIONAL
  on the `Binary` half only.
- **The README.** The index rows and back-link cells, the census, and the *"next free number"* sentence. The same
  sentence also appears in `00-preamble.md`.

**The code.** Write the tests first (§8a.8). Then make these changes:

- **`crates/rk-ports/src/toolchain.rs`.**
  - The enum replaces `pub struct ChangeSummary(pub(crate) ());`.
  - The doc comment's *"This is the one site in `crates/` the name may appear"* and *"no real adapter implements it
    at this stage"* are corrected, with dates.
  - `FixtureToolchain` returns `Text` for text paths it was scripted with. It refuses by name, never with an empty
    `Text`, for a range that carries a `Binary` path.
- **The range's diff is read by `rk-git`.** The Rust adapter's `Cargo.toml` says *"no git is spawned by this crate
  (ADR-0077 confines git to rk-git's one wrapper…)"*, and that stands. The read uses the crate's existing read
  convention: `GitOp`'s reads section already holds `RevParse`, `Status` and `Log`. It is not a write, so ADR-0105
  clause 5's enumerated write set does not move. If the read needs anything that clause or ADR-0077 forbids, halt.
  Do not shell out from the adapter.
- **`RustToolchain::change_summary` returns `Ok(ChangeSummary::Text(…))`** when every path in the range classifies
  `Text` or `Generated`.
  - A range that carries a `Binary` path refuses through the registry's dated `change_summary` row. The row is
    re-dated, and its `reason` is narrowed to the sealed arm.
  - An empty or unresolvable range refuses by name. It never returns an empty `Text`. ⚠️ **This matters today:**
    `crates/rk-kernel/src/runtime/step.rs` calls `"change_summary" => toolchain.change_summary("").map(|_| ())`.
    The kernel passes an **empty** range and discards the value until `s8d`'s resolver binds `artifact:Changeset.sha`.
    An adapter that answered `Ok` to `""` would turn eleven refusals into eleven vacuous greens.
- **The Node adapter's row, body and registry tests are unchanged.** Its `change_summary` still refuses. Finalising
  a Node body is not in D21-42 decision 2 and is not taken here.
- **Stale texts.** Four sites get dated corrections:
  - `crates/rk-kernel/tests/it/runtime.rs`'s path-qualification comment (the exclusion it avoided is gone);
  - the two adapters' `registry.rs` module docs (*"no `Ok` is constructible"*);
  - `crates/rk-cli/tests/fixtures/lint-controls/ac475_change_summary_control.rs`'s prose.

  AC-475's tripwire over the migrator is unaffected, and asserted.

### 8a.3 Checkpoint 5, repaired by amendment under the new record (D21-41 (i))

`s5`'s `ChangeSummary` statements are replaced by a dated amendment that cites the record. The ban now names what
the record forbids. The `::ChangeSummary` exclusion is **dropped**, so a path-qualified definition, alias or impl
fires like a bare one. The amendment contains:

- **The ban.** `crates/` minus `rk-ports`, code lines only, string literals blanked. It rejects any
  `struct`/`enum`/`type`/`trait`/`union` named `…ChangeSummary`, any `type X = …ChangeSummary` alias, any
  `impl … for …ChangeSummary`, and any inherent `impl …ChangeSummary`.
- **The `rk-kernel` production ban.** `crates/rk-kernel/src`, code lines only, strings blanked: no
  `\bChangeSummary\b`.
- **The opacity pin inside `rk-ports`.** The ban excludes `rk-ports`, so this pin is what guards opacity there.
  `rk-ports` declares exactly one `pub enum ChangeSummary`, and the sealed payload type declares no `pub` field. The
  **behavioural** pin is the `trybuild` case in §8a.8, because a grep cannot prove non-constructibility.
- **Synthetic controls, through the same chain, one line at a time.** A combined control would pass on a single
  hit. These must fire: a definition, a `Text…` redefinition, a path-qualified alias, a trait impl, a path-qualified
  inherent impl, a generic impl on the sealed type, and an impl on a line that also carries a string. These must be
  admitted: the `use` line, both signature spellings, `Self::ChangeSummary =>`, a bare `ChangeSummary,` variant
  line, a `Text` construction, a comment, and a string.

The positive control `pub struct ChangeSummary(` in `s5` no longer matches, and the same line appears in two other
places. Each one moves in the same commit, by dated amendment citing the record:

- **Checkpoint 7a's line**, *"expected exactly 1 — ADR-0099 clause 5 (:90-91) decides the type is an opaque,
  non-finalised newtype"*. Checkpoint 7a is re-run whole at this exit, because this stage moves its subject.
- **`s8`'s bullet and guard**, *"No finalised `ChangeSummary` type; no synthesised `ChangeSummary` value"*. The
  bullet becomes *"no `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it"*. The
  guard's comment *"RED with four hits in the two adapters"* is corrected: eight literal sites, six through the old
  filter. ⚠️ **The stage plan assigns this edit to no stage** (finding (2)). If it does not land here, Checkpoint 8's
  mechanical block, green at HEAD, goes red at this stage's commit.

### 8a.4 `catalog-b`'s four defects, and Checkpoint 7b re-pinned (D21-41 (i); D21-42 decision 23)

**The repair.** Patch terminator-agnostically and assert a hit count of exactly 1 per anchor before writing
(golden constraint 7; the memory rule on mixed line endings). Run the patch from Python, never with `sed -i` over
a smudged file.

| file | from | to | N's warrant |
| --- | --- | --- | --- |
| `spikes/p9/catalog-b/blameless-postmortem.toml` | `kind in [incident, security-vuln]` | `kind in [incident, 'security-vuln']` | ADR-0048's quoting rule; F42's own suggested repair |
| same, step `complete` | `on_false = "draft"` + `max     = 2` | `on_false = { to = "draft", max = 2 }` | the step's own sibling `max = 2`, which moves into the table. 62 of 66 loop-backs in `spikes/p9` use `max = 2` |
| same, step `reviewed` | `on_false = "draft"` + `max     = 1` | `on_false = { to = "draft", max = 1 }` | the step's own sibling `max = 1`; `max = 1` is the sibling convention's second form (3 of 66) |
| `spikes/p9/catalog-b/api-contract-spec.toml`, step `policy` | `on_false = "assemble"` + `max     = 2` | `on_false = { to = "assemble", max = 2 }` | the step's own sibling `max = 2` |

In each row the stray step-level `max` line is removed, because its value now lives in the table. This was measured
on 2026-09-24 over a `git archive HEAD spikes/p9` copy in the scratchpad, not over the tree, because the probe
rewrites a tracked file. After these four edits the probe prints `predicates parsed 279/279; failed 0/279`,
`mixed unparenthesised and/or (precedence-dependent) 0/279`, and
`on_false forms 99/99 in the closed set: {'loop-back': 69, 'keyword': 30}`, and it exits 0.

`catalog-b` is the unshipped extended file (F42's defect table: *"no — catalog-b is the unshipped extended file
(ADR-0027)"*). ADR-0072's rule for a **shipped** activity edit (version bump, changelog row, P6 re-run) therefore
does not reach it. Re-run the probe and commit the regenerated `spikes/p9/results-s-grammar.json` in the same
commit. The probe `json.dump`s into it on every run.

**The record** (a new record, next free id at writing) supersedes **ADR-0114 decision 1 in part**, at the sentence
*"The two F42 parse failures and three `on_false` defects are repaired at Step 7e before the tag; nothing else
stands between the v0 pack and its tag."* The record states what the tag's condition was **over the shipped
pack**: decision 1's condition (b) was met by the one in-scope repair, and `redkiln-pack-v0` stands. The other four
defects were never in a shipped pack, and they are repaired here, after the tag. Conditions (a) and (b), decision 2
and every forbids bullet stand. Back-link: ADR-0114 `superseded_by: [<id>]`. Checkpoint 7b's re-pin is folded into
this record (D21-42 decision 23), and it is a re-pin rather than a widening:

- `grep -qxF 'predicates parsed 279/279; failed 0/279'`;
- `grep -qxF 'mixed unparenthesised and/or (precedence-dependent) 0/279'`;
- `grep -qE '^on_false forms 99/99 in the closed set'`;
- the probe's rc is tightened from `-le 1` to `-eq 0`, because a clean probe exits 0;
- the stale comment *"s7e:117 expects 287/287 afterwards"* and the FAIL message naming `exists(ProblemRecord)` are
  rewritten, with dates.

**Owed beside it.**

- A dated correction to `spikes/F42-DRAFT.md`, under rule 2.4, beside its *"**285/287** parse; 2 fail"*.
- Dated notes in `s7e` §7e.2, whose *"the count settles at **278/279**"* was true until this stage.
- The historical-capture doc comments in `crates/rk-kernel/tests/it/probe_differential.rs` (*"valid **until `s7e`
  §7e.2 repairs the five defects**"*) and `crates/rk-kernel/tests/it/on_false.rs` (*"the **only live instances**"*).
  Both tests pin F42's historical measurement over literals and a captured fixture, and their assertions do not
  move.

⚠️ D21-41 (i) said *"re-pinned to 278/279"*. D21-42 decision 23, which was taken later and adds the `catalog-b`
repair, makes the measured reading 279/279. This stage pins what the probe prints after the repair.

### 8a.5 Checkpoint 7f: D21-9 executed

D21-9 already decided the disposition, and ADR-0134 decision clause 5 named the consequence (*"**Checkpoint 7f
becomes non-re-runnable at that commit.**"*). That sentence is about the text 7f carried then, and that text stays
non-re-runnable. The re-scope below is a new, dated text, not a reversal of the clause (finding (3)). Executing D21-9
has two parts:

1. **The tracker.** `PROGRESS.md` row 7f's *"RE-RUN PENDING"* becomes a terminal disposition that quotes D21-9, so
   that it does not read as dropping the preamble's re-run obligation. The `79e0ef4` pass stands as evidence about
   the text it attested.
2. **The literal is re-scoped to the discharged-state form**, by dated amendment in `s7f` §7f.5, so that Checkpoint
   7f can be run whole at this exit (D21-40 decision 4: *"repaired … not carried as debt"*). The block asserts the
   state that is true now:
   - exactly three `conformance_` lints;
   - `conformance::conformance_every_step_kind_has_an_executor` and
     `conformance::conformance_every_port_has_a_call_site` each `... ok`;
   - `conformance::conformance_every_trailer_key_has_a_producer` ignored with `ARMED: Slice-Verdict has 0
     producers`;
   - `test result: ok. 2 passed; 0 failed; 1 ignored`.

   The armed reason's tail *"discharged by s7h"* is false: s7h did not discharge it (ADR-0160). It is corrected to
   `discharged by s8e`, which is D21-41 (e)'s stage. `s7f`'s reason regex follows. `s7h` guard (6) greps only the
   reason's prefix and is unaffected.

⚠️ **Packet (i) option A read D21-9 as "a PROGRESS cell edit only … no checkpoint edit".** This stage does both,
because D21-40 decision 4 and this stage's brief require 7f to run whole at the exit (finding (3)). `s8e`'s
Slice-Verdict commit moves this literal again, together with `s7h` guards (6) and (7).

### 8a.6 Codex's phase (D21-42 decision 22)

**The record** (a new record, next free id at writing) supersedes **ADR-0094 decision 2 in part**, at *"The Harness
port's second real adapter (Codex) is Phase 2 and stays behind the CI decision ADR-0010 defers"*, toward ADR-0007,
which is accepted and has not been superseded: *"the Codex adapter ships in Phase 4 ("reach"), and only after
S-CODEX passes."* The rest of decision 2 stands, including AC-499's phase clause. Back-link: ADR-0094
`superseded_by` gains this id beside §8a.2's.

**E-0094-2** (ERRATA, under ADR-0094) records that DECISIONS-2026-09-12 E12 was taken with ADR-0007's phase
sentence out of view. E12's row cites only ADR-0094 and the design. It corrects no Decision.

**Dated corrections**, each re-measured before it is written:

- **The design.** ⚠️ The phase-bearing S-CODEX row is in **design §10.4** (Open items): *"**Phase 2 is the
  phase** (ADR-0094 decision 2, 2026-09-11 …)"*. It is not in design §10.5. The design §10.5 S-CODEX row states no
  phase, and the design §10.2 phase table already says Phase 4 (finding (4)).
- **The registers carry six sites, not two.**
  - Four are in `references/2026-09-08-acceptance-criteria.md`: the `| 5 | The second harness |` row, the area's
    *"Phase-1 subset"* note, AC-496's body, and the *"Phasing means roughly half the area"* bullet.
  - Two are in `references/2026-09-08-success-measures.md`: SM-148's baseline correction and its table row.
    `success-measures.md` already says Phase 4 in its area summary.
  - The runbook's `s7a` (one site: AC-496's exclusion), `s7b-ii` (two sites) and `s7c` (one site) exclusion reasons
    also say *"Phase 2"*. They take dated notes.

  Checkpoint 8a's sweep (9) requires every register line that matches `Codex[^|]*\bPhase 2\b` to cite the new
  record. At `ca8f387` it matches exactly those six lines.

`rk-story-codex-adapter` is retired at `s8j` (D21-41 (f)), not here.

### 8a.7 ADR-0085's option headings, enforced from ADR-0164 on (D21-42 decision 24a)

`runbook/tools/check-adr-options.py` is added beside the other two checkers. It works as follows:

- It reads every `references/adr/NNNN-*.md` with `NNNN >= 0164`, or the records in `--dir <path>` for the controls.
- Under `## Considered options` it requires at least one `### Option — <name>` heading, and no `###` heading of any
  other form.
- It prints `scanned N records (>= 0164); flagged K`. It exits 1 when K > 0 **or when N = 0**. A lint over zero
  records is the `lattice.py --packs <path>` green that printed a baseline over zero packs.
- `INDEX.md`'s validation block gains it at the tracker commit.

**One dated erratum** goes under ADR-0085's section (next free `E-0085-n`; `E-0085-3` at authoring). It lists
ADR-0132 through ADR-0161, **thirty records**, as non-conforming to *"Each considered option is one `### Option —
<name>` heading"*. Measured at `ca8f387`: each of the thirty has zero such headings, and 0162, 0163 and 0164 carry
3, 7 and 4. **Their accepted bodies are not edited**, and Checkpoint 8a asserts that byte for byte against the
stage's base commit. All three of this stage's records are written in the conforming form.

### 8a.8 Red first — the named tests

Each test is watched red for the **right** reason (the absent behaviour, not a compile error or a filter that
matches nothing) before the code that turns it green.

| crate | `tests/it` name | red because |
| --- | --- | --- |
| `rk-ports` | `change_summary::text_variant_serialises_sha_diff_and_manifest` | there is no `Text` arm, so no `sha`/`diff`/`manifest` members reach a catalog reference |
| `rk-ports` | `change_summary::binary_arm_is_not_constructible_outside_rk_ports` | a `trybuild` case (`tests/ui/change_summary_binary_not_constructible.rs` plus a committed `.stderr`) that must fail for the **privacy** reason. It is its own harness test: Checkpoint 7a counts `trybuild_pins_the_reason_each_port_fixture_fails`'s four cases |
| `rk-ports` | `fake_behaviour::toolchain_fake_classifies_binary` *(amended)* | its `assert_eq!(format!("{summary:?}"), "ChangeSummary(())")` becomes: a text range yields `Text`, and a binary range refuses by name |
| `rk-git` | `diff::range_diff_lists_every_changed_path_with_its_change_kind` | no range read exists |
| `rk-git` | `diff::range_diff_refuses_an_unresolvable_range` | same |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_text_variant_from_git_diff` | the body answers `Unsupported` from the registry. Over a two-commit fixture repository it must return `Text` with one manifest row per changed path |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_a_range_carrying_a_binary_path` | a `.uasset` in the range must refuse through the re-dated row and never yield a `Text` that silently omits the path (AC-497) |
| `rk-adapter-toolchain-rust` | `change_summary::change_summary_refuses_an_empty_range` | the kernel passes `""` today (§8a.2), and an `Ok` there is a vacuous green |
| `rk-adapter-toolchain-rust` | `registry::ac489_capability_registry_dated` *(amended)* | its *"change_summary answers Unsupported from its registry row"* over `HEAD~1..HEAD` moves to the binary case |

That is seven new tests and two amended ones. Unchanged, and asserted as regression pins:

- `rk-adapter-toolchain-node`'s `registry::ac489_capability_registry_dated` (Node still refuses);
- `rk-kernel`'s `probe_differential::grammar_agrees_with_the_s_grammar_probe` and `on_false::on_false_closed_set`;
- `redkiln-migrate`'s `output::ac475_zero_change_summary_values_in_output`.

⚠️ **Checkpoint 7c becomes provisional.** Its named `ac489_capability_registry_dated` is amended here. Its block must
run from a shell outside Claude Code, and it makes a paid haiku probe, so it cannot be re-run before `s8i`.
`PROGRESS.md` row 7c is marked *"passed …, re-run pending `s8a`'s `change_summary` body"*, and the re-run belongs
with `s8j`'s probes. The stage plan does not list it there (finding (5)).

### 8a.9 Checkpoint 8a

- **The three records exist and are `accepted`.** Each carries its supersession back-links, and each uses `### Option
  — ` headings. The two errata exist. The thirty non-conforming bodies are byte-unchanged.
- **`ChangeSummary` is split.** `Text` is finalised and `Binary` is sealed. The Rust adapter produces `Text` through
  `rk-git`, and refuses a binary path and an empty range by name.
- **Checkpoint 5's new ban holds**, including the `rk-kernel` production ban and the opacity pin. Every control
  fires or is admitted as designed.
- **`catalog-b` is repaired**, the probe prints 279/279, 0/279 and 99/99 at rc 0, and the committed JSON is
  unchanged by the re-run (compared as parsed JSON, not with `git diff` under autocrlf).
- **Checkpoint 7f's discharged state is asserted by name.**
- **The Codex phase sites carry their dated corrections, and E-0094-2 exists.**
- **The whole suite is green** above its floor, and fmt and clippy are green.
- **Checkpoints 7i (entry), 5, 7a, 7b and 7f each run whole from their own bytes and reach their last line.**

🛑 **ADR-0164's flip is not this stage's act.** It is `proposed` and flips only on the operator's ledger line. This
block does not read its status.

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# Run it AFTER `git add` of the work: (5), (8) and Checkpoint 7a read the index and the tracked set.
# Every cargo reading is `out=$(…); rc=$?`, never a pipe's status. Every selection is `--test it`,
# MODULE-QUALIFIED and `--exact`, and asserts each `test <name> ... ok` line AND an exact count.

# ---- (1) BUILD-OR-FAIL, FIRST. A stale binary makes every later whole run a statement about old code.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — Checkpoint 8a NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -f ./target/debug/redkiln || test -f ./target/debug/redkiln.exe || { echo "FAIL: the build succeeded and no redkiln binary exists"; exit 1; }

# ---- (2) INPUTS. The ids are allocated at writing, so this block takes them rather than guessing them.
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM=rk-fix-doctor-test-count — Checkpoint 7i's whole run needs it"; exit 1; }
test -n "$CS_ADR" && test -n "$PACK_ADR" && test -n "$CODEX_ADR" || { echo "FAIL: set CS_ADR, PACK_ADR and CODEX_ADR to the ids of the ChangeSummary, ADR-0114 and Codex-phase records"; exit 1; }
test -n "$S8A_BASE" && git rev-parse --verify --quiet "$S8A_BASE^{commit}" >/dev/null || { echo "FAIL: set S8A_BASE to the commit this stage opened on — the erratum's untouched-bodies guard compares against it"; exit 1; }
export FIX_ITEM

# ---- (3) THE THREE RECORDS: one file each, accepted, supersessions declared, back-links present (ADR-0092 r1).
for id in "$CS_ADR" "$PACK_ADR" "$CODEX_ADR"; do
  printf '%s\n' "$id" | grep -qxE '0[0-9]{3}' || { echo "FAIL: '$id' is not a four-digit record id"; exit 1; }
  set -- references/adr/"$id"-*.md
  [ "$#" -eq 1 ] && [ -f "$1" ] || { echo "FAIL: expected exactly one references/adr/$id-*.md"; exit 1; }
  grep -qxF 'status: accepted' "$1" || { echo "FAIL: $1 is not accepted — the checkpoint amendments rest on it; a proposed record flips only on the operator's dated line"; exit 1; }
done
cs_f=$(ls references/adr/"$CS_ADR"-*.md); pk_f=$(ls references/adr/"$PACK_ADR"-*.md); cx_f=$(ls references/adr/"$CODEX_ADR"-*.md)
for t in 0093 0094 0099 0135; do
  grep -qE '^supersedes: \[.*"'"$t"'"' "$cs_f" || { echo "FAIL: the ChangeSummary record does not declare it supersedes $t in part"; exit 1; }
  grep -qE '^superseded_by: \[.*"'"$CS_ADR"'"' references/adr/"$t"-*.md || { echo "FAIL: ADR-$t carries no back-link to $CS_ADR"; exit 1; }
done
grep -qE '^superseded_by: \["0115", "0127", "'"$CS_ADR"'"\]' references/adr/0099-*.md || { echo "FAIL: ADR-0099's back-link is not an APPEND to [0115, 0127]"; exit 1; }
grep -qE '^supersedes: \[.*"0114"' "$pk_f" && grep -qE '^superseded_by: \[.*"'"$PACK_ADR"'"' references/adr/0114-*.md || { echo "FAIL: the ADR-0114 supersession or its back-link is missing"; exit 1; }
grep -qE '^supersedes: \[.*"0094"' "$cx_f" && grep -qE '^superseded_by: \[.*"'"$CODEX_ADR"'"' references/adr/0094-*.md || { echo "FAIL: the ADR-0094 d2 supersession or its back-link is missing"; exit 1; }
grep -qF 'D17-9' "$cs_f" || { echo "FAIL: the ChangeSummary record does not root the conflict in D17-9 by name"; exit 1; }
set -- references/adr/[0-9][0-9][0-9][0-9]-*.md
n=$#; last=$(basename "${!n}"); last=${last%%-*}; next=$(printf '%04d' $((10#$last + 1)))
[ "$((10#$last))" -eq "$n" ] || { echo "FAIL: $n records but the last id is $last — the set is not contiguous"; exit 1; }
grep -qF "next free number is **\`$next\`**" references/adr/README.md && grep -qF "next free number is **\`$next\`**" runbook/00-preamble.md || { echo "FAIL: README or the preamble does not name $next as the next free number"; exit 1; }

# ---- (4) ADR-0085 OPTION HEADINGS FROM 0164 ON, and the lint can go red: a mutated copy, and an empty dir.
out=$(python runbook/tools/check-adr-options.py 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: check-adr-options.py rc=$rc"; printf '%s\n' "$out"; exit 1; }
k=$(printf '%s\n' "$out" | sed -nE 's/^scanned ([0-9]+) records.*flagged 0$/\1/p')
[ -n "$k" ] && [ "$k" -ge 4 ] || { echo "FAIL: the option-heading lint scanned '${k:-none}' records (0164 plus this stage's three is 4)"; exit 1; }
rm -rf target/s8a-ctl && mkdir -p target/s8a-ctl/good target/s8a-ctl/bad target/s8a-ctl/empty || { echo "FAIL: cannot make the control dirs"; exit 1; }
cp references/adr/0164-*.md target/s8a-ctl/good/ && cp references/adr/0164-*.md target/s8a-ctl/bad/ || { echo "FAIL: cannot copy ADR-0164 for the controls"; exit 1; }
sed -i 's/^### Option — /### Option: /' target/s8a-ctl/bad/0164-*.md || { echo "FAIL: the control mutation did not run"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/good 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the lint rejects an unmutated ADR-0164 (rc=$rc)"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/bad 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the lint admitted '### Option: ' (rc=$rc) — it cannot go red"; exit 1; }
out=$(python runbook/tools/check-adr-options.py --dir target/s8a-ctl/empty 2>&1); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: the lint passed over ZERO records (rc=$rc)"; exit 1; }

# ---- (5) THE TWO ERRATA, and the thirty bodies byte-unchanged since the stage opened.
sec=$(awk '/^## ADR-0085 /{f=1;next} /^## ADR-/{f=0} f' references/adr/ERRATA.md)
printf '%s\n' "$sec" | grep -q 'ADR-0132' && printf '%s\n' "$sec" | grep -q 'ADR-0161' || { echo "FAIL: no ADR-0085 erratum lists ADR-0132 through ADR-0161"; exit 1; }
grep -q 'E-0094-2' references/adr/ERRATA.md || { echo "FAIL: E-0094-2 (E12 taken without ADR-0007's phase sentence) is absent"; exit 1; }
set -- references/adr/013[2-9]-*.md references/adr/01[45][0-9]-*.md references/adr/016[01]-*.md
[ "$#" -eq 30 ] || { echo "FAIL: $# files in 0132..0161, expected 30"; exit 1; }
git diff --quiet "$S8A_BASE" -- "$@"; rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: an accepted body in 0132..0161 differs from S8A_BASE (git diff rc=$rc) — the erratum lists them, it does not rewrite them"; exit 1; }

# ---- (6) CHECKPOINT 5'S NEW BAN (the record's rule), code lines only, strings blanked, NO ::ChangeSummary escape.
CS_BAN='\b(struct|enum|type|trait|union)[[:space:]]+[A-Za-z_]*ChangeSummary\b|\btype[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[^=]*=.*\b[A-Za-z_]*ChangeSummary\b|\bimpl\b.*\bfor[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\bimpl(<[^>]*>)?[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b'
cs_filter() { grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\]|\.)*"/""/g' | grep -E "$CS_BAN"; }
raw=$(grep -rn 'ChangeSummary' crates/ --include='*.rs'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ChangeSummary search found nothing or could not run (rc=$rc) — rk-ports' own enum must be seen"; exit 1; }
hits=$(printf '%s\n' "$raw" | cs_filter)
[ -z "$hits" ] || { echo "FAIL: a ChangeSummary definition, alias or impl outside rk-ports:"; printf '%s\n' "$hits"; exit 1; }
for ctl in 'F|x.rs:1:pub struct ChangeSummary;' 'F|x.rs:2:pub enum TextChangeSummary { A }' 'F|x.rs:3:type Cs = rk_ports::toolchain::ChangeSummary;' 'F|x.rs:4:impl Clone for ChangeSummary {' 'F|x.rs:5:impl rk_ports::toolchain::ChangeSummary {' 'F|x.rs:6:impl<T> From<T> for rk_ports::toolchain::BinaryChangeSummary {' 'F|x.rs:7:    let s = "x"; impl Copy for ChangeSummary {}' 'P|x.rs:8:use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};' 'P|x.rs:9:    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {' 'P|x.rs:10:    ) -> Result<rk_ports::toolchain::ChangeSummary, rk_types::error::StepError> {' 'P|x.rs:11:            Self::ChangeSummary => "change_summary",' 'P|x.rs:12:    ChangeSummary,' 'P|x.rs:13:        Ok(ChangeSummary::Text(text))' 'P|x.rs:14:    // impl Clone for ChangeSummary {}' 'P|x.rs:15:    let k = "impl Clone for ChangeSummary";'; do
  want=${ctl%%|*}; line=${ctl#*|}; got=$(printf '%s\n' "$line" | cs_filter)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: control must FIRE and did not — the ban is vacuous for: $line"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: control must be ADMITTED and fired — the ban blocks what the record licenses: $line"; exit 1; }; fi
done
kraw=$(grep -rn 'ChangeSummary' crates/rk-kernel/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the rk-kernel search could not run (rc=$rc)"; exit 1; }
k_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\]|\.)*"/""/g' | grep -E '\bChangeSummary\b'; }
khits=$(printf '%s\n' "$kraw" | k_filter)
[ -z "$khits" ] || { echo "FAIL: rk-kernel production code names the ChangeSummary type (ADR-0135's surviving forbids):"; printf '%s\n' "$khits"; exit 1; }
kctl=$(printf 'crates/rk-kernel/src/x.rs:1:fn f(s: &str) -> Option<ChangeSummary> { None }\n' | k_filter)
[ -n "$kctl" ] || { echo "FAIL: the rk-kernel ban's control did not fire"; exit 1; }
nenum=$(grep -cE '^pub enum ChangeSummary[[:space:]]*\{' crates/rk-ports/src/toolchain.rs)
[ "$nenum" -eq 1 ] || { echo "FAIL: $nenum 'pub enum ChangeSummary {' in rk-ports, expected 1 — the split did not land"; exit 1; }
nbin=$(grep -cE 'struct BinaryChangeSummary\b' crates/rk-ports/src/toolchain.rs)
[ "$nbin" -eq 1 ] || { echo "FAIL: $nbin sealed Binary payload declarations in rk-ports, expected 1 — the pin below would pass over nothing"; exit 1; }
npub=$(grep -cE 'struct BinaryChangeSummary[[:space:]]*\([[:space:]]*pub[[:space:]]*[^(]' crates/rk-ports/src/toolchain.rs)
[ "$npub" -eq 0 ] || { echo "FAIL: the sealed Binary payload has a pub field — it is constructible"; exit 1; }
graw=$(grep -rnE 'Command::new\([[:space:]]*"git"' crates/rk-adapter-toolchain-rust/src --include='*.rs'); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the git-spawn floor could not run (rc=$rc)"; exit 1; }
g_filter() { grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))'; }
[ -z "$(printf '%s\n' "$graw" | g_filter | grep .)" ] || { echo "FAIL: the Rust adapter spawns git itself — the range is read through rk-git (ADR-0077)"; exit 1; }
[ -n "$(printf 'x.rs:1:    let c = Command::new("git");\n' | g_filter)" ] || { echo "FAIL: the git-spawn floor's control did not fire"; exit 1; }

# ---- (7) THE NAMED TESTS, per crate, each `... ok` and an exact count.
run_named() {  # $1 = package, $2 = exact count, then the module-qualified names
  pkg=$1; want=$2; shift 2
  out=$(cargo test -p "$pkg" --test it -- "$@" --exact 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $pkg's named selection rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  for t in "$@"; do printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $pkg: $t did not report ok — renamed, deleted or not selected"; exit 1; }; done
  printf '%s\n' "$out" | grep -qE "test result: ok\. $want passed; 0 failed" || { echo "FAIL: $pkg's selection did not run exactly $want tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
}
run_named rk-ports 3 change_summary::text_variant_serialises_sha_diff_and_manifest change_summary::binary_arm_is_not_constructible_outside_rk_ports fake_behaviour::toolchain_fake_classifies_binary
run_named rk-git 2 diff::range_diff_lists_every_changed_path_with_its_change_kind diff::range_diff_refuses_an_unresolvable_range
run_named rk-adapter-toolchain-rust 4 change_summary::change_summary_text_variant_from_git_diff change_summary::change_summary_refuses_a_range_carrying_a_binary_path change_summary::change_summary_refuses_an_empty_range registry::ac489_capability_registry_dated
run_named rk-adapter-toolchain-node 1 registry::ac489_capability_registry_dated
run_named rk-kernel 4 conformance::conformance_every_step_kind_has_an_executor conformance::conformance_every_port_has_a_call_site probe_differential::grammar_agrees_with_the_s_grammar_probe on_false::on_false_closed_set
run_named redkiln-migrate 1 output::ac475_zero_change_summary_values_in_output
out=$(cargo test -p rk-kernel --test it -- conformance::conformance_every_trailer_key_has_a_producer --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the armed trailer lint's selection rc=$rc"; exit 1; }
printf '%s\n' "$out" | grep -qF 'test conformance::conformance_every_trailer_key_has_a_producer ... ignored, ARMED: Slice-Verdict has 0 producers' || { echo "FAIL: the trailer lint is not ARMED naming Slice-Verdict"; exit 1; }
printf '%s\n' "$out" | grep -qF 'discharged by s8e' || { echo "FAIL: the armed reason still names a stage that did not discharge it (ADR-0160)"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 0 passed; 0 failed; 1 ignored' || { echo "FAIL: the armed selection did not report exactly one ignored"; exit 1; }

# ---- (8) catalog-b REPAIRED, and the probe's three readings at rc 0; the committed JSON is what the probe writes.
grep -qF "kind in [incident, 'security-vuln']" spikes/p9/catalog-b/blameless-postmortem.toml || { echo "FAIL: security-vuln is not quoted"; exit 1; }
braw=$(grep -nE '^on_false[[:space:]]*=[[:space:]]*"' spikes/p9/catalog-b/*.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the bare on_false search could not run (rc=$rc)"; exit 1; }
b_filter() { grep -vE '"(continue|fail)"[[:space:]]*$'; }
[ -z "$(printf '%s\n' "$braw" | b_filter | grep .)" ] || { echo "FAIL: a bare step-name on_false remains in catalog-b"; exit 1; }
[ -n "$(printf 'x.toml:1:on_false = "draft"\n' | b_filter)" ] || { echo "FAIL: the bare on_false control did not fire"; exit 1; }
for want in 'on_false = { to = "draft", max = 2 }' 'on_false = { to = "draft", max = 1 }'; do grep -qxF "$want" spikes/p9/catalog-b/blameless-postmortem.toml || { echo "FAIL: blameless-postmortem lacks: $want"; exit 1; }; done
grep -qxF 'on_false = { to = "assemble", max = 2 }' spikes/p9/catalog-b/api-contract-spec.toml || { echo "FAIL: api-contract-spec's loop-back is not { to, max = 2 }"; exit 1; }
mkdir -p target/s8a-whole && git cat-file blob :spikes/p9/results-s-grammar.json > target/s8a-whole/committed.json || { echo "FAIL: cannot read the staged results-s-grammar.json — run this block after git add"; exit 1; }
out=$(PYTHONIOENCODING=utf-8 python spikes/p9/grammar-probe.py 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: grammar-probe rc=$rc — a clean corpus exits 0"; printf '%s\n' "$out" | grep -E 'FAIL'; exit 1; }
printf '%s\n' "$out" | grep -qxF 'predicates parsed 279/279; failed 0/279' || { echo "FAIL: the parse count is not 279/279"; exit 1; }
printf '%s\n' "$out" | grep -qxF 'mixed unparenthesised and/or (precedence-dependent) 0/279' || { echo "FAIL: the mixed-connective count is not 0/279"; exit 1; }
printf '%s\n' "$out" | grep -qE '^on_false forms 99/99 in the closed set' || { echo "FAIL: the on_false count is not 99/99"; exit 1; }
python -c "import json,sys; a=json.load(open(sys.argv[1],encoding='utf-8')); b=json.load(open(sys.argv[2],encoding='utf-8')); sys.exit(0 if a==b and a['parsed']==279 and a['failed']==[] and a['on_false_ok']==99 else 1)" target/s8a-whole/committed.json spikes/p9/results-s-grammar.json || { echo "FAIL: the probe's JSON differs from the committed one, or does not read 279 / [] / 99"; exit 1; }

# ---- (9) THE CODEX PHASE: the design §10.4 row, and every register site that says Phase 2 carries a dated correction.
row=$(grep -F '| **S-CODEX — harness portability** |' references/2026-09-08-architecture-design.md); rc=$?
[ "$rc" -eq 0 ] && printf '%s\n' "$row" | grep -qF "ADR-$CODEX_ADR" || { echo "FAIL: design §10.4's S-CODEX row carries no dated correction citing ADR-$CODEX_ADR — it already says 'Phase 4' in an older note, so that word proves nothing"; exit 1; }
craw=$(grep -nE 'Codex[^|]*\bPhase 2\b' references/2026-09-08-acceptance-criteria.md references/2026-09-08-success-measures.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the Codex-phase register sweep could not run (rc=$rc)"; exit 1; }
c_filter() { grep -vF "ADR-$CODEX_ADR"; }
[ -z "$(printf '%s\n' "$craw" | c_filter | grep .)" ] || { echo "FAIL: a register line still says Codex is Phase 2 with no dated correction citing ADR-$CODEX_ADR"; printf '%s\n' "$craw" | c_filter; exit 1; }
[ -n "$(printf 'x.md:1:the Codex adapter is Phase 2\n' | c_filter)" ] || { echo "FAIL: the Codex sweep's control did not fire"; exit 1; }

# ---- (10) FMT, CLIPPY, AND THE WHOLE SUITE above its floor (740 measured at ca8f387 plus this stage's 7 new).
out=$(cargo fmt --all --check 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo fmt --check rc=$rc"; printf '%s\n' "$out" | head -20; exit 1; }
out=$(cargo clippy --workspace --all-targets 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: clippy rc=$rc"; printf '%s\n' "$out" | tail -20; exit 1; }
out=$(cargo test --workspace --no-fail-fast 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the whole suite rc=$rc (a fresh checkout needs pnpm install for the node fixture first)"; printf '%s\n' "$out" | grep -E '^test .* FAILED|panicked' | head -20; exit 1; }
total=$(printf '%s\n' "$out" | awk '/^test result:/{for(i=1;i<NF;i++) if($(i+1) ~ /^passed;?$/) s+=$i} END{print s+0}')
[ "$total" -ge 747 ] || { echo "FAIL: the whole suite passed $total tests, below the floor of 747"; exit 1; }
printf '%s\n' "$out" | grep -E '^test result:' | grep -vqE ' 0 failed;' && { echo "FAIL: a result line reports failures"; exit 1; }

# ---- (11) THE ENTRY GATE AND FOUR OTHER CHECKPOINTS, WHOLE, FROM THEIR OWN BYTES. The helper copies the ONE bash
# ---- block that carries an anchor and appends a sentinel, so "rc 0 and the sentinel" means the block ran to its end.
extract() {
  python - "$1" "$2" "$3" <<'PY'
import re, sys
fence = chr(96) * 3
pat = re.compile(re.escape(fence) + "bash\n(.*?)" + re.escape(fence), re.S)
text = open(sys.argv[1], encoding="utf-8").read()
found = [m.group(1) for m in pat.finditer(text) if sys.argv[2] in m.group(1)]
if len(found) != 1:
    sys.exit("FAIL: %d bash blocks in %s carry the anchor, expected exactly 1" % (len(found), sys.argv[1]))
open(sys.argv[3], "w", encoding="utf-8", newline="\n").write(found[0] + "\necho CP8A-WHOLE-RUN-SENTINEL\n")
PY
}
whole() {  # $1 = label, $2 = stage file, $3 = anchor
  extract "$2" "$3" "target/s8a-whole/$1.sh" || { echo "FAIL: could not extract Checkpoint $1"; exit 1; }
  out=$(bash "target/s8a-whole/$1.sh" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: Checkpoint $1 run whole rc=$rc"; printf '%s\n' "$out" | tail -25; exit 1; }
  printf '%s\n' "$out" | grep -qxF 'CP8A-WHOLE-RUN-SENTINEL' || { echo "FAIL: Checkpoint $1 did not reach its last line"; exit 1; }
}
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
cp "$store" target/s8a-whole/store.before.sqlite3 || { echo "FAIL: cannot back up the store before the whole runs"; exit 1; }
whole 7i runbook/phase-1/s7i-cost.md 'CP7I-END-OF-BLOCK reached'
whole 5 runbook/phase-1/s5-workspace-ci-rk-types.md 'FAIL: deny.toml absent'
whole 7a runbook/phase-1/s7a-ports-and-fakes.md 'FAIL: cargo test -p rk-ports exited'
whole 7b runbook/phase-1/s7b-kernel.md 'spikes/p9/grammar-probe.py absent'
# 7f runs `redkiln run` on the fix item. It must be unable to dispatch: every shipped card lacks permission_mode, so
# lint_card refuses before any spawn. From s8h on that is false, and this line must be re-scoped before it runs.
set -- .redkiln/cards/*.json
[ "$#" -eq 8 ] && [ -f "$1" ] || { echo "FAIL: expected 8 shipped cards"; exit 1; }
pm=$(grep -l 'permission_mode' "$@"); rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL: a card carries permission_mode (rc=$rc) — Checkpoint 7f's run could reach a REAL dispatch; stop (D21-40 decision 2)"; exit 1; }
whole 7f runbook/phase-1/s7f-conformance-and-records.md 'CP7F-END-OF-BLOCK reached'

echo "CP8A-END-OF-BLOCK reached"
```

**Closing.** The stage closes in two commits: the work plus its checkpoint output, then the tracker. That second
commit updates `PROGRESS.md`:

- rows 5, 7b and 7f move to their passed hashes;
- row 7a is re-run;
- row 7c is marked provisional (§8a.8);
- a new `s8a` row is added.

Adversarial refuting rounds run over the diff until a round moves no code and no guard.

**Findings for the lead, recorded rather than absorbed:**

1. D21-42 decision 2 names ADR-0099 cl.5 and ADR-0093 d4. **ADR-0094 decision 1's** *"remaining an opaque
   non-finalised newtype until S-BIN"* is also a `## Decision` sentence that the split moves, and so are **ADR-0135
   decision 4 and its third forbids bullet**, which forbid editing the adapter's `change_summary` row to succeed.
   §8a.2 supersedes all of them in part.
2. **`s8`'s bullet and guard go red at this stage's commit.** They read *"No finalised `ChangeSummary` type"* and
   check `pub struct ChangeSummary(`. The plan assigns that edit to no stage, so §8a.3 takes it. Checkpoint 7a's
   declaration line breaks the same way.
3. **Packet (i) A(3) says Checkpoint 7f takes no checkpoint edit.** This stage's brief asks for a re-scope, and
   D21-40 decision 4 requires 7f to be repaired. §8a.5 does both. `s8e` must move the literal again. ADR-0134
   decision clause 5 says, in its `## Decision`, that 7f *"becomes non-re-runnable at that commit"*. This file reads
   that sentence as being about the old text, so the re-scope does not supersede it. Confirm that reading, or route
   the re-scope as a supersession in part.
4. **The design's phase-bearing S-CODEX row is §10.4's, not §10.5's**, and there are six register sites, not two.
5. **Checkpoint 7c becomes provisional here**, and its re-run needs a paid probe. The plan does not list it under
   `s8j`.
6. **The kernel passes `change_summary("")` today.** A Text body that accepts an empty range would be a vacuous
   green over eleven shipped steps. The empty-range refusal is named as a test.
7. **The stage plan's "via git diff" must not be read as a shell-out from the adapter.** The adapter's own manifest
   comment confines git to `rk-git`.

---
