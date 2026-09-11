#!/usr/bin/env python
"""ID census for D:/repos/redkiln-rs (read-only on the repo).

Scans every .md file under the repo EXCEPT spikes/fixtures/** (vendored copies of a
different repo, excluded to avoid multiply-counting) and .git/**.

For each namespace it records every DEFINITION anchor and every CITATION anchor as
file:line, then derives dangling / orphan / duplicate_definition sets.

All output is ASCII (ensure_ascii=True); the Windows console cannot print some glyphs.
Run:  python census_ids.py   -> writes ../census/ids.json
"""
import json
import os
import re
import sys
from collections import defaultdict, OrderedDict

REPO = "D:/repos/redkiln-rs"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "census", "ids.json")
OUT = os.path.normpath(OUT)

EXCLUDE_PREFIXES = ("spikes/fixtures/", ".git/")

AC_REGISTER = "references/2026-09-08-acceptance-criteria.md"
SM_REGISTER = "references/2026-09-08-success-measures.md"
ARCH_DESIGN = "references/2026-09-08-architecture-design.md"
ARCH_PROPOSAL = "references/2026-09-05-architecture-proposal.md"
PC_PROPOSAL = "references/2026-09-07-process-catalog-proposal.md"
SCHEMA_DOC = "references/2026-09-07-activity-graph-schema.md"
RFC = "references/2026-08-31-rust-inversion-RFC.md"
RFC_AMEND = "references/2026-09-02-rfc-amendment.md"
CHARTER = "references/2026-08-31-rust-inversion-spike-charter.md"
SCORECARD = "references/conformance-2026-09-04-scorecard.md"
LESSONS = "references/2026-09-01-shipmates-lessons-for-redkiln-rs.md"
GROUNDING = "references/research/2026-09-06-grounding-s-results.md"
ADR_README = "references/adr/README.md"
FINDINGS = "spikes/FINDINGS.md"
P9_DECISIONS = "spikes/p9/DECISIONS-2026-09-08.md"
HANDOFF = "spikes/HANDOFF.md"
F33 = "spikes/F33-DRAFT.md"


# ----------------------------------------------------------------------------
# file collection
# ----------------------------------------------------------------------------
def collect_files():
    files = []
    for root, dirs, names in os.walk(REPO):
        rel_root = os.path.relpath(root, REPO).replace("\\", "/")
        if rel_root == ".":
            rel_root = ""
        # prune excluded trees
        dirs[:] = [d for d in dirs if not ((rel_root + "/" + d if rel_root else d) + "/").startswith(EXCLUDE_PREFIXES)]
        for n in names:
            if n.lower().endswith(".md"):
                rel = (rel_root + "/" + n) if rel_root else n
                if rel.startswith(EXCLUDE_PREFIXES):
                    continue
                files.append(rel)
    files.sort()
    return files


def read_lines(rel):
    with open(os.path.join(REPO, rel), "r", encoding="utf-8", errors="replace") as fh:
        return fh.read().split("\n")


FILES = collect_files()
LINES = {f: read_lines(f) for f in FILES}


def anchor(f, i):
    return "%s:%d" % (f, i)


# boundary helpers: an id token must not be glued to alnum / hyphen / slash on the left
# and must not be followed by an alnum.  '-' on the left excludes things like DeepSeek-R1.
LEFT = r"(?<![A-Za-z0-9_/-])"
RIGHT = r"(?![A-Za-z0-9_])"


def scan(pattern, files=None, flags=0):
    """Yield (file, lineno, match) for every regex match over the corpus."""
    rx = re.compile(pattern, flags)
    for f in (files or FILES):
        for i, line in enumerate(LINES[f], 1):
            for m in rx.finditer(line):
                yield f, i, m


def namespace_result(defined, cited, own_register_files_for, rule, dup_tag=None):
    """Build the standard block: defined, cited, dangling, orphan, duplicate_definition.

    dup_tag: when given, only definition anchors carrying this tag string count towards duplicate_definition
    (namespaces whose ids legitimately carry several anchor KINDS - filename + frontmatter, proposal row + legacy_id)."""
    defined = OrderedDict(sorted(defined.items()))
    cited = OrderedDict(sorted(cited.items()))
    dangling = sorted(k for k in cited if k not in defined)
    orphan = []
    for k, defs in defined.items():
        own = set(own_register_files_for(k, defs))
        outside = [c for c in cited.get(k, []) if c.rsplit(":", 1)[0] not in own]
        if not outside:
            orphan.append(k)
    if dup_tag:
        dup = OrderedDict((k, [x for x in v if dup_tag in x]) for k, v in defined.items() if len([x for x in v if dup_tag in x]) > 1)
    else:
        dup = OrderedDict((k, v) for k, v in defined.items() if len(v) > 1)
    return OrderedDict([
        ("rule", rule),
        ("counts", OrderedDict([
            ("defined_distinct", len(defined)),
            ("definition_anchors", sum(len(v) for v in defined.values())),
            ("cited_distinct", len(cited)),
            ("citation_anchors", sum(len(v) for v in cited.values())),
            ("dangling", len(dangling)),
            ("orphan", len(orphan)),
            ("duplicate_definition", len(dup)),
        ])),
        ("defined", defined),
        ("cited", cited),
        ("dangling", dangling),
        ("orphan", orphan),
        ("duplicate_definition", dup),
        ("duplicate_definition_rule", ("ids with more than one anchor tagged %r" % dup_tag) if dup_tag else "ids with more than one definition anchor"),
    ])


def natural_key(s):
    return [int(t) if t.isdigit() else t for t in re.split(r"(\d+)", s)]


anomalies = []


def note(anchor_, text, conf):
    anomalies.append(OrderedDict([("anchor", anchor_), ("note", text), ("confidence", conf)]))


