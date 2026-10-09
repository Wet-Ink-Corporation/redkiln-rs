#!/usr/bin/env node
// Applies a backlog-ledger reconciliation manifest to GitHub issues.
//
// Usage: node tools/backlog/apply-manifest.mjs <manifest.json> [--apply] [--delete-retired]
//
// Without --apply it is a dry run: it prints every action and changes nothing.
// It is idempotent: new issues are matched by their `<!-- ledger: ... -->` marker,
// comments by a `<!-- ledger-reconcile ... -->` marker, and labels, types, parents
// and blocked-by links are compared with the live state before any write. A second
// run against the same manifest reports zero actions. See docs/BACKLOG.md.

import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const args = process.argv.slice(2);
const manifestPath = args.find((a) => !a.startsWith('--'));
const APPLY = args.includes('--apply');
const DELETE_RETIRED = args.includes('--delete-retired');
if (!manifestPath) {
  process.stderr.write('usage: apply-manifest.mjs <manifest.json> [--apply] [--delete-retired]\n');
  process.exit(2);
}

const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
const REPO = manifest.repo ?? 'Wet-Ink-Corporation/redkiln-rs';
const [OWNER] = REPO.split('/');
let actions = 0;

function gh(argv, { write = false, input } = {}) {
  if (write) {
    actions += 1;
    process.stdout.write(`${APPLY ? 'RUN ' : 'DRY '} gh ${argv.map((a) => (/\s/.test(a) ? JSON.stringify(a.length > 60 ? `${a.slice(0, 57)}...` : a) : a)).join(' ')}\n`);
    if (!APPLY) return '';
  }
  return execFileSync('gh', argv, { encoding: 'utf8', input, maxBuffer: 64 * 1024 * 1024 });
}
const json = (argv) => JSON.parse(gh(argv));

// --- preflight: issue types the manifest needs must exist at org level ---
const orgTypes = json(['api', 'graphql', '-f', `query={organization(login:"${OWNER}"){issueTypes(first:25){nodes{name isEnabled}}}}`])
  .data.organization.issueTypes.nodes.filter((t) => t.isEnabled).map((t) => t.name);
const wantedTypes = new Set([...manifest.existing, ...manifest.new].map((i) => i.type));
const missingTypes = [...wantedTypes].filter((t) => !orgTypes.includes(t));
if (missingTypes.length) {
  const msg = `org ${OWNER} lacks issue types: ${missingTypes.join(', ')} (add them in org Settings > Planning > Issue types)`;
  if (APPLY) { process.stderr.write(`refusing: ${msg}\n`); process.exit(1); }
  process.stdout.write(`WARN ${msg}; the dry run continues\n`);
}

