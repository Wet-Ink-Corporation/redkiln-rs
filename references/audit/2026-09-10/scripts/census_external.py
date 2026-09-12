#!/usr/bin/env python
# census_external.py -- builds the EXTERNAL CLAIM WORK-LIST for the redkiln-rs
# pre-implementation audit.  READ-ONLY against the repo; writes one JSON file.
#
# Every number in external.json is produced here.  Re-run to regenerate.
# ASCII-only stdout (Windows console cannot print section-sign / em-dash).
import json
import os
import re
import sys
from collections import OrderedDict, defaultdict

ROOT = "D:/repos/redkiln-rs"
OUT = "D:/repos/redkiln-rs/references/audit/2026-09-10/census/external.json"

EXCLUDE_PREFIXES = ("spikes/fixtures/", "spikes/runner/target", ".git/", ".idea/", ".junie/")

TAG_TOKENS = ("1P", "peer", "preprint", "vendor", "blog", "Pr")
TAG_RE = re.compile(r"\[(1P(?:/blog)?|peer|preprint|vendor|blog|Pr)\]")
UNVERIFIED_RE = re.compile(
    r"UNVERIFIED|not read this pass|URLs? not returned|not verified|never (been )?checked"
    r"|not fetched|search-only|via search only|could not (be )?fetch|not re-verified"
    r"|unverified provenance|returned (only )?page titles|undecodable|truncated on fetch"
    r"|404'd|not checked", re.I)
FETCH_DATE_RE = re.compile(
    r"(?:fetched|read|as of|checked|snapshot|API read[s]?|dated|verified)[^.|]{0,40}?(20\d\d-\d\d-\d\d)", re.I)
ANY_DATE_RE = re.compile(r"(20\d\d-\d\d-\d\d)")
DECIDED_RE = re.compile(r"\*\*DECIDED\*\*")

URL_RE = re.compile(r"https?://[A-Za-z0-9._~:/?#@!$&'()*+,;=%\-]+")
ARXIV_RE = re.compile(r"(?:arXiv[:\s]*|arxiv\.org/(?:abs|html|pdf)/)(\d{4}\.\d{4,5})(?:v\d+)?", re.I)
ADV_RE = re.compile(r"(RUSTSEC-\d{4}-\d{4}|CVE-\d{4}-\d{4,7}|GHSA-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4})")
# bare "-0351, -0353" continuation form used in the arch-design register
ADV_CONT_RE = re.compile(r"RUSTSEC-(\d{4})-(\d{4})((?:\s*/\s*-?\d{4}|,\s*-\d{4})+)")
DOI_RE = re.compile(r"\b(10\.\d{4,9}/[-._;()/:A-Za-z0-9]+)")
PII_RE = re.compile(r"\b(S\d{16})\b")
STARS_RE = re.compile(r"(~?[\d,]+(?:\.\d+)?K?)\s*(?:\u2605|stars)", re.I)


def rel(path):
    return os.path.relpath(path, ROOT).replace("\\", "/")


def in_scope(relpath):
    return not any(relpath.startswith(p) for p in EXCLUDE_PREFIXES)


def corpus_files():
    out = []
    for dp, dns, fns in os.walk(ROOT):
        r = rel(dp)
        if r == ".":
            r = ""
        else:
            r += "/"
        if not in_scope(r):
            dns[:] = []
            continue
        for fn in fns:
            if fn.endswith(".md"):
                p = r + fn
                if in_scope(p):
                    out.append(p)
    return sorted(out)


def clean(s):
    return re.sub(r"\s+", " ", s).strip()


def excerpt(line, m_start, m_end, width=400):
    """Verbatim window around a match, preferring the table cell / sentence."""
    if "|" in line:
        cells = line.split("|")
        pos = 0
        for c in cells:
            if pos <= m_start < pos + len(c) + 1:
                cell = clean(c)
                if 0 < len(cell) <= width:
                    return cell
                break
            pos += len(c) + 1
    lo = max(0, m_start - 180)
    hi = min(len(line), m_end + 180)
    s = clean(line[lo:hi])
    if lo > 0:
        s = "..." + s
    if hi < len(line):
        s = s + "..."
    return s[:width]


def strip_url(u):
    u = u.rstrip(").,;:'\"*]>")
    u = re.sub(r"\*+$", "", u)
    if u.endswith("/") and u.count("/") > 3:
        u = u[:-1]
    return u


def host_of(u):
    m = re.match(r"https?://([^/]+)", u)
    return m.group(1).lower() if m else ""


# ----------------------------------------------------------------- families
# URL -> (family, verify_method) by host/path rules, first match wins.
URL_RULES = [
    (r"arxiv\.org", "preprints", "arxiv_abs"),
    (r"rustsec\.org|github\.com/advisories|rustsec/advisory-db|osv\.dev", "advisories", "rustsec_osv"),
    (r"crates\.io/crates/(rusqlite)|docs\.rs/rusqlite|sqlite\.org", "sqlite", None),
    (r"crates\.io/crates/(gix|git2)|docs\.rs/gix|GitoxideLabs|libgit2|git-scm\.com|git-branchless", "gitoxide", None),
    (r"crates\.io/crates/(temporalio|graph-flow)|temporalio|iopsystems|docs\.temporal\.io|temporal\.io"
     r"|durable-execution|step-functions|exponential-backoff-and-jitter|restate\.dev|trigger\.dev"
     r"|inngest\.com|dbos\.dev|docs\.github\.com/[^ ]*(deployments|environments)|langchain|langgraph"
     r"|learn\.microsoft\.com/[^ ]*durable", "durable_execution", None),
    (r"crates\.io|docs\.rs|lib\.rs", "crate_versions", None),
    (r"doc\.rust-lang\.org|rust-lang\.github\.io|github\.com/rust-lang|microsoft\.github\.io/rust"
     r"|matklad\.github\.io|swatinem\.de|nickb\.dev|debasishg\.github\.io|oneuptime\.com|rustfaq"
     r"|internals\.rust-lang\.org|axodotdev\.github\.io|releases\.rs|blog\.rust-lang\.org", "toolchain", None),
    (r"code\.claude\.com|github\.com/anthropics/claude-code|claude\.com/blog|claude\.ai", "claude_code", None),
    (r"platform\.claude\.com|docs\.anthropic\.com|anthropic\.com|github\.com/anthropics", "anthropic_platform", None),
    (r"github\.com/adr/|adr\.github\.io|npryce/adr-tools|backstage\.io/docs/architecture-decisions"
     r"|log4brains|architecture-decision", "decision_records", None),
    (r"doi\.org|springer|sciencedirect|dl\.acm\.org|ieeexplore|ieee\.org|wiley|aclanthology|ceur-ws"
     r"|researchgate|iso\.org|omg\.org|computer\.org|istqb|nist\.gov|cs\.ubc\.ca|manchester\.ac\.uk"
     r"|semanticscholar|huggingface\.co/papers|openreview|usenix\.org|ncbi\.nlm|jucs\.org|theiet\.org"
     r"|scholar\.|ptgmedia\.pearsoncmg|uni-ulm\.de|kodu\.ut\.ee|sei\.cmu\.edu|w3\.org|ietf\.org|rfc-editor"
     r"|ivarjacobson|essence|swebok|pmi\.org|iiba\.org|isaca", "peer_and_standards", None),
    (r"github\.com|gitlab\.com|kiro\.dev|cursor\.com|factory\.ai|atlassian|linear\.app|cloudflare|mergify"
     r"|beam\.ai|mintmcp|cloudnuro|vibekanban|buildermethods|cline\.bot|openai\.com|community\.openai"
     r"|deepwiki|docs\.dagster|dagster\.io|sudoall|augmentcode|simspace|speakeasy|humanlayer|devin\.ai"
     r"|cognition|ampcode|sourcegraph|jetbrains|copilot|aider|continue\.dev|servicenow|shortcut\.com"
     r"|github\.blog|docs\.github\.com|tekai|dev\.to|medium\.com|mintlify|mindstudio|nesbitt\.io"
     r"|gerrit-review|jj-vcs|kilocode|kilo\.ai", "competitors", None),
]
METHOD_BY_HOST = [
    (r"crates\.io", "crates_io_api"),
    (r"docs\.rs", "docs_rs"),
    (r"rustsec|advisories|osv\.dev", "rustsec_osv"),
    (r"arxiv\.org", "arxiv_abs"),
    (r"doi\.org|springer|sciencedirect|dl\.acm\.org|ieee|wiley|aclanthology|ceur-ws|huggingface\.co/papers"
     r"|semanticscholar|openreview|ncbi\.nlm|jucs\.org|theiet\.org", "doi_resolve"),
    (r"github\.com/[^/]+/[^/]+(/(issues|discussions|pull|releases|blob|tree))?", "github_api"),
    (r"raw\.githubusercontent\.com|github\.blog|docs\.github\.com", "vendor_docs"),
    (r"^https?://(docs\.|platform\.|code\.|learn\.|developer\.|developers\.|support\.|help\.)"
     r"|/docs/|/documentation/|doc\.rust-lang\.org|rust-lang\.github\.io|kiro\.dev|cursor\.com|restate\.dev"
     r"|trigger\.dev|inngest\.com|dbos\.dev|temporal\.io|aws\.amazon\.com|azure|microsoft\.com|atlassian"
     r"|linear\.app|gitlab\.com|servicenow|backstage\.io|adr\.github\.io|omg\.org|iso\.org|nist\.gov"
     r"|sre\.google|dora\.dev|pagerduty|axodotdev|kubernetes|w3\.org|owasp|cloudflare\.com|mergify\.com"
     r"|factory\.ai|cline\.bot|openai\.com|anthropic\.com|claude\.com|wikipedia", "vendor_docs"),
]
VALID_METHODS = {"crates_io_api", "docs_rs", "rustsec_osv", "vendor_docs", "arxiv_abs",
                 "doi_resolve", "github_api", "web_search"}


