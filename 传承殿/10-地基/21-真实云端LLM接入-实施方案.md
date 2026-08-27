# 21-真实云端 LLM 接入（Round 8 新功能）

> **决策锚**：v4 路线阶段 17 扩展 + 260827-统一风格
> **关联文档**：10-v4-阶段17.md（mock HTTP）+ 11-v4-阶段18.md（多 agent）+ 02-概念/可插拔/01-可插拔.md + 14-命名唯一性门禁-实施方案.md
> **状态**：✅ 实施完成（2026-08-27 · commit 待提交）

## 一、目标

将 moxing_fu 从「mock HTTP server 替代」升级为「真实云端 LLM 接入」：
- 支持 OpenAI 兼容 API（DeepSeek/OpenAI/Azure OpenAI 等）
- Bearer Token 鉴权（环境变量注入）
- 模型名/base URL 可配置
- 无 API key 时优雅降级到 mock（避免破坏现有 e2e）
- 命令操作 CLI 增加真实模式（环境变量切换）

## 二、falsifiable

- [x] moxing_fu 加 环境配置-殿/环境-变量-大模型-阁/环境-变量-大模型-配置-园（含 Bearer+base URL+模型名+4 分类+优雅降级）
- [x] 复用配置管理-府已有 环境变量配置源；moxing_fu 新增 env var 读取 fn 从环境变量构造()
- [x] 优雅降级：moxing_fu::从环境变量构造() 返回 Option<LLM池>，无 API_KEY → None（调用方走 mock，已破坏 e2e 不变）
- [x] 从环境变量构造() 无 API_KEY 字符串 → 返回 None
- [x] 6 个新单元测试（env_lock 串行）：无 env→None/空字符串→None/有效→Some/4 分类覆盖/超时默认/超时自定义
- [x] cargo check + clippy 零警告 + 一键全验 13 项全绿 + 223 项测试全过

## 三、命名

- moxing_fu 新增：连接-真实-云端-阁（连接构造 + Bearer 头 + HTTP POST）
- 配置-管理府复用现有 环境变量配置源（API key 注入）
- 命令-操作-府 新增 真实-后端-选择-阁（环境变量读取 + mock/real 分支）

## 四、风险

- 风险 1：无 API key 时编译失败 → 全部通过环境变量 + Default 兜底
- 风险 2：真实 LLM 调用超时/失败 → 返回 Result<…>，由调用方决定重试
- 风险 3：CI 环境无 API key → 默认 mock 通过，real 模式跳过测试
