# 05-Cargo-workspace

> 底层视图——13 个 lib crate 的依赖方向。

## 一、13 个 lib crate 布局

```mermaid
flowchart TB
    subgraph 鸿蒙["鸿蒙（核心地基）"]
        plugin[插件上下文<br/>Service Definition + 注册表]:::core
        events[跨维事件总线<br/>3 类事件 + 2 模式]:::core
        memory[记忆承载<br/>36 格位 + 3 档 + 3 源]:::core
        state[状态共享<br/>进程级状态读写]
        observe[观测探针<br/>24 项事件通道埋点]
        logging[日志记录<br/>tracing 定制封装]
        pipeline[流水线驱动<br/>4 阶段状态机]:::core
        exec[任务执行<br/>4 分类角色卡 + 工具循环]:::core
        llm[模型连接<br/>ureq POST]
    end

    subgraph 配置["世界配置"]
        config[配置管理<br/>.env 密钥注入]
    end

    subgraph 乾坤["乾坤（空间）"]
        cli[命令操作<br/>CLI 入口]
    end

    subgraph 证道["证道（测试）"]
        test[单元测试<br/>镜像被测]
    end

    subgraph 顶层["顶层"]
        world[世界<br/>13 crate 统一入口（占位）]
    end

    world --> pipeline
    world --> exec
    world --> memory
    world --> events

    cli --> pipeline
    cli --> exec

    pipeline --> memory
    pipeline --> events
    pipeline --> plugin
    pipeline --> exec
    pipeline --> config

    exec --> memory
    exec --> llm
    exec --> events
    exec --> plugin
    exec --> config

    memory --> events
    memory --> plugin

    llm --> config
    events --> plugin
    events --> state
    events --> observe

    test --> memory
    test --> pipeline
    test --> exec
    test --> llm
    test --> config
    test --> logging

    classDef core fill:#e1f5e1,stroke:#0a0,stroke-width:2px
```

## 二、依赖关系（按方向）

```mermaid
flowchart LR
    subgraph 上层["上层（被依赖最多）"]
        plugin[插件上下文]
    end

    subgraph 中层["中层（业务核心）"]
        pipeline[流水线驱动]
        exec[任务执行]
        memory[记忆承载]
    end

    subgraph 下层["下层（基础设施）"]
        events[事件总线]
        llm[模型连接]
        config[配置管理]
        logging[日志记录]
        state[状态共享]
        observe[观测探针]
    end

    pipeline --> plugin
    exec --> plugin
    memory --> plugin
    pipeline --> events
    memory --> events
    exec --> llm
    pipeline --> config
    exec --> config
    llm --> config
    events --> state
    events --> observe
```

## 三、错误的依赖方向（禁止）

- ❌ 下层依赖上层（如 plugin 不应依赖 pipeline）
- ❌ 跨层直调（如 cli 不应直接调用 memory，应经 pipeline）
- ❌ 循环依赖（任何两个 crate 不能互引）

## 四、维度归属

| crate | 维度 | 域 | 备注 |
|:--|:--|:--|:--|
| plugin | 鸿蒙 | 基础设施 | 核心 |
| events | 鸿蒙 | 基础设施 | 核心 |
| memory | 鸿蒙 | 基础设施 | 核心 |
| state | 鸿蒙 | 基础设施 | |
| observe | 鸿蒙 | 基础设施 | |
| logging | 鸿蒙 | 基础设施 | |
| pipeline | 鸿蒙 | 基础设施 | 核心 |
| exec | 鸿蒙 | 基础设施 | 核心 |
| llm | 鸿蒙 | 基础设施 | |
| config | 鸿蒙 | 世界配置 | |
| cli | 乾坤 | 呈现 | |
| test | 证道 | 鸿蒙 | |
| world | 顶层 | — | 统一入口 |

## 五、falsifiable

- 上线 1 个月：cargo metadata 验证依赖方向 100% 合规
- 上线 3 个月：移除任何下层 crate，上层 crate 编译失败

---

*传承殿 · 2026-08-26 · decided_by: 界主*
*implements: 术·Cargo workspace 依赖图*
