#!/usr/bin/env python3
"""P1 arm D — facet derivability (METHOD-P §1.2 arm D, PC7/PC8). Zero model cost.

For each item in spikes/plans/p1-items.txt, compute the DERIVED facets from the item's affected paths and diff
against the corpus repo at the pinned SHA, and write one JSONL record per (item, facet) with the evidence
(paths / diff lines) that produced the value. The human panel labels the same items on the same facets
BEFORE this output is unblinded (METHOD-P §0 rule 1) — see p1-arm-d-labels-BLANK.md.

Facets (facets.toml names): toolchain (set), change (set: text/binary/generated), binary_change (bool),
auth, data_migration, ui, flag_exists (bools), recent_deploy (bool-or-null). `dynamism` is not a registered
facet (F35 I8) -- and NEITHER IS `binary_change` (#12): METHOD-P 1.2 pre-registers it as a derived facet and
the label template asks for it, but facets.toml has no [facet.binary_change]. It is still measured (1.2
binds) and F36 reports that it can feed no `require` row until the registry gains it.

Affected paths come from THREE sources as of 2026-09-09 (DECISIONS-2026-09-09 #6), all recorded separately:
(a) `docs`   — backtick-quoted paths in the item's implementation-report.md / report.md / spec.md
               (the S-MIG lift_paths locator);
(b) `commits`— `git log --grep=<slug-leaf>` commits' --numstat at or before the pin. MEASURED to return 0-2
               commits for real items, so this source is near-empty; it is KEPT so F36 can show why (b) alone
               could not answer METHOD-P 1.4;
(c) `links`  — the commit SHAs recorded in the card's own frontmatter `links.commits`. This is the item's
               declared provenance rather than a text search, and is the primary commit source.
`union` is (a) | (b) | (c). Precision/recall is reported per source and for the union.

`recent_deploy` (facets.toml, added by DECISIONS-2026-09-08 #9; anchor "a deploy touching the item's surface
within the last 24 h") is derived from the registered feed `toolchain:deploy_log`. THAT FEED DOES NOT EXIST IN
THE FROZEN CORPUS, so the facet is emitted with value `null` and rule `R-recent-deploy-feed-absent`. That is
the measurement, not a bug: the facet is underivable at this pin and F36 reports it as such (#12).

Every record carries: corpus sha, item slug, facet, value, source, evidence[:8], rule id.
Usage: python3 spikes/p1/derive-facets.py --redkiln D:/repos/redkiln --sha 47faec9f... --items spikes/plans/p1-items.txt --out spikes/results/p1-arm-d.jsonl
NOT YET RUN: corpus repo not mounted when written.
"""
import argparse, json, re, subprocess, os, datetime

PATH_RE = re.compile(r"`([^`\n]+?\.[A-Za-z0-9]{1,6}(?::\d+(?:-\d+)?)?)`")
RULES = {
    "toolchain": [("node", re.compile(r"(^|/)(package\.json|package-lock\.json|pnpm-lock\.yaml|tsconfig.*\.json)$|\.(ts|tsx|js|jsx|mjs|cjs)$")),
                  ("dotnet", re.compile(r"\.(cs|csproj|sln|fsproj)$")), ("python", re.compile(r"\.(py|pyi)$|(^|/)(pyproject\.toml|requirements.*\.txt)$")),
                  ("rust", re.compile(r"\.rs$|(^|/)Cargo\.(toml|lock)$")), ("unreal", re.compile(r"\.(uasset|umap|uproject|uplugin)$|(^|/)(Content|Config)/.*\.ini$"))],
    "generated": re.compile(r"(^|/)(dist|build|out|node_modules|target|bin|obj)/|\.(lock|min\.js|snap|generated\.\w+)$|(^|/)(package-lock\.json|pnpm-lock\.yaml|Cargo\.lock)$"),
    "auth": re.compile(r"auth|login|logout|session|token|jwt|oauth|permission|role|acl|password|credential", re.I),
    "data_migration": re.compile(r"(^|/)migrations?/|\.sql$|schema\.(prisma|graphql|sql|rb)$|(^|/)(prisma|drizzle|knex|alembic|flyway|liquibase)/|ALTER TABLE|CREATE TABLE|DROP COLUMN", re.I),
    "ui": re.compile(r"\.(tsx|jsx|vue|svelte|css|scss|html|umap)$|(^|/)(components?|pages|views|ui|screens|widgets)/", re.I),
    "flag_exists": re.compile(r"feature[_ -]?flag|featureFlag|isEnabled\(|LaunchDarkly|unleash|flags?\.(toml|json|yaml)|FEATURE_[A-Z_]+", re.I),
}

def git(repo, *a, ok=False):
    r = subprocess.run(["git", *a], cwd=repo, capture_output=True)
    if r.returncode and not ok: raise SystemExit(r.stderr.decode(errors="replace"))
    return r.stdout.decode("utf-8", "replace")

def paths_from_docs(repo, sha, slug):
    out = set()
    for name in ("implementation-report.md", "report.md", "spec.md"):
        t = git(repo, "show", f"{sha}:.bklg/{slug}/{name}", ok=True)
        for m in PATH_RE.finditer(t): out.add(m.group(1).split(":")[0])
    return out

LINKS_RE = re.compile(r"^links:\s*$(.*?)^[A-Za-z_]", re.M | re.S)
SHA_RE = re.compile(r"^\s+-\s+([0-9a-f]{7,40})\s*$", re.M)


