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

---

*Later errata are appended under the record's heading with the next `E-NNNN-k` id. A heading with no entries is not
created.*
