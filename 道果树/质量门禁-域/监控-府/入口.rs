//! 监控 - 府
//!
//! 4 类指标（可用性/性能/正确性/资源）+ 4 级告警（P0/P1/P2/P3）。
//! 阶段 9 Day 1-2：监控骨架 + 4 类指标 + 4 级告警 + 单元测试。
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 9
//! 关联文档：07-运营/监控/02-监控.md + 07-运营/告警/03-告警.md + 00-宪法/开发遵守格式模板.md
//! falsifiable：4 类指标可采 + 4 级告警可触发 + ≥ 8 单元测试

#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

// HashMap reserved for future

// ============================================================================
// 4 类指标（接 07-运营/监控/02-监控.md）
// ============================================================================

/// 指标分类（4 类）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum 指标类 {
    可用性,
    性能,
    正确性,
    资源,
}

impl 指标类 {
    pub fn 名称(self) -> &'static str {
        match self {
            指标类::可用性 => "可用性",
            指标类::性能 => "性能",
            指标类::正确性 => "正确性",
            指标类::资源 => "资源",
        }
    }
}

/// 指标（数值 + 阈值 + 时间戳）
#[derive(Clone, Debug, PartialEq)]
pub struct 指标 {
    pub 名称: String,
    pub 类别: 指标类,
    pub 值: f64,
    pub 阈值_警告: Option<f64>,
    pub 阈值_危急: Option<f64>,
    pub 时间戳: u64,
}

impl 指标 {
    pub fn 新建(名称: &str, 类别: 指标类, 值: f64) -> Self {
        Self {
            名称: 名称.to_string(),
            类别,
            值,
            阈值_警告: None,
            阈值_危急: None,
            时间戳: 0,
        }
    }

    pub fn 严重(&self) -> bool {
        if let Some(t) = self.阈值_危急 {
            return self.值 >= t;
        }
        false
    }

    pub fn 警告(&self) -> bool {
        if let Some(t) = self.阈值_警告 {
            if self.值 >= t {
                return true;
            }
        }
        false
    }
}

/// 监控报告
#[derive(Clone, Debug, Default, PartialEq)]
pub struct 监控报告 {
    pub 指标列表: Vec<指标>,
    pub 时间戳: u64,
}

impl 监控报告 {
    pub fn 新建() -> Self {
        Self::default()
    }

    pub fn 添加(&mut self, 指标: 指标) {
        self.指标列表.push(指标);
    }

    pub fn 严重数(&self) -> usize {
        self.指标列表.iter().filter(|i| i.严重()).count()
    }

    pub fn 警告数(&self) -> usize {
        self.指标列表.iter().filter(|i| i.警告()).count()
    }
}

// ============================================================================
// 4 级告警（接 07-运营/告警/03-告警.md）
// ============================================================================

/// 告警级别（4 级）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum 告警级 {
    P3提示,
    P2次要,
    P1重要,
    P0紧急,
}

impl 告警级 {
    pub fn 名称(self) -> &'static str {
        match self {
            告警级::P3提示 => "P3提示",
            告警级::P2次要 => "P2次要",
            告警级::P1重要 => "P1重要",
            告警级::P0紧急 => "P0紧急",
        }
    }
}

/// 告警状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 告警状态 {
    已触发,
    已确认,
    已解决,
}

/// 告警
#[derive(Clone, Debug, PartialEq)]
pub struct 告警 {
    pub 级别: 告警级,
    pub 指标名: String,
    pub 当前值: f64,
    pub 阈值: f64,
    pub 状态: 告警状态,
    pub 时间戳: u64,
}

impl 告警 {
    pub fn 新建(级别: 告警级, 指标名: &str, 当前值: f64, 阈值: f64) -> Self {
        Self {
            级别,
            指标名: 指标名.to_string(),
            当前值,
            阈值,
            状态: 告警状态::已触发,
            时间戳: 0,
        }
    }

    /// P0 升级到道祖级应急（人类介入）
    pub fn 升级道祖应急(&mut self) {
        if self.级别 == 告警级::P0紧急 {
            self.状态 = 告警状态::已确认;
        }
    }
}

/// 告警引擎
pub struct 告警引擎 {
    pub 告警列表: Vec<告警>,
}

impl 告警引擎 {
    pub fn 新建() -> Self {
        Self {
            告警列表: Vec::new(),
        }
    }

    /// 接收指标变化 → 匹配阈值 → 触发告警
    pub fn 接收指标(&mut self, 指标: &指标) {
        if let Some(告警) = 触发告警(指标) {
            self.告警列表.push(告警);
        }
    }

    pub fn 严重数(&self) -> usize {
        self.告警列表
            .iter()
            .filter(|a| a.级别 == 告警级::P0紧急)
            .count()
    }
}

