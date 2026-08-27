//! 时刻契约 - 时间戳特征

pub trait 时刻 {
    fn 当前(&self) -> u64;
}
