"""Probe: generic numeric-token sweep over the redkiln-rs planning corpus.

Read-only. Writes only into the scratchpad. ASCII-only stdout.
"""
import os
import re
import json
import collections

ROOT = r"D:\repos\redkiln-rs"
OUT = r"C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-redkiln-rs\953a92ec-7709-4a24-b01d-8a5e2c5b86c8\scratchpad\audit\census"


def corpus_files():
    out = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        parts = dirpath.replace(ROOT, "").lower().split(os.sep)
        if ".git" in parts or "fixtures" in parts:
            dirnames[:] = [d for d in dirnames if d not in (".git", "fixtures")]
        if ".git" in parts or "fixtures" in parts:
            continue
        for fn in filenames:
            if fn.endswith(".md"):
                p = os.path.join(dirpath, fn)
                rel = os.path.relpath(p, ROOT).replace("\\", "/")
                out.append((rel, p))
    return sorted(out)


NUM = r"\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?"

PATTERNS = [
    ("pct", re.compile(r"(?<![\w.])(" + NUM + r")\s?%")),
    ("money", re.compile(r"\$\s?(" + NUM + r")")),
    ("mult", re.compile(r"(?<![\w.])(" + NUM + r")\s?[x\u00d7](?![\w])")),
    ("ratio", re.compile(r"(?<![\w.])(" + NUM + r")\s?/\s?(" + NUM + r")(?![\w/.])")),
    ("of", re.compile(r"(?<![\w.])(" + NUM + r")\s+of\s+(" + NUM + r")(?![\w.])")),
    ("unitword", re.compile(
        r"(?<![\w.])(" + NUM + r")\s?"
        r"(tokens?|commits?|runs?|records?|files?|rows?|criteria|criterion|ADRs?|areas?|findings?|"
        r"dispatch(?:es)?|draws?|activities|servers?|tools?|mutants?|slices?|gates?|items?|cases?|"
        r"flips?|decisions?|measures?|stories|hours?|minutes?|seconds?|ms|days?|"
        r"confirmations?|contradictions?|cells?|mutations?|packs?|repos?|ties|cycles?|"
        r"h|s|min|kB|KB|MB|LOC|PRs?|issues?)\b")),
    ("bigcomma", re.compile(r"(?<![\w.$])(\d{1,3}(?:,\d{3})+)(?![\d,])")),
    ("semver", re.compile(r"(?<![\w.])(\d+\.\d+\.\d+)(?![\w.])")),
]

# things to ignore: section references (para 4.4), AC-###, SM-###, ADR-####,
# markdown headings numbers, dates.
SECTION_RE = re.compile(r"[\u00a7#]\s?$")


def main():
    files = corpus_files()
    tok = collections.defaultdict(lambda: {"occ": 0, "files": set(), "kinds": set()})
    total_lines = 0
    for rel, path in files:
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            for lineno, line in enumerate(fh, 1):
                total_lines += 1
                for kind, rx in PATTERNS:
                    for m in rx.finditer(line):
                        pre = line[max(0, m.start() - 2):m.start()]
                        if SECTION_RE.search(pre):
                            continue
                        t = re.sub(r"\s+", "", m.group(0))
                        e = tok[t]
                        e["occ"] += 1
                        e["files"].add(rel)
                        e["kinds"].add(kind)
    rows = []
    for t, e in tok.items():
        if e["occ"] >= 3 and len(e["files"]) >= 2:
            rows.append({"token": t, "occ": e["occ"], "nfiles": len(e["files"]),
                         "kinds": sorted(e["kinds"])})
    rows.sort(key=lambda r: (-r["occ"], r["token"]))
    with open(os.path.join(OUT, "_probe_generic.json"), "w", encoding="utf-8") as fh:
        json.dump({"files": len(files), "lines": total_lines,
                   "distinct_tokens": len(tok), "kept": len(rows), "rows": rows},
                  fh, ensure_ascii=False, indent=1)
    print("files", len(files), "lines", total_lines,
          "distinct", len(tok), "kept", len(rows))


if __name__ == "__main__":
    main()
