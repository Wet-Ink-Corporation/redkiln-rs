#!/usr/bin/env python3
# census_known.py -- build the KNOWN-ISSUES REGISTER for the redkiln-rs pre-implementation audit.
#
# READ-ONLY over D:/repos/redkiln-rs. Writes exactly one file: the JSON artifact.
# ASCII-only output to the console (Windows console cannot print non-ASCII).
#
# Method: section-anchored extraction. Every source section is located by its VERBATIM heading
# string (not a line number), so the script survives line drift. Each spec declares an extraction
# mode and an anchor assertion (min_rows); a spec that yields fewer rows than its anchor is
# reported as an ANCHOR FAILURE, never silently dropped.

import json
import os
import re
import sys
import math
import hashlib
from collections import Counter, defaultdict

REPO = "D:/repos/redkiln-rs"
OUT = "D:/repos/redkiln-rs/references/audit/2026-09-10/census/known-issues.json"

ARCH = "references/2026-09-08-architecture-design.md"
AC = "references/2026-09-08-acceptance-criteria.md"
SM = "references/2026-09-08-success-measures.md"
EA = "references/2026-09-08-evidence-augmentation.md"
EB = "references/2026-09-08-evidence-brief.md"
RB = "RUNBOOK.md"
H10 = "spikes/plans/HANDOFF-2026-09-10-design-session.md"
HST = "spikes/HANDOFF.md"
DEC = "spikes/DECISIONS-2026-09-09.md"

_cache = {}


def lines_of(rel):
    if rel not in _cache:
        p = os.path.join(REPO, rel).replace("\\", "/")
        with open(p, "r", encoding="utf-8") as fh:
            _cache[rel] = fh.read().split("\n")
    return _cache[rel]


def fence_mask(lines):
    """True where the line sits inside a ``` fenced block (fence lines themselves True)."""
    mask = [False] * len(lines)
    inside = False
    for i, l in enumerate(lines):
        if l.lstrip().startswith("```"):
            mask[i] = True
            inside = not inside
            continue
        mask[i] = inside
    return mask


HEAD_RE = re.compile(r"^(#{1,6})\s+(.*)$")


def find_section(rel, heading_prefix):
    """Return (start, end) 0-based line indices for the section whose heading starts with
    heading_prefix (outside code fences). end is exclusive, at the next heading of
    same-or-shallower level. Raises if not found or ambiguous."""
    lines = lines_of(rel)
    mask = fence_mask(lines)
    hits = []
    for i, l in enumerate(lines):
        if mask[i]:
            continue
        m = HEAD_RE.match(l)
        if m and m.group(0).startswith(heading_prefix):
            hits.append((i, len(m.group(1))))
    if not hits:
        raise KeyError("heading not found in %s: %r" % (rel, heading_prefix))
    exact = [h for h in hits if lines[h[0]].rstrip() == heading_prefix]
    start, level = (exact[0] if exact else hits[0])
    end = len(lines)
    for i in range(start + 1, len(lines)):
        if mask[i]:
            continue
        m = HEAD_RE.match(lines[i])
        if m and len(m.group(1)) <= level:
            end = i
            break
    return start, end


# ---------------------------------------------------------------- extractors

SEP_RE = re.compile(r"^\|[\s:\-|]+\|$")


def norm_cell(s):
    return re.sub(r"\s+", " ", s).strip()


def extract_tables(rel, start, end):
    """Yield dicts {headers, cells, line} for every data row of every markdown table."""
    lines = lines_of(rel)
    mask = fence_mask(lines)
    out = []
    headers = None
    pending_header = None
    for i in range(start, end):
        if mask[i]:
            continue
        l = lines[i].rstrip()
        if not (l.startswith("|") and l.endswith("|") and len(l) > 2):
            headers = None
            pending_header = None
            continue
        if SEP_RE.match(l):
            headers = pending_header
            pending_header = None
            continue
        cells = [norm_cell(c) for c in l[1:-1].split("|")]
        if headers is None:
            pending_header = cells
            continue
        out.append({"headers": headers, "cells": cells, "line": i + 1})
    return out


ITEM_RE = re.compile(r"^\s{0,3}(?:(\d{1,2})\.|[-*])\s+(.*)$")


def extract_list(rel, start, end, want_numbered=None):
    """Top-level list items with their continuation lines joined."""
    lines = lines_of(rel)
    mask = fence_mask(lines)
    out = []
    cur = None
    for i in range(start, end):
        if mask[i]:
            continue
        raw = lines[i]
        m = ITEM_RE.match(raw)
        indent = len(raw) - len(raw.lstrip())
        if m and indent <= 3:
            numbered = m.group(1) is not None
            if want_numbered is not None and numbered != want_numbered:
                # not the list family we want; still terminate any open item
                if cur:
                    out.append(cur)
                    cur = None
                continue
            if cur:
                out.append(cur)
            cur = {"text": m.group(2).strip(), "line": i + 1, "marker": m.group(1)}
        elif cur is not None:
            if raw.strip() == "":
                continue
            m2 = HEAD_RE.match(raw)
            if m2 or (raw.startswith("|") and raw.endswith("|")):
                out.append(cur)
                cur = None
                continue
            if indent >= 2 or raw.strip().startswith(("**", "`", "(")):
                cur["text"] += " " + raw.strip()
            else:
                out.append(cur)
                cur = None
    if cur:
        out.append(cur)
    return out


