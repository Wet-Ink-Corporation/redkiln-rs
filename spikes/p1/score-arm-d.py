#!/usr/bin/env python3
"""P1 arm D scoring — precision/recall per facet per source against the human labels (METHOD-P §1.3/§1.4).
Refuses to run unless every label file's `labelled_at` timestamp precedes the engine output's earliest `at`
(METHOD-P §0 rule 1). Set-valued facets are scored per (item, value) membership.
Usage: python3 spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl --labels spikes/results/p1-arm-d-labels-ryan.md [--labels ...]

METHOD-P amendment 8 (2026-09-09) — the attested-blind path.
  The chronology check above is a *proxy* for the property §0 rule 1 actually protects, which is blindness. For P1
  arm D that proxy has expired: the AFK run of 2026-09-08 wrote `p1-arm-d.jsonl` (earliest `at`
  2026-09-08T04:54:07Z) before the label template was filled, because it deliberately stopped at the label boundary.
  Any truthful `labelled_at` written afterwards is later than the engine's, so the check refuses permanently.
  Where blindness nonetheless holds in fact, the label file may carry a `blind_attestation:` line and this script may
  be invoked with --attested-blind. Both are required; neither alone is enough.
  This is STRICTLY WEAKER EVIDENCE. A timestamp is machine-checkable; an attestation is a claim by a named person.
  So it must be asked for explicitly on the command line, it is echoed on every run, and any result produced this way
  is reported as ATTESTED-BLIND and never as timestamp-verified.
  Do not backdate `labelled_at`, and do not re-run `derive-facets.py` to refresh the engine's timestamps: arm D is a
  deterministic zero-cost derivation at a fixed pin, so re-running would pass the check while hiding the issue.
"""
import argparse, json, re, collections

ATTESTATION_MIN_LEN = 40


def parse_attestation(path):
    """METHOD-P amendment 8. Return the `blind_attestation:` line, or None if the file carries none.

    The line must name who attests, what they attest they had not opened, and when. A bare marker is refused:
    an attestation nobody signed is not an attestation.
    """
    t = open(path, encoding="utf-8").read()
    m = re.search(r"^blind_attestation:\s*(.+?)\s*$", t, re.M)
    if not m:
        return None
    v = m.group(1).strip()
    if "FILL" in v or len(v) < ATTESTATION_MIN_LEN:
        raise SystemExit(
            f"{path}: blind_attestation is present but not filled in. It must name who attests, what they "
            f"attest they had not opened, and when (METHOD-P amendment 8)."
        )
    return v


def parse_labels(path):
    t = open(path, encoding="utf-8").read(); m = re.search(r"labelled_at:\s*(\S+)", t)
    if not m or "FILL-ME" in m.group(1): raise SystemExit(f"{path}: labelled_at missing — labels must be timestamped before unblinding")
    labels = {}
    for row in re.finditer(r"^\|\s*([^|#]+?)\s*\|\s*(toolchain|change|binary_change|auth|data_migration|ui|flag_exists)\s*\|\s*([^|]*?)\s*\|", t, re.M):
        item, facet, val = row.groups(); val = val.strip()
        if val in ("", "?"): continue
        labels[(item, facet)] = set(v.strip() for v in val.split(",")) if facet in ("toolchain", "change") else (val.lower() in ("1", "true", "yes"))
    return m.group(1), labels


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--engine", required=True)
    ap.add_argument("--labels", action="append", required=True)
    ap.add_argument(
        "--attested-blind", action="store_true",
        help="METHOD-P amendment 8: accept a `blind_attestation:` line in place of an expired chronology check. "
             "Strictly weaker evidence; the run is reported as ATTESTED-BLIND.",
    )
    a = ap.parse_args()
    eng = [json.loads(l) for l in open(a.engine, encoding="utf-8")]; first = min(r["at"] for r in eng)
    attested = {}
    for lp in a.labels:
        ts, labels = parse_labels(lp)
        if ts < first:
            print(f"== {lp} (labelled_at {ts} < engine {first}) — TIMESTAMP-VERIFIED")
        else:
            att = parse_attestation(lp)
            if not a.attested_blind:
                raise SystemExit(
                    f"{lp}: labelled_at {ts} is not before engine first output {first} — METHOD-P §0 rule 1."
                    f"\n  The chronology check has expired for this file. If the labels were nonetheless written"
                    f"\n  blind, add a `blind_attestation:` line and re-run with --attested-blind (amendment 8)."
                    f"\n  Do NOT backdate labelled_at, and do NOT re-run the engine to refresh its timestamps."
                )
            if not att:
                raise SystemExit(
                    f"{lp}: --attested-blind was passed but the file carries no `blind_attestation:` line."
                    f"\n  The flag alone attests nothing (METHOD-P amendment 8)."
                )
            attested[lp] = att
            print(f"== {lp} (labelled_at {ts} >= engine {first}) — ATTESTED-BLIND, not timestamp-verified")
            print(f"   attestation: {att}")
        for source in ("docs", "commits", "union"):
            tp = collections.Counter(); fp = collections.Counter(); fn = collections.Counter(); unknown = collections.Counter()
            for r in (x for x in eng if x["source"] == source):
                key = (r["item"], r["facet"]); lab = labels.get(key)
                if lab is None: unknown[r["facet"]] += 1; continue
                if isinstance(lab, set):
                    got = set(r["value"]); tp[r["facet"]] += len(got & lab); fp[r["facet"]] += len(got - lab); fn[r["facet"]] += len(lab - got)
                else:
                    got = bool(r["value"]); tp[r["facet"]] += got and lab; fp[r["facet"]] += got and not lab; fn[r["facet"]] += (not got) and lab
            for f in sorted(set(tp) | set(fp) | set(fn)):
                p = tp[f] / (tp[f] + fp[f]) if tp[f] + fp[f] else float("nan"); rc = tp[f] / (tp[f] + fn[f]) if tp[f] + fn[f] else float("nan")
                verdict = "LEAVES THE MODEL (>=0.95)" if p >= 0.95 else "stays LLM-emitted"
                print(f"  {source:8} {f:15} precision={p:.2f} recall={rc:.2f} tp={tp[f]} fp={fp[f]} fn={fn[f]} unlabelled={unknown[f]}  {verdict}")
    if attested:
        print()
        print("!! ATTESTED-BLIND RUN (METHOD-P amendment 8).")
        print("!! The blindness of these labels rests on a signed claim, not on a machine-checkable timestamp.")
        print("!! Report every number above as attested-blind. Do not cite it as timestamp-verified.")
        for lp, att in attested.items():
            print(f"!!   {lp}: {att}")


if __name__ == "__main__": main()
