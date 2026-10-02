#!/bin/sh
# tools/live-run.sh — the only entry to a real run (runbook s8h §8h.7; D21-42 17a, 17b; D21-46; D21-47).
#
# Committed, LF (ADR-0164), POSIX sh for Git Bash. Run it from a PLAIN terminal, never from a Claude Code session:
# a session exports CLAUDECODE and CLAUDE_CODE_*, and step 2 REFUSES them (it never strips them).
#
#   tools/live-run.sh run <item> [--lane fix|story]
#   tools/live-run.sh probe live::<test>
#   tools/live-run.sh wrap -- <argv...>
#   tools/live-run.sh answer <item> <option> [<flag>...]
#   tools/live-run.sh --check [<mode> [<its arguments>]]   # every refusal and capture; spawns no child, writes no record
#
# Captures go to stdout, one KEY=value per line: MODE, CLI_VERSION, CLAUDE_CONFIG_DIR, DISABLE_AUTOUPDATER, and
# LAUNCH_CHECK=ok once every refusal has passed. Every refusal is one stderr line of the fixed form
#   live-run.sh: refused: <NAME> — <reason>
# and step 9's failure is `live-run.sh: failed: CLI_MOVED — Halt 2`. Checkpoint 8h (10) reads these anchored forms.
#
# Steps (s8h §8h.7's table): 1 platform; 2 session names; 3 the committed denylist, case-insensitively, read by
# anchored extraction from crates/rk-adapter-claude/src/env_denylist.rs; 4 CLAUDE_CONFIG_DIR SET (never inherited);
# 5 DISABLE_AUTOUPDATER=1; 6 the resolved claude, its version and sha256 (run, probe and answer compare the version to
# the pin, naming Halt 2; wrap records it and never compares); 7 the child; 8 the launch record under
# <git-common-dir>/redkiln/launches/, created without clobbering; 9 the version and sha256 again, a difference failing
# the launch. The launch record is a guard against accident, not a security boundary: REDKILN_LAUNCH_RECORD can be
# forged, and that is stated (s8h §8h.7).

me=live-run.sh

refuse() {  # refuse <NAME> <reason...>
  name=$1
  shift
  printf '%s: refused: %s — %s\n' "$me" "$name" "$*" >&2
  exit 2
}

usage() {
  sed -n '7,11p' "$0" >&2
}

# The repository whose launcher this is: the script's own tree, so the pin and the denylist are the ones committed
# beside it, wherever the operator's working directory is.
root=$(cd "$(dirname "$0")/.." && pwd) || refuse LAUNCHER_ROOT "cannot resolve the launcher's repository from $0"

# ---- Arguments (before step 1: answer's ANSWER_USAGE, wrap's WRAP_ARGV and the others' usage refusals).
check=0
if [ "${1:-}" = --check ]; then
  check=1
  shift
fi
if [ "$#" -eq 0 ]; then
  if [ "$check" -eq 1 ]; then
    mode=run
  else
    usage
    refuse USAGE "no mode: run, probe, wrap, answer or --check"
  fi
else
  mode=$1
  shift
fi

screen_wrap_argv() {  # wrap's argv screen (D21-47; s8h review #16): words only, a guard against accident
  prevb=""
  for w in "$@"; do
    b=${w##*/}
    b=${b##*\\}
    case "$prevb" in
      redkiln|redkiln.exe)
        case "$w" in
          run|answer) refuse WRAP_ARGV "'$prevb $w' has its own mode (tools/live-run.sh $w), which compares the pin (s8h §8h.7)" ;;
        esac
        ;;
    esac
    prevb=$b
  done
  first=${1##*/}
  first=${first##*\\}
  case "$first" in
    cargo|cargo.exe) : ;;
    *) return 0 ;;
  esac
  ign=0
  live=0
  feat_next=0
  for w in "$@"; do
    if [ "$feat_next" -eq 1 ]; then
      feat_next=0
      for f in $(printf '%s\n' "$w" | tr ', ' '\n\n'); do
        case "$f" in live|*/live) live=1 ;; esac
      done
      continue
    fi
    case "$w" in
      --ignored|--include-ignored) ign=1 ;;
      --all-features) live=1 ;;
      --features|-F) feat_next=1 ;;
      --features=*|-F*)
        fl=${w#--features=}
        fl=${fl#-F}
        for f in $(printf '%s\n' "$fl" | tr ', ' '\n\n'); do
          case "$f" in live|*/live) live=1 ;; esac
        done
        ;;
    esac
  done
  if [ "$ign" -eq 1 ] && [ "$live" -eq 1 ]; then
    refuse WRAP_ARGV "a cargo argv carrying --ignored with the live feature is a live probe; it has its own mode (tools/live-run.sh probe live::<name>), which compares the pin (s8h §8h.7)"
  fi
  return 0
}

