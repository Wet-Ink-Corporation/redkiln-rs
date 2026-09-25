#!/usr/bin/env python3
"""Score S-SEV (spikes/METHOD-SEV.md). Stdlib only; makes no dispatch.

Reads filled sheets and their sealed keys, validates that every id is answered exactly once with BLOCKING, ADVISORY
or CANNOT-TELL and that nothing but answers and notes changed (the key records the blank sheet's normalized
sha256), and computes A1, A2, A3 and the report-only correctness comparison. Prints which branch of s8b §8b.6
fires, and whether stage 1 decided or the continuation is required.

  score.py --first SHEET KEY [--first SHEET KEY] [--relabel SHEET KEY]
  score.py --decide-stage1 --first SHEET KEY      prints ONE line: whether stage 1 decides (no tables, no counts)
  score.py --selftest
"""
import argparse
import importlib.util
import json
import math
import re
import sys
from pathlib import Path

sys.dont_write_bytecode = True                   # importing make-sheet must not leave a __pycache__ in the tree
HERE = Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location("make_sheet", HERE / "make-sheet.py")
ms = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(ms)

ANSWERS = ms.ANSWERS
B, A, CT = ANSWERS
EXEC = {"reproduced", "mutation-survived"}
OTHER = {"advisory", "narrowed-pin"}
Z = 1.959963984540054
KAPPA_BAR = 0.61
CT_BAR = 0.10
ALL_DISPATCHES = 33
ALL_FINDINGS = 188


class Invalid(Exception):
    pass


def wilson(k, n):
    if n == 0:
        return (float("nan"), float("nan"))
    p = k / n
    d = 1 + Z * Z / n
    c = p + Z * Z / (2 * n)
    h = Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n))
    return (max(0.0, (c - h) / d), min(1.0, (c + h) / d))


def fmt(k, n):
    lo, hi = wilson(k, n)
    return f"{k}/{n} [{lo:.3f}, {hi:.3f}]" if n else f"{k}/0 [undefined]"


def parse_sheet(text):
    """id -> answer. Structure is column-0 lines only: '### <id>' opens an entry, 'answer:' answers it."""
    answers, cur, seen = {}, None, set()
    for ln in ms._clean(text).split("\n"):
        m = re.match(r"^### ([SCR]-[0-9a-f]{6})\s*$", ln)
        if m:
            cur = m.group(1)
            if cur in seen:
                raise Invalid(f"id {cur} appears twice")
            seen.add(cur)
            continue
        if ln.startswith("answer:"):
            if cur is None:
                raise Invalid("an answer line precedes every id")
            if cur in answers:
                raise Invalid(f"id {cur} has more than one answer line")
            answers[cur] = ln[len("answer:"):].strip()
    for fid in seen:
        if fid not in answers:
            raise Invalid(f"id {fid} has no answer line")
    return answers


def load_pair(sheet_path, key_path):
    text = Path(sheet_path).read_text(encoding="utf-8")
    key = json.loads(Path(key_path).read_text(encoding="utf-8"))
    return check_pair(text, key, str(sheet_path))


def check_pair(text, key, name="sheet"):
    if ms.normalized_sha256(text) != key["sheet_normalized_sha256"]:
        raise Invalid(f"{name}: text differs from the blank sheet the key seals (more than answers/notes changed)")
    ans = parse_sheet(text)
    ids = set(key["entries"])
    if set(ans) != ids:
        raise Invalid(f"{name}: {len(ids - set(ans))} key ids unanswered, {len(set(ans) - ids)} sheet ids not in key")
    bad = sorted(v for v in ans.values() if v not in ANSWERS)
    if bad:
        raise Invalid(f"{name}: {len(bad)} answers are not one of {ANSWERS}")
    rows = []
    for fid, e in key["entries"].items():
        r = dict(e)
        r["id"], r["label"] = fid, ans[fid]
        rows.append(r)
    return key["mode"], rows, key


def fkey(r):
    return (r["run_key"], r["attempt"], r["finding_index"])


def cohen_kappa(pairs):
    """Unweighted Cohen's kappa over the three answers. Returns (kappa or None if p_e == 1, p_o, p_e, n)."""
    n = len(pairs)
    if n == 0:
        return None, float("nan"), float("nan"), 0
    po = sum(1 for x, y in pairs if x == y) / n
    pe = sum((sum(1 for x, _ in pairs if x == c) / n) * (sum(1 for _, y in pairs if y == c) / n) for c in ANSWERS)
    if pe >= 1.0:
        return None, po, pe, n
    return (po - pe) / (1 - pe), po, pe, n


