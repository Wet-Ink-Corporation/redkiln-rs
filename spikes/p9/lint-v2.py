#!/usr/bin/env python3
"""P9 lint — v2 FROZEN 2026-09-08 (schema §9 + §9a + §9b); reads registry-v2.toml. v1 is frozen as lint-v1.py so F32/F33 result files replay; v3 is lint.py.

Rules: R1 graph shape (acyclic, loops bounded, on_false table form, feedback refs, loop-learning)
       R2 numeric timeouts/budget after substitution
       R3 refs resolve (step AND path), '?' on conditional targets, emit shape, gate.answer only
       R4 well-formed at every rigor >= min_rigor
       R5 evidence promise vs minted evidence (normative table §9a.6)
       R6 questions in registry; escalate has no inline options
       R7 for_each list-typed; fan-out outputs consumed via [*] or whole
       R8 attempts_total >= agent/tool steps on one pass (for_each counts once)
       R9 predicate vocabulary + grammar (quoted non-identifier values, ordered facets for >=)
       R10 registries: toolchain verbs/feeds/timeouts, checklist:, kb:
       FRAG fragment detector (unchanged)
Usage: python3 lint.py [catalog_dir]  -> exit 1 on any error.
"""
import re, sys, glob, os
from collections import defaultdict
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
RIGOR = ["forbidden", "discouraged", "optional", "recommended", "mandatory"]
BUILTINS = {"rigor", "item", "true", "false", "null", "and", "or", "not", "in", "len", "none", "all", "any"}
REF = re.compile(r"\$\{([^}]+)\}")
IDENT = re.compile(r"^[a-z_][a-z0-9_]*$")

def load(p):
    with open(p, "rb") as f: return tomllib.load(f)

facets = load(os.path.join(HERE, "facets.toml"))["facet"]
questions = load(os.path.join(HERE, "questions.toml"))["question"]
schemas = load(os.path.join(HERE, "schemas.toml"))["value"]
registry = load(os.path.join(HERE, "registry-v2.toml"))   # frozen 2026-09-08: replay F34 against the registry shape of the time
VERBS = set(registry["toolchain"]["verbs"]["values"]); FEEDS = set(registry["toolchain"]["feeds"]["values"])
TIMEOUTS = set(registry["toolchain"]["timeouts"]["values"])
CHECKLISTS = set(registry["checklists"]["values"]); KB = set(registry["kb"]["values"])
FACET_VALUES = {str(v) for f in facets.values() for v in f["values"]}
BARE_VALUES = {v for v in FACET_VALUES if IDENT.match(v)}

class Report:
    def __init__(self): self.items = []
    def err(self, act, rule, msg): self.items.append(("ERROR", act, rule, msg))
    def warn(self, act, rule, msg): self.items.append(("WARN", act, rule, msg))

def parse_ref(r):
    """'${step.outputs[*].field?}' -> (step, path list, optional, projected)."""
    r = r.strip(); opt = r.endswith("?")
    if opt: r = r[:-1]
    proj = "[*]" in r; r = r.replace("[*]", "")
    parts = r.split("."); return parts[0], parts[1:], opt, proj

def refs_in(value):
    if isinstance(value, str): return [parse_ref(m) for m in REF.findall(value)]
    if isinstance(value, dict): return [x for v in value.values() for x in refs_in(v)]
    if isinstance(value, list): return [x for v in value for x in refs_in(v)]
    return []

def coalesce_groups(value):
    """For a string like '${a?} ?? ${b}', return [[refA, refB]]; plain refs return singletons."""
    if not isinstance(value, str): return []
    return [[parse_ref(m) for m in REF.findall(part)] for part in [value] if "??" in value]

def rigor_eval(when, rigor):
    if not when: return True
    for m in re.finditer(r"rigor\s*(>=|<=|<|>|==)\s*(\w+)", when):
        op, lvl = m.groups()
        if lvl not in RIGOR: return True
        a, b = RIGOR.index(rigor), RIGOR.index(lvl)
        ok = {">=": a >= b, "<=": a <= b, "<": a < b, ">": a > b, "==": a == b}[op]
        if not ok and " or " not in when: return False
    return True

def strip_strings(expr): return re.sub(r"'[^']*'|\"[^\"]*\"", "''", expr)

def on_false_of(s):
    """§9b.14: on_false = { to, max } | 'continue' | 'fail'. Returns (to, max, form_ok)."""
    of = s.get("on_false")
    if of is None: return None, None, True
    if isinstance(of, str): return of, None, of in ("continue", "fail")
    if isinstance(of, dict): return of.get("to"), of.get("max"), "to" in of and (of["to"] in ("continue", "fail", "deviate") or "max" in of)
    return None, None, False

