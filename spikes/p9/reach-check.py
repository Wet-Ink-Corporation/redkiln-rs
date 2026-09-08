#!/usr/bin/env python3
"""F34 rule 6 / F35 §2 — check every activity's `# reach: (kind, stage, rigor)` comment against what lattice.py selects.
Usage: python3 reach-check.py   (~30 s; zero model cost)"""
import re, glob, os, sys
sys.argv = [sys.argv[0]]; import lattice as L
from collections import defaultdict
HERE = os.path.dirname(os.path.abspath(__file__))
facets = L.load(os.path.join(HERE, "facets.toml"))["facet"]; cat = L.Catalog(os.path.join(HERE, "catalog"), facets)
packs = [L.Pack(p) for p in sorted(glob.glob(os.path.join(HERE, "packs", "*.toml")))]
reach = {}
for p in glob.glob(os.path.join(HERE, "catalog", "*.toml")):
    m = re.search(r"# reach: \(([\w-]+), ([\w-]+), ([\w*-]+)\)", open(p).read())
    if m: reach[os.path.basename(p)[:-5]] = m.groups()
sel = defaultdict(set)
for rig, tname, F in L.contexts(facets):
    Fp = dict(F); Fp["rigor"] = L.Ord("optional", L.RIGOR)
    for pk in packs:
        if not cat.pred.eval(pk.applies, Fp): continue
        for aid, info in L.Run(cat, pk, F, tname, rig).run().selected.items(): sel[aid].add((F["kind"], info["stage"], rig))
hit = 0; misses = []
for aid, (k, st, rg) in sorted(reach.items()):
    if any(kk == k and ss == st for kk, ss, _ in sel.get(aid, ())): hit += 1
    else: misses.append(f"{aid}: expected ({k}, {st}, {rg}); selected as {sorted({(kk, ss) for kk, ss, _ in sel.get(aid, ())})[:3] or 'NEVER'}")
print(f"cat:{L.digest_paths(cat.paths)} reach comments={len(reach)} hit(kind,stage)={hit} miss={len(misses)}")
print("\n".join(misses))
