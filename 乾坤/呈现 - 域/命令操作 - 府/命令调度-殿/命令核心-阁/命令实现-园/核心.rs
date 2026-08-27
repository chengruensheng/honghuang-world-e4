//! 核心园 - 命令结果 + 命令 trait + 帮助命令 + 分发
//!
//! 殿核心类型与分发函数，桥接 init/status/run/e2e 四阁。

// 跨阁引用：从殿层 re-export 拿各阁符号
use super::super::super::{
    跑流水线_mock_llm, 跑流水线_反序, 跑流水线_循环打回, 跑流水线_跳层, Init命令, Run命令,
    Status命令,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct 命令结果 {
    pub 退出码: i32,
    pub 输出: String,
}

impl 命令结果 {
    pub fn 成功(输出: impl Into<String>) -> Self {
        Self {
            退出码: 0,
            输出: 输出.into(),
        }
    }
    pub fn 失败(码: i32, 输出: impl Into<String>) -> Self {
        Self {
            退出码: 码,
            输出: 输出.into(),
        }
    }
}

pub trait 命令: Send + Sync {
    fn 名称(&self) -> &str;
    fn 执行(&self, 参数: &[&str]) -> 命令结果;
}

pub struct 帮助命令;

impl 命令 for 帮助命令 {
    fn 名称(&self) -> &str {
        "帮助"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        命令结果::成功(
            "洪荒 · 世界 v3 · 阶段 8 端到端验证 + CLI\n\n             命令：\n               帮助                显示此帮助\n               init                环境检查 + 输出就绪状态\n               status              健康检查（workspace metadata）\n               run --task=<id>     跑任务（4 分类流水线）\n               e2e                 端到端 mock LLM（v4 阶段 17：真实 HTTP）\n               跳层测试            跳层拒绝路径\n               反序测试            反序拒绝路径\n               循环测试            循环打回升级道祖终裁"
        )
    }
}

pub fn 分发(参数: &[&str]) -> 命令结果 {
    if 参数.is_empty() {
        return 帮助命令.执行(&[]);
    }
    match 参数[0] {
        "帮助" | "--help" | "-h" => 帮助命令.执行(&[]),
        "init" => Init命令.执行(&参数[1..]),
        "status" => Status命令.执行(&参数[1..]),
        "run" => Run命令.执行(&参数[1..]),
        "e2e" => 跑流水线_mock_llm("e2e-默认任务"),
        "跳层测试" => 跑流水线_跳层(),
        "反序测试" => 跑流水线_反序(),
        "循环测试" => 跑流水线_循环打回(),
        other => 命令结果::失败(1, format!("未知命令：{}（运行 '帮助'）", other)),
    }
}