def lint_activity(path, rep):
    doc = load(path); act = doc["activity"]; aid = act["id"]
    steps = doc.get("step", []); ids = [s["id"] for s in steps]; by_id = {s["id"]: s for s in steps}
    decl = {sid: i for i, sid in enumerate(ids)}
    if len(ids) != len(set(ids)): rep.err(aid, "R3", "duplicate step ids")
    known_keys = {"id","kind","role","run","when","inputs","outputs","timeout_secs","retry","repeat","on_exhausted","escalate","expr","on_false","deviate","question","artifacts","evidence","for_each","memo"}
    act_keys = {"id","version","stage","provides","requires","derived_from","tags","applicable_if","method_rank","min_rigor","cost_class","evidence","budget","human_source_ok","deviation_ok","keywords","rationale"}
    for k in set(act) - act_keys: rep.err(aid, "R3", f"[activity] unknown key '{k}'")
    for s in steps:
        for k in set(s) - known_keys: rep.err(aid, "R3", f"{s['id']}: unknown key '{k}'")

    step_refs = {}
    for s in steps:
        rs = []
        for key in ("inputs", "for_each", "when", "repeat", "artifacts", "evidence", "expr"):
            if key in s: rs += refs_in(s[key]["until"] if key == "repeat" else s[key])
        step_refs[s["id"]] = rs

    # --- conditional steps: `when`, or only reachable through on_false
    conditional = {s["id"] for s in steps if s.get("when")}
    onfalse_targets = {on_false_of(s)[0] for s in steps if on_false_of(s)[0] in by_id}

    # --- R3: refs resolve (step + path), '?' marks, gate.answer, emit shape
    def out_schema(s): return s.get("outputs", "").replace("value:", "")
    for s in steps:
        for tgt, path_, opt, proj in step_refs[s["id"]]:
            if tgt == "item":
                if not s.get("for_each"): rep.err(aid, "R3", f"{s['id']}: ${{item}} used without for_each")
                continue
            if tgt not in by_id: rep.err(aid, "R3", f"{s['id']}: unresolved ref ${{{tgt}}}"); continue
            t = by_id[tgt]
            if tgt in conditional and not opt and s.get("when") != t.get("when"):
                rep.err(aid, "R3", f"{s['id']}: ref to conditional step '{tgt}' lacks '?'")
            if t["kind"] == "gate":
                if path_ and path_ != ["answer"]: rep.err(aid, "R3", f"{s['id']}: a gate exposes only '.answer' (got ${{{tgt}.{'.'.join(path_)}}})")
                continue
            if t["kind"] in ("check", "emit"):
                if path_ or s["kind"] != "emit": rep.err(aid, "R3", f"{s['id']}: {t['kind']} step '{tgt}' has no outputs to reference")
                continue
            if not path_: continue                       # bare ${step} (evidence lists) is allowed
            if path_[0] != "outputs": rep.err(aid, "R3", f"{s['id']}: ${{{tgt}.{path_[0]}}} — only '.outputs' exists on {t['kind']} steps"); continue
            if len(path_) > 1:
                sch = schemas.get(out_schema(t), {})
                if path_[1] not in sch.get("fields", []): rep.err(aid, "R3", f"{s['id']}: field '{path_[1]}' not in {out_schema(t)} schema")
    emits = [s for s in steps if s["kind"] == "emit"]
    if len(emits) != 1: rep.err(aid, "R3", f"expected exactly one emit step, found {len(emits)}")
    else:
        e = emits[0]; bound = set(e.get("artifacts", {}))
        for k in act["provides"]:
            if k not in bound: rep.err(aid, "R3", f"provides '{k}' not bound in emit")
        for k, v in e.get("artifacts", {}).items():
            rs = refs_in(v)
            if all(opt for _, _, opt, _ in rs) and rs:
                rep.err(aid, "R3", f"emit: artifact '{k}' has no unconditional source (use '??' with an unconditional fallback)")

    # --- R1: cycles, loops bounded, on_false form, feedback refs, loop-learning
    edges = defaultdict(set); feedback = []
    for s in steps:
        for tgt, _, opt, _ in step_refs[s["id"]]:
            if tgt in by_id and tgt != s["id"]:
                if opt: feedback.append((tgt, s["id"]))
                else: edges[tgt].add(s["id"])
    state = {}
    def dfs(u, stack):
        state[u] = 1; stack.append(u)
        for v in edges[u]:
            if state.get(v) == 1: rep.err(aid, "R1", f"cycle through {' -> '.join(stack[stack.index(v):] + [v])}"); return
            if v not in state: dfs(v, stack)
        stack.pop(); state[u] = 2
    for u in ids:
        if u not in state: dfs(u, [])
    back_targets = onfalse_targets | {s["id"] for s in steps if s.get("repeat")}
    for tgt, src in feedback:
        if decl[tgt] > decl[src] and src not in back_targets:
            rep.err(aid, "R1", f"{src}: feedback ref to later step '{tgt}' but no on_false/repeat targets '{src}' (§9b.12)")
    for s in steps:
        if s.get("repeat") and "max" not in s["repeat"]: rep.err(aid, "R1", f"{s['id']}: repeat without max")
        to, mx, ok = on_false_of(s)
        if s["kind"] == "check":
            if not ok: rep.err(aid, "R1", f"{s['id']}: on_false must be {{ to = <earlier step>, max = N }} | 'continue' | 'fail' | {{ to = 'deviate', ... }} (§9b.14)")
            elif to in by_id:
                if decl[to] > decl[s["id"]]: rep.err(aid, "R1", f"{s['id']}: on_false targets LATER step '{to}' — forward branching is `when` on the downstream step (§9a.1)")
                if not isinstance(mx, int): rep.err(aid, "R1", f"{s['id']}: on_false loop-back needs a numeric max")
            elif to == "deviate":
                d = s.get("deviate") or {}
                if not {"kind", "target", "claimed"} <= set(d): rep.err(aid, "R1", f"{s['id']}: deviate needs kind/target/claimed (§9b.16)")
            elif to not in ("continue", "fail", None): rep.err(aid, "R1", f"{s['id']}: on_false target '{to}' does not exist")
        elif s.get("on_false"): rep.err(aid, "R1", f"{s['id']}: only check steps take on_false")
        if s["kind"] == "check" and "on_false" not in s: rep.err(aid, "R1", f"{s['id']}: check without on_false")
    # loop-learning (§9b.13): every loop-back target has a feedback ref or memo=false
    for t in back_targets:
        if t not in by_id: continue
        ts = by_id[t]
        if ts["kind"] not in ("agent", "tool"): continue
        has_feedback = any(decl.get(x, -1) >= decl[t] and o for x, _, o, _ in step_refs[t] if x in by_id)
        if not has_feedback and ts.get("memo", True) is not False:
            rep.err(aid, "R1", f"{t}: loop-back target has no feedback ref and memo != false — the loop cannot learn (§9b.13)")

    # --- R2
    b = act.get("budget", {})
    for s in steps:
        if s["kind"] in ("agent", "tool"):
            t = s.get("timeout_secs")
            if t is None: rep.err(aid, "R2", f"{s['id']}: missing timeout_secs")
            elif isinstance(t, str):
                m = re.fullmatch(r"\$\{toolchain\.(\w+)\}", t)
                if not m or m.group(1) not in TIMEOUTS: rep.err(aid, "R2", f"{s['id']}: timeout '{t}' not numeric after substitution")
        elif "timeout_secs" in s: rep.warn(aid, "R2", f"{s['id']}: timeout on a {s['kind']} step is ignored")
    for k in ("attempts_total", "tokens", "wall_clock_secs"):
        if not isinstance(b.get(k), int): rep.err(aid, "R2", f"budget.{k} must be a number, got {b.get(k)!r}")

    # --- R4
    min_r = act.get("min_rigor", "optional")
    for r in RIGOR[RIGOR.index(min_r):]:
        present = {s["id"] for s in steps if rigor_eval(s.get("when"), r)}
        for s in steps:
            if s["id"] not in present: continue
            for tgt, _, opt, _ in step_refs[s["id"]]:
                if tgt in by_id and tgt not in present and not opt: rep.err(aid, "R4", f"@{r}: {s['id']} needs '{tgt}' which is inactive at this rigor")
        if emits and emits[0]["id"] not in present: rep.err(aid, "R4", f"@{r}: emit inactive")
        for s in steps:
            if s["kind"] == "check" and s["id"] in present:
                ins = [(t, o) for t, _, o, _ in refs_in(s.get("inputs", {})) if t in by_id]
                if ins and all(o and t not in present for t, o in ins):
                    rep.warn(aid, "R4", f"@{r}: check '{s['id']}' evaluates over null inputs only (vacuous per §9a.2)")

    # --- R5 evidence (normative minting table §9a.6)
    def evidence_of(s):
        k = s["kind"]; out = out_schema(s)
        if k == "gate": return {"HumanAnswer"}
        if k == "tool": return {"ToolReport"} | ({"GreenAt"} if s.get("run") == "toolchain:test" else set()) | ({"Mutation"} if out == "MutationReport" else set())
        if k == "agent": return {"Findings"} if out == "Findings" else {"SelfCheck"} if out == "SelfCheck" else set()
        return set()
    if emits:
        ev_refs = [parse_ref(m) for e in emits[0].get("evidence", []) for m in REF.findall(e)]
        produced_uncond, produced_any = set(), set()
        for tgt, _, opt, _ in ev_refs:
            if tgt not in by_id: rep.err(aid, "R5", f"emit evidence ref '{tgt}' unresolved"); continue
            ev = evidence_of(by_id[tgt])
            if not ev: rep.err(aid, "R5", f"emit evidence ref '{tgt}' ({by_id[tgt]['kind']}) mints no Evidence")
            produced_any |= ev
            if not opt and tgt not in conditional: produced_uncond |= ev
            if tgt in conditional and not opt: rep.err(aid, "R5", f"emit evidence '{tgt}' is conditional but not marked '?'")
        for p in act.get("evidence", []):
            name, opt = (p[:-1], True) if p.endswith("?") else (p, False)
            if name not in produced_any: rep.err(aid, "R5", f"promised evidence '{p}' minted by no step")
            elif not opt and name not in produced_uncond: rep.err(aid, "R5", f"promised evidence '{name}' is unconditional but only conditional steps mint it")
        for e in produced_any - {p.rstrip("?") for p in act.get("evidence", [])}:
            rep.warn(aid, "R5", f"evidence '{e}' attached but not promised")

    # --- R6
    for s in steps:
        q = s.get("question") or (s.get("escalate") or {}).get("question")
        if s["kind"] == "gate" or s.get("escalate"):
            if q not in questions: rep.err(aid, "R6", f"{s['id']}: question '{q}' not in registry")
        if s.get("on_exhausted") == "escalate" and not s.get("escalate"): rep.err(aid, "R6", f"{s['id']}: on_exhausted=escalate without escalate block")
        if s.get("escalate") and "options" in s["escalate"]: rep.err(aid, "R6", f"{s['id']}: options on escalate — the registry owns them (§9a.8)")

    # --- R7
    fan = {s["id"] for s in steps if s.get("for_each")}
    for s in steps:
        fe = s.get("for_each")
        if fe:
            m = REF.fullmatch(fe.strip())
            if m:
                tgt, path_, _, _ = parse_ref(m.group(1))
                if tgt not in by_id: rep.err(aid, "R7", f"{s['id']}: for_each source '{tgt}' is not a step (artifact sources use 'artifact:Kind.field')")
                else:
                    out = out_schema(by_id[tgt]); fld = path_[1] if len(path_) > 1 else None
                    if fld is None or fld not in schemas.get(out, {}).get("lists", []): rep.err(aid, "R7", f"{s['id']}: for_each source is not a list field of {out}")
            elif fe.startswith("artifact:"):
                kind, _, fld = fe[9:].partition(".")
                if fld not in schemas.get(kind, {}).get("lists", []): rep.err(aid, "R7", f"{s['id']}: for_each artifact field '{fe}' not a known list")
            else: rep.err(aid, "R7", f"{s['id']}: for_each must be a ${{ref}} or artifact:Kind.field")
        for tgt, path_, _, proj in step_refs[s["id"]]:
            if tgt in fan and len(path_) > 1 and not proj and not s.get("for_each"):
                rep.err(aid, "R7", f"{s['id']}: reads '{'.'.join(path_)}' of fan-out step '{tgt}' without [*] projection (§9a.5)")

    # --- R8 (§9a.7 / §9b.23)
    need = sum(1 for s in steps if s["kind"] in ("agent", "tool"))
    if isinstance(b.get("attempts_total"), int) and b["attempts_total"] < need:
        rep.err(aid, "R8", f"budget.attempts_total={b['attempts_total']} < {need} agent/tool steps on one pass")

    # --- R9 grammar + vocabulary
    def check_expr(where, expr, local):
        raw = REF.sub("REF", expr)
        for m in re.finditer(r"(?<![\w'\"])([a-z]+(?:-[a-z0-9]+)+)(?![\w'\"])", raw):      # bare hyphenated token
            rep.err(aid, "R9", f"{where}: '{m.group(1)}' must be quoted — not an identifier (§9b.18)")
        for m in re.finditer(r"\b([a-z_]+)\s*(>=|<=|<|>)", raw):
            f = m.group(1)
            if f in facets and not facets[f].get("ordered"): rep.err(aid, "R9", f"{where}: '{f}' compared with {m.group(2)} but facet is not ordered")
        e = strip_strings(raw)
        e = re.sub(r"\.(none|all|any)\([^()]*\)", ".QUANT()", e)
        e = re.sub(r"\.[A-Za-z_][A-Za-z0-9_]*", "", e)
        for ident in set(re.findall(r"[A-Za-z_][A-Za-z0-9_]*", e)):
            if ident in ("QUANT", "REF") or ident in BUILTINS or ident in local or ident in facets or ident in BARE_VALUES: continue
            rep.err(aid, "R9", f"{where}: identifier '{ident}' is not a facet, input, or builtin")
    if act.get("applicable_if"): check_expr("applicable_if", act["applicable_if"], set())
    for t in act.get("tags", []):
        f, _, v = t.partition(":")
        if f not in facets or v not in {str(x) for x in facets[f]["values"]}: rep.err(aid, "R9", f"tag '{t}' not in facet registry")
    for s in steps:
        local = set(s.get("inputs", {}).keys())
        if s.get("when"): check_expr(f"{s['id']}.when", s["when"], local)
        if s.get("expr"): check_expr(f"{s['id']}.expr", s["expr"], local)
        if s.get("repeat"): check_expr(f"{s['id']}.repeat.until", s["repeat"]["until"], local)
        if s.get("deviate", {}).get("claimed"): check_expr(f"{s['id']}.deviate.claimed", s["deviate"]["claimed"], local)

    # --- R10 registries: verbs, feeds, checklist:, kb:, artifact:
    for s in steps:
        if s["kind"] == "tool":
            run = s.get("run", "")
            if not run.startswith("toolchain:") or run[10:] not in VERBS: rep.err(aid, "R10", f"{s['id']}: tool verb '{run}' not in registry")
        for k, v in (s.get("inputs") or {}).items():
            if not isinstance(v, str): continue
            src, _, rest = v.partition(":")
            if src == "toolchain" and rest.rstrip("?") not in FEEDS: rep.err(aid, "R10", f"{s['id']}.{k}: toolchain feed '{rest}' not in registry")
            if src == "checklist" and rest not in CHECKLISTS: rep.err(aid, "R10", f"{s['id']}.{k}: checklist '{rest}' not in registry")
            if src == "kb" and rest not in KB: rep.err(aid, "R10", f"{s['id']}.{k}: kb atom '{rest}' not in registry")
            if src == "artifact":
                kind, _, fld = rest.rstrip("?").partition(".")
                if fld and fld.split("[")[0].split(".")[0] not in schemas.get(kind, {}).get("fields", []) and kind in schemas:
                    rep.err(aid, "R10", f"{s['id']}.{k}: '{fld}' not a field of {kind}")

    # --- misc shape
    for s in steps:
        if s["kind"] == "agent" and "role" not in s: rep.err(aid, "R3", f"{s['id']}: agent without role")
        if s["kind"] == "agent" and s.get("role") not in facets["role"]["values"]: rep.err(aid, "R3", f"{s['id']}: role '{s.get('role')}' not in facet registry")
        if s["kind"] in ("agent", "tool"):
            if "outputs" not in s: rep.err(aid, "R3", f"{s['id']}: no outputs")
            elif out_schema(s) not in schemas: rep.err(aid, "R3", f"{s['id']}: output schema '{out_schema(s)}' unknown")
    return doc

def fragments(docs):
    def shape(s):
        return (s["kind"], s.get("role"), s.get("run"), bool(s.get("when")), bool(s.get("repeat")), bool(s.get("on_false")), bool(s.get("for_each")))
    seen = defaultdict(set)
    for aid, doc in docs.items():
        st = doc.get("step", [])
        for n in (2, 3):
            for i in range(len(st) - n + 1): seen[tuple(shape(s) for s in st[i:i+n])].add(aid)
    return {k: v for k, v in seen.items() if len(v) >= 2}

def main():
    cat = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "catalog")
    rep = Report(); docs = {}
    for p in sorted(glob.glob(os.path.join(cat, "*.toml"))):
        try:
            d = lint_activity(p, rep); docs[d["activity"]["id"]] = d
        except Exception as ex: rep.err(os.path.basename(p), "PARSE", repr(ex))
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

if __name__ == "__main__": main()
