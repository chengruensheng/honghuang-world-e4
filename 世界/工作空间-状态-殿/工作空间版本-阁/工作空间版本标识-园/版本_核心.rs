//! 工作空间版本标识

pub const 版本: &str = "v0.1.0";
pub const 阶段: &str = "阶段 1 地基";

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 版本字符串非空() {
        assert!(!版本.is_empty());
        assert!(!阶段.is_empty());
    }
}
