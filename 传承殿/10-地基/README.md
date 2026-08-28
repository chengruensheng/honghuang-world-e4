# 10-地基 · 阶段 1 实施

> **已完成** · 阶段 1（地基）的实施目录。
>
> **更新（2026-08-27）**：当前 workspace 共 21 个 member，205 项单元测试，一键全验 10/10 全绿；后续重构入稿见 12-殿阁园重构-实施方案.md（拆分）与 13-殿阁园唯一命名-实施方案.md（命名）。
>
> **更正**：原 README 标记 Day 1-4 为"✓ 完成"但实际为空。已于 2026-08-26 由 MiniMax-M3 全量重建。

## 实施状态

| Day | 任务 | 状态 | 实际产出 |
|:--|:--|:--|:--|
| 1 | workspace + 工具链（Cargo.toml + rustfmt + clippy + deny + .gitignore + .gitattributes + lefthook） | ✅ 完成 | 8 个根文件 |
| 2 | 12 个 lib crate 空骨架 | ✅ 完成 | 鸿蒙 9 + 乾坤 1 + 证道 1 + 世界 1（lib+bin）|
| 3 | 10 个验证脚本 + 一键全验.sh | ✅ 完成 | 道果树/质量门禁 - 域/门禁 - 府/ |
| 4 | CI 配置（.github/workflows/ci.yml） | ✅ 完成 | Ubuntu + Rust 1.75 |
| 5 | 端到端验证 + 首次 commit | ✅ 完成 | 10/10 全绿 |

## 12 个 lib crate（lib 名 = pinyin，目录 = 中文）

```
鸿蒙/
├── 基础设施 - 域/
│   ├── 插件上下文 - 府/          (chajian_xiawenxian_fu)
│   ├── 跨维事件总线 - 府/        (kuawai_shijian_zongxian_fu)
│   ├── 记忆承载 - 府/            (jiyi_chengzai_fu)
│   ├── 状态共享 - 府/            (zhuangtai_gongxiang_fu)
│   ├── 观测探针 - 府/            (guance_tanzhen_fu)
│   ├── 日志记录 - 府/            (rizhi_jilu_fu)
│   ├── 流水线驱动 - 府/          (liushuixian_qudong_fu)
│   └── 任务执行 - 府/            (renwu_zhixing_fu)
└── 世界配置 - 域/
    └── 配置管理 - 府/            (peizhi_guanli_fu)
乾坤/
└── 呈现 - 域/
    └── 命令操作 - 府/            (mingling_caozuo_fu)
证道/
└── 鸿蒙 - 域/
    └── 单元测试 - 府/            (danyuan_ceshi_fu)
世界/                            (shijie：lib + bin)
```

> 注：原 README 称"13 lib crate"，实际严格按目录树为 12（lib 11 + bin 1 = 12 工作空间成员）。
> 工作空间声明 12 项（11 lib + 1 lib+bin = 12 个 package）——此为阶段 1 骨架；截至 2026-08-27 已扩展为 21 个 member（含 道韵/规则、道果树/质量门禁-监控·评估·校验、道果树/运营-实时·调遣·升级、鸿蒙/模型连接·追问引擎 等）。

## 关键约定（v3 严格）

- **零 src/ 平铺**：每个 crate 用 `入口.rs` 作为库入口，Cargo.toml `[lib]` path 指向 `入口.rs`
- **零英文目录命名**：目录与文件全中文（除 cargo-required 文件名）
- **二进制入口分离**：`世界` 同时拥有 lib 与 bin，bin 文件为 `二进制入口.rs`，lib 文件为 `入口.rs`
- **构建产物归位**：`target-dir = "道果树/构建物-域"`（`.cargo/config.toml`）
- **bash 兼容**：`.sh` 脚本中变量名用 ASCII，避免 Git Bash 中文标识符解析问题

## 质量门禁

道果树/质量门禁 - 域/门禁 - 府/ 下 10 个 PowerShell 脚本。

一键全验：
- `bash 一键全验.sh`（Linux/Mac/Git Bash）
- `pwsh 一键全验.ps1`（Windows PowerShell）

## 验证结果（2026-08-26 重建后）

```
╔══════════════════════════════════════════════╗
║  洪荒 · 世界 v3 · 阶段 1 · 一键全验 10 项      ║
╚══════════════════════════════════════════════╝
─── [1] 格式 ───          [PASS]
─── [2] 静态分析 ───      [PASS]
─── [3] 单元测试 ───      [PASS]   21 项全过
─── [4] 编译 ───          [PASS]
─── [5] 文档 ───          [PASS]
─── [6] 安全审计 ───      [PASS]
─── [7] 依赖审查 ───      [PASS]
─── [8] 无 src/ 平铺 ─── [PASS]
─── [9] 无空目录 ───      [PASS]
─── [10] 无临时目录残留 ── [PASS]
║  汇总：通过 10 / 失败 0 / 跳过 0                ║
```

## 入稿点

- 决策锚：00-宪法/开发顺序.md § 阶段 1
- 命名映射：00-宪法/ARCHITECTURE.md § 五
- 风格契约：00-宪法/AGENTS-风格契约.md
- 注释规范：00-宪法/代码注释规范.md
- 决策契约：00-宪法/DECISION-CONTRACT.md

## falsifiable（全部达成）

- ✅ Day 5：`cargo metadata` 验证 12 个 crate 全部存在
- ✅ Day 5：`cargo build` 成功（空 crate 编过）
- ✅ Day 5：`bash 一键全验.sh` 跑通 10 项脚本

---

*10-地基 · 2026-08-26 重建 · decided_by: 界主 · implemented_by: MiniMax-M3*
*implements: 道（cargo workspace + 12 crate 空骨架 + 一键全验 10 项）*
