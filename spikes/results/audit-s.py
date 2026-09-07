"""Audit and score METHOD-S records (S-VAR and S-TYPED).

Same void rules as audit-e3.py: `card_adopted`, `node_modules_linked`, and
`fix_unreachable` on s-silent must all hold, and a digest must exist. A typed
digest that failed its schema is `void: schema` — an instrument finding, not a
miss (METHOD-S §0 rule 2).

Usage:
    python results/audit-s.py var   results/s-var.jsonl
    python results/audit-s.py typed results/s-typed.jsonl results/e3.jsonl [results/e3h.jsonl] [results/s-var-citation-targets.json]
"""

import json
import math
import sys
from collections import defaultdict

# METHOD-S §1.2 — the evidence class of each S-VAR item's KNOWN finding (F24).
# Fixed here so the secondary hypothesis is scored against a table written
# before the data, not against a reading of the data.
KNOWN_CLASS = {
    "fix-advance-write-lock-gate-scope": "reproduced",
    "issue-146-147-boundary-window": "mutation-survived",
    "issues-135-129-declared-boundary-parse": "mutation-survived",
    "issue-74-plan-briefs-dead-plan-anchor": "mutation-survived",
    "issue-63-advance-gate-diagnostic-truncated": "narrowed-pin",
    "issue-62-new-stale-pinned-template": "none",
}
CLASS_ORDER = ["reproduced", "mutation-survived", "narrowed-pin", "none"]


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
    failed = []
    if not r.get("card_adopted"):
        failed.append("card_adopted (F7/F20)")
    if not r.get("node_modules_linked"):
        failed.append("node_modules")
    if r.get("condition") == "s-silent" and not r.get("fix_unreachable"):
        failed.append("fix_unreachable (F17)")
    if r.get("scored") is None:
        kind = (r.get("step") or {}).get("error_kind")
        failed.append("no digest: schema" if kind == "schema" else "no digest (F20)")
    return (not failed, failed)


def wilson(k, n, z=1.96):
    """Wilson score interval — METHOD-S §0 rule 1."""
    if n == 0:
        return (float("nan"), float("nan"))
    p = k / n
    denom = 1 + z * z / n
    centre = (p + z * z / (2 * n)) / denom
    half = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / denom
    return (max(0.0, centre - half), min(1.0, centre + half))


def band(lo, hi):
    """METHOD-S §1.4 bands, calibrated to n=10 (10/10 and 9/10 are stable-reject;
    0/10 and 1/10 are stable-approve; 2/10 .. 8/10 are flaky)."""
    if lo > 0.55:
        return "stable-reject"
    if hi < 0.45:
        return "stable-approve"
    return "flaky"


def audit_var(path):
    recs = load(path)
    valid, voids = [], []
    for r in recs:
        ok, failed = checks(r)
        (valid if ok else voids).append((r, failed))
    print(f"S-VAR: {len(recs)} records, {len(valid)} valid, {len(voids)} void")
    for r, failed in voids:
        print(f"  void  {r['slug']:<46} r{r.get('repeat')}  {', '.join(failed)}")

    per_item = defaultdict(list)
    for r, _ in valid:
        per_item[r["slug"]].append(r)

    print("\nper-item verdict flip rate (primary — §1.4)")
    print(f"  {'item':<46} {'class':<18} n  reject  rate   wilson95        band            on-target")
    by_class = defaultdict(list)
    for slug in KNOWN_CLASS:
        rs = per_item.get(slug, [])
        n = len(rs)
        k = sum(1 for r in rs if r["scored"]["rejected"])
        cited = sum(1 for r in rs if r["scored"]["rejected"] and r["scored"]["matched_files"])
        lo, hi = wilson(k, n)
        b = band(lo, hi) if n else "-"
        rate = k / n if n else float("nan")
        print(f"  {slug:<46} {KNOWN_CLASS[slug]:<18} {n:<2} {k:<7} {rate:.2f}   [{lo:.2f}, {hi:.2f}]    {b:<15} {cited}/{k}")
        if n:
            by_class[KNOWN_CLASS[slug]].append(rate)

    print("\nevidence class vs stability (secondary — §1.4; predicted reproduced >= mutation-survived > narrowed-pin, control ~0)")
    for c in CLASS_ORDER:
        rates = by_class.get(c, [])
        if rates:
            print(f"  {c:<18} mean reject rate {sum(rates)/len(rates):.2f} over {len(rates)} item(s)")

    print("\ndecision rule (§1.5) — read the bands above against:")
    print("  reproduced flaky           -> review gate cannot be a sole blocker; pair with the deterministic gate")
    print("  mutation-survived flaky    -> block on the FINDING, not the reviewer's pass (strengthens D4)")
    print("  control rejects >= 2/10    -> false-positive floor is real; policy needs an advisory sink")


