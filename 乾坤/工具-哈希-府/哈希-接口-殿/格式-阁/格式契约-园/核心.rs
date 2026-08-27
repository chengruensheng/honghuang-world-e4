//! 格式契约 - 格式化特征

pub trait 格式 {
    fn 十六进制(&self, 值: u64) -> String;
}
