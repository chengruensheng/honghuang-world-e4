//! 计数操作 - 计数器探针的 trait impl
//!
//! 决策锚：260826-2230 工程-DSH

use crate::探针_接口_殿::接口_契约_阁::契约_落地_园::探针;
use crate::计数_核心_殿::结构_落地_园::计数器探针;
use std::sync::atomic::Ordering;

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
    use crate::计数_核心_殿::结构_落地_园::计数器探针;

    #[test]
    fn 计数器单调增() {
        let p = 计数器探针::新建("测试通道");
        assert_eq!(p.当前(), 0);
        assert_eq!(<计数器探针 as 探针>::计数(&p), 1);
        assert_eq!(<计数器探针 as 探针>::计数(&p), 2);
        assert_eq!(p.当前(), 2);
    }
}
