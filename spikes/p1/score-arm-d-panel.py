#!/usr/bin/env python3
"""P1 arm D — final scoring under METHOD-P amendment 9 (panel-adjudicated oracle).

Assembles the truth set from three provenances and scores derive-facets.py against it, per facet per source.

  MECHANICAL (toolchain, change, binary_change)  <- spikes/p1/oracle-mechanical.py, deterministic, no human
  SEMANTIC, uncontested                          <- unanimous 3-seat panel, accepted with its evidence
  SEMANTIC, contested                            <- Ryan's blind adjudication (32 cells)

`unknown` is excluded from scoring, never counted against the script (METHOD-P §1.4).
Report this as PANEL-ADJUDICATED, never as human-panel (amendment 9).

Standing caution recorded with the result: the uncontested cells rest on unanimous 3-seat agreement, and
"Nine Judges, Two Effective Votes" measured a 9-judge panel supplying ~2.18 effective independent votes with
accuracy 8-22 points short of independent-vote predictions. Unanimity here is a weaker correctness signal than
its arithmetic suggests; the contested cells carry a human and the uncontested ones do not.

Corrected 2026-09-11 (audit PIA-077, ERRATA E-0089-2). Two defects in the 2026-09-09 scoring run:
  1. `p1-arm-d-panel-accepted.jsonl` keys four cells of one item under the slug
     `.../kb-harvest-and-closeout/resolve-open-questions` (plural); the engine, the oracle and the contested file
     all key that item `.../resolve-open-question` (singular). The four cells (auth, data_migration, flag_exists,
     ui — all `false`) therefore had no engine record and were skipped without a word by the `continue` below.
     The result files are records and are not edited; the known mis-key is normalised on read (PANEL_KEY_FIX).
  2. A truth cell with no engine record was silently dropped. The scorer now FAILS CLOSED: every truth cell must
     have an engine record for every source, or the run aborts naming the cells.
F39's table was computed over 136 scored cells of the 140 it claimed; see F39-DRAFT.md "Correction (2026-09-11)".

`--out <path>` added 2026-09-13 (runbook Step 7b Slice 0; PROGRESS.md row L2; AC-520 as amended 2026-09-13).
Until then this script printed and persisted nothing, so the Wilson intervals and positive counts AC-520 requires to
"exist, be pinned and be read" lived only in terminal output. `--out` writes them as the machine-readable record every
`deriver_record` in `spikes/p9/facets.toml` already names, `results/p1-arm-d-panel-scored.json`. **stdout is byte-for-byte
what it always was** whether or not `--out` is given; with `--out` one extra `wrote <path>` line follows the report, which
is `analyse-arm-l.py`'s convention in this same directory. The record carries UNROUNDED precision, recall and Wilson
bounds — the printed table rounds to 2dp for reading, and a difference between the two is the rounding, not a defect.
No wall-clock stamp is written: the inputs are pinned, so the record must be byte-reproducible from them.

`--results-dir` added at the same time, and it is a SECOND defect, not a convenience. The five input constants below
were repo-relative to `spikes/results/`, and runbook Step 2 (`bbdaf35`, 2026-09-12) moved that directory into
`redkiln-corpus`. Measured 2026-09-13 at this repo's HEAD: `python spikes/p1/score-arm-d-panel.py` raises
`FileNotFoundError: spikes/results/p1-arm-d-oracle.jsonl` before scoring a single cell. The default is left at the old
path deliberately — it is what every existing invocation and every quotation of this script says — so the failure stays
visible rather than being papered over by a default that reaches into a working tree. AC-464 requires a corpus read to
resolve through `CORPUS_PIN` at `CORPUS_SHA` via the object store and never through a working tree; `--results-dir`
pointing at a corpus checkout is an operator read, not that resolution, and the object-store route is owed to the stage
file, not taken here.
"""
import argparse, io, json, collections, math, os, sys

RESULTS_DIR = "spikes/results"   # the default, unchanged: see the --results-dir paragraph above
INPUTS = {
    "engine":    "p1-arm-d.jsonl",
    "oracle":    "p1-arm-d-oracle.jsonl",
    "accepted":  "p1-arm-d-panel-accepted.jsonl",
    "contested": "p1-arm-d-contested.json",
    "adj":       "p1-arm-d-adjudications.json",
}

SEMANTIC = ["auth", "data_migration", "ui", "flag_exists"]
MECHANICAL = ["toolchain", "change", "binary_change"]
SOURCES = ("docs", "commits", "union")

# Item slugs the panel files spell differently from the engine records. Applied on read to every panel-derived
# file (accepted, contested, adjudications); never to the engine or oracle records, which carry the true slug.
PANEL_KEY_FIX = {
    "_archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-questions":
        "_archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question",
}


def key(item):
    return PANEL_KEY_FIX.get(item, item)


def norm(v):
    if isinstance(v, bool):
        return "true" if v else "false"
    if v is None:
        return "unknown"
    return str(v).strip().lower()


