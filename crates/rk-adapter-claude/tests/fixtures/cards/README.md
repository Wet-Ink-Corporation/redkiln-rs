# cards/ — the binder's fixture cards (s7c §7c.12, §7c.17; D-11; O-6)

`.redkiln/` does not exist until 7e, so **these are the only cards on the tree** and the catalog AC-269's
card-validator iterates at 7c (O-6's default; the catalog is RAISED in §7c.22). A card file is our own
closed record (`description`, `prompt`, `tools`, `cache_profile`, `permission_mode`; the name is the file
stem) — **never** `model` or `effort`, which the role owns (AC-118, AC-418) and the binder injects (§7c.12).

| directory | what it holds |
| --- | --- |
| `catalog/` | every card that must load and bind: `probe.json` (the live probe's haiku card — StructuredOutput in `tools`, an explicit `permission_mode`, a `cache_profile`; AC-205/AC-214/AC-215/AC-383 all hold on it), `golden-static.json` / `golden-default.json` (the two golden-argv cards — one body, the two profiles), `reviewer.json` (a sonnet card, so `--effort` is exercised) |
| `red/` | the must-fail cards, one per criterion: `no-structured-output.json` (AC-205/AC-417), `tools-empty.json` and `tools-absent.json` (AC-493), `no-profile.json` (AC-383), `no-permission-mode.json` (AC-214), `authored-with-model.json` (AC-118/AC-418's leak), `names-answer.json` (AC-269) |
| `roles/` | the role tier fixtures the binder injects: `haiku.json` (the probe's), `sonnet-high.json`, and AC-212's reds — `sonnet-no-effort.json` (case a), `no-model.json` (case b) — plus `haiku-with-effort.json` (an effort on a tier that rejects it) |

The at-the-cap card is under `../argv/` beside the golden vectors it is measured against.
