//! 校验实现 - 决策契约字段校验 + 接单评估
//!
//! 评估接单：遍历 RULE_REGISTRY，首条违反即拒绝。
//! 决策契约校验：8 必填字段 frontmatter 检查。
//!
//! 决策锚：260826-2220 治理-司衡 + 260826-2240 传承殿启动
//! 关联文档：00-宪法/DECISION-CONTRACT.md

use crate::规则_注册_殿::{接单候选, 接单决策, 获取注册表};

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

fn 违反(
    规则: &crate::规则_注册_殿::规则条目, 候选: &接单候选
) -> Option<String> {
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

/// 简化校验：仅检查 decided_by 非空 + falsifiable 包含可证伪命题
/// 用于事件流 / 记忆写入路径（不需要完整 8 字段 frontmatter）
pub fn 校验关键字段(原文: &str) -> Result<(), Vec<String>> {
    let mut 缺失 = Vec::new();
    if !原文.lines().any(|行| {
        行.contains("decided_by:")
            && 行
                .split_once(':')
                .map(|x| !x.1.trim().is_empty())
                .unwrap_or(false)
    }) {
        缺失.push("decided_by".to_string());
    }
    if !原文.lines().any(|行| {
        行.contains("falsifiable:")
            && 行
                .split_once(':')
                .map(|x| !x.1.trim().is_empty())
                .unwrap_or(false)
    }) {
        缺失.push("falsifiable".to_string());
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
    use crate::规则_注册_殿::接单候选;

    #[test]
    fn 校验关键字段_有效() {
        assert!(校验关键字段("decided_by: 界主\nfalsifiable: 上线 1 周").is_ok());
    }

    #[test]
    fn 校验关键字段_缺decided_by拒绝() {
        let r = 校验关键字段("falsifiable: 上线 1 周");
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), vec!["decided_by".to_string()]);
    }

    #[test]
    fn 校验关键字段_空decided_by拒绝() {
        let r = 校验关键字段("decided_by:\nfalsifiable: 上线 1 周");
        assert!(r.is_err());
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
