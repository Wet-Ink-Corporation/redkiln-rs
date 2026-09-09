#!/usr/bin/env python3
"""P1 arm L — the UNLABELLED half (METHOD-P §1.3). Zero model cost.

Everything here is a property of the model's own draws. Nothing is compared to
the human agreement panel: §1.2's panel is two labels per item per facet written
from the item alone BEFORE unblinding (§0 rule 1), and that is Ryan's to write.
So `score-arm-d.py`'s counterpart for arm L does not exist yet and this script
deliberately stops short of it.

What is computed:
  1. per-facet Krippendorff's alpha across the 10 draws per item (5 repeats x 2
     models), nominal for `kind` and the booleans, ordinal for the four axes;
  2. per item, the fraction of draws whose derived required-artifact set
     R(stage, ctx) under the v0 `bug-standard` / `story` packs equals the modal
     one, with a Wilson 95% interval — per item, never pooled first (F27);
  3. `unknown` rate per axis (an `unknown` routes to a human: a cost, not an
     error);
  4. haiku-vs-sonnet modal disagreement per axis (reported, never a kill).

Usage:
  python spikes/p1/analyse-arm-l.py --records spikes/results/p1-arm-l.jsonl
"""
import argparse
import collections
import hashlib
import io
import itertools
import json
import math
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
P9 = os.path.join(HERE, "..", "p9")
sys.path.insert(0, P9)

ORDINALS = ["criticality", "novelty", "size", "blast_radius"]
BOOLEANS = ["market_facing", "external_dep", "has_repro"]
BANDS = ["low", "med", "high", "unknown"]


# ---------------------------------------------------------------- statistics
def wilson(k, n, z=1.96):
    """Wilson score interval. METHOD-S §0 rule 1: every repeated draw is
    reported as a rate with an interval, per item, never pooled first."""
    if n == 0:
        return (float("nan"), float("nan"))
    p = k / n
    d = 1 + z * z / n
    centre = (p + z * z / (2 * n)) / d
    half = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, centre - half), min(1.0, centre + half))


def krippendorff(units, metric):
    """Krippendorff's alpha over a dict {unit: [values, ...]}.

    Computed from the coincidence matrix directly, which handles the unequal
    numbers of draws per unit that a void or a halt produces. `None` is a
    missing value, not a category — a void is never a miss (METHOD §0), so it
    must not become a rater who said "void".
    """
    vals = sorted({v for vs in units.values() for v in vs if v is not None})
    if len(vals) < 2:
        return None  # no disagreement is possible; alpha is undefined, not 1.0
    idx = {v: i for i, v in enumerate(vals)}
    n_units = 0
    coinc = collections.defaultdict(float)
    for vs in units.values():
        obs = [v for v in vs if v is not None]
        m = len(obs)
        if m < 2:
            continue  # a unit with one observation carries no disagreement
        n_units += m
        for a, b in itertools.permutations(obs, 2):
            coinc[(idx[a], idx[b])] += 1.0 / (m - 1)
    if n_units == 0:
        return None
    n_c = collections.defaultdict(float)
    for (i, j), w in coinc.items():
        n_c[i] += w
    total = sum(n_c.values())

    def delta(i, j):
        if metric == "nominal":
            return 0.0 if i == j else 1.0
        return (i - j) ** 2  # ordinal, on the band index

    d_obs = sum(w * delta(i, j) for (i, j), w in coinc.items())
    d_exp = sum(
        n_c[i] * n_c[j] * delta(i, j)
        for i in range(len(vals))
        for j in range(len(vals))
        if i != j
    ) / max(total - 1, 1)
    if d_exp == 0:
        return None
    return 1.0 - (d_obs / d_exp)


