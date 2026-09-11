import json, glob, os, re
from collections import Counter, defaultdict
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
F = os.path.join(BASE, 'findings')
finders, verdicts = {}, {}
for f in sorted(glob.glob(os.path.join(F, '*.json'))):
    name = os.path.basename(f)
    if name.startswith('verdict-') or name == 'index.json':
        continue
    try:
        d = json.load(open(f, encoding='utf-8'))
    except Exception as ex:
        print('BAD', name, ex); continue
    finders[name[:-5]] = d
for f in sorted(glob.glob(os.path.join(F, 'verdict-*.json'))):
    name = os.path.basename(f)[len('verdict-'):-5]
    try:
        d = json.load(open(f, encoding='utf-8'))
    except Exception as ex:
        print('BAD verdict', name, ex); continue
    for v in d.get('verdicts', []):
        verdicts[(name, v['finding_id'])] = v
rows = []
for label, d in finders.items():
    for fd in d.get('findings', []):
        v = verdicts.get((label, fd['id']))
        r = dict(fd)
        r['finder'] = label
        if v:
            r['truth_verdict'] = v.get('truth_verdict') or v.get('verdict')
            r['truth_reason'] = v.get('truth_reason') or v.get('reason', '')
            r['material_verdict'] = v.get('material_verdict') or v.get('verdict')
            r['material_reason'] = v.get('material_reason', '')
            cp = v.get('corrected_priority', 'unchanged')
            cc = v.get('corrected_classification', 'unchanged')
            r['final_priority'] = cp if cp and cp != 'unchanged' else fd['priority']
            r['final_classification'] = cc if cc and cc != 'unchanged' else fd['classification']
            r['corrected_statement'] = v.get('corrected_statement', '')
            r['survives'] = r['truth_verdict'] != 'REFUTED' and r['material_verdict'] != 'REFUTED'
        else:
            r['truth_verdict'] = r['material_verdict'] = 'NO-VERDICT'
            r['final_priority'] = fd['priority']; r['final_classification'] = fd['classification']
            r['survives'] = False
        rows.append(r)
surv = [r for r in rows if r['survives']]
print('finders', len(finders), 'candidates', len(rows), 'survivors', len(surv))
print('truth', Counter(r['truth_verdict'] for r in rows))
print('material', Counter(r['material_verdict'] for r in rows))
print('final priority (survivors)', Counter(r['final_priority'] for r in surv))
print('final classification (survivors)', Counter(r['final_classification'] for r in surv))
print('category (survivors)', Counter(r['category'] for r in surv))
print('per finder survivors', {k: sum(1 for r in surv if r['finder'] == k) for k in finders})
# coarse clustering by shared anchor file:line and shared ids
def keys(r):
    ks = set()
    for a in r.get('anchors', []):
        ks.add(f"{a['file']}:{a['line']}")
    for m in re.findall(r'\b(?:AC|SM|ADR|F|S|P)-?\d{1,4}\b', r['title'] + ' ' + r['statement']):
        ks.add(m)
    return ks
clusters = []
for r in surv:
    ks = keys(r)
    placed = False
    for c in clusters:
        if len(ks & c['keys']) >= 2:
            c['members'].append(r['id']); c['keys'] |= ks; placed = True; break
    if not placed:
        clusters.append({'keys': set(ks), 'members': [r['id']]})
multi = [c for c in clusters if len(c['members']) > 1]
print('coarse clusters', len(clusters), 'multi-member', len(multi), 'largest', max(len(c['members']) for c in clusters))
out = {'finders': list(finders), 'candidates': len(rows), 'survivors': len(surv),
       'coverage': {k: (d.get('coverage') or '')[:1500] for k, d in finders.items()},
       'rows': rows}
json.dump(out, open(os.path.join(F, 'INTERNAL-MERGED.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
p1 = [r for r in surv if r['final_priority'] in ('P0', 'P1')]
print('\n=== P0/P1 survivors:', len(p1))
for r in sorted(p1, key=lambda x: x['finder']):
    print(f"- [{r['final_priority']}/{r['final_classification']}/{r['truth_verdict']}|{r['material_verdict']}] {r['finder']} {r['id']}: {r['title'][:120]}")
