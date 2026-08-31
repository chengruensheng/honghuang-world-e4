//! 结构 - 原子计数器探针数据结构
//!
//! 决策锚：260826-2230 工程-DSH

use std::sync::atomic::AtomicU64;

/// 原子计数器探针
pub struct 计数器探针 {
    pub 名称: String,
    pub 计数: AtomicU64,
}

impl 计数器探针 {
    pub fn 新建(名称: impl Into<String>) -> Self {
        Self {
            名称: 名称.into(),
            计数: AtomicU64::new(0),
        }
    }
}