def extract_paras(rel, start, end, min_len=80):
    """Prose paragraphs (not tables, headings, fences, list items)."""
    lines = lines_of(rel)
    mask = fence_mask(lines)
    out = []
    buf = []
    bstart = None
    def flush():
        if buf:
            txt = norm_cell(" ".join(buf))
            if len(txt) >= min_len:
                out.append({"text": txt, "line": bstart + 1})
        return
    for i in range(start, end):
        if mask[i]:
            flush(); buf[:] = []; bstart = None
            continue
        l = lines[i]
        s = l.strip()
        if s == "" or HEAD_RE.match(l) or (s.startswith("|") and s.endswith("|")) or s == "---":
            flush(); buf[:] = []; bstart = None
            continue
        if not buf:
            bstart = i
        buf.append(s)
    flush()
    return out


def extract_regex(rel, pattern, flags=0, start=0, end=None):
    lines = lines_of(rel)
    end = len(lines) if end is None else end
    rx = re.compile(pattern, flags)
    return [{"text": norm_cell(lines[i]), "line": i + 1}
            for i in range(start, end) if rx.search(lines[i])]


# ---------------------------------------------------------------- classification

STRIP_MD = re.compile(r"[*`]+|(?<![A-Za-z0-9])_+|_+(?![A-Za-z0-9])")


def plain(s):
    return STRIP_MD.sub("", s)


# ordered: first match wins. (regex, kind)
KIND_RULES = [
    (r"not answerable|NOT ANSWERABLE|circular by construction|empty denominator|"
     r"no denominator|has no denominator|cannot be evaluated at all|denominator does not exist|"
     r"empty positive class|empty ledger join", "not_answerable"),
    (r"SATURATED|saturated|at ceiling|ceiling result|ceiling/floor|zero variance|"
     r"nowhere left to move", "saturated_metric"),
    (r"POWERLESS|powerless|Wilson interval.{0,30}overlap|overlap.{0,20}by construction|"
     r"power, not (its |the )?verdict", "powerless_criterion"),
    (r"\bn\s?=\s?1\b|n=1|at n=1|ARM DROPPED|arm dropped|uncomputable at n", "n_equals_1"),
    (r"NOT MEASURABLE|not measurable|unmeasurable|MEASURED ABSENT|not computable|"
     r"cannot be measured|is not measurable|below the instrument|quantised to|"
     r"instrument cannot resolve|resolution limit|STRUCK", "not_measurable"),
    (r"CONFOUNDED|confound|not instrument-matched|lineage break|instrument defect|"
     r"INSTRUMENT DEFECT|version move mid-run|straddled", "instrument_confound"),
    (r"CONTESTED|contested|UNRECONCILED|unreconciled|contradiction|contradicts|"
     r"contradicted|KNOWN WRONG|known wrong|two numbers, one quantity|"
     r"in direct tension|collide|mismatch", "known_contradiction"),
    (r"UNRUN|unrun|Not started|not started|Not registered|never ran|never run|"
     r"has not been run|was never taken|Read not taken|closed unmeasured|"
     r"registered and \*\*unrun|REGISTERED AND UNRUN|Table open", "unrun_spike"),
    (r"BLOCKED|blocked|gates Phase 1|holds the block", "blocked_decision"),
    (r"stale|STALE|runs stale|superseded|SUPERSEDED|misled", "stale_document"),
    (r"does not exist|no ledger entry|never recorded|no entry in|was never captured|"
     r"never captured|no baseline|has no baseline|no such|absent|no payload|"
     r"no record|no scorer|does not appear|no row in|nothing holds them|"
     r"no measured baseline|no external comparator", "missing_artifact"),
    (r"awaiting Ryan|Awaiting Ryan|owed by Ryan|Owed by Ryan|Ryan's|needs Ryan|"
     r"sign-off|signoff|human decision|needs a decision|human judgement|"
     r"one operator's judgement|adjudication is open|reverse or confirm|OWED", "owed_by_human"),
    (r"UNVERIFIED|unverified|search-only|Search-only|not verified|un(-| )?corroborated|"
     r"\[inferred\]|inferred|read implying|assumed|unproven|documentation only|"
     r"\[documented\] from|borrowed", "unverified_claim"),
    (r"deliberate|deliberately|refused outright|never reuse|forbids|forbidden|"
     r"exemption|exception|must not be|may never|is refused|struck rather than|"
     r"cut rather than", "deliberate_exception"),
    (r"open item|Open item|open question|still open|left open|leaves open|is OPEN|"
     r"undecided|unset|undeclared|no declared number|not yet", "scope_limit"),
]
KIND_RULES = [(re.compile(p), k) for p, k in KIND_RULES]

VALID_KINDS = {
    "blocked_decision", "unrun_spike", "unverified_claim", "powerless_criterion",
    "not_measurable", "not_answerable", "saturated_metric", "n_equals_1",
    "instrument_confound", "stale_document", "known_contradiction",
    "deliberate_exception", "missing_artifact", "owed_by_human", "scope_limit", "other",
}


def classify(text, default_kind, force_kind=None):
    if force_kind:
        return force_kind
    for rx, k in KIND_RULES:
        if rx.search(text):
            return k
    return default_kind or "other"


STATUS_PATTERNS = [
    r"\bBLOCKED\b", r"\bPROVISIONAL\b", r"\bDECIDED\b", r"\bUNRUN\b", r"\bSATURATED\b",
    r"\bSTRUCK\b", r"\bCONTESTED\b", r"\bRESOLVED\b", r"\bCLOSED\b", r"\bSCORED\b",
    r"\bOWED, NOT DONE\b", r"\bUNMEASURED\b", r"\bCONFOUNDED\b", r"\bSUPERSEDED\b",
    r"\bKNOWN WRONG\b", r"\bFAILED OUTRIGHT\b", r"\bVACUOUS\b", r"\bDIRTY TREE\b",
    r"\bNot started\b", r"\bNot registered\b", r"\bTable open\b", r"\bOpen\b",
    r"\bRead not taken\b", r"\bproposed\b", r"\baccepted\b", r"\bPARTIAL\b",
]
STATUS_RE = re.compile("|".join(STATUS_PATTERNS))

