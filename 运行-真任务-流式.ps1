# 洪荒 · 真实 LLM 流水线 · 真任务（流式完整输出）
# 双击本脚本运行，或右键「使用 PowerShell 运行」。
# 你会看到：进度 + 每个角色完整产出（道祖/圣人/大罗/准圣 全文）+ 终裁。

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

& "E:\洪荒 - 世界\道果树\构建物-域\debug\洪荒.exe" 真任务

Write-Host ""
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host "  任务执行完毕，退出码: $LASTEXITCODE" -ForegroundColor Cyan
Write-Host "======================================================" -ForegroundColor Cyan
Write-Host ""
Read-Host "按回车关闭窗口"
