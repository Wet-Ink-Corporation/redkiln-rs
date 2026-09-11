# -*- coding: utf-8 -*-
"""
census_adr.py -- ADR graph census for the redkiln-rs pre-implementation audit.

READ-ONLY over D:/repos/redkiln-rs. Writes exactly one artifact:
    <OUT>/census/adr-graph.json

Every number in the artifact is computed here. Nothing is eyeballed.
ASCII-only output on stdout (Windows console cannot print non-ASCII).
"""

import io
import json
import os
import re
import sys
from collections import Counter, OrderedDict

REPO = r"D:/repos/redkiln-rs"
ADR_DIR = os.path.join(REPO, "references", "adr")
OUT_JSON = r"D:/repos/redkiln-rs/references/audit/2026-09-10/census/adr-graph.json"

FM_KEYS_EXPECTED = ["id", "title", "status", "date", "legacy_id", "source",
                    "supersedes", "superseded_by", "related", "evidence",
                    "consumers"]

ANCHOR_FAILURES = []


def anchor(name, ok, detail):
    """Assert a pattern matched what we believe. Record, never silently pass."""
    rec = {"assertion": name, "ok": bool(ok), "detail": detail}
    if not ok:
        ANCHOR_FAILURES.append(rec)
    return rec


ANCHORS = []


def read_text(path):
    with io.open(path, "r", encoding="utf-8", errors="replace") as fh:
        return fh.read()


# ---------------------------------------------------------------- frontmatter

SCALAR_LIST_RE = re.compile(r"^\[(.*)\]$")
QUOTED_RE = re.compile(r'^"(.*)"$|^\'(.*)\'$')


def parse_scalar(raw):
    raw = raw.strip()
    m = QUOTED_RE.match(raw)
    if m:
        return m.group(1) if m.group(1) is not None else m.group(2)
    return raw


def parse_value(raw):
    raw = raw.strip()
    m = SCALAR_LIST_RE.match(raw)
    if m:
        inner = m.group(1).strip()
        if inner == "":
            return []
        # split on commas not inside quotes
        parts, buf, q = [], [], None
        for ch in inner:
            if q:
                if ch == q:
                    q = None
                buf.append(ch)
            elif ch in "\"'":
                q = ch
                buf.append(ch)
            elif ch == ",":
                parts.append("".join(buf))
                buf = []
            else:
                buf.append(ch)
        parts.append("".join(buf))
        return [parse_scalar(p) for p in parts if p.strip() != ""]
    return parse_scalar(raw)


FM_LINE_RE = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*):\s*(.*)$")


def parse_frontmatter(text):
    """Return (ordered dict of key->value, key order list, body string, error)."""
    lines = text.split("\n")
    if not lines or lines[0].strip() != "---":
        return None, [], text, "no opening --- fence"
    end = None
    for i in range(1, len(lines)):
        if lines[i].strip() == "---":
            end = i
            break
    if end is None:
        return None, [], text, "no closing --- fence"
    fm = OrderedDict()
    order = []
    cur = None
    for ln in lines[1:end]:
        if ln.strip() == "":
            continue
        m = FM_LINE_RE.match(ln)
        if m and not ln.startswith(" "):
            cur = m.group(1)
            order.append(cur)
            fm[cur] = parse_value(m.group(2))
        elif ln.lstrip().startswith("- ") and cur is not None:
            # block list continuation
            if not isinstance(fm[cur], list):
                fm[cur] = []
            fm[cur].append(parse_scalar(ln.lstrip()[2:]))
        else:
            # folded scalar continuation
            if cur is not None and isinstance(fm[cur], str):
                fm[cur] = (fm[cur] + " " + ln.strip()).strip()
    body = "\n".join(lines[end + 1:])
    return fm, order, body, None


# ------------------------------------------------------------------- forbids

# Three punctuations recorded at adr/README.md:195.
FORBIDS_RE = re.compile(
    r"^\*\*What this forbids\*\*:?\s*(.*)$|^\*\*What this forbids:\*\*\s*(.*)$")
# Stop markers: next ## heading, or a new bold lead-in paragraph.
BOLD_LEAD_RE = re.compile(r"^\*\*[^*]+\*\*[:.]?\s*$")


def extract_forbids(body):
    """Return (found, variant, start_line_1based_in_body, text)."""
    lines = body.split("\n")
    for i, ln in enumerate(lines):
        s = ln.rstrip()
        m = FORBIDS_RE.match(s)
        if not m:
            continue
        if s.startswith("**What this forbids:**"):
            variant = "**What this forbids:**"
            inline = m.group(2) or ""
        elif s.startswith("**What this forbids**:"):
            variant = "**What this forbids**:"
            inline = m.group(1) or ""
        else:
            variant = "**What this forbids**"
            inline = m.group(1) or ""
        chunk = []
        if inline.strip():
            chunk.append(inline.strip())
        j = i + 1
        while j < len(lines):
            t = lines[j].rstrip()
            if t.startswith("## "):
                break
            if BOLD_LEAD_RE.match(t) and t.strip() != "":
                break
            chunk.append(t)
            j += 1
        txt = "\n".join(chunk).strip()
        txt = re.sub(r"\n{3,}", "\n\n", txt)
        return True, variant, i + 1, txt
    return False, None, None, ""


