#!/usr/bin/env python3
"""
S-GRAMMAR — a throwaway recursive-descent parser for the predicate grammar as ADR-0103 fixes it, run over every
`applicable_if`, `when` and `on_false` in spikes/p9 (packs, catalog, catalog-b). Registered per ADR-0012 (audit §8
M-02); consumer: runbook Step 7b's lint and ADR-0103. Zero model cost; reads TOML only.

Grammar (EBNF; ADR-0033 + 0040 + 0048 + 0049 + 0070 + 0103):

  predicate  := or_expr
  or_expr    := and_expr { "or" and_expr }                      # left-assoc
  and_expr   := not_expr { "and" not_expr }                     # left-assoc
  not_expr   := "not" not_expr | comparison
  comparison := operand [ ( "==" | "!=" | "<" | "<=" | ">" | ">=" ) value
                       | "in" list ]                            # no chaining
              | string "in" operand                             # membership in a set-valued facet (ADR-0050)
  operand    := path [ "." call ]                               # a field, a root, a ${ref}, or a call on one
  call       := "len" "(" ")" | ("any" | "all" | "none") "(" body ")"
  body       := ident "in" operand "," predicate                # prefix quantifier form (alias)
              | predicate                                       # method form: <list>.any(<body>) with the element in scope
  path       := ident { "." ident } | ref
  ref        := "${" refseg { "." refseg } [ "?" ] "}"           # refseg admits '-' (step ids such as green-verify)
  value      := literal | ref | path "." "len" "(" ")"          # unquoted ident in value position is a literal
  literal    := ident | string | number | "true" | "false" | "null"
  list       := "[" [ value { "," value } ] "]"
  string     := '"' ... '"' | "'" ... "'"     (escapes \" \' \\)
  ident      := [a-z_][a-z0-9_]*  (a bare value literal may also carry '-' inside quotes only)

on_false    := { to = <ident>, max = <int> } | "continue" | "fail" | { to = "deviate", ... }

The probe reports: strings parsed / failed (with the failing string and position), and the feature census the
precedence and literal rules bear on — mixed unparenthesised `and`/`or`, bare identifiers in value position,
`not` applications, `null` literals, quantifier and `len()` uses, `${ref}` uses (with `?`), comparison chains.
"""
import glob, re, sys, tomllib, collections, json, os

TOKEN = re.compile(r"""
    (?P<ws>\s+)
  | (?P<ref>\$\{[a-zA-Z_][a-zA-Z0-9_-]*(?:\.[a-zA-Z_][a-zA-Z0-9_-]*)*\??\})
  | (?P<str>"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')
  | (?P<num>-?\d+(?:\.\d+)?)
  | (?P<op>==|!=|<=|>=|<|>)
  | (?P<punct>[()\[\],.])
  | (?P<ident>[a-zA-Z_][a-zA-Z0-9_]*)
""", re.X)
KEYWORDS = {"and", "or", "not", "in", "true", "false", "null", "len", "any", "all", "none"}

class ParseError(Exception):
    pass

def lex(s):
    pos = 0; out = []
    while pos < len(s):
        m = TOKEN.match(s, pos)
        if not m:
            raise ParseError(f"lex error at {pos}: {s[pos:pos+12]!r}")
        pos = m.end()
        if m.lastgroup == "ws":
            continue
        out.append((m.lastgroup, m.group(m.lastgroup), m.start()))
    out.append(("eof", "", len(s)))
    return out

