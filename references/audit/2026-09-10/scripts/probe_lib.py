"""Shared corpus loader + ASCII-safe probe printer (scratchpad only)."""
import os
import re
import sys

ROOT = r"D:\repos\redkiln-rs"


def corpus_files():
    out = []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in (".git", "fixtures")]
        for fn in filenames:
            if fn.endswith(".md"):
                p = os.path.join(dirpath, fn)
                rel = os.path.relpath(p, ROOT).replace("\\", "/")
                out.append((rel, p))
    return sorted(out)


_CACHE = None


def corpus_lines():
    """[(rel, lineno, text)] for all corpus files (cached)."""
    global _CACHE
    if _CACHE is None:
        _CACHE = []
        for rel, path in corpus_files():
            with open(path, "r", encoding="utf-8", errors="replace") as fh:
                for i, line in enumerate(fh, 1):
                    _CACHE.append((rel, i, line.rstrip("\n")))
    return _CACHE


def asc(s):
    return s.encode("ascii", "replace").decode("ascii")


def probe(label, pattern, ctx=60, maxshow=6, flags=0):
    rx = re.compile(pattern, flags)
    hits = []
    for rel, ln, text in corpus_lines():
        for m in rx.finditer(text):
            a = max(0, m.start() - ctx)
            b = min(len(text), m.end() + ctx)
            hits.append((rel, ln, text[a:b]))
    files = sorted({h[0] for h in hits})
    print("## %s  occ=%d files=%d  /%s/" % (label, len(hits), len(files), asc(pattern)))
    seen = set()
    shown = 0
    for rel, ln, c in hits:
        key = re.sub(r"\s+", " ", c)[:80]
        if key in seen:
            continue
        seen.add(key)
        print("   %s:%d  %s" % (rel, ln, asc(re.sub(r"\s+", " ", c))))
        shown += 1
        if shown >= maxshow:
            break
    sys.stdout.flush()
