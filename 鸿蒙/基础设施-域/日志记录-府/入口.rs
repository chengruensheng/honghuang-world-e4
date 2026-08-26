//! 日志记录 - 府
//!
//! tracing 定制封装 + 结构化字段。
//! 阶段 1 提供特征与构造方法；阶段 2 接入事件总线。
//!
//! 决策锚：260826-2230 工程-DSH § DSH 万物皆插件
//! 关联文档：DSH 架构 tracing 子系统

/// 日志级别（与 tracing::Level 对应）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 级别 {
    跟踪,
    调试,
    信息,
    警告,
    错误,
}

/// 日志特征：所有日志实现必须满足
pub trait 日志: Send + Sync {
    fn 输出(&self, 级别: 级别, 消息: &str);
}

/// 标准输出日志（阶段 1 占位实现）
pub struct 标准输出日志;

impl 标准输出日志 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 日志 for 标准输出日志 {
    fn 输出(&self, 级别: 级别, 消息: &str) {
        let 前缀 = match 级别 {
            级别::跟踪 => "[TRC]",
            级别::调试 => "[DBG]",
            级别::信息 => "[INF]",
            级别::警告 => "[WRN]",
            级别::错误 => "[ERR]",
        };
        println!("{} {}", 前缀, 消息);
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 级别变体数匹配() {
        let 所有 = [级别::跟踪, 级别::调试, 级别::信息, 级别::警告, 级别::错误];
        assert_eq!(所有.len(), 5);
    }
}
