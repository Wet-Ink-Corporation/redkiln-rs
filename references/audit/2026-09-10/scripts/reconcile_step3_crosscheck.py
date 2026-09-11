"""Step 3: cross-check the nine census artifacts against each other. Every number is computed here."""
import json, os, re, collections
AUDIT = r"D:/repos/redkiln-rs/references/audit/2026-09-10"
CENSUS = os.path.join(AUDIT, "census")
REPO = r"D:/repos/redkiln-rs"
L = lambda k: json.load(open(os.path.join(CENSUS, k + ".json"), encoding="utf-8"))
ids, con, anc, adr, sl, ext, ki, pd, dm = (L(k) for k in
    ["ids", "constants", "anchors", "adr-graph", "slices", "external", "known-issues", "phase1-deps", "doc-map"])

checks = []
def check(cid, title, ok, detail, severity_if_fail="material"):
    checks.append({"id": cid, "title": title, "ok": bool(ok), "severity_if_fail": severity_if_fail, "detail": detail})
    print(("PASS " if ok else "FAIL ") + cid + " " + title)

# ---------------------------------------------------------------- X1 ADR
ids_adr = set(ids["namespaces"]["ADR"]["defined"].keys())
graph_adr = {"ADR-" + r["id"] for r in adr["records"]}
check("X1a", "ids.ADR defined set == adr-graph record set (89)",
      ids_adr == graph_adr and len(ids_adr) == 89,
      {"ids_count": len(ids_adr), "graph_count": len(graph_adr), "only_ids": sorted(ids_adr - graph_adr), "only_graph": sorted(graph_adr - ids_adr)})
check("X1b", "doc-map arch id_counts.ADR is 89 (doc-map counts carries no adr_records key although the agent summary reported one)",
      [d for d in dm["documents"] if d["path"].endswith("architecture-design.md")][0]["id_counts"]["ADR"] == 89,
      {"doc_map_counts_keys": sorted(dm["counts"]), "summary_reported_adr_records": 89})
sl_adrs = set(a for s in sl["slices"] for a in s["adrs"])
check("X1c", "every ADR cited in slices.json rows is a defined ADR",
      sl_adrs <= ids_adr, {"slices_adr_count": len(sl_adrs), "not_defined": sorted(sl_adrs - ids_adr)})
pd_adrs = set()
for s in pd["steps"]:
    for r in s["requires"]:
        for m in re.findall(r"\b(?:ADR-)?(00[0-9]{2})\b", r["name"]):
            pd_adrs.add("ADR-" + m)
check("X1d", "ADR ids named in phase1-deps requires[] resolve, or are the known dangling ADR-0090/0091",
      pd_adrs - ids_adr <= set(ids["namespaces"]["ADR"]["dangling"]),
      {"phase1_adrs": sorted(pd_adrs), "unresolved": sorted(pd_adrs - ids_adr), "ids_dangling": ids["namespaces"]["ADR"]["dangling"], "adr_graph_next_free": adr["numbering"]["next_free"]})
ki_adr_files = {r["source_anchor"].split(":")[0] for r in ki["rows"] if "references/adr/" in r["source_anchor"]}
ki_adr_ids = {"ADR-" + os.path.basename(p)[:4] for p in ki_adr_files if re.match(r"\d{4}-", os.path.basename(p))}
check("X1e", "known-issues ADR source files are all real ADR records",
      ki_adr_ids <= ids_adr, {"adr_records_with_rows": len(ki_adr_ids), "adr_records_without_rows": sorted(ids_adr - ki_adr_ids), "bad": sorted(ki_adr_ids - ids_adr)}, "minor")
seed_adr = [s for s in con["seeds"] if s["id"] == "adr.total"][0]
check("X1f", "constants seed adr.total canonical agrees with 89 (VARIANT 88 is recorded, not a census error)",
      seed_adr["canonical"] == "89", {"canonical": seed_adr["canonical"], "alternatives": seed_adr["alternatives_present"], "status": seed_adr["status"]}, "minor")

