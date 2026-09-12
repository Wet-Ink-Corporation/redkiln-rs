"""Quick ASCII-only inspection of doc-map.json sections (read-only helper)."""
import json, sys
P = ("C:/Users/ryanm/AppData/Local/Temp/claude/D--repos-redkiln-rs/"
     "953a92ec-7709-4a24-b01d-8a5e2c5b86c8/scratchpad/audit/census/doc-map.json")
d = json.load(open(P, encoding="utf-8"))
def say(s): sys.stdout.write(s.encode("ascii", "replace").decode("ascii") + "\n")
what = sys.argv[1]
if what == "auth":
    for a in d["authority_map"]["documents"]:
        say("== %s | date=%s | status=%s" % (a["path"], a["declared_date"], (a["declared_status"] or "")[:90]))
        for c in a["verbatim_claims"][:3]:
            say("   L%d: %s" % (c["line"], c["claim"][:220]))
        say("   scope: %s" % a["declared_scope"])
elif what == "status":
    for x in d["documents"]:
        say("%-70s %-10s %s" % (x["path"], x["declared_date"], (x["declared_status"] or "-")[:70]))
elif what == "big":
    for x in d["documents"]:
        if x["read_plan"]:
            say("== %s tokens=%s lines=%d maxlen=%d windows=%d ok=%s" % (x["path"], x["est_tokens"], x["lines"], x["max_line_length"], x["read_plan"]["window_count"], x["read_plan"]["coverage_verified"]))
            for w in x["read_plan"]["windows"]:
                say("   %5d-%5d %7s  %s" % (w["start_line"], w["end_line"], w["est_tokens"], w["heading"][:80]))
elif what == "orphans":
    for o in d["orphans"]:
        say("%s strict=%s basename_refs=%d" % (o["path"], o["strict_orphan"], len(o["referenced_by_basename_only"])))
elif what == "stale":
    for s in d["stale_risk"]:
        say("== %s date=%s current-citers=%d newer-citers=%d" % (s["path"], s["declared_date"], len(s["cited_as_current_by_newer"]), s["cited_by_newer_count"]))
        for c in s["cited_by_newer"]:
            if c["any_currency_assertion"]:
                say("   <- %s (%s)" % (c["citer"], c["citer_date"]))
elif what == "overlap":
    for r in d["authority_map"]["pairwise_id_overlap"]:
        say("%s  <->  %s" % (r["a"], r["b"]))
        for fam, f in r["families"].items():
            say("    %s a=%d b=%d shared=%d jac=%s" % (fam, f["a_count"], f["b_count"], f["shared"], f["jaccard"]))
elif what == "central":
    c = d["authority_map"]["central_precedence_pair"]
    for k in ("ac_register_ids","arch_ac_ids","ac_shared","ac_in_arch_not_in_register","ac_in_register_not_in_arch","sm_register_ids","arch_sm_ids","sm_shared","sm_in_arch_not_in_register","sm_in_register_not_in_arch_count","arch_authority_claim","ac_register_authority_claim"):
        say("%s: %s" % (k, c[k]))
elif what == "noauth":
    for x in d["documents"]:
        if x["authority_claim"] is None and x["authority_mentions"]:
            say("%-70s mentions=%d self=%d" % (x["path"], x["authority_mentions"], x["authority_self_claim_sentences"]))
