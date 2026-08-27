//! 基线值 - 评估府的基线数据结构
//!
//! 决策锚：v4 阶段 15 性能基准
//! 关联文档：04-设计/数据模型/基线值.md

use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct 基线值 {
    pub 名称: String,
    pub 耗时: Duration,
    pub 内存_字节: u64,
}

impl 基线值 {
    pub fn 内存_兆(&self) -> u64 {
        self.内存_字节 / 1_048_576
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_内存_兆() {
        let b = 基线值 {
            名称: "x".into(),
            耗时: Duration::from_millis(1),
            内存_字节: 5 * 1_048_576,
        };
        assert_eq!(b.内存_兆(), 5);
    }
}
