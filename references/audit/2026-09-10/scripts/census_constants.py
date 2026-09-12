"""Restated-constant census for the redkiln-rs planning corpus.

READ-ONLY over D:/repos/redkiln-rs. Scans every *.md except spikes/fixtures/**
and .git/**. Writes ONE artifact:

  <scratchpad>/audit/census/constants.json

Two tiers:
  1. SEEDS  - hand-labelled claim slots. Each slot has candidate VALUES, each
             with a keyword-anchored regex, in two roles:
               alternative  competing values for the SAME quantity
               component    parts of a bundled claim (different quantities)
             Status is computed:
               AGREE            at most one alternative value has occurrences
               VARIANT          two or more alternative values have occurrences
                                (may be legitimately different measurements -
                                flagged for the audit, NOT called a defect)
               RANGE-STATEMENT  the claim is an "X to Y" span / transition
               ABSENT           nothing matched (reported, never dropped)
             Some slots also carry a SLOT regex that captures whatever number sits
             next to the claim keyword; values not on the candidate list are
             surfaced UNFILTERED as `slot.other_values` (leads, not findings).
  2. GENERIC - every number-with-unit / percentage / ratio / "N of M" / money /
             multiplier / comma-grouped / semver token that occurs >= 3 times
             across >= 2 files. Tokens already covered by a seed candidate are
             marked covered_by_seed; the rest are the sweep's additions.

Stdout is ASCII-only (Windows console). Files are written UTF-8.
Re-run: python census_constants.py
"""
import collections
import datetime
import hashlib
import json
import os
import re
import subprocess
import sys

ROOT = r"D:\repos\redkiln-rs"
SCRATCH = r"D:/repos/redkiln-rs/references/audit/2026-09-10"
OUT_JSON = os.path.join(SCRATCH, "census", "constants.json")

EN, EM, TIMES, ARROW, GEQ, SECT, APPROX = (
    "\u2013", "\u2014", "\u00d7", "\u2192", "\u2265", "\u00a7", "\u2248")
DASH = "[-" + EN + EM + "]"          # hyphen / en-dash / em-dash
ARR = "(?:->|" + ARROW + "|to)"      # ASCII arrow / real arrow / "to"
MULT = r"\s?[x" + TIMES + r"](?![\w])"

CTX = 100          # chars of context on each side of a match
GENERIC_CTX_CAP = 20   # occurrences with context kept per generic token
GENERIC_MIN_OCC = 3
GENERIC_MIN_FILES = 2


# --------------------------------------------------------------------------
# corpus
# --------------------------------------------------------------------------
def corpus_files():
    out = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = sorted(d for d in dirnames if d not in (".git", "fixtures"))
        for fn in sorted(filenames):
            if fn.endswith(".md"):
                p = os.path.join(dirpath, fn)
                rel = os.path.relpath(p, ROOT).replace("\\", "/")
                out.append((rel, p))
    return sorted(out)


def load_corpus():
    lines = []
    for rel, path in corpus_files():
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            for i, line in enumerate(fh, 1):
                lines.append((rel, i, line.rstrip("\n")))
    return lines


def context(text, a, b):
    s = max(0, a - CTX)
    e = min(len(text), b + CTX)
    c = text[s:e]
    c = re.sub(r"\s+", " ", c)
    if s > 0:
        c = "..." + c
    if e < len(text):
        c = c + "..."
    return c


# --------------------------------------------------------------------------
# regex helpers for seed candidates
# --------------------------------------------------------------------------
def num(s):
    """Literal numeric string (commas optional in corpus) with number guards."""
    esc = re.escape(s).replace(",", ",?")
    return r"(?<![\w.,])" + esc + r"(?!\d)(?!\.\d)"


def pct(s):
    return num(s) + r"\s?%"


def money(s):
    return r"\$\s?" + num(s)


def mult(s):
    return num(s) + MULT


NEAR_MARK = "NEAR\x00"


def near(a, kw, gap=30):
    """`a` within `gap` chars before or after keyword regex `kw` (same line).

    Encoded as a marker string; the matcher reports the span of `a` ONLY, so
    overlap de-duplication keys on the number, not on the keyword."""
    return NEAR_MARK + str(gap) + "\x00" + a + "\x00" + kw


class NearMatcher:
    """finditer() over a NEAR marker: yields match-like objects spanning `a`."""

    class M:
        def __init__(self, s, e, text):
            self._s, self._e, self._t = s, e, text

        def start(self):
            return self._s

        def end(self):
            return self._e

        def group(self, i=0):
            return self._t[self._s:self._e]

    def __init__(self, marker):
        _, gap, a, kw = marker.split("\x00", 3)
        self.gap = int(gap)
        self.a = re.compile(a)
        self.kw = re.compile(kw)
        self.pattern = "NEAR(gap=%s, value=/%s/, keyword=/%s/)" % (gap, a, kw)

    EXT = 80  # room for the keyword text itself; the gap bounds only the distance

    def finditer(self, text):
        for m in self.a.finditer(text):
            b0 = max(0, m.start() - self.gap - self.EXT)
            before = text[b0:m.start()]
            after = text[m.end():m.end() + self.gap + self.EXT]
            ok = any(mk.start() <= self.gap for mk in self.kw.finditer(after)) or \
                any(len(before) - mk.end() <= self.gap for mk in self.kw.finditer(before))
            if ok:
                yield NearMatcher.M(m.start(), m.end(), text)


def compile_cand(pat):
    """A candidate is one pattern or a list of patterns; each is a regex or a NEAR marker."""
    pats = pat if isinstance(pat, list) else [pat]
    out = []
    for p in pats:
        if p.startswith(NEAR_MARK):
            out.append(NearMatcher(p))
        else:
            rx = re.compile(p)
            out.append(rx)
    return out


def describe_cand(compiled):
    return " | ".join(c.pattern for c in compiled)


def frac(a, b):
    """a/b or a of (the) b."""
    return r"(?:" + num(a) + r"\s?/\s?" + num(b) + r"|" + num(a) + \
        r"\s+of\s+(?:the\s+)?(?:its\s+)?" + num(b) + r")"


def words(*alts):
    return r"(?:" + "|".join(alts) + r")"


# --------------------------------------------------------------------------
# SEED TABLE
# --------------------------------------------------------------------------
# Each entry: id, group, label, kind, canonical, cands = {value_label: regex},
# optional slot = regex with (?P<v>...) capturing the value next to the claim
# keyword, optional note.
SEEDS = []


def seed(id_, group, label, kind, canonical, cands=None, parts=None, slot=None, note=None):
    """cands = COMPETING alternative values for the same quantity (2+ present => VARIANT).
    parts = COMPONENT values of a bundled claim (different quantities; never VARIANT)."""
    SEEDS.append({"id": id_, "group": group, "label": label, "kind": kind,
                  "canonical": canonical, "cands": cands or {}, "parts": parts or {},
                  "slot": slot, "note": note})


