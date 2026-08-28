# 传承殿 · v3 + v4 完成 + Round 1-10 收口

> **项目代号**：洪荒 · 世界
> **v3 状态**：✅ 10 阶段全部 phase end + tag v0.10.0
> **v4 状态**：✅ 阶段 15-18 完成 + Round 8 真实云端 + Round 9 CI matrix + Round 10 端到端验证
> **tags**：v0.10.0 · v1.0.0（b6f5241 旧基线）· **V0.0.0.199999 待拍板**（commit 3f12827/c7de41d 新基线）— 4 段数字版本号反映「早期迭代未到 1.0」

## 1. 工作空间统计（当前 head：bd89852）

- workspace：**32 crate**（21 府级 + 11 工具，含 世界 lib）
- 命名约定：目录全中文 + lib 名拼音 + 无 src/ 平铺 + 21/21 府级 crate 统一六层风格
- 测试：**339 项**全过（基线 188 + Round 1-10 增量 63 + SQLite 持久化/36格位闭环/记忆工具/模型连接/追问边界/玉玺+世界事实+三档投影+播种格位+地道精炼+会话记录+事件流+并发互斥+完成度刻度+格位统计+世界快照+任务收尾+分发器边界 增量 88）
- 一键全验：**15/15** 全绿（含决策契约 / 命名唯一 / 防退化 ≥2 殿 / 格位稀缺 36 上限 / 文档收割门防污染；破坏性验证缩水设计稿→FAIL 拦截）
- cargo clippy：零警告
- 记忆闭环（5bb3055 → 451f77a）：关键词表可配置 → 永驻加载36行摘要 → 持久读取 → 记忆工具（写/读/查/摘）→ 任务记忆闭环 + 文件库编号覆盖修复 → 写入_按格位 → 格位级仓库分区深挖 → 六工具闭环演示 → 种子来源标注修正 → 关键词表六范畴防回归 → 流水线记忆闭环端到端（集成链路）→ 确认格位记忆（盖玉玺：人类确认=来源人类+decided_by界主，覆盖链头）→ 登记世界事实（世界范畴=代码写，测试数自动落库防漂移）→ 九件套玉玺链演练（记忆工具演练：写/读/按格位/确认玉玺/世界事实/读仓/查全部/永驻摘要/任务闭环，退出0）→ 读取_三档投影（档位维度首次读取侧生效：首因=经档最早/近因=权档最新/会话=行档降序）→ 播种格位36（无人开发：AI 自主补齐 23 个本质×阶段合法格位，来源LLM无玉玺可执行）+ 分发记忆命令（记忆 <三档|查|播种>）→ 地道精炼（执行结果自动提炼 教训/机制/波及 → 经历/归档·实施·验收，信号词规则 E盘自主设计；分发 记忆 精炼 <执行结果> [库路径]）→ 会话记录（工作记忆完整保留：记会话 逐行写入 经历/实施 含任务锚点；查会话 按锚点读回；终点归档 查会话→地道精炼；分发 记忆 会话 <查|归档>）→ 事件流（append-only 时序事实：存储层 trait+SQLite AUTOINCREMENT 互斥写；分发 记忆 事件 <记录|读取|全部>）→ 完成度刻度（执行收尾 1-5 自评+依据 → 经历/验收；刻度非法拒绝；分发 记忆 完成度 <刻度> <依据>）→ 格位统计（每格位条目数+总条数+最稀疏/最密集，承接 格位稀缺 契约；分发 记忆 统计）→ 永驻摘要 36 行全齐（非法格位标注本质不可用，与合法空格位区分）→ 世界快照（测试数/套件数自动登记世界/验收，decided_by=扫描；分发 记忆 世界快照）→ 任务收尾（完成度自评+终点归档一步到位；分发 记忆 收尾 <任务> <结果> <刻度>）→ 分发器参数边界（世界快照套件数空检查 / 兜底用法含收尾 / 帮助列表全命令；+2 测试 11 路径失败码矩阵）
- 追问引擎：4 问题 enum（道二/顺因/有度/知止）+ 3 mock LLM 投票（70 阈值一致率，2/1 分歧或全弃权升级道祖终裁；边界测试：全弃权/反对派 2 票）
- 模型连接：真实云端 LLM 接入（minimax，Bearer auth + OpenAI 兼容），env 三级回退 LLM_* → DEEPSEEK_URL/DEEPSEEK_MODEL → OpenAI（空串过滤；界主只设 LLM_API_KEY 即零配置解锁）
- 排障提示：自检瞬时 12/13 且报「单元测试」失败、详细为 rust-lld permission denied → 是 Windows exe 文件锁（残留进程/Defender 扫描），先 `cargo build -p mingling_caozuo_fu --example 自检入口` 后串行重跑即恢复；勿并发跑两个自检实例、勿用管道吞 cargo 退出码
- 决策契约：**9 个文件**入 03-决策/已定/（260826×4 + 260827×5）
- 10-地基 入稿方案：**24 份**
- 真实云端 LLM 调用：✅ minimax API（Bearer auth + OpenAI 兼容）
- 端到端自开发能力：✅ Round 10 验证（minimax 生成 乾坤/工具-hello-府 → cargo check → 1 测试通过）

