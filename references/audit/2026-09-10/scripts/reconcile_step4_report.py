"""Step 4: assemble reconcile.json from steps 1-3 plus a deduplicated anomaly pool harvested from the artifacts.

Anomaly-pool rule: only ids.json carries a structured `anomalies` list. The other eight artifacts reported an
anomaly_count in their prose summaries but hold no such list, so their anomalies are HARVESTED here from the
structured anomaly-equivalent records each artifact does carry (dangling refs, VARIANT seeds, hazards, stale-risk,
zero-match patterns, README disagreements, duplicate/out-of-range anchors). The pool therefore is not the same set
the agents counted; the per-agent reported count is carried alongside for comparison.
"""
import json, os, re, collections, subprocess, hashlib
AUDIT = r"D:/repos/redkiln-rs/references/audit/2026-09-10"
CENSUS = os.path.join(AUDIT, "census")
REPO = r"D:/repos/redkiln-rs"
L = lambda k: json.load(open(os.path.join(CENSUS, k + ".json"), encoding="utf-8"))
ids, con, anc, adr, sl, ext, ki, pd, dm = (L(k) for k in
    ["ids", "constants", "anchors", "adr-graph", "slices", "external", "known-issues", "phase1-deps", "doc-map"])
step1 = json.load(open(os.path.join(AUDIT, "reconcile_step1.json"), encoding="utf-8"))
step2 = json.load(open(os.path.join(AUDIT, "reconcile_step2.json"), encoding="utf-8"))
step3 = json.load(open(os.path.join(AUDIT, "reconcile_step3.json"), encoding="utf-8"))
probe = json.load(open(os.path.join(AUDIT, "reconcile_probe_arxiv.json"), encoding="utf-8"))

REPORTED_ANOMALY_COUNTS = {"ids": 13, "constants": 9, "anchors": 11, "adr-graph": 6, "slices": 9, "external": 9, "known-issues": 6, "phase1-deps": 10, "doc-map": 9}
# note: the orchestrator's ids row said anomaly_count 13 but the ids.json counts block says anomalies 18 and the list has 18

pool = []
def add(source, anchor, note, subject=None, kind=None, confidence=None, extra=None):
    pool.append({"source": source, "anchor": anchor, "subject": subject, "kind": kind, "note": note, "confidence": confidence, "extra": extra})

# ids.json -- verbatim
for a in ids["anomalies"]:
    subj = None
    m = re.search(r"\b(AC-\d{3}|SM-\d{3}|ADR-\d{4}|F\d{1,2}|E7|P3|P4|P5|P9b|D[4-6]|R10|R-recent-deploy-feed-absent|DEPTH-01|Q1 \(R2\))\b", a["note"])
    if m: subj = m.group(1)
    add("ids", a["anchor"], a["note"], subj, "id-namespace", a.get("confidence"))

# constants -- VARIANT seeds and zero-match candidates
for s in con["seeds"]:
    if s["status"] == "VARIANT":
        anchors = []
        for c in s["candidates"]:
            occ = c.get("occurrences") or c.get("anchors") or []
            first = occ[0] if occ else None
            if isinstance(first, dict):
                first = first.get("anchor") or ("%s:%s" % (first.get("file"), first.get("line")))
            anchors.append({"value": c.get("value"), "n_occ": c.get("n_occ"), "first_anchor": first})
        add("constants", anchors[0]["first_anchor"] if anchors and anchors[0]["first_anchor"] else "(see seed)",
            "seed %s (%s) has alternative values %s; note: %s" % (s["id"], s["label"], s["alternatives_present"], (s.get("note") or "")[:200]),
            s["id"], "constant-variant", None, {"candidates": anchors})
    zero = [c.get("value") for c in s["candidates"] if (c.get("n_occ") or 0) == 0]
    if zero:
        add("constants", "(no occurrence)", "seed %s candidate value(s) %s matched nothing" % (s["id"], zero), s["id"], "constant-candidate-zero-match")

# anchors -- dangling section refs, C-ref keyword mismatches, dangling relative links
for d in anc["section_refs"]["dangling"]:
    add("anchors", "%s:%s" % (d["file"], d["line"]), "section reference %s resolves to no heading (nearest alias target: %s)" % (d["raw"].replace("\u00a7", "S"), d.get("nearest_alias_target")),
        d["raw"].replace("\u00a7", "S"), "dangling-section-ref", d.get("confidence"))
for r in anc["c_refs"]["resolved"]:
    if r.get("keyword_mismatch_candidate"):
        add("anchors", "%s:%s" % (r["file"], r["line"]), "C-reference %s: cited item title keywords do not match the citing sentence (heuristic)" % r.get("raw", "").replace("\u00a7", "S"),
            r.get("raw", "").replace("\u00a7", "S"), "c-ref-keyword-mismatch", "possible")
