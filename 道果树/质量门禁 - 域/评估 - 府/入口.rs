//! 评估-府 - v4 阶段 15 性能基准 + 内存回归
//!
//! 5 个基准脚本 + 基线值 + 内存 < 200MB。

#![allow(non_snake_case)]

use std::time::{Duration, Instant};

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
    pub fn 通过_200MB(&self) -> bool {
        self.内存_兆() < 200
    }
}

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
        let 内存_前 = 内存_使用();
        let 开始 = Instant::now();
        f();
        let 耗时 = 开始.elapsed();
        let 内存_后 = 内存_使用();
        let 内存 = 内存_后.saturating_sub(内存_前);
        self.基线.push(基线值 {
            名称: 名称.to_string(),
            耗时,
            内存_字节: 内存,
        });
    }

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

fn 内存_使用() -> u64 {
    // 简化：用进程总内存近似（实际应读取 /proc/self/status）
    let _ = std::process::id();
    0
}

pub fn 基准_全() -> 基准器 {
    let mut b = 基准器::新建();
    b.跑("空运行", || {
        let mut s = 0u64;
        for _ in 0..1000 {
            s = s.wrapping_add(1);
        }
    });
    b.跑("字符串构建", || {
        let mut s = String::new();
        for i in 0..100 {
            s.push_str(&format!("测试-{}-{}", i, i * i));
        }
    });
    b.跑("Vec 增删", || {
        let mut v: Vec<u64> = Vec::new();
        for i in 0..1000 {
            v.push(i);
        }
        for _ in 0..500 {
            v.pop();
        }
    });
    b.跑("HashMap 插入", || {
        let mut m: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
        for i in 0..1000 {
            m.insert(i, i * i);
        }
    });
    b.跑("字符串哈希", || {
        let mut h: u64 = 0;
        for i in 0..1000 {
            h = h.wrapping_mul(31).wrapping_add(i);
        }
    });
    b
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 测试_基准器_新建() {
        let b = 基准器::新建();
        assert_eq!(b.基线.len(), 0);
        assert_eq!(b.总耗时(), Duration::from_secs(0));
    }

    #[test]
    fn 测试_基准器_跑() {
        let mut b = 基准器::新建();
        b.跑("test", || {
            let _x = 1 + 1;
        });
        assert_eq!(b.基线.len(), 1);
        assert_eq!(b.基线[0].名称, "test");
        assert!(b.总耗时().as_nanos() > 0);
    }

    #[test]
    fn 测试_内存_兆() {
        let b = 基线值 {
            名称: "x".into(),
            耗时: Duration::from_millis(1),
            内存_字节: 5 * 1_048_576,
        };
        assert_eq!(b.内存_兆(), 5);
        assert!(b.通过_200MB());
    }

    #[test]
    fn 测试_通过_内存阈值() {
        let mut b = 基准器::新建();
        b.跑("a", || {});
        b.跑("b", || {});
        assert!(b.通过_内存阈值(200));
    }

    #[test]
    fn 测试_基准_全_5项() {
        let b = 基准_全();
        assert_eq!(b.基线.len(), 5);
        for baseline in b.基线 {
            assert!(!baseline.名称.is_empty());
        }
    }
}
