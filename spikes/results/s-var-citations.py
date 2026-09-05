"""S-VAR SECONDARY: the on-target citation check of METHOD-S 1.4.

METHOD-S 1.4 records "whether each rejection cites the F24 finding's file
(mechanical, as in E3 strong detection)". The rig cannot deliver it directly:
`review_probe` passes `reverted = []` for `--condition clean`, so
`e3::score_files` has no targets and `scored.matched_files` is empty on every
CLEAN record. `audit-s.py`'s on-target column consequently reads `0/k` for all
60 S-VAR records and measures nothing.

`scored.cited_files` IS recorded, so the sub-metric is recoverable against a
target list fixed before the data. `results/s-var-citation-targets.json` is that
list, generated from `results/e3.jsonl` while `results/s-var.jsonl` did not yet
exist.

`audit-s.py` is deliberately NOT modified. This is a separate instrument and its
output is reported as a SECONDARY, never as `audit-s.py`'s.

Usage:
    python results/s-var-citations.py [results/s-var.jsonl] [results/s-var-citation-targets.json]
"""

import json
import sys
from collections import defaultdict


def norm(s):
    """e3::score_files normalisation, reproduced exactly."""
    return s.replace("\\", "/").removeprefix("./").lower()


def on_target(cited, targets):
    """True if any cited path suffix-matches any target path.

    Suffix matching in both directions, as `score_files` does: a reviewer may
    cite `src/cli/advance.ts`, `./src/cli/advance.ts` or an absolute path, and
    being strict about the form would fail a real citation.
    """
    c = [norm(x) for x in cited]
    t = [norm(x) for x in targets]
    return any(a.endswith(b) or b.endswith(a) for a in c for b in t)


def load(path):
    out = []
    for line in open(path, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        r = json.loads(line)
        if r.get("experiment") == "E3" and "slug" in r:
            out.append(r)
    return out


def valid(r):
    """The same void rules audit-s.py applies, so the two tables agree on n."""
    if not r.get("card_adopted"):
        return False
    if not r.get("node_modules_linked"):
        return False
    if r.get("condition") == "s-silent" and not r.get("fix_unreachable"):
        return False
    return r.get("scored") is not None


def main(argv):
    recs_path = argv[1] if len(argv) > 1 else "results/s-var.jsonl"
    targets_path = argv[2] if len(argv) > 2 else "results/s-var-citation-targets.json"
    spec = json.load(open(targets_path, encoding="utf-8"))
    items = spec["items"]

    per_item = defaultdict(list)
    for r in load(recs_path):
        if valid(r):
            per_item[r["slug"]].append(r)

    print("S-VAR SECONDARY - on-target citations (METHOD-S 1.4 'Recorded')")
    print(f"  targets pre-registered at {spec['generated_at_head']} from {spec['source']}")
    print("  NOT audit-s.py output; audit-s.py's own on-target column reads 0/k by construction.\n")
    print(f"  {'item':<46} {'class':<18} n  reject  on-target  off-target")

    for slug, meta in items.items():
        rs = per_item.get(slug, [])
        n = len(rs)
        rejects = [r for r in rs if r["scored"]["rejected"]]
        hits = [r for r in rejects if on_target(r["scored"].get("cited_files") or [], meta["targets"])]
        k = len(rejects)
        print(f"  {slug:<46} {meta['evidence_class']:<18} {n:<2} {k:<7} {len(hits):<10} {k - len(hits)}")

    print("\noff-target citations, in full (a rejection for a reason F24 did not adjudicate):")
    any_off = False
    for slug, meta in items.items():
        for r in per_item.get(slug, []):
            s = r["scored"]
            if not s["rejected"]:
                continue
            if on_target(s.get("cited_files") or [], meta["targets"]):
                continue
            any_off = True
            print(f"  {slug} r{r.get('repeat')}: {s.get('cited_files')}")
    if not any_off:
        print("  (none)")


if __name__ == "__main__":
    main(sys.argv)
