# 15. 文档收割门（设计稿防 AI 文档污染）
# 决策锚：260828-文档收割门（设计稿是宪法，宪法不许被吃掉）
# 标准：传承殿 .md 设计稿禁止断崖式缩水 / 异常空文档（AI 静默吞文档的两种可判定形态）
# 检测1（git 对比）：未提交改动中，某 .md 删除行数 > 新增行数 且 删除占比 >= 50% → 断崖缩水
# 检测2（异常空文档）：去除空白后 < 30 字符 → 疑似被清空/丢失

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 15/17 文档收割门（防 AI 文档污染） ==="

$违规 = @()

# ---- 检测1：git 对比断崖缩水 ----
# 判据（对齐 260828-文档收割门「内容断崖式缩水（小于五成）」）：
# 仅当「净删除（删 > 增）」且「相对原始行数缩水 ≥ 50%」才判为断崖缩水。
# 纯替换（删 == 增，如去空格/改名）不是缩水，不拦截。
$改动 = git -c core.quotepath=false status --porcelain -- "传承殿/**/*.md" 2>$null
foreach ($行 in $改动) {
    if ($行 -match '^\s*M\s+(.+)$') {
        $文件 = ($Matches[1] -replace '^"|"$','').Trim()
        if (-not $文件) { continue }
        $stat = git -c core.quotepath=false diff --numstat -- "$文件" 2>$null
        foreach ($s in $stat) {
            if ($s -match '^(\d+)\s+(\d+)\s') {
                $增 = [int]$Matches[1]
                $删 = [int]$Matches[2]
                if ($删 -gt $增) {
                    $当前行 = (Get-Content -LiteralPath $文件 -ErrorAction SilentlyContinue).Count
                    $原始行 = $当前行 + $删 - $增
                    if ($原始行 -gt 0) {
                        $缩水率 = [double]($删 - $增) / $原始行
                        if ($缩水率 -ge 0.5) {
                            $违规 += "$文件 : 断崖式缩水（原 $原始行 行 → 现 $当前行 行，净删 $($删 - $增) 行，缩水 $([math]::Round($缩水率 * 100))%）"
                        }
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
