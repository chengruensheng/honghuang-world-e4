# 8. 无 src/ 平铺门禁
# 决策锚：传承殿/00-宪法/AGENTS.md § 18（严禁 src/ 平铺）
# 标准：项目根目录严禁 src/；crate 内部只允许 src/lib.rs 或 src/模块名/mod.rs

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 8/10 无 src/ 平铺 ==="
$违规 = @()

# 检查项目根目录严禁 src/
$根Src = Join-Path (Get-Location) "src"
if (Test-Path $根Src) {
    $违规 += "项目根目录存在 src/：$根Src"
}

# 检查所有 crate 内部：只允许 src/lib.rs 或 src/模块名/mod.rs
$所有入口 = Get-ChildItem -Path . -Recurse -Filter "Cargo.toml" -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -notmatch "道果树\\构建物 - 域" }
foreach ($cargo in $所有入口) {
    $crate根 = $cargo.DirectoryName
    $src目录 = Join-Path $crate根 "src"
    if (-not (Test-Path $src目录)) { continue }
    # 当前约定：crate 使用 入口.rs 而非 src/lib.rs，故 src/ 应为空或不存在
    $src内容 = Get-ChildItem $src目录 -Recurse -Force -ErrorAction SilentlyContinue
    if ($src内容.Count -gt 0) {
        $违规 += "$crate根 内 src/ 存在内容（应使用 入口.rs）"
    }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] src/ 平铺违规：" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}
Write-Host "[PASS] 无 src/ 平铺" -ForegroundColor Green
exit 0