## 2. v3 完整阶段（10 phase end · 188 测试基线）

| 阶段 | commit | 内容 |
|:--|:--|:--|
| 1 | 509e39f | workspace + 12 crate + 10 项门禁 |
| 2 | a5d1cfd | 事件流 + 5 类工具 + 插件上下文 |
| 3 | ac2c937 | 36 格位 + 3 维正交 + 加载档位 |
| 4 | 5c9d68f | RULE_REGISTRY 14 条 + 决策契约 |
| 5 | 882feec | 4 分类状态机 + 4 张角色卡 |
| 6 | 442a076 | 3 CLI 命令 + e2e 拒绝路径 |
| 7 | 3678d2a → 5a64d7e | moxing_fu + 5 工具 + e2e + phase end |
| 8 | dd7a243 → 02942f1 | 追问引擎 + 投票 + 升级 |
| 9 | 0e74bba → 39d7c64 | 监控 + 4 级告警 + 4 级应急 + phase end |
| 10 | 577fa2e → b9712cb | 升级-府 + 复用经验 + phase end |
| 11 | 282845e | SQLite 后端 + 100% 恢复 |
| 12 | 22a3bbe | 决策契约接入写入路径 |
| 13 | ade9ef0 | 架构校验脚本（PowerShell） |

## 3. v4 阶段 15-18 + Round 1-10（251 测试基线 + 13/13 门禁）

| 阶段 | commit | 内容 |
|:--|:--|:--|
| 15 | 66a9203 | 性能基准（chengben_fu 5 基准 + 5 测试） |
| 16 | bc1e209 | 架构校验 Rust 化（jianyan_gongju 11/11 → 15/15 + 命名唯一性 + 防退化） |
| 17 | b0391c7 + 15925f7 + 6d34959 + 3f12827 | mock HTTP（shishi_fu）+ 真实云端 LLM 接入（moxing_fu Bearer + base URL + 4 分类 + 优雅降级）+ mingling_caozuo_fu 调用方集成（真实-后端-选择-殿） |
| 18 | 87d7621 | 多 agent 协同（diaoqian_fu 7 测试） |
| Round 1-5 | 9211aba / 700fe62 / 4612307 / 73c0236 / d9530c5 / 2eef3ff | 21/21 府级 crate 切分为 ≥2 殿（调遣/评估/状态共享/观测探针/单元测试/配置管理/日志记录/世界/命令操作补第 2 殿/实时补第 2 殿） |
| Round 6 | 99873f0 | 修复祖孙语义重复（写-操作-阁→写入-方法-阁、查询-操作-阁→查询-方法-阁、计数-操作-阁→计数-方法-阁） |
| Round 7 | c6b8d6e | 防退化门禁（≥2 殿校验）+ 4 决策入契约（命名门禁/统一风格/平铺-取舍/祖孙语义重复判据）+ 哲学录入（00-宪法 §1.5 / 01-哲学 / 04-设计） |
| Round 8 | 6d34959 | 真实云端 LLM 接入（环境配置-殿 + 从环境变量构造() → Option<LLM池> + Bearer auth + 4 分类各自模型覆盖 + 超时配置） |
| Round 9 | 27c6267 + 3f12827 | CI 13 项门禁 + GitHub Actions matrix（ubuntu+windows）+ moxing_fu 调用方集成 |
| Round 10 | d673d8b | 端到端自开发能力验证（minimax LLM 生成 乾坤/工具-hello-府 + cargo check + 1 测试通过 + commit） |

