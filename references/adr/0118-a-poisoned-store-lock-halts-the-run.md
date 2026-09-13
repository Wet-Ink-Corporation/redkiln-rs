---
id: "0118"
title: A poisoned store lock halts the run — it is not a twelfth StepError variant, ADR-0112's closed set stays at eleven, and ADR-0115's named residual is closed
status: accepted
date: 2026-09-13
legacy_id: none
source: references/adr/0115-the-event-store-port-is-held-behind-interior-mutability.md :135-138 (the residual, and its assignment to Step 7a); references/adr/0112-steperror-gains-platform-unsupported-and-overage.md :51 and :96 (the closed eleven, and the twelfth-variant prohibition); references/adr/0105-rk-git-and-rk-store-contracts-as-rules.md clause 7 :77-79 (one writer connection per process, behind a mutex, no queue); references/2026-09-08-acceptance-criteria.md AC-360 :5192-5197 (one dispatch event, and no path that terminates without appending it); references/2026-09-08-architecture-design.md :1707 (`rk-cli` is the composition root); runbook/phase-1/s7a-ports-and-fakes.md :45-46; runbook/00-preamble.md :53 (the stop rule, re-measured after this pass moved it from :50); crates/rk-types/src/error.rs and crates/rk-store/src/conn.rs at `5580f19`; the operator's decision of 2026-09-13 taken against the Step 7a audit — there is no `spikes/DECISIONS-2026-09-13.md` and this record is that decision's primary record
supersedes: []
superseded_by: []
related: ["0095", "0099", "0105", "0107", "0112", "0115"]
evidence: ["F41"]
consumers: []
---

## Context and problem

ADR-0115 introduced the one interior-mutability seam in the port set — `Arc<Mutex<dyn EventStore>>` — and named
one thing it did not settle. Its Evidence section, :135-138, in full:

> **Residual, named not closed:** lock **poisoning**. The Phase-1 store is one writer per process behind a mutex
> (ADR-0105 clause 7), so a poisoned lock means a panic already happened inside a dispatch. This record does not
> decide whether that surfaces as a typed `StepError` variant or as a halt; it is an open item for Step 7a, and it
> must not be silently recovered by `unwrap_or_else(|e| e.into_inner())`.

Three facts make that residual something a record has to close rather than a stage.

**The two routes are not symmetric.** ADR-0112 :51 — *"`StepError` is this closed set of eleven, and the kernel
and every adapter match it exhaustively"* — and :96 — *"Forbids a twelfth variant without a successor to this
record"*. The typed route is therefore not a stage's edit; it is a successor to ADR-0112 on top of a successor to
ADR-0115. The halt route is a successor to neither. A stage meeting this residual meets two options of very
different cost, with nothing telling it that.

**Step 7a cannot answer it, and Step 7a is where ADR-0115 sent it.** `00-preamble.md:53`: *"Where a stage is
tempted to decide something else, it stops."* And the stage has no call site to decide it at: ADR-0115 clause 4
puts the acquisition at the composition root's call boundary, and design :1707 makes the composition root
`rk-cli`, which is **Step 7d**. So the residual as filed would have been met first not by the stage it was
assigned to but by whoever writes the first `.lock()`, two stages later, in code, with no instruction — which is
the mechanism by which a free residual becomes a default.

**It has already happened once, in the mutex clause 7 names.** `crates/rk-store/src/conn.rs:137-140` at `5580f19`
reads

```rust
let mut guard = self
    .conn
    .lock()
    .unwrap_or_else(std::sync::PoisonError::into_inner);
```

with no comment on the recovery, in the file whose own module doc opens *"Connection discipline — ADR-0105 clause
7"* and quotes *"One writer connection per process, behind a mutex, no queue"*. It shipped at Step 6.3 (`7aa02c7`,
2026-09-13), before this record existed [measured]. It is the prohibited recovery in the method-path spelling;
ADR-0115 states the prohibition in the closure spelling, and a grep for the closure text does not find it.

## Decision drivers

- **A poisoned lock is a report about the process, not about the step.** Under ADR-0105 clause 7 there is one
  writer connection per process behind a mutex, with no queue, so a poisoned lock means a panic unwound while that
  single writer held the lock. What the writer was part-way through is not known, and nothing has re-established
  it. **This is [inferred] from clause 7's one-writer premise; it is not measured, and no poisoned store lock has
  been observed in this programme.**