STATUS_COL_RE = re.compile(r"state|status|disposition|reversible|answerable|gates phase|verdict|severity", re.I)
OWNER_RE = re.compile(r"Ryan Britton|Ryan|PC2 owner|the operator|one operator|Claude|human panel|a human")


def derive_status(cells, headers, text):
    if headers:
        for pref in ("state", "status"):
            for h, c in zip(headers, cells):
                if h and pref in h.lower() and c:
                    return plain(c)[:140]
        for h, c in zip(headers, cells):
            if h and STATUS_COL_RE.search(h) and c:
                return (plain(h) + ": " + plain(c))[:140]
    m = STATUS_RE.search(text)
    return m.group(0) if m else None


def derive_owner(text):
    m = OWNER_RE.search(text)
    return m.group(0) if m else None


def derive_subject(text, cells=None):
    if cells:
        for c in cells:
            if c and c not in ("-", "--", "---"):
                return plain(c)[:160]
    t = plain(text).strip()
    m = re.match(r"^([^.:;|]{8,160})", t)
    return (m.group(1).strip() if m else t[:160])


# ---------------------------------------------------------------- search index

STOP = set("""about above after again against along already also although always among another
answer answers anything appear appears applied applies apply argument around because become
becomes been before behind being below beside besides better between beyond both bring built
cannot carry carries carried change changed changes check checked checks claim claims class
clause close closed could count counts cover covers decide decided decides decision decisions
design detail details different document documents does doing done during each earlier either
enough entire entirely error errors even event events every everything evidence exact exactly
exist exists explicit explicitly fact facts fail failed fails first five following forced four
from front full further given gives going great group half hand have having held here holds
however itself just keep kept know known large last later least leave leaves left less level
like line lines little live lives look looks made make makes making many matter matters
might more most much must name named names needs neither never next nine none note noted notes
nothing number numbers often once only open other others over part parts pass past place point
points possible present produce produced produces question questions quite range rate rates
rather read reads real really reason reasons record recorded records report reported reports
result results right round rule rules runs said same says second section sections seen sets
seven several shall shape share shares should show shows side since single small some something
sometimes state stated states still stop such take taken takes tell than that their them then
there these they thing things think third this those though three through thus time times
today together told took total toward true turn twice twelve under until upon used uses using
value values very want warrant were what when where whether which while whole whose will with
within without work works would write writes written wrong year yet""".split())

ID_RES = [
    re.compile(r"\b(?:AC|SM|ADR)-\d{3,4}\b"),
    re.compile(r"\bF\d{1,2}\b"),
    re.compile(r"\bS-[A-Z0-9]+(?:-[A-Z0-9]+)*\b"),
    re.compile(r"\bP\d(?:-[A-Z]+)?\b"),
    re.compile(r"\bE\d\b"), re.compile(r"\bH\d[a-c]?\b"),
    re.compile(r"\bR-[A-Z0-9]+\b"), re.compile(r"\bD-[A-Z]+\b"),
    re.compile(r"\bS0\.\d\b"), re.compile(r"\bC\.\d{1,2}\b"),
    re.compile(r"\bC[123]\b"),
    re.compile(r"\bSM-\d+\b"),
    re.compile(r"\b\d+\.\d+\.\d{3}\b"),          # CLI versions
    re.compile(r"\b[0-9a-f]{7,40}\b"),           # shas
]
FILE_RE = re.compile(r"[A-Za-z0-9_./\\-]+\.(?:md|toml|json|jsonl|py|rs|ps1|sh|txt|js|cpp)")
NUM_RE = re.compile(r"\d+(?:\.\d+)?(?:%|x|\u00d7)?|\d+\s?/\s?\d+")
WORD_RE = re.compile(r"[a-z][a-z_\-]{4,}")


def raw_keywords(text):
    t = plain(text)
    ks = set()
    for rx in ID_RES:
        for m in rx.findall(t):
            ks.add(m.lower())
    for m in FILE_RE.findall(t):
        ks.add(os.path.basename(m).lower())
    for m in NUM_RE.findall(t):
        s = m.replace(" ", "")
        # keep only numbers that carry signal: a unit, a ratio, a decimal, or >=3 digits
        # (bare 1-2 digit list indices and the year 2026 are noise)
        if s in ("2025", "2026"):
            continue
        if re.search("[%x\u00d7/.]", s) or len(re.sub(r"\D", "", s)) >= 3:
            ks.add(s.lower())
    # drop a bare number that is only the digit suffix of an id already captured (sm-021 -> 021)
    id_suffixes = {k.split("-")[-1] for k in ks if "-" in k}
    ks = {k for k in ks if not (k.isdigit() and k in id_suffixes)}
    words = [w for w in WORD_RE.findall(t.lower()) if w not in STOP]
    return ks, words


# ---------------------------------------------------------------- specs