## 4. 13 项一键全验门禁

1. 格式（cargo fmt --check）
2. 静态分析（cargo clippy -D warnings）
3. 单元测试（cargo test --workspace）
4. 编译（cargo check）
5. 文档（cargo doc）
6. 安全审计（cargo audit）
7. 依赖审查（cargo deny）
8. 无 src/ 平铺
9. 无空目录
10. 无临时目录残留
11. 决策契约 lint（9 文件 8 必填字段）
12. 命名唯一性（祖孙不同名 + 同层全局唯一 + 无英文白名单 SQLite/P0-P3）
13. 防退化 ≥2 殿（jianyan_gongju 跑全部第 15 项）

## 5. 复用经验（v3 10 条 + v4 5 条 + Round 1-10 8 条）

### v3 阶段 1-10 复用

1. **Cargo workspace** + 12 crate + 入口.rs 替代 src/lib.rs
2. **ureq + serde_json** + 简化的 JSON 行式持久化
3. **36 格位** + 3 维正交 + MUST/MIXED/OPTIONAL 加载档位
4. **RULE_REGISTRY 14 条** + 决策契约字段校验
5. **4 分类状态机** + 角色卡 + 分类机械判定
6. **CLI 命令 trait** + 状态机调用
7. **HTTP 连接 trait** + LLM 池 + 4 分类池配置
8. **追问引擎** + 关键词映射 + 3 mock LLM 投票
9. **4 类指标 + 4 级告警 + 4 级应急** + 升级路径映射
10. **3 类升级策略** + 复用经验入 36 格位

### v4 阶段 15-18 复用

11. **chengben_fu** 性能基准（5 项基线 + 内存阈值）
12. **jianyan_gongju** 架构校验 Rust 化（15/15 全绿）
13. **shishi_fu** mock HTTP server（std::net + 4 分类 JSON 响应）
14. **diaoqian_fu** 多 agent 协同（顺序 FIFO + 错误隔离）
15. **mingling_caozuo_fu e2e** 真实 HTTP 走 mock server + minimax 真实云端

### Round 1-10 复用

16. **从环境变量构造()** — moxing_fu 模式：env var → Option<T> 优雅降级（无 key → None → fallback）
17. **env_lock 串行化** — cargo test 默认并行会污染全局 env，用 `OnceLock<Mutex>` 串行化 env var 测试
18. **连接抽象 enum** — 绕开 `LLM调用器<C>` 单态化限制（Mock 与 HTTP 连接类型不同）
19. **祖孙三层命名语义判据** — 阁=方法契约、园=实现实例，核心名不同（中间层不能 `阁名 = 园名`）
20. **GitHub Actions matrix** — ubuntu + windows 双 runner，含 fmt/clippy/test/一键全验
21. **子代理并行分发** — Prompt 用文件传避免 JS 转义；bg 跑后 git log 验证 commit；亲自验收不省
22. **Node https + 中文路径 + Windows** — 用 curl + 文件 body 模式替代 Node https（更稳）
23. **LLM 引导式输出** — minimax 默认 thinking 模式吃大量 token，需明确 "no thinking" + 大 max_tokens

## 6. Round 10 端到端测试（自开发能力首次验证）

**目标**：项目能「让 LLM 自己写一个新 crate + 自动 commit」

