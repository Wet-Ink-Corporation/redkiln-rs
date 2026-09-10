#!/usr/bin/env python3
"""Score P2: precision and recall of `Stale` against the human "should re-run" labels.

METHOD-P §2.3 registers this and nothing in the repository computed it — `reconstruct-derived-from.py
--phase engine` emits verdicts and stops. This is that scorer.

Three reporting rules it will not bend, because each one is a defect this programme has already paid for:

  1. **Recall is reported as NOT COMPUTABLE when the label set carries no positives.** Zero "should
     re-run" labels means TP + FN = 0 and recall has no denominator. Printing 0.00, 1.00 or "n/a" for
     it would each be a claim; the honest output is that the quantity does not exist on this corpus.
     Same shape as `data_migration` in F39 and P7's criterion 3.
  2. **`excluded` (an `unknown` reconstructed fine scope) is an abstention, reported as its own rate,
     and is NEVER counted as a false negative.** A silent miss and an honest abstention are different
     outcomes and may not be summed (METHOD-P §0; the rule F39 §6 states for `unknown`).
  3. **Every rate carries a Wilson 95% interval**, and a rate over zero positives says so.

Usage:
  python spikes/p2/score-p2.py [--engine spikes/results/p2-engine.jsonl]
                               [--labels spikes/results/p2-labels-ryan.md]
"""
import argparse
import io
import json
import math
import re
import sys
from collections import Counter

Z = 1.959963984540054  # 95%


def wilson(k, n):
    """Wilson score interval for k successes in n trials. Returns (low, high) or None when n == 0."""
    if n == 0:
        return None
    p = k / n
    d = 1 + Z * Z / n
    centre = (p + Z * Z / (2 * n)) / d
    half = Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n)) / d
    return (max(0.0, centre - half), min(1.0, centre + half))


def fmt_ci(ci):
    return "—" if ci is None else f"[{ci[0]:.2f}, {ci[1]:.2f}]"


