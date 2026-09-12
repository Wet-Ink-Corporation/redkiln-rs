"""S-MIG — the typed census of the frozen corpus (METHOD-S §4). Zero model cost.

For every markdown artifact under `.bklg/` and `.kb/` at the pinned SHA, attempt
to lift each section into the type the proposal (§4.4, layout C) says it has.
Every (artifact, section) gets exactly one outcome:

  Lifted            parsed into the type with nothing left over
  LiftedWithLoss    parsed, but some content had nowhere to go (what is recorded)
  Opaque            kept as prose BY DESIGN — this is a success, not a failure
  EmptyTemplate     the section is still the unfilled scaffold (counted apart:
                    it is neither content nor a parse failure)
  Missing           an expected section is absent (recorded, not failed —
                    not every spec has an EC block)
  Unmapped          a section the proposed type has no slot for
  Failed            the section is present, expected to lift, and could not

Reads committed content via `git show <sha>:<path>`, never the working tree, so
the census is of the pin and nothing else. Deterministic: no model, no network.

Usage:
    python results/census-s-mig.py --redkiln D:/repos/redkiln --sha 47faec9f --out results/s-mig.jsonl
"""

import argparse
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict

import yaml

# ---------------------------------------------------------------------------
# Corpus access — committed state only
# ---------------------------------------------------------------------------


def git(repo, *args):
    out = subprocess.run(["git", *args], cwd=repo, capture_output=True)
    if out.returncode != 0:
        raise RuntimeError(f"git {args}: {out.stderr.decode(errors='replace').strip()}")
    return out.stdout


def list_files(repo, sha):
    names = git(repo, "ls-tree", "-r", "--name-only", sha).decode("utf-8", "replace").splitlines()
    return [n for n in names if n.endswith(".md") and (n.startswith(".bklg/") or n.startswith(".kb/"))]


def show(repo, sha, path):
    return git(repo, "show", f"{sha}:{path}").decode("utf-8", "replace")


# ---------------------------------------------------------------------------
# Markdown mechanics — a section LOCATOR, not a grammar (proposal D5)
# ---------------------------------------------------------------------------

FM_RE = re.compile(r"\A---\r?\n(.*?)\r?\n---\r?\n?", re.S)
H2_RE = re.compile(r"^## (.+?)\s*$", re.M)
ID_RE = re.compile(r"\b(AC|EC|NF)-(\d{1,3})\b")
PATH_RE = re.compile(r"`([^`\n]+?\.[A-Za-z0-9]{1,6}(?::\d+(?:-\d+)?)?)`")


def split_frontmatter(text):
    m = FM_RE.match(text)
    if not m:
        return None, text
    try:
        fm = yaml.safe_load(m.group(1))
    except yaml.YAMLError as e:  # noqa: BLE001
        return {"__error__": str(e).splitlines()[0]}, text[m.end():]
    return (fm if isinstance(fm, dict) else {"__error__": "frontmatter is not a mapping"}), text[m.end():]


def sections(body):
    """[(heading, content)] for every H2; content before the first H2 is ('__preamble__', ...)."""
    out = []
    matches = list(H2_RE.finditer(body))
    pre = body[: matches[0].start()] if matches else body
    out.append(("__preamble__", pre))
    for i, m in enumerate(matches):
        end = matches[i + 1].start() if i + 1 < len(matches) else len(body)
        out.append((m.group(1).strip(), body[m.end():end]))
    return out


def norm_heading(h):
    h = h.lower()
    h = re.sub(r"\(.*?\)", "", h)          # "(progressive disclosure)", "(from #148)"
    h = re.sub(r"[—–-].*$", "", h)          # "Amendment — …" → "amendment"
    return re.sub(r"\s+", " ", h).strip()


def table_rows(content):
    """Rows of the first markdown table in content, as lists of cells (header excluded)."""
    rows, header = [], None
    for line in content.splitlines():
        if not line.strip().startswith("|"):
            if header is not None and rows:
                break
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if header is None:
            header = cells
            continue
        if all(re.fullmatch(r":?-{2,}:?", c) for c in cells if c):
            continue
        rows.append(cells)
    return header, rows


