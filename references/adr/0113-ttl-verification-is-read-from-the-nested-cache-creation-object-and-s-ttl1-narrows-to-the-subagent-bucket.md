---
id: "0113"
title: TTL verification is read from the nested cache_creation object — a -p run with ephemeral_1h > 0 and ephemeral_5m = 0 is verified 1 h — and S-TTL1 narrows to the subagent bucket and the billing-mode confound
status: accepted
date: 2026-09-11
legacy_id: none
source: spikes/FINDINGS.md F3 "Correction (2026-09-11)" (audit PIA-067); spikes/METHOD.md §E1 :247; spikes/runner/src/event.rs :112-165 (`ttl_split()`); references/adr/0078 clauses (ii) and (iv); runbook/PROGRESS.md §5 (found 2026-09-11, beyond the audit); Ryan's decision of 2026-09-11 (AskUserQuestion)
supersedes: ["0078"]
superseded_by: []
related: ["0010", "0016", "0017", "0110"]
evidence: ["F3", "F10", "F16", "F28", "F1", "S-WT", "S-H1b", "PIA-067"]
consumers: []
---

## Context and problem

ADR-0078 clause (iv) registered S-TTL1 on the belief that F3 had *measured the `ephemeral_*` fields absent* from the
`usage` block, and clause (ii) therefore made every run `TTL-unverified` until a spike established whether the
verification surface existed at all. The audit (PIA-067) found that reading wrong, and the 2026-09-11 recount over
every `spikes/results/*.jsonl` record carrying a `usage` block settled it [measured, F3 Correction]:

- F3's observation was about the **flat** naming from the costs page (`ephemeral_1h_input_tokens` at the top of
  `usage`), which is absent. The documented **nested** object —
  `usage.cache_creation { ephemeral_5m_input_tokens, ephemeral_1h_input_tokens }` — is present on **950 of 955**
  records (the five without are four void records and one P8 driver record with `digest: null`), and the runner has
  parsed it since E0 (`event.rs` `ttl_split()`, :155-165; METHOD.md §E1 already said "present").
- On all **804** records carrying both the total and the object, `cache_creation_input_tokens` equals the two tiers'
  sum. The object is the split, not a second accounting.
- **621 records carry `ephemeral_1h_input_tokens > 0` and not one carries `ephemeral_5m_input_tokens > 0`**; the 329
  "both zero" records are warm re-reads that created nothing. Every cache creation a `-p` dispatch reported in this
  programme was on the 1 h tier — under the pinned CLI's **default** main-bucket TTL (no run set either
  `CLAUDE_CODE_*_PROMPT_CACHE_TTL` variable) on a subscription drawing within plan usage (`isUsingOverage: false` on
  every recorded `rate_limit_event`, 80 rows — F1, PIA-065).

So the per-run evidence clause (ii) demanded exists and has been on the record since E0; clause (iv)'s standing
obligation "that no run can currently discharge" was discharged on every `-p` run the programme made, and its forbids
line — "forbids asserting that the `ephemeral_*` fields exist on this instrument, or that they do not, until S-TTL1
runs" — is false on 950 records. That is a Decision moving, so under ADR-0092 it is a supersession, not an erratum.

What the recount does **not** separate: the second bucket. The documentation places subagent, workflow, teammate,
fork and compaction traffic in a bucket that defaults to 5 m regardless of billing type, and every `forge-*` pipeline
this design ships is that traffic (ADR-0078 drivers). No recorded run carried a subagent card, so the per-run field
for bucket two is unobserved; and the billing-mode confound (usage credits silently forcing 5 m on an explicit 1 h
request) is untested because every recorded run was within plan usage.

This record supersedes ADR-0078 **in part**: clause (iv) and clause (ii)'s "no run can currently discharge" reading,
with the matching forbids lines. Clauses (i) and (iii) — the two-axis rule and the scope restatement — the
both-variables rule in (ii), the billing-mode confound and every other forbids line stand; ADR-0078 stays `accepted`.

## Decision drivers

- A rule that forbids a report the runner's own output licenses is worse than no rule: it makes every cost figure in
  design §7 a point estimate "at an unknown TTL" when the TTL is on the record.
- The per-run check is what makes a TTL claim falsifiable; it should be *read*, not *awaited*.
- The subagent bucket is where the design's traffic lives and where the documentation says the default differs, so
  the spike must be re-aimed there rather than retired.
- Billing mode changes the answer silently (ADR-0010's subscription-only rule makes plan-usage vs usage-credits the
  live confound); it is recorded per dispatch already (`isUsingOverage`), so the check is cheap.

