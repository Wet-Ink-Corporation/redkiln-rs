#!/usr/bin/env python3
"""S-SEV blind labelling sheets over F37's adversary findings (spikes/METHOD-SEV.md).

Stdlib only. Makes no dispatch, no model call and no network call. Reads the corpus through the object store at
the pinned sha (`git -C <CORPUS_REPO> show <CORPUS_SHA>:results/p7.jsonl`), never a working tree (AC-464), with
CORPUS_REPO and CORPUS_SHA parsed from this repository's CORPUS_PIN. Reads its seeds from spikes/METHOD-SEV.md, so
the script and the pre-registration cannot disagree about them.

Modes:
  stage1        one dispatch per item (10), drawn by the Stage-1 seed; every finding of each, shuffled, random ids.
  continuation  the other 23 dispatches; every finding of each, shuffled, random ids.
  relabel       38 findings drawn by the Re-label seed from the stage-1 findings (all of them if fewer), shuffled,
                fresh ids. Refuses until 48 hours after the S-SEV labels commit in the corpus.

Writes to --out: sheet-<mode>.md (the blank sheet) and key-<mode>.SEALED.json (id -> finding key and the masked
fields). Prints ONLY counts and sha256 digests, never finding text, dispatch keys or items. Refuses to overwrite.
"""
import argparse
import hashlib
import json
import os
import random
import re
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
METHOD = ROOT / "spikes" / "METHOD-SEV.md"
PIN = ROOT / "CORPUS_PIN"
P7_PATH = "results/p7.jsonl"
LABELS_PATH = "results/sev-f37-labels.md"

# Measured facts the draw is registered against (METHOD-SEV "Corpus and unit"). A corpus that disagrees is refused.
EXPECT_DISPATCHES = 33
EXPECT_FINDINGS = 188
EXPECT_ITEMS = 10
EXPECT_EVIDENCE = {"reproduced": 118, "mutation-survived": 29, "advisory": 22, "narrowed-pin": 19}
RELABEL_N = 38
RELABEL_MIN_HOURS = 48
PREFIX = {"stage1": "S", "continuation": "C", "relabel": "R"}
ANSWERS = ("BLOCKING", "ADVISORY", "CANNOT-TELL")

# The sheet's fixed text. The question is E3's, verbatim; the standard is the one E3's sheet quotes from the
# reviewer card. No line of it starts with "answer:" or "note:" or "### " at column 0 (the parser's structure).
QUESTION = "should this finding have blocked the item?"
STANDARD = """`agents/redkiln-review.md`, the card the reviewer was given:

> `approved` — every acceptance criterion is genuinely satisfied by real,
> reachable behaviour, **with a real test backing each** […]
>
> `changes-requested` — at least one criterion is **unmet, untested**, only
> superficially met […]

step 2: *"is there a test that would fail if the behaviour regressed?"* … *"hunt
for … missing or **tautological tests**"*."""


def die(msg, code=2):
    print(f"make-sheet: REFUSED: {msg}", file=sys.stderr)
    sys.exit(code)


def git(*args, cwd=None):
    cmd = ["git"] + (["-C", str(cwd)] if cwd else []) + list(args)
    return subprocess.run(cmd, capture_output=True)


def read_pin():
    if not PIN.is_file():
        die(f"{PIN} is absent")
    kv = {}
    for line in PIN.read_text(encoding="utf-8").splitlines():
        m = re.match(r"^([A-Z_]+)=(.*)$", line.strip())
        if m:
            kv[m.group(1)] = m.group(2).strip()
    repo, sha = kv.get("CORPUS_REPO"), kv.get("CORPUS_SHA")
    if not repo or not sha or not re.fullmatch(r"[0-9a-f]{40}", sha):
        die("CORPUS_PIN does not carry CORPUS_REPO and a 40-hex CORPUS_SHA")
    return repo, sha


def read_seeds():
    if not METHOD.is_file():
        die(f"{METHOD} is absent — the method is committed before any sheet exists")
    text = METHOD.read_text(encoding="utf-8")
    out = {}
    for name, label in (("stage1", "Stage-1 seed"), ("relabel", "Re-label seed")):
        hits = re.findall(r"^\*\*" + re.escape(label) + r":\*\* `(\d+)`\s*$", text, flags=re.M)
        if len(hits) != 1:
            die(f"METHOD-SEV.md carries {len(hits)} '**{label}:**' lines, not exactly one")
        out[name] = int(hits[0])
    return out


