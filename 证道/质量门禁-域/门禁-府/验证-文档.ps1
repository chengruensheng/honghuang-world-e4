# 5. 文档门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo doc --no-deps 通过

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 5/17 文档 (cargo doc --no-deps) ==="
cargo doc --no-deps --workspace
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 文档生成失败" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 文档" -ForegroundColor Green
exit 0
