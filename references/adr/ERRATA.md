# Errata to accepted records

**Convention:** ADR-0092 decision 1. An accepted record is never edited; a correction to an evidence line, a citation, a
number or a characterisation that leaves every sentence of `## Decision` and every `**What this forbids**` bullet
standing is recorded here, one section per record, ordered by record number. A correction that would move a Decision
is a supersession and does not belong here. Each entry carries: the frozen sentence quoted verbatim with its line at the
pin it was read at (`009a4ec` unless stated); what is true, with the source, its fetch date and an evidence tag; what
the correction does **not** change; and the id that raised it. The record's index row in `README.md` carries the
marker `(errata)` so a reader knows to look here.

Lines are as at HEAD `009a4ec` (2026-09-10). Nothing below changes a Decision.

---

## ADR-0001 — Inversion architecture bet

**E-0001-1 (2026-09-11, PIA-073).** Line 56: "Headless dispatch parity: 23/23 vs 23/23 detection, 20/20 across two harder
tiers, McNemar p=0.625 (F23, F26)." — **What is true:** the McNemar p = 0.625 is computed on the CLEAN false-positive
discordant pairs (3 vs 1), not on detection; detection 23/23 vs 23/23 has no discordant pairs and admits no test.
[measured, e3.jsonl recount] Parity stands; the test is attached to the right quantity. Decision unchanged.

## ADR-0012 — Open questions become named gating spikes

**E-0012-1 (2026-09-11, PIA-102).** Two relations the README leaves undeclared, recorded here so a discharge-by-id grep
resolves: ADR-0054 *extends* ADR-0040's closed input-source grammar (not a supersession); ADR-0088 (now ADR-0101)
*elaborates* ADR-0074 clause 7 and adds the unrun/sizing obligations (not a supersession). [documented, the records'
own bodies] Decision unchanged.

## ADR-0019 — The per-AC discriminating test is the norm

**E-0019-1 (2026-09-11, PIA-065).** Line 51: "[measured] 0/156 historical reports carry the discriminating-mutation
columns (§4.6 item 8); n=156, the full cited corpus." — **What is true:** the 0/156 count is stated in the 2026-09-05
proposal §4.6 only and is not corroborated by the evidence brief; AC-317's own evidence line already says so. The
figure is re-tagged **[documented, proposal-only; not re-verified]**. The figure that *is* measured is F31's ≥ 12/23
CLEAN items whose defects escaped a green suite, which the registers had marked "unverified" — that one is
**[measured, F31]**. Decision unchanged (it rests on F25's mechanism, not on the count).

## ADR-0056 — Family totality caps rank escalation

**E-0056-1 (2026-09-11, PIA-104).** Line 78: "[measured] The lattice run (`F35-DRAFT.md` scratch run) found
unsatisfiability rose from 11.4% to 30.1% with ceilings" — **What is true:** 30.1% occurs in no F-draft or
`results-p6-*.txt`; its only source is `spikes/p9/DECISIONS-2026-09-08.md` line 5, a "scratch measurement" with no
stamped output and no digests. Re-tagged **[documented, DECISIONS-2026-09-08 scratch, no stamped output]**. Decision
unchanged (the ceiling rule rests on the mechanism, not the rate).

## ADR-0067 — recent_deploy is a derived facet

**E-0067-1 (2026-09-11; the README §Open items note of 2026-09-09, moved here).** Lines 29 and 48: "P1 arm D measures
it." — **What is true:** arm D was scored on 2026-09-09 [measured, F39] and `recent_deploy` does **not** appear in its
precision table: the `toolchain:deploy_log` feed does not exist at pin `47faec9f`, so the facet was emitted `null` under
`R-recent-deploy-feed-absent` on every item and is **NOT MEASURABLE** there. The forward reference is unfulfilled and
ADR-0093 records the facet as derived-with-abstention. Decision unchanged.

## ADR-0074 — Review gate: reviewer proposes, machine confirms

**E-0074-1 (2026-09-11, PIA-087).** Line 100: "Clause 6 has **no external prior art beyond GitLab flaky-quarantine and
Cloudflare 'won't fix'** … `Quarantined` is original process design". — **What is true:** OASIS SARIF 2.1.0 §3.35
defines a `suppression` object with `status` ∈ {`accepted`, `underReview`, `rejected`} and a `justification`; Snyk's
`.snyk` ignore carries `--reason` and `--expiry` (default 30 days) — field-for-field `{reason, until}`; GitHub
code-scanning dismissal records a reason and is reopenable; SonarQube has False Positive / Won't Fix / Accepted;
Semgrep has triage states. `Quarantined` is **convergent**, not original. [documented, fetched 2026-09-10] A design
spike **D4-SARIF** (consumer: the review-gate schema under ADR-0096) asks whether `Quarantined` maps onto
`suppressions[].status`. Decision unchanged.

**E-0074-2 (2026-09-11, PIA-095, PIA-097).** Line 95: "CLEAN blocks 12/23 strict, 9/23 lenient; **14/14 adjudicated
findings confirmed**; **0/23 verified false positives**" — **What is true:** the block counts and sink seeds were
produced by `policy_verdict` (`spikes/runner/src/e3.rs:254-256`) calling `policy.blocks(evidence)` on the
**model-emitted** label — the routing clause 2 rejects — so they are measured under the model-label policy and the
Rust-computed class is unmeasured until S-CLASS-REPLAY runs (ADR-0096 decision 3). [measured, e3.rs] Also, where any
downstream restatement credits "F24" with ≥ 13/23: F24 said ≥ 5/23; the 13 is F31's. Decision unchanged.

**E-0074-3 (2026-09-11, PIA-073).** Line 96 (the E3 arm-(a) MCC): "+0.978" — **What is true:** **+0.957**, from the
matrix tp 23 / fn 0 / fp 1 / tn 22, as F21 computes its own +0.917. [measured, e3.jsonl recount] Decision unchanged.

## ADR-0075 — Typed artifacts, revision-4 refinements

**E-0075-1 (2026-09-11, PIA-078).** Lines 19 and 90: "Spec sections lift at **98% overall**" — **What is true:** 98% is
the *spec-section slot only*; the record-weighted typed-slot overall is **96.0%** (3,011 / 3,138 — Lifted / (Lifted +
LiftedWithLoss + Failed) over typed slots; 98.3% only by counting LiftedWithLoss as lifted). The instrument was
`spikes/results/census-s-mig.py`; its per-AC loss list was cap-truncated at `loss[:6]`, so "45/46" restatements read
**76/87** loss lines (75/85 distinct ids; 131 ACs with a loss; 29 lacking both fields; of 208 in 19 specs) — 19/153
and 88% are unaffected. [measured, uncapped re-run at the pin] Decision unchanged.

**E-0075-2 (2026-09-11, PIA-006, PIA-033).** The amendment note's "19/45 (42%)" companion figure is over a five-kind
subset; the census holds eleven `companion.*` kinds plus `ledger` — read "five of eleven companion kinds". Migration
never populates `gwt` or `verification`; both are authored from the first CLI write (AC-137). [measured, s-mig.jsonl]
Decision unchanged.

## ADR-0077 — gix owns the read path; git owns the write path

**E-0077-1 (2026-09-11, PIA-042).** Line 112: "Shallow clone at a specific commit is supported — [Pr] gitoxide
discussion #2309" — **What is true:** discussion #2309 shows a panic; the API is `PrepareFetch::with_revision` +
`with_shallow`, merged in PR #2824 (2026-08-05). [documented, github.com/GitoxideLabs/gitoxide, fetched 2026-09-10]
Line 111: "`crate-status.md` documents trailer parsing" — **What is true:** `crate-status.md` lists "parse trailers" as
an *unchecked* box, but `BodyRef::trailers()` exists in gix-object 0.64.1, so the read half of S-GIX/b is discharged
[documented, docs.rs, fetched 2026-09-10]. The write half stands as registered. ADR-0098 acts on both. Decision
unchanged.

## ADR-0079 — Context-vector source undecided (proposed)

*(A `proposed` record; ADR-0090 supersedes it at Step 4.4 and carries these.)* **E-0079-1 (2026-09-11, PIA-020,
PIA-077).** Its external corroboration overstates three preprints: arXiv 2603.04417 is Sonnet-4.5 and arXiv 2604.22411
Sonnet-4, and neither states a "schema-induced" mechanism; arXiv 2511.01176's boundary sentence is an unmeasured
premise; arXiv 2508.16445 v2 is retitled and makes no determinism claim. Line 211's F36 cost parenthetical "$9.70" is
**$10.71** (haiku $4.91 + sonnet $5.79). [documented, full text fetched 2026-09-10; measured, F36 log lines]

## ADR-0080 — The quality block's policy predicate is not a terminal condition

**E-0080-1 (2026-09-11, PIA-073).** Lines 52, 104 and 192 restate "+0.978": read **+0.957** (tp 23 / fn 0 / fp 1 /
tn 22). [measured] Decision 5's sizing is superseded by ADR-0111; decisions 1–4 unchanged.

## ADR-0081 — The rigor ladder must be monotone in internal quality

**E-0081-1 (2026-09-11, PIA-092).** Line 190: "no external source frames activity selection as a research question at
all, and names that as a positive gap" — **What is true:** with the qualifier "LLM-driven, per-run, from internal
quality signals" restored, the nearest fields are Situational Method Engineering (Henderson-Sellers, Ralyté, Ågerfalk,
Rossi, Springer 2014 — the OPF deontic matrix ADR-0020 :23 already cites) and agentic workflow optimisation (arXiv
2603.22386, 2026-03-23: AFlow, G-Designer, FlowReasoner, Workflow-R1); no SDLC-tailoring system was found (arXiv
2509.13942 uses fixed Waterfall/V/Agile scaffolds). [documented, fetched 2026-09-10] Decision unchanged.

