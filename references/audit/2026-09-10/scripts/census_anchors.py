#!/usr/bin/env python
# -*- coding: utf-8 -*-
"""
census_anchors.py -- cross-reference anchor census for D:/repos/redkiln-rs.

READ-ONLY on the repo. Writes ONLY the JSON artifact given by OUT_PATH.
Never prints non-ASCII (Windows console). Deterministic: sorted walks, no timestamps in the body
(the generation timestamp lives in meta.generated_at and is excluded from the reproducibility hash).

Sections:
  (a) section references  -- U+00A7 forms, "section N.M" forms, ranges, sub-items
  (b) evidence-brief C.n item references
  (c) repo-relative file-path references, resolved against the real filesystem
  (d) markdown links and bare URLs (inventory only; nothing is fetched)
  plus a per-document heading index (all ATX levels, fenced code blocks excluded)
"""
import hashlib
import json
import os
import re
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone

REPO = "D:/repos/redkiln-rs"
OUT_PATH = "D:/repos/redkiln-rs/references/audit/2026-09-10/census/anchors.json"
EXCLUDE_PREFIXES = ("spikes/fixtures/",)
SS = "\u00a7"  # section sign

# ----------------------------------------------------------------------------------------------
# 0. filesystem enumeration
# ----------------------------------------------------------------------------------------------

def walk_repo():
    """All files under REPO (excluding .git), as sorted forward-slash relative paths."""
    out = []
    for root, dirs, files in os.walk(REPO):
        dirs[:] = sorted(d for d in dirs if d != ".git")
        rel_root = os.path.relpath(root, REPO).replace("\\", "/")
        for f in sorted(files):
            p = f if rel_root == "." else rel_root + "/" + f
            out.append(p)
    return out


ALL_FILES = walk_repo()
ALL_FILES_SET = set(ALL_FILES)
ALL_DIRS_SET = set()
for _p in ALL_FILES:
    parts = _p.split("/")
    for i in range(1, len(parts)):
        ALL_DIRS_SET.add("/".join(parts[:i]))
BASENAME_INDEX = defaultdict(list)
for _p in ALL_FILES:
    BASENAME_INDEX[_p.rsplit("/", 1)[-1]].append(_p)

CORPUS = [p for p in ALL_FILES
          if p.endswith(".md") and not any(p.startswith(x) for x in EXCLUDE_PREFIXES)]


def read_lines(rel):
    with open(os.path.join(REPO, rel), "r", encoding="utf-8", errors="replace", newline="") as fh:
        text = fh.read()
    # keep line numbers 1-based and stable across CRLF/LF
    return text.split("\n")


DOC_LINES = {p: read_lines(p) for p in CORPUS}

# ----------------------------------------------------------------------------------------------
# 1. heading index (fenced code blocks excluded)
# ----------------------------------------------------------------------------------------------

FENCE_RE = re.compile(r"^ {0,3}(`{3,}|~{3,})")
ATX_RE = re.compile(r"^ {0,3}(#{1,6})[ \t]+(.*?)[ \t]*$")
# heading number label: "4.5", "4.2.1", "9a", "4a.1", "0", plus "Step 3", "C." style letters, "E3"
NUM_LABEL_RE = re.compile(r"^\**\s*(\d+[a-z]?(?:\.\d+[a-z]?)*)\.?(?=[\s\u2014\u2013:\-*]|$)")
STEP_LABEL_RE = re.compile(r"^\**\s*(?:Step|Checkpoint|Phase)\s+(\d+[a-z]?)\b", re.I)
LETTER_LABEL_RE = re.compile(r"^\**\s*([A-Z])\.\s")
ALNUM_LABEL_RE = re.compile(r"^\**\s*([A-Z]\d+)\b")
SIGN_TITLE_RE = re.compile(r"^\**\s*" + SS + r"\s?(\d+[a-z]?(?:\.\d+[a-z]?)*)")


def fence_map(lines):
    """Return list[bool]: True where the line is inside a fenced code block (fence lines themselves
    are marked True as well)."""
    inside = []
    open_marker = None
    for ln in lines:
        m = FENCE_RE.match(ln)
        if open_marker is None:
            if m:
                open_marker = m.group(1)[0]
                inside.append(True)
            else:
                inside.append(False)
        else:
            inside.append(True)
            if m and m.group(1)[0] == open_marker:
                open_marker = None
    return inside


def heading_labels(text):
    """Return list of label keys under which this heading can be cited."""
    keys = []
    t = text.strip()
    m = NUM_LABEL_RE.match(t)
    if m:
        keys.append(m.group(1))
    m = STEP_LABEL_RE.match(t)
    if m:
        keys.append(m.group(1))            # "Step 3" cited as section 3 in the runbook
        keys.append("Step " + m.group(1))
    m = LETTER_LABEL_RE.match(t)
    if m:
        keys.append(m.group(1))
    m = ALNUM_LABEL_RE.match(t)
    if m:
        keys.append(m.group(1))
    m = SIGN_TITLE_RE.match(t)
    if m:
        keys.append(m.group(1))            # a heading titled "s10 -- ..." is citable as section 10
    return keys


HEADINGS = {}     # doc -> [ {line, level, text, labels} ]
LABEL_MAP = {}    # doc -> {label -> [line,...]}
FENCED = {}       # doc -> list[bool]
for doc in CORPUS:
    lines = DOC_LINES[doc]
    fm = fence_map(lines)
    FENCED[doc] = fm
    hs = []
    lm = defaultdict(list)
    for i, ln in enumerate(lines, 1):
        if fm[i - 1]:
            continue
        m = ATX_RE.match(ln)
        if not m:
            continue
        text = m.group(2)
        # strip trailing closing hashes
        text = re.sub(r"\s+#+\s*$", "", text)
        labels = heading_labels(text)
        hs.append({"line": i, "level": len(m.group(1)), "text": text, "labels": labels})
        for k in labels:
            lm[k].append(i)
    HEADINGS[doc] = hs
    LABEL_MAP[doc] = dict(lm)

# ----------------------------------------------------------------------------------------------
# 2. document alias table (for inferring which document a section reference names)
# ----------------------------------------------------------------------------------------------
# Each entry: (regex, target_doc_or_callable, kind). Matched case-sensitively unless (?i) given.
# The alias whose match END lies nearest before the reference wins; ties -> longest match.