# ----------------------------------------------------------------------------
# AC-###
# ----------------------------------------------------------------------------
def census_ac():
    defined = defaultdict(list)
    for f, i, m in scan(r"^\*\*(AC-\d{3})\*\*", files=[AC_REGISTER]):
        defined[m.group(1)].append(anchor(f, i))
    cited = defaultdict(list)
    for f, i, m in scan(LEFT + r"(AC-\d{3})" + RIGHT):
        cited[m.group(1)].append(anchor(f, i))
    res = namespace_result(defined, cited, lambda k, d: [AC_REGISTER],
        "definition = line-start bold '**AC-NNN**' in " + AC_REGISTER + " ONLY (the two line-start bold AC anchors "
        "elsewhere, architecture-design.md:465 and RUNBOOK.md:1526, are citations and are deliberately not counted as "
        "definitions); citation = every AC-NNN token anywhere (register self-citations included; orphan = no citation "
        "outside the register)")
    # known-fact checks
    ids = sorted(defined)
    nums = sorted(int(k[3:]) for k in ids)
    retired_claim = ["AC-075", "AC-119", "AC-207", "AC-255", "AC-297", "AC-513"]
    missing_in_001_519 = ["AC-%03d" % n for n in range(1, 520) if n not in set(nums)]
    beyond_519 = [k for k in ids if int(k[3:]) > 519]
    line_start_elsewhere = [anchor(f, i) for f, i, m in scan(r"^\*\*(AC-\d{3})\*\*", files=[x for x in FILES if x != AC_REGISTER])]
    res["known_facts"] = OrderedDict([
        ("claim_514_distinct_defined", OrderedDict([("measured", len(ids)), ("agree", len(ids) == 514)])),
        ("claim_range_001_519_plus_520", OrderedDict([
            ("min", "AC-%03d" % nums[0]), ("max", "AC-%03d" % nums[-1]),
            ("beyond_519", beyond_519),
            ("undefined_numbers_within_001_519", missing_in_001_519),
            ("agree", beyond_519 == ["AC-520"] and set(missing_in_001_519) == set(retired_claim)),
        ])),
        ("claim_AC-428_double_anchored", OrderedDict([
            ("anchors", defined.get("AC-428", [])),
            ("all_duplicate_definitions", list(res["duplicate_definition"].keys())),
            ("agree", list(res["duplicate_definition"].keys()) == ["AC-428"]),
        ])),
        ("claim_six_retired_ids_cited_but_undefined", OrderedDict([
            ("retired_claimed", retired_claim),
            ("dangling_measured", res["dangling"]),
            ("retired_each_cited", OrderedDict((k, len(cited.get(k, []))) for k in retired_claim)),
            ("agree", set(res["dangling"]) == set(retired_claim)),
        ])),
        ("line_start_bold_AC_anchors_outside_register", line_start_elsewhere),
    ])
    return res


# ----------------------------------------------------------------------------
# SM-###
# ----------------------------------------------------------------------------
def census_sm():
    defined = defaultdict(list)
    for f, i, m in scan(r"^\*\*(SM-\d{3}) — ", files=[SM_REGISTER]):
        defined[m.group(1)].append(anchor(f, i))
    cited = defaultdict(list)
    for f, i, m in scan(LEFT + r"(SM-\d{3})" + RIGHT):
        cited[m.group(1)].append(anchor(f, i))
    res = namespace_result(defined, cited, lambda k, d: [SM_REGISTER],
        "definition = line-start bold '**SM-NNN ' followed by a space and an em dash (U+2014) then the title, in " + SM_REGISTER +
        " ONLY; citation = every SM-NNN token anywhere")
    ids = sorted(defined)
    nums = set(int(k[3:]) for k in ids)
    # the register's own declaration at line 31
    decl_line = LINES[SM_REGISTER][30] if len(LINES[SM_REGISTER]) > 30 else ""
    declared_unused = re.findall(r"SM-\d{3}", decl_line)
    # parse: the 18 in-range + SM-155, SM-156 unallocated; SM-150, SM-152 cut/retired
    undefined_001_157 = ["SM-%03d" % n for n in range(1, 158) if n not in nums]
    dangling = res["dangling"]
    dangling_by_file = OrderedDict()
    for k in dangling:
        byf = defaultdict(int)
        for c in cited[k]:
            byf[c.rsplit(":", 1)[0]] += 1
        dangling_by_file[k] = OrderedDict(sorted(byf.items()))
    res["known_facts"] = OrderedDict([
        ("claim_135_distinct_defined", OrderedDict([("measured", len(ids)), ("agree", len(ids) == 135)])),
        ("register_line_31_declaration", OrderedDict([
            ("line", SM_REGISTER + ":31"),
            ("sm_ids_named_on_the_line", declared_unused),
            ("count_named", len(declared_unused)),
            ("text_says", "Twenty ids are unused (18 in-range + SM-155, SM-156 unallocated) and SM-150, SM-152 were cut and retired; 157 - 20 - 2 = 135"),
        ])),
        ("undefined_numbers_within_001_157", OrderedDict([
            ("ids", undefined_001_157), ("count", len(undefined_001_157)),
            ("distinct_ids_named_on_line_31", sorted(set(declared_unused))),
            ("named_on_line_31_but_defined", sorted(k for k in set(declared_unused) if k in defined)),
            ("equals_the_undefined_ids_named_on_line_31", set(undefined_001_157) == set(k for k in declared_unused if k not in defined)),
        ])),
        ("undefined_but_cited_measured", OrderedDict([
            ("ids", dangling), ("count", len(dangling)),
            ("exploration_pass_claim", "22 undefined-but-cited, i.e. the 20 unused plus SM-150 and SM-152"),
            ("agree_with_22", len(dangling) == 22),
            ("per_id_citing_files", dangling_by_file),
        ])),
        ("duplicate_definitions", list(res["duplicate_definition"].keys())),
        ("beyond_157", [k for k in ids if int(k[3:]) > 157]),
    ])
    return res


