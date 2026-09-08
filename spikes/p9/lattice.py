#!/usr/bin/env python3
"""P6 — catalog completeness over the context lattice (METHOD-P §3; catalog proposal §2.5 steps 1, 2, 4, 5; §2.6).

Enumerates the pre-registered lattice, runs the selection function over the v0 packs with every artifact Missing,
and reports unsatisfiable (stage, ctx) pairs, dead activities, rank ties and appetite overruns. Zero model spend.

Every result line carries the catalog / facets / pack digests (METHOD-P §0 rule 2).

Interpretations this script had to fix (each is an F35 ambiguity, none is in the proposal text):
  I1  producer candidates are filtered to activities whose `stage` equals the requiring stage; a producer's own
      `requires` may be satisfied from ANY non-forbidden stage (upstream artifact) or from a human-supplied ROOT kind
  I2  tags compile per §9b.20 (same facet disjoins, distinct facets conjoin) AND-ed with `applicable_if`
  I3  `rigor` is a legal root in `applicable_if` and in pack `when` rows (used by the catalog and packs)
  I4  a require row's own `rigor` field sets the selection rigor for that row's artifacts (proposal §2.3 example)
  I5  equal-rank ties break by selector specificity (a toolchain facet constraint beats none, PC7 ii); a remaining tie
      is a validation error
  I6  cost_class points S=1 M=3 L=8 XL=20 summed over the selected DAG against pack.appetite.points
  I7  the lattice's rigor axis is an item-level override applied to every non-forbidden stage; a fourth mode
      `pack-default` uses each pack's own policy rigor and is reported separately
  I8  `dynamism` is in METHOD-P §3.2 but not in facets.toml — the ordinal axes are the four registered ones (16 bands)
  I9  `exists(Kind)` (§9b.17 spawn rows) is evaluated against the selected DAG's provided kinds
v2 (2026-09-08, decisions 1–9; v1 frozen as lattice-v1.py so results-p6-run0..run3 replay):
  D1  `max_rigor` ceiling honoured; `--validate` checks family totality (§9c.26/28)
  D2  roots come from artifacts.toml (`source = "human"`); `--validate` checks every unconditional `requires` resolves (§9c.27)
  D4  toolchain-keyed kinds (artifacts.toml `keyed_by`) expand to one instance per toolchain; a producer is admissible for an
      instance with F restricted to that toolchain; no specificity tie-break — equal ranks are ties (§9c.43)
  D6  ordinal bands {low, med, high}; plus an `unknown` sweep reported separately (METHOD-P amendment 7)
  D9  `recent_deploy` in the boolean covering array (§9c.37)
Usage: python3 lattice.py [--packs a,b,...] [--out results-p6-run4.txt] [--validate] [--only-mode M] [--only-tc T]
"""
import sys, os, glob, hashlib, itertools, re, json, argparse
from collections import defaultdict, OrderedDict
try:
    import tomllib
except ModuleNotFoundError:
    import tomli as tomllib

HERE = os.path.dirname(os.path.abspath(__file__))
RIGOR = ["forbidden", "discouraged", "optional", "recommended", "mandatory"]
LIVE_RIGOR = ["optional", "recommended", "mandatory"]
COST = {"S": 1, "M": 3, "L": 8, "XL": 20}
ARTIFACTS = None   # loaded in main from artifacts.toml (§9c.27/43)
ROOTS = set(); KEYED = set()
KINDS = ["initiative", "project", "story", "bug", "incident", "spike", "chore", "tech-debt", "security-vuln", "docs"]
PREREG_PACKS = ["bug-standard", "bug-expedite", "story", "story-ue", "incident", "spike", "project", "initiative"]
ORDINALS = ["criticality", "novelty", "size", "blast_radius"]     # I8
BOOLS = ["auth", "data_migration", "external_dep", "ui", "market_facing", "has_repro", "flag_exists", "recent_deploy"]
BANDS = ["low", "med", "high"]
TOOLCHAINS = OrderedDict([("node", ({"node"}, {"text"})), ("unreal", ({"unreal"}, {"binary"})),
                          ("node+unreal", ({"node", "unreal"}, {"text", "binary"})), ("dotnet", ({"dotnet"}, {"text"}))])
CHANGE_OF = {"node": {"text"}, "dotnet": {"text"}, "unreal": {"binary"}, "python": {"text"}, "rust": {"text"}}