def require_committed():
    """The method and this script are committed and unmodified: a sheet made from an uncommitted method is not
    pre-registered (s8b §8b.6: 'committed before the sheet is generated')."""
    rels = ["spikes/METHOD-SEV.md", "spikes/sev/make-sheet.py"]
    for rel in rels:
        if git("ls-files", "--error-unmatch", rel, cwd=ROOT).returncode != 0:
            die(f"{rel} is not tracked — commit the method and this script first (or pass --dry-run)")
    st = git("status", "--porcelain", "--", *rels, cwd=ROOT)
    if st.returncode != 0 or st.stdout.strip():
        die("spikes/METHOD-SEV.md or spikes/sev/make-sheet.py has uncommitted changes (or pass --dry-run)")
    att = [ln for ln in METHOD.read_text(encoding="utf-8").splitlines() if ln.startswith("**Attestation (")]
    if len(att) != 1 or "____" in att[0]:
        die("METHOD-SEV.md's attestation line is absent or unsigned — the operator signs it before the sheet exists")


def repo_head():
    r = git("rev-parse", "HEAD", cwd=ROOT)
    return r.stdout.decode().strip() if r.returncode == 0 else None


def load_corpus(repo, sha):
    if not Path(repo).is_dir():
        die(f"the corpus repository '{repo}' is absent")
    if git("cat-file", "-e", f"{sha}^{{commit}}", cwd=repo).returncode != 0:
        die(f"the pinned corpus commit {sha} is absent from {repo}'s object store")
    r = git("show", f"{sha}:{P7_PATH}", cwd=repo)
    if r.returncode != 0:
        die(f"{P7_PATH} is absent at {sha}")
    raw = r.stdout
    recs = [json.loads(line) for line in raw.decode("utf-8").split("\n") if line.strip()]
    return raw, recs


def build_dispatches(recs):
    """Every adversary dispatch, keyed (run_key, step.attempt); step.identity is not unique (D21-54)."""
    correct = {}
    for r in recs:
        if r.get("phase") == "emit":
            if r["run_key"] in correct:
                die(f"two emit records share a run_key")
            sc = r.get("scored")
            correct[r["run_key"]] = sc.get("correct") if isinstance(sc, dict) else None
    disp = {}
    for r in recs:
        if r.get("phase") != "adversary":
            continue
        key = (r["run_key"], int(r["step"]["attempt"]))
        if key in disp:
            die("two adversary records share a (run_key, attempt) key")
        f1 = r["digest"]["findings"]
        f2 = (r["step"].get("digest") or {}).get("findings")
        if f2 is not None and f2 != f1:
            die("a dispatch's digest.findings differs from its step.digest.findings")
        if r["run_key"] not in correct:
            die("an adversary dispatch has no emit record for its run_key")
        disp[key] = {
            "run_key": key[0],
            "attempt": key[1],
            "item": r["item"],
            "arm": r["arm"],
            "arm_label": r.get("arm_label"),
            "correct": correct[r["run_key"]],
            "findings": f1,
        }
    ds = [disp[k] for k in sorted(disp, key=lambda k: (disp[k]["item"], k[0], k[1]))]
    nf = sum(len(d["findings"]) for d in ds)
    ev = {}
    for d in ds:
        for f in d["findings"]:
            ev[f["evidence"]] = ev.get(f["evidence"], 0) + 1
    items = sorted({d["item"] for d in ds})
    if len(ds) != EXPECT_DISPATCHES or nf != EXPECT_FINDINGS or len(items) != EXPECT_ITEMS or ev != EXPECT_EVIDENCE:
        die(f"the corpus does not match the registered facts: {len(ds)} dispatches, {nf} findings, "
            f"{len(items)} items, evidence {ev}")
    return ds, items


def stage1_draw(ds, items, seed):
    """One dispatch per item: items in sorted order, each item's dispatches in (run_key, attempt) order,
    random.Random(seed).choice once per item."""
    rng = random.Random(seed)
    chosen = []
    for it in items:
        pool = [d for d in ds if d["item"] == it]
        chosen.append(rng.choice(pool))
    return chosen


def flat_findings(dispatches):
    out = []
    for d in dispatches:
        for i, f in enumerate(d["findings"]):
            out.append((d, i, f))
    return out


def assign_ids(entries, mode, seed):
    """Shuffle, then a fresh 6-hex id per entry, from random.Random('S-SEV/<mode>/<seed>')."""
    rng = random.Random(f"S-SEV/{mode}/{seed}")
    entries = list(entries)
    rng.shuffle(entries)
    seen, out = set(), []
    for e in entries:
        while True:
            fid = f"{PREFIX[mode]}-{rng.getrandbits(24):06x}"
            if fid not in seen:
                seen.add(fid)
                break
        out.append((fid, e))
    return out


