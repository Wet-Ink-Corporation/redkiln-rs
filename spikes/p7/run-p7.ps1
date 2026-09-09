# P7 driver — METHOD-P §4. One process per rigor arm, so the three arms make
# progress together and a halt leaves whole (item, arm) cells rather than a
# ragged front.
#
# Ordering: item-major, arm-minor, repeat innermost. The arm is fixed per
# process, so within a process the order is item -> repeat; across the three
# processes an item's three arms advance in step. A spend or headroom halt
# therefore leaves COMPLETE TRIPLES — three repeats of one (item, arm) — which
# is the unit METHOD-P §4.3 reports a Wilson interval over.
#
# DECISIONS-2026-09-09 #26: each arm gets its OWN fixture root,
# spikes/fixtures/p7-<arm>/. The first parallel launch had all three arms
# sharing spikes/fixtures/e3-silent-<slug>, which build_silent_fixture clears
# and rebuilds — so the arms clobbered each other and the mandatory arm died on
# every item with:
#     could not clear fixture ...
#     The process cannot access the file because it is being used by another
#     process. (os error 32)
# The fixture path is derived as <worktree>/../e3-silent-<slug>, so passing a
# per-arm worktree is all the isolation this needs.
#
# Launch DETACHED (HANDOFF F22 — a harness background job is killed at any age):
#   Start-Process pwsh -ArgumentList '-NoProfile','-File','D:\repos\redkiln-rs\spikes\p7\run-p7.ps1','-Arm','optional'
#   ... and again for 'recommended' and 'mandatory'
#
# Poll with a waiter that also matches failure, never a bare `until grep`:
#   grep -qE "P7-DONE|HALT|error|panicked" spikes/p7-<arm>.log

param(
    [Parameter(Mandatory = $true)][ValidateSet('optional', 'recommended', 'mandatory')][string]$Arm,
    [int]$Repeats = 3,
    [double]$MaxCostUsd = 200.0,
    # #26b: a killed vitest can leave a Windows handle on the arm's fixture tree, and
    # build_silent_fixture then cannot clear it (os error 32). Rather than fight the
    # handle, give the arm a fresh root: -Suffix 2.
    [string]$Suffix = '',
    # Raised from the 420 s default once the `mandatory` arm was dropped: with two
    # arms instead of three, a ~250 s discriminating test has room to finish inside
    # the bound, so `confirm` stops being inert (see DECISIONS #31 and #33).
    [int]$ConfirmTimeoutSecs = 900
)

$ErrorActionPreference = 'Continue'
Set-Location 'D:\repos\redkiln-rs\spikes\runner'
$log = "D:\repos\redkiln-rs\spikes\p7-$Arm.log"
$wt = "..\fixtures\p7-$Arm$Suffix\wt"
New-Item -ItemType Directory -Force -Path $wt | Out-Null

"=== P7 $Arm — started $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append

# Repeat is the inner loop so item N's three draws land before item N+1 starts.
# Resume is keyed per (arm, slug, repeat), so a kill costs one graph run.
for ($r = 1; $r -le $Repeats; $r++) {
    "--- $Arm repeat $r of $Repeats ---" | Tee-Object -FilePath $log -Append
    & .\target-dev\debug\redkiln-spike-runner.exe graph-probe --out ..\results\p7.jsonl --arm $Arm --repeat $r --worktree $wt --confirm-timeout-secs $ConfirmTimeoutSecs --max-cost-usd $MaxCostUsd 2>&1 |
        Tee-Object -FilePath $log -Append

    if (Select-String -Path $log -Pattern 'SPEND HALT|HALT before' -Quiet) {
        "halt observed; stopping $Arm before repeat $($r + 1)" | Tee-Object -FilePath $log -Append
        break
    }
}

"=== P7-DONE $Arm $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append
