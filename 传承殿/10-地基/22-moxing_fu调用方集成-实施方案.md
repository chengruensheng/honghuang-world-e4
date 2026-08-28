# 22-moxing_fu 调用方集成（Round 9）

> **决策锚**：260827-moxing_fu调用方集成（Round 9，承接 6d34959 「真实云端 LLM 接入」）
> **关联文档**：21-真实云端LLM接入-实施方案.md + 14-命名唯一性门禁-实施方案.md + 20-祖孙语义重复修复-实施方案.md + 04-设计/命名哲学-判据.md
> **状态**：✅ 完成（2026-08-27）

## 一、目标

将 commit 6d34959 落地的 `moxing_fu::从环境变量构造() -> Option<LLM池>` 真正接入到调用方 `mingling_caozuo_fu::跑流水线_mock_llm`：

- 命令操作-府 新增「真实-后端-选择-殿」承载后端选择策略（≥2 阁：环境-变量-后端-阁 + 端点-配置-阁）
- 「跑流水线_mock_llm」改造为：根据 `LLM_BACKEND` 环境变量在 mock / 真实 之间切换
- 优雅降级：`从环境变量构造()` 返回 `None` → 自动 fallback 到 `MockLLM连接`，不破坏现有 12 项门禁 e2e
- 不需要修改 `moxing_fu` 内部逻辑（环境配置-殿 已具备从 env 构造的能力）
- 命名遵守祖孙不同名 + 跨府唯一（参 04-设计/命名哲学-判据.md）

## 二、falsifiable

- [x] 命令操作-府新增「真实-后端-选择-殿」含 2 阁：环境-变量-后端-阁 + 端点-配置-阁
- [x] 「跑流水线_mock_llm」保留现有 mock 行为（默认）；`LLM_BACKEND=real` 走真实 HTTP 连接
- [x] `LLM_BACKEND=real` + 无 `LLM_API_KEY` → 自动 fallback 到 mock（不报错）
- [x] `LLM_BACKEND=mock` / 空 / 未设置 → 走 mock（与现状完全一致，不破坏 e2e）
- [x] 端点-配置-阁独立读 `LLM_BASE_URL`、`LLM_TIMEOUT_MS`，供真实模式使用
- [x] 单测覆盖：mock 模式（默认）、真实模式（有 key）、真实模式（无 key 降级）、端点自定义
- [x] Cargo.toml 已含 moxing_fu 依赖（确认无需新增）
- [x] 命名通过 12 项门禁（祖孙不同名 + 同层唯一 + 目录名无英文）
- [x] cargo test --workspace 全过 + cargo clippy --workspace --all-targets 零警告 + 一键全验 12 项全绿

## 三、命名

| 层 | 命名 | 判据 |
|:--|:--|:--|
| 殿 | 真实-后端-选择-殿 | 祖孙不同名：殿名「真实-后端-选择」与阁「环境-变量-后端」「端点-配置」无重叠 ✓ |
| 阁 | 环境-变量-后端-阁 | 「环境-变量-后端」vs 园「环境-变量-后端-解析」无祖孙包含 ✓ |
| 阁 | 端点-配置-阁 | 「端点-配置」vs 园「端点-配置-解析」无祖孙包含 ✓ |
| 园 | 环境-变量-后端-解析-园 | 实现 |
| 园 | 端点-配置-解析-园 | 实现 |

跨府唯一性：
- 「环境-变量-后端-阁」 vs moxing_fu「环境-变量-大模型-阁」：去后缀「环境-变量-后端」 vs 「环境-变量-大模型」字符串不等 ✓
- 「端点-配置-阁」 全项目无重复 ✓

## 四、行为契约

### 4.1 后端选择策略

```
读 LLM_BACKEND 环境变量:
  "real" → 真实模式（若从环境变量构造() 返回 None 则降级 mock）
  "mock" → Mock 模式
  ""/未设置 → Mock 模式（默认，保证向后兼容）
```

### 4.2 优雅降级

```
真实模式 + LLM_API_KEY 缺失 → 记录「[降级] 真实模式无可用 API key」 → 走 MockLLM连接
```

不抛错、不修改退出码（仍 0），仅日志中标注降级原因。

### 4.3 端点配置

```
LLM_BASE_URL 默认 https://api.openai.com/v1/chat/completions
LLM_TIMEOUT_MS 默认 30000
LLM_MODEL 默认 gpt-3.5-turbo
```

由 moxing_fu::从环境变量构造() 已处理，本殿只读不写。

## 五、风险

- **风险 1**：CI 环境无 `LLM_API_KEY` → 默认走 mock，12 项门禁 e2e 不变 ✓
- **风险 2**：`LLM_BACKEND=real` 拼写错误（如 `Real`）→ 与「未设置」等价走 mock（`match` 模式）
- **风险 3**：HTTP 调用超时 → 当前实现保留 `LLM调用器 + HTTP连接` 链路，超时由 moxing_fu 内部 ureq 配置处理

## 六、状态机（logicprobe 验证）

```text
选择 ──读取环境(backend=mock/空)──→ Mock模式 ──Mock就绪──→ Mock执行 [终态]
   │
   └──读取环境(backend=real)──→ 真实模式 ──有API_KEY──→ 真实执行 [终态]
                                │
                                └──无API_KEY──→ 降级 ──降级完成──→ Mock执行 [终态]
```

logicprobe 14 检查结果：
- S2 无死锁 ✓ / S4 确定性 ✓ / S6 guard 完备 ✓ / S7 invariant 终态必达 ✓ / A7 最短反例 ✓
- S1 部分状态不可达（默认 backend=2 → mock 路径；real/降级需 env 显式设置）— 设计意图

## 七、回归保证

- `mingling_caozuo_fu::跑流水线_mock_llm` 函数签名不变（仍为 `pub fn 跑流水线_mock_llm(任务标识: &str) -> 命令结果`）
- 现有 e2e 测试 `e2e_mock_llm_4分类调用` / `e2e_mock_llm_分发命令` / `e2e_mock_llm_任务标识传递` 全部继续通过
- 入口 `pub use 跑流水线_mock_llm` 不变
- 新增「跑流水线_真实_llm」与「跑流水线_后端选择」测试入口

---

*22-moxing_fu调用方集成 · 2026-08-27 · 入稿 · decided_by: 界主*
*implements: 应（Round 9）*
*falsifiable: 真实-后端-选择-殿 2 阁 + 后端选择函数 + 单测覆盖 4 场景 + 12/12 门禁全绿*