ALIASES = [
    # explicit repo paths
    (r"(?:references|spikes)/[\w./\-]+\.md", lambda m: m.group(0), "path"),
    (r"\bRUNBOOK\.md", lambda m: "RUNBOOK.md", "path"),
    # pre-registrations
    (r"\bMETHOD-E3H\b", "spikes/METHOD-E3H.md", "name"),
    (r"\bMETHOD-E3\b", "spikes/METHOD-E3.md", "name"),
    (r"\bMETHOD-P9\b", "spikes/METHOD-P9.md", "name"),
    (r"\bMETHOD-P\b", "spikes/METHOD-P.md", "name"),
    (r"\bMETHOD-S\b", "spikes/METHOD-S.md", "name"),
    (r"\bMETHOD-H\b", "spikes/METHOD-H.md", "name"),
    (r"\bMETHOD\b(?!-)", "spikes/METHOD.md", "name"),
    # spike ids cited with a section => the section of METHOD-P / METHOD-P9 / METHOD-S that
    # pre-registers that spike (observed usage: "P8 s5.4(3)", "P2 s2.4's", "P1 s1.4")
    (r"\bP[12678]\b", "spikes/METHOD-P.md", "spike-id"),
    (r"\bP9b?\b", "spikes/METHOD-P9.md", "spike-id"),
    (r"\bS-(?:VAR|TYPED|BIN|MIG|H1b|H1c|WT)\b", "spikes/METHOD-S.md", "spike-id"),
    (r"\bE3H\b", "spikes/METHOD-E3H.md", "spike-id"),
    (r"\bE3\b", "spikes/METHOD-E3.md", "spike-id"),
    # finding drafts
    (r"\bF(29|3\d|40)(?:-DRAFT(?:\.md)?)?\b", lambda m: "spikes/F%s-DRAFT.md" % m.group(1), "finding"),
    (r"\bFINDINGS(?:\.md)?\b", "spikes/FINDINGS.md", "name"),
    (r"\bHANDOFF(?:\.md)?\b", "spikes/HANDOFF.md", "name"),
    (r"\bPRIOR-ART(?:\.md)?\b", "spikes/PRIOR-ART.md", "name"),
    (r"\bDECISIONS-2026-09-09(?:\.md)?\b", "spikes/DECISIONS-2026-09-09.md", "name"),
    (r"\bDECISIONS-2026-09-08(?:\.md)?\b", "spikes/p9/DECISIONS-2026-09-08.md", "name"),
    # references/
    (r"(?i)\bprocess[- ]catalog[- ]proposal\b", "references/2026-09-07-process-catalog-proposal.md", "name"),
    (r"(?i)\bcatalog proposal\b", "references/2026-09-07-process-catalog-proposal.md", "name"),
    (r"(?i)\bcatalog\b", "references/2026-09-07-process-catalog-proposal.md", "name"),
    (r"(?i)\barchitecture[- ]proposal\b", "references/2026-09-05-architecture-proposal.md", "name"),
    (r"(?i)\bproposal\b", "references/2026-09-05-architecture-proposal.md", "name"),
    (r"(?i)\bactivity[- ]graph[- ]schema\b", "references/2026-09-07-activity-graph-schema.md", "name"),
    (r"(?i)\bgraph schema\b", "references/2026-09-07-activity-graph-schema.md", "name"),
    (r"(?i)\bschema\b", "references/2026-09-07-activity-graph-schema.md", "name-weak"),
    (r"(?i)\bevidence[- ]brief\b", "references/2026-09-08-evidence-brief.md", "name"),
    (r"(?i)\bbrief\b", "references/2026-09-08-evidence-brief.md", "name"),
    (r"(?i)\bevidence[- ]augmentation\b", "references/2026-09-08-evidence-augmentation.md", "name"),
    (r"(?i)\baugmentation\b", "references/2026-09-08-evidence-augmentation.md", "name"),
    (r"(?i)\bresearch pass\b", "references/2026-09-08-evidence-augmentation.md", "name"),
    (r"(?i)\bspike[- ]charter\b", "references/2026-08-31-rust-inversion-spike-charter.md", "name"),
    (r"(?i)\bcharter\b", "references/2026-08-31-rust-inversion-spike-charter.md", "name"),
    (r"(?i)\brfc[- ]amendment\b", "references/2026-09-02-rfc-amendment.md", "name"),
    (r"(?i)\bamendment\b", "references/2026-09-02-rfc-amendment.md", "name-weak"),
    (r"\bRFC\b(?!\s?\d)", "references/2026-08-31-rust-inversion-RFC.md", "name"),
    (r"(?i)\brunbook\b", "RUNBOOK.md", "name"),
    (r"(?i)\bacceptance[- ]criteria\b", "references/2026-09-08-acceptance-criteria.md", "name"),
    (r"(?i)\b(?:AC|criteria) register\b", "references/2026-09-08-acceptance-criteria.md", "name"),
    (r"(?i)\bsuccess[- ]measures\b", "references/2026-09-08-success-measures.md", "name"),
    (r"(?i)\b(?:SM|measures|measurement) register\b", "references/2026-09-08-success-measures.md", "name"),
    (r"(?i)\barchitecture[- ]design\b", "references/2026-09-08-architecture-design.md", "name"),
    (r"(?i)\bdesign\b(?![- ]recommendation)", "references/2026-09-08-architecture-design.md", "name-weak"),
    (r"(?i)\bdesign[- ]recommendation\b", "references/2026-09-07-design-recommendation.md", "name"),
    (r"(?i)\bchange[- ]classification\b", "references/2026-09-09-change-classification.md", "name"),
    (r"(?i)\breimagining\b|\bbrainstorm\b", "references/2026-09-03-reimagining-brainstorm.md", "name"),
    (r"(?i)\bfindings[- ]for[- ]product\b", "references/2026-09-04-findings-for-product.md", "name"),
    (r"(?i)\bshipmates[- ]lessons\b", "references/2026-09-01-shipmates-lessons-for-redkiln-rs.md", "name"),
    (r"(?i)\bmethod[- ]engineering\b", "references/research/2026-09-07-method-engineering.md", "name"),
    (r"(?i)\bcompetitive(?:[- ]workflows)?\b", "references/research/2026-09-07-competitive-workflows.md", "name"),
    (r"(?i)\bselection(?:[- ]and[- ]planning)?\b", "references/research/2026-09-07-selection-and-planning.md", "name-weak"),
    (r"(?i)\bactivity[- ]catalog[- ]raw\b", "references/research/2026-09-07-activity-catalog-raw.md", "name"),
    (r"(?i)\bworkflow[- ]model\b", "references/research/2026-09-03-workflow-model.md", "name"),
    (r"(?i)\blandscape[- ]sentiment\b", "references/research/2026-09-03-landscape-sentiment.md", "name"),
    (r"(?i)\bgithub[- ]platform\b", "references/research/2026-09-03-github-platform.md", "name"),
    (r"(?i)\bgrounding[- ]s[- ]results\b", "references/research/2026-09-06-grounding-s-results.md", "name"),
    (r"(?i)\bverification\.md\b", "references/research/2026-09-03-verification.md", "name"),
    (r"(?i)\barchon\b", "references/research/2026-09-03-archon.md", "name"),
    (r"\bADR-?(\d{4})\b", lambda m: adr_path(m.group(1)), "adr"),
    (r"\bREADME(?:\.md)?\b", "references/adr/README.md", "name-weak"),
    # externals (documents not in this repo)
    (r"\bSWEBOK\b(?:\s+v?\d+)?(?:\s+ch\.?\s?\d+)?", "EXTERNAL:SWEBOK", "external"),
    (r"(?i)\bch\.\s?\d+", "EXTERNAL:chapter", "external"),
    (r"\bRFC\s?\d{3,5}\b", "EXTERNAL:IETF-RFC", "external"),
    (r"(?i)\b(?:OMG|Essence|SPEM|ISO|IEEE|W3C|MADR|SEMAT)\b", "EXTERNAL:standard", "external"),
    (r"(?i)\bspec\b(?!\.md)", "EXTERNAL:spec", "external-weak"),
]

