//! 版本差异-阁 - 真实 git diff/log 工具
//!
//! 只读差异查看：子命令 diff 看未提交差异，log 看提交历史；拒绝 commit/push。

use crate::编排_殿::{工具, 调用输入, 调用输出};

pub struct 版本差异工具;

impl 版本差异工具 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 工具 for 版本差异工具 {
    fn 名称(&self) -> &str {
        "版本差异"
    }
    fn 描述(&self) -> &str {
        "真实 git diff/log（只读）"
    }
    fn 执行(&self, 输入: &调用输入) -> 调用输出 {
        let 子命令 = 输入.参数.get("子命令").cloned().unwrap_or_default();
        // 只读白名单：diff / log；拒绝 commit/push 等改写
        let 参数: &[&str] = match 子命令.as_str() {
            "log" => &["log", "--oneline", "-10"],
            _ => &["diff", "--stat"],
        };
        match std::process::Command::new("git").args(参数).output() {
            Ok(输出) => {
                let 码 = 输出.status.code().unwrap_or(-1);
                let 文本 = String::from_utf8_lossy(&输出.stdout).to_string();
                if 码 == 0 {
                    调用输出::成功(if 文本.is_empty() {
                        "无差异".to_string()
                    } else {
                        文本
                    })
                } else {
                    调用输出::失败(format!("git {} 退出码 {}", 子命令, 码))
                }
            }
            Err(e) => 调用输出::失败(format!("git {} 失败：{}", 子命令, e)),
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::编排_殿::{工具ID, 工具注册表, 工具调用循环, 工具调用请求};

    #[test]
    fn 版本差异可执行() {
        let 输出 = 版本差异工具::新建().执行(&调用输入::default());
        let _ = 输出;
    }

    #[test]
    fn 版本进注册表调用循环() {
        let mut 注册表 = 工具注册表::新建();
        注册表.注册(工具ID::版本, Box::new(版本差异工具::新建()));
        let 输出 = 工具调用循环(
            &注册表,
            vec![工具调用请求 {
                id: 工具ID::版本,
                输入: 调用输入::default(),
            }],
        );
        assert_eq!(输出.len(), 1);
    }
}