case "$mode" in
  run)
    if [ "$check" -eq 0 ] || [ "$#" -gt 0 ]; then
      [ "$#" -ge 1 ] || refuse RUN_USAGE "usage: tools/live-run.sh run <item> [--lane fix|story]"
      case "$1" in -*|"") refuse RUN_USAGE "the item '$1' is absent or begins with '-'; usage: tools/live-run.sh run <item> [--lane fix|story]" ;; esac
      if [ "$#" -ne 1 ]; then
        [ "$#" -eq 3 ] && [ "$2" = --lane ] || refuse RUN_USAGE "usage: tools/live-run.sh run <item> [--lane fix|story]"
        case "$3" in fix|story) : ;; *) refuse RUN_USAGE "--lane takes fix or story, not '$3'" ;; esac
      fi
    fi
    ;;
  probe)
    if [ "$check" -eq 0 ] || [ "$#" -gt 0 ]; then
      [ "$#" -eq 1 ] || refuse PROBE_USAGE "usage: tools/live-run.sh probe live::<test> (one module-qualified live:: name)"
      case "$1" in
        live::*) : ;;
        *) refuse PROBE_USAGE "'$1' is not a module-qualified live:: name" ;;
      esac
      case "${1#live::}" in
        ""|*[!A-Za-z0-9_]*) refuse PROBE_USAGE "'$1' is not a module-qualified live:: name" ;;
      esac
    fi
    ;;
  wrap)
    [ "${1:-}" = -- ] || refuse WRAP_ARGV "usage: tools/live-run.sh wrap -- <argv...> (the argv follows a literal --)"
    shift
    [ "$#" -ge 1 ] || refuse WRAP_ARGV "no argv after --"
    screen_wrap_argv "$@"
    ;;
  answer)
    [ "$#" -ge 1 ] || refuse ANSWER_USAGE "no <item>; usage: tools/live-run.sh answer <item> <option> [<flag>...]"
    case "$1" in -*|"") refuse ANSWER_USAGE "the item '$1' begins with '-' or is empty; usage: tools/live-run.sh answer <item> <option> [<flag>...]" ;; esac
    [ "$#" -ge 2 ] || refuse ANSWER_USAGE "no <option> after the item '$1'; usage: tools/live-run.sh answer <item> <option> [<flag>...]"
    ;;
  *)
    usage
    refuse USAGE "unknown mode '$mode': run, probe, wrap, answer or --check"
    ;;
esac

# ---- Step 1: the platform.
case "$(uname -s)" in
  MINGW*|MSYS*) : ;;
  *) refuse PLATFORM "uname -s is '$(uname -s)'; the launcher runs on the Windows box under Git Bash (s8h §8h.7 step 1)" ;;
esac

# The environment's names, one per line, each with its upper-cased spelling (Windows names are case-insensitive).
names=$(env | sed -n 's/^\([A-Za-z_][A-Za-z0-9_]*\)=.*/\1/p' | awk 'NF { print toupper($0) " " $0 }')

