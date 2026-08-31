//! 写操作 - 状态共享写入（含版本递增）
//!
//! 决策锚：260826-2230 工程-DSH § scope

use crate::状态_存储_殿::容器_定义_阁::容器_落地_园::状态共享;

impl 状态共享 {
    /// 写入并自动 bump 版本号
    pub fn 写(&self, 键: impl Into<String>, 值: impl Into<String>) {
        let mut inner = self.inner.锁.write().expect("状态共享锁中毒");
        let 键 = 键.into();
        inner.版本 += 1;
        let 新版本 = inner.版本;
        inner.数据.insert(
            键,
            crate::状态_存储_殿::数据_定义_阁::数据_落地_园::状态值 {
                版本: 新版本,
                值: 值.into(),
            },
        );
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 每次写入版本号加一() {
        let s = 状态共享::新建();
        let v0 = s.inner.锁.read().unwrap().版本;
        s.写("a", "1");
        s.写("b", "2");
        assert_eq!(s.inner.锁.read().unwrap().版本, v0 + 2);
    }
}
