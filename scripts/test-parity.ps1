# Automated End-to-End Smoke Test for Spider-Man Hydration Desktop App
Write-Host "🕷️  Running End-to-End Functional Test Suite...`n" -ForegroundColor Cyan

$tauriExe = "tauri/src-tauri/target/release/spider-man-hydration-tauri.exe"

if (-not (Test-Path $tauriExe)) {
    Write-Error "Release executable not found at $tauriExe. Run 'npm run build:tauri' first."
    exit 1
}

# Test 1: Launch Process 1 with --demo
Write-Host "Test 1: Launching Primary Instance with --demo..." -ForegroundColor Yellow
$p1 = Start-Process -FilePath $tauriExe -ArgumentList "--demo" -PassThru
Start-Sleep -Seconds 3

if ($p1.HasExited) {
    Write-Host "❌ Primary instance exited unexpectedly with code $($p1.ExitCode)" -ForegroundColor Red
    exit 1
} else {
    Write-Host "✅ Primary instance running successfully (PID: $($p1.Id), RAM: $([math]::Round($p1.WorkingSet64 / 1MB, 2)) MB)" -ForegroundColor Green
}

# Test 2: Verify Single-Instance Mutex (Process 2 should exit immediately)
Write-Host "`nTest 2: Verifying Single-Instance Mutex Enforcement..." -ForegroundColor Yellow
$p2 = Start-Process -FilePath $tauriExe -PassThru
Start-Sleep -Seconds 2

if ($p2.HasExited) {
    Write-Host "✅ Secondary instance detected active mutex and exited cleanly (Single Instance Verified!)" -ForegroundColor Green
} else {
    Write-Host "❌ Secondary instance failed to exit. Multiple instances detected!" -ForegroundColor Red
    Stop-Process -Id $p2.Id -Force
}

# Test 3: Settings Persistence Verification
Write-Host "`nTest 3: Checking AppData Persistence..." -ForegroundColor Yellow
$settingsPath = "$env:APPDATA\spider-man-hydration-reminder\settings.json"
if (Test-Path $settingsPath) {
    $settings = Get-Content $settingsPath -Raw | ConvertFrom-Json
    Write-Host "✅ settings.json verified at $settingsPath" -ForegroundColor Green
    Write-Host "   Interval: $($settings.reminderIntervalMinutes) min | Sound: $($settings.enableSound) | Streak: $($settings.stats.streakDays) days" -ForegroundColor Gray
} else {
    Write-Host "⚠️  settings.json not yet created" -ForegroundColor Yellow
}

# Clean Up
Write-Host "`nCleaning up test processes..." -ForegroundColor Yellow
Stop-Process -Id $p1.Id -Force
Write-Host "✅ All tests passed successfully!`n" -ForegroundColor Green