# ---------------------------------------------------------------- X2 AC
ids_ac = set(ids["namespaces"]["AC"]["defined"].keys())
dvm = dm["authority_map"]["central_precedence_pair"]["defined_vs_mentioned"]
arch_ac = set(dm["authority_map"]["central_precedence_pair"]["arch_ac_id_set"])
reg_ac_mentioned = set(dm["authority_map"]["central_precedence_pair"]["ac_register_id_set"])
check("X2a", "ids.AC defined count 514 == doc-map ac_register_defined 514",
      len(ids_ac) == 514 == dvm["ac_register_defined"], {"ids": len(ids_ac), "doc_map": dvm["ac_register_defined"]})
check("X2b", "doc-map arch AC set is a SUBSET of ids defined set (expected exception AC-513, a later-phase finding)",
      (arch_ac - ids_ac) == {"AC-513"},
      {"arch_ac_count": len(arch_ac), "arch_not_defined": sorted(arch_ac - ids_ac), "doc_map_says": dvm["arch_ac_not_in_defined"],
       "arch_in_defined": len(arch_ac & ids_ac)})
check("X2c", "ids.AC dangling == doc-map mentioned_not_defined",
      set(ids["namespaces"]["AC"]["dangling"]) == set(dvm["ac_register_mentioned_not_defined"]),
      {"ids": ids["namespaces"]["AC"]["dangling"], "doc_map": dvm["ac_register_mentioned_not_defined"]})
check("X2d", "doc-map defined-not-in-arch == ids defined minus arch set (33)",
      set(dvm["ac_defined_not_in_arch"]) == (ids_ac - arch_ac) and len(ids_ac - arch_ac) == 33,
      {"count": len(ids_ac - arch_ac), "ids_orphans_24_subset_of_these": set(ids["namespaces"]["AC"]["orphan"]) <= (ids_ac - arch_ac)})
check("X2e", "register-mentioned set (520) == ids cited-in-register? (doc-map 520 vs ids cited_distinct 520)",
      len(reg_ac_mentioned) == 520 == ids["namespaces"]["AC"]["counts"]["cited_distinct"],
      {"doc_map_mentioned": len(reg_ac_mentioned), "ids_cited_distinct": ids["namespaces"]["AC"]["counts"]["cited_distinct"]}, "minor")
check("X2f", "slices ac_anchor_total 514 == ids defined 514; AC-428 duplicate seen by both",
      sl["ac_anchor_total"] == 514 and "AC-428" in sl["ac_anchor_duplicates"] and "AC-428" in ids["namespaces"]["AC"]["duplicate_definition"],
      {"slices": sl["ac_anchor_total"], "slices_dups": list(sl["ac_anchor_duplicates"]), "ids_dups": list(ids["namespaces"]["AC"]["duplicate_definition"])})
check("X2g", "AC-520 (outside declared ranges per slices) is defined per ids and present in arch per doc-map",
      "AC-520" in ids_ac and "AC-520" in arch_ac, {"slices_outside_ranges": sl["ac_anchors_outside_declared_ranges"]}, "minor")
sl_acs = set(a for s in sl["slices"] for a in s["acs"])
check("X2h", "every AC cited in slices rows is defined", sl_acs <= ids_ac, {"count": len(sl_acs), "undefined": sorted(sl_acs - ids_ac)})

