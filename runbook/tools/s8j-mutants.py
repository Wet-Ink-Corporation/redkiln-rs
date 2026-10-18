#!/usr/bin/env python3
"""s8j's mutant driver (§8j.2-§8j.4; build plan package S8J-C5-MUTANT-DRIVER). Stdlib only.

*(dated 2026-10-18, s8j build, S8J-C5-MUTANT-DRIVER.)* §8j.4: *"Each is run from Python, never the shell. Each must
flip the named test red, and a survivor blocks."* §8j.6 step 1b reads this driver's results file,
`target/s8j-mutants/results.tsv` under the repository root, one line per mutant:

    <id><TAB>killed|SURVIVED<TAB><HEAD sha>

Step 1b's reader takes the LAST line per (id, HEAD), so the file is appended to and a re-run at the same HEAD
supersedes an earlier one.

How it runs (from the repository root, in the operator's Git Bash, inside step 1b's no-dispatch wrapper):

    . runbook/tools/nodispatch.sh && nd_setup "$PWD/target/s8j-mutants/nd" || exit 1
    nodispatch python runbook/tools/s8j-mutants.py
    nd_check "the s8j mutants" || exit 1

Other modes: `--self-test` (the synthetic controls, no cargo), `--list`, `--check-anchors <rev>` (every anchor
asserted once against `git show <rev>:<file>`, no tree touched), `--only <id>` (repeatable).

The discipline, for every mutant:
- every anchor is asserted EXACTLY ONCE in its file before anything is mutated; a missing or repeated anchor is a
  hard error, never a skip. A file checked out CRLF (`core.autocrlf=true`) is matched with CRLF anchors;
- the pre-mutation bytes are kept in memory and in `target/s8j-mutants/backup/`, the mutation is written through a
  temporary file and `os.replace`, and the restore is the same; the restored file's sha256 must equal the
  pre-mutation sha256, or the driver aborts loudly (exit 3) naming the backup;
- the restore runs in `finally`, so it runs on an exception, on Ctrl-C (SIGINT), and on SIGTERM / SIGBREAK / SIGHUP,
  which are turned into KeyboardInterrupt; a second Ctrl-C during the restore is ignored until it is done;
- every named test must be reported, by name, as `ok` on the unmutated tree first (the baseline). A test that is
  red, absent or unreported before the mutation would make a kill vacuous, so that refuses the whole run;
- a mutant is `killed` only when EVERY test it names is reported `FAILED`; otherwise it is `SURVIVED`. A run with no
  `test result:` line (a mutant that does not compile), a named test absent from the output, or a timeout is a hard
  error: nothing is recorded for that mutant and the driver stops.

Exit codes: 0 every selected mutant killed; 1 at least one SURVIVED (a survivor blocks); 2 refused or hard error;
3 a restore did not verify (the tree is NOT what HEAD holds: restore from the named backup by hand).
"""
import argparse
import hashlib
import os
import re
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

TBD = "TBD-after-C4"
RESULTS_REL = "target/s8j-mutants/results.tsv"
WORK_REL = "target/s8j-mutants"
EVIDENCE_REL = "runbook/phase-1/s8j-evidence.txt"
# The item variables s8 sets are unset for every child, as step 1b's `wrapped` does.
UNSET_VARS = ("FIX_ITEM", "STORY_ITEM", "ITEM")
DEFAULT_TIMEOUT = 3600

C2 = "origin/claude/s8j-afk-batch-2-tg23uw-c2"
C3 = "origin/claude/s8j-afk-batch-2-tg23uw-c3"
C4 = "origin/claude/s8j-afk-batch-2-tg23uw-c4"


def _it(*names):
    """One test group: rk-cli's `--test it` binary, each name selected with `--exact`."""
    return {"package": "rk-cli", "target": ["--test", "it"], "names": list(names)}


