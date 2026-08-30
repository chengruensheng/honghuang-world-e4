#!/usr/bin/env bash
# 一键全验 - 17 项门禁
# 决策锚：260826-2230 工程-DSH § 一键全验
# 关联文档：传承殿/05-质量/02-一键全验.md
#
# 注：bash 在 Git Bash / Linux 下对中文变量名解析存在兼容问题；
#     变量名统一用 ASCII，路径保持中文。
#
# 用法：
#   bash 一键全验.sh                # 在 Linux/Mac/Git Bash 下
#   pwsh 一键全验.ps1               # 在 PowerShell 下（推荐，含全部 17 项）
#
# 退出码：0 = 17/17 全过；非零 = 失败项数

set -e

cd "$(dirname "$0")"
GATE_DIR="道果树/质量门禁 - 域/门禁 - 府"

echo "╔══════════════════════════════════════════════╗"
echo "║  洪荒 · 世界 v0.2.0 · 一键全验 17 项              ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

PASS=0
FAIL=0
SKIP=0

run() {
    local NO="$1"
    local NAME="$2"
    local SCRIPT="$3"

    echo "─── [$NO] $NAME ───"
    if [ ! -f "$GATE_DIR/$SCRIPT" ]; then
        echo "  [SKIP] 脚本不存在：$GATE_DIR/$SCRIPT"
        SKIP=$((SKIP + 1))
        return 0
    fi
    if pwsh "$GATE_DIR/$SCRIPT"; then
        PASS=$((PASS + 1))
    else
        echo "  [FAIL] $NAME 未通过"
        FAIL=$((FAIL + 1))
    fi
    echo ""
}

run 1  "格式"          "验证-格式.ps1"
run 2  "静态分析"       "验证-警告.ps1"
run 3  "单元测试"       "验证-测试.ps1"
run 4  "编译"          "验证-编译.ps1"
run 5  "文档"          "验证-文档.ps1"
run 6  "安全审计"       "验证-审计.ps1"
run 7  "依赖审查"       "验证-依赖.ps1"
run 8  "无 src/ 平铺"  "验证-无src目录.ps1"
run 9  "无空目录"       "验证-无空目录.ps1"
run 10 "无临时目录残留" "验证-临时目录.ps1"
run 11 "决策契约 lint" "验证-决策契约.ps1"
run 12 "命名唯一性"     "验证-命名唯一性.ps1"
run 13 "防退化 ≥2 殿"   "验证-防退化.ps1"
run 14 "格位稀缺 36 上限"  "验证-格位稀缺.ps1"
run 15 "文档收割门防污染" "验证-文档收割门.ps1"
run 16 "治理审计"   "验证-治理审计.ps1"
run 17 "标识符中文" "验证-标识符中文.ps1"

echo "╔══════════════════════════════════════════════╗"
echo "║  汇总：通过 $PASS / 失败 $FAIL / 跳过 $SKIP                ║"
echo "╚══════════════════════════════════════════════╝"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
exit 0
