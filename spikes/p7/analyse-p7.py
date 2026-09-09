#!/usr/bin/env python3
"""P7 — the encapsulated quality block, priced against what it catches (METHOD-P §4.3/§4.4).

Zero model cost; reads `spikes/results/p7.jsonl`.

Every rate is reported PER ITEM with a Wilson 95% interval and never pooled
first (METHOD-S §0 rule 1, F27). The pooled figure appears once, labelled as
orientation, because §4.4's criteria are per-item interval comparisons and a
pooled rate would answer a question nobody registered.

Usage:
  python spikes/p7/analyse-p7.py --records spikes/results/p7.jsonl
"""
import argparse
import collections
import io
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def wilson(k, n, z=1.96):
    if n == 0:
        return (float("nan"), float("nan"))
    p = k / n
    d = 1 + z * z / n
    centre = (p + z * z / (2 * n)) / d
    half = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, centre - half), min(1.0, centre + half))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--records", default=os.path.join(HERE, "..", "results", "p7.jsonl"))
    ap.add_argument("--items", default=os.path.join(HERE, "..", "plans", "p7-items.txt"))
    ap.add_argument("--out", default=None)
    a = ap.parse_args()

    rows = [json.loads(l) for l in io.open(a.records, encoding="utf-8") if l.strip()]
    em = [r for r in rows if r.get("phase") == "emit"]
    steps = [r for r in rows if r.get("phase") != "emit"]
    items = [l.split("\t")[0] for l in io.open(a.items, encoding="utf-8")
             if l.strip() and not l.startswith("#")]

    out = io.open(a.out, "w", encoding="utf-8", newline="\n") if a.out else sys.stdout
    w = out.write

    by = collections.defaultdict(list)
    for r in em:
        by[(r["item"], r["arm"])].append(r)
    arms = ["optional", "recommended", "mandatory"]
    present = [x for x in arms if any(k[1] == x for k in by)]

    g = next((r.get("graph") for r in rows if r.get("graph")), {})
    w("P7 — encapsulated quality: cost vs catch (METHOD-P §4)\n")
    w(f"graph {g.get('id')} v{g.get('version')} {str(g.get('sha256'))[:12]}\n")
    # The emit record's `corpus_sha` field actually holds the ITEM's checkpoint_sha,
    # one per item — a mislabelling in the driver, reported here rather than
    # silently renamed. The corpus pin is 47faec9f throughout.
    ck = sorted({r.get("corpus_sha") for r in em if r.get("corpus_sha")})
    w(f"corpus pin 47faec9f (fixed); {len(ck)} distinct per-item checkpoint shas are "
      f"recorded under the MISNAMED key `corpus_sha`\n")
    # F20 lineage: report the instrument PER ARM. A version that is not evenly
    # spread across arms confounds the comparison the criteria rest on.
    vc = collections.Counter(
        (r.get("arm"), (r.get("step") or {}).get("claude_version"))
        for r in steps if (r.get("step") or {}).get("claude_version"))
    w("claude version by arm (F20 lineage):\n")
    for (arm, v), n in sorted(vc.items()):
        w(f"    {arm:12} {v}  {n} dispatches\n")
    if len({v for _, v in vc}) > 1:
        w("  !! THE INSTRUMENT CHANGED MID-RUN — see section 8 for the per-repeat split\n")
    w(f"records={len(rows)} graph_runs={len(em)} "
      f"void={sum(1 for r in em if r.get('outcome') != 'ok')}\n")
    cost = sum((r.get("step") or {}).get("total_cost_usd") or 0 for r in rows)
    w(f"total dispatch cost=${cost:.2f}\n\n")

    # ---- 1. per item per arm, the primary metric ------------------------
    w("### 1. Root cause correct, per item per arm, Wilson 95% [measured]\n\n")
    w("| item | " + " | ".join(f"{x} k/n [95%]" for x in present) + " |\n")
    w("| --- | " + " | ".join("---" for _ in present) + " |\n")
    rates = {}
    for i in items:
        cells = []
        for x in present:
            rs = by.get((i, x), [])
            k = sum(1 for r in rs if (r.get("scored") or {}).get("correct"))
            n = len(rs)
            rates[(i, x)] = (k, n)
            if n == 0:
                cells.append("—")
            else:
                lo, hi = wilson(k, n)
                cells.append(f"{k}/{n} [{lo:.2f}, {hi:.2f}]")
        w(f"| `{i[:44]}` | " + " | ".join(cells) + " |\n")

    # ---- 2. §4.4 criterion 1 -------------------------------------------
    w("\n### 2. §4.4 criterion 1 — does `recommended` catch more than `optional`? [measured]\n\n")
    w("> \"`recommended` catches **no more root causes than `optional`** on the pooled per-item rates\n")
    w("> (interval overlap on every item) → the adversary block's default is `mandatory`-only and the\n")
    w("> `quality` shorthand defaults to `{ adversary = none }`.\"\n\n")
    w("| item | optional | recommended | intervals overlap? | recommended strictly better? |\n")
    w("| --- | --- | --- | --- | --- |\n")
    all_overlap = True
    better = []
    for i in items:
        ko, no = rates.get((i, "optional"), (0, 0))
        kr, nr = rates.get((i, "recommended"), (0, 0))
        if no == 0 or nr == 0:
            w(f"| `{i[:40]}` | {ko}/{no} | {kr}/{nr} | (incomplete) | — |\n")
            continue
        lo, ho = wilson(ko, no)
        lr, hr = wilson(kr, nr)
        ov = lo <= hr and lr <= ho
        all_overlap = all_overlap and ov
        strict = (kr / nr) > (ko / no)
        if strict:
            better.append(i)
        w(f"| `{i[:40]}` | {ko}/{no} | {kr}/{nr} | {'yes' if ov else '**NO**'} | "
          f"{'**yes**' if strict else 'no'} |\n")
    w(f"\nIntervals overlap on **every** item: **{all_overlap}**.\n")
    w(f"Items where `recommended`'s point rate is strictly higher: **{len(better)}** "
      f"{better if better else ''}.\n")
    w("\n**Verdict:** " + (
        "on the pre-registered reading the criterion **FIRES** — every interval overlaps, so the block "
        "does not demonstrably catch more.\n" if all_overlap else
        "the criterion **does not fire** — at least one item separates.\n"))
    if all_overlap and better:
        w(f"\nRead with care: {len(better)} item(s) show a strictly higher point rate for `recommended` "
          "even though the intervals overlap. At n=3 per cell a Wilson interval is wide enough that "
          "overlap is close to guaranteed, so 'overlap on every item' is a weak test here and the "
          "criterion's power, not its verdict, is what this run establishes. See Limits.\n")

    # ---- 3. cost --------------------------------------------------------
    w("\n### 3. Cost per graph run [measured]\n\n")
    w("| arm | runs | mean $ | median $ | total $ | mean dispatches |\n| --- | --- | --- | --- | --- | --- |\n")
    means = {}
    for x in present:
        rs = [r for r in em if r["arm"] == x]
        cs = sorted(r.get("run_cost_usd") or 0 for r in rs)
        m = sum(cs) / len(cs)
        means[x] = m
        md = cs[len(cs) // 2]
        dsp = [r.get("model_dispatches") for r in rs if r.get("model_dispatches") is not None]
        w(f"| `{x}` | {len(rs)} | {m:.2f} | {md:.2f} | {sum(cs):.2f} | "
          f"{(sum(dsp)/len(dsp) if dsp else float('nan')):.1f} |\n")
    if "optional" in means and "recommended" in means:
        w(f"\n`recommended` / `optional` = **{means['recommended']/means['optional']:.2f}×**\n")
    if "mandatory" in means and "recommended" in means:
        w(f"`mandatory` / `recommended` = **{means['mandatory']/means['recommended']:.2f}×** "
          f"(n={len([r for r in em if r['arm']=='mandatory'])} — §4.4 criterion 2 is NOT decidable from this)\n")

    # ---- 4. loop-backs, gate arrivals ----------------------------------
    w("\n### 4. Loop-backs and gate arrivals [measured]\n\n")
    w("| arm | mean hypothesise passes | total loop-backs | total `no-hypothesis` arrivals |\n")
    w("| --- | --- | --- | --- |\n")
    for x in present:
        rs = [r for r in em if r["arm"] == x]
        hp = [r.get("hypothesise_passes") or 0 for r in rs]
        w(f"| `{x}` | {sum(hp)/len(hp):.2f} | {sum(r.get('loop_backs') or 0 for r in rs)} | "
          f"{sum(r.get('no_hypothesis_arrivals') or 0 for r in rs)} |\n")

    # ---- 5. adversary findings by evidence class -----------------------
    w("\n### 5. Adversary findings by evidence class [measured]\n\n")
    adv = [r for r in steps if r.get("phase") == "adversary"]
    cls = collections.Counter()
    per = []
    for r in adv:
        fs = ((r.get("digest") or {}).get("findings")) or []
        per.append(len(fs))
        for f in fs:
            cls[f.get("evidence")] += 1
    if adv:
        w(f"{len(adv)} adversary dispatches, {sum(per)} findings, "
          f"mean {sum(per)/len(adv):.1f} per dispatch.\n\n")
        w("| evidence class | findings | blocks under the catalog's policy? |\n| --- | --- | --- |\n")
        for c, n in cls.most_common():
            blocks = c in ("reproduced", "mutation-survived")
            w(f"| `{c}` | {n} | {'yes' if blocks else 'no'} |\n")
    pol = [r for r in steps if r.get("phase") == "policy"]
    if pol:
        npass = sum(1 for r in pol if r.get("pass"))
        w(f"\n`policy` evaluated {len(pol)} times: **{npass} pass, {len(pol)-npass} false**. "
          "The check is `findings.none(evidence in ['Reproduced','MutationSurvived'])`, mapped to "
          "`e3::LENIENT`.\n")

    # ---- 6. deviation channel ------------------------------------------
    w("\n### 6. Deviation channel (P4) [measured]\n\n")
    dev = [r for r in em if r.get("request_deviation")]
    w(f"`RequestDeviation` emitted on **{len(dev)}** of {len(em)} graph runs "
      f"({len(dev)/max(len(em),1):.1%}); requests per item = "
      f"{len(dev)/max(len({i for i,_ in by}),1):.2f}.\n")
    if not dev:
        w("\nWith zero requests, §4.4's deviation thresholds (<30% warranted, >1 per item, <10%) have an "
          "**empty denominator** and are not decidable. The channel was available on every investigator "
          "dispatch and was never used — which is itself the finding: the investigators did not want to "
          "deviate from this graph.\n")

    # ---- 7. fixture difficulty -----------------------------------------
    w("\n### 7. §4.4 criterion 4 — is the fixture set too easy? [measured]\n\n")
    w("> \"If `optional` already reaches the answer key on **≥ 9 of 10 items**, the fixture set is too easy\n")
    w("> for the question; report and re-run on E3-H subtle fixtures before deciding.\"\n\n")
    reached = [i for i in items if rates.get((i, "optional"), (0, 0))[0] > 0]
    allthree = [i for i in items if rates.get((i, "optional"), (0, 0)) == (3, 3)]
    w(f"Items where `optional` reached the key on **at least one** draw: **{len(reached)}/10**.\n")
    w(f"Items where `optional` reached it on **all three**: **{len(allthree)}/10**.\n\n")
    w("**Verdict:** " + ("the clause **FIRES** — halt and report; do not decide on this fixture set.\n"
                         if len(reached) >= 9 else
                         "the clause **does not fire**; the fixture set is not too easy on either reading.\n"))

    # ---- 8. instrument lineage -----------------------------------------
    w("\n### 8. Per-repeat correctness and the instrument behind each draw [measured]\n\n")
    w("F20 records the instrument on every record. It changed during this run, so the split is\n")
    w("reported rather than buried: had a version moved the metric, it would show as a repeat\n")
    w("whose rate departs from its siblings.\n\n")
    w("| arm | repeat | correct / items | claude |\n| --- | --- | --- | --- |\n")
    for x in present:
        for rep in sorted({r["repeat"] for r in em if r["arm"] == x}):
            rs = [r for r in em if r["arm"] == x and r["repeat"] == rep]
            k = sum(1 for r in rs if (r.get("scored") or {}).get("correct"))
            vs = sorted({(r.get("step") or {}).get("claude_version")
                         for r in steps if r.get("arm") == x and r.get("repeat") == rep
                         and (r.get("step") or {}).get("claude_version")})
            w(f"| `{x}` | r{rep} | {k}/{len(rs)} | " + ", ".join(v.split()[0] for v in vs) + " |\n")

    w("\n### Orientation only, NOT a §4.4 criterion\n\n")
    for x in present:
        k = sum(1 for r in em if r["arm"] == x and (r.get("scored") or {}).get("correct"))
        n = sum(1 for r in em if r["arm"] == x)
        w(f"- pooled `{x}`: {k}/{n} = {k/n:.2f}\n")
    w("\nPooled rates are reported once, here, and are not used for any decision: §4.4's criteria are "
      "per-item interval comparisons, and F27 is why.\n")

    if a.out:
        out.close()
        print(f"wrote {a.out}")


if __name__ == "__main__":
    main()
