#!/usr/bin/env python3
"""Emit the P2 labelling evidence as JSON, for the data-entry artifact.

Same evidence as `build-p2-worksheet.py` renders into markdown, and it is produced by IMPORTING that
script rather than re-deriving it, so the worksheet and the artifact cannot drift apart: one git read,
two renderings. If this file and the worksheet ever disagree, that is a defect in one of the renderers,
not two opinions about the corpus.

It carries the same blindness rule: `spikes/results/p2-reconstruct.jsonl`'s reconstructed `fine` and
`coarse` scopes and `reconstruct-derived-from.py`'s COARSE table are NOT read and are NOT emitted. They
are the engine's answer to the question the labeller is being asked.

Output: spikes/results/p2-artifact-data.json, and — when p2-labels-template.html sits beside this
script — the built data-entry page at spikes/results/p2-labels-page.html, which is what gets published
as the Artifact.
"""
import importlib.util
import io
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = "spikes/results/p2-artifact-data.json"
HTML_OUT = "spikes/results/p2-labels-page.html"


def load_worksheet_module():
    """Import build-p2-worksheet.py by path — its filename is hyphenated, so it is not importable
    by name. Same trick oracle-mechanical.py uses on build-arm-d-worksheet.py."""
    path = os.path.join(HERE, "build-p2-worksheet.py")
    spec = importlib.util.spec_from_file_location("build_p2_worksheet", path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main():
    ws = load_worksheet_module()
    if not os.path.isdir(ws.REPO):
        sys.exit(f"corpus repo not found at {ws.REPO}")
    if not ws.git("rev-parse", ws.PIN, ok=True).strip():
        sys.exit(f"corpus pin {ws.PIN[:8]} not reachable in {ws.REPO}")

    cards = ws.read_items()
    cases = ws.read_cases()
    if not cases:
        sys.exit("no perturbation cases parsed")

    out = []
    rows = 0
    for c in cases:
        item, upstream, rev = c["item"], c["upstream"], c["replaced_with_rev"]
        path = f".bklg/{item}/{upstream}"

        diff = ws.git("diff", rev, ws.PIN, "--", path, ok=True)
        lines = diff.split("\n") if diff.strip() else []
        clipped = len(lines) > ws.DIFF_LINE_CAP

        found = ws.commits_for_slug(item)
        files, binary = ws.changed_paths(found)
        code = [f for f in files if not f.startswith(".bklg/")]

        commits = []
        for h in found[:8]:
            s = ws.git("show", "-s", "--format=%h %s", h, ok=True).strip()
            if s:
                commits.append(s)

        downstream = []
        for d in c["downstream"]:
            text = ws.git("show", f"{ws.PIN}:.bklg/{item}/{d}", ok=True)
            downstream.append({
                "name": d,
                "preview": ws.body_summary(text) if text.strip() else "",
                "readable": bool(text.strip()),
            })
            rows += 1

        out.append({
            "case": c["case"],
            "item": item,
            "card": cards.get(item, ""),
            "upstream": upstream,
            "rev": rev,
            "revShort": rev[:12],
            "prevDigest": c.get("prev_digest", ""),
            "curDigest": c.get("cur_digest", ""),
            "path": path,
            "diff": "\n".join(lines[:ws.DIFF_LINE_CAP]),
            "diffLineCount": len(lines),
            "diffClipped": clipped,
            "commits": commits,
            "codeFiles": [{"path": f, "binary": f in binary} for f in code[:30]],
            "totalFiles": len(files),
            "noCodeChange": len(code) == 0 and len(files) > 0,
            "noCommits": not found,
            "downstream": downstream,
        })

    payload = {
        "corpusSha": ws.PIN,
        "corpusRepo": ws.REPO,
        "casesFile": ws.CASES,
        "worksheet": ws.OUT,
        "caseCount": len(out),
        "rowCount": rows,
        "cases": out,
    }
    io.open(OUT, "w", encoding="utf-8", newline="\n").write(
        json.dumps(payload, ensure_ascii=False, indent=1) + "\n")
    print(f"wrote {OUT}: {len(out)} cases, {rows} (case, downstream) rows")
    if rows != 41:
        print(f"WARNING: expected 41 rows, got {rows} — check {ws.CASES}")

    # The data-entry page, if its template is beside this script. The template carries a single
    # __DATA__ placeholder inside a <script type="application/json"> block; only "</" needs
    # neutralising there, and that is done rather than assumed.
    tpl_path = os.path.join(HERE, "p2-labels-template.html")
    if os.path.isfile(tpl_path):
        tpl = io.open(tpl_path, encoding="utf-8").read()
        if tpl.count("__DATA__") != 1:
            sys.exit(f"{tpl_path}: expected exactly one __DATA__ placeholder, "
                     f"found {tpl.count('__DATA__')}")
        blob = json.dumps(payload, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")
        io.open(HTML_OUT, "w", encoding="utf-8", newline="\n").write(tpl.replace("__DATA__", blob))
        print(f"wrote {HTML_OUT}: {os.path.getsize(HTML_OUT)} bytes")


if __name__ == "__main__":
    main()
