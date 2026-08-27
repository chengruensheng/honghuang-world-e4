//! 源契约 - 配置读取特征
//!
//! 决策锚：260826-2230 工程-DSH
//! 关联文档：DSH 架构 system-prompt + scope

/// 配置读取特征
pub trait 配置源: Send + Sync {
    fn 取(&self, 键: &str) -> Option<String>;
}
