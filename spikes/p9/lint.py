#!/usr/bin/env python3
"""P9 lint prototype for the activity-graph schema (references/2026-09-07-activity-graph-schema.md §9).

Python stand-in for the Rust implementation. Rules R1-R9 plus FRAG (fragment detector).
Usage: python3 lint.py [catalog_dir]  -> prints findings, exits 1 on any error.
"""
import re, sys, glob, os, json
from collections import defaultdict
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
RIGOR = ["forbidden", "discouraged", "optional", "recommended", "mandatory"]
TOOLCHAIN_KEYS = {"test_timeout", "implement_timeout", "review_timeout", "repro_timeout", "mutation_timeout"}
BUILTINS = {"rigor", "item", "true", "false", "null", "and", "or", "not", "in", "len", "none", "all", "any"}
REF = re.compile(r"\$\{([^}]+)\}")

def load(p):
    with open(p, "rb") as f:
        return tomllib.load(f)

facets = load(os.path.join(HERE, "facets.toml"))["facet"]
questions = load(os.path.join(HERE, "questions.toml"))["question"]
schemas = load(os.path.join(HERE, "schemas.toml"))["value"]
FACET_VALUES = {str(v) for f in facets.values() for v in f["values"]}

class Report:
    def __init__(self): self.items = []
    def err(self, act, rule, msg): self.items.append(("ERROR", act, rule, msg))
    def warn(self, act, rule, msg): self.items.append(("WARN", act, rule, msg))

def parse_ref(r):
    """'${step.outputs.field?}' -> (step, path, optional). Also '${gate.answer}', '${item}'."""
    r = r.strip()
    opt = r.endswith("?")
    if opt: r = r[:-1]
    parts = r.split(".")
    return parts[0], parts[1:], opt

def refs_in(value):
    if isinstance(value, str):
        return [parse_ref(m) for m in REF.findall(value)]
    if isinstance(value, dict):
        return [x for v in value.values() for x in refs_in(v)]
    if isinstance(value, list):
        return [x for v in value for x in refs_in(v)]
    return []

def rigor_eval(when, rigor):
    """Evaluate only the `rigor` comparisons in `when`; other conjuncts are unknown (treated as True).
    Returns False only if a rigor conjunct is definitely false."""
    if not when: return True
    for m in re.finditer(r"rigor\s*(>=|<=|<|>|==)\s*(\w+)", when):
        op, lvl = m.groups()
        if lvl not in RIGOR: return True
        a, b = RIGOR.index(rigor), RIGOR.index(lvl)
        ok = {">=": a >= b, "<=": a <= b, "<": a < b, ">": a > b, "==": a == b}[op]
        if not ok and " or " not in when: return False
    return True

def strip_strings(expr):
    return re.sub(r"'[^']*'|\"[^\"]*\"", "''", expr)

