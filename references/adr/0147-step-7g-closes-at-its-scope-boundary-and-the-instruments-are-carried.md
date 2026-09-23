---
id: "0147"
title: "Step 7g closes at **its own scope boundary** — design §4.5's five closed step kinds each executed as such, and the `Toolchain` port's first call site — with the executor's own semantics REPAIRED AND ASSERTED, and the **instrument**-class findings carried into Step 7h as recorded-OWED rather than repaired here. The split is *\"is this the executor, or the thing that MEASURES the executor?\"*. What closure does **not** claim is criterion (1): **0 of 102** shipped activities complete a committing walk, and that is criterion (1)'s business and 7h's, not 7g's. A stage that closes on a scope boundary must say which findings it is NOT closing, by number, or the boundary is a way of not counting them"
status: accepted
date: 2026-09-22
legacy_id: none
source: runbook/PROGRESS.md §1 row 37 (findings (1)-(23), the authoritative record of this stage's state); runbook/phase-1/s7g-step-executor.md (§7g.1's scope sentence); runbook/00-preamble.md (the two-commit close; a decision is not taken by implementing it)
supersedes: []
superseded_by: []
related: ["0134", "0135", "0136", "0137", "0138", "0139", "0140", "0141", "0142", "0143", "0144", "0145", "0146", "0148", "0149", "0150", "0151"]
evidence: []
consumers: []
---

## Context and problem

Step 7g was recorded **BLOCKED, not closed**, with its Result/commit column deliberately empty and
findings (1)–(23) in its Status cell. The stage's scope was built and committed; what was not true
was that the executor's semantics were *asserted*. An adversarial pass refuted the previous session
three times after its checkpoint, fmt, clippy and four suites were green, and **six mutations still
survived Checkpoint 7g** — guards (1), (2), (4), (5) and (6) having killed nothing in seventeen
attempts.

The row could not be closed until someone said what closure means here, and that is an operator's
decision rather than an implementer's: the stage's scope IS built, and the thing that most obviously
"does not work" — no shipped activity completing a committing walk — is **not in its scope**.

## Decision drivers

- **The scope sentence is narrow and was met.** §7g.1: *"the step executor — design §4.5's five
  closed step kinds each executed as such, and the `Toolchain` port's first call site."* Five arms
  exist, route, and are now each asserted; the port has a non-test call site with an adapter
  observed behind it.
- **"Built" was never the bar in this runbook; "asserted" is.** Finding (19) measured eleven
  mutations surviving the block. Closing on *built* would repeat the failure the whole stage is a
  record of.
- **Not every finding is about the executor.** Several are about the instruments — a source lint's
  blind spot, a checker's regex, a worktree guard's arithmetic, a heading corpus. Those are real and
  they are not made true or false by anything the executor does.
- **A scope boundary is only honest if it enumerates what it excludes.** Naming the carried findings
  by number, with their measurements intact, is what stops "out of scope" from meaning "forgotten".

## Considered options

**(a) Stay BLOCKED until every one of (1)–(23) is repaired.** Rejected by the operator. It makes the
stage's blast radius the whole instrument set: (16)'s repair edits a CLOSED stage's prose and (22)
touches the shipped pack, neither of which is the step executor. A stage that cannot close until
everything adjacent to it is perfect does not close.

**(b) Close immediately on "the scope is built", carrying all of (5)–(23) forward.** Rejected. It is
the *"built, not asserted"* reading this stage exists as a record of: six mutations survived
Checkpoint 7g at the time the question was asked, and closing there would have certified an executor
whose `emit` terminality, empty-list forbid and `Ok(false)` arm were asserted by nothing.

**(c) Repair the executor's own semantics, carry the instruments, and name them by number.** Taken.

**(d) Repair the instruments too, but silently.** Rejected on the same ground as (a) and worse: a
carried finding that is not written down is a finding that is lost, and the tracker row is the only
place a later stage will look.

## Decision

1. **Step 7g closes at its scope boundary**, with every finding that is the **executor's own
   semantics** repaired, recorded, and asserted by a test that kills a mutant that has been *read*:

   | finding | repair | record |
   | --- | --- | --- |
   | (20) the cannot-dispatch lint defeated three ways | a behavioural half: a counting `Toolchain` over seven `check` fixtures at six walk positions | this record |
   | (21a) `emit` terminality | `walk_semantics` | this record |
   | (21b) ADR-0138's empty-list forbid | `walk_semantics` | this record |
   | (21c) the `check` arm's `Ok(false)` | `check_cannot_dispatch` fixture C4 | this record |
   | (6) `toolchain.test(&[])` | refuse by name | **ADR-0148** |
   | (7) eager adapter validation | `detect()` chooses first | **ADR-0149** |
   | (5) `activity_committed` ordering | the grain stays; the residual is named | **ADR-0150** |
   | (23) the CRLF-pinned digest | the bytes are pinned, not normalised | **ADR-0151** |

2. **The instrument-class findings are carried into Step 7h as recorded-OWED**, by number, with
   their measurements: **(8)** `blank_string_literals` keeps no cross-line state; **(10)** the port
   lint excludes `src/replay.rs`, which is on the execution path; **(11)** the `StepKind` variant
   parse filters on `is_ascii_alphanumeric` and so drops a non-unit variant while `five.len() == 5`
   still holds; **(14)** the worktree guard takes `wt_before` before the prune and compares counts
   not paths; **(15)** `check-checkpoints.py` cannot flag PIPE-STATUS on a `cargo test` line;
   **(16)** `check-refs.py`'s heading corpus is overwhelmingly fenced bash comments — **415 of 555**
   at this close, of which only **140** are real headings *(row 37 records "403 of 543", which was
   true when measured and which THIS STAGE'S OWN EDITS then moved; recounted off disk here rather
   than carried, because carrying a numeral is how the sentence went stale in the first place)*;
   **(22)** a step gate's class has no `[[gate_expiry]]` row, for 0 of 45 classes.