# ---------------------------------------------------------------- the engine
def load_lattice():
    """Import lattice.py and replicate main()'s registry initialisation.

    THE TRAP: `ARTIFACTS`, `ROOTS` and `KEYED` are module globals that stay
    empty until `main()` loads `artifacts.toml` (lattice.py:329-333). A `Run`
    built without them treats every kind as non-root and non-keyed and returns a
    silently wrong R(stage, ctx) — no error, just a different answer. So they
    are set here and asserted non-empty before anything is selected.
    """
    import lattice

    facets = lattice.load(os.path.join(P9, "facets.toml"))["facet"]
    lattice.ARTIFACTS = lattice.load(os.path.join(P9, "artifacts.toml"))["artifact"]
    lattice.ROOTS = {k for k, v in lattice.ARTIFACTS.items() if v.get("source") == "human"}
    lattice.KEYED = {k for k, v in lattice.ARTIFACTS.items() if v.get("keyed_by") == "toolchain"}
    assert lattice.ROOTS, "artifacts.toml declared no human roots — the registry did not load"
    cat = lattice.Catalog(os.path.join(P9, "catalog"), facets)
    packs = {}
    for pid in ("bug-standard", "story"):
        packs[pid] = lattice.Pack(os.path.join(P9, "packs", f"{pid}.toml"))
    digests = {
        "catalog": lattice.digest_paths(cat.paths),
        "facets": lattice.digest_paths(
            [os.path.join(P9, "facets.toml"), os.path.join(P9, "artifacts.toml")]
        ),
        "packs": lattice.digest_paths([p.path for p in packs.values()]),
    }
    return lattice, facets, cat, packs, digests


def required_set(lattice, facets, cat, pack, vec):
    """R(stage, ctx) for one classification draw, as a hashable signature.

    The derived facets are held FIXED across draws: arm L varies only what the
    model emits, so anything else varying would confound the comparison. They
    are set to the neutral values the P6 lattice uses for a text/node item.
    """
    F = {
        "kind": vec["kind"],
        "cos": "standard",
        "toolchain": {"node"},
        "change": {"text"},
    }
    for ax in ORDINALS:
        F[ax] = lattice.Ord(vec[ax], facets[ax]["values"])
    for b in lattice.BOOLS:
        F[b] = bool(vec.get(b, False))
    run = lattice.Run(cat, pack, F, "node", "pack-default").run()
    sig = tuple(sorted(run.selected.keys()))
    return sig, len(run.unsat)


