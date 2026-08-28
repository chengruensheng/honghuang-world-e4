# 15. 文档收割门（设计稿防 AI 文档污染）
# 决策锚：260828-文档收割门（设计稿是宪法，宪法不许被吃掉）
# 标准：传承殿 .md 设计稿禁止断崖式缩水 / 异常空文档（AI 静默吞文档的两种可判定形态）
# 检测1（git 对比）：未提交改动中，某 .md 删除行数 > 新增行数 且 删除占比 >= 50% → 断崖缩水
# 检测2（异常空文档）：去除空白后 < 30 字符 → 疑似被清空/丢失

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 15/15 文档收割门（防 AI 文档污染） ==="

$违规 = @()

# ---- 检测1：git 对比断崖缩水 ----
$改动 = git -c core.quotepath=false status --porcelain -- "传承殿/**/*.md" 2>&1
foreach ($行 in $改动) {
    if ($行 -match '^\s*M\s+(.+)$') {
        $文件 = ($Matches[1] -replace '^"|"$','').Trim()
        if (-not $文件) { continue }
        $stat = git -c core.quotepath=false diff --numstat -- "$文件" 2>&1
        foreach ($s in $stat) {
            if ($s -match '^(\d+)\s+(\d+)\s') {
                $增 = [int]$Matches[1]
                $删 = [int]$Matches[2]
                if (($增 + $删) -gt 0) {
                    $删占比 = [double]$删 / ($增 + $删)
                    if ($删占比 -ge 0.5) {
                        $违规 += "$文件 : 断崖式缩水（+$增/-$删）"
                    }
                }
            }
        }
    }
}

# ---- 检测2：异常空文档 ----
Get-ChildItem -Path "传承殿" -Recurse -Filter "*.md" -Force | ForEach-Object {
    try { $内容 = Get-Content $_.FullName -Raw -Encoding UTF8 } catch { return }
    $非空白 = ($内容 -replace '\s', '').Length
    if ($非空白 -lt 30) {
        $违规 += "$($_.FullName) : 内容异常短（$非空白 非空白字符），疑似清空/丢失"
    }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] $($违规.Count) 个文档污染样本"
    $违规 | ForEach-Object { Write-Host ("  " + $_) }
    exit 1
}

Write-Host "[PASS] 文档收割门：传承殿 .md 无断崖缩水 / 无异常空文档"
exit 0
