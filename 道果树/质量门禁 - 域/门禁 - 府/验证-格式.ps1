# 1. 格式门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo fmt --check 全过

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 1/15 格式 (cargo fmt --check) ==="
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 格式未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 格式" -ForegroundColor Green
exit 0
