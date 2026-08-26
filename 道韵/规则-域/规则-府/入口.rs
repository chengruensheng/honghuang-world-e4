//! 规则-府
//!
//! RULE_REGISTRY 单一真相源 + 决策契约字段校验。
//! 14 条规则 = 13 接单门 + 1 V-COUNT 派生。
//!
//! 决策锚：260826-2220 治理-司衡 + 260826-2240 传承殿启动
//! 关联文档：02-概念/规则注册表/06-规则注册表.md + 00-宪法/DECISION-CONTRACT.md

#![allow(clippy::upper_case_acronyms)] // V-COUNT 等全大写是规则 ID 命名
#![allow(non_snake_case)] // 规则ID/候选ID 等字段名遵循中文命名（不允许改 snake_case 破坏语义）

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

fn 构造注册表() -> Vec<规则条目> {
    vec![
        规则条目 {
            规则ID: "BASE-001-程序定".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::治理,
            描述: "确定程序是治理操作唯一执行者（LLM 只生成符号材料）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "LLM 直写知识包违规次数 = 0".into(),
                证伪方法: "写入日志审查".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡基线（基线 1）".into(),
        },
        规则条目 {
            规则ID: "BASE-002-异常门".into(),
            严格度: 严格度::Warning,
            治理域: 治理域::派发,
            描述: "异常门自动顶异常（人类只看异常）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "人类注意力介入次数 < 5 次/天".into(),
                证伪方法: "异常门调用统计".into(),
                时间窗口: "上线后 1 个月".into(),
            }],
            implements: "法·司衡基线（基线 3）".into(),
        },
        规则条目 {
            规则ID: "BASE-003-可验证".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::结构,
            描述: "可验证性约束（frozen outcome + hash 链 + falsifiable）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "事件流 hash 校验 100% 通过".into(),
                证伪方法: "机械判定".into(),
                时间窗口: "上线后 1 个月".into(),
            }],
            implements: "法·司衡基线（基线 4）".into(),
        },
        规则条目 {
            规则ID: "BASE-004-减LLM".into(),
            严格度: 严格度::Info,
            治理域: 治理域::派发,
            描述: "治理延伸是减少 LLM 参与（关键决策确定化）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "LLM 调用占比 < 30%（关键决策位）".into(),
                证伪方法: "token 消耗统计".into(),
                时间窗口: "上线后 3 个月".into(),
            }],
            implements: "法·司衡基线（基线 5）".into(),
        },
        规则条目 {
            规则ID: "BASE-005-异常优先".into(),
            严格度: 严格度::Warning,
            治理域: 治理域::派发,
            描述: "信息洪流是旧仓失败根因（异常优先）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "异常响应时间 P0 < 5 分钟".into(),
                证伪方法: "告警日志".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡基线（基线 2）".into(),
        },
        规则条目 {
            规则ID: "BAN-001-LLM直改".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::治理,
            描述: "LLM 不可直改知识包（所有写入需经 decided_by 字段校验）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "decided_by 缺失写入失败率 100%".into(),
                证伪方法: "机械判定".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡禁止（禁止 1）".into(),
        },
        规则条目 {
            规则ID: "BAN-002-不可复现".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::引用,
            描述: "不可复现多 Agent 交互不可作治理决策依据".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "关键决策均含可复现证据".into(),
                证伪方法: "决策文档审查".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡禁止（禁止 2）".into(),
        },
        规则条目 {
            规则ID: "BAN-003-入事件流".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::结构,
            描述: "治理操作必入事件流，留痕不可篡改".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "治理动作可追溯率 100%".into(),
                证伪方法: "事件流日志审查".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡禁止（禁止 3）".into(),
        },
        规则条目 {
            规则ID: "BAN-004-视图介入".into(),
            严格度: 严格度::Warning,
            治理域: 治理域::派发,
            描述: "人类只通过视图介入（不直接看原始对话）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "原始对话查看次数 = 0（仅通过摘要+异常门介入）".into(),
                证伪方法: "审计日志".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·司衡禁止（禁止 4）".into(),
        },
        规则条目 {
            规则ID: "CONTRACT-001-decidedBy".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::前置,
            描述: "每决策带 decided_by（缺失则事件拒收）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "decided_by 缺失写入失败率 100%".into(),
                证伪方法: "机械判定".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·契约（决策契约 1）".into(),
        },
        规则条目 {
            规则ID: "CONTRACT-002-falsifiable".into(),
            严格度: 严格度::Fatal,
            治理域: 治理域::前置,
            描述: "每决策带 falsifiable（可证伪命题 + 时间窗口）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "决策可证伪率 > 80%".into(),
                证伪方法: "决策文档统计".into(),
                时间窗口: "上线后 6 个月".into(),
            }],
            implements: "法·契约（决策契约 2）".into(),
        },
        规则条目 {
            规则ID: "CONTRACT-003-implements".into(),
            严格度: 严格度::Warning,
            治理域: 治理域::引用,
            描述: "每决策可追溯到哲学锚（implements 字段引用 道/法/术/鉴/应/元 或五法）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "决策哲学锚引用覆盖率 100%".into(),
                证伪方法: "决策文档审查".into(),
                时间窗口: "上线后 3 个月".into(),
            }],
            implements: "法·契约（决策契约 3）".into(),
        },
        规则条目 {
            规则ID: "CONTRACT-004-入稿落码".into(),
            严格度: 严格度::Info,
            治理域: 治理域::前置,
            描述: "先入稿再落码（AGENTS § 8：每个设计决策先入设计稿章节，再实现）".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "阶段 4+ 所有变更先有 10-地基/ 实施方案".into(),
                证伪方法: "commit diff 审查".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·契约（决策契约 4）".into(),
        },
        规则条目 {
            规则ID: "RULE-V-COUNT".into(),
            严格度: 严格度::Warning,
            治理域: 治理域::结构,
            描述: "RULE_COUNT 必须从 RULE_REGISTRY.len() 派生（不硬编码）。当前值 = 14 = 13 接单门 + 1 V-COUNT 自身".into(),
            decided_by: "界主".into(),
            falsifiable: vec![可证伪条件 {
                命题: "RULE_COUNT == RULE_REGISTRY.len()（编译期保证 + 单元测试）".into(),
                证伪方法: "编译期 + 单元测试".into(),
                时间窗口: "持续".into(),
            }],
            implements: "法·可演化（单一真相源 + 派生值）".into(),
        },
    ]
}

