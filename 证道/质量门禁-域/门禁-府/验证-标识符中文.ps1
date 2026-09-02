# 17. 标识符中文门禁（fn 名无英文，白名单 Rust 强制/标准词/历史债）
# 决策锚：中文硬约束（项目哲学根基，违反=界主删项目推倒重来）
# 标准：.rs 文件的 fn 名不含英文；白名单=Rust 强制（main/default）+ 标准词（LLM/crc32/sha256/sqlite_/fmt/manifest 等）
#       + 历史债（run_pipeline_with_backend/build_mock_pool 等，待架构级清理后逐批收缩）
# 新增业务英文 fn 名（白名单外）→ 违规 exit 1

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 17/17 标识符中文（fn 名无英文） ==="

# 白名单：Rust 强制 + 标准词（历史债已全部中文化，仅剩 Rust 强制与标准词）
$白名单 = @(
    # Rust 强制 / 标准库
    "main", "default", "fmt",
    # 标准词 / 工具名（算法 / 网络 / 模型）
    "crc32", "sha256", "curl_minimax", "manifest",
    "LLM", "llm"
)

$违规 = @()
$目录们 = @("鸿蒙", "乾坤", "证道", "道韵")

foreach ($目录 in $目录们) {
    Get-ChildItem -Path $目录 -Recurse -Filter "*.rs" -Force -ErrorAction SilentlyContinue |
        Where-Object { $_.FullName -notmatch "构建物" } |
        ForEach-Object {
            $文件 = $_.FullName
            foreach ($行 in (Get-Content $文件 -Encoding UTF8)) {
                # 提取 fn 名（pub fn X / fn X，X 为 ASCII 标识符）
                if ($行 -match '(?:pubs+)?fns+([a-zA-Z_][a-zA-Z_0-9]*)s*[<(]') {
                    $名 = $Matches[1]
                    if ($名 -notin $白名单) {
                        $违规 += "英文 fn 名：$名（$文件）"
                    }
                }
            }
        }
}

if ($违规.Count -gt 0) {
    Write-Host "[FAIL] 标识符中文违规：" -ForegroundColor Red
    $违规 | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    exit 1
}
Write-Host "[PASS] 标识符中文：fn 名全部中文或白名单（Rust 强制/标准词/历史债）"
exit 0
