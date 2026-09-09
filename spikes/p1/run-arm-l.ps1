# P1 arm L driver — METHOD-P §1.2. One process per model tier; repeats run in
# sequence inside it so resume is keyed per (arm, slug, repeat) and a kill costs
# one dispatch (METHOD-S §0).
#
# Launch DETACHED, never as a harness background job (HANDOFF F22: anything
# reaching harness background is killed at any age; a Start-Process-detached
# process survived ~12,000 s through ~14 killed watchers):
#
#   Start-Process pwsh -ArgumentList '-File','D:\repos\redkiln-rs\spikes\p1\run-arm-l.ps1','-Model','haiku'
#   Start-Process pwsh -ArgumentList '-File','D:\repos\redkiln-rs\spikes\p1\run-arm-l.ps1','-Model','sonnet'
#
# Poll from the foreground with a waiter that also matches failure, never a bare
# `until grep` (it spins forever if the job dies):
#   grep -qE "ARM-L-DONE|HALT|error|panicked" spikes/p1-arm-l-haiku.log

param(
    [Parameter(Mandatory = $true)][ValidateSet('haiku', 'sonnet')][string]$Model,
    [int]$Repeats = 5,
    [double]$MaxCostUsd = 40.0
)

$ErrorActionPreference = 'Stop'
$runner = 'D:\repos\redkiln-rs\spikes\runner'
$log = "D:\repos\redkiln-rs\spikes\p1-arm-l-$Model.log"
Set-Location $runner

"=== P1 arm L $Model — started $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append

for ($r = 1; $r -le $Repeats; $r++) {
    "--- repeat $r of $Repeats ---" | Tee-Object -FilePath $log -Append
    & .\target\debug\redkiln-spike-runner.exe classify-probe `
        --out ..\results\p1-arm-l.jsonl `
        --model $Model `
        --repeat $r `
        --max-cost-usd $MaxCostUsd 2>&1 | Tee-Object -FilePath $log -Append

    # A spend halt or a headroom halt is a PAUSE, not a failure — but there is
    # no point starting the next repeat into the same wall.
    if (Select-String -Path $log -Pattern 'SPEND HALT' -Quiet) {
        "spend cap reached; stopping before repeat $($r + 1)" | Tee-Object -FilePath $log -Append
        break
    }
}

"=== ARM-L-DONE $Model $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append