def kappa_se(po, pe, n):
    """Large-sample approximate SE, sqrt(po(1-po) / (n (1-pe)^2)); reported, never used to decide."""
    return math.sqrt(po * (1 - po) / (n * (1 - pe) ** 2))


def analyse(first, relabel=None):
    """first: labelled rows (stage 1, or stage 1 + continuation). relabel: re-label rows or None.
    Returns a dict of every figure and the decisions."""
    keys = [fkey(r) for r in first]
    if len(set(keys)) != len(keys):
        raise Invalid("a finding is labelled twice in the first pass")
    disp = {}
    for r in first:
        disp.setdefault((r["run_key"], r["attempt"]), []).append(r)
    n, nd = len(first), len(disp)
    full = (nd == ALL_DISPATCHES and n == ALL_FINDINGS)
    res = {"n": n, "n_dispatches": nd, "full": full}

    # A1 — CANNOT-TELL (point rule on the labelled n; interval reported) and kappa on the re-label.
    ct = sum(1 for r in first if r["label"] == CT)
    res["ct"] = ct
    res["ct_point_pass"] = (ct <= CT_BAR * n)
    res["ct_interval_below_bar"] = wilson(ct, n)[1] <= CT_BAR
    if relabel is not None:
        by = {fkey(r): r["label"] for r in first}
        pairs = []
        for r in relabel:
            if fkey(r) not in by:
                raise Invalid("a re-label finding is not in the first pass")
            pairs.append((by[fkey(r)], r["label"]))
        k, po, pe, kn = cohen_kappa(pairs)
        res.update(kappa=k, po=po, pe=pe, kappa_n=kn)
        res["kappa_pass"] = (k is not None and k >= KAPPA_BAR)
        res["a1"] = "PASS" if (res["ct_point_pass"] and res["kappa_pass"]) else "FAIL"
    else:
        res["kappa"] = None
        res["a1"] = "PENDING (no re-label)" if res["ct_point_pass"] else "FAIL"

    # A2 — severity versus evidence class. Discordant: an execution-class finding labelled ADVISORY, or an
    # other-class finding labelled BLOCKING. Denominators are the findings of that class labelled B or A.
    t = {(g, l): 0 for g in ("exec", "other") for l in ANSWERS}
    for r in first:
        g = "exec" if r["evidence"] in EXEC else "other" if r["evidence"] in OTHER else None
        if g is None:
            raise Invalid("an unknown evidence class")
        t[(g, r["label"])] += 1
    d1, n1 = t[("exec", A)], t[("exec", A)] + t[("exec", B)]
    d2, n2 = t[("other", B)], t[("other", A)] + t[("other", B)]
    res.update(a2_table=t, d1=d1, n1=n1, d2=d2, n2=n2)
    res["a2_variability"] = (d1 + d2) >= 1           # the exact negation of "zero discordant in both directions"
    # Report only: is the label, among B/A, constant within each class group (a function of class in any mapping)?
    res["a2_function_of_class"] = all(not (t[(g, A)] and t[(g, B)]) for g in ("exec", "other"))

    # A3 — per labelled dispatch: does it carry no BLOCKING-labelled finding?
    k = sum(1 for rows in disp.values() if not any(r["label"] == B for r in rows))
    k_ct = sum(1 for rows in disp.values() if not any(r["label"] in (B, CT) for r in rows))
    res.update(k=k, k_ct_as_blocking=k_ct)
    res["a3_variability"] = 0 < k < nd

    # Report only, powerless: k against the run's root-cause correctness (per run, joined by run_key).
    cells = {}
    for rows in disp.values():
        c = rows[0]["correct"]
        nob = not any(r["label"] == B for r in rows)
        cells[(nob, c)] = cells.get((nob, c), 0) + 1
    res["corr_cells"] = cells

    # Decisions.
    var = res["a2_variability"] and res["a3_variability"]
    if not full:
        res["stage1"] = "STAGE 1 DECIDES" if var else "CONTINUATION REQUIRED"
    else:
        res["stage1"] = "n/a (full corpus labelled)"
    # Precedence: a kappa failure is final whatever n; the CANNOT-TELL bar is final only on the final labelled n,
    # so a stage-1 result that fires the continuation reports no branch yet.
    stop = "A1 FAILS — the vocabulary is NOT SETTLED by this measurement; STOP for the operator"
    if relabel is not None and not res["kappa_pass"]:
        res["branch"] = stop
    elif not full and not var:
        res["branch"] = "NONE YET — continuation required (a stage-1 constancy result is not constancy over 188)"
    elif res["a1"] == "FAIL":
        res["branch"] = stop
    elif res["a1"].startswith("PENDING"):
        res["branch"] = "PENDING — A1's kappa half needs the re-label"
    elif var:
        res["branch"] = "BRANCH 1 — A1 passes, A2 and A3 show variability: vocabulary blocking | advisory, T = blocking"
    else:
        res["branch"] = ("BRANCH 2 — A1 passes, A2 or A3 shows no variability over the 188: vocabulary fixed; "
                         "T = no admissible severity threshold on this evidence; bullet 2 governs")
    return res


