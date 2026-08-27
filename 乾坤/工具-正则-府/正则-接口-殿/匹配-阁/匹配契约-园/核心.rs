//! 匹配契约 - 匹配特征

pub trait 匹配 {
    fn 匹配(&self, 模式: &str, 文本: &str) -> bool;
}