NUMV = r"(?<![\w.\-])(?P<v>\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?)"
NUMWORD = (r"(?<![\w-])(?P<v>\d+|[Oo]ne|[Tt]wo|[Tt]hree|[Ff]our|[Ff]ive|[Ss]ix|[Ss]even|[Ee]ight|[Nn]ine|[Tt]en|"
           r"[Ee]leven|[Tt]welve|[Tt]wenty(?:-\w+)?|[Tt]hirty(?:-\w+)?)")

# ---- ADR register ----------------------------------------------------------
seed("adr.total", "ADR register", "Number of ADR records in references/adr/", "count", "89",
     {"89": near(num("89"), r"ADR|record|decision|frontmatter", 30),
      "88": near(num("88"), r"records|ADRs|decisions", 12)},
     slot=NUMV + r"\s+(?:ADR records|records|decisions|frontmatters|ADRs)\b")
seed("adr.accepted", "ADR register", "ADRs at status accepted", "count", "86",
     {"86": near(num("86"), r"accept", 25),
      "87 (if 0079 counted superseded)": [near(num("87"), r"accept", 25), r"87 / 3 / 0"]},
     note="RUNBOOK.md:1342 itself reconciles 86/2/1 against 87/3/0 (ADR-0079 counted as superseded); the corpus records this.",
     slot=NUMV + r"\s+(?:`?accepted`?|are at status `accepted`)")
seed("adr.superseded", "ADR register", "ADRs at status superseded", "count", "2",
     {"2": near(words(num("2"), r"\btwo\b"), r"supersed", 12),
      "3": near(num("3"), r"supersed", 12)},
     slot=NUMV + r"\s+`?superseded`?")
seed("adr.proposed", "ADR register", "ADRs at status proposed", "count", "1",
     {"1": near(words(num("1"), r"\bone\b"), r"proposed", 12),
      "0": near(num("0"), r"proposed", 12)},
     slot=NUMV + r"\s+`?proposed`?")
seed("adr.cited", "ADR register", "ADRs cited by at least one criterion", "count", "56",
     {"56": near(num("56"), r"cited|\+ 33", 30)})
seed("adr.uncited", "ADR register", "ADRs cited by no criterion", "count", "33",
     {"33": [near(num("33"), r"not cited|uncited|no criterion|are not", 30), r"56 \+ 33"]})

# ---- architecture-design area tally ---------------------------------------
seed("areas.total", "Area tally", "Architecture-design areas (rows of section 0.1)", "count", "39",
     {"39": near(words(num("39"), r"[Tt]hirty-nine"), r"areas|rows", 12)})
seed("areas.decided", "Area tally", "Areas DECIDED", "count", "26",
     {"26": near(words(num("26"), r"[Tt]wenty-six"), r"DECIDED|decided", 8)},
     slot=NUMWORD + r"\s+(?:of thirty-nine areas are )?DECIDED")
seed("areas.provisional", "Area tally", "Areas PROVISIONAL", "count", "3",
     {"3": near(words(num("3"), r"\b[Tt]hree\b"), r"PROVISIONAL", 8)},
     slot=NUMWORD + r"\s+PROVISIONAL")
seed("areas.blocked", "Area tally", "Areas BLOCKED", "count", "10",
     {"10": near(words(num("10"), r"\b[Tt]en\b"), r"BLOCKED", 8)},
     slot=NUMWORD + r"\s+BLOCKED")

# ---- acceptance-criteria register ----------------------------------------
seed("crit.total", "AC register", "Criteria in force (total)", "count", "514",
     {"514": near(r"(?<!AC-)" + num("514"), r"criteria|total|in force", 30),
      "513 (rev. 1 snapshot)": near(r"(?<!AC-)" + num("513"), r"criteria|acceptance", 40)},
     note="513 appears in the 2026-09-09 handoff (pre AC-520 / rev. 1); 514 is the register's own total. Dated snapshot, not necessarily a defect.",
     slot=r"(?<!AC-)" + NUMV + r"\s+criteria in force")
seed("crit.binding", "AC register", "Criteria binding", "count", "469",
     {"469": near(r"(?<!AC-)" + num("469"), r"binding|total|below", 30)})
seed("crit.provisional", "AC register", "Criteria provisional", "count", "19",
     {"19": near(r"(?<!AC-)" + num("19"), r"provisional", 3)})
seed("crit.human_gated", "AC register", "Criteria human-gated", "count", "20",
     {"20": near(r"(?<!AC-)" + num("20"), r"human-gated", 3)})
seed("crit.conditional", "AC register", "Criteria conditional", "count", "6",
     {"6": near(r"(?<!AC-)" + num("6"), r"conditional", 3)})
seed("measures.total", "AC register", "Measure rows cross-referenced (SM register count)", "count", "135",
     {"135": near(r"(?<!SM-)(?<!AC-)" + num("135"), r"measure|rows|SM", 20),
      "134": near(r"(?<!SM-)(?<!AC-)" + num("134"), r"measure", 20)},
     slot=r"(?<![-\d])" + NUMV + r"\s+measure(?:s| rows)")

# ---- evidence brief tallies ----------------------------------------------
seed("eb.confirmations", "Evidence brief", "Confirmations", "count", "48",
     {"48": near(num("48"), r"confirmation", 20)},
     slot=NUMV + r"\s+confirmations")
seed("eb.contradictions", "Evidence brief", "Contradictions", "count", "13",
     {"13": near(num("13"), r"contradiction", 20)},
     slot=NUMV + r"\s+contradictions")
seed("eb.new_findings", "Evidence brief", "New findings", "count", "44",
     {"44": near(num("44"), r"new finding", 20)},
     slot=NUMV + r"\s+new findings")
seed("eb.unresolved", "Evidence brief", "Unresolved", "count", "30",
     {"30": near(num("30"), r"unresolved", 20)},
     slot=NUMV + r"\s+unresolved")

# ---- range statements ------------------------------------------------------
seed("range.findings", "Range statements", "Findings covered: F1 to F28 / F38 / F40", "range", "F1-F40",
     {"F1-F28": r"F1\s?" + DASH + r"\s?F28\b|F1 (?:to|through) F28\b",
      "F1-F38": r"F1\s?" + DASH + r"\s?F38\b|F1 (?:to|through) F38\b",
      "F1-F40": r"F1\s?" + DASH + r"\s?F40\b|F1 (?:to|through) F40\b"},
     note="Different documents legitimately freeze at different findings; VARIANT is expected, the audit decides which are stale.")
seed("range.brief_sections", "Range statements", "Evidence-brief section C span: C.1-C.13 vs C.14", "range", "C.1-C.14",
     {"C.1-C.13": r"C\.1\s?" + DASH + r"\s?C\.13\b",
      "C.1-C.14": r"C\.1\s?" + DASH + r"\s?C\.14\b"},
     parts={"C.14 (bare)": r"(?<![\w.])C\.14\b"})