def is_template_placeholder(content):
    """Unfilled scaffold: only prose from the template, an empty numbered list, or an empty table row."""
    stripped = content.strip()
    if not stripped:
        return True
    lines = [l for l in stripped.splitlines() if l.strip()]
    if all(re.fullmatch(r"\d+\.", l.strip()) for l in lines):
        return True
    header, rows = table_rows(content)
    if header and rows and all(all(c == "" for c in r) for r in rows):
        return True
    return False


# ---------------------------------------------------------------------------
# Lifters — one per typed section. Return (outcome, detail).
# ---------------------------------------------------------------------------


def lift_criteria(content, kind, strict=True):
    """AC-/EC-/NF- entries in either the table form or the bullet form.

    `strict` demands GIVEN/WHEN/THEN and a verification per AC (the story spec
    type). Project/initiative cards carry higher-level ACs with neither, and
    that is their type, not a loss."""
    prefix = kind
    entries = []
    # Table form: | AC-001 | criterion | verification |
    header, rows = table_rows(content)
    if header and rows:
        for r in rows:
            cell = r[0].strip("*` ") if r else ""
            if ID_RE.fullmatch(cell) and cell.startswith(prefix):
                entries.append({"id": cell, "criterion": r[1] if len(r) > 1 else "",
                                "verification": r[2] if len(r) > 2 else ""})
    # Bullet form: - AC-001: GIVEN … _Verification:_ …
    if not entries:
        for m in re.finditer(rf"^[ \t]*(?:[-*]\s+)?\**({prefix}-\d{{1,3}})\**[:\s—–-]*(.*?)(?=^[ \t]*(?:[-*]\s+)?\**{prefix}-\d{{1,3}}\b|\Z)",
                             content, re.M | re.S):
            block = m.group(2)
            ver = ""
            vm = re.search(r"(?:_Verification:_|\*\*Verification:?\*\*|\bVerification:|\bTest:)\s*(.*)", block, re.S)
            if vm:
                ver = vm.group(1).strip()
                block = block[: vm.start()]
            entries.append({"id": m.group(1), "criterion": block.strip(), "verification": ver})
    if not entries:
        if is_template_placeholder(content):
            return "EmptyTemplate", {}
        return "Failed", {"why": f"no {prefix}-### entries recognised"}
    loss = []
    for e in entries:
        crit = e["criterion"].upper()
        if strict and kind == "AC" and not ("GIVEN" in crit and "WHEN" in crit and "THEN" in crit):
            loss.append(f"{e['id']}: no GIVEN/WHEN/THEN")
        if strict and kind == "AC" and not e["verification"]:
            loss.append(f"{e['id']}: no verification")
    if loss:
        # 2026-09-11 (audit PIA-078, METHOD-S §7): the loss list is no longer capped at six entries. The recorded
        # run of 2026-09-05 (s-mig.jsonl) was produced under `loss[:6]`, so its per-AC loss counts are truncated;
        # the uncapped counts are in s-mig-report.md "Correction (2026-09-11)".
        return "LiftedWithLoss", {"n": len(entries), "what": loss}
    return "Lifted", {"n": len(entries)}


def lift_paths(content):
    """A file allowlist: table (path | change) or bullets with backticked paths."""
    header, rows = table_rows(content)
    paths = []
    if header and rows:
        paths = [r[0].strip("` ") for r in rows if r and r[0].strip("` ")]
    if not paths:
        paths = [m.group(1) for m in PATH_RE.finditer(content)]
    if not paths:
        return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no paths found"})
    return "Lifted", {"n": len(paths)}


def lift_command(content):
    cmds = re.findall(r"`([^`\n]*(?:pnpm|npm|npx|cargo|dotnet|pytest|make|vitest|node)[^`\n]*)`", content)
    if cmds:
        return "Lifted", {"n": len(cmds)}
    return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no command in backticks"})


def lift_anchors(content):
    header, rows = table_rows(content)
    if not header:
        return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no table"})
    good = [r for r in rows if len(r) >= 3 and r[0].strip("` ")]
    if not good:
        return "EmptyTemplate", {}
    loss = [r[0] for r in good if len(r) < 4]
    if loss:
        return "LiftedWithLoss", {"n": len(good), "what": [f"row without a 'serves AC' column: {l[:40]}" for l in loss]}  # uncapped 2026-09-11 (was loss[:4])
    return "Lifted", {"n": len(good)}


