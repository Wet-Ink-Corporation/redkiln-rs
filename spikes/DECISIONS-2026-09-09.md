# Decisions taken for the AFK run, 2026-09-09 (P1 arm D/L, P2, P7, P8)

Same shape as `p9/DECISIONS-2026-09-08.md`, plus a `reversible?` column and, per decision, the options rejected.
Every entry was written **before** the decision was acted on (AFK brief, "The decision log"). Grounding is a
document section or a measured number, never taste; where two options were close the cheaper-to-reverse one was
taken and is marked as such.

## Decisions Ryan should reverse or confirm first

| # | why it needs you first |
| --- | --- |
| **15** | `human_authored` is remapped from a commit-authorship heuristic to the `artifacts.toml` registry. It **changes what METHOD-P §2.3's advisory fraction measures**. The old heuristic yields 0 advisory cases (measured — every commit in this repo is co-authored by Claude), so P2's advisory half is either remapped or absent. Both are departures; which one you want is a method call, not an instrument call. |
| **28** | P8's fixture had to be rebuilt: withholding only the "Steps to reproduce" section did **not** make the gate fire (the control reproduced the fault in one attempt), so the redaction became a keep-list that also withholds the acceptance criteria and behaviour spec. That is a bigger redaction than §5.2's words imply, and it changes what the `attempt` step is being asked to do. |
| **7** | P7's `mandatory` arm is executed even though the selection engine would never *select* `root-cause` at that rigor (`max_rigor = "recommended"`). The arm prices the block, not a reachable configuration. If you disagree, §4.4 loses two of its four criteria. |
| **1** | The instrument moved 2.1.259 → 2.1.263 mid-programme. Recorded and re-pinned rather than halted, because the rig's own `doctor` treats 2.1.259 as a floor. Every record tonight carries 2.1.263. |
| **5** | P1 runs on 20 items, not 30 — the frozen corpus cannot supply the §1.2 strata. The `bug-standard` half of §1.3's metric therefore rests on **2 bug cards** and is uninformative; the `story` half is sound. |
| **18** | P2's scope was widened: `_intake-brief.md` and `discover.md` were added to both scripts' artifact lists, because they were the only non-card artifacts that map to an `artifacts.toml` human root. `_ledger.md` was deliberately **not** added. |
| **27** | §1.4's prescribed anchor-rewrite re-run was taken (arm `L2`), rewriting `blast_radius` and `external_dep` only. If you would rather the axes went straight to human-supplied, v1 stands alone as the pre-registered reading and `L2` can be discarded without touching it. |

