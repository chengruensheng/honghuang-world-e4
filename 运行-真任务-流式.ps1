# 洪荒 · 真实 LLM 流水线 · 真任务（流式完整输出）
# 双击本脚本运行，或右键「使用 PowerShell 运行」。
# 你会看到：进度 + 每个角色完整产出（道祖/圣人/大罗/准圣 全文）+ 终裁。
#
# 两种模式：
#   默认      —— 四角色接力产出（展示完整流式过程，不落盘）
#   --落盘   —— 确定性自举流水线：LLM 代码真实落盘 + cargo 验证 + 准圣真实验收
#               （根治「0/14 打回」：代码有真实落盘与编译证据才交付）
# 脚本启动后按 D 键选择落盘模式，回车走默认模式。

$host.UI.RawUI.WindowTitle = "洪荒 · 真实 LLM · 真任务（流式）"
Set-Location "E:\洪荒 - 世界"

Write-Host ""
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host "  洪荒 · 真实 LLM 流水线 · 真任务" -ForegroundColor Cyan
Write-Host "  默认工单：两数求和（整数加法，中文标识符+中文测试）" -ForegroundColor Cyan
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "说明：每个角色完成后，完整产出会实时显示（非仅字数）" -ForegroundColor Yellow
Write-Host "      若长时间停在「思考中…」属正常（真实 LLM 单轮最长 120s）" -ForegroundColor Yellow
Write-Host ""
Write-Host "选择模式：" -ForegroundColor Green
Write-Host "  [回车] 流式演示（四角色接力产出，不落盘）" -ForegroundColor Green
Write-Host "  [D]    确定性落盘（代码真实写文件 + cargo 验证，可交付）" -ForegroundColor Green
$选择 = Read-Host "输入 D 或直接回车"

$参数 = @("真任务")
if ($选择 -match "^[Dd]$") {
    $参数 += "--落盘"
    Write-Host ""
    Write-Host "已选：确定性落盘模式 —— LLM 代码将真实落盘到 道韵/自举验证-园/ 并跑 cargo check 验证" -ForegroundColor Magenta
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "已选：流式演示模式 —— 四角色接力产出（不落盘）" -ForegroundColor Magenta
    Write-Host ""
}

& "E:\洪荒 - 世界\道果树\构建物-域\debug\洪荒.exe" @参数

Write-Host ""
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host "  任务执行完毕，退出码: $LASTEXITCODE" -ForegroundColor Cyan
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "完整产出已同步落盘：.workbuddy\流水线流式日志.txt（可随时回看）" -ForegroundColor Yellow
Read-Host "按回车关闭窗口"