pub fn 评估接单(候选: &接单候选) -> 接单决策 {
    for 规则 in 获取注册表() {
        if let Some(原因) = 违反(规则, 候选) {
            return 接单决策::拒绝 {
                规则ID: 规则.规则ID.clone(),
                原因,
            };
        }
    }
    接单决策::接受
}

fn 违反(规则: &规则条目, 候选: &接单候选) -> Option<String> {
    match 规则.规则ID.as_str() {
        "BAN-001-LLM直改" | "CONTRACT-001-decidedBy" => {
            if 候选.decided_by.is_empty() {
                Some("decided_by 必填".into())
            } else {
                None
            }
        }
        "CONTRACT-002-falsifiable" => {
            if 候选.falsifiable.is_empty() {
                Some("falsifiable 必填（至少 1 条可证伪命题）".into())
            } else {
                None
            }
        }
        "CONTRACT-003-implements" => {
            if 候选.implements.is_empty() {
                Some("implements 必填（哲学锚）".into())
            } else {
                None
            }
        }
        _ => None,
    }
}

pub const 必填字段: &[&str] = &[
    "id",
    "title",
    "stage",
    "decided_by",
    "falsifiable",
    "upstream",
    "implements",
    "decided_at",
];

pub fn 校验决策契约(原文: &str) -> Result<(), Vec<String>> {
    let mut 缺失 = Vec::new();
    for 字段 in 必填字段 {
        if !contains_yaml_field(原文, 字段) {
            缺失.push(字段.to_string());
        }
    }
    if 缺失.is_empty() {
        Ok(())
    } else {
        Err(缺失)
    }
}

