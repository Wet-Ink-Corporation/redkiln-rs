"""Step 2: re-run every census script into a TEMP dir and diff against the committed artifact.

Each script hard-codes its output path. We copy the script text into rerun/scripts/ with ONLY the
output-path constant rewritten (recorded per script), run it with cwd=repo, then compare JSON
structurally, ignoring only volatile keys that the artifact itself documents as excluded from its
reproducibility hash (generated_at / generated) plus generator path strings.
"""
import json, os, re, shutil, subprocess, sys, time

AUDIT = r"D:/repos/redkiln-rs/references/audit/2026-09-10"
SCRIPTS = os.path.join(AUDIT, "scripts")
CENSUS = os.path.join(AUDIT, "census")
RERUN = os.path.join(AUDIT, "rerun")
RERUN_SCRIPTS = os.path.join(RERUN, "scripts")
RERUN_CENSUS = os.path.join(RERUN, "census")
REPO = r"D:/repos/redkiln-rs"
os.makedirs(RERUN_SCRIPTS, exist_ok=True)
os.makedirs(RERUN_CENSUS, exist_ok=True)

# (key, script, artifact, list of (old, new) substitutions, env)
PLAN = [
    ("ids", "census_ids.py", "ids.json", [], {}),  # relative to __file__ -> rerun/census automatically
    ("constants", "census_constants.py", "constants.json",
     [('os.path.join(SCRATCH, "census", "constants.json")', 'os.path.join(SCRATCH, "rerun", "census", "constants.json")')], {}),
    ("anchors", "census_anchors.py", "anchors.json",
     [('references/audit/2026-09-10/census/anchors.json', 'references/audit/2026-09-10/rerun/census/anchors.json')], {}),
    ("adr-graph", "census_adr.py", "adr-graph.json",
     [('references/audit/2026-09-10/census/adr-graph.json', 'references/audit/2026-09-10/rerun/census/adr-graph.json')], {}),
    ("slices", "census_slices.py", "slices.json",
     [('references/audit/2026-09-10/census/slices.json', 'references/audit/2026-09-10/rerun/census/slices.json')], {}),
    ("external", "census_external.py", "external.json",
     [('references/audit/2026-09-10/census/external.json', 'references/audit/2026-09-10/rerun/census/external.json')], {}),
    ("known-issues", "census_known.py", "known-issues.json",
     [('references/audit/2026-09-10/census/known-issues.json', 'references/audit/2026-09-10/rerun/census/known-issues.json')], {}),
    ("phase1-deps", "census_deps.py", "phase1-deps.json", [], {"RK_AUDIT_OUT": RERUN_CENSUS}),
    ("doc-map", "census_docmap.py", "doc-map.json",
     [('OUT / "census" / "doc-map.json"', 'OUT / "rerun" / "census" / "doc-map.json"')], {}),
]

VOLATILE_KEYS = {"generated_at", "generated", "generated_by", "generator", "python", "run_at", "timestamp", "script",
                 "content_sha256_excluding_generated"}  # constants.json hashes meta.script (a path), so the hash moves with the script location

def strip(v, path=""):
    if isinstance(v, dict):
        return {k: strip(x, path + "/" + k) for k, x in v.items() if k not in VOLATILE_KEYS}
    if isinstance(v, list):
        return [strip(x, path + "/[]") for x in v]
    return v

def diff(a, b, path="", out=None, limit=200):
    if out is None:
        out = []
    if len(out) >= limit:
        return out
    if type(a) != type(b):
        out.append({"path": path, "kind": "type", "a": str(a)[:120], "b": str(b)[:120]}); return out
    if isinstance(a, dict):
        for k in set(a) | set(b):
            if k not in a:
                out.append({"path": path + "/" + k, "kind": "only_in_rerun", "b": str(b[k])[:120]})
            elif k not in b:
                out.append({"path": path + "/" + k, "kind": "only_in_committed", "a": str(a[k])[:120]})
            else:
                diff(a[k], b[k], path + "/" + k, out, limit)
    elif isinstance(a, list):
        if len(a) != len(b):
            out.append({"path": path, "kind": "len", "a": len(a), "b": len(b)})
        for i, (x, y) in enumerate(zip(a, b)):
            diff(x, y, path + "/[%d]" % i, out, limit)
    else:
        if a != b:
            out.append({"path": path, "kind": "value", "a": str(a)[:120], "b": str(b)[:120]})
    return out

results = {}
for key, script, artifact, subs, env in PLAN:
    src = os.path.join(SCRIPTS, script)
    text = open(src, encoding="utf-8").read()
    applied = []
    for old, new in subs:
        n = text.count(old)
        applied.append({"old": old, "count": n})
        if n == 0:
            results[key] = {"status": "SUBSTITUTION_FAILED", "subs": applied}
            continue
        text = text.replace(old, new)
    if key in results:
        continue
    dst = os.path.join(RERUN_SCRIPTS, script)
    with open(dst, "w", encoding="utf-8") as f:
        f.write(text)
    # also copy probe_lib in case of local import
    for helper in ("probe_lib.py",):
        shutil.copy(os.path.join(SCRIPTS, helper), os.path.join(RERUN_SCRIPTS, helper))
    target = os.path.join(RERUN_CENSUS, artifact)
    if os.path.exists(target):
        os.remove(target)
    e = dict(os.environ); e.update(env); e["PYTHONIOENCODING"] = "utf-8"
    t0 = time.time()
    try:
        p = subprocess.run([sys.executable, dst], cwd=REPO, capture_output=True, text=True, timeout=1500, env=e)
        rc, so, se = p.returncode, p.stdout, p.stderr
    except subprocess.TimeoutExpired:
        rc, so, se = -1, "", "TIMEOUT"
    dt = round(time.time() - t0, 1)
    rec = {"script": script, "subs": applied, "rc": rc, "seconds": dt,
           "stdout_tail": so[-600:], "stderr_tail": se[-800:], "wrote_artifact": os.path.exists(target)}
    if rc == 0 and os.path.exists(target):
        a = json.load(open(os.path.join(CENSUS, artifact), encoding="utf-8"))
        b = json.load(open(target, encoding="utf-8"))
        raw_equal = json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True)
        sa, sb = strip(a), strip(b)
        d = diff(sa, sb)
        rec["byte_identical_json"] = raw_equal
        rec["identical_after_stripping_volatile"] = (len(d) == 0)
        rec["diff_count_capped_200"] = len(d)
        rec["diff_sample"] = d[:40]
        rec["committed_bytes"] = os.path.getsize(os.path.join(CENSUS, artifact))
        rec["rerun_bytes"] = os.path.getsize(target)
        rec["status"] = "REPRODUCIBLE" if not d else "DIVERGES"
    else:
        rec["status"] = "FAILED_TO_RUN"
    results[key] = rec
    print(key, rec["status"], "rc=", rc, "s=", dt, "diffs=", rec.get("diff_count_capped_200"))
    sys.stdout.flush()

# also verify the repo was not mutated by the reruns
g = subprocess.run(["git", "-C", REPO, "status", "--porcelain"], capture_output=True, text=True)
results["_git_porcelain_after"] = g.stdout.splitlines()
with open(os.path.join(AUDIT, "reconcile_step2.json"), "w", encoding="utf-8") as f:
    json.dump(results, f, indent=1)
print("git porcelain lines after rerun:", len(results["_git_porcelain_after"]))
