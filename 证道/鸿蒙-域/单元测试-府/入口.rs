//! 单元测试 - 府
//!
//! 测试承载（镜像被测服务）+ 概念测试枚举。
//! 阶段 1 提供宏观"模块名 → 通过项数"的统计特征。
//!
//! 决策锚：260826-2230 工程-DSH § 一键全验
//! 关联文档：05-质量/02-一键全验.md § 单元测试

/// 测试承载特征：每个被测模块配套一个测试承载 crate
pub trait 测试承载: Send + Sync {
    /// 被测模块名（与 Cargo.toml 中 workspace member 对应）
    fn 模块(&self) -> &str;
    /// 已通过测试项数（用于一键全验统计）
    fn 通过数(&self) -> u32;
}

/// 标准测试统计
pub struct 测试统计 {
    pub 模块: String,
    pub 通过: u32,
    pub 失败: u32,
}

impl 测试统计 {
    pub fn 新建(模块: impl Into<String>, 通过: u32, 失败: u32) -> Self {
        Self {
            模块: 模块.into(),
            通过,
            失败,
        }
    }
    pub fn 通过率(&self) -> f64 {
        let 总 = self.通过 + self.失败;
        if 总 == 0 {
            0.0
        } else {
            self.通过 as f64 / 总 as f64
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 通过率计算() {
        let s = 测试统计::新建("示例", 8, 2);
        assert!((s.通过率() - 0.8).abs() < 1e-9);
    }

    #[test]
    fn 零分母返回零() {
        let s = 测试统计::新建("空", 0, 0);
        assert_eq!(s.通过率(), 0.0);
    }
}
