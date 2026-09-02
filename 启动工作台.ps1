# 启动工作台 - 一键启动门户 Web 工作台
# 用法：双击运行，或 powershell -ExecutionPolicy Bypass -File 启动工作台.ps1
# 决策锚：260902-可用打磨（一键启动）

$ErrorActionPreference = "Continue"
Set-Location (Split-Path -Parent $PSCommandPath)

Write-Host "╔══════════════════════════════════════════════╗"
Write-Host "║  洪荒 · 世界 · 门户工作台启动器                  ║"
Write-Host "╚══════════════════════════════════════════════╝"

# 1. 检查记忆库是否存在（无库则提示先跑 CLI 播种）
if (-not (Test-Path "洪荒记忆库.sq3")) {
    Write-Host "[警告] 未找到 洪荒记忆库.sq3，工作台将显示空数据。" -ForegroundColor Yellow
    Write-Host "        可用命令初始化：cargo run -p mingling_caozuo_fu --bin 洪荒 -- 记忆 播种" -ForegroundColor Yellow
}

# 2. 检查端口占用
$占用 = Get-NetTCPConnection -LocalPort 8020 -State Listen -ErrorAction SilentlyContinue
if ($占用) {
    Write-Host "[信息] 门户服务已在 8020 端口运行" -ForegroundColor Green
    Start-Process "http://127.0.0.1:8020/"
    exit 0
}

# 3. 构建门户服务（增量构建，通常秒级）
# stderr 合并到 stdout：PowerShell 5.1 把 cargo 进度行当 NativeCommandError，用 $LASTEXITCODE 判断真实结果
Write-Host "[1/3] 构建门户服务…"
cargo build -p menhu_fuwu_fu --example 启动门户 2>&1 | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "[错误] 门户服务构建失败" -ForegroundColor Red
    exit 1
}

# 4. 启动服务（后台进程，终端可关）
Write-Host "[2/3] 启动门户服务（8020）…"
$exe = "道果树/构建物-域/debug/examples/启动门户.exe"
if (-not (Test-Path $exe)) {
    Write-Host "[错误] 门户可执行文件未生成：$exe" -ForegroundColor Red
    exit 1
}
Start-Process -FilePath $exe -WorkingDirectory (Split-Path -Parent $PSCommandPath) -WindowStyle Hidden

# 5. 等待端口就绪 + 打开浏览器
Write-Host "[3/3] 等待服务就绪…"
$ok = $false
for ($i = 0; $i -lt 20; $i++) {
    Start-Sleep -Milliseconds 500
    if (Get-NetTCPConnection -LocalPort 8020 -State Listen -ErrorAction SilentlyContinue) {
        $ok = $true
        break
    }
}
if ($ok) {
    Write-Host "门户已启动：http://127.0.0.1:8020/（浏览器已打开）" -ForegroundColor Green
    Start-Process "http://127.0.0.1:8020/"
} else {
    Write-Host "[错误] 门户服务 10 秒内未就绪，请检查端口占用或构建日志" -ForegroundColor Red
    exit 1
}
