//! 版本标识 - CLI 版本字符串

pub const 版本: &str = "v0.2.0";
pub const 项目: &str = "洪荒 · 世界";

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 版本非空() {
        assert!(!版本.is_empty());
        assert!(版本.starts_with('v'));
        assert!(!项目.is_empty());
    }
}
