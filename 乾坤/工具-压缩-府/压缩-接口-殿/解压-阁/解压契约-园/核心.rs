//! 解压契约 - 解压特征

pub trait 解压 {
    fn 解压(&self, 数据: &[u8]) -> Vec<u8>;
}