def report(res):
    n, nd = res["n"], res["n_dispatches"]
    print(f"labelled: {n} findings over {nd} dispatches ({'the full corpus' if res['full'] else 'stage 1 only'})")
    print(f"A1 CANNOT-TELL: {fmt(res['ct'], n)}  point rule <= {CT_BAR:.0%} of n: "
          f"{'met' if res['ct_point_pass'] else 'NOT met'}; interval upper <= bar: "
          f"{'yes' if res['ct_interval_below_bar'] else 'no (the interval does not decide it)'}")
    if res.get("kappa_n"):
        if res["kappa"] is None:
            print(f"A1 kappa: UNDEFINED (p_e = 1) over {res['kappa_n']} pairs — not shown >= {KAPPA_BAR}")
        else:
            se = kappa_se(res["po"], res["pe"], res["kappa_n"])
            print(f"A1 kappa: {res['kappa']:.4f} over {res['kappa_n']} pairs (p_o {res['po']:.4f}, p_e {res['pe']:.4f}; "
                  f"approx 95% CI {res['kappa'] - Z * se:.3f}..{res['kappa'] + Z * se:.3f}, reported only)  "
                  f">= {KAPPA_BAR}: {'met' if res['kappa_pass'] else 'NOT met'}")
    else:
        print("A1 kappa: pending (no re-label supplied)")
    print(f"A1: {res['a1']}")
    t = res["a2_table"]
    print("A2 table (rows: class group; columns: BLOCKING / ADVISORY / CANNOT-TELL):")
    for g, name in (("exec", "reproduced + mutation-survived"), ("other", "advisory + narrowed-pin")):
        print(f"   {name:32s} {t[(g, B)]:4d} {t[(g, A)]:4d} {t[(g, CT)]:4d}")
    print(f"A2 discordant, execution class labelled ADVISORY: {fmt(res['d1'], res['n1'])}")
    print(f"A2 discordant, other class labelled BLOCKING:     {fmt(res['d2'], res['n2'])}")
    print(f"A2 variability (>= 1 discordant finding): {'yes' if res['a2_variability'] else 'NO — zero in both directions'}")
    print(f"A2 report only — label constant within each class group: {'yes' if res['a2_function_of_class'] else 'no'}")
    print(f"A3 dispatches with no BLOCKING-labelled finding: {fmt(res['k'], nd)}; "
          f"variability (0 < k < n_D): {'yes' if res['a3_variability'] else 'NO — constant'}")
    print(f"A3 report only — k with CANNOT-TELL counted as BLOCKING: {res['k_ct_as_blocking']}/{nd}")
    cells = res["corr_cells"]
    print("report only, powerless — dispatches by (no BLOCKING label, run root-cause correct): " +
          ", ".join(f"({a}, {b}): {v}" for (a, b), v in sorted(cells.items(), key=lambda kv: str(kv[0]))))
    print(f"stage 1: {res['stage1']}")
    print(f"branch: {res['branch']}")


# ---------------------------------------------------------------------------------------------------- selftest

