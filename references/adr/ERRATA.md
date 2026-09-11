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
72 under Clopper–Pearson). The F39 table's "140 truth cells" is **136** after the panel-file key
(`resolve-open-questions` → `resolve-open-question`) is fixed and `score-arm-d-panel.py` fails closed on a truth cell
with no engine record; `auth` reads 0/12 [0.00, 0.24]. [measured, re-run] Decision unchanged.

---

*Later errata are appended under the record's heading with the next `E-NNNN-k` id. A heading with no entries is not
created.*
