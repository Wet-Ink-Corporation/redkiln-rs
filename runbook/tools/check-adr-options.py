"""ADR-0085's option headings, enforced from ADR-0164 on (s8a §8a.7; D21-42 decision 24a).

ADR-0085 says each considered option is one `### Option — <name>` heading. Thirty accepted records (ADR-0132 through
ADR-0161) do not follow it. Erratum E-0085-3 lists them, and their accepted bodies are not edited. This lint holds
every record from ADR-0164 on to the rule, so the gap cannot grow. (Noted 2026-09-25, s8a fix round 2: ADR-0090
and ADR-0117, accepted before ADR-0132, also carry no such heading. They are outside D21-42 decision 24a's list and
E-0085-3 names them in a dated note; this lint does not reach them.)

For each `references/adr/NNNN-*.md` with NNNN >= 0164 (or each such file in `--dir <path>`, for the controls):

  - the record must have a `## Considered options` section, running to the next `## ` heading or the end;
  - that section must hold at least one `### Option — <name>` heading (an em dash, then a non-empty name);
  - it must hold no `###` heading of any other form (`### Option: x`, `### Option - x`, `### A`, ...).

Fenced code blocks are skipped, so a `###` line inside a fence is not a heading. CRLF and LF read the same.

Output: one `FLAG <path>: <reason>` line per violation, then `scanned N records (>= 0164); flagged K`.
Exit 1 when K > 0 **or when N = 0**. A lint over zero records is the `lattice.py --packs <path>` green that
printed a baseline over zero packs, so an empty scan fails. Exit 2 on a usage error.
"""
import glob, os, re, sys

FLOOR = 164
OPTION = re.compile(r"^### Option — \S.*$")
H3 = re.compile(r"^###(?!#)")
H2 = re.compile(r"^##(?!#)")
NAME = re.compile(r"^(\d{4})-.*\.md$")


def records(root):
    out = []
    for p in sorted(glob.glob(os.path.join(root, "*.md"))):
        m = NAME.match(os.path.basename(p))
        if m and int(m.group(1)) >= FLOOR:
            out.append(p)
    return out


def check(path):
    lines = open(path, "rb").read().decode("utf-8").replace("\r\n", "\n").split("\n")
    starts = [i for i, l in enumerate(lines) if l.rstrip() == "## Considered options"]
    if len(starts) != 1:
        return ["%d '## Considered options' sections, expected exactly 1" % len(starts)]
    bad, good, fence = [], 0, False
    for k in range(starts[0] + 1, len(lines)):
        l = lines[k]
        if l.lstrip().startswith("```"):
            fence = not fence
            continue
        if fence:
            continue
        if H2.match(l):
            break
        if H3.match(l):
            if OPTION.match(l.rstrip()):
                good += 1
            else:
                bad.append("line %d is a ### heading not of the form '### Option — <name>': %s" % (k + 1, l.strip()[:100]))
    if good == 0:
        bad.append("no '### Option — <name>' heading under '## Considered options'")
    return bad


def main(argv):
    root = os.path.join("references", "adr")
    if argv:
        if len(argv) != 2 or argv[0] != "--dir":
            print("usage: check-adr-options.py [--dir <path>]")
            return 2
        root = argv[1]
    if not os.path.isdir(root):
        print("FLAG %s: not a directory" % root)
        print("scanned 0 records (>= %04d); flagged 0" % FLOOR)
        return 1
    paths = records(root)
    flagged = 0
    for p in paths:
        reasons = check(p)
        if reasons:
            flagged += 1
            for r in reasons:
                print("FLAG %s: %s" % (p.replace(os.sep, "/"), r))
    print("scanned %d records (>= %04d); flagged %d" % (len(paths), FLOOR, flagged))
    return 1 if flagged or not paths else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