seen_targets = collections.defaultdict(list)
for d in anc["file_paths"]["relative_md_link_targets"]["dangling"]:
    seen_targets[d["target"]].append("%s:%s" % (d["file"], d["line"]))
for tgt, ancs in seen_targets.items():
    add("anchors", ancs[0], "relative markdown link target %s is missing (%d citing line(s))" % (tgt, len(ancs)), tgt, "dangling-relative-link", None, {"all_anchors": ancs})

# adr-graph -- README disagreements
for d in adr["readme_vs_frontmatter_diff"]:
    if not d["agrees"]:
        add("adr-graph", "references/adr/README.md:%s" % d["readme_line"], "README claims %s = %s but frontmatter measures %s" % (d["claim"], d["readme_claims"], d["measured"]), d["claim"], "readme-disagreement", "certain")
cdc = adr["citation_date_check"]
add("adr-graph", "(11 doc/ADR pairs)", "%d raw citation-before-ADR-date pairs, all explained by later in-place amendment (unexplained: %d)" % (cdc["violations_count"], len(cdc.get("violations_unexplained", []) if isinstance(cdc.get("violations_unexplained"), list) else [])), None, "citation-date", "explained")

# slices -- duplicate / out-of-range anchors, areas with no AC section
for k, v in sl["ac_anchor_duplicates"].items():
    add("slices", "references/2026-09-08-acceptance-criteria.md:%s" % v[0], "%s is defined twice (lines %s)" % (k, v), k, "duplicate-definition", "certain")
for k in sl["ac_anchors_outside_declared_ranges"]:
    add("slices", "references/2026-09-08-acceptance-criteria.md", "%s sits outside every declared area range in section 0.1" % k, k, "anchor-outside-declared-range", "certain")
for s in sl["slices"]:
    if not s["ac_sections"]:
        add("slices", "references/2026-09-08-architecture-design.md:%s" % s["row_line"], "area %s (%s) has no AC register section joined to it" % (s["slice_id"], s["area_name"]), s["slice_id"], "area-without-ac-section", "mapping")

# external -- zero-match curated patterns
for a in ext["assertions"]:
    if not a["ok"]:
        add("external", "(curated pattern)", "curated claim pattern %s matched 0 lines in the corpus" % a["key"], a["key"], "curated-pattern-zero-match", "certain")

# known-issues -- stale-source rows (spikes/HANDOFF.md is the superseded handoff; 26 rows)
srcs = collections.Counter(r["source_anchor"].split(":")[0] for r in ki["rows"] if r["source_anchor"].startswith("spikes/HANDOFF.md"))
for src, n in srcs.items():
    add("known-issues", src, "%d rows harvested from a source the corpus marks stale (superseded handoff; counts.stale_source_rows = %d)" % (n, ki["counts"]["stale_source_rows"]), src, "stale-source-rows", "documented")

# phase1-deps -- ordering hazards, stale runbook claims, absent tools
for h in pd["ordering_hazards"]["hazards"]:
    add("phase1-deps", h.get("runbook_position") or h.get("disclosure_anchor"), "%s: %s" % (h["title"], h["conflict"]), h["id"], "ordering-hazard", "disclosed" if h.get("disclosed_in_document") else "undisclosed")
for c in pd["runbook_claim_verification"]:
    if not c["verdict"].startswith("CONFIRMED"):
        add("phase1-deps", c["claim"].split(" -- ")[0], "%s -> %s" % (c["claim"][:160], c["verdict"][:160]), None, "runbook-claim-stale", "measured")
for t, v in pd["tools"].items():
    if not v.get("installed", True):
        add("phase1-deps", "RUNBOOK.md:56", "tool %s absent on this box" % t, t, "tool-absent", "measured")

# doc-map -- stale-risk files cited as current, denial claim, arch id not defined
for s in dm["stale_risk"]:
    if s["cited_as_current_by_newer"]:
        add("doc-map", s["path"], "declared %s, cited as current by newer docs %s" % (s["declared_date"], s["cited_as_current_by_newer"]), s["path"], "stale-risk-cited-as-current", "heuristic")
for d in dm["documents"]:
    if d.get("denial_claim"):
        add("doc-map", d["path"], "document carries a denial-of-authority claim: %s" % str(d["denial_claim"])[:160], d["path"], "authority-denial", "heuristic")