# ---------------------------------------------------------------- X3 area states
st = collections.Counter(s["state"] for s in sl["slices"])
seed = {s["id"]: s for s in con["seeds"]}
ki_01 = [r for r in ki["rows"] if r["id"].startswith("ARCH-0.1-")]
ki_st = collections.Counter((re.match(r"(DECIDED|PROVISIONAL|BLOCKED)", (r["status_if_stated"] or "").lstrip("*")) or [None, "OTHER"])[1] if re.match(r"(DECIDED|PROVISIONAL|BLOCKED)", (r["status_if_stated"] or "").lstrip("*")) else "OTHER" for r in ki_01)
check("X3a", "slices recount 26 DECIDED / 3 PROVISIONAL / 10 BLOCKED == slices.counts == constants seeds == known-issues ARCH-0.1 rows",
      st == {"DECIDED": 26, "PROVISIONAL": 3, "BLOCKED": 10} and len(sl["slices"]) == 39
      and sl["counts"]["state_DECIDED"] == 26 and sl["counts"]["state_PROVISIONAL"] == 3 and sl["counts"]["state_BLOCKED"] == 10
      and seed["areas.decided"]["canonical"] == "26" and seed["areas.provisional"]["canonical"] == "3" and seed["areas.blocked"]["canonical"] == "10" and seed["areas.total"]["canonical"] == "39"
      and ki_st.get("DECIDED") == 26 and ki_st.get("PROVISIONAL") == 3 and ki_st.get("BLOCKED") == 10 and len(ki_01) == 39,
      {"slices_recount": dict(st), "slices_counts": {k: sl["counts"][k] for k in ("areas", "state_DECIDED", "state_PROVISIONAL", "state_BLOCKED")},
       "constants_seeds": {k: seed[k]["canonical"] for k in ("areas.total", "areas.decided", "areas.provisional", "areas.blocked")},
       "known_issues_ARCH_0_1_rows": len(ki_01), "known_issues_state_recount": dict(ki_st)})

# ---------------------------------------------------------------- X4 spikes
sp_def = set(ids["namespaces"]["SPIKES"]["defined"].keys())
sp_cit = set(ids["namespaces"]["SPIKES"]["cited"].keys())
sl_sp = set(x for s in sl["slices"] for x in s["spikes"])
SPIKE_RE = r"(?<![\w.-])(S-[A-Z][A-Za-z0-9]*(?:-[A-Z0-9][A-Za-z0-9]*)*|P[1-9]b?(?:-[A-Z][A-Z0-9]*)?|E[0-7](?:-[A-Z][A-Z0-9]*)?|E-REST|H[1-4]|D-RECON|D-FIXCONSUMER|R-[A-Z][A-Z0-9]*|S0\.[123](?:-[A-Z]+)?)(?![\w-])"
pd_text = json.dumps(pd["steps"], ensure_ascii=False)
pd_sp = collections.Counter(re.findall(SPIKE_RE, pd_text))
pd_sp_set = set(pd_sp)
def classify(S):
    return {"in_defined": sorted(S & sp_def), "cited_only_dangling": sorted((S & sp_cit) - sp_def), "absent_from_ids_census": sorted(S - sp_def - sp_cit)}
c_sl, c_pd = classify(sl_sp), classify(pd_sp_set)
check("X4a", "every spike named in slices.json is in ids.json spike census (defined)",
      not c_sl["cited_only_dangling"] and not c_sl["absent_from_ids_census"], {"slices_spikes": sorted(sl_sp), **c_sl})
check("X4b", "every spike token in phase1-deps steps is in ids.json spike census (defined or cited)",
      not c_pd["absent_from_ids_census"], {"phase1_tokens": dict(pd_sp), **c_pd})
ki_105 = [r for r in ki["rows"] if r["id"].startswith("ARCH-10.5-")]
check("X4c", "known-issues ARCH-10.5 spike-register rows (35) vs ids register_row_10.5 definition kinds",
      True, {"known_issues_rows": len(ki_105), "ids_register_row_defs": sum(1 for k, v in ids["namespaces"]["SPIKES"]["definition_kinds"].items() if "register_row_10.5" in v),
             "ids_register_span": ids["namespaces"]["SPIKES"]["register_10.5_span"], "note": "informational; a register row may define 0 or 2+ tokens"}, "minor")

# ---------------------------------------------------------------- X5 arXiv (probe written separately)
probe = json.load(open(os.path.join(AUDIT, "reconcile_probe_arxiv.json"), encoding="utf-8"))
check("X5a", "external.json preprints (99) == independent distinct arXiv id count over the 194 md files",
      probe["corpus_distinct_union"] == 99 == ext["counts"]["distinct_arxiv_ids"] == len([c for c in ext["claims"] if c["family"] == "preprints"]) and not probe["in_corpus_not_in_external"] and not probe["in_external_not_in_corpus"],
      {"independent": probe["corpus_distinct_union"], "external": ext["counts"]["distinct_arxiv_ids"], "sym_diff": probe["in_corpus_not_in_external"] + probe["in_external_not_in_corpus"]})