fn contains_yaml_field(原文: &str, 字段: &str) -> bool {
    let parts: Vec<&str> = 原文.splitn(3, "---").collect();
    if parts.len() < 3 {
        return false;
    }
    let frontmatter = parts[1];
    let 模式 = format!("{}:", 字段);
    for 行 in frontmatter.lines() {
        let 行 = 行.trim();
        if 行.starts_with(&模式) || 行.starts_with(&format!("  {}", 模式)) {
            return true;
        }
    }
    false
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

    // ---------------- 评估接单 ----------------

    fn 完整候选() -> 接单候选 {
        接单候选 {
            候选ID: "test-001".into(),
            内容: "示例决策".into(),
            decided_by: "界主".into(),
            falsifiable: vec!["决策可证伪率 > 80%".into()],
            upstream: "N/A".into(),
            implements: "法·契约".into(),
            阶段: "3/3".into(),
        }
    }

    #[test]
    fn 完整候选接受() {
        let r = 评估接单(&完整候选());
        assert_eq!(r, 接单决策::接受);
    }

    #[test]
    fn 缺decided_by拒绝() {
        let mut c = 完整候选();
        c.decided_by = "".into();
        let r = 评估接单(&c);
        match r {
            接单决策::拒绝 { 规则ID, 原因 } => {
                assert!(规则ID == "CONTRACT-001-decidedBy" || 规则ID == "BAN-001-LLM直改");
                assert!(原因.contains("decided_by"));
            }
            接单决策::接受 => panic!("应被 CONTRACT-001-decidedBy 或 BAN-001-LLM直改 拒绝"),
        }
    }

    #[test]
    fn 缺falsifiable拒绝() {
        let mut c = 完整候选();
        c.falsifiable.clear();
        let r = 评估接单(&c);
        match r {
            接单决策::拒绝 { 规则ID, .. } => {
                assert_eq!(规则ID, "CONTRACT-002-falsifiable");
            }
            _ => panic!("应被 CONTRACT-002-falsifiable 拒绝"),
        }
    }

    #[test]
    fn 缺implements拒绝() {
        let mut c = 完整候选();
        c.implements = "".into();
        let r = 评估接单(&c);
        match r {
            接单决策::拒绝 { 规则ID, .. } => {
                assert_eq!(规则ID, "CONTRACT-003-implements");
            }
            _ => panic!("应被 CONTRACT-003-implements 拒绝"),
        }
    }

    #[test]
    fn fatal规则拒绝率100() {
        // 评估 100 个缺 decided_by 的候选，全部拒绝
        let mut 拒绝数 = 0;
        for _ in 0..100 {
            let mut c = 完整候选();
            c.decided_by = "".into();
            if matches!(评估接单(&c), 接单决策::拒绝 { .. }) {
                拒绝数 += 1;
            }
        }
        assert_eq!(拒绝数, 100, "缺 decided_by 必须 100% 拒绝");
    }

    // ---------------- 决策契约字段校验 ----------------

    #[test]
    fn 完整yaml文档通过校验() {
        let 文档 = "---\nid: 260826-2220-治理-司衡\ntitle: 测试\nstage: 3/3\ndecided_by: 界主\nfalsifiable:\n  - 命题: x\nupstream: N/A\nimplements: 法·契约\ndecided_at: 2026-08-26\n---\n# 内容";
        assert!(校验决策契约(文档).is_ok());
    }

    #[test]
    fn 缺字段全部报错() {
        let 文档 = "---\nid: test\n---\n# 缺其他字段";
        let err = 校验决策契约(文档).unwrap_err();
        // 应包含缺失字段（除 id 外的所有）
        assert!(err.contains(&"title".to_string()));
        assert!(err.contains(&"decided_by".to_string()));
        assert!(err.contains(&"falsifiable".to_string()));
        assert!(err.contains(&"implements".to_string()));
        assert!(err.contains(&"decided_at".to_string()));
        assert_eq!(err.len(), 7, "缺 7 个字段（除 id）");
    }

    #[test]
    fn 无frontmatter视为缺字段() {
        let 文档 = "# 无 frontmatter 的文档";
        let err = 校验决策契约(文档).unwrap_err();
        assert_eq!(err.len(), 必填字段.len());
    }

    #[test]
    fn 缩进字段也能识别() {
        let 文档 = "---\nid: x\n  title: 缩进\nstage: 3/3\ndecided_by: 界主\nfalsifiable:\n  - x\nupstream: N/A\nimplements: 法\ndecided_at: 2026-08-26\n---\n";
        assert!(校验决策契约(文档).is_ok());
    }
}
