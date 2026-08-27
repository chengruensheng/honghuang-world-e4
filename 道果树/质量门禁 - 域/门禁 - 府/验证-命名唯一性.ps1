# 12. 命名唯一性门禁（14 号方案 · 260827-命名门禁）
# 规则 1：祖孙不同名（同一府路径下 殿/阁/园 名两两不同）
# 规则 2：同层全局唯一（全项目 -殿/-阁/-园 名各自不重复）
# 规则 3：目录名无英文（白名单：SQLite、P0-P3）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 12/12 命名唯一性 ==="
$违规 = @()

$后缀 = @("-殿", "-阁", "-园", "-数据", "-配置", "-模板", "-脚本", "-资源")

# 排除目录
function 排除($路径) {
    return $路径 -match "\\.git\\" -or $路径 -match "构建物" -or $路径 -match "node_modules"
}

# 收集所有 殿/阁/园 目录（相对路径 + 层级）
$层级目录 = @()
Get-ChildItem -Path . -Recurse -Directory -Force -ErrorAction SilentlyContinue |
    Where-Object {
        $n = $_.Name
        ($后缀 | Where-Object { $n.EndsWith($_) }) -and -not (排除 $_.FullName)
    } |
    ForEach-Object {
        $n = $_.Name
        $sfx = $后缀 | Where-Object { $n.EndsWith($_) } | Select-Object -First 1
        $名 = $n.Substring(0, $n.Length - $sfx.Length)
        $层级目录 += [PSCustomObject]@{ 路径 = $_.FullName; 名称 = $名; 后缀 = $sfx }
    }

# 规则 1：祖孙不同名 —— 对每个 -殿 目录，收集其下阁/园名，与殿名查重
$殿目录 = $层级目录 | Where-Object { $_.后缀 -eq "-殿" }
foreach ($殿 in $殿目录) {
    $名集 = @($殿.名称)
    $子名 = $层级目录 | Where-Object {
        $_.路径.StartsWith($殿.路径 + "\") -and $_.后缀 -in @("-阁", "-园")
    } | ForEach-Object { $_.名称 }
    $名集 += $子名
    $重复 = $名集 | Group-Object | Where-Object { $_.Count -gt 1 }
    if ($重复) {
        $违规 += "祖孙同名：$($殿.路径) → " + (($重复 | ForEach-Object { $_.Name }) -join ", ")
    }
}

# 规则 2：同层全局唯一
foreach ($层 in @("-殿", "-阁", "-园")) {
    $组 = $层级目录 | Where-Object { $_.后缀 -eq $层 } | Group-Object 名称 | Where-Object { $_.Count -gt 1 }
    foreach ($g in $组) {
        $违规 += "同层重复[$层]：$($g.Name) → " + (($g.Group | ForEach-Object { $_.路径 }) -join " vs ")
    }
}

# 规则 3：目录名无英文
$允许前缀 = @("SQLite", "P0", "P1", "P2", "P3")
$层级目录 | ForEach-Object {
    $名 = $_.名称
    if ($名 -match "[a-zA-Z]") {
        $白 = $允许前缀 | Where-Object { $名.StartsWith($_) }
        if (-not $白) {
            $违规 += "英文目录名：$($名)（$($_.路径)）"
        }
    }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] 命名唯一性违规：" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}
Write-Host "[PASS] 命名唯一性" -ForegroundColor Green
exit 0