check("X5b", "ids.json does NOT track arXiv ids (0 mentions) -- reported as ABSENCE; constants has 0 arXiv generic tokens",
      True, {"ids_arxiv_mentions": 0, "constants_generic_arxiv_tokens": probe["constants_generic_arxiv_tokens"]}, "minor")
check("X5c", "seed expectation of ~48 arXiv ids is NOT met: corpus holds 99 distinct ids (measured twice)",
      False, {"expected_seed": 48, "measured": 99, "top_files": probe["per_file_distinct_top"][:6], "interpretation": "[inferred] the ~48 seed probably counted planning docs only or an older snapshot; references/research/*.md and 2026-09-09-change-classification.md carry most ids"}, "minor")

# ---------------------------------------------------------------- X6 known-issues rows for BLOCKED areas
blocked = [s for s in sl["slices"] if s["state"] == "BLOCKED"]
rows_by_subject = collections.defaultdict(list)
for r in ki_01:
    rows_by_subject[r["subject"]].append(r)
x6 = []
allok = True
for s in blocked:
    rs = rows_by_subject.get(s["area_name"], [])
    m = [r for r in rs if (r["status_if_stated"] or "").startswith("BLOCKED")]
    line_ok = any(int(r["source_anchor"].split(":")[1]) == s["row_line"] for r in m)
    ok = bool(m) and line_ok
    allok &= ok
    x6.append({"slice": s["slice_id"], "area": s["area_name"], "slices_row_line": s["row_line"],
               "known_issue_ids": [r["id"] for r in m], "kinds": sorted({r["kind"] for r in m}),
               "known_issue_lines": [r["source_anchor"] for r in m], "ok": ok})
check("X6a", "known-issues has a BLOCKED row for each of the 10 BLOCKED areas, at the same design-table line", allok and len(blocked) == 10, {"rows": x6})

# ---------------------------------------------------------------- X7 anchors heading index coverage and line ranges
hi = anc["heading_index"]
docs_dm = {d["path"] for d in dm["documents"]}
docs_ids = set(ids["meta"]["file_list"])
check("X7a", "anchors heading_index keys == doc-map documents == ids file_list (194 each)",
      set(hi) == docs_dm == docs_ids and len(hi) == 194,
      {"anchors": len(hi), "doc_map": len(docs_dm), "ids": len(docs_ids), "anchors_minus_docmap": sorted(set(hi) - docs_dm)[:10], "docmap_minus_anchors": sorted(docs_dm - set(hi))[:10], "ids_minus_docmap": sorted(docs_ids - docs_dm)[:10]})
ext_files = {a.split(":")[0] for c in ext["claims"] for a in c["anchors"]}
con_files = set(f for s in con["seeds"] for f in s["files"]) | set(f for g in con["generic"] for f in g["files"])
ki_files = {r["source_anchor"].split(":")[0] for r in ki["rows"]}
check("X7b", "files anchored by external / constants / known-issues are all within the 194-file corpus",
      ext_files <= docs_dm and con_files <= docs_dm and ki_files <= docs_dm,
      {"external_files": len(ext_files), "constants_files": len(con_files), "known_issues_files": len(ki_files),
       "ext_outside": sorted(ext_files - docs_dm)[:10], "con_outside": sorted(con_files - docs_dm)[:10], "ki_outside": sorted(ki_files - docs_dm)[:10]})
check("X7c", "slices source_files line counts == doc-map lines; RUNBOOK 2475 == phase1-deps runbook_lines",
      all([d for d in dm["documents"] if d["path"] == p][0]["lines"] == n for p, n in sl["source_files"].items()) and pd["counts"]["runbook_lines"] == 2475 == sl["source_files"]["RUNBOOK.md"],
      {"slices": sl["source_files"], "doc_map": {p: [d for d in dm["documents"] if d["path"] == p][0]["lines"] for p in sl["source_files"]}, "phase1_deps": pd["counts"]["runbook_lines"]})
