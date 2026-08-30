# 16. 治理审计门禁（元三治·治强 + 治理事件流 append-only 可追溯）
# 决策锚：260829 记忆架构契约 §五 双工流水线 + 06-治理/04-元三治
# 标准：
#   1) 治理事件流 append-only（事件流_追加 为 INSERT，无 UPDATE/DELETE 事件流）
#   2) 治理事件流 6 类事件（终裁通过交付/终裁打回/打回重投/打回达上限/玉玺盖印/废止）均已埋点
#   3) 治益 4 分类不增不减（角色卡 match 穷尽 4 变体，无 default 分支）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 16/16 治理审计（元三治） ==="

$违规 = @()

# 1) 事件流 append-only：追加是 INSERT，区间是 SELECT，无 UPDATE/DELETE 事件流
$存储核心 = "鸿蒙/基础设施 - 域/记忆承载 - 府/记忆存储-殿/SQLite存储-阁/SQLite实现-园/SQLite_核心.rs"
if (Test-Path $存储核心) {
    $sql = Get-Content $存储核心 -Raw -Encoding UTF8
    if ($sql -notmatch "INSERT.*事件流") {
        $违规 += "事件流_追加 无 INSERT（非 append-only）"
    }
    if ($sql -match "UPDATEs+事件流" -or $sql -match "DELETEs+FROMs+事件流") {
        $违规 += "事件流表存在 UPDATE/DELETE（破坏 append-only）"
    }
} else {
    $违规 += "SQLite 存储核心缺失：$存储核心"
}

# 2) 治理事件流 6 类事件埋点（4 类终裁流 + 玉玺盖印/废止）
$模拟 = "乾坤/呈现 - 域/命令操作 - 府/命令调度-殿/端到端-阁/端到端实现-园/模拟_llm.rs"
if (Test-Path $模拟) {
    $文本 = Get-Content $模拟 -Raw -Encoding UTF8
    foreach ($事件 in @("终裁通过交付", "终裁打回", "打回重投", "打回达上限")) {
        if ($文本 -notmatch [regex]::Escape($事件)) {
            $违规 += "治理事件流缺埋点：$事件"
        }
    }
} else {
    $违规 += "模拟_llm.rs 缺失：$模拟"
}
$闭环 = "鸿蒙/基础设施 - 域/记忆承载 - 府/记忆应用-殿/闭环应用-阁/闭环实现-园/闭环_核心.rs"
if (Test-Path $闭环) {
    $文本 = Get-Content $闭环 -Raw -Encoding UTF8
    foreach ($事件 in @("玉玺盖印", "废止")) {
        if ($文本 -notmatch [regex]::Escape($事件)) {
            $违规 += "治理事件流缺埋点：$事件"
        }
    }
} else {
    $违规 += "闭环_核心.rs 缺失：$闭环"
}

# 3) 治益 4 分类不增不减：角色卡 match 穷尽（无 _ => default）
$分类核心 = "鸿蒙/基础设施 - 域/流水线驱动 - 府/流水线分类-殿/分类规则-阁/分类实现-园/分类_核心.rs"
if (Test-Path $分类核心) {
    $文本 = Get-Content $分类核心 -Raw -Encoding UTF8
    foreach ($变体 in @("分类::道祖级", "分类::圣人级", "分类::大罗金仙级", "分类::准圣级")) {
        if ($文本 -notmatch [regex]::Escape($变体)) {
            $违规 += "4 分类缺变体：$变体"
        }
    }
    # 角色卡() match 无 default（治益上限 4 编译期穷尽）
    if ($文本 -match "角色卡(self)[sS]{0,200}_s*=>") {
        $违规 += "角色卡 match 存在 default 分支（分类漂移未编译拦截）"
    }
} else {
    $违规 += "分类_核心.rs 缺失：$分类核心"
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] 治理审计违规：" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}
Write-Host "[PASS] 治理审计：事件流 append-only + 6 类治理事件埋点 + 治益 4 分类穷尽"
exit 0