def lift_tdd_table(content):
    header, rows = table_rows(content)
    if not header or not rows:
        return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no TDD table"})
    def has_id(r):
        return any(ID_RE.search(c) for c in r[:3])
    ac_rows = [r for r in rows if r and has_id(r)]
    other = [r[0] for r in rows if r and not has_id(r)]
    if not ac_rows:
        return "Failed", {"why": "table rows carry no AC/EC/NF id"}
    if other:
        return "LiftedWithLoss", {"n": len(ac_rows), "what": [f"row keyed by non-AC label: {o[:50]}" for o in other]}  # uncapped 2026-09-11 (was other[:4])
    # The proposal's per-AC evidence columns (Assertion / Mutation / Test-run / Red?) do not exist
    # in this corpus by construction (F25 remediation is staged, not applied); record that as loss
    # of a different kind so it is not mistaken for regularity.
    cols = [h.lower() for h in header]
    if not any("mutation" in c for c in cols):
        # The proposal's per-AC Assertion/Mutation/Test-run/Red? columns (F25) are absent by
        # construction in this corpus; migration fills them as `unknown`. Not content lost.
        return "Lifted", {"n": len(ac_rows), "note": "pre-F25 columns; mutation evidence absent"}
    return "Lifted", {"n": len(ac_rows)}


def lift_gates_table(content):
    header, rows = table_rows(content)
    if header and rows and any(len(r) >= 2 for r in rows):
        return "Lifted", {"n": len(rows)}
    # Bullet form: "- `pnpm test` — green: 92 files" — a command plus a result word per line.
    bullets = [l for l in content.splitlines() if re.match(r"^\s*[-*]\s+.*`[^`]+`", l)
               and re.search(r"\b(green|pass(ed)?|red|fail(ed)?|exit \d|\d+ tests?)\b", l, re.I)]
    if bullets:
        return "LiftedWithLoss", {"n": len(bullets), "what": ["gate results as bullets, not a table"]}
    fenced = re.search(r"```[^\n]*\n(.*?)```", content, re.S)
    if fenced and re.search(r"\b(green|pass(ed)?|red|fail(ed)?)\b", fenced.group(1), re.I):
        return "LiftedWithLoss", {"n": len(fenced.group(1).strip().splitlines()), "what": ["gate results in a fenced block, not a table"]}
    return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no gate table"})


def lift_coverage_map(content):
    header, rows = table_rows(content)
    if not header or not rows:
        return ("EmptyTemplate", {}) if is_template_placeholder(content) else ("Failed", {"why": "no coverage table"})
    ok = [r for r in rows if r and ID_RE.search(r[0])]
    if not ok:
        return "Failed", {"why": "coverage rows carry no AC id"}
    return ("Lifted", {"n": len(ok)}) if len(ok) == len(rows) else ("LiftedWithLoss", {"n": len(ok), "what": [f"{len(rows)-len(ok)} rows without an AC id"]})


def lift_checklist(content):
    boxes = re.findall(r"^- \[( |x|X)\]", content, re.M)
    if not boxes:
        return "Failed", {"why": "no checkboxes"}
    return "Lifted", {"n": len(boxes), "ticked": sum(1 for b in boxes if b.lower() == "x")}


def lift_ledger_yaml(body):
    m = re.search(r"```yaml\s*\n(.*?)```", body, re.S)
    if not m:
        return "Failed", {"why": "no fenced yaml block"}
    try:
        rows = yaml.safe_load(m.group(1)) or []
    except yaml.YAMLError as e:  # noqa: BLE001
        return "Failed", {"why": f"yaml: {str(e).splitlines()[0]}"}
    if not isinstance(rows, list) or not rows:
        return "EmptyTemplate", {}
    bad = [r.get("id", "?") for r in rows if not isinstance(r, dict) or not {"id", "criterion", "satisfied", "evidence"} <= set(r)]
    if bad:
        return "LiftedWithLoss", {"n": len(rows), "what": [f"row missing required keys: {b}" for b in bad]}  # uncapped 2026-09-11 (was bad[:4])
    return "Lifted", {"n": len(rows)}


def lift_ordered_list(content):
    items = re.findall(r"^\d+\.\s+\S", content, re.M)
    if items:
        return "Lifted", {"n": len(items)}
    if re.search(r"^\d+\.\s*$", content, re.M) or is_template_placeholder(content):
        return "EmptyTemplate", {}
    return "Failed", {"why": "no ordered steps"}