# ---------------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--records", default=os.path.join(HERE, "..", "results", "p1-arm-l.jsonl"))
    ap.add_argument("--out", default=None)
    # METHOD-P §1.4's remediation is "anchors rewritten first and re-run once;
    # if still < 0.6, human-supplied". Pointing --compare-to at the v1 records
    # produces the per-axis before/after that sentence is decided on.
    ap.add_argument("--compare-to", default=None,
                    help="a prior arm-L results file (v1) to report alpha deltas against")
    a = ap.parse_args()

    rows = [json.loads(l) for l in io.open(a.records, encoding="utf-8") if l.strip()]
    ok = [r for r in rows if r.get("outcome") == "ok" and r.get("vector")]
    voids = [r for r in rows if r.get("outcome") != "ok"]

    out = io.open(a.out, "w", encoding="utf-8", newline="\n") if a.out else sys.stdout
    w = out.write

    card = {r.get("card_sha256") for r in rows}
    schema = {r.get("schema_sha256") for r in rows}
    corpus = {r.get("corpus_sha") for r in rows}
    versions = {(r.get("step") or {}).get("claude_version") for r in rows}
    lattice, facets, cat, packs, dig = load_lattice()
    tag = f"cat:{dig['catalog']} fac+art:{dig['facets']} pk:{dig['packs']}"

    w(f"P1 arm L — unlabelled half (METHOD-P §1.3)\n")
    w(f"{tag}\n")
    w(f"records={len(rows)} valid={len(ok)} void={len(voids)}  "
      f"card={sorted(card)[0][:12]} schema={sorted(schema)[0][:12]}\n")
    w(f"corpus={sorted(corpus)[0]}  claude={sorted(v for v in versions if v)}\n")
    if len(card) != 1 or len(schema) != 1:
        w("!! CARD OR SCHEMA CHANGED MID-RUN — the set is not one experiment\n")
    if voids:
        w(f"void reasons: {collections.Counter(r.get('void_reason') for r in voids)}\n")
    w("\n")

    by_item = collections.defaultdict(list)
    for r in ok:
        by_item[r["item"]].append(r)
    items = sorted(by_item)
    w(f"items={len(items)}  draws/item={collections.Counter(len(v) for v in by_item.values())}\n\n")

    # ---- 1. per-facet Krippendorff's alpha -----------------------------
    w("### 1. Per-facet Krippendorff's alpha across the 10 draws per item [measured]\n\n")
    w("| facet | metric | alpha | kill (<0.6 -> human-supplied) |\n| --- | --- | --- | --- |\n")
    alphas = {}
    for f in ORDINALS + BOOLEANS + ["kind"]:
        if f in ORDINALS:
            units = {i: [BANDS.index(r["vector"][f]) for r in rs if r["vector"].get(f) in BANDS]
                     for i, rs in by_item.items()}
            metric = "ordinal"
        else:
            units = {i: [str(r["vector"].get(f)) for r in rs] for i, rs in by_item.items()}
            metric = "nominal"
        al = krippendorff(units, metric)
        alphas[f] = al
        verdict = "n/a" if al is None else ("**KILL fires**" if al < 0.6 else "does not fire")
        w(f"| `{f}` | {metric} | {'n/a' if al is None else f'{al:.3f}'} | {verdict} |\n")
    w("\nAn alpha of `n/a` means every draw agreed, so no disagreement was available to\n"
      "measure — alpha is undefined there, not 1.0.\n\n")

    # ---- 2. modal R(stage, ctx) per item -------------------------------
    w("### 2. Fraction of draws whose derived R(stage, ctx) equals the modal one [measured]\n\n")
    w("Per item, with a Wilson 95% interval; never pooled first (F27, METHOD-S §0 rule 1).\n")
    w("Derived facets are held fixed at a text/node context so that only what the model\n")
    w("emits varies between draws.\n\n")
    for pid, pack in packs.items():
        w(f"\n**pack `{pid}`**\n\n")
        w("| item | n | modal | rate | Wilson 95% |\n| --- | --- | --- | --- | --- |\n")
        agree_items = 0
        counted = 0
        for i in items:
            sigs = []
            for r in by_item[i]:
                try:
                    sig, _ = required_set(lattice, facets, cat, pack, r["vector"])
                    sigs.append(sig)
                except Exception as e:  # a context the engine cannot select in
                    sigs.append(("<error>", str(e)[:40]))
            if not sigs:
                continue
            counted += 1
            mode, k = collections.Counter(sigs).most_common(1)[0]
            n = len(sigs)
            lo, hi = wilson(k, n)
            if k == n:
                agree_items += 1
            short = i.split("/")[-1][:46]
            w(f"| `{short}` | {n} | {len(mode) if isinstance(mode, tuple) else '?'} act | "
              f"{k}/{n} = {k/n:.2f} | [{lo:.2f}, {hi:.2f}] |\n")
        if counted:
            frac = agree_items / counted
            v = "**KILL fires**" if frac < 0.80 else "does not fire"
            w(f"\nItems with an identical modal R across every draw: **{agree_items}/{counted} "
              f"= {frac:.0%}**. Kill at <80%: {v}.\n")

    # ---- 3. unknown rate per axis --------------------------------------
    w("\n### 3. `unknown` rate per axis [measured]\n\n")
    w("An `unknown` routes to a human at triage — a cost, not an error. But at selection\n")
    w("it is treated as HIGH by position (F34 B14, F35 §1.6), so the rate is also a\n")
    w("statement about how often the anchors send an item up rather than to a person.\n\n")
    w("| axis | unknown / draws | rate |\n| --- | --- | --- |\n")
    for f in ORDINALS:
        n = sum(1 for r in ok if r["vector"].get(f) == "unknown")
        w(f"| `{f}` | {n}/{len(ok)} | {n/len(ok):.1%} |\n")

    # ---- 4. haiku vs sonnet modal disagreement -------------------------
    w("\n### 4. haiku-vs-sonnet modal disagreement per axis [measured]\n\n")
    w("Reported, never a kill (METHOD-P §1.4): the cheaper model that meets alpha is the\n")
    w("triage default.\n\n")
    w("| axis | items where the modal value differs | rate |\n| --- | --- | --- |\n")
    for f in ORDINALS + BOOLEANS + ["kind"]:
        diff = 0
        n = 0
        for i in items:
            mods = {}
            for m in ("haiku", "sonnet"):
                vs = [str(r["vector"].get(f)) for r in by_item[i] if r["model"] == m]
                if vs:
                    mods[m] = collections.Counter(vs).most_common(1)[0][0]
            if len(mods) == 2:
                n += 1
                if mods["haiku"] != mods["sonnet"]:
                    diff += 1
        flag = " **>20%, reported**" if n and diff / n > 0.20 else ""
        w(f"| `{f}` | {diff}/{n} | {0 if not n else diff/n:.0%}{flag} |\n")

    # ---- 5. the METHOD-P 1.4 re-run comparison -------------------------
    if a.compare_to:
        prior = [json.loads(l) for l in io.open(a.compare_to, encoding="utf-8") if l.strip()]
        pok = [r for r in prior if r.get("outcome") == "ok" and r.get("vector")]
        pby = collections.defaultdict(list)
        for r in pok:
            pby[r["item"]].append(r)
        pcard = sorted({r.get("card_sha256") for r in prior})[0]
        # Only the axes that fired were rewritten, so a moved alpha on an
        # untouched axis is drift between two draws of the same instrument, not
        # remediation, and must not be credited as one.
        REWRITTEN = {"blast_radius", "external_dep"}
        w("\n### 5. METHOD-P 1.4 re-run: anchors rewritten, alpha before and after [measured]\n\n")
        w(f"Prior instrument: card `{pcard[:12]}`, {len(pok)} valid draws over {len(pby)} items.\n")
        w(f"This instrument: card `{sorted(card)[0][:12]}`, {len(ok)} valid draws over {len(by_item)} items.\n\n")
        w("An axis below 0.6 has \"its anchors rewritten first and re-run once; if still\n")
        w("< 0.6, human-supplied\". Axes not in the rewrite are shown for drift only.\n\n")
        w("| facet | rewritten? | alpha v1 | alpha v2 | delta | verdict |\n")
        w("| --- | --- | --- | --- | --- | --- |\n")
        for f in ORDINALS + BOOLEANS + ["kind"]:
            if f in ORDINALS:
                u1 = {i: [BANDS.index(r["vector"][f]) for r in rs if r["vector"].get(f) in BANDS]
                      for i, rs in pby.items()}
                u2 = {i: [BANDS.index(r["vector"][f]) for r in rs if r["vector"].get(f) in BANDS]
                      for i, rs in by_item.items()}
                metric = "ordinal"
            else:
                u1 = {i: [str(r["vector"].get(f)) for r in rs] for i, rs in pby.items()}
                u2 = {i: [str(r["vector"].get(f)) for r in rs] for i, rs in by_item.items()}
                metric = "nominal"
            a1 = krippendorff(u1, metric)
            a2 = krippendorff(u2, metric)
            rw = "**yes**" if f in REWRITTEN else "no"
            if a1 is None or a2 is None:
                w(f"| `{f}` | {rw} | {a1} | {a2} | - | undefined |\n")
                continue
            d = a2 - a1
            if f in REWRITTEN:
                verdict = ("**clears 0.6 - remediation worked**" if a2 >= 0.6
                           else "**still < 0.6 -> HUMAN-SUPPLIED at the triage gate**")
            else:
                verdict = ("unchanged" if abs(d) < 0.05
                           else f"**drift {d:+.3f} on an untouched anchor - report, do not credit**")
            w(f"| `{f}` | {rw} | {a1:.3f} | {a2:.3f} | {d:+.3f} | {verdict} |\n")
        w("\n| axis | `unknown` rate v1 | v2 | delta |\n| --- | --- | --- | --- |\n")
        for f in ORDINALS:
            r1 = sum(1 for r in pok if r["vector"].get(f) == "unknown") / max(len(pok), 1)
            r2 = sum(1 for r in ok if r["vector"].get(f) == "unknown") / max(len(ok), 1)
            w(f"| `{f}` | {r1:.1%} | {r2:.1%} | {r2 - r1:+.1%} |\n")

    w("\n### Not computed here\n\n")
    w("Anything requiring the human agreement panel (METHOD-P §1.2/§0 rule 1): the panel is\n")
    w("written from the item alone before unblinding, and it is Ryan's. The skeleton is\n")
    w("`spikes/results/p1-arm-l-labels-RYAN-TODO.md`. Arm D's precision/recall against the\n")
    w("panel — the criterion that decides whether a facet leaves the model — likewise waits.\n")
    if a.out:
        out.close()
        print(f"wrote {a.out}")


if __name__ == "__main__":
    main()