def load_targets(path):
    """METHOD-S §2.5 contradiction secondary: the F24 file targets per CLEAN item,
    pre-registered in results/s-var-citation-targets.json before any S data."""
    try:
        d = json.load(open(path, encoding="utf-8"))
        return {slug: v["targets"] for slug, v in d["items"].items()}
    except (OSError, KeyError, ValueError):
        return {}


def norm_path(s):
    return s.replace("\\", "/").removeprefix("./").lower()


def cites_target(files, targets):
    c = [norm_path(x) for x in files]
    t = [norm_path(x) for x in targets]
    return any(x.endswith(y) or y.endswith(x) for x in c for y in t)


def audit_typed(typed_path, e3_path, e3h_path=None, targets_path=None):
    typed = load(typed_path)
    prose = [r for r in load(e3_path) if r.get("repeat", 1) == 1 and r.get("arm") in ("b-s-silent", "b-clean")]
    if e3h_path:
        # METHOD-S §2.4 (amended 2026-09-06): the five S-SUBTLE seeds are the negative
        # control; the prose comparator is E3-H's sighted arm (b).
        prose += [r for r in load(e3h_path) if r.get("repeat", 1) == 1 and r.get("arm") == "b-s-subtle"]
    targets = load_targets(targets_path) if targets_path else {}

    def valid_map(recs):
        m, voids = {}, []
        for r in recs:
            ok, failed = checks(r)
            if ok:
                m[(r["condition"], r["slug"])] = r
            else:
                voids.append((r, failed))
        return m, voids

    t_map, t_void = valid_map(typed)
    p_map, _ = valid_map(prose)
    print(f"S-TYPED: {len(typed)} typed records, {len(t_map)} valid, {len(t_void)} void")
    schema_voids = [r for r, f in t_void if any("schema" in x for x in f)]
    n_typed_all = len(typed)
    print(f"  schema conformance: {n_typed_all - len(schema_voids)}/{n_typed_all}"
          f" ({100*(n_typed_all - len(schema_voids))/max(1,n_typed_all):.0f}%)  [decision rule: < 90% re-spike the enum shape]")
    for r, failed in t_void:
        print(f"  void  {r['slug']:<46} {r['condition']:<9} {', '.join(failed)}")

    for cond, label in (("s-silent", "S-SILENT strong detection"), ("s-subtle", "S-SUBTLE strong detection (negative control)"), ("clean", "CLEAN false positives")):
        t = {k: v for k, v in t_map.items() if k[0] == cond}
        p = {k: v for k, v in p_map.items() if k[0] == cond}
        paired = sorted(set(t) & set(p))
        if not t:
            continue
        print(f"\n{label} — paired n={len(paired)} (typed n={len(t)}, prose comparator n={len(p)})")
        if cond in ("s-silent", "s-subtle"):
            t_hit = sum(1 for k in paired if t[k]["scored"]["strong_detection"])
            p_hit = sum(1 for k in paired if p[k]["scored"]["strong_detection"])
            print(f"  typed/strict {t_hit}/{len(paired)}   prose {p_hit}/{len(paired)}")
            for k in paired:
                if t[k]["scored"]["strong_detection"] != p[k]["scored"]["strong_detection"]:
                    print(f"    discordant {k[1]}: typed={t[k]['scored']['strong_detection']} prose={p[k]['scored']['strong_detection']}")
        else:
            t_strict = sum(1 for k in paired if t[k]["policy"]["strict"]["blocks"])
            t_len = sum(1 for k in paired if t[k]["policy"]["lenient"]["blocks"])
            p_fp = sum(1 for k in paired if p[k]["scored"]["rejected"])
            print(f"  typed/strict {t_strict}/{len(paired)}   typed/lenient {t_len}/{len(paired)}   prose {p_fp}/{len(paired)}")
            print("  F24 items re-found and classified as adjudicated (§2.5, predicted >= 4/5):")
            for slug, known in KNOWN_CLASS.items():
                k = (cond, slug)
                if k not in t or known == "none":
                    continue
                counts = t[k]["policy"]["strict"]["counts"]
                print(f"    {slug:<46} known={known:<18} reported={ {c: n for c, n in counts.items() if n} }")
            # §2.5 registered secondary (added 2026-09-06, before any S-TYPED dispatch):
            # CONTRADICTION RATE — the F24 file is cited by a finding whose evidence
            # class is blocking-grade. This is the claim D4 rests on: the finding is
            # REPORTED even where the prose reviewer would have approved. Mechanical.
            if targets:
                print("  contradiction secondary — F24 target cited with blocking-grade evidence (reproduced|mutation-survived):")
                for slug, tg in targets.items():
                    k = (cond, slug)
                    if k not in t:
                        continue
                    d = (t[k].get("step") or {}).get("digest") or {}
                    hits = [f for f in d.get("findings", [])
                            if f.get("evidence") in ("reproduced", "mutation-survived") and cites_target(f.get("files", []), tg)]
                    weak = [f for f in d.get("findings", []) if cites_target(f.get("files", []), tg)]
                    prose_verdict = "reject" if k in p and p[k]["scored"]["rejected"] else ("approve" if k in p else "n/a")
                    print(f"    {slug:<46} blocking-grade={len(hits)} any-class={len(weak)}  prose-arm verdict={prose_verdict}")

        # Evidence-class distribution on this condition.
        agg = defaultdict(int)
        for k in t:
            for c, n in t[k]["policy"]["strict"]["counts"].items():
                agg[c] += n
        print(f"  evidence classes reported: {dict(agg)}")
        # repro fill rate: a blocking-grade finding without a command cannot be
        # re-executed, so the flip criterion (§7 amendment 1) has nothing to run.
        blocking, with_cmd = 0, 0
        for k in t:
            for f in ((t[k].get("step") or {}).get("digest") or {}).get("findings", []):
                if f.get("evidence") in ("reproduced", "mutation-survived"):
                    blocking += 1
                    if (f.get("repro") or {}).get("command", "").strip():
                        with_cmd += 1
        if blocking:
            print(f"  repro.command fill rate on blocking-grade findings: {with_cmd}/{blocking}")

        # Cost and wall clock against the prose arm (§2.7 prediction 5: within ±15%).
        def cost(r):
            s = r.get("step") or {}
            return s.get("total_cost_usd") or 0.0, s.get("wall_clock_ms") or 0
        tc = sum(cost(t[k])[0] for k in paired)
        pc = sum(cost(p[k])[0] for k in paired)
        tw = sum(cost(t[k])[1] for k in paired) / 1000
        pw = sum(cost(p[k])[1] for k in paired) / 1000
        if pc:
            print(f"  cost typed ${tc:.2f} vs prose ${pc:.2f} ({100*(tc-pc)/pc:+.0f}%); wall {tw:.0f}s vs {pw:.0f}s")