# The ten mutants, in step 1b's S8J_MUTANTS order. `anchor_from` names the branch the anchor was read from (with
# `git show <branch>:<file>`) when the code was not yet on the integration branch. An entry whose anchor is TBD
# makes the driver refuse to run: its `file` is a hint read from build/phase-0-to-1, not a confirmed site.
MUTANTS = [
    {
        "id": "M-seed-record",
        "section": "8j.4",
        "change": "through_step_seven seeds the stage from the item record, ignoring the reconciled trailer",
        "file": "crates/rk-cli/src/verbs/run.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: rk-cli verbs::run::through_step_seven; choose the anchor once C4's binary arm is green",
        "tests": [_it(
            "lane_proof::gate_then_answer_then_second_process_continues_binary_arm",
            "lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification",
        )],
    },
    {
        "id": "M-land-empty",
        "section": "8j.4",
        "change": "the landing commits allow_empty with nothing staged",
        "file": "crates/rk-git/src/landing.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: rk-git landing's `if !due && !allow_empty` commit path",
        "tests": [_it("lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness")],
    },
    {
        "id": "M-fixture-toolchain",
        "section": "8j.4",
        "change": "the composition root builds FixtureToolchain",
        "file": "crates/rk-cli/src/verbs/run.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: the toolchain choice in rk-cli verbs::run (detect() over the run-owned worktree)",
        "tests": [_it("lane_proof::fix_lane_verify_completed_names_the_rust_adapter")],
    },
    {
        "id": "M-skip-empty-stage-trailer",
        "section": "8j.4",
        "change": "a stage_empty stage mints no trailer",
        "file": "crates/rk-kernel/src/replay.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: rk-kernel replay's stage_empty row and its Story: mark",
        "tests": [_it(
            "lane_proof::fix_lane_trailers_every_non_forbidden_stage_in_order",
            "lane_proof::story_lane_trailers_every_non_forbidden_stage_in_order",
        )],
    },
    {
        "id": "M-spec-digest-pre-render",
        "section": "8j.4",
        "change": "artifact.write's digest is taken over the pre-render value, not the written bytes",
        "file": "crates/rk-store/src/render.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: rk-store render's ArtifactWriter digest (file not confirmed)",
        "tests": [_it("lane_proof::story_lane_spec_is_landed_and_journalled")],
    },
    {
        "id": "M-worktree-under-git",
        "section": "8j.4",
        "change": "the run-worktree root is derived under the common dir again",
        "file": "crates/rk-git/src/worktree.rs",
        "anchor": TBD,
        "replacement": TBD,
        "anchor_from": C4,
        "note": "hint: rk-git worktree's `main.join(\".redkiln\").join(\"worktrees\")`",
        "tests": [_it("lane_proof::fix_lane_reaches_a_landed_content_commit_under_fake_harness")],
    },
    {
        "id": "M-attach-dropped",
        "section": "8j.2",
        "change": "the BugReport question's option loses its attaches (a content mutation, not a loader one)",
        "file": ".redkiln/processes/questions.toml",
        "anchor": 'options = [{ id = "attach", attaches = "BugReport" }]\n',
        "replacement": 'options = [{ id = "attach" }]\n',
        "anchor_from": C2,
        "note": "",
        "tests": [_it(
            "processes::the_two_human_root_questions_load_and_each_attaches_its_kind",
            "lane_proof::human_root_is_attached_through_the_pack_question_on_both_lanes",
        )],
    },
    {
        "id": "M-hold-ignored",
        "section": "8j.3",
        "change": "the hold is not consulted",
        "file": "crates/rk-kernel/src/workflow/fsm.rs",
        "anchor": "        if ctx.hold_before == Some(to) {\n",
        "replacement": "        if false && ctx.hold_before == Some(to) {\n",
        "anchor_from": C3,
        "note": "",
        "tests": [_it("lane_proof::boundary_hold_stops_the_lane_at_analysis_to_specification")],
    },
    {
        "id": "M-hold-spent-refires",
        "section": "8j.3",
        "change": "an answered hold opens again",
        "file": "crates/rk-kernel/src/workflow/fsm.rs",
        "anchor": (
            "            if !super::hold::standing(state, &kind).is_some_and(super::hold::releases) {\n"
            "                return Ok(NextAction::Done);\n"
        ),
        "replacement": (
            "            if super::hold::standing(state, &kind).is_some() {\n"
            "                return Ok(NextAction::OpenGate(GateRecord::derive_reach(&kind, ctx.tree_hash,"
            " ctx.deadline, state.reaches(&kind).saturating_add(1))?));\n"
        ),
        "anchor_from": C3,
        "note": "the change is C3's own driver's, re-anchored to C3 3961b48 (standing takes the kind)",
        "tests": [_it("lane_proof::boundary_hold_answered_lets_the_next_process_enter_specification")],
    },
    {
        "id": "M-hold-unordered-accepted",
        "section": "8j.3",
        "change": "the load check on the held stage is skipped",
        "file": "crates/rk-cli/src/verbs/run.rs",
        "anchor": "    if order.iter().any(|s| s == stage) {\n",
        "replacement": "    if true || order.iter().any(|s| s == stage) {\n",
        "anchor_from": C3,
        "note": "",
        "tests": [_it("run::a_hold_naming_a_stage_the_pack_does_not_order_is_refused_at_load")],
    },
]