## ADR-0082 — The advisory sink is a first-class destination

**E-0082-1 (2026-09-11, PIA-087).** Lines 29 and 90 (prior art "No") — see E-0074-1; convergent, not original.
**E-0082-2 (2026-09-11, PIA-095, PIA-097).** Line 86 (F31 block counts) — see E-0074-2. Line 93 — where "four
survived fully green": two adjudicated defects escaped through fully green suites (issue-70 AC-001 1,890/1,890;
issue-83 AC-002 1,907/1,907) and two (issue-131) with zero new failures against an already-red 54-failure baseline.
[measured, F31] Decision unchanged.

## ADR-0083 — Rust workspace, toolchain and lint policy

*(Partially superseded by ADR-0102, which carries the evidence-line corrections in its own body: the nursery clause is
not a Clippy quotation; "pin for a binary you control" is RFC 3537's framing, swatinem's post corroborates; the
thiserror/anyhow split is documented by the crate author and by Microsoft M-APP-ERROR.)* **E-0083-1 (2026-09-11,
PIA-023).** Line 116's MADR characterisation: MADR's minimal cut keeps Considered Options and has no frontmatter;
"updated over time" is MADR's *Consequences/Confirmation* allowance; Log4brains rejects NNNN numbering
(`YYYYMMDD-slug.md`) and infers metadata from raw text plus git log — cite adr-tools alone for NNNN. [documented,
github.com/adr/madr; log4brains docs, fetched 2026-09-10]

## ADR-0084 — Dependency policy

*(Partially superseded by ADR-0095 and ADR-0100.)* **E-0084-1 (2026-09-11, PIA-083).** Line 19: "RUSTSEC-2025-0021 …
(`gix-features`, missing SHA-1 collision detection)" and the §9.2 table's "Fixed ≥ 0.71.0" — **What is true:**
patched at **gix-features ≥ 0.41.0**; 0.71.0 is the `gix` facade's fixed version (the GHSA lists 27 packages).
RUSTSEC-2024-0351 is `gix-ref` ≥ 0.44.0 and -0353 `gix-worktree` ≥ 0.34.0; -0349's surface is "crafted tree or blob
names" (`gix-worktree` ≥ 0.34.0, `gix-worktree-state` ≥ 0.17.0); RUSTSEC-2024-0352 (`gix-index`) is omitted under the
existing "not complete" flag. Two later advisories: GHSA-2vh6-hw4j-32ww (`gix-packetline` ≤ 0.21.4) and
GHSA-7rhf-42qf-vrvc / CVE-2025-24890 (`gix-sec` ≤ 0.13.2, patched 2026-09-02). Every pin is clear: gix 0.87.1 pulls
gix-features ^0.49.1, gix-ref ^0.67.1, gix-worktree ^0.56.0. [documented, rustsec.org / osv.dev, fetched 2026-09-10]
Line 115: "no advisory affects `bundled`" — **What is true:** RUSTSEC-2022-0090 / CVE-2022-35737 was against the
bundled SQLite in `libsqlite3-sys` < 0.25.1 (three advisories in the rusqlite family); rusqlite 0.40.2 pulls
libsqlite3-sys ^0.38.2, unaffected. Line 19: gix "multi-maintainer … not a single-author risk" — multi-contributor,
**one crates.io owner** (Byron). [documented, crates.io, fetched 2026-09-10] Decision unchanged.

**E-0084-3 (2026-09-11, audit grounding EXT-CRATEVER-019/-026).** The "Verified versions, as at 2026-09-08" line: `jsonschema` 0.55.1 is
now **0.56.0** (2026-09-10; five releases in eleven days — the reason ADR-0100 exact-pins it) and `release-plz` 0.3.163 is **0.3.165**
(0.3.164 on 2026-09-09, 0.3.165 on 2026-09-10). Both were correct on the read date; thirteen of the audit's 860 rows went stale within 48
hours, which is why the runbook's Step 0 re-verifies pins on the day. [documented, crates.io, fetched 2026-09-10/11] Decision unchanged.

**E-0084-2 (2026-09-11, PIA-005).** Line 117: "no limits table was found on the platform structured-outputs page,
which truncated on fetch" — **What is true:** the page carries a "Schema complexity limits" table (24 optional
parameters, 16 union-typed parameters, 20 strict tools per request, 180 s compile timeout; per request) — present on
2026-09-10 and in a Wayback capture of 2026-09-07. ADR-0095 acts on it.

## ADR-0085 — Documentation and traceability conventions

*(Partially superseded by ADR-0092.)* **E-0085-1 (2026-09-11, PIA-086).** Lines 30 and 112: rules 4 and 6 have "no
external norm behind them at all" — **What is true:** NASA SP-2016-6105 Rev 2 Appendix C (TBD/TBR items carry
rationale, owner, closure date) is the antecedent rule 6 extends, and Appendix D's Requirements Verification Matrix
(Table D-1, p. 241; Table 4.2-2, p. 70) with ISO/IEC/IEEE 29148's per-requirement verification-method attributes is
the antecedent rule 4 specialises; the norms name a method class, not a test id. Read "specialises documented
requirements-engineering practice; no agile-AC-specific norm found". [documented, fetched 2026-09-10] Decision
unchanged.

