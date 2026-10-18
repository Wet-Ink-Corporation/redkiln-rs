# Step 8 — The exit criterion

| stage | phase | entry gate | exit checkpoint | previous | next |
| --- | --- | --- | --- | --- | --- |
| `s8` | Phase 1 | Checkpoint 8j | Checkpoint 8 | `s8j` | — |

*Header re-pointed 2026-09-24 (D21-40; `s8j` A0, made at insertion): the entry gate read Checkpoint 7i and `previous` read
`s7i` until the eleven `s8` sub-stages `s8a`–`s8j` were inserted before this stage.*

*Part of `runbook/` — the index is `runbook/INDEX.md`, the tracker `runbook/PROGRESS.md`, the shared preamble
(golden constraints, tool preflight, legend) `runbook/00-preamble.md`, the halt conditions `runbook/halts.md`.
A bare `§N.M` in this file means this file's own section; another stage's section is cited as `s5 §5.3`. Every
shell block is POSIX and runs in Git Bash from `D:/repos/redkiln-rs` unless it opens with its own `cd`.*

---


Phase 1 exits on **six runnable checks** — design §10.2's seven **less (5), withdrawn 2026-09-15** (see (5) below;
`DECISIONS-2026-09-14 D-14`), with (1) now carrying a second half (ADR-0094: self-hosting is both lanes) — not on a
narrative milestone. **(5)'s number is retired and nothing renumbers**: the sequence is (1) (2) (3) (4) (6) (7),
because ADR-0110 (accepted) cites *"Phase-1 exit criterion (7)"* by number twice — in its Context and again as
its decision 5 — and the design carries the literal string *"§10.2 exit (6)"* at three sites: its §0.1
selection-engine row, its §10.2 phase table's own Phase-1 row and its §10.x review row. **Count them in
`references/2026-09-08-architecture-design.md` with `grep -cF`, never by line** — that design §10.2 Phase-1 row
gained its citation in this pass's own withdrawal edit, so a line-numbered count written earlier today was
already short by a site. A renumbering would silently move every one. All six, in order:

**(1) Self-hosting — AC-473, and AC-521.** Redkiln-RS runs its own fix lane end to end, **and** one story-lane run
authors a Phase-2 story under it (ADR-0094 decision 3; ADR-0011's exit bar, which ADR-0002's "solely" never
contradicted). Both run through the **Rust toolchain adapter** (`rk-adapter-toolchain-rust`) — this repository is a
Cargo workspace, and a Node-only Phase 1 could not have passed this line (PIA-004). *(dated 2026-10-02, `s8g` close: the verify stage's toolchain is now journalled, as `verify_completed` (`s8g` §8g.4; ADR-0191, accepted on D21-131), whose `adapter` member names the adapter `detect` chose and whose `origin = "pass"` marks the verify stage's own pass; (1)'s block below does not read it yet, and that amendment is `s8j`'s (D21-116; D21-129))* *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS: it does now. (1)'s check block reads `verify_completed`'s verify-stage passes only, by `origin = "pass"` (s8j §8j.8 A1))*

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **s8j §8j.8 A1 applied: the single block that stood here is replaced by four, in this order: (1) launch, (1) check, (1b) launch, (1b) check.** A launch block spends, under its own go-ahead `S8_GO=<item>` and after D21-47's pre-spend checks (`prespend`, pasted verbatim into each); a check block launches nothing and may be re-run freely. The order of acts for the fix item is (1) launch, the hold answered through `tools/live-run.sh answer`, (3)'s kill and resume blocks, any further gate answered and (1) launch again, then (1) check; the story item is (1b) launch, answers and relaunches as its gates require, then (1b) check (s8j §8j.8 A1). The superseded block follows the four, its fence marked `text` so that it is never run as written.