TEST_LINE = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)\s*$")
RESULT_LINE = re.compile(r"^test result: ")


class Refused(Exception):
    """A precondition failed or a run could not be read: nothing is recorded (exit 2)."""


class RestoreMismatch(Exception):
    """A restored file's sha256 is not the pre-mutation sha256 (exit 3)."""


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def is_tbd(entry):
    return TBD in (entry.get("anchor"), entry.get("replacement"))


def eol_form(data, text):
    """`text` as UTF-8 bytes, in the file's line-ending form: CRLF when the file is CRLF (core.autocrlf=true)."""
    raw = text.encode("utf-8")
    if b"\r\n" in data:
        raw = raw.replace(b"\r\n", b"\n").replace(b"\n", b"\r\n")
    return raw


def mutate_bytes(data, entry):
    """The mutated bytes, after asserting the anchor once. Raises Refused on a missing or repeated anchor."""
    if is_tbd(entry):
        raise Refused(f"{entry['id']}: its anchor is {TBD}")
    anchor = eol_form(data, entry["anchor"])
    replacement = eol_form(data, entry["replacement"])
    if anchor == replacement:
        raise Refused(f"{entry['id']}: the replacement equals the anchor")
    n = data.count(anchor)
    if n != 1:
        raise Refused(f"{entry['id']}: its anchor occurs {n} times in {entry['file']} (exactly one is required)")
    return data.replace(anchor, replacement, 1)


def rel_path(root, rel):
    return Path(root, *rel.split("/"))


def atomic_write(path, data, attempts=10):
    """Write through a temporary file in the same directory and `os.replace` it, retrying a Windows lock."""
    fd, tmp = tempfile.mkstemp(prefix=path.name + ".", suffix=".s8j-tmp", dir=str(path.parent))
    try:
        with os.fdopen(fd, "wb") as fh:
            fh.write(data)
            fh.flush()
            os.fsync(fh.fileno())
        for i in range(attempts):
            try:
                os.replace(tmp, path)
                return
            except PermissionError:
                if i == attempts - 1:
                    raise
                time.sleep(0.5)
    finally:
        if os.path.exists(tmp):
            os.unlink(tmp)


class _Deferred:
    """Hold SIGINT/SIGTERM while a restore runs, then re-deliver one as KeyboardInterrupt."""

    NAMES = ("SIGINT", "SIGTERM", "SIGBREAK", "SIGHUP")

    def __enter__(self):
        self.pending = False
        self.saved = {}
        for name in self.NAMES:
            sig = getattr(signal, name, None)
            if sig is None:
                continue
            try:
                self.saved[sig] = signal.signal(sig, self._note)
            except (ValueError, OSError):
                pass
        return self

    def _note(self, signum, frame):
        self.pending = True

    def __exit__(self, *exc):
        for sig, handler in self.saved.items():
            signal.signal(sig, handler)
        if self.pending and exc[0] is None:
            raise KeyboardInterrupt
        return False


def _interrupt(signum, frame):
    raise KeyboardInterrupt(f"signal {signum}")


def install_signal_handlers():
    """Turn SIGTERM, SIGBREAK (Windows Ctrl-Break) and SIGHUP into KeyboardInterrupt, so `finally` restores."""
    for name in ("SIGTERM", "SIGBREAK", "SIGHUP"):
        sig = getattr(signal, name, None)
        if sig is not None:
            try:
                signal.signal(sig, _interrupt)
            except (ValueError, OSError):
                pass


def cargo_command(group):
    return ["cargo", "test", "-p", group["package"], *group["target"], "--", *group["names"], "--exact"]


def child_env():
    env = dict(os.environ)
    for var in UNSET_VARS:
        env.pop(var, None)
    return env


def run_command(argv, cwd, timeout):
    """Run one test command and return (rc, combined output). A timeout is a hard error."""
    try:
        done = subprocess.run(argv, cwd=str(cwd), env=child_env(), stdout=subprocess.PIPE,
                              stderr=subprocess.STDOUT, timeout=timeout)
    except subprocess.TimeoutExpired as exc:
        raise Refused(f"`{' '.join(argv[:6])} ...` exceeded {timeout}s") from exc
    except FileNotFoundError as exc:
        raise Refused(f"`{argv[0]}` was not found on PATH") from exc
    return done.returncode, done.stdout.decode("utf-8", errors="replace")


