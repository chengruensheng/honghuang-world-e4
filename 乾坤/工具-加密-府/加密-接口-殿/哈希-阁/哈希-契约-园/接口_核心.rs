//! 接口_核心.rs - SHA256 接口定义

pub trait 哈希 {
    type 输出: Sized;
    fn 计算(&self, 输入: &[u8]) -> Self::输出;
}

pub struct SHA256;
