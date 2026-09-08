#!/usr/bin/env python3
"""P2 — item selection (METHOD-P §2.2): 20 completed items from the frozen `.bklg` with >= 3 stage artifacts each,
by sha256(slug) order; written to spikes/plans/p2-items.txt before any freshness verdict is computed.
"Completed" = the card's frontmatter status/stage is one of {done, closed, closeout, retrospective, released}
(the S-MIG census's stage vocabulary; adjust the set if the census shows another spelling and DISCLOSE it).
Usage: python3 spikes/p2/p2-select-items.py --redkiln D:/repos/redkiln --sha 47faec9f...
NOT YET RUN: corpus repo not mounted when written.
"""
import argparse, hashlib, re, subprocess, os, datetime
ARTIFACTS = ["spec.md", "_plan.md", "plan.md", "implementation-report.md", "report.md", "_review.md", "_integration.md", "_retrospective.md", "closure.md"]
DONE = {"done", "closed", "closeout", "retrospective", "released", "complete", "completed"}
def git(repo, *a):
    r = subprocess.run(["git", *a], cwd=repo, capture_output=True)
    if r.returncode: raise SystemExit(r.stderr.decode(errors="replace"))
    return r.stdout.decode("utf-8", "replace")
def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True)
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "plans", "p2-items.txt")); a = ap.parse_args()
    files = [f for f in git(a.redkiln, "ls-tree", "-r", "--name-only", a.sha).splitlines() if f.startswith(".bklg/")]
    folders = {}
    for f in files: folders.setdefault(f.rsplit("/", 1)[0], []).append(f.rsplit("/", 1)[1])
    rows = []
    for folder, names in folders.items():
        card = next((n for n in names if n in ("story.md", "bug.md", "project.md", "initiative.md", "incident.md")), None)
        if not card: continue
        arts = [n for n in names if n in ARTIFACTS]
        if len(arts) < 3: continue
        fm = git(a.redkiln, "show", f"{a.sha}:{folder}/{card}")[:2000]
        m = re.search(r"^(status|stage):\s*([\w-]+)", fm, re.M)
        if not m or m.group(2).lower() not in DONE: continue
        slug = folder[len(".bklg/"):]; rows.append((hashlib.sha256(slug.encode()).hexdigest(), slug, card, arts))
    rows.sort()
    lines = [f"# P2 items — corpus {a.sha} — completed items with >=3 artifacts, sha256(slug) order. Generated {datetime.datetime.utcnow().isoformat()}Z",
             "# slug\tcard\tartifacts\tsha256(slug)[:12]"] + [f"{s}\t{c}\t{','.join(ar)}\t{h[:12]}" for h, s, c, ar in rows[:20]]
    if len(rows) < 20: lines.append(f"# SHORTFALL: only {len(rows)} eligible items (report, do not relax the >=3 rule silently)")
    os.makedirs(os.path.dirname(a.out), exist_ok=True); open(a.out, "w").write("\n".join(lines) + "\n"); print("\n".join(lines))
if __name__ == "__main__": main()