# ----------------------------------------------------------------------------
# ADR-####
# ----------------------------------------------------------------------------
def census_adr():
    defined = defaultdict(list)
    file_ids = []
    for f in FILES:
        m = re.match(r"references/adr/(\d{4})-.*\.md$", f)
        if m:
            file_ids.append(m.group(1))
            defined["ADR-" + m.group(1)].append(anchor(f, 0) + " (filename)")
            # frontmatter id
            for i, line in enumerate(LINES[f][:15], 1):
                mm = re.match(r'^id:\s*"?(\d{4})"?\s*$', line)
                if mm:
                    defined["ADR-" + mm.group(1)].append(anchor(f, i) + " (frontmatter id)")
                    if mm.group(1) != m.group(1):
                        note(anchor(f, i), "ADR frontmatter id %s disagrees with filename number %s" % (mm.group(1), m.group(1)), "certain")
    cited = defaultdict(list)
    forms = defaultdict(int)
    # form 1: ADR-NNNN ; form 2: 'ADR NNNN' ; form 3: adr/NNNN ; form 4: [NNNN](NNNN- ; form 5: quoted "NNNN" in adr frontmatter
    # form 6: backticked `NNNN` with NNNN in 0001..0099
    for f, i, m in scan(r"(?<![A-Za-z0-9])ADR[- ](\d{4})(?![0-9])"):
        cited["ADR-" + m.group(1)].append(anchor(f, i)); forms["ADR-NNNN or 'ADR NNNN'"] += 1
    for f, i, m in scan(r"adr/(\d{4})(?![0-9])"):
        cited["ADR-" + m.group(1)].append(anchor(f, i)); forms["adr/NNNN path"] += 1
    for f, i, m in scan(r"\[(\d{4})\]\(\1-"):
        cited["ADR-" + m.group(1)].append(anchor(f, i)); forms["[NNNN](NNNN-...) link"] += 1
    for f in FILES:
        if not f.startswith("references/adr/") or f == ADR_README:
            continue
        # frontmatter region = between the first two '---' lines
        fm_end = None
        if LINES[f] and LINES[f][0].strip() == "---":
            for i in range(1, min(len(LINES[f]), 40)):
                if LINES[f][i].strip() == "---":
                    fm_end = i
                    break
        if fm_end:
            for i in range(1, fm_end):
                line = LINES[f][i]
                if re.match(r"^id:", line):
                    continue
                for m in re.finditer(r'"(\d{4})"', line):
                    cited["ADR-" + m.group(1)].append(anchor(f, i + 1)); forms["bare quoted NNNN in adr frontmatter"] += 1
    for f, i, m in scan(r"`(00\d{2})`"):
        n = int(m.group(1))
        if 1 <= n <= 99:
            cited["ADR-" + m.group(1)].append(anchor(f, i)); forms["backticked `00NN`"] += 1
    res = namespace_result(defined, cited, lambda k, d: [x.split(":")[0] for x in d],
        "definition = the ADR file references/adr/NNNN-*.md (filename) plus its frontmatter 'id:' line; citation forms: "
        "ADR-NNNN, 'ADR NNNN', adr/NNNN path, [NNNN](NNNN-...) markdown link, bare quoted \"NNNN\" inside adr frontmatter "
        "lists (supersedes/superseded_by/related), and backticked `00NN` (00NN restricted to 0001..0099 to exclude years). "
        "Orphan = no citation outside the ADR's own file (the README index counts as a citation)", dup_tag="(filename)")
    nums = sorted(int(x) for x in file_ids)
    gaps = [n for n in range(1, (nums[-1] if nums else 0) + 1) if n not in set(nums)]
    dupes = [n for n in set(nums) if nums.count(n) > 1]
    # both-sides supersession check is out of scope; record forward references
    fwd = [k for k in res["dangling"]]
    res["citation_forms"] = OrderedDict(sorted(forms.items()))
    res["known_facts"] = OrderedDict([
        ("claim_exactly_89_files_0001_0089_no_gaps_no_duplicates", OrderedDict([
            ("files", len(file_ids)), ("min", "%04d" % nums[0] if nums else None), ("max", "%04d" % nums[-1] if nums else None),
            ("gaps", gaps), ("duplicate_numbers", dupes),
            ("agree", len(file_ids) == 89 and nums[0] == 1 and nums[-1] == 89 and not gaps and not dupes),
        ])),
        ("dangling_forward_references", fwd),
        ("every_definition_has_two_anchors_filename_and_frontmatter", all(len(v) == 2 for v in defined.values())),
    ])
    return res


# ----------------------------------------------------------------------------
# F## findings
# ----------------------------------------------------------------------------
def census_f():
    defined = defaultdict(list)
    def_files = [f for f in FILES if f == FINDINGS or re.match(r"spikes/F\d+-DRAFT\.md$", f)]
    for f, i, m in scan(r"^## (F\d{1,2})\b", files=def_files):
        defined[m.group(1)].append(anchor(f, i))
    cited = defaultdict(list)
    for f, i, m in scan(LEFT + r"(F[1-9]\d?)" + RIGHT):
        cited[m.group(1)].append(anchor(f, i))
    res = namespace_result(defined, cited, lambda k, d: [x.split(":")[0] for x in d],
        "definition = '^## F<n> ' heading in spikes/FINDINGS.md or spikes/F<n>-DRAFT.md; citation = F<n> token (1..99) "
        "not glued to alnum/hyphen/slash on the left (so 'F32-DRAFT.md' counts as a citation of F32; 'E3-H'-style "
        "tokens do not collide). Orphan = no citation outside the file that defines it")
    in_findings = sorted((int(k[1:]) for k, v in defined.items() if any(x.startswith(FINDINGS) for x in v)))
    in_drafts = OrderedDict()
    for k, v in sorted(defined.items(), key=lambda kv: int(kv[0][1:])):
        d = [x for x in v if "-DRAFT.md" in x]
        if d:
            in_drafts[k] = d
    draft_file_vs_heading = []
    for f in def_files:
        m = re.match(r"spikes/F(\d+)-DRAFT\.md$", f)
        if m:
            heads = [k for k, v in defined.items() if any(x.startswith(f + ":") for x in v)]
            if heads != ["F" + m.group(1)]:
                draft_file_vs_heading.append(OrderedDict([("file", f), ("headings", heads)]))
    res["known_facts"] = OrderedDict([
        ("claim_FINDINGS_holds_F1_F28_no_F9", OrderedDict([
            ("in_FINDINGS", ["F%d" % n for n in in_findings]),
            ("agree", in_findings == [n for n in range(1, 29) if n != 9]),
        ])),
        ("claim_F29_F31_to_F40_drafts_only", OrderedDict([
            ("draft_defined", list(in_drafts.keys())),
            ("also_in_FINDINGS", [k for k in in_drafts if any(x.startswith(FINDINGS) for x in defined[k])]),
            ("agree", list(in_drafts.keys()) == ["F29"] + ["F%d" % n for n in range(31, 41)] and not [k for k in in_drafts if any(x.startswith(FINDINGS) for x in defined[k])]),
        ])),
        ("claim_F30_never_written", OrderedDict([
            ("F30_defined", "F30" in defined), ("F30_citations", len(cited.get("F30", []))),
            ("F9_defined", "F9" in defined), ("F9_citations", len(cited.get("F9", []))),
            ("agree", "F30" not in defined),
        ])),
        ("draft_file_name_vs_heading_mismatch", draft_file_vs_heading),
    ])
    return res