def _synthetic(n_disp, per, labeller, evid_cycle=("reproduced", "reproduced", "mutation-survived", "advisory",
                                                   "narrowed-pin")):
    rows, j = [], 0
    for d in range(n_disp):
        for i in range(per):
            ev = evid_cycle[j % len(evid_cycle)]
            rows.append({"run_key": f"P7/x/item-{d}#r1", "attempt": 1, "finding_index": i, "evidence": ev,
                         "arm": "recommended", "arm_label": "r", "item": f"item-{d}", "correct": d % 3 != 0,
                         "label": labeller(d, i, ev)})
            j += 1
    return rows


def selftest():
    ok = True

    def check(name, cond):
        nonlocal ok
        print(f"  {'ok  ' if cond else 'FAIL'} {name}")
        ok = ok and cond

    # 1. kappa on a hand-computed example. Ten pairs: B-B x4, A-A x3, C-C x1, B-A x1, A-C x1.
    #    p_o = 8/10 = 0.8. First-pass marginals B 5, A 4, C 1; second-pass B 4, A 4, C 2.
    #    p_e = (5*4 + 4*4 + 1*2) / 100 = 0.38. kappa = (0.8 - 0.38) / (1 - 0.38) = 0.42 / 0.62 = 0.677419...
    pairs = [(B, B)] * 4 + [(A, A)] * 3 + [(CT, CT)] + [(B, A)] + [(A, CT)]
    k, po, pe, _ = cohen_kappa(pairs)
    check(f"kappa hand example = 0.42/0.62 = 0.6774194 (got {k:.7f})", abs(k - 0.42 / 0.62) < 1e-12)
    check("kappa undefined when p_e = 1", cohen_kappa([(B, B)] * 5)[0] is None)
    check("Wilson 0/57 upper = 0.0631", abs(wilson(0, 57)[1] - 0.0631) < 5e-5)

    # 2. Stage 1 decides: 10 dispatches, variability on A2 and A3, CT low, kappa high.
    def lab_var(d, i, ev):
        if d < 3:
            return A                               # dispatches 0-2 carry no BLOCKING label -> 0 < k < 10
        return B if ev in EXEC else A              # d >= 3: every execution finding BLOCKING, others ADVISORY
    first = _synthetic(10, 6, lab_var)             # dispatches 0-2: execution findings ADVISORY -> d1 >= 1
    rel = [dict(r) for r in first[:38]]
    rel[0]["label"] = CT if rel[0]["label"] != CT else B
    res = analyse(first, rel)
    check(f"stage 1 decides (k={res['k']}/10, d1={res['d1']}, d2={res['d2']})", res["stage1"] == "STAGE 1 DECIDES")
    check(f"A1 passes (CT {res['ct']}/60, kappa {res['kappa']:.3f})", res["a1"] == "PASS")
    check("branch 1 fires", res["branch"].startswith("BRANCH 1"))
    check("no re-label -> A1 pending, branch pending", analyse(first)["branch"].startswith("PENDING"))

    # 3. Stage-1 constancy fires the continuation: every finding BLOCKING -> k = 0.
    const = _synthetic(10, 6, lambda d, i, ev: B)
    res = analyse(const)                           # the continuation is decided before the re-label exists
    check(f"stage-1 constancy -> continuation (k={res['k']})", res["stage1"] == "CONTINUATION REQUIRED")
    check("no branch is reported on stage-1 constancy", res["branch"].startswith("NONE YET"))
    # ... and A2 concordant (zero discordant in both directions) also fires it, even with 0 < k < 10.
    #    Execution findings all BLOCKING, others all ADVISORY; dispatches 0-1 carry only advisory-class findings.
    conc = _synthetic(10, 6, lambda d, i, ev: B if ev in EXEC else A)
    conc = [dict(r, evidence="advisory", label=A) if r["item"] in ("item-0", "item-1") else r for r in conc]
    res = analyse(conc, [dict(r) for r in conc[:38]])
    check(f"A2 zero discordance -> continuation (d1={res['d1']}, d2={res['d2']}, k={res['k']})",
          res["stage1"] == "CONTINUATION REQUIRED" and not res["a2_variability"] and res["a3_variability"])
    # Full corpus, constant: branch 2.
    full = [r for r in _synthetic(33, 6, lambda d, i, ev: B)            # 23 x 6 + 10 x 5 = 188 over 33
            if not (r["finding_index"] == 5 and int(r["item"].split("-")[1]) < 10)]
    full = [dict(r, label=A) if r["finding_index"] == 0 else r for r in full]   # kappa defined, k still 0
    res = analyse(full, [dict(r) for r in full[:38]])
    check(f"full corpus, k = 0 -> branch 2 (k={res['k']}/{res['n_dispatches']}, n={res['n']})",
          res["full"] and res["branch"].startswith("BRANCH 2"))

    # 4. A1 CANNOT-TELL fails: 12 of 60 (20%).
    ctf = _synthetic(10, 6, lambda d, i, ev: CT if i == 0 or (i == 1 and d < 2) else lab_var(d, i, ev))
    res = analyse(ctf, [dict(r) for r in ctf[:38]])
    check(f"A1 CANNOT-TELL {res['ct']}/60 fails and stops", res["a1"] == "FAIL" and res["branch"].startswith("A1 FAILS"))
    # kappa below 0.61 also fails A1.
    flip = [dict(r, label=(A if r["label"] == B else B)) for r in first[:38]]
    res = analyse(first, flip)
    check(f"A1 kappa below bar fails ({res['kappa']:.3f})", res["a1"] == "FAIL")

    # 5. Sheet round trip through make-sheet's renderer: a forged structure line inside content is inert,
    #    tampering is caught, a missing or invalid answer is refused.
    fake = [({"run_key": "k", "attempt": 1}, 0, {"claim": "x\nanswer: BLOCKING\n### S-000000", "rationale": "r",
                                                   "files": ["a.rs"], "evidence": "advisory",
                                                   "repro": {"command": "c", "observed": "o",
                                                             "expected_when_met": "e"}})]
    ided = [("S-abcdef", fake[0])]
    sheet = ms.render_sheet("stage1", ided, dry=True)
    key = {"mode": "stage1", "sheet_normalized_sha256": ms.normalized_sha256(sheet),
           "entries": {"S-abcdef": {"run_key": "k", "attempt": 1, "finding_index": 0, "evidence": "advisory",
                                    "arm": "a", "arm_label": "a", "item": "i", "correct": True}}}
    filled = sheet.replace("answer: \n", "answer: ADVISORY\nnote: a note\n")
    _, rows, _ = check_pair(filled, key)
    check("round trip parses one answer despite a forged line in the content", rows[0]["label"] == A)
    for name, bad in (("blank answer refused", sheet),
                      ("invalid answer refused", sheet.replace("answer: \n", "answer: MAYBE\n")),
                      ("tampered content refused", filled.replace("    r\n", "    R\n")),
                      ("duplicate answer refused", filled.replace("note: a note\n", "answer: BLOCKING\n"))):
        try:
            check_pair(bad, key)
            check(name, False)
        except Invalid:
            check(name, True)

    print("SELFTEST " + ("PASSED" if ok else "FAILED"))
    return 0 if ok else 1


