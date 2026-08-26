# 7. 依赖审查门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo deny check 通过
# 注：阶段 1 暂未引入依赖，若 cargo-deny 不存在则跳过

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 7/10 依赖审查 (cargo deny check) ==="
$which = Get-Command cargo-deny -ErrorAction SilentlyContinue
if ($null -eq $which) {
    Write-Host "[SKIP] cargo-deny 未安装，跳过（阶段 1 0 依赖）" -ForegroundColor Yellow
    exit 0
}
cargo deny check
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 依赖审查未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 依赖审查" -ForegroundColor Green
exit 0
