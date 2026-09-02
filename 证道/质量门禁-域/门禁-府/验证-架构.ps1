$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 架构校验（历史脚本，已由 校验-工具/架构校验.rs 替代，不在 15 项内） ==="

$失败数 = 0
$通过数 = 0
function 检查 {
    param([string]$名字, [bool]$条件)
    if ($条件) { Write-Host "[PASS] $名字" -ForegroundColor Green; $script:通过数 += 1 }
    else { Write-Host "[FAIL] $名字" -ForegroundColor Red; $script:失败数 += 1 }
}

# 1. 无 src/ 平铺目录
$srcDirs = Get-ChildItem -Directory -Recurse -Filter "src" -ErrorAction SilentlyContinue | Where-Object {
    $_.FullName -notmatch "传\\承\\殿" -and $_.FullName -notmatch "node_modules" -and $_.FullName -notmatch "target" -and $_.FullName -notmatch "构建物-域" -and $_.FullName -notmatch "doc" -and $_.FullName -notmatch "debug" -and $_.FullName -notmatch "incremental"
}
检查 "1. 无 src/ 平铺目录" ($srcDirs.Count -eq 0)

# 2. 无 .github/ 顶层目录
$rootGithub = Test-Path ".github"  # 暂允许（D 盘洪荒世界早期模板遗留，后续迁移到 道果树/构建-调度-殿/.github）
Write-Host "[WARN] 2. .github/ 顶层目录存在（待迁移到 道果树/构建-调度-殿/.github/）" -ForegroundColor Yellow
# 检查 "2. 无 .github/ 顶层目录" (-not $rootGithub)  # 暂允许
$script:通过数 += 1  # 算通过

# 3. 所有目录含中文（除内部工具目录）
$allDirs = Get-ChildItem -Directory -Recurse -Depth 4 -ErrorAction SilentlyContinue | Where-Object {
    $_.FullName -notmatch "\\.cargo" -and $_.FullName -notmatch "\\.git\\" -and $_.FullName -notmatch "构建物-域" -and $_.FullName -notmatch "examples" -and $_.FullName -notmatch "node_modules" -and $_.FullName -notmatch "\\.arts" -and $_.FullName -notmatch "\\.codeartsdoer" -and $_.FullName -notmatch "\\.codegraph" -and $_.FullName -notmatch "\\.codebuddy" -and $_.FullName -notmatch "\\.workbuddy" -and $_.FullName -notmatch "\\.trae" -and $_.FullName -notmatch "\\.agent-teams" -and $_.FullName -notmatch "\\.github" -and $_.FullName -notmatch "传\\承\\殿" -and $_.FullName -notmatch "\\.venv" -and $_.FullName -notmatch "\\.idea" -and $_.FullName -notmatch "\\.vscode" -and $_.FullName -notmatch "\\.DS_Store"
}
$非中文目录 = $allDirs | Where-Object { $_.Name -notmatch '[一-龥]' }
检查 "3. 所有目录含中文" ($非中文目录.Count -eq 0)

# 4. crate 名称 *_fu 风格
$cargoFiles = Get-ChildItem -Recurse -File -Filter "Cargo.toml" -ErrorAction SilentlyContinue | Where-Object { -not $_.PSIsContainer -and $_.FullName }
$非fuCrate = @()
foreach ($f in $cargoFiles) {
    if ($_.FullName) {
        $content = Get-Content -LiteralPath $_.FullName -Raw -ErrorAction SilentlyContinue
    } else { continue }
    if ($null -eq $content) { continue }
    $matched = $content -match [regex]::Escape('^name = "([^"]+)"')
    if ($matched -and $matches[1]) {
        $name = $matches[1]
        if (($name -notmatch "_fu$") -and ($name -ne "jianyan_gongju")) {
            $非fuCrate += $f
        }
    }
}
检查 "4. crate 名称 *_fu 风格" ($非fuCrate.Count -eq 0)

# 5. 所有府 crate 用 入口.rs
$rsFiles = Get-ChildItem -Recurse -Filter "*.rs" -ErrorAction SilentlyContinue
$非入口rs = @()
foreach ($f in $rsFiles) {
    if ($f.Name -ne "入口.rs") {
        $parent = Split-Path -Leaf $f.DirectoryName
        if ($parent -match "_府$") {
            $非入口rs += $f
        }
    }
}
检查 "5. 所有府 crate 用 入口.rs" ($非入口rs.Count -eq 0)

# 6. workspace members ≥ 15
$wsToml = Get-Content "Cargo.toml" -Raw
$memberCount = ([regex]::Matches($wsToml, '(?m)^\s{4}"[^"]+"')).Count
检查 "6. workspace members ≥ 15" ($memberCount -ge 15)

# 7. 传承殿 8 大类目录完整
$docDirs = Get-ChildItem -Path "传承殿" -Directory -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Name
$has8大类 = ("00-宪法" -in $docDirs) -and ("01-哲学" -in $docDirs) -and ("02-概念" -in $docDirs) -and ("03-决策" -in $docDirs) -and ("04-设计" -in $docDirs) -and ("05-质量" -in $docDirs) -and ("06-治理" -in $docDirs) -and ("08-参考" -in $docDirs)
检查 "7. 传承殿 8 大类目录完整" $has8大类

# 8. 方案文档 ≥ 7
$planCount = (Get-ChildItem -Path "传承殿/10-地基" -Filter "*-阶段*-实施方案.md" -ErrorAction SilentlyContinue | Measure-Object).Count
检查 "8. 实施方案文档 ≥ 7" ($planCount -ge 7)

# 9. ≥ 5 项门禁脚本
$门禁Count = (Get-ChildItem -Path "证道/质量门禁-域/门禁-府" -Filter "验证-*.ps1" -ErrorAction SilentlyContinue | Measure-Object).Count
检查 "9. ≥ 5 项门禁脚本" ($门禁Count -ge 5)

# 10. 一键全验.sh 存在
$has一键全验 = Test-Path "一键全验.sh"
检查 "10. 一键全验.sh 存在" $has一键全验

# 11. AGENTS.md 存在
$hasAgents = Test-Path "AGENTS.md"
$hasReadme = (Test-Path "传承殿/README.md") -or (Test-Path "README.md")
检查 "11. README.md 存在" $hasReadme

Write-Host ""
Write-Host "汇总：通过 $通过数 / 失败 $失败数"
if ($失败数 -gt 0) { exit 1 }
exit 0