// --- labels ---
const liveLabels = new Map(json(['label', 'list', '-R', REPO, '--limit', '500', '--json', 'name,color,description']).map((l) => [l.name, l]));
for (const l of manifest.labels.filter((x) => !x.deleteAfterMigration)) {
  const color = l.color.replace(/^#/, '');
  if (l.renameFrom && liveLabels.has(l.renameFrom) && !liveLabels.has(l.name)) {
    gh(['label', 'edit', l.renameFrom, '-R', REPO, '--name', l.name, '--color', color, '--description', l.description], { write: true });
  } else if (!liveLabels.has(l.name)) {
    gh(['label', 'create', l.name, '-R', REPO, '--color', color, '--description', l.description], { write: true });
  } else {
    const live = liveLabels.get(l.name);
    if (live.color.toLowerCase() !== color.toLowerCase() || live.description !== l.description) {
      gh(['label', 'edit', l.name, '-R', REPO, '--color', color, '--description', l.description], { write: true });
    }
  }
}

// --- live issues ---
const live = new Map(json(['issue', 'list', '-R', REPO, '--state', 'all', '--limit', '1000',
  '--json', 'number,title,body,labels,issueType,parent,blockedBy,comments']).map((i) => [i.number, i]));
const marker = (body) => (body.match(/<!-- ledger: [^>]*-->/) ?? [null])[0];
const byMarker = new Map([...live.values()].map((i) => [marker(i.body ?? ''), i.number]).filter(([m]) => m));

// --- create new issues (no relations yet; they are linked in the last pass) ---
const keyToNumber = new Map();
for (const n of manifest.new) {
  const m = marker(n.body);
  if (!m) { process.stderr.write(`new:${n.key} has no ledger marker; refusing\n`); process.exit(1); }
  if (byMarker.has(m)) { keyToNumber.set(n.key, byMarker.get(m)); continue; }
  const argv = ['issue', 'create', '-R', REPO, '--title', n.title, '--body-file', '-', '--type', n.type];
  for (const l of n.labels) argv.push('--label', l);
  const out = gh(argv, { write: true, input: n.body });
  const num = APPLY ? Number(out.trim().split('/').pop()) : `new:${n.key}`;
  keyToNumber.set(n.key, num);
  if (APPLY) live.set(num, { number: num, title: n.title, body: n.body, labels: n.labels.map((name) => ({ name })), issueType: { name: n.type }, parent: null, blockedBy: { nodes: [] }, comments: [] });
}
const resolve = (ref) => (ref == null ? null : ref.startsWith('new:') ? keyToNumber.get(ref.slice(4)) : Number(ref.replace('#', '')));

// --- edit existing issues ---
for (const e of manifest.existing) {
  const cur = live.get(e.number);
  if (!cur) { process.stderr.write(`#${e.number} not found; skipping\n`); continue; }
  const argv = ['issue', 'edit', String(e.number), '-R', REPO];
  if (e.title && e.title !== cur.title) argv.push('--title', e.title);
  if (cur.issueType?.name !== e.type) argv.push('--type', e.type);
  const have = new Set(cur.labels.map((l) => l.name));
  const want = new Set(e.labels);
  const add = [...want].filter((l) => !have.has(l));
  // A renamed label shows under its old name until the rename runs; do not remove it twice.
  const renamedFrom = new Set(manifest.labels.filter((l) => l.renameFrom).map((l) => l.renameFrom));
  const remove = [...have].filter((l) => !want.has(l) && !renamedFrom.has(l));
  if (add.length) argv.push('--add-label', add.join(','));
  if (remove.length) argv.push('--remove-label', remove.join(','));
  if (argv.length > 5) gh(argv, { write: true });
  if (e.comment) {
    const tag = `<!-- ledger-reconcile id=#${e.number} manifest=${manifest.id ?? '2026-10-08'} -->`;
    if (!cur.comments.some((c) => (c.body ?? '').includes(tag))) {
      gh(['issue', 'comment', String(e.number), '-R', REPO, '--body-file', '-'], { write: true, input: `${e.comment}\n\n${tag}` });
    }
  }
}

// --- relations: parent and blocked-by ---
for (const i of [...manifest.existing.map((e) => ({ ...e, num: e.number })), ...manifest.new.map((n) => ({ ...n, num: keyToNumber.get(n.key) }))]) {
  const cur = typeof i.num === 'number' ? live.get(i.num) : null;
  const parent = resolve(i.parent);
  if (parent != null && cur?.parent?.number !== parent) {
    gh(['issue', 'edit', String(i.num), '-R', REPO, '--parent', String(parent)], { write: true });
  }
  const haveBlockers = new Set((cur?.blockedBy?.nodes ?? []).map((b) => b.number));
  const addBlockers = (i.blockedBy ?? []).map(resolve).filter((b) => !haveBlockers.has(b));
  if (addBlockers.length) gh(['issue', 'edit', String(i.num), '-R', REPO, '--add-blocked-by', addBlockers.join(',')], { write: true });
}

// --- retire default labels once no issue carries them ---
if (DELETE_RETIRED) {
  for (const l of manifest.labels.filter((x) => x.deleteAfterMigration && liveLabels.has(x.name))) {
    gh(['label', 'delete', l.name, '-R', REPO, '--yes'], { write: true });
  }
}

process.stdout.write(`${APPLY ? 'applied' : 'dry run'}: ${actions} action(s)\n`);
