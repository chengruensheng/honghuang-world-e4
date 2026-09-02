# 3. 单元测试门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo test --workspace --lib -- --test-threads=1 全绿
# 注意：--jobs 1 串行运行 test binary，防 23 crate 并行共享 env 变量跨 binary 污染
# （moxing_fu 与 mingling_caozuo_fu 等共享 LLM_API_KEY 等 env，并行会间歇性污染）
# 补充：显式测试 规则-府 的 生成验证入口 example（固定挂载点，LLM 落盘代码的真实执行载体）
#       ——--lib 不覆盖 example；该 example 是「真任务 --落盘」确定性交付的验证闸门，必须纳入回归。

# stderr 合并到 stdout + Continue 模式：规避 PowerShell 5.1 NativeCommandError 误判（同 2 项修复）
$ErrorActionPreference = "Continue"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 3/17 单元测试 (cargo test) ==="
cargo test --workspace --lib --jobs 1 -- --test-threads=1 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 单元测试未全绿" -ForegroundColor Red
    exit 1
}
cargo test -p guize_fu --example 生成验证入口 --jobs 1 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] 生成验证入口 example 测试未通过" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 单元测试（含 生成验证入口 example）" -ForegroundColor Green
exit 0
