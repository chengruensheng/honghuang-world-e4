# 10. 无临时目录残留门禁
# 决策锚：传承殿/00-宪法/AGENTS.md § 22.2 边界（清理临时目录）
# 标准：临时文件夹/、.上下文/、*.bak 等不应残留

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 10/17 无临时目录残留 ==="
$违规 = @()

# 检查 临时文件夹/
$临时 = Join-Path (Get-Location) "临时文件夹"
if (Test-Path $临时) {
    $违规 += "存在 临时文件夹/：$临时"
}

# 检查 .上下文/
$上下文 = Join-Path (Get-Location) ".上下文"
if (Test-Path $上下文) {
    $违规 += "存在 .上下文/：$上下文"
}

# 检查散落的 .bak / .tmp
$bak = Get-ChildItem -Path . -Recurse -Include "*.bak","*.old","*.swp","*.tmp" -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -notmatch "道果树\\构建物 - 域" }
if ($bak.Count -gt 0) {
    $违规 += "存在 $($bak.Count) 个备份/临时文件"
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] 临时目录残留：" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}
Write-Host "[PASS] 无临时目录残留" -ForegroundColor Green
exit 0
