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
# #18: `_intake-brief.md` and `discover.md` inserted after the card and before spec.md -- they were absent
# from both this list and the selector's, narrowing P2's scope silently. `_ledger.md` is deliberately NOT
# added: it is a running log, not a derived stage artifact, so it has no `derived_from` to reconstruct.
STAGE_ORDER = ["story.md", "bug.md", "project.md", "initiative.md", "incident.md",
               "_intake-brief.md", "discover.md", "spec.md", "_plan.md", "plan.md",
               "implementation-report.md", "report.md", "_integration.md", "_review.md", "_retrospective.md", "closure.md"]
COARSE = {"_intake-brief.md": ["<card>"], "discover.md": ["<card>", "_intake-brief.md"],
          "spec.md": ["<card>", "discover.md"], "_plan.md": ["spec.md"], "plan.md": ["spec.md"], "implementation-report.md": ["spec.md", "_plan.md", "plan.md"],
          "report.md": ["spec.md", "_plan.md", "plan.md"], "_integration.md": ["implementation-report.md", "report.md"],
          "_review.md": ["spec.md", "implementation-report.md", "report.md"], "_retrospective.md": ["_review.md", "report.md"], "closure.md": ["_retrospective.md", "_review.md"]}
PATH_RE = re.compile(r"`([^`\n]+?\.md)`")
# #15: the corpus files that stand for artifacts.toml's `source = "human"` roots (BugReport, IncidentRecord,
# IntakeRecord, ImpactAssessment, SessionNotes). PC3's advisory path is defined over exactly these.
HUMAN_SOURCED = {"bug.md", "incident.md", "story.md", "project.md", "initiative.md", "_intake-brief.md",
                 "discover.md"}
def git(repo, *a, ok=False):
    r = subprocess.run(["git", *a], cwd=repo, capture_output=True)
    if r.returncode and not ok: raise SystemExit(r.stderr.decode(errors="replace"))
    return r.stdout.decode("utf-8", "replace")
def history(repo, sha, path):
    # #14: was `--format=%H\t%ct\t%an\t%b`. %b is the commit BODY -- routinely 20-40 lines in this repo --
    # so .splitlines() produced one row per body line and every row after the first had 1 field, raising
    # IndexError in the human_authored test below. %s is the single-line subject. Rows are padded so a
    # subject containing a tab cannot shorten a row either.
    out = []
    for l in git(repo, "log", "--format=%H\t%ct\t%an\t%s", sha, "--", path, ok=True).splitlines():
        if not l:
            continue
        f = l.split("\t", 3)
        while len(f) < 4:
            f.append("")
        out.append(f)
    return out
def content(repo, rev, path): return git(repo, "show", f"{rev}:{path}", ok=True)
def digest(s): return hashlib.sha256(s.encode()).hexdigest()[:12]
def mentions(text, name):
    """#16: whole-word mention of an artifact filename, not a substring."""
    stem = re.escape(name.removesuffix(".md"))
    return re.search(r"(?<![\w.-])" + re.escape(name) + r"(?![\w])", text) is not None or \
           re.search(r"(?<![\w.-])" + stem + r"(?![\w.-])", text) is not None
def reconstruct(repo, sha, slug, card, arts):
    folder = f".bklg/{slug}"; names = [card] + [a for a in STAGE_ORDER if a in arts and a != card]
    info = {}
    for n in names:
        h = history(repo, sha, f"{folder}/{n}")
        info[n] = {"last_write": h[0][0] if h else None, "last_ts": int(h[0][1]) if h else None, "revisions": len(h),
                   # #15: was a commit-authorship heuristic -- `"redkiln" or "claude" in author+body`.
                   # Every commit in this repo is co-authored by Claude and prose bodies say "redkiln"
                   # constantly, so it was False for EVERY artifact and phase `cases` produced 0 of the 10
                   # advisory cases METHOD-P 2.2 requires. Remapped to the registry: artifacts.toml marks
                   # BugReport / IntakeRecord / IncidentRecord / ImpactAssessment / SessionNotes
                   # `source = "human"`, and in this corpus those are the CARD files.
                   "human_authored": n in HUMAN_SOURCED,
                   "human_authored_rule": "R-artifacts-toml-source-human",
                   "digest": digest(content(repo, sha, f"{folder}/{n}"))}
    recs = []
    for n in names:
        if n == card: continue
        text = content(repo, sha, f"{folder}/{n}"); ups = [u for u in names if names.index(u) < names.index(n)]
        # #16: was `u in text or u.removesuffix(".md") in text` -- a raw substring test. "report.md"
        # matches inside "implementation-report.md", and the stripped forms "plan"/"report"/"spec" match
        # ordinary English prose, so `fine` was near-universally over-broad. Now a word-boundary match on
        # the basename, and the stripped form is required to be a whole word too.
        fine = [u for u in ups
                if info[u]["last_ts"] and info[n]["last_ts"] and info[u]["last_ts"] <= info[n]["last_ts"]
                and mentions(text, u)]
        coarse = [u for u in ups if u in [c.replace("<card>", card) for c in COARSE.get(n, [])]]
        recs.append({"item": slug, "artifact": n, "fine": fine or "unknown", "coarse": coarse, **info[n]})
    return recs