# mode: table | list | numlist | para | regex
SPECS = [
    # ---- architecture design
    dict(src=ARCH, head="## 0.1 What is decided, and what is blocked", mode="table",
         prefix="ARCH-0.1", subject_note="decision area", default="blocked_decision", min_rows=35),
    dict(src=ARCH, head="### 1.2 Not settled, saturated, struck, or known wrong", mode="table",
         prefix="ARCH-1.2", default="unverified_claim", min_rows=35),
    dict(src=ARCH, head="### 1.3 The five results that changed the thesis", mode="table",
         prefix="ARCH-1.3", default="known_contradiction", min_rows=5),
    dict(src=ARCH, head="### 3.10 Open items this section carries", mode="table",
         prefix="ARCH-3.10", default="scope_limit", min_rows=8),
    dict(src=ARCH, head="### 4.9 Open items in this area", mode="table",
         prefix="ARCH-4.9", default="scope_limit", min_rows=10),
    dict(src=ARCH, head="### 5.11 Open items in this section", mode="table",
         prefix="ARCH-5.11", default="scope_limit", min_rows=9),
    dict(src=ARCH, head="### 6.6 Decisions and open items", mode="table",
         prefix="ARCH-6.6", default="scope_limit", min_rows=15),
    dict(src=ARCH, head="### 7.10 Open items in this section", mode="table",
         prefix="ARCH-7.10", default="scope_limit", min_rows=8),
    dict(src=ARCH, head="### 8.10 What this section leaves open", mode="table",
         prefix="ARCH-8.10", default="scope_limit", min_rows=10),
    dict(src=ARCH, head="### 9.4 Two things this policy does not settle", mode="table",
         prefix="ARCH-9.4", default="scope_limit", min_rows=2),
    dict(src=ARCH, head="### 10.3 What Phase 1 ships instead of an answer", mode="table",
         prefix="ARCH-10.3", default="scope_limit", min_rows=3),
    dict(src=ARCH, head="### 10.4 Open items", mode="table",
         prefix="ARCH-10.4", default="scope_limit", min_rows=10),
    dict(src=ARCH, head="### 10.5 The spike register", mode="table",
         prefix="ARCH-10.5", default="unrun_spike", min_rows=30),
    dict(src=ARCH, head="### 10.6 What would falsify this design", mode="table",
         prefix="ARCH-10.6", default="scope_limit", min_rows=8),
    dict(src=ARCH, head="### 10.x Review of this document", mode="table",
         prefix="ARCH-10.x-T", default="known_contradiction", min_rows=18),
    dict(src=ARCH, head="### 10.x Review of this document", mode="list",
         prefix="ARCH-10.x-L", default="deliberate_exception", min_rows=4),

    # ---- acceptance criteria
    dict(src=AC, head="### 0.3 Cross-area duplication, and three contradictions", mode="table",
         prefix="AC-0.3", default="known_contradiction", min_rows=35),
    dict(src=AC, head="### 0.4 Id hazards", mode="table",
         prefix="AC-0.4-T", default="known_contradiction", min_rows=6),
    dict(src=AC, head="### 0.4 Id hazards", mode="para",
         prefix="AC-0.4-P", default="known_contradiction", min_rows=4),
    dict(src=AC, head="### 1.1 The selection engine", mode="para",
         prefix="AC-1.1", default="blocked_decision", min_rows=6),
    dict(src=AC, head="### 1.2 Everything else that is blocked", mode="table",
         prefix="AC-1.2", default="blocked_decision", min_rows=15),
    dict(src=AC, head="### 1.2 Everything else that is blocked", mode="para",
         prefix="AC-1.2-P", default="not_measurable", min_rows=1),
    dict(src=AC, head="### 15.3 Decisions no criterion cites", mode="table",
         prefix="AC-15.3", default="missing_artifact", min_rows=3),
    dict(src=AC, head="### 15.3 Decisions no criterion cites", mode="para",
         prefix="AC-15.3-P", default="missing_artifact", min_rows=1),

    # ---- success measures
    dict(src=SM, head="### 3.1 Saturated", mode="table", prefix="SM-3.1",
         force="saturated_metric", min_rows=5),
    dict(src=SM, head="### 3.2 The instrument cannot resolve it", mode="table", prefix="SM-3.2",
         force="not_measurable", min_rows=6),
    dict(src=SM, head="### 3.3 The denominator does not exist", mode="table", prefix="SM-3.3",
         force="not_answerable", min_rows=14),
    dict(src=SM, head="### 3.4 Refused shapes of reporting", mode="list", prefix="SM-3.4",
         force="deliberate_exception", min_rows=7),
    dict(src=SM, head="### 5.1 Waiting on a first production window", mode="table",
         prefix="SM-5.1", default="missing_artifact", min_rows=45),
    dict(src=SM, head="### 5.2 Waiting on a named unrun spike or an open adjudication", mode="table",
         prefix="SM-5.2", default="unrun_spike", min_rows=10),
    dict(src=SM, head="### 5.3 Waiting on a payload or a field that has never been recorded", mode="table",
         prefix="SM-5.3", default="missing_artifact", min_rows=4),
    dict(src=SM, head="### 5.4 Waiting on a threshold nobody has chosen", mode="table",
         prefix="SM-5.4", default="owed_by_human", min_rows=3),
    dict(src=SM, head="### 5.5 The honest count", mode="para", prefix="SM-5.5",
         default="scope_limit", min_rows=1),

    # ---- evidence augmentation
    dict(src=EA, head="## 1. Contradictions", mode="table", prefix="EA-1",
         force="known_contradiction", min_rows=13),
    dict(src=EA, head="## 5. What external research could not settle", mode="table",
         prefix="EA-5", force="not_answerable", min_rows=28),

    # ---- evidence brief
    dict(src=EB, head="## B. Kill criteria", mode="para", prefix="EB-B",
         default="powerless_criterion", min_rows=3),
    dict(src=EB, head="## E. Open items that must not disappear", mode="para", prefix="EB-E",
         default="scope_limit", min_rows=6),

    # ---- RUNBOOK
    dict(src=RB, head="## Preamble", mode="numlist", prefix="RB-GC",
         force="deliberate_exception", min_rows=11),
    dict(src=RB, head="## Halt conditions", mode="numlist", prefix="RB-HALT",
         force="deliberate_exception", min_rows=13),

    # ---- 2026-09-10 handoff (current)
    dict(src=H10, head="## The open loops, each with its owner", mode="table", prefix="H10-LOOP-T",
         default="blocked_decision", min_rows=3),
    dict(src=H10, head="## The open loops, each with its owner", mode="numlist", prefix="H10-OWED",
         force="owed_by_human", min_rows=3),
    dict(src=H10, head="## The open loops, each with its owner", mode="para", prefix="H10-LOOP-P",
         default="unrun_spike", min_rows=2),
    dict(src=H10, head="## The open loops, each with its owner", mode="list", prefix="H10-AUDIT",
         force="owed_by_human", min_rows=3),
    dict(src=H10, head="## Traps this session actually hit", mode="list", prefix="H10-TRAP",
         default="instrument_confound", min_rows=6),

    # ---- stale handoff  (stale_source = true, set below)
    dict(src=HST, head="## What is genuinely unresolved", mode="numlist", prefix="HST-UNRES",
         default="scope_limit", min_rows=10, stale=True),
    dict(src=HST, head="## Traps that cost time here", mode="list", prefix="HST-TRAP",
         default="instrument_confound", min_rows=6, stale=True),
    dict(src=HST, head="## Pre-registration status", mode="numlist", prefix="HST-AMEND",
         force="instrument_confound", min_rows=9, stale=True),

    # ---- decisions log
    dict(src=DEC, head="## Decisions Ryan should reverse or confirm first", mode="table",
         prefix="DEC-REV", force="owed_by_human", min_rows=7),
    dict(src=DEC, head="## Decisions", mode="table", prefix="DEC-D",
         default="scope_limit", min_rows=8),
    dict(src=DEC, head="## Open instrument problem, disclosed not fixed", mode="para",
         prefix="DEC-OPEN", force="instrument_confound", min_rows=2),

    # ---- method pre-registrations
    dict(src="spikes/METHOD-S.md", head="## 7. Amendments", mode="list", prefix="MS-AMEND",
         force="instrument_confound", min_rows=10),
    dict(src="spikes/METHOD-P.md", head="## 6. Order, and what each can overturn", mode="para",
         prefix="MP-AMEND", force="instrument_confound", min_rows=1),
    dict(src="spikes/METHOD-P.md", head="### 1.5 Limits", mode="para", prefix="MP-1.5",
         default="scope_limit", min_rows=1),
    dict(src="spikes/METHOD-P.md", head="### 2.5 Limits", mode="para", prefix="MP-2.5",
         default="scope_limit", min_rows=1),
    dict(src="spikes/METHOD-P.md", head="### 4.5 Limits", mode="para", prefix="MP-4.5",
         default="scope_limit", min_rows=1),
    dict(src="spikes/METHOD-P.md", head="### 5.5 Limits", mode="para", prefix="MP-5.5",
         default="scope_limit", min_rows=1),
]

