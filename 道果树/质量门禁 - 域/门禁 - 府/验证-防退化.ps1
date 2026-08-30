# 13. 防退化 ≥2 殿门禁
# 决策锚：260827-防退化-≥2殿
# 目的：所有府级 crate 必须 ≥2 殿，防止 Round 5 漏掉的单府回归
# 实现：调用 jianyan_gongju 的 测试_跑全部_返回15项，其中第 15 项 = 检查_府级crate至少2殿

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 13/15 防退化 ≥2 殿 (jianyan_gongju 跑全部 第15项) ==="
cargo test -p jianyan_gongju --lib 测试_跑全部_返回15项
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 防退化 ≥2 殿 未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 防退化 ≥2 殿" -ForegroundColor Green
exit 0
