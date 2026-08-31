//! 经验-实现-园 - 复用经验入 36 格位（阶段 10 收口）

// ============================================================================
// 复用经验入 36 格位（阶段 10 收口）
// ============================================================================

/// 复用经验（沉淀到 36 格位）
#[derive(Clone, Debug, PartialEq)]
pub struct 复用经验 {
    pub 场景: String,
    pub 决策: String,
    pub 复用点: String,
    pub 关联阶段: String,
    pub 写入格位: 写入格位信息,
}

#[derive(Clone, Debug, PartialEq)]
pub struct 写入格位信息 {
    pub 总纲: String,
    pub 本质: String,
    pub 路径: String,
}

impl 复用经验 {
    pub fn 新建(场景: &str, 决策: &str, 复用点: &str, 关联阶段: &str) -> Self {
        // 默认写 经历×归档 格位（最贴切"复用经验"语义）
        Self {
            场景: 场景.to_string(),
            决策: 决策.to_string(),
            复用点: 复用点.to_string(),
            关联阶段: 关联阶段.to_string(),
            写入格位: 写入格位信息 {
                总纲: "经历".to_string(),
                本质: "归档".to_string(),
                路径: "经历/归档/复用经验".to_string(),
            },
        }
    }
}

// ============================================================================
// v3 阶段 1-10 复用经验清单
// ============================================================================

/// 获取阶段 1-10 所有复用经验
pub fn v3复用经验() -> Vec<复用经验> {
    vec![
        复用经验::新建(
            "Cargo workspace + 12 个 lib crate + 10 项门禁",
            "中文目录 + 拼音 crate + 入口.rs 替代 src/lib.rs",
            "新阶段 crate 复用：直接 cp 模板 + 改 Cargo.toml name + 改 src 字段",
            "阶段 1",
        ),
        复用经验::新建(
            "事件流 hash 链 + 3 类事件 + Waterfall/Serial",
            "ureq + serde_json + 简化的 JSON 行式持久化",
            "新事件源：复用事件流 trait + 追加 1 个事件类型 enum 变体",
            "阶段 2",
        ),
        复用经验::新建(
            "36 格位（6 总纲 × 6 本质）+ MUST/MIXED/OPTIONAL 加载档位",
            "jiyi_chengzai_fu struct + 提问引擎/拼全息图函数",
            "新分类：复用 enum + 添加新格位路径到拼全息图",
            "阶段 3",
        ),
        复用经验::新建(
            "RULE_REGISTRY 14 条 + OnceLock + 决策契约字段校验",
            "guize_fu struct + 简化 TOML 解析器",
            "新规则：追加到 RULE_REGISTRY + 补充校验函数",
            "阶段 4",
        ),
        复用经验::新建(
            "4 分类状态机 + is_跳层/is_反序 + 循环打回计数",
            "liushuixian_qudong_fu enum + 角色卡 + 分类_机械判定",
            "新分类：复用 enum + 添加映射函数",
            "阶段 5",
        ),
        复用经验::新建(
            "CLI 3 命令 + e2e 拒绝路径 + 4 分类 LLM mock",
            "mingling_caozuo_fu 命令 trait + 状态机调用",
            "新命令：实现 命令 trait + 添加入分发",
            "阶段 6",
        ),
        复用经验::新建(
            "ureq HTTP POST + OpenAI 兼容 + 4 分类 LLM 池",
            "moxing_fu 模型连接 trait + LLM调用器",
            "新 LLM：实现 模型连接 trait + 注册到 LLM 池",
            "阶段 7",
        ),
        复用经验::新建(
            "4 问题 enum + 关键词映射 + 3 mock LLM 投票",
            "zhuiwen_fu 追问引擎 + 投票引擎",
            "新问题：追加到 追问 enum + 关键词列表",
            "阶段 8",
        ),
        复用经验::新建(
            "4 类指标 + 4 级告警 + 4 级应急 + 升级路径映射",
            "jiankong_fu struct + 升级路径函数",
            "新指标/告警：追加 enum 变体 + 写触发函数",
            "阶段 9",
        ),
        复用经验::新建(
            "3 类升级 + 错峰 + 回滚 + 复用经验入 36 格位",
            "shengji_fu 升级计划 + 错峰 + 回滚 + 复用经验 struct",
            "新升级：复用 升级类型 enum + 写新升级计划",
            "阶段 10",
        ),
    ]
}