def _clean(s):
    return str(s).replace("\r\n", "\n").replace("\r", "\n")


def _block(text):
    lines = _clean(text).split("\n")
    return "\n".join(("    " + ln) if ln.strip() else "" for ln in lines)


def render_entry(fid, f):
    """One finding. Every content line is indented four spaces, so no finding text can forge the column-0
    structure ('### <id>', 'answer:', 'note:') the scorer parses."""
    rp = f.get("repro") or {}
    files = f.get("files") or []
    parts = [f"### {fid}", ""]
    for label, val in (("claim", f.get("claim", "")), ("rationale", f.get("rationale", ""))):
        parts += [f"{label}:", "", _block(val), ""]
    parts += ["files:", "", "\n".join("    - " + _clean(x).replace("\n", " ") for x in files) or "    (none)", ""]
    for label in ("command", "observed", "expected_when_met"):
        parts += [f"repro.{label}:", "", _block(rp.get(label, "")), ""]
    parts += ["answer: ", "", "---", ""]
    return "\n".join(parts)


def render_sheet(mode, entries, dry):
    head = [
        f"# S-SEV labelling sheet — {mode}",
        "",
    ]
    if dry:
        head += ["**DRY RUN — NOT A LABELLING SHEET. Made from an uncommitted method; never label it.**", ""]
    head += [
        f"Spike S-SEV (spikes/METHOD-SEV.md). {len(entries)} findings. Each is shown under a random id. Its",
        "evidence class, arm, rigor, item, dispatch and the run's root-cause correctness are masked.",
        "",
        f"**The question**, verbatim from E3's sheet: *\"{QUESTION}\"*",
        "",
        "Write exactly one of `BLOCKING`, `ADVISORY` or `CANNOT-TELL` after each `answer:` line's colon.",
        "`BLOCKING` = the reviewer was right to return `changes-requested` on this.",
        "`ADVISORY` = true, but should not have blocked. `CANNOT-TELL` = the finding as shown does not let you say.",
        "An optional free-text line starting `note:` at the left margin may follow an answer; the scorer ignores it.",
        "Do not edit any other line: the scorer checks the sheet's text against the sealed key's digest.",
        "",
        "## The standard, quoted so you are annotating against the same one",
        "",
        STANDARD,
        "",
        "---",
        "",
    ]
    body = [render_entry(fid, e[2]) for fid, e in entries]
    return "\n".join(head) + "\n" + "\n".join(body)


def normalized_sha256(text):
    """The digest a sheet is checked by: CR/LF-agnostic, trailing whitespace dropped, every column-0 'answer:'
    line reduced to 'answer:', every column-0 'note:' line removed. A blank and a filled sheet agree iff
    nothing but answers and notes changed."""
    out = []
    for ln in _clean(text).split("\n"):
        ln = ln.rstrip()
        if ln.startswith("note:"):
            continue
        if ln.startswith("answer:"):
            ln = "answer:"
        out.append(ln)
    return hashlib.sha256("\n".join(out).encode("utf-8")).hexdigest()


def inside(child, parent):
    try:
        Path(child).resolve().relative_to(Path(parent).resolve())
        return True
    except ValueError:
        return False


def check_relabel_clock(repo, labels_commit):
    if not labels_commit or not re.fullmatch(r"[0-9a-f]{40}", labels_commit):
        die("relabel needs --labels-commit <40-hex S-SEV labels commit in the corpus>")
    if git("cat-file", "-e", f"{labels_commit}:{LABELS_PATH}", cwd=repo).returncode != 0:
        die(f"{LABELS_PATH} is not at {labels_commit}")
    if git("cat-file", "-e", f"{labels_commit}^:{LABELS_PATH}", cwd=repo).returncode == 0:
        die(f"{LABELS_PATH} already existed before {labels_commit} — that is not the commit that added the labels")
    r = git("show", "-s", "--format=%ct", labels_commit, cwd=repo)
    ct = int(r.stdout.decode().strip())
    wait = ct + RELABEL_MIN_HOURS * 3600 - int(time.time())
    if wait > 0:
        die(f"the re-label is {RELABEL_MIN_HOURS} h after the labels commit; {wait / 3600:.1f} h remain")
    return ct


