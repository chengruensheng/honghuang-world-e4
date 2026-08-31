# 13. 防退化 ≥2 殿门禁
# 决策锚：260827-防退化-≥2殿
# 目的：所有府级 crate 必须 ≥2 殿，防止 Round 5 漏掉的单府回归
# 实现：对工作区根实跑 jianyan_gongju 的 15 项架构校验（第 15 项 = 检查_府级crate至少2殿），任一失败即拦截

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 13/17 防退化 ≥2 殿 (jianyan_gongju 跑全部实跑 15 项) ==="
cargo run -p jianyan_gongju --example 跑全部自检 -- .
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 防退化 ≥2 殿 未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 防退化 ≥2 殿" -ForegroundColor Green
exit 0