def parse_labels(path):
    """The worksheet's own row shape, which is also the RUNBOOK's completion grep."""
    t = io.open(path, encoding="utf-8").read()
    m = re.search(r"^labelled_at:\s*(\S+)", t, re.M)
    if not m or "FILL" in m.group(1):
        sys.exit("labels are not timestamped — refusing to score")
    who = re.search(r"^labeller:\s*(.+)$", t, re.M)
    labels = {}
    for row in re.finditer(
            r"^\|\s*(P\d\d)\s*\|[^|]*\|[^|]*\|\s*([^|]+?)\s*\|\s*(yes|no)\s*\|([^|]*)\|\s*$",
            t, re.M):
        labels[(row.group(1), row.group(2))] = row.group(3)
    return labels, m.group(1), (who.group(1).strip() if who else "")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--engine", default="spikes/results/p2-engine.jsonl")
    ap.add_argument("--labels", default="spikes/results/p2-labels-ryan.md")
    a = ap.parse_args()

    labels, labelled_at, labeller = parse_labels(a.labels)
    recs = [json.loads(l) for l in io.open(a.engine, encoding="utf-8") if l.strip()]
    if not recs:
        sys.exit("no engine records")

    shas = {r["corpus_sha"] for r in recs}
    stamps = {r.get("labels_at") for r in recs}
    if len(shas) != 1:
        sys.exit(f"engine records span several corpus pins: {shas}")
    if stamps != {labelled_at}:
        sys.exit(f"engine was run against different labels: {stamps} vs {labelled_at}")

    print("P2 — short-circuit precision/recall of `Stale` against \"should re-run\"")
    print(f"corpus pin      {list(shas)[0]}")
    print(f"labelled_at     {labelled_at}     labeller: {labeller or '(UNSET)'}")
    print(f"engine records  {len(recs)}   labels {len(labels)}\n")

    pos = sum(1 for v in labels.values() if v == "yes")
    neg = len(labels) - pos
    print(f"LABEL SET: {pos} should re-run, {neg} need not, over {len(labels)} rows.")
    if pos == 0:
        print("  *** NO POSITIVE LABELS. Recall has no denominator on this corpus and is reported")
        print("      as NOT COMPUTABLE below, not as 0.00 and not as 1.00. A method that marks")
        print("      nothing Stale and a method that marks everything Stale are distinguishable")
        print("      here only by precision.\n")

    by_scope = {}
    for scope in ("coarse", "fine"):
        rows = [r for r in recs if r["scope"] == scope]
        tp = fp = tn = fn = excl = unlabelled = 0
        for r in rows:
            key = (r["case"], r["downstream"])
            lab = labels.get(key)
            if lab is None:
                unlabelled += 1
                continue
            if r["verdict"] == "excluded":
                excl += 1
                continue
            stale = (r["verdict"] == "Stale")
            want = (lab == "yes")
            if stale and want:
                tp += 1
            elif stale and not want:
                fp += 1
            elif not stale and want:
                fn += 1
            else:
                tn += 1
        by_scope[scope] = dict(tp=tp, fp=fp, tn=tn, fn=fn, excl=excl,
                               unlabelled=unlabelled, n=len(rows))

    print("SPECIFICITY is the quantity this corpus can actually measure: with no positive labels,")
    print("every Stale the engine emits is a false alarm BY CONSTRUCTION, so `precision = 0.00` is a")
    print("property of the label set, not an independent reading of the engine. What is genuinely")
    print("measured is how often the engine leaves a correctly-unchanged artifact alone.\n")

    print(f"{'scope':<9}{'specificity':<14}{'95% CI':<16}{'precision':<11}{'recall':<18}"
          f"{'tp':<4}{'fp':<4}{'fn':<4}{'tn':<5}{'excl':<6}scored")
    print("-" * 110)
    for scope, s in by_scope.items():
        emitted = s["tp"] + s["fp"]
        prec = f"{s['tp']/emitted:.2f}" if emitted else "—"
        rec = "NOT COMPUTABLE" if (s["tp"] + s["fn"]) == 0 else f"{s['tp']/(s['tp']+s['fn']):.2f}"
        actual_neg = s["tn"] + s["fp"]
        spec = f"{s['tn']/actual_neg:.2f}" if actual_neg else "—"
        sci = fmt_ci(wilson(s["tn"], actual_neg)) if actual_neg else "—"
        scored = s["tp"] + s["fp"] + s["tn"] + s["fn"]
        print(f"{scope:<9}{spec:<14}{sci:<16}{prec:<11}{rec:<18}"
              f"{s['tp']:<4}{s['fp']:<4}{s['fn']:<4}{s['tn']:<5}{s['excl']:<6}{scored} of {s['n']}")
    print()

    for scope, s in by_scope.items():
        if s["excl"]:
            r = s["excl"] / s["n"]
            print(f"abstention ({scope}): {s['excl']}/{s['n']} = {r:.1%} "
                  f"{fmt_ci(wilson(s['excl'], s['n']))} — reconstructed scope `unknown`, "
                  f"excluded, never counted as a miss")
    print()

    # Like-for-like: the rows both scopes actually scored.
    fine_ok = {(r["case"], r["downstream"]) for r in recs
               if r["scope"] == "fine" and r["verdict"] != "excluded"}
    both = {}
    for r in recs:
        k = (r["case"], r["downstream"])
        if k in fine_ok:
            both.setdefault(k, {})[r["scope"]] = r["verdict"]
    agree = sum(1 for v in both.values() if v.get("coarse") == v.get("fine"))
    cs = sum(1 for v in both.values() if v.get("coarse") == "Stale")
    fs = sum(1 for v in both.values() if v.get("fine") == "Stale")
    print(f"LIKE-FOR-LIKE on the {len(both)} rows both scopes scored: "
          f"coarse Stale {cs}, fine Stale {fs}, verdicts agree on {agree}/{len(both)}.")
    flips = [(k, v) for k, v in sorted(both.items()) if v.get("coarse") != v.get("fine")]
    for k, v in flips:
        print(f"    {k[0]} {k[1]:<26} coarse={v.get('coarse'):<8} fine={v.get('fine')}")
    print()

    print("METHOD-P §2.4 kill criteria")
    print("-" * 96)
    c = by_scope["coarse"]
    c_em = c["tp"] + c["fp"]
    c_prec = (c["tp"] / c_em) if c_em else None
    print("1. Coarse recall < 0.9 or precision < 0.7 -> fine scope mandatory for L-class, "
          "stale_policy = gate")
    if (c["tp"] + c["fn"]) == 0:
        print("   recall:    NOT ANSWERABLE — no positive labels, no denominator.")
    else:
        print(f"   recall:    {c['tp']/(c['tp']+c['fn']):.2f}")
    if c_prec is None:
        print("   precision: NOT MEASURABLE — the engine emitted no Stale at all.")
    else:
        print(f"   precision: {c_prec:.2f} {fmt_ci(wilson(c['tp'], c_em))} over {c_em} emitted "
              f"positives -> criterion {'FIRES' if c_prec < 0.7 else 'does not fire'} on its "
              f"precision half alone.")
    f = by_scope["fine"]
    f_em = f["tp"] + f["fp"]
    print("2. Fine scope cannot reach recall >= 0.9 either -> stale_policy = gate for L/XL")
    print("   NOT ANSWERABLE on this corpus, for the same reason: recall has no denominator.")
    if f_em:
        print(f"   (fine precision is {f['tp']/f_em:.2f} {fmt_ci(wilson(f['tp'], f_em))} "
              f"over {f_em} emitted positives)")
    print("3. Advisory useful fraction < 0.3 -> PC3's advisory demoted to a log line")
    print("   OUT OF SCOPE here — the advisory half (A01–A06) is labelled after the engine, in its "
          "own file (METHOD-P §2.3), and DECISIONS-2026-09-09 #15 must be settled first.")


if __name__ == "__main__":
    main()
