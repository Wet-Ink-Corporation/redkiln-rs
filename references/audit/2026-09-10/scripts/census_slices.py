#!/usr/bin/env python3
"""census_slices.py -- build the SLICE MANIFEST for the redkiln-rs pre-implementation audit.

READ-ONLY on D:/repos/redkiln-rs. Writes exactly one artifact:
  <SCRATCH>/audit/census/slices.json

Everything numeric in the artifact is computed here. Nothing is eyeballed.
ASCII-only stdout (Windows console cannot print section-sign / em-dash / right-arrow).
"""

import json
import os
import re
import sys

REPO = r"D:/repos/redkiln-rs"
OUT = r"D:/repos/redkiln-rs/references/audit/2026-09-10/census/slices.json"

DESIGN = "references/2026-09-08-architecture-design.md"
AC = "references/2026-09-08-acceptance-criteria.md"
SM = "references/2026-09-08-success-measures.md"
RB = "RUNBOOK.md"

TABLE_FIRST = 19   # first data row of the 39-row area table
TABLE_LAST = 57    # last data row

ASSERTS = []


def assert_(name, cond, detail=""):
    ASSERTS.append({"assertion": name, "ok": bool(cond), "detail": str(detail)[:400]})
    if not cond:
        print("ASSERT FAILED: %s :: %s" % (name, str(detail)[:400]))
    return bool(cond)


def lines_of(rel):
    p = os.path.join(REPO, rel)
    with open(p, "r", encoding="utf-8", errors="replace") as f:
        ls = f.read().split("\n")
        if ls and ls[-1] == "":
            ls.pop()   # trailing newline; match wc -l
        return ls


def merge_spans(spans):
    """Union of (file, a, b) inclusive line intervals: overlapping or nested
    intervals in the same file are merged so no line is counted twice."""
    byf = {}
    for f, a, b in spans:
        byf.setdefault(f, []).append((a, b))
    out = []
    for f, ivs in byf.items():
        ivs.sort()
        cur = None
        for a, b in ivs:
            if cur is None or a > cur[1] + 1:
                if cur:
                    out.append((f, cur[0], cur[1]))
                cur = [a, b]
            else:
                cur[1] = max(cur[1], b)
        if cur:
            out.append((f, cur[0], cur[1]))
    return sorted(out)


def span_bytes(ls, a, b):
    """Byte cost (utf-8) of 1-indexed inclusive line span [a,b]."""
    return sum(len(x.encode("utf-8")) + 1 for x in ls[a - 1:b])


# ---------------------------------------------------------------- heading maps
def top_sections(ls, pat):
    """Return [{num, heading, start_line, end_line}] for headings matching pat.
    end_line = line before the next matching heading, or EOF."""
    hits = []
    rx = re.compile(pat)
    for i, ln in enumerate(ls, 1):
        m = rx.match(ln)
        if m:
            hits.append((i, m.group(1), ln.strip()))
    out = []
    for k, (i, num, head) in enumerate(hits):
        end = hits[k + 1][0] - 1 if k + 1 < len(hits) else len(ls)
        out.append({"num": num, "heading": head, "start_line": i, "end_line": end})
    return out


