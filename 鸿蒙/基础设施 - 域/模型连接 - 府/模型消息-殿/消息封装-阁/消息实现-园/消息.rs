//! 消息殿 - 角色 enum + 消息 struct
//!
//! 决策锚：260826-2240 传承殿启动 § 阶段 7
//! 关联文档：02-概念/可插拔/01-可插拔.md

// ============================================================================
// 消息
// ============================================================================

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 角色 {
    系统,
    用户,
    助手,
    工具,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 消息 {
    pub 角色: 角色,
    pub 内容: String,
}

impl 消息 {
    pub fn 系统(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::系统,
            内容: 内容.into(),
        }
    }
    pub fn 用户(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::用户,
            内容: 内容.into(),
        }
    }
    pub fn 助手(内容: impl Into<String>) -> Self {
        Self {
            角色: 角色::助手,
            内容: 内容.into(),
        }
    }
}
