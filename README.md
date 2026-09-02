# 洪荒 · 世界

> **原语**：本项目沿用的创世叙事词汇，仅用于命名与溯源。
> **工程理解词语**：将原语翻译为可精确执行、可验证的工程术语；本文正文一律使用工程理解词语，原语仅在《术语对照表》中保留。
> 术语对照见 `传承殿/00-宪法/术语对照表.md`。

意图驱动的自主构建系统内核。中文描述需求，多角色自主完成设计、审查、实现、验收，项目负责人仅做最终裁决，全程可观测、可追溯。

## 快速开始（5 分钟）

**前置要求**：Rust 1.75+（`rustc --version` 确认）；PowerShell 5.1+ 或 pwsh（跑门禁用）。

```bash
# 1. 构建（22 个 crate 全量编译约 1 分钟）
cargo check --workspace

# 2. 零配置演示：mock 后端完整跑通 4 分类流水线（无需 API key）
#    任务 id 任意字符串即可，体验「中文描述 → 多角色自主 → 终裁 → 记忆闭环」
cargo run -p mingling_caozuo_fu --bin 洪荒 -- run --task=我的第一个任务
#    预期输出：[LLM 道祖]…[LLM 圣人]…[LLM 大罗]…[LLM 准圣]…[道祖终裁：通过]…[产出评分 100/100]

# 3. 部署健康检查
cargo run -p mingling_caozuo_fu --bin 洪荒 -- --health
#    预期输出：健康检查通过：工作空间就绪（退出码 0）
```

**接入真实 LLM（可选，1 分钟）**：

在项目根目录创建 `.env`（模板见 `.env.example`，已存在则直接编辑）：

```bash
LLM_API_KEY=<你的 MiniMax API key>
LLM_BASE_URL=https://api.minimaxi.com/v1/chat/completions
LLM_MODEL=MiniMax-M3
```

配置后 `洪荒 run` 自动走真实 API（无需重启）。未配置 key 时 fail-loud 退出码 4，**不会**静默降级为 mock。

> 真实模式单轮 LLM 最长 120s（8000 token 输出窗口），等待期间 stderr 会实时显示
> 「道祖/圣人/大罗/准圣 思考中…」进度，属正常现象，请耐心等待，勿误判卡死。
> 若终端无进度输出（如 CI 无真实 key），会 fail-loud 退出码 4。

**想先看 Web 工作台？** 跳过命令行，直接看「运行入口」的[门户服务](#门户工作台-web-门面)。

## 项目结构

顶层模块分区（九根维度）与六层目录结构：

- **鸿蒙**（基础设施）：插件上下文 / 事件总线 / 记忆承载 / 状态共享 / 观测探针 / 日志记录 / 流水线驱动 / 任务执行 / 工具调用 / 模型连接 / 追问引擎 / 实时服务 / 任务调遣 / 版本升级 / 配置管理。
- **乾坤**（界面呈现）：命令操作（生产 CLI「洪荒」）+ 门户服务（Web 工作台）。
- **证道**（验证）：单元测试 + 质量门禁（监控 / 校验 / 评估）。
- **道韵**（法则）：规则注册表。
- **道果树**（构建产物）：编译输出目录。
- **传承殿**（知识库）：文档与决策沉淀（本目录）。

六层结构：维度 → 域 → 府（lib crate）→ 殿（模块分区）→ 阁（功能分组）→ 园（实现落点）。

## 核心机制

- **4 分类角色分工**：决策者（化要求 / 派遣 / 最终裁决 / 定档）、设计者（设计 / 评审）、验收者（六维验收）、实现者（按领域实现）。各分类独立 LLM 池，关键决策多 LLM 投票。
- **36 格位记忆模型**：6 总纲 × 6 本质 = 36 上下文槽位，四维正交（格位 × 阶段 × 档位 × 来源），永驻加载 36 行摘要。
- **决策契约**：每份决策带 decided_by / falsifiable / implements，可证伪、可追溯。
- **事件流**：append-only 哈希链，Waterfall / Serial 两类事件模式，frozen outcome 不可篡改。

## 运行入口

- 生产命令行 `洪荒`：`cargo run -p mingling_caozuo_fu --bin 洪荒 -- <命令>`
  - `--health` 部署健康检查（退出码 0 即健康）
  - `status` 工作空间实时快照
  - `run --task=<id>` 跑真实流水线（默认走真实 LLM，无 key 时 fail-loud 退出码 4；`LLM_BACKEND=mock` 走确定性 Mock）
  - `记忆 <子命令>` 记忆库读写与统计
  - 完整命令见 `洪荒 帮助`
- 门户工作台（Web 门面）：`cargo run -p menhu_fuwu_fu --example 启动门户`
  - 默认端口 8020，浏览器打开首页即工作台；接口 `/api/总览 /api/任务 /api/事件 /api/记忆 /api/仙官 /api/切面`

## 环境变量

| 变量 | 默认值 | 说明 |
|:--|:--|:--|
| `LLM_API_KEY` | 无 | 真实 LLM API key；缺失时真实模式 fail-loud（退出码 4） |
| `LLM_BASE_URL` | 无 | OpenAI 兼容端点 |
| `LLM_MODEL` | 无 | 模型名 |
| `LLM_BACKEND` | `real` | `mock` 走确定性 Mock LLM（测试/演示）；`real` 强制真实 API |
| `LLM_TIMEOUT_MS` | `120000` | 单次 LLM 调用超时（毫秒），含连接/读/写 |
| `LLM_PROGRESS` | 开 | `off` 关闭流水线实时进度输出（stderr） |
| `MENHU_PORT` / `MEMORY_DB` | `8020` / 默认库 | 门户服务端口与记忆库路径 |

## 构建与验证

- 构建：`cargo check --workspace`
- 测试：`cargo test --workspace --lib`
- 完整验证流水线：`一键全验.ps1`（17 项质量门禁：格式 / 静态分析 / 单元测试 / 编译 / 文档 / 安全审计 / 依赖审查 / 无 src 平铺 / 无空目录 / 无临时目录 / 决策契约 / 命名唯一性 / 防退化 / 格位稀缺 / 文档收割门 / 治理审计 / 标识符中文）
- CI：`.github/workflows/ci.yml` 双平台（Ubuntu / Windows）自动跑全验

## 文档导航

- 宪法：`传承殿/00-宪法/`（AGENTS.md、CHARTER.md、ARCHITECTURE.md、DECISION-CONTRACT.md、术语对照表）
- 哲学：`传承殿/01-哲学/`
- 概念：`传承殿/02-概念/`
- 决策：`传承殿/03-决策/已定/`
- 设计：`传承殿/04-设计/`

---

*洪荒 · 世界 · v0.2.0 · decided_by: 项目负责人*