class Parser:
    def __init__(self, s):
        self.s = s; self.t = lex(s); self.i = 0
        self.feat = collections.Counter()
        self.paren_depth = 0
    def peek(self, k=0): return self.t[self.i + k]
    def take(self, kind=None, val=None):
        tok = self.t[self.i]
        if (kind and tok[0] != kind) or (val is not None and tok[1] != val):
            raise ParseError(f"expected {kind or ''} {val or ''} at {tok[2]}, got {tok[1]!r} in {self.s!r}")
        self.i += 1; return tok
    def at(self, kind, val=None):
        tok = self.t[self.i]
        return tok[0] == kind and (val is None or tok[1] == val)

    def predicate(self):
        node = self.or_expr()
        if not self.at("eof"):
            raise ParseError(f"trailing input at {self.peek()[2]}: {self.peek()[1]!r} in {self.s!r}")
        return node
    def or_expr(self):
        left = self.and_expr(); n = 0
        while self.at("ident", "or"):
            self.take(); right = self.and_expr(); left = ("or", left, right); n += 1
        if n: self.feat["or"] += n
        return left
    def and_expr(self):
        left = self.not_expr(); n = 0
        while self.at("ident", "and"):
            self.take(); right = self.not_expr(); left = ("and", left, right); n += 1
        if n: self.feat["and"] += n
        return left
    def not_expr(self):
        if self.at("ident", "not"):
            self.take(); self.feat["not"] += 1; return ("not", self.not_expr())
        return self.comparison()
    def comparison(self):
        # membership form: <string-literal> in <set-valued field>  (ADR-0050 / ADR-0029 `'text' in change`)
        if self.at("str") and self.peek(1)[0] == "ident" and self.peek(1)[1] == "in":
            lit = self.take()[1]; self.take("ident", "in"); field = self.operand()
            self.feat["in_membership"] += 1; self.feat["quoted"] += 1
            return ("member", ("lit", lit), field)
        if self.at("punct", "("):
            self.take(); self.paren_depth += 1; self.feat["parens"] += 1
            inner = self.or_expr(); self.take("punct", ")"); self.paren_depth -= 1
            left = ("group", inner)
        else:
            left = self.operand()
        if self.at("op"):
            op = self.take()[1]; right = self.value()
            if self.at("op"):
                raise ParseError(f"comparison chain at {self.peek()[2]} in {self.s!r}")
            return ("cmp", op, left, right)
        if self.at("ident", "in"):
            self.take(); lst = self.list(); self.feat["in"] += 1
            return ("in", left, lst)
        return left
    def operand(self):
        if self.at("ref"):
            tok = self.take(); self.feat["ref"] += 1
            if tok[1].endswith("?}"): self.feat["ref_optional"] += 1
            node = ("ref", tok[1])
        elif self.at("ident") and self.peek()[1] not in KEYWORDS - {"len", "any", "all", "none"}:
            parts = [self.take("ident")[1]]
            while self.at("punct", ".") and self.peek(1)[0] == "ident" and self.peek(1)[1] not in {"len", "any", "all", "none"}:
                self.take(); parts.append(self.take("ident")[1])
            node = ("path", ".".join(parts))
            if len(parts) == 1 and parts[0] in ("true", "false", "null"):
                node = ("lit", parts[0])
        elif self.at("ident") and self.peek()[1] in ("len", "any", "all", "none"):
            # prefix forms: len(x), any(x in list, body)
            name = self.take()[1]; self.take("punct", "(")
            if name == "len":
                inner = self.operand(); self.take("punct", ")"); self.feat["len"] += 1
                return ("len", inner)
            var = self.take("ident")[1]; self.take("ident", "in"); src = self.operand(); self.take("punct", ",")
            body = self.or_expr(); self.take("punct", ")"); self.feat[f"quant_{name}"] += 1; self.feat["quant_prefix"] += 1
            return ("quant", name, var, src, body)
        else:
            raise ParseError(f"operand expected at {self.peek()[2]}: {self.peek()[1]!r} in {self.s!r}")
        # method calls: .len() / .any(body)
        while self.at("punct", ".") and self.peek(1)[0] == "ident" and self.peek(1)[1] in ("len", "any", "all", "none"):
            self.take(); name = self.take()[1]; self.take("punct", "(")
            if name == "len":
                self.take("punct", ")"); self.feat["len"] += 1; node = ("len", node)
            else:
                body = self.or_expr(); self.take("punct", ")"); self.feat[f"quant_{name}"] += 1; self.feat["quant_method"] += 1
                node = ("quant", name, None, node, body)
        return node
    def value(self):
        if self.at("ref"):
            tok = self.take(); self.feat["ref"] += 1; node = ("ref", tok[1])
            while self.at("punct", ".") and self.peek(1)[1] == "len":
                self.take(); self.take(); self.take("punct", "("); self.take("punct", ")"); self.feat["len"] += 1; node = ("len", node)
            return node
        if self.at("str"):
            self.feat["quoted"] += 1; return ("lit", self.take()[1])
        if self.at("num"):
            return ("lit", self.take()[1])
        if self.at("ident"):
            v = self.take()[1]
            if v in ("true", "false", "null"):
                if v == "null": self.feat["null"] += 1
                return ("lit", v)
            if v in KEYWORDS:
                raise ParseError(f"keyword {v!r} in value position in {self.s!r}")
            self.feat["bare_literal"] += 1  # the literal-versus-field rule (ADR-0103 clause 2)
            return ("lit", v)
        if self.at("punct", "["):
            return self.list()
        raise ParseError(f"value expected at {self.peek()[2]}: {self.peek()[1]!r} in {self.s!r}")
    def list(self):
        self.take("punct", "["); items = []
        while not self.at("punct", "]"):
            items.append(self.value())
            if self.at("punct", ","): self.take()
        self.take("punct", "]"); return ("list", items)

