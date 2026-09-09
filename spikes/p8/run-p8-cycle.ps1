# P8 — one full suspend / kill / answer / resume cycle for a delay arm.
#
# METHOD-P §5.2's procedure, per run: "drive to the gate mark; kill the process
# (SIGTERM, F21 semantics); wait 0 / 65 min / 24 h; answer via CLI; resume;
# compare every step output digest before the kill with the digest after".
#
# The kill must destroy a process that is HOLDING ITS STATE IN MEMORY, so one
# process is launched per item and killed where it blocks. Writing state and
# reading it back in-process would prove nothing — the risk is that a *fresh*
# process reconstructs something different (F19, human_gate.rs).
#
# Windows has no SIGTERM. `taskkill /T /F` terminates the tree; the polite form
# is `taskkill` without /F (WM_CLOSE). These are different experiments and the
# form used is recorded here: /T /F.
#
# Citation note: the AFK brief says "F21 semantics". F21 is the E3 arm (b)
# detection result, not process semantics — F19/F22 are (DECISIONS #24).
#
#   pwsh -File run-p8-cycle.ps1 -Arm 0-delay -Repeat 1 -DelayMinutes 0
#   pwsh -File run-p8-cycle.ps1 -Arm 65-min  -Repeat 1 -DelayMinutes 65

param(
    [Parameter(Mandatory = $true)][string]$Arm,
    [int]$Repeat = 1,
    [int]$DelayMinutes = 0,
    [int]$TimeoutSecs = 1500,
    [string]$Kill = 'force'          # 'force' = taskkill /T /F, 'polite' = taskkill /T
)

$ErrorActionPreference = 'Continue'
$root = 'D:\repos\redkiln-rs\spikes'
$runner = "$root\runner\target-p8\debug\redkiln-spike-runner.exe"
$log = "$root\p8-$Arm-r$Repeat.log"
Set-Location "$root\runner"

$items = Get-Content "$root\plans\p8-items.txt" |
    Where-Object { $_ -and -not $_.StartsWith('#') } |
    ForEach-Object { ($_ -split "`t")[1] }

"=== P8 cycle arm=$Arm repeat=$Repeat delay=${DelayMinutes}m kill=$Kill $(Get-Date -Format o) ===" |
    Tee-Object -FilePath $log -Append
"items: $($items -join ', ')" | Tee-Object -FilePath $log -Append

# --- 1. drive each item to its gate mark, one process each, and let it block ---
foreach ($it in $items) {
    $wt = "..\fixtures\p8-$Arm-$it\wt"
    New-Item -ItemType Directory -Force -Path $wt | Out-Null
    # One log per item: three Tee-Object writers on a single file contend for
    # the Windows file lock, and two of three processes died on it.
    $ilog = "$root\p8-$Arm-r$Repeat-$it.log"
    $cmd = "Set-Location '$root\runner'; & '$runner' repro-probe --phase run --arm $Arm --repeat $Repeat " +
           "--only $it --worktree '$wt' --out ..\results\p8.jsonl --timeout-secs $TimeoutSecs " +
           "--block-secs 7200 --force-gate 2>&1 | Tee-Object -FilePath '$ilog' -Append"
    Start-Process pwsh -ArgumentList '-NoProfile', '-Command', $cmd -WindowStyle Hidden
    Start-Sleep -Seconds 5
}

# --- 2. wait for every item to reach its mark ---------------------------------
"waiting for $($items.Count) gate mark(s)..." | Tee-Object -FilePath $log -Append
$deadline = (Get-Date).AddMinutes(60)
while ((Get-Date) -lt $deadline) {
    $marks = @(Select-String -Path "$root\p8-$Arm-r$Repeat-*.log" -Pattern 'GATE-MARK' -ErrorAction SilentlyContinue |
               Where-Object { $_.Line -match "$Arm r$Repeat" })
    if ($marks.Count -ge $items.Count) { break }
    Start-Sleep -Seconds 20
}
$marks = @(Select-String -Path "$root\p8-$Arm-r$Repeat-*.log" -Pattern 'GATE-MARK' -ErrorAction SilentlyContinue |
           Where-Object { $_.Line -match "$Arm r$Repeat" })
"gate marks reached: $($marks.Count)/$($items.Count)" | Tee-Object -FilePath $log -Append

# --- 3. the kill. This is the measurement. ------------------------------------
$t0 = Get-Date
$procs = @(Get-Process redkiln-spike-runner -ErrorAction SilentlyContinue)
"killing $($procs.Count) process(es) at $($t0.ToString('o')) with taskkill $(if($Kill -eq 'force'){'/T /F'}else{'/T'})" |
    Tee-Object -FilePath $log -Append
foreach ($p in $procs) {
    if ($Kill -eq 'force') { taskkill /PID $p.Id /T /F 2>&1 | Out-Null }
    else { taskkill /PID $p.Id /T 2>&1 | Out-Null }
}
Start-Sleep -Seconds 6
"runners after kill: $(@(Get-Process redkiln-spike-runner -ErrorAction SilentlyContinue).Count)" |
    Tee-Object -FilePath $log -Append

# --- 4. wait out the delay arm ------------------------------------------------
if ($DelayMinutes -gt 0) {
    "waiting $DelayMinutes min (F18/F28 anchor; the point is to resume outside the ~1 h cache TTL)" |
        Tee-Object -FilePath $log -Append
    Start-Sleep -Seconds ($DelayMinutes * 60)
}

# --- 5. answer, then resume in a FRESH process --------------------------------
foreach ($it in $items) {
    & $runner repro-probe --phase answer --arm $Arm --repeat $Repeat --only $it `
        --answer provide-repro-steps --out ..\results\p8.jsonl 2>&1 | Tee-Object -FilePath $log -Append
}
foreach ($it in $items) {
    $wt = "..\fixtures\p8-$Arm-$it\wt"
    & $runner repro-probe --phase resume --arm $Arm --repeat $Repeat --only $it `
        --worktree $wt --out ..\results\p8.jsonl --timeout-secs $TimeoutSecs --force-gate 2>&1 |
        Tee-Object -FilePath $log -Append
}

"=== P8-CYCLE-DONE $Arm r$Repeat $(Get-Date -Format o) ===" | Tee-Object -FilePath $log -Append