# ---- Step 2: the session names (s7c §7c.15's reason). Compared upper-cased, the fail-closed direction.
while read -r up orig; do
  [ -n "$up" ] || continue
  case "$up" in
    CLAUDECODE|CLAUDE_PID|CLAUDE_EFFORT|AI_AGENT)
      refuse "$orig" "a Claude Code session exported it; run the launcher from a plain terminal (s7c §7c.15; s8h §8h.7 step 2)" ;;
  esac
  case "$up" in
    CLAUDE_CODE_*)
      refuse "$orig" "a CLAUDE_CODE_* name: a Claude Code session exported it; run the launcher from a plain terminal (s7c §7c.15; s8h §8h.7 step 2)" ;;
  esac
done <<EOF
$names
EOF

# ---- Step 3: the committed denylist, read by ANCHORED extraction from the ONE list (never copied here).
dl="$root/crates/rk-adapter-claude/src/env_denylist.rs"
[ -f "$dl" ] || refuse DENYLIST "$dl is absent; the launcher reads the committed list and has no copy (s8h §8h.7 step 3)"
extract_list() {  # extract_list <anchor>: the quoted names from the anchor line through the first line closing with ];
  awk -v a="$1" 'index($0, a) == 1 { on = 1 } on { print } on && /\];/ { exit }' "$dl" | grep -oE '"[A-Za-z0-9_]+"' | tr -d '"'
}
a_exact='pub const DENYLIST: &[&str] = &['
a_prefix='pub const DENYLIST_PREFIXES: &[&str] = &['
for a in "$a_exact" "$a_prefix"; do
  n=$(awk -v a="$a" 'index($0, a) == 1 { n++ } END { print n + 0 }' "$dl")
  [ "$n" -eq 1 ] || refuse DENYLIST "the anchor '$a' matched $n lines of $dl, expected exactly 1"
done
exact=$(extract_list "$a_exact")
prefixes=$(extract_list "$a_prefix")
[ -n "$exact" ] || refuse DENYLIST "the exact-name list extracted from $dl is empty"
[ -n "$prefixes" ] || refuse DENYLIST "the prefix list extracted from $dl is empty"
while read -r up orig; do
  [ -n "$up" ] || continue
  for d in $exact; do
    [ "$up" = "$d" ] && refuse "$orig" "on the committed denylist as $d, compared case-insensitively ($dl); unset it outside the process, never strip it (s7c §7c.6)"
  done
  for p in $prefixes; do
    case "$up" in
      "$p"*) refuse "$orig" "under the committed denylist prefix $p, compared case-insensitively ($dl) (s7c §7c.6)" ;;
    esac
  done
done <<EOF
$names
EOF

# ---- Step 4: CLAUDE_CONFIG_DIR is SET, never inherited (D21-42 17b). An existence test only: no content is read.
[ -n "${HOME:-}" ] || refuse HOME "HOME is unset; the account directory is \$HOME/.claude-personal (D21-42 17b)"
cfgdir="$HOME/.claude-personal"
CLAUDE_CONFIG_DIR=$(cygpath -w "$cfgdir")
export CLAUDE_CONFIG_DIR
[ -d "$cfgdir" ] || refuse CLAUDE_CONFIG_DIR "the account directory $cfgdir is absent (D21-42 17b)"
[ -f "$cfgdir/.credentials.json" ] || refuse CLAUDE_CONFIG_DIR "$cfgdir/.credentials.json is absent; the subscription session is not configured there (D21-42 17b)"

# ---- Step 5: the auto-update freeze reaches the child (s8h §8h.8), overwriting any inherited value.
DISABLE_AUTOUPDATER=1
export DISABLE_AUTOUPDATER

