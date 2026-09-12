"""QA view of constants.json (ASCII-only)."""
import json
import sys

P = (r"C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-redkiln-rs"
     r"\953a92ec-7709-4a24-b01d-8a5e2c5b86c8\scratchpad\audit\census\constants.json")
d = json.load(open(P, encoding="utf-8"))


def asc(s):
    return str(s).encode("ascii", "replace").decode()


mode = sys.argv[1] if len(sys.argv) > 1 else "summary"

if mode == "summary":
    for s in d["seeds"]:
        cands = ", ".join("%s=%d/%df" % (asc(c["value"]), c["n_occ"], c["n_files"]) for c in c_list) if (c_list := s["candidates"]) else ""
        unexp = ""
        if s["slot"] and s["slot"]["other_values"]:
            unexp = "  UNEXPECTED:" + ",".join("%s(%d)" % (asc(u["value"]), u["n"]) for u in s["slot"]["other_values"][:8])
        print("%-22s %-16s occ=%-4d files=%-3d %s%s" % (s["id"], s["status"], s["n_occ"], s["n_files"], cands, unexp))
elif mode == "zero":
    for s in d["seeds"]:
        for c in s["candidates"]:
            if c["n_occ"] == 0:
                print(s["id"], "|", asc(c["value"]), "|", asc(c["pattern"]))
elif mode == "show":
    sid = sys.argv[2]
    n = int(sys.argv[3]) if len(sys.argv) > 3 else 5
    for s in d["seeds"]:
        if s["id"] == sid:
            for c in s["candidates"]:
                print("== cand", asc(c["value"]), c["n_occ"], "occ", c["n_files"], "files")
                for o in c["occurrences"][:n]:
                    print("   %s:%d  %s" % (o["file"], o["line"], asc(o["context"])[:220]))
            if s["slot"]:
                print("== slot values:", asc(json.dumps(s["slot"]["values"])))
                for u in s["slot"]["other_values"][:10]:
                    print("   other", asc(u["value"]), u["n"])
                    for e in u["examples"][:2]:
                        print("      %s:%d %s" % (e["file"], e["line"], asc(e["context"])[:200]))
elif mode == "generic":
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 60
    only_added = len(sys.argv) > 3 and sys.argv[3] == "added"
    k = 0
    for g in d["generic"]:
        if only_added and g["covered_by_seed"]:
            continue
        print("%4d %3d %-6s %s %s" % (g["n_occ"], g["n_files"], "seed" if g["covered_by_seed"] else "ADDED", asc(g["token"]), g["kinds"]))
        k += 1
        if k >= n:
            break
elif mode == "gshow":
    t = sys.argv[2]
    for g in d["generic"]:
        if g["token"] == t:
            for o in g["occurrences"][:8]:
                print("   %s:%d  %s" % (o["file"], o["line"], asc(o["context"])[:220]))
