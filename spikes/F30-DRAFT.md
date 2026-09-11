> **Drafted 2026-09-06, promoted 2026-09-11; the h1b-24h read is still not taken.** Promoted from the linked
> worktree (`D:/repos/redkiln-rs.swt`, where it lived untracked) into the main checkout under the drafts' naming
> convention (audit PIA-019). The 2026-09-06 mark for `h1b-24h` is spent: METHOD-S §7's 2026-09-06 correction
> requires a fresh mark plus a ≥ 24 h no-dispatch window on the machine before the read (PIA-109). Not merged
> into FINDINGS.md; FINDINGS.md's "Continued in" index (2026-09-11) points here.
>
> *Original header (2026-09-06):* DRAFT for review — not committed, not merged into FINDINGS.md.
> The `h1b-24h` arm is still marked and unread; its row is filled in after
> 2026-09-07T18:55:48Z. Everything else below is complete.

## F30 — S-H1b/S-WT RESULT: `--system-prompt` makes worktree fan-out free and makes a timed gate cost the whole prefix

**Established by:** 16 haiku dispatches, 2026-09-06 18:45–20:02Z, corpus pin
`47faec9f`, `claude` 2.1.263. `results/s-h1b-1m.jsonl`,
`results/s-h1b-65m.jsonl`, `results/s-h1b-24h.jsonl`, `results/s-wt.jsonl`,
`results/instrument-checks/s-wt-smoke.jsonl`. **Spend: $0.19, 390 s of wall
clock.** Five-hour headroom 0.18 → 0.18 and seven-day 0.10 → 0.10 across the
whole set — below instrument resolution, as F14 said it would be. Pre-registered
in `METHOD-S.md` §4a and §4b, with five instrument disclosures and one mid-run
correction appended to §7 before the records they govern.

Both spikes ask the same question of the same flag on two different axes. F16
measured `--system-prompt` holding cache-read share at 1.000 across **commits**,
and the proposal priced two things on the inference that it would therefore hold
across **space** (Phase 2's one-worktree-per-story DAG) and across **time** (the
human gate). One of those inferences is right and the other is backwards.

---

### S-WT — worktree fan-out is cache-free under the card rule

E1 arm (d)'s topology exactly: ten detached worktrees of one fixture repo, one
`probe --repeats 1` each, in sequence, all with E1 arm (e)'s static system prompt.

| dispatch | cache_read | cache_creation | share |
| --- | --- | --- | --- |
| w1 | 15,129 | 3,623 | 0.807 |
| w2 | 18,752 | **0** | **1.000** |
| w3 | 18,752 | **0** | **1.000** |
| w4 | 18,752 | **0** | **1.000** |
| w5 | 18,752 | **0** | **1.000** |
| w6 | 18,752 | **0** | **1.000** |
| w7 | 18,752 | **0** | **1.000** |
| w8 | 18,752 | **0** | **1.000** |
| w9 | 18,752 | **0** | **1.000** |
| w10 | 18,752 | **0** | **1.000** |

**Steady state w2..w10: 1.000, nine for nine, zero creation.**

| comparator | share | prefix |
| --- | --- | --- |
| E1 arm (d) — worktrees, default prompt | 0.685 | 25,451 |
| E1 arm (e) — one directory, `--system-prompt` | 1.000 | 18,752 |
| **S-WT — worktrees, `--system-prompt`** | **1.000** | **18,752** |

This is §4b.3's **first branch — `≥ 0.95`**: *"worktree fan-out is cache-free
under the card rule; Phase 2's DAG parallelism carries no cache penalty and the
proposal's '~0 under `--system-prompt`' is measured rather than inferred."* The
registered prediction (`≥ 0.95`) is confirmed, and 18,752 is arm (e)'s prefix
**to the token** — a worktree costs exactly what one directory costs.

Two things are stronger than the registered metric required. The prefix is also
**6,699 tokens smaller** than arm (d)'s 25,451, so the flag deletes the
per-directory block rather than merely stabilising it; and w1's 0.807 is not the
cold write F16 arm (e) paid, because the tool block was already warm from S-H1b
(see *Recorded* below) — the genuinely new bytes in the first worktree were 3,623.

