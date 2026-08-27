# 3. 单元测试门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo test --workspace --lib -- --test-threads=1 全绿

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 3/10 单元测试 (cargo test) ==="
cargo test --workspace --lib -- --test-threads=1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 单元测试未全绿" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 单元测试" -ForegroundColor Green
exit 0
