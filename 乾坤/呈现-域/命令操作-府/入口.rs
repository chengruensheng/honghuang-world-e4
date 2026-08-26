//! 命令操作 - 府
//!
//! CLI 入口 + 命令解析 + 权限校验。
//! 阶段 1 仅承载命令特征与一行解析。
//!
//! 决策锚：260826-2230 工程-DSH § 一键全验
//! 关联文档：DSH 架构 agent-loop + scope

/// 命令结果
#[derive(Clone, Debug)]
pub struct 命令结果 {
    pub 退出码: i32,
    pub 输出: String,
}

/// 命令特征
pub trait 命令: Send + Sync {
    fn 名称(&self) -> &str;
    fn 执行(&self, 参数: &[&str]) -> 命令结果;
}

/// 帮助命令（阶段 1 默认实现）
pub struct 帮助命令;

impl 命令 for 帮助命令 {
    fn 名称(&self) -> &str {
        "帮助"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果 {
            退出码: 0,
            输出: "洪荒 · 世界 v3 · 阶段 1 地基完成".to_string(),
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn 帮助命令零退出() {
        let c = 帮助命令;
        let r = c.执行(&[]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("洪荒"));
    }
}
