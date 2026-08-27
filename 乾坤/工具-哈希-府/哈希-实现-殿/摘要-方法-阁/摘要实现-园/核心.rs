//! 摘要实现 - FNV-1a 哈希

use crate::哈希_接口_殿::摘要_阁::摘要契约_园::摘要;

pub struct 摘要实现;

impl 摘要 for 摘要实现 {
    fn 哈希(&self, 输入: &[u8]) -> u64 {
        let mut 状态: u64 = 0xcbf29ce484222325;
        for &b in 输入 {
            状态 ^= b as u64;
            状态 = 状态.wrapping_mul(0x100000001b3);
        }
        状态
    }
}
