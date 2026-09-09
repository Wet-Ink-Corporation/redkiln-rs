# P1 arm L v2 — the METHOD-P §1.4 anchor-rewrite re-run.
#
# §1.4, first bullet, verbatim: "Any axis that fires a `require` row with alpha
# < 0.6 becomes human-supplied at the triage gate (its anchors are rewritten
# first and re-run once; if still < 0.6, human-supplied)."
#
# v1 measured blast_radius alpha 0.488 and external_dep alpha 0.165. Those two
# anchors — and only those two — were rewritten in
# spikes/prompts/p1-classify-v2.md. This is the "re-run once".
#
# Recorded under arm `L2-<model>` into a SEPARATE results file, so no audit can
# pool the two instruments (DECISIONS-2026-09-09 #27).
#
# Launch DETACHED, per HANDOFF F22:
#   Start-Process pwsh -ArgumentList '-NoProfile','-File','D:\repos\redkiln-rs\spikes\p1\run-arm-l-v2.ps1','-Model','haiku'
#   Start-Process pwsh -ArgumentList '-NoProfile','-File','D:\repos\redkiln-rs\spikes\p1\run-arm-l-v2.ps1','-Model','sonnet'

param(
    [Parameter(Mandatory = $true)][ValidateSet('haiku', 'sonnet')][string]$Model,
    [int]$Repeats = 5,
    [double]$MaxCostUsd = 40.0
)

$ErrorActionPreference = 'Continue'
Set-Location 'D:\repos\redkiln-rs\spikes\runner'
$log = "D:\repos\redkiln-rs\spikes\p1-arm-l-v2-$Model.log"

"=== P1 arm L v2 $Model — started $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append

for ($r = 1; $r -le $Repeats; $r++) {
    "--- v2 repeat $r of $Repeats ---" | Tee-Object -FilePath $log -Append
    & .\target\debug\redkiln-spike-runner.exe classify-probe --out ..\results\p1-arm-l-v2.jsonl --card ..\prompts\p1-classify-v2.md --arm-tag L2 --model $Model --repeat $r --max-cost-usd $MaxCostUsd 2>&1 |
        Tee-Object -FilePath $log -Append

    if (Select-String -Path $log -Pattern 'SPEND HALT|HALT before' -Quiet) {
        "halt observed; stopping v2 $Model before repeat $($r + 1)" | Tee-Object -FilePath $log -Append
        break
    }
}

"=== ARM-L-V2-DONE $Model $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append
