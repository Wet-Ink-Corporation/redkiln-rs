#!/usr/bin/env python3
"""Build the P2 "should re-run" labelling worksheet.

METHOD-P §2.2 says the labels are written FROM THE DIFF ALONE, before the engine runs. The generated
blank (`spikes/results/p2-labels-RYAN-TODO.md`) has 41 rows and a git command in a comment, which is the
same defect `p1-arm-d-labels-RYAN-TODO.md` had: it asks for 41 judgements and gives the labeller nothing
to judge from. This script assembles exactly that raw material inline and nothing else.

IT DOES NOT READ spikes/results/p2-reconstruct.jsonl's `fine` or `coarse` scopes, and it does not
reproduce `reconstruct-derived-from.py`'s COARSE table. Those are the engine's answer to the same
question the labeller is being asked; showing them would make the comparison vacuous. The same rule the
arm D worksheet states applies here — if the script's own scope reconstruction is wrong, the labeller
must be able to disagree with it, which requires seeing the primary source rather than the script's
reading of it.

Blindness: P2 is NOT affected by METHOD-P amendment 8. Its engine phase requires only that the labels
carry a timestamp, and its verdicts do not exist yet, so labelling now is blind on the original terms.
There is therefore no `blind_attestation:` key here and there must not be one — it would have no meaning
and no consumer.

Output is the label file itself, pre-populated with context, so there is no copy step: fill the value
cells in place and run the engine phase against it.
"""
import io, json, os, re, subprocess, sys

REPO = "D:/repos/redkiln"
PIN = "47faec9f46b8d983c2c75f0794164e68415ae706"
CASES = "spikes/results/p2-cases.jsonl"
ITEMS = "spikes/plans/p2-items.txt"
OUT = "spikes/results/p2-labels-ryan.md"

DIFF_LINE_CAP = 400          # the arm D evidence packs use the same cap
DOWNSTREAM_SUMMARY_LINES = 18


def git(*a, ok=False):
    r = subprocess.run(["git", "-C", REPO] + list(a), capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode and not ok:
        return ""
    return r.stdout


def read_items():
    """p2-items.txt is `slug \\t card \\t artifacts \\t sha256`.

    NOTE THE COLUMN ORDER. It is NOT p1-items.txt's `kind \\t slug \\t card \\t sha256`, so
    `build-arm-d-worksheet.read_items()` cannot be reused here: it would read `slug` as the kind and
    `card` as the slug, silently. Returns {slug: card}.
    """
    cards = {}
    for line in io.open(ITEMS, encoding="utf-8"):
        line = line.rstrip("\n")
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) < 2:
            continue
        cards[parts[0]] = parts[1]
    return cards


def read_cases():
    out = []
    for line in io.open(CASES, encoding="utf-8"):
        line = line.strip()
        if not line:
            continue
        c = json.loads(line)
        if c.get("kind") == "perturb":
            out.append(c)
    return out


def commits_for_slug(slug):
    """The item's real commits, found the way redkiln links them: the slug leaf in the subject
    (`fix(<leaf>): ...`) and the Story:/Fix: trailers. Ancestors of the pin only.

    Measured on the arm D corpus: `links.commits` in the card frontmatter resolves on only 4 of 20
    items (16 cards record `commits: []`), while this search resolves 14 of 20. Secondary context here,
    not the primary evidence — the primary evidence is the upstream diff.
    """
    leaf = slug.rstrip("/").split("/")[-1]
    seen = []
    for pat in (leaf, f"Story: {leaf}", f"Fix: {leaf}"):
        out = git("log", PIN, "--format=%H", "--fixed-strings", f"--grep={pat}", ok=True)
        for h in out.split():
            if h not in seen:
                seen.append(h)
    return seen


def changed_paths(shas):
    """Union of files touched by those commits. git's own --numstat marks binaries as '-\\t-'."""
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