def classify_url(u):
    fam, meth = "process_literature", None
    for pat, f, m in URL_RULES:
        if re.search(pat, u, re.I):
            fam, meth = f, m
            break
    if meth is None:
        meth = "web_search"
        for pat, m in METHOD_BY_HOST:
            if re.search(pat, u, re.I):
                meth = m
                break
    return fam, meth


# ----------------------------------------------------------------- curated claims
# (key, family, regex, cited_source, verify_url, verify_method, extra)
# Every pattern is asserted to match >0 lines; a zero is reported, never dropped.
CRATES = OrderedDict([
    ("serde", "https://crates.io/api/v1/crates/serde"),
    ("serde_json", "https://crates.io/api/v1/crates/serde_json"),
    ("thiserror", "https://crates.io/api/v1/crates/thiserror"),
    ("anyhow", "https://crates.io/api/v1/crates/anyhow"),
    ("clap", "https://crates.io/api/v1/crates/clap"),
    ("tokio", "https://crates.io/api/v1/crates/tokio"),
    ("minijinja", "https://crates.io/api/v1/crates/minijinja"),
    ("toml", "https://crates.io/api/v1/crates/toml"),
    ("sha2", "https://crates.io/api/v1/crates/sha2"),
    ("chrono", "https://crates.io/api/v1/crates/chrono"),
    ("jsonschema", "https://crates.io/api/v1/crates/jsonschema"),
    ("schemars", "https://crates.io/api/v1/crates/schemars"),
    ("rusqlite", "https://crates.io/api/v1/crates/rusqlite"),
    ("gix", "https://crates.io/api/v1/crates/gix"),
    ("insta", "https://crates.io/api/v1/crates/insta"),
    ("trycmd", "https://crates.io/api/v1/crates/trycmd"),
    ("assert_cmd", "https://crates.io/api/v1/crates/assert_cmd"),
    ("cargo-audit", "https://crates.io/api/v1/crates/cargo-audit"),
    ("cargo-semver-checks", "https://crates.io/api/v1/crates/cargo-semver-checks"),
    ("git-cliff", "https://crates.io/api/v1/crates/git-cliff"),
    ("release-plz", "https://crates.io/api/v1/crates/release-plz"),
    ("cargo-dist", "https://crates.io/api/v1/crates/cargo-dist"),
    ("async-trait", "https://crates.io/api/v1/crates/async-trait"),
    ("temporalio-sdk", "https://crates.io/api/v1/crates/temporalio-sdk"),
    ("temporalio-sdk-core", "https://crates.io/api/v1/crates/temporalio-sdk-core"),
    ("graph-flow", "https://crates.io/api/v1/crates/graph-flow"),
])
SEMVER_NEAR = r"[^\n]{0,90}?((?<=v)\d+\.\d+(?:\.\d+)?|\d+\.\d+\.\d+)"
SEMVER_ALL = re.compile(r"(?<![\d.])((?<=v)\d+\.\d+(?:\.\d+)?|\d+\.\d+\.\d+)(?![\d.])")

CURATED = []


def cur(key, family, pattern, cited_source, verify_url, verify_method, tag="none", note=None,
        force_lb=False, force_unv=False, cs=False):
    CURATED.append(dict(key=key, family=family, pattern=re.compile(pattern, 0 if cs else re.I),
                        cited_source=cited_source, verify_url=verify_url,
                        verify_method=verify_method, tag=tag, note=note,
                        force_lb=force_lb, force_unv=force_unv))


NOVERSION = {"cargo-audit", "async-trait"}
for c, url in CRATES.items():
    fam = "crate_versions"
    esc = re.escape(c)
    pat = r"(?<![A-Za-z0-9_./-])`?" + esc + r"`?(?![A-Za-z0-9_-])" + ("" if c in NOVERSION else SEMVER_NEAR)
    cur("crate:" + c, fam, pat,
        "crates.io / " + c, url, "crates_io_api", "1P",
        note="adr/0084:104 forbids citing that record's version list as current after 2026-09-08")

# toolchain
cur("toolchain:rustc-1.97.1", "toolchain", r"1\.97\.1", "Rust release 1.97.1",
    "https://github.com/rust-lang/rust/blob/master/RELEASES.md", "github_api", "1P")
cur("toolchain:rust-1.97-release-notes", "toolchain", r"\b1\.97\b(?!\.1)", "Rust 1.97 release notes",
    "https://blog.rust-lang.org/releases/", "vendor_docs", "1P",
    note="corpus records the 1.97 release notes as unread (search-only)", force_unv=True)
cur("toolchain:edition-2024", "toolchain", r"edition\W{0,4}2024", "Rust Edition Guide, 2024 edition",
    "https://doc.rust-lang.org/edition-guide/rust-2024/", "vendor_docs", "1P")
for g in ("correctness", "suspicious", "style", "complexity", "perf", "pedantic", "nursery", "cargo", "restriction", "all"):
    cur("toolchain:clippy-group-" + g, "toolchain", r"clippy::" + g + r"\b",
        "Clippy Lints book, group " + g, "https://doc.rust-lang.org/stable/clippy/lints.html", "vendor_docs", "1P")
cur("toolchain:dyn-async-fn-in-trait", "toolchain", r"async_fn_in_dyn_trait|dyn-compatib|R-ASYNC",
    "rust-lang async-fundamentals-initiative, async fn in dyn trait",
    "https://rust-lang.github.io/async-fundamentals-initiative/explainer/async_fn_in_dyn_trait.html",
    "vendor_docs", "1P", note="R-ASYNC; stabilisation status search-only per evidence-augmentation s5")
cur("toolchain:workspace-lints", "toolchain", r"workspace\.lints|cargo\W{0,5}#?13157",
    "Cargo Book, Lints; rust-lang/cargo #13157", "https://doc.rust-lang.org/cargo/reference/lints.html", "vendor_docs", "1P")
cur("toolchain:rfc-3537-msrv", "toolchain", r"RFC\W?3537|msrv-resolver|MSRV-aware",
    "RFC 3537 MSRV-aware resolver", "https://rust-lang.github.io/rfcs/3537-msrv-resolver.html", "vendor_docs", "1P")
cur("toolchain:set-var-newly-unsafe", "toolchain", r"set_var|remove_var|newly-unsafe|PR\W?#?124636",
    "Rust Edition Guide, newly unsafe functions; rust-lang/rust #124636",
    "https://doc.rust-lang.org/edition-guide/rust-2024/newly-unsafe-functions.html", "vendor_docs", "1P")
cur("toolchain:api-guidelines", "toolchain", r"Rust API Guidelines|api-guidelines",
    "Rust API Guidelines", "https://rust-lang.github.io/api-guidelines/", "vendor_docs", "1P")
cur("toolchain:ms-pragmatic-guidelines", "toolchain", r"Pragmatic Rust Guidelines|microsoft\.github\.io/rust",
    "Microsoft Pragmatic Rust Guidelines", "https://microsoft.github.io/rust-guidelines/", "vendor_docs", "1P")
cur("toolchain:thiserror-anyhow-split-blogs", "toolchain", r"anyhow.{0,80}thiserror|thiserror.{0,80}anyhow",
    "OneUptime 2026-01-25 and Pi Stack 2026-06-22 practitioner blogs (the split rests on no first-party source)",
    "https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view", "web_search", "blog")
