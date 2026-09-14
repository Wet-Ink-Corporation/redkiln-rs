# Step 7b — `rk-kernel` (Phase 1, part 3)

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s7b` | Phase 1 | Checkpoint 7a + 🛑 STOP banner (ADR-0090 exists; 0079 superseded) | Checkpoint 7b | `s7a` | `s7b-ii` |

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§7b.N` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

**The gates are a separate stage file.** Everything this file used to carry in one seven-line paragraph — the human
gate, the review classifier and policy table, re-execution, quarantine, the sink, replay determinism and the verify
harness — is `runbook/phase-1/s7b-ii-gates.md`, with its own acceptance set and its own **Checkpoint 7b-ii**. That is
**120 criteria in force** (the register's three gate areas), and carrying them behind six assertions in this file was
how criteria get claimed on greens nothing pins. Step 7b-ii's entry gate is Checkpoint 7b; Step 7c's entry gate is
Checkpoint 7b-ii.

**Line numbers into the AC register are not used in this file.** The register was under concurrent amendment on
2026-09-13 and its line numbers moved **+39** during this stage's repair — the row `**AC-244**` read at :3689 in the
audit dossier and at :3728 an hour later [measured, both against a working tree `M` from `60f17c6`]. ADR-0119's own
`source:` key records the same hazard and takes the same route. **Every register citation here is by id and named
clause**, which is stable; where a line reference would be load-bearing, the clause is quoted.

**And every `00-preamble.md` line reference in this file and in `s7b-ii` was re-measured on 2026-09-13, because that
file moved too.** The ADR-0119 paragraph added two lines above the golden constraints, so every anchor below the
`Inputs` block shifted **+2**: *"Where a stage is tempted to decide something else, it stops"* is now **:55** (it was
:53), the **Acceptance sets** bullet **:229-231** (was :227-229), *"A checkpoint grep is not this"* **:186-189**,
*"a filter that matches nothing exits 0"* **:178**, the two ADR-0019 figures **:172-173**, and *"A checkpoint that
changed after it passed is provisional"* **:232-237**. `s7a` and the audit dossier carry the pre-shift numbers and are
not edited here — `s7a`:290 names this hazard in general terms (*"a record citing a line number in a file a later
stage may rewrite"*), and **correcting the stale citations in the closed files and in `PROGRESS.md` is owed to the
operator**, not taken here.

**And this pass moved three files that closed files cite, so the OWED list is enumerated rather than left as a
category.** Every item below was measured on 2026-09-13; none of the files named is edited here.

- **`INDEX.md` grew from 147 to 152 lines** — four in the execution-order paragraph and one new stage row — so every
  anchor inside its stage table shifted **+4** and everything below it **+5**. The seven inbound line anchors are all
  in `s7a`: **:278, :280, :281, :1226, :1228, :1229**. Old :58 (Step 5 row) → new :62; old :60 (Step 7a row) → :64;
  old :61 (Step 7b row) → :65; old :91 (*"names seven this runbook executes"*) → :96. **Owed: re-point all seven.**
- **This file grew from 264 to over 1,200 lines**, so three live citations of `s7b`:113 — **`s4a`:244,
  `PROGRESS.md`:25 and `PROGRESS.md`:290**, each for the *"Checkpoint 7b runs it"* claim — now land on unrelated
  prose, and the claim itself is **struck** at §7b.8. **Owed: re-point all three at `s7b` §7b.8**, which is a `§`
  anchor `check-refs.py` can validate where a line number is not. The same rewrite invalidates the AC register's own
  cite of `runbook/phase-1/s7b-kernel.md`:238 in the AC-006 golden-log amendment; **that one IS corrected here**,
  because the register is inside this pass's editable set (it now reads §7b.2(4)).
- **`s7a`:1225-1231 is a dated `[measured]` claim this pass has half-discharged**: *"Checkpoint 7a is the only exit
  checkpoint in the chain that is no stage's entry gate"*, with three named edits owed. **Two landed** — `INDEX.md`:65
  and this file's header row both now read *"Checkpoint 7a + 🛑 STOP banner"* — and the third, `PROGRESS.md`:31, did
  not. **Owed: annotate `s7a`:1225-1231 with the partial discharge and the surviving half**, so the sentence stops
  presenting as a live measurement that is false.
- **Two more `s7a` statements are orphaned by this rewrite.** `s7a`:614 asserts *"Checkpoint 7b already runs
  `cargo test -p rk-kernel -- trybuild`"* — it no longer does; the bare filter was replaced by the harness's own
  name. And `s7a`:1203's hand-forward routes **AC-455** to Step 7b, while AC-455 is claimed at **7b-ii**. **Owed:
  both.**
- **`PROGRESS.md` §1's stage table is the fourth surface of the gate chain and did not move.** :31 is one row for
  two stages (*"7b — `rk-kernel` (with 7b-ii, the gates)"*) with entry gate `🛑 STOP banner` alone, and :32 gives
  Step 7c's entry gate as `Checkpoint 7b`; M3 now requires **Checkpoint 7b-ii** (`INDEX.md`:80) and no row can record
  it. **Owed, and exactly three edits:** split :31 into a `7b` row and a `7b-ii` row; set 7b's entry gate to
  *"Checkpoint 7a + 🛑 STOP banner"*; set :32's Step 7c entry gate to *"Checkpoint 7b-ii"*. `PROGRESS.md` is the
  operator's file and is not edited in this pass.
- **`PROGRESS.md`:156 records PIA-035 as `done` for stages whose files carried none of it.** Its own statement names
  *"Checkpoints 3c, 5, 6 and **7b/8**"* while `s7b-kernel` and `s8-exit-criterion` are absent from its landed-in
  column; the BEFORE readings in §7b.14 are the first this file has carried, and `s8` still carries none. **Owed:
  add this file to that row's landed-in column, or restate the disposition as *done in part (s3, s5, s6); s8 owed*.**

---


> # 🛑 STOP. THIS IS THE STEP STEP 4 EXISTS TO GATE.
>
> **`rk-kernel` is the crate that contains the selection engine.** Everything before this point could be built
> with the gate wide open and no harm done. This step cannot. Run the block; the first two checks stop you,
> the third is a note:
>
> ```bash
> cd D:/repos/redkiln-rs
> ( ls references/adr/0090-*.md >/dev/null 2>&1 \
>     || { echo "STOP: no ADR-0090. Go to Step 4. Do not write selection code."; exit 1; }
>   echo "OK: ADR-0090 exists — the context-vector decision was TAKEN"
>   n=$(grep -c '^status: superseded$' references/adr/0079-*.md)
>   test "${n:-0}" -eq 1 \
>     || { echo "STOP: ADR-0079 is not 'status: superseded' (got ${n:-<unreadable>}) — the gate token is incomplete"; exit 1; }
>   echo "OK: ADR-0079 is discharged, not amended"
>   ls references/adr/0089-*.md >/dev/null 2>&1 \
>     && echo "note: ADR-0089 is the FACET-PROVENANCE record. It is NOT this gate's token." )
> ```
> The block is one subshell, so a pasted STOP ends the gate check and not your interactive shell; its
> status is `$?`. The first two lines **assert and exit non-zero** — until 2026-09-12 all three only
> echoed, so the gate that exists to stop selection work could not stop anything (Halt 8 shape (f)).
>
> **`0089` is not the token, and mistaking it for one is the specific failure this third line exists to stop.**
> ADR-0089 is accepted and it is about facet *provenance* — it partially supersedes ADR-0030, and it explicitly
> does not touch ADR-0079 [documented, ADR-0089]. Seeing it on disk tells you nothing about whether the
> context-vector decision was taken.
>
> **A discharged gate is not an unblocked one.** ADR-0090 records *human-sourced ordinal axes* **and human-sourced
> booleans** for Phase 1 — clause A fails 4/4 on the records that exist, and DECISIONS-2026-09-11 #2 pre-registered
> `has_repro`, `external_dep` and `market_facing` to the same destination. So the thing you may now build is the
> engine's **semantics**; the thing you still may not build is any `require` row keyed on an LLM-emitted
> `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`, `external_dep` or `has_repro` — and after Step
> 4a **no facet is `source = "llm"`**, so the guard in Checkpoint 7b has no true instance today. `kind` is out of
> scope of the block (human input, ADR-0030 rule 4). That is golden constraint 1, **ADR-0090 clauses 1 and 2 — the
> live record, which supersedes ADR-0079 *in full* per ADR-0090 decision 3, exactly as this banner's own second line
> asserts** — AC-071 and design §10.2's "may not" column, all saying the same sentence. *(This read "golden
> constraint 1, ADR-0079, AC-071 …" until 2026-09-13, citing as current authority the record its own banner proves
> superseded. Two sources above this file still call ADR-0079 `proposed` — AC-071's THEN and `00-preamble.md`'s
> golden constraint 1 — and an erratum is owed there; the prohibition itself is unchanged in substance.)*
>
> **F39 moved exactly one of AC-071's five clauses.** Arm D was scored on 2026-09-09, discharging the fourth —
> "arm-D panel `scored`" — and only that one [measured, F39]. The block holds on the **first**: modal required-set
> identity measured **0.25 / 0.15 / 0.10 / 0.10** against the 0.80 line [measured, F36]. **A discharged clause is
> not a partial unblocking, and the five-case fixture set stays at five.**
>
> The enforcement is `redkiln lint --facet-stability` (AC-071), its sibling **`--facet-provenance` (AC-520)**, which
> gates `source = "derived"` and which nothing gated before F39, and the registry loader **AC-072** underneath both.
> **Build all three before the rows they police**, so there is never a window in which a `require` row exists and
> nothing checks it. They are **three rules with three fixture sets** and the register forbids collapsing AC-072 into
> AC-520 (§7b.7).

### 7b.1 What this step builds, and what it may not decide

Workflow/activity split, policy, memo, budgets, retry, replay, and the selection engine's seven-step pure function —
plus the three lints that police the registry the engine reads. **`tokio`, `async-std`, `futures` and `async-trait`
are forbidden here** — no dependency edge, direct or transitive, on any `rk-adapter-*` (AC-484), and no generic
parameter bound by a port trait (AC-480). **The gates are `s7b-ii`'s.**

**Five things this crate must get right, each because a measurement says so:**

- **The FSM has never been replayed.** F19 proved suspension lossless 174/174 across a real process boundary but
  explicitly left "same next action" unproven. The kernel replay test is the *first* time it is replayed at all.
  That test is Phase-1 exit criterion (2), not a nice-to-have. Its full instrument is §7b.11's, and it is three
  assertions and not one.
- **Determinism ban list fails the build** — the `disallowed-methods` / `disallowed-types` tables in the root
  `clippy.toml` (`s5` §5.1 authors them), applied to `rk-kernel::workflow` under a module-level
  `#[deny(clippy::disallowed_methods)]`, with CI running `cargo clippy --workspace --all-targets` **gated on its
  exit code**, plus the must-fail **fixture crate** that reintroduces one banned call and is checked under the same
  invocation (AC-001–AC-004). **Not `-D warnings`, and not a `trybuild` fixture** — AC-001's Verified-by was amended
  on 2026-09-11 (PIA-030/PIA-031) to withdraw both: `-D warnings` promotes `clippy::nursery` to the group deny
  ADR-0083:106 forbids and **ADR-0102 restates on its own source** (ADR-0102:99, *"Forbids `nursery` at any level but
  `warn`"*; its decision 4 sets `nursery = { level = "warn", priority = -1 }`), and *"a `trybuild` fixture cannot fail
  on a lint, only on a compile error, so it is not the instrument here"*. **ADR-0102 supersedes ADR-0083 in part and
  is the live lint-policy record for this crate** — AC-001's Verified-by cites both — so every in-source exception in
  `rk-kernel` is written `#[expect(clippy::<lint>, reason = "…")]` and never `#[allow]` (ADR-0102 decision 3).
  §7b.5 says what is new here and what was discharged at Step 5.
- **The `--stay`-class error is unrepresentable** — a `trybuild` must-fail fixture, not a runtime assertion, and
  Phase-1 exit criterion (5) (design §10.2's Phase-1 row) gates on it. **It has no AC id and no ADR clause**: the only
  record is ADR-0001:42 (*"Every `advance`, gate, retry and budget becomes code, making the `--stay` class of
  corruption unrepresentable"*), which the register itself classes as *"a premise, not a checkable behaviour"*. So
  this bullet is **documented reasoning, not one of the four measured ones** around it. §7b.4 gives the fixture its
  own path and name so Step 8's filter cannot be satisfied by a different stage's fixture, and the missing id is
  recorded as a residual against Step 8 check (5). AC-294's block constructor is the type-level ban that *does* have
  a criterion and a record, and it is `s7b-ii`'s.
- **Selection semantics only.** The seven-step pure function, rank, tie-is-an-error, unsatisfiable-is-a-typed-gate,
  deviation-never-edits-`R`, and the lint set all get built (§7b.9). What does not: any `require` row keyed on an
  LLM-emitted ordinal. `redkiln lint --facet-stability` (AC-071) must **fail closed** on `source = "llm"` without a
  scored stability record, and it must check **all five clauses** — identity, n, draws, arm-D panel scored, corpus
  pinned. A lint that reads identity and skips the panel clause unblocks the entire provisional apparatus silently;
  the five-case fixture set plus the no-record case is the only defence (§7b.7).
- **Its sibling gate, and the one that did not exist before F39.** `redkiln lint --facet-provenance` (**AC-520**,
  Step 4a.4; **THEN amended 2026-09-13**) must **fail closed** on `source = "derived"` unless the facet declares
  **all three** of a named `deriver`, a `deriver_class` of `lexical` or `structural` — declared **by name, never
  inferred from omission** — and a `deriver_record` naming the scored precision record, resolved corpus-relative
  through `CORPUS_PIN` and never through a working tree (AC-464). **AC-071 gates `llm` and nothing gated `derived`**
  — so the semantic facets sat outside the block and ran **ungated while measurably wrong**, at `auth` precision 0.00
  (0/11 [0.00, 0.26] on the 2026-09-09 scoring; 0/12 [0.00, 0.24] after the 2026-09-11 panel-file key fix scored all
  140 cells — F39-DRAFT's Correction; ERRATA E-0089-2) [measured, F39].

**What this step may not decide** (`00-preamble.md`:55 — *"Where a stage is tempted to decide something else, it
stops."*). Four things it used to leave open were decided **in records** on 2026-09-13, before any kernel code, and
are executed here rather than re-opened — they are §7b.2. Everything else that is genuinely open is **raised and not
settled**, with the record that is owed named, in §7b.15.

### 7b.2 The four Slice-0 records, and the one act this session does not perform

**All four are on disk. Read them before citing them.**

1. **`dag_amended` was not a kind, and a record — not this stage — mints one.** AC-064's THEN requires a
   `dag_amended { deviation_id, added, removed, rows_refired }` event. `git grep 'dag_amended\|DagAmended'` over the
   whole tree returns **exactly two hits**: AC-064's THEN and SM-017's *Instrument* line [measured 2026-09-13]. It is
   absent from the design's event-kind table and from `crates/rk-types/src/event_kind.rs`, whose `ALL` is `[Self; 28]`.
   **ADR-0116 is the precedent and it forbids the shortcut by name**: *"the kind is minted by record or not at all"*,
   and its rejected option is *"mint the kind in the runbook"*. **The Slice-0 pass STOPPED rather than minting it**,
   correctly, and recorded two free residuals a stage may not settle — the **tier** (ADR-0109 decision 2's
   deleted-database test; `dag_assembled` is the nearest sibling at tier 2, *"rebuildable by replay"*) and the
   **producing step** (AC-064's evidence line says *"re-run 2–5"*, so the producer is the re-run, not step 5 as first
   fired). **An ADR is OWED before AC-064 is built, and it is Slice 3's blocker, not this session's.** One correction
   the next attempt needs: adding a variant does **not** move `rk-types`' golden snapshot —
   `crates/rk-types/tests/it/schema_snapshot.rs` snapshots `registry::export_all()` plus `CompanionKind::ALL`, and
   `EventKind` is in neither; what moves is the hand-written pin at `crates/rk-types/tests/it/event_kinds.rs`:9
   (`assert_eq!(EventKind::ALL.len(), 28)`) and `event_kind.rs`:81's `[Self; 28]`, plus the `wire_name` and `tier`
   exhaustive matches, which give the `E0004` that forces the tier decision [measured 2026-09-13].
2. **AC-045's key is a fourth key — `ADR-0119`, accepted 2026-09-13.**
   `references/adr/0119-the-selection-memo-key-is-a-fourth-key-and-adr-0097-gains-no-ninth-component.md`:
   `SelectionMemoKey` is SHA-256 over a length-prefixed tuple of the pack digest, the catalog digest, the context
   digest, the artifact digests sorted and folded, and the node's identity in `dag_assembled.nodes`; `inputs_digest`
   (AC-040) is the fold of the first four on the same type. **The key is derived in `rk-kernel::select` and the lookup
   happens OUTSIDE `select()`** — a memo read inside the pure function is the hidden input AC-041's THEN forbids.
   **ADR-0097's step memo key gains no ninth component and is untouched** (`supersedes: []`, and ADR-0097 carries no
   new `superseded_by`); `rk-store`'s eight-field `StepKeyComponents` does not move. **No shared key helper with the
   other three.** The record explicitly does **not** decide AC-043's third state or the memoised value's type, and it
   names three register amendments as owed — AC-045 (name the key; until then **AC-045 reports armed, not
   discharged**), AC-043 (its third state), and design §4.4's *"The memo key"* → *the selection memo key*.
3. **AC-520's fixture count is stale in the register, and `ERRATA.md` now says so — `E-0520-1`, 2026-09-13.** AC-520's
   *Verified by* opens *"six must-fail fixture `facets.toml` files, one clause at a time"* and then enumerates **eight
   clause groups over ten files**, because the 2026-09-13 amendment inserted `deriver_class` and `deriver_record` and
   did not move the count. The erratum's rule is the one this file follows: **read the enumeration, not the number.**
   §7b.7 takes the fixture set from the register's enumeration by reference and states no tally of its own.
4. **Golden-log ownership is settled in the register, not in the runbook.** Three live documents gave three owners;
   the register now carries a dated amendment on **AC-006 and AC-188**: the producer is **`rk-kernel`**, the stage is
   **Step 7b**, in the same slice as the `golden_log_replay` test; the location is `redkiln-corpus` under a
   **`golden-logs/`** directory at the corpus root; the read route is **corpus-relative through `CORPUS_PIN` at
   `CORPUS_SHA` via the object store, never a working tree (AC-464)**; and the per-run file layout inside
   `golden-logs/` is **deliberately left to this stage** (§7b.11). Under ADR-0092 the register outranks the runbook,
   so until that amendment this file was formally overriding a record while being right on the merits.

**And one act this stage owns that this session does not perform.**

> **`CORPUS_PIN` does not move in this session, and it moves exactly once in the whole of Step 7b.**
> `PROGRESS.md`:103 (row L2) fixes it: the pin should move *"once, with 7b's work, not twice"*. The single move must
> carry **three** artifacts together — AC-520's scored precision record, AC-071's stability record, and
> **AC-006/AC-188's golden event logs**. The golden logs are produced by the replay test in **Slice 3**. So the pin
> **cannot** move before Slice 3 is green without forcing a second move, and AC-464 would then freeze the first SHA
> into every test that read it and the second move would re-derive every corpus-bound assertion.
>
> **What Slice 0 does instead is preparatory only, and it is done:** `spikes/p1/score-arm-d-panel.py` now takes
> `--out <path>` and `--results-dir <dir>`, and writes the machine-readable record AC-520's `deriver_record` names.
> Verified by running it: with `--out` the report is byte-identical to the run without it, plus one trailing
> `wrote <path>` line; the record carries the corpus sha **read off the engine records and never hard-coded** (golden
> constraint 8) and **fails closed on a split set** — a positive control re-pinning one row to `0000…` exits 3 with
> *"ABORT: engine records carry 2 distinct corpus_sha value(s)"* and writes nothing.
>
> **The move is a single named act at the END of Step 7b**, after Slice 3's replay test is green and its logs exist,
> and its seven preconditions and its command sequence are §7b.11's. **No agent and no session may write to
> `D:/repos/redkiln-corpus` or edit `CORPUS_PIN` before that act.** Two things it still cannot prove, and they are
> recorded rather than smoothed: nothing at 7b verifies that `lint --facet-provenance` **reads** the record through
> `CORPUS_PIN` rather than an in-tree copy, because `redkiln` is `rk-cli`'s binary and `.redkiln/facets.toml` is Step
> 7e's file; that assertion is **armed, not discharged** at the pin move and the deferral is recorded against 7d/7e.
>
> **And the schedule itself is owed in the tracker, which is the half this session cannot perform.** `PROGRESS.md`
> row L2 says the pin moves once, with 7b's work; it does not yet say **when**. **Owed to the operator: record in
> `PROGRESS.md` §2 against row L2 that the single move is a named act at the END of Step 7b — after Slice 3's replay
> test is green, carrying all three artifacts, with §7b.11's seven preconditions as its gate and Checkpoint 7b run
> after it (§7b.3's slice table, row 4) — and that no agent and no session may write to `D:/repos/redkiln-corpus` or
> edit `CORPUS_PIN` before that act.** `PROGRESS.md` is not edited here.

**AC-071's stability record is a second, larger problem and this step does not choose its shape.** Three things are
measured [2026-09-13]: **(a)** nothing produces the record — its five named keys are split across two instruments
(`modal_required_set_identity` from `spikes/p1/analyse-arm-l.py`, `panel_verdict` from `score-arm-d-panel.py`), no
script performs the join, and `analyse-arm-l.py`'s `--out` writes a **Markdown report** in which
`grep -cE 'modal_required_set_identity|n_items|draws_per_item|panel_verdict|corpus_sha'` returns **0**; **(b)** its
declared path `spikes/results/p1/stability.json` names a directory that has never existed on either side — `results/`
is flat in the corpus; **(c)** 🛑 **re-running the identity instrument today returns a number that would UNBLOCK the
gate.** Frozen at the pin, F36's own output reads `cat:f89154d762a7 fac+art:149a74a0d053 pk:6994b61bc64d`, v1
bug-standard 5/20, v1 story 3/20, v2 both 2/20 — *"KILL fires", 4 of 4*. Re-run today over the same pinned record
files with the same script it reads `cat:fbc5d98ae488 fac+art:592fa138d68d`, **20/20 = 100% on all four**, and *"Kill
at <80%: does not fire"*. The cause is that `analyse-arm-l.py`'s `load_lattice()` reads `spikes/p9/facets.toml`,
`artifacts.toml` and `catalog/` **from the working tree**, and both digests moved when Step 4a rewrote the registry —
`[facet.auth]` no longer exists while `spikes/p9/lattice.py`'s `BOOLS` still supplies it. **Whether that alone
accounts for 20/20 is not isolated and is owed, not asserted.** The consequence stated plainly: a stability record
regenerated by re-running today's instrument would carry `identity = 1.00`, all five clauses would pass, and
`lint --facet-stability` would exit 0 on a number F36 measured at 0.10–0.25. **That is Halt 9's shape and golden
constraint 9's guard-scoped-wrong, one level up: the instrument now measures a different registry than the one it was
pre-registered against.** Therefore, when the record is authored, it is **transcribed from F36's frozen output** — the
four figures 5/20, 3/20, 2/20, 2/20, carried per (instrument, pack) and never averaged into one — together with the
three registry digests it was measured under, and it is **not regenerated by re-running the analyser**. **AC-071 needs
its own dated amendment naming the record's path, its producer and this non-reproducibility; this step does not write
one** (`00-preamble.md`:55) and the `corpus_sha` clause's fixture is authored against the path the amendment names.

### 7b.3 The slices, and where a session is

**Step 7b is six to eight sessions, not one.** Its acceptance set is **115 criteria in this file** plus `s7b-ii`'s
**122** (the register's three gate areas at **120 in force**, plus AC-455 and AC-239's list-is-data half, both
cross-listed), against Step 7a's **24** in a file five times the length.

**The counting rule is stated first, because the same number in no rule is this checkpoint's own recorded defect
(Halt 7), and the previous arithmetic here failed against the very section it said it was counted off.** The rule:
**subtract exactly the ids §7b.15 writes *minus*, *Excluding* or *Excluded* about, and nothing else.** An id §7b.15
labels PROVISIONAL, human-gated-in-part or ARMED-in-part but does **not** subtract stays in the set **with its
label**, because a half claimed here is a claim, and the ARMED table below is where its undischarged half is
reported. Under that one rule, per range, with the same exclusions §7b.15 names:

| range | in force | §7b.15's own subtractions | counted |
| --- | --- | --- | --- |
| AC-001–AC-039 | 39 | − AC-013 (PROVISIONAL on P8, built not claimed), − AC-039 (human-gated) | **37** |
| AC-040–AC-079 | 39 *(AC-075 cut and never defined)* | − AC-044 (armed; §7b.9(8)) | **38** |
| AC-080–AC-118 | 39 *(AC-119 unallocated)* | − AC-092..AC-099 and AC-118 (doctor, 7d's), − AC-101/AC-102 (7e's baseline), − AC-113, − AC-114/AC-115 | **25** |
| AC-520, AC-525, AC-527 | 3 | — *(AC-530 is `s7b-ii`'s)* | **3** |
| AC-161, AC-164 *(next-action halves)* | 2 | — | **2** |
| AC-005 *(`cargo tree -i` half)* | 1 | — | **1** |
| §14 hand-forwards | 10 | − AC-500 (**ARMED, not claimed** — §7b.15 says so in terms) | **9** |

37 + 38 + 25 + 3 + 2 + 1 + 9 = **115**. The nine §14 ids counted are AC-484, AC-487, AC-494, AC-495, AC-498 and the
four part-halves AC-480, AC-483, AC-497, AC-528. **Re-count this off §7b.15 after any change to the set, under the
rule above and not by eye** — three of this block's four previous numbers were written by tallying rather than
counting, which is Halt 7's shape and the reason the rule is written down. Projecting off `rk-git`'s measured
ratio (1,825 src lines / 68 tests over roughly 80 criteria) puts `rk-kernel` at **4,000–6,000 source lines and
200–300 tests**. Anyone attempting it as one pass will discharge the first two slices honestly and claim the rest on
greens nothing pins — the exact failure ADR-0019 was written about (`00-preamble.md`:172-173: **0 of 156** historical
reports carried per-AC mutation evidence; **≥ 12 of 23** shipped, human-approved fixes carried an AC no test pinned).

| slice | what it delivers | ends green on | needs, that does not exist before it |
| --- | --- | --- | --- |
| **0** | the four records of §7b.2, the scorer's `--out`, this repair pass. **No product code.** | the four records on disk; `check-refs.py` and `check-checkpoints.py` at exit 0 | — |
| **1** | the crate skeleton and manifest (§7b.4), the determinism gate (§7b.5), the ADR-0103 grammar parser and evaluator (§7b.6) | the five CI gates, plus the grammar group of Checkpoint 7b | `crates/rk-kernel` itself |
| **2** | the registry loader and the two facet lints as **three** rules with three fixture sets (§7b.7); rigor-monotonicity (§7b.8) | the five CI gates, plus the lint and lattice groups of Checkpoint 7b | Slice 1's parser; the fixture registries of §7b.12 |
| **3** | the seven-step pure selection function (§7b.9) and the golden logs (§7b.11) | the five CI gates, plus the selection and replay groups | Slice 1's parser; the `dag_amended` record of §7b.2(1); the canonical-serialiser layering of §7b.9 |
| **4** | the kernel runtime — workflow/activity split, idempotency, memo, budgets, retry, breaker, degradation (§7b.10) | the single `CORPUS_PIN` move, then **Checkpoint 7b in full** | `rk-ports`' fakes and `rk-store`'s three keys (both exist) |

**The order in row 4 is the pin move FIRST, then the checkpoint, and it is written here because the file gave two
opposite orders and the checkpoint can only pass under one.** §7b.14's block asserts `golden-logs/` non-empty in the
corpus and `results/p1-arm-d-panel-scored.json` present in the corpus; both are true only **after** the move. §7b.11's
seven preconditions are the move's gate and *"Checkpoint 7b has passed"* is deliberately **not** among them — the two
are not a loop, and §7b.11 restates that.

**This session delivered Slice 0 and this repair; Slices 1–2 are its remaining scope** (the operator's decision of
2026-09-13, D-A). **Measured at the time of writing: `crates/` holds `rk-git`, `rk-ports`, `rk-store` and `rk-types`
and no `rk-kernel`** — Slice 1 has not started, and Slice 0's own end state is the one this table's row 0 names (the
four records on disk; the two validators at exit 0). Slices 3–4 are later sessions; `s7b-ii` is the whole of what
used to be called Slice 5 and is now its own stage.

**Checkpoint 7b runs once, at the end of Slice 4.** Each slice ends with the workspace green under the five CI gates
and records that in `PROGRESS.md`; a slice's green is **not** a checkpoint pass and may not be recorded as one. A
`[WILL EXIST]` line would be the wrong instrument for a mid-slice state — the carve-out is for a path a later *stage*
produces, not for one a later *slice of this stage* produces (`halts.md`:87-91).

### 7b.4 The crate skeleton, the manifest and the file layout

**Copying a reference shape's manifest is how a crate lands outside the workspace deny set.** `crates/rk-kernel/Cargo.toml`
carries what `rk-types`, `rk-store`, `rk-git` and `rk-ports` carry:

- `edition` / `rust-version` / `license` / `publish` inherited with `.workspace = true`;
- `[features] corpus = []` — declared even where empty, or a `--features corpus` invocation naming this package exits
  **101** with *"none of the selected packages contains these features"*; the reason is written out at
  `crates/rk-types/Cargo.toml`:10-17, and Checkpoint 6 runs `cargo test --workspace --features corpus`. **This crate
  does have corpus-bound tests** (§7b.11) — and **the feature stays `[]` anyway**: it gates code through
  `#[cfg(feature = "corpus")]`, which needs no feature entry. `crates/rk-git/Cargo.toml` declares `corpus = []` while
  `crates/rk-git/tests/it/corpus_f12.rs`:19 is `#![cfg(feature = "corpus")]` and three tests sit behind it [measured
  2026-09-13], and that is the house form. **`golden_log_replay` and `ac161_ac164_delete_db_replay` read
  `$CORPUS_REPO` and therefore carry that attribute**, because D-N/PIA-002 makes a corpus-bound test *outside* the
  feature a CI failure; Checkpoint 7b runs `--features corpus` **and** the 2-vs-0 differential that proves the
  feature is what gates them, on Checkpoint 6's shape (`s6`:215-220);
- `[dependencies]` — `rk-types`, `rk-ports` and **`rk-git` as a concrete crate** (AC-483's second clause; git is
  deliberately not a port, ADR-0003), plus `serde`/`serde_json` from the workspace table. **No async runtime**, which
  is what AC-528's fixture commit is a commit against, and **no `rk-adapter-*`**, which AC-484 forbids directly or
  transitively. The `rk-store` edge is §7b.9's layering question and is settled there, not here;
- `[dev-dependencies] trybuild.workspace = true`, plus `rk-types` repeated — *"a crate's own `[dependencies]` are not
  in scope inside `tests/`"* (`crates/rk-types/Cargo.toml`:31-32);
- `[[test]] name = "it"` / `path = "tests/it/main.rs"` — **one** integration binary with a `mod` per case, never one
  file per case (ADR-0083:74). This is also what makes `--test it` addressable in the checkpoint;
- `[lints] workspace = true` as the last stanza, **and nothing else** (ADR-0102 decision 2). **This line is
  load-bearing and the mechanism is worth stating exactly**: `clippy.toml` *configures* `disallowed_methods`, it does
  not raise its level — the level comes from the root `Cargo.toml`'s `[workspace.lints.clippy] all = { level =
  "deny", priority = -1 }`, which a member reaches only through this table. `clippy::disallowed_methods` is
  **warn-by-default**; `tests/fixtures/rk-clock-read/Cargo.toml`:21-24 says so in terms — *"it is `clippy::all =
  "deny"` inherited through THIS table that turns the reintroduced clock read into a non-zero exit. Without it the
  fixture would warn and the gate would pass."* Without `[lints] workspace = true` **every lint assertion in
  Checkpoint 7b passes over an unlinted crate.**

**Write the sync rationale and the governing records as a module header.** `crates/rk-types/src/lib.rs`:6-7 and
`crates/rk-store/src/lib.rs`:1-20 are the house pattern. Checkpoint 7b's async guards exclude comment lines by
construction, so a header naming `tokio` in prose will not halt them — the unfiltered form would, which is why they
were narrowed (and why the same guard tripped five Step-6 assertions, `ci.yml`:186-190).

**The file layout is not this step's to decide** (`00-preamble.md`:55). The house layout is one module per concern
with `lib.rs` carrying a doc header and `pub mod` lines — `grep -c '^pub mod' crates/<c>/src/lib.rs` reads **12** for
`rk-types`, **8** for `rk-store` and **11** for `rk-git` [measured 2026-09-13]. **Every assertion in Checkpoint 7b is
therefore recursive over `crates/rk-kernel/src/`, never scoped to `src/lib.rs`.** Two module paths are named because
a criterion names them and nothing else can reach them: **`rk-kernel::workflow`** (AC-001's module-level `#[deny]`)
and **`rk-kernel::policy`** (AC-294's construction-site ban, `s7b-ii`'s).

**One `trybuild` harness, named after its rule and not after the tool.** The house convention is measured:
`crates/rk-git/tests/it/reconciled.rs`:24 is `reconcile_runs_at_every_entry` and
`crates/rk-store/tests/it/tier_ui.rs`:9 is `a_state_field_without_a_tier_is_a_compile_error`, and **neither name
contains the substring `trybuild`** [measured 2026-09-13]. The kernel's harness is
**`trybuild_pins_the_reason_each_kernel_fixture_fails`**, in `crates/rk-kernel/tests/it/ui.rs`, with one
`trybuild::TestCases` over every `crates/rk-kernel/tests/ui/` case, each committing a `.stderr`. **`tests/ui/` is that
glob's root and therefore holds compile-FAIL cases only** — a file that must *compile* (a positive control for a
grep, say) placed there makes `trybuild` report an unexpected success and turns the harness red. Anything of that
kind goes under `crates/rk-kernel/tests/fixtures/lint-controls/`, which is outside the glob (`s7b-ii` §7b-ii.10 adds
that directory and says why). Its Phase-1 cases:

| case file | what must not compile | criterion |
| --- | --- | --- |
| `stay_class_state_is_unconstructible.rs` | the resumed/half-advanced state the `--stay` class depends on | **none** — Phase-1 exit criterion (5) only; the id is a residual (§7b.1, §7b.15) |
| `ac480_generics_leak.rs` | a `<P: Harness>`-shaped bound, or an `impl Trait` return naming a port | AC-480's first THEN clause, handed here by `s7a`:995 |
| `ac050_rigor_is_not_arithmetic.rs` | `Add` / `Sum` / `Into<f64>` on `Rigor` | AC-050 |
| `ac041_selection_reads_no_clock.rs` | a clock, env, filesystem, network or random read reachable from `select()` | AC-041 |
| `ac061_required_set_is_not_constructible_outside_select.rs` | constructing `RequiredSet` outside the selection module, or deserialising one | AC-061 |
| `ac061_required_set_has_no_deserialize.rs` | a `Deserialize` impl on the required set | AC-061's second clause |

**`RequiredSet`'s constructor stays `pub(in crate::select)` and carries no `Deserialize`.** ADR-0099 clause 5 makes
*"inside the selection crate"* mean the module, and AC-058, AC-063 and AC-078 all name **unit** tests — AC-078's DAG
is explicitly *"constructed inside the crate"* — so the `R`-equality assertions live in `src/select/` under
`#[cfg(test)]` rather than in `tests/it/`; say so in the module header. **A `pub(crate)` constructor, or a
`#[cfg(test)]` public one, silently converts AC-061 from a type fact into a convention**, and nothing at this stage
would catch it: nothing in `rk-kernel` is agent-facing until 7c.

**Step 8 filters the same suite, and what it cannot see is stated rather than assumed.** `s8`:121 is
`cargo test -p rk-kernel -- trybuild`. *(This paragraph read "a filter spelled `trybuild` selects **zero** tests
against the house convention" until 2026-09-13. **That is false of this crate's own harness**, whose name contains the
substring: measured on this tree, `cargo test -p rk-ports --test it -- trybuild` selects
`ui::trybuild_pins_the_reason_each_port_fixture_fails` and reports `ok. 1 passed`. The claim is true of `rk-git`'s
`reconcile_runs_at_every_entry` and `rk-store`'s `a_state_field_without_a_tier_is_a_compile_error`, and it was carried
across to the kernel without re-measuring.)* So Step 8 check (5) **does** reach this harness. What it cannot
distinguish is **which cases the harness ran**: it is green whenever any one of them passes, so an absent
`stay_class_state_is_unconstructible.rs` reads exactly like a present one. **Checkpoint 7b closes that here by
asserting all six case files and their six `.stderr` files BY PATH**, and **a matching by-path assertion in `s8`
check (5) is OWED** and is recorded in §7b.15 beside the missing criterion id. The `--stay` case's own filter name
does not fix this and is not claimed to: `stay_class_unrepresentable` is a *fixture* name in §7b.13's red-first list,
not a `cargo test` name, and §7b.13 says so.

### 7b.5 The determinism gate — what Step 5 discharged and what only this stage can

**AC-001–AC-004's `clippy.toml` half and both must-fail fixture crates were discharged at Checkpoint 5** (`96ba9c2`;
`PROGRESS.md`:28) and are **re-run, not re-claimed, here**. `s5` §5.1a authored `tests/fixtures/rk-clock-read/` and
`tests/fixtures/rk-banned-dep/` as feature-gated workspace members (DECISIONS-2026-09-12 E4), and `s5`:170-174 already
asserts the reintroduce fixture turns the workspace clippy run red.

**What is new here, and what Step 5 could not do because the module did not exist, is the module-level
`#[deny(clippy::disallowed_methods)]` on `rk-kernel::workflow`.** `clippy.toml` has **no path or module scoping**
(`clippy.toml`:1-3; `s5` §5.1 says why), so the workspace tables are green with or without the attribute and **no
existing line anywhere in this runbook can detect its absence** — `grep -rn 'deny(clippy::disallowed' runbook/`
returned two hits before this pass, both prose [measured 2026-09-13]. Checkpoint 7b asserts it **anchored on the
declaration**, because a doc comment quoting the attribute satisfies an unanchored grep (`s6`:249-250's house note),
**and pairs it with a run that must go RED** — the reintroduce feature below. The grep shows the attribute is
*declared*; only the red run shows it *bites*, and `clippy.toml` alone does not make it red.

**Do not add a `cargo clippy -p rk-kernel --features rk-clock-read/reintroduce` line.** `rk-clock-read` is a separate
feature-gated workspace member, so `-p rk-kernel` cannot select its feature and the invocation **errors rather than
lints** — the *"failed for some OTHER reason"* defect `s5`:168-169 exists to warn about. The workspace-scoped form is
Step 5's and Step 8 check (4) re-runs it.

**Red first for this gate:** `workflow_clock_read_is_red` — one `SystemTime::now()` inside `rk-kernel::workflow`
behind a **non-default feature of this crate**, `workflow-clock-reintroduce`. **It is committed, not reverted**, on
the `tests/fixtures/rk-clock-read/` precedent (`s5` §5.1a, DECISIONS-2026-09-12 E4): a reverted observation is not
an assertion, and Checkpoint 7b has no other way to show the attribute bites. The invocation is
**workspace-scoped** — `cargo clippy --workspace --all-targets --features rk-kernel/workflow-clock-reintroduce` — and
**must exit non-zero with `disallowed_method` in its output**, both asserted, so an unrelated failure cannot read as
the pass (`s5`:168-174 is the shape). **The module attribute, not `clippy.toml` alone, is what makes it red.**
*(This bullet said "then revert and record the observation" until 2026-09-13, which left §7b.14's `#[deny]` grep as a
bare string search with nothing to pair it with.)*

### 7b.6 The ADR-0103 grammar, its five pack defects, and the evaluator

**The grammar the lint parses is ADR-0103's, and it has been run over every live predicate (F42, S-GRAMMAR,
2026-09-11):** `not` > comparison/`in` > `and` > `or`, left-associative; an unquoted identifier in value position is a
literal (load-bearing at **191 sites** [measured, F42] — `grammar-probe.py`'s `bare_literal` census counts
occurrences, not predicates, so it does not divide by the 287); `null == v` is `null` and connectives are three-valued;
`any` over `null` or an empty list is `false`, `all`/`none` `true`; `on_false` is the closed set `{ to, max } |
"continue" | "fail" | { to = "deviate", … }`.

**Two forms the packs already use are stated in exactly one place, and it is a throwaway probe's docstring.** The
set-membership `in` (`'text' in change`, **11** sites; a string literal on the left, a set-valued facet on the right,
ADR-0050's *"same facet disjoins"*) and hyphens inside `${…}` ref segments are stated **only** in
`spikes/p9/grammar-probe.py`'s docstring EBNF (`:15` and `:20`). Design §4.4 step 2 names set membership as a
predicate *kind* and nothing more, and the design states in terms that the predicate grammar's home is
`references/2026-09-07-activity-graph-schema.md`, whose grammar clause states **neither** form. **Start the parser
from the probe and record that it is one reading by one author** (F42 Limits 1). *(F42's own table says "the design's
grammar section records both"; the design records neither, and a dated correction to F42 is OWED. That is a spike-file
edit and not this stage's.)*

🛑 **But the parser is not where a grammar production first enters a record, and this file may not make it one.**
**ADR-0103 is the grammar's record and it admits neither form.** Its decision 2 puts *"the subject of `in`"* in
**operand position**, where *"an unquoted identifier is a **field**"* — a **quoted string literal** as the subject of
`in` is outside that model — and its forbids clause reads *"Forbids a parser or lint implementing any precedence but
clause 1"* and *"Forbids an unquoted identifier in value position being read as a field"*; hyphens inside a `${…}`
refseg sit against decision 2's `[a-z_][a-z0-9_]*` class and are admitted nowhere in the record. Under ADR-0092 the
ADR outranks this file and outranks F42, so *"the kernel's parser fixing these two forms is where they first enter a
record"* — which this paragraph said until 2026-09-13, with only a spike-file correction owed — was a **stage deciding
a grammar clause an accepted ADR owns**. **What is owed is an amendment or erratum against ADR-0103 adding the
`string in <set-valued facet>` production (ADR-0050's *"same facet disjoins"*, 11 live sites) and the refseg hyphen
(step ids such as `green-verify`), with F42's dated correction beside it. It is NOT written here** (`00-preamble.md`:55).
**Until it lands, both forms are reported as IMPLEMENTED-PENDING-RECORD**: the parser accepts them because 11 live
predicates and every `${…}` step reference depend on them, and the acceptance is a residual against ADR-0103, not a
settled clause. The measurement is unaffected and reproduces: `in_membership: 11`, `bare_literal: 191`,
`predicates parsed 285/287`, `mixed unparenthesised and/or 0/287`, `on_false forms 96/99` [measured, re-run read-only
2026-09-13].

**Five pack defects, not two, and the lint must catch all five** [measured, F42, whose title names both halves;
re-run read-only 2026-09-13 and reproduced]:

- **two parse failures** — `exists(ProblemRecord)` in `packs/incident.toml` (a user function, which ADR-0033 admits
  none of) and a bare hyphenated value in `catalog-b/blameless-postmortem.toml`'s `applicable_if` (which ADR-0048
  requires quoting). `predicates parsed 285/287; failed 2/287`, and `mixed unparenthesised and/or
  (precedence-dependent) 0/287`;
- **three `on_false` values outside ADR-0103 clause 6's closed set** — `catalog-b/api-contract-spec.toml` step[5]
  `'assemble'`, and `catalog-b/blameless-postmortem.toml` steps [2] and [4] `'draft'`. The probe prints
  `on_false forms 96/99 in the closed set: {'keyword': 33, 'loop-back': 66}`.

**Only the first is in a shipped pack** (F42 Limits 3), and **the pack repairs are Step 7e §7e.2's, not this
stage's**; ADR-0114 decision 1 counts exactly this five and repairs them there before the v0 tag. ADR-0103 clause 6
names `spikes/p9/lint.py`'s `on_false_of` as the **reference behaviour** and `{ to = "deviate", … }` as ADR-0046's
form — read both before writing the `on_false` branch. Those three rows are the **only live instances** of the
closed-set rule, so they are its red fixtures.

**Four further clauses the implementer cannot guess, and one this runbook has already got wrong once in prose:**

1. **Comparisons do not chain** — `a < b < c` is a parse error (ADR-0103 decision 1).
2. **Field-to-field comparison requires the `${…}` reference form** (decision 2; ADR-0040).
3. **Exactly one narrowing pattern is recognised** — `x != null and <ordering on x>` at the same conjunction level,
   no `or`, no `not`; any other ordering against a nullable facet is a **load-time lint error** (decision 4). **The
   pattern is syntactic, not semantic** (AC-069, PIA-052), so `not (novelty == unknown) and novelty >= high` and
   `novelty == unknown or novelty >= high` must **both fail** load.
4. **The order of outcomes on a null read.** **(i)** the lint rejects an unguarded ordering at pack load; **(ii)** at
   selection step 2, a `null` top-level `applicable_if` or `require.when` emits `gate_opened { reason: unknown_facet,
   axis }` (AC-079), **counted once per read facet, never per abstaining facet**; **(iii)** `Unsatisfiable(kind,
   failed_predicates)` is **reserved for step 4, over predicates that evaluated to `false`, never for `null`**
   (decision 5). The must-fail fixture the ADR itself names: **`any` over absent evidence must not pass an activity.**
   `PROGRESS.md`:277 records the prior instance — `s4a` routed a `null` to `Unsatisfiable`, which clause 5(iii) and
   AC-079 forbid. **Step 7b builds both the step-2 null path and the step-4 `Unsatisfiable` path, and this clause is
   what keeps them apart.**
5. **`applicable_if` scope is decided once**: evaluated over the item's context vector at selection, **never
   re-evaluated inside the activity** (decision 6).

**The probe parses and does not evaluate.** F42 says so: *"ADR-0103's three-valued connectives and quantifier null
rules are unexercised (0 quantifiers in the corpus). Step 7b's evaluator is the first implementation."* Re-run today,
the feature census carries no quantifier key at all and `null` appears twice [measured 2026-09-13]. So the evaluator's
fixtures are **authored here, not lifted from the corpus**, and §7b.13 names them.

### 7b.7 The registry loader and the two facet lints — three rules, three fixture sets

**AC-072 is a THIRD rule and not a clause of either lint, and the register forbids collapsing them.** Amended
2026-09-13, AC-072 fails load unless a facet entry carries a declared domain, `source`, anchor, **`nullable = true`
where the facet may abstain**, and — where `source = "derived"` — a named **`deriver`**, a **`deriver_class`** of
`lexical` or `structural` and a **`deriver_record`**, *"all three required together; AC-520 owns what the class and
the record MEAN, this criterion owns that the ENTRY carries them — amended 2026-09-13, and the two must not be
collapsed into one lint rule"*. The `nullable` key is DECISIONS-2026-09-12 E1: `null` is not a domain member, the
three domain shapes are unchanged, and a fourth still fails load.

AC-072's *Verified by* is an eleven-case set and its **positive** fixtures are what stop a reject-everything lint
passing: must-fail — predicates naming `dynamism` and `binary_change`; a `derived` entry with no `deriver`; a `source`
outside the five tokens; a `kernel` entry carrying a `deriver`; a `require` row keyed on a `declared` facet; a fourth
domain shape; **an entry that abstains without `nullable = true`**; must-load — a positive `nullable = true` entry
whose `null` read emits **exactly one** `gate_opened { reason: unknown_facet, axis }` per read facet and **never** an
`Unsatisfiable`; **the verbatim `[facet.surfaces]` block** (`source = "declared"`, no `deriver`, `set = true`,
`values`); and a positive `set = true` entry whose member predicate resolves, *"so the shape is recognised rather
than merely tolerated"*.

**AC-071 — the unblocking gate.** Its *Verified by* is **six must-fail fixtures**: *"a fixture `facets.toml` declaring
`source = "llm"` with no record, which must fail load; a second fixture with a record that fails one clause at a time
(5 cases)"*, and its dated note forecloses the obvious reduction: *"A discharged clause is not a partial unblocking,
and the five-case fixture set stays at five."* The five clauses are identity ≥ 0.80, n ≥ 20 items, ≥ 10 draws per
item, arm-D panel **scored**, corpus **pinned**. **Arm D's clause being discharged for arm D does not remove its
fixture** — that is exactly the reasoning the register forecloses. *(This file read "four must-fail fixtures, one per
clause" while naming five clauses, and contradicted its own STOP banner two pages up. The register owns the count
under ADR-0092.)*

**AC-520 — the derived-facet sibling gate.** Take the fixture set from **the register's enumeration, clause by
clause, and never from a count restated here**: no `deriver`; a named `deriver` with no scored record; a record
carrying 0 positives whose facet keys a `require` row; a `deriver_class = "lexical"` facet keying a `require` row; no
`deriver_class`, **and** one whose `deriver_class` is neither of the two tokens; no `deriver_record`, **and** one
whose `deriver_record` names a path absent from the pinned corpus; a `kernel`-class facet carrying a `deriver`, a
`deriver_class`, a `deriver_record` or claiming a record (ADR-0093 forbids); a `require` row keyed on a `declared`
facet — **plus the positive companion that must load** (a `derived` facet with a named `deriver`, a scored record and
positives; a `kernel` facet with none; a `declared` facet keying no row), *"without which the rule is satisfied by a
lint that rejects everything"*. **The register's own headline count is stale and `ERRATA.md`'s E-0520-1 says so
(§7b.2(3)); read the enumeration, not the number.**

Three further things AC-520 fixes that a two-property reading would miss:

- **A record reading NOT MEASURABLE exists, is pinned and is read, and satisfies the clause.** It is neither a pass
  nor a failure (golden constraint 10, Halt 9), and a facet whose record shows no positive instance may key no
  `require` row. At the pin, `recent_deploy` is `structural` and barred for having none; `data_migration` is barred
  twice over.
- **The lexical bar and the no-positive-instance bar are separate clauses doing separate work.** `ui` and
  `flag_exists` are barred as lexical **while carrying positives**.
- **Absence is never inferred**: a `derived` facet with no `deriver_class` fails load exactly as one with no
  `deriver` does. The form is `PROGRESS.md` row L1, taken 2026-09-13, and it is on disk in `spikes/p9/facets.toml`.

**Build them as three rules with three fixture sets, never as clauses of one gate.** AC-071's own residual-risk
sentence names the failure mode: *"a sixth clause with a different subject would be the first thing an implementer
collapsed."* Checkpoint 7b asserts **each rule by its declaration** and **each fixture by name**, not by a count of
lines containing `fn ` — a count of lines is a proxy that two helpers of one rule satisfy while the other rule is
absent, and that a doc comment naming either token also satisfies.

**The record the provenance lint reads exists as of Slice 0 and is not in the corpus yet.** `--facet-provenance`
resolves `deriver_record` **corpus-relative through `CORPUS_PIN`, never through a working tree** (AC-464), and every
`deriver_record` in `spikes/p9/facets.toml` names `results/p1-arm-d-panel-scored.json` in the pinned corpus. That
path becomes resolvable at the single pin move of §7b.11. **Until it does, the provenance fixture set is exercised
over the in-crate fixture registries of §7b.12 and the corpus-resolution half is `armed, not discharged`.**

**AC-520's own record says it both passes and cannot pass, and this step states that rather than choosing.** The
register says *"unlike AC-071, as of ADR-0093 it **passes** on the registry at the pin, because every facet it gates
has a record"*, while the 2026-09-13 amendment's clause (3) says the record it must read **does not exist** and
withdraws nothing. **An erratum reconciling the two is OWED**; the BEFORE/AFTER expectation pair the checkpoint
convention wants (PIA-035) is authored once it names which half stands.

### 7b.8 Rigor monotonicity — the predicate is ADR-0081's

The **rigor-monotonicity lint**, failing closed (**AC-105**, SM-035 — corrected 2026-09-12 from AC-104, which is the
family-*totality* criterion; SM-035's Instrument line names AC-105 by id).

**The predicate is ADR-0081's, not this file's.** ADR-0081 is accepted, `superseded_by: []`, and is the first record
on AC-105's `decisions:` line. Clause 1 (:70-80): for every family and every context in the enumerated lattice,
`evidence(select(family, ctx, r2)) ⊇ evidence(select(family, ctx, r1))` for `r1 < r2`, where `evidence(a)` is the
**closed set of evidence variants `a`'s steps mint**, fixed per step kind by ADR-0036 — so the check is computable
from the catalog with nothing run. **Losing an `agent` step's `Findings` (the adversary) or the `check` step that
consumes it is a violation whatever else the higher-rigor producer adds**; `bisect` at `(has_repro = true, rigor =
mandatory)` is the first recorded instance (ADR-0081:90-91). Clause 2 (:82-88) fixes two more things: monotonicity and
totality are **separate obligations on the same enumeration and both are errors, not warnings** (AC-105 owns
monotonicity, AC-104 owns totality, ADR-0056), and the instrument it names is **`lattice --validate`**, which today
implements only totality — `grep -ci monoton spikes/p9/lattice.py` → **0** [measured 2026-09-13].

**Where AC-105's THEN and ADR-0081 clause 1 differ, ADR-0081 governs (ADR-0092).** AC-105 fails the check when the
graph at *r+1* omits a *quality-apparatus step (adversary, policy, or declared quality block)* present at *r*;
ADR-0081 requires the stronger superset relation over the closed evidence-variant set and names the loss of a `check`
step as a violation — a step kind AC-105's three-way enumeration does not reach. **The lint implements the superset
relation**, and AC-105's THEN is read as a non-exhaustive instance of it. **An erratum is OWED** and this step does
not write one.

**AC-105 is nonetheless reported DISCHARGED, not ARMED, and the difference from AC-044/AC-045/AC-043/AC-056 is
stated.** Those four are ARMED because their THEN is **unsatisfiable or unnamed** at this stage — AC-044's at crate
scope, AC-045's key has no name in the register, AC-043's third state does not exist in it, AC-056's golden rates are
true only on one pack series. **AC-105's THEN is satisfiable and is satisfied**: a superset check over the closed
evidence-variant set passes every case the narrower three-way enumeration names, so an implementation conforming to
ADR-0081 conforms to AC-105 by construction. The erratum owed is about the **record's completeness**, not about
whether the criterion is met, and that is why AC-105 carries no ARMED row in §7b.15.

🛑 **Building the check as `cargo test -p rk-kernel -- rigor_monotonicity` MOVES THE INSTRUMENT AN ACCEPTED ADR
NAMES, and the substitution is recorded rather than called execution order.** ADR-0081 clause 2 is a normative must
on a named tool — *"`lattice --validate` **must check block monotonicity**, alongside the family-totality check it
already performs"* — and `spikes/p9/lattice.py` is the tool that performs that totality check today. This stage fixes
`lattice.py --validate` as the **totality** guard only (**AC-104**) and puts monotonicity in a Rust test. **The
obligation is unchanged and nothing is withdrawn; what moves is where it runs, and a stage may not move an
instrument a record names without saying so.** **Owed: either `spikes/p9/lattice.py --validate` gains the
monotonicity check, or an erratum against ADR-0081 clause 2 records that its Phase-1 instrument is
`rigor_monotonicity` in `rk-kernel`, with the product lint following at 7e/8 and named there. Neither is written
here** (`00-preamble.md`:55). Until one lands, `rigor_monotonicity` is clause 2's **Phase-1 stand-in** and is named
as such rather than as its home. **Neither tool has ever measured monotonicity** —
`grep -ci monoton spikes/p9/lattice.py` → **0** [measured 2026-09-13] — so `rigor_monotonicity` is its first
measurement either way.

**The ladder was not monotone in internal quality**: escalating to `mandatory` with `has_repro` selected `bisect`,
which has **no adversary and no policy step**, so escalating rigor *dropped* the quality block (F37). **P7-CEIL is
decided: `bisect` is a ceiling activity** — `spikes/p9/catalog/bisect.toml`:16 carries `max_rigor = "recommended"`
since 2026-09-11 (ADR-0093 decision 5; DECISIONS-2026-09-11 #1). Re-measured read-only 2026-09-13:
`python spikes/p9/lattice.py --validate` exits **0** and prints `validate errors=0` with **holes=0** in every family.
**Adding the missing steps to `bisect` is the Phase-2 repair under ADR-0072**, which the design says three times.

**The red end is an authored fixture catalog, not a past state of the tree.** The pre-P7-CEIL condition stopped
existing on 2026-09-11, so copy `bisect.toml` into `crates/rk-kernel/tests/fixtures/catalog/` **without** the
`max_rigor` line and assert the lint fails on it. **Green on `spikes/p9/catalog/` as it stands** — that is the catalog
this stage has; the product catalog under `.redkiln/` is Step 7e's and its re-run is Checkpoint 7e's.

🛑 **`spikes/p9/lint.py` is NOT run by Checkpoint 7b, and must not be.** This file claimed *"Checkpoint 7b runs it"*
until 2026-09-13 and no command there ever did — and the claim is not merely unimplemented, it is **unimplementable
here**: the P9 catalog lint has been **red by design since Step 4a withdrew `auth`**. Re-measured read-only
2026-09-13: `PYTHONIOENCODING=utf-8 python spikes/p9/lint.py` exits **1** with **`errors=3 warnings=2`** — three R9
`applicable_if: identifier 'auth' is not a facet` errors at `catalog/security-requirements.toml`,
`catalog/security-review.toml` and `catalog/threat-model.toml`. Those three, plus four `when = "auth"` pack rows the
lint never sees — **seven sites** — are **`s7e` §7e.2's**, and `s7e` §7e.4 is the only block that asserts the lint at
`errors=0`. **And do not silence the three by declaring `auth` among `surfaces`'s values**: `lint.py`:339 accepts any
bare identifier matching a facet *value*, which is 0 errors with it and 3 without, measured both ways
(`PROGRESS.md`:25) — golden constraint 9's too-narrow direction, and Halt 7's *"stop at any refusal and establish the
guard's scope first"*. What Checkpoint 7b asserts is **the kernel's** rigor-monotonicity rule over the shipped
catalog, by named test, with the catalog path pinned in that test — two different instruments over the same catalog,
which this file previously named only as *"it"*.

### 7b.9 The seven-step selection function, and the C-02 pass

**Before this step, not after (audit §8 C-02).** The audit's C-02 row asks for *"one slice finder and refuter over
AC 608–1008 and design §4.4–4.6, with the two-implementer test as the primary lens"*. **"AC 608–1008" are line
anchors at the audit's own pin `009a4ec`, not AC ids** — at that pin `### 3.1` is line 608 and `### 3.7` is line
1009, and the register's highest allocated id is **AC-531**, so no AC-608 exists or can exist. The subject is the
register's **§3.1–§3.6**, which on this tree spans lines **720–1182** and holds **32 criteria: AC-040–AC-068 plus
AC-077, AC-078 and AC-525** [measured 2026-09-13, counted off the file]. It covers **all six** areas the audit names,
not the four this file used to list: purity/replay, rank/ties, **rigor and stage scope**, the unsat gate, typed
deviation, **short-circuit and staleness**; **21 of 40 criteria were untouched** by the 2026-09-10 audit. The second
target, **design §4.4–§4.6**, was dropped from this paragraph and is restored.

**The pass has been run.** It ran on 2026-09-13 as the `design-spikes` lens of the seven-lens adversarial audit that
produced this repair — one finder, two refuters and an adjudicator that reproduced every citation at the primary
source. **Recording it in `PROGRESS.md` against C-02 is owed to the operator; this file states the results.** Nine
survived adjudication and each is a reading an implementer working from the register alone would get wrong:

1. **Rank and ties — the register's §3.2 alone forces the WRONG reading.** There is **no runtime tie-break in producer selection**: a
   tie is a **pack validation error at load** (ADR-0056:71, :81 — accepted, above the register), and **specificity is
   withdrawn** for producer-of-a-kind selection (ADR-0073:24; design row 1c, *"no specificity tie-break is consulted
   for the mixed case"*). AC-047's *Cut* removes the absence claim as a **criterion**, not as the design, and is not
   licence to build a tie-break; AC-049's evidence line records specificity picking a producer as the **measured
   defect**, not the contract. **Two specificity rules exist in this crate and must not become one code path**: the
   binder's subsumption rule over agent-card selectors survives (AC-087); producer specificity does not (AC-088), and
   the anti-tiebreak lint must be scoped to the mixed case **or it breaks AC-087**. Mixed-toolchain items resolve by
   **toolchain-keyed kinds, one instance per element of the item's set** (AC-049), never by specificity.
2. **Purity and replay — what AC-040 does not say.** *"Byte-identical"* is identity **under the canonical serialiser**,
   which already exists as `rk_store::canon` (*"Object keys are sorted byte-wise … LF only, no trailing newline …
   Floats are refused"*) and which ADR-0097 decision 2 component 4 already names for step inputs. **Do not write a
   second one.** `serde_json` carries `preserve_order` **workspace-wide** (root `Cargo.toml`:44), so a plain `to_vec`
   over an insertion-ordered map leaks construction order into AC-040's assertion **and can still pass its 100
   shuffled repetitions** if the shuffle is of call order rather than of map construction. AC-040's *Scope limit*
   licenses **in-process, single-machine** identity only, and the residual is AC-042's — Phase-1 exit criterion (2).
   **The layering is stated here and not taken silently at the first manifest line:** either `rk-kernel` depends on
   `rk-store` for `canon`, or `canon` moves to `rk-types` beside `digest` so `rk-kernel::select` reaches it without
   the store crate. Whichever is chosen is written into §7b.4's dependency list **in the same commit**. **And if
   `canon` MOVES, it edits a crate whose checkpoint has passed** — `rk-store` at Checkpoint 6 (`ea83eac`) and
   `rk-types` at Checkpoint 5 (`96ba9c2`) — so `00-preamble.md`:232-237 applies without exception: *"A checkpoint
   that changed after it passed is provisional."* **Mark the row `passed <hash>, re-run pending 7b canon` in
   `PROGRESS.md` §1 and re-run that checkpoint in the same commit as the move.** `s7b-ii` §7b-ii.12 states the same
   obligation for the same class of edit; this sub-step carried the edit and not the obligation until 2026-09-13.
3. **AC-043 has three states and the register names two.** AC-042's replay assertion runs in CI **after** the run, so
   at memo-read time a fresh entry is *not-yet-asserted* — a state AC-043's GIVEN (*"a run whose replay assertion
   failed"*) does not reach and whose poisoned-run-id fixture cannot distinguish. **Fail closed**: an entry whose
   minting run is not recorded replay-clean is refused and re-materialised, because **absence of evidence is a block**
   (principle 4) and it is the same rule AC-071 and AC-520 mechanise. **The second refusal reason is a register
   amendment and comes from AC-043's, not from this stage** — `memo_evicted { reason: unsound_replay }` is the wrong
   word for *unverified*. ADR-0119 records the same residual as explicitly not decided.
4. **The unsat gate's shape, which the register's §3.4 gives twice and differently.** Step 4's unsatisfiability is **data in the DAG**
   — a gate node — **not an abort**: AC-058's gate *"carr[ies] the DAG"* and its byte-identity test needs an assembled
   `R`, so `select()` returns `Ok(Dag)` with the kind gated and the rest of the graph intact, and **nothing is dropped
   from `R`** (AC-054). Step 5's failures **are** aborts: AC-078 fails *"before any node is dispatched"* on a DAG
   *"constructed inside the crate"*. And `Unsatisfiable`'s `failed_predicates` carries **evaluated operands, not
   predicate ids** — AC-055 requires *"the evaluated value of each facet that predicate read"* and AC-063 requires
   *"its evaluated operands attached"*. Re-deriving them at gate-write time is a **second** evaluation of a
   three-valued grammar over a context that may have been re-expanded, and AC-055's *"rejected by the store's single
   write path"* turns a disagreement into a runtime write failure rather than a test failure. **Type the operands into
   the variant.**
5. **AC-525's "exactly one" is an idempotency key, not a log scan.** Step 3 is inside the pure function and AC-041
   makes the four digests its **only** inputs — so the advisory's identity is **derived deterministically from
   `(root, upstream, new_digest)`** and the store's single write path makes the append idempotent. **A `select()` that
   reads the event log to de-duplicate is the hidden input AC-041–AC-043 exist as a chain to prevent, and it passes
   all three of AC-525's named fixtures.** Author a fourth: a replay assertion over a run that emitted an advisory,
   which must re-derive the same DAG with **no store read**. **Raised, not settled:** AC-525 needs a dated amendment
   saying whether *"the next gate the item reaches"* is **run-scoped** or **item-scoped across runs**, and if the
   latter, which of `advisory_created` / `advisory_retired` carries the lifecycle.
6. **AC-066 and AC-067 classify one artifact two ways, and the registry breaks the tie.** `derived_from = []` is
   **not** the vouched-root test — the **registered kind's `source`** is: AC-067's GIVEN conjoins `source = "human"`,
   and AC-053 with ADR-0057's Decision make `artifacts.toml` the authority. Classify in that order: registered
   `source = "human"` → vouched-root path, `Present` plus at most an advisory that can never promote itself;
   otherwise AC-066's absence rule → `Stale`. An implementer applying AC-066's blanket absence rule first re-dispatches
   a producer against a human-authored artifact on every run — which AC-067's own THEN forbids by name — and
   suppresses the advisory path, making AC-525 dead code. **The discriminating fixtures are a register edit and are
   OWED**: AC-066's 8-case table gains a human-sourced kind carrying a declared input (must be `Present`) and a
   machine-sourced kind of the same shape (must be `Stale`).
7. **A forbidden stage reached by a *conditional* `requires` produces both a skip row and a gate.** AC-051's load-time
   failure covers only the **unconditional** case, so the conditional configuration loads and the runtime case is
   live. ADR-0057's Decision — *"`forbidden` means … nothing in that stage runs, even via `requires`"* — gives
   AC-077's skip row, in **`dag_assembled.skipped`**; AC-054's *"no dropping the artifact from `R`"* gives the gate.
   **Both, not either.** `forbidden` is a member of the `Rigor` ordinal (AC-050) while behaving as a stage class,
   which is what makes the arithmetic route through AC-052 look right — it finds *"no candidates"* and silently drops
   the requirement. Author the fixture neither section has.
8. **AC-044's THEN is unsatisfiable at crate scope and an amendment is OWED.** It requires the selection crate's
   dependency graph not to contain the `Harness` port; ADR-0099 clause 5 makes `rk-select` a **module** of
   `rk-kernel`, and `rk-kernel` depends on `rk-ports`, which declares `Harness` by construction. Until the amendment
   lands, **AC-044 is reported `armed, not discharged`**, and what is built is the reachability property: **no code
   path reachable from `select()` names the `Harness` port or an adapter that can dispatch a model** — a source parse
   over `crates/rk-kernel/src/select/` plus AC-484's graph assertion. *(AC-041 needs no amendment: its *Verified by*
   already offers *"a workspace `forbid` test"* and its *Instrument limit* already assigns the semantic residual to
   AC-042.)*
9. **AC-056's regression fixture pins two rates that are true only on one series, and an amendment is OWED.** Its own
   evidence line says so: `23.5% → 0.3%` is the **8 pre-registered packs at pack-default rigor** series, and *"the
   all-12-pack series beside it is 31.2% → 12.2%"*, while its GIVEN says *"for the configured packs"* and its
   *Verified by* states neither the pack set nor the rigor mode — so **a correct 12-pack measurement fails the golden
   assertion**. AC-057, the very next criterion, already states the fix for itself: *"The pack count is part of the
   assertion."* The stage-side residue stays here: **name where the F35 pre-decision catalog comes from** — a
   committed fixture path or a reconstruction procedure — because the stage cannot be executed without it.

**One step of the seven has no criterion, and that is the design's own record.** Design §4.4 step 6, *Execute*,
records the gap in terms — **no criterion covers step 6**, and *"the execute step of the pure selection
function itself is uncovered. Recorded as a gap, not papered over."* The adjacent rules are pinned elsewhere —
AC-388's stagger; AC-189 and AC-196–AC-198 in the register's git-reconcile section, which are 7c's and Step 6's.
**Build step 6 to its stated invariant — execution cannot change `R`; step 7 is the only channel and it re-enters at
step 2 — and record in `PROGRESS.md` that it ships uncovered, as the design records it.** Checkpoint 7b's green is
not coverage here, and closing the gap with a criterion of this stage's own devising is deciding something the runbook
may not decide.

**And `discouraged` is reserved.** The variant exists in the enum and a pack setting any stage or `min_rigor` to it
**fails validation naming the site** (AC-050's last clause; DECISIONS-2026-09-11 #20 choice A; design §4.4 step 1).
Its semantics are Phase 2's and may not be defaulted here — option B, a fourth live rigor level, was rejected for
having no measured need.

**Four `trybuild` compile-fail obligations, not one.** AC-041 (the selection crate cannot read the clock, env,
filesystem, network or a random source — design §4.4's reproducibility contract; the precedent is measured: a default
`claude -p` inherited 12 MCP servers and ~280 tools, **56% of the cache write**), **AC-050** (no `Add`/`Sum`/`Into<f64>`
on `Rigor`), **AC-020**'s separation test over the three budget counters, and **AC-294**'s block constructor, which is
`s7b-ii`'s. **AC-021, AC-022 and AC-023 are unit tests, not compile-fail, and AC-064's is a unit test on the delta
event — do not fold them in**; assigning `trybuild` to a criterion whose register instrument is a unit test changes an
instrument, which is the one thing a stage file may not do.

### 7b.10 The kernel runtime — the memo, the budgets, and the constants with no measured value

**The memo.** Design §3.5 fixes the step key as SHA-256 over a length-prefixed ordered tuple of **eight** components:
`activity_id@version`; the rigor variant selected; the step id; the digest of each resolved step input in declaration
order; the card digest, **or** for a `tool` step the Toolchain adapter's detected version string plus the command; the
digests of the **resolved** injected KB atoms, sorted; the loop iteration counter (0 outside a loop); the output
type's `schema_version`. *"Changing any one of the eight components while the others are held fixed misses the memo;
all eight identical hits it — one property case per component (AC-014, AC-019)."* Three further rules are separately
regressible: the **activity-level key** is SHA-256 over *"every card digest reachable in the subgraph"* — **not** the
step key and **not** a hash of step keys, and a key computed from a single card reference is a **lint error** (AC-016);
**two loop iterations give two keys and two executions** — the direct regression test for *"loops cannot learn"*
(AC-017); and a memo hit **appends a `memo_hit` event carrying the key and exactly its eight components** — a skip is
never invisible (AC-019). **AC-045's key is a fourth key and is ADR-0119's** (§7b.2(2)); `rk-store`'s three keys share
no code and neither does this one.

**The budgets.** Three counters, **never shared** — distinct newtypes with no `From`/`Add` crossing them, so a shared
counter does not compile; `attempts_total` counts the agent/tool steps of a **pass**, not the sum of retried steps
across passes; a `for_each` over N items charges **once**, independent of N (AC-020–AC-023). Inner retry is AC-026,
outer retry AC-029, the never-retried class AC-027, the disjoint channels AC-032, the breaker AC-033–AC-035,
degradation AC-036–AC-038, exhaustion AC-024/AC-025.

🛑 **`K`, `N`, the backoff base and cap, the two-tick headroom margin and the per-item token appetite HAVE NO MEASURED
VALUES** (design §3.6). `K`, `N` and the appetite are **declared in the pack and refused as code defaults** (AC-033,
AC-023); the backoff base and cap are kernel policy constants (AC-029). The criteria verify conformance to the
**declared envelope**, never that the envelope is right — so build the refusal and the conformance check, and **do not
default a constant here** (`00-preamble.md`:55).

**And the executed-once ledger.** ADR-0114 decision 2: the catalog record carries `executed: { first: Option<{ run_id,
item, at }>, last: Option<{ run_id, item, at }> }` and **the kernel writes it** on the first and every subsequent
`activity_committed` for that activity (ADR-0022's activation record) — nothing else sees that event, and Step 7e
authors TOML only. At v0 every activity but `root-cause`'s P7 members reads `executed: never`. `doctor`'s
never-executed count is 7d's and `lint` **warns, never fails**, past the pack's declared `rot_after` — a pack-level
setting, not a kernel constant. The register routes this through **AC-110**, inside this stage's claimed range.
*(Do not name the red test after AC-114: the register marks AC-114 **HUMAN GATE**, and naming a red test after it
would claim a human gate as buildable.)*

### 7b.11 The golden logs, and the single `CORPUS_PIN` move

**AC-006's instrument is three assertions and this file used to carry one.** *"GIVEN a recorded event log … WHEN the
workflow is replayed against that log **with every port bound to its fake** / THEN the sequence of decisions and the
next action are byte-identical to the recorded ones"*, verified over logs recorded by `rk-kernel` itself on the pinned
corpus **and asserting zero `rk-git` writes on replay** — ADR-0105 decision 1: *"on replay the kernel is served the
journal and `rk-git` performs zero writes"*. **AC-008 adds the fourth and fifth**: *"a replay test with a
scripted-responder Harness fake that fails the test on any `dispatch` call, and that separately asserts the terminal
state matches the record — so a kernel that does nothing at all fails the terminal-state half."*

**AC-161 and AC-164's next-action halves are claimed here**, handed forward by Step 6 because their instrument is
*"the delete-the-database replay test in `rk-kernel`"*, which does not exist until this step (`PROGRESS.md`:343,
:349). **AC-184's projection half is not claimed here** — it waits on `rk-cli`'s `doctor --rebuild` at 7d.

**The per-run layout inside `golden-logs/` is this stage's to fix**, and the register's amendment says so in terms.
Write it into this section in the commit that authors the replay test, before the pin moves.

**The single `CORPUS_PIN` move, at the END of Step 7b.** It carries three artifacts in one corpus commit and one pin
bump: **(1)** AC-520's scored precision record, `results/p1-arm-d-panel-scored.json`, produced by
`spikes/p1/score-arm-d-panel.py --out` (exists as of Slice 0); **(2)** AC-071's stability record, **transcribed from
F36's frozen output and never regenerated** (§7b.2), at the path AC-071's owed amendment names; **(3)** the golden
event logs under `golden-logs/`, produced by Slice 3's replay test.

**Seven preconditions. Any one false, stop:**

1. Slice 3 is green: `golden_log_replay` passes and **its logs were produced by that run, not hand-made**.
2. AC-071's stability record exists, was **transcribed from F36**, and its dated register amendment has landed naming
   its path and its producer.
3. The `golden-logs/` per-run layout is written into this section.
4. `git status --porcelain` is empty in **both** repositories, and `git -C "$CORPUS_REPO" rev-parse HEAD` equals
   `CORPUS_SHA` — AC-464 refuses a dirty or unpinned corpus **before any read**.
5. Nothing corpus-shaped is tracked in `redkiln-rs` (AC-465): the staged records live in a scratch tree, never under
   `spikes/results/`.
6. `CORPUS_BACKUP_BUNDLE` is re-cut **after** the corpus commit and **before** the pin edit — `CORPUS_PIN` documents
   it as PIA-021's off-disk copy, and a pin naming a bundle that predates it is a stale backup.
7. **The operator, not an agent, performs the corpus write and the pin edit.**

**"Checkpoint 7b has passed" is deliberately NOT among the seven, and the two are not a loop.** The move runs
**before** Checkpoint 7b (§7b.3's slice table, row 4), because §7b.14's block asserts `golden-logs/` non-empty and
`results/p1-arm-d-panel-scored.json` present **in the corpus** — both true only after the move — and reads every
corpus-bound line at the **new** pin. What gates the move is the seven preconditions above, of which the first,
Slice 3 green, is the substantive one.

**The order:** produce all three artifacts into a **staging** tree, never into the corpus; verify the staged artifacts
**before** any copy — the scored record carries the pin, 140 truth cells and a per-facet interval-plus-positives pair;
the stability record carries **F36's four figures and not 1.00**; `golden-logs/` is non-empty; then one corpus commit
carrying all three; then re-cut the bundle; then the **one** `CORPUS_PIN` edit, anchor-asserted before it writes
(golden constraint 7) and re-asserted as 40-hex after; then re-run **every** corpus-bound assertion at the new pin;
then commit `CORPUS_PIN`. **Rollback is one line and is rehearsed before the pin edit**: restore the old
`CORPUS_SHA`. The corpus commit itself is additive and need not be reverted — the pin is the switch, and
`s9-rollback.md` already treats `CORPUS_PIN` as in scope.

### 7b.12 Where the fixtures live

Every criterion in this stage's lint set is discharged by fixtures, and **none of them may be a shipped artefact**.
`.redkiln/` does not exist here — Step 7e derives it (`s7e` §7e.1; `ls -d .redkiln` → *"No such file or directory"*,
measured 2026-09-13) — and `spikes/p9/`'s packs, catalog and `facets.toml` are the corpus the probe and the lattice
validator are measured against, so they **may not be edited to make a red case**. Author every fixture under
`crates/rk-kernel/tests/fixtures/`:

| directory | what lives there |
| --- | --- |
| `registries/facet-stability/` | AC-071's six: `no_record.toml`, `identity_below_bar.toml`, `n_below_twenty.toml`, `draws_below_ten.toml`, `panel_not_scored.toml`, `corpus_not_pinned.toml` |
| `registries/facet-provenance/` | one per clause of AC-520's enumeration — `no_deriver`, `deriver_without_record`, `record_zero_positives_keys_require`, `lexical_keys_require`, `no_deriver_class`, `deriver_class_not_a_token`, `no_deriver_record`, `deriver_record_absent_at_pin`, `kernel_facet_carries_deriver`, `require_keyed_on_declared` — plus `positive_companion.toml`, which **must load** |
| `registries/entry-shape/` | AC-072's eleven, including the verbatim `[facet.surfaces]` block copied rather than referenced |
| `catalog/` | the pre-P7-CEIL `bisect.toml` (no `max_rigor`) for `rigor_monotonicity`'s red end; the F35 pre-decision catalog for AC-056 (§7b.9(9) — name its path or its reconstruction) |
| `packs/` | AC-050's `discouraged` pack; AC-069's four narrowing-pattern fixtures; AC-495's two pack-validation packs |
| `activities/` | AC-527's three: the verbatim `findings.none(…)` terminal condition, `findings.all(…)` over an empty set, and the positive that terminates on a verify result |
| `ui/` | the `trybuild` cases of §7b.4, each with a committed `.stderr` |
| `logs/` | the golden event logs' in-crate copies for the red step, before the corpus copies exist |

Each fixture is named for the criterion it discriminates, and each one is committed.

### 7b.13 Red first

**Red first (golden constraint 12; ADR-0019).** The checkpoint below is the floor, not the evidence: every one of its
`cargo test … -- <filter>` lines is satisfied by an implementation that nothing pins. Author each named test
**before** the code it describes, run it, and watch it fail for the right reason — the absent behaviour, not a compile
error or a filter that matched nothing.

**Slice 1 — the crate, the gate and the grammar:**

- `workflow_clock_read_is_red` — §7b.5, and it is a **committed non-default feature**, not a reverted observation:
  `cargo clippy --workspace --all-targets --features rk-kernel/workflow-clock-reintroduce` must exit **non-zero**
  with `disallowed_method` in its output, and Checkpoint 7b asserts both halves. It is the module-level `#[deny]`
  grep's positive control and its only behavioural pair. **The module attribute, not `clippy.toml` alone, is what
  makes it red.**
- `grammar_agrees_with_the_s_grammar_probe` — the kernel's parser and `spikes/p9/grammar-probe.py` agree
  **string-for-string** over the live corpus. Red by accepting one of the two known parse failures.
- `grammar_precedence_and_associativity` — `not` > comparison/`in` > `and` > `or`, left-associative; **0 live
  predicates depend on precedence**, so this is red against an authored fixture, not against the corpus.
- `grammar_bare_identifier_is_a_literal` — red by treating an unquoted identifier as a field reference; **191 sites**
  read one way only because of this rule.
- `grammar_comparisons_do_not_chain` / `grammar_field_to_field_needs_a_ref` — ADR-0103 decisions 1 and 2.
- `on_false_closed_set` — the three `catalog-b` rows are the only live instances; red each.
- `predicate_null_arithmetic` — one must-fail case per arm: `null == null` true; `null == v`, `!=` and `in` yielding
  `null`; `not null` → `null`; three-valued `and`/`or`; and an ordering against a nullable facet a **load-time lint
  error** unless guarded by the one narrowing pattern (ADR-0069).
- `quantifier_empty_and_null` — `all` and `none` over `null` or an empty list are **true** while `any` is **false**,
  including the ADR's named must-fail fixture — **`any` over absent evidence must not pass an activity** — plus the
  positive that a top-level `null` emits **exactly one** `gate_opened { reason: unknown_facet, axis }` per read facet
  and **never** an `Unsatisfiable`.
- `no_generics_leak` — AC-480's compile-fail fixture, handed here by name at `s7a`:995. Red by adding one
  `<P: Harness>` bound. **This is a `trybuild` CASE, not a `cargo test` name**: the file is
  `crates/rk-kernel/tests/ui/ac480_generics_leak.rs`, it runs inside the one harness, and Checkpoint 7b asserts it
  **by path** together with its committed `.stderr`. A `cargo test -- no_generics_leak` filter would select zero
  tests and exit 0.
- `stay_class_unrepresentable` — Phase-1 exit criterion (5)'s fixture, also a `trybuild` CASE and not a `cargo
  test` name: `crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs`, asserted **by path** with its
  `.stderr`. *(This bullet said "under its **own** filter name, so Step 8's filter cannot be satisfied by a
  different stage's case" until 2026-09-13. The premise was false — see §7b.4: `s8`:121's `trybuild` filter DOES
  select this crate's harness, and what it cannot see is which cases ran. The by-path assertion is the fix, and a
  matching one in `s8` check (5) is OWED.)*

**Slice 2 — the three rules:**

- `facet_registry_entry_shape` — AC-072's eleven cases, red separately. The `nullable = true` abstention case and the
  three must-load cases are the discrimination; a reject-everything loader passes without them.
- `facet_stability_five_clause` — **six must-fail fixtures**: one declaring `source = "llm"` with **no record at
  all**, plus **five** carrying a record that fails exactly one clause. Red each separately: a lint that reads
  identity and skips the panel clause passes a single happy-path test and unblocks the whole provisional apparatus.
  The five clause negatives plus the no-record case are the discrimination; one positive test is not.
- `facet_provenance_fixtures` — **every clause fixture AC-520's *Verified by* enumerates, plus the one that must
  LOAD.** Take the set from the register by reference; the must-load companion is not decoration — without it the
  criterion is satisfied by a lint that rejects everything.
- `rigor_monotonicity` — red on the authored pre-P7-CEIL fixture catalog (§7b.8), green on `spikes/p9/catalog/` as it
  stands with `bisect` ceilinged.
- `ac110_executed_ledger_first_and_last` — commit an activity twice; `executed.first` is pinned to the first
  `activity_committed` and `executed.last` moves to the second. Red on a kernel that writes neither.
- `ac527_empty_set_terminal_condition` — **three** fixtures: the verbatim predicate, which must fail;
  **`findings.all(...)` over an empty set, which must ALSO fail**; and a positive terminating on a verify result,
  which must **load**. The `all`-over-empty case is the discrimination — a lint matching the `findings.none(…)`
  string alone passes it, and the rule is on the **form**, not the string: ADR-0080's forbids clause reads *"or any
  policy predicate satisfiable only by an empty finding set"*.
- `rigor_discouraged_refused` — a pack fixture declaring `discouraged` that must fail validation naming the site,
  plus the admissibility unit table across all five levels.

**Slice 3 — the selection engine:**

- `selection_seven_steps_are_pure` / `rank_total_order_and_tie_is_an_error` / `unsatisfiable_is_a_typed_gate` /
  `deviation_never_edits_the_required_set` — one named test per register §3.1–§3.6 clause. *(The fourth was
  `deviation_never_edits_R` until 2026-09-13; the uppercase `R` emits a `non_snake_case` warning at every build of
  the crate — the workspace denies only `unsafe_code` under `[workspace.lints.rust]` (`Cargo.toml`:51-52), so it is
  noise rather than a failure, but it is noise in a record transcribed into `PROGRESS.md`.)* **Nothing else in the checkpoint reaches them**:
  the unfiltered `cargo test -p rk-kernel` is satisfied by any test the implementer wrote.
- `forbidden_stage_via_conditional_requires` — §7b.9(7): **zero dispatches, the skip row in `dag_assembled.skipped`
  naming the stage, AND an `Unsatisfiable` gate for the kind with `R` unchanged**. Both, not either.
- `memo_key_eight_components` — **eight must-miss cases plus one must-hit**, one per component of design §3.5's tuple.
  Hold seven fixed, move one, assert a MISS; all eight identical, assert a HIT.
- `memo_activity_key_distinct` — the activity-level key over every reachable card digest; a key computed from a single
  card reference is a **lint error**.
- `memo_loop_iteration` — two iterations, two keys, two executions.
- `ac525_drift_advisory_once` — **exactly one** `drift_advisory { root, upstream, old_digest, new_digest }` with all
  four fields; a **re-entry** case asserting no second advisory for the same digest; a gate case asserting the next
  `gate_opened` record carries it; **AC-067's out-of-set fixture as the negative companion**; and §7b.9(5)'s fourth
  case — a replay that re-derives the same DAG **with no store read**. SM-022 has no baseline, so the noise rate is
  **reported, never thresholded**.
- `golden_log_replay` — the FSM has **never been replayed**. **Five assertions, red separately**: every port bound to
  its fake; byte-identical decisions; byte-identical next action; **zero `rk-git` writes**; and **zero harness
  dispatches with the terminal state matched** (AC-008 — a kernel that does nothing at all passes without the
  terminal-state half). There is no prior green to regress from, so the red step here is the first honest
  measurement, not a formality. **Behind `#[cfg(feature = "corpus")]`** — it reads `$CORPUS_REPO/golden-logs`, and
  D-N/PIA-002 makes a corpus-bound test outside the feature a CI failure.
- `golden_log_replay_writing_fake_is_red` — **the positive control for the three counters above**, and it exists
  because `rk_git_writes=0`, `harness_dispatches=0` and `terminal_state=matched` are strings the replay prints
  **about itself**: a kernel that counts nothing and prints the three literals passes all three greps. This test
  replays against a `GitOps` fake that performs **one** write and asserts the replay **refuses** — the counter must
  move off 0. Red on a kernel whose counter is a constant. Also `corpus`-gated.
- `ac161_ac164_delete_db_replay` — delete the event database and replay; the next action must be byte-identical to
  the recorded one. Step 6 could not author it. Also `corpus`-gated, for the same reason.

**Slice 4 — the runtime.** One red test per counter-independence pair (AC-020–AC-023) and one per named class
(AC-024–AC-038). **All ten are named here AND asserted by name in §7b.14's third loop** — because "each with a
checkpoint line" without the names left AC-007–AC-012 and AC-014–AC-038 reached by nothing but the unfiltered
`cargo test -p rk-kernel` floor, which `00-preamble.md`:186-189 says is not evidence:

- `budget_three_counters_never_share` — the three counters are distinct newtypes with no `From`/`Add` crossing them;
  the separation half is AC-020's `trybuild` case (§7b.9's four compile-fail obligations), the runtime half is here.
- `attempts_total_counts_a_pass_not_retries` (AC-021) — red on a counter that sums retried steps across passes.
- `for_each_charges_once_independent_of_n` (AC-022/AC-023) — red on a charge that scales with N.
- `inner_retry_ac026`, `outer_retry_ac029`, `never_retried_class_ac027`, `disjoint_channels_ac032` — one per rule.
- `breaker_ac033_ac035`, `degradation_ac036_ac038`, `exhaustion_ac024_ac025` — one per named class, `K`/`N` and the
  appetite read from the pack and **refused as code defaults** (§7b.10; `00-preamble.md`:55).

Plus `ac455`-adjacent policy work that is **`s7b-ii`'s**, not this file's.

**§14 hand-forwards, red here:** `ac484_adapter_edge_fixture` (a commit adding `rk-adapter-sqlite` to `rk-kernel` that
**must fail** the graph assertion), `ac487_kernel_denylist` (the adapter-name / `downcast_ref` / `downcast_mut`
denylist plus its compile-fail fixture), `ac494_resolver`, `ac495_pack_validation` (two fixture packs),
`ac498_no_egress_dispatch_zero` (network egress denied, no adapter crates linked, dispatch counter at 0).

### 7b.14 Checkpoint 7b

**Counted mechanically off the block below on 2026-09-13, and after this repair pass and not before it** — comments
and blanks dropped, backslash continuations joined. **The rule that produces the number is stated so it reproduces:**
a *statement that can stop the block* is one carrying its own `exit 1`. That is the unit `PROGRESS.md` records, and it
is named here because **the same number in no unit is this checkpoint's own recorded defect, twice** — it read
*"eighteen … nine"* and then *"All twenty (eleven … three … six)"*, and neither reconciled off the block (Halt 7:
*"the SECOND undercount of the same number, by the same route: not counting it off the script"*). **149 non-comment
statements, of which 90 can stop the block and 59 cannot.** The 59 that cannot are not unguarded assertions and the
decomposition reproduces: **3** BEFORE readings (PIA-035 — prints for comparison, not assertions); **39** `out=` /
`n=` / `k=` / `c=` / `hits=` / `raw=` / `body=` / `deps=` / `tree=` / `inv=` / `h=` / `d=` / `DECL=` / `DECL_LINE=` /
`DENY_AT=` / `note=` / `list=` / `CORPUS_*=` capture halves whose guard is the statement immediately below them;
**1** clippy line whose last statement is an `echo` and whose guard is the line below; **1** `printf '%s\n' "$out"`
that puts the `#[deny]` attribute's own file, line and text into the checkpoint's record; **1** `echo "AC-001: …"`
recording the reintroduce run's verdict, whose three guards are the lines above it; and **7 + 7**, the `for` and
`done` of the seven loops. 3 + 39 + 1 + 1 + 1 + 14 = 59; 149 − 59 = 90. **Re-count and re-write every number in this
paragraph after any repair to this block** — counted off the script by joining backslash continuations and dropping
comments and blanks, never by eye.

*(The previous reading was **134 / 83 / 51** over six loops and 33 captures, counted on 2026-09-13 before the five
line repairs later the same day: the `#[deny]` declaration's `DECL_LINE=`/`DENY_AT=` capture and its level-source
cross-check (`note=`), the corpus differential's per-name loop and its `list=` capture-then-assert pair, and design
§10.2 principle 3's **second** clause with its own control. Each is recorded at its own line with what it measured.)*

**Two lines were DELETED from this block on 2026-09-13, and the obligation is recorded rather than dropped.** They
were `./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml` and `… --facet-stability …`. **Both
depended on two artifacts from two later stages** — `redkiln` is `rk-cli`'s binary (**Step 7d**) and
`.redkiln/facets.toml` is the registry **Step 7e** derives — and measured 2026-09-13, neither `target/debug/redkiln`
nor `.redkiln/` exists on this tree. **Both were also unable to fail**: the first was a `test … && grep … && echo`
chain terminating in `echo` (Halt 8 shape (f)) and the second's whole body after the invocation was `echo "exit=$?"`
(shape (g)). And the comment above them had **withdrawn their `[WILL EXIST]` marking in terms** — *"the lint now
exists"* — which is false at 7b, so Halt 8's only carve-out no longer applied to them. **What Step 7b owes instead is
the lint BEHAVIOUR under `cargo test`**, which the `facet_provenance_fixtures`, `facet_stability_five_clause` and
`facet_registry_entry_shape` groups below assert over in-crate fixtures — no binary, no product registry, and no
`err.txt` written into the repository root. **The CLI-surface halves are re-run at Checkpoint 7e**, which already
carries both in hardened form behind `test -x ./target/debug/redkiln`, and Step 8 check (6) treats them the same way.
**Record the deferral in `PROGRESS.md` against Step 7e** (the input) **and Step 7d** (the binary) — a re-deferral
naming one destination leaves the other half orphaned.

🛑 **ONE HALF OF THAT DEFERRAL IS UNDISCHARGEABLE AS WRITTEN, AND IT IS RECORDED AS SUCH RATHER THAN RE-HOMED.**
Checkpoint 4a's deferred expectation for these two lines is *exit != 0 **AND** stderr names the failing clause*
(`halts.md`:89-91 fixes that form for a `[WILL EXIST]` line). `s7e`:141 expects the **opposite** from the identical
invocation — `# → exit=0: six derived, six records (AC-520)` [measured 2026-09-13] — and AC-520's own register text
says it *"**passes** on the registry at the pin, because every facet it gates has a record"*. **So a fail-closed CLI
run is reachable only over a deliberately-broken registry, which no stage file authors.** Saying "re-run at 7e" for
both halves converts an undischargeable obligation into a satisfied-looking deferral. **What is re-run at Checkpoint
7e is the PASS branch — `exit=0` over the product registry — and nothing more. The fail-closed obligation is
discharged HERE**, by `facet_provenance_fixtures` and `facet_stability_five_clause` over the must-fail registries of
§7b.12, **and the fail-closed CLI-SURFACE half is recorded UNDISCHARGED with that reason** in `PROGRESS.md` §1, on
`s7a` §7a.11's model. §7b.7 carries the underlying contradiction — AC-520's record *"says it both passes and cannot
pass"*, erratum OWED — and this is where it bites. **Four further things are OWED in the tool and in a closed
stage file and are not edited here:** `runbook/tools/check-checkpoints.py`:33's ALLOW entry now matches no line and
should be deleted with these lines; its entry at :32 gives Checkpoint **7e** as the re-run stage for `s4a`'s deferred
provenance line while `s4a`:258 and `PROGRESS.md`:25 say **7b**, and all three should say 7e, the only stage where
both the binary and `.redkiln/` exist; `s4a`:262's `# → exit=0: four must-fail + ONE must-load` carries the same
stale count E-0520-1 corrects; **and the checker itself is blind to shapes it is cited as covering.**

🛑 **What `check-checkpoints.py` attests is three shapes, not eight, and this file does not report its green as more.**
`INDEX.md`:141's comment on check 4 reads *"every checkpoint line can fail (Halt 8 shapes (a)-(h))"*; the tool
implements **COUNT-ONLY, ECHO-GUARD and PIPE-STATUS** and **neither of the two shapes added 2026-09-13** — a comment
quoting the literal the grep searches for, and an assertion satisfied by its target's own FAIL message. Shapes (a),
(d), (e), (g) and (h) are unimplemented. Its own source shows three further gaps: `asserted()` returns true for
`re.search(r'\btest\s+["\$\-]', line)`, which a bare `test $?` status read matches, so an `&&`-chained echo reads as
guarded; `ECHOGUARD` matches only `|| echo`, so a success token on an `&&` chain is never flagged; and `mask()` blanks
`$(…)` bodies and `||`/`&&` but not quoted strings, so an alternation inside a `grep -E` pattern reads as a pipe.
**And an ALLOW entry that matches no line is silently unexercised, so `flagged == allowed` does not certify the
allowlist** — which is the tool's own *"a filter that matches nothing exits 0"* rule, unapplied to itself.
**Every defect this repair fixed survived a green run of it**, including the two that halted the block on today's
tree. **Owed: tighten `asserted()` to exclude a bare `test $?`; add an AND-ECHO flagged shape; add a self-test that
fails when an ALLOW entry matches no line corpus-wide; and narrow `INDEX.md`:141's comment to the shapes the tool
actually implements.** None of it is edited here. **Read this block's green as attesting three shapes, reference
resolution and bash parseability — never as attesting that every line can fail.**

**One more thing the block cannot assert, stated rather than smoothed.** Nothing here verifies that
`lint --facet-provenance` **reads** AC-520's record through `CORPUS_PIN` rather than an in-tree copy, for the same
reason: the lint's CLI surface is 7d's and its argument is 7e's. That assertion is **armed, not discharged** at the
pin move of §7b.11, and the deferral is recorded against 7d/7e.

**Every absence-based ban below carries a positive control OR says per ban why it cannot**, and every grep a comment
could satisfy is **paired with a behavioural test or anchored on a declaration** (`s6`:249-250's house note). A guard
whose pass condition is the absence of output is Halt 8 shape (g) and is the shape this programme has met six times.
*(This paragraph claimed a blanket "every … carries a positive control" until 2026-09-13 and the block carried none.
The controls now present: the module-level `#[deny]` grep is paired with the reintroduce clippy run that must go red;
the manifest async ban with the same alternation over `crates/rk-ports/Cargo.toml` (1 hit); the LLM-ordinal ban with
the same pattern over `spikes/p9/` (103 hits); **design §10.2 principle 3's SECOND clause — the four lexical-deriver
facets, added 2026-09-13 — with the same pattern over `spikes/p9/` (44 hits)**; the leave-the-model ban with the same
pattern over `references/` (14 hits); the lattice hole ban with the presence of the `holes=` key itself (15); and the
three self-reported replay counters with a deliberately-red `golden_log_replay_writing_fake_is_red`. **Two bans have no control and say so in
place**: the resolved-graph async ban — nothing in this workspace resolves an async crate at 7b, so no tree exists in
which the pattern must match, and the control is owed at Checkpoint 7c — and the `rk-adapter-*` edge count, which is
a standing floor by package absence until 7c and is already recorded as such.)*

```bash
cd D:/repos/redkiln-rs || { echo "FAIL: repo root not found"; exit 1; }

# ---- BEFORE readings (PIA-035): each PRINTS on the untouched tree and is silent after the step. They are
# ---- READINGS FOR COMPARISON, NOT ASSERTIONS — nothing below consumes them, and §7b.14's count puts them
# ---- among the statements that cannot stop the block. Three of this block's groups pass in isolation
# ---- BEFORE Step 7b runs (measured 2026-09-13 with rk-kernel absent): the LLM-ordinal guard and the
# ---- leave-the-model guard both return no hits over a crates/ with no kernel in it, and the grammar probe
# ---- already prints its parsed count. Those three attest the TREE until rk-kernel is inside their scope.
test ! -d crates/rk-kernel && echo "BEFORE: no rk-kernel crate yet"
test ! -f crates/rk-kernel/Cargo.toml && echo "BEFORE: no rk-kernel manifest"
test -z "$(grep -rl 'disallowed_methods' crates/rk-kernel/src/ 2>/dev/null)" && echo "BEFORE: no module-level deny in rk-kernel"

# ---- the corpus precondition, BEFORE any corpus read (AC-464). A dirty or unpinned checkout is refused
# ---- rather than read. This block runs at the END of Step 7b, after §7b.11's single CORPUS_PIN move, so
# ---- the pin it asserts is the NEW one and every corpus-bound line below reads at it.
test -f CORPUS_PIN || { echo "FAIL: CORPUS_PIN absent"; exit 1; }
CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
CORPUS_SHA=$(grep '^CORPUS_SHA=' CORPUS_PIN | cut -d= -f2-)
test -d "$CORPUS_REPO" || { echo "FAIL: corpus absent at $CORPUS_REPO"; exit 1; }
printf '%s\n' "$CORPUS_SHA" | grep -qE '^[0-9a-f]{40}$' || { echo "FAIL: CORPUS_SHA is not an unabbreviated 40-hex SHA"; exit 1; }
h=$(git -C "$CORPUS_REPO" rev-parse HEAD); test "$h" = "$CORPUS_SHA" || { echo "FAIL: corpus HEAD $h != CORPUS_SHA $CORPUS_SHA"; exit 1; }
d=$(git -C "$CORPUS_REPO" status --porcelain); [ -z "$d" ] || { echo "FAIL: the corpus checkout is dirty; AC-464 refuses before any read"; printf '%s\n' "$d"; exit 1; }

# ---- the five CI gates, each guarded, workspace-scoped. Checkpoints 5, 6 and 7a each run all five
# ---- (s5:378-390, s6:193-199, s7a:785-793) and this block ran NONE of them until 2026-09-13.
# ---- `cargo build -p rk-kernel` is NOT Halt 1's instrument from Step 5 on (halts.md:5-6), and
# ---- `fmt`/`deny` bite on contact: rk-kernel is new code, and adding a member moves Cargo.lock so the
# ---- licence set is re-derived (s6:189-192). This block passed on a red workspace without them.
cargo build --workspace || { echo "FAIL: cargo build --workspace (Halt 1's instrument from Step 5 on)"; exit 1; }
cargo fmt --all --check || { echo "FAIL: cargo fmt --all --check"; exit 1; }
cargo clippy --workspace --all-targets; rc=$?; echo "exit=$rc"
[ "$rc" -eq 0 ] || { echo "FAIL: the clippy gate is red — AC-001-AC-004's instrument IS this invocation gated on its exit code, and it is Phase-1 exit criterion (4); nursery WARNINGS are permitted (ADR-0102, PIA-030)"; exit 1; }   # → exit=0
out=$(cargo test --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the workspace suite ran 0 tests"; exit 1; }   # → a FLOOR ONLY: rk-types, rk-store, rk-git and rk-ports tests each satisfy it
# ---- the corpus-feature run AND ITS DIFFERENTIAL (Checkpoint 6's pair, s6:215-220). This crate is the
# ---- second with corpus-bound tests, and under D-N/PIA-002 a corpus-bound test outside the feature is a
# ---- CI failure: golden_log_replay and ac161_ac164_delete_db_replay read $CORPUS_REPO, so both carry
# ---- `#![cfg(feature = "corpus")]` (crates/rk-git/tests/it/corpus_f12.rs:19 is the house form). WITHOUT
# ---- the differential the first line attests only that the feature EXISTS — which is what it attested at
# ---- Step 5, and what s6:213-214 replaced. N is the count of kernel corpus tests at the end of Slice 3.
out=$(cargo test --workspace --features corpus 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test --workspace --features corpus exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
# NOT A COUNT OVER AN ALTERNATION, and it was one until 2026-09-13. §7b.13 requires a THIRD
# corpus-gated test whose name CONTAINS the first — `golden_log_replay_writing_fake_is_red`, the
# positive control for the three self-reported counters — so `n >= 2` over
# `.*(a|b).*` is satisfied by `golden_log_replay` plus its OWN control with
# `ac161_ac164_delete_db_replay` entirely absent, while the FAIL message names both as though both
# had been checked [measured 2026-09-13 against synthetic libtest output: n=2, gate passes]. That is
# the rule this block states fourteen lines below in its own words. AC-161/AC-164's next-action
# halves are claimed off the second test and nothing else in this runbook reaches them. The `\b`
# form is the one the three name loops already use, and it is exactly what stops `golden_log_replay`
# matching inside `golden_log_replay_writing_fake_is_red`.
for t in golden_log_replay ac161_ac164_delete_db_replay ; do
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: $t did not run and pass under --features corpus — the corpus was never read (D-N/PIA-002; AC-006/AC-188 for the first, AC-161/AC-164 for the second)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
# HALT 8 SHAPE (c), AND THIS BLOCK'S OWN HARDENING RULE FORBIDS IT BY NAME: "never put a
# status-bearing command left of a pipe — capture with `out=$(cmd 2>&1); rc=$?`". Until 2026-09-13
# this was `k=$(cargo test … -- --list 2>/dev/null | grep -c …)`, which discarded cargo's exit status
# to the pipeline's last stage AND sent its diagnostics to /dev/null. A crate whose NON-CORPUS test
# targets do not compile prints nothing, k reads 0, and the gate PASSES — measured: a deliberate
# unclosed delimiter in src/lib.rs gave cargo rc=101 with `error: this file contains an unclosed
# delimiter` and k=0. That is precisely the configuration nothing else has guarded at this point:
# the two tests are `#![cfg(feature = "corpus")]`-gated, :1143's `cargo build -p rk-kernel` does not
# build test targets and runs four statements LATER, and :1146's `cargo test -p rk-kernel` is later
# still.
list=$(cargo test -p rk-kernel -- --list 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: 'cargo test -p rk-kernel -- --list' did not run (rc=$rc) — its status was DISCARDED left of a pipe until 2026-09-13, so a crate whose non-corpus test targets do not compile read as k=0 and passed this gate"; printf '%s\n' "$list" | tail -20; exit 1; }
k=$(printf '%s\n' "$list" | grep -cE 'golden_log_replay|ac161_ac164_delete_db_replay' | tr -d '[:space:]')
test "${k:-1}" = 0 || { echo "FAIL: rk-kernel's corpus tests are selected WITHOUT --features corpus (got ${k}) — the feature is not what gates them, and D-N/PIA-002 makes a corpus-bound test outside it a CI failure"; exit 1; }
cargo audit --deny unmaintained || { echo "FAIL: cargo audit --deny unmaintained"; exit 1; }
cargo deny check bans advisories licenses || { echo "FAIL: cargo deny check bans advisories licenses — this is also AC-005's deny.toml half"; exit 1; }
cargo build -p rk-kernel || { echo "FAIL: cargo build -p rk-kernel"; exit 1; }   # golden constraint 3: build before you test
# The crate-wide run, asserted separately: every other rk-kernel line below is FILTERED, so a unit test can
# fail under a name no filter names while every guarded line stays green.
out=$(cargo test -p rk-kernel 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test -p rk-kernel exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: rk-kernel compiled but ran 0 tests"; exit 1; }

# ---- AC-001's OTHER instrument half, which is this step's own output and which no earlier checkpoint can
# ---- assert: clippy.toml has NO path or module scoping (clippy.toml:1-3; s5 §5.1), so the workspace tables
# ---- are green with or without the attribute. ANCHORED ON THE DECLARATION and POSITIVELY CONTROLLED — a doc
# ---- comment quoting the attribute satisfies an unanchored grep (s6:249-250).
test -d crates/rk-kernel/src || { echo "FAIL: crates/rk-kernel/src absent — every recursive grep below (the deny attribute, the three lint rules, the LLM-ordinal guard and the leave-the-model guard) would read as a pass over a missing directory: Halt 8 shape (a)"; exit 1; }
# -A 1 IS LOAD-BEARING: the register fixes the attribute, not the file layout, and TWO in-source forms
# satisfy it — an inner `#![deny(...)]` at the head of the module file, whose PATH carries `workflow`, and
# an outer `#[deny(...)]` above `mod workflow;`, whose path does not. The trailing context line is what
# lets one assertion admit both without this checkpoint imposing a layout no record decides.
out=$(grep -rn -A 1 -E '^[[:space:]]*#!?\[deny\(clippy::disallowed_methods\)\]' crates/rk-kernel/src/); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: no module-level #[deny(clippy::disallowed_methods)] DECLARATION in rk-kernel/src — AC-001's Verified-by names it because clippy.toml cannot scope a lint to rk-kernel::workflow, and no other checkpoint in this runbook asserts it"; exit 1; }
printf '%s\n' "$out"   # the hit is PRINTED: for a criterion whose point is that the attribute exists, the record carries its file, line and text
# THE ANCHOR DEFEATS `///` AND NOT `/* … */`, and this is the SEVENTH instance of the shape this
# programme has been bitten by. A Rust BLOCK comment whose body line begins at column 0 matches this
# pattern exactly [measured 2026-09-13: the literal appended inside /* … */ in src/predicate.rs, with
# the REAL attribute deleted from src/workflow.rs, made this grep AND the `workflow` pairing below
# both PASS]. The comment at :1156-:1158 says the grep is made safe by "a run that must go RED" —
# but the reintroduce run below is red WITHOUT the attribute (see its own comment), so the pairing
# closed nothing. A grep cannot tell a comment from an attribute. What CAN is clippy's own
# level-source note, so the declaration's file:line is captured here and CROSS-CHECKED against it.
DECL_LINE='^[^:]+:[0-9]+:[[:space:]]*#!?\[deny\(clippy::disallowed_methods\)\]$'
DENY_AT=$(printf '%s\n' "$out" | grep -E "$DECL_LINE" | head -1 | cut -d: -f1,2)
test -n "$DENY_AT" || { echo "FAIL: the deny hit is not a bare attribute line — grep -A 1 prints context with a '-' separator, so a match found only in the CONTEXT half is not a declaration. READ the printed hit above"; exit 1; }
# AND THE MODULE NAME IS TIED TO THE SAME HIT, not to the whole -A 1 blob. `grep -q 'workflow'` over
# the blob passes whenever ANY hit's path or context line carries the string, so a declaration in the
# wrong module passed while some other line said `workflow`. Either the declaring file's own path
# names the module (the inner `#![deny]` form) or the item on the context line does (the outer form).
printf '%s\n' "$out" | grep -E "$DECL_LINE" | grep -q 'workflow' || printf '%s\n' "$out" | grep -A1 -E "$DECL_LINE" | grep -qE '^[^:]+-[0-9]+-[[:space:]]*(pub[[:space:]]+)?mod[[:space:]]+workflow' || { echo "FAIL: the deny attribute is present but neither ITS OWN path nor the item directly beneath IT names the workflow module — READ the hit above; AC-001 names rk-kernel::workflow and clippy.toml cannot scope it"; exit 1; }
# THE BEHAVIOURAL PAIR, on Step 5's own shape (s5:168-174). §7b.5's `workflow_clock_read_is_red` is
# one `SystemTime::now()` inside `rk-kernel::workflow` behind a NON-DEFAULT feature OF THIS CRATE, so a
# workspace-scoped clippy selects it. IT MUST EXIT NON-ZERO, and the message is asserted too — an unrelated
# failure (a typo, a missing file) would otherwise read as the pass and the fixture would certify nothing.
# NOT `-p rk-kernel --features rk-clock-read/reintroduce`: that is ANOTHER member's feature and the
# invocation ERRORS rather than lints (s5:168-169; §7b.5 says so in terms).
out=$(cargo clippy --workspace --all-targets --features rk-kernel/workflow-clock-reintroduce 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: rk-kernel::workflow CLIPPED CLEAN with the reintroduce feature on — the determinism ban does not bite at all, which is the whole of AC-001's rk-kernel half"; exit 1; }
printf '%s\n' "$out" | grep -q 'disallowed_method' || { echo "FAIL: the reintroduce run failed for some OTHER reason than the clock ban — read the output before touching the attribute (s5:168-169)"; printf '%s\n' "$out" | tail -20; exit 1; }
# 🛑 THE TWO LINES ABOVE ARE NOT THE DISCRIMINATOR, AND THEY CLAIMED TO BE UNTIL 2026-09-13. Their
# FAIL message read "the module-level #[deny] above is DECLARED but does not BITE" — a strictly
# stronger proposition than the run can establish, which is Halt 8's "assert what you actually
# establish". MEASURED on this tree: with `#![deny(clippy::disallowed_methods)]` DELETED from
# workflow.rs this same invocation STILL exits 101 carrying `disallowed_method` twice, because
# `[lints] workspace = true` inherits `[workspace.lints.clippy] all = { level = "deny" }` and
# `disallowed_methods` is a member of `clippy::all` — clippy then prints
# "note: `-D clippy::disallowed-methods` implied by `-D clippy::all`" instead. §7b.4 says this in
# terms already (tests/fixtures/rk-clock-read/Cargo.toml:21-24), so the file carried both
# propositions and they contradicted. The two lines above are still the right GROSS guards and stay.
# WHAT MOVES WITH THE ATTRIBUTE IS THE LEVEL SOURCE: with it, and only with it, clippy prints
# "note: the lint level is defined here --> <file>:<line>" — which no comment can fake — and that
# file:line is cross-checked against the declaration the grep above found. Together they are AC-001's
# module-level half; neither alone is.
note=$(printf '%s\n' "$out" | grep -A1 'note: the lint level is defined here' | grep -oE '[a-z_0-9]+\.rs:[0-9]+' | head -1)
[ -n "$note" ] || { echo "FAIL: the disallowed_methods level was NOT raised by an in-source attribute — clippy attributes it to the workspace table, so this run is red with rk-kernel::workflow's module-level #[deny] deleted and AC-001's module-level half is pinned by nothing but a grep a block comment satisfies"; printf '%s\n' "$out" | grep -B2 -A8 'disallowed method'; exit 1; }
test "$note" = "${DENY_AT##*/}" || { echo "FAIL: clippy raised disallowed_methods at $note but the declaration this checkpoint found is at ${DENY_AT##*/} — the attribute that BITES is not the attribute that was GREPPED"; exit 1; }
echo "AC-001: rk-kernel::workflow's module-level deny is the LEVEL SOURCE for disallowed_methods, as required"

# ---- the manifest. Without `[lints] workspace = true` the crate is outside the workspace deny set and
# ---- EVERY lint assertion in this block passes over an unlinted crate: clippy.toml CONFIGURES
# ---- disallowed_methods, it does not raise its level, and the level is `clippy::all = deny` inherited
# ---- through this table (tests/fixtures/rk-clock-read/Cargo.toml:21-24 says so in terms).
test -f crates/rk-kernel/Cargo.toml || { echo "FAIL: crates/rk-kernel/Cargo.toml absent"; exit 1; }
grep -q '^\[lints\]' crates/rk-kernel/Cargo.toml || { echo "FAIL: no [lints] table in crates/rk-kernel/Cargo.toml — the workspace lint LEVELS do not reach the new member and the ban is warn-by-default (ADR-0102 decision 2)"; exit 1; }
# THE TABLE IS READ AS A TABLE, not as two independent greps. `^workspace = true$` asserted on its own is
# satisfied by a `workspace = true` under ANY stanza while [lints] is empty, and the FAIL message would then
# claim an exclusivity the pattern never tested (Halt 8's "assert what you actually establish").
body=$(awk '/^\[lints\]/{f=1;next} /^\[/{f=0} f' crates/rk-kernel/Cargo.toml | grep -vE '^[[:space:]]*(#|$)' | tr -d '\r' | tr -s ' ')
test "$body" = "workspace = true" || { echo "FAIL: rk-kernel's [lints] table body is not exactly 'workspace = true' (read: ${body:-<empty>}) — ADR-0102 decision 2"; exit 1; }
grep -q '^corpus = \[\]$' crates/rk-kernel/Cargo.toml || { echo "FAIL: rk-kernel declares no corpus feature — a --features corpus run naming this package exits 101 (rk-types/Cargo.toml:10-17). The feature stays EMPTY and gates code through #[cfg(feature = \"corpus\")], as rk-git does"; exit 1; }
# `name = "it"` is asserted INSIDE the [[test]] stanza, for the same reason: a bare `^name = "it"$` is
# satisfied by a `[package] name = "it"`, and the FAIL message names a stanza the pattern does not read.
awk '/^\[\[test\]\]/{f=1;next} /^\[/{f=0} f' crates/rk-kernel/Cargo.toml | grep -q '^name = "it"$' || { echo "FAIL: no [[test]] stanza declaring name = \"it\" — ADR-0083:74's one integration binary per crate, and what makes --test it addressable below"; exit 1; }
test -f crates/rk-kernel/tests/it/main.rs || { echo "FAIL: crates/rk-kernel/tests/it/main.rs absent (ADR-0083:74)"; exit 1; }
# COMMENT LINES ARE EXCLUDED: the house pattern is a commented manifest naming the governing record and the
# forbidden crate in prose (crates/rk-ports/Cargo.toml:25 says "no runtime for driving futures", and the
# unfiltered pattern exits 0 on it — measured 2026-09-13), and §7b.4 tells the implementer to write one here.
# Unfiltered, this line halts on a CORRECT manifest. NOTE THE ANCHOR: a grep over a SINGLE NAMED FILE prints
# `line:content` with no path prefix, so the filter is `^[0-9]+:` and not the two-colon recursive form. This
# sees DIRECT dependencies only; the transitive half is the resolved-graph assertion below.
raw=$(grep -nE 'tokio|async-trait|async-std|futures' crates/rk-kernel/Cargo.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: rk-kernel's manifest could not be read (grep rc=$rc; 2 = no such file) — a count of 0 here would be a fake 0; AC-528, ADR-0076"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[0-9]+:[[:space:]]*#')
[ -z "$hits" ] || { echo "FAIL: rk-kernel's manifest names an async runtime on a non-comment line — AC-528, ADR-0076, AC-484"; printf '%s\n' "$hits"; exit 1; }   # → silent
# POSITIVE CONTROL for the pattern itself, because the line above passes on absence: assert the SAME
# alternation still matches text of the shape it exists to find. `crates/rk-ports/Cargo.toml`:25 carries
# "no runtime for driving futures" in prose [measured 2026-09-13, 1 hit], so a pattern that silently stopped
# matching — a typo, an escaped `-`, a lost alternation branch — is caught here rather than read as a clean tree.
grep -qE 'tokio|async-trait|async-std|futures' crates/rk-ports/Cargo.toml || { echo "FAIL: the async alternation no longer matches crates/rk-ports/Cargo.toml, which names one in a comment — the ban above is vacuous and would stay silent over any manifest"; exit 1; }

# ---- AC-484 and AC-528's rk-kernel clause, over the RESOLVED graph. AC-484's register instrument is a
# ---- cargo metadata DEPENDENCY-GRAPH assertion; `--no-deps` omits the resolve graph and a discarded output
# ---- asserts nothing, which is what this line was until 2026-09-13. NOTE THE SCOPE: no rk-adapter-*
# ---- package exists until Step 7c, so the adapter count passes at 7b by the absence of the PACKAGES and
# ---- not of an EDGE — a standing floor that first becomes falsifiable at Checkpoint 7c, recorded as such
# ---- rather than read as discharging the criterion. THE PARENTHESISED PATH IS STRIPPED before matching the
# ---- async alternation: `cargo tree` renders every member as `<name> v<ver> (<checkout path>)`, so a
# ---- repository cloned under a path containing `futures` or `tokio` would FAIL a correct crate.
deps=$(cargo metadata --format-version 1 2>/dev/null) || { echo "FAIL: cargo metadata (WITH the resolve graph) did not run — AC-484's instrument is a dependency-GRAPH assertion"; exit 1; }
printf '%s\n' "$deps" | grep -q '"resolve"' || { echo "FAIL: cargo metadata returned no resolve graph — AC-484 cannot be asserted off --no-deps output"; exit 1; }
tree=$(cargo tree -p rk-kernel -e normal) || { echo "FAIL: cargo tree -p rk-kernel did not run"; exit 1; }
n=$(printf '%s\n' "$tree" | grep -c 'rk-adapter-' | tr -d '[:space:]')
[ "${n:-1}" -eq 0 ] || { echo "FAIL: rk-kernel depends on ${n} rk-adapter-* crate(s) — AC-484 forbids the edge, direct or transitive"; printf '%s\n' "$tree" | grep -n 'rk-adapter-'; exit 1; }   # → 0, by package absence at 7b; falsifiable from 7c
# THE HYPHEN FAMILIES ARE IN SCOPE, and the previous boundary excluded them. A ` `-or-EOL boundary after the
# crate name makes `futures-util`, `futures-core`, `futures-task`, `futures-channel`, `tokio-util` and
# `tokio-macros` INVISIBLE — and a transitive `futures-util` edge is the normal way the facade's machinery
# arrives. Measured 2026-09-13 against a synthetic cargo-tree fragment carrying futures-util, futures-core,
# tokio and async-trait: the old pipeline counted 2, the manifest grep's plain substring counted 4, so the
# two guards in this block disagreed about what AC-528 forbids and the weaker one read the transitive graph.
# Anchoring on cargo-tree's ` v<version>` render also makes the parenthetical strip unnecessary: a checkout
# path containing `futures` has no ` v` after it.
n=$(printf '%s\n' "$tree" | grep -cE '(^|[^a-z0-9_-])(tokio|async-std|async-trait|futures)(-[a-z0-9-]+)? v[0-9]' | tr -d '[:space:]')
[ "${n:-1}" -eq 0 ] || { echo "FAIL: ${n} async-runtime crate(s) in rk-kernel's RESOLVED normal graph — AC-528's rk-kernel clause forbids them directly or transitively, INCLUDING the hyphen families (futures-util, tokio-util, …), and the manifest grep above sees direct dependencies only"; printf '%s\n' "$tree" | grep -nE '(^|[^a-z0-9_-])(tokio|async-std|async-trait|futures)(-[a-z0-9-]+)? v[0-9]'; exit 1; }
# NO POSITIVE CONTROL IS AVAILABLE FOR THIS BAN AND THE GAP IS STATED, not implied. Nothing in this workspace
# resolves an async crate at 7b — `cargo tree -p rk-ports -e normal | grep -cE '(^| )(tokio|async-std|async-trait|futures)'`
# → 0 [measured 2026-09-13] — so there is no tree on this box in which the pattern MUST match. It first
# becomes controllable at Checkpoint 7c, where rk-adapter-claude resolves tokio; the control is owed there.
inv=$(cargo tree -i -p rk-adapter-sqlite 2>/dev/null); rc=$?
case "$rc" in 0) printf '%s\n' "$inv" | grep -q 'rk-kernel' && { echo "FAIL: cargo tree -i resolves rk-kernel in rk-adapter-sqlite's inverse graph — AC-005"; printf '%s\n' "$inv"; exit 1; } ;; *) echo "note: rk-adapter-sqlite is not in the workspace until 7c — AC-005's inverse assertion is ARMED, not discharged, and is re-run at Checkpoint 7c" ;; esac

# ---- the named tests, one per criterion, asserted BY NAME. cargo's `--` filter is a SUBSTRING match over
# ---- TEST names, so the aggregate `test result: ok. N passed` line is satisfied by a near-miss name and a
# ---- filter matching nothing exits 0 printing `0 passed` (s6:211-214, s7a:939, measured). Twelve groups in
# ---- this block carried the aggregate form until 2026-09-13; s7a:1173-1177 recorded the substitution as owed.
# ---- THESE LOOPS ARE DELIBERATELY NOT SCOPED `--test it`, and the scoping was a REGRESSION this repair
# ---- removed. `--test it` selects ONLY the integration binary, and this file assigns several of these names
# ---- to `src/`: §7b.4 puts AC-058/AC-063/AC-078's R-equality assertions in `src/select/` under
# ---- `#[cfg(test)]` (which covers `unsatisfiable_is_a_typed_gate` and `deviation_never_edits_the_required_set`)
# ---- and §7b.8 gives the rigor check as `cargo test -p rk-kernel -- rigor_monotonicity` with no `--test it`.
# ---- MEASURED on a probe crate 2026-09-13: a `src/`-resident `#[test] fn unsatisfiable_is_a_typed_gate` is
# ---- reported `0 passed; 1 filtered out` under `--test it` and the name grep does not match — a FAIL for a
# ---- target-selection reason unrelated to the rule. The unscoped `-p rk-kernel --` form reaches both
# ---- binaries; only the trybuild harness, whose file §7b.4 FIXES at `tests/it/ui.rs`, keeps `--test it`.
for t in grammar_agrees_with_the_s_grammar_probe grammar_precedence_and_associativity \
         grammar_bare_identifier_is_a_literal grammar_comparisons_do_not_chain \
         grammar_field_to_field_needs_a_ref on_false_closed_set predicate_null_arithmetic \
         quantifier_empty_and_null facet_registry_entry_shape facet_stability_five_clause \
         facet_provenance_fixtures rigor_monotonicity ac110_executed_ledger_first_and_last \
         ac527_empty_set_terminal_condition rigor_discouraged_refused ; do
  out=$(cargo test -p rk-kernel -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the -- filter is a SUBSTRING match, so an aggregate 'test result: ok' line is satisfied by a near-miss name and a filter matching nothing exits 0"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
# ---- the SELECTION ENGINE ITSELF, which no line above touches. AC-040-AC-068 plus AC-077, AC-078 and
# ---- AC-525 is this stage's headline deliverable and the subject of the C-02 pass §7b.9 records; every
# ---- other filter in this block is a lint, a graph assertion or the runtime.
for t in selection_seven_steps_are_pure rank_total_order_and_tie_is_an_error unsatisfiable_is_a_typed_gate \
         deviation_never_edits_the_required_set forbidden_stage_via_conditional_requires memo_key_eight_components \
         memo_activity_key_distinct memo_loop_iteration ac525_drift_advisory_once golden_log_replay \
         ac161_ac164_delete_db_replay ac484_adapter_edge_fixture ac487_kernel_denylist ac494_resolver \
         ac495_pack_validation ac498_no_egress_dispatch_zero ; do
  out=$(cargo test -p rk-kernel --features corpus -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the -- filter is a SUBSTRING match (s7a:939)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done
# ---- THE RUNTIME, which no line above touches either. AC-007-AC-012 and AC-014-AC-038 — the memo, the three
# ---- budget counters, retry, the breaker, degradation and exhaustion — were reached ONLY by the unfiltered
# ---- `cargo test -p rk-kernel` floor until 2026-09-13, and 00-preamble.md:186-189 says that floor is
# ---- "satisfied by an implementation nothing pins". These are §7b.13's Slice 4 names, one per
# ---- counter-independence pair and one per named class.
for t in budget_three_counters_never_share attempts_total_counts_a_pass_not_retries \
         for_each_charges_once_independent_of_n inner_retry_ac026 outer_retry_ac029 \
         never_retried_class_ac027 disjoint_channels_ac032 breaker_ac033_ac035 \
         degradation_ac036_ac038 exhaustion_ac024_ac025 ; do
  out=$(cargo test -p rk-kernel -- "$t" 2>&1); rc=$?
  [ "$rc" -eq 0 ] || { echo "FAIL: $t exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
  printf '%s\n' "$out" | grep -qE "^test .*\b${t}\b.* \.\.\. ok$" || { echo "FAIL: no test named $t ran and passed — the runtime range AC-007-AC-012 and AC-014-AC-038 is otherwise reached by nothing but the crate-wide floor"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
done

# ---- golden_log_replay's own clauses, which the name filter above does not reach. AC-006's Verified-by is
# ---- ports-bound-to-fakes AND byte-identical decisions AND zero rk-git writes (ADR-0105 decision 1 serves
# ---- the GitOps journal back); AC-008 adds zero harness dispatches AND a matched terminal state, without
# ---- which a kernel that does nothing at all passes.
# ---- NOTE THE ARGUMENT ORDER AND THE SINGLE `--`. `-- golden_log_replay -- --nocapture` is the doubled-`--`
# ---- defect the trybuild comment below forbids by name, in its most silent form: libtest reads the second
# ---- `--` as end-of-options, so `--nocapture` becomes a second free FILTER, capture stays ON, and the three
# ---- greps below fail on a CORRECT kernel. MEASURED on this tree 2026-09-13: `cargo test -p rk-ports
# ---- --test it -- conformance -- --list` RAN the test (ok. 1 passed) instead of listing it, while
# ---- `-- --list conformance` listed it. THE FLAG GOES BEFORE THE FILTER.
out=$(cargo test -p rk-kernel --features corpus -- --nocapture golden_log_replay 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: golden_log_replay exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bgolden_log_replay\b.* \.\.\. ok$' || { echo "FAIL: no test named golden_log_replay ran and passed — it is behind --features corpus (it reads \$CORPUS_REPO), and a filter matching nothing exits 0 printing '0 passed'"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# ---- THE THREE TOKENS BELOW ARE SELF-REPORTED AND THE TRADE IS STATED RATHER THAN IMPLIED (the form
# ---- §7b-ii.13 uses for the ADR-0101 guard). `rk_git_writes=0`, `harness_dispatches=0` and
# ---- `terminal_state=matched` are strings the target prints ABOUT ITSELF: a kernel that counts nothing and
# ---- prints the three literals passes all three, and §7b.14's pairing rule is met only circularly, because
# ---- the behavioural pair IS this test. What makes them non-vacuous is the FOURTH line: a deliberately-red
# ---- control run over a fake that performs one rk-git write, which must flip rk_git_writes off 0. Without
# ---- that control these are three greps over the test's own println!.
printf '%s\n' "$out" | grep -qE 'rk_git_writes[= ]0' || { echo "FAIL: golden_log_replay does not report zero rk-git writes — AC-006's Verified-by requires it and ADR-0105 decision 1 is what makes it true"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'harness_dispatches[= ]0' || { echo "FAIL: golden_log_replay does not report zero harness dispatches — AC-008"; exit 1; }
printf '%s\n' "$out" | grep -qE 'terminal_state[= ]matched' || { echo "FAIL: the replay does not assert the recorded terminal state — a kernel that does nothing at all passes without this half (AC-008)"; exit 1; }
out=$(cargo test -p rk-kernel --features corpus -- golden_log_replay_writing_fake_is_red 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: golden_log_replay_writing_fake_is_red exited $rc — it is the POSITIVE CONTROL for the three self-reported counters above, replaying against a GitOps fake that performs one write and asserting the replay refuses"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\bgolden_log_replay_writing_fake_is_red\b.* \.\.\. ok$' || { echo "FAIL: no test named golden_log_replay_writing_fake_is_red ran and passed — without it the three counter greps above are satisfied by a kernel that prints the literals and counts nothing"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
test -n "$(find "$CORPUS_REPO/golden-logs" -type f -print -quit 2>/dev/null)" || { echo "FAIL: the golden event logs are not in the pinned corpus under golden-logs/ — AC-006/AC-188 as amended 2026-09-13, and §7b.11's single CORPUS_PIN move is what puts them there"; exit 1; }
test -s "$CORPUS_REPO/results/p1-arm-d-panel-scored.json" || { echo "FAIL: AC-520's scored precision record is not in the pinned corpus — every deriver_record in spikes/p9/facets.toml names it and the provenance lint resolves it through CORPUS_PIN (AC-464)"; exit 1; }
# ANCHORED ON THE DECLARATION, NOT ON THE TOKEN. Three of the four occurrences of `--out` in that file are
# PROSE IN ITS MODULE DOCSTRING (:28, :30, :32) and only :97 is the `add_argument` that makes the flag real
# [measured 2026-09-13], so the unanchored form is satisfied by a comment quoting the literal it searches for
# — the exact shape §7b.14's pairing rule forbids. Deleting the argparse line left the old line green.
grep -qE '^[[:space:]]*ap\.add_argument\("--out"' spikes/p1/score-arm-d-panel.py || { echo "FAIL: the scorer DECLARES no --out argument — AC-520's record cannot be written (PROGRESS.md row L2); a docstring naming the flag is not the flag"; exit 1; }

# ---- the trybuild harness, filtered on the HARNESS TEST'S OWN NAME and scoped with --test it, which is
# ---- correct HERE because §7b.4 FIXES this harness's file at `crates/rk-kernel/tests/it/ui.rs`. THE
# ---- HARNESS NAME IS WHAT MAKES THE FILTER DISCRIMINATING, NOT THE ABSENCE OF THE SUBSTRING: this crate's
# ---- harness is `trybuild_pins_the_reason_each_kernel_fixture_fails`, which CONTAINS `trybuild`, so
# ---- `s8`:121's `cargo test -p rk-kernel -- trybuild` DOES select it — measured on this tree 2026-09-13,
# ---- `cargo test -p rk-ports --test it -- trybuild` selects `ui::trybuild_pins_the_reason_each_port_fixture_fails`
# ---- and reports `ok. 1 passed`. What Step 8 check (5) therefore CANNOT see is WHICH cases the harness ran:
# ---- it is green whenever any one of them passes. The six by-path assertions below are what pin the
# ---- `--stay` case at this stage, and **a matching by-path assertion in `s8` check (5) is OWED** (§7b.15).
# ---- AND NEVER TWO FREE FILTERS AFTER A DOUBLED `--`: libtest OR-unions them, so
# ---- `-- trybuild -- ac294_block_constructor` passes whenever ANY test whose name contains `trybuild`
# ---- passes. MEASURED on this tree 2026-09-13: `cargo test -p rk-ports --test it -- trybuild --
# ---- no_such_test_xyz` reported `ok. 1 passed` at rc=0 against a test name that does not exist.
out=$(cargo test -p rk-kernel --test it -- trybuild_pins_the_reason_each_kernel_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the trybuild must-fail suite did not pass (rc=$rc) — a fixture COMPILED, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test of that name ran and passed — a filter matching nothing exits 0 (00-preamble.md:178)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
for f in stay_class_state_is_unconstructible ac480_generics_leak ac050_rigor_is_not_arithmetic \
         ac041_selection_reads_no_clock ac061_required_set_is_not_constructible_outside_select \
         ac061_required_set_has_no_deserialize ; do
  test -f "crates/rk-kernel/tests/ui/$f.rs" || { echo "FAIL: the trybuild case $f.rs is absent — a harness that runs proves nothing about a case that is not there, and Phase-1 exit check (5)'s filter is satisfied by any other case"; exit 1; }
  test -s "crates/rk-kernel/tests/ui/$f.stderr" || { echo "FAIL: $f commits no .stderr, so it is green whenever compilation fails for ANY reason (s7a §7a.8)"; exit 1; }
done

# ---- the three lint rules, ANCHORED ON THE DECLARATION, one per rule. A count of LINES containing `fn ` is
# ---- a PROXY: two helpers of ONE rule satisfies a `-ge 2` floor while another rule is absent, and a doc
# ---- comment naming either token counts. The property is THREE RULES WITH THREE FIXTURE SETS — AC-071
# ---- (stability), AC-520 (provenance) and AC-072 (the registry-entry loader), which the register forbids
# ---- collapsing into one lint rule — so each is asserted by name and each fixture set by its own files.
# THE VISIBILITY SPELLING IS NOT THE RULE. `(pub )?` admits exactly two forms and misses every restricted
# one — `pub(crate) fn`, `pub(super) fn`, `pub(in crate::lint) fn` — and this same file PRESCRIBES restricted
# visibility as the house idiom (§7b.4 fixes `RequiredSet`'s constructor at `pub(in crate::select)`), so a
# correct implementation writing `pub(crate) fn facet_stability_rule(` failed all three lines for a
# visibility-spelling reason. The `[<(]` tail admits a generic declaration too.
DECL='^[[:space:]]*(pub(\([^)]*\))?[[:space:]]+)?(async[[:space:]]+)?fn[[:space:]]+'
grep -rqE "${DECL}[a-z_]*facet_stability[a-z_]*[<(]" crates/rk-kernel/src/ || { echo "FAIL: no facet_stability lint rule is DECLARED in rk-kernel — AC-071"; exit 1; }
grep -rqE "${DECL}[a-z_]*facet_provenance[a-z_]*[<(]" crates/rk-kernel/src/ || { echo "FAIL: no facet_provenance lint rule is DECLARED in rk-kernel — AC-520 is a SECOND rule, never a sixth clause on AC-071"; exit 1; }
grep -rqE "${DECL}[a-z_]*facet_registry[a-z_]*[<(]" crates/rk-kernel/src/ || { echo "FAIL: no facet_registry loader rule is DECLARED in rk-kernel — AC-072 is a THIRD rule and the register forbids collapsing it into AC-520"; exit 1; }
for f in no_record identity_below_bar n_below_twenty draws_below_ten panel_not_scored corpus_not_pinned ; do
  test -f "crates/rk-kernel/tests/fixtures/registries/facet-stability/$f.toml" || { echo "FAIL: AC-071's $f fixture is absent — its Verified-by is a no-record fixture PLUS five one-clause cases, and the five-case set stays at five even though arm D's clause is discharged"; exit 1; }
done
# BY NAME, NOT BY COUNT — §7b.7 forbids the count in terms ("each fixture by name, not by a count") and
# E-0520-1's rule is "read the enumeration, not the number". The previous `ls … | wc -l  -ge 11` line was
# satisfied by eleven copies of `positive_companion.toml`, and by any eleven files with all ten
# discriminating cases absent. The ten names are §7b.12's, taken from the register's enumeration.
for f in no_deriver deriver_without_record record_zero_positives_keys_require lexical_keys_require \
         no_deriver_class deriver_class_not_a_token no_deriver_record deriver_record_absent_at_pin \
         kernel_facet_carries_deriver require_keyed_on_declared positive_companion ; do
  test -f "crates/rk-kernel/tests/fixtures/registries/facet-provenance/$f.toml" || { echo "FAIL: AC-520's $f fixture is absent — its Verified-by enumerates eight clause groups over ten must-fail files PLUS the positive companion, without which the criterion is satisfied by a lint that rejects everything; read the enumeration, not the headline count, which is stale (ERRATA E-0520-1)"; exit 1; }
done

# ---- golden constraint 1 as a command. THE PATTERN IS THE WHOLE ASSERTION, and the previous one could not
# ---- match a true instance in either direction: a live require row is an array-of-tables HEADER whose
# ---- predicate is on the NEXT line (spikes/p9/packs/bug-standard.toml:63-64 is `[[stage.review.require]]`
# ---- then `when = "criticality >= high"`), and every catalog predicate is spelled `applicable_if`, never
# ---- `require` [measured 2026-09-13]. So the pattern keys on the PREDICATE keyword. Comment lines are
# ---- excluded rather than halted on: the house pattern is a file whose header quotes the governing rule in
# ---- prose (spikes/p9/catalog/{change-authorization,blameless-postmortem,integration-tests}.toml each carry
# ---- a line-1 `# reach: … require row`, and a catalog fixture copied under crates/ brings it).
raw=$(grep -rnE '(require|requires|applicable_if|when)[[:space:]]*=.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' crates/ --include=*.toml --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LLM-ordinal search could not run (grep rc=$rc; 2 = no such path) — golden constraint 1, as a command"; exit 1; }
# THE FIXTURE TREE §7b.12 PRESCRIBES IS EXCLUDED BY PATH, AND THE REASON IS THE GUARD'S SCOPE, NOT
# CONVENIENCE (golden constraint 9, Halt 7). §7b.8 and §7b.12 REQUIRE a copy of
# spikes/p9/catalog/bisect.toml under crates/rk-kernel/tests/fixtures/catalog/ — minus its max_rigor
# line and NOTHING else — and its line 12 IS `applicable_if = "has_repro"`, one of the 103 the
# control below counts. Keeping that line is what makes `bisect` selectable at `mandatory` with
# `has_repro`, which IS F37's defect and the whole of the red end; §7b.12 forbids editing spikes/p9
# to make a red case, so the fixture cannot lose it either. An unfiltered ban and §7b.12 are
# JOINTLY UNSATISFIABLE and this line was RED on a correct tree [measured 2026-09-13: exactly one
# hit, crates/rk-kernel/tests/fixtures/catalog/bisect.toml:12]. The exclusion is ONE path prefix; a
# hit anywhere else under crates/ still halts.
#
# AND THE PATTERN ITSELF IS TOO WIDE BY ONE KEYWORD. design §10.2 principle 3 scopes the seven-facet
# prohibition to "authoring a `require` row keyed on criticality, novelty, size, blast_radius,
# market_facing, external_dep or has_repro SOURCED FROM AN LLM CLASSIFICATION DISPATCH".
# `applicable_if` is NOT in that clause — it is in the SECOND clause, over the four lexical-deriver
# facets, which this block omitted entirely and which is now asserted below. It is left in the
# alternation here deliberately, as a wider-than-the-record floor over crates/ that costs nothing
# after the exclusion: after Step 4a NO facet is `source = "llm"` (0 hits for `source = "llm"` in
# spikes/p9/facets.toml) and `has_repro` is `source = "human"` (facets.toml:149; ADR-0090 decision
# 2), so no hit this pattern can produce over crates/ is a true instance of clause 1 today.
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|#)' | grep -vE '^crates/rk-kernel/tests/fixtures/')
[ -z "$hits" ] || { echo "FAIL: an LLM-ordinal predicate exists on a non-comment line outside the prescribed fixture tree — golden constraint 1, ADR-0090 clauses 1 and 2, AC-071, design §10.2's 'may not' column. READ each hit (golden constraint 9); do not silence the grep"; printf '%s\n' "$hits"; exit 1; }   # → silent
# POSITIVE CONTROL, because the line above passes on ABSENCE and a pattern that silently stopped matching
# would read as a clean tree. The corpus is the control: the SAME alternation over spikes/p9/ returns 103
# hits today [measured 2026-09-13] — e.g. catalog/bisect.toml:12 `applicable_if = "has_repro"` and
# catalog/architecture-tradeoff-analysis.toml:11 `applicable_if = "criticality >= high"`. If this goes to
# zero the pattern broke, not the tree. It is asserted as >= 1, never as 103: the corpus is s7e's to repair.
c=$(grep -rcE '(require|requires|applicable_if|when)[[:space:]]*=.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' spikes/p9/ --include=*.toml 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
[ "${c:-0}" -ge 1 ] || { echo "FAIL: the LLM-ordinal pattern matches NOTHING in spikes/p9/, where it matched 103 lines on 2026-09-13 — the pattern is broken, and the ban above is vacuous rather than satisfied"; exit 1; }   # → 103
# THE SECOND PROHIBITION, WHICH THIS BLOCK OMITTED ENTIRELY UNTIL 2026-09-13. design §10.2 principle
# 3 forbids, SEPARATELY, "authoring one — `require` row or `applicable_if` alike — keyed on `auth`,
# `ui`, `flag_exists` or `data_migration` as computed by the lexical deriver, until S-FACET reports"
# (ADR-0089; ADR-0093 scopes S-FACET to exactly those rows and `surfaces`). `applicable_if` IS in
# scope here and is not in the record's clause above — the block had it exactly the wrong way round
# in both directions. Same fixture-tree exclusion, same reason: AC-520's own
# registries/facet-provenance/lexical_keys_require.toml MODELS a forbidden require row on purpose,
# which is the clause-4 fixture, and its single in-crate hit is that file's `when = "ui"`.
raw=$(grep -rnE '(require|requires|applicable_if|when)[[:space:]]*=.*\b(auth|ui|flag_exists|data_migration)\b' crates/ --include=*.toml --include=*.rs); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the lexical-deriver search could not run (grep rc=$rc; 2 = no such path) — design §10.2 principle 3's SECOND clause, as a command"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|#)' | grep -vE '^crates/rk-kernel/tests/fixtures/')
[ -z "$hits" ] || { echo "FAIL: a predicate keyed on a lexical-deriver facet exists on a non-comment line outside the prescribed fixture tree — design §10.2 principle 3's SECOND clause, ADR-0089, ADR-0093; S-FACET is UNRUN and auth scores 0/12 [0.00, 0.24], every positive false. READ each hit (golden constraint 9); do not silence the grep"; printf '%s\n' "$hits"; exit 1; }   # → silent
# ITS OWN POSITIVE CONTROL, for the same reason the LLM-ordinal ban has one: the line above passes
# on ABSENCE, so a pattern that silently stopped matching would read as a clean tree.
c=$(grep -rcE '(require|requires|applicable_if|when)[[:space:]]*=.*\b(auth|ui|flag_exists|data_migration)\b' spikes/p9/ --include=*.toml 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
[ "${c:-0}" -ge 1 ] || { echo "FAIL: the lexical-deriver pattern matches NOTHING in spikes/p9/, where it matched 44 lines on 2026-09-13 — the pattern is broken, and the ban above is vacuous"; exit 1; }   # → 44
# WIDENED to leave[sd]?, because METHOD-P:150 and ADR-0079:240 both say "leave the model" and s8:275-276
# already carries the wider form. Comment lines are NOT excluded here, deliberately: the prohibition is about
# what a document SAYS, so a doc comment quoting METHOD-P §1.4 as licence is exactly the hit this exists to catch.
hits=$(grep -rniE 'leave[sd]? the model' crates/ --include=*.rs --include=*.toml --include=*.md); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the leave-the-model search could not run (grep rc=$rc; 2 = no such path)"; exit 1; }
[ -z "$hits" ] || { echo "FAIL: 'leave(s/d) the model' appears in crates/ — NO FACET DOES: no facet clears METHOD-P §1.4's 0.95 bar and the bar is unreachable at n=20 for any facet [measured, F39 Limits 1]"; printf '%s\n' "$hits"; exit 1; }   # → silent
# POSITIVE CONTROL, same reason: the SAME pattern over references/ returns 14 hits today [measured
# 2026-09-13] — the process-catalog proposal's P1 row and the register's own "Nothing in this register may
# now say a facet *leaves the model*" are two of them. A zero here means the pattern broke.
c=$(grep -rcniE 'leave[sd]? the model' references/ 2>/dev/null | awk -F: '{s+=$2} END{print s+0}')
[ "${c:-0}" -ge 1 ] || { echo "FAIL: the leave-the-model pattern matches NOTHING in references/, where it matched 14 lines on 2026-09-13 — the pattern is broken, and the ban above is vacuous"; exit 1; }   # → 14

# ---- the grammar probe, BOTH halves read and pinned. The probe reports rc=1 for ANY non-empty failure set
# ---- (grammar-probe.py:253), so rc alone distinguishes nothing; and `[1-9][0-9]*/[0-9]+` accepts 1/287.
# ---- THE THREE NUMBERS BELOW ARE VALID UNTIL s7e §7e.2 REPAIRS THE FIVE PACK DEFECTS, which is downstream
# ---- of this stage: s7e:117 expects 287/287 afterwards. If any of them moves, RE-DERIVE from F42 and record
# ---- the new reading — do not widen the pattern (golden constraint 9).
test -f spikes/p9/grammar-probe.py || { echo "FAIL: spikes/p9/grammar-probe.py absent"; exit 1; }
out=$(python spikes/p9/grammar-probe.py); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: grammar-probe did not run to completion (exit $rc)"; exit 1; }   # 0 = clean, 1 = the known pack defects (F42)
printf '%s\n' "$out" | grep -qxF 'predicates parsed 285/287; failed 2/287' || { echo "FAIL: the probe's parse count moved off F42's 285/287 — read the FAIL lines before touching the grammar; the two known failures are exists(ProblemRecord) in packs/incident.toml and a bare hyphenated value in catalog-b"; printf '%s\n' "$out" | grep -E 'FAIL|ON_FALSE-FAIL'; exit 1; }
printf '%s\n' "$out" | grep -qxF 'mixed unparenthesised and/or (precedence-dependent) 0/287' || { echo "FAIL: a precedence-dependent predicate now exists — §7b.6 claims 0 and the kernel parser's precedence rule has a live instance to disagree with"; exit 1; }
printf '%s\n' "$out" | grep -qE '^on_false forms 96/99 in the closed set' || { echo "FAIL: the on_false count moved off F42's 96/99 — ADR-0103 clause 6's closed set is the SECOND rule the lint carries and the three known catalog-b defects are its only live instances"; exit 1; }

# ---- AC-104's per-(family, rigor) totality check, at lattice.py's OWN output strings. This file claimed
# ---- "Checkpoint 7b runs it" of this instrument and of spikes/p9/lint.py and ran NEITHER until 2026-09-13.
# ---- spikes/p9/lint.py is still NOT run here and must not be: it is red BY DESIGN from Step 4a until s7e
# ---- §7e.2 disposes of the seven `auth` sites (errors=3 warnings=2, rc=1, measured 2026-09-13), and
# ---- silencing the three by declaring `auth` a surfaces value is 0 errors with it and 3 without (lint.py:339).
test -f spikes/p9/lattice.py || { echo "FAIL: spikes/p9/lattice.py absent — AC-104's instrument"; exit 1; }
out=$(python spikes/p9/lattice.py --validate 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: lattice.py --validate exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -q 'validate errors=0' || { echo "FAIL: lattice.py --validate did not print 'validate errors=0' — AC-104's per-(family, rigor) totality check is not green"; printf '%s\n' "$out" | tail -20; exit 1; }   # → validate errors=0 [measured 2026-09-13]
hits=$(printf '%s\n' "$out" | grep -E 'holes=[1-9]')
[ -z "$hits" ] || { echo "FAIL: lattice.py --validate reports a family hole — totality (AC-104) and monotonicity (AC-105) are separate obligations on the same enumeration and both are errors, not warnings (ADR-0081 clause 2)"; printf '%s\n' "$hits"; exit 1; }   # → silent
# POSITIVE CONTROL for the line above, which passes on absence: the output must still CARRY `holes=` keys at
# all. If lattice.py renames the key the ban goes permanently silent and reads as a hole-free lattice.
c=$(printf '%s\n' "$out" | grep -c 'holes=' | tr -d '[:space:]')
[ "${c:-0}" -ge 1 ] || { echo "FAIL: lattice.py --validate printed no 'holes=' key at all (it printed 15 on 2026-09-13) — the hole ban above is searching for a string this tool no longer emits"; exit 1; }   # → 15
# ALIGNMENT-TOLERANT AND ANCHORED. `bisect.toml`:16 is column-aligned — `max_rigor    = "recommended"`, four
# spaces — so the single-space pattern returns rc=1 on a CORRECT and UNCHANGED tree [measured 2026-09-13],
# which is the identical defect PROGRESS.md:325 already records against Checkpoint 6's `gix = "=0.87.1"`
# line. The guard's scope is "exactly one ceiling declaration"; alignment is incidental, so the pattern moves
# and the manifest is left alone (golden constraint 9).
grep -qE '^max_rigor[[:space:]]*=[[:space:]]*"recommended"' spikes/p9/catalog/bisect.toml || { echo "FAIL: bisect is no longer ceilinged — P7-CEIL (ADR-0093 decision 5; DECISIONS-2026-09-11 #1) is what makes rigor_monotonicity green on the shipped catalog (F37)"; exit 1; }
# ANCHORED ON A NON-COMMENT LINE, the form used for the manifest and LLM-ordinal bans above: a doc comment
# naming the path satisfies a bare `grep -rq`, and §7b.12 puts an AUTHORED pre-P7-CEIL copy of the catalog
# under crates/rk-kernel/tests/fixtures/catalog/, so a test file naming BOTH paths in prose is the expected
# shape. Its behavioural pair is `rigor_monotonicity` in the loop above, which is green on the shipped catalog.
raw=$(grep -rn 'spikes/p9/catalog' crates/rk-kernel/tests/); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the shipped-catalog search could not run (grep rc=$rc; 2 = no such path)"; exit 1; }
hits=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*|#)')
[ -n "$hits" ] || { echo "FAIL: no rk-kernel test reads the SHIPPED catalog on a non-comment line — 'passes on the catalog as shipped' is otherwise asserted over a fixture of the implementer's choosing, and a doc comment naming the path satisfies the unanchored form"; exit 1; }
```

### 7b.15 Acceptance set

The set is written as explicit ranges **minus** the ids excluded, each named with its reason (`00-preamble.md`:229-231;
PIA-048/PIA-070/PIA-071). **It claims by id ONLY what a discriminating test at this stage can pin**; everything else
is **ARMED, NOT DISCHARGED**, by id and reason, because golden constraint 10's form is that powerless is reported as
powerless and never as a pass.

**At 7b, `crates/` holds `rk-git`, `rk-ports`, `rk-store`, `rk-types` and the `rk-kernel` this step writes** [measured
2026-09-13]. **There is no `rk-adapter-*`, no `rk-cli` and no `doctor`; `.redkiln/` does not exist; no lane has run
and no corpus run is recorded.**

**Claimed ranges:**

- **AC-001–AC-039** (kernel; AC-006/AC-188's golden-log replay included — the logs are produced here, handed forward
  by `s5`:459 and settled in the register on 2026-09-13) **minus AC-013**, PROVISIONAL on P8 (P8 is PARTIAL — 3 of a
  pre-registered 27+3 runs and its recorded state held exactly one step), so the kill-and-resume test is **built, not
  claimed** — Step 6 said the same of the same id — and **minus AC-039**, **human-gated** and not machine-decidable;
  only the mechanism half is asserted here, never the answer. **AC-001–AC-004 are claimed for the
  `rk-kernel::workflow` half only**: the module-level `#[deny]` and the workspace invocation gated on its exit code;
  the `clippy.toml` instrument and both fixture crates are Step 5's and are **re-run, not re-claimed**.
- **AC-040–AC-079** (selection) **minus AC-075** (cut, never reuse), **minus AC-044** (armed — §7b.9(8)) — and
  **building AC-069, AC-070, AC-072 and AC-079** as binding (the registry after Step 4a has zero `llm` facets, six
  `derived`, four `kernel`, one `declared`), with **AC-076** human-gated and **AC-068** PROVISIONAL (P2 — the
  mechanism is binding, the *value* is provisional; the unblocking precondition was inverted on 2026-09-11 to the
  decision rule of DECISIONS-2026-09-11 #4, which AC-068's own text carries, and **ADR-0117, accepted 2026-09-13, is
  what that PROVISIONAL label now points at**: its clause 4 makes **P2-B** the only instrument that may change it, and
  its clause 3 fixes the fault names — criterion 1's recall half NOT ANSWERABLE, criterion 2 NOT ANSWERABLE, criterion
  3 UNRUN, reported by those names and never as a pass, a failure, `0.00`, `1.00` or `n/a`).
- **AC-080–AC-118** (packs and catalog lint; **AC-119** is unallocated) — **the lint-and-loader half only: the rules,
  their fixtures and their failure messages, over the fixture packs of §7b.12. The authoring half is `s7e`'s**, which
  claims the same range on the shipped two-pack set, and the two are **cross-listed, not duplicated**: a criterion is
  credited here when the rule exists in `rk-kernel` and fails closed on a fixture, and at 7e when the shipped pack
  satisfies it. **This site notes `s7e`'s acceptance set; a matching note in `s7e` is OWED**, because the register's
  §0.4 says a shared claim conforms only when *both* sites note the other, and `s7e`'s set claims AC-080–AC-118 flat
  with no cross-list sentence [measured 2026-09-13]. `s7e` is not edited in this pass. **Excluding AC-101/AC-102**, whose lattice re-run baseline is Step 7e's by DECISIONS-2026-09-11 #1;
  **minus AC-113** (PROVISIONAL/BLOCKED on P1's identity clause, P2 and S-FACET — the fixed-vector
  equality-and-non-emptiness property test only) and **AC-114/AC-115** (**human-gated**; only the machine-checkable
  residue is asserted — that the decision record exists and is attributed). **Excluded here by id and reason:
  AC-092–AC-099 and AC-118** — their instrument is `redkiln doctor`, `rk-doctor` is `rk-cli` and `rk-cli` is Step 7d;
  their pack-side preconditions are built here and the rows are discharged at **Checkpoint 7e**, which runs `doctor`
  behind a `test -x` guard.
- **AC-520** — the derived-facet provenance gate, allocated outside the selection range because `AC-040..AC-079` is
  exhausted. Its **kernel-side half is discharged here**; its **corpus-resolution half is armed** until §7b.11's pin
  move, and its **CLI-surface half is 7d/7e's**.
- **AC-525** (the vouched-root advisory note — exactly one `drift_advisory` per changed upstream digest, carried on
  the *next* gate the item reaches; ADR-0025/0024, and SM-022 has no baseline, so **report the noise rate, do not
  threshold it**), **AC-527** (the `rk-catalog` lint that refuses `findings.none(evidence in [Reproduced,
  MutationSurvived])` **or any predicate whose truth over an empty findings set is `true`** as a quality block's
  terminal condition — the rule is on the FORM, not the string: ADR-0080's forbids clause reads *"or any policy
  predicate satisfiable only by an empty finding set"*, and its *Verified by* is **three** fixtures, not one) and
  **AC-530 is NOT claimed here** — it is `s7b-ii`'s, with the advisory sink whose exit invariant it owns.
- **AC-161 and AC-164's next-action halves**, handed forward by Step 6: their instrument is *"the delete-the-database
  replay test in `rk-kernel`"*, which does not exist until this step. **AC-184's projection half is not claimed** —
  it waits on `rk-cli`'s `doctor --rebuild` at 7d.
- **The §14 criteria handed here by `s5`'s exclusion list and `s7a` §7a.11's hand-forward table, claimed by id:**
  **AC-484** (the `cargo metadata` transitive-graph assertion plus the `rk-adapter-sqlite` fixture commit);
  **AC-487** (the kernel lint over the adapter-crate / `downcast_ref` / `downcast_mut` denylist plus a compile-fail
  fixture — **this id is claimed by no other stage**); **AC-494** (the resolver unit test); **AC-495** (pack
  validation over two fixture packs); **AC-498** (the network-egress-denied job with no adapter crates linked and a
  dispatch counter asserted at 0). **And these halves of ids discharged in part at 7a:** **AC-480**'s first THEN
  clause and its API-surface lint over `rk-kernel`, with the compile-fail fixture that introduces one such parameter;
  **AC-483**'s *"`rk-kernel` depends on `rk-git` as a concrete crate"*; **AC-497**'s input-assembly test;
  **AC-528**'s `rk-kernel` clause, asserted over the **resolved** graph and not over the manifest, which sees direct
  dependencies only. **AC-500 is ARMED, not claimed**: its route-around path and mutation-style check are 7b/7e's and
  both need a pack — the path is built here and the pack half is 7e's.
- **AC-005's `cargo tree -i` half** — `PROGRESS.md`:28 hands it to "7a/7c" and 7a could not run it (`cargo tree -p
  rk-kernel` exited 101 with no `rk-kernel` in the workspace). 7b is the first stage at which `rk-kernel` resolves,
  but `rk-adapter-sqlite` does not exist until 7c, so the inverse assertion is **ARMED, re-run at Checkpoint 7c**;
  the `deny.toml` `[bans].deny` half runs here in the five-gate block.

**THE RUNTIME RANGE IS NOW PINNED BY NAME, AND IT WAS NOT.** Until 2026-09-13 **AC-007–AC-012 and AC-014–AC-038**
— the memo, the three budget counters, retry, the breaker, degradation and exhaustion — were reached only by the
**unfiltered** `cargo test -p rk-kernel` floor, which `00-preamble.md`:186-189 says is *"satisfied by an
implementation nothing pins"*: the tokens `budget`, `retry`, `breaker` and `degrad` appeared in **no** filter in the
block. §7b.13's Slice 4 now names ten tests — one per counter-independence pair and one per named class — and
§7b.14's **third loop asserts all ten by name**. They are not ARMED, because nothing external blocked them: the code
and the tests are this stage's, and what was missing was the lines.

**ARMED, NOT DISCHARGED — by id and by reason.** Checkpoint 7b may not be read as discharging any of these, and each
is recorded in `PROGRESS.md` §1 as carried forward:

| id | register instrument | missing at 7b | where it can run |
| --- | --- | --- | --- |
| **AC-044** | *"a workspace dependency test asserting the selection crate's transitive deps exclude the harness crate"* | the THEN is unsatisfiable at crate scope (§7b.9(8)); **a dated amendment is OWED** | here, once the amendment lands |
| **AC-045** | the four-digest key's table-driven test and its struct-level test | the key is **ADR-0119's fourth key** and the register still names none; **an amendment is OWED** (ADR-0119 clause 8) | here, once the amendment lands |
| **AC-043** | the poisoned-run-id unit test plus the `memo_hit`/`memo_evicted` event assertion | its **third state** (replay-not-yet-asserted) and its second refusal reason; **an amendment is OWED** | here, once the amendment lands |
| **AC-064** | the delta-event unit test | **`dag_amended` is not a kind** — ADR-0116 forbids minting it outside a record; **an ADR is OWED** (§7b.2(1)) | Slice 3, once the record lands |
| **AC-056** | the lattice lint plus a two-catalog regression fixture pinned at 23.5% and 0.3% | the pack set and rigor mode are not part of the assertion, and **the F35 pre-decision catalog has no named path**; an amendment is OWED | here, once both land |
| **AC-005** *(inverse)* | `cargo tree -i` returning empty | `rk-adapter-sqlite` | **7c** |
| **AC-092–AC-099, AC-110's doctor half, AC-118** | a `doctor` rule-table row | `rk-cli` / `doctor` | **7d**, discharged at Checkpoint 7e |
| **AC-520** *(corpus half)*, **AC-071** *(record half)* | the record *"exists, is pinned and is read"* | the pin has not moved (§7b.11); AC-071's record has no producer and no lawful path | the single pin move at the end of Step 7b; AC-071 additionally needs its amendment |
| **AC-042** *(cross-process half)* | `redkiln replay --run <id>` | `rk-cli` | **7d** |
| **AC-500** *(route-around)*, **AC-069/AC-070** *(shipped-pack halves)* | a pack | no pack is authored here | **7e** |
| **AC-013** | the kill-and-resume test | P8 is PARTIAL | built here; **not counted as covered** |

**RAISED, NOT SETTLED** (`00-preamble.md`:55; `s7a`:1217 — *"naming a destination is not routing it there"*):

- **AC-239's allocation.** The register assigns its **list-is-data half** to *"the runbook's 7b/7d acceptance sets
  (PIA-072)"* and `s7a`:1199-1206 handed the correction here. It is **claimed at Step 7b-ii** (`s7b-ii` §7b-ii.14),
  because the gate-eligible adapter list is a gate property; `s7c`:193 also holds the id by a bare range sweep with no
  reason stated, and **an erratum aligning the two sets, or a reason-bearing note in `s7c`, is OWED**.
- **AC-291/AC-292 are ORPHANED** and are raised in `s7b-ii` §7b-ii.14, which states the two lawful routes. They are
  not claimed here: their subject is `rk-git` and their instruments are structural assertions over a spawn
  configuration and a clone shape.
- **The `--stay` fixture has no criterion.** Phase-1 exit check (5) gates on it and no criterion in the register
  states it (searched `--stay`, `stay`, `unrepresentable`: the only hits are AC-446's corpus list and ADR-0107's
  denylist). Either a criterion is allocated for it, outside an exhausted range per the register's §0.4, with its own
  GIVEN/WHEN/THEN naming which constructor must not exist, **or exit check (5) is withdrawn and its obligation folded
  into AC-294**. **And a second, separable half is owed at `s8` itself:** `s8`:121's `cargo test -p rk-kernel --
  trybuild` **does** select this crate's harness (§7b.4 corrects the claim that it does not), but it is green
  whenever **any** case in the harness passes, so an absent `stay_class_state_is_unconstructible.rs` reads exactly
  like a present one. **Owed: add the by-path assertion Checkpoint 7b carries — the `.rs` and its committed
  `.stderr` — to `s8` check (5).** `s8` is not edited in this pass. **Record the missing id as a residual in
  `PROGRESS.md` §2 against Step 8 check (5)**, and do not decide here which type carries the ban.
- **Errata owed above this file, each named in place:** E-0079-2 (AC-071's THEN and `00-preamble.md`'s golden
  constraint 1 still call ADR-0079 `proposed`); E-0071-1 (AC-071's dead `--stability-record` path, plus the
  non-reproducibility of §7b.2); E-0105-1 (AC-105's THEN is narrower than ADR-0081 clause 1); the AC-520 self-
  contradiction of §7b.7; AC-043's third state, AC-045's key name and design §4.4's *"The memo key"* (ADR-0119 clause
  8); AC-056's series; AC-066's 8-case table; AC-525's run scope; F42's *"the design's grammar section records both"*;
  **ADR-0103's two missing productions** — `string in <set-valued facet>` and the refseg hyphen (§7b.6), which the
  kernel's parser implements and no record admits; **ADR-0081 clause 2's instrument**, which names `lattice
  --validate` where this stage builds `rigor_monotonicity` (§7b.8); and **the design's two stale copies of AC-001's
  withdrawn instrument** — §3.4's ban-list row and §10.2's Phase-1 exit criterion (4) both still read *"with `-D
  warnings`, plus a `trybuild` must-fail fixture"*, which AC-001's *Verified by* withdrew on 2026-09-11
  (PIA-030/PIA-031) and which `clippy.toml`'s own header and `s8`:102-114 have already absorbed. **That last one is
  the single case in this list where the stage file is right and the record above it is stale**, so it takes a dated
  in-place correction rather than a supersession — neither sentence is an ADR `## Decision`. **None of them is
  written here** (`00-preamble.md`:55).
- **The register reconciliation is OWED.** This file claims **115** ids (§7b.3 carries the rule and the arithmetic) and names its
  discharges and its arms by id; what it does **not** do is reconcile the claimed ranges against the register's own per-area State lines, which
  moved during this session. That is a register-side count and `00-preamble.md`:7-10 gives the register every criterion's
  state. **Record it in `PROGRESS.md` §2 against Step 7b.**

**ADR-0101 clause 5 governs this stage's close-out too**, through the M3 line **Checkpoint 7b-ii** gates — of which
Checkpoint 7b is the first half (`INDEX.md`:80 moved M3 to the later gate on 2026-09-13, in the same change as the
split): until S-SUBTLE-CB runs, **no document may claim the review gate is guarded against false approval**.
Checkpoint 7b's pass says selection and the lints exist and fail closed; Checkpoint 7b-ii's says the gates do.
Neither may be written up as more, and **neither alone reaches M3**.

**Spikes this step consumes:** S-GRAMMAR (F42, done). **Spikes it owes:** P2-B (AC-068's value, ADR-0117 clause 4),
S-FACET (the `require` rows keyed on `ui`, `flag_exists`, `data_migration` and `surfaces`), S-BIN (`ChangeSummary`'s
field set, 7c's) — all registered, none run, and none of them gates this stage's build.
