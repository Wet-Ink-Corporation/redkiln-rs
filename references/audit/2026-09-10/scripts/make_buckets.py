import json, os
from collections import Counter
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(BASE)
os.makedirs('synthesis/buckets', exist_ok=True)
d = json.load(open('findings/INTERNAL-MERGED.json', encoding='utf-8'))
g = json.load(open('grounding/GROUNDING-MERGED.json', encoding='utf-8'))
surv = [r for r in d['rows'] if r['survives']]

def slim(r):
    return {k: r.get(k) for k in ('id', 'finder', 'title', 'category', 'final_priority', 'final_classification',
                                   'known_issue_match', 'anchors', 'statement', 'corrected_statement', 'why_before_step5',
                                   'evidence_tag', 'remediation', 'owner', 'confidence', 'truth_verdict', 'truth_reason',
                                   'material_verdict', 'material_reason')}

BUCKETS = {
    'B01-readiness': lambda r: r['category'] in ('blocked-leakage', 'sequencing', 'gap'),
    'B02-realisability': lambda r: r['category'] in ('buildability', 'operational'),
    'B03-ambiguity': lambda r: r['category'] == 'ambiguity',
    'B04-contradiction-a': lambda r: r['category'] == 'contradiction' and r['finder'] < 'slice-G10',
    'B05-contradiction-b': lambda r: r['category'] == 'contradiction' and r['finder'] >= 'slice-G10',
    'B06-inaccuracy': lambda r: r['category'] == 'inaccuracy',
    'B07-citation-statistics': lambda r: r['category'] in ('citation-drift', 'statistics'),
    'B08-method-integrity': lambda r: r['category'] in ('testability', 'prereg-integrity', 'register-integrity', 'overlap', 'authority', 'other'),
}
assigned = set()
index = []
for name, pred in BUCKETS.items():
    rows = [slim(r) for r in surv if pred(r) and r['id'] + r['finder'] not in assigned]
    for r in rows:
        assigned.add(r['id'] + r['finder'])
    p = f'synthesis/buckets/{name}.json'
    json.dump({'bucket': name, 'n': len(rows), 'findings': rows}, open(p, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    index.append({'bucket': name, 'n': len(rows), 'bytes': os.path.getsize(p), 'priorities': dict(Counter(r['final_priority'] for r in rows))})
left = [r for r in surv if r['id'] + r['finder'] not in assigned]
print('unassigned', len(left))

# grounding buckets
mat = [r for r in g['rows'] if r['verdict'] in ('DRIFTED', 'CONTRADICTED', 'STALE-BY-DATE') and r['impact'] != 'none']
lbn = [r for r in g['rows'] if r['load_bearing'] and r['verdict'] != 'CONFIRMED']
ids = {r['claim_id'] for r in mat} | {r['claim_id'] for r in lbn}
grows = [r for r in g['rows'] if r['claim_id'] in ids]
def gslim(r):
    return {k: r.get(k) for k in ('claim_id', 'family', 'batch', 'verdict', 'first_verdict', 'impact', 'load_bearing',
                                   'self_flagged_unverified', 'claim_text', 'anchors', 'corpus_says', 'source_says',
                                   'source_quote', 'url', 'fetched', 'note', 'second_note', 'second_url')}
half = (len(grows) + 1) // 2
for i, part in enumerate((grows[:half], grows[half:])):
    name = f'B09-grounding-{i+1}'
    p = f'synthesis/buckets/{name}.json'
    json.dump({'bucket': name, 'n': len(part), 'rows': [gslim(r) for r in part]}, open(p, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    index.append({'bucket': name, 'n': len(part), 'bytes': os.path.getsize(p), 'verdicts': dict(Counter(r['verdict'] for r in part))})
p = 'synthesis/buckets/B10-best-practice.json'
json.dump({'bucket': 'B10-best-practice', 'n': len(g['best_practice']), 'rows': g['best_practice']}, open(p, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
index.append({'bucket': 'B10-best-practice', 'n': len(g['best_practice']), 'bytes': os.path.getsize(p), 'verdicts': dict(Counter(r['verdict'] for r in g['best_practice']))})
# coverage file for the critics
json.dump({'internal_coverage': d['coverage'], 'grounding_tally': g['tally'], 'grounding_missing': g['missing_claim_ids'],
           'first_verification_of_self_flagged': g['first_verification_of_self_flagged']},
          open('synthesis/coverage.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
json.dump(index, open('synthesis/buckets/index.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
for b in index:
    print(b)