cur("toolchain:integration-test-binary-cost", "toolchain", r"matklad|Delete Cargo Integration Tests|integration_testing",
    "matklad, Delete Cargo Integration Tests; Rust by Example integration testing",
    "https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html", "web_search", "blog")
cur("toolchain:feature-unification", "toolchain", r"feature[- ]unification",
    "nickb.dev, Cargo Workspace and the Feature Unification Pitfall",
    "https://nickb.dev/blog/cargo-workspace-and-the-feature-unification-pitfall/", "web_search", "blog")
cur("toolchain:cargo-dist-rename", "toolchain", r"rebranded to `?dist`?|crate named `?dist`?|cargo-dist.{0,60}dist\b",
    "cargo-dist install docs (CLI rebranded to dist at v0.24.0; crate still cargo-dist)",
    "https://axodotdev.github.io/cargo-dist/book/install.html", "vendor_docs", "1P")

# claude_code
CC_FLAGS = ["--agent", "--agents", "--strict-mcp-config", "--json-schema", "--system-prompt",
            "--append-system-prompt", "--exclude-dynamic-system-prompt-sections", "--bare",
            "--permission-prompts", "--effort", "--output-format", "--allowed-tools", "--model", "--resume"]
for f in CC_FLAGS:
    pat = re.escape(f) + (r"(?![a-z-])" if f in ("--agent", "--system-prompt") else "")
    cur("claude_code:flag:" + f, "claude_code", pat, "Claude Code CLI reference, flag " + f,
        "https://code.claude.com/docs/en/cli-reference", "vendor_docs", "1P")
cur("claude_code:StructuredOutput-tool", "claude_code", r"StructuredOutput",
    "Claude Code --json-schema / StructuredOutput tool behaviour",
    "https://code.claude.com/docs/en/cli-reference", "vendor_docs", "1P")
cur("claude_code:reasoning-effort", "claude_code", r"reasoning effort|effort level|MAX_THINKING_TOKENS|thinking budget",
    "Claude Code reasoning-effort setting", "https://code.claude.com/docs/en/settings", "vendor_docs", "1P")
cur("claude_code:rate_limit_event", "claude_code", r"rate_limit_event",
    "Claude Code stream-json rate_limit_event", "https://code.claude.com/docs/en/headless", "vendor_docs", "1P")
cur("claude_code:subscription-only-auth", "claude_code", r"subscription[- ]only",
    "Claude Code subscription-only authentication (ADR-0010)", "https://code.claude.com/docs/en/authentication",
    "vendor_docs", "1P")
cur("claude_code:sigterm-143", "claude_code", r"SIGTERM.{0,30}143|exit (code )?143",
    "Claude Code SIGTERM -> exit 143 with no result message", "https://code.claude.com/docs/en/headless",
    "vendor_docs", "1P")
cur("claude_code:tool_deferred-hooks", "claude_code", r"tool_deferred|defer permission",
    "Claude Code hooks doc / changelog: April-2026 tool_deferred permission decision",
    "https://code.claude.com/docs/en/hooks", "vendor_docs", "1P", force_unv=True,
    note="evidence-augmentation s5: changelogs not checked; first-party hooks doc shows no trace")
cur("claude_code:cache-ttl-env-vars", "claude_code", r"CLAUDE_CODE_(SUBAGENT_)?PROMPT_CACHE_TTL",
    "Claude Code prompt-caching env vars CLAUDE_CODE_PROMPT_CACHE_TTL / CLAUDE_CODE_SUBAGENT_PROMPT_CACHE_TTL",
    "https://code.claude.com/docs/en/prompt-caching", "vendor_docs", "1P")
cur("claude_code:subagents-doc", "claude_code", r"sub-agents|code\.claude\.com/docs/en/sub-agents|Subagents doc",
    "Claude Code subagents documentation", "https://code.claude.com/docs/en/sub-agents", "vendor_docs", "1P")
cur("claude_code:context-window-compaction", "claude_code", r"/compact\b|compaction guidance|context-window",
    "Claude Code context window / compaction guidance", "https://code.claude.com/docs/en/context-window",
    "vendor_docs", "1P")
cur("claude_code:compact-98pct-figure", "claude_code", r"98%[- ]reduction|132k|2\.3k[- ]token",
    "secondary blog 98%-reduction / 132k->2.3k /compact figure (corpus: do not cite)",
    None, "web_search", "blog", force_unv=True)
cur("claude_code:version-2.1.242+", "claude_code", r"v?2\.1\.242\+?", "Claude Code v2.1.242+ requirement (cache TTL env vars)",
    "https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md", "github_api", "1P")
for v in ("2.1.251", "2.1.258", "2.1.259", "2.1.263", "2.1.265"):
    cur("claude_code:version-" + v, "claude_code", r"v?" + re.escape(v) + r"\+?",
        "Claude Code version " + v + " (behaviour observed / required at this version)",
        "https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md", "github_api", "1P")

# anthropic_platform
cur("anthropic_platform:two-bucket-ttl", "anthropic_platform",
    r"two-bucket|two bucket|5 m regardless|5m regardless|regardless of billing",
    "code.claude.com prompt-caching: two-bucket TTL (main conversation vs subagents/workflows/forks/compaction)",
    "https://code.claude.com/docs/en/prompt-caching", "vendor_docs", "1P", force_lb=True)
cur("anthropic_platform:precedence-chain", "anthropic_platform", r"precedence chain|layered precedence",
    "code.claude.com prompt-caching: TTL precedence chain", "https://code.claude.com/docs/en/prompt-caching",
    "vendor_docs", "1P")
cur("anthropic_platform:ephemeral-usage-fields", "anthropic_platform", r"ephemeral_1h|ephemeral_5m",
    "platform.claude.com prompt caching: ephemeral_1h_* / ephemeral_5m_* usage fields (S-TTL1 contradiction)",
    "https://platform.claude.com/docs/en/build-with-claude/prompt-caching", "vendor_docs", "1P", force_lb=True)
cur("anthropic_platform:s-ttl1-contradiction", "anthropic_platform", r"S-TTL1",
    "S-TTL1: the opposite-signed TTL observation (in-session subagents on 1 h, not 5 m)",
    "https://code.claude.com/docs/en/prompt-caching", "vendor_docs", "1P", force_lb=True)
cur("anthropic_platform:cache-scope", "anthropic_platform",
    r"(workspace|org|organi[sz]ation)\W{0,4}scoped[^.|]{0,60}(cache|prompt)|(cache|prompt)[^.|]{0,60}(workspace|org|organi[sz]ation)\W{0,4}scoped|machine\+directory|machine-and-directory",
    "platform.claude.com prompt caching: cache scope (workspace vs org vs machine+directory)",
    "https://platform.claude.com/docs/en/build-with-claude/prompt-caching", "vendor_docs", "1P")
cur("anthropic_platform:20-position-lookback", "anthropic_platform",
    r"20-position|20 position|twenty.{0,12}position|20 (cache )?blocks|lookback|looks? back(ward)? (up to )?20",
    "platform.claude.com prompt caching: 20-block lookback for cache hits",
    "https://platform.claude.com/docs/en/build-with-claude/prompt-caching", "vendor_docs", "1P")
cur("anthropic_platform:fan-out-stagger-5s", "anthropic_platform",
    r"(5|five)[- ]?s(econd)?[^.|]{0,30}stagger|stagger[^.|]{0,40}(5|five)[- ]?s(econd)?\b",
    "prompt-caching guidance: 5-second stagger for parallel fan-out so the first call warms the cache",
    "https://platform.claude.com/docs/en/build-with-claude/prompt-caching", "vendor_docs", "1P")
cur("anthropic_platform:subagent-cache-isolation", "anthropic_platform",
    r"subagent[^.|]{0,40}isolat|isolat[^.|]{0,40}subagent|subagents-and-the-cache",
    "code.claude.com prompt-caching: subagents-and-the-cache section",
    "https://code.claude.com/docs/en/prompt-caching", "vendor_docs", "1P")
cur("anthropic_platform:structured-output-limits", "anthropic_platform",
    r"additionalProperties|\u226424 optional|24 optional|16 union|limits table|structured-output|structured output",
    "platform.claude.com structured outputs (truncated on fetch); anthropic-sdk-python #1185",
    "https://platform.claude.com/docs/en/build-with-claude/structured-outputs", "vendor_docs", "1P", force_unv=True)

# gitoxide
cur("gitoxide:common-dir-interplay-quote", "gitoxide", r"GIT_COMMON_DIR",
    "gitoxide crate-status.md: 'the delicate interplay between GIT_COMMON_DIR and GIT_WORK_TREE isn't implemented'",
    "https://github.com/GitoxideLabs/gitoxide/blob/main/crate-status.md", "github_api", "1P", force_lb=True)