## Decision

**1. TTL verification is read from the nested object, per run.** A dispatch's TTL is **verified 1 h** when its
`usage.cache_creation.ephemeral_1h_input_tokens > 0` and `ephemeral_5m_input_tokens == 0`; **verified 5 m** when the
converse holds; **mixed** when both are positive (reported as such, with both counts); and **`TTL-unverified`** only
when the object is absent or both tiers are zero *and* `cache_creation_input_tokens > 0`. A warm re-read (all three
zero) carries no TTL claim and is reported as `no-write`. The adapter records the tier split on every `step_*` event
that carries usage (ADR-0110's cost fields), and `redkiln cost` prints it.

**2. The 621 records are the `-p` bucket's evidence and are cited as such.** Design §0.1 row 28 and §7's cache model
may state that `-p` dispatches at the programme's CLI pins (2.1.259–2.1.263) on plan usage ran at 1 h **[measured, F3 Correction, n = 621 creations
/ 950 records]**. They may not carry that figure to subagent traffic or to usage-credit billing (clause (i) of
ADR-0078, unchanged: no benefit crosses an axis it was not measured on).

**3. S-TTL1 narrows to what is still unobserved.** The spike's question becomes: *on the pinned CLI with both TTL
variables set, does a dispatch carrying a subagent card report `ephemeral_1h_input_tokens > 0` for the subagent's own
cache creation, and does the same dispatch on usage-credit billing report 5 m?* Two dispatches (one per billing mode,
the second only if a usage-credit state can be produced honestly), raw `usage` retained verbatim, `rate_limit_info`
recorded beside it. Consumer: SM-113, SM-114, design §7; owed by Step 7c beside S-CANCEL-WIN and S-RETRY. Cost:
Haiku-scale, ≈ $0.03 per dispatch [measured, F3].

## Considered options

### Option — ERRATA entry only, keep `TTL-unverified` until S-TTL1 runs
Rejected: ADR-0078's forbids line is contradicted by the record, and an erratum may not move a Decision (ADR-0092
clause 1). It would also keep the design reporting a verification as blocked when the verification is in the data.

### Option — retire S-TTL1 as answered
Rejected: the question it was registered against is answered, but the bucket the design's traffic lives in is not
observed, and the documented default there is the opposite tier.

### Option — leave it for Step 7c
Rejected: 7c still owes the spike (decision 3), but the register should not carry a false forbids line for the
duration of Phase 0.

## Consequences

**Positive**
- Every `-p` cost figure in §7 gains its time axis: measured at 1 h, on plan usage, at the pin.
- The adapter's per-run TTL report is a read of a field it already parses — no new instrument.

**Negative**
- `TTL-unverified` becomes rare rather than universal, so a run that does report it is now a signal worth a doctor
  row (the object missing at the pinned CLI version is drift).
- The subagent bucket's tier stays a documented claim until S-TTL1 runs; design §7's subagent figures keep the
  `[documented]` tag until then.

**What this forbids**
- Forbids reporting a `-p` run as `TTL-unverified` when its `cache_creation` object carries a non-zero tier.
- Forbids carrying the 621-record 1 h result to subagent, workflow or fork traffic, or to usage-credit billing.
- Forbids asserting the subagent bucket's tier on this instrument until S-TTL1 has run.
- Forbids reading the flat `ephemeral_*` names as the verification surface; the nested object is the surface.

## Evidence and confidence

- [measured, FINDINGS F3 Correction, recount 2026-09-11] 950/955 records carry the nested object; 804/804 tier sums
  equal `cache_creation_input_tokens`; 621 records with `ephemeral_1h > 0`, 0 with `ephemeral_5m > 0`; 329 both-zero.
- [measured, F1, PIA-065] `isUsingOverage: false` on all 80 recorded `rate_limit_event` rows — plan usage throughout.
- [measured, F10] the main-bucket boundary bracketed in (45.1, 61.6) min — consistent with a 1 h tier, not a location
  of it.
- [documented, 1P code.claude.com/prompt-caching, fetched 2026-09-08 and 2026-09-10] the two buckets, the 5 m
  default for the non-main bucket regardless of billing, the usage-credit override.
- **What this evidence does not license.** It does not license any tier for subagent traffic, any statement about
  usage-credit billing, or a located boundary; and it does not touch ADR-0078's two-axis rule, which rests on
  F16/S-WT and F28/S-H1b, not on the TTL.
