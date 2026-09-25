# runbook/tools/nodispatch.sh — the ONE no-dispatch wrapper for s8 checkpoints and their replays (D21-47).
#
# Source it from the repo root, then call its functions; each prints FAIL and returns 1 on a failed control, so write
# `nd_setup "$PWD/target/<stage>/nd" || exit 1`. It gives:
#   - a COMPILED fake `claude` (runbook/tools/fake-claude.rs) first on PATH, printing the adapter pin for `--version`
#     and logging, then refusing with 97, anything else;
#   - every OTHER PATH directory holding a claude, claude.exe or claude.cmd removed from the wrapper's PATH, so a
#     process that skips the first entry still finds no real binary (s8i review M7);
#   - CLAUDE_CONFIG_DIR at an empty trap directory, which must still be empty afterwards;
#   - REDKILN_LAUNCH_RECORD unset, so s8h's launch check refuses before any spawn.
#
# Functions:
#   nd_setup <posix dir>  build the fake and the trap, run every control. Inside an enclosing wrapper
#                         (ND_ACTIVE=1) it verifies and reuses that wrapper instead, so a nested whole replay keeps
#                         the outer ND_REAL_CLAUDE.
#   nodispatch <cmd...>   run one command under the wrapper (the scoped form, s8i).
#   nd_enter              export the wrapper into this shell (the global form, s8h).
#   nd_check <label>      the trap is empty and the fake's call log is absent.
# After nd_setup, ND_REAL_CLAUDE is the claude PATH resolved BEFORE the wrapper, or empty. Only a live reading that a
# stage file names may use it.
#
# Platform rules measured in s8, kept here so no copy re-learns them:
#   - PATH entries are POSIX (/d/...). A `C:/` entry splits on its colon and leaves the real binary reachable
#     (s8d-ii's slip). nd_setup refuses a directory argument containing a colon.
#   - Python given a Git Bash absolute path inside `python -c` reads /c/Users/... as D:/c/Users/... (s8b). Hand
#     Python relative target/... paths or `cygpath -m` output.
#   - CLAUDE_CONFIG_DIR is handed to Windows processes in `cygpath -m` form.
#
# A change to this file changes every checkpoint that sources it: it lands with a dated note in each stage file that
# names the change, and those checkpoints are re-run whole.