---

### S-H1b — the same flag, across time, costs the whole prefix

Same instrument as H1 (`ttl-mark`/`ttl-read`, the 37 KB review-scale
`payload.txt`, sha `cff4af90…`), with the static card text on both ends of every
pair, and — per §4a.2 as amended — each arm in its own copy of the fixture
directory, its own results file, and its mark the last dispatch of any kind
before its read.

| arm | gap | cache_read | cache_creation | share | H1 comparator (default prompt) |
| --- | --- | --- | --- | --- | --- |
| `h1b-1m` | 1.48 min | 29,619 | **0** | **1.000** | 0.9 min: 0 / 1.000 |
| `h1b-65m` | 66.08 min | 0 | **29,619** | **0.000** | 65.2 min: 18,603 / 0.484 |
| `h1b-24h` | *pending* | | | | 49.7 h: 18,603 / 0.484 |

**Which §4a.3 branch the 65-minute read lands in: none of the three as
written.** §4a.3 offered `≈ 0`, `≈ 18.6k`, or "something in between". The
measured value is **29,619 — the entire prefix, 59% *more* than H1's 18,603.**
The registered prediction of **4k–10k creation is falsified**, and falsified in
the direction opposite to the one the prediction hedged against.

The nearest registered branch is the second (*"the volatile segment is not the
system prompt's git section alone; F28's flat cost stands and nothing
changes"*) — its conclusion holds, but its number does not: under this flag the
cost is not flat at 18.6k, it is the whole prefix.

**Why the number exceeds H1's, and why that is not a surprise.** F28's own
2026-09-05 correction already established this mechanism and already recorded a
`--system-prompt` read at 66.8 min with share 0.000 and creation 29,680. H1's
surviving 48% was never durability; it was the **shared-segment subsidy** that
this programme's constant unrelated dispatching kept warm, and `--system-prompt`
forfeits it. S-H1b's contribution is therefore **replication, not discovery** —
29,619 against 29,680, within 61 tokens — but replication under conditions F28's
arm did not have: a directory never dispatched in, one mark per results file,
and no dispatch of any kind between the mark and the read.

The `h1b-1m` control is new, and it is what makes the 65-minute number
attributable to time. The same prefix, in the same directory, under the same
flag, **read back at 1.48 minutes for zero creation**. Nothing about a static
prompt is inherently uncacheable; what expires, expires on the clock.

---

### What this settles

1. **Phase 2's DAG parallelism is correctly priced at zero.** The proposal's
   "~0 under `--system-prompt`" for one-worktree-per-story was an untested
   sentence in the F16 amendment. It is now measured, at 1.000 across nine
   worktrees, and the per-directory cost F13 put at 31% is not reduced under the
   card rule — it is **absent**.

2. **The human gate is not made free by the card rule; it is made maximally
   expensive.** §4a.1's stated hope — *"the gate is free rather than nearly free,
   and the design's 'one static system prompt per card' rule buys the human gate
   as well as the checkpoint commits"* — does not hold. Under the card rule a
   gate that outlives the TTL costs one **full** prefix re-write (29,619 here on
   a review-scale payload) where the default prompt costs 18,603 of it. D-rule
   *"gate placement is not a cost decision"* gains no second leg from §4a; it
   keeps only the leg F28 gave it, that the cost does not grow with duration.

3. **The two axes want opposite mitigations, so the design must choose per axis
   rather than adopt one flag globally.** F28 stated this for commits vs. time;
   S-WT and S-H1b extend it to space vs. time and measure both ends:

   | threat | best mitigation | measured |
   | --- | --- | --- |
   | commits during a gate | `--system-prompt` | F16: 1.000 across commits |
   | **worktree per story** | **`--system-prompt`** | **S-WT: 1.000 across 9 worktrees** |
   | **elapsed time during a gate** | **the default prompt** | **S-H1b: 0.000 vs H1's 0.484** |

   A card that is static for the parallelism win pays for it on any gate that
   crosses the TTL. Whether that trade is worth taking is a design decision this
   spike does not make; both numbers now exist for it.

---

### Recorded, and NOT licensed as a conclusion by §4a.3 or §4b.3

The set incidentally decomposes the prefix, because three dispatches ran within
six minutes under three **different** system-prompt texts:

| dispatch | system prompt | cache_read | cache_creation |
| --- | --- | --- | --- |
| smoke, 18:45 | "You are a smoke check. Reply ok." | 0 | 18,749 |
| `h1b-1m` mark, 18:48 | "You are a careful software reviewer…" | **15,129** | 14,490 |
| `s-wt` w1, 18:50 | "You are a terse assistant…" | **15,129** | 3,623 |

The same **15,129** tokens were read warm under all three texts;
`15,129 + 3,623 = 18,752` is the whole S-WT prefix, and
`14,490 − 3,623 = 10,867` is the 37 KB payload. That is consistent with a
tool-definition block sitting **before** the system prompt and shared across
system prompts, with only ~3.6k of system and settings after it.

If that is right, F28's attribution of the surviving 48% to *"the shared
**default system prompt**"* names the wrong block. **This is one observation of
opportunity, not a designed measurement**, and E1 arm (e) w1 was fully cold at
18,752 after arms (a)–(d), which the account above does not explain. It is
offered as the specification for a spike, not as a finding.

---

### Limits

1. **S-H1b's 65-minute read is not measured under H1's conditions.** H1 ran in
   `fixtures/h1`, which carried other traffic; S-H1b's arms are isolated and each
   long mark is the last dispatch of any kind before its read. Some of the gap
   between 0.000 and H1's 0.484 is that isolation, not the flag. The
   like-for-like comparator is F28's own `--system-prompt` arm (0.000 / 29,680),
   and against that one S-H1b agrees to 61 tokens.
2. **`claude` 2.1.263** here against **2.1.261** for every H1 record and 2.1.258
   for E1. Tool set and count are identical to E1 arm (d)'s 34;
   `slash_commands_count` moved 80 → 82.
3. **n = 1 per gap and per worktree.** F27's rule stands: nothing is inferred
   from a single draw. These are mechanical token counts rather than draws of a
   stochastic process — S-WT returned the identical integer nine times — but no
   variance is estimated and none is claimed.
4. **The `h1b-24h` arm is unread.** The 49.7 h leg of the H1 comparison has no
   S-H1b counterpart yet. Nothing above depends on it; it can only confirm or
   deny that the cost stays flat at 29,619 rather than growing.
5. **The prefix here is 29,619 tokens on a 37 KB payload.** "One full prefix
   re-write" is bounded by prefix size, so the absolute cost of a gate scales
   with the item under review, not with the gate.
6. **Two rig defects were found by reading the runner before running it**, both
   disclosed in §7, and either would have voided the set silently:
   `journal::last_warm_mark` pairs a read with the globally latest `warm` line in
   the `--out` file regardless of arm or cwd, so three arms in one file mis-pair;
   and `ProbeArgs.flags` lacks `allow_hyphen_values`, so the
   `--flag --system-prompt` form §4b.2 prescribes exits 2 without dispatching.
   METHOD-S §4b.2 and `prompts/run-s-h1b-and-s-wt.md` still carry the broken
   form. HANDOFF's rule — *validate one item before every set, and prefer a haiku
   probe* — caught the second one before it cost anything.

### Instrument checks, all passing

All 16 dispatches `outcome: ok`, `isolation_ok: true`, `mcp_servers: []`, 34
tools matching E1 arm (d)'s set member-for-member. Every S-H1b dispatch sent
`prompt_sha256 cff4af90…` / 37,169 bytes; every pair's mark and read carry the
identical `--system-prompt` string and the identical `effective.cwd`; and each of
the three per-arm results files contains exactly one `warm` line, so no pair can
have been formed against another arm's mark.
