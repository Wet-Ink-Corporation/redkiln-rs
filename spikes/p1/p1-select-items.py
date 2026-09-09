#!/usr/bin/env python3
"""P1 — item selection (METHOD-P §1.2): 30 items from the frozen `.bklg` at the pinned SHA, stratified
8 bug / 6 story / 6 project / 4 initiative / 3 incident-shaped / 3 spike, by sha256(slug) order, written to
spikes/plans/p1-items.txt BEFORE any dispatch. Zero model cost; committed state only (`git show`).

CORRECTION 2026-09-09 (DECISIONS-2026-09-09 #9): the original docstring claimed "the old corpus has no
incident/spike cards". Measured at 47faec9f under .bklg/: 10 real `incident.md`, 2 `bug.md`, 3
`initiative.md`, 165 `story.md`, 29 `project.md`, and 0 spike of any kind. So "incident-shaped" is served
by the real incident cards, and the bug->incident reclassification is REMOVED -- INC_RE matched the bare
word "incident" anywhere in a body and would have drained a stratum that has only 2 members. Kind now comes
from the card frontmatter `type:` field (authoritative), with the filename map as fallback. The spike
heuristic (slug or title contains "spike") is kept; it matches nothing at this SHA.
Shortfalls are reported, never back-filled from another stratum (that would change the pre-registered mix
silently) -- METHOD-P 1.2's quotas are not negotiable, the script is.

Usage: python3 spikes/p1/p1-select-items.py --redkiln D:/repos/redkiln --sha 47faec9f46b8d983c2c75f0794164e68415ae706
NOT YET RUN: the corpus repo was not mounted in the session that wrote this (2026-09-07/08).
"""
import argparse, hashlib, re, subprocess, sys, os
CARDS = {"bug.md": "bug", "story.md": "story", "project.md": "project", "initiative.md": "initiative", "incident.md": "incident"}
QUOTA = [("bug", 8), ("story", 6), ("project", 6), ("initiative", 4), ("incident", 3), ("spike", 3)]
KINDS = {"bug", "story", "project", "initiative", "incident", "spike"}
TYPE_RE = re.compile(r"^type:\s*([A-Za-z-]+)\s*$", re.M)  # #9: authoritative kind field
# #9 retired (kept for provenance): INC_RE = re.compile(r"\b(outage|incident|sev...)", re.I)

def git(repo, *a):
    r = subprocess.run(["git", *a], cwd=repo, capture_output=True)
    if r.returncode: raise SystemExit(r.stderr.decode(errors="replace"))
    return r.stdout.decode("utf-8", "replace")

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True)
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "plans", "p1-items.txt")); a = ap.parse_args()
    files = [f for f in git(a.redkiln, "ls-tree", "-r", "--name-only", a.sha).splitlines() if f.startswith(".bklg/")]
    items = {}
    for f in files:
        parts = f.split("/"); name = parts[-1]
        if name in CARDS and len(parts) >= 3:
            slug = "/".join(parts[1:-1]); body = git(a.redkiln, "show", f"{a.sha}:{f}")
            # #9: frontmatter `type:` is authoritative; the filename map is the fallback.
            _m = TYPE_RE.search(body)
            kind = _m.group(1).lower() if _m and _m.group(1).lower() in KINDS else CARDS[name]
            # #9: bug->incident reclassification removed (see module docstring).
            if "spike" in slug.lower() or re.search(r"^title:.*spike", body, re.I | re.M): kind = "spike"
            items[slug] = (kind, f)
    by = {}
    for slug, (kind, f) in items.items(): by.setdefault(kind, []).append((hashlib.sha256(slug.encode()).hexdigest(), slug, f))
    for k in by: by[k].sort()
    lines = [f"# P1 items — corpus {a.sha} — selection by sha256(slug) order, quotas per METHOD-P §1.2. Generated {__import__('datetime').datetime.now(__import__('datetime').timezone.utc).replace(tzinfo=None).isoformat()}Z",
             "# kind\tslug\tcard\tsha256(slug)[:12]"]
    short = []
    for kind, n in QUOTA:
        pool = by.get(kind, [])
        if len(pool) < n: short.append(f"{kind}: {len(pool)}/{n}")
        for h, slug, f in pool[:n]: lines.append(f"{kind}\t{slug}\t{f}\t{h[:12]}")
    if short: lines.append("# SHORTFALL (report in F-draft, do not back-fill): " + "; ".join(short))
    os.makedirs(os.path.dirname(a.out), exist_ok=True); open(a.out, "w", encoding="utf-8", newline="\n").write("\n".join(lines) + "\n")
    print("\n".join(lines))

if __name__ == "__main__": main()