def audit_repeats(path):
    """pass^k with Wilson intervals for any (condition, slug) with more than one draw
    (METHOD-S §2.5 as amended: never compare arms on a one-item difference)."""
    recs = [r for r in load(path) if checks(r)[0]]
    by = defaultdict(list)
    for r in recs:
        by[(r["condition"], r["slug"])].append(r)
    multi = {k: v for k, v in by.items() if len(v) > 1}
    if not multi:
        return
    print("\nrepeated items — blocks per draw with Wilson 95% (report these, not point estimates)")
    for (cond, slug), rs in sorted(multi.items()):
        n = len(rs)
        k = sum(1 for r in rs if r["scored"]["rejected"])
        lo, hi = wilson(k, n)
        print(f"  {cond:<9} {slug:<46} {k}/{n}  [{lo:.2f}, {hi:.2f}]")


def main(argv):
    if len(argv) >= 3 and argv[1] == "var":
        audit_var(argv[2])
    elif len(argv) >= 4 and argv[1] == "typed":
        e3h = argv[4] if len(argv) >= 5 else None
        targets = argv[5] if len(argv) >= 6 else None
        audit_typed(argv[2], argv[3], e3h, targets)
        audit_repeats(argv[2])
    else:
        print(__doc__)
        sys.exit(2)


if __name__ == "__main__":
    main(sys.argv)