# ------------------------------------------------------------------- sections

SECTION_RE = re.compile(r"^##\s+(.+?)\s*$")
MANDATORY = ["Context and problem", "Decision drivers", "Considered options",
             "Decision", "Consequences", "Evidence and confidence"]


def sections_of(body):
    return [m.group(1) for m in
            (SECTION_RE.match(l) for l in body.split("\n")) if m]


# ------------------------------------------------------------------ main scan

def scan_adrs():
    fnames = sorted(f for f in os.listdir(ADR_DIR)
                    if re.match(r"^\d{4}-.*\.md$", f))
    records = []
    for fn in fnames:
        path = os.path.join(ADR_DIR, fn)
        text = read_text(path)
        fm, order, body, err = parse_frontmatter(text)
        file_num = fn[:4]
        found, variant, fl, ftxt = extract_forbids(body)
        secs = sections_of(body)
        rec = {
            "id": (fm or {}).get("id"),
            "filename": fn,
            "file_number": file_num,
            "title": (fm or {}).get("title"),
            "status": (fm or {}).get("status"),
            "date": (fm or {}).get("date"),
            "legacy_id": (fm or {}).get("legacy_id"),
            "source": (fm or {}).get("source"),
            "supersedes": (fm or {}).get("supersedes", []),
            "superseded_by": (fm or {}).get("superseded_by", []),
            "related": (fm or {}).get("related", []),
            "evidence": (fm or {}).get("evidence", []),
            "consumers": (fm or {}).get("consumers", []),
            "frontmatter_keys": order,
            "frontmatter_parse_error": err,
            "sections": secs,
            "forbids_present": found,
            "forbids_variant": variant,
            "forbids_body_line": fl,
            "forbids_text_truncated": ftxt[:600],
            "forbids_text_len": len(ftxt),
            "forbids_truncated": len(ftxt) > 600,
            "body_lines": len(body.split("\n")),
            "_body": body,
        }
        records.append(rec)
    return records


# ------------------------------------------------------------------- README

def parse_readme():
    path = os.path.join(ADR_DIR, "README.md")
    text = read_text(path)
    lines = text.split("\n")
    claims = {"path": "references/adr/README.md", "line_count": len(lines)}

    def find(pat, label, flags=0):
        rx = re.compile(pat, flags)
        hits = []
        for i, ln in enumerate(lines):
            m = rx.search(ln)
            if m:
                hits.append({"line": i + 1, "match": m.group(0)[:300],
                             "groups": [g for g in m.groups()]})
        ANCHORS.append(anchor("readme:" + label, len(hits) > 0,
                              "%d line(s)" % len(hits)))
        return hits

    claims["claim_total_records"] = find(
        r"\*\*(\d+) records\*\*", "total_records")
    claims["claim_status_counts"] = find(
        r"holds (\d+) `accepted`, (\d+) `superseded` \(([^)]*)\) and (\d+) "
        r"`proposed` \((\d+)", "status_counts")
    claims["claim_supersession_counts"] = find(
        r"(Nine|\d+) supersession links exist in this set, from (eight|\d+) "
        r"superseding records", "supersession_counts")
    claims["claim_full_partial_split"] = find(
        r"(\w+) links are full; (\w+) are partial", "full_partial_split")
    claims["claim_forbids_punctuation"] = find(
        r"punctuated three ways across the set", "forbids_punctuation")
    claims["claim_related_edges"] = find(
        r"\((\d+) of the (\d+) `related` edges point one way only\)",
        "related_edges")
    claims["claim_consumers_empty"] = find(
        r"`consumers` is `\[\]` everywhere", "consumers_empty")
    claims["claim_new_record_dates"] = find(
        r"`0074`.`0088` were decided on (\d{4}-\d{2}-\d{2}) and `0089` on "
        r"(\d{4}-\d{2}-\d{2})", "new_record_dates")
    claims["claim_next_free"] = find(
        r"the next free number is `(\d{4})`", "next_free")
    claims["claim_forbids_on_all"] = find(
        r"It is present on all (\d+)\.", "forbids_on_all")
    claims["claim_forbids_counts"] = find(
        r"`\*\*What this forbids\*\*` \((\d+)\), `\*\*What this forbids\*\*:` "
        r"\((\d+)\), `\*\*What this forbids:\*\*` \((\d+)\)", "forbids_counts")

    # supersession table
    sup_rows = []
    sup_row_re = re.compile(
        r"^\|\s*\[(\d{4})\]\([^)]*\)[^|]*\|\s*\[(\d{4})\]\([^)]*\)[^|]*\|\s*"
        r"(\*\*Full\*\*|Full|Partial|\*\*Partial\*\*)\s*\|\s*(.*?)\s*\|\s*$")
    for i, ln in enumerate(lines):
        m = sup_row_re.match(ln)
        if m:
            sup_rows.append({
                "line": i + 1,
                "superseded": m.group(1),
                "by": m.group(2),
                "kind": m.group(3).replace("*", "").lower(),
                "what_changed_truncated": m.group(4)[:600],
                "what_changed_len": len(m.group(4)),
            })
    ANCHORS.append(anchor("readme:supersession_table_rows", len(sup_rows) > 0,
                          "%d rows" % len(sup_rows)))
    claims["supersession_table"] = sup_rows

    # legacy-id map table
    legacy_rows = []
    legacy_re = re.compile(
        r"^\|\s*([A-Za-z0-9\-. ]+?)\s*\|\s*\[(\d{4})\]\(([^)]*)\)\s*\|\s*"
        r"(.*?)\s*\|\s*(.*?)\s*\|\s*$")
    for i, ln in enumerate(lines):
        m = legacy_re.match(ln)
        if not m:
            continue
        if m.group(1) in ("Legacy id",):
            continue
        legacy_rows.append({
            "line": i + 1,
            "legacy_id": m.group(1),
            "adr": m.group(2),
            "link": m.group(3),
            "title": m.group(4),
            "status_cell": m.group(5),
        })
    ANCHORS.append(anchor("readme:legacy_map_rows", len(legacy_rows) > 0,
                          "%d rows" % len(legacy_rows)))
    claims["legacy_map"] = legacy_rows

    # the three deliberate non-supersessions
    non_sup = []
    start = None
    for i, ln in enumerate(lines):
        if "deliberately **not** supersessions" in ln:
            start = i
            break
    ANCHORS.append(anchor("readme:non_supersessions_header", start is not None,
                          "line %s" % (start + 1 if start is not None else "-")))
    if start is not None:
        for j in range(start + 1, min(start + 12, len(lines))):
            t = lines[j].rstrip()
            if t.startswith("## "):
                break
            if t.startswith("- "):
                ids = []
                for x in re.findall(r"`(\d{4})`", t):
                    if x not in ids:
                        ids.append(x)
                non_sup.append({"line": j + 1, "ids": ids,
                                "text_truncated": t[:600],
                                "text_len": len(t)})
    claims["deliberate_non_supersessions"] = non_sup
    return claims, text