ADR_FILES = sorted(p for p in ALL_FILES if p.startswith("references/adr/") and p.endswith(".md"))


def adr_path(num):
    for p in ADR_FILES:
        if p.startswith("references/adr/%s-" % num):
            return p
    return "references/adr/%s-<MISSING>.md" % num


ALIAS_COMPILED = [(re.compile(rx), tgt, kind) for rx, tgt, kind in ALIASES]

# right-hand naming: "s7 of the RFC", "s3 in METHOD-P"
RIGHT_NAMING_RE = re.compile(r"^[\s'\u2019s]*(?:\)|\])?\s*(?:of|in)\s+(?:the\s+|this\s+)?")


def find_alias(window, allow_kinds=None):
    """Return (target, kind, alias_text, end_pos) for the nearest-preceding alias in window, or None."""
    best = None
    for rx, tgt, kind in ALIAS_COMPILED:
        for m in rx.finditer(window):
            end = m.end()
            length = end - m.start()
            key = (end, length)
            if best is None or key > best[0]:
                t = tgt(m) if callable(tgt) else tgt
                best = (key, t, kind, m.group(0))
    if best is None:
        return None
    return best[1], best[2], best[3], best[0][0]


# ----------------------------------------------------------------------------------------------
# 3. (a) section references
# ----------------------------------------------------------------------------------------------

# numeric / lettered labels:  4.5  4.2.1  9a  4b.2  0  C  C.1  E3
NUM_TOKEN = r"\d+[a-z]?(?:\.\d+[a-z]?)*"
SECT_TOKEN = r"(" + NUM_TOKEN + r"|[A-Z]\d+(?![A-Za-z])|[A-Z](?:\.\d+)?(?![A-Za-z0-9]))"
SECT_RE = re.compile(
    SS + r"(" + SS + r")?\s?" + SECT_TOKEN + r"(\((\d+)\))?"                          # s / ss N[.M][(k)]
    + r"(?:\s?[–—-]\s?" + SS + r"?(" + NUM_TOKEN + r"))?"                       # optional range end
)
# heading-text form:  sConsequences  sLimits  sUnrun
HEAD_TEXT_RE = re.compile(SS + r"\s?([A-Z][a-z]+(?:[ -][A-Z][a-z]+)?)(?![a-z])")
# spelled-out: "section 4.5", "Section 15", "sections 3 and 4"
WORD_SECT_RE = re.compile(r"\b([Ss]ections?)\s+" + SECT_TOKEN + r"(\((\d+)\))?(?![\w-])")

CONTEXT_CHARS = 90
NEAR_CHARS = 40
BRIEF = "references/2026-09-08-evidence-brief.md"
LETTERED_LABEL_RE = re.compile(r"^[A-E](?:\.?\d+)?$")

FRONTMATTER_SOURCE = {}   # doc -> [docs named in YAML 'source:' line]
for doc in CORPUS:
    lines = DOC_LINES[doc]
    if lines and lines[0].strip() == "---":
        for ln in lines[1:40]:
            if ln.strip() == "---":
                break
            if ln.startswith("source:"):
                FRONTMATTER_SOURCE[doc] = [t for t in re.findall(r"(?:references|spikes)/[\w./\-]+\.md|RUNBOOK\.md", ln)]
                break


def resolve_label(doc, label):
    """Return (status, lines) with status in exact|parent_only|brief_list_item|none|target_not_in_corpus."""
    if doc not in LABEL_MAP:
        return "target_not_in_corpus", []
    lm = LABEL_MAP[doc]
    if doc == BRIEF and re.match(r"^[A-E]\d+$", label):
        label = label[0] + "." + label[1:]      # sC12 spelling of the brief's C.12
    if label in lm:
        return "exact", lm[label]
    if doc == BRIEF and re.match(r"^[A-E]\.\d+$", label):
        # lettered-section list items (C.n etc.) -- verified against the item list built in step 4
        return "brief_list_item", lm.get(label[0], [])
    parts = label.split(".")
    while len(parts) > 1:
        parts = parts[:-1]
        p = ".".join(parts)
        if p in lm:
            return "parent_only", lm[p]
    return "none", []


def resolve_heading_text(doc, text):
    if doc not in HEADINGS:
        return "target_not_in_corpus", []
    t = text.lower()
    hits = [h["line"] for h in HEADINGS[doc] if h["text"].lower().startswith(t)]
    if hits:
        return "exact", hits
    lead = re.compile(r"^\**" + re.escape(text) + r"\**\s*[:.]", re.I)
    hits = [i for i, ln in enumerate(DOC_LINES[doc], 1) if lead.match(ln)]
    return ("lead_in_paragraph", hits) if hits else ("none", [])


def aliases_in(window):
    """All (target, kind, text, end) alias hits in a window, nearest-to-end first (ties: longest)."""
    hits = []
    for rx, tgt, kind in ALIAS_COMPILED:
        for m in rx.finditer(window):
            t = tgt(m) if callable(tgt) else tgt
            hits.append(((m.end(), m.end() - m.start()), t, kind, m.group(0)))
    hits.sort(key=lambda h: h[0], reverse=True)
    out, seen = [], set()
    for key, t, kind, text in hits:
        if t in seen:
            continue
        seen.add(t)
        out.append((t, kind, text, key[0]))
    return out


HIT_STATUSES = ("exact", "parent_only", "brief_list_item", "lead_in_paragraph")
STRONG_STATUSES = ("exact", "brief_list_item", "lead_in_paragraph")


def enclosing_heading_line(doc, lineno):
    best = None
    for h in HEADINGS[doc]:
        if h["line"] < lineno:
            best = h["line"]
        else:
            break
    return best


def _status(target, label, is_text_label):
    if target.startswith("EXTERNAL:"):
        return "external", []
    if is_text_label:
        return resolve_heading_text(target, label)
    return resolve_label(target, label)