seed("range.sm", "Range statements", "Success-measure id range", "range", "SM-001..SM-156",
     {"SM-001..SM-154": r"SM-001\s?(?:\.\.|" + DASH + r"|to|through)\s?SM-154\b",
      "SM-001..SM-156": r"SM-001\s?(?:\.\.|" + DASH + r"|to|through)\s?SM-156\b"},
     parts={"SM-157 (plus)": r"plus SM-157"})
seed("range.ac", "Range statements", "Acceptance-criteria id range", "range", "AC-001..AC-519",
     {"AC-001..AC-519": r"AC-001\s?(?:\.\.|" + DASH + r"|to|through)\s?AC-519\b",
      "AC-001..AC-520": r"AC-001\s?(?:\.\.|" + DASH + r"|to|through)\s?AC-520\b"},
     parts={"plus AC-520": r"plus AC-520"})
seed("range.adr_ids", "Range statements", "ADR id range", "range", "ADR-0001..ADR-0089",
     {"ADR-0001..ADR-0089": r"ADR-0001\s?(?:\.\.|" + DASH + r"|to|through)\s?ADR-0089\b",
      "ADR-0001..ADR-0088": r"ADR-0001\s?(?:\.\.|" + DASH + r"|to|through)\s?ADR-0088\b"})

# ---- P1 (classifier stratification) --------------------------------------
seed("p1.strata", "P1", "P1 stratum precisions 0.25/0.15/0.10/0.10 against 0.80", "vector", "0.25/0.15/0.10/0.10 vs 0.80",
     parts={"0.25/0.15/0.10/0.10": r"0\.25\s?/\s?0\.15\s?/\s?0\.10\s?/\s?0\.10",
      "0.80 (target)": near(num("0.80"), r"against|target|gate|threshold|bar", 20)})
seed("p1.strata_counts", "P1", "P1 stratum hit counts 5/20, 3/20, 2/20, 2/20", "vector", "5/20,3/20,2/20,2/20",
     parts={"5/20, 3/20, 2/20, 2/20": r"5/20,?\s?3/20,?\s?2/20,?\s?(?:and\s)?2/20",
      "5/20": r"(?<![\d.])5\s?/\s?20(?![\d])"})
seed("p1.dispatches", "P1", "P1 dispatch count", "count", "400",
     {"400": near(num("400"), r"dispatch", 12)},
     slot=NUMV + r"\s+dispatches")
seed("p1.cost", "P1", "P1 total spend", "money", "$21.37", {"$21.37": money("21.37")})
seed("p1d.cells", "P1 arm D", "Arm D truth cells", "count", "140",
     {"140": near(num("140"), r"cell|truth", 12)},
     slot=NUMV + r"\s+(?:truth )?cells")
seed("p1d.auth", "P1 arm D", "auth facet precision 0.00 [0.00, 0.26]", "interval", "0.00 [0.00, 0.26]",
     {"0.00 [0.00, 0.26]": r"0\.00\s?\[0\.00,\s?0\.26\]"})
seed("p1d.positives", "P1 arm D", "positives count 11", "count", "11",
     {"11": near(num("11"), r"positive", 12)})
seed("p1d.ui", "P1 arm D", "ui facet precision 1.00 [0.65, 1.00]", "interval", "1.00 [0.65, 1.00]",
     {"1.00 [0.65, 1.00]": r"1\.00\s?\[0\.65,\s?1\.00\]"})
seed("p1d.differ", "P1 arm D", "15 of 32 differ", "fraction", "15/32", {"15 of 32": frac("15", "32")})

# ---- P2 / F40 --------------------------------------------------------------
seed("p2.rows", "P2/F40", "P2 rows", "count", "41", {"41": near(num("41"), r"rows", 8)},
     slot=NUMV + r"\s+rows\b")
seed("p2.cases", "P2/F40", "P2 cases", "count", "12", {"12": near(num("12"), r"cases", 8)})
seed("p2.rows_36_41", "P2/F40", "36 of 41 rows", "fraction", "36/41", {"36 of 41": frac("36", "41")})
seed("p2.coarse", "P2/F40", "coarse-scope agreement 0.73 [0.58, 0.84]", "interval", "0.73 [0.58, 0.84]",
     {"0.73 [0.58, 0.84]": r"0\.73\s?\[0\.58,\s?0\.84\]"})
seed("p2.fine", "P2/F40", "fine-scope agreement 0.55 [0.38, 0.70]", "interval", "0.55 [0.38, 0.70]",
     {"0.55 [0.38, 0.70]": r"0\.55\s?\[0\.38,\s?0\.70\]"})
seed("p2.stale", "P2/F40", "coarse Stale 5 vs fine Stale 15", "pair", "5 vs 15",
     parts={"5 vs 15 Stale": r"(?:coarse Stale 5, fine Stale 15|\b5 Stale[^\n]{0,40}\b15 Stale|\b5\s+(?:vs\.?|versus|against)\s+15\b[^\n]{0,20}Stale|Stale[^\n]{0,20}\b5\s+(?:vs\.?|versus|against)\s+15\b)",
      "15 Stale": r"(?<![\d.])15\s+Stale",
      "5 Stale": r"(?<![\d.])5\s+Stale"})
seed("p2.flips", "P2/F40", "flips", "count", "10", {"10": near(num("10"), r"flip", 8)},
     slot=NUMV + r"\s+flips?\b")
seed("p2.abstention", "P2/F40", "abstention 8 of 41 = 19.5%", "fraction", "8/41 = 19.5%",
     parts={"8 of 41": frac("8", "41"), "19.5%": pct("19.5")})
seed("p2.both_scopes", "P2/F40", "rows scored under both scopes", "count", "33",
     {"33": near(num("33"), r"both|scopes", 20)})

# ---- P7 / F37 --------------------------------------------------------------
seed("p7.runs", "P7/F37", "P7 runs", "count", "61", {"61": near(num("61"), r"runs?", 6)},
     slot=NUMV + r"\s+runs\b")
seed("p7.records", "P7/F37", "P7 records", "count", "297", {"297": near(num("297"), r"record", 6)},
     slot=NUMV + r"\s+records\b")
seed("p7.cost", "P7/F37", "P7 total spend", "money", "$131.62", {"$131.62": money("131.62")})
seed("p7.ratio_2_64", "P7/F37", "2.64x ratio", "multiplier", "2.64x", {"2.64x": mult("2.64")})
seed("p7.per_run_cost", "P7/F37", "$2.69 vs $1.02 per-run cost", "pair", "$2.69 vs $1.02",
     parts={"$2.69": money("2.69"), "$1.02": money("1.02")})
seed("p7.33of33", "P7/F37", "33 of 33", "fraction", "33/33", {"33 of 33": frac("33", "33")})
seed("p7.findings", "P7/F37", "findings count 188", "count", "188", {"188": near(num("188"), r"finding", 12)},
     slot=NUMV + r"\s+findings\b")