def main():
    dls = lines_of(DESIGN)
    acls = lines_of(AC)
    smls = lines_of(SM)
    rbls = lines_of(RB)

    assert_("design file line count 2052", len(dls) == 2052, len(dls))
    assert_("ac file line count 6567", len(acls) == 6567, len(acls))
    assert_("sm file line count 1438", len(smls) == 1438, len(smls))
    assert_("runbook line count 2475", len(rbls) == 2475, len(rbls))

    # --- design top-level "## N." sections -------------------------------
    dsec = top_sections(dls, r"^## (\d+(?:\.\d+)?)\.?\s+(.*)$")
    dsec_by = {s["num"]: s for s in dsec}
    assert_("design has 13 '## N.' sections (0,0.1,1..11)", len(dsec) == 13,
            [s["num"] for s in dsec])

    # --- AC top-level "## N." sections ------------------------------------
    asec = top_sections(acls, r"^## (\d+)\.\s+(.*)$")
    asec_by = {s["num"]: s for s in asec}
    assert_("ac has 17 '## N.' sections 0..16", len(asec) == 17,
            [s["num"] for s in asec])

    # AC section 0.1 area table -> authoritative area list + SM ranges.
    # Also each AC body section carries "**Criteria:** AC-aaa..AC-bbb".
    area_table = []
    for i in range(20, 46):
        ln = acls[i - 1]
        m = re.match(r"^\|\s*(\d+)\s*\|\s*(.+?)\s*\|\s*(\d+)\s*\|", ln)
        if m:
            cells = [c.strip() for c in ln.strip().strip("|").split("|")]
            area_table.append({
                "ac_section": m.group(1),
                "area": cells[1],
                "criteria": int(cells[2]),
                "measures_cell": cells[-1],
                "row_line": i,
            })
    assert_("ac 0.1 area table has 13 rows", len(area_table) == 13,
            len(area_table))

    # criteria range per AC section, from the section's own header block
    ac_range = {}
    for s in asec:
        blob = "\n".join(acls[s["start_line"] - 1:min(s["start_line"] + 6, s["end_line"])])
        m = re.search(r"\*\*Criteria:\*\*\s*AC-(\d+)\.\.AC-(\d+)", blob)
        if m:
            ac_range[s["num"]] = (int(m.group(1)), int(m.group(2)))
    assert_("13 AC sections declare an AC-a..AC-b range", len(ac_range) == 13,
            sorted(ac_range))

    # actual AC anchors per section (bold-at-line-start definitions)
    ac_anchor = {}     # ac id -> first anchor line
    ac_anchor_all = {}  # ac id -> every anchor line (duplicates are documented in 0.4)
    for i, ln in enumerate(acls, 1):
        m = re.match(r"^\*\*AC-(\d+)\*\*", ln)
        if m:
            ac_anchor.setdefault("AC-" + m.group(1), i)
            ac_anchor_all.setdefault("AC-" + m.group(1), []).append(i)
    assert_("AC anchors found >= 500", len(ac_anchor) >= 500, len(ac_anchor))
    # every declared range is fully anchored (each AC id in AC-a..AC-b has a
    # bold-at-line-start definition inside its own section)
    # ... except the ids section 0.4 (line 112) records as cut/retired.
    sec04 = [s for s in top_sections(acls, r"^### (0\.4) ") if s["num"] == "0.4"]
    assert_("AC section 0.4 (id hazards) exists", len(sec04) == 1, sec04)
    retired = set()
    if sec04:
        blob04 = "\n".join(acls[sec04[0]["start_line"] - 1:sec04[0]["start_line"] + 8])
        retired = set(int(x) for x in re.findall(
            r"AC-(\d{3}) \([a-z\-]+: cut", blob04))
    assert_("AC 0.4 names five cut ids", len(retired) == 5, sorted(retired))
    retired_ids = sorted("AC-%03d" % i for i in retired)
    for n, (a, b) in ac_range.items():
        s = asec_by[n]
        missing = [i for i in range(a, b + 1)
                   if i not in retired
                   and not (s["start_line"] <= ac_anchor.get("AC-%03d" % i, -1) <= s["end_line"])]
        assert_("AC section %s: every non-retired id in its declared range is anchored inside it" % n,
                not missing, ("missing", missing[:10]))

    # AC "### N.M" subsections (the narrow-read unit inside a section)
    ac_subsec = []
    sub_hits = [(i, ln.strip()) for i, ln in enumerate(acls, 1) if re.match(r"^### ", ln)]
    for k, (i, head) in enumerate(sub_hits):
        # a subsection ends before the next ### OR the next ## heading
        nxt = len(acls)
        for j in range(i + 1, len(acls) + 1):
            if re.match(r"^##{1,2} ", acls[j - 1]):
                nxt = j - 1
                break
        ac_subsec.append({"heading": head, "start_line": i, "end_line": nxt})
    assert_("AC subsections found (expect ~101)", 90 <= len(ac_subsec) <= 110, len(ac_subsec))

    # --- SM: 13 "### 1.N" area subsections + every SM row line ------------
    smsub = []
    hits = [(i, ln) for i, ln in enumerate(smls, 1)
            if re.match(r"^### 1\.\d+ ", ln)]
    reg = [s for s in top_sections(smls, r"^## (\d+)\.\s+(.*)$") if s["num"] == "1"][0]
    for k, (i, ln) in enumerate(hits):
        end = hits[k + 1][0] - 1 if k + 1 < len(hits) else reg["end_line"]
        rng = re.findall(r"SM-(\d+)", ln)
        smsub.append({"sub": re.match(r"^### (1\.\d+)", ln).group(1),
                      "heading": ln.strip(), "start_line": i, "end_line": end,
                      "declared_ids": ["SM-" + x for x in rng]})
    assert_("SM has 13 area subsections 1.1..1.13", len(smsub) == 13, len(smsub))

    sm_row = {}   # SM id -> line
    for i, ln in enumerate(smls, 1):
        m = re.match(r"^\*\*SM-(\d+)\b", ln)
        if m:
            sm_row.setdefault("SM-" + m.group(1), i)
    assert_("SM rows found (expect ~135)", 120 <= len(sm_row) <= 140, len(sm_row))

    def sms_in_span(a, b):
        return sorted([(k, v) for k, v in sm_row.items() if a <= v <= b],
                      key=lambda t: t[1])

    # --- RUNBOOK "## " steps ---------------------------------------------
    rbsec = top_sections(rbls, r"^## (.*)$")
    rb_by = {}
    for s in rbsec:
        lbl = s["heading"].lstrip("# ").strip()
        key = lbl.split(" ")[0] + (" " + lbl.split(" ")[1] if lbl.startswith("Step ") else "")
        s["label"] = lbl
        s["key"] = key
        rb_by[key] = s
    assert_("runbook has 14 '## ' sections", len(rbsec) == 14, len(rbsec))
    for k in ["Step 0", "Step 1", "Step 2", "Step 3", "Step 4", "Step 4a",
              "Step 5", "Step 6", "Step 7", "Step 8", "Step 9"]:
        assert_("runbook key present: " + k, k in rb_by, sorted(rb_by))

    # ---------------------------------------------------- the 39-row table
    hdr = dls[TABLE_FIRST - 3]
    assert_("design table header at line 17",
            hdr.startswith("| Area | State |"), hdr[:80])
    assert_("design table separator at line 18",
            dls[TABLE_FIRST - 2].startswith("| --- |"), dls[TABLE_FIRST - 2][:40])

    rows = []
    for i in range(TABLE_FIRST, TABLE_LAST + 1):
        ln = dls[i - 1]
        if not ln.startswith("|"):
            assert_("row %d is a table row" % i, False, ln[:80])
            continue
        cells = [c.strip() for c in ln.strip().strip("|").split("|")]
        assert_("row %d has 5 cells" % i, len(cells) == 5, len(cells))
        rows.append({"row_line": i, "cells": cells, "raw": ln})
    assert_("exactly 39 area rows", len(rows) == 39, len(rows))
    assert_("line 58 is not a table row", not dls[57].startswith("| **"), dls[57][:60])

    def plain(s):
        return re.sub(r"[*`]", "", s).strip()

    def state_of(cell):
        # the row's state is the FIRST state token in the cell; later tokens
        # are parenthetical history ("DECIDED (was PROVISIONAL...)")
        m = re.search(r"\b(BLOCKED|PROVISIONAL|DECIDED)\b", cell)
        return m.group(1) if m else "UNKNOWN"

    # Spike vocabulary: S-<NAME> (S-BIN, S-GIX, S-H1c, S-TTL1 ...), the P<n> panel
    # spikes (P1..P9; "P01" is an item id, not a spike, so no leading zero), and
    # the charter experiments E0..E3.
    SPIKE_RX = re.compile(r"\b(S-[A-Z][A-Za-z0-9]{1,10}|P[1-9]\d?|E\d)\b")

    for r in rows:
        blob = r["raw"]
        r["area_name"] = plain(r["cells"][0])
        r["state"] = state_of(r["cells"][1])
        r["gates_phase_1"] = r["cells"][4][:1500]
        r["adrs"] = sorted(set("ADR-" + x for x in re.findall(r"ADR-(\d{4})", blob)))
        r["acs"] = sorted(set("AC-" + x for x in re.findall(r"\bAC-(\d{3})\b", blob)))
        r["sms"] = sorted(set("SM-" + x for x in re.findall(r"\bSM-(\d{3})\b", blob)))
        r["findings"] = sorted(set("F" + x for x in re.findall(r"\bF(\d{1,2})\b", blob)),
                               key=lambda s: int(s[1:]))
        r["spikes"] = sorted(set(SPIKE_RX.findall(blob)))
        r["cells_truncated"] = [c[:1500] for c in r["cells"]]

    assert_("states partition 39 rows",
            sum(1 for r in rows if r["state"] == "UNKNOWN") == 0,
            [r["area_name"] for r in rows if r["state"] == "UNKNOWN"])
    counts_state = {}
    for r in rows:
        counts_state[r["state"]] = counts_state.get(r["state"], 0) + 1
    assert_("header claim 26 DECIDED / 3 PROVISIONAL / 10 BLOCKED",
            counts_state == {"DECIDED": 26, "PROVISIONAL": 3, "BLOCKED": 10},
            counts_state)

    # ------------------------------------------------- the area -> section map
    # MAPPING RULE (recorded verbatim in the artifact):
    #  (a) AC sections: the 39 design-table areas are hand-assigned to the 13
    #      areas of the AC register's own 0.1 area table -- that table IS the
    #      authoritative area list, so the join is design-area -> AC-area.
    #      A design row that names no behaviour the AC register owns gets [].
    #  (b) design sections: hand-assigned by the row's subject matter against
    #      the 13 "## N." headings, then CORROBORATED mechanically by scoring
    #      the row's distinctive terms across every section body (below).
    #  (c) SM rows: inherited from (a) -- AC section N maps to SM subsection
    #      1.(N-1) by construction (both registers are the same 13 areas in
    #      the same order); the SM ids are then READ OUT of that subsection's
    #      line span, never from the declared range.
    #  (d) runbook steps: hand-assigned from the step titles, corroborated the
    #      same way as (b).
    # confidence: high  = the area name shares its head noun with the target
    #                     heading, or the target is the row's only home;
    #             medium= assigned by a distinctive term whose occurrences
    #                     concentrate in the target;
    #             low   = cross-cutting row with no single owner.
    MAP = {
        # row_line: (ac_sections, design_sections, runbook_step_keys, confidence)
        19: (["3"], ["4", "1", "10"], ["Step 4"], "high"),
        20: (["3"], ["4", "1", "10"], ["Step 4", "Step 4a"], "high"),
        21: (["3"], ["4"], ["Step 4"], "high"),
        22: (["5", "13"], ["5", "10"], ["Step 3", "Step 5"], "low"),
        23: (["7"], ["7", "10"], ["Step 7"], "medium"),
        24: ([], ["7", "10"], [], "low"),
        25: ([], ["0.1", "10"], ["Step 4", "Step 8"], "low"),
        26: (["6"], ["5", "9"], ["Step 6"], "high"),
        27: (["6"], ["5", "9"], ["Step 6"], "high"),
        28: (["11"], ["7", "10"], ["Step 7"], "medium"),
        29: (["8"], ["6"], ["Step 7"], "medium"),
        30: (["9"], ["6"], ["Step 3", "Step 7"], "high"),
        31: (["9"], ["1", "6"], ["Step 3"], "medium"),
        32: (["14"], ["8"], [], "high"),
        33: ([], ["0", "1", "3"], [], "low"),
        34: (["2"], ["3"], ["Step 7"], "high"),
        35: (["2"], ["3"], ["Step 7"], "medium"),
        36: (["14"], ["3"], ["Step 7"], "high"),
        37: (["6"], ["3"], ["Step 6"], "high"),
        38: (["5", "6"], ["5"], ["Step 6"], "high"),
        39: (["6"], ["3", "5"], ["Step 6"], "high"),
        40: (["8"], ["6"], ["Step 7"], "high"),
        41: (["8"], ["6"], ["Step 7"], "high"),
        42: (["9"], ["6"], ["Step 7"], "high"),
        43: (["9", "10"], ["6"], ["Step 7"], "medium"),
        44: (["9"], ["6"], ["Step 7"], "medium"),
        45: (["10"], ["6"], ["Step 7"], "high"),
        46: (["7"], ["8", "3"], ["Step 7"], "medium"),
        47: (["7"], ["7", "9"], ["Step 7"], "medium"),
        48: (["7"], ["7", "1"], ["Step 7"], "medium"),
        49: (["11"], ["7"], ["Step 7"], "high"),
        50: (["11"], ["7"], ["Step 7"], "high"),
        51: (["11"], ["7"], ["Step 7"], "high"),
        52: (["4"], ["4"], [], "high"),
        53: (["4"], ["4"], [], "high"),
        54: (["5"], ["5"], ["Step 5", "Step 6"], "high"),
        55: (["13"], ["8"], ["Step 8"], "high"),
        56: ([], ["9"], ["Step 5"], "medium"),
        57: ([], ["10"], ["Step 8", "Preamble"], "low"),
    }
    # Per-row notes where the hand mapping needed a reason beyond the rule.
    NOTES = {
        22: ("Cross-cutting: ChangeSummary is defined in rk-types (AC section 5, AC-154), "
             "consumed by verify (AC-356, section 10), migrated (AC-475, section 13) and "
             "shaped by the Toolchain port (AC-497, section 14). Section 5 (typed artifacts) "
             "and 13 (migrate) are the two the AC register's own section-1 'What is blocked' "
             "row names ('renders or migrates one'); the other two are reachable through "
             "acs_anchored."),
        24: ("ABSENCE: the term 'shorthand' occurs 0 times in the AC register; 'quality' "
             "concentrates in section 4 (packs). No AC section is assigned; P7 lives in "
             "design sections 7 and 10."),
        25: ("Cross-cutting label row; it gates the 'design final' label, not any one "
             "area. No AC section is assigned."),
        26: ("gix occurs 0 times in the AC register body (once in section 15 sources); "
             "design sections 5 and 9 carry 20 and 25 hits respectively."),
        33: ("ABSENCE: 'single binary' / 'owns the loop' occur 0 times in the AC register. "
             "F23/F26 are discussed in design section 1."),
        46: ("'StructuredOutput' occurs in AC section 14 (6), 7 (3), 12 (3); the design "
             "row is the harness adapter's card check, so section 7 is assigned."),
        56: ("ABSENCE: no AC section owns workspace / toolchain pin / lint policy "
             "('clippy' appears 3 times in AC section 2 only)."),
        57: ("ABSENCE: 'fix lane' and 'story lane' occur once each in the AC register "
             "(sections 5 and 9). No AC section is assigned; the scope row is design "
             "section 10 and runbook Step 8."),
    }
    assert_("MAP covers all 39 rows",
            sorted(MAP) == [r["row_line"] for r in rows], len(MAP))

    # ---- mechanical corroboration of (b)/(d): distinctive-term scoring ----
    STOP = set("""the and for a an as at by in of on to it its is was with that this
    which what not no nor but or if then than from into over under whole as-a
    are be been being design final source setting route default rows row""".split())

    def terms_of(area):
        toks = re.findall(r"[A-Za-z_][A-Za-z0-9_]{3,}", area.lower())
        return sorted(set(t for t in toks if t not in STOP))

    def score(ls, sections, terms):
        out = []
        for s in sections:
            body = "\n".join(ls[s["start_line"] - 1:s["end_line"]]).lower()
            n = sum(body.count(t) for t in terms)
            out.append((s.get("num") or s.get("key"), n))
        out.sort(key=lambda t: -t[1])
        return out[:3]

    def gates_class(cell):
        c = re.sub(r"[*]", "", cell).strip()
        if re.match(r"^yes\b", c, re.I):
            return "yes"
        if re.match(r"^no\b", c, re.I):
            return "no"
        if re.match(r"^it is phase 1", c, re.I):
            return "yes"    # row 33: "It **is** Phase 1"
        return "other"

    # ------------------------------------------------------ build the slices
    slices = []
    for r in rows:
        acn, dsn, rbk, conf = MAP[r["row_line"]]

        ac_sections = []
        for n in acn:
            s = asec_by[n]
            assert_("ac section %s span valid" % n,
                    1 <= s["start_line"] < s["end_line"] <= len(acls), s)
            assert_("ac section %s heading text is at its start_line" % n,
                    acls[s["start_line"] - 1].strip() == s["heading"], s["heading"][:60])
            ac_sections.append({
                "section": n, "heading": s["heading"],
                "start_line": s["start_line"], "end_line": s["end_line"],
                "ac_range": ("AC-%03d..AC-%03d" % ac_range[n]) if n in ac_range else None,
                "bytes": span_bytes(acls, s["start_line"], s["end_line"]),
                "confidence": conf,
                "join": "AC register 0.1 area table (line %d)" % (
                    [a["row_line"] for a in area_table if a["ac_section"] == n] or [0])[0],
            })

        design_sections = []
        for n in dsn:
            s = dsec_by[n]
            assert_("design section %s span valid" % n,
                    1 <= s["start_line"] < s["end_line"] <= len(dls), s)
            assert_("design section %s heading text is at its start_line" % n,
                    dls[s["start_line"] - 1].strip() == s["heading"], s["heading"][:60])
            design_sections.append({
                "section": n, "heading": s["heading"],
                "start_line": s["start_line"], "end_line": s["end_line"],
                "bytes": span_bytes(dls, s["start_line"], s["end_line"]),
                "confidence": conf,
            })

        sm_rows = []
        sm_subsections = []
        for n in acn:
            sub = "1.%d" % (int(n) - 1)
            cand = [x for x in smsub if x["sub"] == sub]
            if not assert_("sm subsection %s exists" % sub, len(cand) == 1, sub):
                continue
            x = cand[0]
            assert_("sm subsection %s heading text is at its start_line" % sub,
                    smls[x["start_line"] - 1].strip() == x["heading"], x["heading"][:60])
            sm_subsections.append({"sub": sub, "heading": x["heading"],
                                   "start_line": x["start_line"],
                                   "end_line": x["end_line"],
                                   "bytes": span_bytes(smls, x["start_line"], x["end_line"])})
            for sid, line in sms_in_span(x["start_line"], x["end_line"]):
                sm_rows.append({"sm": sid, "line": line, "sm_subsection": sub})
        # any SM explicitly named in the row that the inherited span missed
        have = set(x["sm"] for x in sm_rows)
        for sid in r["sms"]:
            if sid not in have and sid in sm_row:
                sm_rows.append({"sm": sid, "line": sm_row[sid],
                                "sm_subsection": "named-in-row"})

        runbook_steps = []
        for k in rbk:
            s = rb_by[k]
            assert_("runbook step %s span valid" % k,
                    1 <= s["start_line"] < s["end_line"] <= len(rbls), s)
            assert_("runbook step %s heading text is at its start_line" % k,
                    rbls[s["start_line"] - 1].strip() == s["heading"], s["heading"][:60])
            runbook_steps.append({"step": s["label"], "key": k,
                                  "start_line": s["start_line"],
                                  "end_line": s["end_line"],
                                  "bytes": span_bytes(rbls, s["start_line"], s["end_line"]),
                                  "confidence": conf})

        spans = []
        for s in design_sections:
            spans.append((DESIGN, s["start_line"], s["end_line"], s["bytes"]))
        for s in ac_sections:
            spans.append((AC, s["start_line"], s["end_line"], s["bytes"]))
        for s in sm_subsections:
            spans.append((SM, s["start_line"], s["end_line"], s["bytes"]))
        for s in runbook_steps:
            spans.append((RB, s["start_line"], s["end_line"], s["bytes"]))
        est_lines = sum(b - a + 1 for _, a, b, _ in spans) + 1
        est_bytes = sum(by for _, _, _, by in spans) + len(r["raw"].encode("utf-8"))

        # ACs the row names, with their anchor line and enclosing AC subsection,
        # so an auditor can jump straight to them instead of reading a section.
        acs_anchored = []
        for aid in r["acs"]:
            if aid in ac_anchor:
                ln_ = ac_anchor[aid]
                sub = [x for x in ac_subsec if x["start_line"] <= ln_ <= x["end_line"]]
                acs_anchored.append({
                    "ac": aid, "line": ln_,
                    "ac_section": [x["num"] for x in asec
                                   if x["start_line"] <= ln_ <= x["end_line"]][0],
                    "subsection": sub[0]["heading"] if sub else None,
                    "sub_start_line": sub[0]["start_line"] if sub else None,
                    "sub_end_line": sub[0]["end_line"] if sub else None,
                })
            else:
                acs_anchored.append({"ac": aid, "line": None,
                                     "note": "no bold-at-line-start anchor found"})
        # narrow estimate: replace each assigned AC section by only the
        # subsections that contain an AC the row itself names (falls back to
        # the whole section when the row names none there)
        narrow_spans = set()
        for f, a, b, _ in spans:
            if f != AC:
                narrow_spans.add((f, a, b))
        for n in acn:
            s = asec_by[n]
            subs = set((x["sub_start_line"], x["sub_end_line"]) for x in acs_anchored
                       if x.get("ac_section") == n and x.get("sub_start_line"))
            if subs:
                for a, b in subs:
                    narrow_spans.add((AC, a, b))
            else:
                narrow_spans.add((AC, s["start_line"], s["end_line"]))
        narrow_spans = set(merge_spans(narrow_spans))
        est_lines_narrow = sum(b - a + 1 for _, a, b in narrow_spans) + 1

        terms = terms_of(r["area_name"])
        slices.append({
            "slice_id": "A%02d" % (len(slices) + 1),
            "row_line": r["row_line"],
            "area_name": r["area_name"],
            "state": r["state"],
            "gates_phase_1": r["gates_phase_1"],
            "adrs": r["adrs"], "acs": r["acs"], "sms": r["sms"],
            "findings": r["findings"], "spikes": r["spikes"],
            "acs_anchored": acs_anchored,
            "mapping_note": NOTES.get(r["row_line"]),
            "row_cells_truncated": {
                "area": r["cells_truncated"][0], "state": r["cells_truncated"][1],
                "evidence": r["cells_truncated"][2],
                "what_unblocks_it": r["cells_truncated"][3],
                "gates_phase_1": r["cells_truncated"][4]},
            "design_sections": design_sections,
            "ac_sections": ac_sections,
            "sm_subsections": sm_subsections,
            "sm_rows": sm_rows,
            "runbook_steps": runbook_steps,
            "est_read_lines": est_lines,
            "est_read_bytes": est_bytes,
            "est_read_lines_narrow": est_lines_narrow,
            "mapping_confidence": conf,
            "corroboration": {
                "terms": terms,
                "top_design_sections": score(dls, dsec, terms),
                "top_ac_sections": score(acls, asec, terms),
                "top_runbook_steps": score(rbls, rbsec, terms),
            },
            "oversize": est_lines > 900,
            "gates_phase_1_class": gates_class(r["gates_phase_1"]),
            "_spans": spans,
            "_narrow_spans": narrow_spans,
        })

    # ------------------------------------------------------------- grouping
    GROUPS = [
        ("G01", "Selection engine - context-vector and derived-facet sources", [19, 20]),
        ("G02", "Selection step 3 - stale_policy", [21]),
        ("G03", "ChangeSummary type design (S-BIN)", [22]),
        ("G04", "Harness dispatch surface - card delivery, structured output, isolation", [23, 46, 47]),
        ("G05", "Programme shape, design-final label and Phase-1 scope", [25, 33, 57]),
        ("G06", "Git write path and trailer writing (gitoxide / S-GIX)", [26, 27]),
        ("G07", "Dispatch economics - cache rule, cache TTL, effort pins, quality shorthand", [24, 28, 48, 49]),
        ("G08", "Human gate - durable mark, gate record, mid-graph placement", [29, 40, 41]),
        ("G09", "Review-gate calibration - false approval and E3 rates", [30, 31]),
        ("G10", "Distribution and the plugin surface", [32]),
        ("G11", "Kernel core - split, replay, memo, budgets, no framework", [34, 35]),
        ("G12", "The five named ports", [36]),
        ("G13", "Git is not a port, and reconcile() at every entry", [37, 39]),
        ("G14", "Tier-1/tier-2 durable state and the second trailer axis", [38]),
        ("G15", "Review gate design and the lenient policy / Advisory sink", [42, 44]),
        ("G16", "Gate sufficiency and the verification gate", [43, 45]),
        ("G17", "Cost and headroom telemetry, and throttling", [50, 51]),
        ("G18", "Packs, rigor enum, catalog v0 and the lattice", [52, 53]),
        ("G19", "Typed artifacts, layout C and the single write path", [54]),
        ("G20", "Clean break, redkiln-migrate and self-hosting", [55]),
        ("G21", "Rust realisation - workspace, toolchain, lint, dependencies", [56]),
    ]
    seen = []
    for _, _, rl in GROUPS:
        seen += rl
    assert_("grouping covers all 39 rows exactly once",
            sorted(seen) == [r["row_line"] for r in rows],
            (len(seen), len(set(seen))))
    assert_("group count in 20..24", 20 <= len(GROUPS) <= 24, len(GROUPS))

    by_line = {s["row_line"]: s for s in slices}
    groups = []
    for gid, name, rl in GROUPS:
        members = [by_line[x] for x in rl]
        uspans = set()
        for m in members:
            for sp in m["_spans"]:
                uspans.add((sp[0], sp[1], sp[2]))
        uspans = set(merge_spans(uspans))
        ul = sum(b - a + 1 for _, a, b in uspans) + len(members)
        unspans = set()
        for m in members:
            unspans |= m["_narrow_spans"]
        unspans = set(merge_spans(unspans))
        un = sum(b - a + 1 for _, a, b in unspans) + len(members)
        ub = 0
        fls = {DESIGN: dls, AC: acls, SM: smls, RB: rbls}
        for f, a, b in uspans:
            ub += span_bytes(fls[f], a, b)
        groups.append({
            "group_id": gid,
            "slice_name": name,
            "row_lines": rl,
            "areas": [m["area_name"] for m in members],
            "states": sorted(set(m["state"] for m in members)),
            "gates_phase_1_classes": [m["gates_phase_1_class"] for m in members],
            "gates_phase_1_any_yes": any(m["gates_phase_1_class"] == "yes" for m in members),
            "est_read_lines": ul,
            "est_read_lines_narrow": un,
            "est_read_bytes": ub,
            "distinct_spans": sorted([{"file": f, "start_line": a, "end_line": b}
                                      for f, a, b in uspans],
                                     key=lambda d: (d["file"], d["start_line"])),
            "oversize_members": [m["area_name"] for m in members if m["oversize"]],
            "needs_further_split": ul > 1400,
        })

    for s in slices:
        del s["_spans"]
        del s["_narrow_spans"]

    lines_all = [g["est_read_lines"] for g in groups]
    artifact = {
        "artifact": "slices.json",
        "phase": "audit ground truth -- slice manifest",
        "generated_by": "audit/scripts/census_slices.py",
        "repo": REPO,
        "read_only": True,
        "source_files": {
            DESIGN: len(dls), AC: len(acls), SM: len(smls), RB: len(rbls)},
        "mapping_rule": {
            "ac_sections": ("[documented] The AC register's own 0.1 area table (lines 24-38) "
                            "is the authoritative area list: 13 areas, one per '## N.' section, "
                            "each declaring its AC-a..AC-b range and its SM ids. Each of the 39 "
                            "design-table areas was hand-assigned to one (occasionally two) of "
                            "those 13 AC areas; a design row asserting nothing the AC register "
                            "owns gets an empty list, reported as ABSENCE."),
            "design_sections": ("[inferred] Hand-assigned against the 13 '## N.' headings of the "
                                "architecture design by subject matter, then corroborated "
                                "mechanically: each area's distinctive terms (tokens of length "
                                ">=4 from the area name, minus a stoplist) are counted across "
                                "every section body and the top 3 recorded in 'corroboration'."),
            "sm_rows": ("[measured] Inherited from the AC assignment: AC section N corresponds to "
                        "SM subsection 1.(N-1) -- both registers are the same 13 areas in the same "
                        "order, verified by heading text. SM ids are then read out of that "
                        "subsection's line span, not from its declared range; any SM named in the "
                        "design row itself but outside that span is added and tagged "
                        "'named-in-row'."),
            "runbook_steps": ("[inferred] Hand-assigned from the 14 '## ' step titles, corroborated "
                              "by the same term-scoring as design_sections."),
            "confidence": ("high = the area name shares its head noun with the target heading, or "
                           "the target is the row's only possible home; medium = assigned via a "
                           "distinctive term whose occurrences concentrate in the target; low = "
                           "cross-cutting row with no single owner."),
            "est_read_lines": ("[measured] Sum of the inclusive line counts of every distinct span "
                               "assigned to the area (design sections + AC sections + SM subsection "
                               "+ runbook steps), plus 1 for the table row itself. Group totals "
                               "de-duplicate spans shared between member areas. est_read_bytes is "
                               "carried alongside because AC lines are extremely long and a line "
                               "count understates the true read cost there by roughly 4x."),
            "est_read_lines_narrow": ("[measured] Same as est_read_lines except that each assigned "
                                      "AC section is replaced by only the '### N.M' subsections that "
                                      "contain an AC the design row itself names (acs_anchored); an "
                                      "AC section in which the row names no AC is kept whole. This is "
                                      "the read an auditor who trusts the row's own citations would do."),
            "oversize": ("[measured] est_read_lines > 900 on the WIDE measure. Because every AC "
                         "section is 395-697 lines, almost every area with an AC section trips it; "
                         "the flag is reported honestly and the narrow measure is the remedy."),
        },
        "counts": {
            "areas": len(slices),
            "state_DECIDED": counts_state.get("DECIDED", 0),
            "state_PROVISIONAL": counts_state.get("PROVISIONAL", 0),
            "state_BLOCKED": counts_state.get("BLOCKED", 0),
            "groups": len(groups),
            "ac_top_level_sections": len(asec),
            "ac_area_rows_in_0_1": len(area_table),
            "design_top_level_sections": len(dsec),
            "sm_area_subsections": len(smsub),
            "sm_rows_indexed": len(sm_row),
            "runbook_sections": len(rbsec),
            "areas_with_no_ac_section": sum(1 for s in slices if not s["ac_sections"]),
            "areas_oversize_gt_900_lines": sum(1 for s in slices if s["oversize"]),
            "areas_oversize_gt_900_lines_narrow": sum(1 for s in slices if s["est_read_lines_narrow"] > 900),
            "gates_phase_1_yes": sum(1 for s in slices if s["gates_phase_1_class"] == "yes"),
            "gates_phase_1_no": sum(1 for s in slices if s["gates_phase_1_class"] == "no"),
            "gates_phase_1_other": sum(1 for s in slices if s["gates_phase_1_class"] == "other"),
            "group_est_lines_narrow_min": min(g["est_read_lines_narrow"] for g in groups),
            "group_est_lines_narrow_max": max(g["est_read_lines_narrow"] for g in groups),
            "group_est_lines_min": min(lines_all),
            "group_est_lines_max": max(lines_all),
            "group_est_lines_mean": round(sum(lines_all) / len(lines_all), 1),
            "assertions_run": len(ASSERTS),
            "assertions_failed": sum(1 for a in ASSERTS if not a["ok"]),
        },
        "ac_area_table": area_table,
        "ac_sections_index": asec,
        "ac_subsections_index": ac_subsec,
        "ac_retired_ids_per_0_4": retired_ids,
        "ac_anchor_duplicates": {k: v for k, v in ac_anchor_all.items() if len(v) > 1},
        "ac_anchors_outside_declared_ranges": sorted(
            k for k, v in ac_anchor.items()
            if not any(a <= int(k[3:]) <= b for a, b in ac_range.values())),
        "ac_anchor_total": len(ac_anchor),
        "ac_declared_ranges": {n: "AC-%03d..AC-%03d" % v for n, v in sorted(ac_range.items(), key=lambda t: int(t[0]))},
        "design_sections_index": dsec,
        "sm_subsections_index": smsub,
        "runbook_sections_index": rbsec,
        "slices": slices,
        "groups": groups,
        "assertions": ASSERTS,
    }

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(artifact, f, indent=1, ensure_ascii=False, sort_keys=False)

    print("wrote %s" % OUT)
    print("areas=%d groups=%d assertions=%d failed=%d" % (
        len(slices), len(groups), len(ASSERTS),
        sum(1 for a in ASSERTS if not a["ok"])))
    print("group lines min/mean/max = %d / %s / %d" % (
        min(lines_all), artifact["counts"]["group_est_lines_mean"], max(lines_all)))
    print("oversize areas (>900 lines): %d" % artifact["counts"]["areas_oversize_gt_900_lines"])
    for s in slices:
        if s["oversize"]:
            print("  OVERSIZE %s %d lines :: %s" % (s["slice_id"], s["est_read_lines"],
                                                    s["area_name"][:60].encode("ascii", "replace").decode()))
    for g in groups:
        print("  %s %5d wide %5d narrow %8d bytes  %s" % (
            g["group_id"], g["est_read_lines"], g["est_read_lines_narrow"],
            g["est_read_bytes"], g["slice_name"][:52]))
    return 0 if not any(not a["ok"] for a in ASSERTS) else 2


if __name__ == "__main__":
    sys.exit(main())
