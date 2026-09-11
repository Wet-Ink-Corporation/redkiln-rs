"""Step 1: inventory the nine census artifacts, validate JSON, describe top-level shape."""
import json, os, sys
CENSUS = r"D:/repos/redkiln-rs/references/audit/2026-09-10/census"
EXPECTED = ["ids", "constants", "anchors", "adr-graph", "slices", "external", "known-issues", "phase1-deps", "doc-map"]

def shape(v, depth=0):
    if isinstance(v, dict):
        return {"type": "dict", "keys": len(v), "sample_keys": list(v.keys())[:40]}
    if isinstance(v, list):
        return {"type": "list", "len": len(v), "first_type": type(v[0]).__name__ if v else None}
    return {"type": type(v).__name__, "value": v if isinstance(v, (int, float, bool)) or v is None else str(v)[:80]}

out = {}
for key in EXPECTED:
    p = os.path.join(CENSUS, key + ".json")
    rec = {"path": p, "exists": os.path.exists(p)}
    if not rec["exists"]:
        out[key] = rec
        continue
    rec["bytes"] = os.path.getsize(p)
    try:
        with open(p, encoding="utf-8") as f:
            data = json.load(f)
        rec["parses"] = True
    except Exception as e:
        rec["parses"] = False
        rec["error"] = str(e)[:200]
        out[key] = rec
        continue
    rec["top_type"] = type(data).__name__
    if isinstance(data, dict):
        rec["top_keys"] = {k: shape(v) for k, v in data.items()}
    else:
        rec["top_shape"] = shape(data)
    # non-trivial: count total leaf nodes roughly
    def count(v):
        if isinstance(v, dict):
            return sum(count(x) for x in v.values())
        if isinstance(v, list):
            return sum(count(x) for x in v)
        return 1
    rec["leaf_count"] = count(data)
    rec["non_trivial"] = rec["leaf_count"] > 100
    out[key] = rec

extras = [f for f in os.listdir(CENSUS) if f.endswith(".json") and f[:-5] not in EXPECTED]
out["_extras_in_dir"] = extras
with open(os.path.join(CENSUS, "..", "reconcile_step1.json"), "w", encoding="utf-8") as f:
    json.dump(out, f, indent=1)
for k, r in out.items():
    if k.startswith("_"):
        print(k, r); continue
    print(k, r.get("bytes"), r.get("parses"), r.get("top_type"), "leaves=", r.get("leaf_count"))
    if isinstance(r.get("top_keys"), dict):
        for tk, s in r["top_keys"].items():
            print("   ", tk, json.dumps(s)[:200])