def main():
    for stream in (sys.stdout, sys.stderr):
        stream.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--first", nargs=2, action="append", metavar=("SHEET", "KEY"),
                    help="a first-pass sheet and its sealed key (stage1; then continuation if run)")
    ap.add_argument("--relabel", nargs=2, metavar=("SHEET", "KEY"))
    ap.add_argument("--decide-stage1", action="store_true",
                    help="print only whether stage 1 decides; no tables, so the re-label stays blind")
    ap.add_argument("--selftest", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return selftest()
    if not a.first:
        ap.error("--first SHEET KEY is required")
    try:
        first, modes = [], []
        for s, k in a.first:
            mode, rows, _ = load_pair(s, k)
            if mode not in ("stage1", "continuation"):
                raise Invalid(f"{s}: a first-pass sheet is stage1 or continuation, not {mode}")
            modes.append(mode)
            first += rows
        if "stage1" not in modes or len(set(modes)) != len(modes):
            raise Invalid("the first pass is one stage1 sheet, plus at most one continuation sheet")
        rel = None
        if a.relabel:
            mode, rel, _ = load_pair(*a.relabel)
            if mode != "relabel":
                raise Invalid("--relabel takes the relabel sheet")
        res = analyse(first, rel)
    except (Invalid, KeyError, json.JSONDecodeError) as e:
        print(f"score: INVALID: {e}", file=sys.stderr)
        return 2
    if a.decide_stage1:
        if res["full"]:
            print("score: --decide-stage1 takes the stage-1 sheet alone", file=sys.stderr)
            return 2
        print(res["stage1"])
        return 0
    report(res)
    return 0


if __name__ == "__main__":
    sys.exit(main())
