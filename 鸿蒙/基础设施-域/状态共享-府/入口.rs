//! 状态共享 - 府
//!
//! 进程级状态读写：键-值映射 + 版本号。
//! 阶段 1 单进程实现，阶段 7+ 可换多进程同步。
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件（scope 子系统）
//! 关联文档：DSH 架构 scope 子系统

use std::sync::{Arc, RwLock};

/// 状态共享容器：键 → 序列化值 + 版本号
#[derive(Default, Clone)]
pub struct 状态共享 {
    inner: Arc<RwLock<内部>>,
}

#[derive(Default)]
struct 内部 {
    数据: std::collections::BTreeMap<String, 状态值>,
    版本: u64,
}

#[derive(Clone, Debug)]
pub struct 状态值 {
    pub 版本: u64,
    pub 值: String,
}

impl 状态共享 {
    pub fn 新建() -> Self {
        Self::default()
    }

    /// 写入并自动 bump 版本号
    pub fn 写(&self, 键: impl Into<String>, 值: impl Into<String>) {
        let mut inner = self.inner.write().expect("状态共享锁中毒");
        let 键 = 键.into();
        inner.版本 += 1;
        let 新版本 = inner.版本;
        inner.数据.insert(
            键,
            状态值 {
                版本: 新版本,
                值: 值.into(),
            },
        );
    }

    /// 按键读取当前值
    pub fn 读(&self, 键: &str) -> Option<状态值> {
        let inner = self.inner.read().expect("状态共享锁中毒");
        inner.数据.get(键).cloned()
    }

    /// 当前版本号
    pub fn 版本(&self) -> u64 {
        self.inner.read().expect("状态共享锁中毒").版本
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 读写一致() {
        let s = 状态共享::新建();
        s.写("道", "洪荒");
        let got = s.读("道").unwrap();
        assert_eq!(got.值, "洪荒");
    }

    #[test]
    fn 每次写入版本号加一() {
        let s = 状态共享::新建();
        let v0 = s.版本();
        s.写("a", "1");
        s.写("b", "2");
        assert_eq!(s.版本(), v0 + 2);
    }
}
