#!/usr/bin/env python3
"""P1 arm D scoring — precision/recall per facet per source against the human labels (METHOD-P §1.3/§1.4).
Refuses to run unless every label file's `labelled_at` timestamp precedes the engine output's earliest `at`
(METHOD-P §0 rule 1). Set-valued facets are scored per (item, value) membership.
Usage: python3 spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl --labels spikes/results/p1-arm-d-labels-ryan.md [--labels ...]
"""
import argparse, json, re, collections
def parse_labels(path):
    t = open(path).read(); m = re.search(r"labelled_at:\s*(\S+)", t)
    if not m or "FILL-ME" in m.group(1): raise SystemExit(f"{path}: labelled_at missing — labels must be timestamped before unblinding")
    labels = {}
    for row in re.finditer(r"^\|\s*([^|#]+?)\s*\|\s*(toolchain|change|binary_change|auth|data_migration|ui|flag_exists)\s*\|\s*([^|]*?)\s*\|", t, re.M):
        item, facet, val = row.groups(); val = val.strip()
        if val in ("", "?"): continue
        labels[(item, facet)] = set(v.strip() for v in val.split(",")) if facet in ("toolchain", "change") else (val.lower() in ("1", "true", "yes"))
    return m.group(1), labels
def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--engine", required=True); ap.add_argument("--labels", action="append", required=True); a = ap.parse_args()
    eng = [json.loads(l) for l in open(a.engine)]; first = min(r["at"] for r in eng)
    for lp in a.labels:
        ts, labels = parse_labels(lp)
        if ts >= first: raise SystemExit(f"{lp}: labelled_at {ts} is not before engine first output {first} — METHOD-P §0 rule 1")
        print(f"== {lp} (labelled_at {ts} < engine {first})")
        for source in ("docs", "commits", "union"):
            tp = collections.Counter(); fp = collections.Counter(); fn = collections.Counter(); unknown = collections.Counter()
            for r in (x for x in eng if x["source"] == source):
                key = (r["item"], r["facet"]); lab = labels.get(key)
                if lab is None: unknown[r["facet"]] += 1; continue
                if isinstance(lab, set):
                    got = set(r["value"]); tp[r["facet"]] += len(got & lab); fp[r["facet"]] += len(got - lab); fn[r["facet"]] += len(lab - got)
                else:
                    got = bool(r["value"]); tp[r["facet"]] += got and lab; fp[r["facet"]] += got and not lab; fn[r["facet"]] += (not got) and lab
            for f in sorted(set(tp) | set(fp) | set(fn)):
                p = tp[f] / (tp[f] + fp[f]) if tp[f] + fp[f] else float("nan"); rc = tp[f] / (tp[f] + fn[f]) if tp[f] + fn[f] else float("nan")
                verdict = "LEAVES THE MODEL (>=0.95)" if p >= 0.95 else "stays LLM-emitted"
                print(f"  {source:8} {f:15} precision={p:.2f} recall={rc:.2f} tp={tp[f]} fp={fp[f]} fn={fn[f]} unlabelled={unknown[f]}  {verdict}")
if __name__ == "__main__": main()