def lift_signal_table(content):
    header, rows = table_rows(content)
    if header and rows and any(any(c for c in r) for r in rows):
        return "Lifted", {"n": len(rows)}
    return "EmptyTemplate", {}


# ---------------------------------------------------------------------------
# Per-artifact schemas: expected frontmatter keys + section → (slot, lifter|None for opaque)
# ---------------------------------------------------------------------------

CARD_FM = {"id", "uid", "type", "slug", "title", "parent", "initiative", "project", "status", "process",
           "stage", "automation", "severity", "blocked_by", "blocks", "owner", "created", "updated",
           "links", "schema", "process_rev", "gate_open", "terminal", "tier"}
STAGE_FM = {"item", "stage", "created", "updated", "template_sig", "rendered_sig", "status", "title"}
REVIEW_FM = {"title", "story", "slug", "verdict", "overall", "rubric", "item", "stage", "created", "updated",
             "initiative_slug", "project_slug", "terminal", "dod_green", "checklist"}
KB_FM = {"id", "title", "kind", "status", "authority_tier", "reversibility", "phase", "supersedes",
         "superseded_by", "summary", "depends_on", "related", "source_paths", "created", "updated",
         "tags", "owner", "layer", "wave", "aliases", "see_also", "decided", "review_by", "status_note"}

SPEC = {
    # typed
    "acceptance criteria": ("ac", lambda c: lift_criteria(c, "AC")),
    "error conditions": ("ec", lambda c: lift_criteria(c, "EC")),
    "non-functional": ("nf", lambda c: lift_criteria(c, "NF")),
    "frozen pr boundary": ("boundary", lift_paths),
    "pr boundary": ("boundary", lift_paths),
    "frozen boundary": ("boundary", lift_paths),
    "merge gate": ("merge_gate", lift_command),
    "tests and ci": ("merge_gate", lift_command),
    "anchors": ("anchors", lift_anchors),
    # opaque by design
    "context pack": ("prose.context", None), "repro": ("prose.repro", None),
    "behavior": ("prose.behavior", None), "behavior and interfaces": ("prose.behavior", None),
    "out of scope": ("prose.out_of_scope", None), "executive summary": ("prose.summary", None),
    "one": ("prose.summary", None), "scope lock": ("prose.scope", None),
    "implementation notes": ("prose.notes", None), "clarifications resolved during spec": ("prose.notes", None),
    "dependencies": ("prose.deps", None), "data and migrations": ("prose.data", None),
    "risks and coupling": ("prose.risks", None), "interaction quality": ("prose.interaction", None),
    "integration contract": ("prose.integration", None), "renders surfaces": ("prose.surfaces", None),
    "renders surface": ("prose.surfaces", None), "compatibility note": ("prose.notes", None),
    "amendment": ("prose.amendment", None), "scope amendment": ("prose.amendment", None),
}
IMPL_REPORT = {
    "tdd evidence": ("tdd", lift_tdd_table), "gates": ("gates", lift_gates_table),
    "commits": ("commits", None), "changes": ("prose.changes", None), "notes": ("prose.notes", None),
    # older report generation
    "findings ledger": ("findings_table", lift_gates_table), "acceptance": ("prose.acceptance", None),
    "knowledge harvest": ("prose.harvest", None),
}
REVIEW = {
    "coverage map": ("coverage", lift_coverage_map), "ac": ("coverage", lift_coverage_map),
    "rubric": ("rubric_table", lift_gates_table), "affected": ("gate_table", lift_gates_table),
    "checkpoint": ("checkpoint", None), "required changes": ("prose.required_changes", None),
    "evidence": ("prose.evidence", None), "per": ("rubric_table", lift_gates_table), "checklist": ("gate_checklist", lift_checklist),
    "findings": ("prose.findings", None), "summary": ("prose.summary", None), "verdict": ("prose.verdict", None),
}
PLAN = {"sequencing": ("steps", lift_ordered_list), "tests first": ("prose.tests", None), "risks": ("prose.risks", None)}
DISCOVER = {"signal ledger": ("signals", lift_signal_table), "questions": ("prose.questions", None),
            "decision": ("prose.decision", None), "gate": ("gate_checklist", lift_checklist)}