cur("gitoxide:trailer-write-absence", "gitoxide", r"gix[^.|]{0,80}trailer[^.|]{0,25}writ|trailer[^.|]{0,25}writ[^.|]{0,60}(API|gix)|first-class trailer|trailer-writ[a-z]* (API|support)|S-GIX",
    "gix docs.rs: no first-class trailer-WRITING API found (UNVERIFIED absence; 'highest-value single check')",
    "https://docs.rs/gix/0.87.1/gix/", "docs_rs", "1P", force_lb=True, force_unv=True)
cur("gitoxide:shallow-clone-by-sha", "gitoxide", r"shallow[- ]clone[^.|]{0,60}(SHA|commit)|discussion\W{0,5}#?2309",
    "gitoxide discussion #2309: shallow clone at a specific commit",
    "https://github.com/GitoxideLabs/gitoxide/discussions/2309", "github_api", "1P")
cur("gitoxide:git2-libgit2-comparison", "gitoxide", r"libgit2|\bgit2\b",
    "git2 / libgit2 comparison against gix", "https://crates.io/api/v1/crates/git2", "crates_io_api", "1P")
cur("gitoxide:crate-status-doc", "gitoxide", r"crate-status",
    "gitoxide crate-status.md (feature status document)",
    "https://github.com/GitoxideLabs/gitoxide/blob/main/crate-status.md", "github_api", "1P")
cur("gitoxide:gix-0.87.1-pre-1.0-multi-maintainer", "gitoxide", r"gix[^.|]{0,40}0\.87\.1|0\.87\.1[^.|]{0,40}gix",
    "crates.io gix 0.87.1, pre-1.0, facade over gix-* sub-crates, multi-maintainer GitoxideLabs",
    "https://crates.io/api/v1/crates/gix", "crates_io_api", "1P")
cur("gitoxide:git-branchless-1334", "gitoxide", r"branchless|#1334",
    "git-branchless issue #1334: reference-transaction hook not atomic",
    "https://github.com/arxanas/git-branchless/issues/1334", "github_api", "1P")

# sqlite
cur("sqlite:rusqlite-0.40.2", "sqlite", r"rusqlite[^.|]{0,40}0\.40\.2|0\.40\.2[^.|]{0,40}rusqlite",
    "crates.io rusqlite 0.40.2 (2026-08-08, MIT)", "https://crates.io/api/v1/crates/rusqlite", "crates_io_api", "1P")
cur("sqlite:wal-busy-timeout-semantics", "sqlite", r"busy_timeout",
    "SQLite WAL + busy_timeout multi-process semantics (read via search, not docs.rs/rusqlite; R-SQLITE)",
    "https://docs.rs/rusqlite/0.40.2/rusqlite/", "docs_rs", "1P", force_unv=True)
cur("sqlite:single-writer-serialisation", "sqlite",
    r"one writer|single writer|writer connection|serialis[a-z]* writ|writ[a-z]* serialis",
    "SQLite: one writer connection or queued write path, never a pool (ADR-0084)",
    "https://www.sqlite.org/wal.html", "vendor_docs", "1P")
cur("sqlite:r-sqlite-open-item", "sqlite", r"R-SQLITE",
    "R-SQLITE open item: docs.rs/rusqlite not read this pass",
    "https://docs.rs/rusqlite/0.40.2/rusqlite/", "docs_rs", "1P", force_unv=True)
cur("sqlite:rusqlite-rustsec-2021-0128", "sqlite", r"RUSTSEC-2021-0128",
    "RustSec RUSTSEC-2021-0128 (rusqlite)",
    "https://rustsec.org/advisories/RUSTSEC-2021-0128.html", "rustsec_osv", "1P")

# durable_execution
cur("durable:temporal-rust-sdk", "durable_execution", r"temporalio[-_]sdk|sdk-rust|temporalio/|Temporal Rust SDK|Temporal.{0,20}Rust",
    "temporalio-sdk on crates.io (temporalio-sdk-core v0.5.0, Public Preview June 2026); temporalio/sdk-rust",
    "https://crates.io/api/v1/crates/temporalio-sdk", "crates_io_api", "1P", force_lb=True)
cur("durable:temporal-workflow-definition", "durable_execution", r"workflow-definition|docs\.temporal\.io",
    "docs.temporal.io/workflow-definition determinism rules", "https://docs.temporal.io/workflow-definition",
    "vendor_docs", "1P")
cur("durable:temporal-time-skipping-test-server", "durable_execution", r"time-skipping|embedded/local-test|local-test mode",
    "Temporal SDK time-skipping test server / embedded mode (not checked)",
    "https://github.com/temporalio/sdk-rust", "github_api", "1P", force_unv=True)
cur("durable:graph-flow", "durable_execution", r"graph-flow",
    "graph-flow v0.6 on crates.io (postgres optional, in-memory session store)",
    "https://crates.io/api/v1/crates/graph-flow", "crates_io_api", "1P", force_lb=True)
cur("durable:iopsystems-durable", "durable_execution", r"iopsystems",
    "iopsystems/durable (Postgres and WASM-component compilation; ~58 stars; no release tags)",
    "https://api.github.com/repos/iopsystems/durable", "github_api", "1P", force_lb=True)
cur("durable:aws-durable-execution-sdk", "durable_execution", r"AWS Durable Execution|durable-execution",
    "AWS Durable Execution SDK determinism best practices (Lambda-backed, not Rust)",
    "https://docs.aws.amazon.com/durable-execution/patterns/best-practices/determinism/", "vendor_docs", "1P")
cur("durable:aws-backoff-and-jitter", "durable_execution", r"backoff[- ]and[- ]jitter|exponential-backoff|Equal Jitter",
    "AWS Architecture Blog, Exponential Backoff and Jitter ('base 2 s, cap 5 min' does not appear in it)",
    "https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/", "vendor_docs", "1P")
cur("durable:aws-step-functions", "durable_execution", r"Step Functions|SendTaskHeartbeat",
    "AWS Step Functions SendTaskHeartbeat / task tokens",
    "https://docs.aws.amazon.com/step-functions/latest/apireference/API_SendTaskHeartbeat.html", "vendor_docs", "1P")
cur("durable:restate", "durable_execution", r"\bRestate\b",
    "Restate external events / awakeables", "https://docs.restate.dev/develop/ts/external-events", "vendor_docs", "1P", cs=True)
cur("durable:trigger-dev", "durable_execution", r"Trigger\.dev",
    "Trigger.dev waitpoint tokens (URL not returned)", "https://trigger.dev/docs/wait-for-token", "vendor_docs", "1P")
cur("durable:inngest", "durable_execution", r"\bInngest\b",
    "Inngest step.waitForEvent() (URL not returned)",
    "https://www.inngest.com/docs/reference/functions/step-wait-for-event", "vendor_docs", "1P", cs=True)
cur("durable:dbos", "durable_execution", r"\bDBOS\b",
    "DBOS workflow communication (URL not returned)",
    "https://docs.dbos.dev/python/tutorials/workflow-communication", "vendor_docs", "1P", cs=True)
cur("durable:github-environments", "durable_execution", r"GitHub Environments|reviewing-deployments|deployments-and-environments|environment protection",
    "GitHub Docs: deployments and environments / reviewing deployments (URL not returned)",
    "https://docs.github.com/en/actions/managing-workflow-runs-and-deployments/managing-deployments/reviewing-deployments",
    "vendor_docs", "1P")
cur("durable:langgraph-checkpointing", "durable_execution", r"LangGraph",
    "LangGraph per-node checkpointing (read only via a secondary aggregator)",
    "https://docs.langchain.com/oss/python/langgraph/persistence", "vendor_docs", "1P", force_unv=True)
cur("durable:azure-durable-functions-events", "durable_execution", r"Azure Durable|Durable Functions",
    "Azure Durable Functions external-event token semantics (searches returned only tangential pages)",
    "https://learn.microsoft.com/en-us/azure/azure-functions/durable/durable-functions-external-events", "vendor_docs",
    "1P", force_unv=True)
cur("durable:no-framework-decision", "durable_execution", r"ADR-0086|durable-execution framework|No durable-execution framework",
    "ADR-0086 / DECIDED: no durable-execution framework (re-argued on facts that hold)",
    "https://crates.io/api/v1/crates/temporalio-sdk", "crates_io_api", "1P", force_lb=True)

# peer_and_standards
cur("peer:rank-reversal-S0895717711007850", "peer_and_standards", r"S0895717711007850|rank reversal",
    "ScienceDirect S0895717711007850, rank reversal in weighted MCDM methods",
    "https://www.sciencedirect.com/science/article/pii/S0895717711007850", "doi_resolve", "peer", force_lb=True)