dvm = dm["authority_map"]["central_precedence_pair"]["defined_vs_mentioned"]
for x in dvm["arch_ac_not_in_defined"]:
    add("doc-map", "references/2026-09-08-architecture-design.md", "%s is cited by architecture-design but not defined in the AC register" % x, x, "cited-not-defined", "certain")
for x in dvm["arch_sm_not_in_defined"]:
    add("doc-map", "references/2026-09-08-architecture-design.md", "%s is cited by architecture-design but not defined in the SM register" % x, x, "cited-not-defined", "certain")

# ---- dedupe: same subject (id) and same kind-family collapse; keep every source and anchor
def key(a):
    subj = (a["subject"] or "").strip()
    fam = {"cited-not-defined": "dangling", "id-namespace": "dangling" if "no definition" in a["note"] or "never defined" in a["note"] or "no such ADR" in a["note"] else "id-namespace",
           "duplicate-definition": "dup"}.get(a["kind"], a["kind"])
    if subj:
        return (subj, fam)
    return (hashlib.sha1((a["anchor"] + a["note"]).encode("utf-8")).hexdigest()[:12], fam)
merged = collections.OrderedDict()
for a in pool:
    k = key(a)
    if k in merged:
        m = merged[k]
        m["sources"] = sorted(set(m["sources"] + [a["source"]]))
        if a["anchor"] not in m["anchors"]:
            m["anchors"].append(a["anchor"])
        m["notes"].append("[%s] %s" % (a["source"], a["note"]))
    else:
        merged[k] = {"subject": a["subject"], "family": k[1], "sources": [a["source"]], "anchors": [a["anchor"]], "notes": ["[%s] %s" % (a["source"], a["note"])], "confidence": a["confidence"], "extra": a["extra"]}
anomaly_pool = list(merged.values())
# ids dup: D4/D5/D6 are three separate subjects; AC-428 in ids duplicate_definition too
for k, v in ids["namespaces"]["AC"]["duplicate_definition"].items():
    for m in anomaly_pool:
        if m["subject"] == k and m["family"] == "dup" and "ids" not in m["sources"]:
            m["sources"].append("ids"); m["anchors"].extend(v); m["notes"].append("[ids] duplicate_definition %s" % v)

pool_by_source = collections.Counter(s for m in anomaly_pool for s in m["sources"])
raw_by_source = collections.Counter(a["source"] for a in pool)

# ---- inconsistencies between artifacts (from step 3 fails + my own observations)
inconsistencies = []
for c in step3["checks"]:
    if not c["ok"]:
        inconsistencies.append({"between": c["id"] + ": " + c["title"], "detail": json.dumps(c["detail"], ensure_ascii=True)[:600], "severity": c["severity_if_fail"]})
inconsistencies += [
    {"between": "orchestrator summary vs ids.json", "detail": "the ids row reports anomaly_count 13, but ids.json counts.anomalies is 18 and the anomalies list has 18 entries", "severity": "minor"},
    {"between": "orchestrator summary vs doc-map.json counts", "detail": "summary labels (authority_documents_self_claim, adr_records, ac_register_ids_mentioned ...) are not the artifact's counts keys (authority_documents, orphans_by_path_but_referenced_by_id ...); adr_records is absent from doc-map counts (the 89 lives in documents[].id_counts.ADR). Partition off the artifact, not the summary.", "severity": "minor"},
    {"between": "orchestrator summary vs external.json counts", "detail": "summary uses family_x / method_x / tag_x and distinct_arxiv_ids_outside_fixtures; the artifact uses family:x / method:x / tag:x and distinct_arxiv_ids", "severity": "minor"},
    {"between": "eight artifacts vs their reported anomaly_count", "detail": "only ids.json stores a structured anomalies list; constants, anchors, adr-graph, slices, external, known-issues, phase1-deps and doc-map hold no anomaly list, so the anomaly texts the agents counted (69 in total) exist only in their prose returns. The pool below is rebuilt from structured anomaly-equivalent records instead.", "severity": "material"},
    {"between": "constants.json meta.content_sha256_excluding_generated vs its re-run", "detail": "the hash covers meta.script (the absolute script path), so running the identical script from another directory changes the hash while every data field is byte-identical; the hash is not a pure content hash", "severity": "minor"},
    {"between": "anchors.json heading_index vs CRLF files", "detail": "%d of %d indexed heading texts retain a trailing CR (the repo files are CRLF); consumers must strip it before text-matching" % (sum(1 for d, hs in anc["heading_index"].items() for h in hs if h[0].endswith("\r")), sum(len(v) for v in anc["heading_index"].values())), "severity": "minor"},
    {"between": "phase1-deps.json step 3b end_line vs anchors heading index", "detail": "3b is given 676-864, folding the interstitial level-3 heading 'The UTC ...Z timestamp trap' (RUNBOOK.md:774) into 3b; the strict next-same-level-heading rule gives 676-773. Interpretive, not wrong; slices.json does not index sub-steps so it cannot be checked against it.", "severity": "minor"},
    {"between": "external.json arXiv count vs the seed expectation", "detail": "seed expected ~48 distinct arXiv ids; external.json holds 99 and an independent grep of the 194 md files also gives 99 (URL forms abs/pdf/html; the two 'arXiv NNNN.NNNNN' prose forms are among them). The seed was wrong or scoped narrower (references/research/*.md and 2026-09-09-change-classification.md carry 29+27+20 of them).", "severity": "minor"},
]

