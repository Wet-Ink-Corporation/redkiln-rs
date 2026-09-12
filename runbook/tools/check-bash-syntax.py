"""Parse-check every fenced bash block in runbook/ with `bash -n`.

A checkpoint block that does not parse cannot fail *or* pass -- it dies at the first syntax error, which
in a pasted interactive shell looks like noise rather than a result. This runs no command: `bash -n`
reads for syntax only. Blockquoted banner blocks (`> ` prefixed) are unwrapped first.
"""
import re, glob, os, shutil, subprocess, sys, tempfile

# The runbook's shell is Git Bash (00-preamble: "every shell block is POSIX and is executed in Git Bash").
# On Windows a bare `bash` resolves to WSL's, which is a different shell on a different filesystem, so
# name Git Bash explicitly and fall back to PATH elsewhere.
BASH_BIN = next((p for p in (r'C:\Program Files\Git\usr\bin\bash.exe',
                             r'C:\Program Files\Git\bin\bash.exe')
                 if os.path.exists(p)), None) or shutil.which('bash')
if not BASH_BIN:
    print('FAIL: no bash found to parse with')
    sys.exit(1)

BASH = re.compile(r'```bash\n(.*?)```', re.S)
bad = blocks = 0
for path in sorted(glob.glob('runbook/**/*.md', recursive=True)):
    path = path.replace(os.sep, '/')
    text = open(path, encoding='utf-8').read()
    for m in BASH.finditer(text):
        blocks += 1
        line = text.count('\n', 0, m.start(1)) + 1
        body = '\n'.join(re.sub(r'^> ?', '', ln) for ln in m.group(1).split('\n'))
        with tempfile.NamedTemporaryFile('w', suffix='.sh', delete=False,
                                         encoding='utf-8', newline='\n') as fh:
            fh.write(body)
            tmp = fh.name
        try:
            r = subprocess.run([BASH_BIN, '-n', tmp], capture_output=True, text=True)
        finally:
            os.unlink(tmp)
        if r.returncode != 0:
            bad += 1
            print('%s:%d  bash -n failed' % (path, line))
            for e in r.stderr.strip().split('\n')[:6]:
                print('      ' + e.replace(tmp, '<block>'))
print()
print('parsed %d fenced bash blocks; %d failed' % (blocks, bad))
if blocks == 0:
    print('FAIL: no blocks found - the extractor is broken, not the runbook')
    sys.exit(1)
sys.exit(1 if bad else 0)
