import json, glob, os
from collections import Counter
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
G = os.path.join(BASE, 'grounding')
claims = {c['id']: c for c in json.load(open(os.path.join(BASE, 'census/external.json'), encoding='utf-8'))['claims']}
first, second, bp = {}, {}, []
for f in sorted(glob.glob(os.path.join(G, 'verdict-*.json'))):
    name = os.path.basename(f)
    if name.endswith('.min.json'):
        continue
    try:
        d = json.load(open(f, encoding='utf-8'))
    except Exception as ex:
        print('BAD', name, ex); continue
    batch = name[len('verdict-'):-5]
    for v in d.get('verdicts', []):
        v['batch'] = batch
        if batch.startswith('bp-'):
            bp.append(v)
        else:
            first[v['claim_id']] = v
for f in sorted(glob.glob(os.path.join(G, 'verdict2-*.json'))):
    try:
        d = json.load(open(f, encoding='utf-8'))
    except Exception as ex:
        print('BAD', os.path.basename(f), ex); continue
    for v in d.get('verdicts', []):
        second[v['claim_id']] = v
merged = []
flips = 0
for cid, v in first.items():
    s = second.get(cid)
    row = dict(v)
    row['first_verdict'] = v['verdict']
    if s:
        row['verdict'] = s['verdict']
        row['second_note'] = s.get('note', '')
        row['second_url'] = s.get('url', '')
        if s['verdict'] != v['verdict']:
            flips += 1
    c = claims.get(cid, {})
    row['family'] = c.get('family')
    row['load_bearing'] = c.get('load_bearing')
    row['self_flagged_unverified'] = c.get('self_flagged_unverified')
    row['anchors'] = c.get('anchors', [])[:6]
    row['claim_text'] = c.get('claim_text', '')[:400]
    merged.append(row)
missing = [cid for cid in claims if cid not in first]
tally = Counter(r['verdict'] for r in merged)
material = [r for r in merged if r['verdict'] in ('DRIFTED', 'CONTRADICTED') and r['impact'] in ('blocks-decision', 'changes-a-number', 'changes-a-citation')]
out = {
    'generated': 'merge_grounding.py over grounding/verdict-*.json and verdict2-*.json',
    'claims_total': len(claims), 'verdicts': len(merged), 'missing_claim_ids': missing,
    'tally': dict(tally), 'second_opinion_flips': flips,
    'material_count': len(material),
    'material_by_family': dict(Counter(r['family'] for r in material)),
    'load_bearing_nonconfirmed': [r['claim_id'] for r in merged if r['load_bearing'] and r['verdict'] != 'CONFIRMED'],
    'first_verification_of_self_flagged': dict(Counter(r['verdict'] for r in merged if r['self_flagged_unverified'])),
    'best_practice': bp,
    'rows': merged,
}
json.dump(out, open(os.path.join(G, 'GROUNDING-MERGED.json'), 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
print('claims', len(claims), 'verdicts', len(merged), 'missing', len(missing), 'flips', flips)
print('tally', dict(tally))
print('material', len(material), dict(Counter(r['family'] for r in material)))
print('load-bearing non-confirmed', len(out['load_bearing_nonconfirmed']))
print('self-flagged first verification', out['first_verification_of_self_flagged'])
print('best-practice rows', len(bp), dict(Counter(v['verdict'] for v in bp)))
