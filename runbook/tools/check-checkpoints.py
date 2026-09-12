"""Check 4 of INDEX.md's validation block: every checkpoint line can fail.

`halts.md` Halt 8 enumerates the shapes in which a checkpoint line goes green having asserted nothing,
and states the rule for writing a hardening pass: never put a status-bearing command left of a pipe
(capture with `out=$(cmd 2>&1); rc=$?`, assert `rc` first, then grep the capture); prove a `$(...)`
substitution non-empty before grepping with it; and always `|| { echo "FAIL: ..."; exit 1; }`.

This lints for the shapes a *hardening pass itself* introduces -- Halt 8's (c), (f) and the
print-only count -- because those are the ones that survive review by looking hardened:

  COUNT-ONLY  a `grep -c` / `wc -l` whose only consumer is its `# ->` comment. Nothing compares the
              number to the expectation, so the line prints and passes. Worse when the expectation is
              `0`: `grep -c` over a missing path prints `0` and exits 2, which is indistinguishable.
  PIPE-STATUS a status-bearing command on the LEFT of a pipe. A pipeline's status is its last stage's.
  ECHO-GUARD  `... || echo "FAIL: ..."` used AS the guard. `echo` succeeds, so the guard exits 0.

Backslash continuations are joined before judging, because a guard often sits on the line after the
command it guards.

The allowlist below is enumerated BY FILE AND LINE CONTENT, so the check is
`flagged == allowed`, never "flagged is small". Every entry states why it is admissible -- an
advisory preflight, an operator READ, a sanctioned three-state DEFERRAL, an artifact-producing
redirect, or a guard that fails through its `&&` branch. Adding an entry is a claim; state the reason.
"""
import re, glob, os, sys

# --- justified exceptions -------------------------------------------------------------------
# Each entry: (file suffix, substring that identifies the line, reason).
ALLOW = [
    # `[WILL EXIST]` lines are deferred, not failed -- Halt 8's PIA-003 carve-out. They are written in
    # the form they will take and re-run at the stage they name.
    ('s4a-apply-adr-0089.md', 'lint --facet-provenance', '[WILL EXIST], re-run at Checkpoint 7e'),
    ('s7b-kernel.md', 'lint --facet-stability  .redkiln/facets.toml', '[WILL EXIST] zero-llm branch, exit 0 expected'),
    ('s8-exit-criterion.md', 'lint --facet-stability', '[WILL EXIST], re-run at 7e'),
    ('s8-exit-criterion.md', 'lint --facet-provenance', '[WILL EXIST], re-run at 7e'),
    # Deliberate operator READ screens: the value is read and judged, not thresholded.
    ('s7e-pack-authoring.md', 'grammar-probe.py', 'probe output is READ; the FAIL grep below is the assertion'),
    ('s4a-apply-adr-0089.md', 'BEFORE', 'BEFORE readings (PIA-035): printed for comparison, not asserted'),
    ('s7e-pack-authoring.md', 'spikes/p9/facets.toml', 'pre-flight readings of the spike registry; 7e.1 STOPs on them in prose'),
    ('s7e-pack-authoring.md', 'packs catalog', 're-derive-and-record lines: 7e.2 instructs recording the actual numbers'),
    # Inverted greps whose pass condition is legitimately "no output" -- the operator READS the hits.
    ('INDEX.md', 'Step 7b pack authoring', "check 2 is the stale-claim sweep: every hit is read, not counted"),
    ('INDEX.md', 'roughly \\*\\*60 positives\\*\\*', 'check 2b, same sweep: the retired strings of the 2026-09-12 pass'),
    ('s0-preconditions.md', "status --porcelain | grep -v '^?? runbook/'",
     '§0.1 re-derives tree state for the record; the count is written down, not thresholded'),
    ('s1-commit-spike-output.md', "status --porcelain | grep -v '^??'",
     'staged-work reading before the commit; the assertion is Checkpoint 1 line 1'),
    ('s3-review-consumer-and-labels.md', 'spikes/p1/score-arm-d.py',
     "golden constraint 9: establish the guard's scope by READING the scorer's facet regex"),
    # Artifact-producing redirects: the file they write is what a later line compares.
    ('s2-corpus-extraction.md', './predecessor-refs.count', 'writes the count artifact the split compares against'),
    ('s2-corpus-extraction.md', 'git ls-files spikes/results', 'writes the residency list a later line diffs'),
    # Sanctioned three-state deferrals: DEFERRED is a recorded outcome, not a pass (DECISIONS-2026-09-11 #6).
    ('s2-corpus-extraction.md', 'arm L travelled', 'arm L is outstanding by decision; DEFERRED is the third state'),
    ('s3-review-consumer-and-labels.md', 'DEFERRED, NOT A PASS', 'S-BIN is registered and unrun; the deferral is recorded'),
    ('s9-rollback.md', 'p1-arm-l-labels-ryan.md', 'arm L may never have been labelled; absence is expected'),
    ('s9-rollback.md', "references/adr/0091-", 'ADR-0091 exists only if Step 6.1 was reached'),
    # Correctly guarded via the && branch: the FAIL path carries its own `exit 1`.
    ('s2-corpus-extraction.md', 'p8-state travelled', 'the && branch echoes FAIL and exits 1'),
    # The tool preflight is advisory by design: it REPORTS what to install and the operator installs it.
    # It runs before Step 0 and gates nothing; Checkpoint 5 is where the four tools are actually required.
    ('00-preamble.md', 'MISSING:', 'tool preflight names what to install; it is advisory, not a gate'),
    # BEFORE readings (PIA-035): both branches are informational, printed for comparison with AFTER.
    ('s7e-pack-authoring.md', 'BEFORE: .redkiln exists', 'BEFORE reading; both branches are informational'),
]