**E-0085-2 (2026-09-11, PIA-023; audit grounding EXT-DECISION-004/-008, EXT-NOURL-011).** The evidence line "Flat `NNNN-slug.md` with a
generated index is community consensus at 100+ records … adr-tools AND Log4brains regenerate a README index" — **What is true:**
`adr-tools -s` stamps bidirectional Superceded-by/Supercedes links and strips Accepted (confirmed); **Log4brains rejects NNNN numbering**
(it uses `YYYYMMDD-slug.md`), builds a static site with a timeline and infers metadata from raw text plus git log — cite adr-tools alone
for the NNNN convention. MADR's minimal cut keeps Considered Options and has no frontmatter; "updated over time" is MADR's allowance for
Consequences/Confirmation, not a description of its status field. [documented, github.com/adr/madr; log4brains docs, fetched 2026-09-10]
Decision unchanged (the format is this repository's; the citation was the overclaim).

## ADR-0086 — No durable-execution framework; the memo table is built here

**E-0086-1 (2026-09-11, PIA-074).** Lines 19, 36-37: "`temporalio-sdk` on `temporalio-sdk-core` v0.5.0, updated June
2026" — **What is true on the record's own read date (2026-09-08):** `temporalio-sdk` **1.0.0** (2026-09-04, stable) on
`sdk-core` **0.9.0**. Lines 20, 39: "graph-flow v0.6 … makes postgres an optional Cargo feature" — **What is true:**
graph-flow **0.8.0** (2026-09-07) has `postgres` optional **and on by default**, so "no required database" holds only
under `default-features = false`; 0.6.0 hard-required `sqlx`. Line 32: "Rust remains second-class across every
first-party durable-execution SDK checked" — **false**: `dbos` 0.5.0 is a first-party Rust SDK; Temporal's Rust SDK is
1.0. `iopsystems/durable` has tags. The time-skipping test-server item is answered: `ephemeral_server::TestServerConfig`
exists (`testing` feature, 1.0.0); `WorkflowEnvironment::start_local` since v0.7.0. Line 55's instruction to propagate
the versions is withdrawn. [documented, crates.io / docs.rs / GitHub, fetched 2026-09-10] Decision unchanged — the
four crates are excluded on ADR-0003's grounds, which do not depend on version currency.

## ADR-0087 — A mid-graph gate resumes into the same step

*(Partially superseded by ADR-0106, which carries the cost correction.)* **E-0087-1 (2026-09-11, PIA-080).** Line 88:
F38's "0.45× / 0.55× / 0.78×" divide `output_tokens` by F28's `cache_creation_input_tokens`; on the like counter the
resumed dispatches created 84,960 / 49,468 / 46,961 cache tokens. [measured] The losslessness evidence (F38 §1) is
untouched.

## ADR-0088 — (superseded in full by ADR-0101)

*(No errata; the record moved to `superseded` and its corrections are ADR-0101's body.)*

## ADR-0094 — Phase 1 ships a Rust toolchain adapter; self-hosting means both lanes

**E-0094-1 (2026-09-12, DECISIONS-2026-09-12 E12).** Evidence and confidence, line 123: "It does not decide the
Codex adapter's phase or the CI decision ADR-0010 defers." — **What is true:** the record's own **decision 2** does
decide the phase, in terms: *"The Harness port's second real adapter (Codex) is **Phase 2** and stays behind the CI
decision ADR-0010 defers"* (:67-69). The Evidence sentence is right about the **CI decision**, which decision 2
expressly leaves with ADR-0010, and wrong about the **phase**, which decision 2 assigns. Read as written it would
make an accepted Decision self-withdrawing. The sentence reads correctly as: *"It does not license any claim about
the Rust adapter's behaviour — none exists … It does not decide the CI decision ADR-0010 defers."* [documented,
references/adr/0094 :67-69 against :123, read 2026-09-12] **What this does not change:** decision 2 stands verbatim,
and with it Codex = Phase 2; `rk-adapter-codex` is out of Phase 1 under every reading; ADR-0010's CI decision stays
deferred; AC-496 stays PROVISIONAL on S-CODEX, which is unrun. Raised by the 2026-09-12 runbook alignment review,
which found design :1905 carrying "Phase 4 is the earliest it ships" — a row dated 2026-09-08, three days older than
this record and corrected the same day.

## ADR-0089 — Facet provenance, declared surfaces and abstention

*(Partially superseded by ADR-0093.)* **E-0089-1 (2026-09-11, PIA-093).** Lines 75 and 164: "plain Semgrep's measured
numbers on adjacent tasks run from 0.57% to 23.8% recall" — **What is true:** arXiv 2604.13764 is RealVuln (Pellew &
Raza, 2026-04-15; 26 Python repos; Semgrep F1 18.9, precision 0.205, **recall 0.175**); BigVul/Devign and the 2.43%/0.57%
figures appear nowhere in it. The corrected range is **17.5%–23.8%** (RealVuln + arXiv 2603.22853); 0.57% and 2.43% are
withdrawn as unsourced. The 59.9% IDOR recall is Semgrep's 2026-08-27 Multimodal post (precision 57.5%). And the
"nothing found" on data-migration base rates: Qiu, Li & Su, ESEC/FSE 2013 — schema-changing revisions ≈ 1.7% of 160K
(≈ 0.9% after manual validation), so 0 positives in 20 items is the expected count. [documented, fetched 2026-09-10]
Decision unchanged (the option was rejected on cost and on relocating the vocabulary problem, not on the floor).

**E-0089-2 (2026-09-11, PIA-073, PIA-077).** Limits 1: "roughly 60 positives are needed for a lower bound above 0.95"
— **What is true:** **73** all-correct positives at the registered two-sided z = 1.96 (92 with continuity correction;
72 under Clopper–Pearson). The F39 table's "140 truth cells" were **136 scored** on 2026-09-09 (four cells skipped on a panel-file key
mismatch, `resolve-open-questions` vs `resolve-open-question`); with the key fixed and `score-arm-d-panel.py` failing closed on a truth
cell with no engine record, the 2026-09-11 re-run scores **all 140** and `auth` reads 0/12 [0.00, 0.24] (fp 11 → 12); every verdict is
unchanged. [measured, re-run 2026-09-11] Decision unchanged.

## ADR-0100 — Dependency policy: the YAML reader, the 0.x pin list and cargo-deny

**E-0100-1 (2026-09-13, Step 5.2 / Step 6).** Lines 68-69 (as at HEAD `96ba9c2`; this record postdates the `009a4ec`
pin): "**4. `cargo audit --deny unmaintained`** is the form the audit job runs (equivalently `cargo deny check
advisories` with `unmaintained = "deny"`), so an archived transitive dependency is a red build, not a silence."
— **What is true:** the parenthetical **does not parse** at cargo-deny **0.20.2**, the version the preamble's tool
preflight measured on this box. That line changed `[advisories].unmaintained` from a lint **level** to a **scope**,
and the tool refuses any other value:

```text
error[unexpected-value]: expected '["all", "workspace", "transitive", "none"]'
2 | unmaintained = "deny"
  |                 ━━━━ unexpected value
```

`unmaintained = "all"` is the faithful translation — every unmaintained advisory is checked, transitive ones
included, and advisories are errors by default in this version; the same command then reports `advisories ok`.
[measured 2026-09-13, cargo-deny 0.20.2, both values run in an isolated probe crate so the result is a property of
the tool and not of this workspace's graph]

**What this does not change:** the Decision's normative sentence — that **`cargo audit --deny unmaintained` is the
form the audit job runs** — stands as written, and so does the clause it serves, *"an archived transitive dependency
is a red build, not a silence"*, which `unmaintained = "all"` implements exactly. The forbids bullet *"Forbids
describing the `cargo audit` job as covering unmaintained crates unless it runs with `--deny unmaintained`"* is
untouched, and the committed `deny.toml` has carried `"all"` with its reasoning since Step 5. Only the
parenthetical's **claim of equivalence at this cargo-deny version** is withdrawn. Decision unchanged.

*(Recorded here rather than as a supersession on the ADR-0086 precedent: E-0086-1 withdraws line 55's instruction,
also a `## Decision` line, while leaving the decision standing. ADR-0092's test is whether every sentence of
`## Decision` still stands, not whether a correction falls inside that section — and here the normative sentence,
its consequence and its forbids bullet all stand.)*

## AC-520 — The derived-facet sibling gate (`redkiln lint --facet-provenance`)

*(A criterion of the AC register, not an ADR. Recorded here by extension of ADR-0092 decision 1, which writes the
convention for an accepted **record**; clause 2.2 makes the register the in-tree owner of every criterion's
`*Verified by:*` instrument, and the register is amendable in place, so this section is the lighter instrument and not
the only one available. It is the right one because what moves is a **count that restates an enumeration** — not the
enumeration, not the GIVEN/WHEN/THEN, not the state label. **Lines are as at the working tree this erratum is
committed with** (2026-09-13), not the `009a4ec` pin this file's header names and not HEAD `60f17c6`: a first draft
pinned them at `60f17c6` and the AC-006/AC-188 golden-log amendment — written into the same register in the same
session — inserted **30** lines above AC-520, so every one of them was stale before it was committed [measured
2026-09-13]. **The clause each number names is quoted beside it and is the citation that survives; the number is a
convenience and may go stale again.**)*

