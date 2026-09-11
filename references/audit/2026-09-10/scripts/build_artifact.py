import json, os, re
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(BASE)
m = json.load(open('synthesis/concerns/MASTER.json', encoding='utf-8'))
t = open('synthesis/review/REVIEW-VERIFIED.md', encoding='utf-8').read()

def section(n):
    a = re.search(rf'^## {n}\. .*$', t, re.M)
    b = re.search(rf'^## {n+1}\. .*$', t, re.M)
    return t[a.start(): b.start() if b else len(t)]

front = t[:re.search(r'^## 3\. ', t, re.M).start()]
grounding, sota, limits, method, sources = section(6), section(7), section(8), section(9), section(10)

# concern bodies from sections 3-5
body = t[re.search(r'^## 3\. ', t, re.M).start(): re.search(r'^## 6\. ', t, re.M).start()]
parts = re.split(r'^(#{3,4} (PIA-\d{3})[^\n]*)$', body, flags=re.M)
# parts: [pre, heading, id, chunk, heading, id, chunk, ...]
bodies, groups = {}, {}
group = ''
pre = parts[0]
for mm in re.finditer(r'^### (\d\.\d[^\n]*)$', pre, re.M):
    group = mm.group(1)
i = 1
while i < len(parts):
    heading, cid, chunk = parts[i], parts[i + 1], parts[i + 2]
    # a theme sub-heading may sit at the end of this chunk (before the next concern)
    sub = list(re.finditer(r'^### ([^\n]*)$', chunk, re.M))
    own = chunk[: sub[0].start()] if sub else chunk
    groups[cid] = group
    bodies[cid] = heading.lstrip('#').strip() + '\n' + own.strip()
    if sub:
        group = sub[-1].group(1)
    i += 3

def coarse(cat):
    c = cat.lower()
    if 'external-grounding' in c or 'toolchain' in c or 'positioning' in c or 'newer first-party' in c or 'best-practice' in c:
        return 'external grounding'
    if 'contradiction' in c or 'misread' in c:
        return 'contradiction'
    if 'inaccuracy' in c or 'unsourced' in c:
        return 'inaccuracy'
    if 'citation' in c:
        return 'citation drift'
    if 'ambiguity' in c or 'vocabulary' in c or 'grammar' in c:
        return 'ambiguity'
    if 'blocked' in c or 'sequencing' in c or 'runbook' in c or 'coverage' in c:
        return 'readiness'
    if 'buildability' in c or 'operational' in c or 'design gap' in c:
        return 'buildability'
    if 'testability' in c or 'statistics' in c or 'prereg' in c:
        return 'method integrity'
    if 'register' in c or 'traceability' in c or 'authority' in c or 'overlap' in c:
        return 'register integrity'
    return 'hygiene'

concerns = []
for c in m['concerns']:
    cid = c['id']
    concerns.append({
        'id': cid, 'priority': c['priority'], 'classification': c['classification'],
        'category': c['category'], 'group': coarse(c['category']),
        'title': c['title'], 'owner': c['owner'], 'confidence': c['confidence'],
        'evidence_tag': c['evidence_tag'], 'build_stage': c.get('build_stage', ''),
        'theme': groups.get(cid, ''), 'n_anchors': len(c.get('anchors', [])),
        'n_sources': len(c.get('external_sources', [])), 'n_folds': len(c.get('member_ids', [])),
        'md': bodies.get(cid, '_(body not found in review)_'),
    })
missing = [c['id'] for c in concerns if c['id'] not in bodies]
print('concerns', len(concerns), 'bodies', len(bodies), 'missing', missing)
stages = sorted({c['build_stage'] for c in concerns})
print('stages', stages)

data = {
    'head': '009a4ec', 'date': '2026-09-10 / 2026-09-11',
    'counts': {'candidates': 357, 'survivors': 338, 'bucketed': 151, 'concerns': 110, 'claims': 860,
               'confirmed': 697, 'drifted': 105, 'contradicted': 29, 'unverifiable': 16, 'stale': 13,
               'anchors': 602, 'urls': 119, 'agents': 151},
    'by_priority': m['by_priority'], 'by_classification': m['by_classification'],
    'concerns': concerns,
    'sections': {'front': front, 'grounding': grounding, 'sota': sota, 'limits': limits, 'method': method, 'sources': sources},
}
payload = json.dumps(data, ensure_ascii=False).replace('</', '<\\/')

html = open('scripts/artifact_template.html', encoding='utf-8').read().replace('/*__DATA__*/', payload)
out = os.path.join(BASE, 'redkiln-preimplementation-audit.html')
open(out, 'w', encoding='utf-8').write(html)
print('wrote', out, len(html.encode('utf-8')), 'bytes')
