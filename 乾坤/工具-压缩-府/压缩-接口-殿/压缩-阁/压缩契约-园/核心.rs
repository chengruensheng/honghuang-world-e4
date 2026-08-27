//! 压缩契约 - 压缩特征

pub trait 压缩 {
    fn 压缩(&self, 数据: &[u8]) -> Vec<u8>;
}