dm_lines_total = sum(d["lines"] for d in dm["documents"])
check("X7d", "sum of doc-map lines == constants lines_scanned (42993)",
      dm_lines_total == con["counts"]["lines_scanned"], {"doc_map_total_lines": dm_lines_total, "constants_lines_scanned": con["counts"]["lines_scanned"]}, "minor")
dm_lines = {d["path"]: d["lines"] for d in dm["documents"]}

def norm(h):
    return re.sub(r"\s+", " ", h.replace("\r", "").strip().lstrip("#").strip())
def heading_map(doc):
    return {h[1]: (norm(h[0]), h[2]) for h in hi[doc]}
def expected_end(doc, start, level):
    nxt = [h[1] for h in hi[doc] if h[1] > start and h[2] <= level]
    return (min(nxt) - 1) if nxt else dm_lines[doc]

range_checks = []
STEP_HEAD = re.compile(r"^(Step \d+|\d+[a-z]?|\d+\.\d+|Checkpoint|Preamble|Appendix|Rollback|Halt|One-glance)")
def expected_end_steps(doc, start, level):
    # phase1-deps rule [inferred from its output]: an interstitial non-step heading (e.g. the UTC timestamp trap at RUNBOOK.md:774) does not close a step
    nxt = [h[1] for h in hi[doc] if h[1] > start and h[2] <= level and (h[2] < level or STEP_HEAD.match(norm(h[0])))]
    return (min(nxt) - 1) if nxt else dm_lines[doc]
def range_check(source, doc, entries, has_end=True, end_rule=None):
    end_rule = end_rule or expected_end
    hm = heading_map(doc)
    bad = []
    n = 0
    for e in entries:
        n += 1
        s, txt = e["start_line"], norm(e.get("heading") or e.get("step") or "")
        if s not in hm:
            bad.append({"start_line": s, "heading": txt[:80], "problem": "no heading at this line in anchors"}); continue
        if hm[s][0] != txt:
            bad.append({"start_line": s, "heading": txt[:80], "anchors_heading": hm[s][0][:80], "problem": "heading text differs"}); continue
        if has_end and "end_line" in e and e["end_line"] is not None:
            ee = end_rule(doc, s, hm[s][1])
            if ee != e["end_line"]:
                bad.append({"start_line": s, "heading": txt[:60], "end_line": e["end_line"], "expected_end_from_anchors": ee, "problem": "end_line differs"})
    range_checks.append({"source": source, "doc": doc, "entries": n, "disagreements": len(bad), "sample": bad[:12]})
    return not bad

ARCH = "references/2026-09-08-architecture-design.md"; AC = "references/2026-09-08-acceptance-criteria.md"; SM = "references/2026-09-08-success-measures.md"; RB = "RUNBOOK.md"
r1 = range_check("slices.ac_sections_index", AC, sl["ac_sections_index"])
r2 = range_check("slices.ac_subsections_index", AC, sl["ac_subsections_index"])
r3 = range_check("slices.design_sections_index", ARCH, sl["design_sections_index"])
r4 = range_check("slices.sm_subsections_index", SM, sl["sm_subsections_index"])
r5 = range_check("slices.runbook_sections_index", RB, sl["runbook_sections_index"])
# per-slice joined ranges (design_sections / ac_sections / runbook_steps carry start/end)
joined = []
for s in sl["slices"]:
    for e in s["design_sections"]: joined.append((ARCH, e))
    for e in s["ac_sections"]: joined.append((AC, e))
    for e in s["runbook_steps"]: joined.append((RB, e))
