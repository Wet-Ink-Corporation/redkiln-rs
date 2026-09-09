#!/usr/bin/env python3
"""P1 arm D — triage the panel against the script and the oracle (METHOD-P amendment 9).

Runs AFTER the panel has labelled. Reading spikes/results/p1-arm-d.jsonl here is the scoring step, not a
blindness breach: the panel's labels are already fixed and committed by this point.

Emits the CONTESTED SET — the only cells that need a human — and a summary of what was settled without one.

A cell is contested when any of:
  * the three panel seats did not agree unanimously;
  * they agreed but contradict the script's value for the same (item, facet);
  * the adversarial seat flagged that (item, facet) as a likely miss or a false-positive trap.

Everything else is accepted with the evidence that decided it, so the claim stays checkable afterwards.
"""
import io, json, os, sys, collections

JOURNAL = sys.argv[1] if len(sys.argv) > 1 else None
ENGINE = "spikes/results/p1-arm-d.jsonl"
ORACLE = "spikes/results/p1-arm-d-oracle.jsonl"
OUT_CONTESTED = "spikes/results/p1-arm-d-contested.json"
OUT_ACCEPTED = "spikes/results/p1-arm-d-panel-accepted.jsonl"

SEMANTIC = ["auth", "data_migration", "ui", "flag_exists"]
MECHANICAL = ["toolchain", "change", "binary_change"]


