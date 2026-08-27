//! 提取契约 - 提取特征

pub trait 提取 {
    fn 提取(&self, 模式: &str, 文本: &str) -> Option<String>;
}