def main():
    for stream in (sys.stdout, sys.stderr):
        stream.reconfigure(encoding="utf-8")
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("mode", choices=["stage1", "continuation", "relabel"])
    ap.add_argument("--out", required=True, help="output directory, outside both repositories "
                    "(METHOD-SEV names D:/repos/redkiln-sev/)")
    ap.add_argument("--labels-commit", help="relabel only: the corpus commit that added results/sev-f37-labels.md")
    ap.add_argument("--dry-run", action="store_true", help="allow an uncommitted method and skip the re-label "
                    "clock; the sheet is stamped DRY RUN and is never labelled")
    a = ap.parse_args()

    repo, sha = read_pin()
    seeds = read_seeds()
    if not a.dry_run:
        require_committed()
    out = Path(a.out)
    if inside(out, repo) or inside(out, ROOT):
        die("--out is inside a repository; the working sheet lives outside both (the corpus checkout stays clean)")
    sheet_p, key_p = out / f"sheet-{a.mode}.md", out / f"key-{a.mode}.SEALED.json"
    for p in (sheet_p, key_p):
        if p.exists():
            die(f"{p} exists — a sheet is never regenerated over another")

    raw, recs = load_corpus(repo, sha)
    ds, items = build_dispatches(recs)
    s1 = stage1_draw(ds, items, seeds["stage1"])
    s1_keys = {(d["run_key"], d["attempt"]) for d in s1}

    extra = {}
    if a.mode == "stage1":
        disp, entries, seed = s1, flat_findings(s1), seeds["stage1"]
    elif a.mode == "continuation":
        disp = [d for d in ds if (d["run_key"], d["attempt"]) not in s1_keys]
        entries, seed = flat_findings(disp), seeds["stage1"]
    else:
        if not a.dry_run:
            extra["labels_commit"] = a.labels_commit
            extra["labels_commit_time"] = check_relabel_clock(repo, a.labels_commit)
        pool = flat_findings(s1)
        rng = random.Random(seeds["relabel"])
        entries = rng.sample(pool, min(RELABEL_N, len(pool)))
        disp, seed = s1, seeds["relabel"]
        extra["drawn_from_stage1_n"] = len(pool)
    ided = assign_ids(entries, a.mode, seed)

    sheet = render_sheet(a.mode, ided, a.dry_run)
    key = {
        "spike": "S-SEV",
        "mode": a.mode,
        "dry_run": a.dry_run,
        "seed": seed,
        "id_rng": f"S-SEV/{a.mode}/{seed}",
        "corpus_repo": repo,
        "corpus_sha": sha,
        "p7_sha256": hashlib.sha256(raw).hexdigest(),
        "repo_head": repo_head(),
        "sheet_file": sheet_p.name,
        "sheet_normalized_sha256": normalized_sha256(sheet),
        "n_findings": len(ided),
        "n_dispatches": len(disp) if a.mode != "relabel" else len({(e[0]["run_key"], e[0]["attempt"]) for e in entries}),
        "entries": {
            fid: {
                "run_key": d["run_key"], "attempt": d["attempt"], "finding_index": i,
                "evidence": f["evidence"], "arm": d["arm"], "arm_label": d["arm_label"],
                "item": d["item"], "correct": d["correct"],
            }
            for fid, (d, i, f) in ided
        },
    }
    if a.mode != "relabel":
        key["dispatches"] = [{"run_key": d["run_key"], "attempt": d["attempt"], "item": d["item"],
                              "n_findings": len(d["findings"])} for d in disp]
    key.update(extra)

    out.mkdir(parents=True, exist_ok=True)
    with open(sheet_p, "x", encoding="utf-8", newline="\n") as fh:
        fh.write(sheet)
    with open(key_p, "x", encoding="utf-8", newline="\n") as fh:
        fh.write(json.dumps(key, sort_keys=True, indent=1, ensure_ascii=True) + "\n")
    key_sha = hashlib.sha256(key_p.read_bytes()).hexdigest()

    print(f"mode: {a.mode}{'  (DRY RUN)' if a.dry_run else ''}")
    print(f"findings: {key['n_findings']}")
    print(f"dispatches: {key['n_dispatches']}")
    print(f"sheet: {sheet_p}")
    print(f"sheet normalized sha256: {key['sheet_normalized_sha256']}")
    print(f"sealed key: {key_p}")
    print(f"sealed key sha256: {key_sha}")
    print("Record the sealed key sha256 in spikes/METHOD-SEV.md by a dated amendment BEFORE labelling. "
          "Do not open the key file.")


if __name__ == "__main__":
    main()
