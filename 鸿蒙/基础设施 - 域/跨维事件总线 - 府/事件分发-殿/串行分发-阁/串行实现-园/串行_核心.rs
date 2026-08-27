//! 串行阁 - Serial 监听器 trait + Serial 结果（独立拦截点）
//!
//! 决策锚：260826-2230 工程-DSH § Waterfall 事件
//! 关联文档：02-概念/事件流/04-事件流.md

// 跨殿引用：事件定义在事件类型殿（六层返工后改用 crate:: 路径）
use crate::事件类型_殿::事件;

/// Serial 监听器：独立拦截点
pub trait Serial监听器: Send + Sync {
    fn 处理(&self, 事件: &事件) -> Serial结果;
}

/// Serial 监听结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Serial结果 {
    通过,
    拒绝(String),
    拦截(String),
}