def resolve_section_ref(doc, lineno, ln, label, is_text_label, m_start, m_end):
    """Rule chain, run twice: first accepting only strong hits (exact heading / brief item / lead-in),
    then accepting parent_only hits. Returns dict describing the resolution."""
    lines = DOC_LINES[doc]
    left = ln[:m_start]
    right = ln[m_end:m_end + 80]
    res = {"nearest_alias_target": None, "nearest_alias_kind": None, "nearest_alias_text": None,
           "nearest_alias_distance_chars": None, "nearest_alias_has_label": None,
           "names_document": False, "resolution_rule": None, "confidence": None,
           "target": None, "status": None, "target_lines": [], "resolution_pass": None}

    left_hits = aliases_in(left)
    # nearest alias on the line, for the record. Weak aliases (common words) only count when they
    # actually carry the label; otherwise fall through to the next-nearest alias.
    rec = None
    for t, k, txt, end in left_hits:
        st = _status(t, label, is_text_label)[0]
        ok = st in HIT_STATUSES or st == "external"
        if (k.endswith("-weak") or k in ("spike-id", "adr")) and not ok:
            continue
        rec = (t, k, txt, end, ok)
        break
    if rec:
        t, k, txt, end, ok = rec
        dist = len(left) - end
        res.update({"nearest_alias_target": t, "nearest_alias_kind": k, "nearest_alias_text": txt,
                    "nearest_alias_distance_chars": dist, "nearest_alias_has_label": ok})
        res["names_document"] = dist <= NEAR_CHARS

    right_first = None
    rm = RIGHT_NAMING_RE.match(right)
    if rm:
        rh = aliases_in(right[rm.end():rm.end() + 60])
        if rh:
            first = min(rh, key=lambda h: h[3])
            if first[3] <= 45:
                right_first = first[0]
                res["names_document"] = True
                if res["nearest_alias_target"] is None:
                    res.update({"nearest_alias_target": right_first, "nearest_alias_kind": first[1],
                                "nearest_alias_text": first[2], "nearest_alias_distance_chars": 0,
                                "nearest_alias_has_label":
                                    _status(right_first, label, is_text_label)[0] in HIT_STATUSES})

    back, j = [], lineno - 2
    while j >= 0 and len(back) < 3:
        if lines[j].strip():
            back.append(lines[j])
        j -= 1
    hl = enclosing_heading_line(doc, lineno)
    near_alias_lacked = bool(rec) and (len(left) - rec[3]) <= NEAR_CHARS and not rec[4]

    def chain(accept):
        def try_doc(target, rule, conf):
            st, tl = _status(target, label, is_text_label)
            if st in accept or st == "external":
                res.update({"resolution_rule": rule, "confidence": conf, "target": target,
                            "status": st, "target_lines": tl})
                return True
            return False
        if right_first and try_doc(right_first, "right_of_the_X", "high"):
            return True
        if left_hits and (len(left) - left_hits[0][3]) <= NEAR_CHARS:
            if try_doc(left_hits[0][0], "near_alias", "high"):
                return True
        for t, k, txt, end in left_hits:
            if t.startswith("EXTERNAL:"):
                continue
            if try_doc(t, "line_alias", "medium"):
                return True
        if LETTERED_LABEL_RE.match(label) and not is_text_label and doc != BRIEF:
            if try_doc(BRIEF, "lettered_label_brief", "medium"):
                return True
        for t in FRONTMATTER_SOURCE.get(doc, []):
            if try_doc(t, "frontmatter_source", "medium"):
                return True
        if try_doc(doc, "self", "low" if near_alias_lacked else "medium"):
            return True
        for bl in back:
            for t, k, txt, end in aliases_in(bl):
                if not t.startswith("EXTERNAL:") and try_doc(t, "preceding_lines", "low"):
                    return True
        if hl:
            for t, k, txt, end in aliases_in(lines[hl - 1]):
                if not t.startswith("EXTERNAL:") and try_doc(t, "enclosing_heading", "low"):
                    return True
        for hdr in lines[:12]:
            for t, k, txt, end in aliases_in(hdr):
                if not t.startswith("EXTERNAL:") and try_doc(t, "file_header", "low"):
                    return True
        # last resort: the label is specific (dotted or letter-suffixed) and exactly one corpus
        # document carries it as an exact heading
        if not is_text_label and ("." in label or re.search(r"[a-z]", label)):
            uniq = [d for d in CORPUS if resolve_label(d, label)[0] == "exact"]
            if len(uniq) == 1 and try_doc(uniq[0], "unique_label_in_corpus", "low"):
                return True
        return False

    if chain(STRONG_STATUSES):
        res["resolution_pass"] = "strong"
        return res
    if chain(HIT_STATUSES):
        res["resolution_pass"] = "parent_only"
        return res
    res.update({"resolution_rule": "none", "confidence": None,
                "target": (res["nearest_alias_target"] if res["names_document"] else doc),
                "status": "none", "target_lines": []})
    return res


def candidates_anywhere(label, is_text_label):
    if is_text_label or ("." not in label and not re.search(r"[a-z]", label)):
        return None
    hits = [d for d in CORPUS if resolve_label(d, label)[0] == "exact"]
    return {"count": len(hits), "docs": hits[:6]}


section_refs = []
for doc in CORPUS:
    lines = DOC_LINES[doc]
    fm = FENCED[doc]
    for i, ln in enumerate(lines, 1):
        taken = []
        for form, rx in (("sign", SECT_RE), ("word", WORD_SECT_RE), ("heading-text", HEAD_TEXT_RE)):
            for m in rx.finditer(ln):
                if any(a <= m.start() < b for a, b in taken):
                    continue
                taken.append((m.start(), m.end()))
                is_text = form == "heading-text"
                if form == "sign":
                    double, label, subitem, range_end = bool(m.group(1)), m.group(2), m.group(4), m.group(5)
                elif form == "word":
                    double, label, subitem, range_end = m.group(1).lower() == "sections", m.group(2), m.group(4), None
                else:
                    double, label, subitem, range_end = False, m.group(1), None, None
                r = resolve_section_ref(doc, i, ln, label, is_text, m.start(), m.end())
                entry = {
                    "file": doc, "line": i, "col": m.start() + 1,
                    "raw": m.group(0), "form": form, "label": label,
                    "subitem": subitem, "range_end": range_end, "double_sign": double,
                    "in_code_fence": fm[i - 1],
                }
                entry.update(r)
                if r["status"] == "none":
                    entry["candidates_anywhere"] = candidates_anywhere(label, is_text)
                entry["context"] = ln[max(0, m.start() - CONTEXT_CHARS): m.end() + 40].strip()
                section_refs.append(entry)