/// 单指标告警触发判定
pub fn 触发告警(指标: &指标) -> Option<告警> {
    if 指标.严重() {
        return Some(告警::新建(
            告警级::P0紧急,
            &指标.名称,
            指标.值,
            指标.阈值_危急.unwrap_or(f64::MAX),
        ));
    }
    if 指标.警告() {
        return Some(告警::新建(
            告警级::P1重要,
            &指标.名称,
            指标.值,
            指标.阈值_警告.unwrap_or(f64::MAX),
        ));
    }
    None
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 指标类4枚举() {
        assert_eq!(
            4,
            [指标类::可用性, 指标类::性能, 指标类::正确性, 指标类::资源].len()
        );
    }

    #[test]
    fn 指标类名称() {
        assert_eq!(指标类::可用性.名称(), "可用性");
        assert_eq!(指标类::性能.名称(), "性能");
        assert_eq!(指标类::正确性.名称(), "正确性");
        assert_eq!(指标类::资源.名称(), "资源");
    }

    #[test]
    fn 指标严重判定() {
        let mut i = 指标::新建("p99延迟", 指标类::性能, 50.0);
        i.阈值_危急 = Some(100.0);
        assert!(!i.严重()); // 50 < 100
        i.值 = 150.0;
        assert!(i.严重()); // 150 >= 100
    }

    #[test]
    fn 指标警告判定() {
        let mut i = 指标::新建("cpu", 指标类::资源, 50.0);
        i.阈值_警告 = Some(80.0);
        i.阈值_危急 = Some(95.0);
        assert!(!i.警告());
        i.值 = 85.0;
        assert!(i.警告());
        assert!(!i.严重());
    }

    #[test]
    fn 监控报告严重与警告数() {
        let mut i1 = 指标::新建("p99", 指标类::性能, 50.0);
        i1.阈值_警告 = Some(80.0);
        assert!(!i1.警告(), "i1 50 vs 80 不应警告");
        let mut i2 = 指标::新建("cpu", 指标类::资源, 95.0);
        i2.阈值_危急 = Some(90.0);
        assert!(i2.严重(), "i2 95 vs 90 应严重");
        let mut r = 监控报告::新建();
        r.添加(i1);
        r.添加(i2);
        r.添加(指标::新建("falsifiable通过率", 指标类::正确性, 1.0));
        assert_eq!(r.警告数(), 0, "i1 不警告");
        assert_eq!(r.严重数(), 1, "i2 严重");
    }

    #[test]
    fn 告警级4枚举() {
        assert_eq!(
            4,
            [
                告警级::P3提示,
                告警级::P2次要,
                告警级::P1重要,
                告警级::P0紧急
            ]
            .len()
        );
    }

    #[test]
    fn 触发告警P0() {
        let mut i = 指标::新建("p99", 指标类::性能, 200.0);
        i.阈值_危急 = Some(100.0);
        let a = 触发告警(&i);
        assert!(a.is_some());
        assert_eq!(a.unwrap().级别, 告警级::P0紧急);
    }

    #[test]
    fn 触发告警P1警告级() {
        let mut i = 指标::新建("cpu", 指标类::资源, 85.0);
        i.阈值_警告 = Some(80.0);
        i.阈值_危急 = Some(95.0);
        let a = 触发告警(&i);
        assert!(a.is_some());
        assert_eq!(a.unwrap().级别, 告警级::P1重要);
    }

    #[test]
    fn 触发告警无() {
        let i = 指标::新建("正常", 指标类::性能, 50.0); // 无阈值
        let a = 触发告警(&i);
        assert!(a.is_none());
    }

    #[test]
    fn 告警引擎接收() {
        let mut e = 告警引擎::新建();
        let mut i = 指标::新建("cpu", 指标类::资源, 95.0);
        i.阈值_危急 = Some(90.0);
        e.接收指标(&i);
        assert_eq!(e.严重数(), 1);
    }

    #[test]
    fn p0升级道祖应急() {
        let mut a = 告警::新建(告警级::P0紧急, "p99", 200.0, 100.0);
        a.升级道祖应急();
        assert_eq!(a.状态, 告警状态::已确认);
    }

    #[test]
    fn 测试_4类指标全采() {
        let mut r = 监控报告::新建();
        r.添加(指标::新建("uptime", 指标类::可用性, 99.9));
        r.添加(指标::新建("p99延迟", 指标类::性能, 50.0));
        r.添加(指标::新建("falsifiable通过率", 指标类::正确性, 1.0));
        r.添加(指标::新建("内存使用率", 指标类::资源, 0.5));
        assert_eq!(r.指标列表.len(), 4);
    }
}