def lint_activity(path, rep):
    doc = load(path); act = doc["activity"]; aid = act["id"]
    steps = doc.get("step", [])
    ids = [s["id"] for s in steps]
    by_id = {s["id"]: s for s in steps}
    if len(ids) != len(set(ids)): rep.err(aid, "R3", "duplicate step ids")

    # --- collect refs per step
    step_refs = {}
    for s in steps:
        rs = []
        for key in ("inputs", "for_each", "when", "repeat", "artifacts", "evidence", "expr"):
            if key in s:
                v = s[key]["until"] if key == "repeat" else s[key]
                rs += refs_in(v)
        step_refs[s["id"]] = rs

    # --- R3: refs resolve; '?' on conditional targets; emit shape
    conditional = {s["id"] for s in steps if s.get("when")}
    # steps only reachable via on_false are conditional too
    onfalse_targets = {s.get("on_false") for s in steps if s.get("on_false") not in (None, "continue", "fail")}
    conditional |= {t for t in onfalse_targets if t in by_id and not any(
        t in [r[0] for r in step_refs[o]] for o in ids if o != t)}
    for s in steps:
        for tgt, path_, opt in step_refs[s["id"]]:
            if tgt in ("item",):
                if not s.get("for_each"): rep.err(aid, "R3", f"{s['id']}: ${{item}} used without for_each")
                continue
            if tgt not in by_id:
                rep.err(aid, "R3", f"{s['id']}: unresolved ref ${{{tgt}}}"); continue
            if tgt in conditional and not opt and s.get("when") != by_id[tgt].get("when"):
                rep.err(aid, "R3", f"{s['id']}: ref to conditional step '{tgt}' lacks '?'")
            if path_ and path_[0] == "answer" and by_id[tgt]["kind"] != "gate":
                rep.err(aid, "R3", f"{s['id']}: .answer on non-gate '{tgt}'")
            if path_ and path_[0] == "outputs" and by_id[tgt]["kind"] in ("gate", "emit"):
                rep.err(aid, "R3", f"{s['id']}: .outputs on {by_id[tgt]['kind']} step '{tgt}'")
    emits = [s for s in steps if s["kind"] == "emit"]
    if len(emits) != 1: rep.err(aid, "R3", f"expected exactly one emit step, found {len(emits)}")
    else:
        e = emits[0]
        bound = set(e.get("artifacts", {}))
        for k in act["provides"]:
            if k not in bound: rep.err(aid, "R3", f"provides '{k}' not bound in emit")
        for k, v in e.get("artifacts", {}).items():
            for tgt, _, opt in refs_in(v):
                if opt: rep.err(aid, "R3", f"emit: artifact '{k}' bound to optional ref '{tgt}?' — a provided artifact must be unconditional")

    # --- R1: acyclic on non-optional refs; feedback refs need a back-edge
    edges = defaultdict(set); feedback = []
    for s in steps:
        for tgt, _, opt in step_refs[s["id"]]:
            if tgt in by_id and tgt != s["id"]:
                if opt: feedback.append((tgt, s["id"]))
                else: edges[tgt].add(s["id"])
    # cycle check
    state = {}
    def dfs(u, stack):
        state[u] = 1; stack.append(u)
        for v in edges[u]:
            if state.get(v) == 1: rep.err(aid, "R1", f"cycle through {' -> '.join(stack[stack.index(v):] + [v])}"); return
            if v not in state: dfs(v, stack)
        stack.pop(); state[u] = 2
    for u in ids:
        if u not in state: dfs(u, [])
    # order: topological position
    order = {}
    def topo():
        seen = set(); out = []
        def visit(u):
            if u in seen: return
            seen.add(u)
            for v in edges[u]: visit(v)
            out.append(u)
        for u in ids: visit(u)
        return {u: i for i, u in enumerate(reversed(out))}
    order = topo()
    back_targets = set(onfalse_targets) | {s["id"] for s in steps if s.get("repeat")}
    decl = {sid: i for i, sid in enumerate(ids)}
    for tgt, src in feedback:
        if decl[tgt] > decl[src]:  # ref to a later-declared step = feedback
            if src not in back_targets:
                rep.err(aid, "R1", f"{src}: feedback ref to later step '{tgt}' but nothing loops back to '{src}'")
    for s in steps:
        if s.get("repeat") and "max" not in s["repeat"]: rep.err(aid, "R1", f"{s['id']}: repeat without max")
        if s.get("on_false") and s.get("on_false") not in ("continue", "fail") and s["on_false"] not in by_id:
            rep.err(aid, "R1", f"{s['id']}: on_false target '{s['on_false']}' does not exist")
        if s.get("on_false") and s["on_false"] in by_id and decl[s["on_false"]] > decl[s["id"]]:
            rep.warn(aid, "R1", f"{s['id']}: on_false jumps FORWARD to '{s['on_false']}' — a branch, not a loop; schema only defines loop-back semantics")

    # --- R2: numeric timeouts after substitution
    for s in steps:
        if s["kind"] in ("agent", "tool"):
            t = s.get("timeout_secs")
            if t is None: rep.err(aid, "R2", f"{s['id']}: missing timeout_secs")
            elif isinstance(t, str):
                m = re.fullmatch(r"\$\{toolchain\.(\w+)\}", t)
                if not m or m.group(1) not in TOOLCHAIN_KEYS: rep.err(aid, "R2", f"{s['id']}: timeout '{t}' not numeric after substitution")
        elif "timeout_secs" in s: rep.warn(aid, "R2", f"{s['id']}: timeout on a {s['kind']} step is ignored")
    b = act.get("budget", {})
    for k in ("attempts_total", "tokens", "wall_clock_secs"):
        if not isinstance(b.get(k), int): rep.err(aid, "R2", f"budget.{k} must be a number, got {b.get(k)!r}")

    # --- R4: well-formed at every rigor >= min_rigor
    min_r = act.get("min_rigor", "optional")
    for r in RIGOR[RIGOR.index(min_r):]:
        present = {s["id"] for s in steps if rigor_eval(s.get("when"), r)}
        for s in steps:
            if s["id"] not in present: continue
            for tgt, _, opt in step_refs[s["id"]]:
                if tgt in by_id and tgt not in present and not opt:
                    rep.err(aid, "R4", f"@{r}: {s['id']} needs '{tgt}' which is inactive at this rigor")
        if emits and emits[0]["id"] not in present: rep.err(aid, "R4", f"@{r}: emit inactive")
        # policy/check steps whose every input is optional and inactive
        for s in steps:
            if s["kind"] == "check" and s["id"] in present:
                ins = [(t, o) for t, _, o in refs_in(s.get("inputs", {})) if t in by_id]
                if ins and all(o and t not in present for t, o in ins):
                    rep.warn(aid, "R4", f"@{r}: check '{s['id']}' has no live inputs — evaluates over absent values")

    # --- R5: evidence promise satisfiable
    def evidence_of(s):
        k = s["kind"]; out = s.get("outputs", "")
        if k == "gate": return {"HumanAnswer"}
        if k == "tool": return {"ToolReport"} | ({"GreenAt"} if s.get("run") == "toolchain:test" else set()) | ({"Mutation"} if "Mutation" in out else set())
        if k == "agent":
            if "Findings" in out: return {"Findings"}
            if "SelfCheck" in out: return {"SelfCheck"}
        return set()
    if emits:
        ev_refs = [parse_ref(m) for e in emits[0].get("evidence", []) for m in REF.findall(e)]
        produced_uncond, produced_any = set(), set()
        for tgt, _, opt in ev_refs:
            if tgt not in by_id: rep.err(aid, "R5", f"emit evidence ref '{tgt}' unresolved"); continue
            ev = evidence_of(by_id[tgt])
            if not ev: rep.err(aid, "R5", f"emit evidence ref '{tgt}' ({by_id[tgt]['kind']}) produces no Evidence variant")
            produced_any |= ev
            if not opt and tgt not in conditional: produced_uncond |= ev
            if tgt in conditional and not opt: rep.err(aid, "R5", f"emit evidence '{tgt}' is conditional but not marked '?'")
        for p in act.get("evidence", []):
            name, opt = (p[:-1], True) if p.endswith("?") else (p, False)
            if name not in produced_any: rep.err(aid, "R5", f"promised evidence '{p}' produced by no step")
            elif not opt and name not in produced_uncond: rep.err(aid, "R5", f"promised evidence '{name}' is unconditional but only conditional steps produce it")
        for e in produced_any - {p.rstrip("?") for p in act.get("evidence", [])}:
            rep.warn(aid, "R5", f"evidence '{e}' attached but not promised in activity.evidence")

    # --- R6: questions
    for s in steps:
        q = s.get("question") or (s.get("escalate") or {}).get("question")
        if s["kind"] == "gate" or s.get("escalate"):
            if q not in questions: rep.err(aid, "R6", f"{s['id']}: question '{q}' not in registry")
        if s.get("on_exhausted") == "escalate" and not s.get("escalate"): rep.err(aid, "R6", f"{s['id']}: on_exhausted=escalate without escalate block")
        if s.get("escalate") and "options" in s["escalate"]:
            rep.warn(aid, "R6", f"{s['id']}: options duplicated on escalate; registry owns them")

    # --- R7: for_each list-typed
    for s in steps:
        fe = s.get("for_each")
        if not fe: continue
        m = REF.fullmatch(fe.strip())
        if m:
            tgt, path_, _ = parse_ref(m.group(1))
            if tgt not in by_id: rep.err(aid, "R7", f"{s['id']}: for_each source '{tgt}' is not a step (artifact sources must use 'artifact:Kind.field')"); continue
            out = by_id[tgt].get("outputs", "").replace("value:", "")
            fld = path_[1] if len(path_) > 1 else None
            if fld is None or fld not in schemas.get(out, {}).get("lists", []):
                rep.err(aid, "R7", f"{s['id']}: for_each source ${{{m.group(1)}}} is not a list field of {out}")
        elif fe.startswith("artifact:"):
            kind, _, fld = fe[9:].partition(".")
            if fld not in schemas.get(kind, {}).get("lists", []): rep.err(aid, "R7", f"{s['id']}: for_each artifact field '{fe}' not a known list")
        else: rep.err(aid, "R7", f"{s['id']}: for_each must be a ${{ref}} or artifact:Kind.field")
    # list outputs consumed as scalars
    fan = {s["id"] for s in steps if s.get("for_each")}
    for s in steps:
        for tgt, path_, _ in step_refs[s["id"]]:
            if tgt in fan and path_ and path_[0] == "outputs" and len(path_) > 1 and not s.get("for_each"):
                rep.err(aid, "R7", f"{s['id']}: reads field '{'.'.join(path_)}' of fan-out step '{tgt}' whose outputs are a list")

    # --- R8: budget vs attempts
    need = sum(1 + s.get("retry", {}).get("max", 0) for s in steps if s["kind"] in ("agent", "tool") and not s.get("for_each"))
    if isinstance(b.get("attempts_total"), int) and b["attempts_total"] < need:
        rep.warn(aid, "R8", f"budget.attempts_total={b['attempts_total']} < worst-case single-pass attempts {need}")

    # --- R9: predicate vocabulary
    def check_expr(where, expr, local):
        e = strip_strings(REF.sub("REF", expr))
        e = re.sub(r"\.(none|all|any)\([^()]*\)", ".QUANT()", e)   # lambda scope: field names inside quantifiers
        e = re.sub(r"\.[A-Za-z_][A-Za-z0-9_]*", "", e)              # drop field accesses; keep roots
        for ident in set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", e)):
            if ident == "QUANT": continue
            if ident in BUILTINS or ident in local or ident in facets or ident in FACET_VALUES or ident == "REF": continue
            rep.err(aid, "R9", f"{where}: identifier '{ident}' is not a facet, input, or builtin")
    app = act.get("applicable_if")
    if app: check_expr("applicable_if", app, set())
    for t in act.get("tags", []):
        f, _, v = t.partition(":")
        if f not in facets or v not in {str(x) for x in facets[f]["values"]}: rep.err(aid, "R9", f"tag '{t}' not in facet registry")
    for s in steps:
        local = set(s.get("inputs", {}).keys())
        if s.get("when"): check_expr(f"{s['id']}.when", s["when"], local)
        if s.get("expr"): check_expr(f"{s['id']}.expr", s["expr"], local)
        if s.get("repeat"): check_expr(f"{s['id']}.repeat.until", s["repeat"]["until"], local)

    # --- misc schema-shape checks
    for s in steps:
        if s["kind"] == "check" and "on_false" not in s: rep.err(aid, "R1", f"{s['id']}: check without on_false")
        if s["kind"] == "agent" and "role" not in s: rep.err(aid, "R3", f"{s['id']}: agent without role")
        if s["kind"] in ("agent", "tool") and "outputs" not in s: rep.err(aid, "R3", f"{s['id']}: no outputs")
        if s["kind"] in ("agent", "tool"):
            out = s["outputs"].replace("value:", "")
            if out not in schemas: rep.err(aid, "R3", f"{s['id']}: output schema '{out}' unknown")
        if s["kind"] == "gate" and s.get("on_false"): rep.err(aid, "R1", f"{s['id']}: gate cannot have on_false")
    return doc