def load(p):
    with open(p, "rb") as f: return tomllib.load(f)

def digest_paths(paths):
    h = hashlib.sha256()
    for p in sorted(paths):
        h.update(os.path.basename(p).encode()); h.update(open(p, "rb").read())
    return h.hexdigest()[:12]

# ---------------------------------------------------------------- predicate evaluation (closed grammar §9a.3 / §9b.18-19)
class Ord:
    __slots__ = ("v", "order")
    def __init__(self, v, order): self.v, self.order = v, order
    def _r(self, o): return self.order.index(o.v if isinstance(o, Ord) else o)
    def __eq__(self, o): return self._r(self) == self._r(o)
    def __ne__(self, o): return not self.__eq__(o)
    def __lt__(self, o): return self._r(self) < self._r(o)
    def __le__(self, o): return self._r(self) <= self._r(o)
    def __gt__(self, o): return self._r(self) > self._r(o)
    def __ge__(self, o): return self._r(self) >= self._r(o)
    def __hash__(self): return hash(self.v)
    def __repr__(self): return self.v

TOK = re.compile(r"\s*(?:('[^']*'|\"[^\"]*\")|(\d+(?:\.\d+)?)|([A-Za-z_][A-Za-z0-9_\-]*)|(==|!=|<=|>=|<|>|\(|\)|\[|\]|,))")

class Predicates:
    def __init__(self, facets):
        self.facets = facets; self._cache = {}
        self.bare = {str(v) for f in facets.values() for v in f["values"] if isinstance(v, str)}
    def compile(self, expr):
        if expr in self._cache: return self._cache[expr]
        code = self._compile(expr); self._cache[expr] = code; return code
    def _compile(self, expr):
        out = []; pos = 0
        while pos < len(expr):
            m = TOK.match(expr, pos)
            if not m or m.end() == pos: raise ValueError(f"cannot tokenise {expr!r} at {pos}")
            pos = m.end(); s, n, ident, op = m.groups()
            if s: out.append(repr(s[1:-1]))
            elif n: out.append(n)
            elif ident:
                if ident in ("and", "or", "not", "in"): out.append(f" {ident} ")
                elif ident == "true": out.append("True")
                elif ident == "false": out.append("False")
                elif ident == "null": out.append("None")
                elif ident == "exists": out.append("_exists")
                elif ident in self.facets or ident == "rigor": out.append(f'F[{ident!r}]')
                elif ident in self.bare: out.append(repr(ident))
                else: raise ValueError(f"identifier {ident!r} is not a facet or facet value in {expr!r}")
            else: out.append(op)
        return "".join(out)
    def eval(self, expr, F, provided=frozenset()):
        code = self.compile(expr)
        if not isinstance(code, type(compile("0", "", "eval"))): code = self._cache[expr] = compile(code, "<pred>", "eval")
        return bool(eval(code, {"__builtins__": {}}, {"F": F, "_exists": lambda k: k in provided}))

# ---------------------------------------------------------------- catalog + packs
class Catalog:
    def __init__(self, cat_dir, facets):
        self.acts = {}
        self.paths = sorted(glob.glob(os.path.join(cat_dir, "*.toml")))
        for p in self.paths:
            a = load(p)["activity"]; self.acts[a["id"]] = a
        self.facets = facets; self.pred = Predicates(facets)
        self.by_kind_stage = defaultdict(list)
        for a in self.acts.values():
            for k in a["provides"]: self.by_kind_stage[(k, a["stage"])].append(a)
        self.by_kind = defaultdict(list)
        for a in self.acts.values():
            for k in a["provides"]: self.by_kind[k].append(a)
    def tag_expr(self, a):
        groups = defaultdict(list)
        for t in a.get("tags", []):
            f, _, v = t.partition(":"); groups[f].append(v)
        parts = []
        for f, vs in groups.items():
            if self.facets[f].get("set"): parts.append("(" + " or ".join(f"'{v}' in {f}" for v in vs) + ")")
            else: parts.append("(" + " or ".join(f"{f} == '{v}'" for v in vs) + ")")
        return " and ".join(parts) if parts else "true"
    def admissible(self, a, F, rigor, inst=None):
        """returns (ok, reason); inst = the toolchain instance for a keyed kind (F restricted to it, §9c.43)"""
        F = dict(F); F["rigor"] = Ord(rigor, RIGOR)
        if inst is not None: F["toolchain"] = {inst}; F["change"] = CHANGE_OF[inst]
        if RIGOR.index(a.get("min_rigor", "optional")) > RIGOR.index(rigor): return False, f"min_rigor={a.get('min_rigor')}"
        if "max_rigor" in a and RIGOR.index(a["max_rigor"]) < RIGOR.index(rigor): return False, f"max_rigor={a['max_rigor']}"
        if not self.pred.eval(self.tag_expr(a), F): return False, "tags"
        ai = a.get("applicable_if")
        if ai and not self.pred.eval(ai, F): return False, f"applicable_if={ai}"
        return True, ""

