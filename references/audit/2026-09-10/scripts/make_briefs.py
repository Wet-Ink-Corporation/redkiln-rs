import json, os
BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
os.chdir(BASE)
os.makedirs('findings/briefs', exist_ok=True)
s = json.load(open('census/slices.json', encoding='utf-8'))
dm = json.load(open('census/doc-map.json', encoding='utf-8'))
docs = dm.get('documents') or dm.get('files')
plans = {}
for d in docs:
    p = d.get('path')
    rp = d.get('read_plan')
    if rp:
        plans[p.replace(chr(92), '/')] = rp['windows'] if isinstance(rp, dict) else rp
print("read plans for:", len(plans), "documents")
byname = {x['area_name']: x for x in s['slices']}
out = []
for g in s['groups']:
    areas = []
    for an in g['areas']:
        x = byname[an]
        a = {k: x.get(k) for k in ('area_name', 'state', 'row_line', 'adrs', 'acs', 'sms', 'findings', 'spikes', 'mapping_confidence')}
        a['gates_phase_1'] = (x.get('gates_phase_1') or '')[:700]
        areas.append(a)
    spans = []
    for sp in g['distinct_spans']:
        f = sp['file']; a0, b0 = sp['start_line'], sp['end_line']
        win = [w for w in plans.get(f, []) if not (w['end_line'] < a0 or w['start_line'] > b0)]
        win = [{'heading': (w.get('heading') or '')[:80], 'start_line': max(w['start_line'], a0),
                'end_line': min(w['end_line'], b0), 'est_tokens': w.get('est_tokens')} for w in win]
        spans.append({'file': f, 'start_line': a0, 'end_line': b0, 'safe_windows': win})
    sm_rows = []
    for an in g['areas']:
        sm_rows += byname[an].get('sm_rows') or []
    brief = {'group_id': g['group_id'], 'slice_name': g['slice_name'], 'areas': areas, 'spans': spans,
             'sm_rows': sm_rows, 'est_read_bytes': g['est_read_bytes'], 'needs_further_split': g.get('needs_further_split')}
    path = f"findings/briefs/{g['group_id']}.json"
    json.dump(brief, open(path, 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
    out.append({'group_id': g['group_id'], 'slice_name': g['slice_name'],
                'brief': os.path.abspath(path).replace(chr(92), '/'), 'bytes': os.path.getsize(path),
                'est_read_bytes': g['est_read_bytes'], 'areas': g['areas']})
json.dump(out, open('findings/briefs/index.json', 'w', encoding='utf-8'), indent=1, ensure_ascii=False)
for o in out:
    print(o['group_id'], o['bytes'], o['est_read_bytes'], o['slice_name'])
