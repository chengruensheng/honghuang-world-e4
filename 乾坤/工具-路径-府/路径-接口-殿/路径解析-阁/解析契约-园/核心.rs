//! 解析契约 - 路径解析特征

pub trait 路径解析 {
    fn 解析(&self, p: &str) -> (String, String);
}