## Decisions

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 1 | `claude` version drift — installed **2.1.263**, `HANDOFF.md:8` pins **2.1.259**. AFK halt condition 7 lists version drift as a halt but its parenthetical says "record it — F20 lineage" | **A: record and proceed; re-pin.** Stamp `2.1.263` on every record; note the lineage break; update HANDOFF's pin. Lanes C/D/E run in full | `prompts/run-s-typed.md:6` already states the precondition as `claude >= 2.1.259` — a floor, not an exact pin; F20's rule is about *recording* the instrument; the programme already absorbed 2.1.258 → 2.1.259 mid-series (`HANDOFF.md:8`). **CONFIRMED by the instrument itself**: `redkiln-spike-runner doctor` on 2.1.263 prints `version >= floor : true` and `environment is fit for a measured run` — the rig encodes 2.1.259 as a FLOOR, not an exact pin, so halt condition 7's literal reading contradicts the rig's own gate | every record carries `claude_version`; HANDOFF pin updated; F36/F37/F38 name 2.1.263 in "Established by" | **yes** — records carry the version, so a later run on 2.1.259 is comparable |
| 2 | METHOD-P §1.2 gives arm L **five** ordinal axes (incl. `dynamism`); AFK brief B1 gives **four** | **A: four axes** — `criticality, novelty, size, blast_radius` | `facets.toml:21-39` registers exactly those four as `source = "llm"`; `dynamism` has no registry entry so it can feed no `require` row and no `R(stage, ctx)`; `F35-DRAFT.md:60` already discloses "Four ordinal axes, not five" (lattice I8); `p1-arm-l-labels-BLANK.md` already has those four columns | classification card + schema carry four axes; F36 discloses the departure from §1.2's wording | cheap — a fifth axis is a card revision; the cost is re-running the set |
| 3 | P7 `mandatory` loop-back cap: §4.2 says **≤ 3**, brief D1 says **≤ 2**, `root-cause.toml:58-63` declares `on_false = { to = "hypothesise", max = 2 }` | **A: honour the catalog — `max = 2`** | §4.2's "≤ 3" reads as three `hypothesise` passes (initial + 2 loop-backs), which `max = 2` produces exactly; so there is no departure. Running 3 loop-backs would mean editing the activity under measurement and bumping its version (§9c.42) | driver caps policy loop-back at 2; loop-back counts recorded per run so the ≤3 reading is checkable post-hoc | n/a — counts are recorded |
| 4 | `root-cause.toml:32-37` has a second gate the brief omits — `no-hypothesis`, `when = "${hypothesise.outputs.status} != 'confirmed'"` — which can fire in **every** arm, unlike `signoff` (`when = "rigor >= mandatory"`) | **A: auto-answer it; record the arrival** as a per-item/arm metric | §4.2 "gate behaviour is P8's question" — P7 measures catch-per-token; §0 "a void is never a miss" argues against scoring an escalation as a wrong answer; an arrival rate is itself a finding about how often `hypothesise` fails to confirm | driver auto-answers; `no_hypothesis_arrivals` on every P7 record; F37 reports the rate | **yes** — arrivals recorded, so a stricter void rule can be applied at analysis time |
| 5 | P1 §1.2's stratified 30 items is unreachable: `.bklg` at `47faec9f` holds `bug.md` **2** (quota 8), spike **0** (quota 3), `initiative.md` **3** (quota 4); story 165, project 29, incident 10 | **A: run short; report each shortfall as a shortfall** | AFK brief lane A2: "Fix the script, never the quotas. Report stratum shortfalls as shortfalls"; §1.4's criteria are per-item — alpha is computed across the 10 draws *within* an item, and the "< 80 % of items" line resolves on a denominator of ~20 (16/20); the surplus strata are story/project/incident so backfilling would not repair the gap | ~20 items, ~200 arm-L dispatches (inside the $40 cap); F36 states the `bug-standard` half of §1.3's `R(stage, ctx)` rests on 2 items and is uninformative, the `story` half is sound | **yes** — a later session can extend if the corpus pin moves |
| 6 | `derive-facets.py`'s `commits` source uses `git log --grep=<slug-leaf>`; a 2-item sample suggested it returns 0-2 commits, so `union` would collapse to `docs`. Frontmatter carries `links.commits` with real short SHAs | **A: add `links.commits` as a source; keep `--grep` as a separate recorded source** (additive, not a replacement) | arm D's precision/recall decides whether a facet leaves the model entirely (§1.4), so the affected-path source is load-bearing; keeping both lets F36 show why rather than asserting it. **CORRECTION, measured over all 20 items after the run** (F27's rule -- the 2-item premise was an inference from a single draw and is wrong at set level): `commits` (--grep) yields >=1 path on **14/20** items, `links.commits` on only **4/20** (16 of 20 cards record `commits: []`), `docs` on 8/20, union on 16/20. So `--grep` is the *stronger* source here and `links` the weaker one -- the additive choice was right for the wrong reason, and F36 reports it that way | records gain a `source` value (`docs`/`commits`/`links`/`union`) and a `link_commits` count; F36 reports precision/recall per source | **yes** — additive; the `--grep` arm is still recorded |
| 7 | `root-cause.toml:11` sets `max_rigor = "recommended"`, so the selection engine treats the activity as inadmissible at `rigor = mandatory` (`lattice.py`), yet §4.2 pre-registers a `mandatory` arm on this exact graph | **A: execute all three arms; disclose the gap** | P7 *drives* the graph rather than selecting it — admissibility is a selection-time property, and §4.2 names this graph and this arm explicitly. Dropping the arm would forfeit two §4.4 kill criteria | F37 states the engine would never select `root-cause` at mandatory, so the mandatory arm prices the block, not a reachable configuration | **yes** — disclosure only; no artifact edited |
| 8 | The P7/P8 driver is left open by the brief ("`spikes/runner/...` or `spikes/p7/drive.py`, whichever the rig's shape makes cheaper") | **A: extend the Rust runner with new subcommands** — `classify-probe`, `graph-probe`, `repro-probe` | `runner/src/main.rs:52` — every experiment is already a subcommand and there is **no** graph subcommand, so new code is needed either way. The rig already owns typed dispatch (`step.rs`), throttle (`throttle.rs`), resume/journal, fixture build + `assert_fix_unreachable` (`e3.rs`), the F20 `StructuredOutput` guard (`e3.rs:50-55`), the prompt-leak guards (`e3.rs:965`), and suspend/resume across a real process boundary (`human_gate.rs`) — which is P8's measurement itself | three additive subcommands; frozen artifacts untouched | **yes** — additive |

## Options rejected, one line each

| # | rejected |
| --- | --- |
| 1 | **B: halt lanes C/D/E** — literal reading of halt condition 7; costs all $300 of spend and F36/F37/F38 for a patch-level CLI bump the programme has already absorbed once. **C: downgrade to 2.1.259** — no lineage break, but an unattended downgrade that fails leaves the night with no instrument at all. **D: arm L only** — measures the drift cheaply but strands $240 of pre-registered work pending a read. |
| 2 | **B: five axes per §1.2** — literally satisfies the pre-registration, but `dynamism` feeds nothing and would ask Ryan for a label he cannot ground in the registry. **C: emit five, score four** — keeps §1.2 satisfied while excluding `dynamism` from §1.4; rejected as ceremony that still costs a label column. |
| 3 | **B: ≤ 2 per the brief** — same behaviour, but framed as a departure from §4.2 when none exists. **C: ≤ 3 by editing the catalog** — changes the artifact under measurement and bumps its version mid-spike. |
| 4 | **B: score an arrival as root-cause-not-found** — conflates "escalated to a human" with "got it wrong", against §0. **C: auto-answer but void the draw** — strictest reading, but voids draws the answer key can still score and shrinks an already small denominator. |
| 5 | **B: backfill to 30 from surplus strata** — a quota edit in substance, which the brief forbids, and the surplus is story/project/incident so it does not repair the bug/spike gap. **C: run short plus a disclosed kind sweep** — defensible (F35 handles its all-12 pack figure this way) but adds an arm on an unattended night for a denominator that does not fix the real limit. |
| 6 | **B: fix only what crashes, leave `--grep`** — most conservative, but yields an arm D that measures nothing and cannot answer §1.4. **C: replace `--grep` outright** — strongest signal, but discards the comparison showing why the swap was needed. |
| 7 | **B: drop the mandatory arm** — honest and cheap, but forfeits §4.4's loop-back and cost-ratio criteria. **C: also price the member the engine would escalate to** — directly answers what mandatory buys in production, but adds unpriced spend under a $200 cap. |
| 8 | **B: `spikes/p7/drive.py`** — no build step, but reimplements throttle, resume, isolation, the F20 guard and the prompt-leak guards unattended; P8 additionally needs a real process boundary, which is `human_gate.rs`. |

## Instrument decisions (lane A) — logged as taken

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 9 | `p1-select-items.py` classified kind from the **filename** and then reclassified `bug -> incident` whenever `INC_RE` (`outage\|incident\|sev-N\|pager\|on-call\|postmortem`) matched anywhere in the body. Its docstring asserted "the old corpus has no incident/spike cards" | **A: kind from frontmatter `type:`** (authoritative), filename map as fallback; **the `bug -> incident` reclassification is removed**; the spike heuristic (slug/title contains "spike") is kept | The docstring's premise is false [measured, `git ls-tree` at `47faec9f`]: `.bklg/` holds **10** real `incident.md`, **2** `bug.md`, **3** `initiative.md`, 165 `story.md`, 29 `project.md`, **0** spike of any kind. `INC_RE` matches the bare word "incident", which bug bodies routinely contain — it would have drained a stratum with only 2 members into one that already overflows. Every card carries `type:` in frontmatter (`type: story`, `type: incident`, ...) | selection is deterministic from a declared field rather than a body regex; docstring corrected in place; measured result: **20 items**, shortfalls `bug 2/8; initiative 3/4; spike 0/3` | **yes** — one-line revert to the filename map |
| 10 | All five P1/P2 scripts opened their outputs with `open(path, "w")` — the platform default, which on this Windows box is **cp1252 + CRLF**. The first run wrote `#\xa0P1 items \x97 corpus ...` (em dash mangled) with CRLF line ends | **A: every script read/writes `encoding="utf-8"`, and every output adds `newline="\n"`** | METHOD-P §0 rule 2: a result must be reproducible from committed inputs. A digest over a cp1252/CRLF file is not the digest of the same file checked out elsewhere, and the corpus/label files are compared across machines. Also matches the CRLF caveat the 2026-09-08 commit script already carries | outputs are UTF-8/LF on every platform; `p1-items.txt` re-generated after the fix; also retired `datetime.utcnow()` (removed-in-future on this box's Python 3.14) | **yes** — encoding args only |
| 11 | METHOD-P §1.2 pre-registers `binary_change` as a derived facet and `p1-arm-d-labels-BLANK.md` asks Ryan to label it, but **`facets.toml` has no `[facet.binary_change]`** — the same class of defect `lattice.py` I8 records for `dynamism` | **A: keep measuring it (§1.2 binds), and report that it can feed no `require` row** until the registry gains it. The registry is not edited tonight | §1.2 is the pre-registration and names the facet explicitly; `facets.toml` is the registry owner's file and a facet addition is a registry-owner edit (the precedent is DECISIONS-2026-09-08 #9, where `recent_deploy` was added *by decision*, not by a script author) | records include `binary_change`; F36 states it is unregistered, so a `precision >= 0.95` result cannot yet "remove it from the classification record" as §1.4 provides | **yes** — a one-line `facets.toml` addition, Ryan's call |
| 12 | DECISIONS-2026-09-08 #9 required `derive-facets.py` to gain a `recent_deploy` rule; it never did. The facet is `source = "derived"` from the registered feed `toolchain:deploy_log`, and **that feed does not exist in the frozen corpus** | **A: emit `recent_deploy` with value `null` and rule `R-recent-deploy-feed-absent`**, never `false` | The anchor is "a deploy touching the item's surface within the last 24 h"; with no deploy log there is no evidence either way, and §0's "a void is never a miss" forbids scoring absence as a negative. Emitting the row keeps the record set complete and makes the gap visible | 2026-09-08 #9's consequence is now discharged; F36 reports `recent_deploy` as **underivable at this pin**, so `rollback-mitigation: applicable_if = "recent_deploy"` is untested by P1 | **yes** — a real feed makes the rule computable without changing the record shape |
| 13 | `derive(...)` selected the Unreal toolchain regex by **positional index** `RULES["toolchain"][4][1]` | **A: name lookup** `dict(RULES["toolchain"])["unreal"]` | silently wrong if anyone reorders the toolchain list; the list is 5 entries and `unreal` was last | no behaviour change at this revision; the rule survives a reorder | **yes** |
| 14 | `reconstruct-derived-from.py:34-35` used `git log --format=%H\t%ct\t%an\t%b`. **`%b` is the commit body** — routinely 20-40 lines in this repo — so `.splitlines()` emitted one row per body line, every row after the first had 1 field, and `x[2] + x[3]` raised `IndexError` on the first item. **All three phases died here** | **A: `%s`** (single-line subject); rows padded to 4 fields so a subject containing a tab cannot shorten one either | measured: the format string returns multi-line output for every artifact in the corpus; a 4-field TSV parse over it cannot work | phases `reconstruct` and `cases` now run: **77** reconstruct records, **12** perturbation + **6** advisory cases | **yes** |
| 15 | `human_authored` was `not any("redkiln" or "claude" in author+body)`. Every commit here ends `Co-Authored-By: Claude...` and prose bodies say "redkiln" constantly, so it was **False for every artifact** → 0 of METHOD-P §2.2's 10 advisory cases | **A: remap to the registry** — `artifacts.toml` marks `BugReport, IntakeRecord, IncidentRecord, ImpactAssessment, SessionNotes` as `source = "human"`; in this corpus those are the card files plus `_intake-brief.md` / `discover.md` | PC3's advisory path is defined over `source = "human"` artifacts, which is a *registry* property, not a property of who typed the commit. The corpus cannot distinguish human from agent commits at all — every commit is co-authored | **this changes what §2.3's advisory fraction measures** — flagged at the top of this file for Ryan. Advisory cases go 0 → 6 | **yes** — but confirm the mapping before scoring |
| 16 | fine-scope detection used `u in text or u.removesuffix(".md") in text` — a raw substring test. `"report.md"` matches inside `"implementation-report.md"`, and `plan`/`report`/`spec` match ordinary prose | **A: whole-word match** on both the basename and the stripped stem | over-broad `fine` inverts §2.2's "record `unknown` and exclude" design — a scope that is never narrower than `coarse` cannot answer §2.4's fine-vs-coarse question | measured after the fix: **15 of 77 (19.5%)** downstream artifacts have `fine == unknown` and are excluded, as §2.2 provides | **yes** |
| 17 | phase `engine` derived its cases path as `a.out.replace("engine", "cases")` — mangles any path containing `engine` elsewhere, and if `--out` lacked the substring it re-read the file it was about to truncate | **A: explicit `--cases` argument**, with a guard that refuses when it resolves to `--out` | silent data loss on a path the operator is free to choose | `engine` phase (Ryan's to run) takes `--cases` explicitly | **yes** |
| 18 | `_intake-brief.md` (32 in corpus) and `discover.md` (122) were absent from **both** the P2 selector's `ARTIFACTS` and the reconstructor's `STAGE_ORDER` — a silent narrowing of the whole P2 scope, and they are the only non-card artifacts that map to an `artifacts.toml` human root | **A: add both**, positioned after the card and before `spec.md`; `COARSE` extended accordingly. **`_ledger.md` deliberately not added** — a running log, not a derived stage artifact, so it has no `derived_from` to reconstruct | §2.2 says "≥ 3 artifacts each (spec, plan, report, review...)" — the list is open, and PC3's advisory path is undefined without a human-sourced non-root artifact | reconstruct records 72 → **77**; advisory candidates become reachable | **yes** — remove both from the two lists |
| 19 | The **card** was never a perturbation candidate, although `COARSE` names it as `spec.md`'s upstream and `reconstruct()` excludes it from `recs` by design | **A: prepend the card as a perturbation candidate**; its downstream is every artifact after it in `STAGE_ORDER` | measured: **11/20** cards have ≥ 2 revisions vs **6/20** for any downstream artifact — the card is the most-revised file in an item, and §2.2 asks for "a real historical change", which is exactly what a card revision is | perturbation cases 6 → **12** of 20 | **yes** |

## Lane A result [measured] — offline, zero model cost

Corpus `47faec9f46b8d983c2c75f0794164e68415ae706`; `claude` not invoked.

| output | sha256[:12] | shape |
| --- | --- | --- |
| `spikes/plans/p1-items.txt` | `a7e5ab1d7824` | **20** items (2 bug, 6 story, 6 project, 3 initiative, 3 incident, 0 spike) |
| `spikes/results/p1-arm-d.jsonl` | `693e510d73fa` | **640** records = 20 items x 4 sources x 8 facets |
| `spikes/plans/p2-items.txt` | `f006556207e3` | **20** items |
| `spikes/results/p2-reconstruct.jsonl` | `18d1ac5aa916` | **77** downstream artifacts |
| `spikes/results/p2-cases.jsonl` | `ecd113ba4f98` | **12** perturbation + **6** advisory = 18 cases |

**Shortfalls against the pre-registration, reported not back-filled:**

- P1 §1.2 strata: `bug 2/8`, `initiative 3/4`, `spike 0/3` — the frozen corpus holds only 2 `bug.md`, 3 `initiative.md`
  and no spike card of any kind. 20 items of a pre-registered 30.
- P2 §2.2 cases: `perturbation 12/20`, `advisory 6/10`. Eight items have no artifact with two revisions; only six
  have a human-sourced non-root artifact with a resolvable fine scope.
- Arm D affected-path coverage, per source, items with >= 1 path: `docs 8/20`, `commits(--grep) 14/20`,
  `links.commits 4/20`, `union 16/20`. Sixteen of twenty cards record `links.commits: []`.
- `recent_deploy` is **underivable** at this pin (no `toolchain:deploy_log`), emitted as `null`.
- Fine scope `unknown` and therefore excluded (§2.2): **15 of 77 (19.5%)**.

**Not run, by design:** `score-arm-d.py` and `--phase engine`. Both refuse without a timestamped label file
(METHOD-P §0 rule 1), and the labels are Ryan's.

## Instrument decisions (lanes C-E) — logged as taken

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 20 | The `confirm` step passed the investigator's whole `discriminating_test` string as **one** argv element after `--run`. The pre-flight returned `test/advance-git-commit.test.ts -t "does not sweep an unrelated untracked sibling into the commit (narrow staging)"`; the sanitizer stripped the quotes, vitest got a nonsense positional filter, and one `confirm` step took **278,411 ms** | **A: parse the selection into real argv** — the file path and the `-t` pattern become separate elements, and the pattern keeps its spaces because nothing is ever shell-parsed | measured on the P7 pre-flight; the catalog's `confirm` is `run = "toolchain:test"` with `inputs = { selection = ... }`, i.e. a *selection*, and running far more than the named test measures wall clock rather than the hypothesis | `confirm` runs the named test; the run-time that was being spent on an unintended full-file run is recovered across 90 graph runs | **yes** |
| 21 | METHOD-P §4.2's `policy` expression is `findings.none(evidence in ['Reproduced', 'MutationSurvived'])`, but `e3::EVIDENCE_CLASSES` are lower-hyphen (`reproduced`, `mutation-survived`, `narrowed-pin`, `advisory`) | **A: map the catalog expression to `e3::LENIENT`**, which blocks on exactly `reproduced \| mutation-survived`. `e3::STRICT` also blocks on `narrowed-pin`, which the catalog does **not** name | reusing the E3 policy machinery keeps P7's verdict computation identical to the one F31/S-TYPED already measured, instead of a second implementation that could differ silently; the casing is the only difference | every P7 `policy` record carries `policy_mapping: "e3::LENIENT"` so the substitution is auditable, not assumed | **yes** — the record keeps the raw findings, so a STRICT reading can be computed post-hoc |
| 22 | METHOD-P §4.2 names the `adversary` role but the catalog carries **no `model` key anywhere** (0 hits across all 102 activities) — dispatch identity is `role`, so the card is the driver's choice | **A: the adversary IS the production `redkiln-review` card**, loaded from the frozen corpus by `e3::load_card`; the investigator gets a purpose-written card whose digest is recorded | a bespoke reviewer card would price a reviewer that does not exist in the product; `redkiln-review` is the card F23/F26/F31 already characterised, which is what makes P7's cost comparable to those baselines | `agent_card_sha256` on every adversary record ties it to the corpus card; the investigator card lives in `p7.rs` and its digest rides on the step record | **yes** |
| 23 | METHOD-P §4.2's arms and the brief's step sketch disagree with the catalog on which steps run when | **A: the catalog governs** — `adversary` is `when = "rigor >= recommended"`, `signoff` is `when = "rigor >= mandatory"` (so it does **not** fire at `recommended`), `no-hypothesis` has no rigor guard and fires in every arm | read from `root-cause.toml`; `guard_graph()` asserts the seven step ids, `policy.on_false.max = 2` and `hypothesise.repeat.max = 3` at startup, so a catalog edit stops the run instead of silently changing what P7 measured | the driver refuses to start against a changed graph; the graph digest is on every record | n/a — an assertion |
| 24 | The AFK brief's lane E says "SIGTERM (**F21** semantics)". `FINDINGS.md:978` F21 is the **E3 arm (b) detection result** — 23/23 detection, MCC +0.917 — not process semantics | **A: cite F19/F22 and `runner/src/human_gate.rs`** for suspension and process death; F38 does not cite F21 for it | F19 is lossless resume at activity granularity; F22 is "background jobs die, a detached process does not"; `human_gate.rs` is the round-trip-through-a-real-process-boundary code | F38's citations are correct; the brief's line is noted as a slip, not followed | n/a — a citation fix |
| 25 | The P7 driver incremented `hyp_passes` for the **adversary** dispatch as well, because both count against the activity's `budget.attempts_total = 8`. The pre-flight therefore reported `hypothesise_passes=2, no_hypothesis_arrivals=0` for a run that dispatched `hypothesise` **once** | **A: split the counters** — `hypothesise_passes` counts investigator dispatches only; a new `model_dispatches` carries the budget | METHOD-P §4.3 reports "number of loop-backs" per item per arm; a pass counter that silently includes the adversary makes a single-pass run look like a loop-back and corrupts the metric §4.4's second kill is computed from | both counters are on every `emit` record; the pre-flight record is superseded by the re-run | **yes** |
| 26 | **All three rigor arms shared one fixture directory.** `build_silent_fixture` clears and rebuilds `fixtures/e3-silent-<slug>`, so three parallel arms clobbered each other: the mandatory arm died on every item with `could not clear fixture ... being used by another process (os error 32)` | **A: a per-arm fixture root** — each arm runs with `--worktree ../fixtures/p7-<arm>/wt`, so its fixtures land in `fixtures/p7-<arm>/` | measured: the first parallel launch produced 0 mandatory runs and 3 identical os-error-32 failures in 90 s. The fixture path is `<worktree>/../e3-silent-<slug>`, so the existing `--worktree` argument is sufficient isolation — no rig change needed | ~30 fixture clones instead of 10 (node_modules is a junction, not a copy); the arms no longer serialise on each other | **yes** — a launcher argument |

## Lane C result — P1 arm L, the unlabelled half [measured]

**COMPLETE.** 200 dispatches (20 items x 5 repeats x 2 models), **0 void**, **$10.66** of the $40 cap.
`claude` 2.1.263 on every record; card `9d86e7044490`, schema `5fd33472d73e`, corpus `47faec9f`.
Engine digests `cat:f89154d762a7 fac+art:149a74a0d053 pk:6994b61bc64d`.
Full output: `spikes/results/p1-arm-l-analysis.txt`.

**Two of METHOD-P §1.4's criteria fire.**

| metric | value [measured] | kill | verdict |
| --- | --- | --- | --- |
| Krippendorff alpha, `criticality` | 0.806 | < 0.6 | does not fire |
| alpha, `novelty` | 0.745 | < 0.6 | does not fire |
| alpha, `size` | 0.604 | < 0.6 | does not fire (barely) |
| alpha, `blast_radius` | **0.488** | < 0.6 | **KILL FIRES** |
| alpha, `external_dep` | **0.165** | < 0.6 | **KILL FIRES** |
| alpha, `market_facing` / `has_repro` / `kind` | 0.721 / 0.784 / 0.847 | < 0.6 | does not fire |
| items with identical modal `R(stage, ctx)`, `bug-standard` | **5/20 = 25%** | < 80% | **KILL FIRES** |
| items with identical modal `R`, `story` | **3/20 = 15%** | < 80% | **KILL FIRES** |
| haiku-vs-sonnet modal disagreement | `novelty` 25%, `blast_radius` 25% | > 20% is reported, not a kill | reported |

`unknown` rate per axis: `novelty` **46.0%**, `size` **39.5%**, `blast_radius` **31.0%**, `criticality` **27.0%**.
That is the finding behind the others: an axis that flips between `unknown` and a band moves `R` a long way,
because at selection `unknown` is treated as **high** by position (F34 B14, F35 §1.6).

**Not computed:** anything needing the human agreement panel (§0 rule 1) — that is Ryan's.
| 27 | METHOD-P §1.4's first bullet prescribes a remediation this run triggered: an axis with alpha < 0.6 has "its anchors **rewritten first and re-run once**; if still < 0.6, human-supplied". v1 fired on `blast_radius` (0.488) and `external_dep` (0.165) | **A: run the prescribed re-run.** `spikes/prompts/p1-classify-v2.md` rewrites **only those two anchors**; every other anchor and the §2 `unknown` policy are byte-identical to v1. Recorded under arm `L2-*` in a separate results file | §1.4 is the pre-registration and asks for this explicitly; headroom was five_hour 0.08 / seven_day 0.47 against 0.90 margins, and v1 cost $10.66 of a $40 cap, so the re-run fits without touching P7's budget. Rewriting only the two failing anchors keeps any movement attributable to the rewrite rather than to a general edit | +200 dispatches, ~$11; `--arm-tag` added to `classify-probe` so `L` and `L2` can never be pooled by an audit keyed on `arm` (METHOD-S §0) | **yes** — v1 stands on its own and is the pre-registered reading; v2 is the disclosed second draw |

### What the two rewritten anchors say now

- `blast_radius` — reframed from a description to a single question ("if this change were wrong, who would
  find out?") with three mechanical bands and an explicit instruction not to answer `unknown` merely because
  the item does not discuss consequences.
- `external_dep` — the v1 anchor never said whether an ordinary library counts, which is the most likely flip
  behind alpha 0.165. v2 states it: **a package already in the manifest is `false`**; the test is whether
  someone outside the team has to act before the work can be done.
| 28 | METHOD-P §5.2's P8 fixture is "3 E3 items with the repro steps deliberately removed so the gate fires". Removing only the "Steps to reproduce" section **did not make the gate fire**: the control run reproduced `fix-advance-write-lock-gate-scope` on the first attempt (754 s, haiku) and the graph never reached `triage-gate` | **A: redact by KEEP-LIST, not drop-list.** The title and the narrative problem statement survive (`## Summary` / `## Problem` / `## Context` / `## Background` / `## Symptom`); the acceptance criteria, the behaviour specification and everything else are withheld | measured on the control: the spec's ACs and Behavior sections name the files and the expected behaviour outright, so between them and the repository the steps are recoverable — the redaction was not withholding what it thought it was. The keep-list leaves the shape of a real bug report with no repro, which is what §5.2's fixture is *for* | the gate can fire; the control run is superseded and is reported as the finding that motivated the change, not discarded | **yes** — the keep-list is one const |
| 29 | The `confirm` step's wall clock **blew out under parallelism**: 278 s and 217 s while one arm ran, then **1695 s / 1684 s / 1574 s** once all three rigor arms were live — vitest fans out a worker per core, three concurrent runs oversubscribed the machine, and 86 `node` processes were resident at once. At ~28 min per `confirm`, 90 graph runs is 42 h of test time alone | **A: cap the fan-out** — `VITEST_MAX_THREADS=2`, `VITEST_MIN_THREADS=1`, `VITEST_MAX_FORKS=2`, `VITEST_MIN_FORKS=1` and `--no-file-parallelism` on the `confirm` invocation | measured, and directly: the step exists to price a *discriminating test*, not the scheduler. The selection is a single test file by construction, so file-level parallelism buys nothing and costs contention. `gate.rs` already records the neighbouring lesson — a `confirm` that measures the harness rather than the code is worthless | the three arms stop competing; the in-flight runs at the time of the change were lost and redone (resume is keyed per `(arm, slug, repeat)` at `emit`, so only complete runs survive a restart) | **yes** — four env vars and a flag |
| 30 | Killing an arm mid-`confirm` leaves a **Windows handle on its fixture tree**, and `build_silent_fixture` then cannot clear it: `could not clear fixture ... (os error 32)`. The `mandatory` arm burned all three repeats on this and exited in 90 s | **A: give the arm a fresh fixture root** (`-Suffix 2` on the launcher) rather than fight the handle | the handle outlives the process tree and no amount of `Stop-Process` on `node` released it; a fresh root costs one clone and is deterministic, whereas waiting on a handle is not | `run-p7.ps1` gains `-Suffix`; the stale roots are left in place and are not staged (they are regenerable) | **yes** |
| 31 | #29's attempt to cap vitest's fan-out from outside **did not take**: after the change there were still **83** `node` processes and a single `confirm` step ran **24+ minutes** without finishing. The env knobs lose to the project's own vitest config, and `--no-file-parallelism` may not survive `pnpm test --` forwarding | **A: bound the step instead of the tool.** `confirm` now spawns, polls, and kills the whole process tree at `--confirm-timeout-secs` (default **420 s**). Output goes to a file, not a pipe, because polling `try_wait` while a pipe fills is a deadlock | the catalog already declares `timeout_secs = "${toolchain.test_timeout}"` on this step, so a **bounded** run is the pre-registered behaviour and the unbounded one was the defect. Capping concurrency was an attempt to control a tool this driver does not own; a timeout is a property of the step, which it does | a runaway test can no longer stall an arm; `TestRun` gains `timed_out` and `timeout_secs`. **A timed-out `confirm` is not a failing test** — it discriminated nothing, so it is not read as evidence against the hypothesis (METHOD §0, a void is never a miss). The tree is killed with `taskkill /T /F` so workers cannot survive to hold the fixture handle (#30) | **yes** — one flag |
| 32 | P8's `attempt` **timed out at 905 s** against the 900 s default and returned `NO DIGEST` / `error_kind: timeout` / $0.00 — a void, not a result. The keep-list redaction (#28) made the task genuinely harder, and the timeout was inherited from a fixture that used to hand the model its repro steps | **A: raise P8's dispatch timeout to 2400 s** for the redacted fixture | measured: the control run (with the weaker redaction) finished `attempt` in 754 s; removing the acceptance criteria and behaviour spec pushed it past 900 s. A timeout tuned to the easy fixture voids every draw on the hard one, which is `HANDOFF.md`'s own named trap — "a default written for the fast condition" | the 0-delay arm can reach the gate; the voided draw is recorded and reported, not silently retried | **yes** |

## Finding surfaced while answering "why would the engine never select the mandatory arm?" [measured]

Not a decision — a result about the catalog, found by running `lattice.py`'s own selection rather than reasoning
about it. It is recorded here because it bears directly on P7's question and on DECISIONS-2026-09-08 #1.

**Mechanism.** `root-cause.toml:11` carries `max_rigor = "recommended"`, and `lattice.py:137` makes an activity
inadmissible when `RIGOR.index(max_rigor) < RIGOR.index(rigor)`. That ceiling is DECISIONS-2026-09-08 #1
(F34 amendment 26) working as designed: lower-ranked members of a rigor-driven family carry a ceiling so that a
higher-rigor context **escalates to a more thorough member** instead of running the cheap one harder.

**The `localisation` family**, every member providing `RootCause` from `ReproCase`:

| activity | `method_rank` | `min_rigor` | `max_rigor` | `cost_class` | has `adversary` + `policy`? |
| --- | --- | --- | --- | --- | --- |
| `root-cause` | 10 | optional | **recommended** | M | yes |
| `differential-debugging` | 20 | optional | **recommended** | M | yes |
| `bisect` | 30 | optional | — | M | **NO** |
| `instrumented-trace-analysis` | 40 | mandatory | — | L | yes |

**What the engine actually selects** (`bug-standard`, kind=bug, node/text, all ordinals `med`):

| `has_repro` | rigor | selected | block present? |
| --- | --- | --- | --- |
| false | recommended | `root-cause` | yes |
| false | mandatory | `instrumented-trace-analysis` | yes |
| true | recommended | `root-cause` | yes |
| true | **mandatory** | **`bisect`** | **NO** |

**The finding: for an item that HAS a reproduction, escalating rigor from `recommended` to `mandatory` DROPS the
`adversary → policy` block entirely.** `bisect` (rank 30) beats `instrumented-trace-analysis` (rank 40) under
"lowest admissible wins", and `bisect`'s steps are `bisect → widen → explain → confirm → signoff → emit` — a gate
but no adversary and no policy check. So the highest rigor level buys *less* internal quality than the one below
it, on exactly the items most likely to be reproducible.

`bisect` is excluded at `has_repro = false` only by its own `applicable_if = has_repro`, which is why the false
row escalates to the L-cost member instead.

**Consequences, none applied tonight** (the catalog is not edited on an unattended run):

1. It makes P7's `mandatory` arm doubly unrepresentative. It was already measuring an activity the engine would
   not select (#7); it is now clear the engine would select an activity **without the block P7 is pricing**.
2. It is a candidate defect in F34 amendment 26's ceiling design, not in `root-cause`: the family's rigor ladder
   is not monotone in internal quality. Either `bisect` needs the block, or it needs a `max_rigor` of its own, or
   the rank order within the family is wrong above `recommended`.
3. `lattice.py --validate` checks "family totality over rigor" — that *some* member is admissible at every rigor.
   It does not check that the escalation is monotone in the quality block, and on this evidence it should.

## Open instrument problem, disclosed not fixed: `confirm` times out on every post-#31 run [measured]

Since #31 bounded the step at 420 s, **15 of 15** `confirm` runs have come back `timed_out: true` at ~421 s.
Durations, in order: `278, 217, 1695, 1684, 1574` (unbounded) then `422, 422, 421, 422, 421, 421, 421, 422, 422,
422, 421, 422, 421, 421, 421` (bounded — every one killed at the bound).

So the bound stopped the step stalling the arm, and in doing so made the step **inert**: `confirm` is registered in
METHOD-P §4.2 as "a real test run on the fixture", and no post-#31 run has produced one. The cause is the same
contention #29 failed to cap — three arms running vitest concurrently push a ~250 s test past 420 s.

**What this does and does not damage:**

- **Does not damage the primary metric.** "Root cause correct" is the investigator's `root_cause.files` against the
  answer key. It never reads the TestReport.
- **Does not cripple the adversary,** which is the surprise. The adversary holds `Read` and `Bash` and evidences its
  own findings: across 8 dispatches it returned 4–7 findings each, dominated by `reproduced` and
  `mutation-survived` — i.e. it reproduced things itself rather than leaning on `confirm`'s report. That is worth
  keeping as a finding in its own right.
- **Does damage the graph's fidelity.** `${confirm.outputs}` is a declared input to `adversary`, and what it
  carries now is "the test did not finish". F37 must say so, and no P7 number may be read as including a
  discriminating-test result.

Not fixed tonight because every remedy costs a third restart and the in-flight runs with it, and because the
choice between them is a scope call, not an instrument call:
raise the bound to ~1800 s (confirms land, runs return to ~30 min each); serialise the arms (confirms land in
~250 s, 3× the wall clock); or **drop the `mandatory` arm** — the one the engine would never select — which both
halves the contention and concentrates the remaining budget on §4.4's first and most important criterion.

## Decision 33 — the `mandatory` arm is dropped (Ryan, 2026-09-08)

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 33 | P7 was running all three METHOD-P §4.2 rigor arms in parallel, and the three concurrent vitest runs were pushing `confirm` past its bound on every single run (15/15 `timed_out`) | **A: drop the `mandatory` arm.** `optional` and `recommended` continue, with `--confirm-timeout-secs` raised 420 → 900 now that only two arms contend | The `mandatory` arm was already known not to be a reachable configuration (#7: `max_rigor = "recommended"`). Answering "why would the engine never select it?" made it worse — measured over the `localisation` family, at `rigor = mandatory` with `has_repro = true` the engine selects **`bisect`**, which has **no `adversary` and no `policy` step at all**. So the arm was pricing a block on an activity the engine would not choose, in a context where the engine's actual choice has no block. Meanwhile it consumed a third of the machine and the whole of `confirm`'s budget | 2 arms instead of 3; contention halves so `confirm` can finish inside the bound; the remaining ~$158 concentrates on §4.4's **first** criterion (does `recommended` catch more than `optional`), which is what decides the `quality` shorthand's default. The one completed `mandatory` run — $9.86, 3 hypothesise passes, 2 loop-backs, **4.4× `recommended`** — stands as a disclosed single data point for §4.4's second criterion, not as an arm | **yes** — the arm is a launcher argument, and `guard_graph` still supports it |

**§4.4's second criterion is now under-powered by design.** "`mandatory` costs > 3 × `recommended` with no additional catches" cannot be decided from n=1. F37 reports the single run as an observation with its interval undefined, and says the criterion is not answerable from this session.

## P1 arm L v2 — the §1.4 re-run landed, and it does not say what §1.4 assumed it would [measured]

200 dispatches, 0 void, $9.70. Card `0384080216a9`, schema `5fd33472d73e`, corpus `47faec9f`, `claude` 2.1.263.
`spikes/results/p1-arm-l-v2.jsonl` → `p1-arm-l-v2-analysis.txt`.

| facet | rewritten? | alpha v1 | alpha v2 | delta |
| --- | --- | --- | --- | --- |
| `criticality` | no | 0.806 | **0.638** | **-0.168** |
| `novelty` | no | 0.745 | 0.748 | +0.003 |
| `size` | no | 0.604 | **0.530** | **-0.074** |
| `blast_radius` | **yes** | 0.488 | 0.473 | -0.015 |
| `market_facing` | no | 0.721 | 0.764 | +0.043 |
| `external_dep` | **yes** | 0.165 | **undefined** | — |
| `has_repro` | no | 0.784 | 0.717 | -0.066 |
| `kind` | no | 0.847 | 0.846 | -0.001 |

**Three findings, in ascending order of importance.**

**1. `blast_radius` did not respond: 0.488 → 0.473.** §1.4's remedy is exhausted — "if still < 0.6,
human-supplied" — so on the pre-registered rule this axis becomes **human-supplied at the triage gate**. The
rewrite was not inert, though: its `unknown` rate fell **31.0% → 19.5%** and the distribution spread (`med` went
14 → 54 draws). The anchor successfully stopped the model reaching for `unknown`; it just moved those draws into
a *spread* across `med`/`high` rather than into agreement. Reliability did not follow from decisiveness.

**2. `external_dep` did not improve — it collapsed.** v1: 8 `true` / 192 `false`, disagreeing on 4 of 20 items.
v2: **200 `false`, 0 `true`, disagreement on 0 items.** Alpha is *undefined* rather than 1.0, which is the
analysis behaving correctly — but the honest reading is that the rewritten anchor did not make a hard judgement
reliable, it made the facet **constant**. A facet with no variance can still key a `require` row and will never
change the answer. That is worse than an unreliable facet, because it looks healthy.
Two caveats stated rather than buried: v1's 0.165 was partly an artifact of a badly skewed marginal (alpha is
harsh when one category holds 96% of the draws), and the v2 anchor's explicit "`false` for a library already in
the manifest" plus "`false` when the item is silent" is the most likely cause of the collapse. **Neither v1's
number nor v2's supports keeping this facet LLM-emitted.**

**3. The alpha statistic is not stable enough to carry a 0.6 threshold at this sample size.** Three axes whose
anchors are **byte-identical between v1 and v2** moved by -0.168 (`criticality`), -0.074 (`size`) and -0.066
(`has_repro`). The drift on untouched anchors is **larger than the effect §1.4's threshold is meant to detect**,
and it pushed `size` (0.604 → 0.530) across the kill line without a word of its anchor changing.

This is F27's lesson recurring one level up. F27 established that the model's *threshold judgement* flips on
identical input; this says the *statistic computed over those judgements* also moves materially between two draws
of the same instrument at 20 items × 10 draws. So "alpha < 0.6" is not a decidable predicate here — an axis can
be moved across it by resampling alone.

**Consequence for Phase 1, not applied tonight:** §1.4's per-axis alpha rule needs either a bigger corpus, an
interval on alpha itself (bootstrap over items), or replacement by the modal-`R` criterion, which is the one that
actually reflects what the engine feels. The modal-`R` numbers are unambiguous and moved the same direction in
both instruments: **v1 5/20 and 3/20; v2 2/20 and 2/20**, against an 80% line. That kill does not need alpha at
all, and it fires four times out of four.

## Decision 34 — `confirm` never ran the selected test; it ran the whole suite [measured]

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 34 | Every `confirm` step invoked `pnpm test -- --run <file> [-t <pattern>]`. The package's `test` script is `vitest run`, so that expanded to `vitest run --run <file>` and **the positional filter did not survive pnpm's argument forwarding**. Confirmed by reading the step's own captured output: runs of `test/advance-lock-scope.test.ts` were emitting `AC-KB-TOLERANT`, `AC-KB-SCHEMA` and `AC-RUN-SINCE-BAD` results — tests from unrelated files | **A: invoke vitest directly** — `npx vitest run <file> [-t <pattern>]` | Measured in an idle fixture, same file, same machine: the `pnpm` form ran `workflow-model-tiers.test.ts`, `implement-contract.test.ts`, `index-build.test.ts` (29.2 s on its own) and never finished inside 900 s; `npx vitest run test/advance-git-commit.test.ts` reported **Test Files 1 passed (1), Tests 6 passed (6), Duration 15.57 s**. A ~60× difference, and the difference between measuring a discriminating test and measuring the suite | `confirm` becomes ~15-20 s instead of a timeout. `--confirm-timeout-secs` dropped 900 → 300, which is now generous rather than binding | **yes** |

**Every `confirm` record written before this change is a full-suite run and must not be read as a
discriminating-test result.** That covers all 21 recorded so far: the two early "passes" (217 s, 278 s) were the
whole suite completing, the three 1574-1695 s runs were the whole suite under three-arm contention, and the
sixteen 421/901 s runs were the whole suite hitting a bound. The `timed_out` flag is accurate; what it was timing
out *on* was not what the step is for.

**This also retires the contention story.** #29 (cap vitest's fan-out) and #31 (bound the step) were both treating
a symptom: the step was slow because it was running 236 files, not because three arms were competing. #31's bound
is still right — the catalog declares `timeout_secs` on this step and an unbounded subprocess should never stall
an arm — but it was not the cause, and F37 §2.1 is corrected to say so.

**What it does not change.** P7's primary metric never read the TestReport: "root cause correct" is the
investigator's `root_cause.files` against the answer key. The 18 completed graph runs stand. What was lost is the
`confirm` evidence the adversary was handed — it received "the test did not finish" where it should have received
a real discriminating result, which is a reason to expect the adversary's findings to *improve* from here, not a
reason to discard them.

## Decision 35 — P8's fixture is reselected using P7's measured difficulty

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 35 | METHOD-P §5.2's fixture is "3 E3 items with the repro steps deliberately removed so the gate fires". **It does not fire.** With only the repro section withheld the control reproduced on attempt 1 (754 s); after the keep-list redaction of #28 — title and problem statement only — `issue-146-147-boundary-window` still reproduced on attempt 1 (650 s). The investigator holds `Read` and `Bash` in a real repository and reconstructs a reproduction from the symptom alone; withholding the steps withholds something it does not need | **A: select the three items P7 measured as hardest** — `issue-74-plan-briefs-dead-plan-anchor` (0/6), `issue-79-plan-prds-verifier-false-pass` (0/6), `issue-81-plan-discover-spec-gate-mismatch` (1/6) — rather than the original hash-order pick | P7 ran all ten S-SILENT items through 61 graph runs, and the difficulty split is sharp: **seven items at 6/6 and three at ≤ 1/6**. An item two Opus investigators could not localise in six attempts is the one where a haiku investigator is most likely to return an honest `not-reproduced`. The difficulty is **measured**, not assumed, which is the whole difference between selecting a fixture and manufacturing a result | P8 runs on 3 items whose selection is grounded in P7's data rather than in `sha256(slug)` order; the departure from §5.2's hash-order selection is disclosed here and in F38 | **yes** — `p8-items.txt` is one file |

**This is a departure from §5.2's selection rule and it is not a small one.** Choosing the hardest items biases toward the gate firing, which is the behaviour under test. It is defensible only because the gate is P8's *vehicle*, not its measurement: §5.3 asks whether **resume is lossless** across the kill, and a run that never reaches the gate measures nothing at all. F38 states the bias plainly, and the alternative — a fixture that cannot produce a single observation — is worse.

**The finding stands on its own regardless of what P8 goes on to measure:** *"remove the repro steps so the gate fires" does not work on an agent with repository access.* Two redactions of increasing severity both failed. Whatever makes a triage gate fire in practice, it is not the absence of reproduction steps from the report.

## Decision 36 — the triage gate is forced, because §5.2's fixture design does not work

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 36 | METHOD-P §5.2's fixture is "3 E3 items with the repro steps deliberately removed **so the gate fires**". Measured, it never fires: (a) repro section withheld → reproduced in 754 s; (b) keep-list redaction leaving only title + problem statement (#28) → reproduced in 650 s; (c) items reselected to P7's three hardest (#35) → `issue-74`, which two Opus investigators failed to localise 0/6 times, **reproduced on attempt 1 in 480 s**. Three redactions, two selection rules, zero gate arrivals | **A: add `--force-gate`.** The gate's `when` predicate is overridden and **nothing else is**. The state written, killed, reloaded and digested is the investigator's real output from a real dispatch; only the decision to suspend at that point is the driver's. Every affected record carries `gate_forced: true` and the real `attempt_status` | §5.1 and §5.3 are not questions about the gate's firing condition. They ask whether a resume **into a mid-graph gate** is lossless, what it costs against F28's ~18.6k, and whether a re-reached gate mints a new mark (P9b A8). The resume machinery does not care why the gate opened. Forcing it measures the registered question; not forcing it measures nothing at all | P8 produces `resume_check` records. Every one is marked `gate_forced`, so no reader can mistake this for a naturally-occurring escalation | **yes** — a flag, default off |

**The disclosure that matters more than the flag.** This is P8's first real finding and it is about the pre-registration, not the rig:

> **"Remove the repro steps so the gate fires" does not work on an agent with repository access.**

An investigator holding `Read` and `Bash` reconstructs a reproduction from the symptom alone; the steps in the report are not what it was using. Note the asymmetry against P7: `issue-74` was localised **0 times in 6** Opus attempts but **reproduced on the first** haiku attempt. Reproducing a symptom and localising its cause are different tasks with very different difficulty, and §5.2 assumed the harder one gates the easier.

What this leaves open, and F38 must say so: **P8 has not observed a gate that fired on its own.** Every arrival is forced. So the arrival *rate*, and anything about when a real triage gate opens in practice, is unmeasured — while the resume mechanism behind it is measurable and is what §5.3 registered.

## Decision 37 — three defects in what the P8 emit record claimed, found on the first complete cycle

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 37 | The first resume record claimed three things it had not measured. **(A)** `second_arrival_minted_a_new_mark: false` conflated "the gate was not re-reached" with "it was re-reached and minted nothing" — the post-resume `attempt` returned `reproduced`, so the gate's `when` was false and it never fired again. Read literally that fires §5.4's third criterion and **reverses §9b's A8 on no evidence**. **(B)** `gate_forced` read the *resume* invocation's flag, not what the run suspended under, so a forced gate reported as unforced. **(C)** `ms_from_answer_to_first_post_resume_dispatch` was captured after `runner.run()` returned, recording **686,454 ms** — the dispatch's duration, not §5.3's latency to its start | **A: fix all three.** `second_arrival_minted_a_new_mark` is **null** unless the gate was actually reached, with `second_arrival_reached` and `post_resume_attempt_status` beside it; `gate_forced` travels in the persisted `GraphState`; the latency is timestamped before the dispatch | each is a claim the record made that the run did not support. (A) is the serious one: it would have overturned a schema decision | corrected records for runs 2 and 3; the first record is **superseded, not deleted** — the results file is append-only — and F38 names it. Its `lossless` and token figures were never affected and stand | **yes** |

## Decision 38 — P8's orchestrator waits on a fixed count, and stalls

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 38 | `run-p8-cycle.ps1` waits for `items.Count` gate marks before killing. When `issue-74` died at fixture build (`os error 32`, the recurring Windows handle-on-fixture problem) only 2 of 3 marks ever appeared, and the orchestrator sat until its 60-minute deadline with two processes blocked at their gates | **A: taken over manually for this cycle** and recorded; the script should wait on "all **surviving** processes at their marks", not a fixed count | measured: the cycle stalled ~30 min and the two completed runs show `gap_minutes` of 28.4 and 29.1 instead of ~0, so the arm label and the measurement diverged | F38 reports the measured `gap_minutes` per record and treats arm names as intent, not measurement. **Fix before the 65-minute arm**, where a stall costs an hour | **yes** |

## Decision 39 — D-FIXCONSUMER: the consumer is fixed, the runtime is NOT verified (added 2026-09-12, at Step 3a)

*Appended 2026-09-12 by the runbook's Step 3a, after this log's 2026-09-09 date, because s3's Checkpoint 3a
requires one of two artefacts and the other one cannot be produced. Dated inline; nothing above is renumbered.*

| # | decision | choice | grounding | consequence | reversible? |
| --- | --- | --- | --- | --- | --- |
| 39 | **D-FIXCONSUMER.** `forge-fix.js:982` read an ABSENT review verdict as approval (`if (review && review.pass === false)`), the shape F20 measures: a tool-restricted card silently disables `--json-schema`, `structured_output` is absent, and the process exits **0** with `subtype: success`. The consumer defect is settled. What is **not** settled is whether the **Workflow runtime** reproduces the same absent-verdict path that the CLI path does | **The consumer is FIXED; the runtime check is DEFERRED and its behaviour is therefore still UNVERIFIED.** Not a preference between two options: s3 §3a's probe is marked **`[WILL EXIST]`** and `grep -c 'fix-consumer-probe' spikes/runner/src/main.rs` returns **0** [measured 2026-09-12]. The rig carries `doctor`, `probe`, `ttl-probe`, `ttl-mark`, `ttl-read`, `corpus-manifest`, `gate-probe`, `reconcile-replay`, `review-probe`, `digest-probe`, `human-gate`, `classify-probe`, `graph-probe`, `repro-probe` and `rehydrate`, and none poses this question. Running it means first building a subcommand, which is not Step 3's work | Fix applied to `D:/repos/redkiln/workflows/forge-fix.js` **working tree only** — the predecessor is frozen at `47faec9f46b8d983c2c75f0794164e68415ae706` and every corpus read resolves at that SHA through the object store, never the working tree (AC-464), so the pin is undisturbed and the edit is **not committed** into the frozen history. Measured after: `review.pass !== true` = **2** (982 fixed + 1019 pre-existing), `review.pass === false` = **0** | **The runtime's behaviour is UNVERIFIED and must be reported that way, not as absent-equals-safe.** The generalised principle — absence is a block, never a pass (AC-282, AC-439, design §9.6's `spoke()` row) — is Phase-1 design, and **generalising it does not discharge this check**; s3 says so in terms. Anything relying on the Workflow runtime surfacing a typed absence is relying on an untested claim | **yes** — one haiku-scale dispatch through the Workflow runtime with a card omitting `StructuredOutput`, at `--timeout-secs 3600`, in the foreground (GC2/GC3/GC4/GC6), once the subcommand exists. PASS = a typed absence (a `StepError`, or an explicit null the caller must handle); FAIL = a value indistinguishable from an approval |

**Owed to the operator.** This row records a *forced* deferral, not an adjudicated one: the runbook offers two
branches and only one is executable today. It needs countersigning, or the dispatch needs scheduling. Instrument at
the time of writing, read from the file and not retyped: `2.1.269 (Claude Code)`.

Also recorded, because it cost two runs: **a resume launched through the harness's own background mechanism was killed with it.** That is F22 exactly — "background jobs die; a detached process does not" — and it had already been written down. Relaunched with `Start-Process`, both completed.
