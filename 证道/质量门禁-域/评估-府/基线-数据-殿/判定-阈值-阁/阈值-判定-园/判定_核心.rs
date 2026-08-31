//! 阈值判定 - 基线值通过内存阈值判定
//!
//! 决策锚：v4 阶段 15 性能基准 § 内存 < 200MB

use super::super::super::基线_数值_园::基线值;

impl 基线值 {
    pub fn 通过_200MB(&self) -> bool {
        self.内存_兆() < 200
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_通过_200MB() {
        let b = 基线值 {
            名称: "x".into(),
            耗时: std::time::Duration::from_millis(1),
            内存_字节: 100 * 1_048_576,
        };
        assert!(b.通过_200MB());
        let b2 = 基线值 {
            名称: "y".into(),
            耗时: std::time::Duration::from_millis(1),
            内存_字节: 300 * 1_048_576,
        };
        assert!(!b2.通过_200MB());
    }
}