# ---- Step 6: the resolved claude, its version and its sha256.
read_pin() {
  p=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9.]+)";$/\1/p' "$root/crates/rk-adapter-claude/src/config.rs" 2>/dev/null)
  [ -n "$p" ] && [ "$(printf '%s\n' "$p" | grep -c .)" -eq 1 ] || refuse TESTED_CLI_VERSION "the pin's anchored form did not match exactly once in crates/rk-adapter-claude/src/config.rs (s7c §7c.21)"
  printf '%s\n' "$p"
}
read_instrument() {
  i=$(sed -nE 's/^([0-9.]+) \(Claude Code\)$/\1/p' "$root/spikes/results/instrument-checks/RUNBOOK-instrument.txt" 2>/dev/null)
  [ -n "$i" ] || refuse CLI_VERSION "spikes/results/instrument-checks/RUNBOOK-instrument.txt holds no '<v> (Claude Code)' line: the pinned instrument is unreadable (Halt 2)"
  printf '%s\n' "$i"
}
resolve_claude() {  # prints the resolved path, or nothing
  hash -r 2>/dev/null
  c=$(command -v claude 2>/dev/null) || return 0
  if [ ! -f "$c" ] && [ -f "$c.exe" ]; then
    c="$c.exe"
  fi
  printf '%s\n' "$c"
}
read_version() {  # read_version <path>: the bare version, or nothing
  "$1" --version 2>/dev/null | tr -d '\r' | sed -n 's/^\([0-9][0-9.]*\) (Claude Code)$/\1/p' | head -n 1
}
read_sha() {
  sha256sum "$1" 2>/dev/null | cut -d' ' -f1
}

pin=$(read_pin) || exit 2
cpath=$(resolve_claude)
[ -n "$cpath" ] || refuse CLI_VERSION "no claude on PATH: there is no CLI to read (Halt 2)"
cver=$(read_version "$cpath")
[ -n "$cver" ] || refuse CLI_VERSION "$cpath --version printed no '<v> (Claude Code)' line (Halt 2)"
csha=$(read_sha "$cpath")
[ -n "$csha" ] || refuse CLI_VERSION "cannot read the sha256 of $cpath"

printf 'MODE=%s\n' "$mode"
printf 'CLI_VERSION=%s\n' "$cver"
printf 'CLAUDE_CONFIG_DIR=%s\n' "$CLAUDE_CONFIG_DIR"
printf 'DISABLE_AUTOUPDATER=%s\n' "$DISABLE_AUTOUPDATER"

case "$mode" in
  run|probe|answer)
    inst=$(read_instrument) || exit 2
    if [ "$cver" != "$pin" ] || [ "$cver" != "$inst" ]; then
      refuse CLI_VERSION "claude --version reports $cver; the adapter pin TESTED_CLI_VERSION is $pin and RUNBOOK-instrument.txt records $inst: Halt 2 (s8h §8h.7 step 6; the pin moves only at s8j, §8h.8)"
    fi
    ;;
esac
# wrap records the version above and never compares it to the pin (s8h §8h.7: a wrap child that dispatches through
# rk-adapter-claude is pinned by the adapter's own MismatchPolicy::Refuse, never by the launcher).

printf 'LAUNCH_CHECK=ok\n'
if [ "$check" -eq 1 ]; then
  exit 0
fi

# ---- Step 8's location, read in the launcher's own working directory BEFORE the child starts.
gcd=$(git rev-parse --path-format=absolute --git-common-dir 2>/dev/null)
[ -n "$gcd" ] || refuse GIT_COMMON_DIR "the working directory is not inside a git repository; the launch record lives under <git-common-dir>/redkiln/launches/"
head_sha=$(git rev-parse HEAD 2>/dev/null)
[ -n "$head_sha" ] || refuse HEAD "git rev-parse HEAD failed in $(pwd)"

# ---- Step 7, its builds first (Halt 1): nothing runs a binary this did not just build.
case "$mode" in
  run|answer)
    cd "$root" || refuse LAUNCHER_ROOT "cannot enter $root"
    cargo build --workspace || {
      printf '%s: failed: BUILD — cargo build --workspace failed (Halt 1)\n' "$me" >&2
      exit 1
    }
    ;;
  probe)
    cd "$root" || refuse LAUNCHER_ROOT "cannot enter $root"
    ;;
esac

