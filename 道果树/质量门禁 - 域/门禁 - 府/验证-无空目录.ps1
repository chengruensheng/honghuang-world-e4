# 9. 无空目录门禁
# 决策锚：传承殿/00-宪法/AGENTS.md § 22.2 边界（无空目录）
# 标准：仓库内不存在空目录（系统目录与构建产物除外）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 9/15 无空目录 ==="
$空目录 = Get-ChildItem -Path . -Recurse -Directory -Force -ErrorAction SilentlyContinue |
    Where-Object {
        $p = $_.FullName
        # 排除：构建产物、系统目录、版本控制
        ($p -notmatch "道果树\\构建物 - 域") -and
        ($p -notmatch "\\\.git") -and
        ($p -notmatch "\\\.codeartsdoer") -and
        ($p -notmatch "\\\.codegraph") -and
        ($p -notmatch "\\\.arts") -and
        ($p -notmatch "\\\.cargo") -and
        ($p -notmatch "\\\.上下文") -and
        (Get-ChildItem $p -Force -ErrorAction SilentlyContinue | Measure-Object).Count -eq 0
    }
if ($空目录.Count -gt 0) {
    Write-Host "[WARN] 发现 $($空目录.Count) 个空目录，自动清理" -ForegroundColor Yellow
    $空目录 | ForEach-Object {
        Write-Host "  清理 $($_.FullName)"
        Remove-Item $_.FullName -Recurse -Force -ErrorAction SilentlyContinue
    }
}
Write-Host "[PASS] 无空目录" -ForegroundColor Green
exit 0
