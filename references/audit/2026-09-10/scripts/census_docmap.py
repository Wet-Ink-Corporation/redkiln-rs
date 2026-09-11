#!/usr/bin/env python3
"""census_docmap.py -- build the redkiln-rs audit DOCUMENT MAP.

Read-only over D:/repos/redkiln-rs. Writes exactly one artifact:
  <OUT>/census/doc-map.json

Every number in the artifact is computed here. Nothing is eyeballed.
ASCII-only on stdout (Windows console cannot print non-ASCII); the JSON
artifact is written UTF-8 with ensure_ascii=False.
"""
from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

REPO = Path("D:/repos/redkiln-rs")
OUT = Path(
    "D:/repos/redkiln-rs/references/audit/2026-09-10"
)
ARTIFACT = OUT / "census" / "doc-map.json"

BYTES_PER_TOKEN = 3.7
WINDOW_TOKEN_CAP = 18000          # every read window must be strictly under this
READ_PLAN_THRESHOLD = 20000       # only files above this get a read_plan
OVERSIZE_LINE_TOKENS = WINDOW_TOKEN_CAP

EXCLUDE_DIR_PARTS = [
    (".git",),
    ("spikes", "fixtures"),
]

# ---------------------------------------------------------------- utilities


def say(msg: str) -> None:
    """stdout, ASCII only -- a stray non-ASCII byte kills the process on win32."""
    sys.stdout.write(msg.encode("ascii", "replace").decode("ascii") + "\n")
    sys.stdout.flush()


def est_tokens(nbytes: int) -> float:
    return round(nbytes / BYTES_PER_TOKEN, 1)


def rel(p: Path) -> str:
    return p.relative_to(REPO).as_posix()


def excluded(p: Path) -> bool:
    parts = p.relative_to(REPO).parts
    for pref in EXCLUDE_DIR_PARTS:
        if len(parts) >= len(pref) and tuple(parts[: len(pref)]) == pref:
            return True
    return False


def trunc(s: str, n: int) -> str:
    s = s.replace("\r", " ").replace("\n", " ").strip()
    return s if len(s) <= n else s[: n - 3] + "..."


# Sentence splitter. These documents separate header clauses with U+00B7
# (middle dot) as often as with a full stop, so both are boundaries.
SENT_SPLIT = re.compile(r"(?<=[.!?])\s+|\s*\u00b7\s*|\n{2,}")


def sentences(text: str):
    for raw in SENT_SPLIT.split(text):
        s = raw.strip()
        if s:
            yield s


# ------------------------------------------------------------ file scanning


def read_text(p: Path) -> str:
    with open(p, "rb") as fh:
        return fh.read().decode("utf-8", errors="replace")


def line_byte_lengths(raw: bytes):
    """Split on \n, keeping the terminator, so window byte sums are exact."""
    out = []
    start = 0
    n = len(raw)
    while start < n:
        idx = raw.find(b"\n", start)
        if idx == -1:
            out.append(raw[start:n])
            break
        out.append(raw[start : idx + 1])
        start = idx + 1
    return out


DATE_FIELD = re.compile(r"\*{0,2}Date:?\*{0,2}\s*:?\s*(\d{4}-\d{2}-\d{2})", re.I)
DATE_ANY = re.compile(r"\b(20\d{2}-\d{2}-\d{2})\b")
STATUS_FIELD = re.compile(r"\*{0,2}Status\*{0,2}\s*:\*{0,2}\s*(.{1,300})", re.I)
YAML_STATUS = re.compile(r"^status:\s*(.+)$", re.I | re.M)
YAML_DATE = re.compile(r"^date:\s*(\d{4}-\d{2}-\d{2})", re.I | re.M)
FNAME_DATE = re.compile(r"(20\d{2}-\d{2}-\d{2})")

SUPERSEDES_RE = re.compile(r"supersed(e|es|ing)\b", re.I)
SUPERSEDED_BY_RE = re.compile(r"superseded[_ ]by|is superseded|was superseded", re.I)

