# 6. 安全审计门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo audit 无高危漏洞
# 注：阶段 1 暂未引入依赖，若 cargo audit 不存在则跳过

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 6/10 安全审计 (cargo audit) ==="
$which = Get-Command cargo-audit -ErrorAction SilentlyContinue
if ($null -eq $which) {
    Write-Host "[SKIP] cargo-audit 未安装，跳过（阶段 1 0 依赖）" -ForegroundColor Yellow
    exit 0
}
cargo audit
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 安全审计发现高危漏洞" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 安全审计" -ForegroundColor Green
exit 0