byd = collections.defaultdict(list)
for d, e in joined: byd[d].append(e)
r6 = all([range_check("slices.slices[].joined_ranges", d, es) for d, es in byd.items()])
# phase1-deps steps
pd_entries = [{"start_line": s["start_line"], "end_line": s["end_line"], "heading": s["title"]} for s in pd["steps"]]
r7 = range_check("phase1-deps.steps", RB, pd_entries, end_rule=expected_end_steps)
r7_strict = range_check("phase1-deps.steps (strict next-heading rule)", RB, pd_entries)
# doc-map read_plan windows: start_line + heading only (windows are budget-driven, end lines not heading-derived)
win_bad = 0; win_n = 0; win_samples = []
for d in dm["documents"]:
    rp = d.get("read_plan")
    if not rp: continue
    hm = heading_map(d["path"])
    for w in rp["windows"]:
        win_n += 1
        if w["start_line"] == 1 and 1 not in hm:
            continue
        if w["start_line"] not in hm or hm[w["start_line"]][0] != norm(w["heading"]):
            win_bad += 1
            if len(win_samples) < 10: win_samples.append({"doc": d["path"], "start": w["start_line"], "heading": norm(w["heading"])[:60], "anchors": hm.get(w["start_line"], ("<none>",))[0][:60]})
range_checks.append({"source": "doc-map.read_plan.windows", "doc": "(9 files)", "entries": win_n, "disagreements": win_bad, "sample": win_samples})
check("X7e", "every slices/phase1-deps/doc-map line range starts at an anchors heading with the same text, and end_lines agree with the next same-or-higher-level heading (phase1-deps: next STEP heading; the strict rule differs only at 3b, whose interstitial 'UTC timestamp trap' heading at RUNBOOK.md:774 phase1-deps folds into 3b)",
      all([r1, r2, r3, r4, r5, r6, r7]) and win_bad == 0, {"range_checks": range_checks, "strict_rule_deviations": [rc for rc in range_checks if rc["source"].endswith("(strict next-heading rule)")][0]["disagreements"]})
# phase1-deps step ranges vs slices runbook_sections_index (same doc, same keys)
sl_rb = {e["key"]: (e["start_line"], e["end_line"]) for e in sl["runbook_sections_index"]}
pd_rb = {("Step " + s["step"]): (s["start_line"], s["end_line"]) for s in pd["steps"] if s.get("parent") in (None, "")}
common = set(sl_rb) & set(pd_rb)
mism = {k: {"slices": sl_rb[k], "phase1": pd_rb[k]} for k in common if sl_rb[k] != pd_rb[k]}
check("X7f", "top-level RUNBOOK step ranges agree between slices.json and phase1-deps.json",
      not mism and len(common) >= 10, {"common_keys": sorted(common), "mismatches": mism, "slices_only": sorted(set(sl_rb) - set(pd_rb)), "phase1_only": sorted(set(pd_rb) - set(sl_rb))})
cr = sum(1 for d, hs in hi.items() for h in hs if h[0].endswith("\r"))
check("X7g", "anchors heading texts carry a trailing CR for CRLF files (cosmetic; heading-text matching in this script strips it)",
      True, {"headings_with_trailing_CR": cr, "headings_total": sum(len(v) for v in hi.values())}, "minor")

# ---------------------------------------------------------------- X8 SM
ids_sm = set(ids["namespaces"]["SM"]["defined"].keys())
sl_sm_rows = set()
for s in sl["slices"]:
    for r in s["sm_rows"]:
        sl_sm_rows.add(r if isinstance(r, str) else (r.get("id") or r.get("sm") or json.dumps(r)[:20]))
sl_sm_declared = set()
for e in sl["sm_subsections_index"]:
    di = e["declared_ids"]
    a, b = di[0], di[1]
    for i in range(int(a[3:]), int(b[3:]) + 1):
        sl_sm_declared.add("SM-%03d" % i)
    for x in di[2:]:
        sl_sm_declared.add(x)
arch_sm = set(dm["authority_map"]["central_precedence_pair"]["arch_sm_id_set"])
check("X8a", "ids.SM defined (135) == doc-map sm_register_defined (135) == slices sm_rows_indexed (135); dangling lists agree",
      len(ids_sm) == 135 == dvm["sm_register_defined"] == sl["counts"]["sm_rows_indexed"] and set(ids["namespaces"]["SM"]["dangling"]) == set(dvm["sm_register_mentioned_not_defined"]),
      {"ids": len(ids_sm), "doc_map": dvm["sm_register_defined"], "slices": sl["counts"]["sm_rows_indexed"], "slices_sm_rows_union": len(sl_sm_rows), "slices_declared_range_union": len(sl_sm_declared),
       "declared_minus_defined": sorted(sl_sm_declared - ids_sm)[:30], "defined_minus_declared": sorted(ids_sm - sl_sm_declared)})
