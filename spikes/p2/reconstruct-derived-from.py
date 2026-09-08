#!/usr/bin/env python3
"""P2 — corpus reconstruction + short-circuit engine (METHOD-P §2.2–2.3, catalog §2.5 step 3, PC3). Zero model cost.

Phase A (reconstruct): for every artifact in every P2 item, from the git history at the pinned SHA:
  last_write      = last commit touching the artifact
  fine derived_from = the upstream artifacts (by stage order) whose own last write precedes this artifact's
                      last write AND which are referenced from its text (backtick path or heading mention); `unknown`
                      when neither signal exists (excluded, per METHOD-P §2.2)
  coarse derived_from = `requires` of the v0 catalog activity that produces this artifact's kind (spec←card,
                      plan←spec, impl-report←spec+plan, review←impl-report+spec, …) — the "= requires" scope
  human_authored  = no agent trailer / `Co-authored-by: redkiln` in any of its commits (PC3 vouched root)
Phase B (cases): 20 perturbation cases — for each item, the first upstream artifact (stage order) with >= 2
  revisions is replaced by its PREVIOUS revision; plus 10 advisory cases where a human-authored artifact's
  upstream changed. Written to spikes/results/p2-cases.jsonl BEFORE the labels (the label template is generated
  from the cases; the human labels each downstream artifact "should re-run / need not" from the diff alone).
Phase C (engine, run only after labels are timestamped): Present / Stale / Missing per downstream artifact under
  coarse and fine scope; advisory lines for human-authored roots (PC3). Output carries corpus sha, item, case,
  scope, verdict, and the digests compared.
Usage: python3 spikes/p2/reconstruct-derived-from.py --redkiln D:/repos/redkiln --sha 47faec9f... --items spikes/plans/p2-items.txt \
           --phase reconstruct|cases|engine --out spikes/results/p2-<phase>.jsonl
NOT YET RUN: corpus repo not mounted when written.
"""
import argparse, json, re, subprocess, hashlib, os, datetime
STAGE_ORDER = ["story.md", "bug.md", "project.md", "initiative.md", "incident.md", "spec.md", "_plan.md", "plan.md",
               "implementation-report.md", "report.md", "_integration.md", "_review.md", "_retrospective.md", "closure.md"]
COARSE = {"spec.md": ["<card>"], "_plan.md": ["spec.md"], "plan.md": ["spec.md"], "implementation-report.md": ["spec.md", "_plan.md", "plan.md"],
          "report.md": ["spec.md", "_plan.md", "plan.md"], "_integration.md": ["implementation-report.md", "report.md"],
          "_review.md": ["spec.md", "implementation-report.md", "report.md"], "_retrospective.md": ["_review.md", "report.md"], "closure.md": ["_retrospective.md", "_review.md"]}
PATH_RE = re.compile(r"`([^`\n]+?\.md)`")
def git(repo, *a, ok=False):
    r = subprocess.run(["git", *a], cwd=repo, capture_output=True)
    if r.returncode and not ok: raise SystemExit(r.stderr.decode(errors="replace"))
    return r.stdout.decode("utf-8", "replace")
def history(repo, sha, path):
    return [l.split("\t") for l in git(repo, "log", "--format=%H\t%ct\t%an\t%b", sha, "--", path, ok=True).splitlines() if l]
def content(repo, rev, path): return git(repo, "show", f"{rev}:{path}", ok=True)
def digest(s): return hashlib.sha256(s.encode()).hexdigest()[:12]
def reconstruct(repo, sha, slug, card, arts):
    folder = f".bklg/{slug}"; names = [card] + [a for a in STAGE_ORDER if a in arts and a != card]
    info = {}
    for n in names:
        h = history(repo, sha, f"{folder}/{n}")
        info[n] = {"last_write": h[0][0] if h else None, "last_ts": int(h[0][1]) if h else None, "revisions": len(h),
                   "human_authored": bool(h) and not any("redkiln" in (x[2] + x[3]).lower() or "claude" in (x[2] + x[3]).lower() for x in h),
                   "digest": digest(content(repo, sha, f"{folder}/{n}"))}
    recs = []
    for n in names:
        if n == card: continue
        text = content(repo, sha, f"{folder}/{n}"); ups = [u for u in names if names.index(u) < names.index(n)]
        fine = [u for u in ups if info[u]["last_ts"] and info[n]["last_ts"] and info[u]["last_ts"] <= info[n]["last_ts"] and (u in text or u.removesuffix(".md") in text)]
        coarse = [u for u in ups if u in [c.replace("<card>", card) for c in COARSE.get(n, [])]]
        recs.append({"item": slug, "artifact": n, "fine": fine or "unknown", "coarse": coarse, **info[n]})
    return recs