class Pack:
    def __init__(self, path):
        d = load(path); self.id = d["pack"]["id"]; self.applies = d["pack"]["applies"]
        self.appetite = d["pack"].get("appetite", {}).get("points", 10**9)
        self.policy = d["policy"]["stages"]; self.stages = d.get("stage", {}); self.path = path

# ---------------------------------------------------------------- lattice
def covering_array(n):
    """greedy pairwise covering array over n binary factors (rows of tuples)."""
    rows = []; pairs = {(i, j, a, b) for i in range(n) for j in range(i + 1, n) for a in (0, 1) for b in (0, 1)}
    import random; rnd = random.Random(7)
    while pairs:
        best, bestc = None, -1
        for _ in range(400):
            r = tuple(rnd.randint(0, 1) for _ in range(n))
            c = sum(1 for (i, j, a, b) in pairs if r[i] == a and r[j] == b)
            if c > bestc: best, bestc = r, c
        rows.append(best); pairs -= {(i, j, a, b) for (i, j, a, b) in pairs if best[i] == a and best[j] == b}
    return rows

def contexts(facets, sweep=False):
    """sweep=False: the lattice (3 bands). sweep=True: the `unknown` sweep — one axis unknown, others low, pack-default only."""
    ca = covering_array(len(BOOLS))
    band_sets = [tuple("unknown" if j == i else "low" for j in range(len(ORDINALS))) for i in range(len(ORDINALS))] if sweep \
                else list(itertools.product(BANDS, repeat=len(ORDINALS)))
    for kind in KINDS:
        for rig in (["pack-default"] if sweep else LIVE_RIGOR + ["pack-default"]):
            for bands in band_sets:
                for tname, (tools, change) in TOOLCHAINS.items():
                    for row in ca:
                        for cos in (["standard", "expedite"] if kind == "bug" else ["standard"]):
                            F = {"kind": kind, "cos": cos, "toolchain": tools, "change": change}
                            for ax, b in zip(ORDINALS, bands): F[ax] = Ord(b, facets[ax]["values"])
                            for bn, v in zip(BOOLS, row): F[bn] = bool(v)
                            yield rig, tname, F
    return

def ctx_label(F, tname):
    return f"kind={F['kind']} cos={F['cos']} " + " ".join(f"{a}={F[a]}" for a in ORDINALS) + f" tc={tname} " + " ".join(f"{b}={int(F[b])}" for b in BOOLS)

