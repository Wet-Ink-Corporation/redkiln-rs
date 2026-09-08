#!/usr/bin/env python3
"""P1 arm D — facet derivability (METHOD-P §1.2 arm D, PC7/PC8). Zero model cost.

For each item in spikes/plans/p1-items.txt, compute the DERIVED facets from the item's affected paths and diff
against the corpus repo at the pinned SHA, and write one JSONL record per (item, facet) with the evidence
(paths / diff lines) that produced the value. The human panel labels the same items on the same facets
BEFORE this output is unblinded (METHOD-P §0 rule 1) — see p1-arm-d-labels-BLANK.md.

Facets (facets.toml names): toolchain (set), change (set: text/binary/generated), binary_change (bool),
auth, data_migration, ui, flag_exists (bools). `dynamism` is not a registered facet (F35 I8).

Affected paths come from two sources, both recorded: (a) backtick-quoted paths in the item's
implementation-report.md / report.md / spec.md (the S-MIG lift_paths locator); (b) `git log --grep=<slug>`
commits' --name-status at or before the pin. Precision/recall is reported per source and for their union.

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
    if binary_paths or any(RULES["toolchain"][4][1].search(p) for p in paths): change.add("binary")
    if any(RULES["generated"].search(p) for p in paths): change.add("generated")
    rec["change"] = (sorted(change), sorted(binary_paths)[:8], "R-change-numstat+ext")
    rec["binary_change"] = ("binary" in change, sorted(binary_paths)[:8], "R-binary")
    for f in ("auth", "data_migration", "ui"):
        hits = [p for p in sorted(paths) if RULES[f].search(p)]
        dhits = [l for l in diff_lines if RULES[f].search(l)][:8] if f != "ui" else []
        rec[f] = (bool(hits or dhits), (hits + dhits)[:8], f"R-{f}-path{'+diff' if f != 'ui' else ''}")
    fh = [l for l in diff_lines if RULES["flag_exists"].search(l)][:8] + [p for p in sorted(paths) if RULES["flag_exists"].search(p)]
    rec["flag_exists"] = (bool(fh), fh[:8], "R-flag-diff+path")
    return rec

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True)
    ap.add_argument("--items", required=True); ap.add_argument("--out", required=True); a = ap.parse_args()
    items = [l.split("\t") for l in open(a.items) if l.strip() and not l.startswith("#")]
    with open(a.out, "w") as out:
        for kind, slug, card, h in items:
            pd = paths_from_docs(a.redkiln, a.sha, slug); pc, binary, diff, commits = paths_from_commits(a.redkiln, a.sha, slug)
            for source, paths, bp, dl in (("docs", pd, set(), []), ("commits", pc, binary, diff), ("union", pd | pc, binary, diff)):
                for facet, (value, evidence, rule) in derive(paths, bp, dl).items():
                    out.write(json.dumps({"corpus_sha": a.sha, "item": slug, "kind": kind, "facet": facet, "value": value, "source": source,
                                          "n_paths": len(paths), "commits": len(commits), "evidence": evidence, "rule": rule,
                                          "at": datetime.datetime.utcnow().isoformat() + "Z"}) + "\n")
    print(f"wrote {a.out}")

if __name__ == "__main__": main()