# --------------------------------------------------- citation / date checking

DATE_IN_NAME_RE = re.compile(r"(\d{4}-\d{2}-\d{2})")


def collect_citing_docs():
    docs = []
    for root, dirs, files in os.walk(REPO):
        dirs[:] = [d for d in dirs if d not in (".git", "target", "node_modules")]
        norm = root.replace("\\", "/")
        if "/references/adr" in norm:
            continue
        for fn in files:
            if not fn.lower().endswith(".md"):
                continue
            m = DATE_IN_NAME_RE.search(fn)
            if not m:
                continue
            docs.append((os.path.join(root, fn), m.group(1)))
    return docs


CITE_PATTERNS = [
    ("adr_path", re.compile(r"adr/(\d{4})-")),
    ("adr_word", re.compile(r"ADR[\s\-\u2011]?(\d{4})\b")),
    ("md_link", re.compile(r"\[(\d{4})\]\((\d{4})-[^)]*\)")),
    ("backtick_id", re.compile(r"`(\d{4})`")),
]


def citation_check(records):
    valid = set(r["id"] for r in records if r["id"])
    by_id = dict((r["id"], r) for r in records if r["id"])
    docs = collect_citing_docs()
    ANCHORS.append(anchor("citation:dated_docs_found", len(docs) > 0,
                          "%d dated .md documents outside references/adr"
                          % len(docs)))
    pattern_hits = Counter()
    violations = []
    per_doc = []
    for path, ddate in docs:
        text = read_text(path)
        found = {}
        for name, rx in CITE_PATTERNS:
            for m in rx.finditer(text):
                aid = m.group(1)
                if aid not in valid:
                    continue
                pattern_hits[name] += 1
                found.setdefault(aid, set()).add(name)
        rel = os.path.relpath(path, REPO).replace("\\", "/")
        per_doc.append({"doc": rel, "doc_date": ddate,
                        "adr_ids_cited": sorted(found.keys())})
        for aid, names in sorted(found.items()):
            adate = by_id[aid].get("date")
            if isinstance(adate, str) and re.match(r"^\d{4}-\d{2}-\d{2}$", adate):
                if ddate < adate:
                    # locate a sample line that cites THIS id
                    sample = ""
                    sample_line_no = None
                    for li, ln in enumerate(text.split("\n")):
                        hit = False
                        for _, rx in CITE_PATTERNS:
                            for m in rx.finditer(ln):
                                if m.group(1) == aid:
                                    hit = True
                                    break
                            if hit:
                                break
                        if hit:
                            sample = ln.strip()[:300]
                            sample_line_no = li + 1
                            break
                    all_dates = re.findall(r"\b(2026-\d{2}-\d{2})\b", text)
                    doc_max = max(all_dates) if all_dates else None
                    violations.append({
                        "doc": rel, "doc_date": ddate, "adr": aid,
                        "adr_date": adate,
                        "patterns": sorted(names),
                        "sample_line": sample_line_no,
                        "sample_line_truncated": sample,
                        "doc_max_date_mentioned": doc_max,
                        "doc_amended_on_or_after_adr_date":
                            bool(doc_max and doc_max >= adate),
                    })
    ANCHORS.append(anchor("citation:pattern_hits_nonzero",
                          sum(pattern_hits.values()) > 0,
                          dict(pattern_hits)))
    return {
        "method": ("dated .md documents (YYYY-MM-DD in filename) outside "
                   "references/adr, scanned for ADR-id citations via four "
                   "patterns; flagged where document filename date < ADR "
                   "frontmatter date"),
        "caveat": ("[inferred] a filename date is the document's nominal date, "
                   "not necessarily the date every line in it was written; "
                   "backtick_id is the loosest pattern and can match a bare "
                   "4-digit number that is not an ADR reference"),
        "docs_scanned": len(docs),
        "pattern_hit_counts": dict(sorted(pattern_hits.items())),
        "violations": violations,
        "violations_count": len(violations),
        "violations_where_doc_carries_a_later_date_than_the_adr":
            sum(1 for v in violations
                if v["doc_amended_on_or_after_adr_date"]),
        "violations_unexplained": [
            v for v in violations
            if not v["doc_amended_on_or_after_adr_date"]],
        "per_doc_citations": per_doc,
    }


