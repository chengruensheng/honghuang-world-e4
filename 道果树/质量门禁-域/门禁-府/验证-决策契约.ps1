# 11. 决策契约 lint
# 决策锚：260826-2230 工程-DSH § 一键全验 + 260826-2220 治理-司衡
# 标准：传承殿/03-决策/**/*.md 的 YAML frontmatter 必填 8 字段
#      （id/title/stage/decided_by/falsifiable/upstream/implements/decided_at）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 11/11 决策契约 lint ==="

$必填 = @(
    "id", "title", "stage", "decided_by",
    "falsifiable", "upstream", "implements", "decided_at"
)

$决策目录 = "传承殿/03-决策"
if (-not (Test-Path $决策目录)) {
    Write-Host "[SKIP] 决策目录不存在：$决策目录" -ForegroundColor Yellow
    Write-Host "[PASS] 决策契约"
    exit 0
}

$违规 = @()
$总文件 = 0
$已校验 = 0

# 递归找所有 .md
Get-ChildItem -Path $决策目录 -Recurse -Filter "*.md" -Force | ForEach-Object {
    $总文件++
    $文件 = $_.FullName
    $内容 = Get-Content $文件 -Raw -Encoding UTF8

    # 取 frontmatter（第一个 --- 至第二个 --- 之间）
    if ($内容 -notmatch '^---s*$') {
        # 无 frontmatter 开头，跳过（不是决策文档格式）
        return
    }
    $parts = $内容 -split '^---', 3
    if ($parts.Count -lt 3) {
        # 无完整 frontmatter
        $违规 += "$文件 : 缺完整 YAML frontmatter（需 --- 开头与结尾）"
        return
    }
    $已校验++
    $frontmatter = $parts[1]

    foreach ($字段 in $必填) {
        $模式 = "^$($字段):"  # 行首匹配 key:
        if ($frontmatter -notmatch "(?m)$模式") {
            $违规 += "$文件 : 缺字段 [$字段]"
        }
    }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] $($违规.Count) 个决策契约违规（$已校验/$总文件 文件已校验）" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}

Write-Host "[PASS] 决策契约：$已校验 个文件全部满足 8 必填字段" -ForegroundColor Green
exit 0
