#!/usr/bin/env python3
"""P1 arm D — final scoring under METHOD-P amendment 9 (panel-adjudicated oracle).

Assembles the truth set from three provenances and scores derive-facets.py against it, per facet per source.

  MECHANICAL (toolchain, change, binary_change)  <- spikes/p1/oracle-mechanical.py, deterministic, no human
  SEMANTIC, uncontested                          <- unanimous 3-seat panel, accepted with its evidence
  SEMANTIC, contested                            <- Ryan's blind adjudication (32 cells)

`unknown` is excluded from scoring, never counted against the script (METHOD-P §1.4).
Report this as PANEL-ADJUDICATED, never as human-panel (amendment 9).

Standing caution recorded with the result: the uncontested cells rest on unanimous 3-seat agreement, and
"Nine Judges, Two Effective Votes" measured a 9-judge panel supplying ~2.18 effective independent votes with
accuracy 8-22 points short of independent-vote predictions. Unanimity here is a weaker correctness signal than
its arithmetic suggests; the contested cells carry a human and the uncontested ones do not.
"""
import io, json, collections, math

ENGINE = "spikes/results/p1-arm-d.jsonl"
ORACLE = "spikes/results/p1-arm-d-oracle.jsonl"
ACCEPTED = "spikes/results/p1-arm-d-panel-accepted.jsonl"
CONTESTED = "spikes/results/p1-arm-d-contested.json"
ADJ = "spikes/results/p1-arm-d-adjudications.json"

SEMANTIC = ["auth", "data_migration", "ui", "flag_exists"]
MECHANICAL = ["toolchain", "change", "binary_change"]


def norm(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if v is None:
        return "unknown"
    return str(v).strip().lower()


def wilson(k, n, z=1.96):
    if n == 0:
        return None
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


def main():
    truth, prov = {}, {}

    for line in io.open(ORACLE, encoding="utf-8"):
        r = json.loads(line)
        # set-valued facets stay lists; binary_change is a JSON bool and must be normalised to the same
        # string domain as the panel/human labels, or `t == "true"` silently reads every positive as false.
        v = r["value"] if r["facet"] in ("toolchain", "change") else norm(r["value"])
        truth[(r["item"], r["facet"])] = v
        prov[(r["item"], r["facet"])] = "oracle"

    for line in io.open(ACCEPTED, encoding="utf-8"):
        r = json.loads(line)
        truth[(r["item"], r["facet"])] = norm(r["value"])
        prov[(r["item"], r["facet"])] = "panel"

    # cells whose only flag was a mechanically-moot adversarial prediction keep the panel's unanimous value
    c = json.load(io.open(CONTESTED, encoding="utf-8"))
    for r in c.get("moot", []):
        vals = {norm(p["value"]) for p in r["panel"]}
        if len(vals) == 1:
            truth[(r["item"], r["facet"])] = vals.pop()
            prov[(r["item"], r["facet"])] = "panel(moot-flag)"

    for r in json.load(io.open(ADJ, encoding="utf-8")):
        truth[(r["item"], r["facet"])] = norm(r["value"])
        prov[(r["item"], r["facet"])] = "human"

    eng = collections.defaultdict(dict)
    for line in io.open(ENGINE, encoding="utf-8"):
        r = json.loads(line)
        eng[(r["item"], r["facet"])][r["source"]] = r["value"]

    print(f"truth cells: {len(truth)}   provenance: {dict(collections.Counter(prov.values()))}\n")
    print(f"{'facet':16}{'source':9}{'prec':>7}{'  95% CI':>16}{'rec':>7}   tp  fp  fn  tn  excl")
    print("-" * 84)
    verdicts = {}
    for facet in MECHANICAL + SEMANTIC:
        for source in ("docs", "commits", "union"):
            tp = fp = fn = tn = excl = 0
            for (item, f), t in truth.items():
                if f != facet:
                    continue
                if source not in eng.get((item, f), {}):
                    continue
                g = eng[(item, f)][source]
                if facet in ("toolchain", "change"):
                    T, G = set(t or []), set(g or [])
                    tp += len(T & G); fp += len(G - T); fn += len(T - G)
                else:
                    if t == "unknown":
                        excl += 1; continue
                    T, G = t == "true", norm(g) == "true"
                    tp += T and G; fp += G and not T; fn += T and not G; tn += (not T) and (not G)
            p = tp / (tp + fp) if tp + fp else None
            r = tp / (tp + fn) if tp + fn else None
            ci = wilson(tp, tp + fp) if tp + fp else None
            cis = f"[{ci[0]:.2f},{ci[1]:.2f}]" if ci else "     n/a"
            print(f"{facet:16}{source:9}{(f'{p:.2f}' if p is not None else '  n/a'):>7}{cis:>16}"
                  f"{(f'{r:.2f}' if r is not None else '  n/a'):>7}  {tp:3} {fp:3} {fn:3} {tn:3}  {excl:3}")
            if source == "union":
                verdicts[facet] = (p, ci, tp + fp, excl)
        print()

    print("=" * 84)
    print("VERDICT on the union source — METHOD-P §1.4: precision >= 0.95 removes the facet from the model")
    print("=" * 84)
    for facet, (p, ci, n, excl) in verdicts.items():
        if p is None:
            print(f"  {facet:16} NOT MEASURABLE — the script emitted no positive; nothing to be precise about"
                  f"{f' ({excl} cells excluded as unknown)' if excl else ''}")
        else:
            ok = "LEAVES THE MODEL" if ci and ci[0] >= 0.95 else ("point >= 0.95 but interval does not clear"
                                                                 if p >= 0.95 else "STAYS")
            print(f"  {facet:16} precision {p:.2f} over {n} positives, 95% CI [{ci[0]:.2f},{ci[1]:.2f}] -> {ok}")
    print()
    print("Reported as PANEL-ADJUDICATED (METHOD-P amendment 9), never as human-panel.")


if __name__ == "__main__":
    main()
