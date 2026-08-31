//! 端口配置 - mock HTTP server 默认端口

pub const 默认端口: u16 = 8080;
pub const 端口范围_最小: u16 = 1024;
pub const 端口范围_最大: u16 = 65535;

pub fn 端口_有效(端口: u16) -> bool {
    (端口范围_最小..=端口范围_最大).contains(&端口)
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 默认端口有效() {
        assert!(端口_有效(默认端口));
    }
    #[test]
    fn 端口_0_系统分配_超出常规范围() {
        // 0 表示让 OS 分配，但不在 1024-65535 常规范围内
        assert!(!端口_有效(0));
    }
}
