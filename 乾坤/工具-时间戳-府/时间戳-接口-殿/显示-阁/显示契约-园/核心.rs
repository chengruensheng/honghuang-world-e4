//! 显示契约 - 显示特征

pub trait 显示 {
    fn 显示(&self, 时间戳: u64) -> String;
}
