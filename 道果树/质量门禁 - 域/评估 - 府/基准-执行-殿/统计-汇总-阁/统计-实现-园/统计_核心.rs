//! 统计汇总 - 基准器的统计查询方法

use super::super::super::super::基线_数据_殿::结构_定义_阁::基线_值_园::基线值;
use super::super::super::运行_执行_阁::运行_实现_园::基准器;
use std::time::Duration;

impl 基准器 {
    pub fn 全部(&self) -> &[基线值] {
        &self.基线
    }
    pub fn 总耗时(&self) -> Duration {
        self.基线.iter().map(|b| b.耗时).sum()
    }
    pub fn 总内存(&self) -> u64 {
        self.基线.iter().map(|b| b.内存_字节).sum()
    }
    pub fn 通过_内存阈值(&self, 阈值_兆: u64) -> bool {
        self.基线.iter().all(|b| b.内存_兆() < 阈值_兆)
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_通过_内存阈值() {
        let mut b = 基准器::新建();
        b.跑("a", || {});
        b.跑("b", || {});
        assert!(b.通过_内存阈值(200));
    }
    #[test]
    fn 测试_基准_全_5项() {
        let b = super::super::super::super::度量_查询_阁::内存_度量_园::基准_全();
        assert_eq!(b.基线.len(), 5);
    }
}