# the mis-citation candidate list: the sentence names a document (alias within NEAR_CHARS) that LACKS the label
named_doc_lacks_heading = [r for r in section_refs
                           if r["names_document"] and r["nearest_alias_has_label"] is False
                           and r["nearest_alias_target"] and not r["nearest_alias_target"].startswith("EXTERNAL:")]

# ----------------------------------------------------------------------------------------------
# 4. (b) evidence-brief C.n item references
# ----------------------------------------------------------------------------------------------

brief_lines = DOC_LINES[BRIEF]
c_head = LABEL_MAP[BRIEF].get("C", [])
d_head = LABEL_MAP[BRIEF].get("D", [])
assert len(c_head) == 1 and len(d_head) == 1, "brief must have exactly one C. and one D. heading"
C_ITEMS = []
ITEM_RE = re.compile(r"^(\d+)\.\s+\*\*(.+?)(?:\*\*|$)")
for i in range(c_head[0] + 1, d_head[0]):
    m = ITEM_RE.match(brief_lines[i - 1])
    if m:
        C_ITEMS.append({"n": int(m.group(1)), "line": i, "title": m.group(2)})
assert [x["n"] for x in C_ITEMS] == list(range(1, len(C_ITEMS) + 1)), "C items must be 1..N contiguous"
C_MAX = len(C_ITEMS)
C_TITLE = {x["n"]: x["title"] for x in C_ITEMS}

# crude keyword sets per item, from the titles, used only to flag CANDIDATE mismatches [inferred]
C_KEYWORDS = {
    1: ["cache", "split rule", "system-prompt", "subsidy"],
    2: ["gate cost", "mid-graph gate", "frequency"],
    3: ["drift", "reconcile()"],
    4: ["resume", "lossless"],
    5: ["structured output", "json-schema", "structuredoutput"],
    6: ["process survival", "process surviving", "durable mark"],
    7: ["neither gate", "high-precision", "low-recall", "add findings"],
    8: ["relaxation", "narrowed-pin", "advisory sink"],
    9: ["selection input", "unstable", "modal", "required-artifact"],
    10: ["policy predicate", "constant function", "monotone", "bisect"],
    11: ["isolation", "measurement precondition", "mcp server", "system/init"],
    12: ["method finding", "saturated", "success rate"],
    13: ["derived", "rescue", "lexical", "auth"],
    14: ["derivation", "trace", "unsound", "refinement"],
}

C_REF_RE = re.compile(
    r"(?:" + SS + r"\s?C\s?\.?\s?(\d+)\b"          # sC.n  /  sCn  /  s C.n
    r"|(?<![\w./])C\.(\d+)\b)"                      # bare C.n
)
PRINCIPLE_RE = re.compile(r"(?i)\bprinciple\s+(\d+)\b")

c_refs = []
principle_mentions = []
for doc in CORPUS:
    lines = DOC_LINES[doc]
    fm = FENCED[doc]
    for i, ln in enumerate(lines, 1):
        for m in C_REF_RE.finditer(ln):
            n = int(m.group(1) or m.group(2))
            raw = m.group(0)
            spelling = ("sign-dot" if raw.startswith(SS) and "." in raw
                        else "sign-nodot" if raw.startswith(SS) else "bare-dot")
            left = ln[:m.start()]
            hit = find_alias(left)
            names_brief = bool(hit) and hit[0] == BRIEF
            other_doc_named = hit[0] if (hit and hit[0] != BRIEF and not hit[0].startswith("EXTERNAL")) else None
            status = "resolved" if 1 <= n <= C_MAX else "past_end"
            title = C_TITLE.get(n)
            ctx = ln[max(0, m.start() - 160): m.end() + 120].lower()
            kw_hits = sorted(k for k, kws in C_KEYWORDS.items() if any(w in ctx for w in kws))
            mismatch_candidate = bool(kw_hits) and (n not in kw_hits) and status == "resolved"
            c_refs.append({
                "file": doc, "line": i, "col": m.start() + 1, "raw": raw, "spelling": spelling,
                "n": n, "status": status, "item_title": title,
                "item_line_in_brief": next((x["line"] for x in C_ITEMS if x["n"] == n), None),
                "names_brief_on_line": names_brief,
                "nearest_named_doc_if_not_brief": other_doc_named,
                "in_code_fence": fm[i - 1],
                "keyword_items_in_context": kw_hits,
                "keyword_mismatch_candidate": mismatch_candidate,
                "context": ln[max(0, m.start() - CONTEXT_CHARS): m.end() + 60].strip(),
            })
        for m in PRINCIPLE_RE.finditer(ln):
            principle_mentions.append({
                "file": doc, "line": i, "col": m.start() + 1, "raw": m.group(0), "n": int(m.group(1)),
                "context": ln[max(0, m.start() - 70): m.end() + 70].strip(),
            })

# ----------------------------------------------------------------------------------------------
# 5. (d) links and URLs  (done before paths so URL spans can be masked out of path matching)
# ----------------------------------------------------------------------------------------------