def body_summary(text, n=DOWNSTREAM_SUMMARY_LINES):
    body = re.sub(r"^---\n.*?\n---\n", "", text, flags=re.S)
    lines = body.split("\n")
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


def main():
    if not os.path.isdir(REPO):
        sys.exit(f"corpus repo not found at {REPO}")
    if not git("rev-parse", PIN, ok=True).strip():
        sys.exit(f"corpus pin {PIN[:8]} not reachable in {REPO}")
    cards = read_items()
    if not cards:
        sys.exit("no items parsed from " + ITEMS)
    cases = read_cases()
    if not cases:
        sys.exit("no perturbation cases parsed from " + CASES)

    o = []
    A = o.append
    A("# P2 — human \"should re-run\" labels (Ryan). Worksheet-populated 2026-09-09.")
    A("#")
    A("# METHOD-P §0 rule 1 / §2.2: for every downstream artifact in every perturbation case, label FROM")
    A("# THE DIFF ALONE. Each case's diff is inline below, so there is no lookup step and no shell.")
    A("#")
    A("# THE QUESTION, once per row: the upstream artifact named in the case was replaced by its previous")
    A("# real git revision. Reading that change and nothing else — WOULD IT HAVE REQUIRED THE NAMED")
    A("# DOWNSTREAM ARTIFACT TO BE RE-AUTHORED? Answer `yes` or `no` in the fifth cell and give a one-line")
    A("# reason in the sixth. This is the ground truth the engine's `Stale` verdict is scored against.")
    A("#")
    A("# The engine's answer was NOT read when building this file. `p2-reconstruct.jsonl`'s reconstructed")
    A("# `fine` and `coarse` scopes, and reconstruct-derived-from.py's COARSE table, are deliberately")
    A("# absent: they answer the same question and would prejudice the label. If you think the engine's")
    A("# scope will disagree with you, that is the measurement, not a problem.")
    A("#")
    A("# Blindness: P2 is not affected by METHOD-P amendment 8 — its engine phase requires only that these")
    A("# labels be timestamped, and its verdicts do not exist yet, so labelling now is blind on the")
    A("# original terms. There is no `blind_attestation` key and there must not be one.")
    A("#")
    A("# WHEN DONE: set labelled_at to the UTC time you finished. It MUST end in Z — the guard compares")
    A("# timestamps as strings, so a local-offset stamp mis-compares silently. Produce it with a command:")
    A("#   date -u +%Y-%m-%dT%H:%M:%SZ")
    A("# then run the engine phase, which refuses until the stamp is real:")
    A("#   python spikes/p2/reconstruct-derived-from.py --redkiln D:/repos/redkiln \\")
    A(f"#     --sha {PIN} \\")
    A("#     --items spikes/plans/p2-items.txt --phase engine \\")
    A("#     --cases spikes/results/p2-cases.jsonl \\")
    A("#     --labels spikes/results/p2-labels-ryan.md \\")
    A("#     --out spikes/results/p2-engine.jsonl")
    A("# Pass --cases explicitly (decision #17); it does not default correctly from --out.")
    A("#")
    A("# Advisory cases A01–A06 are labelled AFTER the engine, in a separate file (METHOD-P §2.3), and")
    A("# decision #15 — the `human_authored` remap — changes what that advisory fraction measures. Confirm")
    A("# or reverse #15 before the advisory half is scored; it does not affect the rows below.")
    A("")
    A("labelled_at: FILL-ME-ISO8601Z")
    A("labeller: FILL-ME")
    A(f"corpus_sha: {PIN}")
    A(f"cases_file: {CASES}")
    A("")

    rows_total = 0
    for n, c in enumerate(cases, 1):
        item, upstream, rev = c["item"], c["upstream"], c["replaced_with_rev"]
        card = cards.get(item, "")
        path = f".bklg/{item}/{upstream}"
        A("---")
        A("")
        A(f"## {c['case']} — {n}/{len(cases)} — `{item}`")
        A("")
        bits = [f"**upstream replaced:** `{upstream}`", f"**previous revision:** `{rev[:12]}`"]
        if card:
            bits.insert(0, f"**card:** `{card}`")
        if c.get("prev_digest") and c.get("cur_digest"):
            bits.append(f"**digests:** `{c['prev_digest']}` → `{c['cur_digest']}`")
        A(" · ".join(bits))
        A("")
        A(f"**Downstream artifacts to label ({len(c['downstream'])}):** "
          + ", ".join(f"`{d}`" for d in c["downstream"]))
        A("")

        # ---- the primary evidence: the upstream diff, inline ----
        diff = git("diff", rev, PIN, "--", path, ok=True)
        A(f"**The upstream change — this is what you label from.** `git -C {REPO} diff {rev[:12]} "
          f"{PIN[:8]} -- {path}`")
        A("")
        if not diff.strip():
            A("> **NO DIFF RESOLVED at this pin.** The two revisions of this artifact are identical, or")
            A("> the path did not exist at one of them. Label `no` only if you are satisfied that is a")
            A("> real absence of change rather than a lookup failure — and say which in the reason.")
        else:
            lines = diff.split("\n")
            clipped = len(lines) > DIFF_LINE_CAP
            A("```diff")
            for l in lines[:DIFF_LINE_CAP]:
                A(l)
            A("```")
            if clipped:
                A(f"> Clipped at {DIFF_LINE_CAP} lines of {len(lines)}. Run the command above for the rest.")
        A("")

        # ---- secondary context: what the item actually did in code ----
        found = commits_for_slug(item)
        files, binary = changed_paths(found)
        code = [f for f in files if not f.startswith(".bklg/")]
        A("<details><summary>secondary context — what this item did in code (NOT the thing you are "
          "labelling)</summary>")
        A("")
        if found:
            A(f"Commits matched by the slug leaf in the subject and by `Story:`/`Fix:` trailer "
              f"({len(found)}):")
            A("")
            for h in found[:8]:
                s = git("show", "-s", "--format=%h %s", h, ok=True).strip()
                if s:
                    A(f"- `{s}`")
            A("")
            if code:
                A(f"Files changed outside `.bklg/` ({len(code)} of {len(files)} total):")
                A("")
                for f in code[:30]:
                    A(f"- `{f}`" + ("  **[binary]**" if f in binary else ""))
            else:
                A("- *(every changed file is `.bklg/` bookkeeping — this item records no code change "
                  "at the pin)*")
        else:
            A("No commits resolved by slug leaf or trailer at this pin. That is a lookup limit, not a")
            A("statement about the item.")
        A("")
        A("</details>")
        A("")

        # ---- the downstream artifacts themselves, so the labeller can see what depends on what ----
        for d in c["downstream"]:
            text = git("show", f"{PIN}:.bklg/{item}/{d}", ok=True)
            A(f"<details><summary>downstream: <code>{d}</code> — first lines at the pin</summary>")
            A("")
            if text.strip():
                A("```markdown")
                A(body_summary(text))
                A("```")
            else:
                A(f"*(not readable at the pin: `.bklg/{item}/{d}`)*")
            A("")
            A("</details>")
            A("")

        # ---- this case's answer rows ----
        A("| case | item | upstream replaced | downstream artifact | should re-run (yes/no) | why (one line) |")
        A("| --- | --- | --- | --- | --- | --- |")
        for d in c["downstream"]:
            A(f"| {c['case']} | {item} | {upstream} | {d} |  |  |")
            rows_total += 1
        A("")

    io.open(OUT, "w", encoding="utf-8", newline="\n").write("\n".join(o) + "\n")
    print(f"wrote {OUT}: {len(cases)} perturbation cases, {rows_total} (case, downstream) rows to fill")
    if rows_total != 41:
        print(f"WARNING: expected 41 rows (12 cases x their downstream sets), got {rows_total} — "
              f"check {CASES} against METHOD-P §2.2 before labelling")


if __name__ == "__main__":
    main()