# ---------------------------------------------------------------- selection (steps 1, 2, 4, 5)
class Run:
    def __init__(self, cat, pack, F, tname, rig_mode):
        self.cat, self.pack, self.F, self.tname, self.rig_mode = cat, pack, F, tname, rig_mode
        self.selected = OrderedDict(); self.unsat = []; self.ties = []; self.rows_fired = []; self.visiting = set()
        self.provided = set(); self.deviation_reachable = set(); self.cycles = []; self.selected_inst = set()
    def stage_rigor(self, stage):
        pol = self.pack.policy.get(stage, "forbidden")
        if pol == "forbidden": return "forbidden"
        return pol if self.rig_mode == "pack-default" else self.rig_mode
    def required(self, stage):
        sec = self.pack.stages.get(stage)
        if sec is None: return {}
        r = self.stage_rigor(stage)
        F = dict(self.F); F["rigor"] = Ord(r, RIGOR)
        req = OrderedDict((k, r) for k in sec.get("baseline", []))
        for row in sec.get("require", []):
            if self.cat.pred.eval(row["when"], F):
                self.rows_fired.append((stage, row["when"]))
                rr = row.get("rigor", r)
                if RIGOR.index(rr) < RIGOR.index(r): rr = r      # I4: row rigor raises, never lowers
                for k in row["artifacts"]: req[k] = rr
        return req
    def producer(self, kind, stage, rigor, why, inst=None):
        """step 4 for one Missing artifact (instance). Returns activity id or None (records unsat)."""
        if kind in KEYED and inst is None:                      # §9c.43: expand to one instance per toolchain
            got = [self.producer(kind, stage, rigor, why, t) for t in sorted(self.F["toolchain"])]
            return got[0] if all(got) else None
        key = (kind, stage, inst)
        if key in self.visiting: self.cycles.append((kind, stage)); return None
        if kind in ROOTS: return "ROOT"
        cands = self.cat.by_kind_stage.get((kind, stage), [])
        adm, fails = [], []
        for a in cands:
            ok, reason = self.cat.admissible(a, self.F, rigor, inst)
            (adm if ok else fails).append((a, reason))
        if not adm:
            other = [a["id"] + "@" + a["stage"] for a in self.cat.by_kind.get(kind, []) if a["stage"] != stage]
            self.unsat.append({"stage": stage, "kind": kind + (f"{{{inst}}}" if inst else ""), "rigor": rigor, "why": why,
                               "failed": [f"{a['id']}:{r}" for a, r in fails], "other_stage": other}); return None
        adm.sort(key=lambda ar: (ar[0]["method_rank"], ar[0]["id"]))
        top = adm[0][0]
        same = [a for a, _ in adm if a["method_rank"] == top["method_rank"]]
        if len(same) > 1: self.ties.append({"stage": stage, "kind": kind + (f"{{{inst}}}" if inst else ""), "tie": sorted(a["id"] for a in same)})
        self.deviation_reachable.update(a["id"] for a, _ in adm[1:])
        if (top["id"], inst) in self.selected_inst: return top["id"]
        self.visiting.add(key); self.selected_inst.add((top["id"], inst))
        self.selected.setdefault(top["id"], {"stage": stage, "for": kind, "rigor": rigor, "specificity_win": False, "instances": []})["instances"].append(inst)
        for k in top["provides"]: self.provided.add((k, inst if k in KEYED else None))
        for req in top.get("requires", []):
            if req.endswith("?"): continue
            rk = req; rinst = inst if rk in KEYED else None
            if (rk, rinst) in self.provided or rk in ROOTS: continue
            # I1: satisfy from the same stage first, then any non-forbidden stage (upstream), preferring pack order
            got = None; tried = []
            stages = [stage] + [s for s in self.pack.policy if s != stage and self.stage_rigor(s) != "forbidden"]
            for s in stages:
                if not self.cat.by_kind_stage.get((rk, s)): continue
                sr = self.stage_rigor(s) if s != stage else rigor
                unsat_before = len(self.unsat)
                got = self.producer(rk, s, sr, f"requires of {top['id']}", rinst)
                if got: break
                tried += [f"{f}@{s}" for u in self.unsat[unsat_before:] for f in (u["failed"] or [f"{u['kind']}:transitive"])]
                del self.unsat[unsat_before:]
            if not got:
                forb = [a["id"] + "@" + a["stage"] for a in self.cat.by_kind.get(rk, []) if self.stage_rigor(a["stage"]) == "forbidden"]
                self.unsat.append({"stage": stage, "kind": rk, "rigor": rigor, "why": f"requires of {top['id']}",
                                   "failed": tried, "other_stage": forb, "forbidden_stage": bool(forb) and not tried})
        self.visiting.discard(key)
        return top["id"]
    def run(self):
        for stage, pol in self.pack.policy.items():
            if pol == "forbidden" or stage == "triage": continue
            for kind, rigor in self.required(stage).items():
                if kind not in KEYED and (kind, None) in self.provided: continue
                self.producer(kind, stage, rigor, "R")
        # step 5: acyclic over selected (requires -> provides edges)
        prov = defaultdict(list)
        for aid in self.selected:
            for k in self.cat.acts[aid]["provides"]: prov[k].append(aid)
        def cycles(include_optional):
            edges = defaultdict(set)
            for aid in self.selected:
                for req in self.cat.acts[aid].get("requires", []):
                    if req.endswith("?") and not include_optional: continue
                    for p in prov.get(req.rstrip("?"), []):
                        if p != aid: edges[p].add(aid)
            state = {}; cyc = []
            def dfs(u, st):
                state[u] = 1; st.append(u)
                for v in edges[u]:
                    if state.get(v) == 1: cyc.append(tuple(st[st.index(v):] + [v]))
                    elif v not in state: dfs(v, st)
                st.pop(); state[u] = 2
            for u in self.selected:
                if u not in state: dfs(u, [])
            return cyc
        self.dag_cycles = cycles(False); self.soft_cycles = cycles(True)
        self.cost = sum(COST[self.cat.acts[a]["cost_class"]] for a in self.selected)
        return self

