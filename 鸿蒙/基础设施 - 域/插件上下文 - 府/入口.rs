//! 插件上下文 - 府
//!
//! 服务定义特征 + 注册表 + 类型擦除查找 + 替换语义 + manifest 解析。
//! 万物皆插件（DSH § 万物皆插件）的入口。
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件
//! 关联文档：02-概念/可插拔/01-可插拔.md
//! 接口契约：04-设计/接口契约/01-插件-manifest.md

use std::any::{Any, TypeId};
use std::collections::HashMap;

// ============================================================================
// 能力描述（manifest 的契约层）
// ============================================================================

/// 能力描述：插件对外暴露的接口单元
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 能力描述 {
    pub 名称: String,
    pub 输入: String,
    pub 输出: String,
    pub 副作用: 副作用,
    pub 不可改: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum 副作用 {
    #[default]
    无,
    修改,
    外部,
}

#[derive(Clone, Debug, Default)]
pub struct 调用输入 {
    pub 参数: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct 调用输出 {
    pub 结果: String,
    pub 副作用已发生: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum 错误 {
    插件不存在(String),
    重复注册(String),
    类型不匹配,
    调用失败(String),
}

impl std::fmt::Display for 错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            错误::插件不存在(名) => write!(f, "插件不存在：{}", 名),
            错误::重复注册(名) => write!(f, "插件已存在：{}", 名),
            错误::类型不匹配 => write!(f, "类型不匹配"),
            错误::调用失败(msg) => write!(f, "调用失败：{}", msg),
        }
    }
}

impl std::error::Error for 错误 {}

// ============================================================================
// 服务定义特征
// ============================================================================

pub trait 服务定义: Any + Send + Sync {
    fn 名称(&self) -> &str;
    fn 版本(&self) -> &str {
        "0.1.0"
    }
    fn 能力清单(&self) -> Vec<能力描述> {
        Vec::new()
    }
    fn 调用(&self, _输入: 调用输入) -> Result<调用输出, 错误> {
        Err(错误::调用失败("该服务未提供调用入口".to_string()))
    }
    fn 任意(&self) -> &dyn Any;
}

// ============================================================================
// 注册表
// ============================================================================

#[derive(Default)]
pub struct 注册表 {
    by_name: HashMap<String, Box<dyn 服务定义>>,
    by_type: HashMap<TypeId, String>,
}

impl 注册表 {
    pub fn 新建() -> Self {
        Self::default()
    }

    pub fn 注册(&mut self, 服务: Box<dyn 服务定义>) -> Option<Box<dyn 服务定义>> {
        let 名称 = 服务.名称().to_string();
        let 旧 = self.by_name.insert(名称.clone(), 服务);
        if let Some(旧服务) = &旧 {
            let t = 旧服务.任意().type_id();
            self.by_type.remove(&t);
        }
        let t = self.by_name[&名称].任意().type_id();
        self.by_type.insert(t, 名称);
        旧
    }

    pub fn 查找(&self, 名称: &str) -> Option<&dyn Any> {
        self.by_name.get(名称).map(|s| s.任意())
    }

    pub fn 查找类型<T: Any>(&self) -> Option<&T> {
        let t = TypeId::of::<T>();
        let 名称 = self.by_type.get(&t)?;
        let any = self.by_name.get(名称)?;
        any.任意().downcast_ref::<T>()
    }

    pub fn 调用(&self, 名称: &str, 输入: 调用输入) -> Result<调用输出, 错误> {
        let s = self
            .by_name
            .get(名称)
            .ok_or_else(|| 错误::插件不存在(名称.to_string()))?;
        s.调用(输入)
    }

    pub fn 全部能力(&self) -> Vec<(String, Vec<能力描述>)> {
        self.by_name
            .iter()
            .map(|(名, s)| (名.clone(), s.能力清单()))
            .collect()
    }

    pub fn 数量(&self) -> usize {
        self.by_name.len()
    }

    pub fn 移除(&mut self, 名称: &str) -> Option<Box<dyn 服务定义>> {
        let 旧 = self.by_name.remove(名称)?;
        let t = 旧.任意().type_id();
        self.by_type.remove(&t);
        Some(旧)
    }

    pub fn 名称清单(&self) -> Vec<String> {
        let mut v: Vec<String> = self.by_name.keys().cloned().collect();
        v.sort();
        v
    }
}

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