BASH = re.compile(r'```bash\n(.*?)```', re.S)
ARROW = re.compile(r'#.*(?:\u2192|->)')
STATUS = re.compile(r'(?:^|[;&]\s*)(grep|test|ls|cargo|git|python|sed|awk|\./target/\S+|redkiln)\b')
COUNTER = re.compile(r'\b(?:grep\s+-[a-zA-Z]*c\b|wc\s+-l)')
ECHOGUARD = re.compile(r'\|\|\s*echo(?!.*\bexit\b)')
SUBST = re.compile(r'\$\([^()]*\)')


def mask(line):
    """Blank out $(...) bodies and the || / && operators, so a real pipe can be seen."""
    prev = None
    while prev != line:                       # nested substitutions
        prev = line
        line = SUBST.sub(lambda m: ' ' * len(m.group(0)), line)
    return line.replace('||', '  ').replace('&&', '  ')


def asserted(line):
    """Does this line carry a mechanism that can make it fail?"""
    return bool(re.search(r'\|\|\s*\{', line) or re.search(r'\brc=\$\?', line)
                or re.match(r'\s*\w+=\$\(', line)
                or re.search(r'\btest\s+["\$\-]', line) or re.search(r'^\s*\[\s', line)
                or re.search(r'\bexit\s+1\b', line))


def allowed(path, line):
    for suffix, needle, _reason in ALLOW:
        if path.endswith(suffix) and needle in line:
            return True
    return False


flagged, scanned = [], 0
for path in sorted(glob.glob('runbook/**/*.md', recursive=True)):
    path = path.replace(os.sep, '/')
    text = open(path, encoding='utf-8').read()
    for block in BASH.finditer(text):
        base = text.count('\n', 0, block.start(1)) + 1
        # Join backslash continuations: a guard often sits on the line after the command it guards,
        # and judging the halves separately reports the first half as unasserted.
        joined, buf, buf_at = [], '', None
        for i, raw in enumerate(block.group(1).split('\n')):
            if buf_at is None:
                buf_at = base + i
            buf += raw.rstrip()
            if buf.endswith('\\'):
                buf = buf[:-1] + ' '
                continue
            joined.append((buf_at, buf))
            buf, buf_at = '', None
        if buf:
            joined.append((buf_at, buf))
        for lineno, raw in joined:
            line = raw.strip()
            if not line or line.startswith('#'):
                continue
            scanned += 1
            kinds = []
            masked = mask(line)
            if COUNTER.search(line) and ARROW.search(line) and not asserted(line):
                kinds.append('COUNT-ONLY')
            if ECHOGUARD.search(line):
                kinds.append('ECHO-GUARD')
            if '|' in masked and not asserted(line):
                head = masked.split('|')[0]
                if STATUS.search(head) and 'printf' not in head:
                    kinds.append('PIPE-STATUS')
            if kinds and not allowed(path, line):
                flagged.append((path, lineno, ','.join(sorted(set(kinds))), line[:104]))

for p, n, k, l in flagged:
    print('%s:%d  [%s]  %s' % (p, n, k, l))
print()
print('scanned %d executable lines in fenced bash blocks; flagged %d; allowlist %d entries'
      % (scanned, len(flagged), len(ALLOW)))
if scanned == 0:
    print('FAIL: no bash blocks parsed - the checker is broken, not the runbook')
    sys.exit(1)
sys.exit(1 if flagged else 0)