# ---------------------------------------------------------------- --validate (catalog-level lint, §2.6 / §9c.26-28)
def validate(cat, facets):
    errs = []
    fams = defaultdict(list)
    for a in cat.acts.values():
        if a.get("family"): fams[a["family"]].append(a)
        for req in a.get("requires", []):
            if req.endswith("?"): continue
            if req not in ROOTS and not cat.by_kind.get(req): errs.append(f"requires: {a['id']} needs '{req}' — no producer and not source = human (§9c.27)")
    for k, v in ARTIFACTS.items():
        if v.get("source") == "activity" and v.get("status") != "extended" and not cat.by_kind.get(k): errs.append(f"artifacts.toml: '{k}' is source = activity but nothing provides it")
    ca = covering_array(len(BOOLS))
    for name, members in fams.items():
        provs = {tuple(sorted(m["provides"])) for m in members}
        if len({m["provides"][0] for m in members}) > 1: errs.append(f"family {name}: members provide different primary kinds {sorted(provs)} (§9c.28)")
        ranks = [m["method_rank"] for m in members]
        if len(ranks) != len(set(ranks)): errs.append(f"family {name}: duplicate ranks {ranks} (§9c.28)")
        low = min(members, key=lambda m: m["method_rank"])
        # totality: for every context low is admissible at its floor, and every live rigor >= floor, some member is admissible
        floor = low.get("min_rigor", "optional"); holes = defaultdict(int); n = 0
        for kind in KINDS:
            for bands in itertools.product(BANDS, repeat=len(ORDINALS)):
                for tname, (tools, change) in TOOLCHAINS.items():
                    for row in ca:
                        F = {"kind": kind, "cos": "standard", "toolchain": tools, "change": change}
                        for ax, b in zip(ORDINALS, bands): F[ax] = Ord(b, facets[ax]["values"])
                        for bn, v in zip(BOOLS, row): F[bn] = bool(v)
                        insts = sorted(tools) if low["provides"][0] in KEYED else [None]
                        for inst in insts:
                            if not cat.admissible(low, F, floor, inst)[0]: continue
                            n += 1
                            for r in LIVE_RIGOR:
                                if RIGOR.index(r) < RIGOR.index(floor): continue
                                if not any(cat.admissible(m, F, r, inst)[0] for m in members): holes[(r, kind, tname)] += 1
        for (r, kind, tname), c in sorted(holes.items()):
            errs.append(f"family {name}: no member admissible at rigor={r} for kind={kind} tc={tname} in {c} contexts where {low['id']} is admissible at its floor (§9c.26 totality)")
        print(f"family {name:24} members={len(members)} lowest={low['id']} floor={floor} contexts-checked={n} holes={sum(holes.values())}")
    print("\n".join(errs) if errs else "validate: no errors"); print(f"validate errors={len(errs)}")
    sys.exit(1 if errs else 0)