#[cfg(test)]
mod 测试 {
    use super::*;

    struct 示例服务;

    impl 服务定义 for 示例服务 {
        fn 名称(&self) -> &str {
            "示例服务"
        }
        fn 任意(&self) -> &dyn Any {
            self
        }
    }

    struct 文件读取 {
        路径: String,
    }

    impl 服务定义 for 文件读取 {
        fn 名称(&self) -> &str {
            "文件读取"
        }
        fn 能力清单(&self) -> Vec<能力描述> {
            vec![能力描述 {
                名称: "读取".to_string(),
                输入: "路径".to_string(),
                输出: "内容".to_string(),
                副作用: 副作用::外部,
                不可改: true,
            }]
        }
        fn 调用(&self, 输入: 调用输入) -> Result<调用输出, 错误> {
            let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
            if 路径.is_empty() {
                return Err(错误::调用失败("路径缺失".to_string()));
            }
            Ok(调用输出 {
                结果: format!("已读：{}", 路径),
                副作用已发生: true,
            })
        }
        fn 任意(&self) -> &dyn Any {
            self
        }
    }

    struct 文件读取V2;

    impl 服务定义 for 文件读取V2 {
        fn 名称(&self) -> &str {
            "文件读取"
        }
        fn 任意(&self) -> &dyn Any {
            self
        }
        fn 调用(&self, 输入: 调用输入) -> Result<调用输出, 错误> {
            let 路径 = 输入.参数.get("路径").cloned().unwrap_or_default();
            Ok(调用输出 {
                结果: format!("v2 已读：{}", 路径),
                副作用已发生: true,
            })
        }
    }

    #[test]
    fn 注册表可注册与查找() {
        let mut t = 注册表::新建();
        t.注册(Box::new(示例服务));
        assert_eq!(t.数量(), 1);
        assert!(t.查找("示例服务").is_some());
        assert!(t.查找("不存在").is_none());
    }

    #[test]
    fn 按类型擦除查找() {
        let mut t = 注册表::新建();
        t.注册(Box::new(文件读取 {
            路径: "/a".to_string(),
        }));
        let s: Option<&文件读取> = t.查找类型::<文件读取>();
        assert!(s.is_some());
        assert_eq!(s.unwrap().路径, "/a");
    }

    #[test]
    fn 能力清单可枚举() {
        let mut t = 注册表::新建();
        t.注册(Box::new(文件读取 {
            路径: "/a".to_string(),
        }));
        let 能力s = t.全部能力();
        assert_eq!(能力s.len(), 1);
        assert_eq!(能力s[0].0, "文件读取");
        assert_eq!(能力s[0].1.len(), 1);
        assert!(能力s[0].1[0].不可改);
    }

    #[test]
    fn 调用统一入口() {
        let mut t = 注册表::新建();
        t.注册(Box::new(文件读取 {
            路径: "/init".to_string(),
        }));
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), "/x".to_string());
        let out = t.调用("文件读取", 输入).unwrap();
        assert_eq!(out.结果, "已读：/x");
    }

    #[test]
    fn 同名服务替换不影响其他() {
        let mut t = 注册表::新建();
        t.注册(Box::new(示例服务));
        t.注册(Box::new(文件读取 {
            路径: "/a".to_string(),
        }));
        assert_eq!(t.数量(), 2);
        let 旧 = t.注册(Box::new(文件读取V2));
        assert!(旧.is_some(), "替换应返回旧实例");
        assert_eq!(t.数量(), 2, "替换后总数不变");
        // 示例服务 仍可访问（DSH 万物皆可替换：替换 A 不影响 B）
        assert!(t.查找("示例服务").is_some());
        // 文件读取 现在是 V2
        let mut 输入 = 调用输入::default();
        输入.参数.insert("路径".to_string(), "/y".to_string());
        let out = t.调用("文件读取", 输入).unwrap();
        assert!(out.结果.starts_with("v2"));
    }

    #[test]
    fn 移除按名() {
        let mut t = 注册表::新建();
        t.注册(Box::new(示例服务));
        t.注册(Box::new(文件读取 {
            路径: "/a".to_string(),
        }));
        let 移除 = t.移除("文件读取");
        assert!(移除.is_some());
        assert_eq!(t.数量(), 1);
        assert!(t.查找("文件读取").is_none());
    }

    #[test]
    fn 名称清单排序() {
        let mut t = 注册表::新建();
        t.注册(Box::new(示例服务));
        t.注册(Box::new(文件读取 {
            路径: "/a".to_string(),
        }));
        let 清单 = t.名称清单();
        // UTF-8 字节序："件" (0xE4 BBB3) 早于 "示" (0xE7 A4 BA)
        assert_eq!(清单, vec!["文件读取".to_string(), "示例服务".to_string()]);
    }

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
