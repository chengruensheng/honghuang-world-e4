# 3. 单元测试门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo test --workspace --lib -- --test-threads=1 全绿
# 注意：--jobs 1 串行运行 test binary，防 23 crate 并行共享 env 变量跨 binary 污染
# （moxing_fu 与 mingling_caozuo_fu 等共享 LLM_API_KEY 等 env，并行会间歇性污染）

$ErrorActionPreference = "Stop"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 3/17 单元测试 (cargo test) ==="
cargo test --workspace --lib --jobs 1 -- --test-threads=1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 单元测试未全绿" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 单元测试" -ForegroundColor Green
exit 0
