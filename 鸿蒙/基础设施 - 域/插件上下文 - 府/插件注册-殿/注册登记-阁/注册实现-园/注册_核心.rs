//! 注册殿 - 能力描述 + 服务定义特征 + 注册表（类型擦除查找 + 替换语义）
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
// 单元测试
// ============================================================================

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
}