# ----------------------------------------------------------------------------
# Named spikes (register rows + METHOD headings)
# ----------------------------------------------------------------------------
SPIKE_TOKEN = (LEFT + r"("
    r"S-[A-Z][A-Z0-9]*(?:-[A-Z0-9]+)*"     # S-BIN, S-H1b handled below, S-SUBTLE-CB, S-CANCEL-WIN
    r"|S-H1[bc](?:-24H)?"                   # S-H1b, S-H1c, S-H1b-24H (lower-case suffix)
    r"|S0\.[1-3](?:-SCOPE)?"                # S0.1, S0.2, S0.3, S0.1-SCOPE
    r"|E[0-7](?:-[A-Z]+)?"                  # E0..E7, E1-TTL, E3-H, E3-SEED, E3-SEV, E0-VAR, E-REST is separate
    r"|E-REST|D-RECON|D-FIXCONSUMER"
    r"|H[1-4]"
    r"|P[1-9]b?(?:-(?:D|B|CEIL|BOUND|PACKS))?"   # P1, P9b, P1-D, P2-B, P8-B, P7-CEIL, P1-BOUND, P6-PACKS (P9-DRAFT is a file ref, excluded)
    r"|R-[A-Z][A-Z0-9]+"                    # reading tasks R-ASYNC .. R-P7RATIO (upper-case only)
    r")" + RIGHT)


def census_spikes():
    # candidate ordering matters: S-H1b must win over S-H1 (regex alternation is ordered; the generic S- branch
    # requires upper-case segments so 'S-H1b' falls to the second alternative).
    tokens = defaultdict(list)
    for f, i, m in scan(SPIKE_TOKEN):
        tokens[m.group(1)].append(anchor(f, i))
    defined = defaultdict(list)
    def_kinds = defaultdict(list)
    # D1: headings in spikes/METHOD*.md and the charter
    method_files = [f for f in FILES if re.match(r"spikes/METHOD[^/]*\.md$", f)] + [CHARTER]
    # subject-first rule: strip '#+ ', an optional section number ('1.', '4a.', '1.2', '9b.'), and an optional
    # 'Pre-registration [#n] <dash> ' prefix; the id must then be the FIRST token of what remains.
    subject_rx = re.compile(r"^#{1,6}\s+(?:\d+[a-z]?(?:\.\d+)*\.?\s+)?(?:Pre-registration(?:\s+#\d+)?\s+[-—–:]\s+)?"
                            + SPIKE_TOKEN[len(LEFT):])
    for f in method_files:
        in_fence = False
        for i, line in enumerate(LINES[f], 1):
            if line.startswith("```"):
                in_fence = not in_fence
                continue
            if in_fence or not re.match(r"^#{1,6} ", line):
                continue
            m = subject_rx.match(line)
            if m:
                defined[m.group(1)].append(anchor(f, i))
                def_kinds[m.group(1)].append("heading")
    # D2: spike-register rows in architecture-design.md between '### 10.5' and '### 10.6'
    start = end = None
    for i, line in enumerate(LINES[ARCH_DESIGN], 1):
        if line.startswith("### 10.5"):
            start = i
        elif line.startswith("### 10.6") and start:
            end = i
            break
    assert start and end, "spike register section 10.5 not found in architecture-design.md"
    for i in range(start, end):
        line = LINES[ARCH_DESIGN][i - 1]
        m = re.match(r"^\| \*\*(" + SPIKE_TOKEN[len(LEFT) + 1:-len(RIGHT) - 1] + r")\b", line)
        if m:
            defined[m.group(1)].append(anchor(ARCH_DESIGN, i))
            def_kinds[m.group(1)].append("register_row_10.5")
    # D3: E3 condition rows and E3-H bold definition
    for f, i, m in scan(r"^\| \*\*(S-[A-Z]+)\*\* \|", files=["spikes/METHOD-E3.md"]):
        defined[m.group(1)].append(anchor(f, i)); def_kinds[m.group(1)].append("METHOD-E3_condition_row")
    for f, i, m in scan(r"^\*\*(S-[A-Z]+)\.\*\*", files=["spikes/METHOD-E3H.md"]):
        defined[m.group(1)].append(anchor(f, i)); def_kinds[m.group(1)].append("METHOD-E3H_bold_definition")
    cited = tokens
    res = namespace_result(defined, cited, lambda k, d: sorted(set(x.split(":")[0] for x in d)),
        "token = S-NAME (upper-case segments; S-H1b/S-H1c/S-H1b-24H special-cased), S0.1..S0.3(-SCOPE), E0..E7(-SUFFIX), "
        "E-REST, D-RECON, D-FIXCONSUMER, H1..H4, P1..P9(b)(-SUFFIX), R-NAME (upper-case). Definition anchors: (D1) a "
        "markdown heading in spikes/METHOD*.md or the spike charter whose first 40 chars contain the id; (D2) a table row "
        "whose first cell opens with **ID** inside architecture-design.md section 10.5 'The spike register'; (D3) a "
        "condition row '| **S-X** |' in METHOD-E3.md or a bold '**S-X.**' definition in METHOD-E3H.md. Citation = every "
        "token occurrence. Orphan = no citation outside the file(s) defining it")
    res["definition_kinds"] = OrderedDict((k, def_kinds[k]) for k in sorted(defined, key=natural_key))
    res["register_10.5_span"] = anchor(ARCH_DESIGN, start) + " .. " + anchor(ARCH_DESIGN, end)
    task_list = ("S-BIN S-FACET S-GIX S-TTL1 S-H1b S-H1c S-WT S-MIG S-VAR S-TYPED S-SUBTLE S-SUBTLE-CB S-CODEX S-RETRY "
                 "S-SILENT S-FAIL S-CASCADE S-FANOUT S-FEAT S-CANCEL-WIN S-P7B S-WT-REC E0 E1 E2 E3 E4 E5 E6 E7 H1 H2 H3 H4 "
                 "P1 P2 P3 P4 P5 P6 P7 P8 P9 P1-D P2-B P8-B P7-CEIL P9b R-ASYNC R-SQLITE R-MUT R-LINEAR R-DOCS R-P7RATIO "
                 "P1-BOUND D-RECON E-REST D-FIXCONSUMER E0-VAR E3-SEED E3-SEV S-H1b-24H").split()
    res["task_listed_ids"] = OrderedDict((k, OrderedDict([
        ("defined", len(defined.get(k, []))), ("cited", len(cited.get(k, []))),
        ("status", "defined" if k in defined else ("cited_only" if k in cited else "ABSENT: 0 matches"))])) for k in task_list)
    res["tokens_not_in_task_list"] = sorted((k for k in set(defined) | set(cited) if k not in task_list), key=natural_key)
    return res