# ---- Step 8: the launch record, created without clobbering.
ldir="$gcd/redkiln/launches"
mkdir -p "$ldir" || refuse LAUNCH_RECORD "cannot create $ldir"
stamp=$(date -u +%Y%m%dT%H%M%S.%NZ)
rec="$ldir/$stamp-$mode.txt"
argv_line=""
for w in "$@"; do
  w1=$(printf '%s' "$w" | tr '\n\r' '  ')
  argv_line="$argv_line${argv_line:+ }$w1"
done
set -C
{
  printf 'MODE=%s\n' "$mode"
  printf 'ARGV=%s\n' "$argv_line"
  case "$mode" in run|answer) printf 'ITEM=%s\n' "$1" ;; esac
  printf 'CLAUDE_PATH=%s\n' "$cpath"
  printf 'CLI_VERSION_BEFORE=%s\n' "$cver"
  printf 'CLI_SHA256_BEFORE=%s\n' "$csha"
  printf 'CLAUDE_CONFIG_DIR=%s\n' "$CLAUDE_CONFIG_DIR"
  printf 'DISABLE_AUTOUPDATER=%s\n' "$DISABLE_AUTOUPDATER"
  printf 'PIN=%s\n' "$pin"
  printf 'HEAD=%s\n' "$head_sha"
} > "$rec" || refuse LAUNCH_RECORD "$rec already exists or cannot be created (set -C: a collision refuses)"
set +C
REDKILN_LAUNCH_RECORD=$(cygpath -m "$rec")
export REDKILN_LAUNCH_RECORD
printf 'LAUNCH_RECORD=%s\n' "$REDKILN_LAUNCH_RECORD"

# ---- Step 7: the child, in the foreground with the launcher's own stdin, stdout and stderr (wrap, run, answer).
case "$mode" in
  wrap)
    "$@"
    rc=$?
    ;;
  run)
    ./target/debug/redkiln run "$@"
    rc=$?
    ;;
  answer)
    ./target/debug/redkiln answer "$@"
    rc=$?
    ;;
  probe)
    # The probe's output is kept beside its record, then printed: the selection is asserted on it, because a filter
    # that matches nothing is a green (s8h §8h.7 step 7).
    pout="$ldir/$stamp-probe.output.txt"
    cargo test -p rk-adapter-claude --features live --test it -- --ignored --nocapture "$1" --exact > "$pout" 2>&1
    rc=$?
    cat "$pout"
    if [ "$rc" -eq 0 ]; then
      if ! grep -qxF "test $1 ... ok" "$pout" || ! grep -qE '^test result: ok\. 1 passed;' "$pout"; then
        printf '%s: failed: PROBE_SELECTION — the run did not report `test %s ... ok` and `1 passed`\n' "$me" "$1" >&2
        rc=1
      fi
    fi
    ;;
esac

# ---- Step 9: the version and sha256 again; a difference fails the launch (Halt 2).
cpath2=$(resolve_claude)
cver2=""
csha2=""
if [ -n "$cpath2" ]; then
  cver2=$(read_version "$cpath2")
  csha2=$(read_sha "$cpath2")
fi
{
  printf 'CLAUDE_PATH_AFTER=%s\n' "$cpath2"
  printf 'CLI_VERSION_AFTER=%s\n' "$cver2"
  printf 'CLI_SHA256_AFTER=%s\n' "$csha2"
  printf 'RC=%s\n' "$rc"
} >> "$rec"
printf 'CLI_VERSION_AFTER=%s\n' "$cver2"
printf 'RC=%s\n' "$rc"
if [ "$cpath2" != "$cpath" ] || [ "$cver2" != "$cver" ] || [ "$csha2" != "$csha" ]; then
  printf '%s: failed: CLI_MOVED — Halt 2: claude was %s %s (%s) before the child and is %s %s (%s) after; the launch spans a version boundary (s8h §8h.7 step 9)\n' "$me" "$cpath" "$cver" "$csha" "$cpath2" "$cver2" "$csha2" >&2
  exit 3
fi
exit "$rc"
