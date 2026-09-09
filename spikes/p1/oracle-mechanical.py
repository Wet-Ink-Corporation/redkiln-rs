#!/usr/bin/env python3
"""P1 arm D — the deterministic oracle for the mechanical facets (METHOD-P amendment 9).

`derive-facets.py` decides `toolchain`, `change` and `binary_change` by regex over file paths plus git's
`--numstat` binary marker. A human reading the same file list agrees with that regex by construction, so a
human label on those three measures regex coverage rather than semantics. This is the independent oracle
instead: a different implementation over the same primary source, so a disagreement means the script's rule
is wrong rather than that two readers read differently.

Independence, and where it is honestly limited:
  * `toolchain` — decided by a canonical extension/manifest table written here, NOT by the script's regexes.
    Build manifests (package.json, *.csproj, *.uproject, Cargo.toml, pyproject.toml) are stronger evidence
    than an extension and are reported separately.
  * `binary_change` — decided by git's `--numstat` "-\t-" marker ALONE. The script additionally fires on any
    Unreal-looking path, so a text `.uproject` is a live disagreement and this facet is genuinely measured.
  * `change` — text/binary from git; "generated" from an explicit path list written here. git's own marker is
    the ground truth for binary, so on that component the oracle and the script agree by construction and the
    measurement is only of the Unreal shortcut. Said plainly rather than claimed as independent.

It also writes one evidence pack per item for the semantic panel, so panel members read an assembled diff
rather than each shelling out to git.

IT DOES NOT READ spikes/results/p1-arm-d.jsonl.
"""
import io, json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from importlib import import_module

_ws = import_module("build-arm-d-worksheet".replace("-", "_")) if False else None

# build-arm-d-worksheet.py has a hyphenated name; load it by path.
import importlib.util as _ilu
_spec = _ilu.spec_from_file_location(
    "ws", os.path.join(os.path.dirname(os.path.abspath(__file__)), "build-arm-d-worksheet.py")
)
ws = _ilu.module_from_spec(_spec)
_spec.loader.exec_module(ws)

OUT_ORACLE = "spikes/results/p1-arm-d-oracle.jsonl"
EVIDENCE_DIR = "spikes/results/p1-evidence"
DIFF_LINE_CAP = 400

# Canonical extension -> toolchain. Written from the toolchain vocabulary in spikes/p9/facets.toml
# (node, dotnet, python, rust, unreal), NOT copied from derive-facets.py's RULES.
EXT = {
    "node":   {".ts", ".tsx", ".js", ".jsx", ".mjs", ".cjs", ".json", ".vue", ".svelte"},
    "dotnet": {".cs", ".fs", ".vb", ".csproj", ".fsproj", ".sln", ".razor", ".cshtml"},
    "python": {".py", ".pyi", ".ipynb"},
    "rust":   {".rs"},
    "unreal": {".uasset", ".umap", ".uproject", ".uplugin", ".upluginmanifest"},
}
MANIFEST = {
    "node":   {"package.json", "pnpm-lock.yaml", "package-lock.json", "yarn.lock", "tsconfig.json"},
    "dotnet": set(),
    "python": {"pyproject.toml", "setup.py", "requirements.txt", "Pipfile"},
    "rust":   {"Cargo.toml", "Cargo.lock"},
    "unreal": set(),
}
MANIFEST_SUFFIX = {"dotnet": (".csproj", ".fsproj", ".sln"), "unreal": (".uproject", ".uplugin")}

GENERATED_DIRS = ("dist/", "build/", "node_modules/", "target/", "out/", ".astro/", "coverage/")
GENERATED_SUFFIX = (".min.js", ".min.css", ".map", "-lock.json", ".lock", ".g.cs", ".generated.ts")


def toolchains_for(paths):
    tools, why = set(), {}
    for p in paths:
        base, ext = os.path.basename(p), os.path.splitext(p)[1].lower()
        for t, exts in EXT.items():
            if ext in exts:
                tools.add(t); why.setdefault(t, []).append(p)
        for t, names in MANIFEST.items():
            if base in names:
                tools.add(t); why.setdefault(t, []).append(f"{p} (manifest)")
        for t, sfx in MANIFEST_SUFFIX.items():
            if p.endswith(sfx):
                tools.add(t); why.setdefault(t, []).append(f"{p} (manifest)")
    return sorted(tools), why


