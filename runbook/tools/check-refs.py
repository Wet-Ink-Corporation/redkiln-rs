"""Check 3 of INDEX.md's validation block: every bare section reference in runbook/ resolves.

The convention (00-preamble.md, Legend): a bare `§N.M` refers to the stage file it appears in;
another stage's section is `sN §N.M`; another document's is prefixed (`design §10.5`, `METHOD-P §1.4`,
`AC §12.10`). This script fails when a bare ref resolves to no heading in its own file and carries no
qualifier naming another document.

It lives in a FILE, not in an INDEX.md heredoc, on purpose. Until 2026-09-12 this check was inlined as
`python - <<'PY'` and its regexes were double-escaped (`\\d` inside a quoted heredoc is a literal
backslash), so it matched 0 headings and 0 references and printed `unresolved bare § refs: 0` on every
tree -- Halt 8 shape (g), inside the validator that polices Halt 8. Escaping a regex through a heredoc
is the hazard; a file removes it. The corpus-wide self-test below is the regression guard.
"""
import re, glob, os, sys
files = {f.replace(os.sep, '/'): open(f, encoding='utf-8').read()
         for f in glob.glob('runbook/**/*.md', recursive=True)}
HEAD = re.compile(r'^#+ .*?(\d+[a-z]?(?:\.\d+)?)', re.M)
REF = re.compile(r'(?<![A-Za-z-])§(\d+[a-z]?\.\d+)')
QUAL = re.compile(r'(design|METHOD[A-Za-z0-9-]*|README|brief|register|audit|charter|RFC'
                  r'|\bAC\b|\bSM\b|\bEA\b|\bs\d+[a-z]?\b|\bP\d+\b|\bF\d+\b|ADR-\d+'
                  r'|[A-Za-z0-9._-]+\.md)', re.I)
CLAUSE = re.compile(r'[.!?][ \t\n]|\|[ \t]|\n[ \t]*[-*][ \t]|\n[ \t]*\n')
bad, heads_total, refs_total = 0, 0, 0
for f, s in sorted(files.items()):
    heads = set(HEAD.findall(s))
    heads_total += len(heads)
    for m in REF.finditer(s):
        refs_total += 1
        clause = CLAUSE.split(s[max(0, m.start() - 200):m.start()])[-1]
        if QUAL.search(clause):
            continue
        g = m.group(1)
        if g in heads or any(h.startswith(g) for h in heads):
            continue
        print('%s:%d unresolved bare section ref %s' % (f, s.count('\n', 0, m.start()) + 1, m.group(0)))
        bad += 1
# self-test: the 2026-09-12 defect was a double-escaped regex that matched nothing and
# therefore always printed 0. If either pattern stops matching corpus-wide, say so.
if heads_total == 0 or refs_total == 0:
    print('FAIL: the checker matched %d headings and %d refs corpus-wide - its regexes are '
          'broken, not the runbook (this is the 2026-09-12 double-escape defect)' % (heads_total, refs_total))
    sys.exit(1)
print('unresolved bare section refs: %d  (over %d refs against %d headings)' % (bad, refs_total, heads_total))
sys.exit(1 if bad else 0)
