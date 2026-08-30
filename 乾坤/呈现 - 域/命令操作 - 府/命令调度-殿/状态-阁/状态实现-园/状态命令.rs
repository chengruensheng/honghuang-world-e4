//! 状态实现-园 - Status 命令实现（实时快照，非硬编码）
//!
//! 决策锚：260828-自动状态报告（第五十三轮）
//! falsifiable：status 输出含「最近提交」「未提交/未跟踪」「工作空间」三节，
//!             且工作空间府数 = 根 Cargo.toml members 路径行数（当前 34），非硬编码常量。

use super::super::super::{命令, 命令结果};
use crate::{状态报告, 默认记忆库路径};
use std::path::Path;
use std::process::Command;

pub struct Status命令;

impl 命令 for Status命令 {
    fn 名称(&self) -> &str {
        "status"
    }
    fn 执行(&self, _参数: &[&str]) -> 命令结果 {
        let 根 = Path::new("E:\\洪荒 - 世界");
        let mut 输出 = String::from("洪荒 · 世界 · 状态报告（实时）\n\n");

        输出.push_str(&最近提交(根));
        输出.push('\n');
        输出.push_str(&未提交统计(根));
        输出.push('\n');
        输出.push_str(&format!("工作空间：{} 个府\n", 府数(根)));
        输出.push_str("\n质量门禁（16 项）：运行「自检」命令获取最新通过/失败\n");

        // 记忆库状态报告（36 格位分布 + 债务 + 待补提炼 + 玉玺块数）
        输出.push_str("\n── 记忆库状态报告 ──\n");
        match 状态报告(默认记忆库路径) {
            Ok(行) => {
                for 每行 in 行 {
                    输出.push_str(&每行);
                    输出.push('\n');
                }
            }
            Err(e) => 输出.push_str(&format!("记忆库状态报告不可用：{}\n", e)),
        }

        命令结果::成功(输出)
    }
}

/// 最近 5 条提交（git log --oneline -5）
fn 最近提交(根: &Path) -> String {
    match Command::new("git")
        .args(["log", "--oneline", "-5"])
        .current_dir(根)
        .output()
    {
        Ok(o) if o.status.success() => {
            let 文本 = String::from_utf8_lossy(&o.stdout);
            format!("最近提交：\n{}", 文本)
        }
        _ => "最近提交：不可用（非 git 仓库或 git 不可用）".to_string(),
    }
}

/// 未提交/未跟踪文件数（git status --porcelain 行数）
fn 未提交统计(根: &Path) -> String {
    match Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(根)
        .output()
    {
        Ok(o) => {
            let 行数 = String::from_utf8_lossy(&o.stdout).lines().count();
            if 行数 == 0 {
                "未提交/未跟踪：0（工作区干净）".to_string()
            } else {
                format!("未提交/未跟踪：{} 个文件", 行数)
            }
        }
        _ => "未提交/未跟踪：不可用".to_string(),
    }
}

/// 工作空间府数：根 Cargo.toml members 路径行数（非硬编码）
fn 府数(根: &Path) -> usize {
    let 内容 = std::fs::read_to_string(根.join("Cargo.toml")).unwrap_or_default();
    内容
        .lines()
        .filter(|l| l.trim_start().starts_with('"'))
        .count()
}

#[cfg(test)]
mod 测试 {
    use super::*;
    use crate::命令;

    /// 防退化：工作空间府数必须与根 Cargo.toml members 路径行数一致（当前 34），
    /// 杜绝状态报告退回硬编码假数字。
    #[test]
    fn 府数_读根Cargo_toml为34() {
        let 根 = Path::new("E:\\洪荒 - 世界");
        assert_eq!(
            府数(根),
            34,
            "工作空间府数应为 34（根 Cargo.toml members 路径行数）"
        );
    }

    /// 状态命令实时快照三节齐全 + 退出码 0。
    #[test]
    fn 状态命令_输出含三节() {
        let r = Status命令.执行(&[]);
        assert_eq!(r.退出码, 0);
        assert!(r.输出.contains("最近提交"), "应含最近提交：{}", r.输出);
        assert!(
            r.输出.contains("未提交/未跟踪"),
            "应含未提交统计：{}",
            r.输出
        );
        assert!(r.输出.contains("工作空间"), "应含工作空间府数：{}", r.输出);
    }
}
