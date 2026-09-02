# 2. 静态分析门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 标准：cargo clippy --workspace -- -D warnings 零警告

# stderr 合并到 stdout + Continue 模式：PowerShell 5.1 把外部命令写 stderr 的进度行当
# NativeCommandError，与 $ErrorActionPreference="Stop" 叠加会把「正常完成」误判为异常
# （全验 2/4 项曾因此误 FAIL）。Continue = 错误仅显示不终止，$LASTEXITCODE 仍可用。
$ErrorActionPreference = "Continue"
Set-Location (Split-Path -Parent $PSScriptRoot)
Set-Location "../.."

Write-Host "=== 2/17 静态分析 (cargo clippy) ==="
cargo clippy --workspace --all-targets -- -D warnings 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[FAIL] clippy 警告未清零" -ForegroundColor Red
    exit 1
}
Write-Host "[PASS] 静态分析" -ForegroundColor Green
exit 0