- **A typed error invites a caller to continue.** `StepError` exists to say *how to carry on*: every variant
  carries a retry class and exactly one event arm (`crates/rk-types/src/error.rs`, `retry_class()` and `event()`).
  A variant whose only correct handling is "stop everything" is a variant in the wrong set.
- **AC-360 makes the typed route self-defeating.** AC-360 (:5192-5197) requires exactly one dispatch event on any
  termination *"AND no dispatch path exists that can terminate without appending that event"*. Appending goes
  through the store behind the poisoned lock. A `StepError` returned to the kernel would manufacture precisely the
  path AC-360 forbids: a dispatch terminating with its event unappendable. A halt does not create that path — the
  dispatch path does not terminate, the run stops.
- **The residual must be closed where a reader will meet it**, not left for the first call site. That is the whole
  of the defect this record repairs, and it is the same defect ADR-0115 was written to repair one level up: *"The
  narrowing must be stated so the next implementer does not re-derive it from a spike."*

## Decision

**1. A poisoned store lock halts the run.** The run stops at the poisoned acquisition. No `StepError` is
constructed, nothing crosses a port boundary, no retry, no next activity, no gate answer.

**2. ADR-0112 is NOT superseded and the closed set stays at eleven.** This record carries `supersedes: []`.
`crates/rk-types/src/error.rs` is untouched by it, and ADR-0112's *"Forbids a twelfth variant without a successor
to this record"* stands unmoved — this record deliberately does not become that successor.

**3. The prohibition ADR-0115 states survives and now has a record behind it.** A poisoned store lock may not be
silently recovered: not by `unwrap_or_else(|e| e.into_inner())`, not by
`unwrap_or_else(std::sync::PoisonError::into_inner)`, not by any other spelling of `PoisonError::into_inner`, and
not by a `catch_unwind` that resumes the run.

**4. Scope: both store locks, and this record says so rather than leaving it to be read off.** Two distinct
mutexes are in play — the port handle's (`Arc<Mutex<dyn EventStore>>`, ADR-0115) and `rk-store`'s writer-connection
mutex (`Mutex<Connection>`, ADR-0105 clause 7). They are not the same object. Clauses 1 and 3 govern **both**,
because the argument is identical at each — one writer, a panic while its invariants were mid-flight — and because
a rule that covered one would leave the recovery available one crate over. `crates/rk-store/src/conn.rs:137-140`
is therefore a violation of this record from its acceptance, and **its repair is owed**. This record names the
violation and does not schedule it into a stage it does not own; carrying it as a tracked debt in `PROGRESS.md` is
owed and is not done here.

**5. Where the halt lives, and what each stage owes.** The acquisition is the composition root's (ADR-0115 clause
4: *"no port method acquires it"*), and the composition root is `rk-cli` (design :1707) at **Step 7d**. So:

- **Step 7a writes no `lock()` call site at all**, and therefore owes the **prohibition as an assertion**, not the
  halt's behaviour: that `crates/rk-ports/src/` contains no poison recovery. Written in Halt 8's hardened form —
  the directory proven present first, the search's status captured and asserted `-eq 1` before anything is read
  from its output — because a pass condition that is the absence of output exits 0 over a tree that is not there
  (Halt 8 shape (g)), and because a `grep` written against the closure spelling would have missed the one live
  instance this corpus has. That assertion is a floor, not evidence (golden constraint 12).
- **Step 7d owes the discriminating test**, and it is the only stage that can write one: a fixture that poisons the
  store lock (a thread that panics while holding it) and asserts the run stops — no `StepError` value produced, no
  recovered guard, no subsequent activity.

**6. What this record does not fix, named so it is not a residual in a smaller form.** The process exit code, the
diagnostic's wording, and whether the stop is reached by an unhandled panic or an explicit abort are **the CLI's
surface, owed at Step 7d under ADR-0107**, with `rk-cli` as the named consumer. What is fixed here and is not
Step 7d's to reopen: the stop is loud and names the poisoned lock, and it is never reported as a step failure, a
gate answer or a completed run.