def mixed_unparenthesised(node, depth=0):
    """True if an `and` sits directly under an `or` (or vice versa) without a group node between them."""
    if not isinstance(node, tuple): return False
    kind = node[0]
    if kind in ("and", "or"):
        for child in node[1:]:
            if isinstance(child, tuple) and child[0] in ("and", "or") and child[0] != kind:
                return True
            if mixed_unparenthesised(child): return True
    elif kind == "group":
        return mixed_unparenthesised(node[1])
    elif kind == "not":
        return mixed_unparenthesised(node[1])
    return False

ON_FALSE_OK = re.compile(r"^(continue|fail)$")

def check_on_false(v):
    if isinstance(v, str):
        return bool(ON_FALSE_OK.match(v)), "keyword"
    if isinstance(v, dict):
        if v.get("to") == "deviate":
            return True, "deviate"
        if set(v.keys()) == {"to", "max"} and isinstance(v["to"], str) and isinstance(v["max"], int):
            return True, "loop-back"
        return False, f"bad table {sorted(v.keys())}"
    return False, f"bad type {type(v).__name__}"

def collect(root):
    found = []
    def walk(o, f, path):
        if isinstance(o, dict):
            for k, v in o.items():
                if k in ("applicable_if", "when"): found.append((k, v, f, path + "/" + k))
                if k == "on_false": found.append((k, v, f, path + "/" + k))
                walk(v, f, path + "/" + k)
        elif isinstance(o, list):
            for i, x in enumerate(o): walk(x, f, f"{path}[{i}]")
    for f in sorted(glob.glob(os.path.join(root, "**", "*.toml"), recursive=True)):
        walk(tomllib.load(open(f, "rb")), os.path.relpath(f, root).replace(os.sep, "/"), "")
    return found

def main():
    root = os.path.dirname(os.path.abspath(__file__))
    found = collect(root)
    by_kind = collections.Counter(k for k, *_ in found)
    feat = collections.Counter(); fails = []; mixed = []; ok = 0
    of_ok = 0; of_fail = []; of_forms = collections.Counter()
    for k, v, f, path in found:
        if k == "on_false":
            good, form = check_on_false(v); of_forms[form] += 1
            if good: of_ok += 1
            else: of_fail.append((f, path, v, form))
            continue
        try:
            p = Parser(str(v)); tree = p.predicate(); ok += 1; feat.update(p.feat)
            if mixed_unparenthesised(tree): mixed.append((f, path, v))
        except ParseError as e:
            fails.append((f, path, v, str(e)))
    n_pred = by_kind["applicable_if"] + by_kind["when"]
    print(f"S-GRAMMAR over spikes/p9 — {len(found)} strings: applicable_if {by_kind['applicable_if']}, when {by_kind['when']}, on_false {by_kind['on_false']}")
    print(f"predicates parsed {ok}/{n_pred}; failed {len(fails)}/{n_pred}")
    for f, path, v, e in fails: print(f"  FAIL {f}{path}: {v!r} -> {e}")
    print(f"mixed unparenthesised and/or (precedence-dependent) {len(mixed)}/{n_pred}")
    for f, path, v in mixed: print(f"  MIXED {f}{path}: {v!r}")
    print("feature census over parsed predicates:", dict(sorted(feat.items())))
    print(f"on_false forms {of_ok}/{by_kind['on_false']} in the closed set: {dict(of_forms)}")
    for f, path, v, form in of_fail: print(f"  ON_FALSE-FAIL {f}{path}: {v!r} ({form})")
    json.dump({"strings": len(found), "by_kind": dict(by_kind), "parsed": ok, "failed": [list(x) for x in fails],
               "mixed_unparenthesised": [list(x) for x in mixed], "features": dict(feat),
               "on_false_ok": of_ok, "on_false_forms": dict(of_forms), "on_false_failed": [list(map(str, x)) for x in of_fail]},
              # newline="\n" (2026-09-25, s8a fix round 5): a text-mode write on Windows otherwise turns every
              # terminator into CRLF, and the tracked file shows w/crlf on an unpinned path (ADR-0164 decision 6).
              open(os.path.join(root, "results-s-grammar.json"), "w", encoding="utf-8", newline="\n"), indent=1)
    return 0 if not fails and not of_fail else 1

if __name__ == "__main__":
    sys.exit(main())