seed("p7.pct78", "P7/F37", "78%", "pct", "78%", {"78%": pct("78")})
seed("p7.offered_used", "P7/F37", "deviation channel offered 71, used 0", "pair", "71 offered / 0 used",
     parts={"71 offered": near(num("71"), r"offer|times", 12), "0 used": r"(?:used\s+\*?\*?0\b|\b0\s+used|0 requests in 71 offers)"},
     slot=r"offered\s+\*?\*?(?P<v>\d+)\s+times")
seed("p7.cost_ratio", "P7/F37", "Mandatory-arm cost ratio: 3.66x (F37) vs 4.4x (decisions log)", "multiplier", "DISPUTED 3.66x vs 4.4x",
     {"3.66x": mult("3.66"),
      "4.4x": near(r"(?<!" + SECT + r")" + mult("4.4"), r"decisions log|recommended|mandatory|F37|3\.66|cost|\$9\.86", 60)},
     parts={"4.4x (homonym: Google error-amplification 17.2x vs 4.4x)": near(r"(?<!" + SECT + r")" + mult("4.4"), r"amplification|centralis|17\.2", 60)},
     note="THE COST RATIO DISPUTE. The corpus itself records this as UNRECONCILED; both values are censused. 4.4x also occurs as an unrelated prior-art figure (error amplification), separated out as a component.")

# ---- P8 / F38 --------------------------------------------------------------
seed("p8.draws", "P8/F38", "3 draws of 27+3", "count", "3 x (27+3)",
     parts={"27+3": r"27\s?\+\s?3\b", "3 draws": near(num("3"), r"draws", 4), "three draws": r"[Tt]hree draws"})
seed("p8.cost", "P8/F38", "$3.01 of $40 cap", "money", "$3.01 of $40",
     parts={"$3.01": money("3.01"), "$40": money("40")})
seed("p8.ratios", "P8/F38", "0.45x / 0.55x / 0.78x", "multiplier", "0.45x/0.55x/0.78x",
     parts={"0.45x": mult("0.45"), "0.55x": mult("0.55"), "0.78x": mult("0.78")})
seed("p8.tokens", "P8/F38", "~18,603 tokens", "count", "18,603", {"18,603": num("18,603")})
seed("p8.gap_minutes", "P8/F38", "gap_minutes 0.9, 28.4, 29.1", "vector", "0.9, 28.4, 29.1",
     parts={"0.9, 28.4, 29.1": r"0\.9,\s?28\.4,\s?29\.1", "gap_minutes": r"gap_minutes"})

# ---- F19 / F18 -------------------------------------------------------------
seed("f19.174", "F19/F18", "174/174", "fraction", "174/174", {"174/174": frac("174", "174")})
seed("f19.drift", "F19/F18", "12/174 = 7% drift", "fraction", "12/174 = 7%",
     parts={"12/174": frac("12", "174"), "7%": near(pct("7"), r"drift|12/174|174", 25)})
seed("f19.split", "F19/F18", "9/79 = 11% vs 3/95 = 3%", "pair", "9/79 vs 3/95",
     parts={"9/79": frac("9", "79"), "3/95": frac("3", "95"), "11%": near(pct("11"), r"9/79|79", 12), "3%": near(pct("3"), r"3/95|95", 12)})
seed("f19.3_6x", "F19/F18", "3.6x", "multiplier", "3.6x", {"3.6x": mult("3.6")})
seed("f18.gates_1h", "F19/F18", "45-48% of gates exceed 1 h", "range", "45-48%",
     {"45-48%": r"45\s?" + DASH + r"\s?48\s?%"})

# ---- F12 -------------------------------------------------------------------
seed("f12.pct", "F12", "20.3% (15/74) vs 21.6%", "pct", "20.3%",
     {"20.3%": pct("20.3")},
     parts={"15/74": frac("15", "74"), "21.6%": pct("21.6")},
     note="21.6% is the sealed-more-than-once rate, a different quantity from the 20.3% ever-rejected rate; both are components. AC:3232 carries an unrelated Wilson bound 6.1-20.3% (a homonym of the value).")
seed("f12.commits", "F12", "360 commits", "count", "360", {"360": near(num("360"), r"commit", 10)},
     slot=NUMV + r"\s+commits\b")
seed("f12.slices", "F12", "74 slices", "count", "74", {"74": near(num("74"), r"slice", 10)},
     slot=NUMV + r"\s+slices\b")

# ---- F23 / F26 -------------------------------------------------------------
seed("f23.detection", "F23/F26", "23/23 vs 23/23", "fraction", "23/23",
     parts={"23/23 vs 23/23": r"23/23\s+(?:vs\.?|versus|and|against)\s+23/23", "23/23": frac("23", "23")})
seed("f23.mcnemar", "F23/F26", "McNemar p = 0.625", "stat", "0.625", {"p=0.625": r"p\s?=\s?0\.625"})
seed("f23.cost_ratio", "F23/F26", "in-session 1.72x", "multiplier", "1.72x", {"1.72x": mult("1.72")})
seed("f23.costs", "F23/F26", "$97.39 vs $56.47", "pair", "$97.39 vs $56.47",
     parts={"$97.39": money("97.39"), "$56.47": money("56.47")})
seed("f26.20of20", "F23/F26", "20/20 in four cells", "fraction", "20/20",
     parts={"20/20": frac("20", "20"), "four cells": r"(?:four|4)\s+cells"})

# ---- F29 -------------------------------------------------------------------
seed("f29.draws", "F29", "60 draws", "count", "60", {"60": near(num("60"), r"draws", 6)},
     slot=NUMV + r"\s+draws\b")
seed("f29.cost", "F29", "$176.96", "money", "$176.96", {"$176.96": money("176.96")})
seed("f29.hours", "F29", "16.2 h", "duration", "16.2 h", {"16.2 h": num("16.2") + r"\s?h(?:ours?)?\b"})
seed("f29.19of19", "F29", "19/19 rejections on-target", "fraction", "19/19", {"19/19": frac("19", "19")})
seed("f29.means", "F29", "0.70 / 0.37 / 0.10 / 0.00", "vector", "0.70/0.37/0.10/0.00",
     {"0.70 / 0.37 / 0.10 / 0.00": r"0\.70\s?/\s?0\.37\s?/\s?0\.10\s?/\s?0\.00"})
seed("f29.3of10", "F29", "3 of 10", "fraction", "3/10", {"3 of 10": frac("3", "10")})