# F-series draft Limits sections (discovered, then asserted)
F_LIMIT_SECTIONS = [
    ("spikes/F29-DRAFT.md", "### Limits"),
    ("spikes/F31-DRAFT.md", "### Limits"),
    ("spikes/F32-DRAFT.md", "### Limits"),

    ("spikes/F34-DRAFT.md", "### Limits"),
    ("spikes/F35-DRAFT.md", "### Limits"),
    ("spikes/F36-DRAFT.md", "### 4. Limits"),
    ("spikes/F37-DRAFT.md", "### Limits"),
    ("spikes/F38-DRAFT.md", "### Limits"),
    ("spikes/F39-DRAFT.md", "### Limits"),
    ("spikes/F40-DRAFT.md", "### Limits"),
]
SPECS.append(dict(src="spikes/F33-DRAFT.md", head="### 3. Ambiguity list", mode="table",
                  prefix="F33-LIM", default="scope_limit", min_rows=20))
SPECS.append(dict(src="spikes/F33-DRAFT.md", head="### 3. Ambiguity list", mode="para",
                  prefix="F33-LIMP", default="scope_limit", min_rows=1))
SPECS.append(dict(src="spikes/F33-DRAFT.md", head="### 2. What the lint accepted that it should not have", mode="anylist",
                  prefix="F33-ACC", default="scope_limit", min_rows=1))
for rel, head in F_LIMIT_SECTIONS:
    SPECS.append(dict(src=rel, head=head, mode="anylist",
                      prefix=os.path.basename(rel).split("-")[0] + "-LIM",
                      default="scope_limit", min_rows=1))
    SPECS.append(dict(src=rel, head=head, mode="para",
                      prefix=os.path.basename(rel).split("-")[0] + "-LIMP",
                      default="scope_limit", min_rows=0))


# ---------------------------------------------------------------- build

rows = []
anchors = []
seen_hash = set()


def add(rid, rel, line, verbatim, kind, subject, status, owner, stale=False,
        extra=None):
    v = norm_cell(verbatim)
    if len(v) > 500:
        v = v[:497] + "..."
    h = hashlib.sha1((rel + "|" + v).encode("utf-8")).hexdigest()
    if h in seen_hash:
        return False
    seen_hash.add(h)
    r = {
        "id": rid,
        "source_anchor": "%s:%d" % (rel, line),
        "verbatim": v,
        "kind": kind,
        "subject": subject,
        "status_if_stated": status,
        "owner_if_stated": owner,
    }
    if stale:
        r["stale_source"] = True
    if extra:
        r.update(extra)
    rows.append(r)
    return True


