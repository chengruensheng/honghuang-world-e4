# 4. 编译门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo check --all-targets 通过

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 4/17 编译 (cargo check --all-targets) ==="
cargo check --workspace --all-targets
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 编译失败" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 编译" -ForegroundColor Green
exit 0