nd_setup() {
  local dir="$1" pin src exe out rc d v n
  case "$dir" in
    /*) : ;;
    *) echo "FAIL: nd_setup needs an absolute POSIX directory, got '$dir'"; return 1 ;;
  esac
  case "$dir" in *:*) echo "FAIL: nd_setup's directory '$dir' contains a colon — it would split PATH"; return 1 ;; esac
  if [ "${ND_ACTIVE:-}" = 1 ]; then
    [ -n "${ND_BIN:-}" ] && [ -n "${ND_TRAP:-}" ] || { echo "FAIL: ND_ACTIVE is set without ND_BIN/ND_TRAP"; return 1; }
    nd_is_fake "$(command -v claude)" || { echo "FAIL: an enclosing wrapper is declared but claude resolves to '$(command -v claude)'"; return 1; }
    [ -z "${REDKILN_LAUNCH_RECORD:-}" ] || { echo "FAIL: REDKILN_LAUNCH_RECORD is set inside the enclosing wrapper"; return 1; }
    nd_check "reuse" || return 1
    echo "nodispatch: reusing the enclosing wrapper at $ND_BIN"
    return 0
  fi
  pin=$(sed -nE 's/^pub const TESTED_CLI_VERSION: &str = "([0-9.]+)";$/\1/p' crates/rk-adapter-claude/src/config.rs)
  printf '%s\n' "$pin" | grep -qxE '[0-9]+[.][0-9]+[.][0-9]+' || { echo "FAIL: the adapter pin's anchored form did not match: '$pin'"; return 1; }
  src="runbook/tools/fake-claude.rs"
  test -f "$src" || { echo "FAIL: $src not found — source nodispatch.sh from the repo root"; return 1; }
  case "$(uname -s)" in MINGW*|MSYS*|CYGWIN*) ND_EXE=".exe" ;; *) ND_EXE="" ;; esac
  ND_REAL_CLAUDE=$(command -v claude 2>/dev/null || true)
  ND_BIN="$dir/bin"; ND_TRAP="$dir/trap"; ND_LOG="$dir/calls.log"
  rm -rf "$dir" && mkdir -p "$ND_BIN" "$ND_TRAP" || { echo "FAIL: could not create $dir"; return 1; }
  if [ -n "$ND_EXE" ]; then
    ND_LOG_NATIVE=$(cygpath -m "$ND_LOG") && ND_TRAP_NATIVE=$(cygpath -m "$ND_TRAP") || { echo "FAIL: cygpath could not convert the wrapper's paths"; return 1; }
  else
    ND_LOG_NATIVE="$ND_LOG"; ND_TRAP_NATIVE="$ND_TRAP"
  fi
  sed -e "s|RK_PIN|$pin|" -e "s|RK_LOG|$ND_LOG_NATIVE|" "$src" > "$dir/fake_claude.rs" || { echo "FAIL: could not fill the fake's source"; return 1; }
  [ "$(grep -c -e 'RK_PIN' -e 'RK_LOG' "$dir/fake_claude.rs")" -eq 0 ] || { echo "FAIL: a placeholder survived in the fake's source"; return 1; }
  out=$(rustc -O -o "$ND_BIN/claude$ND_EXE" "$dir/fake_claude.rs" 2>&1); rc=$?
  [ "$rc" -eq 0 ] && [ -f "$ND_BIN/claude$ND_EXE" ] || { echo "FAIL: the fake claude did not compile (rc=$rc)"; printf '%s\n' "$out" | tail -10; return 1; }
  ND_PATH="$ND_BIN"
  local IFS=:
  for d in $PATH; do
    [ -n "$d" ] || continue
    if [ -e "$d/claude" ] || [ -e "$d/claude.exe" ] || [ -e "$d/claude.cmd" ]; then continue; fi
    ND_PATH="$ND_PATH:$d"
  done
  unset IFS
  # Controls. Resolution is checked BEFORE the first call, so a failed wrapper never reaches a real binary.
  nd_is_fake "$(PATH="$ND_PATH" command -v claude)" || { echo "FAIL: CONTROL — the fake is not the claude the wrapper's PATH resolves"; return 1; }
  for d in claude claude.exe claude.cmd; do
    v=$(PATH="$ND_PATH" type -a -p "$d" 2>/dev/null | grep -v -x -F -e "$ND_BIN/claude" -e "$ND_BIN/claude$ND_EXE" | head -1)
    [ -z "$v" ] || { echo "FAIL: CONTROL — a real $d is still reachable on the wrapper's PATH: $v"; return 1; }
  done
  v=$(nodispatch claude --version); rc=$?
  [ "$rc" -eq 0 ] && [ "$v" = "$pin (Claude Code)" ] || { echo "FAIL: CONTROL — the fake did not print the pin (rc=$rc, '$v')"; return 1; }
  nodispatch claude -p control >/dev/null 2>&1; rc=$?
  [ "$rc" -eq 97 ] && [ "$(grep -c . "$ND_LOG" 2>/dev/null)" = 1 ] || { echo "FAIL: CONTROL — the fake did not refuse and log a non-version call (rc=$rc)"; return 1; }
  rm -f "$ND_LOG" || { echo "FAIL: could not clear the fake's control entry"; return 1; }
  : > "$ND_TRAP/control" || { echo "FAIL: could not write the trap's control file"; return 1; }
  n=$(find "$ND_TRAP" -mindepth 1 | grep -c .)
  [ "$n" -eq 1 ] || { echo "FAIL: CONTROL — the trap's emptiness reading saw $n entries where one was written"; return 1; }
  rm -f "$ND_TRAP/control" || { echo "FAIL: could not clear the trap's control file"; return 1; }
  nd_check "start" || return 1
  echo "nodispatch: fake claude $pin at $ND_BIN; trap $ND_TRAP_NATIVE; real claude before the wrapper: ${ND_REAL_CLAUDE:-none}"
}

# Git Bash reports a PATH hit on `claude.exe` as .../claude when asked for `claude`, and as .../claude.exe when asked
# for `claude.exe`: both spellings are the fake.
nd_is_fake() {
  [ "$1" = "$ND_BIN/claude" ] || [ "$1" = "$ND_BIN/claude$ND_EXE" ]
}

nodispatch() {
  [ -n "${ND_PATH:-}" ] || { echo "FAIL: nodispatch called before nd_setup"; return 1; }
  env -u REDKILN_LAUNCH_RECORD ND_ACTIVE=1 ND_BIN="$ND_BIN" ND_TRAP="$ND_TRAP" ND_LOG="$ND_LOG" ND_EXE="$ND_EXE" \
    ND_PATH="$ND_PATH" ND_TRAP_NATIVE="$ND_TRAP_NATIVE" ND_REAL_CLAUDE="$ND_REAL_CLAUDE" \
    PATH="$ND_PATH" CLAUDE_CONFIG_DIR="$ND_TRAP_NATIVE" "$@"
}

nd_enter() {
  [ -n "${ND_PATH:-}" ] || { echo "FAIL: nd_enter called before nd_setup"; return 1; }
  unset REDKILN_LAUNCH_RECORD
  PATH="$ND_PATH"; CLAUDE_CONFIG_DIR="$ND_TRAP_NATIVE"; ND_ACTIVE=1
  export PATH CLAUDE_CONFIG_DIR ND_ACTIVE ND_BIN ND_TRAP ND_LOG ND_EXE ND_PATH ND_TRAP_NATIVE ND_REAL_CLAUDE
}

nd_check() {
  local left
  left=$(find "$ND_TRAP" -mindepth 1 2>&1) || { echo "FAIL: nd_check ($1) could not read the trap $ND_TRAP"; return 1; }
  [ -z "$left" ] || { echo "FAIL: nd_check ($1): something wrote into the trap — a real claude ran:"; printf '%s\n' "$left" | head -5; return 1; }
  [ ! -e "$ND_LOG" ] || { echo "FAIL: nd_check ($1): the fake claude was called:"; head -5 "$ND_LOG"; return 1; }
}