# ---- F31 / F24 -------------------------------------------------------------
seed("f31.52", "F31/F24", "52/52", "fraction", "52/52", {"52/52": frac("52", "52")})
seed("f31.14", "F31/F24", "14/14", "fraction", "14/14", {"14/14": frac("14", "14")})
seed("f31.0of23", "F31/F24", "0/23", "fraction", "0/23", {"0/23": frac("0", "23")})
seed("f31.plus74", "F31/F24", "+74%", "pct", "+74%", {"+74%": r"\+\s?74\s?%"})
seed("f31.1890", "F31/F24", "1,890/1,890 and 1,907/1,907", "fraction", "1,890 vs 1,907",
     parts={"1,890/1,890": frac("1,890", "1,890"), "1,907/1,907": frac("1,907", "1,907"),
      "1,890": num("1,890"), "1,907": num("1,907")},
     note="Two run sizes; likely two runs, not a contradiction. Flagged for the audit.")
seed("f24.1of23", "F31/F24", "1/23 MCC +0.978", "fraction", "1/23 / +0.978",
     parts={"1/23": frac("1", "23"), "+0.978": r"\+\s?0\.978"})
seed("f24.0of23_mcc", "F31/F24", "0/23 MCC +1.000", "stat", "+1.000", {"+1.000": r"\+\s?1\.000"})
seed("clean.13of23", "F31/F24", "CLEAN ground truth wrong for >=13 of 23 (57%)", "fraction", ">=13/23 = 57%",
     {">=13 of 23": r"(?:" + GEQ + r"|>=|at least\s)\s?\*?\*?" + frac("13", "23") + r"|" + frac("13", "23") + r"\s?(?:=\s?)?\(?57\s?%",
      ">=12 of 23": r"(?:" + GEQ + r"|>=|at least\s)\s?\*?\*?" + frac("12", "23") + r"|" + GEQ + r"12-of-23|" + frac("12", "23") + r"\s?(?:=\s?)?\(?52\s?%"},
     parts={"57%": near(pct("57"), r"23|CLEAN|ground|wrong", 30),
            "52% (ground-truth-wrong phrasing)": near(pct("52"), r"ground|wrong|corpus|rev\. 4|pinn", 40),
            "12/23 (strict block rate, different quantity)": frac("12", "23"),
            "13 of 23 (bare)": frac("13", "23")},
     note="Two values for the SAME claim (CLEAN ground truth wrong for >= N of 23): >=12/23 (52%) in architecture-proposal, design-recommendation, ADR-0015, ADR-0019; >=13/23 (57%) in later documents. success-measures.md records the 52% figure as cut/mis-labelled. 12/23 also names the strict block RATE, a different quantity - a homonym. Flagged for the audit.")

# ---- F11 -------------------------------------------------------------------
seed("f11.mutants", "F11", "9 of 10 mutants", "fraction", "9/10",
     {"9 of 10": frac("9", "10")})
seed("f11.headline", "F11", "93.3% headline / 90.0% test-reachable", "pct", "93.3% / 90.0%",
     parts={"93.3%": pct("93.3"), "90.0%": pct("90.0")})
seed("f11.323s", "F11", "323 s median per mutant", "duration", "323 s", {"323 s": num("323") + r"\s?s(?:ec(?:onds?)?)?\b"})
seed("f11.8of23", "F11", "8 of 23 items outside S0.1", "fraction", "8/23", {"8 of 23": frac("8", "23")})

# ---- F8 --------------------------------------------------------------------
seed("f8.servers", "F8", "12 MCP servers", "count", "12", {"12 MCP servers": num("12") + r"\s+MCP\s+servers"},
     slot=NUMV + r"\s+MCP servers")
seed("f8.tools", "F8", "~280 tools", "count", "280", {"280 tools": r"~?\s?" + num("280") + r"\s+tools"})
seed("f8.tokens", "F8", "8,243 -> 3,611", "range", "8,243 -> 3,611",
     parts={"8,243 -> 3,611": num("8,243") + r"\s?" + ARR + r"\s?" + num("3,611"), "8,243": num("8,243"), "3,611": num("3,611")})
seed("f8.mcp_share", "F8", "4,631 tokens = 56%", "pct", "4,631 = 56%",
     parts={"4,631": num("4,631"), "56%": near(pct("56"), r"4,?631|cache|MCP", 30)})
seed("f8.tools_cut", "F8", "280 -> 34", "range", "280 -> 34",
     parts={"280 -> 34": num("280") + r"\s?(?:tools\s?)?" + ARR + r"\s?" + num("34")})

# ---- F6 / F14 / F21 --------------------------------------------------------
seed("f6.quantised", "F6/F14", "utilization quantised to 0.01", "stat", "0.01",
     {"quantised 0.01": r"(?:quanti[sz]ed?[^\n]{0,30}0\.01|0\.01[^\n]{0,30}quanti)"})
seed("f14.dispatches", "F6/F14", "40 dispatches", "count", "40", {"40": near(num("40"), r"dispatch", 6)})
seed("f14.tokens", "F6/F14", "~1.9M tokens", "count", "1.9M", {"1.9M": r"1\.9\s?M\b"})
seed("f14.seven_day", "F6/F14", "seven_day 0.100 -> 0.100", "range", "0.100 -> 0.100",
     {"0.100 -> 0.100": r"0\.100\s?" + ARR + r"\s?0\.100"})
seed("f21.five_hour", "F6/F14", "five_hour per draw 0.0098 (F29) vs 0.077 (F21), 7.8x", "stat", "0.0098 vs 0.077",
     {"0.0098": num("0.0098"), "0.077": num("0.077")},
     parts={"7.8x": mult("7.8")},
     note="Corpus records F21's 0.077 as superseded by F29's 0.0098; both censused.")

# ---- F16 / S-WT ------------------------------------------------------------
seed("swt.share", "F16/S-WT", "share 1.000", "stat", "1.000", {"share 1.000": near(r"1\.000", r"share", 15)})
seed("swt.w1", "F16/S-WT", "w1 0.807 / 0.685", "stat", "0.807",
     parts={"0.807": num("0.807"), "0.685": num("0.685")})
seed("swt.tokens", "F16/S-WT", "tokens 29,680 / 29,619 / 18,603", "count", "29,680 / 29,619 / 18,603",
     {"29,680": num("29,680"), "29,619": num("29,619"), "18,603": num("18,603")},
     note="Three different runs' token counts; VARIANT is expected across runs.")
seed("swt.minutes", "F16/S-WT", "66.8 / 66.1 / 65.2 min", "duration", "66.8 / 66.1 / 65.2 min",
     {"66.8 min": num("66.8") + r"\s?min", "66.1 min": num("66.1") + r"\s?min", "65.2 min": num("65.2") + r"\s?min"})
seed("swt.49_7h", "F16/S-WT", "49.7 h", "duration", "49.7 h", {"49.7 h": num("49.7") + r"\s?h(?:ours?)?\b"})

# ---- F35 -------------------------------------------------------------------
seed("f35.unsat", "F35", "unsatisfiable 23.5% -> 11.4% -> 0.3%", "range", "23.5% -> 11.4% -> 0.3%",
     parts={"23.5%": pct("23.5"), "11.4%": pct("11.4"), "0.3%": pct("0.3"), "30.1%": pct("30.1")},
     note="30.1% is the scratch-run value the corpus mentions alongside 11.4%.")