for spec in SPECS:
    rel, head, mode = spec["src"], spec["head"], spec["mode"]
    prefix = spec["prefix"]
    default = spec.get("default")
    force = spec.get("force")
    stale = spec.get("stale", False)
    try:
        start, end = find_section(rel, head)
    except KeyError as e:
        anchors.append({"spec": prefix, "source": rel, "heading": head,
                        "found": False, "rows": 0, "expected_min": spec["min_rows"],
                        "note": "HEADING NOT FOUND"})
        continue
    n = 0
    if mode == "table":
        for t in extract_tables(rel, start, end):
            txt = " | ".join(t["cells"])
            if len(plain(txt).strip(" |")) < 12:
                continue
            # a State/Status column, when present, is the corpus's own classification and wins
            state_cell = ""
            for h, c in zip(t["headers"] or [], t["cells"]):
                if h and re.search(r"^(state|status)", h.strip().lower()) and c:
                    state_cell = plain(c)
                    break
            k = None
            if state_cell and not force:
                su = state_cell.upper()
                if su.startswith("BLOCKED"):
                    k = "blocked_decision"
                elif su.startswith("PROVISIONAL"):
                    k = "unverified_claim"
                else:
                    k = classify(state_cell, None)
                if k == "other":
                    k = None
            if k is None:
                k = classify(txt, default, force)
            if state_cell.upper().startswith("DECIDED") and k == "blocked_decision":
                k = "scope_limit"   # a caveat inside a decided row is a limit, not a block
            n += add("%s-%03d" % (prefix, n + 1), rel, t["line"], txt, k,
                     derive_subject(txt, t["cells"]),
                     derive_status(t["cells"], t["headers"], txt),
                     derive_owner(txt), stale,
                     {"table_headers": t["headers"]})
    elif mode in ("list", "numlist", "anylist"):
        want = {"numlist": True, "list": False, "anylist": None}[mode]
        for it in extract_list(rel, start, end, want_numbered=want):
            if len(plain(it["text"])) < 40:
                continue
            k = classify(it["text"], default, force)
            n += add("%s-%03d" % (prefix, n + 1), rel, it["line"], it["text"], k,
                     derive_subject(it["text"]), derive_status([], None, it["text"]),
                     derive_owner(it["text"]), stale)
    elif mode == "para":
        for p in extract_paras(rel, start, end, min_len=spec.get("min_len", 80)):
            k = classify(p["text"], default, force)
            n += add("%s-%03d" % (prefix, n + 1), rel, p["line"], p["text"], k,
                     derive_subject(p["text"]), derive_status([], None, p["text"]),
                     derive_owner(p["text"]), stale)
    anchors.append({"spec": prefix, "source": rel, "heading": head, "found": True,
                    "rows": n, "expected_min": spec["min_rows"],
                    "anchor_ok": n >= spec["min_rows"]})

# ---------------------------------------------------------------- DECISIONS log: self-critical section headings
# "## Decision 33 -- the mandatory arm is dropped", "## Finding surfaced while ...", "## Open instrument problem ..."
# Each heading is itself a recorded admission; the first paragraph under it is the verbatim.
dec_lines = lines_of(DEC)
dec_mask = fence_mask(dec_lines)
n = 0
for i, l in enumerate(dec_lines):
    if dec_mask[i]:
        continue
    m = re.match(r"^## (Decision 3\d|Finding surfaced|Open instrument problem|P1 arm L v2)", l)
    if not m:
        continue
    # first non-empty prose line after the heading
    body = ""
    for j in range(i + 1, min(i + 12, len(dec_lines))):
        sj = dec_lines[j].strip()
        if sj and not sj.startswith("|") and not sj.startswith("#"):
            body = sj
            break
    txt = norm_cell(l.lstrip("# ") + " :: " + body)
    n += add("DEC-SEC-%03d" % (n + 1), DEC, i + 1, txt,
             classify(txt, "instrument_confound"), derive_subject(l.lstrip("# ")),
             derive_status([], None, txt), derive_owner(txt))
anchors.append({"spec": "DEC-SEC", "source": DEC, "heading": "Decision 3x / Finding surfaced / Open instrument headings",
                "found": True, "rows": n, "expected_min": 7, "anchor_ok": n >= 7})

# ---------------------------------------------------------------- per-criterion / per-measure tags inside the big registers
# Lines in the AC register carrying PROVISIONAL / UNVERIFIED, and lines in the SM register carrying
# "no baseline". Lines there can exceed 3,000 chars, so the verbatim is a window round the FIRST match.
ID_IN_LINE = re.compile(r"\b(?:AC|SM)-\d{3}\b")


def window_rows(rel, pattern, prefix, default_kind, expected_min, force_kind=None, skip_before=0):
    lines = lines_of(rel)
    mask = fence_mask(lines)
    rx = re.compile(pattern)
    n = 0
    last_id = None          # nearest preceding AC-/SM- id (the register is one entry per id)
    for i, l in enumerate(lines):
        if i < skip_before or mask[i]:
            continue
        head_ids = ID_IN_LINE.findall(l[:80])
        if head_ids and (HEAD_RE.match(l) or l.startswith("|") or l.startswith("**")):
            last_id = head_ids[0]
        m = rx.search(l)
        if not m:
            continue
        a = max(0, m.start() - 220)
        b = min(len(l), m.end() + 220)
        win = ("..." if a > 0 else "") + l[a:b] + ("..." if b < len(l) else "")
        ids = ID_IN_LINE.findall(l[:m.end() + 40])
        subj = (ids[-1] if ids else (last_id + " :: " + derive_subject(win) if last_id else derive_subject(win)))
        n += add("%s-%03d" % (prefix, n + 1), rel, i + 1, win,
                 classify(win, default_kind, force_kind), subj,
                 derive_status([], None, win), derive_owner(win))
    anchors.append({"spec": prefix, "source": rel, "heading": "line scan: %s" % pattern,
                    "found": True, "rows": n, "expected_min": expected_min,
                    "anchor_ok": n >= expected_min})


