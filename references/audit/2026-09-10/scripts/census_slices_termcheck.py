#!/usr/bin/env python3
"""Diagnostic for census_slices.py: distribution of specific distinctive terms
across the top-level sections of the AC register, the design doc and the runbook.
Read-only. ASCII stdout."""
import re, os, sys

REPO = r"D:/repos/redkiln-rs"
FILES = {
    "AC": "references/2026-09-08-acceptance-criteria.md",
    "DES": "references/2026-09-08-architecture-design.md",
    "RB": "RUNBOOK.md",
}
TERMS = sys.argv[1:] or [
    "ChangeSummary", "common-dir", "gix", "interpret-trailers", "E3", "MCC",
    "StructuredOutput", "structured output", "effort", "model pin", "cache",
    "ephemeral", "TTL", "five_hour", "isolation", "clippy", "workspace",
    "toolchain", "fix lane", "story lane", "durable-execution", "quality",
    "shorthand", "append", "--system-prompt", "narrowed-pin", "Advisory",
    "stale_policy", "plugin", "rk-plugin-gen", "layout C", "reconcile",
    "tier-2", "trailer", "sufficient alone", "discriminating", "lattice",
    "rigor", "self-host", "redkiln-migrate", "batch-delta", "resolution_limited",
    "mid-graph", "resume", "false approval", "context vector", "derived",
]


def sections(ls, pat):
    hits = [(i, re.match(pat, ln).group(1)) for i, ln in enumerate(ls, 1) if re.match(pat, ln)]
    out = []
    for k, (i, num) in enumerate(hits):
        end = hits[k + 1][0] - 1 if k + 1 < len(hits) else len(ls)
        out.append((num, i, end))
    return out


data = {}
for tag, rel in FILES.items():
    ls = open(os.path.join(REPO, rel), encoding="utf-8", errors="replace").read().split("\n")
    if tag == "RB":
        secs = sections(ls, r"^## (\S+(?: \S+)?)")
    else:
        secs = sections(ls, r"^## (\d+(?:\.\d+)?)")
    data[tag] = (ls, secs)

for t in TERMS:
    row = []
    for tag in ("AC", "DES", "RB"):
        ls, secs = data[tag]
        cnt = []
        for num, a, b in secs:
            body = "\n".join(ls[a - 1:b]).lower()
            n = body.count(t.lower())
            if n:
                cnt.append((num.replace("Step", "S"), n))
        cnt.sort(key=lambda x: -x[1])
        row.append("%s:%s" % (tag, ",".join("%s=%d" % c for c in cnt[:4]) or "0"))
    print("%-22s %s" % (t[:22], "  ".join(row)))
