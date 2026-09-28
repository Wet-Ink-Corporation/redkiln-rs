---
id: "0184"
title: "**The `Gate:` value carries kind, reach and state, and its deadline is integer unix seconds** (`s8e`'s record; D21-42 8b; D21-43 N6 and N8; D21-46 R2's grammar half): the value becomes `<token> SP <tree_hash> SP <deadline> SP <kind> SP <reach> SP <state> [SP <outcome>…]`, the state one of `opened`, `answered`, `expired` or `voided`; by the deleted-database test an `answered` mark carries the chosen option and an `expired` mark its terminal action (and an `auto-decide`'s verdict), because each decides the next action; the old `(token, tree_hash, deadline)` triple is legacy drift and an RFC 3339 deadline is malformed drift; `reconcile` folds the latest state per token and keeps every transition; `gate_expired` and `gate_answer_void` ride the `Gate` axis, and `gate_answer_void` moves to tier 1 with the kernel at entry as its producer. Supersedes in part `0105` clause 3's value delegation for `Gate:`, and design §5.4's `Gate:` row with its RFC 3339 spelling and design §5's `gate_answer_void` tier and producer"
status: proposed
date: 2026-09-28
legacy_id: none
source: spikes/DECISIONS-2026-09-21.md D21-42 (row 8b — *"**The `Gate:` value is widened** to carry kind and reach. Supersedes design §5.4's grammar row and ADR-0105 cl.3's value delegation in part."*), D21-43 (N6 — *"**Every gate transition (opened, answered, expired, voided) is a git-first tier-1 event** with a `Gate:` trailer carrying its state. `t1_gate` is append-only and state is a fold. Extends D21-42 8b's record."*; N8 — *"**Integer unix seconds.** Supersedes design §5.4's RFC 3339 spelling in part."*), D21-46 (R2) and D21-48 (X11 — *"Accepted as a stated residual. RK-D-UNREAD reads null as an unknown run, never as a match. The `Gate:` grammar does not widen."*); runbook/phase-1/s8e-tier-one-and-the-lane.md (the records table's third row, §8e.7, §8e.12 and §8e.13 findings 1, 5 and 6); crates/rk-git/src/trailer/grammar.rs, crates/rk-git/src/reconcile.rs, crates/rk-types/src/event_kind.rs, crates/rk-kernel/src/workflow/fsm.rs, crates/rk-kernel/src/workflow/gate.rs, crates/rk-kernel/src/runtime/step.rs, crates/rk-cli/src/verbs/run.rs, crates/rk-cli/src/verbs/answer.rs and crates/rk-cli/src/doctor/rebuild.rs, as read and changed 2026-09-28 at `d80609b`; references/2026-09-08-architecture-design.md §5 and §5.4; references/2026-09-08-acceptance-criteria.md AC-161, AC-174, AC-180, AC-183 and AC-444
supersedes: ["0105"]
superseded_by: []
related: ["0017", "0092", "0105", "0109", "0123", "0131", "0163", "0181", "0183"]
evidence: ["F12"]
consumers: []
---

## Context and problem

**No gate survives a deleted database.** `doctor/rebuild.rs`'s header, as it stood at `s8e`'s open:
*"The fold's `gate_opened` arm REQUIRES `kind` and `deadline` and **derives** the token from `(kind,
tree_hash, deadline, reach)` … The `Gate:` trailer carries `(token, tree_hash, deadline)` and no `kind` or
`reach`."* So a rebuilt gate row would either be malformed or carry a token that is not the trailer's, and
AC-161's *"the gate records the trailers carry"* had nothing to carry. `s7d` §7d.18 RAISED the widening.

**An opening and an answer were indistinguishable in git.** `answer` wrote *"THE SAME TRIPLE `run`
writes"*, and `reconcile` kept the latest mark per token. After a deleted database the rebuild could not
tell an answered gate from an open one, and the next action differs (`AwaitGate` against proceed).

**`gate_expired` had no carrier.** `EventKind::tier()` lists `GateExpired` under tier 1 while
`trailer_axis()` mapped it to `None`, so the expiry pass appended it to the store alone — the shape
ADR-0109's forbids bullet names: *"Forbids a tier-1 transition recorded only in the store"*.

**The deadline spelling disagreed with every producer.** Design §5.4's row says *"`deadline` RFC 3339"*.
Both producers wrote integers — `terminal_read` from the row's `as_i64`, `answer` from
`record.deadline().unix_seconds()` — and ADR-0163 already found that *"no crate formats RFC 3339"*.

**Measured 2026-09-28 at `d80609b`, across all refs** (`git log --all
--format='%(trailers:key=Gate,valueonly=true)'`): **0** `Gate:` values. No repository value moves; only
fixtures and F12's corpus are read differently.

The operator took the decisions: D21-42 8b (widen with kind and reach), D21-43 N6 (every transition
git-first, with its state), N8 (integer seconds), and D21-48 X11 (the rebuilt opening's null run). This
record writes them down, fixes each token's grammar, and decides the three things `s8e` §8e.7 left to it:
whether `answered` carries the option, ADR-0109 decision 2's list, and the order assumption.

## Decision drivers

- The deleted-database test decides what git must carry: *"if the database were deleted, would `redkiln
  run` take a different next action? If yes, the state may not live only in the database."* (design §5.4).
- A gate's identity is its token, and the token is a content hash the kernel derives from `(kind,
  tree_hash, deadline, reach)` (`GateRecord::derive_reach`), so a rebuild that holds all four can check the
  token instead of inventing one.
- Malformed values are drift, never coerced (AC-183); a legacy value is counted, never a transition.
- `t1_gate_transition` is append-only (ADR-0181 decision 5; D21-43 N6), so the state is a fold over marks.

## Considered options

### Option — six tokens with the state last, plus the outcome tokens the state takes (taken)

D21-42 8b adds kind and reach; N6 adds the state; the deleted-database test adds the outcome where one
decides the next action. The first five tokens are the gate's identity and are the same on every mark for
a token; the sixth says which transition this mark is.

### Option — the state as a separate trailer key (`Gate-State:`)

Rejected: a fourth axis (ADR-0109 decision 3 fixes three) and two lines that must be read together, which
a partial rewrite could split.

### Option — `answered` without the option

Rejected by the deleted-database test. The option is read to decide the next action in two places:
`gate::answered_facet` binds an answered `unknown_facet:<axis>` gate's verdict as the facet's value, and
`runtime::step`'s `gate_answer_at` hands a gate step `option: answer.verdict()` to route on. With the store
deleted, an answered mark without the option would rebuild a gate that is answered with no answer.

### Option — keep the triple and add the members as separate tokens only on openings

Rejected: an answered, expired or voided mark would then not carry the identity a rebuild needs to derive
and check the token.

## Decision

**1. The value.** `<token> SP <tree_hash> SP <deadline> SP <kind> SP <reach> SP <state> [SP <outcome>…]`,
split on whitespace. Each token's grammar:
- `<token>`: non-empty, no whitespace. It is the kernel's derived digest (hex); `rk-git` does not
  re-derive it, and a rebuild that holds the four members below can.
- `<tree_hash>`: 40-hex, the gate's bound tree (ADR-0105 clause 4), the same on every mark for the token.
- `<deadline>`: **integer unix seconds** as unsigned ASCII decimal digits (D21-43 N8). An RFC 3339
  spelling, a sign or a fraction is malformed drift. `t1_gate_transition.deadline` is `TEXT` and holds the
  decimal (ADR-0181 decision 5).
- `<kind>`: non-empty, no whitespace — the `GateRecord::kind()` string, for example
  `unknown_facet:criticality`. **The mint refuses a kind carrying whitespace** rather than writing a value
  that splits differently (`s8e` §8e.3's `tier_one::a_gate_kind_with_whitespace_is_refused_at_the_mint`).
- `<reach>`: a positive integer, the gate step's reach (the first is 1; ADR-0105 clause 10's fresh mark
  per re-reach).
- `<state>` and its outcome tokens, exactly:

| state | outcome tokens | why the outcome is in git |
| --- | --- | --- |
| `opened` | none | — |
| `answered` | `<option>`, one token, no whitespace | the chosen option decides the next action (the Considered options) |
| `expired` | `fail`, `escalate`, or `auto-decide <verdict>` | `fail` halts the run and `auto-decide` answers the gate with `<verdict>`; `escalate`'s effect is its own `opened` mark for the escalation gate |
| `voided` | none | a void answer re-opens the gate (ADR-0109 decision 1); nothing further decides |

A value whose token count is not six to eight — **the old triple included** — is **legacy drift**
(`MalformedReason::LegacyGateValue` → `legacy_gate_trailer`), counted, surfaced with its text, never a
gate, exactly as design §5.4 row 3 treated a non-triple. A value with the count whose tokens break their
grammar (a tree that is not 40-hex, a non-integer deadline, reach 0, an unknown state, outcome tokens the
state does not take) is **malformed drift** (`MalformedReason::MalformedGateValue` →
`unparsable_trailer` on the `Gate` key).

**2. The record-owned choice on the option: `answered` carries it,** by the deleted-database test, as the
table says. `s8e` §8e.7 asked the record to decide this *"by that test, and say so"*. The same test,
applied to the other states, gives `expired` its action and `auto-decide`'s verdict; this record takes that
extension and names it for the adversarial pass, because §8e.7 did not raise it. The answered row of
`t1_gate_transition` writes the option into its `option_id` column (ADR-0181 decision 5).

**3. The fold.** `reconcile` folds a gate's **latest state per token**, commits oldest-first (the order
ADR-0183 decision 4 states), and keeps **every transition** in the token's history, so a rebuild emits the
opening as well as the answer. A rebuild writes each transition into `t1_gate_transition` as its anchored
row (ADR-0181), which discharges `s7d` §7d.18's RAISED widening item when `s8e` §8e.9 lands it.

**4. Every transition is a git-first tier-1 mark (N6).** An opening, an answer, an expiry and a void each
commit their `Gate:` mark through the kernel's boundary function before any row (ADR-0181 decision 7).
- **`gate_expired` has its carrier**: a `Gate: … expired <action>` mark, and `EventKind::trailer_axis()`
  maps `GateExpired` to `Gate`.
- **`gate_answer_void` moves to tier 1.** N6 makes the voiding a git-first transition, so design §5's row
  changes its tier from `2 (drift observation)` to `1 (Gate: trailer)` and its producer from
  *"`reconcile()`"* to the kernel at entry (`s8e` §8e.3). `reconcile` keeps only the observation (the
  `Drift::GateAnswerVoid` value), and writes nothing to git (AC-174). `EventKind::tier()` and
  `trailer_axis()` move with the row.

**5. ADR-0109 decision 2's list is read as illustrative.** Its parenthetical is *"Tier 1 (stage
transitions, gate opened/answered, verdicts, slice seals)"*. `gate_expired` was already tier 1 in design §5
and in `EventKind::tier()` before this record, so the list was never exhaustive of the design; the
decision's operative sentence is the test, and the list names examples of what it selects. This record
therefore does **not** supersede decision 2. It names the reading for its adversarial pass: if the list is
judged enumerative, this record supersedes that parenthetical in part — to add `gate_expired` and
`gate_answer_void` — and nothing else of decision 2.

**6. The opening's run is a column, not a token (D21-48 X11).** The `opened` row of `t1_gate_transition`
carries the opening stage run's `stage` and `run_id` from the kernel's `OpenGate` act (ADR-0181 decision
2). The `Gate:` value does not carry the run, so an `opened` row `doctor --rebuild` re-derives has a
**null `run_id`**. The operator accepted that residual; the rebuild never invents a value, and `Gate:`
gains no run token. `RK-D-UNREAD` reads the null as an unknown run, never as a match (`s8f` pins it).

**7. No new gate kind, so no `gate_expiry` row.** The per-class `gate_expiry` table is `s8h`'s (D21-42
21a); this record mints states, not kinds.

**8. Supersessions (ADR-0092 rule 1), each in part.**
- **`0105` clause 3**, only its delegation of the `Gate:` value to design §5.4's triple row (*"values
  follow the trailer grammar table in design §5.4"*). The rest of the clause stands; the legacy `Gate:`
  drift rule it names (*"the legacy `Gate:` drift rule stands"*) stands, applied to the widened count.
- **Design §5.4's `Gate:` row**, its triple and its *"`deadline` RFC 3339"* spelling, amended by a dated
  line; and **design §5's `gate_answer_void` row**, its tier and producer, amended by a dated line.

**9. Cited, not superseded.** ADR-0105 clause 10 (a fresh mark per re-reach); ADR-0109 decisions 1 (a
void answer re-opens the gate) and 2 (read as illustrative, decision 5); ADR-0131 decision 6 (the store is
disposable, which is R2's ground); ADR-0017 (the single-use token); AC-174 (the void is minted by the
kernel, not by `reconcile`); AC-180 (the migration is explicit, ADR-0181's); `s7d` §7d.18's RAISED item,
which this discharges with ADR-0181.

## Consequences

**Positive**
- A deleted database loses no gate: the state, the option and the expiry action come back from git, and
  the token can be checked against its derivation.
- An answered gate and an open gate are told apart in git, so the rebuilt next action is the recorded one.
- Every tier-1 kind names its trailer axis; `EventKind::tier()` and `trailer_axis()` agree with no
  exception.

**Negative**
- An `answered` or `expired` mark is longer than six tokens, so a reader that takes the state as the last
  token is wrong; the state is the sixth.
- F12's corpus triples are now legacy drift twice over (count and deadline). AC-444's drift counts are
  corpus-bound Phase 2; the register carries a dated note and nothing Phase 1 asserts is re-pinned.
- A gate's `opened` row rebuilt from git has a null `run_id` (decision 6).

**What this forbids**
- A `Gate:` value in the triple form, or with an RFC 3339, signed or fractional deadline, written by any
  producer.
- A kind, option or verdict token carrying whitespace, written by any producer.
- An `answered` mark without its option, or an `expired` mark without its action.
- A tier-1 gate transition — opening, answer, expiry or void — recorded only in the store.
- Reading the state as a `Gate:` value's last token.
- A `Gate:` run token, or a rebuilt `opened` row with an invented `run_id` (D21-48 X11).
- Minting a gate kind in this record's name.

## Evidence and confidence

**Measured** (2026-09-28 at `d80609b`): zero `Gate:` values across all refs. **Built and tested in `s8e`
(package S8E-P4), red first:** `trailers::the_widened_gate_value_carries_kind_reach_and_state`,
`trailers::a_gate_value_without_kind_reach_and_state_is_legacy_drift`,
`trailers::reconcile_folds_a_gates_latest_state_per_token` and
`trailers::an_rfc3339_gate_deadline_is_malformed_drift` in `rk-git`; `rk-git`'s `grammar` test that fed an
RFC 3339 triple is re-pointed, not deleted; `event_kinds::exactly_five_kinds_are_tier_one` and
`event_kinds::the_trailer_axes_belong_to_the_tier_one_kinds` are re-pointed for `gate_answer_void`'s tier
and the two axes. Mutant S8E-M13's `rk-git` half (the value drops kind and reach, in the renderer and in the
parser) reddens the widened-value row. The existing producers (`terminal_read`, `answer`) write the widened
value through `rk-git`'s round-trip-checked renderer. **Not yet built, and named:** every transition through
the kernel's boundary (`s8e` §8e.3), `terminal_read` reading the fold's awaited gate (§8e.7), and the
rebuild writing `t1_gate_transition` (§8e.9), measured by §8e.11's `lane_e2e` tests and
`doctor::rebuild_recovers_every_gate_state_from_the_widened_gate_value`. This record is `proposed` and flips
only on the operator's dated ledger line.
