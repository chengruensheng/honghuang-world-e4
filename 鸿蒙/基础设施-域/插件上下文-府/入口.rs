//! 插件上下文 - 府
//!
//! 提供服务定义特征、注册表、查找与替换能力。
//! 万物皆插件（DSH § 万物皆插件）的入口。
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件
//! 关联文档：02-概念/可插拔/01-可插拔.md

use std::any::Any;

/// 服务定义特征：所有可注册到注册表的服务必须实现此特征。
///
/// 此特征是接 02-概念/可插拔/01-可插拔.md 的工程入口：
/// 插件必须自我描述（名称 + 版本）且支持查找（Any）。
pub trait 服务定义: Any + Send + Sync {
    /// 返回服务唯一名称（建议 pinyin_xxx_fu 格式以统一命名）
    fn 名称(&self) -> &str;

    /// 返回语义化版本（默认 0.1.0）
    fn 版本(&self) -> &str {
        "0.1.0"
    }

    /// 向下转型为 `&dyn Any`，供注册表查找时使用
    fn 任意(&self) -> &dyn Any;
}

/// 插件注册表：键-值映射 + 类型擦除查找。
///
/// 阶段 1 仅承载结构；阶段 2（事件流 + 插件上下文）实现完整替换语义。
#[derive(Default)]
pub struct 注册表 {
    服务: std::collections::HashMap<String, Box<dyn 服务定义>>,
}

impl 注册表 {
    /// 构造空注册表
    pub fn 新建() -> Self {
        Self::default()
    }

    /// 注册一个服务；同名服务覆盖前一服务（DSH 万物皆可替换）
    pub fn 注册(&mut self, 服务: Box<dyn 服务定义>) {
        let 名称 = 服务.名称().to_string();
        self.服务.insert(名称, 服务);
    }

    /// 按名称查找服务，返回 `Option<&dyn Any>`
    pub fn 查找(&self, 名称: &str) -> Option<&dyn Any> {
        self.服务.get(名称).map(|s| s.任意())
    }

    /// 当前已注册服务数
    pub fn 数量(&self) -> usize {
        self.服务.len()
    }
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

    #[test]
    fn 注册表可注册与查找() {
        let mut 注册表 = 注册表::新建();
        注册表.注册(Box::new(示例服务));
        assert_eq!(注册表.数量(), 1);
        assert!(注册表.查找("示例服务").is_some());
        assert!(注册表.查找("不存在").is_none());
    }
}