seed("f35.dead", "F35", "dead activities 15/101 -> 6/102", "range", "15/101 -> 6/102",
     parts={"15/101": frac("15", "101"), "6/102": frac("6", "102")})
seed("f35.ties", "F35", "0 rank ties / 0 hard cycles", "count", "0",
     parts={"rank ties 0": r"(?:rank ties\s+\*?\*?0\b|\b0 rank ties|ties\s+\*?\*?0\b)", "hard cycles 0": r"(?:hard cycles\s+\*?\*?0\b|\b0 hard cycles|cycles\s+\*?\*?0\b)"})
seed("f35.27s", "F35", "lattice runtime 27 s", "duration", "27 s", {"27 s": num("27") + r"\s?s\b"})

# ---- F34 / F32 / F33 -------------------------------------------------------
seed("f34.activities", "F34/F32/F33", "90 activities", "count", "90", {"90": near(num("90"), r"activit", 6)},
     slot=NUMV + r"\s+activities\b")
seed("f34.median", "F34/F32/F33", "median 53", "stat", "53", {"median 53": near(num("53"), r"median", 15)})
seed("f34.lint", "F34/F32/F33", "0.11 vs 4.7 lint errors, 2.8x", "pair", "0.11 vs 4.7",
     parts={"0.11": near(num("0.11"), r"lint|error|4\.7", 40), "4.7": near(num("4.7"), r"lint|error|0\.11|P9", 40), "2.8x": mult("2.8")})
seed("f32.median", "F34/F32/F33", "median 56.5 / 73 / +29%", "stat", "56.5",
     {"56.5": num("56.5"), "73": near(num("73"), r"median|F33|\+\s?29", 20)},
     parts={"+29%": r"\+\s?29\s?%"},
     note="56.5 (F32, first author) vs 73 (F33, second author): two authors, legitimately different; flagged, not defective.")
seed("f32.kill", "F34/F32/F33", ">150 kill line", "threshold", "150",
     {">150": r"(?:>|" + GEQ + r"|vs|against|kill)\s?\*?\*?150\b"})

# ---- S-MIG -----------------------------------------------------------------
seed("smig.files", "S-MIG", "1,312 files / 6,877 records", "count", "1,312 / 6,877",
     parts={"1,312": num("1,312"), "6,877": num("6,877")},
     slot=NUMV + r"\s+files\b")
seed("smig.19of153", "S-MIG", "19 of 153 (12%)", "fraction", "19/153 = 12%",
     parts={"19 of 153": frac("19", "153"), "12%": near(pct("12"), r"153|19", 15)})
seed("smig.acs", "S-MIG", "45 ACs / 46 ACs", "count", "45 / 46",
     parts={"45 ACs": num("45") + r"\s+ACs", "46 ACs": num("46") + r"\s+ACs"},
     note="45 = ACs without GWT, 46 = ACs without a verification note; two different counts.")
seed("smig.39of156", "S-MIG", "39/156", "fraction", "39/156", {"39/156": frac("39", "156")})
seed("smig.19of23", "S-MIG", "19/23", "fraction", "19/23", {"19/23": frac("19", "23")})
seed("smig.110", "S-MIG", "110/110", "fraction", "110/110", {"110/110": frac("110", "110")})
seed("smig.98", "S-MIG", "98%", "pct", "98%", {"98%": pct("98")})

# ---- toolchain / pins ------------------------------------------------------
seed("pin.corpus", "Pins", "corpus pin 47faec9f", "hash", "47faec9f", {"47faec9f": r"\b47faec9f[0-9a-f]*\b"})
for v in ["2.1.242", "2.1.248", "2.1.251", "2.1.258", "2.1.259", "2.1.261", "2.1.263", "2.1.265", "2.1.266"]:
    seed("pin.cli." + v, "Pins", "Claude Code CLI version " + v, "version", v, {v: r"(?<![\d.])" + re.escape(v) + r"(?![\d.])"})
seed("pin.rustc", "Pins", "rustc 1.97.1", "version", "1.97.1", {"1.97.1": r"(?<![\d.])1\.97\.1(?![\d.])"})
seed("pin.gix", "Pins", "gix 0.87.1", "version", "0.87.1", {"0.87.1": r"(?<![\d.])0\.87\.1(?![\d.])"})

# ---- competitor star counts ------------------------------------------------
STARS = {"spec-kit 134,257": "134,257", "OpenSpec 67,690": "67,690", "BMAD-METHOD 52,802": "52,802",
         "claude-task-master 28,064": "28,064", "sum 282,813": "282,813", "23,407": "23,407",
         "Beads 26,983": "26,983", "283,457": "283,457", "Vibe Kanban 28,040": "28,040",
         "Roo-Code 24,306": "24,306", "Terragon 258": "258", "Crystal 3,115": "3,115",
         "11,495": "11,495", "6,676": "6,676"}
for lab, v in STARS.items():
    rx = num(v) if "," in v else near(num(v), r"star|" + "\u2605|\u2b50|repo", 10)
    seed("stars." + v.replace(",", ""), "Competitor stars", "GitHub star count " + lab, "count", v, {v: rx})
seed("stars.gastown", "Competitor stars", "Beads 26,983 vs Gas Town ~15.9K", "pair", "26,983 vs ~15.9K",
     parts={"26,983": num("26,983"), "~15.9K": r"~?\s?15\.9\s?[Kk]\b"})

# ---- promoted from the generic sweep (high-count restated literals) --------
seed("adr.dir_files", "ADR register", "ls references/adr/*.md count = 90 (89 records + README)", "count", "90",
     {"90": near(num("90"), r"README|wc -l|records \+", 30), "89 (old expectation)": r"expected 89 = 88 records \+ README"})
seed("gen.f10_ttl", "Promoted", "F10 cache TTL ~1 h", "duration", "~1 h",
     {"~1 h TTL": r"~\s?1\s?h(?:our)?\b[^\n]{0,6}TTL|TTL[^\n]{0,12}~\s?1\s?h\b"})
seed("gen.cost_112_25", "Promoted", "$112.25", "money", "$112.25", {"$112.25": money("112.25")})
seed("gen.cost_3_03", "Promoted", "$3.03", "money", "$3.03", {"$3.03": money("3.03")})
seed("gen.draw_4561s", "Promoted", "4,561 s draw", "duration", "4,561 s", {"4,561 s": num("4,561") + r"\s?s\b"})
seed("gen.review_1071s", "Promoted", "1,071 s review median", "duration", "1,071 s", {"1,071 s": num("1,071") + r"\s?s\b"})
seed("gen.prefix_tokens", "Promoted", "prefix ~17,439 shared + ~7,827 per-directory", "count", "17,439 + 7,827",
     parts={"17,439": num("17,439"), "7,827": num("7,827")})