**7. ADR-0115's other clauses stand.** Clauses 1–5 of ADR-0115 are untouched: the `&mut dyn EventStore` signature,
the four unchanged ports, the user-defined-wrapper prohibition and its test, the acquisition-and-release rule, and
the one-seam rule. This record closes exactly the residual ADR-0115's Evidence section names and nothing else.

**8. The relation to ADR-0115 is recorded in `related`, because this corpus has no other key for it.** The
frontmatter key list is fixed at eleven (README, *Format*), and the only relations it can express are
`supersedes`/`superseded_by` and `related`. **There is no `narrows`, `closes` or `resolves` key, and this record
does not invent one.** Nothing of ADR-0115 is withdrawn — a supersession would be false, and would move a record
that is right in every clause — so the relation is `related` plus this paragraph. ADR-0115 is frozen and is not
edited: its Evidence section will go on reading *"it is an open item for Step 7a"*, and this record is where the
reader finds the answer. If that relation is later wanted machine-visible, it is a decision about the key list, not
an edit here.

## Considered options

### Option — a twelfth `StepError` variant

`StorePoisoned`, retry class `Never`, `spoke()` false. **Rejected on three grounds, the first of which is
sufficient:** it has no honest event arm. AC-360 gives every variant exactly one appended event, and appending
needs the store the poisoned lock guards — so the one thing the variant would owe is the one thing it cannot do,
and returning it creates the dispatch path AC-360's AND clause forbids. Second, it needs a successor to ADR-0112
(:96) on top of a successor to ADR-0115, for a condition that is not a step outcome. Third, `StepError` is the set
the kernel matches to decide how to *continue*; a member that means "do not continue" weakens the set's meaning for
the other eleven.

### Option — recover with `into_inner()` and carry on

The shape already on the tree at `crates/rk-store/src/conn.rs:139-140`. **Rejected, and this record says why
rather than only that ADR-0115 forbids it:** the recovery continues over state no one re-established, and it
recovers *silently* — the panic that poisoned the lock is not what the operator is shown, and the next
inconsistency is attributed to whatever touched the store afterwards.

### Option — halt, and append a poisoning event on the way out

Attractive, and **rejected on ADR-0105 clause 7**: recording it would need a second writer connection while the
first is poisoned, and clause 7 is *"one writer connection per process, behind a mutex, no queue"*. A second writer
is a successor to ADR-0105, not a clause of this record. The cost is real and is carried in Consequences: **the
halt leaves no mark in the event log.**

### Option — leave it open and let Step 7d decide

**Rejected: it is the shape that produced this residual.** ADR-0115 left it to Step 7a; Step 7a writes no
acquisition; deferring again puts it in front of the first `.lock()` with no instruction, which is what
`00-preamble.md:53` and `INDEX.md`'s stop rule exist to prevent — and once, already, produced a silent recovery in
shipped code.

## Consequences

**Positive:**
- The first `.lock()` written at the composition root has an instruction, and the operator who meets a poisoned
  lock meets a decided behaviour rather than an implementer's reflex.
- The closed set stays closed. `StepError` remains eleven variants whose exhaustive matches (`spoke()`,
  `retry_class()`, `event()`) are the compile-time instrument ADR-0112 built them to be.
- AC-360's AND clause is not violated by a path this record could have created.
- A prohibition that existed in one spelling now covers the spelling that actually shipped, with an assertion owed
  at Step 7a and a discriminating test owed at Step 7d — both named, neither yet written.

**Negative:**
- **The halt leaves no durable mark.** The store is precisely what cannot be written, so the run's last event is
  whatever was appended before the panic, and `doctor` cannot report the poisoning from the log. The operator's
  evidence is the process's own diagnostic and the panic that caused it. Named, not solved.
- A halt is the harshest failure mode available, chosen for a condition whose rate is **unmeasured** — no poisoned
  store lock has been observed in this programme, and the first one may well be a test's.
- One live violation is now on the tree with its repair owed, where before there was an open item and no violation.
  That is the honest trade: the rule is what makes the existing line visible as a defect.