def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--redkiln", required=True); ap.add_argument("--sha", required=True); ap.add_argument("--items", required=True)
    ap.add_argument("--phase", required=True, choices=["reconstruct", "cases", "engine"]); ap.add_argument("--out", required=True)
    # #17: phase `engine` derived its cases path as a.out.replace("engine", "cases") -- which mangles any
    # path containing "engine" elsewhere, and if --out lacked the substring it re-read the file it was about
    # to truncate (silently producing an empty result). Now explicit.
    ap.add_argument("--labels", default=None); ap.add_argument("--cases", default=None); a = ap.parse_args()
    items = [l.rstrip("\n").split("\t") for l in open(a.items, encoding="utf-8") if l.strip() and not l.startswith("#")]
    now = datetime.datetime.now(datetime.timezone.utc).replace(tzinfo=None).isoformat() + "Z"
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
                # #19: the CARD is a legitimate upstream (COARSE maps spec.md -> <card>) and is the most
                # revised file in an item, but `reconstruct` excludes it from `recs`, so it could never be
                # perturbed. Prepend it as a candidate; its downstream is every artifact after it.
                card_rec = {"artifact": card, "digest": digest(content(a.redkiln, a.sha, f"{folder}/{card}"))}
                for r in [card_rec] + recs:
                    hist = history(a.redkiln, a.sha, f"{folder}/{r['artifact']}")
                    if len(hist) >= 2 and n_pert < 20:
                        prev = hist[1][0]; n_pert += 1
                        out.write(json.dumps({"corpus_sha": a.sha, "at": now, "case": f"P{n_pert:02}", "kind": "perturb", "item": slug, "upstream": r["artifact"],
                                              "replaced_with_rev": prev, "prev_digest": digest(content(a.redkiln, prev, f"{folder}/{r['artifact']}")), "cur_digest": r["digest"],
                                              "downstream": [x["artifact"] for x in recs if STAGE_ORDER.index(x["artifact"]) > STAGE_ORDER.index(r["artifact"])]}) + "\n")
                        break
                for r in recs:
                    # #18: a card is a ROOT (no upstream), so it cannot be an advisory case; the advisory
                    # candidates are the human-sourced artifacts that do have an upstream.
                    if r["human_authored"] and n_adv < 10 and r["fine"] != "unknown":
                        n_adv += 1
                        out.write(json.dumps({"corpus_sha": a.sha, "at": now, "case": f"A{n_adv:02}", "kind": "advisory", "item": slug, "artifact": r["artifact"], "upstream": r["fine"]}) + "\n")
                        break
        print(f"perturbation cases={n_pert}/20 advisory cases={n_adv}/10")
    else:
        if not a.labels: raise SystemExit("--labels required for the engine phase (METHOD-P §0 rule 1)")
        lt = open(a.labels).read(); m = re.search(r"labelled_at:\s*(\S+)", lt)
        if not m or "FILL" in m.group(1): raise SystemExit("labels are not timestamped — do not run the engine")
        cases_path = a.cases or a.out.replace("engine", "cases")  # #17
        if os.path.abspath(cases_path) == os.path.abspath(a.out):
            raise SystemExit("--cases resolves to --out; pass --cases explicitly (#17)")
        cases = [json.loads(l) for l in open(cases_path, encoding="utf-8")]
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
