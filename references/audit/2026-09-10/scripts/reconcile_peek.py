"""Peek at sub-structures of census artifacts with truncation (never dumps whole files)."""
import json, sys
CENSUS = r"C:/Users/ryanm/AppData/Local/Temp/claude/D--repos-redkiln-rs/953a92ec-7709-4a24-b01d-8a5e2c5b86c8/scratchpad/audit/census"
cache = {}
def load(k):
    if k not in cache:
        cache[k] = json.load(open(f"{CENSUS}/{k}.json", encoding="utf-8"))
    return cache[k]
def get(obj, path):
    for seg in path.split("/"):
        if seg == "":
            continue
        if isinstance(obj, list):
            obj = obj[int(seg)]
        else:
            obj = obj[seg]
    return obj
def show(v, width=300):
    if isinstance(v, dict):
        print("  dict keys:", list(v.keys())[:60])
        for k in list(v.keys())[:8]:
            print("   ", k, "=>", json.dumps(v[k], ensure_ascii=True)[:width])
    elif isinstance(v, list):
        print("  list len", len(v))
        for x in v[:4]:
            print("   -", json.dumps(x, ensure_ascii=True)[:width])
    else:
        print("  ", json.dumps(v, ensure_ascii=True)[:width])
for spec in sys.argv[1:]:
    k, _, path = spec.partition(":")
    print("==", spec)
    try:
        show(get(load(k), path))
    except Exception as e:
        print("  ERR", e)