# ---------------------------------------------------------------- main
def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--packs", default=None); ap.add_argument("--out", default=None)
    ap.add_argument("--catalog", default=os.path.join(HERE, "catalog"))
    ap.add_argument("--only-mode", default=None); ap.add_argument("--only-tc", default=None); ap.add_argument("--validate", action="store_true"); args = ap.parse_args()
    facets = load(os.path.join(HERE, "facets.toml"))["facet"]
    global ARTIFACTS, ROOTS, KEYED
    ARTIFACTS = load(os.path.join(HERE, "artifacts.toml"))["artifact"]
    ROOTS = {k for k, v in ARTIFACTS.items() if v.get("source") == "human"}; KEYED = {k for k, v in ARTIFACTS.items() if v.get("keyed_by") == "toolchain"}
    cat = Catalog(args.catalog, facets)
    if args.validate: return validate(cat, facets)
    pack_paths = sorted(glob.glob(os.path.join(HERE, "packs", "*.toml")))
    packs = [Pack(p) for p in pack_paths]
    if args.packs: packs = [p for p in packs if p.id in args.packs.split(",")]
    dig = {"catalog": digest_paths(cat.paths), "facets": digest_paths([os.path.join(HERE, "facets.toml"), os.path.join(HERE, "artifacts.toml")]),
           "packs": digest_paths([p.path for p in packs])}
    tag = f"cat:{dig['catalog']} fac+art:{dig['facets']} pk:{dig['packs']}"
    out = open(args.out, "w") if args.out else sys.stdout
    def line(s=""): out.write(f"{tag} | {s}\n")
    line(f"P6 lattice run; catalog={len(cat.acts)} activities; packs={[p.id for p in packs]}; pack digests: " +
         " ".join(f"{p.id}={digest_paths([p.path])}" for p in packs))
    ca = covering_array(len(BOOLS)); line(f"lattice v2: kinds={len(KINDS)} rigor={LIVE_RIGOR}+pack-default ordinals={ORDINALS} bands={BANDS}^{len(ORDINALS)}={len(BANDS)**len(ORDINALS)} toolchains={list(TOOLCHAINS)} bool-covering-rows={len(ca)} (pairwise over {BOOLS}); keyed kinds={sorted(KEYED)}; roots={sorted(ROOTS)}; + unknown sweep ({len(ORDINALS)} rows, pack-default)")
    # ---- enumerate
    stats = defaultdict(lambda: defaultdict(int)); unsat_by = defaultdict(lambda: defaultdict(int)); unsat_examples = {}
    selected_ever = defaultdict(set); devreach = set(); ties = defaultdict(set); overruns = defaultdict(int); pairs = defaultdict(int)
    mixed = defaultdict(lambda: defaultdict(int)); spec_wins = defaultdict(int); pack_for_kind = defaultdict(set); dag_cycles = defaultdict(int)
    unsat_forbidden = defaultdict(int); unsat_other_stage = defaultdict(int); n_ctx = defaultdict(int)
    soft_cycles = defaultdict(int); hard_cycle_examples = {}; soft_cycle_examples = {}; binary_only = defaultdict(int)
    pairs_tc = defaultdict(int); unsat_tc = defaultdict(int)
    unsat_sets = {}   # (pack, mode, ctx-sans-toolchain, tname) -> frozenset((stage, kind))
    for rig, tname, F in contexts(facets):
        if args.only_mode and rig != args.only_mode: continue
        if args.only_tc and tname != args.only_tc: continue
        Fp = dict(F); Fp["rigor"] = Ord("optional", RIGOR)
        for pack in packs:
            if not cat.pred.eval(pack.applies, Fp): continue
            pack_for_kind[F["kind"]].add(pack.id)
            mode = rig; r = Run(cat, pack, F, tname, mode).run()
            n_ctx[(pack.id, mode)] += 1
            stages = [s for s, p in pack.policy.items() if p != "forbidden" and s != "triage" and s in pack.stages]
            bad = {u["stage"] for u in r.unsat}
            for s in stages:
                pairs[(pack.id, mode)] += 1; pairs_tc[(mode, tname)] += 1
                if s in bad: stats[(pack.id, mode)]["unsat_pairs"] += 1; unsat_tc[(mode, tname)] += 1
            for u in r.unsat:
                k = (pack.id, u["stage"], u["kind"]); unsat_by[(pack.id, mode)][k] += 1
                if u.get("forbidden_stage"): unsat_forbidden[k] += 1
                if u.get("other_stage") and not u["failed"]: unsat_other_stage[k] += 1
                unsat_examples.setdefault(k, (ctx_label(F, tname), mode, u["why"], u["failed"][:8], u["other_stage"][:4]))
                if tname == "unreal" and u["kind"] in ("Changeset", "ImplReport", "ChangeSummary"): binary_only[(pid_k := pack.id, u["stage"], u["kind"])] += 1
                if tname == "node+unreal": mixed[(pack.id, mode)][(u["stage"], u["kind"])] += 1
            unsat_sets[(pack.id, mode, ctx_label(F, "-"), tname)] = frozenset((u["stage"], u["kind"]) for u in r.unsat)
            for aid, info in r.selected.items():
                selected_ever[aid].add((pack.id, mode))
                if info["specificity_win"]: spec_wins[(pack.id, aid, tname)] += 1
            devreach |= r.deviation_reachable
            for t in r.ties: ties[(pack.id, t["stage"], t["kind"])].add(tuple(t["tie"]))
            if r.cost > pack.appetite: overruns[(pack.id, mode)] += 1
            if r.dag_cycles: dag_cycles[(pack.id, mode)] += 1; hard_cycle_examples.setdefault(pack.id, r.dag_cycles[0])
            if r.soft_cycles: soft_cycles[(pack.id, mode)] += 1; soft_cycle_examples.setdefault(pack.id, r.soft_cycles[0])
            stats[(pack.id, mode)]["cost_sum"] += r.cost; stats[(pack.id, mode)]["cost_max"] = max(stats[(pack.id, mode)]["cost_max"], r.cost)
    # ---- report
    line(); line("== unsatisfiable (stage, ctx) pairs per pack × rigor mode ==")
    tot = defaultdict(int)
    for (pid, mode) in sorted(pairs):
        u = stats[(pid, mode)]["unsat_pairs"]; n = pairs[(pid, mode)]
        pre = "prereg" if pid in PREREG_PACKS else "supp"
        line(f"{pid:14} {mode:13} {pre:6} pairs={n:6} unsat={u:6} ({100*u/n:5.1f}%) contexts={n_ctx[(pid,mode)]:5} mean-cost={stats[(pid,mode)]['cost_sum']/max(1,n_ctx[(pid,mode)]):6.1f} max-cost={stats[(pid,mode)]['cost_max']:4} appetite-overruns={overruns[(pid,mode)]} hard-cycles={dag_cycles[(pid,mode)]} soft-cycles(optional-edge)={soft_cycles[(pid,mode)]}")
        tot[("all", mode)] += u; tot[("alln", mode)] += n
        if pid in PREREG_PACKS: tot[("pre", mode)] += u; tot[("pren", mode)] += n
    for mode in LIVE_RIGOR + ["pack-default"]:
        line(f"TOTAL prereg-8 {mode:13} unsat={tot[('pre',mode)]}/{tot[('pren',mode)]} ({100*tot[('pre',mode)]/max(1,tot[('pren',mode)]):.1f}%)   all-12 unsat={tot[('all',mode)]}/{tot[('alln',mode)]} ({100*tot[('all',mode)]/max(1,tot[('alln',mode)]):.1f}%)")
    line(); line("== unsatisfiable pairs by rigor mode × toolchain slice (all packs) ==")
    for (mode, tn), n in sorted(pairs_tc.items()): line(f"{mode:13} {tn:12} unsat={unsat_tc[(mode,tn)]:6}/{n:6} ({100*unsat_tc[(mode,tn)]/n:5.1f}%)")
    line(); line("== unsatisfiable by (pack, stage, kind): count of contexts, then why (first example) ==")
    agg = defaultdict(int)
    for (pid, mode), d in unsat_by.items():
        for k, n in d.items(): agg[k] += n
    for k, n in sorted(agg.items(), key=lambda kv: -kv[1]):
        ex = unsat_examples[k]
        line(f"{k[0]:14} {k[1]:13} {k[2]:22} n={n:5} forbidden-stage={unsat_forbidden[k]:5} wrong-stage-producer={unsat_other_stage[k]:5} why={ex[2]!r} failed={ex[3]} other_stage={ex[4]} e.g. [{ex[1]}] {ex[0]}")
    line(); line("== DAG cycles: hard (through unconditional requires) and soft (through `?` requires) — one example per pack ==")
    for pid, c in sorted(hard_cycle_examples.items()): line(f"HARD {pid:14} {' -> '.join(c)}")
    for pid, c in sorted(soft_cycle_examples.items()): line(f"soft {pid:14} {' -> '.join(c)}")
    line(); line("== mixed Node+UE slice: unsatisfiable (stage, kind) counts (METHOD-P §3.4 second kill) ==")
    for (pid, mode), d in sorted(mixed.items()):
        for k, n in sorted(d.items()): line(f"{pid:14} {mode:13} {k[0]:13} {k[1]:22} n={n}")
    line(); line("== mixed Node+UE DIFFERENTIAL: (stage, kind) unsatisfiable at tc=node+unreal but satisfiable at tc=node for the same context ==")
    diff = defaultdict(int); sdiff = defaultdict(int); nd = 0; nds = 0; nctx = 0
    twin = {"story-ue": "story"}     # the D14 variant pack that applies at tc=node for the same story context
    for (pid, mode, cl, tn), us in unsat_sets.items():
        if tn != "node+unreal": continue
        nctx += 1; base = unsat_sets.get((twin.get(pid, pid), mode, cl, "node"), frozenset())
        extra = us - base
        if extra: nd += 1
        for e in extra: diff[(pid, e[0], e[1])] += 1
        sextra = {st for st, _ in us} - {st for st, _ in base}
        if sextra: nds += 1
        for st in sextra: sdiff[(pid, st)] += 1
    line(f"mixed contexts={nctx}; with an extra unsatisfiable (stage,kind) vs pure node={nd}; with an extra unsatisfiable STAGE vs pure node={nds} (the kill-criterion reading)")
    for k, n in sorted(diff.items(), key=lambda kv: -kv[1]): line(f"(stage,kind) {k[0]:14} {k[1]:13} {k[2]:22} n={n}")
    for k, n in sorted(sdiff.items(), key=lambda kv: -kv[1]): line(f"STAGE        {k[0]:14} {k[1]:13} n={n}")
    if not diff: line("none — every unsatisfiable pair in the mixed slice is also unsatisfiable for pure Node")
    line(); line("== binary-only (tc=unreal, change={binary}) contexts lacking an implement-stage producer for the text-realised kinds ==")
    for k, n in sorted(binary_only.items()): line(f"{k[0]:14} {k[1]:13} {k[2]:22} n={n}")
    line(); line("== unknown sweep (one ordinal axis = unknown, others low; pack-default): unsatisfiable pairs and which activities an `unknown` reaches via `>=` ==")
    sw_pairs = 0; sw_unsat = 0; sw_sel = defaultdict(int)
    for rig, tname, F in contexts(facets, sweep=True):
        Fp = dict(F); Fp["rigor"] = Ord("optional", RIGOR)
        for pack in packs:
            if not cat.pred.eval(pack.applies, Fp): continue
            r = Run(cat, pack, F, tname, rig).run(); bad = {u["stage"] for u in r.unsat}
            for s in [s for s, p in pack.policy.items() if p != "forbidden" and s != "triage" and s in pack.stages]:
                sw_pairs += 1; sw_unsat += s in bad
            ax = [a for a in ORDINALS if F[a].v == "unknown"][0]
            Fl = dict(F); Fl[ax] = Ord("low", facets[ax]["values"]); base = Run(cat, pack, Fl, tname, rig).run().selected
            for aid in set(r.selected) - set(base): sw_sel[(ax, aid)] += 1
    line(f"unknown sweep: unsat={sw_unsat}/{sw_pairs} ({100*sw_unsat/max(1,sw_pairs):.1f}%); activities selected at `<axis> = unknown` but NOT at `<axis> = low` for the same context — i.e. unknown is routed as high by `>=` (F34 B14):")
    for (ax, aid), n in sorted(sw_sel.items()): line(f"  {ax:13} {aid:28} n={n}")
    line(); line("== rank ties (validation errors) ==")
    if not ties: line("none")
    for k, v in sorted(ties.items()): line(f"{k[0]:14} {k[1]:13} {k[2]:22} {sorted(v)}")
    line(); line("== dead activities (never selected in any pack × context) ==")
    dead = sorted(a for a in cat.acts if a not in selected_ever)
    dead_dev = [a for a in dead if a in devreach]; dead_never = [a for a in dead if a not in devreach]
    line(f"dead={len(dead)}/{len(cat.acts)} ({100*len(dead)/len(cat.acts):.1f}%)  of which deviation-reachable (an admissible higher-rank candidate in some context)={len(dead_dev)}  never-admissible-or-never-required={len(dead_never)}")
    line("dead, deviation-reachable: " + ", ".join(dead_dev))
    line("dead, never: " + ", ".join(dead_never))
    live_pre = {a for a, s in selected_ever.items() if any(p in PREREG_PACKS for p, _ in s)}
    line(f"under the eight pre-registered packs only: dead={len(cat.acts)-len(live_pre)}/{len(cat.acts)} ({100*(len(cat.acts)-len(live_pre))/len(cat.acts):.1f}%)")
    line(); line("== kinds with no pack ==")
    line(", ".join(k for k in KINDS if not pack_for_kind.get(k)) or "none")
    line(); line("== per-activity selection count (pack×mode combinations) ==")
    for a in sorted(cat.acts): line(f"{a:28} {len(selected_ever.get(a, ())):3} {'DEAD' if a in dead else ''}")
    if args.out: out.close()

if __name__ == "__main__": main()