3. **Closure does not claim criterion (1).** **0 of 102** shipped activities complete a committing
   walk. That is stated in the closing row rather than left to be inferred, and nothing downstream
   may read a closed 7g as "the executor runs the pack".

4. **Two of the carried findings are load-bearing for the guards that certify this stage**, and that
   is said here rather than discovered later: (8) is the defeat that blinds the cannot-dispatch
   source lint — which is why (20)'s repair is a **behavioural backstop** and not a widened ban
   list — and (15) means Checkpoint 7g's `flagged 0` is weaker evidence than it reads.

## Consequences

The stage closes with its scope discharged and seven findings explicitly owed to 7h. The gates at
closure: `rk-cli` **107**, `rk-kernel` **219 + 11 (5 ignored)**, `rk-adapter-claude` **102**,
`redkiln-migrate` **25**; `cargo fmt --all --check` rc=0; `cargo clippy --workspace --all-targets`
**0 errors**.

The measured kill matrix for (20) is the reason this record can say "asserted" rather than "built".
Four mutants, each diffed and read before either colour was believed: with the dispatch hidden
behind finding (8)'s truncating literal, the **source lint goes green — blind** and the behavioural
half goes **red**. That is the claim finding (20) makes, measured rather than argued.

🛑 **One mutant cycle in this stage produced a false reading and it is recorded rather than
smoothed.** The first attempt at defeat (c) put the truncating literal on the arm's opening line;
the lint went RED, but on `joined.len() > 40` — *"joined to 27 chars — the scan is vacuous"* — which
is the vacuity floor, not the ban. That draft was a weaker mutant than the finding describes, and
believing it would have credited the lint with catching something it cannot catch. Real code in
front of the literal made both floors hold and the truncation silent.

**What this forbids**

- Reading a closed Step 7g as evidence that any shipped activity executes.
- Closing a later stage's row on "built" where this one required "asserted".
- Dropping a carried finding from the tracker without a record that closes it.
- Treating (20)'s behavioural half as a repair of (8).

## Evidence and confidence

**Measured, not argued.** The closure rests on a kill matrix taken over four mutants of
`crates/rk-kernel/src/runtime/step.rs`, each applied by script, **diffed and read** before either
colour was believed, and restored byte-exact:

| mutant | source lint `the_check_arm_has_no_receiver_and_so_cannot_reach_a_port` | the new behavioural tests |
| --- | --- | --- |
| (20a) a `&super::Runtime` parameter named `rt` | RED | RED |
| (20b) `self.run_tool_step(step).and_then(…)` in the arm | RED | RED |
| **(20c)** the truncating literal, dispatch past the scan | **GREEN — blind** | **RED** |
| **(20ac)** (a) hidden behind (c) | **GREEN — blind** | **RED** |
| **(20f)** dispatch on the arm's NON-`Continue` paths only | **GREEN — blind** | **RED** *(see below)* |
| **(20g)** dispatch only when a `when` guard is present | **GREEN — blind** | **RED** *(see below)* |

and over three of the walk's own semantics, each run against the **whole** `rk-kernel` `it` binary:
`StepVerdict::Emitted => {}` → **218 passed / 1 failed**; `if false && steps.is_empty()` → 218 / 1;
`Ok(false) => StepVerdict::Continue` → **216 / 0 before** fixture C4 existed and **218 / 1 after**;
and ADR-0148's `toolchain.test(&[])` → 218 / 1. Each single failure is the new test and nothing
else, which is what makes "asserted by nothing until now" a measurement rather than a claim.

🛑 **THE LAST TWO ROWS WERE SURVIVORS, AND THEY ARE THE REASON THIS RECORD SAYS "ASSERTED" RATHER
THAN "COVERED".** An adversarial pass built **(20f)** and **(20g)** against the FIRST draft of the
behavioural half and both **passed all 219 tests and the source lint**. The first draft gave the
synthetic `run` key to one fixture whose `expr` returns `Ok(true)`, so a dispatch placed on the
arm's *other two* paths was unreachable by the counter; and no fixture carried a `when` guard that
**held**, so `step.when` was an unobserved discriminator inside the arm. Three blindnesses composed:
the wrong path, a missing `run` key, and `let _ =` discarding the refusal so the message needles
still matched. C1 and C4 now carry the key alongside C7 — one per outcome — and C8 carries a guard
that holds. Re-measured: both mutants **KILLED**. A fixture set that covers a function but not its
*branches* is the same defect as a guard whose fixtures all sit at one position, which this stage has
now paid for three times.

**Confidence is bounded and the bound is named.** The behavioural half does **not** subsume the
source lint: a `let _ = rt.run_tool_step(step)` on the pack's real `check` steps is genuinely
unobservable to a counter, because **0 of the 94** shipped `check` steps carry a `run` key — which is
why the module carries a second instrument (the arm's own refusal text) and one synthetic
`run`-bearing fixture, and says so in its header rather than claiming a clean kill.

Gates at closure: `rk-cli` **107**, `rk-kernel` **219 + 11 (5 ignored)**, `rk-adapter-claude` **102**,
`redkiln-migrate` **25**; `cargo fmt --all --check` rc=0; `cargo clippy --workspace --all-targets`
0 errors; Checkpoint 7g WHOLE rc=0 (`CP7G-END-OF-BLOCK reached`, 9 guards / 10 `cargo test`
invocations); Checkpoint 7a re-run WHOLE rc=0, because this stage added a port fake and 7a asserts an
exact count of them.