def wilson(k, n, z=1.96):
    if n == 0:
        return None
    p = k / n
    d = 1 + z * z / n
    c = (p + z * z / (2 * n)) / d
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / d
    return (max(0.0, c - h), min(1.0, c + h))


def main():
    ap = argparse.ArgumentParser(description="P1 arm D final scoring (METHOD-P amendment 9).")
    ap.add_argument("--results-dir", default=RESULTS_DIR,
                    help="directory holding the five p1-arm-d input files (default: %(default)s, which runbook "
                         "Step 2 moved into redkiln-corpus)")
    ap.add_argument("--out", default=None,
                    help="write the scored record as JSON to this path (AC-520's deriver_record). "
                         "stdout is unchanged either way.")
    a = ap.parse_args()
    path = {k: os.path.join(a.results_dir, v).replace("\\", "/") for k, v in INPUTS.items()}
    ENGINE, ORACLE, ACCEPTED, CONTESTED, ADJ = (
        path["engine"], path["oracle"], path["accepted"], path["contested"], path["adj"])

    truth, prov = {}, {}

    for line in io.open(ORACLE, encoding="utf-8"):
        r = json.loads(line)
        # set-valued facets stay lists; binary_change is a JSON bool and must be normalised to the same
        # string domain as the panel/human labels, or `t == "true"` silently reads every positive as false.
        v = r["value"] if r["facet"] in ("toolchain", "change") else norm(r["value"])
        truth[(r["item"], r["facet"])] = v
        prov[(r["item"], r["facet"])] = "oracle"

    for line in io.open(ACCEPTED, encoding="utf-8"):
        r = json.loads(line)
        truth[(key(r["item"]), r["facet"])] = norm(r["value"])
        prov[(key(r["item"]), r["facet"])] = "panel"

    # cells whose only flag was a mechanically-moot adversarial prediction keep the panel's unanimous value
    c = json.load(io.open(CONTESTED, encoding="utf-8"))
    for r in c.get("moot", []):
        vals = {norm(p["value"]) for p in r["panel"]}
        if len(vals) == 1:
            truth[(key(r["item"]), r["facet"])] = vals.pop()
            prov[(key(r["item"]), r["facet"])] = "panel(moot-flag)"

    for r in json.load(io.open(ADJ, encoding="utf-8")):
        truth[(key(r["item"]), r["facet"])] = norm(r["value"])
        prov[(key(r["item"]), r["facet"])] = "human"

    eng = collections.defaultdict(dict)
    shas = set()
    for line in io.open(ENGINE, encoding="utf-8"):
        r = json.loads(line)
        eng[(r["item"], r["facet"])][r["source"]] = r["value"]
        shas.add(r.get("corpus_sha"))

    # The record's own pin, read off the engine records rather than hard-coded (golden constraint 8: the
    # instrument's pin is part of the record). Fail closed on a split set — a record spanning two corpus
    # SHAs is two measurements, and AC-071's `rec.corpus_sha` clause has no answer for it.
    if len(shas) != 1 or None in shas:
        sys.stderr.write(f"ABORT: engine records carry {len(shas)} distinct corpus_sha value(s): "
                         f"{sorted(str(s) for s in shas)}\n")
        sys.exit(3)
    corpus_sha = shas.pop()

    # Fail closed: a truth cell the engine never scored is a defect in the inputs, not a cell to skip.
    missing = sorted((item, f, src) for (item, f) in truth for src in SOURCES if src not in eng.get((item, f), {}))
    if missing:
        sys.stderr.write(f"ABORT: {len(missing)} truth cell/source pairs have no engine record:\n")
        for item, f, src in missing:
            sys.stderr.write(f"  {item}  {f}  {src}\n")
        sys.exit(2)

    print(f"truth cells: {len(truth)}   provenance: {dict(collections.Counter(prov.values()))}")
    print(f"every truth cell has an engine record for every source (fail-closed check passed; key fix applied to "
          f"{len(PANEL_KEY_FIX)} panel slug{'s' if len(PANEL_KEY_FIX) != 1 else ''})\n")
    print(f"{'facet':16}{'source':9}{'prec':>7}{'  95% CI':>16}{'rec':>7}   tp  fp  fn  tn  excl")
    print("-" * 84)
    verdicts = {}
    cells = collections.defaultdict(dict)   # facet -> source -> the same numbers the table prints, unrounded
    for facet in MECHANICAL + SEMANTIC:
        for source in SOURCES:
            tp = fp = fn = tn = excl = 0
            for (item, f), t in truth.items():
                if f != facet:
                    continue
                g = eng[(item, f)][source]  # guaranteed present by the fail-closed check above
                if facet in ("toolchain", "change"):
                    T, G = set(t or []), set(g or [])
                    tp += len(T & G); fp += len(G - T); fn += len(T - G)
                else:
                    if t == "unknown":
                        excl += 1; continue
                    T, G = t == "true", norm(g) == "true"
                    tp += T and G; fp += G and not T; fn += T and not G; tn += (not T) and (not G)
            p = tp / (tp + fp) if tp + fp else None
            r = tp / (tp + fn) if tp + fn else None
            ci = wilson(tp, tp + fp) if tp + fp else None
            cis = f"[{ci[0]:.2f},{ci[1]:.2f}]" if ci else "     n/a"
            print(f"{facet:16}{source:9}{(f'{p:.2f}' if p is not None else '  n/a'):>7}{cis:>16}"
                  f"{(f'{r:.2f}' if r is not None else '  n/a'):>7}  {tp:3} {fp:3} {fn:3} {tn:3}  {excl:3}")
            cells[facet][source] = {
                "precision": p, "recall": r,
                "wilson95": list(ci) if ci else None,
                "positives": tp + fp,          # emitted positives — the denominator of precision
                "tp": tp, "fp": fp, "fn": fn, "tn": tn, "excluded_unknown": excl,
                "measurable": p is not None,
            }
            if source == "union":
                verdicts[facet] = (p, ci, tp + fp, excl)
        print()

    print("=" * 84)
    print("VERDICT on the union source — METHOD-P §1.4: precision >= 0.95 removes the facet from the model")
    print("=" * 84)
    verdict_text = {}
    for facet, (p, ci, n, excl) in verdicts.items():
        if p is None:
            verdict_text[facet] = "NOT MEASURABLE"
            print(f"  {facet:16} NOT MEASURABLE — the script emitted no positive; nothing to be precise about"
                  f"{f' ({excl} cells excluded as unknown)' if excl else ''}")
        else:
            ok = "LEAVES THE MODEL" if ci and ci[0] >= 0.95 else ("point >= 0.95 but interval does not clear"
                                                                 if p >= 0.95 else "STAYS")
            verdict_text[facet] = ok
            print(f"  {facet:16} precision {p:.2f} over {n} positives, 95% CI [{ci[0]:.2f},{ci[1]:.2f}] -> {ok}")
    print()
    print("Reported as PANEL-ADJUDICATED (METHOD-P amendment 9), never as human-panel.")

    if a.out:
        write_record(a.out, corpus_sha, truth, prov, cells, verdict_text, path)
        print(f"wrote {a.out}")