**流程**：
1. 用 minimax LLM（minimaxi.com + MiniMax-M3）接收任务描述 + 命名规范 + 模板
2. 解析响应中的 `=== FILE: 路径 ===` 代码块
3. 写入目标 crate 目录
4. cargo check + cargo test（251 测试 +1 = 252 通过）
5. cargo fmt + cargo clippy -D warnings（零警告）
6. 一键全验 13/13 全绿
7. git commit 中文 message

**结果**（commit `d673d8b`）：
- minimax 5.3s + 321 tokens 生成 2 文件（Cargo.toml + 入口.rs）
- 输出 1 殿 1 阁（未达 ≥2 殿 ≥2 阁 ≥1 园 6 层标准，Round 11+ 工作）
- cargo check + cargo test 1 passed
- 写 `乾坤/工具-hello-府/` + workspace members + commit

**端到端链路 100% 工作**，证明项目具备「自开发自己」雏形。

## 7. v4 路线收口（24 号决策待界主拍板）

24 号决策文件 `传承殿/03-决策/已定/260827-v1收口.md` 待拍板：

- **方案 A（推荐）**：打 `v1.0.1` tag（基于新基线 251 测试 + 13/13 + 真实云端 + CI matrix + 21/21 六层统一 + 9 决策契约）
- 方案 B：不推荐 v2.0.0（无破坏性变更）
- 方案 C：不推荐延后（CI 已验证代码应已 tag）

## 8. 命名哲学

每阶段命名遵循"哲学层 → 概念层 → 工程层"三段推导：
- 性能 → 评估 → 评估-府（chengben_fu）
- 架构 → 校验 → 校验-工具（jianyan_gongju）
- 实时 → 模拟 → 实时-府（shishi_fu）
- 调遣 → 调度 → 调遣-府（diaoqian_fu）
- 接入 → 真实 → 环境配置-殿（Round 8）
- 集成 → 选择 → 真实-后端-选择-殿（Round 9）

## 9. 下一步候选

1. 界主拍板 24 号 v1.0.1 → git tag + 推 origin → CI 云端首跑
2. Round 11 增强 LLM 端到端（max_tokens 8000-16000 + retry + 完整 6 层）
3. v4 阶段 19 收口（README v1.0.1 章节 + 部署文档 + ≥10 真实任务跑通）
4. ci.yml 加 lefthook pre-commit 检查

---

*Round 10 完成 · 2026-08-27 · 23 crate + 251 测试 + 13/13 全绿 + 9 决策契约 + 真实云端 LLM + 端到端自开发验证*
*待界主拍板：24 号 v1.0.1 release 决策*
## 10. 可用阶段

✅ **V0.0.1.0 达成**（2026-08-27 · 24 号决策 4 条判定标准全部 ✅）

### 判定标准达成证据

| 标准 | 状态 | 证据 |
|:--|:--|:--|
| 1. 端到端自开发 | ✅ | 工具-加密-府 6 层结构 + 13/13 门禁（commit 6986f6c）|
| 2. ≥10 真实任务 | ✅ | 10 个工具 crate（加密/日志/配置/时钟/路径/随机/压缩/哈希/正则/时间戳）· 280 测试 + 13/13 门禁 |
| 3. README 可用阶段 | ✅ | 本章节完整 |
| 4. AI meta 验证 | ✅ | 自检 13/13 + 5 必读 + 7 检查（commit 33c19c3）|

### 当前基线

- **crate 数**：31（21 府级 + 10 工具）
- **测试**：280 项全过
- **门禁**：13/13 全绿（格式/clippy/测试/编译/文档/审计/依赖/无src/无空目录/无临时/决策契约/命名唯一/防退化）
- **决策契约**：10 个决策文件 8 必填字段
- **命名**：祖孙三层语义判据 + 同层全局唯一 + 无英文（白名单 SQLite/P0-P3）

### 使用方式

一键全验：powershell -NoProfile -ExecutionPolicy Bypass -File 一键全验.ps1

AI 助手自检（13 项门禁）：cargo run -p mingling_caozuo_fu --example 自检入口 -- 自检

### 里程碑

- V0.0.1.0 tag 已打（24 号决策 传承殿/03-决策/已定/260827-v1收口.md）
