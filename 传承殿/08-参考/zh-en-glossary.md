# 术语对照表（中英）

> **元层·生态对接**——DSH / Claude Code / Codex CLI 等英文生态理解传承殿的关键。

## 一、命名约定

- 本表只翻译**核心概念**，不翻译**文档内容**
- 翻译以"达意"为优先，不强求 1:1 字面对应
- 每个术语保留中文原文 + 英文 + 简短说明 + 哲学锚
- 命名纪律：英文术语**不替代**中文术语，两者并列使用

## 二、9 根维度

| 中文 | English | 说明 | 哲学锚 |
|:--|:--|:--|:--|
| 太初 | Existence | 存在，万物之先 | 道 |
| 混沌 | Possibility | 可能性，一切未发 | 道 |
| 乾坤 | Space | 空间（容器） | 道 |
| 道韵 | Law | 法则，万物运转根本 | 道 |
| 量劫 | Time | 时间（劫数） | 道 |
| 鸿蒙 | Foundation | 地基，通用能力集中地 | 道 |
| 证道 | Verification | 测试，证明规则有效 | 道 |
| 道果树 | Build | 构建产物目录 | 道 |
| 传承殿 | Inheritance | 跨代际知识沉淀 | 道 |

## 三、6 层结构

| 中文 | English | 说明 |
|:--|:--|:--|
| 维度 | Dimension | 世界由什么构成 |
| 域 | Domain | 靠什么活着（纯目录） |
| 府 | Compound | 什么能力（lib crate） |
| 殿 | Hall | 哪些模块 |
| 阁 | Pavilion | 什么功能 |
| 园 | Garden | 怎么实现（代码园/存储园/配置园/模板园/脚本园/资源园） |

## 四、4 分类抽象

| 中文 | English | 职责 |
|:--|:--|:--|
| 道祖级 | Daoist Tier | 化要求 / 派遣 / 终裁 / 定档 |
| 圣人级 | Sage Tier | 设计 / 评审 |
| 准圣级 | Quasi-Saint Tier | 验收（六维） |
| 大罗金仙级 | Arhat Tier | 实现（按道分工） |

## 五、36 格位（6 范畴 × 6 格位）

### 6 范畴

| 中文 | English | 说明 |
|:--|:--|:--|
| 目标 | Goal | 我们要到哪里去 |
| 规则 | Rule | 必须遵守的约束 |
| 自我 | Self | 我们是谁 |
| 程序 | Process | 怎么做 |
| 世界 | World | 外部状态 |
| 经历 | Experience | 已经发生的事 |

### 每范畴 6 格位

**目标 Goal**：初心·使命（Original Aspiration）/ 任务（Task）/ 目标（Objective）/ 验收标准（Acceptance Criterion）/ 待澄清（To Clarify）/ 暂缓（Deferred）

**规则 Rule**：铁律·总纲（Iron Law）/ 标准（Standard）/ 架构（Architecture）/ 细则·解读（Interpretation）/ 例外·临时（Exception）/ 待审议（To Review）

**自我 Self**：价值观·原则（Value Principle）/ 身份（Identity）/ 底线（Bottom Line）/ 能力（Capability）/ 权限（Permission）/ 待自省（To Reflect）

**程序 Process**：环节规则（Stage Rule）/ 工作流（Workflow）/ 工具链（Toolchain）/ 接口（Interface）/ 范式（Paradigm）/ 反思（Reflection）

**世界 World**：环境·依赖（Environment）/ 结构（Structure）/ 文件（File）/ 数据（Data）/ 调用（Invocation）/ 状态（State）

**经历 Experience**：事件（Event）/ 变更（Change）/ 产物（Artifact）/ 教训（Lesson）/ 理解·记忆（Understanding）/ 错误复盘（Postmortem）

## 六、3 档位（时间维度）

| 中文 | English | 说明 |
|:--|:--|:--|
| 经档 | Canon Tier | 永久不变（儒家"经权之辨"的"经"） |
| 权档 | Discretion Tier | 当前 session 可变（"权变"） |
| 行档 | Action Tier | session 内临时（"当下行动"） |

## 七、3 源记录

| 中文 | English |
|:--|:--|
| 代码 | Code |
| LLM | LLM |
| 人类 | Human |

## 八、8 核心抽象

| 中文 | English | 哲学层 |
|:--|:--|:--|
| 可插拔 | Pluggability | 系统长期演化需局部替换 |
| 流水线 | Pipeline | 多角色分工比单角色更可靠 |
| 记忆 | Memory | 系统自我修正需结构化记忆 |
| 事件流 | Event Stream | 治理动作必须可追溯 |
| 追问 | Grilling | 治理上游要先收敛意图 |
| 规则注册表 | Rule Registry | 治理规则必须可证伪可演化 |
| 不可逆结果 | Frozen Outcome | 治理决策不可逆的工程实现 |
| 验证分离 | Verification Separation | 验证者与生成者必须分离 |

## 九、决策契约字段

| 中文 | English |
|:--|:--|
| decided_by | decided_by | 人类标识符（必填） |
| falsifiable | falsifiable | 可证伪命题 + 时间窗口 |
| implements | implements | 哲学锚（道/法/术/鉴/应/元 或五法） |
| upstream | upstream | 上游决策 ID 或 N/A |
| 命题 | proposition | falsifiable 中的具体陈述 |
| 证伪方法 | falsification_method | 机械判定 / 人工统计 / 文档审查 |
| 时间窗口 | time_window | N 个月 |

## 十、一键全验 10 项

