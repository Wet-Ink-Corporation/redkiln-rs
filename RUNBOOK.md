# Runbook — Redkiln-RS from here to the end of Phase 1

**This file is a pointer.** On 2026-09-11 the runbook was split into one file per stage under
[`runbook/`](runbook/INDEX.md) after the pre-implementation audit of 2026-09-10
(`references/2026-09-10-pre-implementation-audit.md`, 110 concerns) was absorbed — so a build session loads the
stage it is executing, not 190 KB. Start at [`runbook/INDEX.md`](runbook/INDEX.md); track progress in
[`runbook/PROGRESS.md`](runbook/PROGRESS.md).

**Execution order:** `0 → 1 → 3 → 4 → 4a → 2 → 5 → 6 → 7a → 7b → 7c → 7d → 7e → 8` (Step 2 runs sixth; Step 9 is
rollback). The preamble (`runbook/00-preamble.md`) and the halt conditions (`runbook/halts.md`) apply at every stage.

**The previous monolithic text** — the revision the audit measured — is at git pin `009a4ec`
(`git show 009a4ec:RUNBOOK.md`); every `RUNBOOK.md:NNNN` anchor in the audit resolves there, and `runbook/PROGRESS.md`
§4 maps each concern to where its remediation landed.