def commits_from_links(repo, sha, card):
    """#6: the commit SHAs the card itself declares under frontmatter `links.commits`."""
    body = git(repo, "show", f"{sha}:{card}", ok=True)
    m = LINKS_RE.search(body)
    if not m:
        return []
    blk = m.group(1)
    cut = re.search(r"^\s+(pr|kb):", blk[blk.find("commits:"):], re.M) if "commits:" in blk else None
    seg = blk[blk.find("commits:"):] if "commits:" in blk else ""
    if cut:
        seg = seg[: cut.start()]
    out = []
    for h in SHA_RE.findall(seg):
        # keep only SHAs that resolve to a commit in this repo
        if git(repo, "rev-parse", "--verify", f"{h}^{{commit}}", ok=True).strip():
            out.append(h)
    return out


def paths_from_shas(repo, shas):
    """#6: shared extractor -- numstat + unified diff for an explicit commit list."""
    files, binary, diff = set(), set(), []
    for h in shas[:50]:
        for line in git(repo, "show", "--numstat", "--format=", h, ok=True).splitlines():
            parts = line.split("\t")
            if len(parts) == 3:
                files.add(parts[2])
                if parts[0] == "-" and parts[1] == "-":
                    binary.add(parts[2])
        diff += [l for l in git(repo, "show", "--format=", "-U0", h, ok=True).splitlines()
                 if l.startswith(("+", "-")) and not l.startswith(("+++", "---"))][:2000]
    return files, binary, diff, shas[:50]


def paths_from_commits(repo, sha, slug):
    log = git(repo, "log", "--format=%H", f"--grep={slug.split('/')[-1]}", sha, ok=True).split()
    files, binary, diff = set(), set(), []
    for h in log[:50]:
        for line in git(repo, "show", "--numstat", "--format=", h, ok=True).splitlines():
            parts = line.split("\t")
            if len(parts) == 3:
                files.add(parts[2])
                if parts[0] == "-" and parts[1] == "-": binary.add(parts[2])
        diff += [l for l in git(repo, "show", "--format=", "-U0", h, ok=True).splitlines() if l.startswith(("+", "-")) and not l.startswith(("+++", "---"))][:2000]
    return files, binary, diff, log[:50]

def derive(paths, binary_paths, diff_lines):
    rec = {}
    tools = {t for p in paths for t, rx in RULES["toolchain"] if rx.search(p)}
    rec["toolchain"] = (sorted(tools), [p for p in sorted(paths) if any(rx.search(p) for _, rx in RULES["toolchain"])][:8], "R-tc-ext")
    change = set()
    if any(not RULES["generated"].search(p) and p not in binary_paths for p in paths): change.add("text")
    _unreal = dict(RULES["toolchain"])["unreal"]  # #12: was RULES["toolchain"][4][1], a positional index
    if binary_paths or any(_unreal.search(p) for p in paths): change.add("binary")
    if any(RULES["generated"].search(p) for p in paths): change.add("generated")
    rec["change"] = (sorted(change), sorted(binary_paths)[:8], "R-change-numstat+ext")
    rec["binary_change"] = ("binary" in change, sorted(binary_paths)[:8], "R-binary")
    for f in ("auth", "data_migration", "ui"):
        hits = [p for p in sorted(paths) if RULES[f].search(p)]
        dhits = [l for l in diff_lines if RULES[f].search(l)][:8] if f != "ui" else []
        rec[f] = (bool(hits or dhits), (hits + dhits)[:8], f"R-{f}-path{'+diff' if f != 'ui' else ''}")
    fh = [l for l in diff_lines if RULES["flag_exists"].search(l)][:8] + [p for p in sorted(paths) if RULES["flag_exists"].search(p)]
    rec["flag_exists"] = (bool(fh), fh[:8], "R-flag-diff+path")
    # #12: registered facet, registered feed `toolchain:deploy_log` -- absent from the frozen corpus.
    # Emitted as null so the record set is complete and the gap is visible, never as False.
    rec["recent_deploy"] = (None, ["toolchain:deploy_log not present at the corpus pin"],
                            "R-recent-deploy-feed-absent")
    return rec

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True)
    ap.add_argument("--items", required=True); ap.add_argument("--out", required=True); a = ap.parse_args()
    items = [l.split("\t") for l in open(a.items, encoding="utf-8") if l.strip() and not l.startswith("#")]
    with open(a.out, "w", encoding="utf-8", newline="\n") as out:
        for kind, slug, card, h in items:
            pd = paths_from_docs(a.redkiln, a.sha, slug); pc, binary, diff, commits = paths_from_commits(a.redkiln, a.sha, slug)
            # #6: the card's own declared provenance, as a separate recorded source.
            lsha = commits_from_links(a.redkiln, a.sha, card)
            pl, lbin, ldiff, lcommits = paths_from_shas(a.redkiln, lsha)
            ubin, udiff = binary | lbin, diff + ldiff
            for source, paths, bp, dl in (("docs", pd, set(), []), ("commits", pc, binary, diff),
                                          ("links", pl, lbin, ldiff), ("union", pd | pc | pl, ubin, udiff)):
                for facet, (value, evidence, rule) in derive(paths, bp, dl).items():
                    out.write(json.dumps({"corpus_sha": a.sha, "item": slug, "kind": kind, "facet": facet, "value": value, "source": source,
                                          "n_paths": len(paths), "commits": len(commits), "link_commits": len(lcommits),
                                          "evidence": evidence, "rule": rule,
                                          "at": datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None).isoformat() + "Z"}) + "\n")
    print(f"wrote {a.out}")

if __name__ == "__main__": main()