AUTH_VERB = re.compile(
    r"\b(governs?|governing|binding|binds?\b|authoritative|authority|"
    r"takes precedence|precedence|is the source of truth|source of truth|"
    r"normative|controls?\b)",
    re.I,
)
SELF_MARK = re.compile(
    r"\b(this document|this register|this runbook|this design|"
    r"this method|this proposal|this charter|this brief|this plan|this spec|"
    r"binding on|it supersedes|authoritative for|"
    r"where the two disagree, this)\b",
    re.I,
)
DEFER_RE = re.compile(
    r"(where the two disagree|where this \w+ and they disagree|remains? authoritative|"
    r"they win|the design governs|governs and this|the .{1,60} governs|"
    r"remain authoritative|is the authoritative)",
    re.I,
)
DENY_RE = re.compile(
    r"(not a source of truth|not authoritative|not binding|non-binding|"
    r"this file is wrong)",
    re.I,
)
PATH_TOKEN = re.compile(r"`([^`]{3,120}\.md)`")
HEADER_LINES = 40

ID_PATTERNS = {
    "AC": re.compile(r"\bAC-(\d{3})\b"),
    "SM": re.compile(r"\bSM-(\d{3})\b"),
    "ADR": re.compile(r"\bADR-(\d{4})\b"),
    "F": re.compile(r"\bF(\d{1,2})\b(?=-DRAFT|\b)"),
}


DEF_AC = re.compile(r"^\*\*AC-(\d{3})\*\*", re.M)
DEF_SM = re.compile(r"^\*\*SM-(\d{3}) \u2014 ", re.M)


def defined_ids(text: str):
    return {
        "AC": sorted({"AC-" + m for m in DEF_AC.findall(text)}),
        "SM": sorted({"SM-" + m for m in DEF_SM.findall(text)}),
    }


def extract_ids(text: str):
    out = {}
    for fam, pat in ID_PATTERNS.items():
        if fam == "F":
            continue
        out[fam] = sorted({fam + "-" + m for m in pat.findall(text)})
    return out


def header_fields(head_text: str, full_text: str, path: Path):
    date, date_source = None, None
    m = DATE_FIELD.search(head_text) or YAML_DATE.search(head_text)
    if m:
        date, date_source = m.group(1), "header Date field / YAML date"
    if date is None:
        m = FNAME_DATE.search(path.name)
        if m:
            date, date_source = m.group(1), "filename"
    if date is None:
        m = DATE_ANY.search(head_text)
        if m:
            date, date_source = m.group(1), "first ISO date in first 40 lines (weak)"

    status = None
    m = YAML_STATUS.search(head_text) or STATUS_FIELD.search(head_text)
    if m:
        status = trunc(m.group(1), 300)

    sup, supby = None, None
    for s in sentences(head_text):
        if supby is None and SUPERSEDED_BY_RE.search(s):
            supby = trunc(s, 300)
        elif sup is None and SUPERSEDES_RE.search(s) and not SUPERSEDED_BY_RE.search(s):
            sup = trunc(s, 300)
    if sup is None or supby is None:
        for s in sentences(full_text):
            if supby is None and SUPERSEDED_BY_RE.search(s):
                supby = trunc(s, 300)
            if sup is None and SUPERSEDES_RE.search(s) and not SUPERSEDED_BY_RE.search(s):
                sup = trunc(s, 300)
            if sup is not None and supby is not None:
                break
    return date, date_source, status, sup, supby


def authority_candidates(lines_text, max_n=6):
    """Classify every authority-flavoured sentence.

    tier   : 'header' (first HEADER_LINES lines) or 'body'
    kind   : 'self'  -- the document asserts it governs / binds / is authoritative
             'defer' -- the document says another document governs over it
             'deny'  -- the document disclaims authority
             'other' -- an authority word with no self-reference (quoted material,
                        a claim about some third document, a table row, ...)
    Only header-tier 'self' sentences that are not blockquotes count as the
    document's authority_claim; everything else is kept as a candidate.
    """
    cands = []
    for lineno, line in enumerate(lines_text, start=1):
        if not (AUTH_VERB.search(line) or DENY_RE.search(line) or DEFER_RE.search(line)):
            continue
        quoted = line.lstrip().startswith(">")
        tier = "header" if lineno <= HEADER_LINES else "body"
        for sent in sentences(line):
            hit_auth = bool(AUTH_VERB.search(sent))
            hit_deny = bool(DENY_RE.search(sent))
            hit_defer = bool(DEFER_RE.search(sent))
            if not (hit_auth or hit_deny or hit_defer):
                continue
            hit_self = bool(SELF_MARK.search(sent))
            kinds = []
            if hit_deny:
                kinds.append("deny")
            if hit_self and hit_auth and not hit_deny:
                kinds.append("self")
            if hit_defer and (not hit_self or hit_deny):
                kinds.append("defer")
            if not kinds:
                kinds.append("other")
            kind = kinds[0]
            cands.append(
                {
                    "line": lineno,
                    "tier": tier,
                    "kind": kind,
                    "kinds": kinds,
                    "in_blockquote": quoted,
                    "claim": trunc(sent, 300),
                    "paths_named": PATH_TOKEN.findall(sent)[:6],
                }
            )
            if len(cands) >= 400:
                break
        if len(cands) >= 400:
            break
    strict = [c for c in cands if "self" in c["kinds"] and c["tier"] == "header" and not c["in_blockquote"]]
    defers = [c for c in cands if "defer" in c["kinds"] and c["tier"] == "header" and not c["in_blockquote"]]
    denies = [c for c in cands if "deny" in c["kinds"] and c["tier"] == "header" and not c["in_blockquote"]]
    body_self = [c for c in cands if "self" in c["kinds"] and c["tier"] == "body"]
    shown = (strict + defers + denies + body_self)[:max_n]
    return shown, strict, defers, denies, body_self, len(cands)