INTAKE = {"problem": ("prose", None), "desired outcome": ("prose", None), "constraints": ("prose", None),
          "open questions": ("prose", None), "gate": ("gate_checklist", lift_checklist), "research angles": ("prose.research", None)}
CARD = {"summary": ("prose", None), "goals": ("prose", None), "stories": ("prose", None), "notes": ("prose", None),
        # archived-generation project / initiative / bug / incident bodies
        "acceptance criteria": ("ac", lambda c: lift_criteria(c, "AC", strict=False)),
        "business requirements matrix": ("br_table", lift_gates_table),
        "definition of done": ("prose.dod", None), "one": ("prose.summary", None),
        "how this advances the initiative": ("prose.summary", None), "in scope": ("prose.scope", None),
        "out of scope": ("prose.scope", None), "derived requirements": ("prose.requirements", None),
        "dependencies": ("prose.deps", None), "risks and coupling notes": ("prose.risks", None),
        "risks": ("prose.risks", None), "context anchors": ("prose.anchors", None), "companions": ("prose.companions", None),
        "what shipped": ("prose.shipped", None), "timeline": ("prose.timeline", None), "impact": ("prose.impact", None),
        "mitigation": ("prose.mitigation", None), "findings ledger": ("findings_table", lift_gates_table),
        "reproduction": ("prose.repro", None), "vision and narrative": ("prose.vision", None),
        "non": ("prose.scope", None), "who this is for": ("prose.audience", None),
        "referenced personas & journeys": ("prose.audience", None), "why now": ("prose.vision", None),
        "open design tensions": ("prose.open", None), "open questions for the planning team": ("prose.open", None)}

KIND_OF = {
    "spec.md": ("spec", STAGE_FM, SPEC), "implementation-report.md": ("impl-report", STAGE_FM, IMPL_REPORT),
    "report.md": ("report", STAGE_FM, IMPL_REPORT), "_review.md": ("review", REVIEW_FM, REVIEW),
    "plan.md": ("plan", STAGE_FM, PLAN), "discover.md": ("discover", STAGE_FM, DISCOVER),
    "_intake-brief.md": ("intake-brief", STAGE_FM, INTAKE), "_ledger.md": ("ledger", STAGE_FM, None),
    "story.md": ("card.story", CARD_FM, CARD), "project.md": ("card.project", CARD_FM, CARD),
    "initiative.md": ("card.initiative", CARD_FM, CARD), "incident.md": ("card.incident", CARD_FM, CARD),
    "bug.md": ("card.bug", CARD_FM, CARD),
}
SIDECAR_COMPANIONS = {"_storymap.md", "_decomposition.md", "_slices.md", "_integration.md", "_grounding.md",
                      "_design.md", "_design-review.md", "closure.md", "_retrospective.md", "_plan.md",
                      "_implementation.md"}


KEY_CENSUS = defaultdict(Counter)   # kind -> key -> count; filled before lifting
KIND_COUNT = Counter()


def lift_frontmatter(fm, expected, kind):
    if fm is None:
        return "Failed", {"why": "no frontmatter"}
    if "__error__" in fm:
        return "Failed", {"why": fm["__error__"]}
    keys = set(fm)
    # A key is "known" if the corpus itself uses it on >= 2% of this kind's files.
    n_kind = KIND_COUNT[kind] or 1
    known = {k for k, c in KEY_CENSUS[kind].items() if c / n_kind >= 0.02}
    extra = sorted(keys - expected - known)
    core = {"card": {"id", "type", "slug", "title", "status", "stage"}, "stage": {"item", "stage"},
            "review": {"verdict"}, "kb": {"id", "title", "kind", "status"}}
    need = core["card"] if kind.startswith("card") else core["review"] if kind == "review" else core["kb"] if kind == "kb" else core["stage"]
    if kind == "review" and "dod_green" in keys:
        need = {"dod_green"}          # the project-review generation
    missing = sorted(need - keys)
    if missing:
        return "Failed", {"why": f"missing required keys {missing}"}
    if extra:
        return "LiftedWithLoss", {"what": [f"unknown key `{k}`" for k in extra]}  # uncapped 2026-09-11 (was extra[:6])
    return "Lifted", {}


