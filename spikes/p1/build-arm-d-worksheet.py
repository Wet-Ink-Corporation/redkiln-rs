#!/usr/bin/env python3
"""Build the P1 arm D labelling worksheet.

METHOD-P §0 rule 1 says the labels are written from THE ITEM AND ITS DIFF ALONE, before the engine's
output is seen. This script assembles exactly that raw material and nothing else.

IT DOES NOT READ spikes/results/p1-arm-d.jsonl. That file is the engine's answer; showing it would make
the comparison vacuous. Nor does it reuse derive-facets.py's derivation rules — if the script's own path
extraction is wrong, the labeller must be able to disagree with it, which requires seeing the primary
source rather than the script's reading of it. The path index below is a neutral regex over the card text,
offered as an index INTO the card, never as a conclusion drawn FROM it.

Output is the label file itself, pre-populated with context, so there is no copy step: fill the value
cells in place and score it directly.
"""
import io, os, re, subprocess, sys

REPO = "D:/repos/redkiln"
PIN = "47faec9f46b8d983c2c75f0794164e68415ae706"
ITEMS = "spikes/plans/p1-items.txt"
OUT = "spikes/results/p1-arm-d-labels-ryan.md"

FACETS = ["toolchain", "change", "binary_change", "auth", "data_migration", "ui", "flag_exists", "recent_deploy"]
LEGEND = {
    "toolchain":      'set from {node, dotnet, python, rust, unreal} — comma-separated if several',
    "change":         'set from {text, binary, generated} — comma-separated if several',
    "binary_change":  '1/0 — did the change touch a binary asset? (NOT in facets.toml; measured anyway, decision #11)',
    "auth":           '1/0 — does the change touch authentication/authorisation code?',
    "data_migration": '1/0 — does it carry a schema or data migration?',
    "ui":             '1/0 — does it change a user-visible surface?',
    "flag_exists":    '1/0 — is a feature flag involved?',
    "recent_deploy":  'pre-filled ? — underivable at this pin, no deploy_log in the frozen corpus (decision #12)',
}


def git(*a, ok=False):
    r = subprocess.run(["git", "-C", REPO] + list(a), capture_output=True, text=True, encoding="utf-8", errors="replace")
    if r.returncode and not ok:
        return ""
    return r.stdout


def commits_for_slug(slug):
    """The item's real commits, found the way redkiln links them: the slug leaf in the subject
    (`fix(<leaf>): ...`) and the Story:/Fix: trailers. Ancestors of the pin only."""
    leaf = slug.rstrip("/").split("/")[-1]
    seen = []
    for pat in (leaf, f"Story: {leaf}", f"Fix: {leaf}"):
        out = git("log", PIN, "--format=%H", "--fixed-strings", f"--grep={pat}", ok=True)
        for h in out.split():
            if h not in seen:
                seen.append(h)
    return seen


def changed_paths(shas):
    """Union of files touched by those commits. git's own --numstat marks binaries as '-\t-'."""
    paths, binary = [], set()
    for h in shas[:12]:
        out = git("show", "--numstat", "--format=", h, ok=True)
        for line in out.splitlines():
            parts = line.split("\t")
            if len(parts) == 3:
                add, dele, path = parts
                if path not in paths:
                    paths.append(path)
                if add == "-" and dele == "-":
                    binary.add(path)
    return paths, binary


def read_items():
    out = []
    for line in io.open(ITEMS, encoding="utf-8"):
        line = line.rstrip("\n")
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 3:
            continue
        out.append((parts[0], parts[1], parts[2]))
    return out


def frontmatter(card_text):
    m = re.match(r"^---\n(.*?)\n---\n", card_text, re.S)
    return m.group(1) if m else ""


def fm_get(fm, key):
    m = re.search(rf"^{key}:\s*(.*)$", fm, re.M)
    return m.group(1).strip() if m else ""


def linked_commits(fm):
    m = re.search(r"^\s*commits:\s*(\[.*?\]|\n(?:\s+-\s*.*\n)*)", fm, re.M)
    if not m:
        return []
    blob = m.group(1)
    return re.findall(r"[0-9a-f]{7,40}", blob)


def body_summary(card_text, n=22):
    body = re.sub(r"^---\n.*?\n---\n", "", card_text, flags=re.S)
    lines = [l for l in body.split("\n")]
    # drop the leading H1 and blank lines
    while lines and (not lines[0].strip() or lines[0].startswith("# ")):
        lines.pop(0)
    keep, blanks = [], 0
    for l in lines:
        if len(keep) >= n:
            break
        if not l.strip():
            blanks += 1
            if blanks > 1:
                continue
        else:
            blanks = 0
        keep.append(l)
    return "\n".join(keep).rstrip()


PATH_RE = re.compile(r"`([^`\s]*?/[^`\s]*?\.[A-Za-z0-9]{1,6})`|(?<![\w/])((?:src|docs|test|tests|scripts|tools|bin|workflows|commands|agents|skills|templates|processes|hooks|new|brand|kb-skeleton)/[\w./-]+\.[A-Za-z0-9]{1,6})")


def path_index(card_text):
    hits = []
    for m in PATH_RE.finditer(card_text):
        p = m.group(1) or m.group(2)
        if p and p not in hits:
            hits.append(p)
    return hits


