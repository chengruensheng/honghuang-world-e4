//! 延迟配置 - mock HTTP server 默认延迟（毫秒）

pub const 默认延迟_毫秒: u64 = 50;
pub const 最小延迟_毫秒: u64 = 0;
pub const 最大延迟_毫秒: u64 = 5000;

pub fn 延迟_有效(毫秒: u64) -> bool {
    (最小延迟_毫秒..=最大延迟_毫秒).contains(&毫秒)
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 默认延迟有效() {
        assert!(延迟_有效(默认延迟_毫秒));
    }
}
