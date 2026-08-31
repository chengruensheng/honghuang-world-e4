# 验证-一键全验完整性.ps1 - 一键全验双版本引用完整性门禁
# 决策锚：260828-循环推进 § 第四十五轮（自检第 15 项）
# falsifiable：新门禁（如 验证-格位稀缺 / 验证-文档收割门）未同步进 一键全验.sh/.ps1 时 exit 1
$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSCommandPath) | Out-Null
cd ..\..\..   # 回工作区根（门禁-府 → 质量门禁-域 → 证道 → 根）

$目标 = @(
    @{ 脚本 = "验证-格位稀缺.ps1"; 说明 = "格位稀缺 36 上限" },
    @{ 脚本 = "验证-文档收割门.ps1"; 说明 = "文档收割门防污染" }
)
$聚合 = @("一键全验.sh", "一键全验.ps1")

$失败 = 0
foreach ($门禁 in $目标) {
    foreach ($聚合文件 in $聚合) {
        $内容 = Get-Content -Raw (Join-Path (Get-Location) $聚合文件) -ErrorAction SilentlyContinue
        if (-not $内容) {
            Write-Host "[FAIL] $聚合文件 不存在或不可读" -ForegroundColor Red
            $失败++
            continue
        }
        if ($内容 -notmatch [regex]::Escape($门禁.脚本)) {
            Write-Host "  [FAIL] $聚合文件 未引用 $($门禁.脚本)（$($门禁.说明)）" -ForegroundColor Red
            $失败++
        }
    }
}

Write-Host "=== 17/17 一键全验完整性（.sh/.ps1 双版本） ==="
if ($失败 -gt 0) {
    Write-Host "[FAIL] 一键全验聚合脚本存在 $失败 处引用缺失" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 一键全验 .sh/.ps1 均引用两个新门禁脚本齐全（含 格位稀缺 / 文档收割门）" -ForegroundColor Green
exit 0