verdict_per_artifact = {}
for k, r in step2.items():
    if k.startswith("_"): continue
    verdict_per_artifact[k] = {
        "exists": step1[k]["exists"], "bytes": step1[k]["bytes"], "parses": step1[k]["parses"], "leaf_count": step1[k]["leaf_count"],
        "rerun_status": r["status"], "rerun_seconds": r["seconds"], "byte_identical": r.get("byte_identical_json"),
        "identical_after_stripping_volatile_meta": r.get("identical_after_stripping_volatile"),
        "usable_for_partition": r["status"] == "REPRODUCIBLE",
    }
verdict_per_artifact["constants"]["note"] = "REPRODUCIBLE in every data field; meta.script and the path-sensitive content hash moved with the script location (see inconsistencies)"
verdict_per_artifact["doc-map"]["note"] = "slowest re-run (153 s, scans 8,416 files for references)"

git_after = subprocess.run(["git", "-C", REPO, "status", "--porcelain"], capture_output=True, text=True).stdout.splitlines()
out = {
    "artifact": "reconcile.json",
    "phase": "audit ground truth -- census reconciliation",
    "generated_by": "scripts/reconcile_step1_inventory.py, reconcile_step2_rerun.py, reconcile_probe_arxiv.py, reconcile_step3_crosscheck.py, reconcile_step4_report.py",
    "repo": REPO, "repo_read_only": True,
    "repo_git_porcelain_lines_before_and_after_reruns": {"before": 17, "after": len(git_after)},
    "step1_inventory": {k: v for k, v in step1.items()},
    "step2_rerun": {k: {kk: vv for kk, vv in v.items() if kk not in ("stdout_tail", "stderr_tail")} for k, v in step2.items() if not k.startswith("_")},
    "reproducible_count": sum(1 for k, v in step2.items() if not k.startswith("_") and v["status"] == "REPRODUCIBLE"),
    "step3_cross_checks": step3,
    "arxiv_probe": probe,
    "verdict_per_artifact": verdict_per_artifact,
    "inconsistencies": inconsistencies,
    "anomaly_pool_method": __doc__.strip(),
    "anomaly_pool_counts": {"raw_harvested": len(pool), "after_dedupe": len(anomaly_pool), "raw_by_source": dict(raw_by_source), "pool_membership_by_source": dict(pool_by_source), "agent_reported_anomaly_count": REPORTED_ANOMALY_COUNTS, "agent_reported_total": sum(REPORTED_ANOMALY_COUNTS.values())},
    "anomaly_pool": anomaly_pool,
    "gaps_carried_forward": {
        "agent_gaps_are_in_the_orchestrator_message_not_in_artifacts": True,
        "reconcile_gaps": [
            "The anomaly pool is a harvest of structured anomaly-equivalent records, not the agents' own anomaly lists (which live only in their prose returns).",
            "Line-range agreement was checked at heading granularity (start line + heading text + end line derived from the anchors heading index); byte offsets were not compared.",
            "Spike-id membership for phase1-deps was computed by regex over the steps JSON text; a spike named in prose by another alias would not be caught.",
            "The re-run harness rewrote each script's output-path constant textually (substitutions recorded per script); no other line was changed.",
            "Cross-artifact set checks used the sets the artifacts carry; where an artifact carries only a count (doc-map id_counts, slices sm_rows_indexed) only the count was compared.",
        ],
    },
}
with open(os.path.join(CENSUS, "reconcile.json"), "w", encoding="utf-8") as f:
    json.dump(out, f, indent=1, ensure_ascii=True)
print("reproducible", out["reproducible_count"], "checks", step3["summary"], "pool raw", len(pool), "dedup", len(anomaly_pool))
print("raw_by_source", dict(raw_by_source))
print("pool_by_source", dict(pool_by_source))
print("bytes", os.path.getsize(os.path.join(CENSUS, "reconcile.json")))
