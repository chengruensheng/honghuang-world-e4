//! 版本状态-阁 - 真实 git status 工具
//!
//! 阶段 7 工具循环升级：版本工具拆为「版本状态」（git status）与「版本差异」（git diff/log），
//! 均只读，拒绝 commit/push（治理铁律：版本库不可被 LLM 改写）。

use crate::编排_殿::{工具, 调用输入, 调用输出};

pub struct 版本状态工具;

impl 版本状态工具 {
    pub fn 新建() -> Self {
        Self
    }
}

impl 工具 for 版本状态工具 {
    fn 名称(&self) -> &str {
        "版本状态"
    }
    fn 描述(&self) -> &str {
        "真实 git status（只读）"
    }
    fn 执行(&self, _输入: &调用输入) -> 调用输出 {
        match std::process::Command::new("git")
            .args(["status", "--short"])
            .output()
        {
            Ok(输出) => {
                let 码 = 输出.status.code().unwrap_or(-1);
                let 文本 = String::from_utf8_lossy(&输出.stdout).to_string();
                if 码 == 0 {
                    调用输出::成功(if 文本.is_empty() {
                        "工作区干净".to_string()
                    } else {
                        文本
                    })
                } else {
                    调用输出::失败(format!("git status 退出码 {}", 码))
                }
            }
            Err(e) => 调用输出::失败(format!("git status 失败：{}", e)),
        }
    }
}

#[cfg(test)]
mod 测试 {
    use super::*;

    #[test]
    fn git状态可执行() {
        let 输出 = 版本状态工具::新建().执行(&调用输入::default());
        // 无 git 仓库时失败，有则成功——不 panic 即可
        let _ = 输出;
    }
}