# ----------------------------------------------------------------------------
# The R namespaces (five kinds, plus the selection symbol)
# ----------------------------------------------------------------------------
LINT_FILES = {SCHEMA_DOC, "spikes/F32-DRAFT.md", "spikes/F33-DRAFT.md", "spikes/F34-DRAFT.md", "spikes/F35-DRAFT.md",
              "references/adr/0064-tool-steps-typed-by-verb-composing-emit.md",
              "references/adr/0065-lint-r4b-on-false-target-rigor-reachability.md",
              "references/adr/0080-quality-block-policy-predicate-is-not-a-terminal-condition.md",
              "spikes/p9/RYAN-TODO-2026-09-08.md"}
RFC_FILES = {RFC, RFC_AMEND, ARCH_PROPOSAL, ARCH_DESIGN, AC_REGISTER, "references/2026-09-08-evidence-augmentation.md",
             "references/2026-09-08-evidence-brief.md", "references/adr/0002-clean-break-self-hosting-switch.md",
             "references/research/2026-09-03-redkiln-lessons-brief.md", FINDINGS, "spikes/METHOD.md", "spikes/METHOD-H.md"}


def classify_numeric_r(f, line, m):
    tok = m.group(1)
    pre = line[max(0, m.start() - 12):m.start()]
    if tok.endswith("b"):
        return "lint_rule"
    if f == RFC and re.match(r"^\| R\d+ \|", line):
        return "rfc_risk"
    if f == P9_DECISIONS and re.match(r"^\| R\d ", line):
        return "p6_remediation_row"
    if re.search(r"RFC\s*$|risk\s*$|RFC's\s*$", pre):
        return "rfc_risk"
    if re.search(r"[Ll]int\s*$|\(rule\s*$", pre):
        return "lint_rule"
    if f == GROUNDING and re.match(r"^## Q\d \(R\d\)", line):
        return "grounding_partD_question"
    if f == HANDOFF and "DECISIONS" in line:
        return "p6_remediation_row"
    if f == P9_DECISIONS:
        return "p6_remediation_row"
    if f in LINT_FILES:
        return "lint_rule"
    if f in RFC_FILES or f == HANDOFF:
        return "rfc_risk"
    return "UNCLASSIFIED"


def census_r():
    out = OrderedDict()
    out["disambiguation_rule"] = (
        "Prefix R is split into: (1) reading tasks 'R-NAME' with an all-upper-case name (R-ASYNC, R-SQLITE, R-MUT, "
        "R-LINEAR, R-DOCS, R-P7RATIO) - defined as rows of the section-10.5 spike register; (2) lint rule ids: the "
        "kebab-case 'R-recent-deploy-feed-absent', 'R4b', and numeric R<n> when preceded by 'Lint'/'lint' or found in the "
        "lint-context file set (activity-graph-schema.md, F32..F35 drafts, ADR-0064/0065/0080, p9/RYAN-TODO) - the "
        "schema's section 9 numbers its rules 1..9 without the R prefix, so R1..R9 lint definitions are MAPPED [inferred] "
        "to those numbered items and R4b to schema:300 and ADR-0065; (3) RFC risk ids R1..R14 - defined as rows "
        "'| R<n> |' of the RFC risk table, and cited when preceded by 'RFC'/'risk' or found in the RFC-context file set; "
        "(4) P6 remediation rows R1..R7 in spikes/p9/DECISIONS-2026-09-08.md (table rows '| R<n> ') cited from HANDOFF "
        "where the line names DECISIONS; (5) grounding-s-results Part D headings 'Q<n> (R<m>)' - four anchors, no "
        "definition in the repo; (6) the bare selection symbol 'R(stage, ctx)' matched as R( ... ). Tokens glued to a "
        "hyphen or alnum on the left (DeepSeek-R1) are excluded. Anything the rules cannot place is listed as "
        "UNCLASSIFIED rather than silently binned.")
    # (1) reading tasks: reuse the spikes census token
    rt_def, rt_cit = defaultdict(list), defaultdict(list)
    for f, i, m in scan(LEFT + r"(R-[A-Z][A-Z0-9]+)" + RIGHT):
        rt_cit[m.group(1)].append(anchor(f, i))
        if f == ARCH_DESIGN and re.match(r"^\| \*\*R-[A-Z0-9]+\*\*", LINES[f][i - 1]):
            rt_def[m.group(1)].append(anchor(f, i))
    out["reading_tasks"] = namespace_result(rt_def, rt_cit, lambda k, d: [ARCH_DESIGN],
        "R-NAME upper-case; definition = '| **R-NAME**' row in architecture-design.md (section 10.5 register)")
    # (2) lint rules
    lint_def, lint_cit = defaultdict(list), defaultdict(list)
    # schema section 9 numbered items -> R1..R9
    in9 = False
    for i, line in enumerate(LINES[SCHEMA_DOC], 1):
        if line.startswith("## 9. Lint"):
            in9 = True
            continue
        if in9 and line.startswith("## "):
            break
        if in9:
            mm = re.match(r"^(\d)\. ", line)
            if mm:
                lint_def["R" + mm.group(1)].append(anchor(SCHEMA_DOC, i) + " [inferred: numbered lint item, no R prefix in source]")
    for f, i, m in scan(r"\*\*Lint (R4b)\*\*", files=[SCHEMA_DOC]):
        lint_def[m.group(1)].append(anchor(f, i))
    for f, i, m in scan(r"^title: Lint (R4b)", files=["references/adr/0065-lint-r4b-on-false-target-rigor-reachability.md"]):
        lint_def[m.group(1)].append(anchor(f, i))
    for f, i, m in scan(r"(R-recent-deploy-feed-absent)"):
        lint_cit[m.group(1)].append(anchor(f, i))
    # (3),(4),(5) numeric
    rfc_def, rfc_cit = defaultdict(list), defaultdict(list)
    p6_def, p6_cit = defaultdict(list), defaultdict(list)
    ground_cit = defaultdict(list)
    unclassified = []
    for f, i, m in scan(LEFT + r"(R\d{1,2}b?)" + RIGHT):
        line = LINES[f][i - 1]
        cls = classify_numeric_r(f, line, m)
        tok = m.group(1)
        a = anchor(f, i)
        if cls == "lint_rule":
            lint_cit[tok].append(a)
        elif cls == "rfc_risk":
            rfc_cit[tok].append(a)
            if f == RFC and re.match(r"^\| R\d+ \|", line):
                rfc_def[tok].append(a)
        elif cls == "p6_remediation_row":
            p6_cit[tok].append(a)
            if f == P9_DECISIONS and re.match(r"^\| R\d ", line):
                p6_def[tok].append(a)
        elif cls == "grounding_partD_question":
            ground_cit[tok].append(a)
        else:
            unclassified.append(OrderedDict([("anchor", a), ("token", tok), ("context", line[max(0, m.start() - 40):m.end() + 30])]))
    out["lint_rules"] = namespace_result(lint_def, lint_cit, lambda k, d: sorted(set(x.split(":")[0] for x in d)),
        "see disambiguation_rule (2)")
    out["rfc_risks"] = namespace_result(rfc_def, rfc_cit, lambda k, d: [RFC],
        "see disambiguation_rule (3); orphan = no citation outside the RFC itself")
    out["p6_remediation_rows"] = namespace_result(p6_def, p6_cit, lambda k, d: [P9_DECISIONS],
        "see disambiguation_rule (4)")
    out["grounding_partD_questions"] = namespace_result({}, ground_cit, lambda k, d: [],
        "see disambiguation_rule (5): 'Q<n> (R<m>)' headings in research/2026-09-06-grounding-s-results.md; no definition of "
        "this R series exists in the repo (ABSENCE)")
    sel = defaultdict(list)
    for f, i, m in scan(r"(?<![A-Za-z0-9])(R\((?:stage|ctx|item)[^)]{0,40}\))"):
        sel[m.group(1)].append(anchor(f, i))
    out["selection_symbol"] = OrderedDict([
        ("rule", "bare symbol R( ... ) opening with stage/ctx/item; cited only, defined as prose in the selection-engine sections"),
        ("counts", OrderedDict([("forms", len(sel)), ("anchors", sum(len(v) for v in sel.values()))])),
        ("cited", OrderedDict(sorted(sel.items()))),
    ])
    out["UNCLASSIFIED_numeric_R"] = unclassified
    return out


