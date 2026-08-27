//! 接口_核心.rs - 校验接口定义 + 等值校验实现

pub trait 校验 {
    fn 验证(&self, 输入: &[u8], 期望: &[u8]) -> bool;
}

pub struct 等值校验;

impl 校验 for 等值校验 {
    fn 验证(&self, 输入: &[u8], 期望: &[u8]) -> bool {
        // 借用 crate 顶级加密实现（避免重复）
        let 实际 = crate::sha256(输入);
        实际 == 期望
    }
}
