import json, os, re
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(BASE)
os.makedirs('grounding/batches', exist_ok=True)
e = json.load(open('census/external.json', encoding='utf-8'))
cl = e['claims']
byfam = {}
for c in cl:
    byfam.setdefault(c['family'], []).append(c)

def arxiv_id(c):
    m = re.search(r'(\d{4}\.\d{4,5})', (c.get('verify_url') or '') + ' ' + (c.get('cited_source') or '') + ' ' + c.get('claim_text', ''))
    return m.group(1) if m else None

DEEP = {'2603.04417', '2604.22411', '2603.18740', '2609.04167', '2603.00539', '2605.29800', '2512.08296', '2606.13003', '2606.13685', '2511.01176'}

def chunk(lst, n):
    k = max(1, (len(lst) + n - 1) // n)
    return [lst[i:i + k] for i in range(0, len(lst), k)]

batches = []
def add(label, claims, method, note=''):
    batches.append({'label': label, 'method': method, 'note': note, 'n': len(claims),
                    'load_bearing': sum(1 for c in claims if c.get('load_bearing')), 'claims': claims})

add('crates-versions', byfam.get('crate_versions', []), 'crates_io_api+docs_rs',
    'Every pin in adr/0084 + architecture-design.md 9.2; the corpus self-expires this list after 2026-09-08.')
add('advisories-gix-sqlite', byfam.get('advisories', []) + byfam.get('gitoxide', []) + byfam.get('sqlite', []), 'rustsec_osv+github+docs_rs',
    'RUSTSEC/CVE table reconstructed from GHSA because rustsec.org 404d; gitoxide crate-status quotes; rusqlite WAL/writer claims flagged UNVERIFIED.')
add('toolchain', byfam.get('toolchain', []), 'vendor_docs', 'rustc 1.97.1, edition 2024, clippy groups, dyn-async-fn-in-trait status (R-ASYNC).')
cc = byfam.get('claude_code', [])
for i, part in enumerate(chunk(cc, 2)):
    add(f'claude-code-{i+1}', part, 'vendor_docs', 'Claude Code CLI flags, version requirements, dispatch surface.')
add('anthropic-platform', byfam.get('anthropic_platform', []), 'vendor_docs', 'Prompt caching TTL buckets, scope, ephemeral_* usage fields (S-TTL1), structured-output limits.')
add('durable-execution', byfam.get('durable_execution', []), 'vendor_docs+github', 'Temporal Rust SDK, graph-flow, iopsystems/durable, AWS, Step Functions, Restate, Trigger.dev, Inngest, DBOS, GitHub Environments.')

pp = byfam.get('preprints', [])
deep = [c for c in pp if arxiv_id(c) in DEEP]
rest = [c for c in pp if arxiv_id(c) not in DEEP]
add('preprints-deep', deep, 'arxiv_fulltext',
    'LOAD-BEARING. Read FULL TEXT (arxiv.org/pdf/<id> or /html/<id>), not the abstract. Verify every attributed number and mechanism. See grounding/preprint-probes.md for three abstract-level drift candidates.')
for i, part in enumerate(chunk(rest, 5)):
    add(f'preprints-{i+1}', part, 'arxiv_abs+targeted_fulltext', 'Abstract first; open full text for any claim attributing a specific number, n, or percentage.')
for i, part in enumerate(chunk(byfam.get('peer_and_standards', []), 3)):
    add(f'peer-standards-{i+1}', part, 'doi_resolve+web_search', 'Peer-reviewed, DOIs, ISO/IEEE standards, EARS, rank reversal (S0895717711007850).')
for i, part in enumerate(chunk(byfam.get('competitors', []), 4)):
    add(f'competitors-{i+1}', part, 'github_api+vendor_docs', 'Star counts are 2026-09-08 GitHub API reads; product characterisations; note Beads 26,983 vs Gas Town ~15.9K conflict.')
for i, part in enumerate(chunk(byfam.get('process_literature', []), 4)):
    add(f'process-literature-{i+1}', part, 'web_search+arxiv_abs', 'Method-engineering / process literature cited from references/research; lower build impact; verify existence and the attributed claim.')
for i, part in enumerate(chunk(byfam.get('stated_absences', []), 2)):
    add(f'stated-absences-{i+1}', part, 'web_search', 'Claims of the form "no source/benchmark/convention exists". A single credible counter-example refutes the absence.')
add('no-url', byfam.get('no_url', []), 'web_search', 'Sources whose URL was never captured; locate a primary first, then verify.')
add('decision-records', byfam.get('decision_records', []), 'vendor_docs+github', 'MADR immutability semantics, MADR 4.0 release date (self-flagged UNVERIFIED), YADR.')

for b in batches:
    json.dump(b, open(f"grounding/batches/{b['label']}.json", 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
idx = [{k: b[k] for k in ('label', 'method', 'note', 'n', 'load_bearing')} for b in batches]
json.dump(idx, open('grounding/batches/index.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
tot = sum(b['n'] for b in batches)
print('batches', len(batches), 'claims', tot, 'of', len(cl))
for b in idx:
    print(f"{b['label']:24s} n={b['n']:3d} lb={b['load_bearing']:3d} {b['method']}")
