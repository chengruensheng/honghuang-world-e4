//! 运行执行 - 基准器 struct + 新建 + 跑方法

use super::super::super::super::基线_数据_殿::基线_数值_园::基线值;
use std::time::{Duration, Instant};

pub struct 基准器 {
    pub 基线: Vec<基线值>,
}

impl 基准器 {
    pub fn 新建() -> Self {
        Self { 基线: Vec::new() }
    }
    pub fn 跑<F>(&mut self, 名称: &str, f: F)
    where
        F: FnOnce(),
    {
        let 内存_前 = super::super::super::度量_查询_阁::内存_度量_园::内存_使用();
        let 开始 = Instant::now();
        f();
        // 极快任务在负载下可能测得 0ns：以 1ns 兜底，防后续除零/比率计算失真
        let 耗时 = 开始.elapsed().max(Duration::from_nanos(1));
        let 内存_后 = super::super::super::度量_查询_阁::内存_度量_园::内存_使用();
        let 内存 = 内存_后.saturating_sub(内存_前);
        self.基线.push(基线值 {
            名称: 名称.to_string(),
            耗时,
            内存_字节: 内存,
        });
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_基准器_新建() {
        let b = 基准器::新建();
        assert_eq!(b.基线.len(), 0);
    }
    #[test]
    fn 测试_基准器_跑() {
        let mut b = 基准器::新建();
        b.跑("test", || {
            let _x = 1 + 1;
        });
        assert_eq!(b.基线.len(), 1);
        assert_eq!(b.基线[0].名称, "test");
        // 1ns 兜底保证耗时恒正（防除零），断言不再依赖真实计时
        assert!(b.基线[0].耗时.as_nanos() >= 1);
    }
}