**E-0520-1 (2026-09-13, the Step 7b audit, finding RECORD-OWED-A).** Line 1356, the opening of `*Verified by:*`:
"the `--facet-provenance` lint rule; **six** must-fail fixture `facets.toml` files, one clause at a time" — **What is
true:** the enumeration that the same sentence then carries, running :1356-:1366 (*"a `derived` facet with no
`deriver` field"* through *"a `require` row keyed on a `declared` facet"*), is **eight clause groups over ten
must-fail files**, and the positive companion stands beside them at :1366-:1369 (*"and a **positive companion** that
must load"*). Counted off the register:

| # | Clause group, as the register writes it | Files |
| --- | --- | --- |
| 1 | a `derived` facet with no `deriver` field | 1 |
| 2 | a named `deriver` with no scored record | 1 |
| 3 | a record carrying 0 positives whose facet keys a `require` row | 1 |
| 4 | a `deriver` declared lexical (`deriver_class = "lexical"`) whose facet keys a `require` row | 1 |
| 5 | a `derived` facet with no `deriver_class`, **and** one whose `deriver_class` is neither of the two tokens | **2** |
| 6 | a `derived` facet with no `deriver_record`, **and** one whose `deriver_record` names a path absent from the pinned corpus | **2** |
| 7 | a `kernel`-class facet carrying a `deriver`, a `deriver_class`, a `deriver_record` or claiming a record (ADR-0093 forbids) | 1 |
| 8 | a `require` row keyed on a `declared` facet | 1 |

— **eight groups, ten files**; plus the **positive companion** that must load (:1328-:1330: a `derived` facet with a
named `deriver`, a scored record and positives; a `kernel` facet with neither; a `declared` facet keying no row),
"without which the rule is satisfied by a lint that rejects everything". [measured, counted off
`references/2026-09-08-acceptance-criteria.md` :1356-:1369, 2026-09-13]

**Why "six" was once exact.** Groups **5** and **6** are the two the **2026-09-13 amendment** at :1372-:1388
inserted — :1375 "It is now `deriver_class`, declared **by name rather than by omission**" and :1381 "It is now
`deriver_record`, corpus-relative, resolved through `CORPUS_PIN` and never through a working tree (AC-464)". Before
that amendment the list was groups 1-4, 7 and 8: **six groups, six files, one clause each**, and "six … one clause at
a time" was exact for what stood. The amendment added two groups of two files apiece and did not revisit the count,
and it states at :1388 that "**Nothing is withdrawn by this amendment**" — so the six pre-amendment clauses stand and
four files join them. Nothing is withdrawn here either: this corrects arithmetic that went stale under an amendment,
not a requirement.

**The enumeration is the authority; the number is not.** The fixture set is the list at :1356-:1366 plus the
companion, taken **by reference**. Ten is the count of that list as the register writes it today, and it is reported
here so a reader meeting "six" knows the sentence disagrees with itself — it is not a new bar, and a downstream
document that restates **any** number instead of the list will go stale again the next time a clause is added. One
group makes the point: group 7 is written with "or" across four forbidden fields, so a fixture author who chose one
file per forbidden field would count more than ten and still satisfy the clause exactly. The list decides what must
exist; the arithmetic decides nothing.

**Checkpoint 4a's six is a DIFFERENT six, and it is untouched.** Line 1369-1370 — "Runbook Checkpoint 4a asserts
**six** derived facets and six records, reconciled by name — not eleven" — counts **facets in `.redkiln/facets.toml`
and their scored records**, not fixture files. Verified against the runbook: `runbook/phase-0/s4a-apply-adr-0089.md`
:73-74 asserts `grep -c '^source = "derived"'` = 6 and `grep -c '^deriver = '` = 6 on that file, and Checkpoint 4a's
own line :205 asserts `d==6 && v==6` over the same two patterns, with §4a.1 :52 requiring the six be "reconciled by
name, never by the number" — the six being `toolchain`, `change`, `ui`, `flag_exists`, `data_migration`,
`recent_deploy`, the facets ADR-0093 decision 2 leaves `source = "derived"`. [measured, s4a :52, :73-74, :205, read
2026-09-13] The two sixes coincided by accident until 2026-09-13 and the coincidence is now broken. **This erratum
moves the must-fail fixture count and nothing else**; the six `derived` facets, their six records and every Checkpoint
4a assertion over them stand as written.

**What this does not change:** the GIVEN, WHEN and THEN (:1338-:1355) in full, including the three-clause THEN the
amendment wrote (`deriver`, `deriver_class`, `deriver_record`) and ":1352 Absence is never inferred"; the enumeration
itself, every clause of which stands verbatim; the positive companion and the reason it exists; the criterion's state
— **not provisional**, and passing on the registry at the pin (:1370-:1371); the SM-157 back-link; the amendment's
three findings, including that the machine-readable scored record is `[WILL EXIST]` and **Step 7b's to produce**
(:1382-:1385).

**The back-pointer this erratum names as owed HAS been landed.** AC-520's `*Verified by:*` now carries a one-line
`ERRATA E-0520-1` citation immediately after "one clause at a time", in the same commit as this section, on the form
the register already uses elsewhere. A builder reading only the register therefore meets the correction where the
stale count is, rather than only through `ERRATA.md`'s header — which is the discoverability gap this erratum
otherwise conceded, since AC-520 is a criterion and can carry no `(errata)` marker in the ADR README's index.

*(Recorded as an erratum rather than as a further register amendment on the E-0100-1 / E-0086-1 precedent:
ADR-0092's test is whether every normative sentence still stands, not whether the correction falls inside a section
that carries normative weight — E-0086-1 withdraws a `## Decision` line's instruction while leaving the decision
standing, and E-0100-1 cites it for exactly that. Here the `*Verified by:*` line's normative content is the
enumeration and the positive companion; both stand verbatim, and only the arithmetic restating them moves.)*

*(Discoverability, stated rather than smoothed. ADR-0092 decision 1 hangs an erratum's findability on the corrected
record's index row in `README.md` carrying `(errata)`. **AC-520 has no such row** — that README indexes the ADR
records `0001`-`0118` and nothing else — so no row was marked, and none could be. The back-pointer this section needs
is a citation of **E-0520-1** at register :1322, in the form the register already uses for errata it depends on
(`ERRATA E-0089-2` at :1296, `ERRATA E-0056-1` at :790). That edit is **owed and not made here**: this erratum was
written under a task that names this file alone.)*

## ADR-0132 — `ImplReport` is the sixth toolchain-keyed kind

**E-0132-1 (2026-09-21, `s7f`'s own adversarial pass).** `## Context and problem`: *"because **every activity that provides `Changeset` also provides `ImplReport`**, and `Changeset` is keyed while `ImplReport` is not"*, restated in `## Decision drivers` as *"`ImplReport` and `Changeset` are produced by the same activities"* and in Context as *"The two kinds are the same activities' two outputs"*. — **What is true:** the universal holds in the OTHER direction only. Over the shipped catalog at this record's own pin there are **10** `Changeset` providers and **9** `ImplReport` providers; the difference is `docs-update`, which provides `["Docs", "Changeset"]` and does not name `ImplReport` anywhere in its file. So: **every activity that provides `ImplReport` also provides `Changeset`**, and the set difference in that direction is empty. [measured 2026-09-21, `comm -23` over `grep -l` on `.redkiln/catalog/*.toml`, cross-checked by an independent TOML parser] **What this does not change:** the conclusion the sentence was offered as warrant for, which is measured separately and independently — the two `ImplReport` equal-rank sets and the two `Changeset` ones have **identical membership** at both ranks (`dependency-update`/`hotfix`/`implement-ue`/`test-after` at 10, `bdd-outside-in`/`walking-skeleton` at 30), because `docs-update` sits at `method_rank = 5` and is therefore in neither tied set. The Decision, both supersession sites, the pack change and every forbids bullet stand as written. **Why it matters even though the conclusion survives:** a reader who trusted the sentence as written would conclude that `docs-update` must emit an `ImplReport`, which the catalog does not ask of it.

## ADR-0133 — `rate_limit_reading` gains a producer and a typed payload

**E-0133-1 (2026-09-21, `s7f`'s own adversarial pass).** `## Context and problem`, bullet 1: *"Its **only three references in the workspace** are kernel exhaustive-match no-op arms — `workflow/breaker.rs`, `workflow/fsm.rs`, `workflow/memo.rs` — **each classifying it as tier 2**"*, restated in `## Decision` clause 1 as *"The three kernel match arms stand as written — tier 2, no-op."* — **What is true:** the three kernel arms classify nothing. Each is a bare `=> {}` inside a FOLD — `BreakerState::from_events`, `apply`, `tally_activations` — and no `Tier` token appears in any of the three match statements. The tier is assigned in exactly one place on the tree, `rk_types::EventKind::tier()`, whose `| Self::RateLimitReading` arm falls to `Tier::Two`. That is a **fourth** reference, in `rk-types` rather than `rk-kernel`, which the record's own grep pattern (`EventKind::RateLimitReading`) cannot see because the arm is spelled `Self::RateLimitReading`. [measured 2026-09-21] **What this does not change:** the absence the record rests on — that **nothing constructs or appends an `Event` of this kind** — which the fourth reference does not affect, since `tier()` is a classifier and not a producer. ADR-0126 decision 5, the template this record cites, draws the same distinction correctly (*"`wire_name` and `tier` gain arms; … the kernel's exhaustive matches route it to their no-state-change arms"*); ADR-0133 collapsed the two. The Decision, the payload, the producer and clause 5's RT-19 settlement all stand as written; the phrase *"only three references"* is to read *"only three in `rk-kernel`, and a fourth in `rk-types` that classifies rather than produces"*.

## ADR-0138 — the step list reaches the runtime beside the spec

**E-0138-1 (2026-09-22, `s7g`'s build, at clause 1's first execution).** `## Decision` clause 4: *"If the transcription fans out more widely than **the three sites that thread `RecordedSpec` today**, the fallback is option 4 — `&[Step]` as a parameter to `run_activity`"*, and `## Considered options` 4, restating the same figure as *"it widens a signature that **three call sites** already thread"*. — **What is true:** neither figure is three. Measured over `crates/`: `RecordedSpec::supplied` has **17** call sites (1 production, in `rk-cli`'s `bind_nodes`, and 16 across `rk-cli`'s and `rk-kernel`'s integration suites), and `run_activity` has **19** (2 production — `runtime.rs`'s `step()` `Dispatch` arm and `replay.rs`'s `dispatch_through` — and 17 in `rk-kernel`'s `runtime.rs` and `cross_process.rs` suites). The "three" counted the *production* thread only and silently excluded every test call site, which is precisely the population a signature change is paid for in. [measured 2026-09-22, `grep -rn` over `crates/ --include=*.rs`, excluding the `fn` declarations] **What this does not change:** clause 1, which is what was executed. The step list **is** carried on `RecordedSpec`, the fallback clause 4 names was **not** taken, and `ActivitySpec` and residual 7 are untouched. **What the measurement did change, and where that is recorded:** clause 4's condition was met on its own terms, and the shape taken is a third one neither the clause nor its options list names — `steps: Option<Vec<Step>>`, where `RecordedSpec::supplied` keeps its signature and yields `None` ("this caller transcribed none") and a new `RecordedSpec::with_steps` is what the composition root calls. `None` runs the pre-`s7g` agent dispatch and **never an empty walk**, so the forbids bullet — *"a missing transcription is an error naming the activation, never an empty walk"* — stands, and an explicitly EMPTY transcribed list is refused by name. That shape was put to the operator with the measured fan-out in front of them and taken on 2026-09-22; clause 4's own instruction was to record the switch **with the measurement that caused it**, which this erratum is. **Why it matters even though clause 1 survives:** a reader deciding between clause 1 and clause 4's fallback would have weighed 3 against 17 and could have reached the opposite conclusion on a figure that was never measured — in a record whose every other figure was.

~~*(Recorded as an erratum rather than a supersession on E-0132-1's route: ADR-0092 decision 1's test is whether every normative sentence still stands. Clause 4's normative content is its **instruction** — take the fallback, and record the switch with the measurement — and that instruction stands and was followed.)*~~ **That parenthetical is WITHDRAWN, 2026-09-22, by the same adversarial pass: it is self-refuting.** Clause 4's instruction is *take the fallback*, and this erratum's own body records that the fallback was **not** taken (*"the shape taken is a third one neither the clause nor its options list names"*). An if-then `## Decision` sentence whose antecedent fired and whose consequent was declined does not "still stand", so ADR-0092 rule 1 routes it by **supersession**. **That supersession is OWED and is not written here.** What survives as a legitimate erratum is the arithmetic: *"the three sites that thread `RecordedSpec` today"* was wrong (17, and 19 for `run_activity`), and the same figure is restated in a non-normative options bullet. **The corpus took the opposite route to the identical shape 53 minutes earlier**, and that is the inconsistency this withdrawal names: ADR-0139 supersedes ADR-0137 clause 8 — one clause, same author, same day, prohibition retained — citing ADR-0092 rule 1 in terms. Two opposite routes, one rule, one morning; the cheaper one was taken here on a ground this entry disproves two sentences earlier.

## ADR-0134 — the port call-site lint is scoped to Phase-1 real adapters

**E-0134-1 (2026-09-22, `s7g`'s own adversarial pass).** The record's central warrant, at six sites — the `title`, `## Context and problem` (twice), `## Decision` clause 2, `## Evidence and confidence`, and the README index row — reads *"design:265 — `Tracker`, Phase-1 real adapter column: **\"none in Phase 1 (GitHub is Phase 3)\"**"* and *"design:266 — `DocsSite`"*. — **What is true:** those are the wrong rows, and they are the rows that refute the claim. §3.3's port table runs `EventStore` **265**, `Harness` **266**, **`Tracker` 267, `DocsSite` 268**, `Toolchain` **269**; the two lines cited name the two ports that *do* have a Phase-1 real adapter. [measured 2026-09-22, `grep -n "none in Phase 1" references/2026-09-08-architecture-design.md` → 267, 268] The corpus already carried the correct citation in three places the record did not consult — `s7a`:195 (*"design:**267** is their only occurrence in this corpus"*), `s7a`:199 (*"fake-only until Phase 3 (design:**267-268**…)"*) and ADR-0131:18, which cites `design:265` for the EventStore row correctly. This was wrong at birth, not drift: the design file's last commit predates every one of these records. **What this does not change:** design:267-268 say verbatim what the record quotes, so the conclusion — `Tracker` and `DocsSite` have no Phase-1 real adapter, and the emptiness form was unsatisfiable inside Phase 1 — stands on rows 267/268, together with AC-499's phase clause and `s7a` §7a.1:47, both of which verify exactly. **Why it matters even though the conclusion survives:** clause 2 requires each entry's warrant to live **in the source, not in a commit message**, so this citation is load-bearing code in `conformance.rs`'s `expected_uncalled` table — a lint that runs on every commit was pointing a reader at the rows that contradict it. Those two warrant strings are corrected in place at `s7g`'s repair pass and re-stated **by quotation** rather than by line.

**Second, in the same entry.** Four quotations are attributed to *"`s7a` §7a:198-199"*; measured, they are at **200** (*"so no adapter and no kernel consumer of either type exists before Phase 3"*), **202** (`pub struct ExternalRef(pub(crate) ());`), **203** (*"deliberately: an opaque newtype declares the port shape…"*) and **204** (*"No field set is invented and no idempotency key is designed"*). Lines 198-199 carry the routing sentence — and line 199 is the one holding the correct `design:267-268` citation the first half of this erratum got wrong. The source warrants now cite `s7a` §7a.4 and quote the text.

## ADR-0136 — a step guard with an unresolved reference refuses

**E-0136-1 (2026-09-22, `s7g`'s own adversarial pass).** `## Context and problem` and the `title`: *"**121 step-level `when` clauses, of which 54 carry a `${…}` reference**"*, and the framing throughout that the 54 are what refuse. — **What is true:** the counts are right and the SCOPE they imply is badly wrong. **All 121 guards refuse, and all 94 `check` exprs with them.** `guard_holds` and `check_verdict` evaluate against `predicate::Context::new()`, which binds nothing, and `eval.rs:457` maps any unbound path to `EvalError::UnknownField`. Measured over the shipped pack: of the 67 ref-free guards, **67 read `rigor`** (plus `blast_radius` ×2 and `criticality` ×1, co-occurring), so every one of them errors before any reference is reached; 9 of the 54 ref-bearing guards die on `UnknownField("rigor")` rather than `UnresolvedRef`, because `rigor >= mandatory` is the left conjunct. [measured 2026-09-22, every `when`/`expr` in `.redkiln/catalog/*.toml` driven through the real `parse`+`evaluate`: 215 lines, **zero** `Ok`] **What this does not change:** clauses 1-5 say what to do with a guard carrying an unresolvable reference, and what they say is still right. **What it does change, and it is not small:** the record presents the gap as bounded by reference resolution, and the gap is the whole guard language — nothing on this tree ever binds a facet into the executor's `Context`, so the reference resolver clause 4 records as OWED would not by itself make one guard evaluable. **The executor's resulting behaviour is a defect and is recorded as such** in `runbook/PROGRESS.md` §1's Step 7g row rather than repaired here: `execute_step` maps the error to `StepVerdict::Failed` and the walk hard-stops, so every guarded step and every `check` step ends its activation.

**Second, in the same entry.** The Evidence line *"[measured 2026-09-22] … Its call sites are `quality_block.rs`, `rigor.rs`, `select/step2.rs`, `select/step4.rs`, `select/step7.rs` and `rk-cli`'s `explain.rs` — **all selection- or lint-time, none in `runtime/`**"* was falsified the same day by this record's own implementation: `crates/rk-kernel/src/runtime/step.rs` now holds **three** `predicate::evaluate` call sites. The measurement was true when taken and the dated tag reads as current, which is the hazard.

## ADR-0137 — the real toolchain adapter is built at the composition root

**E-0137-1 (2026-09-22, `s7g`'s own adversarial pass).** `## Decision` clause 5: *"this single bound backstops every child the adapter spawns, **while a step's own `timeout_secs` remains the kernel's**"*, and its reasoning *"the backstop may not be smaller than the largest step it could serve"*. — **What is true:** no step timeout is applied by anything, because **`catalog::Step` parses none**. `catalog.rs`'s loader doc says in terms *"It types only what the loader sees … The loader reads no `timeout_secs`"*, and the struct carries no such field. So the 3600 s value is not a backstop behind a per-step bound; it is the **only** bound any toolchain child ever gets. The floor is also measured over a partial corpus: of **298** `timeout_secs` declarations in the shipped catalog, **204 are literals** (largest 3600) and **94 are unresolved `${toolchain.*_timeout}` strings** whose values nothing on this tree resolves — so 94 of the steps the bound "could serve" declare an unknown. **What this does not change:** clause 5's *number* is defensible on its own terms (3600 is the largest declared literal, and the record says "literal"), and clauses 1-4 and 6-7 are untouched. 🛑 **What this erratum does NOT do, deliberately:** the sentence *"a step's own `timeout_secs` remains the kernel's"* is a `## Decision` sentence and it is false, which ADR-0092 rule 1 routes through **supersession, not errata**. That supersession is **OWED and is not written here** — the same boundary E-0138-1 is corrected for crossing, and it is named rather than quietly taken.

**Second, in the same entry.** Every `run.rs` line citation in the record — `:285-292` and `:291` (the `FixtureToolchain::default()` construction) in Context and clause 9, `:222-292` and `:223` in `source:` — was moved by the record's **own implementing commit**, 53 minutes later. At HEAD `run.rs:291` is a comment about `Binder::new`, and no `FixtureToolchain` remains in that file. Cite by quotation.

## ADR-0139 — an undetected toolchain refuses at the port, not at startup

**E-0139-1 (2026-09-22, `s7g`'s own adversarial pass).** Six sites say the superseded record is *"one day"* old: `## Decision drivers` (*"even though the record being moved is one day old"*), the **Confidence** block (*"this record is one day younger than the one it moves"*), the README index row, and D21-14's heading and body. — **What is true:** **54 minutes.** ADR-0137 was committed `proposed` at `085d1aa`, 2026-09-22 **10:34**, and superseded in part at `fbf3c4c`/`305398c`, **11:27-11:28** the same morning. Both records carry `date: 2026-09-22`, and D21-14's own body says so two lines after the claim (*"written on 2026-09-22 and superseded in part on 2026-09-22"*), so the entry contradicts itself in place. **What this does not change:** the record's substance, its route, or its confidence — ADR-0092 rule 1 turns on whether a `## Decision` sentence moved, not on how old the record was, and the supersession is correct either way. **Why it matters:** the false half sits in the **Confidence** block, whose entire job is to state the record's limit honestly. It also *understates* the finding: a decision corrected inside one hour by its own first execution is a sharper instance of "taken ahead of its first measurement" than a one-day one, and the sharper reading is the true one.

## ADR-0152 — the lane loop lives above `resume` and the advance arm breaks

**E-0152-1 (2026-09-23, `s7h`'s own adversarial pass).** Three present-tense, unqualified citations in
`## Context and problem` and `## Evidence and confidence`: *"`fsm::fold`'s second parameter at
`replay.rs:1767`"*, *"`next_action`'s arm 4 (`fsm.rs:1700-1707`) tests `ctx.successor`"*, and
*"`fsm_context` (`replay.rs:1546-1558`)"*. — **What is true:** all three were exact at `fc45eeb` and
none is exact at this stage's close. `replay.rs:1767` is now a comment about `prior_len` (the `fold`
call moved); `fsm.rs:1700-1707` is now the **halt** check, because **ADR-0154 — this record's own
batch-mate — reordered the arms**, so the citation is contradicted by a decision taken the same day;
and `fsm_context` moved. The dated block's figures moved too: the sole `AdvanceStage` construction is
now at `fsm.rs:1736`, `base_inputs` moved off `run.rs:1047`, `run.rs` **does** chain `.advances_to(..)`,
and `grep -rn "advances_to" crates/` returns **10** hits rather than four. [measured]
**What this does not change:** every `## Decision` clause and every forbid. The mechanism the citations
point at is unaltered — `inputs.stage` is still fixed before the loop, `fold` still reads it every
iteration, and `run_id` is still stage-prefixed. **The lesson, recorded because it recurs:** a record
that cites a line into a file its own stage is still editing has a self-falsifying citation. The dated
figures at least say when they were true; the three above did not.

## ADR-0153 — the lane order is a declared array

**E-0153-1 (2026-09-23, `s7h`'s own adversarial pass).** Under *"The no-breakage claim was checked
rather than assumed"*, present tense: *"`pack.rs` reads `policy` twice (`:212`, `:251`)"*, and in the
same section *"`pack.rs:250-263` … the key-by-key insert"*. — **What is true:** at this stage's close
`pack.rs` reads `policy` **three** times — and **this record's own key added the third**. The line
numbers moved with it: `:212`/`:251` are now unrelated lines and `250-263` is the `[mutation]` reader.
Both were exact at `fc45eeb`. The related claim *"`grep -n 'stage_order' .redkiln/packs/*.toml` returns
nothing — the key does not exist yet"* is dated to `fc45eeb` and so remains honest; both packs now
declare it. [measured]
**What this does not change:** the decision, the forbids, or the no-breakage conclusion — there is
still no unknown-key rejection in `select/pack.rs` or `pack_validation.rs`, and `doctor`'s
`RK-D-DEAD-DEF` still reads `stages.keys()` alone. A record whose evidence counts occurrences of a
thing it is itself adding must say which side of its own change it counted.

## ADR-0154 — a recorded halt outranks a supplied successor

**E-0154-1 (2026-09-23, `s7h`'s own adversarial pass).** *"`grep -rn \"advances_to\"
crates/rk-kernel/tests/` returns two sites"*, and *"no third context exists"*. — **What is true:** there
are now **three** call sites — `tests/it/replay.rs`, `tests/it/resume.rs` and
`tests/it/lane_order.rs` — and the fsm order sweep now **does** carry a context supplying both a halt
and a successor. [measured] This is stale **by design**: the record's own clause 4 says the new test is
part of the decision, so the absence it measured is what the record then removed. It is logged so a
reader who reproduces the grep and gets a different number knows why, rather than doubting the finding.
**What this does not change:** nothing. The decision, the forbids and the mutant attribution all stand.

## ADR-0155 — one `Story:` trailer per item per lane

**E-0155-1 (2026-09-23, `s7h`'s own adversarial pass).** The title and clause 1 say *"one `Story:`
trailer per item **per lane**"*. — **What is true:** the producer's idempotence read is
`rk_git::reconcile::read_commits`, which is `git log --all` over every ref, matched on
`(project, slug)` with **no lane, stage, run or time component**. So it is one trailer per item **for
the lifetime of the repository**: a second lane over the same item mints nothing. The doc comment inside
`story_trailer` is the accurate one — *"ONCE per item"*. [measured, the function's own read]
**What this does not change:** the decision is unaffected and arguably stronger than its title:
`reconcile` folds a repeated `(project, slug)` as `Drift::DuplicateStoryTrailer` **regardless of which
lane wrote it**, so once-per-item-for-ever is the only cardinality that keeps that instrument green.
The title's *"per lane"* is a narrower claim than the code makes and than the reader needs.

## ADR-0156 — the two axes split and the shipped review stage is unreachable

**E-0156-1 (2026-09-23, Checkpoint 7h's guard (10), read at `f25440e`).** `## Consequences` opens
*"Both axes get their first producer, the ARMED lint `conformance_every_trailer_key_has_a_producer`
is discharged with a behavioural assertion behind it rather than by a substring, and
`Exit::FindingsBlock` gains the producer its own declaration has been promising since 7b-ii. `s7i`'s
first blocker … is cleared for the fixture corpus"*, and a `**What this forbids**` bullet reads
*"Giving **the fixture pack's** review activity a `check` or `gate` step …"*. — **What is true:**
both sentences describe the **pre-amendment** plan. `## Decision` clauses 2 and 3 were *"OVERTAKEN BY
MEASUREMENT AND … AMENDED HERE, while this record was still `proposed` and had bound nothing"*, and
clause 3 as it stands reads *"The `review`/`verify` call site and `Exit::FindingsBlock` are **NOT
delivered at `s7h`** … **No fixture pack is authored**"*. So on the record's own amended terms
neither axis got a producer, `Exit::FindingsBlock` gained none, the ARMED lint kept its `#[ignore]`
(clause 2), and there is no fixture pack for the forbids bullet to bind. The record's own **title**
already carries the amendment — *"clauses 2 and 3 prescribed a fixture pack and were amended"* — so
`## Consequences` is the only half that was not carried forward. Measured at `f25440e` on
2026-09-23: `grep -rn 'Exit::FindingsBlock' crates/rk-cli/src/ --include=*.rs | grep -v 'exit.rs'`
returns **0** — which is precisely the condition Checkpoint 7h's guard (10) fails on, citing this
record; and `crates/rk-cli/tests/fixtures/pack-min/` declares `review = "mandatory"` under
`[policy] stages` while carrying **no `[stage.review]` block** and exactly two catalog activities,
`emit-report.toml` and `second-activity.toml`, neither of them a review.
**What this does not change:** every sentence of `## Decision` stands, and every
`**What this forbids**` bullet stands **as written** — the fixture-pack bullet is vacuous rather
than wrong, and it binds the moment a fixture pack exists, which is how a later stage should read
it. The six blockers, the `0 of 102` claim and `0101` clause 5 are untouched.
**Why the wording mattered enough to correct:** guard (10) names this record as the reason it
fails, so a reader who follows that citation lands in `## Consequences` and is told the producer was
delivered — the two halves of one accepted record disagreeing about whether the thing the checkpoint
blocks on was ever built. Checkpoint 7h is the instrument that surfaced it, by failing exactly where
the record says it should.

**E-0156-2 (2026-09-23, the same change's adversarial pass, correcting E-0156-1).** E-0156-1 says
*"so `## Consequences` is **the only half that was not carried forward**."* — **What is true:** the
pre-amendment plan survives in **four** places, not one. Besides `## Consequences`, ADR-0156's
`## Decision drivers` still read *"a call site can be exercised without 61 schema files"* and *"A
fixture pack is the tree's own established instrument for a run that cannot be driven off shipped
content"*; its `## Considered options` **option 3, the chosen one**, reads *"**3. Split the corpora
(chosen).** … plus the review call site plus `Exit::FindingsBlock` over a fixture pack built for
it. Both axes get a real producer and a real assertion"*; and its `## Evidence and confidence`
reads *"A fixture pack proves the wiring, not the catalog"*. [measured 2026-09-23 at `5e67ac0`,
`git show HEAD:references/adr/0156-*.md`]
**And the batch contradicted itself on it**, which is the shape this stage has now shipped twice:
E-0156-1 says "only `## Consequences`" while **ADR-0162, written in the same commit**, says
*"`0156`'s `## Consequences`, **its decision drivers** and two of its forbids bullets all still
assume the fixture pack"*. The ADR's count is nearer the truth and is itself wrong on the bullets
(E-0162-1 clause 3).
**A second correction to E-0156-1, on the same reading:** it says *"and a `**What this forbids**`
bullet reads…"*, singular, which is **right** — ADR-0156 has **seven** forbids bullets and exactly
**one** mentions a fixture pack. ADR-0162's "two" is the error, not this.
**What this does not change:** E-0156-1's core finding stands and is the reason ADR-0162 exists —
`## Decision` clause 3 and `## Consequences` do contradict each other, textually and checkably.
Only the *scope* of the un-carried-forward text was understated.
**Format, recorded rather than silently fixed:** E-0156-1 carries no line numbers and no bracketed
evidence tag, where this file's header asks for *"the frozen sentence quoted verbatim with its line
at the pin it was read at … what is true, with the source, its fetch date and an evidence tag"*,
and where its three 2026-09-23 neighbours all carry one. It also adds **bold emphasis inside
quotations** (*"**No fixture pack is authored**"*, *"**the fixture pack's** review activity"*) that
is not in ADR-0156, unmarked as added.

## ADR-0158 — the tier-1 store layer has no writer on any axis

**E-0158-1 (2026-09-23, `s7h`'s own adversarial pass).** The `source` line and the Evidence section
both cite the live store as `t2_event` **30**. — **What is true:** the live
`.git/redkiln/store.sqlite3` now holds **4** `t2_event` rows. The store was deleted and recreated
several times during this stage's own lane experiments — it is untracked derived state under `.git/`
— so the figure was true of the store that existed when the record was written and is not a stable
quantity at all. [measured, read-only `sqlite3` open with `mode=ro&immutable=1`]
**What this does not change:** the record's entire substance. The three t1 counts are **0, 0, 0** on
every reading before and after, which is the claim; `t2_event`'s count was context, not evidence, and
citing a mutable derived number as though it were a measurement is the error being corrected.
**Also surfaced, and not a correction:** `rk_store::conn::Writer::transaction` is a generic escape
hatch through which arbitrary SQL *is* reachable, so "no method a caller could use" is true of the
`EventStore` port and not of `rk-store` as a whole. Its four non-test callers run the migration, read
`PRAGMA`s, and append `t2_event` rows; none touches a t1 table. The claim stands as stated about the
port. Decision unchanged.

## ADR-0159 — the project component is declared and refused when absent

**E-0159-1 (2026-09-23, `s7h`'s own adversarial pass).** In `## Evidence and confidence`, undated and
in the present tense: *"`.redkiln/config.toml` is thirteen lines and declares `[adapters]`, `[packs]`
and `[doctor]`; `grep -rn 'project' .redkiln/config.toml` returns nothing."* — **What is true:** the
file is now **20 lines** and declares `[project] name = "redkiln-rs"` plus this record's own four-line
comment, so the grep returns several hits. **This record's own decision is what falsified its own
evidence line**, and unlike its batch-mates it carried no measurement date to protect it. Two smaller
slips in the same section: the quoted *"the stage is read once, from the record, and is not defaulted"*
is attributed to `run.rs:550` and is at **`run.rs:690`** at the close (it was at `:552-553`, not `:550`,
even at `fc45eeb`); and the `bare_story_ids` insert is at `reconcile.rs:127-129`, not `126-128`.
[measured]
**What this does not change:** the decision, the forbids, or the silent-degradation mechanism —
`split_qualified`'s `(!p.is_empty() && !s.is_empty())` still makes an empty project a `BareStory` that
`reconcile` files under `bare_story_ids` and never treats as a transition, which is the whole warrant.
**Also extended, not corrected:** ADR-0161 clause 3 adds that a name containing `/` is refused too — a
constraint this record did not anticipate, found by the same pass.

---

## ADR-0160 — the `Slice-Verdict:` axis needs a slice identity nobody declares

**E-0160-1 (2026-09-23, `s7h`'s own adversarial pass).** The title and Evidence absence (1) say
`EventKind::SliceSealed` has *"exactly one construction in the whole workspace"*. — **What is true:**
that is **overstated as worded**. `SliceSealed` is passed to `Event::new` at exactly one site in
`crates/*/src` — `rk-cli/src/doctor/rebuild.rs:214` — which is the claim that matters and is the
definition the tree's own lint uses (`rk-kernel/src/module_boundary.rs`'s
`stage_exit_refused_single_site_rule`: *"a construction is the variant passed to `Event::new(` or
written as an `Event {` literal"*). But the variant is also constructed at
`rk-types/src/event_kind.rs:150`, inside `pub const ALL: [Self; 38]` — a literal in `src/`, not a test
and not a match arm — and at two test sites (`rk-cli/tests/it/doctor.rs:2325`,
`rk-kernel/tests/it/fsm.rs:476`); and `EventKind::from_wire("slice_sealed")` rehydrates it on **every**
`EventStore::query`, so reading the store constructs `SliceSealed` values routinely.
[measured, `rg 'SliceSealed|slice_sealed' crates/`]
**What this does not change:** every clause of `## Decision` and every `**What this forbids**` bullet.
The axis still has no **run-path producer**: the one `Event::new` site is the REBUILD path, `ALL` is an
enumeration rather than an emission, and rehydration on read is not production. The record's reasoning
turns on there being nothing that *writes* a seal during a run, which none of the above disturbs.
**Why the wording mattered enough to correct:** "exactly one construction in the whole workspace" is
the kind of absolute a reader can falsify in one `rg`, and a record whose headline is falsifiable by a
single command invites the substance to be dismissed with it.

---

*Later errata are appended under the record's heading with the next `E-NNNN-k` id. A heading with no entries is not
created.*

## ADR-0162 — the review call site is built over a fixture corpus and guard (10) becomes behavioural

**E-0162-1 (2026-09-23, this record's own adversarial pass, before the stage closed).** Six
corrections, all self-inflicted, and the first two are at the load-bearing joint.

**(1) The supersession's stated warrant is false to the sentence it supersedes.** ADR-0162's title,
its decision drivers, its README row and the `s7h` stage-file edit all say clause 3's sentence
*"was reasoned from ADR-0160's SLICE-IDENTITY absences"*. — **What is true:** ADR-0156 clause 3 says
the opposite, in terms: *"The `review`/`verify` call site and `Exit::FindingsBlock` are **NOT
delivered at `s7h`** either, **and the reason is this record's own blocker list rather than
ADR-0160's**: the review stage is unreachable, blocker (0) being the one a real run stops at."*
ADR-0162 never quotes, cites or acknowledges that half-sentence anywhere.
[measured 2026-09-23, `git show HEAD:references/adr/0156-*.md`]
**What this does not change:** the supersession itself, which stands on its merits — ADR-0160 clause
5 does say the call site is separable, the fixture pack was authored, and the producer a real run
reaches now exists. What is corrected is the *warrant*: clause 3's own reason was ADR-0156's six
blockers, and the honest argument is that a fixture corpus answers those blockers rather than that
clause 3 was reasoned from something it says it was not.

**(2) A load-bearing ellipsis.** ADR-0162 renders ADR-0160 clause 5 as *"§7h.3's call site is
separable and is also not delivered **…** a review call site could exist without sealing a slice"*.
— **What is true:** the elided words are *"**for ADR-0156's six blockers**. This record does not
merge the two:"*. The ellipsis removes the clause that attributes non-delivery to ADR-0156's
blockers — i.e. the very proposition correction (1) is about. E-0156-1 and the `s7h` edit repeat
the same truncation. A quotation that omits the half that cuts against the argument it is cited for
is the citation defect this programme errata'd five times one stage ago, in a new costume.

**(3) "two of its forbids bullets" is a false count, and the batch disagreed with itself.**
ADR-0162's title, its `## Considered options` and the commit message all say **two** of ADR-0156's
forbids bullets assume the fixture pack. — **What is true:** ADR-0156 has **seven** forbids bullets
and exactly **ONE** mentions a fixture pack (*"Giving the fixture pack's review activity a `check`
or `gate` step…"*). The other bullet ADR-0162 carries forward — *"Authoring `.redkiln/schemas/`,
populating `.redkiln/processes/`, or mapping `toolchain:verdict_record`…"* — presupposes no fixture
pack at all. E-0156-1, its own batch-mate, says *"a bullet"*, singular, and is right.
[measured 2026-09-23: 7 bullets, 1 matching `fixture pack`]

**(4) An undated `source:` claim, falsified by this record's own decision.** The frontmatter carries,
in the present tense and with **no date pin**: *"crates/rk-cli/tests/fixtures/pack-min/ (no
`[stage.review]` block; two catalog activities, neither a review)"*. — **What is true:** at the
commit that accepted this record, `pack-min/catalog/` holds **three** activities — `emit-report.toml`,
`review-min.toml`, `second-activity.toml` — and `pack.review.toml` carries `[stage.review]`. Both
were authored *by this record*. Every absence in the body is pinned to `f25440e`; this one was not.
This is ADR-0159's failure verbatim — *"an undated claim falsified by its own decision"* — and the
lesson that did survive is the one restated here: **cite by quotation and pin every measurement to
a tree.**

**(5) Decision 8 is not delivered.** It reads *"**8. The lane loop gains its first automated test.**
… The call site added here sits on that same path and is not delivered without one"*, and
`## Consequences` repeats *"The lane loop acquires its first automated test"*. — **What is true:**
`grep -rln "verbs::run::run(" crates/*/tests` **still returns nothing** [measured 2026-09-23 at
`5e67ac0`, rc=1]. The record's own named instrument is unmoved. What the change did add is one row
to `ac438_table_driven_exit_codes_over_fixtures_binary_arm`, a test that already existed, and five
`#[test]`s over the pass's PURE halves whose own module header says *"They do not run the binary,
and a green module here is **not** evidence that a real `redkiln run` ever returns `4`."*
**The debt ADR-0161 named therefore STANDS, and is carried forward rather than claimed closed** —
ADR-0161's forbids bullet *"Describing the lane loop as covered by tests"* is what decision 8 sails
close to. The call site IS covered by a binary run; the lane loop is not covered by anything.

**(6) A misattributed quotation.** Decision 10 says *"the path-set form `0156` clause 5(d) required
precisely because *"counts cannot tell a leaked registration from a replaced one"*"*. — **What is
true:** ADR-0156 clause 5(d) reads in full *"(d) the worktree guard compares **path sets, not
counts**, reading BEFORE any prune;"* and gives no reason. The quoted sentence is the **checkpoint's
own** comment in `runbook/phase-1/s7h-lane-completes.md`: *"Counts cannot tell a leaked registration
from a replaced one; paths can."* — reattributed here, with its capitalisation and its second clause
restored.

**AND TWO THINGS THE RECORD SHOULD HAVE SAID AND DID NOT, both about what the fixture proves.**
`## Evidence and confidence`'s *"What is NOT asserted"* list is incomplete:

- **The candidate and reference worktrees are checkouts of the SAME COMMIT.** `recreate_run_worktree`
  adds `rk-run/<item>` at `HEAD` and `reference_worktree` adds `--detach` at `HEAD`, so there is no
  change under review and the two trees are byte-identical. The `Flipped` that reaches
  `policy::classify` is manufactured by the stub toolchain's check on whether its working
  directory's last component ends `.review-ref` — not by any difference between the trees. The stub
  says so in its own header (*"🛑 THIS FIXTURE IS RIGGED, AND SAYING SO IS THE POINT"*); the record
  did not, and the record is what gets cited. **The wiring is real and the evidence is manufactured.**
- **A second run over the same repository would exit 4 without dispatching.** `Runtime::run_activity`
  short-circuits on `is_committed` (AC-013), so a re-run re-reads the memoised `activity_committed`
  row out of `prior` and re-confirms it. The fixture is sound because `pack_min_repo` builds a fresh
  `TempDir` per case — but *"the dispatch committed"* is a property of that freshness, not something
  `review_pass` enforces.

**What this does not change:** every sentence of `## Decision` except as clause (5) above records,
and every `**What this forbids**` bullet. The producer exists, a real `redkiln run` exits `4`, and
guard (10) is behavioural. **Decision 6 is not listed here because it was repaired in code rather
than corrected in prose**: it said the rows are appended *"and the journal is re-read"* and the
first cut did not re-read, so the re-read was implemented and the record left standing.
