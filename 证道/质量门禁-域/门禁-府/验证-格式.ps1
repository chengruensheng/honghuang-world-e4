# 1. 格式门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo fmt --check 全过

# stderr 合并到 stdout + Continue 模式：规避 PowerShell 5.1 NativeCommandError 误判（同 2 项修复）
$ErrorActionPreference = "Continue"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 1/17 格式 (cargo fmt --check) ==="
cargo fmt --all -- --check 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 格式未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 格式" -ForegroundColor Green
exit 0