def census_file(path, text, kb_ids):
    name = path.rsplit("/", 1)[-1]
    fm, body = split_frontmatter(text)
    recs = []

    def rec(section, slot, outcome, detail=None):
        recs.append({"path": path, "kind": kind, "section": section, "slot": slot, "outcome": outcome, **(detail or {})})

    if path.startswith(".kb/"):
        kind = "kb"
        if name in ("README.md",) or any(seg in path for seg in ("/_templates/", "/_intake/", "/_governance/")):
            kind = "kb.meta"
            rec("__file__", "meta", "Opaque")
            return recs
        rec("frontmatter", "frontmatter", *lift_frontmatter(fm, KB_FM, "kb"))
        if isinstance(fm, dict) and "__error__" not in fm:
            dangling = [r for k in ("related", "depends_on") for r in (fm.get(k) or []) if isinstance(r, str) and r not in kb_ids]
            rec("links", "links", "LiftedWithLoss" if dangling else "Lifted",
                {"what": [f"dangling link {d}" for d in dangling]} if dangling else {"n": len((fm.get('related') or [])) + len((fm.get('depends_on') or []))})  # uncapped 2026-09-11 (was dangling[:4])
        rec("body", "prose.body", "Opaque")
        return recs

    if any(seg in path for seg in ("/_discovery/", "/research/", "/_triage/", "/design/")) or name.startswith("_finding-") or name == "_closeout-ledger.md":
        kind = "discovery"
        rec("__file__", "sidecar.body", "Opaque")
        return recs

    if name in SIDECAR_COMPANIONS:
        kind = "companion." + name.strip("_").removesuffix(".md")
        rec("frontmatter", "frontmatter", *lift_frontmatter(fm, STAGE_FM | REVIEW_FM | {"project", "story", "slug", "verdict", "overall", "rubric", "captures", "surfaces"}, "stage"))
        rec("body", "sidecar.body", "Opaque")
        return recs

    if name not in KIND_OF:
        kind = "other"
        rec("__file__", "unmapped", "Unmapped", {"why": f"no schema for {name}"})
        return recs

    kind, fm_keys, schema = KIND_OF[name]
    rec("frontmatter", "frontmatter", *lift_frontmatter(fm, fm_keys, kind))

    if kind == "ledger":
        rec("ledger", "ledger", *lift_ledger_yaml(body))
        return recs

    seen_slots = set()
    for heading, content in sections(body):
        if heading == "__preamble__":
            continue
        nh = norm_heading(heading)
        match = schema.get(nh)
        if match is None:
            # prefix match for headings like "AC-### coverage map", "Gate: Discover", "Affected-area gate result"
            for key, val in schema.items():
                if nh.startswith(key) or key.startswith(nh):
                    match = val
                    break
        if match is None:
            rec(heading, "unmapped", "Unmapped", {"why": "no slot for this heading"})
            continue
        slot, lifter = match
        seen_slots.add(slot)
        if lifter is None:
            rec(heading, slot, "EmptyTemplate" if is_template_placeholder(content) else "Opaque")
        else:
            rec(heading, slot, *lifter(content))

    expected = {"spec": ["ac", "boundary", "anchors"], "impl-report": ["tdd", "gates"], "review": ["coverage"],
                "plan": ["steps"], "discover": ["signals", "gate_checklist"]}.get(kind, [])
    for slot in expected:
        if slot not in seen_slots:
            rec(f"(expected {slot})", slot, "Missing")
    return recs


# ---------------------------------------------------------------------------
# Reporting — METHOD-S §4.3
# ---------------------------------------------------------------------------