seed("gen.days_9_4", "Promoted", "9.4 days", "duration", "9.4 days", {"9.4 days": num("9.4") + r"\s?days"})
seed("gen.hours_226", "Promoted", "226 h", "duration", "226 h", {"226 h": num("226") + r"\s?h(?:ours?)?\b"})
seed("gen.timeout_12000s", "Promoted", "12,000 s", "duration", "12,000 s", {"12,000 s": num("12,000") + r"\s?s\b"})
seed("gen.min_10_5", "Promoted", "10.5 min", "duration", "10.5 min", {"10.5 min": num("10.5") + r"\s?min"})
seed("gen.900s", "Promoted", "900 s", "duration", "900 s", {"900 s": num("900") + r"\s?s\b"})
seed("gen.147", "Promoted", "147/147", "fraction", "147/147", {"147/147": frac("147", "147")})
seed("gen.200", "Promoted", "200/200", "fraction", "200/200", {"200/200": frac("200", "200")})
seed("gen.36", "Promoted", "36/36", "fraction", "36/36", {"36/36": frac("36", "36")})
seed("gen.p6_kill_5pct", "Promoted", "P6 kill > 5% unsatisfiable", "threshold", "5%",
     {"5% kill": r"(?:>|" + GEQ + r"|kill|vs)\s?\*?\*?5\s?%|5\s?%\s+unsatisfiable"})

# ---- Cloudflare comparator -------------------------------------------------
seed("cf.runs", "Cloudflare", "131,246 runs / 30 days", "count", "131,246", {"131,246": num("131,246")})
seed("cf.breakglass", "Cloudflare", "288 of 48,095 = 0.6%", "fraction", "288/48,095 = 0.6%",
     parts={"288 of 48,095": frac("288", "48,095"), "48,095": num("48,095"), "0.6%": pct("0.6")})
seed("cf.costs", "Cloudflare", "$1.19 / $0.98 / $1.68 / $1.47", "money", "$0.98 median, $1.68 full",
     parts={"$1.19": money("1.19"), "$0.98": money("0.98"), "$1.68": money("1.68"), "$1.47": money("1.47")})


# --------------------------------------------------------------------------
# GENERIC SWEEP patterns
# --------------------------------------------------------------------------
GNUM = r"\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?"
GPRE = r"(?<![\w.,\-])"
UNITS = (r"tokens?|commits?|runs?|records?|files?|rows?|criteria|criterion|ADRs?|areas?|findings?|"
         r"dispatch(?:es)?|draws?|activities|servers?|tools?|mutants?|slices?|gates?|items?|cases?|"
         r"flips?|decisions?|measures?|stories|hours?|minutes?|seconds?|ms|days?|"
         r"confirmations?|contradictions?|cells?|mutations?|packs?|repos?|ties|cycles?|"
         r"h|s|min|kB|KB|MB|LOC|PRs?|issues?|specs?|lines?|bytes|labels?|positives?|"
         r"ACs?|SMs?|scenarios?|worktrees?|passes|arms?|tiers?|operators?|authors?|clones?")
GENERIC_PATTERNS = [
    ("pct", re.compile(GPRE + r"(" + GNUM + r")\s?%")),
    ("money", re.compile(r"\$\s?(" + GNUM + r")(?![\d])")),
    ("mult", re.compile(GPRE + r"(" + GNUM + r")\s?[x" + TIMES + r"](?![\w])")),
    ("ratio", re.compile(GPRE + r"(" + GNUM + r")\s?/\s?(" + GNUM + r")(?![\w/.])")),
    ("of", re.compile(GPRE + r"(" + GNUM + r")\s+of\s+(?:the\s+)?(" + GNUM + r")(?![\w.])")),
    ("unitword", re.compile(GPRE + r"(" + GNUM + r")\s?(" + UNITS + r")\b")),
    ("bigcomma", re.compile(r"(?<![\w.$,\-])(\d{1,3}(?:,\d{3})+)(?![\d,])")),
    ("semver", re.compile(GPRE + r"(\d+\.\d+\.\d+)(?![\w.])")),
]
SECTION_PRE = re.compile(r"[" + SECT + r"#]\s?$")


def norm_token(s):
    s = re.sub(r"\s+", "", s)
    s = s.replace(TIMES, "x")
    s = re.sub(r"(?<=\d)of(?=\d)", "of", s)
    return s


# --------------------------------------------------------------------------
def run_seeds(lines):
    results = []
    seed_literal_norms = set()
    for sd in SEEDS:
        cand_out = []
        seen = {}
        values_present = []
        roles = [("alternative", k, v) for k, v in sd["cands"].items()] + \
                [("component", k, v) for k, v in sd["parts"].items()]
        for role, vlabel, pat in roles:
            compiled = compile_cand(pat)
            pat = describe_cand(compiled)
            occ = []
            for rel, ln, text in lines:
                for m in (mm for c in compiled for mm in c.finditer(text)):
                    # de-duplicate overlapping spans on the same line (earlier
                    # candidates in the seed's declaration order win)
                    spans = seen.setdefault((rel, ln), [])
                    if any(m.start() < b and a < m.end() for a, b in spans):
                        continue
                    spans.append((m.start(), m.end()))
                    occ.append({"file": rel, "line": ln, "col": m.start() + 1,
                                "match": re.sub(r"\s+", " ", m.group(0))[:120],
                                "context": context(text, m.start(), m.end())})
            files = sorted({o["file"] for o in occ})
            cand_out.append({"value": vlabel, "role": role, "pattern": pat, "n_occ": len(occ),
                             "n_files": len(files), "files": files, "occurrences": occ})
            if occ and role == "alternative":
                values_present.append(vlabel)
            # normalised label forms used to mark generic tokens as seed-covered
            base = re.sub(r"\(.*?\)", "", vlabel).strip()
            for form in {base, base.lstrip(">=~+ ").strip()}:
                seed_literal_norms.add(norm_token(form))
                if " of " in form:
                    seed_literal_norms.add(norm_token(form.replace(" of ", "/")))
            for o in occ[:50]:
                seed_literal_norms.add(norm_token(o["match"]))
        slot_out = None
        if sd["slot"]:
            srx = re.compile(sd["slot"])
            vals = collections.Counter()
            examples = {}
            for rel, ln, text in lines:
                for m in srx.finditer(text):
                    v = m.group("v")
                    vals[v] += 1
                    examples.setdefault(v, []).append(
                        {"file": rel, "line": ln, "context": context(text, m.start(), m.end())})
            known = set(sd["cands"].keys()) | set(sd["parts"].keys())
            other = sorted((v for v in vals if not any(v.lower() == k.lower() or v == k.split()[0] for k in known)),
                           key=lambda v: -vals[v])
            slot_out = {"pattern": sd["slot"],
                        "note": "UNFILTERED: every value found next to the claim keyword anywhere in the corpus; other_values are NOT contradictions, they are leads for the audit",
                        "values": {v: vals[v] for v in sorted(vals, key=lambda k: -vals[k])},
                        "other_values": [{"value": v, "n": vals[v], "examples": examples[v][:3]} for v in other[:15]]}
        n_occ = sum(c["n_occ"] for c in cand_out)
        all_files = sorted({f for c in cand_out for f in c["files"]})
        if n_occ == 0:
            status = "ABSENT"
        elif sd["kind"] == "range":
            status = "RANGE-STATEMENT"
        elif len(values_present) >= 2:
            status = "VARIANT"
        else:
            status = "AGREE"
        results.append({"id": sd["id"], "group": sd["group"], "label": sd["label"],
                        "kind": sd["kind"], "canonical": sd["canonical"], "status": status,
                        "alternatives_present": values_present,
                        "multi_valued_range": (sd["kind"] == "range" and len(values_present) >= 2),
                        "restated": n_occ >= 2,
                        "n_occ": n_occ, "n_files": len(all_files), "files": all_files,
                        "note": sd["note"], "candidates": cand_out, "slot": slot_out})
    return results, seed_literal_norms


