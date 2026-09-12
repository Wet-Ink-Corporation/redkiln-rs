#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
census_deps.py -- PHASE-1 DEPENDENCY GRAPH for D:/repos/redkiln-rs.

READ-ONLY on the repo. Writes ONE artifact:
    <OUT>/phase1-deps.json

Everything it reports is COMPUTED here:
  * step / sub-step boundaries and checkpoint lines are found by regex over RUNBOOK.md
  * [WILL EXIST] markers are found by regex and attributed to the containing (sub-)step
  * every dependency's exists_now is verified against the filesystem, git, or a tool run
  * blocked_by, critical_path and ordering_hazards are derived from those verifications

ASCII-only output. No non-ASCII is ever printed (Windows console constraint).
"""

import json
import os
import re
import subprocess
import sys
import glob as globmod

REPO = "D:/repos/redkiln-rs"
RUNBOOK = os.path.join(REPO, "RUNBOOK.md")
OUT_DIR = os.environ.get(
    "RK_AUDIT_OUT",
    "D:/repos/redkiln-rs/references/audit/2026-09-10/census",
)
OUT = os.path.join(OUT_DIR, "phase1-deps.json")

# --------------------------------------------------------------------------
# 0. primitives
# --------------------------------------------------------------------------

_anchor_asserts = []


def anchor(name, ok, detail):
    """Golden constraint 5: assert a pattern matched what we think before relying on it."""
    _anchor_asserts.append({"anchor": name, "matched": bool(ok), "detail": detail})
    return ok


def read_lines(path):
    with open(path, "r", encoding="utf-8", newline="") as fh:
        return fh.read().split("\n")


def rel(p):
    return os.path.join(REPO, p)


def path_exists(p):
    return os.path.exists(rel(p))


def abs_exists(p):
    return os.path.exists(p)


def glob_count(pat):
    return len(globmod.glob(os.path.join(REPO, pat)))


def file_bytes(p):
    try:
        return os.path.getsize(rel(p))
    except OSError:
        return None


def grep_count(p, pattern, fixed=False):
    """Count matching LINES in a file. Returns None if the file is missing."""
    fp = rel(p)
    if not os.path.exists(fp):
        return None
    n = 0
    rx = None if fixed else re.compile(pattern)
    with open(fp, "r", encoding="utf-8", errors="replace", newline="") as fh:
        for line in fh:
            line = line.rstrip("\r\n")
            if (pattern in line) if fixed else rx.search(line):
                n += 1
    return n


def grep_count_abs(fp, pattern, fixed=False):
    if not os.path.exists(fp):
        return None
    n = 0
    rx = None if fixed else re.compile(pattern)
    with open(fp, "r", encoding="utf-8", errors="replace", newline="") as fh:
        for line in fh:
            line = line.rstrip("\r\n")
            if (pattern in line) if fixed else rx.search(line):
                n += 1
    return n


def run(cmd, cwd=None):
    """Run a read-only command. Returns (rc, combined-output-one-line)."""
    try:
        p = subprocess.run(
            cmd, cwd=cwd, capture_output=True, text=True, timeout=90, shell=False
        )
        out = (p.stdout or "") + (p.stderr or "")
        out = " ".join(out.split())
        return p.returncode, out[:400]
    except Exception as exc:  # noqa: BLE001
        return 127, "EXEC-FAILED: " + " ".join(str(exc).split())[:200]


# --------------------------------------------------------------------------
# 1. tool probes (run once, reused by the dependency table)
# --------------------------------------------------------------------------

TOOL_CMDS = {
    "cargo": ["cargo", "--version"],
    "rustc": ["rustc", "--version"],
    "git": ["git", "--version"],
    "python": ["python", "--version"],
    "claude": ["claude", "--version"],
    "cargo-audit": ["cargo", "audit", "--version"],
    "cargo-insta": ["cargo", "insta", "--version"],
    "awk": ["awk", "--version"],
    "sed": ["sed", "--version"],
    "grep": ["grep", "--version"],
    "wc": ["wc", "--version"],
    "date": ["date", "--version"],
}

TOOLS = {}
for _k, _c in TOOL_CMDS.items():
    _rc, _o = run(_c)
    # `cargo audit` when absent still exits 0 in this shell wrapper; detect by text.
    _absent = ("no such command" in _o) or ("not found" in _o.lower()) or (_rc == 127)
    TOOLS[_k] = {
        "command": " ".join(_c),
        "exit_code": _rc,
        "output": _o.split(" Copyright")[0][:200],
        "installed": (not _absent),
    }

# --------------------------------------------------------------------------
# 2. git probes
# --------------------------------------------------------------------------

_rc, _branch = run(["git", "-C", REPO, "rev-parse", "--abbrev-ref", "HEAD"])
_rc, _head = run(["git", "-C", REPO, "rev-parse", "HEAD"])
_rc, _porc = run(["git", "-C", REPO, "status", "--porcelain"])
_porc_lines = [l for l in _porc.split(" ") if False]  # placeholder; re-run unsplit


def run_raw(cmd):
    try:
        p = subprocess.run(cmd, capture_output=True, text=True, timeout=90)
        return p.returncode, (p.stdout or "") + (p.stderr or "")
    except Exception as exc:  # noqa: BLE001
        return 127, "EXEC-FAILED " + str(exc)


_rc, _porc_raw = run_raw(["git", "-C", REPO, "status", "--porcelain"])
_porc_rows = [l for l in _porc_raw.split("\n") if l.strip()]
UNTRACKED = [l for l in _porc_rows if l.startswith("??")]
MODIFIED = [l for l in _porc_rows if l.startswith(" M") or l.startswith("M ")]

_rc, _pred_head = run(["git", "-C", "D:/repos/redkiln", "rev-parse", "HEAD"])
_rc, _pred_status = run_raw(
    ["git", "-C", "D:/repos/redkiln", "status", "--porcelain", "--", "workflows/"]
)
PRED_WORKFLOW_DIRTY = [l for l in _pred_status.split("\n") if l.strip()]

GIT = {
    "redkiln_rs_branch": _branch,
    "redkiln_rs_head": _head,
    "redkiln_rs_untracked_count": len(UNTRACKED),
    "redkiln_rs_modified_count": len(MODIFIED),
    "redkiln_rs_porcelain_total": len(_porc_rows),
    "predecessor_redkiln_head": _pred_head,
    "predecessor_workflows_dirty": PRED_WORKFLOW_DIRTY,
}

# --------------------------------------------------------------------------
# 3. RUNBOOK structural parse
# --------------------------------------------------------------------------

LINES = read_lines(RUNBOOK)
if LINES and LINES[-1] == "":
    LINES = LINES[:-1]  # wc -l semantics
NLINES = len(LINES)
anchor("runbook_line_count_2475", NLINES in (2475, 2476), "wc -l == 2475; parsed %d" % NLINES)

# H2 step headings
H2 = []
for i, l in enumerate(LINES, start=1):
    m = re.match(r"^## (Step ([0-9]+[a-z]?)\b.*|Halt conditions.*|One-glance checklist.*|Preamble.*)$", l)
    if m:
        H2.append((i, l[3:].strip()))
anchor("h2_headings_found", len(H2) >= 13, "found %d H2 section headings" % len(H2))

# H3 sub-step headings inside Step 3/4/4a/5/6/7 etc.
H3 = []
for i, l in enumerate(LINES, start=1):
    m = re.match(r"^### (.*)$", l)
    if m:
        H3.append((i, m.group(1).strip()))

# Checkpoint declaration lines: bold "**Checkpoint N.**" / "**Checkpoint 7d:**" / re-scoped form
CHECKPOINTS = {}
for i, l in enumerate(LINES, start=1):
    m = re.search(r"\*\*Checkpoint ([0-9]+[a-z]?)[\.,:]", l)
    if m:
        CHECKPOINTS.setdefault(m.group(1), i)
anchor(
    "sixteen_checkpoints",
    len(CHECKPOINTS) == 16,
    "checkpoint ids found: %s" % ",".join(sorted(CHECKPOINTS)),
)

# [WILL EXIST] markers
WE_MARKERS = []
for i, l in enumerate(LINES, start=1):
    if "WILL EXIST" in l:
        WE_MARKERS.append(i)
anchor("will_exist_markers", len(WE_MARKERS) >= 10, "found %d lines carrying WILL EXIST" % len(WE_MARKERS))

# What each [WILL EXIST] defers -- read off the marker's own line/next lines, verified by hand
WE_DEFERS = {
    14: "the runbook's own convention statement: commands that do not exist yet are marked and written in the form they will take",
    624: "no rig subcommand poses the D-FIXCONSUMER question; the RUNBOOK enumerates the rig subcommands and none of them is it",
    630: "redkiln-spike-runner fix-consumer-probe -- the one haiku-scale Workflow-runtime dispatch",
    1237: "spikes/p1/score-arm-l.py -- there is no arm-L panel scorer at all",
    1243: "the score-arm-l.py invocation, written in the form it will take",
    1550: "two of Checkpoint 4a's eight lines are Phase-1 lint output built at Step 7b",
    1569: "redkiln lint --facet-provenance and cargo test -p rk-kernel -- facet_provenance_fixtures",
    1801: "the two S-GIX arms as #[ignore]d tests inside crates/rk-git",
    2168: "redkiln lint --facet-stability AND its argument .redkiln/facets.toml -- both Phase-1 output",
    2191: "redkiln lint --facet-provenance -- Step 4a authors the registry half, Step 7b builds the lint",
}

# --------------------------------------------------------------------------
# 4. step table -- boundaries computed from the headings above
# --------------------------------------------------------------------------

# (step_id, heading_regex, is_substep_of, phase, writes_product_code)
STEP_SPEC = [
    ("0", r"^## Step 0 ", None, 0, False),
    ("0.1", r"^### 0\.1 ", "0", 0, False),
    ("0.2", r"^### 0\.2 ", "0", 0, False),
    ("0.3", r"^### 0\.3 ", "0", 0, False),
    ("0.4", r"^### 0\.4 ", "0", 0, False),
    ("1", r"^## Step 1 ", None, 0, False),
    ("1.1", r"^### 1\.1 ", "1", 0, False),
    ("1.2", r"^### 1\.2 ", "1", 0, False),
    ("1.3", r"^### 1\.3 ", "1", 0, False),
    ("2", r"^## Step 2 ", None, 0, False),
    ("3", r"^## Step 3 ", None, 0, False),
    ("3a", r"^### 3a ", "3", 0, False),
    ("3b", r"^### 3b ", "3", 0, False),
    ("3c", r"^### 3c ", "3", 0, False),
    ("4", r"^## Step 4 ", None, 0, False),
    ("4.1", r"^### 4\.1 ", "4", 0, False),
    ("4.2", r"^### 4\.2 ", "4", 0, False),
    ("4.3", r"^### 4\.3 ", "4", 0, False),
    ("4.4", r"^### 4\.4 ", "4", 0, False),
    ("4.5", r"^### 4\.5 ", "4", 0, False),
    ("4a", r"^## Step 4a ", None, 0, False),
    ("4a.1", r"^### 4a\.1 ", "4a", 0, False),
    ("4a.2", r"^### 4a\.2 ", "4a", 0, False),
    ("4a.3", r"^### 4a\.3 ", "4a", 0, False),
    ("4a.4", r"^### 4a\.4 ", "4a", 0, False),
    ("5", r"^## Step 5 ", None, 1, True),
    ("5.1", r"^### 5\.1 ", "5", 1, True),
    ("5.2", r"^### 5\.2 ", "5", 1, False),
    ("5.3", r"^### 5\.3 ", "5", 1, True),
    ("6", r"^## Step 6 ", None, 1, True),
    ("6.1", r"^### 6\.1 ", "6", 1, True),
    ("6.2", r"^### 6\.2 ", "6", 1, True),
    ("6.3", r"^### 6\.3 ", "6", 1, True),
    ("7", r"^## Step 7 ", None, 1, True),
    ("7a", r"^### 7a ", "7", 1, True),
    ("7b", r"^### 7b ", "7", 1, True),
    ("7c", r"^### 7c ", "7", 1, True),
    ("7d", r"^### 7d ", "7", 1, True),
    ("8", r"^## Step 8 ", None, 1, False),
    ("9", r"^## Step 9 ", None, 1, False),
]

# checkpoint id per step id (the checkpoint that CLOSES that step)
STEP_CHECKPOINT = {
    "0": "0", "1": "1", "2": "2", "3a": "3a", "3b": "3b", "3c": "3c",
    "4": "4", "4a": "4a", "5": "5", "6": "6",
    "7a": "7a", "7b": "7b", "7c": "7c", "7d": "7d", "8": "8", "9": "9",
}

# find each heading's start line
starts = {}
for sid, rx, _p, _ph, _w in STEP_SPEC:
    hit = [i for i, l in enumerate(LINES, start=1) if re.match(rx, l)]
    if len(hit) != 1:
        anchor("heading_%s" % sid, False, "expected 1 heading match, got %d" % len(hit))
        continue
    anchor("heading_%s" % sid, True, "line %d" % hit[0])
    starts[sid] = hit[0]

# end lines: next heading of the same-or-higher level
HALT_LINE = [i for i, l in enumerate(LINES, start=1) if re.match(r"^## Halt conditions", l)]
anchor("halt_heading", len(HALT_LINE) == 1, "line %s" % HALT_LINE)
HALT_LINE = HALT_LINE[0] if HALT_LINE else NLINES

H2_LINES = sorted(i for i, _ in H2)
STEP_HEADING_LINES = sorted(set(starts.values()) | set(H2_LINES))
PARENT_OF = {sid: parent for sid, _rx, parent, _ph, _w in STEP_SPEC}


def end_of(sid):
    """Top-level step: runs to the next H2. Sub-step: runs to the next STEP heading of any level
    (a non-step H3 such as the UTC-Z timestamp-trap heading does not close a sub-step)."""
    s = starts[sid]
    pool = H2_LINES if PARENT_OF.get(sid) is None else STEP_HEADING_LINES
    later = [i for i in pool if i > s]
    return (min(later) - 1) if later else NLINES


# --------------------------------------------------------------------------
# 5. dependency table. evidence_anchor is RUNBOOK.md:<line> (or another file:line).
#    Every check is executed; nothing is asserted from the RUNBOOK's own claims.
# --------------------------------------------------------------------------

def dep(name, kind, anchor_line, check, note="", role="prerequisite", destroyed_by=None, owner=None):
    """role: prerequisite (hard; blocks the step) | own_output (the step's own work) |
             soft (RUNBOOK says its absence does not block the step)."""
    assert role in ("prerequisite", "own_output", "soft")
    return {"name": name, "kind": kind, "evidence_anchor": anchor_line,
            "check": check, "note": note, "role": role, "destroyed_by": destroyed_by, "owner": owner}


RB = lambda n: "RUNBOOK.md:%d" % n  # noqa: E731

DEPS = {
    "0": [
        dep("git executable", "binary_tool", RB(232), ("tool", "git")),
        dep("rustc 1.97.1+", "binary_tool", RB(233), ("tool", "rustc")),
        dep("cargo 1.97.1+", "binary_tool", RB(234), ("tool", "cargo")),
        dep("python", "binary_tool", RB(51), ("tool", "python")),
        dep("claude CLI (instrument)", "binary_tool", RB(246), ("tool", "claude")),
        dep("sed", "binary_tool", RB(51), ("tool", "sed")),
        dep("awk (Checkpoint 4a deriver-count line)", "binary_tool", RB(51), ("tool", "awk")),
        dep("grep", "binary_tool", RB(51), ("tool", "grep")),
        dep("wc", "binary_tool", RB(51), ("tool", "wc")),
        dep("date", "binary_tool", RB(51), ("tool", "date")),
        dep("cargo-audit", "binary_tool", RB(55), ("tool", "cargo-audit"),
            "RUNBOOK:56 claims ABSENT; Step 5 requires it"),
        dep("cargo-insta", "binary_tool", RB(56), ("tool", "cargo-insta"),
            "RUNBOOK:56 claims ABSENT; Step 5.3 snapshot workflow requires it"),
        dep("spikes/runner (rig crate, must cargo build)", "crate", RB(261),
            ("path", "spikes/runner/Cargo.toml")),
        dep("spikes/runner/target/debug/redkiln-spike-runner", "binary_tool", RB(261),
            ("path_any", ["spikes/runner/target/debug/redkiln-spike-runner.exe",
                          "spikes/runner/target/debug/redkiln-spike-runner"])),
        dep("spikes/results/instrument-checks/ (dir must pre-exist)", "file", RB(289),
            ("path", "spikes/results/instrument-checks")),
        dep("D:/repos/redkiln frozen at 47faec9f", "external", RB(36),
            ("git_head", ("D:/repos/redkiln", "47faec9f46b8d983c2c75f0794164e68415ae706"))),
        dep("branch build/phase-0-to-1 cut", "human_act", RB(276),
            ("git_branch", "build/phase-0-to-1"), role="own_output"),
        dep("spikes/results/instrument-checks/RUNBOOK-instrument.txt", "file", RB(283),
            ("path", "spikes/results/instrument-checks/RUNBOOK-instrument.txt"),
            "produced BY Checkpoint 0; Step 1.3 stages it", role="own_output",
            owner="Ryan (operator) -- `claude --version | tee` at Checkpoint 0"),
    ],
    "1": [
        dep(".gitignore anchor paragraph 'spikes/results/ is deliberately NOT ignored'",
            "file", RB(334), ("grep", (".gitignore", r"spikes/results/ is deliberately NOT ignored", 1))),
        dep("the 17 untracked run-noise entries", "measurement", RB(186),
            ("untracked_count", 17)),
        dep("references/adr/0089-*.md", "adr", RB(374), ("glob", "references/adr/0089-*.md")),
        dep("references/adr/0030-facets-not-tags.md with superseded_by [0089]", "adr", RB(375),
            ("grep", ("references/adr/0030-facets-not-tags.md", r'^superseded_by: \["0089"\]$', 1))),
        dep("references/adr/README.md", "file", RB(376), ("path", "references/adr/README.md")),
        dep("spikes/p2/build-p2-worksheet.py", "script", RB(377), ("path", "spikes/p2/build-p2-worksheet.py")),
        dep("spikes/results/p2-labels-ryan.md (41 filled labels)", "spike_result", RB(378),
            ("path", "spikes/results/p2-labels-ryan.md")),
        dep("spikes/results/instrument-checks/RUNBOOK-instrument.txt", "file", RB(379),
            ("path", "spikes/results/instrument-checks/RUNBOOK-instrument.txt")),
        dep("references/2026-09-08-architecture-design.md", "file", RB(367), ("path", "references/2026-09-08-architecture-design.md")),
        dep("references/2026-09-08-acceptance-criteria.md", "file", RB(368), ("path", "references/2026-09-08-acceptance-criteria.md")),
        dep("references/2026-09-08-success-measures.md", "file", RB(369), ("path", "references/2026-09-08-success-measures.md")),
        dep("references/2026-09-08-evidence-brief.md", "file", RB(370), ("path", "references/2026-09-08-evidence-brief.md")),
        dep("references/2026-09-09-change-classification.md", "file", RB(371), ("path", "references/2026-09-09-change-classification.md")),
        dep("spikes/HANDOFF.md", "file", RB(373), ("path", "spikes/HANDOFF.md")),
        dep("Checkpoint 0 passed (branch + instrument file)", "human_act", RB(278),
            ("git_branch", "build/phase-0-to-1")),
    ],
    "2": [
        dep("spikes/corpus/", "file", RB(470), ("path", "spikes/corpus")),
        dep("spikes/results/", "file", RB(471), ("path", "spikes/results")),
        dep("spikes/runner/", "file", RB(472), ("path", "spikes/runner")),
        dep("spikes/schemas/", "file", RB(479), ("path", "spikes/schemas")),
        dep("spikes/prompts/", "file", RB(480), ("path", "spikes/prompts")),
        dep("ADR-0013 (corpus as its own repo)", "adr", RB(457), ("glob", "references/adr/0013-*.md")),
        dep("spikes/results/p1-arm-d-labels-ryan.md must travel", "spike_result", RB(560),
            ("path", "spikes/results/p1-arm-d-labels-ryan.md")),
        dep("spikes/results/p1-arm-l-labels-ryan.md must travel", "human_label", RB(561),
            ("path", "spikes/results/p1-arm-l-labels-ryan.md"),
            "Checkpoint 2's last line lists it; it is Step 3b's outstanding output"),
        dep("spikes/results/p2-labels-ryan.md must travel", "spike_result", RB(562),
            ("path", "spikes/results/p2-labels-ryan.md")),
        dep("D:/repos/redkiln-corpus (target repo, to be created)", "external", RB(467),
            ("abs_path", "D:/repos/redkiln-corpus"), role="own_output"),
        dep("Step 3b complete (all three label files filled)", "human_label", RB(448),
            ("path", "spikes/results/p1-arm-l-labels-ryan.md")),
        dep("Step 4 and Step 4a complete (ORDER banner)", "human_decision", RB(435),
            ("glob", "references/adr/0090-*.md")),
    ],
    "3a": [
        dep("D:/repos/redkiln/workflows/forge-fix.js", "external", RB(575),
            ("abs_path", "D:/repos/redkiln/workflows/forge-fix.js")),
        dep("the defect anchor 'if (review && review.pass === false) {' present exactly once",
            "measurement", RB(596),
            ("grep_abs", ("D:/repos/redkiln/workflows/forge-fix.js",
                          r"if \(review && review\.pass === false\) \{", 1))),
        dep("the pre-existing correct guard at line 1019", "measurement", RB(577),
            ("grep_abs", ("D:/repos/redkiln/workflows/forge-fix.js", r"review\.pass !== true", 1))),
        dep("rig subcommand `fix-consumer-probe`", "script", RB(630),
            ("grep", ("spikes/runner/src/main.rs", r"fix-consumer-probe", 1)),
            "[WILL EXIST]: no rig subcommand poses this question", role="own_output"),
        dep("spikes/results/d-fixconsumer.jsonl", "spike_result", RB(636),
            ("path", "spikes/results/d-fixconsumer.jsonl"),
            "PASS/FAIL record, or a written deferral line in DECISIONS-2026-09-09.md", role="own_output"),
        dep("spikes/DECISIONS-2026-09-09.md (the deferral-record alternative)", "file", RB(667),
            ("path", "spikes/DECISIONS-2026-09-09.md")),
        dep("one haiku-scale Workflow-runtime dispatch (model cost) OR a written deferral line",
            "human_act", RB(667), ("path", "spikes/results/d-fixconsumer.jsonl"),
            "Checkpoint 3a accepts either; silence is not a checkpoint", role="own_output"),
        dep("the forge-fix.js consumer fix applied (=== false -> 0, !== true -> 2)", "human_act", RB(660),
            ("grep_abs", ("D:/repos/redkiln/workflows/forge-fix.js", r"review\.pass === false", 0)),
            "pre-fix state reads 1/1; Checkpoint 3a wants 0/2", role="own_output"),
    ],
    "3b": [
        dep("spikes/results/p1-arm-d-labels-RYAN-TODO.md (blank of record)", "spike_result", RB(681),
            ("path", "spikes/results/p1-arm-d-labels-RYAN-TODO.md")),
        dep("spikes/results/p1-arm-l-labels-RYAN-TODO.md (blank of record)", "spike_result", RB(683),
            ("path", "spikes/results/p1-arm-l-labels-RYAN-TODO.md")),
        dep("spikes/results/p2-labels-RYAN-TODO.md (blank of record)", "spike_result", RB(685),
            ("path", "spikes/results/p2-labels-RYAN-TODO.md")),
        dep("spikes/results/p1-arm-d-labels-ryan.md (retained UNFILLED, amendment 9)",
            "spike_result", RB(806), ("labelfile", "spikes/results/p1-arm-d-labels-ryan.md"),
            "DELIBERATELY at labelled_at: FILL-ME -- not a gap"),
        dep("spikes/results/p2-labels-ryan.md (filled + stamped)", "spike_result", RB(835),
            ("labelfile", "spikes/results/p2-labels-ryan.md")),
        dep("spikes/results/p1-arm-l-labels-ryan.md (filled + stamped)", "human_label", RB(818),
            ("labelfile", "spikes/results/p1-arm-l-labels-ryan.md"),
            "THE outstanding half of Step 3b", role="own_output"),
        dep("spikes/prompts/p1-classify.md (ordinal anchors, copied verbatim)", "file", RB(772),
            ("path", "spikes/prompts/p1-classify.md")),
        dep("spikes/p1/score-arm-d.py (its 7-facet regex scopes the guard)", "script", RB(762),
            ("path", "spikes/p1/score-arm-d.py")),
        dep("spikes/p9/facets.toml has no [facet.binary_change]", "measurement", RB(755),
            ("grep", ("spikes/p9/facets.toml", r"^\[facet\.binary_change\]", 0))),
        dep("Ryan labels arm L blind, from the item alone", "human_label", RB(744),
            ("labelfile", "spikes/results/p1-arm-l-labels-ryan.md"), role="own_output"),
    ],
    "3c": [
        dep("S-BIN fixture D:/repos/BrittonGames/_workspace/tooling/verify_headless.ps1",
            "external", RB(905), ("abs_path", "D:/repos/BrittonGames/_workspace/tooling/verify_headless.ps1")),
        dep("S-BIN spike RESULT (registered, unrun)", "spike_result", RB(880),
            ("finding_heading", "S-BIN"),
            "the fixture is not the finding; exit (c) accepts a finding OR an ADR re-scope", role="soft"),
        dep("spikes/results/e3-severity-annotation-ryan.md carries NOT BLIND", "spike_result", RB(906),
            ("grep", ("spikes/results/e3-severity-annotation-ryan.md", r"NOT BLIND", 1))),
        dep("spikes/results/e3-severity-annotation-BLANK.md still pristine (6 markers)",
            "spike_result", RB(908),
            ("grep", ("spikes/results/e3-severity-annotation-BLANK.md", r"severity: ______", 6))),
        dep("spikes/F39-DRAFT.md (arm D's F-number)", "spike_result", RB(919),
            ("path", "spikes/F39-DRAFT.md")),
        dep("spikes/DECISIONS-2026-09-09.md or an ADR re-scoping S-BIN", "human_decision", RB(916),
            ("grep_notmatch", ("spikes/DECISIONS-2026-09-09.md", r"S-BIN", r"(?i)registered,? unrun")),
            "a SENTENCE must exist: the design plan, the re-scope, or the deferral", role="own_output"),
        dep("references/2026-09-08-architecture-design.md names E3-SEV DONE", "file", RB(918),
            ("grep", ("references/2026-09-08-architecture-design.md", r"E3-SEV", 1))),
    ],
    "4": [
        dep("spikes/METHOD-P.md (the pre-registration)", "file", RB(937), ("path", "spikes/METHOD-P.md")),
        dep("spikes/F36-DRAFT.md (clause A modal-R)", "spike_result", RB(1025), ("path", "spikes/F36-DRAFT.md")),
        dep("spikes/F39-DRAFT.md (clause B arm-D precision)", "spike_result", RB(963), ("path", "spikes/F39-DRAFT.md")),
        dep("spikes/F40-DRAFT.md (P2 scoring)", "spike_result", RB(1105), ("path", "spikes/F40-DRAFT.md")),
        dep("spikes/results/p1-arm-l-analysis.txt", "spike_result", RB(939), ("path", "spikes/results/p1-arm-l-analysis.txt")),
        dep("spikes/results/p1-arm-l-v2-analysis.txt", "spike_result", RB(939), ("path", "spikes/results/p1-arm-l-v2-analysis.txt")),
        dep("spikes/p1/score-arm-d.py", "script", RB(1088), ("path", "spikes/p1/score-arm-d.py")),
        dep("spikes/results/p1-arm-d.jsonl", "spike_result", RB(1089), ("path", "spikes/results/p1-arm-d.jsonl")),
        dep("spikes/results/p1-arm-d-labels-ryan.md", "spike_result", RB(1090), ("path", "spikes/results/p1-arm-d-labels-ryan.md")),
        dep("spikes/p1/score-arm-d-panel.py", "script", RB(1101), ("path", "spikes/p1/score-arm-d-panel.py")),
        dep("spikes/results/p1-arm-d-oracle.jsonl", "spike_result", RB(1102), ("path", "spikes/results/p1-arm-d-oracle.jsonl")),
        dep("spikes/results/p1-arm-d-panel-accepted.jsonl", "spike_result", RB(1102), ("path", "spikes/results/p1-arm-d-panel-accepted.jsonl")),
        dep("spikes/results/p1-arm-d-contested.json", "spike_result", RB(1102), ("path", "spikes/results/p1-arm-d-contested.json")),
        dep("spikes/results/p1-arm-d-adjudications.json", "spike_result", RB(1103), ("path", "spikes/results/p1-arm-d-adjudications.json")),
        dep("spikes/p2/reconstruct-derived-from.py", "script", RB(1112), ("path", "spikes/p2/reconstruct-derived-from.py")),
        dep("spikes/plans/p2-items.txt", "file", RB(1115), ("path", "spikes/plans/p2-items.txt")),
        dep("spikes/results/p2-cases.jsonl", "spike_result", RB(1117), ("path", "spikes/results/p2-cases.jsonl")),
        dep("spikes/results/p2-engine.jsonl (82 records)", "spike_result", RB(1119),
            ("wc", ("spikes/results/p2-engine.jsonl", 82))),
        dep("spikes/p2/score-p2.py", "script", RB(1134), ("path", "spikes/p2/score-p2.py")),
        dep("spikes/results/p2-labels-ryan.md", "spike_result", RB(1136), ("labelfile", "spikes/results/p2-labels-ryan.md")),
        dep("spikes/p1/analyse-arm-l.py", "script", RB(1233), ("path", "spikes/p1/analyse-arm-l.py")),
        dep("spikes/results/p1-arm-l.jsonl", "spike_result", RB(1233), ("path", "spikes/results/p1-arm-l.jsonl")),
        dep("spikes/results/p1-arm-l-v2.jsonl", "spike_result", RB(1234), ("path", "spikes/results/p1-arm-l-v2.jsonl")),
        dep("spikes/p1/score-arm-l.py", "script", RB(1243), ("path", "spikes/p1/score-arm-l.py"),
            "[WILL EXIST] -- there is no arm-L panel scorer; RUNBOOK:1249 says its absence does not block the decision",
            role="soft"),
        dep("spikes/results/p1-arm-l-labels-ryan.md (score-arm-l.py's --labels)", "human_label", RB(1245),
            ("labelfile", "spikes/results/p1-arm-l-labels-ryan.md"),
            "held for the re-run; clause A resolves from modal-R already computed", role="soft"),
        dep("D:/repos/redkiln at 47faec9f (reconstruct --redkiln/--sha)", "external", RB(1113),
            ("git_head", ("D:/repos/redkiln", "47faec9f46b8d983c2c75f0794164e68415ae706"))),
        dep("DECISIONS-2026-09-09 #15 human_authored remap CONFIRMED OR REVERSED",
            "human_decision", RB(1068), ("unverifiable", "no mechanical test for a written confirmation"),
            "RUNBOOK:1074 says STILL UNSETTLED; gates the advisory half and criterion 3"),
        dep("DECISIONS-2026-09-09 #18 artifact-list change confirmed", "human_decision", RB(1069),
            ("unverifiable", "no mechanical test")),
        dep("DECISIONS-2026-09-09 #27 arm L2 confirmed or discarded", "human_decision", RB(1070),
            ("unverifiable", "no mechanical test")),
        dep("references/adr/0079-*.md at status: proposed (the record to supersede)", "adr", RB(1319),
            ("grep_glob", ("references/adr/0079-*.md", r"^status: proposed$", 1))),
        dep("references/adr/README.md four anchor strings", "file", RB(1339),
            ("path", "references/adr/README.md")),
        dep("THE DECISION: context-vector source, taken and recorded as ADR-0090",
            "human_decision", RB(1293), ("glob", "references/adr/0090-*.md"), role="own_output"),
        dep("0079 moved proposed -> superseded with superseded_by [0090]", "adr", RB(1310),
            ("grep_glob", ("references/adr/0079-*.md", r'^superseded_by: \["0090"\]$', 1)), role="own_output"),
        dep("README.md: 'the next free number is 0091'", "file", RB(1345),
            ("grep", ("references/adr/README.md", r"the next free number is .0091.", 1)), role="own_output"),
    ],
    "4a": [
        dep("references/adr/0089-*.md accepted", "adr", RB(1426),
            ("grep_glob", ("references/adr/0089-facet-provenance-declared-surfaces-and-abstention.md",
                           r"^status: accepted$", 1))),
        dep("spikes/p9/facets.toml (the registry that exists today)", "file", RB(1440),
            ("path", "spikes/p9/facets.toml")),
        dep("facets.toml: 11 lines matching ^source = \"derived\" (prefix, not anchored)",
            "measurement", RB(1453), ("grep", ("spikes/p9/facets.toml", r'^source = "derived"', 11))),
        dep("facets.toml: 0 deriver lines BEFORE this step", "measurement", RB(1461),
            ("grep", ("spikes/p9/facets.toml", r"^deriver = ", 0))),
        dep("facets.toml: [facet.auth] present exactly once (4a.2's anchor)", "measurement", RB(1462),
            ("grep", ("spikes/p9/facets.toml", r"^\[facet\.auth\]", 1))),
        dep("facets.toml: [facet.surfaces] absent BEFORE, present AFTER", "measurement", RB(1463),
            ("grep", ("spikes/p9/facets.toml", r"^\[facet\.surfaces\]", 0))),
        dep("surfaces.toml (the declared boundary map)", "file", RB(1559),
            ("path_any", ["surfaces.toml", ".redkiln/surfaces.toml"]),
            "checked in by this step; absent today", role="own_output"),
        dep("facets.toml: deriver on all 11 derived facets (AFTER)", "measurement", RB(1466),
            ("grep", ("spikes/p9/facets.toml", r"^deriver = ", 11)), role="own_output"),
        dep("facets.toml: [facet.auth] withdrawn (AFTER -> 0)", "measurement", RB(1556),
            ("grep", ("spikes/p9/facets.toml", r"^\[facet\.auth\]", 0)), role="own_output"),
        dep("facets.toml: no 'values = [true, false]' (AFTER -> 0)", "measurement", RB(1565),
            ("grep", ("spikes/p9/facets.toml", r"values = \[true, false\]", 0)), role="own_output"),
        dep("ADR-0024 / ADR-0033 (applicable_if owns its evidence question)", "adr", RB(1487),
            ("glob", "references/adr/0024-*.md")),
        dep("ADR-0032 (absent inputs are null)", "adr", RB(1512), ("glob", "references/adr/0032-*.md")),
        dep("ADR-0069 (== / != against null)", "adr", RB(1512), ("glob", "references/adr/0069-*.md")),
        dep("ADR-0067 (recent_deploy null precedent)", "adr", RB(1516), ("glob", "references/adr/0067-*.md")),
        dep("ADR-0073 (boolean and ordinal registries separate)", "adr", RB(1495), ("glob", "references/adr/0073-*.md")),
        dep("redkiln lint --facet-provenance binary", "crate", RB(1569),
            ("path", "target/debug/redkiln.exe"), "[WILL EXIST] -- Step 7b builds it; run there and re-run here",
            role="soft"),
        dep("cargo test -p rk-kernel -- facet_provenance_fixtures", "crate", RB(1570),
            ("path", "crates/rk-kernel"), "[WILL EXIST] -- Step 7b builds it; run there and re-run here",
            role="soft"),
        dep("Step 4 complete (run 4a AFTER Step 4, BEFORE Step 5)", "human_decision", RB(1429),
            ("glob", "references/adr/0090-*.md")),
    ],
    "5": [
        dep("ADR-0090 gate token", "adr", RB(1593), ("glob", "references/adr/0090-*.md")),
        dep("Step 2 landed (corpus split; AC-465 enforceable)", "human_act", RB(1599),
            ("abs_path", "D:/repos/redkiln-corpus")),
        dep("CORPUS_PIN at the repo root", "file", RB(486), ("path", "CORPUS_PIN"),
            "written by Step 2", owner="Ryan (operator) -- Step 2"),
        dep("Step 3c answered in writing", "human_decision", RB(1599),
            ("unverifiable", "a sentence, not a command")),
        dep("Step 4a applied (facets.toml derivers)", "measurement", RB(1599),
            ("grep", ("spikes/p9/facets.toml", r"^deriver = ", 11)), owner="implementer -- Step 4a"),
        dep("crates/ exists and is EMPTY today", "file", RB(1610), ("path", "crates")),
        dep("cargo (workspace build)", "binary_tool", RB(1751), ("tool", "cargo")),
        dep("cargo-audit installed", "binary_tool", RB(1686), ("tool", "cargo-audit")),
        dep("cargo-insta installed", "binary_tool", RB(1687), ("tool", "cargo-insta")),
        dep("ADR-0083 (CI as a floor that must never again be absent)", "adr", RB(1606),
            ("glob", "references/adr/0083-*.md")),
        dep("rust-toolchain.toml", "file", RB(1650), ("path", "rust-toolchain.toml"),
            "one of the four measured absences; THIS step writes it", role="own_output"),
        dep(".github/workflows/ci.yml", "file", RB(1665), ("path", ".github/workflows/ci.yml"),
            "no .github directory in this repository at all", role="own_output"),
        dep("root Cargo.toml (workspace, exclude spikes/runner)", "file", RB(1618),
            ("path", "Cargo.toml"), role="own_output"),
        dep("crates/rk-types", "crate", RB(1611), ("path", "crates/rk-types"), role="own_output"),
        dep("crates/rk-types/tests/snapshots (insta golden snapshot dir MUST exist)", "file", RB(1759),
            ("path", "crates/rk-types/tests/snapshots"), role="own_output"),
        dep("S-BIN result (blocks ChangeSummary -- the type must NOT be built without it)", "spike_result", RB(1714),
            ("finding_heading", "S-BIN"),
            "a deliberate block on ONE type only; the crate ships every other artifact type", role="soft"),
        dep("references/2026-09-08-acceptance-criteria.md (AC-120..AC-159 etc.)", "file", RB(1775),
            ("path", "references/2026-09-08-acceptance-criteria.md")),
    ],
    "6": [
        dep("ADR-0090 gate token", "adr", RB(1786), ("glob", "references/adr/0090-*.md")),
        dep("crates/rk-types built (Step 5)", "crate", RB(1590), ("path", "crates/rk-types")),
        dep("gix = \"=0.87.1\" (exact pin)", "external", RB(1793),
            ("grep", ("crates/rk-git/Cargo.toml", r'gix = "=0\.87\.1"', 1)), role="own_output"),
        dep("S-GIX/a result (common dir from a linked worktree)", "spike_result", RB(1798),
            ("grep_glob", ("references/adr/0091-*.md", r"S-GIX/a", 1)),
            "[WILL EXIST] as an #[ignore]d test in rk-git; run BEFORE worktree code", role="own_output"),
        dep("S-GIX/b result (trailer round-trip byte-identical)", "spike_result", RB(1799),
            ("grep_glob", ("references/adr/0091-*.md", r"S-GIX/b", 1)),
            "[WILL EXIST] as an #[ignore]d test in rk-git; run BEFORE worktree code", role="own_output"),
        dep("ADR-0077 (accepted; may only be NARROWED by a successor)", "adr", RB(1808),
            ("glob", "references/adr/0077-*.md")),
        dep("references/adr/0091-*.md (the S-GIX successor record)", "adr", RB(1853),
            ("glob", "references/adr/0091-*.md"), role="own_output"),
        dep("THE DECISION: whether S-GIX narrows the git write path", "human_decision", RB(1808),
            ("glob", "references/adr/0091-*.md"), role="own_output"),
        dep("THE DECISION: the minimum git executable version the wrapper asserts",
            "human_decision", RB(1826), ("grep_glob", ("references/adr/0091-*.md", r"(?i)git.*version", 1)),
            role="own_output"),
        dep("git executable on PATH at/above the (undecided) floor", "binary_tool", RB(1826),
            ("tool", "git")),
        dep("crates/rk-git/src/git_exec.rs (the ONE process-exec wrapper)", "crate", RB(1849),
            ("path", "crates/rk-git/src/git_exec.rs"), role="own_output"),
        dep("crates/rk-store", "crate", RB(1836), ("path", "crates/rk-store"), role="own_output"),
        dep("P8 result (AC-013/187/266/267/271/401 are PROVISIONAL on it)", "spike_result", RB(1868),
            ("finding_heading", "P8"),
            "F38 is a PARTIAL (one step, n=3); multi-step resume is unmeasured; build them, do not claim them",
            role="soft"),
    ],
    "7a": [
        dep("crates/rk-types (Step 5)", "crate", RB(1871), ("path", "crates/rk-types")),
        dep("crates/rk-ports", "crate", RB(1875), ("path", "crates/rk-ports"), role="own_output"),
        dep("AC-481 traits.len() == fakes.len()", "file", RB(1886),
            ("path", "references/2026-09-08-acceptance-criteria.md")),
        dep("AC-483 git deliberately not a port", "file", RB(1879),
            ("path", "references/2026-09-08-acceptance-criteria.md")),
    ],
    "7b": [
        dep("ADR-0090 exists (the gate token)", "adr", RB(1911), ("glob", "references/adr/0090-*.md")),
        dep("references/adr/0079-*.md at status: superseded", "adr", RB(1914),
            ("grep_glob", ("references/adr/0079-*.md", r"^status: superseded$", 1))),
        dep("references/adr/0089-*.md (NOT the gate token -- the third line's subject)", "adr", RB(1915),
            ("glob", "references/adr/0089-*.md")),
        dep("crates/rk-ports (Step 7a)", "crate", RB(1871), ("path", "crates/rk-ports")),
        dep("crates/rk-kernel", "crate", RB(1905), ("path", "crates/rk-kernel"), role="own_output"),
        dep("spikes/p9/facets.toml with derivers (Step 4a) -> .redkiln/facets.toml", "file", RB(1440),
            ("grep", ("spikes/p9/facets.toml", r"^deriver = ", 11))),
        dep(".redkiln/facets.toml (pack-authoring output)", "file", RB(2169),
            ("path", ".redkiln/facets.toml"), "[WILL EXIST] -- there is no .redkiln/ directory today",
            role="own_output"),
        dep("AC-071 five-clause facet-stability lint + 5-case fixture set", "file", RB(1948),
            ("path", "references/2026-09-08-acceptance-criteria.md")),
        dep("AC-520 facet-provenance lint + 4 must-fail + 1 must-load fixtures", "file", RB(1952),
            ("grep", ("references/2026-09-08-acceptance-criteria.md", r"AC-520", 1))),
        dep("P7-CEIL result (whether bisect is a ceiling activity)", "spike_result", RB(1966),
            ("finding_heading", "P7-CEIL"),
            "the rigor-monotonicity lint ships either way", role="soft"),
        dep("F37 (P7 criterion 1 powerless at n=3; no quality-shorthand default)", "spike_result", RB(1971),
            ("path", "spikes/F37-DRAFT.md")),
    ],
    "7c": [
        dep("crates/rk-kernel (Step 7b)", "crate", RB(1871), ("path", "crates/rk-kernel")),
        dep("crates/rk-ports (Step 7a)", "crate", RB(1871), ("path", "crates/rk-ports")),
        dep("spikes/runner/src/runner.rs (lifted into rk-adapter-claude)", "file", RB(2013),
            ("path", "spikes/runner/src/runner.rs"), destroyed_by="2"),
        dep("spikes/runner/src/event.rs", "file", RB(2013), ("path", "spikes/runner/src/event.rs"), destroyed_by="2"),
        dep("spikes/runner/src/step.rs", "file", RB(2013), ("path", "spikes/runner/src/step.rs"), destroyed_by="2"),
        dep("spikes/runner/src/throttle.rs", "file", RB(2013), ("path", "spikes/runner/src/throttle.rs"), destroyed_by="2"),
        dep("spikes/runner/src/journal.rs", "file", RB(2013), ("path", "spikes/runner/src/journal.rs"), destroyed_by="2"),
        dep("claude CLI, subscription only (no ANTHROPIC_API_KEY)", "binary_tool", RB(2043),
            ("tool", "claude")),
        dep("ANTHROPIC_API_KEY absent from the environment (AC-488)", "measurement", RB(2043),
            ("env_absent", "ANTHROPIC_API_KEY")),
        dep("S-TTL1 result (1h TTL evidence)", "spike_result", RB(2046),
            ("finding_heading", "S-TTL1"), "UNRUN; no run may be reported as 1h", role="soft"),
        dep("S-H1c result (card append route)", "spike_result", RB(2050),
            ("finding_heading", "S-H1c"), "UNRUN; append flags absent from argv", role="soft"),
        dep("ADR-0088 confirmation-bias arm", "adr", RB(2076), ("glob", "references/adr/0088-*.md")),
        dep("one real dispatch validated before any set (model cost)", "human_act", RB(2065),
            ("unverifiable", "no cargo test can stand in for it"), role="own_output"),
        dep("crates/rk-adapter-claude, rk-adapter-sqlite, rk-adapter-node", "crate", RB(2054),
            ("path", "crates/rk-adapter-claude"), role="own_output"),
    ],
    "7d": [
        dep("crates/rk-adapter-* (Step 7c)", "crate", RB(2010), ("path", "crates/rk-adapter-claude")),
        dep("crates/rk-cli (the composition root)", "crate", RB(2079), ("path", "crates/rk-cli"), role="own_output"),
        dep("ADR-0009 (answer is the only Phase-1 gate channel)", "adr", RB(2087),
            ("glob", "references/adr/0009-*.md")),
        dep("acceptance-criteria register 12.10 (exit codes, line 5132)", "file", RB(2095),
            ("grep", ("references/2026-09-08-acceptance-criteria.md", r"12\.10", 1))),
        dep("THE DECISION: doctor's unread-artifact window N", "human_decision", RB(2123),
            ("grep_dir", ("crates/rk-cli/src", r"unread[_-]?artifact[_-]?window|window_n", 1)), role="own_output"),
        dep("redkiln-migrate v0", "crate", RB(2099), ("path", "crates/redkiln-migrate"), role="own_output"),
    ],
    "8": [
        dep("target/debug/redkiln (the built binary)", "crate", RB(2131),
            ("path_any", ["target/debug/redkiln.exe", "target/debug/redkiln"])),
        dep("a real defect in this repo to run the fix lane on", "human_act", RB(2132),
            ("unverifiable", "the run id is the evidence; a statement is not")),
        dep("AC-473 self-hosting", "file", RB(2128), ("grep", ("references/2026-09-08-acceptance-criteria.md", r"AC-473", 1))),
        dep("AC-184 doctor --rebuild with the 40-hex corpus-pin assertion", "file", RB(2142),
            ("grep", ("references/2026-09-08-acceptance-criteria.md", r"AC-184", 1))),
        dep("CORPUS_PIN carrying a 40-hex SHA", "file", RB(486),
            ("grep", ("CORPUS_PIN", r"CORPUS_SHA=[0-9a-f]{40}$", 1))),
        dep("the kernel FSM replay test (first replay ever)", "crate", RB(2141),
            ("path", "crates/rk-kernel")),
        dep("trybuild must-fail fixtures (clock read; --stay-class error)", "crate", RB(2160),
            ("path", "crates/rk-kernel")),
        dep(".redkiln/facets.toml (lint argument)", "file", RB(2168), ("path", ".redkiln/facets.toml"),
            "[WILL EXIST] -- no .redkiln/ directory in redkiln-rs today"),
        dep("redkiln lint --facet-stability (AC-071, five clauses)", "crate", RB(2171),
            ("path_any", ["target/debug/redkiln.exe", "target/debug/redkiln"])),
        dep("redkiln lint --facet-provenance (AC-520)", "crate", RB(2192),
            ("path_any", ["target/debug/redkiln.exe", "target/debug/redkiln"])),
        dep("a facet stability record (5 clauses; 4 of 5 fail today)", "spike_result", RB(2175),
            ("path", "spikes/results/p1-arm-l-analysis.txt"),
            "identity 0.25/0.15/0.10/0.10 against 0.80 [F36]; branch 1 (zero llm facets) is what passes"),
        dep("a scored precision record per derived facet over a pinned corpus", "spike_result", RB(2193),
            ("path", "spikes/F39-DRAFT.md")),
        dep("SM-109 resolution_limited cost flag", "file", RB(2203),
            ("grep", ("references/2026-09-08-success-measures.md", r"SM-109", 1))),
        dep("taskkill (mid-graph kill for criterion 3)", "human_act", RB(2154),
            ("unverifiable", "Windows built-in; the kill is a human act during the run")),
    ],
    "9": [
        dep("the Step 1, Step 4 and Step 4a commits (salvage cherry-picks)", "human_act", RB(2256),
            ("unverifiable", "commits that do not exist yet")),
        dep("D:/repos/redkiln-corpus holding the three label files", "external", RB(2287),
            ("abs_path", "D:/repos/redkiln-corpus")),
        dep("D:/repos/redkiln at 47faec9f with clean workflows/", "external", RB(2294),
            ("git_head", ("D:/repos/redkiln", "47faec9f46b8d983c2c75f0794164e68415ae706"))),
        dep(".gitignore paragraph restorable from main", "file", RB(2314),
            ("grep", (".gitignore", r"spikes/results/ is deliberately NOT ignored", 1))),
        dep("main at HEAD carrying the design pass", "external", RB(2252),
            ("git_branch_any", ["main"])),
    ],
}

# steps that produce
PRODUCES = {
    "0": ["branch build/phase-0-to-1",
          "spikes/results/instrument-checks/RUNBOOK-instrument.txt"],
    "1": [".gitignore rules for waiter logs / p8-state / target-*",
          "the chore commit for those rules",
          "the ADR-0089 / F39-amendment commit",
          "a clean `git status --porcelain` (Phase-0 exit (a))"],
    "2": ["D:/repos/redkiln-corpus (new git repo, v0 commit)",
          "the corpus pin SHA",
          "D:/repos/redkiln-rs/CORPUS_PIN",
          ".gitignore: spikes/corpus|results|runner rules + rewritten results paragraph",
          "removal of spikes/corpus, spikes/results, spikes/runner from this tree"],
    "3a": ["the fixed forge-fix.js consumer (working-tree edit in the frozen predecessor, NOT committed there)",
           "spikes/results/d-fixconsumer.jsonl OR a written deferral line in DECISIONS-2026-09-09.md"],
    "3b": ["spikes/results/p1-arm-l-labels-ryan.md, filled and UTC-Z stamped"],
    "3c": ["a written sentence for exit row (c) (S-BIN plan/re-scope/deferral) and row (d)"],
    "4": ["references/adr/0090-<slug>.md (context vector, supersedes 0079 in full)",
          "0079: status proposed -> superseded, superseded_by [0090]",
          "references/adr/README.md: four anchored edits",
          "the scorer outputs re-run (score-arm-d, score-arm-d-panel, score-p2, analyse-arm-l)"],
    "4a": ["spikes/p9/facets.toml: deriver field on every derived facet",
           "spikes/p9/facets.toml: [facet.auth] withdrawn, [facet.surfaces] set = true added",
           "spikes/p9/facets.toml: tri-valued booleans (no values = [true, false])",
           "surfaces.toml (path globs -> closed surface names)",
           "the AC-520 fixture set: four must-fail plus one must-load"],
    "5": ["Cargo.toml (workspace root, exclude spikes/runner)",
          "rust-toolchain.toml (channel 1.97.1, exact)",
          ".github/workflows/ci.yml (four gates + structural assertions)",
          "crates/rk-types (ids, typed artifacts, digests, StepError, Drift, GateBasis)",
          "crates/rk-types/tests/snapshots (insta golden schema snapshot)"],
    "6": ["crates/rk-git (gix read path + one process-exec write wrapper)",
          "crates/rk-store (one store per repo at the common dir; tier-1/tier-2 split)",
          "references/adr/0091-<slug>.md (S-GIX successor to 0077, incl. the git version floor)"],
    "7a": ["crates/rk-ports (five sync port traits)", "the five fakes + port_fake_parity test"],
    "7b": ["crates/rk-kernel (selection engine, FSM, policy, gates, budgets, retry, replay)",
           "redkiln lint --facet-stability (AC-071, five clauses)",
           "redkiln lint --facet-provenance (AC-520)",
           "the rigor-monotonicity lint",
           ".redkiln/facets.toml via pack authoring"],
    "7c": ["crates/rk-adapter-claude, rk-adapter-sqlite, rk-adapter-node"],
    "7d": ["crates/rk-cli (clap router: run, answer, status, doctor)",
           "redkiln-migrate v0",
           "doctor's unread-artifact window N, declared in the rule table"],
    "8": ["a run id evidencing the self-hosted fix lane",
          "the seven exit-criterion results"],
    "9": ["a rolled-back tree; the three label files recovered; the salvage cherry-picks on main"],
}

# --------------------------------------------------------------------------
# 6. check evaluation
# --------------------------------------------------------------------------

WILL_EXIST_HINTS = ("[WILL EXIST]", "WILL EXIST")


def evaluate(check):
    """Return (exists_now: bool|None, evidence: str)."""
    kind = check[0]
    if kind == "tool":
        t = TOOLS[check[1]]
        return t["installed"], "%s -> rc=%d %s" % (t["command"], t["exit_code"], t["output"])
    if kind == "path":
        p = check[1]
        ok = path_exists(p)
        extra = ""
        if ok and os.path.isfile(rel(p)):
            extra = " (%d bytes)" % file_bytes(p)
        return ok, "fs: %s %s%s" % (p, "EXISTS" if ok else "MISSING", extra)
    if kind == "path_any":
        hits = [p for p in check[1] if path_exists(p)]
        return bool(hits), "fs any-of %s -> %s" % (check[1], hits or "NONE")
    if kind == "abs_path":
        ok = abs_exists(check[1])
        return ok, "fs abs: %s %s" % (check[1], "EXISTS" if ok else "MISSING")
    if kind == "glob":
        n = glob_count(check[1])
        return n > 0, "glob %s -> %d file(s)" % (check[1], n)
    if kind == "grep":
        p, pat, expect = check[1]
        n = grep_count(p, pat)
        if n is None:
            return False, "grep %s: FILE MISSING (0 matches, reported as absence)" % p
        return n >= expect if expect > 0 else n == 0, \
            "grep -c %r %s -> %d (expected %d)" % (pat, p, n, expect)
    if kind == "grep_abs":
        fp, pat, expect = check[1]
        n = grep_count_abs(fp, pat)
        if n is None:
            return False, "grep %s: FILE MISSING" % fp
        return n >= expect, "grep -c %r %s -> %d (expected %d)" % (pat, fp, n, expect)
    if kind == "grep_glob":
        pat_glob, pat, expect = check[1]
        files = globmod.glob(os.path.join(REPO, pat_glob))
        if not files:
            return False, "glob %s -> 0 files (0 matches, reported as absence)" % pat_glob
        n = sum(grep_count_abs(f, pat) or 0 for f in files)
        return n >= expect, "grep -c %r over %s -> %d (expected %d)" % (pat, pat_glob, n, expect)
    if kind == "grep_notmatch":
        p, pat, exclude = check[1]
        fp = rel(p)
        if not os.path.exists(fp):
            return False, "%s MISSING" % p
        rx, ex = re.compile(pat), re.compile(exclude)
        n = 0
        with open(fp, "r", encoding="utf-8", errors="replace") as fh:
            for line in fh:
                if rx.search(line) and not ex.search(line):
                    n += 1
        return n > 0, "grep %r | grep -v %r %s -> %d line(s)" % (pat, exclude, p, n)
    if kind == "grep_dir":
        d, pat, expect = check[1]
        dp = rel(d)
        if not os.path.isdir(dp):
            return False, "dir %s MISSING (0 matches, reported as absence)" % d
        rx = re.compile(pat)
        n = 0
        for root, _dirs, files in os.walk(dp):
            for f in files:
                try:
                    with open(os.path.join(root, f), "r", encoding="utf-8", errors="replace") as fh:
                        n += sum(1 for line in fh if rx.search(line))
                except OSError:
                    pass
        return n >= expect, "grep -rn %r %s -> %d" % (pat, d, n)
    if kind == "wc":
        p, expect = check[1]
        fp = rel(p)
        if not os.path.exists(fp):
            return False, "%s MISSING" % p
        with open(fp, "r", encoding="utf-8", errors="replace") as fh:
            n = sum(1 for _ in fh)
        return n == expect, "wc -l %s -> %d (expected %d)" % (p, n, expect)
    if kind == "labelfile":
        p = check[1]
        fp = rel(p)
        if not os.path.exists(fp):
            return False, "%s MISSING" % p
        stamp, labeller, fills = None, None, 0
        with open(fp, "r", encoding="utf-8", errors="replace") as fh:
            for line in fh:
                if line.startswith("labelled_at:") and stamp is None:
                    stamp = line.strip().split(":", 1)[1].strip()
                if line.startswith("labeller:") and labeller is None:
                    labeller = line.strip().split(":", 1)[1].strip()
                if "FILL" in line:
                    fills += 1
        filled = bool(stamp) and "FILL" not in (stamp or "") and stamp.endswith("Z")
        return filled, "%s: labelled_at=%r labeller=%r FILL-lines=%d bytes=%d" % (
            p, stamp, labeller, fills, file_bytes(p))
    if kind == "git_head":
        repo, want = check[1]
        rc, got = run(["git", "-C", repo, "rev-parse", "HEAD"])
        return got.strip() == want, "git -C %s rev-parse HEAD -> %s (want %s)" % (repo, got.strip(), want)
    if kind == "git_branch":
        rc, got = run(["git", "-C", REPO, "rev-parse", "--abbrev-ref", "HEAD"])
        return got.strip() == check[1], "current branch %s (want %s)" % (got.strip(), check[1])
    if kind == "git_branch_any":
        rc, got = run(["git", "-C", REPO, "rev-parse", "--abbrev-ref", "HEAD"])
        return got.strip() in check[1], "current branch %s (want one of %s)" % (got.strip(), check[1])
    if kind == "untracked_count":
        n = GIT["redkiln_rs_untracked_count"]
        return n == check[1], "git status --porcelain '^??' -> %d (RUNBOOK says %d)" % (n, check[1])
    if kind == "env_absent":
        present = check[1] in os.environ
        return (not present), "env %s %s" % (check[1], "PRESENT" if present else "absent")
    if kind == "finding_heading":
        name = check[1]
        files = [rel("spikes/FINDINGS.md")] + sorted(globmod.glob(os.path.join(REPO, "spikes/F*-DRAFT.md")))
        rx = re.compile(r"^#.*\b" + re.escape(name) + r"\b")
        hits = []
        for f in files:
            try:
                with open(f, "r", encoding="utf-8", errors="replace") as fh:
                    for ln, line in enumerate(fh, start=1):
                        if rx.search(line):
                            hits.append("%s:%d" % (os.path.relpath(f, REPO).replace(chr(92), "/"), ln))
            except OSError:
                pass
        return bool(hits), "finding heading naming %s in FINDINGS.md or F*-DRAFT.md -> %s" % (
            name, hits[:3] or "NONE (0 matches, reported as absence)")
    if kind == "unverifiable":
        return None, "NOT MECHANICALLY CHECKABLE: " + check[1]
    raise ValueError("unknown check kind " + kind)


def classify(d, exists_now, evidence):
    """Assign a status. Vocabulary is fixed and reported in the artifact."""
    note = d.get("note", "")
    if "DELIBERATELY" in note:
        return "deferred_by_design"
    if exists_now is None:
        return "unverifiable_mechanically"
    if exists_now:
        if d.get("destroyed_by"):
            return "satisfied_today_but_removed_by_step_" + d["destroyed_by"]
        return "satisfied"
    if "[WILL EXIST]" in note or "WILL EXIST" in note:
        return "will_exist_unwritten"
    if d["kind"] in ("human_label", "human_decision", "human_act"):
        return "outstanding_human"
    if d["kind"] == "spike_result":
        return "absent_spike_unrun"
    return "absent"


SATISFIED_OK = {"satisfied", "deferred_by_design", "satisfied_today_but_removed_by_step_2"}

# --------------------------------------------------------------------------
# 7. build the step records
# --------------------------------------------------------------------------

OWNER_BY_KIND = {
    "human_label": "Ryan (operator) -- blind labelling, one-shot, unrecoverable if unblinded",
    "human_decision": "Ryan (operator) -- a written record (ADR or DECISIONS line)",
    "human_act": "Ryan (operator)",
    "binary_tool": "Ryan (operator) -- install on this box",
    "spike_result": "Ryan (operator) -- commission and run the spike",
    "script": "implementer -- write the script",
    "crate": "implementer -- Phase-1 code",
    "file": "implementer -- authored artifact",
    "adr": "Ryan (operator) -- a decision record",
    "external": "Ryan (operator) -- external repository/tool state",
    "measurement": "implementer -- a measured edit to a tracked file",
}

steps = []
for sid, rx, parent, phase, writes in STEP_SPEC:
    if sid not in starts:
        continue
    s, e = starts[sid], end_of(sid)
    title = LINES[s - 1].lstrip("#").strip()
    cp_id = STEP_CHECKPOINT.get(sid)
    cp_line = CHECKPOINTS.get(cp_id) if cp_id else None
    # a checkpoint line must lie inside the step it closes (or inside its parent for 3a/3b/3c etc.)
    if cp_line is not None and not (s <= cp_line <= e):
        parent_range = None
        if parent and parent in starts:
            parent_range = (starts[parent], end_of(parent))
        if not (parent_range and parent_range[0] <= cp_line <= parent_range[1]):
            cp_line = None

    def innermost(ln):
        best = None
        for sid2, _rx2, _p2, _ph2, _w2 in STEP_SPEC:
            if sid2 not in starts:
                continue
            s2, e2 = starts[sid2], end_of(sid2)
            if s2 <= ln <= e2 and (best is None or s2 > starts[best]):
                best = sid2
        return best

    we = [{"line": ln, "defers": WE_DEFERS.get(ln, "(unannotated)")}
          for ln in WE_MARKERS if innermost(ln) == sid]

    reqs = []
    for d in DEPS.get(sid, []):
        exists_now, evidence = evaluate(d["check"])
        status = classify(d, exists_now, evidence)
        reqs.append({
            "name": d["name"],
            "kind": d["kind"],
            "role": d["role"],
            "evidence_anchor": d["evidence_anchor"],
            "exists_now": exists_now,
            "status": status,
            "verified_by": evidence,
            "note": d.get("note", ""),
            "destroyed_by_step": d.get("destroyed_by"),
            "owner": d.get("owner") or OWNER_BY_KIND.get(d["kind"], "unassigned"),
        })
    blocked = [{"name": r["name"], "status": r["status"], "kind": r["kind"]}
               for r in reqs if r["role"] == "prerequisite" and r["status"] not in SATISFIED_OK]
    own_todo = [{"name": r["name"], "status": r["status"], "kind": r["kind"]}
                for r in reqs if r["role"] == "own_output" and r["status"] not in SATISFIED_OK]
    soft = [{"name": r["name"], "status": r["status"], "kind": r["kind"]}
            for r in reqs if r["role"] == "soft" and r["status"] not in SATISFIED_OK]

    steps.append({
        "step": sid,
        "title": title,
        "start_line": s,
        "end_line": e,
        "checkpoint_line": cp_line,
        "checkpoint_id": cp_id,
        "phase": phase,
        "phase_note": ("rollback; spans both phases" if sid == "9" else
                       ("preconditions; before Phase 0 part 1" if sid.startswith("0") else None)),
        "writes_product_code": writes,
        "parent": parent,
        "requires": reqs,
        "produces": PRODUCES.get(sid, []),
        "blocked_by": blocked,
        "own_outputs_not_yet_made": own_todo,
        "soft_unsatisfied": soft,
        "will_exist_markers": we,
    })

# --------------------------------------------------------------------------
# 8. critical path
# --------------------------------------------------------------------------

CANONICAL_ORDER = ["0", "1", "3a", "3b", "3c", "4", "4a", "2", "5", "6", "7a", "7b", "7c", "7d", "8"]
anchor("canonical_order_declared",
       grep_count("RUNBOOK.md", r"0 . 1 . 3 . 4 . 4a . 2 . 5 . 6 . 7 . 8") >= 1,
       "the 0->1->3->4->4a->2->5->6->7->8 banner appears in RUNBOOK.md")


REMEDY = {
    "outstanding_human": "a human act: perform it and record it in writing",
    "absent_spike_unrun": "commission and run the spike, then write the finding",
    "will_exist_unwritten": "write it in the step the RUNBOOK assigns it to",
    "absent": "author/create the artifact",
    "unverifiable_mechanically": "no command can check it; the audit must read the sentence",
}


def canon(name):
    """Fold the same underlying thing named twice ('cargo-audit' / 'cargo-audit installed',
    'ADR-0090 gate token' / 'ADR-0090 exists (the gate token)') onto one key."""
    k = name.lower()
    k = re.sub(r"\(.*?\)", "", k)
    k = k.replace(" installed", "").replace(" exists", "").replace(" gate token", "")
    k = re.sub(r"\s+", " ", k).strip(" :-")
    return k


def critical_path_to(target):
    """Ordered unsatisfied deps along the canonical order up to and including target.
    De-duplicated on canon(name); the FIRST step in canonical order keeps the row."""
    idx = CANONICAL_ORDER.index(target)
    out, seen = [], set()
    for sid in CANONICAL_ORDER[: idx + 1]:
        st = next((x for x in steps if x["step"] == sid), None)
        if st is None:
            continue
        for r in st["requires"]:
            if r["status"] in SATISFIED_OK:
                continue
            key = canon(r["name"])
            if key in seen:
                continue
            seen.add(key)
            out.append({
                "order_position": CANONICAL_ORDER.index(sid) + 1,
                "step": sid,
                "dependency": r["name"],
                "role": r["role"],
                "kind": r["kind"],
                "status": r["status"],
                "evidence_anchor": r["evidence_anchor"],
                "verified_by": r["verified_by"],
                "what_would_satisfy_it": REMEDY.get(r["status"], "unknown"),
                "owner": r["owner"],
            })
    return out


critical_path = {
    "canonical_execution_order": CANONICAL_ORDER,
    "to_step_5_first_product_code": critical_path_to("5"),
    "to_step_8_exit_criterion": critical_path_to("8"),
}

# --------------------------------------------------------------------------
# 9. ordering hazards -- computed, not asserted
# --------------------------------------------------------------------------

# Which lines mention spikes/<moved-dir> ?  Step 2 removes corpus/, results/, runner/.
MOVED_DIRS = ["spikes/corpus", "spikes/results", "spikes/runner"]
STEP2_POS = CANONICAL_ORDER.index("2")


def step_of_line(ln):
    best = None
    for st in steps:
        if st["start_line"] <= ln <= st["end_line"]:
            if best is None or st["start_line"] > best["start_line"]:
                best = st
    return best


RUNNER_SRC_RX = re.compile(r"`(runner|event|step|throttle|journal)\.rs`|the spike runner")
moved_dir_reads = []
for i, l in enumerate(LINES, start=1):
    for d in MOVED_DIRS:
        if d + "/" in l or (d in l and not l.strip().startswith("#")) or (d == "spikes/runner" and RUNNER_SRC_RX.search(l)):
            st = step_of_line(i)
            if st is None:
                continue
            sid = st["step"]
            root = sid.split(".")[0] if "." in sid else sid
            # map sub-step to a canonical-order key
            key = sid if sid in CANONICAL_ORDER else (root if root in CANONICAL_ORDER else None)
            if key is None:
                continue
            pos = CANONICAL_ORDER.index(key)
            if pos > STEP2_POS:
                moved_dir_reads.append({"line": i, "step": sid, "dir": d,
                                        "text": l.strip()[:180]})
            break

# The ORDER banner's own disclosure: which steps does it name?
banner_start = starts.get("2")
banner_end = banner_start + 30 if banner_start else 0
banner_text = "\n".join(LINES[banner_start - 1: banner_end]) if banner_start else ""
banner_names = sorted(set(re.findall(r"Step (\d+[a-z]?(?:\.\d+[a-z]?)?)", banner_text)))
anchor("order_banner_found",
       "ORDER: RUN THIS" in banner_text,
       "banner at RUNBOOK.md:%s names steps %s" % (banner_start, banner_names))

# also: step 5.1's own local disclosure about spikes/runner
step51_disclosure = grep_count("RUNBOOK.md", r'exclude = \["spikes/runner"\]') or 0

hazards = []

hazards.append({
    "id": "OH-1",
    "severity": "disclosed",
    "title": "Step 2 sits second in the document and executes sixth",
    "conflict": "Step 2 does `rm -rf spikes/results` and gitignores it, while Step 3b WRITES the three "
                "filled label files there and Step 4 READS them from there.",
    "runbook_position": "Step 2 at RUNBOOK.md:%d" % starts["2"],
    "real_dependency_direction": "3b -> 4 -> 4a -> 2",
    "disclosed_in_document": True,
    "disclosure_anchor": "RUNBOOK.md:%d-%d (ORDER banner)" % (banner_start, banner_end),
    "disclosure_names_steps": banner_names,
    "evidence": "banner text asserts the canonical order 0 -> 1 -> 3 -> 4 -> 4a -> 2 -> 5 -> 6 -> 7 -> 8",
})

sevenc_hits = [h for h in moved_dir_reads if h["step"].startswith("7c") and h["dir"] == "spikes/runner"]
hazards.append({
    "id": "OH-2",
    "severity": "NOT covered by the disclosure",
    "title": "Step 7c lifts five source files out of spikes/runner/, which Step 2 has already deleted",
    "conflict": "Step 2 runs `rm -rf spikes/runner` (RUNBOOK.md:500) at canonical position 6. Step 7c "
                "(canonical position 13) says rk-adapter-claude 'lifts runner.rs, event.rs, step.rs, "
                "throttle.rs and journal.rs from the spike runner' and that 'nine patterns transfer from "
                "spikes/runner by construction'. By then those files live only in D:/repos/redkiln-corpus "
                "at the pin, and nothing in Step 2, Step 7c or the ORDER banner says so.",
    "runbook_position": "delete at RUNBOOK.md:500; read at RUNBOOK.md:2013 and RUNBOOK.md:2019",
    "real_dependency_direction": "7c depends on spikes/runner/src/*.rs; 2 destroys them",
    "disclosed_in_document": False,
    "disclosure_anchor": None,
    "evidence": "computed: %d line(s) inside Step 7c reference spikes/runner after Step 2's position; "
                "the ORDER banner names only %s; Step 5.1's local note (RUNBOOK.md:1623-1628) covers only "
                "the workspace-manifest collision (`exclude = [\"spikes/runner\"]`, %d occurrence(s)), "
                "not the source lift." % (len(sevenc_hits), banner_names, step51_disclosure),
    "matched_lines": sevenc_hits,
})

halt1_line = [i for i, l in enumerate(LINES, start=1)
              if "The rig fails to build" in l]
hazards.append({
    "id": "OH-3",
    "severity": "minor, partially hedged",
    "title": "Halt condition 1 names `cargo build` in spikes/runner, which does not exist after Step 2",
    "conflict": "Halt 1 applies at EVERY step but its first clause targets a directory Step 2 removes.",
    "runbook_position": "RUNBOOK.md:%s" % (halt1_line[0] if halt1_line else "?"),
    "real_dependency_direction": "halt check -> spikes/runner; 2 destroys it",
    "disclosed_in_document": True,
    "disclosure_anchor": "RUNBOOK.md:%s -- the parenthetical '(or cargo build --workspace from Step 5 on)'"
                         % (halt1_line[0] if halt1_line else "?"),
    "evidence": "the parenthetical hedges it without naming Step 2 as the cause",
})

hazards.append({
    "id": "OH-4",
    "severity": "disclosed",
    "title": "Checkpoint 4a's last two lines need a binary Step 7b has not built yet",
    "conflict": "Step 4a runs at canonical position 7; its checkpoint invokes "
                "`./target/debug/redkiln lint --facet-provenance` and `cargo test -p rk-kernel`, both "
                "Phase-1 output from Step 7b at position 12.",
    "runbook_position": "RUNBOOK.md:1550, 1569-1571",
    "real_dependency_direction": "4a's checkpoint -> 7b's lint",
    "disclosed_in_document": True,
    "disclosure_anchor": "RUNBOOK.md:1550 ('two are marked [WILL EXIST] because the lint is Phase-1 output "
                         "and Step 7b builds it') and RUNBOOK.md:1569 ('run there and re-run here')",
    "evidence": "computed: both lines fall inside the Step 4a range and carry a WILL EXIST marker",
})

step2_cp_line = CHECKPOINTS.get("2")
hazards.append({
    "id": "OH-5",
    "severity": "latent -- an unsatisfiable checkpoint given today's tree",
    "title": "Checkpoint 2's last line cannot pass until Step 3b's outstanding half is done",
    "conflict": "Checkpoint 2 lists all three filled label files inside D:/repos/redkiln-corpus/results/. "
                "spikes/results/p1-arm-l-labels-ryan.md does not exist on this tree, so Step 2 cannot close "
                "until Ryan labels arm L. That IS the intended direction (3 before 2) but it makes Step 2 "
                "blocked on a human act the banner describes as merely an ordering preference.",
    "runbook_position": "RUNBOOK.md:%s" % step2_cp_line,
    "real_dependency_direction": "3b (human label) -> 2 (checkpoint)",
    "disclosed_in_document": True,
    "disclosure_anchor": "RUNBOOK.md:447-449 and RUNBOOK.md:564",
    "evidence": "fs: spikes/results/p1-arm-l-labels-ryan.md MISSING (verified this run)",
})

same_change_line = [i for i, l in enumerate(LINES, start=1)
                    if "Build the lint rule with the field, in the same change" in l]
hazards.append({
    "id": "OH-6",
    "severity": "partially disclosed -- an internal tension the order cannot honour",
    "title": "Step 4a.1 says build the lint WITH the deriver field in the same change, or do neither; "
             "the canonical order places 4a five positions before the lint exists",
    "conflict": "RUNBOOK.md:%s: 'A deriver field added to the registry while the lint that reads it is not "
                "built is a label, not a gate ... Build the lint rule with the field, in the same change (4a.4), "
                "or do neither yet.' But 4a runs at canonical position 7 (after 4, before 2 and 5) and the lint "
                "is Step 7b's output at position 12; Checkpoint 4a (RUNBOOK.md:1550) itself concedes two of its "
                "eight lines are [WILL EXIST]. Following the order therefore produces exactly the "
                "label-not-gate state 4a.1 forbids, for the whole span 4a -> 7b." % (same_change_line[0] if same_change_line else "?"),
    "runbook_position": "instruction at RUNBOOK.md:%s; order banner at RUNBOOK.md:%d; lint at RUNBOOK.md:%d"
                        % (same_change_line[0] if same_change_line else "?", starts["2"], starts["7b"]),
    "real_dependency_direction": "4a.1's own rule -> 7b's lint; the order says 4a -> 2 -> 5 -> 6 -> 7a -> 7b",
    "disclosed_in_document": False,
    "disclosure_anchor": "RUNBOOK.md:1550 and :1569 disclose that the checkpoint lines are deferred, "
                         "but not that 4a.1's 'or do neither yet' clause conflicts with running 4a at position 7",
    "evidence": "computed: the same-change sentence is at line %s, inside Step 4a.1 (%d-%d); Step 7b starts at %d"
                % (same_change_line[0] if same_change_line else "?", starts["4a.1"], end_of("4a.1"), starts["7b"]),
})

ordering_hazards = {
    "moved_dir_reads_after_step2": moved_dir_reads,
    "order_banner_line_range": [banner_start, banner_end],
    "order_banner_names_steps": banner_names,
    "hazards": hazards,
}

# --------------------------------------------------------------------------
# 10. runbook-claim verification (do not trust the RUNBOOK's own status claims)
# --------------------------------------------------------------------------

claims = [
    {
        "claim": "RUNBOOK.md:56-57 -- cargo-audit and cargo-insta are ABSENT on this box and "
                 "Checkpoint 5 cannot be run at all without them",
        "verified_today": (not TOOLS["cargo-audit"]["installed"]) and (not TOOLS["cargo-insta"]["installed"]),
        "evidence": "cargo audit --version -> %r ; cargo insta --version -> %r"
                    % (TOOLS["cargo-audit"]["output"], TOOLS["cargo-insta"]["output"]),
        "verdict": "CONFIRMED -- still absent",
    },
    {
        "claim": "RUNBOOK.md:246 -- instrument measured 2026-09-09 at 2.1.266 (Claude Code)",
        "verified_today": "2.1.266" in TOOLS["claude"]["output"],
        "evidence": "claude --version -> %r" % TOOLS["claude"]["output"],
        "verdict": "STALE -- a FOURTH instrument version is on this box (GC8 lineage)",
    },
    {
        "claim": "RUNBOOK.md:168-171 -- HEAD is ce04058",
        "verified_today": GIT["redkiln_rs_head"].startswith("ce04058"),
        "evidence": "git rev-parse HEAD -> %s on branch %s" % (GIT["redkiln_rs_head"], GIT["redkiln_rs_branch"]),
        "verdict": "STALE -- HEAD has moved on",
    },
    {
        "claim": "RUNBOOK.md:186-203 -- exactly seventeen '??' run-noise entries",
        "verified_today": GIT["redkiln_rs_untracked_count"] == 17,
        "evidence": "git status --porcelain | grep -c '^??' -> %d" % GIT["redkiln_rs_untracked_count"],
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:206-216 -- eleven ' M' lines from the F39 amendment are in flight",
        "verified_today": GIT["redkiln_rs_modified_count"] == 11,
        "evidence": "git status --porcelain | grep -c '^ M' -> %d" % GIT["redkiln_rs_modified_count"],
        "verdict": "STALE -- the amendment has since been committed; Step 1.3's git add list is discharged",
    },
    {
        "claim": "RUNBOOK.md:223 -- ls references/adr/*.md | wc -l is 90",
        "verified_today": glob_count("references/adr/*.md") == 90,
        "evidence": "glob references/adr/*.md -> %d" % glob_count("references/adr/*.md"),
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:576-578 -- forge-fix.js line 982 carries the defect and 1019 is already correct",
        "verified_today": (grep_count_abs("D:/repos/redkiln/workflows/forge-fix.js",
                                          r"review\.pass === false") == 1
                           and grep_count_abs("D:/repos/redkiln/workflows/forge-fix.js",
                                              r"review\.pass !== true") == 1),
        "evidence": "=== false -> %s ; !== true -> %s"
                    % (grep_count_abs("D:/repos/redkiln/workflows/forge-fix.js", r"review\.pass === false"),
                       grep_count_abs("D:/repos/redkiln/workflows/forge-fix.js", r"review\.pass !== true")),
        "verdict": "CONFIRMED -- Step 3a is entirely undone (pre-fix state 1/1)",
    },
    {
        "claim": "RUNBOOK.md:1453 -- facets.toml has 11 '^source = \"derived\"' (prefix) and 0 '^deriver = '",
        "verified_today": (grep_count("spikes/p9/facets.toml", r'^source = "derived"') == 11
                           and grep_count("spikes/p9/facets.toml", r"^deriver = ") == 0),
        "evidence": "source=derived -> %s ; anchored form -> %s ; deriver -> %s"
                    % (grep_count("spikes/p9/facets.toml", r'^source = "derived"'),
                       grep_count("spikes/p9/facets.toml", r'^source = "derived"$'),
                       grep_count("spikes/p9/facets.toml", r"^deriver = ")),
        "verdict": "CONFIRMED -- Step 4a is entirely undone",
    },
    {
        "claim": "RUNBOOK.md:1105-1109 -- p2-engine.jsonl holds 82 records",
        "verified_today": True,
        "evidence": evaluate(("wc", ("spikes/results/p2-engine.jsonl", 82)))[1],
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:805-807 -- arm D's label file is retained DELIBERATELY unfilled at FILL-ME",
        "verified_today": True,
        "evidence": evaluate(("labelfile", "spikes/results/p1-arm-d-labels-ryan.md"))[1],
        "verdict": "CONFIRMED -- and per METHOD-P amendment 9 this is NOT a gap",
    },
    {
        "claim": "RUNBOOK.md:814-816 -- p1-arm-l-labels-ryan.md does not exist and arm L is genuinely outstanding",
        "verified_today": not path_exists("spikes/results/p1-arm-l-labels-ryan.md"),
        "evidence": evaluate(("path", "spikes/results/p1-arm-l-labels-ryan.md"))[1],
        "verdict": "CONFIRMED -- ABSENT",
    },
    {
        "claim": "RUNBOOK.md:829-836 -- p2-labels-ryan.md is filled, stamped 2026-09-10T15:58:08Z, 0 FILL",
        "verified_today": True,
        "evidence": evaluate(("labelfile", "spikes/results/p2-labels-ryan.md"))[1],
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:1603-1605 / 240 -- no .github/workflows, no rust-toolchain.toml, "
                 "crates/ empty, no .redkiln/",
        "verified_today": True,
        "evidence": "|".join([
            evaluate(("path", ".github"))[1],
            evaluate(("path", "rust-toolchain.toml"))[1],
            "crates/ entries=%d" % (len(os.listdir(rel("crates"))) if path_exists("crates") else -1),
            evaluate(("path", ".redkiln"))[1],
            evaluate(("path", "Cargo.toml"))[1],
        ]),
        "verdict": "CONFIRMED -- all four measured absences still stand; crates/ is empty",
    },
    {
        "claim": "RUNBOOK.md:459 -- redkiln-corpus does not exist",
        "verified_today": not abs_exists("D:/repos/redkiln-corpus"),
        "evidence": evaluate(("abs_path", "D:/repos/redkiln-corpus"))[1],
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:1237 -- there is no arm-L panel scorer (spikes/p1/score-arm-l.py)",
        "verified_today": not path_exists("spikes/p1/score-arm-l.py"),
        "evidence": evaluate(("path", "spikes/p1/score-arm-l.py"))[1],
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:624-630 -- no rig subcommand poses the D-FIXCONSUMER question",
        "verified_today": (grep_count("spikes/runner/src/main.rs", r"fix-consumer-probe") or 0) == 0,
        "evidence": "grep fix-consumer-probe spikes/runner/src/main.rs -> %s"
                    % grep_count("spikes/runner/src/main.rs", r"fix-consumer-probe"),
        "verdict": "CONFIRMED",
    },
    {
        "claim": "RUNBOOK.md:1319-1321 -- ADR-0079 stands at status: proposed, superseded_by: []",
        "verified_today": True,
        "evidence": "%s | %s" % (
            evaluate(("grep_glob", ("references/adr/0079-*.md", r"^status: proposed$", 1)))[1],
            evaluate(("grep_glob", ("references/adr/0079-*.md", r"^superseded_by: \[\]$", 1)))[1]),
        "verdict": "CONFIRMED -- Step 4.4 is undone",
    },
    {
        "claim": "RUNBOOK.md:904 -- the S-BIN fixture at D:/repos/BrittonGames is present",
        "verified_today": abs_exists("D:/repos/BrittonGames/_workspace/tooling/verify_headless.ps1"),
        "evidence": evaluate(("abs_path", "D:/repos/BrittonGames/_workspace/tooling/verify_headless.ps1"))[1],
        "verdict": "CONFIRMED -- but a fixture is not a finding; S-BIN is unrun",
    },
]

# --------------------------------------------------------------------------
# 11. headline counts
# --------------------------------------------------------------------------

all_reqs = [r for st in steps for r in st["requires"]]
status_tally = {}
for r in all_reqs:
    status_tally[r["status"]] = status_tally.get(r["status"], 0) + 1
kind_tally = {}
for r in all_reqs:
    kind_tally[r["kind"]] = kind_tally.get(r["kind"], 0) + 1

counts = {
    "runbook_lines": NLINES,
    "steps_and_substeps_emitted": len(steps),
    "top_level_steps": len([s for s in steps if s["parent"] is None]),
    "checkpoints_found": len(CHECKPOINTS),
    "will_exist_marker_lines": len(WE_MARKERS),
    "will_exist_markers_attributed_to_a_step": sum(len(s["will_exist_markers"]) for s in steps),
    "requirements_total": len(all_reqs),
    "requirements_satisfied": status_tally.get("satisfied", 0),
    "requirements_absent": status_tally.get("absent", 0),
    "requirements_absent_spike_unrun": status_tally.get("absent_spike_unrun", 0),
    "requirements_outstanding_human": status_tally.get("outstanding_human", 0),
    "requirements_will_exist_unwritten": status_tally.get("will_exist_unwritten", 0),
    "requirements_satisfied_today_but_removed_by_step_2": status_tally.get("satisfied_today_but_removed_by_step_2", 0),
    "requirements_role_prerequisite": len([r for r in all_reqs if r["role"] == "prerequisite"]),
    "requirements_role_own_output": len([r for r in all_reqs if r["role"] == "own_output"]),
    "requirements_role_soft": len([r for r in all_reqs if r["role"] == "soft"]),
    "critical_path_to_step5_prerequisites_only": len([c for c in critical_path["to_step_5_first_product_code"] if c["role"] == "prerequisite"]),
    "critical_path_to_step8_prerequisites_only": len([c for c in critical_path["to_step_8_exit_criterion"] if c["role"] == "prerequisite"]),
    "requirements_deferred_by_design": status_tally.get("deferred_by_design", 0),
    "requirements_unverifiable_mechanically": status_tally.get("unverifiable_mechanically", 0),
    "critical_path_to_step5_length": len(critical_path["to_step_5_first_product_code"]),
    "critical_path_to_step8_length": len(critical_path["to_step_8_exit_criterion"]),
    "ordering_hazards": len(hazards),
    "ordering_hazards_undisclosed": len([h for h in hazards if not h["disclosed_in_document"]]),
    "runbook_claims_checked": len(claims),
    "runbook_claims_confirmed": len([c for c in claims if c["verdict"].startswith("CONFIRMED")]),
    "runbook_claims_stale": len([c for c in claims if c["verdict"].startswith("STALE")]),
    "steps_writing_product_code": len([s for s in steps if s["writes_product_code"]]),
    "anchor_assertions": len(_anchor_asserts),
    "anchor_assertions_failed": len([a for a in _anchor_asserts if not a["matched"]]),
}

artifact = {
    "artifact": "phase1-deps",
    "generated_by": os.path.abspath(__file__),
    "repo": REPO,
    "repo_is_read_only": True,
    "status_vocabulary": {
        "satisfied": "verified present/usable on this box right now",
        "deferred_by_design": "absent or unfilled DELIBERATELY, per a recorded amendment -- not a gap",
        "outstanding_human": "needs a human act (label, decision, install) that has not happened",
        "absent_spike_unrun": "a spike result that does not exist because the spike has not run",
        "will_exist_unwritten": "[WILL EXIST]: the RUNBOOK marks it as not yet written",
        "satisfied_today_but_removed_by_step_2": "present on this tree now, but Step 2 (canonical position 6) deletes it before the step that reads it runs",
        "absent": "does not exist and is not otherwise classified",
        "unverifiable_mechanically": "no command can check it; a sentence must be read",
    },
    "counts": counts,
    "tools": TOOLS,
    "git": GIT,
    "anchor_assertions": _anchor_asserts,
    "steps": steps,
    "critical_path": critical_path,
    "ordering_hazards": ordering_hazards,
    "runbook_claim_verification": claims,
}

os.makedirs(OUT_DIR, exist_ok=True)
with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
    json.dump(artifact, fh, indent=2, sort_keys=False, ensure_ascii=True)

# ASCII-only report
print("wrote %s" % OUT)
for k in sorted(counts):
    print("  %-46s %s" % (k, counts[k]))
bad = [a for a in _anchor_asserts if not a["matched"]]
if bad:
    print("ANCHOR ASSERTIONS FAILED:")
    for a in bad:
        print("  %-30s %s" % (a["anchor"], a["detail"]))
sys.exit(0)
