# P1 arm D — human labels (BLANK template). METHOD-P §0 rule 1: fill this from the item and its diff ALONE,
# before running or looking at spikes/p1/derive-facets.py output. Copy to p1-arm-d-labels-<who>.md, fill,
# then set `labelled_at` to the UTC time you finished. score-arm-d.py refuses labels stamped after the engine ran.
labelled_at: FILL-ME-ISO8601Z
labeller: FILL-ME
# METHOD-P amendment 8 (2026-09-09). The chronology check that normally proves blindness has EXPIRED for this
# file: the engine output spikes/results/p1-arm-d.jsonl was written 2026-09-08T04:54:07Z, before this template
# was filled, so any truthful labelled_at is later than the engine's and score-arm-d.py refuses by default.
# If -- and only if -- you completed these labels without opening that file, fill the line below and score with
#   python spikes/p1/score-arm-d.py --engine spikes/results/p1-arm-d.jsonl --labels <this file> --attested-blind
# It must name who attests, what they attest they did not open, and when. Results scored this way are reported
# as ATTESTED-BLIND, never as timestamp-verified. Do NOT backdate labelled_at. Do NOT re-run derive-facets.py to
# refresh the engine's timestamps -- it would pass the check while hiding the substitution.
blind_attestation: FILL-ME (e.g. "<name> attests on <date> that they had not opened spikes/results/p1-arm-d.jsonl before completing these labels")
corpus_sha: 47faec9f46b8d983c2c75f0794164e68415ae706
items_file: spikes/plans/p1-items.txt

