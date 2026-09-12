"""Independent arXiv-id count over the corpus (.md outside fixtures / runner target) vs external.json."""
import os, re, json
REPO = r"D:/repos/redkiln-rs"
CENSUS = r"C:/Users/ryanm/AppData/Local/Temp/claude/D--repos-redkiln-rs/953a92ec-7709-4a24-b01d-8a5e2c5b86c8/scratchpad/audit/census"
ids_prefix, ids_url = set(), set()
files = 0
per_file = {}
for root, dirs, fs in os.walk(REPO):
    r = root.replace("\\", "/")
    if "spikes/fixtures" in r or "/.git" in r or "runner/target" in r:
        dirs[:] = []
        continue
    for f in fs:
        if not f.endswith(".md"):
            continue
        files += 1
        p = os.path.join(root, f)
        t = open(p, encoding="utf-8", errors="replace").read()
        a = set(re.findall(r"arXiv:\s?(\d{4}\.\d{4,5})", t, flags=re.I))
        b = set(re.findall(r"arxiv\.org/(?:abs|pdf|html)/(\d{4}\.\d{4,5})", t, flags=re.I))
        ids_prefix |= a; ids_url |= b
        if a | b:
            per_file[os.path.relpath(p, REPO).replace("\\", "/")] = len(a | b)
u = ids_prefix | ids_url
e = json.load(open(f"{CENSUS}/external.json", encoding="utf-8"))
pre = [c for c in e["claims"] if c["family"] == "preprints"]
ext = set()
for c in pre:
    for m in re.findall(r"(\d{4}\.\d{4,5})", c["cited_source"] + " " + (c["verify_url"] or "")):
        ext.add(m)
c = json.load(open(f"{CENSUS}/constants.json", encoding="utf-8"))
g = [x for x in c["generic"] if "arxiv" in x["token"].lower()]
out = {
    "md_files_scanned": files,
    "corpus_distinct_prefix_form": len(ids_prefix),
    "corpus_distinct_url_form": len(ids_url),
    "corpus_distinct_union": len(u),
    "external_preprint_claims": len(pre),
    "external_distinct_ids_in_cited_source_or_verify_url": len(ext),
    "in_corpus_not_in_external": sorted(u - ext),
    "in_external_not_in_corpus": sorted(ext - u),
    "per_file_distinct_top": sorted(per_file.items(), key=lambda kv: -kv[1])[:12],
    "constants_generic_arxiv_tokens": len(g),
}
json.dump(out, open(f"{CENSUS}/../reconcile_probe_arxiv.json", "w", encoding="utf-8"), indent=1)
for k, v in out.items():
    print(k, json.dumps(v)[:300])
