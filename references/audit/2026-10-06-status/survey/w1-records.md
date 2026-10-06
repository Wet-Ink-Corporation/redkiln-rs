# W1 — Records health (redkiln-rs status-quo audit)

Snapshot: worktree `audit-wt`, commit `36dd8ac` (branch build/phase-0-to-1, 2026-10-05). Read-only; no cargo run. Paths are relative to the worktree. "RECORDS say" = the tracker/ADR/decision text; "CODE shows" is stated only where I checked a tree fact.

## 0. Verdict

The programme's records are unusually complete and internally cross-referenced: 198 ADRs (193 accepted, 1 proposed, 4 fully superseded), 184 id-labelled operator/session decisions, a 110-row audit ledger with 0 open rows, and a spend ledger the S-BIN driver re-sums mechanically. The weakness is not missing decisions but record decay and volume: the tracker's §2 rows are append-only sediment, so at least five rows still read OPEN or "owed" for items that later work closed (L2, L3, D17-4, D21-143, O-12..O-16), and about 20 AFK-provisional rulings (D21-106..128, D21-137) have no recorded operator confirmation. What is genuinely open for Phase-1 exit is small in decision terms (R-64, Record A / ADR-0197's flip, the unconfirmed provisionals) but large in operator-action terms: S-BIN B2/B3(/B4) go-aheads, all of 8j (never started, including paid probes, a fix-item pick and a first-ever CI push), then Step 8's two real lane runs. Recorded paid spend is about US$660 API-equivalent (floor; some early spikes have no figure) against an S-BIN cap of $90 of which $5.56 is spent.

## (a) ADRs

Method: `status:` frontmatter of each `references/adr/0*.md` (first 12 lines).

| status | count | note |
| --- | --- | --- |
| accepted | 193 | frontmatter `status: accepted` |
| proposed | 1 | ADR-0197 (`references/adr/0197-cards-models-and-the-run-environment-...md`, dated 2026-10-02); README: "Record A awaits s8j's probes" (`runbook/PROGRESS.md` row 8h) |
| superseded (in full) | 4 | 0004 (by 0074), 0041 (by 0064), 0079 (by 0090), 0088 (by 0101) |
| withdrawn | 0 | no ADR carries that status. ADR-0113 withdrew one clause of ADR-0078 (PROGRESS §2 F3 row) but did not change a status |
| superseded in part | 61 | accepted records whose `superseded_by` is non-empty (frontmatter scan). The scan counts any non-empty edge, so some of the 61 may be "elaborates"/"extends" edges recorded as `superseded_by`; I did not read each. [unverified split] |
| total | 198 files + README.md + ERRATA.md = 200 | `ls references/adr | wc -l` = 200 |

- Record dates run 2026-09-05 (0001) to 2026-10-02 (0197/0198). README.md:3 says "198 records", consistent with the frontmatter count. 73 were retrofitted from the older proposal at their source status (README.md:5).
- README's next free id is 0199 (README.md:73). Every record carries a `What this forbids` block (README.md:64, claims 198/198; I did not re-verify per record).
- ADR-0198 (RT-57 pack scalars, per-item envelope, fail-closed gate expiry) is accepted on D21-139 (operator signed value by value and row by row, AC-276) — `spikes/DECISIONS-2026-09-21.md:6269`.
- ERRATA.md: 745 lines, 84,642 bytes, 41 sections (one per corrected record), 51 `**E-nnnn-n**` entries. Convention (ERRATA.md:3-12): an accepted record is never edited; evidence/citation/number corrections go here, a correction that would move a Decision is a supersession instead. Themes, from the section headings and sampled entries:
  1. Early-record evidence mis-attribution (0001, 0012, 0019, 0056, 0067, 0074-0089: statistics attached to the wrong quantity, citations pointing at the wrong source, count drift).
  2. Policy-ADR evidence lines (0083 toolchain/lint, 0084 dependencies, 0085 conventions, 0100 dependency pins, 0077 gix read path).
  3. Count/register drift (AC-520 section, README tallies).
  4. Build-time corrections to the Phase-1 implementation records 0132-0168 (step list reaching the runtime, port call-site lint scope, tool-verb routing, guard resolution, lane loop, trailer cardinality, severity vocabulary). That is, the records had to be amended once the code met them.
  Positive signal: the corrections are dated, cited and kept out of the Decision text. Negative signal: category 4 shows records written ahead of the code needing correction on contact.
- Supersession graph: PROGRESS §6 (`runbook/PROGRESS.md` §6, census_adr row) reports edges reciprocal and acyclic at the 2026-09-11 tree; I did not re-run `references/audit/2026-09-10/scripts/census_adr.py` here (no execution outside git reads). [unverified at 36dd8ac]

## (b) Decision files

Files: `spikes/DECISIONS-2026-09-{09,11,12,14,15,17,18,20,21}.md` — 8,805 lines total; the 2026-09-21 file alone is 6,865 lines (78%) (`wc -l`).

| file | content | count |
| --- | --- | --- |
| DECISIONS-2026-09-09 | AFK spike-session instrument decisions, numbered | #1-#39 (tracker C-06: "37 decisions", #38/#39 added later; 26 are instrument decisions with homes in METHOD files, `runbook/PROGRESS.md` §5 C-06) |
| DECISIONS-2026-09-11 | post-audit decisions, numbered table | ~27 rows (`grep` of the Decisions table) |
| DECISIONS-2026-09-12 | runbook alignment review, E-rows | E1-E13 (PROGRESS §2: "All thirteen have landed") |
| DECISIONS-2026-09-14 | D-rows | D-1..D-14 |
| DECISIONS-2026-09-15 | D15-1..D15-4 | 4 |
| DECISIONS-2026-09-17 | D17-5..D17-13 plus RT/O/R | 9 id-headed |
| DECISIONS-2026-09-18 | D18-1..D18-19 | 19 |
| DECISIONS-2026-09-20 | D20-1 | 1 |
| DECISIONS-2026-09-21 | D21-1..D21-151 | 151 (`grep -c '^## D21-'` = 151) |

Id-headed decisions in total: 151 + 19 + 9 + 4 + 1 = 184, plus roughly 100 numbered earlier-format decisions. Date range 2026-09-09 to 2026-10-05 (the D21 title says "... 2026-10-06" because D21-151 is stamped 2026-10-06 UTC; the commit is 2026-10-05 20:21 -0700, `git show 36dd8ac`).

Most recent 15 (D21-137..151), one line each (headings at `spikes/DECISIONS-2026-09-21.md`, lines 6031-6854):
- D21-137 (s8h): the child's settings go as a per-dispatch file `--settings <path>` outside the worktree; ADR-0197 decision 9 amended. TAKEN PROVISIONALLY.
- D21-138 (s8h close): cards bind least privilege, models pinned by full id, schemas read from `EXPORTED`, one launcher is the only entry to a real run; Records A and B stay proposed. Checkpoint 8h passed `d24d15a`.
- D21-139: operator signs ADR-0198 (Record B) value by value; it flips to accepted.
- D21-140: operator raises S-BIN's spend cap from $30 to $90 and routes the corpus build through a private copy and patch.
- D21-141: Checkpoint 8i (8)'s type scan blanks string literals; later REJECTED by the operator for option (B), a counted two-line exemption owed before the formal 8i (`runbook/PROGRESS.md` §2 D21-141 row).
- D21-142: operator registers a second CLAUDE.md strip rule and picks seed 5's replacement commit R = `6a101bdcd`.
- D21-143: the S-BIN go-ahead ledger entry; B0 signed 2026-10-05T03:06Z and B1 signed 2026-10-05T04:18Z (DECISIONS lines 6575-6577); B2/B3/B4 lines not yet written.
- D21-144: the out-of-tree audit is a hardened measured denylist with an allowlist shadow audit; CONFIRMED by operator 2026-10-03 (panel was 2-1).
- D21-145: the reviewer's own background-task output is exempt from the out-of-tree void; CONFIRMED.
- D21-146: operator rules the review trees' editor launcher neutralised in all five trees before B1.
- D21-147: one identical generated backlog stub in all five trees; a surviving UnrealTraceServer is not Halt 6.
- D21-148: tree 5's red greenness run is an instrument defect (Monolith indexer), not a red seed; base 5 rebuilt with the later verify-script blob.
- D21-149: at a §8i.6 stop in B1, a `capture_scene_preview` rejection yields an empty capture list; needs a driver change, new dry run and re-signed B1 before B2.
- D21-150: item 5's capture session stalled (Monolith dropped under memory pressure, $0.76, no capture); one retry on a fresh copy.
- D21-151: owed item (f): the driver checks free space before every dispatch copy.

Observation: 15 decisions in about 4 days (2026-10-02 to 10-05) are almost all S-BIN instrument/fixture hygiene, not product behaviour. The decision stream has been dominated by measurement-harness defence for roughly the last third of the file (D21-141..151 are all 8i).

## (c) Open items

Classification key: BLOCKS = must resolve or be consciously waived before Checkpoint 8 / Phase-1 exit; NOT-BLOCKING = recorded as deferred, Phase-2/4, or audit hygiene.

### Stage-level (what the tracker itself says)
| item | state per records | cite | class |
| --- | --- | --- | --- |
| Step 8i (S-BIN) | IN PROGRESS. B0 and B1 done; B1 = 1 haiku probe + 6 capture sessions, $5.561026 spent. Remaining: B2 (5 Opus), B3 (5 Opus), retries (<=4), B4 (<=4, only if METHOD-S §3.4 branch 2) | `runbook/phase-1/s8i-s-bin.md` §8i.5 spend table; commit `ee44bf1` | BLOCKS (Checkpoint 8i gates 8j) |
| 8i owed before B2 | driver change for D21-149 (empty capture list), new zero-spend dry run, re-signed B1 line; item 5 retry on a fresh copy (D21-150); owed item (f) free-space check (D21-151, ruled, build pending); owed item (e) fixture completion (spec.md/report per item, review repos `D:/sb/rev/item-n`, `fixture.json`, `ceiling_secs`); (g) item-4 gate; (h) remaining housekeeping; D21-141 (B) counted exemption before formal 8i | `s8i-s-bin.md` ~2027-2133; DECISIONS D21-141/149/150/151 | BLOCKS |
| 8i formal close | Checkpoint 8i whole run, `Binary` ADR flip on the operator's dated ledger line, F41/F42 FINDINGS index rows, AC-475 lift criterion, two-commit close | `s8i-s-bin.md` §8i.10 | BLOCKS |
| Step 8j | PLANNED, "Not started" (tracker row 8j). Scope: fixture proofs of both lanes under FakeHarness; human roots; staged first run hold; fix-item shortlist and operator pick (§8j.5); probe session (pin move, 7c split re-run, `s8h` probes 0/1/2, each an operator go-ahead; §8j.6); CI green on the first-ever push (§8j.7: "The branch has never been pushed ... `ci.yml` has never run"); `s8` instrument amendments (§8j.8) | `runbook/phase-1/s8j-run-preparation.md` (2,816 lines) headings at 326-2010 | BLOCKS |
| Step 8 | RE-OPENED 2026-09-24; Checkpoint 8 has NOT passed; M5 not reached. Six runnable exit checks (1)(2)(3)(4)(6)(7); (5) withdrawn, number retired | `runbook/phase-1/s8-exit-criterion.md:15-19`; tracker row 8 | BLOCKS (it is the exit) |
| "provisional, re-run pending `s8j`" marks | 7c, 7f, 8a, 8c, 8d carry re-run-pending marks that `s8j`'s entry-chain re-run clears | tracker §1 rows 7c, 7f, 8a, 8c, 8d | BLOCKS (mechanical; part of 8j) |
| ADR-0197 (Record A) flip | proposed; flips only after `s8j`'s live probes pass and the operator signs decision 7's table enum by enum (AC-156) | `spikes/DECISIONS-2026-09-21.md` D21-138 (~6119-6268); `references/adr/0197-*.md` | BLOCKS (8h's records held) |

### Named items from the brief
| item | state | cite | class |
| --- | --- | --- | --- |
| R-24 | ADR-0113 d3 subagent-bucket question still open after S-TTL1; needs one dispatch of a card granting a subagent tool, owed to "whichever stage first ships such a card (7e's packs)". 7e closed 2026-09-21; I found no record that it was run | `spikes/DECISIONS-2026-09-17.md:337` | NOT-BLOCKING |
| RT-27 | AC-204 adoption reading TAKEN 2026-09-18 (composite `agent`/`agents`+`model`+tools); AC-204 PROVISIONAL on it; "register amendment OWED". Code: s8h later settled "RT-27" in the guard reading (DECISIONS D21-137 text near line 6190: "RT-27 settled; `model_belongs_to_tier` deleted"). Whether the register amendment landed: not verified | `runbook/PROGRESS.md` §2 RT-27 row (line 181); DECISIONS-2026-09-21:6190 | NOT-BLOCKING (verify) |
| R-23 | AC-369's sqlite half partial; root commit branch-dependent; "record OWED" for moved-repository policy, harness-version column, multi-root rule | PROGRESS §2 R-23 row (line 179); `s7c` §7c.13/§7c.22 | NOT-BLOCKING (Phase-1 exit does not read it) [inferred; no check names it] |
| R-64 | whether `doctor --rebuild` re-derives every tag from git alone; "UNMEASURED"; bears on the `rebuild_differential` test authored at Step 8 | PROGRESS §2 R-64 (line 324); ADR-0131 decision 10 | BLOCKS-ish: Step 8 check (2) reads AC-184's rebuild differential (`s8-exit-criterion.md` ~72-85); if tags cannot be rebuilt the differential must exclude them, which ADR-0131 d2 already allows |
| Arm L labels | arm L (P1 LLM-context-vector) labels DEFERRED by decision to a new corpus pin; reports "DEFERRED" through the three-state form, never a pass | `spikes/DECISIONS-2026-09-11.md` #6; INDEX.md "Where the human is needed" | NOT-BLOCKING ("a decision, not a blocker") |
| S-SUBTLE-CB | UNRUN; AC-312 PROVISIONAL on it; ledger text: "no document may claim that gate is guarded against false approval until S-SUBTLE-CB runs. It has not." | `spikes/DECISIONS-2026-09-21.md:235,1461`; AC register AC-312 (`references/2026-09-08-acceptance-criteria.md:4735,5231`); referenced at s8j:845 | NOT-BLOCKING for exit; blocks any claim about review-gate calibration |
| S-CODEX | Codex adapter is Phase 4 "after S-CODEX" (ADR-0167 accepted, D21-53); the Codex item is retired from 8j (s8j §8j.1, line 401) | `spikes/DECISIONS-2026-09-21.md:2168` | NOT-BLOCKING (Phase 4) |
| D-FIXCONSUMER | COUNTERSIGNED 2026-09-13; Workflow-runtime behaviour stays UNVERIFIED, probe not scheduled; Step 8 check (1) is Rust-only so it does not bite | PROGRESS row 3; INDEX.md "Where the human is needed" (struck-through row) | NOT-BLOCKING (closed) |
| S-SPF | registered, unrun (D17-6) | PROGRESS §2 D17-6 (line 169) | NOT-BLOCKING |
| S-CLASS-REPLAY | AC-284 PROVISIONAL until it runs | AC register state line (`...acceptance-criteria.md:4735`) | NOT-BLOCKING |

### Audit-follow-up residue (PROGRESS §5): never run
M-05/M-14 (second blind refuter), M-08 (known-issue proximity), M-03 (Wilson recompute), M-04 (481-id GWT similarity), M-07 (re-run unrun scorers), M-09 (preprint full-text grep), M-10 (refuter over best-practice rows), C-03 (decidability sample), C-07 (read 30 unread ADRs against citation sites), C-08/M-13 (doc-map lens), and C-02 (slice finder over AC §3.1-3.6, "before Step 7b" — Step 7b passed 2026-09-15; not recorded as run) — all "not run" (`runbook/PROGRESS.md` lines ~493-505). NOT-BLOCKING, but they are the audit's own listed limits and none was ever dispositioned.

### Provisional (AFK) rulings awaiting operator confirmation
Under D21-105 (2026-09-28 standing AFK instructions) the session took rulings "PROVISIONALLY, pending the operator's confirmation". Tracker rows with PROVISIONALLY and no confirmation text: D21-106, 108, 109, 110, 111, 112, 113, 114, 115, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 137 (20 rows; `runbook/PROGRESS.md` §2 scan). D21-107 was confirmed (wi-838696 A), D21-62/63 confirmed by D21-64, D21-141/144/145 ruled by the operator 2026-10-03. D21-129's close says "none of D21-105 ... D21-128 records an objection" (DECISIONS 5853-5888), and D21-138 asks the operator to "confirm or reverse D21-137" (6240); no later entry in the file records that confirmation. Several ADR flips (0191, 0194, 0196 etc.) were conditioned on "operator has not objected", so silent non-objection is carrying accepted records. Class: NOT-BLOCKING mechanically, but a ratification debt that touches accepted ADRs.

### Stale tracker rows (records say open; later work closed them)
- L2 (AC-520 scored record "owed to Step 7b", PROGRESS line 119): closed — Checkpoint 7b's row says the pin moved to `61f590e5`, "carrying AC-520's scored precision record, AC-071's stability record and the golden-logs tree" (PROGRESS line 605; `CORPUS_PIN` = 61f590e5).
- L3 (`rk-store/src/conn.rs` poison `into_inner`, "owed — rk-store; unscheduled", PROGRESS line 123): closed — `crates/rk-store/src/conn.rs:136-140` now documents a halting path and the `PoisonError` recovery was removed at `da5dff8` (s8e, 2026-09-28; `git log -S` on that file).
- D17-4 ("OPEN — 7c, with a record", PROGRESS line 167): closed by ADR-0191's `verify_completed` (DECISIONS-2026-09-21:1678; s8g file line 961 "D17-4 closes in the tracker commit" — the tracker row was never updated).
- D21-143 (PROGRESS line 310 "OPEN, drafted 2026-10-03"): B0 and B1 lines are signed (commits `a4c4700`, `e430c95`).
- O-12..O-16 (PROGRESS line 320 "OPEN 2026-09-18"): D18-19 (O-16) was taken 2026-09-19 (PROGRESS line 200) and 7d closed; the row is stale at least in part. [partly unverified]
- Tracker §1 row 8i's last dated note is 2026-10-04 (D21148-8i-B); the B1 results (6 capture sessions, $5.56) and D21-149..151 are in the stage file and DECISIONS but not in the row. (An uncommitted PROGRESS.md edit exists on the live branch, so this may be being fixed; not visible in this snapshot.)

## (d) Spikes and recorded paid spend

Spend is `total_cost_usd`, i.e. API-equivalent notional on a subscription account (FINDINGS.md:118; s8i spend table says "API-equivalent"), not cash.

| spike / experiment | outcome (records) | spend | cite |
| --- | --- | --- | --- |
| E3 (in-session vs headless review), 92 dispatches | kill criterion does NOT fire; detection 23/23 vs 23/23 | $153.86 ($97.39 arm a + $56.47 arm b) | `spikes/FINDINGS.md:1208,1262` (F23) |
| E3-H (two harder tiers), 20 dispatches | 20/20, prediction falsified twice, line closed | $36.18 | FINDINGS.md:1550 (F26) |
| H1/H4 (gate cost, digest fidelity) | gate costs a bounded 18.6k tokens; digest sufficient | $18.85 | FINDINGS.md:1771 (F28) |
| S-VAR (60 CLEAN dispatches) | no defect class stable-reject; verdict is a weighted coin | $176.96 | `spikes/F29-DRAFT.md:8` |
| S-TYPED (52 dispatches) | typed contract costs no detection; blocks 9/23 CLEAN with 0 verified FP; cost +74% vs prose | $112.25 | `spikes/F31-DRAFT.md:9,166` |
| S-H1b/S-WT (16 haiku) | worktree fan-out free; timed gate costs whole prefix; h1b-24h read still not taken | $0.19 | `spikes/F30-DRAFT.md:16` |
| P1 arm L v1+v2 (400 dispatches) | LLM-emitted context vector not stable; kill fires 4/4 | $21.37 ($10.66 + $10.71) | `spikes/F36-DRAFT.md:126-130`; HANDOFF.md:229 |
| P7 (61 graph runs) | quality block 2.64x cost, changes root cause on 1/10, policy check constant | $131.62 | HANDOFF.md:229-231; F37-DRAFT.md |
| P8 (3 resume cycles, PARTIAL) | resume lossless 3/3; 65-min/24-h arms and controls unrun | $3.01 | `spikes/F38-DRAFT.md:5` |
| P1 arm D / P2 / P9 / P9b / P6 / S-GRAMMAR / S-PORTS / S-GIX a-b-c | arm D: no derived facet clears 0.95; P2: no staleness in corpus, 2/3 kill criteria unanswerable; P9/P9b/P6 authoring: run (P6 catalog 0.3% after decisions); S-GRAMMAR 285/287 parse (F42); S-PORTS 5/5 + 2/2 compile_fail (F41); S-GIX a go, b NO-GO, c go | $0 (offline) or not stated | HANDOFF.md:196-215; F39/F40/F41/F42 drafts; PROGRESS line 562 |
| D21-44 probe (worktree `.git` write denial) | run-owned worktrees move out of `.git` | $0.123 | PROGRESS §2 D21-44 (line 211) |
| S-SEV (Jev as rater) | rater of record per D21-58; ADR-0168 settled severity `blocking|advisory`, no threshold | ~US$0.01 (operator-authorised) | `spikes/METHOD-SEV.md:368`; D21-58/59 |
| Step 7c spikes S-CANCEL-WIN, S-TTL1, S-RETRY | run; S-TTL1 subagent bucket unobserved (R-24); S-RETRY exhaustion arm NOT MEASURABLE | no figure found | D17-6; PROGRESS line 613 [unverified cost] |
| S-BIN B1 (1 haiku probe + 6 capture sessions) | probe 1 passed all three properties; item 2/3 captures rejected by tool, item 5 stalled | $5.561026 ($0.021026 probe + $5.54 captures) | `s8i-s-bin.md` §8i.5 dated 2026-10-06; commits `6dcaf57`, `ee44bf1` |
| S-SUBTLE-CB, S-CODEX, S-SPF, S-CLASS-REPLAY, P1 arm-L labels, P8 arms | UNRUN / deferred | $0 | see (c) |

Total of recorded figures: 153.86 + 36.18 + 18.85 + 176.96 + 112.25 + 0.19 + 21.37 + 131.62 + 3.01 + 0.123 + 0.01 + 5.561 = about US$660. This is a floor: the early S0.1/S0.2/E0/E1/E2/H2/H3 spikes, the 7c live probes and the P9/P6 authoring passes have no recorded dollar figure that I found. Earlier recorded caps: $300 (HANDOFF session, spent $156.00), $200 (DECISIONS-2026-09-09:43), $30 (D21-105 for S-BIN) raised to $90 (D21-140). S-BIN remaining headroom: $90 - $5.56 = $84.44; the 8i table itself warns B3/retries/B4 "may not fit" at the planning bounds (§8i.5; B4 does not fit unless spent so far < 70).

## (e) Human-owed obligations (named in the records)

1. S-BIN B2 go-ahead line (5 Opus, up to ~$25 at bound), then B3, any retries, and B4 only on METHOD-S branch 2 — each a signed line in D21-143 (s8i §8i.5 "The go-ahead line"); and a re-signed B1 line after the D21-149 driver change.
2. Operator-run steps from a plain Git Bash outside Claude Code: every real dispatch goes through `tools/live-run.sh` (8h §8h.7; 8j §8j.6); the operator records each result in `runbook/phase-1/s8j-evidence.txt` ("No agent writes a pass line").
3. 8i operator hands: five greenness runs done; remaining: item 5 capture retry, any further UE capture sessions, housekeeping (h), owed item (g) item-4 gate, the (B) counted exemption decision for Checkpoint 8i (8).
4. 8j: pick the fix item from a shortlist of >= 3 behavioural defects (D21-42 18; §8j.5); approve the pin move/CLI freeze and DISABLE_AUTOUPDATER setting; go-ahead lines for probes 0, 1, 2; confirm the first-ever push to a remote ("Each push is confirmed with the operator", §8j.7).
5. Sign Record A (ADR-0197) decision 7's table enum by enum (AC-156) and its flip after probes pass; (Record B already signed, D21-139).
6. Confirm or reverse the ~20 provisional AFK rulings (D21-106..128, D21-137) and the open D21-128 ruling (left open for the operator).
7. D21-42 14a: CI green on the pushed branch, both OS legs, as a Checkpoint 8 condition.
8. Twenty human-gated AC rows across Phase-1 stages (INDEX.md, "Where the human is needed"; AC-309, AC-311 among them) — excluded from stage acceptance sets by name; they gate the criterion, not the build.
9. Deferred by decision, not owed for exit: arm L labels, S-SUBTLE-CB draw, the h1b-24h read.
10. Done and closed (for contrast): M-06 operator label file 47/47 (0 disagreements), E3 severity countersign, D-FIXCONSUMER countersign, PIA ledger 110/110 dispositioned (60 recorded, 50 done, 0 open in PROGRESS §4 scan).

## Completeness estimate

Of the decisions the records say are owed for Phase 1: 110/110 audit concerns dispositioned; 184 id-labelled decisions plus ~100 numbered earlier ones taken; open decision-shaped items are R-64, Record A's signature, the ~20 unconfirmed provisionals, D21-128, and the minor R-23/RT-27 record work. Treating the ~20 provisionals and ~6 other open items as the unresolved set against roughly 190 counted decisions gives about 86% resolved. The larger remaining volume is operator actions and runs (section e), not decisions.

## Unverified
- Per-record split of the 61 "superseded in part" edges (in-part vs elaborates/extends).
- Whether `census_adr.py` and the other census scripts still pass at 36dd8ac (execution not permitted).
- Whether RT-27's register amendment and R-24's one-dispatch run ever landed.
- Spend for S0.1/S0.2/E0/E1/E2/H2/H3, the 7c live probes, and P6/P9 authoring; and the Step 7c spike costs.
- Whether D21-119..127 and D21-137 were confirmed verbally outside the ledger (the ledger text shows no confirmation).
- The live branch has uncommitted edits to PROGRESS.md, s8i-s-bin.md, DECISIONS-2026-09-21.md and METHOD-S.md that are not in this snapshot; some staleness noted above may already be fixed there.
