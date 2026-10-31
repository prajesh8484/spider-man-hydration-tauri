# Automated Empirical Benchmark Runner: Electron vs. Rust (Tauri / Tower)
Write-Host "Spider-Man Hydration Benchmark Suite`n" -ForegroundColor Cyan

# 1. Executable and Installer Binary Sizes
$tauriExe = "tauri/src-tauri/target/release/spider-man-hydration-tauri.exe"
$electronExe = "electron/release/win-unpacked/Spider-Man Hydration Reminder.exe"
$tauriInstaller = Get-ChildItem -Path "tauri/src-tauri/target/release/bundle/nsis/*.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
$electronInstaller = Get-ChildItem -Path "electron/release/Spider-Man Hydration Setup*.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

$sizeTable = @()

if (Test-Path $electronExe) {
    $eSize = (Get-Item $electronExe).Length / 1MB
    $tSize = if (Test-Path $tauriExe) { (Get-Item $tauriExe).Length / 1MB } else { 0 }
    $reduction = if ($eSize -gt 0) { [math]::Round((($eSize - $tSize) / $eSize) * 100, 2) } else { 0 }
    $sizeTable += [PSCustomObject]@{
        Target = "Primary Executable (.exe)"
        Electron_MB = [math]::Round($eSize, 2)
        Rust_Tauri_MB = [math]::Round($tSize, 2)
        Savings = "$reduction %"
    }
}

if ($electronInstaller -and $tauriInstaller) {
    $eiSize = $electronInstaller.Length / 1MB
    $tiSize = $tauriInstaller.Length / 1MB
    $iReduction = [math]::Round((($eiSize - $tiSize) / $eiSize) * 100, 2)
    $sizeTable += [PSCustomObject]@{
        Target = "NSIS Windows Setup (.exe)"
        Electron_MB = [math]::Round($eiSize, 2)
        Rust_Tauri_MB = [math]::Round($tiSize, 2)
        Savings = "$iReduction %"
    }
}

Write-Host "Binary and Installer Footprint:" -ForegroundColor Yellow
$sizeTable | Format-Table -AutoSize

# 2. Check for active running instances
$tauriProcs = Get-Process -Name "*spider-man-hydration-tauri*" -ErrorAction SilentlyContinue
$electronProcs = Get-Process -Name "*Spider-Man Hydration Reminder*" -ErrorAction SilentlyContinue

if ($tauriProcs -or $electronProcs) {
    Write-Host "`nLive Process and RAM Measurements:" -ForegroundColor Yellow
    $memTable = @()
    if ($electronProcs) {
        $eWS = ($electronProcs | Measure-Object -Property WorkingSet64 -Sum).Sum / 1MB
        $ePriv = ($electronProcs | Measure-Object -Property PrivateMemorySize64 -Sum).Sum / 1MB
        $memTable += [PSCustomObject]@{
            Framework = "Electron"
            Processes = $electronProcs.Count
            WorkingSet_RAM_MB = [math]::Round($eWS, 2)
            PrivateBytes_MB = [math]::Round($ePriv, 2)
        }
    }
    if ($tauriProcs) {
        $tWS = ($tauriProcs | Measure-Object -Property WorkingSet64 -Sum).Sum / 1MB
        $tPriv = ($tauriProcs | Measure-Object -Property PrivateMemorySize64 -Sum).Sum / 1MB
        $memTable += [PSCustomObject]@{
            Framework = "Rust (Tauri)"
            Processes = $tauriProcs.Count
            WorkingSet_RAM_MB = [math]::Round($tWS, 2)
            PrivateBytes_MB = [math]::Round($tPriv, 2)
        }
    }
    $memTable | Format-Table -AutoSize
} else {
    Write-Host "`nNeither app is currently running in the background." -ForegroundColor Gray
    Write-Host "Launch Tauri:    ./tauri/src-tauri/target/release/spider-man-hydration-tauri.exe" -ForegroundColor Gray
    Write-Host "Launch Electron: ./electron/release/win-unpacked/'Spider-Man Hydration Reminder.exe'" -ForegroundColor Gray
}