# ----------------------------------------------------------------------------
# Legacy namespaces
# ----------------------------------------------------------------------------
def adr_legacy_map():
    """legacy_id -> ADR anchor, from every ADR file's frontmatter."""
    mp = defaultdict(list)
    for f in FILES:
        if not re.match(r"references/adr/\d{4}-", f):
            continue
        for i, line in enumerate(LINES[f][:20], 1):
            m = re.match(r"^legacy_id:\s*(.+?)\s*$", line)
            if m:
                mp[m.group(1)].append(anchor(f, i))
    return mp


def census_legacy():
    out = OrderedDict()
    legacy = adr_legacy_map()
    out["adr_legacy_id_values"] = OrderedDict(sorted(legacy.items(), key=lambda kv: natural_key(kv[0])))
    out["adr_legacy_id_count"] = sum(len(v) for v in legacy.values())
    dup_legacy = OrderedDict((k, v) for k, v in legacy.items() if len(v) > 1 and k != "none")
    out["adr_legacy_id_none_count"] = len(legacy.get("none", []))
    out["adr_legacy_id_duplicates"] = dup_legacy

    def readme_rows(prefix_rx):
        d = defaultdict(list)
        for f, i, m in scan(r"^\| (" + prefix_rx + r") \| \[(\d{4})\]", files=[ADR_README]):
            d[m.group(1)].append(anchor(f, i) + " -> ADR-" + m.group(2))
        return d

    # --- D## proposal decisions
    d_def = defaultdict(list)
    for f, i, m in scan(r"^\| \*{0,2}(D\d{1,2})(?:/(D\d{1,2}))?", files=[ARCH_PROPOSAL]):
        d_def[m.group(1)].append(anchor(f, i) + " (proposal decision row)")
        if m.group(2):
            d_def[m.group(2)].append(anchor(f, i) + " (proposal decision row, combined cell)")
    for k, v in legacy.items():
        for tok in re.findall(r"\bD\d{1,2}\b", k):
            for a in v:
                d_def[tok].append(a + " (adr legacy_id: " + k + ")")
    for k, v in readme_rows(r"D\d{1,2}(?:/D\d{1,2})?(?: rev\.\d)?").items():
        for tok in re.findall(r"D\d{1,2}", k):
            d_def[tok].extend([x + " (adr README row)" for x in v])
    d_cit = defaultdict(list)
    for f, i, m in scan(LEFT + r"(D\d{1,2})" + RIGHT):
        d_cit[m.group(1)].append(anchor(f, i))
    out["D_proposal_decisions"] = namespace_result(d_def, d_cit, lambda k, d: sorted(set(x.split(":")[0] for x in d)),
        "definition = a table row whose first cell opens with D<n> (optionally bold / 'D5/D6' combined / '(rev. 4)') in the "
        "architecture proposal, plus every ADR 'legacy_id:' naming it and the ADR README index row; citation = D<n> token "
        "not glued left to alnum/hyphen (so D-RECON does not collide). D<n> with n>22 is outside the declared D1..D22 set", dup_tag="(proposal decision row")
    # --- PC#
    pc_def = defaultdict(list)
    for f, i, m in scan(r"^\| \*{0,2}(PC\d)\b", files=[PC_PROPOSAL]):
        pc_def[m.group(1)].append(anchor(f, i) + " (catalog proposal row)")
    for k, v in legacy.items():
        if re.fullmatch(r"PC\d", k):
            pc_def[k].extend([a + " (adr legacy_id)" for a in v])
    for k, v in readme_rows(r"PC\d").items():
        pc_def[k].extend([x + " (adr README row)" for x in v])
    pc_cit = defaultdict(list)
    for f, i, m in scan(LEFT + r"(PC\d)" + RIGHT):
        pc_cit[m.group(1)].append(anchor(f, i))
    out["PC_process_catalog_decisions"] = namespace_result(pc_def, pc_cit, lambda k, d: sorted(set(x.split(":")[0] for x in d)),
        "definition = '| PC<n>' row in the process-catalog proposal, ADR legacy_id, ADR README row; citation = PC<n> token", dup_tag="(catalog proposal row)")
    # --- SCHEMA-##
    sc_def = defaultdict(list)
    for k, v in legacy.items():
        if re.fullmatch(r"SCHEMA-\d{1,2}", k):
            sc_def[k].extend([a + " (adr legacy_id)" for a in v])
    for k, v in readme_rows(r"SCHEMA-\d{1,2}").items():
        sc_def[k].extend([x + " (adr README row)" for x in v])
    sc_cit = defaultdict(list)
    for f, i, m in scan(LEFT + r"(SCHEMA-\d{1,2})" + RIGHT):
        sc_cit[m.group(1)].append(anchor(f, i))
    out["SCHEMA_amendments"] = namespace_result(sc_def, sc_cit, lambda k, d: sorted(set(x.split(":")[0] for x in d)),
        "definition = ADR legacy_id and ADR README row ONLY (the activity-graph-schema doc numbers its amendments "
        "9a.N/9b.N/9c.N and never uses the SCHEMA-N form: ABSENCE of a source-side anchor); citation = SCHEMA-<n> token", dup_tag="(adr legacy_id)")
    # --- conformance ids GATE/DEPTH/SHAPE/SURF/PACK/MEAS
    cf_def = defaultdict(list)
    for f, i, m in scan(r"^### ((?:GATE|DEPTH|SHAPE|SURF|PACK|MEAS)-\d{2})\b", files=[SCORECARD]):
        cf_def[m.group(1)].append(anchor(f, i))
    cf_cit = defaultdict(list)
    for f, i, m in scan(LEFT + r"((?:GATE|DEPTH|SHAPE|SURF|PACK|MEAS)-\d{2})" + RIGHT):
        cf_cit[m.group(1)].append(anchor(f, i))
    out["conformance_rubric_ids"] = namespace_result(cf_def, cf_cit, lambda k, d: [SCORECARD],
        "definition = '### <ID>' heading in conformance-2026-09-04-scorecard.md (only the 'rows that move' are headed; the "
        "19 carried-forward ids are listed at scorecard:200 as a bare list and have NO definition in this repo - the rubric "
        "lives elsewhere: ABSENCE); citation = every token")
    # --- L#.# shipmates lessons
    l_def = defaultdict(list)
    for f, i, m in scan(r"^\*\*(L\d\.\d{1,2})\b", files=[LESSONS]):
        l_def[m.group(1)].append(anchor(f, i))
    l_cit = defaultdict(list)
    for f, i, m in scan(LEFT + r"(L\d\.\d{1,2})" + RIGHT):
        l_cit[m.group(1)].append(anchor(f, i))
    out["L_shipmates_lessons"] = namespace_result(l_def, l_cit, lambda k, d: [LESSONS],
        "definition = line-start bold '**L<n>.<m>' in the shipmates-lessons doc; citation = L<n>.<m> token")
    # --- A01..A06 advisory cases (P2) and the colliding P9b ambiguity ids A1..A27
    adv_cit = defaultdict(list)
    range_cit = []
    for f, i, m in scan(LEFT + r"(A0[1-9])" + RIGHT):
        adv_cit[m.group(1)].append(anchor(f, i))
        line = LINES[f][i - 1]
        if re.search(r"A0[1-9][^A-Za-z0-9]{1,3}A(0[1-9]|10)", line[max(0, m.start() - 4):m.end() + 6]):
            range_cit.append(anchor(f, i))
    a10_range = [anchor(f, i) for f, i, m in scan(r"A01[^A-Za-z0-9]{1,3}A10")]
    out["A_advisory_cases_P2"] = namespace_result({}, adv_cit, lambda k, d: [],
        "token = A01..A10 (zero-padded); NO individual definition anchor exists anywhere in the repo - the ids occur only "
        "as the range expressions 'A01-A06' (and once 'A01-A10' in results/p2-labels-BLANK.md): ABSENCE")
    out["A_advisory_cases_P2"]["range_expression_anchors"] = sorted(set(range_cit))
    out["A_advisory_cases_P2"]["range_A01_to_A10_anchors"] = a10_range
    out["A_advisory_cases_P2"]["note"] = ("A10 is NOT censused as a token here because bare 'A10' collides with the P9b ambiguity id A10 in "
                                          "F33-DRAFT.md; only the explicit range 'A01-A10' is recorded")
    p9b_def, p9b_cit = defaultdict(list), defaultdict(list)
    for f, i, m in scan(r"^\| (A\d{1,2}) \|", files=[F33]):
        p9b_def[m.group(1)].append(anchor(f, i))
    for f, i, m in scan(LEFT + r"(A[1-9]\d?)" + RIGHT):
        p9b_cit[m.group(1)].append(anchor(f, i))
    out["A_p9b_ambiguities"] = namespace_result(p9b_def, p9b_cit, lambda k, d: [F33],
        "SEPARATE namespace that shares the letter: P9b ambiguity ids A1..A27 (not zero-padded) defined as '| A<n> |' rows "
        "in spikes/F33-DRAFT.md; citation = A<n> token without zero padding. Recorded so the audit does not confuse it "
        "with the P2 advisory cases A01..A06")
    return out


