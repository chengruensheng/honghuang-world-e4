# 14. 格位稀缺门禁（36 格位上限）
# 决策锚：260828-格位稀缺原则（36 格位稀缺与演进约束）
# 标准：记忆类型定义的 总纲数 与 每总纲本质数 必须都为 6（笛卡尔积派生 格位总数=36），
#      阶段数 独立为 6 生命周期；严禁新增第 7 总纲 / 第 7 本质（否则格位总数超 36，违反稀缺上限）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 14/14 格位稀缺（36 格位上限） ==="

$类型定义 = "鸿蒙/基础设施 - 域/记忆承载 - 府/记忆类型-殿/类型定义-阁/类型实现-园/类型定义_核心.rs"
if (-not (Test-Path $类型定义)) {
    Write-Host "[FAIL] 类型定义文件不存在：$类型定义"
    exit 1
}

$内容 = Get-Content $类型定义 -Raw -Encoding UTF8
$违规 = @()

# 校验 总纲数 == 6
if ($内容 -match 'pub const 总纲数: usize = (\d+);') {
    $n = [int]$Matches[1]
    if ($n -ne 6) { $违规 += "总纲数 = $n，必须为 6（总纲维度永久固定）" }
} else {
    $违规 += "未找到 总纲数 常量定义"
}

# 校验 每总纲本质数 == 6
if ($内容 -match 'pub const 每总纲本质数: usize = (\d+);') {
    $n = [int]$Matches[1]
    if ($n -ne 6) { $违规 += "每总纲本质数 = $n，必须为 6（本质维度永久固定）" }
} else {
    $违规 += "未找到 每总纲本质数 常量定义"
}

# 校验 本质数 = 总纲数 * 每总纲本质数（派生）
if ($内容 -match 'pub const 本质数: usize = 总纲数 \* 每总纲本质数;') {
    # 派生定义符合预期
} else {
    $违规 += "本质数 必须为派生值 总纲数 * 每总纲本质数（不硬编码）"
}

# 校验 格位总数 = 总纲数 * 每总纲本质数（派生）
if ($内容 -match 'pub const 格位总数: usize = 总纲数 \* 每总纲本质数;') {
    # 派生定义符合预期
} else {
    $违规 += "格位总数 必须为派生值 总纲数 * 每总纲本质数（不硬编码）"
}

# 校验 阶段数 == 6（阶段为独立决策状态机，生命周期仍固定 6）
if ($内容 -match 'pub const 阶段数: usize = (\d+);') {
    $n = [int]$Matches[1]
    if ($n -ne 6) { $违规 += "阶段数 = $n，必须为 6（阶段维度永久固定）" }
} else {
    $违规 += "未找到 阶段数 常量定义"
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] $($违规.Count) 个违规"
    $违规 | ForEach-Object { Write-Host ("  " + $_) }
    exit 1
}

Write-Host "[PASS] 格位稀缺：总纲数=6 × 每总纲本质数=6 = 格位总数 36（上限未超）"
exit 0