def report(recs):
    by_kind = defaultdict(Counter)
    by_slot = defaultdict(Counter)
    for r in recs:
        by_kind[r["kind"]][r["outcome"]] += 1
        by_slot[(r["kind"], r["slot"])][r["outcome"]] += 1

    order = ["Lifted", "LiftedWithLoss", "Opaque", "EmptyTemplate", "Missing", "Unmapped", "Failed"]
    print(f"\nS-MIG census — {len(recs)} (artifact, section) records\n")
    print(f"  {'kind':<28}" + "".join(f"{o:>16}" for o in order) + "   lift-or-opaque share")
    for kind in sorted(by_kind):
        c = by_kind[kind]
        typed_total = sum(c[o] for o in ("Lifted", "LiftedWithLoss", "Failed"))
        share = (c["Lifted"] / typed_total) if typed_total else float("nan")
        print(f"  {kind:<28}" + "".join(f"{c[o]:>16}" for o in order) + f"   {share:>6.0%} of typed sections Lifted" if typed_total else f"  {kind:<28}" + "".join(f"{c[o]:>16}" for o in order))

    print("\nper typed slot (decision rule §4.3: >= 95% Lifted stands; < 80% is reclassified opaque in Phase 1)")
    print(f"  {'kind / slot':<40} {'Lifted':>7} {'w/loss':>7} {'Failed':>7} {'empty':>7}  share   verdict")
    for (kind, slot), c in sorted(by_slot.items()):
        n = c["Lifted"] + c["LiftedWithLoss"] + c["Failed"]
        if n == 0 or slot.startswith("prose") or slot in ("unmapped", "meta", "sidecar.body", "prose"):
            continue
        share = c["Lifted"] / n
        verdict = "stands" if share >= 0.95 else ("REVIEW" if share >= 0.80 else "RECLASSIFY-OPAQUE")
        print(f"  {kind + ' / ' + slot:<40} {c['Lifted']:>7} {c['LiftedWithLoss']:>7} {c['Failed']:>7} {c['EmptyTemplate']:>7}  {share:>5.0%}   {verdict}")

    print("\nfrontmatter key census per kind (keys on >= 2% of files count as the type; the rest are loss)")
    for kind in sorted(KEY_CENSUS):
        n = KIND_COUNT[kind]
        if n == 0:
            continue
        keys = ", ".join(f"{k}:{c}" for k, c in sorted(KEY_CENSUS[kind].items(), key=lambda kv: -kv[1]))
        print(f"  {kind} (n={n}): {keys}")

    print("\nloss reasons (top 15)")
    loss = Counter()
    for r in recs:
        if r["outcome"] == "LiftedWithLoss":
            for w in r.get("what", []):
                loss[re.sub(r"(AC|EC|NF)-\d+: ", "", re.sub(r"`[^`]*`", "`…`", w))[:70]] += 1
    for w, n in loss.most_common(15):
        print(f"  {n:>5}  {w}")

    print("\nfailures (top 15 reasons)")
    fails = Counter(f"{r['kind']} / {r['slot']}: {r.get('why','?')}" for r in recs if r["outcome"] == "Failed")
    for w, n in fails.most_common(15):
        print(f"  {n:>5}  {w}")

    print("\nunmapped headings (top 20) — candidates for a slot or for opaque-by-design")
    un = Counter(f"{r['kind']}: {r['section']}" for r in recs if r["outcome"] == "Unmapped")
    for w, n in un.most_common(20):
        print(f"  {n:>5}  {w}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--redkiln", required=True)
    ap.add_argument("--sha", default="47faec9f")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()

    files = list_files(args.redkiln, args.sha)
    texts = {p: show(args.redkiln, args.sha, p) for p in files}
    kb_ids = set()
    for p, t in texts.items():
        if p.startswith(".kb/"):
            fm, _ = split_frontmatter(t)
            if isinstance(fm, dict) and isinstance(fm.get("id"), str):
                kb_ids.add(fm["id"])

    # Key census first: which frontmatter keys each artifact kind actually carries.
    for p, t in texts.items():
        name = p.rsplit("/", 1)[-1]
        kind = "kb" if p.startswith(".kb/") else (KIND_OF[name][0] if name in KIND_OF else
                "companion." + name.strip("_").removesuffix(".md") if name in SIDECAR_COMPANIONS else None)
        if kind is None:
            continue
        fm, _ = split_frontmatter(t)
        KIND_COUNT[kind] += 1
        if isinstance(fm, dict) and "__error__" not in fm:
            for k in fm:
                KEY_CENSUS[kind][k] += 1

    recs = []
    for p in sorted(files):
        recs.extend(census_file(p, texts[p], kb_ids))

    with open(args.out, "w", encoding="utf-8") as f:
        f.write(json.dumps({"record": "header", "experiment": "S-MIG", "corpus_sha": args.sha,
                            "files": len(files), "kb_ids": len(kb_ids)}) + "\n")
        for r in recs:
            f.write(json.dumps(r, ensure_ascii=False) + "\n")
    print(f"corpus {args.sha}: {len(files)} markdown files under .bklg/ and .kb/; wrote {args.out}")
    report(recs)


if __name__ == "__main__":
    sys.exit(main())