# ----------------------------------------------------------------------------
# anomalies tripped over while building the index (not chased)
# ----------------------------------------------------------------------------
def derive_anomalies(ns):
    sp = ns["SPIKES"]
    for k in sp["dangling"]:
        c = sp["cited"][k]
        note(c[0], "spike id %s is cited %d times but has no definition anchor under the register/METHOD rules (first citation shown)" % (k, len(c)), "probable" if k in ("P9b", "E7") else "possible")
    adr = ns["ADR"]
    for k in adr["dangling"]:
        note(adr["cited"][k][0], "%s is cited %d times but no such ADR file exists (RUNBOOK names it as a future record)" % (k, len(adr["cited"][k])), "certain")
    sm = ns["SM"]
    if "SM-157" in sm["defined"]:
        note(sm["defined"]["SM-157"][0], "SM-157 sits beyond the last area range (SM-154); the register records this deliberately at line 31/33", "certain")
    adv = ns["LEGACY"]["A_advisory_cases_P2"]
    for a in adv.get("range_A01_to_A10_anchors", []):
        note(a, "this file says advisory cases A01-A10 while every other file (and results/p2-labels-ryan.md:32) says A01-A06", "possible")
    lint = ns["R"]["lint_rules"]
    if "R10" in lint["cited"]:
        note(lint["cited"]["R10"][0], "F34 refers to lint rules R1-R10 but the schema section 9 numbers nine rules (plus R4b); R10 has no definition anchor", "possible")
    if "R-recent-deploy-feed-absent" in lint["dangling"]:
        note(lint["cited"]["R-recent-deploy-feed-absent"][0], "R-recent-deploy-feed-absent is used as a facet-abstention rule id (rule `R-...` emitted with null), not a lint rule; it has no line-anchored definition anywhere - only uses", "probable")
    d = ns["LEGACY"]["D_proposal_decisions"]
    for k, v in d["duplicate_definition"].items():
        note(v[1], "%s has two proposal decision rows (rev.3 row and the rev.4 rebuilt row); both are counted as definition anchors" % k, "certain")
    f = ns["F"]
    for k in ("F9", "F30"):
        if k in f["cited"]:
            note(f["cited"][k][0], "%s is cited %d times and never defined; the corpus documents this (F9 struck, F30 reserved and never written)" % (k, len(f["cited"][k])), "certain")
    conf = ns["LEGACY"]["conformance_rubric_ids"]
    if conf["dangling"]:
        note(conf["cited"][conf["dangling"][0]][0], "%d conformance rubric ids (%s ...) are cited but defined nowhere in this repo; the rubric is external" % (len(conf["dangling"]), ", ".join(conf["dangling"][:4])), "certain")
    gq = ns["R"]["grounding_partD_questions"]
    if gq["dangling"]:
        note(gq["cited"][gq["dangling"][0]][0], "grounding-s-results Part D headings carry an R-series (Q1 (R2) ...) that is defined nowhere in the repo and is not the RFC risk series", "probable")