def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True); ap.add_argument("--items", required=True)
    ap.add_argument("--phase", required=True, choices=["reconstruct", "cases", "engine"]); ap.add_argument("--out", required=True)
    ap.add_argument("--labels", default=None); a = ap.parse_args()
    items = [l.rstrip("\n").split("\t") for l in open(a.items) if l.strip() and not l.startswith("#")]
    now = datetime.datetime.utcnow().isoformat() + "Z"
    if a.phase == "reconstruct":
        with open(a.out, "w") as out:
            for slug, card, arts, h in items:
                for r in reconstruct(a.redkiln, a.sha, slug, card, arts.split(",")): out.write(json.dumps({"corpus_sha": a.sha, "at": now, **r}) + "\n")
    elif a.phase == "cases":
        n_pert = 0; n_adv = 0
        with open(a.out, "w") as out:
            for slug, card, arts, h in items:
                recs = reconstruct(a.redkiln, a.sha, slug, card, arts.split(","))
                folder = f".bklg/{slug}"
                for r in recs:
                    hist = history(a.redkiln, a.sha, f"{folder}/{r['artifact']}")
                    if len(hist) >= 2 and n_pert < 20:
                        prev = hist[1][0]; n_pert += 1
                        out.write(json.dumps({"corpus_sha": a.sha, "at": now, "case": f"P{n_pert:02}", "kind": "perturb", "item": slug, "upstream": r["artifact"],
                                              "replaced_with_rev": prev, "prev_digest": digest(content(a.redkiln, prev, f"{folder}/{r['artifact']}")), "cur_digest": r["digest"],
                                              "downstream": [x["artifact"] for x in recs if STAGE_ORDER.index(x["artifact"]) > STAGE_ORDER.index(r["artifact"])]}) + "\n")
                        break
                for r in recs:
                    if r["human_authored"] and n_adv < 10 and r["fine"] != "unknown":
                        n_adv += 1
                        out.write(json.dumps({"corpus_sha": a.sha, "at": now, "case": f"A{n_adv:02}", "kind": "advisory", "item": slug, "artifact": r["artifact"], "upstream": r["fine"]}) + "\n")
                        break
        print(f"perturbation cases={n_pert}/20 advisory cases={n_adv}/10")
    else:
        if not a.labels: raise SystemExit("--labels required for the engine phase (METHOD-P §0 rule 1)")
        lt = open(a.labels).read(); m = re.search(r"labelled_at:\s*(\S+)", lt)
        if not m or "FILL" in m.group(1): raise SystemExit("labels are not timestamped — do not run the engine")
        cases = [json.loads(l) for l in open(a.out.replace("engine", "cases"))]
        with open(a.out, "w") as out:
            for c in cases:
                if c["kind"] != "perturb": continue
                recs = {r["artifact"]: r for r in reconstruct(a.redkiln, a.sha, c["item"], next(i[1] for i in items if i[0] == c["item"]), next(i[2] for i in items if i[0] == c["item"]).split(","))}
                for d in c["downstream"]:
                    for scope in ("coarse", "fine"):
                        ups = recs[d][scope]
                        if ups == "unknown": verdict = "excluded"
                        elif c["upstream"] in ups: verdict = "Stale"
                        else: verdict = "Present"
                        out.write(json.dumps({"corpus_sha": a.sha, "at": now, "labels_at": m.group(1), "case": c["case"], "item": c["item"], "downstream": d, "scope": scope,
                                              "verdict": verdict, "upstream_changed": c["upstream"], "derived_from": ups, "digests": [c["prev_digest"], c["cur_digest"]]}) + "\n")
if __name__ == "__main__": main()