cur("peer:ears-mavin-re09", "peer_and_standards", r"Mavin|\bEARS\b",
    "Mavin, Wilkinson, Harwood & Novak, EARS, IEEE RE'09 pp.317-322",
    "https://doi.org/10.1109/RE.2009.9", "doi_resolve", "peer", force_lb=True)
cur("peer:inozemtseva-holmes-icse2014", "peer_and_standards", r"Inozemtseva",
    "Inozemtseva & Holmes, Coverage Is Not Strongly Correlated with Test Suite Effectiveness, ICSE 2014",
    "https://doi.org/10.1145/2568225.2568271", "doi_resolve", "peer")
cur("peer:iso-12207", "peer_and_standards", r"12207", "ISO/IEC/IEEE 12207 software life cycle processes",
    "https://www.iso.org/standard/63712.html", "vendor_docs", "peer")
cur("peer:iso-29119", "peer_and_standards", r"29119", "ISO/IEC/IEEE 29119 software testing",
    "https://www.iso.org/standard/81291.html", "vendor_docs", "peer")
cur("peer:ieee-29148", "peer_and_standards", r"29148", "ISO/IEC/IEEE 29148 requirements engineering",
    "https://www.iso.org/standard/72089.html", "vendor_docs", "peer")
cur("peer:omg-essence", "peer_and_standards", r"OMG Essence|Essence (kernel|standard|Coach)|\bEssence\b",
    "OMG Essence (kernel and language for software engineering methods)",
    "https://www.omg.org/spec/Essence/", "vendor_docs", "peer", cs=True)
cur("peer:swebok", "peer_and_standards", r"SWEBOK", "IEEE SWEBOK guide",
    "https://www.computer.org/education/bodies-of-knowledge/software-engineering", "vendor_docs", "peer")
cur("peer:istqb", "peer_and_standards", r"ISTQB", "ISTQB syllabus / glossary",
    "https://www.istqb.org/", "vendor_docs", "peer")
cur("peer:nist-ssdf", "peer_and_standards", r"NIST SSDF|SSDF|SP 800-218", "NIST SSDF SP 800-218",
    "https://csrc.nist.gov/pubs/sp/800/218/final", "vendor_docs", "peer")
cur("peer:state-of-mutation-testing-google-2018", "peer_and_standards",
    r"State of Mutation Testing|one live mutant|70% of high-priority|Petrovi",
    "State of Mutation Testing at Google, ICSE-SEIP 2018 (named as an unverified provenance candidate only; R-MUT)",
    "https://doi.org/10.1145/3183519.3183521", "doi_resolve", "peer", force_unv=True)
cur("peer:opf-deontic-matrix-firesmith", "peer_and_standards", r"Firesmith|Henderson-Sellers|deontic matrix|\bOPF\b",
    "Firesmith & Henderson-Sellers, OPEN Process Framework deontic matrix (five levels not quoted verbatim)",
    "https://doi.org/10.1007/978-1-4471-0299-4", "doi_resolve", "peer", force_unv=True)
cur("peer:shop2-htn", "peer_and_standards", r"SHOP2|HTN planning",
    "SHOP2 ordered-method HTN planning (Nau et al., JAIR 2003)",
    "https://doi.org/10.1613/jair.1141", "doi_resolve", "peer")
cur("peer:agentless-vs-coder-cost", "peer_and_standards", r"Agentless|CodeR\b",
    "Agentless (FSE 2025, arXiv 2407.01489) vs CodeR cost/benchmark comparison ($0.70 at 32.00% vs $3.34 at 28.33%)",
    "https://arxiv.org/abs/2407.01489", "arxiv_abs", "peer")
cur("peer:meta-ach-fse2025", "peer_and_standards", r"Meta.{0,5}ACH|\bACH\b.{0,20}FSE",
    "Meta ACH, FSE 2025 / arXiv 2501.12862", "https://arxiv.org/abs/2501.12862", "arxiv_abs", "peer")
cur("peer:anthropic-code-review-price", "peer_and_standards", r"Anthropic Code Review|\$15.{0,3}25 per review",
    "Anthropic Code Review priced at $15-25 per review", None, "web_search", "vendor")

# competitors
COMPETITORS = [
    ("beads", r"\bBeads\b|steveyegge/beads|gastownhall/beads", "Beads (steveyegge -> gastownhall/beads), 26,983 stars per evidence-augmentation vs ~15.9K in landscape-sentiment (internal conflict)",
     "https://api.github.com/repos/gastownhall/beads", "github_api", True),
    ("gas-town", r"Gas Town|gastown", "Gas Town (Yegge), ~15.9K stars, v1.0 April / v1.2.1 June 2026", "https://api.github.com/repos/gastownhall/gastown", "github_api", False),
    ("archon", r"\bArchon\b", "Archon (coleam00/Archon) product characterisation and star count", "https://api.github.com/repos/coleam00/Archon", "github_api", False),
    ("factory", r"Factory (droids|release|AI|\.ai)|factory\.ai|\bDroids?\b", "Factory release notes / droids", "https://docs.factory.ai/changelog/release-notes", "vendor_docs", False),
    ("cursor", r"\bCursor\b", "Cursor headless CLI / auto-review classifier / SDK updates Jun 2026", "https://cursor.com/docs/cli/headless", "vendor_docs", False),
    ("cloudflare-review-figures", r"Cloudflare|131,246|48,095|break glass|ai-code-review", "Cloudflare, Orchestrating AI Code Review at scale: 131,246 runs / 48,095 MRs / $1.19 avg / 0.6% break-glass", "https://blog.cloudflare.com/ai-code-review/", "web_search", True),
    ("mergify", r"Mergify", "Mergify State of Merge Queues 2026: AI PRs broke main 1.9% vs 4.4%", "https://mergify.com/reports/state-of-merge-queues-2026", "vendor_docs", False),
    ("beam-ai", r"beam\.ai", "beam.ai multi-agent orchestration patterns 2026 (worker-count threshold, no data)", "https://beam.ai/agentic-insights/multi-agent-orchestration-patterns-production", "vendor_docs", False),
    ("mintmcp", r"mintmcp", "mintmcp.com LLM cost optimisation for agent teams (secondary, UNVERIFIED)", "https://www.mintmcp.com/blog/llm-cost-optimization-agent-teams", "web_search", False),
    ("cloudnuro", r"cloudnuro", "cloudnuro.ai (secondary, UNVERIFIED, URL not returned)", None, "web_search", False),
    ("sudoall", r"sudoall", "sudoall.com multi-agent coordination playbook (secondary, UNVERIFIED)", "https://sudoall.com/multi-agent-coordination-2026-playbook/", "web_search", False),
    ("jira-agents-approval", r"Agents in Jira|Jira.{0,40}Approval|Approval step", "Atlassian, Introducing Agents in Jira; Jira Approval step for agent-authored changes", "https://community.atlassian.com/forums/Jira-articles/Introducing-Agents-in-Jira/ba-p/3194583", "vendor_docs", False),
    ("kiro", r"\bKiro\b", "Kiro steering docs; Kiro issue #9176", "https://kiro.dev/docs/steering/", "vendor_docs", False),
    ("spec-kit", r"spec-kit", "GitHub spec-kit", "https://api.github.com/repos/github/spec-kit", "github_api", False),
    ("openspec", r"OpenSpec", "Fission-AI/OpenSpec docs/concepts.md", "https://api.github.com/repos/Fission-AI/OpenSpec", "github_api", False),
    ("bmad", r"BMAD", "bmad-code-org/BMAD-METHOD", "https://api.github.com/repos/bmad-code-org/BMAD-METHOD", "github_api", False),
    ("claude-task-master", r"task-master", "eyaltoledano/claude-task-master", "https://api.github.com/repos/eyaltoledano/claude-task-master", "github_api", False),
    ("superpowers", r"superpowers", "obra/superpowers", "https://api.github.com/repos/obra/superpowers", "github_api", False),
    ("backlog-md", r"Backlog\.md", "MrLesk/Backlog.md (issue #558; standalone binary adoption)", "https://api.github.com/repos/MrLesk/Backlog.md", "github_api", False),
    ("vibe-kanban", r"Vibe Kanban|vibe-kanban|vibekanban", "BloopAI/vibe-kanban (shutdown notice at vibekanban.com/blog/shutdown)", "https://api.github.com/repos/BloopAI/vibe-kanban", "github_api", False),
    ("roo-code", r"Roo-Code|Roo Code", "RooCodeInc/Roo-Code", "https://api.github.com/repos/RooCodeInc/Roo-Code", "github_api", False),
    ("crystal", r"stravu/crystal|\bCrystal\b", "stravu/crystal (dead-with-traction)", "https://api.github.com/repos/stravu/crystal", "github_api", False),
    ("humanlayer", r"humanlayer", "humanlayer/humanlayer", "https://api.github.com/repos/humanlayer/humanlayer", "github_api", False),
    ("terragon", r"terragon", "terragon-labs/terragon-oss (dead-with-traction)", "https://api.github.com/repos/terragon-labs/terragon-oss", "github_api", False),
    ("agent-os", r"Agent OS|buildermethods", "Agent OS v3 CHANGELOG (retiring phases); buildermethods.com migration", "https://github.com/buildermethods/agent-os/blob/main/CHANGELOG.md", "github_api", False),
    ("harness-rs", r"harness-rs", "liliang-cn/harness-rs", "https://api.github.com/repos/liliang-cn/harness-rs", "github_api", False),
    ("dagster", r"Dagster", "Dagster discussion #22811; Dagster RFC on closed enums", "https://github.com/dagster-io/dagster/discussions/22811", "github_api", False),
    ("devin-amp", r"\bDevin\b|\bAmp\b", "Devin and Amp stance on durable mid-run approval (indirect secondary sources only)", None, "web_search", False),
    ("codex-approval-modes", r"Codex.{0,30}approval|approval modes|Codex Handoff", "OpenAI Codex approval modes; community.openai.com handoff-with-approval-gate thread", "https://community.openai.com/t/chatgpt-codex-handoff-with-human-approval-gate/1380791", "web_search", False),
    ("symphony", r"\bSymphony\b", "Symphony (tracker-as-state-machine anti-pattern)", None, "web_search", False),
    ("linear", r"linear\.app|\bLinear\b", "Linear (tracker workflow states / API)", "https://linear.app/docs", "vendor_docs", False),
    ("gitlab-widgets", r"GitLab", "GitLab merge-request widgets / flaky-test quarantine (not re-verified this pass)", "https://docs.gitlab.com/", "vendor_docs", False),
    ("azure-devops-states", r"Azure DevOps|\bADO\b", "Azure DevOps work-item state categories (not re-verified this pass)", "https://learn.microsoft.com/en-us/azure/devops/boards/work-items/workflow-and-state-categories", "vendor_docs", False),
    ("servicenow-change-models", r"ServiceNow", "ServiceNow change models (not re-verified this pass)", "https://www.servicenow.com/docs/", "vendor_docs", False),
    ("augment-worktrees", r"augmentcode|Augment Code", "Augment Code guide: git worktrees for parallel AI agents", "https://www.augmentcode.com/guides/git-worktrees-parallel-ai-agent-execution", "web_search", False),
    ("cost-figures-1.3M-603B", r"\$1\.3M|603B|40.{0,3}60% (multi-provider )?savings", "$1.3M/30-day, 603B-token, 40-60% multi-provider-savings figures (secondary blog claims, never fetched from a primary)", None, "web_search", False),
    ("worker-count-threshold", r"worker-count threshold|context overflow threshold", "vendor-blog worker-count threshold for orchestrator context overflow (no data)", None, "web_search", False),
    ("deterministic-skeleton-consensus", r"deterministic skeleton|AI brain", "'deterministic skeleton, AI brain' as a 2026 production consensus (split evidence)", None, "web_search", False),
]
for k, pat, src, url, meth, lb in COMPETITORS:
    cur("competitors:" + k, "competitors", pat, src, url, meth, "none", force_lb=lb, cs=True)

