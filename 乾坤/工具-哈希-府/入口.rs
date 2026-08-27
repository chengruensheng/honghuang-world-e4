//! 工具-哈希-府
//!
//! 决策锚：260827-AI助手自给自足（Round 11.5）
//! 6 层结构：2 殿（接口-殿 + 实现-殿）× 2 阁 × 2 园

#![allow(non_snake_case)]

#[path = "哈希-实现-殿/模块.rs"]
pub mod 哈希_实现_殿;
#[path = "哈希-接口-殿/模块.rs"]
pub mod 哈希_接口_殿;

pub use 哈希_实现_殿::摘要_方法_阁::摘要实现_园::摘要实现;
pub use 哈希_实现_殿::格式_方法_阁::格式实现_园::格式实现;
pub use 哈希_接口_殿::摘要_阁::摘要契约_园::摘要;
pub use 哈希_接口_殿::格式_阁::格式契约_园::格式;

/// FNV-1a 哈希
pub fn 哈希(输入: &[u8]) -> u64 {
    let mut 状态: u64 = 0xcbf29ce484222325;
    for &b in 输入 {
        状态 ^= b as u64;
        状态 = 状态.wrapping_mul(0x100000001b3);
    }
    状态
}

/// 十六进制格式化
pub fn 十六进制(值: u64) -> String {
    format!("{:016x}", 值)
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 哈希_确定性() {
        assert_eq!(哈希(b"hello"), 哈希(b"hello"));
    }
    #[test]
    fn 哈希_不同输入不同() {
        assert_ne!(哈希(b"hello"), 哈希(b"world"));
    }
    #[test]
    fn 十六进制_格式() {
        assert_eq!(十六进制(255), "00000000000000ff");
    }
}
