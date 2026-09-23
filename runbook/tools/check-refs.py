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


def outside_fences(s):
    """`s` with every fenced block blanked, line count preserved.

    Finding (16), carried into Step 7h and repaired here. `HEAD` below is `^#+ ` under `re.M`, which
    matches a markdown heading -- and ALSO matches a `# ...` comment on the first column of a fenced
    bash block, because a fence is not part of the regex's world. Measured 2026-09-22: the heading
    corpus was **555**, of which only **140** were real headings; **415** were fenced bash comments.
    That inflation is not cosmetic, it is a silent pass: a bare `<section>` ref resolves against any
    heading whose number it prefixes, so 415 spurious "headings" resolve refs that name nothing. With
    fences excluded, `0 unresolved` became **1** -- `phase-1/s7b-kernel.md`'s bare cite of another
    stage's section, now qualified.

    The lines are blanked rather than dropped so that the byte offsets a failure message reports stay
    the file's own.
    """
    out, infence = [], False
    for line in s.split('\n'):
        if line.lstrip().startswith('```'):
            infence = not infence
            out.append('')
            continue
        out.append('' if infence else line)
    return '\n'.join(out)


HEAD = re.compile(r'^#+ .*?(\d+[a-z]?(?:\.\d+)?)', re.M)
REF = re.compile(r'(?<![A-Za-z-])§(\d+[a-z]?\.\d+)')
QUAL = re.compile(r'(design|METHOD[A-Za-z0-9-]*|README|brief|register|audit|charter|RFC'
                  r'|\bAC\b|\bSM\b|\bEA\b|\bs\d+[a-z]?\b|\bP\d+\b|\bF\d+\b|ADR-\d+'
                  r'|[A-Za-z0-9._-]+\.md)', re.I)
CLAUSE = re.compile(r'[.!?][ \t\n]|\|[ \t]|\n[ \t]*[-*][ \t]|\n[ \t]*\n')
bad, heads_total, refs_total = 0, 0, 0
for f, s in sorted(files.items()):
    # Finding (16): headings come from OUTSIDE fenced blocks. Refs are still read from the whole
    # file, because a bare ref inside a fenced block is still a ref a reader has to resolve.
    heads = set(HEAD.findall(outside_fences(s)))
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
