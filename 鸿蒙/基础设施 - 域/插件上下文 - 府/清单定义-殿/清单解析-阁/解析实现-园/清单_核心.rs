//! manifest 殿 - 插件清单解析（名称/实现/依赖/副作用/decided_by/falsifiable/implements）
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件
//! 关联文档：02-概念/可插拔/01-可插拔.md
//! 接口契约：04-设计/接口契约/01-插件-manifest.md

// 跨殿引用：副作用定义在注册殿（六层返工后改用 crate:: 路径）
use crate::插件注册_殿::副作用;

// ============================================================================
// Manifest
// ============================================================================

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Manifest {
    pub 名称: String,
    pub 实现: Vec<String>,
    pub 依赖: Vec<String>,
    pub 副作用: 副作用,
    pub 不可改: bool,
    pub decided_by: String,
    pub falsifiable: String,
    pub implements: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum Manifest错误 {
    字段缺失(String),
    字段类型错(String),
    行格式错(String),
}

impl std::fmt::Display for Manifest错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Manifest错误::字段缺失(名) => write!(f, "manifest 字段缺失：{}", 名),
            Manifest错误::字段类型错(名) => write!(f, "manifest 字段类型错：{}", 名),
            Manifest错误::行格式错(行) => write!(f, "manifest 行格式错：{}", 行),
        }
    }
}

impl std::error::Error for Manifest错误 {}

impl Manifest {
    pub fn 解析(原文: &str) -> Result<Self, Manifest错误> {
        let mut m = Manifest::default();
        for 行 in 原文.lines() {
            let 行 = 行.trim();
            if 行.is_empty() || 行.starts_with('#') || 行.starts_with('[') {
                continue;
            }
            let kv: Vec<&str> = 行.splitn(2, '=').collect();
            if kv.len() != 2 {
                return Err(Manifest错误::行格式错(行.to_string()));
            }
            let key = kv[0].trim();
            let value = kv[1].trim();
            match key {
                "名称" => m.名称 = parse_string(value)?,
                "副作用" => m.副作用 = parse_side_effect(value)?,
                "不可改" => m.不可改 = parse_bool(value)?,
                "decided_by" => m.decided_by = parse_string(value)?,
                "falsifiable" => m.falsifiable = parse_string(value)?,
                "implements" => m.implements = parse_string(value)?,
                "实现" | "依赖" => {
                    let arr = parse_array(value)?;
                    if key == "实现" {
                        m.实现 = arr;
                    } else {
                        m.依赖 = arr;
                    }
                }
                _ => { /* 跳过未知字段（如 [资源限制] 段、嵌套对象） */ }
            }
        }
        if m.名称.is_empty() {
            return Err(Manifest错误::字段缺失("名称".to_string()));
        }
        if m.decided_by.is_empty() {
            return Err(Manifest错误::字段缺失("decided_by".to_string()));
        }
        Ok(m)
    }
}

fn parse_string(value: &str) -> Result<String, Manifest错误> {
    let trimmed = value.trim().trim_matches(',');
    Ok(trimmed.trim_matches('"').trim_matches('\'').to_string())
}

fn parse_bool(value: &str) -> Result<bool, Manifest错误> {
    match value.trim().trim_matches(',') {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(Manifest错误::字段类型错("bool".to_string())),
    }
}

fn parse_side_effect(value: &str) -> Result<副作用, Manifest错误> {
    match value.trim().trim_matches(',').trim_matches('"') {
        "none" => Ok(副作用::无),
        "mutating" => Ok(副作用::修改),
        "external" => Ok(副作用::外部),
        _ => Err(Manifest错误::字段类型错("副作用".to_string())),
    }
}

fn parse_array(value: &str) -> Result<Vec<String>, Manifest错误> {
    let trimmed = value
        .trim()
        .trim_matches(',')
        .trim_matches(|c| c == '[' || c == ']');
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    Ok(trimmed
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
        .collect())
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn manifest解析基础字段() {
        let raw = "\n# 注释\n名称 = \"测试插件\"\n副作用 = \"none\"\n不可改 = true\ndecided_by = \"界主\"\nfalsifiable = \"替换成功率>95%\"\nimplements = \"法·可演化\"\n实现 = [\"trait A\", \"trait B\"]\n依赖 = [\"插件X\"]\n";
        let m = Manifest::解析(raw).unwrap();
        assert_eq!(m.名称, "测试插件");
        assert_eq!(m.副作用, 副作用::无);
        assert!(m.不可改);
        assert_eq!(m.decided_by, "界主");
        assert_eq!(m.实现, vec!["trait A", "trait B"]);
        assert_eq!(m.依赖, vec!["插件X"]);
    }

    #[test]
    fn manifest必填校验() {
        let raw = "副作用 = \"none\"";
        let err = Manifest::解析(raw).unwrap_err();
        assert_eq!(err, Manifest错误::字段缺失("名称".to_string()));
    }

    #[test]
    fn manifest跳过段标记() {
        let raw = "\n[资源限制]\n内存上限 = \"256MB\"\n\n名称 = \"测试\"\n副作用 = \"external\"\n不可改 = false\ndecided_by = \"界主\"\n";
        let m = Manifest::解析(raw).unwrap();
        assert_eq!(m.名称, "测试");
        assert_eq!(m.副作用, 副作用::外部);
    }

    #[test]
    fn manifest字段类型错() {
        let raw = "\n名称 = \"测试\"\n副作用 = \"未知\"\n不可改 = true\ndecided_by = \"界主\"\n";
        assert!(Manifest::解析(raw).is_err());
    }
}
