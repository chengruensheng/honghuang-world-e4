# 02-DSH-架构.md

> **参考资料**——DeepSeek Harness 架构的关键内容摘要。

## 一、仓库地址

- GitHub：https://github.com/deepseek-ai/deepseek-harness
- 首发：2026-08-13（developer preview）
- 当前 Star：18 万+

## 二、核心设计哲学

> "**没有特权的核心可供修改——你只需把插件挂到其他插件旁边来扩展**"

Agent Loop 本身都可被替换——这是 DSH 与其他 Agent 框架的根本区别。

## 三、6 子系统（packages/core/）

| 子系统 | 职责 |
|:--|:--|
| `session` | append-only 会话日志 |
| `system-prompt` | 提示词组装 |
| `tools` | 工具注册表 |
| `agent` | Agent 接口与注册表 |
| `agent-loop` | 默认 driver |
| `scope` | 作用域原语 |

## 四、3 类事件

### Session 事件（持久化）

- append 到日志
- broadcast 通过 session/event
- 适用："事实必须穿越 reload"

### Agent 事件（agent/*）

- 携带 live Agent 对象
- 适用：观察或拦截 in-flight 工作

### Capability 事件（fs/* / tools/* / telemetry/*）

- attach policy 和 adapter 到接缝
- 适用：fs/tools/telemetry 的接缝拦截

## 五、2 模式

### Waterfall 事件

- 必须 `next()` 把控制权传给下一个 listener
- 典型：`agent/pre-step`、`tools/pre-execute`

### Serial 事件

- 独立拦截点，不调 `next()`
- 典型：`agent/turn-stopping`

## 六、Code Mode (PTC) —— DSH 最大创新

### 机制

模型写一段 TypeScript 程序调用多个工具：
```typescript
await tools.read_file({ path: "foo.rs" });
await tools.edit_file({ path: "foo.rs", old: "x", new: "y" });
return { success: true };  // 只送回 return 内容
```

### 价值

- 5 次 function calling 往返合并为 1 次代码块执行
- 工具调度与上下文占用解耦
- 模型生成的代码跑在**独立 Node worker thread**
- 环境空、heap cap、wall clock cap、硬终止
- 官方定位："containment, not a security boundary"（权限同 bash）

## 七、Step / Turn 概念

- **Step**：一次模型请求 + 它触发的工具调用
- **Turn**：零或多 step

### Turn 生命周期

```
turn/start → claim 输入 → 组装 prompt + tool schemas → agent/pre-step waterfall
   → 消息 append → agent/request waterfall → llm/stream waterfall
   → assistant/message → tool/call → tools waterfall → tool/result
   → step/end → agent/turn-stopping → turn/end
```

## 八、Profile × Bundle × Patch

三层正交组合：
- **Profile**：用户/项目/场景的元数据
- **Bundle**：工具/插件包
- **Patch**：运行时补丁

## 九、同时为 Claude Code 与其他 Agent 服务

DSH 仓库同时包含 `.agents/` 和 `.claude/`，根目录 `AGENTS.md` 和 `CLAUDE.md` 并存——不是兼容性宣称，是根目录摆两份入口。

## 十、传承殿对其的承接

- ✅ 万物皆插件（已承接，见 `02-概念/可插拔/01-可插拔.md`）
- ✅ 6 子系统架构骨架（已承接，见 `04-设计/01-架构总览.md § 五`）
- ✅ 3 类事件 + 2 模式（已承接，见 `04-设计/数据模型/02-事件流.md`）
- ✅ frozen outcome（已承接，见 `02-概念/不可逆结果/07-不可逆结果.md`）
- ✅ 一键全验 10 项（已承接，见 `05-质量/02-一键全验.md`）
- ⏳ Code Mode (PTC)（待 Phase 4 评估）

## 十一、引用

- 哲学文档：`01-哲学/03-工程哲学.md`
- 决策文档：`03-决策/已定/260826-2230-工程-DSH.md`
- 数据模型：`04-设计/数据模型/`

---

*传承殿 · 2026-08-26 · decided_by: 界主*
*implements: 参考·DSH 工程架构*