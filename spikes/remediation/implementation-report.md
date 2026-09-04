---
item: "{{item}}"
stage: implement
created: "{{created}}"
updated: "{{updated}}"
---

# Implementation Report — {{title}}

## TDD Evidence

Which tests went red then green, mapped to each AC.

**Red against the PRE-FIX tree does not count on its own.** A new capability's
test is red before the fix because the capability is absent, so that redness is
free and says nothing about whether the assertion pinning THIS criterion
discriminates. One row per AC-###, and the mutation column decides the row:

- **Assertion** — the single `file:line` assertion that pins this AC. Not the
  test name: the assertion. If one assertion pins two ACs, say so; if no single
  assertion pins this AC, the AC is untested and the row cannot be completed.
- **Mutation** — the smallest change to the POST-FIX tree that should break that
  assertion, as `file:line` → what you changed it to. Mutate the behaviour the
  AC claims, not the assertion.
- **Test run** — the command you ran with the mutation applied. Scope it to the
  AC's own test file, not the full suite.
- **Red?** — what actually happened. `RED` is the only passing value. `GREEN`
  means the assertion does not discriminate this AC's claim: the criterion is
  untested and the story is not done, whatever the full suite says.

| AC | Assertion (`file:line`) | Mutation (post-fix) | Test run | Red? |
| -- | ----------------------- | ------------------- | -------- | ---- |
|    |                         |                     |          |      |

## Commits

The checkpoint commits this story shipped as. Record them on the item too —
`redkiln record-links <item> --sha <shas>` — before the advance that files this
report: the sha of a commit containing the item file cannot be inside that item
file, so `links.commits` records the WORK commits, which exist by now.

Written here as well as on the item because this is where a reader looks, and
because the field corpus proves the data has always existed in a report and
never anywhere queryable: consumer repos recorded eight-character SHAs in an
`_implementation.md` redkiln had never heard of, while `links.commits` was empty
on every one of 150 items.

| SHA | Subject |
| --- | ------- |
|     |         |

## Changes

The files touched and the shape of the change.

## Gates

Result of test / typecheck / lint / build.

## Notes

Deviations from the plan and why.