URL_RE = re.compile(r"(?:https?|ftp)://[^\s<>\"'`)\]]+")
MDLINK_RE = re.compile(r"!?\[([^\]]*)\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
AUTOLINK_RE = re.compile(r"<((?:https?|ftp)://[^>\s]+)>")
REFDEF_RE = re.compile(r"^\s{0,3}\[([^\]]+)\]:\s+(\S+)")

TRAIL_PUNCT = ".,;:!?)]}'\"*_"


def strip_trail(u):
    while u and u[-1] in TRAIL_PUNCT:
        # keep balanced closing paren if there is an opening one
        if u[-1] == ")" and u.count("(") >= u.count(")"):
            break
        u = u[:-1]
    return u


md_links = []
urls = []
url_spans = defaultdict(list)   # (doc,line) -> [(s,e)]
for doc in CORPUS:
    lines = DOC_LINES[doc]
    fm = FENCED[doc]
    for i, ln in enumerate(lines, 1):
        for m in MDLINK_RE.finditer(ln):
            text, target = m.group(1), m.group(2)
            if target.startswith("#"):
                kind = "anchor"
            elif re.match(r"^[a-z][a-z0-9+.-]*:", target, re.I):
                kind = "url"
            else:
                kind = "relative"
            md_links.append({"file": doc, "line": i, "col": m.start() + 1, "text": text,
                             "target": target, "kind": kind, "image": m.group(0).startswith("!"),
                             "in_code_fence": fm[i - 1]})
        for m in REFDEF_RE.finditer(ln):
            md_links.append({"file": doc, "line": i, "col": m.start() + 1, "text": m.group(1),
                             "target": m.group(2), "kind": "refdef", "image": False,
                             "in_code_fence": fm[i - 1]})
        for m in URL_RE.finditer(ln):
            u = strip_trail(m.group(0))
            urls.append({"file": doc, "line": i, "col": m.start() + 1, "url": u,
                         "in_code_fence": fm[i - 1]})
            url_spans[(doc, i)].append((m.start(), m.start() + len(u)))

# ----------------------------------------------------------------------------------------------
# 6. (c) file-path references
# ----------------------------------------------------------------------------------------------

PATH_EXT = r"(?:md|py|json|jsonl|toml|ps1|rs)"
PATH_RE = re.compile(
    r"(?<![A-Za-z0-9_@\-#*])"                                # not glued to a preceding token / glob
    r"((?:\.{1,2}/)?(?:[A-Za-z0-9_.\-]+/)*[A-Za-z0-9_\-][A-Za-z0-9_.\-]*\." + PATH_EXT + r")"
    r"(?![A-Za-z0-9_/\-])"
)
GLOB_RE = re.compile(r"[A-Za-z0-9_./\-]*[*?#][A-Za-z0-9_./\-*?#]*\." + PATH_EXT + r"\b")
DRIVE_RE = re.compile(r"[A-Za-z]:[/\\]")


def mask(line, spans):
    if not spans:
        return line
    chars = list(line)
    for s, e in spans:
        for k in range(s, min(e, len(chars))):
            chars[k] = " "
    return "".join(chars)


def norm_path(p):
    p = p.replace("\\", "/")
    while p.startswith("./"):
        p = p[2:]
    return p


HOSTNAME_LOOKALIKES = {"docs.rs", "crates.io", "lib.rs"}


def resolve_path(p, citing_doc):
    """Return dict with resolution info."""
    p = norm_path(p)
    if p in HOSTNAME_LOOKALIKES:
        return {"status": "not-a-path", "resolved_as": None, "how": "hostname-lookalike"}
    citing_dir = citing_doc.rsplit("/", 1)[0] if "/" in citing_doc else ""
    if p in ALL_FILES_SET:
        return {"status": "exists", "resolved_as": p, "how": "repo-relative"}
    if citing_dir:
        cand = os.path.normpath(os.path.join(citing_dir, p)).replace("\\", "/")
        if cand in ALL_FILES_SET:
            return {"status": "exists", "resolved_as": cand, "how": "relative-to-citing-dir"}
    else:
        cand = os.path.normpath(p).replace("\\", "/")
        if cand in ALL_FILES_SET:
            return {"status": "exists", "resolved_as": cand, "how": "normalised"}
    base = p.rsplit("/", 1)[-1]
    elsewhere = BASENAME_INDEX.get(base, [])
    non_fixture = [x for x in elsewhere if not x.startswith("spikes/fixtures/")]
    if "/" not in p and len(non_fixture) == 1:
        return {"status": "basename-unique", "resolved_as": non_fixture[0],
                "how": "bare-basename-unique-outside-fixtures"}
    fixture_only = bool(elsewhere) and all(x.startswith("spikes/fixtures/") for x in elsewhere)
    if elsewhere:
        return {"status": "missing", "resolved_as": None,
                "how": "basename-exists-elsewhere" + ("-fixtures-only" if fixture_only else ""),
                "elsewhere": elsewhere[:5], "elsewhere_count": len(elsewhere)}
    # does the directory exist at least?
    d = p.rsplit("/", 1)[0] if "/" in p else None
    return {"status": "missing", "resolved_as": None,
            "how": "nothing-by-that-name",
            "parent_dir_exists": (d in ALL_DIRS_SET) if d else None}


def citing_class(doc):
    if doc.startswith("spikes/results/") or doc.startswith("spikes/remediation/") or doc.startswith("spikes/p9/"):
        return "results-data"
    if doc.startswith("spikes/"):
        return "spike-method"
    return "planning-doc"


path_refs = []
glob_refs = []
drive_refs = []
for doc in CORPUS:
    lines = DOC_LINES[doc]
    fm = FENCED[doc]
    for i, ln in enumerate(lines, 1):
        masked = mask(ln, url_spans.get((doc, i), []))
        for m in GLOB_RE.finditer(masked):
            glob_refs.append({"file": doc, "line": i, "col": m.start() + 1, "raw": m.group(0),
                              "in_code_fence": fm[i - 1]})
        for m in PATH_RE.finditer(masked):
            raw = m.group(1)
            pre = masked[max(0, m.start() - 3):m.start()]
            absolute = bool(DRIVE_RE.search(pre)) or pre.endswith("/") or pre.endswith("\\")
            if absolute:
                # reconstruct the absolute token for the record
                s = m.start()
                while s > 0 and masked[s - 1] not in " \t`'\"(<[":
                    s -= 1
                drive_refs.append({"file": doc, "line": i, "col": s + 1, "raw": masked[s:m.end()],
                                   "in_code_fence": fm[i - 1]})
                continue
            res = resolve_path(raw, doc)
            entry = {"file": doc, "line": i, "col": m.start() + 1, "raw": raw,
                     "citing_class": citing_class(doc),
                     "has_dir_component": "/" in raw,
                     "prefix_bucket": (norm_path(raw).split("/")[0] if "/" in norm_path(raw) else "<bare>"),
                     "in_code_fence": fm[i - 1]}
            entry.update(res)
            entry["context"] = ln[max(0, m.start() - 70): m.end() + 40].strip()
            path_refs.append(entry)

# relative markdown-link targets are also file-path citations: resolve them
link_path_refs = []
for L in md_links:
    if L["kind"] == "relative":
        tgt = L["target"].split("#", 1)[0]
        if not tgt:
            continue
        res = resolve_path(tgt, L["file"])
        e = dict(L)
        e.update(res)
        link_path_refs.append(e)

# ----------------------------------------------------------------------------------------------
# 7. anchor assertions (a silent zero-match is not a result)
# ----------------------------------------------------------------------------------------------

assertions = []


def check(name, cond, detail=""):
    assertions.append({"name": name, "ok": bool(cond), "detail": detail})
    if not cond:
        print("ASSERTION FAILED: %s %s" % (name, detail))


check("corpus_size_194", len(CORPUS) == 194, "corpus=%d" % len(CORPUS))
check("arch_design_has_4.5", "4.5" in LABEL_MAP["references/2026-09-08-architecture-design.md"])
check("arch_design_has_3.6.1", "3.6.1" in LABEL_MAP["references/2026-09-08-architecture-design.md"])
check("schema_has_9a", "9a" in LABEL_MAP["references/2026-09-07-activity-graph-schema.md"])
check("method_s_has_4b.2", "4b.2" in LABEL_MAP["spikes/METHOD-S.md"])
check("runbook_step_0_1", "0.1" in LABEL_MAP["RUNBOOK.md"] and "5" in LABEL_MAP["RUNBOOK.md"])
check("runbook_fenced_hash_excluded",
      not any(h["line"] in (56, 57, 169) for h in HEADINGS["RUNBOOK.md"]),
      "lines 56/57/169 are inside bash fences")
check("brief_C_items_14", C_MAX == 14, "C_MAX=%d" % C_MAX)
check("brief_C12_is_method_finding", C_TITLE.get(12, "").startswith("Method finding"), C_TITLE.get(12))
ac = "references/2026-09-08-acceptance-criteria.md"
check("ac_4460_is_correction_line",
      "C.12" in DOC_LINES[ac][4459] and "Principle 11" in DOC_LINES[ac][4459])
n_sign = sum(1 for r in section_refs if r["form"] == "sign")
n_word = sum(1 for r in section_refs if r["form"] == "word")
check("section_sign_refs_nonzero", n_sign > 1000, "sign=%d" % n_sign)
check("word_section_refs_reported", True, "word-form refs=%d (0 is a legitimate value)" % n_word)
check("heading_text_refs_reported", True, "heading-text refs=%d (0 is a legitimate value)" % sum(1 for r in section_refs if r["form"] == "heading-text"))
check("adr_frontmatter_sources_found", len(FRONTMATTER_SOURCE) >= 80, "docs with frontmatter source=%d" % len(FRONTMATTER_SOURCE))
check("no_single_letter_label_glued_to_word", not any(r["form"] == "sign" and re.match(r"^[A-Z]$", r["label"]) and re.match(r"[a-z]", r["context"][r["context"].find(r["raw"]) + len(r["raw"]):][:1] or "") for r in section_refs))
check("c_refs_nonzero", len(c_refs) > 100, "c_refs=%d" % len(c_refs))
check("c_refs_three_spellings",
      len({r["spelling"] for r in c_refs}) == 3, sorted({r["spelling"] for r in c_refs}))
missing_raw = {r["raw"] for r in path_refs if r["status"] == "missing"}
for known in ("2026-08-08-scorecard.md", "2026-08-09-scorecard.md", "spikes/FINDINGS-DRAFT-F30.md"):
    check("known_missing_" + known, any(known in r for r in missing_raw), known)
check("known_missing_competitor_analyses",
      any("references/competitor-analyses/shipmates/" in r for r in missing_raw))
check("paths_nonzero", len(path_refs) > 500, "paths=%d" % len(path_refs))
check("urls_nonzero", len(urls) > 50, "urls=%d" % len(urls))
check("mdlinks_reported", True, "md_links=%d (0 is a legitimate value)" % len(md_links))

# ----------------------------------------------------------------------------------------------
# 8. assemble and write
# ----------------------------------------------------------------------------------------------


def split(rows, ok_statuses):
    good = [r for r in rows if r["status"] in ok_statuses]
    bad = [r for r in rows if r["status"] not in ok_statuses]
    return good, bad


sec_external = [r for r in section_refs if r["status"] == "external"]
sec_inrepo = [r for r in section_refs if r["status"] != "external"]
sec_resolved, sec_dangling = split(sec_inrepo, ("exact", "parent_only", "brief_list_item", "lead_in_paragraph"))

c_resolved, c_dangling = split(c_refs, ("resolved",))
p_resolved, p_dangling = split(path_refs, ("exists",))
p_basename_unique = [r for r in path_refs if r["status"] == "basename-unique"]
p_not_path = [r for r in path_refs if r["status"] == "not-a-path"]
p_dangling = [r for r in p_dangling if r["status"] == "missing"]
lp_resolved, lp_dangling = split(link_path_refs, ("exists",))


def counter(rows, key):
    return {str(k): v for k, v in sorted(Counter(r[key] for r in rows).items(), key=lambda kv: (-kv[1], str(kv[0])))}


def per_file(rows):
    return counter(rows, "file")


heading_index = {doc: [[h["text"], h["line"], h["level"], h["labels"]] for h in HEADINGS[doc]]
                 for doc in CORPUS}

out = {
    "meta": {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "repo": REPO,
        "script": os.path.abspath(__file__).replace("\\", "/"),
        "corpus_rule": "every *.md under the repo except .git/** and spikes/fixtures/**",
        "corpus_files": len(CORPUS),
        "repo_files_indexed_for_existence": len(ALL_FILES),
        "heading_rule": "ATX headings # .. ###### outside fenced code blocks; setext headings NOT indexed",
        "label_rule": "numeric prefix (4.5, 4.2.1, 9a, 4b.2), 'Step N'/'Checkpoint N'/'Phase N' (as N and 'Step N'), "
                      "single letter 'C.', alphanumeric 'E3'",
        "section_ref_rule": "U+00A7 forms incl. double sign, ranges, (k) sub-items; spelled 'section N.M'; "
                            "document inferred from nearest-preceding alias on the same line (or 'of the X' "
                            "immediately after); no alias => self-reference to the citing document",
        "section_resolution_rules": "right_of_the_X > near_alias(<=40 chars) > line_alias > lettered_label_brief > frontmatter_source > self > preceding_lines > enclosing_heading > file_header > unique_label_in_corpus > none; the chain runs twice: pass 1 accepts only exact/brief_list_item/lead_in_paragraph hits, pass 2 also accepts parent_only; confidence high/medium/low records which tier fired; nearest_alias_* fields always record what the sentence literally names nearest",
        "section_status_values": {
            "exact": "target document has a heading with that label",
            "parent_only": "target has the parent label (e.g. 5.4 for 5.4(3) or 9a for 9a.2) but not the full one",
            "brief_list_item": "evidence-brief lettered section list item (C.n / D.n / E.n), not a heading",
            "lead_in_paragraph": "heading-text form (sContested) matched a bold/colon paragraph lead-in rather than a heading",
            "none": "no heading with that label (or its parents) in the inferred target",
            "target_not_in_corpus": "inferred target document is not in the corpus (e.g. missing ADR)",
            "external": "names a document outside this repo (SWEBOK, IETF RFC, OMG spec, chapter refs)",
        },
        "alias_kinds": {"path": "explicit repo path", "name": "document nickname", "name-weak": "common word used as "
                        "a nickname (schema/design/selection/amendment) -- lower confidence", "spike-id": "P1/P2/P7/P8/"
                        "S-x/E3 cited with a section => the pre-registration that defines that spike", "finding":
                        "F29..F40 => spikes/F##-DRAFT.md", "adr": "ADR-nnnn", "external": "outside the repo",
                        "self": "no alias found; the citing document itself"},
        "alias_table": [{"regex": rx, "target": (tgt if isinstance(tgt, str) else "<computed>"), "kind": k}
                        for rx, tgt, k in ALIASES],
        "assertions": assertions,
    },
    "heading_index": heading_index,
    "section_refs": {
        "counts": {
            "total": len(section_refs), "sign_form": n_sign, "word_form": n_word,
            "heading_text_form": sum(1 for r in section_refs if r["form"] == "heading-text"),
            "in_repo": len(sec_inrepo), "resolved": len(sec_resolved), "dangling": len(sec_dangling),
            "external": len(sec_external),
            "by_status": counter(section_refs, "status"),
            "by_resolution_rule": counter(section_refs, "resolution_rule"),
            "by_resolution_pass": counter(section_refs, "resolution_pass"),
            "by_confidence": counter(section_refs, "confidence"),
            "names_document_true": sum(1 for r in section_refs if r["names_document"]),
            "names_document_false": sum(1 for r in section_refs if not r["names_document"]),
            "named_doc_lacks_heading": len(named_doc_lacks_heading),
            "named_doc_lacks_heading_by_named_doc": counter(named_doc_lacks_heading, "nearest_alias_target"),
            "named_doc_lacks_heading_by_citing_file": per_file(named_doc_lacks_heading),
            "by_nearest_alias_kind": counter(section_refs, "nearest_alias_kind"),
            "by_target": counter(section_refs, "target"),
            "dangling_by_citing_file": per_file(sec_dangling),
            "dangling_by_target": counter(sec_dangling, "target"),
            "in_code_fence": sum(1 for r in section_refs if r["in_code_fence"]),
        },
        "named_doc_lacks_heading": named_doc_lacks_heading,
        "resolved": sec_resolved,
        "dangling": sec_dangling,
        "external": sec_external,
    },
    "c_refs": {
        "brief_section_C_items": C_ITEMS,
        "brief_section_C_item_count": C_MAX,
        "correction_note": {"file": ac, "line": 4460, "text": DOC_LINES[ac][4459].strip()},
        "counts": {
            "total": len(c_refs), "resolved": len(c_resolved), "past_end": len(c_dangling),
            "by_spelling": counter(c_refs, "spelling"), "by_n": counter(c_refs, "n"),
            "by_citing_file": per_file(c_refs),
            "names_brief_on_line": sum(1 for r in c_refs if r["names_brief_on_line"]),
            "other_doc_named_on_line": sum(1 for r in c_refs if r["nearest_named_doc_if_not_brief"]),
            "keyword_mismatch_candidates": sum(1 for r in c_refs if r["keyword_mismatch_candidate"]),
            "principle_N_mentions": len(principle_mentions),
            "principle_11_mentions": sum(1 for p in principle_mentions if p["n"] == 11),
        },
        "resolved": c_resolved,
        "dangling": c_dangling,
        "principle_mentions": principle_mentions,
    },
    "file_paths": {
        "counts": {
            "total": len(path_refs), "resolved": len(p_resolved), "dangling": len(p_dangling),
            "basename_unique": len(p_basename_unique), "not_a_path": len(p_not_path),
            "status_rule": "exists = resolves as a repo-relative path or relative to the citing file's directory; basename-unique = a bare filename (no directory) that exists exactly once outside spikes/fixtures; missing = neither; not-a-path = hostname lookalike (docs.rs, crates.io, lib.rs)",
            "dangling_distinct_raw": len({r["raw"] for r in p_dangling}),
            "dangling_by_how": counter(p_dangling, "how"),
            "dangling_by_prefix_bucket": counter(p_dangling, "prefix_bucket"),
            "dangling_by_citing_file": per_file(p_dangling),
            "resolved_by_how": counter(p_resolved, "how"),
            "glob_patterns_seen": len(glob_refs),
            "absolute_drive_paths_seen": len(drive_refs),
            "relative_md_link_targets": len(link_path_refs),
            "relative_md_link_targets_dangling": len(lp_dangling),
        },
        "citing_class_rule": "planning-doc = RUNBOOK.md + references/**; spike-method = spikes/** except results/remediation/p9; results-data = spikes/results/**, spikes/remediation/**, spikes/p9/** (these describe the OLD redkiln repo and its fixtures, so their paths are expected not to exist here)",
        "dangling_by_citing_class": counter(p_dangling, "citing_class"),
        "dangling_distinct_planning_and_method_docs": [
            {"raw": raw, "citations": n,
             "how": next(r["how"] for r in p_dangling if r["raw"] == raw and r["citing_class"] != "results-data"),
             "first": next("%s:%d" % (r["file"], r["line"]) for r in p_dangling if r["raw"] == raw and r["citing_class"] != "results-data")}
            for raw, n in sorted(Counter(r["raw"] for r in p_dangling if r["citing_class"] != "results-data").items(), key=lambda kv: (-kv[1], kv[0]))
        ],
        "dangling_distinct": [
            {"raw": raw, "citations": n,
             "how": next(r["how"] for r in p_dangling if r["raw"] == raw),
             "first": next("%s:%d" % (r["file"], r["line"]) for r in p_dangling if r["raw"] == raw)}
            for raw, n in sorted(Counter(r["raw"] for r in p_dangling).items(), key=lambda kv: (-kv[1], kv[0]))
        ],
        "resolved": p_resolved,
        "basename_unique": p_basename_unique,
        "not_a_path": p_not_path,
        "dangling": p_dangling,
        "glob_patterns": glob_refs,
        "absolute_drive_paths": drive_refs,
        "relative_md_link_targets": {"resolved": lp_resolved, "dangling": lp_dangling},
    },
    "links": {
        "counts": {
            "markdown_links": len(md_links), "by_kind": counter(md_links, "kind"),
            "bare_urls": len(urls), "distinct_urls": len({u["url"] for u in urls}),
            "urls_by_host": dict(sorted(Counter(re.sub(r"^[a-z]+://", "", u["url"]).split("/")[0]
                                                for u in urls).items(), key=lambda kv: (-kv[1], kv[0]))),
            "urls_by_citing_file": per_file(urls),
        },
        "markdown_links": md_links,
        "urls": urls,
    },
}

os.makedirs(os.path.dirname(OUT_PATH), exist_ok=True)
body = json.dumps(out, indent=1, ensure_ascii=True, sort_keys=False)
with open(OUT_PATH, "w", encoding="utf-8", newline="\n") as fh:
    fh.write(body)

# reproducibility digest over everything except the timestamp
stable = dict(out)
stable["meta"] = {k: v for k, v in out["meta"].items() if k != "generated_at"}
digest = hashlib.sha256(json.dumps(stable, sort_keys=True, ensure_ascii=True).encode("utf-8")).hexdigest()
print("wrote", OUT_PATH, "bytes", len(body))
print("content_sha256(excluding generated_at)", digest)
print("corpus", len(CORPUS), "headings", sum(len(v) for v in HEADINGS.values()))
print("section_refs", out["section_refs"]["counts"])
print("c_refs", out["c_refs"]["counts"])
print("file_paths", out["file_paths"]["counts"])
print("links", {k: v for k, v in out["links"]["counts"].items() if k not in ("urls_by_host", "urls_by_citing_file")})
print("assertions_failed", [a["name"] for a in assertions if not a["ok"]])