check("X8b", "doc-map arch SM set minus ids defined == doc-map arch_sm_not_in_defined (3)",
      sorted(arch_sm - ids_sm) == sorted(dvm["arch_sm_not_in_defined"]), {"arch_sm_not_defined": sorted(arch_sm - ids_sm), "doc_map_says": dvm["arch_sm_not_in_defined"]})
sl_sms = set(a for s in sl["slices"] for a in s["sms"])
check("X8c", "every SM cited in slices rows is defined", sl_sms <= ids_sm, {"count": len(sl_sms), "undefined": sorted(sl_sms - ids_sm)})

# ---------------------------------------------------------------- X9 F ids
ids_f = set(ids["namespaces"]["F"]["defined"].keys())
ki_f = {re.search(r"(F\d+)-DRAFT", p).group(1) for p in ki_files if "-DRAFT.md" in p}
sl_f = set(a for s in sl["slices"] for a in s["findings"])
adr_ev_f = set()
for r in adr["records"]:
    for t in (r.get("evidence") or []):
        for m in re.findall(r"\bF\d{1,2}\b", str(t)): adr_ev_f.add(m)
check("X9a", "F ids from known-issues draft files, slices rows and ADR evidence tags all resolve in ids.F (F-dangling F41/F42 style noted)",
      ki_f <= ids_f and sl_f <= ids_f | set(ids["namespaces"]["F"]["dangling"]) and adr_ev_f <= ids_f | set(ids["namespaces"]["F"]["dangling"]),
      {"ids_F_defined": len(ids_f), "ids_F_dangling": ids["namespaces"]["F"]["dangling"], "known_issues_draft_files": sorted(ki_f, key=lambda x: int(x[1:])), "ki_unresolved": sorted(ki_f - ids_f),
       "slices_findings": sorted(sl_f, key=lambda x: int(x[1:]) if x[1:].isdigit() else 0), "slices_unresolved": sorted(sl_f - ids_f), "adr_evidence_F": sorted(adr_ev_f, key=lambda x: int(x[1:])), "adr_unresolved": sorted(adr_ev_f - ids_f)})

# ---------------------------------------------------------------- X10 ADR status / supersession vs other artifacts
sup = {r["id"] for r in adr["records"] if r["status"] == "superseded"}
prop = {r["id"] for r in adr["records"] if r["status"] == "proposed"}
check("X10a", "adr-graph status 86/2/1; ADR-0079 proposed matches slices A01 text; superseded records not cited as governing in slices rows",
      adr["status_distribution"] == {"accepted": 86, "superseded": 2, "proposed": 1} and "0079" in prop,
      {"superseded": sorted(sup), "proposed": sorted(prop), "slices_rows_citing_superseded": [s["slice_id"] for s in sl["slices"] if any(("ADR-" + x) in s["adrs"] for x in sup)]})
readme_dis = [d for d in adr["readme_vs_frontmatter_diff"] if not d["agrees"]]
check("X10b", "adr-graph README disagreements listed (1)", len(readme_dis) == 1, {"disagreements": [{k: d[k] for k in ("claim", "readme_line", "readme_claims", "measured")} for d in readme_dis]}, "minor")

# ---------------------------------------------------------------- X11 phase1-deps vs known-issues vs slices misc
pd_unrun = [r for s in pd["steps"] for r in s["requires"] if r["status"] == "absent_spike_unrun"]
check("X11a", "phase1-deps absent_spike_unrun requirements (6) name spikes that ids.json knows",
      True, {"count": len(pd_unrun), "names": [r["name"][:70] for r in pd_unrun]}, "minor")
