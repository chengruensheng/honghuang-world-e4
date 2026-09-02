//! 注册表实现 - RULE_REGISTRY 单一真相源
//!
//! 14 条规则 = 13 接单门 + 1 V-COUNT 派生。
//!
//! 决策锚：260826-2220 治理-司衡 + 260826-2240 传承殿启动
//! 关联文档：02-概念/规则注册表/06-规则注册表.md + 00-宪法/DECISION-CONTRACT.md

use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 严格度 {
    Fatal,
    Warning,
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 治理域 {
    前置,
    结构,
    引用,
    治理,
    派发,
}

#[derive(Clone, Debug)]
pub struct 可证伪条件 {
    pub 命题: String,
    pub 证伪方法: String,
    pub 时间窗口: String,
}

#[derive(Clone, Debug)]
pub struct 规则条目 {
    pub 规则ID: String,
    pub 严格度: 严格度,
    pub 治理域: 治理域,
    pub 描述: String,
    pub decided_by: String,
    pub falsifiable: Vec<可证伪条件>,
    pub implements: String,
}

#[derive(Clone, Debug, Default)]
pub struct 接单候选 {
    pub 候选ID: String,
    pub 内容: String,
    pub decided_by: String,
    pub falsifiable: Vec<String>,
    pub upstream: String,
    pub implements: String,
    pub 阶段: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 接单决策 {
    接受,
    拒绝 { 规则ID: String, 原因: String },
}

pub const RULE_COUNT: usize = 14;

static RULE_REGISTRY: OnceLock<Vec<规则条目>> = OnceLock::new();

pub fn 获取注册表() -> &'static [规则条目] {
    RULE_REGISTRY.get_or_init(构造注册表)
}

/// 规则数据行：规则ID / 严格度 / 治理域 / 描述 / 可证伪命题 / 证伪方法 / 时间窗口 / 哲学锚
type 规则行 = (
    &'static str,
    严格度,
    治理域,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
);

fn 构造注册表() -> Vec<规则条目> {
    // 14 条规则 = 13 接单门 + 1 V-COUNT 派生；decided_by 均为「界主」（单一真相源）
    const 表: [规则行; 14] = [
        ("BASE-001-程序定", 严格度::Fatal, 治理域::治理, "确定程序是治理操作唯一执行者（LLM 只生成符号材料）", "LLM 直写知识包违规次数 = 0", "写入日志审查", "持续", "法·司衡基线（基线 1）"),
        ("BASE-002-异常门", 严格度::Warning, 治理域::派发, "异常门自动顶异常（人类只看异常）", "人类注意力介入次数 < 5 次/天", "异常门调用统计", "上线后 1 个月", "法·司衡基线（基线 3）"),
        ("BASE-003-可验证", 严格度::Fatal, 治理域::结构, "可验证性约束（frozen outcome + hash 链 + falsifiable）", "事件流 hash 校验 100% 通过", "机械判定", "上线后 1 个月", "法·司衡基线（基线 4）"),
        ("BASE-004-减LLM", 严格度::Info, 治理域::派发, "治理延伸是减少 LLM 参与（关键决策确定化）", "LLM 调用占比 < 30%（关键决策位）", "token 消耗统计", "上线后 3 个月", "法·司衡基线（基线 5）"),
        ("BASE-005-异常优先", 严格度::Warning, 治理域::派发, "信息洪流是旧仓失败根因（异常优先）", "异常响应时间 P0 < 5 分钟", "告警日志", "持续", "法·司衡基线（基线 2）"),
        ("BAN-001-LLM直改", 严格度::Fatal, 治理域::治理, "LLM 不可直改知识包（所有写入需经 decided_by 字段校验）", "decided_by 缺失写入失败率 100%", "机械判定", "持续", "法·司衡禁止（禁止 1）"),
        ("BAN-002-不可复现", 严格度::Fatal, 治理域::引用, "不可复现多 Agent 交互不可作治理决策依据", "关键决策均含可复现证据", "决策文档审查", "持续", "法·司衡禁止（禁止 2）"),
        ("BAN-003-入事件流", 严格度::Fatal, 治理域::结构, "治理操作必入事件流，留痕不可篡改", "治理动作可追溯率 100%", "事件流日志审查", "持续", "法·司衡禁止（禁止 3）"),
        ("BAN-004-视图介入", 严格度::Warning, 治理域::派发, "人类只通过视图介入（不直接看原始对话）", "原始对话查看次数 = 0（仅通过摘要+异常门介入）", "审计日志", "持续", "法·司衡禁止（禁止 4）"),
        ("CONTRACT-001-decidedBy", 严格度::Fatal, 治理域::前置, "每决策带 decided_by（缺失则事件拒收）", "decided_by 缺失写入失败率 100%", "机械判定", "持续", "法·契约（决策契约 1）"),
        ("CONTRACT-002-falsifiable", 严格度::Fatal, 治理域::前置, "每决策带 falsifiable（可证伪命题 + 时间窗口）", "决策可证伪率 > 80%", "决策文档统计", "上线后 6 个月", "法·契约（决策契约 2）"),
        ("CONTRACT-003-implements", 严格度::Warning, 治理域::引用, "每决策可追溯到哲学锚（implements 字段引用 道/法/术/鉴/应/元 或五法）", "决策哲学锚引用覆盖率 100%", "决策文档审查", "上线后 3 个月", "法·契约（决策契约 3）"),
        ("CONTRACT-004-入稿落码", 严格度::Info, 治理域::前置, "先入稿再落码（AGENTS § 8：每个设计决策先入设计稿章节，再实现）", "阶段 4+ 所有变更先有 10-地基/ 实施方案", "commit diff 审查", "持续", "法·契约（决策契约 4）"),
        ("RULE-V-COUNT", 严格度::Warning, 治理域::结构, "RULE_COUNT 必须从 RULE_REGISTRY.len() 派生（不硬编码）。当前值 = 14 = 13 接单门 + 1 V-COUNT 自身", "RULE_COUNT == RULE_REGISTRY.len()（编译期保证 + 单元测试）", "编译期 + 单元测试", "持续", "法·可演化（单一真相源 + 派生值）"),
    ];
    表.into_iter()
        .map(|(id, 严, 域, 描, 命, 法, 窗, 锚)| 规则(id, 严, 域, 描, 命, 法, 窗, 锚))
        .collect()
}

/// 组装单条规则（falsifiable 固定为单条可证伪条件；decided_by 恒为界主）
fn 规则(
    id: &str,
    严: 严格度,
    域: 治理域,
    描: &str,
    命: &str,
    法: &str,
    窗: &str,
    锚: &str,
) -> 规则条目 {
    规则条目 {
        规则ID: id.into(),
        严格度: 严,
        治理域: 域,
        描述: 描.into(),
        decided_by: "界主".into(),
        falsifiable: vec![可证伪条件 {
            命题: 命.into(),
            证伪方法: 法.into(),
            时间窗口: 窗.into(),
        }],
        implements: 锚.into(),
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn rule_count等于14() {
        // 编译期提示常量 RULE_COUNT 必须等于 RULE_REGISTRY.len()
        assert_eq!(RULE_COUNT, 获取注册表().len());
        assert_eq!(RULE_COUNT, 14);
    }

    #[test]
    fn 规则id唯一() {
        let mut seen = std::collections::HashSet::new();
        for r in 获取注册表() {
            assert!(seen.insert(r.规则ID.clone()), "重复 ID：{}", r.规则ID);
        }
    }

    #[test]
    fn 所有规则都有decided_by() {
        for r in 获取注册表() {
            assert!(!r.decided_by.is_empty(), "规则 {} 缺 decided_by", r.规则ID);
        }
    }

    #[test]
    fn 包含所有14规则() {
        let ids: std::collections::HashSet<&str> =
            获取注册表().iter().map(|r| r.规则ID.as_str()).collect();
        let 期望: std::collections::HashSet<&str> = [
            "BASE-001-程序定",
            "BASE-002-异常门",
            "BASE-003-可验证",
            "BASE-004-减LLM",
            "BASE-005-异常优先",
            "BAN-001-LLM直改",
            "BAN-002-不可复现",
            "BAN-003-入事件流",
            "BAN-004-视图介入",
            "CONTRACT-001-decidedBy",
            "CONTRACT-002-falsifiable",
            "CONTRACT-003-implements",
            "CONTRACT-004-入稿落码",
            "RULE-V-COUNT",
        ]
        .into_iter()
        .collect();
        assert_eq!(ids, 期望, "RULE_REGISTRY 包含的规则必须 = 期望集");
    }

    #[test]
    fn fatal_警告_info分级() {
        let mut fatal_count = 0;
        let mut warning_count = 0;
        let mut info_count = 0;
        for r in 获取注册表() {
            match r.严格度 {
                严格度::Fatal => fatal_count += 1,
                严格度::Warning => warning_count += 1,
                严格度::Info => info_count += 1,
            }
        }
        assert_eq!(fatal_count, 7); // BASE-001/003 + BAN-001/002/003 + CONTRACT-001/002
        assert_eq!(warning_count, 5); // BASE-002/005 + BAN-004 + CONTRACT-003 + RULE-V-COUNT
        assert_eq!(info_count, 2); // BASE-004 + CONTRACT-004
    }
}
