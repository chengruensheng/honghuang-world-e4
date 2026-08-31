//! 瀑布阁 - Waterfall 监听器 trait（责任链模式）
//!
//! 决策锚：01-哲学/03-工程哲学.md § Waterfall 事件
//! 关联文档：02-概念/事件流/04-事件流.md

// 跨殿引用：事件与错误定义在事件类型殿（六层返工后改用 crate:: 路径）
use crate::事件_类型_殿::{事件, 错误};

/// Waterfall 监听器：监听器决定是否调 `下一步`
///
/// 决策锚：01-哲学/03-工程哲学.md § Waterfall 事件
pub trait Waterfall监听器: Send + Sync {
    fn 处理(
        &self,
        事件: &事件,
        下一步: &mut dyn FnMut() -> Result<(), 错误>,
    ) -> Result<(), 错误>;
}
