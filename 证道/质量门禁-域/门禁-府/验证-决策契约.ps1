# 11. 决策契约 lint（12 契约）
# 决策锚：260826-2230 工程-DSH + 260826-2220 治理-司衡
# 标准：传承殿/03-决策/**/*.md 的 YAML frontmatter 必填 8 字段
#      （id/title/stage/decided_by/falsifiable/upstream/implements/decided_at）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 11/17 决策契约 lint ==="

$必填 = @(
    "id", "title", "stage", "decided_by",
    "falsifiable", "upstream", "implements", "decided_at"
)

$决策目录 = "传承殿/03-决策"
if (-not (Test-Path $决策目录)) {
    Write-Host "[SKIP] 决策目录不存在"
    exit 0
}

# 提取 frontmatter（从 --- 到 --- 之间的内容，子串方法避开 regex 兼容问题）
function 提取-FrontMatter([string]$内容) {
    $trimmed = $内容.TrimStart("`r", "`n", " ", "`t")
    if (-not $trimmed.StartsWith("---")) { return $null }
    # 去掉起始 ---
    $rest = $trimmed.Substring(3).TrimStart("`r", "`n")
    # 逐行找下一行 ---
    $lines = $rest -split "\r?\n"
    for ($i = 0; $i -lt $lines.Count; $i++) {
        $line = $lines[$i].Trim()
        if ($line -eq "---") {
            if ($i -eq 0) { return "" }
            return ($lines[0..($i - 1)] -join "`n")
        }
    }
    return $null
}

function 测试-字段([string]$FrontMatter, [string]$字段) {
    foreach ($line in ($FrontMatter -split "\r?\n")) {
        $trimmed = $line.TrimStart()
        if ($trimmed.StartsWith($字段 + ":")) {
            return $true
        }
    }
    return $false
}

$违规 = @()
$总文件 = 0
$已校验 = 0

Get-ChildItem -Path $决策目录 -Recurse -Filter "*.md" -Force | ForEach-Object {
    $总文件++
    $文件 = $_.FullName
    try {
        $内容 = Get-Content $文件 -Raw -Encoding UTF8
    } catch {
        return
    }

    $frontmatter = 提取-FrontMatter $内容
    if ($null -eq $frontmatter) {
        $违规 += "$文件 : 缺完整 YAML frontmatter"
        return
    }
    $已校验++

    foreach ($字段 in $必填) {
        if (-not (测试-字段 $frontmatter $字段)) {
            $违规 += "$文件 : 缺字段 [$字段]"
        }
    }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] $($违规.Count) 个违规"
    $违规 | ForEach-Object { Write-Host ("  " + $_) }
    exit 1
}

Write-Host "[PASS] 决策契约：$已校验 个文件全部满足 8 必填字段"
exit 0