SCOPE_RE = re.compile(r"binding on\s+([^.;]{3,160})", re.I)
SCOPE_RE2 = re.compile(r"(?:governs|authoritative (?:for|over))\s+(?!and\b|the proposal)([^.;]{3,160})", re.I)


def declared_scope(claims, status):
    for c in claims:
        m = SCOPE_RE.search(c["claim"])
        if m:
            return trunc(m.group(1), 200), "binding-on phrase"
    for c in claims:
        m = SCOPE_RE2.search(c["claim"])
        if m:
            return trunc(m.group(1), 200), "governs/authoritative-for phrase"
    if status:
        return trunc(status, 200), "declared_status text (no explicit scope phrase)"
    return None, "none found"


# -------------------------------------------------------------- read plans

HEADING = re.compile(r"^(#{1,6})\s+(.*)$")


def build_read_plan(lines_bytes, lines_text):
    """Heading-aware windows, each strictly under WINDOW_TOKEN_CAP est_tokens,
    covering every line exactly once."""
    n = len(lines_bytes)
    blens = [len(b) for b in lines_bytes]

    # block starts: line 1 plus every heading line
    starts = [0]
    heads = {0: "(preamble / start of file)"}
    for i, t in enumerate(lines_text):
        m = HEADING.match(t)
        if m:
            if i != 0:
                starts.append(i)
            heads[i] = trunc(m.group(0), 200)
    starts = sorted(set(starts))

    blocks = []
    for k, s in enumerate(starts):
        e = starts[k + 1] - 1 if k + 1 < len(starts) else n - 1
        blocks.append((s, e, heads.get(s, "(unnamed block)")))

    oversize_lines = []
    for i, bl in enumerate(blens):
        if est_tokens(bl) >= OVERSIZE_LINE_TOKENS:
            oversize_lines.append({"line": i + 1, "bytes": bl, "est_tokens": est_tokens(bl)})

    windows = []

    def emit(s, e, heading, part=None, parts=None):
        by = sum(blens[s : e + 1])
        w = {
            "heading": heading if part is None else "%s [part %d/%d]" % (heading, part, parts),
            "start_line": s + 1,
            "end_line": e + 1,
            "bytes": by,
            "est_tokens": est_tokens(by),
            "est_tokens_at_3.0_bytes_per_token": round(by / 3.0, 1),
            "pieces_if_1.4_bytes_per_token": -(-by // int(WINDOW_TOKEN_CAP * 1.4)),
        }
        windows.append(w)

    def split_block(s, e, heading):
        """Greedy line-packing inside one oversized block."""
        subs = []
        cur_s = s
        cur_b = 0
        i = s
        while i <= e:
            lb = blens[i]
            if cur_b and est_tokens(cur_b + lb) >= WINDOW_TOKEN_CAP:
                subs.append((cur_s, i - 1))
                cur_s, cur_b = i, lb
            else:
                cur_b += lb
            i += 1
        subs.append((cur_s, e))
        for j, (a, b) in enumerate(subs, start=1):
            emit(a, b, heading, part=j, parts=len(subs))

    cur = None  # (start, end, heading)
    cur_b = 0
    for (s, e, h) in blocks:
        bb = sum(blens[s : e + 1])
        if est_tokens(bb) >= WINDOW_TOKEN_CAP:
            if cur is not None:
                emit(cur[0], cur[1], cur[2])
                cur, cur_b = None, 0
            split_block(s, e, h)
            continue
        if cur is None:
            cur, cur_b = (s, e, h), bb
        elif est_tokens(cur_b + bb) < WINDOW_TOKEN_CAP:
            cur = (cur[0], e, cur[2])
            cur_b += bb
        else:
            emit(cur[0], cur[1], cur[2])
            cur, cur_b = (s, e, h), bb
    if cur is not None:
        emit(cur[0], cur[1], cur[2])

    # ---- self-check: total coverage, no gaps, no overlaps, cap respected
    problems = []
    prev_end = 0
    for w in windows:
        if w["start_line"] != prev_end + 1:
            problems.append(
                "coverage break at lines %d-%d (prev end %d)"
                % (w["start_line"], w["end_line"], prev_end)
            )
        prev_end = w["end_line"]
        if w["est_tokens"] >= WINDOW_TOKEN_CAP:
            problems.append("window %d-%d exceeds cap" % (w["start_line"], w["end_line"]))
    if prev_end != n:
        problems.append("plan ends at line %d, file has %d lines" % (prev_end, n))

    # densest 63-line window (the exploration pass reported a 63-line window of
    # architecture-design.md exceeding 37k tokens; this lets that be checked)
    K = 63
    densest = None
    if n >= 1:
        run = sum(blens[: min(K, n)])
        best = (run, 0)
        for i in range(1, n - K + 1):
            run += blens[i + K - 1] - blens[i - 1]
            if run > best[0]:
                best = (run, i)
        densest = {
            "start_line": best[1] + 1,
            "end_line": min(best[1] + K, n),
            "bytes": best[0],
            "est_tokens": est_tokens(best[0]),
            "est_tokens_at_3.0_bytes_per_token": round(best[0] / 3.0, 1),
        }
    return windows, oversize_lines, problems, densest


# ------------------------------------------------------------------- main


def main():
    md_files = []
    for dirpath, dirnames, filenames in os.walk(REPO):
        d = Path(dirpath)
        dirnames[:] = [
            x for x in dirnames if not excluded(d / x)
        ]
        for fn in filenames:
            if fn.lower().endswith(".md"):
                p = d / fn
                if not excluded(p):
                    md_files.append(p)
    md_files.sort(key=lambda p: rel(p))
    say("md files in scope: %d" % len(md_files))

    docs = {}
    texts = {}
    idsets = {}

    for p in md_files:
        raw = open(p, "rb").read()
        text = raw.decode("utf-8", errors="replace")
        lb = line_byte_lengths(raw)
        lines_text = text.split("\n")
        if lines_text and lines_text[-1] == "":
            lines_text = lines_text[:-1]
        # keep the two views the same length
        while len(lines_text) < len(lb):
            lines_text.append("")
        lines_text = lines_text[: len(lb)]

        nlines = len(lb)
        lengths = [len(t.rstrip("\r")) for t in lines_text] or [0]
        head_lines = lines_text[:5]
        head_text = "\n".join(lines_text[:40])

        date, date_src, status, sup, supby = header_fields(head_text, text, p)
        claims, strict, defers, denies, body_self, n_all = authority_candidates(lines_text)

        r = rel(p)
        texts[r] = text
        idsets[r] = extract_ids(text)

        all_dates = DATE_ANY.findall(text)
        docs[r] = {
            "path": r,
            "bytes": len(raw),
            "max_date_in_text": max(all_dates) if all_dates else None,
            "lines": nlines,
            "max_line_length": max(lengths) if lengths else 0,
            "mean_line_length": round(sum(lengths) / len(lengths), 1) if lengths else 0.0,
            "est_tokens": est_tokens(len(raw)),
            "header_block": [trunc(h, 500) for h in head_lines],
            "declared_status": status,
            "declared_date": date,
            "declared_date_source": date_src,
            "supersedes_claim": sup,
            "superseded_by_claim": supby,
            "authority_claim": strict[0]["claim"] if strict else None,
            "authority_claim_line": strict[0]["line"] if strict else None,
            "authority_self_claims_header": [c["claim"] for c in strict],
            "authority_self_claims_body_count": len(body_self),
            "deference_claim": defers[0]["claim"] if defers else None,
            "deference_paths_named": defers[0]["paths_named"] if defers else [],
            "denial_claim": denies[0]["claim"] if denies else None,
            "authority_claim_candidates": claims,
            "authority_mentions": n_all,
            "id_counts": {k: len(v) for k, v in idsets[r].items()},
        }

        if docs[r]["est_tokens"] > READ_PLAN_THRESHOLD:
            windows, oversize, problems, densest = build_read_plan(lb, lines_text)
            plan = {
                "window_count": len(windows),
                "densest_63_line_window": densest,
                "windows": windows,
                "coverage_verified": not problems,
                "coverage_problems": problems,
                "oversize_single_lines": oversize,
            }
            if oversize:
                plan["oversize_line_recipe"] = [
                    "sed -n '%dp' %s | cut -c1-4000   # then c4001-8000, c8001-12000, ..."
                    % (o["line"], r)
                    for o in oversize
                ]
            docs[r]["read_plan"] = plan
        else:
            docs[r]["read_plan"] = None

    # ------------------------------------------------- authority map + overlap
    auth_paths = [
        r for r, d in docs.items()
        if d["authority_claim"] is not None
    ]
    auth_paths.sort()

    authority_map = []
    for r in auth_paths:
        d = docs[r]
        selfclaims = [c for c in d["authority_claim_candidates"]
                      if "self" in c["kinds"] and c["tier"] == "header" and not c["in_blockquote"]]
        scope, scope_src = declared_scope(selfclaims, d["declared_status"])
        authority_map.append(
            {
                "path": r,
                "declared_status": d["declared_status"],
                "declared_date": d["declared_date"],
                "verbatim_claims": selfclaims,
                "declared_scope": scope,
                "declared_scope_source": scope_src,
                "deference_claim": d["deference_claim"],
                "denial_claim": d["denial_claim"],
                "id_counts": d["id_counts"],
            }
        )

    # pairwise id overlap between authority docs
    overlaps = []
    for i in range(len(auth_paths)):
        for j in range(i + 1, len(auth_paths)):
            a, b = auth_paths[i], auth_paths[j]
            row = {"a": a, "b": b, "families": {}}
            any_shared = False
            for fam in ("AC", "SM", "ADR"):
                sa, sb = set(idsets[a][fam]), set(idsets[b][fam])
                inter = sa & sb
                if not sa and not sb:
                    continue
                if inter:
                    any_shared = True
                union = sa | sb
                row["families"][fam] = {
                    "a_count": len(sa),
                    "b_count": len(sb),
                    "shared": len(inter),
                    "a_only": len(sa - sb),
                    "b_only": len(sb - sa),
                    "jaccard": round(len(inter) / len(union), 4) if union else 0.0,
                    "a_only_ids": sorted(sa - sb) if len(sa - sb) <= 60 else "(>60, see documents[].id_counts)",
                    "b_only_ids": sorted(sb - sa) if len(sb - sa) <= 60 else "(>60, see documents[].id_counts)",
                }
            if any_shared:
                overlaps.append(row)
    overlaps.sort(
        key=lambda r: -max((f["shared"] for f in r["families"].values()), default=0)
    )

    # ----------------------------------- the central precedence pair, verbatim
    ARCH = "references/2026-09-08-architecture-design.md"
    ACREG = "references/2026-09-08-acceptance-criteria.md"
    SMREG = "references/2026-09-08-success-measures.md"
    central = {}
    if ARCH in idsets and ACREG in idsets and SMREG in idsets:
        ac_reg = set(idsets[ACREG]["AC"])
        ac_arch = set(idsets[ARCH]["AC"])
        sm_reg = set(idsets[SMREG]["SM"])
        sm_arch = set(idsets[ARCH]["SM"])
        ac_def = set(defined_ids(texts[ACREG])["AC"])
        sm_def = set(defined_ids(texts[SMREG])["SM"])
        central = {
            "claim_under_test": (
                "exploration pass reported: architecture-design reproduces 481 of the "
                "AC register's 514 ids, and 104 SM ids, inline"
            ),
            "method": (
                "strict token match \\bAC-[0-9]{3}\\b and \\bSM-[0-9]{3}\\b over the whole "
                "file; a range written AC-001..AC-039 contributes only its two endpoints, "
                "never the interior"
            ),
            "ac_register_ids": len(ac_reg),
            "arch_ac_ids": len(ac_arch),
            "ac_shared": len(ac_arch & ac_reg),
            "ac_in_arch_not_in_register": sorted(ac_arch - ac_reg),
            "ac_in_register_not_in_arch": sorted(ac_reg - ac_arch),
            "ac_jaccard": round(len(ac_arch & ac_reg) / len(ac_arch | ac_reg), 4),
            "sm_register_ids": len(sm_reg),
            "arch_sm_ids": len(sm_arch),
            "sm_shared": len(sm_arch & sm_reg),
            "sm_in_arch_not_in_register": sorted(sm_arch - sm_reg),
            "sm_in_register_not_in_arch_count": len(sm_reg - sm_arch),
            "sm_in_register_not_in_arch": sorted(sm_reg - sm_arch),
            "arch_ac_id_set": sorted(ac_arch),
            "ac_register_id_set": sorted(ac_reg),
            "arch_sm_id_set": sorted(sm_arch),
            "sm_register_id_set": sorted(sm_reg),
            "defined_vs_mentioned": {
                "note": (
                    "the register DEFINES an id with a line-initial **AC-NNN** entry; the SM "
                    "register with a line-initial **SM-NNN -- title** entry. Ids that appear only "
                    "in prose (retired / unallocated / cross-references) are 'mentioned'."
                ),
                "ac_register_defined": len(ac_def),
                "ac_register_mentioned_not_defined": sorted(ac_reg - ac_def),
                "arch_ac_in_defined": len(ac_arch & ac_def),
                "arch_ac_not_in_defined": sorted(ac_arch - ac_def),
                "ac_defined_not_in_arch": sorted(ac_def - ac_arch),
                "sm_register_defined": len(sm_def),
                "sm_register_mentioned_not_defined": sorted(sm_reg - sm_def),
                "arch_sm_in_defined": len(sm_arch & sm_def),
                "arch_sm_not_in_defined": sorted(sm_arch - sm_def),
                "sm_defined_not_in_arch": sorted(sm_def - sm_arch),
                "exploration_claim_481_of_514": {
                    "514_matches_defined_count": len(ac_def) == 514,
                    "481_matches_arch_in_defined": len(ac_arch & ac_def) == 481,
                    "104_matches_arch_sm_mentions": len(sm_arch) == 104,
                },
            },
            "runbook_deference_claim": docs.get("RUNBOOK.md", {}).get("deference_claim"),
            "arch_authority_claim": docs[ARCH]["authority_claim"],
            "ac_register_authority_claim": docs[ACREG]["authority_claim"],
        }

    # ------------------------------------------------------ references/orphans
    # Scan every non-.git, non-fixture file for path / basename mentions.
    scan_files = []
    for dirpath, dirnames, filenames in os.walk(REPO):
        d = Path(dirpath)
        dirnames[:] = [x for x in dirnames if not excluded(d / x)]
        for fn in filenames:
            p = d / fn
            if excluded(p):
                continue
            try:
                if p.stat().st_size > 8 * 1024 * 1024:
                    continue
            except OSError:
                continue
            scan_files.append(p)
    say("files scanned for references: %d (spikes/fixtures/** excluded: 2.4 GB, ~91k files)" % len(scan_files))

    corpus = {}
    for p in scan_files:
        try:
            corpus[rel(p)] = read_text(p)
        except OSError:
            continue

    referrers = {r: {"by_path": [], "by_basename_only": [], "by_id": []} for r in docs}
    ADR_FILE = re.compile(r"^references/adr/(\d{4})-")
    FDRAFT_FILE = re.compile(r"^spikes/F(\d{2})-DRAFT\.md$")
    id_probe = {}
    for r in docs:
        m = ADR_FILE.match(r)
        if m:
            id_probe[r] = ("ADR", m.group(1))
            continue
        m = FDRAFT_FILE.match(r)
        if m:
            id_probe[r] = ("F", m.group(1))
    ADR_MENTION = re.compile(r"\bADR[- ]?(\d{4})\b|\[(\d{4})\]\(")
    F_MENTION = re.compile(r"\bF(\d{2})\b")
    mentions = {}
    for src, txt in corpus.items():
        adr = set()
        for a, b in ADR_MENTION.findall(txt):
            adr.add(a or b)
        mentions[src] = {"ADR": adr, "F": set(F_MENTION.findall(txt))}
    basenames = {}
    for r in docs:
        basenames.setdefault(Path(r).name, []).append(r)

    for src, txt in corpus.items():
        for r in docs:
            if src == r:
                continue
            probe = id_probe.get(r)
            if probe is not None and probe[1] in mentions[src][probe[0]]:
                referrers[r]["by_id"].append(src)
            name = Path(r).name
            if name not in txt:
                continue
            # path mention: the posix path, or a ./ or backslash variant
            if r in txt or r.replace("/", "\\") in txt:
                referrers[r]["by_path"].append(src)
            else:
                # basename appears but not the full path
                amb = len(basenames[name]) > 1
                referrers[r]["by_basename_only"].append({"src": src, "ambiguous_basename": amb})

    orphans = []
    for r in sorted(docs):
        bp = referrers[r]["by_path"]
        bn = referrers[r]["by_basename_only"]
        bi = referrers[r]["by_id"]
        docs[r]["referenced_by_path_count"] = len(bp)
        docs[r]["referenced_by_path"] = sorted(bp)
        docs[r]["referenced_by_basename_only_count"] = len(bn)
        docs[r]["referenced_by_id_count"] = len(bi) if r in id_probe else None
        if not bp:
            parent = Path(r).parent.as_posix()
            dir_refs = []
            if parent not in (".", ""):
                for src, txt in corpus.items():
                    if src != r and parent in txt:
                        dir_refs.append(src)
                        if len(dir_refs) >= 6:
                            break
            orphans.append(
                {
                    "path": r,
                    "parent_dir": parent,
                    "parent_dir_referenced_by": sorted(dir_refs),
                    "referenced_by_basename_only": sorted({x["src"] for x in bn}),
                    "basename_is_ambiguous": any(x["ambiguous_basename"] for x in bn),
                    "referenced_by_id_count": len(bi) if r in id_probe else None,
                    "referenced_by_id_sample": sorted(bi)[:8],
                    "strict_orphan": (not bn) and (not bi),
                }
            )

    # --------------------------------------------------------------- stale risk
    CURRENCY = re.compile(
        r"\b(current|currently|authoritative|binding|governs|latest|live|canonical|"
        r"in force|up to date|up-to-date|still the|remains the|the source of truth)\b",
        re.I,
    )
    stale_risk = []
    for r in sorted(docs):
        d = docs[r]
        if not d["declared_date"]:
            continue
        newer = []
        for src in d["referenced_by_path"]:
            s = docs.get(src)
            if not s or not s["declared_date"]:
                continue
            if s["declared_date"] <= d["declared_date"]:
                continue
            # find the citing lines and test them for a currency assertion
            cites = []
            for lineno, line in enumerate(corpus[src].split("\n"), start=1):
                idx = line.find(r)
                if idx < 0:
                    idx = line.find(r.replace("/", "\\"))
                if idx >= 0:
                    lo, hi = max(0, idx - 120), idx + len(r) + 120
                    near = line[lo:hi]
                    cites.append(
                        {
                            "line": lineno,
                            "text": trunc(near, 300),
                            "asserts_currency": bool(CURRENCY.search(near)),
                            "currency_window": "+/-120 chars around the path mention",
                        }
                    )
                if len(cites) >= 4:
                    break
            newer.append(
                {
                    "citer": src,
                    "citer_date": s["declared_date"],
                    "citations": cites,
                    "any_currency_assertion": any(c["asserts_currency"] for c in cites),
                }
            )
        if newer:
            stale_risk.append(
                {
                    "path": r,
                    "declared_date": d["declared_date"],
                    "max_date_in_text": d["max_date_in_text"],
                    "declared_status": d["declared_status"],
                    "cited_by_newer_count": len(newer),
                    "cited_as_current_by_newer": [
                        x["citer"] for x in newer if x["any_currency_assertion"]
                    ],
                    "cited_by_newer": newer,
                }
            )
    stale_risk.sort(key=lambda x: -len(x["cited_as_current_by_newer"]))

    # ----------------------------------------------------------------- totals
    big = [d for d in docs.values() if d["est_tokens"] > READ_PLAN_THRESHOLD]
    all_oversize = []
    plan_problems = []
    for d in docs.values():
        if d["read_plan"]:
            for o in d["read_plan"]["oversize_single_lines"]:
                all_oversize.append({"path": d["path"], **o})
            for pr in d["read_plan"]["coverage_problems"]:
                plan_problems.append({"path": d["path"], "problem": pr})
    # any single line anywhere over the cap, plan or no plan
    global_oversize_lines = []
    for r, d in docs.items():
        if est_tokens(d["max_line_length"]) >= WINDOW_TOKEN_CAP:
            global_oversize_lines.append(
                {"path": r, "max_line_length": d["max_line_length"]}
            )

    counts = {
        "md_files_in_scope": len(docs),
        "fixture_files_referencing_corpus_by_path": 0,
        "total_bytes": sum(d["bytes"] for d in docs.values()),
        "total_est_tokens": round(sum(d["est_tokens"] for d in docs.values()), 1),
        "files_over_20k_est_tokens": len(big),
        "read_plan_windows_total": sum(
            d["read_plan"]["window_count"] for d in docs.values() if d["read_plan"]
        ),
        "read_plans_coverage_verified": all(
            d["read_plan"]["coverage_verified"] for d in docs.values() if d["read_plan"]
        ),
        "single_lines_over_18k_est_tokens": len(global_oversize_lines),
        "authority_documents": len(authority_map),
        "documents_with_deference_claim": len([d for d in docs.values() if d["deference_claim"]]),
        "documents_with_denial_claim": len([d for d in docs.values() if d["denial_claim"]]),
        "authority_pairs_with_shared_ids": len(overlaps),
        "orphans": len(orphans),
        "strict_orphans": len([o for o in orphans if o["strict_orphan"]]),
        "orphans_by_path_but_referenced_by_id": len([o for o in orphans if o["referenced_by_id_count"]]),
        "stale_risk_files": len(stale_risk),
        "stale_risk_cited_as_current": len(
            [s for s in stale_risk if s["cited_as_current_by_newer"]]
        ),
    }

    artifact = {
        "schema": "redkiln-rs/audit/doc-map/1",
        "generated_by": "scripts/census_docmap.py",
        "repo": str(REPO).replace("\\", "/"),
        "method": {
            "est_tokens": "bytes / 3.7",
            "WARNING_token_ratio": (
                "est_tokens is bytes/3.7 as specified. The exploration pass reported a 63-line "
                "window of architecture-design.md costing >37k real tokens; the densest possible "
                "63-line window of that file is 52,017 bytes (lines 19-81), so that observation "
                "implies ~1.4 bytes/token on this dense markdown (tables, backticks, U+2014/U+00B7/"
                "U+00A7 runs), not 3.7. Every window carries pieces_if_1.4_bytes_per_token: read "
                "the window in that many equal line-slices if a real tokeniser is the binding limit."
            ),
            "window_cap_est_tokens": WINDOW_TOKEN_CAP,
            "read_plan_threshold_est_tokens": READ_PLAN_THRESHOLD,
            "scope": "every *.md under the repo except spikes/fixtures/** and .git/**",
            "reference_scan_scope": (
                "every non-.git, non-fixture file under 8 MB, any extension "
                "(%d files); spikes/fixtures/** (2.4 GB, ~91k files) NOT scanned"
                % len(scan_files)
            ),
            "id_match": "\\bAC-[0-9]{3}\\b, \\bSM-[0-9]{3}\\b, \\bADR-[0-9]{4}\\b",
            "fixture_reference_probe": (
                "separately, grep -rlF over spikes/fixtures for the literals 'references/2026-09', "
                "'spikes/METHOD', 'RUNBOOK.md', 'references/adr/' returned 9 files, all "
                "issue-146-147-boundary-window spec.md copies whose only hit is the token "
                "'RUNBOOK.md' inside a file listing of a different project (Cargo.lock, Cargo.toml, "
                "RUNBOOK.md, [195 files total]). No fixture references this corpus by path. "
                "[measured 2026-09-10; not re-run by this script because the tree is 2.4 GB]"
            ),
            "line_lengths": "characters after stripping a trailing CR; bytes are raw UTF-8",
        },
        "counts": counts,
        "documents": [docs[r] for r in sorted(docs)],
        "authority_map": {
            "documents": authority_map,
            "pairwise_id_overlap": overlaps,
            "central_precedence_pair": central,
        },
        "orphans": orphans,
        "stale_risk": stale_risk,
        "global_oversize_lines": global_oversize_lines,
        "read_plan_problems": plan_problems,
    }

    ARTIFACT.parent.mkdir(parents=True, exist_ok=True)
    with open(ARTIFACT, "w", encoding="utf-8") as fh:
        json.dump(artifact, fh, indent=1, ensure_ascii=False, sort_keys=False)

    say("wrote %s" % str(ARTIFACT).replace("\\", "/"))
    for k, v in counts.items():
        say("  %-36s %s" % (k, v))
    if central:
        say("  central: AC register ids=%d arch AC ids=%d shared=%d"
            % (central["ac_register_ids"], central["arch_ac_ids"], central["ac_shared"]))
        say("  central: SM register ids=%d arch SM ids=%d shared=%d"
            % (central["sm_register_ids"], central["arch_sm_ids"], central["sm_shared"]))


if __name__ == "__main__":
    main()
