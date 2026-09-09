# P7 — deviation adjudication (METHOD-P §4.2, the P4 channel). RYAN FILLS THIS.

METHOD-P §4.2, verbatim on the disclosure that makes this file different from the other three:

> **Deviation channel (P4):** the investigator's output schema includes an optional
> `RequestDeviation { target, kind, claimed_predicate, why }`. Every emitted request is labelled
> warranted/unwarranted by a human after the run (**rule 1 does not apply** — the label is of the
> request, which does not exist before the run — disclosed).

So unlike `p1-arm-{d,l}-labels-RYAN-TODO.md` and `p2-labels-RYAN-TODO.md`, this one is **post-hoc by
design**. You are labelling the request, not the item, and the request is a thing the run produced.
There is no blinding to preserve and no timestamp guard on this file.

labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
records_file: spikes/results/p7.jsonl

## Regenerate the rows

The run appends as it goes, so regenerate this table's rows before filling it:

```bash
python - <<'EOF'
import json, io
n = 0
for l in io.open('spikes/results/p7.jsonl', encoding='utf-8'):
    r = json.loads(l)
    if r.get('phase') != 'emit':
        continue
    d = r.get('request_deviation')
    if not d:
        continue
    n += 1
    print(f"| D{n:02} | {r['item']} | {r['arm']} | r{r['repeat']} | {d.get('target')} | "
          f"{d.get('kind')} | {d.get('claimed_predicate','')[:80]} |  |  |")
print(f"# {n} deviation request(s)")
EOF
```

## What each column means

- **kind** — `add`, `downgrade` or `skip`. `root-cause.toml` declares `deviation_ok = ["add", "downgrade"]`,
  so a `skip` request is **out of contract** even if it is a good idea; note that in `why not`.
- **claimed_predicate** — the fact about *this item* the investigator says justifies the deviation. §4.3 asks
  how many the engine would **auto-accept** (the predicate holds against the evidence) versus **gate**. Judge
  the predicate's truth separately from whether the deviation is a good idea; they come apart.
- **warranted** — `yes` if the process as specified was genuinely wrong for this item; `no` if it was merely
  inconvenient, or if the investigator was routing around work it did not want to do.
- **predicate holds** — `yes` / `no` / `cannot tell`. This is the auto-accept-vs-gate column.

## What the labels decide (METHOD-P §4.4)

- warranted fraction **< 30%**, or **> 1 request per item** → `deviation_ok` is restricted to `skip`
- warranted fraction **< 10%** → the channel is removed from the schema entirely

Both are computed over the runs that actually completed, and the denominator is stated with the result.

| # | item | arm | repeat | target | kind | claimed_predicate | warranted (yes/no) | predicate holds (yes/no/cannot tell) | why (one line) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| — | *(no deviation requests recorded yet — regenerate with the snippet above)* | | | | | | | | |

## Note on the denominator

P7 did not complete: a graph run costs ~$2.2 and ~13 minutes, so the pre-registered 90 runs is 15–18 hours.
The three rigor arms ran in parallel, item-major, so whatever completed is a set of whole (item, arm) cells.
**Report requests-per-item over the items that actually ran all three arms, not over 10.** If that denominator
is small, §4.4's "> 1 request per item" clause is not decidable and should be reported as such rather than
computed on sand.