def read_outcomes(text, names, label):
    """Each named test's reported outcome. Refuses a run that reported no result or left a named test out."""
    if not any(RESULT_LINE.match(line) for line in text.splitlines()):
        raise Refused(f"{label}: no `test result:` line (did it compile?)")
    seen = {}
    for line in text.splitlines():
        m = TEST_LINE.match(line.strip())
        if m and m.group(1) in names:
            if m.group(1) in seen:
                raise Refused(f"{label}: `{m.group(1)}` was reported twice")
            seen[m.group(1)] = m.group(2)
    missing = [n for n in names if n not in seen]
    if missing:
        raise Refused(f"{label}: named test(s) not reported: {', '.join(missing)}")
    return seen


class Driver:
    def __init__(self, root, mutants, command_for=cargo_command, head_fn=None, clean_fn=None,
                 timeout=DEFAULT_TIMEOUT, out=sys.stdout, results_rel=RESULTS_REL, runner=run_command):
        self.root = Path(root)
        self.mutants = mutants
        self.command_for = command_for
        self.head_fn = head_fn or (lambda: git_head(self.root))
        self.clean_fn = clean_fn or (lambda: git_dirty(self.root))
        self.timeout = timeout
        self.out = out
        self.results = rel_path(self.root, results_rel)
        self.work = self.results.parent
        self.runner = runner

    def say(self, msg):
        self.out.write(msg + "\n")
        self.out.flush()

    def preflight(self, selected):
        tbd = [m["id"] for m in self.mutants if is_tbd(m)]
        if tbd:
            raise Refused(f"{len(tbd)} mutant(s) still carry {TBD}: {', '.join(tbd)}")
        ids = [m["id"] for m in self.mutants]
        if len(set(ids)) != len(ids):
            raise Refused("a mutant id is repeated")
        dirty = self.clean_fn()
        if dirty:
            raise Refused("the tree is not clean apart from the evidence file; the HEAD recorded would not be "
                          "what ran:\n" + dirty)
        for entry in selected:
            path = rel_path(self.root, entry["file"])
            if not path.is_file():
                raise Refused(f"{entry['id']}: {entry['file']} does not exist")
            mutate_bytes(path.read_bytes(), entry)

    def baseline(self, selected):
        """Every named test, unmutated, must be reported `ok`."""
        groups = {}
        for entry in selected:
            for g in entry["tests"]:
                key = (g["package"], tuple(g["target"]))
                bucket = groups.setdefault(key, {"package": g["package"], "target": g["target"], "names": []})
                for n in g["names"]:
                    if n not in bucket["names"]:
                        bucket["names"].append(n)
        for g in groups.values():
            rc, text = self.runner(self.command_for(g), self.root, self.timeout)
            (self.work / "baseline.log").write_text(text, encoding="utf-8")
            seen = read_outcomes(text, g["names"], "baseline")
            red = [n for n, o in seen.items() if o != "ok"]
            if red or rc != 0:
                raise Refused(f"baseline: not green unmutated (rc={rc}): {', '.join(red) or 'see baseline.log'}")
        self.say(f"baseline: {sum(len(g['names']) for g in groups.values())} named test(s) ok unmutated")

    def run_one(self, entry, head):
        path = rel_path(self.root, entry["file"])
        original = path.read_bytes()
        before = sha256(original)
        mutated = mutate_bytes(original, entry)
        backup = self.work / "backup" / (entry["id"] + ".orig")
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(original)
        log = []
        try:
            atomic_write(path, mutated)
            self.say(f"{entry['id']}: applied to {entry['file']} (pre-mutation sha256 {before[:16]})")
            outcomes = {}
            for g in entry["tests"]:
                rc, text = self.runner(self.command_for(g), self.root, self.timeout)
                log.append(text)
                outcomes.update(read_outcomes(text, g["names"], entry["id"]))
        finally:
            with _Deferred():
                atomic_write(path, original)
                after = sha256(path.read_bytes())
                if after != before:
                    raise RestoreMismatch(f"{entry['id']}: {entry['file']} restored to sha256 {after}, expected "
                                          f"{before}; the pre-mutation bytes are in {backup}")
            backup.unlink()
            (self.work / (entry["id"] + ".log")).write_text("\n".join(log), encoding="utf-8")
            self.say(f"{entry['id']}: restored, sha256 {before[:16]} verified")
        verdict = "killed" if all(o == "FAILED" for o in outcomes.values()) else "SURVIVED"
        self.say(f"{entry['id']}: {verdict} ({', '.join(f'{n} {o}' for n, o in outcomes.items())})")
        with open(self.results, "a", encoding="utf-8", newline="\n") as fh:
            fh.write(f"{entry['id']}\t{verdict}\t{head}\n")
            fh.flush()
            os.fsync(fh.fileno())
        return verdict

    def run(self, only=()):
        unknown = [i for i in only if i not in {m["id"] for m in self.mutants}]
        if unknown:
            raise Refused(f"unknown mutant id(s): {', '.join(unknown)}")
        selected = [m for m in self.mutants if not only or m["id"] in only]
        self.work.mkdir(parents=True, exist_ok=True)
        self.preflight(selected)
        head = self.head_fn()
        self.say(f"HEAD {head}; CARGO_TARGET_DIR={os.environ.get('CARGO_TARGET_DIR', '(unset)')}; "
                 f"results {self.results}")
        self.baseline(selected)
        verdicts = {m["id"]: self.run_one(m, head) for m in selected}
        if self.head_fn() != head:
            raise Refused("HEAD moved during the run; its results are not this code's")
        dirty = self.clean_fn()
        if dirty:
            raise Refused("the tree is not clean after the run:\n" + dirty)
        survived = [i for i, v in verdicts.items() if v != "killed"]
        self.say(f"{len(verdicts) - len(survived)} killed, {len(survived)} SURVIVED"
                 + (f": {', '.join(survived)} (a survivor blocks)" if survived else ""))
        return 1 if survived else 0


