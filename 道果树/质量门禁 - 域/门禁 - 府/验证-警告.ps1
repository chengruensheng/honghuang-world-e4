# 2. 静态分析门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo clippy --workspace -- -D warnings 零警告

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 2/17 静态分析 (cargo clippy) ==="
cargo clippy --workspace --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] clippy 警告未清零" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 静态分析" -ForegroundColor Green
exit 0