# ----------------------------------------------------------------------------
# main
# ----------------------------------------------------------------------------
def main():
    result = OrderedDict()
    result["meta"] = OrderedDict([
        ("repo", REPO),
        ("generated_by", os.path.basename(__file__)),
        ("scanned_md_files", len(FILES)),
        ("excluded", "spikes/fixtures/** (vendored copies of a different repo; would multiply-count) and .git/**"),
        ("evidence_tags", "[measured] every count here is computed by this script; rule strings marked [inferred] are interpretive"),
        ("file_list", FILES),
    ])
    ns = OrderedDict()
    ns["AC"] = census_ac()
    ns["SM"] = census_sm()
    ns["ADR"] = census_adr()
    ns["F"] = census_f()
    ns["SPIKES"] = census_spikes()
    ns["R"] = census_r()
    ns["LEGACY"] = census_legacy()
    result["namespaces"] = ns
    derive_anomalies(ns)

    # anchor assertions: every namespace must have matched something
    assertions = OrderedDict()
    assertions["AC_defined_nonzero"] = ns["AC"]["counts"]["defined_distinct"] > 0
    assertions["SM_defined_nonzero"] = ns["SM"]["counts"]["defined_distinct"] > 0
    assertions["ADR_defined_nonzero"] = ns["ADR"]["counts"]["defined_distinct"] > 0
    assertions["F_defined_nonzero"] = ns["F"]["counts"]["defined_distinct"] > 0
    assertions["SPIKES_defined_nonzero"] = ns["SPIKES"]["counts"]["defined_distinct"] > 0
    assertions["R_reading_defined_nonzero"] = ns["R"]["reading_tasks"]["counts"]["defined_distinct"] > 0
    assertions["R_rfc_defined_nonzero"] = ns["R"]["rfc_risks"]["counts"]["defined_distinct"] > 0
    assertions["R_lint_cited_nonzero"] = ns["R"]["lint_rules"]["counts"]["cited_distinct"] > 0
    assertions["R_p6_defined_nonzero"] = ns["R"]["p6_remediation_rows"]["counts"]["defined_distinct"] > 0
    assertions["R_selection_symbol_nonzero"] = ns["R"]["selection_symbol"]["counts"]["anchors"] > 0
    for k in ["D_proposal_decisions", "PC_process_catalog_decisions", "SCHEMA_amendments", "conformance_rubric_ids",
              "L_shipmates_lessons", "A_p9b_ambiguities"]:
        assertions["LEGACY_" + k + "_defined_nonzero"] = ns["LEGACY"][k]["counts"]["defined_distinct"] > 0
    assertions["LEGACY_A_advisory_cited_nonzero"] = ns["LEGACY"]["A_advisory_cases_P2"]["counts"]["cited_distinct"] > 0
    result["anchor_assertions"] = assertions
    failed = [k for k, v in assertions.items() if not v]

    result["anomalies"] = anomalies
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(result, fh, indent=1, ensure_ascii=True)
        fh.write("\n")

    # ASCII-only console summary
    print("wrote", OUT)
    print("files scanned:", len(FILES))
    for name in ["AC", "SM", "ADR", "F", "SPIKES"]:
        print(name, json.dumps(ns[name]["counts"]))
    for sub in ["reading_tasks", "lint_rules", "rfc_risks", "p6_remediation_rows", "grounding_partD_questions"]:
        print("R/" + sub, json.dumps(ns["R"][sub]["counts"]))
    print("R/selection_symbol", json.dumps(ns["R"]["selection_symbol"]["counts"]))
    print("R/UNCLASSIFIED", len(ns["R"]["UNCLASSIFIED_numeric_R"]))
    for sub, v in ns["LEGACY"].items():
        if isinstance(v, dict) and "counts" in v:
            print("LEGACY/" + sub, json.dumps(v["counts"]))
    print("failed anchor assertions:", failed)
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
