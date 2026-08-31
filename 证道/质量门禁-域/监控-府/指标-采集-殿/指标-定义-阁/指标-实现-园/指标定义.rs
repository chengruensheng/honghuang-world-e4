//! 指标实现 - 4 类指标（可用性/性能/正确性/资源）
//!
//! 接 07-运营/监控/02-监控.md
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 9

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
    fn 测试_4类指标全采() {
        let mut r = 监控报告::新建();
        r.添加(指标::新建("uptime", 指标类::可用性, 99.9));
        r.添加(指标::新建("p99延迟", 指标类::性能, 50.0));
        r.添加(指标::新建("falsifiable通过率", 指标类::正确性, 1.0));
        r.添加(指标::新建("内存使用率", 指标类::资源, 0.5));
        assert_eq!(r.指标列表.len(), 4);
    }
}