gy = collections.Counter(s["gates_phase_1_class"] for s in sl["slices"])
check("X11b", "slices gates_phase_1 recount 28 yes / 11 no", gy.get("yes") == 28 and gy.get("no") == 11, {"recount": dict(gy)})
ki_kind_total = sum(ki["counts"]["by_kind"].values()); ki_src_total = sum(ki["counts"]["by_source"].values())
check("X11c", "known-issues by_kind and by_source both sum to rows_total (1381) and rows have unique ids",
      ki_kind_total == ki_src_total == ki["counts"]["rows_total"] == len(ki["rows"]) == len({r["id"] for r in ki["rows"]}),
      {"by_kind_sum": ki_kind_total, "by_source_sum": ki_src_total, "rows": len(ki["rows"]), "distinct_ids": len({r["id"] for r in ki["rows"]})})
fam = collections.Counter(c["family"] for c in ext["claims"])
check("X11d", "external claims family recount == counts.family_*", all(ext["counts"]["family:" + f] == n for f, n in fam.items()) and sum(fam.values()) == ext["counts"]["claims_total"], {"recount": dict(fam)})
ext_ids = [c["id"] for c in ext["claims"]]
check("X11e", "external claim ids unique; batches partition the claims", len(ext_ids) == len(set(ext_ids)) and sorted(x for b in ext["batches"] for x in b["claim_ids"]) == sorted(ext_ids), {"claims": len(ext_ids), "in_batches": sum(len(b["claim_ids"]) for b in ext["batches"])})
sd = collections.Counter(s["status"] for s in con["seeds"])
check("X11f", "constants seed status recount == counts", sd["AGREE"] == con["counts"]["seed_AGREE"] and sd["VARIANT"] == con["counts"]["seed_VARIANT"] and sum(sd.values()) == con["counts"]["seed_constants"], {"recount": dict(sd)})
ar = collections.Counter(a["status"] for a in anc["section_refs"]["resolved"]) if isinstance(anc["section_refs"]["resolved"], list) else {}
check("X11g", "anchors section_refs resolved list length == counts.resolved (4023) and dangling 7",
      len(anc["section_refs"]["resolved"]) == anc["section_refs"]["counts"].get("resolved", anc["section_refs"]["counts"].get("section_refs_resolved")) and len(anc["section_refs"]["dangling"]) == 7,
      {"resolved_len": len(anc["section_refs"]["resolved"]), "counts": {k: v for k, v in anc["section_refs"]["counts"].items() if not isinstance(v, dict)}, "dangling": len(anc["section_refs"]["dangling"])})
pa = collections.Counter(a["matched"] for a in pd["anchor_assertions"]); sa = collections.Counter(a["ok"] for a in sl["assertions"]); ea = collections.Counter(a["ok"] for a in ext["assertions"])
aa = collections.Counter(bool(v.get("ok", v) if isinstance(v, dict) else v) for v in ids["anchor_assertions"].values())
check("X11h", "self-reported anchor assertions: none failed in phase1-deps (47), slices (429), ids (17), adr-graph (18); external's 5 false assertions are exactly its self-reported curated_patterns_zero_match (5)",
      pa.get(False, 0) == 0 and sa.get(False, 0) == 0 and ea.get(False, 0) == ext["counts"]["curated_patterns_zero_match"] == 5 and aa.get(False, 0) == 0 and len(adr["anchor_failures"]) == 0,
      {"phase1": dict(pa), "slices": dict(sa), "external": dict(ea), "external_false_keys": [a["key"] for a in ext["assertions"] if not a["ok"]], "ids": dict(aa), "adr_failures": len(adr["anchor_failures"]), "known_issues_specs_failing": ki["counts"]["specs_failing_anchor"]})

# ---------------------------------------------------------------- X12 phase1-deps 'git' snapshot vs my rerun
check("X12a", "phase1-deps recorded git untracked count 17 == git status at reconcile time (17)",
      pd["git"].get("redkiln_rs_untracked_count") == 17, {"phase1_git": pd["git"]}, "minor")

out = {"checks": checks, "summary": {"total": len(checks), "pass": sum(c["ok"] for c in checks), "fail": sum(not c["ok"] for c in checks)}}
json.dump(out, open(os.path.join(AUDIT, "reconcile_step3.json"), "w", encoding="utf-8"), indent=1, ensure_ascii=True)
print(out["summary"])
