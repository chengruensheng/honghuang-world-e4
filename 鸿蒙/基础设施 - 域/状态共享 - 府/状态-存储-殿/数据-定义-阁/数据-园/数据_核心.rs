//! 数据定义 - 状态共享内部数据结构
//!
//! 决策锚：260826-2230 工程-DSH § scope
//! 关联文档：04-设计/接口契约/状态共享.md

use std::sync::RwLock;

/// 内部存储：键值映射 + 版本号
#[derive(Default)]
pub struct 内部 {
    pub 数据: std::collections::BTreeMap<String, 状态值>,
    pub 版本: u64,
}

/// 锁包装
pub struct 内部锁 {
    pub 锁: RwLock<内部>,
}

impl Default for 内部锁 {
    fn default() -> Self {
        Self {
            锁: RwLock::new(内部::default()),
        }
    }
}

/// 单条状态值（带版本）
#[derive(Clone, Debug)]
pub struct 状态值 {
    pub 版本: u64,
    pub 值: String,
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_内部默认值() {
        let i = 内部::default();
        assert_eq!(i.版本, 0);
        assert!(i.数据.is_empty());
    }
}