def git(root, *args):
    done = subprocess.run(["git", *args], cwd=str(root), stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if done.returncode != 0:
        raise Refused(f"git {' '.join(args)} failed: {done.stderr.decode('utf-8', errors='replace').strip()}")
    return done.stdout.decode("utf-8", errors="replace")


def git_head(root):
    return git(root, "rev-parse", "HEAD").strip()


def git_dirty(root):
    return git(root, "status", "--porcelain", "--", ".", f":!{EVIDENCE_REL}").strip()


def check_wrapper():
    """Refuse unless run inside nodispatch.sh's wrapper (step 1b's D21-47 wrapper)."""
    if os.environ.get("ND_ACTIVE") != "1" or not os.environ.get("ND_TRAP_NATIVE"):
        raise Refused("not inside the no-dispatch wrapper: run `. runbook/tools/nodispatch.sh && nd_setup "
                      "\"$PWD/target/s8j-mutants/nd\"`, then `nodispatch python runbook/tools/s8j-mutants.py`")
    if "REDKILN_LAUNCH_RECORD" in os.environ:
        raise Refused("REDKILN_LAUNCH_RECORD is set inside the wrapper")


def check_trap():
    trap = Path(os.environ["ND_TRAP_NATIVE"])
    left = list(trap.iterdir()) if trap.is_dir() else [trap]
    calls = trap.parent / "calls.log"
    if left or calls.exists():
        raise Refused(f"a claude process ran under the mutants: trap {trap} holds {len(left)} entr(y/ies), "
                      f"calls.log {'present' if calls.exists() else 'absent'}")


def check_anchors(root, rev, mutants, out):
    """Assert every non-TBD anchor once against `git show <rev>:<file>`; touches no file."""
    bad = 0
    for entry in mutants:
        if is_tbd(entry):
            out.write(f"{entry['id']}\t{TBD}\n")
            continue
        try:
            data = git(root, "show", f"{rev}:{entry['file']}").encode("utf-8")
            mutate_bytes(data, entry)
            out.write(f"{entry['id']}\tanchor once\t{rev}:{entry['file']}\n")
        except Refused as exc:
            bad += 1
            out.write(f"{entry['id']}\tFAIL\t{exc}\n")
    return 2 if bad else 0


# --- the synthetic controls (no cargo) -------------------------------------------------------------------------

FAKE_TEST = r'''
import sys
src = open(sys.argv[1], "rb").read().decode("utf-8")
names = [a for a in sys.argv[2:] if a != "--exact"]
for name in names:
    if name == "lib::adds":
        ok = "return a + b" in src
    elif name == "lib::constant":
        ok = True
    else:
        continue
    print(f"test {name} ... {'ok' if ok else 'FAILED'}")
print("test result: done")
'''

SELF_LIB = "def add(a, b):\n    # adds two numbers\n    return a + b\n"


def self_test_mutants():
    group = {"package": "fake", "target": [], "names": ["lib::adds"]}
    return [
        {"id": "CTL-kill", "file": "lib.py", "anchor": "    return a + b\n", "replacement": "    return a - b\n",
         "tests": [group]},
        {"id": "CTL-survive", "file": "lib.py", "anchor": "    # adds two numbers\n",
         "replacement": "    # adds two numbers, mutated\n", "tests": [group]},
        {"id": "CTL-partial", "file": "lib.py", "anchor": "    return a + b\n", "replacement": "    return a - b\n",
         "tests": [{"package": "fake", "target": [], "names": ["lib::adds", "lib::constant"]}]},
    ]


def self_test_driver(tmp, mutants, out, eol="\n"):
    root = Path(tmp)
    (root / "lib.py").write_bytes(SELF_LIB.replace("\n", eol).encode("utf-8"))
    (root / "fake_test.py").write_text(FAKE_TEST, encoding="utf-8")

    def command_for(group):
        return [sys.executable, "-I", str(root / "fake_test.py"), str(root / "lib.py"), *group["names"], "--exact"]

    return Driver(root, mutants, command_for=command_for, head_fn=lambda: "selftest", clean_fn=lambda: "",
                  timeout=60, out=out)


def read_results(driver):
    return [line.split("\t") for line in driver.results.read_text(encoding="utf-8").splitlines()]


def self_test(out=sys.stdout):
    """A known kill reads `killed`, a known survivor and a half-kill read `SURVIVED`, and every file is restored."""
    for eol in ("\n", "\r\n"):
        with tempfile.TemporaryDirectory() as tmp:
            d = self_test_driver(tmp, self_test_mutants(), out, eol)
            original = (Path(tmp) / "lib.py").read_bytes()
            rc = d.run()
            got = {r[0]: r[1] for r in read_results(d)}
            want = {"CTL-kill": "killed", "CTL-survive": "SURVIVED", "CTL-partial": "SURVIVED"}
            if got != want or rc != 1 or (Path(tmp) / "lib.py").read_bytes() != original:
                out.write(f"SELF-TEST FAIL ({eol!r}): results {got}, rc {rc}\n")
                return 2
            if any(r[2] != "selftest" for r in read_results(d)):
                out.write("SELF-TEST FAIL: a results line does not carry the HEAD\n")
                return 2
    out.write("self-test: CTL-kill killed, CTL-survive SURVIVED, CTL-partial SURVIVED; LF and CRLF; restored\n")
    return 0


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--self-test", action="store_true", help="run the synthetic controls (no cargo)")
    ap.add_argument("--list", action="store_true", help="print the mutant table and exit")
    ap.add_argument("--check-anchors", metavar="REV", help="assert every anchor once at REV; touch nothing")
    ap.add_argument("--only", action="append", default=[], metavar="ID", help="run only this mutant (repeatable)")
    ap.add_argument("--timeout", type=int, default=DEFAULT_TIMEOUT, help="seconds per cargo command")
    args = ap.parse_args(argv)
    root = Path(__file__).resolve().parent.parent.parent
    install_signal_handlers()
    try:
        if args.self_test:
            return self_test()
        if args.list:
            for m in MUTANTS:
                names = [n for g in m["tests"] for n in g["names"]]
                anchor = TBD if is_tbd(m) else f"from {m['anchor_from']}"
                print(f"{m['id']}\t§{m['section']}\t{m['file']}\t{anchor}\t{' '.join(names)}")
            return 0
        if args.check_anchors:
            return check_anchors(root, args.check_anchors, MUTANTS, sys.stdout)
        if not (root / "Cargo.toml").is_file() or not (root / "runbook").is_dir():
            raise Refused(f"{root} is not the redkiln-rs repository root")
        check_wrapper()
        rc = Driver(root, MUTANTS, timeout=args.timeout).run(only=args.only)
        check_trap()
        return rc
    except RestoreMismatch as exc:
        sys.stderr.write(f"ABORT: RESTORE DID NOT VERIFY: {exc}\n")
        return 3
    except Refused as exc:
        sys.stderr.write(f"FAIL: {exc}\n")
        return 2
    except KeyboardInterrupt:
        sys.stderr.write("FAIL: interrupted; every applied mutant was restored and verified before this line\n")
        return 2


if __name__ == "__main__":
    sys.exit(main())
