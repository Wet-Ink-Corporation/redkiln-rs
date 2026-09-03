"""Audit and score E3 records.

Every record must clear three checks before it counts (HANDOFF, F7/F17/F20):
`card_adopted`, `node_modules_linked`, and — for `s-silent` — `fix_unreachable`.
A record failing any of them is VOID, not a miss: F7, F17 and F20 are each a
failure mode that would have manufactured a kill-criterion result out of an
instrument fault.

Usage:  python results/audit-e3.py results/e3.jsonl
"""

import json
import sys
from collections import defaultdict


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


def checks(r):
    """The three gates, plus the digest. Returns (ok, [failed names])."""
    failed = []
    if not r.get("card_adopted"):
        failed.append("card_adopted (F7/F20)")
    if not r.get("node_modules_linked"):
        failed.append("node_modules (cannot-vs-chose-not-to)")
    if r.get("condition") == "s-silent" and not r.get("fix_unreachable"):
        failed.append("fix_unreachable (F17)")
    if r.get("scored") is None:
        failed.append("no digest (F20)")
    return (not failed, failed)


def main(path):
    records = load(path)
    if not records:
        print("no E3 records in", path)
        return

    print(f"{'slug':<44} {'cond':<9} {'verdict':<9} {'strong':<7} {'wall':>6} {'cost':>7}")
    print("-" * 92)

    by_arm = defaultdict(lambda: defaultdict(list))
    voids = []
    cost = 0.0
    for r in records:
        ok, failed = checks(r)
        s = r.get("scored") or {}
        step = r.get("step") or {}
        cost += step.get("total_cost_usd") or 0.0
        verdict = "—" if not s else ("REJECT" if s.get("rejected") else "approve")
        print(
            f"{r['slug']:<44} {r['condition']:<9} {verdict:<9} "
            f"{str(s.get('strong_detection', '—')):<7} "
            f"{(step.get('wall_clock_ms') or 0) / 1000:>5.0f}s "
            f"${step.get('total_cost_usd') or 0:>6.2f}"
        )
        if not ok:
            voids.append((r["slug"], r["condition"], failed))
        else:
            by_arm[r["arm"].split("-")[0]][r["condition"]].append(s)

    if voids:
        print("\nVOID — excluded from every rate, per METHOD-E3 §9:")
        for slug, cond, failed in voids:
            print(f"  {slug} [{cond}]: {', '.join(failed)}")

    print("\n--- rates over records that cleared every check ---")
    for arm, conds in sorted(by_arm.items()):
        silent = conds.get("s-silent", [])
        clean = conds.get("clean", [])
        print(f"arm {arm}:")
        if silent:
            strong = sum(1 for s in silent if s.get("strong_detection"))
            weak = sum(1 for s in silent if s.get("weak_detection"))
            print(f"  S-SILENT n={len(silent):<3} strong detection {strong}/{len(silent)}"
                  f"   weak {weak}/{len(silent)}"
                  f"   (wrong-reason gap {weak - strong})")
        if clean:
            fp = sum(1 for s in clean if s.get("rejected"))
            print(f"  CLEAN    n={len(clean):<3} false positives  {fp}/{len(clean)}")
        # MCC needs both arms of the confusion matrix; report only when it exists.
        if silent and clean:
            tp = sum(1 for s in silent if s.get("strong_detection"))
            fn = len(silent) - tp
            fp = sum(1 for s in clean if s.get("rejected"))
            tn = len(clean) - fp
            den = ((tp + fp) * (tp + fn) * (tn + fp) * (tn + fn)) ** 0.5
            mcc = (tp * tn - fp * fn) / den if den else float("nan")
            print(f"  MCC {mcc:+.3f}   (tp={tp} fn={fn} fp={fp} tn={tn})")

    print(f"\nrecords {len(records)}   void {len(voids)}   spend ${cost:.2f}")


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "results/e3.jsonl")