def run_generic(lines, seed_literal_norms):
    tok = {}
    for rel, ln, text in lines:
        for kind, rx in GENERIC_PATTERNS:
            for m in rx.finditer(text):
                pre = text[max(0, m.start() - 2):m.start()]
                if SECTION_PRE.search(pre):
                    continue
                t = norm_token(m.group(0))
                e = tok.setdefault(t, {"occ": [], "kinds": set()})
                e["kinds"].add(kind)
                e["occ"].append((rel, ln, m.start(), m.end(), text))
    rows = []
    for t, e in tok.items():
        files = sorted({o[0] for o in e["occ"]})
        if len(e["occ"]) < GENERIC_MIN_OCC or len(files) < GENERIC_MIN_FILES:
            continue
        covered = t in seed_literal_norms
        occs = []
        anchors = []
        for i, (rel, ln, a, b, text) in enumerate(e["occ"]):
            anchors.append("%s:%d" % (rel, ln))
            if i < GENERIC_CTX_CAP:
                occs.append({"file": rel, "line": ln, "col": a + 1, "context": context(text, a, b)})
        rows.append({"token": t, "kinds": sorted(e["kinds"]), "n_occ": len(e["occ"]),
                     "n_files": len(files), "files": files, "covered_by_seed": covered,
                     "status": "AGREE", "anchors": anchors, "occurrences": occs})
    rows.sort(key=lambda r: (-r["n_occ"], r["token"]))
    return rows


def git_head():
    try:
        return subprocess.check_output(["git", "-C", ROOT, "rev-parse", "HEAD"],
                                       text=True).strip()
    except Exception:
        return None


def main():
    lines = load_corpus()
    files = sorted({l[0] for l in lines})
    seeds, seed_norms = run_seeds(lines)
    generic = run_generic(lines, seed_norms)

    # seed candidate literals also matched by the generic sweep (both directions)
    generic_tokens = {g["token"] for g in generic}
    for sd in seeds:
        for c in sd["candidates"]:
            c["in_generic_sweep"] = norm_token(c["value"]) in generic_tokens

    st = collections.Counter(s["status"] for s in seeds)
    counts = {
        "files_scanned": len(files),
        "lines_scanned": len(lines),
        "seed_constants": len(seeds),
        "seed_candidate_values": sum(len(s["candidates"]) for s in seeds),
        "seed_occurrences": sum(s["n_occ"] for s in seeds),
        "seed_AGREE": st.get("AGREE", 0),
        "seed_VARIANT": st.get("VARIANT", 0),
        "seed_RANGE_STATEMENT": st.get("RANGE-STATEMENT", 0),
        "seed_ABSENT": st.get("ABSENT", 0),
        "seed_candidates_with_zero_matches": sum(1 for s in seeds for c in s["candidates"] if c["n_occ"] == 0),
        "seed_not_restated_single_occurrence": sum(1 for s in seeds if s["n_occ"] == 1),
        "seed_single_file": sum(1 for s in seeds if s["n_files"] == 1),
        "seed_slots": sum(1 for s in seeds if s["slot"]),
        "seed_slots_with_other_values": sum(1 for s in seeds if s["slot"] and s["slot"]["other_values"]),
        "generic_tokens_kept": len(generic),
        "generic_covered_by_seed": sum(1 for g in generic if g["covered_by_seed"]),
        "generic_added_beyond_seeds": sum(1 for g in generic if not g["covered_by_seed"]),
        "generic_occurrences": sum(g["n_occ"] for g in generic),
    }
    meta = {
        "generated": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "corpus_root": ROOT.replace("\\", "/"),
        "corpus_git_head": git_head(),
        "scope": "all *.md under the repo, excluding .git/** and any directory named fixtures (spikes/fixtures/**)",
        "script": os.path.abspath(__file__).replace("\\", "/"),
        "context_chars_each_side": CTX,
        "generic_thresholds": {"min_occurrences": GENERIC_MIN_OCC, "min_files": GENERIC_MIN_FILES,
                               "context_kept_per_token": GENERIC_CTX_CAP},
        "status_semantics": {
            "AGREE": "exactly one candidate value of the claim slot occurs in the corpus",
            "VARIANT": "two or more candidate values of the same claim slot occur; may be legitimately different measurements - flagged for the audit, NOT a defect",
            "RANGE-STATEMENT": "the claim is an X-to-Y span or X->Y transition; multi_valued_range says whether more than one span is stated",
            "ABSENT": "no candidate pattern matched anything; reported explicitly, never dropped",
        },
        "generic_status_note": "generic tokens are literal strings, so their occurrences agree by construction (AGREE); cross-token contradictions are the audit phase's job",
        "files": files,
    }
    out = {"meta": meta, "counts": counts, "seeds": seeds, "generic": generic}
    # content digest over everything except the run timestamp, so two runs can
    # be compared for reproducibility
    stable = json.dumps({"meta": {k: v for k, v in meta.items() if k != "generated"},
                         "counts": counts, "seeds": seeds, "generic": generic},
                        ensure_ascii=False, sort_keys=True).encode("utf-8")
    meta["content_sha256_excluding_generated"] = hashlib.sha256(stable).hexdigest()
    os.makedirs(os.path.dirname(OUT_JSON), exist_ok=True)
    with open(OUT_JSON, "w", encoding="utf-8") as fh:
        json.dump(out, fh, ensure_ascii=False, indent=1, sort_keys=False)
    print("wrote", OUT_JSON.replace("\\", "/"))
    for k, v in counts.items():
        print("  %-40s %s" % (k, v))
    print("content sha256 (excluding meta.generated):", meta["content_sha256_excluding_generated"])


if __name__ == "__main__":
    main()
