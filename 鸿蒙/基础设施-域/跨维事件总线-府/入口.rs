//! 跨维事件总线 - 府
//!
//! 三类事件（会话/智能体/能力）+ 两种分发模式（Waterfall/Serial）+ hash 链。
//! 治理动作必入事件流，留痕不可篡改（司衡 § 8）。
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件 + frozen outcome
//! 关联文档：02-概念/事件流/04-事件流.md + 02-概念/不可逆结果/07-不可逆结果.md

/// 事件分类（DSH 三类事件）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 事件类型 {
    /// 会话级事件（持久化）
    会话,
    /// 智能体级事件（live）
    智能体,
    /// 能力级事件（policy + adapter）
    能力,
}

/// 分发模式（DSH Waterfall/Serial）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 分发模式 {
    /// 责任链：每个监听器决定是否 `下一步`，不调则停
    瀑布,
    /// 串行广播：所有监听器必收，无 next 概念
    串行,
}

/// 事件载荷：所有事件共用的最小数据集
#[derive(Clone, Debug)]
pub struct 事件 {
    pub 类型: 事件类型,
    pub 名称: String,
    pub 时间戳_毫秒: u64,
    pub 哈希: u64,
}

/// 计算载荷的 64-bit 哈希（FNV-1a 简化版，阶段 1 即可满足可追溯）
///
/// 决策锚：02-概念/不可逆结果/07-不可逆结果.md § hash 校验
pub fn 算哈希(载荷: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for 字节 in 载荷 {
        h ^= *字节 as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 哈希稳定() {
        // 相同输入产生相同输出（frozen outcome 的必要条件）
        let a = 算哈希("洪荒 · 世界".as_bytes());
        let b = 算哈希("洪荒 · 世界".as_bytes());
        assert_eq!(a, b);
    }

    #[test]
    fn 哈希区分输入() {
        assert_ne!(算哈希("事件 A".as_bytes()), 算哈希("事件 B".as_bytes()));
    }

    #[test]
    fn 三类事件可枚举() {
        let 所有 = [事件类型::会话, 事件类型::智能体, 事件类型::能力];
        assert_eq!(所有.len(), 3);
    }
}