# One row per (item, facet). toolchain/change: comma-separated set from facets.toml values. Booleans: 1/0. "?" = cannot tell from the item.
# GENERATED 2026-09-09 from p1-items.txt (20 items x 8 facets = 160 rows). Fill `value`; leave `?` where the
# item alone cannot tell. `recent_deploy` is underivable at this pin (no deploy_log) -- label it `?` unless you
# know otherwise; the engine emitted null (DECISIONS-2026-09-09 #12). `binary_change` is NOT in facets.toml (#11).
| item (slug) | facet | value | note |
| --- | --- | --- | --- |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | toolchain |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | change |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | binary_change |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | auth |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | data_migration |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | ui |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | flag_exists |  |  |
| _archive/docs-site-brand-identity/typography-delivery/plex-per-weight-fallback-metrics | recent_deploy |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | toolchain |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | binary_change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | auth |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | data_migration |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | ui |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | flag_exists |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/dark-cut-focus-ring-on-pre | recent_deploy |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | toolchain |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | binary_change |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | auth |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | data_migration |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | ui |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | flag_exists |  |  |
| _archive/docs-site-brand-identity/highlight-and-states/landing-and-component-consumers | recent_deploy |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | toolchain |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | change |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | binary_change |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | auth |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | data_migration |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | ui |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | flag_exists |  |  |
| support/phase-1-total-outcomes/gate-passes-its-known-base | recent_deploy |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/kb-and-graph-validation | recent_deploy |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | toolchain |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | change |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | binary_change |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | auth |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | data_migration |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | ui |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | flag_exists |  |  |
| _archive/live-views/proof-and-readings/fixme-cannot-pass-tripwire | recent_deploy |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout/resolve-open-question | recent_deploy |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | toolchain |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | change |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | binary_change |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | auth |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | data_migration |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | ui |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | flag_exists |  |  |
| support/fix-lane-plumbing/issue-50-fix-story-before-worktree | recent_deploy |  |  |
| _archive/docs-site-brand-identity/typography-delivery | toolchain |  |  |
| _archive/docs-site-brand-identity/typography-delivery | change |  |  |
| _archive/docs-site-brand-identity/typography-delivery | binary_change |  |  |
| _archive/docs-site-brand-identity/typography-delivery | auth |  |  |
| _archive/docs-site-brand-identity/typography-delivery | data_migration |  |  |
| _archive/docs-site-brand-identity/typography-delivery | ui |  |  |
| _archive/docs-site-brand-identity/typography-delivery | flag_exists |  |  |
| _archive/docs-site-brand-identity/typography-delivery | recent_deploy |  |  |
| support/fix-gate-honesty-sweep | toolchain |  |  |
| support/fix-gate-honesty-sweep | change |  |  |
| support/fix-gate-honesty-sweep | binary_change |  |  |
| support/fix-gate-honesty-sweep | auth |  |  |
| support/fix-gate-honesty-sweep | data_migration |  |  |
| support/fix-gate-honesty-sweep | ui |  |  |
| support/fix-gate-honesty-sweep | flag_exists |  |  |
| support/fix-gate-honesty-sweep | recent_deploy |  |  |
| support/fix-json-error-envelope | toolchain |  |  |
| support/fix-json-error-envelope | change |  |  |
| support/fix-json-error-envelope | binary_change |  |  |
| support/fix-json-error-envelope | auth |  |  |
| support/fix-json-error-envelope | data_migration |  |  |
| support/fix-json-error-envelope | ui |  |  |
| support/fix-json-error-envelope | flag_exists |  |  |
| support/fix-json-error-envelope | recent_deploy |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | toolchain |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | binary_change |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | auth |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | data_migration |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | ui |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | flag_exists |  |  |
| _archive/docs-site-brand-identity/kb-harvest-and-closeout | recent_deploy |  |  |
| support/fix-ci-pinned-pack-drift-guard | toolchain |  |  |
| support/fix-ci-pinned-pack-drift-guard | change |  |  |
| support/fix-ci-pinned-pack-drift-guard | binary_change |  |  |
| support/fix-ci-pinned-pack-drift-guard | auth |  |  |
| support/fix-ci-pinned-pack-drift-guard | data_migration |  |  |
| support/fix-ci-pinned-pack-drift-guard | ui |  |  |
| support/fix-ci-pinned-pack-drift-guard | flag_exists |  |  |
| support/fix-ci-pinned-pack-drift-guard | recent_deploy |  |  |
| _archive/docs-site-brand-identity/contrast-verification | toolchain |  |  |
| _archive/docs-site-brand-identity/contrast-verification | change |  |  |
| _archive/docs-site-brand-identity/contrast-verification | binary_change |  |  |
| _archive/docs-site-brand-identity/contrast-verification | auth |  |  |
| _archive/docs-site-brand-identity/contrast-verification | data_migration |  |  |
| _archive/docs-site-brand-identity/contrast-verification | ui |  |  |
| _archive/docs-site-brand-identity/contrast-verification | flag_exists |  |  |
| _archive/docs-site-brand-identity/contrast-verification | recent_deploy |  |  |
| _archive/live-views | toolchain |  |  |
| _archive/live-views | change |  |  |
| _archive/live-views | binary_change |  |  |
| _archive/live-views | auth |  |  |
| _archive/live-views | data_migration |  |  |
| _archive/live-views | ui |  |  |
| _archive/live-views | flag_exists |  |  |
| _archive/live-views | recent_deploy |  |  |
| _archive/docs-site-brand-identity | toolchain |  |  |
| _archive/docs-site-brand-identity | change |  |  |
| _archive/docs-site-brand-identity | binary_change |  |  |
| _archive/docs-site-brand-identity | auth |  |  |
| _archive/docs-site-brand-identity | data_migration |  |  |
| _archive/docs-site-brand-identity | ui |  |  |
| _archive/docs-site-brand-identity | flag_exists |  |  |
| _archive/docs-site-brand-identity | recent_deploy |  |  |
| support | toolchain |  |  |
| support | change |  |  |
| support | binary_change |  |  |
| support | auth |  |  |
| support | data_migration |  |  |
| support | ui |  |  |
| support | flag_exists |  |  |
| support | recent_deploy |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | toolchain |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | change |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | binary_change |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | auth |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | data_migration |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | ui |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | flag_exists |  |  |
| support/links-hand-edit-denied-with-no-backfill-path | recent_deploy |  |  |
| support/advance-commit-ignored-path | toolchain |  |  |
| support/advance-commit-ignored-path | change |  |  |
| support/advance-commit-ignored-path | binary_change |  |  |
| support/advance-commit-ignored-path | auth |  |  |
| support/advance-commit-ignored-path | data_migration |  |  |
| support/advance-commit-ignored-path | ui |  |  |
| support/advance-commit-ignored-path | flag_exists |  |  |
| support/advance-commit-ignored-path | recent_deploy |  |  |
| support/telemetry-gitignored-so-runs-never-travel | toolchain |  |  |
| support/telemetry-gitignored-so-runs-never-travel | change |  |  |
| support/telemetry-gitignored-so-runs-never-travel | binary_change |  |  |
| support/telemetry-gitignored-so-runs-never-travel | auth |  |  |
| support/telemetry-gitignored-so-runs-never-travel | data_migration |  |  |
| support/telemetry-gitignored-so-runs-never-travel | ui |  |  |
| support/telemetry-gitignored-so-runs-never-travel | flag_exists |  |  |
| support/telemetry-gitignored-so-runs-never-travel | recent_deploy |  |  |
