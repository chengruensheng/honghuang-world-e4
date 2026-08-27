//! 拼接契约 - 路径拼接特征

pub trait 拼接 {
    fn 拼接(&self, a: &str, b: &str) -> String;
}