**(1) launch.**

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1) LAUNCH. ONE real run of the fix item, its own block and its own go-ahead
# (D21-47). Run from a plain terminal as `S8_GO=$FIX_ITEM bash <this block>`: s8h's launcher refuses a
# Claude-spawned shell. A gate-open exits 0 (C-01 D-01; AC-403); (1)'s check block tells a gated lane from a landed one.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT LAUNCHED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1) NOT LAUNCHED (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent — s8h's launcher is the only entry to a real run (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
# prespend(), pasted from s8j §8j.8 A1 verbatim (D21-47):
# D21-47's pre-spend checks (s8j §8j.8 A1), carried verbatim by every s8 block that launches a real run. It prints the
# defect and returns 1, or prints nothing. The go-ahead is per run: S8_GO=<item> as THIS block's command prefix.
prespend() {   # $1 = the item this block is about to launch
  pev=runbook/phase-1/s8j-evidence.txt
  CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
  [ "${S8_GO:-}" = "$1" ] || { echo "no go-ahead for $1: run this block as S8_GO=$1 bash <block>, one launch per go-ahead (D21-47)"; return 1; }
  test -s "$pev" || { echo "$pev is absent or empty"; return 1; }
  if od -c "$pev" | grep -qF '\r'; then echo "$pev carries a CR byte"; return 1; fi
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$pev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $pev"; return 1; }
  ci=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  read -r _ ci_run ci_sha _ <<< "$ci"
  [ -n "$ci_sha" ] && [ "$frozen" = "$ci_sha" ] || { echo "$1's frozen sha $frozen is not the CI-green sha '$ci_sha'"; return 1; }
  git merge-base --is-ancestor "$frozen" HEAD || { echo "HEAD does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" HEAD -- $CODE_PATHS || { echo "code, registry, items or intake changed since the frozen sha: re-run CI and re-record (s8j §8j.7)"; return 1; }
  fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  fp_sha=$(printf '%s\n' "$fp" | awk '{print $2}')
  [ -n "$fp_sha" ] && git diff --quiet "$fp_sha" "$frozen" -- $CODE_PATHS || { echo "no fixture-proofs line over the frozen code: the spend gate (D21-40 decision 2; s8j §8j.6 step 1b)"; return 1; }
  rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$pev" | tail -1)
  rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
  [ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; return 1; }
  if grep -qE '^[[:space:]]*harness_binary[[:space:]]*=' .redkiln/config.toml; then echo ".redkiln/config.toml names a harness_binary: a real run must reach the real CLI, never a fixture child"; return 1; fi
  if git merge-base --is-ancestor "rk-run/$1" HEAD 2>/dev/null; then echo "rk-run/$1 is already merged into HEAD: the run ref does not merge before Checkpoint 8"; return 1; fi
  j=$(gh run view "$ci_run" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha 2>&1) || { echo "gh run view $ci_run failed: $j"; return 1; }
  ok=$(printf '%s\n' "$j" | python -c 'import json,sys; d=json.load(sys.stdin); print("ok" if d.get("conclusion")=="success" and d.get("headSha")==sys.argv[1] else "no")' "$frozen")
  [ "$ok" = ok ] || { echo "CI run $ci_run is not green on $frozen"; return 1; }
}
why=$(prespend "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1) NOT LAUNCHED: $why"; exit 1; }
tools/live-run.sh run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero: stop and report, do not spend again (D21-40 decision 2)"; exit 1; }
echo "(1) launched once. Read explain; answer an open gate through tools/live-run.sh answer; then (3)'s blocks, or (1) launch again with a new go-ahead, and last (1)'s check block"
```

**(1) check.** It launches nothing.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1)'s CHECK — AC-473. It launches nothing and may be re-run freely.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1) NOT CHECKED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build (Halt 8)"; exit 1; }
# verify_completed's names (s8g §8g.4; D21-42 10b): the interim `item` key in the t2_event.item column (s8e §8e.10;
# D21-46 R3), the `adapter` payload member, and the member that marks the verify stage's own pass (s8g's review 4).
VC_ITEM_KEY=item
VC_ADAPTER_KEY=adapter
VC_ORIGIN_KEY=origin
VC_PASS=pass
grep -qxF 'name = "redkiln-rs"' .redkiln/config.toml || { echo "FAIL: [project] name is not redkiln-rs — the Story: value's <project> would not be what this block compares"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — the frozen sha this run must descend from is unrecorded"; exit 1; }
CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# The expected stage list, derived from the pack and never typed (D21-42 7): stage_order minus forbidden.
order='import sys,tomllib; p=tomllib.load(open(sys.argv[1],"rb"))["policy"]; print(" ".join(s for s in p["stage_order"] if p["stages"][s]!="forbidden"))'
# The lane's stage tokens: git's own trailer parser, oldest first, the LAST token of each Story: value; every value
# must be exactly `<project>/<item> <stage>` (the stage-bearing form of D21-41 (a)), and the last is N9's `closed`.
lane_check() {   # $1 item  $2 pack file — prints nothing on success, the defect otherwise
  exp=$(python -c "$order" "$2"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$exp" ] || { echo "the stage order could not be read from $2 (rc=$rc)"; return 1; }
  exp="$exp closed"   # D21-43 N9: a complete lane ends in the terminal 'Story: <project>/<item> closed' mark (s8e)
  git rev-parse --verify --quiet "refs/heads/rk-run/$1" >/dev/null || { echo "no rk-run/$1 ref — no run"; return 1; }
  if git merge-base --is-ancestor "rk-run/$1" HEAD; then echo "rk-run/$1 is merged into HEAD — it does not merge before Checkpoint 8"; return 1; fi
  base=$(git merge-base HEAD "rk-run/$1") || { echo "rk-run/$1 shares no history with HEAD"; return 1; }
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$ev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $ev"; return 1; }
  git merge-base --is-ancestor "$frozen" "$base" || { echo "rk-run/$1 forked from $base, which does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" "$base" -- $CODE_PATHS || { echo "code, registry, items or intake changed between the CI-green frozen sha and the run's fork — re-run CI and re-record (s8j §8j.7)"; return 1; }
  ahead=$(git rev-list --count "$base..rk-run/$1") || { echo "rev-list failed"; return 1; }
  [ "$ahead" -ge 1 ] || { echo "rk-run/$1 is $ahead commits ahead of its fork — a stale ref is not a run"; return 1; }
  lg=$(git log --reverse "$base..rk-run/$1" --format='%(trailers:key=Story,valueonly=true)') || { echo "git log over the run failed"; return 1; }
  bad=$(printf '%s\n' "$lg" | awk -v want="redkiln-rs/$1" 'NF && ($1!=want || NF!=2)')
  [ -z "$bad" ] || { echo "Story: values not of the form redkiln-rs/$1 <stage>: $bad"; return 1; }
  got=$(printf '%s\n' "$lg" | awk 'NF {printf "%s%s", sep, $NF; sep=" "}')
  [ "$got" = "$exp" ] || { echo "stages: expected '$exp', the run's trailers say '$got' — if explain shows an open gate, answer it through the launcher's answer mode (tools/live-run.sh answer; D21-46) and launch again with a new go-ahead (s8j §8j.3's hold is the first); a gated lane is not failed, and it is not a pass"; return 1; }
}
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to the item s8j §8j.5 reserved"; exit 1; }
test -f ".redkiln/items/$FIX_ITEM.toml" || { echo "FAIL: .redkiln/items/$FIX_ITEM.toml does not exist"; exit 1; }
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$FIX_ITEM" .redkiln/packs/bug-standard.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1): $why"; exit 1; }
base=$(git merge-base HEAD "rk-run/$FIX_ITEM") || { echo "FAIL: no merge base"; exit 1; }
git diff --quiet "$base" "rk-run/$FIX_ITEM" -- crates; rc=$?
[ "$rc" -eq 1 ] || { echo "FAIL (1): the run landed no content change under crates/ (rc=$rc) — trailers without a landed commit (D21-41 (b))"; exit 1; }
vc='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND "+sys.argv[3]+"=?",("verify_completed",sys.argv[2]))]; a=sorted({str(x.get(sys.argv[4])) for x in r if x.get(sys.argv[5])==sys.argv[6]}); print(" ".join(a))'
ids=$(python -c "$vc" "$store" "$FIX_ITEM" "$VC_ITEM_KEY" "$VC_ADAPTER_KEY" "$VC_ORIGIN_KEY" "$VC_PASS"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the verify_completed probe did not run (rc=$rc)"; exit 1; }
[ "$ids" = "rk-adapter-toolchain-rust" ] || { echo "FAIL (1): the verify stage's own passes name '$ids', not exactly rk-adapter-toolchain-rust (AC-473; D21-42 10b) — implement-stage calls are not the verify stage"; exit 1; }
om='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND item=?",("dispatch_completed",sys.argv[2]))]; print(len(r), sum(1 for x in r if str(x.get("observed_model") or "").strip()))'
cnt=$(python -c "$om" "$store" "$FIX_ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the observed_model probe did not run (rc=$rc)"; exit 1; }
read -r n_dc n_om <<< "$cnt"
[ "$n_dc" -ge 1 ] && [ "$n_om" = "$n_dc" ] || { echo "FAIL (1): $n_om of $n_dc dispatch_completed rows for $FIX_ITEM carry an observed_model — a real run's every dispatch names the model that answered (s8h)"; exit 1; }
printf '%s\n' "$out"; git log "$base..rk-run/$FIX_ITEM" --format='%h %s%n%(trailers)'   # → READ: no stage performed by the old redkiln; the landed commit is the fix. The run id is the evidence.
```

**(1b) launch.**

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1b) LAUNCH. ONE real run of the story item, its own block and its own go-ahead
# (D21-47): `S8_GO=$STORY_ITEM bash <this block>`, from a plain terminal.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1b) NOT LAUNCHED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build — criterion (1b) NOT LAUNCHED (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent — s8h's launcher is the only entry to a real run (D21-42 17a)"; exit 1; }
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
# prespend(), pasted from s8j §8j.8 A1 verbatim (D21-47):
# D21-47's pre-spend checks (s8j §8j.8 A1), carried verbatim by every s8 block that launches a real run. It prints the
# defect and returns 1, or prints nothing. The go-ahead is per run: S8_GO=<item> as THIS block's command prefix.
prespend() {   # $1 = the item this block is about to launch
  pev=runbook/phase-1/s8j-evidence.txt
  CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
  [ "${S8_GO:-}" = "$1" ] || { echo "no go-ahead for $1: run this block as S8_GO=$1 bash <block>, one launch per go-ahead (D21-47)"; return 1; }
  test -s "$pev" || { echo "$pev is absent or empty"; return 1; }
  if od -c "$pev" | grep -qF '\r'; then echo "$pev carries a CR byte"; return 1; fi
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$pev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $pev"; return 1; }
  ci=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  read -r _ ci_run ci_sha _ <<< "$ci"
  [ -n "$ci_sha" ] && [ "$frozen" = "$ci_sha" ] || { echo "$1's frozen sha $frozen is not the CI-green sha '$ci_sha'"; return 1; }
  git merge-base --is-ancestor "$frozen" HEAD || { echo "HEAD does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" HEAD -- $CODE_PATHS || { echo "code, registry, items or intake changed since the frozen sha: re-run CI and re-record (s8j §8j.7)"; return 1; }
  fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  fp_sha=$(printf '%s\n' "$fp" | awk '{print $2}')
  [ -n "$fp_sha" ] && git diff --quiet "$fp_sha" "$frozen" -- $CODE_PATHS || { echo "no fixture-proofs line over the frozen code: the spend gate (D21-40 decision 2; s8j §8j.6 step 1b)"; return 1; }
  rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$pev" | tail -1)
  rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
  [ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; return 1; }
  if grep -qE '^[[:space:]]*harness_binary[[:space:]]*=' .redkiln/config.toml; then echo ".redkiln/config.toml names a harness_binary: a real run must reach the real CLI, never a fixture child"; return 1; fi
  if git merge-base --is-ancestor "rk-run/$1" HEAD 2>/dev/null; then echo "rk-run/$1 is already merged into HEAD: the run ref does not merge before Checkpoint 8"; return 1; fi
  j=$(gh run view "$ci_run" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha 2>&1) || { echo "gh run view $ci_run failed: $j"; return 1; }
  ok=$(printf '%s\n' "$j" | python -c 'import json,sys; d=json.load(sys.stdin); print("ok" if d.get("conclusion")=="success" and d.get("headSha")==sys.argv[1] else "no")' "$frozen")
  [ "$ok" = ok ] || { echo "CI run $ci_run is not green on $frozen"; return 1; }
}
why=$(prespend "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b) NOT LAUNCHED: $why"; exit 1; }
tools/live-run.sh run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero: stop and report, do not spend again (D21-40 decision 2)"; exit 1; }
echo "(1b) launched once. Answer an open gate through tools/live-run.sh answer and launch again with a new go-ahead; last, (1b)'s check block"
```

**(1b) check.** It launches nothing.

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A1): (1b)'s CHECK — AC-521. It launches nothing and may be re-run freely.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (1b) NOT CHECKED (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent after a green build (Halt 8)"; exit 1; }
# (1)'s check block, from its name guard through lane_check's closing brace, pasted verbatim (s8j §8j.8 A1):
grep -qxF 'name = "redkiln-rs"' .redkiln/config.toml || { echo "FAIL: [project] name is not redkiln-rs — the Story: value's <project> would not be what this block compares"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
ev=runbook/phase-1/s8j-evidence.txt
test -s "$ev" || { echo "FAIL: $ev absent or empty — the frozen sha this run must descend from is unrecorded"; exit 1; }
CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
# The expected stage list, derived from the pack and never typed (D21-42 7): stage_order minus forbidden.
order='import sys,tomllib; p=tomllib.load(open(sys.argv[1],"rb"))["policy"]; print(" ".join(s for s in p["stage_order"] if p["stages"][s]!="forbidden"))'
# The lane's stage tokens: git's own trailer parser, oldest first, the LAST token of each Story: value; every value
# must be exactly `<project>/<item> <stage>` (the stage-bearing form of D21-41 (a)), and the last is N9's `closed`.
lane_check() {   # $1 item  $2 pack file — prints nothing on success, the defect otherwise
  exp=$(python -c "$order" "$2"); rc=$?
  [ "$rc" -eq 0 ] && [ -n "$exp" ] || { echo "the stage order could not be read from $2 (rc=$rc)"; return 1; }
  exp="$exp closed"   # D21-43 N9: a complete lane ends in the terminal 'Story: <project>/<item> closed' mark (s8e)
  git rev-parse --verify --quiet "refs/heads/rk-run/$1" >/dev/null || { echo "no rk-run/$1 ref — no run"; return 1; }
  if git merge-base --is-ancestor "rk-run/$1" HEAD; then echo "rk-run/$1 is merged into HEAD — it does not merge before Checkpoint 8"; return 1; fi
  base=$(git merge-base HEAD "rk-run/$1") || { echo "rk-run/$1 shares no history with HEAD"; return 1; }
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$ev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $ev"; return 1; }
  git merge-base --is-ancestor "$frozen" "$base" || { echo "rk-run/$1 forked from $base, which does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" "$base" -- $CODE_PATHS || { echo "code, registry, items or intake changed between the CI-green frozen sha and the run's fork — re-run CI and re-record (s8j §8j.7)"; return 1; }
  ahead=$(git rev-list --count "$base..rk-run/$1") || { echo "rev-list failed"; return 1; }
  [ "$ahead" -ge 1 ] || { echo "rk-run/$1 is $ahead commits ahead of its fork — a stale ref is not a run"; return 1; }
  lg=$(git log --reverse "$base..rk-run/$1" --format='%(trailers:key=Story,valueonly=true)') || { echo "git log over the run failed"; return 1; }
  bad=$(printf '%s\n' "$lg" | awk -v want="redkiln-rs/$1" 'NF && ($1!=want || NF!=2)')
  [ -z "$bad" ] || { echo "Story: values not of the form redkiln-rs/$1 <stage>: $bad"; return 1; }
  got=$(printf '%s\n' "$lg" | awk 'NF {printf "%s%s", sep, $NF; sep=" "}')
  [ "$got" = "$exp" ] || { echo "stages: expected '$exp', the run's trailers say '$got' — if explain shows an open gate, answer it through the launcher's answer mode (tools/live-run.sh answer; D21-46) and launch again with a new go-ahead (s8j §8j.3's hold is the first); a gated lane is not failed, and it is not a pass"; return 1; }
}
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM (rk-story-harden-spike-pack)"; exit 1; }
test -f ".redkiln/items/$STORY_ITEM.toml" || { echo "FAIL: .redkiln/items/$STORY_ITEM.toml does not exist"; exit 1; }
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
why=$(lane_check "$STORY_ITEM" .redkiln/packs/story.toml); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (1b): $why"; exit 1; }
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the Spec's path under the item's managed artifact directory (D21-42 5b)"; exit 1; }
git cat-file -e "rk-run/$STORY_ITEM:$AUTHORED_STORY" || { echo "FAIL (1b): $AUTHORED_STORY is not on rk-run/$STORY_ITEM"; exit 1; }
git cat-file blob "rk-run/$STORY_ITEM:$AUTHORED_STORY" > "$tdir/spec.blob" || { echo "FAIL: could not read the landed Spec blob"; exit 1; }
landed=$(sha256sum < "$tdir/spec.blob") || { echo "FAIL: sha256sum failed"; exit 1; }
landed=${landed%% *}
# artifact.write is item-scoped (s8e §8e.10): its item is the t2_event.item column, never a payload member (cross-file #15).
aw='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); r=[json.loads(p) for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=? AND item=? ORDER BY id",("artifact.write",sys.argv[2]))]; d=[x.get("digest") for x in r if x.get("kind")=="Spec"]; print(d[-1] if d else "")'
dg=$(python -c "$aw" "$store" "$STORY_ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the artifact.write probe did not run (rc=$rc)"; exit 1; }
test -n "$dg" || { echo "FAIL (1b): no artifact.write row with kind Spec for $STORY_ITEM — the authored story is not reachable from the event log (AC-521)"; exit 1; }
[ "$dg" = "$landed" ] || { echo "FAIL (1b): the last Spec artifact.write digest $dg is not the landed blob's $landed (ADR-0116 decisions 3 and 6)"; exit 1; }
printf '%s\n' "$out"   # → READ against AC-521: every story-lane stage (asserted above), the Spec authored by the run. The run id and the story id are the evidence.
```

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **Superseded: do not run the block below.** It is (1)'s block as it stood until s8j §8j.8 A1, kept for the record. It ran both lanes in one block under one go-ahead, ran `redkiln run` with no build, no launch record and no pre-spend check, asserted no stage set or order, and asserted tree membership with `git ls-files --error-unmatch`, which ADR-0116 decision 3 rejects.

```text
cd D:/repos/redkiln-rs
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (1) NOT RUN (Halt 8)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM to a real defect id first — a bare <placeholder> is a bash redirect, not an argument"; exit 1; }
./target/debug/redkiln run "$FIX_ITEM" --lane fix || { echo "FAIL: the fix-lane run exited non-zero"; exit 1; }   # ADR-0128 grammar (C-01 D-02, 2026-09-18): positional item, --lane optional. A run that STOPS AT A GATE exits 0 (C-01 D-01; AC-403) — this guard does not distinguish "landed" from "gated"; the trailer read below does.
out=$(./target/debug/redkiln explain "$FIX_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }   # POSITIONAL, the recorded resolution (AC-430–AC-432). Until 2026-09-18 this read `explain --run "$RUN_ID" --stages` — a run-keyed stage-listing mode no register criterion, ADR or design row names (C-01 D-03); the stage sequence is read from the event log's tier-1 trailer commits instead, next line.
test -n "$out" || { echo "FAIL: empty explain output — there is nothing to read against the PASS conditions below"; exit 1; }
log=$(git log "rk-run/$FIX_ITEM" --format=%B -E --grep='^(Story|Slice-Verdict|Gate):' 2>&1) || { echo "FAIL: git log over the trailer commits exited non-zero"; exit 1; }   # SCOPED TO THE RUN REF 2026-09-21 (Step 8, operator decision). This read `git log --all`, and `--all`
# made the guard below SATISFIABLE WITH ZERO RUNS PERFORMED: spike commit ba46ed9 ("H2/H3 RESULT:
# suspension is lossless...") carries a body line beginning `Slice-Verdict:`, so a run that wrote no
# tier-1 event at all still passed "the run wrote a tier-1 event" [measured 2026-09-21]. That is Halt 8's
# shape one level over, on the guard the whole self-hosting claim rests on. The run-owned ref satisfies
# `--all`'s OWN stated reason exactly -- the tier-1 commits live on rk-run/<item> (s7d H-5, ADR-0105
# clause 8; AC-518), never on the checkout's HEAD, which is why a bare `git log` would fail a correct
# binary (corrected 2026-09-18, re-spelt under RT-33) -- while admitting no commit the run did not make.
# A missing ref exits non-zero and fires the guard: no run ref means no run, which is fail-closed.
test -n "$log" || { echo "FAIL: no Story:/Slice-Verdict:/Gate: trailer commit on any ref (the run's trailers land on rk-run/<item> — s7d H-5) — the run wrote no tier-1 event (ADR-0105: git is the record)"; exit 1; }
printf '%s\n' "$out"; printf '%s\n' "$log"   # → READ both against the three lines below. NO stage-line pattern is pinned: the trailer values are the pack's stage names (7e's), so a grep for one would false-fail a correct binary.
# PASS only when the event log contains EVERY lane stage from intake to a landed commit,
# with NO stage performed by the old redkiln, and the verify stage's toolchain is rk-adapter-toolchain-rust.
# The claim is REFUSED if any stage is missing. The run id is the evidence; a statement is not.

# (1b) — AC-521, the story-lane twin
test -n "$STORY_ITEM" || { echo "FAIL: set STORY_ITEM to the Phase-2 story to author"; exit 1; }
./target/debug/redkiln run "$STORY_ITEM" --lane story || { echo "FAIL: the story-lane run exited non-zero"; exit 1; }   # ADR-0128 grammar; a gate-open exits 0 (C-01 D-01) — see (1)
out=$(./target/debug/redkiln explain "$STORY_ITEM") || { echo "FAIL: explain exited non-zero"; exit 1; }   # POSITIONAL; was `explain --run "$RUN_ID" --stages` until 2026-09-18 (C-01 D-03)
test -n "$out" || { echo "FAIL: empty explain output"; exit 1; }
log=$(git log "rk-run/$STORY_ITEM" --format=%B -E --grep='^(Story|Slice-Verdict|Gate):' 2>&1) || { echo "FAIL: git log over the trailer commits exited non-zero"; exit 1; }   # scoped to the run ref: see (1)
test -n "$log" || { echo "FAIL: no trailer commit on any ref (the run's trailers land on rk-run/<item> — s7d H-5) — the story-lane run wrote no tier-1 event"; exit 1; }
printf '%s\n' "$out"; printf '%s\n' "$log"   # → READ against the line below; no stage-line pattern is pinned, for the reason given in (1)
test -n "$AUTHORED_STORY" || { echo "FAIL: set AUTHORED_STORY to the path the run authored"; exit 1; }
git ls-files --error-unmatch "$AUTHORED_STORY" || { echo "FAIL: the authored story is not tracked — the second half of the line below"; exit 1; }
# PASS only when the event log contains every story-lane stage and the authored story is in the tree.
```

**(2) Replay — AC-184 / SM-124, and the kernel replay test.**

```bash
cd D:/repos/redkiln-rs
# The instruments are the ones the REGISTER names, not doctor's stdout (DECISIONS-2026-09-12 E5).
# AC-161 asks for the stage-level next action "equal, under the CANONICAL SERIALISER"; AC-184 names "the
# rebuild differential test, run in CI on every commit" over the tier-1 projection (ADR-0105 clause 9).
# Neither needs a doctor output contract, and pinning one would create a second source of truth that drifts
# from the serialiser — and would go red on a rebuild timestamp. doctor's stdout stays a human affordance.
out=$(cargo test -p rk-kernel -- rebuild_differential 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rebuild_differential exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched rebuild_differential — AC-184's tier-1 projection differential does not exist under that name"; exit 1; }
out=$(cargo test -p rk-kernel -- stage_level_next_action 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: stage_level_next_action exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched stage_level_next_action — AC-161's canonical-serialiser equality is unasserted"; exit 1; }
# *(dated 2026-10-18, D21-163, issue #90)* A8 and A2 below compare the tier-1 projection that AC-184's 2026-10-18 amendment
# names, with its four exclusions: `t2_event` rows and payloads; `t2_event.item`; an `opened` gate row's `stage`/`run_id`
# (D21-48 X11); a void's `observed_tree_hash` (D21-96). A2's last line drops the `t2_event` count by `${rebuilt% *}`.
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A8 inserted below, immediately above A2's anchor; no line rewritten.
# AMENDED at s8j (s8j §8j.8 A8; s8e §8e.9): the ROUTE of ADR-0105 clause 9's tier-1 projection — rows whose anchor is
# reachable from the refs, the store deleted, doctor --rebuild, canonical equality — which the kernel test cannot see.
ANCHOR_TESTS="doctor::ac184_rebuild_differential_over_anchored_tier_one_rows"
out=$(cargo test -p rk-cli --test it -- $ANCHOR_TESTS --exact 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the anchored rebuild differential did not pass (rc=$rc) — AC-184's route is unmeasured"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qxF "test $ANCHOR_TESTS ... ok" || { echo "FAIL: $ANCHOR_TESTS did not report ok by name"; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. 1 passed; 0 failed' || { echo "FAIL: the anchored differential selection did not run exactly one test"; exit 1; }
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A2: the three lines from the anchor through `doctor --rebuild` are replaced by the lines
# below, whose first line is the anchor, byte for byte. Was:
# ~~test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }~~
# ~~./target/debug/redkiln doctor --rebuild || { echo "FAIL: doctor --rebuild exited non-zero (AC-427)"; exit 1; }~~
# The operator path must still work (AC-427), but its STDOUT is not the comparison.
# AMENDED at s8j (s8j §8j.8 A2): built first; the store `doctor --rebuild` replaces is backed up and restored, so (7)
# reads the store the run wrote; and the REBUILT store's tier-1 counts equal the pre-rebuild ones (AC-184; ADR-0105
# clause 9), read before the restore. The line above is A8's anchor and stays byte-identical.
# *(dated 2026-10-18, D21-163, issue #90)* A2's counts compare the tier-1 tables only; the four exclusions are AC-184's amendment's list.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (2)'s operator half NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (2)'s operator half NOT RUN (Halt 8)"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
STORE_BAK="$store.s8-bak"
bk='import sqlite3,sys; s=sqlite3.connect(sys.argv[1]); d=sqlite3.connect(sys.argv[2]); s.backup(d); d.close(); s.close()'
rows='import sqlite3,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); print(" ".join(str(c.execute("SELECT COUNT(*) FROM "+t).fetchone()[0]) for t in ("t1_stage_transition","t1_gate_transition","t1_slice_seal","t2_event")))'   # t1_gate_transition: s8e's 0002 migration (D21-46 R2)
before=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$before" ] || { echo "FAIL: could not count the store's rows (rc=$rc)"; exit 1; }
python -c "$bk" "$store" "$STORE_BAK" || { echo "FAIL: the store could not be backed up — doctor --rebuild NOT RUN"; exit 1; }
./target/debug/redkiln doctor --rebuild; drc=$?
rebuilt=$(python -c "$rows" "$store"); rrc=$?
python -c "$bk" "$STORE_BAK" "$store" || { echo "FAIL: the store could NOT be restored from $STORE_BAK — restore it by hand before anything reads it"; exit 1; }
after=$(python -c "$rows" "$store"); rc=$?
[ "$rc" -eq 0 ] && [ "$after" = "$before" ] || { echo "FAIL: the restored store's row counts ($after) are not the pre-rebuild ones ($before)"; exit 1; }
rm -f "$STORE_BAK"
[ "$drc" -eq 0 ] || { echo "FAIL: doctor --rebuild exited $drc (AC-427)"; exit 1; }
[ "$rrc" -eq 0 ] && [ -n "$rebuilt" ] || { echo "FAIL: the rebuilt store's rows could not be counted (rc=$rrc)"; exit 1; }
[ "${rebuilt% *}" = "${before% *}" ] || { echo "FAIL (2): the rebuilt tier-1 counts ($rebuilt) are not the pre-rebuild ones ($before) — the projection does not rebuild from the refs (AC-184; ADR-0105 clause 9)"; exit 1; }
echo "(2) asserted: AC-184 differential + AC-161 next action, both under the canonical serialiser"
# This is the FIRST time the FSM is replayed at all — F19 proved suspension lossless 174/174 and explicitly
# left "same next action" unproven. Includes the adversarial-store variant and the 40-hex corpus-pin assertion.
```

**(3) Deterministic resume across a real kill, at MORE THAN ONE STEP.**

```bash
cd D:/repos/redkiln-rs
# Two halves (DECISIONS-2026-09-12 E5). The ASSERTED half is a kernel test over AC-013's N-step fixture graph;
# the OPERATIONAL half is the real-process arm ADR-0107 decision 4 pins, which is READ, not asserted. The digest
# field is NAMED since 2026-09-19 — ADR-0128 decision 4: each committed step's `output_digest` as `explain` prints it
# (E5's owed erratum closed by that record, DECISIONS-2026-09-18 O-13). AC-013 is PROVISIONAL on P8: F38 held exactly
# ONE step at n=3.
out=$(cargo test -p rk-kernel -- kill_and_resume_n_step 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: kill_and_resume_n_step exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' \
  || { echo "FAIL: 0 tests matched kill_and_resume_n_step — AC-013's multi-step digest equality is unasserted"; exit 1; }
echo "(3) asserted: AC-013 N-step kill-and-resume, digest equality, zero re-dispatch"
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A3: the operational arm that stood here, from the line opening
# `# --- the operational arm, in order.` through the block's last line, is deleted; the asserted half above stays.
# The arm is now (3)'s kill and resume blocks below, each its own real run and go-ahead (D21-47). It is kept, struck,
# under the `text` fence after them.
```

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **s8j §8j.8 A3 and A7 applied.** (3)'s operational arm is bound to the fix item (`ITEM="$FIX_ITEM"`) and runs on the lane's continuation, after §8j.3's hold is answered (s8j §8j.3), as two blocks, each opening with A1's `prespend`: **(3) kill** starts the run through the launcher, polls (bounded) until the run ref moves, takes the PRE-KILL capture and kills the launcher's tree; **(3) resume** re-invokes the same idempotent entry through the launcher (A7, folded into A3) and diffs the captures. A run that exits first, or commits nothing within the bound, is reported and not retried (D21-40 decision 2).

**(3) kill.**

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A3): (3)'s OPERATIONAL ARM, part 1 — ONE real run of the fix item, killed mid-graph. Its
# own block and go-ahead (D21-47): `S8_GO=$FIX_ITEM bash <this block>`, from a plain terminal, after (1)'s hold is
# answered. The PRE-KILL capture is taken HERE, after the run has committed a step and before the kill.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s operational arm NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm NOT RUN (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM — (3) is bound to the fix item"; exit 1; }
ITEM="$FIX_ITEM"
# prespend(), pasted from s8j §8j.8 A1 verbatim (D21-47):
# D21-47's pre-spend checks (s8j §8j.8 A1), carried verbatim by every s8 block that launches a real run. It prints the
# defect and returns 1, or prints nothing. The go-ahead is per run: S8_GO=<item> as THIS block's command prefix.
prespend() {   # $1 = the item this block is about to launch
  pev=runbook/phase-1/s8j-evidence.txt
  CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
  [ "${S8_GO:-}" = "$1" ] || { echo "no go-ahead for $1: run this block as S8_GO=$1 bash <block>, one launch per go-ahead (D21-47)"; return 1; }
  test -s "$pev" || { echo "$pev is absent or empty"; return 1; }
  if od -c "$pev" | grep -qF '\r'; then echo "$pev carries a CR byte"; return 1; fi
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$pev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $pev"; return 1; }
  ci=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  read -r _ ci_run ci_sha _ <<< "$ci"
  [ -n "$ci_sha" ] && [ "$frozen" = "$ci_sha" ] || { echo "$1's frozen sha $frozen is not the CI-green sha '$ci_sha'"; return 1; }
  git merge-base --is-ancestor "$frozen" HEAD || { echo "HEAD does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" HEAD -- $CODE_PATHS || { echo "code, registry, items or intake changed since the frozen sha: re-run CI and re-record (s8j §8j.7)"; return 1; }
  fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  fp_sha=$(printf '%s\n' "$fp" | awk '{print $2}')
  [ -n "$fp_sha" ] && git diff --quiet "$fp_sha" "$frozen" -- $CODE_PATHS || { echo "no fixture-proofs line over the frozen code: the spend gate (D21-40 decision 2; s8j §8j.6 step 1b)"; return 1; }
  rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$pev" | tail -1)
  rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
  [ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; return 1; }
  if grep -qE '^[[:space:]]*harness_binary[[:space:]]*=' .redkiln/config.toml; then echo ".redkiln/config.toml names a harness_binary: a real run must reach the real CLI, never a fixture child"; return 1; fi
  if git merge-base --is-ancestor "rk-run/$1" HEAD 2>/dev/null; then echo "rk-run/$1 is already merged into HEAD: the run ref does not merge before Checkpoint 8"; return 1; fi
  j=$(gh run view "$ci_run" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha 2>&1) || { echo "gh run view $ci_run failed: $j"; return 1; }
  ok=$(printf '%s\n' "$j" | python -c 'import json,sys; d=json.load(sys.stdin); print("ok" if d.get("conclusion")=="success" and d.get("headSha")==sys.argv[1] else "no")' "$frozen")
  [ "$ok" = ok ] || { echo "CI run $ci_run is not green on $frozen"; return 1; }
}
why=$(prespend "$ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (3) NOT LAUNCHED: $why"; exit 1; }
git rev-parse --verify --quiet "refs/heads/rk-run/$ITEM" >/dev/null || { echo "FAIL: no rk-run/$ITEM — run (1)'s launch block and answer the hold first; (3) kills the continuation"; exit 1; }
lg=$(git log -1 --format='%(trailers:key=Story,valueonly=true)' "rk-run/$ITEM") || { echo "FAIL: git log over rk-run/$ITEM failed"; exit 1; }
last=$(printf '%s\n' "$lg" | awk 'NF {t=$NF} END {print t}')
[ -n "$last" ] && [ "$last" != closed ] || { echo "FAIL: rk-run/$ITEM's last stage token is '$last' — a kill over a closed or unread lane is a no-op pass"; exit 1; }
pre_tip=$(git rev-parse "rk-run/$ITEM") || { echo "FAIL: cannot read rk-run/$ITEM"; exit 1; }
tdir=target/s8-cp; mkdir -p "$tdir" || { echo "FAIL: cannot create $tdir"; exit 1; }
rm -f "$tdir/s8-3-pre.txt"
tools/live-run.sh run "$ITEM" --lane fix > "$tdir/s8-3-killed.out" 2>&1 &
bg=$!
# A BOUNDED POLL: until the run ref moves past pre_tip (the process committed a step), the process exits, or 900 s pass.
n=0
while [ "$(git rev-parse "rk-run/$ITEM")" = "$pre_tip" ] && kill -0 "$bg" 2>/dev/null && [ "$n" -lt 900 ]; do sleep 1; n=$((n + 1)); done
kill -0 "$bg" 2>/dev/null || { echo "FAIL: the run exited after ${n}s, before it could be killed — no kill mid-graph happened; read $tdir/s8-3-killed.out, stop and report (D21-40 decision 2)"; exit 1; }
wp=$(cat "/proc/$bg/winpid") || { echo "FAIL: no Windows PID for the launcher (pid $bg) — stop it by hand"; exit 1; }
[ "$(git rev-parse "rk-run/$ITEM")" != "$pre_tip" ] || { taskkill //T //F //PID "$wp" >/dev/null 2>&1; echo "FAIL: no step committed within ${n}s (the bound fired) — the run was killed before any step; stop and report"; exit 1; }
pre=$(./target/debug/redkiln explain "$ITEM") || { taskkill //T //F //PID "$wp" >/dev/null 2>&1; echo "FAIL: pre-kill explain exited non-zero (the run was killed)"; exit 1; }
taskkill //T //F //PID "$wp" || { echo "FAIL: taskkill on the launcher's tree ($wp) failed — the run may still be live; stop it by hand"; exit 1; }
wait "$bg"
test -n "$pre" || { echo "FAIL: empty pre-kill capture — two empty outputs compare equal"; exit 1; }
printf '%s\n' "$pre" > "$tdir/s8-3-pre.txt" || { echo "FAIL: cannot write the pre-kill capture"; exit 1; }
echo "(3) killed mid-graph after ${n}s at $(git rev-parse --short "rk-run/$ITEM"); run (3)'s resume block with its own go-ahead"
```

**(3) resume.**

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# AMENDED at s8j (s8j §8j.8 A3, A7): (3)'s OPERATIONAL ARM, part 2 — re-invoke the SAME idempotent entry through the
# launcher, its own go-ahead: `S8_GO=$FIX_ITEM bash <this block>`. There is no --resume flag and no resume verb
# (ADR-0107; AC-013/AC-186/AC-264/AC-400). That --resume is refused is itself AC-400's check, at s7d's checkpoint.
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — (3)'s resume NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s resume NOT RUN (Halt 8)"; exit 1; }
test -f tools/live-run.sh || { echo "FAIL: tools/live-run.sh is absent (D21-42 17a)"; exit 1; }
test -n "$FIX_ITEM" || { echo "FAIL: set FIX_ITEM — (3) is bound to the fix item"; exit 1; }
ITEM="$FIX_ITEM"
tdir=target/s8-cp
test -s "$tdir/s8-3-pre.txt" || { echo "FAIL: no pre-kill capture at $tdir/s8-3-pre.txt — run (3)'s kill block first"; exit 1; }
# prespend(), pasted from s8j §8j.8 A1 verbatim (D21-47):
# D21-47's pre-spend checks (s8j §8j.8 A1), carried verbatim by every s8 block that launches a real run. It prints the
# defect and returns 1, or prints nothing. The go-ahead is per run: S8_GO=<item> as THIS block's command prefix.
prespend() {   # $1 = the item this block is about to launch
  pev=runbook/phase-1/s8j-evidence.txt
  CODE_PATHS="crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln"
  [ "${S8_GO:-}" = "$1" ] || { echo "no go-ahead for $1: run this block as S8_GO=$1 bash <block>, one launch per go-ahead (D21-47)"; return 1; }
  test -s "$pev" || { echo "$pev is absent or empty"; return 1; }
  if od -c "$pev" | grep -qF '\r'; then echo "$pev carries a CR byte"; return 1; fi
  frozen=$(awk -v it="$1" '($1=="fix-item" || $1=="story-item") && $2==it {f=$3} END {print f}' "$pev")
  printf '%s\n' "$frozen" | grep -qxE '[0-9a-f]{40}' || { echo "no frozen sha recorded for $1 in $pev"; return 1; }
  ci=$(grep -E '^ci [0-9]+ [0-9a-f]{40} [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  read -r _ ci_run ci_sha _ <<< "$ci"
  [ -n "$ci_sha" ] && [ "$frozen" = "$ci_sha" ] || { echo "$1's frozen sha $frozen is not the CI-green sha '$ci_sha'"; return 1; }
  git merge-base --is-ancestor "$frozen" HEAD || { echo "HEAD does not descend from the frozen sha $frozen"; return 1; }
  git diff --quiet "$frozen" HEAD -- $CODE_PATHS || { echo "code, registry, items or intake changed since the frozen sha: re-run CI and re-record (s8j §8j.7)"; return 1; }
  fp=$(grep -E '^fixture-proofs [0-9a-f]{40} pass [0-9]{4}-[0-9]{2}-[0-9]{2}$' "$pev" | tail -1)
  fp_sha=$(printf '%s\n' "$fp" | awk '{print $2}')
  [ -n "$fp_sha" ] && git diff --quiet "$fp_sha" "$frozen" -- $CODE_PATHS || { echo "no fixture-proofs line over the frozen code: the spend gate (D21-40 decision 2; s8j §8j.6 step 1b)"; return 1; }
  rt=$(sed -nE 's/^record-rt57 ([0-9]{4})$/\1/p' "$pev" | tail -1)
  rtf=$(ls references/adr/"$rt"-*.md 2>/dev/null)
  [ -n "$rt" ] && [ -f "$rtf" ] && grep -qxF 'status: accepted' "$rtf" || { echo "RT-57's record (ADR-${rt:-none}, s8h's Record B) is not accepted (D21-42 19c)"; return 1; }
  if grep -qE '^[[:space:]]*harness_binary[[:space:]]*=' .redkiln/config.toml; then echo ".redkiln/config.toml names a harness_binary: a real run must reach the real CLI, never a fixture child"; return 1; fi
  if git merge-base --is-ancestor "rk-run/$1" HEAD 2>/dev/null; then echo "rk-run/$1 is already merged into HEAD: the run ref does not merge before Checkpoint 8"; return 1; fi
  j=$(gh run view "$ci_run" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha 2>&1) || { echo "gh run view $ci_run failed: $j"; return 1; }
  ok=$(printf '%s\n' "$j" | python -c 'import json,sys; d=json.load(sys.stdin); print("ok" if d.get("conclusion")=="success" and d.get("headSha")==sys.argv[1] else "no")' "$frozen")
  [ "$ok" = ok ] || { echo "CI run $ci_run is not green on $frozen"; return 1; }
}
why=$(prespend "$ITEM"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL (3) NOT RE-INVOKED: $why"; exit 1; }
tools/live-run.sh run "$ITEM" --lane fix || { echo "FAIL: the re-invocation exited non-zero"; exit 1; }   # AMENDED at s8j (s8j §8j.8 A7): through s8h's launcher, the only entry to a real run (D21-42 17a). A re-invocation that reaches a gate exits 0 (C-01 D-01), so this guard passes on a gated run too; the digest diff below is the assertion.
post=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: post-resume explain exited non-zero"; exit 1; }
test -n "$post" || { echo "FAIL: empty post-resume capture"; exit 1; }
diff "$tdir/s8-3-pre.txt" <(printf '%s\n' "$post")   # → READ this. Whole-output equality is NOT asserted: the post-resume output carries the resume's own appended events. Each committed step's output_digest (ADR-0128 decision 4) must be unchanged; AC-265 (the gate answer surviving the kill) is exercised here.
```

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **Superseded: do not run the lines below.** They are (3)'s operational arm as it stood until s8j §8j.8 A3, kept for the record, their fence marked `text`. They read `ITEM` from whatever the shell held, left the kill to a comment, and ran `redkiln run` with no launch record.

```text
# --- the operational arm, in order. The PRE-KILL capture is taken HERE; until 2026-09-12 this block
# --- diffed against a capture it never took, and never performed the kill (it lived in a comment).
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — (3)'s operational arm NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item the run is on"; exit 1; }
pre=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: pre-kill explain exited non-zero"; exit 1; }   # POSITIONAL (ADR-0107 decision 4; design:1311) — not --item
test -n "$pre" || { echo "FAIL: empty pre-kill capture — two empty outputs compare equal"; exit 1; }
# Now kill the running process mid-graph, from another shell:   taskkill /T /F /PID <pid>
# Then re-invoke the SAME idempotent entry — there is no --resume flag and no resume verb (ADR-0107;
# AC-013/AC-186/AC-264/AC-400). That --resume is refused is itself AC-400's check, at s7d's checkpoint.
./target/debug/redkiln run "$ITEM" --lane fix || { echo "FAIL: the re-invocation exited non-zero"; exit 1; }   # ADR-0128 grammar (C-01 D-02): was `run --lane fix --item "$ITEM"`. A re-invocation that reaches a gate exits 0 (C-01 D-01), so this guard passes on a gated run too; the digest diff below is the assertion.
post=$(./target/debug/redkiln explain "$ITEM") || { echo "FAIL: post-resume explain exited non-zero"; exit 1; }
test -n "$post" || { echo "FAIL: empty post-resume capture"; exit 1; }
diff <(printf '%s\n' "$pre") <(printf '%s\n' "$post")   # → READ this. Whole-output equality is NOT asserted: the
# post-resume output necessarily carries the resume's own appended events, so a bare `=` test would false-fail
# on a correct binary. The kernel test above is the generalising instrument; this arm is the real-process check.
# AC-265 (the gate answer surviving the kill) is exercised here, not in 7b-ii.
```

**(4) The determinism ban list fails the build.** `cargo clippy --workspace --all-targets` gated on its exit code
(never `-D warnings` on the command line — that form misparses beside the manifest's `[lints]` table and promotes
nursery to a group deny ADR-0083 forbids; PIA-030) plus the `clippy.toml`-driven fixture crate that reintroduces a
clock read (AC-001–AC-004; PIA-031 — a `trybuild` fixture cannot fail on a lint). The fixture **failing the lint** is
the pass.

```bash
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
test -f Cargo.toml && test -f clippy.toml || { echo "FAIL: no workspace-root Cargo.toml / clippy.toml — AC-001–AC-004 have no instrument; criterion (4) NOT RUN (Halt 8)"; exit 1; }
cargo clippy --workspace --all-targets || { echo "FAIL: the clippy gate is red (never -D warnings on the CLI — ADR-0083; PIA-030)"; exit 1; }
CLOCK_FIXTURE=rk-clock-read   # s5 §5.1a names it; a feature-gated workspace member (DECISIONS-2026-09-12 E4)
cargo metadata --format-version 1 --no-deps | grep -q "\"$CLOCK_FIXTURE\"" || { echo "FAIL: $CLOCK_FIXTURE is not a workspace member — a missing fixture must never read as the pass"; exit 1; }
# REPAIRED 2026-09-21 (Step 8, operator decision). This ran `cargo clippy -p "$CLOCK_FIXTURE"` with
# NO FEATURE, and under that invocation the fixture CANNOT FAIL: its offending call sits behind the
# non-default `reintroduce` feature, which is the whole point of the feature-gated-member shape
# (DECISIONS-2026-09-12 E4) -- with the feature off the default workspace gate stays green. So the
# line clipped clean and fired its own "the fixture PASSED the lint" FAIL on a CORRECT tree
# [measured 2026-09-21]. The fixture's own `tests/fixtures/rk-clock-read/Cargo.toml` spells the pair
# verbatim, and this is now the second of them:
#   green:  cargo clippy --workspace --all-targets
#   RED:    cargo clippy --workspace --all-targets --features rk-clock-read/reintroduce
# The workspace form is the one AC-001's Verified-by asks for -- *"checked under the same `cargo
# clippy` invocation"* -- and `$CLOCK_FIXTURE` stays load-bearing as the feature's package.
# AND THE REASON IS ASSERTED, NOT ONLY THE REDNESS (golden constraint 12): a fixture that failed to
# compile for any other cause would satisfy a bare non-zero check while measuring nothing.
out=$(cargo clippy --workspace --all-targets --features "$CLOCK_FIXTURE/reintroduce" 2>&1); rc=$?
[ "$rc" -ne 0 ] || { echo "FAIL: the clock-read fixture PASSED the lint — the fixture FAILING it is the pass (PIA-031)"; exit 1; }
printf '%s\n' "$out" | grep -q 'use of a disallowed method' || { echo "FAIL: the fixture is red for some OTHER reason than the determinism ban — a red for the wrong reason is not a pass (golden constraint 12)"; printf '%s\n' "$out" | tail -20; exit 1; }
```

**(5) WITHDRAWN 2026-09-15.** It read *"The `--stay`-class error is unrepresentable — a `trybuild` must-fail
fixture, not a runtime assertion"*, and its obligation is **folded into AC-294** (`DECISIONS-2026-09-14 D-14`):
AC-294 already owns a private constructor, a module-boundary architecture lint and a `trybuild` case — the
identical instrument family in the same crate — while check (5)'s only warrant was ADR-0001:42, which the
register classes as *"a premise, not a checkable behaviour"*. **The number is retired; nothing renumbers.**

**Its shell block is cut, not annotated, and what was wrong with it is recorded here rather than left standing.**
The block ran `cargo test -p rk-kernel -- trybuild` and asserted `test result: ok. [1-9][0-9]* passed`. **That
filter is green whenever ANY ONE case passes**: 13 case files live in `crates/rk-kernel/tests/ui/` and the
`trybuild` filter does select the harness [measured 2026-09-15], so an **absent** `--stay` case read exactly like a
present one. **This is not a Halt 8 shape and must not be described as one** — it captured `rc` before grepping,
greped the capture and not a pipe, and every guard ended in `exit 1`, so it is none of Halt 8's sub-shapes
(a)–(h); and Halt 8's headline (*"a path it names does not exist"*) cannot engage, because the block **named no
fixture path at all**. The defect is the any-case-passes filter, and its repair is the **by-path** assertion now
carried in Checkpoint 8 below — not a re-run of the harness. AC-294's own discharge stays **Checkpoint 7b-ii's**
(`s7b-ii` :697-698, its block-constructor case); Step 8 asserts only that the `--stay` case file and its committed
`.stderr` are still there.

**(6) `redkiln lint --facet-stability` (AC-071) passes**, which means one of exactly two things and nothing else:

```bash
# [WILL EXIST] — and so does its argument: there is NO .redkiln/ directory in redkiln-rs today
# (verified 2026-09-11). The facet registry that exists now is spikes/p9/facets.toml, which STEP 7e's
# pack authoring derives .redkiln/facets.toml from (PIA-011). Both the binary and the file are Phase-1 output.
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A6: the three lines below are inserted above the lint line; no line rewritten.
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A6): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (6) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (6) NOT RUN (Halt 8)"; exit 1; }
./target/debug/redkiln lint --facet-stability .redkiln/facets.toml
# The passing branch after Step 4a is the ZERO-llm branch, which needs no stability record -- this is the
# form s7e Checkpoint 7e uses FOR THIS LINT, and it is why the argument is dropped here. **Scoped
# 2026-09-21:** this said "the form s7e Checkpoint 7e uses" unqualified, which is true of
# `--facet-stability` (7e drops `--stability-record` too) and FALSE of `--facet-provenance` (7e passes
# `--pinned-corpus`, and so does this file's own line below, as amended the same day). A dropped argument
# is the right form for one lint and the exit-3 defect for the other. A bare <path> is a bash
# redirect, not an argument: the line above never ran until 2026-09-12. If a facet is ever
# source = "llm", the record-bearing form is
#   CORPUS_REPO=$(grep '^CORPUS_REPO=' CORPUS_PIN | cut -d= -f2-)
#   STABILITY_RECORD="$CORPUS_REPO/results/p1-arm-l-stability.json"   # the path, filled 2026-09-15; the
#                                    # placeholder that stood here (`STABILITY_RECORD=  # set to a real path
#                                    # first`) was the landing record's to fill and is now filled. Read
#                                    # CORPUS-RELATIVE THROUGH CORPUS_PIN, never through a working tree (AC-464).
#   test -n "$STABILITY_RECORD" || { echo "FAIL: no stability record path"; exit 1; }
#   test -s "$STABILITY_RECORD" || { echo "FAIL: no stability record at $STABILITY_RECORD — an absent record must never read as the pass"; exit 1; }
#   ./target/debug/redkiln lint --facet-stability .redkiln/facets.toml --stability-record "$STABILITY_RECORD"
# [WILL EXIST] -- re-run at Checkpoint 7e, which runs both lints against the product registry. CITED BY
# QUOTATION, NOT BY LINE (corrected 2026-09-21): this read "where s7e:120-121 runs both lints" and s7e has
# since grown to 234 lines, where :120-121 resolves to the `stale_policy` warrant prose instead. The two
# lines meant are the ones opening `./target/debug/redkiln lint --facet-stability  .redkiln/facets.toml`
# and `./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml --pinned-corpus`, checkable
# with `grep -cF` at any later moment.
# exit 0 only when EITHER zero facets are declared source = "llm",
# OR a stability record satisfies ALL FIVE clauses:
#   modal_required_set_identity >= 0.80  AND  n_items >= 20  AND  draws_per_item >= 10
#   AND panel_verdict == "scored"        AND  corpus_sha == <the pin the MEASUREMENT was taken at>
#
# READ THE SECOND BRANCH AS FAILING. It is stated plainly here because the record's PATH is now named --
# by AC-071's dated register amendment, 2026-09-15 -- and the block must not imply the gate could pass on it.
# THE RECORD FILE NOW EXISTS. *(Corrected 2026-09-15, later the same day: this read "THE RECORD FILE
# ITSELF DOES NOT EXIST", measured against pin 27d97e9b before s7b §7b.11's move. The operator minted it
# and moved the pin 27d97e9b -> 61f590e5, so s7b §7b.11 precondition 2 is SATISFIED and Checkpoint 7b has
# PASSED. NONE OF THAT CHANGES THE BRANCH ABOVE, which still reads as FAILING -- and that is the point of
# saying so here: the record's existence was never what the gate turned on.) It is a TRANSCRIPTION of F36
# that s7b §7b.2 forbids regenerating; its scalar is the MINIMUM of F36's four
# identity figures, 0.10, so
# CLAUSE 1 FAILS at 0.10 against the 0.80 line [measured, F36: 0.25 / 0.15 / 0.10 / 0.10].
# CLAUSE 5 DOES NOT CLEAR EITHER, and that is fail-closed and correct. Under DECISIONS-2026-09-15 D15-1,
# `corpus_sha` in a TRANSCRIBED record denotes THE PIN THE MEASUREMENT WAS TAKEN AT --
# 47faec9f46b8d983c2c75f0794164e68415ae706, the value F36's own engine rows carry and the value
# crates/rk-kernel/tests/it/facet_stability.rs:37 already hard-codes as CATALOG_CORPUS_PIN -- and NOT the
# catalog pin at read time, which CORPUS_PIN moves. So clause 5 is unsatisfied whenever CORPUS_PIN is not
# that sha. A record edited to make clause 5 pass while clause 1 failed would be the "a lint that reads
# identity and skips the panel clause" failure one level over, and D15-1 refuses it by name.
# panel_verdict == "scored" IS SATISFIABLE AND, FOR ARM D, SATISFIED — F39 scored it on 2026-09-09
# over 140 truth cells at 47faec9f (all scored after the 2026-09-11 panel-file key fix; the 2026-09-09 run scored 136 — F39-DRAFT's Correction).
# A discharged clause is not a partial unblocking: the gate still fails on the second branch — on clause 1
# and on clause 5 — and the fixture set stays at FIVE. After Step 4a the FIRST branch is what passes — zero
# llm facets (ADR-0090; DECISIONS-2026-09-11 #2) — which is the expected outcome, and it is the branch this
# check takes: nothing in Phase 1 reaches the record-bearing form.
```

**(6, second lint) `redkiln lint --facet-provenance` (AC-520) passes** — the derived-facet sibling, new since F39,
which gates the half of the registry AC-071 never reached. **This is not an added exit criterion**: design §10.2
names seven, this runbook **withdraws (5)** (2026-09-15) and adds none. It is stated here because AC-520 is the criterion that makes
(6)'s neighbouring half checkable, and a Phase 1 that exits on (6) with `--facet-provenance` unbuilt has left the
measurably-wrong half of the registry ungated — which is the state F39 found:

```bash
# [WILL EXIST] — Step 4a authors the registry half, Step 7b builds the lint, Step 7e derives the file.
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A6: the three lines below are inserted above the lint line; no line rewritten.
out=$(cargo build --workspace 2>&1); rc=$?   # AMENDED at s8j (s8j §8j.8 A6): golden constraint 3
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (6) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (6) NOT RUN (Halt 8)"; exit 1; }
./target/debug/redkiln lint --facet-provenance .redkiln/facets.toml --pinned-corpus .redkiln/corpus-records.toml
# AMENDED 2026-09-21 (Step 8, operator decision). This line ran WITHOUT `--pinned-corpus` and exited 3:
# `crates/rk-cli/src/verbs/lint.rs` refuses to default that path in terms -- *"its default ... is specified
# by no record, and a path invented here would be read as one that was decided."* Checkpoint 7e line 141
# carried the IDENTICAL defect and was fixed the same way: author `.redkiln/corpus-records.toml` and pass
# it. The argument is the file 7e authored (six `[[corpus_record]]` rows transcribed from arm D's panel).
# The line stays a bare invocation, as its `--facet-stability` twin above is: this block is a READ of an
# exit code, and adding a `|| { ...; exit 1; }` guard here would change what the criterion asserts.
# exit 0 only when EVERY facet declared source = "derived" names a `deriver` AND a scored precision
# record exists for it over a pinned corpus, carrying a Wilson 95% interval and a positive-instance
# count. A facet with NO positive instance is reported NOT MEASURABLE — neither passed nor failed —
# and may key no require row. A lexical deriver never satisfies PC8 rule 2 for require-row purposes.
# NO THRESHOLD is re-stated here on purpose: METHOD-P §1.4's 0.95 bar is not reachable at n=20 for
# any facet however good [measured, F39 Limits 1]. The gate requires a record that EXISTS, IS PINNED
# AND IS READ.
#
# WHICH BRANCH PASSES, BY NAME (PIA-001; ADR-0093 decision 2): six facets are `derived` and every one has a
# record at 47faec9f — toolchain 0.73 [0.48, 0.89], change 0.72 [0.54, 0.85], ui 1.00 [0.65, 1.00],
# flag_exists 0.00 [0.00, 0.66] (F39); data_migration NOT MEASURABLE (0 positives); recent_deploy NOT
# MEASURABLE (feed absent) — so the lint exits 0 with two NOT MEASURABLE lines and those two facets key no
# require row. stage, rigor, role and cost are `kernel`; surfaces is `declared`; none of them is gated here.
```

On today's evidence (6)'s **first** branch is what passes, and (6, second lint) passes on six named records. That is
the expected outcome, not a shortfall.

**(7) Every dispatch event carries a cost row (AC-394), and headroom is reported only as a batch delta with `n` and
`resolution_limited`** (AC-372/AC-373/AC-395; **SM-111** — the previous revision cited SM-109, which is attribution
completeness, PIA-061). `utilization` is quantised to 0.01, so per-run headroom is **not measurable**; the delta is
computed in integer ticks over readings sharing one `resetsAt`, over the account's complete dispatch sequence
(`redkiln cost <item>` prints `not attributable` with the interleaving count otherwise), and a set with `n < 10` is
`low_sample` with no delta quoted (ADR-0110). `redkiln cost` renders the flag; a run that reports a per-run figure
without it has laundered an unmeasurable into a measurement.

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **s8j §8j.8 A4 applied: (7)'s whole block is replaced.** Built first, and AC-394 is asserted as owned readings == `dispatch_completed` for the item, with no rendering over n = 0 (D21-42 12a). The superseded block follows, its fence marked `text`.

```bash
# AMENDED at s8j (s8j §8j.8 A4): built first; AC-394 asserted as owned readings == dispatch_completed (D21-42 12a).
out=$(cargo build --workspace 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo build --workspace rc=$rc — criterion (7) NOT RUN (Halt 1)"; printf '%s\n' "$out" | tail -20; exit 1; }
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
store="$(git rev-parse --git-common-dir)/redkiln/store.sqlite3"
test -f "$store" || { echo "FAIL: no store at $store"; exit 1; }
own='import sqlite3,json,sys; c=sqlite3.connect("file:"+sys.argv[1]+"?mode=ro",uri=True); rd=sum(1 for (p,) in c.execute("SELECT payload FROM t2_event WHERE kind=?",("rate_limit_reading",)) if json.loads(p).get("item")==sys.argv[2]); dc=c.execute("SELECT COUNT(*) FROM t2_event WHERE kind=? AND item=?",("dispatch_completed",sys.argv[2])).fetchone()[0]; print(rd, dc)'   # readings: payload item (ADR-0163); dispatches: the interim key's column (s8e; D21-46 R3)
cnt=$(python -c "$own" "$store" "$ITEM"); rc=$?
[ "$rc" -eq 0 ] && [ -n "$cnt" ] || { echo "FAIL: the ownership probe did not run (rc=$rc)"; exit 1; }
read -r owned dc <<< "$cnt"
[ "$dc" -ge 1 ] || { echo "FAIL (7): no dispatch_completed row names $ITEM — a reading over n = 0 is not a reading (D21-42 12a)"; exit 1; }
[ "$owned" = "$dc" ] || { echo "FAIL (7): $ITEM owns $owned readings and $dc dispatch_completed rows — AC-394 is one reading per dispatch (ADR-0163 decision 2)"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
if printf '%s\n' "$out" | grep -qE ' nothing to attribute |(^|[^0-9])n=0([^0-9]|$)'; then echo "FAIL (7): a rendering over n = 0 for an item that owns $dc dispatches (D21-42 12a)"; printf '%s\n' "$out"; exit 1; fi
printf '%s\n' "$out"   # → READ: no per-run headroom figure (AC-373).
```

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* **Superseded: do not run the block below.** (7)'s block as it stood until s8j §8j.8 A4: no build, and an AC-394 READ line nothing asserted.

```text
test -x ./target/debug/redkiln || { echo "FAIL: ./target/debug/redkiln absent — criterion (7) NOT RUN (Halt 8)"; exit 1; }
test -n "$ITEM" || { echo "FAIL: set ITEM to the item whose cost rows are being read"; exit 1; }
out=$(./target/debug/redkiln cost "$ITEM") || { echo "FAIL: redkiln cost exited non-zero"; exit 1; }
test -n "$out" || { echo "FAIL: redkiln cost printed nothing"; exit 1; }
printf '%s\n' "$out" | grep -qE 'resolution_limited|low_sample|not attributable' || { echo "FAIL: a headroom report carrying none of ADR-0110's three renderings — an unmeasurable laundered into a measurement"; exit 1; }
printf '%s\n' "$out"   # → READ: every dispatch carries a cost row (AC-394) and no per-run headroom figure appears (AC-373). `n` is deliberately NOT greped for — ADR-0110 requires the count, never a rendering of it.
```

**Checkpoint 8.** All six pass (with (1)'s two halves and (6)'s two lints), and the following remain true and
unviolated across the whole phase:

- No `require` row keyed on an LLM-emitted `criticality`, `novelty`, `size`, `blast_radius`, `market_facing`,
  `external_dep` or `has_repro` — and, after Step 4a, no facet is `source = "llm"` at all.
- **No `stale_policy` value was defaulted from P2**: each cost class's setting in every Phase-1 pack is
  `rematerialise`, coarse scope, with F40 §3's contraindication quoted as its warrant (decision 5; Step 7e).
- **No `require` row keyed on a `declared` facet** (`surfaces`) or on a NOT MEASURABLE one (`data_migration`,
  `recent_deploy`) — ADR-0093.
- **`bisect` carries `max_rigor = "recommended"`** and the rigor-monotonicity lint passes on the shipped catalog
  (P7-CEIL, DECISIONS-2026-09-11 #1).
- No `Binary` arm constructed outside `rk-ports`, no `ChangeSummary` definition outside it. *(Amended 2026-10-14, s8i §8i.8, ADR-0199 decision 2, proposed *(dated 2026-10-15, `s8i` close, D21-154: F43 and ADR-0199 accepted)*; package S8I-C4-PORTS: S-BIN has reported (F43, proposed), and the record's construction rule replaces the first clause. The `Binary` payload is built outside `rk-ports` only through its recorded checked constructor, `BinaryChangeSummary::new`, and only in test code (`crates/*/tests`, `benches`, `examples`) while the record's `Binary producers in Phase 1` line reads `none`. No struct literal, tuple construction or other associated function builds it anywhere outside `rk-ports`. The guard below is amended to match.)* *(Amended 2026-10-15, s8i §8i.8, package S8I-F1-GUARDS, operator ruling R10 on s8i refuting round 21's blocker B1, proposed: test code includes an in-source `#[cfg(test)] mod`. The recorded constructor is admitted inside one, as it is under `crates/*/tests`. The guard finds the module by counting braces, and a production call in `src` still fires. s8i's fix round E put such a fake in `rk-cli`'s `kept_refusal.rs`, and the guard below gains the lines and controls for it.)* *(Amended 2026-09-24,
  s8a §8a.3, ADR-0165. This bullet read "No finalised `ChangeSummary` type; no synthesised `ChangeSummary` value. **The
  S-BIN fixture existing at `D:/repos/BrittonGames` does not change this** — the spike has not run." ADR-0165 finalises
  the `Text` arm now and seals the `Binary` slot until S-BIN reports at `s8i`, so the Rust adapter returns a real `Text`
  value. What stays forbidden is a `ChangeSummary` definition, alias, renaming import or impl outside `rk-ports`, and a
  `Binary` payload built outside it. The BrittonGames sentence still holds for `Binary`: the spike has not run.)*
- No card append route enabled as anything but an unmeasured config option.
- No quality-shorthand default; no `findings.none(evidence in [Reproduced, MutationSurvived])` as a terminal
  condition.
- No run reported as 1 h TTL without per-run `ephemeral_*` field evidence.
- gix does not own the write path.
- **No document authored or amended from here on ASSERTS that a facet "leaves the model".** None does: no facet
  clears METHOD-P §1.4's 0.95 bar and the bar is **not reachable at n=20 for any facet however good**
  [measured, F39]. **Scope, because the unscoped form of this rule is unsatisfiable:** the phrase lives, correctly
  and permanently, in `spikes/METHOD-P.md` §1.4 (the registered rule itself), in `references/adr/0079-*.md`, in
  `spikes/F36-DRAFT.md` and in `spikes/DECISIONS-2026-09-09.md`. Those are frozen sources and quoting a registered
  rule is not asserting it — editing them to satisfy this line is the one thing the rule must never cause. The
  mechanical check is scoped to `crates/` for exactly that reason (Checkpoint 8 above).
- **No facet declares `source = "derived"` without a named `deriver` and a scored precision record** (AC-520,
  ADR-0089), and **no `require` row is keyed on a facet whose scored record shows no positive instance** — that
  facet is reported *not measurable*, neither passed nor failed.
- **No derived boolean resolves an absence of evidence to `false`.** It is `null`, and `null` routes to AC-079's
  `gate_opened { reason: unknown_facet }`. **No `null` is counted as a false negative** in any scoring of a facet
  deriver.
- **No document reports F39's oracle as human-panel.** It is **panel-adjudicated**: 60 deterministic-oracle cells,
  48 unanimous panel, 32 human adjudication of 140 [measured, F39 §5].
- **No document quotes "four questions on twenty items" as a triage-gate load forecast.** It is a measurement of an
  oracle-construction exercise; SM-024 has no baseline and S-FACET is what gives it one.
- **No section decides source asymmetry.** `commits` beats `docs` and `union` is worse than `commits` alone on
  `toolchain` and `change` [measured, F39 §3]; the decision is an arm of S-FACET and ADR-0089 does not take it.
- **No document, section or commit message describes this design as final.** S-BIN, S-H1c and P8's unrun arms gate [S-BIN reported (s8i) 2026-10-14: S-H1c and P8's unrun arms still gate the label]
  that label, and P8 has no result on its own question.
- **The `--stay`-class state stays unconstructible, and its `trybuild` case is asserted BY PATH.** *(Added
  2026-09-15 with the withdrawal of exit criterion (5); `DECISIONS-2026-09-14 D-14`.)* Both
  `crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs` **and its committed `.stderr`** exist, and
  the kernel's must-fail suite passes under its own name. A harness that runs proves nothing about a case that is
  not there — which is exactly what the withdrawn check (5) could not see.

- ***`ci.yml` is green on the pushed branch, every job, both OS legs*** (D21-42 14a), *read from the recorded run and
  never re-pushed by this checkpoint.* *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* *(s8j §8j.8 A5.4)*
- ***`.gitattributes` opens with `* text=auto eol=lf`*** (ADR-0164). *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* *(s8j §8j.8 A5.4)*

Those nineteen are not read, they are run. *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* *(s8j §8j.8 A5.4: was ~~Those seventeen are not read, they are run.~~)* Checkpoint 8's mechanical lines, in bullet order:

```bash
cd D:/repos/redkiln-rs
# Run this block as a script or in a subshell — every guard below ends in `exit 1`.
# Guards first: an absent or empty tree answers every prohibition below for free (Halt 8).
test -n "$(ls -A crates)" || { echo "FAIL: crates/ empty — Checkpoint 8's crates/ checks NOT RUN (Halt 8)"; exit 1; }
test -f .redkiln/facets.toml && test -d .redkiln/packs && test -d .redkiln/catalog || { echo "FAIL: .redkiln/ not derived — Checkpoint 8's registry and pack checks NOT RUN (Halt 8)"; exit 1; }
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.1's verbatim copy of Checkpoint 8b sections (5) and (6) runs
# its cargo selections through `nodispatch` and reads `nd_check`, both from runbook/tools/nodispatch.sh (D21-47's
# one no-dispatch wrapper). It is set up here, before anything runs, as Checkpoint 8b's section (W) sets it up. Inserted.
test -f runbook/tools/nodispatch.sh || { echo "FAIL: runbook/tools/nodispatch.sh is absent — A5.1's copied selection cannot run"; exit 1; }
. runbook/tools/nodispatch.sh || { echo "FAIL: could not source runbook/tools/nodispatch.sh"; exit 1; }
nd_setup "$PWD/target/s8-cp/nd" || exit 1

# no require row keyed on an LLM-emitted facet, and after Step 4a no llm facet at all
# REPAIRED 2026-09-21 (Step 8, operator decision). This was ONE statement and it was RED on a correct
# tree: nine hits, EVERY ONE A COMMENT -- five doc-comments under crates/ (including
# `crates/rk-kernel/tests/it/rigor.rs`, whose comment carries this very ban's regex) and four `#`
# lines under .redkiln/ (three catalog `# reach: ... require row` headers and one 7e pack note).
# NOT ONE is a require row. s7b settled this family's reading already: *"Comment lines are excluded
# rather than halted on: the house pattern is a file whose header quotes the governing rule in
# prose."* Halt 7's rule there -- *"do not widen the guard, and do not write the hit"* -- is why the
# PROSE does not give way here: it is pre-existing, and one hit is the ban itself.
# THE PATTERN IS DELIBERATELY NOT WIDENED to s7b's `(require|requires|applicable_if|when)` predicate
# form. Measured 2026-09-21: that form returns 79 comment-free hits over this line's scope, every one
# legitimate -- `when = "criticality >= high"` rows in the SHIPPED packs and `applicable_if` rows in
# the prescribed fixture trees -- because all seven axes are `source = "human"` after Step 4a
# (ADR-0090), and design 10.2 principle 3 scopes the ban to a facet *sourced from an LLM
# classification dispatch*. s7b carries that form only behind its two path exclusions; this line's
# scope includes .redkiln/, where those rows correctly live. The llm half is asserted by the NEXT
# line, mechanically and completely: zero facets declare `source = "llm"`.
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.5: the stanza below is inserted, and the walk after it gains
# --exclude-dir=worktrees after -rnE. Was:
# ~~raw=$(grep -rnE 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' crates/ .redkiln/ --include=*.rs --include=*.toml); rc=$?~~
# AMENDED at s8j (s8j §8j.8 A5.5; D21-44, whose walk rule is s8d-ii's): the two walks over .redkiln/ below skip every
# directory named worktrees — a run worktree under .redkiln/worktrees/<item> is a full repository copy.
WALKED='require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b|append(_route)? *= *true|findings\.none\(evidence in \[|quality_shorthand *='
lost=$(git ls-files | grep -E '(^|/)worktrees/' | while IFS= read -r wf; do grep -lE "$WALKED" "$wf"; done)
test -z "$lost" || { echo "FAIL: a tracked file under a directory named worktrees carries a walked pattern — the skip would hide it:"; printf '%s\n' "$lost"; exit 1; }
ctl=target/s8-a55; rm -rf "$ctl"; mkdir -p "$ctl/.redkiln/worktrees/x" "$ctl/.redkiln/packs" || { echo "FAIL: cannot create $ctl"; exit 1; }
printf 'append = true\n' > "$ctl/.redkiln/worktrees/x/a.toml" && printf 'append = true\n' > "$ctl/.redkiln/packs/b.toml" || { echo "FAIL: cannot write the worktrees-skip controls"; exit 1; }
hits=$(grep -rlE --exclude-dir=worktrees 'append(_route)? *= *true' "$ctl/.redkiln" --include=*.toml); rc=$?
[ "$rc" -eq 0 ] && [ "$hits" = "$ctl/.redkiln/packs/b.toml" ] || { echo "FAIL: the worktrees skip is wrong (rc=$rc) — it must drop the run-worktree copy and keep the pack: $hits"; exit 1; }
rm -rf "$ctl"
raw=$(grep -rnE --exclude-dir=worktrees 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' crates/ .redkiln/ --include=*.rs --include=*.toml); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the LLM-ordinal search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
live=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE '^[[:space:]]*$'); test -z "$live" || { echo "FAIL: an LLM-emitted facet keys a require row:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf 'p.toml:1:require = "has_repro"\n' | grep -E 'require.*\b(criticality|novelty|size|blast_radius|market_facing|external_dep|has_repro)\b' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)'); test -n "$syn" || { echo "FAIL: the comment exclusion swallowed a live require row -- this ban is vacuous"; exit 1; }
test "$(grep -c '^source = "llm"' .redkiln/facets.toml)" = 0 || { echo "FAIL: a facet still declares source = \"llm\" (ADR-0090; DECISIONS-2026-09-11 #2)"; exit 1; }

# stale_policy recorded per cost class, never defaulted from P2 (decision 5; Step 7e; warrant F40 §3)
set=$(grep -rh 'stale_policy' .redkiln/packs --include=*.toml); test -n "$set" || { echo "FAIL: no stale_policy written in any pack — decision 5 was defaulted, not recorded"; exit 1; }
printf '%s\n' "$set" | grep -v 'rematerialise'; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a stale_policy is not rematerialise (rc=$rc)"; exit 1; }

# no require row on a declared or NOT MEASURABLE facet (ADR-0093)
# REPAIRED 2026-09-21 (Step 8, operator decision), same shape as the line above and same warrant.
# RED on a correct tree: six hits, ALL SIX Step 7e's own `-- DROPPED: [[stage.X.require]] when =
# 'data_migration'` bookkeeping comments -- records of rows 7e REMOVED yesterday under this very rule.
# A gate broken by the record of its own enforcement is the defect, not the record.
# THE PATTERN IS DELIBERATELY NOT WIDENED to the predicate-keyword form here either, and the reason is
# sharper than above: measured 2026-09-21, that form returns 8 comment-free hits over this scope and
# EVERY ONE IS AN `applicable_if`, NOT A `require` ROW -- .redkiln/catalog/threat-model.toml,
# security-review.toml, security-requirements.toml, data-migration-plan.toml and four more. ADR-0093
# bars these facets from keying a REQUIRE ROW and leaves `applicable_if` as the sanctioned route; 7e's
# own DROPPED notes say so in terms (*"The demand rides on catalog/threat-model.toml's
# applicable_if"*). Widening here would make the gate red on the route the record prescribes.
raw=$(grep -rnE 'require.*\b(surfaces|data_migration|recent_deploy)\b' .redkiln/packs .redkiln/catalog); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the declared/NOT-MEASURABLE search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }
live=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -vE '^[[:space:]]*$'); test -z "$live" || { echo "FAIL: a declared or NOT MEASURABLE facet keys a require row:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf 'p.toml:1:require = "data_migration"\n' | grep -E 'require.*\b(surfaces|data_migration|recent_deploy)\b' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)'); test -n "$syn" || { echo "FAIL: the comment exclusion swallowed a live require row -- this ban is vacuous"; exit 1; }

# bisect's ceiling on the SHIPPED catalog — spikes/p9's copy is already green, so it may not be the path (PIA-035)
grep -q 'max_rigor *= *"recommended"' .redkiln/catalog/bisect.toml || { echo "FAIL: bisect has no max_rigor = \"recommended\" ceiling (P7-CEIL; DECISIONS-2026-09-11 #1)"; exit 1; }
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.1 (D21-42 13a, 13b): the three lines that opened
# `out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?` walked spikes/p9/catalog through rigor.rs's old
# SHIPPED_CATALOG, and are replaced by Checkpoint 8b section (5)'s code lines and section (6)'s five walked-count readings,
# copied verbatim from runbook/phase-1/s8b-catalog-quality.md as landed (Checkpoint 8j (8) compares them). Was:
# ~~out=$(cargo test -p rk-kernel -- rigor_monotonicity 2>&1); rc=$?~~
# ~~[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }~~
# ~~printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the rigor_monotonicity filter matched 0 tests"; exit 1; }~~
# --- Checkpoint 8b section (5), verbatim:
K_TESTS="lattice::shipped_axes_add_rust_and_surfaces_and_the_p6_axes_are_unchanged memo::ac018_live_verb_may_not_memoise memo::ac018_frozen_p9_catalog_is_red quality_block::adr0072_s8b_repairs_are_versioned_and_changelogged pack_validation::tdd_red_verify_projects_tests_added_per_adr_0035 pack_validation::sca_scan_blocks_on_unrated_or_routes_to_accept_findings lattice::p6_lattice_parity_over_the_p9_tree lattice::p6_validate_parity_over_the_p9_tree lattice::p6_lattice_over_the_shipped_pack lattice::p6_validate_over_the_shipped_pack lattice::zero_packs_is_refused_not_reported lattice::covering_array_is_pairwise_and_matches_the_python_rows lattice::unsatisfiable_fixture_names_its_cell lattice::rank_tie_fixture_is_an_error lattice::hard_cycle_fixture_is_an_error lattice::dead_activity_fixture_is_listed lattice::family_hole_at_mandatory_fails_totality p9_pin::reads_the_object_store_at_the_named_commit_not_the_working_tree rigor::rigor_monotonicity rigor::rigor_monotonicity_p9_historical_2026_09_13 review::ac303_confirm_step_survives_escalation review::ac303_multi_hop_confirm_is_seen review::ac303_twin_broken_chain_is_lost review::ac303_read_binding_is_strict review::ac303_back_edge_counts_and_is_reported quality_block::ac527_empty_set_terminal_condition quality_block::ac527_frozen_p9_catalog_is_red quality_block::d2_per_class_witness_refuses_one_member_rows quality_block::d2_named_form_refuses_any_member_list quality_block::d2_severity_only_predicates_load quality_block::d2_rewritten_rows_are_not_constant_over_their_quantity quality_block::d2_rewritten_rows_read_a_quantity_the_loop_varies quality_block::adr0072_d2_edits_are_versioned_and_changelogged conformance::shipped_pack_loads"
nk=$(printf '%s\n' $K_TESTS | grep -c .)
[ "$nk" -eq 34 ] || { echo "FAIL: K_TESTS names $nk tests, not 34"; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- $K_TESTS --exact 2>&1); rc=$?
nd_check "the named selection" || exit 1
[ "$rc" -eq 0 ] || { echo "FAIL: the stage's named selection did not pass (rc=$rc)"; printf '%s\n' "$out" | tail -25; exit 1; }
for t in $K_TESTS; do
  printf '%s\n' "$out" | grep -qxF "test $t ... ok" || { echo "FAIL: $t did not report ok — renamed, deleted or not selected"; printf '%s\n' "$out" | grep -E '^test ' | tail -25; exit 1; }
done
printf '%s\n' "$out" | grep -qE 'test result: ok\. 34 passed; 0 failed' || { echo "FAIL: the named selection did not run exactly 34 tests"; printf '%s\n' "$out" | grep -E 'test result:'; exit 1; }
# --- Checkpoint 8b section (6), verbatim: the shipped-pack lattice's reading and the guards on what it walked:
out=$(nodispatch cargo test -p rk-kernel --test it -- lattice::p6_lattice_over_the_shipped_pack --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the shipped-pack lattice run failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
line=$(printf '%s\n' "$out" | grep -oE 'lattice-shipped: .*' | grep -E '^lattice-shipped: packs=[0-9]+ rigor=pack-default contexts=[0-9]+ pairs=[0-9]+ unsat=[0-9]+ dead=[0-9]+/102 ties=[0-9]+ hard_cycles=[0-9]+ ')
test -n "$line" || { echo "FAIL: the shipped-pack lattice printed no reading of the agreed shape — nothing says what it walked"; exit 1; }
[ "$(printf '%s\n' "$line" | grep -c .)" -eq 1 ] || { echo "FAIL: the shipped-pack lattice printed more than one reading"; exit 1; }
field() { printf '%s\n' "$line" | sed -E "s/^.* $1=([0-9]+)[ /].*$/\1/"; }
packs=$(field packs); ctx=$(field contexts); pairs=$(field pairs); unsat=$(field unsat); dead=$(field dead); ties=$(field ties); hc=$(field hard_cycles)
[ "$packs" -eq 2 ] || { echo "FAIL: the lattice walked $packs packs over .redkiln/, not the installed 2"; exit 1; }
[ "$ctx" -gt 0 ] && [ "$pairs" -gt 0 ] || { echo "FAIL: the lattice walked zero contexts or pairs — a report over nothing (Halt 8)"; exit 1; }
[ "$ties" -eq 0 ] && [ "$hc" -eq 0 ] || { echo "FAIL: AC-103 over the installed pair: ties=$ties hard_cycles=$hc — the tolerance is zero"; exit 1; }
echo "CP8B (6) AC-101/AC-102 over the INSTALLED pair, reported and NOT thresholded (D21-47 V3; ADR-0011, E7): packs=$packs rigor=pack-default unsat=$unsat/$pairs dead=$dead/102 (all modes, as lattice.py counts it)"
printf '%s\n' "$line" | grep -qE ' toolchains=([^ ]*,)?rust(,[^ ]*)? surfaces=[1-9][0-9]*$' || { echo "FAIL: the shipped-pack lattice did not walk the rust toolchain and a non-empty surfaces binding (D21-43 N26)"; printf '%s\n' "$line"; exit 1; }
# --- Checkpoint 8b section (6), verbatim: the ac018, rigor_monotonicity, ac527 and ac303 readings:
out=$(nodispatch cargo test -p rk-kernel --test it -- memo::ac018_live_verb_may_not_memoise --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the AC-018 lint failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -oE 'ac018: .*' | grep -qE '^ac018: catalog=\.redkiln/catalog activities=102 steps=[1-9][0-9]* violations=0 registry=tests/fixtures/registries/verbs/verbs-live\.toml$' || { echo "FAIL: the AC-018 lint did not report .redkiln/catalog, 102 activities, a non-zero step count, zero violations and the FIXTURE registry it read"; printf '%s\n' "$out" | grep -oE 'ac018: .*'; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- rigor::rigor_monotonicity --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: rigor_monotonicity failed (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -oE 'rigor_monotonicity: .*' | grep -qE '^rigor_monotonicity: catalog=\.redkiln/catalog activities=102 families=[1-9][0-9]* contexts=[1-9][0-9]* ' || { echo "FAIL: rigor_monotonicity did not report reading .redkiln/catalog with a non-zero walk"; printf '%s\n' "$out" | grep -oE 'rigor_monotonicity: .*'; exit 1; }
out=$(nodispatch cargo test -p rk-kernel --test it -- quality_block::ac527_empty_set_terminal_condition --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the AC-527 lint failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -oE 'ac527: .*' | grep -qE '^ac527: catalog=\.redkiln/catalog checks=[1-9][0-9]* refused=0 indeterminate=[0-9]+$' || { echo "FAIL: the AC-527 lint did not report .redkiln/catalog, a non-zero check count and refused=0"; printf '%s\n' "$out" | grep -oE 'ac527: .*'; exit 1; }
# (dated 2026-09-25, refuting round 5) AC-303's reading too, so that every re-pointed instrument's printed path is read.
out=$(nodispatch cargo test -p rk-kernel --test it -- review::ac303_confirm_step_survives_escalation --exact --nocapture 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: AC-303's confirm rule failed over the shipped catalog (rc=$rc)"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -oE 'ac303: .*' | grep -qE '^ac303: catalog=\.redkiln/catalog families=[1-9][0-9]* contexts=[1-9][0-9]* comparisons=[1-9][0-9]* violations=0 forward=4 back_edge=16$' || { echo "FAIL: AC-303 did not report .redkiln/catalog, a non-zero walk, zero violations and forward=4 back_edge=16"; printf '%s\n' "$out" | grep -oE 'ac303: .*'; exit 1; }
# (dated 2026-09-26, s8d build, package P2; D21-47's time-bound rule; D21-67 ruling 1) The pin above read
# `forward=16 back_edge=4` until s8d's catalog commit: class C removed the severity conjunct and its `findings` input
# from twelve family members' `policy` checks, each of which now consumes the reviewer's Findings only through the
# loop's back-edge, so twelve confirm checks move from forward to back-edge and none is lost (review.rs's dated re-pin).
nd_check "sections (1) to (6)" || exit 1

# ChangeSummary nowhere in crates/ but its rk-ports newtype (ADR-0093; S-BIN unrun — the BrittonGames fixture is not the spike)
# REPAIRED 2026-09-21 (Step 8, operator decision). This was a bare MENTION-grep and it was RED on a
# correct tree: 67 hits across 16 files, most of them comments and every non-comment one a `use` or a
# trait-method signature that ADR-0094 decision 1 REQUIRES -- both toolchain adapters implement
# `Toolchain::change_summary`, so they must name the type. NARROWED TO THIS BULLET'S OWN WORDS: *"No
# finalised `ChangeSummary` type; no synthesised `ChangeSummary` value."* A definition, or a
# construction, outside rk-ports. Measured 2026-09-21: the ONLY definition in the workspace is
# `crates/rk-ports/src/toolchain.rs`'s `pub struct ChangeSummary(pub(crate) ())` -- an opaque newtype
# whose single field is crate-private, so it is neither finalised nor constructible from outside --
# and there are ZERO constructions anywhere outside rk-ports; both adapters' `change_summary` returns
# `Err(registry::refusal(...))` and synthesises nothing. So this bullet is TRUE of the tree.
# [amended 2026-09-24, s8a §8a.3, ADR-0165] CORRECTED: the comment above said Checkpoint 5 was "RED with four hits in
# the two adapters". Measured at ca8f387 it was EIGHT literal sites (an import and the change_summary signature in each
# of the two adapters and rk-cli's review_pass.rs, plus rk-cli's run.rs and rk-kernel's runtime.rs, which wrote the
# path-qualified form), of which SIX came through its ::ChangeSummary-excluding filter. ADR-0165 resolves the conflict
# behind them (D17-9; ADR-0094 decision 1 against ADR-0099's forbids bullet) and Checkpoint 5 is repaired by amendment
# in s8a. This guard now asserts the bullet's amended words: no ChangeSummary definition outside rk-ports (Checkpoint
# 5's ban, the same CS_BAN as Checkpoint 8a guard (6)), and no Binary payload constructed outside rk-ports. The Text
# arm may be constructed by any Toolchain implementor, and a match arm naming ChangeSummary::Binary is not a
# construction. Code lines only, string literals blanked; the positive anchor is rk-ports' enum.
# [amended 2026-09-25, s8a fix round 1; the same arms as Checkpoint 8a guard (6)'s CS_BAN] The rename arm fired only on a capital or _ after `as`, so a lowercase
# renaming import (`use …::ChangeSummary as summary;`, `{ChangeSummary as cs, …}`) passed, and rustc -D warnings
# accepts one. Two arms are added: a renaming import on a `use` line, whatever the alias's case, and a line made only
# of import-list items one of which renames …ChangeSummary. Controls 12-13 fire; 14 (a numeric cast) is
# admitted. Stated limit: a lowercase cast that is the whole of a list-item line (`X::ChangeSummary as u8,`) fires.
# [amended 2026-09-25, s8a fix round 3] Stated limit, corrected: the ban reads one PHYSICAL line at a time, and rustfmt
# wraps any definition wider than its width, so fmt --check does NOT hold the tree to a one-line form for those. Measured
# 2026-09-25: rustfmt writes a long alias as `pub type X =` then `    Result<…ChangeSummary, …>;`, and a long impl
# header with `    for ChangeSummary` on its own line; the ban sees neither, alias or impl. A line-joined (statement)
# reading is the fix, owed and not taken here.
# [amended 2026-09-25, s8a fix round 4] The Binary arm fired on `BinaryChangeSummary` followed by `(` or `{` wherever it
# stood, so it also refused two forms that construct nothing and that ADR-0165 licenses: a return type
# (`-> BinaryChangeSummary {`) and a `{ .. }` rest pattern (`Binary(b @ BinaryChangeSummary { .. }) => b,`). rustc
# accepts both outside rk-ports; a real construction is already refused by the crate-private field (E0451). S8_UNCON
# blanks those two forms, and ONLY for the Binary arm: the definition arm reads the line unblanked. Controls 15-17 are
# admitted; 18 (a closure returning a constructed payload) still fires.
# [amended 2026-09-25, s8a fix round 13] The impl arm took only a (path-qualified) name straight after `for`,
# so `impl Render for &ChangeSummary`, `for &'a rk_ports::toolchain::ChangeSummary`, `for Box<ChangeSummary>` and
# `for Option<…BinaryChangeSummary>` passed, and each compiles outside rk-ports under rustc -D warnings. The arm
# now takes any text between `for` and the name on the line; no admitted control carries `impl`. Controls 21 and 22
# (a reference and a Box<…>) fire. The one-physical-line limit above is unchanged.
# [amended 2026-09-25, s8a fix round 14] The definition arm and the `impl … for` arm matched the name as
# [A-Za-z_]*ChangeSummary, so a raw identifier (`pub struct r#ChangeSummary;`, a type literally named
# ChangeSummary) and a digit in the prefix (`pub struct V2ChangeSummary;`, `impl Clone for Sha1ChangeSummary {`)
# passed, and each compiles under rustc -D warnings. Both arms now take [A-Za-z0-9_]*, and the definition arm an
# optional r# prefix. Controls 23, 24 and 25 fire.
S8_DEF='\b(struct|enum|type|trait|union)[[:space:]]+(r#)?[A-Za-z0-9_]*ChangeSummary\b|\btype[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[^=]*=.*\b[A-Za-z_]*ChangeSummary\b|\bimpl\b.*\bfor[[:space:]]+.*\b[A-Za-z0-9_]*ChangeSummary\b|\bimpl(<[^>]*>)?[[:space:]]+([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*ChangeSummary\b|\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Z_]|\buse\b[^;]*\b[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_]|^[^:]+:[0-9]+:[[:space:]]*([A-Za-z_][A-Za-z0-9_:]*([[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*)?[[:space:]]*,[[:space:]]*)*([A-Za-z_][A-Za-z0-9_]*::)*[{]?[A-Za-z_]*ChangeSummary[[:space:]]+as[[:space:]]+[A-Za-z_][A-Za-z0-9_]*[[:space:]]*(,.*)?$'
S8_BIN='\b[A-Za-z_]*BinaryChangeSummary[[:space:]]*[({]|\b[A-Za-z_]*BinaryChangeSummary::[A-Za-z_]+[[:space:]]*\('
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS; s8j section 8j.8 A13; D21-155's owed item; s8j section 8j.0 item 7)*
# S8_BIN SEES THE POINT-FREE AND THE QUALIFIED CONSTRUCTION. The line above needs `(` straight after the associated name
# and nothing between the type name and `::`, so it admitted `parts.map(BinaryChangeSummary::new)` and
# `<BinaryChangeSummary>::new(…)`, both constructions; s8i widened its own BIN_RE for the same two forms (s8i 8i.9 (8),
# operator ruling R10). The line above is not rewritten: the one line below widens its second arm in BIN_RE's form, and
# appends it after the last `|`, so `${S8_BIN%|*}` is the line above again (the controls after the test-module controls
# read it so). S8_CTOR and the in-source test-module blank still turn the recorded `BinaryChangeSummary::new(` into `_(`
# before S8_BIN reads the line, so the admitted test sites stay admitted; a point-free or qualified use in a test path is
# not blanked and fires, which is the safe side.
S8_BIN="$S8_BIN|"'\b[A-Za-z_]*BinaryChangeSummary>?[[:space:]]*::[[:space:]]*[A-Za-z_][A-Za-z0-9_]*'
S8_UNCON='s/(->|@)([[:space:]]*)([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*BinaryChangeSummary\b/\1\2_/g; s/\b([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*BinaryChangeSummary[[:space:]]*\{[[:space:]]*\.\.[[:space:]]*\}/_/g'
# [amended 2026-09-25, s8a fix round 5] The string blank was `s/"([^"\]|\.)*"/""/g`: in ERE a backslash inside a
# bracket is literal and `\.` is a literal dot, so a string holding a backslash escape ("a\n") was never blanked and
# its closing quote paired with the NEXT string on the line, blanking the code between (a ChangeSummary impl, a
# diff_tree_to_tree call). It is now escape-aware, `[^"\\]|\\.` (a non-quote non-backslash, or a backslash and any
# byte), as sealed.py's \\[\s\S] has been since fix round 1. Planted controls carry a backslash-escaped string.
# ~~s8_cs() { local in; in=$(grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g'); { printf '%s\n' "$in" | grep -E "$S8_DEF"; printf '%s\n' "$in" | sed -E "$S8_UNCON" | grep -E "$S8_BIN"; }; }~~
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* THE RECORD'S CONSTRUCTION RULE (s8i section 8i.8; ADR-0199 decisions 2 and 4, proposed).
# S-BIN has reported (F43, proposed). The Binary payload may now be built outside rk-ports through its one recorded
# checked constructor, BinaryChangeSummary::new, and only in test code while the record's producer line reads none:
# the render and review-input tests construct it in crates/<crate>/tests. S8_CTOR blanks that call, path-qualified
# or not, on a line whose path is under crates/<crate>/tests, benches or examples, and ONLY there, before S8_BIN reads
# the line. A struct literal, a tuple construction or any other associated function still fires everywhere, and so
# does the recorded constructor in a crate's src. The line above is the superseded s8_cs; the old controls stand
# (their paths are x.rs, so control 5's `BinaryChangeSummary::new(1)` still fires), and six more follow the loop.
S8_CTOR='\%^crates/[^/:]+/(tests|benches|examples)/%s/\b([A-Za-z_][A-Za-z0-9_]*::)*BinaryChangeSummary::new[[:space:]]*\(/_(/g'
s8_cs() { local in; in=$(grep -v '^crates/rk-ports/' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(//|/\*|\*( |/|$))' | sed -E 's/"([^"\\]|\\.)*"/""/g'); { printf '%s\n' "$in" | grep -E "$S8_DEF"; printf '%s\n' "$in" | sed -E "$S8_UNCON" | sed -E "$S8_CTOR" | grep -E "$S8_BIN"; }; }
# *(dated 2026-10-14, s8i build, S8I-D3-RECORDS; fix round D, measured by S8I-C7-REVIEW-INPUT)* A FALSE POSITIVE, fixed by
# extending S8_UNCON, not by rewriting it: its `->` arm admits a return type only when the name follows `->` directly,
# so a return type written `-> &BinaryChangeSummary {` (or `-> &'a mut path::BinaryChangeSummary {`) reached S8_BIN
# unblanked and fired as a construction, the `{` being the fn body's. The expression appended below blanks the name
# after `->`, `&`, an optional lifetime and an optional `mut`, and ONLY there; the line above stands. A construction
# later on the same line still fires (a body `{ &BinaryChangeSummary(()) }`), as control 18 does for the bare form.
# Four controls follow the construction rule's loop: two return types are admitted, two constructions fire.
S8_UNCON="$S8_UNCON; s/->([[:space:]]*)&([[:space:]]*'[A-Za-z_][A-Za-z0-9_]*)?([[:space:]]*mut[[:space:]]+)?[[:space:]]*([A-Za-z_][A-Za-z0-9_]*::)*[A-Za-z_]*BinaryChangeSummary\b/->\1_/g"
raw=$(grep -rn 'ChangeSummary' crates/ --include=*.rs); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the ChangeSummary search found nothing or could not run (grep rc=$rc) -- rk-ports' own enum must be seen"; exit 1; }
printf '%s\n' "$raw" | grep -qE '^crates/rk-ports/src/toolchain.rs:[0-9]+:pub enum ChangeSummary[[:space:]]*\{' || { echo "FAIL: rk-ports' own ChangeSummary enum was not found by this search -- the split did not land, or the ban is looking at the wrong tree"; exit 1; }
# *(dated 2026-10-15, s8i build, S8I-F1-GUARDS; s8i refuting round 21's blocker B1, operator ruling R10)* IN-SOURCE TEST
# MODULES ARE TEST CODE. s8i's fix round E1 put a test fake inside `crates/rk-cli/src/verbs/run/kept_refusal.rs`'s
# `#[cfg(test)] mod tests`. The fake builds the payload through the recorded constructor, and this guard fired on it,
# because S8_CTOR admits that call only by path (crates/<crate>/tests, benches, examples). The code is test code, and
# it does not exist in a shipped build. s8i owns this amendment because s8i's change made the guard false. The lines
# below are inserted, and no line above is rewritten. Inside an in-source test module, located by s8_cfgtest_mods, the
# recorded constructor `BinaryChangeSummary::new(` (path-qualified or not) is blanked exactly as S8_CTOR blanks it under
# a test path, and ONLY that call. Everything else on such a line still reaches s8_cs unchanged: a definition, an alias,
# a struct literal, a tuple construction and any other associated function all still fire. Outside a test module,
# nothing changes.
# A test module is a line that is exactly `#[cfg(test)]` (or `#[cfg(test)] mod <name> {`), followed on the NEXT line by
# `mod <name> {`. It runs from that line to the line where its braces close. The braces are counted by a lexer that
# runs from the top of the file. The lexer skips `//` comments, nested `/* */` comments, strings that span lines, raw
# strings and char literals, and it leaves lifetimes alone. A `#[cfg(test)]` on any other item opens nothing, and so
# does one inside a string or a comment. A module that never closes fails the guard.
# Measured 2026-10-15 over crates/*/src: eight modules, the same ranges as an independent reading (s8i §8i.9 (8) lists
# them). Of the files this guard's search returns, kept_refusal.rs is the only one with a test module. Its module runs
# from line 63 to line 288, and line 116 is inside it. The controls follow the construction rule's loops.
# (dated 2026-10-15, s8i build, orchestrator; a minor of s8i refuting round 22) Correction: crates/rk-cli/tests/it/review_input.rs (lines 61-215) also holds a
# test module; it sits under tests/, which S8_CTOR already admits, so the count of src files is the claim.
s8_cfgtest_mods() {
  LC_ALL=C awk '
    function lex(s,   i, n, c, d, j, h) {
      n = length(s); i = 1
      while (i <= n) {
        c = substr(s, i, 1); d = substr(s, i, 2)
        if (bc > 0) { if (d == "/*") { bc++; i += 2 } else if (d == "*/") { bc--; i += 2 } else i++; continue }
        if (st == 1) { if (c == "\\") i += 2; else { if (c == "\"") st = 0; i++ }; continue }
        if (st == 2) { if (c == "\"" && substr(s, i + 1, rh) == hs) { st = 0; i += 1 + rh } else i++; continue }
        if (d == "//") return
        if (d == "/*") { bc = 1; i += 2; continue }
        if (c == "\"") { st = 1; i++; continue }
        # (dated 2026-10-15, s8i build, orchestrator; the major of s8i refuting round 22) `cr` joins `br`: a raw C string `cr"\"` was read as an escaped string, which
        # inverted the string state and could admit a later production call. Was: (c == "r" || d == "br"), and j = i + (c == "b" ? 2 : 1).
        if ((c == "r" || d == "br" || d == "cr") && (i == 1 || substr(s, i - 1, 1) !~ /[A-Za-z0-9_]/)) {
          j = i + (c == "b" || c == "c" ? 2 : 1); h = ""
          while (substr(s, j, 1) == "#") { h = h "#"; j++ }
          if (substr(s, j, 1) == "\"") { st = 2; hs = h; rh = length(h); i = j + 1; continue }
        }
        if (c == "\047") {
          if (substr(s, i + 1, 1) == "\\") { j = index(substr(s, i + 3), "\047"); i = j ? i + 3 + j : n + 1; continue }
          if (substr(s, i + 2, 1) == "\047") { i += 3; continue }
          if (substr(s, i + 1, 1) > "\177") { j = index(substr(s, i + 1, 5), "\047"); if (j > 1) { i += j + 1; continue } }
          i++; continue
        }
        if (c == "{") depth++; else if (c == "}") depth--
        i++
      }
    }
    function done_file() { if (inmod) print "UNCLOSED " fname " " start; inmod = 0 }
    FNR == 1 { done_file(); fname = FILENAME; depth = 0; bc = 0; st = 0; pend = 0 }
    {
      line = $0; sub(/\r$/, "", line); clean = (bc == 0 && st == 0)
      if (!inmod && clean) {
        if (pend && line ~ /^[ \t]*(pub(\([^)]*\))?[ \t]+)?mod[ \t]+[A-Za-z_][A-Za-z0-9_]*[ \t]*\{/) { inmod = 1; start = pend; base = depth }
        else if (line ~ /^[ \t]*#\[cfg\(test\)\][ \t]*(pub(\([^)]*\))?[ \t]+)?mod[ \t]+[A-Za-z_][A-Za-z0-9_]*[ \t]*\{/) { inmod = 1; start = FNR; base = depth }
      }
      pend = (!inmod && clean && line ~ /^[ \t]*#\[cfg\(test\)\][ \t]*$/) ? FNR : 0
      lex(line)
      if (inmod && depth <= base) { print fname " " start " " FNR; inmod = 0 }
    }
    END { done_file() }' "$@"
}
s8_cfgtest() {
  local in f ranges=""; local -a fa=()
  in=$(cat)
  while IFS= read -r f; do
    [ -n "$f" ] && grep -qE '^[[:space:]]*#\[cfg\(test\)\]' "$f" && fa+=("$f")
  done <<< "$(printf '%s\n' "$in" | sed -n 's/^\([^:]*\):[0-9][0-9]*:.*/\1/p' | sort -u)"
  if [ "${#fa[@]}" -gt 0 ]; then
    ranges=$(s8_cfgtest_mods "${fa[@]}") || { echo "FAIL: the test-module reader could not run"; return 1; }
  fi
  case "$ranges" in *UNCLOSED*) echo "FAIL: an in-source #[cfg(test)] module never closes:"; printf '%s\n' "$ranges" | grep UNCLOSED; return 1;; esac
  # (dated 2026-10-15, s8i close, orchestrator; runbook/tools/check-checkpoints.py [PIPE-STATUS]) The tagger's output
  # is captured and guarded before the blanking sed, so an awk failure is not lost in the pipe. Was: the awk below
  # piped straight into the sed (`printf .. | CFGR=.. awk '..' | sed -E ..`).
  local tagged
  tagged=$(printf '%s\n' "$in" | CFGR="$ranges" awk '
    BEGIN { n = split(ENVIRON["CFGR"], a, "\n"); for (i = 1; i <= n; i++) { split(a[i], f, " "); F[i] = f[1]; S[i] = f[2] + 0; E[i] = f[3] + 0 } }
    { p = index($0, ":"); file = substr($0, 1, p - 1); r = substr($0, p + 1); ln = substr(r, 1, index(r, ":") - 1) + 0
      t = 0; for (i = 1; i <= n; i++) if (F[i] == file && ln >= S[i] && ln <= E[i]) t = 1
      print (t ? "T|" : "P|") $0 }') || { echo "FAIL: the test-module tagger could not run"; return 1; }
  printf '%s\n' "$tagged" | sed -E '/^T[|]/s/\b([A-Za-z_][A-Za-z0-9_]*::)*BinaryChangeSummary::new[[:space:]]*\(/_(/g; s/^[TP][|]//'
}
raw=$(printf '%s\n' "$raw" | s8_cfgtest) || { printf '%s\n' "$raw"; exit 1; }
out=$(printf '%s\n' "$raw" | s8_cs); test -z "$out" || { echo "FAIL: a ChangeSummary definition, or a Binary payload constructed, outside rk-ports (ADR-0165):"; printf '%s\n' "$out"; exit 1; }
for ctl in 'F|x.rs:1:pub struct ChangeSummary;' 'F|x.rs:2:type Cs = rk_ports::toolchain::ChangeSummary;' 'F|x.rs:3:    let b = BinaryChangeSummary(());' 'F|x.rs:4:    let b = rk_ports::toolchain::BinaryChangeSummary { bytes: 1 };' 'F|x.rs:5:    let b = BinaryChangeSummary::new(1);' 'P|x.rs:6:        Ok(ChangeSummary::Text(text))' 'P|x.rs:7:        ChangeSummary::Binary(_) => {}' 'P|x.rs:8:use rk_ports::toolchain::{ChangeSummary, Classification, Toolchain};' 'P|x.rs:9:    fn change_summary(&self, _range: &str) -> Result<ChangeSummary, StepError> {' 'P|x.rs:10:    let k = "BinaryChangeSummary(())";' 'P|x.rs:11:    // let b = BinaryChangeSummary(());' 'F|x.rs:12:use rk_ports::toolchain::ChangeSummary as summary;' 'F|x.rs:13:    toolchain::{ChangeSummary as cs, Toolchain},' 'P|x.rs:14:    let n = Capability::ChangeSummary as usize;' 'P|x.rs:15:fn binary_of(cs: ChangeSummary) -> BinaryChangeSummary {' 'P|x.rs:16:        ChangeSummary::Binary(b @ BinaryChangeSummary { .. }) => b,' 'P|x.rs:17:        ChangeSummary::Binary(rk_ports::toolchain::BinaryChangeSummary { .. }) => {}' 'F|x.rs:18:    let f = || -> BinaryChangeSummary { BinaryChangeSummary(()) };' 'F|x.rs:19:    const A: &str = "a\n"; impl Clone for ChangeSummary { fn n() -> &str { "z" } }' 'P|x.rs:20:    let k = "a\"impl Clone for ChangeSummary\"";' 'F|x.rs:21:impl Render for &ChangeSummary {' 'F|x.rs:22:impl Render for Box<rk_ports::toolchain::ChangeSummary> {' 'F|x.rs:23:pub struct r#ChangeSummary;' 'F|x.rs:24:pub struct V2ChangeSummary;' 'F|x.rs:25:impl Clone for Sha1ChangeSummary {'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | s8_cs)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL must FIRE and did not -- the ban is vacuous for: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL must be ADMITTED and fired -- the ban blocks what ADR-0165 licenses: ${ctl#*|}"; exit 1; }; fi
done
# *(dated 2026-10-14, s8i build, S8I-C4-PORTS)* The construction rule's six controls: the recorded constructor in two test files is
# admitted (bare and path-qualified); it fires in a src file (producers: none), and an unchecked associated fn, a struct
# literal and a second construction beside an admitted call each fire in a test file.
for ctl in 'P|crates/rk-store/tests/it/render.rs:1:    let b = BinaryChangeSummary::new(assets, captures, report, log, "")?;' 'P|crates/rk-cli/tests/it/review_input.rs:2:    let s = ChangeSummary::Binary(rk_ports::toolchain::BinaryChangeSummary::new(a, c, r, l, "").expect("clean"));' 'F|crates/rk-store/src/render.rs:3:    let b = BinaryChangeSummary::new(assets, captures, report, log, "")?;' 'F|crates/rk-store/tests/it/render.rs:4:    let b = BinaryChangeSummary::unchecked(assets);' 'F|crates/rk-cli/tests/it/review_input.rs:5:    let b = BinaryChangeSummary { assets, captures, tests_report_ref, build_log_ref };' 'F|crates/rk-store/tests/it/render.rs:6:    let b = BinaryChangeSummary::new(a, c, r, l, "")?; let d = BinaryChangeSummary(());'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | s8_cs)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL must FIRE and did not -- the construction rule is vacuous for: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL must be ADMITTED and fired -- the ban blocks ADR-0199's recorded constructor: ${ctl#*|}"; exit 1; }; fi
done
# *(dated 2026-10-14, s8i build, S8I-D3-RECORDS)* The reference-return controls (the S8_UNCON extension above): a return
# type `-> &BinaryChangeSummary {` and `-> &'a mut path::BinaryChangeSummary {` are admitted; a body that constructs
# the payload after either still fires. Under the line S8_UNCON had before the extension, the first two fired.
for ctl in 'P|crates/rk-cli/src/review_input.rs:26:    pub fn summary(&self) -> &BinaryChangeSummary {' 'P|crates/rk-cli/src/review_input.rs:27:    fn binary<'"'"'a>(s: &'"'"'a mut ChangeSummary) -> &'"'"'a mut rk_ports::toolchain::BinaryChangeSummary {' 'F|crates/rk-cli/src/review_input.rs:28:    let f = || -> &BinaryChangeSummary { &BinaryChangeSummary(()) };' 'F|crates/rk-cli/src/review_input.rs:29:    fn g() -> &'"'"'static BinaryChangeSummary { &BinaryChangeSummary { bytes: 1 } }'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | s8_cs)
  if [ "$want" = F ]; then [ -n "$got" ] || { echo "FAIL: CONTROL must FIRE and did not -- the reference-return blank hides a construction: ${ctl#*|}"; exit 1; }; else [ -z "$got" ] || { echo "FAIL: CONTROL must be ADMITTED and fired -- a reference return type reads as a construction: ${ctl#*|}"; exit 1; }; fi
done
# *(dated 2026-10-15, s8i build, S8I-F1-GUARDS; operator ruling R10)* The test-module controls. Each writes one
# synthetic file, crates/x/src/y.rs, into a scratch tree. This guard's own lines then run over that tree: the
# ChangeSummary grep, s8_cfgtest and s8_cs. Each control names the exact line numbers the guard must report.
# - The recorded constructor in a src file fires.
# - The same call inside a #[cfg(test)] module is admitted.
# - A call after the module closes fires.
# - A #[cfg(test)] on a fn or an impl opens nothing.
# - So does a #[cfg(test)] that is in a comment, in a string, negated, or a blank line away from its mod.
# - Inside a test module, a struct literal, another associated function and a definition each still fire, as they do
#   in crates/<crate>/tests.
# - A module that never closes fails the guard.
# - The confuser loop (see s8i §8i.9 (8)) checks the brace count.
S8C_TMP=$(mktemp -d) && mkdir -p "$S8C_TMP/crates/x/src" || { echo "FAIL: no scratch tree for the test-module controls"; exit 1; }
s8c_file() { cat > "$S8C_TMP/crates/x/src/y.rs" || { echo "FAIL: could not write the test-module control file"; exit 1; }; }
s8c_run() ( cd "$S8C_TMP" || exit 1; r=$(grep -rn 'ChangeSummary' crates/ --include=*.rs); printf '%s\n' "$r" | s8_cfgtest )
s8c_want() {
  out=$(s8c_run) || { echo "FAIL: CONTROL '$2': the module reader refused: $out"; exit 1; }
  got=$(printf '%s\n' "$out" | s8_cs | cut -d: -f2 | sort -n | uniq | paste -sd' ' -)
  [ "$got" = "$1" ] || { echo "FAIL: CONTROL '$2': the guard reported line(s) '$got', not '$1'"; exit 1; }
}
s8c_file <<'RS'
pub fn produce(a: A) -> Result<BinaryChangeSummary, E> {
    BinaryChangeSummary::new(a, c, r, l, "")
}
RS
s8c_want "2" "the recorded constructor in a src file fires"
s8c_file <<'RS'
pub fn produce() {}
#[cfg(test)]
mod tests {
    use rk_ports::toolchain::{BinaryChangeSummary, ChangeSummary};
    fn fake() -> Result<ChangeSummary, E> {
        if x { let _ = [1, 2]; }
        rk_ports::toolchain::BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary)
    }
}
RS
s8c_want "" "the same call inside a #[cfg(test)] module is admitted"
s8c_file <<'RS'
#[cfg(test)]
mod tests {
    fn fake() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
}
pub fn after() -> Result<ChangeSummary, E> {
    BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary)
}
RS
s8c_want "6" "a call after the test module closes fires"
s8c_file <<'RS'
#[cfg(test)]
fn helper() -> Result<ChangeSummary, E> {
    BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary)
}
#[cfg(test)]
impl Fake {
    fn f() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
}
RS
s8c_want "3 7" "a #[cfg(test)] on a fn or an impl opens no exclusion"
s8c_file <<'RS'
// #[cfg(test)]
mod not_tests {
    fn f() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
}
const DOC: &str = "
#[cfg(test)]
mod quoted { "; pub fn g() -> Result<ChangeSummary, E> {
    BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary)
}
#[cfg(not(test))]
mod shipped {
    fn h() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
}
#[cfg(test)]

mod spaced {
    fn i() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
}
RS
s8c_want "3 8 12 17" "an attribute in a comment, in a string, negated or a line away opens nothing"
s8c_file <<'RS'
/*
#[cfg(test)]
mod commented { */ pub fn k() -> Result<ChangeSummary, E> {
    BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary)
}
RS
s8c_want "4" "an attribute inside a block comment opens nothing"
s8c_file <<'RS'
#[cfg(test)] mod tests {
    fn a() -> BinaryChangeSummary { BinaryChangeSummary { assets, captures, tests_report_ref, build_log_ref } }
    fn b() -> Result<BinaryChangeSummary, E> { BinaryChangeSummary::unchecked(assets) }
    pub struct FakeChangeSummary;
    fn c() -> Result<BinaryChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "") }
}
RS
s8c_want "2 3 4" "inside a test module, a struct literal, another associated fn and a definition still fire"
s8c_file <<'RS'
#[cfg(test)]
mod tests {
    fn fake() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }
RS
out=$(s8c_run) && { echo "FAIL: CONTROL 'a module that never closes': the module reader admitted it: $out"; exit 1; }
while IFS= read -r cz; do
  cz=${cz//<NL>/$'\n'}
  printf '%s\n' '#[cfg(test)]' 'mod tests {' "$cz" '    fn fake() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }' '}' 'fn after() -> Result<ChangeSummary, E> { BinaryChangeSummary::new(a, c, r, l, "").map(ChangeSummary::Binary) }' | s8c_file
  s8c_want "$(( $(printf '%s\n' "$cz" | grep -c '') + 5 ))" "a confuser inside the module: $cz"
done <<'RS'
    let _ = "{";
    let _ = "}";
    let _ = '{';
    let _ = '}';
    let _ = b'{';
    let _ = '"'; let _ = "{";
    let _ = '\''; let _ = '{';
    let _ = '\u{7b}'; let _ = "}";
    let _ = "a\"{";
    let _ = "a\\"; let _ = '}';
    let _ = r#"a"{"#;
    let _ = br"}";
    let _ = r"\"; let _ = '{';
    let _ = cr"\"; let _ = "}";
    let _ = br"\"; let _ = "}";
    // {
    /* } */
    /* /* */ { */
    let s = "{<NL>    }{";
    /* {<NL>    */
    fn f<'a>(x: &'a str) -> &'a str { x }
RS
rm -rf "$S8C_TMP"
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS; s8j section 8j.8 A13)* The widened S8_BIN's controls, two each way,
# on a crates/x/src/y.rs path: the point-free and the qualified construction fire, and each fails under the line S8_BIN
# had before A13 (`${S8_BIN%|*}`, read in a subshell), so the widening is load-bearing; an import and a reference return
# type are admitted.
for ctl in 'F|crates/x/src/y.rs:1:    let v = x.map(BinaryChangeSummary::new);' 'F|crates/x/src/y.rs:2:    let b = <BinaryChangeSummary>::new(a);' 'P|crates/x/src/y.rs:3:use rk_ports::toolchain::BinaryChangeSummary;' 'P|crates/x/src/y.rs:4:    pub fn summary(&self) -> &BinaryChangeSummary {'; do
  want=${ctl%%|*}; got=$(printf '%s\n' "${ctl#*|}" | s8_cs)
  if [ "$want" = F ]; then
    [ -n "$got" ] || { echo "FAIL: CONTROL must FIRE and did not -- A13's widened S8_BIN is vacuous for: ${ctl#*|}"; exit 1; }
    pre=$(S8_BIN=${S8_BIN%|*}; printf '%s\n' "${ctl#*|}" | s8_cs)
    [ -z "$pre" ] || { echo "FAIL: CONTROL fires under the pre-A13 S8_BIN too -- it does not prove the widening: ${ctl#*|}"; exit 1; }
  else
    [ -z "$got" ] || { echo "FAIL: CONTROL must be ADMITTED and fired -- A13's widened S8_BIN blocks what ADR-0165 licenses: ${ctl#*|}"; exit 1; }
  fi
done

# card append route never defaulted on; no quality-shorthand default or terminal condition
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.5: --exclude-dir=worktrees after -rnE. Was:
# ~~grep -rnE 'append(_route)? *= *true' crates/ .redkiln/ --include=*.rs --include=*.toml; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a card append route is enabled by default (rc=$rc)"; exit 1; }~~
grep -rnE --exclude-dir=worktrees 'append(_route)? *= *true' crates/ .redkiln/ --include=*.rs --include=*.toml; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a card append route is enabled by default (rc=$rc)"; exit 1; }
# REPAIRED 2026-09-21 (Step 8, operator decision). RED on a correct tree: six hits -- four comments
# quoting ADR-0080's forbidden predicate (including the must-REFUSE fixture's own header) and two
# Rust STRING LITERALS at `crates/rk-kernel/tests/it/evaluator.rs`, which feed that predicate to
# `truth()` to assert it evaluates Tri::False over a reproduced finding and Tri::True over an absent
# one. That file is the ENFORCEMENT of the rule; a guard that halts on it and the test that proves the
# predicate behaves are jointly unsatisfiable. Comments are filtered and that one path is excluded --
# s7b's route, the EXCLUSION widened rather than the PATTERN narrowed, so a hit anywhere else halts.
#
# RAISED, NOT ACTED ON -- an operator decision, not an instrument repair. This pattern matches only the
# UNQUOTED spelling, and the shipped catalog writes the predicate with QUOTED members. Measured
# 2026-09-21, a pattern admitting an optional quote returns three live `expr` rows in the SHIPPED
# catalog -- .redkiln/catalog/root-cause.toml:62, differential-debugging.toml:65 and
# adversarial-review.toml:41 -- plus the prescribed red fixture
# crates/rk-kernel/tests/fixtures/activities/terminal_condition_is_findings_none.toml:65, whose own
# header says it copies that very spelling from `spikes/p9/catalog/root-cause.toml`:62. Whether an
# `expr` on a `kind = "check"` step carrying `on_false` IS the *terminal condition* ADR-0080 decision 2
# forbids is a reading of the record, and widening this pattern would turn the gate red on the pack
# Step 7e authored. That is changing what the gate asserts, so it is NOT taken here: it is carried to
# the operator as an open question against ADR-0080 and AC-527.
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.2 (D21-41 (h)): the four lines of the unquoted-only guard are replaced
# by every code line of Checkpoint 8b section (4), copied verbatim from s8b's file as landed, from its `EXCL=` line
# through its last synthetic control (the changelog-scope control). s8b's two_walk already skips worktrees/ on its
# .redkiln/ call; its one unexcluded walk is the plant control's `seen=` line, which proves the plant findable. Was:
# ~~raw=$(grep -rnE 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *=' crates/ .redkiln/); rc=$?~~
# ~~[ "$rc" -le 1 ] || { echo "FAIL: the quality-shorthand search could not run (grep rc=$rc; 2 = no such path) -- a count of 0 here would be a fake 0"; exit 1; }~~
# ~~out=$(printf '%s\n' "$raw" | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -v '^crates/rk-kernel/tests/it/evaluator.rs:' | grep -vE '^[[:space:]]*$'); test -z "$out" || { echo "FAIL: a quality-shorthand default or terminal condition:"; printf '%s\n' "$out"; exit 1; }~~
# ~~syn=$(printf 'a.toml:1:expr = "findings.none(evidence in [Reproduced, MutationSurvived])"\n' | grep -E 'findings\.none\(evidence in \[Reproduced, MutationSurvived\]\)|quality_shorthand *=' | grep -vE '^[^:]+:[0-9]+:[[:space:]]*(#|//|/\*|\*)' | grep -v '^crates/rk-kernel/tests/it/evaluator.rs:'); test -n "$syn" || { echo "FAIL: the exclusions swallowed a live terminal condition -- this ban is vacuous"; exit 1; }~~
EXCL='^(crates/rk-kernel/tests/it/(evaluator|quality_block)\.rs|crates/rk-kernel/tests/fixtures/catalog/(root-cause|differential-debugging|instrumented-trace-analysis)\.toml|crates/rk-kernel/tests/fixtures/activities/terminal_condition_(is_findings_none|does_not_parse|names_any_member_list|is_one_member_conjunct)\.toml|\.redkiln/CHANGELOG\.md):'
# ---- Comment lines are dropped by their file's own syntax only: `//`, `/*` and `*` in .rs, `#` in .toml and .yml. A
# ---- Markdown `*` bullet is not a comment, so no .md line escapes by opening with `*` (review M1).
code_only() { grep -vE '^[^:]+\.rs:[0-9]+:[[:space:]]*(//|/\*|\*)' | grep -vE '^[^:]+\.(toml|ya?ml):[0-9]+:[[:space:]]*#'; }
# ---- --exclude-dir matches a base name at ANY depth, and crates/ holds three fixture directories named worktrees. So
# ---- crates/ is walked whole in its own call, and only the .redkiln/ call carries the flag. The .redkiln/ root is the
# ---- walk's 4th argument, so the planted control below runs the same function over a scratch root. Each walk returns
# ---- grep's convention: 0 = a hit, 1 = none, 2 = an error in either call.
wt=$(find .redkiln -path .redkiln/worktrees -prune -o -type d -name worktrees -print); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: find over .redkiln/ could not run (rc=$rc)"; exit 1; }
[ -z "$wt" ] || { echo "FAIL: a directory named worktrees sits inside .redkiln/ besides .redkiln/worktrees — the exclusion would skip it silently: $wt"; exit 1; }
D2='findings\.none\(evidence in \[|quality_shorthand *='
CAL='guarded against false approval|calibrated against false approval'
two_walk() { grep -rn "$1" "$2" "$3"; a=$?; grep -rn --exclude-dir=worktrees "$1" "$2" "$4"; b=$?; [ "$a" -le 1 ] && [ "$b" -le 1 ] || return 2; [ "$a" -eq 0 ] || [ "$b" -eq 0 ] || return 1; }
d2_walk() { two_walk -E "$D2" crates/ "${1:-.redkiln/}"; }
cal_walk() { two_walk -iE "$CAL" crates/rk-kernel/ "${1:-.redkiln/}"; }
# The planted control lives in a SCRATCH tree shaped like .redkiln/ (review M10): an unexcluded walk of the real
# .redkiln/ would descend into run worktrees (junctions, locked files, grep rc 2). The .gitignore row is asserted by
# path; git check-ignore needs no file to exist.
git check-ignore -q .redkiln/worktrees/s8b-plant/plant.toml || { echo "FAIL: D21-44's .redkiln/worktrees/ row is missing from .gitignore"; exit 1; }
SHAPE=target/cp8b/shape/.redkiln
rm -rf target/cp8b/shape && mkdir -p "$SHAPE/catalog" "$SHAPE/worktrees/s8b-plant" || { echo "FAIL: cannot create the shaped scratch tree"; exit 1; }
printf '%s\n' "expr = \"findings.none(evidence in ['Reproduced'])\"" '# guarded against false approval' > "$SHAPE/worktrees/s8b-plant/plant.toml" || { echo "FAIL: cannot write the planted control"; exit 1; }
printf '%s\n' "expr    = \"findings.none(evidence in ['Reproduced'])\"" > "$SHAPE/catalog/live.toml" || { echo "FAIL: cannot write the shaped catalog row"; exit 1; }
seen=$(grep -rlF 'findings.none(evidence in [' "$SHAPE/"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the unexcluded walk of the shaped tree could not run or found nothing (rc=$rc)"; exit 1; }
printf '%s\n' "$seen" | grep -qxF "$SHAPE/worktrees/s8b-plant/plant.toml" || { echo "FAIL: an unexcluded walk does not see the plant — the control proves nothing"; exit 1; }
raw=$(d2_walk "$SHAPE/"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the d2 walk over the shaped tree found nothing or could not run (rc=$rc)"; exit 1; }
printf '%s\n' "$raw" | grep -qF "$SHAPE/catalog/live.toml:" || { echo "FAIL: the d2 walk's .redkiln/ half does not reach catalog/ in the shaped tree"; exit 1; }
leak=$(printf '%s\n' "$raw" | grep -F "$SHAPE/worktrees/")
[ -z "$leak" ] || { echo "FAIL: the d2 walk descended into worktrees/ (D21-44)"; exit 1; }
cal=$(cal_walk "$SHAPE/"); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the calibration walk over the shaped tree could not run (grep rc=$rc)"; exit 1; }
leak=$(printf '%s\n' "$cal" | grep -F "$SHAPE/worktrees/")
[ -z "$leak" ] || { echo "FAIL: the calibration walk descended into worktrees/ (D21-44)"; exit 1; }
rm -rf target/cp8b/shape || { echo "FAIL: cannot remove the shaped scratch tree"; exit 1; }
# Review N10: the real walk's .redkiln/ half reaches .redkiln/catalog, where every activity carries an expr row.
pos=$(two_walk -E '^expr[[:space:]]*=' crates/ .redkiln/); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the positive control walk found nothing or could not run (rc=$rc)"; exit 1; }
printf '%s\n' "$pos" | grep -qE '^\.redkiln/catalog/[a-z0-9-]+\.toml:[0-9]+:expr' || { echo "FAIL: the .redkiln/ half of the walk does not reach .redkiln/catalog — its silence below would be a fake one"; exit 1; }
raw=$(d2_walk); rc=$?
[ "$rc" -le 1 ] || { echo "FAIL: the d2 guard could not run (grep rc=$rc; 2 = no such path) — a silence here would be a fake one"; exit 1; }
printf '%s\n' "$raw" | grep -q '^crates/rk-kernel/tests/it/quality_block\.rs:' || { echo "FAIL: the d2 walk did not reach the enforcement's own literals in crates/ — the crates/ half is not being walked"; exit 1; }
live=$(printf '%s\n' "$raw" | code_only | grep -vE "$EXCL" | grep .)
[ -z "$live" ] || { echo "FAIL: ADR-0080 d2's forbidden form on a live line:"; printf '%s\n' "$live"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/x.toml:5:expr    = \"findings.none(evidence in ['Reproduced'])\"" | grep -E "$D2" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the exclusions swallowed a live one-member row — this guard is vacuous"; exit 1; }
syn=$(printf '%s\n' "crates/rk-kernel/src/quality_block.rs:9:    let p = \"findings.none(evidence in [\";" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the path exclusion reaches src/quality_block.rs — only the TEST file's literals are exempt"; exit 1; }
syn=$(printf '%s\n' ".redkiln/DISPOSITIONS-2026-09-21.md:4:* findings.none(evidence in ['Reproduced'])" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: a Markdown bullet was dropped as a comment — review M1's hole is open"; exit 1; }
syn=$(printf '%s\n' ".redkiln/CHANGELOG.md:9:| root-cause@5 | old: findings.none(evidence in ['Reproduced', 'MutationSurvived']) |" | code_only | grep -vE "$EXCL")
test -z "$syn" || { echo "FAIL: the changelog's quotation of the old text is not excluded — (4) would go red on ADR-0072's own record"; exit 1; }
syn=$(printf '%s\n' ".redkiln/catalog/CHANGELOG.md:9:findings.none(evidence in [" | code_only | grep -vE "$EXCL")
test -n "$syn" || { echo "FAIL: the changelog exclusion reaches beyond .redkiln/CHANGELOG.md"; exit 1; }

# gix does not own the write path — a SCREEN, because ADR-0098 retains one gix write deliberately
test -f crates/rk-git/Cargo.toml || { echo "FAIL: rk-git absent — write-path check NOT RUN (Halt 8)"; exit 1; }
printf '%s\n' "$(grep -rn 'gix' crates/rk-git/Cargo.toml crates/rk-git/src/)"   # → READ every hit: the write path is the exec wrapper (ADR-0077) and the review clone is the ONE retained gix write (ADR-0098); zero hits and review-clone-only are both passes, anything else is not

# the "leaves the model" check this checkpoint promises — widened to leave[sd]?, because METHOD-P §1.4 and ADR-0079 say "leave the model"
grep -rniE 'leave[sd]? the model' crates/ --include=*.rs --include=*.toml --include=*.md; rc=$?; [ "$rc" -eq 1 ] || { echo "FAIL: a crates/ document asserts a facet leaves the model — none does (F39) (rc=$rc)"; exit 1; }

# AC-520: every derived facet names a deriver, and a vacuous 0 0 is not a pass
awk '/^source = "derived"/{d++} /^deriver = /{v++} END{print d+0, v+0; exit !(d>0 && d==v)}' .redkiln/facets.toml || { echo "FAIL: derived facets without a named deriver, or a vacuous 0 0"; exit 1; }   # → 6 6

# no derived boolean resolves an absence of evidence to false — it is null, routed to AC-079
out=$(cargo test -p rk-kernel -- unknown_facet 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: cargo test exited $rc"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE 'test result: ok\. [1-9][0-9]* passed' || { echo "FAIL: the unknown_facet filter matched 0 tests — the tri-valued path is untested"; exit 1; }

# Three SCREENS. Their predicate is a reading, so no hit count is the verdict and none of them may go red on a hit.
test -d references && test -d runbook && test -d spikes || { echo "FAIL: doc tree absent — Checkpoint 8's screens NOT RUN (Halt 8)"; exit 1; }
printf '%s\n' "$(grep -rniE '1 ?h TTL' --include=*.md references/ runbook/ spikes/ | grep -v 'ephemeral_')"   # → every surviving hit READ: none is a per-run TTL report without ephemeral_* evidence (ADR-0078)
printf '%s\n' "$(grep -rniE 'human[- ]panel' --include=*.md references/ runbook/ spikes/ | grep -vE 'panel-adjudicated|not a human|no human|never')"   # → READ: F39's oracle is 60 oracle / 48 unanimous panel / 32 human of 140; most survivors are METHOD-P §1.2's own human agreement panel, a different object
printf '%s\n' "$(grep -rniE 'design (is|as) final' --include=*.md references/ runbook/ spikes/*.md | grep -vE 'may (not|never)|no document|BLOCKED|gates that|not .*final')"   # → READ; spikes/fixtures/ is excluded on purpose (frozen RFC prose, and unscoped this regex returns 173 lines)

# "four questions on twenty items" — a plain grep, because ZERO hits would mean design §10.5's row was deleted
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.3: the line that walked the whole tree is replaced by the lines below. Was:
# ~~grep -rn 'four questions on twenty items' --include=*.md . ; rc=$?; [ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }   # → each hit READ: an oracle-construction measurement, never a forecast~~
# AMENDED at s8j (s8j §8j.8 A5.3; D21-40; D21-44): tracked files only — git grep never reads .git/, target/, a run
# worktree or any other untracked copy. The plant proves the last; the design row proves it reads.
ctl=target/s8-a53; rm -rf "$ctl"; mkdir -p "$ctl" || { echo "FAIL: cannot create $ctl"; exit 1; }
printf '%s\n' 'four questions on twenty items' > "$ctl/stale-copy.md" || { echo "FAIL: cannot write the untracked plant"; exit 1; }
hits=$(git grep -n 'four questions on twenty items' -- '*.md'); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the phrase is gone (rc=$rc) — design §10.5's row may have been deleted"; exit 1; }
printf '%s\n' "$hits" | grep -q '^references/2026-09-08-architecture-design\.md:' || { echo "FAIL: the design row is not among the hits — the screen is not reading the tracked tree"; exit 1; }
if printf '%s\n' "$hits" | grep -qF "$ctl/"; then echo "FAIL: the screen read the untracked plant under target/"; exit 1; fi
rm -rf "$ctl"
printf '%s\n' "$hits"   # → each hit READ: an oracle-construction measurement, never a forecast

# source asymmetry is NOT a predicate. The runnable half is that the record still holds the question open.
grep -q 'Source asymmetry is an arm of the same spike and is settled nowhere' references/2026-09-08-architecture-design.md || { echo "FAIL: the asymmetry-open statement is gone from design §10.5 — a section may have decided it"; exit 1; }

# the --stay case BY PATH — the assertion withdrawn check (5) never made. Modelled on Checkpoint 7b's own
# by-path loop, in `s7b-kernel.md` under `### 7b.14 Checkpoint 7b`, whose opening line reads
#   for f in stay_class_state_is_unconstructible ac480_generics_leak ac050_rigor_is_not_arithmetic
# CITED BY THAT LITERAL AND NOT BY LINE, checkable with `grep -cF` at any later moment. The line range that
# stood here did NOT hold that loop — it resolves to the GRAMMAR-name loop, whose own opening line reads
# `for t in grammar_agrees_with_the_s_grammar_probe` — and `s7b-kernel.md` says of its by-path block that
# "it moves again with this pass". A green trybuild harness is NOT this: `-- trybuild` passes whenever any one of the
# 13 cases in tests/ui/ passes, so the case file itself is named, and its .stderr is asserted non-empty.
test -f crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.rs || { echo "FAIL: the --stay trybuild case is absent — a harness that runs proves nothing about a case that is not there (AC-294; DECISIONS-2026-09-14 D-14)"; exit 1; }
test -s crates/rk-kernel/tests/ui/stay_class_state_is_unconstructible.stderr || { echo "FAIL: the --stay case commits no .stderr, so it is green whenever compilation fails for ANY reason (s7a §7a.8)"; exit 1; }
out=$(cargo test -p rk-kernel --test it -- trybuild_pins_the_reason_each_kernel_fixture_fails 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: the kernel must-fail suite did not pass (rc=$rc) — a fixture COMPILED, or its .stderr drifted"; printf '%s\n' "$out" | tail -20; exit 1; }
printf '%s\n' "$out" | grep -qE '^test .*\btrybuild_pins_the_reason_each_kernel_fixture_fails\b.* \.\.\. ok$' || { echo "FAIL: no test of that name ran and passed — a filter matching nothing exits 0 (00-preamble.md:178)"; printf '%s\n' "$out" | grep -E '^test ' | tail -20; exit 1; }
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* s8j §8j.8 A5.4: two stanzas appended after the block's last line.
# ADR-0164: `* text=auto eol=lf` is .gitattributes' FIRST rule (every -text row below it still wins for its paths).
first=$(grep -vE '^[[:space:]]*(#|$)' .gitattributes | head -1)
[ "$first" = '* text=auto eol=lf' ] || { echo "FAIL: .gitattributes' first rule is '$first', not ADR-0164's row"; exit 1; }
if od -c .gitattributes | grep -qF '\r'; then echo "FAIL: .gitattributes carries a CR byte (ADR-0164 decision 4)"; exit 1; fi
# D21-42 14a: ci.yml green on the pushed branch, every job, both OS legs — read from the recorded run, never re-pushed.
ci_line=$(grep -E '^ci [0-9]+ [0-9a-f]{40} ' runbook/phase-1/s8j-evidence.txt | tail -1)
test -n "$ci_line" || { echo "FAIL: no recorded CI run in runbook/phase-1/s8j-evidence.txt"; exit 1; }
read -r _ run_id ci_sha _ <<< "$ci_line"
git merge-base --is-ancestor "$ci_sha" HEAD || { echo "FAIL: the CI-green sha $ci_sha is not an ancestor of HEAD"; exit 1; }
git diff --quiet "$ci_sha" HEAD -- crates Cargo.toml Cargo.lock rust-toolchain.toml clippy.toml deny.toml tests tools .github .gitattributes .redkiln || { echo "FAIL: code changed since the green CI run $run_id — push (operator go-ahead) and record a new run"; exit 1; }
j=$(gh run view "$run_id" --repo Wet-Ink-Corporation/redkiln-rs --json conclusion,headSha,headBranch,jobs 2>&1); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: gh run view $run_id failed (rc=$rc): $j"; exit 1; }
cichk='import json,sys
d=json.load(sys.stdin)
want={"gates (ubuntu-latest)","gates (windows-latest)","no egress (AC-498)","structural assertions"}
jobs={x["name"]:x.get("conclusion") for x in d.get("jobs",[])}
bad=[]
if d.get("conclusion")!="success": bad.append("run conclusion "+str(d.get("conclusion")))
if d.get("headSha")!=sys.argv[1]: bad.append("headSha "+str(d.get("headSha")))
if d.get("headBranch")!="build/phase-0-to-1": bad.append("headBranch "+str(d.get("headBranch")))
if set(jobs)!=want: bad.append("job set "+repr(sorted(jobs)))
bad+=["job "+n+" "+str(c) for n,c in sorted(jobs.items()) if c!="success"]
print("; ".join(bad) if bad else "ok")
sys.exit(1 if bad else 0)'
v=$(python -c "$cichk" "$ci_sha" <<< "$j"); rc=$?
[ "$rc" -eq 0 ] || { echo "FAIL: CI run $run_id is not green on every job and both legs: $v"; exit 1; }
# *(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* the wrapper set up at the top of this block: its trap is still empty and the fake logged no call.
nd_check "Checkpoint 8's mechanical block" || exit 1
```

*(dated 2026-10-18, s8j build, S8J-D2-S8-AMENDMENTS)* *s8j §8j.8 A9, appended after Checkpoint 8's mechanical block. Prose, not a checkpoint line.*

> **After Checkpoint 8 passes: AC-114's promotion (`s8j` §8j.1(e)).** `tag-release` is promoted out of
> `.redkiln/catalog-extended/`, citing the fix-lane run's id. The promotion's commit also lands the
> `curation.decision` lint, which is `s8j`'s by `s8g` §8g.7's assignment, and it lands only once the operator has taken
> the backfill decision for the 102 shipped activities. That commit is pushed on the operator's go-ahead, and CI is
> made green on it. Checkpoint 8's recorded pass stands for the commit it ran over and is not re-run over this one.

*(dated 2026-10-18, s8j build, package S8J-D3-RECOVERY; issues #61 and #25, audit O-9 option (a))* *Prose, not a
checkpoint line: no block below is fenced, and nothing in it runs without the operator's go-ahead for that act.*

🛑 **DRAFT: AWAITING THE OPERATOR'S APPROVAL OF THIS TEXT BEFORE THE `s8j` FREEZE (#61's second criterion).** Until
the operator approves it by a dated line under it, this section is not a procedure and nothing here may be run. An
agent drafted it. It changes no Checkpoint 8j assertion, no ADR decision sentence and no ledger entry.

*(dated 2026-10-18, D21-163: **approved by the operator** — Ryan's card tap, weigh-in wi-5ac031, recorded by an agent
session — with the three questions under "Open for the operator's approval" answered by the dated lines there. This
section is now the procedure. Each step still waits on its own operator go-ahead, and nothing in it is fenced or a
checkpoint line.)*

> **After a failed first real run of the fix item.** Checkpoint 8j is one-shot (`s8j` §8j.10): its (3) refuses an
> existing `rk-run/<fix item>`, which the first launch creates. This section is the route back when that first run
> fails: (1) launch exits non-zero, or (1)'s check block refuses a run that did launch. D21-40 decision 2 still
> governs the moment of failure: *"stop and report, not spend again"*. Nothing below is a retry. Each step is its
> own operator go-ahead, and the old item's run is archived, never resumed, rebased or repaired in place.
>
> 1. **Stop and record.** `PROGRESS.md` row 8 records the failure, the launch record's basename, the failing line's
>    output, and `git rev-parse rk-run/<old item>`. The run ref is not merged, and the defect is not fixed by hand
>    (the `reserved-fix-item:` entry forbids it).
> 2. **Archive the ref, outside `refs/heads/rk-run/`.** `git update-ref refs/archive/s8-failed-<yyyy-mm-dd>/<old item>
>    rk-run/<old item>` first. Then the run's checkout `.redkiln/worktrees/<old item>` is copied whole into the
>    archive directory of step 3 (`tar -cf <dir>/worktree.tar -C .redkiln/worktrees <old item>`), and only then
>    removed (`git worktree remove --force .redkiln/worktrees/<old item>`, then `git branch -D rk-run/<old item>`).
>    The commits stay reachable from the archive ref. `git worktree prune --dry-run -v` must then print nothing, as
>    Checkpoint 8j (12) asserts.
> 3. **Archive the store, and leave it in place.** A copy of `$(git rev-parse --git-common-dir)/redkiln/store.sqlite3`
>    is taken with SQLite's own backup API (`sqlite3.connect(src).backup(sqlite3.connect(dst))`, which carries the
>    WAL; a plain `cp` of a WAL-mode file can drop it) into
>    `$(git rev-parse --git-common-dir)/redkiln/archive/<yyyy-mm-dd>-<old item>/store.sqlite3`, with a
>    `counts.txt` of Checkpoint 8j (0)'s `rows` reading over both copies, which must agree. The live store is not
>    deleted, truncated or edited: Checkpoint 7i and `s8`'s (2) read it, and the old item's rows are keyed to the
>    old id, which the fresh item does not share.
> 4. **Mint a fresh item by `s8j` §8j.5's route, steps 1-5 of "Reservation and mint".** The operator picks the
>    defect: the same one under a new id, or another presented candidate. Its new `reserved-fix-item: <new id>
>    <crate>:<function>` line is a dated ledger entry in the operator's voice, never an agent's. The new
>    `.redkiln/items/<new id>.toml`, its BugReport and its repro pair are committed together. The old item's file
>    gains one dated `#` line naming the archive ref and directory; its other lines do not change.
> 5. **Re-freeze.** The mint changes `.redkiln/`, which is in `CODE_PATHS`, so `prespend()` refuses the next launch
>    until the frozen records are re-recorded over the new commit: a `fixture-proofs` line (`s8j` §8j.6 step 1b; no
>    spend), CI green on the pushed head with a new `ci` line (§8j.7), and a new `fix-item <new id> <40-hex frozen
>    sha>` line, appended so that `tail -1` reads it. If the failure needed a repair under any other `CODE_PATHS`
>    entry, that repair re-opens step 1b and the paid steps (`s8j` §8j.10's stage close), and this section does not
>    shorten that.
> 6. **Re-run Checkpoint 8j's (3) and (12) against the fresh item.** They are run from the block as it stands, with
>    its (0), because (12) compares against (0)'s readings, under the same `nodispatch` wrapper, from a plain
>    terminal. (3) then reads the new `fix-item` line, the new reservation, and the absence of `rk-run/<new id>`
>    and `.redkiln/worktrees/<new id>`. (12) shows that the re-run moved nothing. The pass is recorded in
>    `PROGRESS.md` row 8j as a dated line naming the archive ref. `s8`'s (1) then launches `<new id>` under a new
>    `S8_GO`.
>
> **Open for the operator's approval** (each one decides a line above):
> - **Whether the live store stays in place** (step 3) or is replaced by a fresh one. Replacing it would move
>   Checkpoint 7i's shipped-store readings and `s8`'s (2).
>   *(dated 2026-10-18, D21-163: answered — the live store stays in place, as step 3 reads.)*
> - **Whether `s8`'s (2) treats `refs/archive/` as reachable.** A2/A8's projection reads rows whose anchor is
>   reachable from the refs, and the old run's rows anchor on archived commits.
>   *(dated 2026-10-18, D21-163: answered — yes, `refs/archive/` counts as reachable. The tier-1 read is
>   `git log --all` (`crates/rk-git/src/reconcile.rs`:278, `read_commits`, whose `--all` is at :302), so an archive
>   ref's commits are read like any other ref's.)*
> - **Whether a failed story-item run (1b) follows the same route.** Checkpoint 8j (3) does not read
>   `rk-run/rk-story-harden-spike-pack`, and no fresh story item is minted by `s8j` §8j.5.
>   *(dated 2026-10-18, D21-163: answered — a failed story-item run (1b) takes steps 1-3 only (stop and record,
>   archive the ref, archive the store) and then stops. Any re-run of it is a separate operator decision.)*

---
