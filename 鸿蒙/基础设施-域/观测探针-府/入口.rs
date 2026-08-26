//! 观测探针 - 府
//!
//! 24 项事件通道埋点 + 指标采集。
//! 阶段 1 仅定义探针特征与计数器；阶段 9 接入实际监控。
//!
//! 决策锚：260826-2230 工程-DSH § 一键全验
//! 关联文档：07-运营/监控/02-监控.md

use std::sync::atomic::{AtomicU64, Ordering};

/// 探针特征：所有可埋点的组件必须实现
pub trait 探针: Send + Sync {
    /// 探针名（与埋点通道一一对应）
    fn 名称(&self) -> &str;

    /// 自增 1 并返回新值
    fn 计数(&self) -> u64;

    /// 当前计数
    fn 当前(&self) -> u64;
}

/// 原子计数器探针
pub struct 计数器探针 {
    名称: String,
    计数: AtomicU64,
}

impl 计数器探针 {
    pub fn 新建(名称: impl Into<String>) -> Self {
        Self {
            名称: 名称.into(),
            计数: AtomicU64::new(0),
        }
    }
}

impl 探针 for 计数器探针 {
    fn 名称(&self) -> &str {
        &self.名称
    }
    fn 计数(&self) -> u64 {
        self.计数.fetch_add(1, Ordering::Relaxed) + 1
    }
    fn 当前(&self) -> u64 {
        self.计数.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 计数器单调增() {
        let p = 计数器探针::新建("测试通道");
        assert_eq!(p.当前(), 0);
        assert_eq!(p.计数(), 1);
        assert_eq!(p.计数(), 2);
        assert_eq!(p.当前(), 2);
    }
}
