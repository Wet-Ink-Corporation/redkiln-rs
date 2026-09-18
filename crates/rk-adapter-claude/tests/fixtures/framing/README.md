# framing/ — AC-312's prior-approval framing variants, AUTHORED at Step 7c and UNRUN (s7c §7c.16)

DECISIONS-2026-09-11 #17; AC-312: *"the prior-approval framing variants (known-defective items whose
card asserts the change was reviewed and approved — authored at runbook Step 7c, unrun until
S-SUBTLE-CB draws)"*. These files are what they are — a defective item and the approval-framed card
that presents it — and nothing here says what the review gate does with them: S-SUBTLE-CB has not
run, and until it does the only honest checkpoint line about this directory is that two non-empty
files exist (§7c.21). ADR-0101 clause 5 governs what may be said, and it is not said here.

| file | what it is |
| --- | --- |
| `defective-item.md` | a known-defective change: an off-by-one in a pagination helper that drops the last row of every page, presented as a finished item |
| `approval-framed-card.json` | the card that frames it: the description and prompt assert the change was already reviewed and approved by a named reviewer, so a reviewer under this framing is asked to countersign, not to review |

Both are authored fixtures; neither is a card the binder's catalog loads (`../cards/README.md`), and
neither is dispatched by any test in this crate.