# AC register body starts at "## 2." (line ~201); sections 0-1 are harvested structurally above.
ac_body_start, _ = find_section(AC, "## 2. Kernel")
window_rows(AC, r"\bPROVISIONAL\b", "AC-PROV", "unverified_claim", 30, skip_before=ac_body_start)
window_rows(AC, r"\bUNVERIFIED\b|\bunverified\b", "AC-UNVER", "unverified_claim", 8, skip_before=ac_body_start)
sm_body_start, _ = find_section(SM, "## 1. The register")
sm_body_end, _ = find_section(SM, "## 2. Baselines at a glance")
window_rows(SM, r"[Nn]o baseline", "SM-NOBASE", "missing_artifact", 60, skip_before=sm_body_start)

# FINDINGS.md (F1-F28): every Limits / caveat / threat heading's section, list items + paragraphs
fnd = "spikes/FINDINGS.md"
fnd_lines = lines_of(fnd)
fnd_mask = fence_mask(fnd_lines)
n = 0
for i, l in enumerate(fnd_lines):
    if fnd_mask[i]:
        continue
    m = HEAD_RE.match(l)
    if not m or not re.search(r"limit|caveat|threat", m.group(2), re.I):
        continue
    start, end = find_section(fnd, l.rstrip())
    for it in extract_list(fnd, start, end, want_numbered=None):
        if len(plain(it["text"])) < 40:
            continue
        n += add("FND-LIM-%03d" % (n + 1), fnd, it["line"], it["text"],
                 classify(it["text"], "scope_limit"), derive_subject(it["text"]),
                 derive_status([], None, it["text"]), derive_owner(it["text"]))
    for p in extract_paras(fnd, start, end):
        n += add("FND-LIM-%03d" % (n + 1), fnd, p["line"], p["text"],
                 classify(p["text"], "scope_limit"), derive_subject(p["text"]),
                 derive_status([], None, p["text"]), derive_owner(p["text"]))
anchors.append({"spec": "FND-LIM", "source": fnd, "heading": "Limits/caveat/threat sections of F1-F28",
                "found": True, "rows": n, "expected_min": 9, "anchor_ok": n >= 9})

# ---------------------------------------------------------------- RUNBOOK markers

rb_will = extract_regex(RB, r"\[WILL EXIST\]")
n = 0
for it in rb_will:
    n += add("RB-WILLEXIST-%03d" % (n + 1), RB, it["line"], it["text"],
             "missing_artifact", derive_subject(it["text"]),
             "[WILL EXIST] - artifact does not exist yet", None)
anchors.append({"spec": "RB-WILLEXIST", "source": RB, "heading": "[WILL EXIST] marker scan",
                "found": True, "rows": n, "expected_min": 10, "anchor_ok": n >= 10})

# RUNBOOK parenthetical / inline admissions of a gap, outside code fences
RB_GAP_RE = (r"does not exist|has never been|was never|no scorer|not been run|"
             r"cannot be reproduced|flagged for review|recorded rather than resolved|"
             r"runs stale|no baseline|is owed|still owed")
rb_lines = lines_of(RB)
rb_mask = fence_mask(rb_lines)
rx = re.compile(RB_GAP_RE)
n = 0
for i, l in enumerate(rb_lines):
    if rb_mask[i] or not rx.search(l):
        continue
    txt = norm_cell(l)
    if len(plain(txt)) < 45:
        continue
    n += add("RB-GAP-%03d" % (n + 1), RB, i + 1, txt,
             classify(txt, "missing_artifact"), derive_subject(txt),
             derive_status([], None, txt), derive_owner(txt))
anchors.append({"spec": "RB-GAP", "source": RB, "heading": "gap-admission line scan",
                "found": True, "rows": n, "expected_min": 8, "anchor_ok": n >= 8})

# ---------------------------------------------------------------- ADRs

import glob

adr_files = sorted(glob.glob(os.path.join(REPO, "references/adr/*.md").replace("\\", "/")))
ADR_MARK_RE = re.compile(
    r"\[unmeasured\]|\[UNVERIFIED\]|\[inferred\]|does not decide|This ADR does not|"
    r"open question|\bunrun\b|\bUNRUN\b|not measured|no measured|never measured|"
    r"unverified|is not settled|leaves open|deliberately unset|no baseline")
adr_marker_n = 0
adr_neg_n = 0
adr_forb_n = 0
adr_status_n = 0

for p in adr_files:
    rel = "references/adr/" + os.path.basename(p)
    lines = lines_of(rel)
    mask = fence_mask(lines)
    # frontmatter status
    fm_status = None
    fm_sup = None
    for i, l in enumerate(lines[:20]):
        if l.startswith("status:"):
            fm_status = l.split(":", 1)[1].strip()
        if l.startswith("superseded_by:") and l.strip() not in ("superseded_by: []",):
            fm_sup = l.strip()
    if fm_status and fm_status not in ("accepted",):
        adr_status_n += 1
        add("ADR-STATUS-%s" % os.path.basename(p)[:4], rel, 1,
            "status: %s -- %s" % (fm_status, norm_cell(lines[2])[:300]),
            "blocked_decision" if fm_status == "proposed" else "stale_document",
            os.path.basename(p)[:4] + " " + norm_cell(lines[2])[:120],
            fm_status, None)
    if fm_sup:
        adr_status_n += 1
        add("ADR-SUPBY-%s" % os.path.basename(p)[:4], rel, 1,
            "%s ; %s" % (fm_sup, norm_cell(lines[2])[:300]),
            "stale_document", os.path.basename(p)[:4] + " " + norm_cell(lines[2])[:120],
            "superseded_by set", None)
    mode = None
    for i, l in enumerate(lines):
        if mask[i]:
            continue
        s = l.strip()
        if s.startswith("**Negative**"):
            mode = "neg"; continue
        if s.startswith("**What this forbids**"):
            mode = "forb"; continue
        if HEAD_RE.match(l) or (s.startswith("**") and s.endswith("**") and len(s) < 70):
            mode = None
        if mode and s.startswith("- ") and len(plain(s)) > 40:
            k = "scope_limit" if mode == "neg" else "deliberate_exception"
            ok = add("ADR-%s-%s-%d" % (os.path.basename(p)[:4], mode.upper(), i),
                     rel, i + 1, s[2:], classify(s, k),
                     derive_subject(s[2:]), derive_status([], None, s), derive_owner(s))
            if ok:
                if mode == "neg":
                    adr_neg_n += 1
                else:
                    adr_forb_n += 1
            continue
        if ADR_MARK_RE.search(l) and len(plain(s)) > 45 and not s.startswith("- "):
            ok = add("ADR-%s-MARK-%d" % (os.path.basename(p)[:4], i), rel, i + 1, s,
                     classify(s, "unverified_claim"), derive_subject(s),
                     derive_status([], None, s), derive_owner(s))
            if ok:
                adr_marker_n += 1

