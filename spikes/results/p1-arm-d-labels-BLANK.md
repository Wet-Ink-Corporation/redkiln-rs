# P1 arm D — human labels (BLANK template). METHOD-P §0 rule 1: fill this from the item and its diff ALONE,
# before running or looking at spikes/p1/derive-facets.py output. Copy to p1-arm-d-labels-<who>.md, fill,
# then set `labelled_at` to the UTC time you finished. score-arm-d.py refuses labels stamped after the engine ran.
labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
items_file: spikes/plans/p1-items.txt

# One row per (item, facet). toolchain/change: comma-separated set from facets.toml values. Booleans: 1/0. "?" = cannot tell from the item.
| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| <slug-1> | toolchain |  |  |
| <slug-1> | change |  |  |
| <slug-1> | binary_change |  |  |
| <slug-1> | auth |  |  |
| <slug-1> | data_migration |  |  |
| <slug-1> | ui |  |  |
| <slug-1> | flag_exists |  |  |
# … repeat the seven rows for each of the 30 items in p1-items.txt (generate the skeleton with:
#   awk -F'\t' '!/^#/{for(f in F){}; print}' — or simply run: python3 -c "import sys;[print(f'| {l.split(chr(9))[1]} | {f} |  |  |') for l in open('spikes/plans/p1-items.txt') if not l.startswith('#') for f in ['toolchain','change','binary_change','auth','data_migration','ui','flag_exists']]"