| # | 中文 | English |
|:--|:--|:--|
| 1 | 格式 | cargo fmt --check |
| 2 | 静态分析 | cargo clippy -D warnings |
| 3 | 单元测试 | cargo test --workspace --lib |
| 4 | 编译 | cargo check --all-targets |
| 5 | 文档 | cargo doc --no-deps |
| 6 | 安全审计 | cargo audit |
| 7 | 依赖审查 | cargo deny check |
| 8 | 无 src/ 平铺 | no src/ flat directory |
| 9 | 无空目录 | no empty directory |
| 10 | 无临时目录残留 | no temp dir leftover |

## 十一、司衡 5 工程基线

| # | 中文 | English |
|:--|:--|:--|
| 1 | 确定程序是治理操作唯一执行者 | Deterministic programs are the sole executors of governance |
| 2 | 信息洪流是旧仓失败根因 | Information flood is the root cause of old warehouse failure |
| 3 | 人类只看异常，不看原始日志 | Humans only see anomalies, never raw logs |
| 4 | 可验证性约束 | Verifiability constraint |
| 5 | 治理延伸是减少 LLM 参与 | Governance extension reduces LLM participation |

## 十二、司衡 4 禁止条款

| # | 中文 | English |
|:--|:--|:--|
| 1 | LLM 不可直改知识包 | LLM must not directly modify knowledge base |
| 2 | 不可复现多 Agent 不可作治理决策依据 | Non-reproducible multi-agent interaction cannot be governance basis |
| 3 | 治理操作必入事件流 | Governance operations must enter event stream |
| 4 | 人类只通过视图介入 | Humans intervene only through views |

## 十三、DSH 6 子系统

| 中文 | English |
|:--|:--|
| session | session | append-only 会话日志 |
| system-prompt | system-prompt | 提示词组装 |
| tools | tools | 工具注册表 |
| agent | agent | Agent 接口 |
| agent-loop | agent-loop | 默认 driver |
| scope | scope | 作用域原语 |

## 十四、DSH 3 类事件 + 2 模式

| 中文 | English |
|:--|:--|
| Session 事件 | Session events | 持久化 |
| Agent 事件 | Agent events | live |
| Capability 事件 | Capability events | policy + adapter |
| Waterfall 模式 | Waterfall mode | next() 链 |
| Serial 模式 | Serial mode | 独立拦截点 |

## 十五、3 类违规

| 中文 | English |
|:--|:--|:--|
| 命名不当 | Naming violation | 借来的词 / AI 味 |
| 风格不符 | Style violation | 违反 6 风格 |
| 核心规则擅改 | Core rule modification | 未经决策契约 |
| 文档污染 | Documentation pollution | 章节级损坏 |

## 十六、3 类可证伪命题

| 中文 | English |
|:--|:--|:--|
| 机械判定 | Mechanical verification | 编程可自动测 |
| 人工统计 | Manual statistics | 人工采样统计 |
| 文档审查 | Document review | 文档对比检查 |

## 十七、commit message 类型

| 中文 | English |
|:--|:--|:--|
| feat（新功能） | feat (new feature) |
| fix（bug 修复） | fix (bug fix) |
| docs（仅文档） | docs (docs only) |
| refactor（重构） | refactor (refactor) |
| test（仅测试） | test (test only) |
| chore（构建/工具链） | chore (build/toolchain) |

## 十八、5 法 + 6 柱石 + 元三治

### 5 法

| 中文 | English |
|:--|:--|:--|
| 顺因 | Conforming to Causes |
| 有度 | Proportionate |
| 知止 | Knowing When to Stop |
| 损补 | Reducing Excess, Filling Deficit |
| 顺势 | Adapting to Circumstances |

### 6 柱石

| 中文 | English |
|:--|:--|:--|
| 道 | Tao |
| 法 | Method |
| 术 | Art |
| 鉴 | Assay |
| 应 | Response |
| 元 | Meta |

### 元三治

| 中文 | English |
|:--|:--|:--|
| 治病 | Curing Disease |
| 治益 | Curing Excess |
| 治强 | Curing Strength |

## 十九、关键动词

| 中文 | English |
|:--|:--|:--|
| 化要求 | Reify Requirements |
| 派遣 | Dispatch |
| 终裁 | Final Adjudication |
| 定档 | Finalize Version |
| 追问 | Grill |
| 派发 | Dispatch |
| 定档 | Archive |
| 化要求 | Reify |

## 二十、关键形容词

| 中文 | English |
|:--|:--|:--|
| 不可改 | Immutable |
| 不可逆 | Irreversible |
| 可证伪 | Falsifiable |
| 可演化 | Evolvable |
| 可插拔 | Pluggable |
| 可追溯 | Traceable |
| 可机械判定 | Mechanically Verifiable |

---

## 哲学锚引用

本表翻译参考：
- 司衡哲学（sgmov/sihankor 仓库，K-法-术-鉴-应-元六柱石）
- DeepSeek Harness 架构（万物皆插件 + Waterfall + frozen outcome）
- 传承殿世界观（9 根 + 6 层 + 4 分类 + 三维正交记忆）

## falsifiable

- 上线 1 个月：英文生态（DSH / Claude Code / Codex CLI）能识别传承殿关键概念
- 上线 3 个月：术语对照稳定，无重大翻译分歧

---

*传承殿 · 2026-08-26 · decided_by: 界主*
*implements: 法（让传承殿被英文生态理解）*
*falsifiable: 上线 1 个月，英文生态工具能识别 ≥ 80% 核心概念*
