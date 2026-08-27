//! 容器定义 - 状态共享容器
//!
//! 决策锚：260826-2230 工程-DSH

use crate::状态_存储_殿::数据_定义_阁::数据_园::内部锁;
use std::sync::Arc;

/// 状态共享容器：键 → 序列化值 + 版本号
#[derive(Default, Clone)]
pub struct 状态共享 {
    pub inner: Arc<内部锁>,
}

impl 状态共享 {
    pub fn 新建() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_新建() {
        let s = 状态共享::新建();
        assert_eq!(s.inner.锁.read().unwrap().版本, 0);
    }
}