def write_record(out_path, corpus_sha, truth, prov, cells, verdict_text, path):
    """AC-520's `deriver_record`: the machine-readable form of exactly what was just printed.

    No wall-clock stamp and no absolute paths — the inputs are pinned, so two runs over the same inputs must
    produce the same bytes. Numbers are UNROUNDED; the table rounds for reading, this does not.
    """
    record = {
        "record": "p1-arm-d-panel-scored",
        "schema_version": 1,
        "produced_by": "spikes/p1/score-arm-d-panel.py",
        "method": "METHOD-P amendment 9 — panel-adjudicated oracle; reported as PANEL-ADJUDICATED, never as "
                  "human-panel. `unknown` is excluded from scoring, never counted against the script (§1.4).",
        "corpus_sha": corpus_sha,
        # CORPUS-RELATIVE, never the resolved path: an absolute machine-local directory in a record pinned by
        # corpus_sha would make the bytes depend on which box scored it, and AC-464 reads the corpus through
        # CORPUS_PIN, not through whatever directory this invocation happened to point at.
        "inputs": ["results/" + INPUTS[k] for k in ("engine", "oracle", "accepted", "contested", "adj")],
        "wilson_z": 1.96,
        "precision_bar": 0.95,
        "verdict_source": "union",
        "truth_cells": len(truth),
        "provenance": dict(sorted(collections.Counter(prov.values()).items())),
        "panel_key_fix_applied": len(PANEL_KEY_FIX),
        "facets": {
            facet: {
                "verdict": verdict_text[facet],
                "measurable": cells[facet]["union"]["measurable"],
                "precision": cells[facet]["union"]["precision"],
                "wilson95": cells[facet]["union"]["wilson95"],
                "positives": cells[facet]["union"]["positives"],
                "by_source": cells[facet],
            }
            for facet in verdict_text
        },
        "limits": [
            "A facet with no emitted positive is NOT MEASURABLE — not 0.00, not n/a, and not a pass or a failure "
            "(golden constraint 10; Halt 9). A perfect method scores identically.",
            "Unanimous 3-seat panel cells carry no human. arXiv:2605.29800 measured a 9-judge panel supplying "
            "~2.18 effective independent votes, so unanimity fails in the correlated direction.",
            "One repo, one operator, one pin: every number here is over the corpus_sha above.",
            "No facet clears the 0.95 bar, and the bar is not reachable at n=20 for any facet however good — "
            "73 all-correct positives are needed for a lower bound above 0.95 at z=1.96 (F39 Limits 1).",
        ],
    }
    d = os.path.dirname(out_path)
    if d:
        os.makedirs(d, exist_ok=True)
    with io.open(out_path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(record, f, indent=2, ensure_ascii=False)
        f.write("\n")


if __name__ == "__main__":
    main()