def is_generated(p):
    return p.startswith(GENERATED_DIRS) or any(d in p for d in GENERATED_DIRS) or p.endswith(GENERATED_SUFFIX)


def main():
    items = ws.read_items()
    recs, packs = [], 0
    os.makedirs(EVIDENCE_DIR, exist_ok=True)
    for n, (kind, slug, card) in enumerate(items, 1):
        shas = ws.commits_for_slug(slug)
        files, binary = ws.changed_paths(shas)
        card_text = ws.git("show", f"{ws.PIN}:{card}")
        # derive-facets.py scores three sources -- docs (paths named in the card), commits, and their union.
        # The oracle states the TRUTH for the item, so it uses the union: a card that names src/store/verify.ts
        # is evidence about the item even when no commit is discoverable by slug. Commit-only would have scored
        # 10 of 20 items as having no code at all, which is an artefact of commit discovery, not a fact.
        from_commits = [f for f in files if not f.startswith(".bklg/")]
        from_docs = [p for p in ws.path_index(card_text)
                     if not p.startswith((".bklg/", "../", ".kb/")) and "/" in p]
        code = from_commits + [p for p in from_docs if p not in from_commits]

        tools, why = toolchains_for(code)
        gen = [p for p in code if is_generated(p)]
        text = [p for p in code if p not in binary and not is_generated(p)]
        change = sorted(set((["text"] if text else []) + (["binary"] if binary else []) + (["generated"] if gen else [])))

        def rec(facet, value, evidence, rule):
            recs.append({"item": slug, "facet": facet, "value": value, "evidence": evidence[:8],
                         "rule": rule, "source": "oracle-mechanical", "n_files": len(code)})

        rec("toolchain", tools, [f"{t}: {', '.join(why.get(t, [])[:3])}" for t in tools] or ["no code files changed"],
            "O-tc-canonical-ext-and-manifest")
        rec("change", change, (binary and [f"binary: {p}" for p in sorted(binary)[:3]] or []) +
            (gen and [f"generated: {p}" for p in gen[:3]] or []) + [f"text: {p}" for p in text[:3]],
            "O-change-git-numstat-plus-generated-list")
        rec("binary_change", bool(binary), sorted(binary)[:8] or
            ["git --numstat reports no binary file among the %d commit-touched files" % len(from_commits)],
            "O-binary-git-numstat-ONLY (commit-touched files only; card-named paths carry no binary marker)")

        # evidence pack for the semantic panel
        body, shown = [], 0
        for h in shas[:8]:
            d = ws.git("show", "--format=%h %s", "--unified=3", h, ok=True)
            for line in d.splitlines():
                if line.startswith("diff --git") and "/.bklg/" in line:
                    shown = -1
                elif line.startswith("diff --git"):
                    shown = 0
                if shown == -1:
                    continue
                body.append(line); shown += 1
                if len(body) >= DIFF_LINE_CAP:
                    break
            if len(body) >= DIFF_LINE_CAP:
                body.append(f"... TRUNCATED at {DIFF_LINE_CAP} lines; {len(shas)} commits total")
                break
        pack = [f"# {n}/{len(items)} — {slug}", "",
                f"**kind:** {kind} · **commits:** {len(shas)} · **files from commits:** {len(from_commits)} · **paths named only in the card:** {len(code)-len(from_commits)}", "",
                "## Files changed (outside .bklg/)", ""]
        pack += [f"- `{p}`" + ("  [git says BINARY]" if p in binary else "")
                 + ("" if p in from_commits else "  [named in the card; no commit found]") for p in code[:60]] or ["- (none)"]
        pack += ["", "## Card body (the item as written)", "", "```", ws.body_summary(card_text, n=60), "```", "",
                 "## Diff (code only, .bklg/ bookkeeping excluded)", "", "```diff"] + body + ["```", ""]
        io.open(os.path.join(EVIDENCE_DIR, f"{n:02d}-{slug.replace('/', '__')}.md"), "w",
                encoding="utf-8", newline="\n").write("\n".join(pack))
        packs += 1

    with io.open(OUT_ORACLE, "w", encoding="utf-8", newline="\n") as f:
        for r in recs:
            f.write(json.dumps(r) + "\n")
    print(f"wrote {OUT_ORACLE}: {len(recs)} mechanical facet values over {len(items)} items")
    print(f"wrote {packs} evidence packs to {EVIDENCE_DIR}/")


if __name__ == "__main__":
    main()
