# `pack-min/` — the smallest loadable process pack

**Provenance: authored 2026-09-20 for Step 7d Slice 3. No byte of this directory comes from the
pinned corpus**, and nothing here is derived from `spikes/p9/`. It is a fixture, minted for the
`rk-cli` integration suite, and AC-465's residency rule is satisfied by that: these files are
tracked in this repository because they were written here.

`s7d` §7d.13 fixes the contents: *"the smallest loadable pack: one role, one card with
`tools: [StructuredOutput]`, `cache_profile = "static"`, `timeout_secs` on every activity,
`margin_ticks`, both TTLs, an attempt cap of `1` …, a `gate_deadline_secs` and a gate-class expiry
row …, one `require` row reading a facet the gate items leave null …, one `tool` step; a two-activity
variant …; a variant declaring a **1-second** `gate_deadline_secs` …"*.

## How a test installs it

The directory is NOT read in place. Each test mints its own temp repository (D18-9) and copies:

| fixture file | install path in the repository |
| --- | --- |
| `pack.toml` | `.redkiln/packs/bug-standard.toml` **and** `.redkiln/packs/story.toml` |
| `artifacts.toml` | `.redkiln/artifacts.toml` |
| `facets.toml` | `.redkiln/facets.toml` |
| `catalog/*.toml` | `.redkiln/catalog/*.toml` |
| `items/*.toml` | `.redkiln/items/*.toml` |
| `config.toml` | `.redkiln/config.toml` |

`s7d` §7d.5: *"`pack-min/` is installed in the lane fixtures **twice**, as
`.redkiln/packs/bug-standard.toml` and `.redkiln/packs/story.toml` (ADR-0128 decision 3's two names),
with `[packs] enabled = ["bug-standard", "story"]`."*

## The three variants, and why there is no fourth

`pack.toml` is the base. `pack.two-activities.toml` adds a second baseline kind so two nodes assemble
(`ac485_…`, `ac401_…`). `pack.gate-1s.toml` declares `gate_deadline_secs = 1` so a test-side wait can
pass a real deadline without a clock toggle (`ac257_…`, `ac248_…`'s expired arm). **Nothing in the
runbook licenses a fourth**, and a fourth would be a variant no test names.

## Two parsers read `pack.toml`, over the same bytes

`rk_kernel::select::Pack::from_toml` reads the TOML subset and **ignores unknown keys**; `rk-cli`'s
own `PackExtras` reads the adapter-facing values (`gate_deadline_secs`, `[margins]`, `[cache]`,
`[[gate_expiry]]`, `[activity.<id>] timeout_secs`, `[role.<name>]` with its `model` / `effort`).
The kernel's `Pack` is **not** widened for them — that would re-open Checkpoint 7b for a fixture. A
record fixing these key spellings in the pack schema is Step 7e's (RT-57; `s7d` §7d.18 RAISED).

**"Over the same bytes" is a constraint, not a description.** `[[gate_expiry]]` is an array of
tables rather than the table it was, because the class it carries is the full gate kind
`unknown_facet:has_repro`, a TOML key carrying a colon must be quoted, and the kernel's
`toml_subset` walker refuses quoted keys outright. A key shape only one of the two parsers accepts
is available to neither. See the 🛑 note on that table in `pack.toml`.

The three variants — `pack.toml`, `pack.gate-1s.toml`, `pack.two-activities.toml` — are **one base
with one delta each** (the gate deadline; the second activity and its baseline artifact) and are
regenerated from `pack.toml`, never edited in parallel. They were each two concatenated TOML
documents until Slice 3 cycle 5, which is what a hand-regeneration that appends produces.

*(dated 2026-09-26, `s8d` build, package P3.)* A further variant, `pack.step-grain.toml`, is regenerated from `pack.toml` the
same way, with one delta: `s8d` §8d.2 and §8d.8 defect 4 (record R1 = ADR-0174, proposed) need a node with NO agent step
and a node with two agent steps of different roles and output kinds. Its two activities and its second card live under
`step-grain/` (`catalog/two-agents.toml`, `catalog/no-agent-steps.toml`, `cards/reviewer.json`) and are installed only by
`support::PackVariant::StepGrain`, so no other variant sees them. (`pack.review.toml` and `pack.dispatch-1s.toml` were
already a fourth and fifth before it; "there is no fourth" above is history.)

*(dated 2026-09-28, `s8e` build, package S8E-P2.)* `pack.lane.toml` is regenerated from `pack.review.toml` with one delta:
`[policy]` declares a `stage_order` (`triage` empty, `discovery` forbidden, `implement` with the abstention gate, `review`
over `review-min`), so `verbs::run::run`'s lane loop crosses boundaries (`s8e` §8e.11). It is installed only by
`support::PackVariant::Lane`, with `review-min.toml` and `lane/items/`'s three items (`lane-1` omits `has_repro`; `lane-2`
and `lane-3` declare it; each declares a `slice`). `lane/implement-frames.jsonl` is the transcript the fixture child replays
for the lane's `implement` dispatch in the binary arm. `config.toml` now declares `[project] name = "rk-fixture"`
(ADR-0159), and `items/review-item.toml` a `slice`.