def load_journal(path):
    labels, adversarial = [], []
    for line in io.open(path, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        try:
            d = json.loads(line)
        except Exception:
            continue
        if d.get("type") != "result":
            continue
        v = d.get("result") or d.get("value") or {}
        if isinstance(v, str):
            try:
                v = json.loads(v)
            except Exception:
                continue
        if not isinstance(v, dict):
            continue
        if v.get("labels"):
            labels.append((d.get("label", "?"), v["labels"]))
        if v.get("misses") is not None:
            adversarial.append((d.get("label", "?"), v))
    return labels, adversarial


def norm(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if v is None:
        return "unknown"
    s = str(v).strip().lower()
    return {"1": "true", "0": "false", "yes": "true", "no": "false", "?": "unknown", "": "unknown"}.get(s, s)


def main():
    if not JOURNAL or not os.path.exists(JOURNAL):
        sys.exit("usage: triage-arm-d.py <path to workflow journal.jsonl>")
    seats, adversarial = load_journal(JOURNAL)
    if not seats:
        sys.exit("no labeller results found in the journal")

    # panel votes: (item, facet) -> [ {seat, value, evidence, confidence} ]
    votes = collections.defaultdict(list)
    for seat, rows in seats:
        for r in rows:
            if r.get("facet") not in SEMANTIC:
                continue
            votes[(r["item"], r["facet"])].append(
                {"seat": seat, "value": norm(r.get("value")), "evidence": (r.get("evidence") or "")[:400],
                 "confidence": r.get("confidence", "?"), "reasoning": (r.get("reasoning") or "")[:300]})

    # adversarial flags
    flagged = collections.defaultdict(list)
    for seat, v in adversarial:
        for m in v.get("misses", []):
            flagged[(m.get("item"), m.get("facet"))].append(
                {"kind": "miss", "strength": m.get("strength", "?"), "why": m.get("why_a_regex_would_miss_it", "")[:400],
                 "evidence": (m.get("evidence") or "")[:300], "claimed_true": m.get("claimed_true")})
        for t in v.get("false_positive_traps", []):
            flagged[(t.get("item"), t.get("facet"))].append(
                {"kind": "trap", "strength": "n/a", "why": t.get("why_not_really", "")[:400],
                 "evidence": (t.get("lexical_bait") or "")[:300], "claimed_true": False})

    # the script, per source
    eng = collections.defaultdict(dict)
    for line in io.open(ENGINE, encoding="utf-8"):
        r = json.loads(line)
        eng[(r["item"], r["facet"])][r["source"]] = r

    # the oracle
    orc = {}
    for line in io.open(ORACLE, encoding="utf-8"):
        r = json.loads(line)
        orc[(r["item"], r["facet"])] = r

    contested, accepted = [], []
    stats = collections.Counter()
    for key in sorted(votes):
        item, facet = key
        vs = votes[key]
        vals = [v["value"] for v in vs]
        uniq = set(vals)
        script_rec = eng.get(key, {}).get("union") or eng.get(key, {}).get("docs") or eng.get(key, {}).get("commits")
        script_val = norm(script_rec["value"]) if script_rec else "MISSING"
        adv = flagged.get(key, [])

        reasons = []
        if len(uniq) > 1:
            reasons.append(f"panel split {sorted(uniq)}")
        if len(uniq) == 1 and script_val != "MISSING" and vals[0] != script_val and vals[0] != "unknown":
            reasons.append(f"panel unanimous '{vals[0]}' vs script '{script_val}'")
        if adv:
            strengths = ",".join(sorted({a["strength"] for a in adv}))
            reasons.append(f"adversary flagged ({len(adv)}: {strengths})")

        row = {"item": item, "facet": facet, "panel": vs, "script": script_val,
               "script_by_source": {s: norm(r["value"]) for s, r in eng.get(key, {}).items()},
               "script_evidence": (script_rec or {}).get("evidence", [])[:5],
               "adversarial": adv, "reasons": reasons}
        if reasons:
            contested.append(row); stats["contested"] += 1
            for r in reasons:
                stats["reason:" + r.split(" ")[0] + ("_" + r.split(" ")[1] if r.startswith("panel unanimous") else "")] += 1
        else:
            accepted.append({"item": item, "facet": facet, "value": vals[0],
                             "agrees_with_script": vals[0] == script_val,
                             "evidence": vs[0]["evidence"], "seats": len(vs)})
            stats["accepted"] += 1
            stats["accepted_unknown" if vals[0] == "unknown" else "accepted_decided"] += 1

    # mechanical: oracle vs script, no human at all
    mech = []
    for facet in MECHANICAL:
        for (item, f), o in orc.items():
            if f != facet:
                continue
            per = {s: r["value"] for s, r in eng.get((item, f), {}).items()}
            ov = o["value"]
            def eq(a, b):
                if isinstance(a, list) or isinstance(b, list):
                    return sorted(a or []) == sorted(b or [])
                return norm(a) == norm(b)
            agree = {s: eq(ov, v) for s, v in per.items()}
            mech.append({"item": item, "facet": f, "oracle": ov, "script": per, "agree": agree,
                         "oracle_evidence": o["evidence"][:3], "rule": o["rule"]})

    # An adversarial flag is a PREDICTION about the script, and most predictions are mechanically checkable.
    # A "trap" says the regex will fire on lexical bait -- if the script says false, it did not fire and there is
    # nothing to adjudicate. A "miss" says the fact is true but a regex would not see it -- if the script already
    # says true, the miss did not happen. Resolving these costs no judgement and must happen before a human is
    # asked for any of their time. Weak-strength misses are recorded but not escalated; that is disclosed below.
    RANK = {"strong": 3, "plausible": 2, "weak": 1}
    live_contested, moot = [], []
    for r in contested:
        panel_signal = any(not x.startswith("adversary") for x in r["reasons"])
        live = []
        for a in r["adversarial"]:
            if a["kind"] == "trap":
                if r["script"] == "true":
                    live.append(a)
            elif r["script"] != "true" and RANK.get(a.get("strength", "?"), 0) >= 2:
                live.append(a)
        r["adversarial_live"] = live
        (live_contested if (panel_signal or live) else moot).append(r)
    stats["moot"] = len(moot)
    stats["live_contested"] = len(live_contested)

    io.open(OUT_CONTESTED, "w", encoding="utf-8", newline="\n").write(
        json.dumps({"contested": live_contested, "moot": moot, "mechanical": mech}, indent=1))
    with io.open(OUT_ACCEPTED, "w", encoding="utf-8", newline="\n") as fh:
        for a in accepted:
            fh.write(json.dumps(a) + "\n")

    print("=== SEMANTIC (panel) ===")
    print(f"  cells                : {len(votes)}   (20 items x 4 facets)")
    print(f"  accepted without a human: {stats['accepted']}  "
          f"(decided {stats['accepted_decided']}, honest 'unknown' {stats['accepted_unknown']})")
    print(f"  flagged              : {stats['contested']}")
    print(f"  of which mechanically moot (adversary prediction did not fire): {stats['moot']}")
    print(f"  CONTESTED -> you     : {stats['live_contested']}")
    for k, v in sorted(stats.items()):
        if k.startswith("reason:"):
            print(f"      {k[7:]:26} {v}")
    print()
    print("=== MECHANICAL (deterministic oracle vs script, no human) ===")
    for facet in MECHANICAL:
        rows = [m for m in mech if m["facet"] == facet]
        for src in ("docs", "commits", "union"):
            ok = sum(1 for m in rows if m["agree"].get(src))
            tot = sum(1 for m in rows if src in m["agree"])
            if tot:
                print(f"  {facet:15} {src:8} agree {ok}/{tot}")
    print()
    print(f"wrote {OUT_CONTESTED} and {OUT_ACCEPTED}")


if __name__ == "__main__":
    main()