def main():
    if not os.path.isdir(REPO):
        sys.exit(f"corpus repo not found at {REPO}")
    items = read_items()
    if not items:
        sys.exit("no items parsed")
    o = []
    A = o.append
    A("# P1 arm D — human labels (Ryan). Worksheet-populated 2026-09-09.")
    A("#")
    A("# METHOD-P §0 rule 1: fill this from the item and its diff ALONE. Everything you need is inline below;")
    A("# each item block carries the card's own summary, the paths it names, and any linked commits.")
    A("# The engine's answers (spikes/results/p1-arm-d.jsonl) were NOT read when building this file and must")
    A("# not be opened until every value cell is filled.")
    A("#")
    A("# HOW TO FILL: one row per (item, facet). Put the value in the third cell. Leave `?` where the item")
    A("# alone cannot tell — `?` is a real answer and is excluded from scoring, not counted against the script.")
    A("#")
    for f in FACETS:
        A(f"#   {f:15} {LEGEND[f]}")
    A("#")
    A("# WHEN DONE: set labelled_at to the UTC time you finished (must end in Z), fill blind_attestation, then")
    A("#   python spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl \\")
    A("#     --labels spikes/results/p1-arm-d-labels-ryan.md --attested-blind")
    A("# Any facet printing 'LEAVES THE MODEL (>=0.95)' never needs a human or a model again.")
    A("")
    A("labelled_at: FILL-ME-ISO8601Z")
    A("labeller: FILL-ME")
    A("blind_attestation: FILL-ME (e.g. \"Ryan Britton attests on 2026-09-09 that he had not opened spikes/results/p1-arm-d.jsonl before completing these labels\")")
    A(f"corpus_sha: {PIN}")
    A(f"items_file: {ITEMS}")
    A("")

    for n, (kind, slug, card) in enumerate(items, 1):
        text = git("show", f"{PIN}:{card}")
        fm = frontmatter(text)
        title = fm_get(fm, "title") or "(no title in frontmatter)"
        sev = fm_get(fm, "severity")
        status = fm_get(fm, "status")
        typ = fm_get(fm, "type") or kind
        commits = linked_commits(fm)
        paths = path_index(text)

        A("---")
        A("")
        A(f"## {n}/{len(items)} — {title}")
        A("")
        A(f"`{slug}`")
        A("")
        meta = f"**type:** {typ}"
        if sev:
            meta += f" · **severity:** {sev}"
        if status:
            meta += f" · **status:** {status}"
        A(meta)
        A("")
        if not text:
            A("> **CARD NOT READABLE at the pin.** Label every facet `?` and note it.")
            A("")
        else:
            A("<details><summary>card summary (first lines of the body)</summary>")
            A("")
            A("```")
            A(body_summary(text))
            A("```")
            A("")
            A("</details>")
            A("")
        if paths:
            A(f"**Paths named in the card** ({len(paths)}) — an index into the text above, not a conclusion:")
            A("")
            for p in paths[:24]:
                A(f"- `{p}`")
            if len(paths) > 24:
                A(f"- …and {len(paths)-24} more")
            A("")
        else:
            A("**Paths named in the card:** none found by the index. Read the card body above.")
            A("")
        found = commits_for_slug(slug)
        for c in commits:
            full = git("rev-parse", c, ok=True).strip()
            if full and full not in found:
                found.insert(0, full)
        files, binary = changed_paths(found)
        if found:
            A(f"**Commits touching this item** ({len(found)}) — matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer:")
            A("")
            for c in found[:8]:
                subj = git("show", "-s", "--format=%h %s", c, ok=True).strip()
                if subj:
                    A(f"- `{subj}`")
            if len(found) > 8:
                A(f"- …and {len(found)-8} more")
            A("")
        else:
            A("**Commits touching this item:** none found by slug or trailer at this pin.")
            A("")
        if files:
            code = [f for f in files if not f.startswith(".bklg/")]
            book = [f for f in files if f.startswith(".bklg/")]
            A(f"**Files changed** — {len(files)} total: **{len(code)} outside `.bklg/`**, {len(book)} backlog bookkeeping.")
            A("**This is the diff — label from it.**")
            A("")
            if code:
                for f in code[:30]:
                    A(f"- `{f}`" + ("  **[binary]**" if f in binary else ""))
                if len(code) > 30:
                    A(f"- …and {len(code)-30} more outside `.bklg/`")
            else:
                A("- *(every changed file is `.bklg/` bookkeeping — this item records no code change at the pin)*")
            A("")
            if binary:
                A(f"> git reports **{len(binary)}** of these as binary.")
                A("")
        else:
            A("**Files changed:** none resolvable at this pin — label from the card body, and use `?` freely.")
            A("")
        A("**Read the whole card if a row is unclear:**")
        A("")
        A("```bash")
        A(f"git -C {REPO} show {PIN[:8]}:{card}")
        A("```")
        A("")
        A("| item (slug) | facet | value | note |")
        A("| --- | --- | --- | --- |")
        for f in FACETS:
            pre = "?" if f == "recent_deploy" else ""
            A(f"| {slug} | {f} | {pre} |  |")
        A("")

    io.open(OUT, "w", encoding="utf-8", newline="\n").write("\n".join(o) + "\n")
    print(f"wrote {OUT}: {len(items)} items x {len(FACETS)} facets = {len(items)*len(FACETS)} rows")
    print(f"({len(items)*(len(FACETS)-1)} to fill; recent_deploy pre-filled '?')")


if __name__ == "__main__":
    main()
