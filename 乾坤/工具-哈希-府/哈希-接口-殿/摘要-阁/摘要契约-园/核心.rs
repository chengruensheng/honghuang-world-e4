//! 摘要契约 - 哈希特征

pub trait 摘要 {
    fn 哈希(&self, 输入: &[u8]) -> u64;
}