**What this forbids**
- Forbids silently recovering a poisoned store lock — `unwrap_or_else(|e| e.into_inner())`,
  `unwrap_or_else(std::sync::PoisonError::into_inner)`, any other spelling of `PoisonError::into_inner`, and any
  `catch_unwind` that resumes the run after one.
- Forbids mapping a poisoned store lock onto any `StepError` variant, and forbids minting a twelfth variant for it
  (ADR-0112 :96 is not superseded by this record).
- Forbids retrying the acquisition, dispatching another activity, or answering a gate after a poisoned acquisition.
- Forbids a port method acquiring the store lock (ADR-0115 clause 4 restated — it is what makes the composition
  root the only site the halt can live at).
- Forbids reporting the halt as a step failure, a gate answer or a completed run.
- Forbids opening a second writer connection to record the poisoning — that is ADR-0105 clause 7's rule and takes a
  successor to it.
- Forbids asserting this prohibition with an unguarded absence-of-output check, or with a pattern matched against
  the closure spelling alone.

## Evidence and confidence

- [documented, references/adr/0115 :135-138] the residual, its two routes, its assignment to Step 7a and the
  prohibition — quoted above at HEAD.
- [documented, references/adr/0112 :51, :96] the closed set of eleven and the twelfth-variant prohibition.
- [measured, `crates/rk-types/src/error.rs` at `5580f19`, shipped at Step 5 in `96ba9c2`] the enum carries exactly
  eleven variants, and `spoke()` and `retry_class()` are written as or-pattern matches with **no** catch-all — the
  file's own comment says why: *"`matches!(…)` expands to an implicit `_ => false` catch-all, under which a twelfth
  variant would compile silently."* A twelfth variant is `E0004` in three places plus its event arm.
- [documented, references/adr/0105 :77-79] *"One writer connection per process, behind a mutex, no queue."*
- [documented, references/2026-09-08-acceptance-criteria.md :5197] *"AND no dispatch path exists that can terminate
  without appending that event."*
- [documented, references/2026-09-08-architecture-design.md :1707] `rk-cli` is the composition root — Step 7d.
- [measured, 2026-09-13, `5580f19`] `grep -rn '\.lock()' --include=*.rs crates/` returns **one** hit,
  `crates/rk-store/src/conn.rs:139`, and it is recovered with `PoisonError::into_inner` at :140 with no comment;
  `grep -rn 'into_inner' --include=*.rs crates/` returns **two**, the other being `rk-git`'s own newtype accessor
  at `crates/rk-git/src/reconciled.rs:41` — which is why the Step 7a assertion is scoped to `crates/rk-ports/src/`
  and why a bare repo-wide `into_inner` grep is not the instrument.
- [measured, F41 / S-PORTS, 2026-09-11] `Arc<Mutex<dyn EventStore>>` compiles with the clause-1 signature on 1.97.1
  and the 1.98.1 pin. The reference shape decides nothing here: its only `.lock()` is the fake harness's own script
  mutex at `spikes/s-ports/src/lib.rs:202`, `.unwrap()`ed, and it is not a store lock.
- [measured, `Cargo.toml` :51-68] the workspace lint table denies `unsafe_code` and an enumerated clippy subset; it
  carries no `unwrap_used` or `expect_used`, so the halt needs no lint exemption and this record grants none.

**What is inferred, stated plainly.** The step from *one writer behind a mutex* (ADR-0105 clause 7) to *a poisoned
lock means the store's invariants are not known to hold, so the run must stop* is **[inferred]**. It is reasoning
from clause 7's premise and from AC-360's appending requirement. It is **not** a measurement, and it is not dressed
as one: no poisoned store lock has been observed in this programme, no rate is claimed, and `grep -rni poison
--include=*.rs crates/ spikes/` returns **1** — the recovery at `conn.rs:140`, not an occurrence of the condition.

**Confidence: high on the routing, reasoned on the halt.** What is settled and not in doubt is which record owns
what: ADR-0112 stays closed at eleven, ADR-0115 keeps every clause, the acquisition is the composition root's, and
the prohibition now has a record and an assertion. What rests on reasoning is the choice of halt over a typed
variant. **What would move it:** if the Phase-1 store ever grows a second writer or a queue, clause 7's premise —
the whole ground of driver 1 — changes, and this record takes a successor rather than an argument.