# decision_records
cur("decision_records:madr-immutability", "decision_records",
    r"MADR[^.|]{0,80}(in-place|immutab|status transition)|(in-place|immutab)[^.|]{0,80}MADR",
    "MADR permits in-place status transitions; ADR immutability is redkiln's own convention",
    "https://github.com/adr/madr/blob/main/template/adr-template.md", "github_api", "1P", force_lb=True)
cur("decision_records:madr-4.0-release-date", "decision_records", r"MADR 4|4\.0\.0|Deciders",
    "MADR 4.0.0 release date and the Deciders rename (adr.github.io/madr could not be fetched; self-flagged UNVERIFIED)",
    "https://api.github.com/repos/adr/madr/releases", "github_api", "1P", force_unv=True)
cur("decision_records:madr-repo", "decision_records", r"\bMADR\b", "MADR (github.com/adr/madr)",
    "https://github.com/adr/madr", "github_api", "1P")
cur("decision_records:yadr", "decision_records", r"YADR", "YADR (github.com/adr/yadr)",
    "https://github.com/adr/yadr", "github_api", "1P")
cur("decision_records:adr-tools", "decision_records", r"adr-tools|npryce", "npryce/adr-tools",
    "https://github.com/npryce/adr-tools", "github_api", "1P")
cur("decision_records:backstage-adrs", "decision_records", r"Backstage", "Backstage architecture decisions",
    "https://backstage.io/docs/architecture-decisions/", "vendor_docs", "1P", cs=True)
cur("decision_records:log4brains", "decision_records", r"Log4brains", "Log4brains ADR tooling (URL not returned)",
    "https://github.com/thomvaill/log4brains", "github_api", "Pr", force_unv=True)
cur("decision_records:adr-templates-index", "decision_records", r"adr-templates|ADR templates index",
    "adr.github.io ADR templates index", "https://adr.github.io/adr-templates/", "vendor_docs", "1P")

# no_url -- the evidence-augmentation s6 rows recorded as "URL not returned by the agent"
NO_URL = [
    ("dbos-workflow-communication", r"DBOS workflow communication", "DBOS workflow communication"),
    ("trigger-dev-waitpoint", r"Trigger\.dev waitpoint", "Trigger.dev waitpoint tokens"),
    ("inngest-waitforevent", r"waitForEvent", "Inngest step.waitForEvent()"),
    ("github-docs-deployments", r"deployments-and-environments|reviewing-deployments", "GitHub Docs deployments-and-environments / reviewing-deployments"),
    ("google-research-blog-multi-agent", r"Google Research blog", "Google Research blog on the multi-agent scaling study"),
    ("speakeasy-agent-hooks", r"Speakeasy", "Speakeasy, AI agent hooks [vendor]"),
    ("pi-stack-error-types", r"Pi Stack", "Pi Stack (2026-06-22) error-types blog"),
    ("rustfaq-linker", r"rustfaq", "rustfaq.org linker guidance"),
    ("swatinem-msrv", r"swatinem", "swatinem.de on MSRV / toolchain pinning"),
    ("cloudnuro", r"cloudnuro", "cloudnuro.ai cost blog"),
    ("tekai-beads-writeup", r"tekai", "tekai.dev writeup on Beads (2026-04-03) [Pr]"),
    ("rust-internals-workspace-thread", r"Rust internals", "Rust internals workspace dependency resolution thread [Pr]"),
    ("visure-ears", r"Visure", "Visure Solutions EARS explainer [Pr]"),
    ("braingrid-ears", r"BrainGrid", "BrainGrid EARS explainer [Pr]"),
    ("log4brains", r"Log4brains", "Log4brains ADR tooling [Pr]"),
]
for k, pat, src in NO_URL:
    cur("no_url:" + k, "no_url", pat, src + " -- URL not returned by the agent; primary must be located first",
        None, "web_search", "none", force_unv=True)

# stated_absences -- sentence-level; each match becomes its own row keyed by normalised sentence
ABSENCE_RE = re.compile(
    r"(?:no (?:published|first-party|peer-reviewed|authoritative|comparable|named|richer|adoption|stabilisation"
    r"|quantified|hard universal|universal|documented|external|public|single authoritative)\s[a-z ,-]{0,40}?"
    r"(?:source|sources|benchmark|convention|norm|guide|guidance|prior art|announcement|research subfield"
    r"|comparator|limits table|system|figure|evidence|support|literature|standard|data|pattern|threshold)"
    r"|no (?:source|sources|benchmark|convention|norm|prior art|comparator|literature|first-party|external)"
    r"\s[a-z ,-]{0,30}?(?:found|exists|available)"
    r"|nothing (?:first-party|found|new found|peer-reviewed|framing)|none (?:was |were )?found|found none"
    r"|genuine gap|positive gap|real gap|does not appear in it|shows no trace|no literature to compare)", re.I)


def load_bearing_ctx():
    """ADR ids with status accepted."""
    acc = set()
    d = os.path.join(ROOT, "references", "adr")
    for fn in os.listdir(d):
        if not fn.endswith(".md") or fn == "README.md":
            continue
        with open(os.path.join(d, fn), encoding="utf-8", errors="replace") as fh:
            head = fh.read(1500)
        m = re.search(r"^status:\s*(\S+)", head, re.M)
        if m and m.group(1).strip().lower() == "accepted":
            acc.add("references/adr/" + fn)
    return acc