def fragments(docs):
    """Sequences of >=2 consecutive (topologically adjacent by file order) steps with identical shape across activities."""
    def shape(s):
        return (s["kind"], s.get("role"), s.get("run"), bool(s.get("when")), bool(s.get("repeat")), bool(s.get("on_false")), bool(s.get("for_each")))
    seen = defaultdict(set)
    for aid, doc in docs.items():
        st = doc.get("step", [])
        for n in (2, 3):
            for i in range(len(st) - n + 1):
                seen[tuple(shape(s) for s in st[i:i+n])].add(aid)
    return {k: v for k, v in seen.items() if len(v) >= 2}

def main():
    cat = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "catalog")
    rep = Report(); docs = {}
    for p in sorted(glob.glob(os.path.join(cat, "*.toml"))):
        try:
            d = lint_activity(p, rep); docs[d["activity"]["id"]] = d
        except Exception as ex:
            rep.err(os.path.basename(p), "PARSE", repr(ex))
    counts = defaultdict(int)
    for lvl, aid, rule, msg in rep.items:
        print(f"{lvl:5} {aid:24} {rule:5} {msg}"); counts[(lvl, rule)] += 1
    print("\n== summary ==")
    for (lvl, rule), n in sorted(counts.items()): print(f"{lvl:5} {rule:5} {n}")
    print(f"errors={sum(n for (l,_),n in counts.items() if l=='ERROR')} warnings={sum(n for (l,_),n in counts.items() if l=='WARN')}")
    print("\n== fragments repeated in >=2 activities ==")
    for k, v in sorted(fragments(docs).items(), key=lambda kv: -len(kv[1])):
        print(f"{len(v)}x {sorted(v)}: " + " -> ".join(f"{s[0]}{'/'+s[1] if s[1] else ''}{'/'+s[2] if s[2] else ''}{'?' if s[3] else ''}{'↺' if s[4] or s[5] else ''}{'*' if s[6] else ''}" for s in k))
    sys.exit(1 if any(l == "ERROR" for l, *_ in rep.items) else 0)

if __name__ == "__main__":
    main()