# --------------------------------------------------------------- computations

def main():
    records = scan_adrs()
    ANCHORS.append(anchor("adr:file_count", len(records) == 89,
                          "%d NNNN-*.md files" % len(records)))

    by_id = {}
    for r in records:
        if r["id"]:
            by_id.setdefault(r["id"], []).append(r)

    # --- frontmatter conformance
    key_union = set()
    for r in records:
        key_union |= set(r["frontmatter_keys"])
    conformance = {
        "expected_keys_readme": FM_KEYS_EXPECTED,
        "observed_key_union": sorted(key_union),
        "records_with_parse_error": [r["filename"] for r in records
                                     if r["frontmatter_parse_error"]],
        "missing_keys": [],
        "extra_keys": [],
        "key_order_deviations": [],
        "id_filename_mismatches": [],
        "key_presence_counts": {},
    }
    presence = Counter()
    for r in records:
        ks = r["frontmatter_keys"]
        for k in sorted(set(ks)):
            presence[k] += 1
        missing = [k for k in FM_KEYS_EXPECTED if k not in ks]
        extra = [k for k in ks if k not in FM_KEYS_EXPECTED]
        if missing:
            conformance["missing_keys"].append(
                {"filename": r["filename"], "missing": missing})
        if extra:
            conformance["extra_keys"].append(
                {"filename": r["filename"], "extra": extra})
        if [k for k in ks if k in FM_KEYS_EXPECTED] != FM_KEYS_EXPECTED:
            conformance["key_order_deviations"].append(
                {"filename": r["filename"], "order": ks})
        if r["id"] != r["file_number"]:
            conformance["id_filename_mismatches"].append(
                {"filename": r["filename"], "id": r["id"]})
    conformance["key_presence_counts"] = dict(sorted(presence.items()))
    conformance["keys_on_every_record"] = sorted(
        [k for k, v in presence.items() if v == len(records)])
    conformance["keys_missing_from_some"] = sorted(
        [{"key": k, "present_on": v} for k, v in presence.items()
         if v != len(records)], key=lambda d: d["key"])

    # --- sections conformance
    sec_issues = []
    for r in records:
        miss = [s for s in MANDATORY if s not in r["sections"]]
        if miss:
            sec_issues.append({"filename": r["filename"], "missing": miss,
                               "sections": r["sections"]})

    # --- duplicate / gap check
    nums = sorted(int(r["file_number"]) for r in records)
    gaps = [n for n in range(1, (nums[-1] if nums else 0) + 1)
            if n not in set(nums)]
    dupes = [k for k, v in by_id.items() if len(v) > 1]

    # --- status distribution
    status_dist = Counter(r["status"] for r in records)

    # --- legacy id
    legacy_counts = Counter(r["legacy_id"] for r in records)
    legacy_dupes = {k: v for k, v in legacy_counts.items()
                    if v > 1 and k != "none"}

    # --- supersession edges + reciprocity
    edges = []
    seen_pairs = set()
    for r in records:
        for tgt in r["supersedes"]:
            key = (r["id"], tgt)
            seen_pairs.add(key)
            tr = by_id.get(tgt, [None])[0]
            recip = bool(tr and r["id"] in tr["superseded_by"])
            edges.append({
                "superseder": r["id"], "superseded": tgt,
                "declared_on": "supersedes",
                "reciprocal": recip,
                "target_exists": tr is not None,
                "target_status": tr["status"] if tr else None,
                "superseder_status": r["status"],
            })
    for r in records:
        for src in r["superseded_by"]:
            key = (src, r["id"])
            if key in seen_pairs:
                continue
            sr = by_id.get(src, [None])[0]
            edges.append({
                "superseder": src, "superseded": r["id"],
                "declared_on": "superseded_by",
                "reciprocal": False,
                "target_exists": sr is not None,
                "target_status": r["status"],
                "superseder_status": sr["status"] if sr else None,
            })
    edges.sort(key=lambda e: (e["superseded"], e["superseder"]))

    superseding_records = sorted(set(e["superseder"] for e in edges))
    superseded_records = sorted(set(e["superseded"] for e in edges))

    # --- related edges + reciprocity
    rel_edges = []
    rel_pairs = set()
    for r in records:
        for t in r["related"]:
            rel_edges.append({"from": r["id"], "to": t,
                              "target_exists": t in by_id})
            rel_pairs.add((r["id"], t))
    rel_one_way = [e for e in rel_edges
                   if (e["to"], e["from"]) not in rel_pairs]
    rel_reciprocal = [e for e in rel_edges
                      if (e["to"], e["from"]) in rel_pairs]

    # --- dangling references
    dangling = []
    for r in records:
        for k in ("supersedes", "superseded_by", "related"):
            for t in r[k]:
                if t not in by_id:
                    dangling.append({"filename": r["filename"], "key": k,
                                     "unresolved_id": t})

    # --- acyclicity of supersession graph
    adj = {}
    for e in edges:
        adj.setdefault(e["superseded"], set()).add(e["superseder"])
    colour = {}
    cycles = []

    def dfs(n, stack):
        colour[n] = 1
        for m in sorted(adj.get(n, ())):
            if colour.get(m) == 1:
                cycles.append(stack + [n, m])
            elif colour.get(m) is None:
                dfs(m, stack + [n])
        colour[n] = 2

    for n in sorted(set(list(adj.keys()) + superseding_records)):
        if colour.get(n) is None:
            dfs(n, [])

    # --- partial supersessions: accepted but superseded_by non-empty
    readme_claims, readme_text = parse_readme()
    readme_sup = dict(((row["superseded"], row["by"]), row)
                      for row in readme_claims["supersession_table"])

    partials = []
    for r in records:
        if r["superseded_by"] and r["status"] == "accepted":
            entry = {
                "id": r["id"], "filename": r["filename"],
                "legacy_id": r["legacy_id"], "status": r["status"],
                "superseded_by": r["superseded_by"],
                "scope_from_readme": [],
                "scope_from_successor_body": [],
                "scope_from_own_body": [],
            }
            for succ in r["superseded_by"]:
                row = readme_sup.get((r["id"], succ))
                if row:
                    entry["scope_from_readme"].append({
                        "successor": succ,
                        "readme_line": row["line"],
                        "kind": row["kind"],
                        "what_changed_truncated": row["what_changed_truncated"],
                        "what_changed_len": row["what_changed_len"],
                    })
                sr = by_id.get(succ, [None])[0]
                if sr:
                    leg = r["legacy_id"] or ""
                    leg_alt = re.sub(r"\s*rev\..*$", "", leg)
                    hits = []
                    for i, ln in enumerate(sr["_body"].split("\n")):
                        if not ln.strip():
                            continue
                        mentions = (r["id"] in ln or
                                    (leg_alt and re.search(
                                        r"\b" + re.escape(leg_alt) + r"\b", ln)))
                        if not mentions:
                            continue
                        if re.search(r"supersed|withdraw|withdrawn|stands|"
                                     r"survives|unchanged|keeps|stays",
                                     ln, re.I):
                            hits.append({"successor": succ,
                                         "body_line": i + 1,
                                         "text_truncated": ln.strip()[:600],
                                         "text_len": len(ln.strip())})
                    entry["scope_from_successor_body"].extend(hits[:8])
            for i, ln in enumerate(r["_body"].split("\n")):
                if not ln.strip():
                    continue
                if any(s in ln for s in r["superseded_by"]):
                    entry["scope_from_own_body"].append(
                        {"body_line": i + 1, "text_truncated": ln.strip()[:600],
                         "text_len": len(ln.strip())})
            entry["scope_from_own_body"] = entry["scope_from_own_body"][:6]
            partials.append(entry)

    # --- full supersessions (status superseded)
    fulls = [{"id": r["id"], "filename": r["filename"], "status": r["status"],
              "superseded_by": r["superseded_by"]}
             for r in records
             if r["status"] == "superseded"]

    # --- forbids
    forbids_variants = Counter(r["forbids_variant"] for r in records
                               if r["forbids_present"])
    no_forbids = [r["filename"] for r in records if not r["forbids_present"]]
    forbids_index = [{
        "id": r["id"], "filename": r["filename"],
        "variant": r["forbids_variant"],
        "body_line": r["forbids_body_line"],
        "text_len": r["forbids_text_len"],
        "truncated": r["forbids_truncated"],
        "text": r["forbids_text_truncated"],
    } for r in records]
    ANCHORS.append(anchor("adr:forbids_present_all",
                          len(no_forbids) == 0,
                          "%d record(s) with no forbids block" % len(no_forbids)))

    # --- dates
    date_dist = Counter(r["date"] for r in records)
    bad_dates = [r["filename"] for r in records
                 if not (isinstance(r["date"], str)
                         and re.match(r"^\d{4}-\d{2}-\d{2}$", r["date"]))]

    # --- evidence / consumers
    ev_counter = Counter()
    for r in records:
        for e in r["evidence"]:
            ev_counter[e] += 1
    consumers_nonempty = [{"id": r["id"], "consumers": r["consumers"]}
                          for r in records if r["consumers"]]
    ev_nonempty = sum(1 for r in records if r["evidence"])

    # --- README diff
    def first_groups(hits):
        return hits[0]["groups"] if hits else None

    diffs = []

    def diff(name, claimed, measured, line):
        agree = (claimed == measured)
        diffs.append({"claim": name, "readme_line": line,
                      "readme_claims": claimed, "measured": measured,
                      "agrees": agree})

    h = readme_claims["claim_total_records"]
    diff("total record count", int(h[0]["groups"][0]) if h else None,
         len(records), h[0]["line"] if h else None)

    h = readme_claims["claim_status_counts"]
    if h:
        g = h[0]["groups"]
        diff("accepted count", int(g[0]), status_dist.get("accepted", 0),
             h[0]["line"])
        diff("superseded count", int(g[1]), status_dist.get("superseded", 0),
             h[0]["line"])
        diff("superseded ids", sorted(re.findall(r"\d{4}", g[2])),
             sorted(r["id"] for r in records if r["status"] == "superseded"),
             h[0]["line"])
        diff("proposed count", int(g[3]), status_dist.get("proposed", 0),
             h[0]["line"])
        diff("proposed id", [g[4]],
             sorted(r["id"] for r in records if r["status"] == "proposed"),
             h[0]["line"])

    WORDNUM = {"one": 1, "two": 2, "three": 3, "four": 4, "five": 5, "six": 6,
               "seven": 7, "eight": 8, "nine": 9, "ten": 10, "eleven": 11}

    def w2n(s):
        s = s.strip().lower()
        return WORDNUM.get(s, int(s) if s.isdigit() else None)

    h = readme_claims["claim_supersession_counts"]
    if h:
        g = h[0]["groups"]
        diff("supersession link count", w2n(g[0]), len(edges), h[0]["line"])
        diff("superseding record count", w2n(g[1]), len(superseding_records),
             h[0]["line"])

    h = readme_claims["claim_full_partial_split"]
    if h:
        g = h[0]["groups"]
        diff("full supersession count", w2n(g[0]), len(fulls), h[0]["line"])
        measured_partial_links = sum(
            1 for e in edges
            if by_id.get(e["superseded"], [{}])[0].get("status") == "accepted")
        diff("partial supersession link count", w2n(g[1]),
             measured_partial_links, h[0]["line"])

    h = readme_claims["claim_related_edges"]
    if h:
        g = h[0]["groups"]
        diff("related edge total", int(g[1]), len(rel_edges), h[0]["line"])
        diff("one-way related edges", int(g[0]), len(rel_one_way), h[0]["line"])

    h = readme_claims["claim_forbids_counts"]
    if h:
        g = h[0]["groups"]
        diff("forbids variant '**What this forbids**'", int(g[0]),
             forbids_variants.get("**What this forbids**", 0), h[0]["line"])
        diff("forbids variant '**What this forbids**:'", int(g[1]),
             forbids_variants.get("**What this forbids**:", 0), h[0]["line"])
        diff("forbids variant '**What this forbids:**'", int(g[2]),
             forbids_variants.get("**What this forbids:**", 0), h[0]["line"])

    h = readme_claims["claim_forbids_on_all"]
    if h:
        diff("records carrying a forbids block", int(h[0]["groups"][0]),
             len(records) - len(no_forbids), h[0]["line"])

    h = readme_claims["claim_consumers_empty"]
    diff("consumers empty on every record", True, len(consumers_nonempty) == 0,
         h[0]["line"] if h else None)

    # README: 0074-0088 decided 2026-09-08, 0089 on 2026-09-09
    h = readme_claims["claim_new_record_dates"]
    if h:
        g = h[0]["groups"]
        diff("0074-0088 all dated as README states (%s)" % g[0], True,
             all(by_id[i][0]["date"] == g[0]
                 for i in ["%04d" % n for n in range(74, 89)] if i in by_id),
             h[0]["line"])
        diff("0089 dated as README states (%s)" % g[1], True,
             by_id.get("0089", [{}])[0].get("date") == g[1],
             h[0]["line"])

    h = readme_claims["claim_next_free"]
    if h:
        diff("next free ADR number", h[0]["groups"][0],
             "%04d" % (max(nums) + 1), h[0]["line"])

    # README supersession table vs frontmatter
    fm_pairs = set((e["superseded"], e["superseder"]) for e in edges)
    rd_pairs = set(readme_sup.keys())
    diffs.append({
        "claim": "README supersession table pairs == frontmatter pairs",
        "readme_line": (readme_claims["supersession_table"][0]["line"]
                        if readme_claims["supersession_table"] else None),
        "readme_claims": sorted("%s<-%s" % p for p in rd_pairs),
        "measured": sorted("%s<-%s" % p for p in fm_pairs),
        "agrees": rd_pairs == fm_pairs,
        "in_readme_only": sorted("%s<-%s" % p for p in rd_pairs - fm_pairs),
        "in_frontmatter_only": sorted("%s<-%s" % p for p in fm_pairs - rd_pairs),
    })

    # README kind (full/partial) vs frontmatter status
    kind_rows = []
    for (sub, by), row in sorted(readme_sup.items()):
        tr = by_id.get(sub, [None])[0]
        st = tr["status"] if tr else None
        expected = "superseded" if row["kind"] == "full" else "accepted"
        kind_rows.append({"superseded": sub, "by": by,
                          "readme_kind": row["kind"],
                          "frontmatter_status": st,
                          "expected_status": expected,
                          "agrees": st == expected})
    diffs.append({
        "claim": "README full/partial kind implies predecessor status",
        "readme_line": None,
        "readme_claims": "full -> superseded, partial -> accepted",
        "measured": kind_rows,
        "agrees": all(k["agrees"] for k in kind_rows) if kind_rows else None,
    })

    # legacy map vs frontmatter
    legacy_diff = []
    for row in readme_claims["legacy_map"]:
        tr = by_id.get(row["adr"], [None])[0]
        if tr is None:
            legacy_diff.append({"adr": row["adr"], "issue": "no such record",
                                "readme_line": row["line"]})
            continue
        issues = []
        if row["legacy_id"] != (tr["legacy_id"] or ""):
            issues.append("legacy_id README=%s fm=%s"
                          % (row["legacy_id"], tr["legacy_id"]))
        if row["title"].strip() != (tr["title"] or "").strip():
            issues.append("title differs")
        cell = row["status_cell"]
        fm_status = tr["status"]
        if not cell.startswith(fm_status):
            issues.append("status README=%r fm=%r" % (cell, fm_status))
        if issues:
            legacy_diff.append({"adr": row["adr"], "readme_line": row["line"],
                                "issues": issues,
                                "readme_title": row["title"][:200],
                                "fm_title": (tr["title"] or "")[:200]})
    diffs.append({
        "claim": "README legacy-id map rows match frontmatter",
        "readme_line": None,
        "readme_claims": "%d rows" % len(readme_claims["legacy_map"]),
        "measured": {"rows_checked": len(readme_claims["legacy_map"]),
                     "rows_with_issues": len(legacy_diff),
                     "issues": legacy_diff},
        "agrees": len(legacy_diff) == 0,
    })

    # the three deliberate non-supersessions, checked against frontmatter
    non_sup_check = []
    for item in readme_claims["deliberate_non_supersessions"]:
        ids = item["ids"]
        pairs = []
        for a in ids:
            for b in ids:
                if a >= b:
                    continue
                ra = by_id.get(a, [None])[0]
                rb = by_id.get(b, [None])[0]
                if ra is None or rb is None:
                    pairs.append({"pair": [a, b], "issue": "missing record"})
                    continue
                pairs.append({
                    "pair": [a, b],
                    "a_related_b": b in ra["related"],
                    "b_related_a": a in rb["related"],
                    "any_supersession": (b in ra["supersedes"]
                                         or b in ra["superseded_by"]
                                         or a in rb["supersedes"]
                                         or a in rb["superseded_by"]),
                })
        non_sup_check.append({"readme_line": item["line"], "ids": ids,
                              "text_truncated": item["text_truncated"],
                              "pairs": pairs,
                              "no_supersession_confirmed":
                                  all(not p.get("any_supersession", False)
                                      for p in pairs),
                              "related_link_present":
                                  all(p.get("a_related_b") or p.get("b_related_a")
                                      for p in pairs)})
    diffs.append({
        "claim": "three deliberate non-supersessions carry no supersession edge",
        "readme_line": None,
        "readme_claims": "0023/0020 related; 0028/0029/0022 related; "
                         "0089 generalises 0067 (related, one-way)",
        "measured": non_sup_check,
        "agrees": all(n["no_supersession_confirmed"] for n in non_sup_check)
                  if non_sup_check else None,
    })

    citations = citation_check(records)

    for r in records:
        del r["_body"]

    artifact = OrderedDict()
    artifact["_meta"] = {
        "artifact": "adr-graph",
        "generator": os.path.abspath(__file__).replace("\\", "/"),
        "repo": REPO,
        "adr_dir": "references/adr",
        "generated_by": "audit phase: machine-checked ground truth",
        "read_only": True,
        "python": sys.version.split()[0],
        "tagging": ("[measured] = computed here; [documented] = a doc asserts "
                    "it; [inferred] = reasoned"),
    }
    artifact["anchors"] = ANCHORS
    artifact["anchor_failures"] = ANCHOR_FAILURES
    artifact["counts"] = {
        "adr_files": len(records),
        "status_distribution": dict(status_dist),
        "supersession_edges": len(edges),
        "supersession_edges_reciprocal": sum(1 for e in edges if e["reciprocal"]),
        "supersession_edges_non_reciprocal": sum(1 for e in edges
                                                 if not e["reciprocal"]),
        "superseding_records": len(superseding_records),
        "superseded_records": len(superseded_records),
        "partial_supersessions_accepted_predecessor": len(partials),
        "full_supersessions_status_superseded": len(fulls),
        "related_edges": len(rel_edges),
        "related_edges_one_way": len(rel_one_way),
        "related_edges_reciprocal_directed": len(rel_reciprocal),
        "records_with_forbids": len(records) - len(no_forbids),
        "records_without_forbids": len(no_forbids),
        "forbids_variants": dict(forbids_variants),
        "records_with_nonempty_evidence": ev_nonempty,
        "distinct_evidence_tags": len(ev_counter),
        "records_with_nonempty_consumers": len(consumers_nonempty),
        "dangling_id_references": len(dangling),
        "duplicate_ids": len(dupes),
        "numbering_gaps": len(gaps),
        "records_missing_mandatory_section": len(sec_issues),
        "distinct_dates": len(date_dist),
        "readme_diff_claims_checked": len(diffs),
        "readme_diff_disagreements": sum(1 for d in diffs
                                         if d["agrees"] is False),
        "citation_date_violations": len(citations["violations"]),
        "citation_date_violations_unexplained":
            len(citations["violations_unexplained"]),
        "citation_docs_scanned": citations["docs_scanned"],
    }
    artifact["records"] = records
    artifact["frontmatter_conformance"] = conformance
    artifact["section_conformance"] = {
        "mandatory_sections": MANDATORY,
        "records_missing_a_mandatory_section": sec_issues,
    }
    artifact["numbering"] = {"min": min(nums) if nums else None,
                             "max": max(nums) if nums else None,
                             "gaps": gaps, "duplicate_ids": dupes,
                             "next_free": "%04d" % (max(nums) + 1) if nums else None}
    artifact["legacy_ids"] = {
        "duplicates": legacy_dupes,
        "distribution": dict(legacy_counts),
    }
    artifact["status_distribution"] = dict(status_dist)
    artifact["supersession"] = {
        "edges": edges,
        "superseding_records": superseding_records,
        "superseded_records": superseded_records,
        "non_reciprocal_edges": [e for e in edges if not e["reciprocal"]],
        "cycles": cycles,
        "acyclic": len(cycles) == 0,
        "partial_supersessions": partials,
        "full_supersessions": fulls,
    }
    artifact["related_graph"] = {
        "edges": rel_edges,
        "one_way_edges": rel_one_way,
        "dangling": [d for d in dangling if d["key"] == "related"],
    }
    artifact["dangling_references"] = dangling
    artifact["forbids_index"] = forbids_index
    artifact["records_without_forbids"] = no_forbids
    artifact["dates"] = {
        "distribution": dict(date_dist),
        "malformed": bad_dates,
        "by_date": dict((d, sorted(r["id"] for r in records if r["date"] == d))
                        for d in sorted(date_dist)),
    }
    artifact["evidence_tags"] = {
        "counts": dict(ev_counter),
        "by_record": dict((r["id"], r["evidence"]) for r in records
                          if r["evidence"]),
    }
    artifact["consumers"] = {
        "note": ("consumers is the declared traceability edge to downstream "
                 "artifacts; README says it is [] on every record in this set"),
        "nonempty": consumers_nonempty,
        "empty_count": len(records) - len(consumers_nonempty),
    }
    artifact["readme_claims"] = readme_claims
    artifact["readme_vs_frontmatter_diff"] = diffs
    artifact["citation_date_check"] = citations

    outdir = os.path.dirname(OUT_JSON)
    if not os.path.isdir(outdir):
        os.makedirs(outdir)
    with io.open(OUT_JSON, "w", encoding="utf-8") as fh:
        fh.write(json.dumps(artifact, indent=1, ensure_ascii=False,
                            sort_keys=False))

    # ASCII-only console report
    print("adr files: %d" % len(records))
    print("status: %s" % sorted(status_dist.items()))
    print("supersession edges: %d (reciprocal %d)"
          % (len(edges), sum(1 for e in edges if e["reciprocal"])))
    print("partial (accepted predecessor): %d  full (superseded): %d"
          % (len(partials), len(fulls)))
    print("related edges: %d one-way: %d" % (len(rel_edges), len(rel_one_way)))
    print("forbids variants: %s" % sorted(forbids_variants.items()))
    print("no-forbids records: %s" % no_forbids)
    print("dangling refs: %d  dupes: %d  gaps: %s"
          % (len(dangling), len(dupes), gaps))
    print("dates: %s" % sorted(date_dist.items()))
    print("readme diffs checked %d, disagreements %d"
          % (len(diffs), sum(1 for d in diffs if d["agrees"] is False)))
    for d in diffs:
        if d["agrees"] is False:
            print("  DISAGREE: %s" % d["claim"])
    print("citation date violations: %d" % len(citations["violations"]))
    for v in citations["violations"][:20]:
        print("  %s (%s) cites %s (%s)"
              % (v["doc"], v["doc_date"], v["adr"], v["adr_date"]))
    print("anchor failures: %d" % len(ANCHOR_FAILURES))
    for a in ANCHOR_FAILURES:
        print("  FAILED ANCHOR: %s -> %s" % (a["assertion"], a["detail"]))
    print("wrote %s" % OUT_JSON)


if __name__ == "__main__":
    main()