PRIORITY_PREPRINTS = ["2603.04417", "2604.22411", "2603.18740", "2609.04167", "2603.00539", "2605.29800"]


def main():
    files = corpus_files()
    accepted_adrs = load_bearing_ctx()
    claims = OrderedDict()
    assertions = []
    anchors_seen = set()
    fixture_only_arxiv_check = ["2604.16790", "2605.21384", "2606.00953"]
    seen_arxiv = set()

    def get(key, family, cited_source, verify_url, verify_method, tag_default="none", note=None):
        if key not in claims:
            claims[key] = dict(id=None, key=key, family=family, claim_text=None, anchors=[],
                               cited_source=cited_source, stated_fetch_date=None, tag=tag_default,
                               load_bearing=False, self_flagged_unverified=False,
                               verify_url=verify_url, verify_method=verify_method,
                               note=note, _tags=defaultdict(int), _dates=defaultdict(int),
                               _lb_reasons=[], _unv_lines=0, _unv_anchors=[], aliases=set(), versions_claimed=set(), versions_near=set(),
                               star_counts=set(), titles=set())
        return claims[key]

    def add_anchor(c, relp, lineno, line, m_start, m_end, block_tag, in_fetched_block):
        a = "%s:%d" % (relp, lineno)
        if (c["key"], a) in anchors_seen:
            return
        anchors_seen.add((c["key"], a))
        c["anchors"].append(a)
        if c["claim_text"] is None:
            c["claim_text"] = excerpt(line, m_start, m_end)
        # tag: nearest bracket tag on the line, else block tag
        tags = [(abs(t.start() - m_start), t.group(1)) for t in TAG_RE.finditer(line)]
        if tags:
            tags.sort()
            c["_tags"][tags[0][1].split("/")[0]] += 1
        elif block_tag:
            c["_tags"][block_tag] += 1
        # fetch date
        fm = FETCH_DATE_RE.search(line)
        if fm:
            c["_dates"][fm.group(1)] += 1
        elif in_fetched_block:
            c["_dates"][in_fetched_block] += 1
        # load-bearing
        if relp in accepted_adrs:
            c["_lb_reasons"].append("accepted ADR " + relp)
        if relp == "references/2026-09-08-acceptance-criteria.md":
            c["_lb_reasons"].append("AC register " + a)
        if relp == "references/2026-09-08-architecture-design.md" and DECIDED_RE.search(line):
            c["_lb_reasons"].append("DECIDED row " + a)
        if UNVERIFIED_RE.search(line[max(0, m_start - 200):m_end + 200]):
            c["_unv_lines"] += 1
            c["_unv_anchors"].append(a)
        for sm in STARS_RE.finditer(line[max(0, m_start - 80):m_end + 120]):
            c["star_counts"].add(sm.group(1))

    for relp in files:
        with open(os.path.join(ROOT, relp), encoding="utf-8", errors="replace") as fh:
            lines = fh.read().split("\n")
        block_tag = None
        fetched_block = None
        for i, line in enumerate(lines, 1):
            if line.startswith("#"):
                block_tag = None
                fetched_block = None
                fm = re.search(r"fetched (20\d\d-\d\d-\d\d)", line)
                if fm:
                    fetched_block = fm.group(1)
            bm = re.match(r"^\*\*\[(1P(?:/blog)?|peer|preprint|vendor|blog|Pr)\]", line)
            if bm:
                block_tag = bm.group(1).split("/")[0]

            # --- arXiv ids (checked before URLs so arxiv URLs fold into the id claim)
            arxiv_spans = []
            for m in ARXIV_RE.finditer(line):
                aid = m.group(1)
                seen_arxiv.add(aid)
                c = get("arxiv:" + aid, "preprints", "arXiv " + aid, "https://arxiv.org/abs/" + aid, "arxiv_abs", "preprint")
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
                arxiv_spans.append((m.start(), m.end()))
                tm = re.search(r"(?<!\*)\*([A-Za-z][^*]{3,120})\*(?!\*)[^.|]{0,40}arXiv[:\s]*" + re.escape(aid), line)
                if tm:
                    c["titles"].add(clean(tm.group(1)))

            # --- advisories
            for m in ADV_RE.finditer(line):
                aid = m.group(1)
                if aid.startswith("RUSTSEC"):
                    url = "https://rustsec.org/advisories/%s.html" % aid
                elif aid.startswith("CVE"):
                    url = "https://osv.dev/vulnerability/" + aid
                else:
                    url = "https://github.com/advisories/" + aid
                c = get("adv:" + aid, "advisories", aid, url, "rustsec_osv", "1P",
                        note="corpus: rustsec.org 404'd; reconstructed from GitHub Advisory Database")
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
            for m in ADV_CONT_RE.finditer(line):
                yr = m.group(1)
                for n in re.findall(r"(\d{4})", m.group(3)):
                    aid = "RUSTSEC-%s-%s" % (yr, n)
                    c = get("adv:" + aid, "advisories", aid, "https://rustsec.org/advisories/%s.html" % aid,
                            "rustsec_osv", "1P", note="continuation form '-NNNN' in the arch-design register")
                    add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)

            # --- DOIs and PIIs
            for m in DOI_RE.finditer(line):
                doi = m.group(1).rstrip(".,;)")
                c = get("doi:" + doi, "peer_and_standards", "DOI " + doi, "https://doi.org/" + doi, "doi_resolve", "peer")
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
            for m in PII_RE.finditer(line):
                pii = m.group(1)
                c = get("pii:" + pii, "peer_and_standards", "ScienceDirect PII " + pii,
                        "https://www.sciencedirect.com/science/article/pii/" + pii, "doi_resolve", "peer")
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)

            # --- URLs
            for m in URL_RE.finditer(line):
                u = strip_url(m.group(0))
                if re.search(r"arxiv\.org", u, re.I):
                    continue  # folded into arxiv:ID
                if re.search(r"doi\.org/", u, re.I):
                    continue  # folded into doi:
                if re.search(r"github\.com/advisories/|rustsec\.org/advisories|advisory-db/main/crates", u, re.I):
                    continue  # folded into adv:
                fam, meth = classify_url(u)
                nk = re.sub(r"^https?://(www\.)?", "", u).rstrip("/").lower()
                c = get("url:" + nk, fam, u, u, meth)
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
                tm = re.search(r"\[([^\]]{3,160})\]\(" + re.escape(m.group(0)), line)
                if tm:
                    c["titles"].add(clean(tm.group(1)))

            # --- curated
            for cd in CURATED:
                for m in cd["pattern"].finditer(line):
                    c = get(cd["key"], cd["family"], cd["cited_source"], cd["verify_url"], cd["verify_method"],
                            cd["tag"], cd["note"])
                    add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
                    if cd["key"].startswith("crate:"):
                        crate = cd["key"][6:]
                        if line.startswith("| `"):
                            # dependency-table row: crate cell, then version cell; zip when counts agree
                            cells = [x.strip() for x in line.strip().strip("|").split("|")]
                            names = re.findall(r"`([A-Za-z0-9_-]+)`", cells[0]) if cells else []
                            vers = [v.group(1) for v in SEMVER_ALL.finditer(cells[1])] if len(cells) > 1 else []
                            if crate in names and vers:
                                if len(names) == len(vers):
                                    c["versions_claimed"].add(vers[names.index(crate)])
                                else:
                                    c["versions_claimed"].update(vers)
                        elif m.lastindex:
                            c["versions_near"].add(m.group(m.lastindex))
                    if cd["force_lb"]:
                        c["_lb_reasons"].append("named priority")
                    if cd["force_unv"]:
                        c["_unv_lines"] += 1
                    break  # one anchor per line per curated key

            # --- stated absences
            for m in ABSENCE_RE.finditer(line):
                sent = excerpt(line, m.start(), m.end(), 400)
                nk = re.sub(r"[^a-z0-9]+", " ", line[m.start():m.end() + 50].lower()).strip()
                c = get("absence:" + nk, "stated_absences", "stated absence: " + clean(m.group(0)), None, "web_search")
                add_anchor(c, relp, i, line, m.start(), m.end(), block_tag, fetched_block)
                break

    # ---- assertions for every curated pattern
    for cd in CURATED:
        n = len(claims[cd["key"]]["anchors"]) if cd["key"] in claims else 0
        assertions.append(dict(key=cd["key"], matched_lines=n, ok=n > 0))
    for aid in sorted(fixture_only_arxiv_check):
        assertions.append(dict(key="arxiv-fixture-only:" + aid, matched_lines=1 if aid in seen_arxiv else 0,
                               ok=aid not in seen_arxiv,
                               note="exploration pass says this id occurs ONLY under spikes/fixtures; ok means absent here"))
    for aid in PRIORITY_PREPRINTS:
        n = len(claims.get("arxiv:" + aid, {}).get("anchors", []))
        assertions.append(dict(key="arxiv-priority:" + aid, matched_lines=n, ok=n > 0))

    # ---- fold URL-derived rows into the curated row that names the same URL
    def norm_url(u):
        u = re.sub(r"^https?://(www\.)?", "", u or "").rstrip("/").lower()
        u = re.sub(r"^crates\.io/crates/", "crates.io/api/v1/crates/", u)
        u = re.sub(r"^api\.github\.com/repos/", "github.com/", u)
        return u
    by_verify = {}
    for key, c in claims.items():
        if not key.startswith("url:") and c["verify_url"]:
            by_verify.setdefault(norm_url(c["verify_url"]), key)
    folded = 0
    for key in list(claims):
        if key.startswith("url:"):
            tgt = by_verify.get(norm_url(claims[key]["verify_url"]))
            if tgt and tgt != key:
                src, dst = claims[key], claims[tgt]
                for a in src["anchors"]:
                    if a not in dst["anchors"]:
                        dst["anchors"].append(a)
                dst["_lb_reasons"] += src["_lb_reasons"]
                dst["_unv_lines"] += src["_unv_lines"]
                dst["_unv_anchors"] += src["_unv_anchors"]
                for t, n in src["_tags"].items():
                    dst["_tags"][t] += n
                for t, n in src["_dates"].items():
                    dst["_dates"][t] += n
                dst["titles"] |= src["titles"]
                dst["aliases"].add(src["verify_url"])
                del claims[key]
                folded += 1

    # ---- finalise
    out_claims = []
    fam_counter = defaultdict(int)
    for key, c in claims.items():
        if not c["anchors"]:
            continue
        if key.startswith("arxiv:") and key[6:] in PRIORITY_PREPRINTS:
            c["_lb_reasons"].append("named priority preprint")
        c["load_bearing"] = bool(c["_lb_reasons"])
        c["self_flagged_unverified"] = c["_unv_lines"] > 0
        if c["_tags"]:
            c["tag"] = max(c["_tags"].items(), key=lambda kv: kv[1])[0]
        if c["_dates"]:
            c["stated_fetch_date"] = max(c["_dates"].items(), key=lambda kv: kv[1])[0]
        if c["titles"]:
            c["cited_source"] = c["cited_source"] + " -- " + " / ".join(sorted(c["titles"]))[:200]
        if c["verify_method"] not in VALID_METHODS:
            c["verify_method"] = "web_search"
        fam_counter[c["family"]] += 1
        c["id"] = "EXT-%s-%03d" % (c["family"].upper().replace("_", "")[:8], fam_counter[c["family"]])
        rec = OrderedDict()
        for k in ("id", "family", "claim_text", "anchors", "cited_source", "stated_fetch_date", "tag",
                  "load_bearing", "self_flagged_unverified", "verify_url", "verify_method"):
            rec[k] = c[k]
        rec["anchor_count"] = len(c["anchors"])
        rec["load_bearing_reasons"] = sorted(set(c["_lb_reasons"]))[:6]
        rec["unverified_flag_lines"] = c["_unv_lines"]
        rec["unverified_flag_anchors"] = c["_unv_anchors"][:8]
        if c["versions_claimed"]:
            rec["versions_claimed"] = sorted(c["versions_claimed"])
        if c["versions_near"]:
            rec["versions_near_name"] = sorted(c["versions_near"])
        if c["star_counts"]:
            rec["star_counts_on_anchor_lines"] = sorted(c["star_counts"])
        if c["aliases"]:
            rec["folded_urls"] = sorted(c["aliases"])
        if c["note"]:
            rec["note"] = c["note"]
        out_claims.append(rec)

    # ---- batches
    prio = lambda r: (0 if r["load_bearing"] else 1, 0 if r["self_flagged_unverified"] else 1)
    by_method = defaultdict(list)
    MERGE_SMALL = {"docs_rs": "crates_io_api+docs_rs+rustsec_osv", "rustsec_osv": "crates_io_api+docs_rs+rustsec_osv",
                   "crates_io_api": "crates_io_api+docs_rs+rustsec_osv"}
    for r in out_claims:
        by_method[MERGE_SMALL.get(r["verify_method"], r["verify_method"])].append(r)
    total = len(out_claims)
    target_batches = 23
    per = max(8, total // target_batches)
    batches = []
    for meth in sorted(by_method, key=lambda m: -len(by_method[m])):
        rows = sorted(by_method[meth], key=lambda r: (prio(r), r["family"], r["id"]))
        n = max(1, round(len(rows) / per))
        size = -(-len(rows) // n)
        for bi in range(n):
            chunk = rows[bi * size:(bi + 1) * size]
            if not chunk:
                continue
            fams = defaultdict(int)
            for r in chunk:
                fams[r["family"]] += 1
            top = max(fams.items(), key=lambda kv: kv[1])[0]
            batches.append(OrderedDict(
                batch_id="B%02d" % (len(batches) + 1),
                agent_label="%s-%s-%d" % (meth, top, bi + 1),
                verify_method=meth.split("+")[0] if "+" not in meth else "mixed",
                verify_methods=sorted(set(r["verify_method"] for r in chunk)),
                n_claims=len(chunk),
                n_load_bearing=sum(1 for r in chunk if r["load_bearing"]),
                n_self_flagged=sum(1 for r in chunk if r["self_flagged_unverified"]),
                families=dict(sorted(fams.items())),
                claim_ids=[r["id"] for r in chunk]))
    # order batches so load-bearing / self-flagged heavy ones come first
    batches.sort(key=lambda b: (-(b["n_load_bearing"] + b["n_self_flagged"]), b["batch_id"]))
    for i, b in enumerate(batches, 1):
        b["batch_id"] = "B%02d" % i

    counts = OrderedDict()
    counts["claims_total"] = total
    counts["anchors_total"] = sum(r["anchor_count"] for r in out_claims)
    counts["files_scanned"] = len(files)
    counts["load_bearing"] = sum(1 for r in out_claims if r["load_bearing"])
    counts["self_flagged_unverified"] = sum(1 for r in out_claims if r["self_flagged_unverified"])
    counts["verify_url_null"] = sum(1 for r in out_claims if not r["verify_url"])
    counts["batches"] = len(batches)
    counts["url_rows_folded_into_curated"] = folded
    counts["curated_patterns_zero_match"] = sum(1 for a in assertions if not a["ok"] and not a["key"].startswith("arxiv-fixture"))
    for f in sorted(fam_counter):
        counts["family:" + f] = fam_counter[f]
    meth_counter = defaultdict(int)
    for r in out_claims:
        meth_counter[r["verify_method"]] += 1
    for m in sorted(meth_counter):
        counts["method:" + m] = meth_counter[m]
    for t in TAG_TOKENS + ("none",):
        counts["tag:" + t] = sum(1 for r in out_claims if r["tag"] == t)
    counts["distinct_arxiv_ids"] = len(seen_arxiv)

    doc = OrderedDict()
    doc["_meta"] = OrderedDict(
        generated_by="census_external.py", repo=ROOT, scope="*.md outside spikes/fixtures/** and spikes/runner/target*/**",
        families=sorted(fam_counter), verify_methods=sorted(VALID_METHODS),
        load_bearing_rule="anchor in an accepted ADR, or in references/2026-09-08-acceptance-criteria.md, "
                          "or on a **DECIDED** row of architecture-design.md, or a named priority claim [inferred rule]",
        unverified_rule="any anchor line matches the UNVERIFIED/not-read/URL-not-returned/search-only regex, "
                        "or the curated pattern is force-flagged from the corpus's own s5 table [measured, line-level]",
        claim_text_rule="verbatim table cell or +-180 char window around the first anchor, <=400 chars")
    doc["counts"] = counts
    doc["assertions"] = assertions
    doc["claims"] = out_claims
    doc["batches"] = batches
    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=1, ensure_ascii=True)
    for k, v in counts.items():
        print("%-40s %s" % (k, v))
    bad = [a for a in assertions if not a["ok"]]
    print("ASSERTION FAILURES: %d" % len(bad))
    for a in bad:
        print("  ", a["key"], a["matched_lines"])
    print("batch sizes:", [b["n_claims"] for b in batches])


if __name__ == "__main__":
    main()