anchors.append({"spec": "ADR-NEGATIVE", "source": "references/adr/*.md",
                "heading": "**Negative** consequence bullets", "found": True,
                "rows": adr_neg_n, "expected_min": 100, "anchor_ok": adr_neg_n >= 100})
anchors.append({"spec": "ADR-FORBIDS", "source": "references/adr/*.md",
                "heading": "**What this forbids** bullets", "found": True,
                "rows": adr_forb_n, "expected_min": 150, "anchor_ok": adr_forb_n >= 150})
anchors.append({"spec": "ADR-MARKERS", "source": "references/adr/*.md",
                "heading": "[unmeasured]/[UNVERIFIED]/does-not-decide/unrun lines",
                "found": True, "rows": adr_marker_n, "expected_min": 25,
                "anchor_ok": adr_marker_n >= 25})
anchors.append({"spec": "ADR-STATUS", "source": "references/adr/*.md",
                "heading": "non-accepted status / superseded_by frontmatter",
                "found": True, "rows": adr_status_n, "expected_min": 3,
                "anchor_ok": adr_status_n >= 3})

# ---------------------------------------------------------------- search index (IDF)

doc_words = []
doc_ids = []
for r in rows:
    ks, words = raw_keywords(r["verbatim"] + " " + (r["subject"] or ""))
    doc_ids.append(ks)
    doc_words.append(words)

df = Counter()
for ws in doc_words:
    for w in set(ws):
        df[w] += 1
N = max(1, len(rows))

search_index = []
for r, ks, ws in zip(rows, doc_ids, doc_words):
    tf = Counter(ws)
    scored = sorted(tf.items(), key=lambda kv: (-(kv[1] * math.log(N / (1 + df[kv[0]]))), kv[0]))
    top = [w for w, _ in scored[:10]]
    kws = sorted(ks)[:24] + top
    # de-dupe, preserve order
    seen = set()
    flat = []
    for k in kws:
        if k not in seen:
            seen.add(k)
            flat.append(k)
    search_index.append({"id": r["id"], "kind": r["kind"], "keywords": flat[:28]})

# ---------------------------------------------------------------- counts + write

by_kind = Counter(r["kind"] for r in rows)
by_source = Counter(r["source_anchor"].split(":")[0] for r in rows)
by_source_group = Counter()
for r in rows:
    f = r["source_anchor"].split(":")[0]
    by_source_group["references/adr/*" if f.startswith("references/adr/") else f] += 1

bad_kind = sorted({r["kind"] for r in rows} - VALID_KINDS)
failed_anchors = [a for a in anchors if not a.get("anchor_ok", False)]

artifact = {
    "artifact": "known-issues-register",
    "phase": "audit ground truth",
    "generated_by": "scripts/census_known.py",
    "repo": REPO,
    "read_only": True,
    "method": ("Section-anchored extraction. Every section is located by its verbatim markdown "
               "heading, not a line number. Each spec declares an extraction mode and a minimum "
               "row count as an anchor assertion; specs that under-deliver are listed in "
               "anchor_report with anchor_ok=false. Rows are de-duplicated by "
               "sha1(source_file + normalised verbatim). kind is assigned by an ordered rule list "
               "over the verbatim text, with a per-spec default or forced kind."),
    "counts": {
        "rows_total": len(rows),
        "distinct_source_files": len(by_source),
        "by_kind": dict(sorted(by_kind.items(), key=lambda kv: -kv[1])),
        "by_source": dict(sorted(by_source_group.items(), key=lambda kv: -kv[1])),
        "stale_source_rows": sum(1 for r in rows if r.get("stale_source")),
        "specs_total": len(anchors),
        "specs_failing_anchor": len(failed_anchors),
    },
    "anchor_report": anchors,
    "invalid_kinds": bad_kind,
    "rows": rows,
    "search_index": search_index,
}

os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf-8") as fh:
    json.dump(artifact, fh, indent=1, ensure_ascii=False)

print("rows:", len(rows))
print("specs:", len(anchors), "failing anchor:", len(failed_anchors))
for a in failed_anchors:
    print("  ANCHOR-FAIL", a["spec"], a["source"], "rows=%d expected>=%d" % (a["rows"], a["expected_min"]),
          a.get("note", ""))
print("by_kind:")
for k, v in sorted(by_kind.items(), key=lambda kv: -kv[1]):
    print("  %-22s %d" % (k, v))
print("by_source:")
for k, v in sorted(by_source_group.items(), key=lambda kv: -kv[1]):
    print("  %-55s %d" % (k, v))
print("invalid kinds:", bad_kind)
print("wrote", OUT)
